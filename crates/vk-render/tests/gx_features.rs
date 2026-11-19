//! GX features end to end: hand-built command streams go through `gx::Gpu`
//! and a headless `Renderer`, and the EFB is read back the way a CPU peek
//! sees it. Skipped (with a message) when no Vulkan device is available.

use guest_mem::{Mem, NullMmio};
use gx::Gpu;
use vk_render::{Options, Renderer};

const TEX0: u32 = 0x1000;
const TEX1: u32 = 0x2000;

struct Stream(Vec<u8>);

impl Stream {
    fn cp(&mut self, reg: u8, v: u32) -> &mut Self {
        self.0.extend([0x08, reg]);
        self.0.extend(v.to_be_bytes());
        self
    }
    fn xf(&mut self, addr: u16, vals: &[u32]) -> &mut Self {
        self.0.push(0x10);
        self.0.extend(((vals.len() as u32 - 1) << 16 | u32::from(addr)).to_be_bytes());
        for v in vals {
            self.0.extend(v.to_be_bytes());
        }
        self
    }
    fn bp(&mut self, reg: u8, v: u32) -> &mut Self {
        self.0.push(0x61);
        self.0.extend((u32::from(reg) << 24 | (v & 0xFF_FFFF)).to_be_bytes());
        self
    }
    /// A quad over EFB pixels (x0, y0)..(x1, y1) at depth `z` (0..1).
    fn quad(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, rgba: [u8; 4]) -> &mut Self {
        self.0.extend([0x80, 0x00, 0x04]);
        for (x, y) in [(x0, y0), (x1, y0), (x1, y1), (x0, y1)] {
            for c in [x, y, z] {
                self.0.extend(c.to_be_bytes());
            }
            self.0.extend(rgba);
        }
        self
    }
}

/// One colour channel from the vertex, one TEV stage passing it through,
/// an orthographic projection mapping EFB pixels 1:1, alpha test off,
/// colour and alpha writes on, an EFB with alpha.
fn setup() -> Stream {
    let mut s = Stream(Vec::new());
    // VCD: position and colour 0 direct; VAT 0: XYZ f32, RGBA8888.
    s.cp(0x50, (1 << 9) | (1 << 13)).cp(0x70, 1 | (4 << 1) | (5 << 14));
    let one = 1.0f32.to_bits();
    s.xf(0, &[one, 0, 0, 0, 0, one, 0, 0, 0, 0, one, 0]);
    // Viewport (wd, ht, zrange, xorig, yorig, farz) and projection.
    let f = |v: f32| v.to_bits();
    s.xf(0x101A, &[f(320.0), f(264.0), f(16_777_216.0), f(320.0 + 342.0), f(264.0 + 342.0), 0]);
    s.xf(0x1020, &[f(1.0 / 320.0), f(-1.0), f(1.0 / 264.0), f(-1.0), f(1.0), 0, 1]);
    s.xf(0x1009, &[1]).xf(0x100E, &[1]).xf(0x1010, &[1]);
    s.bp(0x59, 171 | (171 << 10));
    s.bp(0x20, 342 | (342 << 12)).bp(0x21, (527 + 342) | ((639 + 342) << 12));
    s.bp(0x00, 1 << 4); // one colour channel, one TEV stage
    s.bp(0x28, 0);
    s.bp(0xC0, (15 << 12) | (15 << 8) | (15 << 4) | 10 | (1 << 19)); // colour = rasterised colour
    s.bp(0xC1, (7 << 13) | (7 << 10) | (7 << 7) | (5 << 4) | (1 << 19)); // alpha = rasterised alpha
    s.bp(0xF6, 4).bp(0xF7, 14); // identity swap table
    s.bp(0xF3, (7 << 16) | (7 << 19)); // alpha test: always
    s.bp(0x41, 0x18).bp(0x40, 0).bp(0x43, 1);
    // Fog, destination alpha, Z texture, indirect: off (cases share the GPU state).
    s.bp(0xF1, 0).bp(0x42, 0).bp(0xF5, 0).bp(0x10, 0).bp(0xE8, 0);
    s
}

