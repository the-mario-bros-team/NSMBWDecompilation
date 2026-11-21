//! Which controller buttons play which of the remote's buttons, as the
//! Settings window's Keybinds tab sets them, kept in `settings.toml`'s
//! `[controller]` table by name with Xbox labels ("B, X"). The actions are
//! [`crate::keymap::ACTIONS`]; the guide button is always HOME.

use gilrs::{Axis, Button};

/// One thing on a controller a binding can be.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PadInput {
    Button(Button),
    /// A stick pushed one way: its axis, and whether towards the axis's
    /// positive end (right, or up).
    Stick(Axis, bool),
}

use PadInput::{Button as B, Stick as S};

/// The names, as saved and shown (Xbox labels).
const NAMES: &[(PadInput, &str)] = &[
    (B(Button::South), "A"),
    (B(Button::East), "B"),
    (B(Button::West), "X"),
    (B(Button::North), "Y"),
    (B(Button::LeftTrigger), "LB"),
    (B(Button::RightTrigger), "RB"),
    (B(Button::LeftTrigger2), "LT"),
    (B(Button::RightTrigger2), "RT"),
    (B(Button::Select), "View"),
    (B(Button::Start), "Menu"),
    (B(Button::LeftThumb), "LS"),
    (B(Button::RightThumb), "RS"),
    (B(Button::DPadUp), "D-Pad Up"),
    (B(Button::DPadDown), "D-Pad Down"),
    (B(Button::DPadLeft), "D-Pad Left"),
    (B(Button::DPadRight), "D-Pad Right"),
    (S(Axis::LeftStickY, true), "Left Stick Up"),
    (S(Axis::LeftStickY, false), "Left Stick Down"),
    (S(Axis::LeftStickX, false), "Left Stick Left"),
    (S(Axis::LeftStickX, true), "Left Stick Right"),
    (S(Axis::RightStickY, true), "Right Stick Up"),
    (S(Axis::RightStickY, false), "Right Stick Down"),
    (S(Axis::RightStickX, false), "Right Stick Left"),
    (S(Axis::RightStickX, true), "Right Stick Right"),
];

/// Everything bindable, in the names' order.
pub fn all() -> impl Iterator<Item = PadInput> {
    NAMES.iter().map(|(p, _)| *p)
}

pub fn name(p: PadInput) -> Option<&'static str> {
    NAMES.iter().find(|(c, _)| *c == p).map(|(_, n)| *n)
}

pub fn by_name(name: &str) -> Option<PadInput> {
    NAMES.iter().find(|(_, n)| n.eq_ignore_ascii_case(name.trim())).map(|(c, _)| *c)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Padmap {
    /// Each action's buttons ([`crate::keymap::ACTIONS`] order).
    pub buttons: [Vec<PadInput>; 12],
}

impl Default for Padmap {
    /// The layout from before buttons could be rebound: the D-pad and the
    /// left stick move, A jumps (2), B and X run (1), Y and the bumpers
    /// shake, the triggers and the right stick tilt, Menu is +, View is -.
    fn default() -> Self {
        Self {
            buttons: [
                vec![B(Button::DPadUp), S(Axis::LeftStickY, true)],
                vec![B(Button::DPadDown), S(Axis::LeftStickY, false)],
                vec![B(Button::DPadLeft), S(Axis::LeftStickX, false)],
                vec![B(Button::DPadRight), S(Axis::LeftStickX, true)],
                vec![B(Button::South)],
                vec![B(Button::East), B(Button::West)],
                vec![],
                vec![B(Button::North), B(Button::LeftTrigger), B(Button::RightTrigger)],
                vec![B(Button::LeftTrigger2), S(Axis::RightStickX, false)],
                vec![B(Button::RightTrigger2), S(Axis::RightStickX, true)],
                vec![B(Button::Start)],
                vec![B(Button::Select)],
            ],
        }
    }
}

impl Padmap {
    /// An action's buttons as shown and saved: "B, X"; "" for none.
    pub fn label(&self, action: usize) -> String {
        self.buttons[action].iter().filter_map(|&p| name(p)).collect::<Vec<_>>().join(", ")
    }

    /// Sets an action's buttons from a saved label; unknown names are skipped.
    pub fn set_label(&mut self, action: usize, label: &str) {
        self.buttons[action] = label.split(',').filter_map(by_name).collect();
    }

    /// `p` alone plays `action` from now on (other actions keep theirs).
    pub fn bind(&mut self, action: usize, p: PadInput) {
        self.buttons[action] = vec![p];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keymap::{ACTIONS, JUMP, RUN, SHAKE, TILT_LEFT};

    #[test]
    fn labels_round_trip() {
        let mut p = Padmap::default();
        assert_eq!(p.label(RUN), "B, X");
        assert_eq!(p.label(TILT_LEFT), "LT, Right Stick Left");
        p.bind(JUMP, B(Button::RightThumb));
        p.buttons[SHAKE].clear();
        let mut back = Padmap::default();
        for i in 0..ACTIONS.len() {
            back.set_label(i, &p.label(i));
        }
        assert_eq!(back, p);
        assert_eq!(by_name(" left stick up "), Some(S(Axis::LeftStickY, true)));
        assert!(all().all(|i| by_name(name(i).unwrap()) == Some(i)));
    }
}
