//! The game's own course background, read from the player's game folder at
//! run time and drawn live behind the launcher: the background models the
//! game itself draws (`files/Object/bgA_*.arc`, `bgB_*.arc`), named per
//! version in `versions.toml`. Nothing of the game is stored.
//!
//! A background model (BRRES: an MDL0 model and its TEX0 textures) is a set
//! of flat sheets and shallow shapes at different depths, seen straight on.
//! Here each is turned into triangles with a texture: the model's vertex
//! arrays and display lists give the triangles, and each material's colour
//! mixing (its TEV stages) is worked out once per texture instead of per
//! pixel, with the vertex colours multiplied in when drawing. The sheets
//! under a model's "Scroll" bones drift at different speeds, as in a course.

use std::path::Path;

use crate::gameart::Picture;

fn be16(p: &[u8], at: usize) -> usize {
    p.get(at..at + 2).map_or(0, |b| usize::from(u16::from_be_bytes([b[0], b[1]])))
}

fn be16s(p: &[u8], at: usize) -> i32 {
    be16(p, at) as u16 as i16 as i32
}

fn be32(p: &[u8], at: usize) -> usize {
    p.get(at..at + 4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize)
}

/// A signed offset from `at`.
fn rel(p: &[u8], base: usize, at: usize) -> usize {
    (base as i64 + i64::from(be32(p, at) as u32 as i32)).max(0) as usize
}

fn bef(p: &[u8], at: usize) -> f32 {
    f32::from_bits(be32(p, at) as u32)
}

fn cstr(p: &[u8], at: usize) -> String {
    let s = p.get(at..).unwrap_or(&[]);
    String::from_utf8_lossy(&s[..s.iter().position(|&c| c == 0).unwrap_or(s.len()).min(128)]).into_owned()
}

/// The entries of a BRRES index group: name and where it points.
fn group(p: &[u8], at: usize) -> Vec<(String, usize)> {
    if at == 0 || at + 8 > p.len() {
        return Vec::new();
    }
    let count = be32(p, at + 4).min(4096);
    (1..=count).map(|i| at + 8 + i * 16).map(|e| (cstr(p, at + be32(p, e + 8)), at + be32(p, e + 12))).collect()
}

/// One sheet of a background: triangles with one texture.
pub struct Piece {
    pub texture: Picture,
    /// The texture repeats past its edges.
    pub repeat: bool,
    /// Model x and y (y up) of each vertex.
    pub positions: Vec<[f32; 2]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[u8; 4]>,
    pub indices: Vec<u32>,
    /// Mean depth: larger is nearer the viewer.
    pub depth: f32,
    /// Which of the model's scroll groups it belongs to: 0 is the nearest.
    pub group: u8,
}

/// A background model's sheets, far to near.
pub struct Model {
    pub pieces: Vec<Piece>,
    /// The model repeats sideways every this many units.
    pub period: f32,
}

struct Layer {
    /// TEX0 name.
    name: String,
    /// Whether it repeats past its edges, sideways and upwards.
    repeat: [bool; 2],
}

struct Material {
    xlu: bool,
    stages: usize,
    cull: usize,
    layers: Vec<Layer>,
    /// Colour registers PREV, C0, C1, C2 and the four constants, as RGBA.
    registers: [[f32; 4]; 4],
    konst: [[f32; 4]; 4],
    /// BP registers the shader and material set (TEV stages, alpha test, blending).
    bp: Box<[u32; 256]>,
}

/// Applies the BP register writes in `p[from..to]`, with their masks.
fn run_bp(p: &[u8], from: usize, to: usize, bp: &mut [u32; 256], mut each: impl FnMut(usize, u32)) {
    let mut mask = 0x00FF_FFFF;
    let mut i = from;
    while i + 5 <= to.min(p.len()) {
        if p[i] != 0x61 {
            i += 1;
            continue;
        }
        let (reg, value) = (usize::from(p[i + 1]), (be32(p, i + 1) & 0x00FF_FFFF) as u32);
        if reg == 0xFE {
            mask = value;
        } else {
            bp[reg] = (bp[reg] & !mask) | (value & mask);
            each(reg, value);
            mask = 0x00FF_FFFF;
        }
        i += 5;
    }
}

