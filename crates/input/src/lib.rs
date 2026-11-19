//! Keyboard and gamepads mapped to up to four sideways Wii Remotes
//! (docs/PORT_PLAN.md, `crates/input`). The result feeds the emulated
//! Bluetooth remotes in `hle-ios`.
//!
//! The keyboard is player 1. Gamepads take players 1..4 in the order they
//! connect, so a single gamepad plays alongside the keyboard as player 1.
//! A player's remote switches on when their first device appears and stays
//! on, like a remote left powered.
//!
//! Keyboard defaults follow WitherBlitz's NSMB-Wii-Recompilation: arrows
//! D-pad, X or Space 2 (jump), Z or Left Shift 1 (run), C or Ctrl shake,
//! Q/E tilt, B the A button, Enter or Esc +, Minus or Tab -, plus Home for
//! the HOME button.

use std::collections::HashSet;

use gilrs::{Axis, Button, GamepadId, Gilrs};
use hle_ios::wiimote::{button, Input};
use winit::keyboard::KeyCode;

pub const PLAYERS: usize = 4;
/// Stick deflection that counts as a D-pad press.
const STICK_PRESS: f32 = 0.5;
/// Analog tilt below this is ignored (stick drift).
const TILT_DEAD_ZONE: f32 = 0.15;

/// A direction on screen, mapped onto the D-pad of a remote held sideways
/// (the D-pad on the left): screen up is the D-pad's right, and so on.
#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn sideways(d: Dir) -> u16 {
    match d {
        Dir::Up => button::RIGHT,
        Dir::Down => button::LEFT,
        Dir::Left => button::UP,
        Dir::Right => button::DOWN,
    }
}

/// What a key or gamepad button does.
#[derive(Clone, Copy)]
enum Action {
    Buttons(u16),
    Shake,
    Tilt(f32),
}

fn key_action(k: KeyCode) -> Option<Action> {
    use Action::*;
    Some(match k {
        KeyCode::ArrowUp => Buttons(sideways(Dir::Up)),
        KeyCode::ArrowDown => Buttons(sideways(Dir::Down)),
        KeyCode::ArrowLeft => Buttons(sideways(Dir::Left)),
        KeyCode::ArrowRight => Buttons(sideways(Dir::Right)),
        KeyCode::KeyX | KeyCode::Space => Buttons(button::TWO),
        KeyCode::KeyZ | KeyCode::ShiftLeft => Buttons(button::ONE),
        KeyCode::KeyB => Buttons(button::A),
        KeyCode::Enter | KeyCode::Escape => Buttons(button::PLUS),
        KeyCode::Minus | KeyCode::Tab => Buttons(button::MINUS),
        KeyCode::Home => Buttons(button::HOME),
        KeyCode::KeyC | KeyCode::ControlLeft | KeyCode::ControlRight => Shake,
        KeyCode::KeyQ => Tilt(-1.0),
        KeyCode::KeyE => Tilt(1.0),
        _ => return None,
    })
}

/// Xbox layout: A jumps (2), B and X run (1), Y and the bumpers shake,
/// Start is +, Back is -, the guide button is HOME; the triggers and the
/// right stick tilt.
fn pad_action(b: Button) -> Option<Action> {
    use Action::*;
    Some(match b {
        Button::DPadUp => Buttons(sideways(Dir::Up)),
        Button::DPadDown => Buttons(sideways(Dir::Down)),
        Button::DPadLeft => Buttons(sideways(Dir::Left)),
        Button::DPadRight => Buttons(sideways(Dir::Right)),
        Button::South => Buttons(button::TWO),
        Button::East | Button::West => Buttons(button::ONE),
        Button::North | Button::LeftTrigger | Button::RightTrigger => Shake,
        Button::Start => Buttons(button::PLUS),
        Button::Select => Buttons(button::MINUS),
        Button::Mode => Buttons(button::HOME),
        _ => return None,
    })
}

const PAD_BUTTONS: [Button; 15] = [
    Button::DPadUp,
    Button::DPadDown,
    Button::DPadLeft,
    Button::DPadRight,
    Button::South,
    Button::East,
    Button::West,
    Button::North,
    Button::LeftTrigger,
    Button::RightTrigger,
    Button::Start,
    Button::Select,
    Button::Mode,
    Button::LeftThumb,
    Button::RightThumb,
];

fn apply(input: &mut Input, a: Action) {
    match a {
        Action::Buttons(b) => input.buttons |= b,
        Action::Shake => input.shake = true,
        Action::Tilt(t) => input.tilt += t,
    }
}

pub struct Inputs {
    keys: HashSet<KeyCode>,
    gilrs: Option<Gilrs>,
    /// Gamepads in the order they connected; index = player.
    pads: Vec<GamepadId>,
    /// Players whose remote has been switched on.
    on: [bool; PLAYERS],
}

impl Default for Inputs {
    fn default() -> Self {
        Self::new()
    }
}

