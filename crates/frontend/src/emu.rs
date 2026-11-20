//! The game on its own thread (like Dolphin's dual-core mode and the
//! "graphics on a second CPU core" of WitherBlitz's port): it runs one
//! video field at a time, paced to 59.94 Hz, and hands each field's GX
//! output to the render thread. The render thread sends back what only the
//! GPU knows: EFB copies to write into RAM, and the EFB for CPU peeks.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use hle_ios::bt::{RemoteStatus, REMOTES};
use hle_ios::wiimote::Input;
use hle_os::System;
use vk_render::{EfbSnapshot, Writeback};

/// One NTSC field.
pub const FIELD: Duration = Duration::from_nanos(1_000_000_000 * 1001 / 60_000);
/// Guest cycles per field; a field that runs 4x longer without a retrace
/// (video interrupts off) is cut short so the window stays responsive.
const FIELD_CYCLES: u64 = 729_000_000 * 1001 / 60_000;

/// One field's picture.
pub struct GameFrame {
    pub out: gx::Output,
    /// The XFB the video interface scans out.
    pub xfb: u32,
    /// When this field is due on screen (paced), or when it was finished.
    pub time: Instant,
    /// Host time the guest took for it.
    pub emu: Duration,
    pub index: u64,
    /// The game read the EFB recently: send a snapshot back.
    pub wants_efb: bool,
    pub remotes: [RemoteStatus; REMOTES],
}

/// Scripted input (`--press`): `input` is held on remote `player` (0-based)
/// for `count` fields from field `start`, on top of the live controls.
/// Remotes 2-4 switch on at their first scripted press and stay on.
#[derive(Clone, Copy, Debug)]
pub struct Press {
    pub player: usize,
    pub start: u64,
    pub count: u64,
    pub input: Input,
}

pub enum ToEmu {
    Writebacks(Vec<Writeback>),
    Efb(EfbSnapshot),
    /// The players' controls.
    Input([Input; REMOTES]),
    /// A field's GX output the renderer no longer needs (its buffers are reused).
    Recycle(Box<gx::Output>),
    /// Whether fields should carry frame-interpolation records.
    Interpolate(bool),
}

/// Totals for the exit report.
#[derive(Default)]
pub struct Totals {
    pub steps: u64,
    pub cpu_time: Duration,
    /// Light-texture redraws: calls, and those skipped (`hle_os::light_texture`).
    pub light_texture_calls: u64,
    pub light_texture_skips: u64,
    /// Times sound output ran dry.
    pub audio_dropouts: u64,
    /// Instructions run by recompiled code, and functions found changed in
    /// memory (so interpreted instead).
    pub recomp_steps: u64,
    pub recomp_changed: u64,
    /// Of `cpu_time`, waiting for the GX thread (syncs at PE events).
    pub gx_wait: Duration,
}

pub struct Emu {
    pub frames: Receiver<Result<GameFrame, String>>,
    pub to_emu: Sender<ToEmu>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<Totals>>,
}

/// Runs the guest until the next VI retrace (at most four fields' time).
fn run_field(sys: &mut System) -> Result<(), String> {
    let limit = sys.cpu.cycles + 4 * FIELD_CYCLES;
    sys.run_field(limit).map_err(|s| format!("guest stopped at {:#010x}: {s:?}", sys.cpu.pc))
}

pub use hle_os::camera::player_position;

fn apply(sys: &mut System, live: &mut [Input; REMOTES], msg: ToEmu) {
    match msg {
        ToEmu::Writebacks(list) => {
            for wb in list {
                let bytes = gx::texture::encode(wb.tex_format, wb.width as usize, wb.height as usize, &wb.rgba);
                // The copy's texels arrive late: write them only where the
                // game has not written since the copy (it may have reused
                // the buffer).
                if sys.mem.ram(wb.addr, bytes.len()).is_none_or(|now| gx::texture::hash(now) != wb.ram_hash) {
                    continue;
                }
                if let Some(dst) = sys.mem.ram_mut(wb.addr, bytes.len()) {
                    dst.copy_from_slice(&bytes);
                    sys.hw.borrow_mut().gpu.note_writeback(wb.addr, &bytes);
                }
            }
        }
        ToEmu::Efb(s) => sys.hw.borrow_mut().efb = Some((s.color, s.depth)),
        ToEmu::Input(i) => *live = i,
        ToEmu::Recycle(out) => sys.hw.borrow_mut().gpu.recycle(*out),
        ToEmu::Interpolate(on) => sys.hw.borrow_mut().gpu.set_records(on),
    }
}

