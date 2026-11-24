//! The Practice tab's tools on the game thread (USA revision 1 addresses):
//!
//! - Restart Course: the course starts again from its start through the
//!   game's restart scene, as d_basesNP 0x809167D0 does: the restart scene's
//!   record (`dScRestartCrsin_c::m_startGameInfo`) is set, then
//!   `dScStage_c::setNextScene(RESTART_CRSIN, 0, exit 2, fader 1)`. That
//!   function restarts a course as a demo (the record's +7, `mIsDemo`, is 1,
//!   and +5/+6 the default entrance 0xFF and area 0); here the record is the
//!   course's own start record (`dInfo_c::m_startGameInfo`), as the course
//!   was entered, so the players play it. The checkpoint is cleared first
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
//!   a course: in this course (since another course was last played; a miss
//!   goes back to the world map, and the course again is the same course)
//!   and in this session. Kept per session, in memory only: it is a practice aid
//!   for the sitting at hand; per save file it would need its own file kept
//!   in step with the game's copies and erases, for little use.
//!
//! The counting only reads the game, so it runs in net games too; the
//! counts are this machine's view of the shared game.

use game_version::E1;
use hle_os::System;

const SCENE: E1 = E1(0x8042_8730); // dScene_c::m_nextScene, m_nowScene (halfwords)
const SCENE_WORLD_MAP: u16 = 3;
const SCENE_STAGE: u16 = 5;
const SCENE_RESTART: u16 = 6; // RESTART_CRSIN
const SCENE_GAME_SETUP: u16 = 0xA;
/// `m_nextScene` with no change pending (`dScene_c::setNextScene` checks for it).
const NO_SCENE: u16 = 0x2EE;
const STAGE_TIMER: E1 = E1(0x8042_A070); // dStageTimer_c::m_instance; +4 the time (seconds << 12), +12 stopped (top byte)
const TIMER_STEP: i32 = 92;
const REST: E1 = E1(0x8035_4E90); // daPyMng_c::mRest[4], a count a character
const PLAYER_MODE: E1 = E1(0x8035_4E70); // daPyMng_c::mPlayerMode[4], each character's power-up
const CREATE_ITEM: E1 = E1(0x8035_4E80); // daPyMng_c::mCreateItem[4]
const DEMO_MANAGER: E1 = E1(0x8042_9C94); // daPyDemoMng_c::mspInstance (r13 - 0x59EC)
const GOAL_LIST: u32 = 0x20; // its players at the goal, 4 words, -1 for none
const INFO: E1 = E1(0x8042_9F7C); // dInfo_c::m_instance; its dCyuukan_c at +8
const CYUUKAN: u32 = 8;
const CYUUKAN_CLEAR: E1 = E1(0x8008_EDB0); // dCyuukan_c::clear
const START_GAME_INFO: E1 = E1(0x8031_5B90); // dInfo_c::m_startGameInfo
const RESTART_GAME_INFO: E1 = E1(0x8037_3D60); // dScRestartCrsin_c::m_startGameInfo
const SET_NEXT_SCENE: E1 = E1(0x8010_2260); // dScStage_c::setNextScene(scene, param, exit, fader)
const PROFILE_OPENING_TITLE: u16 = 0x2BB; // only on the title screen (its course is a stage too)
/// Fields the title screen's object is looked for at an attempt's start.
const TITLE_CHECK: u32 = 60;

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
    /// The world map (or the setup screens) since the last course: the
    /// players' power-ups are taken anew at the next.
    fresh: bool,
    /// The course (world, level) last played, for its deaths.
    course: Option<(u8, u8)>,
    /// Fields into this attempt while it may still turn out to be the title
    /// screen's demo (its object comes a few fields in).
    checking: Option<u32>,
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
    read32(sys, SCENE.va()).map_or(0, |w| w as u16)
}

