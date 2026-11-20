//! The Android app's native side, `libnsmbw.so` (docs/PORT_PLAN.md,
//! "Android"). The Java shell in `android/` (adapted from WitherBlitz's
//! NSMB-Wii-Recompilation app) owns the activity, the `SurfaceView`, the
//! on-screen Wii Remote, the phone's motion sensors, controllers and the
//! settings; it calls the `Native` class's functions below.
//!
//! The game thread is the desktop frontend's own (`emu.rs`, with `net.rs`
//! and `say.rs`, included from `crates/frontend/src`); `audio` and `win`
//! are this crate's versions of the frontend modules of those names. A
//! render thread of this crate takes each field from the game thread and
//! presents it at the game's own 60 fps, as the desktop window does at
//! 60 fps (`play.rs`). It keeps the renderer across surfaces: Android
//! destroys an app's surface whenever the app leaves the screen.
#![cfg(target_os = "android")]

#[allow(dead_code)]
#[path = "../../frontend/src/emu.rs"]
mod emu;
#[allow(dead_code)]
#[path = "../../frontend/src/net.rs"]
mod net;
#[allow(dead_code, unused_imports, unused_macros)]
#[path = "../../frontend/src/say.rs"]
mod say;

mod audio;
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
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jfloat, jint, jstring};
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
}

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
                self.paused = p;
                audio::set_paused(p);
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
        let opts = vk_render::Options { scale, vsync: self.cfg.vsync, pipeline_cache: Some(PathBuf::from("cache/pipelines.bin")), efb_copies_to_ram: false };
        let mut r = Renderer::new(display, handle, opts).map_err(|e| e.to_string())?;
        r.aspect = if self.cfg.widescreen { 16.0 / 9.0 } else { 4.0 / 3.0 };
        println!("vulkan device: {}; EFB {scale}x; surface {}x{}; V-Sync {}", r.device_name, w.width(), w.height(), self.cfg.vsync);
        self.renderer = Some(r);
        self.window = Some(w);
        // The game reads its aspect ratio and language from SYSCONF once, at boot.
        if let Err(e) = set_sysconf(Path::new("saves/nand"), self.cfg.widescreen, self.cfg.language) {
            eprintln!("SYSCONF: {e}");
        }
        let emu = Emu::spawn(self.cfg.game.clone(), EmuOptions { audio: true, ..EmuOptions::default() });
        let _ = emu.to_emu.send(ToEmu::Interpolate(false));
        self.emu = Some(emu);
        Ok(())
    }

    /// Draws a field and presents it (when there is a surface), and sends
    /// the game what only the GPU knows.
    fn ingest(&mut self, f: GameFrame) -> Result<(), String> {
        let (w, h) = self.window.as_ref().map_or((0, 0), |w| (w.width().max(0) as u32, w.height().max(0) as u32));
        let (Some(r), Some(emu)) = (&mut self.renderer, &self.emu) else { return Ok(()) };
        let started = Instant::now();
        r.render(&f.out, &f.out.vertices, true, f.xfb, w, h).map_err(|e| e.to_string())?;
        self.speed.1 += 1;
        self.speed.2 += f.emu;
        self.speed.3 += started.elapsed();
        let secs = self.speed.0.elapsed().as_secs_f32();
        if secs >= SPEED_EVERY.as_secs_f32() {
            let (n, ms) = (self.speed.1.max(1), |d: Duration| d.as_secs_f32() * 1000.0);
            let (textures, used, reserved) = r.memory();
            println!(
                "field {}: game {:.0}% ({:.1} fields/s), emulation {:.1} ms and render {:.1} ms a field; {textures} textures, GPU memory {} MB used of {} MB",
                f.index,
                self.speed.1 as f32 / secs / 59.94 * 100.0,
                self.speed.1 as f32 / secs,
                ms(self.speed.2) / n as f32,
                ms(self.speed.3) / n as f32,
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

    fn run(mut self, rx: Receiver<ToRender>) {
        loop {
            let busy = self.emu.is_some() && !self.paused && !self.failed;
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
            if !(self.emu.is_some() && !self.paused && !self.failed) {
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
            // While paused nothing takes fields: the game thread stops at
            // its next one, its frame queue full.
            match emu.frames.recv_timeout(FIELD) {
                Ok(Ok(f)) => {
                    if let Err(e) = self.ingest(f) {
                        self.fail(e);
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
        let (tx, rx) = mpsc::channel();
        let mut inputs = [Input::default(); REMOTES];
        inputs[0].connected = true;
        SHARED.set(Shared { tx: Mutex::new(tx), inputs: Mutex::new(inputs), error: Mutex::new(None), rumble: AtomicBool::new(false) }).map_err(|_| "already started".to_string())?;
        let cfg = Config { game: PathBuf::from(game), scale: scale.clamp(0, 8) as u32, widescreen: widescreen != 0, vsync: vsync != 0, language: u8::try_from(language).unwrap_or(1) };
        println!("starting: game {}, files {files}", cfg.game.display());
        let render = Render { cfg, renderer: None, window: None, emu: None, paused: false, failed: false, kept: VecDeque::new(), sent: None, speed: (Instant::now(), 0, Duration::ZERO, Duration::ZERO) };
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
/// game stops within a field or so and nothing plays.
#[no_mangle]
pub extern "system" fn Java_com_wither_nsmbwport_Native_setPaused<'l>(_: JNIEnv<'l>, _: JClass<'l>, paused: jboolean) {
    send(ToRender::Paused(paused != 0));
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
