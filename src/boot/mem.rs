//! Byte-copy and byte-fill primitives used during boot.
//!
//! # What this module covers
//!
//! Original address range `0x80004364..=0x800046E3`:
//!
//! | Address | Item | Role |
//! |---------|------|------|
//! | `0x80004364` | [`memcpy`] | 668-byte copy routine (forward/backward overlap-safe) |
//! | `0x80004600` | [`memset`] | 180-byte fill routine (word-at-a-time) |
//! | `0x800046B4` | [`memset_wrapper`] | 48-byte public `memset` that returns `dst` |
//!
//! # Why it exists
//!
//! The CRT's table pass ([`crate::boot::init_tables`]) copies initialized data
//! from its load address to its run address and zeroes the BSS pairs. Both use
//! these hand-written CodeWarrior routines instead of a compiler intrinsic,
//! which is why their block sizes and branches are visible here.
//!
//! # Start here
//!
//! [`memcpy`]; [`memset`] is the fill counterpart.
//!
//! # Dependencies
//!
//! * [`crate::platform`] - volatile big-endian loads/stores.
//! * [`crate::boot::cache::dcbt`] - prefetch hint issued by `memcpy`.
//!
//! # Evidence
//!
//! `work/ghidra/main-gb/export/80004364_FUN_80004364.c`,
//! `80004600_FUN_80004600.c`, `800046b4_FUN_800046b4.c`, plus the DOL
//! instruction stream for the block sizes and the `lfd`/`stfd` moves.

use crate::boot::cache;
use crate::platform;

/// Mask selecting the low two address bits (4-byte alignment).
const ALIGN_4_MASK: usize = 0x3;

/// Mask selecting the low three address bits (8-byte alignment).
const ALIGN_8_MASK: usize = 0x7;

/// Length in bytes at which `memcpy` switches from the 4-byte/16-byte block
/// strategy to the 8-byte/32-byte block strategy.
///
/// observed: `li r6,0x80; cmplw ...; blt` at `0x80004378..0x80004380`.
const LARGE_COPY_THRESHOLD: u32 = 0x80;

/// Minimum length for the 4-byte-aligned fast path.
///
/// observed: `li r6,0x14; cmplw ...; bgt` at `0x80004424..0x8000442C`.
const ALIGNED_MIN_LEN: u32 = 0x14;

// ---------------------------------------------------------------------------
// Small copy helpers.
// ---------------------------------------------------------------------------

/// Copies one 16-byte block forward, loading all source words before storing.
///
/// # Original
/// - observed: `lwz r9/r10/r11/r12` then `stw` x4 at `0x80004470..0x80004498`
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst` - first destination byte (16 bytes must be writable).
/// * `src` - first source byte (16 bytes must be readable).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 16 bytes at `dst`.
///
/// # Safety
/// Both 16-byte ranges must be valid. The load-before-store order keeps the
/// copy correct when source and destination overlap (forward-copy case).
///
/// # Confidence
/// high.
#[inline]
unsafe fn copy_block_16_forward(dst: usize, src: usize) {
    let mut words = [0u32; 4];
    for (i, word) in words.iter_mut().enumerate() {
        *word = platform::read_u32(src + i * 4);
    }
    for (i, word) in words.iter().enumerate() {
        platform::write_u32(dst + i * 4, *word);
    }
}

/// Copies one 32-byte block forward, loading all eight words before storing.
///
/// # Original
/// - observed: four `lfd`/`stfd` pairs (`f1..f4`, offsets 0/8/16/24) at
///   `0x800043D4..0x800043F8`
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst` - first destination byte (32 bytes must be writable).
/// * `src` - first source byte (32 bytes must be readable).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 32 bytes at `dst`.
///
/// # Safety
/// Both 32-byte ranges must be valid. The original moves 8-byte doublewords
/// through floating-point registers (FPRs); two 32-bit moves are byte-identical
/// on the big-endian target.
///
/// # Confidence
/// high.
#[inline]
unsafe fn copy_block_32_forward(dst: usize, src: usize) {
    let mut words = [0u32; 8];
    for (i, word) in words.iter_mut().enumerate() {
        *word = platform::read_u32(src + i * 4);
    }
    for (i, word) in words.iter().enumerate() {
        platform::write_u32(dst + i * 4, *word);
    }
}

