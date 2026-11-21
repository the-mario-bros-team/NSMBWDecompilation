//! The game folder: a sheet that explains what is needed, shows the folder
//! and what picking one found, and a folder browser that works with a
//! controller as well as the mouse (Windows' own dialog is one click away).

use std::path::{Path, PathBuf};

use egui::{pos2, vec2, Align2, Color32, Rect, Stroke, StrokeKind};

use crate::app::{Action, Check, Shared};
use crate::disc::{self, Found, Level};
use crate::draw::*;
use crate::nav::Nav;

// ------------------------------------------------------------------ sheet

pub fn draw(sh: &Shared, cx: &mut Cx, focus: &mut Focus, t: f32) -> Vec<Action> {
    let mut actions = Vec::new();
    let v = &sh.versions[sh.selected];
    let slot = &sh.slots[sh.selected];
    let rect = Rect::from_center_size(cx.screen.center() + vec2(0.0, 26.0 * (1.0 - ease_out(t))), vec2(900.0, 560.0));
    panel(&cx.p, rect, 28.0);
    let x = rect.left() + 40.0;
    let width = rect.width() - 80.0;
    let mut y = rect.top() + 40.0;

    spaced(&cx.p, pos2(x, y + 8.0), Align2::LEFT_CENTER, "GAME FOLDER", 14.0, BOLD, cx.pal.accent[0], 2.4);
    y += 26.0;
    text(&cx.p, pos2(x, y), Align2::LEFT_TOP, &v.title, 30.0, BLACK, Color32::WHITE);
    y += 48.0;
    let wanted: Vec<String> = v
        .discs
        .iter()
        .map(|d| {
            let id = d.revision.map_or(d.id.clone(), |r| format!("{} revision {r}", d.id));
            if d.name.is_empty() {
                id
            } else {
                format!("{} ({id})", d.name)
            }
        })
        .collect();
    let about = format!(
        "The port plays your own copy of the game. Extract it with Dolphin (right-click the game, Properties, Filesystem, right-click the disc, Extract Entire Disc), then pick the folder that holds sys and files. This version needs: {}.",
        wanted.join(" or ")
    );
    y += wrapped(&cx.p, pos2(x, y), width, &about, 15.5, REGULAR, white(0.82)) + 18.0;

    // The folder now.
    let now = Rect::from_min_size(pos2(x, y), vec2(width, 58.0));
    cx.p.rect_filled(now, 14.0, black(0.32));
    cx.p.rect_stroke(now, 14.0, Stroke::new(1.2, white(0.12)), StrokeKind::Inside);
    icon(&cx.p, Icon::Folder, pos2(now.left() + 30.0, now.center().y), 22.0, white(0.75));
    match &slot.game {
        Some(game) => {
            let path = fit_middle(&cx.p, &game.to_string_lossy(), 16.0, SEMI, width - 330.0);
            text(&cx.p, pos2(now.left() + 58.0, now.center().y), Align2::LEFT_CENTER, &path, 16.0, SEMI, Color32::WHITE);
            let (color, words) = match (&slot.found, v.playable, &slot.problem) {
                (None, _, _) => (BAD, "Not found any more".to_string()),
                (Some(f), false, _) => (IDLE, format!("{} rev {}", f.id, f.revision)),
                (Some(_), true, Some(_)) => (BAD, "Cannot start".to_string()),
                (Some(f), true, None) => (GOOD, format!("{} rev {}", f.id, f.revision)),
            };
            let w = measure(&cx.p, &words, 14.5, SEMI).x;
            dot(&cx.p, cx.time, pos2(now.right() - 34.0 - w, now.center().y), color);
            text(&cx.p, pos2(now.right() - 20.0, now.center().y), Align2::RIGHT_CENTER, &words, 14.5, SEMI, white(0.9));
        }
        None => {
            text(&cx.p, pos2(now.left() + 58.0, now.center().y), Align2::LEFT_CENTER, "No folder set yet", 16.0, SEMI, white(0.6));
        }
    }
    y += 58.0 + 18.0;

    // What the last pick found.
    let lines_at = pos2(x + 34.0, y);
    match &sh.check {
        Check::None => {
            text(&cx.p, pos2(x, y + 10.0), Align2::LEFT_CENTER, "You can also drop the folder onto this window.", 14.5, SEMI, white(0.5));
        }
        Check::Running { found } => {
            for k in 0..3 {
                let beat = (cx.time * 5.0 - k as f32 * 0.6).sin().max(0.0);
                cx.p.circle_filled(pos2(x + 6.0 + k as f32 * 10.0, y + 11.0 - 4.0 * beat), 3.5, with_alpha(cx.pal.accent[0], 0.5 + 0.5 * beat));
            }
            text(&cx.p, pos2(x + 44.0, y + 10.0), Align2::LEFT_CENTER, &format!("Found {}. Checking every file...", found.describe()), 16.0, SEMI, Color32::WHITE);
        }
        Check::Done { found, verdict, report, saved } => {
            let (mark, color, head) = match (verdict, saved, v.playable) {
                (Ok(()), true, true) => (Icon::Check, GOOD, format!("This is the right game ({}). The folder is set.", found.as_ref().map_or(String::new(), Found::describe))),
                (Ok(()), true, false) => (Icon::Check, IDLE, format!("Found {}. The folder is kept, and Play stays off until the port can run this version.", found.as_ref().map_or(String::new(), Found::describe))),
                (Ok(()), false, _) => (Icon::Cross, BAD, "The folder could not be saved.".to_string()),
                (Err(e), _, _) => (Icon::Cross, BAD, e.clone()),
            };
            icon(&cx.p, mark, pos2(x + 11.0, y + 11.0), 20.0, color);
            let mut ly = lines_at.y + wrapped(&cx.p, lines_at, width - 34.0, &head, 16.0, SEMI, Color32::WHITE) + 8.0;
            if let Some(report) = report {
                let fails: Vec<_> = report.lines.iter().filter(|l| l.0 == Level::Fail).collect();
                // The failures when there are any; else what passed.
                let shown: Vec<_> = if fails.is_empty() { report.lines.iter().collect() } else { fails };
                for (level, words) in shown.into_iter().take(4) {
                    let (mark, color) = if *level == Level::Ok { (Icon::Check, GOOD) } else { (Icon::Cross, BAD) };
                    icon(&cx.p, mark, pos2(lines_at.x + 7.0, ly + 9.0), 13.0, color);
                    text(&cx.p, pos2(lines_at.x + 22.0, ly + 9.0), Align2::LEFT_CENTER, &fit_middle(&cx.p, words, 13.5, REGULAR, width - 70.0), 13.5, REGULAR, white(0.72));
                    ly += 21.0;
                }
                if let Some(e) = &report.error {
                    wrapped(&cx.p, pos2(lines_at.x, ly), width - 34.0, e, 13.5, REGULAR, WARN);
                }
            }
        }
    }

    // Buttons along the bottom.
    focus.begin();
    let by = rect.bottom() - 40.0 - 52.0;
    let mut bx = x;
    let mut place = |w: f32| {
        let r = Rect::from_min_size(pos2(bx, by), vec2(w, 52.0));
        bx += w + 14.0;
        r
    };
    let busy = matches!(sh.check, Check::Running { .. });
    if button(cx, focus, Button::new("browse", place(190.0), "Choose folder").icon(Icon::Folder).on(slot.game.is_none())) && !busy {
        actions.push(Action::OpenBrowser);
    }
    if button(cx, focus, Button::new("windows", place(190.0), "Windows dialog")) && !busy {
        actions.push(Action::WindowsDialog);
    }
    if slot.game.is_some() && button(cx, focus, Button::new("forget", place(130.0), "Remove")) && !busy {
        actions.push(Action::ForgetFolder);
    }
    let done = Rect::from_min_size(pos2(rect.right() - 40.0 - 130.0, by), vec2(130.0, 52.0));
    if button(cx, focus, Button::new("done", done, "Done").on(slot.game.is_some())) || cx.pressed(Nav::Back) {
        actions.push(Action::Close);
    }
    focus.end(&cx.nav);
    hints(cx, pos2(rect.center().x - 92.0, rect.bottom() + 34.0), &[(Hint::Confirm, "Choose"), (Hint::Back, "Close")], false);
    actions
}

