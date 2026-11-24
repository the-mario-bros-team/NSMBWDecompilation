//! Clicking (or tapping) the game's own menus, after WitherBlitz's
//! NSMB-Wii-Recompilation ("Tap or click the menus"): a click becomes the
//! Wii Remote presses that reach what was clicked, as a player would press
//! them. Nothing in the game is written; the presses go to player 1's remote
//! like the keyboard's, so the game reacts with its own cursor moves,
//! animations and sounds.
//!
//! Clicks are in layout units: the picture is 456 units high, the origin at
//! its centre, y up (a 16:9 picture is about ±405 across). Each press lasts
//! a step of [`STEP`] fields, held for [`HOLD`] of them, and what to press
//! is decided at the start of each step from what the game shows then:
//! - the title screen, the strap screen and the attract movies: 2 and A;
//! - a menu with the corner-bracket cursor (the pause window, the world
//!   map's + menu, yes/no windows, ...): the D-pad toward the click until
//!   the cursor's box holds it, then 2;
//! - the save select: its files, Free-for-All, Coin Battle, Copy and Erase,
//!   and the port's Settings (cog) and Multiplayer buttons; its Back button
//!   is the window's own ([`Clicker::back`]);
//! - Select Players: 1 to 4 players, and Back;
//! - the character screen: player 1's OK bar, its arrows, and Back;
//! - the world map: its button hints (Menu, Select World, View Map,
//!   Items), the Star Coins screen's Back, Select a World's islands and
//!   Back, and its courses, Toad houses, towers and castles: the player
//!   walks there point by point (the map's own paths, found breadth first
//!   and pressed as D-pad moves), and a click on the point it stands on
//!   enters it.
//!
//! A click while the map's player walks stops the walk and is taken
//! afresh; while anything else is under way it is ignored. Not in net
//! games.

use game_version::E1;
use hle_ios::wiimote::button;
use hle_os::System;

use crate::say::say;

/// Fields per press: held for [`HOLD`], then let go for the rest.
pub const STEP: u32 = 8;
pub const HOLD: u32 = 4;

// The sideways remote's D-pad, by direction on screen.
const SCREEN_LEFT: u16 = button::UP;
const SCREEN_RIGHT: u16 = button::DOWN;
const SCREEN_UP: u16 = button::RIGHT;
const SCREEN_DOWN: u16 = button::LEFT;

const SCENE: E1 = E1(0x8042_8730); // dScene_c: the low half is the scene now
const SCENE_STRAP: u32 = 0;
const SCENE_WORLD_MAP: u32 = 3;
const SCENE_STAGE: u32 = 5;
const SCENE_MOVIE: u32 = 8;
const SCENE_GAME_SETUP: u32 = 0xA; // the save select and Select Players
const TREE_ROOT: E1 = E1(0x8037_7A10); // fManager_c's tree of the game's objects
const DEMO_MODE: E1 = E1(0x8042_A204); // 3: the title's attract demo
/// The layouts' horizontal squeeze (608 / the canvas's width): layout x
/// times this is a pane's x.
const SQUEEZE: E1 = E1(0x8042_B084);

const PROFILE_TITLE: u16 = 0x2BB;
const PROFILE_FILE_SELECT: u16 = 0x2D5;
const PROFILE_SELECT_PLAYERS: u16 = 0x2BC;
const PROFILE_CHARACTER: u16 = 0x2B6;
const PROFILE_MAP_MENU: u16 = 0x2B5;
const PROFILE_SELECT_CURSOR: u16 = 0x2CF;

const FILE_SELECT_STATE: u32 = 0x238;
const FILE_SELECT_WAIT: E1 = E1(0x8099_2050); // dFileSelect_c::StateID_SelectWait
const FILE_SELECT_CURSOR: u32 = 0x2A4; // 0-2 files, 3 Free-for-All, 4 Coin Battle, 5 Settings, 6 Multiplayer
const FILE_SELECT_LAYOUT: u32 = 0x74;
const SELECT_PLAYERS_STATE: u32 = 0x9C;
const SELECT_PLAYERS_CURSOR: u32 = 0x26C; // 0-3: 1-4 players
const SELECT_PLAYERS_SELECT: E1 = E1(0x8099_54C0);
const SELECT_PLAYERS_CHANGE: E1 = E1(0x8099_5480);
const MAP_MENU_STATE: u32 = 0x234;
const PROFILE_STAR_COINS: u16 = 0x2B4;
const STAR_COINS_STATE: u32 = 0x434;
const PROFILE_WORLD_SELECT: u16 = 0x2B1;
const WORLD_SELECT_STATE: u32 = 0x238;
const WORLD_SELECT_WORLD: u32 = 0x27C; // 0-8
const WORLD_SELECT_OPEN: u32 = 0x2A8; // a byte per world: it can be picked

// nw4r::lyt::Pane
const PANE_SIZE: u32 = 0x4C;
const PANE_GLOBAL: u32 = 0x84; // 3x4 row-major
const PANE_GLOBAL_ALPHA: u32 = 0xB9;
const PANE_BASE: u32 = 0xBA;
const PANE_FLAGS: u32 = 0xBB; // bit 0: visible

/// A click from the window: where (layout units), and half the window's
/// width in the same units (for things placed from the screen's edge).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Click {
    pub x: f32,
    pub y: f32,
    pub half_width: f32,
}

