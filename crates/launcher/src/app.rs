//! The launcher's window: what it knows (versions, their folders and art,
//! the update check), the screens stacked on the main menu, and each frame's
//! input, drawing and actions.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::process::Child;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use egui::{pos2, vec2, Align2, Color32, ColorImage, Pos2, Rect, TextureHandle, TextureOptions};

use crate::dialogs::{self, Dialog};
use crate::disc::{self, Found, Report};
use crate::draw::*;
use crate::gameart::{self, Pictures};
use crate::nav::{Input, Nav};
use crate::store::Store;
use crate::versions::Version;
use crate::{launch, scene, screen_main, screen_settings, screen_setup, screen_switch};

/// What a version looks like right now: its folder, its state, its art.
pub struct Slot {
    pub game: Option<PathBuf>,
    pub found: Option<Found>,
    /// Why it cannot be started, when it has a game folder but cannot.
    pub problem: Option<String>,
    /// The version's scene as a still picture.
    pub cover: TextureHandle,
    /// From the player's game folder.
    pub logo: Option<TextureHandle>,
    pub gallery: Vec<TextureHandle>,
}

pub enum UpdateStatus {
    /// Not asked: checking is off.
    Off,
    Checking,
    UpToDate,
    Available(port_update::Release),
    Failed(String),
}

pub struct Update {
    pub config: port_update::Config,
    pub status: UpdateStatus,
}

/// Where a picked game folder is in its validation.
pub enum Check {
    None,
    /// Identified and accepted; the game's own check is running.
    Running { found: Found },
    Done {
        found: Option<Found>,
        /// `Err`: why the folder was refused.
        verdict: Result<(), String>,
        report: Option<Report>,
        saved: bool,
    },
}

/// Everything the screens read.
pub struct Shared {
    pub store: Store,
    pub versions: Vec<Version>,
    pub slots: Vec<Slot>,
    pub selected: usize,
    pub update: Update,
    pub check: Check,
    /// The connected controller's name.
    pub pad: Option<String>,
}

/// What a screen asks the launcher to do.
pub enum Action {
    Close,
    OpenSwitcher,
    OpenSetup,
    OpenBrowser,
    OpenSettings,
    OpenUpdate,
    Select(usize),
    Play,
    ToggleSkip,
    SetSkip(bool),
    PickFolder(PathBuf),
    WindowsDialog,
    ForgetFolder,
    Setting(usize, usize),
    SetUpdateCheck(bool),
    CheckUpdates,
    OpenReleasePage,
    OpenVersionFolder,
}

pub enum Kind {
    Switcher(screen_switch::State),
    Setup(Focus),
    Browser(screen_setup::Browser),
    Settings(screen_settings::State),
    Dialog(Dialog),
}

/// A screen over the main menu, fading in (`t` to 1) or out.
pub struct Layer {
    kind: Kind,
    t: f32,
    closing: bool,
}

struct Launch {
    child: Child,
    since: Instant,
    /// Where Play is: the transition grows from there.
    from: Pos2,
}

pub struct Options {
    pub root: PathBuf,
    pub script: Vec<(u64, Nav)>,
    pub shots: Vec<(u64, PathBuf)>,
    pub frames: Option<u64>,
    /// Something to tell the player at start (why skipping did not work).
    pub notice: Option<(String, String)>,
}

pub struct App {
    shared: Shared,
    input: Input,
    anim: Anim,
    main_focus: Focus,
    layers: Vec<Layer>,
    time: f32,
    frame: u64,
    /// Driven by a script: time advances by the frame, not the clock.
    scripted: bool,
    soft: TextureHandle,
    ramp: TextureHandle,
    /// The version selected before the last switch, and the switch's
    /// progress (0..1).
    previous: usize,
    switched: f32,
    play_at: Pos2,
    /// Where the folder browser was last, to open there again.
    browsed: Option<PathBuf>,
    launch: Option<Launch>,
    art_tx: Sender<(usize, PathBuf, Pictures)>,
    art_rx: Receiver<(usize, PathBuf, Pictures)>,
    check_tx: Sender<(usize, Report)>,
    check_rx: Receiver<(usize, Report)>,
    update_rx: Option<Receiver<Result<port_update::Status, String>>>,
    /// A release found while the launcher was skipped: shown once it is
    /// confirmed by this start's check.
    update_pending: bool,
    shots: Vec<(u64, PathBuf)>,
    shots_wanted: VecDeque<PathBuf>,
    frames: Option<u64>,
}

