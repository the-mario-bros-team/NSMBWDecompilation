//! `--check`: validates an extracted disc folder (Dolphin "Extract Entire Disc")
//! of a version of the game the port runs, and for the USA revision 1 disc
//! compares its code against the image this project analysed
//! (`work/dol/main.elf`, `work/rels/*.rel`).

use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use game_version::Version;

const WII_DISC_MAGIC: u32 = 0x5D1C_9EA3;
const RELS: [&str; 4] = ["d_profileNP", "d_basesNP", "d_enemiesNP", "d_en_bossNP"];
const MEM1: std::ops::Range<u32> = 0x8000_0000..0x8180_0000;

#[derive(Default)]
struct Report {
    failures: u32,
}

impl Report {
    fn ok(&mut self, what: &str, detail: impl Display) {
        println!("  OK    {what}: {detail}");
    }
    fn fail(&mut self, what: &str, detail: impl Display) {
        println!("  FAIL  {what}: {detail}");
        self.failures += 1;
    }
    fn skip(&mut self, what: &str, detail: impl Display) {
        println!("  SKIP  {what}: {detail}");
    }
}

fn be32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn read(path: &Path, what: &str, r: &mut Report) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(b) => Some(b),
        Err(e) => {
            r.fail(what, format!("cannot read {}: {e}", path.display()));
            None
        }
    }
}

/// Returns `true` when every check passed.
pub fn check(game: &Path, work: &Path) -> bool {
    println!("game folder: {}", game.display());
    let mut r = Report::default();
    // The analysed image is USA revision 1's: other versions' code is not
    // compared with it.
    let usa = check_boot(game, &mut r) == Some(Version::E1);
    check_dol(game, Some(work).filter(|_| usa), &mut r);
    check_fst(game, &mut r);
    check_rels(game, Some(work).filter(|_| usa), &mut r);
    if r.failures == 0 {
        println!("result: OK");
    } else {
        println!("result: {} check(s) failed", r.failures);
    }
    r.failures == 0
}

/// The disc's version, if the port runs it.
fn check_boot(game: &Path, r: &mut Report) -> Option<Version> {
    let boot = read(&game.join("sys/boot.bin"), "sys/boot.bin", r)?;
    if boot.len() < 0x20 {
        r.fail("sys/boot.bin", "shorter than its 0x20-byte header");
        return None;
    }
    let id = String::from_utf8_lossy(&boot[..6]).into_owned();
    let rev = boot[7];
    if be32(&boot, 0x18) != Some(WII_DISC_MAGIC) {
        r.fail("sys/boot.bin", "no Wii disc magic at 0x18");
        return None;
    }
    match Version::from_boot_bin(&boot) {
        Some(v) => {
            r.ok("game id", format!("{id} rev {rev} ({})", v.describe()));
            Some(v)
        }
        None => {
            r.fail("game id", format!("{id} rev {rev}; the port runs New Super Mario Bros. Wii: SMNE01 or SMNP01 rev 1 or 2, SMNJ01 rev 1 or 2, SMNK01, SMNW01"));
            None
        }
    }
}

struct DolSection {
    name: String,
    offset: usize,
    addr: u32,
    size: u32,
}

/// `work`: the analysed image's folder, if the disc is the one it is of.
fn check_dol(game: &Path, work: Option<&Path>, r: &mut Report) {
    let Some(dol) = read(&game.join("sys/main.dol"), "sys/main.dol", r) else { return };
    let Some(entry) = be32(&dol, 0xE0) else {
        return r.fail("sys/main.dol", "shorter than its 0x100-byte header");
    };
    let mut sections = Vec::new();
    for (kind, count, off_base, addr_base, size_base) in
        [("text", 7, 0x00, 0x48, 0x90), ("data", 11, 0x1C, 0x64, 0xAC)]
    {
        for i in 0..count {
            let (Some(offset), Some(addr), Some(size)) = (
                be32(&dol, off_base + 4 * i),
                be32(&dol, addr_base + 4 * i),
                be32(&dol, size_base + 4 * i),
            ) else {
                return r.fail("sys/main.dol", "truncated header");
            };
            if size != 0 {
                sections.push(DolSection { name: format!("{kind}{i}"), offset: offset as usize, addr, size });
            }
        }
    }

    for s in &sections {
        if s.offset + s.size as usize > dol.len() {
            return r.fail("sys/main.dol", format!("{} runs past the end of the file", s.name));
        }
        if !MEM1.contains(&s.addr) || s.addr as u64 + s.size as u64 > MEM1.end as u64 {
            return r.fail("sys/main.dol", format!("{} at {:#010x} is outside MEM1", s.name, s.addr));
        }
    }
    let in_text = sections
        .iter()
        .any(|s| s.name.starts_with("text") && (s.addr..s.addr + s.size).contains(&entry));
    if !in_text {
        return r.fail("sys/main.dol", format!("entry {entry:#010x} is not inside a text section"));
    }
    r.ok(
        "sys/main.dol",
        format!("{} sections, entry {entry:#010x}, {} bytes", sections.len(), dol.len()),
    );

    let Some(work) = work else {
        return r.skip("code vs analysed image", "the analysed image is USA revision 1's");
    };
    let elf_path = work.join("dol/main.elf");
    let Ok(elf) = fs::read(&elf_path) else {
        return r.skip("code vs analysed image", format!("{} not found (run from D:\\nsmbw)", elf_path.display()));
    };
    match compare_with_elf(&dol, &sections, entry, &elf) {
        Ok(()) => r.ok("code vs analysed image", "every DOL section is byte-identical to work/dol/main.elf"),
        Err(e) => r.fail("code vs analysed image", e),
    }
}

