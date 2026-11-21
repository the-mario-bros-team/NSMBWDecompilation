//! The launcher's own widgets, drawn with egui's painter: text, buttons with
//! a visible focus, icons, button hints, and the focus that the mouse, the
//! keyboard and a controller all move.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use egui::epaint::RectShape;
use egui::text::{LayoutJob, TextFormat};
use egui::{pos2, vec2, Align2, Color32, CornerRadius, FontFamily, FontId, Id, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, TextureId, Vec2};

use crate::nav::{Device, Nav};

// ------------------------------------------------------------------ fonts

/// Font families: Windows' own Segoe UI in four weights, read from the
/// system at start (egui's built-in font stands in for a missing one).
pub const BLACK: &str = "black";
pub const BOLD: &str = "bold";
pub const SEMI: &str = "semi";
pub const REGULAR: &str = "regular";

pub fn install_fonts(ctx: &egui::Context) {
    let mut defs = egui::FontDefinitions::default();
    let fallback = defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let dir = std::path::PathBuf::from(std::env::var_os("WINDIR").unwrap_or_else(|| "C:\\Windows".into())).join("Fonts");
    for (family, file) in [(BLACK, "seguibl.ttf"), (BOLD, "segoeuib.ttf"), (SEMI, "seguisb.ttf"), (REGULAR, "segoeui.ttf")] {
        let mut chain = Vec::new();
        if let Ok(bytes) = std::fs::read(dir.join(file)) {
            defs.font_data.insert(family.to_string(), Arc::new(egui::FontData::from_owned(bytes)));
            chain.push(family.to_string());
        }
        chain.extend(fallback.iter().cloned());
        defs.families.insert(FontFamily::Name(family.into()), chain);
    }
    ctx.set_fonts(defs);
}

pub fn font(size: f32, family: &str) -> FontId {
    FontId::new(size, FontFamily::Name(family.into()))
}

pub fn text(p: &Painter, pos: Pos2, anchor: Align2, s: &str, size: f32, family: &str, color: Color32) -> Rect {
    p.text(pos, anchor, s, font(size, family), color)
}

/// One line with extra space between its letters (labels in capitals).
pub fn spaced(p: &Painter, pos: Pos2, anchor: Align2, s: &str, size: f32, family: &str, color: Color32, spacing: f32) -> Rect {
    let mut job = LayoutJob::single_section(s.to_owned(), TextFormat { font_id: font(size, family), extra_letter_spacing: spacing, color, ..Default::default() });
    job.wrap.max_width = f32::INFINITY;
    let galley = p.layout_job(job);
    // The last letter's spacing is not part of the word.
    let rect = anchor.anchor_size(pos, galley.size() - vec2(spacing, 0.0));
    p.galley(rect.min, galley, color);
    rect
}

/// Text wrapped to `width` from its top-left corner; returns its height.
pub fn wrapped(p: &Painter, pos: Pos2, width: f32, s: &str, size: f32, family: &str, color: Color32) -> f32 {
    let galley = p.layout(s.to_owned(), font(size, family), color, width);
    let height = galley.size().y;
    p.galley(pos, galley, color);
    height
}

pub fn measure(p: &Painter, s: &str, size: f32, family: &str) -> Vec2 {
    p.layout_no_wrap(s.to_owned(), font(size, family), Color32::WHITE).size()
}

/// `s` shortened in the middle with "..." until it fits `width` (for paths).
pub fn fit_middle(p: &Painter, s: &str, size: f32, family: &str, width: f32) -> String {
    if measure(p, s, size, family).x <= width {
        return s.to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    let mut keep = chars.len().saturating_sub(4);
    while keep > 8 {
        let head = keep / 3;
        let short: String = chars[..head].iter().chain("...".chars().collect::<Vec<_>>().iter()).chain(chars[chars.len() - (keep - head)..].iter()).collect();
        if measure(p, &short, size, family).x <= width {
            return short;
        }
        keep -= 2;
    }
    chars[..8.min(chars.len())].iter().collect::<String>() + "..."
}

// ------------------------------------------------------------ small helpers

pub fn white(alpha: f32) -> Color32 {
    Color32::from_white_alpha((alpha.clamp(0.0, 1.0) * 255.0) as u8)
}

pub fn black(alpha: f32) -> Color32 {
    Color32::from_black_alpha((alpha.clamp(0.0, 1.0) * 255.0) as u8)
}

pub fn with_alpha(c: Color32, alpha: f32) -> Color32 {
    c.gamma_multiply(alpha.clamp(0.0, 1.0))
}

/// Ease in and out, 0..1.
pub fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Fast start, gentle stop, 0..1.
pub fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

pub fn scale_rect(r: Rect, s: f32) -> Rect {
    Rect::from_center_size(r.center(), r.size() * s)
}

/// A soft shadow or glow around `rect`.
pub fn glow(p: &Painter, rect: Rect, radius: f32, color: Color32, blur: f32) {
    p.add(RectShape::filled(rect, radius, color).with_blur_width(blur));
}

/// A rounded rectangle filled with a gradient from `a` (top) to `b`.
pub fn gradient_rect(p: &Painter, rect: Rect, radius: impl Into<CornerRadius>, a: Color32, b: Color32, ramp: TextureId) {
    let radius = radius.into();
    p.add(RectShape::filled(rect, radius, a));
    p.add(RectShape::filled(rect, radius, b).with_texture(ramp, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0))));
}

