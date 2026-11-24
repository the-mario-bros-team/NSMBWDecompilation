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
//! - Autosave (after `runtime/src/host_suspend.cpp`'s `AutoSaveFrame`): back
//!   on the world map from a course left by its goal, the game is saved as
//!   the map's own Save does (`SaveGame`), with no prompt, so closing the
//!   game after a course loses nothing. The map records the clear in its
//!   first moments: the save is made once the file's course flags change,
//!   or after 5 s on the map (a replay that records nothing new still saves
//!   its lives and items; a clear the map recorded before this first saw it
//!   too), once the map's menu is closed and the NAND thread idle; it gives
//!   up after 15 s. Not the title screen's demo, Free Mode or Coin Battle.
//! - Exit Course is the hook in `hle_os::menu_settings`; this only sets it.
//! - Player 1's character pick (`hle_os::p1_character`, applied at boot):
//!   each save file remembers the character player 1 last played on its
//!   world map ([`Characters`]), and the character screen's cards start on
//!   it (the game's table of each slot's first card, the saved character's
//!   swapped into player 1's); the screen's own tidying is hle-os's.
//!
//! - Power-up Freeze off (after ral1009's NSMBWiicompiled,
//!   `nsmbw_power_up_freeze.cpp` and `nsmbw_death_freeze.cpp`): the level
//!   keeps moving while a player grows, shrinks or changes power-up, and
//!   while a player dies. `dAcPy_c::executeChangeInit` and
//!   `executeChangeNormal` stop every other actor (`daPlBase_c::stopOther`,
//!   `dAudio::pauseMove`) and return `isPlayerGameStop()`; the death demo's
//!   `stopOtherDownDemo` calls `stopOther` too. Off, those calls are
//!   nops and the returns 0: the game's own no-freeze path.
//! - Player Collision off: players pass through each other.
//!   `dAcPy_c::ccCallBack`, for another player's body, goes on to riding
//!   (standing on a head), stomping and the side push; off, it returns
//!   there instead. Attacks (a ground pound, picking a player up) still
//!   reach players.
//!   Both are code patches ([`PATCHES`]): each word is written only where
//!   the game has USA revision 1's (or the patch's) word, so another
//!   version or a mod is left alone, and a changed function runs
//!   interpreted (`guest_cpu::recomp`) while it differs.
//! - Skipping the opening cutscene and the hint movies ([`MovieSkip`],
//!   after NSMBWiicompiled's `nsmbw_movie_skip.cpp`, which takes -): any
//!   button newly pressed on any remote, or a click or tap on the picture,
//!   ends the opening as its own last state does, or leaves a hint movie
//!   for the world map as its pause menu's exit does.
//!
//! Net games keep the game's own rules: every machine has to run the same
//! (Exit Course only if every player has it: `net`).

use std::sync::atomic::{AtomicI8, AtomicU8, Ordering::Relaxed};

use game_version::E1;
use hle_os::System;