// ---------------------------------------------------------------- browser

struct Entry {
    name: String,
    path: PathBuf,
    /// The game in it, when it is an extracted disc.
    game: Option<Found>,
}

/// A folder browser: drives, then folders; folders holding an extracted
/// game are marked and can be picked directly.
pub struct Browser {
    /// The folder shown; `None` lists the drives.
    dir: Option<PathBuf>,
    entries: Vec<Entry>,
    index: usize,
    /// The game in `dir` itself.
    here: Option<Found>,
    /// The button under the mouse.
    buttons: Focus,
}

fn hidden(path: &Path) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        // The entry's own attributes: a hidden link is hidden wherever it leads.
        std::fs::symlink_metadata(path).is_ok_and(|m| m.file_attributes() & 0x6 != 0)
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        false
    }
}

/// The letters of the computer's drives.
fn drives() -> Vec<char> {
    #[cfg(windows)]
    {
        // SAFETY: a plain query; it does not touch the drives (a network
        // drive that is away would make looking at it slow).
        let mask = unsafe { windows_sys::Win32::Storage::FileSystem::GetLogicalDrives() };
        (0..26u8).filter(|i| mask & (1 << i) != 0).map(|i| (b'A' + i) as char).collect()
    }
    #[cfg(not(windows))]
    Vec::new()
}

