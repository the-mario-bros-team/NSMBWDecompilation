//! What the launcher keeps on disk, all under its own folder (the "root",
//! where `nsmbw-launcher.exe` and `nsmbw-port.exe` are):
//!
//! - `launcher.toml`: the selected version, Skip Launcher, extra game options.
//! - one folder per version (`versions\<id>`, or the version's `data`): the
//!   game runs with it as its working folder, so its `port.toml` (the game
//!   folder), `settings.toml` and `saves\nand` are that version's alone.
//!
//! `port.toml` and `settings.toml` are the game's own files: the launcher
//! writes the game folder into the first and changes single keys of the
//! second, leaving everything else in them as it was.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::versions::Version;

pub const FILE: &str = "launcher.toml";
const DEFAULT_EXE: &str = "nsmbw-port.exe";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    /// The selected version's id.
    pub selected: String,
    /// Start the game straight away instead of showing the launcher.
    pub skip_launcher: bool,
    /// The game program, relative to the root; empty = `nsmbw-port.exe`.
    pub game_exe: String,
    /// A release found while the launcher was skipped: it shows itself once.
    pub update_pending: String,
    /// The newest release the player has been shown.
    pub update_told: String,
    /// Extra options handed to the game, per version id.
    pub options: BTreeMap<String, Vec<String>>,
}

pub struct Store {
    pub root: PathBuf,
    pub state: State,
}

impl Store {
    pub fn load(root: PathBuf) -> Self {
        let mut state = State::default();
        if let Some(t) = std::fs::read_to_string(root.join(FILE)).ok().and_then(|s| s.parse::<toml::Table>().ok()) {
            let text = |k: &str| t.get(k).and_then(toml::Value::as_str).unwrap_or("").to_string();
            state.selected = text("selected");
            state.skip_launcher = t.get("skip_launcher").and_then(toml::Value::as_bool).unwrap_or(false);
            state.game_exe = text("game_exe");
            state.update_pending = text("update_pending");
            state.update_told = text("update_told");
            if let Some(options) = t.get("options").and_then(toml::Value::as_table) {
                for (id, list) in options {
                    let args = list.as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()).unwrap_or_default();
                    state.options.insert(id.clone(), args);
                }
            }
        }
        Self { root, state }
    }

    pub fn save(&self) {
        let s = &self.state;
        let quote = |v: &str| toml::Value::String(v.to_string()).to_string();
        let mut text = format!(
            "# NSMBW port launcher.\n\
             # selected: the version last chosen (see versions.toml for the list).\n\
             selected = {}\n\
             # skip_launcher: start the game straight away. Hold Shift while starting\n\
             # (or Back on an Xbox controller) to see the launcher again.\n\
             skip_launcher = {}\n\
             # game_exe: the game program, when it is not nsmbw-port.exe in this folder.\n\
             game_exe = {}\n\
             # Kept by the launcher: a release found while it was skipped, and the\n\
             # newest release it has shown.\n\
             update_pending = {}\nupdate_told = {}\n\n\
             # Extra options handed to the game, per version: nsmbw = [\"--fullscreen\"]\n\
             # (nsmbw-port --help lists them).\n\
             [options]\n",
            quote(&s.selected),
            s.skip_launcher,
            quote(&s.game_exe),
            quote(&s.update_pending),
            quote(&s.update_told)
        );
        for (id, args) in &s.options {
            let list: Vec<String> = args.iter().map(|a| quote(a)).collect();
            text.push_str(&format!("{id} = [{}]\n", list.join(", ")));
        }
        if let Err(e) = std::fs::write(self.root.join(FILE), text) {
            eprintln!("{FILE}: {e}");
        }
    }

    /// The game program.
    pub fn game_exe(&self) -> PathBuf {
        let name = if self.state.game_exe.is_empty() { DEFAULT_EXE } else { &self.state.game_exe };
        self.root.join(name)
    }

    /// The version's working folder (its settings and saves).
    pub fn version_dir(&self, v: &Version) -> PathBuf {
        match v.data.as_deref() {
            Some(".") | Some("") => self.root.clone(),
            Some(d) => self.root.join(d),
            None => self.root.join("versions").join(&v.id),
        }
    }

    /// The version's game folder, from its `port.toml`.
    pub fn game_dir(&self, v: &Version) -> Option<PathBuf> {
        let text = std::fs::read_to_string(self.version_dir(v).join("port.toml")).ok()?;
        let t = text.parse::<toml::Table>().ok()?;
        t.get("game_dir").and_then(toml::Value::as_str).filter(|s| !s.is_empty()).map(PathBuf::from)
    }

    /// Writes the version's game folder into its `port.toml`; `None` removes it.
    pub fn set_game_dir(&self, v: &Version, game: Option<&Path>) -> Result<(), String> {
        let dir = self.version_dir(v);
        let file = dir.join("port.toml");
        let Some(game) = game else {
            return match std::fs::remove_file(&file) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(format!("{}: {e}", file.display())),
                _ => Ok(()),
            };
        };
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let text = format!(
            "# Settings for the NSMBW port, written by the launcher for {}.\n\
             # game_dir: your own copy of the game, extracted with Dolphin (holds sys and files).\n\
             game_dir = {}\n",
            v.title,
            toml::Value::String(game.to_string_lossy().into_owned())
        );
        std::fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))
    }

    pub fn options(&self, v: &Version) -> &[String] {
        self.state.options.get(&v.id).map_or(&[], Vec::as_slice)
    }
}

