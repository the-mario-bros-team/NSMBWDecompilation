//! Instruction-level tests. Each expected value is worked out by hand from
//! the PowerPC 750CL manual; the cases target the semantics most often wrong
//! in emulators (carries, masks, CR bits, FP conversions, quantisation).

use guest_cpu::{Cpu, Stop, SPR_SRR0, XER_CA, XER_OV, XER_SO};
use guest_mem::{Mem, NullMmio};

const BASE: u32 = 0x8000_1000;

fn d(op: u32, rt: u32, ra: u32, imm: i32) -> u32 {
    (op << 26) | (rt << 21) | (ra << 16) | (imm as u32 & 0xFFFF)
}
fn x(op: u32, rt: u32, ra: u32, rb: u32, xo: u32, rc: u32) -> u32 {
    (op << 26) | (rt << 21) | (ra << 16) | (rb << 11) | (xo << 1) | rc
}
fn xo(rt: u32, ra: u32, rb: u32, oe: u32, xo9: u32, rc: u32) -> u32 {
    (31 << 26) | (rt << 21) | (ra << 16) | (rb << 11) | (oe << 10) | (xo9 << 1) | rc
}
fn rlw(op: u32, rs: u32, ra: u32, sh: u32, mb: u32, me: u32) -> u32 {
    (op << 26) | (rs << 21) | (ra << 16) | (sh << 11) | (mb << 6) | (me << 1)
}
fn a(op: u32, ft: u32, fa: u32, fb: u32, fc: u32, xo5: u32) -> u32 {
    (op << 26) | (ft << 21) | (fa << 16) | (fb << 11) | (fc << 6) | (xo5 << 1)
}
fn mtspr(spr: u32, rs: u32) -> u32 {
    x(31, rs, spr & 0x1F, spr >> 5, 467, 0)
}

fn setup(words: &[u32]) -> (Cpu, Mem) {
    let mut mem = Mem::new(Box::new(NullMmio));
    for (i, w) in words.iter().enumerate() {
        mem.write_u32(BASE + 4 * i as u32, *w).unwrap();
    }
    let mut cpu = Cpu::new();
    cpu.pc = BASE;
    cpu.msr |= guest_cpu::MSR_FP;
    (cpu, mem)
}

#[test]
fn fpu_off_traps_to_0x800() {
    // fadd f1,f2,f3 with MSR[FP] clear: no result, SRR0 at the instruction.
    let (mut cpu, mut mem) = setup(&[(63 << 26) | (1 << 21) | (2 << 16) | (3 << 11) | (21 << 1)]);
    cpu.msr = guest_cpu::MSR_EE;
    cpu.fpr[2] = [1.0, 1.0];
    cpu.fpr[3] = [2.0, 2.0];
    cpu.step(&mut mem).unwrap();
    assert_eq!(cpu.pc, 0x800);
    assert_eq!(cpu.spr[guest_cpu::SPR_SRR0], BASE);
    assert_eq!(cpu.fpr[1][0], 0.0);
    assert_eq!(cpu.msr & guest_cpu::MSR_EE, 0);
    assert!(guest_cpu::is_fp(0xC002_B420), "lfs");
    assert!(!guest_cpu::is_fp(0x3861_000C), "addi");
}

/// Runs a straight-line program (one step per word).
fn exec(words: &[u32], prep: impl FnOnce(&mut Cpu, &mut Mem)) -> (Cpu, Mem) {
    let (mut cpu, mut mem) = setup(words);
    prep(&mut cpu, &mut mem);
    for _ in 0..words.len() {
        cpu.step(&mut mem).unwrap();
    }
    (cpu, mem)
}

/// Runs until the PC reaches `end` (for programs with branches).
fn run_to(words: &[u32], end: u32, prep: impl FnOnce(&mut Cpu, &mut Mem)) -> (Cpu, Mem) {
    let (mut cpu, mut mem) = setup(words);
    prep(&mut cpu, &mut mem);
    for _ in 0..10_000 {
        if cpu.pc == end {
            return (cpu, mem);
        }
        cpu.step(&mut mem).unwrap();
    }
    panic!("never reached {end:#x}; pc {:#x}", cpu.pc);
}

