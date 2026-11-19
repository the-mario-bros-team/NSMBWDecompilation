//! World-map interaction object: pick up, carry and release another actor.
//!
//! # What lives here
//!
//! Seventeen functions of the world-map object that owns an attached actor. The
//! object keeps the attached actor's handle at `+0x1520`; "attached" means the
//! actor's `+0x36C` byte is non-zero and its `+0x368` word points back at the
//! owner. This module is the complete selection cycle:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8014E700` | [`fn_8014e700`] | reset the substate and release the secondary target |
//! | `0x8014E760` | [`fn_8014e760`] | enter interaction mode 1 |
//! | `0x8014E9B0` | [`wm_secondary_target`] | resolve the secondary target handle |
//! | `0x8014E9D0` | [`fn_8014e9d0`] | readiness predicate for a new selection |
//! | `0x8014ED00` | [`wm_reset_attachment_phase`] | clear the phase word and notify the state interface |
//! | `0x8014ED50` | [`fn_8014ed50`] | re-enable bit `0xB2` when the secondary path is active |
//! | `0x8014ED90` | [`wm_attach_target`] | attach a new actor, detaching the previous one |
//! | `0x8014EEF0` | [`wm_confirm_if_target`] | confirm an actor only if it is the current target |
//! | `0x8014EFB0` | [`wm_commit_target`] | validate, consume and release the attached actor |
//! | `0x8014F160` | [`wm_settle_after_attach`] | post-attach cleanup and retry clamp |
//! | `0x80150020` | [`wm_play_select_sound`] | play the target-dependent selection sound |
//! | `0x80150150` | [`wm_detach_target`] | clear the target and the owner status word |
//! | `0x801501D0` | [`wm_consider_candidate`] | keep the nearest idle candidate within X distance |
//! | `0x801503E0` | [`wm_select_candidate`] | select an explicit candidate when idle |
//! | `0x80150650` | [`wm_retry_exhausted`] | retry/timer exhaustion query |
//! | `0x801506A0` | [`wm_notify_attached`] | notify a target with the `+0x1544` guard |
//! | `0x801507C0` | [`wm_target_accept`] | virtual accept callback used by the commit path |
//!
//! # Original address range
//!
//! `0x8014E4E0..0x80150AB0` (coverage band D, 2,016 bytes of translated
//! functions). Promoted from `src/subsystems/r80140_80180.rs`, which still
//! keeps the lane inventory and the untranslated neighbours.
//!
//! # Why it exists
//!
//! The world map lets the player pick up a nearby actor, carry it, and put it
//! down again. The original object owns that small state machine; splitting it
//! into a semantic module makes the flow readable in one file instead of
//! hiding it among unrelated range translations.
//!
//! ```text
//! wm_consider_candidate ──> wm_select_candidate ──> wm_commit_target
//!                                                       │
//!                                                       v
//!                                                 wm_detach_target
//! wm_attach_target ──> wm_notify_attached          wm_settle_after_attach
//!          │
//!          └──> wm_reset_attachment_phase <── wm_confirm_if_target
//! ```
//!
//! # Object layout used here
//!
//! Offsets are relative to the interaction object unless stated otherwise.
//!
//! | offset | width | meaning | confidence |
//! |---|---|---|---|
//! | `+0x14D8` | word | owner/controller object that receives the `+0x294` status word | inferred |
//! | `+0x14E0` | word | embedded interface; vtable slots `+0x28` and `+0x18` | observed |
//! | `+0x151C` | word | attachment phase/result word | observed |
//! | `+0x1520` | word | attached actor handle (the actor's first word) | observed |
//! | `+0x153C` | byte | mode/state byte; selection is only legal at `0` | inferred |
//! | `+0x1541` | byte | colour/variant byte; `0xFF` means "none" | inferred |
//! | `+0x1544` | word | re-entrancy guard used while notifying | observed |
//! | `+0x1558` | word | best candidate wrapper | observed |
//! | `+0x155C` | float | best candidate X distance | observed |
//! | `+0x1560` | word | candidate sub-kind tag (`0`/`1`) | inferred |
//! | `+0x1564` | word | retry counter | observed |
//! | `+0x1568` | word | secondary retry/timer word | observed |
//!
//! The attached actor uses `+0x36C` as its state byte (`0` idle, `1` attached,
//! `3` consumed) and `+0x368` as the owner back-pointer.
//!
//! # Dependencies
//!
//! [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! The Wm actor animation helpers that operate on the same object live in
//! [`crate::game::worldmap`]; this module does not call them directly.
//!
//! # Start here
//!
//! [`wm_consider_candidate`] scores a nearby actor, [`wm_select_candidate`]
//! locks it in, [`wm_commit_target`] consumes it, and [`wm_detach_target`]
//! releases it. [`wm_attach_target`] is the lower-level attach path used by the
//! not-yet-translated caller `0x801502E0`.
//!
//! # Glossary
//!
//! * **actor** - a game object. Its first word is a vtable pointer; `+0x60` is
//!   the virtual base used by the actor system.
//! * **world map (Wm)** - the overworld/level-select map; evidence strings are
//!   prefixed `Wm_`.
//! * **DOL** - the Wii executable container (`sys/main.dol`).
//! * **handle** - here, the actor's first word, which is also its registry key.
//!
//! # Confidence summary
//!
//! Nine bodies are `high`; [`wm_reset_attachment_phase`] is `medium` because
//! Ghidra lost the arguments of its indirect call. The event operand is
//! inferred from both call sites.

