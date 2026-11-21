//! The game on its own thread (like Dolphin's dual-core mode and the
//! "graphics on a second CPU core" of WitherBlitz's port): it runs one
//! video field at a time, paced to 59.94 Hz, and hands each field's GX
//! output to the render thread. The render thread sends back what only the
//! GPU knows: EFB copies to write into RAM, and the EFB for CPU peeks.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use hle_ios::bt::{RemoteStatus, REMOTES};
use hle_ios::wiimote::Input;
use hle_os::System;

use crate::say::{say, say_err};
use vk_render::{EfbSnapshot, Writeback};

/// One NTSC field.
pub const FIELD: Duration = Duration::from_nanos(1_000_000_000 * 1001 / 60_000);
/// How late (paced) a field may finish and still have the time made up by
/// the next fields skipping their sleep; later than this, the schedule
/// restarts from now (PORT_PLAN 11.6).
const CATCH_UP: Duration = Duration::from_nanos(2 * 1_000_000_000 * 1001 / 60_000);
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
    /// Stop running fields until `Pause(false)` (the Settings window is
    /// open). Ignored in a net game, which the other players keep running.
    Pause(bool),
    /// The Settings window that a game menu's Settings button opened has
    /// closed: that menu opens again (`hle_os::menu_settings`).
    SettingsClosed,
    /// The Settings window's Gameplay options (not in a net game, which
    /// keeps what its players agreed on at the start).
    Gameplay(crate::gameplay::Options),
    /// Give player 1 this power-up (`gameplay::POWER_UPS`) in a course or on
    /// the world map, as soon as the game is in one.
    PowerUp(u8),
    /// A sound of the Settings window's: the game's system sound `id`,
    /// played through the game after the next field (not while paused, and
    /// not in a net game).
    Sound(u32),
    /// A click on the game's picture (`crate::click`; not in a net game).
    Click(crate::click::Click),
    /// The Music, Sound Effects and Menu Sounds volumes (percent), applied
    /// per voice by the mixer (`hw::ax`). Not in a net game, whose players
    /// must mix the same: there only the master volume (host side) counts.
    Volumes([u32; 3]),
    /// A real Wii Remote for this player (0-3) in place of the emulated one,
    /// or (`None`) the emulated one again.
    RealRemote(usize, Option<Box<dyn hle_ios::bt::HidRemote>>),
}

/// The save cards' "Cheated", in the language the Settings buttons read.
fn cheated_word(settings: &str) -> &'static str {
    ["fr", "es"].into_iter().find(|l| crate::ui::text::tr(l, "Settings") == settings).map_or("Cheated", |l| crate::ui::text::tr(l, "Cheated"))
}

/// The Settings window's sounds, played as the game plays its menus' own:
/// `SndAudioMgr::startSystemSe(SndAudioMgr::sInstance, id, 1)`.
fn play_menu_sounds(sys: &mut System, sounds: &mut Vec<u32>) {
    const SND_AUDIO_MGR: u32 = 0x8042_A488;
    const START_SYSTEM_SE: u32 = 0x8019_5380;
    let Ok(manager) = sys.mem.read_u32(SND_AUDIO_MGR) else { return sounds.clear() };
    for id in sounds.drain(..) {
        if manager != 0 {
            let _ = sys.call_guest(START_SYSTEM_SE, &[manager, id, 1], &[]);
        }
    }
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
    /// Times the dispatcher entered recompiled code.
    pub recomp_entries: u64,
    /// Of `cpu_time`, waiting for the GX thread (syncs at PE events).
    pub gx_wait: Duration,
    /// Waiting for the GX thread's output at the end of each field (not in
    /// `cpu_time`).
    pub output_wait: Duration,
    /// The game thread's own CPU time over the whole run (`win::thread_cpu_time`).
    pub thread_cpu: Duration,
    /// Fields whose emulation took longer than [`FIELD`]: index, time, and
    /// of that, waiting for the GX thread.
    pub slow_fields: Vec<(u64, Duration, Duration)>,
}

