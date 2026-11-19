//! The game partition as `/dev/di` sees it, served from the extracted folder.
//!
//! Wii disc offsets are partition-relative and given in 4-byte units. The
//! layout is rebuilt from `sys/boot.bin` (DOL and FST offsets) and `fst.bin`
//! (file offsets), so every byte the game can ask for maps to one host file.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

struct Range {
    start: u64,
    len: u64,
    path: PathBuf,
}

pub struct Disc {
    ranges: Vec<Range>,
    pub header: Vec<u8>,
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
            ranges.push(Range { start, len, path });
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
        // Directory stack of (end index, path).
        let mut dirs: Vec<(usize, PathBuf)> = vec![(count, game.join("files"))];
        for i in 1..count {
            while dirs.last().is_some_and(|d| i >= d.0) {
                dirs.pop();
            }
            let parent = dirs.last().map(|d| d.1.clone()).ok_or("FST: entry outside root")?;
            let e = &fst[i * 12..i * 12 + 12];
            if e[0] != 0 {
                dirs.push((be32(e, 8) as usize, parent.join(name(i))));
            } else {
                ranges.push(Range { start: u64::from(be32(e, 4)) << 2, len: u64::from(be32(e, 8)), path: parent.join(name(i)) });
            }
        }
        ranges.sort_by_key(|r| r.start);
        Ok(Self { ranges, header })
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
