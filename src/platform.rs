//! Wii / Broadway low-level access helpers.
//!
//! The target is big-endian PowerPC; byte-wise volatile access keeps host
//! `cargo check` semantics independent of the host endianness.
//!
//! These are host-side helpers, not recovered functions. They model the
//! primitive loads, stores and indirect calls used by the reconstruction.
//!
//! The `guest-memory` feature routes loads, stores, and indirect calls through
//! `with_guest_runtime`. A `with_guest_memory` scope supports memory access alone.
//! The legacy helpers panic on missing scopes, invalid addresses, and guest faults.

/// Broadway's single-precision multiply (`fmuls`, `ps_mul`): the multiplier
/// (frC) keeps only 25 significant bits, the product is rounded to single.
/// Matches `guest_cpu::fp` (`force25`, `round_single`), which is the oracle.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Confidence
/// high - same arithmetic as the interpreter's `fp_single`.
#[inline]
pub fn fmuls(a: f64, c: f64) -> f32 {
    let c = if c.is_finite() {
        let b = c.to_bits();
        f64::from_bits((b & 0xFFFF_FFFF_F800_0000).wrapping_add(b & 0x0800_0000))
    } else {
        c
    };
    (a * c) as f32
}

/// Broadway's single-precision add (`fadds`): the double sum, rounded once
/// to single, as the interpreter does it.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Confidence
/// high - same arithmetic as the interpreter's `fp_single`.
#[inline]
pub fn fadds(a: f64, b: f64) -> f32 {
    (a + b) as f32
}

/// Broadway's single-precision subtract (`fsubs`): the double difference,
/// rounded once to single, as the interpreter does it.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Confidence
/// high - same arithmetic as the interpreter's `fp_single`.
#[inline]
pub fn fsubs(a: f64, b: f64) -> f32 {
    (a - b) as f32
}

#[cfg(feature = "guest-memory")]
extern crate std;

#[cfg(feature = "guest-memory")]
use core::{cell::RefCell, ptr::NonNull};
#[cfg(feature = "guest-memory")]
use guest_cpu::{GuestCallResult, GuestRuntime, GuestWord, RelAddress};
#[cfg(feature = "guest-memory")]
use guest_mem::{Fault, Mem};
#[cfg(feature = "guest-memory")]
use std::string::String;

#[cfg(feature = "guest-memory")]
enum GuestContext {
    Memory(NonNull<Mem>),
    Runtime(NonNull<dyn GuestRuntime>),
}

#[cfg(feature = "guest-memory")]
std::thread_local! {
    static GUEST_CONTEXT: RefCell<Option<GuestContext>> = const { RefCell::new(None) };
    /// Guest accesses since the runtime scope began (see [`RUNAWAY_LIMIT`]).
    static ACCESSES: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// A body that makes more guest accesses than this in one call is stuck in a
/// loop (a wrong exit test): it panics, so the checker fails the call instead
/// of hanging. The longest checked bodies make a few thousand.
#[cfg(feature = "guest-memory")]
const RUNAWAY_LIMIT: u64 = 20_000_000;

#[cfg(feature = "guest-memory")]
#[inline]
fn count_access() {
    let n = ACCESSES.with(|a| {
        let n = a.get() + 1;
        a.set(n);
        n
    });
    assert!(n <= RUNAWAY_LIMIT, "Rust body ran away: more than {RUNAWAY_LIMIT} guest accesses in one call");
}

/// Runs a synchronous call with `mem` backing the platform memory helpers.
///
/// The previous context is restored when the call returns or unwinds. A memory
/// operation holds an exclusive borrow through MMIO, so MMIO callbacks cannot
/// reenter these helpers.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - scopes the port's existing guest memory.
#[cfg(feature = "guest-memory")]
pub fn with_guest_memory<T>(mem: &mut Mem, f: impl FnOnce() -> T) -> T {
    with_guest_context(GuestContext::Memory(NonNull::from(mem)), f)
}

/// Runs a synchronous call with the runtime backing platform memory and calls.
///
/// Install this scope between guest fields. A guest call or MMIO operation
/// borrows the runtime until it returns, preventing reentry into this scope.
/// The previous scope is restored on return or unwind.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - connects the platform helpers to the port's guest runtime.
#[cfg(feature = "guest-memory")]
pub fn with_guest_runtime<T>(
    runtime: &mut (dyn GuestRuntime + 'static),
    f: impl FnOnce() -> T,
) -> T {
    ACCESSES.with(|a| a.set(0));
    with_guest_context(GuestContext::Runtime(NonNull::from(runtime)), f)
}

#[cfg(feature = "guest-memory")]
fn with_guest_context<T>(context: GuestContext, f: impl FnOnce() -> T) -> T {
    struct Restore<'a> {
        slot: &'a RefCell<Option<GuestContext>>,
        previous: Option<GuestContext>,
    }

    impl Drop for Restore<'_> {
        fn drop(&mut self) {
            self.slot.replace(self.previous.take());
        }
    }

    GUEST_CONTEXT.with(|slot| {
        let _restore = Restore {
            slot,
            previous: slot.replace(Some(context)),
        };
        f()
    })
}

#[cfg(feature = "guest-memory")]
#[inline]
fn access_guest_memory<T>(
    addr: usize,
    access: impl FnOnce(&mut Mem, u32) -> Result<T, Fault>,
) -> T {
    let addr = u32::try_from(addr).expect("guest address is wider than 32 bits");
    count_access();
    flush_guest_locals();
    GUEST_CONTEXT.with(|slot| {
        let mut active = slot.borrow_mut();
        let context = active.as_mut().expect("guest memory is not in scope");
        // SAFETY: the scope keeps the original exclusive reference alive.
        // This borrow prevents reentry through MMIO or the runtime.
        let mem = unsafe {
            match context {
                GuestContext::Memory(mem) => mem.as_mut(),
                GuestContext::Runtime(runtime) => runtime.as_mut().memory(),
            }
        };
        access(mem, addr).unwrap_or_else(|fault| panic!("guest memory fault: {fault:?}"))
    })
}

/// Calls a guest address with r3–r10 and f1–f8 register arguments.
///
/// Callers lay out any register pairs and pass single-precision inputs as
/// widened scalar values. Stack arguments and varargs are unsupported.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - forwards register arguments and returns to the port.
#[cfg(feature = "guest-memory")]
pub fn call_registers(addr: usize, gprs: &[u32], fprs: &[f64]) -> Result<GuestCallResult, String> {
    let addr = u32::try_from(addr).map_err(|_| "guest address is wider than 32 bits")?;
    access_guest_runtime(|runtime| runtime.call_registers(addr, gprs, fprs))
}