const SCENE: E1 = E1(0x8042_8730); // dScene_c::m_nextScene, m_nowScene (halfwords)
const STAGE_TIMER: E1 = E1(0x8042_A070); // dStageTimer_c::m_instance
const PLAYER_TYPE: E1 = E1(0x8035_4E60); // daPyMng_c::mPlayerType[4]
const PLAYER_MODE: E1 = E1(0x8035_4E70); // daPyMng_c::mPlayerMode[4], each character's power-up
const CREATE_ITEM: E1 = E1(0x8035_4E80); // daPyMng_c::mCreateItem[4]; bit 0: a Star
const REST: E1 = E1(0x8035_4E90); // daPyMng_c::mRest[4]
const MAP_MODELS: E1 = E1(0x8093_CDF0); // the world map player's model of each slot
const WM_FIGURES: E1 = E1(0x809A_2390); // the world map's players
const WM_FIGURES_BY_CHARACTER: u32 = 1304; // their figures, one a character
const WM_FIGURE_MODELS: u32 = 296; // a figure's dPyMdlMng_c
const WM_FIGURE_CHANGE: E1 = E1(0x808C_6DF0); // a figure's power-up and model (the Items screen's call)
const GET_PLAYER: E1 = E1(0x8005_F900); // daPyMng_c::getPlayer(int)
const CHANGE_POWER_UP: E1 = E1(0x8014_5AC0); // dAcPy_c::ChangePowerupWithAnimation
const PLAYER_VTABLE: u32 = 0x60;
const PLAYER_SET_STAR: u32 = 0x3D8; // dAcPy_c's setStar(kind, frames) in its vtable
const STAR_FRAMES: u32 = 0x294; // what the Star item gives
const GAME_DISPLAY: E1 = E1(0x8042_A328); // dGameDisplay_c::m_instance
const GAME_DISPLAY_TIME: u32 = 0x520; // its N_time_00 pane
const PANE_FLAGS: u32 = 0xBB; // nw4r::lyt::Pane's flags; bit 0: visible
const TREE_ROOT: E1 = E1(0x8037_7A10); // fManager_c's tree of the game's objects
const PROFILE_OPENING_TITLE: u16 = 0x2BB; // only on the title screen (its course is a stage too)
const SCENE_WORLD_MAP: u16 = 3;
const SCENE_STAGE: u16 = 5;
const SCENE_GAME_SETUP: u16 = 0xA; // the save select and Select Players
const PROFILE_FILE_SELECT: u16 = 0x2D5; // dFileSelect_c
const FILE_SELECT_STATE: u32 = 0x238; // its current state
const FILE_SELECT_WAIT: E1 = E1(0x8099_2050); // dFileSelect_c::StateID_SelectWait
const FILE_SELECT_CARDS: u32 = 0x24C; // its three files' cards
const FILE_SELECT_CURSOR: u32 = 0x2A4; // 0-2: on a file
const SAVE_MNG: E1 = E1(0x8042_A040); // dSaveMng_c::m_instance
const SAVE_GAMES: u32 = 0x6C0; // its three games; a game's +2 bit 0: new
const SAVE_GAME_SIZE: u32 = 0x980;
const SAVE_SLOT: u32 = 0x26; // the open file (a byte)
const COURSE_FLAGS: u32 = 0x6C; // a game's goals and Star Coins: 10 worlds of 42 courses
const COURSE_FLAG_WORDS: usize = 420;
const EXIT_MODE: E1 = E1(0x8042_900C); // dScStage_c::m_exitMode: 0 the goal, 1 all lost, 2 quit
const GAME_FLAG: E1 = E1(0x8042_9F80); // dInfo_c::mGameFlag
const FREE_MODE_OR_COIN_BATTLE: u32 = 1 << 5 | 1 << 6;
const NAND_THREAD: E1 = E1(0x8042_9FB8); // dNandThread_c::m_instance; +0x74 a command in progress
const NAND_BUSY: u32 = 0x74;
const SAVE_GAME: E1 = E1(0x8092_F5F0); // SaveGame(_, quick) in d_basesNP, the map's Save
const SAVE_GAME_FIRST: u32 = 0x9421_FFF0; // its first word (stwu r1, -0x10(r1))
const PROFILE_MAP_MENU: u16 = 0x2B5; // dCourseSelectMenu_c, the world map's + menu
const MAP_MENU_STATE: u32 = 0x234;
const MAP_MENU_CLOSED: E1 = E1(0x8099_15F0); // dCourseSelectMenu_c::StateID_InitWait
/// Fields on the world map before a replay that recorded nothing new is
/// saved anyway, and before the autosave gives up.
const AUTOSAVE_REPLAY: u32 = 5 * 60;
const AUTOSAVE_WATCH: u32 = 15 * 60;
const PROFILE_CHARACTER_SCREEN: u16 = 0x2B6;
/// The setup screens' first card for each player slot (d_basesNP data), in
/// the cards' order (0 Mario, 1 Blue Toad, 2 Yellow Toad, 3 Luigi): the
/// game's table, and each character's card.
const CARD_STARTS: E1 = E1(0x8093_3D20);
const GAME_CARD_STARTS: [u32; 4] = [0, 3, 2, 1];
const CARD_OF_CHARACTER: [u32; 4] = [0, 3, 1, 2];
const TIMER_STEP: i32 = 92;
const TIMER_SLACK: i32 = 4 * TIMER_STEP;

/// Power-ups in the list: the game's own numbers (0 small .. 6 ice), then the Star.
pub const POWER_UPS: [&str; 8] = ["Small", "Super Mushroom", "Fire Flower", "Mini Mushroom", "Propeller Mushroom", "Penguin Suit", "Ice Flower", "Star"];
const STAR: u8 = 7;

