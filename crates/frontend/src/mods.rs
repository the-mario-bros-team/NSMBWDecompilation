//! The Settings window's Mods tab, after WitherBlitz's NSMB-Wii-Recompilation
//! (`runtime/src/hd_texture_pack.cpp`): HD texture packs and file mods, in
//! folders beside `port.toml`.
//!
//! - `texture_packs/<name>/`: a Dolphin texture pack ([`gx::replace`]), its
//!   files anywhere under that folder (Dolphin's `Load/Textures/SMN` as it
//!   is). One pack at a time. Render-only, so net games allow it.
//! - `texture_dumps/`: with `texture_dumps` on, every texture no pack
//!   replaces is written there under its pack name, to make a pack from.
//! - `mods/<name>/`: files laid out as the disc's `files/` folder, served in
//!   place of the disc's own from the next start ([`hle_ios::Disc::replace`]).
//!   Mods load in name order, a later one's file winning. Not in net games:
//!   every player would need the same.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::settings::Settings;

pub const PACKS: &str = "texture_packs";
pub const DUMPS: &str = "texture_dumps";
pub const MODS: &str = "mods";

/// The folders in `dir`, by name.
fn folders(dir: &Path) -> Vec<String> {
    let Ok(list) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut names: Vec<String> = list.flatten().filter(|e| e.file_type().is_ok_and(|t| t.is_dir())).map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort_by_key(|n| n.to_lowercase());
    names
}

/// The texture packs installed.
pub fn packs() -> Vec<String> {
    folders(Path::new(PACKS))
}

/// The file mods installed, in load order.
pub fn mods() -> Vec<String> {
    folders(Path::new(MODS))
}

/// The pack the settings name if it is installed, else the first one.
pub fn pack_in_use(s: &Settings, packs: &[String]) -> Option<String> {
    packs.iter().find(|p| **p == s.texture_pack).or(packs.first()).cloned()
}

/// Who made the pack in folder `name`, for the packs known.
pub fn credits(name: &str) -> Option<&'static str> {
    // Bighead's NSMBW HD Texture Pack (its archive's folder is NSMBW_HD_v2.81_PNG).
    name.to_ascii_lowercase().starts_with("nsmbw_hd").then_some("Bighead, razius, jaco8, CDAMJC and Amon1995")
}

/// `path` in full, for telling the player where a folder is.
pub fn full_path(path: &str) -> String {
    std::env::current_dir().map(|d| d.join(path)).unwrap_or_else(|_| PathBuf::from(path)).display().to_string()
}

/// What the GX thread replaces textures with.
#[derive(Clone, Default)]
pub struct PackSpec {
    /// The pack's folder.
    pub folder: Option<PathBuf>,
    /// Where textures without a replacement are written.
    pub dumps: Option<PathBuf>,
    /// Images used ahead of the pack's files, by the key of the texture
    /// they replace: the hook for textures the port supplies itself (such
    /// as button prompts for the controller in use).
    pub images: Vec<(gx::replace::Key, Arc<gx::replace::Image>)>,
}

impl PackSpec {
    pub fn from_settings(s: &Settings) -> Self {
        let folder = s.hd_textures.then(|| pack_in_use(s, &packs())).flatten().map(|p| Path::new(PACKS).join(p));
        Self { folder, dumps: s.texture_dumps.then(|| PathBuf::from(DUMPS)), images: Vec::new() }
    }

    /// The pack, or `None` when there is nothing to replace or dump.
    pub fn open(&self) -> Option<gx::replace::Pack> {
        if self.folder.is_none() && self.dumps.is_none() && self.images.is_empty() {
            return None;
        }
        let mut pack = gx::replace::Pack::new(self.dumps.clone());
        if let Some(f) = &self.folder {
            let n = pack.add_folder(f);
            println!("texture pack: {n} textures in {}", f.display());
        }
        for (key, image) in &self.images {
            pack.add(*key, image.clone());
        }
        if let Some(d) = &self.dumps {
            println!("texture dumps: textures without a replacement go to {}", d.display());
        }
        Some(pack)
    }
}

// ---------------------------------------------------------------- the HD pack's download

/// Bighead's NSMBW HD Texture Pack v2.81, PNG edition, from the pack's own
/// links (its Dolphin forum thread): Google Drive, else its MediaFire mirror,
/// whose page carries the file's own address.
const DRIVE_URL: &str = "https://drive.usercontent.google.com/download?id=1gcPQ2u_GXl3IpziEVvIsVzS90-mar2ZM&export=download&confirm=t";
const MEDIAFIRE_PAGE: &str = "https://www.mediafire.com/file/goz5ep7dgz4ripo/NSMBW_HD_v2.81_PNG.7z/file";
const ARCHIVE_SIZE: u64 = 23_346_788;
const ARCHIVE_SHA256: &str = "acd46f6490d02acbcb871f3e0a20b4b9f8534367e32a4535b364a51ef874b1d2";
/// Inside the archive: `NSMBW_HD_v2.81_PNG/SMN/...` are the textures; its
/// "~PSX Buttons" alternative, Thumbs.db files and readme stay out.
const ARCHIVE_ROOT: &str = "NSMBW_HD_v2.81_PNG/";
/// The pack's folder under [`PACKS`] once installed.
pub const HD_PACK: &str = "NSMBW_HD_v2.81";
/// Downloaded and unpacked here first, and moved into [`PACKS`] last, so an
/// interrupted download never leaves a pack behind.
const STAGING: &str = "hd_pack.tmp";

