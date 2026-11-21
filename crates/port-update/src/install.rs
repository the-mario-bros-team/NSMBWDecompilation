//! Installing a release in place: its Windows download saved next to the
//! programs, checked (its size, and its SHA-256 when the release gives
//! one), unpacked, and its files put where the old ones were. A program
//! that is running can be renamed but not replaced, so each file it
//! replaces is first moved aside (`<name>.old`); those are deleted at once
//! where Windows lets them go, the rest at the next start ([`cleanup`]).
//! Should a file fail to move, everything done so far is put back.
//!
//! The player's own files are never touched: settings, saves, caches and
//! logs (see [`kept`]).

use std::path::{Component, Path, PathBuf};
use std::sync::atomic::AtomicBool;

use crate::Release;

/// The folder, next to the programs, an update is unpacked in.
pub const WORK: &str = "update";
/// In [`WORK`]: the old files that could not be deleted yet.
const OLD_LIST: &str = "old-files.txt";

/// How far an update has got.
#[derive(Clone, Debug, PartialEq)]
pub enum Progress {
    Downloading { done: u64, total: u64 },
    Installing,
}

/// Downloads `release` and installs it in `folder` (the programs' folder),
/// which must then hold a new `program` (the running program's file name,
/// such as `nsmbw-port.exe`). Blocks; `cancel` stops the download. On
/// success the caller starts `program` again ([`restart`]).
pub fn update(release: &Release, folder: &Path, program: &str, progress: &dyn Fn(Progress), cancel: &AtomicBool) -> Result<(), String> {
    let download = release.download.as_ref().ok_or("this release has no download for Windows")?;
    cleanup(folder);
    let work = folder.join(WORK);
    // What an earlier try left; not the list of old files still to delete.
    let _ = std::fs::remove_dir_all(work.join("files"));
    std::fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let zip = work.join("download.zip");
    let result = (|| {
        crate::http::download(&download.url, &zip, download.size, &|done, total| progress(Progress::Downloading { done, total }), cancel)?;
        progress(Progress::Installing);
        let data = std::fs::read(&zip).map_err(|e| format!("{}: {e}", zip.display()))?;
        if download.size > 0 && data.len() as u64 != download.size {
            return Err("the download is incomplete".to_string());
        }
        if let Some(want) = &download.sha256 {
            if hex(&sha256(&data)) != *want {
                return Err("the download is damaged (its SHA-256 does not match the release's)".into());
            }
        }
        let files = strip_top_folder(read_zip(&data)?);
        let mut keep = Vec::new();
        for (name, contents) in files {
            let rel = safe_path(&name).ok_or_else(|| format!("the download holds a file outside its folder: {name}"))?;
            if !kept(&rel) {
                keep.push((rel, contents));
            }
        }
        if !keep.iter().any(|(rel, _)| rel.as_os_str().eq_ignore_ascii_case(program)) {
            return Err(format!("the download does not hold {program}"));
        }
        let staged = work.join("files");
        for (rel, contents) in &keep {
            let to = staged.join(rel);
            if let Some(parent) = to.parent() {
                std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
            }
            std::fs::write(&to, contents).map_err(|e| format!("{}: {e}", to.display()))?;
        }
        let rels: Vec<PathBuf> = keep.into_iter().map(|(rel, _)| rel).collect();
        install(&staged, folder, &rels)
    })();
    let _ = std::fs::remove_file(&zip);
    let _ = std::fs::remove_dir_all(work.join("files"));
    result
}

/// Starts `program` again with this run's arguments, in this run's working
/// folder: the new one, once [`update`] has put it in place.
pub fn restart(program: &Path) -> Result<(), String> {
    std::process::Command::new(program).args(std::env::args_os().skip(1)).spawn().map(|_| ()).map_err(|e| format!("{}: {e}", program.display()))
}

/// Deletes the old files an update left behind (the running program's,
/// which Windows keeps until it exits). Called at start.
pub fn cleanup(folder: &Path) {
    let list = folder.join(WORK).join(OLD_LIST);
    let Ok(text) = std::fs::read_to_string(&list) else { return };
    let left: Vec<&str> = text.lines().filter(|l| !l.is_empty()).filter(|l| safe_path(l).is_some_and(|rel| std::fs::remove_file(folder.join(&rel)).is_err() && folder.join(&rel).exists())).collect();
    if left.is_empty() {
        let _ = std::fs::remove_dir_all(folder.join(WORK));
    } else {
        let _ = std::fs::write(&list, left.join("\n"));
    }
}

