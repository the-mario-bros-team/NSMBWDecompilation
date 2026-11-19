//! Course-data decoding: bgdat tile-layer readers and object-id tables.
//!
//! ## What this module covers
//!
//! Readers and lookups over data that comes out of a course. Original
//! addresses are inside `0x800818B0..0x800822E0` and
//! `0x8008E310..0x8008E800` of `main.dol`:
//!
//! | address | name here | reads |
//! |---|---|---|
//! | `0x800818B0` | [`bg_layer_data`] | the `course<N>_bgdatL1.bin` handle of one stage/unit |
//! | `0x800818E0` | [`object_id_table_entry`] | `u16` entry `id` of the table at `0x802EF934` |
//! | `0x80081900` | [`object_id_in_list`] | linear search of the `0xFFFF`-terminated id list |
//! | `0x80081AB0` | [`object_id_remap`] | course/version-specific object-id remapping |
//! | `0x80081BA0` | [`is_special_object_id`] | whether the remap of id `0x2E` is `0x2F` |
//! | `0x80082120` | [`bg_layer_u16`] | `u16` entry through a loaded bgdat layer |
//! | `0x80082140` | [`bg_layer_write_u16`] | writes one `u16` entry through a bgdat layer |
//! | `0x800822E0` | [`object_tile_id`] | remap then mask an object id to 10 bits |
//! | `0x8008E310` | [`section1_entry_by_id`] | 0x18-byte entry of section 1A by `u16` id |
//! | `0x8008E350` | [`section2_entry_by_id`] | 0x18-byte entry of section 2A by `u16` id |
//! | `0x8008E390` | [`section2b_entry_by_id`] | 0x18-byte entry of section 2B by `u16` id |
//! | `0x8008E3D0` | [`section3_entry_by_id`] | 0x14-byte entry of section 3A by `u8` id |
//! | `0x8008E410` | [`section4b_entry_by_id`] | 0x18-byte entry of section 4B by `u8` id |
//! | `0x8008E4D0` | [`section4b_byte_c`] | byte `+0xC` of a section-4B entry |
//! | `0x8008E500` | [`section4b_byte_d`] | byte `+0xD` of a section-4B entry |
//! | `0x8008E650` | [`section4b_byte_12`] | byte `+0x12` of a section-4B entry |
//! | `0x8008E680` | [`section4b_byte_13`] | byte `+0x13` of a section-4B entry |
//! | `0x8008E770` | [`section4b_entry_at_point`] | section-4B entry containing a point |
//! | `0x8008E800` | [`section4b_entry_overlapping`] | section-4B entry overlapping a box |
//!
//! ## Why it exists
//!
//! The course file (see [`crate::game::stage::parse_course_file`]) stores
//! section tables of `u16` tile/object values. Two different arrays answer
//! questions about those values:
//!
//! * the **bgdat layer arrays**, loaded from `course/course<N>_bgdatL{0,1,2}.bin`
//!   and reached through the BG manager slots that
//!   [`crate::game::stage::load_bg_units`] fills; [`bg_layer_u16`] is how the
//!   rest of the game reads a single tile value out of one of them;
//! * the **object-id tables**, two `.rodata` arrays at `0x802EF900` and
//!   `0x802EF934` that translate/validate the object ids (`entry+0xC` of a
//!   0x10-byte course object entry) before an actor is spawned;
//! * the **seven section tables** that [`crate::game::stage::parse_course_file`]
//!   lays out at `+0x08..+0xB0` of the stage object. Each pair is a pointer
//!   array and a row count; the lookup functions below are named after the
//!   pair they walk (`section1A`, `section2A`, `section2B`, `section3A`,
//!   `section4B`). Entries have different strides and id fields, all observed
//!   from the loop arithmetic.
//!
//! Keeping them together here means "how do I read a value that came out of a
//! course file?" has one answer, and the stage runtime (manager, scene, tile
//! buffers) stays in [`crate::game::stage`].
//!
//! ## What is *not* here
//!
//! The `course/course<N>.bin` parser itself, the BG manager that owns the
//! handles, and the BG scene/tile buffers are **not** duplicated here; they
//! live in [`crate::game::stage`] because they are the stage bootstrap path.
//! This module only decodes the arrays.
//!
//! ## Data model
//!
//! The BG manager layout these readers use (observed from the writers in
//! `0x800814D0`; full map in [`crate::game::stage`]):
//!
//! * `+0x04 + stage*0xC + unit*4`: the `course<N>_bgdatL1.bin` handle that
//!   [`bg_layer_data`] returns;
//! * that handle points at the layer array [`bg_layer_u16`] indexes.
//!
//! The two object-id tables are raw `.rodata`; their contents and meaning are
//! only partly known (see [`object_id_table_entry`]).
//!
//! ## Start here
//!
//! [`bg_layer_u16`] is the most-used reader: one `u16` tile value out of a
//! loaded bgdat layer. [`object_id_in_list`] is the reference example of the
//! id tables.
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - big-endian `u16`/`u32` loads and raw calls.
//! * No other module is modified by this file; the manager is written by
//!   [`crate::game::stage`].
//!
//! `GX`, `BRRES` and `r13` are defined in `docs/GLOSSARY.md`.

