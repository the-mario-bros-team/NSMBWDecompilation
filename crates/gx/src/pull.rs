//! Vertex pulling: draws whose vertices the renderer decodes and transforms
//! itself ([`crate::Gpu::gpu_vertices`]). The decode thread copies each
//! draw's vertex bytes from the command stream as they are, and the indexed
//! arrays they use, into one byte buffer per field ([`Output::pull`]); the
//! renderer's vertex stage (`render`'s `pulled.wgsl`) reads the GX formats
//! from it (a [`Format`] per vertex format) and transforms them with the XF
//! state blocks (`xf::block`). The decode thread does no per-vertex work:
//! in World 1-1 a field's vertex bytes are about 440 KB and the array data
//! its draws read about 540 KB, where loading and converting every vertex
//! took most of the decode thread's time on a phone.
//!
//! [`transform_pulled`] is what the renderer computes, on the CPU: the same
//! bits as the CPU's own transform (`gx_bench --check-gpu`).

use std::collections::HashMap;

use crate::vertex::{color, comps, RawVertex};
use crate::{Output, Vertex};

/// Words in a [`Format`].
pub const FORMAT_WORDS: usize = 22;

/// A vertex format for the renderer: word 0 the vertex's size in bytes
/// (bits 0-7) and its attribute count (8-15), then one word per attribute in
/// stream order: what it is (bits 0-2: 0 position matrix index, 1 texture
/// matrix index, 2 position, 3 normal or NBT, 4 colour, 5 texture
/// coordinate), which one (3-5: texture matrix, colour or coordinate),
/// offset in the vertex (6-13), direct, 8-bit or 16-bit index (14-15: 1, 2,
/// 3), array (16-19), component or colour format (20-22), components
/// (23-26), and the scale as a power of two (27-31: 2^-n).
pub type Format = [u32; FORMAT_WORDS];

/// Words in a [`Output::pull_draws`] entry: where the draw's vertices start
/// in [`Output::pull`] (bytes), its [`Output::pull_formats`] entry, its
/// [`Output::states`] entry, a spare word, then per array the offset of its
/// element 0 in `pull` (may be outside the copy: only the elements the draw
/// reads are there) and the arrays' strides (bytes, four to a word).
pub const DRAW_WORDS: usize = 20;
pub const DRAW_ARRAYS: usize = 4;
pub const DRAW_STRIDES: usize = 16;

/// Bytes of an attribute: `dst` and the format word's fields.
fn attr_size(dst: u32, fmt: u32, count: u32) -> u32 {
    match dst {
        0 | 1 => 1,
        4 => [2, 3, 4, 2, 3, 4, 0, 0][(fmt & 7) as usize],
        _ => count * [1, 1, 2, 2, 4, 0, 0, 0][(fmt & 7) as usize],
    }
}

/// One attribute of a [`Format`].
pub(crate) fn step_word(dst: u32, idx: u32, at: u32, desc: u32, array: u32, fmt: u32, count: u32, scale: f32) -> u32 {
    // Scales are 1 or 2^-n: the exponent says which.
    let shift = 127 - ((scale.to_bits() >> 23) & 0xFF);
    dst | idx << 3 | at << 6 | desc << 14 | array << 16 | fmt << 20 | count << 23 | shift << 27
}

