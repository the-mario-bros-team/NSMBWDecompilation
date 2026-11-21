//! The Settings window, in the pause window's style: its red-framed panel,
//! its green buttons with their on / off / hit animations, the cursor's
//! corners. A port of WitherBlitz's NSMB-Wii-Recompilation settings window
//! (`runtime/src/netplay/netplay_ui.cpp`: `DrawSettings` and the tabs), on
//! this port's own settings. Layout units: 456 across the picture's height,
//! y up, the window centred.

use std::sync::Arc;

use super::layout::{self, Animation, DrawList, DrawParams, Font, Layout, Rgba, View};
use super::{Change, Nav, UpdateState};
use crate::gameplay::POWER_UPS;
use crate::settings::Settings;

// ---------------------------------------------------------------- assets

struct Assets {
    yes_no: Option<Layout>,
    cursor: Option<Layout>,
    cursor_loop: Option<Animation>,
    message: Arc<Font>,
    mario: Arc<Font>,
    picture: Arc<Font>,
    pause: Option<Layout>,
    in_window: Option<Animation>,
    out_window: Option<Animation>,
    on_button: Option<Animation>,
    off_button: Option<Animation>,
    hit_button: Option<Animation>,
    /// The pause window's buttons are three slices (Newer's).
    pause_slices: bool,
}

impl Assets {
    fn load(ctx: &egui::Context) -> Self {
        const PAUSE: &str = "/Layout/pauseMenu/pauseMenu.arc";
        const CURSOR: &str = "/Layout/select_cursor/select_cursor.arc";
        let pause = Layout::load(ctx, PAUSE, "pauseMenu_16.brlyt").filter(|l| l.pane_rect("P_SBBase_00").is_some() && l.pane_rect("W_N_pauseMenu_00").is_some());
        let anim = |name: &str| pause.as_ref().and_then(|_| Animation::load(PAUSE, name));
        Self {
            yes_no: Layout::load(ctx, "/Layout/yesnoWindow/yesnoWindow.arc", ""),
            cursor: Layout::load(ctx, CURSOR, ""),
            cursor_loop: Animation::load(CURSOR, "loopCursor.brlan"),
            message: layout::load_font(ctx, "mj2d00_MessageFont_32_I4.brfnt"),
            mario: layout::load_font(ctx, "mj2d01_marioFont_64_IA4.brfnt"),
            picture: layout::load_font(ctx, "mj2d00_PictureFont_32_RGBA8.brfnt"),
            in_window: anim("_inWindow.brlan"),
            out_window: anim("_outWindow.brlan"),
            on_button: anim("_onButton.brlan"),
            off_button: anim("_offButon.brlan"),
            hit_button: anim("_hitButton.brlan"),
            pause_slices: pause.as_ref().is_some_and(|l| l.pane_rect("Btn1_Middle").is_some()),
            pause,
        }
    }
}

#[derive(Clone, Copy)]
struct Canvas {
    view: View,
    alpha: f32,
}

// ---------------------------------------------------------------- layout

/// Made for a 16:9 screen (456 units tall, 811 across).
const WIDTH: f32 = 766.0;
const HEIGHT: f32 = 424.0;
const SCREEN_W: f32 = 456.0 * 16.0 / 9.0;
const TITLE_Y: f32 = 181.0;
const TABS_Y: f32 = 133.0;
const TABS_RULE_Y: f32 = 105.0;
const TABS_H: f32 = 44.0;
const NOTE_Y: f32 = -136.0;
const PROMPT_RULE_Y: f32 = -152.0;
const PROMPT_Y: f32 = -181.0;
const TAB_KEY: i32 = 1000;
const TEXT_GROW: f32 = 1.12;

/// The rows' controls fill one column, COL_X0 to COL_X1.
const COL_X0: f32 = 50.0;
const COL_X1: f32 = 352.0;
const LABEL_X: f32 = -352.0;
const COL_W: f32 = COL_X1 - COL_X0;
const COL_X: f32 = (COL_X0 + COL_X1) / 2.0;
const PAIR_GAP: f32 = 8.0;
const PAIR_W: f32 = (COL_W - PAIR_GAP) / 2.0;
/// A picker's arrow, at each end of the column.
const ARROW_W: f32 = 28.0;
const PERF_BUTTON_W: f32 = 92.0;
const PERF_SLIDER_X0: f32 = COL_X0 + PERF_BUTTON_W + 12.0;
const LABEL_DIM: u32 = 0xD8E8_D0FF;

const EV_LEFT: u32 = 1;
const EV_RIGHT: u32 = 2;
const EV_CONFIRM: u32 = 16;
const EV_BACK: u32 = 32;
const EV_TAB0: u32 = 64;

const DROPDOWN_HIT: i32 = 5000;
/// A question's Yes (0) and No (1), as hit cursors.
const ASK_HIT: i32 = 6000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tab {
    Display,
    Graphics,
    Audio,
    Keybinds,
    Gameplay,
    Language,
}

const TABS: [Tab; 6] = [Tab::Display, Tab::Graphics, Tab::Audio, Tab::Keybinds, Tab::Gameplay, Tab::Language];

impl Tab {
    fn name(self) -> &'static str {
        match self {
            Tab::Display => "Display",
            Tab::Graphics => "Graphics",
            Tab::Audio => "Audio",
            Tab::Keybinds => "Keybinds",
            Tab::Gameplay => "Gameplay",
            Tab::Language => "Language",
        }
    }

    fn help(self) -> &'static str {
        match self {
            Tab::Display => "How the game is shown on your screen.",
            Tab::Graphics => "How the game is drawn, and how smoothly it moves.",
            Tab::Audio => "How loud the game's music and sounds are.",
            Tab::Keybinds => "Which keys play the Wii Remote's buttons.",
            Tab::Gameplay => "Optional helpers. Online, only Exit Course applies, if every player has it on.",
            Tab::Language => "The language of the game's text and these menus.",
        }
    }
}

// Display rows.
const ROW_RESOLUTION: i32 = 1;
const ROW_ASPECT: i32 = 2;
const ROW_FULLSCREEN: i32 = 3;
const ROW_VSYNC: i32 = 4;
const ROW_GRAPH: i32 = 5;
const DISPLAY_LAST: i32 = 5;
// Graphics rows.
const ROW_RENDERER: i32 = 1;
const ROW_BATTERY: i32 = 2;
const ROW_INTERPOLATION: i32 = 3;
const GRAPHICS_LAST: i32 = 3;
// Gameplay rows.
const ROW_TIME: i32 = 1;
const ROW_LIVES: i32 = 2;
const ROW_POWER_UP: i32 = 3;
const ROW_AUTOSAVE: i32 = 4;
const ROW_EXIT_COURSE: i32 = 5;
const ROW_P1_CHARACTER: i32 = 6;
const GAMEPLAY_LAST: i32 = 6;
// Audio rows: 1-4 the sliders.
const VOLUMES: [(&str, &str); 4] = [
    ("Master", "The volume of everything."),
    ("Music", "The background music."),
    ("Sound Effects", "Jumps, coins, enemies and the rest."),
    ("Menu Sounds", "The menus' cursor, select and back sounds."),
];
const SLIDER_H: f32 = 34.0;
// Keybinds: 1-12 the controls (two columns of six, `input::keymap::ACTIONS`).
const KEY_HELP: [&str; 12] = [
    "The D-pad's up: climb vines, and go through doors and pipes.",
    "The D-pad's down: crouch, and ground-pound in the air.",
    "The D-pad's left: walk left, and move a menu's cursor.",
    "The D-pad's right: walk right, and move a menu's cursor.",
    "The 2 button: jump and swim, and choose in menus.",
    "The 1 button: run, throw fireballs, and go back in menus.",
    "The A button: view the whole world map.",
    "Shakes the Wii Remote: a spin jump, or picking up and throwing.",
    "Tilts the Wii Remote left, for the platforms that tilt.",
    "Tilts the Wii Remote right, for the platforms that tilt.",
    "The + button: pause a course, or open the world map's menu.",
    "The - button: open Select a World on the world map.",
];
/// A key capture with nothing pressed for this long is cancelled.
const CAPTURE_TIMEOUT: f64 = 10.0;
// Language rows: 1-3.
const LANGUAGES: [(&str, &str, &str); 3] = [
    ("en", "English", "Restart the game to play in this language."),
    ("es", "Español", "Reinicia el juego para jugar en este idioma."),
    ("fr", "Français", "Redémarrez le jeu pour jouer dans cette langue."),
];

/// Resolution choices: EFB multiples, then Native (0).
const SCALES: [u32; 7] = [1, 2, 3, 4, 6, 8, 0];
/// Frame rates: 60 is the game's own; the others interpolate.
const FRAME_RATES: [u32; 11] = [60, 90, 120, 144, 165, 180, 240, 300, 360, 400, 0];

