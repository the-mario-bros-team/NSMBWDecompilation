//! The second half of GX decoding: transform, lighting and building the
//! field's [`Output`]. It needs no guest RAM: the first half ([`Gpu`]) loads
//! vertices and resolves textures and sends [`Op`]s in batches, so this half
//! can run on its own thread.
//!
//! [`Gpu`]: crate::Gpu

use crate::vertex::RawVertex;
use crate::xf::{self, Xf};
use crate::{depth_plane, hash_words, line_quad, point_quad, screen_point, sh, Cmd, Draw, DrawRecord, Output, Raster, Rect, Shading, Vertex, EFB_HEIGHT, EFB_WIDTH, FLAG_CLEAR, FLAG_ZFREEZE, NO_TEXTURE, SHADING_WORDS};

/// One step for [`Back`], in stream order.
pub(crate) enum Op {
    /// An XF register or memory write (matrices, lights, setup).
    Xf { addr: u32, v: u32 },
    Draw(DrawOp),
    /// A command that goes to the output as it is (copies, clears, pokes).
    Cmd(Cmd),
    /// A clear of colour or alpha only, as a masked full-rectangle draw.
    ClearQuad { rect: Rect, color: [u8; 4], z: u32, color_update: bool },
    /// The game camera for the draws after it (`Gpu::set_world`).
    World(Option<crate::WorldCamera>),
    /// Each player's window (`Gpu::set_views`).
    Views([Option<xf::View>; 4]),
}

/// A draw as the first half loaded it.
pub(crate) struct DrawOp {
    pub op: u8,
    /// Range in [`Batch::raws`].
    pub first: u32,
    pub n: u32,
    pub has_color: [bool; 2],
    pub screen: xf::Screen,
    /// Index in [`Batch::shadings`]; the z-freeze plane is filled in here.
    pub shading: u32,
    pub raster: Raster,
    pub scissor: Rect,
    /// Lines and points: their width and point size (BP 0x22).
    pub line_point: u32,
}

/// Ops with the vertices and shading blocks they refer to.
#[derive(Default)]
pub(crate) struct Batch {
    pub ops: Vec<Op>,
    pub raws: Vec<RawVertex>,
    pub shadings: Vec<Shading>,
}

impl Batch {
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

pub(crate) struct Back {
    /// The XF state, kept in step with the first half's by [`Op::Xf`].
    xf: Xf,
    /// Whether this field keeps what frame interpolation needs
    /// ([`Output::model`], [`Output::records`]); changes only between fields,
    /// so `model` stays aligned with `vertices`.
    records: bool,
    /// The last [`xf::DrawSetup`] and the XF generation it was built for.
    setup: Option<(u64, xf::DrawSetup)>,
    /// A finished field's buffers, for the next [`Back::take_output`].
    spare: Option<Output>,
    scratch_verts: Vec<Vertex>,
    scratch_model: Vec<([f32; 3], u8)>,
    /// Depth plane of the last triangle drawn without z-freeze.
    zplane: [f32; 3],
    /// The game camera while in a course (`Op::World`).
    world_camera: Option<crate::WorldCamera>,
    out: Output,
}

impl Default for Back {
    fn default() -> Self {
        Self { xf: Xf::default(), records: true, setup: None, spare: None, scratch_verts: Vec::new(), scratch_model: Vec::new(), zplane: [0.0; 3], world_camera: None, out: Output::default() }
    }
}

impl Back {
    pub fn apply(&mut self, batch: &Batch) {
        for op in &batch.ops {
            match op {
                Op::Xf { addr, v } => self.xf.write(*addr, *v),
                Op::Draw(d) => self.draw(d, batch),
                Op::Cmd(c) => self.out.cmds.push(*c),
                Op::ClearQuad { rect, color, z, color_update } => self.clear_quad(*rect, *color, *z, *color_update),
                Op::World(c) => {
                    self.world_camera = *c;
                    self.out.world.camera = c.unwrap_or_default();
                }
                Op::Views(v) => self.out.views = *v,
            }
        }
    }