/// Calls a guest function using a temporary buffer for pointer arguments.
///
/// Offsets refer to bytes in `scratch`. The caller encodes values in guest
/// byte order and supplies enough space for each pointed-to value. On
/// success the guest's output replaces the buffer contents.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - uses the port's bounded scratch buffer and fixed register ABI.
#[cfg(feature = "guest-memory")]
pub fn call_registers_with_scratch(
    addr: usize,
    gprs: &[GuestWord],
    fprs: &[f64],
    scratch: &mut [u8],
) -> Result<GuestCallResult, String> {
    call_frame_with_scratch(addr, gprs, fprs, &[], scratch)
}

/// Calls a guest function with explicit outgoing stack argument bytes.
///
/// Encode stack arguments in guest byte order, starting at `r1 + 8`, and
/// include ABI padding between arguments. The runtime aligns the frame and
/// preserves the outgoing argument area when the call returns.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - follows the fixed-prototype PowerPC EABI stack layout.
#[cfg(feature = "guest-memory")]
pub fn call_frame_with_scratch(
    addr: usize,
    gprs: &[GuestWord],
    fprs: &[f64],
    stack: &[u8],
    scratch: &mut [u8],
) -> Result<GuestCallResult, String> {
    let addr = u32::try_from(addr).map_err(|_| "guest address is wider than 32 bits")?;
    access_guest_runtime(|runtime| {
        runtime.call_frame_with_scratch(addr, gprs, fprs, stack, scratch)
    })
}

/// Looks up a byte range in a linked REL section using the active runtime.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - uses the port's live module list.
#[cfg(feature = "guest-memory")]
pub fn resolve_rel_address(
    address: RelAddress,
    len: usize,
    executable: bool,
) -> Result<Option<u32>, String> {
    access_guest_runtime(|runtime| Ok(runtime.resolve_rel_address(address, len, executable)))
}

/// Calls an executable REL location with the same ABI as [`call_registers`].
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - resolves and dispatches within the same runtime borrow.
#[cfg(feature = "guest-memory")]
pub fn call_rel_registers(
    address: RelAddress,
    gprs: &[u32],
    fprs: &[f64],
) -> Result<GuestCallResult, String> {
    access_guest_runtime(|runtime| {
        let addr = runtime
            .resolve_rel_address(address, 4, true)
            .ok_or("REL entry is unavailable or not executable")?;
        runtime.call_registers(addr, gprs, fprs)
    })
}

#[cfg(feature = "guest-memory")]
fn access_guest_runtime<T>(
    run: impl FnOnce(&mut dyn GuestRuntime) -> Result<T, String>,
) -> Result<T, String> {
    count_access();
    flush_guest_locals();
    let result = GUEST_CONTEXT.with(|slot| {
        let mut active = slot
            .try_borrow_mut()
            .map_err(|_| "guest runtime is already in use")?;
        let Some(GuestContext::Runtime(runtime)) = active.as_mut() else {
            return Err("guest runtime is not in scope".into());
        };
        // SAFETY: the scope owns the exclusive reference and the slot
        // remains borrowed until the operation returns.
        run(unsafe { runtime.as_mut() })
    });
    // The guest ran: any local it can see may have changed.
    bump_guest_epoch();
    result
}

#[cfg(feature = "guest-memory")]
fn call_integer_registers(addr: usize, gprs: &[u32]) -> GuestCallResult {
    call_registers(addr, gprs, &[]).expect("guest call failed")
}

/// Reads one byte from a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models the PowerPC byte load used by
/// the reconstructed functions.
///
/// # Safety
/// `addr` must be readable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn read_u8(addr: usize) -> u8 {
    #[cfg(feature = "guest-memory")]
    {
        access_guest_memory(addr, Mem::read_u8)
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        core::ptr::read_volatile(addr as *const u8)
    }
}

/// Reads a big-endian `u16` from a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC halfword load.
///
/// # Safety
/// `addr..addr+2` must be readable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn read_u16(addr: usize) -> u16 {
    #[cfg(feature = "guest-memory")]
    {
        access_guest_memory(addr, Mem::read_u16)
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        u16::from_be_bytes([read_u8(addr), read_u8(addr + 1)])
    }
}

/// Reads a big-endian `u32` from a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC word load.
///
/// # Safety
/// `addr..addr+4` must be readable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn read_u32(addr: usize) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        access_guest_memory(addr, Mem::read_u32)
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        u32::from_be_bytes([
            read_u8(addr),
            read_u8(addr + 1),
            read_u8(addr + 2),
            read_u8(addr + 3),
        ])
    }
}

/// Reads a big-endian `i32` from a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC signed word load.
///
/// # Safety
/// `addr..addr+4` must be readable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn read_i32(addr: usize) -> i32 {
    read_u32(addr) as i32
}

/// Reads a 32-bit IEEE-754 float from a virtual address (big-endian bits).
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC single-precision load.
///
/// # Safety
/// `addr..addr+4` must be readable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn read_f32(addr: usize) -> f32 {
    f32::from_bits(read_u32(addr))
}

/// Writes a 32-bit IEEE-754 float to a virtual address (big-endian bits).
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC single-precision store.
///
/// # Safety
/// `addr..addr+4` must be writable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn write_f32(addr: usize, value: f32) {
    write_u32(addr, value.to_bits());
}

/// Writes one byte to a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC byte store.
///
/// # Safety
/// `addr` must be writable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn write_u8(addr: usize, value: u8) {
    #[cfg(feature = "guest-memory")]
    {
        access_guest_memory(addr, |mem, addr| mem.write_u8(addr, value));
        bump_guest_epoch();
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        core::ptr::write_volatile(addr as *mut u8, value);
    }
}

/// Writes a big-endian `u16` to a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC halfword store.
///
/// # Safety
/// `addr..addr+2` must be writable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn write_u16(addr: usize, value: u16) {
    #[cfg(feature = "guest-memory")]
    {
        access_guest_memory(addr, |mem, addr| mem.write_u16(addr, value));
        bump_guest_epoch();
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let b = value.to_be_bytes();
        for (i, byte) in b.iter().enumerate() {
            write_u8(addr + i, *byte);
        }
    }
}

/// Writes a big-endian `u32` to a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models a PowerPC word store.
///
/// # Safety
/// `addr..addr+4` must be writable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn write_u32(addr: usize, value: u32) {
    #[cfg(feature = "guest-memory")]
    {
        access_guest_memory(addr, |mem, addr| mem.write_u32(addr, value));
        bump_guest_epoch();
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let b = value.to_be_bytes();
        for (i, byte) in b.iter().enumerate() {
            write_u8(addr + i, *byte);
        }
    }
}

