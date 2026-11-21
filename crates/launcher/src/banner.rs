//! The game's own banner (`opening.bnr`, what the Wii Menu shows for the
//! disc), read from the player's game folder at run time and played on the
//! launcher's art card: the nearest thing to cover art that the game holds.
//! Nothing of it is stored.
//!
//! The banner is a layout (BRLYT) with its textures (TPL) and animations
//! (BRLAN) in a compressed archive. This is a small player for it, after
//! the game-menu layout code on branch `ui`
//! (`crates/frontend/src/ui/layout.rs`): picture panes in their hierarchy,
//! with each material's first texture tinted between its two colours, and
//! the animations' moves, fades, visibility, texture turns and texture
//! swaps. Text and window panes, which banners do not use, are left out.

use std::collections::HashMap;
use std::path::Path;

use crate::gameart::{list_u8, parse_tpl, Picture};

fn be16(p: &[u8], at: usize) -> usize {
    p.get(at..at + 2).map_or(0, |b| usize::from(u16::from_be_bytes([b[0], b[1]])))
}

fn be32(p: &[u8], at: usize) -> usize {
    p.get(at..at + 4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize)
}

fn bef(p: &[u8], at: usize) -> f32 {
    f32::from_bits(be32(p, at) as u32)
}

fn cstr(p: &[u8], at: usize, max: usize) -> String {
    let s = p.get(at..(at + max).min(p.len())).unwrap_or(&[]);
    String::from_utf8_lossy(&s[..s.iter().position(|&c| c == 0).unwrap_or(s.len())]).into_owned()
}

/// Nintendo's LZ77 (type 0x10), as the banner's inner archives are packed.
fn lz77(src: &[u8]) -> Option<Vec<u8>> {
    if src.first() != Some(&0x10) || src.len() < 4 {
        return None;
    }
    let size = usize::from(src[1]) | usize::from(src[2]) << 8 | usize::from(src[3]) << 16;
    let mut out = Vec::with_capacity(size);
    let mut pos = 4;
    while out.len() < size {
        let flags = *src.get(pos)?;
        pos += 1;
        for bit in (0..8).rev() {
            if out.len() >= size {
                break;
            }
            if flags >> bit & 1 == 0 {
                out.push(*src.get(pos)?);
                pos += 1;
                continue;
            }
            let (b1, b2) = (usize::from(*src.get(pos)?), usize::from(*src.get(pos + 1)?));
            pos += 2;
            let back = ((b1 & 0xF) << 8 | b2) + 1;
            if back > out.len() {
                return None;
            }
            for _ in 0..(b1 >> 4) + 3 {
                out.push(out[out.len() - back]);
            }
        }
    }
    Some(out)
}

#[derive(Clone, Copy)]
struct Key {
    frame: f32,
    value: f32,
    slope: f32,
}

/// One animated value: which (`target`, and for textures `index`), and its
/// keys.
struct Track {
    index: u8,
    target: u8,
    step: bool,
    keys: Vec<Key>,
}

/// The layout library's curve: Hermite between the keys either side of
/// `frame`, held flat outside them.
fn sample(track: &Track, frame: f32) -> f32 {
    let keys = &track.keys;
    let (Some(first), Some(last)) = (keys.first(), keys.last()) else { return 0.0 };
    if keys.len() == 1 || frame <= first.frame {
        return first.value;
    }
    if frame >= last.frame {
        return last.value;
    }
    let mut i = 0;
    while i + 2 < keys.len() && keys[i + 1].frame <= frame {
        i += 1;
    }
    let (k0, k1) = (keys[i], keys[i + 1]);
    if track.step || k1.frame <= k0.frame {
        return if frame >= k1.frame { k1.value } else { k0.value };
    }
    let t1 = frame - k0.frame;
    let t2 = 1.0 / (k1.frame - k0.frame);
    let a = t1 * t1 * t2;
    let b = a * t2;
    let c = t1 * b;
    let d = c * t2;
    k0.value * (2.0 * d - 3.0 * b + 1.0) + k1.value * (-2.0 * d + 3.0 * b) + k0.slope * (c - 2.0 * a + t1) + k1.slope * (c - a)
}

