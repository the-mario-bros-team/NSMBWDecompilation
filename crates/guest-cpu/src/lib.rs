//! Broadway (Wii PowerPC 750CL) CPU state and interpreter.
//!
//! The interpreter is the port's reference execution engine (docs/PORT_PLAN.md,
//! section 2.2): the first way to run the game, the oracle the recompiler is
//! checked against, and the fallback for code the recompiler did not cover.

pub mod blocks;
pub mod fast;
pub mod fp;
pub mod interp;
pub mod recomp;

use guest_mem::{Fault, Mem};

/// Guest return registers captured by a synchronous PowerPC call.
#[derive(Clone, Copy, Debug)]
pub struct GuestCallResult {
    pub r3: u32,
    pub r4: u32,
    pub f1: f64,
}

/// A location in the original section table of a linked REL module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelAddress {
    pub module: u32,
    pub section: u32,
    pub offset: u32,
}

/// One general-purpose argument to a guest call with a scratch buffer.
#[derive(Clone, Copy, Debug)]
pub enum GuestWord {
    Immediate(u32),
    /// A guest pointer to a byte within the buffer for this call.
    ScratchOffset(usize),
}

/// A CPU runtime and the memory used by its synchronous register calls.
pub trait GuestRuntime {
    fn memory(&mut self) -> &mut Mem;

    fn resolve_rel_address(&self, address: RelAddress, len: usize, executable: bool) -> Option<u32>;

    fn call_registers(
        &mut self,
        addr: u32,
        gprs: &[u32],
        fprs: &[f64],
    ) -> Result<GuestCallResult, String>;

    /// Calls with outgoing stack argument bytes and a temporary guest buffer.
    fn call_frame_with_scratch(
        &mut self,
        addr: u32,
        gprs: &[GuestWord],
        fprs: &[f64],
        stack: &[u8],
        scratch: &mut [u8],
    ) -> Result<GuestCallResult, String>;
}

pub const SPR_XER: usize = 1;
pub const SPR_LR: usize = 8;
pub const SPR_CTR: usize = 9;
pub const SPR_DEC: usize = 22;
pub const SPR_SRR0: usize = 26;
pub const SPR_SRR1: usize = 27;
pub const SPR_TBL_W: usize = 284;
pub const SPR_TBU_W: usize = 285;
/// GQR0..GQR7 (paired-single quantisation) are SPRs 912..919.
pub const SPR_GQR0: usize = 912;
pub const SPR_HID0: usize = 1008;
pub const SPR_HID2: usize = 920;
pub const SPR_HID4: usize = 1011;

/// MSR[EE]: external and decrementer interrupts enabled.
pub const MSR_EE: u32 = 0x8000;
/// MSR[FP]: floating point available. With it clear, a floating-point
/// instruction takes the FP-unavailable exception (0x800), which the OS
/// uses to switch FPU state between threads lazily.
pub const MSR_FP: u32 = 0x2000;

/// Whether `w` is a floating-point instruction (loads and stores included),
/// i.e. one that needs MSR[FP].
pub fn is_fp(w: u32) -> bool {
    match w >> 26 {
        4 => (w >> 1) & 0x3FF != 1014, // paired singles, not dcbz_l
        48..=63 => !matches!(w >> 26, 58 | 62),
        31 => matches!((w >> 1) & 0x3FF, 535 | 567 | 599 | 631 | 663 | 695 | 727 | 759 | 983),
        _ => false,
    }
}

/// Whether `w` is a branch (`b`, `bc`, `bclr`, `bcctr`, any form, taken or
/// not). Slices end only after one (`hle_os::SLICE`).
pub fn is_branch(w: u32) -> bool {
    match w >> 26 {
        16 | 18 => true,
        19 => matches!((w >> 1) & 0x3FF, 16 | 528),
        _ => false,
    }
}

pub const XER_SO: u32 = 0x8000_0000;
pub const XER_OV: u32 = 0x4000_0000;
pub const XER_CA: u32 = 0x2000_0000;

/// CPU clock 729 MHz; the timebase runs at bus/4 = 60.75 MHz, i.e. one tick
/// per 12 cycles. The interpreter counts one cycle per instruction.
pub const CYCLES_PER_TICK: u64 = 12;

/// Why execution stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// An instruction word this interpreter does not implement.
    Unimplemented { pc: u32, word: u32 },
    /// A load, store or fetch from an unmapped address.
    Fault { pc: u32, fault: Fault },
    /// `tw`/`twi` whose condition held (the OS uses these for fatal asserts).
    Trap { pc: u32 },
}

impl Stop {
    /// The instruction it stopped at.
    pub fn pc(&self) -> u32 {
        match *self {
            Stop::Unimplemented { pc, .. } | Stop::Fault { pc, .. } | Stop::Trap { pc } => pc,
        }
    }
}

