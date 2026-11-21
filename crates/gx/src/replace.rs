//! Texture packs in Dolphin's format, after WitherBlitz's NSMB-Wii-Recompilation
//! (`aurora-main/lib/gfx/texture_replacement.cpp`): PNG or DDS files named
//! `tex1_{W}x{H}[_m]_{hash}[_{palette hash}]_{format}` stand in for the game's
//! textures with those contents. Render-only: guest RAM never sees them, so
//! each machine in a net game chooses for itself.
//!
//! - The texture hash is XXH64 (seed 0) of level 0's bytes as they are in RAM.
//! - Paletted formats (C4, C8, C14X2) also name their palette: XXH64 of the
//!   palette's entries from the lowest index level 0 uses to the highest.
//! - `$` for either hash matches any; `_m` is ignored and a trailing `_arb`
//!   accepted.
//! - A file's mip levels are files beside it named `<name>_mip1`, `_mip2`...;
//!   without them, a chain is made by halving, so a large image drawn small
//!   does not shimmer.
//!
//! A file is read on the pack's own threads the first time its texture is
//! drawn, so drawing never waits for one: the game's texture stands in until
//! [`Pack::take_loaded`] hands the image over (a field or two). Read images
//! are kept, up to [`BUDGET`] bytes, the least recently used going first.
//! [`Pack::add`] puts in an image from memory instead, ready at once: the
//! hook for the port's own textures, such as button prompts for the
//! controller in use.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::texture::{self, C4, C8};

// ---------------------------------------------------------------- XXH64

const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;
const P5: u64 = 0x27D4_EB2F_1656_67C5;

fn round(acc: u64, lane: u64) -> u64 {
    acc.wrapping_add(lane.wrapping_mul(P2)).rotate_left(31).wrapping_mul(P1)
}

fn merge(acc: u64, v: u64) -> u64 {
    (acc ^ round(0, v)).wrapping_mul(P1).wrapping_add(P4)
}

fn le64(b: &[u8]) -> u64 {
    u64::from_le_bytes(b[..8].try_into().unwrap())
}

/// XXH64 of `data` (what Dolphin names textures by).
pub fn xxh64(data: &[u8], seed: u64) -> u64 {
    let mut rest = data;
    let mut h = if data.len() >= 32 {
        let mut v = [seed.wrapping_add(P1).wrapping_add(P2), seed.wrapping_add(P2), seed, seed.wrapping_sub(P1)];
        let mut stripes = data.chunks_exact(32);
        for s in &mut stripes {
            for (i, v) in v.iter_mut().enumerate() {
                *v = round(*v, le64(&s[8 * i..]));
            }
        }
        rest = stripes.remainder();
        let h = v[0].rotate_left(1).wrapping_add(v[1].rotate_left(7)).wrapping_add(v[2].rotate_left(12)).wrapping_add(v[3].rotate_left(18));
        v.iter().fold(h, |h, &v| merge(h, v))
    } else {
        seed.wrapping_add(P5)
    };
    h = h.wrapping_add(data.len() as u64);
    let mut words = rest.chunks_exact(8);
    for w in &mut words {
        h = (h ^ round(0, le64(w))).rotate_left(27).wrapping_mul(P1).wrapping_add(P4);
    }
    let mut rest = words.remainder();
    if rest.len() >= 4 {
        let w = u64::from(u32::from_le_bytes(rest[..4].try_into().unwrap()));
        h = (h ^ w.wrapping_mul(P1)).rotate_left(23).wrapping_mul(P2).wrapping_add(P3);
        rest = &rest[4..];
    }
    for &b in rest {
        h = (h ^ u64::from(b).wrapping_mul(P5)).rotate_left(11).wrapping_mul(P1);
    }
    h ^= h >> 33;
    h = h.wrapping_mul(P2);
    h ^= h >> 29;
    h = h.wrapping_mul(P3);
    h ^ (h >> 32)
}

// ---------------------------------------------------------------- names

/// `$`: either hash, any value.
pub const ANY: u64 = u64::MAX;

/// What a pack's file name says about the texture it replaces.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Key {
    pub tex: u64,
    /// The palette's hash (paletted formats only).
    pub tlut: Option<u64>,
    pub width: u32,
    pub height: u32,
    /// GX texture format (0 I4 .. 14 CMPR).
    pub format: u32,
}

/// The key of a `width` x `height` texture in `format`, from level 0's bytes
/// and, for a paletted format, its palette as loaded in TMEM.
pub fn key(format: u32, width: u32, height: u32, level0: &[u8], tlut: &[u8]) -> Key {
    let size = texture::level_size(format, width as usize, height as usize).min(level0.len());
    let data = &level0[..size];
    let tlut = texture::is_paletted(format).then(|| palette_hash(format, data, tlut));
    Key { tex: xxh64(data, 0), tlut, width, height, format }
}