const FAILED_DOWNLOAD: &str = "The download failed. Check your connection and try again.";
const FAILED_DAMAGED: &str = "The download was damaged. Try again.";
const FAILED_INSTALL: &str = "Couldn't install the pack: is there room on the drive?";

/// The HD pack's download, as the Mods tab shows it.
#[derive(Clone, Debug, PartialEq)]
pub enum PackDownload {
    Downloading { done: u64, total: u64 },
    Installing,
    /// Installed, with this many textures.
    Done(usize),
    /// Why, for the player.
    Failed(String),
}

/// Downloads and installs the HD pack on a thread of its own; the state
/// says how far it is.
pub fn download_hd_pack() -> Arc<std::sync::Mutex<PackDownload>> {
    let state = Arc::new(std::sync::Mutex::new(PackDownload::Downloading { done: 0, total: ARCHIVE_SIZE }));
    let s = state.clone();
    std::thread::spawn(move || {
        let set = |v: PackDownload| *s.lock().unwrap_or_else(|e| e.into_inner()) = v;
        let staging = Path::new(STAGING);
        let r = install_hd_pack(staging, &set);
        let _ = std::fs::remove_dir_all(staging);
        set(match r {
            Ok(n) => {
                println!("HD texture pack: {n} textures installed in {}", full_path(&format!("{PACKS}\\{HD_PACK}")));
                PackDownload::Done(n)
            }
            Err(e) => PackDownload::Failed(e),
        });
    });
    state
}

/// The archive from either link, checked; then its textures into place.
fn install_hd_pack(staging: &Path, set: &dyn Fn(PackDownload)) -> Result<usize, String> {
    let _ = std::fs::remove_dir_all(staging);
    std::fs::create_dir_all(staging).map_err(|e| {
        eprintln!("HD texture pack: {}: {e}", staging.display());
        FAILED_INSTALL.to_string()
    })?;
    let archive = staging.join("NSMBW_HD_v2.81_PNG.7z");
    let mut error = FAILED_DOWNLOAD;
    let mut fetched = false;
    for source in ["Google Drive", "MediaFire"] {
        let url = if source == "MediaFire" {
            match port_update::http::get(MEDIAFIRE_PAGE) {
                Ok(Some(page)) => match page.find("https://download").map(|at| &page[at..]) {
                    Some(rest) => rest[..rest.find(['"', '\'', ' ', '<', '>']).unwrap_or(rest.len())].to_string(),
                    None => String::new(),
                },
                Ok(None) => String::new(),
                Err(e) => {
                    eprintln!("HD texture pack: the mirror's page: {e}");
                    continue;
                }
            }
        } else {
            DRIVE_URL.to_string()
        };
        if !url.contains(".mediafire.com/") && source == "MediaFire" {
            eprintln!("HD texture pack: no file address on the mirror's page");
            continue;
        }
        let progress = |done: u64, total: u64| set(PackDownload::Downloading { done, total: if total > 0 { total } else { ARCHIVE_SIZE } });
        if let Err(e) = port_update::http::download(&url, &archive, ARCHIVE_SIZE, &progress, &std::sync::atomic::AtomicBool::new(false)) {
            eprintln!("HD texture pack: {source}: {e}");
            continue;
        }
        let bytes = std::fs::read(&archive).unwrap_or_default();
        if bytes.len() as u64 != ARCHIVE_SIZE || port_update::install::hex(&port_update::install::sha256(&bytes)) != ARCHIVE_SHA256 {
            eprintln!("HD texture pack: {source}: {} bytes, not the pack's {ARCHIVE_SIZE} or not its SHA-256", bytes.len());
            error = FAILED_DAMAGED;
            continue;
        }
        println!("HD texture pack: downloaded from {source}");
        fetched = true;
        break;
    }
    if !fetched {
        return Err(error.into());
    }
    set(PackDownload::Installing);
    let unpacked = staging.join("unpacked");
    let failed = |e: String| {
        eprintln!("HD texture pack: {e}");
        FAILED_INSTALL.to_string()
    };
    let written = unpack(&archive, &unpacked).map_err(failed)?;
    // The pack has 321.
    if written < 300 {
        return Err(failed(format!("only {written} textures in the archive")));
    }
    let install = Path::new(PACKS).join(HD_PACK);
    let _ = std::fs::remove_dir_all(&install);
    std::fs::create_dir_all(PACKS).map_err(|e| failed(format!("{PACKS}: {e}")))?;
    // A virus scanner still reading the new files refuses the move for a moment.
    let mut moved = std::fs::rename(&unpacked, &install);
    for _ in 0..40 {
        if moved.is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        moved = std::fs::rename(&unpacked, &install);
    }
    moved.map_err(|e| failed(format!("moving the textures into place: {e}")))?;
    Ok(written)
}

