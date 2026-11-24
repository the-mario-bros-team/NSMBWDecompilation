//! The Practice tab's tools on the game thread (USA revision 1 addresses):
//!
//! - Restart Course: the course starts again from its start, as the game's
//!   own "restart this course" does (d_basesNP 0x809167D0): the restart
//!   scene's record (`dScRestartCrsin_c::m_startGameInfo`) is the course's
//!   start record (`dInfo_c::m_startGameInfo`) with the default entrance
//!   (0xFF) and area 0, then `dScStage_c::setNextScene(RESTART_CRSIN, 0,
//!   exit 2, fader 1)`. The checkpoint is cleared first
//!   (`dCyuukan_c::clear`), so the start is the course's own, and the
//!   players' power-ups go back to what they had when they came in from the
//!   map. No life is lost (no one died). Never in a net game.
//! - The level timer counts the frames the course's clock ran
//!   (`dStageTimer_c` steps 92 a frame while it runs), so it stops with the
//!   game: paused, at a miss, at the goal. It starts at each attempt (a new
//!   stage scene), and is split at the goal (a player in the goal demo's
//!   list, `daPyDemoMng_c` +0x20). Game frames, not wall time, so it is the
//!   same at any game speed or frame rate.
//! - The death counter counts lives lost (`daPyMng_c::mRest` going down) in
//!   a course: in this course (since it was entered from the map) and in
//!   this session. Kept per session, in memory only: it is a practice aid
//!   for the sitting at hand; per save file it would need its own file kept
//!   in step with the game's copies and erases, for little use.
//!
//! The counting only reads the game, so it runs in net games too; the
//! counts are this machine's view of the shared game.

use hle_os::System;

const SCENE: u32 = 0x8042_8730; // dScene_c::m_nextScene, m_nowScene (halfwords)
const SCENE_WORLD_MAP: u16 = 3;
const SCENE_STAGE: u16 = 5;
const SCENE_RESTART: u16 = 6; // RESTART_CRSIN
const SCENE_GAME_SETUP: u16 = 0xA;
const STAGE_TIMER: u32 = 0x8042_A070; // dStageTimer_c::m_instance; +4 the time (seconds << 12), +12 stopped (top byte)
const TIMER_STEP: i32 = 92;
const REST: u32 = 0x8035_4E90; // daPyMng_c::mRest[4], a count a character
const PLAYER_MODE: u32 = 0x8035_4E70; // daPyMng_c::mPlayerMode[4], each character's power-up
const CREATE_ITEM: u32 = 0x8035_4E80; // daPyMng_c::mCreateItem[4]
const DEMO_MANAGER: u32 = 0x8042_9C94; // daPyDemoMng_c::mspInstance (r13 - 0x59EC)
const GOAL_LIST: u32 = 0x20; // its players at the goal, 4 words, -1 for none
const INFO: u32 = 0x8042_9F7C; // dInfo_c::m_instance; its dCyuukan_c at +8
const CYUUKAN: u32 = 8;
const CYUUKAN_CLEAR: u32 = 0x8008_EDB0; // dCyuukan_c::clear
const START_GAME_INFO: u32 = 0x8031_5B90; // dInfo_c::m_startGameInfo
const RESTART_GAME_INFO: u32 = 0x8037_3D60; // dScRestartCrsin_c::m_startGameInfo
const SET_NEXT_SCENE: u32 = 0x8010_2260; // dScStage_c::setNextScene(scene, param, exit, fader)
const PROFILE_OPENING_TITLE: u16 = 0x2BB; // only on the title screen (its course is a stage too)

/// What the overlay shows, after each field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    /// In a course (not the title screen's).
    pub in_course: bool,
    /// Frames the course's clock has run in this attempt.
    pub frames: u32,
    /// Reached the goal: the time is the split.
    pub goal: bool,
    /// Lives lost in this course, and in this session.
    pub course_deaths: u32,
    pub deaths: u32,
}

