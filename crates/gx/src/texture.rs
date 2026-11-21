//! Texture formats: tiled GX texels to linear RGBA8, and a cache keyed by
//! address, format and a hash of the bytes, so each texture is decoded and
//! uploaded once while its contents stay the same.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

/// A fast non-cryptographic hasher for the cache's small keys (the default
/// SipHash showed up in profiles).
#[derive(Default, Clone, Copy)]
pub struct FxHasher(u64);

impl FxHasher {
    fn add(&mut self, w: u64) {
        self.0 = (self.0.rotate_left(5) ^ w).wrapping_mul(0x517C_C1B7_2722_0A95);
    }
}

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        for c in bytes.chunks(8) {
            let mut b = [0u8; 8];
            b[..c.len()].copy_from_slice(c);
            self.add(u64::from_le_bytes(b));
        }
    }
    fn write_u8(&mut self, i: u8) {
        self.add(u64::from(i));
    }
    fn write_u16(&mut self, i: u16) {
        self.add(u64::from(i));
    }
    fn write_u32(&mut self, i: u32) {
        self.add(u64::from(i));
    }
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

pub type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub const I4: u32 = 0x0;
pub const I8: u32 = 0x1;
pub const IA4: u32 = 0x2;
pub const IA8: u32 = 0x3;
pub const RGB565: u32 = 0x4;
pub const RGB5A3: u32 = 0x5;
pub const RGBA8: u32 = 0x6;
pub const C4: u32 = 0x8;
pub const C8: u32 = 0x9;
pub const C14X2: u32 = 0xA;
pub const CMPR: u32 = 0xE;

/// (block width, block height, bytes per block).
pub fn block(fmt: u32) -> (usize, usize, usize) {
    match fmt {
        I4 | C4 | CMPR => (8, 8, 32),
        I8 | IA4 | C8 => (8, 4, 32),
        RGBA8 => (4, 4, 64),
        _ => (4, 4, 32),
    }
}

/// Bytes one mip level of `w` x `h` occupies in memory.
pub fn level_size(fmt: u32, w: usize, h: usize) -> usize {
    let (bw, bh, bytes) = block(fmt);
    w.div_ceil(bw) * h.div_ceil(bh) * bytes
}

pub fn is_paletted(fmt: u32) -> bool {
    matches!(fmt, C4 | C8 | C14X2)
}

/// Palette entries a paletted format can index.
pub fn palette_entries(fmt: u32) -> usize {
    match fmt {
        C4 => 16,
        C8 => 256,
        _ => 16384,
    }
}

fn x5(v: u16) -> u8 {
    let v = (v & 0x1F) as u8;
    (v << 3) | (v >> 2)
}

fn x6(v: u16) -> u8 {
    let v = (v & 0x3F) as u8;
    (v << 2) | (v >> 4)
}

fn x4(v: u16) -> u8 {
    (v & 0xF) as u8 * 17
}

fn x3(v: u16) -> u8 {
    let v = (v & 7) as u8;
    (v << 5) | (v << 2) | (v >> 1)
}

fn rgb565(v: u16) -> [u8; 4] {
    [x5(v >> 11), x6(v >> 5), x5(v), 255]
}

fn rgb5a3(v: u16) -> [u8; 4] {
    if v & 0x8000 != 0 {
        [x5(v >> 10), x5(v >> 5), x5(v), 255]
    } else {
        [x4(v >> 8), x4(v >> 4), x4(v), x3(v >> 12)]
    }
}

fn ia8(v: u16) -> [u8; 4] {
    let (a, i) = ((v >> 8) as u8, v as u8);
    [i, i, i, a]
}

/// One palette entry in TLUT format `tlut_fmt` (0 IA8, 1 RGB565, 2 RGB5A3).
fn palette(tlut: &[u8], tlut_fmt: u32, i: usize) -> [u8; 4] {
    let v = tlut.get(2 * i..2 * i + 2).map_or(0, |b| u16::from_be_bytes([b[0], b[1]]));
    match tlut_fmt {
        0 => ia8(v),
        1 => rgb565(v),
        _ => rgb5a3(v),
    }
}

