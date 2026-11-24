//! Vertex loader: reads vertices in the format the CP's VCD and VAT
//! registers describe (direct data in the stream, or 8/16-bit indices into
//! arrays in RAM) into floats.

use guest_mem::Ram;
#[cfg(test)]
use guest_mem::Mem;

/// Command-processor state the loader needs.
#[derive(Clone, Default)]
pub struct Cp {
    pub matidx_a: u32,
    pub matidx_b: u32,
    pub vcd_lo: u32,
    pub vcd_hi: u32,
    pub vat_a: [u32; 8],
    pub vat_b: [u32; 8],
    pub vat_c: [u32; 8],
    pub array_base: [u32; 16],
    pub array_stride: [u32; 16],
}

impl Cp {
    pub fn load(&mut self, reg: u8, v: u32) {
        let i = (reg & 7) as usize;
        match reg & 0xF0 {
            0x30 => self.matidx_a = v,
            0x40 => self.matidx_b = v,
            0x50 => self.vcd_lo = v,
            0x60 => self.vcd_hi = v,
            0x70 => self.vat_a[i] = v,
            0x80 => self.vat_b[i] = v,
            0x90 => self.vat_c[i] = v,
            0xA0 => self.array_base[(reg & 0xF) as usize] = v,
            0xB0 => self.array_stride[(reg & 0xF) as usize] = v & 0xFF,
            _ => {}
        }
    }
}

/// A vertex as loaded, before transform.
#[derive(Clone, Copy, Debug, Default)]
pub struct RawVertex {
    pub pnmtx: u8,
    pub texmtx: [u8; 8],
    pub pos: [f32; 3],
    /// Normal, tangent and binormal (NBT); only the normal without NBT.
    pub nbt: [[f32; 3]; 3],
    /// Colours 0 and 1, components 0..1. White when the vertex has none.
    pub color: [[f32; 4]; 2],
    pub tex: [[f32; 2]; 8],
}

/// How one attribute is stored: 0 absent, 1 direct, 2 index8, 3 index16.
fn desc(v: u32, shift: u32) -> u32 {
    (v >> shift) & 3
}

/// Component size for u8, s8, u16, s16, f32.
fn comp_size(fmt: u32) -> usize {
    [1, 1, 2, 2, 4, 0, 0, 0][(fmt & 7) as usize]
}

fn color_size(fmt: u32) -> usize {
    [2, 3, 4, 2, 3, 4, 0, 0][(fmt & 7) as usize]
}

fn index_size(d: u32, direct: usize) -> usize {
    match d {
        1 => direct,
        2 => 1,
        3 => 2,
        _ => 0,
    }
}

/// Format of one numeric attribute: element count, component format, and
/// fixed-point scale.
#[derive(Clone, Copy)]
struct Num {
    count: usize,
    fmt: u32,
    scale: f32,
}

impl Num {
    fn size(&self) -> usize {
        self.count * comp_size(self.fmt)
    }
}

/// The vertex layout for one VAT (kept until the CP registers change).
#[derive(Clone, Copy)]
pub struct Layout {
    pnmtx: bool,
    texmtx: [bool; 8],
    pos: (u32, Num),
    nrm: (u32, Num),
    /// NBT with three separate indices.
    nrm_index3: bool,
    color: [(u32, u32); 2],
    tex: [(u32, Num); 8],
    pub size: usize,
    /// The attributes in stream order, each at a fixed offset in the
    /// vertex ([`Layout::read_into`]); `None` for formats it does not
    /// cover (NBT with three indices), which take the general path.
    plan: Option<Plan>,
}

/// Where one attribute goes in a [`RawVertex`].
#[derive(Clone, Copy, Default)]
enum Dst {
    #[default]
    Pnmtx,
    Texmtx(u8),
    Pos,
    Nbt,
    Color(u8),
    Tex(u8),
}

/// One attribute of a vertex format, compiled.
#[derive(Clone, Copy, Default)]
struct Step {
    dst: Dst,
    /// Byte offset in the vertex.
    at: u8,
    /// 1 direct, 2 index8, 3 index16.
    desc: u8,
    array: u8,
    /// Bytes of attribute data (direct or in the array).
    size: u8,
    fmt: u8,
    count: u8,
    scale: f32,
}

/// A vertex format's attributes, at most 1 + 8 + 1 + 1 + 2 + 8.
#[derive(Clone, Copy)]
struct Plan {
    steps: [Step; 21],
    len: u8,
}

