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

use game_version::E1;
use hle_ios::bt::{RemoteStatus, REMOTES};
use hle_ios::wiimote::{button, Input};
use hle_os::System;

use crate::say::{say, say_err};
use render::{EfbSnapshot, Writeback};

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
    /// Of `emu`, waiting for the GX thread (syncs at PE events); then
    /// waiting for the GX threads to finish the field's output.
    pub gx_wait: Duration,
    pub output_wait: Duration,
    pub index: u64,
    /// The game read the EFB recently: send a snapshot back.
    pub wants_efb: bool,
    pub remotes: [RemoteStatus; REMOTES],
    /// The player the main window shows (0-based): this machine's remote
    /// online, `--view-player`, or player 1.
    pub main_player: usize,
    /// Online: other players have their own machines, so no window opens
    /// for them here.
    pub online: bool,
    /// The video interface shows a picture (not black): the startup screen
    /// lifts a second after the first such field.
    pub visible: bool,
    /// The field's length at the game speed now: [`FIELD`] at 100%, zero
    /// at Unlimited (as fast as it runs).
    pub period: Duration,
    /// The players' controls the field ran with (the input display).
    pub inputs: [Input; REMOTES],
    /// The Practice tab's counts after the field.
    pub practice: crate::practice::Stats,
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
    /// `restart`: Restart Course at field `start` (instead of buttons).
    pub restart: bool,
}

/// Wii Remote buttons by name, for scripted input (`--press`): a b 1 2 up
/// down left right (the remote's own D-pad) plus minus home, and shake.
pub fn parse_buttons(names: &str) -> Result<Input, String> {
    let mut input = Input { connected: true, ..Input::default() };
    for n in names.split('+').filter(|s| !s.is_empty()) {
        input.buttons |= match n.to_ascii_lowercase().as_str() {
            "a" => button::A,
            "b" => button::B,
            "1" => button::ONE,
            "2" => button::TWO,
            "up" => button::UP,
            "down" => button::DOWN,
            "left" => button::LEFT,
            "right" => button::RIGHT,
            "plus" => button::PLUS,
            "minus" => button::MINUS,
            "home" => button::HOME,
            "shake" => {
                input.shake = true;
                0
            }
            other => return Err(format!("unknown Wii Remote button: {other}")),
        };
    }
    Ok(input)
}

/// One scripted press: `[p<n>:]<frame>:<buttons>[:<frames>]` (6 frames
/// when not given); `<frame>:restart` restarts the course then.
pub fn parse_press(v: &str) -> Result<Press, String> {
    let (player, rest) = match v.strip_prefix('p').and_then(|r| r.split_once(':')) {
        Some((n, rest)) => match n.parse::<usize>() {
            Ok(n @ 1..=4) => (n - 1, rest),
            _ => return Err(format!("--press {v}: player is p1..p4")),
        },
        None => (0, v),
    };
    let mut parts = rest.split(':');
    let start = parts.next().and_then(|s| s.parse().ok()).ok_or(format!("--press {v}: expected <frame>:<buttons>[:<frames>]"))?;
    let buttons = parts.next().unwrap_or("");
    if buttons.eq_ignore_ascii_case("restart") {
        return Ok(Press { player, start, count: 0, input: Input::default(), restart: true });
    }
    let input = parse_buttons(buttons)?;
    let count = match parts.next() {
        Some(n) => n.parse().map_err(|_| format!("--press {v}: bad frame count"))?,
        None => 6,
    };
    Ok(Press { player, start, count, input, restart: false })
}