#[test]
fn rotate_and_mask() {
    let (c, _) = exec(
        &[rlw(21, 3, 4, 8, 24, 31), rlw(21, 3, 5, 0, 16, 15), rlw(21, 3, 6, 4, 0, 27), rlw(20, 7, 8, 8, 16, 23)],
        |c, _| {
            c.gpr[3] = 0x1234_5678;
            c.gpr[7] = 0xAABB_CCDD;
            c.gpr[8] = 0x1122_3344;
        },
    );
    assert_eq!(c.gpr[4], 0x12);
    assert_eq!(c.gpr[5], 0x1234_5678, "mb > me wraps to a full mask");
    assert_eq!(c.gpr[6], 0x2345_6780);
    assert_eq!(c.gpr[8], 0x1122_DD44, "rlwimi inserts under the mask");
}

#[test]
fn shifts_and_carry() {
    let (c, _) = exec(&[x(31, 3, 4, 2, 824, 0)], |c, _| c.gpr[3] = 0xFFFF_FFF5);
    assert_eq!(c.gpr[4], 0xFFFF_FFFD);
    assert_ne!(c.xer & XER_CA, 0, "1-bits shifted out of a negative value set CA");

    let (c, _) = exec(&[x(31, 3, 4, 2, 824, 0)], |c, _| c.gpr[3] = 0xFFFF_FFF4);
    assert_eq!(c.gpr[4], 0xFFFF_FFFD);
    assert_eq!(c.xer & XER_CA, 0);

    let (c, _) = exec(&[x(31, 3, 4, 5, 792, 0), x(31, 6, 7, 5, 792, 0)], |c, _| {
        c.gpr[3] = 0x8000_0000;
        c.gpr[5] = 40;
        c.gpr[6] = 0x7000_0000;
    });
    assert_eq!(c.gpr[4], 0xFFFF_FFFF);
    assert_eq!(c.gpr[7], 0);
    assert_eq!(c.xer & XER_CA, 0, "last sraw was on a positive value");

    let (c, _) = exec(&[x(31, 3, 4, 5, 24, 0), x(31, 3, 6, 7, 536, 0)], |c, _| {
        c.gpr[3] = 0xFFFF_FFFF;
        c.gpr[5] = 32;
        c.gpr[7] = 31;
    });
    assert_eq!(c.gpr[4], 0, "slw by 32 clears");
    assert_eq!(c.gpr[6], 1);
}

#[test]
fn add_subtract_carries() {
    let (c, _) = exec(
        &[xo(5, 3, 4, 0, 10, 0), xo(6, 3, 4, 0, 138, 0), xo(7, 4, 3, 0, 8, 0), xo(8, 3, 4, 0, 8, 0)],
        |c, _| {
            c.gpr[3] = 0xFFFF_FFFF;
            c.gpr[4] = 1;
        },
    );
    assert_eq!(c.gpr[5], 0, "addc wraps");
    assert_eq!(c.gpr[6], 1, "adde adds the carry from addc");
    assert_eq!(c.gpr[7], 0xFFFF_FFFE, "subfc: r3 - r4");
    assert_eq!(c.gpr[8], 2, "subfc: r4 - r3 wraps");
    assert_eq!(c.xer & XER_CA, 0, "r4 - r3 borrows, so CA is clear");

    let (c, _) = exec(&[d(8, 9, 4, 10)], |c, _| c.gpr[4] = 1);
    assert_eq!(c.gpr[9], 9);
    assert_ne!(c.xer & XER_CA, 0);
    let (c, _) = exec(&[d(8, 9, 4, 0)], |c, _| c.gpr[4] = 1);
    assert_eq!(c.gpr[9], 0xFFFF_FFFF);
    assert_eq!(c.xer & XER_CA, 0);

    let (c, _) = exec(&[xo(3, 3, 0, 0, 202, 0)], |c, _| {
        c.gpr[3] = 5;
        c.xer = XER_CA;
    });
    assert_eq!(c.gpr[3], 6, "addze");
    assert_eq!(c.xer & XER_CA, 0);
}

