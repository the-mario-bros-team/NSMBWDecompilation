//! Yoshi actor, tongue interaction and state descriptors.
//!
//! # What lives here
//!
//! This is the semantic home for Yoshi-specific gameplay: the `daYoshi_c`
//! actor, its tongue interaction with the world map, and the per-state
//! descriptors registered by the actor initialiser.
//!
//! Seven helper bodies are translated here, all from the `daYoshi_c` band:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80152480` | [`fn_80152480`] | copy the owner's 12-word interaction state block |
//! | `0x80152520` | [`fn_80152520`] | forward a 0x10-byte query to the owner's virtual `+0x50` slot |
//! | `0x801525A0` | [`fn_801525A0`] | start/stop the secondary target animation and notify the primary target |
//! | `0x80152750` | [`fn_80152750`] | forward a query to the secondary target's virtual `+0x3EC` slot |
//! | `0x80152880` | [`fn_80152880`] | spawn `Wm_mr_ystonguehit` once and play sound `0x159` |
//! | `0x80154550` | [`fn_80154550`] | five-level `+0x30` chain walk with tail recursion |
//! | `0x80154630` | [`fn_80154630`] | compare the last `':'`-separated name component |
//!
//! # Original address ranges
//!
//! | band | addresses | entries | what it contains |
//! |---|---|---|---|
//! | E | `0x80151320..0x80152CF0` | 20 (18 eligible) | Yoshi tongue interaction (`Wm_mr_ystonguehit`) and small geometry accessors |
//! | F | `0x801533A0..0x80154A70` | 21 (10 eligible) | `daYoshi_c`; the 3,832-byte initialiser `0x80153400` registers 19 `daYoshi_c::StateID_*` descriptors |
//!
//! Both bands belong to the address lane `0x80140000..0x80180000`, documented
//! in `docs/COVERAGE_r80140_80180.md`. The seven bodies below stay inside the
//! low band; the state-descriptor initialiser `0x80153400` is still
//! untranslated.
//!
//! # Why it exists
//!
//! Yoshi is not just another enemy: the tongue is a world-map interaction that
//! has to agree with the pick-up/release object in
//! [`crate::game::interaction`], and the 19 state descriptors are the actor's
//! animation/behaviour table. Grouping that evidence here keeps the actor
//! semantics out of the range-staging file.
//!
//! # Object layout used here
//!
//! Offsets are relative to the `daYoshi_c` object unless stated otherwise.
//!
//! | offset | width | meaning | confidence |
//! |---|---|---|---|
//! | `+0x030` | word | child link used by the chain walk | observed |
//! | `+0x060` | word | actor virtual base (vtable pointer) | observed |
//! | `+0x348` | byte | player/variant selector; `1` flips the tongue effect | observed |
//! | `+0x38C` | byte | "ready" state checked before a notification | inferred |
//! | `+0xDEC` | word | embedded sound/effect interface; slot `+0x24` plays a sound | observed |
//! | `+0x14D8` | word | owner/controller interface; slot `+0x50` fills a state buffer | observed |
//! | `+0x1520` | word | primary target handle used by the registry lookup | observed |
//! | `+0x1524` | word | secondary target handle used by the tongue path | observed |
//! | `+0x17BC` | word | "tongue effect already spawned" latch | observed |
//!
//! # Dependencies
//!
//! [`crate::platform`] for volatile loads/stores and indirect calls. The
//! registry lookup `0x80162D00` is the same function documented in
//! [`crate::game::interaction`]; the remaining callees are called by address
//! because their reconstructed homes are other lanes.
//!
//! # Start here
//!
//! [`fn_80152880`] is the visible tongue effect: one latch, one effect spawn
//! and one sound id. [`fn_80154550`] is the smallest complete control-flow
//! example of the `+0x30` chain family; compare it with `0x80040720` in
//! [`crate::runtime::cpp`].
//!
//! # Glossary
//!
//! * **actor** - a game object. Its first word is a vtable pointer; `+0x60` is
//!   the virtual base used by the actor system.
//! * **world map (Wm)** - the overworld/level-select map; evidence strings are
//!   prefixed `Wm_`.
//! * **state descriptor** - a small record that names one actor state and its
//!   callbacks; `daYoshi_c` registers 19 of them.
//! * **DOL** - the Wii executable container (`sys/main.dol`).
//!
//! # Confidence summary
//!
//! Six bodies are `high`; [`fn_801525A0`] is `medium` because the owner's
//! virtual `+0x3E0` slot and the `+0x38C` byte are only described by their call
//! shape. All stores and calls are explicit in the evidence.

use crate::game::interaction::wm_secondary_target;
use crate::platform;
use crate::platform::{call1, call2, call3, call5};