// ------------------------------------------------------------ the game's settings

/// A value of one of the game's settings, as `settings.toml` holds it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    Flag(bool),
    Number(i64),
    Word(&'static str),
}

/// One of the game's settings the launcher shows: a key of `settings.toml`
/// (`crates/frontend/src/settings.rs`) and the values it can take.
pub struct Setting {
    pub key: &'static str,
    pub label: &'static str,
    pub help: &'static str,
    /// The choices, each with its label; the first one marked `true` is the
    /// game's default.
    pub choices: &'static [(Value, &'static str, bool)],
}

use Value::{Flag, Number, Word};

const ON_OFF: [(Value, &str, bool); 2] = [(Flag(false), "Off", true), (Flag(true), "On", false)];

pub const SETTINGS: [Setting; 7] = [
    Setting {
        key: "fps",
        label: "Frame rate",
        help: "Frames shown each second. The game itself always runs at 60; higher rates draw frames in between.",
        choices: &[
            (Number(30), "30", false),
            (Number(60), "60 (the game's own)", true),
            (Number(90), "90", false),
            (Number(120), "120", false),
            (Number(144), "144", false),
            (Number(165), "165", false),
            (Number(180), "180", false),
            (Number(240), "240", false),
            (Number(300), "300", false),
            (Number(360), "360", false),
            (Number(400), "400", false),
            (Word("unlocked"), "Unlocked", false),
        ],
    },
    Setting { key: "vsync", label: "V-Sync", help: "Wait for the display's refresh: no tearing, a little more delay.", choices: &ON_OFF },
    Setting { key: "fullscreen", label: "Fullscreen", help: "Start in borderless fullscreen. F11 switches while playing.", choices: &ON_OFF },
    Setting {
        key: "scale",
        label: "Resolution",
        help: "How finely the game is drawn. Higher is sharper and asks more of the graphics card.",
        choices: &[
            (Number(1), "1x (480p)", false),
            (Number(2), "2x (960p)", true),
            (Number(3), "3x (1440p)", false),
            (Number(4), "4x (1920p)", false),
            (Number(5), "5x", false),
            (Number(6), "6x", false),
            (Number(7), "7x", false),
            (Number(8), "8x", false),
            (Word("native"), "The window's size", false),
        ],
    },
    Setting { key: "aspect", label: "Picture shape", help: "Widescreen or the classic 4:3 picture.", choices: &[(Word("16:9"), "16:9", true), (Word("4:3"), "4:3", false)] },
    Setting { key: "perf_monitor", label: "Performance monitor", help: "Show the frame rate and a frame-time graph while playing.", choices: &ON_OFF },
    Setting { key: "battery_saver", label: "Battery saver", help: "Show 30 frames a second while the game keeps running at 60.", choices: &ON_OFF },
];

impl Setting {
    fn default_index(&self) -> usize {
        self.choices.iter().position(|c| c.2).unwrap_or(0)
    }
}

fn settings_file(dir: &Path) -> PathBuf {
    dir.join("settings.toml")
}