/// What an animation does to one pane or material.
#[derive(Default)]
struct Entry {
    /// The pane or material it applies to.
    target: usize,
    /// Position, rotation, scale, size (RLPA).
    place: Vec<Track>,
    /// Visibility (RLVI).
    visible: Vec<Track>,
    /// Vertex colours and the pane's alpha (RLVC).
    color: Vec<Track>,
    /// Texture position, rotation, scale (RLTS).
    texture_place: Vec<Track>,
    /// Which texture (RLTP): the value indexes `Animation::textures`.
    texture: Vec<Track>,
}

#[derive(Default)]
struct Animation {
    frames: f32,
    /// The texture of each name the animation swaps in, per material.
    textures: Vec<String>,
    panes: Vec<Entry>,
    materials: Vec<Entry>,
}

struct Material {
    name: String,
    /// The layout's own colours: a texel runs from `black` to `white`.
    black: [f32; 4],
    white: [f32; 4],
    /// Whether that tint is applied (materials with their own colour mixing
    /// and an opaque `black` are drawn plain).
    tinted: bool,
    /// The first texture's name, and its picture in `Banner::pictures`.
    texture_name: Option<String>,
    texture: Option<usize>,
    /// Pictures of other textures, tinted as this material tints.
    swaps: HashMap<String, usize>,
    /// The first texture's translation, rotation (degrees) and scale.
    srt: [f32; 5],
}

/// What animations change of a pane.
#[derive(Clone, Copy)]
struct PaneState {
    visible: bool,
    alpha: f32,
    translate: [f32; 2],
    rotate: f32,
    scale: [f32; 2],
    size: [f32; 2],
    /// Corner colours (RGBA): top left, top right, bottom left, bottom right.
    colors: [[f32; 4]; 4],
}

struct Pane {
    name: String,
    picture: bool,
    /// Left out by the version's settings, whatever the animations say.
    hidden: bool,
    /// Its alpha also fades its children.
    passes_alpha: bool,
    origin: u8,
    material: Option<usize>,
    /// Texture coordinates of the corners, in the colours' order.
    uv: [[f32; 2]; 4],
    children: Vec<usize>,
    /// As the layout has it, before any animation.
    state: PaneState,
}

/// A textured, coloured quad ready to draw: corners in the banner's own
/// units (x right, y up, from its middle), in the order top left, top
/// right, bottom right, bottom left.
pub struct Quad {
    /// Index into [`Banner::pictures`]; `None` is a plain colour.
    pub picture: Option<usize>,
    pub corners: [[f32; 2]; 4],
    pub uv: [[f32; 2]; 4],
    pub colors: [[u8; 4]; 4],
}

pub struct Banner {
    /// The banner's width and height in its own units.
    pub size: [f32; 2],
    pub pictures: Vec<Picture>,
    materials: Vec<Material>,
    panes: Vec<Pane>,
    /// The opening, played once, and the loop that follows.
    start: Option<Animation>,
    repeat: Option<Animation>,
}

/// The files of the banner's own archive (`meta/banner.bin` inside
/// `opening.bnr`): an IMET header, a U8 archive, an IMD5 header, LZ77.
fn unpack(bnr: &[u8]) -> Option<Vec<u8>> {
    let start = [0x600, 0x640, 0].into_iter().find(|&at| bnr.get(at..at + 4) == Some(&[0x55, 0xAA, 0x38, 0x2D]))?;
    let outer = &bnr[start..];
    let (_, offset, size) = list_u8(outer).into_iter().find(|f| f.0 == "banner.bin")?;
    let mut inner = outer.get(offset..offset.checked_add(size)?)?;
    if inner.starts_with(b"IMD5") {
        inner = inner.get(0x20..)?;
    }
    if inner.starts_with(b"LZ77") {
        return lz77(inner.get(4..)?);
    }
    Some(inner.to_vec())
}

