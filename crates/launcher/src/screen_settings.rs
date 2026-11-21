//! Settings: the selected version's display settings (single keys of its
//! `settings.toml`, the game's own file) and the launcher's own switches.
//! Up and down choose a row; left and right (or confirm) change it.

use std::path::Path;

use egui::{pos2, vec2, Align2, Color32, Rect, Stroke, StrokeKind};

use crate::app::{Action, Shared, UpdateStatus};
use crate::draw::*;
use crate::nav::Nav;
use crate::store::{read_settings, SETTINGS};

pub struct State {
    index: usize,
    /// The choice of each of [`SETTINGS`].
    values: [usize; SETTINGS.len()],
}

impl State {
    /// For the version whose folder is `dir`.
    pub fn new(dir: &Path) -> Self {
        Self { index: 0, values: read_settings(dir) }
    }
}

/// The launcher's own rows, after the game's.
const SKIP: usize = SETTINGS.len();
const UPDATES: usize = SKIP + 1;
const CHECK_NOW: usize = SKIP + 2;
const OPEN_FOLDER: usize = SKIP + 3;
const ROWS: usize = SKIP + 4;

const ROW: f32 = 46.0;
const HEADING: f32 = 40.0;

pub fn draw(sh: &Shared, cx: &mut Cx, st: &mut State, t: f32) -> Vec<Action> {
    let mut actions = Vec::new();
    let s = cx.screen;
    let v = &sh.versions[sh.selected];
    let width = 600.0;
    let rect = Rect::from_min_max(pos2(s.right() - width - 28.0, s.top() + 28.0), pos2(s.right() - 28.0, s.bottom() - 28.0)).translate(vec2(90.0 * (1.0 - ease_out(t)), 0.0));
    panel(&cx.p, rect, 28.0);
    let x = rect.left() + 34.0;
    let inner = rect.width() - 68.0;

    spaced(&cx.p, pos2(x, rect.top() + 42.0), Align2::LEFT_CENTER, "SETTINGS", 14.0, BOLD, cx.pal.accent[0], 2.4);
    text(&cx.p, pos2(x, rect.top() + 76.0), Align2::LEFT_CENTER, &fit_middle(&cx.p, &v.title, 26.0, BLACK, inner), 26.0, BLACK, Color32::WHITE);

    if cx.pressed(Nav::Up) {
        st.index = st.index.saturating_sub(1);
    }
    if cx.pressed(Nav::Down) {
        st.index = (st.index + 1).min(ROWS - 1);
    }

    // The rows scroll inside this area when the window is short.
    let area = Rect::from_min_max(pos2(rect.left() + 20.0, rect.top() + 104.0), pos2(rect.right() - 20.0, rect.bottom() - 96.0));
    let content = ROWS as f32 * ROW + 2.0 * HEADING;
    let row_top = |i: usize| i as f32 * ROW + HEADING + if i >= SKIP { HEADING } else { 0.0 };
    let mut want = cx.anim.to("settings-scroll", 0.0, 0.0, cx.dt);
    if cx.hovered(area) && cx.wheel != 0.0 {
        want -= cx.wheel;
    }
    if !cx.nav.is_empty() {
        want = want.clamp(row_top(st.index) + ROW - area.height(), row_top(st.index) - HEADING);
    }
    let want = want.clamp(0.0, (content - area.height()).max(0.0));
    cx.anim.set("settings-scroll", want);
    let scroll = cx.anim.to("settings-shown", want, 18.0, cx.dt);
    let rows = cx.p.with_clip_rect(area);

    let heading = |y: f32, words: &str| {
        spaced(&rows, pos2(x, area.top() + y - scroll + HEADING - 14.0), Align2::LEFT_CENTER, words, 12.5, BOLD, white(0.5), 2.0);
    };
    heading(0.0, "THE GAME'S DISPLAY");
    heading(row_top(SKIP) - HEADING, "THE LAUNCHER");

    let checking = match &sh.update.status {
        UpdateStatus::Off => "Off".to_string(),
        UpdateStatus::Checking => "Checking...".to_string(),
        UpdateStatus::UpToDate => format!("Version {} is the newest", port_update::VERSION),
        UpdateStatus::Available(r) => format!("Version {} is out", r.version),
        UpdateStatus::Failed(_) => "Could not check".to_string(),
    };
    let mut help = String::new();
    for i in 0..ROWS {
        let row = Rect::from_min_size(pos2(area.left(), area.top() + row_top(i) - scroll), vec2(area.width(), ROW - 4.0));
        if row.bottom() < area.top() || row.top() > area.bottom() {
            continue;
        }
        let over = cx.hovered(row) && cx.hovered(area);
        if over && cx.moved {
            st.index = i;
        }
        let focused = st.index == i;
        let f = cx.anim.to(("settings-row", i), if focused { 1.0 } else { 0.0 }, 20.0, cx.dt);
        if f > 0.01 {
            rows.rect_filled(row, 12.0, white(0.10 * f));
            rows.rect_stroke(row, 12.0, Stroke::new(2.0, white(0.85 * f)), StrokeKind::Inside);
        }
        // Each row: a label, its value, and (for a choice) arrows either side.
        let (label, value, arrows, text_help): (&str, String, bool, &str) = if i < SKIP {
            let setting = &SETTINGS[i];
            (setting.label, setting.choices[st.values[i]].1.to_string(), true, setting.help)
        } else {
            match i {
                SKIP => ("Skip Launcher", if sh.store.state.skip_launcher { "On" } else { "Off" }.to_string(), false, "Start the game straight away. Hold Shift while starting (or Back on an Xbox controller) to see the launcher."),
                UPDATES => ("Look for updates", if sh.update.config.enabled { "On" } else { "Off" }.to_string(), false, "Ask the release source for a newer version each time the launcher starts."),
                CHECK_NOW => ("Check now", checking.clone(), false, "Ask for the newest release now."),
                OPEN_FOLDER | _ => ("Open this version's folder", String::new(), false, "Where this version's settings, saves and log are kept."),
            }
        };
        if focused {
            help = text_help.to_string();
            if i == CHECK_NOW {
                if let UpdateStatus::Failed(e) = &sh.update.status {
                    help = e.clone();
                }
            }
        }
        text(&rows, pos2(row.left() + 16.0, row.center().y), Align2::LEFT_CENTER, label, 16.5, SEMI, Color32::WHITE);
        let right = row.right() - 16.0;
        let mut step = 0i32;
        if arrows {
            let n = SETTINGS[i].choices.len();
            let (less, more) = (st.values[i] > 0, st.values[i] + 1 < n);
            let (left_at, right_at) = (pos2(right - 250.0, row.center().y), pos2(right - 6.0, row.center().y));
            icon(&rows, Icon::ChevronLeft, left_at, 15.0, white(if less { 0.35 + 0.6 * f } else { 0.12 }));
            icon(&rows, Icon::ChevronRight, right_at, 15.0, white(if more { 0.35 + 0.6 * f } else { 0.12 }));
            text(&rows, pos2(right - 128.0, row.center().y), Align2::CENTER_CENTER, &value, 15.5, BOLD, cx.pal.accent[0]);
            if over && cx.clicked {
                let x = cx.pointer.map_or(0.0, |p| p.x);
                // Click the left arrow to go back; anywhere else goes on.
                step = if x < left_at.x + 30.0 && x > left_at.x - 30.0 { -1 } else { 1 };
            }
            if focused && cx.pressed(Nav::Left) {
                step = -1;
            }
            if focused && (cx.pressed(Nav::Right) || cx.pressed(Nav::Confirm)) {
                step = 1;
            }
            let next = if step < 0 && less {
                Some(st.values[i] - 1)
            } else if step > 0 && more {
                Some(st.values[i] + 1)
            } else if step > 0 && (cx.pressed(Nav::Confirm) || cx.clicked) {
                // Confirm and clicks go round; right stops at the end.
                Some(0)
            } else {
                None
            };
            if let Some(next) = next {
                st.values[i] = next;
                actions.push(Action::Setting(i, next));
            }
        } else {
            let on = value == "On";
            if i == SKIP || i == UPDATES {
                // A switch.
                let track = Rect::from_center_size(pos2(right - 26.0, row.center().y), vec2(48.0, 26.0));
                let k = cx.anim.to(("settings-switch", i), if on { 1.0 } else { 0.0 }, 16.0, cx.dt);
                rows.rect_filled(track, 13.0, crate::scene::mix(white(0.18), cx.pal.accent[0], k));
                rows.circle_filled(pos2(track.left() + 13.0 + 22.0 * k, track.center().y), 9.5, Color32::WHITE);
                text(&rows, pos2(track.left() - 12.0, row.center().y), Align2::RIGHT_CENTER, &value, 15.5, BOLD, white(0.85));
            } else {
                text(&rows, pos2(right - 22.0, row.center().y), Align2::RIGHT_CENTER, &value, 14.5, SEMI, white(0.7));
                icon(&rows, Icon::ChevronRight, pos2(right - 6.0, row.center().y), 15.0, white(0.35 + 0.6 * f));
            }
            let pressed = (over && cx.clicked) || (focused && (cx.pressed(Nav::Confirm) || ((i == SKIP || i == UPDATES) && (cx.pressed(Nav::Left) || cx.pressed(Nav::Right)))));
            if pressed {
                actions.push(match i {
                    SKIP => Action::ToggleSkip,
                    UPDATES => Action::SetUpdateCheck(!sh.update.config.enabled),
                    CHECK_NOW => Action::CheckUpdates,
                    _ => Action::OpenVersionFolder,
                });
            }
        }
    }

    // What the highlighted row does, and where the game's file is.
    let foot = rect.bottom() - 84.0;
    wrapped(&cx.p, pos2(x, foot), inner, &help, 14.0, REGULAR, white(0.78));
    let file = sh.store.version_dir(v).join("settings.toml");
    text(&cx.p, pos2(x, rect.bottom() - 24.0), Align2::LEFT_CENTER, &fit_middle(&cx.p, &format!("Kept in {}", file.display()), 12.0, REGULAR, inner), 12.0, REGULAR, white(0.42));

    if cx.pressed(Nav::Back) || (cx.clicked && cx.pointer.is_some_and(|p| !rect.contains(p))) {
        actions.push(Action::Close);
    }
    hints(cx, pos2(s.left() + 72.0, s.bottom() - 40.0), &[(Hint::Move, "Choose and change"), (Hint::Back, "Back")], false);
    actions
}
