//! The Android app's native side, `libnsmbw.so` (docs/PORT_PLAN.md,
//! "Android"). The Java shell in `android/` (adapted from WitherBlitz's
//! NSMB-Wii-Recompilation app) owns the activity, the `SurfaceView`, the
//! on-screen Wii Remote, the phone's motion sensors, controllers and the
//! settings; it calls the `Native` class's functions below.
//!
//! The game thread is the desktop frontend's own (`emu.rs` and `say.rs`,
//! included from `crates/frontend/src`; `net` is the `netplay` crate's); `audio` and `win`
//! are this crate's versions of the frontend modules of those names. A
//! render thread of this crate takes each field from the game thread and
//! presents it at the game's own 60 fps, as the desktop window does at
//! 60 fps (`play.rs`). It keeps the renderer across surfaces: Android
//! destroys an app's surface whenever the app leaves the screen.
#![cfg(target_os = "android")]

#[allow(dead_code)]
#[path = "../../frontend/src/emu.rs"]
mod emu;
// Online multiplayer (shared with the desktop), and LAN Play's side of it.
use netplay::net;
mod lobby;
#[allow(dead_code, unused_imports, unused_macros)]
#[path = "../../frontend/src/say.rs"]
mod say;
// The game thread's clicks, Gameplay options, texture packs and the save
// cards' translated "Cheated", as on the desktop.
#[allow(dead_code)]
#[path = "../../frontend/src/click.rs"]
mod click;
#[allow(dead_code)]
#[path = "../../frontend/src/gameplay.rs"]
mod gameplay;
#[allow(dead_code)]
#[path = "../../frontend/src/mods.rs"]
mod mods;
// The save's backups at each start, as on the desktop.
#[path = "../../frontend/src/save_backup.rs"]
mod save_backup;
// Per-field timings: on while the app's files hold a file named `perf`
// (its text: the draw categories to leave out in turn, if any), into
// `perf.csv` there and logcat.
#[allow(dead_code)]
#[path = "../../frontend/src/perf.rs"]
mod perf;
mod ui {
    #[allow(dead_code)]
    #[path = "../../../frontend/src/ui/text.rs"]
    pub mod text;
}
// The Practice tab's speed, counts and overlays, and sound away from 100%.
#[allow(dead_code)]
#[path = "../../frontend/src/practice.rs"]
mod practice;
#[path = "../../frontend/src/stretch.rs"]
mod stretch;
#[allow(dead_code)]
#[path = "../../frontend/src/overlay.rs"]
mod overlay;
/// The desktop settings' texture-pack fields, all `mods.rs` reads (the app
/// has no Mods tab yet, so they stay off).
mod settings {
    #[derive(Default)]
    pub struct Settings {
        pub texture_pack: String,
        pub hd_textures: bool,
        pub texture_dumps: bool,
    }
}

mod audio;
mod gpu_driver;
mod logcat;
mod win;

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use hle_ios::bt::REMOTES;
use hle_ios::wiimote::Input;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jboolean, jbyteArray, jfloat, jint, jstring};
use jni::JNIEnv;
use ndk::native_window::NativeWindow;
use raw_window_handle::{AndroidDisplayHandle, AndroidNdkWindowHandle, RawDisplayHandle, RawWindowHandle};
use vk_render::Renderer;

use crate::emu::{Emu, EmuOptions, GameFrame, ToEmu, FIELD};

/// What the Java side chose at start.
struct Config {
    game: PathBuf,
    /// EFB resolution: 1..8 times 480 lines, 0 the screen's own height.
    scale: u32,
    widescreen: bool,
    vsync: bool,
    /// The Wii's language (`IPL.LNG`: 1 English, 3 French, 4 Spanish, 5
    /// Italian).
    language: u8,
}

enum ToRender {
    /// The surface was created or changed (`Some`) or is being destroyed
    /// (`None`); acknowledged once the renderer no longer uses the old one.
    Window(Option<NativeWindow>, SyncSender<()>),
    Resized,
    Paused(bool),
    /// Pause, after writing the game to this file (a backup in case
    /// Android ends the app while it is away); then say how that went.
    Backup(PathBuf, SyncSender<Result<(), String>>),
    /// A tap on the game off the on-screen remote's buttons.
    Tap,
}

/// A state file to start the game from instead of power-on (the backup of
/// a run Android ended), set before `start`.
static LOAD_STATE: Mutex<Option<PathBuf>> = Mutex::new(None);

