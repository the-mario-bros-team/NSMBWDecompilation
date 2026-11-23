//! NSMBW Rust port — frontend binary (docs/PORT_PLAN.md).
//!
//! - default: runs the game in a window (interpreter, hardware and IOS HLE,
//!   GX decoded and drawn through Vulkan).
//! - `--check` validates the extracted game folder against the analysed image.
//! - `--boot` boots `main.dol` without a window and reports progress.

mod audio;
mod boot;
mod click;
// The processor check before main (Windows x86-64 only).
#[cfg(all(windows, target_arch = "x86_64"))]
mod cpu_guard;
mod emu;
mod gameplay;
mod gamefiles;
mod mods;
mod overlay;
mod play;
mod replay;
mod say;
mod settings;
mod ui;
mod win;

// Online multiplayer and finding rooms: shared with the Android app.
use netplay::{lan, net};

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage: nsmbw-port [--check | --boot [--steps <n>]] [--game <folder>] [--frames <n>] [--screenshot <file.png>]
                   [--fps <30|60|90|120|144|165|180|240|300|360|400|unlocked>] [--vsync | --no-vsync] [--fullscreen] [--scale <n|native>] [--unpaced]
                   [--press <frame>:<buttons>[:<frames>]]... [--shot <frame>:<file.png>]... [--trace-ios] [--profile-guest]
       nsmbw-port --replay <log.dff> [--frame <n>] [--scale <n>] [--screenshot <png>] [--compare <png>] [--min-psnr <dB>]
  (no mode)          run the game in a window (display settings from settings.toml; F1 lists the hotkeys)
  --replay <log.dff> play a Dolphin FIFO log headless, capture a frame (default the last), compare by PSNR (default 30 dB)
  --check            validate the extracted game folder and exit
  --boot             boot main.dol without a window and report progress (default limit 500M guest cycles)
  --steps <n>        guest-cycle limit for --boot (instructions plus idle time skipped)
  --game <folder>    extracted disc (holds sys/ and files/); default: game_dir in port.toml, else ./game
  --frames <n>       exit after presenting n frames and print the Vulkan validation summary
  --screenshot <png> with --frames: save the last frame's XFB
  --fps <rate>       frames shown per second; 30 shows every other field; above 60, in-between frames are interpolated
                     (the game always runs at 59.94)
  --vsync, --no-vsync  wait for the display's refresh, or not
  --fullscreen       borderless fullscreen (F11 toggles)
  --scale <n|native> EFB resolution: 0.5..8 times 480 lines, or the window's height
  --unpaced          run the game as fast as possible instead of 59.94 frames per second
  --press [p<k>:]<f>:<b>[:<n>]  hold Wii Remote k's (default 1) buttons b (a b 1 2 up down left right plus
                     minus home shake, joined with +; the remote's own D-pad) for n game frames (default 6)
                     from frame f; remotes 2-4 switch on at their first press (p2:<f>: with no buttons)
  --press-file <file>  presses from a file: <f>:<b>[:<n>] separated by spaces or lines, # comments
  --click <f>:<x>:<y>  click the game's picture after frame f, at (x, y) in layout units (456 high,
                     origin at the centre, y up), as a mouse click would (repeatable)
  --shot <f>:<png>   save game frame f's XFB (repeatable)
  --trace-ios        print every IOS request and the Bluetooth conversation
  --profile-guest    sample the guest PC and print the hottest functions at exit
  --log-fps          print frames shown and game speed once a second
  --gx-inline        decode GX on the game thread instead of its own (slower; for comparisons)
  --watch-player     print the players' positions every 10 game frames (for scripting --press runs)
  --save-state <f>:<file>  save the whole machine at the start of game frame f (repeatable)
  --load-state <file>  start from a saved state (frame numbers carry on from it)
  --hash-every <n>   print a hash of guest RAM every n game frames
  --hash-gx          with --hash-every: also print a running digest of the GX output (what the renderer gets)
  --dump-ram <f>:<file>  write MEM1 then MEM2 to a file at the start of game frame f (repeatable)
  --view-player <n>  the main window shows player n (default 1; online: your own); other local players get their own windows
  --view-zoom <z>    with --view-player: show 1/z of the camera around the player (for testing)
  --game-camera      keep the game's own multiplayer camera instead of a box around every player (not online)
  --no-feedback      ignore EFB copies and snapshots from the renderer (deterministic when paced too)
  --host <port>      host an online game: you are player 1, others join at any time with --join
  --wait-join <f>    with --host: at game frame f, wait until someone joins (for scripted tests)
  --join <host>:<port>  join an online game in progress as the next free player
  --net-latency <ms> online: hold every message this machine sends for ms first (for testing long connections)
  --wav <file>       also write the sound the game plays (16-bit stereo, 32 kHz) to a WAV file
  --no-audio         no sound output (unpaced runs never play sound)
  --interpreter      do not run recompiled code (crates/game-recomp), only the interpreter
  --wide <w:h>       headless: draw the game's view that wide, as Ultrawide in a window of that shape does (lockstep checks)
  --headless         with --frames: no window or renderer (for checks such as --hash-every): runs the same
                     fields as a windowed run, without input from the host or anything the renderer sends back
  --code-profile <file>  at exit, write every guest address a decoded block started at (input for ppc-recomp)
  --settings-buttons the game menus' Settings buttons in a scripted (--press) or headless run too
  --gameplay-options with --headless: settings.toml's Gameplay options, as a window has them (else all off);
                     with --press or --headless: player 1's character pick too (otherwise a player's game only)
  --course <w>-<l>[:<area>[:<entrance>]]  when the game next starts a course, start this one instead
                     (numbered as Stage/WW-LL.arc; scripts/to-map.press enters World 1-1 from boot)
  flags override settings.toml for this run only";