impl Browser {
    /// Opens at `start`, or at the list of drives.
    pub fn new(start: Option<PathBuf>) -> Self {
        let mut b = Self { dir: None, entries: Vec::new(), index: 0, here: None, buttons: Focus::default() };
        b.go(start.filter(|d| d.is_dir()), None);
        b
    }

    /// Shows `dir`, with the entry named `select` highlighted.
    fn go(&mut self, dir: Option<PathBuf>, select: Option<&Path>) {
        self.entries.clear();
        match &dir {
            None => {
                for letter in drives() {
                    self.entries.push(Entry { name: format!("{letter}:"), path: PathBuf::from(format!("{letter}:\\")), game: None });
                }
            }
            Some(d) => {
                let mut folders: Vec<PathBuf> = std::fs::read_dir(d).map(|rd| rd.flatten().map(|e| e.path()).filter(|p| p.is_dir() && !hidden(p)).collect()).unwrap_or_default();
                folders.sort_by_key(|p| p.file_name().map(|n| n.to_string_lossy().to_lowercase()));
                for path in folders {
                    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                    let game = disc::find_root(&path).and_then(|_| disc::identify(&path).ok());
                    self.entries.push(Entry { name, path, game });
                }
            }
        }
        self.here = dir.as_deref().and_then(|d| disc::identify(d).ok());
        self.index = select.and_then(|s| self.entries.iter().position(|e| e.path == s)).unwrap_or(0);
        self.dir = dir;
    }

    /// The folder shown.
    pub fn dir(&self) -> Option<&Path> {
        self.dir.as_deref()
    }

    /// One level up; false at the list of drives.
    fn up(&mut self) -> bool {
        let Some(dir) = self.dir.clone() else { return false };
        let parent = dir.parent().map(PathBuf::from);
        self.go(parent, Some(&dir));
        true
    }
}

const ROW: f32 = 46.0;

