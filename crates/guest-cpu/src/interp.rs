//! Instruction semantics. One `match` per opcode group; every arm is the full
//! architected behaviour of that instruction on Broadway, except where a
//! comment says otherwise.
//!
//! The group entry points ([`execute`], [`op19`], [`op31`], [`fp_single`],
//! [`fp_double`], [`paired`]) are public for recompiled code, which calls
//! them with constant words for instructions without a [`crate::fast`]
//! handler.

use crate::fp;
use crate::{Cpu, Stop, SPR_CTR, SPR_DEC, SPR_LR, SPR_SRR0, SPR_SRR1, SPR_XER};
use guest_mem::{Fault, Mem};

type R = Result<(), Stop>;

const SIGN: u64 = 0x8000_0000_0000_0000;
const SPR_DMAU: usize = 922;
const SPR_DMAL: usize = 923;

macro_rules! m {
    ($cpu:expr, $e:expr) => {
        $e.map_err(|fault| Stop::Fault { pc: $cpu.cia, fault })?
    };
}

fn unimpl(cpu: &Cpu, word: u32) -> R {
    Err(Stop::Unimplemented { pc: cpu.cia, word })
}

/// Rotate mask MASK(mb, me), big-endian bit numbering, wrapping when mb > me.
#[inline]
pub(crate) fn mask(mb: u32, me: u32) -> u32 {
    let a = u32::MAX >> mb;
    let b = u32::MAX << (31 - me);
    if mb <= me {
        a & b
    } else {
        a | b
    }
}

/// `a + b + c` with carry-out and signed overflow.
#[inline]
fn add3(a: u32, b: u32, c: u32) -> (u32, bool, bool) {
    let wide = a as u64 + b as u64 + c as u64;
    let r = wide as u32;
    (r, wide > 0xFFFF_FFFF, ((a ^ r) & (b ^ r)) >> 31 != 0)
}

#[inline]
fn cmp_s(a: i32, b: i32) -> u32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 8,
        std::cmp::Ordering::Greater => 4,
        std::cmp::Ordering::Equal => 2,
    }
}

#[inline]
fn cmp_u(a: u32, b: u32) -> u32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 8,
        std::cmp::Ordering::Greater => 4,
        std::cmp::Ordering::Equal => 2,
    }
}

fn trap_cond(to: u32, a: u32, b: u32) -> bool {
    let (sa, sb) = (a as i32, b as i32);
    (to & 16 != 0 && sa < sb)
        || (to & 8 != 0 && sa > sb)
        || (to & 4 != 0 && a == b)
        || (to & 2 != 0 && a < b)
        || (to & 1 != 0 && a > b)
}

#[inline]
fn ea_d(cpu: &Cpu, ra: usize, d: u32) -> u32 {
    if ra == 0 {
        d
    } else {
        cpu.gpr[ra].wrapping_add(d)
    }
}

#[inline]
fn ea_x(cpu: &Cpu, ra: usize, rb: usize) -> u32 {
    ea_d(cpu, ra, cpu.gpr[rb])
}

/// Evaluates BO/BI. `decrement` is false for `bcctr`, which never touches CTR.
fn branch_taken(cpu: &mut Cpu, bo: u32, bi: u32, decrement: bool) -> bool {
    let ctr_ok = if decrement && bo & 4 == 0 {
        cpu.ctr = cpu.ctr.wrapping_sub(1);
        (cpu.ctr != 0) != (bo & 2 != 0)
    } else {
        true
    };
    let cond_ok = bo & 16 != 0 || cpu.cr_bit(bi) == (bo & 8 != 0);
    ctr_ok && cond_ok
}

#[inline]
fn set_cr1(cpu: &mut Cpu) {
    let f = cpu.fpscr >> 28;
    cpu.set_cr_field(1, f);
}

pub fn execute(cpu: &mut Cpu, mem: &mut Mem, w: u32) -> R {
    op_main(cpu, mem, w)
}

