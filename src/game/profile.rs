//! # Profile / yes-no window state registry (`d_profileNP`)
//!
//! ## What this module covers
//!
//! `d_profileNP` is REL module 1. The name suggests a "profile" module, but the
//! recovered code contains no actor descriptors. Its only named class is
//! `dYesNoWindow_c` (observed in `work/rels/d_profileNP.strings.txt`), together
//! with a `ControllerInformation` layout. This is the state registry and the
//! yes/no confirmation dialog used by the profile / file-select front end.
//!
//! The module has two jobs:
//!
//! 1. **Register UI state descriptors.** At boot it builds eight
//!    `dYesNoWindow_c` state objects in BSS, each with a name, a vtable and
//!    three callback triples, and links them into a global singly linked list.
//!    [`register_yes_no_states`] is the table initializer;
//!    [`register_state_descriptor`] and [`unregister_all_states`] operate the
//!    list.
//! 2. **Run the dialog.** Once registered, the state callbacks in this file
//!    drive the open/select/hit/close animation sequence and the
//!    controller-information overlay.
//!
//! There is no hash table or searched descriptor array. "Descriptor lookup"
//! here means that a state callback passes a fixed descriptor address to the
//! state manager, which then dispatches through the descriptor's vtable.
//!
//! ## Synthetic-address warning
//!
//! Every address in this module is an **analysis address**, not a runtime
//! address. The REL was relocated to the documented canonical slot
//! **`0x80500000`** by `tools/rel2elf.mjs`; see `docs/REL.md` and
//! `docs/REL2ELF.md`. The entry point is **`0x805000E0`**. The retail loader may
//! place the module anywhere, and every internal pointer would change.
//!
//! Most `0x80xxxxxx` addresses outside the `0x80500000..0x80503B08` slot are
//! `main.dol` imports. The descriptor block at `0x80502288` also contains
//! cross-REL pointers into `d_basesNP`, `d_enemiesNP` and `d_en_bossNP`. All of
//! them are file-absolute analysis addresses, not REL-relative ones and not
//! live-process addresses.
//!
//! ## Original address range
//!
//! | region | range | contents |
//! |---|---|---|
//! | text | `0x805000E0..0x80502098` | all 36 exported functions |
//! | data | `0x80502098..0x80503884` | strings, vtables, seed and lookup tables |
//! | BSS | `0x80503900..0x80503B08` | state list head and eight 0x40-byte state records |
//!
//! The evidence export is `work/ghidra/rel-d_profileNP/export/` (36 C files and
//! `functions.tsv`). Function addresses in this file are the Ghidra entry
//! addresses in that export.
//!
//! ## Why it exists
//!
//! The profile screens need a modal question: "yes / no / centre" with an
//! optional controller diagram. This module owns both the state machine for
//! that question and the controller-information layout that appears beside it.
//! Keeping the registry, the object lifecycle and the state callbacks in one
//! semantic file makes the sequence readable without walking a range lane.
//!
//! ## Start here
//!
//! 1. [`entry`] (`0x805000E0`) queues [`register_yes_no_states`] through a
//!    `main.dol` initializer table and installs the module descriptor table.
//! 2. [`register_yes_no_states`] (`0x80501980`) builds the eight descriptors
//!    and links them into `STATE_LIST_HEAD`.
//! 3. [`state_enter_open_wait`] (`0x80501050`) is the first dialog callback;
//!    the `state_enter_*` / `state_poll_*` pairs then walk the sequence
//!    open -> button-on -> button-change -> select -> hit -> close.
//! 4. [`state_poll_close`] (`0x80501700`) returns to the `InitWait` descriptor.
//!
//! ## Object layout used by the state callbacks
//!
//! The yes/no window object is at least `0x29B` bytes. All offsets below are
//! relative to that object; confidence is marked because the Ghidra C exposes
//! raw offsets rather than field names.
//!
//! | offset | width | observed use | confidence |
//! |---|---|---|---|
//! | `+0x60` | word | vtable `0x80503470` | observed |
//! | `+0x70` | object | embedded NW4R `lyt` layout (`0x800C8950` ctor) | observed |
//! | `+0x208` | word | state-manager interface vtable, finally `0x805034C0` | observed |
//! | `+0x20C` | word | secondary interface `0x80503580` | observed |
//! | `+0x210` | word | secondary interface `0x80503520` | observed |
//! | `+0x214` | word | secondary interface `0x80503538` | observed |
//! | `+0x218` | word | back-pointer to the window object | observed |
//! | `+0x21C` | word | zeroed by the constructor | observed |
//! | `+0x220` | object | state-manager object built by `0x8015FC90` | inferred |
//! | `+0x244..+0x280` | words | pointers to layout panes | observed |
//! | `+0x284` | word | current focused button, 0..2 | inferred |
//! | `+0x288` | word | previous button, or `-1` | inferred |
//! | `+0x28C` | word | profile/question index, 0..28 | inferred |
//! | `+0x290` | word | animation/layout binding used by `0x800B3B60` | unknown |
//! | `+0x294..+0x29A` | bytes | state-machine flags | observed |
//!
//! The flag bytes have these observed uses:
//!
//! * `+0x295` is cleared when the close animation finishes.
//! * `+0x296` distinguishes the normal open/select path from the short path.
//! * `+0x297` is set by the `state_enter_*` callbacks.
//! * `+0x298` selects `state_poll_hit`'s next descriptor.
//! * `+0x299` forces the close path when non-zero.
//! * `+0x29A` gates the two optional button animations.
//!
//! ## The state registry
//!
//! The list head is `STATE_LIST_HEAD` (`0x80503900`, BSS). A record occupies
//! `0x40` bytes:
//!
//! ```text
//! +0x00  next record             (list link)
//! +0x04  cleanup callback        (always state_cleanup 0x80501FB0)
//! +0x08  descriptor pointer      (record + 0x10)
//! +0x0C  padding
//! +0x10  descriptor: vtable 0x80503850
//! +0x1C..+0x2F  nine descriptor words copied from the seed table
//! ```
//!
//! The seed table at `0x805035D0` contains eight entries of nine words. Each
//! entry is three `{0, -1, callback}` triples. [`register_yes_no_states`]
//! reorders them into the descriptor's three callback slots.
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! * `main.dol` - the constructors, allocator, NW4R layout helpers and event
//!   helper called by address. They are not part of this REL.
//!
//! No other `game` module is called. The data tables in this REL are referenced
//! by absolute address because they have no Rust owner yet.
//!
//! ## Status
//!
//! All 36 functions in `functions.tsv` are represented: 34 are `translated` and
//! 2 (`0x80501920`, `0x80501950`) are `partial` because their hidden `r12`
//! operand cannot be passed through [`crate::platform::call3`]. No function is a
//! stub. The 12 state callbacks that are referenced by the seed table but
//! missing from the export are listed in `docs/COVERAGE_REL_d_profileNP.md`.
//!
//! ## Glossary
//!
//! * **REL** - the Wii relocatable module format; see `docs/GLOSSARY.md`.
//! * **NW4R / lyt** - Nintendo's layout/animation library used for the dialog.
//! * **pane** - one drawable node in a `lyt` layout. `+0xBB` bit 0 is the
//!   visibility flag this module toggles.
//! * **descriptor** - a `dYesNoWindow_c` state object: name, vtable and three
//!   callback triples.
//! * **synthetic base** - the fixed analysis address chosen for a REL, here
//!   `0x80500000`.