/// A press file (`--press-file`; the Android app's `press`): one press per
/// line or several per line; `#` starts a comment.
pub fn parse_script(text: &str) -> Result<Vec<Press>, String> {
    text.lines().flat_map(|l| l.split('#').next().unwrap_or("").split_whitespace()).map(parse_press).collect()
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
    /// Replace textures as this says from now on (the Mods tab).
    TexturePack(crate::mods::PackSpec),
    /// Give player 1 this power-up (`gameplay::POWER_UPS`) in a course or on
    /// the world map, as soon as the game is in one.
    PowerUp(u8),
    /// A sound of the Settings window's: the game's system sound `id`,
    /// played through the game after the next field (not while paused, and
    /// not in a net game).
    Sound(u32),
    /// A click on the game's picture (`crate::click`; not in a net game).
    Click(crate::click::Click),
    /// The save select's Back button was clicked (`Clicker::back`).
    FileBack,
    /// The Music, Sound Effects and Menu Sounds volumes (percent), applied
    /// per voice by the mixer (`hw::ax`). Not in a net game, whose players
    /// must mix the same: there only the master volume (host side) counts.
    Volumes([u32; 3]),
    /// A real Wii Remote for this player (0-3) in place of the emulated one,
    /// or (`None`) the emulated one again.
    RealRemote(usize, Option<Box<dyn hle_ios::bt::HidRemote>>),
    /// Bluetooth passthrough: the console's sync button pressed (pair
    /// remotes), or (`true`) held 10 s (forget them all).
    SyncButton(bool),
    /// The picture's shape and the Graphics tab's options.
    Display(DisplayOptions),
    /// Write the machine to this file (a state file, as `--save-state`
    /// writes it) before the next field, if `Pause(true)` paused the game;
    /// then say how it went. The Android app's backup when it leaves the
    /// screen (the desktop never sends it).
    #[allow(dead_code)]
    SaveState(PathBuf, Sender<Result<(), String>>),
    /// The picture was tapped (the Android app's touch screen, off the
    /// on-screen remote's buttons): it skips the opening cutscene or a hint
    /// movie, as a click does (`gameplay::MovieSkip`).
    #[allow(dead_code)]
    Tap,
    /// The game's speed in percent (0: unlimited), the Practice tab's or
    /// Fast Forward's. Ignored in a net game, which runs at 100%.
    Speed(u32),
    /// Restart Course (`practice`). Ignored in a net game.
    RestartCourse,
}

/// A field's length at `speed` percent; zero for unlimited.
fn field_period(speed: u32) -> Duration {
    if speed == 0 {
        Duration::ZERO
    } else {
        FIELD * 100 / speed
    }
}

/// The save cards' "Cheated", in the language the Settings buttons read.
fn cheated_word(settings: &str) -> &'static str {
    ["fr", "es", "de", "it"].into_iter().find(|l| crate::ui::text::tr(l, "Settings") == settings).map_or("Cheated", |l| crate::ui::text::tr(l, "Cheated"))
}

/// The Settings window's sounds, played as the game plays its menus' own:
/// `SndAudioMgr::startSystemSe(SndAudioMgr::sInstance, id, 1)`.
fn play_menu_sounds(sys: &mut System, sounds: &mut Vec<u32>) {
    const SND_AUDIO_MGR: E1 = E1(0x8042_A488);
    const START_SYSTEM_SE: E1 = E1(0x8019_5380);
    let Ok(manager) = sys.mem.read_u32(SND_AUDIO_MGR.va()) else { return sounds.clear() };
    for id in sounds.drain(..) {
        if manager != 0 {
            let _ = sys.call_guest(START_SYSTEM_SE.va(), &[manager, id, 1], &[]);
        }
    }
}

/// What the game thread does for the picture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayOptions {
    /// The picture's aspect: the game's 16:9 ([`hle_os::display::ASPECT_16_9`]),
    /// or wider or narrower (Ultrawide, 16:10), which widens the game's
    /// view. Net games always keep 16:9.
    pub aspect: f32,
    /// Animated tiles (`gx::Cmd::Patch`).
    pub spinning_coins: bool,
    pub depth_blur: bool,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self { aspect: hle_os::display::ASPECT_16_9, spinning_coins: true, depth_blur: true }
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
    /// The texture pack's counts, if one was in use at the end.
    pub texture_pack: Option<std::sync::Arc<gx::replace::Stats>>,
}

/// What the game thread shares with the window's for the game's menus: the
/// Settings and Multiplayer buttons picked, and the save select's buttons.
type MenuShare = (Arc<AtomicBool>, Arc<AtomicBool>, Arc<std::sync::Mutex<Option<crate::click::SaveSelect>>>);

pub struct Emu {
    pub frames: Receiver<Result<GameFrame, String>>,
    pub to_emu: Sender<ToEmu>,
    /// Set when a game menu's Settings button was picked and the menu has
    /// gone: the Settings window should open.
    pub settings_button: Arc<AtomicBool>,
    /// Set when the save select's Multiplayer button was picked: the
    /// Multiplayer screen should open.
    pub multiplayer_button: Arc<AtomicBool>,
    /// The save select's buttons the port draws, after the last field
    /// (`None` when it is not up, and in net games).
    pub save_select: Arc<std::sync::Mutex<Option<crate::click::SaveSelect>>>,
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

pub use hle_os::camera::player_position;
use hle_os::camera::player_actor;

/// `_SDA_BASE_` (r13) of USA revision 1.
const SDA: u32 = 0x8042_F680;

fn read32(mem: &guest_mem::Mem, a: u32) -> Option<u32> {
    mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()))
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