impl Layout {
    pub fn new(cp: &Cp, vat: usize) -> Self {
        let (lo, hi) = (cp.vcd_lo, cp.vcd_hi);
        let (a, b, c) = (cp.vat_a[vat], cp.vat_b[vat], cp.vat_c[vat]);
        let frac = |f: u32| 1.0 / (1u32 << (f & 31)) as f32;
        let fixed = |fmt: u32, f: u32| if fmt & 7 == 4 { 1.0 } else { frac(f) };
        let pos = Num { count: if a & 1 != 0 { 3 } else { 2 }, fmt: (a >> 1) & 7, scale: fixed((a >> 1) & 7, (a >> 4) & 31) };
        let nbt = (a >> 9) & 1 != 0;
        let nfmt = (a >> 10) & 7;
        let nscale = match comp_size(nfmt) {
            1 => 1.0 / 64.0,
            2 => 1.0 / 16384.0,
            _ => 1.0,
        };
        let nrm = Num { count: if nbt { 9 } else { 3 }, fmt: nfmt, scale: nscale };
        // (elements bit, format, frac) for TEX0..TEX7.
        let t = [
            (a >> 21, a >> 22, a >> 25),
            (b, b >> 1, b >> 4),
            (b >> 9, b >> 10, b >> 13),
            (b >> 18, b >> 19, b >> 22),
            (b >> 27, b >> 28, c),
            (c >> 5, c >> 6, c >> 9),
            (c >> 14, c >> 15, c >> 18),
            (c >> 23, c >> 24, c >> 27),
        ];
        let tex = std::array::from_fn(|i| {
            let (e, f, fr) = t[i];
            (desc(hi, 2 * i as u32), Num { count: 1 + (e & 1) as usize, fmt: f & 7, scale: fixed(f & 7, fr & 31) })
        });
        let mut l = Layout {
            pnmtx: lo & 1 != 0,
            texmtx: std::array::from_fn(|i| (lo >> (1 + i)) & 1 != 0),
            pos: (desc(lo, 9), pos),
            nrm: (desc(lo, 11), nrm),
            nrm_index3: nbt && (a >> 31) != 0,
            color: [(desc(lo, 13), (a >> 14) & 7), (desc(lo, 15), (a >> 18) & 7)],
            tex,
            size: 0,
            plan: None,
        };
        let mut size = l.pnmtx as usize + l.texmtx.iter().filter(|&&t| t).count();
        size += index_size(l.pos.0, l.pos.1.size());
        size += match (l.nrm.0, l.nrm_index3) {
            (2, true) => 3,
            (3, true) => 6,
            (d, _) => index_size(d, l.nrm.1.size()),
        };
        for (d, f) in l.color {
            size += index_size(d, color_size(f));
        }
        for (d, n) in l.tex {
            size += index_size(d, n.size());
        }
        l.size = size;
        l.plan = l.compile();
        l
    }

    /// The attributes as [`Step`]s, in the order the stream has them.
    fn compile(&self) -> Option<Plan> {
        if self.nrm_index3 && self.nrm.0 >= 2 {
            return None;
        }
        let mut p = Plan { steps: [Step::default(); 21], len: 0 };
        let mut at = 0usize;
        let mut push = |p: &mut Plan, step: Step, bytes: usize| {
            p.steps[p.len as usize] = Step { at: at as u8, ..step };
            p.len += 1;
            at += bytes;
        };
        if self.pnmtx {
            push(&mut p, Step { dst: Dst::Pnmtx, desc: 1, size: 1, ..Step::default() }, 1);
        }
        for i in 0..8 {
            if self.texmtx[i] {
                push(&mut p, Step { dst: Dst::Texmtx(i as u8), desc: 1, size: 1, ..Step::default() }, 1);
            }
        }
        let num = |dst: Dst, d: u32, array: usize, n: Num| Step { dst, desc: d as u8, array: array as u8, size: n.size() as u8, fmt: n.fmt as u8, count: n.count as u8, scale: n.scale, at: 0 };
        let (pd, pn) = self.pos;
        if pd != 0 {
            push(&mut p, num(Dst::Pos, pd, 0, pn), index_size(pd, pn.size()));
        }
        let (nd, nn) = self.nrm;
        if nd != 0 {
            push(&mut p, num(Dst::Nbt, nd, 1, nn), index_size(nd, nn.size()));
        }
        for c in 0..2 {
            let (d, f) = self.color[c];
            if d != 0 {
                let step = Step { dst: Dst::Color(c as u8), desc: d as u8, array: 2 + c as u8, size: color_size(f) as u8, fmt: f as u8, ..Step::default() };
                push(&mut p, step, index_size(d, color_size(f)));
            }
        }
        for t in 0..8 {
            let (d, n) = self.tex[t];
            if d != 0 {
                push(&mut p, num(Dst::Tex(t as u8), d, 4 + t, n), index_size(d, n.size()));
            }
        }
        (at == self.size).then_some(p)
    }