fn compare_with_elf(dol: &[u8], sections: &[DolSection], entry: u32, elf: &[u8]) -> Result<(), String> {
    if elf.get(..6) != Some(&[0x7F, b'E', b'L', b'F', 1, 2][..]) {
        return Err("work/dol/main.elf is not a 32-bit big-endian ELF".into());
    }
    let elf_entry = be32(elf, 0x18).ok_or("ELF header truncated")?;
    if elf_entry != entry {
        return Err(format!("entry {entry:#010x} vs ELF {elf_entry:#010x}"));
    }
    let phoff = be32(elf, 0x1C).ok_or("ELF header truncated")? as usize;
    let half = |at: usize| elf.get(at..at + 2).map(|b| u16::from_be_bytes([b[0], b[1]]) as usize);
    let phentsize = half(0x2A).ok_or("ELF header truncated")?;
    let phnum = half(0x2C).ok_or("ELF header truncated")?;
    let mut loads = Vec::new();
    for i in 0..phnum {
        let ph = phoff + i * phentsize;
        let field = |o: usize| be32(elf, ph + o).ok_or("ELF program header truncated");
        if field(0)? == 1 {
            loads.push((field(4)? as usize, field(8)?, field(0x10)?)); // offset, vaddr, filesz
        }
    }
    for s in sections {
        let (p_off, vaddr, _) = loads
            .iter()
            .copied()
            .find(|&(_, v, sz)| v <= s.addr && s.addr as u64 + s.size as u64 <= v as u64 + sz as u64)
            .ok_or_else(|| format!("{} at {:#010x} has no ELF segment", s.name, s.addr))?;
        let start = p_off + (s.addr - vaddr) as usize;
        let theirs = elf
            .get(start..start + s.size as usize)
            .ok_or_else(|| format!("ELF segment for {} is truncated", s.name))?;
        if theirs != &dol[s.offset..s.offset + s.size as usize] {
            return Err(format!("{} at {:#010x} differs", s.name, s.addr));
        }
    }
    Ok(())
}

