//! The game in a window. The game runs on its own thread (`emu`), one
//! video field at a time at 59.94 Hz; this thread renders each field's GX
//! output as it arrives and presents:
//!
//! - at 60 fps (native), every field as it is;
//! - at higher rates (90 .. 360, or unlocked), in-between frames: the two
//!   newest fields are drawn with every paired draw's matrices interpolated
//!   (`gx::interp`), one field behind real time. The game itself still
//!   advances 59.94 times a second.
//!
//! Hotkeys (F1 shows them) change the display settings and save them to
//! `settings.toml`; F10 opens the Settings window (`ui`), which pauses the
//! game while it is up.

use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use vk_render::Renderer;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

use crate::emu::{Emu, EmuOptions, GameFrame, ToEmu, FIELD};
use hle_ios::bt::{RemoteStatus, REMOTES};
use hle_ios::wiimote::Input;
use crate::overlay;
use crate::say::{say, say_err};
use crate::settings::{self, Settings, WideMode};
use crate::ui;

pub struct Options {
    pub game: PathBuf,
    /// Exit after this many game frames.
    pub max_frames: Option<u64>,
    /// With `max_frames`: save the last frame's XFB here.
    pub screenshot: Option<PathBuf>,
    /// Run the game as fast as possible (frames shown as they come).
    pub unpaced: bool,
    pub settings: Settings,
    /// Tracing, profiling and scripted input for the game thread.
    pub emu: EmuOptions,
    /// Screenshots of given game frames (1-based field numbers; after
    /// `--load-state` or joining a net game they carry on from the state's).
    pub shots: Vec<(u64, PathBuf)>,
    /// Print frames shown and game speed once a second.
    pub log_fps: bool,
    /// Time every field (`crate::perf`).
    pub perf: Option<crate::perf::Options>,
}

/// Fields kept for interpolation; older ones are dropped and their evicted
/// textures released.
const QUEUE: usize = 3;

/// The startup screen (WitherBlitz's NSMB-Wii-Recompilation's): the credits
/// over the game from launch until it has shown a picture for
/// [`STARTUP_COVER_FRAMES`] and the startup pipelines are built.
struct Startup {
    on: bool,
    began: Instant,
    /// Pipelines were still being built when it was checked: the shader
    /// line shows until they all are.
    notice: bool,
    /// The frame count (`presented`) at which it may lift, from the game's
    /// first field with a picture.
    lift_at: Option<u64>,
}

/// Frames the startup screen stays up over the game's first picture.
const STARTUP_COVER_FRAMES: u64 = 60;
/// The longest the startup screen waits for the startup pipelines.
const STARTUP_SHADER_LIMIT: Duration = Duration::from_secs(180);

/// A field kept for frames shown in between; its output is shared with the
/// [`Mover`] thread while it pairs the field with the one before.
struct Queued {
    index: u64,
    time: Instant,
    xfb: u32,
    out: Arc<gx::Output>,
}

/// What moves a field's draws between it and the field before
/// (`gx::interp::pair` and `moves`, about 3 ms a field on the world map),
/// built on a thread of its own as soon as the field arrives: by the time
/// frames are shown between the two (a field later), the window thread only
/// uploads it.
struct Mover {
    jobs: Sender<(u64, u64, Arc<gx::Output>, Arc<gx::Output>)>,
    done: Receiver<((u64, u64), gx::interp::Moves)>,
    ready: VecDeque<((u64, u64), gx::interp::Moves)>,
}

impl Mover {
    fn spawn() -> Self {
        let (jobs, rx) = mpsc::channel::<(u64, u64, Arc<gx::Output>, Arc<gx::Output>)>();
        let (tx, done) = mpsc::channel();
        std::thread::Builder::new()
            .name("interp".into())
            .spawn(move || {
                for (ia, ib, a, b) in rx {
                    let moves = gx::interp::moves(&a, &b, &gx::interp::pair(&a, &b));
                    if tx.send(((ia, ib), moves)).is_err() {
                        break;
                    }
                }
            })
            .expect("spawn the interpolation thread");
        Self { jobs, done, ready: VecDeque::new() }
    }

    /// The moves for `key` if they are built; older ones are dropped.
    fn take(&mut self, key: (u64, u64)) -> Option<gx::interp::Moves> {
        self.ready.extend(self.done.try_iter());
        self.ready.retain(|(k, _)| k.1 >= key.1);
        let i = self.ready.iter().position(|(k, _)| *k == key)?;
        self.ready.remove(i).map(|(_, m)| m)
    }
}
/// A field whose emulation takes longer than this is logged.
const SLOW_FIELD: Duration = Duration::from_millis(20);

/// Another local player's own window (docs/PORT_PLAN.md, section 12): the
/// same fields as the main window, through that player's view. It draws on
/// a thread of its own ([`run_player_window`]) with a renderer on the main
/// window's GPU device, so the windows draw side by side rather than one
/// after another on the window thread. It shows each field as it comes (no
/// in-between frames).
struct PlayerWindow {
    player: usize,
    /// To its thread (taken to stop it).
    tx: Option<Sender<ToPlayer>>,
    thread: Option<std::thread::JoinHandle<()>>,
    // Dropped last: the thread's renderer (and its Vulkan surface) first.
    window: Window,
}