/// A picture with rounded corners, tinted by `tint`.
pub fn picture(p: &Painter, rect: Rect, radius: impl Into<CornerRadius>, texture: TextureId, uv: Rect, tint: Color32) {
    p.add(RectShape::filled(rect, radius, tint).with_texture(texture, uv));
}

pub const FULL_UV: Rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));

// -------------------------------------------------------------- animation

/// Values that glide towards their targets, keyed by name.
#[derive(Default)]
pub struct Anim {
    values: HashMap<Id, f32>,
}

impl Anim {
    /// Moves the value named `id` towards `target` (faster with a higher
    /// `rate`) and returns it. A new value starts at its target.
    pub fn to(&mut self, id: impl Hash + std::fmt::Debug, target: f32, rate: f32, dt: f32) -> f32 {
        let v = self.values.entry(Id::new(id)).or_insert(target);
        *v += (target - *v) * (1.0 - (-rate * dt).exp());
        if (*v - target).abs() < 0.0005 {
            *v = target;
        }
        *v
    }

    pub fn set(&mut self, id: impl Hash + std::fmt::Debug, value: f32) {
        self.values.insert(Id::new(id), value);
    }
}

// ------------------------------------------------------------------ focus

/// Which control of a screen is focused, and where the screen's controls
/// are, so a direction can move the focus to the nearest one that way.
#[derive(Default)]
pub struct Focus {
    pub current: Option<Id>,
    items: Vec<(Id, Rect)>,
}

impl Focus {
    /// Call before drawing the screen's controls.
    pub fn begin(&mut self) {
        self.items.clear();
    }

    fn add(&mut self, id: Id, rect: Rect) {
        self.items.push((id, rect));
    }

    /// A focus that starts on the control `name`.
    pub fn starting_at(name: &str) -> Self {
        Self { current: Some(Id::new(name)), items: Vec::new() }
    }

    pub fn set(&mut self, name: &str) {
        self.current = Some(Id::new(name));
    }

    /// Call after drawing: applies this frame's direction moves, and keeps
    /// the focus on a control that exists.
    pub fn end(&mut self, nav: &[Nav]) {
        if !self.items.iter().any(|i| Some(i.0) == self.current) {
            self.current = self.items.first().map(|i| i.0);
        }
        for n in nav {
            self.step(*n);
        }
    }

    fn step(&mut self, nav: Nav) {
        let dir = match nav {
            Nav::Up => vec2(0.0, -1.0),
            Nav::Down => vec2(0.0, 1.0),
            Nav::Left => vec2(-1.0, 0.0),
            Nav::Right => vec2(1.0, 0.0),
            _ => return,
        };
        let Some(from) = self.items.iter().find(|i| Some(i.0) == self.current).map(|i| i.1) else { return };
        let mut best: Option<(f32, Id)> = None;
        for &(id, rect) in &self.items {
            let delta = rect.center() - from.center();
            // How far past this control's edge the other one starts, that
            // way: only controls that lie beyond it count (a few points of
            // overlap allowed).
            let along = if dir.x > 0.0 {
                rect.left() - from.right()
            } else if dir.x < 0.0 {
                from.left() - rect.right()
            } else if dir.y > 0.0 {
                rect.top() - from.bottom()
            } else {
                from.top() - rect.bottom()
            };
            if Some(id) == self.current || along < -8.0 {
                continue;
            }
            // The gap across the direction: none when the two overlap there.
            let across = if dir.x != 0.0 { (rect.top() - from.bottom()).max(from.top() - rect.bottom()).max(0.0) } else { (rect.left() - from.right()).max(from.left() - rect.right()).max(0.0) };
            let score = along + across * 3.0 + delta.length() * 0.01;
            if best.is_none_or(|b| score < b.0) {
                best = Some((score, id));
            }
        }
        if let Some((_, id)) = best {
            self.current = Some(id);
        }
    }
}

