//! The game's own Settings buttons (after WitherBlitz's NSMB-Wii-Recompilation,
//! `runtime/src/hle/nsmbw_menu_settings.cpp`): with [`crate::menu_layout`]'s
//! archives, the pause window and the world map's + menu each have a
//! Settings button, and these hooks teach the game's code to use it.
//!
//! The pause window knows two buttons, Continue (0) and Exit (1); Settings
//! is 2 and sits between them on screen:
//! - `LytBase_c::GroupRegister` (also the map menu's): the copy's four
//!   animation groups (on, idle, off, hit) are registered with the rest;
//! - `Pausewindow_c`'s cursor setup, and its open, button-change and hit
//!   states: the corner cursor and the button's animations;
//! - `PauseManager_c::SelectSoundSet` (the cursor's move): Continue,
//!   Settings, Exit in screen order (Exit only where the game allows it);
//! - `PauseManager_c::KeyChack`: 2 or A on Settings plays the decide sound
//!   and the hit animation, and the window closes as for its other
//!   buttons; the Settings window then opens ([`MenuSettings::take_open`])
//!   while the game stays paused. When it closes ([`MenuSettings::reopen`]),
//!   the pause window comes back through its own opening, on Settings.
//!
//! The world map's + menu (`dCourseSelectMenu_c`, in the prelinked
//! `d_basesNP`): Star Coins, Add/Drop Players, Quick Save (0-2), Settings
//! (4), Title Screen (3) on screen. Its cursor setup, StartWait, Open's end,
//! ButtonChange, input, and Hit's two halves know Settings; picking it plays
//! its hit animation, then the menu's out animation, and the Settings
//! window opens once the menu is gone. When it closes, the menu opens again
//! through StartWait, on Settings.
//!
//! The save-select screen (`dFileSelect_c`, also in `d_basesNP`) keeps its
//! cursor as a number: its three files (0-2), Free-for-All (3) and Coin
//! Battle (4). In the title bar the port draws Settings (a cog, 5) and
//! Multiplayer (6) on two empty panes of the layout's: up from a file goes
//! to Multiplayer, left and right between the two, and down back to the
//! file, through the screen's own button-change state (the file's card goes
//! out or lights up, the corner cursor moves, the cursor sound plays); 2 or
//! A plays the decide sound and, once the button's hit animation has had
//! its time, the Settings window or the Multiplayer screen opens. Closing
//! it leaves the cursor on its button.
//!
//! Each hook acts only while the live layout has its button (so never in a
//! net game, whose archives are not patched); otherwise it declines and the
//! game's function runs as it is. A hook calls the game's functions with
//! [`System::call_nested`].
//!
//! Exit Course in every course ([`MenuSettings::exit_course`], after
//! WitherBlitz's NSMB-Wii-Recompilation, `runtime/src/hle/pause_exit.cpp`):
//! the pause window asks `dGameCom::isNowCourseClear` whether Exit may be
//! picked (`Pausewindow_c::create` greys it out, the cursor's move and the
//! confirm refuse it). In a course it says yes, and the player leaves
//! through the game's own quit path (exit mode 2: nothing saved, the
//! power-ups the course began with). The world map enemies' ambushes
//! (levels 0x20-0x22) keep the game's answer, as their window's cursor
//! never moves in the game either, and so does the title screen's course
//! (01-40, level 0x27), which has no pause window but asks too.

use game_version::E1;
use guest_cpu::{MSR_EE, MSR_FP};

use crate::System;

/// `LytBase_c::GroupRegister(names, animations, count)`.
const GROUP_REGISTER: E1 = E1(0x800C_90F0);
const PAUSE_CURSOR_SETUP: E1 = E1(0x8015_AC30);
const PAUSE_OPEN: E1 = E1(0x8015_AD80);
const PAUSE_BUTTON_CHANGE: E1 = E1(0x8015_AE70);
const PAUSE_HIT: E1 = E1(0x8015_B030);
/// `PauseManager_c::SelectSoundSet`, the cursor's move.
const PAUSE_CURSOR_MOVE: E1 = E1(0x800D_0C00);
const PAUSE_KEY_CHECK: E1 = E1(0x800D_0CB0);
const MAP_CURSOR_SETUP: E1 = E1(0x8077_AD70);
const MAP_START_WAIT: E1 = E1(0x8077_ADC0);
const MAP_OPEN_END: E1 = E1(0x8077_AEF0);
const MAP_BUTTON_CHANGE: E1 = E1(0x8077_AF10);
const MAP_INPUT: E1 = E1(0x8077_B010);
const MAP_HIT: E1 = E1(0x8077_B1A0);
const MAP_HIT_WAIT: E1 = E1(0x8077_B1C0);
/// `dFileSelect_c`: SelectWait's execute, and ButtonSelectAnimeEndWait's
/// initialize and finalize.
const FILE_SELECT_WAIT: E1 = E1(0x8078_3C10);
const FILE_CHANGE: E1 = E1(0x8078_3A00);
const FILE_CHANGE_END: E1 = E1(0x8078_3BB0);

/// The hooked functions, for [`crate::SPECIAL`].
pub const HOOKS: [E1; 18] = [
    GROUP_REGISTER,
    PAUSE_CURSOR_SETUP,
    PAUSE_OPEN,
    PAUSE_BUTTON_CHANGE,
    PAUSE_HIT,
    PAUSE_CURSOR_MOVE,
    PAUSE_KEY_CHECK,
    MAP_CURSOR_SETUP,
    MAP_START_WAIT,
    MAP_OPEN_END,
    MAP_BUTTON_CHANGE,
    MAP_INPUT,
    MAP_HIT,
    MAP_HIT_WAIT,
    FILE_SELECT_WAIT,
    FILE_CHANGE,
    FILE_CHANGE_END,
    IS_NOW_COURSE_CLEAR,
];