use crate::game::bases::interface_call;
use crate::platform::{
    call0, call1, call2, call3, call4, call5, read_i32, read_u8, read_u32, write_u8, write_u32,
};

// ---------------------------------------------------------------------------
// Shared constants
// ---------------------------------------------------------------------------

/// Head of the intrusive state list in BSS (`DAT_80503900`).
///
/// A zero word means "no states registered". The list is only touched by
/// [`register_state_descriptor`] and [`unregister_all_states`].
const STATE_LIST_HEAD: usize = 0x8050_3900;

/// First 0x40-byte state record. The descriptor starts at record `+0x10`.
const STATE_RECORD_BASE: usize = 0x8050_3908;

/// Distance between two state records.
const STATE_RECORD_STRIDE: usize = 0x40;

/// Seed table for the eight descriptors: nine words per state.
///
/// Layout per entry is `{0, -1, callback}` three times. The `-1` is a state ID
/// placeholder; the callback words point at both exported and unexported text
/// addresses.
const STATE_SEED_TABLE: usize = 0x8050_35D0;

/// Distance between two seed entries.
const STATE_SEED_STRIDE: usize = 0x24;

/// Vtable installed in every state descriptor (`DAT_80503850`).
const STATE_DESCRIPTOR_VTABLE: u32 = 0x8050_3850;

/// `main.dol` global that receives the module descriptor table
/// (`_DAT_8042A3B8`). Its consumer is outside this REL.
const PROFILE_DESCRIPTOR_SLOT: usize = 0x8042_A3B8;

/// The descriptor/vtable block at `DAT_80502288` (a run of relocated
/// `main.dol` and cross-REL function pointers).
const PROFILE_DESCRIPTOR_TABLE: u32 = 0x8050_2288;

/// `main.dol` global whose word is passed to the `0x80195380` event helper.
///
/// unknown: the owning subsystem is not identified. `0x80195380` forwards to
/// `0x80195090(manager, manager + 0x8F8, event_id, flag)`, which looks like an
/// event/sound cue but is not proven.
const EVENT_MANAGER: usize = 0x8042_A488;

/// First of two adjacent `main.dol` bytes compared against `7` and `0x17`.
/// inferred: controller/extension identity; the pair only selects one hint
/// layout at profile index `0x14`.
const HINT_VARIANT_A: usize = 0x8031_5B9C;

/// Second hint-variant byte; see [`HINT_VARIANT_A`].
const HINT_VARIANT_B: usize = 0x8031_5B9D;

/// 29-entry table of animation/frame ids indexed by the profile index
/// (`DAT_80502138`).
const PROFILE_ANIM_IDS: usize = 0x8050_2138;

/// 29 x 3 bytes of button-pane visibility flags (`DAT_805021AC`).
///
/// Entry `i` at `i*3` belongs to the three panes at `+0x248`, `+0x24C` and
/// `+0x250`. Observed values are only `0` and `1`.
const PROFILE_PANE_FLAGS: usize = 0x8050_21AC;

/// 29-entry table of the default focused button (`DAT_80502204`).
/// Observed values are `0`, `1` and `2`.
const PROFILE_DEFAULT_BUTTON: usize = 0x8050_2204;

/// Animation binding table used only for profile index `6`
/// (`DAT_80503384`). Its first word is `2`; the following words are text
/// pointers starting with `0x80501100` and `0x805010E4`.
const CONTROLLER_HIT_TABLE: usize = 0x8050_3384;

// ---------------------------------------------------------------------------
// Small helpers shared by several original functions
// ---------------------------------------------------------------------------

/// Sets or clears bit 0 of a pane's flag byte at `+0xBB`.
///
/// This is the read-modify-write pattern repeated in `0x80500C70`,
/// `0x80501050` and `0x80501400`. The mask `0xFE` preserves every other flag
/// bit; bit 0 is the pane visibility/enable bit.
///
/// # Safety
/// `obj + field_offset` must hold a readable pane pointer, and `pane + 0xBB`
/// must be writable.
#[inline]
unsafe fn set_pane_bit0(obj: usize, field_offset: usize, on: bool) {
    let pane = read_u32(obj + field_offset) as usize;
    let flags = read_u8(pane + 0xbb);
    let value = (flags & 0xfe) | u8::from(on);
    write_u8(pane + 0xbb, value);
}

/// Changes the dialog state through the state-manager vtable at `obj + 0x208`.
///
/// The manager's vtable slot `+0x18` takes the interface pointer and the
/// descriptor address. Every `state_poll_*` callback uses this exact shape.
///
/// # Safety
/// `obj + 0x208` must be an initialized state-manager interface and
/// `descriptor` must point at a registered state descriptor.
#[inline]
unsafe fn switch_state(obj: usize, descriptor: usize) {
    let vtable = read_u32(obj + 0x208) as usize;
    let change = read_u32(vtable + 0x18) as usize;
    call2(change, (obj + 0x208) as u32, descriptor as u32);
}

// ---------------------------------------------------------------------------
// Boot and module install
// ---------------------------------------------------------------------------

/// REL entry point: queue the state-table initializer and install the module
/// descriptor table.
///
/// The first call receives `&PTR_FUN_80502098`. The word at `0x80502098` is
/// `0x80501980`, so the `main.dol` initializer runner ultimately calls
/// [`register_yes_no_states`]. The second call duplicates
/// `FUN_805001d0` (Ghidra emitted a separate thunk symbol for it).
///
/// # Original
/// - Ghidra: `entry` at `0x805000E0` (44 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805000e0_entry.c`
/// - synthetic base: 0x80500000
/// - observed: `func_0x80160a20(&PTR_FUN_80502098); thunk_FUN_805001d0();`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Queues a `main.dol` static-initializer entry and stores
/// `0x80502288` into the global at `0x8042A3B8`.
///
/// # Safety
/// Must run in the REL's initialized image with the `main.dol` allocator and
/// initializer table available.
///
/// # Confidence
/// high - both calls and the table word are observed.
pub unsafe fn entry() {
    // unknown: 0x80160A20 is an initializer-table runner; only the call shape
    // is recovered. 0x80502098 holds 0x80501980 (register_yes_no_states).
    call1(0x8016_0A20, 0x8050_2098);
    thunk_install_profile_descriptor();
}

/// Installs the module's descriptor block at `0x80502288` into the
/// `main.dol` global `0x8042A3B8`.
///
/// # Original
/// - Ghidra: `FUN_805001d0` at `0x805001D0` (20 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805001d0_FUN_805001d0.c`
/// - synthetic base: 0x80500000
/// - observed: `_DAT_8042a3b8 = &DAT_80502288;`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Writes one word at `0x8042A3B8`. The table is a list of `main.dol` function
/// pointers; its consumer is outside this REL.
///
/// # Safety
/// The `main.dol` global must be writable.
///
/// # Confidence
/// high - one observed store.
pub unsafe fn install_profile_descriptor() {
    write_u32(PROFILE_DESCRIPTOR_SLOT, PROFILE_DESCRIPTOR_TABLE);
}