/// The Gameplay options; all off (the default) is the game's own rules.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Options {
    pub no_time_limit: bool,
    pub infinite_lives: bool,
    pub autosave: bool,
    pub exit_course: bool,
    /// Applied at boot only, and not in scripted, headless or net runs.
    pub p1_character: bool,
    /// Power-up Freeze off: the level keeps moving while a player changes
    /// size or dies.
    pub no_power_up_freeze: bool,
    /// Player Collision off: players pass through each other.
    pub no_player_collision: bool,
}

/// A code patch: (address, USA revision 1's word, the patch's word).
type Patch = (E1, u32, u32);
const NOP: u32 = 0x6000_0000;
const LI_R3_0: u32 = 0x3860_0000;
/// Power-up Freeze off.
const NO_FREEZE: [Patch; 5] = [
    (E1(0x8014_0F84), 0x4BF0_D0CD, NOP),     // executeChangeInit: bl stopOther
    (E1(0x8014_0F90), 0x4BF2_9841, NOP),     // executeChangeInit: bl dAudio::pauseMove
    (E1(0x8014_0FD0), 0x4BF0_D071, LI_R3_0), // executeChangeInit: bl isPlayerGameStop
    (E1(0x8014_1438), 0x4BF0_CC09, LI_R3_0), // executeChangeNormal: bl isPlayerGameStop
    (E1(0x8013_D930), 0x4BF1_0721, NOP),     // stopOtherDownDemo: bl stopOther
];
/// Player Collision off: ccCallBack's `mr r3, r30` before checkRideActor,
/// reached for another player's body, becomes a branch to its return.
const NO_COLLISION: [Patch; 1] = [(E1(0x8014_1D58), 0x7FC3_F378, 0x4800_00B8)];

/// Sets each patch's word to the patched one (`on`) or the game's own, if it
/// holds one of the two.
fn patch(sys: &mut System, patches: &[Patch], on: bool) {
    for &(at, original, patched) in patches {
        let (Some(addr), original, patched) = (at.try_va(), branch_at(at, original), branch_at(at, patched)) else { continue };
        let (now, want) = (read32(sys, addr), if on { patched } else { original });
        if now != Some(want) && (now == Some(original) || now == Some(patched)) {
            let _ = sys.mem.write_u32(addr, want);
        }
    }
}

/// `word`, an instruction at USA address `at`, for the version that runs: a
/// relative branch (`b`, `bl`) is re-aimed at its target's address there.
fn branch_at(at: E1, word: u32) -> u32 {
    if word >> 26 != 18 || word & 2 != 0 {
        return word;
    }
    let offset = ((word & 0x03FF_FFFC) as i32) << 6 >> 6;
    match (at.try_va(), E1(at.0.wrapping_add(offset as u32)).try_va()) {
        (Some(from), Some(to)) => (word & 0xFC00_0003) | (to.wrapping_sub(from) & 0x03FF_FFFC),
        _ => word,
    }
}

const GAME_STOP: E1 = E1(0x8042_9F48); // nonzero while the game is stopped (its pause menu, a warning)
const SCENE_MOVIE: u16 = 8;
const MOVIE: E1 = E1(0x8042_A1B8); // dScMovie_c::m_instance
const MOVIE_STATE: u32 = 0x164; // 1: playing
const MOVIE_ID: u32 = 0x198; // 0: the opening
const MOVIE_END: u32 = 0x19C;
const MOVIE_STATE_END: u32 = 10;
const GAME_MODE: E1 = E1(0x8031_5B98); // dInfo_c::m_startGameInfo's game mode; 4: a hint movie
const GAME_MODE_HINT_MOVIE: u32 = 4;
const REPLAY_PLAY: E1 = E1(0x8037_41D0); // dScStage_c's replay players; player 1's once the movie runs
const SET_NEXT_SCENE: E1 = E1(0x8010_2260); // dScStage_c::setNextScene(scene, arg, exit, fader)
const SET_NEXT_SCENE_FIRST: u32 = 0x9421_FFE0;

/// What can be skipped now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Skippable {
    Opening,
    HintMovie,
}

/// Skipping the opening cutscene and the hint movies: see the module's notes.
#[derive(Default)]
pub struct MovieSkip {
    /// The remotes' buttons at the last field.
    held: [u16; 4],
    /// The scene skipped (so a press during the fade out does not skip again).
    skipped: Option<u16>,
}

