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
pub mod combo;
pub mod gyro;
pub mod keymap;
pub mod motion;
pub mod padmap;
pub mod pads;
pub mod smart_spin;
#[cfg(windows)]
pub mod bt_usb;
#[cfg(windows)]
pub mod wii_hid;

use std::collections::HashSet;
use std::time::{Duration, Instant};

use hle_ios::wiimote::{button, Input};
use padmap::{Button, PadInput};
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

fn apply(input: &mut Input, a: Action) {
    match a {
        Action::Buttons(b) => input.buttons |= b,
        Action::Shake => input.shake = true,
        Action::Tilt(t) => input.tilt += t,
    }
}

/// The actions held through combinations (`1 << action` bits).
fn apply_bits(input: &mut Input, bits: u32) {
    for action in 0..keymap::ACTIONS.len() {
        if bits & (1 << action) != 0 {
            apply(input, keymap_action(action));
        }
    }
}

/// A single let go while it waited for a combination (a tap) still plays
/// its action this long, so a field reads it.
const TAP: Duration = Duration::from_millis(40);
/// A shake asked for by a press (a smart spin) lasts this long: ten fields,
/// enough for the game to see one shake.
const SHAKE_PULSE: Duration = Duration::from_millis(170);

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
    /// The mouse's buttons held (bit n - 1 for button n), counted only
    /// while no menu has the mouse.
    mouse: u8,
    keymap: keymap::Keymap,
    padmap: padmap::Padmap,
    /// The combinations' rules: the keyboard's, and each controller's.
    key_combos: combo::Engine<keymap::Key>,
    pad_combos: [combo::Engine<PadInput>; PLAYERS],
    /// Actions played for a moment (taps): player, action, until when.
    pulses: Vec<(usize, usize, Duration)>,
    /// Times are from here.
    epoch: Instant,
    /// Space Smart Spin: on, Space's state, player 1 as the game left them.
    smart_spin: bool,
    space: smart_spin::SmartSpin,
    player: Option<smart_spin::Player>,
    /// Shaking until then (a smart spin, a mouse swing), per player.
    shake_until: [Duration; PLAYERS],
    /// Mouse as Gyro: on, and the pointer's x and the window's width.
    mouse_gyro: bool,
    gyro: gyro::Gyro,
    pointer: Option<(f32, f32)>,
    /// Every remote's tilt mirrored (the Reverse Tilt switch).
    reverse_tilt: bool,
    /// The controllers (SDL), each in its player's slot.
    pads: Option<pads::Pads>,
    /// Controllers' motion sensors tilt and shake (the Motion Sensors
    /// switch), and each player's controller's motion.
    motion_sensors: bool,
    pad_motion: [motion::PadMotion; PLAYERS],
    /// Players whose remote has been switched on.
    on: [bool; PLAYERS],
    /// The Keybinds tab's Rumble switch.
    rumble_enabled: bool,
    /// Players whose controller is rumbling.
    rumbling: [bool; PLAYERS],
    /// What each controller had pressed at the last poll, and whether one
    /// pressed something new since [`Inputs::take_pad_pressed`].
    pad_was: [Vec<PadInput>; PLAYERS],
    pad_pressed: bool,
}

impl Default for Inputs {
    fn default() -> Self {
        Self::new()
    }
}

impl Inputs {
    pub fn new() -> Self {
        let mut s = Self { pads: pads::Pads::new(), ..Self::without_pads() };
        s.update_pads();
        s
    }

    /// The keyboard alone (no gamepad library).
    fn without_pads() -> Self {
        Self { keys: HashSet::new(), mouse: 0, keymap: keymap::Keymap::default(), padmap: padmap::Padmap::default(), key_combos: combo::Engine::default(), pad_combos: Default::default(), pulses: Vec::new(), epoch: Instant::now(), smart_spin: true, space: smart_spin::SmartSpin::default(), player: None, shake_until: [Duration::ZERO; PLAYERS], mouse_gyro: false, gyro: gyro::Gyro::default(), pointer: None, reverse_tilt: false, pads: None, motion_sensors: true, pad_motion: Default::default(), on: [true, false, false, false], rumble_enabled: true, rumbling: [false; PLAYERS], pad_was: Default::default(), pad_pressed: false }
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
        let Some(pads) = &self.pads else { return };
        let (low, high, ms) = if on { (RUMBLE_LOW, RUMBLE_HIGH, RUMBLE_MAX_MS) } else { (0, 0, 0) };
        pads.rumble(player, low, high, ms);
    }

    /// Takes in controllers that connected since the last look; each new
    /// one switches its player's remote on.
    fn update_pads(&mut self) {
        let Some(pads) = &mut self.pads else { return };
        for (player, name) in pads.update() {
            println!("gamepad {} ({name}) is player {}", player + 1, player + 1);
            self.on[player] = true;
            self.rumbling[player] = false;
            self.pad_motion[player].reset();
        }
    }