// ---------------------------------------------------------------- context

/// The selected version's colours.
#[derive(Clone, Copy)]
pub struct Palette {
    pub accent: [Color32; 2],
    pub ink: Color32,
}

/// What a screen needs to draw one frame and take input.
pub struct Cx<'a> {
    pub p: Painter,
    pub screen: Rect,
    pub time: f32,
    pub dt: f32,
    /// Whether this screen takes input (it is the top one).
    pub active: bool,
    /// This frame's moves; empty unless `active`.
    pub nav: Vec<Nav>,
    pub pointer: Option<Pos2>,
    /// The mouse moved this frame.
    pub moved: bool,
    pub clicked: bool,
    /// Mouse-wheel scrolling this frame, in points.
    pub wheel: f32,
    pub anim: &'a mut Anim,
    pub pal: Palette,
    pub device: Device,
    /// A transparent-to-white vertical ramp, for gradients.
    pub ramp: TextureId,
}

impl Cx<'_> {
    pub fn pressed(&self, nav: Nav) -> bool {
        self.nav.contains(&nav)
    }

    pub fn hovered(&self, rect: Rect) -> bool {
        self.active && self.pointer.is_some_and(|p| rect.contains(p))
    }

    /// A click or a confirm on the control `name` at `rect`; the mouse over
    /// it moves the focus there.
    pub fn interact(&mut self, focus: &mut Focus, name: &str, rect: Rect) -> (bool, bool) {
        let id = Id::new(name);
        focus.add(id, rect);
        if focus.current.is_none() {
            focus.current = Some(id);
        }
        let hovered = self.hovered(rect);
        if hovered && self.moved {
            focus.current = Some(id);
        }
        let focused = self.active && focus.current == Some(id);
        let clicked = (hovered && self.clicked) || (focused && self.pressed(Nav::Confirm));
        if clicked {
            focus.current = Some(id);
            self.anim.set((name, "press"), 1.0);
        }
        (focus.current == Some(id), clicked)
    }
}

// ---------------------------------------------------------------- buttons

#[derive(Clone, Copy, PartialEq)]
pub enum Look {
    /// The one big action: filled with the version's accent.
    Primary,
    /// A translucent panel.
    Glass,
}

pub struct Button<'a> {
    pub name: &'a str,
    pub rect: Rect,
    pub label: &'a str,
    /// A smaller second line; empty for none.
    pub sub: &'a str,
    pub icon: Option<Icon>,
    pub look: Look,
    /// Looks unavailable; it can still be focused and pressed (to say why).
    pub dim: bool,
    /// A switch that is on.
    pub on: bool,
}

impl<'a> Button<'a> {
    pub fn new(name: &'a str, rect: Rect, label: &'a str) -> Self {
        Self { name, rect, label, sub: "", icon: None, look: Look::Glass, dim: false, on: false }
    }
    pub fn sub(mut self, sub: &'a str) -> Self {
        self.sub = sub;
        self
    }
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn primary(mut self) -> Self {
        self.look = Look::Primary;
        self
    }
    pub fn dim(mut self, dim: bool) -> Self {
        self.dim = dim;
        self
    }
    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self
    }
}