pub struct Emu {
    pub frames: Receiver<Result<GameFrame, String>>,
    pub to_emu: Sender<ToEmu>,
    /// Set when a game menu's Settings button was picked and the menu has
    /// gone: the Settings window should open.
    pub settings_button: Arc<AtomicBool>,
    /// This machine's player after the last field ([`player_motion`],
    /// packed: the state in the high half, the move's bits in the low; 0
    /// for no player), for Space Smart Spin.
    player: Arc<AtomicU64>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<Totals>>,
}

/// Runs the guest until the next VI retrace (at most four fields' time).
fn run_field(sys: &mut System) -> Result<(), String> {
    let limit = sys.cpu.cycles + 4 * FIELD_CYCLES;
    sys.run_field(limit).map_err(|s| format!("guest stopped at {:#010x}: {s:?}", sys.cpu.pc))
}

/// Player `i`'s actor: `daPyMng_c`'s player IDs (0x80354E10, one word per
/// player), looked up as `fManager_c::searchBaseByID` does (lists at
/// 0x80377A70 by ID & 7, nodes linked at +4 with their `fBase_c` at +8, its
/// ID at +0).
fn player_actor(mem: &guest_mem::Mem, i: usize) -> Option<u32> {
    let r = |a: u32| mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()));
    let id = r(0x8035_4E10 + 4 * i as u32)?;
    if id == 0 {
        return None;
    }
    let mut node = r(0x8037_7A70 + 8 * (id & 7))?;
    for _ in 0..10_000 {
        if node == 0 {
            return None;
        }
        let base = r(node + 8)?;
        if r(base)? == id {
            return Some(base);
        }
        node = r(node + 4)?;
    }
    None
}

fn read32(mem: &guest_mem::Mem, a: u32) -> Option<u32> {
    mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()))
}

/// Player `i`'s position (`dBaseActor_c`, +0xAC).
pub fn player_position(mem: &guest_mem::Mem, i: usize) -> Option<[f32; 2]> {
    let base = player_actor(mem, i)?;
    Some([f32::from_bits(read32(mem, base + 0xAC)?), f32::from_bits(read32(mem, base + 0xB0)?)])
}

/// Player `i`'s state and last frame's vertical move, for Space Smart Spin
/// (after WitherBlitz's port, `game_menus.cpp` `PlayerInSpinnableAir`):
/// the state manager at +0x1464 holds its method at +0x18, whose current
/// `StateID` is at +0x1C; the move is `dBaseActor_c`'s +0xC8 (position
/// minus the one before).
pub fn player_motion(mem: &guest_mem::Mem, i: usize) -> Option<(u32, f32)> {
    let base = player_actor(mem, i)?;
    Some((read32(mem, base + 0x1464 + 0x18 + 0x1C)?, f32::from_bits(read32(mem, base + 0xC8)?)))
}

/// `_SDA_BASE_` (r13) of this build.
const SDA: u32 = 0x8042_F680;

/// The game's camera in a course: (left, top, width, height) in world units
/// (y up). `dBg_c::m_bg_p` (r13 - 0x58B0) keeps it at +0x8FE88 (left, top)
/// and +0x8FE78 (size); the world projection is `2 / width`, `2 / height`.
pub fn game_camera(mem: &guest_mem::Mem) -> Option<[f32; 4]> {
    let r = |a: u32| mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()));
    let bg = r(SDA - 0x58B0).filter(|&p| p >= 0x8000_0000)?;
    let f = |o: u32| r(bg + o).map(f32::from_bits);
    let cam = [f(0x8FE88)?, f(0x8FE8C)?, f(0x8FE78)?, f(0x8FE7C)?];
    (cam.iter().all(|v| v.is_finite()) && (16.0..20_000.0).contains(&cam[2]) && (16.0..20_000.0).contains(&cam[3])).then_some(cam)
}

