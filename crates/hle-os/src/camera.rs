//! The shared camera for online play (docs/PORT_PLAN.md, section 12).
//!
//! In a course the game's camera follows the players and pushes, then
//! bubbles, a player it cannot keep on screen. Online, each PC shows its own
//! player, so the game's camera becomes a box around every player instead:
//! actors near any player stay active and nobody is pushed. Each PC then
//! draws a window the game camera's own size around its player
//! ([`gx::xf::View`]); only that projection differs between PCs, never
//! guest state.
//!
//! The box is written at the end of `dBgGm_c::execute` (which runs the
//! game's camera, `dBg_c::calcScroll`) and the game's own values are put
//! back at its start, so the game's camera logic always works from its own
//! state while everything else (actor activity, the players, drawing) sees
//! the box.
//!
//! The box is used only with two or more players, in zones whose camera
//! mode is 0 (follow) and without auto-scroll or a locked camera. Boss
//! rooms, auto-scrolling zones and other camera modes keep the game's rules.

use guest_mem::snap::{Snap, Snapshot};
use game_version::E1;
use guest_mem::Mem;

/// `dBgGm_c::execute`: its first instruction and its only `blr`.
pub const BG_EXECUTE: E1 = E1(0x8015_4F60);
pub const BG_EXECUTE_END: E1 = E1(0x8015_6814);

/// `_SDA_BASE_` (r13) of USA revision 1.
const SDA: u32 = 0x8042_F680;
/// `dBg_c::m_bg_p` and `dBgParameter_c::ms_Instance_p`.
const BG_PTR: E1 = E1(SDA - 0x58B0);
const PARAM_PTR: E1 = E1(SDA - 0x5880);
/// The camera's width at zoom 1 (408.7; 224 high).
const BASE_W: E1 = E1(SDA - 0x5594);
/// The game mode: 0 in a course played normally, 2 in the title's demo.
const GAME_MODE: E1 = E1(SDA - 0x547C);

// `dBg_c` fields (y up).
const BG_LEFT: u32 = 0x8FE88;
const BG_TOP: u32 = 0x8FE8C;
const BG_W: u32 = 0x8FE78;
const BG_H: u32 = 0x8FE7C;
/// Copies of left and top kept outside auto-scroll.
const BG_LEFT2: u32 = 0x900AC;
const BG_TOP2: u32 = 0x900B0;
/// Zoom (size = base / zoom) and its two copies.
const BG_ZOOM: [u32; 3] = [0x9000C, 0x90088, 0x9009C];
/// The furthest zoom-out: actors stay active within a screen of
/// base / this around the camera's centre (`0x800B56E0`).
const BG_ZOOM_OUT: u32 = 0x90028;
/// The zone: left, right, top, bottom.
const ZONE: [u32; 4] = [0x90030, 0x90034, 0x90038, 0x9003C];
/// Camera mode (0: follow), auto-scroll on, camera calculation off, locked.
const BG_MODE: u32 = 0x900FE;
const BG_AUTOSCROLL: u32 = 0x900C6;
const BG_NO_CALC: u32 = 0x909CA;
const BG_LOCKED: u32 = 0x90117;

// `dBgParameter_c` fields: left, top, width, height, centre x and y, and a
// second copy of left and top.
const P_LEFT: u32 = 0x38;
const P_TOP: u32 = 0x3C;
const P_W: u32 = 0x40;
const P_H: u32 = 0x44;
const P_CX: u32 = 0x48;
const P_CY: u32 = 0x4C;
const P_LEFT2: u32 = 0x60;
const P_TOP2: u32 = 0x64;

fn read32(mem: &Mem, a: u32) -> Option<u32> {
    mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()))
}

fn readf(mem: &Mem, a: u32) -> Option<f32> {
    read32(mem, a).map(f32::from_bits).filter(|v| v.is_finite())
}

/// Player `i`'s position (0-based): `daPyMng_c`'s player IDs (0x80354E10,
/// one word per player), looked up as `fManager_c::searchBaseByID` does
/// (lists at 0x80377A70 by ID & 7, nodes linked at +4 with their `fBase_c`
/// at +8, its ID at +0), then the actor's position (`dBaseActor_c`, +0xAC).
pub fn player_position(mem: &Mem, i: usize) -> Option<[f32; 2]> {
    let base = player_actor(mem, i)?;
    Some([readf(mem, base + 0xAC)?, readf(mem, base + 0xB0)?])
}