/// Draw `d`'s vertex `k` from `pull`, over `template` (the defaults):
/// what the renderer's vertex stage reads.
pub fn read_pulled(f: &Format, d: &[u32; DRAW_WORDS], pull: &[u8], k: u32, template: &RawVertex) -> RawVertex {
    let mut v = *template;
    let b = |a: u32| pull.get(a as usize).copied().unwrap_or(0);
    let size = f[0] & 0xFF;
    let at = d[0].wrapping_add(k * size);
    let stride = |a: usize| (d[DRAW_STRIDES + a / 4] >> (8 * (a % 4))) & 0xFF;
    for &w in &f[1..1 + ((f[0] >> 8) & 0xFF) as usize] {
        let (dst, idx, off, desc, array, fmt, count, shift) = (w & 7, (w >> 3) & 7, (w >> 6) & 0xFF, (w >> 14) & 3, ((w >> 16) & 15) as usize, (w >> 20) & 7, (w >> 23) & 15, w >> 27);
        let src = if desc == 1 {
            at + off
        } else {
            let i = if desc == 2 { u32::from(b(at + off)) } else { u32::from(b(at + off)) << 8 | u32::from(b(at + off + 1)) };
            d[DRAW_ARRAYS + array].wrapping_add(i.wrapping_mul(stride(array)))
        };
        let bytes: [u8; 36] = std::array::from_fn(|i| b(src.wrapping_add(i as u32)));
        let bytes = &bytes[..attr_size(dst, fmt, count).min(36) as usize];
        let (count, scale, idx) = (count as usize, f32::from_bits((127 - shift) << 23), idx as usize);
        match dst {
            0 => v.pnmtx = bytes[0] & 0x3F,
            1 => v.texmtx[idx] = bytes[0] & 0x3F,
            2 => comps(bytes, fmt, scale, &mut v.pos[..count.min(3)]),
            3 => comps(bytes, fmt, scale, &mut v.nbt.as_flattened_mut()[..count.min(9)]),
            4 => v.color[idx & 1] = color(bytes, fmt),
            _ => comps(bytes, fmt, scale, &mut v.tex[idx][..count.min(2)]),
        }
    }
    v
}

/// Which colours a [`Format`] has.
pub fn format_colors(f: &Format) -> [bool; 2] {
    let mut c = [false; 2];
    for &w in &f[1..1 + ((f[0] >> 8) & 0xFF) as usize] {
        if w & 7 == 4 {
            c[((w >> 3) & 1) as usize] = true;
        }
    }
    c
}

/// Draw `draw`'s vertex `k` of `out`, transformed as the renderer does.
pub fn transform_pulled(out: &Output, draw: u32, k: u32) -> Vertex {
    let d = &out.pull_draws[draw as usize];
    let f = &out.pull_formats[d[1] as usize];
    let state = out.states[d[2] as usize];
    let (pm, tm) = crate::xf::block_defaults(&out.xf_blocks, state);
    let template = crate::vertex::Layout::template(pm, tm);
    let v = read_pulled(f, d, &out.pull, k, &template);
    crate::xf::transform_with_blocks(&out.xf_blocks, state, &v, format_colors(f))
}

/// Chunks a [`Region`] can cover (256 KiB); a longer copy is made again
/// for every draw that reads it.
const REGION_CHUNKS: usize = 4;

/// A copy of part of RAM in the field's [`Output::pull`]: where it came from,
/// where it is, and the write generations of the 64 KiB chunks it covers
/// when it was made.
#[derive(Clone, Copy)]
struct Region {
    base: u32,
    start: u32,
    end: u32,
    offset: u32,
    chunks: [(u32, u32); REGION_CHUNKS],
    len: u8,
}

impl Region {
    /// Covers `start..end` and RAM under it is unwritten since the copy.
    fn holds(&self, start: u32, end: u32, view: &guest_mem::RamView) -> bool {
        self.start <= start && end <= self.end && self.chunks[..self.len as usize].iter().all(|&(c, g)| view.chunk_gen(c as usize) == g)
    }
}

/// A multiplicative hasher for the small integer and word-array keys here
/// (the standard one took a fifth of the decode thread).
#[derive(Default)]
struct Fx(u64);

impl std::hash::Hasher for Fx {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for c in bytes.chunks(8) {
            let mut b = [0u8; 8];
            b[..c.len()].copy_from_slice(c);
            self.write_u64(u64::from_le_bytes(b));
        }
    }

    fn write_u32(&mut self, v: u32) {
        self.write_u64(u64::from(v));
    }

    fn write_u64(&mut self, v: u64) {
        self.0 = (self.0.rotate_left(5) ^ v).wrapping_mul(0x517C_C1B7_2722_0A95);
    }
}

type FxMap<K, V> = HashMap<K, V, std::hash::BuildHasherDefault<Fx>>;