/// The map menu's and the save-select screen's functions are in a module:
/// their first words, checked before a hook acts.
const MODULE_WORDS: [(E1, u32); 10] = [
    (MAP_CURSOR_SETUP, 0x8803_0274),
    (MAP_START_WAIT, 0x9421_FFF0),
    (MAP_OPEN_END, 0x3880_0000),
    (MAP_BUTTON_CHANGE, 0x9421_FFF0),
    (MAP_INPUT, 0x9421_FFE0),
    (MAP_HIT, 0x8083_0268),
    (MAP_HIT_WAIT, 0x9421_FFF0),
    (FILE_SELECT_WAIT, 0x9421_FFE0),
    (FILE_CHANGE, 0x9421_FFF0),
    (FILE_CHANGE_END, 0x8003_02A4),
];

const PAUSEWINDOW: E1 = E1(0x8042_A33C); // Pausewindow_c::m_instance
const GAME_KEY: E1 = E1(0x8042_9F50); // dGameKey_c::m_instance
const GAME_STOP: E1 = E1(0x8042_9F48); // dGameCom::l_gameStop
const SND_AUDIO_MGR: E1 = E1(0x8042_A488); // SndAudioMgr::sInstance
const START_GAME_INFO: E1 = E1(0x8031_5B90); // dInfo_c::m_startGameInfo, +0xD the level
const START_SYSTEM_SE: E1 = E1(0x8019_5380);
/// `dGameCom::isNowCourseClear`: whether the pause window's Exit may be picked.
const IS_NOW_COURSE_CLEAR: E1 = E1(0x800B_4DE0);
const NOW_SCENE: E1 = E1(0x8042_8732); // dScene_c::m_nowScene
/// The title screen's course, 01-40 (`m_startGameInfo`'s level there).
const TITLE_LEVEL: u8 = 0x27;
const SCENE_STAGE: u16 = 5;
const SELECT_CURSOR_SETUP: E1 = E1(0x800B_44D0); // dGameCom::SelectCursorSetup(pane, slot, bool)
const ANIME_START: E1 = E1(0x800C_92F0);
const REVERSE_ANIME_START: E1 = E1(0x800C_93D0);
const ALL_ANIME_END: E1 = E1(0x800C_9500);

// Pausewindow_c
const LAYOUT: u32 = 0x70;
const NEXT_BUTTON: u32 = 0x284;
const ACTIVE_BUTTON: u32 = 0x288;
const IS_ACTIVE: u32 = 0x291;
const BUTTON_HIT: u32 = 0x293;
const IS_ANIMATING: u32 = 0x294;
// PauseManager_c: +8 the cursor, +0xC the player who paused.
const CURSOR: u32 = 0x8;
const PLAYER: u32 = 0xC;

const SETTINGS: u32 = 2;
/// The pause window's animations: on, off and hit of Continue (0), Exit (1)
/// and Settings (2).
const ON: [u32; 3] = [3, 4, 11];
const OFF: [u32; 3] = [7, 8, 13];
const HIT: [u32; 3] = [9, 10, 14];
const WINDOW_GROUPS: u32 = 11;

// dCourseSelectMenu_c: +0x70 the layout (LytBase_c: +0x188 each group's
// playing flag, +0x194 the last one started), +0x208 its state manager,
// +0x244 the window pane, +0x268 the button the
// cursor goes to, +0x26C the one it is on, +0x272 a button was picked,
// +0x273 input off, +0x274 the cursor wants placing. Animation groups: 0 in,
// on 2-5, hit 10-13, off 14-17, 18 out; Settings' on, idle, hit, off 19-22.
const MAP_WINDOW: u32 = 0x244;
const MAP_NEXT: u32 = 0x268;
const MAP_ACTIVE: u32 = 0x26C;
const MAP_PICKED: u32 = 0x272;
const MAP_LOCKED: u32 = 0x273;
const MAP_STATES: u32 = 0x208; // changeState is its vtable's +0x18
const MAP_STATE_START_WAIT: E1 = E1(0x8099_15F0);
const MAP_STATE_BUTTON_CHANGE: E1 = E1(0x8099_1670);
const MAP_STATE_HIT: E1 = E1(0x8099_16F0);
const MAP_STATE_EXIT: E1 = E1(0x8099_1730);
const MAP_STATE_OPEN: E1 = E1(0x8099_1630);
/// Its create's tables: the group names (0x80941288 + 404) and their
/// animations.
const MAP_GROUP_NAMES: E1 = E1(0x8094_1288 + 404);
const MAP_GROUP_ANIMS: E1 = E1(0x8093_3FD8);
const MAP_CLOSE_KEY: E1 = E1(0x800B_53A0); // dGameCom::chkCancelButton
const MAP_KEYS: E1 = E1(0x8042_A468); // its vtable's +0x20: was one of these pressed
const MAP_PLAYER: E1 = E1(0x8042_A464); // the player the menu reads
const MAP_IN: u32 = 0;
const MAP_OUT: u32 = 18;
const MAP_GROUPS: u32 = 19;
const MAP_SETTINGS: u32 = 4;
const SE_CURSOR: u32 = 0x78;
const SE_DECIDE: u32 = 0x79;
const SE_BACK: u32 = 0x7A;