/// The still picture's size: enough for the largest card on a 1440p screen.
const COVER: [usize; 2] = [760, 532];

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, options: Options) -> Self {
        let ctx = &cc.egui_ctx;
        install_fonts(ctx);
        let soft = ctx.load_texture("soft", scene::soft_dot(), TextureOptions::LINEAR);
        let ramp_pixels = (0..64).map(|y| Color32::from_white_alpha((y as f32 / 63.0 * 255.0) as u8)).collect();
        let ramp = ctx.load_texture("ramp", ColorImage::new([1, 64], ramp_pixels), TextureOptions::LINEAR);

        let mut store = Store::load(options.root.clone());
        let (versions, trouble) = crate::versions::load(&options.root);
        let selected = versions.iter().position(|v| v.id == store.state.selected).unwrap_or(0);
        let slots = versions.iter().map(|v| Slot { game: None, found: None, problem: None, cover: ctx.load_texture(format!("cover-{}", v.id), scene::render(&v.theme.scene, COVER[0], COVER[1]), TextureOptions::LINEAR), logo: None, gallery: Vec::new() }).collect();
        let update_pending = !store.state.update_pending.is_empty();
        if update_pending {
            store.state.update_pending.clear();
            store.save();
        }
        let config = port_update::Config::load(&options.root);
        let (art_tx, art_rx) = channel();
        let (check_tx, check_rx) = channel();
        let scripted = !options.script.is_empty() || !options.shots.is_empty() || options.frames.is_some();
        let mut app = Self {
            shared: Shared { store, versions, slots, selected, update: Update { config, status: UpdateStatus::Off }, check: Check::None, pad: None },
            input: Input::new(options.script),
            anim: Anim::default(),
            main_focus: Focus::starting_at("play"),
            layers: Vec::new(),
            time: 0.0,
            frame: 0,
            scripted,
            soft,
            ramp,
            previous: selected,
            switched: 1.0,
            play_at: Pos2::ZERO,
            browsed: None,
            launch: None,
            art_tx,
            art_rx,
            check_tx,
            check_rx,
            update_rx: None,
            update_pending,
            shots: options.shots,
            shots_wanted: VecDeque::new(),
            frames: options.frames,
        };
        for i in 0..app.shared.versions.len() {
            app.refresh(i);
        }
        app.check_updates();
        for (title, body) in trouble.map(|t| ("The version list has a mistake".to_string(), t)).into_iter().chain(options.notice) {
            app.open(Kind::Dialog(Dialog::message(&title, &body)));
        }
        app
    }

    /// Reads version `i`'s folder and state again, and its art from the
    /// game folder (on another thread; it arrives a moment later).
    fn refresh(&mut self, i: usize) {
        let sh = &mut self.shared;
        let v = &sh.versions[i];
        let game = sh.store.game_dir(v);
        let found = game.as_deref().and_then(|g| disc::identify(g).ok());
        let problem = game.as_ref().and_then(|_| launch::problem(&sh.store, v)).filter(|_| v.playable);
        let slot = &mut sh.slots[i];
        if slot.game != game {
            slot.logo = None;
            slot.gallery.clear();
            if let Some(game) = game.clone() {
                let (tx, art) = (self.art_tx.clone(), v.art.clone());
                std::thread::spawn(move || {
                    let pictures = gameart::load(&game, &art);
                    let _ = tx.send((i, game, pictures));
                });
            }
        }
        slot.game = game;
        slot.found = found;
        slot.problem = problem;
    }

    fn check_updates(&mut self) {
        if !self.shared.update.config.enabled {
            self.shared.update.status = UpdateStatus::Off;
            self.update_rx = None;
            return;
        }
        let (tx, rx) = channel();
        let config = self.shared.update.config.clone();
        std::thread::spawn(move || {
            let _ = tx.send(port_update::check(&config));
        });
        self.shared.update.status = UpdateStatus::Checking;
        self.update_rx = Some(rx);
    }

    /// Takes what the background threads finished.
    fn poll_jobs(&mut self, ctx: &egui::Context) {
        while let Ok((i, game, pictures)) = self.art_rx.try_recv() {
            let slot = &mut self.shared.slots[i];
            if slot.game.as_ref() != Some(&game) {
                continue;
            }
            let upload = |name: String, p: &gameart::Picture| ctx.load_texture(name, ColorImage::from_rgba_unmultiplied([p.width, p.height], &p.rgba[..p.width * p.height * 4]), TextureOptions::LINEAR);
            slot.logo = pictures.logo.as_ref().map(|p| upload(format!("logo-{i}"), p));
            slot.gallery = pictures.gallery.iter().enumerate().map(|(k, p)| upload(format!("gallery-{i}-{k}"), p)).collect();
        }
        while let Ok((i, report)) = self.check_rx.try_recv() {
            let Check::Running { found } = &self.shared.check else { continue };
            let found = found.clone();
            // A check that could not run at all (the game program is
            // missing) does not refuse the folder: Play says what is wrong.
            let accept = report.passed || (report.error.is_some() && report.lines.is_empty());
            let mut verdict = Ok(());
            let mut saved = false;
            if accept {
                match self.shared.store.set_game_dir(&self.shared.versions[i], Some(&found.root)) {
                    Ok(()) => saved = true,
                    Err(e) => verdict = Err(e),
                }
            } else {
                verdict = Err("The game's own check found problems with this folder, so it was not set.".into());
            }
            self.shared.check = Check::Done { found: Some(found), verdict, report: Some(report), saved };
            self.refresh(i);
        }
        if let Some(result) = self.update_rx.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.update_rx = None;
            self.shared.update.status = match result {
                Ok(port_update::Status::Available(release)) => {
                    if self.update_pending {
                        self.update_pending = false;
                        self.open(Kind::Dialog(Dialog::update()));
                        self.told(&release.version);
                    }
                    UpdateStatus::Available(release)
                }
                Ok(port_update::Status::UpToDate) => UpdateStatus::UpToDate,
                Err(e) => UpdateStatus::Failed(e),
            };
        }
    }

    /// Remembers that the player has been shown release `version`.
    fn told(&mut self, version: &str) {
        if self.shared.store.state.update_told != version {
            self.shared.store.state.update_told = version.to_string();
            self.shared.store.save();
        }
    }

    fn open(&mut self, kind: Kind) {
        self.layers.push(Layer { kind, t: 0.0, closing: false });
    }

    fn close_top(&mut self) {
        if let Some(layer) = self.layers.iter_mut().rev().find(|l| !l.closing) {
            layer.closing = true;
        }
    }

    fn message(&mut self, title: &str, body: &str) {
        self.open(Kind::Dialog(Dialog::message(title, body)));
    }

    /// A folder was picked for the selected version: identify it, judge it,
    /// and run the game's own check when the version can be played.
    fn pick_folder(&mut self, picked: PathBuf) {
        // Back to the setup sheet, which shows the result.
        if let Some(Kind::Browser(browser)) = self.layers.iter().rev().find(|l| !l.closing).map(|l| &l.kind) {
            self.browsed = browser.dir().map(PathBuf::from);
            self.close_top();
        }
        if !self.layers.iter().any(|l| !l.closing && matches!(l.kind, Kind::Setup(_))) {
            self.open(Kind::Setup(Focus::default()));
        }
        let sh = &mut self.shared;
        let v = &sh.versions[sh.selected];
        let found = match disc::identify(&picked) {
            Ok(f) => f,
            Err(e) => {
                sh.check = Check::Done { found: None, verdict: Err(e), report: None, saved: false };
                return;
            }
        };
        if let Err(e) = disc::judge(&found, v, &sh.versions) {
            sh.check = Check::Done { found: Some(found), verdict: Err(e), report: None, saved: false };
            return;
        }
        if !v.playable {
            // Nothing can check it yet: it is kept for when the port can.
            let verdict = sh.store.set_game_dir(v, Some(&found.root));
            let saved = verdict.is_ok();
            sh.check = Check::Done { found: Some(found), verdict, report: None, saved };
            let i = sh.selected;
            self.refresh(i);
            return;
        }
        let (exe, dir, game, tx, i) = (sh.store.game_exe(), sh.store.version_dir(v), found.root.clone(), self.check_tx.clone(), sh.selected);
        sh.check = Check::Running { found };
        std::thread::spawn(move || {
            let _ = tx.send((i, disc::full_check(&exe, &dir, &game)));
        });
    }

    fn apply(&mut self, action: Action, frame: &eframe::Frame) {
        match action {
            Action::Close => self.close_top(),
            Action::OpenSwitcher => self.open(Kind::Switcher(screen_switch::State::new(self.shared.selected))),
            Action::OpenSetup => {
                self.shared.check = Check::None;
                self.open(Kind::Setup(Focus::default()));
            }
            Action::OpenBrowser => {
                let start = self.browsed.clone().or_else(|| self.shared.slots[self.shared.selected].game.as_ref().and_then(|g| g.parent().map(PathBuf::from)));
                self.open(Kind::Browser(screen_setup::Browser::new(start)));
            }
            Action::OpenSettings => {
                let dir = self.shared.store.version_dir(&self.shared.versions[self.shared.selected]);
                self.open(Kind::Settings(screen_settings::State::new(&dir)));
            }
            Action::OpenUpdate => {
                if let UpdateStatus::Available(release) = &self.shared.update.status {
                    let version = release.version.clone();
                    self.told(&version);
                    self.open(Kind::Dialog(Dialog::update()));
                }
            }
            Action::Select(i) => {
                if i != self.shared.selected && i < self.shared.versions.len() {
                    self.previous = self.shared.selected;
                    self.shared.selected = i;
                    self.switched = 0.0;
                    self.shared.store.state.selected = self.shared.versions[i].id.clone();
                    self.shared.store.save();
                    self.shared.check = Check::None;
                }
            }
            Action::Play => self.play(),
            Action::ToggleSkip => {
                if self.shared.store.state.skip_launcher {
                    self.shared.store.state.skip_launcher = false;
                    self.shared.store.save();
                } else {
                    let title = &self.shared.versions[self.shared.selected].title;
                    self.open(Kind::Dialog(Dialog::skip(title)));
                }
            }
            Action::SetSkip(on) => {
                self.shared.store.state.skip_launcher = on;
                self.shared.store.save();
            }
            Action::PickFolder(picked) => self.pick_folder(picked),
            Action::WindowsDialog => {
                let picked = rfd::FileDialog::new().set_title("Pick the extracted game folder (it holds sys and files)").set_parent(frame).pick_folder();
                if let Some(picked) = picked {
                    self.pick_folder(picked);
                }
            }
            Action::ForgetFolder => {
                let i = self.shared.selected;
                if let Err(e) = self.shared.store.set_game_dir(&self.shared.versions[i], None) {
                    self.message("The game folder could not be removed", &e);
                }
                self.shared.check = Check::None;
                self.refresh(i);
            }
            Action::Setting(setting, choice) => {
                let dir = self.shared.store.version_dir(&self.shared.versions[self.shared.selected]);
                if let Err(e) = crate::store::write_setting(&dir, setting, choice) {
                    self.message("The setting could not be saved", &e);
                }
            }
            Action::SetUpdateCheck(on) => {
                self.shared.update.config.enabled = on;
                if let Err(e) = self.shared.update.config.save(&self.shared.store.root) {
                    self.message("The setting could not be saved", &format!("{}: {e}", port_update::FILE));
                }
                self.check_updates();
            }
            Action::CheckUpdates => self.check_updates(),
            Action::OpenReleasePage => {
                if let UpdateStatus::Available(release) = &self.shared.update.status {
                    if !launch::open_url(&release.page) {
                        self.message("The page could not be opened", "The release has no web page to open.");
                    }
                }
            }
            Action::OpenVersionFolder => {
                let dir = self.shared.store.version_dir(&self.shared.versions[self.shared.selected]);
                let _ = std::fs::create_dir_all(&dir);
                launch::open_folder(&dir);
            }
        }
    }

    fn play(&mut self) {
        let i = self.shared.selected;
        self.refresh(i);
        let v = &self.shared.versions[i];
        if !v.playable {
            let body = if v.status.is_empty() { format!("The port cannot run {} yet.", v.title) } else { v.status.clone() };
            self.message("Not playable yet", &body);
        } else if self.shared.slots[i].game.is_none() {
            self.shared.check = Check::None;
            self.open(Kind::Setup(Focus::default()));
        } else {
            match launch::start(&self.shared.store, v) {
                Ok(child) => self.launch = Some(Launch { child, since: Instant::now(), from: self.play_at }),
                Err(e) => self.message("The game could not start", &e),
            }
        }
    }

    /// The game is starting: the accent colour floods out from Play, then
    /// the launcher closes; if the game stops at once, it says why instead.
    fn draw_launch(&mut self, ctx: &egui::Context, p: &egui::Painter, screen: Rect, pal: Palette) {
        let Some(launch) = &mut self.launch else { return };
        let age = launch.since.elapsed().as_secs_f32();
        let reach = screen.size().length();
        p.circle_filled(launch.from, reach * ease_out(age / 0.60), pal.accent[1]);
        p.circle_filled(launch.from, reach * ease_out((age - 0.10) / 0.70), pal.accent[0]);
        let shown = smooth((age - 0.25) / 0.35);
        let title = &self.shared.versions[self.shared.selected].title;
        spaced(p, screen.center() - vec2(0.0, 34.0), Align2::CENTER_CENTER, "STARTING", 16.0, BOLD, with_alpha(pal.ink, 0.75 * shown), 4.0);
        text(p, screen.center() + vec2(0.0, 8.0), Align2::CENTER_CENTER, title, 40.0, BLACK, with_alpha(pal.ink, shown));
        for k in 0..3 {
            let beat = (age * 5.0 - k as f32 * 0.6).sin().max(0.0);
            p.circle_filled(screen.center() + vec2((k as f32 - 1.0) * 22.0, 62.0 - 8.0 * beat), 5.0, with_alpha(pal.ink, shown * (0.45 + 0.55 * beat)));
        }
        match launch.child.try_wait() {
            Ok(Some(status)) => {
                let (dir, code) = (self.shared.store.version_dir(&self.shared.versions[self.shared.selected]), status.code());
                self.launch = None;
                if !status.success() {
                    let tail = launch::log_tail(&dir);
                    let code = code.map_or(String::new(), |c| format!(" (exit code {c})"));
                    let said = if tail.is_empty() { String::new() } else { format!(" It said:\n{tail}") };
                    self.message("The game stopped right away", &format!("{} closed as soon as it started{code}.{said}", self.shared.store.game_exe().display()));
                }
            }
            _ if age > 1.7 => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            _ => {}
        }
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.03, 0.04, 0.09, 1.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.frame += 1;

        // One layout for every window size: the picture is 720 points tall
        // (or 1280 wide when the window is narrower than 16:9), and the
        // points grow with the window.
        let ppp = ctx.pixels_per_point();
        let pixels = ctx.content_rect().size() * ppp;
        let fit = (pixels.x / 1280.0).min(pixels.y / 720.0).max(0.3);
        if (fit - ppp).abs() > 0.002 {
            ctx.set_pixels_per_point(fit);
        }
        let screen = ctx.content_rect();

        let dt = if self.scripted { 1.0 / 60.0 } else { ctx.input(|i| i.stable_dt).min(0.1) };
        self.time += dt;
        self.poll_jobs(&ctx);

        let nav = self.input.poll(&ctx, self.frame, dt);
        self.shared.pad = self.input.pad.clone();
        let (pointer, moved, clicked, wheel, fullscreen_key, dropped) = ctx.input(|i| {
            let dropped = i.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).next();
            (i.pointer.hover_pos(), i.pointer.delta() != egui::Vec2::ZERO, i.pointer.primary_clicked(), i.smooth_scroll_delta.y, i.key_pressed(egui::Key::F11), dropped)
        });
        if fullscreen_key {
            let now = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!now));
        }
        if let Some(path) = dropped.filter(|_| self.launch.is_none()) {
            // A folder dropped on the window is a picked game folder.
            self.pick_folder(if path.is_dir() { path } else { path.parent().map(PathBuf::from).unwrap_or(path) });
        }

        // The selected version's picture and colours, crossing over from
        // the previous version's after a switch.
        self.switched = (self.switched + dt / 0.55).min(1.0);
        let cross = smooth(self.switched);
        let (now, before) = (&self.shared.versions[self.shared.selected].theme, &self.shared.versions[self.previous].theme);
        let pal = Palette { accent: [scene::mix(before.accent[0].0, now.accent[0].0, cross), scene::mix(before.accent[1].0, now.accent[1].0, cross)], ink: scene::mix(before.ink.0, now.ink.0, cross) };
        let painter = ui.painter().clone();
        let lean = pointer.map_or(0.0, |p| ((p.x - screen.center().x) / screen.width()).clamp(-0.5, 0.5));
        let shift = self.anim.to("lean", lean, 3.0, dt);
        if cross < 1.0 {
            scene::paint(&before.scene, &painter, screen, self.time, 1.0, shift, self.soft.id());
        }
        scene::paint(&now.scene, &painter, screen, self.time, cross, shift, self.soft.id());
        // A little shade behind the words on the left and along the bottom.
        let mut shade = egui::Mesh::default();
        for (x, a) in [(screen.left(), 0.34), (screen.left() + screen.width() * 0.62, 0.0)] {
            shade.colored_vertex(pos2(x, screen.top()), black(a));
            shade.colored_vertex(pos2(x, screen.bottom()), black(a));
        }
        shade.add_triangle(0, 2, 1);
        shade.add_triangle(1, 2, 3);
        painter.add(egui::Shape::mesh(shade));

        let top = self.layers.iter().rposition(|l| !l.closing);
        let busy = self.launch.is_some();
        let mut actions = Vec::new();
        let (ramp, device, time) = (self.ramp.id(), self.input.device, self.time);
        {
            let active = top.is_none() && !busy;
            let mut cx = Cx { p: painter.clone(), screen, time, dt, active, nav: if active { nav.clone() } else { Vec::new() }, pointer, moved, clicked: clicked && active, wheel, anim: &mut self.anim, pal, device, ramp };
            let (asked, play_at) = screen_main::draw(&self.shared, &mut cx, &mut self.main_focus, self.switched);
            actions.extend(asked);
            self.play_at = play_at;
        }

        for i in 0..self.layers.len() {
            let layer = &mut self.layers[i];
            layer.t = if layer.closing { (layer.t - dt / 0.16).max(0.0) } else { (layer.t + dt / 0.24).min(1.0) };
            let shown = smooth(layer.t);
            painter.rect_filled(screen, 0.0, Color32::from_rgba_unmultiplied(6, 8, 20, (shown * 178.0) as u8));
            let active = top == Some(i) && !busy;
            let mut p = painter.clone();
            p.multiply_opacity(shown);
            let mut cx = Cx { p, screen, time, dt, active, nav: if active { nav.clone() } else { Vec::new() }, pointer, moved, clicked: clicked && active, wheel, anim: &mut self.anim, pal, device, ramp };
            let t = layer.t;
            actions.extend(match &mut layer.kind {
                Kind::Switcher(state) => screen_switch::draw(&self.shared, &mut cx, state, t),
                Kind::Setup(focus) => screen_setup::draw(&self.shared, &mut cx, focus, t),
                Kind::Browser(browser) => screen_setup::browse(&self.shared, &mut cx, browser, t),
                Kind::Settings(state) => screen_settings::draw(&self.shared, &mut cx, state, t),
                Kind::Dialog(dialog) => dialogs::draw(&self.shared, &mut cx, dialog, t),
            });
        }
        self.layers.retain(|l| !(l.closing && l.t <= 0.0));

        self.draw_launch(&ctx, &painter, screen, pal);
        for action in actions {
            self.apply(action, frame);
        }

        // Scripted runs: screenshots at given frames, then the end.
        let frame_no = self.frame;
        for (_, path) in self.shots.iter().filter(|s| s.0 == frame_no) {
            self.shots_wanted.push_back(path.clone());
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        }
        let taken: Vec<_> = ctx.input(|i| i.events.iter().filter_map(|e| if let egui::Event::Screenshot { image, .. } = e { Some(image.clone()) } else { None }).collect());
        for image in taken {
            if let Some(path) = self.shots_wanted.pop_front() {
                if let Err(e) = save_png(&path, &image) {
                    eprintln!("{}: {e}", path.display());
                }
            }
        }
        if self.frames.is_some_and(|n| frame_no >= n) && self.shots_wanted.is_empty() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // The picture is always moving, and controllers are polled each
        // frame; out of focus a few frames a second are enough.
        if self.scripted || ctx.input(|i| i.focused) {
            ctx.request_repaint();
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(120));
        }
    }
}

fn save_png(path: &std::path::Path, image: &ColorImage) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), image.size[0] as u32, image.size[1] as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    let bytes: Vec<u8> = image.pixels.iter().flat_map(|c| [c.r(), c.g(), c.b(), 255]).collect();
    writer.write_image_data(&bytes).map_err(|e| e.to_string())
}
