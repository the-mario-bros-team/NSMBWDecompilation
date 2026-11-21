//! The game's own button prompts drawn as the player's keys or controller
//! buttons (after WitherBlitz's NSMB-Wii-Recompilation, `key_caps.cpp` and
//! `prompt_style.cpp`). The picture font's sheet with the Wii Remote's 2, 1,
//! A, +, - and D-pad, and the two prompts the game draws as textures of
//! their own (`os_2btn_00`, the strap screen's and the letters' 2;
//! `ImIconjujiButton`, the world map guides' D-pad), are painted again from
//! the player's disc with [`super::key_caps`]' caps, 4 times larger, and
//! handed to the texture-replacement hook (`gx::replace`) keyed by the
//! textures they stand in for: only what is drawn changes, never the game's
//! memory.
//!
//! The keyboard shows each action's first binding (a combination's
//! modifiers on top), and the D-pad as four keys when each direction is one
//! key; a controller shows its buttons and keeps the game's D-pad.

use std::collections::HashMap;
use std::sync::Arc;

use gx::replace::{Image, Key};
use input::keymap::{self, Keymap};
use input::padmap::Padmap;

use super::key_caps::{self, Glyph};
use super::layout;

/// What the prompts show: the game's own art (a Wii Remote's), the keys,
/// or a controller's buttons.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Style {
    Game,
    Keys,
    Pad,
}

/// Output pixels per texel of the game's art.
const SCALE: usize = 4;
const PICTURE: &str = "/US/EngUS/Font/mj2d00_PictureFont_32_RGBA8.brfnt";
const MESSAGE: &str = "/US/EngUS/Font/mj2d00_MessageFont_32_I4.brfnt";
/// The picture font's Wii Remote buttons.
const CODE_DPAD: char = '!';
const CODE_A: char = '"';
const CODE_PLUS: char = '%';
const CODE_MINUS: char = '&';
const CODE_TWO: char = '(';
/// Prompts that are textures of their own: an archive holding one, the
/// texture, and the font cell whose contents they show. The same texture in
/// other archives is the same image, so one replacement covers them all.
const ICONS: [(&str, &str, char); 2] = [
    ("/Layout/controllerInformation/controllerInformation.arc", "os_2btn_00.tpl", CODE_TWO),
    ("/Layout/worldSelectMapGuide/worldSelectMapGuide.arc", "ImIconjujiButton.tpl", CODE_DPAD),
];

// ---------------------------------------------------------------- pixels

/// An image as read: RGBA8, rows top down, alpha not premultiplied.
struct Pixels {
    w: usize,
    h: usize,
    rgba: Vec<u8>,
}

/// What is being painted: premultiplied RGBA.
struct Canvas {
    w: usize,
    h: usize,
    px: Vec<[f32; 4]>,
}

impl Canvas {
    fn new(w: usize, h: usize) -> Self {
        Self { w, h, px: vec![[0.0; 4]; w * h] }
    }

    fn over(&mut self, x: usize, y: usize, s: [f32; 4]) {
        let d = &mut self.px[y * self.w + x];
        for i in 0..4 {
            d[i] = s[i] + d[i] * (1.0 - s[3]);
        }
    }

    fn clear(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for y in y0..y1.min(self.h) {
            for x in x0..x1.min(self.w) {
                self.px[y * self.w + x] = [0.0; 4];
            }
        }
    }

    /// Back to straight RGBA8.
    fn into_rgba(self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.px.len() * 4);
        for [r, g, b, a] in self.px {
            let un = |c: f32| if a > 0.0 { (c / a * 255.0 + 0.5).clamp(0.0, 255.0) as u8 } else { 0 };
            out.extend([un(r), un(g), un(b), (a * 255.0 + 0.5).clamp(0.0, 255.0) as u8]);
        }
        out
    }
}

