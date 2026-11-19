//! PowerPC cache maintenance required after copying code or data into RAM.
//!
//! # What this module covers
//!
//! | Address | Item | Role |
//! |---------|------|------|
//! | `0x80004330` | [`flush_cache`] | write back data lines, then invalidate instruction lines |
//! | `0x80004390` | [`dcbt`] | prefetch hint issued by [`crate::boot::mem::memcpy`] |
//!
//! # Why it exists
//!
//! The DOL is loaded by the Wii loader, but the CRT still copies blocks from
//! their load address to their run address (`init_tables`). On PowerPC the new
//! bytes may still be dirty in the data cache while the instruction fetch path
//! has stale lines, so the copy is followed by a `dcbst`/`sync`/`icbi`/`isync`
//! sequence. Without it, executing freshly copied code sees old contents.
//!
//! # Start here
//!
//! [`flush_cache`]. [`dcbt`] is a side helper with no architectural effect.
//!
//! # Dependencies
//!
//! * `core::arch::asm!` - the cache instructions have no `crate::platform`
//!   wrapper, so they are emitted directly on PowerPC targets.
//!
//! # Portability
//!
//! The crate is checked on the host with `cargo check`. On non-PowerPC targets
//! the four cache instructions are documented no-ops: they cannot be assembled
//! for x86, and the host build never executes target memory anyway.
//!
//! # Evidence
//!
//! `work/ghidra/main-gb/export/80004330_FUN_80004330.c` and the DOL bytes
//! `0x7C00286C` (`dcbst`), `0x7C0004AC` (`sync`), `0x7C002FAC` (`icbi`),
//! `0x4C00012C` (`isync`).

/// Writes back and invalidates the cache lines that cover `addr..addr+size`.
///
/// The loop steps in 8-byte units so that every 32-byte PowerPC cache block
/// touched by the range is covered, even when the start address is unaligned.
///
/// # Original
/// - Ghidra: `FUN_80004330` at `0x80004330` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004330_FUN_80004330.c`
///
/// # Arguments
/// * `addr` - start of the range to publish (typically a copy destination).
/// * `size` - signed byte count. The original takes `int`; the value is
///   adjusted by the start misalignment and then counted down to `< 0`, so the
///   loop always executes at least once, including for `size <= 0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * `dcbst` on each step (write back a dirty data-cache block)
/// * `sync` (wait for the write-back to complete)
/// * `icbi` on each step (invalidate the instruction-cache block)
/// * `isync` once at the end
///
/// # Safety
/// Only meaningful on PowerPC; `addr` need not be aligned. No memory is
/// dereferenced by the Rust code.
///
/// # Confidence
/// high - instruction sequence and loop bound match the exported C exactly.
/// `// unknown:` the mask `0xFFFFFFF1` clears address bits 1..3 but keeps bit 0;
/// the reason for preserving bit 0 is not established (the original compiler
/// may be relying on cache-block granularity).
pub unsafe fn flush_cache(addr: u32, size: i32) {
    // observed: `lis r5,0xFFFF / ori r5,r5,0xFFF1 / and r5,r5,r3`.
    const START_MASK: u32 = 0xFFFF_FFF1;
    // Cache-maintenance step: half of the 32-byte Gekko cache block.
    // `// inferred:` from `addic r5,r5,8` at `0x80004350`.
    const STEP: u32 = 8;

    let mut p = addr & START_MASK;
    // observed: `subf r3,r5,r3; add r4,r4,r3` - fold the skipped lead bytes
    // into the count so the whole requested range is covered.
    let mut left = size.wrapping_add(addr.wrapping_sub(p) as i32);

    loop {
        dcbst(p);
        sync();
        icbi(p);
        p = p.wrapping_add(STEP);
        left = left.wrapping_sub(STEP as i32);
        // observed: `addic. r4,r4,-8; bge` - a signed `>= 0` test.
        if left < 0 {
            break;
        }
    }
    isync();
}