// dFileSelect_c: +0x74 the layout, +0x20C its state manager, +0x24C its
// three file cards (a card's +0xDD8: 1 to light it up, 4 to put it out),
// +0x2A4 the cursor, +0x2A8 where it was, +0x2CC the file it was last on.
const FILE_LAYOUT: u32 = 0x74;
const FILE_STATES: u32 = 0x20C;
const FILE_CARDS: u32 = 0x24C;
const FILE_CURSOR: u32 = 0x2A4;
const FILE_PREV: u32 = 0x2A8;
const FILE_LAST: u32 = 0x2CC;
const CARD_COMMAND: u32 = 0xDD8;
const FILE_SETTINGS: u32 = 5;
const FILE_MULTI: u32 = 6;
/// Fields between 2 or A on one of them and its window (the port's hit
/// animation of the button).
const FILE_HIT_FIELDS: u8 = 18;
const FILE_STATE_CHANGE: E1 = E1(0x8099_2010); // ButtonSelectAnimeEndWait
const FILE_STATE_RETURN_TITLE: E1 = E1(0x8099_20D0);
/// The multi-player buttons' on animations (Free-for-All, Coin Battle).
const FILE_MULTI_ON: u32 = 0x22;

/// Where nested calls return to; never executed (the call stops there).
const NESTED_RETURN: u32 = 0x8000_1800;
/// A nested call that runs longer than this is cut off.
const NESTED_LIMIT: u64 = 20_000_000;

pub(crate) fn guest_ram(a: u32) -> bool {
    (0x8000_0000..0x8180_0000).contains(&a) || (0x9000_0000..0x9400_0000).contains(&a)
}

/// The Settings buttons' state on the host.
#[derive(Default)]
pub struct MenuSettings {
    /// The pause window, the world map's menu and the save-select screen
    /// were given their button ([`crate::menu_layout::add_buttons`]).
    pub buttons: (bool, bool, bool),
    /// The save files' cards were given their "Cheated" text
    /// ([`crate::menu_layout::add_cheated_label`]).
    pub cheated_label: bool,
    /// Exit Course in every course, not only the cleared ones (the Settings
    /// window's Gameplay option; in a net game only if every player has it).
    pub exit_course: bool,
    /// The pause window's button was picked: the Settings window opens once
    /// the window has closed.
    open_requested: bool,
    /// The map menu's button was picked and the menu is gone.
    map_open_requested: bool,
    /// The Settings window closed: the pause window opens again ...
    reopen: bool,
    /// ... and is opening now, Settings selected.
    reopening: bool,
    map_reopen: bool,
    /// Map menu, Settings after its hit: 0 not picked, 1 the menu going
    /// out, 2 gone (the Settings window is up).
    map_settings: u8,
    /// The map menu is opening again after the Settings window: on Settings.
    map_back: bool,
    /// The save-select screen's Settings or Multiplayer after 2 or A: 0 not
    /// picked, 1 its hit animation playing, 2 its window is up.
    file_settings: u8,
    /// Which was picked (5 or 6), and the fields its hit still plays.
    file_picked: u32,
    file_hit_left: u8,
    file_open_requested: bool,
    file_multiplayer_requested: bool,
    /// The save-select screen and its Settings and Multiplayer panes, once
    /// found.
    file_panes: (u32, [u32; 2]),
    /// A hooked function the next call runs as the game has it.
    bypass: Option<u32>,
    /// Main.dol's copies of the groups' names (0: not found yet).
    group_names: Option<(u32, u32)>,
}

impl MenuSettings {
    /// A Settings button was picked since the last call: true once its
    /// menu is gone, for the Settings window to open.
    pub fn take_open(&mut self) -> bool {
        std::mem::take(&mut self.open_requested) | std::mem::take(&mut self.map_open_requested) | std::mem::take(&mut self.file_open_requested)
    }

    /// The save-select screen's Multiplayer was picked: true once, for the
    /// Multiplayer screen to open.
    pub fn take_open_multiplayer(&mut self) -> bool {
        std::mem::take(&mut self.file_multiplayer_requested)
    }

    /// The save select's button picked (5 Settings, 6 Multiplayer) while its
    /// hit plays or its window is up.
    pub fn file_picked(&self) -> Option<u32> {
        (self.file_settings != 0).then_some(self.file_picked)
    }

    /// The pause window's Settings was picked, waiting for it to close.
    pub fn waiting_for_pause(&self) -> bool {
        self.open_requested
    }

    /// The Settings window opened from a game menu has closed: that menu
    /// opens again, on Settings.
    pub fn reopen(&mut self) {
        if self.file_settings == 2 {
            // The save-select screen never left: its cursor is on Settings.
            self.file_settings = 0;
        } else if self.map_settings == 2 {
            self.map_reopen = true;
        } else {
            self.reopen = true;
        }
    }
}

impl System {
    /// Calls guest function `addr` with `args` in r3.. from a hook, with its
    /// stack at `sp`, interrupts off and the FPU on, and returns its r3. The
    /// CPU is then put back as it was (the clock keeps the time the call
    /// took), so a hook changes no register but those it sets itself.
    pub fn call_nested(&mut self, addr: u32, args: &[u32], sp: u32) -> u32 {
        self.call_nested_cpu(addr, args, sp).gpr[3]
    }

