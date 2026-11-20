//! The game partition as `/dev/di` sees it, served from the extracted folder.
//!
//! Wii disc offsets are partition-relative and given in 4-byte units. The
//! layout is rebuilt from `sys/boot.bin` (DOL and FST offsets) and `fst.bin`
//! (file offsets), so every byte the game can ask for maps to one host file.
//!
//! A file can be served changed ([`Disc::replace`]): its new bytes are kept
//! in memory past the disc's last file, and the FST's entry points there
//! (the FST in RAM comes from [`Disc::fst`]). The original stays where it
//! was, for a game whose FST came from a save state.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

struct Range {
    start: u64,
    len: u64,
    path: PathBuf,
    /// The bytes, for a file served from memory.
    data: Option<Vec<u8>>,
}

pub struct Disc {
    ranges: Vec<Range>,
    pub header: Vec<u8>,
    /// `sys/fst.bin` with the replaced files' entries.
    fst: Vec<u8>,
    /// Each file's disc path ("/Layout/pauseMenu/pauseMenu.arc") and FST entry.
    files: Vec<(String, usize)>,
}

fn be32(b: &[u8], at: usize) -> u32 {
    u32::from_be_bytes(b[at..at + 4].try_into().unwrap())
}

impl Disc {
    pub fn open(game: &Path) -> Result<Self, String> {
        let sys = game.join("sys");
        let read = |p: &Path| std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()));
        let header = read(&sys.join("boot.bin"))?;
        let fst = read(&sys.join("fst.bin"))?;
        let mut ranges = Vec::new();
        let mut add = |start: u64, path: PathBuf| -> Result<(), String> {
            let len = std::fs::metadata(&path).map_err(|e| format!("{}: {e}", path.display()))?.len();
            ranges.push(Range { start, len, path, data: None });
            Ok(())
        };
        add(0, sys.join("boot.bin"))?;
        add(0x440, sys.join("bi2.bin"))?;
        add(0x2440, sys.join("apploader.img"))?;
        add(u64::from(be32(&header, 0x420)) << 2, sys.join("main.dol"))?;
        add(u64::from(be32(&header, 0x424)) << 2, sys.join("fst.bin"))?;

        // FST: 12-byte entries; the root's size field is the entry count.
        let count = be32(&fst, 8) as usize;
        let names = count * 12;
        let name = |i: usize| -> String {
            let off = names + (be32(&fst, i * 12) & 0x00FF_FFFF) as usize;
            let end = fst[off..].iter().position(|&b| b == 0).map_or(fst.len(), |n| off + n);
            String::from_utf8_lossy(&fst[off..end]).into_owned()
        };
        // Directory stack of (end index, host path, disc path).
        let mut dirs: Vec<(usize, PathBuf, String)> = vec![(count, game.join("files"), String::new())];
        let mut files = Vec::new();
        for i in 1..count {
            while dirs.last().is_some_and(|d| i >= d.0) {
                dirs.pop();
            }
            let (_, parent, disc_parent) = dirs.last().cloned().ok_or("FST: entry outside root")?;
            let e = &fst[i * 12..i * 12 + 12];
            let disc_path = format!("{disc_parent}/{}", name(i));
            if e[0] != 0 {
                dirs.push((be32(e, 8) as usize, parent.join(name(i)), disc_path));
            } else {
                ranges.push(Range { start: u64::from(be32(e, 4)) << 2, len: u64::from(be32(e, 8)), path: parent.join(name(i)), data: None });
                files.push((disc_path, i));
            }
        }
        ranges.sort_by_key(|r| r.start);
        Ok(Self { ranges, header, fst, files })
    }

    /// The FST as the game should have it in RAM.
    pub fn fst(&self) -> &[u8] {
        &self.fst
    }

    /// FST entry of the file at disc path `path` (any case).
    fn entry(&self, path: &str) -> Option<usize> {
        self.files.iter().find(|(p, _)| p.eq_ignore_ascii_case(path)).map(|f| f.1)
    }

    /// The bytes of the file at disc path `path`, as the game would read them.
    pub fn file(&self, path: &str) -> Option<Vec<u8>> {
        let e = self.entry(path)? * 12;
        let mut out = vec![0; be32(&self.fst, e + 8) as usize];
        self.read(u64::from(be32(&self.fst, e + 4)) << 2, &mut out).then_some(out)
    }

    /// Serves `data` as the file at disc path `path` from now on. False if
    /// the disc has no such file.
    pub fn replace(&mut self, path: &str, data: Vec<u8>) -> bool {
        let Some(e) = self.entry(path).map(|e| e * 12) else { return false };
        let end = self.ranges.iter().map(|r| r.start + r.len).max().unwrap_or(0);
        let start = (end + 0x7FFF) & !0x7FFF;
        let Ok(units) = u32::try_from(start >> 2) else { return false };
        self.fst[e + 4..e + 8].copy_from_slice(&units.to_be_bytes());
        self.fst[e + 8..e + 12].copy_from_slice(&(data.len() as u32).to_be_bytes());
        self.ranges.push(Range { start, len: data.len() as u64, path: PathBuf::from(path.trim_start_matches('/')), data: Some(data) });
        // fst.bin itself is served as changed.
        let fst_at = u64::from(be32(&self.header, 0x424)) << 2;
        if let Some(r) = self.ranges.iter_mut().find(|r| r.start == fst_at) {
            r.data = Some(self.fst.clone());
        }
        self.ranges.sort_by_key(|r| r.start);
        true
    }

    /// The host file holding partition byte `offset`.
    pub fn file_at(&self, offset: u64) -> Option<&Path> {
        let i = self.ranges.partition_point(|r| r.start + r.len <= offset);
        self.ranges.get(i).filter(|r| r.start <= offset).map(|r| r.path.as_path())
    }

    /// Fills `out` from partition byte offset `offset`. Bytes no file covers
    /// read as zero. Returns false if a host file could not be read.
    pub fn read(&self, offset: u64, out: &mut [u8]) -> bool {
        out.fill(0);
        let end = offset + out.len() as u64;
        let first = self.ranges.partition_point(|r| r.start + r.len <= offset);
        for r in &self.ranges[first..] {
            if r.start >= end {
                break;
            }
            let from = offset.max(r.start);
            let to = end.min(r.start + r.len);
            if from >= to {
                continue;
            }
            let dst = &mut out[(from - offset) as usize..(to - offset) as usize];
            if let Some(data) = &r.data {
                dst.copy_from_slice(&data[(from - r.start) as usize..(to - r.start) as usize]);
                continue;
            }
            let ok = File::open(&r.path)
                .and_then(|mut f| {
                    f.seek(SeekFrom::Start(from - r.start))?;
                    f.read_exact(dst)
                })
                .is_ok();
            if !ok {
                return false;
            }
        }
        true
    }
}
