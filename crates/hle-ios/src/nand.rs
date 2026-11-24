//! The NAND as a host folder, plus the system files the game reads from it
//! (`SYSCONF`, `setting.txt`), generated with defaults on first use.

use std::path::{Path, PathBuf};

/// Host path for a NAND path such as `/shared2/sys/SYSCONF`. Rejects `..`,
/// empty and non-absolute paths.
pub fn host_path(root: &Path, nand: &str) -> Option<PathBuf> {
    let rest = nand.strip_prefix('/')?;
    let mut p = root.to_path_buf();
    for part in rest.split('/').filter(|s| !s.is_empty()) {
        if part == "." || part == ".." || part.contains(['\\', ':']) {
            return None;
        }
        p.push(part);
    }
    Some(p)
}

/// Writes the system files the SDK expects if they are missing, and replaces
/// a `SYSCONF` the SDK would reject (earlier versions of this port wrote one).
pub fn ensure_system_files(root: &Path) -> std::io::Result<()> {
    let files: [(&str, fn() -> Vec<u8>); 2] =
        [("shared2/sys/SYSCONF", sysconf), ("title/00000001/00000002/data/setting.txt", setting_txt)];
    for (rel, make) in files {
        let path = root.join(rel);
        let stale = rel.ends_with("SYSCONF") && std::fs::read(&path).is_ok_and(|c| !sysconf_valid(&c));
        if !path.exists() || stale {
            std::fs::create_dir_all(path.parent().unwrap())?;
            std::fs::write(&path, make())?;
        }
    }
    Ok(())
}

/// `setting.txt`: `KEY=VALUE\r\n` lines in 0x100 bytes, XORed with a rotating key.
fn setting_txt() -> Vec<u8> {
    setting_txt_for(b'E')
}

/// [`setting_txt`] of a console of the region a game ID's fourth letter
/// names (E USA, P Europe, J Japan, K Korea, W Taiwan; others as USA).
fn setting_txt_for(region: u8) -> Vec<u8> {
    let (area, code, video, game) = match region {
        b'P' => ("EUR", "LEH", "PAL", "EU"),
        b'J' => ("JPN", "LJ", "NTSC", "JP"),
        b'K' => ("KOR", "LKH", "NTSC", "KR"),
        b'W' => ("TWN", "LT", "NTSC", "JP"),
        _ => ("USA", "LU", "NTSC", "US"),
    };
    let text = format!("AREA={area}\r\nMODEL=RVL-001({area})\r\nDVD=0\r\nMPCH=0x7FFE\r\nCODE={code}\r\nSERNO=000000000\r\nVIDEO={video}\r\nGAME={game}\r\n");
    let mut out = vec![0u8; 0x100];
    out[..text.len()].copy_from_slice(text.as_bytes());
    let mut key: u32 = 0x73B5_DBFA;
    for b in &mut out {
        *b ^= key as u8;
        key = key.rotate_left(1);
    }
    out
}

enum Item {
    Byte(u8),
    Long(u32),
    Bool(bool),
    Small(Vec<u8>),
    Big(Vec<u8>),
}