/// Four-byte branch thunk to [`install_profile_descriptor`].
///
/// Ghidra exported this as `thunk_FUN_805001d0` because the linker emitted a
/// separate symbol for the same body. It is kept as a real entry because
/// [`entry`] calls it.
///
/// # Original
/// - Ghidra: `thunk_FUN_805001d0` at `0x805001F0` (4 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805001f0_thunk_FUN_805001d0.c`
/// - synthetic base: 0x80500000
///
/// # Side effects
/// Same as [`install_profile_descriptor`].
///
/// # Safety
/// Same as [`install_profile_descriptor`].
///
/// # Confidence
/// high - the C body is identical to the target function.
pub unsafe fn thunk_install_profile_descriptor() {
    install_profile_descriptor();
}

/// Clears the module descriptor block installed by
/// [`install_profile_descriptor`].
///
/// # Original
/// - Ghidra: `FUN_80500200` at `0x80500200` (16 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500200_FUN_80500200.c`
/// - synthetic base: 0x80500000
/// - observed: `_DAT_8042a3b8 = 0;`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Writes zero to `0x8042A3B8`.
///
/// # Safety
/// The `main.dol` global must be writable.
///
/// # Confidence
/// high - one observed store.
pub unsafe fn clear_profile_descriptor() {
    write_u32(PROFILE_DESCRIPTOR_SLOT, 0);
}

// ---------------------------------------------------------------------------
// State / profile table registration
// ---------------------------------------------------------------------------

/// Pushes one state descriptor onto the global intrusive list.
///
/// The record is three words before the descriptor. The function links the
/// record at the head, stores the cleanup callback and a back-pointer to the
/// descriptor, then publishes the new head.
///
/// # Original
/// - Ghidra: `FUN_80500150` at `0x80500150` (28 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500150_FUN_80500150.c`
/// - synthetic base: 0x80500000
/// - observed: `node[0] = DAT_80503900; node[1] = cleanup; node[2] = descriptor;
///   DAT_80503900 = node;`
///
/// # Arguments
/// - `descriptor`: state descriptor object (`record + 0x10`).
/// - `cleanup`: callback invoked by [`unregister_all_states`] with the
///   descriptor and `0xFFFFFFFF` as its delete flag.
/// - `node`: list record to link.
///
/// # Side effects
/// Writes four words: three record fields and the list head.
///
/// # Safety
/// `node..node+0x0C` and `STATE_LIST_HEAD` must be writable, and
/// `descriptor` must outlive the list entry.
///
/// # Confidence
/// high - all four stores are observed.
pub unsafe fn register_state_descriptor(descriptor: usize, cleanup: usize, node: usize) {
    write_u32(node, read_u32(STATE_LIST_HEAD));
    write_u32(node + 0x04, cleanup as u32);
    write_u32(node + 0x08, descriptor as u32);
    write_u32(STATE_LIST_HEAD, node as u32);
}

/// Pops every state record and calls its cleanup callback with delete flag
/// `-1`.
///
/// The head is advanced *before* the callback so a callback that unlinks itself
/// cannot corrupt the traversal.
///
/// # Original
/// - Ghidra: `FUN_80500170` at `0x80500170` (84 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500170_FUN_80500170.c`
/// - synthetic base: 0x80500000
/// - observed: `while (DAT_80503900 != 0) { next = *head; head = next;
///   (*head[1])(head[2], 0xFFFFFFFF); }`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Empties the list and calls one cleanup callback per record. With
/// [`state_cleanup`] as the callback and a negative delete flag, the objects are
/// not freed.
///
/// # Safety
/// Every list record must follow the layout written by
/// [`register_state_descriptor`].
///
/// # Confidence
/// high - traversal and callback order are observed.
pub unsafe fn unregister_all_states() {
    while read_u32(STATE_LIST_HEAD) != 0 {
        let node = read_u32(STATE_LIST_HEAD) as usize;
        let next = read_u32(node);
        let cleanup = read_u32(node + 0x04) as usize;
        let descriptor = read_u32(node + 0x08);
        write_u32(STATE_LIST_HEAD, next);
        call2(cleanup, descriptor, 0xffff_ffff);
    }
}

/// State-record cleanup callback: runs the base destructor and optionally frees
/// the object.
///
/// The base destructor call is unconditional when the object is non-null; the
/// deallocation wrapper only runs when the delete flag is positive. The
/// function still returns the (possibly freed) pointer, matching the original
/// deleting-destructor convention.
///
/// # Original
/// - Ghidra: `FUN_80501fb0` at `0x80501FB0` (88 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501fb0_FUN_80501fb0.c`
/// - synthetic base: 0x80500000
/// - observed: `func_0x8015f800(param_1, 0); if (0 < param_2)
///   func_0x802b9280(param_1);`
///
/// # Arguments
/// - `param_1`: state descriptor object.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`, even when it was released.
///
/// # Side effects
/// Calls `0x8015F800` (base destructor) and, when `param_2 > 0`, `0x802B9280`
/// (deallocation wrapper).
///
/// # Safety
/// `param_1` must be null or a live state descriptor allocated with the
/// `main.dol` allocator.
///
/// # Confidence
/// high - the two-call destructor shape is observed.
pub unsafe fn state_cleanup(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 {
        call2(0x8015_F800, param_1 as u32, 0);
        if param_2 > 0 {
            call1(0x802B_9280, param_1 as u32);
        }
    }
    param_1
}

/// Builds and registers all eight `dYesNoWindow_c` state descriptors.
///
/// For each state the function:
///
/// 1. loads nine seed words from `0x805035D0 + i*0x24`;
/// 2. constructs a state base with `0x8015F7C0(descriptor, name)`;
/// 3. overwrites the base vtable with `0x80503850`;
/// 4. copies the nine words into the descriptor's three callback triples;
/// 5. pushes the record with [`register_state_descriptor`].
///
/// The callback triples are stored in a different order than the seed table:
/// seed words 6..8 go to `+0x0C`, words 3..5 to `+0x18`, and words 0..2 to
/// `+0x24`. Each triple is `{0, -1, callback}`.
///
/// # Original
/// - Ghidra: `FUN_80501980` at `0x80501980` (1576 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501980_FUN_80501980.c`
/// - synthetic base: 0x80500000
/// - observed: eight repetitions of `func_0x8015f7c0`, nine field stores and
///   `FUN_80500150`; prologue/epilogue are `__savegprlr_27` helpers.
///
/// # Arguments
/// None.
///
/// # Side effects
/// Writes the BSS records at `0x80503908..0x80503B08`, calls the state-base
/// constructor eight times, and links eight records into the global list.
///
/// # Safety
/// The BSS records and list head must be writable; `0x8015F7C0` must be the
/// matching state-base constructor.
///
/// # Confidence
/// high - the unrolled original and the seed table were both read from the
/// relocated image.
pub unsafe fn register_yes_no_states() {
    // observed: 0x802DCD74 = __savegprlr_27 prologue; it saves registers only
    // and has no Rust equivalent. The original ends with __restgprlr_27
    // (0x802DCDC0), likewise omitted.

    // Names are the eight strings referenced by the original; they are not a
    // fixed-stride table (each StateID string has a different length).
    const STATE_NAMES: [usize; 8] = [
        0x8050_36F0, // dYesNoWindow_c::StateID_InitWait
        0x8050_3714, // dYesNoWindow_c::StateID_OpenAnimeEndWait
        0x8050_3740, // dYesNoWindow_c::StateID_ButtonOnStageAnimeEndWait
        0x8050_3774, // dYesNoWindow_c::StateID_ButtonChangeAnimeEndWait
        0x8050_37A8, // dYesNoWindow_c::StateID_SelectWait
        0x8050_37D0, // dYesNoWindow_c::StateID_HitAnimeEndWait
        0x8050_37F8, // dYesNoWindow_c::StateID_ClouseAnimeEndWait (sic)
        0x8050_3824, // dYesNoWindow_c::StateID_HitAnimeAfterWait
    ];

    for (i, name) in STATE_NAMES.iter().enumerate() {
        let node = STATE_RECORD_BASE + i * STATE_RECORD_STRIDE;
        let descriptor = node + 0x10;
        let seed = STATE_SEED_TABLE + i * STATE_SEED_STRIDE;

        let s0 = read_u32(seed);
        let s1 = read_u32(seed + 0x04);
        let s2 = read_u32(seed + 0x08);
        let s3 = read_u32(seed + 0x0c);
        let s4 = read_u32(seed + 0x10);
        let s5 = read_u32(seed + 0x14);
        let s6 = read_u32(seed + 0x18);
        let s7 = read_u32(seed + 0x1c);
        let s8 = read_u32(seed + 0x20);

        // observed: dStateBase_ctor (0x8015F7C0) writes a vtable, stores the
        // name pointer and assigns a state id. This REL replaces the vtable.
        call2(0x8015_F7C0, descriptor as u32, *name as u32);

        write_u32(descriptor, STATE_DESCRIPTOR_VTABLE);
        write_u32(descriptor + 0x0c, s6);
        write_u32(descriptor + 0x10, s7);
        write_u32(descriptor + 0x14, s8);
        write_u32(descriptor + 0x18, s3);
        write_u32(descriptor + 0x1c, s4);
        write_u32(descriptor + 0x20, s5);
        write_u32(descriptor + 0x24, s0);
        write_u32(descriptor + 0x28, s1);
        write_u32(descriptor + 0x2c, s2);

        register_state_descriptor(descriptor, 0x8050_1FB0, node);
    }
}