impl Inputs {
    pub fn new() -> Self {
        let gilrs = match Gilrs::new() {
            Ok(g) => Some(g),
            Err(gilrs::Error::NotImplemented(g)) => Some(g),
            Err(e) => {
                eprintln!("gamepads unavailable: {e}");
                None
            }
        };
        let mut s = Self { keys: HashSet::new(), gilrs, pads: Vec::new(), on: [true, false, false, false] };
        if let Some(g) = &s.gilrs {
            let ids: Vec<GamepadId> = g.gamepads().filter(|(_, p)| p.is_connected()).map(|(id, _)| id).collect();
            for id in ids {
                s.add_pad(id);
            }
        }
        s
    }

    fn add_pad(&mut self, id: GamepadId) {
        if self.pads.contains(&id) || self.pads.len() >= PLAYERS {
            return;
        }
        if let Some(g) = &self.gilrs {
            println!("gamepad {} ({}) is player {}", self.pads.len() + 1, g.gamepad(id).name(), self.pads.len() + 1);
        }
        self.on[self.pads.len()] = true;
        self.pads.push(id);
    }

    /// A key went down or up. Returns whether the key is a game control.
    pub fn key(&mut self, code: KeyCode, down: bool) -> bool {
        if key_action(code).is_none() {
            return false;
        }
        if down {
            self.keys.insert(code);
        } else {
            self.keys.remove(&code);
        }
        true
    }

    /// Releases every key (the window lost focus).
    pub fn release_keys(&mut self) {
        self.keys.clear();
    }

    /// Current state of the four remotes.
    pub fn poll(&mut self) -> [Input; PLAYERS] {
        let mut connected = Vec::new();
        if let Some(g) = &mut self.gilrs {
            while let Some(ev) = g.next_event() {
                if matches!(ev.event, gilrs::EventType::Connected) {
                    connected.push(ev.id);
                }
            }
        }
        for id in connected {
            self.add_pad(id);
        }
        let mut out = [Input::default(); PLAYERS];
        for k in &self.keys {
            if let Some(a) = key_action(*k) {
                apply(&mut out[0], a);
            }
        }
        if let Some(g) = &self.gilrs {
            for (player, &id) in self.pads.iter().enumerate() {
                let pad = g.gamepad(id);
                if !pad.is_connected() {
                    continue;
                }
                let input = &mut out[player];
                for b in PAD_BUTTONS {
                    if pad.is_pressed(b) {
                        if let Some(a) = pad_action(b) {
                            apply(input, a);
                        }
                    }
                }
                let (x, y) = (pad.value(Axis::LeftStickX), pad.value(Axis::LeftStickY));
                for (on, d) in [(y > STICK_PRESS, Dir::Up), (y < -STICK_PRESS, Dir::Down), (x < -STICK_PRESS, Dir::Left), (x > STICK_PRESS, Dir::Right)] {
                    if on {
                        input.buttons |= sideways(d);
                    }
                }
                let trigger = |b: Button| pad.button_data(b).map_or(0.0, |d| d.value());
                let analog = pad.value(Axis::RightStickX) + trigger(Button::RightTrigger2) - trigger(Button::LeftTrigger2);
                if analog.abs() > TILT_DEAD_ZONE {
                    input.tilt += analog;
                }
            }
        }
        for (i, input) in out.iter_mut().enumerate() {
            input.connected = self.on[i];
            input.tilt = input.tilt.clamp(-1.0, 1.0);
        }
        out
    }
}

/// Wii Remote buttons by name, for scripted input (`--press`): a b 1 2 up
/// down left right (the remote's own D-pad) plus minus home, and shake.
pub fn parse_buttons(names: &str) -> Result<Input, String> {
    let mut input = Input { connected: true, ..Input::default() };
    for n in names.split('+').filter(|s| !s.is_empty()) {
        input.buttons |= match n.to_ascii_lowercase().as_str() {
            "a" => button::A,
            "b" => button::B,
            "1" => button::ONE,
            "2" => button::TWO,
            "up" => button::UP,
            "down" => button::DOWN,
            "left" => button::LEFT,
            "right" => button::RIGHT,
            "plus" => button::PLUS,
            "minus" => button::MINUS,
            "home" => button::HOME,
            "shake" => {
                input.shake = true;
                0
            }
            other => return Err(format!("unknown Wii Remote button: {other}")),
        };
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_plays_a_sideways_remote() {
        let mut i = Inputs { keys: HashSet::new(), gilrs: None, pads: Vec::new(), on: [true, false, false, false] };
        assert!(i.key(KeyCode::ArrowRight, true));
        assert!(i.key(KeyCode::Space, true));
        assert!(i.key(KeyCode::KeyC, true));
        assert!(i.key(KeyCode::KeyE, true));
        assert!(!i.key(KeyCode::F3, true), "hotkeys are not game controls");
        let s = i.poll();
        assert_eq!(s[0].buttons, button::DOWN | button::TWO);
        assert!(s[0].shake && s[0].connected);
        assert_eq!(s[0].tilt, 1.0);
        assert!(!s[1].connected);
        i.release_keys();
        assert_eq!(i.poll()[0].buttons, 0);
    }

    #[test]
    fn scripted_buttons() {
        let s = parse_buttons("2+right+shake").unwrap();
        assert_eq!(s.buttons, button::TWO | button::RIGHT);
        assert!(s.shake);
        assert!(parse_buttons("x").is_err());
    }
}
