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
        l
    }

    pub fn has_normal(&self) -> bool {
        self.nrm.0 != 0
    }

    pub fn has_color(&self, i: usize) -> bool {
        self.color[i].0 != 0
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
    [r as f32 / 255.0, gg as f32 / 255.0, bb as f32 / 255.0, a as f32 / 255.0]
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