fn material(p: &[u8], m: usize) -> Material {
    let mut bp = Box::new([0u32; 256]);
    let mut registers = [[0.0f32; 4]; 4];
    let mut konst = [[0.0f32; 4]; 4];
    let signed = |v: u32| ((v & 0x7FF) as i32 - if v & 0x400 != 0 { 0x800 } else { 0 }) as f32 / 255.0;
    // The material's own display list: alpha test, blending, TEV colours.
    let (dl, size) = (m + be32(p, m + 0x3C), be32(p, m));
    run_bp(p, dl, m + size, &mut bp, |reg, value| {
        if (0xE0..0xE8).contains(&reg) {
            let (index, high, constant) = ((reg - 0xE0) / 2, reg & 1 == 1, value >> 23 != 0);
            let slot = if constant { &mut konst[index] } else { &mut registers[index] };
            if high {
                slot[2] = signed(value);
                slot[1] = signed(value >> 12);
            } else {
                slot[0] = signed(value);
                slot[3] = signed(value >> 12);
            }
        }
    });
    // The shader: which texture and constants each TEV stage uses, and how.
    let shader = rel(p, m, m + 0x28);
    run_bp(p, shader + 0x20, shader + be32(p, shader), &mut bp, |_, _| {});
    let layers_at = m + be32(p, m + 0x30);
    let layers = (0..be32(p, m + 0x2C).min(8)).map(|i| layers_at + i * 0x34).map(|l| Layer { name: cstr(p, l + be32(p, l)), repeat: [be32(p, l + 0x18) != 0, be32(p, l + 0x1C) != 0] }).collect();
    Material { xlu: be32(p, m + 0x10) >> 31 != 0, stages: usize::from(p.get(m + 0x16).copied().unwrap_or(0)).min(16), cull: be32(p, m + 0x18), layers, registers, konst, bp }
}

/// A TEX0 texture as RGBA.
fn texture(p: &[u8], t: usize) -> Option<Picture> {
    let (width, height, format) = (be16(p, t + 0x1C), be16(p, t + 0x1E), be32(p, t + 0x20));
    if width == 0 || height == 0 || width > 2048 || height > 2048 || !matches!(format, 0..=6 | 14) {
        return None;
    }
    let rgba = gx::texture::decode(format as u32, width, height, p.get(t + be32(p, t + 0x10)..)?, &[], 0);
    (rgba.len() >= width * height * 4).then_some(Picture { width, height, rgba })
}

fn compare(function: u32, value: f32, reference: f32) -> bool {
    match function & 7 {
        0 => false,
        1 => value < reference,
        2 => value == reference,
        3 => value <= reference,
        4 => value > reference,
        5 => value != reference,
        6 => value >= reference,
        _ => true,
    }
}