#[derive(Default)]
pub struct Clicker {
    job: Option<Job>,
    /// The field within the current step.
    tick: u32,
    /// What the current step presses.
    press: u16,
}

enum Job {
    /// Press these once.
    Once(u16),
    /// Steer the corner-bracket cursor onto the click, then press 2.
    Cursor(Steer),
    /// Move a screen's cursor (a number) to `target`, then press 2.
    Grid { screen: Grid, target: u32, steps: u32 },
    /// Turn Select a World's ring to world `target` (0-7).
    World { target: u32, presses: u32, steps: u32 },
    /// Walk the world map's player to point `target`.
    Walk(Walk),
}

struct Walk {
    target: usize,
    /// Fields so far, and how many in a row the player stood still.
    reads: u32,
    still: u32,
    last_pos: Option<[f32; 3]>,
    /// The hop the last press asked for, and how often it didn't move.
    pressed: Option<(usize, usize)>,
    fails: u32,
    /// Links that would not walk.
    blocked: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Grid {
    /// The save select: files 0-2 across the top, Free-for-All (3) and
    /// Coin Battle (4) below, the port's Multiplayer (6) and Settings (5)
    /// above, at the right.
    FileSelect,
    /// Select Players: 1 player (0) on top, 2-4 players (1-3) below.
    SelectPlayers,
}

struct Steer {
    /// The click, in pane units.
    x: f32,
    y: f32,
    steps: u32,
    stuck: u32,
    /// The cursor's centre at the last step.
    last: Option<(f32, f32)>,
    /// The last step pressed a direction (vertical or not).
    pressed: Option<bool>,
    /// The axis a press found blocked.
    blocked: Option<bool>,
}

/// What a step does.
enum Step {
    Press(u16),
    Wait,
    Done,
}

impl Clicker {
    /// After each field: a new click (if any), then the buttons player 1's
    /// remote presses during the next field.
    pub fn field(&mut self, sys: &mut System, click: Option<Click>) -> u16 {
        if let Some(c) = click {
            // A click while the map's player walks stops the walk (and is
            // taken afresh); while anything else is under way it is ignored.
            if matches!(self.job, Some(Job::Walk(_))) {
                self.job = None;
            }
            if self.job.is_none() {
                self.job = start(sys, c);
                self.tick = 0;
                if std::env::var_os("NSMBW_CLICK_LOG").is_some() {
                    log_click(sys, c, self.job.as_ref());
                }
            }
        }
        let Some(job) = &mut self.job else { return 0 };
        if let Job::Walk(w) = job {
            w.observe(sys);
        }
        if self.tick == 0 {
            match step(sys, job) {
                Step::Press(b) => self.press = b,
                Step::Wait => self.press = 0,
                Step::Done => {
                    self.job = None;
                    return 0;
                }
            }
        }
        let held = if self.tick < HOLD { self.press } else { 0 };
        self.tick = (self.tick + 1) % STEP;
        held
    }

    /// The save select's Back button (the window's, after
    /// NSMB-Wii-Recompilation's): 1, the screen's way back to the title,
    /// while it waits for a choice and nothing else is under way.
    pub fn back(&mut self, sys: &System) {
        let waiting = scene(sys) == SCENE_GAME_SETUP && find(sys, PROFILE_FILE_SELECT).is_some_and(|fs| r32(sys, fs + FILE_SELECT_STATE) == FILE_SELECT_WAIT.va());
        if waiting && self.job.is_none() {
            self.job = Some(Job::Once(button::ONE));
            self.tick = 0;
        }
    }
}

/// The save select's buttons the port draws (Settings as a cog, and
/// Multiplayer, in the title bar), as the game thread last saw them
/// ([`save_select`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SaveSelect {
    /// Settings' and Multiplayer's places in layout units (x0, y0, x1, y1;
    /// y up), from the empty panes the game's layout keeps for them.
    pub buttons: [(f32, f32, f32, f32); 2],
    /// Their places' alpha (they fade with the title).
    pub alpha: f32,
    /// The one with the game's cursor (0 Settings, 1 Multiplayer).
    pub cursor: Option<u8>,
    /// The one picked: its hit plays, then its window is up.
    pub picked: Option<u8>,
    /// The screen waits for a choice (the Back button can show).
    pub waiting: bool,
}

/// The save select's buttons the port draws over the title bar
/// ([`SaveSelect`]), from their empty panes in the game's layout;
/// `None` when it is not up (or has no such panes).
pub fn save_select(sys: &mut System) -> Option<SaveSelect> {
    if scene(sys) != SCENE_GAME_SETUP {
        return None;
    }
    let fs = find(sys, PROFILE_FILE_SELECT)?;
    let root = sys.root_pane(fs + FILE_SELECT_LAYOUT);
    if root == 0 {
        return None;
    }
    let mut buttons = [(0.0, 0.0, 0.0, 0.0); 2];
    let mut alpha = 0;
    for (i, name) in [hle_os::menu_layout::FILE_COG_PANE, hle_os::menu_layout::FILE_MULTI_PANE].into_iter().enumerate() {
        let pane = sys.find_pane(root, name, 0);
        if pane == 0 || !shown(sys, pane) {
            return None;
        }
        let (x0, y0, x1, y1) = pane_box(sys, pane)?;
        let squeeze = pane_squeeze(sys, pane);
        buttons[i] = (x0 / squeeze, y0, x1 / squeeze, y1);
        alpha = r8(sys, pane + PANE_GLOBAL_ALPHA);
    }
    // The game's numbers for them (5 Settings, 6 Multiplayer) to 0 and 1.
    let ours = |c: u32| (5..=6).contains(&c).then(|| (c - 5) as u8);
    Some(SaveSelect {
        buttons,
        alpha: f32::from(alpha) / 255.0,
        cursor: ours(r32(sys, fs + FILE_SELECT_CURSOR)),
        picked: sys.menu.file_picked().and_then(ours),
        waiting: r32(sys, fs + FILE_SELECT_STATE) == FILE_SELECT_WAIT.va(),
    })
}