/// `src` at texel coordinates (x, y), bilinear, premultiplied, kept inside
/// `clip` (x0, y0, x1, y1) so nothing bleeds in from the next cell.
fn sample(src: &Pixels, x: f32, y: f32, clip: [usize; 4]) -> [f32; 4] {
    let (x, y) = (x - 0.5, y - 0.5);
    let (fx, fy) = (x.floor(), y.floor());
    let (tx, ty) = (x - fx, y - fy);
    let texel = |ix: f32, iy: f32| {
        let cx = (ix.max(clip[0] as f32) as usize).min(clip[2] - 1);
        let cy = (iy.max(clip[1] as f32) as usize).min(clip[3] - 1);
        let p = &src.rgba[(cy * src.w + cx) * 4..][..4];
        let a = f32::from(p[3]) / 255.0;
        [f32::from(p[0]) / 255.0 * a, f32::from(p[1]) / 255.0 * a, f32::from(p[2]) / 255.0 * a, a]
    };
    let (p00, p10, p01, p11) = (texel(fx, fy), texel(fx + 1.0, fy), texel(fx, fy + 1.0), texel(fx + 1.0, fy + 1.0));
    std::array::from_fn(|i| (p00[i] * (1.0 - tx) + p10[i] * tx) * (1.0 - ty) + (p01[i] * (1.0 - tx) + p11[i] * tx) * ty)
}

/// Draws `src`'s rectangle `s` (x, y, w, h texels) on `dst`'s rectangle `d`
/// (pixels), over what is there. `ink`: use only the source's alpha, in
/// this colour (a font's letters).
fn blit(dst: &mut Canvas, src: &Pixels, s: [f32; 4], d: [f32; 4], clip: [usize; 4], ink: Option<[f32; 4]>) {
    if d[2] <= 0.0 || d[3] <= 0.0 {
        return;
    }
    let (x0, y0) = (d[0].floor().max(0.0) as usize, d[1].floor().max(0.0) as usize);
    let (x1, y1) = (((d[0] + d[2]).ceil().max(0.0) as usize).min(dst.w), ((d[1] + d[3]).ceil().max(0.0) as usize).min(dst.h));
    for py in y0..y1 {
        let cy = py as f32 + 0.5;
        if cy < d[1] || cy >= d[1] + d[3] {
            continue;
        }
        for px in x0..x1 {
            let cx = px as f32 + 0.5;
            if cx < d[0] || cx >= d[0] + d[2] {
                continue;
            }
            let mut c = sample(src, s[0] + (cx - d[0]) / d[2] * s[2], s[1] + (cy - d[1]) / d[3] * s[3], clip);
            if let Some([r, g, b, a]) = ink {
                let k = c[3] * a;
                c = [r * k, g * k, b * k, k];
            }
            dst.over(px, py, c);
        }
    }
}

fn rgba(c: u32) -> [f32; 4] {
    [(c >> 24) as f32 / 255.0, (c >> 16 & 0xFF) as f32 / 255.0, (c >> 8 & 0xFF) as f32 / 255.0, (c & 0xFF) as f32 / 255.0]
}

// ---------------------------------------------------------------- the disc's fonts

fn be16(d: &[u8], at: usize) -> usize {
    d.get(at..at + 2).map_or(0, |b| usize::from(u16::from_be_bytes([b[0], b[1]])))
}
fn be32(d: &[u8], at: usize) -> usize {
    d.get(at..at + 4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize)
}

/// A glyph's widths: its ink's left edge, its ink's width, its advance.
#[derive(Clone, Copy, Default)]
struct Widths {
    left: f32,
    advance: f32,
}

/// A BRFNT's sheets as pixels, and where each character is: what the
/// painting needs (`layout::Font` keeps them on the GPU instead).
struct FontPixels {
    height: f32,
    ascent: f32,
    cell_w: usize,
    cell_h: usize,
    per_row: usize,
    per_sheet: usize,
    format: u32,
    sheets: Vec<Pixels>,
    /// The sheets' bytes as the game hands them to the GPU (their keys).
    raw: Vec<Vec<u8>>,
    widths: Vec<Widths>,
    map: HashMap<u16, usize>,
}

