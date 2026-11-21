//! Switch Version: one card per version, each in its own colours with its
//! own picture, title and state. Left and right choose, confirm switches.

use egui::epaint::RectShape;
use egui::{pos2, vec2, Align2, Color32, CornerRadius, Rect, Stroke, StrokeKind};

use crate::app::{Action, Shared};
use crate::draw::*;
use crate::nav::Nav;
use crate::screen_main::status;

pub struct State {
    index: usize,
}

impl State {
    pub fn new(selected: usize) -> Self {
        Self { index: selected }
    }
}

const CARD: egui::Vec2 = vec2(330.0, 440.0);
const GAP: f32 = 34.0;

pub fn draw(sh: &Shared, cx: &mut Cx, st: &mut State, t: f32) -> Vec<Action> {
    let mut actions = Vec::new();
    let s = cx.screen;
    let n = sh.versions.len();
    if cx.pressed(Nav::Left) || cx.pressed(Nav::Prev) {
        st.index = st.index.saturating_sub(1);
    }
    if cx.pressed(Nav::Right) || cx.pressed(Nav::Next) {
        st.index = (st.index + 1).min(n - 1);
    }

    let rise = 20.0 * (1.0 - ease_out(t));
    spaced(&cx.p, pos2(s.center().x, s.top() + 72.0 - rise), Align2::CENTER_CENTER, "SWITCH VERSION", 15.0, BOLD, cx.pal.accent[0], 3.0);
    text(&cx.p, pos2(s.center().x, s.top() + 112.0 - rise), Align2::CENTER_CENTER, "Choose what to play", 38.0, BLACK, Color32::WHITE);

    // The row of cards: centred when it fits, else following the choice.
    let total = n as f32 * CARD.x + (n as f32 - 1.0) * GAP;
    let room = s.width() - 144.0;
    let follow = if total <= room { s.center().x - total / 2.0 } else { (s.center().x - CARD.x / 2.0 - st.index as f32 * (CARD.x + GAP)).clamp(s.right() - 72.0 - total, s.left() + 72.0) };
    let x0 = cx.anim.to("switch-scroll", follow, 10.0, cx.dt);
    let top = s.center().y - CARD.y / 2.0 + 44.0;

    let mut chosen = None;
    for i in 0..n {
        let v = &sh.versions[i];
        let slot = &sh.slots[i];
        let base = Rect::from_min_size(pos2(x0 + i as f32 * (CARD.x + GAP), top), CARD);
        if cx.hovered(base) {
            if cx.moved {
                st.index = i;
            }
            if cx.clicked {
                chosen = Some(i);
            }
        }
        let f = cx.anim.to(("switch-card", i), if st.index == i { 1.0 } else { 0.0 }, 12.0, cx.dt);
        let arrive = ease_out((t * 1.7 - i as f32 * 0.16).clamp(0.0, 1.0));
        let rect = scale_rect(base, 0.94 + 0.06 * f).translate(vec2(0.0, 60.0 * (1.0 - arrive) - 8.0 * f));
        let mut p = cx.p.clone();
        p.multiply_opacity(arrive);
        let radius = 24.0;
        let [a0, a1] = [v.theme.accent[0].0, v.theme.accent[1].0];

        glow(&p, rect.translate(vec2(0.0, 22.0)), radius, black(0.50), 56.0);
        if f > 0.01 {
            glow(&p, rect.translate(vec2(0.0, 8.0)), radius, with_alpha(a1, 0.55 * f), 54.0);
        }
        // The picture, drifting slowly; the title under it.
        let art = Rect::from_min_max(rect.min, pos2(rect.right(), rect.top() + rect.height() * 0.60));
        let body = Rect::from_min_max(pos2(rect.left(), art.bottom()), rect.max);
        let drift = cx.time * 0.1 + i as f32 * 2.0;
        let uv = Rect::from_min_size(pos2(0.14 + 0.05 * drift.sin(), 0.04 + 0.03 * (drift * 0.8).cos()), vec2(0.62, 0.82));
        picture(&p, art, CornerRadius { nw: 24, ne: 24, sw: 0, se: 0 }, slot.cover.id(), uv, Color32::WHITE);
        p.add(RectShape::filled(body, CornerRadius { nw: 0, ne: 0, sw: 24, se: 24 }, Color32::from_rgb(15, 18, 36)));
        match &slot.logo {
            Some(logo) => {
                let size = logo.size_vec2();
                let fit = (art.width() * 0.84 / size.x).min(art.height() * 0.70 / size.y);
                let at = Rect::from_center_size(art.center(), size * fit);
                picture(&p, at.translate(vec2(0.0, 5.0)), 0.0, logo.id(), FULL_UV, black(0.40));
                picture(&p, at, 0.0, logo.id(), FULL_UV, Color32::WHITE);
            }
            None => {
                let name = v.short().to_uppercase();
                let size = (art.width() * 0.80 / (name.chars().count() as f32 * 0.72)).clamp(30.0, 62.0);
                text(&p, art.center() + vec2(0.0, 4.0), Align2::CENTER_CENTER, &name, size, BLACK, black(0.35));
                text(&p, art.center(), Align2::CENTER_CENTER, &name, size, BLACK, Color32::WHITE);
            }
        }
        // A band of the version's colours between picture and words.
        gradient_rect(&p, Rect::from_min_size(pos2(body.left(), body.top()), vec2(body.width(), 4.0)), 0.0, a0, a1, cx.ramp);

        let x = body.left() + 24.0;
        let title_h = wrapped(&p, pos2(x, body.top() + 20.0), body.width() - 48.0, &v.title, 21.0, BOLD, Color32::WHITE);
        wrapped(&p, pos2(x, body.top() + 24.0 + title_h), body.width() - 48.0, &v.tagline, 13.5, SEMI, white(0.62));
        let (color, words) = status(v, slot);
        let words = words.split("  ·  ").next().unwrap_or("").to_string();
        let line = body.bottom() - 28.0;
        dot(&p, cx.time, pos2(x + 7.0, line), color);
        text(&p, pos2(x + 24.0, line), Align2::LEFT_CENTER, &fit_middle(&p, &words, 14.5, SEMI, body.width() - 80.0), 14.5, SEMI, white(0.90));

        if i == sh.selected {
            let tag = Rect::from_min_size(rect.min + vec2(16.0, 16.0), vec2(92.0, 26.0));
            p.rect_filled(tag, 13.0, black(0.55));
            spaced(&p, tag.center(), Align2::CENTER_CENTER, "CURRENT", 11.5, BOLD, Color32::WHITE, 1.6);
        }
        p.rect_filled(rect, radius, black(0.30 * (1.0 - f)));
        p.rect_stroke(rect, radius, Stroke::new(1.5, white(0.16 + 0.2 * f)), StrokeKind::Inside);
        if f > 0.01 && cx.active {
            let pulse = 1.5 * (cx.time * 3.2).sin();
            p.rect_stroke(rect.expand(6.0 + pulse * f), radius + 6.0, Stroke::new(3.0, white(0.95 * f)), StrokeKind::Middle);
        }
    }

    if cx.pressed(Nav::Confirm) {
        chosen = Some(st.index);
    }
    if let Some(i) = chosen {
        actions.extend([Action::Select(i), Action::Close]);
    } else if cx.pressed(Nav::Back) || cx.pressed(Nav::Switch) {
        actions.push(Action::Close);
    }
    hints(cx, pos2(s.center().x - 150.0, s.bottom() - 40.0), &[(Hint::Move, "Choose"), (Hint::Confirm, "Switch"), (Hint::Back, "Back")], false);
    actions
}
