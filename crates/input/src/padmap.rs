//! Which controller buttons play which of the remote's buttons, as the
//! Settings window's Keybinds tab sets them, kept in `settings.toml`'s
//! `[controller]` table by name with Xbox labels ("B, X"). The actions are
//! [`crate::keymap::ACTIONS`]; the guide button is always HOME.

use crate::combo;

/// A controller's buttons, by place on an Xbox-style controller (South is
/// Xbox's A, PlayStation's cross; the bumpers are LeftTrigger and
/// RightTrigger, the triggers LeftTrigger2 and RightTrigger2; Mode is the
/// guide button).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    South,
    East,
    West,
    North,
    LeftTrigger,
    RightTrigger,
    LeftTrigger2,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

/// A stick's axis; up and right are positive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}
use crate::keymap::{DOWN, LEFT, RIGHT, TILT_LEFT, TILT_RIGHT, UP};

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
    pub buttons: [Vec<PadInput>; crate::keymap::ACTIONS.len()],
    /// Each action's combinations: 2 or 3 held together (`combo`).
    pub combos: [Vec<Vec<PadInput>>; crate::keymap::ACTIONS.len()],
}

impl Default for Padmap {
    /// The layout from before buttons could be rebound: the D-pad and the
    /// left stick move, A jumps (2), B and X run (1), Y and the bumpers
    /// shake, the triggers and the right stick tilt, Menu is +, View is -.
    /// The right stick's click (RS) fast-forwards; both sticks' clicks
    /// together restart the course, which a press by accident never does.
    fn default() -> Self {
        let mut combos: [Vec<Vec<PadInput>>; crate::keymap::ACTIONS.len()] = Default::default();
        combos[crate::keymap::RESTART] = vec![vec![B(Button::LeftThumb), B(Button::RightThumb)]];
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
                vec![B(Button::RightThumb)],
                vec![],
            ],
            combos,
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

    /// An action's combinations by name (as saved).
    pub fn combo_names(&self, action: usize) -> Vec<Vec<&'static str>> {
        self.combos[action].iter().map(|c| c.iter().filter_map(|&m| name(m)).collect()).collect()
    }

    /// Sets an action's combinations from saved names; unknown or
    /// malformed ones are skipped, four at most.
    pub fn set_combo_names(&mut self, action: usize, combos: &[Vec<String>]) {
        self.combos[action] = combos.iter().filter_map(|names| combo::parse(names, by_name)).take(combo::MAX_PER_ACTION).collect();
    }

    /// What a Keybinds-tab capture picked for `action`: one alone is its
    /// only binding from now on; 2 or 3 together are a combination added to
    /// its others (taken from any action that had it).
    pub fn take(&mut self, action: usize, picked: Vec<PadInput>) {
        match picked.len() {
            0 => {}
            1 => self.buttons[action] = picked,
            _ => combo::add(&mut self.combos, action, picked),
        }
    }

    /// No bindings or combinations left for `action`.
    pub fn clear(&mut self, action: usize) {
        self.buttons[action].clear();
        self.combos[action].clear();
    }

    /// Part of any combination.
    pub fn in_combo(&self, c: PadInput) -> bool {
        self.combos.iter().flatten().any(|m| m.contains(&c))
    }

    /// An action's bindings as shown: each one, a combination's members
    /// joined by +, separated by /.
    pub fn display(&self, action: usize) -> Vec<Vec<PadInput>> {
        let singles = self.buttons[action].iter().map(|&c| vec![c]);
        singles.chain(self.combos[action].iter().cloned()).collect()
    }

    /// The rules' view of these bindings (`combo`): the combinations, every
    /// binding of its own, and the directions' and tilt's, which only gate.
    pub fn combo_setup(&self) -> combo::Setup<PadInput> {
        let mut setup = combo::Setup::default();
        for (action, list) in self.combos.iter().enumerate() {
            for members in list {
                setup.combos.push(combo::Combo { action, members: members.clone() });
            }
        }
        for (action, bound) in self.buttons.iter().enumerate() {
            for &c in bound {
                setup.singles.push(c);
                if matches!(action, UP | DOWN | LEFT | RIGHT | TILT_LEFT | TILT_RIGHT) {
                    setup.conditions.push(c);
                }
            }
        }
        setup
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
        p.take(SHAKE, vec![B(Button::LeftTrigger), B(Button::South)]);
        assert_eq!(p.combo_names(SHAKE), vec![vec!["LB", "A"]]);
    }
}