/// XXH64 of the palette entries from the lowest index `data` uses to the
/// highest (Dolphin's palette hash).
fn palette_hash(format: u32, data: &[u8], tlut: &[u8]) -> u64 {
    let (mut lo, mut hi) = (usize::MAX, 0);
    let mut see = |i: usize| {
        lo = lo.min(i);
        hi = hi.max(i);
    };
    match format {
        C4 => data.iter().for_each(|&b| {
            see(usize::from(b >> 4));
            see(usize::from(b & 0xF));
        }),
        C8 => data.iter().for_each(|&b| see(usize::from(b))),
        _ => data.chunks_exact(2).for_each(|c| see(usize::from(u16::from_be_bytes([c[0], c[1]]) & 0x3FFF))),
    }
    let range = if lo <= hi { tlut.get(2 * lo..2 * (hi + 1)).unwrap_or(&[]) } else { &[] };
    xxh64(range, 0)
}

/// The file name (without extension) Dolphin gives the texture of `key`;
/// `mipmapped` adds its `_m`.
pub fn name(key: &Key, mipmapped: bool) -> String {
    let (w, h, m) = (key.width, key.height, if mipmapped { "_m" } else { "" });
    match key.tlut {
        Some(t) => format!("tex1_{w}x{h}{m}_{:016x}_{t:016x}_{}", key.tex, key.format),
        None => format!("tex1_{w}x{h}{m}_{:016x}_{}", key.tex, key.format),
    }
}

/// The key a pack file's name stands for, or `None` if it is not one
/// (including a `_mipN` level).
pub fn parse_name(file: &str) -> Option<Key> {
    let (stem, ext) = file.rsplit_once('.')?;
    if !ext.eq_ignore_ascii_case("png") && !ext.eq_ignore_ascii_case("dds") {
        return None;
    }
    let mut parts: Vec<&str> = stem.strip_prefix("tex1_")?.split('_').collect();
    if parts.last() == Some(&"arb") {
        parts.pop();
    }
    let (dims, mut rest) = parts.split_first()?;
    if rest.first() == Some(&"m") {
        rest = &rest[1..];
    }
    let (w, h) = dims.split_once('x')?;
    let hash = |s: &str| match s {
        "$" => Some(ANY),
        _ if (1..=16).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_hexdigit()) => u64::from_str_radix(s, 16).ok(),
        _ => None,
    };
    let (tex, tlut, format) = match rest {
        [t, f] => (hash(t)?, None, f),
        [t, p, f] => (hash(t)?, Some(hash(p)?), f),
        _ => return None,
    };
    let number = |s: &str| s.bytes().all(|b| b.is_ascii_digit()).then(|| s.parse().ok()).flatten();
    Some(Key { tex, tlut, width: number(w)?, height: number(h)?, format: number(format)? })
}

// ---------------------------------------------------------------- images

/// A replacement texture: RGBA8 levels, largest first, each half the last.
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub levels: Vec<Vec<u8>>,
}

impl Image {
    pub fn bytes(&self) -> usize {
        self.levels.iter().map(Vec::len).sum()
    }

    /// `rgba` (`width` x `height`), with its mip chain made by halving down
    /// to 1 x 1.
    pub fn with_mips(width: u32, height: u32, rgba: Vec<u8>) -> Self {
        let mut levels = vec![rgba];
        let (mut w, mut h) = (width as usize, height as usize);
        while w > 1 || h > 1 {
            let (next, nw, nh) = half(levels.last().unwrap(), w, h);
            levels.push(next);
            (w, h) = (nw, nh);
        }
        Self { width, height, levels }
    }
}

/// `src` (`w` x `h` RGBA8) halved: each texel the average of up to four,
/// colours weighted by their alpha so transparent texels don't darken edges.
fn half(src: &[u8], w: usize, h: usize) -> (Vec<u8>, usize, usize) {
    let (nw, nh) = ((w / 2).max(1), (h / 2).max(1));
    let mut out = vec![0u8; nw * nh * 4];
    for y in 0..nh {
        for x in 0..nw {
            let mut sum = [0u32; 4];
            let mut plain = [0u32; 3];
            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let (sx, sy) = ((2 * x + dx).min(w - 1), (2 * y + dy).min(h - 1));
                let p = &src[(sy * w + sx) * 4..][..4];
                let a = u32::from(p[3]);
                for c in 0..3 {
                    sum[c] += u32::from(p[c]) * a;
                    plain[c] += u32::from(p[c]);
                }
                sum[3] += a;
            }
            let o = &mut out[(y * nw + x) * 4..][..4];
            for c in 0..3 {
                o[c] = if sum[3] > 0 { ((sum[c] + sum[3] / 2) / sum[3]) as u8 } else { ((plain[c] + 2) / 4) as u8 };
            }
            o[3] = ((sum[3] + 2) / 4) as u8;
        }
    }
    (out, nw, nh)
}