/// An 8x8 texture in map `map` (RAM `addr`, format `fmt`) addressed by
/// texcoord 0, generated from the position as (x / 64, y / 64).
fn texture(s: &mut Stream, map: u8, addr: u32, fmt: u32) {
    s.bp(0x80 + map, 0).bp(0x84 + map, 0);
    s.bp(0x88 + map, 7 | (7 << 10) | (fmt << 20)).bp(0x94 + map, addr >> 5);
    s.bp(0x30, 7).bp(0x31, 7);
}

fn texgen(s: &mut Stream) {
    let f = |v: f32| v.to_bits();
    // Texture matrix at row 30 (XF words 120..): s = x / 64, t = y / 64.
    s.xf(120, &[f(1.0 / 64.0), 0, 0, 0, 0, f(1.0 / 64.0), 0, 0]);
    s.xf(0x1018, &[30 << 6]).xf(0x103F, &[1]).xf(0x1040, &[0]);
}

struct Rig {
    mem: Mem,
    gpu: Gpu,
    r: Renderer,
}

fn rig() -> Option<Rig> {
    match Renderer::headless(Options { scale: 2, ..Default::default() }) {
        Ok(r) => Some(Rig { mem: Mem::new(Box::new(NullMmio)), gpu: Gpu::default(), r }),
        Err(e) => {
            eprintln!("skipped: no Vulkan device ({e})");
            None
        }
    }
}

impl Rig {
    fn run(&mut self, s: &Stream) -> vk_render::EfbSnapshot {
        let mut ev = Vec::new();
        self.gpu.feed(&s.0, &self.mem, &mut ev);
        let out = self.gpu.take_output();
        self.r.render(&out, &out.vertices, true, 0, 0, 0).unwrap();
        self.r.efb_snapshot().unwrap()
    }
}

fn px(s: &vk_render::EfbSnapshot, x: usize, y: usize) -> [u8; 4] {
    let i = (y * 640 + x) * 4;
    [s.color[i], s.color[i + 1], s.color[i + 2], s.color[i + 3]]
}

fn near(a: [u8; 4], b: [u8; 4]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 2)
}

