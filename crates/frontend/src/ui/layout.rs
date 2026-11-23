//! New Super Mario Bros. Wii's own menu graphics, drawn over the game so the
//! port's menus (the settings window) look like the rest of the game: nw4r
//! layouts (BRLYT) with their TPL textures and BRFNT fonts, and their
//! animations (BRLAN), read from the player's disc at run time and drawn
//! through egui. A port of WitherBlitz's NSMB-Wii-Recompilation
//! `runtime/src/netplay/game_layout.cpp` (there drawn through ImGui).
//!
//! Covers what the game's menu layouts use: picture, text, window and null
//! panes; textures interpolated between a material's black and white
//! colours, tinted by vertex colours; alpha inheritance; z rotation.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use egui::epaint::{Mesh, Vertex};
use egui::{Color32, Pos2, TextureHandle, TextureId};

fn be16(p: &[u8], at: usize) -> u16 {
    p.get(at..at + 2).map_or(0, |b| u16::from_be_bytes([b[0], b[1]]))
}
fn be32(p: &[u8], at: usize) -> u32 {
    p.get(at..at + 4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}
fn bef(p: &[u8], at: usize) -> f32 {
    f32::from_bits(be32(p, at))
}
fn bes16(p: &[u8], at: usize) -> i16 {
    be16(p, at) as i16
}
fn cstr(p: &[u8], at: usize, max: usize) -> String {
    let end = p.len().min(at + max);
    let s = p.get(at..end).unwrap_or(&[]);
    let n = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    String::from_utf8_lossy(&s[..n]).into_owned()
}

// ---------------------------------------------------------------- files

/// The extracted disc (holds `files/`), set once at start.
static DISC: OnceLock<PathBuf> = OnceLock::new();
/// The game's language folder for fonts ("EngUS", "SpaUS", "FraUS"), first.
static LANGUAGE: OnceLock<&'static str> = OnceLock::new();

pub fn set_disc(game: &Path, language: &'static str) {
    let _ = DISC.set(game.to_path_buf());
    let _ = LANGUAGE.set(language);
}

/// A file from the disc's `files/` folder, by its disc path ("/Layout/...").
pub fn read_disc_file(dvd_path: &str) -> Option<Vec<u8>> {
    let root = DISC.get()?;
    std::fs::read(root.join("files").join(dvd_path.trim_start_matches('/'))).ok().filter(|d| !d.is_empty())
}

/// A U8 archive's files: path -> (offset, size).
pub fn list_u8(data: &[u8]) -> Vec<(String, u32, u32)> {
    let mut files = Vec::new();
    if data.len() < 0x20 || be32(data, 0) != 0x55AA_382D {
        return files;
    }
    let root = be32(data, 4) as usize;
    if root + 12 > data.len() {
        return files;
    }
    let total = be32(data, root + 8) as usize;
    let strings = root + total * 12;
    if strings > data.len() {
        return files;
    }
    let name = |off: u32| cstr(data, strings + off as usize, 256);
    let mut dirs: Vec<(String, usize)> = vec![(String::new(), total)];
    for i in 1..total {
        while dirs.len() > 1 && i >= dirs.last().unwrap().1 {
            dirs.pop();
        }
        let node = root + i * 12;
        let type_name = be32(data, node);
        let node_name = name(type_name & 0xFF_FFFF);
        let mut path = String::new();
        for d in &dirs {
            if !d.0.is_empty() {
                path += &d.0;
                path.push('/');
            }
        }
        path += &node_name;
        if type_name >> 24 == 1 {
            dirs.push((node_name, be32(data, node + 8) as usize));
        } else {
            files.push((path, be32(data, node + 4), be32(data, node + 8)));
        }
    }
    files
}

// ---------------------------------------------------------------- textures

/// GX texture `format` (0..14) of `width` x `height` as RGBA8, rows top down.
fn decode_texture(src: &[u8], format: u32, width: u32, height: u32, palette: &[u8], palette_format: u32) -> Option<Vec<u8>> {
    if width == 0 || height == 0 || !matches!(format, 0..=6 | 8..=10 | 14) {
        return None;
    }
    let rgba = gx::texture::decode(format, width as usize, height as usize, src, palette, palette_format);
    (rgba.len() >= (width * height * 4) as usize).then_some(rgba)
}

struct Image {
    width: u32,
    height: u32,
    wrap_s: u8,
    wrap_t: u8,
    rgba: Vec<u8>,
}

fn parse_tpl(data: &[u8]) -> Option<Image> {
    if data.len() < 12 || be32(data, 0) != 0x0020_AF30 || be32(data, 4) == 0 {
        return None;
    }
    let table = be32(data, 8) as usize;
    let header = be32(data, table) as usize;
    let palette_header = be32(data, table + 4) as usize;
    if header + 0x24 > data.len() {
        return None;
    }
    let height = u32::from(be16(data, header));
    let width = u32::from(be16(data, header + 2));
    let format = be32(data, header + 4);
    let offset = be32(data, header + 8) as usize;
    let (wrap_s, wrap_t) = (be32(data, header + 12) as u8, be32(data, header + 16) as u8);
    let (mut palette, mut palette_format): (&[u8], u32) = (&[], 0);
    if palette_header != 0 && palette_header + 12 <= data.len() {
        let count = be16(data, palette_header) as usize;
        palette_format = be32(data, palette_header + 4);
        let at = be32(data, palette_header + 8) as usize;
        palette = data.get(at..(at + count * 2).min(data.len())).unwrap_or(&[]);
    }
    let rgba = decode_texture(data.get(offset..)?, format, width, height, palette, palette_format)?;
    Some(Image { width, height, wrap_s, wrap_t, rgba })
}

/// An RGBA image (rows top down) on the GPU.
pub fn upload_rgba(ctx: &egui::Context, name: &str, width: u32, height: u32, rgba: &[u8]) -> TextureHandle {
    upload(ctx, name, &Image { width, height, wrap_s: 0, wrap_t: 0, rgba: rgba.to_vec() })
}

fn upload(ctx: &egui::Context, name: &str, image: &Image) -> TextureHandle {
    let pixels = egui::ColorImage::from_rgba_unmultiplied([image.width as usize, image.height as usize], &image.rgba[..(image.width * image.height * 4) as usize]);
    ctx.load_texture(name, pixels, egui::TextureOptions::LINEAR)
}

/// A 1x1 white texture, for untextured quads.
fn white(ctx: &egui::Context) -> TextureId {
    static WHITE: OnceLock<TextureHandle> = OnceLock::new();
    WHITE.get_or_init(|| upload(ctx, "white", &Image { width: 1, height: 1, wrap_s: 0, wrap_t: 0, rgba: vec![255; 4] })).id()
}

// ---------------------------------------------------------------- colours

#[derive(Clone, Copy, Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Rgba {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

impl Rgba {
    pub fn from_rgba8(c: u32) -> Self {
        Self { r: (c >> 24 & 0xFF) as f32 / 255.0, g: (c >> 16 & 0xFF) as f32 / 255.0, b: (c >> 8 & 0xFF) as f32 / 255.0, a: (c & 0xFF) as f32 / 255.0 }
    }
    fn mul(self, o: Self) -> Self {
        Self { r: self.r * o.r, g: self.g * o.g, b: self.b * o.b, a: self.a * o.a }
    }
    fn lerp(self, o: Self, t: f32) -> Self {
        Self { r: self.r + (o.r - self.r) * t, g: self.g + (o.g - self.g) * t, b: self.b + (o.b - self.b) * t, a: self.a + (o.a - self.a) * t }
    }
    fn color32(self) -> Color32 {
        let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        Color32::from_rgba_unmultiplied(q(self.r), q(self.g), q(self.b), q(self.a))
    }
}

// ---------------------------------------------------------------- fonts

#[derive(Clone, Copy, Default)]
struct Glyph {
    left: i8,
    glyph_width: u8,
    char_width: i8,
    sheet: u16,
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
    /// Drawn in colour: an icon, not a letter.
    picture: bool,
}

/// One of the game's BRFNT fonts.
#[derive(Default)]
pub struct Font {
    ok: bool,
    linefeed: i8,
    height: u8,
    width: u8,
    ascent: u8,
    cell_width: u8,
    cell_height: u8,
    /// The rows of a capital H's ink in its cell (from the top; 0, 0 unknown).
    ink_top: f32,
    ink_bottom: f32,
    default_index: u16,
    sheets: Vec<TextureHandle>,
    glyphs: Vec<Glyph>,
    map: HashMap<u16, u16>,
    /// The other language folders' copies, asked for letters this one lacks.
    fallbacks: Vec<Arc<Font>>,
    /// Letters for characters this font draws as icons.
    plain: Option<Arc<Font>>,
    /// A colour font's sheets as RGBA (the picture font's icons, for the
    /// Keybinds tab's key caps).
    pixels: Vec<Image>,
}

impl Font {
    pub fn ready(&self) -> bool {
        self.ok
    }

    /// The cell of `code` in a colour font as RGBA: (width, height, pixels).
    pub fn cell_rgba(&self, code: u16) -> Option<(u32, u32, Vec<u8>)> {
        let g = self.glyphs.get(*self.map.get(&code)? as usize)?;
        let sheet = self.pixels.get(g.sheet as usize)?;
        let (w, h) = (u32::from(self.cell_width), u32::from(self.cell_height));
        let (x0, y0) = ((g.u0 * sheet.width as f32).round() as u32, (g.v0 * sheet.height as f32).round() as u32);
        if x0 + w > sheet.width || y0 + h > sheet.height {
            return None;
        }
        let mut rgba = Vec::with_capacity((w * h * 4) as usize);
        for y in y0..y0 + h {
            let row = ((y * sheet.width + x0) * 4) as usize;
            rgba.extend_from_slice(&sheet.rgba[row..row + (w * 4) as usize]);
        }
        Some((w, h, rgba))
    }

    /// The glyph for `code` and the font whose sheets hold it.
    fn find(&self, code: u16, plain_text: bool) -> Option<(&Glyph, &Font)> {
        let own = self.map.get(&code).copied();
        if plain_text {
            if let (Some(plain), Some(i)) = (&self.plain, own) {
                if self.glyphs.get(i as usize).is_some_and(|g| g.picture) {
                    if let Some(&j) = plain.map.get(&code) {
                        if let Some(g) = plain.glyphs.get(j as usize) {
                            return Some((g, plain));
                        }
                    }
                }
            }
        }
        if own.is_none() {
            for other in &self.fallbacks {
                if let Some(&j) = other.map.get(&code) {
                    if let Some(g) = other.glyphs.get(j as usize) {
                        return Some((g, other));
                    }
                }
            }
        }
        let index = own.unwrap_or(self.default_index);
        self.glyphs.get(index as usize).map(|g| (g, self))
    }

    /// This font's size over `owner`'s, for a letter drawn from another font.
    fn owner_scale(&self, owner: &Font, vertical: bool) -> f32 {
        let (mine, theirs) = if vertical { (self.height, owner.height) } else { (self.width, owner.width) };
        if std::ptr::eq(owner, self) || theirs == 0 {
            1.0
        } else {
            f32::from(mine) / f32::from(theirs)
        }
    }

    fn load_file(ctx: &egui::Context, path: &str) -> Font {
        let mut font = Font::default();
        let Some(data) = read_disc_file(path) else { return font };
        if data.len() <= 0x30 || be32(&data, 0) != 0x5246_4E54 {
            return font;
        }
        let finf = be16(&data, 12) as usize;
        if be32(&data, finf) != 0x4649_4E46 {
            return font;
        }
        let f = finf + 8;
        font.linefeed = data[f + 1] as i8;
        font.default_index = be16(&data, f + 2);
        let tglp = be32(&data, f + 8) as usize;
        let mut cwdh = be32(&data, f + 12) as usize;
        let mut cmap = be32(&data, f + 16) as usize;
        font.height = data[f + 20];
        font.width = data[f + 21];
        font.ascent = data[f + 22];
        let mut all_sheets: Vec<Image> = Vec::new();
        if tglp + 0x18 <= data.len() {
            let t = tglp;
            font.cell_width = data[t];
            font.cell_height = data[t + 1];
            let sheet_size = be32(&data, t + 4) as usize;
            let sheet_count = be16(&data, t + 8);
            let sheet_format = u32::from(be16(&data, t + 10));
            let per_row = u32::from(be16(&data, t + 12));
            let per_column = u32::from(be16(&data, t + 14));
            let (sheet_w, sheet_h) = (u32::from(be16(&data, t + 16)), u32::from(be16(&data, t + 18)));
            let sheet_data = be32(&data, t + 20) as usize;
            let colour = (4..=6).contains(&sheet_format);
            for i in 0..sheet_count as usize {
                let at = sheet_data + i * sheet_size;
                let Some(src) = data.get(at..at + sheet_size) else { break };
                let Some(rgba) = decode_texture(src, sheet_format, sheet_w, sheet_h, &[], 0) else { break };
                let sheet = Image { width: sheet_w, height: sheet_h, wrap_s: 0, wrap_t: 0, rgba };
                font.sheets.push(upload(ctx, &format!("{path}#{i}"), &sheet));
                all_sheets.push(sheet);
            }
            let per_sheet = per_row * per_column;
            while cwdh != 0 && cwdh + 8 <= data.len() {
                let (first, last) = (be16(&data, cwdh) as usize, be16(&data, cwdh + 2) as usize);
                for index in first..=last {
                    let e = cwdh + 8 + (index - first) * 3;
                    if e + 3 > data.len() {
                        break;
                    }
                    if index >= font.glyphs.len() {
                        font.glyphs.resize(index + 1, Glyph::default());
                    }
                    let g = &mut font.glyphs[index];
                    g.left = data[e] as i8;
                    g.glyph_width = data[e + 1];
                    g.char_width = data[e + 2] as i8;
                }
                cwdh = be32(&data, cwdh + 4) as usize;
            }
            if per_sheet != 0 {
                let (cw, ch) = (u32::from(font.cell_width), u32::from(font.cell_height));
                for (index, g) in font.glyphs.iter_mut().enumerate() {
                    let index = index as u32;
                    g.sheet = (index / per_sheet) as u16;
                    let cell = index % per_sheet;
                    let x = (cell % per_row) * (cw + 1);
                    let y = (cell / per_row) * (ch + 1);
                    g.u0 = x as f32 / sheet_w as f32;
                    g.v0 = y as f32 / sheet_h as f32;
                    g.u1 = (x + cw) as f32 / sheet_w as f32;
                    g.v1 = (y + ch) as f32 / sheet_h as f32;
                    // An icon: a good part of what it draws is in colour.
                    if colour {
                        if let Some(sheet) = all_sheets.get(g.sheet as usize) {
                            let (mut opaque, mut coloured) = (0u32, 0u32);
                            for py in y..(y + ch).min(sheet.height) {
                                for px in x..(x + u32::from(g.glyph_width)).min(sheet.width) {
                                    let p = &sheet.rgba[((py * sheet.width + px) * 4) as usize..][..4];
                                    if p[3] > 128 {
                                        opaque += 1;
                                        let (hi, lo) = (p[0].max(p[1]).max(p[2]), p[0].min(p[1]).min(p[2]));
                                        coloured += u32::from(hi - lo > 60);
                                    }
                                }
                            }
                            g.picture = opaque != 0 && coloured * 100 > opaque * 15;
                        }
                    }
                }
            }
        }
        while cmap != 0 && cmap + 12 <= data.len() {
            let (begin, end, method) = (u32::from(be16(&data, cmap)), u32::from(be16(&data, cmap + 2)), be16(&data, cmap + 4));
            let body = cmap + 12;
            match method {
                0 => {
                    let offset = u32::from(be16(&data, body));
                    for code in begin..=end {
                        font.map.insert(code as u16, (code - begin + offset) as u16);
                    }
                }
                1 => {
                    for code in begin..=end {
                        let index = be16(&data, body + (code - begin) as usize * 2);
                        if index != 0xFFFF {
                            font.map.insert(code as u16, index);
                        }
                    }
                }
                2 => {
                    for i in 0..be16(&data, body) as usize {
                        font.map.insert(be16(&data, body + 2 + i * 4), be16(&data, body + 4 + i * 4));
                    }
                }
                _ => {}
            }
            cmap = be32(&data, cmap + 8) as usize;
        }
        font.ok = !font.sheets.is_empty() && !font.glyphs.is_empty();
        // The capital H's ink rows.
        if let Some(g) = font.map.get(&u16::from(b'H')).and_then(|&i| font.glyphs.get(i as usize)).filter(|_| font.ok) {
            if let Some(sheet) = all_sheets.get(g.sheet as usize) {
                let x0 = (g.u0 * sheet.width as f32 + 0.5) as u32;
                let y0 = (g.v0 * sheet.height as f32 + 0.5) as u32;
                let (mut top, mut bottom) = (-1i32, -1i32);
                for row in 0..u32::from(font.cell_height) {
                    if y0 + row >= sheet.height {
                        break;
                    }
                    let inked = (0..u32::from(font.cell_width)).take_while(|c| x0 + c < sheet.width).any(|c| sheet.rgba[(((y0 + row) * sheet.width + x0 + c) * 4 + 3) as usize] > 127);
                    if inked {
                        if top < 0 {
                            top = row as i32;
                        }
                        bottom = row as i32 + 1;
                    }
                }
                if top >= 0 {
                    font.ink_top = top as f32;
                    font.ink_bottom = bottom as f32;
                }
            }
        }
        if font.glyphs.iter().any(|g| g.picture) {
            font.pixels = all_sheets;
        }
        font
    }
}

static FONTS: OnceLock<Mutex<HashMap<String, Arc<Font>>>> = OnceLock::new();

/// One of the game's fonts ("mj2d00_MessageFont_32_I4.brfnt",
/// "mj2d01_marioFont_64_IA4.brfnt", ...), from the game's language folder,
/// with the other languages' copies behind it.
pub fn load_font(ctx: &egui::Context, name: &str) -> Arc<Font> {
    let cache = FONTS.get_or_init(Default::default);
    if let Some(f) = cache.lock().unwrap().get(name) {
        return f.clone();
    }
    let first = LANGUAGE.get().copied().unwrap_or("EngUS");
    let mut folders = vec![first];
    folders.extend(["EngUS", "SpaUS", "FraUS"].into_iter().filter(|f| *f != first));
    let mut font = Font::load_file(ctx, &format!("/US/{}/Font/{name}", folders[0]));
    if font.ok {
        for folder in &folders[1..] {
            let other = Font::load_file(ctx, &format!("/US/{folder}/Font/{name}"));
            if other.ok {
                font.fallbacks.push(Arc::new(other));
            }
        }
    }
    const PLAIN: &str = "mj2d01_marioFont_64_IA4.brfnt";
    if name != PLAIN && font.glyphs.iter().any(|g| g.picture) {
        let plain = load_font(ctx, PLAIN);
        if plain.ok {
            font.plain = Some(plain);
        }
    }
    let font = Arc::new(font);
    cache.lock().unwrap().insert(name.to_string(), font.clone());
    font
}

// ---------------------------------------------------------------- drawing

/// Where the game picture is on screen, in egui points: layout units mapped
/// with the layout origin at `center`, `scale` points per unit, y up.
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub center_x: f32,
    pub center_y: f32,
    pub scale: f32,
}

