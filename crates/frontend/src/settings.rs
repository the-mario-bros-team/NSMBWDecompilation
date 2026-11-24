//! Display settings, kept in `settings.toml` next to `port.toml` and changed
//! with hotkeys while playing. Modelled on WitherBlitz's
//! NSMB-Wii-Recompilation display settings (frame interpolation, V-Sync,
//! fullscreen, resolution, aspect ratio, performance monitor, battery saver).

use std::path::Path;

use input::keymap::{Keymap, ACTIONS};
use input::padmap::Padmap;

/// Frame-rate choices: 60 is the game's own rate; 30 shows every other
/// field (as the battery saver), the game still running at 60; the others
/// draw interpolated frames in between; 0 is unlocked.
pub const FPS_CHOICES: [u32; 12] = [30, 60, 90, 120, 144, 165, 180, 240, 300, 360, 400, 0];

pub const FILE: &str = "settings.toml";

/// What the widescreen picture is (after WitherBlitz's port's aspect modes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WideMode {
    /// The game's 16:9, with bars on a screen of another shape.
    Sixteen9,
    /// A 16:10 picture, the game's view a little narrower.
    Sixteen10,
    /// Ultrawide: on a screen wider than 16:9 the picture fills it, with a
    /// wider view (Hor+); otherwise 16:9.
    Fill,
}

/// The graphics API the game draws with (applies at the next start).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendererChoice {
    /// Vulkan where there is a Vulkan 1.3 device with what the renderer
    /// needs, else Direct3D 12, else Direct3D 11 (`crate::gfx`).
    Auto,
    Vulkan,
    D3d12,
    D3d11,
}

impl RendererChoice {
    /// The choices the Settings window cycles through on this platform.
    pub const ALL: &'static [RendererChoice] = if cfg!(windows) { &[Self::Auto, Self::D3d12, Self::Vulkan, Self::D3d11] } else { &[Self::Auto, Self::Vulkan] };