use crate::platform;

// ---------------------------------------------------------------------------
// Shared memory and ABI helpers
// ---------------------------------------------------------------------------

/// Reads the 32-bit word at `base + offset`.
#[inline]
unsafe fn read_word(base: usize, offset: usize) -> u32 {
    platform::read_u32(base + offset)
}

/// Writes the 32-bit word `value` at `base + offset`.
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

/// Reads a big-endian IEEE-754 single at `base + offset`.
#[inline]
unsafe fn read_f32(base: usize, offset: usize) -> f32 {
    f32::from_bits(platform::read_u32(base + offset))
}

/// Writes a big-endian IEEE-754 single at `base + offset`.
#[inline]
unsafe fn write_f32(base: usize, offset: usize, value: f32) {
    platform::write_u32(base + offset, value.to_bits());
}

/// Calls virtual slot `slot` (a byte offset) with `this = this_arg`.
///
/// `this_arg` starts with the vtable pointer, matching the PowerPC C++ ABI
/// where an object's first word is its vtable.
#[inline]
unsafe fn vcall0(this_arg: usize, slot: usize) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call1(entry, this_arg as u32)
}

/// [`vcall0`] with one integer argument (r4).
#[inline]
unsafe fn vcall1(this_arg: usize, slot: usize, arg: u32) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call2(entry, this_arg as u32, arg)
}

/// Calls a virtual slot of an object whose vtable pointer is a *member* at
/// `object + vtable_field` rather than at offset 0. `this` is still `object`.
///
/// Used for actors: `+0x60` holds the actor virtual base (observed at every
/// `(**(code **)(*(int *)(param_1 + 0x60) + ...))` call site below).
#[inline]
unsafe fn member_vcall0(object: usize, vtable_field: usize, slot: usize) -> u32 {
    let vtable = read_word(object, vtable_field) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call1(entry, object as u32)
}

/// [`member_vcall0`] with one integer argument.
#[inline]
unsafe fn member_vcall1(object: usize, vtable_field: usize, slot: usize, a0: u32) -> u32 {
    let vtable = read_word(object, vtable_field) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call2(entry, object as u32, a0)
}

// ---------------------------------------------------------------------------
// Field offsets, table addresses and event operands
// ---------------------------------------------------------------------------

/// `0x80162D00`: registry lookup from a 32-bit key to a registered object.
///
/// Bucket `key & 7` of the table at `0x80377A70` (8 bytes per bucket, linked
/// list at `+0`/`+4`) is searched by `0x80162C30`, which compares the key with
/// the first word of the object stored at `entry + 8`. The lookup returns that
/// object, or null. `// observed:` bodies of `80162d00_FUN_80162d00.c` and
/// `80162c30_FUN_80162c30.c`.
const ACTOR_LOOKUP: usize = 0x8016_2D00;

/// Object field `+0x14D8`: pointer to the world-map owner/controller object
/// that receives the `+0x294` status word.
const OWNER: usize = 0x14D8;

/// Object field `+0x14E0`: embedded interface whose vtable is queried with
/// slot `+0x28` (returns an object) and `+0x18` (state-changed event).
const STATE_IFACE: usize = 0x14E0;

