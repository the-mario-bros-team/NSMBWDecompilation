//! 16-slot id/slot handle table in BSS.
//!
//! # What lives here
//!
//! The four operations on a fixed-capacity handle table: register an id,
//! look an id up, free a slot, and free an id.
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80192820` | [`fn_80192820`] | allocate the first free slot for an id |
//! | `0x80192880` | [`fn_80192880`] | id → slot lookup |
//! | `0x801928A0` | [`fn_801928a0`] | free by slot |
//! | `0x801928D0` | [`fn_801928d0`] | free by id |
//!
//! # Original address range
//!
//! `0x80192820..0x801928FC` (four functions, 192 bytes of code plus alignment
//! padding), promoted from the staging lane `src/subsystems/r80180_801c0.rs`
//! (range `0x80180000..0x801C0000`).
//!
//! # Why it exists
//!
//! The game needs a small id namespace for objects that are looked up by a
//! byte-sized handle. Two adjacent byte tables in BSS hold the mapping:
//!
//! | address | size | meaning |
//! |---|---|---|
//! | `0x80378078..0x80378087` | 16 bytes | forward table, slot → id; `0xFF` = free |
//! | `0x80378088..0x80378187` | 256 bytes | inverse table, id → slot; `0xFF` = unused |
//!
//! The tables are adjacent because the forward array ends where the inverse
//! array begins. This module only reads and writes them; the initialiser that
//! first fills both with `0xFF` is not among these four functions and is not
//! translated here.
//!
//! # Behaviour worth knowing
//!
//! * Allocation takes the first entry whose sign bit is set, so only `0xFF`
//!   (and any other byte with bit 7 set) counts as free.
//! * No function bounds-checks `id`; callers are expected to pass `0..=255`.
//! * Freeing an empty slot reads `0xFF` as a signed char (`-1`) and writes the
//!   inverse entry at `0x80378087`, the last byte of the forward table. That
//!   adjacent write is part of the original and is preserved, not fixed.
//! * The free routines use signed-char indexing for the same reason: `0xFF`
//!   becomes `-1`, so an unregistered id is detected as a negative slot.
//!
//! # Dependencies
//!
//! [`crate::platform`] - big-endian volatile byte reads and writes. No other
//! translated module is required.
//!
//! # Start here
//!
//! [`fn_80192820`] (allocate), then [`fn_80192880`] (look up) and
//! [`fn_801928d0`] (release by id). [`fn_801928a0`] is the slot-first release
//! path.
//!
//! # Callers outside this range
//!
//! `0x80192E60` (allocate + free by slot), `0x80192C70` (free by id),
//! `0x80193E40` (lookup + free by slot), `0x80193F00` (free by id). Their
//! translations belong to the staging range that owns them.
//!
//! # Glossary
//!
//! * **BSS** - zero-initialised main RAM (`0x80351680..0x8042FC3C` in this
//!   build).
//! * **DOL** - the Wii executable container (`sys/main.dol`); this code lives
//!   in its text segment.
//! * **Handle / slot** - a byte-sized id stored in one table, with its small
//!   integer slot index stored in the other.
//!
//! # Confidence summary
//!
//! All four bodies are observed exactly in the Ghidra output, so every function
//! is `high` confidence. The tables' purpose is inferred from the paired
//! read/write patterns and from the external callers, not from a symbol name.

use crate::platform::{read_u8, write_u8};

/// First handle table (`0x80378078`, 16 slots). `0xFF` means "free".
const HANDLE_SLOT_TO_ID: usize = 0x8037_8078;
/// Inverse handle table (`0x80378088`, 256 entries). `0xFF` means "unused".
const HANDLE_ID_TO_SLOT: usize = 0x8037_8088;
/// Number of entries in [`HANDLE_SLOT_TO_ID`].
const HANDLE_SLOTS: usize = 16;

/// Allocates a slot for `id` and returns its index, or -1 when all 16 slots
/// are occupied.
///
/// Scans [`HANDLE_SLOT_TO_ID`] for the first byte with the sign bit set (the
/// tables use `0xFF` as "free"). On success it stores the id in the forward
/// table and the slot index in the inverse table at `id`. No range check is
/// performed on `id`; callers are expected to pass 0..=255.
///
/// # Original
/// - Ghidra: `FUN_80192820` at `0x80192820` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80192820_FUN_80192820.c`
///
/// # Arguments
/// * `id` - caller id to register (a byte value in practice).
///
/// # Returns
/// Slot index 0..=15, or -1 if no slot is free.
///
/// # Side effects
/// Writes one byte in each table.
///
/// # Safety
/// `id` must be in `0..=255` for the inverse-table index to stay in bounds.
///
/// # Confidence
/// high - the scan and both stores are observed exactly.
pub unsafe fn fn_80192820(id: i32) -> i32 {
    let mut slot = 0usize;
    while slot < HANDLE_SLOTS {
        let forward = HANDLE_SLOT_TO_ID + slot;
        if (read_u8(forward) as i8) < 0 {
            write_u8(forward, id as u8);
            write_u8(HANDLE_ID_TO_SLOT.wrapping_add(id as usize), slot as u8);
            return slot as i32;
        }
        slot += 1;
    }
    -1
}

