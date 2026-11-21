//! The main menu: the selected version's title and state, Play with Skip
//! Launcher next to it, Switch Version, the game folder and settings, and on
//! the right a card of art (the player's own game's logo and course
//! previews once a game folder is set; the launcher's own picture before).

use egui::epaint::{RectShape, TextShape};
use egui::{pos2, vec2, Align2, Color32, Pos2, Rect, Stroke, StrokeKind};

use crate::app::{Action, Shared, Slot, UpdateStatus};
use crate::draw::*;
use crate::nav::{Device, Nav};
use crate::versions::Version;

/// The one-line state of a version: a colour and words.
pub fn status(v: &Version, slot: &Slot) -> (Color32, String) {
    if !v.playable {
        return (IDLE, if slot.game.is_some() { "Not playable yet  ·  game folder set".into() } else { "Not playable yet".into() });
    }
    if slot.game.is_none() {
        return (WARN, "Add your copy of the game to play".into());
    }
    match (&slot.problem, &slot.found) {
        // The short of it; Play says the rest.
        (Some(p), _) => (BAD, p.split(':').next().unwrap_or(p).to_string()),
        (None, Some(found)) => (GOOD, format!("Ready to play  ·  {}", found.describe())),
        (None, None) => (GOOD, "Ready to play".into()),
    }
}