/// Decodes one level to RGBA8 (`w * h * 4` bytes). Missing input reads as zero.
pub fn decode(fmt: u32, w: usize, h: usize, data: &[u8], tlut: &[u8], tlut_fmt: u32) -> Vec<u8> {
    let mut out = vec![0u8; w * h * 4];
    let (bw, bh, bytes) = block(fmt);
    let bx_count = w.div_ceil(bw);
    let byte = |i: usize| data.get(i).copied().unwrap_or(0);
    let half = |i: usize| u16::from_be_bytes([byte(i), byte(i + 1)]);
    let mut put = |x: usize, y: usize, px: [u8; 4]| {
        if x < w && y < h {
            out[(y * w + x) * 4..(y * w + x) * 4 + 4].copy_from_slice(&px);
        }
    };
    for by in 0..h.div_ceil(bh) {
        for bx in 0..bx_count {
            let base = (by * bx_count + bx) * bytes;
            let (x0, y0) = (bx * bw, by * bh);
            match fmt {
                CMPR => {
                    for sub in 0..4 {
                        let s = base + sub * 8;
                        let (c0, c1) = (half(s), half(s + 2));
                        let (a, b) = (rgb565(c0), rgb565(c1));
                        let mix = |p: u16, q: u16, d: u16| -> [u8; 4] {
                            let m = |i: usize| ((u16::from(a[i]) * p + u16::from(b[i]) * q) / d) as u8;
                            [m(0), m(1), m(2), 255]
                        };
                        let colors = if c0 > c1 { [a, b, mix(2, 1, 3), mix(1, 2, 3)] } else { [a, b, mix(1, 1, 2), [0, 0, 0, 0]] };
                        let (sx, sy) = (x0 + (sub & 1) * 4, y0 + (sub >> 1) * 4);
                        for row in 0..4 {
                            let bits = byte(s + 4 + row);
                            for col in 0..4 {
                                put(sx + col, sy + row, colors[((bits >> (6 - 2 * col)) & 3) as usize]);
                            }
                        }
                    }
                }
                RGBA8 => {
                    for i in 0..16 {
                        let (ar, gb) = (half(base + 2 * i), half(base + 32 + 2 * i));
                        put(x0 + i % 4, y0 + i / 4, [ar as u8, (gb >> 8) as u8, gb as u8, (ar >> 8) as u8]);
                    }
                }
                _ => {
                    for i in 0..bw * bh {
                        let (x, y) = (x0 + i % bw, y0 + i / bw);
                        let px = match fmt {
                            I4 | C4 => {
                                let n = (byte(base + i / 2) >> if i % 2 == 0 { 4 } else { 0 }) & 0xF;
                                if fmt == C4 {
                                    palette(tlut, tlut_fmt, n as usize)
                                } else {
                                    let v = n * 17;
                                    [v, v, v, v]
                                }
                            }
                            I8 => {
                                let v = byte(base + i);
                                [v, v, v, v]
                            }
                            C8 => palette(tlut, tlut_fmt, byte(base + i) as usize),
                            IA4 => {
                                let v = byte(base + i);
                                let (a, l) = ((v >> 4) * 17, (v & 0xF) * 17);
                                [l, l, l, a]
                            }
                            IA8 => ia8(half(base + 2 * i)),
                            RGB565 => rgb565(half(base + 2 * i)),
                            RGB5A3 => rgb5a3(half(base + 2 * i)),
                            C14X2 => palette(tlut, tlut_fmt, (half(base + 2 * i) & 0x3FFF) as usize),
                            _ => [255, 0, 255, 255],
                        };
                        put(x, y, px);
                    }
                }
            }
        }
    }
    out
}