/// Writes a signed 32-bit big-endian value.
///
/// # Original
/// Host-side helper; no binary address. Models the PowerPC `stw` store used by
/// the reconstructed code.
///
/// # Safety
/// `addr..addr+4` must be writable in the target's address space.
///
/// # Confidence
/// high - reuses [`write_u32`].
#[inline]
pub unsafe fn write_i32(addr: usize, value: i32) {
    write_u32(addr, value as u32);
}

/// Calls a PowerPC function by absolute address with no arguments.
///
/// # Original
/// Host-side helper; no binary address. Models the PowerPC EABI indirect call
/// used throughout the reconstruction.
///
/// # Safety
/// `addr` must point to a function with this ABI. The direct call path
/// requires a PowerPC target or an ABI-compatible host.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call0(addr: usize) {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[]);
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn() = core::mem::transmute(addr);
        f()
    }
}

/// Calls a PowerPC function with one integer argument.
///
/// # Original
/// Host-side helper; no binary address. The argument is passed in `r3`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call1(addr: usize, a: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn(u32) -> u32 = core::mem::transmute(addr);
        f(a)
    }
}

/// Calls a PowerPC function with two integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in `r3`/`r4`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call2(addr: usize, a: u32, b: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a, b]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn(u32, u32) -> u32 = core::mem::transmute(addr);
        f(a, b)
    }
}

/// Calls a PowerPC function with three integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in `r3`/`r4`/`r5`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call3(addr: usize, a: u32, b: u32, c: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a, b, c]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn(u32, u32, u32) -> u32 = core::mem::transmute(addr);
        f(a, b, c)
    }
}

/// Calls a PowerPC function with four integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in
/// `r3`/`r4`/`r5`/`r6`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call4(addr: usize, a: u32, b: u32, c: u32, d: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a, b, c, d]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
        f(a, b, c, d)
    }
}

/// Calls a PowerPC function with five integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in
/// `r3`..`r7`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call5(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a, b, c, d, e]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn(u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
        f(a, b, c, d, e)
    }
}

/// Calls a PowerPC function with six integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in
/// `r3`..`r8`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call6(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a, b, c, d, e, f]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let func: extern "C" fn(u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
        func(a, b, c, d, e, f)
    }
}

/// Calls a PowerPC function with seven integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in
/// `r3`..`r9`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call7(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    #[cfg(feature = "guest-memory")]
    {
        call_integer_registers(addr, &[a, b, c, d, e, f, g]).r3
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let func: extern "C" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(addr);
        func(a, b, c, d, e, f, g)
    }
}

/// Calls a PowerPC function that returns a 64-bit value in `r3:r4`.
///
/// # Original
/// Host-side helper; no binary address. Used for `OSGetTime`-style functions.
///
/// # Safety
/// `addr` must point to a function returning `u64` with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call_u64(addr: usize) -> u64 {
    #[cfg(feature = "guest-memory")]
    {
        let result = call_integer_registers(addr, &[]);
        (u64::from(result.r3) << 32) | u64::from(result.r4)
    }
    #[cfg(not(feature = "guest-memory"))]
    {
        let f: extern "C" fn() -> u64 = core::mem::transmute(addr);
        f()
    }
}

// ---------------------------------------------------------------------------
// Locals whose address the game sees
// ---------------------------------------------------------------------------
//
// The original functions keep these on the guest stack and pass their
// address to other guest code. The transcription made them Rust locals, whose
// address means nothing to the guest. A `GuestLocal` gives such a local a
// guest address and keeps the two copies in step.

/// Where `GuestLocal`s live with `guest-memory`: 1 MiB of MEM2 above the
/// game's memory (which ends at 0x93400000), clear of `System::call_guest`'s
/// stack and the portrait at 0x93420000. The body checker ignores it, as it
/// ignores the stacks. Keep in step with `GUEST_LOCALS` in `hle-os`.
pub const GUEST_LOCALS: (u32, u32) = (0x9380_0000, 0x9390_0000);

/// A value that can be stored in guest memory in the guest's byte order.
///
/// `Elem` is what `as_ptr` points to: the element for an array, the value
/// itself for a scalar, so pointer arithmetic keeps its meaning.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - big-endian scalars, arrays element by element.
pub trait GuestPod: Copy + 'static {
    type Elem;
    /// Bytes the value takes in guest memory.
    const SIZE: usize;
    /// # Safety
    /// `addr..addr + SIZE` must be writable guest memory.
    unsafe fn store(&self, addr: usize);
    /// # Safety
    /// `addr..addr + SIZE` must be readable guest memory.
    unsafe fn load(&mut self, addr: usize);
}

macro_rules! guest_pod_scalar {
    ($($t:ty: $size:expr, |$s:ident, $a:ident| $store:expr, |$b:ident| $load:expr;)*) => {$(
        impl GuestPod for $t {
            type Elem = $t;
            const SIZE: usize = $size;
            #[inline]
            unsafe fn store(&self, $a: usize) {
                let $s = *self;
                $store
            }
            #[inline]
            unsafe fn load(&mut self, $b: usize) {
                *self = $load;
            }
        }
    )*};
}

guest_pod_scalar! {
    u8: 1, |v, a| write_u8(a, v), |a| read_u8(a);
    i8: 1, |v, a| write_u8(a, v as u8), |a| read_u8(a) as i8;
    bool: 1, |v, a| write_u8(a, u8::from(v)), |a| read_u8(a) != 0;
    u16: 2, |v, a| write_u16(a, v), |a| read_u16(a);
    i16: 2, |v, a| write_u16(a, v as u16), |a| read_u16(a) as i16;
    u32: 4, |v, a| write_u32(a, v), |a| read_u32(a);
    i32: 4, |v, a| write_u32(a, v as u32), |a| read_u32(a) as i32;
    // Guest words: the transcription keeps addresses in `usize`.
    usize: 4, |v, a| write_u32(a, v as u32), |a| read_u32(a) as usize;
    isize: 4, |v, a| write_u32(a, v as u32), |a| read_u32(a) as i32 as isize;
    f32: 4, |v, a| write_u32(a, v.to_bits()), |a| f32::from_bits(read_u32(a));
    u64: 8, |v, a| { write_u32(a, (v >> 32) as u32); write_u32(a + 4, v as u32) },
        |a| (u64::from(read_u32(a)) << 32) | u64::from(read_u32(a + 4));
    i64: 8, |v, a| { write_u32(a, (v >> 32) as u32); write_u32(a + 4, v as u32) },
        |a| ((u64::from(read_u32(a)) << 32) | u64::from(read_u32(a + 4))) as i64;
    f64: 8, |v, a| { let b = v.to_bits(); write_u32(a, (b >> 32) as u32); write_u32(a + 4, b as u32) },
        |a| f64::from_bits((u64::from(read_u32(a)) << 32) | u64::from(read_u32(a + 4)));
}