#[test]
fn overflow_and_cr0() {
    let (c, _) = exec(&[xo(5, 3, 4, 1, 266, 1)], |c, _| {
        c.gpr[3] = 0x7FFF_FFFF;
        c.gpr[4] = 1;
    });
    assert_eq!(c.gpr[5], 0x8000_0000);
    assert_eq!(c.xer & (XER_OV | XER_SO), XER_OV | XER_SO);
    assert_eq!(c.cr_field(0), 8 | 1, "LT plus the copied SO");

    let (c, _) = exec(&[xo(5, 3, 4, 1, 235, 0)], |c, _| {
        c.gpr[3] = 0x10000;
        c.gpr[4] = 0x10000;
    });
    assert_eq!(c.gpr[5], 0);
    assert_ne!(c.xer & XER_OV, 0, "mullwo overflow");
}

#[test]
fn multiply_divide() {
    let (c, _) = exec(
        &[xo(5, 3, 4, 0, 75, 0), xo(6, 7, 7, 0, 11, 0), xo(8, 9, 4, 0, 491, 0)],
        |c, _| {
            c.gpr[3] = (-2i32) as u32;
            c.gpr[4] = 3;
            c.gpr[7] = 0xFFFF_FFFF;
            c.gpr[9] = 7;
        },
    );
    assert_eq!(c.gpr[5], 0xFFFF_FFFF, "mulhw of -6");
    assert_eq!(c.gpr[6], 0xFFFF_FFFE, "mulhwu");
    assert_eq!(c.gpr[8], 2, "7 / 3");

    let (c, _) = exec(
        &[xo(5, 3, 4, 0, 491, 0), xo(6, 7, 0, 0, 491, 0), xo(8, 9, 0, 0, 491, 0), xo(10, 9, 0, 0, 459, 0), xo(11, 12, 13, 0, 491, 0)],
        |c, _| {
            c.gpr[3] = 7;
            c.gpr[4] = (-2i32) as u32;
            c.gpr[7] = (-5i32) as u32;
            c.gpr[9] = 5;
            c.gpr[12] = 0x8000_0000;
            c.gpr[13] = u32::MAX;
        },
    );
    assert_eq!(c.gpr[5] as i32, -3, "divw truncates toward zero");
    assert_eq!(c.gpr[6], 0xFFFF_FFFF, "negative / 0");
    assert_eq!(c.gpr[8], 0, "positive / 0");
    assert_eq!(c.gpr[10], 0, "divwu / 0");
    assert_eq!(c.gpr[11], 0xFFFF_FFFF, "INT_MIN / -1");
}

#[test]
fn logic_and_extend() {
    let (c, _) = exec(
        &[x(31, 3, 4, 0, 26, 0), x(31, 5, 6, 0, 26, 0), x(31, 7, 8, 0, 954, 0), x(31, 7, 9, 0, 922, 0), x(31, 3, 10, 5, 476, 0)],
        |c, _| {
            c.gpr[3] = 0;
            c.gpr[5] = 0x8000;
            c.gpr[7] = 0x0000_8080;
        },
    );
    assert_eq!(c.gpr[4], 32);
    assert_eq!(c.gpr[6], 16);
    assert_eq!(c.gpr[8], 0xFFFF_FF80, "extsb");
    assert_eq!(c.gpr[9], 0xFFFF_8080, "extsh");
    assert_eq!(c.gpr[10], 0xFFFF_FFFF, "nand of 0 and 0x8000");
}

#[test]
fn compares_and_cr_logic() {
    let (c, _) = exec(
        &[x(31, 7 << 2, 3, 4, 0, 0), x(31, 6 << 2, 3, 4, 32, 0), d(11, 0, 3, -1)],
        |c, _| {
            c.gpr[3] = u32::MAX;
            c.gpr[4] = 1;
        },
    );
    assert_eq!(c.cr_field(7), 8, "signed: -1 < 1");
    assert_eq!(c.cr_field(6), 4, "unsigned: 0xFFFFFFFF > 1");
    assert_eq!(c.cr_field(0), 2, "cmpwi -1, -1");

    // cror 2,1,2 = EQ |= GT (a >= test).
    let (c, _) = exec(&[x(19, 2, 1, 2, 449, 0)], |c, _| c.cr = 0x4000_0000);
    assert!(c.cr_bit(2));
    // crxor 6,6,6 clears CR1[EQ] (the CodeWarrior varargs idiom).
    let (c, _) = exec(&[x(19, 6, 6, 6, 193, 0)], |c, _| c.cr = 0x0200_0000);
    assert!(!c.cr_bit(6));

    let (c, _) = exec(&[x(31, 3, 0x81 >> 4, (0x81 & 0xF) << 1, 144, 0), x(31, 4, 0, 0, 19, 0)], |c, _| {
        c.gpr[3] = 0x1234_5678;
    });
    assert_eq!(c.cr, 0x1000_0008, "mtcrf 0x81 writes fields 0 and 7");
    assert_eq!(c.gpr[4], 0x1000_0008);

    let (c, _) = exec(&[x(31, 3 << 2, 0, 0, 512, 0)], |c, _| c.xer = 0xE000_0012);
    assert_eq!(c.cr_field(3), 0xE);
    assert_eq!(c.xer, 0x0000_0012, "mcrxr clears XER[0:3]");
}

