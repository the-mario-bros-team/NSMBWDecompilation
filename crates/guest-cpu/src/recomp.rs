//! Support for recompiled guest code (docs/PORT_PLAN.md, P6).
//!
//! `ppc-recomp` turns each guest function into one Rust function,
//! `fn(&mut Cpu, &mut Mem, &mut Rt) -> R`, entered at `cpu.pc`: any of its
//! basic blocks can be the entry, so execution can resume anywhere a block
//! starts (after a call, an interrupt, or a thread switch). Every guest
//! instruction runs through the interpreter's own handler for it
//! ([`crate::fast`], [`crate::interp`]); only control flow is native.
//!
//! The contract with the caller:
//! - Guest state lives in [`Cpu`] and [`Mem`] only. A recompiled function may
//!   return at any block start with `cpu.pc` there and nothing lost, so
//!   whoever called it simply carries on at `cpu.pc`.
//! - A direct call (`bl`) is a Rust call; afterwards the caller continues
//!   only if `cpu.pc` is its return address, and otherwise returns too.
//!   Leaving through a thread switch, a hook, a jump the function does not
//!   know, or the budget therefore unwinds to the dispatcher
//!   (`hle_os::System::run_field`), which looks `cpu.pc` up again.
//! - After every branch, the code checks the slice budget ([`Rt::end`], in
//!   cycles) with [`slice`], as the interpreter does after its blocks, so
//!   both end slices at the same instructions. Ending one (hardware,
//!   interrupts) happens in place; the code unwinds only if an interrupt was
//!   taken or the field is over.
//! - A function runs only while its code in guest memory is still the code
//!   it was generated from ([`Rt::enter`]).

use guest_mem::Mem;

use crate::{Cpu, Stop};

pub type R = Result<(), Stop>;

/// A recompiled function. On x86-64 it uses the `sysv64` calling convention
/// whatever the host ([`GUEST_FN_ABI`], which `ppc-recomp` writes into each
/// definition): it has no callee-saved vector registers, so a call does not
/// save and restore xmm6..xmm15 as the Windows convention makes every
/// function that touches them do (most do: LLVM keeps constants there), it
/// saves two general registers fewer, and the 16-byte result comes back in
/// two registers instead of through memory.
/// (Only Rust calls these, so the types need not be FFI-safe.)
#[cfg(target_arch = "x86_64")]
#[allow(improper_ctypes_definitions)]
pub type GuestFn = extern "sysv64" fn(&mut Cpu, &mut Mem, &mut Rt) -> R;
#[cfg(not(target_arch = "x86_64"))]
pub type GuestFn = fn(&mut Cpu, &mut Mem, &mut Rt) -> R;

/// [`GuestFn`]'s calling convention as `ppc-recomp` writes it.
#[cfg(target_arch = "x86_64")]
pub const GUEST_FN_ABI: &str = "extern \"sysv64\" ";
#[cfg(not(target_arch = "x86_64"))]
pub const GUEST_FN_ABI: &str = "";

/// One recompiled guest function.
pub struct FnInfo {
    /// Its first instruction's address.
    pub start: u32,
    /// The instruction words it was generated from (`start` onwards).
    pub words: &'static [u32],
    /// Addresses where it can be entered (its block starts).
    pub entries: &'static [u32],
    pub f: GuestFn,
}

/// Ends a slice (see [`Rt::set_slice_end`]).
pub type SliceEnd = Box<dyn FnMut(&mut Cpu, &mut Mem) -> Option<u64>>;

/// Calls deeper than this unwind to the dispatcher (bounds the host stack).
const MAX_DEPTH: u32 = 400;
/// Entries of [`Rt`]'s lookup cache (a power of two).
const CACHE: usize = 1 << 13;
/// `checked` value for a function never checked.
const UNCHECKED: u32 = u32::MAX;

pub struct Rt {
    /// Budget: blocks may run while `cpu.cycles` stays at or below this.
    pub end: u64,
    pub depth: u32,
    fns: &'static [FnInfo],
    /// Per function: the code epoch at which it was last found intact.
    checked: Vec<u32>,
    /// Per function: whether it was intact then, and the sum of its pages'
    /// write generations at that check.
    intact: Vec<bool>,
    gen_sum: Vec<u32>,
    pages: Vec<Option<(u32, u32)>>,
    /// Entry address -> function index + 1, per 4 KiB of MEM1.
    table: Vec<Option<Box<[u32; 1024]>>>,
    /// Recent lookups (address, function), direct-mapped by address: a hit
    /// is one load from a small table instead of two from large ones (the
    /// page of `table`, then `fns`), which miss the cache. The dispatcher
    /// and indirect calls (`bctrl`, virtual functions) look up all the time.
    cache: Box<[(u32, Option<GuestFn>)]>,
    /// Functions found changed in memory (not run until they match again).
    pub changed: u64,
    slice_end: Option<SliceEnd>,
}