/// Draws the main menu. `appear` (0..1) fades the version's own parts in
/// after a switch. Returns what was asked for and where Play is.
pub fn draw(sh: &Shared, cx: &mut Cx, focus: &mut Focus, appear: f32) -> (Vec<Action>, Pos2) {
    let mut actions = Vec::new();
    let s = cx.screen;
    let v = &sh.versions[sh.selected];
    let slot = &sh.slots[sh.selected];
    let left = s.left() + 72.0;
    let cy = s.center().y;
    focus.begin();

    // The top bar: the launcher's name, the update notice, the version.
    let bar = s.top() + 50.0;
    let mark = Rect::from_center_size(pos2(left + 17.0, bar), vec2(34.0, 34.0));
    gradient_rect(&cx.p, mark, 10.0, cx.pal.accent[0], cx.pal.accent[1], cx.ramp);
    icon(&cx.p, Icon::Play, mark.center() + vec2(1.0, 0.0), 17.0, cx.pal.ink);
    let name = spaced(&cx.p, pos2(left + 48.0, bar), Align2::LEFT_CENTER, "NSMBW PORT", 15.0, BOLD, Color32::WHITE, 2.6);
    spaced(&cx.p, pos2(name.right() + 14.0, bar), Align2::LEFT_CENTER, "LAUNCHER", 15.0, REGULAR, white(0.62), 2.6);

    let right = s.right() - 72.0;
    let build = text(&cx.p, pos2(right, bar), Align2::RIGHT_CENTER, &format!("v{}", port_update::VERSION), 13.5, SEMI, white(0.55));
    if let UpdateStatus::Available(release) = &sh.update.status {
        let label = format!("Update available  ·  {}", release.version);
        let w = measure(&cx.p, &label, 14.5, BOLD).x + 62.0;
        let pill = Rect::from_min_size(pos2(build.left() - 18.0 - w, bar - 19.0), vec2(w, 38.0));
        let (focused, clicked) = cx.interact(focus, "update", pill);
        let f = cx.anim.to("update-focus", if focused && cx.active { 1.0 } else { 0.0 }, 16.0, cx.dt);
        let pulse = 0.5 + 0.5 * (cx.time * 2.2).sin();
        glow(&cx.p, pill, 19.0, with_alpha(cx.pal.accent[0], 0.25 + 0.25 * pulse), 22.0);
        cx.p.rect_filled(pill, 19.0, Color32::from_rgba_unmultiplied(12, 16, 34, 205));
        cx.p.rect_stroke(pill, 19.0, Stroke::new(1.5, with_alpha(cx.pal.accent[0], 0.9)), StrokeKind::Inside);
        if f > 0.01 {
            cx.p.rect_stroke(pill.expand(4.0), 23.0, Stroke::new(3.0, white(0.95 * f)), StrokeKind::Middle);
        }
        icon(&cx.p, Icon::Download, pos2(pill.left() + 24.0, bar), 17.0, cx.pal.accent[0]);
        text(&cx.p, pos2(pill.left() + 44.0, bar), Align2::LEFT_CENTER, &label, 14.5, BOLD, Color32::WHITE);
        if clicked {
            actions.push(Action::OpenUpdate);
        }
    }

    // The version: tagline, title, state. These glide in after a switch.
    let mut vp = cx.p.clone();
    vp.multiply_opacity(smooth(appear));
    let slide = vec2(-26.0 * (1.0 - ease_out(appear)), 0.0);
    let top = cy - 196.0;
    spaced(&vp, pos2(left, top + 9.0) + slide, Align2::LEFT_CENTER, &v.tagline.to_uppercase(), 15.0, BOLD, cx.pal.accent[0], 2.4);
    let title_at = pos2(left - 3.0, top + 22.0) + slide;
    wrapped(&vp, title_at + vec2(0.0, 4.0), 660.0, &v.title, 61.0, BLACK, black(0.30));
    let title_h = wrapped(&vp, title_at, 660.0, &v.title, 61.0, BLACK, Color32::WHITE);
    let state_y = title_at.y + title_h + 26.0;
    let (color, words) = status(v, slot);
    let words = fit_middle(&vp, &words, 17.0, SEMI, 640.0);
    dot(&vp, cx.time, pos2(left + 9.0, state_y) + slide, color);
    text(&vp, pos2(left + 28.0, state_y) + slide, Align2::LEFT_CENTER, &words, 17.0, SEMI, white(0.94));

    // Play, Skip Launcher, Switch Version.
    let row = state_y + 40.0;
    let play = Rect::from_min_size(pos2(left, row), vec2(224.0, 88.0));
    let skip = Rect::from_min_size(pos2(play.right() + 16.0, row), vec2(204.0, 88.0));
    let switch = Rect::from_min_size(pos2(skip.right() + 16.0, row), vec2(214.0, 88.0));
    let play_button = if !v.playable {
        Button::new("play", play, "NOT YET").icon(Icon::Lock).primary().dim(true)
    } else if slot.game.is_none() {
        Button::new("play", play, "SET UP").icon(Icon::Folder).primary()
    } else {
        Button::new("play", play, "PLAY").icon(Icon::Play).primary().dim(slot.problem.is_some())
    };
    if button(cx, focus, play_button) || cx.pressed(Nav::Play) {
        actions.push(Action::Play);
    }
    let skipping = sh.store.state.skip_launcher;
    if button(cx, focus, Button::new("skip", skip, "SKIP LAUNCHER").sub(if skipping { "On" } else { "Off" }).icon(Icon::Skip).on(skipping)) {
        actions.push(Action::ToggleSkip);
    }
    if button(cx, focus, Button::new("switch", switch, "SWITCH VERSION").sub(v.short()).icon(Icon::Swap)) || cx.pressed(Nav::Switch) {
        actions.push(Action::OpenSwitcher);
    }

    // The game folder and settings.
    let row2 = row + 88.0 + 16.0;
    let folder = Rect::from_min_size(pos2(left, row2), vec2(184.0, 52.0));
    let settings = Rect::from_min_size(pos2(folder.right() + 14.0, row2), vec2(152.0, 52.0));
    if button(cx, focus, Button::new("folder", folder, "Game folder").icon(Icon::Folder)) {
        actions.push(Action::OpenSetup);
    }
    if button(cx, focus, Button::new("settings", settings, "Settings").icon(Icon::Sliders)) {
        actions.push(Action::OpenSettings);
    }
    if skipping {
        let note = "Skip Launcher is on: the next start goes straight into the game. Hold Shift while starting (or Back on an Xbox controller) to come back here.";
        let galley = cx.p.layout(note.to_string(), font(14.0, SEMI), white(0.92), 620.0);
        let plate = Rect::from_min_size(pos2(left, row2 + 66.0), galley.size() + vec2(28.0, 18.0));
        cx.p.rect_filled(plate, 12.0, Color32::from_rgba_unmultiplied(12, 16, 34, 150));
        cx.p.galley(plate.min + vec2(14.0, 9.0), galley, white(0.92));
    }

    hero(sh, cx, pos2(right - 196.0, cy - if slot.banner.is_some() { 84.0 } else { 64.0 }), appear);

    // The bumpers step through the versions.
    if cx.pressed(Nav::Prev) || cx.pressed(Nav::Next) {
        let n = sh.versions.len();
        actions.push(Action::Select(if cx.pressed(Nav::Next) { (sh.selected + 1) % n } else { (sh.selected + n - 1) % n }));
    }

    // Hints, and the device in use.
    let foot = s.bottom() - 40.0;
    hints(cx, pos2(left, foot), &[(Hint::Confirm, "Select"), (Hint::Switch, "Switch version"), (Hint::Sides, "Next version")], false);
    let device = match (cx.device, &sh.pad) {
        (Device::Pad, Some(name)) => name.clone(),
        (Device::Pad, None) => "Controller".to_string(),
        (Device::Keyboard, Some(_)) => "Keyboard and mouse  ·  controller ready".to_string(),
        (Device::Keyboard, None) => "Keyboard and mouse".to_string(),
    };
    text(&cx.p, pos2(right, foot), Align2::RIGHT_CENTER, &fit_middle(&cx.p, &device, 13.5, SEMI, 420.0), 13.5, SEMI, white(0.55));

    focus.end(&cx.nav);
    (actions, play.center())
}