/// `SYSCONF`: "SCv0", item count, item offsets (plus the end offset), the
/// items, and "SCed" in the last 4 of 0x4000 bytes. Each item starts with
/// `type << 5 | (name length - 1)` and its name.
fn sysconf() -> Vec<u8> {
    let mut address = vec![0u8; 0x1007];
    address[0] = 49; // country: United States
    let items: Vec<(&str, Item)> = vec![
        ("IPL.LNG", Item::Byte(1)), // English
        ("IPL.AR", Item::Byte(1)),  // 16:9
        ("IPL.PGS", Item::Byte(1)), // progressive scan
        ("IPL.E60", Item::Byte(1)),
        ("IPL.SND", Item::Byte(1)), // stereo
        ("IPL.SSV", Item::Byte(0)), // screen burn-in reduction off
        ("IPL.DH", Item::Byte(0)),
        ("IPL.CB", Item::Long(0)), // RTC counter bias
        ("IPL.CD", Item::Bool(true)),
        ("IPL.CD2", Item::Bool(true)),
        ("IPL.EULA", Item::Bool(true)),
        ("IPL.UPT", Item::Byte(2)),
        ("IPL.IDL", Item::Small(vec![0, 1])),
        ("IPL.NIK", Item::Small(vec![0; 0x16])),
        ("IPL.PC", Item::Small(vec![0; 0x4A])),
        ("IPL.SADR", Item::Big(address)),
        ("BT.SENS", Item::Long(3)),
        ("BT.BAR", Item::Byte(0)), // sensor bar below the screen
        ("BT.MOT", Item::Byte(1)),
        ("BT.SPKV", Item::Byte(0x58)),
        ("BT.DINF", Item::Big(vec![0; 0x461])), // filled by `register_wiimotes`
        ("BT.CDIF", Item::Big(vec![0; 0x205])),
    ];
    let mut body = Vec::new();
    let mut offsets = Vec::new();
    let header_len = 4 + 2 + 2 * (items.len() + 1);
    for (name, item) in &items {
        offsets.push((header_len + body.len()) as u16);
        let ty = match item {
            Item::Big(_) => 1,
            Item::Small(_) => 2,
            Item::Byte(_) => 3,
            Item::Long(_) => 5,
            Item::Bool(_) => 7,
        };
        body.push((ty << 5) | (name.len() as u8 - 1));
        body.extend_from_slice(name.as_bytes());
        match item {
            Item::Byte(v) => body.push(*v),
            Item::Long(v) => body.extend_from_slice(&v.to_be_bytes()),
            Item::Bool(v) => body.push(*v as u8),
            Item::Small(d) => {
                body.push(d.len() as u8 - 1);
                body.extend_from_slice(d);
            }
            Item::Big(d) => {
                body.extend_from_slice(&(d.len() as u16 - 1).to_be_bytes());
                body.extend_from_slice(d);
            }
        }
    }
    offsets.push((header_len + body.len()) as u16);
    let mut out = Vec::with_capacity(0x4000);
    out.extend_from_slice(b"SCv0");
    out.extend_from_slice(&(items.len() as u16).to_be_bytes());
    for o in offsets {
        out.extend_from_slice(&o.to_be_bytes());
    }
    out.extend_from_slice(&body);
    out.resize(0x4000 - 4, 0);
    out.extend_from_slice(b"SCed");
    out
}

/// Length of the item at `at`, as the SDK's parser computes it: header,
/// name, the length field of arrays, and the data (no terminator).
fn sysconf_item_len(conf: &[u8], at: usize) -> Option<usize> {
    let &head = conf.get(at)?;
    let name = (head & 0x1F) as usize + 1;
    let p = at + 1 + name;
    let (field, data) = match head >> 5 {
        1 => (2, u16::from_be_bytes([*conf.get(p)?, *conf.get(p + 1)?]) as usize + 1),
        2 => (1, *conf.get(p)? as usize + 1),
        3 | 7 => (0, 1),
        4 => (0, 2),
        5 => (0, 4),
        6 => (0, 8),
        _ => return None,
    };
    Some(1 + name + field + data)
}

/// Whether the SDK accepts a `SYSCONF` image: the markers, and an offset
/// table in which every item ends where the next begins, clear of the last
/// 0x48 bytes (`SCInit`'s check, as reconstructed at `0x801DBEB0`).
pub fn sysconf_valid(conf: &[u8]) -> bool {
    if conf.len() != 0x4000 || &conf[..4] != b"SCv0" || &conf[0x3FFC..] != b"SCed" {
        return false;
    }
    let count = u16::from_be_bytes([conf[4], conf[5]]) as usize;
    let offset = |i: usize| conf.get(6 + 2 * i..8 + 2 * i).map(|o| u16::from_be_bytes([o[0], o[1]]) as usize);
    let mut at = 6 + 2 * (count + 1);
    for i in 0..count {
        if offset(i) != Some(at) {
            return false;
        }
        match sysconf_item_len(conf, at) {
            Some(n) => at += n,
            None => return false,
        }
    }
    offset(count) == Some(at) && at <= 0x4000 - 4 - 0x48
}