    /// The field so far; fields from the next one carry interpolation
    /// records if `records_next`.
    pub fn take_output(&mut self, records_next: bool) -> Output {
        let mut fresh = self.spare.take().unwrap_or_default();
        // The camera and the windows hold until they are set again.
        fresh.world.camera = self.out.world.camera;
        fresh.views = self.out.views;
        self.records = records_next;
        std::mem::replace(&mut self.out, fresh)
    }

    pub fn recycle(&mut self, mut old: Output) {
        old.vertices.clear();
        old.indices.clear();
        old.model.clear();
        old.records.clear();
        old.shadings.clear();
        old.cmds.clear();
        old.new_textures = Vec::new();
        old.evicted = Vec::new();
        old.world.ranges.clear();
        old.world.layers.clear();
        self.spare = Some(old);
    }

    fn clear_quad(&mut self, rect: Rect, color: [u8; 4], z: u32, color_update: bool) {
        let mut s = [0u32; SHADING_WORDS];
        s[sh::FLAGS] = FLAG_CLEAR;
        s[sh::CLEAR_COLOR] = u32::from_le_bytes(color);
        for m in 0..8 {
            s[sh::TEX + m] = NO_TEXTURE;
        }
        let px = |x: u32, y: u32| Vertex {
            pos: [x as f32 / (EFB_WIDTH / 2.0) - 1.0, y as f32 / (EFB_HEIGHT / 2.0) - 1.0, z as f32 / 16_777_216.0, 1.0],
            ..Default::default()
        };
        let (x0, y0, x1, y1) = (rect.x, rect.y, rect.x + rect.w, rect.y + rect.h);
        let base = self.out.vertices.len() as u32;
        self.out.vertices.extend([px(x0, y0), px(x1, y0), px(x1, y1), px(x0, y1)]);
        if self.records {
            self.out.model.extend([([0.0; 3], 0); 4]);
        }
        let first = self.out.indices.len() as u32;
        self.out.indices.extend([0, 1, 2, 0, 2, 3].map(|i| base + i));
        self.out.shadings.push(Shading(s));
        let raster = Raster { blend: if color_update { 8 } else { 0x10 }, zmode: 0, cull: 0, efb_alpha: true };
        let shading = self.out.shadings.len() as u32 - 1;
        self.out.cmds.push(Cmd::Draw(Draw { first, count: 6, shading, raster, scissor: rect }));
    }

