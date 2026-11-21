//! Space Smart Spin (after WitherBlitz's NSMB-Wii-Recompilation,
//! `virtual_remote.cpp` `SmartSpinButton`, `game_menus.cpp`
//! `PlayerInSpinnableAir`): with Space as a Jump key, a press of Space while
//! the player is in the air is a shake (a spin) instead of 2; Space then
//! counts as no button until it is let go.
//!
//! - Rising: the spin at once.
//! - Falling: a press within a small Mario's height (16 units) of the ground
//!   is meant as the next jump. It waits 16 / the fall speed, plus a frame
//!   (the state shows a landing a frame late), 2 to 7 frames; landing
//!   meanwhile makes it a 2 press, else it is the spin.
//! - On a springboard a press is always 2, and a spin's Space still held
//!   when the player lands on one is 2 held there (the high bounce).

use std::time::Duration;

/// The player as the game last left them: their state's `StateID` and their
/// last frame's vertical move (units a frame, up is positive).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Player {
    pub state: u32,
    pub vertical: f32,
}

/// The air states: `dAcPy_c`'s and `daPlBase_c`'s Jump, Fall and SitJump,
/// WallJump (kicked off a wall) and Propel (the propeller suit's flight).
const AIR: [u32; 8] = [0x8037_5C6C, 0x8037_5CAC, 0x8037_5D6C, 0x8037_61B0, 0x8037_6230, 0x8035_429C, 0x8035_431C, 0x8035_42DC];
/// `daPlBase_c::StateID_JumpDai`: on a springboard.
const SPRINGBOARD: u32 = 0x8035_44DC;
/// A field.
const FRAME: f32 = 1.0 / 60.0;
const SMALL_MARIO: f32 = 16.0;
/// A 2 press after a landing lasts this long, held or not, so a field reads it.
const TWO_PULSE: Duration = Duration::from_millis(40);

impl Player {
    pub fn in_air(self) -> bool {
        AIR.contains(&self.state)
    }

    pub fn on_springboard(self) -> bool {
        self.state == SPRINGBOARD
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Is {
    #[default]
    Two,
    Spin,
    Waiting,
}

/// What Space is now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Space {
    /// Space plays 2 (when held, or for a moment after a press that landed).
    pub two: bool,
    /// Shake now: the spin starts.
    pub spin: bool,
}

#[derive(Default)]
pub struct SmartSpin {
    was_down: bool,
    is: Is,
    wait_until: Duration,
    two_until: Duration,
}

impl SmartSpin {
    /// `down`: Space held; `applies`: Smart Spin is on and Space is a Jump
    /// key; `player`: player 1 now, if in a course.
    pub fn update(&mut self, down: bool, applies: bool, player: Option<Player>, now: Duration) -> Space {
        let mut spin = false;
        let air = player.is_some_and(Player::in_air);
        if down && !self.was_down && applies {
            self.is = Is::Two;
            if let Some(p) = player.filter(|p| p.in_air()) {
                if p.vertical >= 0.0 {
                    self.is = Is::Spin;
                    spin = true;
                } else {
                    let frames = (SMALL_MARIO / -p.vertical + 1.0).clamp(2.0, 7.0);
                    self.is = Is::Waiting;
                    self.wait_until = now + Duration::from_secs_f32(frames * FRAME);
                }
            }
        } else if self.is == Is::Spin && down {
            if player.is_some_and(Player::on_springboard) {
                self.is = Is::Two;
            }
        } else if self.is == Is::Waiting {
            if !air {
                // Landed (or grabbed a wall): the press was the next jump.
                self.is = Is::Two;
                self.two_until = now + TWO_PULSE;
            } else if now >= self.wait_until {
                self.is = Is::Spin;
                spin = true;
            }
        }
        if !down && self.is != Is::Waiting {
            self.is = Is::Two;
        }
        self.was_down = down;
        Space { two: self.is == Is::Two && (down || now < self.two_until), spin }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JUMP: u32 = 0x8037_5C6C;
    const WALK: u32 = 0x8037_0000;

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    fn p(state: u32, vertical: f32) -> Option<Player> {
        Some(Player { state, vertical })
    }

    #[test]
    fn on_the_ground_space_jumps() {
        let mut s = SmartSpin::default();
        assert_eq!(s.update(true, true, p(WALK, 0.0), ms(0)), Space { two: true, spin: false });
        assert_eq!(s.update(true, true, p(JUMP, 3.0), ms(16)), Space { two: true, spin: false }, "held into the air: still 2");
        assert_eq!(s.update(false, true, p(JUMP, 3.0), ms(32)).two, false);
    }

    #[test]
    fn rising_spins_at_once_and_space_is_then_no_button() {
        let mut s = SmartSpin::default();
        assert_eq!(s.update(true, true, p(JUMP, 2.0), ms(0)), Space { two: false, spin: true });
        assert_eq!(s.update(true, true, p(JUMP, 1.0), ms(16)), Space { two: false, spin: false });
        // Let go and pressed again on the ground: 2.
        s.update(false, true, p(WALK, 0.0), ms(32));
        assert!(s.update(true, true, p(WALK, 0.0), ms(48)).two);
    }

    #[test]
    fn falling_waits_to_see_if_the_player_lands() {
        // 4 units a frame: 16 / 4 + 1 = 5 frames.
        let mut s = SmartSpin::default();
        assert_eq!(s.update(true, true, p(JUMP, -4.0), ms(0)), Space { two: false, spin: false });
        assert_eq!(s.update(true, true, p(JUMP, -4.0), ms(50)), Space { two: false, spin: false });
        assert_eq!(s.update(true, true, p(JUMP, -4.0), ms(84)), Space { two: false, spin: true }, "still in the air: the spin");
        // Landing within the wait: a 2 press, even once let go.
        let mut s = SmartSpin::default();
        s.update(true, true, p(JUMP, -4.0), ms(0));
        s.update(false, true, p(JUMP, -4.0), ms(20));
        assert_eq!(s.update(false, true, p(WALK, 0.0), ms(40)), Space { two: true, spin: false });
        assert!(s.update(false, true, p(WALK, 0.0), ms(70)).two, "for a moment");
        assert!(!s.update(false, true, p(WALK, 0.0), ms(90)).two);
    }

    #[test]
    fn springboards_and_switched_off() {
        let mut s = SmartSpin::default();
        assert!(s.update(true, true, p(SPRINGBOARD, 0.0), ms(0)).two, "on a springboard: 2");
        let mut s = SmartSpin::default();
        assert!(s.update(true, true, p(JUMP, 1.0), ms(0)).spin);
        assert!(s.update(true, true, p(SPRINGBOARD, 0.0), ms(16)).two, "spun onto a springboard, held: 2 held");
        let mut s = SmartSpin::default();
        assert_eq!(s.update(true, false, p(JUMP, 1.0), ms(0)), Space { two: true, spin: false }, "off: Space is 2");
        assert!(s.update(true, true, None, ms(16)).two, "no player: 2");
    }
}
