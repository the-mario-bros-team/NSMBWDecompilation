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
    GUEST_CONTEXT.with(|slot| {
        let mut active = slot
            .try_borrow_mut()
            .map_err(|_| "guest runtime is already in use")?;
        let Some(GuestContext::Runtime(runtime)) = active.as_mut() else {
            return Err("guest runtime is not in scope".into());
        };
        // SAFETY: the scope owns the exclusive reference and the slot
        // remains borrowed until the operation returns.
        run(unsafe { runtime.as_mut() })
    })
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
            _gprs: &[GuestWord],
            _fprs: &[f64],
            _stack: &[u8],
            _scratch: &mut [u8],
        ) -> Result<GuestCallResult, String> {
            Err("scratch is unavailable in this fixture".into())
        }
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
