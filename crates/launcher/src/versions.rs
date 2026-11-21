//! The games the launcher offers, as data: `versions.toml` (built in, or the
//! file of that name next to the launcher). See that file for the fields.

use std::path::Path;

use egui::Color32;
use serde::Deserialize;

const BUILT_IN: &str = include_str!("versions.toml");
pub const FILE: &str = "versions.toml";

/// A colour written `#rrggbb`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hex(pub Color32);

impl<'de> Deserialize<'de> for Hex {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        parse_hex(&s).map(Hex).ok_or_else(|| serde::de::Error::custom(format!("{s}: a colour is written #rrggbb")))
    }
}

fn parse_hex(s: &str) -> Option<Color32> {
    let h = s.strip_prefix('#')?;
    if h.len() != 6 || !h.is_ascii() {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some(Color32::from_rgb(byte(0)?, byte(2)?, byte(4)?))
}

fn hex(s: &str) -> Hex {
    Hex(parse_hex(s).unwrap_or(Color32::GRAY))
}

#[derive(Clone, Debug, Deserialize)]
pub struct Disc {
    pub id: String,
    pub revision: Option<u8>,
    #[serde(default)]
    pub name: String,
}

/// A picture in the player's game folder: a U8 archive and a texture in it.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ArtRef {
    pub archive: String,
    /// The texture's file name; empty = the archive's largest texture.
    #[serde(default)]
    pub name: String,
    /// For a gallery: the textures whose names start with this.
    #[serde(default)]
    pub prefix: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Art {
    pub logo: Option<ArtRef>,
    pub gallery: Option<ArtRef>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Glow {
    pub color: Hex,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub disc: f32,
}

impl Default for Glow {
    fn default() -> Self {
        Self { color: hex("#ffffff"), x: 0.75, y: 0.25, size: 0.5, disc: 0.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Clouds {
    pub color: Hex,
    pub amount: f32,
}

impl Default for Clouds {
    fn default() -> Self {
        Self { color: hex("#ffffff"), amount: 0.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Particles {
    pub color: Hex,
    pub amount: f32,
    pub fall: bool,
}

impl Default for Particles {
    fn default() -> Self {
        Self { color: hex("#ffffff"), amount: 0.0, fall: false }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Ridge {
    pub top: Hex,
    pub bottom: Hex,
    pub height: f32,
    pub amp: f32,
    pub rough: f32,
    pub width: f32,
    pub speed: f32,
}

impl Default for Ridge {
    fn default() -> Self {
        Self { top: hex("#3a4a6a"), bottom: hex("#1a2438"), height: 0.3, amp: 0.06, rough: 0.0, width: 1.0, speed: 0.01 }
    }
}

/// The picture drawn behind everything (`scene.rs`).
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct SceneSpec {
    pub sky: [Hex; 3],
    pub glow: Glow,
    pub stars: f32,
    pub aurora: Vec<Hex>,
    pub clouds: Clouds,
    pub particles: Particles,
    pub ridge: Vec<Ridge>,
}

impl Default for SceneSpec {
    fn default() -> Self {
        Self { sky: [hex("#101830"), hex("#24406e"), hex("#6f9bd0")], glow: Glow::default(), stars: 0.3, aurora: Vec::new(), clouds: Clouds::default(), particles: Particles::default(), ridge: vec![Ridge::default()] }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub accent: [Hex; 2],
    pub ink: Hex,
    pub scene: SceneSpec,
}

impl Default for Theme {
    fn default() -> Self {
        Self { accent: [hex("#6fd0ff"), hex("#3a7bff")], ink: hex("#ffffff"), scene: SceneSpec::default() }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub short: String,
    #[serde(default)]
    pub tagline: String,
    pub data: Option<String>,
    #[serde(default)]
    pub playable: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub discs: Vec<Disc>,
    #[serde(default)]
    pub with: Vec<String>,
    #[serde(default)]
    pub without: Vec<String>,
    #[serde(default)]
    pub art: Art,
    #[serde(default)]
    pub theme: Theme,
}

impl Version {
    /// The button label: `short`, or the title when there is none.
    pub fn short(&self) -> &str {
        if self.short.is_empty() {
            &self.title
        } else {
            &self.short
        }
    }
}

#[derive(Deserialize)]
struct File {
    version: Vec<Version>,
}

fn parse(text: &str) -> Result<Vec<Version>, String> {
    let file: File = toml::from_str(text).map_err(|e| e.to_string())?;
    if file.version.is_empty() {
        return Err("no [[version]] in it".into());
    }
    for (i, v) in file.version.iter().enumerate() {
        let plain = !v.id.is_empty() && v.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !plain {
            return Err(format!("version id \"{}\": letters, digits, - and _ only", v.id));
        }
        if file.version[..i].iter().any(|o| o.id == v.id) {
            return Err(format!("version id \"{}\" is used twice", v.id));
        }
    }
    Ok(file.version)
}

/// The versions: `versions.toml` in `root` when there is a valid one, else
/// the built-in list. The second value is what was wrong with the file.
pub fn load(root: &Path) -> (Vec<Version>, Option<String>) {
    let built_in = || parse(BUILT_IN).expect("the built-in versions.toml is valid");
    match std::fs::read_to_string(root.join(FILE)) {
        Ok(text) => match parse(&text) {
            Ok(v) => (v, None),
            Err(e) => (built_in(), Some(format!("{FILE}: {e}. Using the built-in list."))),
        },
        Err(_) => (built_in(), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_built_in_list_is_valid() {
        let v = parse(BUILT_IN).unwrap();
        assert_eq!(v[0].id, "nsmbw");
        assert!(v[0].playable && v[0].data.as_deref() == Some("."));
        assert_eq!(v[0].discs[0].revision, Some(1));
        assert_eq!(v[0].theme.scene.ridge.len(), 4);
        assert_eq!(v[1].id, "newer");
        assert!(!v[1].playable && !v[1].status.is_empty());
        assert_eq!(v[1].discs[0].revision, None);
    }

    #[test]
    fn a_version_is_only_data() {
        let v = parse("[[version]]\nid = \"demo\"\ntitle = \"Demo\"\n[version.theme]\naccent = [\"#102030\", \"#405060\"]\n").unwrap();
        assert_eq!(v[0].short(), "Demo");
        assert_eq!(v[0].theme.accent[0], Hex(Color32::from_rgb(0x10, 0x20, 0x30)));
        assert!(!v[0].playable);
        assert!(parse("[[version]]\nid = \"a b\"\ntitle = \"x\"\n").is_err());
        assert!(parse("[[version]]\nid = \"a\"\ntitle = \"x\"\n[[version]]\nid = \"a\"\ntitle = \"y\"\n").is_err());
        assert!(parse("[[version]]\nid = \"a\"\ntitle = \"x\"\n[version.theme]\nink = \"red\"\n").is_err());
        assert!(parse("").is_err());
    }
}
