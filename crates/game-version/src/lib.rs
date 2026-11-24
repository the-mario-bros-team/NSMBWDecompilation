//! Which New Super Mario Bros. Wii disc the port runs, and where that
//! version keeps what the port's code names by its USA revision 1 address.
//!
//! The port's hooks, patches and memory reads were written against the USA
//! disc (SMNE01 revision 1, "E1"). They name the game's functions and data
//! by E1 addresses ([`E1`]); [`E1::va`] gives the address in the version
//! that runs ([`set`]), through NSMBW-Updated's address map
//! (`address-map.txt`, MIT, by Ninji, CLF78 and RoadrunnerWMC; its notice
//! is in `LICENSE-address-map.txt`). That map's base is PAL revision 1
//! ("P1"): each other version's block maps P1 addresses (or those of the
//! version it extends) to its own, range by range. An E1 address is taken
//! back to P1 through the inverse of the E1 block, then forward to the
//! version. An address no range covers has no counterpart there.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

/// A release of the game: the address map's block names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Version {
    /// USA, revision 1 (SMNE01), the one the port was written for.
    E1,
    /// USA, revision 2.
    E2,
    /// Europe and Australia, revision 1 (SMNP01).
    P1,
    /// Europe and Australia, revision 2.
    P2,
    /// Japan, revision 1 (SMNJ01).
    J1,
    /// Japan, revision 2.
    J2,
    /// South Korea (SMNK01).
    K,
    /// Taiwan (SMNW01).
    W,
}

/// The console region a version is for: what the system menu's
/// `setting.txt` says, and the languages its disc has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    Usa,
    Europe,
    Japan,
    Korea,
    Taiwan,
}

impl Version {
    pub const ALL: [Version; 8] = [Version::E1, Version::E2, Version::P1, Version::P2, Version::J1, Version::J2, Version::K, Version::W];

    /// The version a disc is: its game ID (`sys/boot.bin` bytes 0-5) and
    /// revision (byte 7). Korea and Taiwan had one release each.
    pub fn from_disc(id: &[u8], revision: u8) -> Option<Self> {
        Some(match (id, revision) {
            (b"SMNE01", 1) => Version::E1,
            (b"SMNE01", 2) => Version::E2,
            (b"SMNP01", 1) => Version::P1,
            (b"SMNP01", 2) => Version::P2,
            (b"SMNJ01", 1) => Version::J1,
            (b"SMNJ01", 2) => Version::J2,
            (b"SMNK01", _) => Version::K,
            (b"SMNW01", _) => Version::W,
            _ => return None,
        })
    }

    /// The version of an extracted disc's `sys/boot.bin`.
    pub fn from_boot_bin(boot: &[u8]) -> Option<Self> {
        Self::from_disc(boot.get(..6)?, *boot.get(7)?)
    }