    /// As `settings.toml` and `--renderer` spell it.
    pub fn key(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Vulkan => "vulkan",
            Self::D3d12 => "d3d12",
            Self::D3d11 => "d3d11",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "auto" | "automatic" => Self::Auto,
            "vulkan" | "vk" => Self::Vulkan,
            "d3d12" | "dx12" => Self::D3d12,
            "d3d11" | "dx11" => Self::D3d11,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "Automatic",
            Self::Vulkan => "Vulkan",
            Self::D3d12 => "Direct3D 12",
            Self::D3d11 => "Direct3D 11",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// The graphics API (next start).
    pub renderer: RendererChoice,
    /// One of [`FPS_CHOICES`].
    pub fps: u32,
    pub vsync: bool,
    pub fullscreen: bool,
    /// EFB resolution multiplier (480 lines each; [`SCALES`]), 0 = the
    /// window's own height. Applies at the next start.
    pub scale: f32,
    /// 16:9, else 4:3. The game reads it at boot, so a change applies at the
    /// next start; the picture's shape changes at once.
    pub widescreen: bool,
    /// The widescreen picture's shape (`aspect`); changes at once.
    pub wide_mode: WideMode,
    /// Coins, ? blocks and other animated tiles move (EFB copies patched
    /// into their textures); off they stand still, for slow machines.
    pub spinning_coins: bool,
    /// The depth blur and heat haze of desert, beach, water, lava and
    /// Bowser areas.
    pub depth_blur: bool,
    /// FPS counter and frame-time graph.
    pub perf_monitor: bool,
    /// Show 30 frames a second while the game keeps running at 60.
    pub battery_saver: bool,
    /// Also write EFB-to-texture copies into guest RAM (slower; off keeps
    /// them on the GPU, as Dolphin does by default).
    pub efb_copies_to_ram: bool,
    /// The performance monitor's size, 5..200 percent.
    pub perf_size: u32,
    /// Volumes in percent: master, music, sound effects, menu sounds.
    pub volume: [u32; 4],
    /// The game's language (the console's, which it reads at boot): "en",
    /// "es" or "fr".
    pub language: String,
    /// Gameplay: the course timer stands still.
    pub no_time_limit: bool,
    /// Gameplay: a miss costs no life.
    pub infinite_lives: bool,
    /// Gameplay: the power-up last picked to give player 1 (`gameplay::POWER_UPS`).
    pub power_up: u32,
    /// Save files 1-3 marked "Cheated": Infinite Lives or the Power-Up was
    /// used on them.
    pub cheated: [bool; 3],
    /// Gameplay: the game is saved after every course cleared.
    pub autosave: bool,
    /// Gameplay: the pause window's Exit Course in every course, cleared or
    /// not (in a net game only if every player has it).
    pub exit_course: bool,
    /// Gameplay: player 1 picks a character too, on the game's own screen
    /// (applies from the next start; never in scripted, headless or net
    /// runs).
    pub p1_character: bool,
    /// Player 1's last character on each save file (0 Mario, 1 Luigi,
    /// 2 Blue Toad, 3 Yellow Toad), where the character screen starts.
    pub file_characters: [u32; 3],
    /// Gameplay: everything stops for a moment while a player grows,
    /// shrinks, changes power-up or dies (the game's own rule; off, the
    /// level keeps moving). Net games keep the game's rules.
    pub power_up_freeze: bool,
    /// Gameplay: players bump into and stand on each other (the game's own
    /// rule; off, they pass through each other). Net games keep the game's
    /// rules.
    pub player_collision: bool,
    /// The keys for the remote's buttons (`[keyboard]`).
    pub keymap: Keymap,
    // The controls (the Keybinds tab), after WitherBlitz's
    // NSMB-Wii-Recompilation; in `[keyboard]` and `[controller]`.
    /// The controllers' buttons for the remote's (`[controller]`).
    pub padmap: Padmap,
    /// `[controller] rumble`: controllers rumble when the game rumbles the remote.
    pub rumble: bool,
    /// `[keyboard] smart_spin`: with Space as a Jump key, Space in the air spins.
    pub smart_spin: bool,
    /// `[keyboard] mouse_gyro`: the mouse tilts and shakes the remote while
    /// no controller is connected.
    pub mouse_gyro: bool,
    /// `[keyboard] reverse_tilt`: every remote's tilt mirrored, so platforms
    /// lean the way the keys or stick go.
    pub reverse_tilt: bool,
    /// `[controller] wii_remotes`: Wii Remotes paired with Windows play in
    /// place of players' emulated remotes (Windows only).
    pub wii_remotes: bool,
    /// `[controller] motion_sensors`: a controller's own motion sensors
    /// tilt (turned like a steering wheel) and shake (a jolt) its remote.
    pub motion_sensors: bool,
    /// `[controller] bt_passthrough` (experimental): a USB Bluetooth adapter
    /// on the WinUSB driver runs real Wii Remotes through the game's own
    /// Bluetooth stack (from the next start; Windows only).
    pub bt_passthrough: bool,
    /// Mods tab (`crate::mods`): HD textures from a texture pack.
    pub hd_textures: bool,
    /// The pack's folder under `texture_packs/` ("": the first there).
    pub texture_pack: String,
    /// Write the textures no pack replaces to `texture_dumps/`.
    pub texture_dumps: bool,
    /// The file mods under `mods/` turned on (from the next start).
    pub mods: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { renderer: RendererChoice::Auto, fps: 60, vsync: false, fullscreen: false, scale: 2.0, widescreen: true, wide_mode: WideMode::Fill, spinning_coins: true, depth_blur: true, perf_monitor: false, battery_saver: false, efb_copies_to_ram: false, perf_size: 50, volume: [100; 4], language: "en".into(), no_time_limit: false, infinite_lives: false, power_up: 1, cheated: [false; 3], autosave: true, exit_course: true, p1_character: true, file_characters: [0; 3], power_up_freeze: true, player_collision: true, keymap: Keymap::default(), padmap: Padmap::default(), rumble: true, smart_spin: true, mouse_gyro: false, reverse_tilt: true, wii_remotes: false, motion_sensors: true, bt_passthrough: false, hd_textures: false, texture_pack: String::new(), texture_dumps: false, mods: Vec::new() }
    }
}

/// Resolution choices (times 480 lines), then 0 for the window's own.
pub const SCALES: [f32; 10] = [0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0, 8.0, 0.0];

/// A resolution multiplier from a file or the command line: 0.5 to 8.
pub fn clamp_scale(s: f32) -> f32 {
    if s.is_finite() {
        s.clamp(0.5, 8.0)
    } else {
        2.0
    }
}

/// A table's `<action>_combos`: lists of names.
fn combo_names(table: &toml::Table, action: &str) -> Vec<Vec<String>> {
    let Some(list) = table.get(&format!("{action}_combos")).and_then(toml::Value::as_array) else { return Vec::new() };
    list.iter().filter_map(toml::Value::as_array).map(|c| c.iter().filter_map(toml::Value::as_str).map(String::from).collect()).collect()
}