/// Player `i`'s actor (see [`player_position`]).
pub fn player_actor(mem: &Mem, i: usize) -> Option<u32> {
    let id = read32(mem, E1(0x8035_4E10).va() + 4 * i as u32)?;
    if id == 0 {
        return None;
    }
    let mut node = read32(mem, E1(0x8037_7A70).va() + 8 * (id & 7))?;
    for _ in 0..10_000 {
        if node == 0 {
            return None;
        }
        let base = read32(mem, node + 8)?;
        if read32(mem, base)? == id {
            return Some(base);
        }
        node = read32(mem, node + 4)?;
    }
    None
}

/// The players the game's camera follows, as `daPyMng_c::getActScrollInfo`
/// (0x8005FC70) picks them: in the game (0x80354E50, one word per player)
/// and not in a bubble (actor +0x153C is 1 there). A player in a bubble is
/// placed by the camera, so the box must not follow it.
pub fn followed_players(mem: &Mem) -> Vec<[f32; 2]> {
    (0..4)
        .filter(|&i| read32(mem, E1(0x8035_4E50).va() + 4 * i as u32).is_some_and(|v| v != 0))
        .filter_map(|i| {
            let base = player_actor(mem, i)?;
            (mem.ram(base + 0x153C, 1)?[0] != 1).then(|| player_position(mem, i)).flatten()
        })
        .collect()
}

/// The bottom of the current zone (y up), where a player waiting to come
/// back in a bubble is out of sight.
pub fn zone_bottom(mem: &Mem) -> Option<f32> {
    let bg = read32(mem, BG_PTR.va()).filter(|&p| p >= 0x8000_0000)?;
    readf(mem, bg + ZONE[3])
}

/// A rectangle in world units, y up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    fn right(&self) -> f32 {
        self.left + self.w
    }

    fn bottom(&self) -> f32 {
        self.top - self.h
    }

    fn centre(&self) -> [f32; 2] {
        [self.left + self.w / 2.0, self.top - self.h / 2.0]
    }

    /// This rectangle grown about its centre to `shape`'s width:height.
    fn with_shape_of(&self, shape: &Rect) -> Rect {
        let aspect = shape.w / shape.h;
        let (w, h) = if self.w / self.h > aspect { (self.w, self.w / aspect) } else { (self.h * aspect, self.h) };
        let [cx, cy] = self.centre();
        Rect { left: cx - w / 2.0, top: cy + h / 2.0, w, h }
    }

    fn union(&self, o: &Rect) -> Rect {
        let (l, r) = (self.left.min(o.left), self.right().max(o.right()));
        let (t, b) = (self.top.max(o.top), self.bottom().min(o.bottom()));
        Rect { left: l, top: t, w: r - l, h: t - b }
    }

    /// A rectangle this one's size centred on `c`, moved to lie inside
    /// `within` (centred on it where it does not fit).
    fn around(&self, c: [f32; 2], within: &Rect) -> Rect {
        let fit = |c: f32, half: f32, lo: f32, hi: f32| if hi - lo <= 2.0 * half { (lo + hi) / 2.0 } else { c.clamp(lo + half, hi - half) };
        let x = fit(c[0], self.w / 2.0, within.left, within.right());
        let y = fit(c[1], self.h / 2.0, within.bottom(), within.top);
        Rect { left: x - self.w / 2.0, top: y + self.h / 2.0, w: self.w, h: self.h }
    }
}

/// The `dBg_c` pointer and its zone's bounds (bits), which change when a
/// scene or zone is set up.
fn scene_key(mem: &Mem) -> Option<[u32; 5]> {
    let bg = read32(mem, BG_PTR.va())?;
    Some([bg, read32(mem, bg + ZONE[0])?, read32(mem, bg + ZONE[1])?, read32(mem, bg + ZONE[2])?, read32(mem, bg + ZONE[3])?])
}

/// The game's camera as `dBgParameter_c` publishes it: what the game draws
/// with. In a multiplayer course `dBg_c`'s own rectangle (below) can lag
/// behind it (it stayed at x 630 at the start of 1-1 while the screen
/// followed Mario to x 4400).
fn live_camera(mem: &Mem, param: u32) -> Option<Rect> {
    let r = Rect { left: readf(mem, param + P_LEFT)?, top: readf(mem, param + P_TOP)?, w: readf(mem, param + P_W)?, h: readf(mem, param + P_H)? };
    ((16.0..20_000.0).contains(&r.w) && (16.0..20_000.0).contains(&r.h)).then_some(r)
}