    pub fn has_normal(&self) -> bool {
        self.nrm.0 != 0
    }

    pub fn has_color(&self, i: usize) -> bool {
        self.color[i].0 != 0
    }

    /// The vertices carry a position or texture matrix index.
    pub fn has_matrix_indices(&self) -> bool {
        self.pnmtx || self.texmtx.contains(&true)
    }

    /// How [`Pack`] stores this format's vertices.
    pub(crate) fn pack(&self) -> Pack {
        Pack::new(self)
    }

    /// Reads one vertex from `data` (exactly `self.size` bytes); indexed
    /// attributes come from `arrays` (made for this layout from `mem`).
    pub fn read<'a>(&self, cp: &Cp, data: &'a [u8], arrays: &Arrays<'a>, mem: &'a dyn Ram, default_pnmtx: u8, default_texmtx: [u8; 8]) -> RawVertex {
        let mut v = Self::template(default_pnmtx, default_texmtx);
        self.read_into(&mut v, cp, data, arrays, mem);
        v
    }

    /// A vertex with no attributes read yet: the default matrices, white.
    pub fn template(default_pnmtx: u8, default_texmtx: [u8; 8]) -> RawVertex {
        RawVertex { pnmtx: default_pnmtx, texmtx: default_texmtx, color: [[1.0; 4]; 2], ..Default::default() }
    }

    /// [`Layout::read`] into `v`, which holds [`Layout::template`]: only the
    /// attributes the vertex has are written. Built on the stack and copied
    /// out, every vertex stalled on reading back its own fresh stores.
    pub fn read_into<'a>(&self, v: &mut RawVertex, cp: &Cp, data: &'a [u8], arrays: &Arrays<'a>, mem: &'a dyn Ram) {
        match &self.plan {
            Some(plan) if data.len() >= self.size => Self::read_planned(plan, v, cp, data, arrays, mem),
            _ => self.read_general(v, cp, data, arrays, mem),
        }
    }

    /// [`Layout::read_into`] by the compiled plan: each attribute at its
    /// fixed offset, with the same conversions as the general path.
    #[inline(always)]
    fn read_planned<'a>(plan: &Plan, v: &mut RawVertex, cp: &Cp, data: &'a [u8], arrays: &Arrays<'a>, mem: &'a dyn Ram) {
        for s in &plan.steps[..plan.len as usize] {
            let at = s.at as usize;
            let size = s.size as usize;
            let b: &[u8] = match s.desc {
                1 => &data[at..at + size],
                d => {
                    let idx = if d == 2 { u32::from(data[at]) } else { u32::from(u16::from_be_bytes([data[at], data[at + 1]])) };
                    let array = s.array as usize;
                    let off = (idx * cp.array_stride[array]) as usize;
                    match arrays.bank[array] {
                        Some(bank) => bank.get(off..off + size),
                        None => mem.ram(cp.array_base[array].wrapping_add(off as u32), size),
                    }
                    .unwrap_or(&ZEROS[..size.min(ZEROS.len())])
                }
            };
            let (fmt, count, scale) = (u32::from(s.fmt), s.count as usize, s.scale);
            match s.dst {
                Dst::Pnmtx => v.pnmtx = b[0] & 0x3F,
                Dst::Texmtx(i) => v.texmtx[i as usize] = b[0] & 0x3F,
                Dst::Pos => comps(b, fmt, scale, &mut v.pos[..count]),
                Dst::Nbt => comps(b, fmt, scale, &mut v.nbt.as_flattened_mut()[..count]),
                Dst::Color(c) => v.color[c as usize] = color(b, fmt),
                Dst::Tex(t) => comps(b, fmt, scale, &mut v.tex[t as usize][..count]),
            }
        }
    }

    /// The vertex at `data` as [`Pack::write_into`] packs it (`pack` is this
    /// layout's, `pm` and `tm` the default matrices), written straight into
    /// `out` (`pack.words` words): each attribute converted as
    /// [`Layout::read_into`] converts it, without a whole [`RawVertex`] in
    /// between. False (nothing written) for formats without a plan.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn read_packed<'a>(&self, pack: &Pack, pm: u8, tm: [u8; 8], cp: &Cp, data: &'a [u8], arrays: &Arrays<'a>, mem: &'a dyn Ram, out: &mut [u32]) -> bool {
        let Some(plan) = &self.plan else { return false };
        if data.len() < self.size {
            return false;
        }
        let mut mtx = [pm, tm[0], tm[1], tm[2], tm[3], tm[4], tm[5], tm[6], tm[7]];
        // Steps come in the order the words go (matrix indices first).
        let mut w = if pack.mtx { 3 } else { 0 };
        for s in &plan.steps[..plan.len as usize] {
            let at = s.at as usize;
            let size = s.size as usize;
            let b: &[u8] = match s.desc {
                1 => &data[at..at + size],
                d => {
                    let idx = if d == 2 { u32::from(data[at]) } else { u32::from(u16::from_be_bytes([data[at], data[at + 1]])) };
                    let array = s.array as usize;
                    let off = (idx * cp.array_stride[array]) as usize;
                    match arrays.bank[array] {
                        Some(bank) => bank.get(off..off + size),
                        None => mem.ram(cp.array_base[array].wrapping_add(off as u32), size),
                    }
                    .unwrap_or(&ZEROS[..size.min(ZEROS.len())])
                }
            };
            let (fmt, count, scale) = (u32::from(s.fmt), s.count as usize, s.scale);
            match s.dst {
                Dst::Pnmtx => mtx[0] = b[0] & 0x3F,
                Dst::Texmtx(i) => mtx[1 + i as usize] = b[0] & 0x3F,
                Dst::Pos => {
                    let mut p = [0.0f32; 3];
                    comps(b, fmt, scale, &mut p[..count]);
                    out[w..w + 3].copy_from_slice(&p.map(f32::to_bits));
                    w += 3;
                }
                Dst::Nbt => {
                    let mut n = [0.0f32; 9];
                    comps(b, fmt, scale, &mut n[..count]);
                    for (o, f) in out[w..w + count].iter_mut().zip(&n) {
                        *o = f.to_bits();
                    }
                    w += count;
                }
                Dst::Color(_) => {
                    out[w] = u32::from_le_bytes(color_u8(b, fmt).map(|x| x as u8));
                    w += 1;
                }
                Dst::Tex(_) => {
                    let mut t = [0.0f32; 2];
                    comps(b, fmt, scale, &mut t[..count]);
                    out[w] = t[0].to_bits();
                    out[w + 1] = t[1].to_bits();
                    w += 2;
                }
            }
        }
        if pack.mtx {
            out[0] = u32::from_le_bytes([mtx[0], mtx[1], mtx[2], mtx[3]]);
            out[1] = u32::from_le_bytes([mtx[4], mtx[5], mtx[6], mtx[7]]);
            out[2] = u32::from(mtx[8]);
        }
        true
    }

    /// [`Layout::read_into`] for any format, reading the attributes in turn.
    fn read_general<'a>(&self, v: &mut RawVertex, cp: &Cp, data: &'a [u8], arrays: &Arrays<'a>, mem: &'a dyn Ram) {
        let mut cur = Cursor { data, at: 0 };
        if self.pnmtx {
            v.pnmtx = cur.take(1).first().copied().unwrap_or(0) & 0x3F;
        }
        for i in 0..8 {
            if self.texmtx[i] {
                v.texmtx[i] = cur.take(1).first().copied().unwrap_or(0) & 0x3F;
            }
        }
        // Bytes of attribute `array` under descriptor `d`: inline, or indexed
        // (borrowed, not copied: this runs for every attribute of every vertex).
        let fetch = |d: u32, array: usize, size: usize, cur: &mut Cursor<'a>| -> &'a [u8] {
            match d {
                1 => cur.take(size),
                2 | 3 => {
                    let ib = cur.take(if d == 2 { 1 } else { 2 });
                    let idx = ib.iter().fold(0u32, |a, &b| (a << 8) | u32::from(b));
                    let off = (idx * cp.array_stride[array]) as usize;
                    match arrays.bank[array] {
                        Some(bank) => bank.get(off..off + size),
                        None => mem.ram(cp.array_base[array].wrapping_add(off as u32), size),
                    }
                    .unwrap_or(&ZEROS[..size.min(ZEROS.len())])
                }
                _ => &[],
            }
        };
        let (pd, pn) = self.pos;
        if pd != 0 {
            let b = fetch(pd, 0, pn.size(), &mut cur);
            comps(b, pn.fmt, pn.scale, &mut v.pos[..pn.count]);
        }
        let (nd, nn) = self.nrm;
        if nd != 0 {
            let mut three = [0u8; 36];
            let b: &[u8] = if self.nrm_index3 && nd >= 2 {
                // Three indices, each selecting one vector from the same array.
                let one = 3 * comp_size(nn.fmt);
                for k in 0..3 {
                    let part = fetch(nd, 1, one, &mut cur);
                    let at = (k * one).min(three.len());
                    let n = part.len().min(three.len() - at);
                    three[at..at + n].copy_from_slice(&part[..n]);
                }
                &three[..(3 * one).min(three.len())]
            } else {
                fetch(nd, 1, nn.size(), &mut cur)
            };
            comps(b, nn.fmt, nn.scale, &mut v.nbt.as_flattened_mut()[..nn.count]);
        }
        for c in 0..2 {
            let (d, f) = self.color[c];
            if d != 0 {
                let b = fetch(d, 2 + c, color_size(f), &mut cur);
                v.color[c] = color(b, f);
            }
        }
        for t in 0..8 {
            let (d, n) = self.tex[t];
            if d != 0 {
                let b = fetch(d, 4 + t, n.size(), &mut cur);
                comps(b, n.fmt, n.scale, &mut v.tex[t][..n.count]);
            }
        }
    }
}

