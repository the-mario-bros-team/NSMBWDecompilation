//! A debugging aid for the renderers: `NSMBW_PROBE_FIELD=<n>` (with
//! `NSMBW_PROBE_RENDERER=d3d12` or `d3d11`) renders field n headless on
//! Vulkan and on the other renderer, at native resolution (or
//! `NSMBW_PROBE_SCALE`), and looks for
//! the first command after which their EFBs or EFB copies differ (a binary
//! search over the field's command prefixes, each on fresh renderers).
//! Pixel numbers past 640 x 528 are in the copies.

use std::collections::HashMap;

use crate::settings::RendererChoice;

/// The EFB after the first `n` commands, and every EFB copy's texels.
fn snapshot(choice: RendererChoice, textures: &HashMap<u32, gx::texture::NewTexture>, out: &gx::Output, n: usize) -> Result<render::EfbSnapshot, String> {
    let scale = std::env::var("NSMBW_PROBE_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let opts = render::Options { scale, vsync: false, pipeline_cache: None, efb_copies_to_ram: true };
    let mut r = crate::gfx::headless(choice, opts)?;
    let o = gx::Output { vertices: out.vertices.clone(), indices: out.indices.clone(), shadings: out.shadings.clone(), cmds: out.cmds[..n].to_vec(), new_textures: textures.values().cloned().collect(), ..Default::default() };
    r.render(&o, &o.vertices, true, 0, 0, 0).map_err(|e| e.to_string())?;
    r.finish().map_err(|e| e.to_string())?;
    let copies: Vec<u8> = r.take_writebacks().into_iter().flat_map(|w| w.rgba).collect();
    let mut snap = r.efb_snapshot().map_err(|e| e.to_string())?;
    // The copies after the EFB, as more "pixels".
    snap.color.extend(copies);
    Ok(snap)
}

/// (pixels differing, largest difference, first differing pixel).
fn differ(a: &render::EfbSnapshot, b: &render::EfbSnapshot) -> (usize, u8, Option<(usize, [u8; 4], [u8; 4])>) {
    let mut n = 0;
    let mut max = 0;
    let mut first = None;
    for (i, (p, q)) in a.color.chunks_exact(4).zip(b.color.chunks_exact(4)).enumerate() {
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        if d > 0 {
            n += 1;
            max = max.max(d);
            if first.is_none() {
                first = Some((i, [p[0], p[1], p[2], p[3]], [q[0], q[1], q[2], q[3]]));
            }
        }
    }
    (n, max, first)
}

pub fn run(textures: &HashMap<u32, gx::texture::NewTexture>, out: &gx::Output) {
    let other = std::env::var("NSMBW_PROBE_RENDERER").ok().and_then(|s| RendererChoice::parse(&s)).unwrap_or(RendererChoice::D3d12);
    let same = |n: usize| -> Result<bool, String> {
        let a = snapshot(RendererChoice::Vulkan, textures, out, n)?;
        let b = snapshot(other, textures, out, n)?;
        let (count, max, first) = differ(&a, &b);
        if count > 0 {
            println!("probe: after {n} commands, {count} pixels differ (largest {max}); first at {:?}", first.map(|(i, p, q)| ((i % 640, i / 640), p, q)));
        }
        Ok(count == 0)
    };
    let run = || -> Result<(), String> {
        // NSMBW_PROBE_AT=<n>,<x>,<y>: that pixel before and after command n, on both.
        if let Some(v) = std::env::var("NSMBW_PROBE_AT").ok() {
            let p: Vec<usize> = v.split(',').filter_map(|x| x.parse().ok()).collect();
            if let [n, x, y] = p[..] {
                let i = (y * 640 + x) * 4;
                for (name, choice) in [("Vulkan", RendererChoice::Vulkan), ("other", other)] {
                    let (a, b) = (snapshot(choice, textures, out, n)?, snapshot(choice, textures, out, n + 1)?);
                    println!("probe: {name} at ({x}, {y}): {:?} -> {:?}", &a.color[i..i + 4], &b.color[i..i + 4]);
                }
                return Ok(());
            }
        }
        let total = out.cmds.len();
        if same(total)? {
            println!("probe: the field's {total} commands give the same EFB");
            return Ok(());
        }
        // Same after lo commands, not after hi.
        let (mut lo, mut hi) = (0, total);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if same(mid)? {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let c = out.cmds[hi - 1];
        println!("probe: command {} of {total} makes them differ: {c:?}", hi - 1);
        // The first differing pixel, before and after it, on both.
        let (va, vb) = (snapshot(RendererChoice::Vulkan, textures, out, hi)?, snapshot(other, textures, out, hi)?);
        if let (_, _, Some((i, _, _))) = differ(&va, &vb) {
            let (pa, pb) = (snapshot(RendererChoice::Vulkan, textures, out, lo)?, snapshot(other, textures, out, lo)?);
            let at = |s: &render::EfbSnapshot| s.color[i * 4..i * 4 + 4].to_vec();
            println!("probe: pixel {i}: Vulkan {:?} -> {:?}, other {:?} -> {:?}", at(&pa), at(&va), at(&pb), at(&vb));
        }
        if let gx::Cmd::Draw(d) = c {
            if let Some(s) = out.shadings.get(d.shading as usize) {
                println!("probe: its draw state: {:08x?}", s.0);
                for m in 0..8 {
                    let id = s.0[gx::sh::TEX + m];
                    if let Some(t) = textures.get(&id) {
                        let l0 = t.levels.first().map(|l| l[..l.len().min(16)].to_vec());
                        println!("probe: map {m}: texture {id:#x} {}x{}, {} levels, first texels {l0:?}", t.width, t.height, t.levels.len());
                    } else if id != gx::NO_TEXTURE {
                        println!("probe: map {m}: texture {id:#x} not uploaded");
                    }
                }
            }
            let idx = &out.indices[d.first as usize..(d.first + d.count) as usize];
            for &i in idx.iter().take(3) {
                println!("probe: vertex {i}: {:?}", out.vertices[i as usize]);
            }
        }
        Ok(())
    };
    if let Err(e) = run() {
        println!("probe: {e}");
    }
}
