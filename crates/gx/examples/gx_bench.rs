//! Times GX decoding and transform on a capture ([`gx::capture`]), away
//! from the game: on the PC, or on a phone from `adb shell`.
//!
//!   gx_bench <capture> [--rounds <n>] [--records] [--inline] [--digest] [--heat] [--gpu-vertices]
//!   gx_bench <capture> --check-gpu
//!
//! Each round starts from the capture's registers and RAM with a fresh
//! decoder (so textures are decoded again in its first field) and runs every
//! field. Decoding runs on the calling thread and the transform on its own,
//! as in the game; per field it prints the medians over the rounds:
//! decoding (without its waits for the transform thread), the transform
//! thread's busy time, those waits, and the wall time until the field's
//! output was ready. `--inline` runs both halves on one thread and
//! reports only the wall time. `--records` keeps frame-interpolation records
//! (the desktop's default; the Android app runs without). `--gpu-vertices`
//! leaves triangles' vertices for the renderer to decode and transform
//! (`gx::pull`), as for the Vulkan renderer. `--check-gpu` runs the capture
//! both ways and checks every triangle of every field: pulled and
//! transformed as the renderer will (`pull::transform_pulled`), it must be
//! the same bits as transformed on the CPU. `--digest` prints
//! a digest of every field's vertices, indices, shading blocks and commands
//! (first round only), to check that a change leaves the output alone.

use std::time::{Duration, Instant};

use gx::capture::{Capture, Rec};
use gx::Gpu;
use guest_mem::{Mem, NullMmio};

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = args.first() else {
        eprintln!("usage: gx_bench <capture> [--rounds <n>] [--records] [--inline]");
        std::process::exit(2);
    };
    let rounds: usize = args.iter().position(|a| a == "--rounds").and_then(|i| args.get(i + 1)).and_then(|n| n.parse().ok()).unwrap_or(5);
    let records = args.iter().any(|a| a == "--records");
    let gpu_vertices = args.iter().any(|a| a == "--gpu-vertices");
    let inline = args.iter().any(|a| a == "--inline");
    let digest = args.iter().any(|a| a == "--digest");
    // A phone's governor clocks a cluster by how busy it is: with the two
    // GX threads waiting on each other part of the time it lowers the
    // clock (2.46 to 1.6 GHz on a Snapdragon 835), so a faster thread can
    // measure slower. `--heat` keeps one more thread busy, as the game
    // thread is in the game, so builds compare at the same clock.
    let heat = args.iter().any(|a| a == "--heat");
    let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    if heat {
        let done = done.clone();
        std::thread::spawn(move || {
            let mut x = 1u64;
            while !done.load(std::sync::atomic::Ordering::Relaxed) {
                for _ in 0..1000 {
                    x = std::hint::black_box(x.wrapping_mul(6364136223846793005).wrapping_add(1));
                }
            }
        });
    }
    let mut sum = 0xCBF2_9CE4_8422_2325u64;
    let cap = Capture::parse(&std::fs::read(path).expect("reading the capture")).expect("parsing the capture");
    if args.iter().any(|a| a == "--check-gpu") {
        return check_gpu(&cap);
    }
    let mut mem = Mem::new(Box::new(NullMmio));
    if let Some(m) = mem.ram_mut(0x8000_0000, cap.mem1.len()) {
        m.copy_from_slice(&cap.mem1);
    }
    if let Some(m) = mem.ram_mut(0x9000_0000, cap.mem2.len()) {
        m.copy_from_slice(&cap.mem2);
    }
    let ram = mem.view();
    let n = cap.fields.len();
    // Per field: decode, transform, decode's waits for the transform, wall
    // (ms), one per round.
    let mut times = vec![[Vec::new(), Vec::new(), Vec::new(), Vec::new()]; n];
    let (mut draws, mut gx_draws, mut vertices) = (0usize, 0u64, 0usize);
    for round in 0..rounds {
        let mut gpu = Gpu::default();
        gpu.records_next = records;
        gpu.gpu_vertices = gpu_vertices;
        gpu.snap_regs(&mut guest_mem::snap::Snap::loader(cap.regs.clone()));
        if !inline {
            gpu.start_transform_thread(|| {});
        }
        let mut events = Vec::new();
        let ms = |d: Duration| d.as_secs_f64() * 1000.0;
        for (i, field) in cap.fields.iter().enumerate() {
            let (t, w) = (Instant::now(), gpu.waited_for_transform());
            for r in field {
                match r {
                    Rec::Data(d) => gpu.feed(d, &ram, &mut events),
                    Rec::World(c) => gpu.set_world(*c),
                }
                events.clear();
            }
            gpu.flush_back();
            let waited = gpu.waited_for_transform() - w;
            let decode = t.elapsed().saturating_sub(waited);
            let out = gpu.take_output();
            let wall = t.elapsed();
            times[i][0].push(ms(decode));
            times[i][1].push(ms(out.busy[1]));
            times[i][2].push(ms(waited));
            times[i][3].push(ms(wall));
            if round == 0 && digest {
                let mut eat = |b: &[u8]| {
                    for c in b.chunks(8) {
                        let mut w = [0u8; 8];
                        w[..c.len()].copy_from_slice(c);
                        sum = (sum.rotate_left(5) ^ u64::from_le_bytes(w)).wrapping_mul(0x517C_C1B7_2722_0A95);
                    }
                };
                for v in &out.vertices {
                    for f in v.pos.iter().chain(v.color.iter().flatten()).chain(v.tex.iter().flatten()) {
                        eat(&f.to_bits().to_le_bytes());
                    }
                }
                for i in &out.indices {
                    eat(&i.to_le_bytes());
                }
                for s in &out.shadings {
                    for w in s.0 {
                        eat(&w.to_le_bytes());
                    }
                }
                for c in &out.cmds {
                    eat(format!("{c:?}").as_bytes());
                }
            }
            if round == 0 {
                draws += out.cmds.iter().filter(|c| matches!(c, gx::Cmd::Draw(_))).count();
                gx_draws += u64::from(out.draw_ops);
                vertices += out.vertices.len();
            }
            gpu.recycle(out);
        }
    }
    done.store(true, std::sync::atomic::Ordering::Relaxed);
    println!("field,decode_ms,transform_ms,decode_waits_ms,wall_ms");
    let mut tot = [0.0f64; 4];
    for (i, t) in times.into_iter().enumerate() {
        let m = t.map(median);
        for k in 0..4 {
            tot[k] += m[k];
        }
        println!("{i},{:.3},{:.3},{:.3},{:.3}", m[0], m[1], m[2], m[3]);
    }
    let nf = n.max(1) as f64;
    if digest {
        println!("digest {sum:016x}");
    }
    println!(
        "{n} fields, {rounds} rounds{}{}: a field (mean of medians) decode {:.2} ms, transform {:.2} ms, decode waiting for the transform {:.2} ms, wall {:.2} ms; {} draws ({} before merging), {} vertices",
        if inline { ", inline" } else { "" },
        if records { ", records" } else { "" },
        tot[0] / nf,
        tot[1] / nf,
        tot[2] / nf,
        tot[3] / nf,
        draws / n.max(1),
        gx_draws / n.max(1) as u64,
        vertices / n.max(1)
    );
}