/// The Gameplay options (`Native.setGameplay`), set before `start`.
static GAMEPLAY: Mutex<gameplay::Options> = Mutex::new(gameplay::Options { no_time_limit: false, infinite_lives: false, autosave: false, exit_course: false, p1_character: false, no_power_up_freeze: false, no_player_collision: false });
/// The app's Practice settings (`Native.setPractice`): the game speed and
/// Fast Forward's (percent, 0 unlimited), the level timer, the death counter
/// and the input display (0 off, 1 player 1, 2 every player).
#[derive(Clone, Copy)]
struct PracticeSettings {
    speed: u32,
    fast_forward_speed: u32,
    timer: bool,
    deaths: bool,
    inputs: u32,
}

static PRACTICE: Mutex<PracticeSettings> = Mutex::new(PracticeSettings { speed: 100, fast_forward_speed: 300, timer: false, deaths: false, inputs: 0 });
/// Fast Forward held (`Native.setFastForward`).
static FAST_FORWARD: AtomicBool = AtomicBool::new(false);
/// Restart Course pressed (`Native.restartCourse`), until the render thread
/// passes it on.
static RESTART: AtomicBool = AtomicBool::new(false);

struct Shared {
    tx: Mutex<Sender<ToRender>>,
    inputs: Mutex<[Input; REMOTES]>,
    /// Why the game or the renderer stopped, for the Java side to show.
    error: Mutex<Option<String>>,
    /// Wii Remote 1's rumble motor, as of the last field shown.
    rumble: AtomicBool,
}

static SHARED: OnceLock<Shared> = OnceLock::new();

/// Fields kept after they are drawn before their evicted textures are freed
/// and their buffers go back to the GX threads (as `play.rs` keeps them).
const KEEP: usize = 3;

fn handles(w: &NativeWindow) -> (RawDisplayHandle, RawWindowHandle) {
    let window = AndroidNdkWindowHandle::new(w.ptr().cast());
    (RawDisplayHandle::Android(AndroidDisplayHandle::new()), RawWindowHandle::AndroidNdk(window))
}

struct Render {
    cfg: Config,
    // Before `window`: the Vulkan surface goes first.
    renderer: Option<Renderer>,
    window: Option<NativeWindow>,
    emu: Option<Emu>,
    paused: bool,
    failed: bool,
    kept: VecDeque<gx::Output>,
    sent: Option<[Input; REMOTES]>,
    /// Speed over the last [`SPEED_EVERY`]: since when, fields, and their
    /// emulation and render times.
    speed: (Instant, u32, Duration, Duration),
    /// A backup being written: the game thread's answer, who waits for it,
    /// and the file.
    backup: Option<(Receiver<Result<(), String>>, SyncSender<Result<(), String>>)>,
    /// The last backup written: removed when the game carries on.
    backup_file: Option<PathBuf>,
    /// The state the game started from: removed once it has shown a field.
    loaded: Option<PathBuf>,
    perf: Option<perf::Perf>,
    /// A LAN game (which runs at 100%), the speed the game thread was last
    /// told, and the last field's Practice counts and controls.
    net_game: bool,
    sent_speed: Option<u32>,
    practice: practice::Stats,
    shown_inputs: [Input; REMOTES],
}

/// How often the game's speed goes to logcat.
const SPEED_EVERY: Duration = Duration::from_secs(10);

impl Render {
    fn fail(&mut self, e: String) {
        eprintln!("stopped: {e}");
        if let Some(s) = SHARED.get() {
            *s.error.lock().unwrap() = Some(e);
        }
        self.failed = true;
        if let Some((_, ack)) = self.backup.take() {
            let _ = ack.send(Err("the game stopped".into()));
        }
    }

    /// Whether fields are taken from the game: while it runs, and while a
    /// backup waits for the game thread to stop.
    fn busy(&self) -> bool {
        // A LAN game runs on while the app is away: the others wait for it.
        self.emu.is_some() && (!self.paused || self.backup.is_some() || lobby::online()) && !self.failed
    }

    /// Hands on the game thread's answer about a backup, once it is in.
    fn check_backup(&mut self) {
        let Some((rx, _)) = &self.backup else { return };
        let result = match rx.try_recv() {
            Ok(r) => r,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Err("the game thread stopped".into()),
        };
        let (_, ack) = self.backup.take().unwrap();
        if let Err(e) = &result {
            println!("backup: {e}");
        }
        let _ = ack.send(result);
    }

