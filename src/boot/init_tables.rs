//! Copy/clear pass over the linker-generated boot tables.
//!
//! # What this module covers
//!
//! | Address | Item | Role |
//! |---------|------|------|
//! | `0x80004250` | [`init_tables`] | walk two tables, `memcpy` + cache-flush, then zero BSS |
//!
//! The two tables live in `.text` at the end of the first text section:
//!
//! * `0x80006618` - array of 12-byte triples `{ src, dst, size }`. The walk
//!   stops at the first triple whose `size` is zero. When `dst != src` the
//!   block is copied and the destination range is flushed from the caches.
//! * `0x8000669C` - array of 8-byte pairs `{ dst, size }`. The walk stops at
//!   the first pair whose `size` is zero. Each pair becomes
//!   `memset(dst, 0, size)`.
//!
//! # Why it exists
//!
//! The DOL image is loaded by the Wii loader but CodeWarrior splits the image
//! so that some initialized data is stored at a different address than it runs
//! at, and BSS is not part of the file at all. This function performs that
//! final placement before any C code (or the OS) runs. (DOL is the Wii's
//! executable container; BSS is the zero-initialized data segment.)
//!
//! # Start here
//!
//! [`init_tables`]; it is called once from [`crate::boot::entry::entry`].
//!
//! # Dependencies
//!
//! * [`crate::platform`] - reads the tables from raw memory.
//! * [`crate::boot::mem`] - `memcpy` and the `memset` wrapper.
//! * [`crate::boot::cache`] - instruction/data cache flush after each copy.
//!
//! # Evidence
//!
//! `work/ghidra/main-gb/export/80004250_FUN_80004250.c` and the DOL bytes
//! `0x80004250..0x800042F8`. The Ghidra C prints the second loop's call as
//! `FUN_800046b4(*ppuVar1,0)`; the instruction stream shows the third argument
//! in `r5` (`lwz r5,4(r29)`), i.e. the pair's size.

use crate::boot::entry::{SDA1_BASE, SDA2_BASE};
use crate::boot::{cache, mem};
use crate::platform;

/// Start of the `{ src, dst, size }` triple table.
///
/// observed: `lis r29,0x8000 / addi r29,r29,0x6618` at `0x80004268`.
pub const RELOCATION_TRIPLETS: usize = 0x8000_6618;

/// Start of the `{ dst, size }` zero-fill pair table.
///
/// observed: `lis r29,0x8000 / addi r29,r29,0x669C` at `0x800042B0`.
pub const ZERO_PAIRS: usize = 0x8000_669C;

/// Size in bytes of one `{ src, dst, size }` triple.
const TRIPLET_SIZE: usize = 12;

/// Offset of `size` inside a relocation triple.
const TRIPLET_SIZE_OFFSET: usize = 8;

/// Size in bytes of one `{ dst, size }` zero-fill pair.
const PAIR_SIZE: usize = 8;

/// Offset of `size` inside a zero-fill pair.
const PAIR_SIZE_OFFSET: usize = 4;

/// Relocates the initialized-data blocks and zeroes the BSS blocks listed in
/// the linker tables.
///
/// # Original
/// - Ghidra: `FUN_80004250` at `0x80004250` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004250_FUN_80004250.c`
///
/// # Arguments
/// None. Both tables are at fixed link-time addresses.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * copies each triple with `dst != src` and calls
///   [`crate::boot::cache::flush_cache`] over the destination
/// * zero-fills every pair in the second table
///
/// # Safety
/// Both tables must be readable at their link-time addresses, and every
/// destination range must be writable MEM1 (the Wii's 24 MiB main RAM). Intended
/// to run once, before interrupts and before any other code uses the
/// destination ranges.
///
/// # Confidence
/// high - loop order, termination condition (`size == 0`), the `dst != src`
/// guard and the cache flush were checked against the instruction stream. The
/// redundant `size != 0` test in the second loop is preserved because it exists
/// as a branch in the original (see the comment below).
pub unsafe fn init_tables() {
    // --- initialized-data blocks -----------------------------------------
    let mut triple = RELOCATION_TRIPLETS;
    loop {
        // observed: `lwz r30,8(r29); cmpwi r30,0; beq` at 0x80004270..0x80004278.
        let size = platform::read_u32(triple + TRIPLET_SIZE_OFFSET);
        if size == 0 {
            break;
        }
        let src = platform::read_u32(triple);
        let dst = platform::read_u32(triple + 4);
        // observed: `cmplw r31,r4; beq` at 0x80004288..0x8000428C - a block that
        // is already in place is not copied and not flushed.
        if dst != src {
            mem::memcpy(dst as usize, src as usize, size);
            // observed: `mr r3,r31; mr r4,r30; bl 0x80004330` - flush the
            // destination range with the same size as the copy.
            cache::flush_cache(dst, size as i32);
        }
        triple += TRIPLET_SIZE;
    }

    // --- BSS blocks -------------------------------------------------------
    let mut pair = ZERO_PAIRS;
    loop {
        // observed: `lwz r5,4(r29); cmpwi r5,0; beq` at 0x800042B8..0x800042C0.
        let size = platform::read_u32(pair + PAIR_SIZE_OFFSET);
        if size == 0 {
            break;
        }
        let dst = platform::read_u32(pair);
        // observed: `lwz r3,0(r29); li r4,0; bl 0x800046B4` at
        // 0x800042C4..0x800042D0. The `beq` at 0x800042C8 re-tests the CR0
        // left by the loop condition, so it can never be taken; it is the
        // compiler's copy of the source-level `if (size != 0)`. Kept for
        // control-flow fidelity.
        if size != 0 {
            mem::memset_wrapper(dst as usize, 0, size);
        }
        pair += PAIR_SIZE;
    }
}

// ===========================================================================
// Wave 229 lane B: the first fourteen entries of the `.ctors` table
// ===========================================================================
//
// The constructor table starts at `0x802ED9E0` and holds 181 code pointers
// (`0x802ED9E0..0x802EDCB4`), followed by three zero words at `0x802EDCB4`,
// `0x802EDCB8` and `0x802EDCBC`. `run_init_table` in
// [`crate::runtime::os`] walks it in table order and calls each entry with no
// arguments.
//
// The first fourteen bodies by address are:
//
// | slot | table word  | target       | extent | Rust body |
// |-----:|-------------|--------------|-------:|-----------|
// | 1 | `0x802ED9E4` | `0x8000D720` | 32 B | [`fn_8000d720`] |
// | 2 | `0x802ED9E8` | `0x800129A0` | 6,204 B | [`crate::ui::hud::fn_800129a0`] (already translated) |
// | 3 | `0x802ED9EC` | `0x80016A80` | 1,012 B | [`crate::ui::hud::fn_80016a80`] (already translated) |
// | 4 | `0x802ED9F0` | `0x80018330` | 2,168 B | [`crate::ui::hud::fn_80018330`] (already translated) |
// | 5 | `0x802ED9F4` | `0x80019560` | 824 B | [`crate::ui::hud::fn_80019560`] (already translated) |
// | 6 | `0x802ED9F8` | `0x8001A280` | 824 B | [`crate::ui::hud::fn_8001a280`] (already translated) |
// | 7 | `0x802ED9FC` | `0x8001AF70` | 824 B | [`crate::ui::hud::fn_8001af70`] (already translated) |
// | 8 | `0x802EDA00` | `0x8001D2C0` | 964 B | [`fn_8001d2c0`] |
// | 9 | `0x802EDA04` | `0x8001EC70` | 1,704 B | [`fn_8001ec70`] |
// | 10 | `0x802EDA08` | `0x800207A0` | 88 B | [`fn_800207a0`] |
// | 11 | `0x802EDA0C` | `0x800233B0` | 1,296 B | [`fn_800233b0`] |
// | 12 | `0x802EDA10` | `0x80026140` | 1,704 B | [`fn_80026140`] |
// | 13 | `0x802EDA14` | `0x80026E20` | 292 B | [`fn_80026e20`] |
// | 14 | `0x802EDA18` | `0x80028150` | 112 B | [`fn_80028150`] |
//
// Slots 2-7 already had translated bodies in `src/ui/hud.rs` when this lane
// ran, so they are referenced rather than duplicated. The remaining eight
// (6,192 B) are new and are the bodies below. Slot 0 is `0x802DD2E0`, which
// sorts after every other pointer, so it is outside this "lowest addresses"
// batch; slots are the table indices, not rank in address order.
//
// Extents are prologue-to-`blr` inclusive, measured from the raw words in
// `work/wave229/laneB_disasm.txt`. They match the `functions.tsv` sizes for the
// six tabled entries (`6,204`, `1,012`, `2,168`, `824`, `824`, `824`), which is
// a good cross-check on the measurement.

/// `0x8015F7C0`: initialises an object header and stores its state name.
///
/// Every descriptor builder below calls it with `(state, name)` before
/// installing the vtable and payload.
const FN_8015F7C0: usize = 0x8015_F7C0;

/// `0x802DC780`: pushes a three-word node onto the SDA-linked registry.
///
/// observed signature `(value, tag, node)` - stores the old SDA head in
/// `node[0]`, `tag` in `node[1]`, `value` in `node[2]`, then makes `node` the
/// new head (see `work/ghidra/main-gb/export/802dc780_FUN_802dc780.c`).
const FN_802DC780: usize = 0x802D_C780;

/// `0x802DC9A0`: runs a constructor over a `count`-element array, invoking the
/// destructor backwards if a constructor fails (`param_5` is the count, `param_4`
/// the stride).
const FN_802DC9A0: usize = 0x802D_C9A0;

/// Shared `dSys`-style type word returned by the twelve-byte accessor
/// functions in this family.
const W229B_TAG_SHARED: u32 = 0x8037_7A04;

/// Builds one static state descriptor with a straight nine-word payload copy.
///
/// observed: `bl 0x8015F7C0` then nine `lwz`/`stw` pairs in ascending order
/// (`data + 0x00..0x20` -> `state + 0x0C..0x2C`), the vtable store at `state`,
/// the type/tag word at `state + 0x30`, and finally
/// `bl 0x802DC780(state, release, event)`.
///
/// # Arguments
/// - `state`: writable descriptor base (16-byte aligned in every caller).
/// - `name`: state-name string.
/// - `data`: nine readable payload words.
/// - `event`: registry node written by `0x802DC780`.
/// - `vtable`: word installed at `state + 0`.
/// - `tag`: word installed at `state + 0x30`.
/// - `release`: word installed at `event + 4`.
///
/// # Safety
/// `state` must be writable through `+0x30`, `data` readable through `+0x20`,
/// and `event` writable through `+8`.
///
/// # Confidence
/// high - the copy order, vtable offset, tag offset and registry call are
/// identical in all twenty-seven descriptor blocks below and in the wave-113
/// `hud` builders.
unsafe fn w229b_build_state(
    state: usize,
    name: usize,
    data: usize,
    event: usize,
    vtable: u32,
    tag: u32,
    release: u32,
) {
    platform::call2(FN_8015F7C0, state as u32, name as u32);
    platform::write_u32(state, vtable);
    let mut index = 0usize;
    while index < 9 {
        platform::write_u32(
            state + 0x0C + index * 4,
            platform::read_u32(data + index * 4),
        );
        index += 1;
    }
    platform::write_u32(state + 0x30, tag);
    platform::call3(FN_802DC780, state as u32, release, event as u32);
}

/// Initialises the head of the doubly-linked list at `0x80351680`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8000D720`; it is a leaf
///   constructor reached only through the `.ctors` table.
/// - Extent: `0x8000D720..0x8000D740` (32 bytes). Three `lis`/`li`/`addi`
///   setup instructions, three `stw` and `blr`.
/// - Table: slot 1, word `0x802ED9E4`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets the list sentinel at `0x80351680` to point at the node word
/// `0x80351674`, and clears `0x80351688`.
///
/// # Safety
/// The three words at `0x80351674`, `0x80351680` and `0x80351684`/`0x80351688`
/// must be writable.
///
/// # Confidence
/// high - all three stores and both constants are literal.
pub unsafe fn fn_8000d720() {
    // observed: r3 = 0x80351680 - 0x0C = 0x80351674.
    const HEAD: usize = 0x8035_1680;
    const NODE: usize = 0x8035_1674;
    platform::write_u32(HEAD + 4, NODE as u32);
    platform::write_u32(HEAD, NODE as u32);
    platform::write_u32(HEAD + 8, 0);
}

/// Builds four `daBossDemo_c` state descriptors.
///
/// The preamble copies one SDA word and three SDA floats into the source block
/// at `0x803014E0`, registers the block's header node, then builds the four
/// descriptors. Each descriptor takes a straight nine-word payload from a
/// `0x24`-byte source record, installs vtable `0x80301754`, stores the shared
/// type word at `+0x30` and registers itself with `0x8002D710`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8001D2C0`.
/// - Extent: `0x8001D2C0..0x8001D684` (964 bytes), ending at the `blr`;
///   three zero words follow.
/// - Table: slot 8, word `0x802EDA00`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the source block at `0x803014E0` and the four descriptors at
/// `0x80352670`, `0x803526B0`, `0x803526F0`, `0x80352730`, and links their
/// `0x80352664`/`0x803526A4`/`0x803526E4`/`0x80352724` nodes.
///
/// # Safety
/// All those regions must be writable and the SDA words readable. Intended to
/// run once from the constructor table.
///
/// # Confidence
/// medium-high - every call, base and copy is literal. The release word
/// `0x8002D710` is observed but lands mid-body in the untabled region around
/// `0x8002D5E0` (no `functions.tsv` row starts there), so its role is recorded
/// only as "the word the registry stores", not as a verified callback.
pub unsafe fn fn_8001d2c0() {
    const SRC: usize = 0x8030_14E0;
    const REG: usize = 0x8035_2658;
    const STATE_BASE: usize = 0x8035_2670;
    const VTABLE: u32 = 0x8030_1754;
    const RELEASE: u32 = 0x8002_D710;

    // observed: preamble copies one SDA word and three SDA floats into the
    // source header at 0x803014E0 (`lfs`/`stfs` pairs at 0x8001D2D8..0x8001D320).
    platform::write_u32(SRC + 0x0C, platform::read_u32(SDA2_BASE - 0x6114));
    platform::write_f32(SRC + 0x18, platform::read_f32(SDA2_BASE - 0x7E78));
    platform::write_f32(SRC + 0x1C, platform::read_f32(SDA2_BASE - 0x7E74));
    platform::write_f32(SRC + 0x20, platform::read_f32(SDA2_BASE - 0x7E70));
    platform::call3(FN_802DC780, 0, 0x8002_D690u32, REG as u32);

    // observed: four records at +0x18 stride 0x40; each name is one of the
    // four strings at 0x803016D0/+0x1C/+0x24/+0x20 while the payload records
    // are contiguous 0x24-byte blocks from 0x80301640.
    const RECORDS: [(usize, usize, usize); 4] = [
        (0x8030_16D0, 0x8030_1640, 0x8035_2664),
        (0x8030_16EC, 0x8030_1664, 0x8035_26A4),
        (0x8030_1710, 0x8030_1688, 0x8035_26E4),
        (0x8030_1730, 0x8030_16AC, 0x8035_2724),
    ];
    let mut index = 0usize;
    while index < RECORDS.len() {
        let (name, data, event) = RECORDS[index];
        let state = STATE_BASE + index * 0x40;
        w229b_build_state(
            state,
            name,
            data,
            event,
            VTABLE,
            W229B_TAG_SHARED,
            RELEASE,
        );
        index += 1;
    }
}

/// Builds eight `daBullet_c` state descriptors.
///
/// Same descriptor shape as [`fn_8001d2c0`], with the preamble absent. The
/// eight source records start at `0x80301948` (`0x24`-byte stride), the
/// descriptors at `0x80352768` (`0x40`-byte stride, state at node `+0x0C`),
/// vtable `0x80301B5C`, and the no-op release word `0x8002F380`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8001EC70`.
/// - Extent: `0x8001EC70..0x8001F318` (1,704 bytes), ending at the `blr`;
///   two zero words follow.
/// - Table: slot 9, word `0x802EDA04`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight descriptors at `0x80352774` upward and their registry nodes.
///
/// # Safety
/// The descriptor and registry ranges must be writable, and the source records
/// readable. Intended to run once from the constructor table.
///
/// # Confidence
/// high - all bases are literal and every record uses the same block shape;
/// only the release word is mechanically named.
pub unsafe fn fn_8001ec70() {
    const STATE_BASE: usize = 0x8035_2768;
    const VTABLE: u32 = 0x8030_1B5C;
    const RELEASE: u32 = 0x8002_F380;

    const RECORDS: [(usize, usize, usize); 8] = [
        (0x8030_1A68, 0x8030_1948, 0x8035_2768),
        (0x8030_1A84, 0x8030_196C, 0x8035_27A8),
        (0x8030_1AA0, 0x8030_1990, 0x8035_27E8),
        (0x8030_1AC0, 0x8030_19B4, 0x8035_2828),
        (0x8030_1ADC, 0x8030_19D8, 0x8035_2868),
        (0x8030_1AFC, 0x8030_19FC, 0x8035_28A8),
        (0x8030_1B18, 0x8030_1A20, 0x8035_28E8),
        (0x8030_1B38, 0x8030_1A44, 0x8035_2928),
    ];
    let mut index = 0usize;
    while index < RECORDS.len() {
        let (name, data, event) = RECORDS[index];
        w229b_build_state(
            STATE_BASE + index * 0x40 + 0x0C,
            name,
            data,
            event,
            VTABLE,
            W229B_TAG_SHARED,
            RELEASE,
        );
        index += 1;
    }
}

/// Constructs three 60-byte objects and registers their cleanup routine.
///
/// Runs the `0x802DC9A0` constructor loop over the three-element array at
/// `0x80352974` with per-element constructor `0x80020800` and destructor
/// `0x80020850`, then pushes the registry node at `0x80352968` with cleanup
/// word `0x80020890`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800207A0`.
/// - Extent: `0x800207A0..0x800207F8` (88 bytes); two zero words follow.
/// - Table: slot 10, word `0x802EDA08`.
/// - Evidence: `work/wave229/laneB_disasm.txt`; `0x802DC9A0` is the loop
///   printed in `work/ghidra/main-gb/export/802dc9a0_FUN_802dc9a0.c`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Initialises the three objects at `0x80352974 + i * 60` and links their
/// registry node.
///
/// # Safety
/// The object array must be writable for 180 bytes and the registry node
/// writable through `+8`.
///
/// # Confidence
/// high - the five call arguments are literal (`0x80352974`, `0x80020800`,
/// `0x80020850`, `60`, `3`), and so are the registry arguments.
pub unsafe fn fn_800207a0() {
    // observed: r3..r7 = array, ctor, dtor, stride, count.
    platform::call5(
        FN_802DC9A0,
        0x8035_2974,
        0x8002_0800,
        0x8002_0850,
        60,
        3,
    );
    // observed: register(0, cleanup, 0x80352968).
    platform::call3(FN_802DC780, 0, 0x8002_0890, 0x8035_2968);
}

/// Builds six `daBullet_c` state descriptors.
///
/// Six straight-copy descriptors from `0x80302194` upward, installed at
/// `0x80352A34` (`0x40` stride), with vtable `0x80302358` and release word
/// `0x80023920`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800233B0`.
/// - Extent: `0x800233B0..0x800238C0` (1,296 bytes); one zero word follows.
/// - Table: slot 11, word `0x802EDA0C`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes six descriptors and their registry nodes.
///
/// # Safety
/// The descriptor and registry ranges must be writable, and the source records
/// readable.
///
/// # Confidence
/// high - all bases are literal and the six blocks are mechanically identical.
pub unsafe fn fn_800233b0() {
    const STATE_BASE: usize = 0x8035_2A28;
    const VTABLE: u32 = 0x8030_2358;
    const RELEASE: u32 = 0x8002_3920;

    const RECORDS: [(usize, usize, usize); 6] = [
        (0x8030_2270, 0x8030_2194, 0x8035_2A28),
        (0x8030_2290, 0x8030_21B8, 0x8035_2A68),
        (0x8030_22B4, 0x8030_21DC, 0x8035_2AA8),
        (0x8030_22DC, 0x8030_2200, 0x8035_2AE8),
        (0x8030_2304, 0x8030_2224, 0x8035_2B28),
        (0x8030_232C, 0x8030_2248, 0x8035_2B68),
    ];
    let mut index = 0usize;
    while index < RECORDS.len() {
        let (name, data, event) = RECORDS[index];
        w229b_build_state(
            STATE_BASE + index * 0x40 + 0x0C,
            name,
            data,
            event,
            VTABLE,
            W229B_TAG_SHARED,
            RELEASE,
        );
        index += 1;
    }
}

/// Builds eight `daBullet_c` state descriptors with two type-word variants.
///
/// The first six blocks use the shared `0x80377A04` tag. The seventh and
/// eighth take `0x80357BB4` and `0x80357BF4` from their accessors `0x800268C0`
/// and `0x800268B0`; the order is intentionally reversed relative to the
/// address order of the accessors.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80026140`.
/// - Extent: `0x80026140..0x800267E8` (1,704 bytes); two zero words follow.
/// - Table: slot 12, word `0x802EDA10`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight descriptors at `0x80352BB4` upward and their registry nodes.
///
/// # Safety
/// The descriptor and registry ranges must be writable, and the source records
/// readable.
///
/// # Confidence
/// high - all bases and both divergent tag words are literal.
pub unsafe fn fn_80026140() {
    const STATE_BASE: usize = 0x8035_2BA8;
    const VTABLE: u32 = 0x8030_2978;
    const RELEASE: u32 = 0x8002_6850;
    const TAG_A: u32 = 0x8037_7BB4;
    const TAG_B: u32 = 0x8037_7BF4;

    const RECORDS: [(usize, usize, usize, u32); 8] = [
        (0x8030_2870, 0x8030_2750, 0x8035_2BA8, W229B_TAG_SHARED),
        (0x8030_2890, 0x8030_2774, 0x8035_2BE8, W229B_TAG_SHARED),
        (0x8030_28B0, 0x8030_2798, 0x8035_2C28, W229B_TAG_SHARED),
        (0x8030_28D0, 0x8030_27BC, 0x8035_2C68, W229B_TAG_SHARED),
        (0x8030_28F0, 0x8030_27E0, 0x8035_2CA8, W229B_TAG_SHARED),
        (0x8030_2910, 0x8030_2804, 0x8035_2CE8, W229B_TAG_SHARED),
        (0x8030_2938, 0x8030_2828, 0x8035_2D28, TAG_A),
        (0x8030_2958, 0x8030_284C, 0x8035_2D68, TAG_B),
    ];
    let mut index = 0usize;
    while index < RECORDS.len() {
        let (name, data, event, tag) = RECORDS[index];
        w229b_build_state(
            STATE_BASE + index * 0x40 + 0x0C,
            name,
            data,
            event,
            VTABLE,
            tag,
            RELEASE,
        );
        index += 1;
    }
}

/// Builds one state descriptor with vtable `0x80302CAC`.
///
/// A single straight-copy block: nine words from `0x80302C6C`, state at
/// `0x80352DB4`, name `0x80302C90`, shared tag, release word `0x80026FB0` and
/// registry node `0x80352DA8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80026E20`.
/// - Extent: `0x80026E20..0x80026F44` (292 bytes); three zero words follow.
/// - Table: slot 13, word `0x802EDA14`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the descriptor and its registry node.
///
/// # Safety
/// `0x80352DA8..0x80352DE8` and the source record must be accessible.
///
/// # Confidence
/// high - the single block's bases are all literal.
pub unsafe fn fn_80026e20() {
    w229b_build_state(
        0x8035_2DB4,
        0x8030_2C90,
        0x8030_2C6C,
        0x8035_2DA8,
        0x8030_2CAC,
        W229B_TAG_SHARED,
        0x8002_6FB0,
    );
}

/// Writes the two four-float vectors at `0x80352DE8` and `0x80352E04`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80028150`; it is a leaf
///   constructor with no stack frame beyond the 48-byte save area.
/// - Extent: `0x80028150..0x800281C0` (112 bytes), ending at the `blr`.
/// - Table: slot 14, word `0x802EDA18`.
/// - Evidence: `work/wave229/laneB_disasm.txt`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x80352DE8 = {f4, f3, f3, f2}` and `0x80352E04 = {f4, f1, f3, f0}` from
/// the five SDA floats at `SDA2 - 0x7CF8`, `-0x7D1C`, `-0x7CEC`, `-0x7CE8`
/// and `-0x7D0C`.
///
/// # Safety
/// The eight destination words must be writable and the SDA floats readable.
///
/// # Confidence
/// medium-high - the five loads and eight stores are literal; the SDA floats
/// live in the zero-filled tail of `main.elf`'s data segment, so their values
/// are established at run time and are deliberately not baked in.
pub unsafe fn fn_80028150() {
    let f3 = platform::read_f32(SDA2_BASE - 0x7D1C);
    let f4 = platform::read_f32(SDA2_BASE - 0x7CF8);
    let f2 = platform::read_f32(SDA2_BASE - 0x7CEC);
    let f1 = platform::read_f32(SDA2_BASE - 0x7CE8);
    let f0 = platform::read_f32(SDA2_BASE - 0x7D0C);

    const VEC_A: usize = 0x8035_2DE8;
    const VEC_B: usize = 0x8035_2E04;
    platform::write_f32(VEC_A, f4);
    platform::write_f32(VEC_A + 4, f3);
    platform::write_f32(VEC_A + 8, f3);
    platform::write_f32(VEC_A + 0x0C, f2);
    platform::write_f32(VEC_B, f4);
    platform::write_f32(VEC_B + 4, f1);
    platform::write_f32(VEC_B + 8, f3);
    platform::write_f32(VEC_B + 0x0C, f0);
}

// ===========================================================================
// Wave 229, lane C - `.ctors` slots 15..28: actor state-descriptor installers
// ===========================================================================
//
// `0x802ED9E0` is the static-constructor table (`.ctors`) of `main.dol`: 181
// function pointers, `0x802ED9E0..0x802EDCB4`, followed by three zero words
// that terminate it. Its walker is
// [`crate::runtime::os::run_init_table`] (`0x801B8990`), called once from
// `0x801B8970`; it reads a word, stops on zero and otherwise calls it, so every
// body below runs exactly once, in table order, during static initialisation.
//
// Every one of these constructors installs the state-name descriptors of one
// actor or map object, i.e. it is the `daEn*_c`/`daLift*_c` half of the
// `dScene` state machinery. The bodies are the same roughly fifty-instruction
// template repeated once per state:
//
// 1. read the state's nine parameter words from a `.data` block;
// 2. call a one-instruction getter that returns the shared state object;
// 3. `0x8015F7C0(descriptor, name)` sets the name and the name index;
// 4. store the nine words at `descriptor + 0x0C..+0x2C`;
// 5. store the class vtable at `descriptor + 0x00`;
// 6. store the state object at `descriptor + 0x30` (only where the class has
//    one);
// 7. `0x802DC780(descriptor, destructor, node)` pushes the 12-byte node that
//    precedes the descriptor onto the global destructor list.
//
// The nine-word copy is not linear: the compiler emitted the three
// twelve-byte groups last-group-first, so destination word `k` takes source
// word `W229_STATE_PERMUTATION[k]`. That permutation is identical in all
// bodies here and in the wave-92 body [`crate::game::player::fn_800414e0`].
//
// ## Slot map (slots 15..28 of `0x802ED9E0`)
//
// | slot | table word | target | class | owner |
// |---:|---|---|---|---|
// | 15 | `0x802EDA1C` | `0x8002A230` | `daEnDfpakkun_c` | **lane C**, below |
// | 16 | `0x802EDA20` | `0x8002B8C0` | `daEnDoor_c` | **lane C**, below |
// | 17 | `0x802EDA24` | `0x8002C410` | `daEnDpakkun_c` | **lane C**, below |
// | 18 | `0x802EDA28` | `0x8002E660` | `daEnDpakkunBase_c` | **lane C**, below |
// | 19 | `0x802EDA2C` | `0x80030AB0` | `daEnJimenPakkunBase_c` | **lane C**, below |
// | 20 | `0x802EDA30` | `0x80032AB0` | `daEnKuriboBase_c` | **lane C**, below |
// | 21 | `0x802EDA34` | `0x80034E10` | `daEnLkuriboBase_c` | **lane C**, below |
// | 22 | `0x802EDA38` | `0x80036390` | `daEnNetNoko_c` | **lane C**, below |
// | 23 | `0x802EDA3C` | `0x80037750` | `daEnObjCoinBlock_c` | `fn_80037750` in `src/ui/hud.rs` (already translated) |
// | 24 | `0x802EDA40` | `0x8003C050` | `daEnShell_c` | `fn_8003c050` in `src/ui/hud.rs` (already translated) |
// | 25 | `0x802EDA44` | `0x8003D4B0` | `daEnSuperBigPile_c` | **lane C**, below |
// | 26 | `0x802EDA48` | `0x8003FE10` | `daEnTogezoBase_c` | **lane C**, below |
// | 27 | `0x802EDA4C` | `0x800414E0` | `daFireBall_Base_c` | `fn_800414e0` in `src/game/player.rs` (already translated) |
// | 28 | `0x802EDA50` | `0x80042F20` | `daLiftDownOnBase_c` | **lane C**, below |
//
// ## Record layout
//
// Every record is `0x40` bytes and starts with the registration node. In
// `daEnDfpakkun_c` and the other `..Base_c` classes the descriptor begins at
// `node + 0x0C`, directly after the three node words. `daEnDoor_c`,
// `daEnObjCoinBlock_c` and the first five `daEnTogezoBase_c` records align the
// next object to sixteen bytes instead: `node + 0x0C` is not written by these
// bodies and the descriptor begins at `node + 0x10`. Those same classes also
// leave `descriptor + 0x30` alone, which is why `handler` is optional below.
//
// Slots 1..14 are lane B's half of the same table and live in the section
// above; this section continues at slot 15.
//
// Slots 23, 24 and 27 have `functions.tsv` rows and were translated in earlier
// waves, so lane C did not duplicate them; the eleven bodies below are the
// untabled remainder of the run. Their ordering is one body per class, and
// none of them is shared with another table.
//
// ## Measured extents (all read from `work/dol/main.elf`)
//
// | slot | code extent | bytes | records |
// |---:|---|---:|---:|
// | 15 | `0x8002A230..0x8002A740` | 1296 | 6 |
// | 16 | `0x8002B8C0..0x8002BCB4` | 1012 | 5 |
// | 17 | `0x8002C410..0x8002C788` | 888 | 4 |
// | 18 | `0x8002E660..0x8002EBA4` | 1348 | 6 |
// | 19 | `0x80030AB0..0x80030E28` | 888 | 4 |
// | 20 | `0x80032AB0..0x80032E28` | 888 | 4 |
// | 21 | `0x80034E10..0x80035320` | 1296 | 6 |
// | 22 | `0x80036390..0x80036570` | 480 | 2 |
// | 23 | `0x80037750..0x80037D18` | 1480 | 7 |
// | 24 | `0x8003C050..0x8003C62C` | 1500 | 7 |
// | 25 | `0x8003D4B0..0x8003D828` | 888 | 4 |
// | 26 | `0x8003FE10..0x80040540` | 1840 | 9 |
// | 27 | `0x800414E0..0x80041858` | 888 | 4 |
// | 28 | `0x80042F20..0x80043430` | 1296 | 6 |
//
// Each extent runs from the entry point to the closing `bclr 20,0`; the last
// word of every extent is that `bclr`, so no padding is included.
// Total: 12120 bytes in 11 bodies.
//
// **Why they are here.** `.ctors` runs before any subsystem is up, so the
// bodies belong with the boot module rather than with the actor modules that
// own the classes; `src/boot/init_tables.rs` already owns the two link-time
// placement tables that the same startup path walks.
//
// **Dependencies.** [`platform::read_u32`]/[`platform::write_u32`], the
// indirect-call helpers, and the permutation below. The bodies call the name
// setter `0x8015F7C0`, the registration push `0x802DC780` and the per-state
// getters through their absolute addresses, exactly as the reconstructed code
// does everywhere else.
// ---------------------------------------------------------------------------

// The two callees `0x8015F7C0` and `0x802DC780` are already named by lane B
// above as [`FN_8015F7C0`] and [`FN_802DC780`]; this section reuses those
// constants instead of declaring a second copy. `0x8015F7C0(descriptor,
// name)` stores the shared name-header vtable `0x80328A18` at `+0x00`, the
// `name` at `+0x04` and a monotonic name index (zero for a null name) at
// `+0x08`. `0x802DC780(descriptor, destructor, node)` sets `node[0] =
// r13 - 0x46A8`, `node[1] = destructor`, `node[2] = descriptor` and makes
// `node` the new head; the exit path walks that list backwards, so the last
// constructor registered runs its destructor first.

/// Destination word `k` of the nine-word state block takes source word `k`
/// permuted by this table.
///
/// # Original
/// The compiler copied three twelve-byte groups in reverse order:
/// `+0x0C..+0x14` receives source `+0x18..+0x20`, `+0x18..+0x20` receives
/// `+0x0C..+0x14` and `+0x24..+0x2C` receives `+0x00..+0x08`. Every `.ctors`
/// body in this section uses the same order, and so does
/// [`crate::game::player::fn_800414e0`].
///
/// # Confidence
/// high - identical in all eleven bodies and in the wave-92 body.
const W229_STATE_PERMUTATION: [usize; 9] = [6, 7, 8, 3, 4, 5, 0, 1, 2];

/// Calls a no-argument getter and returns its `r3`.
///
/// # Original
/// Host-side helper; the getters are three-instruction leaves
/// (`lis`/`addi`/`bclr`) that publish a static state object.
///
/// # Safety
/// `addr` must be the entry of a function that returns in `r3`.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn w229_call0_u32(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Installs one state descriptor: name, nine parameter words, vtable, state
/// object and destructor registration.
///
/// # Original
/// Host-side factoring of the repeated block in the `.ctors` bodies below; it
/// has no binary address of its own. The instruction-level order of the
/// original is preserved: the nine source words are read first, the state
/// getter runs before the name setter, and the nine-word copy precedes the
/// vtable store.
///
/// # Arguments
/// * `node` - the 12-byte registration node that precedes `descriptor`.
/// * `descriptor` - the state descriptor to fill.
/// * `name` - the `StateID_*` string address.
/// * `source` - the nine-word parameter block.
/// * `vtable` - the class state vtable stored at `descriptor + 0x00`.
/// * `destructor` - the callback registered with the descriptor.
/// * `handler` - the getter that publishes the state object for
///   `descriptor + 0x30`, or `None` when the class leaves that word alone.
///
/// # Safety
/// `descriptor` must be writable for `0x34` bytes, `node` for 12, `source` must
/// be readable for 36 and `handler`, when present, must be a valid getter.
///
/// # Confidence
/// high - one-for-one transcription of the repeated block.
#[inline]
unsafe fn w229_install_state(
    node: usize,
    descriptor: usize,
    name: usize,
    source: usize,
    vtable: u32,
    destructor: usize,
    handler: Option<usize>,
) {
    // observed: all nine source words are loaded before the first call.
    let mut words = [0u32; 9];
    let mut word = 0;
    while word < 9 {
        words[word] = platform::read_u32(source + word * 4);
        word += 1;
    }

    // observed: `bl <getter>` follows the nine loads and precedes the name
    // setter, so the state object exists before the descriptor is named.
    let handler = match handler {
        Some(getter) => Some(w229_call0_u32(getter) as usize),
        None => None,
    };

    platform::call2(FN_8015F7C0, descriptor as u32, name as u32);
    // observed: three 12-byte groups copied last group first.
    let mut slot = 0;
    while slot < 9 {
        platform::write_u32(
            descriptor + 0x0C + slot * 4,
            words[W229_STATE_PERMUTATION[slot]],
        );
        slot += 1;
    }
    platform::write_u32(descriptor, vtable);
    if let Some(handler) = handler {
        platform::write_u32(descriptor + 0x30, handler as u32);
    }
    platform::call3(
        FN_802DC780,
        descriptor as u32,
        destructor as u32,
        node as u32,
    );
}

/// Installs the six `daEnDfpakkun_c` names (a Df-style Pakkun enemy).
///
/// This is the static constructor of `daEnDfpakkun_c`: the CRT calls it from
/// `.ctors` before `main`, so the six descriptor records of the class are in
/// place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Wait` | `daEnDfpakkun_c::StateID_Wait` | `0x80352E2C` | `0x80303438` |
/// | `StateID_Appear` | `daEnDfpakkun_c::StateID_Appear` | `0x80352E6C` | `0x8030345C` |
/// | `StateID_Attack` | `daEnDfpakkun_c::StateID_Attack` | `0x80352EAC` | `0x80303480` |
/// | `StateID_Disappear` | `daEnDfpakkun_c::StateID_Disappear` | `0x80352EEC` | `0x803034A4` |
/// | `StateID_DieVanish` | `daEnDfpakkun_c::StateID_DieVanish` | `0x80352F2C` | `0x803034C8` |
/// | `StateID_DieIceBreak` | `daEnDfpakkun_c::StateID_DieIceBreak` | `0x80352F6C` | `0x803034EC` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8002A230` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x8002A230..0x8002A740` (1296 bytes); the
///   closing `bclr 20,0` is the last word at `0x8002A73C`.
/// - Table: slot 15 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA1C`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills six descriptors of `0x40` bytes each:
/// the shared vtable `0x803035DC` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x8002A7A0` through `0x802DC780`.
/// The state objects are published by the getters
/// `0x8035324C`, `0x8035328C`, `0x803532CC`, `0x8035330C`, `0x8035338C`, `0x8035334C`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the six blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_8002a230() {
    // StateID_Wait (0x80303510)
    w229_install_state(
        0x8035_2E20,
        0x8035_2E2C,
        0x8030_3510,
        0x8030_3438,
        0x8030_35DC,
        0x8002_A7A0,
        Some(0x8035_324C),
    );

    // StateID_Appear (0x80303530)
    w229_install_state(
        0x8035_2E60,
        0x8035_2E6C,
        0x8030_3530,
        0x8030_345C,
        0x8030_35DC,
        0x8002_A7A0,
        Some(0x8035_328C),
    );

    // StateID_Attack (0x80303550)
    w229_install_state(
        0x8035_2EA0,
        0x8035_2EAC,
        0x8030_3550,
        0x8030_3480,
        0x8030_35DC,
        0x8002_A7A0,
        Some(0x8035_32CC),
    );

    // StateID_Disappear (0x80303570)
    w229_install_state(
        0x8035_2EE0,
        0x8035_2EEC,
        0x8030_3570,
        0x8030_34A4,
        0x8030_35DC,
        0x8002_A7A0,
        Some(0x8035_330C),
    );

    // StateID_DieVanish (0x80303594)
    w229_install_state(
        0x8035_2F20,
        0x8035_2F2C,
        0x8030_3594,
        0x8030_34C8,
        0x8030_35DC,
        0x8002_A7A0,
        Some(0x8035_338C),
    );

    // StateID_DieIceBreak (0x803035B8)
    w229_install_state(
        0x8035_2F60,
        0x8035_2F6C,
        0x8030_35B8,
        0x8030_34EC,
        0x8030_35DC,
        0x8002_A7A0,
        Some(0x8035_334C),
    );
}

/// Installs the five `daEnDoor_c` names.
///
/// This is the static constructor of `daEnDoor_c`: the CRT calls it from
/// `.ctors` before `main`, so the five descriptor records of the class are in
/// place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Search` | `daEnDoor_c::StateID_Search` | `0x80352FD0` | `0x8030390C` |
/// | `StateID_Open` | `daEnDoor_c::StateID_Open` | `0x80353010` | `0x80303930` |
/// | `StateID_Close` | `daEnDoor_c::StateID_Close` | `0x80353050` | `0x80303954` |
/// | `StateID_Wait` | `daEnDoor_c::StateID_Wait` | `0x80353090` | `0x80303978` |
/// | `StateID_Dummy` | `daEnDoor_c::StateID_Dummy` | `0x803530D0` | `0x8030399C` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8002B8C0` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x8002B8C0..0x8002BCB4` (1012 bytes); the
///   closing `bclr 20,0` is the last word at `0x8002BCB0`.
/// - Table: slot 16 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA20`).
/// - Prologue: `bl 0x802DCD74`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDC0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills five descriptors of `0x40` bytes each:
/// the shared vtable `0x80303A4C` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C` (the class leaves `+0x30` alone),
/// then registers each descriptor with the destructor
/// `0x8002BCC0` through `0x802DC780`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the five blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_8002b8c0() {
    // StateID_Search (0x803039C0)
    w229_install_state(
        0x8035_2FC0,
        0x8035_2FD0,
        0x8030_39C0,
        0x8030_390C,
        0x8030_3A4C,
        0x8002_BCC0,
        None,
    );

    // StateID_Open (0x803039DC)
    w229_install_state(
        0x8035_3000,
        0x8035_3010,
        0x8030_39DC,
        0x8030_3930,
        0x8030_3A4C,
        0x8002_BCC0,
        None,
    );

    // StateID_Close (0x803039F8)
    w229_install_state(
        0x8035_3040,
        0x8035_3050,
        0x8030_39F8,
        0x8030_3954,
        0x8030_3A4C,
        0x8002_BCC0,
        None,
    );

    // StateID_Wait (0x80303A14)
    w229_install_state(
        0x8035_3080,
        0x8035_3090,
        0x8030_3A14,
        0x8030_3978,
        0x8030_3A4C,
        0x8002_BCC0,
        None,
    );

    // StateID_Dummy (0x80303A30)
    w229_install_state(
        0x8035_30C0,
        0x8035_30D0,
        0x8030_3A30,
        0x8030_399C,
        0x8030_3A4C,
        0x8002_BCC0,
        None,
    );
}

/// Installs the four `daEnDpakkun_c` names.
///
/// This is the static constructor of `daEnDpakkun_c`: the CRT calls it from
/// `.ctors` before `main`, so the four descriptor records of the class are in
/// place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Wait` | `daEnDpakkun_c::StateID_Wait` | `0x8035310C` | `0x80303D80` |
/// | `StateID_Appear` | `daEnDpakkun_c::StateID_Appear` | `0x8035314C` | `0x80303DA4` |
/// | `StateID_Attack` | `daEnDpakkun_c::StateID_Attack` | `0x8035318C` | `0x80303DC8` |
/// | `StateID_Disappear` | `daEnDpakkun_c::StateID_Disappear` | `0x803531CC` | `0x80303DEC` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8002C410` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x8002C410..0x8002C788` (888 bytes); the
///   closing `bclr 20,0` is the last word at `0x8002C784`.
/// - Table: slot 17 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA24`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills four descriptors of `0x40` bytes each:
/// the shared vtable `0x80303E90` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x8002C7F0` through `0x802DC780`.
/// The state objects are published by the getters
/// `0x8035324C`, `0x8035328C`, `0x803532CC`, `0x8035330C`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the four blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_8002c410() {
    // StateID_Wait (0x80303E10)
    w229_install_state(
        0x8035_3100,
        0x8035_310C,
        0x8030_3E10,
        0x8030_3D80,
        0x8030_3E90,
        0x8002_C7F0,
        Some(0x8035_324C),
    );

    // StateID_Appear (0x80303E2C)
    w229_install_state(
        0x8035_3140,
        0x8035_314C,
        0x8030_3E2C,
        0x8030_3DA4,
        0x8030_3E90,
        0x8002_C7F0,
        Some(0x8035_328C),
    );

    // StateID_Attack (0x80303E4C)
    w229_install_state(
        0x8035_3180,
        0x8035_318C,
        0x8030_3E4C,
        0x8030_3DC8,
        0x8030_3E90,
        0x8002_C7F0,
        Some(0x8035_32CC),
    );

    // StateID_Disappear (0x80303E6C)
    w229_install_state(
        0x8035_31C0,
        0x8035_31CC,
        0x8030_3E6C,
        0x8030_3DEC,
        0x8030_3E90,
        0x8002_C7F0,
        Some(0x8035_330C),
    );
}

/// Installs the six `daEnDpakkunBase_c` names.
///
/// This is the static constructor of `daEnDpakkunBase_c`: the CRT calls it
/// from `.ctors` before `main`, so the six descriptor records of the class are
/// in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Wait` | `daEnDpakkunBase_c::StateID_Wait` | `0x8035324C` | `0x80304240` |
/// | `StateID_Appear` | `daEnDpakkunBase_c::StateID_Appear` | `0x8035328C` | `0x80304264` |
/// | `StateID_Attack` | `daEnDpakkunBase_c::StateID_Attack` | `0x803532CC` | `0x80304288` |
/// | `StateID_Disappear` | `daEnDpakkunBase_c::StateID_Disappear` | `0x8035330C` | `0x803042AC` |
/// | `StateID_DieIceBreak` | `daEnDpakkunBase_c::StateID_DieIceBreak` | `0x8035334C` | `0x803042D0` |
/// | `StateID_DieVanish` | `daEnDpakkunBase_c::StateID_DieVanish` | `0x8035338C` | `0x803042F4` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8002E660` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x8002E660..0x8002EBA4` (1348 bytes); the
///   closing `bclr 20,0` is the last word at `0x8002EBA0`.
/// - Table: slot 18 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA28`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills six descriptors of `0x40` bytes each:
/// the shared vtable `0x803043F8` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x8002EC10` through `0x802DC780`.
/// The state object is published by the getter
/// `0x80377A04`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the six blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_8002e660() {
    // StateID_Wait (0x80304318)
    w229_install_state(
        0x8035_3240,
        0x8035_324C,
        0x8030_4318,
        0x8030_4240,
        0x8030_43F8,
        0x8002_EC10,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Appear (0x80304338)
    w229_install_state(
        0x8035_3280,
        0x8035_328C,
        0x8030_4338,
        0x8030_4264,
        0x8030_43F8,
        0x8002_EC10,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Attack (0x8030435C)
    w229_install_state(
        0x8035_32C0,
        0x8035_32CC,
        0x8030_435C,
        0x8030_4288,
        0x8030_43F8,
        0x8002_EC10,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Disappear (0x80304380)
    w229_install_state(
        0x8035_3300,
        0x8035_330C,
        0x8030_4380,
        0x8030_42AC,
        0x8030_43F8,
        0x8002_EC10,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_DieIceBreak (0x803043A8)
    w229_install_state(
        0x8035_3340,
        0x8035_334C,
        0x8030_43A8,
        0x8030_42D0,
        0x8030_43F8,
        0x8002_EC10,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_DieVanish (0x803043D0)
    w229_install_state(
        0x8035_3380,
        0x8035_338C,
        0x8030_43D0,
        0x8030_42F4,
        0x8030_43F8,
        0x8002_EC10,
        Some(W229B_TAG_SHARED as usize),
    );
}

/// Installs the four `daEnJimenPakkunBase_c` names.
///
/// This is the static constructor of `daEnJimenPakkunBase_c`: the CRT calls
/// it from `.ctors` before `main`, so the four descriptor records of the class
/// are in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_DieOther` | `daEnJimenPakkunBase_c::StateID_DieOther` | `0x803533EC` | `0x80304748` |
/// | `StateID_DieIceBreak` | `daEnJimenPakkunBase_c::StateID_DieIceBreak` | `0x8035342C` | `0x8030476C` |
/// | `StateID_Attack` | `daEnJimenPakkunBase_c::StateID_Attack` | `0x8035346C` | `0x80304790` |
/// | `StateID_SakasaAttack` | `daEnJimenPakkunBase_c::StateID_SakasaAttack` | `0x803534AC` | `0x803047B4` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80030AB0` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80030AB0..0x80030E28` (888 bytes); the
///   closing `bclr 20,0` is the last word at `0x80030E24`.
/// - Table: slot 19 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA2C`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills four descriptors of `0x40` bytes each:
/// the shared vtable `0x80304880` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x80030E90` through `0x802DC780`.
/// The state objects are published by the getters
/// `0x80357D74`, `0x80377A04`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the four blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_80030ab0() {
    // StateID_DieOther (0x803047D8)
    w229_install_state(
        0x8035_33E0,
        0x8035_33EC,
        0x8030_47D8,
        0x8030_4748,
        0x8030_4880,
        0x8003_0E90,
        Some(0x8035_7D74),
    );

    // StateID_DieIceBreak (0x80304800)
    w229_install_state(
        0x8035_3420,
        0x8035_342C,
        0x8030_4800,
        0x8030_476C,
        0x8030_4880,
        0x8003_0E90,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Attack (0x8030482C)
    w229_install_state(
        0x8035_3460,
        0x8035_346C,
        0x8030_482C,
        0x8030_4790,
        0x8030_4880,
        0x8003_0E90,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_SakasaAttack (0x80304854)
    w229_install_state(
        0x8035_34A0,
        0x8035_34AC,
        0x8030_4854,
        0x8030_47B4,
        0x8030_4880,
        0x8003_0E90,
        Some(W229B_TAG_SHARED as usize),
    );
}

/// Installs the four `daEnKuriboBase_c` names.
///
/// This is the static constructor of `daEnKuriboBase_c`: the CRT calls it
/// from `.ctors` before `main`, so the four descriptor records of the class are
/// in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Walk` | `daEnKuriboBase_c::StateID_Walk` | `0x803534EC` | `0x80304C28` |
/// | `StateID_Turn` | `daEnKuriboBase_c::StateID_Turn` | `0x8035352C` | `0x80304C4C` |
/// | `StateID_TrplnJump` | `daEnKuriboBase_c::StateID_TrplnJump` | `0x8035356C` | `0x80304C70` |
/// | `StateID_DieOther` | `daEnKuriboBase_c::StateID_DieOther` | `0x803535AC` | `0x80304C94` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80032AB0` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80032AB0..0x80032E28` (888 bytes); the
///   closing `bclr 20,0` is the last word at `0x80032E24`.
/// - Table: slot 20 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA30`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills four descriptors of `0x40` bytes each:
/// the shared vtable `0x80304D40` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x80032E90` through `0x802DC780`.
/// The state objects are published by the getters
/// `0x80377A04`, `0x80357D74`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the four blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_80032ab0() {
    // StateID_Walk (0x80304CB8)
    w229_install_state(
        0x8035_34E0,
        0x8035_34EC,
        0x8030_4CB8,
        0x8030_4C28,
        0x8030_4D40,
        0x8003_2E90,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Turn (0x80304CD8)
    w229_install_state(
        0x8035_3520,
        0x8035_352C,
        0x8030_4CD8,
        0x8030_4C4C,
        0x8030_4D40,
        0x8003_2E90,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_TrplnJump (0x80304CF8)
    w229_install_state(
        0x8035_3560,
        0x8035_356C,
        0x8030_4CF8,
        0x8030_4C70,
        0x8030_4D40,
        0x8003_2E90,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_DieOther (0x80304D1C)
    w229_install_state(
        0x8035_35A0,
        0x8035_35AC,
        0x8030_4D1C,
        0x8030_4C94,
        0x8030_4D40,
        0x8003_2E90,
        Some(0x8035_7D74),
    );
}

/// Installs the six `daEnLkuriboBase_c` names.
///
/// This is the static constructor of `daEnLkuriboBase_c`: the CRT calls it
/// from `.ctors` before `main`, so the six descriptor records of the class are
/// in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Walk` | `daEnLkuriboBase_c::StateID_Walk` | `0x803535EC` | `0x80305100` |
/// | `StateID_Turn` | `daEnLkuriboBase_c::StateID_Turn` | `0x8035362C` | `0x80305124` |
/// | `StateID_Press` | `daEnLkuriboBase_c::StateID_Press` | `0x8035366C` | `0x80305148` |
/// | `StateID_Split` | `daEnLkuriboBase_c::StateID_Split` | `0x803536AC` | `0x8030516C` |
/// | `StateID_HipSplit` | `daEnLkuriboBase_c::StateID_HipSplit` | `0x803536EC` | `0x80305190` |
/// | `StateID_DieFall` | `daEnLkuriboBase_c::StateID_DieFall` | `0x8035372C` | `0x803051B4` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80034E10` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80034E10..0x80035320` (1296 bytes); the
///   closing `bclr 20,0` is the last word at `0x8003531C`.
/// - Table: slot 21 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA34`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills six descriptors of `0x40` bytes each:
/// the shared vtable `0x803052A8` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x80035380` through `0x802DC780`.
/// The state objects are published by the getters
/// `0x80377A04`, `0x80357BF4`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the six blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_80034e10() {
    // StateID_Walk (0x803051D8)
    w229_install_state(
        0x8035_35E0,
        0x8035_35EC,
        0x8030_51D8,
        0x8030_5100,
        0x8030_52A8,
        0x8003_5380,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Turn (0x803051F8)
    w229_install_state(
        0x8035_3620,
        0x8035_362C,
        0x8030_51F8,
        0x8030_5124,
        0x8030_52A8,
        0x8003_5380,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Press (0x80305218)
    w229_install_state(
        0x8035_3660,
        0x8035_366C,
        0x8030_5218,
        0x8030_5148,
        0x8030_52A8,
        0x8003_5380,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Split (0x8030523C)
    w229_install_state(
        0x8035_36A0,
        0x8035_36AC,
        0x8030_523C,
        0x8030_516C,
        0x8030_52A8,
        0x8003_5380,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_HipSplit (0x80305260)
    w229_install_state(
        0x8035_36E0,
        0x8035_36EC,
        0x8030_5260,
        0x8030_5190,
        0x8030_52A8,
        0x8003_5380,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_DieFall (0x80305284)
    w229_install_state(
        0x8035_3720,
        0x8035_372C,
        0x8030_5284,
        0x8030_51B4,
        0x8030_52A8,
        0x8003_5380,
        Some(0x8035_7BF4),
    );
}

/// Installs the two `daEnNetNoko_c` names.
///
/// This is the static constructor of `daEnNetNoko_c`: the CRT calls it from
/// `.ctors` before `main`, so the two descriptor records of the class are in
/// place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_DieFumi` | `daEnNetNoko_c::StateID_DieFumi` | `0x8035376C` | `0x803055E8` |
/// | `StateID_DieFall` | `daEnNetNoko_c::StateID_DieFall` | `0x803537AC` | `0x8030560C` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80036390` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80036390..0x80036570` (480 bytes); the
///   closing `bclr 20,0` is the last word at `0x8003656C`.
/// - Table: slot 22 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA38`).
/// - Prologue: `bl 0x802DCD70`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDBC`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills two descriptors of `0x40` bytes each:
/// the shared vtable `0x80305670` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x800365D0` through `0x802DC780`.
/// The state objects are published by the getters
/// `0x80357BB4`, `0x80357BF4`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the two blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_80036390() {
    // StateID_DieFumi (0x80305630)
    w229_install_state(
        0x8035_3760,
        0x8035_376C,
        0x8030_5630,
        0x8030_55E8,
        0x8030_5670,
        0x8003_65D0,
        Some(0x8035_7BB4),
    );

    // StateID_DieFall (0x80305650)
    w229_install_state(
        0x8035_37A0,
        0x8035_37AC,
        0x8030_5650,
        0x8030_560C,
        0x8030_5670,
        0x8003_65D0,
        Some(0x8035_7BF4),
    );
}

/// Installs the four `daEnSuperBigPile_c` names.
///
/// This is the static constructor of `daEnSuperBigPile_c`: the CRT calls it
/// from `.ctors` before `main`, so the four descriptor records of the class are
/// in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_GoWait` | `daEnSuperBigPile_c::StateID_GoWait` | `0x80353BA4` | `0x803061FC` |
/// | `StateID_GoMove` | `daEnSuperBigPile_c::StateID_GoMove` | `0x80353BE4` | `0x80306220` |
/// | `StateID_RetWait` | `daEnSuperBigPile_c::StateID_RetWait` | `0x80353C24` | `0x80306244` |
/// | `StateID_RetMove` | `daEnSuperBigPile_c::StateID_RetMove` | `0x80353C64` | `0x80306268` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8003D4B0` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x8003D4B0..0x8003D828` (888 bytes); the
///   closing `bclr 20,0` is the last word at `0x8003D824`.
/// - Table: slot 25 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA44`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills four descriptors of `0x40` bytes each:
/// the shared vtable `0x8030631C` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x8003D890` through `0x802DC780`.
/// The state object is published by the getter
/// `0x80377A04`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the four blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_8003d4b0() {
    // StateID_GoWait (0x8030628C)
    w229_install_state(
        0x8035_3B98,
        0x8035_3BA4,
        0x8030_628C,
        0x8030_61FC,
        0x8030_631C,
        0x8003_D890,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_GoMove (0x803062B0)
    w229_install_state(
        0x8035_3BD8,
        0x8035_3BE4,
        0x8030_62B0,
        0x8030_6220,
        0x8030_631C,
        0x8003_D890,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_RetWait (0x803062D4)
    w229_install_state(
        0x8035_3C18,
        0x8035_3C24,
        0x8030_62D4,
        0x8030_6244,
        0x8030_631C,
        0x8003_D890,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_RetMove (0x803062F8)
    w229_install_state(
        0x8035_3C58,
        0x8035_3C64,
        0x8030_62F8,
        0x8030_6268,
        0x8030_631C,
        0x8003_D890,
        Some(W229B_TAG_SHARED as usize),
    );
}

/// Installs the nine `daEnTogezoBase_c` names.
///
/// This is the static constructor of `daEnTogezoBase_c`: the CRT calls it
/// from `.ctors` before `main`, so the nine descriptor records of the class are
/// in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Walk` | `daEnTogezoBase_c::StateID_Walk` | `0x80353CA8` | `0x80306704` |
/// | `StateID_Pipo` | `daEnTogezoBase_c::StateID_Pipo` | `0x80353CE8` | `0x80306728` |
/// | `StateID_Change_Pipo` | `daEnTogezoBase_c::StateID_Change_Pipo` | `0x80353D28` | `0x8030674C` |
/// | `StateID_Change_Togezo` | `daEnTogezoBase_c::StateID_Change_Togezo` | `0x80353D68` | `0x80306770` |
/// | `StateID_Turn` | `daEnTogezoBase_c::StateID_Turn` | `0x80353DA8` | `0x80306794` |
/// | `StateID_DieFall` | `daEnTogezoBase_c::StateID_DieFall` | `0x80353DE4` | `0x803067B8` |
/// | `StateID_Wakeup` | `daEnTogezoBase_c::StateID_Wakeup` | `0x80353E24` | `0x803067DC` |
/// | `StateID_WakeupTurn` | `daEnTogezoBase_c::StateID_WakeupTurn` | `0x80353E64` | `0x80306800` |
/// | `StateID_WakeupReverse` | `daEnTogezoBase_c::StateID_WakeupReverse` | `0x80353EA4` | `0x80306824` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8003FE10` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x8003FE10..0x80040540` (1840 bytes); the
///   closing `bclr 20,0` is the last word at `0x8004053C`.
/// - Table: slot 26 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA48`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills nine descriptors of `0x40` bytes each:
/// the shared vtables `0x803069C4`, `0x80306990` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30` on four of the nine records,
/// then registers each descriptor with the destructors
/// `0x80040540`, `0x800405A0` through `0x802DC780`.
/// Records `0..4` use vtable `0x803069C4` and destructor `0x80040540`;
/// records `5..8` switch to vtable `0x80306990` and
/// destructor `0x800405A0`.
/// The state objects are published by the getters
/// `0x80353B64`, `0x80353AA4`, `0x80353B24`, `0x80353AE4`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the nine blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_8003fe10() {
    // StateID_Walk (0x80306848)
    w229_install_state(
        0x8035_3C98,
        0x8035_3CA8,
        0x8030_6848,
        0x8030_6704,
        0x8030_69C4,
        0x8004_0540,
        None,
    );

    // StateID_Pipo (0x80306868)
    w229_install_state(
        0x8035_3CD8,
        0x8035_3CE8,
        0x8030_6868,
        0x8030_6728,
        0x8030_69C4,
        0x8004_0540,
        None,
    );

    // StateID_Change_Pipo (0x80306888)
    w229_install_state(
        0x8035_3D18,
        0x8035_3D28,
        0x8030_6888,
        0x8030_674C,
        0x8030_69C4,
        0x8004_0540,
        None,
    );

    // StateID_Change_Togezo (0x803068B0)
    w229_install_state(
        0x8035_3D58,
        0x8035_3D68,
        0x8030_68B0,
        0x8030_6770,
        0x8030_69C4,
        0x8004_0540,
        None,
    );

    // StateID_Turn (0x803068D8)
    w229_install_state(
        0x8035_3D98,
        0x8035_3DA8,
        0x8030_68D8,
        0x8030_6794,
        0x8030_69C4,
        0x8004_0540,
        None,
    );

    // StateID_DieFall (0x803068F8)
    w229_install_state(
        0x8035_3DD8,
        0x8035_3DE4,
        0x8030_68F8,
        0x8030_67B8,
        0x8030_6990,
        0x8004_05A0,
        Some(0x8035_3B64),
    );

    // StateID_Wakeup (0x8030691C)
    w229_install_state(
        0x8035_3E18,
        0x8035_3E24,
        0x8030_691C,
        0x8030_67DC,
        0x8030_6990,
        0x8004_05A0,
        Some(0x8035_3AA4),
    );

    // StateID_WakeupTurn (0x80306940)
    w229_install_state(
        0x8035_3E58,
        0x8035_3E64,
        0x8030_6940,
        0x8030_6800,
        0x8030_6990,
        0x8004_05A0,
        Some(0x8035_3B24),
    );

    // StateID_WakeupReverse (0x80306968)
    w229_install_state(
        0x8035_3E98,
        0x8035_3EA4,
        0x8030_6968,
        0x8030_6824,
        0x8030_6990,
        0x8004_05A0,
        Some(0x8035_3AE4),
    );
}

/// Installs the six `daLiftDownOnBase_c` names.
///
/// This is the static constructor of `daLiftDownOnBase_c`: the CRT calls it
/// from `.ctors` before `main`, so the six descriptor records of the class are
/// in place before any scene is created.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_Wait` | `daLiftDownOnBase_c::StateID_Wait` | `0x8035400C` | `0x80307190` |
/// | `StateID_Go` | `daLiftDownOnBase_c::StateID_Go` | `0x8035404C` | `0x803071B4` |
/// | `StateID_Return` | `daLiftDownOnBase_c::StateID_Return` | `0x8035408C` | `0x803071D8` |
/// | `StateID_WaitMoveForever` | `daLiftDownOnBase_c::StateID_WaitMoveForever` | `0x803540CC` | `0x803071FC` |
/// | `StateID_MoveForever` | `daLiftDownOnBase_c::StateID_MoveForever` | `0x8035410C` | `0x80307220` |
/// | `StateID_StillForever` | `daLiftDownOnBase_c::StateID_StillForever` | `0x8035414C` | `0x80307244` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80042F20` and the
///   export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80042F20..0x80043430` (1296 bytes); the
///   closing `bclr 20,0` is the last word at `0x8004342C`.
/// - Table: slot 28 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA50`).
/// - Prologue: `bl 0x802DCD6C`, the register-save leaf that stores
///   `r25..r31` below the caller's frame; epilogue `bl 0x802DCDB8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills six descriptors of `0x40` bytes each:
/// the shared vtable `0x80307354` at `+0x00`,
/// the name and name index at `+0x04`/`+0x08`, the nine parameter words at
/// `+0x0C..+0x2C`, and the state object at `+0x30`,
/// then registers each descriptor with the destructor
/// `0x80043490` through `0x802DC780`.
/// The state object is published by the getter
/// `0x80377A04`.
///
/// # Safety
/// The descriptor range and the registration nodes must be writable, and the
/// parameter blocks and name strings readable; all four are the module's own
/// `.data`/BSS, so this holds once `main.dol` is loaded.
///
/// # Confidence
/// high - the six blocks repeat the same template and differ only in their
/// literal addresses; every store and all three calls per block are
/// reproduced, including the reversed nine-word copy.
pub unsafe fn fn_80042f20() {
    // StateID_Wait (0x80307268)
    w229_install_state(
        0x8035_4000,
        0x8035_400C,
        0x8030_7268,
        0x8030_7190,
        0x8030_7354,
        0x8004_3490,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Go (0x8030728C)
    w229_install_state(
        0x8035_4040,
        0x8035_404C,
        0x8030_728C,
        0x8030_71B4,
        0x8030_7354,
        0x8004_3490,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_Return (0x803072AC)
    w229_install_state(
        0x8035_4080,
        0x8035_408C,
        0x8030_72AC,
        0x8030_71D8,
        0x8030_7354,
        0x8004_3490,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_WaitMoveForever (0x803072D0)
    w229_install_state(
        0x8035_40C0,
        0x8035_40CC,
        0x8030_72D0,
        0x8030_71FC,
        0x8030_7354,
        0x8004_3490,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_MoveForever (0x80307300)
    w229_install_state(
        0x8035_4100,
        0x8035_410C,
        0x8030_7300,
        0x8030_7220,
        0x8030_7354,
        0x8004_3490,
        Some(W229B_TAG_SHARED as usize),
    );

    // StateID_StillForever (0x80307328)
    w229_install_state(
        0x8035_4140,
        0x8035_414C,
        0x8030_7328,
        0x8030_7244,
        0x8030_7354,
        0x8004_3490,
        Some(W229B_TAG_SHARED as usize),
    );
}

// ---------------------------------------------------------------------------
// Wave 230, lane C - `.ctors` slots 29..42
// ---------------------------------------------------------------------------
//
// Continuation of the walk over the 181-pointer table at `0x802ED9E0` that
// `run_init_table` (`0x801B8990`) performs before `main`. Lane B covered
// slots 1..14 and wave-229 lane C covered 15..28 in the section above; this
// section covers 29..42, the twelve remaining untabled bodies in that run.
//
// | slot | table word | target | bytes | shape | result |
// |---:|---|---|---:|---|---|
// | 29 | `0x802EDA54` | `0x80044920` | 20 | leaf, one store | **below** |
// | 30 | `0x802EDA58` | `0x80044C70` | 480 | 2 descriptors | **below** |
// | 31 | `0x802EDA5C` | `0x800592C0` | 7620 | 37 descriptors | `fn_800592c0` in `src/game/player.rs` (already translated) |
// | 32 | `0x802EDA60` | `0x8005D750` | 112 | static block | **below** |
// | 33 | `0x802EDA64` | `0x8005DF00` | 208 | 4x30-word copy | **below** |
// | 34 | `0x802EDA68` | `0x80061310` | 156 | 4 constructors | **below** |
// | 35 | `0x802EDA6C` | `0x80061B60` | 492 | 2 descriptors | **below** |
// | 36 | `0x802EDA70` | `0x80062980` | 480 | 2 descriptors | **below** |
// | 37 | `0x802EDA74` | `0x80063C40` | 448 | 2 descriptors | **below** |
// | 38 | `0x802EDA78` | `0x800660A0` | 24 | two float stores | **below** |
// | 39 | `0x802EDA7C` | `0x80066360` | 20 | two word stores | **below** |
// | 40 | `0x802EDA80` | `0x80067830` | 492 | 2 descriptors | `fn_80067830` in `src/game/player.rs` (already translated) |
// | 41 | `0x802EDA84` | `0x8006C020` | 112 | static block | **below** |
// | 42 | `0x802EDA88` | `0x8006CF20` | 20 | two word stores | **below** |
//
// Slots 31 and 40 have `functions.tsv` rows and were translated in earlier
// waves, so they are referenced rather than duplicated. Slots 29 and 38..42 are
// scalar/table initialisers; slots 30 and 35..37 build the same `0x40`-byte
// state descriptors as wave-229 lane C and reuse its
// [`w229_install_state`] helper (and therefore the
// [`W229_STATE_PERMUTATION`] copy order). Slot 34 has its own constructor
// shape, and slots 32/33 are plain memory work.
//
// Measured extents (entry to the closing `bclr 20,0`, last word included):
//
// | slot | extent | bytes |
// |---:|---|---:|
// | 29 | `0x80044920..0x80044934` | 20 |
// | 30 | `0x80044C70..0x80044E50` | 480 |
// | 32 | `0x8005D750..0x8005D7C0` | 112 |
// | 33 | `0x8005DF00..0x8005DFD0` | 208 |
// | 34 | `0x80061310..0x800613AC` | 156 |
// | 35 | `0x80061B60..0x80061D4C` | 492 |
// | 36 | `0x80062980..0x80062B60` | 480 |
// | 37 | `0x80063C40..0x80063E00` | 448 |
// | 38 | `0x800660A0..0x800660B8` | 24 |
// | 39 | `0x80066360..0x80066374` | 20 |
// | 41 | `0x8006C020..0x8006C090` | 112 |
// | 42 | `0x8006CF20..0x8006CF34` | 20 |
//
// Total: 12 bodies, 2,572 bytes. Slot 31's 7,620 bytes and slot 40's 492 bytes
// are already covered by the `player.rs` bodies above.
//
// The `bl 0x802DCDxx` prologues only save and restore non-volatile integer
// registers; they have no observable effect and are not reproduced. `r2` is
// [`SDA2_BASE`] and `r13` is [`SDA1_BASE`].
// ---------------------------------------------------------------------------

/// `SDA1_BASE - 0x5A08`: the ring radius written by [`fn_80044920`].
const SDA1_RING_RADIUS_C230: usize = SDA1_BASE - 0x5A08;

/// `SDA2_BASE - 0x78B0`: first factor of the ring radius (`32.0f`).
const SDA2_RING_SCALE_C230: usize = SDA2_BASE - 0x78B0;

/// `SDA2_BASE - 0x78A4`: second factor of the ring radius (`0.125f`).
const SDA2_RING_FACTOR_C230: usize = SDA2_BASE - 0x78A4;

/// `SDA2_BASE - 0x6114`: word copied into the parameter block by
/// [`fn_8005d750`] and [`fn_8006c020`] (`0x17`).
const SDA2_BLOCK_WORD_C230: usize = SDA2_BASE - 0x6114;

/// `SDA2_BASE - 0x60FC`: word copied into SDA1 by [`fn_8005d750`] and
/// [`fn_8006c020`] (`0x26`).
const SDA2_TAIL_WORD_C230: usize = SDA2_BASE - 0x60FC;

/// `SDA2_BASE - 0x7648`: first float of the parameter block written by
/// [`fn_8005d750`] (`2160.0f`).
const SDA2_BLOCK_F0_C230: usize = SDA2_BASE - 0x7648;

/// `SDA2_BASE - 0x7644`: second float written by [`fn_8005d750`] (`-30.0f`).
const SDA2_BLOCK_F1_C230: usize = SDA2_BASE - 0x7644;

/// `SDA2_BASE - 0x7640`: third float written by [`fn_8005d750`] (`-478.0f`).
const SDA2_BLOCK_F2_C230: usize = SDA2_BASE - 0x7640;

/// `SDA2_BASE - 0x7468`: first float of the parameter block written by
/// [`fn_8006c020`] (`2160.0f`).
const SDA2_BLOCK2_F0_C230: usize = SDA2_BASE - 0x7468;

/// `SDA2_BASE - 0x7464`: second float written by [`fn_8006c020`] (`-30.0f`).
const SDA2_BLOCK2_F1_C230: usize = SDA2_BASE - 0x7464;

/// `SDA2_BASE - 0x7460`: third float written by [`fn_8006c020`] (`-478.0f`).
const SDA2_BLOCK2_F2_C230: usize = SDA2_BASE - 0x7460;

/// `SDA2_BASE - 0x74FC`: first float written by [`fn_800660a0`] (`9.0f`).
const SDA2_IMPULSE_A_C230: usize = SDA2_BASE - 0x74FC;

/// `SDA2_BASE - 0x74F8`: second float written by [`fn_800660a0`] (`12.0f`).
const SDA2_IMPULSE_B_C230: usize = SDA2_BASE - 0x74F8;

/// Runs a static constructor and registers the object's destructor.
///
/// # Original
/// The four non-descriptor records of slot 34 share one shape: `bl <ctor>`
/// with the object in `r3`, then `bl 0x802DC780(object, destructor, node)`.
/// The constructor returns nothing observable, so its `r3` is ignored.
///
/// # Arguments
/// * `object` - the static object passed to the constructor in `r3`.
/// * `constructor` - the class constructor.
/// * `destructor` - the callback stored in the registry node.
/// * `node` - the 12-byte registry node that precedes `object`.
///
/// # Safety
/// `object` must be a valid, writable object for its constructor and `node`
/// must be writable through `+8`.
///
/// # Confidence
/// high - one-for-one transcription of the repeated block.
#[inline]
unsafe fn w230_construct_and_register(
    object: usize,
    constructor: usize,
    destructor: usize,
    node: usize,
) {
    platform::call1(constructor, object as u32);
    platform::call3(FN_802DC780, object as u32, destructor as u32, node as u32);
}

/// Initialises the player ring radius at `0x80429C78`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80044920` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80044920..0x80044934` (20 bytes); the closing `bclr 20,0` is
///   the last word at `0x80044930`.
/// - Table: slot 29 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA54`).
/// - Shape: a frameless leaf - two `lfs` from `.sdata2`, one `fmuls`, one
///   `stfs` to `r13`, then `blr`.
///
/// # Arguments
/// None. All three addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `32.0 * 0.125 = 4.0` at `0x80429C78` (`SDA1_BASE - 0x5A08`), the
/// global radius that the player ring builder copies into every ring record
/// (see [`crate::game::player`]).
///
/// # Safety
/// `0x80429C78` must be writable and both `.sdata2` words readable.
///
/// # Confidence
/// high - five instructions, all literal.
pub unsafe fn fn_80044920() {
    let radius =
        platform::read_f32(SDA2_RING_SCALE_C230) * platform::read_f32(SDA2_RING_FACTOR_C230);
    platform::write_f32(SDA1_RING_RADIUS_C230, radius);
}

/// Installs the two `daNetEnemy_c` names.
///
/// This is the static constructor of `daNetEnemy_c`: the CRT calls it from
/// `.ctors` before `main`, so both descriptor records are in place before any
/// scene is created.
///
/// | state | name string | descriptor | parameter block | state getter |
/// |---|---|---|---|---|
/// | `StateID_NetWait` | `daNetEnemy_c::StateID_NetWait` | `0x8035418C` | `0x80307820` | `0x80044940` |
/// | `StateID_NetMove` | `daNetEnemy_c::StateID_NetMove` | `0x803541CC` | `0x80307844` | `0x80044950` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80044C70` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80044C70..0x80044E50` (480 bytes); the closing `bclr 20,0` is
///   the last word at `0x80044E4C`.
/// - Table: slot 30 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA58`).
/// - Prologue: `bl 0x802DCD70`, the register-save leaf for `r26..r31`;
///   epilogue `bl 0x802DCDBC`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills two `0x40`-byte descriptor records whose nodes are `0x80354180` and
/// `0x803541C0`: the shared vtable `0x803078A8`, the name and name index, the
/// nine parameter words, the state object at `+0x30` published by the getters,
/// and a registry entry with destructor `0x80044EB0`.
///
/// # Safety
/// Both descriptor ranges, both nodes, the two parameter blocks and the name
/// strings must be mapped (the module's own `.data`/BSS) and the getters must
/// be valid.
///
/// # Confidence
/// high - both blocks repeat the wave-229 template and differ only in their
/// literal addresses; every store and all three calls are reproduced.
pub unsafe fn fn_80044c70() {
    // StateID_NetWait (0x80307868)
    w229_install_state(
        0x8035_4180,
        0x8035_418C,
        0x8030_7868,
        0x8030_7820,
        0x8030_78A8,
        0x8004_4EB0,
        Some(0x8004_4940),
    );

    // StateID_NetMove (0x80307888)
    w229_install_state(
        0x8035_41C0,
        0x8035_41CC,
        0x8030_7888,
        0x8030_7844,
        0x8030_78A8,
        0x8004_4EB0,
        Some(0x8004_4950),
    );
}

/// Initialises the static parameter block at `0x80309608`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8005D750` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x8005D750..0x8005D7C0` (112 bytes); the closing `bclr 20,0` is
///   the last word at `0x8005D7BC`.
/// - Table: slot 32 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA60`).
/// - Shape: a small framed leaf - one word and three floats are copied from
///   `.sdata2`, a registry node is pushed, and a second word is copied to SDA1.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0x17` at `0x80309614`, `2160.0f`/`-30.0f`/`-478.0f` at
/// `0x80309620`/`0x80309624`/`0x80309628`, pushes
/// `0x802DC780(0, 0x8005D7C0, 0x80354C20)`, and copies `0x26` from
/// `SDA2_BASE - 0x60FC` to `0x80429C90` (`SDA1_BASE - 0x59F0`).
///
/// # Observed data
/// The block already holds `{6, 0x804278F0, 6, 0, 4, 0x804278F8, 0, 0, 0}`
/// in the image; this body overwrites `+0x0C` and the three float slots at
/// `+0x18..+0x20`. The registry call passes a null descriptor in `r3`, so the
/// node's value word (`node[2]`) receives zero.
///
/// The original also writes the three floats to the dead stack slots
/// `r1+8..0x10` before the call; nothing reads them, so they are omitted.
///
/// # Safety
/// The block at `0x80309608` must be writable through `+0x28`, and the two
/// SDA words readable.
///
/// # Confidence
/// high - every load, store and call is literal; only the dead stack copies
/// are elided.
pub unsafe fn fn_8005d750() {
    platform::write_u32(0x8030_9614, platform::read_u32(SDA2_BLOCK_WORD_C230));
    platform::write_f32(0x8030_9620, platform::read_f32(SDA2_BLOCK_F0_C230));
    platform::write_f32(0x8030_9624, platform::read_f32(SDA2_BLOCK_F1_C230));
    platform::write_f32(0x8030_9628, platform::read_f32(SDA2_BLOCK_F2_C230));
    platform::call3(FN_802DC780, 0, 0x8005_D7C0, 0x8035_4C20);
    platform::write_u32(SDA1_BASE - 0x59F0, platform::read_u32(SDA2_TAIL_WORD_C230));
}

/// Copies four 30-float tuning blocks into `0x80354C30`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8005DF00` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x8005DF00..0x8005DFD0` (208 bytes); the closing `bclr 20,0` is
///   the last word at `0x8005DFCC`.
/// - Table: slot 33 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA64`).
/// - Shape: a frameless leaf with four unrolled `ctr` loops; each iteration
///   moves two words with `lwz`/`lwzu` and `stw`/`stwu`, so one block is 15
///   iterations = 30 words = `0x78` bytes.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies `0x802EEE00 -> 0x80354C30`, `0x802EEE78 -> 0x80354CA8`,
/// `0x802EEEF0 -> 0x80354D20` and `0x802EEF68 -> 0x80354D98`, i.e. the
/// contiguous `0x1E0`-byte range `0x80354C30..0x80354E10` that the player code
/// reads as two `0xF0`-byte records (see [`crate::game::player`]). The next
/// object, `0x80354E10`, is the base that slot 34 constructs.
///
/// # Safety
/// The four source blocks and the `0x1E0`-byte destination must be mapped and
/// writable; each source word is a plain `u32` copy (the words are floats but
/// the copy is bitwise).
///
/// # Confidence
/// high - the unrolled loop bounds and both strides are literal, and the
/// destination interval ends exactly at `0x80354E10`.
pub unsafe fn fn_8005df00() {
    // observed: four blocks of 15 two-word iterations.
    let mut block = 0;
    while block < 4 {
        let source = 0x802E_EE00 + block * 0x78;
        let destination = 0x8035_4C30 + block * 0x78;
        let mut word = 0;
        while word < 30 {
            platform::write_u32(
                destination + word * 4,
                platform::read_u32(source + word * 4),
            );
            word += 1;
        }
        block += 1;
    }
}

/// Constructs and registers the four objects at `0x80354EE0..0x80355054`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80061310` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80061310..0x800613AC` (156 bytes); the closing `bclr 20,0` is
///   the last word at `0x800613A8`.
/// - Table: slot 34 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA68`).
/// - Shape: `stwu`/`mfspr` frame saving `r31`, four `bl <ctor>` /
///   `bl 0x802DC780` pairs, then the LR restore and `blr`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs four no-argument constructors and pushes one registry node each:
///
/// | object | constructor | destructor | node |
/// |---|---|---|---|
/// | `0x80354EE0` | `0x8005B3A0` | `0x8005B3E0` | `0x80354ED0` |
/// | `0x80354F84` | `0x800CE800` | `0x800CE820` | `0x80354F78` |
/// | `0x80354FF0` | `0x800690F0` | `0x80069110` | `0x80354FE0` |
/// | `0x80355054` | `0x800D2C20` | `0x800D2C80` | `0x80355048` |
///
/// The node offsets alternate: `0x80354ED0`/`0x80354EE0` and
/// `0x80354FE0`/`0x80354FF0` are `0x10` apart, while
/// `0x80354F78`/`0x80354F84` and `0x80355048`/`0x80355054` are `0x0C` apart,
/// matching the sixteen-byte and twelve-byte alignments used by the descriptor
/// bodies above.
///
/// # Safety
/// Every object must be valid for its constructor, every node must be writable
/// through `+8`, and the registry head used by `0x802DC780` must exist.
///
/// # Confidence
/// high - eight call sites, four literal object addresses and four literal
/// node addresses; the constructors return nothing that the body reads.
pub unsafe fn fn_80061310() {
    w230_construct_and_register(0x8035_4EE0, 0x8005_B3A0, 0x8005_B3E0, 0x8035_4ED0);
    w230_construct_and_register(0x8035_4F84, 0x800C_E800, 0x800C_E820, 0x8035_4F78);
    w230_construct_and_register(0x8035_4FF0, 0x8006_90F0, 0x8006_9110, 0x8035_4FE0);
    w230_construct_and_register(0x8035_5054, 0x800D_2C20, 0x800D_2C80, 0x8035_5048);
}

/// Installs the two `daLightBase_c` names.
///
/// | state | name string | descriptor | parameter block | vtable | getter |
/// |---|---|---|---|---|---|
/// | `StateID_SearchID` | `daLightBase_c::StateID_SearchID` | `0x80355CD0` | `0x80309848` | `0x80309900` | none |
/// | `StateID_Move` | `daLightBase_c::StateID_Move` | `0x80355D0C` | `0x8030986C` | `0x803098CC` | `0x800613B0` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80061B60` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80061B60..0x80061D4C` (492 bytes); the closing `bclr 20,0` is
///   the last word at `0x80061D48`.
/// - Table: slot 35 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA6C`).
/// - Prologue: `stwu r1,-0x60` plus the inline save of `r28..r31`; no
///   `0x802DCDxx` stub.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills two `0x40`-byte records whose nodes are `0x80355CC0` and
/// `0x80355D00`, with destructors `0x80061D50` and `0x80061DB0`.
/// The first record is sixteen-byte aligned (`descriptor = node + 0x10`) and
/// leaves `descriptor + 0x30` untouched; the second uses `node + 0x0C` and
/// stores the state object returned by `0x800613B0` at `+0x30`.
///
/// # Safety
/// Both descriptor ranges, both nodes, the two parameter blocks and the name
/// strings must be mapped and writable as required by
/// [`w229_install_state`].
///
/// # Confidence
/// high - the two blocks differ only in literals and in the presence of the
/// getter, which the instruction stream shows as an extra `bl 0x800613B0`
/// before the second name setter.
pub unsafe fn fn_80061b60() {
    // StateID_SearchID (0x80309890): no state object.
    w229_install_state(
        0x8035_5CC0,
        0x8035_5CD0,
        0x8030_9890,
        0x8030_9848,
        0x8030_9900,
        0x8006_1D50,
        None,
    );

    // StateID_Move (0x803098B0)
    w229_install_state(
        0x8035_5D00,
        0x8035_5D0C,
        0x8030_98B0,
        0x8030_986C,
        0x8030_98CC,
        0x8006_1DB0,
        Some(0x8006_13B0),
    );
}

/// Installs the two `daRotObjsBase_c` names.
///
/// | state | name string | descriptor | parameter block | getter |
/// |---|---|---|---|---|
/// | `StateID_Search` | `daRotObjsBase_c::StateID_Search` | `0x80355D4C` | `0x80309B44` | `0x80062300` |
/// | `StateID_Move` | `daRotObjsBase_c::StateID_Move` | `0x80355D8C` | `0x80309B68` | `0x80062310` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80062980` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80062980..0x80062B60` (480 bytes); the closing `bclr 20,0` is
///   the last word at `0x80062B5C`.
/// - Table: slot 36 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA70`).
/// - Prologue: `bl 0x802DCD70`; epilogue `bl 0x802DCDBC`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills two `0x40`-byte records whose nodes are `0x80355D40` and
/// `0x80355D80`, both using vtable `0x80309BD0` and destructor `0x80062BC0`;
/// each stores its getter result at `descriptor + 0x30`.
///
/// # Safety
/// Both descriptor ranges, both nodes and the source blocks must be mapped and
/// writable as required by [`w229_install_state`].
///
/// # Confidence
/// high - identical template to [`fn_80044c70`], only the literals differ.
pub unsafe fn fn_80062980() {
    // StateID_Search (0x80309B90)
    w229_install_state(
        0x8035_5D40,
        0x8035_5D4C,
        0x8030_9B90,
        0x8030_9B44,
        0x8030_9BD0,
        0x8006_2BC0,
        Some(0x8006_2300),
    );

    // StateID_Move (0x80309BB0)
    w229_install_state(
        0x8035_5D80,
        0x8035_5D8C,
        0x8030_9BB0,
        0x8030_9B68,
        0x8030_9BD0,
        0x8006_2BC0,
        Some(0x8006_2310),
    );
}

/// Installs the two `daSpinChildBase_c` names.
///
/// | state | name string | descriptor | parameter block |
/// |---|---|---|---|
/// | `StateID_SearchID` | `daSpinChildBase_c::StateID_SearchID` | `0x80355DD0` | `0x80309E8C` |
/// | `StateID_Move` | `daSpinChildBase_c::StateID_Move` | `0x80355E10` | `0x80309EB0` |
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80063C40` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80063C40..0x80063E00` (448 bytes); the closing `bclr 20,0` is
///   the last word at `0x80063DFC`.
/// - Table: slot 37 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA74`).
/// - Prologue: `bl 0x802DCD74`; epilogue `bl 0x802DCDC0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills two `0x40`-byte records whose nodes are `0x80355DC0` and
/// `0x80355E00`, both sixteen-byte aligned (`descriptor = node + 0x10`), both
/// using vtable `0x80309F18` and destructor `0x80063E00`. Neither record has a
/// state getter, so `descriptor + 0x30` is left untouched.
///
/// The second record's in-range copy is the same reversed nine-word order as
/// the first; `r7` (source word 0) is simply kept live across the name setter.
///
/// # Safety
/// Both descriptor ranges, both nodes and the source blocks must be mapped and
/// writable as required by [`w229_install_state`].
///
/// # Confidence
/// high - both blocks repeat the wave-229 template, and the absence of a
/// `+0x30` store is visible in both.
pub unsafe fn fn_80063c40() {
    // StateID_SearchID (0x80309ED4)
    w229_install_state(
        0x8035_5DC0,
        0x8035_5DD0,
        0x8030_9ED4,
        0x8030_9E8C,
        0x8030_9F18,
        0x8006_3E00,
        None,
    );

    // StateID_Move (0x80309EF8)
    w229_install_state(
        0x8035_5E00,
        0x8035_5E10,
        0x8030_9EF8,
        0x8030_9EB0,
        0x8030_9F18,
        0x8006_3E00,
        None,
    );
}

/// Initialises the two-word impulse record at `0x80429D08`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800660A0` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x800660A0..0x800660B8` (24 bytes); the closing `bclr 20,0` is
///   the last word at `0x800660B4`.
/// - Table: slot 38 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA78`).
/// - Shape: a frameless leaf - two `lfs`, one address materialisation with
///   `addi r3,r13,-0x5978`, and two `stfs`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `9.0f` at `0x80429D08` (`r13 - 0x5978`) and `12.0f` at `0x80429D0C`.
/// Those two words are the vector that the player code passes as one impulse
/// through `0x800651C0` (see [`crate::game::bases`]).
///
/// # Safety
/// `0x80429D08..0x80429D10` must be writable and both `.sdata2` words
/// readable.
///
/// # Confidence
/// high - six instructions, all literal.
pub unsafe fn fn_800660a0() {
    platform::write_f32(SDA1_BASE - 0x5978, platform::read_f32(SDA2_IMPULSE_A_C230));
    platform::write_f32(SDA1_BASE - 0x5974, platform::read_f32(SDA2_IMPULSE_B_C230));
}

/// Clears the head of the `0x28`-byte record list at `0x80429D38`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80066360` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x80066360..0x80066374` (20 bytes); the closing `bclr 20,0` is
///   the last word at `0x80066370`.
/// - Table: slot 39 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA7C`).
/// - Shape: a frameless leaf - `addi r3,r13,-0x5948`, `li r0,0`, two `stw`.
///
/// # Arguments
/// None. The address is `r13`-relative and fixed at link time.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Zeroes `0x80429D38` and `0x80429D3C`: the head and the second word of the
/// `0x28`-byte record list that [`crate::game::player`] allocates into and
/// walks.
///
/// # Safety
/// `0x80429D38..0x80429D40` must be writable.
///
/// # Confidence
/// high - three data instructions, all literal.
pub unsafe fn fn_80066360() {
    platform::write_u32(SDA1_BASE - 0x5948, 0);
    platform::write_u32(SDA1_BASE - 0x5944, 0);
}

/// Initialises the static parameter block at `0x8030F020`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8006C020` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x8006C020..0x8006C090` (112 bytes); the closing `bclr 20,0` is
///   the last word at `0x8006C08C`.
/// - Table: slot 41 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA84`).
/// - Shape: byte-for-byte the same sequence as [`fn_8005d750`], with the
///   block, registry node and SDA destination moved.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0x17` at `0x8030F02C`, `2160.0f`/`-30.0f`/`-478.0f` at
/// `0x8030F038`/`0x8030F03C`/`0x8030F040`, pushes
/// `0x802DC780(0, 0x8006C090, 0x80355EE8)`, and copies `0x26` from
/// `SDA2_BASE - 0x60FC` to `0x80429D58` (`SDA1_BASE - 0x5928`).
///
/// # Observed data
/// The block already holds `{6, 0x80427920, 6, 0, 4, 0x80427928, 0, 0, 0}` in
/// the image. The dead stack copies at `r1+8..0x10` are omitted, as in
/// [`fn_8005d750`].
///
/// # Safety
/// The block at `0x8030F020` must be writable through `+0x40`, and the two SDA
/// words readable.
///
/// # Confidence
/// high - every load, store and call is literal; only the dead stack copies
/// are elided.
pub unsafe fn fn_8006c020() {
    platform::write_u32(0x8030_F02C, platform::read_u32(SDA2_BLOCK_WORD_C230));
    platform::write_f32(0x8030_F038, platform::read_f32(SDA2_BLOCK2_F0_C230));
    platform::write_f32(0x8030_F03C, platform::read_f32(SDA2_BLOCK2_F1_C230));
    platform::write_f32(0x8030_F040, platform::read_f32(SDA2_BLOCK2_F2_C230));
    platform::call3(FN_802DC780, 0, 0x8006_C090, 0x8035_5EE8);
    platform::write_u32(SDA1_BASE - 0x5928, platform::read_u32(SDA2_TAIL_WORD_C230));
}

/// Clears the head/tail pair at `0x80429DA0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8006CF20` and the export
///   tree has no file for it; the body was read from `work/dol/main.elf`.
/// - Extent: `0x8006CF20..0x8006CF34` (20 bytes); the closing `bclr 20,0` is
///   the last word at `0x8006CF30`.
/// - Table: slot 42 of the 181-pointer `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA88`).
/// - Shape: a frameless leaf - `addi r3,r13,-0x58E0`, `li r0,0`, two `stw`.
///
/// # Arguments
/// None. The address is `r13`-relative and fixed at link time.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Zeroes `0x80429DA0` and `0x80429DA4`, the head/tail pair of the SDA list
/// that [`crate::game::player`] pushes nodes through `0x80160FE0`.
///
/// # Safety
/// `0x80429DA0..0x80429DA8` must be writable.
///
/// # Confidence
/// high - three data instructions, all literal.
pub unsafe fn fn_8006cf20() {
    platform::write_u32(SDA1_BASE - 0x58E0, 0);
    platform::write_u32(SDA1_BASE - 0x58DC, 0);
}

// ===========================================================================
// Wave 230, lane D - `.ctors` slots 43..56: the static object/data constructors
// ===========================================================================
//
// This is the second half of the `0x802ED9E0` constructor table. The table
// itself, its 181 entries and its walker
// [`crate::runtime::os::run_init_table`] are described in the lane B and lane C
// sections above. Lane C covered slots 15..28 (the `daEn*_c` state-descriptor
// installers); the slots below are the next fourteen entries and belong to
// completely different translation units: leaf vector/matrix data fills, small
// lazy-initialisation guards, one `d_profile` instance, one static object graph
// and two `dEnemyCarry_c` state records.
//
// ## Slot map (slots 43..56 of `0x802ED9E0`)
//
// | slot | table word | target | shape | owner |
// |---:|---|---|---|---|
// | 43 | `0x802EDA8C` | `0x80076BB0` | empty constructor | below |
// | 44 | `0x802EDA90` | `0x8007E170` | one SDA word | below |
// | 45 | `0x802EDA94` | `0x8007EC20` | 62-row float table + 4 registrations | below |
// | 46 | `0x802EDA98` | `0x80087100` | vector broadcast into three objects | below |
// | 47 | `0x802EDA9C` | `0x80089ED0` | stage-camera defaults | [`crate::game::stage::stage_camera_defaults`] (already translated) |
// | 48 | `0x802EDAA0` | `0x8008DC40` | lazy one-shot initialiser | below |
// | 49 | `0x802EDAA4` | `0x8008DEB0` | two-word clear | below |
// | 50 | `0x802EDAA8` | `0x8008DFE0` | empty constructor | below |
// | 51 | `0x802EDAAC` | `0x8008F8C0` | `d_profile` instance | below |
// | 52 | `0x802EDAB0` | `0x80092BE0` | two-word clear | below |
// | 53 | `0x802EDAB4` | `0x80094950` | static object graph | below |
// | 54 | `0x802EDAB8` | `0x8009A320` | seven `dEnBoss` records | [`crate::game::stage::fn_8009a320`] (already translated) |
// | 55 | `0x802EDABC` | `0x8009DD60` | eleven `dEnBossKoopaJrBase` records | [`crate::game::stage::fn_8009dd60`] (already translated) |
// | 56 | `0x802EDAC0` | `0x8009F070` | two `dEnemyCarry_c` records | below |
//
// Slots 47, 54 and 55 already had translated bodies in `src/game/stage.rs`
// (`stage_camera_defaults` and the two `dEnBoss` registrars), so lane D did not
// duplicate them. The eleven remaining bodies are the new work here.
//
// ## Measured extents (read from the raw words in `work/dol/main.elf`)
//
// | slot | code extent | bytes | note |
// |---:|---|---:|---|
// | 43 | `0x80076BB0..0x80076BB4` | 4 | single `blr` |
// | 44 | `0x8007E170..0x8007E17C` | 12 | `lis`/`stw`/`blr` |
// | 45 | `0x8007EC20..0x8007F6D4` | 2740 | `stwu` frame, final `blr` |
// | 46 | `0x80087100..0x800872D4` | 468 | three-word vector broadcast |
// | 47 | `0x80089ED0..0x8008A004` | 308 | tabled, skip |
// | 48 | `0x8008DC40..0x8008DC78` | 56 | early `bnelr` plus body |
// | 49 | `0x8008DEB0..0x8008DEC4` | 20 | leaf |
// | 50 | `0x8008DFE0..0x8008DFE4` | 4 | single `blr` |
// | 51 | `0x8008F8C0..0x8008F914` | 84 | `d_profile` constructor |
// | 52 | `0x80092BE0..0x80092BF4` | 20 | leaf |
// | 53 | `0x80094950..0x80094DDC` | 1164 | object graph |
// | 54 | `0x8009A320..0x8009A8FC` | 1500 | tabled, skip |
// | 55 | `0x8009DD60..0x8009E66C` | 2316 | tabled, skip |
// | 56 | `0x8009F070..0x8009F250` | 480 | two state records |
//
// Each extent runs from the entry point to the last `bclr 20,0` of the body,
// inclusive. For the three tabled slots the measured extents match the
// `functions.tsv` sizes exactly (`308`, `1500`, `2316`), which cross-checks the
// measurement on the untabled ones. New work: 11 bodies, 5052 bytes.
//
// The register-save prologue `0x802DCD6C`/`0x802DCD70` and the matching restore
// `0x802DCDB8`/`0x802DCDBC` are not functions: they preserve `r25..r31` and
// leave `r3` untouched, so they are omitted from the bodies below, as in the
// lane B/C sections.
// ---------------------------------------------------------------------------

/// `r13 - 0x58B8` - word initialised by [`fn_8007e170`].
///
/// # Original
/// `lis r0,0x1` / `stw r0,-0x58B8(r13)`; the low half is zero, so the stored
/// word is `0x00010000`.
///
/// # Confidence
/// high - one literal store.
const D230_SDA_58B8: usize = SDA1_BASE - 0x58B8;

/// `r13 - 0x5818` - the byte guard used by [`fn_8008dc40`].
///
/// # Original
/// `lbz r0,-0x5818(r13)` / `extsb. r0,r0` / `bnelr`, then `stb r0,-0x5818(r13)`
/// with `r0 = 1`.
///
/// # Confidence
/// high - the whole body is the standard lazy-init guard.
const D230_SDA_5818: usize = SDA1_BASE - 0x5818;

/// `r13 - 0x5808` - two-word BSS pair cleared by [`fn_8008deb0`].
///
/// # Confidence
/// high - two zero stores.
const D230_SDA_5808: usize = SDA1_BASE - 0x5808;

/// `r13 - 0x5790` - two-word BSS pair cleared by [`fn_80092be0`].
///
/// # Confidence
/// high - two zero stores.
const D230_SDA_5790: usize = SDA1_BASE - 0x5790;

/// `0x8030F520` - first row of the matrix table written by [`fn_8007ec20`].
///
/// # Original
/// `lis r31,0x8031` / `addi r31,r31,-2784` (`0x8030F520`). The four blocks
/// start at `+0x00`, `+0x40`, `+0x3C0` and `+0x620`, each record `0x20` bytes
/// with five floats at `+0x08..+0x18`.
///
/// # Confidence
/// high - all four base displacements are literal.
const D230_MATRIX_BASE: usize = 0x8030_F520;

/// `0x80355F30` - first of the four 12-byte registry nodes linked by
/// [`fn_8007ec20`].
///
/// # Confidence
/// high - `lis r30,0x8035` / `addi r30,r30,24368`; nodes are `+0x00`, `+0x0C`,
/// `+0x18` and `+0x24`.
const D230_REGISTRY_BASE: usize = 0x8035_5F30;

/// `0x80355FB8` - the static `d_profile` instance built by [`fn_8008f8c0`].
///
/// # Confidence
/// high - `lis r31,0x8035` / `addi r31,r31,24504`.
const D230_PROFILE: usize = 0x8035_5FB8;

/// `0x80355FA8` - the registry node that precedes [`D230_PROFILE`].
///
/// # Confidence
/// high - `addi r5,r31,24488` = `D230_PROFILE - 0x10`.
const D230_PROFILE_NODE: usize = 0x8035_5FA8;

/// `0x80310EC8` - the `"d_profile"` name passed to the constructor.
///
/// # Original
/// read from the image; the string is `"d_profile"` (terminated at
/// `0x80310ED2`).
///
/// # Confidence
/// high - string read from `work/dol/main.elf`.
const D230_PROFILE_NAME: usize = 0x8031_0EC8;

/// `0x80160080` - `d_profile` constructor (`FUN_80160080`).
///
/// observed: `(object, name, param)` stores the vtable `0x80328AE8` at
/// `object + 0x0C`, the name at `+0x20` and `param` at `+0x30`.
const FN_80160080_D230: usize = 0x8016_0080;

/// `0x80160110` - `d_profile` destructor (`FUN_80160110`).
///
/// observed: releases the `+0x34` sub-object, then the profile, and frees the
/// block when `param_2 > 0`.
const FN_80160110_D230: usize = 0x8016_0110;

/// `0x802D7AA0` - base-object constructor used by the graph in [`fn_80094950`].
///
/// observed: installs `0x803507B8` at `object + 0x00`, clears `+0x24`/`+0x04`,
/// runs the `+0x74`/`+0x7C` sub-constructors and dispatches `+0x98`.
const FN_802D7AA0_D230: usize = 0x802D_7AA0;

/// The 62 five-float rows written by [`fn_8007ec20`].
///
/// # Original
/// Every value is loaded from `r2 - 0x7230..0x71FC` in `.sdata2`
/// (`0.0`, `8.0`, `-8.0`, `32.0`, `24.0`, `48.0`, `16.0`, `-16.0`) and
/// stored at `row + 8, +12, +16, +20, +24` of a `0x20`-byte record. The rows
/// are grouped exactly as the four fills: 2, then 28, then 19, then 13.
///
/// # Confidence
/// high - 62 rows x 5 stores, every offset and value read from the body.
const D230_ROWS: [[f32; 5]; 62] = [
    // block 0: 2 rows
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
    // block 1: 28 rows
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, 0.0, 0.0, 24.0, 48.0],
    [8.0, 0.0, 0.0, 32.0, 48.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [16.0, 0.0, 0.0, 48.0, 48.0],
    [16.0, 0.0, 0.0, 48.0, 48.0],
    [16.0, -16.0, 0.0, 48.0, 48.0],
    [0.0, -16.0, 0.0, 48.0, 48.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -16.0, 0.0, 32.0, 48.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [8.0, -16.0, 0.0, 32.0, 48.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
    // block 2: 19 rows
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -16.0, 0.0, 32.0, 48.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [8.0, -16.0, 0.0, 32.0, 48.0],
    [16.0, 0.0, 0.0, 48.0, 48.0],
    [16.0, 0.0, 0.0, 48.0, 48.0],
    [16.0, -16.0, 0.0, 48.0, 48.0],
    [0.0, -16.0, 0.0, 48.0, 48.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
    // block 3: 13 rows
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -16.0, 0.0, 32.0, 48.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [16.0, -8.0, 0.0, 48.0, 32.0],
    [8.0, -16.0, 0.0, 32.0, 48.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [8.0, -8.0, 0.0, 32.0, 32.0],
    [0.0, 0.0, 0.0, 0.0, 0.0],
];

/// Field offsets of `0x803104D8` that receive the `0x802FD328` vector in
/// [`fn_80087100`].
///
/// # Original
/// 60 triple stores in ascending order; the gaps `0x84..0xA4`, `0xB4..0xF8`
/// and `0x180..0x188` are not written.
///
/// # Confidence
/// high - read from the store list of the body.
const D230_VEC3_A: [usize; 60] = [
    0x60, 0x64, 0x68, 0x6C, 0x70, 0x74, 0x78, 0x7C, 0x80, 0xA8, 0xAC, 0xB0,
    0xFC, 0x100, 0x104, 0x108, 0x10C, 0x110, 0x114, 0x118, 0x11C, 0x120,
    0x124, 0x128, 0x12C, 0x130, 0x134, 0x138, 0x13C, 0x140, 0x144, 0x148,
    0x14C, 0x150, 0x154, 0x158, 0x15C, 0x160, 0x164, 0x168, 0x16C, 0x170,
    0x174, 0x178, 0x17C, 0x18C, 0x190, 0x194, 0x198, 0x19C, 0x1A0, 0x1A4,
    0x1A8, 0x1AC, 0x1B0, 0x1B4, 0x1B8, 0x1BC, 0x1C0, 0x1C4,
];

/// Field offsets of `0x803106B8` that receive the `0x802FD328` vector in
/// [`fn_80087100`].
///
/// # Original
/// 24 triple stores: `+0x0C..+0x74`, `+0xA8..+0xB0`; the gaps are the fields
/// the compiler did not touch.
///
/// # Confidence
/// high - read from the store list of the body.
const D230_VEC3_B: [usize; 24] = [
    0x0C, 0x10, 0x14, 0x24, 0x28, 0x2C, 0x3C, 0x40, 0x44, 0x48, 0x4C, 0x50,
    0x54, 0x58, 0x5C, 0x60, 0x64, 0x68, 0x6C, 0x70, 0x74, 0xA8, 0xAC, 0xB0,
];

/// Consecutive word offsets of `0x80355F60` that receive the `0x802FD328`
/// vector in [`fn_80087100`].
///
/// # Original
/// 18 triple stores at `+0x00..+0x44`, i.e. the whole three-word stride runs
/// contiguously.
///
/// # Confidence
/// high - read from the store list of the body.
const D230_VEC3_C: [usize; 18] = [
    0x00, 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28, 0x2C,
    0x30, 0x34, 0x38, 0x3C, 0x40, 0x44,
];

/// Fills a run of `0x20`-byte rows with five floats each.
///
/// # Original
/// Host-side factoring of the four repeated fill blocks in
/// [`fn_8007ec20`]. The compiler stores the five words at `row + 8, +12, +16,
/// +20, +24` for every `0x20`-byte row; the order of the stores is preserved.
///
/// # Arguments
/// * `start` - address of the first row's first float (`row + 8`).
/// * `first` - index of the first row in [`D230_ROWS`].
/// * `count` - number of consecutive rows.
///
/// # Safety
/// `start..start + count * 0x20` must be writable.
///
/// # Confidence
/// high - mechanical loop over the measured offsets.
#[inline]
unsafe fn d230_fill_rows(start: usize, first: usize, count: usize) {
    let mut row = 0usize;
    while row < count {
        let values = &D230_ROWS[first + row];
        let base = start + row * 0x20;
        let mut field = 0usize;
        while field < values.len() {
            platform::write_f32(base + field * 4, values[field]);
            field += 1;
        }
        row += 1;
    }
}

/// Writes one three-word vector at each listed offset of `base`.
///
/// # Original
/// Host-side factoring of the repeated `stw rX,base+off` triples in
/// [`fn_80087100`]. The vector is the `.rodata` word triple at `0x802FD328`;
/// the object field offsets are listed in the three `D230_VEC3_*` tables below.
///
/// # Arguments
/// * `base` - object base address.
/// * `offsets` - field offsets, in the order the compiler emitted them.
/// * `value` - the three words to store at `offset`, `offset + 4`,
///   `offset + 8`.
///
/// # Safety
/// `base + offset .. base + offset + 12` must be writable for every offset.
///
/// # Confidence
/// high - one-for-one transcription of the store triples.
#[inline]
unsafe fn d230_store_vec3(base: usize, offsets: &[usize], value: [u32; 3]) {
    let mut index = 0usize;
    while index < offsets.len() {
        let field = base + offsets[index];
        platform::write_u32(field, value[0]);
        platform::write_u32(field + 4, value[1]);
        platform::write_u32(field + 8, value[2]);
        index += 1;
    }
}

/// Builds one of the `dEnemyCarry_c` state records.
///
/// # Original
/// Host-side factoring of the two identical blocks in [`fn_8009f070`]. It is
/// the same template as lane C's `w229_install_state`: the state getter runs
/// first, then `0x8015F7C0(descriptor, name)`, the nine-word copy in the
/// reversed three-group order, the vtable, the state object at `+0x30`, and
/// the `0x802DC780` registration.
///
/// # Arguments
/// * `node` - 12-byte registration node (`descriptor - 0x0C`).
/// * `descriptor` - state descriptor base.
/// * `name` - `StateID_*` string.
/// * `source` - nine-word parameter block.
/// * `vtable` - `dEnemyCarry_c` state vtable (`0x8031341C`).
/// * `getter` - state-object getter (`0x8009EAF0` or `0x8009EB00`).
///
/// # Safety
/// `node`, `descriptor` and `source` must be writable/readable as in
/// [`w229_install_state`].
///
/// # Confidence
/// high - both records repeat [`w229_install_state`] exactly; the only
/// difference is the parameter block, name and getter.
#[inline]
unsafe fn d230_install_carry_state(
    node: usize,
    descriptor: usize,
    name: usize,
    source: usize,
    vtable: u32,
    getter: usize,
) {
    w229_install_state(
        node,
        descriptor,
        name,
        source,
        vtable,
        0x8009_F2B0,
        Some(getter),
    );
}

/// Empty `.ctors` constructor.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80076BB0`.
/// - Extent: `0x80076BB0..0x80076BB4` (4 bytes); the body is a single
///   `bclr 20,0` (`blr`).
/// - Table: slot 43 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA8C`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None. The translation unit registered an empty static constructor.
///
/// # Safety
/// Safe to call; it touches no memory.
///
/// # Confidence
/// high - the measured extent is one instruction.
pub unsafe fn fn_80076bb0() {}

/// Initialises one small-data word to `0x00010000`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8007E170`.
/// - Extent: `0x8007E170..0x8007E17C` (12 bytes); `lis r0,0x1`,
///   `stw r0,-0x58B8(r13)`, `blr`.
/// - Table: slot 44 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA90`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `0x00010000` at [`D230_SDA_58B8`].
///
/// # Safety
/// [`D230_SDA_58B8`] must be writable; it is the module's own SDA1 area.
///
/// # Confidence
/// high - literal store with no control flow.
pub unsafe fn fn_8007e170() {
    platform::write_u32(D230_SDA_58B8, 0x0001_0000);
}

/// Writes the 62-row orientation/matrix table and registers its four nodes.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8007EC20`.
/// - Extent: `0x8007EC20..0x8007F6D4` (2740 bytes); `stwu` frame with
///   `r30`/`r31` saved, final `blr` at `0x8007F6D0`.
/// - Table: slot 45 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA94`).
/// - Prologue: `stwu r1,-0x4F0(r1)` plus the inline `mfspr`/`stw` pair; no
///   register-save thunk. The repeated `stfs` stores to `r1+...` are the
///   compiler spilling the eight `.sdata2` constants; they are register
///   allocation artefacts and are not reproduced.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills 62 five-float rows starting at [`D230_MATRIX_BASE`]`+0x08` (blocks of
/// 2, 28, 19 and 13 rows) and links four 12-byte nodes at
/// [`D230_REGISTRY_BASE`]`+0x00/+0x0C/+0x18/+0x24`, with destructors
/// `0x8007F6E0`, `0x8007F740`, `0x8007F760` and `0x8007F780`.
///
/// # Safety
/// The matrix table and the four nodes must be writable; both are the
/// module's own `.data`/BSS.
///
/// # Confidence
/// high - the four fills are the same five-store row template and every
/// offset/constant was read from the body; the four registrations follow each
/// fill in program order.
pub unsafe fn fn_8007ec20() {
    // block 0: two rows, then node +0x00
    d230_fill_rows(D230_MATRIX_BASE + 0x08, 0, 2);
    platform::call3(FN_802DC780, 0, 0x8007_F6E0, D230_REGISTRY_BASE as u32);
    // block 1: 28 rows, then node +0x0C
    d230_fill_rows(D230_MATRIX_BASE + 0x48, 2, 28);
    platform::call3(FN_802DC780, 0, 0x8007_F740, (D230_REGISTRY_BASE + 0x0C) as u32);
    // block 2: 19 rows, then node +0x18
    d230_fill_rows(D230_MATRIX_BASE + 0x3C8, 30, 19);
    platform::call3(FN_802DC780, 0, 0x8007_F760, (D230_REGISTRY_BASE + 0x18) as u32);
    // block 3: 13 rows, then node +0x24
    d230_fill_rows(D230_MATRIX_BASE + 0x628, 49, 13);
    platform::call3(FN_802DC780, 0, 0x8007_F780, (D230_REGISTRY_BASE + 0x24) as u32);
}

/// Broadcasts one three-word `.rodata` vector into three static objects.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80087100`.
/// - Extent: `0x80087100..0x800872D4` (468 bytes); `stwu r1,-0x20(r1)` and
///   a final `blr` at `0x800872D0`.
/// - Table: slot 46 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDA98`).
/// - observed: `lwzu r6,-0x2CD8(r7)` with `r7 = 0x80300000` loads
///   `0x802FD328`, then `r5 = [r7+4]`, `r0 = [r7+8]`; all three words are
///   copied as a unit into every listed field.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the vector `{0x00000000, 0x00000000, 0x00000000}` at 60 offsets of
/// `0x803104D8`, 24 offsets of `0x803106B8` and 18 consecutive words at
/// `0x80355F60`.
///
/// # Safety
/// Those three objects must be writable; they are the module's own data.
///
/// # Confidence
/// high - every destination offset is listed literally in the body and the
/// source triple is read from `0x802FD328` in `work/dol/main.elf`.
pub unsafe fn fn_80087100() {
    let value = [
        platform::read_u32(0x802F_D328),
        platform::read_u32(0x802F_D32C),
        platform::read_u32(0x802F_D330),
    ];
    d230_store_vec3(0x8031_04D8, &D230_VEC3_A, value);
    d230_store_vec3(0x8031_06B8, &D230_VEC3_B, value);
    d230_store_vec3(0x8035_5F60, &D230_VEC3_C, value);
}

/// One-shot initialiser of the `0x80310DA0` descriptor pair.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8008DC40`.
/// - Extent: `0x8008DC40..0x8008DC78` (56 bytes); the guard `bnelr` at
///   `0x8008DC48` returns when the flag is already set.
/// - Table: slot 48 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAA0`).
/// - observed: `lbz r0,-0x5818(r13)`, `extsb. r0,r0`, `bnelr`; then
///   `lis r4,0x8031`/`addi r4,r4,3488` = `0x80310DA0`, two literal stores and
///   `stb r0,-0x5818(r13)` with `r0 = 1`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// On the first call only, stores `0x00A000A0` at `0x80310DA4`,
/// `0xA00000A0` at `0x80310DA8` and sets [`D230_SDA_5818`] to 1.
///
/// # Safety
/// The descriptor pair and the guard byte must be writable.
///
/// # Confidence
/// high - the guard and the two literal stores are complete.
pub unsafe fn fn_8008dc40() {
    if platform::read_u8(D230_SDA_5818) != 0 {
        return;
    }
    platform::write_u32(0x8031_0DA4, 0x00A0_00A0);
    platform::write_u32(0x8031_0DA8, 0xA000_00A0);
    platform::write_u8(D230_SDA_5818, 1);
}

/// Clears the two-word object at [`D230_SDA_5808`].
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8008DEB0`.
/// - Extent: `0x8008DEB0..0x8008DEC4` (20 bytes); `addi r3,r13,-22536`,
///   `li r0,0`, `stw r0,-0x5808(r13)`, `stw r0,4(r3)`, `blr`.
/// - Table: slot 49 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAA4`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Zeroes words `0x80429E78`/`0x80429E7C` (the SDA1 pair).
///
/// # Safety
/// [`D230_SDA_5808`]`..+8` must be writable.
///
/// # Confidence
/// high - two zero stores, no control flow.
pub unsafe fn fn_8008deb0() {
    platform::write_u32(D230_SDA_5808, 0);
    platform::write_u32(D230_SDA_5808 + 4, 0);
}

/// Empty `.ctors` constructor.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8008DFE0`.
/// - Extent: `0x8008DFE0..0x8008DFE4` (4 bytes); the body is a single
///   `bclr 20,0` (`blr`).
/// - Table: slot 50 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAA8`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Safety
/// Safe to call; it touches no memory.
///
/// # Confidence
/// high - the measured extent is one instruction.
pub unsafe fn fn_8008dfe0() {}

/// Builds the static `d_profile` instance.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8008F8C0`; the callees
///   are `FUN_80160080` and `FUN_802dc780`.
/// - Extent: `0x8008F8C0..0x8008F914` (84 bytes); `stwu r1,-0x10(r1)` with
///   `r31` saved and a final `blr` at `0x8008F910`.
/// - Table: slot 51 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAAC`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80160080(`[`D230_PROFILE`]`, `[`D230_PROFILE_NAME`]`, 0)` - which
/// installs the `d_profile` vtable `0x80328AE8`, the name `"d_profile"` and a
/// zero third argument - then links the object through
/// `0x802DC780(profile, 0x80160110, profile - 0x10)`.
///
/// # Safety
/// The profile object is writable `.bss`; the name string is readable.
///
/// # Confidence
/// high - the two call argument sets and the node displacement are literal.
pub unsafe fn fn_8008f8c0() {
    platform::call3(
        FN_80160080_D230,
        D230_PROFILE as u32,
        D230_PROFILE_NAME as u32,
        0,
    );
    platform::call3(
        FN_802DC780,
        D230_PROFILE as u32,
        FN_80160110_D230 as u32,
        D230_PROFILE_NODE as u32,
    );
}

/// Clears the two-word object at [`D230_SDA_5790`].
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80092BE0`.
/// - Extent: `0x80092BE0..0x80092BF4` (20 bytes); `addi r3,r13,-22416`,
///   `li r0,0`, `stw r0,-0x5790(r13)`, `stw r0,4(r3)`, `blr`.
/// - Table: slot 52 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAB0`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Zeroes words `0x80429EF0`/`0x80429EF4` (the SDA1 pair).
///
/// # Safety
/// [`D230_SDA_5790`]`..+8` must be writable.
///
/// # Confidence
/// high - two zero stores, no control flow.
pub unsafe fn fn_80092be0() {
    platform::write_u32(D230_SDA_5790, 0);
    platform::write_u32(D230_SDA_5790 + 4, 0);
}

/// Constructs and registers the static object graph at `0x803560D0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80094950`; the callees
///   are `FUN_802d7aa0`, `FUN_802dc780` and `FUN_802dc9a0`.
/// - Extent: `0x80094950..0x80094DDC` (1164 bytes); `stwu r1,-0x30(r1)` with
///   `r30`/`r31` saved and a final `blr` at `0x80094DD8`.
/// - Table: slot 53 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAB4`).
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds three small objects at `+0x0C/+0x12C/+0x24C`, eleven derived
/// objects at `+0x370 / +0x4B8 / +0x600 / +0x748 / +0x9B0 / +0xAF8 / +0xC40 /
/// +0xD88 / +0xED0 / +0x1018 / +0x1160` and one at `+0x88C`, constructs two
/// arrays of 14 elements with a 20-byte stride at `+0x12E0` and `+0x1408`, and one
/// final object at `+0x152C`. Every object is linked through `0x802DC780`.
///
/// # Safety
/// The whole `0x803560D0` region must be writable; the vtables and thunks used
/// here live in the module's `.data`/`.text`.
///
/// # Confidence
/// high - the offsets, vtables, destructors and counts are literal in the
/// body; the eleven derived objects repeat one template exactly.
pub unsafe fn fn_80094950() {
    // Three leaf objects, each `0x120` bytes: constructor, vtable, registration.
    let leaf = [0x0Cusize, 0x12C, 0x24C];
    let mut index = 0usize;
    while index < leaf.len() {
        let object = 0x8035_60D0 + leaf[index];
        platform::call1(FN_802D7AA0_D230, object as u32);
        platform::write_u32(object, 0x8032_97F8);
        platform::call3(FN_802DC780, object as u32, 0x8009_4DE0, (object - 0x0C) as u32);
        index += 1;
    }

    // The derived objects are built in table order, with the `+0x88C` leaf
    // object inserted between `+0x748` and `+0x9B0`; the registration list is
    // order-sensitive, so the split is kept.
    let derived = [
        0x370usize, 0x4B8, 0x600, 0x748, 0x9B0, 0xAF8, 0xC40, 0xD88, 0xED0,
        0x1018, 0x1160,
    ];
    let mut index = 0usize;
    while index < derived.len() {
        let object = 0x8035_60D0 + derived[index];
        platform::call1(FN_802D7AA0_D230, object as u32);
        platform::write_u32(object + 0x114, 0);
        platform::write_u32(object + 0x118, 0);
        platform::write_u8(object + 0x11C, 0);
        platform::write_u8(object + 0x11D, 0);
        platform::write_u32(object + 0x120, 0);
        platform::write_u32(object + 0x124, 0);
        platform::write_u32(object, 0x8032_989C);
        let vtable = platform::read_u32(object) as usize;
        let method = platform::read_u32(vtable + 0x98) as usize;
        platform::call1(method, object as u32);
        platform::call3(
            FN_802DC780,
            object as u32,
            0x8003_2A30,
            (object - 0x10) as u32,
        );
        if index == 3 {
            // The `+0x88C` object uses the leaf family and is registered here.
            let leaf_extra = 0x8035_60D0 + 0x88C;
            platform::call1(FN_802D7AA0_D230, leaf_extra as u32);
            platform::write_u32(leaf_extra, 0x8032_97F8);
            platform::call3(
                FN_802DC780,
                leaf_extra as u32,
                0x8009_4DE0,
                (leaf_extra - 0x0C) as u32,
            );
        }
        index += 1;
    }

    // Two arrays of 14 elements with a 20-byte stride.
    platform::call5(
        FN_802DC9A0,
        0x8035_73B0,
        0x8008_F140,
        0x8008_F170,
        20,
        14,
    );
    platform::call3(FN_802DC780, 0, 0x8009_4E40, 0x8035_73A0);
    platform::call5(
        FN_802DC9A0,
        0x8035_74D8,
        0x8008_F140,
        0x8008_F170,
        20,
        14,
    );
    platform::call3(FN_802DC780, 0, 0x8009_4E60, 0x8035_74C8);

    // Final object: vtable, two zero words, registration.
    let last = 0x8035_60D0 + 0x152C;
    platform::write_u32(last, 0x8031_0A78);
    platform::write_u32(last + 0x24, 0);
    platform::write_u32(last + 0x28, 0);
    platform::call3(FN_802DC780, last as u32, 0x8008_8FD0, (last - 0x0C) as u32);
}

/// Installs the two `dEnemyCarry_c` names (`Carry`, `Throw`).
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8009F070`.
/// - Extent: `0x8009F070..0x8009F250` (480 bytes); `stwu r1,-0x70(r1)` with
///   `r27..r31` saved through `0x802DCD70` and restored through
///   `0x802DCDBC`.
/// - Table: slot 56 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAC0`).
/// - observed: both records use the lane C nine-word copy order
///   (`6,7,8,3,4,5,0,1,2`), the vtable `0x8031341C`, the state getters
///   `0x8009EAF0`/`0x8009EB00` (both return `0x80377A04`) and the destructor
///   `0x8009F2B0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills two `0x40`-byte records at `0x80357B28` and `0x80357B68`, registering
/// them with `0x802DC780`.
///
/// # Safety
/// The two records and their nine-word source blocks must be writable/readable;
/// they are the module's own `.data`/BSS.
///
/// # Confidence
/// high - the two blocks repeat the lane C template exactly; the names read
/// `"dEnemyCarry_c::StateID_Carry"` and `"...::StateID_Throw"` from the image.
pub unsafe fn fn_8009f070() {
    d230_install_carry_state(
        0x8035_7B28,
        0x8035_7B34,
        0x8031_33DC,
        0x8031_3394,
        0x8031_341C,
        0x8009_EAF0,
    );
    d230_install_carry_state(
        0x8035_7B68,
        0x8035_7B74,
        0x8031_33FC,
        0x8031_33B8,
        0x8031_341C,
        0x8009_EB00,
    );
}

// ===========================================================================
// Wave 231 lane B: `.ctors` slots 57-70
// ===========================================================================
//
// Table `0x802ED9E0`; slots 57-70 are the words `0x802EDAC4..0x802EDAF8`.
//
// | slot | table word   | target       | extent | Rust body |
// |-----:|--------------|--------------|-------:|-----------|
// | 57 | `0x802EDAC4` | `0x800A1E20` | 1,704 B | [`fn_800a1e20`] |
// | 58 | `0x802EDAC8` | `0x800A6A00` | 1,704 B | [`fn_800a6a00`] |
// | 59 | `0x802EDACC` | `0x800A82C0` | 1,092 B | [`fn_800a82c0`] |
// | 60 | `0x802EDAD0` | `0x800AED40` | 5,784 B | [`fn_800aed40`] |
// | 61 | `0x802EDAD4` | `0x800B1490` |    72 B | [`fn_800b1490`] |
// | 62 | `0x802EDAD8` | `0x800B28F0` |   824 B | [`crate::game::stage::fn_800b28f0`] (already translated) |
// | 63 | `0x802EDADC` | `0x800B5840` |   120 B | [`fn_800b5840`] |
// | 64 | `0x802EDAE0` | `0x800B80C0` |    24 B | [`fn_800b80c0`] |
// | 65 | `0x802EDAE4` | `0x800B8340` |   256 B | [`fn_800b8340`] |
// | 66 | `0x802EDAE8` | `0x800B8FA0` |   168 B | [`fn_800b8fa0`] |
// | 67 | `0x802EDAEC` | `0x800BA5E0` | 1,236 B | [`crate::game::stage::fn_800ba5e0`] (already translated) |
// | 68 | `0x802EDAF0` | `0x800BBC90` |   124 B | [`crate::game::stage::fn_800bbc90`] (already translated) |
// | 69 | `0x802EDAF4` | `0x800BFE50` |    28 B | [`fn_800bfe50`] |
// | 70 | `0x802EDAF8` | `0x800C75B0` | 4,772 B | [`crate::game::stage::fn_800c75b0`] (already translated) |
//
// Slots 62, 67, 68 and 70 already had translated bodies in `src/game/stage.rs`
// when this lane ran, so they are referenced rather than duplicated. The
// remaining ten are the bodies below. Extents are prologue-to-`blr` inclusive,
// from the raw words in `work/wave231/laneB_disasm_57_70.txt`.
//
// Slots 57-60 follow the same `dSys`-style descriptor template that wave 229
// lane B documented in [`w229b_build_state`]: per record a name pointer through
// `0x8015F7C0`, a straight nine-word payload copy, the vtable word, a 12-byte
// getter returning the type tag, and the `0x802DC780` registry node. The getters
// are `0x8009F7C0..0x8009F830`, `0x800A2B70..0x800A2BE0`, `0x800A7DA0..0x800A7DE0`,
// `0x800A8710..0x800A8890` and `0x800B04A0..0x800B04C0`; every one of them is a
// two-instruction `return &DAT_80377a04` except the three `0x800B04Ax` getters
// used by slot 60, which return the per-state tag words listed below.

/// Builds one state record in the slots 57-60 layout.
///
/// The record lives at `dest`, with the state descriptor at `dest + 0x0C`; the
/// registry node written by `0x802DC780` is `dest` itself. This is exactly
/// [`w229b_build_state`] with the destination offsets folded in.
///
/// # Safety
/// `dest..dest + 0x40` must be writable, `data` readable through `+0x20`, and
/// `name` a valid C string.
unsafe fn w231b_build_state(
    dest: usize,
    name: usize,
    data: usize,
    vtable: u32,
    tag: u32,
    release: u32,
) {
    w229b_build_state(dest + 0x0C, name, data, dest, vtable, tag, release);
}

/// Builds the eight `dEn_c` death-state records at `0x80357BA8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800A1E20`.
/// - Extent: `0x800A1E20..0x800A24C8` (1,704 bytes); `stwu r1,-0x150(r1)`
///   with `r25..r31` saved through `0x802DCD6C` and restored through
///   `0x802DCDB8`.
/// - Table: slot 57 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAC4`).
/// - observed: eight repetitions of the `w231b_build_state` template with
///   destination stride `0x40`, payload stride `0x24` and getter stride `0x10`;
///   vtable `0x80313698`, tag `0x80377A04`, release `0x800A2530`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Registers the eight `dEn_c` death states `DieFumi`, `DieFall`, `DieBigFall`,
/// `DieSmoke`, `DieIceVanish`, `DieYoshiFumi`, `DieGoal` and `DieOther`.
///
/// # Safety
/// `0x80357BA8..0x80357DA8` and the payload table must be readable/writable
/// `.data`/BSS; they are the module's own storage.
///
/// # Confidence
/// high - the eight blocks repeat one template exactly and the name strings sit
/// at the literal pointers below.
pub unsafe fn fn_800a1e20() {
    const NAMES: [usize; 8] = [
        0x8031_35C8,
        0x8031_35E0,
        0x8031_35F8,
        0x8031_3618,
        0x8031_3630,
        0x8031_364C,
        0x8031_3668,
        0x8031_3680,
    ];
    let mut index = 0usize;
    while index < NAMES.len() {
        w231b_build_state(
            0x8035_7BA8 + index * 0x40,
            NAMES[index],
            0x8031_34A8 + index * 0x24,
            0x8031_3698,
            W229B_TAG_SHARED,
            0x800A_2530,
        );
        index += 1;
    }
}

/// Builds the eight `dEnJrClownBase_c` state records at `0x80357DD0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800A6A00`.
/// - Extent: `0x800A6A00..0x800A70A8` (1,704 bytes); `stwu r1,-0x150(r1)`
///   with the same `0x802DCD6C` save sequence as [`fn_800a1e20`].
/// - Table: slot 58 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAC8`).
/// - observed: eight repetitions of the template with destination base
///   `0x80357DD0`, payload base `0x80313C30` (stride `0x24`) and getters
///   `0x800A2B70..0x800A2BE0`; vtable `0x80313E68`, tag `0x80377A04`,
///   release `0x800A7110`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Registers the `dEnJrClownBase_c` states `DemoWait`, `DemoAwake`, `Move`,
/// `Damaged`, `FumiHit`, `FireReady`, `Fire` and `DieFall`.
///
/// # Safety
/// `0x80357DD0..0x80357FD0` and the payload table must be readable/writable.
///
/// # Confidence
/// high - the template and the eight name pointers are literal.
pub unsafe fn fn_800a6a00() {
    const NAMES: [usize; 8] = [
        0x8031_3D50,
        0x8031_3D74,
        0x8031_3D98,
        0x8031_3DB8,
        0x8031_3DDC,
        0x8031_3E00,
        0x8031_3E24,
        0x8031_3E44,
    ];
    let mut index = 0usize;
    while index < NAMES.len() {
        w231b_build_state(
            0x8035_7DD0 + index * 0x40,
            NAMES[index],
            0x8031_3C30 + index * 0x24,
            0x8031_3E68,
            W229B_TAG_SHARED,
            0x800A_7110,
        );
        index += 1;
    }
}

/// Builds the five `dEn_c` ice/eat state records at `0x80357FF8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800A82C0`.
/// - Extent: `0x800A82C0..0x800A8704` (1,092 bytes); `stwu r1,-0xE0(r1)` with
///   `r25..r31` saved through `0x802DCD6C`.
/// - Table: slot 59 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDACC`).
/// - observed: five repetitions of the template with destination base
///   `0x80357FF8`, payload base `0x80313ED0` (stride `0x24`) and getters
///   `0x800A7DA0..0x800A7DE0`; vtable `0x80313698`, tag `0x80377A04`,
///   release `0x800A2530`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Registers the `dEn_c` states `Ice`, `HitSpin`, `EatIn`, `EatNow` and
/// `EatOut`.
///
/// # Safety
/// `0x80357FF8..0x80358138` and the payload table must be readable/writable.
///
/// # Confidence
/// high - five identical blocks, literal name pointers and the shared
/// `dEn_c` vtable.
pub unsafe fn fn_800a82c0() {
    const NAMES: [usize; 5] = [
        0x8031_3F84,
        0x8031_3F98,
        0x8031_3FB0,
        0x8031_3FC8,
        0x8031_3FE0,
    ];
    let mut index = 0usize;
    while index < NAMES.len() {
        w231b_build_state(
            0x8035_7FF8 + index * 0x40,
            NAMES[index],
            0x8031_3ED0 + index * 0x24,
            0x8031_3698,
            W229B_TAG_SHARED,
            0x800A_2530,
        );
        index += 1;
    }
}

/// Builds the twenty-eight `dEnTorideKokoopa_c` state records at `0x80358138`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800AED40`.
/// - Extent: `0x800AED40..0x800B03D8` (5,784 bytes); `stwu r1,-0x420(r1)`
///   with `r25..r31` saved through `0x802DCD6C`.
/// - Table: slot 60 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAD0`).
/// - observed: twenty-eight repetitions of the template with destination base
///   `0x80358138`, payload base `0x803146F0` (stride `0x24`) and getters
///   `0x800A8710..0x800A8840`, `0x800B04C0`, `0x800B04B0`, `0x800B04A0`,
///   `0x800A8850..0x800A8890`; vtable `0x80314F30`, release `0x800B0440`.
/// - The getters return `0x80377A04` except the three middle ones: the
///   `DieFire`, `DieShell` and `DemoWait` records carry `0x80357734`,
///   `0x803577B4` and `0x803576B4` respectively.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Registers the `dEnTorideKokoopa_c` states from `Jump_St` through
/// `DemoEscape_St`.
///
/// # Safety
/// `0x80358138..0x80358838` and the payload table must be readable/writable.
///
/// # Confidence
/// high - all twenty-eight blocks are the same template; the three different
/// tag words come from the literal `lis`/`addi` pairs of their getters.
pub unsafe fn fn_800aed40() {
    const NAMES: [usize; 28] = [
        0x8031_4AE0,
        0x8031_4B04,
        0x8031_4B28,
        0x8031_4B50,
        0x8031_4B74,
        0x8031_4B98,
        0x8031_4BC0,
        0x8031_4BE8,
        0x8031_4C14,
        0x8031_4C38,
        0x8031_4C60,
        0x8031_4C84,
        0x8031_4CA8,
        0x8031_4CCC,
        0x8031_4CF4,
        0x8031_4D1C,
        0x8031_4D48,
        0x8031_4D70,
        0x8031_4D98,
        0x8031_4DC0,
        0x8031_4DE8,
        0x8031_4E0C,
        0x8031_4E34,
        0x8031_4E5C,
        0x8031_4E84,
        0x8031_4EB0,
        0x8031_4ED8,
        0x8031_4F04,
    ];
    const TAGS: [u32; 28] = [
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        0x8035_7734,
        0x8035_77B4,
        0x8035_76B4,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
        W229B_TAG_SHARED,
    ];
    let mut index = 0usize;
    while index < NAMES.len() {
        w231b_build_state(
            0x8035_8138 + index * 0x40,
            NAMES[index],
            0x8031_46F0 + index * 0x24,
            0x8031_4F30,
            TAGS[index],
            0x800B_0440,
        );
        index += 1;
    }
}

/// Constructs the font-record owner at `0x80358848` and registers it.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800B1490`; the callees
///   are `FUN_800b11d0` and `FUN_802dc780`.
/// - Extent: `0x800B1490..0x800B14D8` (72 bytes); `stwu r1,-0x10(r1)` with
///   `r31` saved and a final `blr`.
/// - Table: slot 61 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAD4`).
/// - observed: `addi r3,r31,-30648` = `0x80358848`, `bl 0x800B11D0`; then
///   `bl 0x802DC780(0x80358848, 0x800B1250, 0x80358838)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Initialises the two `0x802DC9A0` containers and clears the four handles of
/// [`crate::game::stage::fn_800b11d0`], then links the object with
/// `0x800B1250` as its release callback and `0x80358838` as the node.
///
/// # Safety
/// `0x80358848` must cover `0xD0` writable bytes and `0x80358838` must be
/// writable through `+8`.
///
/// # Confidence
/// high - one constructor call and one registry call, both with literal
/// operands.
pub unsafe fn fn_800b1490() {
    crate::game::stage::fn_800b11d0(0x8035_8848);
    platform::call3(FN_802DC780, 0x8035_8848, 0x800B_1250, 0x8035_8838);
}

/// Initialises the `Wm_en_quicksand` descriptor at `0x803155D8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800B5840`.
/// - Extent: `0x800B5840..0x800B58B8` (120 bytes); `stwu r1,-0x20(r1)` with
///   the link register saved and a final `blr`.
/// - Table: slot 63 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDADC`).
/// - observed: `stw` of `r2-0x6114` at `0x803155E4`, three `stfs` of
///   `r2-0x6AE0`/`-0x6ADC`/`-0x6AD8` at `0x803155F0..0x803155F8`,
///   `bl 0x802DC780(0, 0x800B58C0, 0x80358A48)`, then `stw -1` at
///   `r13-0x573C` and a copy of `r2-0x60FC` to `r13-0x5740`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills the descriptor fields with `23` and the floats `2160.0`, `-30.0` and
/// `-478.0`, registers the `0x800B58C0` release callback, and stores `-1`/`38`
/// in the SDA1 pair `0x80429F44`/`0x80429F40`.
///
/// # Safety
/// The descriptor and the two SDA1 words must be writable.
///
/// # Confidence
/// high - every stored constant is a literal `lwz`/`lfs` from `r2` with the
/// resolved values above.
pub unsafe fn fn_800b5840() {
    platform::write_u32(0x8031_55E4, platform::read_u32(SDA2_BASE - 0x6114));
    platform::write_f32(0x8031_55F0, platform::read_f32(SDA2_BASE - 0x6AE0));
    platform::write_f32(0x8031_55F4, platform::read_f32(SDA2_BASE - 0x6ADC));
    platform::write_f32(0x8031_55F8, platform::read_f32(SDA2_BASE - 0x6AD8));
    platform::call3(FN_802DC780, 0, 0x800B_58C0, 0x8035_8A48);
    platform::write_u32(SDA1_BASE - 0x573C, u32::MAX);
    platform::write_u32(SDA1_BASE - 0x5740, platform::read_u32(SDA2_BASE - 0x60FC));
}

/// Stores the `0x2AAB`/`0xD555` halfword pair at `0x80429F60`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800B80C0`.
/// - Extent: `0x800B80C0..0x800B80D8` (24 bytes); four instructions plus
///   `blr`, no stack frame.
/// - Table: slot 64 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAE0`).
/// - observed: `addi r3,r13,-22304`, `li r4,10923`, `li r0,-10923`, then
///   `sth r4,0(r3)` and `sth r0,2(r3)`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the signed halfwords `10923` and `-10923` at `SDA1_BASE - 0x5720`
/// and `+0x2` (`0x2AAB`, `0xD555`).
///
/// # Safety
/// Those four bytes must be writable.
///
/// # Confidence
/// high - the body is a literal halfword pair.
pub unsafe fn fn_800b80c0() {
    platform::write_u16(SDA1_BASE - 0x5720, 0x2AAB);
    platform::write_u16(SDA1_BASE - 0x571E, 0xD555);
}

/// Fills the 48-float table at `0x80358AA0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800B8340`.
/// - Extent: `0x800B8340..0x800B8440` (256 bytes), a straight run of `stfs`
///   ending in `blr`; no stack frame.
/// - Table: slot 65 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAE4`).
/// - observed: thirteen `lfs` from `r2-0x6A54..r2-0x6A24`, i.e. the values
///   `1.0, 0.5, 1.4, 0.8, 1.3, 1.7, 1.8, 1.6, 1.5, 2.0, 2.5, 2.8, 3.0`,
///   then 48 `stfs` at `0x80358AA0 + 4n`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Overwrites the 48 words `0x80358AA0..0x80358B5C` with the value sequence
/// below.
///
/// # Safety
/// The 192-byte table must be writable BSS.
///
/// # Confidence
/// high - the constant loads and the 48 stores are fully linear.
pub unsafe fn fn_800b8340() {
    let one = platform::read_f32(SDA2_BASE - 0x6A54);
    let half = platform::read_f32(SDA2_BASE - 0x6A50);
    let one_four = platform::read_f32(SDA2_BASE - 0x6A4C);
    let zero_eight = platform::read_f32(SDA2_BASE - 0x6A48);
    let one_three = platform::read_f32(SDA2_BASE - 0x6A44);
    let one_seven = platform::read_f32(SDA2_BASE - 0x6A40);
    let one_eight = platform::read_f32(SDA2_BASE - 0x6A3C);
    let one_six = platform::read_f32(SDA2_BASE - 0x6A38);
    let one_five = platform::read_f32(SDA2_BASE - 0x6A34);
    let two = platform::read_f32(SDA2_BASE - 0x6A30);
    let two_five = platform::read_f32(SDA2_BASE - 0x6A2C);
    let two_eight = platform::read_f32(SDA2_BASE - 0x6A28);
    let three = platform::read_f32(SDA2_BASE - 0x6A24);
    let values = [
        one, one, one, one, one, one, one, half, one, one, one, one, one, one_four, one_four,
        zero_eight, one, one, one, one, one, one_eight, one_eight, zero_eight, one_seven,
        one_eight, one_six, one_five, two, two_five, two_five, one_three, one_eight, one_eight,
        one_eight, one_five, two, two_eight, two_eight, one_three, one_eight, one_eight,
        one_seven, one_eight, two, three, three, one_three,
    ];
    let mut index = 0usize;
    while index < values.len() {
        platform::write_f32(0x8035_8AA0 + index * 4, values[index]);
        index += 1;
    }
}

/// Fills the 26-float table at `0x80358B60`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800B8FA0`.
/// - Extent: `0x800B8FA0..0x800B9048` (168 bytes), a straight run of `stfs`
///   ending in `blr`; no stack frame.
/// - Table: slot 66 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAE8`).
/// - observed: six `lfs` from `r2-0x6A10..r2-0x69FC`, i.e. `20.0, 64.0,
///   24.0, 36.0, 96.0, 0.0`, then 26 `stfs` at `0x80358B60 + 4n`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Overwrites the 26 words `0x80358B60..0x80358BC4` with the value sequence
/// below.
///
/// # Safety
/// The 104-byte table must be writable BSS.
///
/// # Confidence
/// high - the constant loads and the 26 stores are fully linear.
pub unsafe fn fn_800b8fa0() {
    let twenty = platform::read_f32(SDA2_BASE - 0x6A10);
    let sixty_four = platform::read_f32(SDA2_BASE - 0x6A04);
    let twenty_four = platform::read_f32(SDA2_BASE - 0x6A0C);
    let thirty_six = platform::read_f32(SDA2_BASE - 0x6A08);
    let ninety_six = platform::read_f32(SDA2_BASE - 0x6A00);
    let zero = platform::read_f32(SDA2_BASE - 0x69FC);
    let values = [
        twenty, twenty, twenty, twenty_four, thirty_six, twenty_four, thirty_six, twenty_four,
        twenty_four, sixty_four, sixty_four, sixty_four, sixty_four, ninety_six, sixty_four,
        ninety_six, sixty_four, sixty_four, zero, zero, zero, zero, zero, zero, zero, zero,
    ];
    let mut index = 0usize;
    while index < values.len() {
        platform::write_f32(0x8035_8B60 + index * 4, values[index]);
        index += 1;
    }
}

/// Stores three copies of the float at `r2-0x68D0` at `0x80358DF0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800BFE50`.
/// - Extent: `0x800BFE50..0x800BFE6C` (28 bytes); `lis`, `lfs`, `addi` and
///   three `stfs`, then `blr`.
/// - Table: slot 69 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDAF4`).
/// - observed: `lfs f0,r2-0x68D0` (the value `1.0`) and `stfs f0` at
///   `0x80358DF0`, `+0x4` and `+0x8`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `1.0f` to `0x80358DF0..0x80358DF8`.
///
/// # Safety
/// Those twelve bytes must be writable BSS.
///
/// # Confidence
/// high - three identical stores of one literal.
pub unsafe fn fn_800bfe50() {
    let value = platform::read_f32(SDA2_BASE - 0x68D0);
    platform::write_f32(0x8035_8DF0, value);
    platform::write_f32(0x8035_8DF4, value);
    platform::write_f32(0x8035_8DF8, value);
}

// ---------------------------------------------------------------------------
// Wave 231 lane C: `.ctors` slots 71-84 (`0x802EDAFC..0x802EDB30`).
//
// Four of the fourteen slots already have translated bodies elsewhere and are
// not repeated here: slot 71 `0x800C9A10` is
// [`crate::engine::frame::fn_800c9a10`]; slots 79-81 `0x800D7C80`,
// `0x800DD2B0` and `0x800DE510` are the three descriptor-table builders in
// [`crate::engine::display`]. The ten bodies below are the remaining slots,
// all reached only through the `.ctors` table.
//
// Extents are prologue-to-`blr` inclusive, measured from the raw words in
// `work/dol/main.elf`; where the body ends in an unconditional `b` the extent
// stops at that branch. None of the ten addresses has a `functions.tsv` row.
// ---------------------------------------------------------------------------

/// `0x80359C50`: twelve-float table filled by [`fn_800cd150`].
const TABLE_80359C50_C231: usize = 0x8035_9C50;
/// `r2 - 0x6704` (`0.0f`), first fill value of [`fn_800cd150`].
const F32_SDA2_6704_C231: usize = SDA2_BASE - 0x6704;
/// `r2 - 0x6700` (`1.0f`), second fill value of [`fn_800cd150`].
const F32_SDA2_6700_C231: usize = SDA2_BASE - 0x6700;
/// `r13 - 0x56DC` (`0x80429FA4`): one-shot guard of [`fn_800cd6c0`].
const GUARD_56DC_C231: usize = SDA1_BASE - 0x56DC;
/// `0x803178B4`: word written by [`fn_800cd6c0`].
const WORD_803178B4_C231: usize = 0x8031_78B4;
/// `0x803178F0`: timing record initialised by [`fn_800cd9e0`].
const RECORD_803178F0_C231: usize = 0x8031_78F0;
/// `r2 - 0x6114`: word (`23`) copied to the record by [`fn_800cd9e0`].
const WORD_SDA2_6114_C231: usize = SDA2_BASE - 0x6114;
/// `r2 - 0x66E0`: first timing float (`2160.0f`).
const F32_SDA2_66E0_C231: usize = SDA2_BASE - 0x66E0;
/// `r2 - 0x66DC`: second timing float (`-30.0f`).
const F32_SDA2_66DC_C231: usize = SDA2_BASE - 0x66DC;
/// `r2 - 0x66D8`: third timing float (`-478.0f`).
const F32_SDA2_66D8_C231: usize = SDA2_BASE - 0x66D8;
/// `0x800CDA50`: tag/destructor registered by [`fn_800cd9e0`].
const FN_800CDA50_C231: usize = 0x800C_DA50;
/// `0x80359C80`: registration node of [`fn_800cd9e0`].
const NODE_80359C80_C231: usize = 0x8035_9C80;
/// `r13 - 0x56D8` (`0x80429FA8`): SDA word written by [`fn_800cd9e0`].
const SDA_56D8_C231: usize = SDA1_BASE - 0x56D8;
/// `r2 - 0x60FC`: word (`26`) copied to SDA1 by [`fn_800cd9e0`].
const WORD_SDA2_60FC_C231: usize = SDA2_BASE - 0x60FC;
/// `0x800CDA70`: constructor of the object built by [`fn_800cde70`].
const FN_800CDA70_C231: usize = 0x800C_DA70;
/// `0x80359C9C`: object constructed by [`fn_800cde70`].
const OBJECT_80359C9C_C231: usize = 0x8035_9C9C;
/// `0x800CDAB0`: destructor registered by [`fn_800cde70`].
const FN_800CDAB0_C231: usize = 0x800C_DAB0;
/// `0x80359C90`: registration node of [`fn_800cde70`].
const NODE_80359C90_C231: usize = 0x8035_9C90;
/// `r13 - 0x56B0` (`0x80429FD0`): two-word object cleared by
/// [`fn_800d05f0`].
const SDA_56B0_C231: usize = SDA1_BASE - 0x56B0;
/// `0x80370D10`: four-element array constructed by [`fn_800d08b0`].
const ARRAY_80370D10_C231: usize = 0x8037_0D10;
/// `0x800D0910`: element constructor used by [`fn_800d08b0`].
const FN_800D0910_C231: usize = 0x800D_0910;
/// `0x800D0940`: element destructor used by [`fn_800d08b0`].
const FN_800D0940_C231: usize = 0x800D_0940;
/// `0x800D0980`: tag/destructor registered by [`fn_800d08b0`].
const FN_800D0980_C231: usize = 0x800D_0980;
/// `0x80370D00`: registration node of [`fn_800d08b0`].
const NODE_80370D00_C231: usize = 0x8037_0D00;
/// `0x8005DB00`: constructor of the object built by [`fn_800d7050`].
const FN_8005DB00_C231: usize = 0x8005_DB00;
/// `0x80370DA0`: object constructed by [`fn_800d7050`].
const OBJECT_80370DA0_C231: usize = 0x8037_0DA0;
/// `0x8005DBE0`: destructor registered by [`fn_800d7050`].
const FN_8005DBE0_C231: usize = 0x8005_DBE0;
/// `0x80370D90`: registration node of [`fn_800d7050`].
const NODE_80370D90_C231: usize = 0x8037_0D90;
/// `0x803504E0`: halfword table read by [`fn_800e2a50`].
const TABLE_803504E0_C231: usize = 0x8035_04E0;
/// `r13 - 0x5628` (`0x8042A058`): first float written by
/// [`fn_800e2a50`].
const SDA_5628_C231: usize = SDA1_BASE - 0x5628;
/// `r13 - 0x6F38` (`0x80428748`): pair whose `+4` word is written by
/// [`fn_800e37e0`].
const SDA_6F38_C231: usize = SDA1_BASE - 0x6F38;
/// `r13 - 0x6A78` (`0x80428C08`): source word of [`fn_800e37e0`].
const SDA_6A78_C231: usize = SDA1_BASE - 0x6A78;
/// `0x80371AA0`: four-word record initialised by [`fn_800e5330`].
const RECORD_80371AA0_C231: usize = 0x8037_1AA0;
/// `0x8015F600`: tail callee of [`fn_800e5330`].
const FN_8015F600_C231: usize = 0x8015_F600;
/// `r13 - 0x55E0` (`0x8042A0A0`): pair written by the tail callee.
const SDA_55E0_C231: usize = SDA1_BASE - 0x55E0;
/// `0x8031B080`: word stored by the tail callee.
const WORD_8031B080_C231: usize = 0x8031_B080;

/// `0x800CD150`: fills the twelve-float table at `0x80359C50`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800CD150`.
/// - Extent: `0x800CD150..0x800CD194` (68 bytes); no stack frame, final
///   `bclr 20,0` at `0x800CD190`.
/// - Table: slot 72 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB00`).
/// - observed: `lfs f1,-0x6704(r2)` (`0.0f`) and `lfs f0,-0x6700(r2)`
///   (`1.0f`), then twelve `stfs` at `0x80359C50 + 0x00..0x2C`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0]`
/// to `0x80359C50..0x80359C80`.
///
/// # Safety
/// The twelve words must be writable; they are the module's own `.bss`.
///
/// # Confidence
/// high - every offset and both float constants are read literally from the
/// body and from `work/dol/main.elf`.
pub unsafe fn fn_800cd150() {
    let zero = platform::read_f32(F32_SDA2_6704_C231);
    let one = platform::read_f32(F32_SDA2_6700_C231);
    let values = [
        zero, zero, zero, zero, one, zero, one, one, zero, one, zero, zero,
    ];
    let mut index = 0usize;
    while index < values.len() {
        platform::write_f32(TABLE_80359C50_C231 + index * 4, values[index]);
        index += 1;
    }
}

/// One-shot initialiser of the word at `0x803178B4`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800CD6C0`.
/// - Extent: `0x800CD6C0..0x800CD708` (72 bytes); `stwu r1,-0x10(r1)` and a
///   final `bclr 20,0` at `0x800CD704`.
/// - Table: slot 73 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB04`).
/// - observed: `lbz r0,-0x56DC(r13)`, `extsb. r0,r0`, `bne 0x800CD700`
///   returns when the guard byte is already set; otherwise four `stb` build
///   `0x00004040` on the stack and one `stw` stores it at `0x803178B4`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// On the first call only, stores `0x00004040` at `0x803178B4` and sets the
/// guard byte `0x80429FA4` to 1.
///
/// # Safety
/// The destination word and the guard byte must be writable.
///
/// # Confidence
/// high - the guard, the four-byte assembly of the constant and the single
/// store are complete.
pub unsafe fn fn_800cd6c0() {
    if platform::read_u8(GUARD_56DC_C231) != 0 {
        return;
    }
    platform::write_u32(WORD_803178B4_C231, 0x0000_4040);
    platform::write_u8(GUARD_56DC_C231, 1);
}

/// Initialises the timing record at `0x803178F0` and its SDA mirror.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800CD9E0`.
/// - Extent: `0x800CD9E0..0x800CDA50` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800CDA4C`.
/// - Table: slot 74 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB08`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x66E0`, `r2-0x66DC` and `r2-0x66D8`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x800CDA50, 0x80359C80)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x803178FC`, the three floats at `0x80317908`,
/// `0x8031790C` and `0x80317910`, pushes the node `0x80359C80` onto the
/// shutdown list with `0x800CDA50` as its tag/destructor, and copies the word
/// `26` from `r2-0x60FC` to `0x80429FA8`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal; the constants
/// were read from `work/dol/main.elf`.
pub unsafe fn fn_800cd9e0() {
    platform::write_u32(
        RECORD_803178F0_C231 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_C231),
    );
    platform::write_f32(
        RECORD_803178F0_C231 + 0x18,
        platform::read_f32(F32_SDA2_66E0_C231),
    );
    platform::write_f32(
        RECORD_803178F0_C231 + 0x1C,
        platform::read_f32(F32_SDA2_66DC_C231),
    );
    platform::write_f32(
        RECORD_803178F0_C231 + 0x20,
        platform::read_f32(F32_SDA2_66D8_C231),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800CDA50_C231 as u32,
        NODE_80359C80_C231 as u32,
    );
    platform::write_u32(SDA_56D8_C231, platform::read_u32(WORD_SDA2_60FC_C231));
}

/// Constructs and registers the object at `0x80359C9C`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800CDE70`.
/// - Extent: `0x800CDE70..0x800CDEB8` (72 bytes); `stwu r1,-0x10(r1)` with
///   `r31` saved and a final `bclr 20,0` at `0x800CDEB4`.
/// - Table: slot 75 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB0C`).
/// - observed: `bl 0x800CDA70(object)`, then
///   `0x802DC780(object, 0x800CDAB0, object - 0x0C)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x800CDA70` runs `0x8008F140` and clears the object's `+0x14`/`+0x18`
/// words; the object is then linked at `0x80359C90`.
///
/// # Safety
/// The object and its registration node must be writable.
///
/// # Confidence
/// high - both calls and the `-0x0C` node displacement are literal.
pub unsafe fn fn_800cde70() {
    let _ = platform::call1(FN_800CDA70_C231, OBJECT_80359C9C_C231 as u32);
    platform::call3(
        FN_802DC780,
        OBJECT_80359C9C_C231 as u32,
        FN_800CDAB0_C231 as u32,
        NODE_80359C90_C231 as u32,
    );
}

/// Clears the two-word object at `0x80429FD0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800D05F0`.
/// - Extent: `0x800D05F0..0x800D0604` (20 bytes); `addi r3,r13,-0x56B0`,
///   `li r0,0`, two `stw` and a final `bclr 20,0`.
/// - Table: slot 76 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB10`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Zeroes `0x80429FD0` and `0x80429FD4`.
///
/// # Safety
/// Both words must be writable; they are the module's own SDA1 area.
///
/// # Confidence
/// high - two zero stores, no control flow.
pub unsafe fn fn_800d05f0() {
    platform::write_u32(SDA_56B0_C231, 0);
    platform::write_u32(SDA_56B0_C231 + 4, 0);
}

/// Runs the four-element array constructor at `0x80370D10` and links it.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800D08B0`.
/// - Extent: `0x800D08B0..0x800D0908` (88 bytes); `stwu r1,-0x10(r1)` and a
///   final `bclr 20,0` at `0x800D0904`.
/// - Table: slot 77 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB14`).
/// - observed:
///   `0x802DC9A0(0x80370D10, 0x800D0910, 0x800D0940, 0x20, 4)`, then
///   `0x802DC780(0, 0x800D0980, 0x80370D00)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Constructs four `0x20`-byte elements with `0x800D0910` and destructor
/// `0x800D0940`, then registers the node `0x80370D00`.
///
/// # Safety
/// The array, the element constructors and the node must be valid.
///
/// # Confidence
/// high - the five constructor arguments and the registration triple are
/// literal.
pub unsafe fn fn_800d08b0() {
    platform::call5(
        FN_802DC9A0,
        ARRAY_80370D10_C231 as u32,
        FN_800D0910_C231 as u32,
        FN_800D0940_C231 as u32,
        0x20,
        4,
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800D0980_C231 as u32,
        NODE_80370D00_C231 as u32,
    );
}

/// Constructs and registers the object at `0x80370DA0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800D7050`.
/// - Extent: `0x800D7050..0x800D7098` (72 bytes); `stwu r1,-0x10(r1)` with
///   `r31` saved and a final `bclr 20,0` at `0x800D7094`.
/// - Table: slot 78 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB18`).
/// - observed: `bl 0x8005DB00(object)`, then
///   `0x802DC780(object, 0x8005DBE0, object - 0x10)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Constructs the large display object and links it at `0x80370D90`.
///
/// # Safety
/// The object and its registration node must be writable.
///
/// # Confidence
/// high - both calls and the `-0x10` node displacement are literal.
pub unsafe fn fn_800d7050() {
    let _ = platform::call1(FN_8005DB00_C231, OBJECT_80370DA0_C231 as u32);
    platform::call3(
        FN_802DC780,
        OBJECT_80370DA0_C231 as u32,
        FN_8005DBE0_C231 as u32,
        NODE_80370D90_C231 as u32,
    );
}

/// Converts four halfwords of `0x803504E0` into the SDA1 float record.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800E2A50`.
/// - Extent: `0x800E2A50..0x800E2ACC` (124 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800E2AC8`.
/// - Table: slot 82 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB28`).
/// - observed: four `lhz` at `0x803504E0 + 0x00/0x02/0x0C/0x0E` are widened
///   by the `0x43300000`/`2^52` double trick (`lfd` at `r2-0x63E8` is
///   `2^52`) and stored as floats at `0x8042A058` in the order
///   `+0x0C, +0x0E, +0x00, +0x02`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `832.0f`, `456.0f`, `608.0f` and `456.0f` to
/// `0x8042A058..0x8042A068`.
///
/// # Safety
/// The four destination words must be writable; the halfword table is
/// read-only data.
///
/// # Confidence
/// high - the conversion is the standard unsigned-integer widening and every
/// halfword was read from `work/dol/main.elf`.
pub unsafe fn fn_800e2a50() {
    let source = TABLE_803504E0_C231;
    platform::write_f32(SDA_5628_C231, platform::read_u16(source + 0x0C) as f32);
    platform::write_f32(SDA_5628_C231 + 4, platform::read_u16(source + 0x0E) as f32);
    platform::write_f32(SDA_5628_C231 + 8, platform::read_u16(source) as f32);
    platform::write_f32(
        SDA_5628_C231 + 0x0C,
        platform::read_u16(source + 0x02) as f32,
    );
}

/// Copies one SDA1 pointer into the pair at `0x80428748`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800E37E0`.
/// - Extent: `0x800E37E0..0x800E37F0` (16 bytes); `lwz r0,-0x6A78(r13)`,
///   `addi r3,r13,-0x6F38`, `stw r0,4(r3)`, `bclr 20,0`.
/// - Table: slot 83 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB2C`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies the word at `0x80428C08` to `0x8042874C`.
///
/// # Safety
/// The destination and source words must be readable/writable; both are the
/// module's own SDA1 area.
///
/// # Confidence
/// high - one load and one store, no control flow.
pub unsafe fn fn_800e37e0() {
    platform::write_u32(SDA_6F38_C231 + 4, platform::read_u32(SDA_6A78_C231));
}

/// Initialises the four-word record at `0x80371AA0` and its SDA1 pair.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800E5330`.
/// - Extent: `0x800E5330..0x800E5370` (64 bytes); the body ends in the
///   unconditional tail branch `b 0x8015F600` at `0x800E536C`. The `bclr` at
///   `0x800E5370` is not reached by this body.
/// - Table: slot 84 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB30`).
/// - observed: four literal stores build the record
///   (`0x8031B0CC`, `0x00087400`, `0x00080000`, `0`), then the tail call
///   `0x8015F600(0x8042A0A0, 0x8031B080, 3)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets `0x80371AA0 = 0x8031B0CC`, `0x80371AD4 = 0x00087400`,
/// `0x80371AD8 = 0x00080000` and `0x80371ADC = 0`. The tail callee writes
/// `0x8031B080` at `0x8042A0A0`, halfword `3` at `+4` and halfword `0` at
/// `+6`.
///
/// # Safety
/// The record and the SDA1 pair must be writable.
///
/// # Confidence
/// high - all four stores and all three tail-call operands are literal.
pub unsafe fn fn_800e5330() {
    platform::write_u32(RECORD_80371AA0_C231, 0x8031_B0CC);
    platform::write_u32(RECORD_80371AA0_C231 + 0x34, 0x0008_7400);
    platform::write_u32(RECORD_80371AA0_C231 + 0x38, 0x0008_0000);
    platform::write_u32(RECORD_80371AA0_C231 + 0x3C, 0);
    let _ = platform::call3(
        FN_8015F600_C231,
        SDA_55E0_C231 as u32,
        WORD_8031B080_C231 as u32,
        3,
    );
}

// ---------------------------------------------------------------------------
// Wave 232 lane E: `.ctors` slots 85-98 (`0x802EDB34..0x802EDB68`).
//
// Four of the fourteen slots already have translated bodies elsewhere and are
// not repeated here: slot 86 `0x800EA060` is
// [`crate::engine::dsys::fn_800ea060`], slot 87 `0x800F18B0` is
// [`crate::engine::dsys::fn_800f18b0`], slot 89 `0x800F2680` is
// [`crate::engine::dsys::fn_800f2680`] and slot 91 `0x800F33D0` is
// [`crate::engine::dsys::fn_800f33d0`]. The ten bodies below are the
// remaining slots, all reached only through the `.ctors` table.
//
// Extents are prologue-to-`blr` inclusive, measured from the raw words in
// `work/dol/main.elf`; none of the ten addresses has a `functions.tsv` row.
// Slots 85 and 98 are one-shot SDA/record initialisers. Slots 88, 90 and 92-97
// are eight instances of one state-descriptor installer that mirrors the
// lane-C `fn_800cd9e0` shape: fill a 0x24-byte descriptor record (`+0x0C`
// word, three floats at `+0x18..+0x20`), push a registration node with
// `0x802DC780(node = r5, callback = r4, id = r3)` and publish one SDA word.
// The three `stfs` into the caller's frame before the call are dead stores and
// are not reproduced, following the lane-C precedent.
// ---------------------------------------------------------------------------

/// `r13 - 0x55A8` (`0x8042A0D8`): two-word SDA1 pair cleared by
/// [`fn_800e77a0`].
const SDA_55A8_E232: usize = SDA1_BASE - 0x55A8;
/// `0x8031C7F8`: descriptor record patched by [`fn_800fffc0`].
const RECORD_8031C7F8_E232: usize = 0x8031_C7F8;
/// `r2 - 0x5FE8` (`0x8042D098`): halfword (`1`) copied into the record by
/// [`fn_800fffc0`].
const HALF_SDA2_5FE8_E232: usize = SDA2_BASE - 0x5FE8;
/// `r2 - 0x6114`: word (`23`) stored at descriptor-record `+0x0C` by the
/// installers that read it instead of materialising the immediate.
const WORD_SDA2_6114_E232: usize = SDA2_BASE - 0x6114;
/// `r2 - 0x60FC`: word (`38`) published to SDA1 by every installer.
const WORD_SDA2_60FC_E232: usize = SDA2_BASE - 0x60FC;

/// `0x8031BBC8`: 0x24-byte descriptor record installed by [`fn_800f2300`].
const RECORD_8031BBC8_E232: usize = 0x8031_BBC8;
/// `r2 - 0x6170` (`2160.0f`), first timing value of [`fn_800f2300`].
const F32_SDA2_6170_E232: usize = SDA2_BASE - 0x6170;
/// `r2 - 0x616C` (`-30.0f`), second timing value of [`fn_800f2300`].
const F32_SDA2_616C_E232: usize = SDA2_BASE - 0x616C;
/// `r2 - 0x6168` (`-478.0f`), third timing value of [`fn_800f2300`].
const F32_SDA2_6168_E232: usize = SDA2_BASE - 0x6168;
/// `0x800F2370`: shutdown callback installed by [`fn_800f2300`].
const FN_800F2370_E232: usize = 0x800F_2370;
/// `0x80373C90`: registration node of [`fn_800f2300`].
const NODE_80373C90_E232: usize = 0x8037_3C90;
/// `r13 - 0x5568` (`0x8042A118`): SDA word written by [`fn_800f2300`].
const SDA_5568_E232: usize = SDA1_BASE - 0x5568;

/// `0x8031BC38`: 0x24-byte descriptor record installed by [`fn_800f2a60`].
const RECORD_8031BC38_E232: usize = 0x8031_BC38;
/// `r2 - 0x6148` (`2160.0f`), first timing value of [`fn_800f2a60`].
const F32_SDA2_6148_E232: usize = SDA2_BASE - 0x6148;
/// `r2 - 0x6144` (`-30.0f`), second timing value of [`fn_800f2a60`].
const F32_SDA2_6144_E232: usize = SDA2_BASE - 0x6144;
/// `r2 - 0x6140` (`-478.0f`), third timing value of [`fn_800f2a60`].
const F32_SDA2_6140_E232: usize = SDA2_BASE - 0x6140;
/// `0x800F2AD0`: shutdown callback installed by [`fn_800f2a60`].
const FN_800F2AD0_E232: usize = 0x800F_2AD0;
/// `0x80373CB0`: registration node of [`fn_800f2a60`].
const NODE_80373CB0_E232: usize = 0x8037_3CB0;
/// `r13 - 0x5558` (`0x8042A128`): SDA word written by [`fn_800f2a60`].
const SDA_5558_E232: usize = SDA1_BASE - 0x5558;

/// `0x8031BCE8`: 0x24-byte descriptor record installed by [`fn_800f5f40`].
const RECORD_8031BCE8_E232: usize = 0x8031_BCE8;
/// `r2 - 0x60F4` (`2160.0f`), first timing value of [`fn_800f5f40`].
const F32_SDA2_60F4_E232: usize = SDA2_BASE - 0x60F4;
/// `r2 - 0x60F0` (`-30.0f`), second timing value of [`fn_800f5f40`].
const F32_SDA2_60F0_E232: usize = SDA2_BASE - 0x60F0;
/// `r2 - 0x60EC` (`-478.0f`), third timing value of [`fn_800f5f40`].
const F32_SDA2_60EC_E232: usize = SDA2_BASE - 0x60EC;
/// `0x800F5FB0`: shutdown callback installed by [`fn_800f5f40`].
const FN_800F5FB0_E232: usize = 0x800F_5FB0;
/// `0x80373CD0`: registration node of [`fn_800f5f40`].
const NODE_80373CD0_E232: usize = 0x8037_3CD0;
/// `r13 - 0x5548` (`0x8042A138`): SDA word written by [`fn_800f5f40`].
const SDA_5548_E232: usize = SDA1_BASE - 0x5548;

/// `0x8031BE30`: 0x24-byte descriptor record installed by [`fn_800f6eb0`].
const RECORD_8031BE30_E232: usize = 0x8031_BE30;
/// `r2 - 0x60BC` (`2160.0f`), first timing value of [`fn_800f6eb0`].
const F32_SDA2_60BC_E232: usize = SDA2_BASE - 0x60BC;
/// `r2 - 0x60B8` (`-30.0f`), second timing value of [`fn_800f6eb0`].
const F32_SDA2_60B8_E232: usize = SDA2_BASE - 0x60B8;
/// `r2 - 0x60B4` (`-478.0f`), third timing value of [`fn_800f6eb0`].
const F32_SDA2_60B4_E232: usize = SDA2_BASE - 0x60B4;
/// `0x800F6F20`: shutdown callback installed by [`fn_800f6eb0`].
const FN_800F6F20_E232: usize = 0x800F_6F20;
/// `0x80373CE0`: registration node of [`fn_800f6eb0`].
const NODE_80373CE0_E232: usize = 0x8037_3CE0;
/// `r13 - 0x5530` (`0x8042A150`): SDA word written by [`fn_800f6eb0`].
const SDA_5530_E232: usize = SDA1_BASE - 0x5530;

/// `0x8031BED8`: 0x24-byte descriptor record installed by [`fn_800f91e0`].
const RECORD_8031BED8_E232: usize = 0x8031_BED8;
/// `r2 - 0x6098` (`2160.0f`), first timing value of [`fn_800f91e0`].
const F32_SDA2_6098_E232: usize = SDA2_BASE - 0x6098;
/// `r2 - 0x6094` (`-30.0f`), second timing value of [`fn_800f91e0`].
const F32_SDA2_6094_E232: usize = SDA2_BASE - 0x6094;
/// `r2 - 0x6090` (`-478.0f`), third timing value of [`fn_800f91e0`].
const F32_SDA2_6090_E232: usize = SDA2_BASE - 0x6090;
/// `0x800F9250`: shutdown callback installed by [`fn_800f91e0`].
const FN_800F9250_E232: usize = 0x800F_9250;
/// `0x80373CF0`: registration node of [`fn_800f91e0`].
const NODE_80373CF0_E232: usize = 0x8037_3CF0;
/// `r13 - 0x5528` (`0x8042A158`): SDA word written by [`fn_800f91e0`].
const SDA_5528_E232: usize = SDA1_BASE - 0x5528;

/// `0x8031C09C`: 0x24-byte descriptor record installed by [`fn_800fd500`].
const RECORD_8031C09C_E232: usize = 0x8031_C09C;
/// `r2 - 0x6054` (`2160.0f`), first timing value of [`fn_800fd500`].
const F32_SDA2_6054_E232: usize = SDA2_BASE - 0x6054;
/// `r2 - 0x6050` (`-30.0f`), second timing value of [`fn_800fd500`].
const F32_SDA2_6050_E232: usize = SDA2_BASE - 0x6050;
/// `r2 - 0x604C` (`-478.0f`), third timing value of [`fn_800fd500`].
const F32_SDA2_604C_E232: usize = SDA2_BASE - 0x604C;
/// `0x800FD570`: shutdown callback installed by [`fn_800fd500`].
const FN_800FD570_E232: usize = 0x800F_D570;
/// `0x80373D00`: registration node of [`fn_800fd500`].
const NODE_80373D00_E232: usize = 0x8037_3D00;
/// `r13 - 0x5520` (`0x8042A160`): SDA word written by [`fn_800fd500`].
const SDA_5520_E232: usize = SDA1_BASE - 0x5520;

/// `0x8031C1B8`: 0x24-byte descriptor record installed by [`fn_800fd9c0`].
const RECORD_8031C1B8_E232: usize = 0x8031_C1B8;
/// `r2 - 0x6048` (`2160.0f`), first timing value of [`fn_800fd9c0`].
const F32_SDA2_6048_E232: usize = SDA2_BASE - 0x6048;
/// `r2 - 0x6044` (`-30.0f`), second timing value of [`fn_800fd9c0`].
const F32_SDA2_6044_E232: usize = SDA2_BASE - 0x6044;
/// `r2 - 0x6040` (`-478.0f`), third timing value of [`fn_800fd9c0`].
const F32_SDA2_6040_E232: usize = SDA2_BASE - 0x6040;
/// `0x800FDA30`: shutdown callback installed by [`fn_800fd9c0`].
const FN_800FDA30_E232: usize = 0x800F_DA30;
/// `0x80373D10`: registration node of [`fn_800fd9c0`].
const NODE_80373D10_E232: usize = 0x8037_3D10;
/// `r13 - 0x5518` (`0x8042A168`): SDA word written by [`fn_800fd9c0`].
const SDA_5518_E232: usize = SDA1_BASE - 0x5518;

/// `0x8031C1F0`: 0x24-byte descriptor record installed by [`fn_800fdfa0`].
const RECORD_8031C1F0_E232: usize = 0x8031_C1F0;
/// `r2 - 0x6038` (`2160.0f`), first timing value of [`fn_800fdfa0`].
const F32_SDA2_6038_E232: usize = SDA2_BASE - 0x6038;
/// `r2 - 0x6034` (`-30.0f`), second timing value of [`fn_800fdfa0`].
const F32_SDA2_6034_E232: usize = SDA2_BASE - 0x6034;
/// `r2 - 0x6030` (`-478.0f`), third timing value of [`fn_800fdfa0`].
const F32_SDA2_6030_E232: usize = SDA2_BASE - 0x6030;
/// `0x800FE010`: shutdown callback installed by [`fn_800fdfa0`].
const FN_800FE010_E232: usize = 0x800F_E010;
/// `0x80373D20`: registration node of [`fn_800fdfa0`].
const NODE_80373D20_E232: usize = 0x8037_3D20;
/// `r13 - 0x5510` (`0x8042A170`): SDA word written by [`fn_800fdfa0`].
const SDA_5510_E232: usize = SDA1_BASE - 0x5510;

/// Clears the two-word SDA1 pair at `0x8042A0D8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800E77A0`.
/// - Extent: `0x800E77A0..0x800E77B4` (20 bytes); leaf, `bclr 20,0` at
///   `0x800E77B0`.
/// - Table: slot 85 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB34`).
/// - observed: `addi r3,r13,-0x55A8`, `li r0,0`, `stw r0,0(r13)` and
///   `stw r0,4(r3)`.
///
/// # Arguments
/// None. The address is a link-time constant.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `0` at `0x8042A0D8` and `0x8042A0DC`.
///
/// # Safety
/// The pair must be writable; it is the module's own `.bss`.
///
/// # Confidence
/// high - both stores and the base register are literal.
pub unsafe fn fn_800e77a0() {
    platform::write_u32(SDA_55A8_E232, 0);
    platform::write_u32(SDA_55A8_E232 + 4, 0);
}

/// Installs the state descriptor at `0x8031BBC8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800F2300`.
/// - Extent: `0x800F2300..0x800F2370` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800F236C`.
/// - Table: slot 88 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB40`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x6170`, `r2-0x616C` and `r2-0x6168`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x800F2370, 0x80373C90)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031BBD4`, the three floats at `0x8031BBE0`,
/// `0x8031BBE4` and `0x8031BBE8`, pushes the node `0x80373C90` onto the
/// shutdown list with `0x800F2370` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A118`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal; the constants
/// were read from `work/dol/main.elf`.
pub unsafe fn fn_800f2300() {
    platform::write_u32(
        RECORD_8031BBC8_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031BBC8_E232 + 0x18,
        platform::read_f32(F32_SDA2_6170_E232),
    );
    platform::write_f32(
        RECORD_8031BBC8_E232 + 0x1C,
        platform::read_f32(F32_SDA2_616C_E232),
    );
    platform::write_f32(
        RECORD_8031BBC8_E232 + 0x20,
        platform::read_f32(F32_SDA2_6168_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800F2370_E232 as u32,
        NODE_80373C90_E232 as u32,
    );
    platform::write_u32(SDA_5568_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Installs the state descriptor at `0x8031BC38`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800F2A60`.
/// - Extent: `0x800F2A60..0x800F2AD0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800F2ACC`.
/// - Table: slot 90 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB48`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x6148`, `r2-0x6144` and `r2-0x6140`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x800F2AD0, 0x80373CB0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031BC44`, the three floats at `0x8031BC50`,
/// `0x8031BC54` and `0x8031BC58`, pushes the node `0x80373CB0` onto the
/// shutdown list with `0x800F2AD0` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A128`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_800f2a60() {
    platform::write_u32(
        RECORD_8031BC38_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031BC38_E232 + 0x18,
        platform::read_f32(F32_SDA2_6148_E232),
    );
    platform::write_f32(
        RECORD_8031BC38_E232 + 0x1C,
        platform::read_f32(F32_SDA2_6144_E232),
    );
    platform::write_f32(
        RECORD_8031BC38_E232 + 0x20,
        platform::read_f32(F32_SDA2_6140_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800F2AD0_E232 as u32,
        NODE_80373CB0_E232 as u32,
    );
    platform::write_u32(SDA_5558_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Installs the state descriptor at `0x8031BCE8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800F5F40`.
/// - Extent: `0x800F5F40..0x800F5FB0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800F5FAC`.
/// - Table: slot 92 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB50`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x60F4`, `r2-0x60F0` and `r2-0x60EC`; unlike its seven siblings this
///   body materialises the two published words as immediates (`li r0,23`
///   before the call and `li r0,38` after it) instead of reading them from
///   SDA2; the call is `0x802DC780(0, 0x800F5FB0, 0x80373CD0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031BCF4`, the three floats at `0x8031BD00`,
/// `0x8031BD04` and `0x8031BD08`, pushes the node `0x80373CD0` onto the
/// shutdown list with `0x800F5FB0` as its callback, and stores the word `38`
/// at `0x8042A138`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores, immediates and the one call are all literal.
pub unsafe fn fn_800f5f40() {
    platform::write_u32(RECORD_8031BCE8_E232 + 0x0C, 0x17);
    platform::write_f32(
        RECORD_8031BCE8_E232 + 0x18,
        platform::read_f32(F32_SDA2_60F4_E232),
    );
    platform::write_f32(
        RECORD_8031BCE8_E232 + 0x1C,
        platform::read_f32(F32_SDA2_60F0_E232),
    );
    platform::write_f32(
        RECORD_8031BCE8_E232 + 0x20,
        platform::read_f32(F32_SDA2_60EC_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800F5FB0_E232 as u32,
        NODE_80373CD0_E232 as u32,
    );
    platform::write_u32(SDA_5548_E232, 0x26);
}

/// Installs the state descriptor at `0x8031BE30`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800F6EB0`.
/// - Extent: `0x800F6EB0..0x800F6F20` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800F6F1C`.
/// - Table: slot 93 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB54`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x60BC`, `r2-0x60B8` and `r2-0x60B4`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x800F6F20, 0x80373CE0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031BE3C`, the three floats at `0x8031BE48`,
/// `0x8031BE4C` and `0x8031BE50`, pushes the node `0x80373CE0` onto the
/// shutdown list with `0x800F6F20` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A150`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_800f6eb0() {
    platform::write_u32(
        RECORD_8031BE30_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031BE30_E232 + 0x18,
        platform::read_f32(F32_SDA2_60BC_E232),
    );
    platform::write_f32(
        RECORD_8031BE30_E232 + 0x1C,
        platform::read_f32(F32_SDA2_60B8_E232),
    );
    platform::write_f32(
        RECORD_8031BE30_E232 + 0x20,
        platform::read_f32(F32_SDA2_60B4_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800F6F20_E232 as u32,
        NODE_80373CE0_E232 as u32,
    );
    platform::write_u32(SDA_5530_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Installs the state descriptor at `0x8031BED8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800F91E0`.
/// - Extent: `0x800F91E0..0x800F9250` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800F924C`.
/// - Table: slot 94 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB58`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x6098`, `r2-0x6094` and `r2-0x6090`; `lwz r0,-0x6114(r2)` reads
///   `23`; the callback is formed as `lis 0x8010` + `addi -0x6DB0`, i.e.
///   `0x800F9250`; the call is `0x802DC780(0, 0x800F9250, 0x80373CF0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031BEE4`, the three floats at `0x8031BEF0`,
/// `0x8031BEF4` and `0x8031BEF8`, pushes the node `0x80373CF0` onto the
/// shutdown list with `0x800F9250` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A158`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_800f91e0() {
    platform::write_u32(
        RECORD_8031BED8_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031BED8_E232 + 0x18,
        platform::read_f32(F32_SDA2_6098_E232),
    );
    platform::write_f32(
        RECORD_8031BED8_E232 + 0x1C,
        platform::read_f32(F32_SDA2_6094_E232),
    );
    platform::write_f32(
        RECORD_8031BED8_E232 + 0x20,
        platform::read_f32(F32_SDA2_6090_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800F9250_E232 as u32,
        NODE_80373CF0_E232 as u32,
    );
    platform::write_u32(SDA_5528_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Installs the state descriptor at `0x8031C09C`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800FD500`.
/// - Extent: `0x800FD500..0x800FD570` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800FD56C`.
/// - Table: slot 95 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB5C`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x6054`, `r2-0x6050` and `r2-0x604C`; `lwz r0,-0x6114(r2)` reads
///   `23`; the callback is formed as `lis 0x8010` + `addi -0x2A90`, i.e.
///   `0x800FD570`; the call is `0x802DC780(0, 0x800FD570, 0x80373D00)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031C0A8`, the three floats at `0x8031C0B4`,
/// `0x8031C0B8` and `0x8031C0BC`, pushes the node `0x80373D00` onto the
/// shutdown list with `0x800FD570` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A160`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_800fd500() {
    platform::write_u32(
        RECORD_8031C09C_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031C09C_E232 + 0x18,
        platform::read_f32(F32_SDA2_6054_E232),
    );
    platform::write_f32(
        RECORD_8031C09C_E232 + 0x1C,
        platform::read_f32(F32_SDA2_6050_E232),
    );
    platform::write_f32(
        RECORD_8031C09C_E232 + 0x20,
        platform::read_f32(F32_SDA2_604C_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800FD570_E232 as u32,
        NODE_80373D00_E232 as u32,
    );
    platform::write_u32(SDA_5520_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Installs the state descriptor at `0x8031C1B8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800FD9C0`.
/// - Extent: `0x800FD9C0..0x800FDA30` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800FDA2C`.
/// - Table: slot 96 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB60`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x6048`, `r2-0x6044` and `r2-0x6040`; `lwz r0,-0x6114(r2)` reads
///   `23`; the callback is formed as `lis 0x8010` + `addi -0x25D0`, i.e.
///   `0x800FDA30`; the call is `0x802DC780(0, 0x800FDA30, 0x80373D10)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031C1C4`, the three floats at `0x8031C1D0`,
/// `0x8031C1D4` and `0x8031C1D8`, pushes the node `0x80373D10` onto the
/// shutdown list with `0x800FDA30` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A168`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_800fd9c0() {
    platform::write_u32(
        RECORD_8031C1B8_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031C1B8_E232 + 0x18,
        platform::read_f32(F32_SDA2_6048_E232),
    );
    platform::write_f32(
        RECORD_8031C1B8_E232 + 0x1C,
        platform::read_f32(F32_SDA2_6044_E232),
    );
    platform::write_f32(
        RECORD_8031C1B8_E232 + 0x20,
        platform::read_f32(F32_SDA2_6040_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800FDA30_E232 as u32,
        NODE_80373D10_E232 as u32,
    );
    platform::write_u32(SDA_5518_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Installs the state descriptor at `0x8031C1F0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800FDFA0`.
/// - Extent: `0x800FDFA0..0x800FE010` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x800FE00C`.
/// - Table: slot 97 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB64`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x6038`, `r2-0x6034` and `r2-0x6030`; `lwz r0,-0x6114(r2)` reads
///   `23`; the callback is formed as `lis 0x8010` + `addi -0x1FF0`, i.e.
///   `0x800FE010`; the call is `0x802DC780(0, 0x800FE010, 0x80373D20)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031C1FC`, the three floats at `0x8031C208`,
/// `0x8031C20C` and `0x8031C210`, pushes the node `0x80373D20` onto the
/// shutdown list with `0x800FE010` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A170`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_800fdfa0() {
    platform::write_u32(
        RECORD_8031C1F0_E232 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E232),
    );
    platform::write_f32(
        RECORD_8031C1F0_E232 + 0x18,
        platform::read_f32(F32_SDA2_6038_E232),
    );
    platform::write_f32(
        RECORD_8031C1F0_E232 + 0x1C,
        platform::read_f32(F32_SDA2_6034_E232),
    );
    platform::write_f32(
        RECORD_8031C1F0_E232 + 0x20,
        platform::read_f32(F32_SDA2_6030_E232),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_800FE010_E232 as u32,
        NODE_80373D20_E232 as u32,
    );
    platform::write_u32(SDA_5510_E232, platform::read_u32(WORD_SDA2_60FC_E232));
}

/// Publishes the halfword `1` in the descriptor at `0x8031C7F8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x800FFFC0`.
/// - Extent: `0x800FFFC0..0x800FFFD4` (20 bytes); leaf, `bclr 20,0` at
///   `0x800FFFD0`.
/// - Table: slot 98 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB68`).
/// - observed: `lis r3,0x8032`, `lhz r0,-0x5FE8(r2)` (the halfword reads
///   `1`), `addi r3,r3,-0x3808` (`0x8031C7F8`) and `sth r0,2(r3)`.
///
/// # Arguments
/// None. The address is a link-time constant.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores the halfword `1` at `0x8031C7FA`, leaving the `0x0003` word at
/// `0x8031C7F8` otherwise untouched.
///
/// # Safety
/// The record must be writable.
///
/// # Confidence
/// high - the load, the address construction and the store are literal.
pub unsafe fn fn_800fffc0() {
    platform::write_u16(
        RECORD_8031C7F8_E232 + 2,
        platform::read_u16(HALF_SDA2_5FE8_E232),
    );
}

// ---------------------------------------------------------------------------
// Wave 233 lane E: `.ctors` slots 99-112 (`0x802EDB6C..0x802EDBA0`).
//
// Four of the fourteen slots already have translated bodies and are not
// repeated here: slot 99 `0x80100830` is
// [`crate::engine::globals::fn_80100830`], slot 102 `0x801025D0` is
// [`crate::game::worldmap::fn_801025d0`], slot 103 `0x80102C10` is
// [`crate::game::worldmap::wm_projection_globals_init`] (a semantic name for
// the same address) and slot 111 `0x8010B890` is
// [`crate::game::worldmap::fn_8010b890`]. The ten bodies below are the
// remaining slots, all reached only through the `.ctors` table.
//
// Extents are prologue-to-`blr` inclusive, measured by recursive descent from
// the raw words in `work/dol/main.elf`. Only slot 103 has a `functions.tsv`
// row; the ten bodies here have none.
//
// Slots 100, 101 and 104-108 are seven instances of the state-descriptor
// installer already seen in wave 232 lane E: fill a 0x24-byte record (`+0x0C`
// word `23`, three floats at `+0x18..+0x20`), push a registration node with
// `0x802DC780(id = r3, callback = r4, node = r5)` and publish the word `38`
// from `r2-0x60FC` into one SDA1 word. Slot 100 additionally owns a one-shot
// guard and a larger secondary table at `0x8031D39C`; slot 112 is a guarded
// ten-float block with no registration call; slot 109 stores `255` into one
// SDA1 word; slot 110 clears a scene singleton and registers a shutdown node.
//
// The original `stfs` copies of the timing floats into the caller's frame are
// dead stores and are not reproduced, following the lane-C precedent.
// ---------------------------------------------------------------------------

/// `r2 - 0x6114`: word (`23`) stored at descriptor-record `+0x0C` by the
/// installers that read it instead of materialising the immediate.
const WORD_SDA2_6114_E233: usize = SDA2_BASE - 0x6114;
/// `r2 - 0x60FC`: word (`38`) published to SDA1 by every installer.
const WORD_SDA2_60FC_E233: usize = SDA2_BASE - 0x60FC;

/// `0x8031D378`: 0x24-byte descriptor record installed by [`fn_80101430`].
const RECORD_8031D378_E233: usize = 0x8031_D378;
/// `0x8031D39C`: second table owned by [`fn_80101430`] (`record + 0x24`).
const SECONDARY_8031D39C_E233: usize = 0x8031_D39C;
/// `r2 - 0x5F8C` (`2160.0f`), first timing value of [`fn_80101430`].
const F32_SDA2_5F8C_E233: usize = SDA2_BASE - 0x5F8C;
/// `r2 - 0x5F88` (`-30.0f`), second timing value of [`fn_80101430`].
const F32_SDA2_5F88_E233: usize = SDA2_BASE - 0x5F88;
/// `r2 - 0x5F84` (`-478.0f`), third timing value of [`fn_80101430`].
const F32_SDA2_5F84_E233: usize = SDA2_BASE - 0x5F84;
/// `0x80101580`: shutdown callback installed by [`fn_80101430`].
const FN_80101580_E233: usize = 0x8010_1580;
/// `0x80373D40`: registration node of [`fn_80101430`].
const NODE_80373D40_E233: usize = 0x8037_3D40;
/// `r13 - 0x54E8` (`0x8042A198`): SDA word written by [`fn_80101430`].
const SDA_54E8_E233: usize = SDA1_BASE - 0x54E8;
/// `r13 - 0x54DC` (`0x8042A1A4`): `.bss` guard byte of [`fn_80101430`].
const GUARD_54DC_E233: usize = SDA1_BASE - 0x54DC;
/// `r2 - 0x5FB8` (`1.0f`), first pair component of [`fn_80101430`].
const F32_SDA2_5FB8_E233: usize = SDA2_BASE - 0x5FB8;
/// `r2 - 0x5FA4` (`30.0f`), repeated pair component of [`fn_80101430`].
const F32_SDA2_5FA4_E233: usize = SDA2_BASE - 0x5FA4;
/// `r2 - 0x5F9C` (`0.0f`), filler of the secondary table in [`fn_80101430`].
const F32_SDA2_5F9C_E233: usize = SDA2_BASE - 0x5F9C;
/// `r2 - 0x5F80` (`15.0f`), first pair component of [`fn_80101430`].
const F32_SDA2_5F80_E233: usize = SDA2_BASE - 0x5F80;
/// `r2 - 0x5FA8` (`10.0f`), third pair component of [`fn_80101430`].
const F32_SDA2_5FA8_E233: usize = SDA2_BASE - 0x5FA8;
/// `r2 - 0x5F7C` (`600.0f`), third triplet component of [`fn_80101430`].
const F32_SDA2_5F7C_E233: usize = SDA2_BASE - 0x5F7C;
/// `r2 - 0x5F78` (`-600.0f`), first triplet component of [`fn_80101430`].
const F32_SDA2_5F78_E233: usize = SDA2_BASE - 0x5F78;

/// `0x8031D890`: 0x24-byte descriptor record installed by [`fn_80101720`].
const RECORD_8031D890_E233: usize = 0x8031_D890;
/// `r2 - 0x5F70` (`2160.0f`), first timing value of [`fn_80101720`].
const F32_SDA2_5F70_E233: usize = SDA2_BASE - 0x5F70;
/// `r2 - 0x5F6C` (`-30.0f`), second timing value of [`fn_80101720`].
const F32_SDA2_5F6C_E233: usize = SDA2_BASE - 0x5F6C;
/// `r2 - 0x5F68` (`-478.0f`), third timing value of [`fn_80101720`].
const F32_SDA2_5F68_E233: usize = SDA2_BASE - 0x5F68;
/// `0x80101790`: shutdown callback installed by [`fn_80101720`].
const FN_80101790_E233: usize = 0x8010_1790;
/// `0x80373D50`: registration node of [`fn_80101720`].
const NODE_80373D50_E233: usize = 0x8037_3D50;
/// `r13 - 0x54D8` (`0x8042A1A8`): SDA word written by [`fn_80101720`].
const SDA_54D8_E233: usize = SDA1_BASE - 0x54D8;

/// `0x80321488`: 0x24-byte descriptor record installed by [`fn_80102e80`].
const RECORD_80321488_E233: usize = 0x8032_1488;
/// `r2 - 0x5F00` (`2160.0f`), first timing value of [`fn_80102e80`].
const F32_SDA2_5F00_E233: usize = SDA2_BASE - 0x5F00;
/// `r2 - 0x5EFC` (`-30.0f`), second timing value of [`fn_80102e80`].
const F32_SDA2_5EFC_E233: usize = SDA2_BASE - 0x5EFC;
/// `r2 - 0x5EF8` (`-478.0f`), third timing value of [`fn_80102e80`].
const F32_SDA2_5EF8_E233: usize = SDA2_BASE - 0x5EF8;
/// `0x80102EF0`: shutdown callback installed by [`fn_80102e80`].
const FN_80102EF0_E233: usize = 0x8010_2EF0;
/// `0x803748D0`: registration node of [`fn_80102e80`].
const NODE_803748D0_E233: usize = 0x8037_48D0;
/// `r13 - 0x5420` (`0x8042A260`): SDA word written by [`fn_80102e80`].
const SDA_5420_E233: usize = SDA1_BASE - 0x5420;

/// `0x803214C0`: 0x24-byte descriptor record installed by [`fn_80103730`].
const RECORD_803214C0_E233: usize = 0x8032_14C0;
/// `r2 - 0x5EF0` (`2160.0f`), first timing value of [`fn_80103730`].
const F32_SDA2_5EF0_E233: usize = SDA2_BASE - 0x5EF0;
/// `r2 - 0x5EEC` (`-30.0f`), second timing value of [`fn_80103730`].
const F32_SDA2_5EEC_E233: usize = SDA2_BASE - 0x5EEC;
/// `r2 - 0x5EE8` (`-478.0f`), third timing value of [`fn_80103730`].
const F32_SDA2_5EE8_E233: usize = SDA2_BASE - 0x5EE8;
/// `0x801037A0`: shutdown callback installed by [`fn_80103730`].
const FN_801037A0_E233: usize = 0x8010_37A0;
/// `0x803748E0`: registration node of [`fn_80103730`].
const NODE_803748E0_E233: usize = 0x8037_48E0;
/// `r13 - 0x5418` (`0x8042A268`): SDA word written by [`fn_80103730`].
const SDA_5418_E233: usize = SDA1_BASE - 0x5418;

/// `0x80321890`: 0x24-byte descriptor record installed by [`fn_80103a30`].
const RECORD_80321890_E233: usize = 0x8032_1890;
/// `r2 - 0x5EC8` (`2160.0f`), first timing value of [`fn_80103a30`].
const F32_SDA2_5EC8_E233: usize = SDA2_BASE - 0x5EC8;
/// `r2 - 0x5EC4` (`-30.0f`), second timing value of [`fn_80103a30`].
const F32_SDA2_5EC4_E233: usize = SDA2_BASE - 0x5EC4;
/// `r2 - 0x5EC0` (`-478.0f`), third timing value of [`fn_80103a30`].
const F32_SDA2_5EC0_E233: usize = SDA2_BASE - 0x5EC0;
/// `0x80103AA0`: shutdown callback installed by [`fn_80103a30`].
const FN_80103AA0_E233: usize = 0x8010_3AA0;
/// `0x803748F0`: registration node of [`fn_80103a30`].
const NODE_803748F0_E233: usize = 0x8037_48F0;
/// `r13 - 0x5410` (`0x8042A270`): SDA word written by [`fn_80103a30`].
const SDA_5410_E233: usize = SDA1_BASE - 0x5410;

/// `0x803218C8`: 0x24-byte descriptor record installed by [`fn_80105f40`].
const RECORD_803218C8_E233: usize = 0x8032_18C8;
/// `r2 - 0x5E88` (`2160.0f`), first timing value of [`fn_80105f40`].
const F32_SDA2_5E88_E233: usize = SDA2_BASE - 0x5E88;
/// `r2 - 0x5E84` (`-30.0f`), second timing value of [`fn_80105f40`].
const F32_SDA2_5E84_E233: usize = SDA2_BASE - 0x5E84;
/// `r2 - 0x5E80` (`-478.0f`), third timing value of [`fn_80105f40`].
const F32_SDA2_5E80_E233: usize = SDA2_BASE - 0x5E80;
/// `0x80105FB0`: shutdown callback installed by [`fn_80105f40`].
const FN_80105FB0_E233: usize = 0x8010_5FB0;
/// `0x80374900`: registration node of [`fn_80105f40`].
const NODE_80374900_E233: usize = 0x8037_4900;
/// `r13 - 0x5408` (`0x8042A278`): SDA word written by [`fn_80105f40`].
const SDA_5408_E233: usize = SDA1_BASE - 0x5408;

/// `0x80321BB0`: 0x24-byte descriptor record installed by [`fn_80106d60`].
const RECORD_80321BB0_E233: usize = 0x8032_1BB0;
/// `r2 - 0x5E48` (`2160.0f`), first timing value of [`fn_80106d60`].
const F32_SDA2_5E48_E233: usize = SDA2_BASE - 0x5E48;
/// `r2 - 0x5E44` (`-30.0f`), second timing value of [`fn_80106d60`].
const F32_SDA2_5E44_E233: usize = SDA2_BASE - 0x5E44;
/// `r2 - 0x5E40` (`-478.0f`), third timing value of [`fn_80106d60`].
const F32_SDA2_5E40_E233: usize = SDA2_BASE - 0x5E40;
/// `0x80106DD0`: shutdown callback installed by [`fn_80106d60`].
const FN_80106DD0_E233: usize = 0x8010_6DD0;
/// `0x80374910`: registration node of [`fn_80106d60`].
const NODE_80374910_E233: usize = 0x8037_4910;
/// `r13 - 0x5400` (`0x8042A280`): SDA word written by [`fn_80106d60`].
const SDA_5400_E233: usize = SDA1_BASE - 0x5400;

/// `r13 - 0x53D8` (`0x8042A2A8`): SDA word set to `255` by [`fn_80108f00`].
const SDA_53D8_E233: usize = SDA1_BASE - 0x53D8;

/// `0x803749A0`: scene singleton whose first word is cleared by
/// [`fn_8010aef0`]; `crate::game::worldmap` records the same address as the
/// scene/renderer object.
const SCENE_803749A0_E233: usize = 0x8037_49A0;
/// `0x80374990`: registration node of [`fn_8010aef0`].
const NODE_80374990_E233: usize = 0x8037_4990;
/// `0x80014330`: clears one word of an object; documented in
/// [`crate::ui::hud::fn_80014330`].
const FN_80014330_E233: usize = 0x8001_4330;
/// `0x80014340`: destructor-shaped callback; documented in
/// [`crate::game::actor::fn_80014340`].
const FN_80014340_E233: usize = 0x8001_4340;

/// `r13 - 0x53AC` (`0x8042A2D4`): `.bss` guard byte of [`fn_8010d0f0`].
const GUARD_53AC_E233: usize = SDA1_BASE - 0x53AC;
/// `0x80374E80`: ten-float block filled by [`fn_8010d0f0`].
const RECORD_80374E80_E233: usize = 0x8037_4E80;
/// `r2 - 0x5D70` (`0.8f`), field `+0x0C` of [`fn_8010d0f0`].
const F32_SDA2_5D70_E233: usize = SDA2_BASE - 0x5D70;
/// `r2 - 0x5D6C` (`0.8f`), field `+0x10` of [`fn_8010d0f0`].
const F32_SDA2_5D6C_E233: usize = SDA2_BASE - 0x5D6C;
/// `r2 - 0x5D68` (`1.0f`), field `+0x14` of [`fn_8010d0f0`].
const F32_SDA2_5D68_E233: usize = SDA2_BASE - 0x5D68;
/// `r2 - 0x5D64` (`1.0f`), field `+0x18` of [`fn_8010d0f0`].
const F32_SDA2_5D64_E233: usize = SDA2_BASE - 0x5D64;
/// `r2 - 0x5D58` (`0.8f`), field `+0x1C` of [`fn_8010d0f0`].
const F32_SDA2_5D58_E233: usize = SDA2_BASE - 0x5D58;
/// `r2 - 0x5D54` (`0.8f`), field `+0x20` of [`fn_8010d0f0`].
const F32_SDA2_5D54_E233: usize = SDA2_BASE - 0x5D54;
/// `r2 - 0x5D60` (`1.1f`), field `+0x24` of [`fn_8010d0f0`].
const F32_SDA2_5D60_E233: usize = SDA2_BASE - 0x5D60;
/// `r2 - 0x5D5C` (`1.1f`), field `+0x28` of [`fn_8010d0f0`].
const F32_SDA2_5D5C_E233: usize = SDA2_BASE - 0x5D5C;
/// `r2 - 0x5D50` (`640.0f`), field `+0x2C` of [`fn_8010d0f0`].
const F32_SDA2_5D50_E233: usize = SDA2_BASE - 0x5D50;
/// `r2 - 0x5D4C` (`352.0f`), field `+0x30` of [`fn_8010d0f0`].
const F32_SDA2_5D4C_E233: usize = SDA2_BASE - 0x5D4C;

/// Installs the descriptor at `0x8031D378` and its secondary float table.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80101430`.
/// - Extent: `0x80101430..0x80101580` (336 bytes); `stwu r1,-0x70(r1)` at the
///   entry and `bclr 20,0` at `0x8010157C`. The one-shot guard branch at
///   `0x80101498` (`bc 4,2`) jumps to the epilogue at `0x80101570`.
/// - Table: slot 100 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB70`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5F8C`, `r2-0x5F88` and `r2-0x5F84`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x80101580, 0x80373D40)`; the guard is
///   `lbz r0,-0x54DC(r13)` plus `extsb.`; the secondary table reads `1.0f`,
///   `30.0f`, `0.0f`, `15.0f`, `10.0f`, `600.0f` and `-600.0f` from
///   `r2-0x5FB8..-0x5F78`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8032D384` and the three timing floats at `0x8032D390`,
/// `0x8032D394` and `0x8032D398`; registers the node `0x80373D40` with
/// callback `0x80101580`; copies the word `38` from `r2-0x60FC` to
/// `0x8042A198`. Then, only while the byte at `0x8042A1A4` is zero, stores
/// the halfword `25` at `0x8032D3BA`, fourteen floats at `0x8032D414..0x8032D448`
/// and six floats at `0x8032D5BC..0x8032D5D0`, and finally sets that byte to
/// `1`.
///
/// # Safety
/// The record, the registration node and the SDA words must be writable.
///
/// # Confidence
/// high - every read, store, branch target and constant is literal.
pub unsafe fn fn_80101430() {
    platform::write_u32(
        RECORD_8031D378_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_8031D378_E233 + 0x18,
        platform::read_f32(F32_SDA2_5F8C_E233),
    );
    platform::write_f32(
        RECORD_8031D378_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5F88_E233),
    );
    platform::write_f32(
        RECORD_8031D378_E233 + 0x20,
        platform::read_f32(F32_SDA2_5F84_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_80101580_E233 as u32,
        NODE_80373D40_E233 as u32,
    );
    platform::write_u32(SDA_54E8_E233, platform::read_u32(WORD_SDA2_60FC_E233));
    if platform::read_u8(GUARD_54DC_E233) != 0 {
        return;
    }
    platform::write_u16(SECONDARY_8031D39C_E233 + 0x1E, 25);
    let one = platform::read_f32(F32_SDA2_5FB8_E233);
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x78, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x7C,
        platform::read_f32(F32_SDA2_5F80_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x80, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x84,
        platform::read_f32(F32_SDA2_5FA4_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x88, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x8C,
        platform::read_f32(F32_SDA2_5FA4_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x90, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x94,
        platform::read_f32(F32_SDA2_5FA8_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x98, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x9C,
        platform::read_f32(F32_SDA2_5FA4_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0xA0, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0xA4,
        platform::read_f32(F32_SDA2_5FA4_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0xA8, one);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0xAC,
        platform::read_f32(F32_SDA2_5FA4_E233),
    );
    let zero = platform::read_f32(F32_SDA2_5F9C_E233);
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x220, zero);
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x224, zero);
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x228,
        platform::read_f32(F32_SDA2_5F7C_E233),
    );
    platform::write_f32(
        SECONDARY_8031D39C_E233 + 0x22C,
        platform::read_f32(F32_SDA2_5F78_E233),
    );
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x230, zero);
    platform::write_f32(SECONDARY_8031D39C_E233 + 0x234, zero);
    platform::write_u8(GUARD_54DC_E233, 1);
}

/// Installs the state descriptor at `0x8031D890`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80101720`.
/// - Extent: `0x80101720..0x80101790` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x8010178C`.
/// - Table: slot 101 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB74`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5F70`, `r2-0x5F6C` and `r2-0x5F68`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x80101790, 0x80373D50)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8031D89C`, the three floats at `0x8031D8A8`,
/// `0x8031D8AC` and `0x8031D8B0`, pushes the node `0x80373D50` onto the
/// shutdown list with `0x80101790` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A1A8`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_80101720() {
    platform::write_u32(
        RECORD_8031D890_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_8031D890_E233 + 0x18,
        platform::read_f32(F32_SDA2_5F70_E233),
    );
    platform::write_f32(
        RECORD_8031D890_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5F6C_E233),
    );
    platform::write_f32(
        RECORD_8031D890_E233 + 0x20,
        platform::read_f32(F32_SDA2_5F68_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_80101790_E233 as u32,
        NODE_80373D50_E233 as u32,
    );
    platform::write_u32(SDA_54D8_E233, platform::read_u32(WORD_SDA2_60FC_E233));
}

/// Installs the state descriptor at `0x80321488`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80102E80`.
/// - Extent: `0x80102E80..0x80102EF0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x80102EEC`.
/// - Table: slot 104 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB80`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5F00`, `r2-0x5EFC` and `r2-0x5EF8`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x80102EF0, 0x803748D0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x80321494`, the three floats at `0x803214A0`,
/// `0x803214A4` and `0x803214A8`, pushes the node `0x803748D0` onto the
/// shutdown list with `0x80102EF0` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A260`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_80102e80() {
    platform::write_u32(
        RECORD_80321488_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_80321488_E233 + 0x18,
        platform::read_f32(F32_SDA2_5F00_E233),
    );
    platform::write_f32(
        RECORD_80321488_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5EFC_E233),
    );
    platform::write_f32(
        RECORD_80321488_E233 + 0x20,
        platform::read_f32(F32_SDA2_5EF8_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_80102EF0_E233 as u32,
        NODE_803748D0_E233 as u32,
    );
    platform::write_u32(SDA_5420_E233, platform::read_u32(WORD_SDA2_60FC_E233));
}

/// Installs the state descriptor at `0x803214C0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80103730`.
/// - Extent: `0x80103730..0x801037A0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x8010379C`.
/// - Table: slot 105 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB84`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5EF0`, `r2-0x5EEC` and `r2-0x5EE8`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x801037A0, 0x803748E0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x803214CC`, the three floats at `0x803214D8`,
/// `0x803214DC` and `0x803214E0`, pushes the node `0x803748E0` onto the
/// shutdown list with `0x801037A0` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A268`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_80103730() {
    platform::write_u32(
        RECORD_803214C0_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_803214C0_E233 + 0x18,
        platform::read_f32(F32_SDA2_5EF0_E233),
    );
    platform::write_f32(
        RECORD_803214C0_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5EEC_E233),
    );
    platform::write_f32(
        RECORD_803214C0_E233 + 0x20,
        platform::read_f32(F32_SDA2_5EE8_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_801037A0_E233 as u32,
        NODE_803748E0_E233 as u32,
    );
    platform::write_u32(SDA_5418_E233, platform::read_u32(WORD_SDA2_60FC_E233));
}

/// Installs the state descriptor at `0x80321890`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80103A30`.
/// - Extent: `0x80103A30..0x80103AA0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x80103A9C`.
/// - Table: slot 106 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB88`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5EC8`, `r2-0x5EC4` and `r2-0x5EC0`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x80103AA0, 0x803748F0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x8032189C`, the three floats at `0x803218A8`,
/// `0x803218AC` and `0x803218B0`, pushes the node `0x803748F0` onto the
/// shutdown list with `0x80103AA0` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A270`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_80103a30() {
    platform::write_u32(
        RECORD_80321890_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_80321890_E233 + 0x18,
        platform::read_f32(F32_SDA2_5EC8_E233),
    );
    platform::write_f32(
        RECORD_80321890_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5EC4_E233),
    );
    platform::write_f32(
        RECORD_80321890_E233 + 0x20,
        platform::read_f32(F32_SDA2_5EC0_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_80103AA0_E233 as u32,
        NODE_803748F0_E233 as u32,
    );
    platform::write_u32(SDA_5410_E233, platform::read_u32(WORD_SDA2_60FC_E233));
}

/// Installs the state descriptor at `0x803218C8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80105F40`.
/// - Extent: `0x80105F40..0x80105FB0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x80105FAC`.
/// - Table: slot 107 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB8C`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5E88`, `r2-0x5E84` and `r2-0x5E80`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x80105FB0, 0x80374900)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x803218D4`, the three floats at `0x803218E0`,
/// `0x803218E4` and `0x803218E8`, pushes the node `0x80374900` onto the
/// shutdown list with `0x80105FB0` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A278`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_80105f40() {
    platform::write_u32(
        RECORD_803218C8_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_803218C8_E233 + 0x18,
        platform::read_f32(F32_SDA2_5E88_E233),
    );
    platform::write_f32(
        RECORD_803218C8_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5E84_E233),
    );
    platform::write_f32(
        RECORD_803218C8_E233 + 0x20,
        platform::read_f32(F32_SDA2_5E80_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_80105FB0_E233 as u32,
        NODE_80374900_E233 as u32,
    );
    platform::write_u32(SDA_5408_E233, platform::read_u32(WORD_SDA2_60FC_E233));
}

/// Installs the state descriptor at `0x80321BB0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80106D60`.
/// - Extent: `0x80106D60..0x80106DD0` (112 bytes); `stwu r1,-0x20(r1)` and a
///   final `bclr 20,0` at `0x80106DCC`.
/// - Table: slot 108 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB90`).
/// - observed: `lfs` reads `2160.0f`, `-30.0f` and `-478.0f` from
///   `r2-0x5E48`, `r2-0x5E44` and `r2-0x5E40`; `lwz r0,-0x6114(r2)` reads
///   `23`; the call is `0x802DC780(0, 0x80106DD0, 0x80374910)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `23` at `0x80321BBC`, the three floats at `0x80321BC8`,
/// `0x80321BCC` and `0x80321BD0`, pushes the node `0x80374910` onto the
/// shutdown list with `0x80106DD0` as its callback, and copies the word `38`
/// from `r2-0x60FC` to `0x8042A280`.
///
/// # Safety
/// The record, the registration node and the SDA word must be writable.
///
/// # Confidence
/// high - the reads, stores and the one call are all literal.
pub unsafe fn fn_80106d60() {
    platform::write_u32(
        RECORD_80321BB0_E233 + 0x0C,
        platform::read_u32(WORD_SDA2_6114_E233),
    );
    platform::write_f32(
        RECORD_80321BB0_E233 + 0x18,
        platform::read_f32(F32_SDA2_5E48_E233),
    );
    platform::write_f32(
        RECORD_80321BB0_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5E44_E233),
    );
    platform::write_f32(
        RECORD_80321BB0_E233 + 0x20,
        platform::read_f32(F32_SDA2_5E40_E233),
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_80106DD0_E233 as u32,
        NODE_80374910_E233 as u32,
    );
    platform::write_u32(SDA_5400_E233, platform::read_u32(WORD_SDA2_60FC_E233));
}

/// Publishes the word `255` in the SDA block.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80108F00`.
/// - Extent: `0x80108F00..0x80108F0C` (12 bytes); leaf, `bclr 20,0` at
///   `0x80108F08`.
/// - Table: slot 109 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB94`).
/// - observed: `li r0,255` and `stw r0,-0x53D8(r13)`.
///
/// # Arguments
/// None. The address is a link-time constant.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores the word `255` at `0x8042A2A8` (SDA1 `-0x53D8`).
///
/// # Safety
/// The SDA word must be writable; it is the module's own `.bss`.
///
/// # Confidence
/// high - the immediate and the store are literal.
pub unsafe fn fn_80108f00() {
    platform::write_u32(SDA_53D8_E233, 255);
}

/// Clears a scene singleton and registers its shutdown node.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8010AEF0`.
/// - Extent: `0x8010AEF0..0x8010AF38` (72 bytes); `stwu r1,-0x10(r1)` and a
///   final `bclr 20,0` at `0x8010AF34`.
/// - Table: slot 110 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDB98`).
/// - observed: `lis r31,0x8037`, `addi r3,r31,0x49A0`, `bl 0x80014330`, then
///   `0x802DC780(0x803749A0, 0x80014340, 0x80374990)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears the word at `0x803749A0` through `0x80014330`, then pushes the node
/// `0x80374990` onto the shutdown list with `0x80014340` as its callback and
/// `0x803749A0` as the registration id. `0x80014330` is documented in
/// [`crate::ui::hud::fn_80014330`] and `0x80014340` in
/// [`crate::game::actor::fn_80014340`].
///
/// # Safety
/// The singleton word and the registration node must be writable.
///
/// # Confidence
/// high - the addresses and the two calls are literal; the role of
/// `0x803749A0` is inferred from the shared scene-singleton use in
/// `crate::game::worldmap`.
pub unsafe fn fn_8010aef0() {
    platform::call1(FN_80014330_E233, SCENE_803749A0_E233 as u32);
    platform::call3(
        FN_802DC780,
        SCENE_803749A0_E233 as u32,
        FN_80014340_E233 as u32,
        NODE_80374990_E233 as u32,
    );
}

/// Fills a ten-float block once, guarded by an SDA byte.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8010D0F0`.
/// - Extent: `0x8010D0F0..0x8010D160` (112 bytes); leaf, the early
///   `bclr 4,2` at `0x8010D0F8` returns when the guard is non-zero and the
///   final `bclr 20,0` is at `0x8010D15C`.
/// - Table: slot 112 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBA0`).
/// - observed: `lbz r0,-0x53AC(r13)` plus `extsb.` before the guard; the
///   record base is `lis 0x8037` + `addi 0x4E80`; ten `lfs` read
///   `r2-0x5D70`, `-0x5D6C`, `-0x5D68`, `-0x5D64`, `-0x5D58`, `-0x5D54`,
///   `-0x5D60`, `-0x5D5C`, `-0x5D50` and `-0x5D4C`; the guard is set with
///   `stb` of `1`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Only while the byte at `0x8042A2D4` is zero, stores the ten floats
/// `0.8`, `0.8`, `1.0`, `1.0`, `0.8`, `0.8`, `1.1`, `1.1`, `640.0` and
/// `352.0` at `0x80374E8C..0x80374EB0`, then sets that byte to `1`.
///
/// # Safety
/// The record and the SDA byte must be writable.
///
/// # Confidence
/// high - every load, store and the guard test are literal.
pub unsafe fn fn_8010d0f0() {
    if platform::read_u8(GUARD_53AC_E233) != 0 {
        return;
    }
    platform::write_f32(
        RECORD_80374E80_E233 + 0x0C,
        platform::read_f32(F32_SDA2_5D70_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x10,
        platform::read_f32(F32_SDA2_5D6C_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x14,
        platform::read_f32(F32_SDA2_5D68_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x18,
        platform::read_f32(F32_SDA2_5D64_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x1C,
        platform::read_f32(F32_SDA2_5D58_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x20,
        platform::read_f32(F32_SDA2_5D54_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x24,
        platform::read_f32(F32_SDA2_5D60_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x28,
        platform::read_f32(F32_SDA2_5D5C_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x2C,
        platform::read_f32(F32_SDA2_5D50_E233),
    );
    platform::write_f32(
        RECORD_80374E80_E233 + 0x30,
        platform::read_f32(F32_SDA2_5D4C_E233),
    );
    platform::write_u8(GUARD_53AC_E233, 1);
}

// ===========================================================================
// Wave 234 lane E: `.ctors` slots 113-126 (`0x802EDBA4..0x802EDBD8`)
// ===========================================================================
//
// Table base `0x802ED9E0`; slot `n`'s word is at `0x802ED9E0 + 4n`. Four of the
// fourteen slots already have translated bodies in the tree and are referenced
// rather than repeated:
//
// | slot | word | target | existing body |
// |-----:|------|--------|---------------|
// | 113 | `0x802EDBA4` | `0x8010E830` | [`crate::game::worldmap::fn_8010e830`] |
// | 123 | `0x802EDBCC` | `0x80153400` | [`crate::engine::display::fn_80153400`] |
// | 125 | `0x802EDBD4` | `0x80159F10` | [`crate::engine::display::dgame_display_register_states`] |
// | 126 | `0x802EDBD8` | `0x8015B320` | [`crate::engine::display::fn_8015b320`] |
//
// The ten bodies below are the remaining slots. Nine of them are state-bank
// installers of the same family as the wave-229/233 sections: each builds
// `0x40`-byte (or `0x34`-byte) state descriptors, so the shared installer
// [`w234b_install`] is table-driven. The descriptor tables were generated from
// a concrete interpretation of the DOL words (see Evidence) rather than typed
// by hand, so every state, name, data record and callback address is literal.
//
// Extents are prologue-to-`blr` inclusive, measured by recursive descent from
// the raw words. None of the ten addresses has a `functions.tsv` row.
//
// Only two bodies contain control flow: `0x80147BD0` and `0x801573B0` each own
// one one-shot guard byte in SDA1, tested with `lbz`/`extsb.`/`bc 4,2`.
// ---------------------------------------------------------------------------

/// One state descriptor installed by a wave-234 `.ctors` body.
///
/// Field roles are the ones documented for the wave-229 installer:
/// `0x8015F7C0(state, name)` initialises the header, `vtable` is stored at
/// `+0`, the `data` record supplies the nine handler words at `+0x0C..+0x2C`,
/// `tag` (when present) is stored at `+0x30`, and
/// `0x802DC780(state, release, event)` pushes the registry node.
struct W234B_Descriptor {
    /// Descriptor base; writable through `+0x30`.
    state: usize,
    /// `StateID_*` string passed to `0x8015F7C0`.
    name: u32,
    /// Three-triplet handler record copied to `+0x0C..+0x2C`.
    data: u32,
    /// Vtable word installed at `+0`.
    vtable: u32,
    /// Optional tenth word installed at `+0x30`.
    tag: Option<u32>,
    /// Release callback passed to `0x802DC780`.
    release: u32,
    /// Registry node written by `0x802DC780`.
    event: usize,
}

/// Installs one [`W234B_Descriptor`] row.
///
/// observed: every bank in this lane copies its handler record as three
/// 12-byte triplets from `data + 0x18`, `data + 0x0C` and `data + 0x00`, in
/// that order, targeting `state + 0x0C`, `state + 0x18` and `state + 0x24`
/// respectively. The vtable store at `+0` lands after the `0x8015F7C0` header
/// call and before the registration push, so it overwrites the header's first
/// word.
unsafe fn w234b_install(entry: &W234B_Descriptor) {
    platform::call2(FN_8015F7C0, entry.state as u32, entry.name);
    platform::write_u32(entry.state, entry.vtable);
    let mut triplet = 0usize;
    while triplet < 3 {
        let source = entry.data as usize + (2 - triplet) * 0x0C;
        let target = entry.state + 0x0C + triplet * 0x0C;
        let mut word = 0usize;
        while word < 3 {
            platform::write_u32(
                target + word * 4,
                platform::read_u32(source + word * 4),
            );
            word += 1;
        }
        triplet += 1;
    }
    if let Some(value) = entry.tag {
        platform::write_u32(entry.state + 0x30, value);
    }
    platform::call3(
        FN_802DC780,
        entry.state as u32,
        entry.release,
        entry.event as u32,
    );
}

/// `r13 - 0x5390` (`0x8042A2F0`): first SDA word cleared by [`fn_8010f840`].
const SDA_5390_E234: usize = SDA1_BASE - 0x5390;
/// `r2 - 0x5D20`: the float both SDA words receive (`0.0f` in the image).
const F32_SDA2_5D20_E234: usize = SDA2_BASE - 0x5D20;

/// Clears two adjacent SDA floats.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8010F840`.
/// - Extent: `0x8010F840..0x8010F854` (20 bytes, leaf).
/// - Table: slot 114 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBA8`).
/// - observed: `lfs f0,-0x5D20(r2)`; `addi r3,r13,-0x5390`;
///   `stfs f0,-0x5390(r13)`; `stfs f0,4(r3)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the same float to `0x8042A2F0` and `0x8042A2F4`. Both words are zero
/// in the shipped image, but the body is a copy, not a constant store, so the
/// read is kept.
///
/// # Safety
/// Both SDA words must be writable.
///
/// # Confidence
/// high - the whole body is five literal instructions.
pub unsafe fn fn_8010f840() {
    let value = platform::read_f32(F32_SDA2_5D20_E234);
    platform::write_f32(SDA_5390_E234, value);
    platform::write_f32(SDA_5390_E234 + 4, value);
}

/// `0x8011FE20`: destructor-shaped callback registered by [`fn_8010fc90`].
const FN_8011FE20_E234: usize = 0x8011_FE20;

/// Installs the descriptor at `0x803750C4`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8010FC90`.
/// - Extent: `0x8010FC90..0x8010FDB4` (292 bytes); `stwu r1,-0x40(r1)` at the
///   entry and `bclr 20,0` at `0x8010FDB0`.
/// - Table: slot 115 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBAC`).
/// - observed: three 12-byte triplets are read from `0x80323228+0x18`,
///   `+0x0C`, `+0x00`; `0x8015F7C0(0x803750C4, 0x8032324C)`; vtable
///   `0x8032326C`; the tenth word is the return of `0x8010FE80` (a 12-byte
///   `return &0x80358104` in `crate::game::worldmap`); the registration is
///   `0x802DC780(0x803750C4, 0x8011FE20, 0x803750B8)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills the `0x34`-byte descriptor at `0x803750C4` and pushes the node
/// `0x803750B8` onto the shutdown list.
///
/// # Safety
/// The descriptor and node must be writable.
///
/// # Confidence
/// high - the address of the tenth word is spelled out literally; the
/// `worldmap` helper returns `0x80358104` per the DOL words.
pub unsafe fn fn_8010fc90() {
    let rows = [W234B_Descriptor {
        state: 0x8037_50C4,
        name: 0x8032_324C,
        data: 0x8032_3228,
        vtable: 0x8032_326C,
        tag: Some(0x8035_8104),
        release: FN_8011FE20_E234 as u32,
        event: 0x8037_50B8,
    }];
    for entry in rows.iter() {
        w234b_install(entry);
    }
}

/// `0x80114940`: release callback shared by the six descriptors of
/// [`fn_80114440`].
const FN_80114940_E234: usize = 0x8011_4940;
/// `0x80375278`: first of the nine tail floats of [`fn_80114440`].
const TAIL_80375278_E234: usize = 0x8037_5278;
/// `r2 - 0x5C70` (`1.5f`), first tail float.
const F32_SDA2_5C70_E234: usize = SDA2_BASE - 0x5C70;
/// `0x8042D394` (`16.0f`), third tail float.
const F32_8042D394_E234: usize = 0x8042_D394;
/// `0x8042D378` (`4.0f`), repeated tail float.
const F32_8042D378_E234: usize = 0x8042_D378;
/// `0x8042D424` (`18.0f`), sixth tail float.
const F32_8042D424_E234: usize = 0x8042_D424;
/// `0x8042D428` (`10.0f`), eighth tail float.
const F32_8042D428_E234: usize = 0x8042_D428;
/// `0x8042D42C` (`22.0f`), ninth tail float.
const F32_8042D42C_E234: usize = 0x8042_D42C;
/// Source of each of the nine tail floats, in store order.
const W234B_TAIL_80114440_E234: [usize; 9] = [
    F32_SDA2_5C70_E234,
    F32_SDA2_5C70_E234,
    F32_8042D394_E234,
    F32_8042D378_E234,
    F32_8042D378_E234,
    F32_8042D424_E234,
    F32_8042D378_E234,
    F32_8042D428_E234,
    F32_8042D42C_E234,
];

/// Installs six descriptors at `0x80375108` and their tail floats.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80114440`.
/// - Extent: `0x80114440..0x80114938` (1272 bytes); `stwu r1,-0x100(r1)` and
///   the `0x802DCD74` register-save thunk at the entry, `bclr 20,0` at
///   `0x80114934`.
/// - Table: slot 116 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBB0`).
/// - observed: six descriptor blocks at `0x80375108` step `0x40`, vtable
///   `0x803237E4`, `0x802DC780(state, 0x80114940, state-0x10)`.
/// - observed: nine `stfs` follow at `0x80375278` (the `+0x30` field of the
///   last descriptor) reading `1.5`, `1.5`, `16`, `4`, `4`, `18`, `4`, `10`,
///   `22`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds six state descriptors, pushes their registry nodes and writes the
/// nine-float tail block.
///
/// # Safety
/// The descriptors, the nodes and the tail block must be writable.
///
/// # Confidence
/// high - the six blocks and the nine reads are literal.
pub unsafe fn fn_80114440() {
    for entry in W234B_ROWS_80114440_E234.iter() {
        w234b_install(entry);
    }
    let mut index = 0usize;
    while index < 9 {
        platform::write_f32(
            TAIL_80375278_E234 + index * 4,
            platform::read_f32(W234B_TAIL_80114440_E234[index]),
        );
        index += 1;
    }
}

/// Handler rows of `0x80114440`, one per descriptor block.
const W234B_ROWS_80114440_E234: [W234B_Descriptor; 6] = [
    W234B_Descriptor {
        state: 0x8037_5108,
        name: 0x8032_36F0,
        data: 0x8032_3618,
        vtable: 0x8032_37E4,
        tag: None,
        release: 0x8011_4940,
        event: 0x8037_50F8,
    },
    W234B_Descriptor {
        state: 0x8037_5148,
        name: 0x8032_371C,
        data: 0x8032_363C,
        vtable: 0x8032_37E4,
        tag: None,
        release: 0x8011_4940,
        event: 0x8037_5138,
    },
    W234B_Descriptor {
        state: 0x8037_5188,
        name: 0x8032_3748,
        data: 0x8032_3660,
        vtable: 0x8032_37E4,
        tag: None,
        release: 0x8011_4940,
        event: 0x8037_5178,
    },
    W234B_Descriptor {
        state: 0x8037_51C8,
        name: 0x8032_376C,
        data: 0x8032_3684,
        vtable: 0x8032_37E4,
        tag: None,
        release: 0x8011_4940,
        event: 0x8037_51B8,
    },
    W234B_Descriptor {
        state: 0x8037_5208,
        name: 0x8032_3790,
        data: 0x8032_36A8,
        vtable: 0x8032_37E4,
        tag: None,
        release: 0x8011_4940,
        event: 0x8037_51F8,
    },
    W234B_Descriptor {
        state: 0x8037_5248,
        name: 0x8032_37B8,
        data: 0x8032_36CC,
        vtable: 0x8032_37E4,
        tag: None,
        release: 0x8011_4940,
        event: 0x8037_5238,
    },
];

/// `r2 - 0x5C34` (`20.0f`), first column of the wave-234 lane-E float rows.
const F32_SDA2_5C34_E234: usize = SDA2_BASE - 0x5C34;
/// `r2 - 0x5C30` (`24.0f`), second column.
const F32_SDA2_5C30_E234: usize = SDA2_BASE - 0x5C30;
/// `r2 - 0x5C2C` (`36.0f`), third column.
const F32_SDA2_5C2C_E234: usize = SDA2_BASE - 0x5C2C;
/// `r2 - 0x5C28` (`64.0f`), fourth column.
const F32_SDA2_5C28_E234: usize = SDA2_BASE - 0x5C28;
/// `r2 - 0x5C24` (`96.0f`), fifth column.
const F32_SDA2_5C24_E234: usize = SDA2_BASE - 0x5C24;
/// Column order of the two 18-float rows written by [`fn_80115510`].
const W234B_COLUMNS_80115510_E234: [u8; 18] = [
    0, 0, 0, 1, 2, 1, 2, 1, 1, 3, 3, 3, 3, 4, 3, 4, 3, 3,
];
/// First of the two 18-float rows written by [`fn_80115510`].
const ROWS_803752A0_E234: usize = 0x8037_52A0;
/// `0x80115910`: release callback shared by the four descriptors.
const FN_80115910_E234: usize = 0x8011_5910;

/// Fills two 18-float rows and installs four descriptors.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80115510`.
/// - Extent: `0x80115510..0x80115908` (1016 bytes); `stwu r1,-0xC0(r1)` at the
///   entry and `bclr 20,0` at `0x80115904`.
/// - Table: slot 117 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBB4`).
/// - observed: five `lfs` read `r2-0x5C34..-0x5C24`; 36 `stfs` write
///   `0x803752A0..0x8037532C` (two identical rows of 18); four descriptor
///   blocks follow at `0x80375340`, `+0x40`, `+0x80`, `+0xC0` with vtable
///   `0x80323A78` and `0x802DC780(state, 0x80115910, state-0x10)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two float rows, then builds four state descriptors and pushes
/// their registry nodes.
///
/// # Safety
/// The float rows, the descriptors and the nodes must be writable.
///
/// # Confidence
/// high - the row pattern is the literal column order of the 36 `stfs`.
pub unsafe fn fn_80115510() {
    let columns = [
        F32_SDA2_5C34_E234,
        F32_SDA2_5C30_E234,
        F32_SDA2_5C2C_E234,
        F32_SDA2_5C28_E234,
        F32_SDA2_5C24_E234,
    ];
    let mut row = 0usize;
    while row < 2 {
        let mut index = 0usize;
        while index < 18 {
            platform::write_f32(
                ROWS_803752A0_E234 + row * 0x48 + index * 4,
                platform::read_f32(columns[W234B_COLUMNS_80115510_E234[index] as usize]),
            );
            index += 1;
        }
        row += 1;
    }
    for entry in W234B_ROWS_80115510_E234.iter() {
        w234b_install(entry);
    }
}

/// Handler rows of `0x80115510`, one per descriptor block.
const W234B_ROWS_80115510_E234: [W234B_Descriptor; 4] = [
    W234B_Descriptor {
        state: 0x8037_5340,
        name: 0x8032_3A00,
        data: 0x8032_3970,
        vtable: 0x8032_3A78,
        tag: None,
        release: 0x8011_5910,
        event: 0x8037_5330,
    },
    W234B_Descriptor {
        state: 0x8037_5380,
        name: 0x8032_3A20,
        data: 0x8032_3994,
        vtable: 0x8032_3A78,
        tag: None,
        release: 0x8011_5910,
        event: 0x8037_5370,
    },
    W234B_Descriptor {
        state: 0x8037_53C0,
        name: 0x8032_3A40,
        data: 0x8032_39B8,
        vtable: 0x8032_3A78,
        tag: None,
        release: 0x8011_5910,
        event: 0x8037_53B0,
    },
    W234B_Descriptor {
        state: 0x8037_5400,
        name: 0x8032_3A5C,
        data: 0x8032_39DC,
        vtable: 0x8032_3A78,
        tag: None,
        release: 0x8011_5910,
        event: 0x8037_53F0,
    },
];

/// `0x80115A90`: constructor passed to the `0x802DC9A0` array helper.
const FN_80115A90_E234: usize = 0x8011_5A90;
/// `0x8011A410`: constructor argument (19 instructions past this body).
const ARG_8011A410_E234: usize = 0x8011_A410;
/// `0x8011A450`: registration tag of [`fn_8011a380`].
const FN_8011A450_E234: usize = 0x8011_A450;
/// `0x80375440`: first of the two `0x80`-byte records built by [`fn_8011a380`].
const RECORD_80375440_E234: usize = 0x8037_5440;
/// `0x80375430`: registry node of [`fn_8011a380`].
const NODE_80375430_E234: usize = 0x8037_5430;
/// `r2 - 0x5BA8` (`480.0f`), base value of [`fn_8011a380`].
const F32_SDA2_5BA8_E234: usize = SDA2_BASE - 0x5BA8;
/// `r2 - 0x5C00` (`256.0f`), scale value of [`fn_8011a380`].
const F32_SDA2_5C00_E234: usize = SDA2_BASE - 0x5C00;
/// `r13 - 0x6380` (`0x80429300`, `1.0f`): first multiplier of both values.
const F32_SDA1_6380_E234: usize = SDA1_BASE - 0x6380;
/// `r2 - 0x5C10` (`0x8042D470`, `0.5f`): half-scale multiplier.
const F32_SDA2_5C10_E234: usize = SDA2_BASE - 0x5C10;
/// `r13 - 0x5388` (`0x8042A2F8`): base value after the `1.0f` multiply.
const SDA_5388_E234: usize = SDA1_BASE - 0x5388;
/// `r13 - 0x5384` (`0x8042A2FC`): half of the base value.
const SDA_5384_E234: usize = SDA1_BASE - 0x5384;
/// `r13 - 0x5380` (`0x8042A300`): scale value after the `1.0f` multiply.
const SDA_5380_E234: usize = SDA1_BASE - 0x5380;
/// `r13 - 0x537C` (`0x8042A304`): half of the scale value.
const SDA_537C_E234: usize = SDA1_BASE - 0x537C;

/// Derives four SDA scale words, builds two records and registers a node.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8011A380`.
/// - Extent: `0x8011A380..0x8011A408` (136 bytes); `stwu r1,-0x10(r1)` at the
///   entry and `bclr 20,0` at `0x8011A404`.
/// - Table: slot 118 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBB8`).
/// - observed: `f1 = [r2-0x5BA8] * [r13-0x6380]` and
///   `f0 = [r2-0x5C00] * [r13-0x6380]` (`480.0f` and `256.0f`, multiplied by
///   `1.0f`); `f1`/`f0` go to `r13-0x5388`/`-0x5380`. Then
///   `f1 *= [r2-0x5C10]` and `f0 *= [r2-0x5C10]` (`0.5f`), and the halves go
///   to `r13-0x5384`/`-0x537C`. Then
///   `0x802DC9A0(0x80375440, 0x80115A90, 0x8011A410, 0x80, 2)` and
///   `0x802DC780(0, 0x8011A450, 0x80375430)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four SDA floats, constructs two `0x80`-byte records with the shared
/// array helper and pushes the registry node `0x80375430`.
///
/// # Safety
/// The SDA words, the record array and the node must be writable.
///
/// # Confidence
/// high - every multiply, store order and call argument is literal.
pub unsafe fn fn_8011a380() {
    let unit = platform::read_f32(F32_SDA1_6380_E234);
    let half = platform::read_f32(F32_SDA2_5C10_E234);
    let width = platform::read_f32(F32_SDA2_5BA8_E234) * unit;
    let height = platform::read_f32(F32_SDA2_5C00_E234) * unit;
    platform::write_f32(SDA_5388_E234, width);
    platform::write_f32(SDA_5380_E234, height);
    platform::write_f32(SDA_5384_E234, width * half);
    platform::write_f32(SDA_537C_E234, height * half);
    platform::call5(
        FN_802DC9A0,
        RECORD_80375440_E234 as u32,
        FN_80115A90_E234 as u32,
        ARG_8011A410_E234 as u32,
        0x80,
        2,
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_8011A450_E234 as u32,
        NODE_80375430_E234 as u32,
    );
}

/// `0x8012B1B0`: destructor-shaped callback registered by [`fn_8011b020`].
const FN_8012B1B0_E234: usize = 0x8012_B1B0;

/// Installs the descriptor at `0x8037556C`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8011B020`.
/// - Extent: `0x8011B020..0x8011B144` (292 bytes); `stwu r1,-0x40(r1)` at the
///   entry and `bclr 20,0` at `0x8011B140`.
/// - Table: slot 119 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBBC`).
/// - observed: handler record `0x80323E8C`, name `0x80323EB0`, vtable
///   `0x80323ED4`; the tenth word is the return of `0x8011B210`, whose whole
///   body is `lis r3,0x8035 / addi r3,r3,0x3F0C` (`work/ghidra/main-gb/export/8011b210_FUN_8011b210.c`);
///   registration `0x802DC780(0x8037556C, 0x8012B1B0, 0x80375560)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills the descriptor at `0x8037556C` and pushes the node `0x80375560`.
///
/// # Safety
/// The descriptor and node must be writable.
///
/// # Confidence
/// high - the tenth word is the literal constant the accessor returns.
pub unsafe fn fn_8011b020() {
    let rows = [W234B_Descriptor {
        state: 0x8037_556C,
        name: 0x8032_3EB0,
        data: 0x8032_3E8C,
        vtable: 0x8032_3ED4,
        tag: Some(0x8035_3F0C),
        release: FN_8012B1B0_E234 as u32,
        event: 0x8037_5560,
    }];
    for entry in rows.iter() {
        w234b_install(entry);
    }
}

/// `0x80358B60`: 18-word layout block copied to `0x803755A0` by [`fn_801221f0`].
///
/// The block is in `.bss`, so the copy carries values produced by an earlier
/// constructor; it is reproduced as a read, not as zeros.
const SOURCE_80358B60_E234: usize = 0x8035_8B60;
/// `0x803755A0`: destination of the 18-word copy.
const DEST_803755A0_E234: usize = 0x8037_55A0;
/// `0x80123130`: release callback shared by the nineteen descriptors.
const FN_80123130_E234: usize = 0x8012_3130;

/// Copies a layout block and installs nineteen descriptors.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x801221F0`.
/// - Extent: `0x801221F0..0x80123130` (3904 bytes); `stwu r1,-0x320(r1)` at
///   the entry and `bclr 20,0` at `0x8012312C`.
/// - Table: slot 120 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBC0`).
/// - observed: eighteen `lfs`/`stfs` pairs copy `0x80358B60..0x80358BA4` to
///   `0x803755A0..0x803755E4`; nineteen descriptor blocks follow at
///   `0x803755F8` step `0x40` with vtable `0x803246A0` and
///   `0x802DC780(state, 0x80123130, state-0x10)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies the layout block, then builds nineteen state descriptors and pushes
/// their registry nodes.
///
/// # Safety
/// The destination block, the descriptors and the nodes must be writable.
///
/// # Confidence
/// high - the copy is word-for-word and the table rows come from the same
/// literal stores.
pub unsafe fn fn_801221f0() {
    let mut index = 0usize;
    while index < 18 {
        platform::write_f32(
            DEST_803755A0_E234 + index * 4,
            platform::read_f32(SOURCE_80358B60_E234 + index * 4),
        );
        index += 1;
    }
    for entry in W234B_ROWS_801221F0_E234.iter() {
        w234b_install(entry);
    }
}

/// Handler rows of `0x801221F0`, one per descriptor block.
const W234B_ROWS_801221F0_E234: [W234B_Descriptor; 19] = [
    W234B_Descriptor {
        state: 0x8037_55F8,
        name: 0x8032_44B8,
        data: 0x8032_4208,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_55E8,
    },
    W234B_Descriptor {
        state: 0x8037_5638,
        name: 0x8032_44D0,
        data: 0x8032_422C,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5628,
    },
    W234B_Descriptor {
        state: 0x8037_5678,
        name: 0x8032_44E8,
        data: 0x8032_4250,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5668,
    },
    W234B_Descriptor {
        state: 0x8037_56B8,
        name: 0x8032_4500,
        data: 0x8032_4274,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_56A8,
    },
    W234B_Descriptor {
        state: 0x8037_56F8,
        name: 0x8032_4518,
        data: 0x8032_4298,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_56E8,
    },
    W234B_Descriptor {
        state: 0x8037_5738,
        name: 0x8032_4534,
        data: 0x8032_42BC,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5728,
    },
    W234B_Descriptor {
        state: 0x8037_5778,
        name: 0x8032_454C,
        data: 0x8032_42E0,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5768,
    },
    W234B_Descriptor {
        state: 0x8037_57B8,
        name: 0x8032_4568,
        data: 0x8032_4304,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_57A8,
    },
    W234B_Descriptor {
        state: 0x8037_57F8,
        name: 0x8032_4584,
        data: 0x8032_4328,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_57E8,
    },
    W234B_Descriptor {
        state: 0x8037_5838,
        name: 0x8032_45A0,
        data: 0x8032_434C,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5828,
    },
    W234B_Descriptor {
        state: 0x8037_5878,
        name: 0x8032_45BC,
        data: 0x8032_4370,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5868,
    },
    W234B_Descriptor {
        state: 0x8037_58B8,
        name: 0x8032_45D4,
        data: 0x8032_4394,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_58A8,
    },
    W234B_Descriptor {
        state: 0x8037_58F8,
        name: 0x8032_45EC,
        data: 0x8032_43B8,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_58E8,
    },
    W234B_Descriptor {
        state: 0x8037_5938,
        name: 0x8032_4604,
        data: 0x8032_43DC,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5928,
    },
    W234B_Descriptor {
        state: 0x8037_5978,
        name: 0x8032_461C,
        data: 0x8032_4400,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5968,
    },
    W234B_Descriptor {
        state: 0x8037_59B8,
        name: 0x8032_4638,
        data: 0x8032_4424,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_59A8,
    },
    W234B_Descriptor {
        state: 0x8037_59F8,
        name: 0x8032_4650,
        data: 0x8032_4448,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_59E8,
    },
    W234B_Descriptor {
        state: 0x8037_5A38,
        name: 0x8032_4668,
        data: 0x8032_446C,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5A28,
    },
    W234B_Descriptor {
        state: 0x8037_5A78,
        name: 0x8032_4680,
        data: 0x8032_4490,
        vtable: 0x8032_46A0,
        tag: None,
        release: 0x8012_3130,
        event: 0x8037_5A68,
    },
];

/// `0x80124A80`: release callback shared by the five descriptors of
/// [`fn_80124680`].
const FN_80124A80_E234: usize = 0x8012_4A80;

/// Installs five descriptors at `0x80375AD8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80124680`.
/// - Extent: `0x80124680..0x80124A74` (1012 bytes); `stwu r1,-0x...` prologue
///   through the `0x802DCD74` register-save thunk and `bclr 20,0` at
///   `0x80124A70`.
/// - Table: slot 121 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBC4`).
/// - observed: five descriptor blocks at `0x80375AD8` step `0x40`, vtable
///   `0x803249A8`, `0x802DC780(state, 0x80124A80, state-0x10)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds five state descriptors and pushes their registry nodes.
///
/// # Safety
/// The descriptors and nodes must be writable.
///
/// # Confidence
/// high - the five blocks are identical apart from the table words.
pub unsafe fn fn_80124680() {
    for entry in W234B_ROWS_80124680_E234.iter() {
        w234b_install(entry);
    }
}

/// Handler rows of `0x80124680`, one per descriptor block.
const W234B_ROWS_80124680_E234: [W234B_Descriptor; 5] = [
    W234B_Descriptor {
        state: 0x8037_5AD8,
        name: 0x8032_4918,
        data: 0x8032_4864,
        vtable: 0x8032_49A8,
        tag: None,
        release: 0x8012_4A80,
        event: 0x8037_5AC8,
    },
    W234B_Descriptor {
        state: 0x8037_5B18,
        name: 0x8032_4938,
        data: 0x8032_4888,
        vtable: 0x8032_49A8,
        tag: None,
        release: 0x8012_4A80,
        event: 0x8037_5B08,
    },
    W234B_Descriptor {
        state: 0x8037_5B58,
        name: 0x8032_4954,
        data: 0x8032_48AC,
        vtable: 0x8032_49A8,
        tag: None,
        release: 0x8012_4A80,
        event: 0x8037_5B48,
    },
    W234B_Descriptor {
        state: 0x8037_5B98,
        name: 0x8032_4970,
        data: 0x8032_48D0,
        vtable: 0x8032_49A8,
        tag: None,
        release: 0x8012_4A80,
        event: 0x8037_5B88,
    },
    W234B_Descriptor {
        state: 0x8037_5BD8,
        name: 0x8032_498C,
        data: 0x8032_48F4,
        vtable: 0x8032_49A8,
        tag: None,
        release: 0x8012_4A80,
        event: 0x8037_5BC8,
    },
];

/// `r13 - 0x5370` (`0x8042A310`): scale word written by [`fn_80147bd0`].
const SDA_5370_E234: usize = SDA1_BASE - 0x5370;
/// `0x8042B848`: source of the `3.628f` scale written by [`fn_80147bd0`].
const F32_8042B848_E234: usize = 0x8042_B848;
/// `r13 - 0x5369` (`0x8042A317`): `.bss` guard byte of [`fn_80147bd0`].
const GUARD_5369_E234: usize = SDA1_BASE - 0x5369;
/// `0x80325360`: first of the three float triplets of the guarded block.
const BLOCK_80325360_E234: usize = 0x8032_5360;
/// `0x8042D758`: the `0.0f` filler of the guarded block.
const F32_8042D758_E234: usize = 0x8042_D758;
/// `0x8042D780`: the `10.0f` middle of the first triplet.
const F32_8042D780_E234: usize = 0x8042_D780;
/// `0x8042D834`: the `16.0f` middle of the second triplet.
const F32_8042D834_E234: usize = 0x8042_D834;
/// `0x8042D9E0`: the `28.0f` middle of the third triplet.
const F32_8042D9E0_E234: usize = 0x8042_D9E0;
/// `0x8005D7E0`: constructor passed to the `0x802DC9A0` array helper.
const FN_8005D7E0_E234: usize = 0x8005_D7E0;
/// `0x8005D830`: constructor argument of the `0x802DC9A0` array helper.
const ARG_8005D830_E234: usize = 0x8005_D830;
/// `0x803768B0`: first of the two `0xF0`-byte records built at the end.
const RECORD_803768B0_E234: usize = 0x8037_68B0;
/// `0x8014A320`: registration tag of the trailing `0x802DC780` call.
const FN_8014A320_E234: usize = 0x8014_A320;
/// `0x803768A0`: registry node of the trailing `0x802DC780` call.
const NODE_803768A0_E234: usize = 0x8037_68A0;

/// Installs the fifty-descriptor block at `0x80375C2C` plus its tail work.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80147BD0`.
/// - Extent: `0x80147BD0..0x8014A258` (9864 bytes); the `0x802DCD68`
///   register-save thunk follows `stwu r1,-0x760(r1)`, the one-shot guard
///   branch is `bc 4,2` at `0x8014A1A4`, and `bclr 20,0` is at `0x8014A254`.
/// - Table: slot 122 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBC8`).
/// - observed: `lfs f0,-0x7838(r2)` / `stfs f0,-0x5370(r13)` writes `3.628f`;
///   fifty descriptor blocks follow at `0x80375C2C` step `0x40`. The first
///   twelve use vtable `0x803265B0`, take their tenth word from one of the
///   thirteen twelve-byte accessors `0x8014A340..0x8014A400` and register with
///   `0x8014A2C0`; the remaining thirty-eight use vtable `0x803265E4`, no tenth
///   word and `0x8014A260`. Every node is `state-0xC`.
/// - observed: while `0x8042A317` is zero, three float triplets are written to
///   `0x80325360` (`0,10,0` / `0,16,0` / `0,28,0`), then the guard becomes `1`.
///   The dead copies of those floats into the caller's frame are not
///   reproduced.
/// - observed: `0x802DC9A0(0x803768B0, 0x8005D7E0, 0x8005D830, 0xF0, 2)` and
///   `0x802DC780(0, 0x8014A320, 0x803768A0)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the SDA scale, builds fifty state descriptors and their registry
/// nodes, fills the guarded triplet block once, constructs two `0xF0`-byte
/// records and pushes one final node.
///
/// # Safety
/// The descriptors, the nodes, the guarded block and the record array must be
/// writable.
///
/// # Confidence
/// high - the fifty rows and the tail are literal; only the runtime role of
/// `0x80325360` is inferred from its use as a float-triplet block.
pub unsafe fn fn_80147bd0() {
    platform::write_f32(
        SDA_5370_E234,
        platform::read_f32(F32_8042B848_E234),
    );
    for entry in W234B_ROWS_80147BD0_E234.iter() {
        w234b_install(entry);
    }
    if platform::read_u8(GUARD_5369_E234) == 0 {
        let zero = platform::read_f32(F32_8042D758_E234);
        platform::write_f32(BLOCK_80325360_E234, zero);
        platform::write_f32(
            BLOCK_80325360_E234 + 0x04,
            platform::read_f32(F32_8042D780_E234),
        );
        platform::write_f32(BLOCK_80325360_E234 + 0x08, zero);
        platform::write_f32(BLOCK_80325360_E234 + 0x0C, zero);
        platform::write_f32(
            BLOCK_80325360_E234 + 0x10,
            platform::read_f32(F32_8042D834_E234),
        );
        platform::write_f32(BLOCK_80325360_E234 + 0x14, zero);
        platform::write_f32(BLOCK_80325360_E234 + 0x18, zero);
        platform::write_f32(
            BLOCK_80325360_E234 + 0x1C,
            platform::read_f32(F32_8042D9E0_E234),
        );
        platform::write_f32(BLOCK_80325360_E234 + 0x20, zero);
        platform::write_u8(GUARD_5369_E234, 1);
    }
    platform::call5(
        FN_802DC9A0,
        RECORD_803768B0_E234 as u32,
        FN_8005D7E0_E234 as u32,
        ARG_8005D830_E234 as u32,
        0xF0,
        2,
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_8014A320_E234 as u32,
        NODE_803768A0_E234 as u32,
    );
}

/// Handler rows of `0x80147BD0`, one per descriptor block.
const W234B_ROWS_80147BD0_E234: [W234B_Descriptor; 50] = [
    W234B_Descriptor {
        state: 0x8037_5C2C,
        name: 0x8032_6050,
        data: 0x8032_5948,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_425C),
        release: 0x8014_A2C0,
        event: 0x8037_5C20,
    },
    W234B_Descriptor {
        state: 0x8037_5C6C,
        name: 0x8032_6068,
        data: 0x8032_596C,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_429C),
        release: 0x8014_A2C0,
        event: 0x8037_5C60,
    },
    W234B_Descriptor {
        state: 0x8037_5CAC,
        name: 0x8032_6080,
        data: 0x8032_5990,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_431C),
        release: 0x8014_A2C0,
        event: 0x8037_5CA0,
    },
    W234B_Descriptor {
        state: 0x8037_5CEC,
        name: 0x8032_6098,
        data: 0x8032_59B4,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_435C),
        release: 0x8014_A2C0,
        event: 0x8037_5CE0,
    },
    W234B_Descriptor {
        state: 0x8037_5D2C,
        name: 0x8032_60B0,
        data: 0x8032_59D8,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_439C),
        release: 0x8014_A2C0,
        event: 0x8037_5D20,
    },
    W234B_Descriptor {
        state: 0x8037_5D6C,
        name: 0x8032_60C8,
        data: 0x8032_59FC,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_42DC),
        release: 0x8014_A2C0,
        event: 0x8037_5D60,
    },
    W234B_Descriptor {
        state: 0x8037_5DAC,
        name: 0x8032_60E4,
        data: 0x8032_5A20,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_43DC),
        release: 0x8014_A2C0,
        event: 0x8037_5DA0,
    },
    W234B_Descriptor {
        state: 0x8037_5DEC,
        name: 0x8032_60FC,
        data: 0x8032_5A44,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_441C),
        release: 0x8014_A2C0,
        event: 0x8037_5DE0,
    },
    W234B_Descriptor {
        state: 0x8037_5E2C,
        name: 0x8032_6114,
        data: 0x8032_5A68,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_445C),
        release: 0x8014_A2C0,
        event: 0x8037_5E20,
    },
    W234B_Descriptor {
        state: 0x8037_5E6C,
        name: 0x8032_6130,
        data: 0x8032_5A8C,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_449C),
        release: 0x8014_A2C0,
        event: 0x8037_5E60,
    },
    W234B_Descriptor {
        state: 0x8037_5EAC,
        name: 0x8032_6148,
        data: 0x8032_5AB0,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_459C),
        release: 0x8014_A2C0,
        event: 0x8037_5EA0,
    },
    W234B_Descriptor {
        state: 0x8037_5EEC,
        name: 0x8032_6160,
        data: 0x8032_5AD4,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_45DC),
        release: 0x8014_A2C0,
        event: 0x8037_5EE0,
    },
    W234B_Descriptor {
        state: 0x8037_5F30,
        name: 0x8032_6178,
        data: 0x8032_5AF8,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_5F20,
    },
    W234B_Descriptor {
        state: 0x8037_5F70,
        name: 0x8032_6198,
        data: 0x8032_5B1C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_5F60,
    },
    W234B_Descriptor {
        state: 0x8037_5FB0,
        name: 0x8032_61B8,
        data: 0x8032_5B40,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_5FA0,
    },
    W234B_Descriptor {
        state: 0x8037_5FF0,
        name: 0x8032_61D4,
        data: 0x8032_5B64,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_5FE0,
    },
    W234B_Descriptor {
        state: 0x8037_6030,
        name: 0x8032_61EC,
        data: 0x8032_5B88,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6020,
    },
    W234B_Descriptor {
        state: 0x8037_6070,
        name: 0x8032_6204,
        data: 0x8032_5BAC,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6060,
    },
    W234B_Descriptor {
        state: 0x8037_60B0,
        name: 0x8032_621C,
        data: 0x8032_5BD0,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_60A0,
    },
    W234B_Descriptor {
        state: 0x8037_60F0,
        name: 0x8032_6238,
        data: 0x8032_5BF4,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_60E0,
    },
    W234B_Descriptor {
        state: 0x8037_6130,
        name: 0x8032_6250,
        data: 0x8032_5C18,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6120,
    },
    W234B_Descriptor {
        state: 0x8037_6170,
        name: 0x8032_6268,
        data: 0x8032_5C3C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6160,
    },
    W234B_Descriptor {
        state: 0x8037_61B0,
        name: 0x8032_6288,
        data: 0x8032_5C60,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_61A0,
    },
    W234B_Descriptor {
        state: 0x8037_61F0,
        name: 0x8032_62A4,
        data: 0x8032_5C84,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_61E0,
    },
    W234B_Descriptor {
        state: 0x8037_6230,
        name: 0x8032_62C0,
        data: 0x8032_5CA8,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6220,
    },
    W234B_Descriptor {
        state: 0x8037_6270,
        name: 0x8032_62D8,
        data: 0x8032_5CCC,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6260,
    },
    W234B_Descriptor {
        state: 0x8037_62B0,
        name: 0x8032_62F8,
        data: 0x8032_5CF0,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_62A0,
    },
    W234B_Descriptor {
        state: 0x8037_62F0,
        name: 0x8032_6314,
        data: 0x8032_5D14,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_62E0,
    },
    W234B_Descriptor {
        state: 0x8037_6330,
        name: 0x8032_6330,
        data: 0x8032_5D38,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6320,
    },
    W234B_Descriptor {
        state: 0x8037_6370,
        name: 0x8032_6350,
        data: 0x8032_5D5C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6360,
    },
    W234B_Descriptor {
        state: 0x8037_63B0,
        name: 0x8032_636C,
        data: 0x8032_5D80,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_63A0,
    },
    W234B_Descriptor {
        state: 0x8037_63F0,
        name: 0x8032_6384,
        data: 0x8032_5DA4,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_63E0,
    },
    W234B_Descriptor {
        state: 0x8037_6430,
        name: 0x8032_63A0,
        data: 0x8032_5DC8,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6420,
    },
    W234B_Descriptor {
        state: 0x8037_6470,
        name: 0x8032_63BC,
        data: 0x8032_5DEC,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6460,
    },
    W234B_Descriptor {
        state: 0x8037_64B0,
        name: 0x8032_63D8,
        data: 0x8032_5E10,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_64A0,
    },
    W234B_Descriptor {
        state: 0x8037_64F0,
        name: 0x8032_63F4,
        data: 0x8032_5E34,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_64E0,
    },
    W234B_Descriptor {
        state: 0x8037_6530,
        name: 0x8032_6410,
        data: 0x8032_5E58,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6520,
    },
    W234B_Descriptor {
        state: 0x8037_6570,
        name: 0x8032_642C,
        data: 0x8032_5E7C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6560,
    },
    W234B_Descriptor {
        state: 0x8037_65B0,
        name: 0x8032_6448,
        data: 0x8032_5EA0,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_65A0,
    },
    W234B_Descriptor {
        state: 0x8037_65F0,
        name: 0x8032_6464,
        data: 0x8032_5EC4,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_65E0,
    },
    W234B_Descriptor {
        state: 0x8037_662C,
        name: 0x8032_6480,
        data: 0x8032_5EE8,
        vtable: 0x8032_65B0,
        tag: Some(0x8035_4B2C),
        release: 0x8014_A2C0,
        event: 0x8037_6620,
    },
    W234B_Descriptor {
        state: 0x8037_6670,
        name: 0x8032_649C,
        data: 0x8032_5F0C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6660,
    },
    W234B_Descriptor {
        state: 0x8037_66B0,
        name: 0x8032_64B8,
        data: 0x8032_5F30,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_66A0,
    },
    W234B_Descriptor {
        state: 0x8037_66F0,
        name: 0x8032_64D4,
        data: 0x8032_5F54,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_66E0,
    },
    W234B_Descriptor {
        state: 0x8037_6730,
        name: 0x8032_64F0,
        data: 0x8032_5F78,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6720,
    },
    W234B_Descriptor {
        state: 0x8037_6770,
        name: 0x8032_6510,
        data: 0x8032_5F9C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6760,
    },
    W234B_Descriptor {
        state: 0x8037_67B0,
        name: 0x8032_6530,
        data: 0x8032_5FC0,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_67A0,
    },
    W234B_Descriptor {
        state: 0x8037_67F0,
        name: 0x8032_6550,
        data: 0x8032_5FE4,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_67E0,
    },
    W234B_Descriptor {
        state: 0x8037_6830,
        name: 0x8032_656C,
        data: 0x8032_6008,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6820,
    },
    W234B_Descriptor {
        state: 0x8037_6870,
        name: 0x8032_6590,
        data: 0x8032_602C,
        vtable: 0x8032_65E4,
        tag: None,
        release: 0x8014_A260,
        event: 0x8037_6860,
    },
];

/// `r13 - 0x535C` (`0x8042A324`): `.bss` guard byte of [`fn_801573b0`].
const GUARD_535C_E234: usize = SDA1_BASE - 0x535C;
/// `0x80376F78`: first word of the `0xE4`-byte block of [`fn_801573b0`].
const BLOCK_80376F78_E234: usize = 0x8037_6F78;
/// `0x8042C498`: halfword (480) stored at `+0xE0` of the block.
const HALF_8042C498_E234: usize = 0x8042_C498;
/// Destination offsets and float sources of the guarded block.
const W234B_BLOCK_801573B0_E234: [(usize, usize); 56] = [
    (0x00, 0x8042_C49C),
    (0x04, 0x8042_C4A4),
    (0x08, 0x8042_C4AC),
    (0x0C, 0x8042_C4B4),
    (0x10, 0x8042_C4BC),
    (0x14, 0x8042_C4C4),
    (0x18, 0x8042_C4CC),
    (0x1C, 0x8042_C4D4),
    (0x20, 0x8042_C4DC),
    (0x24, 0x8042_C4E4),
    (0x28, 0x8042_C4EC),
    (0x2C, 0x8042_C4F4),
    (0x30, 0x8042_C4FC),
    (0x34, 0x8042_C504),
    (0x38, 0x8042_C50C),
    (0x3C, 0x8042_C514),
    (0x40, 0x8042_C51C),
    (0x44, 0x8042_C524),
    (0x48, 0x8042_C52C),
    (0x4C, 0x8042_C534),
    (0x50, 0x8042_C53C),
    (0x54, 0x8042_C50C),
    (0x58, 0x8042_C514),
    (0x5C, 0x8042_C51C),
    (0x60, 0x8042_C524),
    (0x64, 0x8042_C52C),
    (0x68, 0x8042_C534),
    (0x6C, 0x8042_C53C),
    (0x70, 0x8042_C4A0),
    (0x74, 0x8042_C4A8),
    (0x78, 0x8042_C4B0),
    (0x7C, 0x8042_C4B8),
    (0x80, 0x8042_C4C0),
    (0x84, 0x8042_C4C8),
    (0x88, 0x8042_C4D0),
    (0x8C, 0x8042_C4D8),
    (0x90, 0x8042_C4E0),
    (0x94, 0x8042_C4E8),
    (0x98, 0x8042_C4F0),
    (0x9C, 0x8042_C4F8),
    (0xA0, 0x8042_C500),
    (0xA4, 0x8042_C508),
    (0xA8, 0x8042_C510),
    (0xAC, 0x8042_C518),
    (0xB0, 0x8042_C520),
    (0xB4, 0x8042_C528),
    (0xB8, 0x8042_C530),
    (0xBC, 0x8042_C538),
    (0xC0, 0x8042_C540),
    (0xC4, 0x8042_C510),
    (0xC8, 0x8042_C518),
    (0xCC, 0x8042_C520),
    (0xD0, 0x8042_C528),
    (0xD4, 0x8042_C530),
    (0xD8, 0x8042_C538),
    (0xDC, 0x8042_C540),
];

/// Fills a 56-float block once, guarded by an SDA byte.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x801573B0`.
/// - Extent: `0x801573B0..0x801576D8` (808 bytes); the prologue saves
///   `f14..f31` with `stfd`/`psq_st` pairs, the guard branch is `bc 4,2` at
///   `0x8015744C`, and `bclr 20,0` is at `0x801576D4`.
/// - Table: slot 124 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBD0`).
/// - observed: `lbz r0,-0x535C(r13)` plus `extsb.`; while zero, 56 `stfs`
///   write `0x80376F78..0x80377054` from `0x8042C49C..0x8042C540` and
///   `sth r3,0xE0(r4)` writes the halfword 480 read from `0x8042C498`; then the
///   guard becomes `1`.
/// - observed: the `stfd`/`lfd` pairs around each source float are CodeWarrior
///   spill slots; the value stored is the source float unchanged.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Only while `0x8042A324` is zero, fills the block at `0x80376F78` and sets
/// the guard byte.
///
/// # Safety
/// The block and the guard byte must be writable.
///
/// # Confidence
/// high - every destination offset and source word is literal; the guard test
/// and the halfword store are copied one-for-one.
pub unsafe fn fn_801573b0() {
    if platform::read_u8(GUARD_535C_E234) != 0 {
        return;
    }
    for (offset, source) in W234B_BLOCK_801573B0_E234.iter() {
        platform::write_f32(
            BLOCK_80376F78_E234 + offset,
            platform::read_f32(*source),
        );
    }
    platform::write_u16(
        BLOCK_80376F78_E234 + 0xE0,
        platform::read_u16(HALF_8042C498_E234),
    );
    platform::write_u8(GUARD_535C_E234, 1);
}

// ---------------------------------------------------------------------------
// Wave 235 lane B: `.ctors` slots 127-140 (`0x802EDBDC..0x802EDC10`).
//
// Table base `0x802ED9E0`; slot `n`'s word is at `0x802ED9E0 + 4n`. Two of the
// fourteen slots already have translated bodies in the tree and are referenced
// rather than repeated:
//
// | slot | word | target | existing body |
// |-----:|------|--------|---------------|
// | 127 | `0x802EDBDC` | `0x8015DDD0` | [`crate::engine::display::fn_8015ddd0`] |
// | 130 | `0x802EDBE8` | `0x80162ED0` | [`crate::resource::registry::fn_80162ed0`] |
//
// The twelve bodies below are the remaining slots (128, 129 and 131-140;
// 792 bytes). None of them has a `functions.tsv` row, so all twelve are
// untabled and reached only through `.ctors`:
//
// | slot | word | target | extent | body |
// |-----:|------|--------|--------|------|
// | 128 | `0x802EDBE0` | `0x8015F940` | 84 B | null-name header |
// | 129 | `0x802EDBE4` | `0x80161440` | 20 B | two SDA words |
// | 131 | `0x802EDBEC` | `0x8016AA20` | 76 B | four `u16` rows |
// | 132 | `0x802EDBF0` | `0x8016C190` | 220 B | handler table + mount queue |
// | 133 | `0x802EDBF4` | `0x8016DA60` | 56 B | two list heads + clear |
// | 134 | `0x802EDBF8` | `0x8016F190` | 88 B | twelve-float block |
// | 135 | `0x802EDBFC` | `0x8016F670` | 88 B | array constructor + node |
// | 136 | `0x802EDC00` | `0x80170BF0` | 84 B | twelve-float basis |
// | 137 | `0x802EDC04` | `0x8017E4E0` | 20 B | one link store |
// | 138 | `0x802EDC08` | `0x80184690` | 16 B | one clear store |
// | 139 | `0x802EDC0C` | `0x80184BE0` | 20 B | one link store |
// | 140 | `0x802EDC10` | `0x80186090` | 20 B | one link store |
//
// Extents are prologue-to-`bclr 20,0` (or prologue-to-tail-`b`) inclusive,
// measured by recursive descent from the raw words
// (`work/wave235/laneB_dis.txt`). The bodies are ordinary startup constructors:
// literal stores plus calls to helpers already reconstructed elsewhere in the
// tree. `0x802DCD60`/`0x802DCDAC` in slot 132 are the compiler's
// `__savegpr`/`__restgpr` stubs and are not reproduced.
//
// Every store and call below was re-derived by a concrete interpreter over the
// DOL words (`work/wave235/laneB_emu.mjs`; trace in `laneB_emu.txt`), and the
// interpreter's address/value pairs are the ones the Rust writes.
// ---------------------------------------------------------------------------

/// `0x8015F800`: release callback registered for the header built by
/// [`fn_8015f940`].
///
/// observed: `if (param_1 != 0 && 0 < param_2) 0x802B9280(); return param_1;`
/// (`work/ghidra/main-gb/export/8015f800_FUN_8015f800.c`).
const FN_8015F800_B235: usize = 0x8015_F800;
/// `0x80377A04`: shared name-header word installed by [`fn_8015f940`] (the same
/// address as [`W229B_TAG_SHARED`]).
const HEADER_80377A04_B235: usize = 0x8037_7A04;
/// `0x803779F8`: registry node pushed by [`fn_8015f940`].
const NODE_803779F8_B235: usize = 0x8037_79F8;
/// `r13 - 0x5320` (`0x8042A360`): name counter reset by [`fn_8015f940`].
const SDA_5320_B235: usize = SDA1_BASE - 0x5320;

/// Initialises the shared header at `0x80377A04` with a null name.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8015F940`.
/// - Extent: `0x8015F940..0x8015F994` (84 bytes); `stwu r1,-0x10(r1)` at the
///   entry and `bclr 20,0` at `0x8015F990`.
/// - Table: slot 128 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBE0`).
/// - observed: `li r4,0`; `lis r31,0x8037 / addi r3,r31,0x7A04`;
///   `stw r0,-0x5320(r13)` with `r0 = 0`; `bl 0x8015F7C0`; then
///   `0x802DC780(0x80377A04, 0x8015F800, 0x803779F8)`.
/// - observed: the body installs no vtable word and no payload of its own; the
///   `0x8015F7C0` helper writes the shared header word `0x80328A18` at `+0`,
///   the null name at `+4` and the index `0` at `+8`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears the name counter at `0x8042A360`, initialises the header word at
/// `0x80377A04` and pushes the node `0x803779F8` onto the shutdown list.
///
/// # Safety
/// The header, the node and the counter must be writable, and `0x8015F800`
/// must be a valid release callback.
///
/// # Confidence
/// high - four literal constants and the two calls; the byte trace matches.
pub unsafe fn fn_8015f940() {
    // observed: the counter is cleared before the header is named.
    platform::write_u32(SDA_5320_B235, 0);
    platform::call2(FN_8015F7C0, HEADER_80377A04_B235 as u32, 0);
    platform::call3(
        FN_802DC780,
        HEADER_80377A04_B235 as u32,
        FN_8015F800_B235 as u32,
        NODE_803779F8_B235 as u32,
    );
}

/// `r13 - 0x52F0` (`0x8042A390`): first word stored by [`fn_80161440`].
const SDA_52F0_B235: usize = SDA1_BASE - 0x52F0;
/// `r13 - 0x52EC` (`0x8042A394`): second word stored by [`fn_80161440`].
const SDA_52EC_B235: usize = SDA1_BASE - 0x52EC;

/// Publishes the pair `(100, 101)` into two adjacent SDA words.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80161440`.
/// - Extent: `0x80161440..0x80161454` (20 bytes, leaf).
/// - Table: slot 129 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBE4`).
/// - observed: `li r3,100`; `li r0,101`; `stw r3,-0x52F0(r13)`;
///   `stw r0,-0x52EC(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. Both addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `100` and `101` to `0x8042A390`/`0x8042A394`.
///
/// # Safety
/// Both words must be writable.
///
/// # Confidence
/// high - the whole body is five literal instructions.
pub unsafe fn fn_80161440() {
    platform::write_u32(SDA_52F0_B235, 100);
    platform::write_u32(SDA_52EC_B235, 101);
}

/// Four three-`u16` rows written by [`fn_8016aa20`], in address order.
///
/// observed: the rows are the zero row and the three one-hot rows
/// `[1,0,0]`, `[0,1,0]`, `[0,0,1]`; the last three share a single `1`.
const ROWS_8042A3F0_B235: [(usize, [u16; 3]); 4] = [
    (SDA1_BASE - 0x5290, [0, 0, 0]),
    (SDA1_BASE - 0x5288, [1, 0, 0]),
    (SDA1_BASE - 0x5280, [0, 1, 0]),
    (SDA1_BASE - 0x5278, [0, 0, 1]),
];

/// Writes four three-halfword rows into SDA1.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8016AA20`.
/// - Extent: `0x8016AA20..0x8016AA6C` (76 bytes, leaf).
/// - Table: slot 131 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBEC`).
/// - observed: four `addi rX,r13,-0x52XX` bases, then twelve `sth`: each base
///   receives `{0,0,0}`, `{1,0,0}`, `{0,1,0}` or `{0,0,1}` at
///   `+0`/`+2`/`+4`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills `0x8042A3F0..0x8042A40E` with the four rows.
///
/// # Safety
/// The twelve halfwords must be writable.
///
/// # Confidence
/// high - twelve stores with literal addresses and literal values.
pub unsafe fn fn_8016aa20() {
    for (base, row) in ROWS_8042A3F0_B235.iter() {
        let mut index = 0usize;
        while index < 3 {
            platform::write_u16(*base + index * 2, row[index]);
            index += 1;
        }
    }
}

/// One `{ code, kind, slot }` row of the five-entry table written by
/// [`fn_8016c190`].
struct B235_Handler {
    /// Word at `+0`: handler code pointer.
    code: u32,
    /// Byte at `+4`: handler selector.
    kind: u8,
    /// Word at `+8`: SDA slot the handler receives.
    slot: usize,
}

/// `0x80377AE4`: first of the five twelve-byte handler rows.
const HANDLERS_80377AE4_B235: usize = 0x8037_7AE4;
/// `0x80377B20`: mount-queue object initialised after the rows.
const QUEUE_80377B20_B235: usize = 0x8037_7B20;
/// Rows written by [`fn_8016c190`], in address order.
const ROWS_8016C190_B235: [B235_Handler; 5] = [
    B235_Handler {
        code: 0x8032_97E0,
        kind: 3,
        slot: SDA1_BASE - 0x6210,
    },
    B235_Handler {
        code: 0x8032_97CC,
        kind: 5,
        slot: SDA1_BASE - 0x6208,
    },
    B235_Handler {
        code: 0x8032_97B8,
        kind: 7,
        slot: SDA1_BASE - 0x6204,
    },
    B235_Handler {
        code: 0x8032_97A4,
        kind: 8,
        slot: SDA1_BASE - 0x6200,
    },
    B235_Handler {
        code: 0x8032_9790,
        kind: 4,
        slot: SDA1_BASE - 0x61F8,
    },
];

/// Builds the five-entry handler table at `0x80377AE4`, then initialises the
/// mount queue at `0x80377B20`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8016C190`.
/// - Extent: `0x8016C190..0x8016C26C` (220 bytes); `stwu r1,-0x30(r1)` and the
///   `0x802DCD60` register-save stub at the entry, the `0x802DCDAC` restore
///   stub before `bclr 20,0` at `0x8016C268`.
/// - Table: slot 132 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBF0`).
/// - observed: `lis r27,0x8037 / addi r27,r27,0x7AC0`, five rows of
///   `{stw,stb,stw}` at `+0x24`/`+0x30`/`+0x3C`/`+0x48`/`+0x54`, then
///   `addi r3,r27,0x60` and `bl 0x8016B540`.
/// - observed: the code words descend by `0x14` (`0x803297E0`, `0x803297CC`,
///   `0x803297B8`, `0x803297A4`, `0x80329790`), the selectors are
///   `3,5,7,8,4` and the SDA words ascend (`0x80429470`..`0x80429488`).
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills `0x80377AE4..0x80377B20` and publishes the queue object through
/// [`crate::resource::registry::mount_queue_init`].
///
/// # Safety
/// The table, the queue object and the SDA words must be writable; the OS
/// mutex/thread-queue calls inside the callee must be usable at ctor time.
///
/// # Confidence
/// high - fifteen literal stores and one call; the callee is already
/// reconstructed as `crate::resource::registry::mount_queue_init`.
pub unsafe fn fn_8016c190() {
    let mut base = HANDLERS_80377AE4_B235;
    for row in ROWS_8016C190_B235.iter() {
        platform::write_u32(base, row.code);
        platform::write_u8(base + 4, row.kind);
        platform::write_u32(base + 8, row.slot as u32);
        base += 0x0C;
    }
    crate::resource::registry::mount_queue_init(QUEUE_80377B20_B235);
}

/// `0x80377C30`: first of the two three-word list heads written by
/// [`fn_8016da60`].
const HEAD_A_80377C30_B235: usize = 0x8037_7C30;
/// `0x80377C3C`: second list head.
const HEAD_B_80377C3C_B235: usize = 0x8037_7C3C;
/// `0x80377B1C`: node word the first head links to.
const NODE_80377B1C_B235: usize = 0x8037_7B1C;
/// `0x80296750`: two-word clear tail-called with the SDA node.
const FN_80296750_B235: usize = 0x8029_6750;
/// `r13 - 0x5248` (`0x8042A438`): node cleared by the tail call.
const SDA_5248_B235: usize = SDA1_BASE - 0x5248;

/// Initialises two adjacent three-word list heads and clears an SDA node.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8016DA60`.
/// - Extent: `0x8016DA60..0x8016DA98` (56 bytes, leaf); the last instruction is
///   an unconditional `b 0x80296750` with `r3 = r13 - 0x5248`.
/// - Table: slot 133 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBF4`).
/// - observed: head `0x80377C30` gets `{0x80377B1C, 0x80377B1C, 0}` and head
///   `0x80377C3C` gets `{0x80377C3C, 0x80377C3C, 0}`; then
///   `0x80296750(0x8042A438)`, which clears the node's two words.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the six head words and clears `0x8042A438`/`0x8042A43C`.
///
/// # Safety
/// The heads and the cleared node must be writable.
///
/// # Confidence
/// high - six literal stores and one tail call; the trace matches.
pub unsafe fn fn_8016da60() {
    platform::write_u32(HEAD_B_80377C3C_B235 + 4, HEAD_B_80377C3C_B235 as u32);
    platform::write_u32(HEAD_A_80377C30_B235 + 4, NODE_80377B1C_B235 as u32);
    platform::write_u32(HEAD_A_80377C30_B235, NODE_80377B1C_B235 as u32);
    platform::write_u32(HEAD_A_80377C30_B235 + 8, 0);
    platform::write_u32(HEAD_B_80377C3C_B235, HEAD_B_80377C3C_B235 as u32);
    platform::write_u32(HEAD_B_80377C3C_B235 + 8, 0);
    platform::call1(FN_80296750_B235, SDA_5248_B235 as u32);
}

/// `r2 - 0x535C` (`0x8042DD24`): `1.0f`, the basis value of
/// [`fn_8016f190`].
const F32_SDA2_535C_B235: usize = SDA2_BASE - 0x535C;
/// `r2 - 0x5358` (`0x8042DD28`): `0.0f`, the repeated filler of
/// [`fn_8016f190`].
const F32_SDA2_5358_B235: usize = SDA2_BASE - 0x5358;
/// `0x80377C58`: twelve-float destination of [`fn_8016f190`].
const BLOCK_80377C58_B235: usize = 0x8037_7C58;

/// Packs a twelve-float basis block through the `0x8016EBA0` helper.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8016F190`.
/// - Extent: `0x8016F190..0x8016F1E8` (88 bytes); `stwu r1,-0x20(r1)` at the
///   entry and `bclr 20,0` at `0x8016F1E4`.
/// - Table: slot 134 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBF8`).
/// - observed: `lfs f2,-0x5358(r2)` (`0.0f`) and `lfs f1,-0x535C(r2)`
///   (`1.0f`); six `fmr` copies move `1.0` into `f6` and `0.0` into
///   `f3`/`f4`/`f5`/`f7`/`f8` (so the eight register arguments are
///   `1,0,0,0, 0,1,0,0`); four `stfs` fill the outgoing stack slots at
///   `8(r1)..0x14(r1)` with `0, 0, 1, 0`; `bl 0x8016EBA0` with `r3 = 0x80377C58`.
/// - observed: the callee `0x8016EBA0` stores its eight register floats and
///   four stack floats in order at `dst + 0..0x2C`, i.e. the twelve words are
///   `1,0,0,0, 0,1,0,0, 0,0,1,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes twelve floats at `0x80377C58..0x80377C88`.
///
/// # Safety
/// The destination must be writable for 48 bytes.
///
/// # Confidence
/// high - the operand order is read from the `fmr`/`stfs` sequence and the
/// callee is already reconstructed as
/// [`crate::resource::registry::fn_8016eba0`].
pub unsafe fn fn_8016f190() {
    let one = platform::read_f32(F32_SDA2_535C_B235);
    let zero = platform::read_f32(F32_SDA2_5358_B235);
    let values = [
        one, zero, zero, zero, zero, one, zero, zero, zero, zero, one, zero,
    ];
    crate::resource::registry::fn_8016eba0(BLOCK_80377C58_B235, values);
}

/// `0x80377CA8`: four-element, 24-byte-stride array built by
/// [`fn_8016f670`].
const ARRAY_80377CA8_B235: usize = 0x8037_7CA8;
/// `0x8016F6D0`: element constructor of the array.
const FN_8016F6D0_B235: usize = 0x8016_F6D0;
/// `0x8016F6E0`: element destructor of the array.
const FN_8016F6E0_B235: usize = 0x8016_F6E0;
/// `0x8016F720`: shutdown callback registered for the array.
const FN_8016F720_B235: usize = 0x8016_F720;
/// `0x80377C98`: registry node of the array.
const NODE_80377C98_B235: usize = 0x8037_7C98;

/// Runs the array constructor and registers its shutdown node.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8016F670`.
/// - Extent: `0x8016F670..0x8016F6C8` (88 bytes); `stwu r1,-0x10(r1)` at the
///   entry and `bclr 20,0` at `0x8016F6C4`.
/// - Table: slot 135 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDBFC`).
/// - observed: `0x802DC9A0(0x80377CA8, 0x8016F6D0, 0x8016F6E0, 0x18, 4)`
///   (`r6 = 24` is the stride, `r7 = 4` the count); then
///   `0x802DC780(0, 0x8016F720, 0x80377C98)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Constructs four 24-byte elements at `0x80377CA8` and pushes the shutdown
/// node.
///
/// # Safety
/// The array and the node must be writable; both callbacks must be valid.
///
/// # Confidence
/// high - both calls are literal and use the same argument order as the
/// wave-228/234 array constructors.
pub unsafe fn fn_8016f670() {
    platform::call5(
        FN_802DC9A0,
        ARRAY_80377CA8_B235 as u32,
        FN_8016F6D0_B235 as u32,
        FN_8016F6E0_B235 as u32,
        0x18,
        4,
    );
    platform::call3(
        FN_802DC780,
        0,
        FN_8016F720_B235 as u32,
        NODE_80377C98_B235 as u32,
    );
}

/// `0x80377DC8`: twelve-float block written by [`fn_80170bf0`].
const BLOCK_80377DC8_B235: usize = 0x8037_7DC8;
/// `r2 - 0x5324` (`0x8042DD5C`): `0.0f`, the block filler.
const F32_SDA2_5324_B235: usize = SDA2_BASE - 0x5324;
/// `r2 - 0x5328` (`0x8042DD58`): `1.0f`, the basis entry.
const F32_SDA2_5328_B235: usize = SDA2_BASE - 0x5328;
/// `(offset, source)` for the twelve stores of [`fn_80170bf0`]: source `0` is
/// the `r2-0x5324` filler and source `1` the `r2-0x5328` basis.
const STORES_80170BF0_B235: [(usize, usize); 12] = [
    (0x00, 0),
    (0x04, 0),
    (0x08, 0),
    (0x0C, 1),
    (0x10, 0),
    (0x14, 0),
    (0x18, 0),
    (0x1C, 1),
    (0x20, 0),
    (0x24, 0),
    (0x28, 0),
    (0x2C, 1),
];

/// Writes the four-row, three-column float basis at `0x80377DC8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80170BF0`.
/// - Extent: `0x80170BF0..0x80170C44` (84 bytes, leaf).
/// - Table: slot 136 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC00`).
/// - observed: `lfs f1,-0x5324(r2)` (`0.0f`) and `lfs f0,-0x5328(r2)`
///   (`1.0f`); twelve `stfs` fill `0x80377DC8..0x80377DF4` as the rows
///   `[0,0,0]`, `[1,0,0]`, `[0,1,0]`, `[0,0,1]`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve float stores.
///
/// # Safety
/// The block must be writable for 48 bytes.
///
/// # Confidence
/// high - twelve stores with literal offsets; the sources are read, not
/// folded, because the original copies them.
pub unsafe fn fn_80170bf0() {
    let sources = [
        platform::read_f32(F32_SDA2_5324_B235),
        platform::read_f32(F32_SDA2_5328_B235),
    ];
    for (offset, source) in STORES_80170BF0_B235.iter() {
        platform::write_f32(BLOCK_80377DC8_B235 + *offset, sources[*source]);
    }
}

/// `0x80377FD8`: word made to point at [`SELF_80378010_B235`] by
/// [`fn_8017e4e0`].
const SLOT_80377FD8_B235: usize = 0x8037_7FD8;
/// `0x80378010`: shared target word.
const SELF_80378010_B235: usize = 0x8037_8010;
/// `0x80378018`: second word pointing at [`SELF_80378010_B235`].
const SLOT_80378018_B235: usize = 0x8037_8018;
/// `0x80378020`: third word pointing at [`SELF_80378010_B235`].
const SLOT_80378020_B235: usize = 0x8037_8020;

/// Links `0x80377FD8` to the shared target word `0x80378010`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8017E4E0`.
/// - Extent: `0x8017E4E0..0x8017E4F4` (20 bytes, leaf).
/// - Table: slot 137 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC04`).
/// - observed: `lis r4,0x8038 / addi r4,r4,-0x7FF0` (`0x80378010`);
///   `lis r3,0x8037 / stw r4,0x7FD8(r3)` (`0x80377FD8`); `bclr 20,0`.
///
/// # Arguments
/// None. Both addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores the word `0x80378010`.
///
/// # Safety
/// `0x80377FD8` must be writable.
///
/// # Confidence
/// high - four literal instructions.
pub unsafe fn fn_8017e4e0() {
    platform::write_u32(SLOT_80377FD8_B235, SELF_80378010_B235 as u32);
}

/// Clears the shared word `0x80378010`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80184690`.
/// - Extent: `0x80184690..0x801846A0` (16 bytes, leaf).
/// - Table: slot 138 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC08`).
/// - observed: `lis r3,0x8038`; `li r0,0`; `stw r0,-0x7FF0(r3)`; `bclr 20,0`.
///
/// # Arguments
/// None. The address is a link-time constant.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes zero to `0x80378010`.
///
/// # Safety
/// The word must be writable.
///
/// # Confidence
/// high - four literal instructions.
pub unsafe fn fn_80184690() {
    platform::write_u32(SELF_80378010_B235, 0);
}

/// Links `0x80378018` to the shared target word `0x80378010`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80184BE0`.
/// - Extent: `0x80184BE0..0x80184BF4` (20 bytes, leaf).
/// - Table: slot 139 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC0C`).
/// - observed: `lis r4,0x8038 / addi r4,r4,-0x7FF0`; `lis r3,0x8038`;
///   `stw r4,-0x7FE8(r3)` (`0x80378018`); `bclr 20,0`.
///
/// # Arguments
/// None. Both addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores the word `0x80378010`.
///
/// # Safety
/// `0x80378018` must be writable.
///
/// # Confidence
/// high - four literal instructions.
pub unsafe fn fn_80184be0() {
    platform::write_u32(SLOT_80378018_B235, SELF_80378010_B235 as u32);
}

/// Links `0x80378020` to the shared target word `0x80378010`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80186090`.
/// - Extent: `0x80186090..0x801860A4` (20 bytes, leaf).
/// - Table: slot 140 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC10`).
/// - observed: `lis r4,0x8038 / addi r4,r4,-0x7FF0`; `lis r3,0x8038`;
///   `stw r4,-0x7FE0(r3)` (`0x80378020`); `bclr 20,0`.
///
/// # Arguments
/// None. Both addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores the word `0x80378010`.
///
/// # Safety
/// `0x80378020` must be writable.
///
/// # Confidence
/// high - four literal instructions.
pub unsafe fn fn_80186090() {
    platform::write_u32(SLOT_80378020_B235, SELF_80378010_B235 as u32);
}

// ===========================================================================
// Wave 235 lane C: `.ctors` slots 141-154 (`0x802EDC14..0x802EDC48`)
// ===========================================================================
//
// Table base `0x802ED9E0`; slot `n`'s word is at `0x802ED9E0 + 4n`. None of the
// fourteen targets has a `functions.tsv` row and none had a Rust body in the
// tree when this section was written, so all fourteen are translated here.
//
// | slot | word | target | extent | role |
// |-----:|------|--------|-------:|------|
// | 141 | `0x802EDC14` | `0x80188590` |     20 B | pointer store |
// | 142 | `0x802EDC18` | `0x80190F00` |    164 B | two guarded registrations |
// | 143 | `0x802EDC1C` | `0x80229D30` |     12 B | SDA pointer store |
// | 144 | `0x802EDC20` | `0x80229EC0` |     12 B | SDA pointer store |
// | 145 | `0x802EDC24` | `0x8022A580` |     12 B | SDA pointer store |
// | 146 | `0x802EDC28` | `0x8022A9B0` |     12 B | SDA pointer store |
// | 147 | `0x802EDC2C` | `0x8022B220` |     12 B | SDA pointer store |
// | 148 | `0x802EDC30` | `0x8022B3F0` |     24 B | zero a byte, tail call |
// | 149 | `0x802EDC34` | `0x80233110` |    132 B | two guarded registrations |
// | 150 | `0x802EDC38` | `0x8024DA70` |    624 B | bulk static initialiser |
// | 151 | `0x802EDC3C` | `0x80261D20` |     64 B | five SDA pointer stores |
// | 152 | `0x802EDC40` | `0x802677F0` |     12 B | SDA pointer store |
// | 153 | `0x802EDC44` | `0x80272C30` |     12 B | SDA pointer store |
// | 154 | `0x802EDC48` | `0x802753B0` |     12 B | tail call into an accessor |
//
// Extents are prologue-to-`blr` (or prologue-to-tail-`b`) inclusive, measured by
// recursive descent from the raw words. Every address below is resolved from
// the `lis`/`addi` pair as written; several of them therefore land one page
// below the `lis` immediate (for example slot 141's `0x80378028`, not
// `0x80388028`), and the literals follow the arithmetic, not the high half.
//
// Slots 143-147 and 151-153 chain five SDA words used as a small intrusive
// list: `0x8042AC10` is cleared, then `0x8042AC18` points at it,
// `0x8042AC20` at `0x8042AC18`, `0x8042AC28` at `0x8042AC20`, and
// `0x8042AC40` at `0x8042AC18`. Slots 152/153 repeat the pattern for the
// `0x8042AD00`/`0x8042AD30` pair.
// ---------------------------------------------------------------------------

/// `0x80378010`: static object installed by [`fn_80188590`].
const OBJ_80378010_C235: u32 = 0x8037_8010;
/// `0x80378028`: the pointer written by [`fn_80188590`].
const PTR_80378028_C235: usize = 0x8037_8028;

/// Installs the static object pointer at `0x80378028`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80188590`.
/// - Extent: `0x80188590..0x801885A4` (20 bytes, leaf).
/// - Table: slot 141 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC14`).
/// - observed: `lis r4,0x8038`; `lis r3,0x8038`; `addi r4,r4,-0x7FF0`;
///   `stw r4,-0x7FD8(r3)`; `bclr 20,0`. The two displacements resolve the pair
///   to `0x80378010` stored at `0x80378028`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word in `.data`.
///
/// # Safety
/// `0x80378028` must be writable.
///
/// # Confidence
/// high - the whole body is four literal instructions.
pub unsafe fn fn_80188590() {
    platform::write_u32(PTR_80378028_C235, OBJ_80378010_C235);
}

/// `0x8018B080`: state-bank constructor for the descriptor at `0x80378064`.
const FN_8018B080_C235: usize = 0x8018_B080;
/// `0x8018B5E0`: state-bank constructor for the descriptor at `0x8037806C`.
const FN_8018B5E0_C235: usize = 0x8018_B5E0;
/// `0x8018B090`: release callback of the first registration.
const FN_8018B090_C235: u32 = 0x8018_B090;
/// `0x8018B5F0`: release callback of the second registration.
const FN_8018B5F0_C235: u32 = 0x8018_B5F0;
/// `0x80378064`: first descriptor.
const DESC_80378064_C235: u32 = 0x8037_8064;
/// `0x8037806C`: second descriptor.
const DESC_8037806C_C235: u32 = 0x8037_806C;
/// `0x80378048`: registry node of the first descriptor.
const NODE_80378048_C235: u32 = 0x8037_8048;
/// `0x80378054`: registry node of the second descriptor.
const NODE_80378054_C235: u32 = 0x8037_8054;
/// `0x80378070`: one-shot guard byte of the first registration.
const GUARD_80378070_C235: usize = 0x8037_8070;
/// `0x80378071`: one-shot guard byte of the second registration.
const GUARD_80378071_C235: usize = 0x8037_8071;

/// Registers two state banks, each behind its own one-shot guard byte.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80190F00`.
/// - Extent: `0x80190F00..0x80190FA4` (164 bytes); `stwu r1,-0x10(r1)` at the
///   entry and `bclr 20,0` at `0x80190FA0`.
/// - Table: slot 142 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC18`).
/// - observed: while `0x80378070 == 0`: `0x8018B080(0x80378064)`, then
///   `0x802DC780(0x80378064, 0x8018B090, 0x80378048)`, then the guard is set.
/// - observed: while `0x80378071 == 0`: `0x8018B5E0(0x8037806C)`, then
///   `0x802DC780(0x8037806C, 0x8018B5F0, 0x80378054)`, then the guard is set.
/// - observed: both tests are `lbz`; `extsb.`; `bc 4,2` (branch-if-nonzero),
///   so any non-zero byte - including a negative one - counts as registered.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds two state banks and pushes their shutdown nodes, once per guard byte.
///
/// # Safety
/// The descriptors, the nodes and the guard bytes must be writable.
///
/// # Confidence
/// high - two identical guarded blocks with literal arguments.
pub unsafe fn fn_80190f00() {
    if platform::read_u8(GUARD_80378070_C235) == 0 {
        platform::call1(FN_8018B080_C235, DESC_80378064_C235);
        platform::call3(
            FN_802DC780,
            DESC_80378064_C235,
            FN_8018B090_C235,
            NODE_80378048_C235,
        );
        platform::write_u8(GUARD_80378070_C235, 1);
    }
    if platform::read_u8(GUARD_80378071_C235) == 0 {
        platform::call1(FN_8018B5E0_C235, DESC_8037806C_C235);
        platform::call3(
            FN_802DC780,
            DESC_8037806C_C235,
            FN_8018B5F0_C235,
            NODE_80378054_C235,
        );
        platform::write_u8(GUARD_80378071_C235, 1);
    }
}

/// `r13 - 0x4A70` (`0x8042AC10`): head of the small intrusive list.
const SDA_4A70_C235: usize = SDA1_BASE - 0x4A70;

/// Clears the head word of the small intrusive list.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80229D30`.
/// - Extent: `0x80229D30..0x80229D3C` (12 bytes, leaf).
/// - Table: slot 143 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC1C`).
/// - observed: `li r0,0`; `stw r0,-0x4A70(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_80229d30() {
    platform::write_u32(SDA_4A70_C235, 0);
}

/// `r13 - 0x4A68` (`0x8042AC18`): second word of the list chain.
const SDA_4A68_C235: usize = SDA1_BASE - 0x4A68;

/// Points the second chain word at the cleared head.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80229EC0`.
/// - Extent: `0x80229EC0..0x80229ECC` (12 bytes, leaf).
/// - Table: slot 144 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC20`).
/// - observed: `addi r0,r13,-0x4A70`; `stw r0,-0x4A68(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_80229ec0() {
    platform::write_u32(SDA_4A68_C235, SDA_4A70_C235 as u32);
}

/// `r13 - 0x4A60` (`0x8042AC20`): third word of the list chain.
const SDA_4A60_C235: usize = SDA1_BASE - 0x4A60;

/// Points the third chain word at the second.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8022A580`.
/// - Extent: `0x8022A580..0x8022A58C` (12 bytes, leaf).
/// - Table: slot 145 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC24`).
/// - observed: `addi r0,r13,-0x4A68`; `stw r0,-0x4A60(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_8022a580() {
    platform::write_u32(SDA_4A60_C235, SDA_4A68_C235 as u32);
}

/// `r13 - 0x4A58` (`0x8042AC28`): fourth word of the list chain.
const SDA_4A58_C235: usize = SDA1_BASE - 0x4A58;

/// Points the fourth chain word at the third.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8022A9B0`.
/// - Extent: `0x8022A9B0..0x8022A9BC` (12 bytes, leaf).
/// - Table: slot 146 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC28`).
/// - observed: `addi r0,r13,-0x4A60`; `stw r0,-0x4A58(r13)`; `bclr r20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_8022a9b0() {
    platform::write_u32(SDA_4A58_C235, SDA_4A60_C235 as u32);
}

/// `r13 - 0x4A40` (`0x8042AC40`): second entry of the small list.
const SDA_4A40_C235: usize = SDA1_BASE - 0x4A40;

/// Points the second list entry at the second chain word.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8022B220`.
/// - Extent: `0x8022B220..0x8022B22C` (12 bytes, leaf).
/// - Table: slot 147 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC2C`).
/// - observed: `addi r0,r13,-0x4A68`; `stw r0,-0x4A40(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_8022b220() {
    platform::write_u32(SDA_4A40_C235, SDA_4A68_C235 as u32);
}

/// `0x803DA0D8`: guard byte cleared by [`fn_8022b3f0`].
const GUARD_803DA0D8_C235: usize = 0x803D_A0D8;
/// `0x803DA0DC`: mutex handed to `0x801B2E20` by [`fn_8022b3f0`].
const MUTEX_803DA0DC_C235: u32 = 0x803D_A0DC;
/// `0x801B2E20`: mutex constructor (`OSInitMutex` elsewhere in the tree).
const FN_801B2E20_C235: usize = 0x801B_2E20;

/// Clears a guard byte and initialises a mutex.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8022B3F0`.
/// - Extent: `0x8022B3F0..0x8022B408` (24 bytes; the tail is
///   `b 0x801B2E20`, so the body has no `blr`).
/// - Table: slot 148 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC30`).
/// - observed: `lis r4,0x803E`; `li r0,0`; `addi r3,r4,-0x5F28`;
///   `stb r0,-0x5F28(r4)`; `addi r3,r3,4`; `b 0x801B2E20`. The displacements
///   resolve to byte `0x803DA0D8` and mutex `0x803DA0DC`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing; the tail call's result is the body's result and is unused.
///
/// # Side effects
/// Clears the guard byte and builds the mutex at `0x803DA0DC`.
///
/// # Safety
/// The guard byte and mutex must be writable.
///
/// # Confidence
/// high - the body is five literal instructions plus the tail call.
pub unsafe fn fn_8022b3f0() {
    platform::write_u8(GUARD_803DA0D8_C235, 0);
    platform::call1(FN_801B2E20_C235, MUTEX_803DA0DC_C235);
}

/// `0x80229360`: state-bank constructor for the descriptor at `0x8042AC54`.
const FN_80229360_C235: usize = 0x8022_9360;
/// `0x80229810`: state-bank constructor for the descriptor at `0x8042AC5C`.
const FN_80229810_C235: usize = 0x8022_9810;
/// `0x80239370`: release callback of the first registration.
const FN_80239370_C235: u32 = 0x8023_9370;
/// `0x80239820`: release callback of the second registration.
const FN_80239820_C235: u32 = 0x8023_9820;
/// `r13 - 0x4A2C` (`0x8042AC54`): first descriptor.
const DESC_8042AC54_C235: u32 = 0x8042_AC54;
/// `r13 - 0x4A24` (`0x8042AC5C`): second descriptor.
const DESC_8042AC5C_C235: u32 = 0x8042_AC5C;
/// `0x803DA108`: registry node of the first descriptor.
const NODE_803DA108_C235: u32 = 0x803D_A108;
/// `0x803DA114`: registry node of the second descriptor.
const NODE_803DA114_C235: u32 = 0x803D_A114;
/// `r13 - 0x4A20` (`0x8042AC60`): guard byte of the first registration.
const GUARD_8042AC60_C235: usize = SDA1_BASE - 0x4A20;
/// `r13 - 0x4A1F` (`0x8042AC61`): guard byte of the second registration.
const GUARD_8042AC61_C235: usize = SDA1_BASE - 0x4A1F;

/// Registers two SDA-resident state banks behind one-shot guards.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80233110`.
/// - Extent: `0x80233110..0x80233194` (132 bytes); `stwu r1,-0x10(r1)` at the
///   entry and `bclr 20,0` at `0x80233190`.
/// - Table: slot 149 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC34`).
/// - observed: while `0x8042AC60 == 0`: `0x80229360(0x8042AC54)`, then
///   `0x802DC780(0x8042AC54, 0x80239370, 0x803DA108)`, then the guard is set.
/// - observed: while `0x8042AC61 == 0`: `0x80229810(0x8042AC5C)`, then
///   `0x802DC780(0x8042AC5C, 0x80239820, 0x803DA114)`, then the guard is set.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds two state banks in SDA1 and pushes their shutdown nodes.
///
/// # Safety
/// The descriptors, the nodes and the guard bytes must be writable.
///
/// # Confidence
/// high - two identical guarded blocks with literal arguments.
pub unsafe fn fn_80233110() {
    if platform::read_u8(GUARD_8042AC60_C235) == 0 {
        platform::call1(FN_80229360_C235, DESC_8042AC54_C235);
        platform::call3(
            FN_802DC780,
            DESC_8042AC54_C235,
            FN_80239370_C235,
            NODE_803DA108_C235,
        );
        platform::write_u8(GUARD_8042AC60_C235, 1);
    }
    if platform::read_u8(GUARD_8042AC61_C235) == 0 {
        platform::call1(FN_80229810_C235, DESC_8042AC5C_C235);
        platform::call3(
            FN_802DC780,
            DESC_8042AC5C_C235,
            FN_80239820_C235,
            NODE_803DA114_C235,
        );
        platform::write_u8(GUARD_8042AC61_C235, 1);
    }
}

/// `0x803DEF40`: base of the block filled by [`fn_8024da70`] (`r31`).
const BASE_803DEF40_C235: usize = 0x803D_EF40;
/// `r13 - 0x49D0` (`0x8042ACB0`): first state word cleared by [`fn_8024da70`].
const SDA_49D0_C235: usize = SDA1_BASE - 0x49D0;
/// `r13 - 0x49CC` (`0x8042ACB4`): second state word cleared.
const SDA_49CC_C235: usize = SDA1_BASE - 0x49CC;
/// `r13 - 0x49C8` (`0x8042ACB8`): third state word cleared.
const SDA_49C8_C235: usize = SDA1_BASE - 0x49C8;
/// `r13 - 0x49C4` (`0x8042ACBC`): fourth state word cleared.
const SDA_49C4_C235: usize = SDA1_BASE - 0x49C4;
/// `r13 - 0x49B8` (`0x8042ACC8`): pointer/flag pair cleared at the end.
const SDA_49B8_C235: usize = SDA1_BASE - 0x49B8;
/// `0x800046B4`: `memset`, used for the `0x6C`-byte body of the first record.
const FN_800046B4_C235: usize = 0x8000_46B4;
/// `0x8025F2B0`: stores its second argument through the stack slot.
const FN_8025F2B0_C235: usize = 0x8025_F2B0;
/// `0x8025F2C0`: initialises the element just published through the slot.
const FN_8025F2C0_C235: usize = 0x8025_F2C0;
/// `0x8025F6D0`: array constructor for the block at `0x803DFF88`.
const FN_8025F6D0_C235: usize = 0x8025_F6D0;
/// `0x8024B610`: constructor for `0x803E29B4`.
const FN_8024B610_C235: usize = 0x8024_B610;
/// `0x801C04D0`: identity-matrix writer (already named in `render::gx_band`).
const FN_801C04D0_C235: usize = 0x801C_04D0;
/// `0x8024B010`: vtable word repeated 252 times by the record loop.
const VTABLE_8024B010_C235: u32 = 0x8024_B010;
/// `0x8024A6E0`: callback pair 1, stored twice in the header.
const CB_8024A6E0_C235: u32 = 0x8024_A6E0;
/// `0x8024AAC0`: callback stored at header word `+0x10`.
const CB_8024AAC0_C235: u32 = 0x8024_AAC0;
/// `0x8024ABD0`: callback stored at header word `+0x20`.
const CB_8024ABD0_C235: u32 = 0x8024_ABD0;
/// `0x8024B0D0`: fourth argument of the `0x802DC9A0` hash init.
const HASH_8024B0D0_C235: u32 = 0x8024_B0D0;
/// `0x8024B090`: fifth argument of the `0x802DC9A0` hash init.
const HASH_8024B090_C235: u32 = 0x8024_B090;
/// `0x8034E708`: vtable word of the object at `0x803E4428`.
const VTABLE_8034E708_C235: u32 = 0x8034_E708;

/// Ten-word header written at `0x803DF180` (before the 252-record table).
const W235C_HEADER_8024DA70_C235: [u32; 10] = [
    VTABLE_8024B010_C235,
    0,
    CB_8024A6E0_C235,
    1,
    CB_8024AAC0_C235,
    2,
    CB_8024A6E0_C235,
    1,
    CB_8024ABD0_C235,
    1,
];

/// Bulk static initialiser for the block headed at `0x803DEF40`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8024DA70`.
/// - Extent: `0x8024DA70..0x8024DCE0` (624 bytes); `stwu r1,-0x20(r1)` at the
///   entry and `bclr 20,0` at `0x8024DCDC`. The saved `r31` is the base
///   `0x803DEF40` (from `lis r31,0x803E`; `addi r31,r31,-0x10C0`).
/// - Table: slot 150 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC38`).
/// - observed, in order:
///   1. `0x8042ACB0 = 1`; bytes `0x803DEF40..0x803DEF43 = 1,0,1,0`;
///      words `+4 = 2`, `+8 = 0`; `memset(0x803DEF4C, 0, 0x6C)`.
///   2. byte `0x803DF0C0 = 0`; halfword `0x803DF140 = 0`;
///      words `0x803DF144..0x803DF14F = 0`, `0x803DF150..0x803DF16F = 0`,
///      `0x803DF170..0x803DF17B = 0`; `0x8042ACB4 = 0`, `0x8042ACB8 = 0`,
///      `0x8042ACBC = 0`.
///   3. a ten-word header at `0x803DF180`, then 252 eight-byte records from
///      `0x803DF1A8` to `0x803DF984`, each `{0x8024B010, 0}` (a `bdnz` loop).
///   4. `0x803DF980 = 0`, `0x803DF984 = -1`; then 32 elements of stride `0x30`
///      starting at `0x803DF988`, each published through the `r1+8` stack slot
///      by `0x8025F2B0` and initialised by `0x8025F2C0`.
///   5. `0x8025F6D0(0x803DFF88, 0x803DFFB4, 0x803E21B4, 0x80, 0x803E23B4,
///      0x80)`; `0x802DC9A0(0x803DFFB4, 0x8024B0D0, 0x8024B090, 0x44, 0x80)`;
///      `0x803DFF98 = -1`, `0x803DFF9C..0x803DFFAB = 0`, and the eight bytes
///      `0x803DFFAC..0x803DFFB3 = 0xFF`; then `0x8024B610(0x803E29B4)`.
///   6. `0x8042ACC8 = 0`, `0x8042ACCC = 0`; `0x803E4428 = 0x8034E708`; three
///      `0x801C04D0` calls on `0x803E4430`, `0x803E4460`, `0x803E4490`; then
///      `0x803E442C..0x803E442F = 0`.
/// - observed: the loop at `0x8024DC88` only strides `r4` by `0x30` until
///   `r28+0x98` and stores nothing; it is a dead counting loop and is omitted.
/// - observed: the `li`/`addi` pairs resolve `0x8024B010`, `0x8024A6E0`,
///   `0x8024AAC0`, `0x8024ABD0`, `0x8024B0D0`, `0x8024B090` and the vtable
///   `0x8034E708` one page below their `lis` immediates.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills about 3 KiB of `.bss`/`.data`, builds 252 fixed records, 32 linked
/// elements and a vtable-headed object, and calls the six constructors that
/// initialise them.
///
/// # Safety
/// All written ranges must be writable, and the called constructors must accept
/// the documented arguments.
///
/// # Confidence
/// medium-high - every offset, immediate and loop count is literal; only the
/// meaning of the five called constructors is inferred from their shape.
pub unsafe fn fn_8024da70() {
    let base = BASE_803DEF40_C235;
    platform::write_u8(SDA_49D0_C235, 1);
    platform::write_u8(base, 1);
    platform::write_u8(base + 1, 0);
    platform::write_u8(base + 2, 1);
    platform::write_u8(base + 3, 0);
    platform::write_u32(base + 4, 2);
    platform::write_u32(base + 8, 0);
    let _ = platform::call3(FN_800046B4_C235, (base + 0x0C) as u32, 0, 0x6C);

    platform::write_u8(base + 0x180, 0);
    platform::write_u16(base + 0x200, 0);
    platform::write_u32(base + 0x204, 0);
    platform::write_u32(base + 0x208, 0);
    platform::write_u32(base + 0x20C, 0);
    platform::write_u8(SDA_49CC_C235, 0);
    platform::write_u32(SDA_49C8_C235, 0);
    let mut offset = 0x210usize;
    while offset < 0x230 {
        platform::write_u32(base + offset, 0);
        offset += 4;
    }
    platform::write_u32(SDA_49C4_C235, 0);
    platform::write_u32(base + 0x230, 0);
    platform::write_u32(base + 0x234, 0);
    platform::write_u32(base + 0x238, 0);

    let mut word = 0usize;
    while word < W235C_HEADER_8024DA70_C235.len() {
        platform::write_u32(base + 0x240 + word * 4, W235C_HEADER_8024DA70_C235[word]);
        word += 1;
    }
    let mut record = 0usize;
    while record < 252 {
        platform::write_u32(base + 0x268 + record * 8, VTABLE_8024B010_C235);
        platform::write_u32(base + 0x26C + record * 8, 0);
        record += 1;
    }

    platform::write_u32(base + 0xA40, 0);
    platform::write_u32(base + 0xA44, 0xFFFF_FFFF);
    let mut scratch = [0u32; 2];
    let mut element = 0usize;
    while element < 32 {
        platform::call2(
            FN_8025F2B0_C235,
            scratch.as_mut_ptr() as u32,
            (base + 0xA48 + element * 0x30) as u32,
        );
        platform::call1(FN_8025F2C0_C235, scratch.as_mut_ptr() as u32);
        element += 1;
    }

    let block = base + 0x1048;
    let _ = platform::call6(
        FN_8025F6D0_C235,
        block as u32,
        (block + 0x2C) as u32,
        (block + 0x222C) as u32,
        0x80,
        (block + 0x242C) as u32,
        0x80,
    );
    let _ = platform::call5(
        FN_802DC9A0,
        (block + 0x2C) as u32,
        HASH_8024B0D0_C235,
        HASH_8024B090_C235,
        0x44,
        0x80,
    );
    platform::write_u32(block + 0x10, 0xFFFF_FFFF);
    let mut word = 0usize;
    while word < 4 {
        platform::write_u32(block + 0x14 + word * 4, 0);
        word += 1;
    }
    let mut byte = 0usize;
    while byte < 8 {
        platform::write_u8(block + 0x24 + byte, 0xFF);
        byte += 1;
    }
    platform::call1(FN_8024B610_C235, (base + 0x3A74) as u32);

    platform::write_u32(SDA_49B8_C235, 0);
    platform::write_u8(SDA_49B8_C235 + 4, 0);
    let entry = base + 0x54E8;
    platform::write_u32(entry, VTABLE_8034E708_C235);
    platform::call1(FN_801C04D0_C235, (entry + 8) as u32);
    platform::call1(FN_801C04D0_C235, (entry + 0x38) as u32);
    platform::call1(FN_801C04D0_C235, (entry + 0x68) as u32);
    platform::write_u8(entry + 4, 0);
    platform::write_u8(entry + 5, 0);
    platform::write_u8(entry + 6, 0);
    platform::write_u8(entry + 7, 0);
}

/// `r13 - 0x49A8` (`0x8042ACD8`): first of five words written by
/// [`fn_80261d20`].
const SDA_49A8_C235: usize = SDA1_BASE - 0x49A8;

/// Installs five consecutive SDA pointers.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80261D20`.
/// - Extent: `0x80261D20..0x80261D60` (64 bytes, leaf).
/// - Table: slot 151 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC3C`).
/// - observed: five `lis rX,0x8035`/`addi rX,rX,-0x15A8..-0x15E8` pairs store
///   `0x8034EA58`, `0x8034EA48`, `0x8034EA38`, `0x8034EA28`, `0x8034EA18`
///   into `r13-0x49A8..r13-0x4998` in that order.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes five SDA words.
///
/// # Safety
/// The SDA words must be writable.
///
/// # Confidence
/// high - five literal stores.
pub unsafe fn fn_80261d20() {
    let values = [
        0x8034_EA58u32,
        0x8034_EA48,
        0x8034_EA38,
        0x8034_EA28,
        0x8034_EA18,
    ];
    let mut index = 0usize;
    while index < values.len() {
        platform::write_u32(SDA_49A8_C235 + index * 4, values[index]);
        index += 1;
    }
}

/// `r13 - 0x4980` (`0x8042AD00`): head word of the second list.
const SDA_4980_C235: usize = SDA1_BASE - 0x4980;

/// Clears the head word of the second list.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802677F0`.
/// - Extent: `0x802677F0..0x802677FC` (12 bytes, leaf).
/// - Table: slot 152 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC40`).
/// - observed: `li r0,0`; `stw r0,-0x4980(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802677f0() {
    platform::write_u32(SDA_4980_C235, 0);
}

/// `r13 - 0x4950` (`0x8042AD30`): second word of the second list.
const SDA_4950_C235: usize = SDA1_BASE - 0x4950;

/// Points the second word of the second list at its head.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80272C30`.
/// - Extent: `0x80272C30..0x80272C3C` (12 bytes, leaf).
/// - Table: slot 153 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC44`).
/// - observed: `addi r0,r13,-0x4980`; `stw r0,-0x4950(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_80272c30() {
    platform::write_u32(SDA_4950_C235, SDA_4980_C235 as u32);
}

/// `0x803FCC88`: object initialised by the tail call of [`fn_802753b0`].
const OBJ_803FCC88_C235: u32 = 0x803F_CC88;
/// `0x80274CB0`: inline constructor reached by the tail call.
const FN_80274CB0_C235: usize = 0x8027_4CB0;

/// Initialises the object at `0x803FCC88`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802753B0`.
/// - Extent: `0x802753B0..0x802753BC` (12 bytes; the body is
///   `lis r3,0x8040`; `addi r3,r3,-0x3378`; `b 0x80274CB0`, so there is no
///   `blr`). The displacement resolves to `0x803FCC88`, one page below the
///   `lis` immediate.
/// - Table: slot 154 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC48`).
/// - observed: `0x80274CB0` stores vtable `0x8034F060` at `+0` and four floats
///   at `+4..+0x10`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing; the tail call's result is the body's result and is unused.
///
/// # Side effects
/// Writes the vtable word and four floats of the object.
///
/// # Safety
/// The object must be writable.
///
/// # Confidence
/// high - the body is a constant load and a tail call.
pub unsafe fn fn_802753b0() {
    platform::call1(FN_80274CB0_C235, OBJ_803FCC88_C235);
}

// ===========================================================================
// Wave 236 lane B: `.ctors` slots 155-168 (`0x802EDC4C..0x802EDC80`)
// ===========================================================================
//
// Table base `0x802ED9E0`; slot `n`'s word is at `0x802ED9E0 + 4n`. Two of the
// fourteen targets already had Rust bodies in the tree when this section was
// written, so they are referenced rather than duplicated:
//
// | slot | word | target | extent | existing owner |
// |-----:|------|--------|-------:|----------------|
// | 158 | `0x802EDC58` | `0x80287000` | 300 B | `render::gx_band::fn_80287000` |
// | 160 | `0x802EDC60` | `0x80295780` |  80 B | `render::gx_band::fn_80295780` |
//
// The other twelve have no `functions.tsv` row and had no Rust body, so they
// are translated here:
//
// | slot | word | target | extent | role |
// |-----:|------|--------|-------:|------|
// | 155 | `0x802EDC4C` | `0x8027B5A0` |  72 B | resets and registers the render-thread context |
// | 156 | `0x802EDC50` | `0x8027FC40` |  12 B | SDA pointer store |
// | 157 | `0x802EDC54` | `0x802841C0` |  12 B | SDA pointer store |
// | 159 | `0x802EDC5C` | `0x80294B10` |   4 B | empty leaf (`blr` only) |
// | 161 | `0x802EDC64` | `0x80297290` |  88 B | seven SDA pointer stores |
// | 162 | `0x802EDC68` | `0x8029D230` | 136 B | two 3x4 identity matrices |
// | 163 | `0x802EDC6C` | `0x802AC540` |  12 B | SDA word clear |
// | 164 | `0x802EDC70` | `0x802ADE90` |  12 B | SDA pointer store |
// | 165 | `0x802EDC74` | `0x802AF6E0` |  12 B | SDA pointer store |
// | 166 | `0x802EDC78` | `0x802B1C80` |  12 B | SDA pointer store |
// | 167 | `0x802EDC7C` | `0x802B1D50` |  12 B | SDA pointer store |
// | 168 | `0x802EDC80` | `0x802BB000` |  44 B | derives a timer scale from the bus clock |
//
// Extents are prologue-to-`blr` inclusive, measured by recursive descent from
// the raw words. Every address below is resolved from the `lis`/`addi` pair as
// written; several therefore land one page below the `lis` immediate (for
// example slot 161's `0x8034F668`, not `0x8035F668`), and the literals follow
// the arithmetic, not the high half.
//
// Slots 156/157 publish one `r13` address; slot 161 publishes seven consecutive
// table pointers; slots 163-167 chain the same pointer/clear pattern as the
// wave-235 SDA list; slot 168 is the only body with arithmetic.
// ---------------------------------------------------------------------------

/// `0x803FD2D0`: render-thread context reset by [`fn_8027b5a0`].
///
/// The same word is the `thread` argument of `render::gx_band::fn_80280530`
/// (the render-thread creator), so the object is a thread context rather than a
/// DMA descriptor.
const THREAD_803FD2D0_B236: usize = 0x803F_D2D0;
/// `0x803FD2C0`: registry node written by `0x802DC780` (`context - 0x10`).
const NODE_803FD2C0_B236: u32 = 0x803F_D2C0;
/// `0x80280490`: `render::gx_band::fn_80280490`, clears the three control fields.
const FN_80280490_B236: usize = 0x8028_0490;
/// `0x802804B0`: `render::gx_band::fn_802804b0`, stops/detaches the thread.
const FN_802804B0_B236: u32 = 0x8028_04B0;

/// Resets the render-thread context and pushes its shutdown node.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8027B5A0`.
/// - Extent: `0x8027B5A0..0x8027B5E8` (72 bytes); `stwu r1,-0x10(r1)` at the
///   entry and `bclr 20,0` at `0x8027B5E4`.
/// - Table: slot 155 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC4C`).
/// - observed: `lis r31,0x8040`; `addi r3,r31,-0x2D30` (`0x803FD2D0`);
///   `bl 0x80280490`; then `0x802DC780(0x803FD2D0, 0x802804B0, 0x803FD2C0)`.
///   The last two arguments are the thread destructor and the node at
///   `context - 0x10`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears `+0x318`, `+0x31C` and `+0x31D` of the thread context, then registers
/// the destructor on the global shutdown list.
///
/// # Safety
/// The thread context and its registry node must be writable.
///
/// # Confidence
/// high - both callees and all four literals are visible in the body.
pub unsafe fn fn_8027b5a0() {
    platform::call1(FN_80280490_B236, THREAD_803FD2D0_B236 as u32);
    platform::call3(
        FN_802DC780,
        THREAD_803FD2D0_B236 as u32,
        FN_802804B0_B236,
        NODE_803FD2C0_B236,
    );
}

/// `r13 - 0x4980` (`0x8042AD00`): pointer source of slots 156/157.
const SDA_4980_B236: usize = SDA1_BASE - 0x4980;

/// `r13 - 0x4928` (`0x8042AD58`): first destination of the `0x8042AD00` pointer.
const SDA_4928_B236: usize = SDA1_BASE - 0x4928;

/// Publishes the `0x8042AD00` address in the SDA word at `r13 - 0x4928`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8027FC40`.
/// - Extent: `0x8027FC40..0x8027FC4C` (12 bytes, leaf).
/// - Table: slot 156 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC50`).
/// - observed: `addi r0,r13,-0x4980`; `stw r0,-0x4928(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_8027fc40() {
    platform::write_u32(SDA_4928_B236, SDA_4980_B236 as u32);
}

/// `r13 - 0x4910` (`0x8042AD70`): second destination of the `0x8042AD00` pointer.
const SDA_4910_B236: usize = SDA1_BASE - 0x4910;

/// Publishes the `0x8042AD00` address in the SDA word at `r13 - 0x4910`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802841C0`.
/// - Extent: `0x802841C0..0x802841CC` (12 bytes, leaf).
/// - Table: slot 157 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC54`).
/// - observed: `addi r0,r13,-0x4980`; `stw r0,-0x4910(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802841c0() {
    platform::write_u32(SDA_4910_B236, SDA_4980_B236 as u32);
}

/// Empty `.ctors` body: a single `blr`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80294B10`.
/// - Extent: `0x80294B10..0x80294B14` (4 bytes).
/// - Table: slot 159 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC5C`).
/// - observed: `4e800020` = `bclr 20,0` and nothing else.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the body reads and writes nothing.
///
/// # Confidence
/// high - the table word points at a bare return.
pub unsafe fn fn_80294b10() {}

/// `0x8034F668`: first table pointer published by [`fn_80297290`].
const TBL_8034F668_B236: u32 = 0x8034_F668;
/// `0x8034F658`: second table pointer.
const TBL_8034F658_B236: u32 = 0x8034_F658;
/// `0x8034F688`: third table pointer.
const TBL_8034F688_B236: u32 = 0x8034_F688;
/// `0x8034F678`: fourth table pointer.
const TBL_8034F678_B236: u32 = 0x8034_F678;
/// `0x8034F698`: fifth table pointer.
const TBL_8034F698_B236: u32 = 0x8034_F698;
/// `0x8034F6A8`: sixth table pointer.
const TBL_8034F6A8_B236: u32 = 0x8034_F6A8;
/// `0x8034F648`: seventh table pointer.
const TBL_8034F648_B236: u32 = 0x8034_F648;

/// `r13 - 0x48F0` (`0x8042AD90`): first of seven published SDA pointers.
const SDA_48F0_B236: usize = SDA1_BASE - 0x48F0;
/// `r13 - 0x48EC`: second word of the run.
const SDA_48EC_B236: usize = SDA1_BASE - 0x48EC;
/// `r13 - 0x48E8`: third word of the run.
const SDA_48E8_B236: usize = SDA1_BASE - 0x48E8;
/// `r13 - 0x48E4`: fourth word of the run.
const SDA_48E4_B236: usize = SDA1_BASE - 0x48E4;
/// `r13 - 0x48E0`: fifth word of the run.
const SDA_48E0_B236: usize = SDA1_BASE - 0x48E0;
/// `r13 - 0x48DC`: sixth word of the run.
const SDA_48DC_B236: usize = SDA1_BASE - 0x48DC;
/// `r13 - 0x48D8`: seventh word of the run.
const SDA_48D8_B236: usize = SDA1_BASE - 0x48D8;

/// Publishes seven adjacent static-table pointers in the SDA run at
/// `r13 - 0x48F0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80297290`.
/// - Extent: `0x80297290..0x802972E8` (88 bytes, leaf).
/// - Table: slot 161 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC64`).
/// - observed: seven `lis rX,0x8035` / `addi rX,rX,-imm` pairs loading
///   `0x8034F668`, `0x8034F658`, `0x8034F688`, `0x8034F678`, `0x8034F698`,
///   `0x8034F6A8` and `0x8034F648`, then seven `stw` to the SDA run and
///   `bclr 20,0`. The negative displacements cross the page boundary, so every
///   pointer is `0x8034F6xx`, not `0x8035F6xx`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes seven SDA words.
///
/// # Safety
/// The SDA run must be writable.
///
/// # Confidence
/// high - seven literal pairs and seven stores with no branches.
pub unsafe fn fn_80297290() {
    platform::write_u32(SDA_48F0_B236, TBL_8034F668_B236);
    platform::write_u32(SDA_48EC_B236, TBL_8034F658_B236);
    platform::write_u32(SDA_48E8_B236, TBL_8034F688_B236);
    platform::write_u32(SDA_48E4_B236, TBL_8034F678_B236);
    platform::write_u32(SDA_48E0_B236, TBL_8034F698_B236);
    platform::write_u32(SDA_48DC_B236, TBL_8034F6A8_B236);
    platform::write_u32(SDA_48D8_B236, TBL_8034F648_B236);
}

/// `0x804237B0`: first 3x4 matrix filled by [`fn_8029d230`].
const MTX_804237B0_B236: usize = 0x8042_37B0;
/// `r2 - 0x4168` (`0x8042EF18`): the `1.0` constant (`0x3F800000`).
const F32_SDA2_4168_B236: usize = SDA2_BASE - 0x4168;
/// `r2 - 0x4164` (`0x8042EF1C`): the `0.0` constant (`0x00000000`).
const F32_SDA2_4164_B236: usize = SDA2_BASE - 0x4164;

/// Fills two adjacent 3x4 matrices at `0x804237B0` and `0x804237E0` with the
/// identity rotation and a zero translation.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x8029D230`.
/// - Extent: `0x8029D230..0x8029D2B8` (136 bytes, leaf).
/// - Table: slot 162 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC68`).
/// - observed: `lis r8,0x8042`; `addi r8,r8,0x37B0`; `lfs f1,r2-0x4168`
///   (`1.0`); `lfs f0,r2-0x4164` (`0.0`); then 24 `stfs` covering
///   `0x804237B0..0x8042380F`.
/// - The stores form six 4-float rows: rows `0`, `1` and `2` at
///   `+0x00/+0x10/+0x20` are `(1,0,0,0)`, `(0,1,0,0)`, `(0,0,1,0)`; rows `3`,
///   `4` and `5` at `+0x30/+0x40/+0x50` repeat the same three rows. That is
///   two 3x4 transform blocks (`Mtx34`-shaped) set to identity.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0x60` bytes of matrix data.
///
/// # Safety
/// `0x804237B0..0x80423810` must be writable.
///
/// # Confidence
/// high - straight-line stores with two literal floats; the two-matrix reading
/// is derived from the store offsets.
pub unsafe fn fn_8029d230() {
    let one = platform::read_f32(F32_SDA2_4168_B236);
    let zero = platform::read_f32(F32_SDA2_4164_B236);

    // first 3x4 matrix: 0x804237B0
    platform::write_f32(MTX_804237B0_B236, one);
    platform::write_f32(MTX_804237B0_B236 + 0x04, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x08, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x0C, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x10, one);
    platform::write_f32(MTX_804237B0_B236 + 0x14, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x18, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x1C, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x20, one);
    platform::write_f32(MTX_804237B0_B236 + 0x24, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x28, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x2C, zero);

    // second 3x4 matrix: 0x804237E0
    platform::write_f32(MTX_804237B0_B236 + 0x30, one);
    platform::write_f32(MTX_804237B0_B236 + 0x34, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x38, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x3C, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x40, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x44, one);
    platform::write_f32(MTX_804237B0_B236 + 0x48, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x4C, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x50, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x54, zero);
    platform::write_f32(MTX_804237B0_B236 + 0x58, one);
    platform::write_f32(MTX_804237B0_B236 + 0x5C, zero);
}

/// `r13 - 0x48C0` (`0x8042ADC0`): head word of the small list.
const SDA_48C0_B236: usize = SDA1_BASE - 0x48C0;

/// Clears the list head at `r13 - 0x48C0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802AC540`.
/// - Extent: `0x802AC540..0x802AC54C` (12 bytes, leaf).
/// - Table: slot 163 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC6C`).
/// - observed: `li r0,0`; `stw r0,-0x48C0(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802ac540() {
    platform::write_u32(SDA_48C0_B236, 0);
}

/// `r13 - 0x48B0` (`0x8042ADD0`): first user of the `0x8042ADC0` head.
const SDA_48B0_B236: usize = SDA1_BASE - 0x48B0;

/// Points the SDA word at `r13 - 0x48B0` at the cleared head.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802ADE90`.
/// - Extent: `0x802ADE90..0x802ADE9C` (12 bytes, leaf).
/// - Table: slot 164 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC70`).
/// - observed: `addi r0,r13,-0x48C0`; `stw r0,-0x48B0(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802ade90() {
    platform::write_u32(SDA_48B0_B236, SDA_48C0_B236 as u32);
}

/// `r13 - 0x48A8` (`0x8042ADD8`): second user of the `0x8042ADC0` head.
const SDA_48A8_B236: usize = SDA1_BASE - 0x48A8;

/// Points the SDA word at `r13 - 0x48A8` at the cleared head.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802AF6E0`.
/// - Extent: `0x802AF6E0..0x802AF6EC` (12 bytes, leaf).
/// - Table: slot 165 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC74`).
/// - observed: `addi r0,r13,-0x48C0`; `stw r0,-0x48A8(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802af6e0() {
    platform::write_u32(SDA_48A8_B236, SDA_48C0_B236 as u32);
}

/// `r13 - 0x48A0` (`0x8042ADE0`): third user of the `0x8042ADC0` head.
const SDA_48A0_B236: usize = SDA1_BASE - 0x48A0;

/// Points the SDA word at `r13 - 0x48A0` at the cleared head.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802B1C80`.
/// - Extent: `0x802B1C80..0x802B1C8C` (12 bytes, leaf).
/// - Table: slot 166 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC78`).
/// - observed: `addi r0,r13,-0x48C0`; `stw r0,-0x48A0(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802b1c80() {
    platform::write_u32(SDA_48A0_B236, SDA_48C0_B236 as u32);
}

/// `r13 - 0x4898` (`0x8042ADE8`): fourth user of the `0x8042ADC0` head.
const SDA_4898_B236: usize = SDA1_BASE - 0x4898;

/// Points the SDA word at `r13 - 0x4898` at the cleared head.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802B1D50`.
/// - Extent: `0x802B1D50..0x802B1D5C` (12 bytes, leaf).
/// - Table: slot 167 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC7C`).
/// - observed: `addi r0,r13,-0x48C0`; `stw r0,-0x4898(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802b1d50() {
    platform::write_u32(SDA_4898_B236, SDA_48C0_B236 as u32);
}

/// `0x800000F8`: OS bus clock in Hz, the input of [`fn_802bb000`].
const OS_BUS_CLOCK_B236: usize = 0x8000_00F8;
/// `r13 - 0x4830` (`0x8042AE50`): the derived timer-scale word.
const SDA_4830_B236: usize = SDA1_BASE - 0x4830;

/// Derives a 300-microsecond time-base interval from the bus clock and stores
/// it at `r13 - 0x4830`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802BB000`.
/// - Extent: `0x802BB000..0x802BB02C` (44 bytes, leaf).
/// - Table: slot 168 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC80`).
/// - observed: `lis r4,0x8000`; `lwz r0,0xF8(r4)` (bus clock);
///   `lis r3,0x431C`; `addi r3,r3,-0x217D` (`0x431BDE83`);
///   `rlwinm r0,r0,30,2,31` (clock `>> 2`, the time-base rate);
///   `mulhwu r0,r3,r0`; `rlwinm r0,r0,17,15,31`;
///   `mulli r0,r0,300`; `rlwinm r0,r0,29,3,31`;
///   `stw r0,-0x4830(r13)`; `bclr 20,0`.
/// - The `0x431BDE83` reciprocal multiply plus the two masks is the compiler's
///   fixed-point form of `timebase / 10000 * 3`: with a 243 MHz bus
///   (`timebase = 60,750,000`) the intermediate is `486` and the stored value
///   is `18225`, which is exactly `60,750,000 * 0.0003` - the number of
///   time-base ticks in 300 microseconds.
///
/// # Arguments
/// None. The input is the low-memory bus-clock word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Reads `0x800000F8` and writes one SDA word.
///
/// # Safety
/// `0x800000F8` must be readable and the SDA word writable.
///
/// # Confidence
/// medium - the instruction sequence is exact and the arithmetic is verified
/// numerically, but the consumer of the stored scale is not identified.
pub unsafe fn fn_802bb000() {
    let ticks = platform::read_u32(OS_BUS_CLOCK_B236) >> 2;
    // observed: mulhwu (top 32 bits) of 0x431BDE83, then `rlwinm ..,17,15,31`
    // keeps bits 15..31, i.e. the combined `>> 47` plus a 17-bit mask.
    let scaled = (((ticks as u64 * 0x431B_DE83u64) >> 47) as u32) & 0x1_FFFF;
    // observed: `mulli r0,r0,300`; `rlwinm r0,r0,29,3,31`.
    platform::write_u32(SDA_4830_B236, (scaled.wrapping_mul(300)) >> 3);
}

// ===========================================================================
// Wave 236 lane C: `.ctors` slots 169-180 (`0x802EDC84..0x802EDCB0`) -
// the last twelve bodies of the 181-entry table
// ===========================================================================
//
// Table base `0x802ED9E0`; slot `n`'s word is at `0x802ED9E0 + 4n`. The table
// holds **181** code pointers, indices 0..180, and is terminated by three zero
// words at `0x802EDCB4`, `0x802EDCB8`, `0x802EDCBC`. The wave brief's
// "slots 169-181" is the 181st entry counted from one; the last table index is
// 180, so this section covers the twelve indices 169..180 (the wave-229 lane B
// note that the terminator starts at `0x802EDCB0` is off by one word).
//
// None of the twelve targets has a `functions.tsv` row and none had a Rust body
// in the tree when this section was written, so all twelve are translated here.
//
// | slot | word | target | extent | role |
// |-----:|------|--------|-------:|------|
// | 169 | `0x802EDC84` | `0x802BE3D0` |  72 B | base constructor + vtable swap + flag |
// | 170 | `0x802EDC88` | `0x802BE5E0` |  88 B | builds a Y-mirror 3x4 matrix at `0x80424558` |
// | 171 | `0x802EDC8C` | `0x802BE740` | 120 B | default rows at `0x80424588` + three SDA pairs |
// | 172 | `0x802EDC90` | `0x802BF050` |  12 B | one SDA word = `-1` |
// | 173 | `0x802EDC94` | `0x802C3200` |   4 B | empty body (bare `blr`) |
// | 174 | `0x802EDC98` | `0x802C7340` |  36 B | two SDA float pairs |
// | 175 | `0x802EDC9C` | `0x802C7B10` |   4 B | empty body (bare `blr`) |
// | 176 | `0x802EDCA0` | `0x802D1CE0` | 132 B | six words at `0x803504E0` + two SDA pairs |
// | 177 | `0x802EDCA4` | `0x802D1D90` |  12 B | tail call with the object `0x80424798` |
// | 178 | `0x802EDCA8` | `0x802D5FB0` | 340 B | eight-record static table at `0x80424AF0` |
// | 179 | `0x802EDCAC` | `0x802D7000` |  12 B | tail call with the object `0x80424CD8` |
// | 180 | `0x802EDCB0` | `0x802D88C0` |   8 B | tail call with the SDA object `0x8042AFC8` |
//
// Extents are prologue-to-`blr` (or prologue-to-tail-`b`) inclusive, measured by
// recursive descent from the raw words. Every address below is resolved from
// the instruction as written: the `lis`/`addi` pair for absolute objects and
// `SDA1_BASE`/`SDA2_BASE` for the small-data references, with the signed 16-bit
// displacement taken as negative (for example slot 170's `r2-0xC110` is
// `0x8042F190`, not `0x80434000`).
//
// Slots 173 and 175 are literal one-instruction `blr` bodies: they exist in the
// table but do nothing. Both are transcribed as empty functions so the table
// stays complete and their slot numbers remain visible.
// ---------------------------------------------------------------------------

/// `0x802BC7A0`: base constructor of the object at `0x804238D8`; it installs
/// vtable `0x80350030` and clears the controller header.
const FN_802BC7A0_C236: usize = 0x802B_C7A0;
/// `0x804238D8`: the object slot 169 constructs.
const OBJ_804238D8_C236: usize = 0x8042_38D8;
/// `0x8034FFA0`: derived vtable written over the base one (`{0, 0}` header at
/// `0x8034FFA0`, first method at `0x8034FFA8`).
const VTABLE_8034FFA0_C236: u32 = 0x8034_FFA0;
/// `0x80424504`: one flag byte `0x802BE3D0` writes after the vtable swap.
const FLAG_80424504_C236: usize = 0x8042_4504;

/// Constructs the object at `0x804238D8` and installs its derived vtable.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802BE3D0`.
/// - Extent: `0x802BE3D0..0x802BE418` (72 bytes, leaf).
/// - Table: slot 169 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC84`).
/// - observed: `lis r31,0x8042`; `addi r3,r31,0x38D8`; `bl 0x802BC7A0`;
///   `lis r4,0x8035`; `addi r4,r4,-0x60` (`0x8034FFA0`);
///   `stw r4,0x38D8(r31)`; `li r0,0xFE`; `stb r0,0xC2C(r3)`.
///
/// # Arguments
/// None. All addresses are link-time constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs the base constructor of `0x804238D8` (which writes vtable `0x80350030`
/// and the controller header), replaces that vtable with `0x8034FFA0`, and
/// writes `0xFE` at `0x80424504`.
///
/// # Safety
/// The object and the flag byte must be writable; `0x802BC7A0` must be the
/// constructor described in [`crate::resource::archive::fn_802bc7a0`].
///
/// # Confidence
/// high - the whole body is an address setup, one call and two literal stores.
pub unsafe fn fn_802be3d0() {
    platform::call1(FN_802BC7A0_C236, OBJ_804238D8_C236 as u32);
    platform::write_u32(OBJ_804238D8_C236, VTABLE_8034FFA0_C236);
    platform::write_u8(FLAG_80424504_C236, 0xFE);
}

/// `0x80424558`: 12-float (3x4) matrix written by [`fn_802be5e0`].
const MATRIX_80424558_C236: usize = 0x8042_4558;

/// Writes a Y-mirror 3x4 matrix at `0x80424558`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802BE5E0`.
/// - Extent: `0x802BE5E0..0x802BE638` (88 bytes, leaf).
/// - Table: slot 170 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC88`).
/// - observed: `f2 = r2-0xC110` (`0.0`), `f1 = r2-0xC114` (`1.0`); then
///   `f3..f5 = -f2`, `f6 = -f1`, `f7..f8 = -f2`; four floats are spilled to
///   `r1+8..r1+20` and `bl 0x802BE580` is called with `r3 = 0x80424558`.
/// - observed: `0x802BE580` is a straight twelve-float copy to `r3`
///   (`work/ghidra/main-export/802be580_FUN_802be580.c`), so the matrix is
///   `{1, 0, -0, -0, -0, -1, -0, -0, 0, 0, 1, 0}`.
///
/// # Arguments
/// None. All values are constants and the destination is link-time fixed.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes twelve floats at `0x80424558..0x80424588`.
///
/// # Safety
/// The matrix must be writable.
///
/// # Confidence
/// high - the constants, their negations and the copy order are literal; the
/// callee's body is a flat twelve-store copy.
pub unsafe fn fn_802be5e0() {
    let zero = platform::read_f32(SDA2_BASE - 0x3EF0);
    let one = platform::read_f32(SDA2_BASE - 0x3EEC);
    // The binary calls 0x802BE580 with eight float registers and four stack
    // floats; that ABI cannot be expressed by `platform::callN`, so the
    // callee's twelve stores are inlined in the same order.
    let matrix = [
        one,
        zero,
        -zero,
        -zero,
        -zero,
        -one,
        -zero,
        -zero,
        zero,
        zero,
        one,
        zero,
    ];
    let mut index = 0usize;
    while index < matrix.len() {
        platform::write_f32(MATRIX_80424558_C236 + index * 4, matrix[index]);
        index += 1;
    }
}

/// `0x80424588`: three four-float rows written by [`fn_802be740`].
const ROWS_80424588_C236: usize = 0x8042_4588;
/// `r13 - 0x47F8` (`0x8042AE88`): first 2-float SDA pair.
const SDA_47F8_C236: usize = SDA1_BASE - 0x47F8;
/// `r13 - 0x47F0` (`0x8042AE90`): second 2-float SDA pair.
const SDA_47F0_C236: usize = SDA1_BASE - 0x47F0;
/// `r13 - 0x47E8` (`0x8042AE98`): third 2-float SDA pair.
const SDA_47E8_C236: usize = SDA1_BASE - 0x47E8;

/// Writes three default rows and three unit pairs.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802BE740`.
/// - Extent: `0x802BE740..0x802BE7B8` (120 bytes, leaf).
/// - Table: slot 171 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC8C`).
/// - observed: loads `f1 = r2-0xC118` (`0.0`) and `f0 = r2-0xC11C` (`1.0`);
///   writes `{0.0, 0.0}` to `0x8042AE88`, `{1.0, 0.0}` to `0x8042AE90` and
///   `{0.0, 1.0}` to `0x8042AE98`.
/// - observed: writes twelve floats at `0x80424588..0x804245B8` in the order
///   `{0, 0, 0, 1}`, `{0, 0, 0, 1}`, `{0, 0, 0, 1}` (each row is a
///   `stfs`/`stfs`/`stfs`/`stfs` run with `1.0` last).
///
/// # Arguments
/// None. All values are constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes six SDA floats and twelve `.data` floats.
///
/// # Safety
/// Both ranges must be writable.
///
/// # Confidence
/// high - every store and both constants are literal.
pub unsafe fn fn_802be740() {
    let zero = platform::read_f32(SDA2_BASE - 0x3EE8);
    let one = platform::read_f32(SDA2_BASE - 0x3EE4);
    platform::write_f32(SDA_47F8_C236, zero);
    platform::write_f32(SDA_47F8_C236 + 4, zero);
    platform::write_f32(SDA_47F0_C236, one);
    platform::write_f32(SDA_47F0_C236 + 4, zero);
    platform::write_f32(SDA_47E8_C236, zero);
    platform::write_f32(SDA_47E8_C236 + 4, one);

    let rows = [zero, zero, zero, one, zero, zero, zero, one, zero, zero, zero, one];
    let mut index = 0usize;
    while index < rows.len() {
        platform::write_f32(ROWS_80424588_C236 + index * 4, rows[index]);
        index += 1;
    }
}

/// `r13 - 0x47E0` (`0x8042AEA0`): word set to `-1` by [`fn_802bf050`].
const SDA_47E0_C236: usize = SDA1_BASE - 0x47E0;

/// Stores `-1` in one SDA word.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802BF050`.
/// - Extent: `0x802BF050..0x802BF05C` (12 bytes, leaf).
/// - Table: slot 172 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC90`).
/// - observed: `li r0,-1`; `stw r0,-0x47E0(r13)`; `bclr 20,0`.
///
/// # Arguments
/// None. The address is a link-time SDA displacement.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one SDA word.
///
/// # Safety
/// The SDA word must be writable.
///
/// # Confidence
/// high - the whole body is two instructions and a return.
pub unsafe fn fn_802bf050() {
    platform::write_u32(SDA_47E0_C236, 0xFFFF_FFFF);
}

/// Empty constructor in slot 173.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802C3200`.
/// - Extent: `0x802C3200..0x802C3204` (4 bytes).
/// - Table: slot 173 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC94`).
/// - observed: the entry word points at a single `bclr 20,0`. The next sixteen
///   bytes are zero padding up to the next prologue at `0x802C3210`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None. The table entry is live but the body is a no-op.
///
/// # Safety
/// Nothing is dereferenced.
///
/// # Confidence
/// high - the instruction stream is unambiguous.
pub unsafe fn fn_802c3200() {}

/// `r13 - 0x47C8` (`0x8042AEB8`): first pair written by [`fn_802c7340`].
const SDA_47C8_C236: usize = SDA1_BASE - 0x47C8;
/// `r13 - 0x47C0` (`0x8042AEC0`): second pair written by [`fn_802c7340`].
const SDA_47C0_C236: usize = SDA1_BASE - 0x47C0;

/// Writes two constant float pairs.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802C7340`.
/// - Extent: `0x802C7340..0x802C7364` (36 bytes, leaf).
/// - Table: slot 174 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC98`).
/// - observed: `f1 = r2-0x3CC8` (`1.0`), `f0 = r2-0x3CBC` (`0.0`);
///   `{1.0, 1.0}` goes to `0x8042AEB8` and `{0.0, 0.0}` to `0x8042AEC0`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four SDA floats.
///
/// # Safety
/// Both pairs must be writable.
///
/// # Confidence
/// high - four literal stores with two SDA2 loads.
pub unsafe fn fn_802c7340() {
    let one = platform::read_f32(SDA2_BASE - 0x3CC8);
    let zero = platform::read_f32(SDA2_BASE - 0x3CBC);
    platform::write_f32(SDA_47C8_C236, one);
    platform::write_f32(SDA_47C8_C236 + 4, one);
    platform::write_f32(SDA_47C0_C236, zero);
    platform::write_f32(SDA_47C0_C236 + 4, zero);
}

/// Empty constructor in slot 175.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802C7B10`.
/// - Extent: `0x802C7B10..0x802C7B14` (4 bytes).
/// - Table: slot 175 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDC9C`).
/// - observed: the entry word points at a single `bclr 20,0`; the next twelve
///   bytes are zero padding before the helper at `0x802C7B20`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None. The table entry is live but the body is a no-op.
///
/// # Safety
/// Nothing is dereferenced.
///
/// # Confidence
/// high - the instruction stream is unambiguous.
pub unsafe fn fn_802c7b10() {}

/// `0x803504E0`: six-word block written by [`fn_802d1ce0`].
const BLOCK_803504E0_C236: usize = 0x8035_04E0;
/// `r13 - 0x4768` (`0x8042AF18`): first pair written by [`fn_802d1ce0`].
const SDA_4768_C236: usize = SDA1_BASE - 0x4768;
/// `r13 - 0x4760` (`0x8042AF20`): second pair written by [`fn_802d1ce0`].
const SDA_4760_C236: usize = SDA1_BASE - 0x4760;

/// Writes six words and two SDA pairs from four SDA2 constants.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802D1CE0`.
/// - Extent: `0x802D1CE0..0x802D1D64` (132 bytes, leaf).
/// - Table: slot 176 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDCA0`).
/// - observed: loads `tall = r2-0x3AF0` (`1.0526316`), `one = r2-0x3B10`
///   (`1.0`), `wide = r2-0x3AEC` (`0.7692308`) and `zero = r2-0x3B0C`
///   (`0.0`); spills four floats to `r1+8..r1+20` and reloads them as words.
/// - observed: stores at `0x803504E0`: `+4 = tall`, `+8 = one`, `+16 = wide`,
///   `+20 = one`, `+28 = one`, `+32 = one`; `+0`, `+12` and `+24` are not
///   written.
/// - observed: `{one, one}` goes to `0x8042AF18` and `{zero, zero}` to
///   `0x8042AF20`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes six words in `.data` and four SDA floats.
///
/// # Safety
/// Both ranges must be writable.
///
/// # Confidence
/// high - every store and the two unaligned pairs are literal.
pub unsafe fn fn_802d1ce0() {
    let tall = platform::read_f32(SDA2_BASE - 0x3AF0);
    let one = platform::read_f32(SDA2_BASE - 0x3B10);
    let wide = platform::read_f32(SDA2_BASE - 0x3AEC);
    let zero = platform::read_f32(SDA2_BASE - 0x3B0C);
    platform::write_f32(BLOCK_803504E0_C236 + 0x04, tall);
    platform::write_f32(BLOCK_803504E0_C236 + 0x08, one);
    platform::write_f32(BLOCK_803504E0_C236 + 0x10, wide);
    platform::write_f32(BLOCK_803504E0_C236 + 0x14, one);
    platform::write_f32(BLOCK_803504E0_C236 + 0x1C, one);
    platform::write_f32(BLOCK_803504E0_C236 + 0x20, one);
    platform::write_f32(SDA_4768_C236, one);
    platform::write_f32(SDA_4768_C236 + 4, one);
    platform::write_f32(SDA_4760_C236, zero);
    platform::write_f32(SDA_4760_C236 + 4, zero);
}

/// `0x802D0CC0`: constructor tail-called by [`fn_802d1d90`] (204 bytes).
const FN_802D0CC0_C236: usize = 0x802D_0CC0;
/// `0x80424798`: object passed to [`FN_802D0CC0_C236`].
const OBJ_80424798_C236: usize = 0x8042_4798;

/// Tail-calls the constructor `0x802D0CC0` on the object `0x80424798`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for this
///   `0x802D1D90` slot; the constructor it tail-calls is `FUN_802d0cc0` at
///   `0x802D0CC0` (204 bytes).
/// - Extent: `0x802D1D90..0x802D1D9C` (12 bytes, no `blr`).
/// - Table: slot 177 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDCA4`).
/// - observed: `lis r3,0x8042`; `addi r3,r3,0x4798`; `b 0x802D0CC0`. The
///   displacement resolves to `0x80424798`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing; `0x802D0CC0` returns its argument and the caller discards it.
///
/// # Side effects
/// Runs the 204-byte constructor of `0x80424798` (seeds a sampler from the
/// `0x803504E0` table, installs vtable `0x80350504` and six copies of one
/// constant).
///
/// # Safety
/// The object must be writable and `0x802D0CC0` must be that constructor.
///
/// # Confidence
/// high - one address setup and one tail branch.
pub unsafe fn fn_802d1d90() {
    platform::call1(FN_802D0CC0_C236, OBJ_80424798_C236 as u32);
}

/// `0x80424AF0`: base of the eight-record table written by [`fn_802d5fb0`].
const TABLE_80424AF0_C236: usize = 0x8042_4AF0;

/// Host-side factoring of the repeated `stfs` runs in [`fn_802d5fb0`].
///
/// # Original
/// Host-side helper; no binary address. The original is a straight run of
/// individual `stfs` instructions.
///
/// # Safety
/// `dst` must be writable for `values.len()` floats.
///
/// # Confidence
/// high - mechanical wrapper.
unsafe fn c236_store_floats(dst: usize, values: &[f32]) {
    let mut index = 0usize;
    while index < values.len() {
        platform::write_f32(dst + index * 4, values[index]);
        index += 1;
    }
}

/// Fills the eight-record static table at `0x80424AF0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802D5FB0`.
/// - Extent: `0x802D5FB0..0x802D6104` (340 bytes, leaf).
/// - Table: slot 178 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDCA8`).
/// - observed: loads `f9 = r2-0x3A90` (`0.02`), `f8 = r2-0x3A8C` (`3.0`),
///   `f7 = r2-0x3A88` (`0.6`), `f6 = r2-0x3A84` (`0.4`),
///   `f5 = r2-0x3A80` (`0.1`), `f4 = r2-0x3A7C` (`1.0`),
///   `f3 = r2-0x3A78` (`0.0`), `f2 = r2-0x3A74` (`10.0`),
///   `f1 = r2-0x3A70` (`0.5`), `f0 = r2-0x3A6C` (`160.0`).
/// - observed: `r10 = 5`, `r0 = 0`; the base `0x80424AF0` is offset seven
///   times (`+0`, `+44`, `+88`, `+128`, `+168`, `+188`, `+208`, `+228`) and the
///   records written are, in order:
///   `+0` and `+44`: `{0.02, 3, 0.6, 0.4, 0.1, 1, 5, 0.02, 0, 0, 1}` (44 B);
///   `+88` and `+128`: `{0.02, 3, 0.6, 0.4, 1, 5, 0.02, 0, 0, 1}` (40 B);
///   `+168` and `+188`: `{10, 0.5, 1, 0, 1}` (20 B);
///   `+208` and `+228`: `{160, 0.4, 1, 160, 1}` (20 B).
///   The words `5` and `0` are stored with `stw` in the first four records.
///
/// # Arguments
/// None. All values are constants.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 248 bytes of `.data` at `0x80424AF0..0x80424BE8`.
///
/// # Safety
/// The table must be writable.
///
/// # Confidence
/// high - the loads and the eight record runs are fully linear.
pub unsafe fn fn_802d5fb0() {
    let f9 = platform::read_f32(SDA2_BASE - 0x3A90);
    let f8 = platform::read_f32(SDA2_BASE - 0x3A8C);
    let f7 = platform::read_f32(SDA2_BASE - 0x3A88);
    let f6 = platform::read_f32(SDA2_BASE - 0x3A84);
    let f5 = platform::read_f32(SDA2_BASE - 0x3A80);
    let f4 = platform::read_f32(SDA2_BASE - 0x3A7C);
    let f3 = platform::read_f32(SDA2_BASE - 0x3A78);
    let f2 = platform::read_f32(SDA2_BASE - 0x3A74);
    let f1 = platform::read_f32(SDA2_BASE - 0x3A70);
    let f0 = platform::read_f32(SDA2_BASE - 0x3A6C);
    let five: u32 = 5;
    let zero: u32 = 0;

    // Records +0 and +44.
    let head = [f9, f8, f7, f6, f5, f4];
    for base in [0usize, 44] {
        c236_store_floats(TABLE_80424AF0_C236 + base, &head);
        platform::write_u32(TABLE_80424AF0_C236 + base + 0x18, five);
        platform::write_f32(TABLE_80424AF0_C236 + base + 0x1C, f9);
        platform::write_u32(TABLE_80424AF0_C236 + base + 0x20, zero);
        platform::write_f32(TABLE_80424AF0_C236 + base + 0x24, f3);
        platform::write_f32(TABLE_80424AF0_C236 + base + 0x28, f4);
    }

    // Records +88 and +128: f4 moves to +16 and the `stw` to +20.
    let mid = [f9, f8, f7, f6, f4];
    for base in [88usize, 128] {
        c236_store_floats(TABLE_80424AF0_C236 + base, &mid);
        platform::write_u32(TABLE_80424AF0_C236 + base + 0x14, five);
        platform::write_f32(TABLE_80424AF0_C236 + base + 0x18, f9);
        platform::write_u32(TABLE_80424AF0_C236 + base + 0x1C, zero);
        platform::write_f32(TABLE_80424AF0_C236 + base + 0x20, f3);
        platform::write_f32(TABLE_80424AF0_C236 + base + 0x24, f4);
    }

    // Records +168 and +188.
    let short_a = [f2, f1, f4, f3, f4];
    for base in [168usize, 188] {
        c236_store_floats(TABLE_80424AF0_C236 + base, &short_a);
    }

    // Records +208 and +228.
    let short_b = [f0, f6, f4, f0, f4];
    for base in [208usize, 228] {
        c236_store_floats(TABLE_80424AF0_C236 + base, &short_b);
    }
}

/// `0x802D6C80`: constructor tail-called by [`fn_802d7000`].
const FN_802D6C80_C236: usize = 0x802D_6C80;
/// `0x80424CD8`: object passed to [`FN_802D6C80_C236`].
const OBJ_80424CD8_C236: usize = 0x8042_4CD8;

/// Tail-calls the constructor `0x802D6C80` on the object `0x80424CD8`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for this target `0x802D7000`
///   or for the constructor `0x802D6C80`.
/// - Extent: `0x802D7000..0x802D700C` (12 bytes, no `blr`).
/// - Table: slot 179 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDCAC`).
/// - observed: `lis r3,0x8042`; `addi r3,r3,0x4CD8`; `b 0x802D6C80`. The
///   displacement resolves to `0x80424CD8`. `0x802D6C80` saves `r3`, calls
///   `0x802D6CB0`, which tail-calls `0x80228DD0(0x80424CD8, 40)`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing; the callee returns its argument and the caller discards it.
///
/// # Side effects
/// Runs the constructor of `0x80424CD8`, which registers a 40-byte entry with
/// `0x80228DD0`.
///
/// # Safety
/// The object must be writable and `0x802D6C80` must be that constructor.
///
/// # Confidence
/// high - one address setup and one tail branch.
pub unsafe fn fn_802d7000() {
    platform::call1(FN_802D6C80_C236, OBJ_80424CD8_C236 as u32);
}

/// `0x80296750`: two-word clearing helper tail-called by [`fn_802d88c0`].
const FN_80296750_C236: usize = 0x8029_6750;
/// `r13 - 0x46B8` (`0x8042AFC8`): 8-byte SDA object cleared by the tail call.
const OBJ_8042AFC8_C236: usize = SDA1_BASE - 0x46B8;

/// Tail-calls `0x80296750` on the SDA object at `0x8042AFC8`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for this
///   `0x802D88C0` slot; the target it tail-calls is `FUN_80296750` at
///   `0x80296750` (16 bytes), which takes one pointer and zeroes its first two
///   words.
/// - Extent: `0x802D88C0..0x802D88C8` (8 bytes, no `blr`).
/// - Table: slot 180 of the `.ctors` table at `0x802ED9E0`
///   (table word `0x802EDCB0`, the last pointer of the 181-entry table).
/// - observed: `addi r3,r13,-0x46B8`; `b 0x80296750`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Zeroes the two words at `0x8042AFC8`.
///
/// # Safety
/// The SDA pair must be writable.
///
/// # Confidence
/// high - one address setup and one tail branch.
pub unsafe fn fn_802d88c0() {
    platform::call1(FN_80296750_C236, OBJ_8042AFC8_C236 as u32);
}