/// `<action>_combos = [["Left Ctrl", "X"]]`, or nothing for none.
fn combo_line(action: &str, combos: Vec<Vec<&str>>) -> String {
    if combos.is_empty() {
        return String::new();
    }
    let list = toml::Value::Array(combos.into_iter().map(|c| toml::Value::Array(c.into_iter().map(|n| toml::Value::String(n.into())).collect())).collect());
    format!("{action}_combos = {list}\n")
}

pub fn fps_label(fps: u32) -> String {
    match fps {
        0 => "UNLOCKED".into(),
        30 => "30 (HALF)".into(),
        60 => "60 (NATIVE)".into(),
        n => format!("{n}"),
    }
}

/// Parses `30`..`400` or `unlocked` into one of [`FPS_CHOICES`].
pub fn parse_fps(s: &str) -> Option<u32> {
    let v = if s.eq_ignore_ascii_case("unlocked") { 0 } else { s.parse().ok()? };
    FPS_CHOICES.contains(&v).then_some(v)
}

impl Settings {
    /// The file's settings, defaults for anything missing or unreadable.
    pub fn load(path: &Path) -> Self {
        let mut s = Self::default();
        let Ok(text) = std::fs::read_to_string(path) else { return s };
        let Ok(t) = text.parse::<toml::Table>() else {
            eprintln!("{}: not valid TOML, using defaults", path.display());
            return s;
        };
        let flag = |k: &str, d: bool| t.get(k).and_then(toml::Value::as_bool).unwrap_or(d);
        if let Some(v) = t.get("fps") {
            let text = v.as_str().map(str::to_string).or_else(|| v.as_integer().map(|n| n.to_string())).unwrap_or_default();
            s.fps = parse_fps(&text).unwrap_or(s.fps);
        }
        if let Some(r) = t.get("renderer").and_then(toml::Value::as_str).and_then(RendererChoice::parse) {
            s.renderer = r;
        }
        s.vsync = flag("vsync", s.vsync);
        s.fullscreen = flag("fullscreen", s.fullscreen);
        if let Some(a) = t.get("aspect").and_then(toml::Value::as_str) {
            s.widescreen = a != "4:3";
            match a {
                "16:9" => s.wide_mode = WideMode::Sixteen9,
                "16:10" => s.wide_mode = WideMode::Sixteen10,
                "ultrawide" => s.wide_mode = WideMode::Fill,
                _ => {}
            }
        }
        s.spinning_coins = flag("spinning_coins", s.spinning_coins);
        s.depth_blur = flag("depth_blur", s.depth_blur);
        s.perf_monitor = flag("perf_monitor", s.perf_monitor);
        s.battery_saver = flag("battery_saver", s.battery_saver);
        s.efb_copies_to_ram = flag("efb_copies_to_ram", s.efb_copies_to_ram);
        match t.get("scale") {
            Some(toml::Value::String(v)) if v == "native" => s.scale = 0.0,
            Some(toml::Value::Integer(n)) => s.scale = clamp_scale(*n as f32),
            Some(toml::Value::Float(f)) => s.scale = clamp_scale(*f as f32),
            _ => {}
        }
        let number = |k: &str| t.get(k).and_then(toml::Value::as_integer);
        if let Some(n) = number("perf_size") {
            s.perf_size = n.clamp(5, 200) as u32;
        }
        for (i, k) in ["volume", "music_volume", "sound_volume", "menu_volume"].into_iter().enumerate() {
            if let Some(n) = number(k) {
                s.volume[i] = n.clamp(0, 100) as u32;
            }
        }
        if let Some(l) = t.get("language").and_then(toml::Value::as_str).filter(|l| ["en", "es", "fr"].contains(l)) {
            s.language = l.into();
        }
        s.no_time_limit = flag("no_time_limit", s.no_time_limit);
        s.infinite_lives = flag("infinite_lives", s.infinite_lives);
        if let Some(n) = number("power_up") {
            s.power_up = n.clamp(0, crate::gameplay::POWER_UPS.len() as i64 - 1) as u32;
        }
        if let Some(marks) = t.get("cheated").and_then(toml::Value::as_array) {
            for (mark, v) in s.cheated.iter_mut().zip(marks) {
                *mark = v.as_bool().unwrap_or(false);
            }
        }
        s.autosave = flag("autosave", s.autosave);
        s.exit_course = flag("exit_course", s.exit_course);
        s.p1_character = flag("p1_character", s.p1_character);
        for (i, c) in s.file_characters.iter_mut().enumerate() {
            if let Some(n) = number(&format!("file{}_character", i + 1)) {
                *c = n.clamp(0, 3) as u32;
            }
        }
        s.power_up_freeze = flag("power_up_freeze", s.power_up_freeze);
        s.player_collision = flag("player_collision", s.player_collision);
        s.hd_textures = flag("hd_textures", s.hd_textures);
        if let Some(p) = t.get("texture_pack").and_then(toml::Value::as_str) {
            s.texture_pack = p.into();
        }
        s.texture_dumps = flag("texture_dumps", s.texture_dumps);
        if let Some(mods) = t.get("mods").and_then(toml::Value::as_array) {
            s.mods = mods.iter().filter_map(toml::Value::as_str).map(str::to_string).collect();
        }
        if let Some(keyboard) = t.get("keyboard").and_then(toml::Value::as_table) {
            for (i, (key, _)) in ACTIONS.iter().enumerate() {
                if let Some(label) = keyboard.get(*key).and_then(toml::Value::as_str) {
                    s.keymap.set_label(i, label);
                }
                s.keymap.set_combo_names(i, &combo_names(keyboard, key));
            }
            let flag = |k: &str, d: bool| keyboard.get(k).and_then(toml::Value::as_bool).unwrap_or(d);
            s.smart_spin = flag("smart_spin", s.smart_spin);
            s.mouse_gyro = flag("mouse_gyro", s.mouse_gyro);
            s.reverse_tilt = flag("reverse_tilt", s.reverse_tilt);
        }
        if let Some(controller) = t.get("controller").and_then(toml::Value::as_table) {
            let flag = |k: &str, d: bool| controller.get(k).and_then(toml::Value::as_bool).unwrap_or(d);
            s.rumble = flag("rumble", s.rumble);
            s.wii_remotes = flag("wii_remotes", s.wii_remotes);
            s.motion_sensors = flag("motion_sensors", s.motion_sensors);
            s.bt_passthrough = flag("bt_passthrough", s.bt_passthrough);
            for (i, (key, _)) in ACTIONS.iter().enumerate() {
                if let Some(label) = controller.get(*key).and_then(toml::Value::as_str) {
                    s.padmap.set_label(i, label);
                }
                s.padmap.set_combo_names(i, &combo_names(controller, key));
            }
        }
        s
    }

