//! Small handlers for the most common instructions, picked when a block is
//! decoded. Going through `interp::execute` costs a large prologue and two
//! levels of dispatch even for an `addi`; these do only the work. Each one
//! mirrors its arm in `interp` exactly (checked by `tests/fast_paths.rs`);
//! anything not listed in [`handler`] keeps the generic path.
//!
//! Recompiled code (P6, `ppc-recomp`) calls the same handlers with constant
//! instruction words, so both engines share one definition of each
//! instruction; [`handler_name`] tells the generator which one to call.

use guest_mem::Mem;

use crate::{fp, Cpu, Stop};

pub type Handler = fn(&mut Cpu, &mut Mem, u32) -> Result<(), Stop>;
type R = Result<(), Stop>;

macro_rules! m {
    ($cpu:expr, $e:expr) => {
        $e.map_err(|fault| Stop::Fault { pc: $cpu.cia, fault })?
    };
}

/// How a load or store handler reaches memory: [`Checked`] (any address:
/// MMIO, faults, code-write tracking) or [`Raw`] (recompiled code, for
/// addresses relative to r1, r2 or r13, which are always stack or small
/// data in RAM: `Mem::raw_read_u32` and co.).
pub trait Access {
    fn r8(m: &mut Mem, a: u32) -> Result<u8, guest_mem::Fault>;
    fn r16(m: &mut Mem, a: u32) -> Result<u16, guest_mem::Fault>;
    fn r32(m: &mut Mem, a: u32) -> Result<u32, guest_mem::Fault>;
    fn r64(m: &mut Mem, a: u32) -> Result<u64, guest_mem::Fault>;
    fn w8(m: &mut Mem, a: u32, v: u8) -> Result<(), guest_mem::Fault>;
    fn w16(m: &mut Mem, a: u32, v: u16) -> Result<(), guest_mem::Fault>;
    fn w32(m: &mut Mem, a: u32, v: u32) -> Result<(), guest_mem::Fault>;
    fn w64(m: &mut Mem, a: u32, v: u64) -> Result<(), guest_mem::Fault>;
}

pub struct Checked;
pub struct Raw;

macro_rules! access {
    ($t:ty, $r8:ident, $r16:ident, $r32:ident, $r64:ident, $w8:ident, $w16:ident, $w32:ident, $w64:ident) => {
        impl Access for $t {
            #[inline(always)]
            fn r8(m: &mut Mem, a: u32) -> Result<u8, guest_mem::Fault> {
                m.$r8(a)
            }
            #[inline(always)]
            fn r16(m: &mut Mem, a: u32) -> Result<u16, guest_mem::Fault> {
                m.$r16(a)
            }
            #[inline(always)]
            fn r32(m: &mut Mem, a: u32) -> Result<u32, guest_mem::Fault> {
                m.$r32(a)
            }
            #[inline(always)]
            fn r64(m: &mut Mem, a: u32) -> Result<u64, guest_mem::Fault> {
                m.$r64(a)
            }
            #[inline(always)]
            fn w8(m: &mut Mem, a: u32, v: u8) -> Result<(), guest_mem::Fault> {
                m.$w8(a, v)
            }
            #[inline(always)]
            fn w16(m: &mut Mem, a: u32, v: u16) -> Result<(), guest_mem::Fault> {
                m.$w16(a, v)
            }
            #[inline(always)]
            fn w32(m: &mut Mem, a: u32, v: u32) -> Result<(), guest_mem::Fault> {
                m.$w32(a, v)
            }
            #[inline(always)]
            fn w64(m: &mut Mem, a: u32, v: u64) -> Result<(), guest_mem::Fault> {
                m.$w64(a, v)
            }
        }
    };
}
access!(Checked, read_u8, read_u16, read_u32, read_u64, write_u8, write_u16, write_u32, write_u64);