/// A PNG's texels as RGBA8.
fn read_png(bytes: Vec<u8>) -> Result<(u32, u32, Vec<u8>), String> {
    let mut d = png::Decoder::new_with_limits(std::io::Cursor::new(bytes), png::Limits { bytes: 1 << 30 });
    d.set_transformations(png::Transformations::normalize_to_color8());
    let mut r = d.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; r.output_buffer_size().ok_or("too large")?];
    let info = r.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let px = info.width as usize * info.height as usize;
    let rgba = match info.color_type {
        png::ColorType::Rgba => {
            buf.truncate(px * 4);
            buf
        }
        png::ColorType::Rgb => buf.chunks_exact(3).take(px).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => buf.chunks_exact(2).take(px).flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        png::ColorType::Grayscale => buf.iter().take(px).flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return Err("palette not expanded".into()),
    };
    Ok((info.width, info.height, rgba))
}

#[derive(Clone, Copy, PartialEq)]
enum Dds {
    Rgba,
    Bgra,
    Bgrx,
    Bc1,
    Bc2,
    Bc3,
}

fn rgb565(v: u16) -> [u32; 3] {
    let x = |v: u16, bits: u32| -> u32 {
        let v = u32::from(v);
        (v << (8 - bits)) | (v >> (2 * bits - 8))
    };
    [x(v >> 11, 5), x((v >> 5) & 0x3F, 6), x(v & 0x1F, 5)]
}

/// One 4x4 BC1 colour block into `out` (16 RGBA texels); `alpha_mode`:
/// whether c0 <= c1 means three colours and transparent black (BC1 only).
fn bc1_block(b: &[u8], out: &mut [[u8; 4]; 16], alpha_mode: bool) {
    let (c0, c1) = (u16::from_le_bytes([b[0], b[1]]), u16::from_le_bytes([b[2], b[3]]));
    let (a, z) = (rgb565(c0), rgb565(c1));
    let mix = |p: u32, q: u32, d: u32| -> [u8; 4] {
        let m = |i: usize| ((a[i] * p + z[i] * q) / d) as u8;
        [m(0), m(1), m(2), 255]
    };
    let colors = if c0 > c1 || !alpha_mode { [mix(1, 0, 1), mix(0, 1, 1), mix(2, 1, 3), mix(1, 2, 3)] } else { [mix(1, 0, 1), mix(0, 1, 1), mix(1, 1, 2), [0, 0, 0, 0]] };
    let bits = u32::from_le_bytes(b[4..8].try_into().unwrap());
    for (i, t) in out.iter_mut().enumerate() {
        *t = colors[((bits >> (2 * i)) & 3) as usize];
    }
}

/// One mip level of a DDS file in `format` to RGBA8.
fn dds_level(format: Dds, w: usize, h: usize, data: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; w * h * 4];
    if matches!(format, Dds::Rgba | Dds::Bgra | Dds::Bgrx) {
        for (o, p) in out.chunks_exact_mut(4).zip(data.chunks_exact(4)) {
            o.copy_from_slice(&match format {
                Dds::Rgba => [p[0], p[1], p[2], p[3]],
                Dds::Bgra => [p[2], p[1], p[0], p[3]],
                _ => [p[2], p[1], p[0], 255],
            });
        }
        return out;
    }
    let block = if format == Dds::Bc1 { 8 } else { 16 };
    let bw = w.div_ceil(4);
    for (i, b) in data.chunks_exact(block).take(bw * h.div_ceil(4)).enumerate() {
        let mut texels = [[0u8; 4]; 16];
        match format {
            Dds::Bc1 => bc1_block(b, &mut texels, true),
            Dds::Bc2 => {
                bc1_block(&b[8..], &mut texels, false);
                let bits = u64::from_le_bytes(b[..8].try_into().unwrap());
                for (j, t) in texels.iter_mut().enumerate() {
                    t[3] = ((bits >> (4 * j)) & 0xF) as u8 * 17;
                }
            }
            _ => {
                bc1_block(&b[8..], &mut texels, false);
                let (a0, a1) = (u32::from(b[0]), u32::from(b[1]));
                let alphas: [u32; 8] = if a0 > a1 {
                    [a0, a1, (6 * a0 + a1) / 7, (5 * a0 + 2 * a1) / 7, (4 * a0 + 3 * a1) / 7, (3 * a0 + 4 * a1) / 7, (2 * a0 + 5 * a1) / 7, (a0 + 6 * a1) / 7]
                } else {
                    [a0, a1, (4 * a0 + a1) / 5, (3 * a0 + 2 * a1) / 5, (2 * a0 + 3 * a1) / 5, (a0 + 4 * a1) / 5, 0, 255]
                };
                let mut bits = [0u8; 8];
                bits[..6].copy_from_slice(&b[2..8]);
                let bits = u64::from_le_bytes(bits);
                for (j, t) in texels.iter_mut().enumerate() {
                    t[3] = alphas[((bits >> (3 * j)) & 7) as usize] as u8;
                }
            }
        }
        let (x0, y0) = (i % bw * 4, i / bw * 4);
        for (j, t) in texels.iter().enumerate() {
            let (x, y) = (x0 + j % 4, y0 + j / 4);
            if x < w && y < h {
                out[(y * w + x) * 4..][..4].copy_from_slice(t);
            }
        }
    }
    out
}