/// Live controls plus the presses scripted for field `index`.
fn controls(live: &[Input; REMOTES], script: &[Press], index: u64) -> [Input; REMOTES] {
    let mut out = *live;
    for p in script.iter().filter(|p| index >= p.start) {
        out[p.player].connected = true;
        if index < p.start + p.count {
            out[p.player].buttons |= p.input.buttons;
            out[p.player].shake |= p.input.shake;
        }
    }
    out
}

/// How the game thread runs.
#[derive(Clone, Default)]
pub struct EmuOptions {
    /// As fast as possible instead of 59.94 fields per second.
    pub unpaced: bool,
    /// Print every IOS request and the Bluetooth conversation.
    pub trace_ios: bool,
    /// Sample the guest PC and print the hottest functions at exit.
    pub profile: bool,
    pub script: Vec<Press>,
    /// Decode GX on the game thread instead of its own (for comparisons).
    pub gx_inline: bool,
    /// Print the players' positions every 10 fields (for scripting runs).
    pub watch_player: bool,
    /// Save the machine at the start of these fields (`--save-state`).
    pub save_states: Vec<(u64, PathBuf)>,
    /// Start from a saved state instead of power-on (`--load-state`).
    pub load_state: Option<PathBuf>,
    /// Print a hash of RAM every this many fields (0: never).
    pub hash_every: u64,
    /// Ignore what the renderer sends back (EFB copies to RAM, EFB
    /// snapshots for CPU peeks), so a paced run is as deterministic as an
    /// unpaced one; peeks then read zero. Net sessions need this.
    pub no_feedback: bool,
    /// Online multiplayer (`--host`, `--join`).
    pub net: crate::net::Mode,
    /// Draw the world centred on this player (0-based), within the game's
    /// camera (`--view-player`; a net session uses its own remote).
    pub view_player: Option<usize>,
    /// Show 1/n of the game camera's width and height around that player
    /// (`--view-zoom`, for testing the view on one player).
    pub view_zoom: f32,
    /// The game's camera becomes a box around every player
    /// (`--shared-camera`; always on online).
    pub shared_camera: bool,
    /// Write MEM1 and MEM2 (0x80000000 and 0x90000000, back to back) to a
    /// file at the start of these fields, for reverse engineering.
    pub dump_ram: Vec<(u64, PathBuf)>,
    /// Also write the sound to this WAV file.
    pub wav: Option<PathBuf>,
    /// Write every block start the interpreter decoded here at exit.
    pub code_profile: Option<PathBuf>,
    /// Interpret everything, even with recompiled code built in.
    pub interpreter: bool,
    /// Play sound on the default output device (paced runs only).
    pub audio: bool,
}

/// A state file: this magic, the field it was saved at, then
/// [`System::save_state`]'s bytes.
const STATE_FILE_MAGIC: &[u8; 8] = b"NSMBWfld";