#[derive(Clone)]
pub struct Cpu {
    pub pc: u32,
    pub gpr: [u32; 32],
    /// Paired-single view of the FPRs: `[ps0, ps1]`. Double-precision
    /// instructions use `ps0` only.
    pub fpr: [[f64; 2]; 32],
    pub cr: u32,
    pub xer: u32,
    pub lr: u32,
    pub ctr: u32,
    pub msr: u32,
    pub fpscr: u32,
    pub sr: [u32; 16],
    /// Every other SPR (SRR0/1, GQRs, HIDs, SPRGs, BATs, ...), indexed by number.
    pub spr: Vec<u32>,
    /// Instructions executed.
    pub cycles: u64,
    /// Address of the instruction being executed.
    pub(crate) cia: u32,
    tb_base: u64,
    dec_value: u32,
    dec_set_at: u64,
    /// The decrementer exception for the current DEC value was already raised.
    dec_signalled: bool,
    reservation: Option<u32>,
    /// Exceptions taken so far (interrupts, decrementer, FP unavailable,
    /// `sc`). Diagnostic only: it isn't in save states and never reaches the
    /// guest. The body checker uses it to tell a call that an exception
    /// interrupted from one the body got wrong.
    pub exceptions: u64,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            gpr: [0; 32],
            fpr: [[0.0; 2]; 32],
            cr: 0,
            xer: 0,
            lr: 0,
            ctr: 0,
            msr: 0,
            fpscr: 0,
            sr: [0; 16],
            spr: vec![0; 1024],
            cycles: 0,
            cia: 0,
            tb_base: 0,
            // Counting down from -1, as after reset: no exception until written.
            dec_value: 0xFFFF_FFFF,
            dec_set_at: 0,
            dec_signalled: false,
            reservation: None,
            exceptions: 0,
        }
    }

    pub fn timebase(&self) -> u64 {
        self.tb_base.wrapping_add(self.cycles / CYCLES_PER_TICK)
    }

    pub(crate) fn set_timebase(&mut self, tb: u64) {
        self.tb_base = tb.wrapping_sub(self.cycles / CYCLES_PER_TICK);
    }

    pub(crate) fn read_dec(&self) -> u32 {
        let elapsed = self.timebase().wrapping_sub(self.dec_set_at) as u32;
        self.dec_value.wrapping_sub(elapsed)
    }

    pub(crate) fn write_dec(&mut self, v: u32) {
        self.dec_value = v;
        self.dec_set_at = self.timebase();
        self.dec_signalled = false;
    }

    /// True once DEC has counted past zero (bit 0 went from 0 to 1) and the
    /// exception for it has not been taken yet.
    pub fn decrementer_pending(&self) -> bool {
        !self.dec_signalled && self.timebase().wrapping_sub(self.dec_set_at) > u64::from(self.dec_value) && self.dec_value & 0x8000_0000 == 0
    }

    /// Takes the decrementer exception (vector 0x900) if one is pending.
    pub fn take_decrementer(&mut self) -> bool {
        if !self.decrementer_pending() {
            return false;
        }
        self.dec_signalled = true;
        self.exception(0x900, self.pc);
        true
    }

    pub fn gqr(&self, i: usize) -> u32 {
        self.spr[SPR_GQR0 + i]
    }

    /// Bit `n` of CR, with bit 0 the most significant (CR0[LT]).
    pub fn cr_bit(&self, n: u32) -> bool {
        (self.cr >> (31 - n)) & 1 != 0
    }

    pub(crate) fn set_cr_bit(&mut self, n: u32, v: bool) {
        let m = 1 << (31 - n);
        self.cr = if v { self.cr | m } else { self.cr & !m };
    }

    /// CR field `f` (0..7) as a nibble: LT 8, GT 4, EQ 2, SO 1.
    pub fn cr_field(&self, f: u32) -> u32 {
        (self.cr >> (28 - 4 * f)) & 0xF
    }

    pub(crate) fn set_cr_field(&mut self, f: u32, v: u32) {
        let s = 28 - 4 * f;
        self.cr = (self.cr & !(0xF << s)) | ((v & 0xF) << s);
    }

    pub(crate) fn ca(&self) -> u32 {
        (self.xer >> 29) & 1
    }

    pub(crate) fn set_ca(&mut self, c: bool) {
        self.xer = if c { self.xer | XER_CA } else { self.xer & !XER_CA };
    }

    /// Sets XER[OV] and, when overflowing, the sticky XER[SO].
    pub(crate) fn set_ov(&mut self, o: bool) {
        if o {
            self.xer |= XER_OV | XER_SO;
        } else {
            self.xer &= !XER_OV;
        }
    }

    /// CR0 from a signed comparison of `v` with zero, plus XER[SO].
    pub(crate) fn update_cr0(&mut self, v: u32) {
        let c = match (v as i32).cmp(&0) {
            std::cmp::Ordering::Less => 8,
            std::cmp::Ordering::Greater => 4,
            std::cmp::Ordering::Equal => 2,
        };
        self.set_cr_field(0, c | (self.xer >> 31));
    }

    /// Executes one instruction. On error the PC still points at it.
    pub fn step(&mut self, mem: &mut Mem) -> Result<(), Stop> {
        let pc = self.pc;
        let word = mem.read_u32(pc).map_err(|fault| Stop::Fault { pc, fault })?;
        self.cia = pc;
        self.pc = pc.wrapping_add(4);
        self.cycles += 1;
        if self.msr & MSR_FP == 0 && is_fp(word) {
            self.exception(0x800, pc);
            return Ok(());
        }
        let r = interp::execute(self, mem, word);
        if r.is_err() {
            self.pc = pc;
        }
        r
    }

    /// Takes an exception: SRR0 = `srr0`, SRR1 = MSR, MSR enters supervisor
    /// real mode, PC = `vector` (physical; exception code runs untranslated).
    pub fn exception(&mut self, vector: u32, srr0: u32) {
        self.exceptions += 1;
        self.spr[SPR_SRR0] = srr0;
        self.spr[SPR_SRR1] = self.msr & 0x87C0_FFFF;
        self.msr &= !0x0004_EF36;
        self.pc = vector;
    }
}

/// Every register and the timers; `spr` is saved whole.
impl guest_mem::snap::Snapshot for Cpu {
    fn snap(&mut self, s: &mut guest_mem::snap::Snap) {
        s.tag(b"CPU ");
        guest_mem::snap!(s; self.pc, self.gpr, self.fpr, self.cr, self.xer, self.lr, self.ctr, self.msr, self.fpscr, self.sr, self.spr, self.cycles, self.cia, self.tb_base, self.dec_value, self.dec_set_at, self.dec_signalled, self.reservation);
    }
}