const TABLE_BASE: u32 = 0x8000_0000;
const TABLE_PAGES: usize = guest_mem::MEM1_SIZE >> 12;

impl Rt {
    /// `special`: addresses the dispatcher must see before anything runs
    /// there (hooks, the idle loop), which the code was generated with
    /// (`generated_special`); if they differ, the code is stale and nothing
    /// is registered.
    pub fn new(fns: &'static [FnInfo], mem: &mut Mem, special: &[u32], generated_special: &[u32]) -> Result<Self, String> {
        let (mut a, mut b) = (special.to_vec(), generated_special.to_vec());
        a.sort_unstable();
        b.sort_unstable();
        if !fns.is_empty() && a != b {
            return Err("the recompiled code was generated for other hooks; run ppc-recomp again".into());
        }
        let mut table: Vec<Option<Box<[u32; 1024]>>> = (0..TABLE_PAGES).map(|_| None).collect();
        let mut pages = Vec::with_capacity(fns.len());
        for (i, f) in fns.iter().enumerate() {
            pages.push(mem.mark_code(f.start, 4 * f.words.len()));
            for &e in f.entries {
                // Only the dispatcher runs hooks; an indirect call or jump
                // to one must reach it.
                if special.contains(&e) {
                    continue;
                }
                let Some(page) = e.checked_sub(TABLE_BASE).map(|o| (o >> 12) as usize).filter(|&p| p < TABLE_PAGES) else { continue };
                table[page].get_or_insert_with(|| Box::new([0; 1024]))[((e >> 2) & 1023) as usize] = i as u32 + 1;
            }
        }
        // (0, None) is right for address 0 (no function), the only one that
        // could match an untouched entry: it maps to entry 0.
        let cache = vec![(0, None); CACHE].into_boxed_slice();
        Ok(Self { end: 0, depth: 0, fns, checked: vec![UNCHECKED; fns.len()], intact: vec![false; fns.len()], gen_sum: vec![0; fns.len()], pages, table, cache, changed: 0, slice_end: None })
    }

    /// What ends a slice: advance the hardware and deliver interrupts, then
    /// `Some(next end)` to carry on, or `None` to return to the dispatcher.
    pub fn set_slice_end(&mut self, f: SliceEnd) {
        self.slice_end = Some(f);
    }

    #[cold]
    #[inline(never)]
    fn end_slice(&mut self, c: &mut Cpu, m: &mut Mem) -> bool {
        match self.slice_end.as_mut().and_then(|f| f(c, m)) {
            Some(end) => {
                self.end = end;
                true
            }
            None => false,
        }
    }

    pub fn functions(&self) -> usize {
        self.fns.len()
    }

    /// Function `idx` (calls between generated crates go by index).
    #[inline(always)]
    pub fn func(&self, idx: usize) -> GuestFn {
        self.fns[idx].f
    }

    /// The function with an entry at `pc`.
    #[inline(always)]
    pub fn lookup(&mut self, pc: u32) -> Option<GuestFn> {
        let (at, f) = self.cache[(pc >> 2) as usize & (CACHE - 1)];
        if at == pc {
            return f;
        }
        self.lookup_miss(pc)
    }

    #[inline(never)]
    fn lookup_miss(&mut self, pc: u32) -> Option<GuestFn> {
        let f = self.find(pc);
        self.cache[(pc >> 2) as usize & (CACHE - 1)] = (pc, f);
        f
    }

    fn find(&self, pc: u32) -> Option<GuestFn> {
        let page = (pc.wrapping_sub(TABLE_BASE) >> 12) as usize;
        let i = self.table.get(page)?.as_ref()?[((pc >> 2) & 1023) as usize];
        (i != 0).then(|| self.fns[i as usize - 1].f)
    }

    /// Function `idx`'s prologue: true if it may run (its code is intact
    /// and the host stack has room).
    #[inline(always)]
    pub fn enter(&mut self, m: &Mem, idx: usize) -> bool {
        (self.checked[idx] == m.code_epoch() && self.depth < MAX_DEPTH) || self.enter_slow(m, idx)
    }