/// `--course`: a course to play instead of the next one the game starts.
/// `world` and `level` are 0-based (`Stage/03-24.arc` is world 2, level 23).
#[derive(Clone, Copy, Debug)]
pub struct Course {
    pub world: u8,
    pub level: u8,
    pub area: Option<u8>,
    pub entrance: Option<u8>,
}

/// `dScene_c`'s requested scene (r13 - 0x6F50, a u16): the id waits there
/// until the new scene exists; 7 is a course. Its argument (entrance, and
/// area << 8) is at r13 - 0x5638.
const SCENE_REQUEST: E1 = E1(SDA - 0x6F50);
const SCENE_ARGUMENT: E1 = E1(SDA - 0x5638);
/// `dInfo_c::m_instance`, `dInfo_c::startGame`, and where `startGame` keeps
/// its 16-byte record (entrance at +5, area at +6, then world and level, and
/// the world and level shown, at +0xC..+0xF).
const INFO: E1 = E1(0x8042_9F7C);
const START_GAME: E1 = E1(0x800B_B780);
const START_RECORD: E1 = E1(0x8031_5B90);

/// Once the game has asked for a course (the world map's own entry: the
/// players are set up for it), starts `c` instead: `dInfo_c::startGame`
/// again with the game's record, changed. The scene request is already
/// waiting, so `startGame` leaves it, and its argument is set here.
fn switch_course(sys: &mut System, c: Course) -> Result<(), String> {
    let mut record = sys.mem.ram(START_RECORD.va(), 16).ok_or("course record")?.to_vec();
    let was = (record[12], record[13]);
    record[12..16].copy_from_slice(&[c.world, c.level, c.world, c.level]);
    if let Some(e) = c.entrance {
        record[5] = e;
    }
    if let Some(a) = c.area {
        record[6] = a;
    }
    let info = sys.mem.read_u32(INFO.va()).map_err(|_| "dInfo_c")?;
    // As the world map does just before (d_basesNP 0x8079F790).
    let _ = sys.mem.write_u32(info + 0x50, u32::from(c.world));
    let _ = sys.mem.write_u32(info + 0x54, u32::from(c.level));
    sys.call_guest(START_GAME.va(), &[info, 0x8000_1800], &record)?;
    let _ = sys.mem.write_u32(SCENE_ARGUMENT.va(), u32::from(record[6] & 0xF) << 8 | u32::from(record[5]));
    println!("course: {:02}-{:02} (area {}, entrance {}) instead of {:02}-{:02}", c.world + 1, c.level + 1, record[6], record[5], was.0 + 1, was.1 + 1);
    Ok(())
}

