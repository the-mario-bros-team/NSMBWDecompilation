//! Time-stretching for game speeds other than 100% (the Practice tab): the
//! game makes its sound `speed` times as fast as real time, and this makes
//! it last real time again at its own pitch, by WSOLA (waveform-similarity
//! overlap-add). Windows of 32 ms are taken from the input `speed` times
//! further apart than they are laid down (half a window apart), each moved
//! by up to a quarter window to where it best continues the one before, and
//! cross-faded with Hann windows, which sum to one. At 1.0 it gives back its
//! input (after the first half window, which fades in), a window and a
//! search's reach behind what has come in.

/// Stereo frames.
type Frame = [f32; 2];

#[derive(Default)]
pub struct Stretch {
    rate: u32,
    /// Window length (frames; even), and the search's reach either way.
    n: usize,
    reach: usize,
    window: Vec<f32>,
    /// Input not yet used up.
    input: Vec<Frame>,
    /// Where the next window is ideally taken from (in `input`).
    pos: f64,
    /// What naturally follows the last window taken (its start plus half
    /// a window), which the next one is matched against.
    natural: Option<usize>,
    /// The last window's second half, for the next to overlap.
    tail: Vec<Frame>,
}

impl Stretch {
    fn reset(&mut self, rate: u32) {
        let n = (rate as usize * 32 / 1000) & !1;
        *self = Self {
            rate,
            n,
            reach: n / 4,
            // Periodic Hann: two half a window apart sum to one.
            window: (0..n).map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / n as f32).cos()).collect(),
            tail: vec![[0.0; 2]; n / 2],
            ..Self::default()
        };
    }

    /// Takes interleaved stereo `samples` at `rate`, played `speed` times
    /// as fast as real time, and appends what lasts real time to `out`.
    pub fn process(&mut self, samples: &[i16], rate: u32, speed: f64, out: &mut Vec<i16>) {
        if rate != self.rate || self.n == 0 {
            self.reset(rate);
        }
        self.input.extend(samples.chunks_exact(2).map(|f| [f32::from(f[0]), f32::from(f[1])]));
        let (n, half, reach) = (self.n, self.n / 2, self.reach);
        let speed = speed.clamp(0.25, 4.0);
        loop {
            let ideal = self.pos.round() as usize;
            // Room to search around it and take a whole window.
            if ideal + reach + n > self.input.len() {
                break;
            }
            let start = match self.natural {
                Some(natural) => self.best(ideal, natural),
                None => ideal,
            };
            for i in 0..half {
                let (a, w) = (self.input[start + i], self.window[i]);
                let t = self.tail[i];
                out.extend([clip(t[0] + a[0] * w), clip(t[1] + a[1] * w)]);
            }
            for i in 0..half {
                let (a, w) = (self.input[start + half + i], self.window[half + i]);
                self.tail[i] = [a[0] * w, a[1] * w];
            }
            self.natural = Some(start + half);
            self.pos += half as f64 * speed;
        }
        // Drop what neither the next search nor the next continuation needs.
        let keep = self.natural.map_or(0, |n| n).min((self.pos as usize).saturating_sub(reach));
        if keep > 0 {
            self.input.drain(..keep);
            self.pos -= keep as f64;
            self.natural = self.natural.map(|n| n - keep);
        }
    }

    /// The window start within `reach` of `ideal` (and at least `reach`
    /// from the input's start) that best continues the natural follower of
    /// the last window, at `natural`: the most similar over half a window,
    /// compared on every 4th frame of the mono sum.
    fn best(&self, ideal: usize, natural: usize) -> usize {
        let half = self.n / 2;
        let mono = |i: usize| self.input[i][0] + self.input[i][1];
        let lo = ideal.saturating_sub(self.reach);
        let hi = ideal + self.reach;
        let (mut best, mut score) = (ideal, f32::MIN);
        for start in lo..=hi {
            let mut s = 0.0;
            for k in (0..half).step_by(4) {
                s += mono(start + k) * mono(natural + k);
            }
            if s > score {
                (best, score) = (start, s);
            }
        }
        best
    }
}

/// Fades the first `frames` frames of interleaved stereo `samples` in from
/// silence (sound coming back after Unlimited speed's silence).
pub fn fade_in(samples: &mut [i16], frames: usize) {
    let n = frames.min(samples.len() / 2).max(1);
    for (i, f) in samples.chunks_exact_mut(2).take(n).enumerate() {
        let g = i as f32 / n as f32;
        f[0] = (f32::from(f[0]) * g) as i16;
        f[1] = (f32::from(f[1]) * g) as i16;
    }
}

fn clip(v: f32) -> i16 {
    v.round().clamp(-32768.0, 32767.0) as i16
}

#[cfg(test)]
mod tests {
    use super::Stretch;

    fn tone(frames: usize, rate: f32) -> Vec<i16> {
        (0..frames).flat_map(|i| {
            let v = ((i as f32 * 440.0 / rate) * std::f32::consts::TAU).sin() * 8000.0;
            [v as i16, v as i16]
        }).collect()
    }

    #[test]
    fn lasts_real_time_at_any_speed() {
        for speed in [0.5, 1.5, 2.0, 3.0] {
            let mut s = Stretch::default();
            let mut out = Vec::new();
            // A second at 32 kHz, in fields of 534 frames.
            let input = tone(32_000, 32_000.0);
            for field in input.chunks(534 * 2) {
                s.process(field, 32_000, speed, &mut out);
            }
            // Within what is still held back (a window and the search's
            // reach of input, up to a window of output) or a half window
            // ahead (the first window counts from the input's start).
            let want = 32_000.0 / speed;
            let got = (out.len() / 2) as f64;
            let held = (1024.0 + 256.0) / speed + 1024.0;
            assert!(got <= want + 512.0 && got > want - held, "{speed}: {got} frames for {want}");
            // No clicks: the tone's steps stay small.
            let worst = out.chunks_exact(2).collect::<Vec<_>>().windows(2).map(|w| (i32::from(w[1][0]) - i32::from(w[0][0])).abs()).max().unwrap();
            assert!(worst < 1200, "{speed}: a step of {worst}");
        }
    }

    #[test]
    fn gives_its_input_back_at_one() {
        let mut s = Stretch::default();
        let input = tone(8_000, 32_000.0);
        let mut out = Vec::new();
        s.process(&input, 32_000, 1.0, &mut out);
        let half = 512;
        assert!(out.len() / 2 > 6_000);
        for i in half..out.len() / 2 {
            assert!((i32::from(out[2 * i]) - i32::from(input[2 * i])).abs() <= 1, "frame {i}");
        }
    }
}