/// Prefetches the cache block containing `addr` into the data cache.
///
/// # Original
/// - observed: `dcbt 0,r4` at `0x80004390`, inside
///   [`crate::boot::mem::memcpy`]'s large-copy path.
/// - Ghidra renders it as `dataCacheBlockTouch(param_2)` in
///   `80004364_FUN_80004364.c`.
///
/// # Arguments
/// * `addr` - any address in the block to prefetch.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None architecturally; it is a performance hint and cannot fault.
///
/// # Safety
/// Safe on PowerPC; the non-PowerPC build ignores it.
///
/// # Confidence
/// high.
#[cfg(target_arch = "powerpc")]
pub unsafe fn dcbt(addr: usize) {
    core::arch::asm!("dcbt 0, {0}", in(reg) addr);
}

/// Prefetches the cache block containing `addr` into the data cache.
///
/// Non-PowerPC build: no-op (see the module portability note).
///
/// # Original
/// - observed: `dcbt 0,r4` at `0x80004390`.
///
/// # Confidence
/// high for the PowerPC behaviour; the host build is intentionally empty.
#[cfg(not(target_arch = "powerpc"))]
pub unsafe fn dcbt(_addr: usize) {}

// ---------------------------------------------------------------------------
// Instruction wrappers.
//
// Each wrapper emits one PowerPC cache/barrier instruction. The non-PowerPC
// twin exists solely so `cargo check` works on the development host; it does
// nothing. Evidence for every opcode is the call site in `flush_cache`
// (0x80004344..0x8000435C) except `dcbt` (0x80004390).
// ---------------------------------------------------------------------------

/// `dcbst` - data cache block store (writes back a dirty line).
///
/// # Arguments
/// * `addr` - any address inside the line.
///
/// # Side effects
/// May write a dirty cache line back to memory.
///
/// # Safety
/// PowerPC only; no address translation is performed by this wrapper.
///
/// # Confidence
/// high.
#[cfg(target_arch = "powerpc")]
#[inline]
unsafe fn dcbst(addr: u32) {
    core::arch::asm!("dcbst 0, {0}", in(reg) addr);
}

/// Host replacement for `dcbst`; intentionally empty.
#[cfg(not(target_arch = "powerpc"))]
#[inline]
unsafe fn dcbst(_addr: u32) {}

/// `sync` - wait for all preceding memory accesses to become visible.
///
/// # Arguments
/// None.
///
/// # Side effects
/// Processor synchronization only.
///
/// # Safety
/// PowerPC only; harmless, but has no host equivalent.
///
/// # Confidence
/// high.
#[cfg(target_arch = "powerpc")]
#[inline]
unsafe fn sync() {
    core::arch::asm!("sync");
}

/// Host replacement for `sync`; intentionally empty.
#[cfg(not(target_arch = "powerpc"))]
#[inline]
unsafe fn sync() {}

/// `icbi` - instruction cache block invalidate.
///
/// # Arguments
/// * `addr` - any address inside the instruction-cache line.
///
/// # Side effects
/// Invalidates the line so the next fetch reads the new bytes from memory.
///
/// # Safety
/// PowerPC only; callers must have written the code bytes first.
///
/// # Confidence
/// high.
#[cfg(target_arch = "powerpc")]
#[inline]
unsafe fn icbi(addr: u32) {
    core::arch::asm!("icbi 0, {0}", in(reg) addr);
}

/// Host replacement for `icbi`; intentionally empty.
#[cfg(not(target_arch = "powerpc"))]
#[inline]
unsafe fn icbi(_addr: u32) {}

/// `isync` - wait for the instruction stream to be refetched.
///
/// # Arguments
/// None.
///
/// # Side effects
/// Flushes the processor's instruction pipeline view.
///
/// # Safety
/// PowerPC only; harmless, but has no host equivalent.
///
/// # Confidence
/// high.
#[cfg(target_arch = "powerpc")]
#[inline]
unsafe fn isync() {
    core::arch::asm!("isync");
}

/// Host replacement for `isync`; intentionally empty.
#[cfg(not(target_arch = "powerpc"))]
#[inline]
unsafe fn isync() {}
