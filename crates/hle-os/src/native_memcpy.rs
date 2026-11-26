//! Bounded, RAM-only execution of a copy routine's CPU effects.
use guest_cpu::{blocks::Blocks, Cpu, MSR_FP};
use guest_mem::{Mem, NullMmio, RamView};
use std::ops::Range;

pub(super) struct NativeCopy {
    code: Range<u32>,
    shadow: Mem,
    blocks: Blocks,
    commits: (u64, u64),
}

impl NativeCopy {
    pub fn new(code: Range<u32>) -> Self {
        // Each scratch dispatch must retire exactly one checked instruction.
        let stops: Vec<_> = (code.start..=code.end).step_by(4).collect();
        Self {
            code,
            shadow: Mem::new(Box::new(NullMmio)),
            blocks: Blocks::new(&stops),
            commits: (0, 0),
        }
    }

    pub fn commits(&self) -> (u64, u64) {
        self.commits
    }

    pub fn try_copy(
        &mut self,
        cpu: &mut Cpu,
        live: &mut Mem,
        end: u64,
        special: &[u32],
    ) -> Option<u64> {
        let [dst, src, len] = [cpu.gpr[3], cpu.gpr[4], cpu.gpr[5]];
        let len = len as usize;
        let available = end.checked_sub(cpu.cycles)?;
        // Supported stores move at most eight bytes per retired instruction.
        if cpu.pc != self.code.start
            || len == 0
            || len as u64 > available.saturating_mul(8)
            || !live.is_untracked_ram_range(dst, len)
        {
            return None;
        }

        let code_len = (self.code.end - self.code.start) as usize;
        let view = live.view();
        let source = ram_range(&view, src, len)?;
        let destination = ram_range(&view, dst, len)?;
        let instructions = ram_range(&view, self.code.start, code_len)?;
        if overlaps(&source, &destination) || overlaps(&instructions, &destination) {
            return None;
        }

        let live_code = live.ram(self.code.start, code_len)?;
        if self.shadow.ram(self.code.start, code_len)? != live_code {
            self.shadow
                .ram_mut(self.code.start, code_len)?
                .copy_from_slice(live_code);
        }
        self.shadow
            .ram_mut(src, len)?
            .copy_from_slice(live.ram(src, len)?);
        self.shadow
            .ram_mut(dst, len)?
            .copy_from_slice(live.ram(dst, len)?);

        let mut trial = cpu.clone();
        let return_to = cpu.lr & !3;
        while trial.cycles < end {
            let pc = trial.pc;
            if !self.code.contains(&pc)
                || pc & 3 != 0
                || (trial.cycles != cpu.cycles && special.contains(&pc))
            {
                return None;
            }
            let word = u32::from_be_bytes(self.shadow.ram(pc, 4)?.try_into().ok()?);
            if !allowed(&view, &trial, word, &source, &destination) {
                return None;
            }
            let (steps, branch) = trial.run_block(&mut self.shadow, &mut self.blocks).ok()?;
            if steps != 1 {
                return None;
            }
            if trial.pc != return_to {
                continue;
            }
            if !branch
                || !is_bclr(word)
                || trial.gpr[1] != cpu.gpr[1]
                || trial.gpr[3] != dst
                || self.shadow.ram(dst, len)? != live.ram(src, len)?
                || !live.is_untracked_ram_range(dst, len)
                || !live.copy_ram_nonoverlapping(dst, src, len)
            {
                return None;
            }
            let retired = trial.cycles - cpu.cycles;
            *cpu = trial;
            self.commits.0 += 1;
            self.commits.1 += len as u64;
            return Some(retired);
        }
        None
    }
}

fn ram_range(view: &RamView, address: u32, len: usize) -> Option<Range<usize>> {
    let start = view.offset(address, len)?;
    Some(start..start + len)
}