// ---------------------------------------------------------------------------
// Yes/no window object lifecycle
// ---------------------------------------------------------------------------

/// Constructs the yes/no window object.
///
/// The constructor runs the `main.dol` base constructor, installs the window
/// vtable at `+0x60`, constructs the embedded `lyt` layout at `+0x70`, builds
/// four interface sub-objects at `+0x208..+0x214`, constructs the state manager
/// at `+0x220` with the `InitWait` descriptor, and finally replaces the
/// `+0x208` interface vtable with the state-manager vtable.
///
/// # Original
/// - Ghidra: `FUN_805005f0` at `0x805005F0` (176 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805005f0_FUN_805005f0.c`
/// - synthetic base: 0x80500000
/// - observed: vtable stores `0x80503470`, `0x805034F0`, `0x80503580`,
///   `0x80503520`, `0x80503538`, final `0x805034C0`; call
///   `0x8015FC90(obj+0x220, obj+0x20C, obj+0x210, 0x80503918)`.
/// - inferred: `0x8015FC90` is the state-manager constructor; it writes the
///   vtable at `+0x220` and receives the initial descriptor.
///
/// # Arguments
/// - `param_1`: storage for the object (at least `0x29B` bytes).
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Writes vtables and interface pointers throughout the object, constructs the
/// NW4R layout, and constructs the state manager. The `+0x208` word is written
/// twice: first `0x805034F0`, then `0x805034C0`.
///
/// # Safety
/// `param_1` must point to writable storage large enough for the object and the
/// `main.dol` constructors must be available.
///
/// # Confidence
/// high - every store and call is observed; the class boundary is inferred from
/// the vtable addresses.
pub unsafe fn yes_no_window_ctor(param_1: usize) -> usize {
    // observed: 0x8006C420 takes the object in r3 (main.dol base constructor,
    // confirmed in 8006c420_FUN_8006c420.c). Ghidra printed no argument here.
    call1(0x8006_C420, param_1 as u32);
    write_u32(param_1 + 0x60, 0x8050_3470);
    call1(0x800C_8950, (param_1 + 0x70) as u32);
    write_u32(param_1 + 0x208, 0x8050_34F0);
    write_u32(param_1 + 0x20c, 0x8050_3580);
    write_u32(param_1 + 0x210, 0x8050_3520);
    write_u32(param_1 + 0x214, 0x8050_3538);
    write_u32(param_1 + 0x218, param_1 as u32);
    write_u32(param_1 + 0x21c, 0);

    // inferred: 0x8015FC90 is the state-manager constructor; its extra
    // arguments are the two interface sub-objects and the initial descriptor.
    // The Ghidra main.dol export only proves that it writes a vtable.
    call4(
        0x8015_FC90,
        (param_1 + 0x220) as u32,
        (param_1 + 0x20c) as u32,
        (param_1 + 0x210) as u32,
        0x8050_3918,
    );

    write_u8(param_1 + 0x294, 0);
    write_u32(param_1 + 0x208, 0x8050_34C0);
    param_1
}

/// One of seven identical deleting-destructor stubs at
/// `0x805006A0..0x80500820`.
///
/// The body releases `param_1` only when the object is non-null and the delete
/// flag is positive, then returns the pointer. Ghidra recovered no owning class
/// for these seven slots, so the address name is retained.
///
/// # Original
/// - Ghidra: `FUN_805006a0` at `0x805006A0` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805006a0_FUN_805006a0.c`
/// - synthetic base: 0x80500000
/// - observed: `if ((param_1 != 0) && (0 < param_2)) func_0x802b9280();`
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` (deallocation wrapper) when both conditions hold.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - the free call is observed, the owning class is not.
pub unsafe fn fn_805006a0(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        // observed: Ghidra dropped the argument; the pointer is still in r3.
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Second of seven identical deleting-destructor stubs.
///
/// See [`fn_805006a0`] for the shared body and the unresolved owner.
///
/// # Original
/// - Ghidra: `FUN_805006e0` at `0x805006E0` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805006e0_FUN_805006e0.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` when `param_1 != 0` and `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - identical to [`fn_805006a0`]; owner unresolved.
pub unsafe fn fn_805006e0(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Third of seven identical deleting-destructor stubs.
///
/// # Original
/// - Ghidra: `FUN_80500720` at `0x80500720` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500720_FUN_80500720.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` when `param_1 != 0` and `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - identical to [`fn_805006a0`]; owner unresolved.
pub unsafe fn fn_80500720(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Fourth of seven identical deleting-destructor stubs.
///
/// # Original
/// - Ghidra: `FUN_80500760` at `0x80500760` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500760_FUN_80500760.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` when `param_1 != 0` and `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - identical to [`fn_805006a0`]; owner unresolved.
pub unsafe fn fn_80500760(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Fifth of seven identical deleting-destructor stubs.
///
/// # Original
/// - Ghidra: `FUN_805007a0` at `0x805007A0` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805007a0_FUN_805007a0.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` when `param_1 != 0` and `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - identical to [`fn_805006a0`]; owner unresolved.
pub unsafe fn fn_805007a0(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Sixth of seven identical deleting-destructor stubs.
///
/// # Original
/// - Ghidra: `FUN_805007e0` at `0x805007E0` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805007e0_FUN_805007e0.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` when `param_1 != 0` and `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - identical to [`fn_805006a0`]; owner unresolved.
pub unsafe fn fn_805007e0(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Seventh of seven identical deleting-destructor stubs.
///
/// # Original
/// - Ghidra: `FUN_80500820` at `0x80500820` (64 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500820_FUN_80500820.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x802B9280` when `param_1 != 0` and `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or a heap object from this module.
///
/// # Confidence
/// medium - identical to [`fn_805006a0`]; owner unresolved.
pub unsafe fn fn_80500820(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 && param_2 > 0 {
        call1(0x802B_9280, param_1 as u32);
    }
    param_1
}

/// Deleting destructor that first releases an embedded object at `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_80500860` at `0x80500860` (96 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500860_FUN_80500860.c`
/// - synthetic base: 0x80500000
/// - observed: `func_0x8015f9f0(param_1 + 0x18, 0);` then the usual delete test.
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Calls `0x8015F9F0` for the sub-object at `+0x18`, then `0x802B9280` when
/// `param_2 > 0`.
///
/// # Safety
/// `param_1` must be null or an object whose `+0x18` sub-object is a valid
/// destructible object.
///
/// # Confidence
/// medium - the two-step destructor is observed; the owning class is not.
pub unsafe fn fn_80500860(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 {
        if param_1 + 0x18 != 0 {
            call2(0x8015_F9F0, (param_1 + 0x18) as u32, 0);
        }
        if param_2 > 0 {
            call1(0x802B_9280, param_1 as u32);
        }
    }
    param_1
}

