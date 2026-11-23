//! GX command processor (docs/PORT_PLAN.md, section 2.5): decodes the FIFO
//! stream into CP/XF/BP state and turns draws into batches the renderer can
//! execute: transformed vertices, the packed TEV/PE state for the fragment
//! ubershader, raster state, textures, and EFB copies. No Vulkan types here.
//!
//! Vertex transform and lighting run on the CPU (`xf`); each draw also keeps
//! what frame interpolation needs to place its vertices again with
//! interpolated matrices (`interp`).

mod back;
pub mod dff;
pub mod backdrop;
pub mod interp;
pub mod replace;
pub mod texture;
pub mod thread;
pub mod vertex;
pub mod xf;

use guest_mem::Ram;
#[cfg(test)]
use guest_mem::Mem;
use back::{Batch, DrawOp, Op, Sink};
use texture::TexKey;

/// A texture map's registers, the cache epoch, and the lookup's (id,
/// sampler, size).
type TexMemo = ([u32; 7], u64, (u32, u32, u32));
use vertex::{Cp, Layout};
pub use xf::{EFB_HEIGHT, EFB_WIDTH};

/// One vertex for the renderer: Vulkan clip-space position, the two lit
/// colours (0..1), and eight texture coordinates (s, t, q).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vertex {
    pub pos: [f32; 4],
    pub color: [[f32; 4]; 2],
    pub tex: [[f32; 3]; 8],
}

/// Ops per batch the first half sends the transform thread.
const BACK_BATCH: usize = 1024;

/// Words in a [`Shading`] block; the WGSL side reads it as `array<vec4<u32>, 40>`.
pub const SHADING_WORDS: usize = 160;

/// Word offsets inside a [`Shading`] block (mirrored in `shader.wgsl`).
pub mod sh {
    /// BP 0x00 GENMODE.
    pub const GENMODE: usize = 0;
    /// BP 0x28..0x2F TREF (TEV order), one per two stages.
    pub const TREF: usize = 1;
    /// BP 0xC0..0xDF: colour then alpha combiner for each of the 16 stages.
    pub const ENV: usize = 9;
    /// BP 0xF6..0xFD KSEL: swap tables and konst selectors.
    pub const KSEL: usize = 41;
    /// TEV registers PREV, C0, C1, C2 as (R|A<<12, B|G<<12) 11-bit pairs.
    pub const TEV_COLOR: usize = 49;
    /// Konst colours K0..K3 in the same layout.
    pub const TEV_KONST: usize = 57;
    /// BP 0xF3 alpha compare.
    pub const ALPHA_TEST: usize = 65;
    /// BP 0x43 PE control (EFB pixel format).
    pub const PE_CONTROL: usize = 66;
    /// BP 0x42 constant (destination) alpha.
    pub const CONST_ALPHA: usize = 67;
    /// BP 0x41 blend mode.
    pub const BLEND: usize = 68;
    /// Texture id per texture map (`NO_TEXTURE` if unused); the renderer
    /// replaces it with a descriptor slot.
    pub const TEX: usize = 72;
    /// Sampler key per texture map; the renderer replaces it with a slot.
    pub const SAMPLER: usize = 80;
    /// BP 0x10..0x1F indirect command per TEV stage.
    pub const IND_CMD: usize = 88;
    /// BP 0x27 IREF.
    pub const IREF: usize = 104;
    /// BP 0x25, 0x26 indirect texcoord scales.
    pub const IND_SCALE: usize = 105;
    /// BP 0x06..0x0E indirect matrices.
    pub const IND_MTX: usize = 107;
    /// Texture size per map: width | height << 16.
    pub const TEX_SIZE: usize = 116;
    /// Texcoord scale per texcoord (BP 0x30..0x3F): s size | t size << 16.
    pub const SU_SIZE: usize = 124;
    /// BP 0xEE..0xF2: fog A, B magnitude, B shift, C/projection/function, colour.
    pub const FOG: usize = 132;
    /// BP 0xE8..0xED: fog range adjustment (centre, then the K table).
    pub const FOG_RANGE: usize = 137;
    /// Viewport half width in pixels (f32), for fog range adjustment.
    pub const FOG_WIDTH: usize = 143;
    /// BP 0xF4, 0xF5: Z texture bias and mode.
    pub const ZTEX: usize = 144;
    /// [`FLAG_CLEAR`], [`FLAG_ZFREEZE`].
    pub const FLAGS: usize = 146;
    /// Colour of a masked clear (R | G << 8 | B << 16 | A << 24).
    pub const CLEAR_COLOR: usize = 147;
    /// Z-freeze plane (f32): z24 = a * x + b * y + c in EFB pixels.
    pub const ZPLANE: usize = 148;
    /// EFB scale across (f32), filled in by the renderer.
    pub const SCALE: usize = 151;
    /// EFB scale down (f32), filled in by the renderer.
    pub const SCALE_Y: usize = 152;
}

/// The draw is a masked EFB clear: output `CLEAR_COLOR` as is.
pub const FLAG_CLEAR: u32 = 1;
/// Depth comes from the z-freeze plane.
pub const FLAG_ZFREEZE: u32 = 2;

pub const NO_TEXTURE: u32 = u32::MAX;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shading(pub [u32; SHADING_WORDS]);

/// Fixed-function state the renderer turns into pipeline and dynamic state.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Raster {
    /// BP 0x41 bits 0..15: blend, logic op, colour/alpha update, factors.
    pub blend: u32,
    /// BP 0x40: depth test enable, function, update.
    pub zmode: u32,
    /// GENMODE cull mode: 0 none, 1 back, 2 front, 3 all.
    pub cull: u32,
    /// The EFB keeps alpha (pixel format RGBA6_Z24).
    pub efb_alpha: bool,
}

/// Inclusive-exclusive EFB rectangle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Draw {
    /// Index range in [`Output::indices`] (triangle list).
    pub first: u32,
    pub count: u32,
    pub shading: u32,
    pub raster: Raster,
    pub scissor: Rect,
}

/// EFB clear (after a copy, or a CPU poke of one pixel).
#[derive(Clone, Copy, Debug)]
pub struct Clear {
    pub rect: Rect,
    pub color: [u8; 4],
    /// 24-bit depth.
    pub z: u32,
    pub color_update: bool,
    pub alpha_update: bool,
    pub z_update: bool,
}