    fn draw(&mut self, d: &DrawOp, batch: &Batch) {
        let raw = &batch.raws[d.first as usize..(d.first + d.n) as usize];
        let gen = self.xf.setup_gen;
        if self.setup.as_ref().is_none_or(|(g, _)| *g != gen) {
            self.setup = Some((gen, self.xf.setup(d.screen, d.has_color)));
        }
        let setup = &mut self.setup.as_mut().unwrap().1;
        setup.screen = d.screen;
        setup.has_color = d.has_color;
        let setup = &self.setup.as_ref().unwrap().1;
        let mut verts = std::mem::take(&mut self.scratch_verts);
        let mut model = std::mem::take(&mut self.scratch_model);
        let prim = d.op & 0xF8;
        let fixed = prim >= 0xA8;
        let base = self.out.vertices.len() as u32;
        verts.clear();
        if fixed {
            verts.extend(raw.iter().map(|r| self.xf.transform(r, setup)));
        } else {
            // Triangles keep their vertices as they are: straight to the output.
            let xf = &self.xf;
            self.out.vertices.extend(raw.iter().map(|r| xf.transform(r, setup)));
        }
        model.clear();
        if self.records {
            model.extend(raw.iter().map(|r| (r.pos, r.pnmtx)));
        }

        // Each vertex once; triangle lists as indices, in the order the
        // hardware rasterises them.
        let first = self.out.indices.len() as u32;
        if fixed {
            // Lines and points become screen-space quads (four vertices each).
            let v = &mut self.out.vertices;
            if prim == 0xA8 || prim == 0xB0 {
                let width = (d.line_point & 0xFF) as f32 / 6.0;
                let step = if prim == 0xA8 { 2 } else { 1 };
                let mut i = 0;
                while i + 1 < verts.len() {
                    line_quad(v, verts[i], verts[i + 1], width.max(1.0));
                    i += step;
                }
            } else {
                let size = ((d.line_point >> 8) & 0xFF) as f32 / 6.0;
                for p in &verts {
                    point_quad(v, *p, size.max(1.0));
                }
            }
            let quads = (self.out.vertices.len() as u32 - base) / 4;
            self.out.indices.extend((0..quads).flat_map(|q| [0, 1, 2, 0, 2, 3].map(|i| base + 4 * q + i)));
            if self.records {
                self.out.model.resize(self.out.vertices.len(), ([0.0; 3], 0xFF));
            }
        } else {
            self.out.model.extend_from_slice(&model);
            let idx = &mut self.out.indices;
            let len = raw.len() as u32;
            match prim {
                0x80 | 0x88 => idx.extend((0..len / 4).flat_map(|q| [0, 1, 2, 0, 2, 3].map(|i| base + 4 * q + i))),
                0x90 => idx.extend(base..base + len / 3 * 3),
                0x98 => idx.extend((2..len).flat_map(|i| if i % 2 == 0 { [i - 2, i - 1, i] } else { [i - 1, i - 2, i] }.map(|k| base + k))),
                _ => idx.extend((2..len).flat_map(|i| [0, i - 1, i].map(|k| base + k))),
            }
        }
        let count = self.out.indices.len() as u32 - first;
        let vcount = self.out.vertices.len() as u32 - base;
        if let Some(camera) = self.world_camera {
            let (pr, ortho) = self.xf.projection();
            if ortho && vcount != 0 && xf::is_camera_projection(&pr, camera.size) {
                let vp = self.xf.viewport();
                let (half_w, half_h) = (crate::EFB_WIDTH / 2.0, crate::EFB_HEIGHT / 2.0);
                // As `xf::project` maps projected coordinates to clip space.
                self.out.world.map = [vp[0] / half_w, (vp[3] - d.screen.offset_x) / half_w - 1.0, vp[1] / half_h, (vp[4] - d.screen.offset_y) / half_h - 1.0];
                match self.out.world.ranges.last_mut() {
                    Some(r) if r.0 + r.1 == base => r.1 += vcount,
                    _ => self.out.world.ranges.push((base, vcount)),
                }
                let m = self.xf.position_matrix(raw[0].pnmtx);
                if m[2][3] < crate::LAYER_Z {
                    let layer = crate::Layer { first: base, count: vcount, z: m[2][3].round() as i32, scale: m[0][0].hypot(m[1][0]), t: [m[0][3], m[1][3]] };
                    match self.out.world.layers.last_mut() {
                        Some(l) if l.first + l.count == base && (l.z, l.scale, l.t) == (layer.z, layer.scale, layer.t) => l.count += vcount,
                        _ => self.out.world.layers.push(layer),
                    }
                }
            }
        }
        self.scratch_verts = verts;
        if count == 0 {
            self.scratch_model = model;
            return;
        }
        let mut shading = batch.shadings[d.shading as usize];
        let frozen = shading.0[sh::FLAGS] & FLAG_ZFREEZE != 0;
        // Without z-freeze, this draw's first triangle sets the plane a later
        // frozen draw takes its depth from.
        if !frozen && !fixed && count >= 3 {
            let i = &self.out.indices[first as usize..first as usize + 3];
            let t = [self.out.vertices[i[0] as usize], self.out.vertices[i[1] as usize], self.out.vertices[i[2] as usize]];
            if let (Some(a), Some(b), Some(c)) = (screen_point(&t[0]), screen_point(&t[1]), screen_point(&t[2])) {
                if let Some(p) = depth_plane(a, b, c) {
                    self.zplane = p;
                }
            }
        }
        if frozen {
            for i in 0..3 {
                shading.0[sh::ZPLANE + i] = self.zplane[i].to_bits();
            }
        }
        if self.records {
            self.record(d.op, d.n as usize, base, vcount, &shading, &d.raster, &model, d.screen, fixed);
        }
        self.scratch_model = model;
        let shading = match self.out.shadings.last() {
            Some(last) if *last == shading => self.out.shadings.len() as u32 - 1,
            _ => {
                self.out.shadings.push(shading);
                self.out.shadings.len() as u32 - 1
            }
        };
        let draw = Draw { first, count, shading, raster: d.raster, scissor: d.scissor };
        if let Some(Cmd::Draw(prev)) = self.out.cmds.last_mut() {
            if prev.shading == draw.shading && prev.raster == draw.raster && prev.scissor == draw.scissor && prev.first + prev.count == first {
                prev.count += count;
                return;
            }
        }
        self.out.cmds.push(Cmd::Draw(draw));
    }