    fn handle(&mut self, msg: ToRender) {
        match msg {
            ToRender::Window(window, ack) => {
                match window {
                    Some(w) => {
                        if let Err(e) = self.attach(w) {
                            self.fail(format!("Vulkan: {e}"));
                        }
                    }
                    None => {
                        if let Some(r) = &mut self.renderer {
                            r.detach_window();
                        }
                        self.window = None;
                    }
                }
                let _ = ack.send(());
            }
            ToRender::Resized => {
                if let Some(r) = &mut self.renderer {
                    r.resized();
                }
            }
            ToRender::Paused(p) => {
                if !p {
                    // Back in the app: the backup is not needed. One still
                    // being written is waited for first (the game thread
                    // writes it before it runs on).
                    if let Some((rx, ack)) = self.backup.take() {
                        let result = rx.recv_timeout(Duration::from_secs(10)).unwrap_or_else(|_| Err("timed out".into()));
                        let _ = ack.send(result);
                    }
                    if let Some(f) = self.backup_file.take() {
                        let _ = std::fs::remove_file(f);
                    }
                }
                self.paused = p;
                audio::set_paused(p);
                if let Some(emu) = &self.emu {
                    if !lobby::online() {
                        let _ = emu.to_emu.send(ToEmu::Pause(p));
                    }
                }
            }
            ToRender::Tap => {
                if let Some(emu) = &self.emu {
                    let _ = emu.to_emu.send(ToEmu::Tap);
                }
            }
            ToRender::Backup(path, ack) if lobby::online() => {
                // A net game doesn't stop for a backup: the others wait for it.
                self.paused = true;
                audio::set_paused(true);
                let _ = ack.send(Err(format!("a LAN game keeps running, no backup of {}", path.display())));
            }
            ToRender::Backup(path, ack) => {
                self.paused = true;
                audio::set_paused(true);
                match &self.emu {
                    Some(emu) if !self.failed && self.backup.is_none() => {
                        let (tx, rx) = mpsc::channel();
                        let _ = emu.to_emu.send(ToEmu::Pause(true));
                        let _ = emu.to_emu.send(ToEmu::SaveState(path.clone(), tx));
                        self.backup = Some((rx, ack));
                        self.backup_file = Some(path);
                    }
                    _ => {
                        let _ = ack.send(Err("no game running".into()));
                    }
                }
            }
        }
    }

    /// The first surface makes the renderer and starts the game; later
    /// ones replace the surface it presents to.
    fn attach(&mut self, w: NativeWindow) -> Result<(), String> {
        let (display, handle) = handles(&w);
        if let Some(r) = &mut self.renderer {
            r.attach_window(display, handle).map_err(|e| e.to_string())?;
            self.window = Some(w);
            return Ok(());
        }
        let short = w.width().min(w.height()).max(1) as u32;
        let scale = if self.cfg.scale == 0 { short.div_ceil(480).clamp(1, 8) } else { self.cfg.scale };
        let _ = std::fs::create_dir_all("cache");
        let opts = vk_render::Options { scale: scale as f32, vsync: self.cfg.vsync, pipeline_cache: Some(PathBuf::from("cache/pipelines.bin")), efb_copies_to_ram: false };
        let mut r = Renderer::new(display, handle, opts).map_err(|e| e.to_string())?;
        r.aspect = if self.cfg.widescreen { 16.0 / 9.0 } else { 4.0 / 3.0 };
        println!("vulkan device: {}; EFB {scale}x; surface {}x{}; V-Sync {}{}", r.device_name, w.width(), w.height(), self.cfg.vsync, if r.validation { "; validation layer on" } else { "" });
        self.renderer = Some(r);
        self.window = Some(w);
        // The game reads its aspect ratio and language from SYSCONF once, at boot.
        if let Err(e) = set_sysconf(Path::new("saves/nand"), self.cfg.widescreen, self.cfg.language) {
            eprintln!("SYSCONF: {e}");
        }
        // For silent test runs: a file named `no-audio` in the app's files
        // (`adb shell run-as com.wither.nsmbwport touch files/no-audio`).
        let audio = !Path::new("no-audio").exists();
        if !audio {
            println!("sound off (files/no-audio)");
        }
        // A LAN game (`Native.setNet`): hosting answers room queries; a
        // Tailscale host joined is asked for rooms from now on.
        let (net, name) = lobby::take_net();
        self.net_game = !matches!(net, net::Mode::Off);
        match &net {
            net::Mode::Host { port, .. } => {
                if let Err(e) = netplay::lan::announce(*port, name) {
                    eprintln!("LAN: can't answer room queries: {e}");
                }
            }
            net::Mode::Join { addr } => netplay::lan::joined(addr),
            net::Mode::Off => {}
        }
        // A backup is never loaded into a net game: every machine has to
        // start from power-on.
        let mut load_state = LOAD_STATE.lock().unwrap().take().filter(|_| matches!(net, net::Mode::Off));
        if let Some(p) = &load_state {
            println!("starting from the backup {}", p.display());
        }
        self.loaded = load_state.clone();
        // For measuring: a save state named `state` in the app's files
        // (`--save-state`'s format) is started from, as `--load-state`, and
        // kept (unlike a backup).
        if load_state.is_none() && matches!(net, net::Mode::Off) && Path::new("state").is_file() {
            println!("state: starting from files/state");
            load_state = Some(PathBuf::from("state"));
        }
        if let Ok(skip) = std::fs::read_to_string("perf") {
            match perf::Skip::parse_list(skip.trim()).and_then(|s| perf::Perf::new(Some(Path::new("perf.csv")), s, 300)) {
                Ok(p) => {
                    println!("perf: timing every field into perf.csv (files/perf)");
                    self.perf = Some(p);
                }
                Err(e) => eprintln!("perf: {e}"),
            }
        }
        // A scripted run, for measuring (as `--press-file`): a file named
        // `press` in the app's files.
        let script = match std::fs::read_to_string("press").map(|t| emu::parse_script(&t)) {
            Ok(Ok(s)) => {
                println!("press: {} scripted presses (files/press)", s.len());
                s
            }
            Ok(Err(e)) => {
                eprintln!("press: {e}");
                Vec::new()
            }
            Err(_) => Vec::new(),
        };
        // Before the game opens its save.
        for r in save_backup::backup(Path::new("saves/nand"), Path::new(save_backup::DIR), std::time::SystemTime::now()) {
            match r {
                Ok(p) => println!("save backup: {}", p.display()),
                Err(e) => eprintln!("save backup: {e}"),
            }
        }
        let gameplay = *GAMEPLAY.lock().unwrap();
        let emu = Emu::spawn(self.cfg.game.clone(), EmuOptions { audio, net, load_state, script, gameplay, ..EmuOptions::default() });
        let _ = emu.to_emu.send(ToEmu::Interpolate(false));
        if self.paused {
            let _ = emu.to_emu.send(ToEmu::Pause(true));
        }
        self.emu = Some(emu);
        Ok(())
    }