    /// The keys from now on.
    pub fn set_keymap(&mut self, keymap: keymap::Keymap) {
        self.key_combos = combo::Engine::new(keymap.combo_setup());
        self.keymap = keymap;
    }

    /// The controllers' buttons from now on.
    pub fn set_padmap(&mut self, padmap: padmap::Padmap) {
        self.pad_combos = std::array::from_fn(|_| combo::Engine::new(padmap.combo_setup()));
        self.padmap = padmap;
    }

    /// Space Smart Spin on or off.
    pub fn set_smart_spin(&mut self, on: bool) {
        self.smart_spin = on;
    }

    /// Mouse as Gyro on or off (`gyro`): the mouse tilts and shakes player
    /// 1's remote while no controller is connected.
    pub fn set_mouse_gyro(&mut self, on: bool) {
        self.mouse_gyro = on;
        self.gyro.reset();
    }

    /// Motion Sensors: a controller's own accelerometer tilts (turned like a
    /// steering wheel) and shakes (a jolt) its player's remote (`motion`).
    pub fn set_motion_sensors(&mut self, on: bool) {
        self.motion_sensors = on;
        for m in &mut self.pad_motion {
            m.reset();
        }
    }

    /// Reverse Tilt: every remote's tilt mirrored, whatever it comes from.
    pub fn set_reverse_tilt(&mut self, on: bool) {
        self.reverse_tilt = on;
    }

    /// The pointer moved to `x` in a window `width` across (pixels).
    pub fn pointer(&mut self, x: f32, width: f32) {
        self.pointer = Some((x, width));
    }

    /// Player 1 as the game left them after its last field (`None` outside
    /// a course): their state and vertical move.
    pub fn set_player(&mut self, player: Option<(u32, f32)>) {
        self.player = player.map(|(state, vertical)| smart_spin::Player { state, vertical });
    }

    /// A controller had a button pressed (or a stick or trigger pushed past
    /// half) since the last call: the player is using it.
    pub fn take_pad_pressed(&mut self) -> bool {
        std::mem::take(&mut self.pad_pressed)
    }

    /// A controller is connected.
    pub fn pad_connected(&self) -> bool {
        self.pads.as_ref().is_some_and(pads::Pads::any_connected)
    }

    /// What is pressed on the connected controllers now (for picking a
    /// button in the Keybinds tab).
    pub fn pad_held(&self) -> Vec<PadInput> {
        let Some(pads) = &self.pads else { return Vec::new() };
        let mut held = Vec::new();
        for player in (0..PLAYERS).filter(|&i| pads.connected(i)) {
            for p in padmap::all() {
                if pads.value(player, p) >= STICK_PRESS && !held.contains(&p) {
                    held.push(p);
                }
            }
        }
        held
    }

    /// A key or mouse button plays something: an action, a part in a
    /// combination, or (Home) the HOME button.
    fn is_control(&self, k: keymap::Key) -> bool {
        self.keymap.action(k).is_some() || self.keymap.in_combo(k) || k == keymap::Key::Code(KeyCode::Home)
    }

    /// A key went down or up. Returns whether the key is a game control.
    pub fn key(&mut self, code: KeyCode, down: bool) -> bool {
        if !self.is_control(code.into()) && !self.keys.contains(&code) {
            return false;
        }
        if down {
            self.keys.insert(code);
        } else {
            self.keys.remove(&code);
        }
        true
    }

    /// A mouse button (1 left, 2 middle, 3 right, 4, 5) went down or up.
    /// Returns whether it is a game control.
    pub fn mouse_button(&mut self, n: u8, down: bool) -> bool {
        if !(1..=5).contains(&n) {
            return false;
        }
        let bit = 1 << (n - 1);
        if !down {
            let held = self.mouse & bit != 0;
            self.mouse &= !bit;
            return held;
        }
        if !self.is_control(keymap::Key::Mouse(n)) {
            return false;
        }
        self.mouse |= bit;
        true
    }

    /// Releases every key and mouse button (the window lost focus).
    pub fn release_keys(&mut self) {
        self.keys.clear();
        self.mouse = 0;
        self.key_combos.reset();
    }

    /// Current state of the four remotes.
    pub fn poll(&mut self) -> [Input; PLAYERS] {
        self.poll_at(self.epoch.elapsed())
    }