/// Works the material's TEV stages out for every texel of its main texture
/// (the largest it uses). Its other textures count as one colour each, and
/// the vertex colour as white: that is multiplied in when drawing. Returns the picture and which texture coordinate it uses.
fn bake(mat: &Material, textures: &[Option<Picture>]) -> Option<(Picture, usize)> {
    let main = (0..textures.len()).filter(|&i| textures[i].is_some()).max_by_key(|&i| textures[i].as_ref().map_or(0, |t| t.width * t.height))?;
    let average: Vec<[f32; 4]> = textures
        .iter()
        .enumerate()
        .map(|(i, t)| match (t, mat.layers.get(i).map(|l| l.name.as_str())) {
            // The engine's light maps: stand-ins in the file, which the game
            // replaces with the course's lighting as it runs. Here the first
            // (multiplied in) counts as full light and the others (added
            // shine) as none.
            (_, Some(name)) if name.starts_with("lm_") => [if name.starts_with("lm_01") { 1.0 } else { 0.0 }; 4],
            (Some(t), _) => {
                let n = (t.width * t.height) as f32 * 255.0;
                std::array::from_fn(|c| t.rgba.chunks_exact(4).map(|px| f32::from(px[c])).sum::<f32>() / n)
            }
            (None, _) => [1.0; 4],
        })
        .collect();
    // Each stage's texture map, texture coordinate and constant selectors.
    let stage = |s: usize| {
        let (tref, ksel) = (mat.bp[0x28 + s / 2] >> (12 * (s & 1)), mat.bp[0xF6 + s / 2] >> (10 * (s & 1)));
        let map = if tref >> 6 & 1 != 0 { Some((tref & 7) as usize) } else { None };
        (map, (tref >> 3 & 7) as usize, ksel >> 4 & 31, ksel >> 9 & 31)
    };
    let coord = (0..mat.stages).map(stage).find(|s| s.0 == Some(main)).map_or(0, |s| s.1);
    let konst_color = |sel: u32| -> [f32; 3] {
        match sel {
            0..=7 => [1.0 - sel as f32 / 8.0; 3],
            12..=15 => {
                let k = mat.konst[sel as usize - 12];
                [k[0], k[1], k[2]]
            }
            16..=31 => [mat.konst[sel as usize & 3][(sel as usize - 16) / 4]; 3],
            _ => [0.0; 3],
        }
    };
    let konst_alpha = |sel: u32| match sel {
        0..=7 => 1.0 - sel as f32 / 8.0,
        16..=31 => mat.konst[sel as usize & 3][(sel as usize - 16) / 4],
        _ => 0.0,
    };
    let finish = |value: f32, env: u32| {
        let biased = value + [0.0, 0.5, -0.5, 0.0][(env >> 16 & 3) as usize];
        let scaled = biased * [1.0, 2.0, 4.0, 0.5][(env >> 20 & 3) as usize];
        if env >> 19 & 1 != 0 {
            scaled.clamp(0.0, 1.0)
        } else {
            scaled.clamp(-4.0, 4.0)
        }
    };
    let alpha_test = mat.bp[0xF3];
    let blended = mat.bp[0x41] & 1 != 0;

    let source = textures[main].as_ref()?;
    let mut rgba = Vec::with_capacity(source.rgba.len());
    for px in source.rgba.chunks_exact(4) {
        let texel: [f32; 4] = std::array::from_fn(|c| f32::from(px[c]) / 255.0);
        let mut reg = mat.registers;
        for s in 0..mat.stages {
            let (map, _, kc, ka) = stage(s);
            let tex = match map {
                Some(m) if m == main => texel,
                Some(m) => average.get(m).copied().unwrap_or([1.0; 4]),
                None => [1.0; 4],
            };
            let (color_env, alpha_env) = (mat.bp[0xC0 + 2 * s], mat.bp[0xC1 + 2 * s]);
            let color = |sel: u32| -> [f32; 3] {
                let rgb = |r: [f32; 4]| [r[0], r[1], r[2]];
                match sel & 15 {
                    0 => rgb(reg[0]),
                    1 => [reg[0][3]; 3],
                    2 => rgb(reg[1]),
                    3 => [reg[1][3]; 3],
                    4 => rgb(reg[2]),
                    5 => [reg[2][3]; 3],
                    6 => rgb(reg[3]),
                    7 => [reg[3][3]; 3],
                    8 => rgb(tex),
                    9 => [tex[3]; 3],
                    10 | 11 | 12 => [1.0; 3],
                    13 => [0.5; 3],
                    14 => konst_color(kc),
                    _ => [0.0; 3],
                }
            };
            let alpha = |sel: u32| match sel & 7 {
                0 => reg[0][3],
                1 => reg[1][3],
                2 => reg[2][3],
                3 => reg[3][3],
                4 => tex[3],
                5 => 1.0,
                6 => konst_alpha(ka),
                _ => 0.0,
            };
            let (a, b, c, d) = (color(color_env >> 12), color(color_env >> 8), color(color_env >> 4), color(color_env));
            let sign = if color_env >> 18 & 1 != 0 { -1.0 } else { 1.0 };
            let out: [f32; 3] = std::array::from_fn(|i| finish(d[i] + sign * (a[i] + (b[i] - a[i]) * c[i]), color_env));
            let (a, b, c, d) = (alpha(alpha_env >> 13), alpha(alpha_env >> 10), alpha(alpha_env >> 7), alpha(alpha_env >> 4));
            let sign = if alpha_env >> 18 & 1 != 0 { -1.0 } else { 1.0 };
            let out_alpha = finish(d + sign * (a + (b - a) * c), alpha_env);
            let dest = &mut reg[(color_env >> 22 & 3) as usize];
            dest[..3].copy_from_slice(&out);
            reg[(alpha_env >> 22 & 3) as usize][3] = out_alpha;
        }
        let out = reg[0];
        let (t0, t1) = (compare(alpha_test >> 16, out[3], (alpha_test & 0xFF) as f32 / 255.0), compare(alpha_test >> 19, out[3], (alpha_test >> 8 & 0xFF) as f32 / 255.0));
        let pass = match alpha_test >> 22 & 3 {
            0 => t0 && t1,
            1 => t0 || t1,
            2 => t0 != t1,
            _ => t0 == t1,
        };
        let alpha = if !pass {
            0.0
        } else if mat.xlu || blended {
            out[3].clamp(0.0, 1.0)
        } else {
            1.0
        };
        rgba.extend(out[..3].iter().map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8));
        rgba.push((alpha * 255.0).round() as u8);
    }
    Some((Picture { width: source.width, height: source.height, rgba }, coord))
}