/// Draws a button and returns whether it was pressed this frame.
pub fn button(cx: &mut Cx, focus: &mut Focus, b: Button) -> bool {
    let (focused, clicked) = cx.interact(focus, b.name, b.rect);
    let f = cx.anim.to((b.name, "focus"), if focused && cx.active { 1.0 } else { 0.0 }, 16.0, cx.dt);
    let press = cx.anim.to((b.name, "press"), 0.0, 9.0, cx.dt);
    let rect = scale_rect(b.rect, 1.0 + 0.03 * f - 0.05 * press);
    let big = b.rect.height() >= 70.0;
    let radius = if big { 20.0 } else { 14.0 };
    let p = &cx.p;
    let ink;
    match b.look {
        Look::Primary if !b.dim => {
            glow(p, rect.translate(vec2(0.0, 8.0)), radius, with_alpha(cx.pal.accent[1], 0.30 + 0.45 * f), 30.0 + 16.0 * f);
            gradient_rect(p, rect, radius, cx.pal.accent[0], cx.pal.accent[1], cx.ramp);
            p.rect_stroke(rect.shrink(1.5), radius - 1.5, Stroke::new(1.5, white(0.35)), StrokeKind::Inside);
            ink = cx.pal.ink;
        }
        _ => {
            glow(p, rect.translate(vec2(0.0, 6.0)), radius, black(0.22 + 0.10 * f), 22.0);
            let fill = if b.on { with_alpha(cx.pal.accent[0], 0.30 + 0.10 * f) } else { Color32::from_rgba_unmultiplied(12, 16, 34, (255.0 * (0.46 + 0.14 * f)) as u8) };
            p.rect_filled(rect, radius, fill);
            p.rect_filled(rect, radius, white(0.07 * f));
            let edge = if b.on { with_alpha(cx.pal.accent[0], 0.85) } else { white(0.20) };
            p.rect_stroke(rect, radius, Stroke::new(1.5, edge), StrokeKind::Inside);
            ink = if b.dim { white(0.55) } else { Color32::WHITE };
        }
    }
    if f > 0.01 {
        let pulse = 1.5 * (cx.time * 3.2).sin();
        p.rect_stroke(rect.expand(5.0 + pulse * f), radius + 5.0, Stroke::new(3.0, white(0.95 * f)), StrokeKind::Middle);
    }

    // Icon and words, centred together.
    let primary = b.look == Look::Primary;
    let (label_size, family, tracking, icon_size, gap) = match (primary, big) {
        (true, true) => (25.0, BLACK, 1.4, 30.0, 16.0),
        (false, true) => (15.5, BOLD, 0.8, 24.0, 12.0),
        _ => (15.5, BOLD, 0.4, 20.0, 10.0),
    };
    let label_w = measure(p, b.label, label_size, family).x + (b.label.chars().count() as f32 - 1.0) * tracking;
    let sub_w = if b.sub.is_empty() { 0.0 } else { measure(p, b.sub, 13.5, SEMI).x };
    let words_w = label_w.max(sub_w);
    let total = words_w + if b.icon.is_some() { icon_size + gap } else { 0.0 };
    let mut x = rect.center().x - total / 2.0;
    if let Some(i) = b.icon {
        icon(p, i, pos2(x + icon_size / 2.0, rect.center().y), icon_size, ink);
        x += icon_size + gap;
    }
    let y = rect.center().y - if b.sub.is_empty() { 0.0 } else { 10.0 };
    spaced(p, pos2(x, y), Align2::LEFT_CENTER, b.label, label_size, family, ink, tracking);
    if !b.sub.is_empty() {
        text(p, pos2(x, y + 21.0), Align2::LEFT_CENTER, b.sub, 13.5, SEMI, with_alpha(ink, 0.72));
    }
    clicked
}

// ------------------------------------------------------------------ icons

#[derive(Clone, Copy, PartialEq)]
pub enum Icon {
    Play,
    Skip,
    Swap,
    Folder,
    Sliders,
    Download,
    Check,
    Cross,
    Lock,
    Up,
    Drive,
    ChevronLeft,
    ChevronRight,
}