/// Returns the slot index registered for `id`, or a negative value when the
/// id is unregistered.
///
/// The lookup is the direct inverse table entry; `0xFF` comes back as -1 after
/// the signed-char cast.
///
/// # Original
/// - Ghidra: `FUN_80192880` at `0x80192880` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80192880_FUN_80192880.c`
///
/// # Arguments
/// * `id` - id previously passed to [`fn_80192820`].
///
/// # Returns
/// Slot 0..=15, or -1 when unused.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `id` must be in `0..=255`.
///
/// # Confidence
/// high - exact load/cast is observed.
pub unsafe fn fn_80192880(id: i32) -> i32 {
    read_u8(HANDLE_ID_TO_SLOT.wrapping_add(id as usize)) as i8 as i32
}

/// Frees the slot `slot` and clears the corresponding id mapping.
///
/// Reads the id currently stored in the forward table (as a **signed** char,
/// like the original), marks the forward entry free, then marks the inverse
/// entry free. It does not validate that the slot was occupied: freeing an
/// empty slot reads `0xFF` (-1) and clears the byte immediately *before* the
/// inverse table, which is the last byte of the forward table. That adjacent
/// write is a property of the original code and is preserved.
///
/// # Original
/// - Ghidra: `FUN_801928a0` at `0x801928a0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801928a0_FUN_801928a0.c`
///
/// # Arguments
/// * `slot` - slot index 0..=15.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes two bytes: the slot entry and the inverse entry for its id. If the
/// slot was free, the second write lands at `0x80378087` (index -1).
///
/// # Safety
/// `slot` must be in `0..=15`; an already-free slot causes the adjacent write
/// described above, matching the original.
///
/// # Confidence
/// high - exact stores and signed index are observed.
pub unsafe fn fn_801928a0(slot: i32) {
    let forward = HANDLE_SLOT_TO_ID.wrapping_add(slot as usize);
    let id = read_u8(forward) as i8;
    write_u8(forward, 0xFF);
    write_u8(HANDLE_ID_TO_SLOT.wrapping_add(id as isize as usize), 0xFF);
}

/// Frees the registration for `id`, if any, and clears its slot.
///
/// Looks up the inverse table; a negative entry means the id is not
/// registered and the function returns without writing. Otherwise it marks the
/// inverse entry free and marks the forward slot free.
///
/// # Original
/// - Ghidra: `FUN_801928d0` at `0x801928d0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801928d0_FUN_801928d0.c`
///
/// # Arguments
/// * `id` - id to unregister.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes zero or two bytes (inverse entry and forward slot).
///
/// # Safety
/// `id` must be in `0..=255`.
///
/// # Confidence
/// high - exact branch and stores are observed.
pub unsafe fn fn_801928d0(id: i32) {
    let inverse = HANDLE_ID_TO_SLOT.wrapping_add(id as usize);
    let slot = read_u8(inverse) as i8;
    if slot < 0 {
        return;
    }

    write_u8(inverse, 0xFF);
    write_u8(HANDLE_SLOT_TO_ID.wrapping_add(slot as usize), 0xFF);
}