/// Moves the files `rels` from `staged` into `folder`, each old one moved
/// aside first; all put back if one fails.
fn install(staged: &Path, folder: &Path, rels: &[PathBuf]) -> Result<(), String> {
    let mut done: Vec<(PathBuf, Option<PathBuf>)> = Vec::new();
    let undo = |done: &[(PathBuf, Option<PathBuf>)]| {
        for (to, old) in done.iter().rev() {
            let _ = std::fs::remove_file(to);
            if let Some(old) = old {
                let _ = std::fs::rename(old, to);
            }
        }
    };
    for rel in rels {
        let (from, to) = (staged.join(rel), folder.join(rel));
        if let Some(parent) = to.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                undo(&done);
                return Err(format!("{}: {e}", parent.display()));
            }
        }
        let old = if to.exists() {
            let old = aside(&to);
            if let Err(e) = std::fs::rename(&to, &old) {
                undo(&done);
                return Err(format!("{}: {e}", to.display()));
            }
            Some(old)
        } else {
            None
        };
        if let Err(e) = std::fs::rename(&from, &to) {
            if let Some(old) = &old {
                let _ = std::fs::rename(old, &to);
            }
            undo(&done);
            return Err(format!("{}: {e}", to.display()));
        }
        done.push((to, old));
    }
    // The old files: gone now where Windows lets them go.
    let left: Vec<String> = done
        .iter()
        .filter_map(|(_, old)| old.as_ref())
        .filter(|old| std::fs::remove_file(old).is_err())
        .filter_map(|old| old.strip_prefix(folder).ok().map(|rel| rel.to_string_lossy().into_owned()))
        .collect();
    if !left.is_empty() {
        let list = folder.join(WORK).join(OLD_LIST);
        let mut all = std::fs::read_to_string(&list).unwrap_or_default();
        for l in left {
            all.push_str(&l);
            all.push('\n');
        }
        let _ = std::fs::create_dir_all(folder.join(WORK));
        let _ = std::fs::write(list, all);
    }
    Ok(())
}

/// A free name to move `path` aside to: `<name>.old`, `<name>.old1`, ...
fn aside(path: &Path) -> PathBuf {
    let base = path.as_os_str().to_owned();
    (0..).map(|n| {
        let mut p = base.clone();
        p.push(if n == 0 { ".old".to_string() } else { format!(".old{n}") });
        PathBuf::from(p)
    })
    .find(|p| !p.exists() || std::fs::remove_file(p).is_ok())
    .expect("a free name")
}

/// The player's own files, which an update never replaces: settings,
/// saves, caches, logs, the launcher's versions and this update's folder.
fn kept(rel: &Path) -> bool {
    let first = rel.components().next().map(|c| c.as_os_str().to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    let top = rel.components().count() == 1;
    let name = rel.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    matches!(first.as_str(), "saves" | "cache" | "versions" | "work" | WORK) && !top
        || top && matches!(name.as_str(), "settings.toml" | "port.toml" | "update.toml" | "launcher.toml" | "versions.toml")
        || name.ends_with(".log")
}

/// A zip entry's name as a path inside the folder: `None` if it would
/// leave it (`..`, a drive, a root) or names nothing.
fn safe_path(name: &str) -> Option<PathBuf> {
    let mut out = PathBuf::new();
    for part in name.split(['/', '\\']).filter(|p| !p.is_empty() && *p != ".") {
        if part == ".." || part.contains(':') || part.ends_with(' ') || part.ends_with('.') {
            return None;
        }
        out.push(part);
    }
    let normal = out.components().all(|c| matches!(c, Component::Normal(_)));
    (normal && out.components().next().is_some()).then_some(out)
}

/// Drops a folder every entry is in ("nsmbw-port-0.2.0/..."), as zips
/// made from a folder have.
fn strip_top_folder(files: Vec<(String, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
    let split = |n: &str| n.trim_start_matches(['/', '\\']).split_once(['/', '\\']).map(|(a, b)| (a.to_string(), b.to_string()));
    let tops: Vec<Option<(String, String)>> = files.iter().map(|(n, _)| split(n)).collect();
    let first = tops.first().and_then(|t| t.as_ref()).map(|t| t.0.clone());
    if first.is_none() || tops.iter().any(|t| t.as_ref().map(|t| &t.0) != first.as_ref()) {
        return files;
    }
    files.into_iter().zip(tops).map(|((_, c), t)| (t.map(|t| t.1).unwrap_or_default(), c)).collect()
}

/// The files of a zip archive, stored or deflated, each checked against
/// its CRC-32: (name as written, contents). Folders are left out.
pub fn read_zip(data: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    let bad = || "the download is not a readable zip archive".to_string();
    let u16_at = |at: usize| data.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).ok_or_else(bad);
    let u32_at = |at: usize| data.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize).ok_or_else(bad);
    // The end record: in the last 64 KiB (its comment) and 22 bytes.
    let from = data.len().saturating_sub(22 + 0xFFFF);
    let end = (from..=data.len().saturating_sub(22)).rev().find(|&i| data[i..].starts_with(&[0x50, 0x4B, 0x05, 0x06])).ok_or_else(bad)?;
    let (count, mut at) = (u16_at(end + 10)?, u32_at(end + 16)?);
    if count == 0xFFFF || at == 0xFFFF_FFFF {
        return Err("the download is a zip64 archive, which is not supported".into());
    }
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if u32_at(at)? != 0x0201_4B50 {
            return Err(bad());
        }
        let (flags, method, crc) = (u16_at(at + 8)?, u16_at(at + 10)?, u32_at(at + 16)? as u32);
        let (packed, size) = (u32_at(at + 20)?, u32_at(at + 24)?);
        let (name_len, extra_len, comment_len, local) = (u16_at(at + 28)?, u16_at(at + 30)?, u16_at(at + 32)?, u32_at(at + 42)?);
        let name = String::from_utf8_lossy(data.get(at + 46..at + 46 + name_len).ok_or_else(bad)?).into_owned();
        at += 46 + name_len + extra_len + comment_len;
        if name.ends_with('/') || name.ends_with('\\') {
            continue;
        }
        if flags & 1 != 0 {
            return Err("the download's zip archive is encrypted".into());
        }
        if u32_at(local)? != 0x0403_4B50 {
            return Err(bad());
        }
        let start = local + 30 + u16_at(local + 26)? + u16_at(local + 28)?;
        let raw = data.get(start..start + packed).ok_or_else(bad)?;
        let contents = match method {
            0 => raw.to_vec(),
            8 => miniz_oxide::inflate::decompress_to_vec_with_limit(raw, size).map_err(|_| format!("{name}: damaged in the download"))?,
            m => return Err(format!("{name}: packed in a way not supported ({m})")),
        };
        if contents.len() != size || crc32(&contents) != crc {
            return Err(format!("{name}: damaged in the download"));
        }
        out.push((name, contents));
    }
    Ok(out)
}