    /// [`System::call_nested`], returning the CPU as the call left it (its
    /// results: r3, f1).
    pub fn call_nested_cpu(&mut self, addr: u32, args: &[u32], sp: u32) -> guest_cpu::Cpu {
        let saved = self.cpu.clone();
        self.cpu.gpr[1] = sp;
        for (i, &a) in args.iter().enumerate() {
            self.cpu.gpr[3 + i] = a;
        }
        self.cpu.lr = NESTED_RETURN;
        self.cpu.pc = addr;
        self.cpu.msr = (self.cpu.msr & !MSR_EE) | MSR_FP;
        let mut n = 0;
        while self.cpu.pc != NESTED_RETURN && n < NESTED_LIMIT {
            let pc = self.cpu.pc;
            if self.addrs.may_be_special(pc) {
                self.before(pc);
                if self.cpu.pc != pc {
                    continue;
                }
            }
            if let Err(e) = self.cpu.step(&mut self.mem) {
                eprintln!("menu hook: call to {addr:#010x} stopped at {:#010x}: {e:?}", self.cpu.pc);
                break;
            }
            n += 1;
        }
        if n == NESTED_LIMIT {
            eprintln!("menu hook: call to {addr:#010x} did not return");
        }
        self.steps += n;
        let after = std::mem::replace(&mut self.cpu, saved);
        self.cpu.cycles = after.cycles;
        after
    }

    /// [`System::call_nested`] on the hooked thread's stack.
    fn call(&mut self, addr: u32, args: &[u32]) -> u32 {
        let sp = self.cpu.gpr[1];
        self.call_nested(addr, args, sp)
    }

    /// The game's own `addr` (a hooked function), past its hook.
    fn call_original(&mut self, addr: u32, args: &[u32], sp: u32) -> u32 {
        self.menu.bypass = Some(addr);
        let r = self.call_nested(addr, args, sp);
        self.menu.bypass = None;
        r
    }

    pub(crate) fn r8(&mut self, a: u32) -> u8 {
        self.mem.read_u8(a).unwrap_or(0)
    }
    pub(crate) fn r32(&mut self, a: u32) -> u32 {
        self.mem.read_u32(a).unwrap_or(0)
    }
    pub(crate) fn w8(&mut self, a: u32, v: u8) {
        let _ = self.mem.write_u8(a, v);
    }
    pub(crate) fn w32(&mut self, a: u32, v: u32) {
        let _ = self.mem.write_u32(a, v);
    }

    /// The menu hook at `pc`, if it applies: the function's work done and
    /// the CPU at its return address. False to run the game's function.
    pub(crate) fn menu_hook(&mut self, pc: u32) -> bool {
        if self.menu.bypass == Some(pc) {
            self.menu.bypass = None;
            return false;
        }
        if let Some(&(_, word)) = MODULE_WORDS.iter().find(|w| w.0.try_va() == Some(pc)) {
            if self.r32(pc) != word {
                return false;
            }
        }
        let hooks: [(E1, fn(&mut Self) -> bool); 18] = [
            (GROUP_REGISTER, Self::group_register),
            (PAUSE_CURSOR_SETUP, Self::pause_cursor_setup),
            (PAUSE_OPEN, Self::pause_open),
            (PAUSE_BUTTON_CHANGE, Self::pause_button_change),
            (PAUSE_HIT, Self::pause_hit),
            (PAUSE_CURSOR_MOVE, Self::pause_cursor_move),
            (PAUSE_KEY_CHECK, Self::pause_key_check),
            (MAP_CURSOR_SETUP, Self::map_cursor_setup),
            (MAP_START_WAIT, Self::map_start_wait),
            (MAP_OPEN_END, Self::map_open_end),
            (MAP_BUTTON_CHANGE, Self::map_button_change),
            (MAP_INPUT, Self::map_input),
            (MAP_HIT, Self::map_hit),
            (MAP_HIT_WAIT, Self::map_hit_wait),
            (FILE_SELECT_WAIT, Self::file_select_wait),
            (FILE_CHANGE, Self::file_change),
            (FILE_CHANGE_END, Self::file_change_end),
            (IS_NOW_COURSE_CLEAR, Self::exit_any_course),
        ];
        let ran = hooks.iter().find(|h| h.0.try_va() == Some(pc)).is_some_and(|h| (h.1)(self));
        if ran {
            self.cpu.pc = self.cpu.lr;
        }
        ran
    }

    // ------------------------------------------------------------ layouts

    pub(crate) fn pane_name(&mut self, pane: u32) -> String {
        let mut s = String::new();
        for i in 0..16 {
            match self.r8(pane + 0xBC + i) {
                0 => break,
                c => s.push(c as char),
            }
        }
        s
    }

    /// nw4r::lyt::Pane `name` under `pane` (0 if none; `depth`: 0 from
    /// outside): children at +0x10 (the list's terminator at +0x14), each
    /// linked at +4.
    pub fn find_pane(&mut self, pane: u32, name: &str, depth: u32) -> u32 {
        if depth > 12 || !guest_ram(pane) {
            return 0;
        }
        if self.pane_name(pane) == name {
            return pane;
        }
        let head = pane + 0x14;
        let mut node = self.r32(head);
        for _ in 0..256 {
            if node == 0 || node == head {
                break;
            }
            let found = self.find_pane(node.wrapping_sub(4), name, depth + 1);
            if found != 0 {
                return found;
            }
            node = self.r32(node);
        }
        0
    }

    /// The root pane a layout object at `object` points to (0 if none).
    pub fn root_pane(&mut self, object: u32) -> u32 {
        for off in (0..0x200).step_by(4) {
            let p = self.r32(object + off);
            if guest_ram(p) && self.pane_name(p) == "RootPane" {
                return p;
            }
        }
        0
    }

    /// Where main.dol's constant data holds `name` (0 if nowhere).
    fn find_string(&mut self, name: &str) -> u32 {
        let from = E1(0x802E_0000).va();
        let Some(ram) = self.mem.ram(from, 0x6_0000) else { return 0 };
        let mut needle = name.as_bytes().to_vec();
        needle.push(0);
        (1..ram.len().saturating_sub(needle.len()))
            .find(|&i| ram[i - 1] == 0 && ram[i..i + needle.len()] == needle[..])
            .map_or(0, |i| from + i as u32)
    }

