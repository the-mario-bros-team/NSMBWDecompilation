//! The port's own menus, drawn in the game's style over the picture: the
//! Settings window (F10), after WitherBlitz's NSMB-Wii-Recompilation
//! (`runtime/src/netplay/netplay_ui.cpp`, `game_layout.cpp`). egui carries
//! the drawing (the game's layouts, fonts and textures as textured meshes)
//! and the mouse; the remote's buttons (keyboard or controller, as mapped
//! for the game) move the cursor.

pub mod layout;
mod settings;

use std::time::Instant;

use hle_ios::wiimote::button;
use winit::window::Window;

use crate::settings::Settings;

/// Menu moves this frame, from the remote held sideways.
#[derive(Default, Clone, Copy, Debug)]
pub struct Nav {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub confirm: bool,
    pub back: bool,
}

impl Nav {
    fn any(&self) -> bool {
        self.left || self.right || self.up || self.down || self.confirm || self.back
    }
}

/// A setting the window changed that the app applies (and saves).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Change {
    Fps,
    VSync,
    Fullscreen,
    Aspect,
    Scale,
    PerfMonitor,
    BatterySaver,
    Volume,
    Language,
    /// No Time Limit or Infinite Lives.
    Gameplay,
    /// Give player 1 this power-up (`gameplay::POWER_UPS`).
    PowerUp(u8),
    /// A key or button was rebound, or a Keybinds-tab switch flipped.
    Keymap,
    /// Mark this save file (0-2) "Cheated": Infinite Lives was turned on
    /// while it is the one being played.
    Cheated(u8),
    /// Open the newer release's web page (Yes to the update question, when
    /// the release has no download for Windows).
    UpdatePage,
    /// Download and install the newer release, then restart (Yes to the
    /// update question).
    UpdateInstall,
    /// Stop the download.
    UpdateCancel,
    /// The failed update's message was read.
    UpdateDismiss,
}

/// An update being installed, as the Settings window shows it.
#[derive(Clone, Debug, PartialEq)]
pub enum UpdateState {
    Downloading { done: u64, total: u64 },
    Installing,
    /// Installed: the game closes and starts again.
    Restarting,
    Failed(String),
}

pub struct Menus {
    ctx: egui::Context,
    winit: egui_winit::State,
    window: settings::SettingsWindow,
    started: Instant,
    /// The remote's buttons at the last frame (presses are the new ones).
    held: u16,
    /// Hold-to-repeat for the D-pad: the direction and when it repeats next.
    repeat: Option<(u16, f64)>,
    /// What was last handed to the renderer: a menu, or nothing.
    shown: bool,
}

impl Menus {
    pub fn new(window: &Window, game: &std::path::Path, language: &str) -> Self {
        let folder = match language {
            "es" => "SpaUS",
            "fr" => "FraUS",
            _ => "EngUS",
        };
        layout::set_disc(game, folder);
        let ctx = egui::Context::default();
        let winit = egui_winit::State::new(ctx.clone(), ctx.viewport_id(), window, Some(window.scale_factor() as f32), None, None);
        let mut settings_window = settings::SettingsWindow::default();
        settings_window.active_file = -1;
        Self { ctx, winit, window: settings_window, started: Instant::now(), held: 0, repeat: None, shown: false }
    }

    /// The save file being played (0-2), or -1 before the save select.
    pub fn set_active_file(&mut self, file: i8) {
        self.window.active_file = file;
    }

    /// A newer release of the port is out: the Settings window asks, the
    /// next time it is up, whether to install it (`can_install`: it has a
    /// download for Windows) or else to open its page.
    pub fn set_update(&mut self, version: &str, can_install: bool) {
        self.window.update = Some((version.into(), can_install));
    }