impl<E: GuestPod, const N: usize> GuestPod for [E; N] {
    type Elem = E;
    const SIZE: usize = E::SIZE * N;
    #[inline]
    unsafe fn store(&self, addr: usize) {
        for (i, e) in self.iter().enumerate() {
            e.store(addr + i * E::SIZE);
        }
    }
    #[inline]
    unsafe fn load(&mut self, addr: usize) {
        for (i, e) in self.iter_mut().enumerate() {
            e.load(addr + i * E::SIZE);
        }
    }
}

#[cfg(feature = "guest-memory")]
const LOCAL_HOST: u8 = 0; // the Rust copy is current
#[cfg(feature = "guest-memory")]
const LOCAL_GUEST: u8 = 1; // the guest copy is current (the Rust one may lag)

#[cfg(feature = "guest-memory")]
trait DirtyLocal {
    /// # Safety
    /// The local must still be alive and where it was registered.
    unsafe fn flush(&self);
}

#[cfg(feature = "guest-memory")]
struct LocalArena {
    top: u32,
    live: u32,
}

#[cfg(feature = "guest-memory")]
std::thread_local! {
    /// Bumped whenever the guest may have written memory (a guest call or a
    /// platform store), so a `GuestLocal` re-reads only when it must.
    static GUEST_EPOCH: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Locals the Rust code changed after handing out their address: they
    /// are written back before the guest next sees memory.
    static DIRTY_LOCALS: RefCell<std::vec::Vec<(*const (), *const dyn DirtyLocal)>> =
        const { RefCell::new(std::vec::Vec::new()) };
    static LOCAL_ARENA: RefCell<LocalArena> =
        const { RefCell::new(LocalArena { top: GUEST_LOCALS.0, live: 0 }) };
}

#[cfg(feature = "guest-memory")]
fn bump_guest_epoch() {
    GUEST_EPOCH.with(|e| e.set(e.get().wrapping_add(1)));
}

#[cfg(feature = "guest-memory")]
fn guest_epoch() -> u64 {
    GUEST_EPOCH.with(|e| e.get())
}

#[cfg(feature = "guest-memory")]
#[inline]
fn flush_guest_locals() {
    let dirty = DIRTY_LOCALS.with(|d| {
        let mut d = d.borrow_mut();
        if d.is_empty() {
            None
        } else {
            Some(core::mem::take(&mut *d))
        }
    });
    for (_, local) in dirty.into_iter().flatten() {
        // SAFETY: a local leaves the list when it is dropped.
        unsafe { (*local).flush() };
    }
}

/// A local of an original function whose address the game is given.
///
/// It derefs to the value. `as_ptr`/`as_mut_ptr` return its address: with
/// `guest-memory` a guest address in [`GUEST_LOCALS`], where the value is
/// written in the guest's byte order; without it the host address, as
/// before. Reads through the deref pick up what the guest wrote since; a
/// write through the deref reaches guest memory before the guest next
/// runs or the platform helpers next touch memory.
///
/// Do not move a `GuestLocal` after handing out its address (the rewrite
/// never does: values are copied out with `*`).
///
/// # Original
/// Host-side adapter: the original's stack slots.
///
/// # Confidence
/// high - every access syncs through the platform helpers.
pub struct GuestLocal<T: GuestPod> {
    value: core::cell::UnsafeCell<T>,
    #[cfg(feature = "guest-memory")]
    addr: core::cell::Cell<u32>,
    #[cfg(feature = "guest-memory")]
    state: core::cell::Cell<u8>,
    #[cfg(feature = "guest-memory")]
    epoch: core::cell::Cell<u64>,
}

impl<T: GuestPod> GuestLocal<T> {
    /// Wraps `value`; no guest memory is taken until its address is.
    #[inline]
    pub const fn new(value: T) -> Self {
        GuestLocal {
            value: core::cell::UnsafeCell::new(value),
            #[cfg(feature = "guest-memory")]
            addr: core::cell::Cell::new(0),
            #[cfg(feature = "guest-memory")]
            state: core::cell::Cell::new(LOCAL_HOST),
            #[cfg(feature = "guest-memory")]
            epoch: core::cell::Cell::new(0),
        }
    }

    /// The address the game is given (see the type's docs).
    ///
    /// # Original
    /// Host-side adapter: the address of the original's stack slot.
    ///
    /// # Confidence
    /// high - see [`GuestLocal`].
    #[inline]
    pub fn as_ptr(&self) -> *const T::Elem {
        self.as_mut_ptr()
    }

    /// The address the game is given (see the type's docs).
    ///
    /// # Original
    /// Host-side adapter: the address of the original's stack slot.
    ///
    /// # Confidence
    /// high - see [`GuestLocal`].
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T::Elem {
        #[cfg(feature = "guest-memory")]
        {
            self.export() as *mut T::Elem
        }
        #[cfg(not(feature = "guest-memory"))]
        {
            self.value.get() as *mut T::Elem
        }
    }

    /// The address of element `index`'s first item: what `a[index].as_ptr()`
    /// (a row of a 2-D array) or `a[index..].as_ptr()` gave on the host array.
    ///
    /// # Original
    /// Host-side adapter: an address inside the original's stack slot.
    ///
    /// # Confidence
    /// high - element size from [`GuestPod::SIZE`].
    #[inline]
    pub fn elem_ptr(&self, index: usize) -> *mut <T::Elem as GuestPod>::Elem
    where
        T::Elem: GuestPod,
    {
        #[cfg(feature = "guest-memory")]
        {
            (self.export() + index * <T::Elem as GuestPod>::SIZE) as *mut _
        }
        #[cfg(not(feature = "guest-memory"))]
        {
            // SAFETY: pointer arithmetic only; callers index within the array.
            unsafe { (self.value.get() as *mut T::Elem).add(index) as *mut _ }
        }
    }