fn write_state(path: &std::path::Path, index: u64, state: &[u8]) -> Result<(), String> {
    let mut b = Vec::with_capacity(16 + state.len());
    b.extend_from_slice(STATE_FILE_MAGIC);
    b.extend_from_slice(&index.to_le_bytes());
    b.extend_from_slice(state);
    std::fs::write(path, b).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_state(path: &std::path::Path) -> Result<(u64, Vec<u8>), String> {
    let b = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if b.len() < 16 || &b[..8] != STATE_FILE_MAGIC {
        return Err(format!("{}: not a save state", path.display()));
    }
    Ok((u64::from_le_bytes(b[8..16].try_into().unwrap()), b[16..].to_vec()))
}

/// Function starts from the ledger, for naming profile samples.
fn function_starts() -> Vec<(u32, String)> {
    let mut v: Vec<(u32, String)> = std::fs::read_to_string("ledger/functions.csv")
        .unwrap_or_default()
        .lines()
        .skip(1)
        .filter_map(|l| {
            let mut c = l.splitn(4, ',');
            let (_, a, n) = (c.next()?, c.next()?, c.next()?);
            Some((u32::from_str_radix(a, 16).ok()?, n.to_string()))
        })
        .collect();
    v.sort();
    v
}

fn print_profile(samples: &std::collections::HashMap<u32, u64>) {
    let starts = function_starts();
    let mut by_fn: std::collections::HashMap<u32, u64> = std::collections::HashMap::new();
    for (&pc, &n) in samples {
        let f = match starts.binary_search_by_key(&pc, |s| s.0) {
            Ok(i) => starts[i].0,
            Err(0) => pc,
            Err(i) => starts[i - 1].0,
        };
        *by_fn.entry(f).or_default() += n;
    }
    let total: u64 = by_fn.values().sum();
    let mut v: Vec<(u32, u64)> = by_fn.into_iter().collect();
    v.sort_by_key(|e| std::cmp::Reverse(e.1));
    println!("guest profile ({total} samples, one per slice):");
    for (f, n) in v.into_iter().take(30) {
        let name = starts.binary_search_by_key(&f, |s| s.0).map(|i| starts[i].1.as_str()).unwrap_or("?");
        println!("  {:5.1}%  {f:#010x} {name}", n as f64 * 100.0 / total.max(1) as f64);
    }
}

fn run(game: PathBuf, opts: EmuOptions, frames: SyncSender<Result<GameFrame, String>>, to_emu: Receiver<ToEmu>, stop: Arc<AtomicBool>) -> Totals {
    let EmuOptions { unpaced, trace_ios, profile, script, gx_inline, watch_player, save_states, mut load_state, hash_every, mut no_feedback, net, dump_ram, mut view_player, view_zoom, shared_camera, wav, audio, code_profile, interpreter } = opts;
    // Sound: to the device in real time, and to a WAV file if asked.
    let audio = if audio && !unpaced {
        let a = crate::audio::Audio::open().map_err(|e| eprintln!("audio: {e}; continuing without sound")).ok();
        if let Some(a) = &a {
            println!("audio: default output device at {} Hz", a.device_rate);
        }
        a
    } else {
        None
    };
    let mut wav = wav.and_then(|p| crate::audio::Wav::create(&p, 32_000).map_err(|e| eprintln!("{}: {e}", p.display())).ok());
    let mut totals = Totals::default();
    let light_textures = hle_os::light_texture::LightTextures::default().on();
    let mut nand = PathBuf::from("saves/nand");
    // A joiner gets the host's machine before booting, and boots on the
    // host's NAND files.
    let mut session = None;
    let mut joined: Option<(u64, Vec<u8>)> = None;
    match &net {
        crate::net::Mode::Off => {}
        crate::net::Mode::Host { port, wait_join } => match crate::net::Session::host(*port, *wait_join, &game, light_textures, nand.clone()) {
            Ok(s) => session = Some(s),
            Err(e) => {
                let _ = frames.send(Err(e));
                return totals;
            }
        },
        crate::net::Mode::Join { addr } => match crate::net::Session::join(addr, &game, light_textures) {
            Ok((s, field, state)) => {
                nand = crate::net::joiner_nand();
                session = Some(s);
                joined = Some((field, state));
                load_state = None;
            }
            Err(e) => {
                let _ = frames.send(Err(e));
                return totals;
            }
        },
    }
    // Each machine draws its own view, and only the game's own state may
    // differ by nothing: nothing from the renderer reaches the game.
    no_feedback |= session.is_some();
    if let Some(s) = &session {
        view_player = Some(s.slot);
    }
    let mut sys = match System::boot(&game, nand) {
        Ok(s) => s,
        Err(e) => {
            let _ = frames.send(Err(e));
            return totals;
        }
    };
    // Reverb, delay and chorus: the game's own effect callbacks, mixed back
    // by the DSP (NSMBW_AUXFX=off skips both).
    sys.aux_effects = std::env::var("NSMBW_AUXFX").map_or(true, |v| v != "off");
    sys.camera.enabled = shared_camera || session.is_some();
    sys.camera.view_player = view_player;
    sys.camera.view_zoom = view_zoom;
    let mut fields_without_camera = 0u32;
    sys.hw.borrow_mut().dsp.ax.aux_return = sys.aux_effects;
    if code_profile.is_some() {
        sys.blocks.starts = Some(Default::default());
    }
    if profile {
        sys.slices.borrow_mut().samples = Some(Default::default());
    }
    if !interpreter && !game_recomp::FUNCTIONS.is_empty() {
        match guest_cpu::recomp::Rt::new(&game_recomp::FUNCTIONS, &mut sys.mem, &hle_os::SPECIAL, game_recomp::SPECIAL) {
            Ok(rt) => {
                println!("recompiled code: {} functions", rt.functions());
                sys.set_recomp(rt);
            }
            Err(e) => eprintln!("recompiled code not used: {e}"),
        }
    }
    if !gx_inline {
        let ram = sys.mem.view();
        sys.hw.borrow_mut().gpu.start_thread(ram);
    }
    if trace_ios {
        let mut hw = sys.hw.borrow_mut();
        hw.ios.trace = true;
        hw.ios.bt.trace = true;
    }
    let mut index = 0;
    if let Some((field, state)) = joined {
        index = field;
        if let Err(e) = sys.load_state(state) {
            let _ = frames.send(Err(format!("net: the host's game: {e}")));
            return totals;
        }
    }
    if let Some(path) = &load_state {
        let loaded = read_state(path).and_then(|(i, bytes)| {
            index = i;
            sys.load_state(bytes).map_err(|e| format!("{}: {e}", path.display()))
        });
        if let Err(e) = loaded {
            let _ = frames.send(Err(e));
            return totals;
        }
        println!("loaded {} at field {index}", path.display());
    }
    let mut due = Instant::now() + FIELD;
    let mut peeks = sys.hw.borrow().efb_peeks;
    // Wii Remote 1 is on from the start, as when the game is launched with it.
    let mut live = [Input::default(); REMOTES];
    live[0].connected = true;
    while !stop.load(Ordering::Relaxed) {
        while let Ok(msg) = to_emu.try_recv() {
            if no_feedback && matches!(msg, ToEmu::Writebacks(_) | ToEmu::Efb(_)) {
                continue;
            }
            apply(&mut sys, &mut live, msg);
        }
        for (_, path) in save_states.iter().filter(|s| s.0 == index) {
            let state = sys.save_state();
            match write_state(path, index, &state) {
                Ok(()) => println!("saved {} at field {index} ({} MB)", path.display(), state.len() >> 20),
                Err(e) => eprintln!("{e}"),
            }
        }
        for (_, path) in dump_ram.iter().filter(|s| s.0 == index) {
            let mut b = sys.mem.ram(0x8000_0000, guest_mem::MEM1_SIZE).unwrap_or_default().to_vec();
            b.extend_from_slice(sys.mem.ram(0x9000_0000, guest_mem::MEM2_SIZE).unwrap_or_default());
            match std::fs::write(path, b) {
                Ok(()) => println!("dumped RAM to {} at field {index} (r13 {:#010x}, r2 {:#010x})", path.display(), sys.cpu.gpr[13], sys.cpu.gpr[2]),
                Err(e) => eprintln!("{}: {e}", path.display()),
            }
        }
        if hash_every != 0 && index % hash_every == 0 {
            println!("hash at field {index}: {:016x}", sys.mem.hash());
        }
        let mut inputs = controls(&live, &script, index);
        if let Some(session) = &mut session {
            // Online, this machine's player 1 controls (and `p1` presses)
            // are its own remote; the others come from the network.
            match session.inputs(&mut sys, index, inputs[0], &|| stop.load(Ordering::Relaxed)) {
                Ok(i) => inputs = i,
                Err(e) => {
                    let _ = frames.send(Err(e));
                    break;
                }
            }
        }
        {
            let mut hw = sys.hw.borrow_mut();
            for (i, input) in inputs.into_iter().enumerate() {
                hw.ios.bt.set_input(i, input);
            }
            // The view is set each time the game's camera runs; outside a
            // course (no camera for a few fields) the world is drawn as is.
            if view_player.is_some() {
                if std::mem::take(&mut sys.camera.ran) {
                    fields_without_camera = 0;
                } else {
                    fields_without_camera += 1;
                    if fields_without_camera == 4 {
                        hw.gpu.set_view(None);
                    }
                }
            }
        }
        let started = Instant::now();
        let waited = sys.hw.borrow().gpu.waited();
        let r = run_field(&mut sys);
        if let Err(e) = r {
            let _ = frames.send(Err(e));
            break;
        }
        let emu = started.elapsed();
        totals.gx_wait += sys.hw.borrow().gpu.waited() - waited;
        {
            let mut hw = sys.hw.borrow_mut();
            let rate = hw.dsp.audio_rate;
            let played = std::mem::take(&mut hw.dsp.audio);
            if let Some(a) = &audio {
                a.push(&played, rate);
            }
            if let Some(w) = &mut wav {
                if let Err(e) = w.write(&played) {
                    eprintln!("WAV: {e}");
                    wav = None;
                }
            }
        }
        if let Some(session) = &mut session {
            if let Err(e) = session.after_field(&sys, index) {
                let _ = frames.send(Err(e));
                break;
            }
        }
        if watch_player && index % 10 == 0 {
            for i in 0..REMOTES {
                if let Some([x, y]) = player_position(&sys.mem, i) {
                    let who = if i == 0 { String::new() } else { format!(" {}", i + 1) };
                    println!("player{who} at field {index}: x {x:.1} y {y:.1}");
                }
            }
        }
        totals.cpu_time += emu;
        totals.steps = sys.steps;
        totals.recomp_steps = sys.recomp_steps;
        totals.recomp_changed = sys.recomp.as_ref().map_or(0, |r| r.changed);
        (totals.light_texture_calls, totals.light_texture_skips) = (sys.light_textures.calls, sys.light_textures.skips);
        let (out, xfb, now_peeks, remotes) = {
            let mut hw = sys.hw.borrow_mut();
            let remotes = std::array::from_fn(|i| hw.ios.bt.remote_status(i));
            (hw.gpu.take_output(), hw.xfb_addr(), hw.efb_peeks, remotes)
        };
        let wants_efb = now_peeks != peeks && !no_feedback;
        peeks = now_peeks;
        let time = if unpaced { Instant::now() } else { due };
        if frames.send(Ok(GameFrame { out, xfb, time, emu, index, wants_efb, remotes })).is_err() {
            break;
        }
        index += 1;
        if !unpaced {
            let now = Instant::now();
            if due > now {
                std::thread::sleep(due - now);
            } else if now - due > FIELD / 2 {
                // Behind (a slow field): carry on from now rather than run
                // the next fields quickly to catch up, so the game never goes
                // faster than 100%. Smaller lateness is sleep overshoot,
                // which the schedule absorbs.
                due = now;
            }
            due += FIELD;
        }
    }
    if profile {
        print_profile(&sys.slices.borrow().samples.clone().unwrap_or_default());
    }
    if let Some(path) = code_profile {
        let mut starts: Vec<u32> = sys.blocks.starts.take().unwrap_or_default().into_iter().collect();
        starts.sort_unstable();
        let text: String = starts.iter().map(|a| format!("{a:08x}
")).collect();
        match std::fs::write(&path, text) {
            Ok(()) => println!("code profile: {} block starts in {}", starts.len(), path.display()),
            Err(e) => eprintln!("{}: {e}", path.display()),
        }
    }
    if let Some(w) = wav {
        if let Err(e) = w.finish() {
            eprintln!("WAV: {e}");
        }
    }
    if let Some(a) = &audio {
        totals.audio_dropouts = a.dropouts();
    }
    totals
}

impl Emu {
    pub fn spawn(game: PathBuf, opts: EmuOptions) -> Self {
        let (frames_tx, frames) = mpsc::sync_channel(2);
        let (to_emu, to_emu_rx) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let s = stop.clone();
        let thread = std::thread::Builder::new().name("game".into()).stack_size(64 << 20).spawn(move || run(game, opts, frames_tx, to_emu_rx, s)).expect("spawn the game thread");
        Self { frames, to_emu, stop, thread: Some(thread) }
    }

    /// Stops the game thread and returns its totals.
    pub fn stop(&mut self) -> Totals {
        self.stop.store(true, Ordering::Relaxed);
        // Unblock a pending send.
        while self.frames.try_recv().is_ok() {}
        self.thread.take().map(|t| t.join().unwrap_or_default()).unwrap_or_default()
    }
}