    /// Draws a field and presents it (when there is a surface and
    /// `present`: fields that came faster than the screen are only drawn),
    /// and sends the game what only the GPU knows.
    fn ingest(&mut self, mut f: GameFrame, present: bool) -> Result<(), String> {
        let (w, h) = self.window.as_ref().filter(|_| present).map_or((0, 0), |w| (w.width().max(0) as u32, w.height().max(0) as u32));
        (self.practice, self.shown_inputs) = (f.practice, f.inputs);
        let overlay = if w > 0 && h > 0 { self.overlay(w as i32, h as i32) } else { Vec::new() };
        let (Some(r), Some(emu)) = (&mut self.renderer, &self.emu) else { return Ok(()) };
        if let Some(p) = &self.perf {
            p.filter(&mut f.out);
        }
        let started = Instant::now();
        if let Some(p) = self.loaded.take() {
            let _ = std::fs::remove_file(p);
        }
        r.overlay = overlay;
        r.render(&f.out, &f.out.vertices, true, f.xfb, w, h).map_err(|e| e.to_string())?;
        if let Some(p) = &mut self.perf {
            p.field(&f, started.elapsed(), r.gpu_time());
        }
        self.speed.1 += 1;
        self.speed.2 += f.emu;
        self.speed.3 += started.elapsed();
        let secs = self.speed.0.elapsed().as_secs_f32();
        if secs >= SPEED_EVERY.as_secs_f32() {
            let (n, ms) = (self.speed.1.max(1), |d: Duration| d.as_secs_f32() * 1000.0);
            let (textures, used, reserved) = r.memory();
            println!(
                "field {}: game {:.0}% ({:.1} fields/s), emulation {:.1} ms and render {:.1} ms a field; {} of {} draws specialized; {textures} textures, GPU memory {} MB used of {} MB",
                f.index,
                self.speed.1 as f32 / secs / 59.94 * 100.0,
                self.speed.1 as f32 / secs,
                ms(self.speed.2) / n as f32,
                ms(self.speed.3) / n as f32,
                r.stats.specialized,
                r.stats.draws,
                used >> 20,
                reserved >> 20
            );
            self.speed = (Instant::now(), 0, Duration::ZERO, Duration::ZERO);
        }
        let writebacks = r.take_writebacks();
        if !writebacks.is_empty() {
            let _ = emu.to_emu.send(ToEmu::Writebacks(writebacks));
        }
        if f.wants_efb {
            let snap = r.efb_snapshot().map_err(|e| e.to_string())?;
            let _ = emu.to_emu.send(ToEmu::Efb(snap));
        }
        if let Some(s) = SHARED.get() {
            s.rumble.store(f.remotes[0].rumble, Ordering::Relaxed);
        }
        self.kept.push_back(f.out);
        while self.kept.len() > KEEP {
            let old = self.kept.pop_front().unwrap();
            r.evict(&old.evicted);
            let _ = emu.to_emu.send(ToEmu::Recycle(Box::new(old)));
        }
        Ok(())
    }