/// The view centred on player `p`: a window the game camera's size (or
/// 1/`zoom` of it) around the player, kept inside the camera, which is all
/// the game draws.
fn player_view(mem: &guest_mem::Mem, p: usize, zoom: f32) -> Option<gx::xf::View> {
    let [left, top, w, h] = game_camera(mem)?;
    let [px, py] = player_position(mem, p)?;
    let size = [w / zoom.max(1.0), h / zoom.max(1.0)];
    let (cx, cy) = (left + w / 2.0, top - h / 2.0);
    let vx = px.clamp(left + size[0] / 2.0, left + w - size[0] / 2.0);
    let vy = py.clamp(top - h + size[1] / 2.0, top - size[1] / 2.0);
    Some(gx::xf::View::window([w, h], size, [vx - cx, vy - cy]))
}

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
        ToEmu::RealRemote(i, remote) => sys.hw.borrow_mut().ios.bt.set_real(i, remote),
        ToEmu::Pause(_) | ToEmu::SettingsClosed | ToEmu::Gameplay(_) | ToEmu::PowerUp(_) | ToEmu::Sound(_) | ToEmu::Volumes(_) | ToEmu::Click(_) => {}
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
    /// With `hash_every`: also print a running digest of every field's GX
    /// output so far (`gx::Output::digest`), to check that a change to GX
    /// decoding leaves what the renderer gets alone.
    pub hash_gx: bool,
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
    /// Stop by itself after this many fields and one more (`--headless`
    /// with `--frames`): a windowed `--frames` run gets through the same
    /// fields, the last while the window closes.
    pub stop_after: Option<u64>,
    /// Give the pause window and the world map's menu their Settings
    /// button, reading this (`hle_os::menu_layout`). Never in a net game
    /// or a scripted run, which keep the game's own menus.
    pub settings_label: Option<String>,
    /// The Settings buttons in a scripted run too (`--settings-buttons`).
    pub force_settings_buttons: bool,
    /// No window (`--headless`): a Settings window a game menu asks for
    /// counts as closed 30 fields later, so a scripted run carries on.
    pub auto_close_settings: bool,
    /// The save files' "Cheated" marks, shared with the window's thread.
    pub cheats: Arc<crate::gameplay::Cheats>,
    /// Scripted clicks on the game's menus (`--click`): the field and where,
    /// in layout units.
    pub clicks: Vec<(u64, crate::click::Click)>,
    /// The Gameplay options to start with (a window's from settings.toml;
    /// a headless run's all off, the game's own rules, unless
    /// `--gameplay-options`). A net host offers its Exit Course to joiners.
    pub gameplay: crate::gameplay::Options,
    /// `--gameplay-options`: player 1's character pick in a scripted or
    /// headless run too (otherwise, like the Settings buttons, only a
    /// player's).
    pub force_gameplay_options: bool,
    /// Player 1's character on each save file, shared with the window's thread.
    pub characters: Arc<crate::gameplay::Characters>,
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

