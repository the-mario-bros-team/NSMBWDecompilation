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
    /// The picture's aspect when wider (or narrower) than the game's 16:9:
    /// 16:9 projections are widened to it (0: none). Transform half only.
    pub wide: f32,
    /// Bumped by writes that change a [`DrawSetup`] (registers, post
    /// matrices, lights), but not by position, normal or texture matrices.
    pub setup_gen: u64,
}

impl Default for Xf {
    fn default() -> Self {
        Self { mem: vec![0; 0x800], regs: [0; REGS], wide: 0.0, setup_gen: 0 }
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
        let (mut pr, ortho) = self.game_projection();
        // A wider picture: a camera set up for 16:9 (aspect 1.68 to 1.88)
        // takes the picture's aspect, with the same view height and more
        // to the sides (Hor+), around the same centre. Perspective cameras
        // (the world map, the title) and screen-sized orthographic ones
        // (the levels, 2D scenes); small orthographic views (render to
        // texture) and cameras the game set up wider are left alone.
        if self.wide > 0.0 && pr[0] != 0.0 && pr[2] != 0.0 {
            let aspect = pr[2] / pr[0];
            let screen_sized = !ortho || 2.0 / pr[2] > 150.0;
            if aspect > 1.68 && aspect < 1.88 && screen_sized {
                let widen = aspect / self.wide;
                pr[0] *= widen;
                pr[1] *= widen;
            }
        }
        (pr, ortho)
    }

    /// The projection registers as the game wrote them (before any
    /// widening), and whether the projection is orthographic.
    pub fn game_projection(&self) -> ([f32; 6], bool) {
        (std::array::from_fn(|i| f(self.reg(0x1020 + i as u32))), self.reg(0x1026) & 1 != 0)
    }

