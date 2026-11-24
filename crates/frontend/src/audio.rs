//! Sound out: the samples the emulated AI DMA plays (16-bit stereo at
//! 32 kHz) go to the default output device through `cpal`, and optionally to
//! a WAV file.
//!
//! The game thread pushes each field's samples into a queue; the device's
//! callback drains it, resampling linearly to the device's rate. Playback
//! starts once [`TARGET_MS`] is queued, so field-to-field jitter never
//! drains it; past [`MAX_MS`] the oldest samples are dropped (the device fell
//! behind). If it runs dry (the game fell behind), the device plays silence
//! until the queue is full again, and that counts as one dropout.

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub const TARGET_MS: usize = 100;
const MAX_MS: usize = 250;

struct Queue {
    frames: VecDeque<[f32; 2]>,
    rate: u32,
    /// Fractional read position into `frames`.
    pos: f64,
    /// Playing (else waiting for [`TARGET_MS`] to be queued).
    playing: bool,
    dropouts: u64,
}

/// Output to the default device. Keep it on the thread that made it (the
/// stream is not `Send` on every platform).
/// The master volume in percent (the Settings window's Audio tab).
pub static VOLUME: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(100);

pub struct Audio {
    queue: Arc<Mutex<Queue>>,
    _stream: cpal::Stream,
    pub device_rate: u32,
}

impl Audio {
    /// Opens the default output device, or explains why not.
    pub fn open() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or("no audio output device")?;
        let config = device.default_output_config().map_err(|e| e.to_string())?;
        let device_rate = config.sample_rate().0;
        let channels = usize::from(config.channels());
        let queue = Arc::new(Mutex::new(Queue { frames: VecDeque::new(), rate: 32_000, pos: 0.0, playing: false, dropouts: 0 }));
        let q = queue.clone();
        let fill = move |out: &mut [f32]| {
            let mut q = q.lock().unwrap();
            let step = f64::from(q.rate) / f64::from(device_rate);
            if !q.playing && q.frames.len() >= q.rate as usize * TARGET_MS / 1000 {
                q.playing = true;
            }
            for frame in out.chunks_mut(channels) {
                let i = q.pos as usize;
                let (l, r) = if q.playing && i + 1 < q.frames.len() {
                    let t = (q.pos - i as f64) as f32;
                    let (a, b) = (q.frames[i], q.frames[i + 1]);
                    q.pos += step;
                    (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)
                } else {
                    if q.playing {
                        q.playing = false;
                        q.dropouts += 1;
                    }
                    (0.0, 0.0)
                };
                for (c, s) in frame.iter_mut().enumerate() {
                    *s = if c % 2 == 0 { l } else { r };
                }
            }
            let used = (q.pos as usize).min(q.frames.len());
            q.frames.drain(..used);
            q.pos -= used as f64;
        };
        let err = |e| eprintln!("audio: {e}");
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(&config.into(), move |out: &mut [f32], _| fill(out), err, None),
            cpal::SampleFormat::I16 => {
                let mut buf = Vec::new();
                device.build_output_stream(
                    &config.into(),
                    move |out: &mut [i16], _| {
                        buf.resize(out.len(), 0.0);
                        fill(&mut buf);
                        for (o, s) in out.iter_mut().zip(&buf) {
                            *o = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
                        }
                    },
                    err,
                    None,
                )
            }
            f => return Err(format!("unsupported device sample format {f:?}")),
        }
        .map_err(|e| e.to_string())?;
        stream.play().map_err(|e| e.to_string())?;
        Ok(Self { queue, _stream: stream, device_rate })
    }

    /// Queues interleaved left/right samples at `rate`.
    pub fn push(&self, samples: &[i16], rate: u32) {
        let mut q = self.queue.lock().unwrap();
        if q.rate != rate {
            q.frames.clear();
            q.pos = 0.0;
            q.rate = rate;
        }
        let gain = f32::from(VOLUME.load(std::sync::atomic::Ordering::Relaxed)) / 100.0 / 32768.0;
        q.frames.extend(samples.chunks_exact(2).map(|f| [f32::from(f[0]) * gain, f32::from(f[1]) * gain]));
        let max = rate as usize * MAX_MS / 1000;
        if q.frames.len() > max {
            let drop = q.frames.len() - rate as usize * TARGET_MS / 1000;
            q.frames.drain(..drop);
            q.pos = 0.0;
        }
    }

    /// Sound queued and not yet played, in milliseconds.
    pub fn queued_ms(&self) -> f32 {
        let q = self.queue.lock().unwrap();
        (q.frames.len() as f64 - q.pos).max(0.0) as f32 * 1000.0 / q.rate as f32
    }

    /// Fades the last `ms` of what is queued out to silence (before
    /// [`Audio::fill_silence`], so the sound stops without a click).
    pub fn fade_tail(&self, ms: u32) {
        let mut q = self.queue.lock().unwrap();
        let n = (q.rate as usize * ms as usize / 1000).min(q.frames.len()).max(1);
        let len = q.frames.len();
        for (i, f) in q.frames.range_mut(len.saturating_sub(n)..).enumerate() {
            let g = 1.0 - (i + 1) as f32 / n as f32;
            *f = [f[0] * g, f[1] * g];
        }
    }

    /// Silence queued up to the playing level at `rate` (Unlimited speed,
    /// whose sound is not played): the output keeps going, with no dropout.
    pub fn fill_silence(&self, rate: u32) {
        let mut q = self.queue.lock().unwrap();
        if q.rate != rate {
            q.frames.clear();
            q.pos = 0.0;
            q.rate = rate;
        }
        let want = rate as usize * TARGET_MS / 1000 + q.pos as usize;
        let have = q.frames.len();
        q.frames.extend(std::iter::repeat_n([0.0; 2], want.saturating_sub(have)));
    }

    /// Times playback ran dry, so far.
    pub fn dropouts(&self) -> u64 {
        self.queue.lock().unwrap().dropouts
    }
}

/// A 16-bit stereo WAV file, written as samples arrive.
pub struct Wav {
    file: BufWriter<File>,
    rate: u32,
    bytes: u32,
}

impl Wav {
    pub fn create(path: &Path, rate: u32) -> std::io::Result<Self> {
        let mut w = Self { file: BufWriter::new(File::create(path)?), rate, bytes: 0 };
        w.header()?;
        Ok(w)
    }

    fn header(&mut self) -> std::io::Result<()> {
        let f = &mut self.file;
        f.write_all(b"RIFF")?;
        f.write_all(&(36 + self.bytes).to_le_bytes())?;
        f.write_all(b"WAVEfmt ")?;
        f.write_all(&16u32.to_le_bytes())?;
        f.write_all(&1u16.to_le_bytes())?; // PCM
        f.write_all(&2u16.to_le_bytes())?;
        f.write_all(&self.rate.to_le_bytes())?;
        f.write_all(&(self.rate * 4).to_le_bytes())?;
        f.write_all(&4u16.to_le_bytes())?;
        f.write_all(&16u16.to_le_bytes())?;
        f.write_all(b"data")?;
        f.write_all(&self.bytes.to_le_bytes())
    }

    pub fn write(&mut self, samples: &[i16]) -> std::io::Result<()> {
        for s in samples {
            self.file.write_all(&s.to_le_bytes())?;
        }
        self.bytes += 2 * samples.len() as u32;
        Ok(())
    }

    /// Writes the final sizes into the header.
    pub fn finish(mut self) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.header()?;
        self.file.flush()
    }
}