fn scale_label(scale: u32) -> String {
    if scale == 0 {
        "Native".into()
    } else {
        format!("{scale}x ({}p)", 480 * scale)
    }
}

fn rate_label(fps: u32) -> String {
    match fps {
        60 => "Off".into(),
        0 => "Unlocked".into(),
        n => format!("{n} FPS"),
    }
}

/// Rows: one setting a row, the fewer the rows, the taller.
fn row_step(rows: i32) -> f32 {
    if rows <= 5 {
        45.0
    } else {
        185.0 / (rows - 1) as f32
    }
}
fn row_h(rows: i32) -> f32 {
    row_step(rows) - if rows <= 5 { 10.0 } else { 8.0 }
}
fn rows_top(rows: i32) -> f32 {
    if rows <= 5 {
        80.0
    } else {
        82.0
    }
}
fn row_text(rows: i32) -> f32 {
    match rows {
        ..=5 => 28.0,
        6 => 26.0,
        _ => 22.0,
    }
}

#[derive(Clone, Copy)]
enum DropKind {
    Resolution,
    Aspect,
    Interpolation,
}

struct Dropdown {
    row: i32,
    highlight: usize,
    current: i32,
    labels: Vec<String>,
    kind: DropKind,
    y: f32,
    row_h: f32,
}

struct Hit {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    cursor: i32,
    event: u32,
}

/// What the Gameplay tab asks about before doing it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cheat {
    Lives,
    PowerUp,
}

/// A question over the window, in the game's yes/no window.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ask {
    /// "Are you sure? This will mark your save as cheated."
    Cheat(Cheat),
    /// A newer release of the port is out: open its web page?
    Update,
}

#[derive(Default)]
struct ButtonAnims {
    current: i32,
    previous: i32,
    changed: f64,
    hit: i32,
    hit_at: f64,
}

#[derive(Default)]
pub struct SettingsWindow {
    assets: Option<Arc<Assets>>,
    open: bool,
    opened_at: f64,
    closing: bool,
    closing_at: f64,
    tab: usize,
    cursor: i32,
    dropdown: Option<Dropdown>,
    anims: ButtonAnims,
    hits: Vec<Hit>,
    row_y: [f32; 16],
    row_h: f32,
    drag_slider: Option<usize>,
    /// When Power-Up was last pressed, for its note.
    power_up_at: Option<f64>,
    /// The Keybinds row waiting for its new key, and since when.
    capturing: Option<(usize, f64)>,
    /// The save file being played (0-2), or -1 before the save select.
    pub active_file: i8,
    /// A question is up: what it asks, and the tab's row to go back to.
    ask: Option<(Ask, i32)>,
    /// Its cursor: 0 Yes, 1 No.
    ask_cursor: i32,
    /// A newer release of the port is out: its version, and whether it can
    /// be installed from here (it has a download for Windows).
    pub update: Option<(String, bool)>,
    /// The update being installed, shown over everything.
    pub update_state: Option<UpdateState>,
    /// The question about it has been asked (once a run).
    update_asked: bool,
    /// The question can be drawn (the yes/no window's layout is there).
    can_ask: bool,
    /// Draw yes/no buttons at this scale instead of their own.
    button_scale: Option<f32>,
    // This frame.
    list: DrawList,
    now: f64,
    settings_look: bool,
    text_margin: f32,
    defer_cursor: bool,
    deferred: Option<(Canvas, f32, f32, f32, f32)>,
    /// A pointer press in layout units this frame, and where it is now.
    pointer: Option<(f32, f32)>,
    pressed: bool,
    held: bool,
}

fn pulse(t: f64) -> f32 {
    0.5 + 0.5 * ((t * 60.0) as f32 * 0.12).sin()
}

impl SettingsWindow {
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn capturing(&self) -> bool {
        self.capturing.is_some()
    }

    /// A key pressed while a Keybinds row waits for one: it becomes that
    /// action's key. Esc cancels; Delete or Backspace leaves it no key; F10
    /// and F11 keep their own jobs; keys without a name are not taken.
    pub fn capture_key(&mut self, s: &mut Settings, code: winit::keyboard::KeyCode) -> Vec<Change> {
        use winit::keyboard::KeyCode;
        let Some((action, _)) = self.capturing else { return Vec::new() };
        match code {
            KeyCode::Escape => {}
            KeyCode::Delete | KeyCode::Backspace => s.keymap.keys[action].clear(),
            KeyCode::F10 | KeyCode::F11 => return Vec::new(),
            k if input::keymap::key_name(k).is_some() => s.keymap.bind(action, k),
            _ => return Vec::new(),
        }
        self.capturing = None;
        if code == KeyCode::Escape {
            Vec::new()
        } else {
            vec![Change::Keymap]
        }
    }

    pub fn open(&mut self, now: f64) {
        self.open = true;
        self.closing = false;
        self.opened_at = now;
        self.cursor = 0;
        self.dropdown = None;
        self.ask = None;
    }

    pub fn close(&mut self, now: f64) {
        // Not while an update is being installed.
        if self.update_state.as_ref().is_some_and(|s| !matches!(s, UpdateState::Failed(_))) {
            return;
        }
        if !self.closing {
            self.closing = true;
            self.closing_at = now;
            if self.assets.as_ref().is_none_or(|a| a.out_window.is_none()) {
                self.open = false;
            }
        }
    }

    fn tab(&self) -> Tab {
        TABS[self.tab.min(TABS.len() - 1)]
    }

    fn add_hit(&mut self, x: f32, y: f32, w: f32, h: f32, cursor: i32, event: u32) {
        self.hits.push(Hit { x, y, w, h, cursor, event });
    }

    // ------------------------------------------------------------ drawing helpers

    /// The game's message text: white with its dark edge, centred on (x, y)
    /// unless `origin_h` says otherwise.
    #[allow(clippy::too_many_arguments)]
    fn text(&mut self, a: &Assets, c: &Canvas, text: &str, x: f32, y: f32, size: f32, origin_h: u8, color: u32) {
        if !a.message.ready() {
            return;
        }
        layout::draw_text(&mut self.list, &c.view, &a.message, text, x + 2.5, y - 2.5, size, 0x3020_0AFF, 0x3020_0AFF, 0.55 * c.alpha, origin_h, 1);
        layout::draw_text(&mut self.list, &c.view, &a.message, text, x, y, size, color, color, c.alpha, origin_h, 1);
    }

    /// A number in the Mario font, white.
    fn mario_number(&mut self, a: &Assets, c: &Canvas, text: &str, x: f32, y: f32, size: f32) {
        if a.mario.ready() {
            layout::draw_text(&mut self.list, &c.view, &a.mario, text, x, y, size, u32::MAX, u32::MAX, c.alpha, 1, 1);
        }
    }

    /// The game's selection cursor (four yellow corners) round a w x h box.
    fn cursor_box(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, x: f32, y: f32, w: f32, h: f32) {
        if self.defer_cursor {
            self.deferred = Some((*c, x, y, w, h));
            return;
        }
        let Some(cursor) = &a.cursor else { return };
        const CORNERS: [(&str, f32, f32, f32, f32); 4] = [("N_LU_00", -67.0, 42.0, -1.0, 1.0), ("N_RU_00", 67.0, 42.0, 1.0, 1.0), ("N_LD_00", -67.0, -42.0, -1.0, -1.0), ("N_RD_00", 67.0, -42.0, 1.0, -1.0)];
        let mut lp = DrawParams::new();
        let out = match &a.cursor_loop {
            Some(anim) => {
                anim.apply((self.now * 60.0) as f32 % anim.frames().max(1.0), cursor, &mut lp, &[]);
                0.0
            }
            None => 3.0 * pulse(self.now),
        };
        for (pane, ax, ay, sx, sy) in CORNERS {
            let mut p = lp.clone();
            p.alpha = c.alpha;
            p.offset_x = x + sx * (w / 2.0 + out) - ax;
            p.offset_y = y + sy * (h / 2.0 + out) - ay;
            cursor.draw(ctx, &mut self.list, &c.view, pane, &p);
        }
    }