pub fn browse(_sh: &Shared, cx: &mut Cx, b: &mut Browser, t: f32) -> Vec<Action> {
    let mut actions = Vec::new();
    let rect = Rect::from_center_size(cx.screen.center() + vec2(0.0, 26.0 * (1.0 - ease_out(t))), vec2(920.0, 622.0));
    panel(&cx.p, rect, 28.0);
    let x = rect.left() + 36.0;
    let width = rect.width() - 72.0;

    spaced(&cx.p, pos2(x, rect.top() + 44.0), Align2::LEFT_CENTER, "CHOOSE THE GAME FOLDER", 14.0, BOLD, cx.pal.accent[0], 2.4);
    let place = b.dir.as_ref().map_or("This PC".to_string(), |d| d.to_string_lossy().into_owned());
    text(&cx.p, pos2(x, rect.top() + 78.0), Align2::LEFT_CENTER, &fit_middle(&cx.p, &place, 22.0, BOLD, width), 22.0, BOLD, Color32::WHITE);
    let (color, words) = match (&b.here, &b.dir) {
        (Some(found), _) => (GOOD, format!("This folder holds a game: {}", found.describe())),
        (None, Some(_)) => (IDLE, "Open the folder that holds sys and files, or pick one marked GAME.".to_string()),
        (None, None) => (IDLE, "Pick a drive.".to_string()),
    };
    dot(&cx.p, cx.time, pos2(x + 7.0, rect.top() + 110.0), color);
    text(&cx.p, pos2(x + 24.0, rect.top() + 110.0), Align2::LEFT_CENTER, &words, 14.5, SEMI, white(0.85));

    // Moves.
    let n = b.entries.len();
    if cx.pressed(Nav::Up) {
        b.index = b.index.saturating_sub(1);
    }
    if cx.pressed(Nav::Down) && n > 0 {
        b.index = (b.index + 1).min(n - 1);
    }
    if cx.pressed(Nav::Prev) {
        b.index = b.index.saturating_sub(8);
    }
    if cx.pressed(Nav::Next) && n > 0 {
        b.index = (b.index + 8).min(n - 1);
    }
    let mut open = cx.pressed(Nav::Confirm).then_some(b.index).filter(|_| n > 0);

    // The list.
    let list = Rect::from_min_max(pos2(x, rect.top() + 134.0), pos2(rect.right() - 36.0, rect.bottom() - 110.0));
    cx.p.rect_filled(list, 14.0, black(0.30));
    let visible = (list.height() / ROW).floor();
    let name = ("browser-scroll", b.dir.clone());
    let mut first = cx.anim.to(&name, 0.0, 0.0, cx.dt);
    // The wheel scrolls; a move brings the highlighted row back into view.
    if cx.hovered(list) && cx.wheel != 0.0 {
        first -= cx.wheel / ROW;
    }
    if !cx.nav.is_empty() || first > (n as f32 - visible).max(0.0) {
        first = first.clamp((b.index as f32 + 1.0 - visible).max(0.0), b.index as f32).min((n as f32 - visible).max(0.0));
    }
    let first = first.clamp(0.0, (n as f32 - visible).max(0.0));
    cx.anim.set(&name, first);
    let shown = cx.anim.to(("browser-shown", b.dir.clone()), first, 18.0, cx.dt);
    let rows = cx.p.with_clip_rect(list.shrink(2.0));
    if n == 0 {
        text(&rows, list.center(), Align2::CENTER_CENTER, "No folders in here", 15.0, SEMI, white(0.5));
    }
    for (i, e) in b.entries.iter().enumerate() {
        let row = Rect::from_min_size(pos2(list.left() + 6.0, list.top() + 6.0 + (i as f32 - shown) * ROW), vec2(list.width() - 12.0, ROW - 4.0));
        if row.bottom() < list.top() || row.top() > list.bottom() {
            continue;
        }
        if cx.hovered(row) && cx.hovered(list) {
            if cx.moved {
                b.index = i;
            }
            if cx.clicked {
                open = Some(i);
            }
        }
        let f = cx.anim.to(("browser-row", i, b.dir.clone()), if b.index == i { 1.0 } else { 0.0 }, 20.0, cx.dt);
        if f > 0.01 {
            rows.rect_filled(row, 11.0, white(0.11 * f));
            rows.rect_stroke(row, 11.0, Stroke::new(2.0, white(0.85 * f)), StrokeKind::Inside);
        }
        icon(&rows, if b.dir.is_none() { Icon::Drive } else { Icon::Folder }, pos2(row.left() + 26.0, row.center().y), 20.0, white(0.72));
        let room = row.width() - 60.0 - if e.game.is_some() { 250.0 } else { 40.0 };
        text(&rows, pos2(row.left() + 52.0, row.center().y), Align2::LEFT_CENTER, &fit_middle(&rows, &e.name, 16.0, SEMI, room), 16.0, SEMI, Color32::WHITE);
        match &e.game {
            Some(found) => {
                let label = format!("GAME  ·  {} rev {}", found.id, found.revision);
                let w = measure(&rows, &label, 12.0, BOLD).x + 22.0;
                let tag = Rect::from_min_size(pos2(row.right() - 14.0 - w, row.center().y - 12.0), vec2(w, 24.0));
                rows.rect_filled(tag, 12.0, with_alpha(cx.pal.accent[0], 0.22));
                rows.rect_stroke(tag, 12.0, Stroke::new(1.2, cx.pal.accent[0]), StrokeKind::Inside);
                text(&rows, tag.center(), Align2::CENTER_CENTER, &label, 12.0, BOLD, Color32::WHITE);
            }
            None => icon(&rows, Icon::ChevronRight, pos2(row.right() - 22.0, row.center().y), 14.0, white(0.4)),
        }
    }

    // Buttons, for the mouse; the hints name the controller's. They are
    // pressed by the mouse or their own controller buttons, never by the
    // confirm that opens a row, so they are drawn without the frame's moves
    // and are only highlighted under the mouse.
    let by = rect.bottom() - 36.0 - 52.0;
    let spot = |i: usize, w: f32| Rect::from_min_size(pos2(x + [0.0, 224.0, 408.0][i], by), vec2(w, 52.0));
    let spots = [spot(0, 210.0), spot(1, 170.0), spot(2, 190.0), Rect::from_min_size(pos2(rect.right() - 36.0 - 130.0, by), vec2(130.0, 52.0))];
    if !spots.iter().any(|r| cx.hovered(*r)) {
        b.buttons.set("none");
    }
    let moves = std::mem::take(&mut cx.nav);
    let use_here = button(cx, &mut b.buttons, Button::new("use-here", spots[0], "Use this folder").icon(Icon::Check).on(b.here.is_some()).dim(b.here.is_none()));
    let go_up = button(cx, &mut b.buttons, Button::new("go-up", spots[1], "Up one level").icon(Icon::Up).dim(b.dir.is_none()));
    let windows = button(cx, &mut b.buttons, Button::new("windows-2", spots[2], "Windows dialog"));
    let cancel = button(cx, &mut b.buttons, Button::new("cancel-browse", spots[3], "Cancel"));
    cx.nav = moves;

    if let Some(i) = open {
        let e = &b.entries[i];
        if e.game.is_some() {
            actions.push(Action::PickFolder(e.path.clone()));
        } else {
            let path = e.path.clone();
            b.go(Some(path), None);
        }
    } else if (use_here || cx.pressed(Nav::Extra)) && b.here.is_some() {
        actions.push(Action::PickFolder(b.dir.clone().unwrap_or_default()));
    } else if windows {
        actions.extend([Action::Close, Action::WindowsDialog]);
    } else if cancel || ((go_up || cx.pressed(Nav::Back)) && !b.up() && !go_up) {
        actions.push(Action::Close);
    }
    hints(cx, pos2(rect.center().x - 250.0, rect.bottom() + 34.0), &[(Hint::Confirm, "Open"), (Hint::Extra, "Use this folder"), (Hint::Back, "Up one level"), (Hint::Sides, "Page")], false);
    actions
}