// ---------------------------------------------------------------------------
// Cross-lane callees (opaque here)
// ---------------------------------------------------------------------------

/// `0x80162D00`: registry lookup from a 32-bit handle to a live actor.
///
/// The same entry is used as `ACTOR_LOOKUP` in [`crate::game::interaction`].
const ACTOR_LOOKUP: usize = 0x8016_2D00;

/// `0x80145BD0`: changes the actor's long animation: `(actor, mode, anim_id)`.
///
/// `mode == 0` stops the animation; a non-zero `anim_id` selects a new one.
const FN_80145BD0: usize = 0x8014_5BD0;

/// `0x8016C890`: spawns a named effect at a position:
/// `(name, a, b, position, c)`.
const FN_8016C890: usize = 0x8016_C890;

/// `0x800FF680`: asks the owner interface for its 12-word state block:
/// `(out, owner)`.
const FN_800FF680: usize = 0x800F_F680;

/// `0x802E1C40`: C `strrchr` - last occurrence of a byte in a C string.
const FN_802E1C40: usize = 0x802E_1C40;

/// `0x802E1AB4`: C `strcmp` - 0 when two C strings are equal.
const FN_802E1AB4: usize = 0x802E_1AB4;

/// `0x80326780`: the observed string `"Wm_mr_ystonguehit"`.
const S_WM_MR_YSTONGUEHIT: usize = 0x8032_6780;

// ---------------------------------------------------------------------------
// Shared memory and ABI helpers
// ---------------------------------------------------------------------------

/// Reads the big-endian word at `base + offset`.
#[inline]
unsafe fn read_word(base: usize, offset: usize) -> u32 {
    platform::read_u32(base + offset)
}

/// Writes the big-endian word `value` at `base + offset`.
#[inline]
unsafe fn write_word(base: usize, offset: usize, value: u32) {
    platform::write_u32(base + offset, value);
}

/// Reads the byte at `base + offset`.
#[inline]
unsafe fn read_byte(base: usize, offset: usize) -> u8 {
    platform::read_u8(base + offset)
}

/// Writes the byte `value` at `base + offset`.
#[inline]
unsafe fn write_byte(base: usize, offset: usize, value: u8) {
    platform::write_u8(base + offset, value);
}

/// Calls virtual slot `slot` of an object whose vtable pointer is its first
/// word.
#[inline]
unsafe fn vcall0(this_arg: usize, slot: usize) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    call1(entry, this_arg as u32)
}

/// [`vcall0`] with one integer argument.
#[inline]
unsafe fn vcall1(this_arg: usize, slot: usize, arg: u32) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    call2(entry, this_arg as u32, arg)
}

/// Calls virtual slot `slot` of the actor virtual base stored at
/// `object + field`, with one integer argument.
#[inline]
unsafe fn member_vcall1(object: usize, field: usize, slot: usize, arg: u32) -> u32 {
    let vtable = read_word(object, field) as usize;
    let entry = read_word(vtable, slot) as usize;
    call2(entry, object as u32, arg)
}

// ===========================================================================
// 1. Owner-interface forwarding helpers
// ===========================================================================

/// Copies the owner's 12-word interaction state block into caller storage.
///
/// The block is produced by `0x800FF680(&block, owner)`, which forwards to the
/// owner's virtual slot `+0x50` with the block address and selector `0x17`.
///
/// # Original
/// - Ghidra: `FUN_80152480` at `0x80152480` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152480_FUN_80152480.c`
///
/// # Arguments
/// * `this` - Yoshi actor; the owner pointer is at `+0x14D8`.
/// * `out` - writable 12-word destination.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Calls `0x800FF680`; writes 12 words to `out`.
///
/// # Safety
/// `this+0x14D8` must point at a live interface and `out` must be writable for
/// 48 bytes.
///
/// # Confidence
/// high - the buffer size and the word copy are explicit.
pub unsafe fn fn_80152480(this: usize, out: usize) -> u32 {
    // observed: the local block is 12 words (0x30 bytes).
    let mut block = [0u8; 48];
    let owner = read_word(this, 0x14D8);
    let _ = call2(FN_800FF680, block.as_mut_ptr() as u32, owner);

    let mut i = 0;
    while i < 12 {
        let word = platform::read_u32(block.as_ptr() as usize + i * 4);
        platform::write_u32(out + i * 4, word);
        i += 1;
    }

    1
}