/// Second deleting destructor with an embedded `+0x18` sub-object.
///
/// # Original
/// - Ghidra: `FUN_805008c0` at `0x805008C0` (100 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805008c0_FUN_805008c0.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: object pointer.
/// - `param_2`: delete flag; `> 0` releases the object.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Same two-step destructor as [`fn_80500860`].
///
/// # Safety
/// Same as [`fn_80500860`].
///
/// # Confidence
/// medium - identical body to [`fn_80500860`].
pub unsafe fn fn_805008c0(param_1: usize, param_2: i32) -> usize {
    if param_1 != 0 {
        if param_1 + 0x18 != 0 {
            call2(0x8015_F9F0, (param_1 + 0x18) as u32, 0);
        }
        if param_2 > 0 {
            call1(0x802B_9280, param_1 as u32);
        }
    }
    param_1
}

// ---------------------------------------------------------------------------
// Controller-information overlay
// ---------------------------------------------------------------------------

/// Allocates and constructs a 0x210-byte controller-information object.
///
/// # Original
/// - Ghidra: `FUN_80500210` at `0x80500210` (48 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500210_FUN_80500210.c`
/// - synthetic base: 0x80500000
/// - observed: `iVar1 = func_0x801628c0(0x210); if (iVar1 != 0)
///   FUN_80500240();`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Calls the `main.dol` allocator `0x801628C0` (size `0x210`; it zero-fills the
/// block) and then [`controller_information_ctor`] on the result. The original
/// C omits the constructor argument because Ghidra lost `r3`; the allocator
/// return is the only live value at that point.
///
/// # Safety
/// The `main.dol` heap must be available.
///
/// # Confidence
/// medium - allocation and null test are observed; the argument is inferred
/// from the allocator result and the known constructor signature.
pub unsafe fn create_controller_information() {
    let object = call1(0x8016_28C0, 0x210);
    if object != 0 {
        controller_information_ctor(object as usize);
    }
}

/// Constructs a controller-information object in caller-provided storage.
///
/// # Original
/// - Ghidra: `FUN_80500240` at `0x80500240` (76 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500240_FUN_80500240.c`
/// - synthetic base: 0x80500000
/// - observed: `func_0x8006c420(); *(param_1+0x60) = &DAT_80502f60;
///   func_0x800c8950(param_1+0x70); *(param_1+0x208) = 0;`
///
/// # Arguments
/// - `param_1`: 0x210-byte object storage.
///
/// # Returns
/// `param_1`.
///
/// # Side effects
/// Runs the `main.dol` base constructor, replaces the `+0x60` vtable with
/// `0x80502F60`, constructs the embedded layout at `+0x70`, and clears the byte
/// at `+0x208`.
///
/// # Safety
/// `param_1` must point to writable storage of at least `0x209` bytes.
///
/// # Confidence
/// high - every store is observed; the base constructor argument is confirmed
/// in the `main.dol` export.
pub unsafe fn controller_information_ctor(param_1: usize) -> usize {
    // observed: 0x8006C420 takes the object in r3.
    call1(0x8006_C420, param_1 as u32);
    write_u32(param_1 + 0x60, 0x8050_2F60);
    call1(0x800C_8950, (param_1 + 0x70) as u32);
    write_u8(param_1 + 0x208, 0);
    param_1
}

/// Loads the `ControllerInformation` archive and binds its layout and three
/// animation groups.
///
/// The archive is `ControllerInformation/ControllerInformation.arc`, the layout
/// is `ControllerInformation_07.brlyt`, and the three `brlan` names are held in
/// the pointer arrays at `0x80502EC0`, `0x80502ED8` and `0x80502F08`.
///
/// # Original
/// - Ghidra: `FUN_805003a0` at `0x805003A0` (188 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805003a0_FUN_805003a0.c`
/// - synthetic base: 0x80500000
/// - observed: archive bind, virtual layout build at vtable `+0x14`, then
///   `0x800C8FB0`, `0x800C90F0` and `0x800C8F20` bind calls.
///
/// # Arguments
/// - `param_1`: controller-information object base.
///
/// # Returns
/// `true` when the archive bind succeeded, `false` otherwise.
///
/// # Side effects
/// Loads an archive and constructs/binds layout animations. On failure only the
/// archive probe has run.
///
/// # Safety
/// `param_1` must be a constructed controller-information object and the
/// archive must be present in the game's file system.
///
/// # Confidence
/// high - all call arguments and the early-out are observed. The individual
/// NW4R call semantics are named only at the level the C proves.
pub unsafe fn controller_information_load_layout(param_1: usize) -> bool {
    let layout = param_1 + 0x70;

    // unknown: 0x800C8CB0 is an archive/binding probe; its third argument is 0.
    let loaded = call3(0x800C_8CB0, layout as u32, 0x8050_2F10, 0) != 0;
    if loaded {
        let vtable = read_u32(param_1 + 0x78) as usize;
        let build = read_u32(vtable + 0x14) as usize;
        // observed: virtual slot +0x14 builds the layout from the .brlyt name.
        call3(build, layout as u32, 0x8050_2F40, 0);

        // Three .brlan names: in2btn, loop2btn, hit2btn.
        call3(0x800C_8FB0, layout as u32, 0x8050_2EC0, 3);
        // Three pane names (A00_2btn_00) plus the table at 0x805020B0.
        call4(
            0x800C_90F0,
            layout as u32,
            0x8050_2ED8,
            0x8050_20B0,
            3,
        );
        // Two pane names (T_contInfo_00/01), table at 0x805020C0, count 2.
        call5(
            0x800C_8F20,
            layout as u32,
            0x8050_2F08,
            0x8050_20C0,
            0x68,
            2,
        );
    }
    loaded
}

