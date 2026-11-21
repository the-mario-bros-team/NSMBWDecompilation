//! Display options that need the game's help (after WitherBlitz's
//! NSMB-Wii-Recompilation: `runtime/src/hle/gx/depth_blur.cpp`,
//! `runtime/src/display_settings.cpp`, `runtime/src/hle/nsmbw_camera.cpp`).
//!
//! Depth Blur off: the courses' depth of field and heat haze or underwater
//! shimmer (`EGG::DrawPathDOF`, used by desert, beach, water, lava and
//! Bowser areas) go unseen. Its `internalDraw` runs in three steps: 0 and 1
//! capture the picture and build the blurred copy, 2 draws that copy over
//! the picture. Step 2 still runs, but the GX decoder drops its draws (a
//! switch in the command stream, [`hw::Hw::drop_draws`]), so guest memory
//! is the same either way and each player of a net game may choose.
//!
//! A wider picture (16:10, or Ultrawide filling a screen wider than 16:9):
//! [`System::set_screen_aspect`] widens the game's 16:9 canvas, its 2D
//! layouts' squeeze and the level camera; the renderer widens the 16:9
//! projections the same way (`gx::xf`). Then:
//! - `CrapUpPositions`, which places the Items screen's (and player
//!   select's) models on their layout cards in a 16:9 orthographic camera
//!   the renderer widens, spreads them out again by the same factor;
//! - `dBg_c::getLeftLimit` and `getRightLimit`: a zone narrower than the
//!   widened view is centred in it (a castle boss's arena), so the screen's
//!   centre, which bosses place their moves from, is where it is in 16:9.
//!   The players' own walls (`daPlBase_c::checkSideViewLemit`, which asks
//!   for the same limits) keep the zone's edges.

use crate::System;

/// `EGG::DrawPathDOF::internalDraw(this, step)`, reached through the draw
/// path's vtable.
pub const DOF_DRAW: u32 = 0x802C_3CB0;
/// `CrapUpPositions(Point3d* out, const Point3d* in)`.
pub const CRAP_UP_POSITIONS: u32 = 0x800B_32E0;
/// `dBg_c::getLeftLimit` and `getRightLimit`.
pub const LEFT_LIMIT: u32 = 0x8007_8A70;
pub const RIGHT_LIMIT: u32 = 0x8007_8C10;

/// The hooked functions, for [`crate::SPECIAL`].
pub const HOOKS: [u32; 4] = [DOF_DRAW, CRAP_UP_POSITIONS, LEFT_LIMIT, RIGHT_LIMIT];

/// Where `daPlBase_c::checkSideViewLemit` (0x80055810) gets the limits back.
const SIDE_WALLS: [u32; 2] = [0x8005_5898, 0x8005_58C0];

/// The game's 16:9 aspect.
pub const ASPECT_16_9: f32 = 16.0 / 9.0;

// EGG::Screen: sTVModeInfo's 16:9 record {u16 width, u16 height, f32
// scaleX, f32 scaleY}, the current TV mode, the root screen (+8 its canvas
// width, +0x34 its flags: bit 0 rebuild), and EGG::StateGX::s_widthFb (u16).
const TV_MODE_WIDE: u32 = 0x8035_04E0 + 0xC;
const TV_MODE: u32 = 0x8042_AF08;
const ROOT_SCREEN: u32 = 0x8042_AF0C;
const FB_WIDTH: u32 = 0x8042_AF38;
/// dScreen::l_ScreenSize16x9's width (f32).
const SCREEN_SIZE_16_9: u32 = 0x8042_A058;
/// 608 / width (f32): the x squeeze of 16:9 layouts in the 608-unit 2D
/// projection (d2d::Multi_c's and m2d::Simple_c's).
const LAYOUT_SQUEEZE: [u32; 2] = [0x8042_B084, 0x8042_DC48];
/// The level camera's width, 408 (f32 constant), and mVideo's copies
/// (l_rayoutWidth as s32, l_rayoutWidthF, l_rayoutHeightF, l_rayoutAspect).
const STAGE_WIDTH_CONST: u32 = 0x8042_CD78;
const STAGE_WIDTH_INT: u32 = 0x8042_A0E4;
const STAGE_WIDTH: u32 = 0x8042_A0EC;
const STAGE_HEIGHT: u32 = 0x8042_A0F0;
const STAGE_ASPECT: u32 = 0x8042_A0F4;
/// Columns of tiles bgTex_c::drawBuffer draws (f32, 28).
const TILE_COLUMNS: u32 = 0x8042_BEC4;
/// The game's own sizes: its 16:9 canvas, 2D projection and tile columns.
/// (Its level view is 408.704 units wide, taken from the game.)
const GAME_WIDTH: u16 = 832;
const LAYOUT_WIDTH: f32 = 608.0;
const TILE_COLUMNS_BASE: f32 = 28.0;