/// Forwards a 0x10-byte query to the owner interface's virtual `+0x50` slot.
///
/// # Original
/// - Ghidra: `FUN_80152520` at `0x80152520` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152520_FUN_80152520.c`
///
/// # Arguments
/// * `this` - Yoshi actor; the owner pointer is at `+0x14D8`.
/// * `param_2` - destination buffer forwarded to the owner.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// One virtual call `owner->+0x50(owner, param_2, 0x10)`.
///
/// # Safety
/// `this+0x14D8` must hold a live object with a vtable at offset 0 and a
/// function at `+0x50`; `param_2` must satisfy that function.
///
/// # Confidence
/// high - the slot, selector and return value are explicit.
pub unsafe fn fn_80152520(this: usize, param_2: u32) -> u32 {
    let owner = read_word(this, 0x14D8) as usize;
    let vtable = read_word(owner, 0) as usize;
    let entry = read_word(vtable, 0x50) as usize;
    let _ = call3(entry, owner as u32, param_2, 0x10);
    1
}

// ===========================================================================
// 2. Secondary-target forwarding
// ===========================================================================

/// Updates the secondary target animation and notifies the primary target.
///
/// When a secondary target exists, its animation is changed with
/// `0x80145BD0(secondary, param_2, param_3)`. Then, only when `param_2 == 0`,
/// the primary target is looked up and, if its `+0x38C` byte is `1`, the Yoshi
/// actor's virtual `+0x3E0` slot is called with `(this, primary_target)`.
///
/// # Original
/// - Ghidra: `FUN_801525A0` at `0x801525A0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801525a0_FUN_801525a0.c`
///
/// # Arguments
/// * `this` - Yoshi actor.
/// * `param_2` - animation stop/start selector for `0x80145BD0`.
/// * `param_3` - animation id forwarded to `0x80145BD0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80145BD0`, the registry `0x80162D00`, and the actor's virtual
/// `+0x3E0` slot.
///
/// # Safety
/// `this` must be live; non-zero target handles must resolve to live actors.
///
/// # Confidence
/// medium - calls and stores are exact; the meaning of the `+0x38C` byte and
/// the `+0x3E0` slot is inferred from the call shape.
pub unsafe fn fn_801525a0(this: usize, param_2: u32, param_3: u32) {
    let secondary = wm_secondary_target(this);
    if secondary != 0 {
        let _ = call3(FN_80145BD0, secondary, param_2, param_3);
    }

    if param_2 != 0 {
        return;
    }

    let primary = call1(ACTOR_LOOKUP, read_word(this, 0x1520)) as usize;
    if primary == 0 {
        return;
    }
    if read_byte(primary, 0x38C) != 1 {
        return;
    }

    // observed: `(**(code **)(*(int *)(this + 0x60) + 0x3E0))(this, primary)`.
    let _ = member_vcall1(this, 0x60, 0x3E0, primary as u32);
}

/// Forwards a query to the secondary target's virtual `+0x3EC` slot.
///
/// # Original
/// - Ghidra: `FUN_80152750` at `0x80152750` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152750_FUN_80152750.c`
///
/// # Arguments
/// * `this` - Yoshi actor.
/// * `param_2` - forwarded to the secondary target.
///
/// # Returns
/// The secondary target's virtual result, or `0` when there is no target.
///
/// # Side effects
/// One registry lookup and one virtual call.
///
/// # Safety
/// A non-zero `+0x1524` must resolve to a live actor with a vtable at `+0x60`
/// and a function at `+0x3EC`.
///
/// # Confidence
/// high - both calls and the null guard are explicit.
pub unsafe fn fn_80152750(this: usize, param_2: u32) -> u32 {
    let secondary = wm_secondary_target(this) as usize;
    if secondary == 0 {
        return 0;
    }
    member_vcall1(secondary, 0x60, 0x3EC, param_2)
}

// ===========================================================================
// 3. Tongue effect and name helpers
// ===========================================================================