/// The game's camera rectangle in `dBg_c`.
fn bg_camera(mem: &Mem, bg: u32) -> Option<Rect> {
    let r = Rect { left: readf(mem, bg + BG_LEFT)?, top: readf(mem, bg + BG_TOP)?, w: readf(mem, bg + BG_W)?, h: readf(mem, bg + BG_H)? };
    ((16.0..20_000.0).contains(&r.w) && (16.0..20_000.0).contains(&r.h)).then_some(r)
}

#[derive(Default)]
pub struct SharedCamera {
    /// Use the box (every PC in a session must agree; on for normal play,
    /// off only in tests of the game's own camera).
    pub enabled: bool,
    /// Words the box wrote: (address, the game's value, the box's value).
    saved: Vec<(u32, u32, u32)>,
    /// The `dBg_c` and its zone (left, right, top, bottom as bits) when the
    /// box was written: if either has changed by the next `execute`, the
    /// scene or zone was set up again, and the saved values are stale.
    saved_scene: [u32; 5],
    /// Host only: the share of the game camera's size a player's window
    /// shows (1, or more for testing).
    pub view_zoom: f32,
    /// Host only, set at the end of each `dBgGm_c::execute`: the camera the
    /// game draws with, and each player's window on it.
    pub drawn: gx::WorldCamera,
    pub views: [Option<gx::xf::View>; 4],
    /// Host only: `dBgGm_c::execute` ended since this was last cleared.
    pub ran: bool,
}

impl SharedCamera {
    /// At the start of `dBgGm_c::execute`: the game's own values come
    /// back, so its camera code (`calcScroll` and the multiplayer zoom,
    /// which also run outside `execute` and would otherwise build on the
    /// box) always works from its own state. Not after the scene or zone
    /// was set up again (`InitialScrollSet`), whose values stand.
    pub fn restore(&mut self, mem: &mut Mem) {
        if self.saved.is_empty() {
            return;
        }
        let same_scene = scene_key(mem) == Some(self.saved_scene);
        for (a, game, _) in self.saved.drain(..) {
            if same_scene {
                let _ = mem.write_u32(a, game);
            }
        }
    }

    /// At the end of `dBgGm_c::execute`: writes the box if it is needed,
    /// and works out this PC's view.
    pub fn apply(&mut self, mem: &mut Mem) {
        self.ran = true;
        self.views = [None; 4];
        let (Some(bg), Some(param)) = (read32(mem, BG_PTR.va()).filter(|&p| p >= 0x8000_0000), read32(mem, PARAM_PTR.va()).filter(|&p| p >= 0x8000_0000)) else { return };
        let Some(own) = bg_camera(mem, bg) else { return };
        // What the game draws with when there is no box (in a multiplayer
        // course `dBg_c`'s rectangle can lag behind it).
        let live = live_camera(mem, param).unwrap_or(own);
        let players = followed_players(mem);
        let (drawn, area) = match self.enabled.then(|| self.shared_box(mem, bg, own, &players)).flatten() {
            Some(area) => {
                let drawn = area.with_shape_of(&own);
                self.write_box(mem, bg, param, drawn);
                (drawn, area)
            }
            None => (live, live),
        };
        self.drawn = gx::WorldCamera { size: [drawn.w, drawn.h], centre: drawn.centre(), normal: drawn == live, normal_size: [own.w, own.h] };
        // Windows are the game camera's normal size (`dBg_c`'s; the live
        // camera may be zoomed out to fit the players).
        let size = Rect { w: own.w, h: own.h, ..live };
        // Players' windows only in a course played normally: the title's
        // demo has four players of its own, shown as the game shows them.
        if read32(mem, GAME_MODE.va()) == Some(0) {
            self.views = std::array::from_fn(|p| self.view_for(mem, p, size, drawn, area));
        }
    }

    /// The box around every player, or `None` where the game's own camera
    /// is used: one player, another camera mode, auto-scroll, or nobody
    /// outside the game's camera.
    fn shared_box(&self, mem: &Mem, bg: u32, own: Rect, players: &[[f32; 2]]) -> Option<Rect> {
        // Two players or more, in a course played normally (not the title's
        // demo, mode 2).
        if players.len() < 2 || read32(mem, GAME_MODE.va()) != Some(0) {
            return None;
        }
        let byte = |o: u32| mem.ram(bg + o, 1).map(|b| b[0]);
        if byte(BG_MODE)? != 0 || byte(BG_AUTOSCROLL)? != 0 || byte(BG_NO_CALC)? != 0 || byte(BG_LOCKED)? != 0 {
            return None;
        }
        let z: Vec<f32> = ZONE.iter().map(|&o| readf(mem, bg + o)).collect::<Option<_>>()?;
        let zone = Rect { left: z[0], top: z[2], w: z[1] - z[0], h: z[2] - z[3] };
        if zone.w <= 0.0 || zone.h <= 0.0 {
            return None;
        }
        let mut b = own;
        for &p in players {
            b = b.union(&own.around(p, &zone));
        }
        let same = |x: f32, y: f32| (x - y).abs() < 0.01;
        (!(same(b.left, own.left) && same(b.top, own.top) && same(b.w, own.w) && same(b.h, own.h))).then_some(b)
    }