    /// The yes/no window's green-to-yellow button, w x h, centred on (x, y).
    #[allow(clippy::too_many_arguments)]
    fn yes_no_button(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, label: &str, x: f32, y: f32, w: f32, h: f32, selected: bool, enabled: bool, text_size: f32) {
        let Some(yn) = &a.yes_no else { return };
        let mut p = DrawParams::new();
        p.alpha = c.alpha * if enabled { 1.0 } else { 0.55 };
        let (ax, ay, aw, ah) = yn.pane_rect("P_centerBase_00").unwrap_or((0.0, -118.0, 240.0, 53.0));
        p.offset_x = x - ax;
        p.offset_y = y - ay;
        p.scale = self.button_scale.unwrap_or(if selected { 1.0 + 0.04 * pulse(self.now) } else { 0.94 });
        p.pane_scale.insert("P_centerBase_00".into(), 1.0);
        p.pane_size.insert("P_centerBase_00".into(), (w, h));
        p.pane_size.insert("W_button_02".into(), (w + 5.0, h + 10.0));
        p.pane_size.insert("P_shadow_02".into(), (w, h));
        yn.stretch_slices(&mut p, "SingleBtnLeft", "SingleBtnMiddle", "SingleBtnRight", w - aw, h - ah);
        p.visible.insert("T_center_00".into(), false);
        p.visible.insert("T_center_01".into(), false);
        if !enabled {
            p.vertex_tint.insert("P_centerBase_00".into(), 0xB0B0_B0FF);
        }
        yn.draw(ctx, &mut self.list, &c.view, "P_centerBase_00", &p);
        let scale = p.scale;
        let mut size = text_size * scale;
        if a.message.ready() {
            let room = (w - 22.0) * scale;
            let width = layout::measure_text(&a.message, label, size);
            if width > room {
                size *= room / width;
            }
        }
        let t = Canvas { view: c.view, alpha: p.alpha };
        self.text(a, &t, label, x, y, size, 1, u32::MAX);
        if selected {
            self.cursor_box(ctx, a, c, x, y, w * scale + 6.0, h * scale + 6.0);
        }
    }

    /// A button of the pause window's, w x h, centred on (x, y). selected:
    /// the cursor is on it (on animation); lit: in the on colours without
    /// the cursor (the open tab, the setting in force).
    #[allow(clippy::too_many_arguments)]
    fn pause_button(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, key: i32, label: &str, x: f32, y: f32, w: f32, h: f32, selected: bool, enabled: bool, text_size: f32, lit: bool, grow: f32, cursor: bool) {
        let Some(pause) = &a.pause else {
            self.yes_no_button(ctx, a, c, label, x, y, w, h, selected && cursor, enabled, text_size);
            return;
        };
        let now = self.now;
        let s = &mut self.anims;
        if selected && s.current != key {
            s.previous = s.current;
            s.current = key;
            s.changed = now;
        }
        let mut p = DrawParams::new();
        p.alpha = c.alpha;
        let off_frames = a.off_button.as_ref().map_or(0.0, |x| x.frames());
        let (mut anim, mut frame) = (a.off_button.as_ref(), off_frames);
        if selected && key == s.current {
            let hit_frames = a.hit_button.as_ref().map_or(0.0, |x| x.frames());
            if s.hit == key && ((now - s.hit_at) * 60.0) < f64::from(hit_frames) {
                anim = a.hit_button.as_ref();
                frame = ((now - s.hit_at) * 60.0) as f32;
            } else {
                anim = a.on_button.as_ref();
                frame = (((now - s.changed) * 60.0) as f32).min(a.on_button.as_ref().map_or(0.0, |x| x.frames()));
            }
        } else if lit {
            anim = a.on_button.as_ref();
            frame = a.on_button.as_ref().map_or(0.0, |x| x.frames());
        } else if key == s.previous {
            frame = (((now - s.changed) * 60.0) as f32).min(off_frames);
        }
        if let Some(anim) = anim {
            anim.apply(frame, pause, &mut p, &[("P_SBBase_00", "P_SBBase_00"), ("P_shadow_00", "P_shadow_00"), ("T_tuzukeru_01", "T_tuzukeru_01")]);
        }
        let mut scale = 1.0;
        if let Some(v) = p.pane_scale.get_mut("P_SBBase_00") {
            scale = 1.0 + (*v - 1.0) * grow;
            *v = scale;
        }
        let (_, _, aw, ah) = pause.pane_rect("P_SBBase_00").unwrap_or((0.0, 0.0, 313.0, 53.0));
        let (tx, ty, sx, sy) = pause.pane_transform("P_SBBase_00").unwrap_or((0.0, 0.0, 1.0, 1.0));
        let (base_w, base_h) = (if sx != 0.0 { aw / sx } else { aw }, if sy != 0.0 { ah / sy } else { ah });
        p.offset_x = x - tx;
        p.offset_y = y - ty;
        p.pane_size.insert("P_SBBase_00".into(), (w, h));
        p.pane_size.insert("W_SButton_0".into(), (w + 7.0, h + 10.0));
        p.pane_size.insert("P_shadow_00".into(), (w - 1.0, h + 2.0));
        p.pane_size.insert("T_tuzukeru_00".into(), (w - 16.0, h));
        p.pane_size.insert("T_tuzukeru_01".into(), (w - 16.0, h));
        if a.pause_slices {
            pause.stretch_slices(&mut p, "Btn1_Left", "Btn1_Middle", "Btn1_Right", w - base_w, (h - base_h) * 76.0 / 53.0);
        }
        p.plain_text = true;
        p.text.insert("T_tuzukeru_00".into(), label.into());
        p.text.insert("T_tuzukeru_01".into(), label.into());
        let mut size = text_size;
        if self.settings_look {
            // Only the frame's outline (its gloss covers short buttons).
            p.pane_size.insert("W_SButton_0".into(), (w + 7.0, h + 7.0));
            p.frame_clip.insert("W_SButton_0".into(), (3.5, 3.5));
            p.color_matrix.insert("W_SButton_0".into(), [0.0; 9]);
            p.visible.insert("T_tuzukeru_01".into(), false);
            p.center_ink = true;
            size = (size * TEXT_GROW).min(h * 1.05);
        }
        if a.message.ready() && !label.is_empty() {
            let room = w - if self.settings_look { self.text_margin } else { 24.0 };
            let width = layout::measure_text(&a.message, label, size);
            let spacing = 2.0 * (label.chars().count().saturating_sub(1)) as f32;
            if width + spacing > room && width > 0.0 {
                size *= ((room - spacing) / width).max(0.3);
            }
        }
        p.text_scale.insert("T_tuzukeru_00".into(), size / 32.0);
        p.text_scale.insert("T_tuzukeru_01".into(), size / 32.0);
        if !enabled {
            p.alpha *= 0.6;
            for pane in ["P_SBBase_00", "Btn1_Left", "Btn1_Middle", "Btn1_Right"] {
                p.vertex_tint.insert(pane.into(), 0x9A9A_9AFF);
            }
        }
        pause.draw(ctx, &mut self.list, &c.view, "P_SBBase_00", &p);
        if selected && cursor {
            self.cursor_box(ctx, a, c, x, y, w * scale + 6.0, h * scale + 6.0);
        }
    }

    /// The window's panel: the pause window's dark fill in its red frame.
    fn panel(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, w: f32, h: f32) {
        let Some(pause) = &a.pause else { return };
        let mut p = DrawParams::new();
        p.alpha = c.alpha;
        for hidden in ["N_pauseMenu_01", "N_worldText_00", "N_osusumeText_00", "N_multiText_00", "LevelNumShadow", "LevelNameShadow"] {
            p.visible.insert(hidden.into(), false);
        }
        p.visible.insert("P_bg_00".into(), true);
        let (_, _, fw, fh) = pause.pane_rect("W_N_pauseMenu_00").unwrap_or((0.0, 0.0, 398.0, 229.0));
        p.pane_size.insert("P_bg_00".into(), (w, h));
        p.pane_size.insert("W_N_pauseMenu_00".into(), (w + 12.0, h + 14.0));
        pause.stretch_slices(&mut p, "BG_Left", "BG_Middle", "BG_Right", w + 12.0 - fw, h + 14.0 - fh);
        pause.draw(ctx, &mut self.list, &c.view, "N_pauseMenu_00", &p);
    }

    /// The yes/no window, w x h (528 x 322 as authored), without its texts
    /// and buttons.
    fn small_window(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, w: f32, h: f32) {
        let Some(yn) = &a.yes_no else { return };
        let mut p = DrawParams::new();
        p.alpha = c.alpha;
        for hidden in ["T_questionS_00", "N_button_00", "N_otehonText_00"] {
            p.visible.insert(hidden.into(), false);
        }
        p.pane_size.insert("P_bg_00".into(), (w, h));
        p.pane_size.insert("P_stripe_00".into(), (w + 2.0, h));
        p.pane_size.insert("W_yesnoWindow_00".into(), (w + 14.0, h + 8.0));
        yn.stretch_slices(&mut p, "WindowLeft", "WindowCentre", "WindowRight", w - 528.0, h - 322.0);
        yn.draw(ctx, &mut self.list, &c.view, "N_yesnoWindow_00", &p);
    }

    /// A thin white line across the window.
    fn rule(&mut self, ctx: &egui::Context, c: &Canvas, x0: f32, x1: f32, y: f32) {
        let t = (1.5 * c.view.scale).max(1.0) / c.view.scale;
        self.list.rect(ctx, &c.view, x0, y + t / 2.0, x1, y - t / 2.0, Rgba { r: 1.0, g: 1.0, b: 1.0, a: 110.0 / 255.0 * c.alpha });
    }