/// A texture's path inside the archive's root ("SMN/..."), or `None` for
/// anything else, or a name that could leave the folder.
fn pack_path(name: &str) -> Option<String> {
    let path = name.replace('\\', "/");
    let path = path.strip_prefix(ARCHIVE_ROOT)?;
    let ok = path.starts_with("SMN/") && path.to_ascii_lowercase().ends_with(".png") && path.split('/').all(|p| !p.is_empty() && p != "." && p != "..") && !path.contains([':', '*', '?', '"', '<', '>', '|']);
    ok.then(|| path.to_string())
}

/// The archive's textures into `to` (`to/SMN/...`); returns how many.
fn unpack(archive: &Path, to: &Path) -> Result<usize, String> {
    let mut reader = sevenz_rust2::ArchiveReader::open(archive, sevenz_rust2::Password::empty()).map_err(|e| format!("not a 7z archive: {e}"))?;
    let mut written = 0;
    let mut failure = None;
    reader
        .for_each_entries(|entry, data| {
            let Some(path) = pack_path(entry.name()).filter(|_| !entry.is_directory()) else {
                // Read through: the archive is one solid block.
                std::io::copy(data, &mut std::io::sink())?;
                return Ok(true);
            };
            let target = to.join(&path);
            let mut bytes = Vec::new();
            data.read_to_end(&mut bytes)?;
            let wrote = target.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|()| std::fs::write(&target, &bytes));
            if let Err(e) = wrote {
                failure = Some(format!("{}: {e}", target.display()));
                return Ok(false);
            }
            written += 1;
            Ok(true)
        })
        .map_err(|e| format!("unpacking: {e}"))?;
    failure.map_or(Ok(written), Err)
}

/// The files of the mods named in `enabled` that are installed, as (disc
/// path, bytes), in load order: a later mod's file replaces an earlier's.
pub fn mod_files(enabled: &[String]) -> Vec<(String, Vec<u8>)> {
    mod_files_in(Path::new(MODS), enabled)
}

/// [`mod_files`] for the mods in folder `mods`.
fn mod_files_in(mods: &Path, enabled: &[String]) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for name in folders(mods).into_iter().filter(|m| enabled.contains(m)) {
        let root = mods.join(&name);
        let mut dirs = vec![root.clone()];
        let mut count = 0;
        while let Some(dir) = dirs.pop() {
            let Ok(list) = std::fs::read_dir(&dir) else { continue };
            for e in list.flatten() {
                let path = e.path();
                if e.file_type().is_ok_and(|t| t.is_dir()) {
                    dirs.push(path);
                    continue;
                }
                let Ok(rel) = path.strip_prefix(&root) else { continue };
                let disc: String = rel.components().map(|c| format!("/{}", c.as_os_str().to_string_lossy())).collect();
                match std::fs::read(&path) {
                    Ok(bytes) => {
                        files.retain(|(p, _)| !p.eq_ignore_ascii_case(&disc));
                        files.push((disc, bytes));
                        count += 1;
                    }
                    Err(e) => eprintln!("mod {name}: {}: {e}", path.display()),
                }
            }
        }
        println!("mod {name}: {count} files");
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mods_mirror_the_disc_in_name_order() {
        let dir = std::env::temp_dir().join(format!("nsmbw-mods-{}", std::process::id()));
        let put = |path: &str, bytes: &[u8]| {
            let p = dir.join(path);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, bytes).unwrap();
        };
        put("A/Stage/01-01.arc", b"a");
        put("A/Layout/x.arc", b"ax");
        put("B/Layout/X.arc", b"bx");
        put("Off/Stage/01-02.arc", b"off");
        let enabled = vec!["A".to_string(), "B".to_string()];
        let mut files = mod_files_in(&dir, &enabled);
        files.sort();
        // B loads after A: its X.arc wins over A's x.arc (the disc ignores case).
        assert_eq!(files, vec![("/Layout/X.arc".to_string(), b"bx".to_vec()), ("/Stage/01-01.arc".to_string(), b"a".to_vec())]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn only_the_packs_textures_are_unpacked() {
        assert_eq!(pack_path("NSMBW_HD_v2.81_PNG/SMN/Mario/tex1_64x64_0123_5.png").as_deref(), Some("SMN/Mario/tex1_64x64_0123_5.png"));
        assert_eq!(pack_path("NSMBW_HD_v2.81_PNG\\SMN\\a.PNG").as_deref(), Some("SMN/a.PNG"));
        for bad in ["NSMBW_HD_v2.81_PNG/SMN/Thumbs.db", "NSMBW_HD_v2.81_PNG/~PSX Buttons/b.png", "NSMBW_HD_v2.81_PNG/SMN/../x.png", "Other/SMN/a.png", "NSMBW_HD_v2.81_PNG/SMN//a.png"] {
            assert_eq!(pack_path(bad), None, "{bad}");
        }
    }
}