/// A DDS file's levels as RGBA8: uncompressed 32-bit, or BC1 to BC3.
fn read_dds(b: &[u8]) -> Result<Image, String> {
    if b.len() < 128 || &b[..4] != b"DDS " {
        return Err("not a DDS file".into());
    }
    let u = |at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap());
    let (height, width) = (u(12), u(16));
    let count = if u(8) & 0x2_0000 != 0 { u(28).max(1) } else { 1 };
    let (flags, fourcc) = (u(80), &b[84..88]);
    let (format, mut at) = match fourcc {
        b"DXT1" => (Dds::Bc1, 128),
        b"DXT2" | b"DXT3" => (Dds::Bc2, 128),
        b"DXT4" | b"DXT5" => (Dds::Bc3, 128),
        b"DX10" if b.len() >= 148 => (
            match u(128) {
                28 | 29 => Dds::Rgba,
                87 | 91 => Dds::Bgra,
                88 | 93 => Dds::Bgrx,
                71 | 72 => Dds::Bc1,
                74 | 75 => Dds::Bc2,
                77 | 78 => Dds::Bc3,
                98 | 99 => return Err("BC7 is not supported (use the pack's PNG edition)".into()),
                n => return Err(format!("DXGI format {n} is not supported")),
            },
            148,
        ),
        _ if flags & 0x40 != 0 && u(88) == 32 => match (u(92), flags & 1 != 0) {
            (0xFF, _) => (Dds::Rgba, 128),
            (0xFF_0000, true) => (Dds::Bgra, 128),
            (0xFF_0000, false) => (Dds::Bgrx, 128),
            _ => return Err("unsupported pixel layout".into()),
        },
        _ => return Err("unsupported pixel format".into()),
    };
    let mut levels = Vec::new();
    let (mut w, mut h) = (width as usize, height as usize);
    for _ in 0..count {
        let size = match format {
            Dds::Bc1 => w.div_ceil(4) * h.div_ceil(4) * 8,
            Dds::Bc2 | Dds::Bc3 => w.div_ceil(4) * h.div_ceil(4) * 16,
            _ => w * h * 4,
        };
        let Some(data) = b.get(at..at + size) else { break };
        levels.push(dds_level(format, w, h, data));
        at += size;
        if w == 1 && h == 1 {
            break;
        }
        (w, h) = ((w / 2).max(1), (h / 2).max(1));
    }
    if levels.is_empty() {
        return Err("too short".into());
    }
    Ok(Image { width, height, levels })
}

/// A pack file with its levels: the file's own, else `_mipN` files beside
/// it, else a chain made by halving.
fn load(path: &Path) -> Result<Image, String> {
    let read = |p: &Path| -> Result<Image, String> {
        let bytes = std::fs::read(p).map_err(|e| e.to_string())?;
        if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("dds")) {
            read_dds(&bytes)
        } else {
            read_png(bytes).map(|(width, height, rgba)| Image { width, height, levels: vec![rgba] })
        }
    };
    let mut image = read(path)?;
    if image.width == 0 || image.height == 0 {
        return Err("empty image".into());
    }
    if image.levels.len() == 1 {
        let (stem, ext) = (path.file_stem().unwrap_or_default().to_string_lossy(), path.extension().unwrap_or_default().to_string_lossy());
        for n in 1.. {
            let mip = path.with_file_name(format!("{stem}_mip{n}.{ext}"));
            if !mip.is_file() {
                break;
            }
            let (w, h) = ((image.width >> n).max(1), (image.height >> n).max(1));
            match read(&mip) {
                Ok(l) if (l.width, l.height) == (w, h) => image.levels.push(l.levels.into_iter().next().unwrap_or_default()),
                Ok(l) => return Err(format!("{}: {}x{}, not {w}x{h}", mip.display(), l.width, l.height)),
                Err(e) => return Err(format!("{}: {e}", mip.display())),
            }
            if w == 1 && h == 1 {
                break;
            }
        }
    }
    if image.levels.len() == 1 {
        let rgba = image.levels.pop().unwrap();
        image = Image::with_mips(image.width, image.height, rgba);
    }
    Ok(image)
}