/// The decode thread's side of a field's [`Output::pull`]: how many bytes
/// it has sent, the formats it has sent (by content), and the array copies
/// made (by array base address), reused while RAM under them is unwritten.
/// The last format and each array slot's last copy are kept apart:
/// consecutive draws mostly use the same.
#[derive(Default)]
pub(crate) struct Pulling {
    pub bytes: u32,
    pub draws: u32,
    formats: FxMap<Format, u32>,
    last_format: Option<(Format, u32)>,
    regions: FxMap<u32, Region>,
    last_region: [Option<Region>; 12],
}

impl Pulling {
    /// A new field.
    pub fn reset(&mut self) {
        self.bytes = 0;
        self.draws = 0;
        self.formats.clear();
        self.last_format = None;
        self.regions.clear();
        self.last_region = [None; 12];
    }

    /// The index of `f` in the field's formats, appending it to `out` if new.
    pub fn format(&mut self, f: &Format, out: &mut Vec<Format>) -> u32 {
        if let Some((last, i)) = &self.last_format {
            if last == f {
                return *i;
            }
        }
        let next = self.formats.len() as u32;
        let i = *self.formats.entry(*f).or_insert_with(|| {
            out.push(*f);
            next
        });
        self.last_format = Some((*f, i));
        i
    }

    /// Appends `bytes` to the field's buffer (`out`); their offset.
    pub fn append(&mut self, bytes: &[u8], out: &mut Vec<u8>) -> u32 {
        let at = self.bytes;
        out.extend_from_slice(bytes);
        self.bytes += bytes.len() as u32;
        at
    }

    /// The offset in the field's buffer of array `slot`'s element 0 at RAM
    /// `base`, where RAM from `start` to `end` must be copied: a copy made
    /// before that covers it and is unwritten since, or a new one (appended
    /// to `out`: twice as large as the last when it grows, so the next draws
    /// fit in it too). None if it is not RAM.
    pub fn array(&mut self, slot: usize, base: u32, start: u32, end: u32, view: &guest_mem::RamView, out: &mut Vec<u8>) -> Option<u32> {
        let at = |r: &Region| r.offset.wrapping_add(base.wrapping_sub(r.start));
        if let Some(r) = &self.last_region[slot] {
            if r.base == base && r.holds(start, end, view) {
                return Some(at(r));
            }
        }
        let old = self.regions.get(&base).copied();
        if let Some(r) = &old {
            if r.holds(start, end, view) {
                self.last_region[slot] = Some(*r);
                return Some(at(r));
            }
        }
        // What the old copy covered too, rounded out to 256 bytes, and when
        // growing at least twice its length; within the RAM bank.
        let (mut s, mut e) = (start & !0xFF, end.saturating_add(0xFF) & !0xFF);
        if let Some(r) = &old {
            (s, e) = (s.min(r.start), e.max(r.end));
            let grown = (r.end - r.start).saturating_mul(2);
            if e - s < grown {
                e = s.saturating_add(grown);
            }
        }
        let (s, e) = [(s, e), (s, end.saturating_add(0xFF) & !0xFF), (start, end)].into_iter().find(|&(s, e)| e > s && view.offset(s, (e - s) as usize).is_some())?;
        let len = (e - s) as usize;
        let ram_off = view.offset(s, len)?;
        let (c0, c1) = (ram_off / guest_mem::CHUNK, (ram_off + len - 1) / guest_mem::CHUNK);
        let mut chunks = [(0, 0); REGION_CHUNKS];
        let n = c1 - c0 + 1;
        for (k, c) in (c0..=c1).take(REGION_CHUNKS).enumerate() {
            view.watch(c);
            chunks[k] = (c as u32, view.chunk_gen(c));
        }
        let bytes = guest_mem::Ram::ram(view, s, len)?;
        let offset = self.append(bytes, out);
        let r = Region { base, start: s, end: e, offset, chunks, len: n.min(REGION_CHUNKS) as u8 };
        // Longer copies are not kept: their chunks are not all watched.
        if n <= REGION_CHUNKS {
            self.regions.insert(base, r);
            self.last_region[slot] = Some(r);
        }
        Some(at(&r))
    }
}