#[derive(Default)]
pub struct Practice {
    pub stats: Stats,
    last_scene: u16,
    /// The world map (or the setup screens) since the last course: the next
    /// course is a new one.
    fresh: bool,
    /// The clock and its time after the last field, as the rules left it.
    timer: u32,
    time: i32,
    rest: Option<[u32; 4]>,
    /// The players' power-ups and Star when they came into this course.
    entry: Option<([u8; 16], [u8; 16])>,
}

fn read32(sys: &System, a: u32) -> Option<u32> {
    sys.mem.ram(a, 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

fn pointer(sys: &System, a: u32) -> Option<u32> {
    read32(sys, a).filter(|p| (0x8000_0000..0x8180_0000).contains(p))
}

fn now_scene(sys: &System) -> u16 {
    read32(sys, SCENE).map_or(0, |w| w as u16)
}

fn rests(sys: &System) -> Option<[u32; 4]> {
    let mut r = [0; 4];
    for (i, v) in r.iter_mut().enumerate() {
        *v = read32(sys, REST + 4 * i as u32)?;
    }
    Some(r)
}

impl Practice {
    /// After a field, before the Gameplay options fix anything up (Infinite
    /// Lives gives back a life it sees lost; this must see it first).
    /// Returns lines for the log: an attempt starting, the goal, a life lost.
    pub fn count(&mut self, sys: &System) -> Vec<String> {
        let mut log = Vec::new();
        let scene = now_scene(sys);
        let last = std::mem::replace(&mut self.last_scene, scene);
        if matches!(scene, SCENE_WORLD_MAP | SCENE_GAME_SETUP) {
            self.fresh = true;
        }
        if scene != SCENE_STAGE {
            self.stats.in_course = false;
            return log;
        }
        if last != SCENE_STAGE {
            // An attempt starts: the title screen's demo is no course.
            self.stats = Stats { in_course: !crate::gameplay::has_profile(sys, PROFILE_OPENING_TITLE), frames: 0, goal: false, ..self.stats };
            if self.fresh && self.stats.in_course {
                self.fresh = false;
                self.stats.course_deaths = 0;
                self.entry = sys.mem.ram(PLAYER_MODE, 16).zip(sys.mem.ram(CREATE_ITEM, 16)).map(|(m, c)| (m.try_into().unwrap(), c.try_into().unwrap()));
            }
            if self.stats.in_course {
                log.push(format!("practice: a course attempt starts (lives lost in this course {}, this session {})", self.stats.course_deaths, self.stats.deaths));
            }
        }
        if !self.stats.in_course {
            return log;
        }
        // The clock ran a step this field (not paused, not stopped at a miss
        // or the goal; the goal's count-down into the score is no frame).
        let timer = pointer(sys, STAGE_TIMER);
        if let Some(t) = timer.filter(|&t| t == self.timer) {
            let now = read32(sys, t + 4).map_or(self.time, |v| v as i32);
            if (1..=4 * TIMER_STEP).contains(&(self.time - now)) && !self.stats.goal {
                self.stats.frames += 1;
            }
        }
        if !self.stats.goal {
            let manager = pointer(sys, DEMO_MANAGER);
            self.stats.goal = manager.is_some_and(|m| (0..4).any(|i| read32(sys, m + GOAL_LIST + 4 * i).is_some_and(|p| p != u32::MAX)));
            if self.stats.goal {
                log.push(format!("practice: the goal, split {} ({} frames)", time_label(self.stats.frames), self.stats.frames));
            }
        }
        if let (Some(before), Some(now)) = (self.rest, rests(sys)) {
            let lost: u32 = before.iter().zip(now).map(|(&b, n)| b.saturating_sub(n).min(1)).sum();
            self.stats.course_deaths += lost;
            self.stats.deaths += lost;
            if lost > 0 {
                log.push(format!("practice: a life lost at {} (this course {}, this session {})", time_label(self.stats.frames), self.stats.course_deaths, self.stats.deaths));
            }
        }
        log
    }

    /// After the Gameplay options: the clock and lives as they leave them.
    pub fn settle(&mut self, sys: &System) {
        let in_course = self.stats.in_course && now_scene(sys) == SCENE_STAGE;
        self.rest = if in_course { rests(sys) } else { None };
        self.timer = if in_course { pointer(sys, STAGE_TIMER).unwrap_or(0) } else { 0 };
        if self.timer != 0 {
            self.time = read32(sys, self.timer + 4).map_or(0, |v| v as i32);
        }
    }

    /// Restart Course (not in a net game: the caller checks). Says what
    /// happened.
    pub fn restart(&mut self, sys: &mut System) -> String {
        let Some(w) = read32(sys, SCENE) else { return "restart: no game yet".into() };
        let (next, now) = ((w >> 16) as u16, w as u16);
        if now != SCENE_STAGE || !self.stats.in_course {
            return "restart: not in a course".into();
        }
        if next != now {
            return "restart: the game is already changing scenes".into();
        }
        if !sys.pause_window_closed() {
            return "restart: close the pause menu first".into();
        }
        let Some(info) = pointer(sys, INFO) else { return "restart: no dInfo_c".into() };
        let Some(start) = sys.mem.ram(START_GAME_INFO, 16).map(<[u8]>::to_vec) else { return "restart: no start record".into() };
        if let Err(e) = sys.call_guest(CYUUKAN_CLEAR, &[info + CYUUKAN], &[]) {
            return format!("restart: {e}");
        }
        if let Some((modes, items)) = self.entry {
            if let Some(b) = sys.mem.ram_mut(PLAYER_MODE, 16) {
                b.copy_from_slice(&modes);
            }
            if let Some(b) = sys.mem.ram_mut(CREATE_ITEM, 16) {
                b.copy_from_slice(&items);
            }
        }
        // The record as the game's own restart writes it.
        let mut record = [0u8; 16];
        record[0..4].copy_from_slice(&start[0..4]);
        record[4] = start[4];
        record[5] = 0xFF;
        record[6] = 0;
        record[7] = 1;
        record[8..12].copy_from_slice(&1u32.to_be_bytes());
        record[12..16].copy_from_slice(&start[12..16]);
        let Some(b) = sys.mem.ram_mut(RESTART_GAME_INFO, 16) else { return "restart: no restart record".into() };
        b.copy_from_slice(&record);
        match sys.call_guest(SET_NEXT_SCENE, &[u32::from(SCENE_RESTART), 0, 2, 1], &[]) {
            Ok(_) => format!("restart: course {:02}-{:02} from its start", start[12] + 1, start[13] + 1),
            Err(e) => format!("restart: {e}"),
        }
    }
}

/// What the Practice tab's overlays show this frame.
pub struct View<'a> {
    pub stats: Stats,
    /// The game speed now (percent, 0 unlimited) and whether Fast Forward
    /// is held.
    pub speed: u32,
    pub fast_forward: bool,
    pub timer: bool,
    pub deaths: bool,
    /// 0 off, 1 player 1, 2 every player with a remote on.
    pub input_display: u32,
    pub inputs: &'a [hle_ios::wiimote::Input],
}

/// The Practice tab's overlays, in a `width` x `height` window with pixel
/// size `px`: the speed, the level timer and the death counter at the
/// bottom right, the input display at the bottom left.
pub fn draw(out: &mut Vec<vk_render::OverlayRect>, width: i32, height: i32, px: i32, v: &View) {
    use crate::overlay::{self, GREEN, GREY, WHITE, YELLOW};
    let margin = 4 * px;
    let mut lines = Vec::new();
    if v.speed != 100 {
        let speed = if v.speed == 0 { "UNLIMITED".to_string() } else { format!("{}%", v.speed) };
        lines.push((YELLOW, if v.fast_forward { format!("FAST FORWARD {speed}") } else { format!("SPEED {speed}") }));
    }
    if v.timer && v.stats.in_course {
        let (color, word) = if v.stats.goal { (GREEN, "GOAL") } else { (WHITE, "TIME") };
        lines.push((color, format!("{word} {}  {}F", time_label(v.stats.frames), v.stats.frames)));
    }
    if v.deaths && v.stats.in_course {
        lines.push((WHITE, format!("DEATHS {}  SESSION {}", v.stats.course_deaths, v.stats.deaths)));
    }
    if !lines.is_empty() {
        let w = lines.iter().map(|(_, l)| l.chars().count() as i32).max().unwrap_or(0) * 6 * px + 4 * px;
        let h = lines.len() as i32 * 9 * px + 3 * px;
        overlay::panel(out, (width - w - margin).max(0), (height - h - margin).max(0), px, &lines);
    }
    if v.input_display == 0 {
        return;
    }
    // A row a player: the D-pad as the screen's directions (the remote is
    // held sideways), then 2, 1, A, +, -, the shake and the tilt.
    let players: Vec<usize> = (0..v.inputs.len()).filter(|&i| if v.input_display == 1 { i == 0 } else { v.inputs[i].connected }).collect();
    let u = 8 * px;
    let row = 3 * u + 2 * px;
    let w = 2 * px + 3 * 6 * px + 3 * u + 2 * u + 5 * 2 * u + 6 * 6 * px + 5 * u + 2 * px;
    let top = height - margin - row * players.len() as i32 - 2 * px;
    out.push(vk_render::OverlayRect { x: margin, y: top, w, h: row * players.len() as i32 + 2 * px, color: overlay::BACKDROP });
    for (n, &i) in players.iter().enumerate() {
        let input = v.inputs[i];
        let held = |b: u16| input.buttons & b != 0;
        let color = |on: bool| if on { GREEN } else { GREY };
        let y = top + 2 * px + n as i32 * row;
        let mut x = margin + 2 * px;
        x += overlay::text(out, x, y + u, px, WHITE, &format!("P{}", i + 1)) + 6 * px;
        use hle_ios::wiimote::button;
        let cell = |out: &mut Vec<vk_render::OverlayRect>, cx: i32, cy: i32, on: bool| out.push(vk_render::OverlayRect { x: cx + px / 2, y: cy + px / 2, w: u - px, h: u - px, color: color(on) });
        cell(out, x + u, y, held(button::RIGHT));
        cell(out, x, y + u, held(button::UP));
        cell(out, x + 2 * u, y + u, held(button::DOWN));
        cell(out, x + u, y + 2 * u, held(button::LEFT));
        x += 3 * u + 2 * u;
        for (label, b) in [("2", button::TWO), ("1", button::ONE), ("A", button::A), ("+", button::PLUS), ("-", button::MINUS)] {
            overlay::text(out, x, y + u, px, color(held(b)), label);
            x += 2 * u;
        }
        x += overlay::text(out, x, y + u, px, color(input.shake), "SHAKE") + 6 * px;
        // The tilt: a bar with its middle marked and a block where it leans.
        let bar = 5 * u;
        out.push(vk_render::OverlayRect { x, y: y + u + 3 * px, w: bar, h: px, color: GREY });
        out.push(vk_render::OverlayRect { x: x + bar / 2, y: y + u, w: px, h: 7 * px, color: GREY });
        let at = x + bar / 2 + (input.tilt.clamp(-1.0, 1.0) * (bar / 2 - 2 * px) as f32) as i32;
        out.push(vk_render::OverlayRect { x: at - px, y: y + u + px, w: 3 * px, h: 5 * px, color: color(input.tilt.abs() > 0.05) });
    }
}

/// The time of `frames` game frames (59.94 a second): "1:05.43".
pub fn time_label(frames: u32) -> String {
    let hundredths = u64::from(frames) * 100 * 1001 / 60_000;
    format!("{}:{:02}.{:02}", hundredths / 6000, hundredths / 100 % 60, hundredths % 100)
}

#[cfg(test)]
mod tests {
    #[test]
    fn times_are_game_frames() {
        assert_eq!(super::time_label(0), "0:00.00");
        assert_eq!(super::time_label(60), "0:01.00");
        assert_eq!(super::time_label(3600), "1:00.06");
        assert_eq!(super::time_label(59_940), "16:40.00");
    }
}