/// An EFB-to-texture copy.
#[derive(Clone, Copy, Debug)]
pub struct CopyTex {
    pub src: Rect,
    /// Texture id the renderer keeps the copy as.
    pub id: u32,
    /// Size after the optional 2x2 box filter.
    pub w: u32,
    pub h: u32,
    pub half: bool,
    /// Copy format (0 R4 .. 12 GB8, see `texture::copy_texture_format`).
    pub format: u32,
    /// Convert colour to intensity (Y) for the I/IA formats.
    pub intensity: bool,
    /// The source is the depth buffer (pixel format Z24).
    pub depth: bool,
    pub efb_alpha: bool,
    /// Where the texels go in RAM, and in which texture format.
    pub addr: u32,
    pub tex_format: u32,
    /// [`texture::hash`] of those RAM bytes when the copy was made. The
    /// texels reach RAM a frame or two later; if the game has written there
    /// in the meantime (the buffer was reused), they must not be written.
    pub ram_hash: u64,
    /// Bytes from one row of blocks to the next in RAM when wider than the
    /// copy's own (BP 0x4D), else 0: the copy patches part of a wider
    /// texture ([`texture::copy_rows`]).
    pub stride: u32,
}

/// An EFB copy (texture `copy`) drawn into part of texture `target` at
/// (`x`, `y`), `w` x `h` texels: NSMBW animates coins, ? blocks and other
/// tiles by copying each frame into its tile atlas.
#[derive(Clone, Copy, Debug)]
pub struct Patch {
    pub copy: u32,
    pub target: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Cmd {
    Draw(Draw),
    /// EFB rect to the XFB at `addr`, scaled to `height` lines.
    CopyXfb { src: Rect, addr: u32, height: u32 },
    CopyTex(CopyTex),
    /// Follows the [`Cmd::CopyTex`] it draws from.
    Patch(Patch),
    Clear(Clear),
}

/// Everything about one draw that frame interpolation needs to place its
/// vertices again with interpolated matrices.
#[derive(Clone, Debug)]
pub struct DrawRecord {
    /// Vertex range in [`Output::vertices`].
    pub first: u32,
    pub count: u32,
    /// Stable across frames for the same mesh and material: primitive,
    /// vertex count, combiner state, raster state and the matrix slots its
    /// vertices use. Textures and TEV colours are left out (they animate).
    pub identity: u64,
    pub projection: [f32; 6],
    pub ortho: bool,
    pub viewport: [f32; 6],
    pub screen: xf::Screen,
    /// The position matrices the vertices use, by slot: this range of
    /// [`Output::matrices`] ([`Output::record_matrices`]). One list per field
    /// instead of one per record: the world map has over 10,000 records.
    pub matrices: std::ops::Range<u32>,
    /// Lines and points: expanded in screen space, not re-placeable.
    pub fixed: bool,
}

/// The game camera at the end of the game's camera update, for the draws
/// after it ([`Gpu::set_world`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldCamera {
    /// Width and height (world units): the world projection is
    /// `2 / width`, `2 / height`.
    pub size: [f32; 2],
    /// Its centre in the world (y up).
    pub centre: [f32; 2],
    /// It is the game's own camera (no box around far-apart players), so
    /// background layers are drawn at their normal size and place
    /// ([`backdrop::Backdrop`] learns them from such fields).
    pub normal: bool,
    /// The game camera's size without a box: what players' windows show.
    pub normal_size: [f32; 2],
}

/// View-space depth behind which a world draw is a background layer. In
/// 1-1 the course (tiles, actors) is drawn from about -5500 to -10000 and
/// the background (sun rays at -10050, layers from -13500 to -18500)
/// behind it.
pub const LAYER_Z: f32 = -10020.0;

/// A world draw of a background layer: its vertices and its position
/// matrix's scale and translation (view space, world units).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Layer {
    pub first: u32,
    pub count: u32,
    /// The matrix's z translation, rounded: the same for every frame of a
    /// layer, so it names it.
    pub z: i32,
    pub scale: f32,
    pub t: [f32; 2],
}

/// The world draws of a field, so a player's window can be applied to them
/// after the transform ([`World::apply`]).
#[derive(Clone, Debug, Default)]
pub struct World {
    /// Ranges of [`Output::vertices`] (first, count) drawn through the game
    /// camera's projection.
    pub ranges: Vec<(u32, u32)>,
    /// Those of them that are background layers, in draw order.
    pub layers: Vec<Layer>,
    pub camera: WorldCamera,
    /// How projected coordinates map to clip space (x scale, x offset,
    /// y scale, y offset, per unit of w), from their viewport.
    pub map: [f32; 4],
}

impl World {
    /// The clip-space affine (per axis: X' = a X + b W) that moves a
    /// world draw from the game camera to `view`.
    pub(crate) fn world_affine(&self, view: &xf::View) -> [(f32, f32); 2] {
        std::array::from_fn(|i| {
            let k = self.camera.size[i] / view.size[i];
            let d = 2.0 * view.offset[i] / view.size[i];
            let (s, o) = (self.map[2 * i], self.map[2 * i + 1]);
            (k, o * (1.0 - k) - s * d)
        })
    }

    /// Moves the world vertices in `vertices` (laid out as the field's
    /// output) from the game camera to `view`. Background layers that
    /// `backdrop` knows are drawn at their normal size and shifted by their
    /// parallax instead ([`backdrop::Backdrop::layer_affine`]); ones it does
    /// not know yet move like the rest of the world.
    pub fn apply(&self, vertices: &mut [Vertex], view: &xf::View, backdrop: Option<&backdrop::Backdrop>) {
        if self.ranges.is_empty() || view.size[0] <= 0.0 || view.size[1] <= 0.0 {
            return;
        }
        let world = self.world_affine(view);
        let mut layer = self.layers.iter().peekable();
        for &(first, count) in &self.ranges {
            let (mut at, end) = (first, first + count);
            while at < end {
                // Skip layers already behind us, then take the next one in this range.
                while layer.peek().is_some_and(|l| l.first + l.count <= at) {
                    layer.next();
                }
                let (upto, affine) = match layer.peek() {
                    Some(l) if l.first <= at => {
                        let a = match backdrop {
                            Some(b) => b.layer_affine(self, l, view),
                            None => None,
                        };
                        ((l.first + l.count).min(end), a.unwrap_or(world))
                    }
                    Some(l) if l.first < end => (l.first, world),
                    _ => (end, world),
                };
                if let Some(vs) = vertices.get_mut(at as usize..upto as usize) {
                    for v in vs {
                        let w = v.pos[3];
                        v.pos[0] = affine[0].0 * v.pos[0] + affine[0].1 * w;
                        v.pos[1] = affine[1].0 * v.pos[1] + affine[1].1 * w;
                    }
                }
                at = upto.max(at + 1);
            }
        }
    }
}

#[derive(Default)]
pub struct Output {
    /// Each draw's vertices once, in order; [`Output::indices`] makes them
    /// triangle lists.
    pub vertices: Vec<Vertex>,
    /// Triangle-list indices into `vertices`. `model` and `records` (for frame
    /// interpolation) are left empty when the frontend does not interpolate.
    pub indices: Vec<u32>,
    /// Per vertex: model-space position and position-matrix slot.
    pub model: Vec<([f32; 3], u8)>,
    pub records: Vec<DrawRecord>,
    /// The records' position matrices, by slot (see [`DrawRecord::matrices`]).
    pub matrices: Vec<(u8, [[f32; 4]; 3])>,
    pub shadings: Vec<Shading>,
    pub cmds: Vec<Cmd>,
    pub new_textures: Vec<texture::NewTexture>,
    pub evicted: Vec<u32>,
    /// The world draws, while a game camera is set ([`Gpu::set_world`]).
    pub world: World,
    /// Each player's window, as last set ([`Gpu::set_views`]).
    pub views: [Option<xf::View>; 4],
}

