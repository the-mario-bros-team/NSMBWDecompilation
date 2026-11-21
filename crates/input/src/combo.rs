//! Combination bindings: two or three keys (or a controller's buttons) held
//! together play one action, pressed in any order. The rules alone, with the
//! time passed in, so they can be tested on their own. After WitherBlitz's
//! NSMB-Wii-Recompilation (`combo_engine.h`):
//!
//! 1. A member with no binding of its own is a modifier: alone it does
//!    nothing, and it may be held any time before the rest.
//! 2. A member that also has a binding of its own (a single) waits [`WAIT`]
//!    from its press for the rest of a combination. Completed in time, the
//!    single never happens; otherwise the single starts then, and that member
//!    takes no part in a combination until it is let go (holding Run, then
//!    pressing Jump, still runs and jumps when Run + Jump is a combination).
//!    A tap let go within the wait, with no combination, still counts once
//!    ([`Engine::tapped`]).
//! 3. While a combination is held its members are taken: their singles stay
//!    silent until each is let go, even when the combination breaks first.
//! 4. Of two combinations held where one's members are all in the other's,
//!    the longer wins.
//! 5. Conditions (the directions and tilt) never wait and are never taken:
//!    they keep doing their own thing and only gate a combination (Down +
//!    Jump neither delays nor swallows Down).

use std::time::Duration;

/// How long a single waits for the rest of a combination (3 fields).
pub const WAIT: Duration = Duration::from_millis(50);
/// Members of one combination, at most.
pub const MAX_MEMBERS: usize = 3;
/// Combinations of one action, at most.
pub const MAX_PER_ACTION: usize = 4;

/// Two combinations with the same members, in any order.
pub fn same<C: PartialEq>(a: &[C], b: &[C]) -> bool {
    a.len() == b.len() && a.iter().all(|m| b.contains(m))
}

/// `members` becomes one of `action`'s combinations, taken from any action
/// that had it; past [`MAX_PER_ACTION`] the oldest makes room.
pub fn add<C: PartialEq>(combos: &mut [Vec<Vec<C>>], action: usize, members: Vec<C>) {
    for list in combos.iter_mut() {
        list.retain(|c| !same(c, &members));
    }
    let list = &mut combos[action];
    if list.len() >= MAX_PER_ACTION {
        list.remove(0);
    }
    list.push(members);
}