impl FontPixels {
    fn parse(d: &[u8]) -> Option<Self> {
        if d.get(..4) != Some(b"RFNT") {
            return None;
        }
        let f = be16(d, 12) + 8;
        let (tglp, mut cwdh, mut cmap) = (be32(d, f + 8), be32(d, f + 12), be32(d, f + 16));
        let (height, ascent) = (f32::from(*d.get(f + 20)?), f32::from(*d.get(f + 22)?));
        let t = tglp;
        let (cell_w, cell_h) = (usize::from(*d.get(t)?), usize::from(*d.get(t + 1)?));
        let (sheet_size, count, format) = (be32(d, t + 4), be16(d, t + 8), be16(d, t + 10) as u32);
        let (per_row, per_col, w, h, at) = (be16(d, t + 12), be16(d, t + 14), be16(d, t + 16), be16(d, t + 18), be32(d, t + 20));
        let mut font = Self { height, ascent, cell_w, cell_h, per_row, per_sheet: per_row * per_col, format, sheets: Vec::new(), raw: Vec::new(), widths: Vec::new(), map: HashMap::new() };
        for i in 0..count {
            let raw = d.get(at + i * sheet_size..at + (i + 1) * sheet_size)?.to_vec();
            font.sheets.push(Pixels { w, h, rgba: gx::texture::decode(format, w, h, &raw, &[], 0) });
            font.raw.push(raw);
        }
        while cwdh != 0 && cwdh + 8 <= d.len() {
            let (first, last) = (be16(d, cwdh), be16(d, cwdh + 2));
            for index in first..=last {
                let e = d.get(cwdh + 8 + (index - first) * 3..cwdh + 11 + (index - first) * 3)?;
                if index >= font.widths.len() {
                    font.widths.resize(index + 1, Widths::default());
                }
                font.widths[index] = Widths { left: f32::from(e[0] as i8), advance: f32::from(e[2] as i8) };
            }
            cwdh = be32(d, cwdh + 4);
        }
        while cmap != 0 && cmap + 12 <= d.len() {
            let (begin, end, method, body) = (be16(d, cmap), be16(d, cmap + 2), be16(d, cmap + 4), cmap + 12);
            match method {
                0 => (begin..=end).for_each(|c| {
                    font.map.insert(c as u16, c - begin + be16(d, body));
                }),
                1 => (begin..=end).for_each(|c| {
                    let i = be16(d, body + (c - begin) * 2);
                    if i != 0xFFFF {
                        font.map.insert(c as u16, i);
                    }
                }),
                2 => (0..be16(d, body)).for_each(|i| {
                    font.map.insert(be16(d, body + 2 + i * 4) as u16, be16(d, body + 4 + i * 4));
                }),
                _ => {}
            }
            cmap = be32(d, cmap + 8);
        }
        (!font.sheets.is_empty() && font.per_sheet != 0).then_some(font)
    }

    /// Where `c`'s cell is: its sheet, and its top-left texel there.
    fn cell(&self, c: char) -> Option<(usize, usize, usize)> {
        let index = *self.map.get(&u16::try_from(u32::from(c)).ok()?)?;
        let (sheet, at) = (index / self.per_sheet, index % self.per_sheet);
        Some((sheet, at % self.per_row * (self.cell_w + 1), at / self.per_row * (self.cell_h + 1)))
    }

    /// A cell's pixels on their own (w, h, RGBA).
    fn cell_pixels(&self, c: char) -> Option<Pixels> {
        let (sheet, x, y) = self.cell(c)?;
        let s = self.sheets.get(sheet)?;
        let mut rgba = Vec::with_capacity(self.cell_w * self.cell_h * 4);
        for row in y..y + self.cell_h {
            rgba.extend_from_slice(s.rgba.get((row * s.w + x) * 4..(row * s.w + x + self.cell_w) * 4)?);
        }
        Some(Pixels { w: self.cell_w, h: self.cell_h, rgba })
    }