struct Args {
    /// `--settings-buttons`.
    settings_buttons: bool,
    /// `--gameplay-options`.
    gameplay_options: bool,
    check: bool,
    boot: bool,
    steps: u64,
    frames: Option<u64>,
    screenshot: Option<PathBuf>,
    unpaced: bool,
    game: Option<PathBuf>,
    settings: settings::Settings,
    replay: Option<PathBuf>,
    frame: Option<usize>,
    compare: Option<PathBuf>,
    min_psnr: f64,
    script: Vec<emu::Press>,
    clicks: Vec<(u64, click::Click)>,
    shots: Vec<(u64, PathBuf)>,
    trace_ios: bool,
    profile: bool,
    log_fps: bool,
    gx_inline: bool,
    watch_player: bool,
    save_states: Vec<(u64, PathBuf)>,
    load_state: Option<PathBuf>,
    hash_every: u64,
    hash_gx: bool,
    no_feedback: bool,
    net: net::Mode,
    dump_ram: Vec<(u64, PathBuf)>,
    view_player: Option<usize>,
    view_zoom: f32,
    game_camera: bool,
    wav: Option<PathBuf>,
    no_audio: bool,
    code_profile: Option<PathBuf>,
    interpreter: bool,
    headless: bool,
    wide: Option<f32>,
    course: Option<emu::Course>,
}

/// `<world>-<level>[:<area>[:<entrance>]]`, numbered as the files in
/// `Stage/` (`3-24` is `Stage/03-24.arc`); entrance 255 is the default.
fn parse_course(v: &str) -> Result<emu::Course, String> {
    let bad = || format!("--course {v}: expected <world>-<level>[:<area>[:<entrance>]], e.g. 3-24");
    let mut parts = v.split(':');
    let (w, l) = parts.next().and_then(|c| c.split_once('-')).ok_or_else(bad)?;
    let (world, level): (u8, u8) = (w.parse().map_err(|_| bad())?, l.parse().map_err(|_| bad())?);
    if !(1..=9).contains(&world) || !(1..=99).contains(&level) {
        return Err(bad());
    }
    let mut next = || parts.next().map(|n| n.parse::<u8>().map_err(|_| bad())).transpose();
    let (area, entrance) = (next()?, next()?);
    Ok(emu::Course { world: world - 1, level: level - 1, area, entrance })
}

