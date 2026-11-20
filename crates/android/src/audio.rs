//! Sound out on Android: the frontend's `audio` module (same API, used by
//! `emu.rs`) on AAudio instead of `cpal`.
//!
//! As on Windows, the game thread pushes each field's 32 kHz stereo samples
//! into a queue; AAudio's callback drains it, resampling linearly to the
//! stream's rate. Playback starts once [`TARGET_MS`] is queued; past
//! [`MAX_MS`] the oldest samples are dropped; running dry counts a dropout.
//! While the app is paused ([`set_paused`]) the callback plays silence and
//! leaves the queue alone.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use ndk::audio::{AudioCallbackResult, AudioDirection, AudioFormat, AudioPerformanceMode, AudioSharingMode, AudioStream, AudioStreamBuilder, AudioUsage};

const TARGET_MS: usize = 100;
const MAX_MS: usize = 250;

static PAUSED: AtomicBool = AtomicBool::new(false);

/// The app left the screen (or came back): silence, and the queue kept.
pub fn set_paused(on: bool) {
    PAUSED.store(on, Ordering::Relaxed);
}

struct Queue {
    frames: VecDeque<[f32; 2]>,
    rate: u32,
    /// Fractional read position into `frames`.
    pos: f64,
    /// Playing (else waiting for [`TARGET_MS`] to be queued).
    playing: bool,
    dropouts: u64,
}

pub struct Audio {
    queue: Arc<Mutex<Queue>>,
    _stream: AudioStream,
    pub device_rate: u32,
}

impl Audio {
    /// Opens a low-latency game stream on the default output.
    pub fn open() -> Result<Self, String> {
        let queue = Arc::new(Mutex::new(Queue { frames: VecDeque::new(), rate: 32_000, pos: 0.0, playing: false, dropouts: 0 }));
        let q = queue.clone();
        // The stream's rate, known once it is open; the callback reads it here.
        let rate = Arc::new(std::sync::atomic::AtomicU32::new(48_000));
        let r = rate.clone();
        let callback = move |_: &AudioStream, data: *mut std::ffi::c_void, frames: i32| {
            // SAFETY: AAudio hands the callback `frames` frames of 2 interleaved floats.
            let out = unsafe { std::slice::from_raw_parts_mut(data.cast::<f32>(), frames.max(0) as usize * 2) };
            let mut q = q.lock().unwrap();
            if PAUSED.load(Ordering::Relaxed) {
                out.fill(0.0);
                return AudioCallbackResult::Continue;
            }
            let device_rate = r.load(Ordering::Relaxed).max(1);
            let step = f64::from(q.rate) / f64::from(device_rate);
            if !q.playing && q.frames.len() >= q.rate as usize * TARGET_MS / 1000 {
                q.playing = true;
            }
            for frame in out.chunks_exact_mut(2) {
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
                frame[0] = l;
                frame[1] = r;
            }
            let used = (q.pos as usize).min(q.frames.len());
            q.frames.drain(..used);
            q.pos -= used as f64;
            AudioCallbackResult::Continue
        };
        let stream = AudioStreamBuilder::new()
            .map_err(|e| e.to_string())?
            .direction(AudioDirection::Output)
            .channel_count(2)
            .format(AudioFormat::PCM_Float)
            .performance_mode(AudioPerformanceMode::LowLatency)
            .sharing_mode(AudioSharingMode::Shared)
            .usage(AudioUsage::Game)
            .data_callback(Box::new(callback))
            .error_callback(Box::new(|_, e| eprintln!("audio: {e:?}")))
            .open_stream()
            .map_err(|e| e.to_string())?;
        let device_rate = stream.sample_rate().max(1) as u32;
        rate.store(device_rate, Ordering::Relaxed);
        stream.request_start().map_err(|e| e.to_string())?;
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
        q.frames.extend(samples.chunks_exact(2).map(|f| [f32::from(f[0]) / 32768.0, f32::from(f[1]) / 32768.0]));
        let max = rate as usize * MAX_MS / 1000;
        if q.frames.len() > max {
            let drop = q.frames.len() - rate as usize * TARGET_MS / 1000;
            q.frames.drain(..drop);
            q.pos = 0.0;
        }
    }

    /// Times playback ran dry, so far.
    pub fn dropouts(&self) -> u64 {
        self.queue.lock().unwrap().dropouts
    }
}

/// `--wav` has no Android equivalent; `emu.rs` only makes one when asked.
pub struct Wav;

impl Wav {
    pub fn create(_: &Path, _: u32) -> std::io::Result<Self> {
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "WAV output is not available on Android"))
    }

    pub fn write(&mut self, _: &[i16]) -> std::io::Result<()> {
        Ok(())
    }

    pub fn finish(self) -> std::io::Result<()> {
        Ok(())
    }
}
