//! Decoded-block cache for the interpreter: a straight run of guest code is
//! decoded once into handler calls, so running it again skips the
//! per-instruction fetch, address translation and first-level dispatch.
//! Guest memory counts writes to pages that hold decoded code; a block whose
//! pages were written since it was decoded (REL loading, relocation, code
//! patched by the game) is decoded again.

use guest_mem::Mem;

use crate::fast::{self, Handler};
use crate::{interp, Cpu, Stop};

/// Longest block, in instructions.
const MAX_LEN: usize = 32;
/// Direct-mapped block table, indexed by start address.
const SLOTS: usize = 1 << 16;
/// Decoded instructions kept before the whole cache is dropped.
const MAX_OPS: usize = 1 << 21;

#[derive(Clone, Copy)]
struct Slot {
    start: u32,
    first: u32,
    len: u32,
    /// First and last code page of the block and their write generations
    /// when it was decoded.
    pages: (u32, u32),
    gens: (u32, u32),
}

const EMPTY: Slot = Slot { start: 1, first: 0, len: 0, pages: (0, 0), gens: (0, 0) };

pub struct Blocks {
    slots: Vec<Slot>,
    /// Decoded instructions: (handler, instruction word).
    ops: Vec<(Handler, u32)>,
    /// Addresses the run loop must see as block starts.
    stops: Vec<u32>,
    pub built: u64,
    /// When set, every address a block was decoded at (the code profile
    /// `ppc-recomp` reads).
    pub starts: Option<std::collections::HashSet<u32>>,
}

fn handler(w: u32) -> Handler {
    if let Some(h) = fast::handler(w) {
        return h;
    }
    match w >> 26 {
        4 => interp::paired,
        19 => |c, _, w| interp::op19(c, w),
        31 => interp::op31,
        59 => |c, _, w| interp::fp_single(c, w),
        63 => |c, _, w| interp::fp_double(c, w),
        _ => interp::execute,
    }
}

/// Branches, `sc`, and the op-19 group (`bclr`, `bcctr`, `rfi`, `isync`).
fn ends_block(w: u32) -> bool {
    matches!(w >> 26, 16..=19)
}

impl Blocks {
    /// `stops`: addresses no block may run through (the caller checks them
    /// between blocks).
    pub fn new(stops: &[u32]) -> Self {
        Self { slots: vec![EMPTY; SLOTS], ops: Vec::new(), stops: stops.to_vec(), built: 0, starts: None }
    }

    fn clear(&mut self) {
        self.slots.fill(EMPTY);
        self.ops.clear();
    }

    /// (first op, length) of the valid block at `pc`, decoding it if needed;
    /// `None` when `pc` is not in RAM.
    fn lookup(&mut self, mem: &mut Mem, pc: u32) -> Option<(usize, usize)> {
        let slot = (pc >> 2) as usize & (SLOTS - 1);
        let s = self.slots[slot];
        if s.start == pc && (mem.code_gen(s.pages.0), mem.code_gen(s.pages.1)) == s.gens {
            return Some((s.first as usize, s.len as usize));
        }
        self.build(mem, pc, slot)
    }

    fn build(&mut self, mem: &mut Mem, pc: u32, slot: usize) -> Option<(usize, usize)> {
        if self.ops.len() + MAX_LEN > MAX_OPS {
            self.clear();
        }
        let first = self.ops.len();
        let mut at = pc;
        while let Some(b) = mem.ram(at, 4) {
            let w = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
            self.ops.push((handler(w), w));
            at = at.wrapping_add(4);
            if ends_block(w) || self.ops.len() - first == MAX_LEN || self.stops.contains(&at) {
                break;
            }
        }
        let len = self.ops.len() - first;
        if len == 0 {
            return None;
        }
        self.built += 1;
        if let Some(s) = &mut self.starts {
            s.insert(pc);
        }
        let pages = mem.mark_code(pc, len * 4)?;
        let gens = (mem.code_gen(pages.0), mem.code_gen(pages.1));
        self.slots[slot] = Slot { start: pc, first: first as u32, len: len as u32, pages, gens };
        Some((first, len))
    }
}