    /// The Practice overlays for a `w` x `h` surface.
    fn overlay(&self, w: i32, h: i32) -> Vec<vk_render::OverlayRect> {
        let p = *PRACTICE.lock().unwrap();
        let mut out = Vec::new();
        let px = (h.min(w) / 360).clamp(1, 4);
        let fast_forward = !self.net_game && FAST_FORWARD.load(Ordering::Relaxed);
        let view = practice::View { stats: self.practice, speed: self.sent_speed.unwrap_or(100), fast_forward, timer: p.timer, deaths: p.deaths, input_display: p.inputs, inputs: &self.shown_inputs };
        practice::draw(&mut out, w, h, px, &view);
        out
    }

    fn run(mut self, rx: Receiver<ToRender>) {
        loop {
            let busy = self.busy();
            // With nothing to draw, wait for the Java side.
            let first = if busy {
                rx.try_recv().ok()
            } else {
                match rx.recv() {
                    Ok(m) => Some(m),
                    Err(_) => break,
                }
            };
            if let Some(m) = first {
                self.handle(m);
                while let Ok(m) = rx.try_recv() {
                    self.handle(m);
                }
            }
            self.check_backup();
            if !self.busy() {
                continue;
            }
            let Some(emu) = &self.emu else { continue };
            let inputs = SHARED.get().map(|s| *s.inputs.lock().unwrap()).unwrap_or_default();
            if self.sent != Some(inputs) {
                let was = self.sent.map(|s| (s[0].buttons, s[0].shake));
                if was != Some((inputs[0].buttons, inputs[0].shake)) {
                    println!("Wii Remote 1: buttons {:#06x}{}", inputs[0].buttons, if inputs[0].shake { ", shaking" } else { "" });
                }
                let _ = emu.to_emu.send(ToEmu::Input(inputs));
                self.sent = Some(inputs);
            }
            // The game speed, or Fast Forward's while it is held (a LAN game
            // runs at 100%).
            let p = *PRACTICE.lock().unwrap();
            let speed = if self.net_game {
                100
            } else if FAST_FORWARD.load(Ordering::Relaxed) {
                p.fast_forward_speed
            } else {
                p.speed
            };
            if self.sent_speed != Some(speed) {
                println!("game speed: {}%", speed);
                let _ = emu.to_emu.send(ToEmu::Speed(speed));
                self.sent_speed = Some(speed);
            }
            // Restart Course (the game thread ignores it in a LAN game).
            if RESTART.swap(false, Ordering::Relaxed) {
                println!("restart course");
                let _ = emu.to_emu.send(ToEmu::RestartCourse);
            }
            // While paused nothing takes fields: the game thread stops at
            // its next one, its frame queue full.
            // While paused for a backup the game thread stops at its next
            // field; the fields before it are drawn (unseen) meanwhile.
            match emu.frames.recv_timeout(FIELD) {
                Ok(Ok(f)) => {
                    // Fields that came faster than they are shown (above
                    // 100% game speed): only the newest is presented.
                    let mut fields = vec![Ok(f)];
                    fields.extend(emu.frames.try_iter());
                    let count = fields.len();
                    for (i, f) in fields.into_iter().enumerate() {
                        if let Err(e) = f.and_then(|f| self.ingest(f, i + 1 == count)) {
                            self.fail(e);
                            break;
                        }
                    }
                }
                Ok(Err(e)) => self.fail(e),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => self.fail("the game thread stopped".into()),
            }
        }
        if let Some(emu) = &mut self.emu {
            emu.stop();
        }
    }
}

/// The console's aspect ratio and language in the NAND's `SYSCONF` (as
/// `hle_ios::nand::set_widescreen` does for the aspect ratio).
fn set_sysconf(root: &Path, widescreen: bool, language: u8) -> std::io::Result<()> {
    hle_ios::nand::ensure_system_files(root)?;
    let path = root.join("shared2/sys/SYSCONF");
    let mut conf = std::fs::read(&path)?;
    let a = hle_ios::nand::set_sysconf_byte(&mut conf, "IPL.AR", u8::from(widescreen));
    let l = hle_ios::nand::set_sysconf_byte(&mut conf, "IPL.LNG", language);
    if a || l {
        std::fs::write(&path, conf)?;
    }
    Ok(())
}

fn java_string(env: &mut JNIEnv, s: &JString) -> Result<String, String> {
    env.get_string(s).map(Into::into).map_err(|e| e.to_string())
}