/// Refreshes the controller hint: pane visibility, button animation ids and
/// the current focused button, all indexed by the profile/question index.
///
/// Two paths exist. Profile index `6` binds the special three-word animation
/// table at `0x80503384` and runs the three hit animations at frames `7`, `7`
/// and `8`. Every other index selects one frame id from `0x80502138`, with a
/// special frame `0x38` for index `0x14` when the two hint-variant bytes are
/// `7` and `0x17`.
///
/// # Original
/// - Ghidra: `FUN_80500c70` at `0x80500C70` (876 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80500c70_FUN_80500c70.c`
/// - synthetic base: 0x80500000
/// - observed: table reads at `0x80502138`, `0x805021AC`, `0x80502204`; pane
///   flag updates at `+0xBB`; calls to `0x800C9A60`, `0x800B3B60`,
///   `0x80195380` and `0x800CDC60`.
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Writes pane flag bytes, the focused-button word at `+0x284`, an unnamed
/// global word for profile index `0x0E`, and starts layout animations.
///
/// # Safety
/// `param_1` must be a constructed yes/no window and every pane pointer at
/// `+0x244..+0x280` must be valid.
///
/// # Confidence
/// medium - the control flow and table addresses are exact, but the meaning of
/// the table values and of the NW4R animation calls is inferred.
pub unsafe fn refresh_controller_hint(param_1: usize) {
    // unknown: 0x800CDC60 returns the global word at 0x80359CB4, apparently a
    // frame/clock value. The name is not proven. `call1` is used only because
    // `platform::call0` discards the return value; the extra register is ignored
    // by a no-argument function.
    let frame_clock = call1(0x800C_DC60, 0);
    let index = read_i32(param_1 + 0x28c);
    let mut frame_id = read_u32(PROFILE_ANIM_IDS + index as usize * 4);

    if index == 6 {
        // observed: special binding table whose first word is 2.
        call4(
            0x800B_3B60,
            (param_1 + 0x290) as u32,
            CONTROLLER_HIT_TABLE as u32,
            read_u32(param_1 + 0x268),
            1,
        );
        // unknown: 0x800C9A60 starts one layout animation at a frame id.
        call5(
            0x800C_9A60,
            read_u32(param_1 + 0x25c),
            frame_clock,
            0x12e,
            7,
            0,
        );
        call5(
            0x800C_9A60,
            read_u32(param_1 + 0x260),
            frame_clock,
            0x12e,
            7,
            0,
        );
        call5(
            0x800C_9A60,
            read_u32(param_1 + 0x264),
            frame_clock,
            0x12e,
            8,
            0,
        );
        set_pane_bit0(param_1, 0x254, false);
        set_pane_bit0(param_1, 0x27c, true);
    } else {
        if index == 0x0e {
            // observed: *(u32 *)(0x80429F7C + 0x3B4) = DAT_8042A24D + 1.
            // unknown: Ghidra prints DAT_8042A24D without a width; the address
            // is unaligned, so this reconstruction reads one byte.
            write_u32(0x8042_9F7C + 0x3b4, read_u8(0x8042_A24D) as u32 + 1);
        }
        if index == 0x14
            && read_u8(HINT_VARIANT_A) == 7
            && read_u8(HINT_VARIANT_B) == 0x17
        {
            frame_id = 0x38;
        }

        set_pane_bit0(param_1, 0x254, true);
        set_pane_bit0(param_1, 0x27c, false);
        call5(
            0x800C_9A60,
            read_u32(param_1 + 0x254),
            frame_clock,
            0,
            frame_id,
            0,
        );
        call5(
            0x800C_9A60,
            read_u32(param_1 + 0x258),
            frame_clock,
            0,
            frame_id,
            0,
        );
    }

    // Three button panes, one visibility byte each in the 29 x 3 table.
    let flags = PROFILE_PANE_FLAGS + index as usize * 3;
    set_pane_bit0(param_1, 0x248, read_u8(flags) != 0);
    set_pane_bit0(param_1, 0x24c, read_u8(flags + 1) != 0);
    set_pane_bit0(param_1, 0x250, read_u8(flags + 2) != 0);

    // The focused-button table is read through a fresh load of +0x28C, exactly
    // as the original does after the animation branch.
    let index = read_i32(param_1 + 0x28c);
    write_u32(
        param_1 + 0x284,
        read_u32(PROFILE_DEFAULT_BUTTON + index as usize * 4),
    );

    let (primary, secondary) = match index {
        1 => (0x2a, 0x2b),
        0x0f => (0x2d, 0x2b),
        0x13 => (0x26, 0x27),
        0x14 => {
            let mut primary = 0x24;
            if read_u8(HINT_VARIANT_A) == 7 && read_u8(HINT_VARIANT_B) == 0x17 {
                primary = 0x23;
            }
            (primary, 0x25)
        }
        _ => {
            let primary = if index == 4 || index == 6 { 0x2c } else { 0x28 };
            (primary, 0x29)
        }
    };

    call5(
        0x800C_9A60,
        read_u32(param_1 + 0x26c),
        frame_clock,
        0,
        primary,
        0,
    );
    call5(
        0x800C_9A60,
        read_u32(param_1 + 0x270),
        frame_clock,
        0,
        primary,
        0,
    );
    call5(
        0x800C_9A60,
        read_u32(param_1 + 0x274),
        frame_clock,
        0,
        secondary,
        0,
    );
    call5(
        0x800C_9A60,
        read_u32(param_1 + 0x278),
        frame_clock,
        0,
        secondary,
        0,
    );

    set_pane_bit0(param_1, 0x280, index == 0x10);
}

// ---------------------------------------------------------------------------
// Descriptor lookup and state callbacks
// ---------------------------------------------------------------------------

/// `OpenAnimeEndWait` enter callback: stop the previous animations, reset the
/// button panes and start the open sequence.
///
/// # Original
/// - Ghidra: `FUN_80501050` at `0x80501050` (268 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501050_FUN_80501050.c`
/// - synthetic base: 0x80500000
/// - observed: clears `+0x299`, shows pane `+0x244`, sets `+0x297`, stops
///   animations 4/5/6, plays optional animation `0x14`, plays animation `0`,
///   emits event `0x7F`, then calls [`refresh_controller_hint`].
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Writes flags and pane visibility, calls NW4R animation helpers, emits one
/// event through `0x80195380`, and refreshes the controller hint.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the switch cases and call arguments are observed.
pub unsafe fn state_enter_open_wait(param_1: usize) {
    write_u8(param_1 + 0x299, 0);
    set_pane_bit0(param_1, 0x244, true);
    write_u8(param_1 + 0x297, 1);

    // unknown: 0x800C9500 resets every layout animation in the embedded layout.
    call1(0x800C_9500, (param_1 + 0x70) as u32);
    call3(0x800C_93D0, (param_1 + 0x70) as u32, 4, 0);
    call3(0x800C_93D0, (param_1 + 0x70) as u32, 5, 0);
    call3(0x800C_93D0, (param_1 + 0x70) as u32, 6, 0);

    let index = read_u32(param_1 + 0x28c);
    if matches!(
        index,
        1 | 2 | 3 | 4 | 6 | 10 | 0x0f | 0x10 | 0x12 | 0x19 | 0x1b | 0x1c
    ) {
        if read_u8(param_1 + 0x29a) == 0 {
            call3(0x800C_92F0, (param_1 + 0x70) as u32, 0x14, 0);
        }
    }
    if index == 0x10 {
        call3(0x800C_92F0, (param_1 + 0x70) as u32, 1, 0);
    }
    call3(0x800C_92F0, (param_1 + 0x70) as u32, 0, 0);

    // unknown: 0x80195380(manager, id, flag) is an event/sound cue helper.
    call3(0x8019_5380, read_u32(EVENT_MANAGER), 0x7f, 1);
    refresh_controller_hint(param_1);
}