    fn column_cursor(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, y: f32, h: f32) {
        self.cursor_box(ctx, a, c, COL_X, y, COL_W + 6.0, h + 6.0);
    }

    /// On / Off as a pair of buttons across the column.
    #[allow(clippy::too_many_arguments)]
    fn on_off(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, key: i32, y: f32, h: f32, on: bool, selected: bool) {
        self.pause_button(ctx, a, c, key * 2 + 100_000, "On", COL_X0 + PAIR_W / 2.0, y, PAIR_W, h, false, on, 24.0, on, 0.0, false);
        self.pause_button(ctx, a, c, key * 2 + 100_001, "Off", COL_X1 - PAIR_W / 2.0, y, PAIR_W, h, false, !on, 24.0, !on, 0.0, false);
        if selected {
            self.column_cursor(ctx, a, c, y, h);
        }
    }

    fn picker_arrows(&mut self, a: &Assets, c: &Canvas, x0: f32, x1: f32, y: f32, selected: bool) {
        if !a.mario.ready() {
            return;
        }
        let bump = if selected { 2.0 * pulse(self.now) } else { 0.0 };
        layout::draw_text(&mut self.list, &c.view, &a.mario, "<", x0 - bump, y, 34.0, 0xFFF0_00FF, 0x5AC8_32FF, c.alpha, 1, 1);
        layout::draw_text(&mut self.list, &c.view, &a.mario, ">", x1 + bump, y, 34.0, 0xFFF0_00FF, 0x5AC8_32FF, c.alpha, 1, 1);
    }

    /// A value between two arrows across the column.
    #[allow(clippy::too_many_arguments)]
    fn picker(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, key: i32, label: &str, y: f32, h: f32, selected: bool, text_size: f32) {
        self.pause_button(ctx, a, c, key, label, COL_X, y, COL_W - 2.0 * ARROW_W, h, selected, true, text_size, false, 0.0, false);
        // The arrows' outer edges at the column's ends.
        self.picker_arrows(a, c, COL_X0 + 10.0, COL_X1 - 10.0, y, selected);
        if selected {
            self.column_cursor(ctx, a, c, y, h);
        }
    }

    /// A slider from the pause window's button: a dark track, filled from
    /// the left in the button's on colours.
    #[allow(clippy::too_many_arguments)]
    fn slider_bar(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, x0: f32, x1: f32, y: f32, h: f32, percent: u32, selected: bool) {
        let Some(pause) = &a.pause else { return };
        let (tx, ty, _, _) = pause.pane_transform("P_SBBase_00").unwrap_or((0.0, 0.0, 1.0, 1.0));
        let (w, cx) = (x1 - x0, (x0 + x1) / 2.0);
        let mut p = DrawParams::new();
        p.alpha = c.alpha;
        p.pane_scale.insert("P_SBBase_00".into(), 1.0);
        for hidden in ["W_SButton_0", "P_shadow_00", "T_tuzukeru_00", "T_tuzukeru_01", "Btn1_Left", "Btn1_Middle", "Btn1_Right"] {
            p.visible.insert(hidden.into(), false);
        }
        p.offset_x = cx - tx;
        p.offset_y = y - ty;
        p.pane_size.insert("P_SBBase_00".into(), (w, h));
        p.vertex_colors.insert("P_SBBase_00".into(), [0x2830_28E8, 0x2830_28E8, 0x4858_48E8, 0x4858_48E8]);
        pause.draw(ctx, &mut self.list, &c.view, "P_SBBase_00", &p);
        let fill = w * percent.min(100) as f32 / 100.0;
        if fill > 0.5 {
            p.offset_x = x0 + fill / 2.0 - tx;
            p.pane_size.insert("P_SBBase_00".into(), (fill, h));
            p.vertex_colors.insert("P_SBBase_00".into(), if selected { [0x0096_46FF, 0x0096_46FF, 0xF0FF_32FF, 0xF0FF_32FF] } else { [0x8CDC_5AFF, 0x8CDC_5AFF, 0xFFFA_80FF, 0xFFFA_80FF] });
            pause.draw(ctx, &mut self.list, &c.view, "P_SBBase_00", &p);
        }
        if let Some((wx, wy, _, _)) = pause.pane_rect("W_SButton_0") {
            let mut f = DrawParams::new();
            f.alpha = c.alpha;
            f.offset_x = cx - wx;
            f.offset_y = y - wy;
            f.pane_size.insert("W_SButton_0".into(), (w + 7.0, h + 10.0));
            if self.settings_look {
                f.pane_size.insert("W_SButton_0".into(), (w + 7.0, h + 7.0));
                f.frame_clip.insert("W_SButton_0".into(), (3.5, 3.5));
                f.color_matrix.insert("W_SButton_0".into(), [0.0; 9]);
            }
            pause.draw(ctx, &mut self.list, &c.view, "W_SButton_0", &f);
        }
    }

    /// The help line over the prompts, smaller if it would run past the sides.
    fn help_line(&mut self, a: &Assets, c: &Canvas, text: &str, color: u32) {
        let mut size = 22.0;
        if a.message.ready() && !text.is_empty() {
            let width = layout::measure_text(&a.message, text, size);
            let room = WIDTH - 40.0;
            if width > room {
                size *= room / width;
            }
        }
        self.text(a, c, text, 0.0, NOTE_Y, size, 1, color);
    }

    /// The prompt line, bottom right: the Wii Remote button's picture and its word.
    fn prompt_line(&mut self, a: &Assets, c: &Canvas, right: f32, y: f32, prompts: &[(&str, &str, u32)], scale: f32) {
        if !a.message.ready() {
            return;
        }
        let (size, icon, gap) = (24.0 * scale, 30.0 * scale, 22.0 * scale);
        let items: Vec<(f32, f32)> = prompts.iter().map(|(code, word, _)| (layout::measure_text(&a.picture, code, icon), layout::measure_text(&a.message, word, size))).collect();
        let total: f32 = items.iter().map(|(i, w)| i + 4.0 + w + gap).sum();
        let mut x = right - total + gap;
        for ((code, word, event), (iw, ww)) in prompts.iter().zip(items) {
            if a.picture.ready() {
                layout::draw_text(&mut self.list, &c.view, &a.picture, code, x, y, icon, u32::MAX, u32::MAX, c.alpha, 0, 1);
            }
            self.text(a, c, word, x + iw + 4.0, y, size, 0, u32::MAX);
            let width = iw + 4.0 + ww;
            if *event != 0 {
                self.add_hit(x + width / 2.0, y, width + 16.0, 40.0 * scale, -1, *event);
            }
            x += width + gap;
        }
    }

    // ------------------------------------------------------------ rows

    fn label(&mut self, a: &Assets, c: &Canvas, label: &str, y: f32, cursor: i32, size: f32) {
        let color = if self.cursor == cursor { u32::MAX } else { LABEL_DIM };
        self.text(a, c, label, LABEL_X, y, size, 0, color);
    }

    #[allow(clippy::too_many_arguments)]
    fn row_button(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, rows: i32, index: i32, label: &str, value: &str, cursor: i32, enabled: bool, label_scale: f32, dropdown: bool) -> f32 {
        let (step, h, size) = (row_step(rows), row_h(rows), row_text(rows));
        let y = rows_top(rows) - index as f32 * step;
        self.row_y[cursor as usize] = y;
        self.row_h = h;
        self.label(a, c, label, y, cursor, size * label_scale);
        self.pause_button(ctx, a, c, cursor, value, COL_X, y, COL_W, h, self.cursor == cursor, enabled, size, false, 0.0, true);
        self.add_hit(COL_X, y, COL_W + 6.0, step - 2.0, cursor, EV_CONFIRM);
        if dropdown {
            let s = h.min(30.0) * 0.22;
            let x = COL_X1 - 16.0;
            self.list.triangle(ctx, &c.view, [(x - s, y + s * 0.6), (x + s, y + s * 0.6), (x, y - s * 0.8)], Rgba { r: 0x10 as f32 / 255.0, g: 0x40 as f32 / 255.0, b: 0x10 as f32 / 255.0, a: 230.0 / 255.0 * c.alpha });
        }
        y
    }

    #[allow(clippy::too_many_arguments)]
    fn row_toggle(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, rows: i32, index: i32, label: &str, on: bool, cursor: i32) {
        let (step, h, size) = (row_step(rows), row_h(rows), row_text(rows));
        let y = rows_top(rows) - index as f32 * step;
        self.row_y[cursor as usize] = y;
        self.label(a, c, label, y, cursor, size);
        self.on_off(ctx, a, c, cursor, y, h, on, self.cursor == cursor);
        self.add_hit(COL_X, y, COL_W + 6.0, step - 2.0, cursor, EV_CONFIRM);
    }