// ---------------------------------------------------------------- the pack

/// Decoded files kept for reuse, at most this many bytes (Bighead's pack
/// is about 575 MB decoded, with mips; the GPU keeps what is drawn).
pub const BUDGET: usize = 256 << 20;

/// What a pack did; the counters only grow.
#[derive(Default, Debug)]
pub struct Stats {
    /// Distinct textures that had a replacement.
    pub matched: AtomicU64,
    /// Distinct textures that had none.
    pub missed: AtomicU64,
    /// Files that could not be read.
    pub failed: AtomicU64,
    /// Textures written to the dump folder.
    pub dumped: AtomicU64,
}

enum Source {
    File(PathBuf),
    Image(Arc<Image>),
}

/// What [`Pack::find`] found.
pub enum Found {
    Ready(Arc<Image>),
    /// Its file is being read: [`Pack::take_loaded`] hands it over under
    /// this key.
    Loading(Key),
    None,
}

/// Threads reading the pack's files.
const LOADERS: usize = 2;

type Loaded = (Key, Result<Image, String>);

struct Loader {
    jobs: Sender<(Key, PathBuf)>,
    done: Receiver<Loaded>,
}

impl Loader {
    /// The threads end when the pack (and with it `jobs`) is dropped.
    fn start() -> Self {
        let (jobs, queue) = channel::<(Key, PathBuf)>();
        let (finished, done) = channel::<Loaded>();
        let queue = Arc::new(Mutex::new(queue));
        for _ in 0..LOADERS {
            let (queue, finished) = (queue.clone(), finished.clone());
            std::thread::Builder::new()
                .name("texture pack".into())
                .spawn(move || loop {
                    let job = queue.lock().unwrap_or_else(|e| e.into_inner()).recv();
                    let Ok((key, path)) = job else { return };
                    let r = load(&path).map_err(|e| format!("{}: {e}", path.display()));
                    if finished.send((key, r)).is_err() {
                        return;
                    }
                })
                .expect("texture pack thread");
        }
        Self { jobs, done }
    }
}

/// The replacements in use: a pack's files, images added from memory, or
/// none at all with dumping on.
pub struct Pack {
    entries: HashMap<Key, Source>,
    /// Files read so far, with when each was last asked for.
    loaded: HashMap<Key, (Arc<Image>, u64)>,
    loaded_bytes: usize,
    clock: u64,
    failed: HashSet<Key>,
    seen: HashSet<Key>,
    /// Textures without a replacement are written here as PNGs.
    dumps: Option<PathBuf>,
    dumped: HashSet<Key>,
    loader: Option<Loader>,
    /// Files being read.
    loading: HashSet<Key>,
    pub stats: Arc<Stats>,
}

/// More distinct textures than this (a texture the game redraws every
/// frame makes one a frame) and the counting starts again.
const SEEN_LIMIT: usize = 100_000;

impl Pack {
    /// No replacements; `dumps`: write textures without one to this folder.
    pub fn new(dumps: Option<PathBuf>) -> Self {
        Self { entries: HashMap::new(), loaded: HashMap::new(), loaded_bytes: 0, clock: 0, failed: HashSet::new(), seen: HashSet::new(), dumps, dumped: HashSet::new(), loader: None, loading: HashSet::new(), stats: Arc::default() }
    }

