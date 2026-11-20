//! Transform unit (XF), done on the CPU: position and normal matrices,
//! projection and viewport, the two lighting channels, and texture
//! coordinate generation. The output is in Vulkan clip space for an EFB of
//! 640x528 pixels, so the renderer's vertex shader passes it through.

use crate::vertex::RawVertex;
use crate::Vertex;

pub const EFB_WIDTH: f32 = 640.0;
pub const EFB_HEIGHT: f32 = 528.0;

/// XF registers live at 0x1000 + index.
pub const REGS: usize = 0x58;

/// Per-draw constants for [`Xf::transform`].
pub struct DrawSetup {
    pub screen: Screen,
    /// Which colours the vertex format carries.
    pub has_color: [bool; 2],
    pub projection: [f32; 6],
    pub ortho: bool,
    pub viewport: [f32; 6],
    /// A lit channel needs the transformed normal.
    pub need_normal: bool,
    /// The normal matrix is needed (lighting or emboss).
    need_nrm_mtx: bool,
    /// Colour channels in use, and per channel the colour and alpha control
    /// words, material and ambient registers.
    chans: usize,
    ctl: [[u32; 2]; 2],
    mat: [[f32; 4]; 2],
    amb: [[f32; 4]; 2],
    /// The eight lights, decoded (only when a channel is lit).
    lights: [Light; 8],
    /// Texgens in use, their info words, dual-texture flag, and post-transform
    /// (normalise first, rows) per texgen.
    gens: usize,
    gen_info: [u32; 8],
    dual: bool,
    post: [(bool, [[f32; 4]; 3]); 8],
}

/// One XF light, decoded from its 16 words.
#[derive(Clone, Copy, Default)]
pub(crate) struct Light {
    color: [f32; 4],
    cos_a: [f32; 3],
    dist_k: [f32; 3],
    pos: [f32; 3],
    dir: [f32; 3],
}

/// A player's window on the world (docs/PORT_PLAN.md, section 12): `size`
/// in world units, centred `offset` from the game camera's centre (y up).
/// Applied after the transform by [`crate::World::apply`], so one field can
/// be shown through any number of windows.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct View {
    pub offset: [f32; 2],
    pub size: [f32; 2],
}

/// Whether `pr` is the projection of a camera `camera` in size: orthographic
/// (the caller checks), centred, `2 / width` and `2 / height`.
pub fn is_camera_projection(pr: &[f32; 6], camera: [f32; 2]) -> bool {
    let close = |a: f32, b: f32| (a - b).abs() <= b.abs() * 1e-4;
    pr[1] == 0.0 && pr[3] == 0.0 && close(pr[0] * camera[0], 2.0) && close(pr[2] * camera[1], 2.0)
}

pub struct Xf {
    /// Matrix and light memory, words 0x000..0x800 (floats as bits).
    pub mem: Vec<u32>,
    pub regs: [u32; REGS],
    /// Bumped by writes that change a [`DrawSetup`] (registers, post
    /// matrices, lights), but not by position, normal or texture matrices.
    pub setup_gen: u64,
}

impl Default for Xf {
    fn default() -> Self {
        Self { mem: vec![0; 0x800], regs: [0; REGS], setup_gen: 0 }
    }
}

fn f(v: u32) -> f32 {
    f32::from_bits(v)
}

fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let l = dot3(v, v).sqrt();
    if l > 0.0 {
        [v[0] / l, v[1] / l, v[2] / l]
    } else {
        v
    }
}

fn unpack_rgba(v: u32) -> [f32; 4] {
    [(v >> 24) as u8 as f32 / 255.0, (v >> 16) as u8 as f32 / 255.0, (v >> 8) as u8 as f32 / 255.0, v as u8 as f32 / 255.0]
}

/// Where the viewport and scissor offset place EFB pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Screen {
    /// Scissor offset in pixels (BP 0x59, already doubled).
    pub offset_x: f32,
    pub offset_y: f32,
}

/// A world-space (view-space) position through projection `pr` and viewport
/// `vp` to Vulkan clip space for the 640x528 EFB. Linear in the homogeneous
/// coordinates, so clipping still works.
pub fn project(world: [f32; 3], pr: &[f32; 6], ortho: bool, vp: &[f32; 6], screen: Screen) -> [f32; 4] {
    let (x, y, z) = (world[0], world[1], world[2]);
    let clip = if ortho {
        [pr[0] * x + pr[1], pr[2] * y + pr[3], pr[4] * z + pr[5], 1.0]
    } else {
        [pr[0] * x + pr[1] * z, pr[2] * y + pr[3] * z, pr[4] * z + pr[5], -z]
    };
    let (wd, ht, zrange, xorig, yorig, farz) = (vp[0], vp[1], vp[2], vp[3], vp[4], vp[5]);
    let w = clip[3];
    let cx = xorig - screen.offset_x;
    let cy = yorig - screen.offset_y;
    [
        w * (cx / (EFB_WIDTH / 2.0) - 1.0) + clip[0] * (wd / (EFB_WIDTH / 2.0)),
        w * (cy / (EFB_HEIGHT / 2.0) - 1.0) + clip[1] * (ht / (EFB_HEIGHT / 2.0)),
        (w * farz + clip[2] * zrange) / 16_777_216.0,
        w,
    ]
}