    // ------------------------------------------------------------ tabs

    fn draw_tabs(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas) {
        let n = TABS.len();
        let step = 744.0 / n as f32;
        let w = step - 8.0;
        let margin = 8.0;
        let mut size = 30.0;
        if a.message.ready() {
            for tab in TABS {
                let name = tab.name();
                let (room, width) = (w - margin, layout::measure_text(&a.message, name, size));
                let spacing = 2.0 * (name.chars().count() - 1) as f32;
                if width + spacing > room && width > 0.0 {
                    size *= ((room - spacing) / width).max(0.3);
                }
            }
        }
        self.text_margin = margin;
        for (i, tab) in TABS.into_iter().enumerate() {
            let x = (i as f32 - (n - 1) as f32 / 2.0) * step;
            let selected = self.cursor == 0 && self.tab == i;
            self.pause_button(ctx, a, c, TAB_KEY + i as i32, tab.name(), x, TABS_Y, w, TABS_H, selected, true, size / TEXT_GROW, self.tab == i, 0.0, true);
            self.add_hit(x, TABS_Y, step - 1.0, TABS_H + 10.0, 0, EV_TAB0 << i);
        }
        self.text_margin = 16.0;
        self.rule(ctx, c, -WIDTH / 2.0 + 14.0, WIDTH / 2.0 - 14.0, TABS_RULE_Y);
    }

    fn draw_display(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, s: &Settings) {
        let rows = DISPLAY_LAST;
        self.row_button(ctx, a, c, rows, 0, "Resolution", &scale_label(s.scale), ROW_RESOLUTION, true, 1.0, true);
        self.row_button(ctx, a, c, rows, 1, "Aspect Ratio", if s.widescreen { "16:9" } else { "Original 4:3" }, ROW_ASPECT, true, 1.0, true);
        self.row_toggle(ctx, a, c, rows, 2, "Fullscreen", s.fullscreen, ROW_FULLSCREEN);
        self.row_toggle(ctx, a, c, rows, 3, "V-Sync", s.vsync, ROW_VSYNC);
        // Performance Monitor: On / Off, then its size.
        let (step, h, size) = (row_step(rows), row_h(rows), row_text(rows));
        let y = rows_top(rows) - 4.0 * step;
        self.row_y[ROW_GRAPH as usize] = y;
        let selected = self.cursor == ROW_GRAPH;
        self.label(a, c, "Performance Monitor", y, ROW_GRAPH, size * 0.92);
        self.pause_button(ctx, a, c, ROW_GRAPH, if s.perf_monitor { "On" } else { "Off" }, COL_X0 + PERF_BUTTON_W / 2.0, y, PERF_BUTTON_W, h, selected, true, size, s.perf_monitor, 0.0, false);
        self.add_hit(COL_X0 + PERF_BUTTON_W / 2.0, y, PERF_BUTTON_W, step - 2.0, ROW_GRAPH, EV_CONFIRM);
        let fill = ((s.perf_size.clamp(5, 200) - 5) * 100 / 195).min(100);
        self.slider_bar(ctx, a, c, PERF_SLIDER_X0, COL_X1, y, h, fill, selected);
        self.mario_number(a, c, &format!("{}%", s.perf_size), (PERF_SLIDER_X0 + COL_X1) / 2.0, y, size);
        if selected {
            self.column_cursor(ctx, a, c, y, h);
        }
        let note = match self.cursor {
            0 => Tab::Display.help(),
            ROW_RESOLUTION if s.scale == 0 => "The window's own resolution (from the next start).",
            ROW_RESOLUTION => "Rendered at this many lines, then scaled to the screen (from the next start).",
            ROW_ASPECT => "Switching between 4:3 and widescreen takes effect after a restart.",
            ROW_FULLSCREEN if s.fullscreen => "Borderless: a window covering the whole screen (F11).",
            ROW_FULLSCREEN => "Off: the game plays in a window.",
            ROW_VSYNC => "No tearing: frames wait for the display, the game keeps its pace.",
            _ => "Shows the FPS and each frame's time at the top left; left and right set its size.",
        };
        self.help_line(a, c, note, u32::MAX);
    }

    fn draw_graphics(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, s: &Settings) {
        let rows = 5;
        self.row_button(ctx, a, c, rows, 0, "Renderer", "Vulkan", ROW_RENDERER, false, 1.0, false);
        self.row_toggle(ctx, a, c, rows, 1, "Battery Saver", s.battery_saver, ROW_BATTERY);
        self.row_button(ctx, a, c, rows, 2, "Frame Interpolation", &rate_label(s.fps), ROW_INTERPOLATION, true, 0.92, true);
        let note = match self.cursor {
            0 => Tab::Graphics.help(),
            ROW_RENDERER => "This port draws with Vulkan.",
            ROW_BATTERY => "Shows 30 frames a second while the game keeps running at 60.",
            _ if s.fps == 0 => "Draws as many frames in between as the PC can, the game still at its own speed.",
            _ => "Draws frames in between for smoother motion on screens above 60Hz.",
        };
        self.help_line(a, c, note, u32::MAX);
    }

    fn draw_gameplay(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, s: &Settings) {
        let rows = GAMEPLAY_LAST;
        self.row_toggle(ctx, a, c, rows, 0, "No Time Limit", s.no_time_limit, ROW_TIME);
        self.row_toggle(ctx, a, c, rows, 1, "Infinite Lives", s.infinite_lives, ROW_LIVES);
        // Power-Up: picked with the arrows, given with a press.
        let (step, h, size) = (row_step(rows), row_h(rows), row_text(rows));
        let y = rows_top(rows) - 2.0 * step;
        self.row_y[ROW_POWER_UP as usize] = y;
        let name = POWER_UPS[(s.power_up as usize).min(POWER_UPS.len() - 1)];
        self.label(a, c, "Power-Up", y, ROW_POWER_UP, size);
        self.picker(ctx, a, c, ROW_POWER_UP, name, y, h, self.cursor == ROW_POWER_UP, if name.len() > 14 { size.min(22.0) } else { size });
        self.add_hit(COL_X0 + ARROW_W / 2.0, y, ARROW_W + 8.0, step - 2.0, ROW_POWER_UP, EV_LEFT);
        self.add_hit(COL_X1 - ARROW_W / 2.0, y, ARROW_W + 8.0, step - 2.0, ROW_POWER_UP, EV_RIGHT);
        self.add_hit(COL_X, y, COL_W - 2.0 * ARROW_W - 8.0, step - 2.0, ROW_POWER_UP, EV_CONFIRM);
        self.row_toggle(ctx, a, c, rows, 3, "Autosave", s.autosave, ROW_AUTOSAVE);
        self.row_toggle(ctx, a, c, rows, 4, "Exit Course", s.exit_course, ROW_EXIT_COURSE);
        self.row_toggle(ctx, a, c, rows, 5, "P1 Character", s.p1_character, ROW_P1_CHARACTER);
        let given = self.power_up_at.is_some_and(|t| self.now - t < 3.0);
        let note = match self.cursor {
            ROW_TIME => "The course timer stands still: take your time.",
            ROW_LIVES => "A miss costs no life.",
            ROW_POWER_UP if given => "Player 1 gets it in the next course or on the world map.",
            ROW_POWER_UP => "Left and right pick one; press it to give it to player 1.",
            ROW_AUTOSAVE => "The game is saved after every course you clear, as the map's Save does.",
            ROW_EXIT_COURSE => "The pause menu's Exit Course in every course, cleared or not.",
            ROW_P1_CHARACTER => "Player 1 picks a character too, on the game's own screen (from the next start).",
            _ => Tab::Gameplay.help(),
        };
        self.help_line(a, c, note, u32::MAX);
    }

    fn volume_y(row: usize) -> f32 {
        78.0 - 54.0 * row as f32
    }