    #[allow(clippy::too_many_arguments)]
    fn record(&mut self, op: u8, n: usize, first: u32, count: u32, s: &Shading, raster: &Raster, model: &[([f32; 3], u8)], screen: xf::Screen, fixed: bool) {
        let mut h = 0xCBF2_9CE4_8422_2325u64;
        hash_words(&mut h, &[u32::from(op), n as u32, raster.blend, raster.zmode, raster.cull, u32::from(raster.efb_alpha)]);
        hash_words(&mut h, &s.0[sh::GENMODE..sh::TEV_COLOR]);
        hash_words(&mut h, &s.0[sh::ALPHA_TEST..sh::TEX]);
        let mut slots = 0u64;
        for (_, slot) in model {
            slots |= 1 << (slot & 63);
            h = (h ^ u64::from(*slot)).wrapping_mul(0x0000_0100_0000_01B3);
        }
        let matrices = (0..64u8).filter(|i| slots & (1 << i) != 0).map(|i| (i, self.xf.position_matrix(i))).collect();
        let (projection, ortho) = self.xf.projection();
        self.out.records.push(DrawRecord { first, count, identity: h, projection, ortho, viewport: self.xf.viewport(), screen, matrices, fixed });
    }
}

/// Where the first half's batches go: applied at once, or to the
/// transform thread.
pub(crate) enum Sink {
    Local(Box<Back>),
    Thread(BackThread),
}

enum Msg {
    Batch(Batch),
    Recycle(Box<Output>),
    /// Reply with the field's output; the flag is `records_next`.
    TakeOutput(bool),
}

/// [`Back`] on its own thread.
pub(crate) struct BackThread {
    to: Option<std::sync::mpsc::Sender<Msg>>,
    from: std::sync::mpsc::Receiver<Box<Output>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl BackThread {
    pub fn spawn(mut back: Back) -> Self {
        let (to, msgs) = std::sync::mpsc::channel::<Msg>();
        let (replies, from) = std::sync::mpsc::channel();
        let handle = std::thread::Builder::new()
            .name("gx-transform".into())
            .spawn(move || {
                for msg in msgs {
                    match msg {
                        Msg::Batch(b) => back.apply(&b),
                        Msg::Recycle(out) => back.recycle(*out),
                        Msg::TakeOutput(records_next) => {
                            if replies.send(Box::new(back.take_output(records_next))).is_err() {
                                return;
                            }
                        }
                    }
                }
            })
            .expect("GX transform thread");
        Self { to: Some(to), from, handle: Some(handle) }
    }

    fn send(&self, msg: Msg) {
        self.to.as_ref().expect("GX transform thread").send(msg).expect("the GX transform thread stopped");
    }
}

impl Drop for BackThread {
    fn drop(&mut self) {
        drop(self.to.take());
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Sink {
    pub fn send(&mut self, batch: Batch) {
        match self {
            Self::Local(back) => back.apply(&batch),
            Self::Thread(t) => t.send(Msg::Batch(batch)),
        }
    }

    pub fn take_output(&mut self, records_next: bool) -> Output {
        match self {
            Self::Local(back) => back.take_output(records_next),
            Self::Thread(t) => {
                t.send(Msg::TakeOutput(records_next));
                *t.from.recv().expect("the GX transform thread stopped")
            }
        }
    }

    pub fn recycle(&mut self, out: Output) {
        match self {
            Self::Local(back) => back.recycle(out),
            Self::Thread(t) => t.send(Msg::Recycle(Box::new(out))),
        }
    }

    /// Moves the transform to its own thread (no-op if it already is).
    pub fn start_thread(&mut self) {
        if let Self::Local(back) = std::mem::replace(self, Self::Local(Box::default())) {
            *self = Self::Thread(BackThread::spawn(*back));
        }
    }
}