fn animation(lan: &[u8], panes: &[Pane], materials: &[Material]) -> Option<Animation> {
    if lan.len() < 16 || &lan[..4] != b"RLAN" {
        return None;
    }
    let mut out = Animation::default();
    let mut pos = be16(lan, 12);
    for _ in 0..be16(lan, 14) {
        let size = be32(lan, pos + 4);
        if size < 8 || pos + size > lan.len() {
            break;
        }
        if &lan[pos..pos + 4] == b"pai1" {
            let b = pos + 8;
            out.frames = be16(lan, b) as f32;
            out.textures = (0..be16(lan, b + 4)).map(|i| cstr(lan, b + 12 + be32(lan, b + 12 + 4 * i), 64)).collect();
            let table = pos + be32(lan, b + 8);
            for e in 0..be16(lan, b + 6) {
                let eo = pos + be32(lan, table + e * 4);
                let (name, of_material) = (cstr(lan, eo, 20), lan.get(eo + 21).copied().unwrap_or(0) != 0);
                let target = if of_material { materials.iter().position(|m| m.name == name) } else { panes.iter().position(|p| p.name == name) };
                let Some(target) = target else { continue };
                let mut entry = Entry { target, ..Entry::default() };
                for t in 0..usize::from(lan.get(eo + 20).copied().unwrap_or(0)) {
                    let to = eo + be32(lan, eo + 24 + t * 4);
                    let into = match lan.get(to..to + 4) {
                        Some(b"RLPA") => &mut entry.place,
                        Some(b"RLVI") => &mut entry.visible,
                        Some(b"RLVC") => &mut entry.color,
                        Some(b"RLTS") => &mut entry.texture_place,
                        Some(b"RLTP") => &mut entry.texture,
                        _ => continue,
                    };
                    for k in 0..usize::from(lan.get(to + 4).copied().unwrap_or(0)) {
                        let io = to + be32(lan, to + 8 + k * 4);
                        if io + 12 > lan.len() {
                            break;
                        }
                        let hermite = lan[io + 2] == 2;
                        let (keys_at, stride) = (io + be32(lan, io + 8), if hermite { 12 } else { 8 });
                        let keys = (0..be16(lan, io + 4)).map(|n| keys_at + n * stride).filter(|k| k + stride <= lan.len()).map(|k| if hermite { Key { frame: bef(lan, k), value: bef(lan, k + 4), slope: bef(lan, k + 8) } } else { Key { frame: bef(lan, k), value: be16(lan, k + 4) as f32, slope: 0.0 } }).collect();
                        into.push(Track { index: lan[io], target: lan[io + 1], step: !hermite, keys });
                    }
                }
                if of_material { &mut out.materials } else { &mut out.panes }.push(entry);
            }
        }
        pos += size;
    }
    (out.frames > 0.0).then_some(out)
}

