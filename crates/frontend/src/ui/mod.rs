//! The port's own menus, drawn in the game's style over the picture: the
//! Settings window (F10), after WitherBlitz's NSMB-Wii-Recompilation
//! (`runtime/src/netplay/netplay_ui.cpp`, `game_layout.cpp`). egui carries
//! the drawing (the game's layouts, fonts and textures as textured meshes)
//! and the mouse; the remote's buttons (keyboard or controller, as mapped
//! for the game) move the cursor.

mod key_caps;
pub mod layout;
pub mod prompts;
mod settings;
mod startup;
pub mod text;

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
#[derive(Clone, Debug, PartialEq)]
pub enum Change {
    /// The Renderer (from the next start).
    Renderer,
    Fps,
    VSync,
    Fullscreen,
    Aspect,
    Scale,
    PerfMonitor,
    BatterySaver,
    /// Spinning Coins or Depth Blur.
    Graphics,
    Volume,
    Language,
    /// No Time Limit or Infinite Lives.
    Gameplay,
    /// Give player 1 this power-up (`gameplay::POWER_UPS`).
    PowerUp(u8),
    /// A key or button was rebound, or a Keybinds-tab switch flipped.
    Keymap,
    /// The Mods tab's HD Texture Pack: turned on or off, or another pack.
    TexturePack,
    /// A file mod turned on or off (from the next start).
    Mods,
    /// Download and install the HD texture pack (`crate::mods`).
    PackDownload,
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
    /// Play the game's system sound `id` (the window's cursor, decide and
    /// back sounds).
    Sound(u32),
    /// Host a LAN room (the game starts again with `--host`).
    LanHost,
    /// Join the LAN room at this address (`--join`).
    LanJoin(String),
    /// Stop hosting, or leave the room (the game starts again on its own).
    LanLeave,
}

/// What the LAN tab shows, from the window's thread.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LanView {
    /// 0 not in a room, 1 hosting one, 2 in someone's.
    pub mode: u8,
    /// Hosting: where others join; in a room: its address.
    pub addr: String,
    /// The rooms found on the network: their names and addresses.
    pub rooms: Vec<(String, String)>,
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
    /// The startup screen, while it is up: the game picture's aspect and
    /// the startup pipelines still being built (shown when `Some`).
    startup: Option<(f32, Option<u32>)>,
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
        Self { ctx, winit, window: settings_window, started: Instant::now(), held: 0, repeat: None, shown: false, startup: None }
    }

    /// The LAN tab's rooms and this game's own.
    pub fn set_lan(&mut self, view: LanView) {
        self.window.lan = view;
    }

    /// The LAN tab is up: rooms are worth looking for.
    pub fn wants_rooms(&self) -> bool {
        self.window.is_open() && self.window.on_lan_tab()
    }

    /// The game started in widescreen (the Aspect Ratio row's note).
    pub fn set_booted_widescreen(&mut self, wide: bool) {
        self.window.booted_widescreen = wide;
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

    /// The startup screen ([`startup`]) over everything while `Some`: the
    /// game picture's aspect, and the startup pipelines still being built
    /// when the shader line shows.
    pub fn set_startup(&mut self, screen: Option<(f32, Option<u32>)>) {
        self.startup = screen;
    }

    /// The HD texture pack's download, for the Mods tab (`None`: none).
    pub fn set_pack_download(&mut self, state: Option<crate::mods::PackDownload>) {
        self.window.pack_download = state;
    }

    /// Looks at the texture packs and file mods installed again.
    pub fn refresh_mods(&mut self) {
        self.window.refresh_mods();
    }

    /// The Keybinds tab waits for a key: key presses go to [`Menus::capture_key`].
    pub fn capturing(&self) -> bool {
        self.window.is_open() && self.window.capturing()
    }

    pub fn capture_key(&mut self, settings: &mut Settings, code: winit::keyboard::KeyCode, down: bool) -> Vec<Change> {
        self.window.capture_key(settings, code, down)
    }

    /// The Keybinds tab waits for a key or mouse button: mouse presses go
    /// to [`Menus::capture_mouse`], not to the window.
    pub fn capturing_keys(&self) -> bool {
        self.window.is_open() && self.window.capturing_keys()
    }

    pub fn capture_mouse(&mut self, settings: &mut Settings, button: u8, down: bool) -> Vec<Change> {
        self.window.capture_mouse(settings, button, down)
    }

    /// A controller is connected or not (the Keybinds tab opens on its
    /// buttons if one is).
    pub fn set_pad_connected(&mut self, connected: bool) {
        self.window.pad_connected = connected;
    }

    /// The Keybinds tab waits for a controller button: hand it what is held
    /// ([`Menus::capture_pad`]) every frame.
    pub fn capturing_pad(&self) -> bool {
        self.window.is_open() && self.window.capturing_pad()
    }

    pub fn capture_pad(&mut self, settings: &mut Settings, held: &[input::padmap::PadInput]) -> Vec<Change> {
        self.window.capture_pad(settings, held)
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
    pub fn frame(&mut self, window: &Window, settings: &mut Settings, buttons: u16) -> Option<(Option<render::UiFrame>, egui::TexturesDelta, Vec<Change>)> {
        let now = self.started.elapsed().as_secs_f64();
        let open = self.window.is_open();
        if !open {
            self.held = buttons;
            if self.startup.is_none() {
                if self.shown {
                    self.shown = false;
                    return Some((None, egui::TexturesDelta::default(), Vec::new()));
                }
                return None;
            }
        }
        let nav = if open { self.nav(buttons, now) } else { Nav::default() };
        let _ = nav.any();
        let raw = self.winit.take_egui_input(window);
        let mut changes = Vec::new();
        let Self { ctx, window: settings_window, startup, .. } = self;
        let out = ctx.run_ui(raw, |ui| {
            let ctx = ui.ctx();
            let mut shapes = if open { settings_window.draw(ctx, settings, nav, now, &mut changes) } else { Vec::new() };
            // Over everything, the Settings window too.
            if let Some((aspect, compiling)) = *startup {
                shapes.extend(startup::draw(ctx, aspect, compiling));
            }
            ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("menus"))).extend(shapes);
        });
        self.winit.handle_platform_output(window, out.platform_output);
        let primitives = self.ctx.tessellate(out.shapes, out.pixels_per_point);
        self.shown = true;
        Some((Some(render::UiFrame { primitives, pixels_per_point: out.pixels_per_point }), out.textures_delta, changes))
    }
}