/// Object field `+0x1520`: handle of the currently attached actor - the
/// actor's first word, which is also the registry key (`0` = none).
const TARGET_KEY: usize = 0x1520;

/// Object field `+0x1544`: re-entrancy guard set while an attached actor is
/// being notified. It is `0` for all normal work and `1` inside the callback.
const NOTIFY_GUARD: usize = 0x1544;

/// Object field `+0x151C`: attachment phase/result word.
const PHASE: usize = 0x151C;

/// Interface name handed to vtable slot `+0x10` when the caller must be the
/// "current player" variant. `// observed:` `&DAT_80376e88` at the call sites.
const IFACE_NAME_PLAYER: u32 = 0x8037_6E88;

/// Event operand passed to [`wm_reset_attachment_phase`] after a successful
/// commit. `// observed:` `&DAT_80376f08`.
const EVENT_COMMIT: u32 = 0x8037_6F08;

/// Event operand passed after an explicit selection. `// observed:`
/// `&DAT_80376f48` in `FUN_801503e0`.
const EVENT_SELECT: u32 = 0x8037_6F48;

/// `0x8014F630`: refreshes the attachment phase animation after the target key
/// changes. Called by [`wm_attach_target`] with `phase = 0`; the body is
/// float- and `r2`-heavy and is not translated here.
const ATTACH_PHASE_REFRESH: usize = 0x8014_F630;

/// `0x801507C0`: asks the attached actor to accept the owner. Returns non-zero
/// on success, with the same `+0x1544` guard used by [`wm_notify_attached`].
const TARGET_ACCEPT: usize = 0x8015_07C0;

/// `0x80150700`: asks the attached actor to release the owner (no result).
const TARGET_RELEASE: usize = 0x8015_0700;

/// `0x80150020`: plays the pick-up/selected sound, choosing `0x15D` when the
/// target's type id (`+8`) is `0x13`, otherwise `0xC0`.
const PLAY_SELECT_SOUND: usize = 0x8015_0020;

/// Object field `+0x1524`: handle of the secondary target (used by the Yoshi
/// tongue path). `0` means none.
const SECONDARY_TARGET: usize = 0x1524;

/// `0x8004A8C0`: resets the interaction/attachment substate `(this)`; clears
/// the halfword at `+0x100`, status bits `0x4E`/`0x51`, and bit 2 of `+0x30C`.
const FN_8004A8C0: usize = 0x8004_A8C0;

/// `0x80056CB0`: clears one status bit of an actor (`actor`, `bit`).
const FN_80056CB0: usize = 0x8005_6CB0;

/// `0x80056CF0`: tests one status bit of an actor (`actor`, `bit`).
const FN_80056CF0: usize = 0x8005_6CF0;

/// `0x80147250`: sets the mode byte `+0x153C`, forcing `1` when the actor's
/// virtual `+0xE0` predicate returns non-zero.
const FN_80147250: usize = 0x8014_7250;

/// `0x80151C20`: sets status bit `0x7C` on an actor.
const FN_80151C20: usize = 0x8015_1C20;

/// `0x80057E70`: plays a sound by id through the object at `+0xDEC`.
const FN_80057E70: usize = 0x8005_7E70;

/// Candidate wrapper field `+0x4`: the candidate actor object.
const CANDIDATE_ACTOR: usize = 0x4;

// ===========================================================================
// Attachment state machine
// ===========================================================================

/// Clears the attachment phase word and notifies the embedded state interface
/// that the selection state changed.
///
/// # Original
/// - Ghidra: `FUN_8014ed00` at `0x8014ed00` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014ed00_FUN_8014ed00.c`
///
/// # Arguments
/// * `wm` - base address of the world-map interaction object.
/// * `event` - operand forwarded to the state interface. Call sites pass
///   [`EVENT_COMMIT`] or [`EVENT_SELECT`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0` to `wm+0x151C` and performs an indirect call through
/// `wm+0x14E0`'s vtable slot `+0x18`.
///
/// # Safety
/// `wm` must be a live object whose `+0x14E0` word is a valid vtable pointer.
///
/// # Confidence
/// medium - the callee body is exact, but Ghidra prints the indirect call with
/// no arguments; `event` is inferred from both call sites.
pub unsafe fn wm_reset_attachment_phase(wm: usize, event: u32) {
    // observed: `*(undefined4 *)(param_1 + 0x151c) = 0;`
    write_word(wm, PHASE, 0);
    // unknown: Ghidra renders `(**(code **)(...+0x18))()`; r3 (this) and r4
    // (event) are implied by the ABI and by the two callers.
    let _ = vcall1(wm + STATE_IFACE, 0x18, event);
}