    pub fn save(&self, path: &Path) {
        let fps = if self.fps == 0 { "\"unlocked\"".to_string() } else { self.fps.to_string() };
        let scale = if self.scale == 0.0 { "\"native\"".to_string() } else { self.scale.to_string() };
        let text = format!(
            "# NSMBW port display settings (hotkeys change them while playing; see F1).\n\
             # fps: 30, 60, 90, 120, 144, 165, 180, 240, 300, 360 or \"unlocked\"; 30 shows every other field; above 60 draws interpolated frames.\n\
             # Renderer: \"auto\" (Vulkan, else Direct3D 12, else Direct3D 11), \"vulkan\", \"d3d12\" or \"d3d11\" (next start).\n\
             renderer = \"{}\"\n\
             fps = {fps}\nvsync = {}\nfullscreen = {}\n\
             # EFB resolution: 0.5..8 times 480 lines (0.5, 0.75, 1, 1.5, 2, 3, 4, 6 or 8), or \"native\" for the window's height (next start).\n\
             scale = {scale}\n\
             # \"4:3\", \"16:9\", \"16:10\" or \"ultrawide\" (16:9 on a screen no wider); 4:3 or not is read at boot.\n\
             aspect = \"{}\"\nperf_monitor = {}\nbattery_saver = {}\n\
             # Coins and blocks animate; the depth blur and heat haze of some areas.\n\
             spinning_coins = {}\ndepth_blur = {}\n\
             # Also write EFB copies into guest RAM (slower; NSMBW does not need it).\n\
             efb_copies_to_ram = {}\n\
             # The performance monitor's size (5..200 percent).\n\
             perf_size = {}\n\
             # Volumes in percent.\n\
             volume = {}\nmusic_volume = {}\nsound_volume = {}\nmenu_volume = {}\n\
             # The game's language: \"en\", \"es\" or \"fr\" (the game reads it at boot).\n\
             language = \"{}\"\n\
             # Gameplay options (the Settings window's Gameplay tab).\n\
             no_time_limit = {}\ninfinite_lives = {}\npower_up = {}\n\
             # Save files 1-3 marked \"Cheated\" (Infinite Lives or the Power-Up was used on them).\n\
             cheated = [{}, {}, {}]\n\
             # Save after every course cleared; Exit Course in every course (net games: if every player has it).\n\
             autosave = {}\nexit_course = {}\n\
             # Player 1 picks a character too (from the next start), and each save file's last one\n\
             # (0 Mario, 1 Luigi, 2 Blue Toad, 3 Yellow Toad).\n\
             p1_character = {}\nfile1_character = {}\nfile2_character = {}\nfile3_character = {}\n\
             # The game's own rules (net games keep them): everything stops while a player changes size or dies;\n\
             # players bump into and stand on each other. false: the level keeps going; players pass through each other.\n\
             power_up_freeze = {}\nplayer_collision = {}\n",
            self.renderer.key(),
            self.vsync,
            self.fullscreen,
            self.aspect_name(),
            self.perf_monitor,
            self.battery_saver,
            self.spinning_coins,
            self.depth_blur,
            self.efb_copies_to_ram,
            self.perf_size,
            self.volume[0],
            self.volume[1],
            self.volume[2],
            self.volume[3],
            self.language,
            self.no_time_limit,
            self.infinite_lives,
            self.power_up,
            self.cheated[0],
            self.cheated[1],
            self.cheated[2],
            self.autosave,
            self.exit_course,
            self.p1_character,
            self.file_characters[0],
            self.file_characters[1],
            self.file_characters[2],
            self.power_up_freeze,
            self.player_collision
        );
        let mut text = text;
        let mods: Vec<String> = self.mods.iter().map(|m| toml::Value::String(m.clone()).to_string()).collect();
        text.push_str(&format!(
            "# Mods tab. HD textures from a Dolphin texture pack in texture_packs/<folder> (texture_pack: its folder, \"\" the first);\n\
             # texture_dumps writes the textures no pack replaces to texture_dumps/; mods: the folders in mods/ turned on (next start).\n\
             hd_textures = {}\ntexture_pack = {}\ntexture_dumps = {}\nmods = [{}]\n",
            self.hd_textures,
            toml::Value::String(self.texture_pack.clone()),
            self.texture_dumps,
            mods.join(", ")
        ));
        // Last: a table ends the file's top-level keys.
        text.push_str("\n# Keys for the Wii Remote's buttons, by name, separated by commas (\"\" for none).\n# <action>_combos: keys held together, 2 or 3 a combination: jump_combos = [[\"Left Ctrl\", \"X\"]].\n[keyboard]\n");
        for (i, (key, _)) in ACTIONS.iter().enumerate() {
            text.push_str(&format!("{key} = {}\n", toml::Value::String(self.keymap.label(i))));
            text.push_str(&combo_line(key, self.keymap.combo_names(i)));
        }
        text.push_str("# Space Smart Spin: with Space as a Jump key, Space in the air spins.\n");
        text.push_str(&format!("smart_spin = {}\n", self.smart_spin));
        text.push_str("# Mouse as Gyro: the mouse tilts the remote (a quick swing back shakes it) while no controller is connected.\n");
        text.push_str(&format!("mouse_gyro = {}\n", self.mouse_gyro));
        text.push_str("# Reverse Tilt: platforms lean the way the keys or stick go.\n");
        text.push_str(&format!("reverse_tilt = {}\n", self.reverse_tilt));
        text.push_str("\n# Controller buttons for the Wii Remote's, by their Xbox names (A, B, X, Y, LB, RB, LT, RT,\n# View, Menu, LS, RS, D-Pad Up, Left Stick Up, Right Stick Left...), separated by commas.\n# rumble: controllers rumble when the game rumbles the Wii Remote.\n[controller]\n");
        text.push_str(&format!("rumble = {}\n", self.rumble));
        text.push_str("# Wii Remotes paired with Windows (Settings > Bluetooth) play as players 1-4 in turn.\n");
        text.push_str(&format!("wii_remotes = {}\n", self.wii_remotes));
        text.push_str("# Motion Sensors: a controller's own sensors tilt (turn it like a steering wheel) and shake (jolt it).\n");
        text.push_str(&format!("motion_sensors = {}\n", self.motion_sensors));
        text.push_str("# Bluetooth passthrough (experimental, from the next start): a USB Bluetooth adapter on the WinUSB\n# driver (Zadig) runs real Wii Remotes through the game's own Bluetooth. F8 pairs them.\n");
        text.push_str(&format!("bt_passthrough = {}\n", self.bt_passthrough));
        for (i, (key, _)) in ACTIONS.iter().enumerate() {
            text.push_str(&format!("{key} = {}\n", toml::Value::String(self.padmap.label(i))));
            text.push_str(&combo_line(key, self.padmap.combo_names(i)));
        }
        if let Err(e) = std::fs::write(path, text) {
            eprintln!("{}: {e}", path.display());
        }
    }

