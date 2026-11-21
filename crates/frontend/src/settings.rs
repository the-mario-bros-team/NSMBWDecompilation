//! Display settings, kept in `settings.toml` next to `port.toml` and changed
//! with hotkeys while playing. Modelled on WitherBlitz's
//! NSMB-Wii-Recompilation display settings (frame interpolation, V-Sync,
//! fullscreen, resolution, aspect ratio, performance monitor, battery saver).

use std::path::Path;

use input::keymap::{Keymap, ACTIONS};

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

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
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
    /// The keys for the remote's buttons (`[keyboard]`).
    pub keymap: Keymap,
}

impl Default for Settings {
    fn default() -> Self {
        Self { fps: 60, vsync: false, fullscreen: false, scale: 2.0, widescreen: true, wide_mode: WideMode::Fill, spinning_coins: true, depth_blur: true, perf_monitor: false, battery_saver: false, efb_copies_to_ram: false, perf_size: 50, volume: [100; 4], language: "en".into(), no_time_limit: false, infinite_lives: false, power_up: 1, cheated: [false; 3], keymap: Keymap::default() }
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
        if let Some(keyboard) = t.get("keyboard").and_then(toml::Value::as_table) {
            for (i, (key, _)) in ACTIONS.iter().enumerate() {
                if let Some(label) = keyboard.get(*key).and_then(toml::Value::as_str) {
                    s.keymap.set_label(i, label);
                }
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
             cheated = [{}, {}, {}]\n",
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
            self.cheated[2]
        );
        // Last: a table ends the file's top-level keys.
        let mut text = text;
        text.push_str("\n# Keys for the Wii Remote's buttons, by name, separated by commas (\"\" for none).\n[keyboard]\n");
        for (i, (key, _)) in ACTIONS.iter().enumerate() {
            text.push_str(&format!("{key} = {}\n", toml::Value::String(self.keymap.label(i))));
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
        let s = Settings { fps: 0, vsync: true, fullscreen: true, scale: 0.0, widescreen: false, wide_mode: WideMode::Fill, spinning_coins: false, depth_blur: false, perf_monitor: true, battery_saver: true, efb_copies_to_ram: true, perf_size: 120, volume: [80, 70, 60, 50], language: "fr".into(), no_time_limit: true, infinite_lives: true, power_up: 7, cheated: [true, false, true], keymap: {
            let mut k = Keymap::default();
            k.bind(input::keymap::JUMP, winit::keyboard::KeyCode::Backslash);
            k.keys[input::keymap::SHAKE].clear();
            k
        } };
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