/// Attaches a new actor to this object, detaching the previous one first.
///
/// # Original
/// - Ghidra: `FUN_8014ed90` at `0x8014ed90` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014ed90_FUN_8014ed90.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `target` - actor to attach; the object's first word is used as the handle
///   stored at `wm+0x1520`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Previous target (if any): `+0x36C = 0`, `+0x368 = 0`.
/// * `wm+0x1520` becomes `target[0]`.
/// * Refreshes the phase animation through `0x8014F630(wm, 0)`.
/// * New target: `+0x36C = 1`, `+0x368 = wm[0]` (owner back-pointer).
/// * Calls [`wm_notify_attached`].
///
/// # Safety
/// `wm` must be live and `target` must be a live actor with a valid vtable.
///
/// # Confidence
/// high - every store and call is explicit; only the exact meaning of state
/// byte `1` is inferred.
pub unsafe fn wm_attach_target(wm: usize, target: usize) {
    let previous = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY)) as usize;
    if previous != 0 {
        write_byte(previous, 0x36C, 0);
        write_word(previous, 0x368, 0);
    }
    write_word(wm, TARGET_KEY, read_word(target, 0));
    platform::call2(ATTACH_PHASE_REFRESH, wm as u32, 0);
    // observed: `param_2[0xdb] = 1` (byte +0x36C) and `param_2[0xda] = *param_1`
    // (word +0x368 = the owner's vtable/identity word).
    write_byte(target, 0x36C, 1);
    write_word(target, 0x368, read_word(wm, 0));
    wm_notify_attached(wm, target as u32);
}

/// Confirms the attachment only when the given actor is the current target.
///
/// # Original
/// - Ghidra: `FUN_8014eef0` at `0x8014eef0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014eef0_FUN_8014eef0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `actor` - actor object that is asking to be confirmed.
///
/// # Returns
/// `1` when `actor` is the attached target and the interface name check passed;
/// `0` otherwise.
///
/// # Side effects
/// On success, calls [`wm_reset_attachment_phase`] with [`EVENT_COMMIT`].
///
/// # Safety
/// `wm` and `actor` must be live objects with valid vtables.
///
/// # Confidence
/// high - every branch and field is explicit in the evidence.
pub unsafe fn wm_confirm_if_target(wm: usize, actor: u32) -> u32 {
    // observed: slot +0x28 returns an interface object; slot +0x10 compares a
    // name against &DAT_80376e88 (non-zero = match).
    let iface = vcall0(wm + STATE_IFACE, 0x28) as usize;
    if vcall1(iface, 0x10, IFACE_NAME_PLAYER) == 0 {
        return 0;
    }
    // observed: the original performs this lookup twice (first for the null
    // test, then for the comparison). Both calls are kept.
    let first = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY));
    if first == 0 {
        return 0;
    }
    let second = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY));
    if second != actor {
        return 0;
    }
    wm_reset_attachment_phase(wm, EVENT_COMMIT);
    1
}

/// Commits the current target: validates it, marks it consumed, then detaches
/// it.
///
/// # Original
/// - Ghidra: `FUN_8014efb0` at `0x8014efb0` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014efb0_FUN_8014efb0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `force` - `0` = normal check, non-zero = accept regardless of the
///   target's `+0x38C` state. A successful interface-name match also forces.
///
/// # Returns
/// `1` when a target was consumed, `0` when no target exists or the checks
/// failed.
///
/// # Side effects
/// * `+0x43C` of a state-1 target is armed to `10` frames when forced.
/// * `+0x36C` of the target becomes `3` (consumed).
/// * Calls `0x801507C0` (accept), `0x80150020` (sound) and [`wm_detach_target`].
///
/// # Safety
/// `wm` must be live; the target handle must either be `0` or resolve to a live
/// actor with a valid `+0x60` vtable.
///
/// # Confidence
/// high - the flow is explicit; only the meaning of `+0x38C`/`+0x43C` is
/// inferred (target's "ready" state and a delay counter).
pub unsafe fn wm_commit_target(wm: usize, force: u32) -> u32 {
    let iface = vcall0(wm + STATE_IFACE, 0x28) as usize;
    let mut force = force;
    if vcall1(iface, 0x10, EVENT_COMMIT) != 0 {
        // observed: a name match overrides the caller's `force` argument.
        force = 1;
    }
    let target = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY));
    if target == 0 {
        return 0;
    }
    let target = target as usize;
    if force == 0 {
        // inferred: +0x38C == 1 means "ready to be consumed"; slot +0xE0 is a
        // per-actor veto that returns non-zero when the actor refuses.
        if read_byte(target, 0x38C) != 1 {
            return 0;
        }
        if member_vcall0(target, 0x60, 0xE0) != 0 {
            return 0;
        }
    } else if read_byte(target, 0x38C) == 1 {
        // inferred: arm a 10-frame hold before the target disappears.
        write_word(target, 0x43C, 10);
    }
    // observed: state 3 = consumed.
    write_byte(target, 0x36C, 3);
    platform::call2(TARGET_ACCEPT, wm as u32, target as u32);
    platform::call1(PLAY_SELECT_SOUND, wm as u32);
    wm_detach_target(wm);
    1
}