impl View {
    fn to_screen(&self, m: &Mtx, x: f32, y: f32) -> Pos2 {
        let lx = m.a * x + m.b * y + m.tx;
        let ly = m.c * x + m.d * y + m.ty;
        Pos2::new(self.center_x + lx * self.scale, self.center_y - ly * self.scale)
    }
}

/// Layout-space affine transform: x' = a*x + b*y + tx, y' = c*x + d*y + ty.
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

    /// `self` * T * R * S.
    fn compose(&self, tx: f32, ty: f32, rot_deg: f32, sx: f32, sy: f32) -> Self {
        let (sn, cs) = rot_deg.to_radians().sin_cos();
        let l = Self { a: cs * sx, b: -sn * sy, c: sn * sx, d: cs * sy, tx, ty };
        Self {
            a: self.a * l.a + self.b * l.c,
            b: self.a * l.b + self.b * l.d,
            c: self.c * l.a + self.d * l.c,
            d: self.c * l.b + self.d * l.d,
            tx: self.a * l.tx + self.b * l.ty + self.tx,
            ty: self.c * l.tx + self.d * l.ty + self.ty,
        }
    }
}

/// Shapes collected for one frame: textured quads, in order.
#[derive(Default)]
pub struct DrawList {
    pub meshes: Vec<Mesh>,
}