/// The art on the right: a tilted card with the scene behind the game's own
/// logo (or the version's name), and the game's course previews fanned out
/// in front of it.
fn hero(sh: &Shared, cx: &mut Cx, center: Pos2, appear: f32) {
    let v = &sh.versions[sh.selected];
    let slot = &sh.slots[sh.selected];
    let mut p = cx.p.clone();
    p.multiply_opacity(smooth(appear));
    let t = cx.time;
    let rise = 30.0 * (1.0 - ease_out(appear));
    let center = center + vec2(0.0, 5.0 * (t * 0.7).sin() + rise);
    let angle = if slot.banner.is_some() { 0.0 } else { -0.045 + 0.006 * (t * 0.5).sin() };
    let card = Rect::from_center_size(center, vec2(392.0, 274.0));
    let radius = 26.0;

    p.add(RectShape::filled(card.translate(vec2(0.0, 26.0)), radius, black(0.42)).with_blur_width(64.0).with_angle_and_pivot(angle, center));
    p.add(RectShape::filled(card.translate(vec2(0.0, 10.0)), radius, with_alpha(cx.pal.accent[1], 0.35)).with_blur_width(50.0).with_angle_and_pivot(angle, center));
    if let Some((banner, textures, since)) = &slot.banner {
        // The game's own banner, playing inside a frame. The frame's
        // rounded inside edge hides the square corners of the picture.
        let card = Rect::from_center_size(center, vec2(392.0, 392.0 * banner.size[1] / banner.size[0].max(1.0)).min(vec2(392.0, 320.0)));
        let frame = 8.0;
        let inside = card.shrink(frame - 1.0);
        let inner = p.with_clip_rect(inside.intersect(p.clip_rect()));
        let scale = (inside.width() / banner.size[0]).max(inside.height() / banner.size[1]);
        let place = |c: [f32; 2]| pos2(inside.center().x + c[0] * scale, inside.center().y - c[1] * scale);
        for quad in banner.quads(t - since) {
            let mut mesh = match quad.picture.and_then(|i| textures.get(i)) {
                Some(texture) => egui::Mesh::with_texture(texture.id()),
                None => egui::Mesh::default(),
            };
            for i in 0..4 {
                let c = quad.colors[i];
                let uv = if quad.picture.is_some() { pos2(quad.uv[i][0], quad.uv[i][1]) } else { egui::epaint::WHITE_UV };
                mesh.vertices.push(egui::epaint::Vertex { pos: place(quad.corners[i]), uv, color: Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]) });
            }
            mesh.indices = vec![0, 1, 2, 0, 2, 3];
            inner.add(egui::Shape::mesh(mesh));
        }
        p.rect_stroke(card, radius - 6.0, Stroke::new(frame, Color32::WHITE), StrokeKind::Inside);
        p.rect_stroke(card, radius - 6.0, Stroke::new(1.0, black(0.18)), StrokeKind::Outside);
        // The previews sit lower here: the banner's cast stands along its
        // bottom edge.
        gallery(slot, &p, center + vec2(0.0, 44.0), t);
        text(&p, center + vec2(0.0, if slot.gallery.is_empty() { card.height() / 2.0 + 24.0 } else { 300.0 }), Align2::CENTER_CENTER, "Art from your own game files", 12.5, SEMI, white(0.70));
        return;
    }
    // The still picture, drifting slowly inside the card.
    let uv = Rect::from_min_size(pos2(0.06 + 0.05 * (t * 0.11).sin(), 0.05 + 0.04 * (t * 0.09).cos()), vec2(0.84, 0.84));
    p.add(RectShape::filled(card, radius, Color32::WHITE).with_texture(slot.cover.id(), uv).with_angle(angle));
    p.add(RectShape::filled(card, radius, black(0.10)).with_angle(angle));
    p.add(RectShape::stroke(card, radius, Stroke::new(3.0, white(0.55)), StrokeKind::Inside).with_angle(angle));

    match &slot.logo {
        Some(logo) => {
            let size = logo.size_vec2();
            let fit = (card.width() * 0.84 / size.x).min(card.height() * 0.78 / size.y);
            let rect = Rect::from_center_size(center, size * fit);
            p.add(RectShape::filled(rect.translate(vec2(0.0, 7.0)), 0.0, black(0.40)).with_texture(logo.id(), FULL_UV).with_angle_and_pivot(angle, center));
            p.add(RectShape::filled(rect, 0.0, Color32::WHITE).with_texture(logo.id(), FULL_UV).with_angle(angle));
        }
        None => {
            // No game folder yet: the version's name, set in type.
            let name = v.short().to_uppercase();
            let size = (card.width() * 0.80 / (name.chars().count() as f32 * 0.72)).clamp(34.0, 78.0);
            for (offset, color) in [(vec2(0.0, 5.0), black(0.35)), (vec2(0.0, 0.0), Color32::WHITE)] {
                let galley = p.layout_no_wrap(name.clone(), font(size, BLACK), color);
                let at = center - galley.size() / 2.0 - vec2(0.0, 12.0) + offset;
                let rot = egui::emath::Rot2::from_angle(angle);
                p.add(TextShape::new(center + rot * (at - center), galley, color).with_angle(angle));
            }
            let hint = if v.playable { "Add your game to bring this to life" } else { "A place kept for later" };
            let galley = p.layout_no_wrap(hint.to_string(), font(14.5, SEMI), white(0.85));
            let at = center - galley.size() / 2.0 + vec2(0.0, size * 0.62);
            let rot = egui::emath::Rot2::from_angle(angle);
            p.add(TextShape::new(center + rot * (at - center), galley, white(0.85)).with_angle(angle));
        }
    }

    gallery(slot, &p, center, t);
    if slot.logo.is_some() || !slot.gallery.is_empty() {
        let below = if slot.gallery.is_empty() { 172.0 } else { 258.0 };
        text(&p, center + vec2(0.0, below), Align2::CENTER_CENTER, "Art from your own game files", 12.5, SEMI, white(0.55));
    }
}