fn crc32(data: &[u8]) -> u32 {
    static TABLE: std::sync::OnceLock<[u32; 256]> = std::sync::OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut t = [0u32; 256];
        for (i, e) in t.iter_mut().enumerate() {
            let mut c = i as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            }
            *e = c;
        }
        t
    });
    !data.iter().fold(!0u32, |c, &b| table[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8))
}

/// SHA-256 (FIPS 180-4), for checking a download against the digest the
/// release gives.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut block = |chunk: &[u8]| {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let [a, b, c, d, e, f, g, hh] = v;
            let t1 = hh.wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25)).wrapping_add((e & f) ^ (!e & g)).wrapping_add(K[i]).wrapping_add(w[i]);
            let t2 = (a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22)).wrapping_add((a & b) ^ (a & c) ^ (b & c));
            v = [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g];
        }
        for (x, y) in h.iter_mut().zip(v) {
            *x = x.wrapping_add(y);
        }
    };
    let whole = data.len() / 64 * 64;
    data[..whole].chunks(64).for_each(&mut block);
    let mut tail = data[whole..].to_vec();
    tail.push(0x80);
    while tail.len() % 64 != 56 {
        tail.push(0);
    }
    tail.extend_from_slice(&((data.len() as u64).wrapping_mul(8)).to_be_bytes());
    tail.chunks(64).for_each(&mut block);
    let mut out = [0u8; 32];
    for (o, x) in out.chunks_mut(4).zip(h) {
        o.copy_from_slice(&x.to_be_bytes());
    }
    out
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A zip with "top/a.txt" stored, "top/sub/b.txt" deflated and a folder.
    fn zip(entries: &[(&str, &[u8], bool)]) -> Vec<u8> {
        let (mut out, mut central) = (Vec::new(), Vec::new());
        for (name, data, deflate) in entries {
            let packed = if *deflate { miniz_oxide::deflate::compress_to_vec(data, 6) } else { data.to_vec() };
            let method: u16 = if *deflate { 8 } else { 0 };
            let at = out.len() as u32;
            let head = |sig: u32, central: bool| {
                let mut h = sig.to_le_bytes().to_vec();
                if central {
                    h.extend_from_slice(&20u16.to_le_bytes());
                }
                h.extend_from_slice(&20u16.to_le_bytes());
                h.extend_from_slice(&0u16.to_le_bytes());
                h.extend_from_slice(&method.to_le_bytes());
                h.extend_from_slice(&[0; 4]);
                h.extend_from_slice(&crc32(data).to_le_bytes());
                h.extend_from_slice(&(packed.len() as u32).to_le_bytes());
                h.extend_from_slice(&(data.len() as u32).to_le_bytes());
                h.extend_from_slice(&(name.len() as u16).to_le_bytes());
                h.extend_from_slice(&0u16.to_le_bytes());
                if central {
                    // Comment length, disk, internal and external attributes.
                    h.extend_from_slice(&[0; 10]);
                    h.extend_from_slice(&at.to_le_bytes());
                }
                h.extend_from_slice(name.as_bytes());
                h
            };
            out.extend(head(0x0403_4B50, false));
            out.extend_from_slice(&packed);
            central.extend(head(0x0201_4B50, true));
        }
        let (cd_at, cd_len) = (out.len() as u32, central.len() as u32);
        out.extend(central);
        out.extend_from_slice(&0x0605_4B50u32.to_le_bytes());
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        out.extend_from_slice(&cd_len.to_le_bytes());
        out.extend_from_slice(&cd_at.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    #[test]
    fn sha256_matches_the_standard() {
        assert_eq!(hex(&sha256(b"")), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(hex(&sha256(b"abc")), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(hex(&sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
        assert_eq!(hex(&sha256(&vec![b'a'; 1_000_000])), "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
    }

    #[test]
    fn zips_are_read_and_checked() {
        let text = b"hello hello hello hello hello".to_vec();
        let z = zip(&[("top/", b"", false), ("top/a.txt", b"stored", false), ("top/sub/b.txt", &text, true)]);
        let files = read_zip(&z).unwrap();
        assert_eq!(files, vec![("top/a.txt".to_string(), b"stored".to_vec()), ("top/sub/b.txt".to_string(), text.clone())]);
        let files = strip_top_folder(files);
        assert_eq!(files[0].0, "a.txt");
        assert_eq!(files[1].0, "sub/b.txt");
        // A changed byte fails its CRC.
        let mut broken = z.clone();
        let i = broken.windows(6).position(|w| w == b"stored").unwrap();
        broken[i] ^= 1;
        assert!(read_zip(&broken).is_err());
        assert!(read_zip(b"not a zip").is_err());
    }

    #[test]
    fn paths_stay_inside_the_folder() {
        assert_eq!(safe_path("a/b.exe"), Some(PathBuf::from("a").join("b.exe")));
        assert_eq!(safe_path("a\\b.exe"), Some(PathBuf::from("a").join("b.exe")));
        assert_eq!(safe_path("./a"), Some(PathBuf::from("a")));
        for bad in ["../a", "a/../../b", "C:/Windows/x.exe", "C:x", "/", "", "a/b./c", "a:stream"] {
            assert_eq!(safe_path(bad), None, "{bad}");
        }
        assert_eq!(safe_path("/abs/x"), Some(PathBuf::from("abs").join("x")));
    }

    #[test]
    fn the_players_files_are_kept() {
        for k in ["settings.toml", "Port.toml", "update.toml", "saves/nand/x.bin", "cache/pipelines.bin", "nsmbw-port.log", "versions/newer/settings.toml"] {
            assert!(kept(&safe_path(k).unwrap()), "{k}");
        }
        for k in ["nsmbw-port.exe", "nsmbw-launcher.exe", "README.md", "docs/settings.toml"] {
            assert!(!kept(&safe_path(k).unwrap()), "{k}");
        }
    }

    #[test]
    fn install_moves_files_and_old_ones_aside() {
        let dir = std::env::temp_dir().join(format!("port-update-install-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (staged, folder) = (dir.join("staged"), dir.join("folder"));
        std::fs::create_dir_all(staged.join("sub")).unwrap();
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(staged.join("a.exe"), "new a").unwrap();
        std::fs::write(staged.join("sub").join("b.txt"), "new b").unwrap();
        std::fs::write(folder.join("a.exe"), "old a").unwrap();
        install(&staged, &folder, &[PathBuf::from("a.exe"), PathBuf::from("sub").join("b.txt")]).unwrap();
        assert_eq!(std::fs::read_to_string(folder.join("a.exe")).unwrap(), "new a");
        assert_eq!(std::fs::read_to_string(folder.join("sub").join("b.txt")).unwrap(), "new b");
        // Not running: its old copy is gone already.
        assert!(!folder.join("a.exe.old").exists());
        // A file that cannot be moved in: everything is put back.
        std::fs::write(staged.join("c.txt"), "new c").unwrap();
        std::fs::write(folder.join("a.exe"), "old a").unwrap();
        std::fs::create_dir_all(staged.join("again")).unwrap();
        std::fs::write(staged.join("again").join("a.exe"), "newer a").unwrap();
        let err = install(&staged.join("again"), &folder, &[PathBuf::from("a.exe"), PathBuf::from("missing.txt")]);
        assert!(err.is_err());
        assert_eq!(std::fs::read_to_string(folder.join("a.exe")).unwrap(), "old a");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