/// The attributes of a [`RawVertex`] a vertex format sets, packed into
/// words: for draws the GPU transforms (`Output::packed`), and read back by
/// the CPU for the rest. A whole `RawVertex` is 160 bytes, and writing a
/// field's 76,000 of them (12 MB) took most of the decode thread's time on
/// a phone (a Snapdragon 835 wrote them at about 1 GB/s); packed, 1-1's
/// usual format takes 36 bytes. Colours are packed as the 8-bit components
/// they came from (every colour format's components are integers over
/// 255), so unpacking gives the same floats.
///
/// Words in order: matrix indices (3: position and texture matrices 0-2,
/// 3-6, 7), position (3 floats), normal or NBT (3 or 9), colour 0 and 1
/// (RGBA8, R in the low byte), then two floats per texture coordinate.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Pack {
    /// Position and texture matrix indices (3 words).
    mtx: bool,
    pos: bool,
    /// Normal (3 floats), NBT (9) or none.
    nbt: u8,
    color: [bool; 2],
    /// One bit per texture coordinate (2 floats each).
    tex: u8,
    /// Words per vertex.
    pub words: u8,
}

impl Pack {
    fn new(l: &Layout) -> Self {
        let mut p = Pack { mtx: l.has_matrix_indices(), pos: l.pos.0 != 0, nbt: if l.nrm.0 == 0 { 0 } else { l.nrm.1.count as u8 }, color: [l.color[0].0 != 0, l.color[1].0 != 0], tex: 0, words: 0 };
        for t in 0..8 {
            if l.tex[t].0 != 0 {
                p.tex |= 1 << t;
            }
        }
        p.words = (if p.mtx { 3 } else { 0 } + if p.pos { 3 } else { 0 } + p.nbt + p.color.iter().filter(|&&c| c).count() as u8 + 2 * p.tex.count_ones() as u8) as u8;
        p
    }