/// `[p<n>:]<frame>:<buttons>[:<frames>]`
fn parse_press(v: &str) -> Result<emu::Press, String> {
    let (player, rest) = match v.strip_prefix('p').and_then(|r| r.split_once(':')) {
        Some((n, rest)) => match n.parse::<usize>() {
            Ok(n @ 1..=4) => (n - 1, rest),
            _ => return Err(format!("--press {v}: player is p1..p4")),
        },
        None => (0, v),
    };
    let mut parts = rest.split(':');
    let start = parts.next().and_then(|s| s.parse().ok()).ok_or(format!("--press {v}: expected <frame>:<buttons>[:<frames>]"))?;
    let input = input::parse_buttons(parts.next().unwrap_or(""))?;
    let count = match parts.next() {
        Some(n) => n.parse().map_err(|_| format!("--press {v}: bad frame count"))?,
        None => 6,
    };
    Ok(emu::Press { player, start, count, input })
}

fn number<T: std::str::FromStr>(it: &mut impl Iterator<Item = String>, flag: &str) -> Result<T, String> {
    let v = it.next().ok_or(format!("{flag} needs a number"))?;
    v.parse().map_err(|_| format!("{flag}: not a number: {v}"))
}

fn parse_args() -> Result<Args, String> {
    let settings = settings::Settings::load(Path::new(settings::FILE));
    let mut args = Args { settings_buttons: false, gameplay_options: false, check: false, boot: false, steps: 500_000_000, frames: None, screenshot: None, unpaced: false, game: None, settings, replay: None, frame: None, compare: None, min_psnr: 30.0, script: Vec::new(), clicks: Vec::new(), shots: Vec::new(), trace_ios: false, profile: false, log_fps: false, gx_inline: false, watch_player: false, save_states: Vec::new(), load_state: None, hash_every: 0, hash_gx: false, no_feedback: false, net: net::Mode::Off, dump_ram: Vec::new(), view_player: None, view_zoom: 1.0, game_camera: false, wav: None, no_audio: false, code_profile: None, interpreter: false, headless: false, wide: None, course: None };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--check" => args.check = true,
            "--boot" => args.boot = true,
            "--steps" => args.steps = number(&mut it, "--steps")?,
            "--frames" => args.frames = Some(number(&mut it, "--frames")?),
            "--scale" => {
                let v = it.next().ok_or("--scale needs a number or native")?;
                args.settings.scale = if v == "native" { 0.0 } else { settings::clamp_scale(v.parse::<f32>().map_err(|_| format!("--scale: not a number: {v}"))?) };
            }
            "--fps" => {
                let v = it.next().ok_or("--fps needs a rate")?;
                args.settings.fps = settings::parse_fps(&v).ok_or(format!("--fps: one of 30, 60, 90, 120, 144, 165, 180, 240, 300, 360, 400 or unlocked, not {v}"))?;
            }
            "--vsync" => args.settings.vsync = true,
            "--no-vsync" => args.settings.vsync = false,
            "--fullscreen" => args.settings.fullscreen = true,
            "--replay" => args.replay = Some(it.next().ok_or("--replay needs a .dff file")?.into()),
            "--frame" => args.frame = Some(number(&mut it, "--frame")?),
            "--compare" => args.compare = Some(it.next().ok_or("--compare needs a PNG")?.into()),
            "--min-psnr" => args.min_psnr = number(&mut it, "--min-psnr")?,
            "--screenshot" => args.screenshot = Some(it.next().ok_or("--screenshot needs a file")?.into()),
            "--unpaced" => args.unpaced = true,
            "--trace-ios" => args.trace_ios = true,
            "--profile-guest" => args.profile = true,
            "--log-fps" => args.log_fps = true,
            "--gx-inline" => args.gx_inline = true,
            "--watch-player" => args.watch_player = true,
            "--wav" => args.wav = Some(it.next().ok_or("--wav needs a file")?.into()),
            "--no-audio" => args.no_audio = true,
            "--interpreter" => args.interpreter = true,
            "--wide" => {
                let v = it.next().ok_or("--wide needs <w:h>")?;
                let ratio = match v.split_once(':') {
                    Some((w, h)) => w.parse::<f32>().ok().zip(h.parse::<f32>().ok()).map(|(w, h)| w / h),
                    None => v.parse::<f32>().ok(),
                };
                args.wide = Some(ratio.filter(|r| r.is_finite() && (1.0..=5.0).contains(r)).ok_or(format!("--wide: expected <w:h> such as 21:9, not {v}"))?);
            }
            "--settings-buttons" => args.settings_buttons = true,
            "--gameplay-options" => args.gameplay_options = true,
            "--headless" => args.headless = true,
            "--code-profile" => args.code_profile = Some(it.next().ok_or("--code-profile needs a file")?.into()),
            "--click" => {
                let v = it.next().ok_or("--click needs <frame>:<x>:<y>")?;
                let n: Vec<&str> = v.split(':').collect();
                let bad = || format!("--click {v}: expected <frame>:<x>:<y>");
                let [f, x, y] = n[..] else { return Err(bad()) };
                let c = click::Click { x: x.parse().map_err(|_| bad())?, y: y.parse().map_err(|_| bad())?, half_width: 228.0 * 16.0 / 9.0 };
                args.clicks.push((f.parse().map_err(|_| bad())?, c));
            }
            "--press" => args.script.push(parse_press(&it.next().ok_or("--press needs <frame>:<buttons>")?)?),
            "--press-file" => {
                let path = it.next().ok_or("--press-file needs a file")?;
                let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
                // One press per line or several per line; `#` starts a comment.
                for word in text.lines().flat_map(|l| l.split('#').next().unwrap_or("").split_whitespace()) {
                    args.script.push(parse_press(word)?);
                }
            }
            "--save-state" => {
                let v = it.next().ok_or("--save-state needs <frame>:<file>")?;
                let (f, p) = v.split_once(':').ok_or(format!("--save-state {v}: expected <frame>:<file>"))?;
                args.save_states.push((f.parse().map_err(|_| format!("--save-state {v}: bad frame"))?, PathBuf::from(p)));
            }
            "--dump-ram" => {
                let v = it.next().ok_or("--dump-ram needs <frame>:<file>")?;
                let (f, p) = v.split_once(':').ok_or(format!("--dump-ram {v}: expected <frame>:<file>"))?;
                args.dump_ram.push((f.parse().map_err(|_| format!("--dump-ram {v}: bad frame"))?, PathBuf::from(p)));
            }
            "--view-player" => {
                let n: usize = number(&mut it, "--view-player")?;
                if !(1..=4).contains(&n) {
                    return Err("--view-player: 1..4".into());
                }
                args.view_player = Some(n - 1);
            }
            "--view-zoom" => args.view_zoom = number(&mut it, "--view-zoom")?,
            "--game-camera" => args.game_camera = true,
            "--load-state" => args.load_state = Some(it.next().ok_or("--load-state needs a file")?.into()),
            "--hash-every" => args.hash_every = number(&mut it, "--hash-every")?,
            "--hash-gx" => args.hash_gx = true,
            "--no-feedback" => args.no_feedback = true,
            "--host" => {
                let wait_join = match &args.net {
                    net::Mode::Host { wait_join, .. } => *wait_join,
                    _ => None,
                };
                args.net = net::Mode::Host { port: number(&mut it, "--host")?, wait_join };
            }
            "--wait-join" => {
                let f = number(&mut it, "--wait-join")?;
                args.net = match args.net {
                    net::Mode::Host { port, .. } => net::Mode::Host { port, wait_join: Some(f) },
                    _ => return Err("--wait-join comes after --host".into()),
                };
            }
            "--join" => args.net = net::Mode::Join { addr: it.next().ok_or("--join needs <host>:<port>")? },
            "--net-latency" => net::LATENCY_MS.store(number(&mut it, "--net-latency")?, std::sync::atomic::Ordering::Relaxed),
            "--shot" => {
                let v = it.next().ok_or("--shot needs <frame>:<file.png>")?;
                let (f, p) = v.split_once(':').ok_or(format!("--shot {v}: expected <frame>:<file.png>"))?;
                args.shots.push((f.parse().map_err(|_| format!("--shot {v}: bad frame"))?, PathBuf::from(p)));
            }
            "--game" => args.game = Some(it.next().ok_or("--game needs a folder")?.into()),
            "--course" => args.course = Some(parse_course(&it.next().ok_or("--course needs <world>-<level>")?)?),
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    if args.screenshot.is_some() && args.frames.is_none() && args.replay.is_none() {
        return Err("--screenshot needs --frames".into());
    }
    if args.headless && (args.frames.is_none() || args.screenshot.is_some() || !args.shots.is_empty()) {
        return Err("--headless needs --frames, and draws no screenshots".into());
    }
    Ok(args)
}