    fn draw_audio(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, s: &mut Settings, changes: &mut Vec<Change>) {
        // A press on a slider (a little past either end counts) drags it.
        let reach = 24.0;
        if let (Some((px, py)), true) = (self.pointer, self.pressed) {
            for row in 0..VOLUMES.len() {
                if px >= COL_X0 - reach && px <= COL_X1 + reach && (py - Self::volume_y(row)).abs() <= 27.0 {
                    self.drag_slider = Some(row);
                    self.cursor = 1 + row as i32;
                }
            }
        }
        if let Some(row) = self.drag_slider {
            if let (Some((px, _)), true) = (self.pointer, self.pressed || self.held) {
                let t = ((px - COL_X0) / (COL_X1 - COL_X0)).clamp(0.0, 1.0);
                let percent = (t * 100.0).round() as u32;
                if percent != s.volume[row] {
                    s.volume[row] = percent;
                    changes.push(Change::Volume);
                }
            } else {
                self.drag_slider = None;
            }
        }
        for (row, (label, _)) in VOLUMES.iter().enumerate() {
            let y = Self::volume_y(row);
            let selected = self.cursor == 1 + row as i32;
            let color = if selected { u32::MAX } else { LABEL_DIM };
            self.text(a, c, label, LABEL_X, y, 28.0, 0, color);
            self.slider_bar(ctx, a, c, COL_X0, COL_X1, y, SLIDER_H, s.volume[row], selected);
            self.mario_number(a, c, &format!("{}%", s.volume[row]), COL_X, y, 26.0);
            if selected {
                self.column_cursor(ctx, a, c, y, SLIDER_H);
            }
            self.add_hit(-10.0, y, 660.0, 56.0, 1 + row as i32, 0);
        }
        let row = self.cursor - 1;
        let note = if (0..VOLUMES.len() as i32).contains(&row) { VOLUMES[row as usize].1 } else { Tab::Audio.help() };
        self.help_line(a, c, note, u32::MAX);
    }

    fn draw_keybinds(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, s: &Settings) {
        if self.capturing.is_some_and(|(_, at)| self.now - at > CAPTURE_TIMEOUT) {
            self.capturing = None;
        }
        for (i, (_, name)) in input::keymap::ACTIONS.iter().enumerate() {
            let (column, row) = (i / 6, i % 6);
            let x0 = if column == 0 { -190.0 } else { 190.0 };
            let y = 86.0 - 31.0 * row as f32;
            let color = if self.cursor == 1 + i as i32 { u32::MAX } else { LABEL_DIM };
            self.text(a, c, name, x0 - 168.0, y, 21.0, 0, color);
            let keys = match s.keymap.label(i) {
                _ if self.capturing.is_some_and(|(r, _)| r == i) => "Press a key...".to_string(),
                l if l.is_empty() => "None".to_string(),
                l => l,
            };
            let size = if keys.len() > 16 { 17.0 } else if keys.len() > 11 { 19.0 } else { 22.0 };
            let waiting = self.capturing.is_some_and(|(r, _)| r == i);
            self.pause_button(ctx, a, c, 1 + i as i32, &keys, x0 + 78.0, y, 180.0, 23.0, self.cursor == 1 + i as i32, true, size, waiting, 0.0, true);
            self.add_hit(x0 + 78.0, y, 186.0, 31.0, 1 + i as i32, EV_CONFIRM);
        }
        let note = match (self.capturing, self.cursor) {
            (Some(_), _) => "Press the new key. Esc cancels; Delete or Backspace leaves it no key.",
            (None, 0) => Tab::Keybinds.help(),
            (None, n) if (1..=12).contains(&n) => KEY_HELP[n as usize - 1],
            _ => "",
        };
        self.help_line(a, c, note, u32::MAX);
        self.text(a, c, "Controllers: A jumps, B and X run, Y shakes, Start is +, Back is -.", 0.0, -102.0, 20.0, 1, 0xFFF0_B0FF);
    }

    fn draw_language(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, s: &Settings, at_boot: &str) {
        for (i, (code, name, _)) in LANGUAGES.iter().enumerate() {
            let y = 78.0 - 54.0 * i as f32;
            self.pause_button(ctx, a, c, 1 + i as i32, name, 0.0, y, 420.0, 42.0, self.cursor == 1 + i as i32, true, 30.0, s.language == *code, 0.0, true);
            self.add_hit(0.0, y, 440.0, 52.0, 1 + i as i32, EV_CONFIRM);
        }
        let row = self.cursor - 1;
        if !(0..LANGUAGES.len() as i32).contains(&row) {
            self.help_line(a, c, Tab::Language.help(), u32::MAX);
            return;
        }
        let (code, name, restart) = LANGUAGES[row as usize];
        if s.language == code && code != at_boot {
            self.help_line(a, c, restart, 0xFFF0_B0FF);
        } else {
            self.help_line(a, c, &format!("Shows the game's text and these menus in {name}."), u32::MAX);
        }
    }

    fn open_dropdown(&mut self, kind: DropKind, s: &Settings) {
        let (row, labels, current): (i32, Vec<String>, i32) = match kind {
            DropKind::Resolution => (ROW_RESOLUTION, SCALES.iter().map(|&x| scale_label(x)).collect(), SCALES.iter().position(|&x| x == s.scale).map_or(-1, |i| i as i32)),
            DropKind::Aspect => (ROW_ASPECT, vec!["Original 4:3".into(), "16:9".into()], i32::from(s.widescreen)),
            DropKind::Interpolation => (ROW_INTERPOLATION, FRAME_RATES.iter().map(|&f| rate_label(f)).collect(), FRAME_RATES.iter().position(|&f| f == s.fps).map_or(-1, |i| i as i32)),
        };
        self.dropdown = Some(Dropdown { row, highlight: current.max(0) as usize, current, labels, kind, y: self.row_y[row as usize], row_h: self.row_h });
    }

    fn pick(&mut self, kind: DropKind, item: usize, s: &mut Settings, changes: &mut Vec<Change>) {
        match kind {
            DropKind::Resolution => {
                s.scale = SCALES[item];
                changes.push(Change::Scale);
            }
            DropKind::Aspect => {
                s.widescreen = item == 1;
                changes.push(Change::Aspect);
            }
            DropKind::Interpolation => {
                s.fps = FRAME_RATES[item];
                changes.push(Change::Fps);
            }
        }
    }

    fn draw_dropdown(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas) {
        let Some(d) = &self.dropdown else { return };
        let (labels, current, highlight, dy, drow_h) = (d.labels.clone(), d.current, d.highlight, d.y, d.row_h);
        let count = labels.len();
        const ITEM_H: f32 = 29.0;
        const PITCH: f32 = ITEM_H + 3.5;
        let mut first = dy - drow_h / 2.0 - 10.0 - ITEM_H / 2.0;
        let lowest = -HEIGHT / 2.0 + 24.0;
        if first - (count - 1) as f32 * PITCH - ITEM_H / 2.0 < lowest {
            first = lowest + (count - 1) as f32 * PITCH + ITEM_H / 2.0;
        }
        let (x0, x1) = (COL_X0 - 6.0, COL_X1 + 6.0);
        let (top, bottom) = (first + ITEM_H / 2.0 + 7.0, first - (count - 1) as f32 * PITCH - ITEM_H / 2.0 - 7.0);
        let edge = 2.0 / c.view.scale.max(0.01);
        self.list.rect(ctx, &c.view, x0 - edge, top + edge, x1 + edge, bottom - edge, Rgba { r: 1.0, g: 1.0, b: 1.0, a: 140.0 / 255.0 * c.alpha });
        self.list.rect(ctx, &c.view, x0, top, x1, bottom, Rgba { r: 0x18 as f32 / 255.0, g: 0x18 as f32 / 255.0, b: 0x20 as f32 / 255.0, a: 235.0 / 255.0 * c.alpha });
        self.hits.clear();
        for (i, label) in labels.iter().enumerate() {
            let y = first - i as f32 * PITCH;
            self.pause_button(ctx, a, c, DROPDOWN_HIT + i as i32, label, COL_X, y, COL_W, ITEM_H, false, true, 25.0, i as i32 == current, 0.0, false);
            self.add_hit(COL_X, y, COL_W + 6.0, PITCH, DROPDOWN_HIT + i as i32, EV_CONFIRM);
        }
        self.add_hit(0.0, 0.0, 4000.0, 4000.0, -1, EV_BACK);
        let y = first - highlight as f32 * PITCH;
        self.cursor_box(ctx, a, c, COL_X, y, COL_W + 6.0, ITEM_H + 6.0);
    }

    // ------------------------------------------------------------ the cheat question

    /// Turning Infinite Lives on and giving the Power-Up ask first (No Time
    /// Limit doesn't): "Are you sure? This will mark your save as cheated."
    /// A save already marked isn't asked about again.
    fn ask_cheat(&mut self, cheat: Cheat, s: &mut Settings, changes: &mut Vec<Change>) {
        let marked = usize::try_from(self.active_file).ok().and_then(|f| s.cheated.get(f)).copied().unwrap_or(false);
        if marked || !self.can_ask {
            self.do_cheat(cheat, s, changes);
            return;
        }
        self.ask = Some((Ask::Cheat(cheat), self.cursor));
        self.ask_cursor = 1; // No first
    }

