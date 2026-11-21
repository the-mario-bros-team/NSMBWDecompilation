//! Key caps for the Keybinds tab, made from the player's own disc (after
//! WitherBlitz's NSMB-Wii-Recompilation, `key_caps.cpp`): the picture
//! font's B button with its letter taken out is a key cap, stretched to fit
//! its label; its 1 button with the digit taken out is a round face button.
//! Labels are in the game's message font: dark grey on a key cap, the Xbox
//! colours on a controller's face buttons.

use egui::TextureHandle;

use super::layout::{self, DrawList, Font, Rgba, View};
use input::keymap::Key;
use input::padmap::PadInput;

/// The picture font's B and 1 buttons.
const CODE_B: char = '#';
const CODE_ONE: char = '\'';
/// The B button's letter's grey (labels on key caps).
const LABEL: u32 = 0x3131_31FF;
/// A key cap's edges in texels, which keep their size when it is stretched:
/// the border, the face's top band, its bottom curve.
const EDGE_X: f32 = 5.0;
const EDGE_TOP: f32 = 7.0;
const EDGE_BOTTOM: f32 = 8.0;

/// One thing a cap shows.
#[derive(Clone, Debug, PartialEq)]
pub enum Glyph {
    /// A key cap with a word on it.
    Key(String),
    /// A key cap with an arrow: 0 up, 1 down, 2 left, 3 right.
    Arrow(u8),
    /// A round face button with a coloured word.
    Round(String, u32),
}

/// A cell of the picture font, its ink's bounds (texels), on the GPU.
struct Piece {
    texture: TextureHandle,
    size: (f32, f32),
    bounds: [f32; 4],
}

pub struct Caps {
    square: Piece,
    round: Piece,
}

fn luma(p: &[u8]) -> f32 {
    0.299 * f32::from(p[0]) + 0.587 * f32::from(p[1]) + 0.114 * f32::from(p[2])
}

/// The ink's bounds in a w x h RGBA cell: x, y, width, height.
fn ink(w: usize, h: usize, rgba: &[u8]) -> Option<[usize; 4]> {
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for y in 0..h {
        for x in 0..w {
            if rgba[(y * w + x) * 4 + 3] > 128 {
                (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x + 1), y1.max(y + 1));
            }
        }
    }
    (x1 > x0).then(|| [x0, y0, x1 - x0, y1 - y0])
}

/// The B button's letter taken out: across the face's middle rows, texels
/// darker than the face (the letter) take the face's colour there, which is
/// the same across a row (it shades from top to bottom).
fn blank_key(w: usize, rgba: &mut [u8], b: [usize; 4]) {
    let [bx, by, bw, bh] = b;
    for y in by + bh / 4..by + bh * 3 / 4 + 1 {
        let row = y * w * 4;
        let face: [u8; 4] = rgba[row + (bx + 3) * 4..][..4].try_into().unwrap();
        let face_luma = luma(&face);
        for x in bx + 4..bx + bw - 4 {
            let at = row + x * 4;
            if luma(&rgba[at..at + 4]) < face_luma - 20.0 {
                rgba[at..at + 4].copy_from_slice(&face);
            }
        }
    }
}

/// The 1 button's digit taken out: inside the ring, each run of grey
/// (the digit; the face is white to blue) takes the colours either side of
/// it, blended across.
fn blank_round(w: usize, rgba: &mut [u8], b: [usize; 4]) {
    let [bx, by, bw, bh] = b;
    let digit = |p: &[u8]| (i32::from(p[0]) - i32::from(p[2])).abs() < 14 && luma(p) < 205.0;
    for y in by + 3..by + bh - 3 {
        let row = y * w * 4;
        let px = |x: usize| row + x * 4;
        // The face: from the first to the last bright texel of the row.
        let bright: Vec<usize> = (bx..bx + bw).filter(|&x| luma(&rgba[px(x)..px(x) + 4]) > 190.0).collect();
        let (Some(&first), Some(&last)) = (bright.first(), bright.last()) else { continue };
        if last - first < 4 {
            continue;
        }
        let mut x = first + 1;
        while x < last {
            if !digit(&rgba[px(x)..px(x) + 4]) {
                x += 1;
                continue;
            }
            let left = x - 1;
            let mut right = x + 1;
            while right < last && digit(&rgba[px(right)..px(right) + 4]) {
                right += 1;
            }
            let (a, c): ([u8; 4], [u8; 4]) = (rgba[px(left)..px(left) + 4].try_into().unwrap(), rgba[px(right)..px(right) + 4].try_into().unwrap());
            for k in x..right {
                let t = (k - left) as f32 / (right - left) as f32;
                for ch in 0..4 {
                    rgba[px(k) + ch] = (f32::from(a[ch]) + (f32::from(c[ch]) - f32::from(a[ch])) * t + 0.5) as u8;
                }
            }
            x = right + 1;
        }
    }
}