impl Drop for PlayerWindow {
    fn drop(&mut self) {
        self.tx = None;
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// What a player's window's thread is sent, in order.
enum ToPlayer {
    /// A field: its output, XFB, the window's size, a screenshot to take of
    /// it, and the background layers learned up to it.
    Field { out: Arc<gx::Output>, xfb: u32, size: (u32, u32), shot: Option<PathBuf>, backdrop: gx::backdrop::Backdrop },
    /// Textures the game no longer uses.
    Evict(Vec<u32>),
    Resized,
}

/// A player's window's thread. It shows the newest field it has been sent;
/// fields before that one (when it fell behind) are not shown, and only
/// drawn when they copy the EFB to textures (otherwise their textures are
/// just uploaded), so it catches up at once.
fn run_player_window(mut renderer: Renderer, player: usize, rx: Receiver<ToPlayer>) {
    while let Ok(first) = rx.recv() {
        let mut batch = vec![first];
        batch.extend(rx.try_iter());
        let newest = batch.iter().rposition(|m| matches!(m, ToPlayer::Field { .. }));
        for (i, m) in batch.into_iter().enumerate() {
            let r = match m {
                ToPlayer::Evict(ids) => {
                    renderer.evict(&ids);
                    Ok(())
                }
                ToPlayer::Resized => {
                    renderer.resized();
                    Ok(())
                }
                ToPlayer::Field { out, xfb, size, shot, backdrop } => {
                    let copies = out.cmds.iter().any(|c| matches!(c, gx::Cmd::CopyTex(_)));
                    let show = Some(i) == newest || shot.is_some();
                    let r = if show || copies {
                        if shot.is_some() {
                            renderer.screenshot = shot;
                        }
                        // A field skipped still draws when it copies the EFB to
                        // textures: later fields sample them (light textures
                        // are drawn once and used for many fields).
                        let (w, h) = if show { size } else { (0, 0) };
                        let mut fill = |dst: &mut [gx::Vertex]| fill_view(&out, player, dst, &backdrop);
                        renderer.render_with(&out, out.vertices.len(), &mut fill, true, xfb, w, h).map(|_| ())
                    } else {
                        let cmds = out.cmds.iter().filter(|c| matches!(c, gx::Cmd::CopyTex(_))).copied().collect();
                        let uploads = gx::Output { new_textures: out.new_textures.clone(), cmds, ..Default::default() };
                        renderer.render(&uploads, &[], true, xfb, 0, 0).map(|_| ())
                    };
                    // Only the main window's EFB copies go back to the game.
                    let _ = renderer.take_writebacks();
                    r
                }
            };
            if let Err(e) = r {
                say_err!("player {}'s window: {e}", player + 1);
                return;
            }
        }
    }
}

/// The field's vertices into `dst` (the renderer's GPU buffer, only
/// written), with the world draws moved to `player`'s window when the field
/// has one.
fn fill_view(out: &gx::Output, player: usize, dst: &mut [gx::Vertex], backdrop: &gx::backdrop::Backdrop) {
    match out.views.get(player).copied().flatten() {
        Some(view) => out.world.apply_into(&out.vertices, dst, &view, Some(backdrop)),
        None => dst.copy_from_slice(&out.vertices),
    }
}

/// The main window's view of a field: its player's window while the game
/// camera is a box around far-apart players. Otherwise the game's own
/// camera is shown as it is, with in-between frames moved on the GPU.
fn main_view(out: &gx::Output, player: usize) -> Option<gx::xf::View> {
    out.views.get(player).copied().flatten().filter(|_| !out.world.camera.normal && !out.world.ranges.is_empty())
}

/// `out` kept on the GPU for in-between frames ([`Renderer::prepare`]). In
/// the main player's window ([`main_view`]) its world draws are moved on the
/// CPU first, and it is shown without in-between movement.
fn prepare_field(renderer: &mut Renderer, key: (u64, u64), out: &gx::Output, moves: Option<&gx::interp::Moves>, player: usize, backdrop: &gx::backdrop::Backdrop) -> Result<(), String> {
    match main_view(out, player) {
        Some(view) => {
            let mut vertices = out.vertices.clone();
            out.world.apply(&mut vertices, &view, Some(backdrop));
            renderer.prepare_with(key, out, &vertices, None)
        }
        None => renderer.prepare(key, out, moves),
    }
    .map_err(|e| e.to_string())
}

struct App {
    opts: Options,
    settings: Settings,
    emu: Option<Emu>,
    // Declared before `window`: the Vulkan surface must be destroyed first.
    renderer: Option<Renderer>,
    window: Option<Window>,
    error: Option<String>,
    queue: VecDeque<Queued>,
    mover: Option<Mover>,
    games: u64,
    presented: u64,
    startup: Startup,
    last_present: Instant,
    next_display: Instant,
    /// Recent display frame times (ms) for the performance monitor.
    times: VecDeque<f32>,
    title_at: Instant,
    title_presents: u64,
    title_games: u64,
    shown_fps: f32,
    game_fps: f32,
    emu_ms: f32,
    render_ms: f32,
    /// Render-thread time this second (for --log-fps): new fields taken in,
    /// fields prepared for in-between frames, and frames shown.
    spent: [Duration; 3],
    help: bool,
    slow_fields: u64,
    /// `--perf`.
    perf: Option<crate::perf::Perf>,
    done: bool,
    inputs: input::Inputs,
    sent_input: Option<[Input; REMOTES]>,
    /// What the game thread was last told about interpolation records.
    sent_interpolate: Option<bool>,
    /// A `--shot` to take from the next in-between frame.
    pending_shot: Option<PathBuf>,
    remotes: [RemoteStatus; REMOTES],
    /// The Settings window and the port's other menus.
    menus: Option<ui::Menus>,
    /// Player 1's buttons held when a menu was up: kept from the game until
    /// let go, so the press that closed the menu does not reach it.
    menu_mask: u16,
    /// The Settings window was opened by a game menu's Settings button.
    settings_from_game: bool,
    /// The save files' "Cheated" marks, shared with the game thread.
    cheats: Arc<crate::gameplay::Cheats>,
    /// Player 1's character on each save file, shared with the game thread.
    characters: Arc<crate::gameplay::Characters>,
    /// What the game thread was last told: paused (a menu is up) or not.
    sent_pause: bool,
    /// A menu was up at the last frame.
    was_open: bool,
    /// The Settings window changed something not saved yet (saved when it
    /// closes: a slider drag changes it every frame).
    unsaved: bool,
    /// The update check's answer, when it comes: a newer release.
    update_check: Option<Receiver<port_update::Release>>,
    /// The newer release found, and when (said on screen for a while).
    update: Option<(port_update::Release, Instant)>,
    /// The update being installed on a thread of its own: how far it has
    /// got, and the flag that cancels its download.
    updater: Option<(Arc<std::sync::Mutex<ui::UpdateState>>, Arc<std::sync::atomic::AtomicBool>)>,
    /// The Mods tab's HD texture pack download, while one runs or failed.
    pack_download: Option<Arc<std::sync::Mutex<crate::mods::PackDownload>>>,
    /// Installed: this program to start again once the window has closed.
    restart: Option<PathBuf>,
    /// The game's button prompts: what they show now (keys or a
    /// controller's buttons, after the last device used), their painted
    /// images (sent with every texture pack), the art they are painted from
    /// (read when first needed), and the last few paintings.
    prompt_style: ui::prompts::Style,
    prompt_images: Vec<(gx::replace::Key, Arc<gx::replace::Image>)>,
    prompts: Option<Option<ui::prompts::Prompts>>,
    prompt_cache: Vec<(Vec<ui::prompts::Content>, Vec<(gx::replace::Key, Arc<gx::replace::Image>)>)>,
    /// `NSMBW_PROMPTS=keys` or `pad`: the prompts stay that way (tests).
    prompt_pin: Option<ui::prompts::Style>,
    /// Where the mouse is in the window (pixels).
    pointer: Option<(f64, f64)>,
    /// The LAN tab's search for rooms, while it is up.
    finder: Option<crate::lan::Finder>,
    /// Join, host or leave a LAN room: the game starts again with these
    /// network options (none: on its own) once the window has closed.
    relaunch: Option<Vec<String>>,
    /// Looks for Wii Remotes paired with Windows (the Wii Remotes switch).
    #[cfg(windows)]
    wii_remotes: Option<input::wii_hid::Scanner>,
    /// Bluetooth passthrough's pairing key (F8) held since then, and
    /// whether holding it 10 s already forgot the pairings.
    sync_down: Option<(Instant, bool)>,
    /// The game started in widescreen (it reads 4:3 or not at boot).
    booted_widescreen: bool,
    /// What the game thread was last told about the picture.
    sent_display: Option<crate::emu::DisplayOptions>,
    /// The player the main window shows, and windows of other local players.
    main_player: usize,
    players: Vec<PlayerWindow>,
    /// Textures uploaded and not yet evicted, for a player window opened
    /// later (a renderer only receives a texture in the field it is new).
    textures: HashMap<u32, gx::texture::NewTexture>,
    online: bool,
    /// Background layers as the game draws them with its own camera, for
    /// drawing them in each player's window.
    backdrop: gx::backdrop::Backdrop,
}

/// Set for a game the LAN tab started to join a room.
const LAN_JOIN_ENV: &str = "NSMBW_LAN_JOIN";

/// This run's options without its network ones, then `net`: the options
/// a game starting again into (or out of) a LAN room gets.
fn relaunch_args(net: &[String]) -> Vec<std::ffi::OsString> {
    let mut out = Vec::new();
    let mut args = std::env::args_os().skip(1);
    while let Some(a) = args.next() {
        if matches!(a.to_str(), Some("--host" | "--join" | "--wait-join")) {
            args.next();
        } else {
            out.push(a);
        }
    }
    out.extend(net.iter().map(Into::into));
    out
}

/// How long a newer release found is said on screen.
const UPDATE_NOTE: Duration = Duration::from_secs(10);

/// Asks the release source for a newer release of the port, on a thread of
/// its own (the answer can take seconds). `None` when `update.toml`, next to
/// the program, turns the check off.
fn check_for_update() -> Option<Receiver<port_update::Release>> {
    let config = port_update::Config::load(&port_update::install_dir());
    if !config.enabled {
        return None;
    }
    let (tx, rx) = mpsc::channel();
    // The old files of the last update, now that the old program has exited.
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(3));
        port_update::install::cleanup(&port_update::install_dir());
    });
    std::thread::spawn(move || match port_update::check(&config) {
        Ok(port_update::Status::Available(release)) => {
            let _ = tx.send(release);
        }
        Ok(port_update::Status::UpToDate) => {}
        Err(e) => say_err!("update check: {e}"),
    });
    Some(rx)
}

/// Sleeps until `deadline`, then spins the last stretch: a sleep can
/// overshoot by a millisecond, a third of a 360 fps frame.
fn wait_until(deadline: Instant) {
    // A high-resolution timer wakes within a fraction of a millisecond; the
    // rest is a short spin (at 400 fps a frame is only 2.5 ms).
    const SPIN: Duration = Duration::from_micros(250);
    let now = Instant::now();
    if deadline > now + SPIN {
        crate::win::sleep(deadline - now - SPIN);
    }
    while Instant::now() < deadline {
        std::thread::yield_now();
    }
}

/// The Music, Sound Effects and Menu Sounds volumes, for the mixer.
fn volumes(s: &Settings) -> [u32; 3] {
    [s.volume[1], s.volume[2], s.volume[3]]
}

pub fn gameplay_options(s: &Settings) -> crate::gameplay::Options {
    crate::gameplay::Options { no_time_limit: s.no_time_limit, infinite_lives: s.infinite_lives, autosave: s.autosave, exit_course: s.exit_course, p1_character: s.p1_character }
}

/// The game menus' Settings button, in the game's language.
fn settings_label(language: &str) -> &'static str {
    match language {
        "fr" => "Paramètres",
        "es" => "Configuración",
        _ => "Settings",
    }
}

/// The console's language number (SYSCONF IPL.LNG) for a settings code.
fn language_code(language: &str) -> u8 {
    match language {
        "fr" => 3,
        "es" => 4,
        _ => 1,
    }
}

fn on(b: bool) -> &'static str {
    if b {
        "ON"
    } else {
        "OFF"
    }
}

impl App {
    /// 60 or 30 fps, or unpaced runs: no interpolation.
    fn native(&self) -> bool {
        self.settings.fps == 60 || self.settings.fps == 30 || self.opts.unpaced
    }

    /// Every other field is not shown (30 fps, or the battery saver).
    fn half_rate(&self) -> bool {
        (self.settings.fps == 30 || self.settings.battery_saver) && !self.opts.unpaced
    }