    fn group_names(&mut self) -> (u32, u32) {
        if let Some(g) = self.menu.group_names {
            return g;
        }
        let g = (self.find_string(crate::menu_layout::PAUSE_GROUP), self.find_string(crate::menu_layout::MAP_GROUP));
        self.menu.group_names = Some(g);
        g
    }

    fn sound(&mut self, id: u32) {
        let sound = self.r32(SND_AUDIO_MGR.va());
        if sound != 0 {
            self.call(START_SYSTEM_SE.va(), &[sound, id, 1]);
        }
    }

    /// The key trigger bits (WPAD buttons) of `player`.
    fn trigger(&mut self, player: u32) -> u32 {
        let keys = self.r32(GAME_KEY.va());
        if player >= 4 || keys == 0 {
            return 0;
        }
        let key = self.r32(keys + 4 + player * 4);
        if key == 0 {
            return 0;
        }
        self.r32(key + 0x1C)
    }

    /// The animation groups of a layout with a Settings button, its four
    /// after the game's: `GroupRegister` with tables extended on the stack,
    /// below the caller's frame.
    fn group_register(&mut self) -> bool {
        let (layout, names, anims, count) = (self.cpu.gpr[3], self.cpu.gpr[4], self.cpu.gpr[5], self.cpu.gpr[6]);
        let (pause_group, map_group) = self.group_names();
        let window = self.r32(PAUSEWINDOW.va());
        let (group, settings_anims) = if count == WINDOW_GROUPS && guest_ram(window) && layout == window + LAYOUT {
            // onButton, idleButton, offButon (sic), hitButton.
            (pause_group, [3, 4, 5, 6])
        } else if count == MAP_GROUPS && names == MAP_GROUP_NAMES.va() {
            // Copies of Quick Save's on, idle, hit and off.
            let a = [4, 8, 12, 16].map(|g| self.r32(MAP_GROUP_ANIMS.va() + g * 4));
            (map_group, a)
        } else {
            return false;
        };
        let pane = if count == WINDOW_GROUPS { "P_SBBase_03" } else { "P_SBBase_06" };
        let root = self.root_pane(layout);
        if group == 0 || self.find_pane(root, pane, 0) == 0 {
            return false;
        }
        let sp = self.cpu.gpr[1];
        let frame = (sp - 0x20 - (count + 4) * 8) & !0xF;
        let (new_names, new_anims) = (frame + 0x10, frame + 0x10 + (count + 4) * 4);
        self.w32(frame, sp); // a back chain, as a real frame has
        for i in 0..count {
            let (n, a) = (self.r32(names + i * 4), self.r32(anims + i * 4));
            self.w32(new_names + i * 4, n);
            self.w32(new_anims + i * 4, a);
        }
        for (i, a) in settings_anims.into_iter().enumerate() {
            self.w32(new_names + (count + i as u32) * 4, group);
            self.w32(new_anims + (count + i as u32) * 4, a);
        }
        self.call_original(GROUP_REGISTER.va(), &[layout, new_names, new_anims, count + 4], frame);
        true
    }

    // ------------------------------------------------------------ the pause window

    fn pausewindow(&mut self) -> u32 {
        let w = self.r32(PAUSEWINDOW.va());
        if guest_ram(w) {
            w
        } else {
            0
        }
    }

    /// The pause window's Settings pane (P_SBBase_03), 0 if its layout has none.
    fn pause_settings_pane(&mut self, window: u32) -> u32 {
        let root = self.root_pane(window + LAYOUT);
        self.find_pane(root, "P_SBBase_03", 0)
    }

    fn pause_cursor_setup(&mut self) -> bool {
        let window = self.cpu.gpr[3];
        if self.r32(window + NEXT_BUTTON) != SETTINGS {
            return false;
        }
        let pane = self.pause_settings_pane(window);
        if pane != 0 {
            self.call(SELECT_CURSOR_SETUP.va(), &[pane, 0, 0]);
        }
        true
    }

    fn pause_open(&mut self) -> bool {
        let window = self.cpu.gpr[3];
        if self.pause_settings_pane(window) == 0 {
            return false;
        }
        let sp = self.cpu.gpr[1];
        self.call_original(PAUSE_OPEN.va(), &[window], sp);
        // The Settings button starts plain like the others.
        self.call(REVERSE_ANIME_START.va(), &[window + LAYOUT, ON[2], 0]);
        if self.menu.reopening {
            self.menu.reopening = false;
            self.w32(window + NEXT_BUTTON, SETTINGS); // back from the Settings window
        }
        true
    }

    fn pause_button_change(&mut self) -> bool {
        let window = self.cpu.gpr[3];
        let next = self.r32(window + NEXT_BUTTON);
        let active = self.r32(window + ACTIVE_BUTTON) as i32;
        if next != SETTINGS && active != SETTINGS as i32 {
            return false;
        }
        self.call(ALL_ANIME_END.va(), &[window + LAYOUT]);
        self.w8(window + IS_ANIMATING, 1);
        if active >= 0 {
            self.call(ANIME_START.va(), &[window + LAYOUT, OFF[(active as usize).min(2)], 0]);
        }
        self.w32(window + ACTIVE_BUTTON, next);
        self.call(ANIME_START.va(), &[window + LAYOUT, ON[(next as usize).min(2)], 0]);
        true
    }

    fn pause_hit(&mut self) -> bool {
        let window = self.cpu.gpr[3];
        if self.r32(window + NEXT_BUTTON) != SETTINGS {
            return false;
        }
        self.w8(window + IS_ANIMATING, 1);
        self.call(ANIME_START.va(), &[window + LAYOUT, HIT[2], 0]);
        true
    }

