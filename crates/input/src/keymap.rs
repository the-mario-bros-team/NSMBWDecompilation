//! Which keys play which of the remote's buttons, as the Settings window's
//! Keybinds tab sets them (after WitherBlitz's NSMB-Wii-Recompilation,
//! `runtime/src/keybinds.cpp`), kept in `settings.toml`'s `[keyboard]`
//! table by name ("X, Space"). The mouse's buttons are keys too ("Mouse R").

use winit::keyboard::KeyCode;

/// The actions, in the Keybinds tab's order: their `[keyboard]` key, the
/// name shown, and their default keys.
pub const ACTIONS: [(&str, &str); 12] = [
    ("up", "Up"),
    ("down", "Down"),
    ("left", "Left"),
    ("right", "Right"),
    ("jump", "Jump (2)"),
    ("run", "Run / Fire (1)"),
    ("a", "A"),
    ("shake", "Shake (Spin)"),
    ("tilt_left", "Tilt Left"),
    ("tilt_right", "Tilt Right"),
    ("plus", "+ (Pause)"),
    ("minus", "- (Minus)"),
];

pub const UP: usize = 0;
pub const DOWN: usize = 1;
pub const LEFT: usize = 2;
pub const RIGHT: usize = 3;
pub const JUMP: usize = 4;
pub const RUN: usize = 5;
pub const A: usize = 6;
pub const SHAKE: usize = 7;
pub const TILT_LEFT: usize = 8;
pub const TILT_RIGHT: usize = 9;
pub const PLUS: usize = 10;
pub const MINUS: usize = 11;

/// Keys a binding can be, with their names (as saved and shown). F10 and
/// F11 open Settings and switch fullscreen; Home is the HOME button.
const NAMES: &[(KeyCode, &str)] = &[
    (KeyCode::ArrowUp, "Up"),
    (KeyCode::ArrowDown, "Down"),
    (KeyCode::ArrowLeft, "Left"),
    (KeyCode::ArrowRight, "Right"),
    (KeyCode::Space, "Space"),
    (KeyCode::Enter, "Enter"),
    (KeyCode::Escape, "Esc"),
    (KeyCode::Tab, "Tab"),
    (KeyCode::Backspace, "Backspace"),
    (KeyCode::ShiftLeft, "Left Shift"),
    (KeyCode::ShiftRight, "Right Shift"),
    (KeyCode::ControlLeft, "Left Ctrl"),
    (KeyCode::ControlRight, "Right Ctrl"),
    (KeyCode::AltLeft, "Left Alt"),
    (KeyCode::AltRight, "Right Alt"),
    (KeyCode::Minus, "Minus"),
    (KeyCode::Equal, "Equals"),
    (KeyCode::BracketLeft, "["),
    (KeyCode::BracketRight, "]"),
    (KeyCode::Backslash, "\\"),
    (KeyCode::Semicolon, ";"),
    (KeyCode::Quote, "'"),
    (KeyCode::Comma, ","),
    (KeyCode::Period, "."),
    (KeyCode::Slash, "/"),
    (KeyCode::Backquote, "`"),
    (KeyCode::Insert, "Insert"),
    (KeyCode::Delete, "Delete"),
    (KeyCode::End, "End"),
    (KeyCode::PageUp, "Page Up"),
    (KeyCode::PageDown, "Page Down"),
    (KeyCode::KeyA, "A"),
    (KeyCode::KeyB, "B"),
    (KeyCode::KeyC, "C"),
    (KeyCode::KeyD, "D"),
    (KeyCode::KeyE, "E"),
    (KeyCode::KeyF, "F"),
    (KeyCode::KeyG, "G"),
    (KeyCode::KeyH, "H"),
    (KeyCode::KeyI, "I"),
    (KeyCode::KeyJ, "J"),
    (KeyCode::KeyK, "K"),
    (KeyCode::KeyL, "L"),
    (KeyCode::KeyM, "M"),
    (KeyCode::KeyN, "N"),
    (KeyCode::KeyO, "O"),
    (KeyCode::KeyP, "P"),
    (KeyCode::KeyQ, "Q"),
    (KeyCode::KeyR, "R"),
    (KeyCode::KeyS, "S"),
    (KeyCode::KeyT, "T"),
    (KeyCode::KeyU, "U"),
    (KeyCode::KeyV, "V"),
    (KeyCode::KeyW, "W"),
    (KeyCode::KeyX, "X"),
    (KeyCode::KeyY, "Y"),
    (KeyCode::KeyZ, "Z"),
    (KeyCode::Digit0, "0"),
    (KeyCode::Digit1, "1"),
    (KeyCode::Digit2, "2"),
    (KeyCode::Digit3, "3"),
    (KeyCode::Digit4, "4"),
    (KeyCode::Digit5, "5"),
    (KeyCode::Digit6, "6"),
    (KeyCode::Digit7, "7"),
    (KeyCode::Digit8, "8"),
    (KeyCode::Digit9, "9"),
    (KeyCode::Numpad0, "Keypad 0"),
    (KeyCode::Numpad1, "Keypad 1"),
    (KeyCode::Numpad2, "Keypad 2"),
    (KeyCode::Numpad3, "Keypad 3"),
    (KeyCode::Numpad4, "Keypad 4"),
    (KeyCode::Numpad5, "Keypad 5"),
    (KeyCode::Numpad6, "Keypad 6"),
    (KeyCode::Numpad7, "Keypad 7"),
    (KeyCode::Numpad8, "Keypad 8"),
    (KeyCode::Numpad9, "Keypad 9"),
    (KeyCode::NumpadAdd, "Keypad +"),
    (KeyCode::NumpadSubtract, "Keypad -"),
    (KeyCode::NumpadMultiply, "Keypad *"),
    (KeyCode::NumpadDivide, "Keypad /"),
    (KeyCode::NumpadDecimal, "Keypad ."),
    (KeyCode::NumpadEnter, "Keypad Enter"),
    (KeyCode::F1, "F1"),
    (KeyCode::F2, "F2"),
    (KeyCode::F3, "F3"),
    (KeyCode::F4, "F4"),
    (KeyCode::F5, "F5"),
    (KeyCode::F6, "F6"),
    (KeyCode::F7, "F7"),
    (KeyCode::F8, "F8"),
    (KeyCode::F9, "F9"),
    (KeyCode::F12, "F12"),
];