/// Draws `which` centred on `c`, about `s` points across.
pub fn icon(p: &Painter, which: Icon, c: Pos2, s: f32, color: Color32) {
    let at = |x: f32, y: f32| pos2(c.x + x * s, c.y + y * s);
    let pen = Stroke::new((s * 0.13).max(1.6), color);
    let line = |points: &[(f32, f32)]| {
        p.add(Shape::line(points.iter().map(|&(x, y)| at(x, y)).collect(), pen));
    };
    match which {
        Icon::Play => {
            p.add(Shape::convex_polygon(vec![at(-0.30, -0.46), at(0.48, 0.0), at(-0.30, 0.46)], color, Stroke::NONE));
        }
        Icon::Skip => {
            line(&[(-0.42, -0.36), (-0.06, 0.0), (-0.42, 0.36)]);
            line(&[(0.06, -0.36), (0.42, 0.0), (0.06, 0.36)]);
        }
        Icon::Swap => {
            line(&[(-0.44, -0.20), (0.44, -0.20)]);
            line(&[(0.20, -0.42), (0.44, -0.20), (0.20, 0.02)]);
            line(&[(0.44, 0.24), (-0.44, 0.24)]);
            line(&[(-0.20, 0.02), (-0.44, 0.24), (-0.20, 0.46)]);
        }
        Icon::Folder => {
            line(&[(-0.46, -0.30), (-0.14, -0.30), (-0.02, -0.16), (0.46, -0.16), (0.46, 0.36), (-0.46, 0.36), (-0.46, -0.30)]);
        }
        Icon::Sliders => {
            for (y, knob) in [(-0.30, 0.18), (0.0, -0.20), (0.30, 0.08)] {
                line(&[(-0.44, y), (0.44, y)]);
                p.circle_filled(at(knob, y), s * 0.12, color);
            }
        }
        Icon::Download => {
            line(&[(0.0, -0.44), (0.0, 0.14)]);
            line(&[(-0.24, -0.08), (0.0, 0.16), (0.24, -0.08)]);
            line(&[(-0.40, 0.40), (0.40, 0.40)]);
        }
        Icon::Check => line(&[(-0.36, 0.02), (-0.10, 0.28), (0.38, -0.26)]),
        Icon::Cross => {
            line(&[(-0.30, -0.30), (0.30, 0.30)]);
            line(&[(0.30, -0.30), (-0.30, 0.30)]);
        }
        Icon::Lock => {
            p.rect_filled(Rect::from_min_max(at(-0.32, -0.04), at(0.32, 0.44)), s * 0.08, color);
            line(&[(-0.20, -0.04), (-0.20, -0.26), (-0.10, -0.42), (0.10, -0.42), (0.20, -0.26), (0.20, -0.04)]);
        }
        Icon::Up => {
            line(&[(0.0, 0.40), (0.0, -0.40)]);
            line(&[(-0.26, -0.14), (0.0, -0.40), (0.26, -0.14)]);
        }
        Icon::Drive => {
            line(&[(-0.44, -0.18), (0.44, -0.18), (0.44, 0.24), (-0.44, 0.24), (-0.44, -0.18)]);
            p.circle_filled(at(0.26, 0.03), s * 0.07, color);
        }
        Icon::ChevronLeft => line(&[(0.16, -0.36), (-0.20, 0.0), (0.16, 0.36)]),
        Icon::ChevronRight => line(&[(-0.16, -0.36), (0.20, 0.0), (-0.16, 0.36)]),
    }
}

// ------------------------------------------------------------------ hints

/// A control named in a hint: shown as the controller's button or the key.
#[derive(Clone, Copy)]
pub enum Hint {
    Confirm,
    Back,
    Switch,
    Extra,
    Sides,
    Move,
}

/// Draws "(A) Select" style hints from `pos` rightwards (or leftwards from
/// it with `from_right`), for the device last used.
pub fn hints(cx: &Cx, pos: Pos2, items: &[(Hint, &str)], from_right: bool) {
    // Only the screen on top says what the buttons do.
    if !cx.active {
        return;
    }
    let p = &cx.p;
    let parts: Vec<(Option<(char, Color32)>, String, &str)> = items
        .iter()
        .map(|&(hint, label)| {
            let (round, cap) = match (cx.device, hint) {
                (Device::Pad, Hint::Confirm) => (Some(('A', Color32::from_rgb(0x5f, 0xd3, 0x6a))), ""),
                (Device::Pad, Hint::Back) => (Some(('B', Color32::from_rgb(0xff, 0x5f, 0x5f))), ""),
                (Device::Pad, Hint::Extra) => (Some(('X', Color32::from_rgb(0x4d, 0xa6, 0xff))), ""),
                (Device::Pad, Hint::Switch) => (Some(('Y', Color32::from_rgb(0xff, 0xc9, 0x3c))), ""),
                (Device::Pad, Hint::Sides) => (None, "LB  RB"),
                (Device::Pad, Hint::Move) => (None, "D-PAD"),
                (Device::Keyboard, Hint::Confirm) => (None, "ENTER"),
                (Device::Keyboard, Hint::Back) => (None, "ESC"),
                (Device::Keyboard, Hint::Extra) => (None, "F"),
                (Device::Keyboard, Hint::Switch) => (None, "TAB"),
                (Device::Keyboard, Hint::Sides) => (None, "Q  E"),
                (Device::Keyboard, Hint::Move) => (None, "ARROWS"),
            };
            (round, cap.to_string(), label)
        })
        .collect();
    let width = |part: &(Option<(char, Color32)>, String, &str)| {
        let key = if part.0.is_some() { 24.0 } else { measure(p, &part.1, 11.5, BOLD).x + 16.0 };
        key + 8.0 + measure(p, part.2, 14.5, SEMI).x
    };
    let total: f32 = parts.iter().map(width).sum::<f32>() + 26.0 * parts.len().saturating_sub(1) as f32;
    let mut x = if from_right { pos.x - total } else { pos.x };
    for part in &parts {
        let key_w = match &part.0 {
            Some((letter, color)) => {
                p.circle_filled(pos2(x + 12.0, pos.y), 12.0, black(0.55));
                p.circle_stroke(pos2(x + 12.0, pos.y), 11.0, Stroke::new(2.0, *color));
                text(p, pos2(x + 12.0, pos.y), Align2::CENTER_CENTER, &letter.to_string(), 13.0, BLACK, *color);
                24.0
            }
            None => {
                let w = measure(p, &part.1, 11.5, BOLD).x + 16.0;
                let cap = Rect::from_min_size(pos2(x, pos.y - 11.0), vec2(w, 22.0));
                p.rect_filled(cap, 6.0, black(0.50));
                p.rect_stroke(cap, 6.0, Stroke::new(1.2, white(0.55)), StrokeKind::Inside);
                text(p, cap.center(), Align2::CENTER_CENTER, &part.1, 11.5, BOLD, white(0.95));
                w
            }
        };
        text(p, pos2(x + key_w + 8.0, pos.y), Align2::LEFT_CENTER, part.2, 14.5, SEMI, white(0.88));
        x += width(part) + 26.0;
    }
}