    /// The update being installed (`None`: none), shown over the Settings
    /// window, which stays open meanwhile.
    pub fn set_update_state(&mut self, state: Option<UpdateState>) {
        if state.is_some() && !self.window.is_open() {
            self.window.open(self.started.elapsed().as_secs_f64());
        }
        self.window.update_state = state;
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// The Keybinds tab waits for a key: key presses go to [`Menus::capture_key`].
    pub fn capturing(&self) -> bool {
        self.window.is_open() && self.window.capturing()
    }

    pub fn capture_key(&mut self, settings: &mut Settings, code: winit::keyboard::KeyCode) -> Vec<Change> {
        self.window.capture_key(settings, code)
    }

    /// Opens the Settings window (a game menu's Settings button).
    pub fn open(&mut self) {
        if !self.window.is_open() {
            self.window.open(self.started.elapsed().as_secs_f64());
        }
    }

    /// Opens the Settings window, or closes it if open.
    pub fn toggle(&mut self) {
        let now = self.started.elapsed().as_secs_f64();
        if self.window.is_open() {
            self.window.close(now);
        } else {
            self.window.open(now);
        }
    }

    /// A window event while a menu is up: true if the menus took it (mouse
    /// moves and clicks), so the game must not.
    pub fn on_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        // Only while a menu is up: egui keeps the events until the next
        // frame of its own, which closed there is none of.
        if !self.is_open() {
            return false;
        }
        let r = self.winit.on_window_event(window, event);
        r.consumed || matches!(event, winit::event::WindowEvent::MouseInput { .. } | winit::event::WindowEvent::CursorMoved { .. } | winit::event::WindowEvent::MouseWheel { .. })
    }

    /// Menu moves from player 1's remote buttons this frame: presses, and
    /// the D-pad repeating while held.
    fn nav(&mut self, buttons: u16, now: f64) -> Nav {
        let pressed = buttons & !self.held;
        self.held = buttons;
        // Sideways: the D-pad's right is screen up, and so on.
        let dirs = [(button::UP, 0), (button::DOWN, 1), (button::RIGHT, 2), (button::LEFT, 3)];
        let mut fire = pressed;
        let dir = dirs.iter().map(|d| d.0).find(|&b| buttons & b != 0);
        match (dir, self.repeat) {
            (Some(b), Some((r, at))) if r == b => {
                if now >= at {
                    fire |= b;
                    self.repeat = Some((b, now + 0.08));
                }
            }
            (Some(b), _) => self.repeat = Some((b, now + 0.4)),
            (None, _) => self.repeat = None,
        }
        Nav {
            left: fire & button::UP != 0,
            right: fire & button::DOWN != 0,
            up: fire & button::RIGHT != 0,
            down: fire & button::LEFT != 0,
            confirm: fire & (button::TWO | button::A) != 0,
            // + too (Esc on the keyboard), as it closes the game's pause window.
            back: fire & (button::ONE | button::B | button::PLUS) != 0,
        }
    }

    /// This frame's menus for the renderer (`None` and nothing to upload
    /// while no menu is up), with what the window changed in `settings`.
    pub fn frame(&mut self, window: &Window, settings: &mut Settings, buttons: u16) -> Option<(Option<vk_render::UiFrame>, egui::TexturesDelta, Vec<Change>)> {
        let now = self.started.elapsed().as_secs_f64();
        if !self.window.is_open() {
            self.held = buttons;
            if self.shown {
                self.shown = false;
                return Some((None, egui::TexturesDelta::default(), Vec::new()));
            }
            return None;
        }
        let nav = self.nav(buttons, now);
        let _ = nav.any();
        let raw = self.winit.take_egui_input(window);
        let mut changes = Vec::new();
        let Self { ctx, window: settings_window, .. } = self;
        let out = ctx.run_ui(raw, |ui| {
            let ctx = ui.ctx();
            let shapes = settings_window.draw(ctx, settings, nav, now, &mut changes);
            ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("menus"))).extend(shapes);
        });
        self.winit.handle_platform_output(window, out.platform_output);
        let primitives = self.ctx.tessellate(out.shapes, out.pixels_per_point);
        self.shown = true;
        Some((Some(vk_render::UiFrame { primitives, pixels_per_point: out.pixels_per_point }), out.textures_delta, changes))
    }
}