    #[cfg(feature = "guest-memory")]
    fn export(&self) -> usize {
        if self.addr.get() == 0 {
            let addr = LOCAL_ARENA.with(|arena| {
                let mut arena = arena.borrow_mut();
                let at = arena.top.next_multiple_of(16);
                let end = at as usize + T::SIZE.max(1);
                assert!(end <= GUEST_LOCALS.1 as usize, "guest locals area is full");
                arena.top = end as u32;
                arena.live += 1;
                at
            });
            self.addr.set(addr);
        }
        if self.state.get() == LOCAL_HOST {
            self.forget_dirty();
            // SAFETY: the slot is ours and inside guest RAM.
            unsafe { (*self.value.get()).store(self.addr.get() as usize) };
            self.state.set(LOCAL_GUEST);
            self.epoch.set(guest_epoch());
        }
        self.addr.get() as usize
    }

    #[cfg(feature = "guest-memory")]
    fn pull(&self) {
        if self.state.get() == LOCAL_GUEST && self.epoch.get() != guest_epoch() {
            // SAFETY: no reference into the value outlives a deref.
            unsafe { (*self.value.get()).load(self.addr.get() as usize) };
            self.epoch.set(guest_epoch());
        }
    }

    #[cfg(feature = "guest-memory")]
    fn forget_dirty(&self) {
        let me = self as *const Self as *const ();
        DIRTY_LOCALS.with(|d| d.borrow_mut().retain(|&(id, _)| id != me));
    }
}

#[cfg(feature = "guest-memory")]
impl<T: GuestPod> DirtyLocal for GuestLocal<T> {
    unsafe fn flush(&self) {
        (*self.value.get()).store(self.addr.get() as usize);
        self.state.set(LOCAL_GUEST);
        self.epoch.set(guest_epoch());
    }
}

impl<T: GuestPod> core::ops::Deref for GuestLocal<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        #[cfg(feature = "guest-memory")]
        self.pull();
        // SAFETY: syncs happen only inside these methods.
        unsafe { &*self.value.get() }
    }
}

impl<T: GuestPod> core::ops::DerefMut for GuestLocal<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        #[cfg(feature = "guest-memory")]
        {
            self.pull();
            if self.addr.get() != 0 && self.state.get() == LOCAL_GUEST {
                self.state.set(LOCAL_HOST);
                let me = self as *const Self;
                DIRTY_LOCALS.with(|d| {
                    d.borrow_mut().push((me as *const (), me as *const dyn DirtyLocal));
                });
            }
        }
        self.value.get_mut()
    }
}

impl<T: GuestPod> Drop for GuestLocal<T> {
    fn drop(&mut self) {
        #[cfg(feature = "guest-memory")]
        if self.addr.get() != 0 {
            self.forget_dirty();
            LOCAL_ARENA.with(|arena| {
                let mut arena = arena.borrow_mut();
                arena.live -= 1;
                if arena.live == 0 {
                    arena.top = GUEST_LOCALS.0;
                }
            });
        }
    }
}

/// One argument as the PowerPC EABI passes it: a word in the next GPR, a
/// 64-bit value in an aligned GPR pair, or a float widened into the next FPR.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GuestSlot {
    Word(u32),
    Pair(u64),
    Float(f64),
}

/// A Rust argument type that a guest function receives in registers.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - the EABI's integer, pair and float classes.
pub trait GuestArg {
    fn slot(self) -> GuestSlot;
}

macro_rules! guest_word_arg {
    ($($t:ty => |$v:ident| $e:expr),* $(,)?) => {$(
        impl GuestArg for $t {
            #[inline]
            fn slot(self) -> GuestSlot {
                let $v = self;
                GuestSlot::Word($e)
            }
        }
    )*};
}

guest_word_arg! {
    u32 => |v| v,
    i32 => |v| v as u32,
    u16 => |v| u32::from(v),
    i16 => |v| v as i32 as u32,
    u8 => |v| u32::from(v),
    i8 => |v| v as i32 as u32,
    bool => |v| u32::from(v),
    usize => |v| u32::try_from(v).expect("guest call argument is wider than 32 bits"),
}

impl GuestArg for u64 {
    #[inline]
    fn slot(self) -> GuestSlot {
        GuestSlot::Pair(self)
    }
}

impl GuestArg for f32 {
    #[inline]
    fn slot(self) -> GuestSlot {
        GuestSlot::Float(f64::from(self))
    }
}

impl GuestArg for f64 {
    #[inline]
    fn slot(self) -> GuestSlot {
        GuestSlot::Float(self)
    }
}

/// Whether `addr` is null or inside guest RAM (MEM1, MEM2, their uncached
/// mirrors, or the locked cache), so it can be handed to the guest as is.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - the Wii's RAM map.
pub fn is_guest_pointer(addr: usize) -> bool {
    const RANGES: [(usize, usize); 5] = [
        (0x8000_0000, 0x8180_0000),
        (0x9000_0000, 0x9400_0000),
        (0xC000_0000, 0xC180_0000),
        (0xD000_0000, 0xD400_0000),
        (0xE000_0000, 0xE000_4000),
    ];
    addr == 0 || RANGES.iter().any(|&(lo, hi)| (lo..hi).contains(&addr))
}

fn pointer_slot(addr: usize) -> GuestSlot {
    // A host pointer (usually `&mut local`) means nothing to the guest. Such a
    // call needs `call_registers_with_scratch`; stop here instead of letting
    // the guest write through a truncated host address.
    assert!(
        is_guest_pointer(addr),
        "host pointer {addr:#x} passed to a guest call (needs call_registers_with_scratch)"
    );
    GuestSlot::Word(addr as u32)
}

impl<T> GuestArg for *mut T {
    #[inline]
    fn slot(self) -> GuestSlot {
        pointer_slot(self as usize)
    }
}

impl<T> GuestArg for *const T {
    #[inline]
    fn slot(self) -> GuestSlot {
        pointer_slot(self as usize)
    }
}

/// A Rust return type read back from `r3`, `r3:r4` or `f1`.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - the EABI's return registers.
pub trait GuestRet {
    fn from_registers(r3: u32, r4: u32, f1: f64) -> Self;
}

macro_rules! guest_ret {
    ($($t:ty => |$r3:ident, $r4:ident, $f1:ident| $e:expr),* $(,)?) => {$(
        impl GuestRet for $t {
            #[inline]
            fn from_registers($r3: u32, $r4: u32, $f1: f64) -> Self {
                let _ = ($r3, $r4, $f1);
                $e
            }
        }
    )*};
}