/// A vertex array's values: `components` numbers each, in model units.
fn array(p: &[u8], a: usize, components: usize) -> Vec<Vec<f32>> {
    let (format, divisor, stride, count) = (be32(p, a + 0x18), p.get(a + 0x1C).copied().unwrap_or(0), usize::from(p.get(a + 0x1D).copied().unwrap_or(0)), be16(p, a + 0x1E));
    let data = a + be32(p, a + 8);
    let scale = 1.0 / (1u32 << divisor.min(31)) as f32;
    let width = [1, 1, 2, 2, 4][format.min(4)];
    (0..count)
        .map(|i| {
            (0..components)
                .map(|c| {
                    let at = data + i * stride + c * width;
                    match format {
                        0 => f32::from(p.get(at).copied().unwrap_or(0)) * scale,
                        1 => f32::from(p.get(at).copied().unwrap_or(0) as i8) * scale,
                        2 => be16(p, at) as f32 * scale,
                        3 => be16s(p, at) as f32 * scale,
                        _ => bef(p, at),
                    }
                })
                .collect()
        })
        .collect()
}

fn colors(p: &[u8], a: usize) -> Vec<[u8; 4]> {
    let (format, stride, count) = (be32(p, a + 0x18), usize::from(p.get(a + 0x1C).copied().unwrap_or(0)), be16(p, a + 0x1E));
    let data = a + be32(p, a + 8);
    (0..count)
        .map(|i| {
            let b = |k: usize| p.get(data + i * stride + k).copied().unwrap_or(255);
            match format {
                1 | 2 => [b(0), b(1), b(2), 255],
                5 => [b(0), b(1), b(2), b(3)],
                _ => [255; 4],
            }
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Corner {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

/// The part of the polygon whose texture coordinate `axis` is at least
/// `edge` (`above`) or at most it.
fn cut(polygon: &[Corner], axis: usize, edge: f32, above: bool) -> Vec<Corner> {
    let inside = |c: &Corner| if above { c.uv[axis] >= edge } else { c.uv[axis] <= edge };
    let mut out = Vec::with_capacity(polygon.len() + 1);
    for (i, a) in polygon.iter().enumerate() {
        let b = &polygon[(i + 1) % polygon.len()];
        if inside(a) {
            out.push(*a);
        }
        if inside(a) != inside(b) {
            let t = (edge - a.uv[axis]) / (b.uv[axis] - a.uv[axis]);
            let mix = |x: f32, y: f32| x + (y - x) * t;
            out.push(Corner { pos: [mix(a.pos[0], b.pos[0]), mix(a.pos[1], b.pos[1])], uv: [mix(a.uv[0], b.uv[0]), mix(a.uv[1], b.uv[1])], color: std::array::from_fn(|k| mix(a.color[k], b.color[k])) });
        }
    }
    out
}

/// Cuts the piece's triangles where texture coordinate `axis` leaves 0..1
/// and holds it at the edge beyond, as a texture that does not repeat that
/// way is drawn.
fn hold_at_edges(piece: &mut Piece, axis: usize) {
    let size = if axis == 0 { piece.texture.width } else { piece.texture.height };
    let inset = 0.5 / size.max(1) as f32;
    let (mut positions, mut uvs, mut colors, mut indices) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for triangle in piece.indices.chunks_exact(3) {
        let corners: Vec<Corner> = triangle.iter().map(|&i| Corner { pos: piece.positions[i as usize], uv: piece.uvs[i as usize], color: piece.colors[i as usize].map(f32::from) }).collect();
        let parts = [(cut(&cut(&corners, axis, 0.0, true), axis, 1.0, false), None), (cut(&corners, axis, 0.0, false), Some(inset)), (cut(&corners, axis, 1.0, true), Some(1.0 - inset))];
        for (polygon, held) in parts {
            if polygon.len() < 3 {
                continue;
            }
            let first = positions.len() as u32;
            for mut c in polygon.iter().copied() {
                if let Some(value) = held {
                    c.uv[axis] = value;
                }
                positions.push(c.pos);
                uvs.push(c.uv);
                colors.push(c.color.map(|v| v.round().clamp(0.0, 255.0) as u8));
            }
            for k in 1..polygon.len() as u32 - 1 {
                indices.extend([first, first + k, first + k + 1]);
            }
        }
    }
    (piece.positions, piece.uvs, piece.colors, piece.indices) = (positions, uvs, colors, indices);
}

/// Reads the background model in `archive` (a U8 archive holding a BRRES)
/// of the game folder `game`.
pub fn load(game: &Path, archive: &str) -> Option<Model> {
    let arc = std::fs::read(game.join(archive)).ok()?;
    let (_, offset, size) = crate::gameart::list_u8(&arc).into_iter().find(|f| f.0.to_ascii_lowercase().ends_with(".brres"))?;
    let p = arc.get(offset..offset.checked_add(size)?)?;
    if p.len() < 0x20 || &p[..4] != b"bres" {
        return None;
    }
    let root = be16(p, 12);
    let folders = group(p, root + 8);
    let folder = |name: &str| folders.iter().find(|f| f.0.starts_with(name)).map(|f| group(p, f.1)).unwrap_or_default();
    let textures = folder("Textures");
    let m = folder("3DModels").first()?.1;
    if p.get(m..m + 4)? != b"MDL0" || be32(p, m + 8) < 8 {
        return None;
    }
    // Section 0 definitions, 1 bones, 2 positions, 4 colours, 5 texture
    // coordinates, 8 materials, 10 objects (model versions 10 and 11).
    let section = |i: usize| match be32(p, m + 0x10 + 4 * i) {
        0 => Vec::new(),
        at => group(p, m + at),
    };
    let by_index = |entries: Vec<(String, usize)>, at: usize| {
        let mut list: Vec<(usize, usize)> = entries.iter().map(|e| (be32(p, e.1 + at), e.1)).collect();
        list.sort();
        list
    };
    let find = |list: &[(usize, usize)], index: i32| list.iter().find(|e| e.0 as i64 == i64::from(index)).map(|e| e.1);
    let bones = section(1);
    let (positions, colours, coords) = (by_index(section(2), 0x10), by_index(section(4), 0x10), by_index(section(5), 0x10));
    let (materials, objects) = (by_index(section(8), 0x0C), by_index(section(10), 0x3C));

    // The draw lists: which object is drawn with which material.
    let mut draws = Vec::new();
    for (name, at) in section(0) {
        if !name.starts_with("Draw") {
            continue;
        }
        let mut i = at;
        while p.get(i) == Some(&4) {
            draws.push((be16(p, i + 1), be16(p, i + 3)));
            i += 8;
        }
    }

    let mut pieces = Vec::new();
    let (mut left, mut right) = (f32::MAX, f32::MIN);
    for (mat_index, obj_index) in draws {
        let (Some(mat_at), Some(o)) = (find(&materials, mat_index as i32), find(&objects, obj_index as i32)) else { continue };
        let mat = material(p, mat_at);
        let layer_pictures: Vec<Option<Picture>> = mat.layers.iter().map(|l| textures.iter().find(|t| t.0 == l.name).and_then(|t| texture(p, t.1))).collect();
        let Some((picture, coord)) = bake(&mat, &layer_pictures) else { continue };
        let main_layer = layer_pictures.iter().position(|t| t.as_ref().is_some_and(|t| t.width == picture.width && t.height == picture.height)).unwrap_or(0);

        // The object's bone: its place in the model, and its scroll group.
        let node = be32(p, o + 8) as u32 as i32;
        let bone = bones.iter().map(|b| b.1).find(|&b| be32(p, b + 0x10) as u32 as i32 == node);
        let matrix: [f32; 12] = match bone {
            Some(b) => std::array::from_fn(|i| bef(p, b + 0x70 + 4 * i)),
            None => [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        };
        let mut scroll = 0u8;
        let mut up = bone;
        for _ in 0..32 {
            let Some(b) = up else { break };
            let name = cstr(p, b + be32(p, b + 8));
            if let Some(letter) = name.strip_prefix("Scroll").and_then(|n| n.bytes().next()).filter(u8::is_ascii_uppercase) {
                scroll = letter - b'A';
                break;
            }
            up = (be32(p, b + 0x5C) != 0).then(|| rel(p, b, b + 0x5C));
        }

        // Which arrays the object's vertices index, in display-list order.
        let (lo, hi) = (be32(p, o + 0x0C), be32(p, o + 0x10));
        let kind = |bits: usize| bits & 3;
        let mut layout: Vec<(usize, usize)> = Vec::new(); // (attribute, bytes): 0 skip, 1 position, 2 colour, 3 the coordinate used
        for bit in 0..9 {
            if lo >> bit & 1 != 0 {
                layout.push((0, 1));
            }
        }
        let width = |k: usize| if k == 3 { 2 } else { 1 };
        let (pos_kind, nrm_kind, clr_kind, clr1_kind) = (kind(lo >> 9), kind(lo >> 11), kind(lo >> 13), kind(lo >> 15));
        if pos_kind < 2 || nrm_kind == 1 || clr_kind == 1 || clr1_kind == 1 {
            continue; // values written in the list itself: not used by these models
        }
        layout.push((1, width(pos_kind)));
        for (k, attribute) in [(nrm_kind, 0), (clr_kind, 2), (clr1_kind, 0)] {
            if k != 0 {
                layout.push((attribute, width(k)));
            }
        }
        let mut direct = false;
        for t in 0..8 {
            match kind(hi >> (2 * t)) {
                0 => {}
                1 => direct = true,
                k => layout.push((if t == coord { 3 } else { 0 }, width(k))),
            }
        }
        if direct {
            continue;
        }
        let array_of = |list: &[(usize, usize)], at: usize| find(list, be16s(p, at));
        let (Some(pos_array), uv_array) = (array_of(&positions, o + 0x48), array_of(&coords, o + 0x50 + 2 * coord)) else { continue };
        let xyz = array(p, pos_array, if be32(p, pos_array + 0x14) != 0 { 3 } else { 2 });
        let st = uv_array.map(|a| array(p, a, if be32(p, a + 0x14) != 0 { 2 } else { 1 })).unwrap_or_default();
        let rgba = array_of(&colours, o + 0x4C).map(|a| colors(p, a)).unwrap_or_default();

        // The display list: triangles, strips, fans and quads.
        let (list, list_size) = (o + 0x24 + be32(p, o + 0x2C), be32(p, o + 0x24));
        let wrap = mat.layers.get(main_layer).map_or([false; 2], |l| l.repeat);
        let mut piece = Piece { texture: Picture { width: picture.width, height: picture.height, rgba: Vec::new() }, repeat: wrap[0] || wrap[1], positions: Vec::new(), uvs: Vec::new(), colors: Vec::new(), indices: Vec::new(), depth: 0.0, group: scroll };
        let mut depth_sum = 0.0;
        let mut i = list;
        while i + 3 <= (list + list_size).min(p.len()) {
            let command = p[i] & 0xF8;
            if !matches!(command, 0x80 | 0x90 | 0x98 | 0xA0) {
                i += 1;
                continue;
            }
            let count = be16(p, i + 1);
            i += 3;
            let first = piece.positions.len() as u32;
            for _ in 0..count {
                let (mut pos, mut uv, mut color) = ([0.0f32; 3], [0.0f32; 2], [255u8; 4]);
                for &(attribute, bytes) in &layout {
                    let index = if bytes == 2 { be16(p, i) } else { usize::from(p.get(i).copied().unwrap_or(0)) };
                    i += bytes;
                    match attribute {
                        1 => {
                            let v = xyz.get(index).map_or(&[][..], Vec::as_slice);
                            let (x, y, z) = (v.first().copied().unwrap_or(0.0), v.get(1).copied().unwrap_or(0.0), v.get(2).copied().unwrap_or(0.0));
                            pos = std::array::from_fn(|r| matrix[r * 4] * x + matrix[r * 4 + 1] * y + matrix[r * 4 + 2] * z + matrix[r * 4 + 3]);
                        }
                        2 => color = rgba.get(index).copied().unwrap_or([255; 4]),
                        3 => {
                            let v = st.get(index).map_or(&[][..], Vec::as_slice);
                            uv = [v.first().copied().unwrap_or(0.0), v.get(1).copied().unwrap_or(0.0)];
                        }
                        _ => {}
                    }
                }
                piece.positions.push([pos[0], pos[1]]);
                piece.uvs.push(uv);
                piece.colors.push(color);
                depth_sum += pos[2];
                (left, right) = (left.min(pos[0]), right.max(pos[0]));
            }
            // GX draws clockwise triangles as the front; `cull` 1 hides the
            // front, 2 the back.
            let mut triangle = |a: u32, b: u32, c: u32| {
                let at = |v: u32| piece.positions[(first + v) as usize];
                let (pa, pb, pc) = (at(a), at(b), at(c));
                let area = (pb[0] - pa[0]) * (pc[1] - pa[1]) - (pb[1] - pa[1]) * (pc[0] - pa[0]);
                let front = area < 0.0;
                if area != 0.0 && !((mat.cull == 2 && !front) || (mat.cull == 1 && front) || mat.cull == 3) {
                    piece.indices.extend([first + a, first + b, first + c]);
                }
            };
            let n = count as u32;
            match command {
                0x90 => (0..n / 3).for_each(|t| triangle(t * 3, t * 3 + 1, t * 3 + 2)),
                0x98 => (0..n.saturating_sub(2)).for_each(|t| if t % 2 == 0 { triangle(t, t + 1, t + 2) } else { triangle(t + 1, t, t + 2) }),
                0xA0 => (1..n.saturating_sub(1)).for_each(|t| triangle(0, t, t + 1)),
                _ => (0..n / 4).for_each(|q| {
                    triangle(q * 4, q * 4 + 1, q * 4 + 2);
                    triangle(q * 4, q * 4 + 2, q * 4 + 3);
                }),
            }
        }
        if piece.indices.is_empty() {
            continue;
        }
        piece.depth = depth_sum / piece.positions.len() as f32;
        // A texture that repeats one way only: the drawing here repeats
        // both ways or neither, so the other way is held at the edge by
        // cutting the triangles there.
        if wrap[0] != wrap[1] {
            hold_at_edges(&mut piece, if wrap[0] { 1 } else { 0 });
        }
        piece.texture.rgba = picture.rgba;
        pieces.push(piece);
    }
    if pieces.is_empty() {
        return None;
    }
    pieces.sort_by(|a, b| a.depth.total_cmp(&b.depth));
    Some(Model { pieces, period: (right - left).max(1.0) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(material: &mut Material, color_env: u32, alpha_env: u32) {
        material.stages = 1;
        material.bp[0x28] = 1 << 6; // stage 0 samples texture map 0
        material.bp[0xC0] = color_env;
        material.bp[0xC1] = alpha_env;
        material.bp[0xF3] = 7 << 16 | 7 << 19; // alpha test: always
    }

    fn blank() -> Material {
        Material { xlu: true, stages: 0, cull: 0, layers: Vec::new(), registers: [[0.0; 4]; 4], konst: [[0.0; 4]; 4], bp: Box::new([0; 256]) }
    }

    #[test]
    fn a_texture_times_the_vertex_colour_keeps_the_texture() {
        // a = zero, b = texture, c = raster, d = zero: texture x vertex colour.
        let mut mat = blank();
        flat(&mut mat, 15 << 12 | 8 << 8 | 10 << 4 | 15, 7 << 13 | 4 << 10 | 5 << 7 | 7 << 4);
        let source = Picture { width: 2, height: 1, rgba: vec![255, 128, 0, 255, 10, 20, 30, 64] };
        let (out, coord) = bake(&mat, &[Some(source)]).unwrap();
        assert_eq!(coord, 0);
        assert_eq!(out.rgba, vec![255, 128, 0, 255, 10, 20, 30, 64]);
    }

    #[test]
    fn a_grey_texture_mixes_two_register_colours() {
        // a = C0, b = C1, c = texture, d = zero: a sky painted as a ramp.
        let mut mat = blank();
        mat.xlu = false;
        mat.registers[1] = [1.0, 0.0, 0.0, 1.0];
        mat.registers[2] = [0.0, 0.0, 1.0, 1.0];
        flat(&mut mat, 2 << 12 | 4 << 8 | 8 << 4 | 15, 7 << 13 | 7 << 10 | 7 << 7 | 7 << 4);
        let source = Picture { width: 3, height: 1, rgba: vec![0, 0, 0, 255, 255, 255, 255, 255, 51, 51, 51, 255] };
        let (out, _) = bake(&mat, &[Some(source)]).unwrap();
        // Opaque whatever the stage's alpha says.
        assert_eq!(out.rgba, vec![255, 0, 0, 255, 0, 0, 255, 255, 204, 0, 51, 255]);
    }

    #[test]
    fn a_failed_alpha_test_cuts_the_texel_out() {
        let mut mat = blank();
        mat.xlu = false;
        flat(&mut mat, 15 << 12 | 8 << 8 | 10 << 4 | 15, 7 << 13 | 4 << 10 | 5 << 7 | 7 << 4);
        mat.bp[0xF3] = 128 | 6 << 16 | 7 << 19; // alpha >= 128
        let source = Picture { width: 2, height: 1, rgba: vec![9, 9, 9, 200, 9, 9, 9, 100] };
        let (out, _) = bake(&mat, &[Some(source)]).unwrap();
        assert_eq!(out.rgba, vec![9, 9, 9, 255, 9, 9, 9, 0]);
    }

    #[test]
    fn a_texture_that_does_not_repeat_upwards_is_held_at_its_edge() {
        // One triangle whose v runs from -1 to 2: cut at v = 0 and v = 1.
        let mut piece = Piece { texture: Picture { width: 4, height: 4, rgba: Vec::new() }, repeat: true, positions: vec![[0.0, 0.0], [3.0, 0.0], [0.0, 3.0]], uvs: vec![[0.0, -1.0], [5.0, -1.0], [0.0, 2.0]], colors: vec![[255; 4]; 3], indices: vec![0, 1, 2], depth: 0.0, group: 0 };
        hold_at_edges(&mut piece, 1);
        assert!(piece.uvs.iter().all(|uv| (0.0..=1.0).contains(&uv[1])), "{:?}", piece.uvs);
        // u is left to repeat.
        assert!(piece.uvs.iter().any(|uv| uv[0] > 1.0));
        // The pieces still cover the triangle: same area.
        let area: f32 = piece.indices.chunks_exact(3).map(|t| {
            let (a, b, c) = (piece.positions[t[0] as usize], piece.positions[t[1] as usize], piece.positions[t[2] as usize]);
            ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() / 2.0
        }).sum();
        assert!((area - 4.5).abs() < 1e-3, "{area}");
    }

    #[test]
    fn register_writes_respect_their_masks() {
        let mut bp = [0u32; 256];
        bp[0x41] = 0x00AB_CDEF;
        let list = [0x61, 0xFE, 0x00, 0x00, 0xFF, 0x61, 0x41, 0x12, 0x34, 0x56, 0x61, 0x42, 0x00, 0x00, 0x01];
        run_bp(&list, 0, list.len(), &mut bp, |_, _| {});
        assert_eq!(bp[0x41], 0x00AB_CD56);
        assert_eq!(bp[0x42], 1);
        assert!(load(Path::new("no such folder"), "files/Object/bgA_0102.arc").is_none());
    }
}