use crate::engine::startup::SDA_BASE;
use crate::platform::{
    self, read_i32, read_u16, read_u32, read_u8, write_f32, write_u16,
};

// ---------------------------------------------------------------------------
// Read-only tables (all inside `.rodata`, observed in the ELF)
// ---------------------------------------------------------------------------

/// `u16` lookup table indexed by a 16-bit id; meaning unknown, sampled values
/// are `0`, `8`, `9`.
const OBJECT_ID_TABLE: usize = 0x802E_F934;

/// `u16` list of object ids terminated by `0xFFFF`.
///
/// observed: first entries `0x002E, 0x001E, 0x001F, 0x0431, 0x0831, ...`.
const OBJECT_ID_LIST: usize = 0x802E_F900;

/// `r2` (small-data-2) base = `0x80433080`, needed for the two `double`
/// conversion biases below.
///
/// observed: `lis r2,0x8043 / ori r2,r2,0x3080` in `__init_registers`
/// (`0x800041C0`); the stage module documents the same base.
const SDA2_BASE: usize = 0x8043_3080;

/// `r13 - 0x5680` → `0x8042A000`: global game/version flags object.
///
/// observed: `*(r13-0x5680)` is passed to `0x800D87D0` and `0x800D87B0` by
/// [`object_id_remap`]; both callees only read `+4` of that object.
const G_GAME_FLAGS: usize = SDA_BASE - 0x5680;

/// `r13 - 0x58B0` → `0x80429DD0`: global scene/camera owner.
///
/// observed: `FUN_80078510(*(r13-0x58B0))` in [`object_id_remap`]; the callee
/// reads byte `+0x90AA8`.
const G_CAMERA_OWNER: usize = SDA_BASE - 0x58B0;

/// `r2 - 0x7058` → `0x8042C028`: `double` `0x4330000000000000` = 2^52, the
/// bias used to convert unsigned 16-bit fixed-point x/z/w values to float.
const D_FIXED_BIAS_X: usize = SDA2_BASE - 0x7058;

/// `r2 - 0x7050` → `0x8042C030`: `double` `0x4330000080000000` =
/// 2^52 + 2^31, the bias used for the negated y fixed-point value.
const D_FIXED_BIAS_Z: usize = SDA2_BASE - 0x7050;

// ---------------------------------------------------------------------------
// External call targets (outside this lane, called by absolute address)
// ---------------------------------------------------------------------------

/// `0x800D87D0` (20 bytes): returns `1` when the game-flags word at `+4` is
/// non-zero. Used by [`object_id_remap`] as the "remapping is active" gate.
const FUN_800D87D0: usize = 0x800D_87D0;

/// `0x800D87B0` (20 bytes): `*(u32*)(flags + 4) & (1 << bit)`; returns
/// non-zero when the requested flag bit is set.
const FUN_800D87B0: usize = 0x800D_87B0;

/// `0x80078510` (12 bytes): returns byte `+0x90AA8` of the scene owner.
const FUN_80078510: usize = 0x8007_8510;

/// `0x8008E950` (200 bytes): point-in-rectangle test used by
/// [`section4b_entry_at_point`]. Arguments are `(stage, float_position,
/// entry)`; the first argument is not read by the callee.
const FUN_8008E950: usize = 0x8008_E950;

/// `0x8008EA20` (200 bytes): rectangle-overlap test used by
/// [`section4b_entry_overlapping`]. Arguments are `(stage, short_box, entry)`;
/// the first argument is not read by the callee.
const FUN_8008EA20: usize = 0x8008_EA20;

/// Reads a big-endian IEEE-754 double from `addr`.
///
/// # Safety
/// `addr..addr+8` must be readable and 8-byte aligned.
unsafe fn read_f64(addr: usize) -> f64 {
    let hi = read_u32(addr) as u64;
    let lo = read_u32(addr + 4) as u64;
    f64::from_bits((hi << 32) | lo)
}

// ---------------------------------------------------------------------------
// 0x800818B0
// ---------------------------------------------------------------------------