/// Settles the object after an attachment attempt.
///
/// # Original
/// - Ghidra: `FUN_8014f160` at `0x8014f160` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014f160_FUN_8014f160.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Sets the colour/variant byte `+0x1541` to `0xFF` ("none").
/// * If the target is in state `1`, notifies it through `0x80150700` and
///   detaches it.
/// * If no target remains, clears owner field `+0x294`.
/// * Clamps the retry counter `+0x1564` to `4`.
///
/// # Safety
/// `wm` must be live; `+0x14D8` must point at the owner object.
///
/// # Confidence
/// high - all writes and calls are explicit; only the `0x1541` meaning is
/// inferred.
pub unsafe fn wm_settle_after_attach(wm: usize) {
    // inferred: 0xFF clears the per-player colour/variant tint.
    write_byte(wm, 0x1541, 0xFF);
    let owner = read_word(wm, OWNER) as usize;
    // observed: lookup is performed twice, once before and once after the
    // possible detach.
    let target = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY)) as usize;
    if target != 0 && read_byte(target, 0x36C) == 1 {
        platform::call2(TARGET_RELEASE, wm as u32, target as u32);
        wm_detach_target(wm);
    }
    let remaining = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY));
    if remaining == 0 {
        write_word(owner, 0x294, 0);
    }
    if (read_word(wm, 0x1564) as i32) > 4 {
        write_word(wm, 0x1564, 4);
    }
}

/// Releases the attached actor and clears the owner's status word.
///
/// # Original
/// - Ghidra: `FUN_80150150` at `0x80150150` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150150_FUN_80150150.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// `true` when an attached actor existed and was released.
///
/// # Side effects
/// Clears the actor's `+0x36C` state and `+0x368` back-pointer, clears the
/// object's `+0x1520` handle, and clears owner field `+0x294`.
///
/// # Safety
/// `wm` must be live; a non-null `+0x1520` must resolve to a live actor.
///
/// # Confidence
/// high - direct field writes with no ambiguity.
pub unsafe fn wm_detach_target(wm: usize) -> bool {
    let target = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY)) as usize;
    if target != 0 {
        // observed: state 0 = idle, back-pointer 0 = no owner.
        write_byte(target, 0x36C, 0);
        write_word(target, 0x368, 0);
        write_word(wm, TARGET_KEY, 0);
        let owner = read_word(wm, OWNER) as usize;
        write_word(owner, 0x294, 0);
    }
    target != 0
}