/// Never an error ([`Mem::raw_read_u32`] and co.), so the handlers'
/// error paths fold away.
impl Access for Raw {
    #[inline(always)]
    fn r8(m: &mut Mem, a: u32) -> Result<u8, guest_mem::Fault> {
        Ok(m.raw_read_u8(a))
    }
    #[inline(always)]
    fn r16(m: &mut Mem, a: u32) -> Result<u16, guest_mem::Fault> {
        Ok(m.raw_read_u16(a))
    }
    #[inline(always)]
    fn r32(m: &mut Mem, a: u32) -> Result<u32, guest_mem::Fault> {
        Ok(m.raw_read_u32(a))
    }
    #[inline(always)]
    fn r64(m: &mut Mem, a: u32) -> Result<u64, guest_mem::Fault> {
        Ok(m.raw_read_u64(a))
    }
    #[inline(always)]
    fn w8(m: &mut Mem, a: u32, v: u8) -> Result<(), guest_mem::Fault> {
        m.raw_write_u8(a, v);
        Ok(())
    }
    #[inline(always)]
    fn w16(m: &mut Mem, a: u32, v: u16) -> Result<(), guest_mem::Fault> {
        m.raw_write_u16(a, v);
        Ok(())
    }
    #[inline(always)]
    fn w32(m: &mut Mem, a: u32, v: u32) -> Result<(), guest_mem::Fault> {
        m.raw_write_u32(a, v);
        Ok(())
    }
    #[inline(always)]
    fn w64(m: &mut Mem, a: u32, v: u64) -> Result<(), guest_mem::Fault> {
        m.raw_write_u64(a, v);
        Ok(())
    }
}

/// Whether recompiled code may use [`Raw`] for the load or store `w`: its
/// base register is r1 (stack), r2 or r13 (small data).
pub fn raw_ok(w: u32) -> bool {
    // D-form loads and stores only (not indexed: rb could point anywhere).
    matches!(w >> 26, 32..=55) && matches!((w >> 16) & 31, 1 | 2 | 13)
}

#[inline(always)]
fn rd(w: u32) -> usize {
    ((w >> 21) & 31) as usize
}
#[inline(always)]
fn ra(w: u32) -> usize {
    ((w >> 16) & 31) as usize
}
#[inline(always)]
fn rb(w: u32) -> usize {
    ((w >> 11) & 31) as usize
}
#[inline(always)]
fn simm(w: u32) -> u32 {
    (w & 0xFFFF) as i16 as i32 as u32
}
#[inline(always)]
fn ea_d(cpu: &Cpu, w: u32) -> u32 {
    let a = ra(w);
    if a == 0 {
        simm(w)
    } else {
        cpu.gpr[a].wrapping_add(simm(w))
    }
}
#[inline(always)]
fn ea_x(cpu: &Cpu, w: u32) -> u32 {
    let a = ra(w);
    let b = cpu.gpr[rb(w)];
    if a == 0 {
        b
    } else {
        cpu.gpr[a].wrapping_add(b)
    }
}
#[inline(always)]
fn ord(o: std::cmp::Ordering) -> u32 {
    match o {
        std::cmp::Ordering::Less => 8,
        std::cmp::Ordering::Greater => 4,
        std::cmp::Ordering::Equal => 2,
    }
}
/// BO/BI of a conditional branch (`decrement` false for `bcctr`, which
/// never touches CTR).
#[inline(always)]
pub fn branch_taken(cpu: &mut Cpu, bo: u32, bi: u32, decrement: bool) -> bool {
    let ctr_ok = if decrement && bo & 4 == 0 {
        cpu.ctr = cpu.ctr.wrapping_sub(1);
        (cpu.ctr != 0) != (bo & 2 != 0)
    } else {
        true
    };
    ctr_ok && (bo & 16 != 0 || cpu.cr_bit(bi) == (bo & 8 != 0))
}

