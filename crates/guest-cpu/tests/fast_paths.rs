//! The block cache's dedicated handlers must match the generic interpreter:
//! random encodings of each fast-pathed instruction run once through
//! `Cpu::step` and once through a decoded block, from the same random state.

use guest_cpu::blocks::Blocks;
use guest_cpu::Cpu;
use guest_mem::{Mem, NullMmio};

const BASE: u32 = 0x8000_1000;
/// Registers point into this window so loads and stores mostly hit RAM.
const DATA: u32 = 0x8010_0000;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
}

/// Every fast-pathed form: (primary opcode, extended opcode for 19/31).
const FORMS: &[(u32, Option<u32>)] = &[
    (10, None), (11, None), (14, None), (15, None), (16, None), (18, None), (19, Some(16)), (19, Some(528)),
    (21, None), (24, None), (25, None), (28, None), (32, None), (33, None), (34, None), (36, None), (37, None),
    (38, None), (40, None), (42, None), (44, None), (48, None), (50, None), (52, None), (54, None),
    (31, Some(0)), (31, Some(32)), (31, Some(23)), (31, Some(151)), (31, Some(266)), (31, Some(40)),
    (31, Some(444)), (31, Some(28)), (31, Some(316)), (31, Some(339)), (31, Some(467)),
];

fn state(rng: &mut Rng) -> (Cpu, Mem) {
    let mut mem = Mem::new(Box::new(NullMmio));
    for i in 0..0x4000u32 {
        mem.write_u32(DATA - 0x8000 + 4 * i, rng.next()).unwrap();
    }
    let mut cpu = Cpu::new();
    for r in 0..32 {
        cpu.gpr[r] = if rng.next() % 4 == 0 { rng.next() } else { DATA + (rng.next() & 0x7FFC) };
        let f = f64::from(rng.next() as i32) / 1024.0;
        cpu.fpr[r] = [f, -f];
    }
    cpu.cr = rng.next();
    cpu.xer = rng.next() & 0xE000_007F;
    cpu.lr = BASE + 0x100 + (rng.next() & 0xFC);
    cpu.ctr = rng.next() % 4;
    cpu.pc = BASE;
    (cpu, mem)
}

fn word(rng: &mut Rng, op: u32, xo: Option<u32>) -> u32 {
    let mut w = (op << 26) | (rng.next() & 0x03FF_FFFF);
    if let Some(xo) = xo {
        w = (w & !0x7FE) | (xo << 1);
        if op == 31 {
            w &= !1;
            if xo == 339 || xo == 467 {
                // LR or CTR, as the fast paths cover.
                let spr = if rng.next() & 1 == 0 { 8 } else { 9 };
                w = (w & !0x001F_F800) | ((spr & 0x1F) << 16) | ((spr >> 5) << 11);
            }
        }
    }
    w
}

#[test]
fn fast_handlers_match_the_interpreter() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for &(op, xo) in FORMS {
        for _ in 0..120 {
            let seed = rng.0;
            let w = word(&mut rng, op, xo);
            let (mut a, mut ma) = state(&mut Rng(seed | 1));
            let (mut b, mut mb) = state(&mut Rng(seed | 1));
            ma.write_u32(BASE, w).unwrap();
            mb.write_u32(BASE, w).unwrap();
            // A trap after it, so the block holds exactly this instruction.
            ma.write_u32(BASE + 4, 0x7FE0_0008).unwrap();
            mb.write_u32(BASE + 4, 0x7FE0_0008).unwrap();
            let ra = a.step(&mut ma);
            let mut blocks = Blocks::new(&[BASE + 4]);
            let rb = b.run_block(&mut mb, &mut blocks).map(|_| ());
            let ctx = format!("word {w:#010x}");
            assert_eq!(ra, rb, "{ctx}");
            assert_eq!(a.pc, b.pc, "pc, {ctx}");
            assert_eq!(a.gpr, b.gpr, "gpr, {ctx}");
            assert_eq!(a.cr, b.cr, "cr, {ctx}");
            assert_eq!(a.xer, b.xer, "xer, {ctx}");
            assert_eq!((a.lr, a.ctr), (b.lr, b.ctr), "lr/ctr, {ctx}");
            assert_eq!(a.fpr.map(|p| p.map(f64::to_bits)), b.fpr.map(|p| p.map(f64::to_bits)), "fpr, {ctx}");
            assert_eq!(ma.ram(DATA - 0x10000, 0x20000), mb.ram(DATA - 0x10000, 0x20000), "memory, {ctx}");
        }
    }
}