    /// Sets the aspect 16:9 projections are widened to (0: none).
    pub fn set_wide(&mut self, aspect: f32) {
        if self.wide != aspect {
            self.wide = aspect;
            self.setup_gen += 1;
        }
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
        let mut out = std::mem::MaybeUninit::<Vertex>::uninit();
        // SAFETY: `transform_one` writes every field of `out`.
        unsafe {
            transform_one(self.words(), &self.words(), v, d, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// Transforms a draw's vertices and appends them to `out`, each written
    /// in place. Built on the stack and copied out, a vertex's texture
    /// coordinates (stored a float at a time) were read back with wide loads
    /// that cannot take their data from those stores, and every vertex paid
    /// a call's prologue: about 80 ns a vertex.
    ///
    /// `same_matrices`: every vertex uses the same position and texture
    /// matrices (the format carries no matrix indices), so they are read
    /// from XF memory once for the draw instead of once a vertex. The
    /// results are the same either way.
    pub fn transform_into(&self, raws: &[RawVertex], d: &DrawSetup, out: &mut Vec<Vertex>, same_matrices: bool) {
        let mem = self.words();
        match raws.first() {
            Some(first) if same_matrices && raws.len() > 1 => {
                let m = Hoisted::new(mem, first, d);
                self.transform_with(raws, d, out, &m);
            }
            _ => self.transform_with(raws, d, out, &mem),
        }
    }

    #[inline(always)]
    fn transform_with(&self, raws: &[RawVertex], d: &DrawSetup, out: &mut Vec<Vertex>, m: &impl Matrices) {
        let mem = self.words();
        out.reserve(raws.len());
        let start = out.len();
        for (v, slot) in raws.iter().zip(out.spare_capacity_mut()) {
            // SAFETY: `transform_one` writes every field of the slot.
            unsafe { transform_one(mem, m, v, d, slot.as_mut_ptr()) };
        }
        // SAFETY: the first `raws.len()` spare slots were written above.
        unsafe { out.set_len(start + raws.len()) };
    }

    /// XF memory as a fixed-size array, so masked indices need no bounds
    /// checks.
    fn words(&self) -> &[u32; 0x800] {
        self.mem.as_slice().try_into().expect("XF memory is 0x800 words")
    }
}

/// Row `r` of the matrix at row index `idx` (see [`Xf::row`]).
#[inline(always)]
fn row_of(mem: &[u32; 0x800], idx: usize, r: usize) -> [f32; 4] {
    let b = (idx * 4 + r * 4) & 0xFF;
    std::array::from_fn(|c| f(mem[(b + c) & 0x3FF]))
}

/// Where a vertex's position, normal and texture matrices come from: XF
/// memory, by the vertex's own indices, or read once for a whole draw.
trait Matrices {
    /// Row `r` of the position matrix.
    fn pos_row(&self, v: &RawVertex, r: usize) -> [f32; 4];
    /// The normal matrix's rows.
    fn nrm_rows(&self, v: &RawVertex) -> [[f32; 3]; 3];
    /// Row `r` of texgen `i`'s matrix.
    fn tex_row(&self, v: &RawVertex, i: usize, r: usize) -> [f32; 4];
}

impl Matrices for &[u32; 0x800] {
    #[inline(always)]
    fn pos_row(&self, v: &RawVertex, r: usize) -> [f32; 4] {
        row_of(self, v.pnmtx as usize, r)
    }

    #[inline(always)]
    fn nrm_rows(&self, v: &RawVertex) -> [[f32; 3]; 3] {
        let nb = 0x400 + (v.pnmtx as usize & 31) * 3;
        let nrm_row = |r: usize| -> [f32; 3] { std::array::from_fn(|c| f(self[(nb + r * 3 + c) & 0x7FF])) };
        [nrm_row(0), nrm_row(1), nrm_row(2)]
    }

    #[inline(always)]
    fn tex_row(&self, v: &RawVertex, i: usize, r: usize) -> [f32; 4] {
        row_of(self, v.texmtx[i] as usize, r)
    }
}

/// A draw's matrices read once, for vertices that carry no matrix indices
/// (all use the first vertex's, which are the defaults).
struct Hoisted {
    pos: [[f32; 4]; 3],
    nrm: [[f32; 3]; 3],
    tex: [[[f32; 4]; 3]; 8],
}

impl Hoisted {
    fn new(mem: &[u32; 0x800], first: &RawVertex, d: &DrawSetup) -> Self {
        let pos = std::array::from_fn(|r| mem.pos_row(first, r));
        let nrm = if d.need_nrm_mtx { mem.nrm_rows(first) } else { [[0.0; 3]; 3] };
        let mut tex = [[[0.0; 4]; 3]; 8];
        for (i, t) in tex.iter_mut().enumerate().take(d.gens) {
            *t = std::array::from_fn(|r| mem.tex_row(first, i, r));
        }
        Self { pos, nrm, tex }
    }
}

impl Matrices for Hoisted {
    #[inline(always)]
    fn pos_row(&self, _: &RawVertex, r: usize) -> [f32; 4] {
        self.pos[r]
    }

    #[inline(always)]
    fn nrm_rows(&self, _: &RawVertex) -> [[f32; 3]; 3] {
        self.nrm
    }

    #[inline(always)]
    fn tex_row(&self, _: &RawVertex, i: usize, r: usize) -> [f32; 4] {
        self.tex[i][r]
    }
}

/// [`Xf::transform`] into `out`, writing every field.
///
/// # Safety
/// `out` must be valid for writes of a [`Vertex`].
#[inline(always)]
unsafe fn transform_one(mem: &[u32; 0x800], mats: &impl Matrices, v: &RawVertex, d: &DrawSetup, out: *mut Vertex) {
    let (screen, has_color) = (d.screen, d.has_color);
    let p = [v.pos[0], v.pos[1], v.pos[2], 1.0];
    let world: [f32; 3] = std::array::from_fn(|r| {
        let row = mats.pos_row(v, r);
        row[0] * p[0] + row[1] * p[1] + row[2] * p[2] + row[3]
    });
    let n_rows = if d.need_nrm_mtx { mats.nrm_rows(v) } else { [[0.0; 3]; 3] };
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
    // Written in place, entry by entry; emboss reads earlier entries back.
    // SAFETY (all uses): `out` is valid for writes, and entries are read
    // only after being written.
    let tex = unsafe { std::ptr::addr_of_mut!((*out).tex) }.cast::<[f32; 3]>();
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
                let base = if s < i { unsafe { tex.add(s).read() } } else { [0.0, 0.0, 1.0] };
                let b = 0x600 + light * 16;
                let lpos: [f32; 3] = std::array::from_fn(|k| f(mem[b + 10 + k]));
                let ldir = normalize(sub3(lpos, world));
                let tangent = normalize(xform_n(v.nbt[1]));
                let binormal = normalize(xform_n(v.nbt[2]));
                [base[0] + dot3(ldir, tangent), base[1] + dot3(ldir, binormal), base[2]]
            }
            _ => {
                let d = |r: usize| {
                    let row = mats.tex_row(v, i, r);
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
        unsafe { tex.add(i).write(c) };
    }
    for i in gens..8 {
        unsafe { tex.add(i).write([0.0; 3]) };
    }
    unsafe {
        std::ptr::addr_of_mut!((*out).pos).write(pos);
        std::ptr::addr_of_mut!((*out).color).write(color);
    }
}

/// One channel control word (colour or alpha) for channel `c`. Inlined into
/// the vertex loop: as a call, made once or twice a vertex, it saved and
/// restored eight vector registers each time.
#[inline(always)]
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

/// XF state for draws the renderer transforms ([`crate::Output::packed`]),
/// as blocks of words in [`crate::Output::xf_blocks`]: a setup block
/// (projection, viewport, lighting channels, lights, texgens) and a matrix
/// block per change of matrices. `render`'s `packed.wgsl` reads them at
/// these offsets; [`transform_packed`] is the same on the CPU.
pub mod block {
    /// Setup block: projection (6 floats), orthographic (0/1), viewport
    /// (6: wd, ht, zRange, xOrig, yOrig, farZ), channels in use, the colour
    /// and alpha control words of channels 0 and 1, material and ambient
    /// colours (2 x RGBA floats each), texgens in use, dual texture (0/1),
    /// whether lighting needs the normal and the normal matrix, the eight
    /// texgen info words, the eight post matrices (3 rows of 4), a mask of
    /// the texgens normalised before theirs, and the eight lights (16 words
    /// each: RGBA colour, cos a0-a2, dist k0-k2, position, direction).
    pub const PROJECTION: usize = 0;
    pub const ORTHO: usize = 6;
    pub const VIEWPORT: usize = 7;
    pub const CHANS: usize = 13;
    pub const CTL: usize = 14;
    pub const MAT: usize = 18;
    pub const AMB: usize = 26;
    pub const GENS: usize = 34;
    pub const DUAL: usize = 35;
    pub const NEED_NORMAL: usize = 36;
    pub const NEED_NRM_MTX: usize = 37;
    pub const GEN_INFO: usize = 38;
    pub const POST: usize = 46;
    pub const POST_NORM: usize = 142;
    pub const LIGHTS: usize = 143;
    pub const SETUP_WORDS: usize = 272;

    /// Matrix block, both kinds: the scissor offset (x, y; floats).
    pub const SCREEN: usize = 0;
    /// The draw's own matrices, for vertices without matrix indices:
    /// position (3 rows of 4), normal (3 of 3), and each texgen's (3 of 4).
    pub const POS: usize = 2;
    pub const NRM: usize = 14;
    pub const TEX: usize = 23;
    pub const MATRIX_WORDS: usize = 119;
    /// All of matrix memory, for vertices with matrix indices: XF words
    /// 0x000-0x0FF (position and texture matrices) and 0x400-0x467
    /// (normal matrices).
    pub const MEM: usize = 2;
    pub const NRM_MEM: usize = 258;
    pub const FULL_MATRIX_WORDS: usize = 362;

    /// A state's flags ([`crate::Output::states`] word 3): the matrix block
    /// holds all of matrix memory.
    pub const FULL: u32 = 1;
}

impl Xf {
    /// Appends the setup block for `d` (built by [`Xf::setup`] from this
    /// state).
    pub(crate) fn write_setup_block(&self, d: &DrawSetup, out: &mut Vec<u32>) {
        let at = out.len();
        out.resize(at + block::SETUP_WORDS, 0);
        let b = &mut out[at..];
        let fl = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<u32>>();
        b[block::PROJECTION..block::PROJECTION + 6].copy_from_slice(&fl(&d.projection));
        b[block::ORTHO] = u32::from(d.ortho);
        b[block::VIEWPORT..block::VIEWPORT + 6].copy_from_slice(&fl(&d.viewport));
        b[block::CHANS] = d.chans as u32;
        b[block::CTL..block::CTL + 4].copy_from_slice(&[d.ctl[0][0], d.ctl[0][1], d.ctl[1][0], d.ctl[1][1]]);
        b[block::MAT..block::MAT + 8].copy_from_slice(&fl(d.mat.as_flattened()));
        b[block::AMB..block::AMB + 8].copy_from_slice(&fl(d.amb.as_flattened()));
        b[block::GENS] = d.gens as u32;
        b[block::DUAL] = u32::from(d.dual);
        b[block::NEED_NORMAL] = u32::from(d.need_normal);
        b[block::NEED_NRM_MTX] = u32::from(d.need_nrm_mtx);
        b[block::GEN_INFO..block::GEN_INFO + 8].copy_from_slice(&d.gen_info);
        for (i, (norm, rows)) in d.post.iter().enumerate() {
            b[block::POST + 12 * i..block::POST + 12 * i + 12].copy_from_slice(&fl(rows.as_flattened()));
            b[block::POST_NORM] |= u32::from(*norm) << i;
        }
        // All eight lights as XF memory has them: emboss reads a light's
        // position whether or not a channel is lit.
        for l in 0..8 {
            let m = 0x600 + l * 16;
            let g = |i: usize| self.mem[m + i];
            let o = block::LIGHTS + 16 * l;
            b[o..o + 4].copy_from_slice(&fl(&unpack_rgba(g(3))));
            for i in 0..12 {
                b[o + 4 + i] = g(4 + i);
            }
        }
    }

    /// Appends a matrix block: the scissor offset `screen`, then all of
    /// matrix memory (`full`, for vertices with matrix indices) or the
    /// default matrices' rows.
    pub(crate) fn write_matrix_block(&self, screen: Screen, full: bool, out: &mut Vec<u32>) {
        let mem = self.words();
        out.extend([screen.offset_x.to_bits(), screen.offset_y.to_bits()]);
        if full {
            out.extend_from_slice(&mem[..0x100]);
            out.extend_from_slice(&mem[0x400..0x468]);
            return;
        }
        let (pm, tm) = self.default_matrices();
        let v = RawVertex { pnmtx: pm, texmtx: tm, ..Default::default() };
        for r in 0..3 {
            out.extend(mem.pos_row(&v, r).map(f32::to_bits));
        }
        out.extend(mem.nrm_rows(&v).as_flattened().iter().map(|x| x.to_bits()));
        for i in 0..8 {
            for r in 0..3 {
                out.extend(mem.tex_row(&v, i, r).map(f32::to_bits));
            }
        }
    }
}

impl DrawSetup {
    /// The setup a setup block holds, for vertices whose format carries the
    /// colours `has_color`, at scissor offset `screen`.
    fn from_block(b: &[u32], screen: Screen, has_color: [bool; 2]) -> Self {
        let f = |i: usize| f32::from_bits(b[i]);
        let v4 = |i: usize| -> [f32; 4] { std::array::from_fn(|k| f(i + k)) };
        let v3 = |i: usize| -> [f32; 3] { std::array::from_fn(|k| f(i + k)) };
        let rows = |i: usize| -> [[f32; 4]; 3] { std::array::from_fn(|r| v4(i + 4 * r)) };
        DrawSetup {
            screen,
            has_color,
            projection: std::array::from_fn(|i| f(block::PROJECTION + i)),
            ortho: b[block::ORTHO] != 0,
            viewport: std::array::from_fn(|i| f(block::VIEWPORT + i)),
            need_normal: b[block::NEED_NORMAL] != 0,
            need_nrm_mtx: b[block::NEED_NRM_MTX] != 0,
            chans: b[block::CHANS] as usize,
            ctl: [[b[block::CTL], b[block::CTL + 1]], [b[block::CTL + 2], b[block::CTL + 3]]],
            mat: [v4(block::MAT), v4(block::MAT + 4)],
            amb: [v4(block::AMB), v4(block::AMB + 4)],
            lights: std::array::from_fn(|l| {
                let o = block::LIGHTS + 16 * l;
                Light { color: v4(o), cos_a: v3(o + 4), dist_k: v3(o + 7), pos: v3(o + 10), dir: v3(o + 13) }
            }),
            gens: b[block::GENS] as usize,
            gen_info: std::array::from_fn(|i| b[block::GEN_INFO + i]),
            dual: b[block::DUAL] != 0,
            post: std::array::from_fn(|i| ((b[block::POST_NORM] >> i) & 1 != 0, rows(block::POST + 12 * i))),
        }
    }
}

/// A matrix block's matrices.
struct BlockMatrices<'a> {
    b: &'a [u32],
    full: bool,
}

impl BlockMatrices<'_> {
    fn f(&self, i: usize) -> f32 {
        f32::from_bits(self.b[i])
    }

    /// Row `r` of the matrix at row index `idx` in the block's memory.
    fn mem_row(&self, idx: usize, r: usize) -> [f32; 4] {
        let at = (idx * 4 + r * 4) & 0xFF;
        std::array::from_fn(|c| self.f(block::MEM + ((at + c) & 0xFF)))
    }
}

impl Matrices for BlockMatrices<'_> {
    fn pos_row(&self, v: &RawVertex, r: usize) -> [f32; 4] {
        if self.full {
            self.mem_row(v.pnmtx as usize, r)
        } else {
            std::array::from_fn(|c| self.f(block::POS + 4 * r + c))
        }
    }

