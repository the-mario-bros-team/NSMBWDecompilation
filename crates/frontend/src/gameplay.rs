//! The Settings window's Gameplay options, applied on the game thread after
//! each field by fixing up the game's own state rather than replacing its
//! functions (after WitherBlitz's NSMB-Wii-Recompilation,
//! `runtime/src/hle/gameplay_options.cpp`; USA revision 1 addresses):
//!
//! - No Time Limit: `dStageTimer_c` counts its time (+4, seconds << 12) down
//!   by 92 a frame while it runs; each such step is given back, so the timer
//!   stands still. Bigger steps (the goal counting the time off into the
//!   score) are left alone. The HUD's timer is hidden too.
//! - Infinite Lives: `daPyMng_c::mRest` (one count a character) losing one
//!   life in a course is undone.
//! - Power-Up: given to player 1 between fields: in a course through
//!   `dAcPy_c::ChangePowerupWithAnimation` (or its `setStar`, as the Star
//!   item does), on the world map through `daPyMng_c::mPlayerMode` and the
//!   map figure's own change function, as the map's Items screen does.
//!
//! - "Cheated": Infinite Lives giving a life back, or the Power-Up being
//!   given, marks the save file being played ([`Cheats`]; the Settings
//!   window asks first and marks it at once for Infinite Lives). The save
//!   select then shows "Cheated" on that file's card: a text the port built
//!   into the card's layout (`hle_os::menu_layout`), shown here. An erased
//!   file loses its mark.
//!
//! Net games keep the game's own rules: every machine has to run the same.

use std::sync::atomic::{AtomicI8, AtomicU8, Ordering::Relaxed};

use hle_os::System;

const SCENE: u32 = 0x8042_8730; // dScene_c::m_nextScene, m_nowScene (halfwords)
const STAGE_TIMER: u32 = 0x8042_A070; // dStageTimer_c::m_instance
const PLAYER_TYPE: u32 = 0x8035_4E60; // daPyMng_c::mPlayerType[4]
const PLAYER_MODE: u32 = 0x8035_4E70; // daPyMng_c::mPlayerMode[4], each character's power-up
const CREATE_ITEM: u32 = 0x8035_4E80; // daPyMng_c::mCreateItem[4]; bit 0: a Star
const REST: u32 = 0x8035_4E90; // daPyMng_c::mRest[4]
const MAP_MODELS: u32 = 0x8093_CDF0; // the world map player's model of each slot
const WM_FIGURES: u32 = 0x809A_2390; // the world map's players
const WM_FIGURES_BY_CHARACTER: u32 = 1304; // their figures, one a character
const WM_FIGURE_MODELS: u32 = 296; // a figure's dPyMdlMng_c
const WM_FIGURE_CHANGE: u32 = 0x808C_6DF0; // a figure's power-up and model (the Items screen's call)
const GET_PLAYER: u32 = 0x8005_F900; // daPyMng_c::getPlayer(int)
const CHANGE_POWER_UP: u32 = 0x8014_5AC0; // dAcPy_c::ChangePowerupWithAnimation
const PLAYER_VTABLE: u32 = 0x60;
const PLAYER_SET_STAR: u32 = 0x3D8; // dAcPy_c's setStar(kind, frames) in its vtable
const STAR_FRAMES: u32 = 0x294; // what the Star item gives
const GAME_DISPLAY: u32 = 0x8042_A328; // dGameDisplay_c::m_instance
const GAME_DISPLAY_TIME: u32 = 0x520; // its N_time_00 pane
const PANE_FLAGS: u32 = 0xBB; // nw4r::lyt::Pane's flags; bit 0: visible
const TREE_ROOT: u32 = 0x8037_7A10; // fManager_c's tree of the game's objects
const PROFILE_OPENING_TITLE: u16 = 0x2BB; // only on the title screen (its course is a stage too)
const SCENE_WORLD_MAP: u16 = 3;
const SCENE_STAGE: u16 = 5;
const SCENE_GAME_SETUP: u16 = 0xA; // the save select and Select Players
const PROFILE_FILE_SELECT: u16 = 0x2D5; // dFileSelect_c
const FILE_SELECT_STATE: u32 = 0x238; // its current state
const FILE_SELECT_WAIT: u32 = 0x8099_2050; // dFileSelect_c::StateID_SelectWait
const FILE_SELECT_CARDS: u32 = 0x24C; // its three files' cards
const FILE_SELECT_CURSOR: u32 = 0x2A4; // 0-2: on a file
const SAVE_MNG: u32 = 0x8042_A040; // dSaveMng_c::m_instance
const SAVE_GAMES: u32 = 0x6C0; // its three games; a game's +2 bit 0: new
const SAVE_GAME_SIZE: u32 = 0x980;
const TIMER_STEP: i32 = 92;
const TIMER_SLACK: i32 = 4 * TIMER_STEP;