/// Scores one candidate for attachment, keeping the nearest valid one.
///
/// # Original
/// - Ghidra: `FUN_801501d0` at `0x801501d0` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801501d0_FUN_801501d0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `candidate` - wrapper object; `+0x4` points at the candidate actor.
///
/// # Returns
/// `1` when the candidate became the new best; `0` when it was rejected.
///
/// # Side effects
/// On success stores the wrapper at `+0x1558`, the absolute X distance at
/// `+0x155C`, and the candidate's `+0x36D` tag at `+0x1560`.
///
/// # Safety
/// `wm` must be live and `candidate` either `0` or a live wrapper.
///
/// # Confidence
/// high - the distance test and all stores are explicit. The X source
/// `0x8008C570` is identified only by its use here.
pub unsafe fn wm_consider_candidate(wm: usize, candidate: usize) -> u32 {
    if platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY)) != 0 {
        return 0;
    }
    let iface = vcall0(wm + STATE_IFACE, 0x28) as usize;
    if vcall1(iface, 0x10, IFACE_NAME_PLAYER) == 0 {
        return 0;
    }
    let actor = read_word(candidate, CANDIDATE_ACTOR) as usize;
    if actor == 0 {
        return 0;
    }
    // inferred: +0x36C must be 0 (idle) for a candidate to be attachable.
    if read_byte(actor, 0x36C) != 0 {
        return 0;
    }
    // observed: `0x8008C570(candidate)` is a one-line getter,
    // `(float)(*(float *)(candidate + 0x1C) + *(float *)(actor + 0xAC))`.
    // It is inlined here because Ghidra's declared `double` return disagrees
    // with every call site's `(double)` promotion; inlining keeps the float
    // arithmetic exact either way.
    let actor_x = read_f32(candidate, 0x1C) + read_f32(actor, 0xAC);
    // +0xAC of `wm` is its world X; +0x155C caches the best distance so far.
    let distance = (read_f32(wm, 0xAC) - actor_x).abs();
    if read_f32(wm, 0x155C) <= distance {
        return 0;
    }
    write_word(wm, 0x1558, candidate as u32);
    write_f32(wm, 0x155C, distance);
    // inferred: +0x36D tags which sub-kind of candidate this is; 0/1 is all
    // the consumer needs.
    write_word(
        wm,
        0x1560,
        if read_byte(actor, 0x36D) == 0 { 0 } else { 1 },
    );
    1
}

/// Selects an explicit candidate when the object is idle.
///
/// # Original
/// - Ghidra: `FUN_801503e0` at `0x801503e0` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801503e0_FUN_801503e0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `candidate` - wrapper object, or `0`.
///
/// # Returns
/// `1` when the candidate was selected; `0` otherwise.
///
/// # Side effects
/// Stores the candidate's handle (the wrapper's first word) at `+0x1520` and
/// calls [`wm_reset_attachment_phase`] with [`EVENT_SELECT`].
///
/// # Safety
/// `wm` must be live; `candidate` must be `0` or a live wrapper.
///
/// # Confidence
/// high - all guards are explicit. `+0x36D` is again the candidate sub-kind.
pub unsafe fn wm_select_candidate(wm: usize, candidate: usize) -> u32 {
    // inferred: +0x153C is a mode/state byte; selection is only legal at 0.
    if read_word(wm, 0x153C) != 0 {
        return 0;
    }
    if platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY)) != 0 {
        return 0;
    }
    let iface = vcall0(wm + STATE_IFACE, 0x28) as usize;
    if vcall1(iface, 0x10, IFACE_NAME_PLAYER) == 0 {
        return 0;
    }
    if candidate == 0 {
        return 0;
    }
    // observed: `*(char *)(param_2 + 0xdb)` is byte +0x36C; the candidate must
    // be idle and its +0x36D tag must be non-zero.
    if read_byte(candidate, 0x36C) != 0 {
        return 0;
    }
    if read_byte(candidate, 0x36D) == 0 {
        return 0;
    }
    write_word(wm, TARGET_KEY, read_word(candidate, 0));
    wm_reset_attachment_phase(wm, EVENT_SELECT);
    1
}

/// Reports whether the attachment attempts are exhausted.
///
/// # Original
/// - Ghidra: `FUN_80150650` at `0x80150650` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150650_FUN_80150650.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// `1` when the retry counter `+0x1564` is greater than `4`, or when the
/// signed word `+0x1568` is positive; `0` otherwise.
///
/// # Side effects
/// None. This is a pure query.
///
/// # Safety
/// `wm` must be a readable object.
///
/// # Confidence
/// high - the sign-bit idiom is reproduced literally.
pub unsafe fn wm_retry_exhausted(wm: usize) -> u32 {
    if (read_word(wm, 0x1564) as i32) > 4 {
        return 1;
    }
    // observed: `(-x & ~x) >> 31` extracts "x is positive and non-zero".
    let x = read_word(wm, 0x1568);
    let negated = (!x).wrapping_add(1);
    (negated & !x) >> 31
}