    /// The cursor's move: r4 is the cursor plus or minus one in the game's
    /// numbering (Continue 0, Exit 1); on screen the order is Continue,
    /// Settings, Exit.
    fn pause_cursor_move(&mut self) -> bool {
        let window = self.pausewindow();
        if window == 0 || self.pause_settings_pane(window) == 0 {
            return false;
        }
        let manager = self.cpu.gpr[3];
        let cursor = self.r32(manager + CURSOR).min(2) as i32;
        let delta = if (self.cpu.gpr[4] as i32) - cursor < 0 { -1 } else { 1 };
        const SCREEN: [i32; 3] = [0, 2, 1]; // button -> place on screen
        const BUTTON_AT: [u32; 3] = [0, 2, 1]; // place on screen -> button
        let place = (SCREEN[cursor as usize] + delta).clamp(0, 2);
        let target = BUTTON_AT[place as usize];
        if target == 1 {
            // Exit only where the game allows it (a cleared course or the
            // cannon course; any course with Exit Course on, which
            // isNowCourseClear's hook answers), and never in the map enemies'
            // ambushes (levels 0x20-0x22), whose window doesn't move in the
            // game either.
            let level = self.r8(START_GAME_INFO.va() + 0xD);
            let cleared = self.call(IS_NOW_COURSE_CLEAR.va(), &[]) != 0;
            if !((cleared || level == 0x23) && !(0x20..=0x22).contains(&level)) {
                return true;
            }
        }
        if target != cursor as u32 {
            self.w32(window + NEXT_BUTTON, target);
            self.w32(manager + CURSOR, target);
        }
        true
    }

    fn pause_key_check(&mut self) -> bool {
        let manager = self.cpu.gpr[3];
        let window = self.pausewindow();
        if window == 0 {
            return false;
        }
        // Back from the Settings window: the pause window opens again
        // through its own opening.
        if std::mem::take(&mut self.menu.reopen) {
            self.menu.reopening = true;
            self.w32(manager + CURSOR, SETTINGS);
            self.w8(window + IS_ACTIVE, 1);
            return true;
        }
        if self.r32(manager + CURSOR) != SETTINGS {
            return false;
        }
        let still = self.r32(GAME_STOP.va()) & 0xFFFF_FFFA == 0 && self.r8(window + IS_ANIMATING) == 0 && self.r8(window + IS_ACTIVE) != 0;
        let player = self.r32(manager + PLAYER);
        let trigger = self.trigger(player);
        // 2 or A, not +.
        if !still || trigger & 0x0900 == 0 || trigger & 0x0010 != 0 {
            return false;
        }
        self.w8(window + BUTTON_HIT, 1); // the hit animation, then the window closes
        self.sound(SE_DECIDE);
        self.menu.open_requested = true;
        true
    }

    // ------------------------------------------------------------ the save-select screen

    /// The save-select screen's Settings and Multiplayer panes, 0 if its
    /// layout has none.
    fn file_panes(&mut self, this: u32) -> [u32; 2] {
        let (screen, panes) = self.menu.file_panes;
        let names = [crate::menu_layout::FILE_COG_PANE, crate::menu_layout::FILE_MULTI_PANE];
        if screen == this && panes[0] != 0 && panes[1] != 0 && self.pane_name(panes[0]) == names[0] && self.pane_name(panes[1]) == names[1] {
            return panes;
        }
        let root = self.root_pane(this + FILE_LAYOUT);
        let panes = names.map(|n| self.find_pane(root, n, 0));
        self.menu.file_panes = (this, panes);
        panes
    }

    fn file_change_state(&mut self, this: u32, state: u32) {
        let states = this + FILE_STATES;
        let vtable = self.r32(states);
        let change = self.r32(vtable + 0x18);
        self.call(change, &[states, state]);
    }

    /// The cursor from `from` to `next`, through the screen's button-change
    /// state.
    fn file_move(&mut self, this: u32, from: u32, next: u32) {
        self.w32(this + FILE_PREV, from);
        self.w32(this + FILE_CURSOR, next);
        self.file_change_state(this, FILE_STATE_CHANGE.va());
    }

    /// SelectWait: up from a file goes to Multiplayer; on Settings or
    /// Multiplayer, left and right go between them, down back to the file,
    /// 1 leaves the screen as it does anywhere on it, and 2 or A picks.
    fn file_select_wait(&mut self) -> bool {
        let this = self.cpu.gpr[3];
        let cursor = self.r32(this + FILE_CURSOR);
        let ours = cursor == FILE_SETTINGS || cursor == FILE_MULTI;
        if !ours && cursor >= 3 {
            return false;
        }
        if self.file_panes(this).contains(&0) {
            return false;
        }
        let player = self.r32(MAP_PLAYER.va());
        let trigger = self.trigger(player);
        if !ours {
            // 0x2: up, the remote held sideways.
            if trigger & 0x2 == 0 {
                return false;
            }
            self.file_move(this, cursor, FILE_MULTI);
            return true;
        }
        match self.menu.file_settings {
            1 => {
                // Once the button's hit has played, its window.
                self.menu.file_hit_left = self.menu.file_hit_left.saturating_sub(1);
                if self.menu.file_hit_left == 0 {
                    self.menu.file_settings = 2;
                    if self.menu.file_picked == FILE_MULTI {
                        self.menu.file_multiplayer_requested = true;
                    } else {
                        self.menu.file_open_requested = true;
                    }
                }
                return true;
            }
            2 => return true,
            _ => {}
        }
        // Sideways: 0x1 down, 0x8 left, 0x4 right.
        if trigger & 0x1 != 0 {
            let file = self.r32(this + FILE_LAST).min(2);
            self.file_move(this, cursor, file);
        } else if trigger & 0x8 != 0 && cursor == FILE_SETTINGS {
            self.file_move(this, cursor, FILE_MULTI);
        } else if trigger & 0x4 != 0 && cursor == FILE_MULTI {
            self.file_move(this, cursor, FILE_SETTINGS);
        } else if self.call(MAP_CLOSE_KEY.va(), &[0]) != 0 {
            self.sound(SE_BACK);
            self.file_change_state(this, FILE_STATE_RETURN_TITLE.va());
        } else {
            let keys = self.r32(MAP_KEYS.va());
            let vtable = self.r32(keys);
            let pressed = self.r32(vtable + 0x20);
            if self.call(pressed, &[keys, 0x900]) != 0 {
                self.sound(SE_DECIDE);
                self.menu.file_settings = 1;
                self.menu.file_picked = cursor;
                self.menu.file_hit_left = FILE_HIT_FIELDS;
            }
        }
        true
    }