    /// `text`'s letters, a capital `letter` pixels tall, centred on (cx,
    /// cy), no wider than `max_w` (smaller down to 60%, then squeezed).
    fn label(&self, dst: &mut Canvas, text: &str, cx: f32, cy: f32, letter: f32, max_w: f32, color: u32) {
        // A capital is about 0.62 of the font's height, its middle 0.31
        // above the baseline (the ascent below the cell's top).
        let mut k = letter / (0.62 * self.height);
        let glyphs: Vec<(usize, usize, usize, Widths)> = text.chars().filter_map(|c| self.cell(c).map(|(s, x, y)| (s, x, y, self.widths.get(*self.map.get(&(c as u16)).unwrap_or(&0)).copied().unwrap_or_default()))).collect();
        let width: f32 = glyphs.iter().map(|g| g.3.advance).sum();
        let mut squeeze = 1.0;
        if width * k > max_w && width > 0.0 {
            k *= (max_w / (width * k)).max(0.6);
            if width * k > max_w {
                squeeze = max_w / (width * k);
            }
        }
        let mut pen = cx - width * k * squeeze / 2.0;
        let top = cy - (self.ascent - 0.31 * self.height) * k;
        let ink = Some(rgba(color));
        for (sheet, x, y, w) in glyphs {
            let clip = [x, y, x + self.cell_w, y + self.cell_h];
            let d = [pen + w.left * k * squeeze, top, self.cell_w as f32 * k * squeeze, self.cell_h as f32 * k];
            blit(dst, &self.sheets[sheet], [x as f32, y as f32, self.cell_w as f32, self.cell_h as f32], d, clip, ink);
            pen += w.advance * k * squeeze;
        }
    }
}

/// A cell of the picture font with its letter or digit taken out, and the
/// part its ink covers (x, y, w, h).
struct Piece {
    px: Pixels,
    bounds: [usize; 4],
}

impl Piece {
    fn load(picture: &FontPixels, code: char, blank: fn(usize, &mut [u8], [usize; 4])) -> Option<Self> {
        let mut px = picture.cell_pixels(code)?;
        let bounds = key_caps::ink(px.w, px.h, &px.rgba)?;
        blank(px.w, &mut px.rgba, bounds);
        Some(Self { px, bounds })
    }

    fn all(&self) -> [usize; 4] {
        [0, 0, self.px.w, self.px.h]
    }
}

// ---------------------------------------------------------------- what a cell shows

/// One button prompt: the picture font's character, the caps it shows (top
/// to bottom; none: the game's own), and whether they are the D-pad's four
/// keys (one on top, three under it).
#[derive(Clone, Debug, PartialEq)]
pub struct Content {
    code: char,
    glyphs: Vec<Glyph>,
    cluster: bool,
}

fn is_modifier(k: keymap::Key) -> bool {
    keymap::key_name(k).is_some_and(|n| n.contains("Ctrl") || n.contains("Shift") || n.contains("Alt"))
}

/// An action's first key, or else its first combination's (modifiers on
/// top, as a combination is read: "Ctrl + X").
fn first_keys(k: &Keymap, action: usize) -> Vec<keymap::Key> {
    let mut keys = k.keys[action].first().map(|&k| vec![k]).or_else(|| k.combos[action].first().cloned()).unwrap_or_default();
    keys.sort_by_key(|&k| !is_modifier(k));
    keys
}

/// An action's first button, or else its first combination's.
fn first_buttons(p: &Padmap, action: usize) -> Vec<Glyph> {
    match p.buttons[action].first() {
        Some(&b) => key_caps::pad_glyphs(b),
        None => p.combos[action].first().map(|c| c.iter().flat_map(|&b| key_caps::pad_glyphs(b)).collect()).unwrap_or_default(),
    }
}