/// `Native.start(filesDir, gameDir, scale, widescreen, vsync, language)`: once per
/// process. Saves, settings and caches go under `filesDir` (the working
/// folder); `gameDir` holds the extracted disc (`sys/`, `files/`). The game
/// starts with the first surface. Returns null, or why it cannot start.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_start<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, files: JString<'l>, game: JString<'l>, scale: jint, widescreen: jboolean, vsync: jboolean, language: jint) -> jstring {
    logcat::redirect_std();
    let started = (|| -> Result<(), String> {
        let files = java_string(&mut env, &files)?;
        let game = java_string(&mut env, &game)?;
        if SHARED.get().is_some() {
            return Ok(());
        }
        std::env::set_current_dir(&files).map_err(|e| format!("{files}: {e}"))?;
        logcat::keep_in(Path::new("logs"));
        println!("NSMBW port {} on {}", env!("CARGO_PKG_VERSION"), std::env::consts::ARCH);
        // For finding renderer bugs: a file named `validation` in the app's
        // files turns the Vulkan validation layer on, when the APK carries
        // it (`android/build.ps1 -ValidationLayer`). Set before any other
        // thread of ours runs.
        if Path::new("validation").exists() {
            std::env::set_var("NSMBW_VALIDATION", "1");
        }
        let (tx, rx) = mpsc::channel();
        let mut inputs = [Input::default(); REMOTES];
        inputs[0].connected = true;
        SHARED.set(Shared { tx: Mutex::new(tx), inputs: Mutex::new(inputs), error: Mutex::new(None), rumble: AtomicBool::new(false) }).map_err(|_| "already started".to_string())?;
        let cfg = Config { game: PathBuf::from(game), scale: scale.clamp(0, 8) as u32, widescreen: widescreen != 0, vsync: vsync != 0, language: u8::try_from(language).unwrap_or(1) };
        println!("starting: game {}, files {files}", cfg.game.display());
        let render = Render { cfg, renderer: None, window: None, emu: None, paused: false, failed: false, kept: VecDeque::new(), sent: None, speed: (Instant::now(), 0, Duration::ZERO, Duration::ZERO), backup: None, backup_file: None, loaded: None, perf: None, net_game: false, sent_speed: None, practice: practice::Stats::default(), shown_inputs: [Input::default(); REMOTES] };
        std::thread::Builder::new().name("render".into()).spawn(move || render.run(rx)).map_err(|e| e.to_string())?;
        Ok(())
    })();
    match started {
        Ok(()) => std::ptr::null_mut(),
        Err(e) => env.new_string(e).map(JString::into_raw).unwrap_or(std::ptr::null_mut()),
    }
}

fn send(msg: ToRender) -> bool {
    SHARED.get().is_some_and(|s| s.tx.lock().unwrap().send(msg).is_ok())
}

/// `Native.setSurface(surface)`: the game's `SurfaceView` surface, or null
/// while it is being destroyed. Returns once the renderer has let go of
/// the old one (Android needs that before `surfaceDestroyed` returns).
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setSurface<'l>(env: JNIEnv<'l>, _: JClass<'l>, surface: JObject<'l>) {
    let window = if surface.is_null() {
        None
    } else {
        // SAFETY: a valid JNIEnv and android.view.Surface from the caller.
        unsafe { NativeWindow::from_surface(env.get_raw().cast(), surface.as_raw().cast()) }
    };
    let (ack, done) = mpsc::sync_channel(1);
    if send(ToRender::Window(window, ack)) {
        let _ = done.recv_timeout(Duration::from_secs(3));
    }
}

/// `Native.surfaceResized()`: the surface changed size.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_surfaceResized<'l>(_: JNIEnv<'l>, _: JClass<'l>) {
    send(ToRender::Resized);
}

/// `Native.setRemote(player, connected, buttons, shake, tilt)`: Wii Remote
/// `player` (0..3): its buttons as `hle_ios::wiimote::button` bits, shake,
/// and tilt -1..1.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setRemote<'l>(_: JNIEnv<'l>, _: JClass<'l>, player: jint, connected: jboolean, buttons: jint, shake: jboolean, tilt: jfloat) {
    let Some(s) = SHARED.get() else { return };
    let Some(slot) = usize::try_from(player).ok().filter(|&p| p < REMOTES) else { return };
    let input = Input { connected: connected != 0, buttons: buttons as u16 & hle_ios::wiimote::button::ALL, shake: shake != 0, tilt: if tilt.is_finite() { tilt.clamp(-1.0, 1.0) } else { 0.0 } };
    s.inputs.lock().unwrap()[slot] = input;
}