/// The world-space position of a model-space point under position matrix `m`.
pub fn apply(m: &[[f32; 4]; 3], p: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|r| m[r][0] * p[0] + m[r][1] * p[1] + m[r][2] * p[2] + m[r][3])
}

impl Xf {
    pub fn write(&mut self, addr: u32, v: u32) {
        match addr {
            0..=0x7FF => {
                self.mem[addr as usize] = v;
                if addr >= 0x500 {
                    self.setup_gen += 1;
                }
            }
            0x1000..=0x1057 => {
                self.regs[(addr - 0x1000) as usize] = v;
                self.setup_gen += 1;
            }
            _ => {}
        }
    }

    fn reg(&self, a: u32) -> u32 {
        self.regs[(a - 0x1000) as usize]
    }

    /// Row `r` of the 3x4 or 2x4 matrix at row index `idx`.
    fn row(&self, idx: usize, r: usize) -> [f32; 4] {
        let b = (idx * 4 + r * 4) & 0xFF;
        std::array::from_fn(|c| f(self.mem[(b + c) & 0x3FF]))
    }

    /// Position matrix `idx` (three rows of four).
    pub fn position_matrix(&self, idx: u8) -> [[f32; 4]; 3] {
        [self.row(idx as usize, 0), self.row(idx as usize, 1), self.row(idx as usize, 2)]
    }

    /// Projection registers and whether the projection is orthographic.
    pub fn projection(&self) -> ([f32; 6], bool) {
        (std::array::from_fn(|i| f(self.reg(0x1020 + i as u32))), self.reg(0x1026) & 1 != 0)
    }

    /// Viewport registers: wd, ht, zRange, xOrig, yOrig, farZ.
    pub fn viewport(&self) -> [f32; 6] {
        std::array::from_fn(|i| f(self.reg(0x101A + i as u32)))
    }

    pub fn default_matrices(&self) -> (u8, [u8; 8]) {
        let (a, b) = (self.reg(0x1018), self.reg(0x1019));
        let tex = [a >> 6, a >> 12, a >> 18, a >> 24, b, b >> 6, b >> 12, b >> 18];
        ((a & 0x3F) as u8, tex.map(|t| (t & 0x3F) as u8))
    }

    /// What every vertex of one draw shares: projection, viewport, and
    /// whether any lit channel needs the transformed normal.
    pub fn setup(&self, screen: Screen, has_color: [bool; 2]) -> DrawSetup {
        let (projection, ortho) = self.projection();
        let chans = ((self.reg(0x1009) & 3) as usize).min(2);
        let ctl: [[u32; 2]; 2] = std::array::from_fn(|c| [self.reg(0x100E + c as u32), self.reg(0x1010 + c as u32)]);
        let lit = (0..chans).any(|c| (ctl[c][0] | ctl[c][1]) & 2 != 0);
        let mut lights = [Light::default(); 8];
        if lit {
            for (l, light) in lights.iter_mut().enumerate() {
                let b = 0x600 + l * 16;
                let g = |i: usize| f(self.mem[b + i]);
                *light = Light {
                    color: unpack_rgba(self.mem[b + 3]),
                    cos_a: [g(4), g(5), g(6)],
                    dist_k: [g(7), g(8), g(9)],
                    pos: [g(10), g(11), g(12)],
                    dir: [g(13), g(14), g(15)],
                };
            }
        }
        let gens = ((self.reg(0x103F) & 15) as usize).min(8);
        let gen_info: [u32; 8] = std::array::from_fn(|i| self.reg(0x1040 + i as u32));
        let dual = self.reg(0x1012) & 1 != 0;
        let emboss = gen_info[..gens].iter().any(|info| (info >> 4) & 7 == 1);
        let post = std::array::from_fn(|i| {
            if !dual || i >= gens {
                return (false, [[0.0; 4]; 3]);
            }
            let post = self.reg(0x1050 + i as u32);
            let base = 0x500 + ((post & 0x3F) as usize) * 4;
            ((post >> 8) & 1 != 0, std::array::from_fn(|r| std::array::from_fn(|k| f(self.mem[(base + r * 4 + k) & 0x7FF]))))
        });
        DrawSetup {
            screen,
            has_color,
            projection,
            ortho,
            viewport: self.viewport(),
            need_normal: lit,
            need_nrm_mtx: lit || emboss,
            chans,
            ctl,
            mat: std::array::from_fn(|c| unpack_rgba(self.reg(0x100C + c as u32))),
            amb: std::array::from_fn(|c| unpack_rgba(self.reg(0x100A + c as u32))),
            lights,
            gens,
            gen_info,
            dual,
            post,
        }
    }

