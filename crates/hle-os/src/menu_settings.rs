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
//! Each hook acts only while the live layout has its button (so never in a
//! net game, whose archives are not patched); otherwise it declines and the
//! game's function runs as it is. A hook calls the game's functions with
//! [`System::call_nested`].

use guest_cpu::{MSR_EE, MSR_FP};

use crate::System;

/// `LytBase_c::GroupRegister(names, animations, count)`.
const GROUP_REGISTER: u32 = 0x800C_90F0;
const PAUSE_CURSOR_SETUP: u32 = 0x8015_AC30;
const PAUSE_OPEN: u32 = 0x8015_AD80;
const PAUSE_BUTTON_CHANGE: u32 = 0x8015_AE70;
const PAUSE_HIT: u32 = 0x8015_B030;
/// `PauseManager_c::SelectSoundSet`, the cursor's move.
const PAUSE_CURSOR_MOVE: u32 = 0x800D_0C00;
const PAUSE_KEY_CHECK: u32 = 0x800D_0CB0;
const MAP_CURSOR_SETUP: u32 = 0x8077_AD70;
const MAP_START_WAIT: u32 = 0x8077_ADC0;
const MAP_OPEN_END: u32 = 0x8077_AEF0;
const MAP_BUTTON_CHANGE: u32 = 0x8077_AF10;
const MAP_INPUT: u32 = 0x8077_B010;
const MAP_HIT: u32 = 0x8077_B1A0;
const MAP_HIT_WAIT: u32 = 0x8077_B1C0;

/// The hooked functions, for [`crate::SPECIAL`].
pub const HOOKS: [u32; 14] = [
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
];

/// The map menu's functions are in a module: their first words, checked
/// before a hook acts.
const MAP_WORDS: [(u32, u32); 7] = [
    (MAP_CURSOR_SETUP, 0x8803_0274),
    (MAP_START_WAIT, 0x9421_FFF0),
    (MAP_OPEN_END, 0x3880_0000),
    (MAP_BUTTON_CHANGE, 0x9421_FFF0),
    (MAP_INPUT, 0x9421_FFE0),
    (MAP_HIT, 0x8083_0268),
    (MAP_HIT_WAIT, 0x9421_FFF0),
];

const PAUSEWINDOW: u32 = 0x8042_A33C; // Pausewindow_c::m_instance
const GAME_KEY: u32 = 0x8042_9F50; // dGameKey_c::m_instance
const GAME_STOP: u32 = 0x8042_9F48; // dGameCom::l_gameStop
const SND_AUDIO_MGR: u32 = 0x8042_A488; // SndAudioMgr::sInstance
const START_GAME_INFO: u32 = 0x8031_5B90; // dInfo_c::m_startGameInfo, +0xD the level
const START_SYSTEM_SE: u32 = 0x8019_5380;
const IS_NOW_COURSE_CLEAR: u32 = 0x800B_4DE0;
const SELECT_CURSOR_SETUP: u32 = 0x800B_44D0; // dGameCom::SelectCursorSetup(pane, slot, bool)
const ANIME_START: u32 = 0x800C_92F0;
const REVERSE_ANIME_START: u32 = 0x800C_93D0;
const ALL_ANIME_END: u32 = 0x800C_9500;

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
const MAP_STATE_START_WAIT: u32 = 0x8099_15F0;
const MAP_STATE_BUTTON_CHANGE: u32 = 0x8099_1670;
const MAP_STATE_HIT: u32 = 0x8099_16F0;
const MAP_STATE_EXIT: u32 = 0x8099_1730;
const MAP_STATE_OPEN: u32 = 0x8099_1630;
/// Its create's tables: the group names (0x80941288 + 404) and their
/// animations.
const MAP_GROUP_NAMES: u32 = 0x8094_1288 + 404;
const MAP_GROUP_ANIMS: u32 = 0x8093_3FD8;
const MAP_CLOSE_KEY: u32 = 0x800B_53A0; // dGameCom::chkCancelButton
const MAP_KEYS: u32 = 0x8042_A468; // its vtable's +0x20: was one of these pressed
const MAP_PLAYER: u32 = 0x8042_A464; // the player the menu reads
const MAP_IN: u32 = 0;
const MAP_OUT: u32 = 18;
const MAP_GROUPS: u32 = 19;
const MAP_SETTINGS: u32 = 4;
const SE_CURSOR: u32 = 0x78;
const SE_DECIDE: u32 = 0x79;

/// Where nested calls return to; never executed (the call stops there).
const NESTED_RETURN: u32 = 0x8000_1800;
/// A nested call that runs longer than this is cut off.
const NESTED_LIMIT: u64 = 20_000_000;

fn guest_ram(a: u32) -> bool {
    (0x8000_0000..0x8180_0000).contains(&a) || (0x9000_0000..0x9400_0000).contains(&a)
}