/// Notifies an actor that it has been attached to this object.
///
/// # Original
/// - Ghidra: `FUN_801506a0` at `0x801506a0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801506a0_FUN_801506a0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `target` - actor object to notify, or `0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets `+0x1544` to `1`, calls the target's vtable slot `+0x80` with `wm`,
/// then restores `+0x1544` to `0`.
///
/// # Safety
/// `target` must be `0` or a live actor with a valid vtable.
///
/// # Confidence
/// high - the guard and call are explicit.
pub unsafe fn wm_notify_attached(wm: usize, target: u32) {
    if target != 0 {
        write_word(wm, NOTIFY_GUARD, 1);
        // observed: `*(int *)(param_2 + 0x60) + 0x80`, i.e. the actor virtual
        // base is a member at +0x60, not the object's first word.
        let _ = member_vcall1(target as usize, 0x60, 0x80, wm as u32);
        write_word(wm, NOTIFY_GUARD, 0);
    }
}

// ===========================================================================
// Secondary-target and per-frame helpers
//
// These bodies complete the interaction object around the attach flow above:
// the secondary target slot `+0x1524`, the readiness predicate used before a
// selection, the select sound, and the virtual accept callback that the main
// commit path already calls through `TARGET_ACCEPT`.
// ===========================================================================

/// Resets the interaction substate before a release, then releases the
/// secondary target's animation.
///
/// The body clears status bit `0xB2`, looks up the secondary target handle at
/// `+0x1524`, and asks the target to enter animation mode `0` through
/// `0x80147250` (which writes the target's `+0x153C` byte).
///
/// # Original
/// - Ghidra: `FUN_8014E700` at `0x8014E700` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014e700_FUN_8014e700.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8004A8C0(wm)`, clears status bit `0xB2`, and calls
/// `0x80147250(secondary, 0)` when a secondary target exists.
///
/// # Safety
/// `wm` must be a live actor and `+0x1524` either `0` or a live handle.
///
/// # Confidence
/// high - the calls and the bit number are explicit; Ghidra dropped the
/// argument of `0x8004A8C0`, whose own body reads `r3`.
pub unsafe fn fn_8014e700(wm: usize) {
    // observed: `FUN_8004a8c0()` with r3 still holding `wm`.
    let _ = platform::call1(FN_8004A8C0, wm as u32);
    let _ = platform::call2(FN_80056CB0, wm as u32, 0xB2);

    let secondary = wm_secondary_target(wm);
    if secondary != 0 {
        let _ = platform::call2(FN_80147250, secondary, 0);
    }
}

/// Enters interaction mode 1 and asks the actor's virtual interface to stop its
/// current action.
///
/// # Original
/// - Ghidra: `FUN_8014E760` at `0x8014E760` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014e760_FUN_8014e760.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls virtual slot `+0x378` of the actor base at `+0x60` with `(wm, 0)`,
/// then writes `1` to the mode word `+0x14A4`.
///
/// # Safety
/// `wm+0x60` must hold a vtable with a function at `+0x378`.
///
/// # Confidence
/// high - both effects are explicit; only the meaning of mode `1` is inferred.
pub unsafe fn fn_8014e760(wm: usize) {
    // observed: `(**(code **)(*(int *)(wm + 0x60) + 0x378))(wm, 0)`.
    let _ = member_vcall1(wm, 0x60, 0x378, 0);
    write_word(wm, 0x14A4, 1);
}

/// Resolves the secondary target handle stored at `+0x1524`.
///
/// This is the exact counterpart of the primary [`TARGET_KEY`] lookup, using
/// the registry function `0x80162D00`.
///
/// # Original
/// - Ghidra: `FUN_8014E9B0` at `0x8014E9B0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014e9b0_FUN_8014e9b0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// The registered actor for `+0x1524`, or `0` when the slot is empty.
///
/// # Side effects
/// One registry lookup; no writes.
///
/// # Safety
/// `wm` must be readable at `+0x1524`; a non-zero value must be a live handle.
///
/// # Confidence
/// high - the guard and lookup shape match the primary target exactly. Ghidra
/// dropped the lookup's argument; r3 is the `+0x1524` word at the call site.
pub unsafe fn wm_secondary_target(wm: usize) -> u32 {
    let key = read_word(wm, SECONDARY_TARGET);
    if key == 0 {
        return 0;
    }
    platform::call1(ACTOR_LOOKUP, key)
}