/// `Native.setPaused(paused)`: the app left the screen (or came back). The
/// game stops within a field or so and nothing plays. Coming back removes
/// the backup `backup` wrote.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setPaused<'l>(_: JNIEnv<'l>, _: JClass<'l>, paused: jboolean) {
    send(ToRender::Paused(paused != 0));
}

/// `Native.backup(path, timeoutMs)`: pauses as `setPaused(true)` does, after
/// writing the game to `path` (a state file); returns null once it is
/// written, else why not (a net game, which doesn't pause; the time ran
/// out). The file appears whole or not at all.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_backup<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, path: JString<'l>, timeout_ms: jint) -> jstring {
    let result = (|| -> Result<(), String> {
        let path = PathBuf::from(java_string(&mut env, &path)?);
        let (ack, done) = mpsc::sync_channel(1);
        if !send(ToRender::Backup(path, ack)) {
            return Err("the game has not started".into());
        }
        let started = Instant::now();
        let result = done.recv_timeout(Duration::from_millis(timeout_ms.max(0) as u64)).unwrap_or_else(|_| Err("timed out".into()));
        if result.is_ok() {
            println!("backup written in {} ms", started.elapsed().as_millis());
        }
        result
    })();
    match result {
        Ok(()) => std::ptr::null_mut(),
        Err(e) => env.new_string(e).map(JString::into_raw).unwrap_or(std::ptr::null_mut()),
    }
}

/// `Native.setLoadState(path)`: before `start`, a state file (`backup`'s) to
/// start the game from instead of power-on; it is removed once the game
/// shows a field. Null: power-on.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setLoadState<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, path: JString<'l>) {
    let path = if path.is_null() { None } else { java_string(&mut env, &path).ok().map(PathBuf::from) };
    *LOAD_STATE.lock().unwrap() = path;
}

/// `Native.setGameplay(powerUpFreeze, playerCollision)`: before `start`, the
/// Gameplay options (both on: the game's own rules; a net game keeps them
/// whatever these say).
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setGameplay<'l>(_: JNIEnv<'l>, _: JClass<'l>, power_up_freeze: jboolean, player_collision: jboolean) {
    let mut g = GAMEPLAY.lock().unwrap();
    g.no_power_up_freeze = power_up_freeze == 0;
    g.no_player_collision = player_collision == 0;
}

/// `Native.tap()`: the game was tapped off the on-screen remote's buttons
/// (it skips the opening cutscene or a hint movie).
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_tap<'l>(_: JNIEnv<'l>, _: JClass<'l>) {
    send(ToRender::Tap);
}

/// `Native.setGraphics(compat, hookDir, driverDir, driverLib, tmpDir)`: before
/// `start`. `compat` is Graphics Compatibility (0 Auto, 1 On, 2 Off:
/// `vk_render::Compat`); with `hookDir` the renderer runs on the GPU driver
/// `driverLib` in `driverDir` through libadrenotools (`gpu_driver`). Returns
/// null, or why the driver can't be used (the phone's own is then).
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setGraphics<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, compat: jint, hook_dir: JString<'l>, driver_dir: JString<'l>, driver_lib: JString<'l>, tmp_dir: JString<'l>) -> jstring {
    logcat::redirect_std();
    vk_render::set_compat(match compat {
        1 => vk_render::Compat::On,
        2 => vk_render::Compat::Off,
        _ => vk_render::Compat::Auto,
    });
    if hook_dir.is_null() {
        return std::ptr::null_mut();
    }
    let result = (|| -> Result<(), String> {
        let (hooks, dir, lib, tmp) = (java_string(&mut env, &hook_dir)?, java_string(&mut env, &driver_dir)?, java_string(&mut env, &driver_lib)?, java_string(&mut env, &tmp_dir)?);
        gpu_driver::open(&hooks, &dir, &lib, &tmp)?;
        println!("GPU driver: {lib} (libadrenotools)");
        Ok(())
    })();
    match result {
        Ok(()) => std::ptr::null_mut(),
        Err(e) => {
            eprintln!("GPU driver: {e}; the phone's own is used");
            env.new_string(e).map(JString::into_raw).unwrap_or(std::ptr::null_mut())
        }
    }
}