/// `OpenAnimeEndWait` poll callback: wait for the open animation to finish and
/// advance to the button-on-stage descriptor.
///
/// # Original
/// - Ghidra: `FUN_80501160` at `0x80501160` (92 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501160_FUN_80501160.c`
/// - synthetic base: 0x80500000
/// - observed: `if (func_0x800c9640(layout) == 0) { +0x296 = 0;
///   vtable[+0x18](+0x208, 0x80503998); }`
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Clears `+0x296` and changes state to `0x80503998`
/// (`ButtonOnStageAnimeEndWait`).
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the animation query and descriptor address are observed.
pub unsafe fn state_poll_open_wait(param_1: usize) {
    // unknown: 0x800C9640 returns non-zero while any layout animation runs.
    if call1(0x800C_9640, (param_1 + 0x70) as u32) == 0 {
        write_u8(param_1 + 0x296, 0);
        switch_state(param_1, 0x8050_3998);
    }
}

/// `ButtonOnStageAnimeEndWait` enter callback: start animations 4, 5 and 6.
///
/// # Original
/// - Ghidra: `FUN_805011d0` at `0x805011D0` (96 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805011d0_FUN_805011d0.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Sets `+0x297` and starts three layout animations.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - three calls and one flag store are observed.
pub unsafe fn state_enter_button_on(param_1: usize) {
    write_u8(param_1 + 0x297, 1);
    call3(0x800C_92F0, (param_1 + 0x70) as u32, 4, 0);
    call3(0x800C_92F0, (param_1 + 0x70) as u32, 5, 0);
    call3(0x800C_92F0, (param_1 + 0x70) as u32, 6, 0);
}

/// `ButtonOnStageAnimeEndWait` poll callback: advance to button-change.
///
/// # Original
/// - Ghidra: `FUN_80501230` at `0x80501230` (84 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501230_FUN_80501230.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Changes state to `0x805039D8` (`ButtonChangeAnimeEndWait`) when the layout
/// animation query returns zero.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the descriptor address is observed.
pub unsafe fn state_poll_button_on(param_1: usize) {
    if call1(0x800C_9640, (param_1 + 0x70) as u32) == 0 {
        switch_state(param_1, 0x8050_39D8);
    }
}

/// `ButtonChangeAnimeEndWait` enter callback: play the previous-button
/// animation, then the newly focused button animation.
///
/// # Original
/// - Ghidra: `FUN_805012b0` at `0x805012B0` (108 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805012b0_FUN_805012b0.c`
/// - synthetic base: 0x80500000
/// - observed: if `+0x288 >= 0`, play `+0x288 + 0x11`; then copy
///   `+0x288 = +0x284` and play `+0x284 + 7`.
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Stops all layout animations, plays up to two button animations, updates the
/// previous-button word.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the signed test and both animation ids are observed.
pub unsafe fn state_enter_button_change(param_1: usize) {
    write_u8(param_1 + 0x297, 1);
    call1(0x800C_9500, (param_1 + 0x70) as u32);

    let previous = read_i32(param_1 + 0x288);
    if previous >= 0 {
        call3(
            0x800C_92F0,
            (param_1 + 0x70) as u32,
            previous as u32 + 0x11,
            0,
        );
    }

    let current = read_i32(param_1 + 0x284);
    write_u32(param_1 + 0x288, current as u32);
    call3(
        0x800C_92F0,
        (param_1 + 0x70) as u32,
        current as u32 + 7,
        0,
    );
}

/// `ButtonChangeAnimeEndWait` poll callback: advance to select-wait.
///
/// # Original
/// - Ghidra: `FUN_80501320` at `0x80501320` (88 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501320_FUN_80501320.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Changes state to `0x80503A18` (`SelectWait`) when the current animation flag
/// at layout index `-1` is zero.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the descriptor address and index `-1` are observed.
pub unsafe fn state_poll_button_change(param_1: usize) {
    if call2(0x800C_9610, (param_1 + 0x70) as u32, 0xffff_ffff) == 0 {
        switch_state(param_1, 0x8050_3A18);
    }
}

/// `SelectWait` enter callback: start the idle button animation and hide the
/// currently focused button pane.
///
/// # Original
/// - Ghidra: `FUN_80501390` at `0x80501390` (104 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501390_FUN_80501390.c`
/// - synthetic base: 0x80500000
/// - observed: index `0x10` plays animation 3 before animation 2; then
///   `func_0x800b44d0(pane[+0x284], 0, 0)`.
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Clears animation flags through `0x800C9380`, then hides one of the three
/// button panes through `0x800B44D0`.
///
/// # Safety
/// `param_1` must be a constructed yes/no window and `+0x284` must be `0..2`.
///
/// # Confidence
/// high - the conditional animation id and the pane table index are observed.
pub unsafe fn state_enter_select(param_1: usize) {
    if read_u32(param_1 + 0x28c) == 0x10 {
        // unknown: 0x800C9380 clears one layout animation flag.
        call2(0x800C_9380, (param_1 + 0x70) as u32, 3);
    }
    call2(0x800C_9380, (param_1 + 0x70) as u32, 2);

    let button = read_i32(param_1 + 0x284) as usize;
    let pane = read_u32(param_1 + button * 4 + 0x248);
    // unknown: 0x800B44D0(pane, 0, 0) hides/deactivates the pane; exact
    // semantics live in main.dol.
    call3(0x800B_44D0, pane, 0, 0);
}

/// `SelectWait` poll callback: react to confirm/cancel and choose the next
/// descriptor.
///
/// Three branches are observed; the confirm/cancel labels are inferred from
/// the destination descriptors:
///
/// * no decision: if the focused button changed, emit event `0x78` and go back
///   to button-change;
/// * confirm-like: optionally call `0x8006A990` for profiles `0x0C`, `0x1A` and
///   `0x0D` with button `0`, then go to hit-anime;
/// * cancel-like: stop animations (profile `0x10` only) and go to close-anime.
///
/// # Original
/// - Ghidra: `FUN_80501400` at `0x80501400` (276 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501400_FUN_80501400.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Emits event `0x78`, calls `0x8006A990` on one path, stops animations on
/// another, and changes state to one of three descriptors.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the branch conditions and all three descriptor addresses are
/// observed.
pub unsafe fn state_poll_select(param_1: usize) {
    if read_u8(param_1 + 0x299) == 0 {
        if read_u8(param_1 + 0x296) == 0 {
            if read_i32(param_1 + 0x288) != read_i32(param_1 + 0x284) {
                call3(0x8019_5380, read_u32(EVENT_MANAGER), 0x78, 1);
                switch_state(param_1, 0x8050_39D8);
            }
        } else {
            let index = read_u32(param_1 + 0x28c);
            if (index == 0x0c || index == 0x1a || index == 0x0d)
                && read_u32(param_1 + 0x284) == 0
            {
                // unknown: 0x8006A990 takes no arguments in the C export.
                call0(0x8006_A990);
            }
            switch_state(param_1, 0x8050_3A58);
        }
    } else {
        if read_u32(param_1 + 0x28c) == 0x10 {
            call1(0x800C_9500, (param_1 + 0x70) as u32);
            call3(0x800C_92F0, (param_1 + 0x70) as u32, 0x10, 0);
        }
        switch_state(param_1, 0x8050_3A98);
    }
}