#[inline(always)]
pub fn addi(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[rd(w)] = ea_d(c, w);
    Ok(())
}
#[inline(always)]
pub fn addis(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let a = ra(w);
    let v = w << 16;
    c.gpr[rd(w)] = if a == 0 { v } else { c.gpr[a].wrapping_add(v) };
    Ok(())
}
#[inline(always)]
pub fn ori(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[ra(w)] = c.gpr[rd(w)] | (w & 0xFFFF);
    Ok(())
}
#[inline(always)]
pub fn oris(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[ra(w)] = c.gpr[rd(w)] | (w << 16);
    Ok(())
}
#[inline(always)]
pub fn andi(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let r = c.gpr[rd(w)] & (w & 0xFFFF);
    c.gpr[ra(w)] = r;
    c.update_cr0(r);
    Ok(())
}
#[inline(always)]
pub fn cmpli(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let so = c.xer >> 31;
    c.set_cr_field((w >> 23) & 7, ord(c.gpr[ra(w)].cmp(&(w & 0xFFFF))) | so);
    Ok(())
}
#[inline(always)]
pub fn cmpi(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let so = c.xer >> 31;
    c.set_cr_field((w >> 23) & 7, ord((c.gpr[ra(w)] as i32).cmp(&(simm(w) as i32))) | so);
    Ok(())
}
#[inline(always)]
pub fn rlwinm(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let m = crate::interp::mask((w >> 6) & 31, (w >> 1) & 31);
    let r = c.gpr[rd(w)].rotate_left(rb(w) as u32) & m;
    c.gpr[ra(w)] = r;
    if w & 1 != 0 {
        c.update_cr0(r);
    }
    Ok(())
}
#[inline(always)]
pub fn b(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let li = ((w & 0x03FF_FFFC) ^ 0x0200_0000).wrapping_sub(0x0200_0000);
    c.pc = if w & 2 != 0 { li } else { c.cia.wrapping_add(li) };
    if w & 1 != 0 {
        c.lr = c.cia.wrapping_add(4);
    }
    Ok(())
}
#[inline(always)]
pub fn bc(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    if branch_taken(c, rd(w) as u32, ra(w) as u32, true) {
        let bd = (w & 0xFFFC) as i16 as i32 as u32;
        c.pc = if w & 2 != 0 { bd } else { c.cia.wrapping_add(bd) };
    }
    if w & 1 != 0 {
        c.lr = c.cia.wrapping_add(4);
    }
    Ok(())
}
#[inline(always)]
pub fn bclr(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let target = c.lr & !3;
    if branch_taken(c, rd(w) as u32, ra(w) as u32, true) {
        c.pc = target;
    }
    if w & 1 != 0 {
        c.lr = c.cia.wrapping_add(4);
    }
    Ok(())
}
#[inline(always)]
pub fn bcctr(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let target = c.ctr & !3;
    if branch_taken(c, rd(w) as u32, ra(w) as u32, false) {
        c.pc = target;
    }
    if w & 1 != 0 {
        c.lr = c.cia.wrapping_add(4);
    }
    Ok(())
}

#[inline(always)]
pub fn lwz(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lwz_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lwz_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    c.gpr[rd(w)] = m!(c, A::r32(m, ea));
    Ok(())
}
#[inline(always)]
pub fn lwzu(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lwzu_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lwzu_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    let v = m!(c, A::r32(m, ea));
    c.gpr[ra(w)] = ea;
    c.gpr[rd(w)] = v;
    Ok(())
}
#[inline(always)]
pub fn lbz(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lbz_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lbz_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    c.gpr[rd(w)] = u32::from(m!(c, A::r8(m, ea)));
    Ok(())
}
#[inline(always)]
pub fn lhz(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lhz_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lhz_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    c.gpr[rd(w)] = u32::from(m!(c, A::r16(m, ea)));
    Ok(())
}
#[inline(always)]
pub fn lha(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lha_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lha_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    c.gpr[rd(w)] = m!(c, A::r16(m, ea)) as i16 as i32 as u32;
    Ok(())
}
#[inline(always)]
pub fn stw(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    stw_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn stw_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    m!(c, A::w32(m, ea, c.gpr[rd(w)]));
    Ok(())
}
#[inline(always)]
pub fn stwu(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    stwu_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn stwu_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    m!(c, A::w32(m, ea, c.gpr[rd(w)]));
    c.gpr[ra(w)] = ea;
    Ok(())
}
#[inline(always)]
pub fn stb(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    stb_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn stb_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    m!(c, A::w8(m, ea, c.gpr[rd(w)] as u8));
    Ok(())
}
#[inline(always)]
pub fn sth(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    sth_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn sth_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    m!(c, A::w16(m, ea, c.gpr[rd(w)] as u16));
    Ok(())
}
#[inline(always)]
pub fn lfs(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lfs_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lfs_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    let v = f64::from_bits(fp::single_to_double(m!(c, A::r32(m, ea))));
    c.fpr[rd(w)] = [v, v];
    Ok(())
}
#[inline(always)]
pub fn lfd(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    lfd_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn lfd_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    c.fpr[rd(w)][0] = f64::from_bits(m!(c, A::r64(m, ea)));
    Ok(())
}
#[inline(always)]
pub fn stfs(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    stfs_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn stfs_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    m!(c, A::w32(m, ea, fp::double_to_single(c.fpr[rd(w)][0].to_bits())));
    Ok(())
}
#[inline(always)]
pub fn stfd(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    stfd_via::<Checked>(c, m, w)
}
#[inline(always)]
pub fn stfd_via<A: Access>(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_d(c, w);
    m!(c, A::w64(m, ea, c.fpr[rd(w)][0].to_bits()));
    Ok(())
}
#[inline(always)]
pub fn lwzx(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_x(c, w);
    c.gpr[rd(w)] = m!(c, m.read_u32(ea));
    Ok(())
}
#[inline(always)]
pub fn stwx(c: &mut Cpu, m: &mut Mem, w: u32) -> R {
    let ea = ea_x(c, w);
    m!(c, m.write_u32(ea, c.gpr[rd(w)]));
    Ok(())
}