#[test]
fn branches() {
    // li r3,0; li r4,5; mtctr r4; loop: addi r3,r3,1; bdnz loop
    let words = [d(14, 3, 0, 0), d(14, 4, 0, 5), mtspr(9, 4), d(14, 3, 3, 1), (16 << 26) | (16 << 21) | 0xFFFC];
    let (c, _) = run_to(&words, BASE + 20, |_, _| {});
    assert_eq!(c.gpr[3], 5);
    assert_eq!(c.ctr, 0);

    // bl +12; li r3,1; b end; target: li r4,2; blr; end:
    let words = [
        (18 << 26) | 12 | 1,
        d(14, 3, 0, 1),
        (18 << 26) | 12,
        d(14, 4, 0, 2),
        x(19, 20, 0, 0, 16, 0),
    ];
    let (c, _) = run_to(&words, BASE + 20, |_, _| {});
    assert_eq!((c.gpr[3], c.gpr[4]), (1, 2));
    assert_eq!(c.lr, BASE + 4);

    // A not-taken bcl still writes LR. beq+8 with CR0[EQ] clear:
    let (c, _) = exec(&[(16 << 26) | (12 << 21) | (2 << 16) | 8 | 1], |_, _| {});
    assert_eq!(c.pc, BASE + 4);
    assert_eq!(c.lr, BASE + 4);

    // bcctr does not decrement CTR.
    let (c, _) = exec(&[x(19, 20, 0, 0, 528, 0)], |c, _| c.ctr = 0x8000_2000);
    assert_eq!((c.pc, c.ctr), (0x8000_2000, 0x8000_2000));
}

#[test]
fn system_call_and_rfi() {
    let (mut cpu, mut mem) = setup(&[0x4400_0002, d(14, 3, 0, 7)]);
    mem.write_u32(0x8000_0C00, x(19, 0, 0, 0, 50, 0)).unwrap();
    cpu.msr = 0x2030;
    cpu.step(&mut mem).unwrap();
    assert_eq!(cpu.pc, 0xC00);
    assert_eq!(cpu.spr[SPR_SRR0], BASE + 4);
    assert_eq!(cpu.msr & 0x30, 0, "translation off in the handler");
    cpu.step(&mut mem).unwrap();
    assert_eq!(cpu.pc, BASE + 4);
    assert_eq!(cpu.msr, 0x2030);
    cpu.step(&mut mem).unwrap();
    assert_eq!(cpu.gpr[3], 7);
}

#[test]
fn loads_and_stores() {
    let (c, mut m) = exec(
        &[
            x(31, 3, 0, 4, 790, 0), // lhbrx r3,0,r4
            d(42, 5, 4, 2),         // lha r5,2(r4)
            d(35, 6, 4, 1),         // lbzu r6,1(r4)
            d(47, 29, 1, 0),        // stmw r29,0(r1)
            d(46, 26, 1, -12),      // lmw r26,-12(r1)
        ],
        |c, m| {
            c.gpr[4] = 0x8000_2000;
            m.write_u32(0x8000_2000, 0x1234_8001).unwrap();
            c.gpr[1] = 0x8000_3000;
            m.write_u32(0x8000_2FF4, 0xAAAA_0001).unwrap();
            m.write_u32(0x8000_2FF8, 0xAAAA_0002).unwrap();
            m.write_u32(0x8000_2FFC, 0xAAAA_0003).unwrap();
            c.gpr[29] = 0x29;
            c.gpr[30] = 0x30;
            c.gpr[31] = 0x31;
        },
    );
    assert_eq!(c.gpr[3], 0x3412);
    assert_eq!(c.gpr[5], 0xFFFF_8001);
    assert_eq!((c.gpr[6], c.gpr[4]), (0x34, 0x8000_2001), "lbzu updates rA");
    assert_eq!(m.read_u32(0x8000_3008).unwrap(), 0x31);
    assert_eq!((c.gpr[26], c.gpr[28], c.gpr[29], c.gpr[31]), (0xAAAA_0001, 0xAAAA_0003, 0x29, 0x31));
}