impl Output {
    /// Record `r`'s position matrices, by slot.
    pub fn record_matrices(&self, r: &DrawRecord) -> &[(u8, [[f32; 4]; 3])] {
        &self.matrices[r.matrices.start as usize..r.matrices.end as usize]
    }

    /// A hash of everything the renderer gets from this field (vertices,
    /// indices, shading blocks, commands, new textures, evictions), to check
    /// that a change to decoding leaves its output alone (`--hash-gx`).
    /// Interpolation data (`model`, `records`) is left out.
    pub fn digest(&self) -> u64 {
        let mut h = 0xCBF2_9CE4_8422_2325u64;
        for v in &self.vertices {
            let w: Vec<u32> = v.pos.iter().chain(v.color.iter().flatten()).chain(v.tex.iter().flatten()).map(|f| f.to_bits()).collect();
            hash_words(&mut h, &w);
        }
        hash_words(&mut h, &self.indices);
        for s in &self.shadings {
            hash_words(&mut h, &s.0);
        }
        for c in &self.cmds {
            hash_words(&mut h, &format!("{c:?}").bytes().map(u32::from).collect::<Vec<_>>());
        }
        for t in &self.new_textures {
            hash_words(&mut h, &[t.id, t.width, t.height, t.levels.len() as u32]);
            if let Some(r) = &t.replacement {
                hash_words(&mut h, &[r.width, r.height, r.levels.len() as u32]);
            }
            // Its own hash, so a change to `texture::hash` does not move it.
            for l in &t.levels {
                for c in l.chunks(8) {
                    let mut b = [0u8; 8];
                    b[..c.len()].copy_from_slice(c);
                    h = (h.rotate_left(5) ^ u64::from_le_bytes(b)).wrapping_mul(0x517C_C1B7_2722_0A95);
                }
            }
        }
        hash_words(&mut h, &self.evicted);
        h
    }
}

/// What the stream asked the PE to signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// BP 0x45: draw done.
    Finish,
    /// BP 0x47: token with interrupt.
    TokenInt(u16),
    /// BP 0x48: token without interrupt.
    Token(u16),
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub draws: u64,
    pub vertices: u64,
    pub bad_opcodes: u32,
    pub last_bad: u8,
    pub display_lists: u64,
    pub xfb_copies: u64,
    pub tex_copies: u64,
    pub lines: u64,
    pub points: u64,
    pub tmem_preloads: u64,
    pub pokes: u64,
}

pub struct Gpu {
    pending: Vec<u8>,
    bp: [u32; 256],
    bp_mask: u32,
    cp: Cp,
    /// Per VAT, its vertex layout, valid until the next CP load.
    layouts: [Option<Layout>; 8],
    xf: xf::Xf,
    tev_color: [[u32; 2]; 4],
    tev_konst: [[u32; 2]; 4],
    tmem: Vec<u8>,
    /// Per texture map: the texture registers and cache epoch of the last
    /// lookup, and its result (id, sampler, size).
    tex_memo: [Option<TexMemo>; 8],
    /// Whether fields from the next one keep what frame interpolation needs
    /// ([`Output::model`], [`Output::records`]).
    pub records_next: bool,
    /// EFB copies into part of a texture patch it ([`Cmd::Patch`]); off,
    /// the animated tiles stand still.
    pub patches: bool,
    /// Draws are dropped (switched in the command stream: Depth Blur off).
    drop_draws: bool,
    textures: texture::Cache,
    /// Loaded draws and other steps for the transform half, and where they
    /// go (see `back`).
    batch: Batch,
    sink: Sink,
    pub stats: Stats,
}

impl Default for Gpu {
    fn default() -> Self {
        Self {
            pending: Vec::new(),
            bp: [0; 256],
            bp_mask: 0xFF_FFFF,
            cp: Cp::default(),
            layouts: [None; 8],
            xf: xf::Xf::default(),
            tev_color: [[0; 2]; 4],
            tev_konst: [[0; 2]; 4],
            tmem: vec![0; 1 << 20],
            tex_memo: [None; 8],
            records_next: true,
            patches: true,
            drop_draws: false,
            textures: texture::Cache::default(),
            batch: Batch::default(),
            sink: Sink::Local(Box::default()),
            stats: Stats::default(),
        }
    }
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes(b[..4].try_into().unwrap())
}

/// 64-bit FNV-1a over 32-bit words.
pub(crate) fn hash_words(h: &mut u64, words: &[u32]) {
    for w in words {
        *h = (*h ^ u64::from(*w)).wrapping_mul(0x0000_0100_0000_01B3);
    }
}

/// Clip-space position to EFB pixels and 24-bit depth.
pub(crate) fn screen_point(v: &Vertex) -> Option<[f32; 3]> {
    let w = v.pos[3];
    if w.abs() < 1e-12 {
        return None;
    }
    Some([(v.pos[0] / w + 1.0) * EFB_WIDTH / 2.0, (v.pos[1] / w + 1.0) * EFB_HEIGHT / 2.0, v.pos[2] / w * 16_777_216.0])
}