/// Power-ups in the list: the game's own numbers (0 small .. 6 ice), then the Star.
pub const POWER_UPS: [&str; 8] = ["Small", "Super Mushroom", "Fire Flower", "Mini Mushroom", "Propeller Mushroom", "Penguin Suit", "Ice Flower", "Star"];
const STAR: u8 = 7;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Options {
    pub no_time_limit: bool,
    pub infinite_lives: bool,
}

/// The save files' "Cheated" marks, shared by the game thread (which marks
/// a file when an option does something) and the window's thread (which
/// asks first, and keeps the marks in `settings.toml`).
pub struct Cheats {
    /// Bit n: save file n (0-2) is marked.
    pub marks: AtomicU8,
    /// The save file being played: the one the save select last had its
    /// cursor on; -1 before the save select.
    pub active: AtomicI8,
}

impl Default for Cheats {
    fn default() -> Self {
        Self::new([false; 3])
    }
}

impl Cheats {
    pub fn new(marked: [bool; 3]) -> Self {
        let marks = marked.iter().enumerate().fold(0, |m, (i, &on)| m | u8::from(on) << i);
        Self { marks: AtomicU8::new(marks), active: AtomicI8::new(-1) }
    }

    /// The marks, a file each.
    pub fn marked(&self) -> [bool; 3] {
        let m = self.marks.load(Relaxed);
        [m & 1 != 0, m & 2 != 0, m & 4 != 0]
    }

    /// Marks the save file being played, if one is.
    pub fn mark_active(&self) {
        if let Ok(file @ 0..=2) = u8::try_from(self.active.load(Relaxed)) {
            self.marks.fetch_or(1 << file, Relaxed);
        }
    }
}

/// What the options remember between fields.
#[derive(Default)]
pub struct Gameplay {
    timer: u32,
    last_time: i32,
    hidden_timer: u32,
    rest: Option<[i32; 4]>,
    /// A power-up for player 1, given once the game is in a course or on the map.
    pub pending: Option<u8>,
    /// The save select (the object and its unique number) and its three
    /// cards' "Cheated" panes, once found.
    cards: (u32, u32, [u32; 3]),
}

fn in_ram(a: u32) -> bool {
    (0x8000_0000..0x8180_0000).contains(&a)
}

