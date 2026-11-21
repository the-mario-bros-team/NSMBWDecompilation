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

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// One of [`FPS_CHOICES`].
    pub fps: u32,
    pub vsync: bool,
    pub fullscreen: bool,
    /// EFB resolution multiplier (480 lines each); 0 = the window's own
    /// height. Applies at the next start.
    pub scale: u32,
    /// 16:9, else 4:3. The game reads it at boot, so a change applies at the
    /// next start; the picture's shape changes at once.
    pub widescreen: bool,
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
    // The controls (the Keybinds tab), after WitherBlitz's
    // NSMB-Wii-Recompilation; in `[keyboard]` and `[controller]`.
    /// The controllers' buttons for the remote's (`[controller]`).
    pub padmap: Padmap,
    /// `[controller] rumble`: controllers rumble when the game rumbles the remote.
    pub rumble: bool,
    /// `[keyboard] smart_spin`: with Space as a Jump key, Space in the air spins.
    pub smart_spin: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { fps: 60, vsync: false, fullscreen: false, scale: 2, widescreen: true, perf_monitor: false, battery_saver: false, efb_copies_to_ram: false, perf_size: 50, volume: [100; 4], language: "en".into(), no_time_limit: false, infinite_lives: false, power_up: 1, cheated: [false; 3], keymap: Keymap::default(), padmap: Padmap::default(), rumble: true, smart_spin: true }
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
        s.vsync = flag("vsync", s.vsync);
        s.fullscreen = flag("fullscreen", s.fullscreen);
        s.widescreen = t.get("aspect").and_then(toml::Value::as_str).map_or(s.widescreen, |a| a != "4:3");
        s.perf_monitor = flag("perf_monitor", s.perf_monitor);
        s.battery_saver = flag("battery_saver", s.battery_saver);
        s.efb_copies_to_ram = flag("efb_copies_to_ram", s.efb_copies_to_ram);
        match t.get("scale") {
            Some(toml::Value::String(v)) if v == "native" => s.scale = 0,
            Some(toml::Value::Integer(n)) => s.scale = (*n).clamp(1, 8) as u32,
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
                s.keymap.set_combo_names(i, &combo_names(keyboard, key));
            }
            s.smart_spin = keyboard.get("smart_spin").and_then(toml::Value::as_bool).unwrap_or(s.smart_spin);
        }
        if let Some(controller) = t.get("controller").and_then(toml::Value::as_table) {
            let flag = |k: &str, d: bool| controller.get(k).and_then(toml::Value::as_bool).unwrap_or(d);
            s.rumble = flag("rumble", s.rumble);
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
        let scale = if self.scale == 0 { "\"native\"".to_string() } else { self.scale.to_string() };
        let text = format!(
            "# NSMBW port display settings (hotkeys change them while playing; see F1).\n\
             # fps: 30, 60, 90, 120, 144, 165, 180, 240, 300, 360 or \"unlocked\"; 30 shows every other field; above 60 draws interpolated frames.\n\
             fps = {fps}\nvsync = {}\nfullscreen = {}\n\
             # EFB resolution: 1..8 times 480 lines, or \"native\" for the window's height (next start).\n\
             scale = {scale}\n\
             # \"16:9\" or \"4:3\" (the game reads it at boot).\n\
             aspect = \"{}\"\nperf_monitor = {}\nbattery_saver = {}\n\
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
            if self.widescreen { "16:9" } else { "4:3" },
            self.perf_monitor,
            self.battery_saver,
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
        text.push_str("\n# Keys for the Wii Remote's buttons, by name, separated by commas (\"\" for none).\n# <action>_combos: keys held together, 2 or 3 a combination: jump_combos = [[\"Left Ctrl\", \"X\"]].\n[keyboard]\n");
        for (i, (key, _)) in ACTIONS.iter().enumerate() {
            text.push_str(&format!("{key} = {}\n", toml::Value::String(self.keymap.label(i))));
            text.push_str(&combo_line(key, self.keymap.combo_names(i)));
        }
        text.push_str("# Space Smart Spin: with Space as a Jump key, Space in the air spins.\n");
        text.push_str(&format!("smart_spin = {}\n", self.smart_spin));
        text.push_str("\n# Controller buttons for the Wii Remote's, by their Xbox names (A, B, X, Y, LB, RB, LT, RT,\n# View, Menu, LS, RS, D-Pad Up, Left Stick Up, Right Stick Left...), separated by commas.\n# rumble: controllers rumble when the game rumbles the Wii Remote.\n[controller]\n");
        text.push_str(&format!("rumble = {}\n", self.rumble));
        for (i, (key, _)) in ACTIONS.iter().enumerate() {
            text.push_str(&format!("{key} = {}\n", toml::Value::String(self.padmap.label(i))));
            text.push_str(&combo_line(key, self.padmap.combo_names(i)));
        }
        if let Err(e) = std::fs::write(path, text) {
            eprintln!("{}: {e}", path.display());
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
        let s = Settings { fps: 0, vsync: true, fullscreen: true, scale: 0, widescreen: false, perf_monitor: true, battery_saver: true, efb_copies_to_ram: true, perf_size: 120, volume: [80, 70, 60, 50], language: "fr".into(), no_time_limit: true, infinite_lives: true, power_up: 7, cheated: [true, false, true], keymap: {
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
        }, rumble: false, smart_spin: false };
        s.save(&path);
        assert_eq!(Settings::load(&path), s);
        let s = Settings { fps: 144, scale: 3, ..Settings::default() };
        s.save(&path);
        assert_eq!(Settings::load(&path), s);
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