#[test]
fn reservation() {
    let (c, m) = exec(&[x(31, 3, 0, 4, 20, 0), x(31, 5, 0, 4, 150, 1)], |c, m| {
        c.gpr[4] = 0x8000_2000;
        c.gpr[5] = 99;
        m.write_u32(0x8000_2000, 42).unwrap();
    });
    assert_eq!(c.gpr[3], 42);
    assert_eq!(c.cr_field(0), 2, "stwcx. succeeded");
    let mut m = m;
    assert_eq!(m.read_u32(0x8000_2000).unwrap(), 99);

    let (c, _) = exec(&[x(31, 5, 0, 4, 150, 1)], |c, _| c.gpr[4] = 0x8000_2000);
    assert_eq!(c.cr_field(0), 0, "no reservation: stwcx. fails");
}

#[test]
fn strings_and_dcbz() {
    let (c, mut m) = exec(&[x(31, 5, 4, 6, 597, 0), x(31, 0, 0, 7, 1014, 0)], |c, m| {
        c.gpr[4] = 0x8000_2000;
        m.ram_mut(0x8000_2000, 6).unwrap().copy_from_slice(b"ABCDEF");
        c.gpr[7] = 0x8000_4010;
        m.ram_mut(0x8000_4000, 64).unwrap().fill(0xFF);
    });
    assert_eq!((c.gpr[5], c.gpr[6]), (0x4142_4344, 0x4546_0000));
    assert_eq!(m.read_u32(0x8000_4000).unwrap(), 0, "dcbz clears the aligned line");
    assert_eq!(m.read_u32(0x8000_401C).unwrap(), 0);
    assert_eq!(m.read_u32(0x8000_4020).unwrap(), 0xFFFF_FFFF, "next line untouched");
}

#[test]
fn float_load_store_is_bit_exact() {
    let (c, mut m) = exec(&[d(48, 1, 4, 0), d(52, 1, 4, 4), d(50, 2, 4, 8), d(54, 2, 4, 16)], |c, m| {
        c.gpr[4] = 0x8000_2000;
        m.write_u32(0x8000_2000, 0x7F80_0001).unwrap();
        m.write_u64(0x8000_2008, 0x7FF0_0000_0000_0001).unwrap();
    });
    assert_eq!(m.read_u32(0x8000_2004).unwrap(), 0x7F80_0001, "signalling NaN survives lfs/stfs");
    assert_eq!(m.read_u64(0x8000_2010).unwrap(), 0x7FF0_0000_0000_0001);
    assert_eq!(c.fpr[1][0].to_bits(), c.fpr[1][1].to_bits(), "lfs fills ps1");
}

#[test]
fn float_conversions() {
    let (_, mut m) = exec(
        &[x(63, 2, 0, 1, 15, 0), x(31, 2, 0, 4, 983, 0), x(63, 3, 0, 5, 14, 0), d(14, 4, 4, 4), x(31, 3, 0, 4, 983, 0)],
        |c, _| {
            c.fpr[1][0] = -2.7;
            c.fpr[5][0] = 2.5;
            c.gpr[4] = 0x8000_2000;
        },
    );
    assert_eq!(m.read_u32(0x8000_2000).unwrap(), 0xFFFF_FFFE, "fctiwz truncates");
    assert_eq!(m.read_u32(0x8000_2004).unwrap(), 2, "fctiw rounds ties to even");

    let (c, _) = exec(&[x(63, 2, 0, 1, 12, 0)], |c, _| c.fpr[1][0] = 1.0 / 3.0);
    assert_eq!(c.fpr[2], [(1.0f64 / 3.0) as f32 as f64; 2], "frsp rounds and fills ps1");
}