/// Offset of the value of item `name` with type `ty` in a `SYSCONF` image
/// (for arrays, of the length field).
fn sysconf_item(conf: &[u8], name: &str, ty: u8) -> Option<usize> {
    if conf.len() < 6 || &conf[..4] != b"SCv0" {
        return None;
    }
    let count = u16::from_be_bytes([conf[4], conf[5]]) as usize;
    for i in 0..count {
        let o = conf.get(6 + 2 * i..8 + 2 * i)?;
        let at = u16::from_be_bytes([o[0], o[1]]) as usize;
        let &head = conf.get(at)?;
        let len = (head & 0x1F) as usize + 1;
        if head >> 5 == ty && conf.get(at + 1..at + 1 + len) == Some(name.as_bytes()) {
            return Some(at + 1 + len);
        }
    }
    None
}

/// Sets the byte item `name` (such as `IPL.AR`) in a `SYSCONF` image.
/// Returns false when there is no such byte item.
pub fn set_sysconf_byte(conf: &mut [u8], name: &str, value: u8) -> bool {
    match sysconf_item(conf, name, 3).and_then(|at| conf.get_mut(at)) {
        Some(b) => {
            *b = value;
            true
        }
        None => false,
    }
}

/// Replaces the contents of the big-array item `name`, which must already
/// have `data`'s length. Returns false otherwise.
pub fn set_sysconf_big(conf: &mut [u8], name: &str, data: &[u8]) -> bool {
    let Some(at) = sysconf_item(conf, name, 1) else { return false };
    let Some(len) = conf.get(at..at + 2).map(|l| u16::from_be_bytes([l[0], l[1]]) as usize + 1) else { return false };
    match conf.get_mut(at + 2..at + 2 + len) {
        Some(dst) if len == data.len() => {
            dst.copy_from_slice(data);
            true
        }
        _ => false,
    }
}

/// The contents of the big-array item `name`.
pub fn sysconf_big<'a>(conf: &'a [u8], name: &str) -> Option<&'a [u8]> {
    let at = sysconf_item(conf, name, 1)?;
    let len = u16::from_be_bytes([*conf.get(at)?, *conf.get(at + 1)?]) as usize + 1;
    conf.get(at + 2..at + 2 + len)
}

/// `BT.DINF`: the paired Wii Remotes. A count, ten registered devices,
/// then the active ones; each is the address (most significant byte first)
/// and a 64-byte name. `remotes` holds HCI-order (least significant first)
/// addresses.
pub fn bt_dinf(remotes: &[[u8; 6]]) -> Vec<u8> {
    const ENTRY: usize = 0x46;
    let mut d = vec![0u8; 0x461];
    d[0] = remotes.len().min(10) as u8;
    let mut put = |slot: usize, addr: &[u8; 6]| {
        let at = 1 + ENTRY * slot;
        for (i, b) in addr.iter().rev().enumerate() {
            d[at + i] = *b;
        }
        d[at + 6..at + 6 + 19].copy_from_slice(b"Nintendo RVL-CNT-01");
    };
    for (i, a) in remotes.iter().enumerate().take(4) {
        put(i, a);
        put(10 + i, a);
    }
    d
}

/// Registers the emulated Wii Remotes in `SYSCONF` (as Dolphin does at every
/// start), so the Wii's Bluetooth stack accepts their connections.
pub fn register_wiimotes(root: &Path, remotes: &[[u8; 6]]) -> std::io::Result<()> {
    ensure_system_files(root)?;
    let path = root.join("shared2/sys/SYSCONF");
    let mut conf = std::fs::read(&path)?;
    let dinf = bt_dinf(remotes);
    if set_sysconf_big(&mut conf, "BT.DINF", &dinf) {
        std::fs::write(&path, conf)?;
    }
    Ok(())
}

/// Sets the console's aspect ratio (`IPL.AR`: 0 4:3, 1 16:9) in the NAND
/// under `root`, creating the system files first if needed. The game reads
/// it once at boot.
pub fn set_widescreen(root: &Path, widescreen: bool) -> std::io::Result<()> {
    ensure_system_files(root)?;
    let path = root.join("shared2/sys/SYSCONF");
    let mut conf = std::fs::read(&path)?;
    if set_sysconf_byte(&mut conf, "IPL.AR", u8::from(widescreen)) {
        std::fs::write(&path, conf)?;
    }
    Ok(())
}

/// Makes the console's `setting.txt` say the region of the game whose title
/// ID is `title_id` (its last letter), as a console that plays that disc
/// would: the game reads its area and game region from it.
pub fn set_region(root: &Path, title_id: u64) -> std::io::Result<()> {
    ensure_system_files(root)?;
    let path = root.join("title/00000001/00000002/data/setting.txt");
    let want = setting_txt_for(title_id as u8);
    if std::fs::read(&path).ok().as_deref() != Some(&want[..]) {
        std::fs::write(&path, want)?;
    }
    Ok(())
}

