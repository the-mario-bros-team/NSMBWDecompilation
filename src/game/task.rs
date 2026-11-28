//! Task and state-list helpers: walk a task table, compare keys, step an angle.
//!
//! # What lives here
//!
//! Twenty generic engine helpers recovered from the `0x8015F200..0x8015F9F0`
//! cluster:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8015F200` | [`step_angle_toward`] | move a 16-bit value toward a target with a three-stage speed rule |
//! | `0x8015F340..0x8015F4F0` | [`step_short_clamped`], [`step_int_clamped`], [`step_int_clamped_b`], [`step_float_clamped`], [`step_short_clamped_wrap`] | fixed-step movers that report `1` on arrival |
//! | `0x8015F600` | [`task_list_init`] | seed the `{ table, u16 count, u16 index }` header |
//! | `0x8015F620` | [`run_task_list`] | call a `u16`-indexed function table until a task returns something other than `1` |
//! | `0x8015F6D0`/`0x8015F6E0` | [`task_callback`], [`invoke_task_callback`] | read and call the global task callback |
//! | `0x8015F7C0..0x8015F9F0` | [`state_descriptor_init`], [`state_descriptor_release`], [`state_descriptor_id`], [`state_object_init`], [`state_object_release`] | state-descriptor/state-object construct and release |
//! | `0x8015F840..0x8015F930` | [`fn_8015f840`], [`fn_8015f8c0`], [`fn_8015f8d0`], [`fn_8015f920`] | `0x8032269C` slots 1/3/4/6: id predicate, comparison forwarders and the name getter |
//! | `0x8015F850` | [`keys_equal`] | compare two objects by the key from vtable slot `+0x24` |
//!
//! # Original address range
//!
//! `0x8015F200..0x8015F9F0` (20 functions, 1,384 bytes; the first three are
//! coverage band I of lane `r80140_80180` in
//! `docs/COVERAGE_r80140_80180.md`, the rest were added in wave 7). The first
//! three were promoted from `src/subsystems/r80140_80180.rs`, which now holds
//! no code. The local `read_word`/`read_half`/`write_half`/`vcall0` helpers
//! were added because this module is self-contained; the wave-7 functions still
//! call into `0x802B9280` and the `0x8015F2xx` cluster by absolute address.
//!
//! # Why it exists
//!
//! These are the smallest pieces of the game's task/state base: a task list is
//! just a header pointing at a table of `int (int)` functions, and the other
//! two helpers are integer/angle utilities that such tasks call. Splitting them
//! out of the address-range staging file makes the "what is a task" answer
//! readable in one place.
//!
//! # Task-list layout
//!
//! `// observed:` `0x8015F600` initialises exactly three header fields, which
//! is the layout [`run_task_list`] walks:
//!
//! | offset | width | meaning |
//! |---|---|---|
//! | `+0x00` | word | pointer to the task table |
//! | `+0x04` | `u16` | entry count |
//! | `+0x06` | `u16` | current index (advanced by the runner) |
//!
//! # Dependencies
//!
//! [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! The WorldMap actors whose state machines use these helpers live in
//! [`crate::game::worldmap`]. The neighbouring constructors and destructors
//! `0x8015F7C0`, `0x8015F800`, `0x8015F9A0` and `0x8015F9F0` are translated in
//! the wave-7 section at the bottom of this file.
//!
//! # Start here
//!
//! [`run_task_list`] is the entry point: it shows the three-field header and
//! the table-of-functions calling convention. [`step_angle_toward`] is a
//! self-contained integer helper with no memory dependencies beyond its
//! output pointer, and [`keys_equal`] shows the vtable-key comparison used to
//! identify state objects.
//!
//! # Glossary
//!
//! * **task list** - a header `{ table, u16 count, u16 index }` whose table
//!   entries are called in order until one returns a value other than `1`.
//! * **vtable** - the first word of a C++ object; slot `+0x24` here returns an
//!   integer key for the object.
//! * **DOL** - the Wii executable container (`sys/main.dol`); its text starts
//!   at virtual `0x80004000`.
//!
//! # Confidence summary
//!
//! Thirteen of the sixteen functions are `high`: the task-runner loop, the
//! movers, the header initialiser and the constructors are reproduced
//! instruction-for-instruction.  [`step_float_clamped`] is `medium` (the
//! double/float cast chain and the `r2` zero constant are inferred from the
//! Ghidra prototype) and the two release routines are `medium` (Ghidra prints
//! the `0x802B9280` free call with no argument; `this` is passed in `r3`).
//! Only the label "angle" on [`step_angle_toward`] is inferred from its
//! fixed-point use.