    /// Appends `v`'s attributes.
    #[inline(always)]
    #[cfg(test)]
    pub fn write(&self, v: &RawVertex, out: &mut Vec<u32>) {
        let at = out.len();
        out.resize(at + self.words as usize, 0);
        self.write_into(v, &mut out[at..]);
    }

    /// `v`'s attributes as this format packs them, into `out` (its first
    /// `words` words).
    #[inline(always)]
    pub fn write_into(&self, v: &RawVertex, out: &mut [u32]) {
        let w = out;
        let mut n = 0;
        let mut put = |x: u32| {
            w[n] = x;
            n += 1;
        };
        if self.mtx {
            let m = &v.texmtx;
            put(u32::from_le_bytes([v.pnmtx, m[0], m[1], m[2]]));
            put(u32::from_le_bytes([m[3], m[4], m[5], m[6]]));
            put(u32::from(m[7]));
        }
        if self.pos {
            v.pos.iter().for_each(|f| put(f.to_bits()));
        }
        v.nbt.as_flattened()[..self.nbt as usize].iter().for_each(|f| put(f.to_bits()));
        for c in 0..2 {
            if self.color[c] {
                put(u32::from_le_bytes(v.color[c].map(|x| (x * 255.0).round() as u8)));
            }
        }
        for t in 0..8 {
            if self.tex & (1 << t) != 0 {
                put(v.tex[t][0].to_bits());
                put(v.tex[t][1].to_bits());
            }
        }
    }