fn rests(sys: &System) -> Option<[u32; 4]> {
    let mut r = [0; 4];
    for (i, v) in r.iter_mut().enumerate() {
        *v = read32(sys, REST.va() + 4 * i as u32)?;
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
            // An attempt starts (taken as a course until the title screen's
            // demo shows itself).
            self.stats = Stats { in_course: true, frames: 0, goal: false, ..self.stats };
            self.checking = Some(0);
            self.attempt_start(sys);
        }
        if let Some(n) = self.checking {
            if crate::gameplay::has_profile(sys, PROFILE_OPENING_TITLE) {
                self.checking = None;
                self.stats.in_course = false;
                self.fresh = true;
            } else if n >= TITLE_CHECK {
                self.checking = None;
                log.push(format!("practice: a course attempt started (lives lost in this course {}, this session {})", self.stats.course_deaths, self.stats.deaths));
            } else {
                self.checking = Some(n + 1);
            }
        }
        if !self.stats.in_course {
            return log;
        }
        // The clock ran a step this field (not paused, not stopped at a miss
        // or the goal; the goal's count-down into the score is no frame).
        let timer = pointer(sys, STAGE_TIMER.va());
        if let Some(t) = timer.filter(|&t| t == self.timer) {
            let now = read32(sys, t + 4).map_or(self.time, |v| v as i32);
            if (1..=4 * TIMER_STEP).contains(&(self.time - now)) && !self.stats.goal {
                self.stats.frames += 1;
            }
        }
        if !self.stats.goal {
            let manager = pointer(sys, DEMO_MANAGER.va());
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

    /// A new attempt: another course's deaths start at 0, and coming from
    /// the map the players' power-ups are noted for Restart Course.
    fn attempt_start(&mut self, sys: &System) {
        let course = sys.mem.ram(START_GAME_INFO.va() + 12, 2).map(|b| (b[0], b[1]));
        if course != self.course {
            self.course = course;
            self.stats.course_deaths = 0;
        }
        if std::mem::take(&mut self.fresh) {
            self.entry = sys.mem.ram(PLAYER_MODE.va(), 16).zip(sys.mem.ram(CREATE_ITEM.va(), 16)).map(|(m, c)| (m.try_into().unwrap(), c.try_into().unwrap()));
        }
    }

    /// After the Gameplay options: the clock and lives as they leave them.
    pub fn settle(&mut self, sys: &System) {
        let in_course = self.stats.in_course && now_scene(sys) == SCENE_STAGE;
        self.rest = if in_course { rests(sys) } else { None };
        self.timer = if in_course { pointer(sys, STAGE_TIMER.va()).unwrap_or(0) } else { 0 };
        if self.timer != 0 {
            self.time = read32(sys, self.timer + 4).map_or(0, |v| v as i32);
        }
    }

    /// Restart Course (not in a net game: the caller checks). Says what
    /// happened.
    pub fn restart(&mut self, sys: &mut System) -> String {
        let Some(w) = read32(sys, SCENE.va()) else { return "restart: no game yet".into() };
        let (next, now) = ((w >> 16) as u16, w as u16);
        if now != SCENE_STAGE || !self.stats.in_course {
            return "restart: not in a course".into();
        }
        if next != NO_SCENE {
            return format!("restart: the game is already changing scenes (to {next})");
        }
        if !sys.pause_window_closed() {
            return "restart: close the pause menu first".into();
        }
        let Some(info) = pointer(sys, INFO.va()) else { return "restart: no dInfo_c".into() };
        let Some(start) = sys.mem.ram(START_GAME_INFO.va(), 16).map(<[u8]>::to_vec) else { return "restart: no start record".into() };
        if let Err(e) = sys.call_guest(CYUUKAN_CLEAR.va(), &[info + CYUUKAN], &[]) {
            return format!("restart: {e}");
        }
        if let Some((modes, items)) = self.entry {
            if let Some(b) = sys.mem.ram_mut(PLAYER_MODE.va(), 16) {
                b.copy_from_slice(&modes);
            }
            if let Some(b) = sys.mem.ram_mut(CREATE_ITEM.va(), 16) {
                b.copy_from_slice(&items);
            }
        }
        // The course's start record as it was entered (not a demo: +7 is
        // `mIsDemo`).
        let Some(b) = sys.mem.ram_mut(RESTART_GAME_INFO.va(), 16) else { return "restart: no restart record".into() };
        b.copy_from_slice(&start);
        match sys.call_guest(SET_NEXT_SCENE.va(), &[u32::from(SCENE_RESTART), 0, 2, 1], &[]) {
            Ok(_) => format!("restart: course {:02}-{:02} from its start (record {})", start[12] + 1, start[13] + 1, start.iter().map(|b| format!("{b:02x}")).collect::<String>()),
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
        // 60000/1001 frames a second: 60,000 frames are 1001 s.
        assert_eq!(super::time_label(60_000), "16:41.00");
    }
}