    fn do_cheat(&mut self, cheat: Cheat, s: &mut Settings, changes: &mut Vec<Change>) {
        match cheat {
            Cheat::Lives => {
                s.infinite_lives = true;
                changes.push(Change::Gameplay);
                // The Power-Up marks the save once it is given.
                if let Ok(file @ 0..=2) = u8::try_from(self.active_file) {
                    changes.push(Change::Cheated(file));
                }
            }
            Cheat::PowerUp => {
                self.power_up_at = Some(self.now);
                changes.push(Change::PowerUp(s.power_up.min(POWER_UPS.len() as u32 - 1) as u8));
            }
        }
    }

    /// The question over the window: Yes and No, the only things that take
    /// a press while it is up.
    fn draw_ask(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, screen: egui::Rect, ask: Ask) {
        const HEIGHT: f32 = 250.0;
        let (title, line) = match ask {
            Ask::Cheat(_) => ("Are you sure?", "This will mark your save as cheated.".to_string()),
            Ask::Update => match &self.update {
                Some((v, true)) => ("Update available", format!("Version {v} is out. Update and restart the game?")),
                Some((v, false)) => ("Update available", format!("Version {v} is out. Open its web page?")),
                None => ("Update available", String::new()),
            },
        };
        self.hits.clear();
        self.list.fill_screen(ctx, screen, Rgba { r: 0.0, g: 0.0, b: 0.0, a: 110.0 / 255.0 * c.alpha });
        self.small_window(ctx, a, c, 600.0, HEIGHT);
        self.text(a, c, title, 0.0, 62.0, 32.0, 1, u32::MAX);
        // Smaller if it would run past the window's sides.
        let mut size = 24.0;
        if a.message.ready() {
            let width = layout::measure_text(&a.message, &line, size);
            if width > 560.0 {
                size *= 560.0 / width;
            }
        }
        self.text(a, c, &line, 0.0, 16.0, size, 1, u32::MAX);
        if matches!((ask, &self.update), (Ask::Update, Some((_, true)))) {
            self.text(a, c, "Progress since your last save will be lost.", 0.0, -24.0, 20.0, 1, LABEL_DIM);
        }
        // Yes and No keep one size, chosen or not.
        let y = -HEIGHT / 2.0 + 43.0;
        self.button_scale = Some(1.0);
        for (i, (label, x)) in [("Yes", -126.0), ("No", 126.0)].into_iter().enumerate() {
            self.yes_no_button(ctx, a, c, label, x, y, 232.0, 53.0, self.ask_cursor == i as i32, true, 30.0);
            self.add_hit(x, y, 240.0, 60.0, ASK_HIT + i as i32, EV_CONFIRM);
        }
        self.button_scale = None;
    }

    /// The update being installed: its progress and Cancel, or why it
    /// failed and OK.
    fn draw_update(&mut self, ctx: &egui::Context, a: &Assets, c: &Canvas, screen: egui::Rect, state: &UpdateState) {
        const HEIGHT: f32 = 250.0;
        let version = self.update.as_ref().map(|u| u.0.clone()).unwrap_or_default();
        self.hits.clear();
        self.list.fill_screen(ctx, screen, Rgba { r: 0.0, g: 0.0, b: 0.0, a: 110.0 / 255.0 * c.alpha });
        self.small_window(ctx, a, c, 600.0, HEIGHT);
        let (title, line, button) = match state {
            UpdateState::Downloading { done, total } => {
                let percent = if *total > 0 { (done * 100 / total).min(100) as u32 } else { 0 };
                self.slider_bar(ctx, a, c, -240.0, 240.0, -28.0, 26.0, percent, false);
                ("Updating", format!("Downloading version {version}: {percent}%"), Some("Cancel"))
            }
            UpdateState::Installing => ("Updating", format!("Installing version {version}..."), None),
            UpdateState::Restarting => ("Updating", "Done. The game is restarting.".to_string(), None),
            UpdateState::Failed(e) => {
                let mut chars = e.chars();
                let reason: String = chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default();
                ("The update failed", format!("{reason}."), Some("OK"))
            }
        };
        self.text(a, c, title, 0.0, 62.0, 32.0, 1, u32::MAX);
        let mut size = 24.0;
        if a.message.ready() {
            let width = layout::measure_text(&a.message, &line, size);
            if width > 560.0 {
                size *= 560.0 / width;
            }
        }
        self.text(a, c, &line, 0.0, 16.0, size, 1, u32::MAX);
        if let Some(label) = button {
            let y = -HEIGHT / 2.0 + 43.0;
            self.button_scale = Some(1.0);
            self.yes_no_button(ctx, a, c, label, 0.0, y, 232.0, 53.0, true, true, 30.0);
            self.add_hit(0.0, y, 240.0, 60.0, -1, EV_CONFIRM);
            self.button_scale = None;
        }
    }

    // ------------------------------------------------------------ input

    fn handle(&mut self, nav: Nav, s: &mut Settings, changes: &mut Vec<Change>) {
        let Nav { left, right, up, down, confirm, back } = nav;
        // An update being installed takes every press: Cancel, or OK.
        if let Some(state) = &self.update_state {
            if confirm || back {
                match state {
                    UpdateState::Downloading { .. } => changes.push(Change::UpdateCancel),
                    UpdateState::Failed(_) => changes.push(Change::UpdateDismiss),
                    _ => {}
                }
            }
            return;
        }
        if let Some((ask, row)) = self.ask {
            // A press on Yes or No picks it.
            if self.cursor >= ASK_HIT {
                self.ask_cursor = (self.cursor - ASK_HIT).clamp(0, 1);
            }
            self.cursor = row;
            if left || right {
                self.ask_cursor = 1 - self.ask_cursor;
            } else if confirm || back {
                self.ask = None;
                if confirm && self.ask_cursor == 0 {
                    match ask {
                        Ask::Cheat(cheat) => self.do_cheat(cheat, s, changes),
                        Ask::Update if self.update.as_ref().is_some_and(|u| u.1) => changes.push(Change::UpdateInstall),
                        Ask::Update => changes.push(Change::UpdatePage),
                    }
                }
            }
            return;
        }
        if let Some(d) = &mut self.dropdown {
            let count = d.labels.len();
            if self.cursor >= DROPDOWN_HIT {
                d.highlight = ((self.cursor - DROPDOWN_HIT) as usize).min(count - 1);
                self.cursor = d.row;
            }
            if up {
                d.highlight = d.highlight.saturating_sub(1);
            }
            if down {
                d.highlight = (d.highlight + 1).min(count - 1);
            }
            if confirm {
                let (kind, item) = (d.kind, d.highlight);
                self.dropdown = None;
                self.pick(kind, item, s, changes);
            } else if back {
                self.dropdown = None;
            }
            return;
        }
        if confirm && self.cursor > 0 {
            self.anims.hit = self.cursor;
            self.anims.hit_at = self.now;
        }
        if self.cursor == 0 {
            if left || right {
                self.tab = (self.tab as i32 + if left { -1 } else { 1 }).clamp(0, TABS.len() as i32 - 1) as usize;
            } else if confirm {
                self.tab = (self.tab + 1) % TABS.len();
            } else if down {
                self.cursor = 1;
            } else if back {
                self.close(self.now);
            }
            return;
        }
        if back {
            self.close(self.now);
            return;
        }
        let last = match self.tab() {
            Tab::Display => DISPLAY_LAST,
            Tab::Graphics => GRAPHICS_LAST,
            Tab::Audio => VOLUMES.len() as i32,
            Tab::Keybinds => 12,
            Tab::Gameplay => GAMEPLAY_LAST,
            Tab::Language => LANGUAGES.len() as i32,
        };
        if up {
            self.cursor -= 1;
        }
        if down {
            self.cursor = (self.cursor + 1).min(last);
        }
        let toggle = confirm || left || right;
        match (self.tab(), self.cursor) {
            (Tab::Display, ROW_RESOLUTION) if confirm => self.open_dropdown(DropKind::Resolution, s),
            (Tab::Display, ROW_ASPECT) if confirm => self.open_dropdown(DropKind::Aspect, s),
            (Tab::Display, ROW_FULLSCREEN) if toggle => {
                s.fullscreen = !s.fullscreen;
                changes.push(Change::Fullscreen);
            }
            (Tab::Display, ROW_VSYNC) if toggle => {
                s.vsync = !s.vsync;
                changes.push(Change::VSync);
            }
            (Tab::Display, ROW_GRAPH) if confirm => {
                s.perf_monitor = !s.perf_monitor;
                changes.push(Change::PerfMonitor);
            }
            (Tab::Display, ROW_GRAPH) if left || right => {
                s.perf_size = (s.perf_size as i32 + if left { -5 } else { 5 }).clamp(5, 200) as u32;
                changes.push(Change::PerfMonitor);
            }
            (Tab::Graphics, ROW_BATTERY) if toggle => {
                s.battery_saver = !s.battery_saver;
                changes.push(Change::BatterySaver);
            }
            (Tab::Graphics, ROW_INTERPOLATION) if confirm => self.open_dropdown(DropKind::Interpolation, s),
            (Tab::Audio, n) if (1..=4).contains(&n) && (left || right) => {
                let v = &mut s.volume[n as usize - 1];
                // Steps of 5, landing on a multiple of 5.
                *v = if left { v.saturating_sub(1) / 5 * 5 } else { ((*v + 5) / 5 * 5).min(100) };
                changes.push(Change::Volume);
            }
            (Tab::Keybinds, n) if confirm && (1..=12).contains(&n) => self.capturing = Some((n as usize - 1, self.now)),
            (Tab::Gameplay, ROW_TIME) if toggle => {
                s.no_time_limit = !s.no_time_limit;
                changes.push(Change::Gameplay);
            }
            (Tab::Gameplay, ROW_LIVES) if toggle && s.infinite_lives => {
                s.infinite_lives = false;
                changes.push(Change::Gameplay);
            }
            (Tab::Gameplay, ROW_LIVES) if toggle => self.ask_cheat(Cheat::Lives, s, changes),
            (Tab::Gameplay, ROW_POWER_UP) if left || right => {
                let n = POWER_UPS.len() as u32;
                s.power_up = if left { (s.power_up + n - 1) % n } else { (s.power_up + 1) % n };
                self.power_up_at = None;
                changes.push(Change::Gameplay);
            }
            (Tab::Gameplay, ROW_POWER_UP) if confirm => self.ask_cheat(Cheat::PowerUp, s, changes),
            (Tab::Gameplay, ROW_AUTOSAVE) if toggle => {
                s.autosave = !s.autosave;
                changes.push(Change::Gameplay);
            }
            (Tab::Gameplay, ROW_EXIT_COURSE) if toggle => {
                s.exit_course = !s.exit_course;
                changes.push(Change::Gameplay);
            }
            (Tab::Gameplay, ROW_P1_CHARACTER) if toggle => {
                s.p1_character = !s.p1_character;
                changes.push(Change::Gameplay);
            }
            (Tab::Language, n) if confirm && (1..=3).contains(&n) => {
                s.language = LANGUAGES[n as usize - 1].0.into();
                changes.push(Change::Language);
            }
            _ => {}
        }
    }