/// Walks `sys/fst.bin` and checks that every file exists under `files/` with its size.
fn check_fst(game: &Path, r: &mut Report) {
    let Some(fst) = read(&game.join("sys/fst.bin"), "sys/fst.bin", r) else { return };
    let Some(count) = be32(&fst, 8).map(|n| n as usize) else {
        return r.fail("sys/fst.bin", "shorter than its root entry");
    };
    let strings = count * 12;
    if count == 0 || strings > fst.len() {
        return r.fail("sys/fst.bin", format!("bad entry count {count}"));
    }
    let name_at = |off: usize| -> Option<String> {
        let rest = fst.get(strings + off..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        Some(String::from_utf8_lossy(&rest[..end]).into_owned())
    };

    let root = game.join("files");
    let mut stack: Vec<(usize, PathBuf)> = Vec::new();
    let (mut dirs, mut files, mut bytes) = (0usize, 0usize, 0u64);
    let mut problems = Vec::new();
    for i in 1..count {
        while stack.last().is_some_and(|&(end, _)| i >= end) {
            stack.pop();
        }
        let e = i * 12;
        let (Some(word0), Some(size)) = (be32(&fst, e), be32(&fst, e + 8)) else {
            return r.fail("sys/fst.bin", "entry table truncated");
        };
        let Some(name) = name_at((word0 & 0x00FF_FFFF) as usize) else {
            return r.fail("sys/fst.bin", format!("entry {i} has a bad name offset"));
        };
        let path = stack.last().map_or(root.clone(), |(_, p)| p.clone()).join(&name);
        if fst[e] == 1 {
            dirs += 1;
            stack.push((size as usize, path));
        } else {
            files += 1;
            bytes += size as u64;
            match fs::metadata(&path) {
                Ok(m) if m.len() == size as u64 => {}
                Ok(m) => problems.push(format!("{} is {} bytes, expected {size}", path.display(), m.len())),
                Err(_) => problems.push(format!("{} is missing", path.display())),
            }
        }
    }
    if problems.is_empty() {
        r.ok("files/ vs fst.bin", format!("{files} files in {dirs} folders, {bytes} bytes, all present"));
    } else {
        for p in problems.iter().take(5) {
            println!("        {p}");
        }
        r.fail("files/ vs fst.bin", format!("{} of {files} files missing or the wrong size", problems.len()));
    }
}

fn check_rels(game: &Path, work: Option<&Path>, r: &mut Report) {
    for name in RELS {
        let what = format!("rels/{name}.rel");
        let Some(packed) = read(&game.join(format!("files/rels/{name}.rel.LZ")), &what, r) else { continue };
        let rel = match lz11_decompress(&packed) {
            Ok(b) => b,
            Err(e) => {
                r.fail(&what, format!("cannot decompress: {e}"));
                continue;
            }
        };
        let (Some(id), Some(sections), Some(version)) = (be32(&rel, 0), be32(&rel, 0x0C), be32(&rel, 0x1C)) else {
            r.fail(&what, "shorter than a REL header");
            continue;
        };
        if version != 3 || sections == 0 {
            r.fail(&what, format!("unexpected header: version {version}, {sections} sections"));
            continue;
        }
        let summary = format!("module {id}, {sections} sections, {} bytes", rel.len());
        let Some(work) = work else {
            r.ok(&what, summary);
            continue;
        };
        match fs::read(work.join(format!("rels/{name}.rel"))) {
            Ok(ours) if ours == rel => r.ok(&what, format!("{summary}, identical to work/rels")),
            Ok(_) => r.fail(&what, format!("{summary}, but differs from work/rels/{name}.rel")),
            Err(_) => r.skip(&what, format!("{summary}; work/rels/{name}.rel not found to compare")),
        }
    }
}

/// Nintendo LZ77 type 0x11 ("LZ11"), the format of the disc's `*.rel.LZ` files.
fn lz11_decompress(src: &[u8]) -> Result<Vec<u8>, String> {
    if src.first() != Some(&0x11) {
        return Err(format!("not LZ11 (type byte {:#04x})", src.first().copied().unwrap_or(0)));
    }
    let mut pos = 4;
    let mut size = u32::from_le_bytes([src[1], src[2], src[3], 0]) as usize;
    if size == 0 {
        size = be32(src, 4).map(u32::swap_bytes).ok_or("truncated size")? as usize;
        pos = 8;
    }
    let mut out = Vec::with_capacity(size);
    let next = |pos: &mut usize| -> Result<usize, String> {
        let b = *src.get(*pos).ok_or("input ends early")?;
        *pos += 1;
        Ok(b as usize)
    };
    while out.len() < size {
        let flags = next(&mut pos)?;
        for bit in (0..8).rev() {
            if out.len() >= size {
                break;
            }
            if flags & (1 << bit) == 0 {
                out.push(next(&mut pos)? as u8);
                continue;
            }
            let b1 = next(&mut pos)?;
            let (len, disp) = match b1 >> 4 {
                0 => {
                    let (b2, b3) = (next(&mut pos)?, next(&mut pos)?);
                    ((((b1 & 0xF) << 4) | (b2 >> 4)) + 0x11, (((b2 & 0xF) << 8) | b3) + 1)
                }
                1 => {
                    let (b2, b3, b4) = (next(&mut pos)?, next(&mut pos)?, next(&mut pos)?);
                    ((((b1 & 0xF) << 12) | (b2 << 4) | (b3 >> 4)) + 0x111, (((b3 & 0xF) << 8) | b4) + 1)
                }
                n => {
                    let b2 = next(&mut pos)?;
                    (n + 1, (((b1 & 0xF) << 8) | b2) + 1)
                }
            };
            if disp > out.len() {
                return Err("back-reference before the start of the output".into());
            }
            for _ in 0..len {
                out.push(out[out.len() - disp]);
            }
        }
    }
    out.truncate(size);
    Ok(out)
}