guest_ret! {
    () => |r3, r4, f1| (),
    u32 => |r3, r4, f1| r3,
    i32 => |r3, r4, f1| r3 as i32,
    u16 => |r3, r4, f1| r3 as u16,
    i16 => |r3, r4, f1| r3 as i16,
    u8 => |r3, r4, f1| r3 as u8,
    i8 => |r3, r4, f1| r3 as i8,
    bool => |r3, r4, f1| r3 as u8 != 0,
    usize => |r3, r4, f1| r3 as usize,
    u64 => |r3, r4, f1| (u64::from(r3) << 32) | u64::from(r4),
    f32 => |r3, r4, f1| f1 as f32,
    f64 => |r3, r4, f1| f1,
}

/// Bytes of outgoing stack arguments a [`guest_call`] can lay out.
pub const GUEST_CALL_STACK_BYTES: usize = 64;

/// The registers and stack words for one call, laid out per the EABI.
#[derive(Debug, PartialEq)]
pub struct GuestArgs {
    pub gprs: [u32; 8],
    pub gpr_count: usize,
    pub fprs: [f64; 8],
    pub fpr_count: usize,
    /// Big-endian bytes from `r1 + 8`.
    pub stack: [u8; GUEST_CALL_STACK_BYTES],
    pub stack_len: usize,
}

impl GuestArgs {
    /// Lays out `slots` in order. Integers past `r10` go to the stack (pairs
    /// 8-aligned, and a pair never splits across `r10`); more than eight
    /// floats panics, since no caller needs them.
    ///
    /// # Original
    /// Host-side adapter with no guest address.
    ///
    /// # Confidence
    /// high - the EABI's argument rules; see the tests below.
    pub fn layout(slots: &[GuestSlot]) -> Self {
        let mut args = GuestArgs {
            gprs: [0; 8],
            gpr_count: 0,
            fprs: [0.0; 8],
            fpr_count: 0,
            stack: [0; GUEST_CALL_STACK_BYTES],
            stack_len: 0,
        };
        for slot in slots {
            match *slot {
                GuestSlot::Word(word) if args.gpr_count < 8 => {
                    args.gprs[args.gpr_count] = word;
                    args.gpr_count += 1;
                }
                GuestSlot::Word(word) => args.push_stack(&word.to_be_bytes(), 4),
                GuestSlot::Pair(pair) => {
                    args.gpr_count += args.gpr_count & 1;
                    if args.gpr_count < 8 {
                        args.gprs[args.gpr_count] = (pair >> 32) as u32;
                        args.gprs[args.gpr_count + 1] = pair as u32;
                        args.gpr_count += 2;
                    } else {
                        args.gpr_count = 8;
                        args.push_stack(&pair.to_be_bytes(), 8);
                    }
                }
                GuestSlot::Float(value) => {
                    assert!(args.fpr_count < 8, "guest call has more than eight float arguments");
                    args.fprs[args.fpr_count] = value;
                    args.fpr_count += 1;
                }
            }
        }
        args
    }

    fn push_stack(&mut self, bytes: &[u8], align: usize) {
        let at = self.stack_len.next_multiple_of(align);
        let end = at + bytes.len();
        assert!(end <= GUEST_CALL_STACK_BYTES, "guest call stack arguments overflow");
        self.stack[at..end].copy_from_slice(bytes);
        self.stack_len = end;
    }
}

/// Calls the guest function at `addr` with typed arguments, for
/// [`guest_fn!`]. Panics if the call fails, like the legacy helpers.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - forwards to [`call_registers`] or, with stack arguments,
/// [`call_frame_with_scratch`].
#[cfg(feature = "guest-memory")]
pub fn guest_call<R: GuestRet>(addr: usize, slots: &[GuestSlot]) -> R {
    let args = GuestArgs::layout(slots);
    let gprs = &args.gprs[..args.gpr_count];
    let fprs = &args.fprs[..args.fpr_count];
    let result = if args.stack_len == 0 {
        call_registers(addr, gprs, fprs)
    } else {
        let mut words = [GuestWord::Immediate(0); 8];
        for (word, &value) in words.iter_mut().zip(gprs) {
            *word = GuestWord::Immediate(value);
        }
        let stack = &args.stack[..args.stack_len.next_multiple_of(4)];
        call_frame_with_scratch(addr, &words[..args.gpr_count], fprs, stack, &mut [])
    };
    let result = result.unwrap_or_else(|error| panic!("guest call to {addr:#010x} failed: {error}"));
    R::from_registers(result.r3, result.r4, result.f1)
}

/// A callable for the guest function at `addr` with the given `fn` type:
/// `let f = platform::guest_fn!(addr, fn(u32, f32) -> u32); f(this, x)`.
///
/// With `guest-memory` it is a closure that runs the function on the guest
/// CPU through [`guest_call`]. Without it, it is the legacy host function
/// pointer (a transmute of `addr`), unchanged.
///
/// # Original
/// Host-side adapter with no guest address.
///
/// # Confidence
/// high - mechanical; [`GuestArgs::layout`] carries the ABI.
macro_rules! guest_fn {
    ($addr:expr, fn($($t:ty),* $(,)?) $(-> $r:ty)?) => {
        $crate::platform::guest_fn!(
            @build $addr; [$($t),*]; []; [p0 p1 p2 p3 p4 p5 p6 p7 p8 p9 p10 p11 p12 p13 p14 p15]; $($r)?
        )
    };
    (@build $addr:expr; [$t:ty $(, $rest:ty)*]; [$($done:tt)*]; [$n:ident $($names:ident)*]; $($r:ty)?) => {
        $crate::platform::guest_fn!(
            @build $addr; [$($rest),*]; [$($done)* ($n: $t)]; [$($names)*]; $($r)?
        )
    };
    (@build $addr:expr; []; [$(($n:ident: $t:ty))*]; [$($unused:ident)*]; $($r:ty)?) => {{
        let addr: usize = ($addr) as usize;
        #[cfg(feature = "guest-memory")]
        let function = move |$($n: $t),*| -> $crate::platform::guest_fn!(@ret $($r)?) {
            $crate::platform::guest_call(
                addr,
                &[$($crate::platform::GuestArg::slot($n)),*],
            )
        };
        #[cfg(not(feature = "guest-memory"))]
        #[allow(unused_unsafe)]
        let function = unsafe {
            core::mem::transmute::<usize, extern "C" fn($($t),*) $(-> $r)?>(addr)
        };
        function
    }};
    (@ret) => { () };
    (@ret $r:ty) => { $r };
}
pub(crate) use guest_fn;

#[cfg(test)]
mod guest_args_tests {
    use super::*;