/// `--game`, else `game_dir` from `port.toml` in the current folder, else `./game`.
fn game_dir(cli: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = cli {
        return Ok(p);
    }
    match std::fs::read_to_string("port.toml") {
        Ok(text) => {
            let table: toml::Table = text.parse().map_err(|e| format!("port.toml: {e}"))?;
            table
                .get("game_dir")
                .and_then(|v| v.as_str())
                .map(PathBuf::from)
                .ok_or_else(|| "port.toml: missing game_dir = '<folder>'".to_string())
        }
        Err(_) => Ok(PathBuf::from("game")),
    }
}

fn main() -> ExitCode {
    if cfg!(all(target_arch = "x86_64", not(target_feature = "avx2"))) {
        eprintln!("compat build (x86-64-v2, for processors without AVX2)");
    }
    if std::env::args().any(|a| a == "-h" || a == "--help") {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    if let Some(log) = args.replay {
        let scale = if args.settings.scale == 0.0 { 1.0 } else { args.settings.scale };
        let opts = replay::Options { log, frame: args.frame, scale, screenshot: args.screenshot, compare: args.compare, min_psnr: args.min_psnr };
        return match replay::run(&opts) {
            Ok(true) if vk_render::validation_errors() == 0 => ExitCode::SUCCESS,
            Ok(_) => ExitCode::FAILURE,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let game = match game_dir(args.game) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };

    if args.check || args.boot {
        let ok = if args.check { gamefiles::check(&game, Path::new("work")) } else { boot::run(&game, args.steps) };
        return if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE };
    }

    // A window always applies settings.toml's Gameplay options (play.rs).
    let gameplay = if args.gameplay_options { play::gameplay_options(&args.settings) } else { Default::default() };
    let (texture_pack, mods) = (mods::PackSpec::from_settings(&args.settings), args.settings.mods.clone());
    let opts = play::Options { game, max_frames: args.frames, screenshot: args.screenshot, unpaced: args.unpaced, settings: args.settings, shots: args.shots, log_fps: args.log_fps, emu: emu::EmuOptions { unpaced: args.unpaced, trace_ios: args.trace_ios, profile: args.profile, script: args.script, gx_inline: args.gx_inline, watch_player: args.watch_player, save_states: args.save_states, load_state: args.load_state, hash_every: args.hash_every, hash_gx: args.hash_gx, no_feedback: args.no_feedback, net: args.net, dump_ram: args.dump_ram, view_player: args.view_player, view_zoom: args.view_zoom, game_camera: args.game_camera, wav: args.wav, audio: !args.no_audio, code_profile: args.code_profile, interpreter: args.interpreter, stop_after: None, settings_label: args.settings_buttons.then(|| "Settings".into()), force_settings_buttons: args.settings_buttons, auto_close_settings: false, cheats: Default::default(), gameplay, force_gameplay_options: args.gameplay_options, characters: Default::default(), clicks: args.clicks, wide: args.wide, texture_pack, mods, course: args.course, bt_passthrough: false } };
    if args.headless {
        // As a window would, before the game boots: the game reads its
        // aspect ratio from SYSCONF once.
        if let Err(e) = hle_ios::nand::set_widescreen(Path::new("saves/nand"), opts.settings.widescreen) {
            eprintln!("SYSCONF: {e}");
        }
        win::tune_process();
        let emu = emu::EmuOptions { stop_after: opts.max_frames, auto_close_settings: true, ..opts.emu };
        return match emu::run_headless(opts.game, emu) {
            Ok((totals, fields)) => {
                play::report(&totals);
                println!("{fields} game frames, headless");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        };
    }
    match play::run(opts) {
        Ok((games, presented)) => {
            let errors = vk_render::validation_errors();
            println!("{games} game frames, {presented} presented; validation errors: {errors}, warnings: {}", vk_render::validation_warnings());
            if errors == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