/// What the prompts show for `style` with these bindings.
pub fn contents(style: Style, k: &Keymap, p: &Padmap) -> Vec<Content> {
    let cells = [(CODE_TWO, keymap::JUMP), (key_caps::CODE_ONE, keymap::RUN), (CODE_A, keymap::A), (CODE_PLUS, keymap::PLUS), (CODE_MINUS, keymap::MINUS)];
    let mut out: Vec<Content> = cells.iter().map(|&(code, _)| Content { code, glyphs: Vec::new(), cluster: false }).collect();
    out.push(Content { code: CODE_DPAD, glyphs: Vec::new(), cluster: false });
    match style {
        Style::Game => {}
        Style::Keys => {
            for (c, &(_, action)) in out.iter_mut().zip(&cells) {
                c.glyphs = first_keys(k, action).into_iter().map(key_caps::key_glyph).collect();
            }
            // The D-pad: the four direction keys, when each is one key.
            let dirs: Vec<Vec<keymap::Key>> = [keymap::UP, keymap::DOWN, keymap::LEFT, keymap::RIGHT].iter().map(|&a| first_keys(k, a)).collect();
            if dirs.iter().all(|d| d.len() == 1) {
                out[5].glyphs = dirs.into_iter().map(|d| key_caps::key_glyph(d[0])).collect();
                out[5].cluster = true;
            }
        }
        // A controller's D-pad (and stick) moves as the game's own shows.
        Style::Pad => {
            for (c, &(_, action)) in out.iter_mut().zip(&cells) {
                c.glyphs = first_buttons(p, action);
            }
        }
    }
    out
}

// ---------------------------------------------------------------- painting

/// The game's art the prompts are painted from.
pub struct Prompts {
    picture: FontPixels,
    message: FontPixels,
    square: Piece,
    round: Piece,
    /// The picture font's sheet with the buttons: its key and index.
    sheet: (Key, usize),
    icons: Vec<Icon>,
}

/// A prompt that is a texture of its own.
struct Icon {
    key: Key,
    px: Pixels,
    /// Where its art is (x, y, w, h).
    bounds: [usize; 4],
    code: char,
}

/// The single image of a TPL file: its key and pixels.
fn tpl(data: &[u8]) -> Option<(Key, Pixels)> {
    if be32(data, 0) != 0x0020_AF30 {
        return None;
    }
    let h = be32(data, be32(data, 8));
    let (height, width, format, at) = (be16(data, h), be16(data, h + 2), be32(data, h + 4) as u32, be32(data, h + 8));
    let raw = data.get(at..at + gx::texture::level_size(format, width, height))?;
    Some((gx::replace::key(format, width as u32, height as u32, raw, &[]), Pixels { w: width, h: height, rgba: gx::texture::decode(format, width, height, raw, &[], 0) }))
}

impl Prompts {
    /// The prompts' art from the player's disc (`None` if it is not there).
    pub fn load() -> Option<Self> {
        let picture = FontPixels::parse(&layout::read_disc_file(PICTURE)?)?;
        let message = FontPixels::parse(&layout::read_disc_file(MESSAGE)?)?;
        let square = Piece::load(&picture, key_caps::CODE_B, key_caps::blank_key)?;
        let round = Piece::load(&picture, key_caps::CODE_ONE, key_caps::blank_round)?;
        let (index, _, _) = picture.cell(CODE_TWO)?;
        let s = &picture.sheets[index];
        let key = gx::replace::key(picture.format, s.w as u32, s.h as u32, &picture.raw[index], &[]);
        let mut icons = Vec::new();
        for (arc, name, code) in ICONS {
            let Some(data) = layout::read_disc_file(arc) else { continue };
            let Some((_, at, size)) = layout::list_u8(&data).into_iter().find(|(p, _, _)| p.rsplit('/').next() == Some(name)) else { continue };
            let Some((key, px)) = data.get(at as usize..(at + size) as usize).and_then(tpl) else { continue };
            let Some(bounds) = key_caps::ink(px.w, px.h, &px.rgba) else { continue };
            icons.push(Icon { key, px, bounds, code });
        }
        Some(Self { picture, message, square, round, sheet: (key, index), icons })
    }

