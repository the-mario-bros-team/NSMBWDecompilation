//! The NSMBW port's launcher (Windows): pick a version of the game, point it
//! at your own extracted copy, and play. Each version keeps its own game
//! folder, settings and saves. See `store.rs` for what is kept where and
//! `versions.toml` for the versions.
//!
//! With Skip Launcher on, starting the launcher starts the game straight
//! away; holding Shift (or Back on an Xbox controller) while it starts
//! shows the launcher again.

// No console window of its own (a build with debug assertions keeps one).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod banner;
mod dialogs;
mod disc;
mod draw;
mod gameart;
mod gamebg;
mod launch;
mod nav;
mod scene;
mod screen_main;
mod screen_settings;
mod screen_setup;
mod screen_switch;
mod store;
mod versions;

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

const USAGE: &str = "usage: nsmbw-launcher [--launcher] [--root <folder>]
                      [--script <frame>:<move>,...] [--shot <frame>:<file.png>]... [--frames <n>] [--size <w>x<h>]
  --launcher         show the launcher even when Skip Launcher is on (as holding Shift does)
  --root <folder>    where the launcher keeps everything and looks for nsmbw-port.exe
                     (default: the folder nsmbw-launcher.exe is in)
  for tests, as nsmbw-port's --press and --shot:
  --script <moves>   controller moves at given frames: up down left right a b x y start lb rb
  --shot <f>:<png>   save the window's picture at frame f (repeatable)
  --frames <n>       close after n frames
  --size <w>x<h>     the window's size";

struct Args {
    launcher: bool,
    root: PathBuf,
    script: Vec<(u64, nav::Nav)>,
    shots: Vec<(u64, PathBuf)>,
    frames: Option<u64>,
    size: [f32; 2],
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args { launcher: false, root: port_update::install_dir(), script: Vec::new(), shots: Vec::new(), frames: None, size: [1280.0, 720.0] };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--launcher" => args.launcher = true,
            "--root" => args.root = it.next().ok_or("--root needs a folder")?.into(),
            "--script" => args.script.extend(nav::parse_script(&it.next().ok_or("--script needs moves")?)?),
            "--shot" => {
                let v = it.next().ok_or("--shot needs <frame>:<file.png>")?;
                let (f, p) = v.split_once(':').ok_or(format!("--shot {v}: expected <frame>:<file.png>"))?;
                args.shots.push((f.parse().map_err(|_| format!("--shot {v}: bad frame"))?, PathBuf::from(p)));
            }
            "--frames" => args.frames = Some(it.next().and_then(|n| n.parse().ok()).ok_or("--frames needs a number")?),
            "--size" => {
                let v = it.next().ok_or("--size needs <w>x<h>")?;
                let size = v.split_once('x').and_then(|(w, h)| Some([w.parse().ok()?, h.parse().ok()?]));
                args.size = size.ok_or(format!("--size {v}: expected <w>x<h>"))?;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(args)
}

/// Skip Launcher: starts the selected version's game without showing the
/// launcher. `Ok` when the game is running; `Err` says why the launcher has
/// to show itself after all.
fn skip_to_game(root: &std::path::Path) -> Result<(), (String, String)> {
    let mut store = store::Store::load(root.to_path_buf());
    let (versions, _) = versions::load(root);
    let version = versions.iter().find(|v| v.id == store.state.selected).unwrap_or(&versions[0]);
    let title = format!("{} could not start", version.title);
    let mut child = launch::start(&store, version).map_err(|e| (title.clone(), e))?;
    let started = Instant::now();

    // The launcher's screen is not shown, so the update check is made here:
    // a release found now makes the launcher show itself once, next time.
    let config = port_update::Config::load(root);
    if config.enabled {
        if let Ok(port_update::Status::Available(release)) = port_update::check(&config) {
            if release.version != store.state.update_told {
                store.state.update_pending = release.version;
                store.save();
            }
        }
    }

    // A game that stops at once (a moved folder, a missing file) is not
    // left unexplained.
    loop {
        match child.try_wait() {
            Ok(Some(status)) if !status.success() => {
                let tail = launch::log_tail(&store.version_dir(version));
                let said = if tail.is_empty() { String::new() } else { format!(" It said:\n{tail}") };
                return Err((title, format!("The game closed as soon as it started.{said}")));
            }
            Ok(Some(_)) => return Ok(()),
            _ if started.elapsed() > Duration::from_millis(1500) => return Ok(()),
            _ => std::thread::sleep(Duration::from_millis(50)),
        }
    }
}

fn main() -> ExitCode {
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
    if let Err(e) = std::fs::create_dir_all(&args.root) {
        eprintln!("{}: {e}", args.root.display());
        return ExitCode::FAILURE;
    }

    let mut notice = None;
    let state = store::Store::load(args.root.clone()).state;
    let scripted = !args.script.is_empty() || !args.shots.is_empty() || args.frames.is_some();
    // A release found during the last skipped start is shown first.
    if state.skip_launcher && state.update_pending.is_empty() && !args.launcher && !scripted && !launch::shift_held() && !launch::pad_back_held() {
        match skip_to_game(&args.root) {
            Ok(()) => return ExitCode::SUCCESS,
            Err(why) => notice = Some(why),
        }
    }

    let (versions, _) = versions::load(&args.root);
    let icon = scene::render(&versions[0].theme.scene, 64, 64);
    let viewport = egui::ViewportBuilder::default()
        .with_title("NSMBW Port Launcher")
        .with_inner_size(args.size)
        .with_min_inner_size([960.0, 540.0])
        .with_icon(egui::IconData { rgba: icon.pixels.iter().flat_map(|c| c.to_array()).collect(), width: 64, height: 64 });
    let native = eframe::NativeOptions { viewport, centered: true, ..Default::default() };
    let options = app::Options { root: args.root, script: args.script, shots: args.shots, frames: args.frames, notice };
    match eframe::run_native("nsmbw-launcher", native, Box::new(move |cc| Ok(Box::new(app::App::new(cc, options))))) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("the launcher's window could not be opened: {e}");
            ExitCode::FAILURE
        }
    }
}