/// A saved combination's members by name: kept if all are known, there are
/// 2 or 3 of them, and they differ.
pub fn parse<C: PartialEq>(names: &[String], by_name: impl Fn(&str) -> Option<C>) -> Option<Vec<C>> {
    let members: Vec<C> = names.iter().map(|n| by_name(n)).collect::<Option<_>>()?;
    let distinct = members.iter().enumerate().all(|(i, m)| !members[..i].contains(m));
    ((2..=MAX_MEMBERS).contains(&members.len()) && distinct).then_some(members)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Combo<C> {
    pub action: usize,
    /// 2 or 3 codes.
    pub members: Vec<C>,
}

/// What the engine knows of the bindings.
#[derive(Clone, Debug)]
pub struct Setup<C> {
    pub combos: Vec<Combo<C>>,
    /// Codes with a binding of their own.
    pub singles: Vec<C>,
    /// Codes that only gate a combination (directions, tilt).
    pub conditions: Vec<C>,
}

impl<C> Default for Setup<C> {
    fn default() -> Self {
        Self { combos: Vec::new(), singles: Vec::new(), conditions: Vec::new() }
    }
}

/// A member held now.
#[derive(Clone, Copy, Debug)]
struct Held<C> {
    code: C,
    since: Duration,
    /// Taken by a combination until let go.
    taken: bool,
    /// Its single started: out of combinations until let go.
    committed: bool,
    /// Silenced, waiting for a combination.
    waiting: bool,
}

#[derive(Clone, Debug)]
pub struct Engine<C> {
    setup: Setup<C>,
    held: Vec<Held<C>>,
    actions: u32,
    silenced: Vec<C>,
    tapped: Vec<C>,
}

impl<C: Copy + PartialEq> Default for Engine<C> {
    fn default() -> Self {
        Self::new(Setup::default())
    }
}

impl<C: Copy + PartialEq> Engine<C> {
    /// An engine for `setup`; a combination's repeated members count once,
    /// and one left with fewer than two is dropped.
    pub fn new(mut setup: Setup<C>) -> Self {
        for combo in &mut setup.combos {
            let mut unique: Vec<C> = Vec::new();
            for &m in &combo.members {
                if !unique.contains(&m) {
                    unique.push(m);
                }
            }
            combo.members = unique;
        }
        setup.combos.retain(|c| c.members.len() >= 2);
        Self { setup, held: Vec::new(), actions: 0, silenced: Vec::new(), tapped: Vec::new() }
    }

    /// No combinations: nothing to do.
    pub fn is_empty(&self) -> bool {
        self.setup.combos.is_empty()
    }

    /// Everything let go (focus lost, a capture began).
    pub fn reset(&mut self) {
        self.held.clear();
        self.actions = 0;
        self.silenced.clear();
        self.tapped.clear();
    }

    fn is_member(&self, code: C) -> bool {
        self.setup.combos.iter().any(|c| c.members.contains(&code))
    }

    fn is_condition(&self, code: C) -> bool {
        self.setup.conditions.contains(&code)
    }

    fn find(&self, code: C) -> Option<&Held<C>> {
        self.held.iter().find(|h| h.code == code)
    }

    /// A combination with `code` that none of its other (non-condition)
    /// members' singles have taken out.
    fn can_still_complete(&self, code: C) -> bool {
        self.setup.combos.iter().filter(|c| c.members.contains(&code)).any(|c| {
            !c.members.iter().any(|&m| m != code && !self.is_condition(m) && self.find(m).is_some_and(|h| h.committed))
        })
    }

    /// `held`: every code held now (others are ignored), at time `now`.
    /// Returns the actions held through combinations, as `1 << action` bits.
    pub fn step(&mut self, held: &[C], now: Duration) -> u32 {
        // Let go while still waiting: a tap.
        self.tapped = self.held.iter().filter(|h| h.waiting && !held.contains(&h.code)).map(|h| h.code).collect();
        self.held.retain(|h| held.contains(&h.code));
        for &code in held {
            if self.is_member(code) && self.find(code).is_none() {
                self.held.push(Held { code, since: now, taken: false, committed: false, waiting: false });
            }
        }
        // The combinations held, none of their members committed to its single.
        let active: Vec<usize> = (0..self.setup.combos.len())
            .filter(|&i| self.setup.combos[i].members.iter().all(|&m| self.find(m).is_some_and(|h| !h.committed || self.is_condition(m))))
            .collect();
        self.actions = 0;
        for &i in &active {
            let combo = &self.setup.combos[i];
            let beaten = active.iter().any(|&j| {
                let other = &self.setup.combos[j];
                j != i && other.members.len() > combo.members.len() && combo.members.iter().all(|m| other.members.contains(m))
            });
            if beaten {
                continue;
            }
            self.actions |= 1 << combo.action;
            for m in combo.members.clone() {
                if !self.is_condition(m) {
                    if let Some(h) = self.held.iter_mut().find(|h| h.code == m) {
                        h.taken = true;
                    }
                }
            }
        }
        // Singles wait while a combination can still complete; then they start.
        self.silenced.clear();
        for i in 0..self.held.len() {
            let h = self.held[i];
            self.held[i].waiting = false;
            if self.is_condition(h.code) {
                continue;
            }
            if h.taken {
                self.silenced.push(h.code);
                continue;
            }
            if h.committed || !self.setup.singles.contains(&h.code) {
                continue;
            }
            if now.saturating_sub(h.since) < WAIT && self.can_still_complete(h.code) {
                self.silenced.push(h.code);
                self.held[i].waiting = true;
            } else {
                self.held[i].committed = true;
            }
        }
        self.actions
    }

    /// The actions held through combinations at the last step.
    pub fn actions(&self) -> u32 {
        self.actions
    }

    /// Whether `code`'s own binding is silent now (taken, or waiting).
    pub fn silenced(&self, code: C) -> bool {
        self.silenced.contains(&code)
    }

    /// Codes let go at the last step while they waited: their own binding
    /// counts as pressed once.
    pub fn tapped(&self) -> &[C] {
        &self.tapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Codes: 1 Ctrl (no binding of its own), 2 X (Jump), 3 Z (Run),
    // 4 Down (a direction), 5 Shift (Run).
    const CTRL: u32 = 1;
    const X: u32 = 2;
    const Z: u32 = 3;
    const DOWN: u32 = 4;
    const SHIFT: u32 = 5;
    // Actions.
    const JUMP: usize = 4;
    const A: usize = 6;
    const SHAKE: usize = 7;
    const PLUS: usize = 10;

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    fn engine(combos: &[(usize, &[u32])]) -> Engine<u32> {
        Engine::new(Setup {
            combos: combos.iter().map(|(a, m)| Combo { action: *a, members: m.to_vec() }).collect(),
            singles: vec![X, Z, DOWN, SHIFT],
            conditions: vec![DOWN],
        })
    }

    #[test]
    fn a_modifier_then_a_single() {
        let mut e = engine(&[(A, &[CTRL, X])]);
        assert_eq!(e.step(&[CTRL], ms(0)), 0);
        assert!(!e.silenced(CTRL), "Ctrl has no single to silence");
        // Ctrl held a long time, then X: the combination; X's Jump is silent.
        assert_eq!(e.step(&[CTRL, X], ms(500)), 1 << A);
        assert!(e.silenced(X));
        // Ctrl let go: X stays taken until it is let go too.
        assert_eq!(e.step(&[X], ms(600)), 0);
        assert!(e.silenced(X));
        assert_eq!(e.step(&[], ms(700)), 0);
        assert!(e.tapped().is_empty(), "taken, not tapped");
        // X alone waits, then jumps.
        e.step(&[X], ms(800));
        assert!(e.silenced(X), "waiting: a combination could still complete");
        e.step(&[X], ms(860));
        assert!(!e.silenced(X), "the wait is over: X jumps");
        // Ctrl now: no combination, X has committed.
        assert_eq!(e.step(&[CTRL, X], ms(900)), 0);
        assert!(!e.silenced(X));
    }

    #[test]
    fn the_single_first_within_the_wait() {
        let mut e = engine(&[(A, &[CTRL, X])]);
        e.step(&[X], ms(0));
        assert!(e.silenced(X));
        assert_eq!(e.step(&[X, CTRL], ms(30)), 1 << A);
        assert!(e.silenced(X), "Jump never happened");
    }

    #[test]
    fn holding_run_then_jump_does_both() {
        let mut e = engine(&[(SHAKE, &[Z, X])]);
        e.step(&[Z], ms(0));
        e.step(&[Z], ms(60));
        assert!(!e.silenced(Z), "Run has started");
        assert_eq!(e.step(&[Z, X], ms(100)), 0, "Run committed: no combination");
        assert!(!e.silenced(X), "no combination could complete: X isn't held back");
    }

    #[test]
    fn conditions_never_wait_and_are_never_taken() {
        let mut e = engine(&[(PLUS, &[DOWN, X])]);
        e.step(&[DOWN], ms(0));
        assert!(!e.silenced(DOWN), "never waits");
        assert_eq!(e.step(&[DOWN, X], ms(200)), 1 << PLUS);
        assert!(!e.silenced(DOWN), "never taken");
        assert!(e.silenced(X));
    }

    #[test]
    fn the_longer_combination_wins() {
        let mut e = engine(&[(A, &[CTRL, X]), (SHAKE, &[CTRL, SHIFT, X])]);
        e.step(&[CTRL], ms(0));
        e.step(&[CTRL, SHIFT], ms(10));
        assert_eq!(e.step(&[CTRL, SHIFT, X], ms(20)), 1 << SHAKE);
        // Shift let go: the shorter one is all that is held now.
        assert_eq!(e.step(&[CTRL, X], ms(30)), 1 << A);
    }

    #[test]
    fn a_tap_still_counts_once() {
        let mut e = engine(&[(A, &[CTRL, X])]);
        e.step(&[X], ms(0));
        e.step(&[], ms(20));
        assert_eq!(e.tapped(), &[X]);
        e.step(&[], ms(40));
        assert!(e.tapped().is_empty(), "once");
    }

    #[test]
    fn repeated_members_count_once() {
        let e = engine(&[(A, &[CTRL, CTRL]), (JUMP, &[CTRL, X, X])]);
        assert_eq!(e.setup.combos.len(), 1);
        assert_eq!(e.setup.combos[0].members, vec![CTRL, X]);
    }

    #[test]
    fn combinations_move_between_actions_four_at_most() {
        let mut combos: Vec<Vec<Vec<u32>>> = vec![Vec::new(); 12];
        add(&mut combos, A, vec![CTRL, X]);
        add(&mut combos, JUMP, vec![X, CTRL]);
        assert!(combos[A].is_empty(), "the same one, in another order: moved");
        assert_eq!(combos[JUMP], vec![vec![X, CTRL]]);
        for m in [Z, DOWN, SHIFT, 9] {
            add(&mut combos, JUMP, vec![CTRL, m]);
        }
        assert_eq!(combos[JUMP].len(), MAX_PER_ACTION);
        assert_eq!(combos[JUMP][0], vec![CTRL, Z], "the oldest made room");
        let by_name = |n: &str| n.parse::<u32>().ok();
        let names = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(parse(&names(&["1", "2"]), by_name), Some(vec![1, 2]));
        assert_eq!(parse(&names(&["1"]), by_name), None);
        assert_eq!(parse(&names(&["1", "1"]), by_name), None);
        assert_eq!(parse(&names(&["1", "x"]), by_name), None);
        assert_eq!(parse(&names(&["1", "2", "3", "4"]), by_name), None);
    }

    #[test]
    fn the_same_presses_give_the_same_results() {
        let run = || {
            let mut e = engine(&[(A, &[CTRL, X]), (SHAKE, &[Z, X])]);
            let presses: [&[u32]; 6] = [&[Z], &[Z, X], &[X], &[CTRL, X], &[CTRL], &[]];
            presses.iter().enumerate().map(|(i, held)| (e.step(held, ms(i as u64 * 20)), e.silenced(X), e.silenced(Z))).collect::<Vec<_>>()
        };
        assert_eq!(run(), run());
    }
}