/// Sets the console's language in SYSCONF (`IPL.LNG`: 0 Japanese, 1
/// English, 2 German, 3 French, 4 Spanish, 5 Italian, 6 Dutch, 8
/// Traditional Chinese, 9 Korean), which the game reads at boot.
pub fn set_language(root: &Path, language: u8) -> std::io::Result<()> {
    ensure_system_files(root)?;
    let path = root.join("shared2/sys/SYSCONF");
    let mut conf = std::fs::read(&path)?;
    if set_sysconf_byte(&mut conf, "IPL.LNG", language) {
        std::fs::write(&path, conf)?;
    }
    Ok(())
}

/// The game's save file, its first 4 bytes the disc's game ID (`SMNE`).
const SAVE: &str = "wiimj2d.sav";
/// The save's banner in the system menu, the same for every version; the
/// game starts a new save without it.
const BANNER: &str = "banner.bin";

/// A title's data folder.
fn title_data(root: &Path, title_id: u64) -> PathBuf {
    root.join(format!("title/{:08x}/{:08x}/data", title_id >> 32, title_id as u32))
}

/// If the game whose title ID is `title_id` has no save yet, gives it a
/// copy of the newest save another version of the same game left (the
/// USA disc's for the European one, or the other way): the saves differ
/// only in the game ID's last letter. Returns the title copied from.
pub fn adopt_save(root: &Path, title_id: u64) -> std::io::Result<Option<u64>> {
    let ours = title_data(root, title_id);
    if ours.join(SAVE).exists() {
        return Ok(None);
    }
    let Ok(titles) = std::fs::read_dir(root.join(format!("title/{:08x}", title_id >> 32))) else { return Ok(None) };
    let mut newest: Option<(std::time::SystemTime, u64)> = None;
    for entry in titles.flatten() {
        let Some(low) = entry.file_name().to_str().and_then(|n| u32::from_str_radix(n, 16).ok()) else { continue };
        let other = (title_id & !0xFFFF_FFFF) | u64::from(low);
        // The same game (its ID's first three letters), another region.
        if other == title_id || low >> 8 != (title_id as u32) >> 8 {
            continue;
        }
        let dir = title_data(root, other);
        let Ok(modified) = std::fs::metadata(dir.join(SAVE)).and_then(|m| m.modified()) else { continue };
        if dir.join(BANNER).is_file() && newest.is_none_or(|(t, _)| modified > t) {
            newest = Some((modified, other));
        }
    }
    let Some((_, from)) = newest else { return Ok(None) };
    let dir = title_data(root, from);
    let mut save = std::fs::read(dir.join(SAVE))?;
    if save.len() < 4 || save[..4] != (from as u32).to_be_bytes() {
        return Ok(None);
    }
    save[3] = title_id as u8;
    std::fs::create_dir_all(&ours)?;
    std::fs::copy(dir.join(BANNER), ours.join(BANNER))?;
    std::fs::write(ours.join(SAVE), save)?;
    Ok(Some(from))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_version_adopts_the_other_ones_save() {
        let root = std::env::temp_dir().join(format!("nsmbw-adopt-save-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let (usa, pal) = (0x0001_0000_534d_4e45, 0x0001_0000_534d_4e50);
        // Nothing to adopt yet.
        assert_eq!(adopt_save(&root, pal).unwrap(), None);
        let usa_dir = title_data(&root, usa);
        std::fs::create_dir_all(&usa_dir).unwrap();
        std::fs::write(usa_dir.join(SAVE), b"SMNE\x0e\0\0\0save").unwrap();
        std::fs::write(usa_dir.join(BANNER), b"banner").unwrap();
        // Another game's save (another ID) is never taken.
        let other = title_data(&root, 0x0001_0000_5241_4245);
        std::fs::create_dir_all(&other).unwrap();
        std::fs::write(other.join(SAVE), b"RABE").unwrap();
        std::fs::write(other.join(BANNER), b"b").unwrap();
        assert_eq!(adopt_save(&root, pal).unwrap(), Some(usa));
        let pal_dir = title_data(&root, pal);
        assert_eq!(std::fs::read(pal_dir.join(SAVE)).unwrap(), b"SMNP\x0e\0\0\0save");
        assert_eq!(std::fs::read(pal_dir.join(BANNER)).unwrap(), b"banner");
        // Once it has its own, it keeps it.
        std::fs::write(pal_dir.join(SAVE), b"SMNP mine").unwrap();
        assert_eq!(adopt_save(&root, pal).unwrap(), None);
        assert_eq!(std::fs::read(pal_dir.join(SAVE)).unwrap(), b"SMNP mine");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn setting_txt_round_trips() {
        let enc = setting_txt();
        let mut key: u32 = 0x73B5_DBFA;
        let dec: Vec<u8> = enc
            .iter()
            .map(|b| {
                let d = b ^ key as u8;
                key = key.rotate_left(1);
                d
            })
            .collect();
        assert!(dec.starts_with(b"AREA=USA\r\n"));
        assert_eq!(enc.len(), 0x100);
        let pal = setting_txt_for(b'P');
        let mut key: u32 = 0x73B5_DBFA;
        let dec: Vec<u8> = pal.iter().map(|b| { let d = b ^ key as u8; key = key.rotate_left(1); d }).collect();
        assert!(dec.starts_with(b"AREA=EUR\r\nMODEL=RVL-001(EUR)\r\n"));
    }

    #[test]
    fn sysconf_layout() {
        let s = sysconf();
        assert_eq!(s.len(), 0x4000);
        assert_eq!(&s[..4], b"SCv0");
        assert_eq!(&s[0x3FFC..], b"SCed");
        let count = u16::from_be_bytes([s[4], s[5]]) as usize;
        let first = u16::from_be_bytes([s[6], s[7]]) as usize;
        assert_eq!(first, 6 + 2 * (count + 1));
        assert_eq!(s[first], (3 << 5) | 6); // IPL.LNG: byte, 7-char name
        assert_eq!(&s[first + 1..first + 8], b"IPL.LNG");
        assert!(sysconf_valid(&s));
        // The earlier layout (a null after each array) is rejected.
        let mut bad = s.clone();
        let at = bad.windows(7).position(|w| w == b"IPL.IDL").unwrap() + 7 + 3;
        bad.insert(at, 0);
        bad.truncate(0x3FFC);
        bad.extend_from_slice(b"SCed");
        assert!(!sysconf_valid(&bad));
    }

    #[test]
    fn wiimotes_are_registered() {
        let mut s = sysconf();
        let remote = [0x11, 0x02, 0x19, 0x79, 0x00, 0x01];
        assert!(set_sysconf_big(&mut s, "BT.DINF", &bt_dinf(&[remote])));
        assert_eq!(sysconf_big(&s, "BT.DINF"), Some(&bt_dinf(&[remote])[..]));
        assert!(sysconf_valid(&s));
        let at = s.windows(7).position(|w| w == b"BT.DINF").unwrap() + 7;
        assert_eq!(&s[at..at + 2], &[0x04, 0x60]);
        let d = &s[at + 2..at + 2 + 0x461];
        assert_eq!(d[0], 1);
        assert_eq!(&d[1..7], &[0x01, 0x00, 0x79, 0x19, 0x02, 0x11]);
        assert_eq!(&d[7..26], b"Nintendo RVL-CNT-01");
        assert_eq!(&d[1 + 0x46 * 10..7 + 0x46 * 10], &d[1..7], "also the first active slot");
    }

    #[test]
    fn sysconf_bytes_can_be_set() {
        let mut s = sysconf();
        assert!(set_sysconf_byte(&mut s, "IPL.AR", 0));
        assert!(!set_sysconf_byte(&mut s, "IPL.CB", 0), "a long, not a byte");
        let at = s.windows(6).position(|w| w == b"IPL.AR").unwrap();
        assert_eq!(s[at + 6], 0);
    }

    #[test]
    fn host_paths_stay_inside_root() {
        let r = Path::new("root");
        assert_eq!(host_path(r, "/shared2/sys/SYSCONF"), Some(r.join("shared2").join("sys").join("SYSCONF")));
        assert_eq!(host_path(r, "/a/../b"), None);
        assert_eq!(host_path(r, "relative"), None);
    }
}