/// `NSMBW_CLICK_LOG`: what a click started, and on the world map where its
/// places are on screen (for scripting `--click`).
fn log_click(sys: &mut System, c: Click, job: Option<&Job>) {
    let what = match job {
        None => "nothing".to_string(),
        Some(Job::Once(b)) => format!("press {b:#06x}"),
        Some(Job::Cursor(_)) => format!("steer the cursor (box {:?})", cursor_box(sys)),
        Some(Job::Grid { screen, target, .. }) => format!("{screen:?} cursor to {target}"),
        Some(Job::World { target, .. }) => format!("Select a World to world {}", target + 1),
        Some(Job::Walk(w)) => format!("walk to point {}", w.target),
    };
    say!("click at ({:.0}, {:.0}) in scene {}: {what}", c.x, c.y, scene(sys));
    if scene(sys) == SCENE_WORLD_MAP {
        if let Some(points) = map_points(sys) {
            let here = points.current(sys);
            let list: Vec<String> = (0..points.pos.len()).filter(|&i| points.clickable[i]).filter_map(|i| project(sys, points.pos[i]).map(|(x, y)| format!("{i}{} ({x:.0}, {y:.0})", if Some(i) == here { "*" } else { "" }))).collect();
            say!("  map points: {}", list.join(", "));
        }
    }
}

// ---------------------------------------------------------------- reading the game