#[allow(clippy::too_many_arguments)]
fn run(game: PathBuf, opts: EmuOptions, frames: SyncSender<Result<GameFrame, String>>, to_emu: Receiver<ToEmu>, stop: Arc<AtomicBool>, settings_button: Arc<AtomicBool>, player: Arc<AtomicU64>) -> Totals {
    let EmuOptions { unpaced, trace_ios, profile, script, gx_inline, watch_player, save_states, mut load_state, hash_every, hash_gx, mut no_feedback, net, dump_ram, mut view_player, view_zoom, wav, audio, code_profile, interpreter, stop_after, settings_label, force_settings_buttons, auto_close_settings, cheats, clicks, gameplay: mut gameplay_options, force_gameplay_options, characters } = opts;
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
    crate::win::raise_this_thread();
    let cpu_at_start = crate::win::thread_cpu_time();
    let light_textures = hle_os::light_texture::LightTextures::default().on();
    let mut nand = PathBuf::from("saves/nand");
    // A joiner gets the host's machine before booting, and boots on the
    // host's NAND files.
    let mut session = None;
    let mut joined: Option<(u64, Vec<u8>)> = None;
    match &net {
        crate::net::Mode::Off => {}
        crate::net::Mode::Host { port, wait_join } => match crate::net::Session::host(*port, *wait_join, &game, light_textures, nand.clone(), gameplay_options.exit_course) {
            Ok(s) => session = Some(s),
            Err(e) => {
                let _ = frames.send(Err(e));
                return totals;
            }
        },
        crate::net::Mode::Join { addr } => match crate::net::Session::join(addr, &game, light_textures, gameplay_options.exit_course) {
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
    let settings_label = settings_label.filter(|_| session.is_none() && (script.is_empty() || force_settings_buttons));
    // The save cards' text is "Cheated" in every language for now.
    let mut sys = match System::boot_with(&game, nand, settings_label.as_deref().map(|s| (s, cheated_word(s)))) {
        Ok(s) => s,
        Err(e) => {
            let _ = frames.send(Err(e));
            return totals;
        }
    };
    // Exit Course: a net game's as its players agreed.
    sys.menu.exit_course = session.as_ref().map_or(gameplay_options.exit_course, |s| s.exit_course);
    if settings_label.is_some() {
        let (pause, map, files) = sys.menu.buttons;
        let yes = |b| if b { "yes" } else { "no" };
        println!("Settings buttons: pause window {}, world map menu {}, save select {}; save cards' \"Cheated\" text {}", yes(pause), yes(map), yes(files), yes(sys.menu.cheated_label));
    }
    // Reverb, delay and chorus: the game's own effect callbacks, mixed back
    // by the DSP (NSMBW_AUXFX=off skips both).
    sys.aux_effects = std::env::var("NSMBW_AUXFX").map_or(true, |v| v != "off");
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
        sys.hw.borrow_mut().gpu.start_thread(ram, crate::win::raise_this_thread);
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
    // Player 1's character pick: a player's game (not a scripted one unless
    // asked, nor a net game), from the start: after a loaded state, whose
    // modules are placed already.
    if gameplay_options.p1_character && session.is_none() && (script.is_empty() || force_gameplay_options) {
        let on = sys.enable_p1_character();
        println!("player 1's character pick: {}", if on { "on" } else { "not applied (main.dol is not the one it is for)" });
    }
    let last_field = stop_after.map(|n| index + n);
    let mut gx_digest = 0u64;
    let mut due = Instant::now() + FIELD;
    let mut peeks = sys.hw.borrow().efb_peeks;
    // Wii Remote 1 is on from the start, as when the game is launched with it.
    let mut live = [Input::default(); REMOTES];
    live[0].connected = true;
    let mut paused = false;
    // The Settings window's sounds still to play.
    let mut sounds = Vec::new();
    // Clicks on the game's menus: the last one not taken yet, and the
    // buttons they press during the next field.
    let mut clicker = crate::click::Clicker::default();
    let mut click = None;
    let mut click_buttons = 0u16;
    let mut gameplay = crate::gameplay::Gameplay::default();
    // With `auto_close_settings`: the field the Settings window "closes" at.
    let mut close_settings_at = None;
    while !stop.load(Ordering::Relaxed) && last_field.is_none_or(|l| index <= l) {
        loop {
            while let Ok(msg) = to_emu.try_recv() {
                match msg {
                    ToEmu::Pause(p) => paused = p && session.is_none(),
                    ToEmu::SettingsClosed => sys.menu.reopen(),
                    ToEmu::Gameplay(o) => {
                        gameplay_options = o;
                        if session.is_none() {
                            sys.menu.exit_course = o.exit_course;
                        }
                    }
                    ToEmu::PowerUp(p) => gameplay.pending = Some(p),
                    // A sound waiting out a pause would play late.
                    ToEmu::Sound(id) => {
                        if !paused && session.is_none() {
                            sounds.push(id);
                        }
                    }
                    ToEmu::Click(c) => click = Some(c),
                    ToEmu::Volumes(v) => {
                        let gains = if session.is_none() { v.map(|p| (p.min(100) * 0x8000 / 100) as u16) } else { [0x8000; 3] };
                        sys.hw.borrow_mut().dsp.ax.gains = gains;
                    }
                    ToEmu::Writebacks(_) | ToEmu::Efb(_) if no_feedback => {}
                    msg => apply(&mut sys, &mut live, msg),
                }
            }
            if !paused || stop.load(Ordering::Relaxed) {
                break;
            }
            // Paused: no field runs, and the schedule starts again from the
            // end of the pause.
            std::thread::sleep(Duration::from_millis(5));
            due = Instant::now() + FIELD;
        }
        for (_, path) in save_states.iter().filter(|s| s.0 == index) {
            let state = sys.save_state();
            match write_state(path, index, &state) {
                Ok(()) => say!("saved {} at field {index} ({} MB)", path.display(), state.len() >> 20),
                Err(e) => say_err!("{e}"),
            }
        }
        for (_, path) in dump_ram.iter().filter(|s| s.0 == index) {
            let mut b = sys.mem.ram(0x8000_0000, guest_mem::MEM1_SIZE).unwrap_or_default().to_vec();
            b.extend_from_slice(sys.mem.ram(0x9000_0000, guest_mem::MEM2_SIZE).unwrap_or_default());
            match std::fs::write(path, b) {
                Ok(()) => say!("dumped RAM to {} at field {index} (r13 {:#010x}, r2 {:#010x})", path.display(), sys.cpu.gpr[13], sys.cpu.gpr[2]),
                Err(e) => say_err!("{}: {e}", path.display()),
            }
        }
        if hash_every != 0 && index % hash_every == 0 {
            say!("hash at field {index}: {:016x}", sys.mem.hash());
            if hash_gx {
                say!("gx hash at field {index}: {gx_digest:016x}");
            }
        }
        let mut inputs = controls(&live, &script, index);
        inputs[0].buttons |= click_buttons;
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
            let view = view_player.and_then(|p| player_view(&sys.mem, p, view_zoom));
            let mut hw = sys.hw.borrow_mut();
            for (i, mut input) in inputs.into_iter().enumerate() {
                // A real remote is on while Windows has it connected.
                input.connected |= hw.ios.bt.has_real(i);
                hw.ios.bt.set_input(i, input);
            }
            if view_player.is_some() {
                hw.gpu.set_view(view);
            }
        }
        let started = Instant::now();
        let waited = sys.hw.borrow().gpu.waited();
        let r = run_field(&mut sys);
        if let Err(e) = r {
            let _ = frames.send(Err(e));
            break;
        }
        // Net games keep the game's own rules.
        if session.is_none() {
            for line in gameplay.after_field(&mut sys, gameplay_options, &cheats, &characters) {
                say!("{line}");
            }
            play_menu_sounds(&mut sys, &mut sounds);
            if let Some(&(_, c)) = clicks.iter().find(|c| c.0 == index) {
                click = Some(c);
            }
            click_buttons = clicker.field(&mut sys, click.take());
            // A game menu's Settings button: the Settings window once the
            // pause window has closed (the map menu's is gone already).
            let closed = !sys.menu.waiting_for_pause() || sys.pause_window_closed();
            if closed && sys.menu.take_open() {
                say!("field {index}: a game menu's Settings button");
                if auto_close_settings {
                    close_settings_at = Some(index + 30);
                } else {
                    settings_button.store(true, Ordering::Relaxed);
                }
            }
            if close_settings_at == Some(index) {
                close_settings_at = None;
                sys.menu.reopen();
            }
        }
        // This machine's player for Space Smart Spin (reading changes nothing).
        let motion = player_motion(&sys.mem, view_player.unwrap_or(0));
        player.store(motion.map_or(0, |(state, vertical)| u64::from(state) << 32 | u64::from(vertical.to_bits())), Ordering::Relaxed);
        let emu = started.elapsed();
        let gx_wait = sys.hw.borrow().gpu.waited() - waited;
        if emu > FIELD {
            totals.slow_fields.push((index, emu, gx_wait));
        }
        totals.gx_wait += gx_wait;
        {
            let mut hw = sys.hw.borrow_mut();
            let rate = hw.dsp.audio_rate;
            let played = std::mem::take(&mut hw.dsp.audio);
            if let Some(a) = &audio {
                a.push(&played, rate);
            }
            if let Some(w) = &mut wav {
                if let Err(e) = w.write(&played) {
                    say_err!("WAV: {e}");
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
                    say!("player{who} at field {index}: x {x:.1} y {y:.1}");
                }
            }
        }
        totals.cpu_time += emu;
        totals.steps = sys.steps;
        totals.recomp_steps = sys.recomp_steps;
        totals.recomp_entries = sys.recomp_entries;
        totals.recomp_changed = sys.recomp.as_ref().map_or(0, |r| r.changed);
        (totals.light_texture_calls, totals.light_texture_skips) = (sys.light_textures.calls, sys.light_textures.skips);
        let (xfb, now_peeks, remotes) = {
            let mut hw = sys.hw.borrow_mut();
            let remotes = std::array::from_fn(|i| hw.ios.bt.remote_status(i));
            // The GX threads finish the field's output (transform, and the
            // interpolation records above 60 fps) while this thread sleeps
            // until the field's slot ends; it used to wait for them first.
            hw.gpu.request_output();
            (hw.xfb_addr(), hw.efb_peeks, remotes)
        };
        let wants_efb = now_peeks != peeks && !no_feedback;
        peeks = now_peeks;
        let time = if unpaced { Instant::now() } else { due };
        if !unpaced {
            let now = Instant::now();
            if due > now {
                std::thread::sleep(due - now);
            } else if now - due > CATCH_UP {
                // Far behind (a stall): carry on from now rather than run
                // fields back to back to catch up. Up to CATCH_UP late (a
                // scene load's slow field, or sleep overshoot), the next
                // fields skip their sleep until the schedule is met again.
                // No field starts before its slot, so the game never runs
                // ahead of real time.
                due = now;
            }
            due += FIELD;
        }
        let out = {
            let mut hw = sys.hw.borrow_mut();
            let waited = hw.gpu.waited();
            let out = hw.gpu.finish_output();
            totals.output_wait += hw.gpu.waited() - waited;
            if hash_gx {
                gx_digest = (gx_digest.rotate_left(13) ^ out.digest()).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            }
            out
        };
        if frames.send(Ok(GameFrame { out, xfb, time, emu, index, wants_efb, remotes })).is_err() {
            break;
        }
        index += 1;
    }
    // Lines from the loop first.
    crate::say::flush();
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
    totals.thread_cpu = crate::win::thread_cpu_time() - cpu_at_start;
    totals
}

/// Runs the game thread with no window or renderer (`--headless`) until it
/// stops by itself (`opts.stop_after`), dropping each field's output.
/// Nothing comes back from a renderer (EFB copies, EFB snapshots for CPU
/// peeks) and no host input arrives, so a run depends on its options only.
/// Returns the totals and the fields run.
pub fn run_headless(game: PathBuf, opts: EmuOptions) -> Result<(Totals, u64), String> {
    let mut emu = Emu::spawn(game, opts);
    // As a window at 60 fps (no frame interpolation) asks for.
    let _ = emu.to_emu.send(ToEmu::Interpolate(false));
    let mut fields = 0;
    let mut error = None;
    // Ends when the game thread stops and drops its sender.
    for f in emu.frames.iter() {
        match f {
            Ok(f) => {
                fields += 1;
                let _ = emu.to_emu.send(ToEmu::Recycle(Box::new(f.out)));
            }
            Err(e) => {
                error = Some(e);
                break;
            }
        }
    }
    let totals = emu.stop();
    error.map_or(Ok((totals, fields)), Err)
}

impl Emu {
    pub fn spawn(game: PathBuf, opts: EmuOptions) -> Self {
        // Room for a few fields: the game thread sends a field's output
        // when its slot ends, and must not wait there for a render.
        let (frames_tx, frames) = mpsc::sync_channel(4);
        let (to_emu, to_emu_rx) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let settings_button = Arc::new(AtomicBool::new(false));
        let player = Arc::new(AtomicU64::new(0));
        let (s, b, p) = (stop.clone(), settings_button.clone(), player.clone());
        let thread = std::thread::Builder::new().name("game".into()).stack_size(64 << 20).spawn(move || run(game, opts, frames_tx, to_emu_rx, s, b, p)).expect("spawn the game thread");
        Self { frames, to_emu, settings_button, player, stop, thread: Some(thread) }
    }

    /// This machine's player after the last field: their state and vertical
    /// move ([`player_motion`]); `None` outside a course.
    pub fn player(&self) -> Option<(u32, f32)> {
        let v = self.player.load(Ordering::Relaxed);
        (v != 0).then(|| ((v >> 32) as u32, f32::from_bits(v as u32)))
    }

    /// Stops the game thread and returns its totals.
    pub fn stop(&mut self) -> Totals {
        self.stop.store(true, Ordering::Relaxed);
        // Unblock a pending send.
        while self.frames.try_recv().is_ok() {}
        self.thread.take().map(|t| t.join().unwrap_or_default()).unwrap_or_default()
    }
}