fn read32(sys: &System, a: u32) -> Option<u32> {
    sys.mem.ram(a, 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

fn write32(sys: &mut System, a: u32, v: u32) {
    if let Some(b) = sys.mem.ram_mut(a, 4) {
        b.copy_from_slice(&v.to_be_bytes());
    }
}

/// A pointer read from guest RAM, if it points into it.
fn pointer(sys: &System, a: u32) -> Option<u32> {
    read32(sys, a).filter(|&p| in_ram(p))
}

impl Gameplay {
    /// After a field. Returns a line to log when a power-up was given.
    pub fn after_field(&mut self, sys: &mut System, opts: Options, cheats: &Cheats) -> Option<String> {
        let scene = read32(sys, SCENE).map_or(0, |w| w as u16);
        self.hold_timer(sys, scene, opts.no_time_limit);
        self.hide_timer(sys, scene, opts.no_time_limit);
        if self.hold_lives(sys, scene, opts.infinite_lives) {
            cheats.mark_active();
        }
        self.save_select(sys, scene, cheats);
        let power_up = self.pending?;
        let given = match scene {
            SCENE_STAGE => give_in_course(sys, power_up),
            SCENE_WORLD_MAP => give_on_map(sys, power_up),
            _ => None,
        };
        if given.is_some() {
            self.pending = None;
            cheats.mark_active();
        }
        given
    }

    /// The save select: the file its cursor is on is the one being played;
    /// an erased file loses its mark; each card's "Cheated" shows on a
    /// marked file (if the cards have the text).
    fn save_select(&mut self, sys: &mut System, scene: u16, cheats: &Cheats) {
        let screen = if scene == SCENE_GAME_SETUP { find_profile(sys, PROFILE_FILE_SELECT) } else { None };
        let Some(screen) = screen else {
            self.cards = Default::default();
            return;
        };
        // Only while it waits for a pick: its files are loaded by then.
        let waiting = read32(sys, screen + FILE_SELECT_STATE) == Some(FILE_SELECT_WAIT);
        if waiting {
            if let Some(file) = read32(sys, screen + FILE_SELECT_CURSOR).filter(|&c| c < 3) {
                cheats.active.store(file as i8, Relaxed);
            }
        }
        let id = read32(sys, screen).unwrap_or(0);
        if (self.cards.0, self.cards.1) != (screen, id) {
            self.cards = (screen, id, [0; 3]);
        }
        let games = pointer(sys, SAVE_MNG);
        for i in 0..3u32 {
            let new = games.and_then(|g| sys.mem.ram(g + SAVE_GAMES + SAVE_GAME_SIZE * i + 2, 1)).is_some_and(|b| b[0] & 1 != 0);
            if waiting && new {
                cheats.marks.fetch_and(!(1 << i), Relaxed);
            }
            if !sys.menu.cheated_label {
                continue;
            }
            let pane = &mut self.cards.2[i as usize];
            if *pane == 0 {
                // Not there until the card has built its layout.
                if let Some(card) = pointer(sys, screen + FILE_SELECT_CARDS + 4 * i) {
                    let root = sys.root_pane(card);
                    *pane = sys.find_pane(root, hle_os::menu_layout::CHEATED_PANE, 0);
                }
            }
            let (pane, marked) = (*pane, cheats.marks.load(Relaxed) >> i & 1 != 0);
            if let Some(flags) = Some(pane).filter(|&p| p != 0).and_then(|p| sys.mem.ram_mut(p + PANE_FLAGS, 1)) {
                flags[0] = if marked && !new { flags[0] | 1 } else { flags[0] & !1 };
            }
        }
    }

    fn hold_timer(&mut self, sys: &mut System, scene: u16, on: bool) {
        let timer = if on && scene == SCENE_STAGE { pointer(sys, STAGE_TIMER) } else { None };
        let (Some(timer), Some(time), Some(stopped)) = (timer, timer.and_then(|t| read32(sys, t + 4)), timer.and_then(|t| read32(sys, t + 12))) else {
            self.timer = 0;
            return;
        };
        let now = time as i32;
        if timer == self.timer && stopped >> 24 == 0 && now < self.last_time && self.last_time - now <= TIMER_SLACK {
            write32(sys, timer + 4, self.last_time as u32);
            return;
        }
        self.timer = timer;
        self.last_time = now;
    }

    /// No Time Limit has no time to show. Shown again if the option is turned
    /// off in the course; a new course builds its HUD anew.
    fn hide_timer(&mut self, sys: &mut System, scene: u16, on: bool) {
        let pane = if scene == SCENE_STAGE { pointer(sys, GAME_DISPLAY).and_then(|d| pointer(sys, d + GAME_DISPLAY_TIME)) } else { None };
        let Some(pane) = pane else {
            self.hidden_timer = 0;
            return;
        };
        let Some(flags) = sys.mem.ram_mut(pane + PANE_FLAGS, 1) else { return };
        if on {
            flags[0] &= !1;
            self.hidden_timer = pane;
        } else if self.hidden_timer == pane {
            flags[0] |= 1;
            self.hidden_timer = 0;
        }
    }

    /// True if a life was given back.
    fn hold_lives(&mut self, sys: &mut System, scene: u16, on: bool) -> bool {
        if scene != SCENE_STAGE || !on {
            self.rest = None;
            return false;
        }
        let mut now = [0i32; 4];
        for (i, n) in now.iter_mut().enumerate() {
            let Some(v) = read32(sys, REST + 4 * i as u32) else {
                self.rest = None;
                return false;
            };
            *n = v as i32;
        }
        let mut given = false;
        if let Some(last) = self.rest {
            for i in 0..4 {
                if now[i] == last[i] - 1 {
                    write32(sys, REST + 4 * i as u32, last[i] as u32);
                    now[i] = last[i];
                    given = true;
                }
            }
        }
        self.rest = Some(now);
        given
    }
}

/// The first object of `profile` in the game's object tree (walked depth
/// first: +4 child, +0xC next, +0 parent; +0x10 the object, whose profile is
/// the high half of its +8 and whose unique number is its +0).
fn find_profile(sys: &System, profile: u16) -> Option<u32> {
    let mut node = read32(sys, TREE_ROOT).unwrap_or(0);
    for _ in 0..4096 {
        if node == 0 {
            break;
        }
        let object = read32(sys, node + 0x10).filter(|&o| o != 0);
        if object.and_then(|o| read32(sys, o + 8)).is_some_and(|w| (w >> 16) as u16 == profile) {
            return object;
        }
        match read32(sys, node + 4) {
            Some(child) if child != 0 => node = child,
            _ => loop {
                if node == 0 {
                    break;
                }
                match read32(sys, node + 0xC) {
                    Some(next) if next != 0 => {
                        node = next;
                        break;
                    }
                    _ => node = read32(sys, node).unwrap_or(0),
                }
            },
        }
    }
    None
}

fn has_profile(sys: &System, profile: u16) -> bool {
    find_profile(sys, profile).is_some()
}

/// In a course: once player 1 is in it (not the title screen's).
fn give_in_course(sys: &mut System, power_up: u8) -> Option<String> {
    if has_profile(sys, PROFILE_OPENING_TITLE) {
        return None;
    }
    let player = sys.call_guest(GET_PLAYER, &[0], &[]).ok().filter(|&p| in_ram(p))?;
    if power_up == STAR {
        let set_star = pointer(sys, player + PLAYER_VTABLE).and_then(|v| pointer(sys, v + PLAYER_SET_STAR))?;
        sys.call_guest(set_star, &[player, 0, STAR_FRAMES], &[]).ok()?;
    } else {
        sys.call_guest(CHANGE_POWER_UP, &[player, u32::from(power_up)], &[]).ok()?;
    }
    Some(format!("gameplay: player 1 gets {} in the course", POWER_UPS[power_up as usize]))
}

/// On the world map, as its Items screen does with a power-up item.
fn give_on_map(sys: &mut System, power_up: u8) -> Option<String> {
    let character = read32(sys, PLAYER_TYPE).filter(|&t| t <= 3)?;
    if power_up == STAR {
        // The map's own Star item: the next course starts with one.
        let items = read32(sys, CREATE_ITEM + 4 * character).unwrap_or(0);
        write32(sys, CREATE_ITEM + 4 * character, items | 1);
        return Some("gameplay: player 1 gets a Star on the world map (for the next course)".into());
    }
    write32(sys, PLAYER_MODE + 4 * character, u32::from(power_up));
    // Going into a course takes each figure's power-up back into
    // mPlayerMode, so the figure has to have it too (the game's own map only).
    let ours = (0..4u32).all(|i| read32(sys, MAP_MODELS + 4 * i) == Some([0, 1, 3, 2][i as usize]));
    let figure = pointer(sys, WM_FIGURES).and_then(|m| pointer(sys, m + WM_FIGURES_BY_CHARACTER + 4 * character)).filter(|&f| pointer(sys, f + WM_FIGURE_MODELS).is_some());
    if let (true, Some(figure)) = (ours, figure) {
        let _ = sys.call_guest(WM_FIGURE_CHANGE, &[figure, u32::from(power_up)], &[]);
    }
    Some(format!("gameplay: player 1 gets {} on the world map", POWER_UPS[power_up as usize]))
}
