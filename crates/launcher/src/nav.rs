//! Menu input from every device as one stream of moves: a controller
//! (through `gilrs`, as `crates/input` reads it for the game), the keyboard,
//! and a script (`--script`, for tests). The mouse is read by the widgets.
//!
//! Controller, Xbox layout: D-pad or left stick moves, A confirms, B goes
//! back, Y opens Switch Version, X is the screen's extra action, Start plays,
//! the bumpers step through versions.
//! Keyboard: arrows or WASD move, Enter or Space confirm, Esc or Backspace
//! go back, Tab opens Switch Version, F is the extra action, Q and E step
//! through versions.

use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nav {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Back,
    /// Y: Switch Version.
    Switch,
    /// X: the screen's extra action.
    Extra,
    /// Start: Play.
    Play,
    /// The bumpers.
    Prev,
    Next,
}

impl Nav {
    fn is_direction(self) -> bool {
        matches!(self, Nav::Up | Nav::Down | Nav::Left | Nav::Right)
    }

    /// The name used in scripts.
    pub fn parse(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "up" => Nav::Up,
            "down" => Nav::Down,
            "left" => Nav::Left,
            "right" => Nav::Right,
            "a" => Nav::Confirm,
            "b" => Nav::Back,
            "y" => Nav::Switch,
            "x" => Nav::Extra,
            "start" => Nav::Play,
            "lb" => Nav::Prev,
            "rb" => Nav::Next,
            _ => return None,
        })
    }
}

/// What a controller button does in the menus.
pub fn pad_nav(b: Button) -> Option<Nav> {
    Some(match b {
        Button::DPadUp => Nav::Up,
        Button::DPadDown => Nav::Down,
        Button::DPadLeft => Nav::Left,
        Button::DPadRight => Nav::Right,
        Button::South => Nav::Confirm,
        Button::East => Nav::Back,
        Button::North => Nav::Switch,
        Button::West => Nav::Extra,
        Button::Start => Nav::Play,
        Button::LeftTrigger => Nav::Prev,
        Button::RightTrigger => Nav::Next,
        _ => return None,
    })
}

/// What a key does in the menus.
pub fn key_nav(k: egui::Key) -> Option<Nav> {
    use egui::Key;
    Some(match k {
        Key::ArrowUp | Key::W => Nav::Up,
        Key::ArrowDown | Key::S => Nav::Down,
        Key::ArrowLeft | Key::A => Nav::Left,
        Key::ArrowRight | Key::D => Nav::Right,
        Key::Enter | Key::Space => Nav::Confirm,
        Key::Escape | Key::Backspace => Nav::Back,
        Key::Tab => Nav::Switch,
        Key::F => Nav::Extra,
        Key::Q => Nav::Prev,
        Key::E => Nav::Next,
        _ => return None,
    })
}

/// The stick direction past the threshold, if any.
fn stick_nav(x: f32, y: f32) -> Option<Nav> {
    const PRESS: f32 = 0.55;
    if x.abs() < PRESS && y.abs() < PRESS {
        None
    } else if x.abs() > y.abs() {
        Some(if x > 0.0 { Nav::Right } else { Nav::Left })
    } else {
        Some(if y > 0.0 { Nav::Up } else { Nav::Down })
    }
}

/// The device last used, which decides the button hints shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Device {
    Keyboard,
    Pad,
}

/// A scripted move at a frame (`--script 30:down,45:a`).
pub fn parse_script(text: &str) -> Result<Vec<(u64, Nav)>, String> {
    let mut script = Vec::new();
    for word in text.split([',', ' ']).filter(|w| !w.is_empty()) {
        let (frame, name) = word.split_once(':').ok_or(format!("--script {word}: expected <frame>:<move>"))?;
        let frame = frame.parse().map_err(|_| format!("--script {word}: bad frame"))?;
        let nav = Nav::parse(name).ok_or(format!("--script {word}: moves are up down left right a b x y start lb rb"))?;
        script.push((frame, nav));
    }
    Ok(script)
}

/// Seconds a direction is held before it repeats, and between repeats.
const REPEAT_DELAY: f32 = 0.38;
const REPEAT_EVERY: f32 = 0.11;

pub struct Input {
    gilrs: Option<Gilrs>,
    /// The controller direction held (D-pad or stick) and the time until it
    /// repeats.
    held: Option<(Nav, f32)>,
    script: Vec<(u64, Nav)>,
    pub device: Device,
    /// The name of the controller last used or connected.
    pub pad: Option<String>,
}