    /// The replacement images for `contents` (none for the game's own).
    pub fn paint(&self, contents: &[Content]) -> Vec<(Key, Arc<Image>)> {
        let mut out = Vec::new();
        if contents.iter().all(|c| c.glyphs.is_empty()) {
            return out;
        }
        let (key, index) = self.sheet;
        let src = &self.picture.sheets[index];
        let (cw, ch) = (self.picture.cell_w, self.picture.cell_h);
        let mut sheet = Canvas::new(src.w * SCALE, src.h * SCALE);
        // The game's own cells, each made larger on its own.
        for y in (0..src.h).step_by(ch + 1).take_while(|y| y + ch <= src.h) {
            for x in (0..src.w).step_by(cw + 1).take_while(|x| x + cw <= src.w) {
                let s = SCALE as f32;
                blit(&mut sheet, src, [x as f32, y as f32, cw as f32, ch as f32], [x as f32 * s, y as f32 * s, cw as f32 * s, ch as f32 * s], [x, y, x + cw, y + ch], None);
            }
        }
        for c in contents.iter().filter(|c| !c.glyphs.is_empty()) {
            let Some((_, x, y)) = self.picture.cell(c.code).filter(|cell| cell.0 == index) else { continue };
            sheet.clear(x * SCALE, y * SCALE, (x + cw) * SCALE, (y + ch) * SCALE);
            self.cell(&mut sheet, c, (x * SCALE) as f32, (y * SCALE) as f32, (cw * SCALE) as f32, SCALE as f32);
        }
        out.push((key, Arc::new(Image::with_mips(sheet.w as u32, sheet.h as u32, sheet.into_rgba()))));
        // The prompts that are textures of their own: what their cell shows,
        // fitted to the square where the game's art is.
        for icon in &self.icons {
            let Some(c) = contents.iter().find(|c| c.code == icon.code && !c.glyphs.is_empty()) else { continue };
            let mut canvas = Canvas::new(icon.px.w * SCALE, icon.px.h * SCALE);
            let [bx, by, bw, bh] = icon.bounds;
            let side = bw.min(bh) as f32;
            let (x0, y0) = (bx as f32 + (bw as f32 - side) / 2.0, by as f32 + (bh as f32 - side) / 2.0);
            let s = SCALE as f32;
            self.cell(&mut canvas, c, x0 * s, y0 * s, side * s, s * side / cw as f32);
            out.push((icon.key, Arc::new(Image::with_mips(canvas.w as u32, canvas.h as u32, canvas.into_rgba()))));
        }
        out
    }

    /// A cell's caps in the square at (x, y), `cell` pixels across, `scale`
    /// pixels a texel of the font's art.
    fn cell(&self, dst: &mut Canvas, c: &Content, x: f32, y: f32, cell: f32, scale: f32) {
        let pad = scale;
        let g = &c.glyphs;
        if c.cluster && g.len() == 4 {
            let (w, h) = ((cell - 2.0 * pad) / 3.0, (cell - 2.0 * pad) / 2.0);
            self.glyph(dst, &g[0], x + pad + w, y + pad, w, h, scale * 0.5);
            self.glyph(dst, &g[2], x + pad, y + pad + h, w, h, scale * 0.5);
            self.glyph(dst, &g[1], x + pad + w, y + pad + h, w, h, scale * 0.5);
            self.glyph(dst, &g[3], x + pad + 2.0 * w, y + pad + h, w, h, scale * 0.5);
            return;
        }
        let n = g.len().min(3);
        if n == 1 {
            // A one-letter key is as square as the game's own B button; a
            // word takes the cell's width.
            let [_, by, bw, bh] = self.square.bounds.map(|v| v as f32);
            let narrow = matches!(&g[0], Glyph::Arrow(_)) || matches!(&g[0], Glyph::Key(l) if l.chars().count() <= 1);
            let w = if narrow { bw * scale } else { cell - 2.0 * pad };
            self.glyph(dst, &g[0], x + (cell - w) / 2.0, y + by * scale, w, bh * scale, scale);
            return;
        }
        let h = (cell - 2.0 * pad) / n as f32;
        for (i, glyph) in g.iter().take(n).enumerate() {
            self.glyph(dst, glyph, x + pad, y + pad + h * i as f32, cell - 2.0 * pad, h, scale * if n == 2 { 0.55 } else { 0.4 });
        }
    }