/// `HitAnimeEndWait` enter callback: start the button-hit animation.
///
/// # Original
/// - Ghidra: `FUN_80501530` at `0x80501530` (132 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501530_FUN_80501530.c`
/// - synthetic base: 0x80500000
/// - observed: stop animations, set `+0x297 = 1`, clear `+0x296`, play
///   animation `0x10` for profile `0x10`, then play `+0x284 + 0x0D`; emit
///   event `0x79`.
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Stops and starts layout animations and emits event `0x79`.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - all flags and ids are observed.
pub unsafe fn state_enter_hit(param_1: usize) {
    call1(0x800C_9500, (param_1 + 0x70) as u32);
    write_u8(param_1 + 0x297, 1);
    write_u8(param_1 + 0x296, 0);
    if read_u32(param_1 + 0x28c) == 0x10 {
        call3(0x800C_92F0, (param_1 + 0x70) as u32, 0x10, 0);
    }
    let button = read_i32(param_1 + 0x284);
    call3(
        0x800C_92F0,
        (param_1 + 0x70) as u32,
        button as u32 + 0x0d,
        0,
    );
    call3(0x8019_5380, read_u32(EVENT_MANAGER), 0x79, 1);
}

/// `HitAnimeEndWait` poll callback: wait for the hit animation and choose
/// close or hit-after.
///
/// # Original
/// - Ghidra: `FUN_805015c0` at `0x805015C0` (132 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/805015c0_FUN_805015c0.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Changes state to `0x80503A98` (`ClouseAnimeEndWait`) when `+0x298` is zero,
/// otherwise to `0x80503AD8` (`HitAnimeAfterWait`).
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - both descriptor addresses and the flag test are observed.
pub unsafe fn state_poll_hit(param_1: usize) {
    if call2(0x800C_9610, (param_1 + 0x70) as u32, 0xffff_ffff) == 0 {
        let next = if read_u8(param_1 + 0x298) == 0 {
            0x8050_3A98
        } else {
            0x8050_3AD8
        };
        switch_state(param_1, next);
    }
}

/// `ClouseAnimeEndWait` enter callback: start the close animation.
///
/// (The misspelling `Clouse` is in the original state string.)
///
/// # Original
/// - Ghidra: `FUN_80501660` at `0x80501660` (156 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501660_FUN_80501660.c`
/// - synthetic base: 0x80500000
/// - observed: set `+0x297 = 1`, stop animations, optional animation `0x15`
///   for the same 12 profile indices, event `0x7A`, then animation `0x16`.
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Writes a flag, stops/starts layout animations and emits event `0x7A`.
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the switch list, event id and final animation id are observed.
pub unsafe fn state_enter_close(param_1: usize) {
    write_u8(param_1 + 0x297, 1);
    call1(0x800C_9500, (param_1 + 0x70) as u32);

    let index = read_u32(param_1 + 0x28c);
    if matches!(
        index,
        1 | 2 | 3 | 4 | 6 | 10 | 0x0f | 0x10 | 0x12 | 0x19 | 0x1b | 0x1c
    ) {
        if read_u8(param_1 + 0x29a) == 0 {
            call3(0x800C_92F0, (param_1 + 0x70) as u32, 0x15, 0);
        }
    }
    call3(0x8019_5380, read_u32(EVENT_MANAGER), 0x7a, 1);
    call3(0x800C_92F0, (param_1 + 0x70) as u32, 0x16, 0);
}

/// `ClouseAnimeEndWait` poll callback: finish the dialog and return to
/// `InitWait`.
///
/// # Original
/// - Ghidra: `FUN_80501700` at `0x80501700` (96 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501700_FUN_80501700.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: yes/no window object base.
///
/// # Side effects
/// Clears `+0x295` and changes state to `0x80503918` (`InitWait`).
///
/// # Safety
/// `param_1` must be a constructed yes/no window.
///
/// # Confidence
/// high - the flag store and descriptor address are observed.
pub unsafe fn state_poll_close(param_1: usize) {
    if call2(0x800C_9610, (param_1 + 0x70) as u32, 0xffff_ffff) == 0 {
        write_u8(param_1 + 0x295, 0);
        switch_state(param_1, 0x8050_3918);
    }
}

// ---------------------------------------------------------------------------
// Virtual-forwarding thunks
// ---------------------------------------------------------------------------

/// Vtable forwarder at `0x80503850 + 0x2C` with hidden `this` adjustment
/// `+0x18`.
///
/// The assembly loads `r12 = self + 0x18` before the branch and passes `r3 =
/// arg`, `r4 = arg`, `r5 = self`. Ghidra shows the three visible arguments
/// only; the `r12` adjustment is the fourth, hidden operand.
///
/// # Original
/// - Ghidra: `FUN_80501920` at `0x80501920` (48 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501920_FUN_80501920.c`
/// - synthetic base: 0x80500000
/// - observed: `func_0x802dcbfc(param_2, param_2, param_1);`
///
/// # Arguments
/// - `param_1`: state object base.
/// - `param_2`: forwarded argument.
///
/// # Side effects
/// Calls the unresolved indirect-jump stub `0x802DCBFC`.
///
/// # Safety
/// The `+0x18` sub-object must be a valid object for the called vtable slot.
///
/// # Confidence
/// high - `mr r5,r3`, `mr r3,r4`, `addi r12,r5,0x18` and `bl 0x802DCBFC`
/// are the entire body; `interface_call` builds the `+0x18` descriptor
/// from `param_1` and forwards `param_2` as the receiver.
pub unsafe fn fn_80501920(param_1: usize, param_2: u32) {
    // observed: `mr r5,r3`, `mr r3,r4`, `addi r12,r5,0x18`, `bl 0x802DCBFC`.
    interface_call(param_1 as usize, param_2, 0x18);
}

/// Vtable forwarder at `0x80503850 + 0x28` with hidden `this` adjustment
/// `+0x0C`.
///
/// Identical to [`fn_80501920`] except that `r12` is `self + 0x0C`.
///
/// # Original
/// - Ghidra: `FUN_80501950` at `0x80501950` (48 bytes)
/// - Evidence: `work/ghidra/rel-d_profileNP/export/80501950_FUN_80501950.c`
/// - synthetic base: 0x80500000
///
/// # Arguments
/// - `param_1`: state object base.
/// - `param_2`: forwarded argument.
///
/// # Side effects
/// Calls the unresolved indirect-jump stub `0x802DCBFC`.
///
/// # Safety
/// The `+0x0C` sub-object must be a valid object for the called vtable slot.
///
/// # Confidence
/// high - `mr r5,r3`, `mr r3,r4`, `addi r12,r5,0xc` and `bl 0x802DCBFC`
/// are the entire body; `interface_call` builds the `+0xc` descriptor
/// from `param_1` and forwards `param_2` as the receiver.
pub unsafe fn fn_80501950(param_1: usize, param_2: u32) {
    // observed: `mr r5,r3`, `mr r3,r4`, `addi r12,r5,0xc`, `bl 0x802DCBFC`.
    interface_call(param_1 as usize, param_2, 0xc);
}