/// [`execute`]'s body, always inlined: recompiled code calls it with a
/// constant word, so only that instruction's arm is left.
#[inline(always)]
pub fn op_main(cpu: &mut Cpu, mem: &mut Mem, w: u32) -> R {
    let op = w >> 26;
    let rd = ((w >> 21) & 31) as usize;
    let ra = ((w >> 16) & 31) as usize;
    let rb = ((w >> 11) & 31) as usize;
    let rc = w & 1 != 0;
    let simm = (w & 0xFFFF) as i16 as i32 as u32;
    let uimm = w & 0xFFFF;
    match op {
        3 => {
            if trap_cond(rd as u32, cpu.gpr[ra], simm) {
                return Err(Stop::Trap { pc: cpu.cia });
            }
        }
        4 => return paired(cpu, mem, w),
        7 => cpu.gpr[rd] = (cpu.gpr[ra] as i32).wrapping_mul(simm as i32) as u32,
        8 => {
            let (r, c, _) = add3(!cpu.gpr[ra], simm, 1);
            cpu.gpr[rd] = r;
            cpu.set_ca(c);
        }
        10 => {
            let so = cpu.xer >> 31;
            cpu.set_cr_field((w >> 23) & 7, cmp_u(cpu.gpr[ra], uimm) | so);
        }
        11 => {
            let so = cpu.xer >> 31;
            cpu.set_cr_field((w >> 23) & 7, cmp_s(cpu.gpr[ra] as i32, simm as i32) | so);
        }
        12 | 13 => {
            let (r, c, _) = add3(cpu.gpr[ra], simm, 0);
            cpu.gpr[rd] = r;
            cpu.set_ca(c);
            if op == 13 {
                cpu.update_cr0(r);
            }
        }
        14 => cpu.gpr[rd] = ea_d(cpu, ra, simm),
        15 => cpu.gpr[rd] = ea_d(cpu, ra, uimm << 16),
        16 => {
            if branch_taken(cpu, rd as u32, ra as u32, true) {
                let bd = (w & 0xFFFC) as i16 as i32 as u32;
                cpu.pc = if w & 2 != 0 { bd } else { cpu.cia.wrapping_add(bd) };
            }
            if w & 1 != 0 {
                cpu.lr = cpu.cia.wrapping_add(4);
            }
        }
        17 => {
            let next = cpu.cia.wrapping_add(4);
            cpu.exception(0xC00, next);
        }
        18 => {
            let li = ((w & 0x03FF_FFFC) ^ 0x0200_0000).wrapping_sub(0x0200_0000);
            cpu.pc = if w & 2 != 0 { li } else { cpu.cia.wrapping_add(li) };
            if w & 1 != 0 {
                cpu.lr = cpu.cia.wrapping_add(4);
            }
        }
        19 => return op19(cpu, w),
        20 | 21 | 23 => {
            let sh = if op == 23 { cpu.gpr[rb] & 31 } else { rb as u32 };
            let m = mask((w >> 6) & 31, (w >> 1) & 31);
            let rot = cpu.gpr[rd].rotate_left(sh);
            let r = if op == 20 { (rot & m) | (cpu.gpr[ra] & !m) } else { rot & m };
            cpu.gpr[ra] = r;
            if rc {
                cpu.update_cr0(r);
            }
        }
        24 => cpu.gpr[ra] = cpu.gpr[rd] | uimm,
        25 => cpu.gpr[ra] = cpu.gpr[rd] | (uimm << 16),
        26 => cpu.gpr[ra] = cpu.gpr[rd] ^ uimm,
        27 => cpu.gpr[ra] = cpu.gpr[rd] ^ (uimm << 16),
        28 | 29 => {
            let r = cpu.gpr[rd] & if op == 28 { uimm } else { uimm << 16 };
            cpu.gpr[ra] = r;
            cpu.update_cr0(r);
        }
        31 => return op31(cpu, mem, w),
        32..=45 => {
            let size = match op {
                32 | 33 | 36 | 37 => 4,
                34 | 35 | 38 | 39 => 1,
                _ => 2,
            };
            let store = matches!(op, 36..=39 | 44 | 45);
            let ea = ea_d(cpu, ra, simm);
            int_access(cpu, mem, ea, rd, ra, size, op == 42 || op == 43, store, op & 1 == 1, false)?;
        }
        46 => {
            let mut ea = ea_d(cpu, ra, simm);
            for r in rd..32 {
                cpu.gpr[r] = m!(cpu, mem.read_u32(ea));
                ea = ea.wrapping_add(4);
            }
        }
        47 => {
            let mut ea = ea_d(cpu, ra, simm);
            for r in rd..32 {
                m!(cpu, mem.write_u32(ea, cpu.gpr[r]));
                ea = ea.wrapping_add(4);
            }
        }
        48..=55 => {
            let ea = ea_d(cpu, ra, simm);
            let single = matches!(op, 48 | 49 | 52 | 53);
            fp_access(cpu, mem, ea, rd, ra, single, op >= 52, op & 1 == 1)?;
        }
        56 | 57 | 60 | 61 => {
            let d = ((w & 0xFFF) ^ 0x800).wrapping_sub(0x800);
            let ea = ea_d(cpu, ra, d);
            let single = (w >> 15) & 1 != 0;
            let i = ((w >> 12) & 7) as usize;
            if op < 60 {
                let v = quantized_load(cpu, mem, ea, i, single)?;
                cpu.fpr[rd] = v;
            } else {
                let v = cpu.fpr[rd];
                quantized_store(cpu, mem, ea, i, single, v)?;
            }
            if op & 1 == 1 {
                cpu.gpr[ra] = ea;
            }
        }
        59 => return fp_single(cpu, w),
        63 => return fp_double(cpu, w),
        _ => return unimpl(cpu, w),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[inline]
fn int_access(
    cpu: &mut Cpu,
    mem: &mut Mem,
    ea: u32,
    rd: usize,
    ra: usize,
    size: u32,
    signed: bool,
    store: bool,
    update: bool,
    reverse: bool,
) -> R {
    if store {
        let v = cpu.gpr[rd];
        match (size, reverse) {
            (1, _) => m!(cpu, mem.write_u8(ea, v as u8)),
            (2, false) => m!(cpu, mem.write_u16(ea, v as u16)),
            (2, true) => m!(cpu, mem.write_u16(ea, (v as u16).swap_bytes())),
            (_, false) => m!(cpu, mem.write_u32(ea, v)),
            (_, true) => m!(cpu, mem.write_u32(ea, v.swap_bytes())),
        }
        if update {
            cpu.gpr[ra] = ea;
        }
    } else {
        let v = match (size, signed, reverse) {
            (1, _, _) => m!(cpu, mem.read_u8(ea)) as u32,
            (2, true, _) => m!(cpu, mem.read_u16(ea)) as i16 as i32 as u32,
            (2, false, false) => m!(cpu, mem.read_u16(ea)) as u32,
            (2, false, true) => m!(cpu, mem.read_u16(ea)).swap_bytes() as u32,
            (_, _, false) => m!(cpu, mem.read_u32(ea)),
            (_, _, true) => m!(cpu, mem.read_u32(ea)).swap_bytes(),
        };
        if update {
            cpu.gpr[ra] = ea;
        }
        cpu.gpr[rd] = v;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[inline]
fn fp_access(cpu: &mut Cpu, mem: &mut Mem, ea: u32, fd: usize, ra: usize, single: bool, store: bool, update: bool) -> R {
    match (store, single) {
        (false, true) => {
            let v = f64::from_bits(fp::single_to_double(m!(cpu, mem.read_u32(ea))));
            cpu.fpr[fd] = [v, v];
        }
        (false, false) => cpu.fpr[fd][0] = f64::from_bits(m!(cpu, mem.read_u64(ea))),
        (true, true) => m!(cpu, mem.write_u32(ea, fp::double_to_single(cpu.fpr[fd][0].to_bits()))),
        (true, false) => m!(cpu, mem.write_u64(ea, cpu.fpr[fd][0].to_bits())),
    }
    if update {
        cpu.gpr[ra] = ea;
    }
    Ok(())
}

#[inline]
pub fn op19(cpu: &mut Cpu, w: u32) -> R {
    let d = (w >> 21) & 31;
    let a = (w >> 16) & 31;
    let b = (w >> 11) & 31;
    match (w >> 1) & 0x3FF {
        0 => {
            let f = cpu.cr_field((w >> 18) & 7);
            cpu.set_cr_field((w >> 23) & 7, f);
        }
        16 => {
            let target = cpu.lr & !3;
            if branch_taken(cpu, d, a, true) {
                cpu.pc = target;
            }
            if w & 1 != 0 {
                cpu.lr = cpu.cia.wrapping_add(4);
            }
        }
        528 => {
            let target = cpu.ctr & !3;
            if branch_taken(cpu, d, a, false) {
                cpu.pc = target;
            }
            if w & 1 != 0 {
                cpu.lr = cpu.cia.wrapping_add(4);
            }
        }
        50 => {
            const RFI_MASK: u32 = 0x87C0_FF73;
            cpu.msr = (cpu.msr & !RFI_MASK) | (cpu.spr[SPR_SRR1] & RFI_MASK);
            cpu.pc = cpu.spr[SPR_SRR0] & !3;
        }
        150 => {} // isync
        xo @ (33 | 129 | 193 | 225 | 257 | 289 | 417 | 449) => {
            let (x, y) = (cpu.cr_bit(a), cpu.cr_bit(b));
            let v = match xo {
                33 => !(x | y),
                129 => x & !y,
                193 => x ^ y,
                225 => !(x & y),
                257 => x & y,
                289 => x == y,
                417 => x | !y,
                _ => x | y,
            };
            cpu.set_cr_bit(d, v);
        }
        _ => return unimpl(cpu, w),
    }
    Ok(())
}

#[inline]
pub fn op31(cpu: &mut Cpu, mem: &mut Mem, w: u32) -> R {
    let rd = ((w >> 21) & 31) as usize;
    let ra = ((w >> 16) & 31) as usize;
    let rb = ((w >> 11) & 31) as usize;
    let rc = w & 1 != 0;
    let xo = (w >> 1) & 0x3FF;

    // XO-form arithmetic: 9-bit opcode plus the OE bit.
    let xo9 = xo & 0x1FF;
    if matches!(xo9, 8 | 10 | 11 | 40 | 75 | 104 | 136 | 138 | 200 | 202 | 232 | 234 | 235 | 266 | 459 | 491) {
        let (a, b, ca) = (cpu.gpr[ra], cpu.gpr[rb], cpu.ca());
        let (r, carry, ov) = match xo9 {
            8 => {
                let (r, c, o) = add3(!a, b, 1);
                (r, Some(c), o)
            }
            10 => {
                let (r, c, o) = add3(a, b, 0);
                (r, Some(c), o)
            }
            11 => ((((a as u64) * (b as u64)) >> 32) as u32, None, false),
            40 => {
                let (r, _, o) = add3(!a, b, 1);
                (r, None, o)
            }
            75 => ((((a as i32 as i64) * (b as i32 as i64)) >> 32) as u32, None, false),
            104 => (a.wrapping_neg(), None, a == 0x8000_0000),
            136 | 138 | 200 | 202 | 232 | 234 => {
                let (x, y) = match xo9 {
                    136 => (!a, b),
                    138 => (a, b),
                    200 => (!a, 0),
                    202 => (a, 0),
                    232 => (!a, u32::MAX),
                    _ => (a, u32::MAX),
                };
                let (r, c, o) = add3(x, y, ca);
                (r, Some(c), o)
            }
            235 => {
                let p = (a as i32 as i64) * (b as i32 as i64);
                (p as u32, None, p != p as i32 as i64)
            }
            266 => {
                let (r, _, o) = add3(a, b, 0);
                (r, None, o)
            }
            459 => {
                if b == 0 {
                    (0, None, true)
                } else {
                    (a / b, None, false)
                }
            }
            _ => {
                let (sa, sb) = (a as i32, b as i32);
                if sb == 0 || (sa == i32::MIN && sb == -1) {
                    // Hardware leaves all-ones for a negative dividend, zero otherwise.
                    (if sa < 0 { u32::MAX } else { 0 }, None, true)
                } else {
                    ((sa / sb) as u32, None, false)
                }
            }
        };
        cpu.gpr[rd] = r;
        if let Some(c) = carry {
            cpu.set_ca(c);
        }
        if w & 0x400 != 0 {
            cpu.set_ov(ov);
        }
        if rc {
            cpu.update_cr0(r);
        }
        return Ok(());
    }

    let rs = cpu.gpr[rd];
    match xo {
        0 | 32 => {
            let so = cpu.xer >> 31;
            let (a, b) = (cpu.gpr[ra], cpu.gpr[rb]);
            let c = if xo == 0 { cmp_s(a as i32, b as i32) } else { cmp_u(a, b) };
            cpu.set_cr_field((w >> 23) & 7, c | so);
        }
        4 => {
            if trap_cond(rd as u32, cpu.gpr[ra], cpu.gpr[rb]) {
                return Err(Stop::Trap { pc: cpu.cia });
            }
        }
        19 => cpu.gpr[rd] = cpu.cr,
        20 => {
            let ea = ea_x(cpu, ra, rb);
            cpu.gpr[rd] = m!(cpu, mem.read_u32(ea));
            cpu.reservation = Some(ea);
        }
        150 => {
            let ea = ea_x(cpu, ra, rb);
            let so = cpu.xer >> 31;
            if cpu.reservation.take().is_some() {
                m!(cpu, mem.write_u32(ea, rs));
                cpu.set_cr_field(0, 2 | so);
            } else {
                cpu.set_cr_field(0, so);
            }
        }
        // Integer loads and stores, X-form.
        23 | 55 | 87 | 119 | 279 | 311 | 343 | 375 | 151 | 183 | 215 | 247 | 407 | 439 | 534 | 662 | 790 | 918 => {
            let (size, signed, store, update, reverse) = match xo {
                23 => (4, false, false, false, false),
                55 => (4, false, false, true, false),
                87 => (1, false, false, false, false),
                119 => (1, false, false, true, false),
                279 => (2, false, false, false, false),
                311 => (2, false, false, true, false),
                343 => (2, true, false, false, false),
                375 => (2, true, false, true, false),
                151 => (4, false, true, false, false),
                183 => (4, false, true, true, false),
                215 => (1, false, true, false, false),
                247 => (1, false, true, true, false),
                407 => (2, false, true, false, false),
                439 => (2, false, true, true, false),
                534 => (4, false, false, false, true),
                662 => (4, false, true, false, true),
                790 => (2, false, false, false, true),
                _ => (2, false, true, false, true),
            };
            let ea = ea_x(cpu, ra, rb);
            int_access(cpu, mem, ea, rd, ra, size, signed, store, update, reverse)?;
        }
        // Floating loads and stores, X-form.
        535 | 567 | 599 | 631 | 663 | 695 | 727 | 759 => {
            let ea = ea_x(cpu, ra, rb);
            let single = matches!(xo, 535 | 567 | 663 | 695);
            fp_access(cpu, mem, ea, rd, ra, single, xo >= 663, matches!(xo, 567 | 631 | 695 | 759))?;
        }
        983 => {
            let ea = ea_x(cpu, ra, rb);
            m!(cpu, mem.write_u32(ea, cpu.fpr[rd][0].to_bits() as u32));
        }
        533 | 597 => {
            let (ea, n) = if xo == 597 {
                (ea_d(cpu, ra, 0), if rb == 0 { 32 } else { rb as u32 })
            } else {
                (ea_x(cpu, ra, rb), cpu.xer & 0x7F)
            };
            load_string(cpu, mem, ea, rd, n)?;
        }
        661 | 725 => {
            let (ea, n) = if xo == 725 {
                (ea_d(cpu, ra, 0), if rb == 0 { 32 } else { rb as u32 })
            } else {
                (ea_x(cpu, ra, rb), cpu.xer & 0x7F)
            };
            store_string(cpu, mem, ea, rd, n)?;
        }
        24 | 536 => {
            let n = cpu.gpr[rb] & 0x3F;
            let r = if n > 31 { 0 } else if xo == 24 { rs << n } else { rs >> n };
            cpu.gpr[ra] = r;
            if rc {
                cpu.update_cr0(r);
            }
        }
        792 | 824 => {
            let n = if xo == 824 { rb as u32 } else { cpu.gpr[rb] & 0x3F };
            let neg = (rs as i32) < 0;
            let (r, ca) = if n > 31 {
                (if neg { u32::MAX } else { 0 }, neg)
            } else if n == 0 {
                (rs, false)
            } else {
                (((rs as i32) >> n) as u32, neg && (rs << (32 - n)) != 0)
            };
            cpu.gpr[ra] = r;
            cpu.set_ca(ca);
            if rc {
                cpu.update_cr0(r);
            }
        }
        26 | 28 | 60 | 124 | 284 | 316 | 412 | 444 | 476 | 922 | 954 => {
            let b = cpu.gpr[rb];
            let r = match xo {
                26 => rs.leading_zeros(),
                28 => rs & b,
                60 => rs & !b,
                124 => !(rs | b),
                284 => !(rs ^ b),
                316 => rs ^ b,
                412 => rs | !b,
                444 => rs | b,
                476 => !(rs & b),
                922 => rs as i16 as i32 as u32,
                _ => rs as i8 as i32 as u32,
            };
            cpu.gpr[ra] = r;
            if rc {
                cpu.update_cr0(r);
            }
        }
        83 => cpu.gpr[rd] = cpu.msr,
        146 => cpu.msr = rs,
        210 => cpu.sr[ra & 15] = rs,
        242 => cpu.sr[(cpu.gpr[rb] >> 28) as usize] = rs,
        595 => cpu.gpr[rd] = cpu.sr[ra & 15],
        659 => cpu.gpr[rd] = cpu.sr[(cpu.gpr[rb] >> 28) as usize],
        144 => {
            let crm = (w >> 12) & 0xFF;
            for f in 0..8 {
                if crm & (0x80 >> f) != 0 {
                    cpu.set_cr_field(f, rs >> (28 - 4 * f));
                }
            }
        }
        512 => {
            cpu.set_cr_field((w >> 23) & 7, cpu.xer >> 28);
            cpu.xer &= 0x0FFF_FFFF;
        }
        339 | 371 => {
            let n = (((w >> 16) & 0x1F) | (((w >> 11) & 0x1F) << 5)) as usize;
            cpu.gpr[rd] = match n {
                SPR_XER => cpu.xer,
                SPR_LR => cpu.lr,
                SPR_CTR => cpu.ctr,
                SPR_DEC => cpu.read_dec(),
                268 => cpu.timebase() as u32,
                269 => (cpu.timebase() >> 32) as u32,
                _ => cpu.spr[n],
            };
        }
        467 => {
            let n = (((w >> 16) & 0x1F) | (((w >> 11) & 0x1F) << 5)) as usize;
            match n {
                SPR_XER => cpu.xer = rs,
                SPR_LR => cpu.lr = rs,
                SPR_CTR => cpu.ctr = rs,
                SPR_DEC => cpu.write_dec(rs),
                284 => {
                    let tb = cpu.timebase();
                    cpu.set_timebase((tb & !0xFFFF_FFFF) | rs as u64);
                }
                285 => {
                    let tb = cpu.timebase();
                    cpu.set_timebase(((rs as u64) << 32) | (tb & 0xFFFF_FFFF));
                }
                _ => {
                    cpu.spr[n] = rs;
                    if n == SPR_DMAL {
                        locked_cache_dma(cpu, mem)?;
                    }
                }
            }
        }
        1014 => dcbz(cpu, mem, ea_x(cpu, ra, rb))?,
        // Cache and ordering hints: no effect on an interpreter with coherent memory.
        54 | 86 | 246 | 278 | 470 | 982 | 598 | 854 | 306 | 566 => {}
        _ => return unimpl(cpu, w),
    }
    Ok(())
}

fn load_string(cpu: &mut Cpu, mem: &mut Mem, mut ea: u32, rd: usize, n: u32) -> R {
    let (mut r, mut shift) = (rd, 24i32);
    for _ in 0..n {
        if shift == 24 {
            cpu.gpr[r] = 0;
        }
        cpu.gpr[r] |= (m!(cpu, mem.read_u8(ea)) as u32) << shift;
        ea = ea.wrapping_add(1);
        shift -= 8;
        if shift < 0 {
            shift = 24;
            r = (r + 1) % 32;
        }
    }
    Ok(())
}

fn store_string(cpu: &mut Cpu, mem: &mut Mem, mut ea: u32, rd: usize, n: u32) -> R {
    let (mut r, mut shift) = (rd, 24i32);
    for _ in 0..n {
        m!(cpu, mem.write_u8(ea, (cpu.gpr[r] >> shift) as u8));
        ea = ea.wrapping_add(1);
        shift -= 8;
        if shift < 0 {
            shift = 24;
            r = (r + 1) % 32;
        }
    }
    Ok(())
}

fn dcbz(cpu: &mut Cpu, mem: &mut Mem, ea: u32) -> R {
    let line = ea & !31;
    match mem.ram_mut(line, 32) {
        Some(s) => {
            s.fill(0);
            Ok(())
        }
        None => Err(Stop::Fault { pc: cpu.cia, fault: Fault { addr: line, write: true } }),
    }
}

/// Locked-cache DMA, triggered by writing DMAL with its T bit set.
fn locked_cache_dma(cpu: &mut Cpu, mem: &mut Mem) -> R {
    let (dmau, dmal) = (cpu.spr[SPR_DMAU], cpu.spr[SPR_DMAL]);
    if dmal & 2 == 0 {
        return Ok(());
    }
    let mut lines = ((dmau & 0x1F) << 2) | ((dmal >> 2) & 3);
    if lines == 0 {
        lines = 128;
    }
    let len = lines as usize * 32;
    let (main, cache) = (dmau & !0x1F, dmal & !0x1F);
    let (src, dst) = if dmal & 0x10 != 0 { (main, cache) } else { (cache, main) };
    let fault = |addr, write| Stop::Fault { pc: cpu.cia, fault: Fault { addr, write } };
    let data = mem.ram(src, len).ok_or_else(|| fault(src, false))?.to_vec();
    mem.ram_mut(dst, len).ok_or_else(|| fault(dst, true))?.copy_from_slice(&data);
    cpu.spr[SPR_DMAL] &= !2;
    Ok(())
}

#[inline]
pub fn fp_single(cpu: &mut Cpu, w: u32) -> R {
    let (fd, fa, fb, fc) = fields(w);
    let (a, b, c) = (cpu.fpr[fa][0], cpu.fpr[fb][0], fp::force25(cpu.fpr[fc][0]));
    let v = match (w >> 1) & 31 {
        18 => a / b,
        20 => a - b,
        21 => a + b,
        24 => fp::reciprocal_estimate(b),
        25 => a * c,
        28 => fp::fma(a, c, -b),
        29 => fp::fma(a, c, b),
        30 => -fp::fma(a, c, -b),
        31 => -fp::fma(a, c, b),
        _ => return unimpl(cpu, w),
    };
    let v = fp::round_single(v);
    cpu.fpr[fd] = [v, v];
    if w & 1 != 0 {
        set_cr1(cpu);
    }
    Ok(())
}

fn fields(w: u32) -> (usize, usize, usize, usize) {
    (
        ((w >> 21) & 31) as usize,
        ((w >> 16) & 31) as usize,
        ((w >> 11) & 31) as usize,
        ((w >> 6) & 31) as usize,
    )
}

#[inline]
pub fn fp_double(cpu: &mut Cpu, w: u32) -> R {
    let (fd, fa, fb, fc) = fields(w);
    let (a, b, c) = (cpu.fpr[fa][0], cpu.fpr[fb][0], cpu.fpr[fc][0]);
    let xo5 = (w >> 1) & 31;
    if xo5 >= 16 {
        cpu.fpr[fd][0] = match xo5 {
            18 => a / b,
            20 => a - b,
            21 => a + b,
            23 => {
                if a >= 0.0 {
                    c
                } else {
                    b
                }
            }
            25 => a * c,
            26 => fp::rsqrt_estimate(b),
            28 => fp::fma(a, c, -b),
            29 => fp::fma(a, c, b),
            30 => -fp::fma(a, c, -b),
            31 => -fp::fma(a, c, b),
            _ => return unimpl(cpu, w),
        };
    } else {
        match (w >> 1) & 0x3FF {
            0 | 32 => {
                let cc = fp::compare(a, b);
                cpu.set_cr_field((w >> 23) & 7, cc);
                cpu.fpscr = (cpu.fpscr & !0xF000) | (cc << 12);
                return Ok(());
            }
            12 => {
                let v = fp::round_single(b);
                cpu.fpr[fd] = [v, v];
            }
            14 => cpu.fpr[fd][0] = f64::from_bits(fp::to_int_bits(b, cpu.fpscr & 3)),
            15 => cpu.fpr[fd][0] = f64::from_bits(fp::to_int_bits(b, 1)),
            38 => cpu.fpscr |= 1 << (31 - fd),
            70 => cpu.fpscr &= !(1 << (31 - fd)),
            40 => cpu.fpr[fd][0] = f64::from_bits(b.to_bits() ^ SIGN),
            72 => cpu.fpr[fd][0] = b,
            136 => cpu.fpr[fd][0] = f64::from_bits(b.to_bits() | SIGN),
            264 => cpu.fpr[fd][0] = f64::from_bits(b.to_bits() & !SIGN),
            64 => {
                let f = (cpu.fpscr >> (28 - 4 * ((w >> 18) & 7))) & 0xF;
                cpu.set_cr_field((w >> 23) & 7, f);
                return Ok(());
            }
            583 => cpu.fpr[fd][0] = f64::from_bits(0xFFF8_0000_0000_0000 | cpu.fpscr as u64),
            711 => {
                let fm = (w >> 17) & 0xFF;
                let mut m = 0u32;
                for f in 0..8 {
                    if fm & (0x80 >> f) != 0 {
                        m |= 0xF << (28 - 4 * f);
                    }
                }
                cpu.fpscr = (cpu.fpscr & !m) | (b.to_bits() as u32 & m);
            }
            134 => {
                let s = 28 - 4 * ((w >> 23) & 7);
                cpu.fpscr = (cpu.fpscr & !(0xF << s)) | (((w >> 12) & 0xF) << s);
            }
            _ => return unimpl(cpu, w),
        }
    }
    if w & 1 != 0 {
        set_cr1(cpu);
    }
    Ok(())
}

fn sext6(v: u32) -> i32 {
    ((v & 0x3F) as i32 ^ 0x20) - 0x20
}

/// 2^n for a GQR scale (n in -32..=32), built from its bits: the exact
/// value `powi` gives, without a call (which made every `psq_l` and
/// `psq_st` spill registers, even for float data, which has no scale).
#[inline(always)]
fn pow2(n: i32) -> f64 {
    f64::from_bits(((1023 + n) as u64) << 52)
}

/// `psq_l*`: loads one or two values through GQR `i`. A single load sets ps1 = 1.0.
///
/// Float data (load types 0-3), what the game's matrix code loads, is done
/// here; the integer types in [`quantized_load_int`], out of line. Inlined
/// whole, every one of the thousands of `psq_l` in the recompiled code
/// carried all eight types, unrolled for both values (on the Switch's
/// Cortex-A57, whose 48 KB instruction cache that overflowed).
#[inline(always)]
fn quantized_load(cpu: &mut Cpu, mem: &mut Mem, ea: u32, i: usize, single: bool) -> Result<[f64; 2], Stop> {
    let gqr = cpu.gqr(i);
    if (gqr >> 16) & 7 >= 4 {
        return quantized_load_int(cpu, mem, ea, gqr, single);
    }
    let x = f64::from_bits(fp::single_to_double(m!(cpu, mem.read_u32(ea))));
    let y = if single { 1.0 } else { f64::from_bits(fp::single_to_double(m!(cpu, mem.read_u32(ea.wrapping_add(4))))) };
    Ok([x, y])
}

/// [`quantized_load`] of the integer types (4-7), dequantized by the GQR's scale.
#[inline(never)]
fn quantized_load_int(cpu: &mut Cpu, mem: &mut Mem, ea: u32, gqr: u32, single: bool) -> Result<[f64; 2], Stop> {
    let ty = (gqr >> 16) & 7;
    let scale = pow2(-sext6(gqr >> 24));
    let size = if ty & 1 == 0 { 1 } else { 2 };
    let mut out = [0.0, 1.0];
    for (k, slot) in out.iter_mut().enumerate().take(if single { 1 } else { 2 }) {
        let a = ea.wrapping_add(k as u32 * size);
        *slot = match ty {
            4 => m!(cpu, mem.read_u8(a)) as f64 * scale,
            5 => m!(cpu, mem.read_u16(a)) as f64 * scale,
            6 => m!(cpu, mem.read_u8(a)) as i8 as f64 * scale,
            _ => m!(cpu, mem.read_u16(a)) as i16 as f64 * scale,
        };
    }
    Ok(out)
}

/// `psq_st*`: stores ps0 (and ps1) through GQR `i`, saturating integer
/// types. Float data inline, the integer types out of line, as
/// [`quantized_load`].
#[inline(always)]
fn quantized_store(cpu: &mut Cpu, mem: &mut Mem, ea: u32, i: usize, single: bool, v: [f64; 2]) -> R {
    let gqr = cpu.gqr(i);
    if gqr & 7 >= 4 {
        return quantized_store_int(cpu, mem, ea, gqr, single, v);
    }
    m!(cpu, mem.write_u32(ea, fp::double_to_single(v[0].to_bits())));
    if !single {
        m!(cpu, mem.write_u32(ea.wrapping_add(4), fp::double_to_single(v[1].to_bits())));
    }
    Ok(())
}

/// [`quantized_store`] of the integer types (4-7), quantized by the GQR's scale.
#[inline(never)]
fn quantized_store_int(cpu: &mut Cpu, mem: &mut Mem, ea: u32, gqr: u32, single: bool, v: [f64; 2]) -> R {
    let ty = gqr & 7;
    let scale = pow2(sext6(gqr >> 8));
    let size = if ty & 1 == 0 { 1 } else { 2 };
    for (k, &x) in v.iter().enumerate().take(if single { 1 } else { 2 }) {
        let a = ea.wrapping_add(k as u32 * size);
        let q = x * scale;
        match ty {
            4 => m!(cpu, mem.write_u8(a, q as u8)),
            5 => m!(cpu, mem.write_u16(a, q as u16)),
            6 => m!(cpu, mem.write_u8(a, q as i8 as u8)),
            _ => m!(cpu, mem.write_u16(a, q as i16 as u16)),
        }
    }
    Ok(())
}

#[inline]
pub fn paired(cpu: &mut Cpu, mem: &mut Mem, w: u32) -> R {
    let (fd, fa, fb, fc) = fields(w);
    let [a0, a1] = cpu.fpr[fa];
    let [b0, b1] = cpu.fpr[fb];
    let [c0, c1] = cpu.fpr[fc];
    let rs = fp::round_single;
    let f25 = fp::force25;
    let xo5 = (w >> 1) & 31;

    let result: [f64; 2] = match xo5 {
        10 => [rs(a0 + b1), rs(c1)],
        11 => [rs(c0), rs(a0 + b1)],
        12 => [rs(a0 * f25(c0)), rs(a1 * f25(c0))],
        13 => [rs(a0 * f25(c1)), rs(a1 * f25(c1))],
        14 => [rs(fp::fma(a0, f25(c0), b0)), rs(fp::fma(a1, f25(c0), b1))],
        15 => [rs(fp::fma(a0, f25(c1), b0)), rs(fp::fma(a1, f25(c1), b1))],
        18 => [rs(a0 / b0), rs(a1 / b1)],
        20 => [rs(a0 - b0), rs(a1 - b1)],
        21 => [rs(a0 + b0), rs(a1 + b1)],
        23 => [if a0 >= 0.0 { c0 } else { b0 }, if a1 >= 0.0 { c1 } else { b1 }],
        24 => [fp::reciprocal_estimate(b0), fp::reciprocal_estimate(b1)],
        25 => [rs(a0 * f25(c0)), rs(a1 * f25(c1))],
        26 => [rs(fp::rsqrt_estimate(b0)), rs(fp::rsqrt_estimate(b1))],
        28 => [rs(fp::fma(a0, f25(c0), -b0)), rs(fp::fma(a1, f25(c1), -b1))],
        29 => [rs(fp::fma(a0, f25(c0), b0)), rs(fp::fma(a1, f25(c1), b1))],
        30 => [-rs(fp::fma(a0, f25(c0), -b0)), -rs(fp::fma(a1, f25(c1), -b1))],
        31 => [-rs(fp::fma(a0, f25(c0), b0)), -rs(fp::fma(a1, f25(c1), b1))],
        _ => {
            // psq_lx / psq_stx / psq_lux / psq_stux use a 6-bit opcode.
            let xo6 = (w >> 1) & 0x3F;
            if matches!(xo6, 6 | 7 | 38 | 39) {
                let ea = ea_x(cpu, fa, fb);
                let single = (w >> 10) & 1 != 0;
                let i = ((w >> 7) & 7) as usize;
                if xo6 & 1 == 0 {
                    let v = quantized_load(cpu, mem, ea, i, single)?;
                    cpu.fpr[fd] = v;
                } else {
                    let v = cpu.fpr[fd];
                    quantized_store(cpu, mem, ea, i, single, v)?;
                }
                if xo6 >= 38 {
                    cpu.gpr[fa] = ea;
                }
                return Ok(());
            }
            match (w >> 1) & 0x3FF {
                xo @ (0 | 32 | 64 | 96) => {
                    let cc = if xo < 64 { fp::compare(a0, b0) } else { fp::compare(a1, b1) };
                    cpu.set_cr_field((w >> 23) & 7, cc);
                    cpu.fpscr = (cpu.fpscr & !0xF000) | (cc << 12);
                    return Ok(());
                }
                40 => [-b0, -b1],
                72 => [b0, b1],
                136 => [-b0.abs(), -b1.abs()],
                264 => [b0.abs(), b1.abs()],
                528 => [a0, b0],
                560 => [a0, b1],
                592 => [a1, b0],
                624 => [a1, b1],
                1014 => return dcbz(cpu, mem, ea_x(cpu, fa, fb)),
                _ => return unimpl(cpu, w),
            }
        }
    };
    cpu.fpr[fd] = result;
    if w & 1 != 0 {
        set_cr1(cpu);
    }
    Ok(())
}
