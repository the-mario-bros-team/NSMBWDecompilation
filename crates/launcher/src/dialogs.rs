//! Small windows over the launcher: a message, the question before Skip
//! Launcher is switched on, and the notice of a new release.

use egui::{pos2, vec2, Align2, Color32, Rect};

use crate::app::{Action, Shared, UpdateStatus};
use crate::draw::*;
use crate::nav::Nav;

enum What {
    Message { title: String, body: String },
    Skip { game: String },
    Update,
}

pub struct Dialog {
    what: What,
    focus: Focus,
}

impl Dialog {
    pub fn message(title: &str, body: &str) -> Self {
        Self { what: What::Message { title: title.into(), body: body.into() }, focus: Focus::default() }
    }

    /// Asked before Skip Launcher is switched on, for the game it will start.
    pub fn skip(game: &str) -> Self {
        Self { what: What::Skip { game: game.into() }, focus: Focus::default() }
    }

    /// The newest release (`Shared::update`).
    pub fn update() -> Self {
        Self { what: What::Update, focus: Focus::default() }
    }
}

/// The release notes, cut to what a small window holds.
fn excerpt(notes: &str) -> String {
    let lines: Vec<&str> = notes.lines().map(str::trim_end).filter(|l| !l.trim().is_empty()).take(9).collect();
    let mut text = lines.join("\n");
    if text.chars().count() > 620 {
        text = text.chars().take(620).collect::<String>() + "...";
    } else if lines.len() < notes.lines().filter(|l| !l.trim().is_empty()).count() {
        text.push_str("\n...");
    }
    text
}

pub fn draw(sh: &Shared, cx: &mut Cx, dialog: &mut Dialog, t: f32) -> Vec<Action> {
    let mut actions = Vec::new();
    let (eyebrow, title, body, buttons): (&str, String, String, Vec<(&str, &str, Option<Icon>)>) = match &dialog.what {
        What::Message { title, body } => ("", title.clone(), body.clone(), vec![("ok", "OK", None)]),
        What::Skip { game } => (
            "SKIP LAUNCHER",
            "Go straight to the game?".into(),
            format!(
                "From the next start, the launcher opens {game} right away without showing this screen.\n\n\
                 To come back here, hold Shift while it starts, or hold Back on an Xbox controller. When a new release comes out, the launcher shows itself once to tell you."
            ),
            vec![("skip-on", "Turn on", Some(Icon::Skip)), ("cancel", "Cancel", None)],
        ),
        What::Update => match &sh.update.status {
            UpdateStatus::Available(r) => {
                let mut body = format!("You have version {}.", port_update::VERSION);
                if r.name != r.version {
                    body = format!("{}\n{body}", r.name);
                }
                if let Some(d) = &r.download {
                    body.push_str(&format!("\nDownload: {} ({:.1} MB)", d.name, d.size as f64 / 1_048_576.0));
                }
                let notes = excerpt(&r.notes);
                if !notes.is_empty() {
                    body.push_str(&format!("\n\n{notes}"));
                }
                let mut buttons = vec![("later", "Later", None)];
                if !r.page.is_empty() {
                    buttons.insert(0, ("page", "Open the release page", Some(Icon::Download)));
                }
                ("UPDATE", format!("Version {} is out", r.version), body, buttons)
            }
            _ => ("UPDATE", "No new release".into(), format!("Version {} is the newest.", port_update::VERSION), vec![("ok", "OK", None)]),
        },
    };

    let width = 640.0;
    let pad = 36.0;
    let body_galley = cx.p.layout(body, font(16.5, REGULAR), white(0.86), width - pad * 2.0);
    let title_galley = cx.p.layout(title, font(28.0, BLACK), Color32::WHITE, width - pad * 2.0);
    let head = if eyebrow.is_empty() { 0.0 } else { 28.0 };
    let height = pad + head + title_galley.size().y + 16.0 + body_galley.size().y + 30.0 + 52.0 + pad;
    let rect = Rect::from_center_size(cx.screen.center() + vec2(0.0, 26.0 * (1.0 - ease_out(t))), vec2(width, height));
    panel(&cx.p, rect, 26.0);
    let mut y = rect.top() + pad;
    let x = rect.left() + pad;
    if !eyebrow.is_empty() {
        spaced(&cx.p, pos2(x, y + 8.0), Align2::LEFT_CENTER, eyebrow, 14.0, BOLD, cx.pal.accent[0], 2.4);
        y += head;
    }
    let title_h = title_galley.size().y;
    cx.p.galley(pos2(x, y), title_galley, Color32::WHITE);
    y += title_h + 16.0;
    let body_h = body_galley.size().y;
    cx.p.galley(pos2(x, y), body_galley, white(0.86));
    y += body_h + 30.0;

    // The buttons, right-aligned; the first is the one suggested.
    dialog.focus.begin();
    let mut right = rect.right() - pad;
    for (i, (name, label, icon)) in buttons.iter().enumerate().rev() {
        let w = measure(&cx.p, label, 16.0, BOLD).x + if icon.is_some() { 84.0 } else { 56.0 };
        let r = Rect::from_min_size(pos2(right - w, y), vec2(w.max(120.0), 52.0));
        let r = r.translate(vec2(w - r.width(), 0.0));
        right = r.left() - 14.0;
        let mut b = Button::new(name, r, label).on(i == 0 && buttons.len() > 1);
        if let Some(icon) = icon {
            b = b.icon(*icon);
        }
        if button(cx, &mut dialog.focus, b) {
            match *name {
                "skip-on" => actions.extend([Action::SetSkip(true), Action::Close]),
                "page" => actions.extend([Action::OpenReleasePage, Action::Close]),
                _ => actions.push(Action::Close),
            }
        }
    }
    // Focus starts on the first button, whichever was drawn first.
    if dialog.focus.current.is_none() || t < 0.05 {
        dialog.focus.set(buttons[0].0);
    }
    dialog.focus.end(&cx.nav);
    if cx.pressed(Nav::Back) {
        actions.push(Action::Close);
    }
    hints(cx, pos2(rect.center().x, rect.bottom() + 34.0) - vec2(92.0, 0.0), &[(Hint::Confirm, "Choose"), (Hint::Back, "Close")], false);
    actions
}