/// `Native.setPractice(speed, fastForwardSpeed, timer, deaths, inputDisplay)`:
/// the Practice settings: the game speed and the speed while Fast Forward
/// is held, in percent (0 unlimited; a LAN game runs at 100%), the level
/// timer, the death counter, and the input display (0 off, 1 player 1, 2
/// every player). Any time; they apply at once.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setPractice<'l>(_: JNIEnv<'l>, _: JClass<'l>, speed: jint, fast_forward_speed: jint, timer: jboolean, deaths: jboolean, input_display: jint) {
    let speed = u32::try_from(speed).ok().filter(|s| [50, 100, 150, 200, 300, 0].contains(s)).unwrap_or(100);
    let fast_forward_speed = u32::try_from(fast_forward_speed).ok().filter(|s| [150, 200, 300, 0].contains(s)).unwrap_or(300);
    *PRACTICE.lock().unwrap() = PracticeSettings { speed, fast_forward_speed, timer: timer != 0, deaths: deaths != 0, inputs: input_display.clamp(0, 2) as u32 };
}

/// `Native.setFastForward(held)`: Fast Forward is held (the on-screen
/// button, a controller's right stick click); the game runs at the
/// Practice tab's Fast-Forward Speed meanwhile (not in a LAN game).
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setFastForward<'l>(_: JNIEnv<'l>, _: JClass<'l>, held: jboolean) {
    FAST_FORWARD.store(held != 0, Ordering::Relaxed);
}

/// `Native.restartCourse()`: Restart Course (the on-screen button, both
/// stick clicks). The game thread ignores it in a LAN game.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_restartCourse<'l>(_: JNIEnv<'l>, _: JClass<'l>) {
    RESTART.store(true, Ordering::Relaxed);
}

/// `Native.rumble()`: whether Wii Remote 1's rumble motor is on.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_rumble<'l>(_: JNIEnv<'l>, _: JClass<'l>) -> jboolean {
    SHARED.get().is_some_and(|s| s.rumble.load(Ordering::Relaxed)).into()
}

/// `Native.takeError()`: why the game stopped, once; else null.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_takeError<'l>(env: JNIEnv<'l>, _: JClass<'l>) -> jstring {
    let e = SHARED.get().and_then(|s| s.error.lock().unwrap().take());
    match e {
        Some(e) => env.new_string(e).map(JString::into_raw).unwrap_or(std::ptr::null_mut()),
        None => std::ptr::null_mut(),
    }
}

/// The disc image open as `fd`, which the app detached for us: ours to own
/// and close.
fn disc_image(fd: jint) -> Result<disc_image::Image, String> {
    // SAFETY: the app hands over a valid descriptor it no longer uses.
    let file = unsafe { <std::fs::File as std::os::fd::FromRawFd>::from_raw_fd(fd) };
    disc_image::Image::from_file(file)
}

/// `Native.discHeader(fd)`: the image's game ID and revision as the first 8
/// bytes of `sys/boot.bin` hold them (ID, disc number, revision), so the app
/// can refuse another disc before extracting it; null when it is not a Wii
/// disc image this can read.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_discHeader<'l>(env: JNIEnv<'l>, _: JClass<'l>, fd: jint) -> jbyteArray {
    let Ok(image) = disc_image(fd) else { return std::ptr::null_mut() };
    let mut header: Vec<u8> = image.id().into_bytes();
    header.resize(6, b' ');
    header.extend([0, image.revision()]);
    env.byte_array_from_slice(&header).map(|a| a.into_raw()).unwrap_or(std::ptr::null_mut())
}

/// `Native.extractDisc(fd, dest, progress)`: extracts the disc image open as
/// `fd` into the folder `dest` (`disc_image::Image::extract`: sys/, files/,
/// tmd.bin, ...), calling `progress.update(permille, 1000, text)` (a
/// `GameFiles.Progress`) as it goes. Returns null when done, or why not.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_extractDisc<'l>(mut env: JNIEnv<'l>, _: JClass<'l>, fd: jint, dest: JString<'l>, progress: JObject<'l>) -> jstring {
    let result = (|| -> Result<(), String> {
        let dest = java_string(&mut env, &dest)?;
        let image = disc_image(fd)?;
        let mut shown = u64::MAX;
        let mut tell = |done: u64, total: u64| {
            let permille = done * 1000 / total.max(1);
            if permille == shown {
                return;
            }
            shown = permille;
            if let Ok(text) = env.new_string(format!("{} of {} MB", done >> 20, total >> 20)) {
                let _ = env.call_method(&progress, "update", "(IILjava/lang/String;)V", &[JValue::Int(permille as i32), JValue::Int(1000), JValue::Object(&text)]);
                let _ = env.delete_local_ref(text);
            }
        };
        image.extract(std::path::Path::new(&dest), &mut tell, &std::sync::atomic::AtomicBool::new(false)).map(|_| ())
    })();
    match result {
        Ok(()) => std::ptr::null_mut(),
        Err(e) => env.new_string(e).map(JString::into_raw).unwrap_or(std::ptr::null_mut()),
    }
}