fn apply(sys: &mut System, live: &mut [Input; REMOTES], msg: ToEmu) {
    match msg {
        ToEmu::Writebacks(list) => {
            for wb in list {
                let bytes = gx::texture::encode(wb.tex_format, wb.width as usize, wb.height as usize, &wb.rgba);
                // The copy's texels arrive late: write them only where the
                // game has not written since the copy (it may have reused
                // the buffer).
                let (w, h, stride) = (wb.width as usize, wb.height as usize, wb.stride as usize);
                if gx::texture::copy_bytes(&sys.mem, wb.addr, wb.tex_format, w, h, stride).is_none_or(|now| gx::texture::hash(&now) != wb.ram_hash) {
                    continue;
                }
                // Row by row: a copy into part of a wider texture has its
                // rows that texture's row apart.
                let mut from = 0;
                for (at, len) in gx::texture::copy_rows(wb.tex_format, w, h, stride) {
                    if let Some(dst) = sys.mem.ram_mut(wb.addr + at as u32, len) {
                        dst.copy_from_slice(&bytes[from..from + len]);
                    }
                    from += len;
                }
                sys.hw.borrow_mut().gpu.note_writeback(wb.addr, &bytes);
            }
        }
        ToEmu::Efb(s) => sys.hw.borrow_mut().efb = Some((s.color, s.depth)),
        ToEmu::Input(i) => *live = i,
        ToEmu::Recycle(out) => sys.hw.borrow_mut().gpu.recycle(*out),
        ToEmu::Interpolate(on) => sys.hw.borrow_mut().gpu.set_records(on),
        ToEmu::RealRemote(i, remote) => sys.hw.borrow_mut().ios.bt.set_real(i, remote),
        ToEmu::SyncButton(held) => {
            let mut hw = sys.hw.borrow_mut();
            if hw.ios.bt.passthrough.is_some() {
                hw.ios.sync_button(held);
                say!("Bluetooth passthrough: {}", if held { "forgetting every paired Wii Remote" } else { "pairing: press the red SYNC button on each Wii Remote (under its battery cover)" });
            }
        }
        ToEmu::Pause(_) | ToEmu::SettingsClosed | ToEmu::Gameplay(_) | ToEmu::PowerUp(_) | ToEmu::Sound(_) | ToEmu::Volumes(_) | ToEmu::Click(_) | ToEmu::FileBack | ToEmu::Display(_) | ToEmu::TexturePack(_) | ToEmu::SaveState(..) | ToEmu::Tap | ToEmu::Speed(_) | ToEmu::RestartCourse => {}
    }
}

/// Opens the texture pack `spec` says for the GX thread; returns its counts.
fn set_texture_pack(sys: &mut System, spec: &crate::mods::PackSpec) -> Option<Arc<gx::replace::Stats>> {
    let pack = spec.open();
    let stats = pack.as_ref().map(|p| p.stats.clone());
    sys.hw.borrow_mut().gpu.set_texture_pack(pack);
    stats
}