#[test]
fn float_arithmetic() {
    let (c, _) = exec(&[a(63, 4, 1, 2, 3, 23)], |c, _| {
        c.fpr[1][0] = -0.0;
        c.fpr[2][0] = 1.0;
        c.fpr[3][0] = 2.0;
    });
    assert_eq!(c.fpr[4][0], 2.0, "fsel treats -0.0 as >= 0");

    let e = (2f64).powi(-30);
    let (c, _) = exec(&[a(63, 4, 1, 2, 3, 29)], |c, _| {
        c.fpr[1][0] = 1.0 + e;
        c.fpr[3][0] = 1.0 - e;
        c.fpr[2][0] = -1.0;
    });
    assert_eq!(c.fpr[4][0], -(2f64).powi(-60), "fmadd is fused");

    let (c, _) = exec(&[x(63, 3 << 2, 1, 2, 0, 0)], |c, _| {
        c.fpr[1][0] = f64::NAN;
        c.fpr[2][0] = 1.0;
    });
    assert_eq!(c.cr_field(3), 1, "unordered");
}

#[test]
fn paired_singles() {
    let (c, _) = exec(&[x(4, 4, 1, 2, 592, 0), a(4, 5, 1, 2, 3, 10), a(4, 6, 1, 2, 3, 29), a(4, 7, 1, 2, 3, 14)], |c, _| {
        c.fpr[1] = [1.0, 2.0];
        c.fpr[2] = [3.0, 4.0];
        c.fpr[3] = [5.0, 6.0];
    });
    assert_eq!(c.fpr[4], [2.0, 3.0], "ps_merge10");
    assert_eq!(c.fpr[5], [5.0, 6.0], "ps_sum0");
    assert_eq!(c.fpr[6], [8.0, 16.0], "ps_madd");
    assert_eq!(c.fpr[7], [8.0, 14.0], "ps_madds0 uses c.ps0 for both");
}

#[test]
fn quantized_load_store() {
    let psq = |op: u32, fr: u32, ra: u32, w: u32, i: u32, dd: i32| (op << 26) | (fr << 21) | (ra << 16) | (w << 15) | (i << 12) | (dd as u32 & 0xFFF);
    let (c, mut m) = exec(
        &[psq(56, 1, 4, 0, 2, 0), psq(56, 2, 4, 0, 3, 0), psq(56, 3, 4, 1, 2, 0), psq(60, 5, 4, 0, 4, 8)],
        |c, m| {
            c.gpr[4] = 0x8000_2000;
            m.write_u16(0x8000_2000, 0x0AC8).unwrap(); // bytes 10, 200
            c.spr[912 + 2] = 4 << 16; // u8, scale 0
            c.spr[912 + 3] = (1 << 24) | (4 << 16); // u8, scale 1 (halves)
            c.spr[912 + 4] = (2 << 8) | 7; // store s16, scale 2 (x4)
            c.fpr[5] = [1.5, -40000.0];
        },
    );
    assert_eq!(c.fpr[1], [10.0, 200.0]);
    assert_eq!(c.fpr[2], [5.0, 100.0]);
    assert_eq!(c.fpr[3], [10.0, 1.0], "single load sets ps1 = 1.0");
    assert_eq!(m.read_u16(0x8000_2008).unwrap(), 6);
    assert_eq!(m.read_u16(0x8000_200A).unwrap(), 0x8000, "saturates to i16::MIN");
}

#[test]
fn locked_cache_dma() {
    let (_, mut m) = exec(&[mtspr(922, 3), mtspr(923, 4)], |c, m| {
        m.ram_mut(0x8000_5000, 64).unwrap().fill(0xAB);
        c.gpr[3] = 0x8000_5000; // DMAU: main address, length high bits 0
        c.gpr[4] = 0xE000_0000 | 0x10 | (2 << 2) | 2; // DMAL: cache address, load, 2 lines, trigger
    });
    assert_eq!(m.read_u32(0xE000_0000).unwrap(), 0xABAB_ABAB);
    assert_eq!(m.read_u32(0xE000_003C).unwrap(), 0xABAB_ABAB);
    assert_eq!(m.read_u32(0xE000_0040).unwrap(), 0);
}

#[test]
fn unknown_word_stops_without_advancing() {
    let (mut cpu, mut mem) = setup(&[0x0000_0000]);
    assert_eq!(cpu.step(&mut mem), Err(Stop::Unimplemented { pc: BASE, word: 0 }));
    assert_eq!(cpu.pc, BASE);
}