fn r32(sys: &System, a: u32) -> u32 {
    sys.mem.ram(a, 4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

fn r8(sys: &System, a: u32) -> u8 {
    sys.mem.ram(a, 1).map_or(0, |b| b[0])
}

fn f32_at(sys: &System, a: u32) -> f32 {
    f32::from_bits(r32(sys, a))
}

fn in_ram(a: u32) -> bool {
    (0x8000_0000..0x8180_0000).contains(&a) || (0x9000_0000..0x9400_0000).contains(&a)
}

fn scene(sys: &System) -> u32 {
    r32(sys, SCENE.va()) & 0xFFFF
}

fn squeeze(sys: &System) -> f32 {
    let s = f32_at(sys, SQUEEZE.va());
    if (0.1..=2.0).contains(&s) {
        s
    } else {
        1.0
    }
}

/// Every object of the game's object tree, depth first (+4 child, +0xC
/// next, +0 parent; +0x10 the object, whose profile is the high half of
/// its +8).
fn objects(sys: &System) -> Vec<(u16, u32)> {
    let mut out = Vec::new();
    let mut node = r32(sys, TREE_ROOT.va());
    for _ in 0..4096 {
        if node == 0 {
            break;
        }
        let object = r32(sys, node + 0x10);
        if in_ram(object) {
            out.push(((r32(sys, object + 8) >> 16) as u16, object));
        }
        let child = r32(sys, node + 4);
        if child != 0 {
            node = child;
            continue;
        }
        loop {
            if node == 0 {
                break;
            }
            let next = r32(sys, node + 0xC);
            if next != 0 {
                node = next;
                break;
            }
            node = r32(sys, node);
        }
    }
    out
}

fn find(sys: &System, profile: u16) -> Option<u32> {
    objects(sys).into_iter().find(|o| o.0 == profile).map(|o| o.1)
}

/// The name of the state an object's state manager at `at` is in (its
/// StateID's +4).
fn state_name(sys: &System, at: u32) -> String {
    let id = r32(sys, at);
    let name = if in_ram(id) { r32(sys, id + 4) } else { 0 };
    let mut s = String::new();
    if in_ram(name) {
        for i in 0..96 {
            match r8(sys, name + i) {
                0 => break,
                c => s.push(c as char),
            }
        }
    }
    s
}

/// A pane's box in pane units: (x0, y0, x1, y1), from its global matrix,
/// size and base position; `None` if it is hidden or degenerate.
fn pane_box(sys: &System, pane: u32) -> Option<(f32, f32, f32, f32)> {
    if !in_ram(pane) || r8(sys, pane + PANE_FLAGS) & 1 == 0 || r8(sys, pane + PANE_GLOBAL_ALPHA) == 0 {
        return None;
    }
    let m = |i: u32| f32_at(sys, pane + PANE_GLOBAL + 4 * i);
    let (w, h) = (f32_at(sys, pane + PANE_SIZE) * m(0).abs(), f32_at(sys, pane + PANE_SIZE + 4) * m(5).abs());
    if !(0.5..2000.0).contains(&w) || !(0.5..2000.0).contains(&h) {
        return None;
    }
    let base = u32::from(r8(sys, pane + PANE_BASE)).min(8);
    let x0 = m(3) - w * (base % 3) as f32 / 2.0;
    let y1 = m(7) + h * (base / 3) as f32 / 2.0;
    Some((x0, y1 - h, x0 + w, y1))
}

/// The horizontal squeeze of a pane that keeps its shape on a 16:9 screen
/// (layout x times this is its x): its global x scale over its y scale.
/// The save select's buttons' places use this rather than [`SQUEEZE`],
/// which put them a third too wide in a 4:3 game.
fn pane_squeeze(sys: &System, pane: u32) -> f32 {
    let (sx, sy) = (f32_at(sys, pane + PANE_GLOBAL).abs(), f32_at(sys, pane + PANE_GLOBAL + 0x14).abs());
    let s = if sy > 0.0 { sx / sy } else { 1.0 };
    if (0.1..=2.0).contains(&s) {
        s
    } else {
        1.0
    }
}

/// The corner-bracket cursor's box, in pane units: from the visible
/// `SELECT_CURSOR` layout's four corner panes' global positions.
fn cursor_box(sys: &mut System) -> Option<(f32, f32, f32, f32)> {
    let cursor = find(sys, PROFILE_SELECT_CURSOR)?;
    let roots: Vec<u32> = (0..0x1000).step_by(4).map(|off| r32(sys, cursor + off)).filter(|&p| in_ram(p)).collect();
    for root in roots {
        if pane_name(sys, root) != "RootPane" || r8(sys, root + PANE_FLAGS) & 1 == 0 {
            continue;
        }
        let shown = sys.find_pane(root, "N_cursor_00", 0);
        if shown == 0 || r8(sys, shown + PANE_FLAGS) & 1 == 0 || r8(sys, shown + PANE_GLOBAL_ALPHA) == 0 {
            continue;
        }
        let mut corners = [(0f32, 0f32); 4];
        let mut found = true;
        for (i, name) in ["N_LU_00", "N_RU_00", "N_LD_00", "N_RD_00"].into_iter().enumerate() {
            let p = sys.find_pane(root, name, 0);
            if p == 0 {
                found = false;
                break;
            }
            corners[i] = (f32_at(sys, p + PANE_GLOBAL + 0xC), f32_at(sys, p + PANE_GLOBAL + 0x1C));
        }
        if found {
            let [lu, ru, ld, rd] = corners;
            return Some((lu.0.min(ld.0), ld.1.min(rd.1), ru.0.max(rd.0), lu.1.max(ru.1)));
        }
    }
    None
}

fn pane_name(sys: &System, pane: u32) -> String {
    let mut s = String::new();
    for i in 0..16 {
        match r8(sys, pane + 0xBC + i) {
            0 => break,
            c => s.push(c as char),
        }
    }
    s
}

/// (cx, cy, w, h) holds (x, y).
fn inside(x: f32, y: f32, b: (f32, f32, f32, f32)) -> bool {
    (x - b.0).abs() <= b.2 / 2.0 && (y - b.1).abs() <= b.3 / 2.0
}

// ---------------------------------------------------------------- what a click starts

fn start(sys: &mut System, c: Click) -> Option<Job> {
    let scene = scene(sys);
    let objects = objects(sys);
    let has = |p: u16| objects.iter().any(|o| o.0 == p);
    let attract = (scene == SCENE_STAGE && r32(sys, DEMO_MODE.va()) == 3) || scene == SCENE_MOVIE;
    if scene == SCENE_STRAP || (scene == SCENE_STAGE && has(PROFILE_TITLE)) || attract {
        return Some(Job::Once(button::TWO | button::A));
    }
    let squeeze = squeeze(sys);
    let steer = Job::Cursor(Steer { x: c.x * squeeze, y: c.y, steps: 0, stuck: 0, last: None, pressed: None, blocked: None });
    if scene == SCENE_GAME_SETUP {
        // The save select and Select Players while they wait for a choice
        // (the character screen's object is there all along).
        if let Some(fs) = objects.iter().find(|o| o.0 == PROFILE_FILE_SELECT).map(|o| o.1) {
            if r32(sys, fs + FILE_SELECT_STATE) == FILE_SELECT_WAIT.va() {
                return file_select(sys, fs, c);
            }
        }
        if let Some(sp) = objects.iter().find(|o| o.0 == PROFILE_SELECT_PLAYERS).map(|o| o.1) {
            if [SELECT_PLAYERS_SELECT.va(), SELECT_PLAYERS_CHANGE.va()].contains(&r32(sys, sp + SELECT_PLAYERS_STATE)) {
                return select_players(c);
            }
        }
        if has(PROFILE_CHARACTER) {
            if let Some(job) = character_screen(sys, &objects, c, squeeze) {
                return Some(job);
            }
        }
        return cursor_box(sys).map(|_| steer);
    }
    let object = |p: u16| objects.iter().find(|o| o.0 == p).map(|o| o.1);
    let r = c.half_width;
    if scene == SCENE_WORLD_MAP {
        // The Star Coins screen: its Back, bottom right; nothing else.
        if let Some(coins) = object(PROFILE_STAR_COINS) {
            if state_name(sys, coins + STAR_COINS_STATE).ends_with("StateID_KeyWait") {
                return (c.x > 0.0 && c.y < -170.0).then_some(Job::Once(button::ONE));
            }
        }
        // Select a World: Back, or an island of the ring.
        if let Some(ws) = object(PROFILE_WORLD_SELECT) {
            let state = state_name(sys, ws + WORLD_SELECT_STATE);
            if !state.is_empty() && !state.ends_with("StateID_OnStageWait") {
                return world_select(sys, ws, &state, c);
            }
        }
        // The + menu: its Back (the rest through its cursor).
        if let Some(menu) = object(PROFILE_MAP_MENU) {
            let state = state_name(sys, menu + MAP_MENU_STATE);
            if state.contains("StateID_PauseDisp") || state.contains("StateID_ButtonChangeAnimeEndWait") {
                if inside(c.x, c.y, (189.0, -183.0, 210.0, 40.0)) {
                    return Some(Job::Once(button::ONE));
                }
                return cursor_box(sys).map(|_| steer);
            }
        }
    }
    if cursor_box(sys).is_some() {
        return Some(steer);
    }
    if scene != SCENE_WORLD_MAP {
        return None;
    }
    // The map's button hints, by the screen's edges.
    let between = |v: f32, a: f32, b: f32| (a..=b).contains(&v);
    if between(c.x, r - 150.0, r - 25.0) && between(c.y, 160.0, 202.0) {
        return Some(Job::Once(button::PLUS)); // Menu
    }
    if between(c.x, r - 250.0, r - 25.0) && between(c.y, 122.0, 164.0) {
        return Some(Job::Once(button::MINUS)); // Select World
    }
    if between(c.x, -r + 25.0, -r + 210.0) && between(c.y, -200.0, -152.0) {
        return Some(Job::Once(button::A)); // View Map
    }
    if between(c.x, r - 160.0, r - 25.0) && between(c.y, -200.0, -152.0) {
        return Some(Job::Once(button::ONE)); // Items
    }
    // A course, Toad house, tower or castle on the map.
    let points = map_points(sys)?;
    let target = (0..points.pos.len())
        .filter(|&i| points.clickable[i])
        .filter_map(|i| project(sys, points.pos[i]).map(|(x, y)| (i, (x - c.x).hypot(y - c.y))))
        .filter(|&(_, d)| d <= 30.0)
        .min_by(|a, b| a.1.total_cmp(&b.1))?
        .0;
    if points.current(sys) == Some(target) {
        return Some(Job::Once(button::TWO));
    }
    Some(Job::Walk(Walk { target, reads: 0, still: 0, last_pos: None, pressed: None, fails: 0, blocked: Vec::new() }))
}

/// Select a World's Back, and its ring's islands (front first, then round
/// the ring), in layout units.
const ISLANDS: [(f32, f32); 8] = [(-5.0, -75.0), (164.0, -65.0), (203.0, -7.0), (116.0, 46.0), (0.0, 90.0), (-121.0, 46.0), (-188.0, 2.0), (-179.0, -75.0)];

fn world_select(sys: &System, ws: u32, state: &str, c: Click) -> Option<Job> {
    let r = c.half_width;
    if (r - 150.0..=r - 20.0).contains(&c.x) && (-190.0..=-135.0).contains(&c.y) {
        return Some(Job::Once(button::ONE));
    }
    let world = r32(sys, ws + WORLD_SELECT_WORLD);
    if state != "dWorldSelect_c::StateID_Select" || world >= 8 {
        return None;
    }
    let (k, _) = ISLANDS
        .iter()
        .enumerate()
        .map(|(k, &(x, y))| (k, (x - c.x).hypot(y - c.y) * if k == 0 { 0.6 } else { 1.0 }))
        .filter(|&(_, d)| d <= 130.0)
        .min_by(|a, b| a.1.total_cmp(&b.1))?;
    if k == 0 {
        return Some(Job::Once(button::TWO));
    }
    let target = (world + k as u32) % 8;
    (r8(sys, ws + WORLD_SELECT_OPEN + target) != 0).then_some(Job::World { target, presses: 0, steps: 0 })
}

// ---------------------------------------------------------------- the world map

const WM_MAP: E1 = E1(0x8042_A18C); // the world map's point data
const WM_PLAYER: E1 = E1(0x8042_A1A0); // daWmPlayer_c; +0xAC its position
const WM_CAMERA: E1 = E1(0x8042_A3CC); // +0xF8 camera 0
const POINTS: usize = 192;
const POINT_SIZE: u32 = 0x54;

struct Points {
    pos: Vec<[f32; 3]>,
    /// In use, and a place the player can go into (its name starts with W).
    used: Vec<bool>,
    clickable: Vec<bool>,
    links: Vec<[Option<usize>; 6]>,
}

/// The current sub-map's points: +4 the name, +0xC the position, +0x18 six
/// links (pointers into the same table).
fn map_points(sys: &System) -> Option<Points> {
    let map = r32(sys, WM_MAP.va());
    if !in_ram(map) {
        return None;
    }
    let sub = r32(sys, map + 0x338C);
    if sub > 3 {
        return None;
    }
    let base = map + 0x5_C7F4 + sub * 0x3F08 + 4;
    let mut p = Points { pos: Vec::new(), used: Vec::new(), clickable: Vec::new(), links: Vec::new() };
    for i in 0..POINTS as u32 {
        let e = base + i * POINT_SIZE;
        let first = r8(sys, e + 4);
        p.used.push(first != 0);
        p.clickable.push(first == b'W');
        p.pos.push([f32_at(sys, e + 0xC), f32_at(sys, e + 0x10), f32_at(sys, e + 0x14)]);
        p.links.push(std::array::from_fn(|k| {
            let l = r32(sys, e + 0x18 + 4 * k as u32);
            (l >= base && (l - base) % POINT_SIZE == 0 && ((l - base) / POINT_SIZE) < POINTS as u32).then(|| ((l - base) / POINT_SIZE) as usize)
        }));
    }
    Some(p)
}

impl Points {
    /// The point the player stands on: the nearest in use within 6 units
    /// across the ground.
    fn current(&self, sys: &System) -> Option<usize> {
        let at = player_pos(sys)?;
        (0..self.pos.len()).filter(|&i| self.used[i]).map(|i| (i, (self.pos[i][0] - at[0]).hypot(self.pos[i][2] - at[2]))).filter(|&(_, d)| d <= 6.0).min_by(|a, b| a.1.total_cmp(&b.1)).map(|p| p.0)
    }

    /// The first hop from `from` toward `to`, breadth first, around
    /// `blocked` links.
    fn first_hop(&self, from: usize, to: usize, blocked: &[(usize, usize)]) -> Option<usize> {
        let mut came = vec![usize::MAX; self.pos.len()];
        came[from] = from;
        let mut queue = std::collections::VecDeque::from([from]);
        while let Some(n) = queue.pop_front() {
            if n == to {
                let mut hop = to;
                while came[hop] != from {
                    hop = came[hop];
                }
                return Some(hop);
            }
            for next in self.links[n].iter().flatten().copied() {
                if came[next] == usize::MAX && self.used[next] && !blocked.contains(&(n, next)) {
                    came[next] = n;
                    queue.push_back(next);
                }
            }
        }
        None
    }
}

fn player_pos(sys: &System) -> Option<[f32; 3]> {
    let player = r32(sys, WM_PLAYER.va());
    in_ram(player).then(|| [f32_at(sys, player + 0xAC), f32_at(sys, player + 0xB0), f32_at(sys, player + 0xB4)])
}

/// A world position on screen (layout units) through the map's camera 0:
/// its 3x4 view matrix at +0, and its projection (a 4x4 at +0x30 when
/// +0x70 has 0x80, else from the field of view at +0xAC and aspect +0xB0).
fn project(sys: &System, p: [f32; 3]) -> Option<(f32, f32)> {
    let cam = r32(sys, WM_CAMERA.va());
    if !in_ram(cam) {
        return None;
    }
    let cam = cam + 0xF8;
    let v = |i: u32| f32_at(sys, cam + 4 * i);
    let e: [f32; 3] = std::array::from_fn(|r| v(4 * r as u32) * p[0] + v(4 * r as u32 + 1) * p[1] + v(4 * r as u32 + 2) * p[2] + v(4 * r as u32 + 3));
    let m: [f32; 16] = if r32(sys, cam + 0x70) & 0x80 != 0 {
        std::array::from_fn(|i| f32_at(sys, cam + 0x30 + 4 * i as u32))
    } else {
        let (fovy, aspect) = (f32_at(sys, cam + 0xAC), f32_at(sys, cam + 0xB0));
        let p5 = 1.0 / (fovy.to_radians() / 2.0).tan();
        let mut m = [0.0; 16];
        (m[0], m[5], m[14]) = (p5 / aspect, p5, -1.0);
        m
    };
    let row = |r: usize| m[4 * r] * e[0] + m[4 * r + 1] * e[1] + m[4 * r + 2] * e[2] + m[4 * r + 3];
    let (cx, cy, w) = (row(0), row(1), row(3));
    if w.abs() <= 1e-5 || !m[0].is_finite() || m[0] == 0.0 {
        return None;
    }
    Some((cx / w * 228.0 * (m[5] / m[0]), cy / w * 228.0))
}

impl Walk {
    /// Every field: how long the player has stood still.
    fn observe(&mut self, sys: &System) {
        self.reads += 1;
        let pos = player_pos(sys);
        let moved = match (pos, self.last_pos) {
            (Some(a), Some(b)) => (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs() >= 0.01,
            _ => true,
        };
        self.still = if moved { 0 } else { self.still + 1 };
        self.last_pos = pos;
    }

    fn step(&mut self, sys: &System) -> Step {
        if self.reads > 3600 || scene(sys) != SCENE_WORLD_MAP {
            return Step::Done;
        }
        if self.still < 12 {
            return Step::Wait;
        }
        let Some(points) = map_points(sys) else { return Step::Done };
        let Some(at) = points.current(sys) else { return Step::Wait };
        if at == self.target {
            return Step::Done;
        }
        if let Some((from, to)) = self.pressed.take() {
            if from == at {
                self.fails += 1;
                if self.fails >= 3 {
                    self.blocked.push((from, to));
                    self.fails = 0;
                }
            } else {
                self.fails = 0;
            }
        }
        let Some(hop) = points.first_hop(at, self.target, &self.blocked) else { return Step::Done };
        let (Some(a), Some(b)) = (project(sys, points.pos[at]), project(sys, points.pos[hop])) else { return Step::Done };
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        self.pressed = Some((at, hop));
        self.still = 0;
        Step::Press(match (dx.abs() >= dy.abs(), dx, dy) {
            (true, dx, _) if dx > 0.0 => SCREEN_RIGHT,
            (true, _, _) => SCREEN_LEFT,
            (false, _, dy) if dy > 0.0 => SCREEN_UP,
            (false, _, _) => SCREEN_DOWN,
        })
    }
}

/// A pane and every pane above it are visible, and it isn't faded out.
fn shown(sys: &System, pane: u32) -> bool {
    if !in_ram(pane) || r8(sys, pane + PANE_GLOBAL_ALPHA) == 0 {
        return false;
    }
    let mut p = pane;
    for _ in 0..16 {
        if !in_ram(p) {
            return true;
        }
        if r8(sys, p + PANE_FLAGS) & 1 == 0 {
            return false;
        }
        p = r32(sys, p + 0xC);
    }
    true
}

/// The character screen (after Select Players): each player's card has an
/// OK bar (`P_InfoBase_00`) with arrows beside it, lit lamps say whose
/// card it is, and `N_back` is Back. A click on player 1's OK bar presses
/// 2, on one of its arrows the D-pad that way, on Back 1; anything else is
/// left to the cursor.
fn character_screen(sys: &mut System, objects: &[(u16, u32)], c: Click, squeeze: f32) -> Option<Job> {
    let x = c.x * squeeze;
    let hit = |b: (f32, f32, f32, f32), margin: f32| (b.0 - margin..=b.2 + margin).contains(&x) && (b.1 - margin..=b.3 + margin).contains(&c.y);
    let mut roots: Vec<u32> = Vec::new();
    for &(_, o) in objects {
        for off in (0..0x1000).step_by(4) {
            let p = r32(sys, o + off);
            if in_ram(p) && !roots.contains(&p) && pane_name(sys, p) == "RootPane" {
                roots.push(p);
            }
        }
    }
    let mut back = None;
    // Each card: its OK bar, right and left arrows, and lamps 08 and 09 lit.
    let mut cards = Vec::new();
    for root in roots {
        let mut pane = |name: &str| {
            let p = sys.find_pane(root, name, 0);
            (p != 0 && shown(sys, p)).then(|| pane_box(sys, p)).flatten()
        };
        if let Some(b) = pane("N_back") {
            back = Some(b);
        }
        if let Some(ok) = pane("P_InfoBase_00") {
            let arrows = [pane("P_arrowR_00"), pane("P_arrowL_01")];
            let lamps = [pane("P_lamp_08").is_some(), pane("P_lamp_09").is_some()];
            cards.push((ok, arrows, lamps));
        }
    }
    if let Some((_, y0, x1, y1)) = back {
        let mid = (y0 + y1) / 2.0;
        if hit((x1 - 150.0 * squeeze, mid - 20.0, x1, mid + 20.0), 8.0) {
            return Some(Job::Once(button::ONE));
        }
    }
    let p1 = if cards.len() == 1 { cards.first() } else { cards.iter().find(|c| c.2[0] && !c.2[1]) }?;
    let (ok, arrows, _) = *p1;
    if hit(ok, 6.0) {
        return Some(Job::Once(button::TWO));
    }
    let reach = (ok.2 - ok.0) / 2.0 + 40.0 * squeeze;
    let ok_x = (ok.0 + ok.2) / 2.0;
    for (arrow, press) in arrows.into_iter().zip([SCREEN_RIGHT, SCREEN_LEFT]) {
        if let Some(a) = arrow {
            if ((a.0 + a.2) / 2.0 - ok_x).abs() <= reach && hit(a, 14.0) {
                return Some(Job::Once(press));
            }
        }
    }
    None
}

/// The save select's buttons (only while it waits for a choice).
fn file_select(sys: &mut System, fs: u32, c: Click) -> Option<Job> {
    let grid = |target| Some(Job::Grid { screen: Grid::FileSelect, target, steps: 0 });
    for n in 0..3 {
        if inside(c.x, c.y, (-190.0 + 190.0 * n as f32, 45.0, 184.0, 196.0)) {
            return grid(n);
        }
    }
    if inside(c.x, c.y, (-143.0, -143.0, 259.0, 92.0)) {
        return grid(3);
    }
    if inside(c.x, c.y, (143.0, -143.0, 259.0, 92.0)) {
        return grid(4);
    }
    if inside(c.x, c.y, (-265.0, -62.0, 190.0, 46.0)) {
        return Some(Job::Once(button::MINUS)); // Erase
    }
    if inside(c.x, c.y, (265.0, -62.0, 190.0, 46.0)) {
        return Some(Job::Once(button::PLUS)); // Copy
    }
    // The port's Settings (5) and Multiplayer (6) buttons in the title bar.
    let layout = fs + FILE_SELECT_LAYOUT;
    let root = sys.root_pane(layout);
    for (target, name) in [(5, hle_os::menu_layout::FILE_COG_PANE), (6, hle_os::menu_layout::FILE_MULTI_PANE)] {
        let pane = if root == 0 { 0 } else { sys.find_pane(root, name, 0) };
        if let Some((x0, y0, x1, y1)) = pane_box(sys, pane) {
            let x = c.x * pane_squeeze(sys, pane);
            if (x0 - 4.0..=x1 + 4.0).contains(&x) && (y0 - 4.0..=y1 + 4.0).contains(&c.y) {
                return grid(target);
            }
        }
    }
    None
}

fn select_players(c: Click) -> Option<Job> {
    let grid = |target| Some(Job::Grid { screen: Grid::SelectPlayers, target, steps: 0 });
    if inside(c.x, c.y, (0.0, 40.0, 410.0, 122.0)) {
        return grid(0);
    }
    for n in 2..=4u32 {
        if inside(c.x, c.y, (-120.0 + 120.0 * (n - 2) as f32, -81.0, 112.0, 104.0)) {
            return grid(n - 1);
        }
    }
    if inside(c.x, c.y, (320.0, -183.0, 260.0, 70.0)) {
        return Some(Job::Once(button::ONE)); // Back
    }
    None
}

// ---------------------------------------------------------------- a step of a job

fn step(sys: &mut System, job: &mut Job) -> Step {
    match job {
        Job::Once(b) => {
            let b = *b;
            *job = Job::Once(0);
            if b == 0 {
                Step::Done
            } else {
                Step::Press(b)
            }
        }
        Job::Cursor(s) => steer(sys, s),
        Job::Walk(w) => w.step(sys),
        Job::World { target, presses, steps } => {
            *steps += 1;
            let Some(ws) = find(sys, PROFILE_WORLD_SELECT).filter(|_| *steps <= 150) else { return Step::Done };
            if state_name(sys, ws + WORLD_SELECT_STATE) != "dWorldSelect_c::StateID_Select" {
                return Step::Wait;
            }
            let world = r32(sys, ws + WORLD_SELECT_WORLD);
            if world == *target {
                return Step::Done;
            }
            *presses += 1;
            if *presses > 8 {
                return Step::Done;
            }
            Step::Press(if (*target + 8 - world % 8) % 8 <= 4 { SCREEN_RIGHT } else { SCREEN_LEFT })
        }
        Job::Grid { screen, target, steps } => {
            *steps += 1;
            if *steps > 16 {
                return Step::Done;
            }
            let s = grid_step(sys, *screen, *target);
            if matches!(s, Step::Press(button::TWO)) {
                *steps = u32::MAX - 1; // done after this press
            }
            s
        }
    }
}

/// Toward the click with the corner-bracket cursor, then 2.
fn steer(sys: &mut System, s: &mut Steer) -> Step {
    s.steps = s.steps.saturating_add(1);
    if s.steps > 24 {
        return Step::Done;
    }
    let Some((x0, y0, x1, y1)) = cursor_box(sys) else {
        // Not up yet (a menu opening): wait a little.
        return if s.steps <= 3 { Step::Wait } else { Step::Done };
    };
    let centre = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    if let Some(last) = s.last {
        if (centre.0 - last.0).abs() > 0.5 || (centre.1 - last.1).abs() > 0.5 {
            // Still moving: wait for it to settle.
            s.last = Some(centre);
            s.stuck = 0;
            s.pressed = None;
            return Step::Wait;
        }
    }
    s.last = Some(centre);
    if let Some(vertical) = s.pressed.take() {
        // The last press didn't move it.
        s.stuck += 1;
        s.blocked = Some(vertical);
    }
    if (x0..=x1).contains(&s.x) && (y0..=y1).contains(&s.y) {
        s.steps = u32::MAX - 1; // done after this press
        return Step::Press(button::TWO);
    }
    if s.stuck >= 2 {
        return Step::Done;
    }
    let (w, h) = ((x1 - x0).max(1.0), (y1 - y0).max(1.0));
    let (dx, dy) = ((s.x - centre.0) / w, (s.y - centre.1) / h);
    let mut vertical = dy.abs() >= dx.abs();
    if s.blocked == Some(vertical) {
        vertical = !vertical;
    }
    s.pressed = Some(vertical);
    Step::Press(match (vertical, dx, dy) {
        (true, _, dy) if dy > 0.0 => SCREEN_UP,
        (true, _, _) => SCREEN_DOWN,
        (false, dx, _) if dx > 0.0 => SCREEN_RIGHT,
        (false, _, _) => SCREEN_LEFT,
    })
}

/// A screen whose cursor is a number: one press at a time toward `target`
/// (row first), then 2.
fn grid_step(sys: &mut System, screen: Grid, target: u32) -> Step {
    let (settled, cursor) = match screen {
        Grid::FileSelect => {
            let Some(fs) = find(sys, PROFILE_FILE_SELECT) else { return Step::Done };
            (r32(sys, fs + FILE_SELECT_STATE) == FILE_SELECT_WAIT.va(), r32(sys, fs + FILE_SELECT_CURSOR))
        }
        Grid::SelectPlayers => {
            let Some(sp) = find(sys, PROFILE_SELECT_PLAYERS) else { return Step::Done };
            (r32(sys, sp + SELECT_PLAYERS_STATE) == SELECT_PLAYERS_SELECT.va(), r32(sys, sp + SELECT_PLAYERS_CURSOR))
        }
    };
    if !settled {
        return Step::Wait;
    }
    if cursor == target {
        return Step::Press(button::TWO);
    }
    // (row, column) of a cursor value.
    let place = |v: u32| -> (i32, i32) {
        match screen {
            Grid::FileSelect => match v {
                0..=2 => (1, v as i32),
                3 => (2, 0),
                4 => (2, 2),
                5 => (0, 2), // Settings and Multiplayer, above the files
                _ => (0, 1),
            },
            Grid::SelectPlayers => match v {
                0 => (0, 1),
                n => (1, n as i32 - 1),
            },
        }
    };
    let (from, to) = (place(cursor), place(target));
    Step::Press(if from.0 != to.0 {
        if to.0 < from.0 {
            SCREEN_UP
        } else {
            SCREEN_DOWN
        }
    } else if to.1 > from.1 {
        SCREEN_RIGHT
    } else {
        SCREEN_LEFT
    })
}