    /// Transforms one loaded vertex of a draw set up by [`Xf::setup`].
    pub fn transform(&self, v: &RawVertex, d: &DrawSetup) -> Vertex {
        let (screen, has_color) = (d.screen, d.has_color);
        let p = [v.pos[0], v.pos[1], v.pos[2], 1.0];
        let pm = v.pnmtx as usize;
        let world: [f32; 3] = std::array::from_fn(|r| {
            let row = self.row(pm, r);
            row[0] * p[0] + row[1] * p[1] + row[2] * p[2] + row[3]
        });
        let nb = 0x400 + (pm & 31) * 3;
        let nrm_row = |r: usize| -> [f32; 3] { std::array::from_fn(|c| f(self.mem[(nb + r * 3 + c) & 0x7FF])) };
        let n_rows = if d.need_nrm_mtx { [nrm_row(0), nrm_row(1), nrm_row(2)] } else { [[0.0; 3]; 3] };
        let xform_n = |n: [f32; 3]| -> [f32; 3] { std::array::from_fn(|r| dot3(n_rows[r], n)) };
        let normal = if d.need_normal { normalize(xform_n(v.nbt[0])) } else { [0.0; 3] };

        let pos = project(world, &d.projection, d.ortho, &d.viewport, screen);

        // Lighting channels.
        let mut color = [[0.0f32; 4]; 2];
        for c in 0..d.chans {
            let vc = if has_color[c] { v.color[c] } else { [1.0; 4] };
            let [cc, ac] = d.ctl[c];
            let rgb = light_channel(d, cc, vc, c, world, normal);
            let a = if ac == cc { rgb } else { light_channel(d, ac, vc, c, world, normal) };
            color[c] = [rgb[0], rgb[1], rgb[2], a[3]];
        }

        // Texture coordinate generation.
        let (gens, dual) = (d.gens, d.dual);
        let mut tex = [[0.0f32; 3]; 8];
        for i in 0..gens {
            let info = d.gen_info[i];
            let stq = (info >> 1) & 1 != 0;
            let ab11 = (info >> 2) & 1 == 0;
            let kind = (info >> 4) & 7;
            let src_row = (info >> 7) & 31;
            let mut src = match src_row {
                0 => [v.pos[0], v.pos[1], v.pos[2], 1.0],
                1 => [v.nbt[0][0], v.nbt[0][1], v.nbt[0][2], 1.0],
                2 => [color[0][0], color[0][1], color[0][2], 1.0],
                3 => [v.nbt[1][0], v.nbt[1][1], v.nbt[1][2], 1.0],
                4 => [v.nbt[2][0], v.nbt[2][1], v.nbt[2][2], 1.0],
                5..=12 => {
                    let t = v.tex[(src_row - 5) as usize];
                    [t[0], t[1], 1.0, 1.0]
                }
                _ => [0.0, 0.0, 1.0, 1.0],
            };
            if ab11 {
                src[2] = 1.0;
            }
            let mut c = match kind {
                // Colour texgens take s and t from the lit colour.
                2 => [color[0][0], color[0][1], 1.0],
                3 => [color[1][0], color[1][1], 1.0],
                // Emboss: the source texgen's coordinates moved by the light
                // direction in the surface's tangent space.
                1 => {
                    let s = ((info >> 12) & 7) as usize;
                    let light = ((info >> 15) & 7) as usize;
                    let base = if s < i { tex[s] } else { [0.0, 0.0, 1.0] };
                    let b = 0x600 + light * 16;
                    let lpos: [f32; 3] = std::array::from_fn(|k| f(self.mem[b + 10 + k]));
                    let ldir = normalize(sub3(lpos, world));
                    let tangent = normalize(xform_n(v.nbt[1]));
                    let binormal = normalize(xform_n(v.nbt[2]));
                    [base[0] + dot3(ldir, tangent), base[1] + dot3(ldir, binormal), base[2]]
                }
                _ => {
                    let m = v.texmtx[i] as usize;
                    let d = |r: usize| {
                        let row = self.row(m, r);
                        row[0] * src[0] + row[1] * src[1] + row[2] * src[2] + row[3] * src[3]
                    };
                    [d(0), d(1), if stq { d(2) } else { 1.0 }]
                }
            };
            if dual && kind == 0 {
                let (norm, rows) = &d.post[i];
                if *norm {
                    c = normalize(c);
                }
                let prow = |r: usize| -> f32 {
                    let w = rows[r];
                    w[0] * c[0] + w[1] * c[1] + w[2] * c[2] + w[3]
                };
                c = [prow(0), prow(1), if stq { prow(2) } else { 1.0 }];
            }
            if c[2] == 0.0 {
                c[2] = 1.0;
            }
            tex[i] = c;
        }
        Vertex { pos, color, tex }
    }

}