    /// The remotes at time `now` (from `epoch`).
    fn poll_at(&mut self, now: Duration) -> [Input; PLAYERS] {
        self.update_pads();
        let mut out = [Input::default(); PLAYERS];
        self.pulses.retain(|p| p.2 > now);
        // The keyboard and mouse: combinations first, then the keys they
        // leave alone.
        let mouse = (1..=5u8).filter(|n| self.mouse & (1 << (n - 1)) != 0).map(keymap::Key::Mouse);
        let held: Vec<keymap::Key> = self.keys.iter().map(|&k| keymap::Key::Code(k)).chain(mouse).collect();
        let combos = if self.key_combos.is_empty() { 0 } else { self.key_combos.step(&held, now) };
        // Space as Jump in the air: a spin (`smart_spin`).
        let space = keymap::Key::Code(KeyCode::Space);
        let smart = self.smart_spin && self.keymap.keys[keymap::JUMP].contains(&space);
        let space_down = (held.contains(&space) && !self.key_combos.silenced(space)) || self.key_combos.tapped().contains(&space);
        let spin = self.space.update(space_down, smart, self.player, now);
        if spin.spin {
            self.shake_until[0] = now + SHAKE_PULSE;
        }
        if smart && spin.two {
            apply(&mut out[0], keymap_action(keymap::JUMP));
        }
        for &k in &held {
            if self.key_combos.silenced(k) {
                continue;
            }
            // Every action the key plays (a key may be bound to several).
            for (i, keys) in self.keymap.keys.iter().enumerate() {
                if keys.contains(&k) && !(smart && k == space && i == keymap::JUMP) {
                    apply(&mut out[0], keymap_action(i));
                }
            }
            if k == keymap::Key::Code(KeyCode::Home) {
                apply(&mut out[0], Action::Buttons(button::HOME));
            }
        }
        for &k in self.key_combos.tapped() {
            for (i, keys) in self.keymap.keys.iter().enumerate() {
                if keys.contains(&k) && !(smart && k == space && i == keymap::JUMP) {
                    self.pulses.push((0, i, now + TAP));
                }
            }
        }
        apply_bits(&mut out[0], combos);
        // The mouse as the remote's motion (keys' tilt comes first).
        match self.pointer.filter(|_| self.mouse_gyro && !self.pad_connected()) {
            Some((x, width)) => {
                let motion = self.gyro.update(x, width, now);
                if motion.shake {
                    self.shake_until[0] = now + SHAKE_PULSE;
                }
                if out[0].tilt == 0.0 {
                    out[0].tilt = motion.tilt;
                }
            }
            None => self.gyro.reset(),
        }
        if let Some(pads) = &mut self.pads {
            for player in 0..PLAYERS {
                if !pads.connected(player) {
                    self.pad_motion[player].reset();
                    continue;
                }
                let pad_value = |p| pads.value(player, p);
                let input = &mut out[player];
                let engine = &mut self.pad_combos[player];
                let held: Vec<PadInput> = padmap::all().filter(|&p| pad_value(p) >= STICK_PRESS).collect();
                self.pad_pressed |= held.iter().any(|p| !self.pad_was[player].contains(p));
                self.pad_was[player].clone_from(&held);
                let combos = if engine.is_empty() { 0 } else { engine.step(&held, now) };
                // Tilt follows how far its buttons are pushed; the rest are
                // pressed or not (and not taken by a combination).
                let mut tilt = [0.0f32; 2];
                for (i, bound) in self.padmap.buttons.iter().enumerate() {
                    match i {
                        keymap::TILT_LEFT | keymap::TILT_RIGHT => {
                            tilt[usize::from(i == keymap::TILT_RIGHT)] = bound.iter().map(|&p| pad_value(p)).fold(0.0, f32::max);
                        }
                        _ if bound.iter().any(|&p| !engine.silenced(p) && pad_value(p) >= STICK_PRESS) => apply(input, keymap_action(i)),
                        _ => {}
                    }
                    if bound.iter().any(|p| engine.tapped().contains(p)) {
                        self.pulses.push((player, i, now + TAP));
                    }
                }
                apply_bits(input, combos);
                let analog = tilt[1] - tilt[0];
                if analog.abs() > TILT_DEAD_ZONE {
                    input.tilt += analog;
                }
                if pad_value(PadInput::Button(Button::Mode)) >= STICK_PRESS {
                    input.buttons |= button::HOME;
                }
                // The controller's own motion sensors: its bound tilt
                // inputs, when in use, come first.
                if let Some(a) = self.motion_sensors.then(|| pads.accel(player)).flatten() {
                    let m = self.pad_motion[player].update(a, now);
                    input.shake |= m.shake;
                    if analog.abs() <= TILT_DEAD_ZONE {
                        input.tilt += m.tilt;
                    }
                }
            }
        }
        for &(player, action, _) in &self.pulses {
            apply(&mut out[player], keymap_action(action));
        }
        for (i, input) in out.iter_mut().enumerate() {
            input.connected = self.on[i];
            input.shake |= now < self.shake_until[i];
            input.tilt = input.tilt.clamp(-1.0, 1.0);
            if self.reverse_tilt {
                input.tilt = -input.tilt;
            }
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
    fn a_combination_plays_its_action_alone() {
        let mut i = Inputs::without_pads();
        let mut k = keymap::Keymap::default();
        // Left Ctrl shakes; Left Ctrl + X is A.
        k.take(keymap::A, vec![KeyCode::ControlLeft.into(), KeyCode::KeyX.into()]);
        i.set_keymap(k);
        let ms = Duration::from_millis;
        assert!(i.key(KeyCode::ControlLeft, true));
        assert_eq!(i.poll_at(ms(0))[0].shake, false, "Ctrl waits for the rest");
        assert!(i.key(KeyCode::KeyX, true));
        let s = i.poll_at(ms(20))[0];
        assert_eq!((s.buttons, s.shake), (button::A, false), "neither Ctrl's shake nor X's 2");
        i.key(KeyCode::ControlLeft, false);
        assert_eq!(i.poll_at(ms(40))[0].buttons, 0, "X stays taken until let go");
        i.key(KeyCode::KeyX, false);
        assert_eq!(i.poll_at(ms(60))[0].buttons, 0, "taken: no tap");
        // X tapped alone: its 2 still comes, for a moment.
        i.key(KeyCode::KeyX, true);
        assert_eq!(i.poll_at(ms(100))[0].buttons, 0);
        i.key(KeyCode::KeyX, false);
        assert_eq!(i.poll_at(ms(110))[0].buttons, button::TWO);
        assert_eq!(i.poll_at(ms(140))[0].buttons, button::TWO);
        assert_eq!(i.poll_at(ms(160))[0].buttons, 0);
        // Space (also 2) is not in a combination: at once.
        i.key(KeyCode::Space, true);
        assert_eq!(i.poll_at(ms(200))[0].buttons, button::TWO);
    }

    #[test]
    fn space_in_the_air_spins() {
        let mut i = Inputs::without_pads();
        let ms = Duration::from_millis;
        // Rising: Space shakes, and plays no 2.
        i.set_player(Some((0x8037_5C6C, 3.0)));
        i.key(KeyCode::Space, true);
        let s = i.poll_at(ms(0))[0];
        assert_eq!((s.buttons, s.shake), (0, true));
        assert!(i.poll_at(ms(100))[0].shake, "for a while");
        assert!(!i.poll_at(ms(200))[0].shake);
        i.key(KeyCode::Space, false);
        i.poll_at(ms(210));
        // X (also Jump) is not Space: 2 in the air.
        i.key(KeyCode::KeyX, true);
        assert_eq!(i.poll_at(ms(220))[0].buttons, button::TWO);
        i.key(KeyCode::KeyX, false);
        // On the ground Space jumps; switched off, it always does.
        i.set_player(Some((0x8037_0000, 0.0)));
        i.key(KeyCode::Space, true);
        assert_eq!(i.poll_at(ms(300))[0].buttons, button::TWO);
        i.key(KeyCode::Space, false);
        i.poll_at(ms(310));
        i.set_smart_spin(false);
        i.set_player(Some((0x8037_5C6C, 3.0)));
        i.key(KeyCode::Space, true);
        let s = i.poll_at(ms(400))[0];
        assert_eq!((s.buttons, s.shake), (button::TWO, false));
    }

    #[test]
    fn the_mouse_tilts_and_shakes() {
        let mut i = Inputs::without_pads();
        let ms = Duration::from_millis;
        i.pointer(600.0, 1280.0);
        i.poll_at(ms(0));
        i.pointer(640.0, 1280.0);
        assert_eq!(i.poll_at(ms(20))[0].tilt, 0.0, "off by default");
        i.set_mouse_gyro(true);
        i.poll_at(ms(40));
        i.pointer(680.0, 1280.0);
        assert_eq!(i.poll_at(ms(60))[0].tilt, 1.0);
        // Keys first.
        i.key(KeyCode::KeyQ, true);
        assert_eq!(i.poll_at(ms(80))[0].tilt, -1.0);
        i.key(KeyCode::KeyQ, false);
        // Back the other way soon: a shake, and the tilt ends.
        i.pointer(640.0, 1280.0);
        let s = i.poll_at(ms(100))[0];
        assert_eq!((s.tilt, s.shake), (0.0, true));
        // Reverse Tilt mirrors it all.
        i.set_reverse_tilt(true);
        i.key(KeyCode::KeyE, true);
        assert_eq!(i.poll_at(ms(400))[0].tilt, -1.0);
    }

    #[test]
    fn scripted_buttons() {
        let s = parse_buttons("2+right+shake").unwrap();
        assert_eq!(s.buttons, button::TWO | button::RIGHT);
        assert!(s.shake);
        assert!(parse_buttons("x").is_err());
    }
}