    /// The format as one word for the GPU (`shader.wgsl`'s `vs_packed`):
    /// bit 0 matrix indices, 1 position, 2-5 NBT floats (0, 3 or 9), 6 and
    /// 7 colours, 8-15 texture coordinates, 16-23 words per vertex.
    pub fn bits(&self) -> u32 {
        u32::from(self.mtx) | u32::from(self.pos) << 1 | u32::from(self.nbt) << 2 | u32::from(self.color[0]) << 6 | u32::from(self.color[1]) << 7 | u32::from(self.tex) << 8 | u32::from(self.words) << 16
    }

    /// Whether the vertices carry matrix indices.
    pub fn has_matrix_indices(&self) -> bool {
        self.mtx
    }

    /// The format [`Pack::bits`] describes.
    pub fn from_bits(b: u32) -> Self {
        Pack { mtx: b & 1 != 0, pos: b & 2 != 0, nbt: ((b >> 2) & 15) as u8, color: [b & 0x40 != 0, b & 0x80 != 0], tex: (b >> 8) as u8, words: (b >> 16) as u8 }
    }

    /// Which colours the vertices carry.
    pub fn colors(&self) -> [bool; 2] {
        self.color
    }

    /// The vertex `w` holds, over `template` (the defaults).
    #[inline(always)]
    pub fn read(&self, w: &[u32], template: &RawVertex) -> RawVertex {
        let mut v = *template;
        let mut at = 0;
        let mut take = || {
            let x = w.get(at).copied().unwrap_or(0);
            at += 1;
            x
        };
        if self.mtx {
            let [p, a, b, c] = take().to_le_bytes();
            let [d, e, f, g] = take().to_le_bytes();
            v.pnmtx = p;
            v.texmtx = [a, b, c, d, e, f, g, take() as u8];
        }
        if self.pos {
            v.pos = std::array::from_fn(|_| f32::from_bits(take()));
        }
        for f in v.nbt.as_flattened_mut()[..self.nbt as usize].iter_mut() {
            *f = f32::from_bits(take());
        }
        for c in 0..2 {
            if self.color[c] {
                v.color[c] = take().to_le_bytes().map(|x| x as f32 / 255.0);
            }
        }
        for t in 0..8 {
            if self.tex & (1 << t) != 0 {
                v.tex[t] = [f32::from_bits(take()), f32::from_bits(take())];
            }
        }
        v
    }
}

/// Stands in for array data outside RAM.
static ZEROS: [u8; 64] = [0; 64];

/// For one draw, each indexed attribute array's RAM from its base to the
/// end of the bank it is in (MEM1, MEM2 or the locked cache), looked up once
/// instead of per vertex. An index reaches at most 65535 * 255 bytes past
/// the base, never into another bank, so an attribute is in RAM exactly when
/// it lies inside this slice. Arrays based outside RAM are read per access.
pub struct Arrays<'a> {
    bank: [Option<&'a [u8]>; 12],
}

impl<'a> Arrays<'a> {
    pub fn new(cp: &Cp, layout: &Layout, mem: &'a dyn Ram) -> Self {
        let mut bank = [None; 12];
        let mut used = [false; 12];
        used[0] = layout.pos.0 >= 2;
        used[1] = layout.nrm.0 >= 2;
        for c in 0..2 {
            used[2 + c] = layout.color[c].0 >= 2;
        }
        for t in 0..8 {
            used[4 + t] = layout.tex[t].0 >= 2;
        }
        for (a, slot) in bank.iter_mut().enumerate() {
            if used[a] {
                let base = cp.array_base[a];
                *slot = bank_rest(base).and_then(|n| mem.ram(base, n));
            }
        }
        Self { bank }
    }
}

/// Bytes from `addr` to the end of the RAM bank it would be in, at any of
/// the bank's addresses (`guest_mem` maps MEM1 at 0x0, 0x8 and 0xC
/// prefixes, MEM2 at 0x1, 0x9 and 0xD, the locked cache at 0xE).
fn bank_rest(addr: u32) -> Option<usize> {
    let low = (addr & 0x0FFF_FFFF) as usize;
    let size = match addr >> 28 {
        0x0 | 0x8 | 0xC => guest_mem::MEM1_SIZE,
        0x1 | 0x9 | 0xD => guest_mem::MEM2_SIZE,
        0xE => guest_mem::LOCKED_CACHE_SIZE,
        _ => return None,
    };
    (low < size).then(|| size - low)
}

struct Cursor<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> &'a [u8] {
        let s = &self.data[self.at.min(self.data.len())..(self.at + n).min(self.data.len())];
        self.at += n;
        s
    }
}