/// One channel control word (colour or alpha) for channel `c`.
fn light_channel(d: &DrawSetup, ctl: u32, vertex: [f32; 4], c: usize, pos: [f32; 3], normal: [f32; 3]) -> [f32; 4] {
    let mat = if ctl & 1 != 0 { vertex } else { d.mat[c] };
    if ctl & 2 == 0 {
        return mat;
    }
    let amb = if ctl & 0x40 != 0 { vertex } else { d.amb[c] };
    let diffuse_fn = (ctl >> 7) & 3;
    let spot = (ctl >> 9) & 1 != 0;
    let attn_on = (ctl >> 10) & 1 != 0;
    let mask = ((ctl >> 2) & 0xF) | (((ctl >> 11) & 0xF) << 4);
    let mut sum = amb;
    for (l, light) in d.lights.iter().enumerate() {
        if mask & (1 << l) == 0 {
            continue;
        }
        let Light { color: lc, cos_a, dist_k, pos: lpos, dir: ldir } = *light;
        let (dir, attn) = if !attn_on {
            (normalize(sub3(lpos, pos)), 1.0)
        } else if spot {
            let dv = sub3(lpos, pos);
            let dist2 = dot3(dv, dv);
            let dist = dist2.sqrt();
            let dir = if dist > 0.0 { [dv[0] / dist, dv[1] / dist, dv[2] / dist] } else { dv };
            let cs = dot3(dir, ldir).max(0.0);
            let num = (cos_a[0] + cos_a[1] * cs + cos_a[2] * cs * cs).max(0.0);
            let den = dist_k[0] + dist_k[1] * dist + dist_k[2] * dist2;
            (dir, if den != 0.0 { num / den } else { 0.0 })
        } else {
            // Specular: the light "position" holds the light direction and
            // "direction" the half-angle vector.
            let dir = normalize(lpos);
            let s = if dot3(normal, dir) >= 0.0 { dot3(normal, ldir).max(0.0) } else { 0.0 };
            let num = (cos_a[0] + cos_a[1] * s + cos_a[2] * s * s).max(0.0);
            let den = dist_k[0] + dist_k[1] * s + dist_k[2] * s * s;
            (dir, if den != 0.0 { num / den } else { 0.0 })
        };
        let nd = dot3(normal, dir);
        let diffuse = match diffuse_fn {
            0 => 1.0,
            1 => nd,
            _ => nd.max(0.0),
        };
        for k in 0..4 {
            sum[k] += attn * diffuse * lc[k];
        }
    }
    std::array::from_fn(|k| mat[k] * sum[k].clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ortho_identity_maps_to_efb_pixels() {
        let mut xf = Xf::default();
        // Identity position matrix 0.
        for (i, v) in [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0].iter().enumerate() {
            xf.mem[i] = v.to_bits();
        }
        // Ortho projection mapping x 0..640 and y 0..480 to -1..1 (y up).
        let p = [2.0f32 / 640.0, -1.0, -2.0 / 480.0, 1.0, 0.0, 0.0];
        for (i, v) in p.iter().enumerate() {
            xf.regs[0x20 + i] = v.to_bits();
        }
        xf.regs[0x26] = 1;
        // Viewport 640x480 at the origin: wd, ht, zrange, xorig, yorig, farz.
        let vp = [320.0f32, -240.0, 16_777_215.0, 320.0 + 342.0, 240.0 + 342.0, 16_777_215.0];
        for (i, v) in vp.iter().enumerate() {
            xf.regs[0x1A + i] = v.to_bits();
        }
        xf.regs[0x09] = 1;
        xf.regs[0x0C] = 0xFF80_40FF; // material colour, lighting off
        let screen = Screen { offset_x: 342.0, offset_y: 342.0 };
        let raw = RawVertex { pos: [160.0, 120.0, 0.0], ..Default::default() };
        let v = xf.transform(&raw, &xf.setup(screen, [false, false]));
        // EFB pixel (160, 120) -> Vulkan NDC.
        assert!((v.pos[0] - (160.0 / 320.0 - 1.0)).abs() < 1e-5);
        assert!((v.pos[1] - (120.0 / 264.0 - 1.0)).abs() < 1e-5);
        assert_eq!(v.pos[3], 1.0);
        assert_eq!(v.color[0], [1.0, 128.0 / 255.0, 64.0 / 255.0, 1.0]);
    }
}
