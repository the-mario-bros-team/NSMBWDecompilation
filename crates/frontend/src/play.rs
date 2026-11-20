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
//! `settings.toml`.

use std::collections::VecDeque;
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
use crate::settings::{self, Settings};

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
}

/// Fields kept for interpolation; older ones are dropped and their evicted
/// textures released.
const QUEUE: usize = 3;
/// A field whose emulation takes longer than this is logged.
const SLOW_FIELD: Duration = Duration::from_millis(20);

struct App {
    opts: Options,
    settings: Settings,
    emu: Option<Emu>,
    // Declared before `window`: the Vulkan surface must be destroyed first.
    renderer: Option<Renderer>,
    window: Option<Window>,
    error: Option<String>,
    queue: VecDeque<GameFrame>,
    /// Draw pairing between two queued fields: (older index, newer index, pairs).
    pairs: Option<(u64, u64, Vec<Option<u32>>)>,
    games: u64,
    presented: u64,
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
    help: bool,
    slow_fields: u64,
    done: bool,
    inputs: input::Inputs,
    sent_input: Option<[Input; REMOTES]>,
    /// What the game thread was last told about interpolation records.
    sent_interpolate: Option<bool>,
    /// A `--shot` to take from the next in-between frame.
    pending_shot: Option<PathBuf>,
    remotes: [RemoteStatus; REMOTES],
}