/// Reports whether the interaction object is ready to start a new selection.
///
/// The predicate is true only when all three checks pass: status bit `0x91` is
/// clear, bit 28 of the flag word at `+0x10D4` is clear, and
/// [`wm_retry_exhausted`] is false.
///
/// # Original
/// - Ghidra: `FUN_8014E9D0` at `0x8014E9D0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014e9d0_FUN_8014e9d0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// `1` when all three checks pass, `0` otherwise.
///
/// # Side effects
/// Calls `0x80056CF0` and [`wm_retry_exhausted`]; no writes.
///
/// # Safety
/// `wm` must be readable at `+0x10D4` and through the retry fields.
///
/// # Confidence
/// high - the short-circuit order is reproduced literally.
pub unsafe fn fn_8014e9d0(wm: usize) -> u32 {
    if platform::call2(FN_80056CF0, wm as u32, 0x91) != 0 {
        return 0;
    }
    if (read_word(wm, 0x10D4) & 0x1000_0000) != 0 {
        return 0;
    }
    if wm_retry_exhausted(wm) != 0 {
        return 0;
    }
    1
}

/// Re-enables status bit `0xB2` when the secondary-target path is active.
///
/// # Original
/// - Ghidra: `FUN_8014ED50` at `0x8014ED50` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8014ed50_FUN_8014ed50.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// If status bit `0xB2` is set, calls `0x80151C20(wm)`, which sets bit `0x7C`.
///
/// # Safety
/// `wm` must be a live actor.
///
/// # Confidence
/// high - one test and one call.
pub unsafe fn fn_8014ed50(wm: usize) {
    if platform::call2(FN_80056CF0, wm as u32, 0xB2) != 0 {
        let _ = platform::call1(FN_80151C20, wm as u32);
    }
}

/// Plays the selection sound for the current target.
///
/// The sound id depends on the target's type id at `+8`: `0x15D` when it is
/// `0x13`, otherwise `0xC0`.
///
/// # Original
/// - Ghidra: `FUN_80150020` at `0x80150020` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150020_FUN_80150020.c`
///
/// # Arguments
/// * `wm` - world-map interaction object; its target handle is at
///   [`TARGET_KEY`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One registry lookup and one sound call `0x80057E70(wm, id, 0)`.
///
/// # Safety
/// `wm` must be live; a non-zero target handle must resolve to a live actor.
///
/// # Confidence
/// high - the type-id comparison and both sound ids are explicit; the sound
/// names are not recovered.
pub unsafe fn wm_play_select_sound(wm: usize) {
    let target = platform::call1(ACTOR_LOOKUP, read_word(wm, TARGET_KEY)) as usize;
    if target == 0 {
        return;
    }

    // observed: `*(short *)(target + 8) == 0x13`.
    let kind = platform::read_u16(target + 8);
    let sound = if kind == 0x13 { 0x15D } else { 0xC0 };
    let _ = platform::call3(FN_80057E70, wm as u32, sound, 0);
}

/// Asks the attached actor to accept the owner, returning whether it did.
///
/// The read/write guard at [`NOTIFY_GUARD`] is set for the duration of the
/// virtual call and restored afterwards, exactly like
/// [`wm_notify_attached`]. The return value is folded to one bit with
/// `(-value | value) >> 31`, i.e. "value != 0".
///
/// # Original
/// - Ghidra: `FUN_801507C0` at `0x801507C0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801507c0_FUN_801507c0.c`
///
/// # Arguments
/// * `wm` - world-map interaction object.
/// * `target` - actor to ask, or `0`.
///
/// # Returns
/// `1` when the target's virtual `+0x8C` slot returned non-zero, `0` otherwise
/// (including a null target).
///
/// # Side effects
/// Writes `1` then `0` to `+0x1544`; one virtual call with `(target, wm)`.
///
/// # Safety
/// A non-null `target` must have a vtable at `+0x60` with a function at
/// `+0x8C`.
///
/// # Confidence
/// high - the guard, call and sign-fold are explicit.
pub unsafe fn wm_target_accept(wm: usize, target: u32) -> u32 {
    let mut result: u32 = 0;
    if target != 0 {
        write_word(wm, NOTIFY_GUARD, 1);
        result = member_vcall1(target as usize, 0x60, 0x8C, wm as u32);
        write_word(wm, NOTIFY_GUARD, 0);
    }

    // observed: `(-result | result) >> 31`.
    (result.wrapping_neg() | result) >> 31
}