    #[cold]
    #[inline(never)]
    fn enter_slow(&mut self, m: &Mem, idx: usize) -> bool {
        if self.depth >= MAX_DEPTH {
            return false;
        }
        let epoch = m.code_epoch();
        if self.checked[idx] == epoch {
            return self.intact[idx];
        }
        let Some((a, b)) = self.pages[idx] else { return false };
        let sum = (a..=b).fold(0u32, |s, p| s.wrapping_add(m.code_gen(p)));
        if self.checked[idx] == UNCHECKED || sum != self.gen_sum[idx] {
            let f = &self.fns[idx];
            let now = m.ram(f.start, 4 * f.words.len());
            let same = now.is_some_and(|b| b.chunks_exact(4).zip(f.words).all(|(c, &w)| u32::from_be_bytes([c[0], c[1], c[2], c[3]]) == w));
            if !same && self.intact[idx] {
                self.changed += 1;
            }
            self.intact[idx] = same;
            self.gen_sum[idx] = sum;
        }
        // An intact function is marked checked at this epoch, so its next
        // calls take the fast path; a changed one is checked again each time.
        self.checked[idx] = if self.intact[idx] { epoch } else { UNCHECKED };
        self.intact[idx]
    }
}

/// After a branch to `next`: `pc` = `next`, and the slice ends here if its
/// budget is spent. False if the code must return to the dispatcher.
#[inline(always)]
pub fn slice(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, next: u32) -> bool {
    c.pc = next;
    c.cycles < rt.end || rt.end_slice(c, m)
}

/// A direct call (`bl`): `lr` = `ret`, run `f` from `target`; true if it
/// came back to `ret` (the caller carries on), false to unwind.
#[inline(always)]
pub fn call(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, f: GuestFn, target: u32, ret: u32) -> Result<bool, Stop> {
    c.lr = ret;
    c.pc = target;
    rt.depth += 1;
    let r = f(c, m, rt);
    rt.depth -= 1;
    r?;
    Ok(c.pc == ret)
}

/// A direct call into function `idx` of another generated crate.
#[inline(always)]
pub fn call_index(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, idx: usize, target: u32, ret: u32) -> Result<bool, Stop> {
    let f = rt.func(idx);
    call(c, m, rt, f, target, ret)
}

/// [`tail`] into function `idx` of another generated crate.
#[inline(always)]
pub fn tail_index(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, idx: usize, target: u32) -> R {
    let f = rt.func(idx);
    tail(c, m, rt, f, target)
}

/// An indirect call (`bctrl`, `blrl`) to `target`.
#[inline(always)]
pub fn call_indirect(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, target: u32, ret: u32) -> Result<bool, Stop> {
    match rt.lookup(target) {
        Some(f) => call(c, m, rt, f, target, ret),
        None => {
            c.lr = ret;
            c.pc = target;
            Ok(false)
        }
    }
}

/// A computed jump (`bctr`) out of the function: run the function with an
/// entry at `target` and return as it does, or return to the dispatcher.
#[inline(always)]
pub fn tail_indirect(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, target: u32) -> R {
    match rt.lookup(target) {
        Some(f) => tail(c, m, rt, f, target),
        None => {
            c.pc = target;
            Ok(())
        }
    }
}

/// A jump (`b`) into another function: run it, then return to our caller.
#[inline(always)]
pub fn tail(c: &mut Cpu, m: &mut Mem, rt: &mut Rt, f: GuestFn, target: u32) -> R {
    c.pc = target;
    rt.depth += 1;
    let r = f(c, m, rt);
    rt.depth -= 1;
    r
}

/// A handler's result, with an error moved to `pc` (handlers report the
/// interpreter's current instruction, which recompiled code does not keep).
/// The dispatcher sets `cpu.pc` from the error ([`Stop::pc`]); leaving `cpu`
/// out of this path lets the compiler keep guest registers in host
/// registers across loads and stores.
#[inline(always)]
pub fn at(r: R, pc: u32) -> R {
    match r {
        Ok(()) => Ok(()),
        Err(e) => Err(failed(e, pc)),
    }
}

#[cold]
#[inline(never)]
fn failed(e: Stop, pc: u32) -> Stop {
    match e {
        Stop::Unimplemented { word, .. } => Stop::Unimplemented { pc, word },
        Stop::Fault { fault, .. } => Stop::Fault { pc, fault },
        Stop::Trap { .. } => Stop::Trap { pc },
    }
}
