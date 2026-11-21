//! `--replay <log.dff>`: plays a Dolphin FIFO log through `gx` and a
//! headless renderer, the way Dolphin's FIFO player does (start state, EFB
//! clear, TMEM, then each frame's FIFO bytes with its RAM updates at their
//! FIFO positions), saves the chosen frame's XFB, and compares it with a
//! reference image (P3's check: PSNR against Dolphin's own frame dump).

use std::path::PathBuf;

use guest_mem::{Mem, NullMmio};
use gx::Gpu;
use vk_render::Renderer;

pub struct Options {
    pub log: PathBuf,
    /// 0-based frame to capture; default the last.
    pub frame: Option<usize>,
    pub scale: f32,
    pub screenshot: Option<PathBuf>,
    /// Reference image (same size as the capture).
    pub compare: Option<PathBuf>,
    pub min_psnr: f64,
}

/// PSNR in dB over RGB (infinite when identical).
pub fn psnr(a: &[u8], b: &[u8]) -> f64 {
    let (mut sum, mut n) = (0f64, 0f64);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        for c in 0..3 {
            let d = f64::from(p[c]) - f64::from(q[c]);
            sum += d * d;
            n += 1.0;
        }
    }
    if sum == 0.0 {
        return f64::INFINITY;
    }
    10.0 * (255.0f64 * 255.0 / (sum / n.max(1.0))).log10()
}

fn feed(gpu: &mut Gpu, mem: &mut Mem, frame: &gx::dff::Frame) {
    let mut events = Vec::new();
    let mut at = 0usize;
    for u in &frame.updates {
        let pos = (u.fifo_position as usize).min(frame.fifo.len());
        if pos > at {
            gpu.feed(&frame.fifo[at..pos], mem, &mut events);
            at = pos;
        }
        match mem.ram_mut(u.address, u.data.len()) {
            Some(dst) => dst.copy_from_slice(&u.data),
            None => eprintln!("memory update outside RAM: {:#010x} (+{})", u.address, u.data.len()),
        }
    }
    gpu.feed(&frame.fifo[at..], mem, &mut events);
}

/// Replays the log; returns whether the comparison (if any) passed.
pub fn run(o: &Options) -> Result<bool, String> {
    let bytes = std::fs::read(&o.log).map_err(|e| format!("{}: {e}", o.log.display()))?;
    let log = gx::dff::parse(&bytes).map_err(|e| format!("{}: {e}", o.log.display()))?;
    if log.frames.is_empty() {
        return Err("the log has no frames".into());
    }
    let target = o.frame.unwrap_or(log.frames.len() - 1);
    if target >= log.frames.len() {
        return Err(format!("frame {target}: the log has {} frames", log.frames.len()));
    }
    println!("{}: version {}, {}, game {:?}, {} frames; capturing frame {target}", o.log.display(), log.version, if log.is_wii { "Wii" } else { "GameCube" }, log.game_id, log.frames.len());
    let mut mem = Mem::new(Box::new(NullMmio));
    let mut gpu = Gpu::default();
    gpu.load_tmem(&log.tmem);
    let mut r = Renderer::headless(vk_render::Options { scale: o.scale, vsync: false, pipeline_cache: None, efb_copies_to_ram: true }).map_err(|e| e.to_string())?;
    let shot = o.screenshot.clone().unwrap_or_else(|| std::env::temp_dir().join("nsmbw-replay.png"));
    let _ = std::fs::remove_file(&shot);
    gpu.feed(&log.setup_stream(), &mem, &mut Vec::new());
    for (i, frame) in log.frames.iter().enumerate().take(target + 1) {
        feed(&mut gpu, &mut mem, frame);
        let out = gpu.take_output();
        if i == target {
            r.screenshot = Some(shot.clone());
        }
        r.render(&out, &out.vertices, true, 0, 0, 0).map_err(|e| e.to_string())?;
        r.evict(&out.evicted);
        r.finish().map_err(|e| e.to_string())?;
        for wb in r.take_writebacks() {
            let data = gx::texture::encode(wb.tex_format, wb.width as usize, wb.height as usize, &wb.rgba);
            let mut from = 0;
            for (at, len) in gx::texture::copy_rows(wb.tex_format, wb.width as usize, wb.height as usize, wb.stride as usize) {
                if let Some(dst) = mem.ram_mut(wb.addr + at as u32, len) {
                    dst.copy_from_slice(&data[from..from + len]);
                }
                from += len;
            }
            gpu.note_writeback(wb.addr, &data);
        }
    }
    let s = &gpu.stats;
    println!("GX: {} draws, {} vertices, {} XFB copies, {} texture copies, bad opcodes {}", s.draws, s.vertices, s.xfb_copies, s.tex_copies, s.bad_opcodes);
    if !shot.exists() {
        return Err(format!("frame {target} has no XFB copy to capture"));
    }
    println!("captured {}", shot.display());
    let Some(reference) = &o.compare else { return Ok(true) };
    let (w, h, ours) = vk_render::load_png(&shot).map_err(|e| e.to_string())?;
    let (rw, rh, theirs) = vk_render::load_png(reference).map_err(|e| format!("{}: {e}", reference.display()))?;
    if (w, h) != (rw, rh) {
        return Err(format!("size differs: capture {w}x{h}, reference {rw}x{rh} (try --scale)"));
    }
    let db = psnr(&ours, &theirs);
    let ok = db >= o.min_psnr;
    println!("PSNR {db:.2} dB against {} (threshold {} dB): {}", reference.display(), o.min_psnr, if ok { "PASS" } else { "FAIL" });
    Ok(ok)
}