/// The Settings buttons' state on the host.
#[derive(Default)]
pub struct MenuSettings {
    /// The pause window and the world map's menu were given their button
    /// ([`crate::menu_layout::patch`]).
    pub buttons: (bool, bool),
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
    /// A hooked function the next call runs as the game has it.
    bypass: Option<u32>,
    /// Main.dol's copies of the groups' names (0: not found yet).
    group_names: Option<(u32, u32)>,
}

impl MenuSettings {
    /// A Settings button was picked since the last call: true once its
    /// menu is gone, for the Settings window to open.
    pub fn take_open(&mut self) -> bool {
        std::mem::take(&mut self.open_requested) | std::mem::take(&mut self.map_open_requested)
    }

    /// The pause window's Settings was picked, waiting for it to close.
    pub fn waiting_for_pause(&self) -> bool {
        self.open_requested
    }

    /// The Settings window opened from a game menu has closed: that menu
    /// opens again, on Settings.
    pub fn reopen(&mut self) {
        if self.map_settings == 2 {
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
            if crate::may_be_special(pc) {
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
        let r = self.cpu.gpr[3];
        let cycles = self.cpu.cycles;
        self.cpu = saved;
        self.cpu.cycles = cycles;
        r
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

    fn r8(&mut self, a: u32) -> u8 {
        self.mem.read_u8(a).unwrap_or(0)
    }
    fn r32(&mut self, a: u32) -> u32 {
        self.mem.read_u32(a).unwrap_or(0)
    }
    fn w8(&mut self, a: u32, v: u8) {
        let _ = self.mem.write_u8(a, v);
    }
    fn w32(&mut self, a: u32, v: u32) {
        let _ = self.mem.write_u32(a, v);
    }

    /// The menu hook at `pc`, if it applies: the function's work done and
    /// the CPU at its return address. False to run the game's function.
    pub(crate) fn menu_hook(&mut self, pc: u32) -> bool {
        if self.menu.bypass == Some(pc) {
            self.menu.bypass = None;
            return false;
        }
        if let Some(&(_, word)) = MAP_WORDS.iter().find(|w| w.0 == pc) {
            if self.r32(pc) != word {
                return false;
            }
        }
        let ran = match pc {
            GROUP_REGISTER => self.group_register(),
            PAUSE_CURSOR_SETUP => self.pause_cursor_setup(),
            PAUSE_OPEN => self.pause_open(),
            PAUSE_BUTTON_CHANGE => self.pause_button_change(),
            PAUSE_HIT => self.pause_hit(),
            PAUSE_CURSOR_MOVE => self.pause_cursor_move(),
            PAUSE_KEY_CHECK => self.pause_key_check(),
            MAP_CURSOR_SETUP => self.map_cursor_setup(),
            MAP_START_WAIT => self.map_start_wait(),
            MAP_OPEN_END => self.map_open_end(),
            MAP_BUTTON_CHANGE => self.map_button_change(),
            MAP_INPUT => self.map_input(),
            MAP_HIT => self.map_hit(),
            MAP_HIT_WAIT => self.map_hit_wait(),
            _ => false,
        };
        if ran {
            self.cpu.pc = self.cpu.lr;
        }
        ran
    }

    // ------------------------------------------------------------ layouts

    fn pane_name(&mut self, pane: u32) -> String {
        let mut s = String::new();
        for i in 0..16 {
            match self.r8(pane + 0xBC + i) {
                0 => break,
                c => s.push(c as char),
            }
        }
        s
    }

    /// nw4r::lyt::Pane `name` under `pane`: children at +0x10 (the list's
    /// terminator at +0x14), each linked at +4.
    fn find_pane(&mut self, pane: u32, name: &str, depth: u32) -> u32 {
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

    /// The root pane a layout object at `object` points to.
    fn root_pane(&mut self, object: u32) -> u32 {
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
        let Some(ram) = self.mem.ram(0x802E_0000, 0x6_0000) else { return 0 };
        let mut needle = name.as_bytes().to_vec();
        needle.push(0);
        (1..ram.len().saturating_sub(needle.len()))
            .find(|&i| ram[i - 1] == 0 && ram[i..i + needle.len()] == needle[..])
            .map_or(0, |i| 0x802E_0000 + i as u32)
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
        let sound = self.r32(SND_AUDIO_MGR);
        if sound != 0 {
            self.call(START_SYSTEM_SE, &[sound, id, 1]);
        }
    }

    /// The key trigger bits (WPAD buttons) of `player`.
    fn trigger(&mut self, player: u32) -> u32 {
        let keys = self.r32(GAME_KEY);
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
        let window = self.r32(PAUSEWINDOW);
        let (group, settings_anims) = if count == WINDOW_GROUPS && guest_ram(window) && layout == window + LAYOUT {
            // onButton, idleButton, offButon (sic), hitButton.
            (pause_group, [3, 4, 5, 6])
        } else if count == MAP_GROUPS && names == MAP_GROUP_NAMES {
            // Copies of Quick Save's on, idle, hit and off.
            let a = [4, 8, 12, 16].map(|g| self.r32(MAP_GROUP_ANIMS + g * 4));
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
        let frame = (sp - 0x100) & !0xF;
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
        self.call_original(GROUP_REGISTER, &[layout, new_names, new_anims, count + 4], frame);
        true
    }

    // ------------------------------------------------------------ the pause window

    fn pausewindow(&mut self) -> u32 {
        let w = self.r32(PAUSEWINDOW);
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
            self.call(SELECT_CURSOR_SETUP, &[pane, 0, 0]);
        }
        true
    }

    fn pause_open(&mut self) -> bool {
        let window = self.cpu.gpr[3];
        if self.pause_settings_pane(window) == 0 {
            return false;
        }
        let sp = self.cpu.gpr[1];
        self.call_original(PAUSE_OPEN, &[window], sp);
        // The Settings button starts plain like the others.
        self.call(REVERSE_ANIME_START, &[window + LAYOUT, ON[2], 0]);
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
        self.call(ALL_ANIME_END, &[window + LAYOUT]);
        self.w8(window + IS_ANIMATING, 1);
        if active >= 0 {
            self.call(ANIME_START, &[window + LAYOUT, OFF[(active as usize).min(2)], 0]);
        }
        self.w32(window + ACTIVE_BUTTON, next);
        self.call(ANIME_START, &[window + LAYOUT, ON[(next as usize).min(2)], 0]);
        true
    }

    fn pause_hit(&mut self) -> bool {
        let window = self.cpu.gpr[3];
        if self.r32(window + NEXT_BUTTON) != SETTINGS {
            return false;
        }
        self.w8(window + IS_ANIMATING, 1);
        self.call(ANIME_START, &[window + LAYOUT, HIT[2], 0]);
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
            // cannon course), and never in the map enemies' ambushes (levels
            // 0x20-0x22), whose window doesn't move in the game either.
            let level = self.r8(START_GAME_INFO + 0xD);
            let cleared = self.call(IS_NOW_COURSE_CLEAR, &[]) != 0;
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
        let still = self.r32(GAME_STOP) & 0xFFFF_FFFA == 0 && self.r8(window + IS_ANIMATING) == 0 && self.r8(window + IS_ACTIVE) != 0;
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
            self.call(SELECT_CURSOR_SETUP, &[pane, 4, 0]); // the menu's cursor slot
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
        self.call(ALL_ANIME_END, &[layout]);
        for button in 0..3 {
            self.call(REVERSE_ANIME_START, &[layout, Self::map_on(button), 0]);
        }
        self.call(REVERSE_ANIME_START, &[layout, Self::map_on(MAP_SETTINGS), 0]);
        self.call(ANIME_START, &[layout, MAP_IN, 0]);
        self.w8(menu + MAP_PICKED, 0);
        self.map_show_window(menu, true);
        self.map_change_state(menu, MAP_STATE_OPEN);
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
        self.call(ALL_ANIME_END, &[layout]);
        if active >= 0 {
            self.sound(SE_CURSOR);
            self.call(ANIME_START, &[layout, Self::map_off(active as u32), 0]);
        }
        self.w32(menu + MAP_ACTIVE, next);
        self.call(ANIME_START, &[layout, Self::map_on(next), 0]);
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
        self.call(MAP_CURSOR_SETUP, &[menu]);
        let player = self.r32(MAP_PLAYER);
        let trigger = self.trigger(player);
        if trigger & 0x3 != 0 {
            // 0x2 up, else 0x1 down (the remote held sideways).
            let next = self.r32(menu + MAP_NEXT);
            self.w32(menu + MAP_NEXT, Self::map_step(next, trigger & 0x2 == 0));
            self.map_change_state(menu, MAP_STATE_BUTTON_CHANGE);
            return true;
        }
        let close = trigger & 0x10 != 0 || self.call(MAP_CLOSE_KEY, &[0]) != 0;
        if close {
            self.map_change_state(menu, MAP_STATE_EXIT);
            return true;
        }
        let keys = self.r32(MAP_KEYS);
        let vtable = self.r32(keys);
        let pressed = self.r32(vtable + 0x20);
        if self.call(pressed, &[keys, 0x900]) != 0 {
            self.sound(SE_DECIDE);
            self.map_change_state(menu, MAP_STATE_HIT);
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
        self.call(ANIME_START, &[menu + LAYOUT, Self::map_hit_anim(MAP_SETTINGS), 0]);
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
                self.call(ALL_ANIME_END, &[menu + LAYOUT]);
                self.call(ANIME_START, &[menu + LAYOUT, MAP_OUT, 0]);
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
                self.map_change_state(menu, MAP_STATE_START_WAIT); // its own opening, from the next frame
            }
            _ => {}
        }
        true
    }
}