/// Returns the BG-layer file handle at `+4` for one stage/unit slot.
///
/// This is the `course<N>_bgdatL1.bin` handle: [`crate::game::stage::load_bg_units`]
/// stores the L1 file into this slot, and the model factory `0x80083560` reads
/// it before building a model. The array behind the handle is read with
/// [`bg_layer_u16`].
///
/// # Original
/// - Ghidra: `FUN_800818b0` at `0x800818b0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800818b0_FUN_800818b0.c`
/// - observed: `return *(undefined4 *)(p + stage*0xC + unit*4 + 4);`
///
/// # Arguments
/// - `manager`: BG manager base.
/// - `stage`: stage slot, 0..=3.
/// - `unit`: layer slot, 0..=3.
///
/// # Returns
/// The `course<N>_bgdatL1.bin` handle written by
/// [`crate::game::stage::load_bg_units`], or `0` when the load failed.
///
/// # Safety
/// `manager` must be a BG manager.
///
/// # Confidence
/// high - offset matches the L1 store in
/// [`crate::game::stage::load_bg_units`]; `0x80083560` also reads this slot
/// before creating a model.
pub unsafe fn bg_layer_data(manager: usize, stage: u32, unit: u32) -> u32 {
    read_u32(manager + (stage as usize) * 0xc + (unit as usize) * 4 + 4)
}

// ---------------------------------------------------------------------------
// 0x800818E0 / 0x80081900 - object-id tables
// ---------------------------------------------------------------------------

/// Looks up a 16-bit id in the `u16` table at `0x802EF934`.
///
/// # Original
/// - Ghidra: `FUN_800818e0` at `0x800818E0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800818e0_FUN_800818e0.c`
/// - observed: `return *(undefined2 *)(&DAT_802ef934 + (param_1 & 0xffff) * 2);`
///
/// # Arguments
/// - `id`: index; only the low 16 bits are used.
///
/// # Returns
/// The table entry (sampled values: `0`, `8`, `9`).
///
/// # Safety
/// `id` must index inside the table; nothing validates it.
///
/// # Confidence
/// medium - the load is certain, the table's meaning is not.
pub unsafe fn object_id_table_entry(id: u32) -> u16 {
    read_u16(OBJECT_ID_TABLE + (id & 0xffff) as usize * 2)
}

