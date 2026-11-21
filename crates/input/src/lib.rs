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
//! the HOME button. All but Home can be rebound ([`keymap`]), and so can a
//! controller's buttons but its guide button, HOME ([`padmap`]).

pub mod capture;
pub mod keymap;
pub mod padmap;

use std::collections::HashSet;

use gilrs::{Button, GamepadId, Gilrs};
use hle_ios::wiimote::{button, Input};
use padmap::PadInput;
use winit::keyboard::KeyCode;

pub const PLAYERS: usize = 4;
/// A stick or trigger pushed this far counts as pressed.
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

/// What a [`keymap::ACTIONS`] entry does.
fn keymap_action(action: usize) -> Action {
    use Action::*;
    match action {
        keymap::UP => Buttons(sideways(Dir::Up)),
        keymap::DOWN => Buttons(sideways(Dir::Down)),
        keymap::LEFT => Buttons(sideways(Dir::Left)),
        keymap::RIGHT => Buttons(sideways(Dir::Right)),
        keymap::JUMP => Buttons(button::TWO),
        keymap::RUN => Buttons(button::ONE),
        keymap::A => Buttons(button::A),
        keymap::SHAKE => Shake,
        keymap::TILT_LEFT => Tilt(-1.0),
        keymap::TILT_RIGHT => Tilt(1.0),
        keymap::PLUS => Buttons(button::PLUS),
        _ => Buttons(button::MINUS),
    }
}

/// How far `p` is pushed on `pad`, 0 to 1.
fn pad_value(pad: &gilrs::Gamepad, p: PadInput) -> f32 {
    match p {
        PadInput::Button(b) => {
            let v = pad.button_data(b).map_or(0.0, |d| d.value());
            if pad.is_pressed(b) {
                v.max(STICK_PRESS)
            } else {
                v
            }
        }
        PadInput::Stick(axis, positive) => {
            let v = pad.value(axis);
            if positive {
                v.max(0.0)
            } else {
                (-v).max(0.0)
            }
        }
    }
}

fn apply(input: &mut Input, a: Action) {
    match a {
        Action::Buttons(b) => input.buttons |= b,
        Action::Shake => input.shake = true,
        Action::Tilt(t) => input.tilt += t,
    }
}

/// The game's rumble on a controller, as in WitherBlitz's
/// NSMB-Wii-Recompilation (`hle/input/wpad.cpp`): a controller's motors are
/// bigger than the remote's, so both run well under full strength (the
/// strong, low-frequency one at 0x3000, the weak one at 0x7000); and a
/// rumble lasts 2 s at most, so a missed stop can't run on.
const RUMBLE_LOW: u16 = 0x3000;
const RUMBLE_HIGH: u16 = 0x7000;
const RUMBLE_MAX_MS: u32 = 2000;

