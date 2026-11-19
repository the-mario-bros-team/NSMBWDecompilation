//! Dolphin FIFO logs (`.dff`, file version 1..6), the P3 reference input:
//! the GX state at the start (BP, CP, XF and TMEM snapshots) and, per
//! frame, the FIFO bytes plus the RAM updates that must land at given FIFO
//! positions. Layout and replay order follow Dolphin's
//! `Core/FifoPlayer/FifoDataFile.cpp` and `FifoPlayer.cpp` (packed,
//! little-endian structs).

pub const FILE_ID: u32 = 0x0D01_F1F0;
const HEADER: usize = 128;
const FRAME_INFO: usize = 64;
const MEMORY_UPDATE: usize = 24;
pub const BP_SIZE: usize = 256;
pub const CP_SIZE: usize = 256;
pub const XF_MEM_SIZE: usize = 4096;
pub const XF_REGS_SIZE: usize = 88;
pub const TMEM_SIZE: usize = 1 << 20;
const FLAG_IS_WII: u32 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryUpdate {
    /// Offset in the frame's FIFO data before which the bytes must be in RAM.
    pub fifo_position: u32,
    /// Physical address.
    pub address: u32,
    pub data: Vec<u8>,
    /// 1 texture, 2 XF data, 4 vertex stream, 8 TMEM.
    pub kind: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Frame {
    pub fifo: Vec<u8>,
    pub updates: Vec<MemoryUpdate>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FifoLog {
    pub version: u32,
    pub is_wii: bool,
    pub game_id: String,
    pub bp: Vec<u32>,
    pub cp: Vec<u32>,
    pub xf_mem: Vec<u32>,
    pub xf_regs: Vec<u32>,
    pub tmem: Vec<u8>,
    pub frames: Vec<Frame>,
}

fn u32_at(b: &[u8], at: usize) -> Result<u32, String> {
    b.get(at..at + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap())).ok_or_else(|| format!("truncated at {at:#x}"))
}

fn u64_at(b: &[u8], at: usize) -> Result<usize, String> {
    b.get(at..at + 8).map(|s| u64::from_le_bytes(s.try_into().unwrap()) as usize).ok_or_else(|| format!("truncated at {at:#x}"))
}

fn bytes(b: &[u8], at: usize, len: usize) -> Result<&[u8], String> {
    b.get(at..at.checked_add(len).ok_or("bad length")?).ok_or_else(|| format!("{len} bytes at {at:#x} run past the end"))
}

fn words(b: &[u8], at: usize, count: usize, max: usize) -> Result<Vec<u32>, String> {
    let mut v = vec![0; max];
    for (i, w) in v.iter_mut().enumerate().take(count.min(max)) {
        *w = u32_at(b, at + 4 * i)?;
    }
    Ok(v)
}

pub fn parse(b: &[u8]) -> Result<FifoLog, String> {
    if u32_at(b, 0)? != FILE_ID {
        return Err("not a Dolphin FIFO log (bad file id)".into());
    }
    let version = u32_at(b, 4)?;
    let min_loader = u32_at(b, 8)?;
    if min_loader > 6 {
        return Err(format!("needs FIFO player version {min_loader} (this reads up to 6)"));
    }
    let bp = words(b, u64_at(b, 12)?, u32_at(b, 20)? as usize, BP_SIZE)?;
    let cp = words(b, u64_at(b, 24)?, u32_at(b, 32)? as usize, CP_SIZE)?;
    let xf_mem = words(b, u64_at(b, 36)?, u32_at(b, 44)? as usize, XF_MEM_SIZE)?;
    let xf_regs = words(b, u64_at(b, 48)?, u32_at(b, 56)? as usize, XF_REGS_SIZE)?;
    let (frame_list, frame_count, flags) = (u64_at(b, 60)?, u32_at(b, 68)? as usize, u32_at(b, 72)?);
    let mut tmem = vec![0u8; TMEM_SIZE];
    if version >= 4 {
        let (at, len) = (u64_at(b, 76)?, (u32_at(b, 84)? as usize).min(TMEM_SIZE));
        tmem[..len].copy_from_slice(bytes(b, at, len)?);
    }
    let game_id = if version >= 6 { String::from_utf8_lossy(bytes(b, 96, 8)?).trim_end_matches('\0').to_string() } else { String::new() };
    let mut frames = Vec::with_capacity(frame_count);
    for i in 0..frame_count {
        let f = frame_list + i * FRAME_INFO;
        let fifo = bytes(b, u64_at(b, f)?, u32_at(b, f + 8)? as usize)?.to_vec();
        let (list, count) = (u64_at(b, f + 20)?, u32_at(b, f + 28)? as usize);
        let mut updates = Vec::with_capacity(count);
        for u in 0..count {
            let m = list + u * MEMORY_UPDATE;
            let data = bytes(b, u64_at(b, m + 8)?, u32_at(b, m + 16)? as usize)?.to_vec();
            updates.push(MemoryUpdate { fifo_position: u32_at(b, m)?, address: u32_at(b, m + 4)?, data, kind: *b.get(m + 20).ok_or("truncated")? });
        }
        frames.push(Frame { fifo, updates });
    }
    Ok(FifoLog { version, is_wii: flags & FLAG_IS_WII != 0, game_id, bp, cp, xf_mem, xf_regs, tmem, frames })
}

/// Writes a version-6 log (for tests and for recording our own streams).
pub fn write(log: &FifoLog) -> Vec<u8> {
    let mut out = vec![0u8; HEADER + log.frames.len() * FRAME_INFO];
    let put_words = |out: &mut Vec<u8>, w: &[u32]| -> usize {
        let at = out.len();
        for v in w {
            out.extend(v.to_le_bytes());
        }
        at
    };
    let bp = put_words(&mut out, &log.bp);
    let cp = put_words(&mut out, &log.cp);
    let xf_mem = put_words(&mut out, &log.xf_mem);
    let xf_regs = put_words(&mut out, &log.xf_regs);
    let tmem = out.len();
    out.extend(&log.tmem);
    for (i, f) in log.frames.iter().enumerate() {
        let data = out.len();
        out.extend(&f.fifo);
        let list = out.len();
        out.resize(list + f.updates.len() * MEMORY_UPDATE, 0);
        for (u, m) in f.updates.iter().enumerate() {
            let at = out.len();
            out.extend(&m.data);
            let e = list + u * MEMORY_UPDATE;
            out[e..e + 4].copy_from_slice(&m.fifo_position.to_le_bytes());
            out[e + 4..e + 8].copy_from_slice(&m.address.to_le_bytes());
            out[e + 8..e + 16].copy_from_slice(&(at as u64).to_le_bytes());
            out[e + 16..e + 20].copy_from_slice(&(m.data.len() as u32).to_le_bytes());
            out[e + 20] = m.kind;
        }
        let fi = HEADER + i * FRAME_INFO;
        out[fi..fi + 8].copy_from_slice(&(data as u64).to_le_bytes());
        out[fi + 8..fi + 12].copy_from_slice(&(f.fifo.len() as u32).to_le_bytes());
        out[fi + 16..fi + 20].copy_from_slice(&(f.fifo.len() as u32).to_le_bytes());
        out[fi + 20..fi + 28].copy_from_slice(&(list as u64).to_le_bytes());
        out[fi + 28..fi + 32].copy_from_slice(&(f.updates.len() as u32).to_le_bytes());
    }
    let mut h = Vec::with_capacity(HEADER);
    h.extend(FILE_ID.to_le_bytes());
    h.extend(6u32.to_le_bytes());
    h.extend(1u32.to_le_bytes());
    for (at, n) in [(bp, log.bp.len()), (cp, log.cp.len()), (xf_mem, log.xf_mem.len()), (xf_regs, log.xf_regs.len())] {
        h.extend((at as u64).to_le_bytes());
        h.extend((n as u32).to_le_bytes());
    }
    h.extend((HEADER as u64).to_le_bytes());
    h.extend((log.frames.len() as u32).to_le_bytes());
    h.extend(u32::from(log.is_wii).to_le_bytes());
    h.extend((tmem as u64).to_le_bytes());
    h.extend((log.tmem.len() as u32).to_le_bytes());
    h.extend(0x0180_0000u32.to_le_bytes());
    h.extend(0x0400_0000u32.to_le_bytes());
    let mut id = log.game_id.as_bytes().to_vec();
    id.resize(8, 0);
    h.extend(id);
    h.resize(HEADER, 0);
    out[..HEADER].copy_from_slice(&h);
    out
}

impl FifoLog {
    /// The GX commands that put the GPU in the log's starting state, as
    /// Dolphin's `FifoPlayer::LoadRegisters` and `ClearEfb` issue them
    /// (TMEM is loaded separately).
    pub fn setup_stream(&self) -> Vec<u8> {
        let mut s = Vec::new();
        let bp = |s: &mut Vec<u8>, reg: u8, v: u32| {
            s.push(0x61);
            s.extend((u32::from(reg) << 24 | (v & 0xFF_FFFF)).to_be_bytes());
        };
        let cp = |s: &mut Vec<u8>, reg: u8, v: u32| {
            s.extend([0x08, reg]);
            s.extend(v.to_be_bytes());
        };
        // Not loaded: draw done, PE tokens, the copy trigger, TLUT load,
        // preload mode and a perf register (they would act, not set state).
        for (i, &v) in self.bp.iter().enumerate() {
            if !matches!(i, 0x45 | 0x47 | 0x48 | 0x52 | 0x65 | 0x63 | 0x67) {
                bp(&mut s, i as u8, v);
            }
        }
        for reg in [0x30u8, 0x40, 0x50, 0x60] {
            cp(&mut s, reg, self.cp[reg as usize]);
        }
        for i in 0..8u8 {
            for base in [0x70u8, 0x80, 0x90] {
                cp(&mut s, base + i, self.cp[(base + i) as usize]);
            }
        }
        for i in 0..16u8 {
            for base in [0xA0u8, 0xB0] {
                cp(&mut s, base + i, self.cp[(base + i) as usize]);
            }
        }
        for (i, chunk) in self.xf_mem.chunks(16).enumerate() {
            s.push(0x10);
            s.extend(((chunk.len() as u32 - 1) << 16 | (16 * i as u32)).to_be_bytes());
            for v in chunk {
                s.extend(v.to_be_bytes());
            }
        }
        for (i, v) in self.xf_regs.iter().enumerate() {
            s.push(0x10);
            s.extend((0x1000 + i as u32).to_be_bytes());
            s.extend(v.to_be_bytes());
        }
        // A copy that clears the whole EFB, then the copy registers restored.
        bp(&mut s, 0x49, 0);
        bp(&mut s, 0x4A, 639 | (527 << 10));
        bp(&mut s, 0x4D, 0x140);
        bp(&mut s, 0x4B, 0);
        bp(&mut s, 0x52, (12 << 3) | (1 << 11));
        for r in [0x49u8, 0x4A, 0x4D, 0x4B] {
            bp(&mut s, r, self.bp[r as usize]);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_parse() {
        let log = FifoLog {
            version: 6,
            is_wii: true,
            game_id: "SMNE01".into(),
            bp: (0..256).collect(),
            cp: vec![7; 256],
            xf_mem: (0..4096).map(|i| i * 3).collect(),
            xf_regs: vec![1; 88],
            tmem: (0..TMEM_SIZE).map(|i| (i % 251) as u8).collect(),
            frames: vec![
                Frame { fifo: vec![0x61, 0x45, 0, 0, 1], updates: vec![MemoryUpdate { fifo_position: 2, address: 0x1000, data: vec![1, 2, 3], kind: 1 }] },
                Frame { fifo: vec![0; 9], updates: Vec::new() },
            ],
        };
        assert_eq!(parse(&write(&log)).unwrap(), log);
        assert!(parse(b"nope").is_err());
    }
}