/// Spawns the `Wm_mr_ystonguehit` effect once and plays sound `0x159`.
///
/// The latch at `+0x17BC` is set on the first call; later calls do nothing.
/// The effect position is three zero halfwords, except that the middle
/// component is `0x8000` when the player/variant byte at `+0x348` is `1`.
///
/// # Original
/// - Ghidra: `FUN_80152880` at `0x80152880` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152880_FUN_80152880.c`
///
/// # Arguments
/// * `this` - Yoshi actor.
/// * `param_2` - position/owner operand forwarded to `0x8016C890`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets `+0x17BC`, calls `0x8016C890("Wm_mr_ystonguehit", 0, param_2,
/// &position, 0)`, and plays sound id `0x159` through the `+0xDEC` interface.
///
/// # Safety
/// `this` must be live; `+0xDEC` must hold a sound interface with a vtable.
///
/// # Confidence
/// high - the string address, latch and sound id are explicit; the three
/// halfwords are copied as observed.
pub unsafe fn fn_80152880(this: usize, param_2: u32) {
    if read_word(this, 0x17BC) != 0 {
        return;
    }
    write_word(this, 0x17BC, 1);

    // observed: three `u16` on the stack, all zero except the middle one.
    // The original passes big-endian halfwords; keep byte-level stores so the
    // host endianness cannot change the payload.
    let mut position = [0u8; 6];
    if read_byte(this, 0x348) == 1 {
        platform::write_u16(position.as_mut_ptr() as usize + 2, 0x8000);
    }

    let _ = call5(
        FN_8016C890,
        S_WM_MR_YSTONGUEHIT as u32,
        0,
        param_2,
        position.as_mut_ptr() as u32,
        0,
    );

    // observed: `(**(code **)(*(int *)(this + 0xDEC) + 0x24))(
    //     this + 0xDEC, 0x159, 0)`.
    let iface = this + 0xDEC;
    let vtable = read_word(iface, 0) as usize;
    let entry = read_word(vtable, 0x24) as usize;
    let _ = call3(entry, iface as u32, 0x159, 0);
}

/// Walks up to five `+0x30` child links and returns the parent of the first
/// child whose virtual `+0x0C` predicate returns non-zero.
///
/// The original is five unrolled levels. Every exit returns the object that was
/// current *before* the matching child was tested; after five failed levels it
/// tail-calls itself with the sixth child.
///
/// # Original
/// - Ghidra: `FUN_80154550` at `0x80154550` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154550_FUN_80154550.c`
///
/// # Arguments
/// * `this` - head of the chain; its child at `+0x30` is tested first.
///
/// # Returns
/// The parent of the first matching child, or the recursive result.
///
/// # Side effects
/// One virtual call per level through the child's vtable slot `+0x0C`.
///
/// # Safety
/// Every `+0x30` link must be null or point at a readable object whose first
/// word is a vtable with a function at `+0x0C`.
///
/// # Confidence
/// high - the five levels and the parent-returning exits are explicit; the
/// chain role is the same as `0x80040720` in [`crate::runtime::cpp`].
pub unsafe fn fn_80154550(this: usize) -> usize {
    let mut current = this;

    // observed: five unrolled `+0x30` levels. The predicate receives the
    // child; every successful exit returns the parent held in `current`.
    let mut level = 0;
    while level < 5 {
        let child = read_word(current, 0x30) as usize;
        let vtable = read_word(child, 0) as usize;
        let entry = read_word(vtable, 0x0C) as usize;
        if call1(entry, child as u32) != 0 {
            return current;
        }
        current = child;
        level += 1;
    }

    // observed: after the fifth failure the recursion starts at the sixth
    // child (`*(current + 0x30)`).
    fn_80154550(read_word(current, 0x30) as usize)
}

/// Compares the last `':'`-separated component of an object's name with a
/// supplied name.
///
/// Both sides are split with `strrchr(..., ':')`; the comparison then uses
/// `strcmp` on the component after the final colon. A missing colon is treated
/// as "the whole string" for the supplied name, exactly as the original's
/// `col + 1` pointer arithmetic does for the object's own name.
///
/// # Original
/// - Ghidra: `FUN_80154630` at `0x80154630` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154630_FUN_80154630.c`
///
/// # Arguments
/// * `this` - object whose virtual `+0x20` slot returns a C string.
/// * `name` - C string to compare against.
///
/// # Returns
/// `true` when the last name components are equal.
///
/// # Side effects
/// One virtual call and two `strrchr`/`strcmp` calls; no writes.
///
/// # Safety
/// `name` must be a NUL-terminated string; the virtual `+0x20` slot must return
/// a NUL-terminated string. As in the original, if that string contains no
/// colon the code forms `base + 1`; callers must ensure the pointer is
/// meaningful enough for `strcmp`.
///
/// # Confidence
/// high - the two `strrchr` calls and the final equality are explicit.
pub unsafe fn fn_80154630(this: usize, name: usize) -> bool {
    let mut right = name;

    // observed: `col = strrchr(name, ':'); if (col != 0) right = col + 1;`.
    let colon = call2(FN_802E1C40, right as u32, 0x3A);
    if colon != 0 {
        right = colon.wrapping_add(1) as usize;
    }

    // observed: `(**(code **)(*this + 0x20))(this)` returns the object name.
    let own = vcall0(this, 0x20) as usize;
    let own_colon = call2(FN_802E1C40, own as u32, 0x3A);

    // observed: `strcmp(own_colon + 1, right) == 0`.
    call2(FN_802E1AB4, own_colon.wrapping_add(1), right as u32) == 0
}