    /// Adds every pack file under `folder` (and its subfolders); returns how
    /// many. A name already added keeps its first file.
    pub fn add_folder(&mut self, folder: &Path) -> usize {
        let mut count = 0;
        let mut dirs = vec![folder.to_path_buf()];
        while let Some(dir) = dirs.pop() {
            let Ok(list) = std::fs::read_dir(&dir) else { continue };
            let mut list: Vec<_> = list.flatten().collect();
            list.sort_by_key(|e| e.file_name());
            for e in list {
                let path = e.path();
                if e.file_type().is_ok_and(|t| t.is_dir()) {
                    dirs.push(path);
                } else if let Some(key) = parse_name(&e.file_name().to_string_lossy()) {
                    if let std::collections::hash_map::Entry::Vacant(v) = self.entries.entry(key) {
                        v.insert(Source::File(path));
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Uses `image` for the texture of `key` (whose hashes may be [`ANY`]),
    /// ahead of any file for it.
    pub fn add(&mut self, key: Key, image: Arc<Image>) {
        self.entries.insert(key, Source::Image(image));
        self.loaded.remove(&key);
        self.failed.remove(&key);
    }

    /// Replacements known (files and images).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn dumping(&self) -> bool {
        self.dumps.is_some()
    }

    /// The entry for `key`: exact, else any palette, else any texture.
    fn entry(&self, key: &Key) -> Option<Key> {
        let any_tlut = key.tlut.map(|_| Key { tlut: Some(ANY), ..*key });
        [Some(*key), any_tlut, Some(Key { tex: ANY, ..*key })].into_iter().flatten().find(|k| self.entries.contains_key(k))
    }

    /// The replacement for the texture of `key`: ready, or being read (the
    /// first time its file is asked for), or none.
    pub fn find(&mut self, key: &Key) -> Found {
        if self.seen.len() >= SEEN_LIMIT {
            self.seen.clear();
        }
        let first = self.seen.insert(*key);
        let Some(at) = self.entry(key).filter(|at| !self.failed.contains(at)) else {
            if first {
                self.stats.missed.fetch_add(1, Ordering::Relaxed);
            }
            return Found::None;
        };
        if first {
            self.stats.matched.fetch_add(1, Ordering::Relaxed);
        }
        self.clock += 1;
        let path = match &self.entries[&at] {
            Source::Image(image) => return Found::Ready(image.clone()),
            Source::File(path) => path.clone(),
        };
        if let Some((image, used)) = self.loaded.get_mut(&at) {
            *used = self.clock;
            return Found::Ready(image.clone());
        }
        if self.loading.insert(at) {
            let _ = self.loader.get_or_insert_with(Loader::start).jobs.send((at, path));
        }
        Found::Loading(at)
    }

    /// The files read since the last call, by the key [`Found::Loading`]
    /// gave: each one's image, or `None` if it could not be read.
    pub fn take_loaded(&mut self) -> Vec<(Key, Option<Arc<Image>>)> {
        let Some(loader) = &self.loader else { return Vec::new() };
        let mut out = Vec::new();
        while let Ok((at, r)) = loader.done.try_recv() {
            self.loading.remove(&at);
            match r {
                Ok(image) => {
                    let image = Arc::new(image);
                    self.loaded_bytes += image.bytes();
                    self.loaded.insert(at, (image.clone(), self.clock));
                    while self.loaded_bytes > BUDGET && self.loaded.len() > 1 {
                        let oldest = *self.loaded.iter().min_by_key(|(_, (_, used))| *used).unwrap().0;
                        let (gone, _) = self.loaded.remove(&oldest).unwrap();
                        self.loaded_bytes -= gone.bytes();
                    }
                    out.push((at, Some(image)));
                }
                Err(e) => {
                    eprintln!("texture pack: {e}");
                    self.failed.insert(at);
                    self.stats.failed.fetch_add(1, Ordering::Relaxed);
                    out.push((at, None));
                }
            }
        }
        out
    }

    /// With dumping on, writes the texture of `key` (level 0 as `rgba`,
    /// `width` x `height`) to the dump folder under its pack name, once.
    pub fn dump(&mut self, key: &Key, mipmapped: bool, rgba: &[u8]) {
        let Some(folder) = &self.dumps else { return };
        if !self.dumped.insert(*key) {
            return;
        }
        let path = folder.join(format!("{}.png", name(key, mipmapped)));
        if path.exists() {
            return;
        }
        let write = || -> Result<(), String> {
            std::fs::create_dir_all(folder).map_err(|e| e.to_string())?;
            let file = std::io::BufWriter::new(std::fs::File::create(&path).map_err(|e| e.to_string())?);
            let mut enc = png::Encoder::new(file, key.width, key.height);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            enc.write_header().and_then(|mut w| w.write_image_data(rgba)).map_err(|e| e.to_string())
        };
        match write() {
            Ok(()) => {
                self.stats.dumped.fetch_add(1, Ordering::Relaxed);
            }
            Err(e) => eprintln!("texture dump: {}: {e}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xxh64_vectors() {
        assert_eq!(xxh64(b"", 0), 0xEF46_DB37_51D8_E999);
        assert_eq!(xxh64(b"a", 0), 0xD24E_C4F1_A98C_6E5B);
        assert_eq!(xxh64(b"abc", 0), 0x44BC_2CF5_AD77_0999);
        assert_eq!(xxh64(b"Nobody inspects the spammish repetition", 0), 0xFBCE_A83C_8A37_8BF1);
        assert_eq!(xxh64(b"xxhash", 20_141_025), 0xB559_B98D_844E_0635);
        // Every length through stripes, words, a half word and bytes, against the reference crate.
        let data: Vec<u8> = (0..300u32).map(|i| (i.wrapping_mul(2_654_435_761) >> 13) as u8).collect();
        for n in 0..data.len() {
            for seed in [0, 1, 0x9E37_79B9_7F4A_7C15] {
                assert_eq!(xxh64(&data[..n], seed), xxhash_rust::xxh64::xxh64(&data[..n], seed), "length {n} seed {seed:#x}");
            }
        }
    }

    #[test]
    fn names_round_trip() {
        let k = Key { tex: 0x0123_4567_89AB_CDEF, tlut: None, width: 64, height: 32, format: 14 };
        assert_eq!(name(&k, true), "tex1_64x32_m_0123456789abcdef_14");
        assert_eq!(parse_name("tex1_64x32_m_0123456789abcdef_14.png"), Some(k));
        assert_eq!(parse_name("tex1_64x32_0123456789ABCDEF_14.DDS"), Some(k));
        assert_eq!(parse_name("tex1_64x32_0123456789abcdef_14_arb.png"), Some(k));
        let p = Key { tlut: Some(0xFEDC), format: 9, ..k };
        assert_eq!(name(&p, false), "tex1_64x32_0123456789abcdef_000000000000fedc_9");
        assert_eq!(parse_name(&format!("{}.png", name(&p, false))), Some(p));
        assert_eq!(parse_name("tex1_64x32_$_000000000000fedc_9.png"), Some(Key { tex: ANY, ..p }));
        assert_eq!(parse_name("tex1_64x32_0123456789abcdef_$_9.png"), Some(Key { tlut: Some(ANY), ..p }));
        for bad in ["tex1_64x32_0123456789abcdef_14_mip1.png", "tex1_64x32_0123456789abcdef_14.jpg", "tex1_64_0123_14.png", "tex2_64x32_0123_14.png", "tex1_64x32_xyz_14.png", "tex1_64x32_0123456789abcdef0_14.png"] {
            assert_eq!(parse_name(bad), None, "{bad}");
        }
    }

    #[test]
    fn palette_hash_covers_the_indices_used() {
        // C8 8x4 using indices 2..5: the hash is of entries 2..=5 only.
        let mut data = vec![3u8; 32];
        data[0] = 2;
        data[31] = 5;
        let tlut: Vec<u8> = (0..512).map(|i| i as u8).collect();
        let k = key(C8, 8, 4, &data, &tlut);
        assert_eq!(k.tex, xxh64(&data, 0));
        assert_eq!(k.tlut, Some(xxh64(&tlut[4..12], 0)));
        // C4: both nibbles count.
        let k = key(C4, 8, 8, &[0x71; 32], &tlut);
        assert_eq!(k.tlut, Some(xxh64(&tlut[2..16], 0)));
        assert_eq!(key(texture::RGB565, 4, 4, &[0; 32], &tlut).tlut, None);
    }

    fn ready(f: Found) -> Option<Arc<Image>> {
        match f {
            Found::Ready(i) => Some(i),
            _ => None,
        }
    }

    /// Waits for the pack's threads to read what was asked for.
    fn wait(p: &mut Pack) -> Vec<(Key, Option<Arc<Image>>)> {
        for _ in 0..500 {
            let got = p.take_loaded();
            if !got.is_empty() {
                return got;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        panic!("nothing was read");
    }

    #[test]
    fn lookups_try_wildcards() {
        let mut p = Pack::new(None);
        let k = Key { tex: 1, tlut: Some(2), width: 8, height: 8, format: 9 };
        let img = |n: u8| Arc::new(Image { width: 1, height: 1, levels: vec![vec![n; 4]] });
        p.add(Key { tex: ANY, ..k }, img(1));
        assert_eq!(ready(p.find(&k)).unwrap().levels[0][0], 1);
        p.add(Key { tlut: Some(ANY), ..k }, img(2));
        assert_eq!(ready(p.find(&k)).unwrap().levels[0][0], 2);
        p.add(k, img(3));
        assert_eq!(ready(p.find(&k)).unwrap().levels[0][0], 3);
        assert!(matches!(p.find(&Key { width: 16, ..k }), Found::None));
        assert_eq!(p.stats.matched.load(Ordering::Relaxed), 1);
        assert_eq!(p.stats.missed.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn mips_halve_and_keep_edges() {
        // 2x2: one opaque red texel, three transparent black ones.
        let rgba = vec![255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let img = Image::with_mips(2, 2, rgba);
        assert_eq!(img.levels.len(), 2);
        assert_eq!(img.levels[1], vec![255, 0, 0, 64]);
        assert_eq!(Image::with_mips(5, 3, vec![0; 60]).levels.iter().map(Vec::len).collect::<Vec<_>>(), vec![60, 8, 4]);
    }

    #[test]
    fn files_load_with_their_mips() {
        let dir = std::env::temp_dir().join(format!("nsmbw-pack-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        let write_png = |path: &Path, w: u32, h: u32, v: u8| {
            let mut enc = png::Encoder::new(std::fs::File::create(path).unwrap(), w, h);
            enc.set_color(png::ColorType::Rgb);
            enc.set_depth(png::BitDepth::Eight);
            enc.write_header().unwrap().write_image_data(&vec![v; (w * h * 3) as usize]).unwrap();
        };
        let k = Key { tex: 0xAB, tlut: None, width: 4, height: 4, format: 5 };
        let base = dir.join("sub").join(format!("{}.png", name(&k, false)));
        write_png(&base, 16, 8, 10);
        write_png(&dir.join("sub").join(format!("{}_mip1.png", name(&k, false))), 8, 4, 20);
        // A plain BGRA DDS for another texture.
        let d = Key { tex: 0xCD, ..k };
        let mut dds = vec![0u8; 128];
        dds[..4].copy_from_slice(b"DDS ");
        // 2x2, RGB with alpha, 32 bits, red in the third byte (BGRA).
        for (at, v) in [(12, 2u32), (16, 2), (80, 0x41), (88, 32), (92, 0xFF_0000)] {
            dds[at..at + 4].copy_from_slice(&v.to_le_bytes());
        }
        dds.extend([1, 2, 3, 4].repeat(4));
        std::fs::write(dir.join(format!("{}.dds", name(&d, false))), &dds).unwrap();
        let mut p = Pack::new(Some(dir.join("dumps")));
        assert_eq!(p.add_folder(&dir), 2);
        // Read on the pack's threads, then ready at once.
        assert!(matches!(p.find(&k), Found::Loading(at) if at == k));
        let got = wait(&mut p);
        assert_eq!(got.len(), 1);
        let img = got[0].1.clone().unwrap();
        assert_eq!((img.width, img.height, img.levels.len()), (16, 8, 2));
        assert_eq!((img.levels[0][0], img.levels[0][3], img.levels[1][0]), (10, 255, 20));
        assert!(ready(p.find(&k)).is_some());
        assert!(matches!(p.find(&d), Found::Loading(_)));
        let img = wait(&mut p)[0].1.clone().unwrap();
        assert_eq!((img.levels.len(), &img.levels[0][..4]), (2, &[3, 2, 1, 4][..]));
        // A file that can't be read: none from then on.
        let bad = Key { tex: 0x12, ..k };
        std::fs::write(dir.join(format!("{}.png", name(&bad, false))), b"not a png").unwrap();
        assert_eq!(p.add_folder(&dir), 1);
        assert!(matches!(p.find(&bad), Found::Loading(_)));
        assert!(wait(&mut p)[0].1.is_none());
        assert!(matches!(p.find(&bad), Found::None));
        assert_eq!(p.stats.failed.load(Ordering::Relaxed), 1);
        // A miss is dumped once, under its name.
        let miss = Key { tex: 0xEF, ..k };
        assert!(matches!(p.find(&miss), Found::None));
        p.dump(&miss, false, &[7; 64]);
        p.dump(&miss, false, &[7; 64]);
        assert!(dir.join("dumps").join(format!("{}.png", name(&miss, false))).is_file());
        assert_eq!(p.stats.dumped.load(Ordering::Relaxed), 1);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn bc1_and_bc3_blocks() {
        // BC1: c0 white > c1 black, texel 0 index 0, texel 1 index 1, texel 2 index 2.
        let b = [0xFF, 0xFF, 0, 0, 0b10_01_00, 0, 0, 0];
        let px = dds_level(Dds::Bc1, 4, 4, &b);
        assert_eq!((&px[0..4], &px[4..8], &px[8..12]), (&[255, 255, 255, 255][..], &[0, 0, 0, 255][..], &[170, 170, 170, 255][..]));
        // BC3: alpha 255 and 0, texel 0 index 1 (0), texel 1 index 0 (255).
        let mut b = [0u8; 16];
        (b[0], b[1], b[2]) = (255, 0, 0b000_001);
        b[8..16].copy_from_slice(&[0xFF, 0xFF, 0, 0, 0, 0, 0, 0]);
        let px = dds_level(Dds::Bc3, 4, 4, &b);
        assert_eq!((px[3], px[7]), (0, 255));
    }
}