/// Writes `log` to `path` (tests).
#[cfg(test)]
fn save(path: &std::path::Path, log: &gx::dff::FifoLog) {
    std::fs::write(path, gx::dff::write(log)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-frame log: pass-through TEV, an orthographic 1:1 projection, a
    /// green quad, and a display copy of the whole EFB.
    fn log() -> gx::dff::FifoLog {
        let f = |v: f32| v.to_bits();
        let mut bp = vec![0u32; 256];
        bp[0x00] = 1 << 4;
        bp[0x20] = 342 | (342 << 12);
        bp[0x21] = (527 + 342) | ((639 + 342) << 12);
        bp[0x59] = 171 | (171 << 10);
        bp[0xC0] = (15 << 12) | (15 << 8) | (15 << 4) | 10 | (1 << 19);
        bp[0xC1] = (7 << 13) | (7 << 10) | (7 << 7) | (5 << 4) | (1 << 19);
        bp[0xF6] = 4;
        bp[0xF7] = 14;
        bp[0xF3] = (7 << 16) | (7 << 19);
        bp[0x41] = 0x18;
        bp[0x43] = 1;
        bp[0x49] = 0;
        bp[0x4A] = 639 | (527 << 10);
        let mut cp = vec![0u32; 256];
        cp[0x50] = (1 << 9) | (1 << 13);
        cp[0x70] = 1 | (4 << 1) | (5 << 14);
        let mut xf_mem = vec![0u32; 4096];
        for (i, v) in [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0].iter().enumerate() {
            xf_mem[i] = v.to_bits();
        }
        let mut xf_regs = vec![0u32; 88];
        xf_regs[0x09] = 1;
        xf_regs[0x0E] = 1;
        xf_regs[0x10] = 1;
        xf_regs[0x1A..0x20].copy_from_slice(&[f(320.0), f(264.0), f(16_777_216.0), f(662.0), f(606.0), 0]);
        xf_regs[0x20..0x27].copy_from_slice(&[f(1.0 / 320.0), f(-1.0), f(1.0 / 264.0), f(-1.0), f(1.0), 0, 1]);
        let mut fifo = vec![0x80, 0x00, 0x04];
        for (x, y) in [(0.0f32, 0.0f32), (640.0, 0.0), (640.0, 528.0), (0.0, 528.0)] {
            for c in [x, y, 0.5] {
                fifo.extend(c.to_be_bytes());
            }
            fifo.extend([0, 255, 0, 255]);
        }
        // Display copy (bit 14) of the whole EFB.
        fifo.extend([0x61, 0x52, 0x00, 0x40, 0x00]);
        gx::dff::FifoLog {
            version: 6,
            is_wii: true,
            game_id: "TEST01".into(),
            bp,
            cp,
            xf_mem,
            xf_regs,
            tmem: vec![0; gx::dff::TMEM_SIZE],
            frames: vec![gx::dff::Frame { fifo, updates: Vec::new() }],
        }
    }

    #[test]
    fn replays_a_synthetic_log() {
        if Renderer::headless(vk_render::Options::default()).is_err() {
            eprintln!("skipped: no Vulkan device");
            return;
        }
        let dir = std::env::temp_dir().join(format!("nsmbw-replay-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (path, shot) = (dir.join("quad.dff"), dir.join("quad.png"));
        save(&path, &log());
        let opts = Options { log: path.clone(), frame: None, scale: 1.0, screenshot: Some(shot.clone()), compare: None, min_psnr: 30.0 };
        assert!(run(&opts).unwrap());
        let (w, h, px) = vk_render::load_png(&shot).unwrap();
        assert_eq!((w, h), (640, 528));
        assert_eq!(&px[..4], &[0, 255, 0, 255]);
        // Against itself: identical.
        let again = Options { compare: Some(shot.clone()), screenshot: Some(dir.join("again.png")), ..opts };
        assert!(run(&again).unwrap());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
