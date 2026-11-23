//! A controller's own motion sensors as the remote's, after WitherBlitz's
//! NSMB-Wii-Recompilation (`virtual_remote.cpp` `ApplyControllerMotion`):
//! turned like a steering wheel it tilts the remote, and a sharp jolt is a
//! shake.
//!
//! Gravity is the accelerometer's slow part (each 60 Hz look moves it a
//! quarter of the way to the reading), so it needs no calibration: pitching
//! the controller towards the player does not tilt, and a turn shows up on
//! its x however it is pitched. What is left over beyond 13 m/s² is a shake, held 180 ms, long
//! enough for the game's four-field test. The turn's angle past a 4°
//! dead zone, up to 30°, is the tilt.

use std::time::Duration;

use crate::gyro::Motion;

/// Each look a 60th of a second after the last moves gravity this share of
/// the way to the reading (scaled for other gaps).
const GRAVITY_FOLLOW: f32 = 0.25;
const FIELD: f32 = 1.0 / 60.0;
/// m/s² of motion beyond gravity.
const SHAKE: f32 = 13.0;
const SHAKE_HOLD: Duration = Duration::from_millis(180);
/// A slightly crooked grip does nothing.
const DEAD_ZONE_DEG: f32 = 4.0;
/// The controller's angle that is a full remote tilt.
const FULL_DEG: f32 = 30.0;

#[derive(Default)]
pub struct PadMotion {
    gravity: Option<[f32; 3]>,
    last: Duration,
    shake_until: Duration,
}

impl PadMotion {
    /// Off, or the controller went away.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// `a`: the accelerometer in SDL's frame (m/s², x to the controller's
    /// right, y up out of its face), at time `now`.
    pub fn update(&mut self, a: [f32; 3], now: Duration) -> Motion {
        if a == [0.0; 3] {
            return Motion::default(); // nothing read yet
        }
        let g = match &mut self.gravity {
            None => self.gravity.insert(a),
            Some(g) => {
                let gap = now.saturating_sub(self.last).as_secs_f32().clamp(0.0, 0.25);
                let follow = 1.0 - (1.0 - GRAVITY_FOLLOW).powf(gap / FIELD);
                for i in 0..3 {
                    g[i] += (a[i] - g[i]) * follow;
                }
                g
            }
        };
        self.last = now;
        let moved: f32 = (0..3).map(|i| (a[i] - g[i]).powi(2)).sum();
        if moved > SHAKE * SHAKE {
            self.shake_until = now + SHAKE_HOLD;
        }
        let length = g.iter().map(|v| v * v).sum::<f32>().sqrt();
        let mut tilt = 0.0;
        if length > 1.0 {
            // Turned clockwise, the right side drops and x goes negative.
            let degrees = (-g[0] / length).clamp(-1.0, 1.0).asin().to_degrees();
            let past = (degrees.abs() - DEAD_ZONE_DEG).max(0.0);
            tilt = (past / (FULL_DEG - DEAD_ZONE_DEG)).min(1.0).copysign(degrees);
        }
        Motion { tilt, shake: now < self.shake_until }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: f32 = 9.80665;

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    /// Gravity as read with the controller turned `deg` clockwise (the
    /// right side down), pitched `pitch` towards the player.
    fn turned(deg: f32, pitch: f32) -> [f32; 3] {
        let (r, p) = (deg.to_radians(), pitch.to_radians());
        [-G * r.sin() * p.cos(), G * r.cos() * p.cos(), G * p.sin()]
    }

    #[test]
    fn turning_tilts() {
        let mut m = PadMotion::default();
        assert_eq!(m.update([0.0; 3], ms(0)), Motion::default(), "no reading yet");
        assert_eq!(m.update(turned(0.0, 0.0), ms(0)).tilt, 0.0);
        assert_eq!(m.update(turned(3.0, 0.0), ms(17)).tilt, 0.0, "in the dead zone");
        // Held at 17° (halfway between 4 and 30) long enough to settle.
        let mut t = 17;
        let mut last = Motion::default();
        for _ in 0..60 {
            t += 17;
            last = m.update(turned(17.0, 0.0), ms(t));
        }
        assert!((last.tilt - 0.5).abs() < 0.01, "{}", last.tilt);
        assert!(!last.shake);
        for _ in 0..60 {
            t += 17;
            last = m.update(turned(-45.0, 0.0), ms(t));
        }
        assert_eq!(last.tilt, -1.0, "past 30° is a full tilt");
    }

    #[test]
    fn pitch_alone_does_not_tilt() {
        let mut m = PadMotion::default();
        assert!(m.update(turned(0.0, 50.0), ms(0)).tilt.abs() < 1e-6);
        // A turn still shows, a little less the more it is pitched.
        let mut pitched = PadMotion::default();
        let mut flat = PadMotion::default();
        let p = pitched.update(turned(-20.0, 30.0), ms(0)).tilt;
        let f = flat.update(turned(-20.0, 0.0), ms(0)).tilt;
        assert!(f < p && p < 0.0, "{p} {f}");
    }

    #[test]
    fn a_jolt_shakes_for_a_while() {
        let mut m = PadMotion::default();
        m.update(turned(0.0, 0.0), ms(0));
        // A small nudge is not a shake.
        assert!(!m.update([5.0, G, 0.0], ms(17)).shake);
        m.update(turned(0.0, 0.0), ms(34));
        let jolt = m.update([0.0, G + 25.0, 0.0], ms(51));
        assert!(jolt.shake);
        assert!(m.update(turned(0.0, 0.0), ms(200)).shake, "held");
        assert!(!m.update(turned(0.0, 0.0), ms(240)).shake, "180 ms on");
    }

    #[test]
    fn gravity_follows_at_any_rate() {
        // Looks at 120 Hz settle the same as at 60 Hz.
        let mut slow = PadMotion::default();
        let mut fast = PadMotion::default();
        slow.update(turned(0.0, 0.0), ms(0));
        fast.update(turned(0.0, 0.0), Duration::ZERO);
        let a = turned(20.0, 0.0);
        let s = slow.update(a, Duration::from_secs_f32(FIELD)).tilt;
        fast.update(a, Duration::from_secs_f32(FIELD / 2.0));
        let f = fast.update(a, Duration::from_secs_f32(FIELD)).tilt;
        assert!((s - f).abs() < 1e-3, "{s} {f}");
    }
}