impl Cpu {
    /// Runs the block at PC (one instruction when PC is not in RAM) and
    /// returns how many instructions completed and whether the last was a
    /// branch ([`crate::is_branch`]). It stops after a taken branch or an
    /// exception; on an error PC points at the failing instruction, as with
    /// [`Cpu::step`].
    pub fn run_block(&mut self, mem: &mut Mem, blocks: &mut Blocks) -> Result<(u32, bool), Stop> {
        let start = self.pc;
        let Some((first, len)) = blocks.lookup(mem, start) else {
            let branch = mem.read_u32(start).is_ok_and(crate::is_branch);
            self.step(mem)?;
            return Ok((1, branch));
        };
        // FPU off (after a thread switch): a block with an FP instruction
        // runs one step at a time so the instruction traps where it is.
        if self.msr & crate::MSR_FP == 0 && blocks.ops[first..first + len].iter().any(|&(_, w)| crate::is_fp(w)) {
            let branch = crate::is_branch(blocks.ops[first].1);
            self.step(mem)?;
            return Ok((1, branch));
        }
        let mut pc = start;
        for (i, &(handler, word)) in blocks.ops[first..first + len].iter().enumerate() {
            let next = pc.wrapping_add(4);
            self.cia = pc;
            self.pc = next;
            self.cycles += 1;
            if let Err(e) = handler(self, mem, word) {
                self.pc = pc;
                return Err(e);
            }
            if self.pc != next {
                return Ok((i as u32 + 1, crate::is_branch(word)));
            }
            pc = next;
        }
        Ok((len as u32, crate::is_branch(blocks.ops[first + len - 1].1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use guest_mem::NullMmio;

    /// `addi r3, r3, imm` then `blr`.
    fn put(mem: &mut Mem, at: u32, imm: u16) {
        mem.write_u32(at, (14 << 26) | (3 << 21) | (3 << 16) | u32::from(imm)).unwrap();
        mem.write_u32(at + 4, 0x4E80_0020).unwrap();
    }

    fn run(cpu: &mut Cpu, mem: &mut Mem, blocks: &mut Blocks, at: u32) -> u32 {
        cpu.pc = at;
        cpu.gpr[3] = 0;
        cpu.lr = 0x8000_1000;
        cpu.run_block(mem, blocks).unwrap();
        cpu.gpr[3]
    }

    #[test]
    fn rewritten_code_is_decoded_again() {
        let mut mem = Mem::new(Box::new(NullMmio));
        let mut cpu = Cpu::new();
        let mut blocks = Blocks::new(&[]);
        let at = 0x8000_4000;
        put(&mut mem, at, 1);
        assert_eq!(run(&mut cpu, &mut mem, &mut blocks, at), 1);
        assert_eq!(run(&mut cpu, &mut mem, &mut blocks, at), 1);
        assert_eq!(blocks.built, 1, "the second run reuses the block");
        // A store through the uncached mirror.
        mem.write_u32(0xC000_4000, (14 << 26) | (3 << 21) | (3 << 16) | 2).unwrap();
        assert_eq!(run(&mut cpu, &mut mem, &mut blocks, at), 2);
        // A bulk write (DMA, disc read).
        let code = mem.ram_mut(0x0000_4000, 4).unwrap();
        code.copy_from_slice(&((14u32 << 26) | (3 << 21) | (3 << 16) | 3).to_be_bytes());
        assert_eq!(run(&mut cpu, &mut mem, &mut blocks, at), 3);
        // Writes to other pages leave the block alone.
        mem.write_u32(0x8000_8000, 0).unwrap();
        let built = blocks.built;
        run(&mut cpu, &mut mem, &mut blocks, at);
        assert_eq!(blocks.built, built);
    }
}