// CrapUpPositions: mVideo::m_video (its +4 width, +6 height, u16), and
// dGameCom::GetAspectRatio (1: widescreen) with the 4:3 to 16:9 factor.
const VIDEO: u32 = 0x8042_A480;
const GET_ASPECT_RATIO: u32 = 0x800B_54B0;
const ASPECT_FACTOR: u32 = 0x8042_8750;

// The camera limits: dBg_c's +0x90000 +0xC zoom, and the f32 1 the limits
// divide by it.
const BG_ZOOM: u32 = 0x9_000C;
const ONE: u32 = 0x8042_BD18;

/// The display options' state on the host.
pub struct Display {
    /// Depth Blur: step 2 of the depth of field is drawn.
    pub depth_blur: bool,
    /// The aspect the renderer widens 16:9 projections to (0: none).
    pub wide: f32,
    /// The game's own level view width (its constant before any widening).
    stage_base: f32,
    /// Step 2 with its draws dropped: where it returns to (and the stack
    /// pointer there), to switch them back on.
    dof_return: Option<(u32, u32)>,
    /// A hooked function the next call runs as the game has it.
    bypass: Option<u32>,
}

impl Default for Display {
    fn default() -> Self {
        Self { depth_blur: true, wide: 0.0, stage_base: 0.0, dof_return: None, bypass: None }
    }
}

impl Display {
    /// Watching for step 2 of the depth of field to return.
    pub(crate) fn watching(&self) -> bool {
        self.dof_return.is_some()
    }

    /// A state was loaded: no step 2 is in flight (the hardware's draw drop
    /// was reset with it).
    pub(crate) fn forget(&mut self) {
        self.dof_return = None;
        self.bypass = None;
    }
}

impl System {
    fn rf(&mut self, a: u32) -> f32 {
        f32::from_bits(self.mem.read_u32(a).unwrap_or(0))
    }
    fn wf(&mut self, a: u32, v: f32) {
        let _ = self.mem.write_u32(a, v.to_bits());
    }
    fn rh(&mut self, a: u32) -> u16 {
        self.mem.read_u16(a).unwrap_or(0)
    }

    /// Before the instruction at `pc` while step 2 of the depth of field
    /// runs with its draws dropped: once it returns, they come back.
    pub(crate) fn display_return(&mut self, pc: u32) {
        if self.display.dof_return == Some((pc, self.cpu.gpr[1])) {
            self.display.dof_return = None;
            self.hw.borrow_mut().drop_draws(false);
        }
    }

    /// The display hook at `pc`, if it applies: the function's work done and
    /// the CPU at its return address. False to run the game's function.
    pub(crate) fn display_hook(&mut self, pc: u32) -> bool {
        if self.display.bypass == Some(pc) {
            self.display.bypass = None;
            return false;
        }
        let ran = match pc {
            DOF_DRAW => {
                // Step 2 (the final full-screen draw) runs as it is, with
                // its draws dropped.
                if self.cpu.gpr[4] & 0xFFFF == 2 && !self.display.depth_blur && self.display.dof_return.is_none() {
                    self.display.dof_return = Some((self.cpu.lr, self.cpu.gpr[1]));
                    self.hw.borrow_mut().drop_draws(true);
                }
                false
            }
            CRAP_UP_POSITIONS => self.crap_up_positions(),
            LEFT_LIMIT | RIGHT_LIMIT => self.camera_limit(pc == RIGHT_LIMIT),
            _ => false,
        };
        if ran {
            self.cpu.pc = self.cpu.lr;
        }
        ran
    }

    /// The game's own `addr` (a hooked function) past its hook, on the
    /// hooked thread's stack: the CPU as it returned.
    fn call_game(&mut self, addr: u32, args: &[u32]) -> guest_cpu::Cpu {
        self.display.bypass = Some(addr);
        let sp = self.cpu.gpr[1];
        let after = self.call_nested_cpu(addr, args, sp);
        self.display.bypass = None;
        after
    }

    /// The Items screen's models onto their cards in a widened view: the
    /// game's position, its distance from the camera's centre spread by
    /// the renderer's widening.
    fn crap_up_positions(&mut self) -> bool {
        let wide = self.display.wide;
        if wide <= 0.0 {
            return false;
        }
        let out = self.cpu.gpr[3];
        let args = [out, self.cpu.gpr[4]];
        self.call_game(CRAP_UP_POSITIONS, &args);
        // The game's camera, as it works it out.
        let video = self.mem.read_u32(VIDEO).unwrap_or(0);
        let video = self.mem.read_u32(video).unwrap_or(0);
        let (mut width, height) = (f32::from(self.rh(video + 4)), f32::from(self.rh(video + 6)));
        let sp = self.cpu.gpr[1];
        if self.call_nested(GET_ASPECT_RATIO, &[], sp) & 0xFF == 1 {
            width *= self.rf(ASPECT_FACTOR);
        }
        let aspect = width / height;
        if height > 0.0 && aspect > 1.68 && aspect < 1.88 {
            let centre = 0.5 * width;
            let x = self.rf(out);
            self.wf(out, centre + (x - centre) * (wide / aspect));
        }
        true
    }