    /// ButtonSelectAnimeEndWait's start, to or from Settings or
    /// Multiplayer: a file's card goes out or lights up (the port draws its
    /// own buttons' looks), with the cursor sound.
    fn file_change(&mut self) -> bool {
        let this = self.cpu.gpr[3];
        let (prev, next) = (self.r32(this + FILE_PREV), self.r32(this + FILE_CURSOR));
        let ours = |c: u32| c == FILE_SETTINGS || c == FILE_MULTI;
        if !ours(prev) && !ours(next) {
            return false;
        }
        let layout = this + FILE_LAYOUT;
        self.call(ALL_ANIME_END.va(), &[layout]);
        if prev < 3 {
            let card = self.r32(this + FILE_CARDS + prev * 4);
            self.w32(card + CARD_COMMAND, 4);
        }
        self.sound(SE_CURSOR);
        if next < 3 {
            let card = self.r32(this + FILE_CARDS + next * 4);
            self.w32(card + CARD_COMMAND, 1);
        } else if !ours(next) {
            self.call(ANIME_START.va(), &[layout, FILE_MULTI_ON + (next - 3).min(1), 0]);
        }
        true
    }

    /// ButtonSelectAnimeEndWait's end: the corner cursor onto Settings or
    /// Multiplayer.
    fn file_change_end(&mut self) -> bool {
        let this = self.cpu.gpr[3];
        let cursor = self.r32(this + FILE_CURSOR);
        if cursor != FILE_SETTINGS && cursor != FILE_MULTI {
            return false;
        }
        let pane = self.file_panes(this)[(cursor - FILE_SETTINGS) as usize];
        if pane != 0 {
            self.call(SELECT_CURSOR_SETUP.va(), &[pane, 1, 0]); // the screen's cursor slot
        }
        true
    }

    /// `isNowCourseClear` with Exit Course on: yes in a course, but in the
    /// map enemies' ambushes and the title screen's course. The game's
    /// function only reads, so its answer is not needed when it would be
    /// replaced anyway.
    fn exit_any_course(&mut self) -> bool {
        let level = self.r8(START_GAME_INFO.va() + 0xD);
        if !self.menu.exit_course || self.mem.read_u16(NOW_SCENE.va()) != Ok(SCENE_STAGE) || (0x20..=0x22).contains(&level) || level == TITLE_LEVEL {
            return false;
        }
        self.cpu.gpr[3] = 1;
        true
    }

    /// The pause window has closed (or there is none).
    pub fn pause_window_closed(&mut self) -> bool {
        let window = self.pausewindow();
        window == 0 || (self.r8(window + IS_ACTIVE) == 0 && self.r8(window + IS_ANIMATING) == 0)
    }

    // ------------------------------------------------------------ the world map's + menu

    /// The map menu's Settings pane (P_SBBase_06), 0 if its layout has none.
    fn map_settings_pane(&mut self, menu: u32) -> u32 {
        let root = self.root_pane(menu + LAYOUT);
        self.find_pane(root, "P_SBBase_06", 0)
    }

    fn map_change_state(&mut self, menu: u32, state: u32) {
        let states = menu + MAP_STATES;
        let vtable = self.r32(states);
        let change = self.r32(vtable + 0x18);
        self.call(change, &[states, state]);
    }

    /// `LytBase_c::isAnime(-1)`: the last animation started still plays.
    fn map_animating(&mut self, menu: u32) -> bool {
        let layout = menu + LAYOUT;
        let last = self.r32(layout + 0x194);
        let flags = self.r32(layout + 0x188);
        self.r8(flags.wrapping_add(last)) != 0
    }

    fn map_show_window(&mut self, menu: u32, shown: bool) {
        let pane = self.r32(menu + MAP_WINDOW);
        let flags = self.r8(pane + 0xBB);
        self.w8(pane + 0xBB, if shown { flags | 1 } else { flags & 0xFE });
    }

    /// The next button up or down in screen order, round at the ends.
    fn map_step(button: u32, down: bool) -> u32 {
        const ORDER: [u32; 5] = [0, 1, 2, MAP_SETTINGS, 3];
        let at = ORDER.iter().position(|&b| b == button).unwrap_or(0);
        ORDER[(at + if down { 1 } else { 4 }) % 5]
    }

    fn map_on(button: u32) -> u32 {
        if button < 4 {
            2 + button
        } else {
            19
        }
    }
    fn map_hit_anim(button: u32) -> u32 {
        if button < 4 {
            10 + button
        } else {
            21
        }
    }
    fn map_off(button: u32) -> u32 {
        if button < 4 {
            14 + button
        } else {
            22
        }
    }