/// Copies one 16-byte block backwards; both addresses are one-past-the-end.
///
/// # Original
/// - observed: `lwz -4/-8/-12/-16(r4)` then the four `stw`s at
///   `0x800045A8..0x800045C8`
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst_end` - one past the last destination byte.
/// * `src_end` - one past the last source byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the 16 bytes at `dst_end - 16 .. dst_end`.
///
/// # Safety
/// Both 16-byte ranges below the two end pointers must be valid.
///
/// # Confidence
/// high.
#[inline]
unsafe fn copy_block_16_backward(dst_end: usize, src_end: usize) {
    let mut words = [0u32; 4];
    for (i, word) in words.iter_mut().enumerate() {
        *word = platform::read_u32(src_end - 4 * (i + 1));
    }
    for (i, word) in words.iter().enumerate() {
        platform::write_u32(dst_end - 4 * (i + 1), *word);
    }
}

/// Copies one 32-byte block backwards; both addresses are one-past-the-end.
///
/// # Original
/// - observed: four `lfd -8/-16/-24/-32(r4)` then four `stfd`s at
///   `0x80004524..0x80004548`
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst_end` - one past the last destination byte.
/// * `src_end` - one past the last source byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the 32 bytes at `dst_end - 32 .. dst_end`.
///
/// # Safety
/// Both 32-byte ranges below the two end pointers must be valid. Loading the
/// whole block before storing keeps an overlapping copy correct.
///
/// # Confidence
/// high.
#[inline]
unsafe fn copy_block_32_backward(dst_end: usize, src_end: usize) {
    let mut words = [0u32; 8];
    for (i, word) in words.iter_mut().enumerate() {
        *word = platform::read_u32(src_end - 4 * (i + 1));
    }
    for (i, word) in words.iter().enumerate() {
        platform::write_u32(dst_end - 4 * (i + 1), *word);
    }
}

/// Copies `len` bytes forward, one at a time.
///
/// # Original
/// - observed: `lbzu r9,1(r7)` / `stbu r9,1(r8)` / `bdnz` at
///   `0x800044CC..0x800044D4`; the counters start at `src-1`/`dst-1`
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst` - first destination byte.
/// * `src` - first source byte.
/// * `len` - byte count; `0` performs no access.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes at `dst`.
///
/// # Safety
/// `dst..dst+len` writable, `src..src+len` readable.
///
/// # Confidence
/// high.
#[inline]
unsafe fn copy_bytes_forward(dst: usize, src: usize, len: u32) {
    let mut s = src.wrapping_sub(1);
    let mut d = dst.wrapping_sub(1);
    let mut left = len;
    while left != 0 {
        s = s.wrapping_add(1);
        d = d.wrapping_add(1);
        platform::write_u8(d, platform::read_u8(s));
        left -= 1;
    }
}

/// Copies `len` bytes backwards, one at a time; addresses are one-past-the-end.
///
/// # Original
/// - observed: `lbzu r9,-1(r4)` / `stbu r9,-1(r12)` / `bdnz` at
///   `0x800045DC..0x800045E4`
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst_end` - one past the last destination byte.
/// * `src_end` - one past the last source byte.
/// * `len` - byte count; `0` performs no access.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes ending at `dst_end`.
///
/// # Safety
/// `dst_end-len..dst_end` writable, `src_end-len..src_end` readable.
///
/// # Confidence
/// high.
#[inline]
unsafe fn copy_bytes_backward(dst_end: usize, src_end: usize, len: u32) {
    let mut d = dst_end;
    let mut s = src_end;
    let mut left = len;
    while left != 0 {
        d -= 1;
        s -= 1;
        platform::write_u8(d, platform::read_u8(s));
        left -= 1;
    }
}

// ---------------------------------------------------------------------------
// memcpy
// ---------------------------------------------------------------------------