/// Encodes RGBA8 texels (`w * h * 4` bytes, as [`decode`] produces them)
/// into GX texture format `fmt`: the bytes an EFB copy leaves in RAM.
/// Intensity formats take red as the intensity. Paletted and CMPR formats
/// are not copy targets.
pub fn encode(fmt: u32, w: usize, h: usize, rgba: &[u8]) -> Vec<u8> {
    let (bw, bh, bytes) = block(fmt);
    let (bx_count, by_count) = (w.div_ceil(bw), h.div_ceil(bh));
    let mut out = vec![0u8; bx_count * by_count * bytes];
    let px = |x: usize, y: usize| -> [u8; 4] {
        if x < w && y < h {
            let i = (y * w + x) * 4;
            [rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3]]
        } else {
            [0; 4]
        }
    };
    let rgb565 = |p: [u8; 4]| (u16::from(p[0] >> 3) << 11) | (u16::from(p[1] >> 2) << 5) | u16::from(p[2] >> 3);
    let rgb5a3 = |p: [u8; 4]| {
        if p[3] >= 0xE0 {
            0x8000 | (u16::from(p[0] >> 3) << 10) | (u16::from(p[1] >> 3) << 5) | u16::from(p[2] >> 3)
        } else {
            (u16::from(p[3] >> 5) << 12) | (u16::from(p[0] >> 4) << 8) | (u16::from(p[1] >> 4) << 4) | u16::from(p[2] >> 4)
        }
    };
    for by in 0..by_count {
        for bx in 0..bx_count {
            let base = (by * bx_count + bx) * bytes;
            let (x0, y0) = (bx * bw, by * bh);
            if fmt == RGBA8 {
                for i in 0..16 {
                    let p = px(x0 + i % 4, y0 + i / 4);
                    out[base + 2 * i] = p[3];
                    out[base + 2 * i + 1] = p[0];
                    out[base + 32 + 2 * i] = p[1];
                    out[base + 32 + 2 * i + 1] = p[2];
                }
                continue;
            }
            for i in 0..bw * bh {
                let p = px(x0 + i % bw, y0 + i / bw);
                match fmt {
                    I4 => out[base + i / 2] |= (p[0] >> 4) << if i % 2 == 0 { 4 } else { 0 },
                    I8 => out[base + i] = p[0],
                    IA4 => out[base + i] = (p[3] & 0xF0) | (p[0] >> 4),
                    IA8 => {
                        out[base + 2 * i] = p[3];
                        out[base + 2 * i + 1] = p[0];
                    }
                    RGB565 => out[base + 2 * i..base + 2 * i + 2].copy_from_slice(&rgb565(p).to_be_bytes()),
                    RGB5A3 => out[base + 2 * i..base + 2 * i + 2].copy_from_slice(&rgb5a3(p).to_be_bytes()),
                    _ => {}
                }
            }
        }
    }
    out
}

/// Where an EFB copy of `w` x `h` texels in format `fmt` lies in RAM, from
/// its address: each row of blocks' offset and length. The rows follow each
/// other unless `stride` (bytes from one row of blocks to the next, BP 0x4D)
/// is wider: the copy then patches part of a wider texture.
pub fn copy_rows(fmt: u32, w: usize, h: usize, stride: usize) -> impl Iterator<Item = (usize, usize)> {
    let (bw, bh, bytes) = block(fmt);
    let row = w.div_ceil(bw) * bytes;
    let step = stride.max(row);
    (0..h.div_ceil(bh)).map(move |r| (r * step, row))
}

/// The RAM bytes an EFB copy covers ([`copy_rows`]), its rows one after
/// another: what its texels encode to. `None` if a row is outside RAM.
pub fn copy_bytes(mem: &dyn guest_mem::Ram, addr: u32, fmt: u32, w: usize, h: usize, stride: usize) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    for (at, len) in copy_rows(fmt, w, h, stride) {
        out.extend_from_slice(mem.ram(addr.wrapping_add(at as u32), len)?);
    }
    Some(out)
}

/// The texture format an EFB copy in copy format `fmt` (BP 0x52, after
/// unscrambling) leaves in RAM.
pub fn copy_texture_format(fmt: u32) -> u32 {
    match fmt {
        0 => I4,
        1 | 7..=10 => I8,
        2 => IA4,
        3 | 11 | 12 => IA8,
        4 => RGB565,
        5 => RGB5A3,
        _ => RGBA8,
    }
}

/// 64-bit hash of texture bytes (not cryptographic; collisions only cost a
/// stale texture).
pub fn hash(data: &[u8]) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325 ^ data.len() as u64;
    let mut chunks = data.chunks_exact(8);
    for c in &mut chunks {
        h = (h.rotate_left(5) ^ u64::from_le_bytes(c.try_into().unwrap())).wrapping_mul(0x517C_C1B7_2722_0A95);
    }
    for &b in chunks.remainder() {
        h = (h.rotate_left(5) ^ u64::from(b)).wrapping_mul(0x517C_C1B7_2722_0A95);
    }
    h
}