    fn start(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let mut attrs = Window::default_attributes().with_title(self.title()).with_inner_size(LogicalSize::new(1280.0, 720.0));
        if self.settings.fullscreen {
            attrs = attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let window = event_loop.create_window(attrs).map_err(|e| e.to_string())?;
        let display = window.display_handle().map_err(|e| e.to_string())?.as_raw();
        let handle = window.window_handle().map_err(|e| e.to_string())?.as_raw();
        // "Native" resolution: enough 480-line multiples to cover the window.
        let scale = if self.settings.scale == 0.0 {
            let monitor = window.current_monitor().map(|m| m.size().height);
            let h = if self.settings.fullscreen { monitor.unwrap_or(window.inner_size().height) } else { window.inner_size().height };
            h.div_ceil(480).clamp(1, 8) as f32
        } else {
            self.settings.scale
        };
        let _ = std::fs::create_dir_all("cache");
        let ropts = vk_render::Options { scale, vsync: self.settings.vsync, pipeline_cache: Some(PathBuf::from("cache/pipelines.bin")), efb_copies_to_ram: self.settings.efb_copies_to_ram };
        let renderer = Renderer::new(display, handle, ropts).map_err(|e| e.to_string())?;
        self.booted_widescreen = self.settings.widescreen;
        println!(
            "vulkan device: {} (validation layer: {}); EFB {}x; {} fps; V-Sync {}",
            renderer.device_name,
            if renderer.validation { "on" } else { "not available" },
            scale,
            settings::fps_label(self.settings.fps),
            on(self.settings.vsync)
        );
        println!("keys: F1 help, F3 performance monitor, F4 battery saver, F5/F6 frame rate, F7 V-Sync, F9 aspect, F10 settings, F11 fullscreen");
        println!("controls (sideways Wii Remote): arrows D-pad, X/Space 2, Z/Shift 1, C/Ctrl shake, Q/E tilt, B the A button, Enter/Esc +, Tab/- minus, Home HOME; gamepads too");
        // The game reads its aspect ratio from SYSCONF once, at boot.
        if let Err(e) = hle_ios::nand::set_widescreen(Path::new("saves/nand"), self.settings.widescreen) {
            eprintln!("SYSCONF: {e}");
        }
        if let Err(e) = hle_ios::nand::set_language(Path::new("saves/nand"), language_code(&self.settings.language)) {
            eprintln!("SYSCONF: {e}");
        }
        crate::audio::VOLUME.store(self.settings.volume[0].min(100) as u8, std::sync::atomic::Ordering::Relaxed);
        self.apply_controls();
        let mut menus = ui::Menus::new(&window, &self.opts.game, &self.settings.language);
        menus.set_booted_widescreen(self.booted_widescreen);
        self.menus = Some(menus);
        let settings_label = Some(settings_label(&self.settings.language).to_string());
        let emu = Emu::spawn(self.opts.game.clone(), EmuOptions { unpaced: self.opts.unpaced, settings_label, cheats: self.cheats.clone(), characters: self.characters.clone(), gameplay: gameplay_options(&self.settings), bt_passthrough: self.settings.bt_passthrough, ..self.opts.emu.clone() });
        let _ = emu.to_emu.send(ToEmu::Volumes(volumes(&self.settings)));
        if let crate::net::Mode::Host { port, .. } = self.opts.emu.net {
            if crate::lan::announce(port, crate::lan::computer_name()).is_err() {
                say_err!("LAN: another game on this PC already answers room queries");
            }
        }
        self.emu = Some(emu);
        if let Some(pin) = self.prompt_pin {
            self.note_device(pin);
        }
        self.renderer = Some(renderer);
        self.window = Some(window);
        let now = Instant::now();
        (self.last_present, self.next_display, self.title_at) = (now, now, now);
        Ok(())
    }

    /// The picture's aspect, and whether it widens the game's view: 4:3 (or
    /// 16:9 while the game still runs in 4:3 until the next start), the
    /// game's 16:9, 16:10, or for Ultrawide the window's when it is wider
    /// than 16:9. Net games keep 16:9.
    fn picture(&self) -> (f32, bool) {
        const WIDE: f32 = hle_os::display::ASPECT_16_9;
        let net = !matches!(self.opts.emu.net, crate::net::Mode::Off);
        if !self.settings.widescreen {
            return (4.0 / 3.0, false);
        }
        if !self.booted_widescreen || net {
            return (WIDE, false);
        }
        match self.settings.wide_mode {
            WideMode::Sixteen9 => (WIDE, false),
            WideMode::Sixteen10 => (1.6, true),
            WideMode::Fill => {
                let size = self.window.as_ref().map(|w| w.inner_size()).filter(|s| s.width > 0 && s.height > 0);
                // A window a little off 16:9 keeps the game's own picture.
                match size.map(|s| s.width as f32 / s.height as f32) {
                    Some(a) if a > WIDE * 1.01 => (a, true),
                    _ => (WIDE, false),
                }
            }
        }
    }

    /// The picture's shape to the renderer, and to the game thread with the
    /// Graphics tab's options, when they change.
    fn update_picture(&mut self) {
        let (aspect, widened) = self.picture();
        if let Some(r) = &mut self.renderer {
            r.aspect = aspect;
            r.set_stretch(if widened { aspect / hle_os::display::ASPECT_16_9 } else { 1.0 });
        }
        let d = crate::emu::DisplayOptions { aspect: if widened { aspect } else { hle_os::display::ASPECT_16_9 }, spinning_coins: self.settings.spinning_coins, depth_blur: self.settings.depth_blur };
        if self.sent_display != Some(d) {
            if let Some(emu) = &self.emu {
                let _ = emu.to_emu.send(ToEmu::Display(d));
                self.sent_display = Some(d);
            }
        }
    }

    fn frame(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        self.update_picture();
        // The Keybinds tab: a controller connected, and a button being picked.
        let mut picked = Vec::new();
        if let Some(m) = &mut self.menus {
            m.set_pad_connected(self.inputs.pad_connected());
            if m.capturing_pad() {
                picked = m.capture_pad(&mut self.settings, &self.inputs.pad_held());
            }
        }
        self.apply_changes(picked);
        // A controller pressed at the last poll: the prompts show its buttons
        // (and the save select's Back button goes).
        if self.inputs.take_pad_pressed() {
            self.note_device(ui::prompts::Style::Pad);
            if let Some(m) = &mut self.menus {
                m.set_pointer_used(false);
            }
        }
        self.sync_held();
        let Some(emu) = &self.emu else { return Ok(()) };
        let interpolate = !self.native();
        if self.sent_interpolate != Some(interpolate) {
            let _ = emu.to_emu.send(ToEmu::Interpolate(interpolate));
            self.sent_interpolate = Some(interpolate);
        }
        self.inputs.set_player(emu.player());
        #[cfg(windows)]
        while let Some((player, remote, name)) = self.wii_remotes.as_ref().and_then(input::wii_hid::Scanner::take) {
            say!("Wii Remote {name:?} (paired with Windows) is player {}", player + 1);
            let _ = emu.to_emu.send(ToEmu::RealRemote(player, Some(Box::new(remote))));
        }
        let mut controls = self.inputs.poll();
        // A save file newly marked "Cheated" (or an erased one, unmarked) is
        // written down at once: it may happen long after the window closed.
        let marked = self.cheats.marked();
        let characters = self.characters.get();
        if marked != self.settings.cheated || characters != self.settings.file_characters {
            self.settings.cheated = marked;
            self.settings.file_characters = characters;
            self.settings.save(Path::new(settings::FILE));
        }
        let (aspect, _) = self.picture();
        if let Some(m) = &mut self.menus {
            m.set_active_file(self.cheats.active.load(std::sync::atomic::Ordering::Relaxed));
            m.set_save_select(*emu.save_select.lock().unwrap_or_else(|e| e.into_inner()), aspect);
        }
        // A newer release of the port: said on screen for a while, and the
        // Settings window asks whether to open its page.
        if let Some(mut release) = self.update_check.as_ref().and_then(|rx| rx.try_recv().ok()) {
            // The release's own text: kept short and plain for the screen.
            release.version = release.version.chars().filter(char::is_ascii_graphic).take(16).collect();
            say!("update: version {} is out {}", release.version, release.page);
            let can_install = release.download.is_some();
            if let Some(m) = self.menus.as_mut().filter(|_| can_install || !release.page.is_empty()) {
                m.set_update(&release.version, can_install);
            }
            self.update = Some((release, Instant::now()));
            self.update_check = None;
        }
        // An update being installed: shown in the Settings window; once in
        // place, the window closes and the new program starts (in `run`).
        if let Some((state, cancel)) = &self.updater {
            let state = state.lock().unwrap_or_else(|e| e.into_inner()).clone();
            let cancelled = cancel.load(std::sync::atomic::Ordering::Relaxed) && matches!(state, ui::UpdateState::Failed(_));
            if let Some(m) = &mut self.menus {
                m.set_update_state((!cancelled).then(|| state.clone()));
            }
            if cancelled {
                self.updater = None;
            } else if state == ui::UpdateState::Restarting && self.restart.is_none() {
                self.settings.save(Path::new(settings::FILE));
                self.restart = std::env::current_exe().ok();
                event_loop.exit();
            }
        }
        // The Multiplayer screen: the rooms on the network, and this game's own.
        if self.menus.as_ref().is_some_and(ui::Menus::wants_rooms) {
            if self.finder.is_none() {
                self.finder = crate::lan::Finder::new().map_err(|e| say_err!("LAN: {e}")).ok();
            }
            let rooms = self.finder.as_mut().map(|f| f.poll()).unwrap_or_default();
            let here = self.finder.as_ref().and_then(|f| f.here).map_or_else(|| "this PC".to_string(), |ip| ip.to_string());
            // Over Tailscale, others join at this PC's Tailscale address.
            let tailscale = self.finder.as_ref().and_then(|f| f.tailscale);
            let (mode, addr) = match &self.opts.emu.net {
                crate::net::Mode::Host { port, .. } => (1, tailscale.map_or_else(|| format!("{here}:{port}"), |ts| format!("{here}:{port} (Tailscale {ts}:{port})"))),
                crate::net::Mode::Join { addr } => (2, addr.clone()),
                crate::net::Mode::Off => (0, String::new()),
            };
            let view = ui::LanView { mode, addr, rooms: rooms.into_iter().map(|r| (if r.tailscale() { format!("{} (Tailscale)", r.name) } else { r.name }, r.addr.to_string())).collect() };
            if let Some(m) = &mut self.menus {
                m.set_lan(view);
            }
        } else {
            self.finder = None;
        }
        if self.relaunch.is_some() && self.restart.is_none() {
            self.settings.save(Path::new(settings::FILE));
            self.restart = std::env::current_exe().ok();
            event_loop.exit();
        }
        // The HD texture pack's download: shown in the Mods tab; once
        // installed, it is turned on.
        if let Some(d) = &self.pack_download {
            let state = d.lock().unwrap_or_else(|e| e.into_inner()).clone();
            let done = matches!(state, crate::mods::PackDownload::Done(_));
            if done {
                self.settings.hd_textures = true;
                self.settings.texture_pack = crate::mods::HD_PACK.into();
                self.settings.save(Path::new(settings::FILE));
                let _ = emu.to_emu.send(ToEmu::TexturePack(self.texture_spec()));
                self.pack_download = None;
            }
            if let Some(m) = &mut self.menus {
                m.set_pack_download((!done).then_some(state));
                if done {
                    m.refresh_mods();
                }
            }
        }
        // A game menu's Settings button was picked and its menu has gone, or
        // the save select's Multiplayer button.
        if emu.settings_button.swap(false, std::sync::atomic::Ordering::Relaxed) {
            if let Some(m) = &mut self.menus {
                m.open();
                self.settings_from_game = true;
            }
        }
        if emu.multiplayer_button.swap(false, std::sync::atomic::Ordering::Relaxed) {
            if let Some(m) = &mut self.menus {
                m.open_multiplayer();
                self.settings_from_game = true;
            }
        }
        // A menu up: the game is paused and player 1's buttons move the
        // menu's cursor instead.
        let open = self.menus.as_ref().is_some_and(ui::Menus::is_open);
        let p1 = controls[0].buttons;
        self.menu_mask = if open { p1 } else { self.menu_mask & p1 };
        controls[0].buttons &= !self.menu_mask;
        if open {
            controls[0].shake = false;
        }
        // The remotes' rumble on the players' controllers (not under a menu,
        // where the game stands still).
        for (i, r) in self.remotes.iter().enumerate() {
            self.inputs.rumble(i, r.rumble && !open);
        }
        // Opened from a game menu, the game keeps running under the window
        // (its menu waits for the window, and the window's sounds play
        // through it); opened with F10, the game stops.
        let pause = open && !self.settings_from_game;
        if self.sent_pause != pause {
            let _ = emu.to_emu.send(ToEmu::Pause(pause));
            self.sent_pause = pause;
        }
        if self.was_open != open {
            self.was_open = open;
            // Back to the game menu the window came from, on Settings.
            if !open && std::mem::take(&mut self.settings_from_game) {
                let _ = emu.to_emu.send(ToEmu::SettingsClosed);
            }
        }
        if self.sent_input != Some(controls) {
            let _ = emu.to_emu.send(ToEmu::Input(controls));
            self.sent_input = Some(controls);
        }
        self.update_startup();
        self.update_menus(p1)?;
        let Some(emu) = &self.emu else { return Ok(()) };
        let mut new = Vec::new();
        loop {
            match emu.frames.try_recv() {
                Ok(f) => new.push(f?),
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => return Err("the game thread stopped".into()),
            }
        }
        if new.is_empty() && self.native() {
            // Nothing to show until the next field.
            match emu.frames.recv_timeout(FIELD) {
                Ok(f) => new.push(f?),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return Err("the game thread stopped".into()),
            }
        }
        let count = new.len();
        let native = self.native();
        for (i, f) in new.into_iter().enumerate() {
            self.ingest(f, native && i + 1 == count)?;
            if self.done {
                event_loop.exit();
                return Ok(());
            }
        }
        if !native {
            self.present_interpolated()?;
        } else if count == 0 && open {
            // Paused under a menu: the last field again, with the menu.
            self.present_still()?;
        }
        self.open_player_windows(event_loop)?;
        self.update_counters();
        Ok(())
    }

    /// Renders a new field once (uploading its textures and reading back its
    /// EFB copies), presenting it when `present`.
    fn ingest(&mut self, mut f: GameFrame, present: bool) -> Result<(), String> {
        self.games += 1;
        self.title_games += 1;
        if f.visible && self.startup.lift_at.is_none() {
            self.startup.lift_at = Some(self.presented + STARTUP_COVER_FRAMES);
            say!("startup screen: the game shows a picture from field {}", f.index);
        }
        let last = self.opts.max_frames.is_some_and(|m| self.games >= m);
        // 30 fps (or the battery saver): every other field is not shown,
        // and only partly drawn (below), unless its EFB is read back.
        let copies = f.out.cmds.iter().any(|c| matches!(c, gx::Cmd::CopyTex(_)));
        let skip = self.half_rate() && f.index % 2 == 1 && !last && !f.wants_efb;
        // Interpolating, a new field is only shown by the in-between frames,
        // which draw it again: unless its EFB is read back (copies to RAM, CPU
        // peeks, a screenshot), it only needs its textures uploaded and its
        // copy targets made.
        let native = self.native();
        let prepare_only = !present && !native && !last && !f.wants_efb && !(copies && self.settings.efb_copies_to_ram);
        let show = (present || last) && !skip;
        let Some(size) = self.window.as_ref().map(|w| w.inner_size()) else { return Ok(()) };
        let rects = if show { self.overlay_rects(size.width, size.height) } else { Vec::new() };
        let (w, h) = if show { (size.width, size.height) } else { (0, 0) };
        let (Some(renderer), Some(emu)) = (&mut self.renderer, &self.emu) else { return Ok(()) };
        if last {
            renderer.screenshot = self.opts.screenshot.clone();
        }
        if let Some((_, path)) = self.opts.shots.iter().find(|(n, _)| *n == f.index + 1) {
            // Interpolating, the shot is the next frame shown (in between
            // this field and the one before).
            if native {
                renderer.screenshot = Some(path.clone());
            } else {
                self.pending_shot = Some(path.clone());
            }
            say!("frame {}: {} draws, {} texture copies", f.index + 1, f.out.cmds.iter().filter(|c| matches!(c, gx::Cmd::Draw(_))).count(), f.out.cmds.iter().filter(|c| matches!(c, gx::Cmd::CopyTex(_))).count());
        }
        renderer.overlay = rects;
        if let Some(p) = &self.perf {
            p.filter(&mut f.out);
        }
        (self.main_player, self.online) = (f.main_player, f.online);
        self.backdrop.learn(&f.out.world);
        if !self.online {
            for t in &f.out.new_textures {
                self.textures.insert(t.id, t.clone());
            }
            for id in &f.out.evicted {
                self.textures.remove(id);
            }
        }
        let started = Instant::now();
        if skip {
            // Its draws up to its last EFB-to-texture copy still run (later
            // fields sample the copies: light textures, every field in a
            // course), and its clears and copies after that (the next field
            // finds the EFB cleared as the game left it); the draws after
            // the last copy, the bulk of the scene, do not. It used to be
            // skipped only without copies, which in practice was never.
            let cmds = std::mem::take(&mut f.out.cmds);
            let last_copy = cmds.iter().rposition(|c| matches!(c, gx::Cmd::CopyTex(_)));
            f.out.cmds = cmds.iter().enumerate().filter(|&(i, c)| last_copy.is_some_and(|l| i <= l) || !matches!(c, gx::Cmd::Draw(_))).map(|(_, c)| *c).collect();
            // Only the vertices those draws use go to the GPU.
            let used = f.out.cmds.iter().filter_map(|c| match c {
                gx::Cmd::Draw(d) => f.out.indices.get(d.first as usize..(d.first + d.count) as usize)?.iter().max().map(|&m| m as usize + 1),
                _ => None,
            });
            let used = used.max().unwrap_or(0).min(f.out.vertices.len());
            let r = renderer.render(&f.out, &f.out.vertices[..used], true, f.xfb, 0, 0);
            f.out.cmds = cmds;
            r.map_err(|e| e.to_string())?;
        } else if prepare_only {
            // The copies make their target textures (their contents come
            // from the in-between frames, which run them again).
            let cmds = f.out.cmds.iter().filter(|c| matches!(c, gx::Cmd::CopyTex(_))).copied().collect();
            let uploads = gx::Output { new_textures: std::mem::take(&mut f.out.new_textures), cmds, ..Default::default() };
            renderer.render(&uploads, &[], true, f.xfb, 0, 0).map_err(|e| e.to_string())?;
        } else {
            match main_view(&f.out, self.main_player) {
                Some(view) => {
                    let (out, backdrop) = (&f.out, &self.backdrop);
                    let mut fill = |dst: &mut [gx::Vertex]| out.world.apply_into(&out.vertices, dst, &view, Some(backdrop));
                    renderer.render_with(&f.out, f.out.vertices.len(), &mut fill, true, f.xfb, w, h)
                }
                None => renderer.render(&f.out, &f.out.vertices, true, f.xfb, w, h),
            }
            .map_err(|e| e.to_string())?;
        }
        self.render_ms = started.elapsed().as_secs_f32() * 1000.0;
        self.spent[0] += started.elapsed();
        if let Some(p) = &mut self.perf {
            p.field(&f, started.elapsed(), renderer.gpu_time());
        }
        let writebacks = renderer.take_writebacks();
        if !writebacks.is_empty() {
            let _ = emu.to_emu.send(ToEmu::Writebacks(writebacks));
        }
        if f.wants_efb {
            let snap = renderer.efb_snapshot().map_err(|e| e.to_string())?;
            let _ = emu.to_emu.send(ToEmu::Efb(snap));
        }
        self.emu_ms = f.emu.as_secs_f32() * 1000.0;
        for (i, (old, new)) in self.remotes.iter().zip(f.remotes).enumerate() {
            if old.connected != new.connected {
                say!("Wii Remote {}: {}", i + 1, if new.connected { "connected" } else { "disconnected" });
            }
        }
        self.remotes = f.remotes;
        if f.emu > SLOW_FIELD {
            self.slow_fields += 1;
            if self.slow_fields <= 20 || self.slow_fields.is_power_of_two() {
                say_err!("slow field {}: emulation {:.1} ms, render {:.1} ms ({} slow so far)", f.index, self.emu_ms, self.render_ms, self.slow_fields);
            }
        }
        if show {
            self.presented_now();
        }
        let queued = Queued { index: f.index, time: f.time, xfb: f.xfb, out: Arc::new(f.out) };
        // The players' windows draw it on their own threads meanwhile.
        if !self.players.is_empty() {
            let shot = self.opts.shots.iter().find(|(n, _)| *n == queued.index + 1).map(|(_, p)| p.clone());
            for pw in &self.players {
                let size = pw.window.inner_size();
                let shot = shot.as_ref().map(|path| path.with_file_name(format!("{}-p{}.png", path.file_stem().and_then(|s| s.to_str()).unwrap_or("shot"), pw.player + 1)));
                if let Some(tx) = &pw.tx {
                    let _ = tx.send(ToPlayer::Field { out: queued.out.clone(), xfb: queued.xfb, size: (size.width, size.height), shot, backdrop: self.backdrop.clone() });
                }
            }
        }
        if !native {
            if let Some(prev) = self.queue.back() {
                let mover = self.mover.get_or_insert_with(Mover::spawn);
                let _ = mover.jobs.send((prev.index, queued.index, prev.out.clone(), queued.out.clone()));
            }
        }
        self.queue.push_back(queued);
        while self.queue.len() > QUEUE {
            let old = self.queue.pop_front().unwrap();
            if let Some(r) = &mut self.renderer {
                r.evict(&old.out.evicted);
            }
            for pw in &self.players {
                if let Some(tx) = &pw.tx {
                    let _ = tx.send(ToPlayer::Evict(old.out.evicted.clone()));
                }
            }
            // Its buffers go back to the GX threads unless the mover still
            // holds it (then they are freed).
            if let (Some(emu), Ok(out)) = (&self.emu, Arc::try_unwrap(old.out)) {
                let _ = emu.to_emu.send(ToEmu::Recycle(Box::new(out)));
            }
        }
        if last {
            self.done = true;
        }
        Ok(())
    }

    /// One in-between frame, one field behind real time.
    fn present_interpolated(&mut self) -> Result<(), String> {
        if self.settings.fps != 0 {
            let period = Duration::from_secs_f64(1.0 / f64::from(self.settings.fps));
            wait_until(self.next_display);
            // Catch up after a slow frame, but not by more than two.
            self.next_display += period;
            let now = Instant::now();
            if now > self.next_display + 2 * period {
                self.next_display = now;
            }
        }
        if self.queue.is_empty() {
            return Ok(());
        }
        let now = Instant::now();
        let shown = now.checked_sub(FIELD).unwrap_or(now);
        let q = &self.queue;
        let (a, b, t) = match (0..q.len() - 1).find(|&i| q[i].time <= shown && shown < q[i + 1].time) {
            Some(i) => {
                let span = (q[i + 1].time - q[i].time).as_secs_f32().max(1e-6);
                (i, i + 1, (shown - q[i].time).as_secs_f32() / span)
            }
            None if q.len() >= 2 && shown >= q[q.len() - 1].time => (q.len() - 2, q.len() - 1, 1.0),
            None => (0, 0, 1.0),
        };
        let Some(size) = self.window.as_ref().map(|w| w.inner_size()) else { return Ok(()) };
        let rects = self.overlay_rects(size.width, size.height);
        let started = Instant::now();
        // Once per pair of fields: the newer one goes to the GPU with what
        // moves its draws (built by the mover, or here if it has not got to
        // them yet); every frame shown in between only records draws.
        let key = (self.queue[a].index, self.queue[b].index);
        let prepare = self.renderer.as_ref().is_some_and(|r| r.prepared() != Some(key));
        let moves = if prepare && a != b {
            let built = self.mover.as_mut().and_then(|m| m.take(key));
            let (fa, fb) = (&self.queue[a], &self.queue[b]);
            Some(built.unwrap_or_else(|| gx::interp::moves(&fa.out, &fb.out, &gx::interp::pair(&fa.out, &fb.out))))
        } else {
            None
        };
        let Some(renderer) = &mut self.renderer else { return Ok(()) };
        renderer.overlay = rects;
        if let Some(path) = self.pending_shot.take() {
            renderer.screenshot = Some(path);
        }
        let fb = &self.queue[b];
        if prepare {
            prepare_field(renderer, key, &fb.out, moves.as_ref(), self.main_player, &self.backdrop)?;
            self.spent[1] += started.elapsed();
        }
        let shown = Instant::now();
        renderer.render_prepared(&fb.out, t.clamp(0.0, 1.0), fb.xfb, size.width, size.height).map_err(|e| e.to_string())?;
        self.spent[2] += shown.elapsed();
        self.render_ms = started.elapsed().as_secs_f32() * 1000.0;
        self.presented_now();
        Ok(())
    }

    /// Opens a window for each other local player in a course (one whose
    /// window the last field has). Online, the other players are on their
    /// own machines. A window stays open once opened, until it is closed.
    fn open_player_windows(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        if self.online {
            return Ok(());
        }
        let Some(last) = self.queue.back() else { return Ok(()) };
        let wanted: Vec<usize> = (0..last.out.views.len()).filter(|&p| p != self.main_player && last.out.views[p].is_some() && self.players.iter().all(|w| w.player != p)).collect();
        // The main window's picture: its aspect, and the widening the game
        // and the renderer share (`update_picture`).
        let (aspect, widened) = self.picture();
        for player in wanted {
            let attrs = Window::default_attributes().with_title(format!("New Super Mario Bros. Wii - Rust port - player {}", player + 1)).with_inner_size(LogicalSize::new(f64::from(540.0 * aspect), 540.0));
            let window = event_loop.create_window(attrs).map_err(|e| e.to_string())?;
            let display = window.display_handle().map_err(|e| e.to_string())?.as_raw();
            let handle = window.window_handle().map_err(|e| e.to_string())?.as_raw();
            let Some(main) = self.renderer.as_ref() else { return Ok(()) };
            // No pipeline cache file: the main window's renderer owns it.
            let ropts = vk_render::Options { scale: main.scale(), vsync: false, pipeline_cache: None, efb_copies_to_ram: false };
            // On the main window's device: no device (and driver memory) of its own.
            let mut renderer = Renderer::new_sharing(main, display, handle, ropts).map_err(|e| e.to_string())?;
            renderer.aspect = aspect;
            renderer.set_stretch(if widened { aspect / hle_os::display::ASPECT_16_9 } else { 1.0 });
            // Every texture the game still uses, before the first field.
            let uploads = gx::Output { new_textures: self.textures.values().cloned().collect(), ..Default::default() };
            renderer.render(&uploads, &[], true, last.xfb, 0, 0).map_err(|e| e.to_string())?;
            let (tx, rx) = mpsc::channel();
            let thread = std::thread::Builder::new().name(format!("player {} window", player + 1)).spawn(move || run_player_window(renderer, player, rx)).map_err(|e| e.to_string())?;
            println!("player {}: own window", player + 1);
            self.players.push(PlayerWindow { player, tx: Some(tx), thread: Some(thread), window });
        }
        Ok(())
    }

    /// The newest field shown again as it is (the game is paused under a
    /// menu, and the menu moves).
    fn present_still(&mut self) -> Result<(), String> {
        let Some(size) = self.window.as_ref().map(|w| w.inner_size()) else { return Ok(()) };
        let rects = self.overlay_rects(size.width, size.height);
        let (Some(renderer), Some(fb)) = (&mut self.renderer, self.queue.back()) else { return Ok(()) };
        renderer.overlay = rects;
        let key = (fb.index, fb.index);
        if renderer.prepared() != Some(key) {
            prepare_field(renderer, key, &fb.out, None, self.main_player, &self.backdrop)?;
        }
        renderer.render_prepared(&fb.out, 1.0, fb.xfb, size.width, size.height).map_err(|e| e.to_string())?;
        self.presented_now();
        Ok(())
    }

    /// The startup screen ([`Startup`]): its shader line, and lifting it once
    /// the game has shown a picture for a second and the startup pipelines
    /// are built (or after [`STARTUP_SHADER_LIMIT`]).
    fn update_startup(&mut self) {
        if !self.startup.on {
            return;
        }
        let compiling = self.renderer.as_mut().map_or(0, |r| r.pipelines_compiling());
        let ready = compiling == 0 || self.startup.began.elapsed() > STARTUP_SHADER_LIMIT;
        self.startup.notice |= compiling > 0;
        let (aspect, _) = self.picture();
        let Some(menus) = &mut self.menus else { return };
        if ready && self.startup.lift_at.is_some_and(|at| self.presented >= at) {
            self.startup.on = false;
            menus.set_startup(None);
            say!("startup screen lifted after {:.1} s", self.startup.began.elapsed().as_secs_f32());
            return;
        }
        menus.set_startup(Some((aspect, (self.startup.notice && !ready).then_some(compiling))));
    }

    /// The menus' next frame to the renderer, and what they changed applied.
    fn update_menus(&mut self, buttons: u16) -> Result<(), String> {
        let (Some(menus), Some(window)) = (&mut self.menus, &self.window) else { return Ok(()) };
        let Some((frame, textures, changes)) = menus.frame(window, &mut self.settings, buttons) else { return Ok(()) };
        let closed = frame.is_none();
        if let Some(r) = &mut self.renderer {
            r.set_ui(frame, &textures).map_err(|e| e.to_string())?;
        }
        self.apply_changes(changes);
        if closed && self.unsaved {
            self.settings.save(Path::new(settings::FILE));
            self.unsaved = false;
        }
        Ok(())
    }

    /// What the Settings window changed, put into effect (saved when it closes).
    fn apply_changes(&mut self, changes: Vec<ui::Change>) {
        for change in changes {
            if let ui::Change::Sound(id) = change {
                if let Some(emu) = &self.emu {
                    let _ = emu.to_emu.send(ToEmu::Sound(id));
                }
                continue;
            }
            self.unsaved = true;
            match change {
                ui::Change::Fps => {
                    self.next_display = Instant::now();
                    if let Some(w) = &self.window {
                        w.set_title(&self.title());
                    }
                }
                ui::Change::VSync => {
                    if let Some(r) = &mut self.renderer {
                        r.set_vsync(self.settings.vsync);
                    }
                }
                ui::Change::Fullscreen => {
                    if let Some(w) = &self.window {
                        w.set_fullscreen(self.settings.fullscreen.then_some(Fullscreen::Borderless(None)));
                    }
                }
                ui::Change::Aspect => {
                    // The picture follows in `update_picture`.
                    if let Err(e) = hle_ios::nand::set_widescreen(Path::new("saves/nand"), self.settings.widescreen) {
                        say_err!("SYSCONF: {e}");
                    }
                }
                ui::Change::Volume => {
                    crate::audio::VOLUME.store(self.settings.volume[0].min(100) as u8, std::sync::atomic::Ordering::Relaxed);
                    if let Some(emu) = &self.emu {
                        let _ = emu.to_emu.send(ToEmu::Volumes(volumes(&self.settings)));
                    }
                }
                ui::Change::Language => {
                    if let Err(e) = hle_ios::nand::set_language(Path::new("saves/nand"), language_code(&self.settings.language)) {
                        say_err!("SYSCONF: {e}");
                    }
                }
                ui::Change::Gameplay => {
                    if let Some(emu) = &self.emu {
                        let _ = emu.to_emu.send(ToEmu::Gameplay(gameplay_options(&self.settings)));
                    }
                }
                ui::Change::PowerUp(p) => {
                    if let Some(emu) = &self.emu {
                        let _ = emu.to_emu.send(ToEmu::PowerUp(p));
                    }
                }
                ui::Change::Keymap => {
                    self.apply_controls();
                    self.refresh_prompts();
                }
                ui::Change::PackDownload => self.pack_download = Some(crate::mods::download_hd_pack()),
                ui::Change::TexturePack => {
                    if let Some(emu) = &self.emu {
                        let _ = emu.to_emu.send(ToEmu::TexturePack(self.texture_spec()));
                    }
                }
                ui::Change::Cheated(file) => {
                    self.cheats.marks.fetch_or(1 << file.min(2), std::sync::atomic::Ordering::Relaxed);
                }
                ui::Change::UpdateInstall => {
                    let (Some((release, _)), Ok(program)) = (self.update.clone(), std::env::current_exe()) else { continue };
                    let state = Arc::new(std::sync::Mutex::new(ui::UpdateState::Downloading { done: 0, total: 0 }));
                    let cancel = Arc::new(std::sync::atomic::AtomicBool::new(false));
                    let (s, c) = (state.clone(), cancel.clone());
                    std::thread::spawn(move || {
                        // Its path in the installation: compat\nsmbw-port.exe for the compat build.
                        let folder = port_update::install_dir();
                        let name = program.strip_prefix(&folder).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                        let set = |v: ui::UpdateState| *s.lock().unwrap_or_else(|e| e.into_inner()) = v;
                        let r = port_update::install::update(&release, &folder, &name, &|p| set(match p {
                            port_update::install::Progress::Downloading { done, total } => ui::UpdateState::Downloading { done, total },
                            port_update::install::Progress::Installing => ui::UpdateState::Installing,
                        }), &c);
                        set(match r {
                            Ok(()) => ui::UpdateState::Restarting,
                            Err(e) => ui::UpdateState::Failed(e),
                        });
                    });
                    self.updater = Some((state, cancel));
                }
                ui::Change::LanHost => self.relaunch = Some(vec!["--host".into(), crate::lan::GAME_PORT.to_string()]),
                ui::Change::LanJoin(addr) => {
                    let addr = crate::lan::with_port(&addr);
                    crate::lan::joined(&addr);
                    self.relaunch = Some(vec!["--join".into(), addr]);
                }
                ui::Change::LanLeave => self.relaunch = Some(Vec::new()),
                ui::Change::UpdateCancel => {
                    if let Some((_, cancel)) = &self.updater {
                        cancel.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                ui::Change::UpdateDismiss => {
                    self.updater = None;
                    if let Some(m) = &mut self.menus {
                        m.set_update_state(None);
                    }
                }
                ui::Change::UpdatePage => {
                    if let Some((release, _)) = &self.update {
                        if !crate::win::open_page(&release.page) {
                            say_err!("update: could not open {}", release.page);
                        }
                    }
                }
                // From the next start, or read every frame.
                ui::Change::Scale | ui::Change::PerfMonitor | ui::Change::BatterySaver | ui::Change::Sound(_) | ui::Change::Graphics | ui::Change::Mods => {}
            }
        }
    }

    /// The texture pack the Mods tab picked, with the button prompts' images.
    fn texture_spec(&self) -> crate::mods::PackSpec {
        let mut spec = crate::mods::PackSpec::from_settings(&self.settings);
        spec.images = self.prompt_images.clone();
        spec
    }

    /// The player used the keyboard or mouse (`Keys`) or a controller
    /// (`Pad`): the game's prompts follow, except in scripted and net runs,
    /// which keep the game's own.
    fn note_device(&mut self, style: ui::prompts::Style) {
        let scripted = !self.opts.emu.script.is_empty() || !matches!(self.opts.emu.net, crate::net::Mode::Off);
        let style = self.prompt_pin.unwrap_or(style);
        if scripted || style == self.prompt_style {
            return;
        }
        self.prompt_style = style;
        self.refresh_prompts();
    }

    /// The prompts painted for the style and bindings now, if they changed.
    fn refresh_prompts(&mut self) {
        // Still the game's own: nothing to send.
        if self.prompt_style == ui::prompts::Style::Game && self.prompt_images.is_empty() {
            return;
        }
        let contents = ui::prompts::contents(self.prompt_style, &self.settings.keymap, &self.settings.padmap);
        let images = match self.prompt_cache.iter().position(|(c, _)| *c == contents) {
            Some(i) if i + 1 == self.prompt_cache.len() => return,
            Some(i) => {
                let entry = self.prompt_cache.remove(i);
                let images = entry.1.clone();
                self.prompt_cache.push(entry);
                images
            }
            None => {
                let Some(prompts) = self.prompts.get_or_insert_with(ui::prompts::Prompts::load) else { return };
                let started = Instant::now();
                let images = prompts.paint(&contents);
                say!("button prompts: {:?} ({} images, painted in {:.1} ms)", self.prompt_style, images.len(), started.elapsed().as_secs_f64() * 1000.0);
                if self.prompt_cache.len() >= 4 {
                    self.prompt_cache.remove(0);
                }
                self.prompt_cache.push((contents, images.clone()));
                images
            }
        };
        self.prompt_images = images;
        if let Some(emu) = &self.emu {
            let _ = emu.to_emu.send(ToEmu::TexturePack(self.texture_spec()));
        }
    }

    /// The Keybinds tab's settings to the controls.
    fn apply_controls(&mut self) {
        self.inputs.set_keymap(self.settings.keymap.clone());
        self.inputs.set_padmap(self.settings.padmap.clone());
        self.inputs.set_rumble_enabled(self.settings.rumble);
        self.inputs.set_smart_spin(self.settings.smart_spin);
        self.inputs.set_mouse_gyro(self.settings.mouse_gyro);
        self.inputs.set_reverse_tilt(self.settings.reverse_tilt);
        self.inputs.set_motion_sensors(self.settings.motion_sensors);
        // Real Wii Remotes: not in net games (their reports would bypass the
        // shared input) or scripted runs.
        #[cfg(windows)]
        {
            let allowed = matches!(self.opts.emu.net, crate::net::Mode::Off) && self.opts.emu.script.is_empty();
            let want = self.settings.wii_remotes && allowed;
            if want && self.wii_remotes.is_none() {
                say!("Wii Remotes: looking for remotes paired with Windows");
                self.wii_remotes = Some(input::wii_hid::Scanner::start());
            } else if !want && self.wii_remotes.take().is_some() {
                if let Some(emu) = &self.emu {
                    for i in 0..REMOTES {
                        let _ = emu.to_emu.send(ToEmu::RealRemote(i, None));
                    }
                }
            }
        }
    }

    fn presented_now(&mut self) {
        let now = Instant::now();
        self.times.push_back((now - self.last_present).as_secs_f32() * 1000.0);
        if self.times.len() > 120 {
            self.times.pop_front();
        }
        self.last_present = now;
        self.presented += 1;
        self.title_presents += 1;
    }

    fn update_counters(&mut self) {
        let now = Instant::now();
        let secs = (now - self.title_at).as_secs_f32();
        if secs < 1.0 {
            return;
        }
        self.shown_fps = self.title_presents as f32 / secs;
        self.game_fps = self.title_games as f32 / secs;
        (self.title_at, self.title_presents, self.title_games) = (now, 0, 0);
        if self.opts.log_fps {
            // Render-thread share of this second: taking fields in,
            // preparing them, showing frames.
            let [i, p, f] = std::mem::take(&mut self.spent).map(|d| d.as_secs_f32() / secs * 100.0);
            say!(
                "game frame {}: shown {:.1} fps, game {:.1} fps ({:.0}%), emulation {:.1} ms, render {:.2} ms (thread: fields {i:.0}%, prepare {p:.0}%, frames {f:.0}%)",
                self.games,
                self.shown_fps,
                self.game_fps,
                self.game_fps / 59.94 * 100.0,
                self.emu_ms,
                self.render_ms
            );
        }
    }

    /// The window title names the frame-rate target. It is not rewritten
    /// with live numbers every second, as it once was: setting a title can
    /// block the window's thread for 15-40 ms (more with overlays and
    /// accessibility tools hooked into the window), long enough to drop
    /// frames at 400 fps. The live numbers are in the performance monitor
    /// (F3) and `--log-fps`.
    fn title(&self) -> String {
        format!("New Super Mario Bros. Wii - Rust port - {} fps", settings::fps_label(self.settings.fps).to_lowercase())
    }

    fn overlay_rects(&self, width: u32, height: u32) -> Vec<vk_render::OverlayRect> {
        let mut out = Vec::new();
        let px = (height / 360).clamp(1, 4) as i32;
        let margin = 4 * px;
        if self.settings.perf_monitor {
            let budget = match self.settings.fps {
                0 => 1000.0 / self.shown_fps.max(1.0),
                60 => 1000.0 / 59.94,
                n => 1000.0 / n as f32,
            };
            let lines = vec![
                (overlay::WHITE, format!("FPS {:.1} / {}", self.shown_fps, settings::fps_label(self.settings.fps))),
                (overlay::WHITE, format!("GAME {:.1} FPS  CPU {:.1} MS", self.game_fps, self.emu_ms)),
                (overlay::WHITE, format!("RENDER {:.1} MS  VSYNC {}", self.render_ms, on(self.settings.vsync))),
                (overlay::WHITE, {
                    let on: Vec<String> = self.remotes.iter().enumerate().filter(|(_, r)| r.connected).map(|(i, _)| format!("P{}", i + 1)).collect();
                    format!("WII REMOTES {}", if on.is_empty() { "NONE".to_string() } else { on.join(" ") })
                }),
            ];
            // Its size (Settings > Display): 50% is the base size.
            let px = ((px * self.settings.perf_size as i32 + 25) / 50).max(1);
            let (_, h) = overlay::panel(&mut out, margin, margin, px, &lines);
            let times: Vec<f32> = self.times.iter().copied().collect();
            overlay::graph(&mut out, margin, margin + h + px, px, &times, budget);
        }
        if self.help {
            let lines: Vec<(u8, String)> = [
                "F1  THIS HELP".to_string(),
                format!("F3  PERFORMANCE MONITOR ({})", on(self.settings.perf_monitor)),
                format!("F4  BATTERY SAVER ({})", on(self.settings.battery_saver)),
                format!("F5/F6  FRAME RATE ({})", settings::fps_label(self.settings.fps)),
                format!("F7  V-SYNC ({})", on(self.settings.vsync)),
                format!("F9  ASPECT ({}; GAME AT NEXT START)", self.settings.aspect_name().to_uppercase()),
                "F10 SETTINGS".to_string(),
                format!("F11 FULLSCREEN ({})", on(self.settings.fullscreen)),
            ]
            .into_iter()
            .chain(self.settings.bt_passthrough.then(|| "F8  PAIR WII REMOTES (HOLD 10 S: FORGET)".to_string()))
            .map(|l| (overlay::WHITE, l))
            .collect();
            let w = lines.iter().map(|(_, l)| l.len() as i32).max().unwrap_or(0) * 6 * px + 4 * px;
            overlay::panel(&mut out, (width as i32 - w - margin).max(0), margin, px, &lines);
        }
        if let Some((release, _)) = self.update.as_ref().filter(|(_, found)| found.elapsed() < UPDATE_NOTE) {
            let lines = [(overlay::WHITE, format!("UPDATE {} IS OUT - SEE SETTINGS (F10)", release.version))];
            let h = lines.len() as i32 * 9 * px + 3 * px;
            overlay::panel(&mut out, margin, (height as i32 - h - margin).max(0), px, &lines);
        }
        out
    }

    /// A click (or tap) at window pixel (x, y), in layout units for the
    /// game thread: the picture, letterboxed to its aspect, is 456 units
    /// high with the origin at its centre and y up.
    fn click_at(&mut self, x: f64, y: f64) {
        let (Some(window), Some(emu)) = (&self.window, &self.emu) else { return };
        // The save select's Back button is the window's own.
        if self.menus.as_ref().is_some_and(|m| m.on_back(window, x, y)) {
            let _ = emu.to_emu.send(ToEmu::FileBack);
            return;
        }
        let size = window.inner_size();
        let (w, h) = (f64::from(size.width.max(1)), f64::from(size.height.max(1)));
        let aspect = if self.settings.widescreen { 16.0 / 9.0 } else { 4.0 / 3.0 };
        let picture_h = if w / h > aspect { h } else { w / aspect };
        let scale = picture_h / 456.0;
        let c = crate::click::Click { x: ((x - w / 2.0) / scale) as f32, y: ((h / 2.0 - y) / scale) as f32, half_width: (w / 2.0 / scale) as f32 };
        let _ = emu.to_emu.send(ToEmu::Click(c));
    }

    /// The mouse moved or clicked, or the screen was touched: the save
    /// select's Back button shows (until a key or controller is pressed).
    fn pointer_used(&mut self) {
        if let Some(m) = &mut self.menus {
            m.set_pointer_used(true);
        }
    }

    /// Bluetooth passthrough's pairing key: a press looks for remotes to
    /// pair; held 10 s, it forgets them all (`sync_held`).
    fn sync_key(&mut self, down: bool) {
        if down {
            self.sync_down = Some((Instant::now(), false));
        } else if let Some((_, false)) = self.sync_down.take() {
            if let Some(emu) = &self.emu {
                let _ = emu.to_emu.send(ToEmu::SyncButton(false));
            }
        }
    }

    fn sync_held(&mut self) {
        if let (Some((at, forgot @ false)), Some(emu)) = (&mut self.sync_down, &self.emu) {
            if at.elapsed() >= Duration::from_secs(10) {
                *forgot = true;
                let _ = emu.to_emu.send(ToEmu::SyncButton(true));
            }
        }
    }

    fn key(&mut self, code: KeyCode) {
        let changed = match code {
            KeyCode::F1 => {
                self.help = !self.help;
                false
            }
            KeyCode::F3 => {
                self.settings.perf_monitor = !self.settings.perf_monitor;
                true
            }
            KeyCode::F4 => {
                self.settings.battery_saver = !self.settings.battery_saver;
                true
            }
            KeyCode::F5 | KeyCode::F6 => {
                self.settings.step_fps(code == KeyCode::F6);
                self.next_display = Instant::now();
                say!("frame rate: {}", settings::fps_label(self.settings.fps));
                if let Some(w) = &self.window {
                    w.set_title(&self.title());
                }
                true
            }
            KeyCode::F7 => {
                self.settings.vsync = !self.settings.vsync;
                if let Some(r) = &mut self.renderer {
                    r.set_vsync(self.settings.vsync);
                }
                true
            }
            KeyCode::F9 => {
                self.settings.widescreen = !self.settings.widescreen;
                if let Err(e) = hle_ios::nand::set_widescreen(Path::new("saves/nand"), self.settings.widescreen) {
                    say_err!("SYSCONF: {e}");
                }
                true
            }
            KeyCode::F10 => {
                if let Some(m) = &mut self.menus {
                    m.toggle();
                }
                false
            }
            KeyCode::F11 => {
                self.settings.fullscreen = !self.settings.fullscreen;
                if let Some(w) = &self.window {
                    w.set_fullscreen(self.settings.fullscreen.then_some(Fullscreen::Borderless(None)));
                }
                true
            }
            _ => false,
        };
        if changed {
            self.settings.save(Path::new(settings::FILE));
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        if self.window.is_none() {
            if let Err(e) = self.start(event_loop) {
                self.error = Some(e);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        // A player's own window: closing it closes only that window.
        if let Some(i) = self.players.iter().position(|w| w.window.id() == id) {
            match event {
                WindowEvent::CloseRequested => {
                    self.players.remove(i);
                    return;
                }
                WindowEvent::Resized(_) => {
                    if let Some(tx) = &self.players[i].tx {
                        let _ = tx.send(ToPlayer::Resized);
                    }
                    return;
                }
                WindowEvent::RedrawRequested => return,
                _ => {}
            }
        }
        // Mouse as Gyro follows the pointer while no menu has it.
        if let (WindowEvent::CursorMoved { position, .. }, Some(window)) = (&event, &self.window) {
            if !self.menus.as_ref().is_some_and(ui::Menus::is_open) {
                self.inputs.pointer(position.x as f32, window.inner_size().width as f32);
            }
        }
        if let WindowEvent::MouseInput { state, button, .. } = &event {
            let n = mouse_button(*button);
            let down = *state == ElementState::Pressed;
            // The Keybinds tab waiting for keys takes the mouse's buttons too
            // (a release still reaches the window, which saw the press that
            // began the wait).
            if self.menus.as_ref().is_some_and(ui::Menus::capturing_keys) {
                let changes = self.menus.as_mut().map(|m| m.capture_mouse(&mut self.settings, n, down)).unwrap_or_default();
                self.apply_changes(changes);
                if down {
                    return;
                }
            }
            // Let go always counts (it may have been pressed before a menu
            // opened); pressed only while no menu has the mouse.
            if (!down || !self.menus.as_ref().is_some_and(ui::Menus::is_open)) && self.inputs.mouse_button(n, down) && down {
                self.note_device(ui::prompts::Style::Keys);
            }
        }
        if let (Some(menus), Some(window)) = (&mut self.menus, &self.window) {
            if menus.on_event(window, &event) {
                return;
            }
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                if let Some(r) = &mut self.renderer {
                    r.resized();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let down = event.state == ElementState::Pressed;
                    // The Keybinds tab waiting for keys takes them, held and let
                    // go (a release still reaches the game's keys).
                    if self.menus.as_ref().is_some_and(ui::Menus::capturing) {
                        if !event.repeat {
                            let changes = self.menus.as_mut().map(|m| m.capture_key(&mut self.settings, code, down)).unwrap_or_default();
                            self.apply_changes(changes);
                        }
                        if down {
                            return;
                        }
                    }
                    if !event.repeat {
                        if code == KeyCode::F8 && self.settings.bt_passthrough {
                            self.sync_key(down);
                        } else if self.inputs.key(code, down) {
                            if down {
                                self.note_device(ui::prompts::Style::Keys);
                                if let Some(m) = &mut self.menus {
                                    m.set_pointer_used(false);
                                }
                            }
                        } else if down {
                            self.key(code);
                        }
                    }
                }
            }
            WindowEvent::Focused(false) => self.inputs.release_keys(),
            // Clicks and taps on the game's menus (`crate::click`).
            WindowEvent::CursorMoved { position, .. } => {
                if self.pointer.is_some_and(|p| p != (position.x, position.y)) {
                    self.pointer_used();
                }
                self.pointer = Some((position.x, position.y));
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: winit::event::MouseButton::Left, .. } => {
                self.pointer_used();
                if let Some((x, y)) = self.pointer {
                    self.click_at(x, y);
                }
            }
            WindowEvent::Touch(t) if t.phase == winit::event::TouchPhase::Started => {
                self.pointer_used();
                self.click_at(t.location.x, t.location.y);
            }
            _ => {}
        }
    }

    /// Frames are driven from here, every turn of the event loop, rather than
    /// by redraw requests: Windows delivers those as paint messages, which it
    /// may hold back (every ~9 ms, or far longer, while the window is covered
    /// or in the background), and that capped the frame rate whatever the
    /// renderer could do. `frame` paces itself (the next field, or the next
    /// frame's time).
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() || self.done {
            return;
        }
        if let Err(e) = self.frame(event_loop) {
            self.error = Some(e);
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        crate::say::flush();
        if let Some(emu) = &mut self.emu {
            report(&emu.stop());
        }
        self.players.clear();
        self.renderer = None;
    }
}

/// A mouse button as a binding's number: 1 left, 2 middle, 3 right, 4 back,
/// 5 forward; 0 for others.
fn mouse_button(b: winit::event::MouseButton) -> u8 {
    use winit::event::MouseButton;
    match b {
        MouseButton::Left => 1,
        MouseButton::Middle => 2,
        MouseButton::Right => 3,
        MouseButton::Back => 4,
        MouseButton::Forward => 5,
        MouseButton::Other(_) => 0,
    }
}

/// The exit report: guest speed, waits, recompiled share, light textures.
pub fn report(t: &crate::emu::Totals) {
    let secs = t.cpu_time.as_secs_f64();
    println!("guest: {} instructions in {secs:.2}s of host time ({:.1} M/s)", t.steps, t.steps as f64 / secs.max(1e-9) / 1e6);
    if !t.gx_wait.is_zero() {
        println!("of which {:.2}s waiting for the GX thread", t.gx_wait.as_secs_f64());
    }
    if !t.output_wait.is_zero() {
        println!("and {:.2}s more waiting for the GX thread's output at the ends of fields", t.output_wait.as_secs_f64());
    }
    if !t.thread_cpu.is_zero() {
        println!("game thread: {:.2}s of CPU time", t.thread_cpu.as_secs_f64());
    }
    if t.recomp_steps > 0 {
        println!(
            "recompiled code ran {:.2}% of them, entered from the dispatcher {} times ({} functions found changed in memory)",
            100.0 * t.recomp_steps as f64 / t.steps.max(1) as f64,
            t.recomp_entries,
            t.recomp_changed
        );
    }
    if !t.slow_fields.is_empty() {
        let mut worst = t.slow_fields.clone();
        worst.sort_by(|a, b| b.1.cmp(&a.1));
        let ms = |d: &Duration| d.as_secs_f64() * 1e3;
        let list: Vec<String> = worst.iter().take(8).map(|(i, d, gx)| format!("{i} ({:.1} ms, GX wait {:.1})", ms(d), ms(gx))).collect();
        println!("{} fields took longer than 16.7 ms to emulate; slowest: {}", t.slow_fields.len(), list.join(", "));
    }
    if t.audio_dropouts > 0 {
        println!("audio: {} dropouts (the game fell behind)", t.audio_dropouts);
    }
    if t.light_texture_calls > 0 {
        println!("light textures: {} of {} redraws skipped (nothing they read changed)", t.light_texture_skips, t.light_texture_calls);
    }
    if let Some(s) = &t.texture_pack {
        let n = |c: &std::sync::atomic::AtomicU64| c.load(std::sync::atomic::Ordering::Relaxed);
        println!("texture pack: {} distinct textures with a replacement, {} without; {} files unreadable; {} dumped", n(&s.matched), n(&s.missed), n(&s.failed), n(&s.dumped));
    }
}

/// Runs until the window closes (or `max_frames` game frames); returns
/// (game frames, frames presented).
pub fn run(opts: Options) -> Result<(u64, u64), String> {
    crate::win::tune_process();
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    let now = Instant::now();
    let settings = opts.settings.clone();
    let cheats = Arc::new(crate::gameplay::Cheats::new(settings.cheated));
    let characters = Arc::new(crate::gameplay::Characters::new(settings.file_characters));
    // Not in test runs (a frame count, scripted presses, screenshots).
    let perf = opts.perf.as_ref().map(crate::perf::Perf::open).transpose()?;
    let update_check = if opts.max_frames.is_none() && opts.emu.script.is_empty() && opts.shots.is_empty() && !opts.unpaced { check_for_update() } else { None };
    let mut app = App {
        opts,
        settings,
        emu: None,
        renderer: None,
        window: None,
        error: None,
        queue: VecDeque::new(),
        mover: None,
        games: 0,
        presented: 0,
        startup: Startup { on: true, began: now, notice: false, lift_at: None },
        last_present: now,
        next_display: now,
        times: VecDeque::new(),
        title_at: now,
        title_presents: 0,
        title_games: 0,
        shown_fps: 0.0,
        game_fps: 0.0,
        emu_ms: 0.0,
        render_ms: 0.0,
        spent: [Duration::ZERO; 3],
        help: false,
        slow_fields: 0,
        perf,
        done: false,
        inputs: input::Inputs::new(),
        sent_input: None,
        sent_interpolate: None,
        pending_shot: None,
        remotes: [RemoteStatus::default(); REMOTES],
        menus: None,
        menu_mask: 0,
        sent_pause: false,
        was_open: false,
        booted_widescreen: true,
        sent_display: None,
        settings_from_game: false,
        cheats,
        characters,
        unsaved: false,
        update_check,
        update: None,
        updater: None,
        pack_download: None,
        restart: None,
        prompt_style: ui::prompts::Style::Game,
        prompt_images: Vec::new(),
        prompts: None,
        prompt_cache: Vec::new(),
        prompt_pin: match std::env::var("NSMBW_PROMPTS").as_deref() {
            Ok("keys") => Some(ui::prompts::Style::Keys),
            Ok("pad") => Some(ui::prompts::Style::Pad),
            _ => None,
        },
        pointer: None,
        finder: None,
        relaunch: None,
        #[cfg(windows)]
        wii_remotes: None,
        sync_down: None,
        main_player: 0,
        players: Vec::new(),
        textures: HashMap::new(),
        online: false,
        backdrop: gx::backdrop::Backdrop::default(),
    };
    event_loop.run_app(&mut app).map_err(|e| e.to_string())?;
    if let Some(p) = &mut app.perf {
        p.finish();
    }
    // A room the LAN tab joined could not be joined (gone, or out of
    // reach): the game starts again on its own rather than closing.
    if let (Some(e), crate::net::Mode::Join { .. }) = (&app.error, &app.opts.emu.net) {
        if std::env::var_os(LAN_JOIN_ENV).is_some() {
            eprintln!("LAN: {e}; the game starts again on its own");
            app.relaunch = Some(Vec::new());
            app.restart = std::env::current_exe().ok();
        }
    }
    // An update was installed, or a LAN room joined, hosted or left: the
    // game again, now the window and the game have closed.
    if let Some(program) = app.restart.take() {
        crate::say::flush();
        let started = match app.relaunch.take() {
            Some(net) => {
                let mut game = std::process::Command::new(&program);
                game.args(relaunch_args(&net));
                // Only a join from the LAN tab falls back to playing alone.
                if net.first().map(String::as_str) == Some("--join") {
                    game.env(LAN_JOIN_ENV, "1");
                } else {
                    game.env_remove(LAN_JOIN_ENV);
                }
                game.spawn().map(|_| ()).map_err(|e| e.to_string())
            }
            None => port_update::install::restart(&program),
        };
        if let Err(e) = started {
            eprintln!("could not start the game again: {e}");
        }
    }
    match app.error {
        Some(e) => Err(e),
        None => Ok((app.games, app.presented)),
    }
}