/// Copies `len` bytes from `src` to `dst`, choosing a forward or backward walk
/// so that overlapping ranges are copied correctly (a `memmove`).
///
/// # Original
/// - Ghidra: `FUN_80004364` at `0x80004364` (668 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Arguments
/// * `dst` - destination byte address.
/// * `src` - source byte address.
/// * `len` - byte count; `0` returns without touching memory.
///
/// # Returns
/// Nothing in this reconstruction (Ghidra types the function `void`).
/// `// observed:` the original never writes `r3`, so `r3` still holds `dst` at
/// every `blr`; the boot caller ignores it.
///
/// # Side effects
/// Writes exactly `len` bytes at `dst`. Issues a `dcbt` prefetch for `src` on
/// the large-copy path (`0x80004390`). Performs no cache flush; the caller
/// ([`crate::boot::init_tables`]) does that after the copy.
///
/// # Safety
/// `dst..dst+len` must be writable and `src..src+len` readable. Overlap is
/// allowed. Unaligned addresses are handled.
///
/// # Confidence
/// high - branch shape, thresholds (`0x80`, `0x14`), alignment masks and block
/// sizes were checked against the instruction stream, and the result matches
/// the canonical CodeWarrior `memcpy`.
pub unsafe fn memcpy(dst: usize, src: usize, len: u32) {
    if len == 0 {
        return;
    }

    // observed: `cmplw cr1,r4,r3` at 0x8000436C; `blt` to the backward path
    // when `src < dst`. Equal pointers return immediately.
    if dst <= src {
        if dst == src {
            return;
        }

        if len < LARGE_COPY_THRESHOLD {
            // Short copies: only use blocks when the 4-byte alignments match.
            if len > ALIGNED_MIN_LEN && (dst & ALIGN_4_MASK) == (src & ALIGN_4_MASK) {
                // observed: `li r6,4; subf r9,r9,r6` -> 4 - (src & 3).
                // A 4-aligned source still copies a full 4-byte lead here.
                let lead = 4 - (src & ALIGN_4_MASK);
                let mut d = dst;
                let mut s = src;
                let mut left = len - lead as u32;
                let mut n = lead;
                loop {
                    platform::write_u8(d, platform::read_u8(s));
                    d += 1;
                    s += 1;
                    n -= 1;
                    if n == 0 {
                        break;
                    }
                }

                // observed: `rlwinm r6,r5,28,4,31` -> len >> 4.
                let mut blocks = left >> 4;
                loop {
                    copy_block_16_forward(d, s);
                    d += 16;
                    s += 16;
                    blocks -= 1;
                    if blocks == 0 {
                        break;
                    }
                }

                left &= 0x0F; // remaining bytes after the 16-byte blocks (len & 15)
                if left == 0 {
                    return;
                }
                copy_bytes_forward(d, s, left);
                return;
            }
        } else {
            // Large copies: align the destination to 8 bytes when the source
            // has the same misalignment, then move 32-byte blocks.
            let misalign = dst & ALIGN_8_MASK;
            let mut d = dst - misalign;
            let mut s = src;
            cache::dcbt(src);

            if misalign == (src & ALIGN_8_MASK) {
                let mut left = len;
                if misalign != 0 {
                    // observed: `li r6,8; subf r9,r9,r6` -> 8 - (src & 7).
                    let lead = 8 - (src & ALIGN_8_MASK);
                    left -= lead as u32;
                    d = dst;
                    let mut n = lead;
                    loop {
                        platform::write_u8(d, platform::read_u8(s));
                        d += 1;
                        s += 1;
                        n -= 1;
                        if n == 0 {
                            break;
                        }
                    }
                }

                // observed: `rlwinm r6,r5,27,5,31` -> len >> 5.
                let mut blocks = left >> 5;
                loop {
                    copy_block_32_forward(d, s);
                    d += 32;
                    s += 32;
                    blocks -= 1;
                    if blocks == 0 {
                        break;
                    }
                }

                left &= 0x1F; // remaining bytes after the 32-byte blocks (len & 31)
                if left == 0 {
                    return;
                }
                copy_bytes_forward(d, s, left);
                return;
            }
        }

        // Alignment did not match (or the length is too small): byte copy.
        copy_bytes_forward(dst, src, len);
        return;
    }

    // ---------------------------------------------------------------------
    // Backward copy: `dst > src`, so walk from the end to stay overlap-safe.
    // observed: `add r4,r4,r5` and `add r12,r3,r5` at 0x800044DC..0x800044E0.
    // ---------------------------------------------------------------------
    let dst_end = dst.wrapping_add(len as usize);
    let src_end = src.wrapping_add(len as usize);

    if len < LARGE_COPY_THRESHOLD {
        if len > ALIGNED_MIN_LEN && (dst_end & ALIGN_4_MASK) == (src_end & ALIGN_4_MASK) {
            // observed: `clrlwi r10,r12,30` - the lead is the end misalignment
            // itself, not `4 - misalignment` (unlike the forward path).
            let lead = dst_end & ALIGN_4_MASK;
            let mut d = dst_end;
            let mut s = src_end;
            let mut left = len - lead as u32;
            if lead != 0 {
                let mut n = lead;
                loop {
                    d -= 1;
                    s -= 1;
                    platform::write_u8(d, platform::read_u8(s));
                    n -= 1;
                    if n == 0 {
                        break;
                    }
                }
            }

            // observed: `rlwinm r6,r5,28,4,31` -> (len - lead) >> 4.
            let mut blocks = left >> 4;
            loop {
                copy_block_16_backward(d, s);
                d -= 16;
                s -= 16;
                blocks -= 1;
                if blocks == 0 {
                    break;
                }
            }

            left &= 0x0F; // remaining bytes after the 16-byte blocks (len & 15)
            if left == 0 {
                return;
            }
            copy_bytes_backward(d, s, left);
            return;
        }
    } else {
        let misalign = dst_end & ALIGN_8_MASK;
        if misalign == (src_end & ALIGN_8_MASK) {
            let mut d = dst_end;
            let mut s = src_end;
            let mut left = len - misalign as u32;
            if misalign != 0 {
                let mut n = misalign;
                loop {
                    d -= 1;
                    s -= 1;
                    platform::write_u8(d, platform::read_u8(s));
                    n -= 1;
                    if n == 0 {
                        break;
                    }
                }
            }

            // observed: `rlwinm r6,r5,27,5,31` -> (len - lead) >> 5.
            let mut blocks = left >> 5;
            loop {
                copy_block_32_backward(d, s);
                d -= 32;
                s -= 32;
                blocks -= 1;
                if blocks == 0 {
                    break;
                }
            }

            left &= 0x1F; // remaining bytes after the 32-byte blocks (len & 31)
            if left == 0 {
                return;
            }
            copy_bytes_backward(d, s, left);
            return;
        }
    }

    // observed: `mtspr ctr,r5` + `lbzu/stbu -1` at 0x800045EC..0x800045F8.
    copy_bytes_backward(dst_end, src_end, len);
}