/// The index of each of [`SETTINGS`]' current choice in `dir`'s
/// `settings.toml` (the game's default for anything missing or unknown).
pub fn read_settings(dir: &Path) -> [usize; SETTINGS.len()] {
    let table = std::fs::read_to_string(settings_file(dir)).ok().and_then(|s| s.parse::<toml::Table>().ok());
    std::array::from_fn(|i| {
        let s = &SETTINGS[i];
        let found = table.as_ref().and_then(|t| t.get(s.key)).and_then(|v| {
            s.choices.iter().position(|c| match c.0 {
                Flag(b) => v.as_bool() == Some(b),
                Number(n) => v.as_integer() == Some(n),
                Word(w) => v.as_str() == Some(w),
            })
        });
        found.unwrap_or_else(|| s.default_index())
    })
}

/// Sets one of [`SETTINGS`] in `dir`'s `settings.toml`, keeping the rest of
/// the file (the game's other settings and its comments) as it is.
pub fn write_setting(dir: &Path, setting: usize, choice: usize) -> Result<(), String> {
    let s = &SETTINGS[setting];
    let file = settings_file(dir);
    let text = std::fs::read_to_string(&file).unwrap_or_default();
    let mut doc = text.parse::<toml_edit::DocumentMut>().unwrap_or_default();
    doc[s.key] = match s.choices[choice].0 {
        Flag(b) => toml_edit::value(b),
        Number(n) => toml_edit::value(n),
        Word(w) => toml_edit::value(w),
    };
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    std::fs::write(&file, doc.to_string()).map_err(|e| format!("{}: {e}", file.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nsmbw-launcher-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn versions() -> Vec<Version> {
        crate::versions::load(Path::new("no such folder")).0
    }

    #[test]
    fn state_round_trip() {
        let root = temp("state");
        let mut store = Store::load(root.clone());
        assert_eq!(store.state, State::default());
        store.state = State { selected: "newer".into(), skip_launcher: true, game_exe: r"bin\port.exe".into(), update_pending: "0.2.0".into(), update_told: "0.1.5".into(), options: BTreeMap::from([("nsmbw".to_string(), vec!["--no-audio".to_string(), "--fps".into(), "120".into()])]) };
        store.save();
        let again = Store::load(root.clone());
        assert_eq!(again.state, store.state);
        assert_eq!(again.game_exe(), root.join(r"bin\port.exe"));
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn each_version_has_its_own_folder() {
        let root = temp("dirs");
        let store = Store::load(root.clone());
        let v = versions();
        assert_eq!(store.version_dir(&v[0]), root);
        assert_eq!(store.version_dir(&v[1]), root.join("versions").join("newer"));
        assert_eq!(store.game_dir(&v[1]), None);
        store.set_game_dir(&v[1], Some(Path::new(r"E:\it's here\newer"))).unwrap();
        assert_eq!(store.game_dir(&v[1]), Some(PathBuf::from(r"E:\it's here\newer")));
        assert_eq!(store.game_dir(&v[0]), None);
        store.set_game_dir(&v[1], None).unwrap();
        assert_eq!(store.game_dir(&v[1]), None);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_setting_changes_and_the_rest_stays() {
        let dir = temp("settings");
        let defaults = read_settings(&dir);
        assert_eq!(SETTINGS[0].choices[defaults[0]].0, Number(60));
        assert_eq!(SETTINGS[3].choices[defaults[3]].0, Number(2));
        std::fs::write(settings_file(&dir), "# mine\nfps = 144\nvolume = 80\naspect = \"4:3\"\n\n[keyboard]\njump = \"X\"\n").unwrap();
        let now = read_settings(&dir);
        assert_eq!(SETTINGS[0].choices[now[0]].0, Number(144));
        assert_eq!(SETTINGS[4].choices[now[4]].0, Word("4:3"));
        write_setting(&dir, 0, 11).unwrap();
        write_setting(&dir, 1, 1).unwrap();
        let text = std::fs::read_to_string(settings_file(&dir)).unwrap();
        assert!(text.contains("# mine") && text.contains("volume = 80") && text.contains("[keyboard]") && text.contains("jump = \"X\""));
        assert!(text.contains("fps = \"unlocked\"") && text.contains("vsync = true"));
        // The new key went before the [keyboard] table, where the game reads it.
        assert!(text.find("vsync").unwrap() < text.find("[keyboard]").unwrap());
        let now = read_settings(&dir);
        assert_eq!((now[0], now[1]), (11, 1));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