/// `out.len()` components of format `fmt` from `b`, each [`comp`] times
/// `scale` (the same values), with the format chosen once for all of them:
/// this runs for every attribute of every vertex, and a call to `comp` per
/// component was most of the GX decode thread's time on the world map.
#[inline(always)]
fn comps(b: &[u8], fmt: u32, scale: f32, out: &mut [f32]) {
    fn each<const N: usize>(b: &[u8], scale: f32, out: &mut [f32], f: impl Fn([u8; N]) -> f32) {
        for (i, o) in out.iter_mut().enumerate() {
            *o = b.get(i * N..i * N + N).map_or(0.0, |c| f(c.try_into().unwrap())) * scale;
        }
    }
    match fmt {
        0 => each::<1>(b, scale, out, |c| f32::from(c[0])),
        1 => each::<1>(b, scale, out, |c| f32::from(c[0] as i8)),
        2 => each::<2>(b, scale, out, |c| f32::from(u16::from_be_bytes(c))),
        3 => each::<2>(b, scale, out, |c| f32::from(i16::from_be_bytes(c))),
        4 => each::<4>(b, scale, out, |c| f32::from_bits(u32::from_be_bytes(c))),
        _ => out.iter_mut().for_each(|o| *o = 0.0 * scale),
    }
}

/// Component `i` of format `fmt` (u8, s8, u16, s16, f32) as an unscaled float.
#[cfg(test)]
fn comp(b: &[u8], i: usize, fmt: u32) -> f32 {
    let s = comp_size(fmt);
    let Some(c) = b.get(i * s..i * s + s) else { return 0.0 };
    match fmt {
        0 => f32::from(c[0]),
        1 => f32::from(c[0] as i8),
        2 => f32::from(u16::from_be_bytes([c[0], c[1]])),
        3 => f32::from(i16::from_be_bytes([c[0], c[1]])),
        4 => f32::from_bits(u32::from_be_bytes([c[0], c[1], c[2], c[3]])),
        _ => 0.0,
    }
}

/// A colour in format `fmt` (RGB565, RGB888, RGB888x, RGBA4444, RGBA6, RGBA8888).
fn color(b: &[u8], fmt: u32) -> [f32; 4] {
    color_u8(b, fmt).map(|x| x as f32 / 255.0)
}