impl MovieSkip {
    /// Before a field, with the buttons the remotes give the game in it and
    /// whether the picture was clicked or tapped since the last field.
    /// Returns a line to log once it skips. While a movie can be skipped,
    /// the game sees no buttons (+ would open a hint movie's pause menu
    /// as it ends).
    pub fn before_field(&mut self, sys: &mut System, buttons: &mut [u16; 4], tapped: bool) -> Option<String> {
        let pressed = buttons.iter().zip(self.held).any(|(&b, h)| b & !h != 0);
        self.held = *buttons;
        let scene = read32(sys, SCENE.va()).map_or(0, |w| w as u16);
        if self.skipped.is_some_and(|s| s != scene) {
            self.skipped = None;
        }
        let what = skippable(sys, scene)?;
        *buttons = [0; 4];
        if !(pressed || tapped) || self.skipped.is_some() {
            return None;
        }
        match what {
            Skippable::Opening => {
                let movie = pointer(sys, MOVIE.va())?;
                let _ = sys.mem.write_u8(movie + MOVIE_END, 1);
                write32(sys, movie + MOVIE_STATE, MOVIE_STATE_END);
            }
            Skippable::HintMovie => {
                // Back to the world map (3) as the hint movie's pause
                // menu leaves it: exit kind 2, fader 5.
                if read32(sys, SET_NEXT_SCENE.va()) != Some(SET_NEXT_SCENE_FIRST) {
                    return None;
                }
                sys.call_guest(SET_NEXT_SCENE.va(), &[3, 0, 2, 5], &[]).ok()?;
            }
        }
        self.skipped = Some(scene);
        Some(format!("skipped the {}", if what == Skippable::Opening { "opening cutscene" } else { "hint movie" }))
    }
}

fn skippable(sys: &System, scene: u16) -> Option<Skippable> {
    if read32(sys, GAME_STOP.va()) != Some(0) {
        return None;
    }
    match scene {
        SCENE_MOVIE => {
            let movie = pointer(sys, MOVIE.va())?;
            (read32(sys, movie + MOVIE_STATE) == Some(1) && read32(sys, movie + MOVIE_ID) == Some(0)).then_some(Skippable::Opening)
        }
        SCENE_STAGE => (read32(sys, GAME_MODE.va()) == Some(GAME_MODE_HINT_MOVIE) && pointer(sys, REPLAY_PLAY.va()).is_some()).then_some(Skippable::HintMovie),
        _ => None,
    }
}

/// Player 1's last character on each save file (0-3, as
/// `daPyMng_c::mPlayerType`), shared with the window's thread, which keeps
/// them in `settings.toml`.
#[derive(Default)]
pub struct Characters(pub [AtomicU8; 3]);

impl Characters {
    pub fn new(saved: [u32; 3]) -> Self {
        Self(saved.map(|c| AtomicU8::new(c.min(3) as u8)))
    }

    pub fn get(&self) -> [u32; 3] {
        std::array::from_fn(|i| u32::from(self.0[i].load(Relaxed)))
    }
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
    /// The scene at the last field.
    last_scene: u16,
    /// Back on the world map after a course's goal: the autosave's watch.
    watch: Option<Watch>,
}

struct Watch {
    /// Fields on the map so far.
    fields: u32,
    /// The course flags as found on arrival, or as last saved.
    flags: u64,
    saved: bool,
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
    /// After a field. Returns lines to log: a power-up given, the autosave.
    pub fn after_field(&mut self, sys: &mut System, opts: Options, cheats: &Cheats, characters: &Characters) -> Vec<String> {
        let scene = read32(sys, SCENE.va()).map_or(0, |w| w as u16);
        patch(sys, &NO_FREEZE, opts.no_power_up_freeze);
        patch(sys, &NO_COLLISION, opts.no_player_collision);
        self.hold_timer(sys, scene, opts.no_time_limit);
        self.hide_timer(sys, scene, opts.no_time_limit);
        if self.hold_lives(sys, scene, opts.infinite_lives) {
            cheats.mark_active();
        }
        self.save_select(sys, scene, cheats);
        if sys.p1.on {
            character_pick(sys, scene, cheats, characters);
        }
        let mut lines: Vec<String> = self.autosave(sys, scene, opts.autosave).into_iter().collect();
        let Some(power_up) = self.pending else { return lines };
        let given = match scene {
            SCENE_STAGE => give_in_course(sys, power_up),
            SCENE_WORLD_MAP => give_on_map(sys, power_up),
            _ => None,
        };
        if given.is_some() {
            self.pending = None;
            cheats.mark_active();
        }
        lines.extend(given);
        lines
    }