impl Caps {
    /// The caps from the picture font (`None` if it lacks the buttons).
    pub fn load(ctx: &egui::Context, picture: &Font) -> Option<Self> {
        let piece = |code: char, blank: fn(usize, &mut [u8], [usize; 4])| -> Option<Piece> {
            let (w, h, mut rgba) = picture.cell_rgba(code as u16)?;
            let (w, h) = (w as usize, h as usize);
            let b = ink(w, h, &rgba)?;
            blank(w, &mut rgba, b);
            Some(Piece {
                texture: layout::upload_rgba(ctx, &format!("key cap {code}"), w as u32, h as u32, &rgba),
                size: (w as f32, h as f32),
                bounds: b.map(|v| v as f32),
            })
        };
        Some(Self { square: piece(CODE_B, blank_key)?, round: piece(CODE_ONE, blank_round)? })
    }

    /// A cap's width at height `h` (layout units).
    pub fn width(&self, message: &Font, g: &Glyph, h: f32) -> f32 {
        let [_, _, bw, bh] = self.square.bounds;
        let narrow = bw / bh * h;
        match g {
            Glyph::Arrow(_) => narrow,
            Glyph::Round(..) => self.round.bounds[2] / self.round.bounds[3] * h,
            Glyph::Key(label) if label.chars().count() <= 1 => narrow,
            Glyph::Key(label) => (layout::measure_text(message, label, h * 0.62) + h * 0.55).max(narrow),
        }
    }

    /// Draws `g` `h` tall with its left edge at `x`, centred on `y`; returns
    /// its width.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(&self, ctx: &egui::Context, list: &mut DrawList, view: &View, message: &Font, g: &Glyph, x: f32, y: f32, h: f32, alpha: f32) -> f32 {
        let w = self.width(message, g, h);
        let tint = Rgba { r: 1.0, g: 1.0, b: 1.0, a: alpha };
        let top = y + h / 2.0;
        match g {
            Glyph::Round(label, color) => {
                let p = &self.round;
                let [bx, by, bw, bh] = p.bounds;
                let uv = [bx / p.size.0, by / p.size.1, (bx + bw) / p.size.0, (by + bh) / p.size.1];
                list.image(view, p.texture.id(), x, top, x + w, top - h, uv, tint);
                let size = h * 0.5;
                let fit = layout::measure_text(message, label, size);
                let size = if fit > w * 0.62 { size * w * 0.62 / fit } else { size };
                layout::draw_text(list, view, message, label, x + w / 2.0, y + h * 0.02, size, *color, *color, alpha, 1, 1);
            }
            _ => {
                self.nine_slice(list, view, x, top, w, h, tint);
                // The face, between its top band and bottom curve.
                let k = h / self.square.bounds[3];
                let face_y = top - (3.0 * k + (h - 8.0 * k)) / 2.0;
                match g {
                    Glyph::Arrow(dir) => {
                        let s = (w.min(h * 0.6)) * 0.3;
                        let cx = x + w / 2.0;
                        let pts = match dir {
                            0 => [(cx, face_y + s), (cx - s, face_y - s * 0.7), (cx + s, face_y - s * 0.7)],
                            1 => [(cx, face_y - s), (cx + s, face_y + s * 0.7), (cx - s, face_y + s * 0.7)],
                            2 => [(cx - s, face_y), (cx + s * 0.7, face_y - s), (cx + s * 0.7, face_y + s)],
                            _ => [(cx + s, face_y), (cx - s * 0.7, face_y + s), (cx - s * 0.7, face_y - s)],
                        };
                        let mut c = Rgba::from_rgba8(LABEL);
                        c.a *= alpha;
                        list.triangle(ctx, view, pts, c);
                    }
                    Glyph::Key(label) => {
                        let mut size = h * 0.62;
                        let room = w - h * 0.3;
                        let fit = layout::measure_text(message, label, size);
                        if fit > room && fit > 0.0 {
                            size *= room / fit;
                        }
                        layout::draw_text(list, view, message, label, x + w / 2.0, face_y, size, LABEL, LABEL, alpha, 1, 1);
                    }
                    Glyph::Round(..) => {}
                }
            }
        }
        w
    }

    /// The key cap stretched to `w` x `h` (top-left at `x`, `top`), its
    /// edges kept their size.
    #[allow(clippy::too_many_arguments)]
    fn nine_slice(&self, list: &mut DrawList, view: &View, x: f32, top: f32, w: f32, h: f32, tint: Rgba) {
        let p = &self.square;
        let [bx, by, bw, bh] = p.bounds;
        let k = h / bh;
        let sx = [bx, bx + EDGE_X, bx + bw - EDGE_X, bx + bw];
        let sy = [by, by + EDGE_TOP, by + bh - EDGE_BOTTOM, by + bh];
        let edge = (EDGE_X * k).min(w / 2.0);
        let dx = [x, x + edge, x + w - edge, x + w];
        let dy = [top, top - EDGE_TOP * k, top - h + EDGE_BOTTOM * k, top - h];
        for j in 0..3 {
            for i in 0..3 {
                let uv = [sx[i] / p.size.0, sy[j] / p.size.1, sx[i + 1] / p.size.0, sy[j + 1] / p.size.1];
                list.image(view, p.texture.id(), dx[i], dy[j], dx[i + 1], dy[j + 1], uv, tint);
            }
        }
    }
}

