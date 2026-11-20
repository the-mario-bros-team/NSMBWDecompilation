//! Display settings, kept in `settings.toml` next to `port.toml` and changed
//! with hotkeys while playing. Modelled on WitherBlitz's
//! NSMB-Wii-Recompilation display settings (frame interpolation, V-Sync,
//! fullscreen, resolution, aspect ratio, performance monitor, battery saver).

use std::path::Path;

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
}

impl Default for Settings {
    fn default() -> Self {
        Self { fps: 60, vsync: false, fullscreen: false, scale: 2, widescreen: true, perf_monitor: false, battery_saver: false, efb_copies_to_ram: false }
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
        s.widescreen = t.get("aspect").and_then(toml::Value::as_str).map_or(s.widescreen, |a| a != "4:3");
        s.perf_monitor = flag("perf_monitor", s.perf_monitor);
        s.battery_saver = flag("battery_saver", s.battery_saver);
        s.efb_copies_to_ram = flag("efb_copies_to_ram", s.efb_copies_to_ram);
        match t.get("scale") {
            Some(toml::Value::String(v)) if v == "native" => s.scale = 0,
            Some(toml::Value::Integer(n)) => s.scale = (*n).clamp(1, 8) as u32,
            _ => {}
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
             efb_copies_to_ram = {}\n",
            self.vsync,
            self.fullscreen,
            if self.widescreen { "16:9" } else { "4:3" },
            self.perf_monitor,
            self.battery_saver,
            self.efb_copies_to_ram
        );
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
        let s = Settings { fps: 0, vsync: true, fullscreen: true, scale: 0, widescreen: false, perf_monitor: true, battery_saver: true, efb_copies_to_ram: true };
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