    /// The corner cursor onto Settings (the game's own for the others).
    fn map_cursor_setup(&mut self) -> bool {
        let menu = self.cpu.gpr[3];
        if self.r32(menu + MAP_NEXT) != MAP_SETTINGS {
            return false;
        }
        let pane = self.map_settings_pane(menu);
        if pane != 0 {
            self.call(SELECT_CURSOR_SETUP.va(), &[pane, 4, 0]); // the menu's cursor slot
        }
        true
    }

    /// StartWait: the world map has opened the menu. Every button plain,
    /// the in-animation last (the menu waits on the last one started).
    fn map_start_wait(&mut self) -> bool {
        let menu = self.cpu.gpr[3];
        if self.map_settings_pane(menu) == 0 {
            return false;
        }
        let layout = menu + LAYOUT;
        self.call(ALL_ANIME_END.va(), &[layout]);
        for button in 0..3 {
            self.call(REVERSE_ANIME_START.va(), &[layout, Self::map_on(button), 0]);
        }
        self.call(REVERSE_ANIME_START.va(), &[layout, Self::map_on(MAP_SETTINGS), 0]);
        self.call(ANIME_START.va(), &[layout, MAP_IN, 0]);
        self.w8(menu + MAP_PICKED, 0);
        self.map_show_window(menu, true);
        self.map_change_state(menu, MAP_STATE_OPEN.va());
        true
    }

    /// Open's end: the cursor starts on Settings when back from its window.
    fn map_open_end(&mut self) -> bool {
        if !std::mem::take(&mut self.menu.map_back) {
            return false;
        }
        let menu = self.cpu.gpr[3];
        self.w32(menu + MAP_NEXT, MAP_SETTINGS);
        self.w32(menu + MAP_ACTIVE, 0xFFFF_FFFF);
        true
    }

    /// ButtonChange: the old button off (with the cursor sound), the new on.
    fn map_button_change(&mut self) -> bool {
        let menu = self.cpu.gpr[3];
        let next = self.r32(menu + MAP_NEXT);
        let active = self.r32(menu + MAP_ACTIVE) as i32;
        if next != MAP_SETTINGS && active != MAP_SETTINGS as i32 {
            return false;
        }
        let layout = menu + LAYOUT;
        self.call(ALL_ANIME_END.va(), &[layout]);
        if active >= 0 {
            self.sound(SE_CURSOR);
            self.call(ANIME_START.va(), &[layout, Self::map_off(active as u32), 0]);
        }
        self.w32(menu + MAP_ACTIVE, next);
        self.call(ANIME_START.va(), &[layout, Self::map_on(next), 0]);
        true
    }

    /// PauseDisp's input: up and down through the five buttons, + (or the
    /// game's other way out) closes, 2 or A picks.
    fn map_input(&mut self) -> bool {
        let menu = self.cpu.gpr[3];
        if self.map_settings_pane(menu) == 0 {
            return false;
        }
        self.w8(menu + MAP_PICKED, 0);
        if self.r8(menu + MAP_LOCKED) != 0 {
            return true;
        }
        self.call(MAP_CURSOR_SETUP.va(), &[menu]);
        let player = self.r32(MAP_PLAYER.va());
        let trigger = self.trigger(player);
        if trigger & 0x3 != 0 {
            // 0x2 up, else 0x1 down (the remote held sideways).
            let next = self.r32(menu + MAP_NEXT);
            self.w32(menu + MAP_NEXT, Self::map_step(next, trigger & 0x2 == 0));
            self.map_change_state(menu, MAP_STATE_BUTTON_CHANGE.va());
            return true;
        }
        let close = trigger & 0x10 != 0 || self.call(MAP_CLOSE_KEY.va(), &[0]) != 0;
        if close {
            self.map_change_state(menu, MAP_STATE_EXIT.va());
            return true;
        }
        let keys = self.r32(MAP_KEYS.va());
        let vtable = self.r32(keys);
        let pressed = self.r32(vtable + 0x20);
        if self.call(pressed, &[keys, 0x900]) != 0 {
            self.sound(SE_DECIDE);
            self.map_change_state(menu, MAP_STATE_HIT.va());
        }
        true
    }

    /// Hit: Settings' hit animation.
    fn map_hit(&mut self) -> bool {
        let menu = self.cpu.gpr[3];
        if self.r32(menu + MAP_NEXT) != MAP_SETTINGS {
            return false;
        }
        self.menu.map_settings = 0;
        self.call(ANIME_START.va(), &[menu + LAYOUT, Self::map_hit_anim(MAP_SETTINGS), 0]);
        true
    }

    /// Hit's wait for Settings: the menu goes out, the Settings window opens
    /// once it's gone, and when that has closed the menu opens again.
    fn map_hit_wait(&mut self) -> bool {
        let menu = self.cpu.gpr[3];
        if self.r32(menu + MAP_NEXT) != MAP_SETTINGS {
            return false;
        }
        let animating = self.map_animating(menu);
        match self.menu.map_settings {
            0 if !animating => {
                self.call(ALL_ANIME_END.va(), &[menu + LAYOUT]);
                self.call(ANIME_START.va(), &[menu + LAYOUT, MAP_OUT, 0]);
                self.menu.map_settings = 1;
            }
            1 if !animating => {
                self.map_show_window(menu, false);
                self.menu.map_settings = 2;
                self.menu.map_reopen = false;
                self.menu.map_open_requested = true;
            }
            2 if std::mem::take(&mut self.menu.map_reopen) => {
                self.menu.map_settings = 0;
                self.menu.map_back = true;
                self.map_change_state(menu, MAP_STATE_START_WAIT.va()); // its own opening, from the next frame
            }
            _ => {}
        }
        true
    }
}