    /// Short name, as the address map and the port's files call it: "E1".
    pub fn name(self) -> &'static str {
        ["E1", "E2", "P1", "P2", "J1", "J2", "K", "W"][self as usize]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|v| v.name().eq_ignore_ascii_case(name))
    }

    pub fn game_id(self) -> &'static str {
        match self.region() {
            Region::Usa => "SMNE01",
            Region::Europe => "SMNP01",
            Region::Japan => "SMNJ01",
            Region::Korea => "SMNK01",
            Region::Taiwan => "SMNW01",
        }
    }

    /// The disc revision; `None` for the versions released once.
    pub fn revision(self) -> Option<u8> {
        match self {
            Version::E1 | Version::P1 | Version::J1 => Some(1),
            Version::E2 | Version::P2 | Version::J2 => Some(2),
            Version::K | Version::W => None,
        }
    }

    pub fn region(self) -> Region {
        match self {
            Version::E1 | Version::E2 => Region::Usa,
            Version::P1 | Version::P2 => Region::Europe,
            Version::J1 | Version::J2 => Region::Japan,
            Version::K => Region::Korea,
            Version::W => Region::Taiwan,
        }
    }

    /// The title ID: 00010000 then the game ID's first four letters.
    pub fn title_id(self) -> u64 {
        let id = self.game_id().as_bytes();
        0x0001_0000_0000_0000 | u64::from(u32::from_be_bytes([id[0], id[1], id[2], id[3]]))
    }

    /// "USA, revision 1", "Europe, revision 2", "South Korea".
    pub fn describe(self) -> String {
        let region = match self.region() {
            Region::Usa => "USA",
            Region::Europe => "Europe",
            Region::Japan => "Japan",
            Region::Korea => "South Korea",
            Region::Taiwan => "Taiwan",
        };
        match self.revision() {
            Some(r) => format!("{region}, revision {r}"),
            None => region.to_string(),
        }
    }

    /// The versions between P1 and this one, P1 first: each maps the
    /// previous one's addresses to its own.
    fn chain(self) -> &'static [Version] {
        match self {
            Version::P1 => &[],
            Version::E1 => &[Version::E1],
            Version::E2 => &[Version::E1, Version::E2],
            Version::J1 => &[Version::J1],
            Version::J2 => &[Version::J1, Version::J2],
            Version::P2 => &[Version::P2],
            Version::K => &[Version::P2, Version::K],
            Version::W => &[Version::P2, Version::K, Version::W],
        }
    }
}

impl Region {
    /// The languages the game's discs for this region have, as the
    /// console's `IPL.LNG` numbers them (1 English, 2 German, 3 French,
    /// 4 Spanish, 5 Italian, 0 Japanese, 9 Korean, 8 Traditional Chinese).
    /// (The European disc's Dutch folder holds only the strap screen.)
    /// The first is the default.
    pub fn languages(self) -> &'static [u8] {
        match self {
            Region::Usa => &[1, 3, 4],
            Region::Europe => &[1, 2, 3, 4, 5],
            Region::Japan => &[0],
            Region::Korea => &[9],
            Region::Taiwan => &[8],
        }
    }
}

/// One line of a block: `start..=end` of the previous version's addresses
/// move by `delta`.
#[derive(Clone, Copy, Debug)]
struct Range {
    start: u32,
    end: u32,
    delta: i64,
}

impl Range {
    fn map(&self, a: u32) -> Option<u32> {
        (self.start..=self.end).contains(&a).then(|| (i64::from(a) + self.delta) as u32)
    }
}

/// The address map, parsed: each block's ranges, sorted by start.
struct Map {
    blocks: Vec<(Version, Vec<Range>)>,
    /// The E1 block turned around: E1 address ranges back to P1.
    e1_to_p1: Vec<Range>,
}

const ADDRESS_MAP: &str = include_str!("../address-map.txt");

fn parse_hex(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok()
}

fn parse(text: &str) -> Result<Map, String> {
    let mut blocks: Vec<(Version, Vec<Range>)> = Vec::new();
    let mut current: Option<usize> = None;
    for (n, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with("extend ") {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            // China's block (C) has no disc: its lines are skipped.
            current = Version::from_name(name).map(|v| {
                blocks.push((v, Vec::new()));
                blocks.len() - 1
            });
            continue;
        }
        let Some(i) = current else { continue };
        let bad = || format!("address map line {}: {raw}", n + 1);
        let (range, delta) = line.split_once(':').ok_or_else(bad)?;
        let (start, end) = range.split_once('-').ok_or_else(bad)?;
        let start = parse_hex(start).ok_or_else(bad)?;
        let end = if end.trim() == "*" { u32::MAX } else { parse_hex(end).ok_or_else(bad)? };
        let delta = delta.trim();
        let (sign, magnitude) = match delta.as_bytes().first() {
            Some(b'+') => (1, &delta[1..]),
            Some(b'-') => (-1, &delta[1..]),
            _ => return Err(bad()),
        };
        let delta = sign * i64::from(parse_hex(magnitude).ok_or_else(bad)?);
        blocks[i].1.push(Range { start, end, delta });
    }
    for (_, ranges) in &mut blocks {
        ranges.sort_by_key(|r| r.start);
    }
    let e1 = &blocks.iter().find(|b| b.0 == Version::E1).ok_or("no E1 block")?.1;
    let mut e1_to_p1: Vec<Range> = e1
        .iter()
        .map(|r| Range { start: (i64::from(r.start) + r.delta) as u32, end: (i64::from(r.end) + r.delta).min(i64::from(u32::MAX)) as u32, delta: -r.delta })
        .collect();
    e1_to_p1.sort_by_key(|r| r.start);
    Ok(Map { blocks, e1_to_p1 })
}