    fn nrm_rows(&self, v: &RawVertex) -> [[f32; 3]; 3] {
        if self.full {
            let nb = (v.pnmtx as usize & 31) * 3;
            std::array::from_fn(|r| std::array::from_fn(|c| self.f(block::NRM_MEM + nb + r * 3 + c)))
        } else {
            std::array::from_fn(|r| std::array::from_fn(|c| self.f(block::NRM + 3 * r + c)))
        }
    }

    fn tex_row(&self, v: &RawVertex, i: usize, r: usize) -> [f32; 4] {
        if self.full {
            self.mem_row(v.texmtx[i] as usize, r)
        } else {
            std::array::from_fn(|c| self.f(block::TEX + 12 * i + 4 * r + c))
        }
    }
}

/// What the renderer's `vs_packed` computes for the packed vertex at word
/// `at` of `packed`, drawn with `state` (an [`crate::Output::states`]
/// entry) over `blocks` ([`crate::Output::xf_blocks`]), on the CPU: the
/// same as transforming it before it was packed, bit for bit.
pub fn transform_packed(blocks: &[u32], state: [u32; 4], packed: &[u32], at: usize) -> Vertex {
    let pack = crate::vertex::Pack::from_bits(state[2]);
    let full = state[3] & block::FULL != 0;
    let setup = &blocks[state[0] as usize..state[0] as usize + block::SETUP_WORDS];
    let mb = &blocks[state[1] as usize..state[1] as usize + if full { block::FULL_MATRIX_WORDS } else { block::MATRIX_WORDS }];
    let screen = Screen { offset_x: f32::from_bits(mb[block::SCREEN]), offset_y: f32::from_bits(mb[block::SCREEN + 1]) };
    let d = DrawSetup::from_block(setup, screen, pack.colors());
    let v = pack.read(&packed[at..at + pack.words as usize], &RawVertex { color: [[1.0; 4]; 2], ..Default::default() });
    // Emboss reads the light positions from XF memory.
    let mut mem = [0u32; 0x800];
    for l in 0..8 {
        for k in 0..3 {
            mem[0x600 + 16 * l + 10 + k] = setup[block::LIGHTS + 16 * l + 10 + k];
        }
    }
    let mut out = std::mem::MaybeUninit::<Vertex>::uninit();
    // SAFETY: `transform_one` writes every field of `out`.
    unsafe {
        transform_one(&mem, &BlockMatrices { b: mb, full }, &v, &d, out.as_mut_ptr());
        out.assume_init()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Matrices read once for a draw give the same vertices, bit for bit,
    /// as matrices read for every vertex, with texgens from positions,
    /// normals and texture coordinates, dual-texture post matrices and
    /// lighting.
    #[test]
    fn hoisted_matrices_match() {
        let mut xf = Xf::default();
        for (i, w) in xf.mem.iter_mut().enumerate() {
            *w = (((i * 37 % 101) as f32 - 50.0) / 13.0).to_bits();
        }
        xf.regs[0x20] = 0.01f32.to_bits();
        xf.regs[0x22] = 0.02f32.to_bits();
        xf.regs[0x24] = (-0.5f32).to_bits();
        xf.regs[0x25] = 3.0f32.to_bits();
        for (i, v) in [320.0f32, -240.0, 16_777_215.0, 662.0, 582.0, 16_777_215.0].iter().enumerate() {
            xf.regs[0x1A + i] = v.to_bits();
        }
        xf.regs[0x09] = 2;
        // Channel 0 lit by lights 0 and 1 (diffuse clamped, attenuation off).
        xf.regs[0x0E] = 0x1 | 0x2 | (0b11 << 2) | (2 << 7);
        xf.regs[0x10] = 0x700;
        xf.regs[0x12] = 1;
        xf.regs[0x18] = 3 | (6 << 6) | (9 << 12) | (12 << 18);
        xf.regs[0x3F] = 4;
        // Texgens: tex0 (ST), normal (STQ), position (STQ), tex1.
        xf.regs[0x40] = 5 << 7;
        xf.regs[0x41] = (1 << 7) | 2 | 4;
        xf.regs[0x42] = 2 | 4;
        xf.regs[0x43] = 6 << 7;
        for i in 0..4 {
            xf.regs[0x50 + i] = (3 * i as u32) | if i == 1 { 0x100 } else { 0 };
        }
        let (pm, tm) = xf.default_matrices();
        let raws: Vec<RawVertex> = (0..7)
            .map(|k| {
                let k = k as f32;
                RawVertex { pnmtx: pm, texmtx: tm, pos: [k, 2.0 * k - 3.0, 0.5 * k], nbt: [[0.3, k, -1.0], [0.0; 3], [0.0; 3]], color: [[0.5, 0.25, k / 8.0, 1.0], [1.0; 4]], tex: [[k / 4.0, 1.0 - k / 4.0], [k, -k], [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2]] }
            })
            .collect();
        let setup = xf.setup(Screen { offset_x: 342.0, offset_y: 342.0 }, [true, false]);
        let (mut a, mut b) = (Vec::new(), Vec::new());
        xf.transform_into(&raws, &setup, &mut a, true);
        xf.transform_into(&raws, &setup, &mut b, false);
        let bits = |v: &[Vertex]| -> Vec<u32> { v.iter().flat_map(|v| v.pos.iter().chain(v.color.iter().flatten()).chain(v.tex.iter().flatten()).map(|f| f.to_bits()).collect::<Vec<_>>()).collect() };
        assert_eq!(bits(&a), bits(&b));
        assert!(a.iter().any(|v| v.tex[1] != [0.0; 3]) && a.iter().any(|v| v.color[0] != [0.0; 4]));
    }

    /// Blocks and packed vertices, transformed as the renderer does
    /// ([`transform_packed`]), give the CPU transform's vertices bit for bit:
    /// lit channels (spot, specular and plain lights), emboss and colour
    /// texgens, per-vertex matrix indices (all of matrix memory) and NBT.
    #[test]
    fn packed_blocks_match_the_cpu() {
        for full in [false, true] {
            let mut xf = Xf::default();
            for (i, w) in xf.mem.iter_mut().enumerate() {
                *w = (((i * 71 % 113) as f32 - 56.0) / 11.0).to_bits();
            }
            // Light colours as RGBA8.
            for l in 0..8 {
                xf.mem[0x600 + 16 * l + 3] = 0x8040_C0FF ^ (l as u32 * 0x1111_1111);
            }
            for (i, v) in [0.01f32, 0.3, 0.02, -0.2, -0.5, 3.0].iter().enumerate() {
                xf.regs[0x20 + i] = v.to_bits();
            }
            for (i, v) in [320.0f32, -240.0, 16_777_215.0, 662.0, 582.0, 16_777_215.0].iter().enumerate() {
                xf.regs[0x1A + i] = v.to_bits();
            }
            xf.regs[0x09] = 2;
            xf.regs[0x0A] = 0x2040_6080;
            xf.regs[0x0C] = 0x8040_20FF;
            xf.regs[0x0D] = 0x1122_3344;
            // Channel 0: lit by lights 0-3, attenuation, spot; its alpha lit
            // without attenuation; channel 1: specular (attenuation, no spot).
            xf.regs[0x0E] = 0x1 | 0x2 | (0xF << 2) | (2 << 7) | (1 << 9) | (1 << 10);
            xf.regs[0x10] = 0x2 | (0x3 << 2) | (1 << 7) | 0x40;
            xf.regs[0x0F] = 0x2 | (0x1 << 11) | (1 << 10) | (2 << 7);
            xf.regs[0x11] = 0x700;
            xf.regs[0x12] = 1;
            xf.regs[0x18] = 3 | (6 << 6) | (9 << 12) | (12 << 18) | (15 << 24);
            xf.regs[0x3F] = 5;
            // Texgens: tex0, normal (STQ, normalised after), emboss of 0 by
            // light 2, colour 1, binormal (STQ).
            xf.regs[0x40] = 5 << 7;
            xf.regs[0x41] = (1 << 7) | 2 | 4;
            xf.regs[0x42] = (1 << 4) | (2 << 15);
            xf.regs[0x43] = 3 << 4;
            xf.regs[0x44] = (4 << 7) | 2 | 4;
            for i in 0..5 {
                xf.regs[0x50 + i] = (3 * i as u32 + 1) | if i == 1 { 0x100 } else { 0 };
            }
            let (pm, tm) = xf.default_matrices();
            let raws: Vec<RawVertex> = (0..5)
                .map(|k| {
                    let k = k as f32;
                    let (p, t) = if full { (7 + 2 * k as u8, [k as u8 * 3, 9, 12, 15, 18, 21, 24, 27]) } else { (pm, tm) };
                    RawVertex { pnmtx: p, texmtx: t, pos: [k, 2.0 * k - 3.0, 0.5 * k - 1.0], nbt: [[0.3, k, -1.0], [1.0, 0.2 * k, 0.0], [0.0, -1.0, k]], color: [[0.5, 0.25, k / 16.0, 1.0], [0.1, 0.2, 0.3, k / 16.0]], tex: [[k / 4.0, 1.0 - k / 4.0], [k, -k], [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2]] }
                })
                .collect();
            // Colours as packing keeps them (8-bit).
            let raws: Vec<RawVertex> = raws.into_iter().map(|mut v| { v.color = v.color.map(|c| c.map(|x| (x * 255.0).round() / 255.0)); v }).collect();
            let screen = Screen { offset_x: 342.0, offset_y: 340.0 };
            let setup = xf.setup(screen, [true, true]);
            let mut blocks = Vec::new();
            xf.write_setup_block(&setup, &mut blocks);
            let mb = blocks.len() as u32;
            xf.write_matrix_block(screen, full, &mut blocks);
            let pack = crate::vertex::Pack::from_bits(u32::from(full) | 2 | 9 << 2 | 0x40 | 0x80 | 0x3 << 8 | (if full { 3 } else { 0 } + 3 + 9 + 2 + 4) << 16);
            let state = [0, mb, pack.bits(), if full { block::FULL } else { 0 }];
            let mut cpu = Vec::new();
            xf.transform_into(&raws, &setup, &mut cpu, false);
            for (v, want) in raws.iter().zip(&cpu) {
                let mut words = Vec::new();
                pack.write(v, &mut words);
                let got = transform_packed(&blocks, state, &words, 0);
                let bits = |v: &Vertex| -> Vec<u32> { v.pos.iter().chain(v.color.iter().flatten()).chain(v.tex.iter().flatten()).map(|f| f.to_bits()).collect() };
                assert_eq!(bits(&got), bits(want), "full {full}");
            }
            assert!(cpu.iter().any(|v| v.color[0] != v.color[1] && v.tex[2] != [0.0; 3]));
        }
    }

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