/// [`color`]'s components as the 8-bit integers they are made from.
fn color_u8(b: &[u8], fmt: u32) -> [u32; 4] {
    let g = |i: usize| u32::from(b.get(i).copied().unwrap_or(0));
    let (r, gg, bb, a) = match fmt {
        0 => {
            let v = g(0) << 8 | g(1);
            (((v >> 11) & 31) * 255 / 31, ((v >> 5) & 63) * 255 / 63, (v & 31) * 255 / 31, 255)
        }
        1 | 2 => (g(0), g(1), g(2), 255),
        3 => {
            let v = g(0) << 8 | g(1);
            (((v >> 12) & 15) * 17, ((v >> 8) & 15) * 17, ((v >> 4) & 15) * 17, (v & 15) * 17)
        }
        4 => {
            let v = g(0) << 16 | g(1) << 8 | g(2);
            let x = |s: u32| ((v >> s) & 63) * 255 / 63;
            (x(18), x(12), x(6), x(0))
        }
        _ => (g(0), g(1), g(2), g(3)),
    };
    [r, gg, bb, a]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `comps` gives `comp`'s values times the scale, bit for bit, for every
    /// format, short buffers included.
    #[test]
    fn comps_match_comp() {
        let bytes: Vec<u8> = (0..40u32).map(|i| (i.wrapping_mul(97) ^ 0xA5) as u8).collect();
        for fmt in 0..8 {
            for count in 1..=9 {
                for len in [0, 3, 7, 40] {
                    for scale in [1.0, 1.0 / 64.0, 1.0 / 16384.0, 0.25] {
                        let b = &bytes[..len];
                        let mut out = [7.0f32; 9];
                        comps(b, fmt, scale, &mut out[..count]);
                        for (i, o) in out[..count].iter().enumerate() {
                            assert_eq!(o.to_bits(), (comp(b, i, fmt) * scale).to_bits(), "fmt {fmt} count {count} len {len} i {i}");
                        }
                    }
                }
            }
        }
    }

    /// The compiled plan reads every format exactly as the general path,
    /// bit for bit, with direct, index8 and index16 attributes, arrays in
    /// RAM and outside it.
    #[test]
    fn planned_matches_general() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        for i in 0..0x4000u32 {
            mem.write_u32(0x8000_1000 + 4 * i, i.wrapping_mul(0x9E37_79B9) ^ 0x5A5A_1234).unwrap();
        }
        let mut seed = 0x1234_5678u32;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            seed
        };
        let (mut planned, mut checked) = (0, 0);
        for _ in 0..4000 {
            let mut cp = Cp { vcd_lo: rnd() & 0x1_FFFF, vcd_hi: rnd() & 0xFFFF, ..Cp::default() };
            cp.vat_a[0] = rnd();
            cp.vat_b[0] = rnd();
            cp.vat_c[0] = rnd();
            for a in 0..16 {
                // Most arrays in RAM; some at an unmapped base.
                cp.array_base[a] = if rnd() % 8 == 0 { 0x0D00_0000 } else { 0x1000 + (rnd() & 0x3FFF) };
                cp.array_stride[a] = rnd() & 0x3F;
            }
            let l = Layout::new(&cp, 0);
            if l.plan.is_some() {
                planned += 1;
            }
            let arrays = Arrays::new(&cp, &l, &mem);
            let data: Vec<u8> = (0..l.size).map(|_| rnd() as u8).collect();
            let (mut a, mut b) = (Layout::template(3, [1; 8]), Layout::template(3, [1; 8]));
            l.read_into(&mut a, &cp, &data, &arrays, &mem);
            l.read_general(&mut b, &cp, &data, &arrays, &mem);
            assert_eq!(format!("{a:?}"), format!("{b:?}"), "vcd {:x}/{:x} vat {:x} {:x} {:x}", cp.vcd_lo, cp.vcd_hi, cp.vat_a[0], cp.vat_b[0], cp.vat_c[0]);
            // Packed and unpacked, the vertex is the same.
            let pack = l.pack();
            let mut words = Vec::new();
            pack.write(&a, &mut words);
            assert_eq!(words.len(), pack.words as usize);
            // Packed straight from the data, it is the same words.
            let mut direct = vec![0u32; pack.words as usize];
            if l.read_packed(&pack, 3, [1; 8], &cp, &data, &arrays, &mem, &mut direct) {
                assert_eq!(direct, words, "read_packed: vcd {:x}/{:x} vat {:x}", cp.vcd_lo, cp.vcd_hi, cp.vat_a[0]);
            }
            let back = pack.read(&words, &Layout::template(3, [1; 8]));
            assert_eq!(format!("{back:?}"), format!("{a:?}"), "packed: vcd {:x}/{:x} vat {:x}", cp.vcd_lo, cp.vcd_hi, cp.vat_a[0]);
            checked += 1;
        }
        assert!(planned > checked / 2, "{planned} of {checked} formats planned");
    }

    /// Every 8-bit colour component survives packing exactly.
    #[test]
    fn packed_colours_are_exact() {
        for k in 0..=255u32 {
            let f = k as f32 / 255.0;
            assert_eq!(((f * 255.0).round() as u8) as f32 / 255.0, f, "{k}");
        }
    }

    #[test]
    fn direct_and_indexed_attributes() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let mut cp = Cp::default();
        // Position direct s16 XYZ frac 4; colour0 index8 RGBA8888; tex0 direct f32 ST.
        cp.vcd_lo = (1 << 9) | (2 << 13);
        cp.vcd_hi = 1;
        cp.vat_a[2] = 1 | (3 << 1) | (4 << 4) | (5 << 14) | (1 << 21) | (4 << 22);
        cp.array_base[2] = 0x1000;
        cp.array_stride[2] = 4;
        mem.write_u32(0x8000_1008, 0x11223344).unwrap(); // colour index 2
        let l = Layout::new(&cp, 2);
        assert_eq!(l.size, 6 + 1 + 8);
        let arrays = Arrays::new(&cp, &l, &mem);
        let mut d = Vec::new();
        for c in [16i16, -32, 8] {
            d.extend(c.to_be_bytes());
        }
        d.push(2);
        d.extend(0.5f32.to_be_bytes());
        d.extend(2.0f32.to_be_bytes());
        let v = l.read(&cp, &d, &arrays, &mem, 0, [0; 8]);
        assert_eq!(v.pos, [1.0, -2.0, 0.5]);
        assert_eq!(v.color[0], [0x11 as f32 / 255.0, 0x22 as f32 / 255.0, 0x33 as f32 / 255.0, 0x44 as f32 / 255.0]);
        assert_eq!(v.color[1], [1.0; 4]);
        assert_eq!(v.tex[0], [0.5, 2.0]);
    }
}

impl guest_mem::snap::Snapshot for Cp {
    fn snap(&mut self, s: &mut guest_mem::snap::Snap) {
        guest_mem::snap!(s; self.matidx_a, self.matidx_b, self.vcd_lo, self.vcd_hi, self.vat_a, self.vat_b, self.vat_c, self.array_base, self.array_stride);
    }
}