// ---------------------------------------------------------------------------
// memset
// ---------------------------------------------------------------------------

/// Fills `len` bytes at `dst` with `value`, writing whole 32-bit words once the
/// length reaches 32 bytes.
///
/// # Original
/// - Ghidra: `FUN_80004600` at `0x80004600` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004600_FUN_80004600.c`
///
/// # Arguments
/// * `dst` - first byte to write.
/// * `value` - byte value; only the low 8 bits matter.
/// * `len` - byte count; `0` returns immediately.
///
/// # Returns
/// Nothing. Callers that need the `memset` ABI result use
/// [`memset_wrapper`].
///
/// # Side effects
/// Writes exactly `len` bytes starting at `dst`.
///
/// # Safety
/// `dst..dst+len` must be writable.
///
/// # Confidence
/// high - the byte-replication constant, the `0x20` threshold, the `~(dst-1)&3`
/// lead calculation and the `(len>>2)&7` word remainder were checked against
/// the instruction stream.
pub unsafe fn memset(dst: usize, value: u8, len: u32) {
    let byte = value;

    // observed: `addi r6,r3,-1` at 0x80004608; `r6` is the "last byte written"
    // pointer and is advanced by every store below.
    let mut last = dst.wrapping_sub(1);
    let mut left = len;

    // observed: `cmplwi r5,0x20; blt` at 0x80004600/0x8000460C.
    if left >= 0x20 {
        // observed: `nor r0,r6,r6` + `rlwinm r0,r0,0,30,31` = ~(dst-1) & 3:
        // the number of bytes needed to reach a 4-byte boundary.
        let align = (!last) & ALIGN_4_MASK;
        if align != 0 {
            left -= align as u32;
            let mut n = align;
            loop {
                n -= 1;
                last = last.wrapping_add(1);
                platform::write_u8(last, byte);
                if n == 0 {
                    break;
                }
            }
        }

        // observed: 0x8000462C..0x80004648. When the value is non-zero the
        // three `rlwinm` inserts plus two `or`s build 0xVVVVVVVV.
        let mut word = byte as u32;
        if word != 0 {
            word |= (byte as u32) << 8;
            word |= (byte as u32) << 16;
            word |= (byte as u32) << 24;
        }

        // observed: `addi r3,r6,-3` = one word before the next aligned word.
        let mut p = last.wrapping_sub(3);

        // 32-byte blocks: eight `stw`s, the last one updating `p`.
        // observed: `rlwinm r0,r5,27,5,31; ...; beq 0x80004680` - the loop is
        // pre-tested, so a zero count skips it (possible when the alignment
        // lead of a 32-byte request leaves fewer than 32 bytes).
        let mut blocks = left >> 5;
        while blocks != 0 {
            platform::write_u32(p.wrapping_add(4), word);
            platform::write_u32(p.wrapping_add(8), word);
            platform::write_u32(p.wrapping_add(12), word);
            platform::write_u32(p.wrapping_add(16), word);
            platform::write_u32(p.wrapping_add(20), word);
            platform::write_u32(p.wrapping_add(24), word);
            platform::write_u32(p.wrapping_add(28), word);
            p = p.wrapping_add(32);
            platform::write_u32(p, word);
            blocks -= 1;
        }

        // observed: `rlwinm r0,r5,30,29,31` = (len >> 2) & 7 trailing words.
        let mut extra_words = (left >> 2) & 0x7;
        while extra_words != 0 {
            p = p.wrapping_add(4);
            platform::write_u32(p, word);
            extra_words -= 1;
        }

        // observed: `addi r6,r3,3` and `rlwinm r5,r5,0,30,31` (len & 3).
        last = p.wrapping_add(3);
        left &= ALIGN_4_MASK as u32;
    }

    // observed: byte tail loop at 0x8000469C..0x800046AC.
    if left != 0 {
        loop {
            left -= 1;
            last = last.wrapping_add(1);
            platform::write_u8(last, byte);
            if left == 0 {
                break;
            }
        }
    }
}

/// `memset` with the public ABI: forwards to [`memset`] and returns `dst`.
///
/// # Original
/// - Ghidra: `FUN_800046b4` at `0x800046B4` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800046b4_FUN_800046b4.c`
///
/// # Arguments
/// * `dst` - destination byte address (returned unchanged).
/// * `value` - fill byte.
/// * `len` - byte count.
///
/// # Returns
/// `dst`, matching the C `memset` prototype.
///
/// # Side effects
/// Identical to [`memset`].
///
/// # Safety
/// Identical to [`memset`].
///
/// # Confidence
/// high. `// observed:` the wrapper saves `r3` in `r31`, calls `0x80004600`
/// with `r4`/`r5` untouched and moves `r31` back to `r3`. Ghidra shows a
/// one-argument function because it lost the pass-through arguments; the
/// call site in `init_tables` (`r3 = pair[0]`, `r4 = 0`, `r5 = pair[1]`)
/// confirms the three-argument form.
pub unsafe fn memset_wrapper(dst: usize, value: u8, len: u32) -> usize {
    memset(dst, value, len);
    dst
}