    /// One cap in the rectangle (x, y, w, h), pixels.
    #[allow(clippy::too_many_arguments)]
    fn glyph(&self, dst: &mut Canvas, g: &Glyph, x: f32, y: f32, w: f32, h: f32, scale: f32) {
        match g {
            Glyph::Round(label, color) => {
                let r = &self.round;
                let [bx, by, bw, bh] = r.bounds.map(|v| v as f32);
                let s = (w / bw).min(h / bh);
                let (rx, ry) = (x + (w - bw * s) / 2.0, y + (h - bh * s) / 2.0);
                blit(dst, &r.px, [bx, by, bw, bh], [rx, ry, bw * s, bh * s], r.all(), None);
                self.message.label(dst, label, rx + bw * s / 2.0, ry + bh * s * 0.47, bh * s * 0.42, bw * s * 0.62, *color);
            }
            _ => {
                self.nine_slice(dst, x, y, w, h, scale);
                let face_top = y + (3.0 * scale).min(h * 0.15);
                let face_bottom = y + h - (5.0 * scale).min(h * 0.2);
                let cy = (face_top + face_bottom) / 2.0;
                match g {
                    Glyph::Arrow(dir) => arrow(dst, x + w / 2.0, cy, w.min(face_bottom - face_top) * 0.62, *dir, rgba(key_caps::LABEL)),
                    Glyph::Key(label) => {
                        let letter = ((face_bottom - face_top) * 0.62).min(13.0 * scale);
                        self.message.label(dst, label, x + w / 2.0, cy, letter, w - 5.0 * scale, key_caps::LABEL);
                    }
                    Glyph::Round(..) => {}
                }
            }
        }
    }

    /// The key cap stretched to (x, y, w, h); its edges keep their size
    /// (`scale` pixels a texel), less when the cap is smaller than them.
    fn nine_slice(&self, dst: &mut Canvas, x: f32, y: f32, w: f32, h: f32, scale: f32) {
        let p = &self.square;
        let [bx, by, bw, bh] = p.bounds.map(|v| v as f32);
        let (l, t, b) = (key_caps::EDGE_X, key_caps::EDGE_TOP, key_caps::EDGE_BOTTOM);
        let kx = if 2.0 * l * scale > w { w / (2.0 * l) } else { scale };
        let ky = if (t + b) * scale > h { h / (t + b) } else { scale };
        let sx = [bx, bx + l, bx + bw - l, bx + bw];
        let sy = [by, by + t, by + bh - b, by + bh];
        let dx = [x, x + l * kx, x + w - l * kx, x + w];
        let dy = [y, y + t * ky, y + h - b * ky, y + h];
        let clip = [p.bounds[0], p.bounds[1], p.bounds[0] + p.bounds[2], p.bounds[1] + p.bounds[3]];
        for j in 0..3 {
            for i in 0..3 {
                blit(dst, &p.px, [sx[i], sy[j], sx[i + 1] - sx[i], sy[j + 1] - sy[j]], [dx[i], dy[j], dx[i + 1] - dx[i], dy[j + 1] - dy[j]], clip, None);
            }
        }
    }
}