    /// The camera's left or right limit: in a widened view, a zone narrower
    /// than the view is centred in it; otherwise (and for the players'
    /// walls) the game's own.
    fn camera_limit(&mut self, right: bool) -> bool {
        if self.display.wide <= 0.0 || SIDE_WALLS.contains(&self.cpu.lr) {
            return false;
        }
        let view = self.rf(STAGE_WIDTH);
        if !(view > self.display.stage_base + 0.5) {
            return false;
        }
        let bg = self.cpu.gpr[3];
        let own = self.call_game(if right { RIGHT_LIMIT } else { LEFT_LIMIT }, &[bg]).fpr[1][0] as f32;
        let other = self.call_game(if right { LEFT_LIMIT } else { RIGHT_LIMIT }, &[bg]).fpr[1][0] as f32;
        let (left, right_edge) = if right { (other, own) } else { (own, other) };
        // The camera's width as dBg_c::calcScroll takes it: the view over the zoom.
        let width = view * (self.rf(ONE) / self.rf(bg.wrapping_add(BG_ZOOM)));
        let limit = if right_edge - left < width {
            let centre = (left + right_edge) * 0.5;
            if right {
                centre + width * 0.5
            } else {
                centre - width * 0.5
            }
        } else {
            own
        };
        self.cpu.fpr[1][0] = f64::from(limit);
        true
    }

    /// Fits the game to a picture `aspect` wide (16:9 is its own; 16:10, or
    /// wider for Ultrawide): its 16:9 canvas, the 2D layouts' squeeze, and
    /// the level camera (so tiles, backgrounds and actors are drawn and
    /// woken across the whole picture). Call it between fields; it writes
    /// only when the canvas is not that wide yet, so at 16:9 it never
    /// writes. Before `EGG::Screen` is set up it does nothing.
    pub fn set_screen_aspect(&mut self, aspect: f32) {
        if self.mem.read_u32(ROOT_SCREEN).unwrap_or(0) == 0 {
            return;
        }
        let width = self.rh(TV_MODE_WIDE);
        if width == GAME_WIDTH && self.display.stage_base == 0.0 {
            self.display.stage_base = self.rf(STAGE_WIDTH_CONST);
        }
        let target = if (aspect - ASPECT_16_9).abs() < 1e-4 { GAME_WIDTH } else { (f32::from(GAME_WIDTH) * aspect / ASPECT_16_9).round() as u16 };
        if width == target || width == 0 || self.display.stage_base <= 0.0 {
            return;
        }
        let _ = self.mem.write_u16(TV_MODE_WIDE, target);
        let fb = f32::from(self.rh(FB_WIDTH));
        if fb > 0.0 {
            self.wf(TV_MODE_WIDE + 4, fb / f32::from(target));
        }
        self.wf(SCREEN_SIZE_16_9, f32::from(target));
        for a in LAYOUT_SQUEEZE {
            self.wf(a, LAYOUT_WIDTH / f32::from(target));
        }
        let base = self.display.stage_base;
        let stage = base * f32::from(target) / f32::from(GAME_WIDTH);
        self.wf(STAGE_WIDTH_CONST, stage);
        self.wf(TILE_COLUMNS, (TILE_COLUMNS_BASE * stage / base).ceil());
        // mVideo's copies, once mVideo::create has taken them.
        let stage_height = self.rf(STAGE_HEIGHT);
        if stage_height > 0.0 {
            self.wf(STAGE_WIDTH, stage);
            // The game keeps 408 for 408.704.
            let _ = self.mem.write_u32(STAGE_WIDTH_INT, stage as u32);
            self.wf(STAGE_ASPECT, stage / stage_height);
        }
        // What EGG::Screen::SetTVMode would do: the root screen takes the
        // record's width and is rebuilt.
        if self.mem.read_u32(TV_MODE).unwrap_or(0) == 1 {
            let root = self.mem.read_u32(ROOT_SCREEN).unwrap_or(0);
            self.wf(root + 8, f32::from(target));
            let flags = self.rh(root + 0x34);
            let _ = self.mem.write_u16(root + 0x34, flags | 1);
        }
    }
}