/// Sleeps until `deadline`, then spins the last stretch: a sleep can
/// overshoot by a millisecond, a third of a 360 fps frame.
fn wait_until(deadline: Instant) {
    const SPIN: Duration = Duration::from_micros(1500);
    let now = Instant::now();
    if deadline > now + SPIN {
        std::thread::sleep(deadline - now - SPIN);
    }
    while Instant::now() < deadline {
        std::thread::yield_now();
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
        let mut attrs = Window::default_attributes().with_title("New Super Mario Bros. Wii - Rust port").with_inner_size(LogicalSize::new(1280.0, 720.0));
        if self.settings.fullscreen {
            attrs = attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let window = event_loop.create_window(attrs).map_err(|e| e.to_string())?;
        let display = window.display_handle().map_err(|e| e.to_string())?.as_raw();
        let handle = window.window_handle().map_err(|e| e.to_string())?.as_raw();
        // "Native" resolution: enough 480-line multiples to cover the window.
        let scale = match self.settings.scale {
            0 => {
                let monitor = window.current_monitor().map(|m| m.size().height);
                let h = if self.settings.fullscreen { monitor.unwrap_or(window.inner_size().height) } else { window.inner_size().height };
                h.div_ceil(480).clamp(1, 8)
            }
            s => s,
        };
        let _ = std::fs::create_dir_all("cache");
        let ropts = vk_render::Options { scale, vsync: self.settings.vsync, pipeline_cache: Some(PathBuf::from("cache/pipelines.bin")), efb_copies_to_ram: self.settings.efb_copies_to_ram };
        let mut renderer = Renderer::new(display, handle, ropts).map_err(|e| e.to_string())?;
        renderer.aspect = if self.settings.widescreen { 16.0 / 9.0 } else { 4.0 / 3.0 };
        println!(
            "vulkan device: {} (validation layer: {}); EFB {}x; {} fps; V-Sync {}",
            renderer.device_name,
            if renderer.validation { "on" } else { "not available" },
            scale,
            settings::fps_label(self.settings.fps),
            on(self.settings.vsync)
        );
        println!("keys: F1 help, F3 performance monitor, F4 battery saver, F5/F6 frame rate, F7 V-Sync, F9 aspect, F11 fullscreen");
        println!("controls (sideways Wii Remote): arrows D-pad, X/Space 2, Z/Shift 1, C/Ctrl shake, Q/E tilt, B the A button, Enter/Esc +, Tab/- minus, Home HOME; gamepads too");
        // The game reads its aspect ratio from SYSCONF once, at boot.
        if let Err(e) = hle_ios::nand::set_widescreen(Path::new("saves/nand"), self.settings.widescreen) {
            eprintln!("SYSCONF: {e}");
        }
        self.emu = Some(Emu::spawn(self.opts.game.clone(), EmuOptions { unpaced: self.opts.unpaced, ..self.opts.emu.clone() }));
        window.request_redraw();
        self.renderer = Some(renderer);
        self.window = Some(window);
        let now = Instant::now();
        (self.last_present, self.next_display, self.title_at) = (now, now, now);
        Ok(())
    }

    fn frame(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let Some(emu) = &self.emu else { return Ok(()) };
        let interpolate = !self.native();
        if self.sent_interpolate != Some(interpolate) {
            let _ = emu.to_emu.send(ToEmu::Interpolate(interpolate));
            self.sent_interpolate = Some(interpolate);
        }
        let controls = self.inputs.poll();
        if self.sent_input != Some(controls) {
            let _ = emu.to_emu.send(ToEmu::Input(controls));
            self.sent_input = Some(controls);
        }
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
        }
        self.update_title();
        if let Some(w) = &self.window {
            w.request_redraw();
        }
        Ok(())
    }

    /// Renders a new field once (uploading its textures and reading back its
    /// EFB copies), presenting it when `present`.
    fn ingest(&mut self, mut f: GameFrame, present: bool) -> Result<(), String> {
        self.games += 1;
        self.title_games += 1;
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
            println!("frame {}: {} draws, {} texture copies", f.index + 1, f.out.cmds.iter().filter(|c| matches!(c, gx::Cmd::Draw(_))).count(), f.out.cmds.iter().filter(|c| matches!(c, gx::Cmd::CopyTex(_))).count());
        }
        renderer.overlay = rects;
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
            renderer.render(&f.out, &f.out.vertices, true, f.xfb, w, h).map_err(|e| e.to_string())?;
        }
        self.render_ms = started.elapsed().as_secs_f32() * 1000.0;
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
                println!("Wii Remote {}: {}", i + 1, if new.connected { "connected" } else { "disconnected" });
            }
        }
        self.remotes = f.remotes;
        if f.emu > SLOW_FIELD {
            self.slow_fields += 1;
            if self.slow_fields <= 20 || self.slow_fields.is_power_of_two() {
                eprintln!("slow field {}: emulation {:.1} ms, render {:.1} ms ({} slow so far)", f.index, self.emu_ms, self.render_ms, self.slow_fields);
            }
        }
        if show {
            self.presented_now();
        }
        self.queue.push_back(f);
        while self.queue.len() > QUEUE {
            let old = self.queue.pop_front().unwrap();
            if let Some(r) = &mut self.renderer {
                r.evict(&old.out.evicted);
            }
            if let Some(emu) = &self.emu {
                let _ = emu.to_emu.send(ToEmu::Recycle(Box::new(old.out)));
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
        if a != b {
            let (fa, fb) = (&self.queue[a], &self.queue[b]);
            if self.pairs.as_ref().is_none_or(|p| p.0 != fa.index || p.1 != fb.index) {
                self.pairs = Some((fa.index, fb.index, gx::interp::pair(&fa.out, &fb.out)));
            }
        }
        let Some(size) = self.window.as_ref().map(|w| w.inner_size()) else { return Ok(()) };
        let rects = self.overlay_rects(size.width, size.height);
        let Some(renderer) = &mut self.renderer else { return Ok(()) };
        renderer.overlay = rects;
        if let Some(path) = self.pending_shot.take() {
            renderer.screenshot = Some(path);
        }
        let (fa, fb) = (&self.queue[a], &self.queue[b]);
        let pairs = &self.pairs;
        // The in-between vertices go straight into the frame's vertex buffer.
        let mut fill = |dst: &mut [gx::Vertex]| match pairs {
            Some(p) if a != b => gx::interp::interpolate_into(&fa.out, &fb.out, &p.2, t.clamp(0.0, 1.0), dst),
            _ => dst.copy_from_slice(&fb.out.vertices),
        };
        let started = Instant::now();
        renderer.render_with(&fb.out, fb.out.vertices.len(), &mut fill, false, fb.xfb, size.width, size.height).map_err(|e| e.to_string())?;
        self.render_ms = started.elapsed().as_secs_f32() * 1000.0;
        self.presented_now();
        Ok(())
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

    fn update_title(&mut self) {
        let now = Instant::now();
        let secs = (now - self.title_at).as_secs_f32();
        if secs < 1.0 {
            return;
        }
        self.shown_fps = self.title_presents as f32 / secs;
        self.game_fps = self.title_games as f32 / secs;
        (self.title_at, self.title_presents, self.title_games) = (now, 0, 0);
        if self.opts.log_fps {
            println!(
                "game frame {}: shown {:.1} fps, game {:.1} fps ({:.0}%), emulation {:.1} ms, render {:.2} ms",
                self.games,
                self.shown_fps,
                self.game_fps,
                self.game_fps / 59.94 * 100.0,
                self.emu_ms,
                self.render_ms
            );
        }
        if let (Some(w), Some(r)) = (&self.window, &self.renderer) {
            w.set_title(&format!(
                "New Super Mario Bros. Wii - Rust port - {:.1} fps (target {}), game {:.1} fps, {} draws",
                self.shown_fps,
                settings::fps_label(self.settings.fps).to_lowercase(),
                self.game_fps,
                r.stats.draws
            ));
        }
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
                format!("F9  ASPECT ({}; GAME AT NEXT START)", if self.settings.widescreen { "16:9" } else { "4:3" }),
                format!("F11 FULLSCREEN ({})", on(self.settings.fullscreen)),
            ]
            .into_iter()
            .map(|l| (overlay::WHITE, l))
            .collect();
            let w = lines.iter().map(|(_, l)| l.len() as i32).max().unwrap_or(0) * 6 * px + 4 * px;
            overlay::panel(&mut out, (width as i32 - w - margin).max(0), margin, px, &lines);
        }
        out
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
                println!("frame rate: {}", settings::fps_label(self.settings.fps));
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
                if let Some(r) = &mut self.renderer {
                    r.aspect = if self.settings.widescreen { 16.0 / 9.0 } else { 4.0 / 3.0 };
                }
                if let Err(e) = hle_ios::nand::set_widescreen(Path::new("saves/nand"), self.settings.widescreen) {
                    eprintln!("SYSCONF: {e}");
                }
                true
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
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
                    if !event.repeat && !self.inputs.key(code, down) && down {
                        self.key(code);
                    }
                }
            }
            WindowEvent::Focused(false) => self.inputs.release_keys(),
            WindowEvent::RedrawRequested => {
                if let Err(e) = self.frame(event_loop) {
                    self.error = Some(e);
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        if let Some(emu) = &mut self.emu {
            report(&emu.stop());
        }
        self.renderer = None;
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
        let list: Vec<String> = worst.iter().take(8).map(|(i, d)| format!("{i} ({:.1} ms)", d.as_secs_f64() * 1e3)).collect();
        println!("{} fields took longer than 16.7 ms to emulate; slowest: {}", t.slow_fields.len(), list.join(", "));
    }
    if t.audio_dropouts > 0 {
        println!("audio: {} dropouts (the game fell behind)", t.audio_dropouts);
    }
    if t.light_texture_calls > 0 {
        println!("light textures: {} of {} redraws skipped (nothing they read changed)", t.light_texture_skips, t.light_texture_calls);
    }
}

/// Runs until the window closes (or `max_frames` game frames); returns
/// (game frames, frames presented).
pub fn run(opts: Options) -> Result<(u64, u64), String> {
    crate::win::tune_process();
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    let now = Instant::now();
    let settings = opts.settings.clone();
    let mut app = App {
        opts,
        settings,
        emu: None,
        renderer: None,
        window: None,
        error: None,
        queue: VecDeque::new(),
        pairs: None,
        games: 0,
        presented: 0,
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
        help: false,
        slow_fields: 0,
        done: false,
        inputs: input::Inputs::new(),
        sent_input: None,
        sent_interpolate: None,
        pending_shot: None,
        remotes: [RemoteStatus::default(); REMOTES],
    };
    event_loop.run_app(&mut app).map_err(|e| e.to_string())?;
    match app.error {
        Some(e) => Err(e),
        None => Ok((app.games, app.presented)),
    }
}
