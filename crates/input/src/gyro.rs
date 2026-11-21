//! Mouse as Gyro (after WitherBlitz's NSMB-Wii-Recompilation,
//! `virtual_remote.cpp` `ApplyMouseGyro`): the mouse as the remote's motion
//! while no controller is connected. A sideways swing (the larger of 4
//! pixels and 1% of the window's width since the last look, once a field)
//! tilts the remote that way for 5 s, long enough to ride a tilting
//! platform across; a swing back the other way within 0.3 s is a shake,
//! which also ends the tilt, so wiggling the mouse spins without leaving the
//! remote tilted.

use std::time::Duration;

/// The mouse is looked at once a field.
const LOOK: Duration = Duration::from_micros(16_683);
const TILT_FOR: Duration = Duration::from_secs(5);
const SHAKE_WITHIN: Duration = Duration::from_millis(300);

#[derive(Default)]
pub struct Gyro {
    /// The pointer's x at the last look, and when.
    last: Option<(f32, Duration)>,
    /// -1 left, 1 right, 0 none; until when.
    tilt: i8,
    tilt_until: Duration,
    /// The last swing's way, and when.
    swing: i8,
    swing_at: Duration,
}

/// What the mouse does to the remote now.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Motion {
    /// -1 left, 0, 1 right.
    pub tilt: f32,
    /// A shake starts.
    pub shake: bool,
}

impl Gyro {
    /// Off, or a controller took over: forget the mouse.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// `x`: the pointer's x in the window, `width` the window's width (both
    /// in pixels), at time `now`.
    pub fn update(&mut self, x: f32, width: f32, now: Duration) -> Motion {
        let mut shake = false;
        match self.last {
            Some((_, at)) if now < at + LOOK => {}
            Some((last_x, _)) => {
                if (x - last_x).abs() >= (width * 0.01).max(4.0) {
                    let way = if x < last_x { -1 } else { 1 };
                    if self.swing != 0 && way != self.swing && now < self.swing_at + SHAKE_WITHIN {
                        shake = true;
                        self.tilt = 0;
                    } else {
                        self.tilt = way;
                        self.tilt_until = now + TILT_FOR;
                    }
                    self.swing = way;
                    self.swing_at = now;
                }
                self.last = Some((x, now));
            }
            None => self.last = Some((x, now)),
        }
        let tilt = if self.tilt != 0 && now < self.tilt_until { f32::from(self.tilt) } else { 0.0 };
        Motion { tilt, shake }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    #[test]
    fn a_swing_tilts_for_five_seconds() {
        let mut g = Gyro::default();
        assert_eq!(g.update(500.0, 1280.0, ms(0)), Motion::default());
        // 12.8 px is 1% of the window: 10 is not enough.
        assert_eq!(g.update(510.0, 1280.0, ms(17)).tilt, 0.0);
        assert_eq!(g.update(530.0, 1280.0, ms(34)), Motion { tilt: 1.0, shake: false });
        assert_eq!(g.update(530.0, 1280.0, ms(4000)).tilt, 1.0);
        assert_eq!(g.update(530.0, 1280.0, ms(5100)).tilt, 0.0);
    }

    #[test]
    fn a_swing_back_soon_is_a_shake() {
        let mut g = Gyro::default();
        g.update(500.0, 400.0, ms(0));
        assert_eq!(g.update(490.0, 400.0, ms(17)).tilt, -1.0, "4 px at least on a small window");
        assert_eq!(g.update(500.0, 400.0, ms(100)), Motion { tilt: 0.0, shake: true }, "the shake ends the tilt");
        // Back again later: a tilt.
        assert_eq!(g.update(480.0, 400.0, ms(900)), Motion { tilt: -1.0, shake: false });
    }

    #[test]
    fn looked_at_once_a_field() {
        let mut g = Gyro::default();
        g.update(500.0, 1280.0, ms(0));
        // Moves within a field add up to one look.
        assert_eq!(g.update(505.0, 1280.0, ms(5)).tilt, 0.0);
        assert_eq!(g.update(510.0, 1280.0, ms(10)).tilt, 0.0);
        assert_eq!(g.update(515.0, 1280.0, ms(17)).tilt, 1.0);
    }
}