/// The plane z = a * x + b * y + c through three screen points.
pub(crate) fn depth_plane(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> Option<[f32; 3]> {
    let (ux, uy, uz) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let (vx, vy, vz) = (c[0] - a[0], c[1] - a[1], c[2] - a[2]);
    let det = ux * vy - uy * vx;
    if det.abs() < 1e-6 {
        return None;
    }
    let dzdx = (uz * vy - uy * vz) / det;
    let dzdy = (ux * vz - uz * vx) / det;
    Some([dzdx, dzdy, a[2] - dzdx * a[0] - dzdy * a[1]])
}

impl Gpu {
    /// Everything decoded since the last call; also starts a new texture
    /// generation, so textures are re-hashed on next use.
    pub fn take_output(&mut self) -> Output {
        self.flush_back();
        let mut out = self.sink.take_output(self.records_next);
        out.new_textures = std::mem::take(&mut self.textures.new);
        out.evicted = std::mem::take(&mut self.textures.evicted);
        self.textures.next_generation();
        out
    }

    /// Hands back an [`Output`] the renderer is done with: its buffers are
    /// reused (with their capacity) for a later field.
    pub fn recycle(&mut self, old: Output) {
        self.sink.recycle(old);
    }

    /// Hands what was decoded so far to the transform half.
    pub fn flush_back(&mut self) {
        if !self.batch.is_empty() {
            let spare = self.sink.spare();
            let batch = std::mem::replace(&mut self.batch, spare);
            self.sink.send(batch);
        }
    }

    /// Runs the transform half on its own thread from now on; the thread
    /// runs `on_start` first.
    pub fn start_transform_thread(&mut self, on_start: fn()) {
        self.flush_back();
        self.sink.start_thread(on_start);
    }

    /// Consumes bytes the GP reads from its FIFO; PE events go to `events`.
    pub fn feed(&mut self, bytes: &[u8], mem: &dyn Ram, events: &mut Vec<Event>) {
        // Without allocating or copying `bytes` (as `Scanner::feed`).
        let mut buf = std::mem::take(&mut self.pending);
        if buf.is_empty() {
            let at = self.run(bytes, mem, events, false);
            buf.extend_from_slice(&bytes[at.min(bytes.len())..]);
        } else {
            buf.extend_from_slice(bytes);
            let at = self.run(&buf, mem, events, false);
            buf.drain(..at.min(buf.len()));
        }
        self.pending = buf;
        // Inline, the transform half runs at once; on its thread, in batches.
        if matches!(self.sink, Sink::Local(_)) || self.batch.ops.len() >= BACK_BATCH {
            self.flush_back();
        }
    }

    /// Saves or loads the registers (BP, CP, XF, TEV colours, TMEM) and any
    /// incomplete command. Loading also resends XF to the transform half and
    /// forgets texture lookups, since RAM changed under them. Textures and
    /// EFB copies are host-side and come back as the game draws.
    pub fn snap_regs(&mut self, s: &mut guest_mem::snap::Snap) {
        s.tag(b"GXR ");
        guest_mem::snap!(s; self.pending, self.bp, self.bp_mask, self.cp, self.xf.mem, self.xf.regs, self.tev_color, self.tev_konst, self.tmem);
        if s.loading() {
            self.layouts = [None; 8];
            self.xf.setup_gen += 1;
            self.tex_memo = [None; 8];
            self.textures.epoch += 1;
            for i in 0..self.xf.mem.len() {
                self.batch.ops.push(Op::Xf { addr: i as u32, v: self.xf.mem[i] });
            }
            for i in 0..self.xf.regs.len() {
                self.batch.ops.push(Op::Xf { addr: 0x1000 + i as u32, v: self.xf.regs[i] });
            }
            self.flush_back();
        }
    }

    /// The game camera for the draws decoded from now on: draws through its
    /// projection are recorded in [`Output::world`]. `None` outside a
    /// course.
    pub fn set_world(&mut self, camera: Option<WorldCamera>) {
        self.batch.ops.push(Op::World(camera));
    }

    /// Each player's window ([`xf::View`]), kept in the output from here on.
    pub fn set_views(&mut self, views: [Option<xf::View>; 4]) {
        self.batch.ops.push(Op::Views(views));
    }

    /// The draws decoded from now on are dropped (`on`) or drawn.
    pub fn set_drop(&mut self, on: bool) {
        self.drop_draws = on;
    }

    /// The draws decoded from now on widen 16:9 projections to `aspect`
    /// (0: none; see [`xf::Xf::projection`]).
    pub fn set_wide(&mut self, aspect: f32) {
        self.batch.ops.push(Op::Wide(aspect));
    }

    /// An XF write, for both halves.
    fn xf_write(&mut self, addr: u32, v: u32) {
        self.xf.write(addr, v);
        self.batch.ops.push(Op::Xf { addr, v });
    }

    /// A CPU write to the EFB window: one pixel of colour (0xAARRGGBB) or
    /// 24-bit depth, in order with the draws decoded so far.
    pub fn poke(&mut self, x: u32, y: u32, value: u32, z: bool) {
        self.stats.pokes += 1;
        let [a, r, g, b] = value.to_be_bytes();
        self.batch.ops.push(Op::Cmd(Cmd::Clear(Clear {
            rect: Rect { x: x.min(EFB_WIDTH as u32 - 1), y: y.min(EFB_HEIGHT as u32 - 1), w: 1, h: 1 },
            color: [r, g, b, a],
            z: value & 0xFF_FFFF,
            color_update: !z,
            alpha_update: !z,
            z_update: z,
        })));
        if matches!(self.sink, Sink::Local(_)) {
            self.flush_back();
        }
    }

    /// Replaces TMEM (FIFO-log replay starts from a TMEM snapshot).
    pub fn load_tmem(&mut self, data: &[u8]) {
        let n = data.len().min(self.tmem.len());
        self.tmem[..n].copy_from_slice(&data[..n]);
        self.textures.epoch += 1;
    }

    /// The renderer wrote an EFB copy back to RAM at `addr` (see
    /// `texture::Cache::note_writeback`).
    pub fn note_writeback(&mut self, addr: u32, ram: &[u8]) {
        self.textures.note_writeback(addr, ram);
    }

    /// Draws textures from `pack` where it has them from now on (`None`:
    /// the game's own); see [`replace`].
    pub fn set_texture_pack(&mut self, pack: Option<replace::Pack>) {
        self.textures.set_pack(pack);
        self.tex_memo = [None; 8];
    }

    /// Runs whole commands in `buf`; returns the bytes consumed. Display
    /// lists (`in_list`) cannot call further lists.
    fn run(&mut self, buf: &[u8], mem: &dyn Ram, events: &mut Vec<Event>, in_list: bool) -> usize {
        let mut at = 0;
        while at < buf.len() {
            let rest = &buf[at..];
            let op = rest[0];
            let need = match op {
                0x00 | 0x44 | 0x48 => 1,
                0x08 => 6,
                0x10 => {
                    if rest.len() < 5 {
                        break;
                    }
                    5 + 4 * (((be32(&rest[1..]) >> 16) & 0xF) as usize + 1)
                }
                0x20 | 0x28 | 0x30 | 0x38 => 5,
                0x40 => 9,
                0x61 => 5,
                0x80..=0xBF => {
                    if rest.len() < 3 {
                        break;
                    }
                    let n = u16::from_be_bytes([rest[1], rest[2]]) as usize;
                    3 + n * self.layout((op & 7) as usize).size
                }
                _ => {
                    self.stats.bad_opcodes += 1;
                    self.stats.last_bad = op;
                    return buf.len();
                }
            };
            if rest.len() < need {
                break;
            }
            match op {
                0x08 => {
                    self.cp.load(rest[1], be32(&rest[2..]));
                    self.layouts = [None; 8];
                }
                0x10 => {
                    let h = be32(&rest[1..]);
                    let addr = h & 0xFFFF;
                    for i in 0..((h >> 16) & 0xF) + 1 {
                        self.xf_write(addr + i, be32(&rest[5 + 4 * i as usize..]));
                    }
                }
                0x20 | 0x28 | 0x30 | 0x38 => {
                    let v = be32(&rest[1..]);
                    let array = 12 + ((op >> 3) & 3) as usize;
                    let (index, size, addr) = (v >> 16, ((v >> 12) & 0xF) + 1, v & 0xFFF);
                    let src = self.cp.array_base[array].wrapping_add(index * self.cp.array_stride[array]);
                    for i in 0..size {
                        if let Some(w) = mem.ram(src + 4 * i, 4) {
                            self.xf_write(addr + i, be32(w));
                        }
                    }
                }
                0x40 if !in_list => {
                    let (addr, size) = (be32(&rest[1..]), be32(&rest[5..]) as usize);
                    if let Some(list) = mem.ram(addr, size) {
                        let list = list.to_vec();
                        self.stats.display_lists += 1;
                        self.run(&list, mem, events, true);
                    }
                }
                0x61 => self.bp_write(be32(&rest[1..]), mem, events),
                0x80..=0xBF => {
                    let n = u16::from_be_bytes([rest[1], rest[2]]) as usize;
                    self.draw(op, n, &rest[3..need], mem);
                }
                _ => {}
            }
            at += need;
        }
        at
    }

    fn bp_write(&mut self, v: u32, mem: &dyn Ram, events: &mut Vec<Event>) {
        let reg = (v >> 24) as usize;
        let mut val = v & 0xFF_FFFF;
        if reg != 0xFE {
            val = (self.bp[reg] & !self.bp_mask) | (val & self.bp_mask);
            self.bp_mask = 0xFF_FFFF;
        }
        self.bp[reg] = val;
        match reg {
            0x45 => events.push(Event::Finish),
            0x47 => events.push(Event::TokenInt(val as u16)),
            0x48 => events.push(Event::Token(val as u16)),
            0x52 => self.efb_copy(val, mem),
            0x63 if val != 0 => self.tmem_preload(val, mem),
            0x65 => {
                // Load a palette from RAM into TMEM.
                let src = (self.bp[0x64] & 0x1F_FFFF) << 5;
                let dst = ((val & 0x3FF) << 9) as usize;
                let len = (((val >> 10) & 0x7FF) * 32) as usize;
                if let Some(b) = mem.ram(src, len) {
                    let end = (dst + len).min(self.tmem.len());
                    self.tmem[dst..end].copy_from_slice(&b[..end - dst]);
                }
                self.textures.epoch += 1;
            }
            0xE0..=0xE7 => {
                let i = (reg - 0xE0) / 2;
                let half = (reg - 0xE0) % 2;
                if (val >> 23) & 1 != 0 {
                    self.tev_konst[i][half] = val;
                } else {
                    self.tev_color[i][half] = val;
                }
            }
            0xFE => self.bp_mask = val,
            _ => {}
        }
    }

    /// BP 0x63: copy 32-byte lines from RAM (BP 0x60) into TMEM at the even
    /// bank address (BP 0x61); RGBA8 tiles alternate between the even and
    /// odd (BP 0x62) banks.
    fn tmem_preload(&mut self, mode: u32, mem: &dyn Ram) {
        self.stats.tmem_preloads += 1;
        self.textures.epoch += 1;
        let src = (self.bp[0x60] & 0xFF_FFFF) << 5;
        let count = (mode & 0x7FFF) as usize;
        let mut even = (self.bp[0x61] & 0x7FFF) as usize * 32;
        let mut odd = (self.bp[0x62] & 0x7FFF) as usize * 32;
        let tmem = self.tmem.len();
        if (mode >> 15) & 3 != 3 {
            let len = (count * 32).min(tmem.saturating_sub(even));
            if let Some(b) = mem.ram(src, len) {
                self.tmem[even..even + len].copy_from_slice(b);
            }
        } else {
            for i in 0..count as u32 {
                if even + 32 > tmem || odd + 32 > tmem {
                    break;
                }
                if let Some(b) = mem.ram(src + 64 * i, 64) {
                    self.tmem[even..even + 32].copy_from_slice(&b[..32]);
                    self.tmem[odd..odd + 32].copy_from_slice(&b[32..]);
                }
                even += 32;
                odd += 32;
            }
        }
    }

    fn rect(tl: u32, wh: u32) -> Rect {
        Rect { x: tl & 0x3FF, y: (tl >> 10) & 0x3FF, w: (wh & 0x3FF) + 1, h: ((wh >> 10) & 0x3FF) + 1 }
    }

    fn efb_alpha(&self) -> bool {
        self.bp[0x43] & 7 == 1
    }

    fn efb_copy(&mut self, ctl: u32, mem: &dyn Ram) {
        let src = Self::rect(self.bp[0x49], self.bp[0x4A]);
        let addr = (self.bp[0x4B] & 0xFF_FFFF) << 5;
        if ctl & (1 << 14) != 0 {
            // Display copy: the vertical scale is 256 / reg when enabled.
            let scale = if ctl & (1 << 10) != 0 { 256.0 / (self.bp[0x4E] & 0x1FF).max(1) as f32 } else { 1.0 };
            let height = ((src.h as f32 * scale) as u32).clamp(1, 1024);
            self.stats.xfb_copies += 1;
            self.batch.ops.push(Op::Cmd(Cmd::CopyXfb { src, addr, height }));
        } else {
            let half = ctl & (1 << 9) != 0;
            let (w, h) = if half { ((src.w / 2).max(1), (src.h / 2).max(1)) } else { (src.w, src.h) };
            // Bits 3..6 hold the format with its low bit moved to the top.
            let t = (ctl >> 3) & 0xF;
            let format = (t >> 1) | ((t & 1) << 3);
            let tex_format = texture::copy_texture_format(format);
            // A stride wider than the copy's rows: a patch of a wider texture.
            let (bw, _, block_bytes) = texture::block(tex_format);
            let stride = (self.bp[0x4D] & 0x3FF) << 5;
            let stride = if stride as usize > (w as usize).div_ceil(bw) * block_bytes { stride } else { 0 };
            let ram_hash = if stride == 0 {
                let size = texture::level_size(tex_format, w as usize, h as usize);
                texture::hash(mem.ram(addr, size).unwrap_or(&[]))
            } else {
                texture::hash(&texture::copy_bytes(mem, addr, tex_format, w as usize, h as usize, stride as usize).unwrap_or_default())
            };
            let id = self.textures.add_efb_copy(addr, w as u16, h as u16, ram_hash);
            self.stats.tex_copies += 1;
            self.batch.ops.push(Op::Cmd(Cmd::CopyTex(CopyTex {
                src,
                id,
                w,
                h,
                half,
                format,
                intensity: ctl & (1 << 15) != 0,
                depth: self.bp[0x43] & 7 == 3,
                efb_alpha: self.efb_alpha(),
                addr,
                tex_format,
                ram_hash,
                stride,
            })));
            if stride != 0 && self.patches {
                for (target, x, y) in self.textures.patch_targets(addr, tex_format, w, h, stride) {
                    self.batch.ops.push(Op::Cmd(Cmd::Patch(Patch { copy: id, target, x, y, w, h })));
                }
            }
        }
        if ctl & (1 << 11) != 0 {
            self.clear(src);
        }
    }

    /// The clear after an EFB copy. A clear of colour without alpha (or the
    /// reverse) in an EFB with alpha becomes a masked full-rect draw.
    fn clear(&mut self, rect: Rect) {
        let (ar, gb) = (self.bp[0x4F], self.bp[0x50]);
        let blend = self.bp[0x41];
        let color = [ar as u8, (gb >> 8) as u8, gb as u8, (ar >> 8) as u8];
        let (color_update, mut alpha_update) = (blend & 8 != 0, blend & 0x10 != 0);
        let z = self.bp[0x51] & 0xFF_FFFF;
        let z_update = self.bp[0x40] & 0x10 != 0;
        if !self.efb_alpha() {
            alpha_update = color_update;
        }
        if color_update == alpha_update {
            self.batch.ops.push(Op::Cmd(Cmd::Clear(Clear { rect, color, z, color_update, alpha_update, z_update })));
            return;
        }
        if z_update {
            self.batch.ops.push(Op::Cmd(Cmd::Clear(Clear { rect, color, z, color_update: false, alpha_update: false, z_update })));
        }
        self.batch.ops.push(Op::ClearQuad { rect, color, z, color_update });
    }

    fn scissor(&self) -> Rect {
        let (tl, br) = (self.bp[0x20], self.bp[0x21]);
        let (ox, oy) = self.scissor_offset();
        let x0 = ((tl >> 12) & 0x7FF) as i32 - ox as i32;
        let y0 = (tl & 0x7FF) as i32 - oy as i32;
        let x1 = ((br >> 12) & 0x7FF) as i32 - ox as i32 + 1;
        let y1 = (br & 0x7FF) as i32 - oy as i32 + 1;
        let cx = |v: i32, m: f32| v.clamp(0, m as i32) as u32;
        let (x0, y0, x1, y1) = (cx(x0, EFB_WIDTH), cx(y0, EFB_HEIGHT), cx(x1, EFB_WIDTH), cx(y1, EFB_HEIGHT));
        Rect { x: x0, y: y0, w: x1.saturating_sub(x0), h: y1.saturating_sub(y0) }
    }

    /// BP 0x59, in pixels (the register holds half the offset, biased by 342).
    fn scissor_offset(&self) -> (f32, f32) {
        let o = self.bp[0x59];
        (((o & 0x3FF) * 2) as f32, (((o >> 10) & 0x3FF) * 2) as f32)
    }

    /// Texture id for texture map `map`, decoding it if needed.
    fn texture(&mut self, map: usize, mem: &dyn Ram) -> (u32, u32, u32) {
        let r = |base: usize| self.bp[base + (map & 3) + if map >= 4 { 0x20 } else { 0 }];
        let (mode0, mode1, image0, image1, image2, image3, tlut) = (r(0x80), r(0x84), r(0x88), r(0x8C), r(0x90), r(0x94), r(0x98));
        let regs = [mode0, mode1, image0, image1, image2, image3, tlut];
        if let Some((m, epoch, result)) = self.tex_memo[map] {
            if m == regs && epoch == self.textures.epoch {
                return result;
            }
        }
        let w = (image0 & 0x3FF) + 1;
        let h = ((image0 >> 10) & 0x3FF) + 1;
        let format = (image0 >> 20) & 0xF;
        let mipmapped = (mode0 >> 5) & 3 != 0;
        let max_levels = 32 - w.max(h).leading_zeros();
        let levels = if mipmapped { (((mode1 >> 8) & 0xFF) / 16 + 1).min(max_levels) } else { 1 };
        let mut size = 0;
        let (mut lw, mut lh) = (w as usize, h as usize);
        for _ in 0..levels {
            size += texture::level_size(format, lw, lh);
            lw = (lw / 2).max(1);
            lh = (lh / 2).max(1);
        }
        // Preloaded textures (image1 bit 21) are read from TMEM, not RAM;
        // their key carries the TMEM address with the top bit set.
        let preloaded = image1 & (1 << 21) != 0;
        let (addr, tmem_data) = if preloaded {
            let even = (image1 & 0x7FFF) as usize * 32;
            let odd = (image2 & 0x7FFF) as usize * 32;
            let get = |at: usize, n: usize| &self.tmem[at.min(self.tmem.len())..(at + n).min(self.tmem.len())];
            let data = if format == texture::RGBA8 {
                let mut d = Vec::with_capacity(size);
                for b in 0..size / 64 {
                    d.extend_from_slice(get(even + 32 * b, 32));
                    d.extend_from_slice(get(odd + 32 * b, 32));
                }
                d
            } else {
                get(even, size).to_vec()
            };
            (0x8000_0000 | even as u32, Some(data))
        } else {
            ((image3 & 0xFF_FFFF) << 5, None)
        };
        let key = TexKey {
            addr,
            width: w as u16,
            height: h as u16,
            format,
            levels: levels as u8,
            tlut: if texture::is_paletted(format) { tlut & 0x3FF } else { 0 },
            tlut_format: if texture::is_paletted(format) { (tlut >> 10) & 3 } else { 0 },
        };
        let data = match &tmem_data {
            Some(d) => d.as_slice(),
            None => mem.ram(addr, size).unwrap_or(&[]),
        };
        let tlut_bytes: &[u8] = if texture::is_paletted(format) {
            let start = ((tlut & 0x3FF) << 9) as usize;
            let len = texture::palette_entries(format) * 2;
            &self.tmem[start.min(self.tmem.len())..(start + len).min(self.tmem.len())]
        } else {
            &[]
        };
        let id = self.textures.lookup(key, data, tlut_bytes);
        // Sampler key: wrap s/t, mag, min filter, LOD bias, max LOD (if
        // mipmapped), and bit 24 for a texture pack's image, whose own mip
        // levels are all used.
        let sampler = (mode0 & 0x1_FFFF) | if mipmapped { (levels - 1) << 20 } else { 0 } | if self.textures.is_replaced(id) { 1 << 24 } else { 0 };
        let result = (id, sampler, w | h << 16);
        self.tex_memo[map] = Some((regs, self.textures.epoch, result));
        result
    }

    fn shading(&mut self, mem: &dyn Ram) -> Shading {
        let mut s = [0u32; SHADING_WORDS];
        let bp = &self.bp;
        s[sh::GENMODE] = bp[0x00];
        s[sh::TREF..sh::TREF + 8].copy_from_slice(&bp[0x28..0x30]);
        s[sh::ENV..sh::ENV + 32].copy_from_slice(&bp[0xC0..0xE0]);
        s[sh::KSEL..sh::KSEL + 8].copy_from_slice(&bp[0xF6..0xFE]);
        for i in 0..4 {
            s[sh::TEV_COLOR + 2 * i] = self.tev_color[i][0];
            s[sh::TEV_COLOR + 2 * i + 1] = self.tev_color[i][1];
            s[sh::TEV_KONST + 2 * i] = self.tev_konst[i][0];
            s[sh::TEV_KONST + 2 * i + 1] = self.tev_konst[i][1];
        }
        s[sh::ALPHA_TEST] = bp[0xF3];
        s[sh::PE_CONTROL] = bp[0x43];
        s[sh::CONST_ALPHA] = bp[0x42];
        s[sh::BLEND] = bp[0x41];
        s[sh::IND_CMD..sh::IND_CMD + 16].copy_from_slice(&bp[0x10..0x20]);
        s[sh::IREF] = bp[0x27];
        s[sh::IND_SCALE] = bp[0x25];
        s[sh::IND_SCALE + 1] = bp[0x26];
        s[sh::IND_MTX..sh::IND_MTX + 9].copy_from_slice(&bp[0x06..0x0F]);
        for i in 0..8 {
            s[sh::SU_SIZE + i] = ((bp[0x30 + 2 * i] & 0xFFFF) + 1) | (((bp[0x31 + 2 * i] & 0xFFFF) + 1) << 16);
        }
        s[sh::FOG..sh::FOG + 5].copy_from_slice(&bp[0xEE..0xF3]);
        s[sh::FOG_RANGE..sh::FOG_RANGE + 6].copy_from_slice(&bp[0xE8..0xEE]);
        s[sh::FOG_WIDTH] = self.xf.viewport()[0].abs().to_bits();
        s[sh::ZTEX] = bp[0xF4];
        s[sh::ZTEX + 1] = bp[0xF5];
        if (bp[0x00] >> 19) & 1 != 0 {
            // The plane itself comes from the transform half.
            s[sh::FLAGS] |= FLAG_ZFREEZE;
        }

        // Texture maps used by enabled TEV stages and indirect stages.
        let stages = ((bp[0x00] >> 10) & 0xF) as usize + 1;
        let ind_stages = ((bp[0x00] >> 16) & 7) as usize;
        let mut used = 0u8;
        for st in 0..stages {
            let t = bp[0x28 + st / 2] >> (12 * (st & 1));
            if (t >> 6) & 1 != 0 {
                used |= 1 << (t & 7);
            }
        }
        for i in 0..ind_stages {
            used |= 1 << ((bp[0x27] >> (6 * i)) & 7);
        }
        for m in 0..8 {
            s[sh::TEX + m] = NO_TEXTURE;
            if used & (1 << m) != 0 {
                let (id, sampler, size) = self.texture(m, mem);
                s[sh::TEX + m] = id;
                s[sh::SAMPLER + m] = sampler;
                s[sh::TEX_SIZE + m] = size;
            }
        }
        Shading(s)
    }

    /// VAT `vat`'s vertex layout under the current CP registers.
    fn layout(&mut self, vat: usize) -> Layout {
        let cp = &self.cp;
        *self.layouts[vat].get_or_insert_with(|| Layout::new(cp, vat))
    }

    fn draw(&mut self, op: u8, n: usize, data: &[u8], mem: &dyn Ram) {
        if self.drop_draws {
            return;
        }
        self.stats.draws += 1;
        self.stats.vertices += n as u64;
        if n == 0 {
            return;
        }
        let prim = op & 0xF8;
        match prim {
            0xA8 | 0xB0 => self.stats.lines += 1,
            0xB8 => self.stats.points += 1,
            _ => {}
        }
        // Draws too short for one triangle, line or point draw nothing.
        let draws = match prim {
            0x80 | 0x88 => n >= 4,
            0x90 | 0x98 | 0xA0 => n >= 3,
            0xA8 | 0xB0 => n >= 2,
            _ => true,
        };
        if !draws {
            return;
        }
        let layout = self.layout((op & 7) as usize);
        let (pm, tm) = self.xf.default_matrices();
        let (ox, oy) = self.scissor_offset();
        let screen = xf::Screen { offset_x: ox, offset_y: oy };
        let has_color = [layout.has_color(0), layout.has_color(1)];
        let first = self.batch.raws.len() as u32;
        let arrays = vertex::Arrays::new(&self.cp, &layout, mem);
        // Each vertex is read straight into its slot, which starts as the
        // template (defaults); see `Layout::read_into`.
        let chunks = data.chunks_exact(layout.size.max(1)).take(n);
        let start = self.batch.raws.len();
        self.batch.raws.resize(start + chunks.len(), vertex::Layout::template(pm, tm));
        for (v, d) in self.batch.raws[start..].iter_mut().zip(chunks) {
            layout.read_into(v, &self.cp, d, &arrays, mem);
        }
        let n = self.batch.raws.len() as u32 - first;
        let shading = self.shading(mem);
        let shading = match self.batch.shadings.last() {
            Some(last) if *last == shading => self.batch.shadings.len() as u32 - 1,
            _ => {
                self.batch.shadings.push(shading);
                self.batch.shadings.len() as u32 - 1
            }
        };
        let raster = Raster {
            blend: self.bp[0x41] & 0xFFFF,
            zmode: self.bp[0x40] & 0x1F,
            cull: (self.bp[0x00] >> 14) & 3,
            efb_alpha: self.efb_alpha(),
        };
        let scissor = self.scissor();
        self.batch.ops.push(Op::Draw(DrawOp { op, first, n, has_color, screen, shading, raster, scissor, line_point: self.bp[0x22] }));
    }
}

/// Pixels to clip-space units at `w`.
fn px(dx: f32, dy: f32, w: f32) -> (f32, f32) {
    (dx / (EFB_WIDTH / 2.0) * w, dy / (EFB_HEIGHT / 2.0) * w)
}

pub(crate) fn line_quad(out: &mut Vec<Vertex>, a: Vertex, b: Vertex, width: f32) {
    let (ax, ay) = (a.pos[0] / a.pos[3] * EFB_WIDTH / 2.0, a.pos[1] / a.pos[3] * EFB_HEIGHT / 2.0);
    let (bx, by) = (b.pos[0] / b.pos[3] * EFB_WIDTH / 2.0, b.pos[1] / b.pos[3] * EFB_HEIGHT / 2.0);
    let (dx, dy) = (bx - ax, by - ay);
    let len = (dx * dx + dy * dy).sqrt().max(1e-6);
    let (nx, ny) = (-dy / len * width / 2.0, dx / len * width / 2.0);
    let off = |v: Vertex, s: f32| {
        let mut v = v;
        let (ox, oy) = px(nx * s, ny * s, v.pos[3]);
        v.pos[0] += ox;
        v.pos[1] += oy;
        v
    };
    let (a0, a1, b0, b1) = (off(a, 1.0), off(a, -1.0), off(b, 1.0), off(b, -1.0));
    out.extend([a0, b0, b1, a1]);
}

pub(crate) fn point_quad(out: &mut Vec<Vertex>, p: Vertex, size: f32) {
    let c = |sx: f32, sy: f32| {
        let mut v = p;
        let (ox, oy) = px(sx * size / 2.0, sy * size / 2.0, v.pos[3]);
        v.pos[0] += ox;
        v.pos[1] += oy;
        v
    };
    out.extend([c(-1.0, -1.0), c(1.0, -1.0), c(1.0, 1.0), c(-1.0, 1.0)]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_window_matches_projecting_through_it() {
        // A world point in view space (the camera at the origin), the world
        // viewport, and the game camera's projection.
        let (cam, win, off) = ([2000.0f32, 1100.0], [480.0f32, 264.0], [700.0f32, -200.0]);
        let vp = [320.0, -228.0, 16_777_216.0, 662.0, 570.0, 16_777_216.0];
        let screen = xf::Screen { offset_x: 342.0, offset_y: 342.0 };
        let proj = |w: f32, h: f32, ox: f32, oy: f32| [2.0 / w, -2.0 / w * ox, 2.0 / h, -2.0 / h * oy, -5e-5, -1.0];
        let p = [640.0, -150.0, -10.0];
        let mut v = Vertex { pos: xf::project(p, &proj(cam[0], cam[1], 0.0, 0.0), true, &vp, screen), ..Default::default() };
        let half = (EFB_WIDTH / 2.0, EFB_HEIGHT / 2.0);
        let world = World { ranges: vec![(0, 1)], camera: WorldCamera { size: cam, ..Default::default() }, map: [vp[0] / half.0, (vp[3] - screen.offset_x) / half.0 - 1.0, vp[1] / half.1, (vp[4] - screen.offset_y) / half.1 - 1.0], ..Default::default() };
        world.apply(std::slice::from_mut(&mut v), &xf::View { offset: off, size: win }, None);
        let want = xf::project(p, &proj(win[0], win[1], off[0], off[1]), true, &vp, screen);
        for i in 0..4 {
            assert!((v.pos[i] - want[i]).abs() < 1e-4, "{i}: {:?} vs {:?}", v.pos, want);
        }
    }

    fn bp(v: u32) -> [u8; 5] {
        let b = v.to_be_bytes();
        [0x61, b[0], b[1], b[2], b[3]]
    }

    #[test]
    fn decodes_a_quad_and_bp_events() {
        let mem = Mem::new(Box::new(guest_mem::NullMmio));
        let mut g = Gpu::default();
        let mut s = Vec::new();
        // VCD: position direct; VAT0: XY f32 (one position element pair).
        s.extend([0x08, 0x50, 0, 0, 0x02, 0x00]);
        s.extend([0x08, 0x70, 0, 0, 0, 0x08]); // pos fmt f32 (4 << 1), XY
        // XF: one colour channel, ortho identity-ish projection and a viewport.
        s.extend([0x10, 0x00, 0x00, 0x10, 0x09, 0, 0, 0, 1]);
        s.extend([0x10, 0x00, 0x00, 0x10, 0x26, 0, 0, 0, 1]);
        s.extend(bp(0x4500_0001));
        s.extend([0x80, 0x00, 0x04]);
        for (x, y) in [(0.0f32, 0.0f32), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)] {
            s.extend(x.to_be_bytes());
            s.extend(y.to_be_bytes());
        }
        s.extend(bp(0x4700_1234));
        let mut ev = Vec::new();
        let (a, b) = s.split_at(20);
        g.feed(a, &mem, &mut ev);
        g.feed(b, &mem, &mut ev);
        assert_eq!(ev, vec![Event::Finish, Event::TokenInt(0x1234)]);
        assert_eq!(g.stats.bad_opcodes, 0);
        let out = g.take_output();
        // One quad: four vertices, two triangles.
        assert_eq!(out.vertices.len(), 4);
        assert_eq!(out.model.len(), 4);
        assert_eq!(out.indices, vec![0, 1, 2, 0, 2, 3]);
        assert_eq!(out.records.len(), 1);
        assert_eq!((out.records[0].first, out.records[0].count), (0, 4));
        assert_eq!(out.cmds.len(), 1);
        assert!(matches!(out.cmds[0], Cmd::Draw(Draw { first: 0, count: 6, .. })));
    }

    #[test]
    fn bp_mask_limits_the_next_write() {
        let mem = Mem::new(Box::new(guest_mem::NullMmio));
        let mut g = Gpu::default();
        let mut ev = Vec::new();
        let mut s = Vec::new();
        s.extend(bp(0xF6_00_00_0F));
        s.extend(bp(0xFE_00_00_F3));
        s.extend(bp(0xF6_00_00_F0));
        g.feed(&s, &mem, &mut ev);
        assert_eq!(g.bp[0xF6], 0xFC);
    }

    #[test]
    fn copy_format_is_unscrambled() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        mem.write_u32(0x8000_0000, 0).unwrap();
        let mut g = Gpu::default();
        let mut ev = Vec::new();
        let mut s = Vec::new();
        s.extend(bp(0x4900_0000));
        s.extend(bp(0x4A00_0000 | (7 << 10) | 7)); // 8x8
        s.extend(bp(0x4B00_0100));
        // Copy format 8 (R8): bits 3..6 hold 0b0001 (the format's top bit moved to the bottom).
        s.extend(bp(0x5200_0000 | (1 << 3)));
        g.feed(&s, &mem, &mut ev);
        let out = g.take_output();
        let Cmd::CopyTex(c) = out.cmds[0] else { panic!("expected a texture copy") };
        assert_eq!((c.format, c.tex_format, c.addr), (8, texture::I8, 0x2000));
    }

    #[test]
    fn tmem_preload_feeds_textures() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        for i in 0..32u32 {
            mem.write_u8(0x8000_2000 + i, i as u8 + 1).unwrap();
        }
        let mut g = Gpu::default();
        let mut ev = Vec::new();
        let mut s = Vec::new();
        s.extend(bp(0x6000_0100)); // RAM 0x2000
        s.extend(bp(0x6100_0010)); // TMEM 0x200
        s.extend(bp(0x6300_0001)); // one line
        g.feed(&s, &mem, &mut ev);
        assert_eq!(g.stats.tmem_preloads, 1);
        assert_eq!(&g.tmem[0x200..0x204], &[1, 2, 3, 4]);
    }

    #[test]
    fn masked_clear_becomes_a_draw() {
        let mem = Mem::new(Box::new(guest_mem::NullMmio));
        let mut g = Gpu::default();
        let mut ev = Vec::new();
        let mut s = Vec::new();
        s.extend(bp(0x4300_0001)); // RGBA6_Z24: the EFB has alpha
        s.extend(bp(0x4100_0008)); // colour update only
        s.extend(bp(0x4A00_0000 | (15 << 10) | 15));
        s.extend(bp(0x4F00_FF11)); // A=FF R=11
        s.extend(bp(0x5200_0000 | (1 << 11) | (1 << 14)));
        g.feed(&s, &mem, &mut ev);
        let out = g.take_output();
        assert!(matches!(out.cmds[1], Cmd::Draw(Draw { count: 6, raster: Raster { blend: 8, .. }, .. })));
        assert_eq!(out.shadings[0].0[sh::FLAGS], FLAG_CLEAR);
    }
}