#[test]
fn gx_features_on_the_gpu() {
    let Some(mut g) = rig() else { return };

    // Baseline: a red quad.
    let mut s = setup();
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
    let snap = g.run(&s);
    assert_eq!(px(&snap, 10, 10), [255, 0, 0, 255], "baseline");
    assert_eq!(px(&snap, 100, 100), [0, 0, 0, 255], "outside the quad");

    // Fog: linear, orthographic, A = 1, C = 0, so the fog factor is the
    // depth (0.5): red half way to green.
    let mut s = setup();
    s.bp(0xEE, 127 << 11).bp(0xF1, (1 << 20) | (2 << 21)).bp(0xF2, 0x00FF00);
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
    let snap = g.run(&s);
    assert!(near(px(&snap, 10, 10), [127, 128, 0, 255]), "fog: {:?}", px(&snap, 10, 10));

    // Logic op XOR: white, then red XORed over it.
    let mut s = setup();
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 255, 255, 255]);
    s.bp(0x41, 0x18 | 2 | (6 << 12));
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
    let snap = g.run(&s);
    assert_eq!(px(&snap, 10, 10), [0, 255, 255, 0], "logic op");

    // Destination alpha: the EFB gets 0x40 while blending (source alpha,
    // inverse source alpha) still uses the TEV's 255.
    let mut s = setup();
    s.bp(0x42, 0x100 | 0x40).bp(0x41, 0x18 | 1 | (4 << 8) | (5 << 5));
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
    let snap = g.run(&s);
    assert_eq!(px(&snap, 10, 10), [255, 0, 0, 0x40], "destination alpha");

    // Masked clear: white, then a copy's clear that updates alpha only.
    let mut s = setup();
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 255, 255, 255]);
    s.bp(0x49, 0).bp(0x4A, 63 | (63 << 10)).bp(0x4F, 0x0000).bp(0x50, 0x00FF);
    s.bp(0x41, 0x10);
    s.bp(0x52, (1 << 11) | (1 << 14));
    let snap = g.run(&s);
    assert_eq!(px(&snap, 10, 10), [255, 255, 255, 0], "alpha-only clear");

    // CPU poke of one pixel.
    g.gpu.poke(5, 6, 0xFF00_FF00, false);
    let snap = g.run(&Stream(Vec::new()));
    assert_eq!(px(&snap, 5, 6), [0, 255, 0, 255], "poke");

    // Z-freeze: the plane of a quad at z = 0.25, then a quad at z = 0.9
    // drawn frozen writes 0.25.
    let mut s = setup();
    s.bp(0x40, 1 | (7 << 1) | 0x10);
    s.quad(0.0, 0.0, 64.0, 64.0, 0.25, [255, 0, 0, 255]);
    s.bp(0x00, (1 << 4) | (1 << 19));
    s.quad(0.0, 0.0, 64.0, 64.0, 0.9, [0, 0, 255, 255]);
    let snap = g.run(&s);
    let z = snap.depth[10 * 640 + 10];
    assert!(z.abs_diff(1 << 22) < 64, "z-freeze depth {z:#x}");
    assert_eq!(px(&snap, 10, 10), [0, 0, 255, 255]);

    // Z texture, replace with u8: an I8 texture of 0x80 sets depth 0x80.
    for i in 0..64 {
        g.mem.write_u8(0x8000_0000 + TEX0 + i, 0x80).unwrap();
    }
    let mut s = setup();
    texture(&mut s, 0, TEX0, 1);
    texgen(&mut s);
    s.bp(0x00, (1 << 4) | 1).bp(0x28, 1 << 6);
    s.bp(0xF4, 0).bp(0xF5, 2 << 2);
    s.bp(0x40, 1 | (7 << 1) | 0x10);
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
    let snap = g.run(&s);
    assert_eq!(snap.depth[10 * 640 + 10], 0x80, "Z texture");

    // Indirect texturing: map 0 is dark on the left half, bright on the
    // right; map 1 offsets every lookup by four texels to the right.
    for y in 0..8u32 {
        for x in 0..8u32 {
            // I8 tiles are 8x4: (x, y) sits in block y / 4.
            let at = (y / 4) * 32 + (y % 4) * 8 + x;
            g.mem.write_u8(0x8000_0000 + TEX0 + at, if x < 4 { 0 } else { 255 }).unwrap();
        }
    }
    for i in 0..64 {
        // IA8, 4x4 tiles of (A, I): alpha 64 everywhere.
        g.mem.write_u16(0x8000_0000 + TEX1 + 2 * i, 0x4000).unwrap();
    }
    let indirect = |on: bool| {
        let mut s = setup();
        texture(&mut s, 0, TEX0, 1);
        texture(&mut s, 1, TEX1, 3);
        texgen(&mut s);
        s.bp(0x00, (1 << 4) | 1 | if on { 1 << 16 } else { 0 });
        s.bp(0x28, 1 << 6);
        s.bp(0xC0, (15 << 12) | (15 << 8) | (15 << 4) | 8 | (1 << 19)); // colour = texture
        if on {
            s.bp(0x27, 1).bp(0x25, 0);
            s.bp(0x06, 64 | (1 << 22)).bp(0x07, 0).bp(0x08, 1 << 22);
            s.bp(0x10, 1 << 9);
        }
        s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
        s
    };
    let snap = g.run(&indirect(false));
    assert_eq!(px(&snap, 12, 12), [0, 0, 0, 255], "texel 1 without indirect");
    let snap = g.run(&indirect(true));
    assert_eq!(px(&snap, 12, 12), [255, 255, 255, 255], "texel 1 + 4 with indirect");

    // EFB copy to RGB565 in RAM: the writeback carries red, quantised.
    let mut s = setup();
    s.quad(0.0, 0.0, 64.0, 64.0, 0.5, [255, 0, 0, 255]);
    s.bp(0x49, 0).bp(0x4A, 7 | (7 << 10)).bp(0x4B, 0x3000 >> 5);
    s.bp(0x52, 4 << 4); // format 4 (RGB565): t = 0b1000
    g.run(&s);
    g.r.finish().unwrap();
    let wb = g.r.take_writebacks();
    assert_eq!(wb.len(), 1);
    assert_eq!((wb[0].addr, wb[0].tex_format, wb[0].width), (0x3000, gx::texture::RGB565, 8));
    let bytes = gx::texture::encode(wb[0].tex_format, 8, 8, &wb[0].rgba);
    assert_eq!(&bytes[..2], &[0xF8, 0x00], "RGB565 red");
}