// ----------------------------------------------------------------- panels

/// A dark translucent card with a soft shadow: the body of a dialog.
pub fn panel(p: &Painter, rect: Rect, radius: f32) {
    glow(p, rect.translate(vec2(0.0, 18.0)), radius, black(0.55), 60.0);
    p.rect_filled(rect, radius, Color32::from_rgba_unmultiplied(14, 17, 34, 244));
    p.rect_stroke(rect, radius, Stroke::new(1.5, white(0.14)), StrokeKind::Inside);
}

/// A small status dot with a halo.
pub fn dot(p: &Painter, time: f32, c: Pos2, color: Color32) {
    let pulse = 0.5 + 0.5 * (time * 2.4).sin();
    p.circle_filled(c, 9.0 + 2.0 * pulse, with_alpha(color, 0.22));
    p.circle_filled(c, 5.0, color);
}

pub const GOOD: Color32 = Color32::from_rgb(0x6b, 0xe5, 0x8a);
pub const WARN: Color32 = Color32::from_rgb(0xff, 0xc8, 0x57);
pub const BAD: Color32 = Color32::from_rgb(0xff, 0x6b, 0x6b);
pub const IDLE: Color32 = Color32::from_rgb(0xb9, 0xb4, 0xd8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_direction_moves_the_focus_to_the_nearest_control_that_way() {
        // The main menu's shape: three big buttons in a row, two small below.
        let rect = |x: f32, y: f32, w: f32, h: f32| Rect::from_min_size(pos2(x, y), vec2(w, h));
        let mut focus = Focus::starting_at("play");
        let lay_out = |focus: &mut Focus| {
            focus.begin();
            for (name, r) in [("play", rect(72.0, 418.0, 224.0, 88.0)), ("skip", rect(312.0, 418.0, 204.0, 88.0)), ("switch", rect(532.0, 418.0, 214.0, 88.0)), ("folder", rect(72.0, 522.0, 184.0, 52.0)), ("settings", rect(270.0, 522.0, 152.0, 52.0)), ("update", rect(900.0, 31.0, 240.0, 38.0))] {
                focus.add(Id::new(name), r);
            }
        };
        let mut walk = |moves: &[Nav], lands_on: &str| {
            lay_out(&mut focus);
            focus.end(moves);
            assert_eq!(focus.current, Some(Id::new(lands_on)), "after {moves:?}");
        };
        walk(&[Nav::Right], "skip");
        walk(&[Nav::Right], "switch");
        walk(&[Nav::Down], "settings");
        walk(&[Nav::Left], "folder");
        walk(&[Nav::Left], "folder");
        walk(&[Nav::Down], "folder");
        walk(&[Nav::Up], "play");
        walk(&[Nav::Down, Nav::Right], "settings");
        walk(&[Nav::Up], "skip");
        walk(&[Nav::Left], "play");
        walk(&[Nav::Up], "update");
        walk(&[Nav::Down], "switch");
        walk(&[Nav::Confirm], "switch");
    }
}