#[inline(always)]
pub fn add(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[rd(w)] = c.gpr[ra(w)].wrapping_add(c.gpr[rb(w)]);
    Ok(())
}
#[inline(always)]
pub fn subf(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[rd(w)] = c.gpr[rb(w)].wrapping_sub(c.gpr[ra(w)]);
    Ok(())
}
#[inline(always)]
pub fn or(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[ra(w)] = c.gpr[rd(w)] | c.gpr[rb(w)];
    Ok(())
}
#[inline(always)]
pub fn and(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[ra(w)] = c.gpr[rd(w)] & c.gpr[rb(w)];
    Ok(())
}
#[inline(always)]
pub fn xor(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[ra(w)] = c.gpr[rd(w)] ^ c.gpr[rb(w)];
    Ok(())
}
#[inline(always)]
pub fn cmp(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let so = c.xer >> 31;
    c.set_cr_field((w >> 23) & 7, ord((c.gpr[ra(w)] as i32).cmp(&(c.gpr[rb(w)] as i32))) | so);
    Ok(())
}
#[inline(always)]
pub fn cmpl(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    let so = c.xer >> 31;
    c.set_cr_field((w >> 23) & 7, ord(c.gpr[ra(w)].cmp(&c.gpr[rb(w)])) | so);
    Ok(())
}
#[inline(always)]
pub fn mflr(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[rd(w)] = c.lr;
    Ok(())
}
#[inline(always)]
pub fn mtlr(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.lr = c.gpr[rd(w)];
    Ok(())
}
#[inline(always)]
pub fn mfctr(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.gpr[rd(w)] = c.ctr;
    Ok(())
}
#[inline(always)]
pub fn mtctr(c: &mut Cpu, _: &mut Mem, w: u32) -> R {
    c.ctr = c.gpr[rd(w)];
    Ok(())
}

/// A dedicated handler for `w`, if it has one.
pub(crate) fn handler(w: u32) -> Option<Handler> {
    entry(w).map(|e| e.1)
}

/// The name of [`handler`]'s choice for `w` (a function in this module).
pub fn handler_name(w: u32) -> Option<&'static str> {
    entry(w).map(|e| e.0)
}

fn entry(w: u32) -> Option<(&'static str, Handler)> {
    macro_rules! h {
        ($f:ident) => {
            (stringify!($f), $f as Handler)
        };
    }
    let rc = w & 1 != 0;
    let spr = ((w >> 16) & 0x1F) | (((w >> 11) & 0x1F) << 5);
    Some(match w >> 26 {
        10 => h!(cmpli),
        11 => h!(cmpi),
        14 => h!(addi),
        15 => h!(addis),
        16 => h!(bc),
        18 => h!(b),
        19 => match (w >> 1) & 0x3FF {
            16 => h!(bclr),
            528 => h!(bcctr),
            _ => return None,
        },
        21 => h!(rlwinm),
        24 => h!(ori),
        25 => h!(oris),
        28 => h!(andi),
        32 => h!(lwz),
        33 => h!(lwzu),
        34 => h!(lbz),
        36 => h!(stw),
        37 => h!(stwu),
        38 => h!(stb),
        40 => h!(lhz),
        42 => h!(lha),
        44 => h!(sth),
        48 => h!(lfs),
        50 => h!(lfd),
        52 => h!(stfs),
        54 => h!(stfd),
        31 if !rc => match (w >> 1) & 0x3FF {
            0 => h!(cmp),
            32 => h!(cmpl),
            23 => h!(lwzx),
            151 => h!(stwx),
            266 => h!(add),
            40 => h!(subf),
            444 => h!(or),
            28 => h!(and),
            316 => h!(xor),
            339 if spr == 8 => h!(mflr),
            339 if spr == 9 => h!(mfctr),
            467 if spr == 8 => h!(mtlr),
            467 if spr == 9 => h!(mtctr),
            _ => return None,
        },
        _ => return None,
    })
}