    #[test]
    fn words_pairs_and_floats_take_their_own_registers() {
        let args = GuestArgs::layout(&[
            1u32.slot(),
            0x1122_3344_5566_7788u64.slot(),
            2.5f32.slot(),
            (-1i16).slot(),
            7.0f64.slot(),
        ]);
        // r3 = 1, r4 skipped so the pair is r5:r6, then r7.
        assert_eq!(&args.gprs[..args.gpr_count], &[1, 0, 0x1122_3344, 0x5566_7788, 0xFFFF_FFFF]);
        assert_eq!(&args.fprs[..args.fpr_count], &[2.5, 7.0]);
        assert_eq!(args.stack_len, 0);
    }

    #[test]
    fn words_past_r10_go_to_the_stack_in_order() {
        let slots: [GuestSlot; 10] = core::array::from_fn(|i| (i as u32 + 1).slot());
        let args = GuestArgs::layout(&slots);
        assert_eq!(args.gprs, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(&args.stack[..args.stack_len], &[0, 0, 0, 9, 0, 0, 0, 10]);
    }

    #[test]
    fn a_pair_never_splits_across_r10() {
        let mut slots = [GuestSlot::Word(0); 8];
        for (i, slot) in slots.iter_mut().enumerate().take(7) {
            *slot = (i as u32).slot();
        }
        slots[7] = 0x0102_0304_0506_0708u64.slot();
        let args = GuestArgs::layout(&slots);
        assert_eq!(args.gpr_count, 8);
        assert_eq!(&args.stack[..args.stack_len], &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn host_pointers_are_refused() {
        assert_eq!((0x8040_0000usize as *mut u32).slot(), GuestSlot::Word(0x8040_0000));
        assert_eq!(core::ptr::null::<u8>().slot(), GuestSlot::Word(0));
        let mut local = 0u32;
        let host = &mut local as *mut u32;
        if !is_guest_pointer(host as usize) {
            extern crate std;
            assert!(std::panic::catch_unwind(|| host.slot()).is_err());
        }
    }

    #[test]
    fn returns_come_from_r3_r4_and_f1() {
        assert_eq!(u64::from_registers(1, 2, 0.0), 0x1_0000_0002);
        assert_eq!(i16::from_registers(0xFFFF, 0, 0.0), -1);
        assert_eq!(f32::from_registers(0, 0, 0.5), 0.5);
    }
}

#[cfg(all(test, feature = "guest-memory"))]
mod tests {
    use super::*;
    use guest_mem::{Mmio, NullMmio};
    use std::boxed::Box;

    struct TestRuntime(Mem);

    impl GuestRuntime for TestRuntime {
        fn memory(&mut self) -> &mut Mem {
            &mut self.0
        }

        fn resolve_rel_address(
            &self,
            address: RelAddress,
            len: usize,
            executable: bool,
        ) -> Option<u32> {
            match (
                address.module,
                address.section,
                address.offset,
                len,
                executable,
            ) {
                (7, 1, 0, 4, true) => Some(0x8000_2000),
                (7, 3, 0, 1, false) => Some(0x9000_1000),
                _ => None,
            }
        }

        fn call_registers(
            &mut self,
            addr: u32,
            gprs: &[u32],
            fprs: &[f64],
        ) -> Result<GuestCallResult, std::string::String> {
            assert_eq!(addr, 0x8000_2000);
            let calls = self.0.read_u8(0x8000_0100).unwrap();
            self.0.write_u8(0x8000_0100, calls + 1).unwrap();
            let value: u32 = gprs.iter().zip(1..).map(|(word, index)| word * index).sum();
            Ok(GuestCallResult {
                r3: 0x8000_0000 | value,
                r4: 0x1234_5678,
                f1: fprs.iter().copied().sum(),
            })
        }

        fn call_frame_with_scratch(
            &mut self,
            _addr: u32,
            gprs: &[GuestWord],
            _fprs: &[f64],
            stack: &[u8],
            scratch: &mut [u8],
        ) -> Result<GuestCallResult, String> {
            if !scratch.is_empty() || gprs.len() != 8 {
                return Err("scratch is unavailable in this fixture".into());
            }
            // Stack-argument calls report the stack words they were given.
            let words = stack.chunks(4).map(|w| u32::from_be_bytes(w.try_into().unwrap()));
            Ok(GuestCallResult {
                r3: words.fold(0, |sum, word| sum * 100 + word),
                r4: stack.len() as u32,
                f1: 0.0,
            })
        }
    }

    #[test]
    fn guest_locals_share_their_bytes_with_the_guest() {
        let mut mem = Mem::new(Box::new(NullMmio));
        with_guest_memory(&mut mem, || unsafe {
            let mut pos = GuestLocal::new([1.5f32, -2.0, 0.25]);
            let at = pos.as_mut_ptr() as usize;
            assert_eq!(at, GUEST_LOCALS.0 as usize);
            // Exported in the guest's byte order.
            assert_eq!(read_u32(at), 1.5f32.to_bits());
            // A guest write is seen by the next Rust read...
            write_f32(at + 4, 9.0);
            assert_eq!(pos[1], 9.0);
            // ...and a Rust write by the next guest read.
            pos[2] = 4.0;
            assert_eq!(read_f32(at + 8), 4.0);
            assert_eq!(*pos, [1.5, 9.0, 4.0]);

            let mut word = GuestLocal::new(0u32);
            let w = word.as_mut_ptr() as usize;
            assert_eq!(w, at + 16);
            write_u32(w, 7);
            assert_eq!(*word, 7);
            *word += 1;
            assert_eq!(read_u32(w), 8);
            // Pointer arithmetic keeps the element size.
            assert_eq!(pos.as_ptr().add(2) as usize, at + 8);
            drop(word);
            drop(pos);
            // The area is reused once every local is gone.
            let again = GuestLocal::new(0u16);
            assert_eq!(again.as_ptr() as usize, GUEST_LOCALS.0 as usize);
        });
    }

    #[test]
    fn guest_fn_passes_typed_arguments_and_returns() {
        let mut runtime = TestRuntime(Mem::new(Box::new(NullMmio)));
        with_guest_runtime(&mut runtime, || {
            let entry = 0x8000_2000usize;
            // Words take r3, r4 in order; the float goes to f1.
            let words = guest_fn!(entry, fn(u32, f32, usize) -> u32);
            assert_eq!(words(1, 0.5, 2), 0x8000_0005);
            let floats = guest_fn!(entry, fn(f32, f64) -> f32);
            assert_eq!(floats(0.5, 0.25), 0.75);
            let pair = guest_fn!(entry, fn() -> u64);
            assert_eq!(pair(), 0x8000_0000_1234_5678);
            let signed = guest_fn!(entry, fn(i16) -> i32);
            // -1 is sign-extended to 0xFFFF_FFFF in r3.
            assert_eq!(signed(-1), -1);
            let nothing = guest_fn!(entry, fn(u32));
            nothing(3);
            let ten = guest_fn!(
                entry,
                fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u64
            );
            assert_eq!(ten(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), (910 << 32) | 8);
            assert_eq!(unsafe { read_u8(0x8000_0100) }, 5);
        });
    }

    #[test]
    fn guest_call_helpers_use_the_scoped_runtime_and_return_registers() {
        let mut runtime = TestRuntime(Mem::new(Box::new(NullMmio)));
        with_guest_runtime(&mut runtime, || unsafe {
            let entry = 0x8000_2000;
            call0(entry);
            assert_eq!(crate::runtime::os::call0_ret(entry), 0x8000_0000);
            assert_eq!(call1(entry, 1), 0x8000_0001);
            assert_eq!(call2(entry, 1, 2), 0x8000_0005);
            assert_eq!(call3(entry, 1, 2, 3), 0x8000_000e);
            assert_eq!(call4(entry, 1, 2, 3, 4), 0x8000_001e);
            assert_eq!(call5(entry, 1, 2, 3, 4, 5), 0x8000_0037);
            assert_eq!(call6(entry, 1, 2, 3, 4, 5, 6), 0x8000_005b);
            assert_eq!(call7(entry, 1, 2, 3, 4, 5, 6, 7), 0x8000_008c);
            assert_eq!(call_u64(entry), 0x8000_0000_1234_5678);
            let r = call_registers(entry, &[4, 5], &[1.25, -2.5]).unwrap();
            assert_eq!((r.r3, r.r4, r.f1), (0x8000_000e, 0x1234_5678, -1.25));
            assert_eq!(read_u8(0x8000_0100), 11);
        });
        assert!(call_registers(0x8000_2000, &[], &[]).is_err());
    }

    #[test]
    fn relative_calls_use_only_resolved_executable_sections() {
        let mut runtime = TestRuntime(Mem::new(Box::new(NullMmio)));
        let code = RelAddress {
            module: 7,
            section: 1,
            offset: 0,
        };
        let data = RelAddress { section: 3, ..code };
        let missing = RelAddress { module: 8, ..code };
        with_guest_runtime(&mut runtime, || {
            assert_eq!(
                resolve_rel_address(data, 1, false).unwrap(),
                Some(0x9000_1000)
            );
            assert_eq!(resolve_rel_address(missing, 1, false).unwrap(), None);
            assert!(call_rel_registers(data, &[2], &[2.5]).is_err());
            assert!(call_rel_registers(missing, &[], &[]).is_err());
            let returned = call_rel_registers(code, &[2], &[2.5]).unwrap();
            assert_eq!((returned.r3, returned.f1), (0x8000_0002, 2.5));
            assert_eq!(unsafe { read_u8(0x8000_0100) }, 1);
        });
        assert!(resolve_rel_address(code, 4, true).is_err());
    }

    #[test]
    fn memory_only_scope_restores_the_runtime_after_an_unwind() {
        let mut runtime = TestRuntime(Mem::new(Box::new(NullMmio)));
        let mut inner = Mem::new(Box::new(NullMmio));
        with_guest_runtime(&mut runtime, || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                with_guest_memory(&mut inner, || {
                    assert!(call_registers(0x8000_2000, &[], &[]).is_err());
                    panic!("inner scope");
                });
            }));
            assert!(result.is_err());
            assert_eq!(unsafe { call1(0x8000_2000, 1) }, 0x8000_0001);
        });
    }

    #[test]
    fn platform_helpers_read_and_write_big_endian_ram() {
        let mut mem = Mem::new(Box::new(NullMmio));
        mem.ram_mut(0x8000_0100, 8)
            .unwrap()
            .copy_from_slice(&[0x81, 0x23, 0x45, 0x67, 0x3f, 0xc0, 0, 0]);

        with_guest_memory(&mut mem, || unsafe {
            let addr = 0xc000_0100;
            assert_eq!(read_u8(addr), 0x81);
            assert_eq!(read_u16(addr + 1), 0x2345);
            assert_eq!(read_u32(addr), 0x8123_4567);
            assert_eq!(read_i32(addr), -2_128_394_905);
            assert_eq!(read_f32(addr + 4), 1.5);
            write_u8(addr, 0x7e);
            write_u16(addr + 1, 0x1234);
            write_u32(addr + 3, 0x5678_9abc);
            write_i32(addr + 7, -2);
            write_f32(addr + 11, -2.5);
        });

        assert_eq!(
            mem.ram(0x8000_0100, 15).unwrap(),
            &[0x7e, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xff, 0xff, 0xff, 0xfe, 0xc0, 0x20, 0, 0]
        );
    }

    #[test]
    fn platform_passes_word_accesses_to_mmio_at_full_width() {
        struct Register(u32);

        impl Mmio for Register {
            fn read(&mut self, addr: u32, width: u32) -> u32 {
                assert_eq!((addr, width), (0xcc00_0080, 4));
                self.0
            }

            fn write(&mut self, addr: u32, width: u32, value: u32) {
                assert_eq!((addr, width), (0xcc00_0080, 4));
                self.0 = value;
            }
        }

        let mut mem = Mem::new(Box::new(Register(0)));
        with_guest_memory(&mut mem, || unsafe {
            write_u32(0xcc00_0080, 0x1234_5678);
            assert_eq!(read_u32(0xcc00_0080), 0x1234_5678);
        });
    }

    #[test]
    #[should_panic(expected = "guest memory fault")]
    fn unmapped_guest_address_reports_a_fault() {
        let mut mem = Mem::new(Box::new(NullMmio));
        with_guest_memory(&mut mem, || unsafe { read_u32(0x8180_0000) });
    }

    #[test]
    fn guest_memory_scope_restores_the_outer_memory_after_an_unwind() {
        let mut outer = Mem::new(Box::new(NullMmio));
        let mut inner = Mem::new(Box::new(NullMmio));
        with_guest_memory(&mut outer, || {
            unsafe { write_u8(0x8000_0100, 0xaa) };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                with_guest_memory(&mut inner, || unsafe {
                    write_u8(0x8000_0100, 0xbb);
                    read_u8(0x8180_0000);
                });
            }));
            assert!(result.is_err());
            assert_eq!(unsafe { read_u8(0x8000_0100) }, 0xaa);
        });

        assert_eq!(inner.read_u8(0x8000_0100), Ok(0xbb));
        assert!(std::panic::catch_unwind(|| unsafe { read_u8(0x8000_0100) }).is_err());
    }
}