/// Searches the object-id list for `id`, advancing a caller-owned cursor.
///
/// The list at `0x802EF900` is a `u16` array terminated by `0xFFFF`. The search
/// starts at `*cursor`, stores the index it reached back into `*cursor`, and
/// stops on the terminator (not found) or on a match (found).
///
/// # Original
/// - Ghidra: `FUN_80081900` at `0x80081900` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80081900_FUN_80081900.c`
/// - observed: `*param_2 = 0;` then a loop testing `*(short *)(&DAT_802ef900 +
///   *param_2*2) == -1`, incrementing, and comparing with the entry at the new
///   index.
///
/// # Arguments
/// - `id`: object id to look for.
/// - `cursor`: pointer to a `u16` index that is both input and output.
///
/// # Returns
/// `1` if `id` is in the list, `0` if the terminator was reached first.
///
/// # Side effects
/// Writes the running/terminal index through `cursor`.
///
/// # Safety
/// `cursor` must be a writable `u16`; the list must contain a `0xFFFF`
/// terminator, otherwise the scan runs past it.
///
/// # Confidence
/// medium - control flow is observed; the meaning of the list is inferred from
/// its users.
pub unsafe fn object_id_in_list(id: u32, cursor: *mut u16) -> u32 {
    write_u16(cursor as usize, 0);
    loop {
        let index = read_u16(cursor as usize) as usize;
        if read_u16(OBJECT_ID_LIST + index * 2) == 0xffff {
            return 0;
        }
        // observed: the index is bumped *before* the comparison, and the
        // evidence masks the byte offset with 0x1fffe (u16 index wrap).
        let next = (index as u16).wrapping_add(1);
        write_u16(cursor as usize, next);
        if id == read_u16(OBJECT_ID_LIST + next as usize * 2) as u32 {
            return 1;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x80081BA0
// ---------------------------------------------------------------------------

/// Returns `1` when the id remap of `0x2E` is `0x2F` (a "special" object).
///
/// The original calls `fun_80081ab0(0x2E)` - a table-driven id remap - then
/// computes `countLeadingZeros((value & 0xFFFF) - 0x2F) >> 5`, which is `1`
/// exactly when the 16-bit result is `0x2F`.
///
/// # Original
/// - Ghidra: `FUN_80081ba0` at `0x80081BA0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80081ba0_FUN_80081ba0.c`
/// - observed: `uVar1 = FUN_80081ab0(0x2e); countLeadingZeros((uVar1 & 0xffff)
///   - 0x2f) >> 5`.
///
/// # Returns
/// `1` or `0`.
///
/// # Safety
/// `0x80081AB0` must be callable (it reads `r13-0x5680`).
///
/// # Confidence
/// medium - the arithmetic is certain; why object `0x2E` matters is inferred.
pub unsafe fn is_special_object_id() -> u32 {
    let remapped = platform::call1(0x8008_1AB0, 0x2e);
    ((remapped & 0xffff).wrapping_sub(0x2f)).leading_zeros() >> 5
}

// ---------------------------------------------------------------------------
// 0x80082120
// ---------------------------------------------------------------------------

/// Returns one `u16` entry of a BG-layer buffer.
///
/// # Original
/// - Ghidra: `FUN_80082120` at `0x80082120` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80082120_FUN_80082120.c`
/// - observed: `*(undefined2 *)(*(int *)(p + stage*0xC + unit*4 + 4) +
///   (index & 0xffff) * 2)` - reads through the `+4` layer handle, unlike
///   [`crate::game::stage::bg_unit_u16`] which reads the `+0x64` block.
///
/// # Arguments
/// - `scene`: BG scene base (the first argument is `param_1` in the evidence;
///   the same `+4` slot is used by the BG manager).
/// - `stage`: stage slot, 0..=3.
/// - `index`: entry index; only the low 16 bits are used.
/// - `unit`: layer slot, 0..=3.
///
/// # Returns
/// The `u16` at `index` in the stage/unit BG-layer array.
///
/// # Safety
/// The `+4` handle for that slot must be a readable array long enough for
/// `index`.
///
/// # Confidence
/// high - address arithmetic matches the store site in
/// [`crate::game::stage::load_bg_units`].
pub unsafe fn bg_layer_u16(scene: usize, stage: u32, index: u32, unit: u32) -> u16 {
    let data = read_u32(scene + (stage as usize) * 0xc + (unit as usize) * 4 + 4) as usize;
    read_u16(data + (index & 0xffff) as usize * 2)
}

// ---------------------------------------------------------------------------
// 0x80082140
// ---------------------------------------------------------------------------

/// Writes one `u16` entry through a loaded bgdat layer.
///
/// This is the writer that pairs with [`bg_layer_u16`]: both use the layer
/// handle at `+4` of a stage/unit slot, so the same `(stage, index, unit)`
/// addresses the same tile value.
///
/// # Original
/// - Ghidra: `FUN_80082140` at `0x80082140` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80082140_FUN_80082140.c`
/// - observed: `*(u16 *)([scene+stage*0xC+unit*4+4] + (index&0xFFFF)*2)
///   = value;`
///
/// # Arguments
/// - `scene`: BG manager/scene base (the `+4` slot is the L1 layer handle).
/// - `stage`, `unit`: slot indices (stage stride `0xC`, unit stride `4`).
/// - `index`: entry index; only the low 16 bits are used.
/// - `value`: tile value to store.
///
/// # Side effects
/// Writes two bytes into the loaded layer buffer.
///
/// # Safety
/// The `+4` handle must be a writable array long enough for `index`.
///
/// # Confidence
/// high - address arithmetic matches [`bg_layer_u16`] exactly.
pub unsafe fn bg_layer_write_u16(
    scene: usize,
    stage: u32,
    index: u32,
    unit: u32,
    value: u16,
) {
    let data = read_u32(scene + (stage as usize) * 0xc + (unit as usize) * 4 + 4) as usize;
    write_u16(data + (index & 0xffff) as usize * 2, value);
}

// ---------------------------------------------------------------------------
// 0x80081AB0
// ---------------------------------------------------------------------------

/// Remaps a course object id for the active game/version flags.
///
/// Two flag bits of the global flags object (`+4`) select alternate id
/// mappings: bit 0 swaps `0x1E ↔ 0x30` and maps `0x2E → 0x2F`; bit 1 maps
/// `0x3B` and `0x3C` to `0x37`. If the camera owner's `+0x90AA8` byte is
/// non-zero, `0x3B`/`0x3C` are also mapped to `0x37` even when the flag bits
/// are clear.
///
/// # Original
/// - Ghidra: `FUN_80081ab0` at `0x80081AB0` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80081ab0_FUN_80081ab0.c`
/// - observed: `uVar2 = *(r13-0x5680);` gate `FUN_800d87d0(uVar2)`;
///   `FUN_800d87b0(uVar2,0)` branch containing the `0x1E/0x30/0x2E` cases;
///   `FUN_800d87b0(uVar2,1)` branch containing `0x3B/0x3C → 0x37`; then
///   `FUN_80078510(*(r13-0x58b0))` repeats the `0x3B/0x3C → 0x37` mapping.
///
/// # Arguments
/// - `id`: source object id (values above `0xFFFF` are not masked here).
///
/// # Returns
/// The remapped id, or the input unchanged.
///
/// # Side effects
/// Calls three out-of-lane getters; no memory writes.
///
/// # Safety
/// [`G_GAME_FLAGS`] and [`G_CAMERA_OWNER`] must point at live objects.
///
/// # Confidence
/// medium - the id cases and branch order are observed; why each flag bit is
/// set (version/region) is unknown.
pub unsafe fn object_id_remap(id: u32) -> u32 {
    let flags = read_u32(G_GAME_FLAGS) as usize;
    let mut result = id;

    // unknown: which version/region each of the two flag bits selects; only
    // the id transformations themselves are observed.
    if platform::call1(FUN_800D87D0, flags as u32) != 0 {
        if platform::call2(FUN_800D87B0, flags as u32, 0) != 0 {
            if id == 0x1e {
                return 0x30;
            }
            if id == 0x30 {
                return 0x1e;
            }
            if id == 0x2e {
                return 0x2f;
            }
        }
        if platform::call2(FUN_800D87B0, flags as u32, 1) != 0
            && (id == 0x3b || id == 0x3c)
        {
            return 0x37;
        }
    }

    if platform::call1(FUN_80078510, read_u32(G_CAMERA_OWNER)) != 0
        && (result == 0x3b || result == 0x3c)
    {
        result = 0x37;
    }
    result
}

// ---------------------------------------------------------------------------
// 0x800822E0
// ---------------------------------------------------------------------------

/// Remaps an object id (optionally) and keeps its low 10 bits.
///
/// # Original
/// - Ghidra: `FUN_800822e0` at `0x800822E0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800822e0_FUN_800822e0.c`
/// - observed: `if (param_2 != 0) param_1 = FUN_80081ab0();` then
///   `return param_1 & 0x3FF;`. Ghidra drops the argument of the call; `r3`
///   still holds `param_1`, so [`object_id_remap`] receives the same id.
///
/// # Arguments
/// - `id`: object id.
/// - `remap`: non-zero to run [`object_id_remap`] first.
///
/// # Returns
/// `id & 0x3FF` (or the remapped id masked the same way).
///
/// # Safety
/// Same requirements as [`object_id_remap`] when `remap` is non-zero.
///
/// # Confidence
/// medium - the mask is certain; passing the original id to the remap is
/// inferred from the surviving `r3`.
pub unsafe fn object_tile_id(id: u32, remap: u32) -> u32 {
    let mut value = id;
    if remap != 0 {
        // inferred: Ghidra prints the call without an argument, but `r3`
        // still holds `id` at the call site, so the remap sees the same id.
        value = object_id_remap(id);
    }
    value & 0x3ff
}

// ---------------------------------------------------------------------------
// 0x8008E310
// ---------------------------------------------------------------------------

/// Finds a section-1A entry by the low byte of the `u16` at entry `+0x10`.
///
/// Entries are `0x18` bytes; the count is stage `+0x80` and the base is stage
/// `+0x10` (section 1, first pointer). Unlike the other lookups this one
/// returns the section base when no entry matches.
///
/// # Original
/// - Ghidra: `FUN_8008e310` at `0x8008E310` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e310_FUN_8008e310.c`
/// - observed: `iVar1 = [stage+0x80]; iVar2 = [stage+0x10];` loop while
///   `iVar1 != 0`, compare `(*(u16*)(iVar2+0x10) & 0xFF) == id`, advance
///   `0x18`; exhaustion returns `[stage+0x10]`.
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: id to match (only the low 8 bits are meaningful).
///
/// # Returns
/// Entry address, or the section base if no entry matches.
///
/// # Safety
/// The stage section pointers/counts must come from a parsed course file.
///
/// # Confidence
/// medium - the loop is certain; the section number is inferred from the
/// [`crate::game::stage::parse_course_file`] layout.
pub unsafe fn section1_entry_by_id(stage: usize, id: u32) -> usize {
    // inferred: the five section numbers below follow the pointer/count pairs
    // written by crate::game::stage::parse_course_file.
    let mut remaining = read_i32(stage + 0x80);
    let base = read_u32(stage + 0x10) as usize;
    let mut entry = base;
    loop {
        if remaining == 0 {
            return base;
        }
        if (read_u16(entry + 0x10) & 0xff) as u32 == id {
            return entry;
        }
        entry += 0x18;
        remaining -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x8008E350 / 0x8008E390
// ---------------------------------------------------------------------------

/// Finds a section-2A entry by its leading `u16`.
///
/// Entries are `0x18` bytes; the count is stage `+0x88`, the base is stage
/// `+0x18` (section 2, first pointer).
///
/// # Original
/// - Ghidra: `FUN_8008e350` at `0x8008E350` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e350_FUN_8008e350.c`
/// - observed: `iVar1 = [stage+0x88]; puVar2 = [stage+0x18];` compare
///   `*puVar2 == id`, advance `0xC` `u16` (= `0x18` bytes), return `0` on
///   exhaustion.
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: leading `u16` to match.
///
/// # Returns
/// Entry address, or `0`.
///
/// # Safety
/// The section pointer/count must be valid.
///
/// # Confidence
/// medium - loop observed; section number inferred from the course layout.
pub unsafe fn section2_entry_by_id(stage: usize, id: u32) -> usize {
    let mut remaining = read_i32(stage + 0x88);
    let mut entry = read_u32(stage + 0x18) as usize;
    loop {
        if remaining == 0 {
            return 0;
        }
        if read_u16(entry) as u32 == id {
            return entry;
        }
        entry += 0x18;
        remaining -= 1;
    }
}

/// Finds a section-2B entry by its leading `u16`.
///
/// Same shape as [`section2_entry_by_id`] but uses stage `+0x8C` (count) and
/// stage `+0x1C` (base), i.e. section 2, second pointer.
///
/// # Original
/// - Ghidra: `FUN_8008e390` at `0x8008E390` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e390_FUN_8008e390.c`
/// - observed: `iVar1 = [stage+0x8C]; puVar2 = [stage+0x1C];` same loop.
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: leading `u16` to match.
///
/// # Returns
/// Entry address, or `0`.
///
/// # Safety
/// The section pointer/count must be valid.
///
/// # Confidence
/// medium - loop observed; section number inferred from the course layout.
pub unsafe fn section2b_entry_by_id(stage: usize, id: u32) -> usize {
    let mut remaining = read_i32(stage + 0x8c);
    let mut entry = read_u32(stage + 0x1c) as usize;
    loop {
        if remaining == 0 {
            return 0;
        }
        if read_u16(entry) as u32 == id {
            return entry;
        }
        entry += 0x18;
        remaining -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x8008E3D0
// ---------------------------------------------------------------------------

/// Finds a section-3A entry by the byte at entry `+8`.
///
/// Entries are `0x14` bytes; the count is stage `+0x90` and the base is stage
/// `+0x20` (section 3, first pointer). A non-positive count yields `0`.
///
/// # Original
/// - Ghidra: `FUN_8008e3d0` at `0x8008E3D0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e3d0_FUN_8008e3d0.c`
/// - observed: `iVar1 = [stage+0x90]; iVar2 = [stage+0x20];` if `0 < iVar1`
///   loop comparing `*(byte*)(iVar2+8) == id`, advancing `0x14`; else `0`.
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: byte id to match.
///
/// # Returns
/// Entry address, or `0`.
///
/// # Safety
/// The section pointer/count must be valid.
///
/// # Confidence
/// medium - loop observed; section number inferred from the course layout.
pub unsafe fn section3_entry_by_id(stage: usize, id: u32) -> usize {
    let mut remaining = read_i32(stage + 0x90);
    let mut entry = read_u32(stage + 0x20) as usize;
    if remaining <= 0 {
        return 0;
    }
    loop {
        if read_u8(entry + 8) as u32 == id {
            return entry;
        }
        entry += 0x14;
        remaining -= 1;
        if remaining == 0 {
            return 0;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x8008E410
// ---------------------------------------------------------------------------

/// Finds a section-4B entry by the byte at entry `+0xC`.
///
/// Entries are `0x18` bytes; the count is stage `+0x9C` and the base is stage
/// `+0x2C` (section 4, second pointer). When `out` is non-null the entry's
/// four fixed-point `u16` coordinates are converted to floats:
/// `out = [x, -y, z, w]`.
///
/// # Original
/// - Ghidra: `FUN_8008e410` at `0x8008E410` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e410_FUN_8008e410.c`
/// - observed: `iVar2 = [stage+0x9C]; puVar1 = [stage+0x2C];` compare
///   `*(byte*)(puVar1+6) == id` (`puVar1` is `u16*`, so byte `+0xC`), advance
///   `0xC` `u16` (= `0x18`). For the output:
///   `(double)CONCAT44(0x43300000, *puVar1) - [r2-0x7058]`,
///   `(double)CONCAT44(0x43300000, -(u32)puVar1[1] ^ 0x80000000) -
///   [r2-0x7050]`, and the same x bias for `+2`/`+3`.
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: byte id to match.
/// - `out`: optional address of four `f32` values, or `0`.
///
/// # Returns
/// Entry address, or `0`.
///
/// # Side effects
/// Writes 16 bytes through `out` when it is non-null.
///
/// # Safety
/// The section pointer/count must be valid; `out` must be null or writable
/// for 16 bytes.
///
/// # Confidence
/// medium - the scan is certain; the fixed-point layout and the float
/// conversion are observed but the coordinate's unit is unknown.
pub unsafe fn section4b_entry_by_id(stage: usize, id: u32, out: usize) -> usize {
    let mut remaining = read_i32(stage + 0x9c);
    let mut entry = read_u32(stage + 0x2c) as usize;
    loop {
        if remaining == 0 {
            return 0;
        }
        if read_u8(entry + 0xc) as u32 == id {
            break;
        }
        entry += 0x18;
        remaining -= 1;
    }

    if out != 0 {
        // inferred: the four fixed-point u16 values are stage-local
        // coordinates; the unit and the meaning of the negated y are unknown.
        let k = f64::from_bits(0x4330_0000_0000_0000);
        let bias_x = read_f64(D_FIXED_BIAS_X);
        let bias_z = read_f64(D_FIXED_BIAS_Z);

        let x = k + read_u16(entry) as u32 as f64 - bias_x;
        let neg_y = 0u32.wrapping_sub(read_u16(entry + 2) as u32) ^ 0x8000_0000;
        let y = k + neg_y as f64 - bias_z;
        let z = k + read_u16(entry + 4) as u32 as f64 - bias_x;
        let w = k + read_u16(entry + 6) as u32 as f64 - bias_x;

        write_f32(out, x as f32);
        write_f32(out + 4, y as f32);
        write_f32(out + 8, z as f32);
        write_f32(out + 12, w as f32);
    }
    entry
}

// ---------------------------------------------------------------------------
// 0x8008E4D0 / 0x8008E500 / 0x8008E650 / 0x8008E680
// ---------------------------------------------------------------------------

/// Returns byte `+0xC` of the section-4B entry with byte id `id`.
///
/// # Original
/// - Ghidra: `FUN_8008e4d0` at `0x8008E4D0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e4d0_FUN_8008e4d0.c`
/// - observed: `iVar1 = FUN_8008e410(stage,id,0); return *(byte*)(iVar1+0xC);`
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: section-4B byte id.
///
/// # Returns
/// The byte at `+0xC` of the matching entry.
///
/// # Safety
/// An entry with `id` must exist; the original does not check for `0`.
///
/// # Confidence
/// high - direct call and load.
pub unsafe fn section4b_byte_c(stage: usize, id: u32) -> u8 {
    let entry = section4b_entry_by_id(stage, id, 0);
    read_u8(entry + 0xc)
}

/// Returns byte `+0xD` of the section-4B entry with byte id `id`.
///
/// # Original
/// - Ghidra: `FUN_8008e500` at `0x8008E500` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e500_FUN_8008e500.c`
/// - observed: `iVar1 = FUN_8008e410(stage,id,0); return *(byte*)(iVar1+0xD);`
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: section-4B byte id.
///
/// # Returns
/// The byte at `+0xD` of the matching entry.
///
/// # Safety
/// An entry with `id` must exist.
///
/// # Confidence
/// high - direct call and load.
pub unsafe fn section4b_byte_d(stage: usize, id: u32) -> u8 {
    let entry = section4b_entry_by_id(stage, id, 0);
    read_u8(entry + 0xd)
}

/// Returns byte `+0x12` of the section-4B entry with byte id `id`.
///
/// # Original
/// - Ghidra: `FUN_8008e650` at `0x8008E650` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e650_FUN_8008e650.c`
/// - observed: `iVar1 = FUN_8008e410(stage,id,0); return *(byte*)(iVar1+0x12);`
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: section-4B byte id.
///
/// # Returns
/// The byte at `+0x12` of the matching entry.
///
/// # Safety
/// An entry with `id` must exist.
///
/// # Confidence
/// high - direct call and load.
pub unsafe fn section4b_byte_12(stage: usize, id: u32) -> u8 {
    let entry = section4b_entry_by_id(stage, id, 0);
    read_u8(entry + 0x12)
}

/// Returns byte `+0x13` of the section-4B entry with byte id `id`.
///
/// # Original
/// - Ghidra: `FUN_8008e680` at `0x8008E680` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e680_FUN_8008e680.c`
/// - observed: `iVar1 = FUN_8008e410(stage,id,0); return *(byte*)(iVar1+0x13);`
///
/// # Arguments
/// - `stage`: stage object.
/// - `id`: section-4B byte id.
///
/// # Returns
/// The byte at `+0x13` of the matching entry.
///
/// # Safety
/// An entry with `id` must exist.
///
/// # Confidence
/// high - direct call and load.
pub unsafe fn section4b_byte_13(stage: usize, id: u32) -> u8 {
    let entry = section4b_entry_by_id(stage, id, 0);
    read_u8(entry + 0x13)
}

// ---------------------------------------------------------------------------
// 0x8008E770 / 0x8008E800
// ---------------------------------------------------------------------------

/// Finds the section-4B entry whose rectangle contains a 3-float position.
///
/// Walks the `+0x2C`/`+0x9C` section with stride `0x18` and calls the
/// point-in-rectangle helper `0x8008E950` for each entry. Returns `0xFF`
/// when no entry matches.
///
/// # Original
/// - Ghidra: `FUN_8008e770` at `0x8008E770` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e770_FUN_8008e770.c`
/// - observed: `uVar2 = 0; iVar3 = [stage+0x2C];` loop:
///   `if ([stage+0x9C] <= uVar2) return 0xFF;`
///   `iVar1 = FUN_8008e950(stage,position,iVar3); if (iVar1 != 0) break;`
///   then `iVar3 += 0x18; uVar2 += 1;` and finally
///   `return *(byte*)(iVar3+0xC);`.
///
/// # Arguments
/// - `stage`: stage object.
/// - `position`: address of the float vector `[x, z, 0]` used by the helper.
///
/// # Returns
/// Byte `+0xC` of the first matching entry, or `0xFF`.
///
/// # Safety
/// `position` must be readable as three `f32` values; the section must be
/// valid. Callers pass `z` in the second slot because the helper compares the
/// negated y coordinate.
///
/// # Confidence
/// medium - the scan is certain; the helper's rectangle convention and the
/// "area id" meaning of `+0xC` are inferred.
pub unsafe fn section4b_entry_at_point(stage: usize, position: usize) -> u8 {
    let mut index = 0u32;
    let mut entry = read_u32(stage + 0x2c) as usize;
    loop {
        if read_u32(stage + 0x9c) <= index {
            return 0xff;
        }
        let found = platform::call3(
            FUN_8008E950,
            stage as u32,
            position as u32,
            entry as u32,
        );
        if found != 0 {
            break;
        }
        entry += 0x18;
        index += 1;
    }
    read_u8(entry + 0xc)
}

/// Finds the section-4B entry whose rectangle overlaps a short box.
///
/// Same walk as [`section4b_entry_at_point`] but calls the overlap helper
/// `0x8008EA20` and returns `0` when no entry matches.
///
/// # Original
/// - Ghidra: `FUN_8008e800` at `0x8008E800` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8008e800_FUN_8008e800.c`
/// - observed: `uVar2 = 0; iVar3 = [stage+0x2C];` loop:
///   `if ([stage+0x9C] <= uVar2) return 0;`
///   `iVar1 = FUN_8008ea20(stage,box,iVar3); if (iVar1 != 0) break;`
///   then `iVar3 += 0x18; uVar2 += 1;` and finally
///   `return *(byte*)(iVar3+0xC);`.
///
/// # Arguments
/// - `stage`: stage object.
/// - `box`: address of four `i16` values `[x, y, width, height]`.
///
/// # Returns
/// Byte `+0xC` of the first overlapping entry, or `0`.
///
/// # Safety
/// `box` must be readable as four `i16` values; the section must be valid.
///
/// # Confidence
/// medium - the scan is certain; the box layout comes from the caller
/// `0x80068320` (`puVar17[1]`, `puVar17[2]`, two table-scaled `u16`s).
pub unsafe fn section4b_entry_overlapping(stage: usize, box_addr: usize) -> u8 {
    let mut index = 0u32;
    let mut entry = read_u32(stage + 0x2c) as usize;
    loop {
        if read_u32(stage + 0x9c) <= index {
            return 0;
        }
        let found = platform::call3(
            FUN_8008EA20,
            stage as u32,
            box_addr as u32,
            entry as u32,
        );
        if found != 0 {
            break;
        }
        entry += 0x18;
        index += 1;
    }
    read_u8(entry + 0xc)
}