/// A filled triangle pointing `dir` (0 up, 1 down, 2 left, 3 right), `size`
/// across, centred on (cx, cy), 16 samples a pixel.
fn arrow(dst: &mut Canvas, cx: f32, cy: f32, size: f32, dir: u8, color: [f32; 4]) {
    let h = size / 2.0;
    let pts = match dir {
        0 => [(cx, cy - h), (cx + h, cy + h * 0.7), (cx - h, cy + h * 0.7)],
        1 => [(cx, cy + h), (cx - h, cy - h * 0.7), (cx + h, cy - h * 0.7)],
        2 => [(cx - h, cy), (cx + h * 0.7, cy - h), (cx + h * 0.7, cy + h)],
        _ => [(cx + h, cy), (cx - h * 0.7, cy + h), (cx - h * 0.7, cy - h)],
    };
    let edge = |a: (f32, f32), b: (f32, f32), x: f32, y: f32| (b.0 - a.0) * (y - a.1) - (b.1 - a.1) * (x - a.0);
    let area = edge(pts[0], pts[1], pts[2].0, pts[2].1);
    let (x0, x1) = ((cx - h).floor().max(0.0) as usize, ((cx + h).ceil() as usize + 1).min(dst.w));
    let (y0, y1) = ((cy - h).floor().max(0.0) as usize, ((cy + h).ceil() as usize + 1).min(dst.h));
    for y in y0..y1 {
        for x in x0..x1 {
            let mut inside = 0;
            for s in 0..16 {
                let (px, py) = (x as f32 + (s % 4) as f32 / 4.0 + 0.125, y as f32 + (s / 4) as f32 / 4.0 + 0.125);
                let e = [edge(pts[0], pts[1], px, py), edge(pts[1], pts[2], px, py), edge(pts[2], pts[0], px, py)];
                inside += usize::from(if area > 0.0 { e.iter().all(|&v| v >= 0.0) } else { e.iter().all(|&v| v <= 0.0) });
            }
            if inside > 0 {
                let k = color[3] * inside as f32 / 16.0;
                dst.over(x, y, [color[0] * k, color[1] * k, color[2] * k, k]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::KeyCode;

    #[test]
    fn what_the_prompts_show() {
        let mut k = Keymap::default();
        let p = Padmap::default();
        assert!(contents(Style::Game, &k, &p).iter().all(|c| c.glyphs.is_empty()));
        let keys = contents(Style::Keys, &k, &p);
        assert_eq!(keys[0], Content { code: CODE_TWO, glyphs: vec![Glyph::Key("X".into())], cluster: false }, "the first binding");
        assert_eq!(keys[5].glyphs, vec![Glyph::Arrow(0), Glyph::Arrow(1), Glyph::Arrow(2), Glyph::Arrow(3)]);
        assert!(keys[5].cluster);
        // A combination alone: its modifier on top.
        k.keys[keymap::A].clear();
        k.take(keymap::A, vec![KeyCode::KeyV.into(), KeyCode::ControlLeft.into()]);
        k.keys[keymap::UP].clear();
        let keys = contents(Style::Keys, &k, &p);
        assert_eq!(keys[2].glyphs, vec![Glyph::Key("Ctrl".into()), Glyph::Key("V".into())]);
        assert!(keys[5].glyphs.is_empty(), "a direction without its own key: the game's D-pad");
        let pad = contents(Style::Pad, &k, &p);
        assert_eq!(pad[0].glyphs, vec![Glyph::Round("A".into(), 0x3C9C_2AFF)]);
        assert_eq!(pad[3].glyphs, vec![Glyph::Key("Menu".into())]);
        assert!(pad[2].glyphs.is_empty() && pad[5].glyphs.is_empty(), "A unbound, and the game's own D-pad");
    }

    #[test]
    fn blits_scale_and_stay_in_their_cell() {
        // Two 2x2 cells side by side, red and blue.
        let mut rgba = Vec::new();
        for _ in 0..2 {
            rgba.extend([255, 0, 0, 255, 255, 0, 0, 255, 0, 0, 255, 255, 0, 0, 255, 255]);
        }
        let src = Pixels { w: 4, h: 2, rgba };
        let mut c = Canvas::new(8, 8);
        blit(&mut c, &src, [0.0, 0.0, 2.0, 2.0], [0.0, 0.0, 8.0, 8.0], [0, 0, 2, 2], None);
        assert!(c.px.iter().all(|p| p[0] > 0.99 && p[2] < 0.01 && p[3] > 0.99), "no blue from the next cell");
        // Letters take the ink's colour.
        let mut c = Canvas::new(2, 2);
        blit(&mut c, &src, [0.0, 0.0, 2.0, 2.0], [0.0, 0.0, 2.0, 2.0], [0, 0, 2, 2], Some([0.0, 1.0, 0.0, 1.0]));
        assert_eq!(c.into_rgba()[..4], [0, 255, 0, 255]);
        let mut c = Canvas::new(10, 10);
        arrow(&mut c, 5.0, 5.0, 8.0, 0, [1.0; 4]);
        assert!(c.px[5 * 10 + 5][3] > 0.99 && c.px[0][3] == 0.0);
    }
}