/// A texture as this material shows it: tinted between its two colours.
fn tint(source: &Picture, material: &Material) -> Picture {
    let mut rgba = source.rgba[..source.width * source.height * 4].to_vec();
    if material.tinted {
        for px in rgba.chunks_exact_mut(4) {
            for k in 0..4 {
                let c = f32::from(px[k]) / 255.0;
                px[k] = ((material.black[k] + (material.white[k] - material.black[k]) * c).clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        }
    }
    Picture { width: source.width, height: source.height, rgba }
}

/// Reads the banner in `file` of the game folder `game`. Panes named in
/// `hide` are left out, with their children.
pub fn load(game: &Path, file: &str, hide: &[String]) -> Option<Banner> {
    let archive = unpack(&std::fs::read(game.join(file)).ok()?)?;
    let files = list_u8(&archive);
    let named = |name: &str| files.iter().find(|f| f.0 == name).and_then(|f| archive.get(f.1..f.1.checked_add(f.2)?));
    let lyt = files.iter().find(|f| f.0.ends_with(".brlyt")).and_then(|f| archive.get(f.1..f.1.checked_add(f.2)?))?;
    if lyt.len() < 16 || &lyt[..4] != b"RLYT" {
        return None;
    }
    let mut banner = Banner { size: [608.0, 456.0], pictures: Vec::new(), materials: Vec::new(), panes: Vec::new(), start: None, repeat: None };
    let mut texture_names: Vec<String> = Vec::new();
    let (mut stack, mut last): (Vec<usize>, Option<usize>) = (Vec::new(), None);
    // While inside a hidden pane, how deep.
    let mut hidden: Option<usize> = None;
    let mut depth = 0usize;
    let mut pos = be16(lyt, 12);
    for _ in 0..be16(lyt, 14) {
        let size = be32(lyt, pos + 4);
        if size < 8 || pos + size > lyt.len() {
            break;
        }
        let sec = &lyt[pos..pos + size];
        match &sec[..4] {
            b"lyt1" => banner.size = [bef(sec, 12), bef(sec, 16)],
            b"txl1" => texture_names = (0..be16(sec, 8)).map(|i| cstr(sec, 12 + be32(sec, 12 + i * 8), 64)).collect(),
            b"mat1" => {
                for i in 0..be16(sec, 8) {
                    let m = be32(sec, 12 + i * 4);
                    let color = |c: usize| -> [f32; 4] { std::array::from_fn(|k| (be16(sec, m + 20 + c * 8 + k * 2) as u16 as i16) as f32 / 255.0) };
                    let flags = be32(sec, m + 60);
                    let (textures, placements, own_mixing) = (flags & 15, flags >> 4 & 15, flags >> 18 & 31);
                    let (black, white) = (color(0), color(1));
                    let srt_at = m + 64 + textures * 4;
                    banner.materials.push(Material {
                        name: cstr(sec, m, 20),
                        black,
                        white,
                        tinted: (own_mixing == 0 || black[3] == 0.0) && (black != [0.0; 4] || white.iter().any(|&w| w < 1.0)),
                        texture_name: (textures > 0).then(|| texture_names.get(be16(sec, m + 64)).cloned()).flatten(),
                        texture: None,
                        swaps: HashMap::new(),
                        srt: if placements > 0 { std::array::from_fn(|k| bef(sec, srt_at + k * 4)) } else { [0.0, 0.0, 0.0, 1.0, 1.0] },
                    });
                }
            }
            b"pan1" | b"pic1" | b"txt1" | b"wnd1" | b"bnd1" if sec.len() >= 76 => {
                let name = cstr(sec, 12, 16);
                if hidden.is_none() && hide.iter().any(|h| *h == name) {
                    hidden = Some(depth);
                }
                let picture = &sec[..4] == b"pic1";
                let content = 76;
                let mut pane = Pane {
                    name,
                    picture,
                    hidden: hidden.is_some(),
                    passes_alpha: sec[8] & 2 != 0,
                    origin: sec[9],
                    material: None,
                    uv: [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                    children: Vec::new(),
                    state: PaneState { visible: sec[8] & 1 != 0, alpha: f32::from(sec[10]) / 255.0, translate: [bef(sec, 36), bef(sec, 40)], rotate: bef(sec, 56), scale: [bef(sec, 60), bef(sec, 64)], size: [bef(sec, 68), bef(sec, 72)], colors: [[1.0; 4]; 4] },
                };
                if picture && sec.len() >= content + 20 {
                    pane.state.colors = std::array::from_fn(|c| std::array::from_fn(|k| f32::from(sec[content + c * 4 + k]) / 255.0));
                    pane.material = Some(be16(sec, content + 16)).filter(|&i| i < banner.materials.len());
                    if sec[content + 18] > 0 && sec.len() >= content + 52 {
                        pane.uv = std::array::from_fn(|c| [bef(sec, content + 20 + c * 8), bef(sec, content + 24 + c * 8)]);
                    }
                }
                let index = banner.panes.len();
                if let Some(&parent) = stack.last() {
                    banner.panes[parent].children.push(index);
                }
                banner.panes.push(pane);
                last = Some(index);
            }
            b"pas1" => {
                stack.extend(last);
                depth += 1;
            }
            b"pae1" => {
                last = stack.pop();
                depth = depth.saturating_sub(1);
                if hidden.is_some_and(|h| depth <= h) {
                    hidden = None;
                }
            }
            _ => {}
        }
        // A hidden pane without children ends with its own section.
        if hidden == Some(depth) && !matches!(&sec[..4], b"pas1") && lyt.get(pos + size..pos + size + 4) != Some(b"pas1") {
            hidden = None;
        }
        pos += size;
    }
    if banner.panes.is_empty() {
        return None;
    }

    banner.start = files.iter().find(|f| f.0.ends_with("_start.brlan")).and_then(|f| animation(archive.get(f.1..f.1 + f.2)?, &banner.panes, &banner.materials));
    banner.repeat = files.iter().find(|f| f.0.ends_with("_loop.brlan")).or_else(|| files.iter().find(|f| f.0.ends_with(".brlan") && !f.0.ends_with("_start.brlan"))).and_then(|f| animation(archive.get(f.1..f.1 + f.2)?, &banner.panes, &banner.materials));

    // Each material's picture, and those its animations swap in.
    let mut sources: HashMap<String, Option<Picture>> = HashMap::new();
    let mut source = |name: &str| -> Option<Picture> {
        let found = sources.entry(name.to_string()).or_insert_with(|| named(name).and_then(parse_tpl));
        found.as_ref().map(|p| Picture { width: p.width, height: p.height, rgba: p.rgba.clone() })
    };
    for m in 0..banner.materials.len() {
        let mut wanted: Vec<String> = banner.materials[m].texture_name.iter().cloned().collect();
        for anim in [&banner.start, &banner.repeat].into_iter().flatten() {
            if anim.materials.iter().any(|e| e.target == m && !e.texture.is_empty()) {
                wanted.extend(anim.textures.iter().cloned());
            }
        }
        for name in wanted {
            if banner.materials[m].swaps.contains_key(&name) {
                continue;
            }
            if let Some(picture) = source(&name) {
                banner.pictures.push(tint(&picture, &banner.materials[m]));
                banner.materials[m].swaps.insert(name, banner.pictures.len() - 1);
            }
        }
        banner.materials[m].texture = banner.materials[m].texture_name.as_ref().and_then(|n| banner.materials[m].swaps.get(n).copied());
    }
    Some(banner)
}

/// 2-D transform: x' = a x + b y + tx, y' = c x + d y + ty.
#[derive(Clone, Copy)]
struct Mtx {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    tx: f32,
    ty: f32,
}

impl Mtx {
    const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 };

    /// `self` then a move, a turn (degrees) and a scale.
    fn compose(&self, t: [f32; 2], degrees: f32, s: [f32; 2]) -> Self {
        let (sn, cs) = degrees.to_radians().sin_cos();
        let l = Self { a: cs * s[0], b: -sn * s[1], c: sn * s[0], d: cs * s[1], tx: t[0], ty: t[1] };
        Self { a: self.a * l.a + self.b * l.c, b: self.a * l.b + self.b * l.d, c: self.c * l.a + self.d * l.c, d: self.c * l.b + self.d * l.d, tx: self.a * l.tx + self.b * l.ty + self.tx, ty: self.c * l.tx + self.d * l.ty + self.ty }
    }

    fn apply(&self, x: f32, y: f32) -> [f32; 2] {
        [self.a * x + self.b * y + self.tx, self.c * x + self.d * y + self.ty]
    }
}

impl Banner {
    /// The banner's quads, back to front, `seconds` after it first showed:
    /// its opening once, then its loop.
    pub fn quads(&self, seconds: f32) -> Vec<Quad> {
        // Only what animations change is copied: a few numbers per pane.
        let mut frame = seconds.max(0.0) * 60.0;
        let animation = match (&self.start, &self.repeat) {
            (Some(start), _) if frame < start.frames => Some(start),
            (start, Some(repeat)) => {
                frame = (frame - start.as_ref().map_or(0.0, |s| s.frames)).rem_euclid(repeat.frames);
                Some(repeat)
            }
            (Some(start), None) => {
                frame = start.frames;
                Some(start)
            }
            (None, None) => None,
        };
        let mut panes: Vec<PaneState> = self.panes.iter().map(|p| p.state).collect();
        let mut materials: Vec<(Option<usize>, [f32; 5])> = self.materials.iter().map(|m| (m.texture, m.srt)).collect();
        if let Some(anim) = animation {
            for entry in &anim.panes {
                let pane = &mut panes[entry.target];
                for track in &entry.place {
                    let v = sample(track, frame);
                    match track.target {
                        0 => pane.translate[0] = v,
                        1 => pane.translate[1] = v,
                        5 => pane.rotate = v,
                        6 => pane.scale[0] = v,
                        7 => pane.scale[1] = v,
                        8 => pane.size[0] = v,
                        9 => pane.size[1] = v,
                        _ => {}
                    }
                }
                for track in entry.visible.iter().filter(|t| t.target == 0) {
                    pane.visible = sample(track, frame) != 0.0;
                }
                for track in &entry.color {
                    let v = sample(track, frame).clamp(0.0, 255.0) / 255.0;
                    match track.target {
                        16 => pane.alpha = v,
                        t if t < 16 => pane.colors[usize::from(t / 4)][usize::from(t % 4)] = v,
                        _ => {}
                    }
                }
            }
            for entry in &anim.materials {
                let material = &mut materials[entry.target];
                for track in entry.texture_place.iter().filter(|t| t.index == 0 && t.target < 5) {
                    material.1[usize::from(track.target)] = sample(track, frame);
                }
                for track in entry.texture.iter().filter(|t| t.index == 0 && t.target == 0) {
                    let name = anim.textures.get(sample(track, frame) as usize);
                    if let Some(&picture) = name.and_then(|n| self.materials[entry.target].swaps.get(n)) {
                        material.0 = Some(picture);
                    }
                }
            }
        }
        let mut quads = Vec::new();
        if !panes.is_empty() {
            self.pane(&panes, &materials, 0, &Mtx::IDENTITY, 1.0, &mut quads);
        }
        quads
    }

    fn pane(&self, panes: &[PaneState], materials: &[(Option<usize>, [f32; 5])], index: usize, parent: &Mtx, parent_alpha: f32, quads: &mut Vec<Quad>) {
        let (fixed, pane) = (&self.panes[index], &panes[index]);
        if !pane.visible || fixed.hidden {
            return;
        }
        let alpha = pane.alpha * parent_alpha;
        let m = parent.compose(pane.translate, pane.rotate, pane.scale);
        if fixed.picture && alpha > 0.002 {
            let [w, h] = pane.size;
            let x0 = [0.0, -w / 2.0, -w][usize::from(fixed.origin % 3).min(2)];
            let y0 = [0.0, h / 2.0, h][usize::from(fixed.origin / 3).min(2)];
            let material = fixed.material.map(|i| (&self.materials[i], materials[i]));
            let mut uv = fixed.uv;
            if let Some((_, (_, srt))) = material {
                let [tx, ty, turn, sx, sy] = srt;
                if (tx, ty, turn, sx, sy) != (0.0, 0.0, 0.0, 1.0, 1.0) {
                    let (sn, cs) = turn.to_radians().sin_cos();
                    for c in &mut uv {
                        let (u, v) = ((c[0] - 0.5) * sx, (c[1] - 0.5) * sy);
                        *c = [u * cs - v * sn + 0.5 + tx, u * sn + v * cs + 0.5 - ty];
                    }
                }
            }
            let picture = material.and_then(|(_, (texture, _))| texture);
            let colors = std::array::from_fn(|i| {
                // Top left, top right, bottom right, bottom left.
                let mut c = pane.colors[[0, 1, 3, 2][i]];
                if let Some((mat, _)) = material.filter(|_| picture.is_none()) {
                    for k in 0..4 {
                        c[k] *= mat.white[k].clamp(0.0, 1.0);
                    }
                }
                c[3] *= alpha;
                c.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
            });
            quads.push(Quad { picture, corners: [m.apply(x0, y0), m.apply(x0 + w, y0), m.apply(x0 + w, y0 - h), m.apply(x0, y0 - h)], uv: [uv[0], uv[1], uv[3], uv[2]], colors });
        }
        let passed = if fixed.passes_alpha { alpha } else { 1.0 };
        for &child in &fixed.children {
            self.pane(panes, materials, child, &m, passed, quads);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_data_unpacks() {
        // "abcabcabc": three bytes, then six copied from three back.
        let packed = [0x10, 9, 0, 0, 0b0001_0000, b'a', b'b', b'c', 0x30, 0x02];
        assert_eq!(lz77(&packed).unwrap(), b"abcabcabc");
        assert!(lz77(&[0x11, 1, 0, 0, 0, b'a']).is_none());
        assert!(lz77(&[0x10, 4, 0, 0, 0x80, 0x00, 0x05]).is_none());
    }

    #[test]
    fn curves_run_between_their_keys() {
        let line = Track { index: 0, target: 0, step: false, keys: vec![Key { frame: 0.0, value: 0.0, slope: 1.0 }, Key { frame: 10.0, value: 10.0, slope: 1.0 }] };
        assert_eq!(sample(&line, -5.0), 0.0);
        assert!((sample(&line, 4.0) - 4.0).abs() < 1e-4);
        assert_eq!(sample(&line, 99.0), 10.0);
        let steps = Track { index: 0, target: 0, step: true, keys: vec![Key { frame: 0.0, value: 2.0, slope: 0.0 }, Key { frame: 5.0, value: 7.0, slope: 0.0 }] };
        assert_eq!(sample(&steps, 4.9), 2.0);
        assert_eq!(sample(&steps, 5.0), 7.0);
    }

    #[test]
    fn a_pane_sits_where_its_parents_put_it() {
        let pane = |name: &str, picture: bool, translate: [f32; 2], children: Vec<usize>| Pane { name: name.into(), picture, hidden: false, passes_alpha: true, origin: 4, material: None, uv: [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]], children, state: PaneState { visible: true, alpha: 0.5, translate, rotate: 0.0, scale: [2.0, 2.0], size: [10.0, 4.0], colors: [[1.0; 4]; 4] } };
        let banner = Banner { size: [608.0, 456.0], pictures: Vec::new(), materials: Vec::new(), panes: vec![pane("root", false, [100.0, 0.0], vec![1]), pane("child", true, [5.0, 5.0], vec![])], start: None, repeat: None };
        let quads = banner.quads(0.0);
        assert_eq!(quads.len(), 1);
        // The child's middle: 100 + 2 * 5; its half-width 2 * 2 * 10 / 2.
        assert_eq!(quads[0].corners[0], [90.0, 18.0]);
        assert_eq!(quads[0].corners[2], [130.0, 2.0]);
        // Its alpha is its own times its parent's.
        assert_eq!(quads[0].colors[0], [255, 255, 255, 64]);
        assert!(load(Path::new("no such folder"), "files/opening.bnr", &[]).is_none());
    }
}