fn overlaps(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

fn is_bclr(word: u32) -> bool {
    word >> 26 == 19 && (word >> 1) & 0x3ff == 16 && word & 1 == 0
}

// Admit only instructions with effects covered by the scratch CPU and RAM.
fn allowed(
    view: &RamView,
    cpu: &Cpu,
    word: u32,
    source: &Range<usize>,
    destination: &Range<usize>,
) -> bool {
    let op = word >> 26;
    let ra = ((word >> 16) & 31) as usize;
    let base = if ra == 0 { 0 } else { cpu.gpr[ra] };
    let d = || base.wrapping_add((word as u16 as i16 as i32) as u32);
    let x = || base.wrapping_add(cpu.gpr[((word >> 11) & 31) as usize]);
    let (address, size, write) = match op {
        10 | 11 | 14 | 15 | 21 | 24 | 25 | 28 => return true,
        16 | 18 => return word & 1 == 0,
        19 => return is_bclr(word),
        32..=45 => {
            let size = match op {
                32 | 33 | 36 | 37 => 4,
                34 | 35 | 38 | 39 => 1,
                _ => 2,
            };
            (d(), size, matches!(op, 36..=39 | 44 | 45))
        }
        48..=55 if cpu.msr & MSR_FP != 0 => (
            d(),
            if matches!(op, 48 | 49 | 52 | 53) {
                4
            } else {
                8
            },
            op >= 52,
        ),
        31 => match (word >> 1) & 0x3ff {
            23 => (x(), 4, false),
            151 => (x(), 4, true),
            0 | 28 | 32 | 40 | 266 | 278 | 316 | 444 => return true,
            339 | 467 => {
                let spr = ((word >> 16) & 31) | (((word >> 11) & 31) << 5);
                return matches!(spr, 8 | 9);
            }
            _ => return false,
        },
        _ => return false,
    };
    let Some(access) = ram_range(view, address, size) else {
        return false;
    };
    let inside = |range: &Range<usize>| access.start >= range.start && access.end <= range.end;
    inside(destination) || (!write && inside(source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use guest_mem::{
        snap::{Snap, Snapshot},
        CHUNK,
    };

    const ENTRY: u32 = 0x8000_1000;
    const RETURN: u32 = 0x8000_2000;
    const SRC: u32 = 0x8000_3000;
    const DST: u32 = 0x8001_4000;
    const BLR: u32 = 0x4e80_0020;

    const fn d(op: u32, reg: u32, ra: u32, offset: i16) -> u32 {
        (op << 26) | (reg << 21) | (ra << 16) | offset as u16 as u32
    }

    const COPY: [u32; 8] = [
        d(14, 6, 3, -1),                                  // addi r6,r3,-1
        d(14, 7, 4, -1),                                  // addi r7,r4,-1
        (31 << 26) | (5 << 21) | (9 << 16) | (467 << 1),  // mtctr r5
        d(35, 8, 7, 1),                                   // lbzu r8,1(r7)
        d(39, 8, 6, 1),                                   // stbu r8,1(r6)
        d(10, 0, 8, 0),                                   // cmplwi r8,0
        (16 << 26) | (16 << 21) | (-12i16 as u16 as u32), // bdnz loop
        BLR,
    ];

    fn fixture() -> (Cpu, Mem) {
        let mut mem = Mem::new(Box::new(NullMmio));
        for (index, &word) in COPY.iter().enumerate() {
            mem.write_u32(ENTRY + index as u32 * 4, word).unwrap();
        }
        mem.write_u32(SRC, 0x1180_00ef).unwrap();
        mem.write_u32(DST, 0x5566_7788).unwrap();
        let mut cpu = Cpu::new();
        cpu.pc = ENTRY;
        cpu.lr = RETURN;
        cpu.gpr[3] = DST;
        cpu.gpr[4] = SRC;
        cpu.gpr[5] = 4;
        cpu.cr = 0x8765_4321;
        (cpu, mem)
    }

    fn cpu_state(cpu: &Cpu) -> Vec<u8> {
        let mut state = Snap::saver();
        cpu.clone().snap(&mut state);
        state.into_bytes()
    }

    fn hook() -> NativeCopy {
        NativeCopy::new(ENTRY..ENTRY + 4 * COPY.len() as u32)
    }

    #[test]
    fn commits_real_copy_and_preserves_full_cpu_state_at_slice_end() {
        let (mut expected, mut reference) = fixture();
        let mut blocks = Blocks::new(&[]);
        while expected.pc != RETURN {
            expected.run_block(&mut reference, &mut blocks).unwrap();
        }
        assert_eq!(expected.cycles, 20);

        let (mut actual, mut live) = fixture();
        let mut native = hook();
        assert_eq!(native.try_copy(&mut actual, &mut live, 20, &[]), Some(20));
        assert_eq!(live.read_u32(DST), Ok(0x1180_00ef));
        assert_eq!(live.hash(), reference.hash());
        assert_eq!(cpu_state(&actual), cpu_state(&expected));
        assert_eq!(native.commits(), (1, 4));
    }

    #[test]
    fn declines_budget_alias_mmio_and_tracked_pages_without_touching_live_state() {
        for case in 0..8 {
            let (mut cpu, mut mem) = fixture();
            let mut end = 20;
            let mut special = Vec::new();
            match case {
                0 => end = 19,
                1 => cpu.gpr[4] = DST + 1,
                2 => cpu.gpr[4] = 0xcc00_0000,
                3 => {
                    let view = mem.view();
                    view.watch(view.offset(DST, 4).unwrap() / CHUNK);
                }
                4 => {
                    mem.mark_code(DST, 4).unwrap();
                }
                5 => {
                    mem.write_u32(ENTRY, (18 << 26) | 0x400).unwrap();
                }
                6 => {
                    mem.write_u32(ENTRY + 16, d(39, 8, 6, 5)).unwrap();
                }
                7 => special.push(ENTRY + 12),
                _ => unreachable!(),
            }
            let before = (cpu_state(&cpu), mem.hash(), mem.code_epoch());
            let mut native = hook();
            assert_eq!(
                native.try_copy(&mut cpu, &mut mem, end, &special),
                None,
                "case {case}"
            );
            assert_eq!(
                (cpu_state(&cpu), mem.hash(), mem.code_epoch()),
                before,
                "case {case}"
            );
            assert_eq!(native.commits(), (0, 0));
        }
    }
}