/// Every field's triangles, in draw order: per triangle its shading block,
/// raster state, scissor and three vertices (as bits).
fn triangles(out: &gx::Output) -> Vec<(gx::Shading, gx::Raster, gx::Rect, [[u32; 4 + 8 + 24]; 3])> {
    let bits = |v: &gx::Vertex| -> [u32; 36] {
        let mut b = [0u32; 36];
        for (o, f) in b.iter_mut().zip(v.pos.iter().chain(v.color.iter().flatten()).chain(v.tex.iter().flatten())) {
            *o = f.to_bits();
        }
        b
    };
    let mut tris = Vec::new();
    for c in &out.cmds {
        let gx::Cmd::Draw(d) = c else { continue };
        let idx = &out.indices[d.first as usize..(d.first + d.count) as usize];
        for t in idx.chunks_exact(3) {
            let v: [[u32; 36]; 3] = std::array::from_fn(|k| { let i = t[k]; if !d.pulled { bits(&out.vertices[i as usize]) } else { bits(&gx::pull::transform_pulled(out, i >> 16, i & 0xFFFF)) } });
            tris.push((out.shadings[d.shading as usize], d.raster, d.scissor, v));
        }
    }
    tris
}

/// `--check-gpu`.
fn check_gpu(cap: &Capture) {
    let mut mem = Mem::new(Box::new(NullMmio));
    mem.ram_mut(0x8000_0000, cap.mem1.len()).unwrap().copy_from_slice(&cap.mem1);
    mem.ram_mut(0x9000_0000, cap.mem2.len()).unwrap().copy_from_slice(&cap.mem2);
    let ram = mem.view();
    let mut gpus = [false, true].map(|on| {
        let mut gpu = Gpu::default();
        gpu.records_next = false;
        gpu.gpu_vertices = on;
        gpu.snap_regs(&mut guest_mem::snap::Snap::loader(cap.regs.clone()));
        gpu
    });
    let (mut tris, mut packed, mut bad) = (0usize, 0usize, 0usize);
    for (i, field) in cap.fields.iter().enumerate() {
        let outs = gpus.each_mut().map(|gpu| {
            let mut events = Vec::new();
            for r in field {
                match r {
                    Rec::Data(d) => gpu.feed(d, &ram, &mut events),
                    Rec::World(c) => gpu.set_world(*c),
                }
                events.clear();
            }
            gpu.take_output()
        });
        packed += outs[1].cmds.iter().filter(|c| matches!(c, gx::Cmd::Draw(d) if d.pulled)).count();
        let (a, b) = (triangles(&outs[0]), triangles(&outs[1]));
        tris += a.len();
        if a.len() != b.len() {
            println!("field {i}: {} triangles on the CPU, {} pulled", a.len(), b.len());
            bad += 1;
            continue;
        }
        if let Some(k) = a.iter().zip(&b).position(|(x, y)| x.0 != y.0 || x.1 != y.1 || x.2 != y.2 || x.3 != y.3) {
            let (x, y) = (&a[k], &b[k]);
            println!("field {i}: triangle {k} differs (shading {}, raster {}, scissor {}, vertices {})", x.0 == y.0, x.1 == y.1, x.2 == y.2, x.3 == y.3);
            if x.3 != y.3 {
                println!("  cpu    {:?}
  pulled {:?}", x.3.map(|v| v[..4].iter().map(|b| f32::from_bits(*b)).collect::<Vec<_>>()), y.3.map(|v| v[..4].iter().map(|b| f32::from_bits(*b)).collect::<Vec<_>>()));
            }
            bad += 1;
        }
        for (gpu, out) in gpus.iter_mut().zip(outs) {
            gpu.recycle(out);
        }
    }
    println!("{} fields, {tris} triangles, {packed} pulled draw commands: {bad} fields differ", cap.fields.len());
    if bad > 0 {
        std::process::exit(1);
    }
}