    /// The `aspect` key's value.
    pub fn aspect_name(&self) -> &'static str {
        match (self.widescreen, self.wide_mode) {
            (false, _) => "4:3",
            (true, WideMode::Sixteen9) => "16:9",
            (true, WideMode::Sixteen10) => "16:10",
            (true, WideMode::Fill) => "ultrawide",
        }
    }

    /// The next (`up`) or previous frame-rate choice.
    pub fn step_fps(&mut self, up: bool) {
        let i = FPS_CHOICES.iter().position(|&f| f == self.fps).unwrap_or(0);
        let n = FPS_CHOICES.len();
        self.fps = FPS_CHOICES[if up { (i + 1) % n } else { (i + n - 1) % n }];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip() {
        let dir = std::env::temp_dir().join(format!("nsmbw-settings-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(FILE);
        let s = Settings { renderer: RendererChoice::D3d11, fps: 0, vsync: true, fullscreen: true, scale: 0.0, widescreen: false, wide_mode: WideMode::Fill, spinning_coins: false, depth_blur: false, perf_monitor: true, battery_saver: true, efb_copies_to_ram: true, perf_size: 120, volume: [80, 70, 60, 50], language: "fr".into(), no_time_limit: true, infinite_lives: true, power_up: 7, cheated: [true, false, true], autosave: false, exit_course: false, p1_character: false, file_characters: [1, 3, 2], power_up_freeze: false, player_collision: false, keymap: {
            let mut k = Keymap::default();
            k.bind(input::keymap::JUMP, winit::keyboard::KeyCode::Backslash);
            k.bind(input::keymap::RUN, input::keymap::Key::Mouse(5));
            k.take(input::keymap::A, vec![winit::keyboard::KeyCode::ControlLeft.into(), winit::keyboard::KeyCode::KeyX.into()]);
            k.take(input::keymap::A, vec![winit::keyboard::KeyCode::ShiftLeft.into(), input::keymap::Key::Mouse(1), winit::keyboard::KeyCode::KeyQ.into()]);
            k.keys[input::keymap::SHAKE].clear();
            k
        }, padmap: {
            let mut p = Padmap::default();
            p.bind(input::keymap::A, input::padmap::by_name("RS").unwrap());
            p.buttons[input::keymap::TILT_LEFT].clear();
            p.take(input::keymap::PLUS, vec![input::padmap::by_name("LB").unwrap(), input::padmap::by_name("RB").unwrap()]);
            p
        }, rumble: false, smart_spin: false, mouse_gyro: true, reverse_tilt: false, wii_remotes: true, motion_sensors: false, bt_passthrough: true, hd_textures: true, texture_pack: "My \"Pack\"".into(), texture_dumps: true, mods: vec!["One".into(), "Two \\ Three".into()] };
        s.save(&path);
        assert_eq!(Settings::load(&path), s);
        let s = Settings { fps: 144, scale: 3.0, ..Settings::default() };
        s.save(&path);
        assert_eq!(Settings::load(&path), s);
        let s = Settings { scale: 0.75, ..Settings::default() };
        s.save(&path);
        assert_eq!(Settings::load(&path), s);
        for wide_mode in [WideMode::Sixteen9, WideMode::Sixteen10, WideMode::Fill] {
            let s = Settings { wide_mode, ..Settings::default() };
            s.save(&path);
            assert_eq!(Settings::load(&path), s);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn fps_steps_wrap() {
        let mut s = Settings::default();
        s.step_fps(false);
        assert_eq!(s.fps, 30);
        s.step_fps(false);
        assert_eq!(s.fps, 0);
        s.step_fps(true);
        s.step_fps(true);
        s.step_fps(true);
        assert_eq!(s.fps, 90);
        assert_eq!(parse_fps("Unlocked"), Some(0));
        assert_eq!(parse_fps("100"), None);
    }
}