use crate::platform;

/// Small-data anchor for this batch (`r13` at the game entry).
///
/// `0x800041C0` loads `0x8042F680` into `r13`; every SDA global below is
/// `SDA_BASE - displacement`.  Same constant as
/// [`crate::engine::startup::SDA_BASE`], re-exported here for readability.
const SDA_BASE: usize = crate::engine::startup::SDA_BASE;


// ===========================================================================
// Shared memory and ABI helpers
// ===========================================================================

/// Reads the 32-bit word at `base + offset`.
#[inline]
unsafe fn read_word(base: usize, offset: usize) -> u32 {
    platform::read_u32(base + offset)
}

/// Reads the big-endian 16-bit halfword at `base + offset`.
#[inline]
unsafe fn read_half(base: usize, offset: usize) -> u16 {
    platform::read_u16(base + offset)
}

/// Writes the big-endian 16-bit halfword `value` at `base + offset`.
#[inline]
unsafe fn write_half(base: usize, offset: usize, value: u16) {
    platform::write_u16(base + offset, value);
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

// ===========================================================================
// Angle stepping and task-list walking
// (0x8015F200..0x8015F850; three functions)
// ===========================================================================

/// Steps a 16-bit value toward a target using a three-stage speed rule.
///
/// # Original
/// - Ghidra: `FUN_8015f200` at `0x8015f200` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f200_FUN_8015f200.c`
///
/// # Arguments
/// * `value` - in/out 16-bit value (typically a fixed-point angle).
/// * `target` - value to reach.
/// * `divisor` - divides the remaining delta to produce the natural step.
/// * `max_step` - magnitude clamp for that natural step.
/// * `snap_threshold` - when the natural step is at most this, snap directly
///   to `target`.
///
/// # Returns
/// Sign-extended `target - *value` after the update.
///
/// # Side effects
/// Writes the updated value through `value`.
///
/// # Safety
/// `value` must be a valid writable pointer and `divisor` must be non-zero
/// (the PowerPC division would trap otherwise).
///
/// # Confidence
/// high - the arithmetic is reproduced instruction-for-instruction; only the
/// semantic label "angle" is inferred from the fixed-point use.
pub unsafe fn step_angle_toward(
    value: *mut i16,
    target: i16,
    divisor: i16,
    max_step: i16,
    snap_threshold: i16,
) -> i32 {
    let current = platform::read_u16(value as usize) as i16;
    if current != target {
        // observed: `(short)(target - current) / divisor`, truncated to i16.
        let delta = target.wrapping_sub(current);
        let step = (delta as i32 / divisor as i32) as i16;
        let step_i = step as i32;
        let threshold_i = snap_threshold as i32;
        if step_i > threshold_i || step_i < -threshold_i {
            // observed: clamp the natural step to +/- max_step. The original
            // reuses the max_step parameter as the advance.
            let mut advance = max_step;
            if step_i <= max_step as i32 {
                advance = step;
                if step_i < -(max_step as i32) {
                    advance = max_step.wrapping_neg();
                }
            }
            platform::write_u16(value as usize, (current.wrapping_add(advance)) as u16);
        } else if target.wrapping_sub(current) < 0 {
            let next = current.wrapping_sub(snap_threshold);
            platform::write_u16(value as usize, (next) as u16);
            if target.wrapping_sub(next) >= 0 {
                platform::write_u16(value as usize, (target) as u16);
            }
        } else {
            let next = current.wrapping_add(snap_threshold);
            platform::write_u16(value as usize, (next) as u16);
            if target.wrapping_sub(next) < 1 {
                platform::write_u16(value as usize, (target) as u16);
            }
        }
    }
    // observed: return `(int)(short)(target - platform::read_u16(value as usize) as i16)`.
    target.wrapping_sub(platform::read_u16(value as usize) as i16) as i32
}

/// Runs a table of task functions until one of them returns something other
/// than `1`, or the table is exhausted.
///
/// # Original
/// - Ghidra: `FUN_8015f620` at `0x8015f620` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f620_FUN_8015f620.c`
///
/// # Arguments
/// * `header` - task-list header; layout (observed in `0x8015F600`):
///   `+0x0` table pointer, `+0x4` entry count (`u16`), `+0x6` current index
///   (`u16`).
/// * `arg` - forwarded to every task function.
///
/// # Returns
/// The first non-`1` task result, or `2` when the list is exhausted/empty.
///
/// # Side effects
/// Advances the `u16` index at `header+0x6` for each task that returns `1`.
///
/// # Safety
/// `header` must be readable/writable; the table must hold `count` valid
/// function pointers taking one integer argument.
///
/// # Confidence
/// high - header layout confirmed by `8015f600_FUN_8015f600.c`, which
/// initialises exactly these three fields.
pub unsafe fn run_task_list(header: usize, arg: u32) -> i32 {
    let count = read_half(header, 4);
    let mut index = read_half(header, 6);
    while index < count {
        let table = read_word(header, 0) as usize;
        let task = read_word(table, index as usize * 4) as usize;
        let result = platform::call1(task, arg) as i32;
        if result != 1 {
            return result;
        }
        index = index.wrapping_add(1);
        write_half(header, 6, index);
    }
    2
}

/// Compares two objects by the key returned from vtable slot `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_8015f850` at `0x8015f850` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f850_FUN_8015f850.c`
///
/// # Arguments
/// * `a`, `b` - objects with a vtable; slot `+0x24` returns an integer key.
///
/// # Returns
/// `1` when the keys are equal, `0` otherwise.
///
/// # Side effects
/// Two virtual calls (considered read-only).
///
/// # Safety
/// Both objects must be live and expose vtable slot `+0x24`.
///
/// # Confidence
/// high - `countLeadingZeros(x - y) >> 5` is exactly `x == y`; the original
/// evaluates `b` first, then `a`, which is preserved.
pub unsafe fn keys_equal(a: usize, b: usize) -> u32 {
    let key_b = vcall0(b, 0x24);
    let key_a = vcall0(a, 0x24);
    key_b.wrapping_sub(key_a).leading_zeros() >> 5
}

// ===========================================================================
// Wave 7: value stepping, task registration and state descriptors
// (`0x8015F340..0x8015F9F0`).
//
// The neighbouring 0x8015F2xx/0x8015F6xx helpers below all share one idea: a
// 16- or 32-bit value is walked toward a target by a fixed step, or a small
// object is registered in a table.  They are the generic building blocks the
// task tables above are made of.
// ===========================================================================

/// Callback pointer read by [`invoke_task_callback`]
/// (`r13 - 0x6278` = `0x80429408`).
///
/// `// unknown:` what installs it; no writer appears in this band.
const G_TASK_CALLBACK: usize = SDA_BASE - 0x6278;

/// Monotonic descriptor id counter (`r13 - 0x5320` = `0x8042A360`), `s32`.
const G_STATE_DESCRIPTOR_COUNTER: usize = SDA_BASE - 0x5320;

/// Vtable installed by [`state_descriptor_init`] (`DAT_80328A18`).
const VTABLE_STATE_DESCRIPTOR: u32 = 0x8032_8A18;

/// Vtable installed by [`state_object_init`] (`DAT_80328A40`).
const VTABLE_STATE_OBJECT: u32 = 0x8032_8A40;

/// Embedded sub-object pointer written by [`state_object_init`]
/// (`DAT_80377A04`).
const STATE_OBJECT_SUB: u32 = 0x8037_7A04;

/// `r2 - 0x5488` = `0x8042DBF8`: the float constant compared against the step
/// in [`step_float_clamped`].
///
/// `// inferred:` it holds `0.0f` - the routine refuses a zero step and
/// compares the overshoot product against it.  The value is loaded at run time,
/// so the comparison follows the image either way.
const FLOAT_ZERO: usize = 0x8042_DBF8;

/// Moves a `i16` toward `target` by at most `step`, returning `1` on arrival.
///
/// In plain English: if the value already equals the target, report success.
/// Otherwise add the step (negated when the target is below the current value)
/// and, when the step has reached or passed the target, snap to it.
///
/// # Original
/// - Ghidra: `FUN_8015f340` at `0x8015F340` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f340_FUN_8015f340.c`
/// - Thunk: `0x8015F3B0` is a four-byte thunk to the sibling `0x8015F3C0`.
///
/// # Arguments
/// * `value` - in/out 16-bit value (a fixed-point angle at the observed call
///   sites, e.g. `0x8015F340(player + 0x102, angle, 0x800)`).
/// * `target` - value to reach.
/// * `step` - magnitude added per call; `0` disables the update.
///
/// # Returns
/// `1` when the value is at the target after the update, `0` otherwise.
///
/// # Side effects
/// Writes the updated value through `value`.
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - every branch and the two truncations are reproduced.
pub unsafe fn step_short_clamped(value: *mut i16, target: i16, step: i16) -> u32 {
    let current = platform::read_u16(value as usize) as i16;
    let target_wide = target as i32;
    if current as i32 == target_wide {
        return 1;
    }

    let mut delta = step as i32;
    if delta != 0 {
        if target_wide < current as i32 {
            delta = -delta;
        }
        // observed: `short` add, then `short` step sign-extended again.
        let next = current.wrapping_add(delta as i16);
        platform::write_u16(value as usize, (next) as u16);
        if delta.wrapping_mul((next as i32).wrapping_sub(target_wide)) >= 0 {
            platform::write_u16(value as usize, (target) as u16);
            return 1;
        }
    }
    0
}

/// Moves an `i32` toward `target` by at most `step`, returning `1` on arrival.
///
/// # Original
/// - Ghidra: `FUN_8015f3c0` at `0x8015F3C0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f3c0_FUN_8015f3c0.c`
/// - Thunk: `0x8015F3B0` jumps here (four bytes).
///
/// # Arguments
/// * `value` - in/out 32-bit value.
/// * `target` - value to reach.
/// * `step` - magnitude added per call; `0` disables the update.
///
/// # Returns
/// `1` when the value is at the target after the update, `0` otherwise.
///
/// # Side effects
/// Writes the updated value through `value`.
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - straightforward integer loop; wrapping arithmetic matches PPC.
pub unsafe fn step_int_clamped(value: *mut i32, target: i32, step: i32) -> u32 {
    let current = platform::read_u32(value as usize) as i32;
    if current == target {
        return 1;
    }

    let mut delta = step;
    if delta != 0 {
        if target < current {
            delta = delta.wrapping_neg();
        }
        let next = current.wrapping_add(delta);
        platform::write_u32(value as usize, (next) as u32);
        if delta.wrapping_mul(next.wrapping_sub(target)) >= 0 {
            platform::write_u32(value as usize, (target) as u32);
            return 1;
        }
    }
    0
}

/// Second `i32` copy of [`step_int_clamped`].
///
/// `// observed:` the body is byte-for-byte the same logic as `0x8015F3C0`.
/// The two entries exist because the original has two `i32` call sites; the
/// four-byte thunk at `0x8015F410` jumps here.
///
/// # Original
/// - Ghidra: `FUN_8015f420` at `0x8015F420` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f420_FUN_8015f420.c`
/// - Thunk: `0x8015F410` jumps here (four bytes).
///
/// # Arguments
/// * `value` - in/out 32-bit value.
/// * `target` - value to reach.
/// * `step` - magnitude added per call; `0` disables the update.
///
/// # Returns
/// `1` when the value is at the target after the update, `0` otherwise.
///
/// # Side effects
/// Writes the updated value through `value`.
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - identical to the previous body.
pub unsafe fn step_int_clamped_b(value: *mut i32, target: i32, step: i32) -> u32 {
    let current = platform::read_u32(value as usize) as i32;
    if current == target {
        return 1;
    }

    let mut delta = step;
    if delta != 0 {
        if target < current {
            delta = delta.wrapping_neg();
        }
        let next = current.wrapping_add(delta);
        platform::write_u32(value as usize, (next) as u32);
        if delta.wrapping_mul(next.wrapping_sub(target)) >= 0 {
            platform::write_u32(value as usize, (target) as u32);
            return 1;
        }
    }
    0
}

/// Moves an `f32` toward `target` by `step`, returning `1` on arrival.
///
/// In plain English: the value and the target are compared in double
/// precision; when the step is not the zero constant, add it (negated when the
/// target is below), store the single-precision result and snap to the target
/// once the signed overshoot product is non-negative.
///
/// # Original
/// - Ghidra: `FUN_8015f480` at `0x8015F480` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f480_FUN_8015f480.c`
/// - Thunk: `0x8015F470` jumps here (four bytes).
///
/// # Arguments
/// * `target` - value to reach (double register `f1`).
/// * `step` - magnitude added per call (double register `f2`); compared against
///   [`FLOAT_ZERO`].
/// * `value` - in/out single-precision value (first integer argument, `r3`).
///
/// # Returns
/// `1` when the value is at the target after the update, `0` otherwise.
///
/// # Side effects
/// Writes the updated value through `value`.
///
/// # Safety
/// `value` must be a valid writable pointer; the float constant at
/// `0x8042DBF8` must be readable.
///
/// # Confidence
/// medium - the cast chain is reproduced exactly, but the register split
/// (two float arguments + one pointer) is inferred from the Ghidra prototype
/// and the `r2` constant load.
pub unsafe fn step_float_clamped(target: f64, step: f64, value: *mut f32) -> u32 {
    let zero = platform::read_f32(FLOAT_ZERO);

    let current = platform::read_f32(value as usize);
    if current as f64 == target {
        return 1;
    }

    if step != zero as f64 {
        let mut delta = step;
        if target < current as f64 {
            delta = -delta;
        }

        // observed: `(float)((double)current + step)` is stored first.
        let next = ((current as f64) + delta) as f32;
        platform::write_f32(value as usize, next);

        // observed: `(float)(step * (double)(float)((double)next - target))`.
        let overshoot = ((next as f64) - target) as f32;
        let product = (delta * (overshoot as f64)) as f32;
        if zero <= product {
            platform::write_f32(value as usize, target as f32);
            return 1;
        }
    }
    0
}

/// Moves an `i16` toward a wide target with a wrapped `i16` step.
///
/// Differences from [`step_short_clamped`], both observed: the arrival test
/// compares the value against the *untruncated* `i32` target, and the step is
/// truncated to `i16` before it is negated/stored, which is why the original
/// works on `short` arithmetic while the caller passes an `int`.
///
/// # Original
/// - Ghidra: `FUN_8015f4f0` at `0x8015F4F0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f4f0_FUN_8015f4f0.c`
///
/// # Arguments
/// * `value` - in/out 16-bit value.
/// * `target` - target held in a 32-bit register, truncated for the update.
/// * `step` - step held in a 32-bit register, truncated to `i16`.
///
/// # Returns
/// `1` when the value is at the target after the update, `0` otherwise.
///
/// # Side effects
/// Writes the updated value through `value`.
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - all four truncations are visible in the C.
pub unsafe fn step_short_clamped_wrap(value: *mut i16, target: i32, step: i32) -> u32 {
    if *value as i32 == target {
        return 1;
    }

    let mut delta = step;
    if delta != 0 {
        let target_short = target as i16;
        if ((platform::read_u16(value as usize) as i16 as i32).wrapping_sub(target_short as i32) as i16) > 0 {
            delta = -((delta as i16) as i32);
        }
        let next = (platform::read_u16(value as usize) as i16).wrapping_add(delta as i16);
        platform::write_u16(value as usize, (next) as u16);
        let diff = (next as i32).wrapping_sub(target_short as i32) as i16;
        if delta.wrapping_mul(diff as i32) >= 0 {
            platform::write_u16(value as usize, (target_short) as u16);
            return 1;
        }
    }
    0
}

/// Initialises a task-list header.
///
/// In plain English: store the table pointer, the entry count and reset the
/// current index to zero - the exact three fields [`run_task_list`] walks.
///
/// # Original
/// - Ghidra: `FUN_8015f600` at `0x8015F600` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f600_FUN_8015f600.c`
///
/// # Arguments
/// * `header` - writable header address.
/// * `table` - pointer to the array of task functions.
/// * `count` - number of entries in the table.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x00` (table), `+0x04` (`u16` count), `+0x06` (`u16` index).
///
/// # Safety
/// `header` must be writable for 8 bytes.
///
/// # Confidence
/// high - the three stores are the whole body.
pub unsafe fn task_list_init(header: usize, table: u32, count: u16) {
    platform::write_u32(header, table);
    platform::write_u16(header + 4, count);
    platform::write_u16(header + 6, 0);
}

/// Returns the installed global task callback.
///
/// # Original
/// - Ghidra: `FUN_8015f6d0` at `0x8015F6D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f6d0_FUN_8015f6d0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The word at `0x80429408` (`r13 - 0x6278`); `0` when nothing installed.
///
/// # Side effects
/// None.
///
/// # Safety
/// None beyond the target address space.
///
/// # Confidence
/// high - single load; the `// unknown:` writer is outside this batch.
pub unsafe fn task_callback() -> u32 {
    platform::read_u32(G_TASK_CALLBACK)
}

/// Forwards two arguments to the global task callback.
///
/// # Original
/// - Ghidra: `FUN_8015f6e0` at `0x8015F6E0` (76 bytes, including the tail of
///   the save/restore sequence)
/// - Evidence: `work/ghidra/main-gb/export/8015f6e0_FUN_8015f6e0.c`
/// - Thunk: `0x8015F7B0` jumps here.
///
/// # Arguments
/// * `a`, `b` - forwarded unchanged to the callback.
///
/// # Returns
/// Nothing; the callback's result is discarded.
///
/// # Side effects
/// Calls whatever [`task_callback`] returns (no null check in the original).
///
/// # Safety
/// A callback must have been installed; otherwise this calls address `0`.
///
/// # Confidence
/// high - one indirect call with both registers visible.
pub unsafe fn invoke_task_callback(a: u32, b: u32) {
    let callback = task_callback() as usize;
    platform::call2(callback, a, b);
}

/// Constructs a state-descriptor object and assigns it a unique id.
///
/// In plain English: install the descriptor vtable, store the name pointer, and
/// - only when the name is non-null - take the next value of a global counter
/// and store it as the descriptor id.
///
/// # Original
/// - Ghidra: `FUN_8015f7c0` at `0x8015F7C0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f7c0_FUN_8015f7c0.c`
///
/// # Arguments
/// * `this` - writable descriptor (three words).
/// * `name` - name/string pointer; `0` means "unnamed" and keeps id `0`.
///
/// # Returns
/// Nothing (the id is written to `this + 8`).
///
/// # Side effects
/// Writes `+0x00` vtable `0x80328A18`, `+0x04` name, `+0x08` id; increments
/// `0x8042A360` when `name != 0`.
///
/// # Safety
/// `this` must be writable for 0x0C bytes.
///
/// # Confidence
/// high - the counter increment and the `name == 0` bypass are explicit.
pub unsafe fn state_descriptor_init(this: usize, name: u32) {
    platform::write_u32(this, VTABLE_STATE_DESCRIPTOR);
    platform::write_u32(this + 4, name);

    let id = if name == 0 {
        0
    } else {
        let next = platform::read_i32(G_STATE_DESCRIPTOR_COUNTER).wrapping_add(1);
        platform::write_i32(G_STATE_DESCRIPTOR_COUNTER, next);
        next
    };
    platform::write_u32(this + 8, id as u32);
}

/// Releases a state-descriptor object.
///
/// # Original
/// - Ghidra: `FUN_8015f800` at `0x8015F800` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f800_FUN_8015f800.c`
///
/// # Arguments
/// * `this` - object or `0`.
/// * `free` - `> 0` asks `0x802B9280` to free the block.
///
/// # Returns
/// `this`, so the caller can chain the release.
///
/// # Side effects
/// Possible `0x802B9280(this)` free.
///
/// # Safety
/// `this` must be live or `0`.
///
/// # Confidence
/// medium - Ghidra prints `FUN_802b9280()` with no argument; `r3` still holds
/// `this`, so it is passed here (`// inferred:`).
pub unsafe fn state_descriptor_release(this: usize, free: i32) -> usize {
    if this != 0 && free > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Reads the descriptor id assigned by [`state_descriptor_init`].
///
/// # Original
/// - Ghidra: `FUN_8015f930` at `0x8015F930` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f930_FUN_8015f930.c`
///
/// # Arguments
/// * `this` - descriptor object.
///
/// # Returns
/// The word at `this + 8`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `this` must be readable.
///
/// # Confidence
/// high - single load.
pub unsafe fn state_descriptor_id(this: usize) -> u32 {
    platform::read_u32(this + 8)
}

/// Constructs the larger state object used by the task tables.
///
/// In plain English: install the vtable at `+0x00`, copy the name at `+0x04`
/// and the two caller values, clear the five flag bytes at `+0x0C..+0x11`,
/// then seed the embedded sub-object at `+0x14..+0x20`.
///
/// # Original
/// - Ghidra: `FUN_8015f9a0` at `0x8015F9A0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f9a0_FUN_8015f9a0.c`
///
/// # Arguments
/// * `this` - writable object (0x24 bytes).
/// * `name` - name/string pointer (`+0x04`).
/// * `value` - word stored at `+0x08` (`// unknown:` role).
/// * `sub` - word stored at `+0x14`, `+0x1C` and used to seed `+0x18`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the vtable `0x80328A40`, clears `+0x0C..+0x10`, writes
/// `DAT_80377A04` at `+0x18` and zero at `+0x20`.
///
/// # Safety
/// `this` must be writable for 0x24 bytes.
///
/// # Confidence
/// high - the byte/word split is visible in the C's pointer arithmetic.
pub unsafe fn state_object_init(this: usize, name: u32, value: u32, sub: u32) {
    platform::write_u32(this + 4, name);
    platform::write_u32(this, VTABLE_STATE_OBJECT);
    platform::write_u32(this + 8, value);
    platform::write_u8(this + 0x0C, 0);
    platform::write_u8(this + 0x0D, 0);
    platform::write_u8(this + 0x0E, 0);
    platform::write_u8(this + 0x0F, 0);
    platform::write_u8(this + 0x10, 0);
    platform::write_u32(this + 0x14, sub);
    platform::write_u32(this + 0x18, STATE_OBJECT_SUB);
    platform::write_u32(this + 0x1C, sub);
    platform::write_u32(this + 0x20, 0);
}

/// Releases the larger state object.
///
/// # Original
/// - Ghidra: `FUN_8015f9f0` at `0x8015F9F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f9f0_FUN_8015f9f0.c`
///
/// # Arguments
/// * `this` - object or `0`.
/// * `free` - `> 0` asks `0x802B9280` to free the block.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Possible `0x802B9280(this)` free.
///
/// # Safety
/// `this` must be live or `0`.
///
/// # Confidence
/// medium - same dropped-argument call shape as
/// [`state_descriptor_release`]; `this` is passed in `r3` (`// inferred:`).
pub unsafe fn state_object_release(this: usize, free: i32) -> usize {
    if this != 0 && free > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

// ===========================================================================
// Wave 292 lane D (`_D292`) - four open slots of the 0x8032269C state vtable
// ===========================================================================
//
// `0x8032269C` is the 12-slot main-DOL vtable installed on the eight
// `dGoalManager_c` state records built by [`crate::game::worldmap::fn_8010b890`]
// (`0x8010B96C: addi r30,r30,0x269C`, stored at record `+0x00`). Its two header
// words are zero (`0x8032269C`, `0x803226A0`), the method array is
// `0x803226A4..0x803226D4` (12 words, 48 bytes), and the next word is the data
// constant `0x02CF02CF`, so the run has no zero terminator. Slot `n` sits at
// vtable offset `0x08 + 4n`.
//
// The base table is `0x80328A18`, installed by [`state_descriptor_init`]
// (`0x8015F7C0`): the derived table keeps slots 1, 2, 3, 4, 6 and 7 unchanged
// and overrides slot 0 (`0x8010BF30`), slot 5 (`0x8010BF90`) and the four
// previously empty slots 8..11 (`0x8010B860`, `0x8010B830`, `0x8010B800`,
// `0x8010C020`). The four bodies below are the inherited half that lives in
// this module; the worldmap half is under the same wave tag in
// `src/game/worldmap.rs`.
//
// Seven targets were open before this pass; four were prologue-shaped
// (`0x8015F8D0` here and `0x8010BF90`/`0x8010B800`/`0x8010C020` in
// `game::worldmap`). The four here are two 16-byte leaves, one 52-byte wrapper
// and one 8-byte getter.

// ---------------------------------------------------------------------------
// slot 1 - 0x8015F840
// ---------------------------------------------------------------------------

/// Reports whether a state descriptor has no assigned id.
///
/// In plain English: [`state_descriptor_init`] only takes a counter value when
/// the name pointer is non-null, so a descriptor whose `+0x08` word is still
/// zero was created without a name. The body folds `cntlzw(id) >> 5`, which is
/// `1` exactly for `id == 0`.
///
/// # Original (untabled)
/// - Address: `0x8015F840`, 16 B (`0x8015F840..0x8015F850`; the next word is
///   the already-translated [`keys_equal`])
/// - Table: slot 1 of `0x8032269C`, vtable offset `+0x0C`
/// - Not in `functions.tsv`; no Ghidra export
/// - observed: `80030008 lwz r0,8(r3)` / `7c000034 cntlzw r0,r0` /
///   `5403d97e rlwinm r3,r0,27,5,31` / `4e800020 bclr 20,0`
///
/// # Arguments
/// * `this` - state descriptor; `+0x08` is the id written by
///   [`state_descriptor_init`].
///
/// # Returns
/// `1` when the id is zero, `0` otherwise.
///
/// # Side effects
/// None; the body is a single load.
///
/// # Safety
/// `this` must be readable through `+0x0B`.
///
/// # Confidence
/// high - `cntlzw(x) >> 5` is the canonical `x == 0` predicate and the
/// `rlwinm` mask is literal.
pub unsafe fn fn_8015f840(this: usize) -> u32 {
    let id = platform::read_u32(this + 8);
    id.leading_zeros() >> 5
}

// ---------------------------------------------------------------------------
// slot 3 - 0x8015F8C0
// ---------------------------------------------------------------------------

/// Tail-forwards the identity comparison to vtable slot 2.
///
/// The four instructions reload the object's vtable, take `+0x10` (slot 2,
/// [`keys_equal`]) and `bcctr 20,0` without link, so both arguments reach the
/// callee unchanged and its result is this slot's result.
///
/// # Original (untabled)
/// - Address: `0x8015F8C0`, 16 B (`0x8015F8C0..0x8015F8D0`)
/// - Table: slot 3 of `0x8032269C`, vtable offset `+0x14`
/// - Not in `functions.tsv`; no Ghidra export
/// - observed: `81830000 lwz r12,0(r3)` / `818c0010 lwz r12,0x10(r12)` /
///   `7d8903a6 mtctr r12` / `4e800420 bcctr 20,0`
///
/// # Arguments
/// * `this` - left state descriptor.
/// * `other` - right state descriptor.
///
/// # Returns
/// `1` when the two ids are equal, `0` otherwise.
///
/// # Side effects
/// Only the callee's effects.
///
/// # Safety
/// Both objects must expose vtable slot `+0x10` and accept the two arguments.
///
/// # Confidence
/// high - the slot offset and the link-less `bcctr` are literal.
pub unsafe fn fn_8015f8c0(this: usize, other: usize) -> u32 {
    let vtable = platform::read_u32(this) as usize;
    let compare = platform::read_u32(vtable + 0x10) as usize;
    platform::call2(compare, this as u32, other as u32)
}

// ---------------------------------------------------------------------------
// slot 4 - 0x8015F8D0
// ---------------------------------------------------------------------------

/// Returns the negation of the slot-2 identity comparison.
///
/// In plain English: the body calls vtable slot 2 ([`keys_equal`]) with both
/// arguments and folds the result with `cntlzw(x) >> 5`, so equal ids report
/// `0` and different ids report `1`.
///
/// # Original (untabled)
/// - Address: `0x8015F8D0`, 52 B (`0x8015F8D0..0x8015F904`; three zero words,
///   then the 8-byte base slot-5 body `0x8015F910`, two zero words and the
///   already-translated [`fn_8015f920`])
/// - Table: slot 4 of `0x8032269C`, vtable offset `+0x18`
/// - Not in `functions.tsv`; no Ghidra export
/// - observed: `9421fff0 stwu sp,-16(sp)` / `7c0802a6 mflr r0` /
///   `90010014 stw r0,0x14(sp)` / `81830000 lwz r12,0(r3)` /
///   `818c0010 lwz r12,0x10(r12)` / `7d8903a6 mtctr r12` /
///   `4e800421 bcctr 20,0l` / `7c600034 cntlzw r0,r3` /
///   `5403d97e rlwinm r3,r0,27,5,31` / `4e800020 bclr 20,0`
///
/// # Arguments
/// * `this` - left state descriptor.
/// * `other` - right state descriptor.
///
/// # Returns
/// `0` when the two ids are equal, `1` otherwise.
///
/// # Side effects
/// Only the callee's effects.
///
/// # Safety
/// Both objects must expose vtable slot `+0x10` and accept the two arguments.
///
/// # Confidence
/// high - the call, the negation fold and the frame are literal.
pub unsafe fn fn_8015f8d0(this: usize, other: usize) -> u32 {
    let vtable = platform::read_u32(this) as usize;
    let compare = platform::read_u32(vtable + 0x10) as usize;
    let equal = platform::call2(compare, this as u32, other as u32);
    equal.leading_zeros() >> 5
}

// ---------------------------------------------------------------------------
// slot 6 - 0x8015F920
// ---------------------------------------------------------------------------

/// Reads the descriptor's name pointer.
///
/// # Original (untabled)
/// - Address: `0x8015F920`, 8 B (`0x8015F920..0x8015F928`; two zero words
///   then the already-translated [`state_descriptor_id`] at `0x8015F930`)
/// - Table: slot 6 of `0x8032269C`, vtable offset `+0x20`
/// - Not in `functions.tsv`; no Ghidra export
/// - observed: `80630004 lwz r3,4(r3)` / `4e800020 bclr 20,0`
///
/// # Arguments
/// * `this` - state descriptor.
///
/// # Returns
/// The word at `this + 4`, the name pointer stored by
/// [`state_descriptor_init`].
///
/// # Side effects
/// None.
///
/// # Safety
/// `this` must be readable through `+0x07`.
///
/// # Confidence
/// high - a single load and `blr`.
pub unsafe fn fn_8015f920(this: usize) -> u32 {
    platform::read_u32(this + 4)
}