impl DrawList {
    fn quad(&mut self, texture: TextureId, p: [Pos2; 4], uv: [Pos2; 4], col: [Color32; 4]) {
        if self.meshes.last().is_none_or(|m| m.texture_id != texture) {
            self.meshes.push(Mesh::with_texture(texture));
        }
        let m = self.meshes.last_mut().unwrap();
        let base = m.vertices.len() as u32;
        for i in 0..4 {
            m.vertices.push(Vertex { pos: p[i], uv: uv[i], color: col[i] });
        }
        m.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    /// A plain rectangle in layout units (x0, y0 top-left, y up).
    pub fn rect(&mut self, ctx: &egui::Context, view: &View, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba) {
        let m = Mtx::IDENTITY;
        let p = [view.to_screen(&m, x0, y0), view.to_screen(&m, x1, y0), view.to_screen(&m, x1, y1), view.to_screen(&m, x0, y1)];
        let c = color.color32();
        self.quad(white(ctx), p, [Pos2::ZERO; 4], [c; 4]);
    }

    /// `texture`'s `uv` rectangle (u0, v0, u1, v1) on the rectangle from
    /// (x0, y0) to (x1, y1) in layout units (y up), tinted.
    #[allow(clippy::too_many_arguments)]
    pub fn image(&mut self, view: &View, texture: TextureId, x0: f32, y0: f32, x1: f32, y1: f32, uv: [f32; 4], tint: Rgba) {
        let m = Mtx::IDENTITY;
        let p = [view.to_screen(&m, x0, y0), view.to_screen(&m, x1, y0), view.to_screen(&m, x1, y1), view.to_screen(&m, x0, y1)];
        let [u0, v0, u1, v1] = uv;
        let c = tint.color32();
        self.quad(texture, p, [Pos2::new(u0, v0), Pos2::new(u1, v0), Pos2::new(u1, v1), Pos2::new(u0, v1)], [c; 4]);
    }

    /// A filled triangle, its corners in layout units.
    pub fn triangle(&mut self, ctx: &egui::Context, view: &View, pts: [(f32, f32); 3], color: Rgba) {
        let m = Mtx::IDENTITY;
        let p = pts.map(|(x, y)| view.to_screen(&m, x, y));
        let c = color.color32();
        let tex = white(ctx);
        if self.meshes.last().is_none_or(|m| m.texture_id != tex) {
            self.meshes.push(Mesh::with_texture(tex));
        }
        let mesh = self.meshes.last_mut().unwrap();
        let base = mesh.vertices.len() as u32;
        for p in p {
            mesh.vertices.push(Vertex { pos: p, uv: Pos2::ZERO, color: c });
        }
        mesh.indices.extend([base, base + 1, base + 2]);
    }

    /// A rectangle covering all of `screen` (points), for dimming the game.
    pub fn fill_screen(&mut self, ctx: &egui::Context, screen: egui::Rect, color: Rgba) {
        let c = color.color32();
        let p = [screen.left_top(), screen.right_top(), screen.right_bottom(), screen.left_bottom()];
        self.quad(white(ctx), p, [Pos2::ZERO; 4], [c; 4]);
    }

    pub fn into_shapes(self) -> Vec<egui::Shape> {
        self.meshes.into_iter().filter(|m| !m.indices.is_empty()).map(egui::Shape::mesh).collect()
    }
}

struct TextLine {
    text: Vec<u16>,
    width: f32,
}

fn split_lines(font: &Font, text: &[u16], scale_x: f32, char_space: f32, plain: bool) -> Vec<TextLine> {
    let mut lines = vec![TextLine { text: Vec::new(), width: 0.0 }];
    for &c in text {
        if c == u16::from(b'\n') {
            lines.push(TextLine { text: Vec::new(), width: 0.0 });
            continue;
        }
        let Some((g, owner)) = font.find(c, plain) else { continue };
        let line = lines.last_mut().unwrap();
        if !line.text.is_empty() {
            line.width += char_space;
        }
        line.text.push(c);
        line.width += f32::from(g.char_width) * scale_x * font.owner_scale(owner, false);
    }
    lines
}

#[allow(clippy::too_many_arguments)]
fn draw_text_lines(
    list: &mut DrawList,
    view: &View,
    m: &Mtx,
    font: &Font,
    text: &[u16],
    rect: [f32; 4],
    text_position: u8,
    alignment: u8,
    font_size: (f32, f32),
    char_space: f32,
    line_space: f32,
    top: Rgba,
    bottom: Rgba,
    plain: bool,
    center_ink: bool,
) {
    if !font.ok || font.width == 0 || font.height == 0 {
        return;
    }
    let [rx0, ry0, rw, rh] = rect;
    let scale_x = font_size.0 / f32::from(font.width);
    let scale_y = font_size.1 / f32::from(font.height);
    let line_height = f32::from(font.linefeed) * scale_y + line_space;
    let lines = split_lines(font, text, scale_x, char_space, plain);
    let block_w = lines.iter().map(|l| l.width).fold(0.0, f32::max);
    let block_h = line_height * (lines.len() - 1) as f32 + f32::from(font.cell_height) * scale_y;
    let (pos_h, pos_v) = (text_position % 3, text_position / 3);
    let block_x = rx0 + match pos_h {
        0 => 0.0,
        1 => (rw - block_w) / 2.0,
        _ => rw - block_w,
    };
    let mut y = ry0 - match pos_v {
        0 => 0.0,
        1 => (rh - block_h) / 2.0,
        _ => rh - block_h,
    };
    if center_ink && pos_v == 1 && font.ink_bottom > font.ink_top {
        y = ry0 - rh / 2.0 + (font.ink_top + font.ink_bottom) / 2.0 * scale_y + line_height * (lines.len() - 1) as f32 / 2.0;
    }
    let (tc, bc) = (top.color32(), bottom.color32());
    for line in &lines {
        let align = if alignment == 0 { pos_h } else { alignment - 1 };
        let mut x = block_x + match align {
            0 => 0.0,
            1 => (block_w - line.width) / 2.0,
            _ => block_w - line.width,
        };
        if center_ink && align == 1 && !line.text.is_empty() {
            if let (Some((first, fo)), Some((last, lo))) = (font.find(line.text[0], plain), font.find(*line.text.last().unwrap(), plain)) {
                let left = f32::from(first.left) * scale_x * font.owner_scale(fo, false);
                let right = (f32::from(last.char_width) - f32::from(last.left) - f32::from(last.glyph_width)) * scale_x * font.owner_scale(lo, false);
                x -= (left - right) / 2.0;
            }
        }
        for &c in &line.text {
            let Some((g, owner)) = font.find(c, plain) else { continue };
            let Some(sheet) = owner.sheets.get(g.sheet as usize) else { continue };
            let sx = scale_x * font.owner_scale(owner, false);
            let sy = scale_y * font.owner_scale(owner, true);
            let gy = y - (f32::from(font.ascent) * scale_y - f32::from(owner.ascent) * sy);
            let gx = x + f32::from(g.left) * sx;
            let (gw, gh) = (f32::from(owner.cell_width) * sx, f32::from(owner.cell_height) * sy);
            let p = [view.to_screen(m, gx, gy), view.to_screen(m, gx + gw, gy), view.to_screen(m, gx + gw, gy - gh), view.to_screen(m, gx, gy - gh)];
            let uv = [Pos2::new(g.u0, g.v0), Pos2::new(g.u1, g.v0), Pos2::new(g.u1, g.v1), Pos2::new(g.u0, g.v1)];
            list.quad(sheet.id(), p, uv, [tc, tc, bc, bc]);
            x += f32::from(g.char_width) * sx + char_space;
        }
        y -= line_height;
    }
}

pub fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// `text`'s width in layout units at `size`.
pub fn measure_text(font: &Font, text: &str, size: f32) -> f32 {
    if !font.ok || font.width == 0 {
        return 0.0;
    }
    split_lines(font, &utf16(text), size / f32::from(font.width), 0.0, true).iter().map(|l| l.width).fold(0.0, f32::max)
}

/// `text` broken at spaces into lines no wider than `max_width` at `size`
/// (a word wider than that has a line of its own).
pub fn wrap_text(font: &Font, text: &str, size: f32, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split(' ') {
        let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
        if !line.is_empty() && measure_text(font, &candidate, size) > max_width {
            lines.push(std::mem::replace(&mut line, word.to_string()));
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// The distance between lines of `font` at `size`, in layout units.
pub fn line_height(font: &Font, size: f32) -> f32 {
    if font.height == 0 {
        0.0
    } else {
        f32::from(font.linefeed) * size / f32::from(font.height)
    }
}

/// `text` at (`x`, `y`), the point `origin_h`/`origin_v` (0 left/top, 1
/// centre, 2 right/bottom) of its box.
#[allow(clippy::too_many_arguments)]
pub fn draw_text(list: &mut DrawList, view: &View, font: &Font, text: &str, x: f32, y: f32, size: f32, top: u32, bottom: u32, alpha: f32, origin_h: u8, origin_v: u8) {
    let (mut t, mut b) = (Rgba::from_rgba8(top), Rgba::from_rgba8(bottom));
    t.a *= alpha;
    b.a *= alpha;
    let w = measure_text(font, text, size);
    let h = if font.height == 0 { 0.0 } else { f32::from(font.cell_height) * size / f32::from(font.height) };
    let x0 = x - [0.0, w / 2.0, w][origin_h.min(2) as usize];
    let y0 = y + [0.0, h / 2.0, h][origin_v.min(2) as usize];
    draw_text_lines(list, view, &Mtx::IDENTITY, font, &utf16(text), [x0, y0, w, h], 0, 0, (size, size), 0.0, 0.0, t, b, true, false);
}

// ---------------------------------------------------------------- layouts

/// Per-draw adjustments, keyed by pane name.
#[derive(Default, Clone)]
pub struct DrawParams {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
    pub alpha: f32,
    pub text: HashMap<String, String>,
    pub visible: HashMap<String, bool>,
    pub pane_alpha: HashMap<String, f32>,
    pub pane_scale: HashMap<String, f32>,
    pub vertex_tint: HashMap<String, u32>,
    pub pane_size: HashMap<String, (f32, f32)>,
    pub color_matrix: HashMap<String, [f32; 9]>,
    pub pane_offset: HashMap<String, (f32, f32)>,
    pub pane_translate: HashMap<String, (f32, f32)>,
    pub vertex_colors: HashMap<String, [u32; 4]>,
    pub text_scale: HashMap<String, f32>,
    pub plain_text: bool,
    pub center_ink: bool,
    pub frame_clip: HashMap<String, (f32, f32)>,
}

impl DrawParams {
    pub fn new() -> Self {
        Self { scale: 1.0, alpha: 1.0, ..Default::default() }
    }
}

struct Material {
    black: Rgba,
    white: Rgba,
    texture: Option<TextureHandle>,
    texture_w: u32,
    texture_h: u32,
    wrap_s: u8,
    wrap_t: u8,
    srt: [f32; 4],
    mat_color: Option<Rgba>,
    color_from_material: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Null,
    Picture,
    Text,
    Window,
    Bounding,
}

struct Pane {
    kind: Kind,
    name: String,
    flags: u8,
    origin: u8,
    alpha: u8,
    tx: f32,
    ty: f32,
    rot: f32,
    sx: f32,
    sy: f32,
    w: f32,
    h: f32,
    children: Vec<usize>,
    vtx: [u32; 4],
    material: i32,
    uv: [[f32; 2]; 4],
    text: Vec<u16>,
    font: i32,
    text_position: u8,
    text_alignment: u8,
    top_color: u32,
    bottom_color: u32,
    font_w: f32,
    font_h: f32,
    char_space: f32,
    line_space: f32,
    overflow: [f32; 4],
    frames: Vec<(i32, u8)>,
}

impl Pane {
    fn new(kind: Kind) -> Self {
        Self {
            kind,
            name: String::new(),
            flags: 1,
            origin: 4,
            alpha: 255,
            tx: 0.0,
            ty: 0.0,
            rot: 0.0,
            sx: 1.0,
            sy: 1.0,
            w: 0.0,
            h: 0.0,
            children: Vec::new(),
            vtx: [u32::MAX; 4],
            material: -1,
            uv: [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            text: Vec::new(),
            font: -1,
            text_position: 4,
            text_alignment: 0,
            top_color: u32::MAX,
            bottom_color: u32::MAX,
            font_w: 32.0,
            font_h: 32.0,
            char_space: 0.0,
            line_space: 0.0,
            overflow: [0.0; 4],
            frames: Vec::new(),
        }
    }
}

/// One of the game's menu layouts (BRLYT) with its textures and fonts.
#[derive(Default)]
pub struct Layout {
    panes: Vec<Pane>,
    by_name: HashMap<String, usize>,
    materials: Vec<Material>,
    fonts: Vec<Arc<Font>>,
}

impl Layout {
    pub fn loaded(&self) -> bool {
        !self.panes.is_empty()
    }

    /// `arc_path` a disc path ("/Layout/pauseMenu/pauseMenu.arc"); its first
    /// .brlyt unless `brlyt` names one.
    pub fn load(ctx: &egui::Context, arc_path: &str, brlyt: &str) -> Option<Self> {
        let arc = read_disc_file(arc_path)?;
        let files = list_u8(&arc);
        let file = |name: &str| files.iter().find(|(p, ..)| p.ends_with(name)).and_then(|(_, o, s)| arc.get(*o as usize..(o + s) as usize));
        let lyt = files.iter().find(|(p, ..)| p.ends_with(".brlyt") && (brlyt.is_empty() || p.contains(brlyt))).and_then(|(_, o, s)| arc.get(*o as usize..(o + s) as usize))?;
        if lyt.len() < 16 || be32(lyt, 0) != 0x524C_5954 {
            return None;
        }
        let mut out = Layout::default();
        let (mut texture_names, mut font_names) = (Vec::<String>::new(), Vec::<String>::new());
        let mut images: HashMap<String, Option<Image>> = HashMap::new();
        let mut stack: Vec<usize> = Vec::new();
        let mut last: Option<usize> = None;
        let mut pos = be16(lyt, 12) as usize;
        for _ in 0..be16(lyt, 14) {
            if pos + 8 > lyt.len() {
                break;
            }
            let sec = &lyt[pos..];
            let (kind, size) = (be32(sec, 0), be32(sec, 4) as usize);
            if size < 8 || pos + size > lyt.len() {
                break;
            }
            let sec = &lyt[pos..pos + size];
            let body = 8;
            match kind {
                0x7478_6C31 | 0x666E_6C31 => {
                    // txl1, fnl1
                    for i in 0..be16(sec, body) as usize {
                        let off = be32(sec, body + 4 + i * 8) as usize;
                        let name = cstr(sec, body + 4 + off, 128);
                        if kind == 0x7478_6C31 { &mut texture_names } else { &mut font_names }.push(name);
                    }
                    if kind == 0x666E_6C31 {
                        out.fonts = font_names.iter().map(|n| load_font(ctx, n)).collect();
                    }
                }
                0x6D61_7431 => {
                    // mat1
                    for i in 0..be16(sec, body) as usize {
                        let m = be32(sec, body + 4 + i * 4) as usize;
                        let s10 = |c: usize| {
                            let at = m + 20 + c * 8;
                            Rgba { r: f32::from(bes16(sec, at)) / 255.0, g: f32::from(bes16(sec, at + 2)) / 255.0, b: f32::from(bes16(sec, at + 4)) / 255.0, a: f32::from(bes16(sec, at + 6)) / 255.0 }
                        };
                        let name = cstr(sec, m, 20);
                        let mut mat = Material { black: s10(0), white: s10(1), texture: None, texture_w: 0, texture_h: 0, wrap_s: 0, wrap_t: 0, srt: [0.0, 0.0, 1.0, 1.0], mat_color: None, color_from_material: false };
                        let flags = be32(sec, m + 60);
                        let mut p = m + 64;
                        let (tex_count, srt_count, coord_count) = (flags & 15, (flags >> 4) & 15, (flags >> 8) & 15);
                        let mut tex_index: i32 = -1;
                        for t in 0..tex_count {
                            if t == 0 {
                                tex_index = i32::from(be16(sec, p));
                                mat.wrap_s = sec.get(p + 2).copied().unwrap_or(0);
                                mat.wrap_t = sec.get(p + 3).copied().unwrap_or(0);
                            }
                            p += 4;
                        }
                        for t in 0..srt_count {
                            if t == 0 {
                                mat.srt = [bef(sec, p), bef(sec, p + 4), bef(sec, p + 12), bef(sec, p + 16)];
                            }
                            p += 20;
                        }
                        p += coord_count as usize * 4;
                        if flags & (1 << 25) != 0 {
                            mat.color_from_material = sec.get(p).copied() == Some(0);
                            p += 4;
                        }
                        if flags & (1 << 27) != 0 {
                            mat.mat_color = Some(Rgba::from_rgba8(be32(sec, p)));
                        }
                        if let Some(tex) = usize::try_from(tex_index).ok().and_then(|i| texture_names.get(i)) {
                            let decoded = images.entry(tex.clone()).or_insert_with(|| file(tex).and_then(parse_tpl));
                            if let Some(src) = decoded {
                                let mut baked = Image { width: src.width, height: src.height, wrap_s: src.wrap_s, wrap_t: src.wrap_t, rgba: src.rgba.clone() };
                                let (bk, wh) = (mat.black, mat.white);
                                let identity = bk.r == 0.0 && bk.g == 0.0 && bk.b == 0.0 && bk.a == 0.0 && wh.r >= 1.0 && wh.g >= 1.0 && wh.b >= 1.0 && wh.a >= 1.0;
                                if !identity {
                                    let (bk, wh) = ([bk.r, bk.g, bk.b, bk.a], [wh.r, wh.g, wh.b, wh.a]);
                                    for px in baked.rgba.chunks_exact_mut(4) {
                                        for k in 0..4 {
                                            let c = f32::from(px[k]) / 255.0;
                                            px[k] = ((bk[k] + (wh[k] - bk[k]) * c).clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                                        }
                                    }
                                }
                                mat.texture_w = baked.width;
                                mat.texture_h = baked.height;
                                mat.texture = Some(upload(ctx, &format!("{arc_path}:{name}"), &baked));
                            }
                        }
                        out.materials.push(mat);
                    }
                }
                0x7061_6E31 | 0x7069_6331 | 0x7478_7431 | 0x776E_6431 | 0x626E_6431 => {
                    let kind = match kind {
                        0x7069_6331 => Kind::Picture,
                        0x7478_7431 => Kind::Text,
                        0x776E_6431 => Kind::Window,
                        0x626E_6431 => Kind::Bounding,
                        _ => Kind::Null,
                    };
                    let mut pane = Pane::new(kind);
                    pane.flags = sec[body];
                    pane.origin = sec[body + 1];
                    pane.alpha = sec[body + 2];
                    pane.name = cstr(sec, body + 4, 16);
                    pane.tx = bef(sec, body + 28);
                    pane.ty = bef(sec, body + 32);
                    pane.rot = bef(sec, body + 48);
                    pane.sx = bef(sec, body + 52);
                    pane.sy = bef(sec, body + 56);
                    pane.w = bef(sec, body + 60);
                    pane.h = bef(sec, body + 64);
                    let extra = body + 68;
                    let read_content = |pane: &mut Pane, c: usize| {
                        for i in 0..4 {
                            pane.vtx[i] = be32(sec, c + i * 4);
                        }
                        pane.material = i32::from(be16(sec, c + 16));
                        if sec.get(c + 18).copied().unwrap_or(0) > 0 {
                            for i in 0..4 {
                                pane.uv[i] = [bef(sec, c + 20 + i * 8), bef(sec, c + 24 + i * 8)];
                            }
                        }
                    };
                    match kind {
                        Kind::Picture => read_content(&mut pane, extra),
                        Kind::Text => {
                            let string_size = be16(sec, extra + 2) as usize;
                            pane.material = i32::from(be16(sec, extra + 4));
                            pane.font = i32::from(be16(sec, extra + 6));
                            pane.text_position = sec[extra + 8];
                            pane.text_alignment = sec[extra + 9];
                            let string_offset = be32(sec, extra + 12) as usize;
                            pane.top_color = be32(sec, extra + 16);
                            pane.bottom_color = be32(sec, extra + 20);
                            pane.font_w = bef(sec, extra + 24);
                            pane.font_h = bef(sec, extra + 28);
                            pane.char_space = bef(sec, extra + 32);
                            pane.line_space = bef(sec, extra + 36);
                            let mut i = 0;
                            while i + 1 < string_size && string_offset + i + 1 < size {
                                let c = be16(sec, string_offset + i);
                                if c == 0 {
                                    break;
                                }
                                pane.text.push(c);
                                i += 2;
                            }
                        }
                        Kind::Window => {
                            for i in 0..4 {
                                pane.overflow[i] = bef(sec, extra + i * 4);
                            }
                            let frame_count = sec[extra + 16];
                            let content = be32(sec, extra + 20) as usize;
                            let table = be32(sec, extra + 24) as usize;
                            if content < size {
                                read_content(&mut pane, content);
                            }
                            for f in 0..frame_count as usize {
                                if table + f * 4 + 4 > size {
                                    break;
                                }
                                let fr = be32(sec, table + f * 4) as usize;
                                pane.frames.push((i32::from(be16(sec, fr)), sec.get(fr + 2).copied().unwrap_or(0)));
                            }
                        }
                        _ => {}
                    }
                    let index = out.panes.len();
                    if let Some(&parent) = stack.last() {
                        out.panes[parent].children.push(index);
                    }
                    if !pane.name.is_empty() {
                        out.by_name.entry(pane.name.clone()).or_insert(index);
                    }
                    out.panes.push(pane);
                    last = Some(index);
                }
                0x7061_7331 => {
                    // pas1
                    if let Some(l) = last {
                        stack.push(l);
                    }
                }
                0x7061_6531 => {
                    // pae1
                    stack.pop();
                }
                _ => {}
            }
            pos += size;
        }
        out.loaded().then_some(out)
    }

    /// A pane's authored size and translation.
    pub fn pane_rect(&self, name: &str) -> Option<(f32, f32, f32, f32)> {
        let p = &self.panes[*self.by_name.get(name)?];
        Some((p.tx, p.ty, p.w * p.sx, p.h * p.sy))
    }

    /// A pane's authored translation and scale.
    pub fn pane_transform(&self, name: &str) -> Option<(f32, f32, f32, f32)> {
        let p = &self.panes[*self.by_name.get(name)?];
        Some((p.tx, p.ty, p.sx, p.sy))
    }

    /// Grows a row of three slices (left cap, middle, right cap) by `dw` x
    /// `dh` about its centre.
    pub fn stretch_slices(&self, params: &mut DrawParams, left: &str, middle: &str, right: &str, dw: f32, dh: f32) -> bool {
        let Some(&mid) = self.by_name.get(middle) else { return false };
        let keep_centred = |p: &Pane, gw: f32, gh: f32| {
            let (h, v) = (p.origin % 3, p.origin / 3);
            (match h {
                0 => -gw / 2.0,
                2 => gw / 2.0,
                _ => 0.0,
            }, match v {
                0 => gh / 2.0,
                2 => -gh / 2.0,
                _ => 0.0,
            })
        };
        let m = &self.panes[mid];
        params.pane_size.insert(middle.into(), (m.w + dw, m.h + dh));
        params.pane_offset.insert(middle.into(), keep_centred(m, dw, dh));
        for (name, side) in [(left, -1.0), (right, 1.0)] {
            if let Some(&cap) = self.by_name.get(name) {
                let c = &self.panes[cap];
                let (ox, oy) = keep_centred(c, 0.0, dh);
                params.pane_size.insert(name.into(), (c.w, c.h + dh));
                params.pane_offset.insert(name.into(), (ox + side * dw / 2.0, oy));
            }
        }
        true
    }

    /// Draws `name` and its children ("RootPane": the whole layout).
    pub fn draw(&self, ctx: &egui::Context, list: &mut DrawList, view: &View, name: &str, params: &DrawParams) {
        if let Some(&i) = self.by_name.get(name) {
            self.draw_pane(ctx, list, view, i, &Mtx::IDENTITY, params.alpha, true, params, true);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_pane(&self, ctx: &egui::Context, list: &mut DrawList, view: &View, index: usize, parent: &Mtx, parent_alpha: f32, parent_influences: bool, params: &DrawParams, root: bool) {
        let pane = &self.panes[index];
        let visible = params.visible.get(&pane.name).copied().unwrap_or(pane.flags & 1 != 0);
        if !visible {
            return;
        }
        let own_alpha = params.pane_alpha.get(&pane.name).copied().unwrap_or(f32::from(pane.alpha) / 255.0);
        let alpha = own_alpha * if parent_influences { parent_alpha } else { 1.0 };
        let (mut sx, mut sy) = params.pane_scale.get(&pane.name).map_or((pane.sx, pane.sy), |&s| (s, s));
        let (mut tx, mut ty) = params.pane_translate.get(&pane.name).copied().unwrap_or((pane.tx, pane.ty));
        if root {
            tx += params.offset_x;
            ty += params.offset_y;
            sx *= params.scale;
            sy *= params.scale;
        }
        if let Some(&(ox, oy)) = params.pane_offset.get(&pane.name) {
            tx += ox;
            ty += oy;
        }
        let m = parent.compose(tx, ty, pane.rot, sx, sy);
        let (pw, ph) = params.pane_size.get(&pane.name).copied().unwrap_or((pane.w, pane.h));
        let (pos_h, pos_v) = (pane.origin % 3, pane.origin / 3);
        let x0 = [0.0, -pw / 2.0, -pw][pos_h.min(2) as usize];
        let y0 = [0.0, ph / 2.0, ph][pos_v.min(2) as usize];
        let tint = Rgba::from_rgba8(params.vertex_tint.get(&pane.name).copied().unwrap_or(u32::MAX));
        let matrix = params.color_matrix.get(&pane.name);
        let pane_vtx = params.vertex_colors.get(&pane.name).copied().unwrap_or(pane.vtx);
        let material = |i: i32| usize::try_from(i).ok().and_then(|i| self.materials.get(i));
        let white_tex = white(ctx);

        let mut quad = |mat: Option<&Material>, vtx: [u32; 4], ax0: f32, ay0: f32, ax1: f32, ay1: f32, uv_in: [[f32; 2]; 4], flip: u8| {
            let mut uv = uv_in;
            if let Some(mat) = mat {
                let [stx, sty, ssx, ssy] = mat.srt;
                if ssx != 1.0 || ssy != 1.0 || stx != 0.0 || sty != 0.0 {
                    for c in &mut uv {
                        c[0] = (c[0] - 0.5) * ssx + 0.5 + stx;
                        c[1] = (c[1] - 0.5) * ssy + 0.5 - sty;
                    }
                }
            }
            if flip & 1 != 0 {
                uv.swap(0, 1);
                uv.swap(2, 3);
            }
            if flip & 2 != 0 {
                uv.swap(0, 2);
                uv.swap(1, 3);
            }
            let mut colors = [Rgba::default(); 4];
            for (i, c) in colors.iter_mut().enumerate() {
                let mut v = Rgba::from_rgba8(vtx[i]);
                if let Some(mc) = mat.filter(|m| m.color_from_material).and_then(|m| m.mat_color) {
                    v = mc;
                }
                v = v.mul(tint);
                if let Some(m) = matrix {
                    let i = v;
                    v.r = (m[0] * i.r + m[1] * i.g + m[2] * i.b).min(1.0);
                    v.g = (m[3] * i.r + m[4] * i.g + m[5] * i.b).min(1.0);
                    v.b = (m[6] * i.r + m[7] * i.g + m[8] * i.b).min(1.0);
                }
                v.a *= alpha;
                *c = v;
            }
            let textured = mat.and_then(|m| m.texture.as_ref());
            let texture = textured.map_or(white_tex, |t| t.id());
            if textured.is_none() {
                if let Some(mat) = mat {
                    for c in &mut colors {
                        *c = c.mul(mat.white);
                    }
                }
            }
            // Repeating textures: split the quad at whole repeats (the sampler clamps).
            let (u0, u1, v0, v1) = (uv[0][0], uv[1][0], uv[0][1], uv[2][1]);
            let (umin, umax, vmin, vmax) = (u0.min(u1), u0.max(u1), v0.min(v1), v0.max(v1));
            let tile_u = textured.is_some() && mat.is_some_and(|m| m.wrap_s != 0) && (umin < -0.001 || umax > 1.001);
            let tile_v = textured.is_some() && mat.is_some_and(|m| m.wrap_t != 0) && (vmin < -0.001 || vmax > 1.001);
            let cuts = |tile: bool, a: f32, b: f32, lo: f32, hi: f32| {
                let mut v = vec![0.0f32, 1.0];
                if tile && b != a {
                    v = vec![0.0];
                    let mut k = lo.floor() + 1.0;
                    while k < hi {
                        v.push((k - a) / (b - a));
                        k += 1.0;
                    }
                    v.push(1.0);
                    v.sort_by(f32::total_cmp);
                }
                v
            };
            let (us, vs) = (cuts(tile_u, u0, u1, umin, umax), cuts(tile_v, v0, v1, vmin, vmax));
            let color_at = |fx: f32, fy: f32| colors[0].lerp(colors[1], fx).lerp(colors[2].lerp(colors[3], fx), fy);
            let wrap = |t: f32, start: f32, mode: u8| {
                let mut f = t - start.floor();
                if mode == 2 && (start.floor() as i32) & 1 != 0 {
                    f = 1.0 - f;
                }
                f
            };
            for y in vs.windows(2) {
                for x in us.windows(2) {
                    let (fx0, fx1, fy0, fy1) = (x[0], x[1], y[0], y[1]);
                    let (px0, px1) = (ax0 + (ax1 - ax0) * fx0, ax0 + (ax1 - ax0) * fx1);
                    let (py0, py1) = (ay0 + (ay1 - ay0) * fy0, ay0 + (ay1 - ay0) * fy1);
                    let (mut tu0, mut tu1) = (u0 + (u1 - u0) * fx0, u0 + (u1 - u0) * fx1);
                    let (mut tv0, mut tv1) = (v0 + (v1 - v0) * fy0, v0 + (v1 - v0) * fy1);
                    if tile_u {
                        let start = tu0.min(tu1) + 0.0001;
                        let mode = mat.map_or(0, |m| m.wrap_s);
                        (tu0, tu1) = (wrap(tu0, start, mode), wrap(tu1, start, mode));
                    }
                    if tile_v {
                        let start = tv0.min(tv1) + 0.0001;
                        let mode = mat.map_or(0, |m| m.wrap_t);
                        (tv0, tv1) = (wrap(tv0, start, mode), wrap(tv1, start, mode));
                    }
                    let p = [view.to_screen(&m, px0, py0), view.to_screen(&m, px1, py0), view.to_screen(&m, px1, py1), view.to_screen(&m, px0, py1)];
                    let t = [Pos2::new(tu0, tv0), Pos2::new(tu1, tv0), Pos2::new(tu1, tv1), Pos2::new(tu0, tv1)];
                    let c = [color_at(fx0, fy0).color32(), color_at(fx1, fy0).color32(), color_at(fx1, fy1).color32(), color_at(fx0, fy1).color32()];
                    list.quad(texture, p, t, c);
                }
            }
        };

        match pane.kind {
            Kind::Picture if alpha > 0.001 => quad(material(pane.material), pane_vtx, x0, y0, x0 + pw, y0 - ph, pane.uv, 0),
            Kind::Window if alpha > 0.001 => {
                let o = pane.overflow;
                quad(material(pane.material), pane_vtx, x0 - o[0], y0 + o[2], x0 + pw + o[1], y0 - ph - o[3], pane.uv, 0);
                if !pane.frames.is_empty() {
                    let single = pane.frames.len() == 1;
                    let mut mats = [None; 4];
                    let mut flips = [0u8; 4];
                    for i in 0..4 {
                        let frame = pane.frames[if single { 0 } else { i.min(pane.frames.len() - 1) }];
                        mats[i] = material(frame.0);
                        flips[i] = if single {
                            i as u8
                        } else {
                            match frame.1 {
                                1 => 1,
                                2 => 2,
                                4 => 3,
                                _ => 0,
                            }
                        };
                    }
                    let tex_w = |i: usize| mats[i].filter(|m: &&Material| m.texture.is_some()).map_or(16.0, |m| m.texture_w as f32);
                    let tex_h = |i: usize| mats[i].filter(|m: &&Material| m.texture.is_some()).map_or(16.0, |m| m.texture_h as f32);
                    let (l_, r_, t_, b_) = (x0, x0 + pw, y0, y0 - ph);
                    let (t, r, l, b) = (tex_h(0).min(ph), tex_w(1).min(pw), tex_w(2).min(pw), tex_h(3).min(ph));
                    let clip = params.frame_clip.get(&pane.name).copied();
                    let mut draw_whole = |i: usize, qx0: f32, qy0: f32, qx1: f32, qy1: f32, mut u0: f32, mut u1: f32, mut v0: f32, mut v1: f32| {
                        if flips[i] & 1 != 0 {
                            (u0, u1) = (1.0 - u0, 1.0 - u1);
                        }
                        if flips[i] & 2 != 0 {
                            (v0, v1) = (1.0 - v0, 1.0 - v1);
                        }
                        quad(mats[i], [u32::MAX; 4], qx0, qy0, qx1, qy1, [[u0, v0], [u1, v0], [u0, v1], [u1, v1]], 0);
                    };
                    let mut draw = |i: usize, qx0: f32, qy0: f32, qx1: f32, qy1: f32, u0: f32, u1: f32, v0: f32, v1: f32| {
                        let Some((cx, cy)) = clip else {
                            draw_whole(i, qx0, qy0, qx1, qy1, u0, u1, v0, v1);
                            return;
                        };
                        let (ix0, ix1, iy0, iy1) = (l_ + cx, r_ - cx, t_ - cy, b_ + cy);
                        let mut piece = |px0: f32, py0: f32, px1: f32, py1: f32| {
                            if px1 - px0 <= 0.001 || py0 - py1 <= 0.001 {
                                return;
                            }
                            let (fx0, fx1) = ((px0 - qx0) / (qx1 - qx0), (px1 - qx0) / (qx1 - qx0));
                            let (fy0, fy1) = ((qy0 - py0) / (qy0 - qy1), (qy0 - py1) / (qy0 - qy1));
                            draw_whole(i, px0, py0, px1, py1, u0 + (u1 - u0) * fx0, u0 + (u1 - u0) * fx1, v0 + (v1 - v0) * fy0, v0 + (v1 - v0) * fy1);
                        };
                        piece(qx0, qy0, qx1, qy1.max(iy0));
                        piece(qx0, qy0.min(iy1), qx1, qy1);
                        let (my0, my1) = (qy0.min(iy0), qy1.max(iy1));
                        piece(qx0, my0, qx1.min(ix0), my1);
                        piece(qx0.max(ix1), my0, qx1, my1);
                    };
                    draw(0, l_, t_, r_ - r, t_ - t, 0.0, (pw - r) / tex_w(0), 0.0, t / tex_h(0));
                    draw(1, r_ - r, t_, r_, b_ + b, 1.0 - r / tex_w(1), 1.0, 0.0, (ph - b) / tex_h(1));
                    draw(3, l_ + l, b_ + b, r_, b_, 1.0 - (pw - l) / tex_w(3), 1.0, 1.0 - b / tex_h(3), 1.0);
                    draw(2, l_, t_ - t, l_ + l, b_, 0.0, l / tex_w(2), 1.0 - (ph - t) / tex_h(2), 1.0);
                }
            }
            Kind::Text if alpha > 0.001 => {
                if let Some(font) = usize::try_from(pane.font).ok().and_then(|i| self.fonts.get(i)) {
                    let owned;
                    let text: &[u16] = match params.text.get(&pane.name) {
                        Some(t) => {
                            owned = utf16(t);
                            &owned
                        }
                        None => &pane.text,
                    };
                    let (mut top, mut bottom) = (Rgba::from_rgba8(pane.top_color).mul(tint), Rgba::from_rgba8(pane.bottom_color).mul(tint));
                    if let Some(mat) = material(pane.material) {
                        top = top.mul(mat.white);
                        bottom = bottom.mul(mat.white);
                        top.a = Rgba::from_rgba8(pane.top_color).a * tint.a;
                        bottom.a = Rgba::from_rgba8(pane.bottom_color).a * tint.a;
                    }
                    top.a *= alpha;
                    bottom.a *= alpha;
                    let s = params.text_scale.get(&pane.name).copied().unwrap_or(1.0);
                    draw_text_lines(list, view, &m, font, text, [x0, y0, pw, ph], pane.text_position, pane.text_alignment, (pane.font_w * s, pane.font_h * s), pane.char_space, pane.line_space, top, bottom, params.plain_text, params.center_ink);
                }
            }
            _ => {}
        }
        let influences = pane.flags & 2 != 0;
        for &child in &pane.children {
            self.draw_pane(ctx, list, view, child, &m, alpha, influences, params, false);
        }
    }
}

// ---------------------------------------------------------------- animations

#[derive(Clone, Copy)]
struct Key {
    frame: f32,
    value: f32,
    slope: f32,
}

struct Track {
    target: u8,
    step: bool,
    keys: Vec<Key>,
}

/// nw4r's curve: Hermite between the keys either side of `frame`, held flat
/// outside them.
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

struct PaneAnim {
    name: String,
    srt: Vec<Track>,
    vtx: Vec<Track>,
    vis: Vec<Track>,
}

/// One of a layout archive's animations (BRLAN): translation, scale, vertex
/// colours, alpha and visibility per pane.
#[derive(Default)]
pub struct Animation {
    frames: f32,
    panes: Vec<PaneAnim>,
}

impl Animation {
    pub fn load(arc_path: &str, brlan: &str) -> Option<Self> {
        let arc = read_disc_file(arc_path)?;
        let lan = list_u8(&arc).into_iter().find(|(p, ..)| p.contains(brlan)).and_then(|(_, o, s)| arc.get(o as usize..(o + s) as usize))?.to_vec();
        if lan.len() < 16 || be32(&lan, 0) != 0x524C_414E {
            return None;
        }
        let d = &lan[..];
        let mut out = Animation::default();
        let mut pos = be16(d, 12) as usize;
        for _ in 0..be16(d, 14) {
            let (kind, size) = (be32(d, pos), be32(d, pos + 4) as usize);
            if size < 8 || pos + size > d.len() {
                break;
            }
            if kind == 0x7061_6931 && pos + 20 <= d.len() {
                let b = pos + 8;
                out.frames = f32::from(be16(d, b));
                let entries = be16(d, b + 6);
                let table = be32(d, b + 8) as usize;
                for e in 0..entries as usize {
                    let eo = pos + be32(d, pos + table + e * 4) as usize;
                    if eo + 28 > d.len() {
                        break;
                    }
                    let mut pane = PaneAnim { name: cstr(d, eo, 20), srt: Vec::new(), vtx: Vec::new(), vis: Vec::new() };
                    let (tags, material) = (d[eo + 20], d[eo + 21] != 0);
                    for t in 0..tags as usize {
                        let to = eo + be32(d, eo + 24 + t * 4) as usize;
                        if to + 8 > d.len() || material {
                            break;
                        }
                        let tag = be32(d, to);
                        let count = d[to + 4];
                        let into = match tag {
                            0x524C_5041 => &mut pane.srt,
                            0x524C_5643 => &mut pane.vtx,
                            0x524C_5649 => &mut pane.vis,
                            _ => continue,
                        };
                        for k in 0..count as usize {
                            let io = to + be32(d, to + 8 + k * 4) as usize;
                            if io + 12 > d.len() {
                                break;
                            }
                            let curve = d[io + 2];
                            let key_count = be16(d, io + 4) as usize;
                            let ko = io + be32(d, io + 8) as usize;
                            let stride = if curve == 2 { 12 } else { 8 };
                            let mut track = Track { target: d[io + 1], step: curve != 2, keys: Vec::new() };
                            for n in 0..key_count {
                                let key = ko + n * stride;
                                if key + stride > d.len() {
                                    break;
                                }
                                track.keys.push(if curve == 2 { Key { frame: bef(d, key), value: bef(d, key + 4), slope: bef(d, key + 8) } } else { Key { frame: bef(d, key), value: f32::from(be16(d, key + 4)), slope: 0.0 } });
                            }
                            into.push(track);
                        }
                    }
                    out.panes.push(pane);
                }
            }
            pos += size;
        }
        (!out.panes.is_empty()).then_some(out)
    }

    pub fn frames(&self) -> f32 {
        self.frames
    }

    /// The values at `frame` for every pane it animates (or only those in
    /// `only`: from -> to), written into `params`.
    pub fn apply(&self, frame: f32, layout: &Layout, params: &mut DrawParams, only: &[(&str, &str)]) {
        for pane in &self.panes {
            let target = if only.is_empty() {
                pane.name.as_str()
            } else {
                match only.iter().find(|m| m.0 == pane.name) {
                    Some(m) => m.1,
                    None => continue,
                }
            };
            let Some((mut tx, mut ty, mut sx, mut sy)) = layout.pane_transform(target) else { continue };
            let (mut moved, mut scaled) = (false, false);
            for track in &pane.srt {
                let v = sample(track, frame);
                match track.target {
                    0 => (tx, moved) = (v, true),
                    1 => (ty, moved) = (v, true),
                    6 => (sx, scaled) = (v, true),
                    7 => (sy, scaled) = (v, true),
                    _ => {}
                }
            }
            if moved {
                params.pane_translate.insert(target.into(), (tx, ty));
            }
            if scaled {
                params.pane_scale.insert(target.into(), (sx + sy) / 2.0);
            }
            let mut colors = params.vertex_colors.get(target).copied().unwrap_or([u32::MAX; 4]);
            let mut colored = false;
            for track in &pane.vtx {
                let v = sample(track, frame);
                if track.target == 16 {
                    params.pane_alpha.insert(target.into(), v.clamp(0.0, 255.0) / 255.0);
                } else if track.target < 16 {
                    let (corner, channel) = (track.target as usize / 4, track.target % 4);
                    let byte = v.clamp(0.0, 255.0).round() as u32;
                    let shift = 24 - 8 * u32::from(channel);
                    colors[corner] = (colors[corner] & !(0xFF << shift)) | (byte << shift);
                    colored = true;
                }
            }
            if colored {
                params.vertex_colors.insert(target.into(), colors);
            }
            for track in &pane.vis {
                if track.target == 0 {
                    params.visible.insert(target.into(), sample(track, frame) != 0.0);
                }
            }
        }
    }
}