    /// Writes `b`, which already has the game camera's shape, so the zoom
    /// (size = base ÷ zoom) agrees with the width and the height and the
    /// game draws it without stretching.
    fn write_box(&mut self, mem: &mut Mem, bg: u32, param: u32, b: Rect) {
        let Some(base_w) = readf(mem, BASE_W.va()) else { return };
        let Some(scene) = scene_key(mem) else { return };
        self.saved_scene = scene;
        // The zoom of the box, and an activity screen that covers it.
        let fit = base_w / b.w;
        let zoom_out = readf(mem, bg + BG_ZOOM_OUT).map_or(fit, |z| z.min(fit));
        let [cx, cy] = b.centre();
        let mut words = vec![
            (bg + BG_LEFT, b.left),
            (bg + BG_TOP, b.top),
            (bg + BG_W, b.w),
            (bg + BG_H, b.h),
            (bg + BG_LEFT2, b.left),
            (bg + BG_TOP2, b.top),
            (bg + BG_ZOOM_OUT, zoom_out),
            (param + P_LEFT, b.left),
            (param + P_TOP, b.top),
            (param + P_W, b.w),
            (param + P_H, b.h),
            (param + P_CX, cx),
            (param + P_CY, cy),
            (param + P_LEFT2, b.left),
            (param + P_TOP2, b.top),
        ];
        words.extend(BG_ZOOM.iter().map(|&o| (bg + o, fit)));
        for (a, v) in words {
            let Some(game) = read32(mem, a) else { continue };
            if mem.write_u32(a, v.to_bits()).is_ok() {
                self.saved.push((a, game, v.to_bits()));
            }
        }
    }

    /// Player `p`'s window: the game camera's own size (1/zoom of it when
    /// testing) around the player, inside `area` (the box before it was
    /// given the camera's shape, so the window stays in the zone), relative
    /// to `drawn`, what the game draws with. `None` without that player.
    fn view_for(&self, mem: &Mem, p: usize, own: Rect, drawn: Rect, area: Rect) -> Option<gx::xf::View> {
        let at = player_position(mem, p)?;
        let zoom = self.view_zoom.max(1.0);
        let window = Rect { w: own.w / zoom, h: own.h / zoom, ..own };
        let w = window.around(at, &area);
        let ([dx, dy], [ox, oy]) = (drawn.centre(), w.centre());
        Some(gx::xf::View { offset: [ox - dx, oy - dy], size: [w.w, w.h] })
    }
}

/// The words the box wrote, so a loaded state puts the game's values back
/// at the same point; `enabled` and the view are each machine's own.
impl Snapshot for SharedCamera {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.saved, self.saved_scene);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_stay_in_the_zone() {
        let zone = Rect { left: 0.0, top: 0.0, w: 1000.0, h: 400.0 };
        let cam = Rect { left: 300.0, top: -50.0, w: 400.0, h: 200.0 };
        let w = cam.around([50.0, -390.0], &zone);
        assert_eq!((w.left, w.top), (0.0, -200.0));
        let u = cam.union(&w);
        assert_eq!((u.left, u.top, u.w, u.h), (0.0, -50.0, 700.0, 350.0));
        // Given the camera's 2:1 shape, about its centre.
        let s = u.with_shape_of(&cam);
        assert_eq!((s.left, s.top, s.w, s.h), (0.0, -50.0, 700.0, 350.0));
        let t = Rect { left: 0.0, top: 0.0, w: 100.0, h: 100.0 }.with_shape_of(&cam);
        assert_eq!((t.left, t.top, t.w, t.h), (-50.0, 0.0, 200.0, 100.0));
        // Taller than the zone: centred on it.
        let tall = Rect { left: 0.0, top: 0.0, w: 100.0, h: 500.0 };
        assert_eq!(tall.around([500.0, -10.0], &zone).top, 50.0);
    }
}