    /// The autosave: a course left by its goal for the world map starts the
    /// watch; the save is made once the map has recorded the clear (or a
    /// replay's 5 s have passed), at a calm moment.
    fn autosave(&mut self, sys: &mut System, scene: u16, on: bool) -> Option<String> {
        let last = std::mem::replace(&mut self.last_scene, scene);
        if scene != last {
            self.watch = None;
            // The title screen's demo is a course too.
            let cleared = last == SCENE_STAGE && scene == SCENE_WORLD_MAP && read32(sys, EXIT_MODE.va()) == Some(0) && !has_profile(sys, PROFILE_OPENING_TITLE);
            let story = read32(sys, GAME_FLAG.va()).is_some_and(|f| f & FREE_MODE_OR_COIN_BATTLE == 0);
            if on && cleared && story {
                self.watch = Some(Watch { fields: 0, flags: course_flags(sys), saved: false });
            }
        }
        let w = self.watch.as_mut().filter(|_| on)?;
        w.fields += 1;
        if w.fields > AUTOSAVE_WATCH {
            self.watch = None;
            return None;
        }
        let flags = course_flags(sys);
        let recorded = flags != 0 && flags != w.flags;
        if !recorded && (w.saved || w.fields < AUTOSAVE_REPLAY) {
            return None;
        }
        // The map's menu has its own Save; and one write at a time.
        let menu_open = find_profile(sys, PROFILE_MAP_MENU).is_some_and(|m| read32(sys, m + MAP_MENU_STATE) != Some(MAP_MENU_CLOSED.va()));
        let nand_busy = pointer(sys, NAND_THREAD.va()).is_none_or(|t| read32(sys, t + NAND_BUSY) != Some(0));
        if menu_open || nand_busy || read32(sys, SAVE_GAME.va()) != Some(SAVE_GAME_FIRST) {
            return None;
        }
        // r3 unused, r4 = 0: a Save, not a Quick Save. None: not at a
        // moment the guest's threads can be touched; the next field.
        let done = sys.call_guest_as_thread(SAVE_GAME.va(), &[0, 0])?;
        let field = w.fields;
        if let Err(e) = done {
            self.watch = None;
            return Some(format!("autosave: {e}"));
        }
        (w.flags, w.saved) = (flags, true);
        Some(format!("autosave: saved {field} fields into the world map after a cleared course ({})", if recorded { "its course flags changed" } else { "5 s on the map" }))
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
        let waiting = read32(sys, screen + FILE_SELECT_STATE) == Some(FILE_SELECT_WAIT.va());
        if waiting {
            if let Some(file) = read32(sys, screen + FILE_SELECT_CURSOR).filter(|&c| c < 3) {
                cheats.active.store(file as i8, Relaxed);
            }
        }
        let id = read32(sys, screen).unwrap_or(0);
        if (self.cards.0, self.cards.1) != (screen, id) {
            self.cards = (screen, id, [0; 3]);
        }
        let games = pointer(sys, SAVE_MNG.va());
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
        let timer = if on && scene == SCENE_STAGE { pointer(sys, STAGE_TIMER.va()) } else { None };
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
        let pane = if scene == SCENE_STAGE { pointer(sys, GAME_DISPLAY.va()).and_then(|d| pointer(sys, d + GAME_DISPLAY_TIME)) } else { None };
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
            let Some(v) = read32(sys, REST.va() + 4 * i as u32) else {
                self.rest = None;
                return false;
            };
            *n = v as i32;
        }
        let mut given = false;
        if let Some(last) = self.rest {
            for i in 0..4 {
                if now[i] == last[i] - 1 {
                    write32(sys, REST.va() + 4 * i as u32, last[i] as u32);
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
    let mut node = read32(sys, TREE_ROOT.va()).unwrap_or(0);
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

pub(crate) fn has_profile(sys: &System, profile: u16) -> bool {
    find_profile(sys, profile).is_some()
}

/// Player 1's character pick, after a field: on the setup screens the cards
/// start on the save file's character and the character screen is tidied;
/// on the world map, the character player 1 plays is the file's.
fn character_pick(sys: &mut System, scene: u16, cheats: &Cheats, characters: &Characters) {
    let file = usize::try_from(cheats.active.load(Relaxed)).ok().filter(|&f| f < 3);
    match scene {
        SCENE_GAME_SETUP => {
            if let Some(f) = file {
                start_cards(sys, characters.get()[f]);
            }
            if let Some(screen) = find_profile(sys, PROFILE_CHARACTER_SCREEN) {
                sys.character_screen(screen);
            }
        }
        SCENE_WORLD_MAP => {
            if let (Some(f), Some(c)) = (file, read32(sys, PLAYER_TYPE.va()).filter(|&t| t <= 3)) {
                characters.0[f].store(c as u8, Relaxed);
            }
        }
        _ => {}
    }
}

/// The game's table of each slot's first card, with `character`'s card
/// swapped into player 1's (so no two slots start on the same card). Left
/// alone unless it is a permutation (a mod's table, or no module yet).
fn start_cards(sys: &mut System, character: u32) {
    let Some(now) = sys.mem.ram(CARD_STARTS.va(), 16).map(|b| std::array::from_fn::<u32, 4, _>(|i| u32::from_be_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]))) else { return };
    let mut sorted = now;
    sorted.sort_unstable();
    if sorted != [0, 1, 2, 3] {
        return;
    }
    let mut wanted = GAME_CARD_STARTS;
    let card = CARD_OF_CHARACTER[character.min(3) as usize];
    if let Some(i) = (1..4).find(|&i| wanted[i] == card) {
        wanted.swap(0, i);
    }
    if wanted != now {
        for (i, &w) in wanted.iter().enumerate() {
            write32(sys, CARD_STARTS.va() + 4 * i as u32, w);
        }
    }
}

/// FNV-1a over the open save file's course flags; 0 without a save manager
/// or an open file.
fn course_flags(sys: &System) -> u64 {
    let Some(mng) = pointer(sys, SAVE_MNG.va()) else { return 0 };
    let slot = sys.mem.ram(mng + SAVE_SLOT, 1).map_or(3, |b| u32::from(b[0]));
    let flags = (slot < 3).then(|| sys.mem.ram(mng + SAVE_GAMES + SAVE_GAME_SIZE * slot + COURSE_FLAGS, 4 * COURSE_FLAG_WORDS)).flatten();
    flags.map_or(0, |b| b.chunks_exact(4).fold(0xCBF2_9CE4_8422_2325, |h, w| (h ^ u64::from(u32::from_be_bytes([w[0], w[1], w[2], w[3]]))).wrapping_mul(0x100_0000_01B3)))
}

/// In a course: once player 1 is in it (not the title screen's).
fn give_in_course(sys: &mut System, power_up: u8) -> Option<String> {
    if has_profile(sys, PROFILE_OPENING_TITLE) {
        return None;
    }
    let player = sys.call_guest(GET_PLAYER.va(), &[0], &[]).ok().filter(|&p| in_ram(p))?;
    if power_up == STAR {
        let set_star = pointer(sys, player + PLAYER_VTABLE).and_then(|v| pointer(sys, v + PLAYER_SET_STAR))?;
        sys.call_guest(set_star, &[player, 0, STAR_FRAMES], &[]).ok()?;
    } else {
        sys.call_guest(CHANGE_POWER_UP.va(), &[player, u32::from(power_up)], &[]).ok()?;
    }
    Some(format!("gameplay: player 1 gets {} in the course", POWER_UPS[power_up as usize]))
}

/// On the world map, as its Items screen does with a power-up item.
fn give_on_map(sys: &mut System, power_up: u8) -> Option<String> {
    let character = read32(sys, PLAYER_TYPE.va()).filter(|&t| t <= 3)?;
    if power_up == STAR {
        // The map's own Star item: the next course starts with one.
        let items = read32(sys, CREATE_ITEM.va() + 4 * character).unwrap_or(0);
        write32(sys, CREATE_ITEM.va() + 4 * character, items | 1);
        return Some("gameplay: player 1 gets a Star on the world map (for the next course)".into());
    }
    write32(sys, PLAYER_MODE.va() + 4 * character, u32::from(power_up));
    // Going into a course takes each figure's power-up back into
    // mPlayerMode, so the figure has to have it too (the game's own map only).
    let ours = (0..4u32).all(|i| read32(sys, MAP_MODELS.va() + 4 * i) == Some([0, 1, 3, 2][i as usize]));
    let figure = pointer(sys, WM_FIGURES.va()).and_then(|m| pointer(sys, m + WM_FIGURES_BY_CHARACTER + 4 * character)).filter(|&f| pointer(sys, f + WM_FIGURE_MODELS).is_some());
    if let (true, Some(figure)) = (ours, figure) {
        let _ = sys.call_guest(WM_FIGURE_CHANGE.va(), &[figure, u32::from(power_up)], &[]);
    }
    Some(format!("gameplay: player 1 gets {} on the world map", POWER_UPS[power_up as usize]))
}