/// Live controls plus the presses scripted for field `index`.
fn controls(live: &[Input; REMOTES], script: &[Press], index: u64) -> [Input; REMOTES] {
    let mut out = *live;
    for p in script.iter().filter(|p| index >= p.start && !p.restart) {
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
    /// The player the main window shows (0-based; `--view-player`; a net
    /// session uses its own remote). Other local players get windows of
    /// their own.
    pub view_player: Option<usize>,
    /// Show 1/n of the game camera's width and height around that player
    /// (`--view-zoom`, for testing the view on one player).
    pub view_zoom: f32,
    /// Keep the game's own camera in multiplayer instead of a box around
    /// every player (`--game-camera`, for comparisons; not online).
    pub game_camera: bool,
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
    /// The picture's aspect until the window says otherwise (`--wide`, for
    /// headless runs).
    pub wide: Option<f32>,
    /// Textures replaced from a texture pack (the Mods tab).
    pub texture_pack: crate::mods::PackSpec,
    /// The file mods turned on (`crate::mods`; not in a net game).
    pub mods: Vec<String>,
    /// Start this course when the game next starts one (`--course`).
    pub course: Option<Course>,
    /// Bluetooth passthrough (`bt_passthrough`): a real adapter runs the
    /// game's Bluetooth (not in a net game or a scripted run).
    pub bt_passthrough: bool,
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
fn run(game: PathBuf, opts: EmuOptions, frames: SyncSender<Result<GameFrame, String>>, to_emu: Receiver<ToEmu>, stop: Arc<AtomicBool>, menus: MenuShare, player: Arc<AtomicU64>) -> Totals {
    let (settings_button, multiplayer_button, save_select) = menus;
    let EmuOptions { unpaced, trace_ios, profile, script, gx_inline, watch_player, save_states, mut load_state, hash_every, hash_gx, mut no_feedback, net, dump_ram, mut view_player, view_zoom, mut game_camera, wav, audio, code_profile, interpreter, stop_after, settings_label, force_settings_buttons, auto_close_settings, cheats, clicks, gameplay: mut gameplay_options, force_gameplay_options, characters, wide, texture_pack, mods, mut course, bt_passthrough } = opts;
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
    game_camera &= session.is_none();
    if let Some(s) = &session {
        view_player = Some(s.slot);
    }
    let settings_label = settings_label.filter(|_| session.is_none() && (script.is_empty() || force_settings_buttons));
    // Every player in a net game would need the same mods.
    let mods = if session.is_none() { crate::mods::mod_files(&mods) } else { Vec::new() };
    let mut sys = match System::boot_with(&game, nand, settings_label.as_deref().map(|s| (s, cheated_word(s))), mods) {
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
    sys.camera.enabled = !game_camera;
    sys.camera.view_zoom = view_zoom;
    let mut fields_without_camera = 0u32;
    // `NSMBW_INPUT_LOG=1`: each change of a remote's buttons, shake or tilt
    // (in tenths) as it goes to the emulated remote.
    let mut input_log = std::env::var("NSMBW_INPUT_LOG").is_ok_and(|v| v == "1").then(|| [(0u16, false, 0i32); REMOTES]);
    sys.hw.borrow_mut().dsp.ax.aux_return = sys.aux_effects;
    if code_profile.is_some() {
        sys.blocks.starts = Some(Default::default());
    }
    if profile {
        sys.slices.borrow_mut().samples = Some(Default::default());
    }
    let (functions, generated_special) = game_recomp::code(sys.version);
    if !interpreter && !functions.is_empty() {
        match guest_cpu::recomp::Rt::new(functions, &mut sys.mem, &hle_os::special(), generated_special) {
            Ok(rt) => {
                println!("recompiled code: {} functions ({})", rt.functions(), sys.version.describe());
                sys.set_recomp(rt);
            }
            Err(e) => eprintln!("recompiled code not used: {e}"),
        }
    }
    if !gx_inline {
        let ram = sys.mem.view();
        sys.hw.borrow_mut().gpu.start_thread(ram, crate::win::raise_this_thread);
    }
    // Render-only, so a net game allows it.
    let mut pack_stats = set_texture_pack(&mut sys, &texture_pack);
    if trace_ios {
        let mut hw = sys.hw.borrow_mut();
        hw.ios.trace = true;
        hw.ios.bt.trace = true;
    }
    if bt_passthrough {
        let off = if session.is_some() { Some("net games use the usual controls") } else if !script.is_empty() { Some("a scripted run") } else { None };
        start_passthrough(&mut sys, off);
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
    // Skipping the opening cutscene and the hint movies with any button, a
    // click or a tap: a player's game (not a scripted one unless asked, nor
    // a net game).
    let skip_movies = session.is_none() && (script.is_empty() || force_gameplay_options);
    let mut movie_skip = crate::gameplay::MovieSkip::default();
    let mut tapped = false;
    let mut display = DisplayOptions { aspect: wide.unwrap_or(hle_os::display::ASPECT_16_9), ..Default::default() };
    // What the GX decoder was last told: the projections' widening and the
    // animated tiles.
    let mut sent_display = (0.0f32, true);
    // With `auto_close_settings`: the field the Settings window "closes" at.
    let mut close_settings_at = None;
    // The Practice tab: the game's speed (always 100 in a net game), a
    // Restart Course asked for, and the counts.
    let mut speed = 100u32;
    let mut restart = false;
    let mut practice = crate::practice::Practice::default();
    // Sound away from 100%: stretched to last real time (`stretch`), or
    // silent at Unlimited (and faded in again after).
    let mut stretch: Option<crate::stretch::Stretch> = None;
    let mut silent = false;
    let mut sound = Vec::new();
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
                    ToEmu::Click(c) => {
                        click = Some(c);
                        tapped = true;
                    }
                    ToEmu::Tap => tapped = true,
                    ToEmu::FileBack => {
                        if session.is_none() {
                            clicker.back(&sys);
                        }
                    }
                    ToEmu::Volumes(v) => {
                        let gains = if session.is_none() { v.map(|p| (p.min(100) * 0x8000 / 100) as u16) } else { [0x8000; 3] };
                        sys.hw.borrow_mut().dsp.ax.gains = gains;
                    }
                    ToEmu::Display(d) => display = d,
                    ToEmu::TexturePack(spec) => pack_stats = set_texture_pack(&mut sys, &spec),
                    ToEmu::SaveState(path, done) => {
                        // Written beside it first, so a process ended
                        // meanwhile leaves no half a state behind.
                        let result = if paused {
                            let state = sys.save_state();
                            let part = path.with_extension("part");
                            write_state(&part, index, &state).and_then(|()| std::fs::rename(&part, &path).map_err(|e| format!("{}: {e}", path.display())))
                        } else {
                            Err("the game is not paused".into())
                        };
                        let _ = done.send(result);
                    }
                    ToEmu::Speed(s) => {
                        if session.is_none() && s != speed {
                            speed = s;
                            due = Instant::now() + field_period(speed);
                        }
                    }
                    ToEmu::RestartCourse => restart |= session.is_none(),
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
            due = Instant::now() + field_period(speed);
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
        if course.is_some() && sys.mem.read_u16(SCENE_REQUEST.va()) == Ok(7) {
            if let Err(e) = switch_course(&mut sys, course.take().unwrap()) {
                let _ = frames.send(Err(format!("--course: {e}")));
                break;
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
        restart |= session.is_none() && script.iter().any(|p| p.restart && p.start == index);
        if std::mem::take(&mut restart) {
            say!("field {index}: {}", practice.restart(&mut sys));
        }
        if skip_movies {
            let mut buttons = inputs.map(|i| i.buttons);
            if let Some(line) = movie_skip.before_field(&mut sys, &mut buttons, std::mem::take(&mut tapped)) {
                say!("field {index}: {line}");
                // The click that skipped presses nothing afterwards.
                click = None;
            }
            for (input, b) in inputs.iter_mut().zip(buttons) {
                input.buttons = b;
            }
        }
        if let Some(session) = &mut session {
            // Online, this machine's player 1 controls (and `p1` presses)
            // are its own remote; the others come from the network.
            match session.inputs(&mut sys, index, inputs[0], &|| stop.load(Ordering::Relaxed)) {
                Ok(i) => {
                    inputs = i;
                    for slot in session.joining_at(index) {
                        sys.joins.request(slot);
                    }
                    // Time sync: this machine's fields a little later.
                    due += session.take_shift();
                }
                Err(e) => {
                    let _ = frames.send(Err(e));
                    break;
                }
            }
        }
        {
            let mut hw = sys.hw.borrow_mut();
            if let Some(p) = &mut hw.ios.bt.passthrough {
                for line in p.take_log() {
                    say!("[bluetooth] {line}");
                }
            }
            for (i, mut input) in inputs.into_iter().enumerate() {
                if let Some(log) = &mut input_log {
                    let now = (input.buttons, input.shake, (input.tilt * 10.0).round() as i32);
                    if now != log[i] {
                        log[i] = now;
                        say!("input: field {index} player {}: buttons {:#06x}{} tilt {:+.2}", i + 1, input.buttons, if input.shake { " shake" } else { "" }, input.tilt);
                    }
                }
                // A real remote is on while Windows has it connected.
                input.connected |= hw.ios.bt.has_real(i);
                hw.ios.bt.set_input(i, input);
            }
            // The world camera and the players' windows are set each time the
            // game's camera runs; outside a course (no camera for a few
            // fields) there are no world draws, and every window shows the
            // whole screen.
            if std::mem::take(&mut sys.camera.ran) {
                fields_without_camera = 0;
            } else {
                fields_without_camera += 1;
                if fields_without_camera == 4 {
                    hw.gpu.set_world(None);
                }
            }
        }
        {
            // The picture: net games keep the game's 16:9, which decides
            // what the game wakes and so must be the same on every machine.
            let aspect = if session.is_some() { hle_os::display::ASPECT_16_9 } else { display.aspect };
            let wide = if (aspect - hle_os::display::ASPECT_16_9).abs() < 1e-4 { 0.0 } else { aspect };
            sys.set_screen_aspect(aspect);
            sys.display.wide = wide;
            sys.display.depth_blur = display.depth_blur;
            let mut hw = sys.hw.borrow_mut();
            if sent_display.0 != wide {
                hw.gpu.set_wide(wide);
            }
            if sent_display.1 != display.spinning_coins {
                hw.gpu.set_patches(display.spinning_coins);
            }
            sent_display = (wide, display.spinning_coins);
        }
        let started = Instant::now();
        let waited = sys.hw.borrow().gpu.waited();
        let r = run_field(&mut sys);
        if let Err(e) = r {
            let _ = frames.send(Err(e));
            break;
        }
        // Bulk writes the GX thread watches (`gx::texture::CHUNKED`) count
        // again now they are done.
        sys.mem.settle();
        // Before the Gameplay options fix up lives and the clock.
        for line in practice.count(&mut sys) {
            say!("field {index}: {line}");
        }
        // Net games keep the game's own rules.
        if session.is_none() {
            for line in gameplay.after_field(&mut sys, gameplay_options, &cheats, &characters) {
                say!("{line}");
            }
            play_menu_sounds(&mut sys, &mut sounds);
            if let Some(&(_, c)) = clicks.iter().find(|c| c.0 == index) {
                click = Some(c);
                tapped = true;
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
            if closed && sys.menu.take_open_multiplayer() {
                say!("field {index}: the save select's Multiplayer button");
                if auto_close_settings {
                    close_settings_at = Some(index + 30);
                } else {
                    multiplayer_button.store(true, Ordering::Relaxed);
                }
            }
            if close_settings_at == Some(index) {
                close_settings_at = None;
                sys.menu.reopen();
            }
            let view = crate::click::save_select(&mut sys);
            *save_select.lock().unwrap_or_else(|e| e.into_inner()) = view;
        }
        practice.settle(&sys);
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
            // What is heard: at 100% the game's sound as it is; at other
            // speeds stretched to last real time (`stretch`); at Unlimited
            // nothing, the device kept playing silence, and faded in again
            // after. The WAV file gets the same.
            let heard = if audio.is_none() && wav.is_none() {
                None
            } else if speed == 0 {
                if !std::mem::replace(&mut silent, true) {
                    if let Some(a) = &audio {
                        a.fade_tail(10);
                    }
                    stretch = None;
                }
                if let Some(a) = &audio {
                    a.fill_silence(rate);
                }
                None
            } else {
                sound.clear();
                if speed != 100 || stretch.is_some() {
                    // Steered to the playing level, so the queue neither runs
                    // dry nor grows when the PC can't keep up.
                    let level = audio.as_ref().map_or(1.0, |a| f64::from(a.queued_ms()) / crate::audio::TARGET_MS as f64);
                    let ratio = f64::from(speed) / 100.0 * level.clamp(0.5, 1.5);
                    stretch.get_or_insert_with(Default::default).process(&played, rate, ratio, &mut sound);
                } else {
                    sound.extend_from_slice(&played);
                }
                if std::mem::take(&mut silent) {
                    crate::stretch::fade_in(&mut sound, rate as usize / 100);
                }
                Some(&sound)
            };
            if let (Some(a), Some(heard)) = (&audio, heard) {
                a.push(heard, rate);
            }
            if let (Some(w), Some(heard)) = (&mut wav, heard) {
                if let Err(e) = w.write(heard) {
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
        let (xfb, now_peeks, remotes, visible) = {
            let mut hw = sys.hw.borrow_mut();
            let remotes = std::array::from_fn(|i| hw.ios.bt.remote_status(i));
            // The GX threads finish the field's output (transform, and the
            // interpolation records above 60 fps) while this thread sleeps
            // until the field's slot ends; it used to wait for them first.
            hw.gpu.request_output();
            (hw.xfb_addr(), hw.efb_peeks, remotes, hw.display_on())
        };
        let wants_efb = now_peeks != peeks && !no_feedback;
        peeks = now_peeks;
        // Paced at the game speed (Unlimited: as fast as it runs).
        let unlimited = unpaced || speed == 0;
        let period = field_period(speed);
        let time = if unlimited { Instant::now() } else { due };
        if !unlimited {
            let now = Instant::now();
            if due > now {
                std::thread::sleep(due - now);
            } else if now - due > CATCH_UP * 100 / speed {
                // Far behind (a stall): carry on from now rather than run
                // fields back to back to catch up. Up to CATCH_UP late (a
                // scene load's slow field, or sleep overshoot), the next
                // fields skip their sleep until the schedule is met again.
                // No field starts before its slot, so the game never runs
                // ahead of real time.
                due = now;
            }
            due += period;
        }
        let (out, output_wait) = {
            let mut hw = sys.hw.borrow_mut();
            let waited = hw.gpu.waited();
            let out = hw.gpu.finish_output();
            let output_wait = hw.gpu.waited() - waited;
            totals.output_wait += output_wait;
            if hash_gx {
                gx_digest = (gx_digest.rotate_left(13) ^ out.digest()).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            }
            (out, output_wait)
        };
        if frames.send(Ok(GameFrame { out, xfb, time, emu, gx_wait, output_wait, index, wants_efb, remotes, main_player: view_player.unwrap_or(0), online: session.is_some(), visible, period, inputs, practice: practice.stats })).is_err() {
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
    if let Some((stats, counts)) = sys.hw.borrow_mut().ios.stop_passthrough() {
        let counts: Vec<String> = counts.iter().map(|(op, n)| format!("{op:#06x} x{n}")).collect();
        say!("[bluetooth] totals: {}; the game's commands: {}", stats.summary(), counts.join(", "));
    }
    totals.thread_cpu = crate::win::thread_cpu_time() - cpu_at_start;
    totals.texture_pack = pack_stats;
    totals
}

/// Bluetooth passthrough (experimental): the first usable USB Bluetooth
/// adapter (or with `NSMBW_BT_FAKE_ADAPTER=1` one that answers every
/// command itself) in place of the emulated remotes; off, with the reason
/// said, otherwise.
fn start_passthrough(sys: &mut System, off: Option<&str>) {
    use hle_ios::bt_passthrough::{Adapter, FakeAdapter};
    if let Some(why) = off {
        say!("Bluetooth passthrough: off ({why})");
        return;
    }
    let opened: Result<(Box<dyn Adapter>, String, bool), String> = if std::env::var("NSMBW_BT_FAKE_ADAPTER").is_ok_and(|v| v == "1") {
        Ok((Box::<FakeAdapter>::default(), "a fake adapter (NSMBW_BT_FAKE_ADAPTER=1)".into(), false))
    } else {
        #[cfg(windows)]
        let opened = input::bt_usb::open().map(|o| (Box::new(o.adapter) as Box<dyn Adapter>, o.name, o.wii_module));
        #[cfg(not(windows))]
        let opened = Err("Windows only".to_string());
        opened
    };
    match opened {
        Ok((adapter, name, wii_module)) => match sys.hw.borrow_mut().ios.start_passthrough(adapter, wii_module, PathBuf::from("saves/bt-passthrough")) {
            Ok(()) => say!(
                "Bluetooth passthrough: {name}{}. The game's own Bluetooth drives real Wii Remotes; the keyboard and controllers don't. F8 pairs remotes (held 10 s, it forgets them all).",
                if wii_module { " (a Wii's own Bluetooth module)" } else { "" }
            ),
            Err(e) => say_err!("Bluetooth passthrough: {e}; the emulated remotes play"),
        },
        Err(e) => say_err!("Bluetooth passthrough couldn't start: {e}. The keyboard and controllers play as usual."),
    }
}

/// Runs the game thread with no window or renderer (`--headless`) until it
/// stops by itself (`opts.stop_after`), dropping each field's output.
/// Nothing comes back from a renderer (EFB copies, EFB snapshots for CPU
/// peeks) and no host input arrives, so a run depends on its options only.
/// Returns the totals and the fields run.
pub fn run_headless(game: PathBuf, opts: EmuOptions, mut perf: Option<crate::perf::Perf>) -> Result<(Totals, u64), String> {
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
                if let Some(p) = &mut perf {
                    p.field(&f, Duration::ZERO, None);
                }
                let _ = emu.to_emu.send(ToEmu::Recycle(Box::new(f.out)));
            }
            Err(e) => {
                error = Some(e);
                break;
            }
        }
    }
    let totals = emu.stop();
    if let Some(p) = &mut perf {
        p.finish();
    }
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
        let multiplayer_button = Arc::new(AtomicBool::new(false));
        let save_select = Arc::new(std::sync::Mutex::new(None));
        let player = Arc::new(AtomicU64::new(0));
        let (s, m, p) = (stop.clone(), (settings_button.clone(), multiplayer_button.clone(), save_select.clone()), player.clone());
        let thread = std::thread::Builder::new().name("game".into()).stack_size(64 << 20).spawn(move || run(game, opts, frames_tx, to_emu_rx, s, m, p)).expect("spawn the game thread");
        Self { frames, to_emu, settings_button, multiplayer_button, save_select, player, stop, thread: Some(thread) }
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

#[cfg(test)]
mod press_tests {
    use super::*;

    #[test]
    fn scripted_buttons() {
        let s = parse_buttons("2+right+shake").unwrap();
        assert_eq!(s.buttons, button::TWO | button::RIGHT);
        assert!(s.shake);
        assert!(parse_buttons("x").is_err());
        let p = parse_script("10:a # boot\np2:20:2+right:3").unwrap();
        assert_eq!((p[0].player, p[0].start, p[0].count, p[0].input.buttons), (0, 10, 6, button::A));
        assert_eq!((p[1].player, p[1].start, p[1].count), (1, 20, 3));
    }
}