impl Input {
    pub fn new(script: Vec<(u64, Nav)>) -> Self {
        let gilrs = match Gilrs::new() {
            Ok(g) => Some(g),
            Err(gilrs::Error::NotImplemented(g)) => Some(g),
            Err(e) => {
                eprintln!("controllers unavailable: {e}");
                None
            }
        };
        let pad = gilrs.as_ref().and_then(|g| g.gamepads().find(|(_, p)| p.is_connected()).map(|(_, p)| p.name().to_string()));
        // A scripted run stands in for a controller.
        let device = if pad.is_some() || !script.is_empty() { Device::Pad } else { Device::Keyboard };
        Self { gilrs, held: None, script, device, pad }
    }

    /// The moves made since the last frame.
    pub fn poll(&mut self, ctx: &egui::Context, frame: u64, dt: f32) -> Vec<Nav> {
        let mut out = Vec::new();

        for &(_, nav) in self.script.iter().filter(|s| s.0 == frame) {
            self.device = Device::Pad;
            out.push(nav);
        }

        ctx.input(|i| {
            for e in &i.events {
                if let egui::Event::Key { key, pressed: true, repeat, modifiers, .. } = e {
                    if modifiers.alt || modifiers.ctrl {
                        continue;
                    }
                    if let Some(nav) = key_nav(*key).filter(|n| !*repeat || n.is_direction()) {
                        self.device = Device::Keyboard;
                        out.push(nav);
                    }
                }
            }
        });

        let mut active: Option<GamepadId> = None;
        if let Some(g) = &mut self.gilrs {
            while let Some(ev) = g.next_event() {
                match ev.event {
                    EventType::ButtonPressed(b, _) => {
                        active = Some(ev.id);
                        if let Some(nav) = pad_nav(b) {
                            self.device = Device::Pad;
                            out.push(nav);
                            if nav.is_direction() {
                                self.held = Some((nav, REPEAT_DELAY));
                            }
                        }
                    }
                    EventType::Connected => active = Some(ev.id),
                    EventType::Disconnected => {
                        self.pad = None;
                        self.held = None;
                    }
                    _ => {}
                }
            }
            if let Some(id) = active {
                self.pad = Some(g.gamepad(id).name().to_string());
            }
            // What is held now, on any controller: the D-pad, else the stick.
            let mut now = None;
            for (_, pad) in g.gamepads().filter(|(_, p)| p.is_connected()) {
                let dpad = [Button::DPadUp, Button::DPadDown, Button::DPadLeft, Button::DPadRight].into_iter().find(|b| pad.is_pressed(*b)).and_then(pad_nav);
                now = dpad.or_else(|| stick_nav(pad.value(Axis::LeftStickX), pad.value(Axis::LeftStickY)));
                if now.is_some() {
                    break;
                }
            }
            self.held = match (self.held, now) {
                (_, None) => None,
                (Some((was, left)), Some(nav)) if was == nav => {
                    let left = left - dt;
                    if left <= 0.0 {
                        out.push(nav);
                        Some((nav, REPEAT_EVERY))
                    } else {
                        Some((nav, left))
                    }
                }
                // A new direction: the D-pad's press was already taken from
                // its event; the stick's is taken here.
                (_, Some(nav)) => {
                    if !out.contains(&nav) {
                        self.device = Device::Pad;
                        out.push(nav);
                    }
                    Some((nav, REPEAT_DELAY))
                }
            };
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_controller_drives_the_menu() {
        assert_eq!(pad_nav(Button::DPadDown), Some(Nav::Down));
        assert_eq!(pad_nav(Button::South), Some(Nav::Confirm));
        assert_eq!(pad_nav(Button::East), Some(Nav::Back));
        assert_eq!(pad_nav(Button::North), Some(Nav::Switch));
        assert_eq!(pad_nav(Button::Start), Some(Nav::Play));
        assert_eq!(pad_nav(Button::LeftTrigger), Some(Nav::Prev));
        assert_eq!(pad_nav(Button::Mode), None);
        assert_eq!(stick_nav(0.9, 0.2), Some(Nav::Right));
        assert_eq!(stick_nav(-0.1, -0.8), Some(Nav::Down));
        assert_eq!(stick_nav(0.3, 0.3), None);
    }

    #[test]
    fn keys_and_scripts() {
        assert_eq!(key_nav(egui::Key::Enter), Some(Nav::Confirm));
        assert_eq!(key_nav(egui::Key::Escape), Some(Nav::Back));
        assert_eq!(key_nav(egui::Key::F5), None);
        assert_eq!(parse_script("10:down, 20:a 30:lb").unwrap(), vec![(10, Nav::Down), (20, Nav::Confirm), (30, Nav::Prev)]);
        assert!(parse_script("10:jump").is_err());
        assert!(parse_script("down").is_err());
    }
}