/// What identifies a texture in memory, apart from its contents.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TexKey {
    /// Physical address of level 0.
    pub addr: u32,
    pub width: u16,
    pub height: u16,
    pub format: u32,
    pub levels: u8,
    /// TMEM offset and format of the palette (paletted formats only).
    pub tlut: u32,
    pub tlut_format: u32,
}

/// A decoded texture for the renderer to upload under `id`.
pub struct NewTexture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    /// RGBA8 levels, largest first (none when replaced).
    pub levels: Vec<Vec<u8>>,
    /// A texture pack's image to upload instead, at its own size
    /// ([`crate::replace`]).
    pub replacement: Option<std::sync::Arc<crate::replace::Image>>,
}

/// An EFB copy kept on the GPU as texture `id`, standing in for the bytes at
/// its destination while those bytes are unchanged.
struct EfbCopy {
    id: u32,
    width: u16,
    height: u16,
    ram_hash: u64,
}

#[derive(Default)]
pub struct Cache {
    /// (key, content hash) -> (id, generation last used).
    by_content: FastMap<(TexKey, u64), (u32, u64)>,
    /// Keys whose contents were hashed this generation -> id.
    checked: FastMap<TexKey, u32>,
    efb_copies: FastMap<u32, EfbCopy>,
    /// Each key's texture as last decided (id, generation), for EFB copies
    /// that patch part of one ([`Cache::patch_targets`]).
    live: FastMap<TexKey, (u32, u64)>,
    /// Changes whenever a lookup's answer could change (new generation, EFB
    /// copy, writeback, TMEM load); callers may memoise results per epoch.
    pub epoch: u64,
    next_id: u32,
    generation: u64,
    pub new: Vec<NewTexture>,
    pub evicted: Vec<u32>,
    pub decoded_bytes: u64,
    /// The texture pack in use: new contents are looked up in it.
    pack: Option<Box<crate::replace::Pack>>,
    /// Ids uploaded from the pack instead of the game's bytes.
    replaced: std::collections::HashSet<u32, BuildHasherDefault<FxHasher>>,
    /// Ids drawn from the game's texture while their pack file is read:
    /// the pack's key, and the texture's size.
    waiting: FastMap<u32, (crate::replace::Key, u16, u16)>,
}

/// Generations a texture may go unused before the renderer drops it.
const KEEP_GENERATIONS: u64 = 600;