/// The mouse's buttons 1-5, as saved and shown.
const MOUSE_NAMES: [&str; 5] = ["Mouse L", "Mouse M", "Mouse R", "Mouse 4", "Mouse 5"];

/// A key, or one of the mouse's buttons (1 left, 2 middle, 3 right, 4 and
/// 5 the side buttons).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Code(KeyCode),
    Mouse(u8),
}

impl From<KeyCode> for Key {
    fn from(k: KeyCode) -> Self {
        Key::Code(k)
    }
}

pub fn key_name(k: impl Into<Key>) -> Option<&'static str> {
    match k.into() {
        Key::Code(k) => NAMES.iter().find(|(c, _)| *c == k).map(|(_, n)| *n),
        Key::Mouse(n) => MOUSE_NAMES.get(usize::from(n).checked_sub(1)?).copied(),
    }
}

pub fn key_by_name(name: &str) -> Option<Key> {
    let name = name.trim();
    if let Some(i) = MOUSE_NAMES.iter().position(|n| n.eq_ignore_ascii_case(name)) {
        return Some(Key::Mouse(i as u8 + 1));
    }
    NAMES.iter().find(|(_, n)| n.eq_ignore_ascii_case(name)).map(|(c, _)| Key::Code(*c))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keymap {
    /// Each action's keys ([`ACTIONS`] order).
    pub keys: [Vec<Key>; 12],
}

impl Default for Keymap {
    /// Right click crouches, as in WitherBlitz's port.
    fn default() -> Self {
        use KeyCode::*;
        let k = |codes: &[KeyCode]| codes.iter().map(|&c| Key::Code(c)).collect::<Vec<_>>();
        Self {
            keys: [
                k(&[ArrowUp]),
                vec![Key::Code(ArrowDown), Key::Mouse(3)],
                k(&[ArrowLeft]),
                k(&[ArrowRight]),
                k(&[KeyX, Space]),
                k(&[KeyZ, ShiftLeft]),
                k(&[KeyB]),
                k(&[KeyC, ControlLeft]),
                k(&[KeyQ]),
                k(&[KeyE]),
                k(&[Enter, Escape]),
                k(&[Minus, Tab]),
            ],
        }
    }
}

impl Keymap {
    /// The action `k` plays, if any.
    pub fn action(&self, k: impl Into<Key>) -> Option<usize> {
        let k = k.into();
        self.keys.iter().position(|keys| keys.contains(&k))
    }

    /// An action's keys as shown and saved: "X, Space"; "" for none.
    pub fn label(&self, action: usize) -> String {
        self.keys[action].iter().filter_map(|&k| key_name(k)).collect::<Vec<_>>().join(", ")
    }

    /// Sets an action's keys from a saved label; unknown names are skipped.
    pub fn set_label(&mut self, action: usize, label: &str) {
        self.keys[action] = label.split(',').filter_map(key_by_name).collect();
    }

    /// `k` alone plays `action` from now on (other actions keep theirs).
    pub fn bind(&mut self, action: usize, k: impl Into<Key>) {
        self.keys[action] = vec![k.into()];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_round_trip() {
        let mut k = Keymap::default();
        assert_eq!(k.label(JUMP), "X, Space");
        assert_eq!(k.label(PLUS), "Enter, Esc");
        assert_eq!(k.label(DOWN), "Down, Mouse R");
        k.bind(A, Key::Mouse(4));
        k.bind(RUN, KeyCode::Backslash);
        k.keys[SHAKE].clear();
        let mut back = Keymap::default();
        for i in 0..ACTIONS.len() {
            back.set_label(i, &k.label(i));
        }
        assert_eq!(back, k);
        assert_eq!(k.action(KeyCode::Backslash), Some(RUN));
        assert_eq!(k.action(KeyCode::KeyZ), None);
        assert_eq!(key_by_name(" left shift "), Some(Key::Code(KeyCode::ShiftLeft)));
        assert_eq!(k.action(Key::Mouse(4)), Some(A));
        assert_eq!(key_by_name("mouse 5"), Some(Key::Mouse(5)));
        assert_eq!(key_name(Key::Mouse(0)), None);
    }
}