/// A key's cap: the shortest label that reads as that key.
pub fn key_glyph(k: Key) -> Glyph {
    let name = input::keymap::key_name(k).unwrap_or("?");
    let short = match name {
        "Up" => return Glyph::Arrow(0),
        "Down" => return Glyph::Arrow(1),
        "Left" => return Glyph::Arrow(2),
        "Right" => return Glyph::Arrow(3),
        "Left Shift" | "Right Shift" => "Shift",
        "Left Ctrl" | "Right Ctrl" => "Ctrl",
        "Left Alt" | "Right Alt" => "Alt",
        "Backspace" => "Bksp",
        "Delete" => "Del",
        "Insert" => "Ins",
        "Page Up" => "PgUp",
        "Page Down" => "PgDn",
        "Mouse L" => "LMB",
        "Mouse M" => "MMB",
        "Mouse R" => "RMB",
        "Mouse 4" => "M4",
        "Mouse 5" => "M5",
        n => n,
    };
    let label = match short.strip_prefix("Keypad ") {
        Some(rest) => format!("Num{rest}"),
        None => short.to_string(),
    };
    Glyph::Key(label)
}

/// A controller input's caps, with its Xbox label: the face buttons round
/// in their colours, a stick's way as the stick and an arrow.
pub fn pad_glyphs(p: PadInput) -> Vec<Glyph> {
    use input::padmap::{Axis, Button};
    let round = |l: &str, c: u32| Glyph::Round(l.into(), c);
    let key = |l: &str| Glyph::Key(l.into());
    match p {
        PadInput::Button(b) => vec![match b {
            Button::South => round("A", 0x3C9C_2AFF),
            Button::East => round("B", 0xC830_2AFF),
            Button::West => round("X", 0x2A64_C8FF),
            Button::North => round("Y", 0xC896_00FF),
            Button::LeftThumb => round("LS", 0x3131_31FF),
            Button::RightThumb => round("RS", 0x3131_31FF),
            Button::DPadUp => Glyph::Arrow(0),
            Button::DPadDown => Glyph::Arrow(1),
            Button::DPadLeft => Glyph::Arrow(2),
            Button::DPadRight => Glyph::Arrow(3),
            other => key(input::padmap::name(PadInput::Button(other)).unwrap_or("?")),
        }],
        PadInput::Stick(axis, positive) => {
            let stick = if matches!(axis, Axis::LeftStickX | Axis::LeftStickY) { "LS" } else { "RS" };
            let dir = match (axis, positive) {
                (Axis::LeftStickY | Axis::RightStickY, true) => 0,
                (Axis::LeftStickY | Axis::RightStickY, false) => 1,
                (_, false) => 2,
                (_, true) => 3,
            };
            vec![round(stick, 0x3131_31FF), Glyph::Arrow(dir)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::KeyCode;

    #[test]
    fn caps_read_as_their_keys() {
        assert_eq!(key_glyph(Key::Code(KeyCode::ControlLeft)), Glyph::Key("Ctrl".into()));
        assert_eq!(key_glyph(Key::Code(KeyCode::ArrowLeft)), Glyph::Arrow(2));
        assert_eq!(key_glyph(Key::Code(KeyCode::Numpad7)), Glyph::Key("Num7".into()));
        assert_eq!(key_glyph(Key::Mouse(3)), Glyph::Key("RMB".into()));
        assert_eq!(pad_glyphs(input::padmap::by_name("A").unwrap()), vec![Glyph::Round("A".into(), 0x3C9C_2AFF)]);
        assert_eq!(pad_glyphs(input::padmap::by_name("Right Stick Left").unwrap()), vec![Glyph::Round("RS".into(), 0x3131_31FF), Glyph::Arrow(2)]);
        assert_eq!(pad_glyphs(input::padmap::by_name("LT").unwrap()), vec![Glyph::Key("LT".into())]);
    }

    #[test]
    fn letters_come_out_of_the_buttons() {
        // A 12 x 12 cell: a black border, a light face, a dark letter.
        let (w, h) = (12usize, 12usize);
        let mut rgba = vec![0u8; w * h * 4];
        for y in 0..h {
            for x in 0..w {
                let border = x == 0 || y == 0 || x == w - 1 || y == h - 1;
                let c = if border { [0, 0, 0, 255] } else if (5..7).contains(&x) && (4..8).contains(&y) { [60, 60, 60, 255] } else { [230, 235, 240, 255] };
                rgba[(y * w + x) * 4..][..4].copy_from_slice(&c);
            }
        }
        let b = ink(w, h, &rgba).unwrap();
        assert_eq!(b, [0, 0, 12, 12]);
        blank_key(w, &mut rgba, b);
        assert!((0..h).all(|y| (1..w - 1).all(|x| y == 0 || y == h - 1 || rgba[(y * w + x) * 4] > 200)), "no letter left");
    }
}