/// Course previews from the player's game, a few at a time, fanned out in
/// front of the art card.
fn gallery(slot: &Slot, p: &egui::Painter, center: Pos2, t: f32) {
    let n = slot.gallery.len();
    if n > 0 {
        const SPOTS: [(f32, f32, f32); 4] = [(-176.0, 150.0, -0.14), (-60.0, 176.0, 0.05), (60.0, 174.0, -0.06), (176.0, 146.0, 0.12)];
        for (k, (dx, dy, tilt)) in SPOTS.into_iter().enumerate() {
            let period = 6.0;
            let clock = t / period + k as f32 * 0.27;
            let (turn, within) = (clock.floor() as usize, clock.fract() * period);
            let pick = |turn: usize| &slot.gallery[(turn * 7 + k * 3) % n];
            let at = center + vec2(dx, dy + 4.0 * (t * 0.9 + k as f32 * 1.4).sin());
            let (now, before) = (pick(turn), pick(turn.wrapping_sub(1)));
            let size = now.size_vec2() * (112.0 / now.size_vec2().y.max(1.0));
            let rect = Rect::from_center_size(at, size);
            let frame = rect.expand(4.0);
            p.add(RectShape::filled(frame.translate(vec2(0.0, 10.0)), 12.0, black(0.40)).with_blur_width(26.0).with_angle_and_pivot(tilt, at));
            p.add(RectShape::filled(frame, 11.0, Color32::WHITE).with_angle(tilt));
            let fade = smooth(within / 0.7);
            if fade < 1.0 {
                p.add(RectShape::filled(rect, 8.0, Color32::WHITE).with_texture(before.id(), FULL_UV).with_angle(tilt));
            }
            p.add(RectShape::filled(rect, 8.0, white(fade)).with_texture(now.id(), FULL_UV).with_angle(tilt));
        }
    }
}