    /// Mouse and touch: a press on one of this frame's buttons picks it.
    fn pointer_events(&mut self) -> Nav {
        let mut nav = Nav::default();
        let (Some((x, y)), true) = (self.pointer, self.pressed) else { return nav };
        let Some(hit) = self.hits.iter().rev().find(|h| (x - h.x).abs() <= h.w / 2.0 && (y - h.y).abs() <= h.h / 2.0) else { return nav };
        if hit.cursor >= 0 {
            self.cursor = hit.cursor;
        }
        match hit.event {
            EV_CONFIRM => nav.confirm = true,
            EV_BACK => nav.back = true,
            EV_LEFT => nav.left = true,
            EV_RIGHT => nav.right = true,
            e if e >= EV_TAB0 => {
                self.tab = (e / EV_TAB0).trailing_zeros() as usize;
                self.cursor = 0;
            }
            _ => {}
        }
        nav
    }

    // ------------------------------------------------------------ the window

    /// Scale from the pause window's open / close animations (1 when shown).
    fn window_scale(&mut self, a: &Assets, pause: &Layout) -> Option<f32> {
        let anim = if self.closing { &a.out_window } else { &a.in_window };
        let Some(anim) = anim else {
            if self.closing {
                self.open = false;
                return None;
            }
            return Some(1.0);
        };
        let since = self.now - if self.closing { self.closing_at } else { self.opened_at };
        let frame = (since * 60.0) as f32;
        if self.closing && frame >= anim.frames() {
            self.open = false;
            return None;
        }
        let mut p = DrawParams::new();
        anim.apply(frame.min(anim.frames()), pause, &mut p, &[("N_pauseMenu_00", "N_pauseMenu_00")]);
        Some(p.pane_scale.get("N_pauseMenu_00").map_or(1.0, |s| s.clamp(0.0, 1.2)))
    }

    /// Draws the window and takes this frame's moves; returns the shapes.
    pub fn draw(&mut self, ctx: &egui::Context, s: &mut Settings, nav: Nav, now: f64, changes: &mut Vec<Change>) -> Vec<egui::Shape> {
        static AT_BOOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        let at_boot = AT_BOOT.get_or_init(|| s.language.clone()).clone();
        self.now = now;
        self.list = DrawList::default();
        let a = self.assets.get_or_insert_with(|| Arc::new(Assets::load(ctx))).clone();
        let screen = ctx.content_rect();
        // Sized for a 16:9 screen: its height on a wider one, its width on a narrower one.
        let unit = (screen.height() / 456.0).min(screen.width() / SCREEN_W);
        let mut c = Canvas { view: View { center_x: screen.center().x, center_y: screen.center().y, scale: unit }, alpha: 1.0 };
        // The pointer in layout units.
        // Where the pointer last was, even once it has left the window (a
        // touch lifts off; a click can be pressed and let go in one frame).
        let (pos, pressed, held) = ctx.input(|i| (i.pointer.interact_pos(), i.pointer.primary_pressed(), i.pointer.primary_down()));
        self.pointer = pos.map(|p| ((p.x - c.view.center_x) / unit, (c.view.center_y - p.y) / unit));
        self.pressed = pressed;
        self.held = held;
        // Moves from last frame's buttons (hits), then the remote's.
        let mut moves = self.pointer_events();
        moves.left |= nav.left;
        moves.right |= nav.right;
        moves.up |= nav.up;
        moves.down |= nav.down;
        moves.confirm |= nav.confirm;
        moves.back |= nav.back;
        self.can_ask = a.yes_no.is_some();
        // A newer release: asked about once, the first time the window is up.
        if self.update.is_some() && !self.update_asked && self.can_ask && !self.closing && self.ask.is_none() && self.dropdown.is_none() && self.capturing.is_none() {
            self.update_asked = true;
            self.ask = Some((Ask::Update, self.cursor));
            self.ask_cursor = 0;
        }
        if !self.closing && self.capturing.is_none() {
            self.handle(moves, s, changes);
        }
        self.hits.clear();
        if let Some(pause) = &a.pause {
            let Some(scale) = self.window_scale(&a, pause) else { return Vec::new() };
            let dim = if self.closing { scale.min(1.0) } else { 1.0 };
            self.list.fill_screen(ctx, screen, Rgba { r: 0.0, g: 0.0, b: 0.0, a: 130.0 / 255.0 * dim });
            c.view.scale *= scale;
            self.panel(ctx, &a, &c, WIDTH, HEIGHT);
            if a.mario.ready() {
                layout::draw_text(&mut self.list, &c.view, &a.mario, "Settings", 0.0, TITLE_Y, 52.0, u32::MAX, u32::MAX, c.alpha, 1, 1);
            }
            self.rule(ctx, &c, -WIDTH / 2.0 + 14.0, WIDTH / 2.0 - 14.0, PROMPT_RULE_Y);
            // The picture font's 2 and 1.
            self.prompt_line(&a, &c, WIDTH / 2.0 - 24.0, PROMPT_Y, &[("(", "Select", 0), ("'", "Back", EV_BACK)], 1.3);
        } else {
            if self.closing {
                self.open = false;
                return Vec::new();
            }
            self.list.fill_screen(ctx, screen, Rgba { r: 0.0, g: 0.0, b: 0.0, a: 110.0 / 255.0 });
        }
        self.defer_cursor = true;
        self.deferred = None;
        self.settings_look = true;
        self.draw_tabs(ctx, &a, &c);
        match self.tab() {
            Tab::Display => self.draw_display(ctx, &a, &c, s),
            Tab::Graphics => self.draw_graphics(ctx, &a, &c, s),
            Tab::Audio => self.draw_audio(ctx, &a, &c, s, changes),
            Tab::Keybinds => self.draw_keybinds(ctx, &a, &c, s),
            Tab::Gameplay => self.draw_gameplay(ctx, &a, &c, s),
            Tab::Language => self.draw_language(ctx, &a, &c, s, &at_boot),
        }
        self.defer_cursor = false;
        if let Some((cc, x, y, w, h)) = self.deferred.take().filter(|_| self.dropdown.is_none()) {
            self.cursor_box(ctx, &a, &cc, x, y, w, h);
        }
        if !matches!(self.tab(), Tab::Display | Tab::Graphics) || self.closing {
            self.dropdown = None;
        }
        self.draw_dropdown(ctx, &a, &c);
        self.settings_look = false;
        if let Some((ask, _)) = self.ask {
            self.draw_ask(ctx, &a, &c, screen, ask);
        }
        if let Some(state) = self.update_state.clone() {
            self.draw_update(ctx, &a, &c, screen, &state);
        }
        std::mem::take(&mut self.list).into_shapes()
    }
}