fn map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| parse(ADDRESS_MAP).expect("the built-in address map parses"))
}

fn find(ranges: &[Range], a: u32) -> Option<u32> {
    // The last range starting at or below `a` (ranges do not overlap).
    let i = ranges.partition_point(|r| r.start <= a);
    ranges[..i].last()?.map(a)
}

/// The address in `to` of what E1 has at `a`, or `None` if `to` has no
/// counterpart of it.
pub fn translate(a: u32, to: Version) -> Option<u32> {
    if to == Version::E1 {
        return Some(a);
    }
    let m = map();
    let mut a = find(&m.e1_to_p1, a)?;
    for v in to.chain() {
        let ranges = &m.blocks.iter().find(|b| b.0 == *v)?.1;
        a = find(ranges, a)?;
    }
    Some(a)
}

static CURRENT: AtomicU8 = AtomicU8::new(Version::E1 as u8);

/// Chooses the version that runs (at boot, before any address is asked for).
pub fn set(v: Version) {
    CURRENT.store(v as u8, Ordering::Relaxed);
}

/// The version that runs ([`set`]; E1 until then).
pub fn current() -> Version {
    Version::ALL[CURRENT.load(Ordering::Relaxed) as usize]
}

/// An address in USA revision 1's memory: a function, an instruction or
/// data of the game as SMNE01 revision 1 has it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct E1(pub u32);

impl E1 {
    /// The address of the same thing in the version that runs. Panics if
    /// that version has no counterpart: the port's code names only what
    /// every supported version has (`tests` below check them).
    #[inline]
    pub fn va(self) -> u32 {
        let v = current();
        if v == Version::E1 {
            return self.0;
        }
        translate(self.0, v).unwrap_or_else(|| panic!("{:#010x} (USA revision 1) has no counterpart in {}", self.0, v.name()))
    }

    /// The same, or `None` if the version that runs has no counterpart.
    pub fn try_va(self) -> Option<u32> {
        translate(self.0, current())
    }
}

impl std::ops::Add<u32> for E1 {
    type Output = E1;
    fn add(self, o: u32) -> E1 {
        E1(self.0 + o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_map_parses_with_every_version() {
        let m = map();
        for v in Version::ALL {
            if v != Version::P1 {
                assert!(m.blocks.iter().any(|b| b.0 == v && !b.1.is_empty()), "{v:?}");
            }
        }
    }

    #[test]
    fn e1_addresses_go_to_p1() {
        // Before the first shift, the same; later main.text moves (here by
        // 0xF0: E1 = P1 - 0xF0), and so does the small data; RELs do not.
        assert_eq!(translate(0x8000_4050, Version::P1), Some(0x8000_4050));
        assert_eq!(translate(0x800C_9F90, Version::P1), Some(0x800C_A080));
        assert_eq!(translate(0x8042_F680, Version::P1), Some(0x8042_F960));
        assert_eq!(translate(0x8077_AD70, Version::P1), Some(0x8077_AD70));
        assert_eq!(translate(0x8042_F680, Version::E1), Some(0x8042_F680));
    }

    #[test]
    fn discs_name_their_version() {
        assert_eq!(Version::from_disc(b"SMNP01", 1), Some(Version::P1));
        assert_eq!(Version::from_disc(b"SMNE01", 0), None);
        assert_eq!(Version::P1.title_id(), 0x0001_0000_534D_4E50);
        assert_eq!(Version::E1.title_id(), 0x0001_0000_534D_4E45);
        assert_eq!(Version::K.describe(), "South Korea");
    }
}