impl Cache {
    pub fn alloc_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id
    }

    /// Starts a new frame: contents are hashed again on next use, textures
    /// unused for a while are dropped, and the pack's files read since are
    /// uploaded under the ids waiting for them.
    pub fn next_generation(&mut self) {
        self.generation += 1;
        self.epoch += 1;
        self.checked.clear();
        if let Some(pack) = &mut self.pack {
            let loaded = pack.take_loaded();
            if !loaded.is_empty() {
                let (new, replaced) = (&mut self.new, &mut self.replaced);
                self.waiting.retain(|&id, (at, width, height)| match loaded.iter().find(|(k, _)| k == at) {
                    Some((_, Some(image))) => {
                        new.push(NewTexture { id, width: u32::from(*width), height: u32::from(*height), levels: Vec::new(), replacement: Some(image.clone()) });
                        replaced.insert(id);
                        false
                    }
                    Some((_, None)) => false,
                    None => true,
                });
            }
        }
        if self.generation.is_multiple_of(60) {
            let g = self.generation;
            let (evicted, replaced, waiting) = (&mut self.evicted, &mut self.replaced, &mut self.waiting);
            self.by_content.retain(|_, (id, last)| {
                let keep = g - *last < KEEP_GENERATIONS;
                if !keep {
                    evicted.push(*id);
                    replaced.remove(id);
                    waiting.remove(id);
                }
                keep
            });
            self.live.retain(|_, (_, last)| g - *last < 60);
        }
    }

    /// Uses `pack` for textures from now on (`None`: the game's own). Every
    /// texture is decoded again on its next use.
    pub fn set_pack(&mut self, pack: Option<crate::replace::Pack>) {
        self.pack = pack.map(Box::new);
        self.evicted.extend(self.by_content.drain().map(|(_, (id, _))| id));
        self.checked.clear();
        self.replaced.clear();
        self.waiting.clear();
        self.epoch += 1;
    }

    /// Whether texture `id` came from the pack.
    pub fn is_replaced(&self, id: u32) -> bool {
        self.replaced.contains(&id)
    }

    /// Records an EFB copy to `addr` that the renderer keeps as texture `id`.
    /// `ram_hash` is [`hash`] of the destination's bytes.
    pub fn add_efb_copy(&mut self, addr: u32, width: u16, height: u16, ram_hash: u64) -> u32 {
        let id = self.alloc_id();
        self.epoch += 1;
        if let Some(old) = self.efb_copies.insert(addr, EfbCopy { id, width, height, ram_hash }) {
            self.evicted.push(old.id);
        }
        self.checked.retain(|k, _| k.addr != addr);
        id
    }

    /// The renderer wrote an EFB copy's texels back to RAM at `addr`: the GPU
    /// copy stays valid for those bytes.
    pub fn note_writeback(&mut self, addr: u32, ram: &[u8]) {
        self.epoch += 1;
        if let Some(c) = self.efb_copies.get_mut(&addr) {
            c.ram_hash = hash(ram);
        }
    }

    /// The texture id for `key`, decoding it from `levels` (the bytes of each
    /// level) when its contents are new.
    pub fn lookup(&mut self, key: TexKey, data: &[u8], tlut: &[u8]) -> u32 {
        if let Some(&id) = self.checked.get(&key) {
            return id;
        }
        if let Some(c) = self.efb_copies.get(&key.addr) {
            let size = level_size(key.format, key.width.into(), key.height.into()).min(data.len());
            if c.width == key.width && c.height == key.height && hash(&data[..size]) == c.ram_hash {
                let id = c.id;
                self.checked.insert(key, id);
                return id;
            }
            let old = self.efb_copies.remove(&key.addr).unwrap();
            self.evicted.push(old.id);
        }
        let mut h = hash(data);
        if is_paletted(key.format) {
            h ^= hash(tlut).rotate_left(17);
        }
        let generation = self.generation;
        let id = match self.by_content.get_mut(&(key, h)) {
            Some(e) => {
                e.1 = generation;
                e.0
            }
            None => {
                let id = self.alloc_id();
                // A pack's image instead (its key hashed once, here); while
                // its file is read, the game's own.
                let mut pack_key = None;
                let mut replacement = None;
                if let Some(pack) = &mut self.pack {
                    let k = crate::replace::key(key.format, key.width.into(), key.height.into(), data, tlut);
                    match pack.find(&k) {
                        crate::replace::Found::Ready(image) => replacement = Some(image),
                        crate::replace::Found::Loading(at) => {
                            self.waiting.insert(id, (at, key.width, key.height));
                        }
                        crate::replace::Found::None => pack_key = Some(k).filter(|_| pack.dumping()),
                    }
                }
                if replacement.is_some() {
                    self.replaced.insert(id);
                }
                let mut levels = Vec::new();
                let (mut w, mut hgt, mut at) = (usize::from(key.width), usize::from(key.height), 0);
                let decode_levels = if replacement.is_some() { 0 } else { key.levels };
                for _ in 0..decode_levels {
                    let size = level_size(key.format, w, hgt);
                    let src = data.get(at..(at + size).min(data.len())).unwrap_or(&[]);
                    levels.push(decode(key.format, w, hgt, src, tlut, key.tlut_format));
                    self.decoded_bytes += size as u64;
                    at += size;
                    if w == 1 && hgt == 1 {
                        break;
                    }
                    w = (w / 2).max(1);
                    hgt = (hgt / 2).max(1);
                }
                if let (Some(k), Some(pack), Some(rgba)) = (pack_key, &mut self.pack, levels.first()) {
                    pack.dump(&k, key.levels > 1, rgba);
                }
                self.new.push(NewTexture { id, width: key.width.into(), height: key.height.into(), levels, replacement });
                self.by_content.insert((key, h), (id, generation));
                id
            }
        };
        self.checked.insert(key, id);
        self.live.insert(key, (id, generation));
        id
    }

    /// The textures an EFB copy to `addr` patches (`w` x `h` texels in
    /// format `fmt`, rows `stride` bytes apart): each one used in this
    /// generation or the last whose RAM holds `addr`, in the same format,
    /// one level, `stride` its row of blocks, with the patch inside it.
    /// For each, its id and where the patch goes (texels).
    pub fn patch_targets(&self, addr: u32, fmt: u32, w: u32, h: u32, stride: u32) -> Vec<(u32, u32, u32)> {
        let (bw, bh, bytes) = block(fmt);
        let mut out = Vec::new();
        for (key, &(id, used)) in &self.live {
            if used + 1 < self.generation || key.levels != 1 || key.format != fmt || addr < key.addr {
                continue;
            }
            let (kw, kh) = (usize::from(key.width), usize::from(key.height));
            let per_row = kw.div_ceil(bw);
            let at = (addr - key.addr) as usize;
            if per_row * bytes != stride as usize || at >= level_size(fmt, kw, kh) || at % bytes != 0 {
                continue;
            }
            let block = at / bytes;
            let (x, y) = ((block % per_row * bw) as u32, (block / per_row * bh) as u32);
            if x + w <= u32::from(key.width) && y + h <= u32::from(key.height) {
                out.push((id, x, y));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i8_and_rgb5a3_tiles() {
        // I8 8x4 blocks: a 16x4 texture is two blocks side by side.
        let data: Vec<u8> = (0..64).collect();
        let px = decode(I8, 16, 4, &data, &[], 0);
        assert_eq!(px[(0 * 16 + 9) * 4], 33); // second block, row 0, column 1
        assert_eq!(px[(3 * 16 + 7) * 4], 31); // first block, last texel

        // RGB5A3: opaque white, then a translucent 3-bit-alpha texel.
        let mut d = vec![0u8; 32];
        d[0..2].copy_from_slice(&0xFFFFu16.to_be_bytes());
        d[2..4].copy_from_slice(&0x4F00u16.to_be_bytes()); // a=4, r=0xF, g=0, b=0
        let px = decode(RGB5A3, 4, 4, &d, &[], 0);
        assert_eq!(&px[0..4], &[255, 255, 255, 255]);
        assert_eq!(&px[4..8], &[255, 0, 0, 0x92]);
    }

    #[test]
    fn rgba8_and_cmpr() {
        let mut d = vec![0u8; 64];
        d[0] = 0x80; // A of texel 0
        d[1] = 0x10; // R
        d[32] = 0x20; // G
        d[33] = 0x30; // B
        assert_eq!(&decode(RGBA8, 4, 4, &d, &[], 0)[0..4], &[0x10, 0x20, 0x30, 0x80]);

        // CMPR, first 4x4 sub-block: c0 = red, c1 = blue (c0 > c1), texel 1 = index 1.
        let mut d = vec![0u8; 32];
        d[0..2].copy_from_slice(&0xF800u16.to_be_bytes());
        d[2..4].copy_from_slice(&0x001Fu16.to_be_bytes());
        d[4] = 0b0001_1011;
        let px = decode(CMPR, 8, 8, &d, &[], 0);
        assert_eq!(&px[0..4], &[255, 0, 0, 255]);
        assert_eq!(&px[4..8], &[0, 0, 255, 255]);
        assert_eq!(&px[8..12], &[170, 0, 85, 255]);
    }

    #[test]
    fn encode_inverts_decode() {
        let rgba: Vec<u8> = (0..16 * 8 * 4).map(|i| (i * 37 % 251) as u8).collect();
        for fmt in [I4, I8, IA4, IA8, RGB565, RGB5A3, RGBA8] {
            let once = decode(fmt, 16, 8, &encode(fmt, 16, 8, &rgba), &[], 0);
            let twice = decode(fmt, 16, 8, &encode(fmt, 16, 8, &once), &[], 0);
            assert_eq!(once, twice, "format {fmt}");
        }
        assert_eq!(&decode(RGBA8, 16, 8, &encode(RGBA8, 16, 8, &rgba), &[], 0), &rgba);
    }

    #[test]
    fn copy_patches_its_texture() {
        // A 1024x512 RGB5A3 atlas (256 blocks of 32 bytes a row) and a 32x32
        // copy into its blocks at (64, 8) texels.
        let mut c = Cache::default();
        let key = TexKey { addr: 0x10_0000, width: 1024, height: 512, format: RGB5A3, levels: 1, tlut: 0, tlut_format: 0 };
        let data = vec![0u8; level_size(RGB5A3, 1024, 512)];
        let id = c.lookup(key, &data, &[]);
        let stride = 256 * 32;
        let addr = 0x10_0000 + 2 * stride + 16 * 32;
        assert_eq!(c.patch_targets(addr, RGB5A3, 32, 32, stride), vec![(id, 64, 8)]);
        // Another format, or rows the copy does not share, patch nothing.
        assert!(c.patch_targets(addr, RGBA8, 32, 32, stride).is_empty());
        assert!(c.patch_targets(addr, RGB5A3, 32, 32, stride / 2).is_empty());
        // Its rows in RAM: eight rows of 8 blocks, a stride apart.
        let rows: Vec<_> = copy_rows(RGB5A3, 32, 32, stride as usize).collect();
        assert_eq!((rows.len(), rows[1]), (8, (stride as usize, 256)));
        assert_eq!(copy_rows(RGB5A3, 32, 32, 0).nth(1), Some((256, 256)));
    }

    #[test]
    fn cache_reuses_until_contents_change() {
        let mut c = Cache::default();
        let key = TexKey { addr: 0x100, width: 8, height: 4, format: I8, levels: 1, tlut: 0, tlut_format: 0 };
        let mut data = vec![7u8; 32];
        let a = c.lookup(key, &data, &[]);
        assert_eq!(c.lookup(key, &data, &[]), a);
        c.next_generation();
        assert_eq!(c.lookup(key, &data, &[]), a);
        data[3] = 9;
        c.next_generation();
        assert_ne!(c.lookup(key, &data, &[]), a);
        assert_eq!(c.new.len(), 2);
    }

    #[test]
    fn pack_replaces_by_contents() {
        use crate::replace::{Image, Pack};
        let mut c = Cache::default();
        let key = TexKey { addr: 0x100, width: 8, height: 4, format: I8, levels: 1, tlut: 0, tlut_format: 0 };
        let (data, other) = (vec![7u8; 32], vec![8u8; 32]);
        let old = c.lookup(key, &data, &[]);
        let mut pack = Pack::new(None);
        pack.add(crate::replace::key(I8, 8, 4, &data, &[]), std::sync::Arc::new(Image::with_mips(16, 8, vec![1; 16 * 8 * 4])));
        c.set_pack(Some(pack));
        assert_eq!(c.evicted, vec![old]);
        let id = c.lookup(key, &data, &[]);
        assert!(c.is_replaced(id) && id != old);
        let t = c.new.last().unwrap();
        assert!(t.levels.is_empty());
        assert_eq!(t.replacement.as_ref().map(|r| (r.width, r.levels.len())), Some((16, 5)));
        // Other contents at the same address: the game's own.
        c.next_generation();
        let id = c.lookup(key, &other, &[]);
        assert!(!c.is_replaced(id) && c.new.last().unwrap().replacement.is_none());
    }

    #[test]
    fn pack_files_arrive_later_under_the_same_id() {
        use crate::replace::Pack;
        let dir = std::env::temp_dir().join(format!("nsmbw-cache-pack-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let data = vec![5u8; 32];
        let name = crate::replace::name(&crate::replace::key(I8, 8, 4, &data, &[]), false);
        let mut enc = png::Encoder::new(std::fs::File::create(dir.join(format!("{name}.png"))).unwrap(), 32, 16);
        enc.set_color(png::ColorType::Rgba);
        enc.write_header().unwrap().write_image_data(&[9; 32 * 16 * 4]).unwrap();
        let mut c = Cache::default();
        let mut pack = Pack::new(None);
        assert_eq!(pack.add_folder(&dir), 1);
        c.set_pack(Some(pack));
        let key = TexKey { addr: 0x100, width: 8, height: 4, format: I8, levels: 1, tlut: 0, tlut_format: 0 };
        // First the game's texture, decoded as always.
        let id = c.lookup(key, &data, &[]);
        assert!(!c.is_replaced(id) && c.new[0].levels.len() == 1 && c.new[0].replacement.is_none());
        c.new.clear();
        // Then, once read, the pack's image under the same id.
        for _ in 0..500 {
            c.next_generation();
            if !c.new.is_empty() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(c.new.len(), 1);
        assert_eq!((c.new[0].id, c.new[0].replacement.as_ref().map(|r| r.width)), (id, Some(32)));
        assert!(c.is_replaced(id) && c.lookup(key, &data, &[]) == id);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
