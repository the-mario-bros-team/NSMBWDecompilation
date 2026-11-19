//! Wii / Broadway low-level access helpers.
//!
//! The target is big-endian PowerPC; byte-wise volatile access keeps host
//! `cargo check` semantics independent of the host endianness.
//!
//! These are host-side helpers, not recovered functions. They model the
//! primitive loads, stores and indirect calls used by the reconstruction.

/// Reads one byte from a virtual address with a volatile load.
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
    core::ptr::read_volatile(addr as *const u8)
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
    u16::from_be_bytes([read_u8(addr), read_u8(addr + 1)])
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
    u32::from_be_bytes([
        read_u8(addr),
        read_u8(addr + 1),
        read_u8(addr + 2),
        read_u8(addr + 3),
    ])
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

/// Writes one byte to a virtual address with a volatile store.
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
    core::ptr::write_volatile(addr as *mut u8, value);
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
    let b = value.to_be_bytes();
    for (i, byte) in b.iter().enumerate() {
        write_u8(addr + i, *byte);
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
    let b = value.to_be_bytes();
    for (i, byte) in b.iter().enumerate() {
        write_u8(addr + i, *byte);
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
/// `addr` must point to a function with this ABI, and the caller must be on a
/// PowerPC target (or an ABI-compatible host).
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call0(addr: usize) {
    let f: extern "C" fn() = core::mem::transmute(addr);
    f()
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
    let f: extern "C" fn(u32) -> u32 = core::mem::transmute(addr);
    f(a)
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
    let f: extern "C" fn(u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b)
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
    let f: extern "C" fn(u32, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c)
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
    let f: extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d)
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
    let f: extern "C" fn(u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d, e)
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
    let func: extern "C" fn(u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    func(a, b, c, d, e, f)
}

/// Calls a PowerPC function with seven integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Arguments are passed in
/// `r3`..`r9`, which is the limit before stack arguments.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub unsafe fn call7(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    let func: extern "C" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    func(a, b, c, d, e, f, g)
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
    let f: extern "C" fn() -> u64 = core::mem::transmute(addr);
    f()
}