pub struct Inputs {
    keys: HashSet<KeyCode>,
    keymap: keymap::Keymap,
    padmap: padmap::Padmap,
    gilrs: Option<Gilrs>,
    /// Gamepads in the order they connected; index = player.
    pads: Vec<GamepadId>,
    /// Players whose remote has been switched on.
    on: [bool; PLAYERS],
    /// The Keybinds tab's Rumble switch.
    rumble_enabled: bool,
    /// Players whose controller is rumbling.
    rumbling: [bool; PLAYERS],
    /// Each controller's rumble, made the first time it rumbles.
    effects: Vec<(GamepadId, gilrs::ff::Effect)>,
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
        let mut s = Self { gilrs, ..Self::without_pads() };
        if let Some(g) = &s.gilrs {
            let ids: Vec<GamepadId> = g.gamepads().filter(|(_, p)| p.is_connected()).map(|(id, _)| id).collect();
            for id in ids {
                s.add_pad(id);
            }
        }
        s
    }

    /// The keyboard alone (no gamepad library).
    fn without_pads() -> Self {
        Self { keys: HashSet::new(), keymap: keymap::Keymap::default(), padmap: padmap::Padmap::default(), gilrs: None, pads: Vec::new(), on: [true, false, false, false], rumble_enabled: true, rumbling: [false; PLAYERS], effects: Vec::new() }
    }

    /// The Rumble switch: off stops any rumble at once.
    pub fn set_rumble_enabled(&mut self, on: bool) {
        self.rumble_enabled = on;
        if !on {
            for player in 0..PLAYERS {
                self.rumble(player, false);
            }
        }
    }

    /// Player `player`'s remote rumbles (its rumble bit, as the game last
    /// set it) or not: their controller follows.
    pub fn rumble(&mut self, player: usize, on: bool) {
        let on = on && self.rumble_enabled;
        if player >= PLAYERS || self.rumbling[player] == on {
            return;
        }
        self.rumbling[player] = on;
        let (Some(&id), Some(g)) = (self.pads.get(player), self.gilrs.as_mut()) else { return };
        if !g.connected_gamepad(id).is_some_and(|p| p.is_ff_supported()) {
            return;
        }
        let i = match self.effects.iter().position(|(p, _)| *p == id) {
            Some(i) => i,
            None => {
                use gilrs::ff::{BaseEffect, BaseEffectType, EffectBuilder, Repeat, Replay, Ticks};
                let replay = Replay { play_for: Ticks::from_ms(RUMBLE_MAX_MS), ..Replay::default() };
                let motor = |kind| BaseEffect { kind, scheduling: replay, ..BaseEffect::default() };
                let built = EffectBuilder::new()
                    .add_effect(motor(BaseEffectType::Strong { magnitude: RUMBLE_LOW }))
                    .add_effect(motor(BaseEffectType::Weak { magnitude: RUMBLE_HIGH }))
                    .repeat(Repeat::For(Ticks::from_ms(RUMBLE_MAX_MS)))
                    .gamepads(&[id])
                    .finish(g);
                match built {
                    Ok(e) => {
                        self.effects.push((id, e));
                        self.effects.len() - 1
                    }
                    Err(e) => {
                        eprintln!("rumble on gamepad {}: {e}", player + 1);
                        return;
                    }
                }
            }
        };
        let effect = &self.effects[i].1;
        let _ = if on { effect.play() } else { effect.stop() };
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

    /// The keys from now on.
    pub fn set_keymap(&mut self, keymap: keymap::Keymap) {
        self.keymap = keymap;
    }

    /// The controllers' buttons from now on.
    pub fn set_padmap(&mut self, padmap: padmap::Padmap) {
        self.padmap = padmap;
    }

    /// A controller is connected.
    pub fn pad_connected(&self) -> bool {
        self.gilrs.as_ref().is_some_and(|g| self.pads.iter().any(|&id| g.connected_gamepad(id).is_some()))
    }

    /// What is pressed on the connected controllers now (for picking a
    /// button in the Keybinds tab).
    pub fn pad_held(&self) -> Vec<PadInput> {
        let Some(g) = &self.gilrs else { return Vec::new() };
        let mut held = Vec::new();
        for pad in self.pads.iter().filter_map(|&id| g.connected_gamepad(id)) {
            for p in padmap::all() {
                if pad_value(&pad, p) >= STICK_PRESS && !held.contains(&p) {
                    held.push(p);
                }
            }
        }
        held
    }

    /// What a key does: its keymap action, or Home's HOME.
    fn key_action(&self, k: KeyCode) -> Option<Action> {
        match self.keymap.action(k) {
            Some(a) => Some(keymap_action(a)),
            None => (k == KeyCode::Home).then_some(Action::Buttons(button::HOME)),
        }
    }

    /// A key went down or up. Returns whether the key is a game control.
    pub fn key(&mut self, code: KeyCode, down: bool) -> bool {
        if self.key_action(code).is_none() && !self.keys.contains(&code) {
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
            // Every action the key plays (a key may be bound to several).
            for (i, keys) in self.keymap.keys.iter().enumerate() {
                if keys.contains(k) {
                    apply(&mut out[0], keymap_action(i));
                }
            }
            if *k == KeyCode::Home {
                apply(&mut out[0], Action::Buttons(button::HOME));
            }
        }
        if let Some(g) = &self.gilrs {
            for (player, &id) in self.pads.iter().enumerate() {
                let pad = g.gamepad(id);
                if !pad.is_connected() {
                    continue;
                }
                let input = &mut out[player];
                // Tilt follows how far its buttons are pushed; the rest are
                // pressed or not.
                let mut tilt = [0.0f32; 2];
                for (i, bound) in self.padmap.buttons.iter().enumerate() {
                    let v = bound.iter().map(|&p| pad_value(&pad, p)).fold(0.0, f32::max);
                    match i {
                        keymap::TILT_LEFT => tilt[0] = v,
                        keymap::TILT_RIGHT => tilt[1] = v,
                        _ if v >= STICK_PRESS => apply(input, keymap_action(i)),
                        _ => {}
                    }
                }
                let analog = tilt[1] - tilt[0];
                if analog.abs() > TILT_DEAD_ZONE {
                    input.tilt += analog;
                }
                if pad.is_pressed(Button::Mode) {
                    input.buttons |= button::HOME;
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
        let mut i = Inputs::without_pads();
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
