//! # Scene and state framework
//!
//! This module is the semantic home for the engine's **scene lifecycle** and for
//! the **boot/save state steps** that run before the title screen.  Everything
//! here was read from the uncovered `main.dol` band `0x800C0000..0x80100000`.
//!
//! ## What is in this file
//!
//! There are two clusters, both owned by `game::scene`:
//!
//! | addresses | size | role |
//! |---|---|---|
//! | `0x800E1840..0x800E1F68` | 14 functions, 1,424 bytes | base scene object (construct/destroy/finished/update), the scene-transition globals (`request -> commit -> construct next`), the lifecycle helpers (`0x800E1D20..0x800E1DD0`) and the scene sound dispatch (`0x800E2220`, `0x800E2340`) |
//! | `0x800CEC10..0x800CFB30` (last function ends `0x800CFBA8`) | 17 functions, 3,632 bytes | the boot/save worker: a task object, a NAND request hand-off, and the save-file state steps `check -> space -> read/write -> verify -> delete` plus the two save-record helpers at `0x800E1840`/`0x800E1940` |
//!
//! The four functions at `0x800C9F10..0x800C9F90` (`game_startup`, `frame_loop`,
//! `game_main`, `start_main_thread`) belong to [`crate::engine::startup`] and are
//! deliberately **not** duplicated here.
//!
//! ## Why it exists
//!
//! `docs/ENGINE_STARTUP.md` ends the boot story at "the game thread runs one
//! frame forever".  That frame body calls a scene update
//! (`FUN_8015F620(r13-0x55E0, 0)`), and the boot scene
//! (`dScBoot_c`, `0x8015DDD0`) drives the NAND/save steps that are translated
//! here.  Without this module the boot-to-title flow stops at a call address.
//!
//! ## Start here
//!
//! 1. [`scene_request`] - ask for a scene id; this is the public transition
//!    entry point used by gameplay code (`0x800BB780`, `0x801017D0`,
//!    `0x80102260`, `0x80102690`).
//! 2. [`scene_transition_step`] - runs once per frame while a requested scene
//!    is waiting to be constructed.
//! 3. [`scene_base_update`] - the base scene's per-frame update; when its state
//!    table returns `2` the scene is finished.
//!
//! For the boot path start at [`save_task_create`], then read
//! [`save_task_wait_and_start`] and [`save_check_files`].
//!
//! ## State-table layout (observed, framework not owned here)
//!
//! `0x8015F620` is the engine's generic state-table runner.  It is called by
//! [`scene_base_update`] and by the frame loop, but it lives outside this band,
//! so it is called by address and not translated:
//!
//! ```text
//! +0x00  u32  state handler table (array of function pointers)
//! +0x04  u16  state count
//! +0x06  u16  current state index
//! ```
//!
//! The runner calls `table[current](arg)`.  A handler returns `1` to advance to
//! the next state and `2` when the whole chain is finished; any other value is
//! returned to the caller.  This is `// observed:` from `8015f620_FUN_8015f620.c`
//! and from the `FUN_8015F600` initialiser (`*table`, `count`, `index = 0`).
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! * OS/NW4R, called by absolute address, **not** translated here:
//!   `0x801B4B30` current thread, `0x801B5F70` base priority,
//!   `0x801B5860` resume thread, `0x801B3080` queue count,
//!   `0x801B2F40` queue reset, `0x801B2E20`/`0x801B3140` task sync objects,
//!   `0x8015F130` checksum (thunk of `0x801B8620`),
//!   `0x8016E4F0`/`0x802B9210` allocator lock/alloc,
//!   `0x802BA3B0` task constructor, `0x802DCD74` task factory.
//! * NAND/save helpers, called by absolute address, **not** translated here:
//!   `0x801D84E0` open, `0x801D9950` read, `0x801D9F60` close,
//!   `0x801D8AE0` decode, `0x801D9D10` finish, `0x801D9040` size,
//!   `0x801D89F0` compare, `0x801DAE70` file check, `0x801DB140` space check,
//!   `0x801D87E0` delete, `0x801DAA...` banner helpers.
//! * Scene helpers: `0x8015F620` state runner, `0x8006C6C0` request validation
//!   (thunk of `0x80162B20`), `0x80162A20`/`0x80162510` scene flags,
//!   `0x8006A620` global scene id, `0x8006C420`/`0x8006C490`/`0x8006C540`/
//!   `0x8006C5C0` base object.
//!
//! ## Register baseline (why `r13` addresses are absolute here)
//!
//! `0x800041C0` (`__init_registers`) sets:
//!
//! * `r13 = 0x8042F680` - the small-data base; every `r13-offset` below is
//!   resolved against it (see [`SDA_BASE`]).
//! * `r2 = 0x80433080` - the second small-data base.  None of the functions in
//!   this file read `r2`; the neighbouring float-only helpers that do are out of
//!   scope.
//!
//! ## Glossary (first use)
//!
//! * **DOL** - the Wii executable container (`sys/main.dol`); image base
//!   `0x80004000`, so all addresses here are virtual image addresses.
//! * **scene** - one top-level game state (boot, title, world map, course).
//! * **state table** - the `u32 table + u16 count + u16 index` record described
//!   above; each entry is a callback.
//! * **NAND** - the Wii's internal flash, reached through the SDK save-file API.
//! * **MEM1/MEM2** - the Wii's two RAM regions; the save buffer is in BSS in
//!   MEM1 (`0x8035DCC0`).
//! * **r13 / SDA** - see the register baseline above.
//!
//! ## Confidence summary
//!
//! 31 functions, all `translated`.  The three entries that were `partial` after
//! wave 242 (`0x800CF410`, `0x800CF290`, `0x800E19B0`) were settled by reading
//! the dropped operands out of the instruction stream: the preserved `r4`, the
//! error-path `r4 = r3`, and the two helper receivers (`this` and `0x8031AFC0`).
//! All call targets and all observable stores are represented; names are
//! descriptive where the strings/call sites prove the role, and stay
//! `fn_...`-style by address where they do not.
//!
//! The `0x800E1840`/`0x800E1940` pair and the five `0x800E1D20..0x800E2340`
//! helpers were added in the wave-7 pass; see
//! `docs/COVERAGE_WAVE7_SCENE.md` for the per-function table and the open
//! questions (`extraout_r4` on `0x800E1840`, the no-argument vtable/fade calls).

use crate::platform;

/// Small-data anchor: `r13` at the game entry.
///
/// `0x800041C0` loads `0x8042F680` into `r13`; every SDA global in this file is
/// resolved as `SDA_BASE - displacement`.
///
/// # Original
/// - Ghidra: `FUN_800041c0` at `0x800041C0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800041c0_FUN_800041c0.c`
/// - Observed: `0x80004244 lis r13,0x8042` / `0x80004248 ori r13,r13,0xF680`
pub const SDA_BASE: usize = 0x8042_F680;

// ---------------------------------------------------------------------------
// Scene-transition globals (`r13 - 0x6F50 .. r13 - 0x6F44`).
// ---------------------------------------------------------------------------

/// Current scene id (`r13 - 0x6F50` = `0x80428730`), `u16`.
///
/// `0x2EE` is the "no scene / transition slot free" sentinel: [`scene_request`]
/// only accepts a new id while this word is `0x2EE`, and
/// [`scene_transition_step`] swaps the requested id in and stores `0x2EE` back
/// once the scene object exists.
const G_SCENE_CURRENT: usize = SDA_BASE - 0x6F50;

/// Previous scene id (`r13 - 0x6F4E` = `0x80428732`), `u16`.
const G_SCENE_PREVIOUS: usize = SDA_BASE - 0x6F4E;

/// Scene id before the previous one (`r13 - 0x6F4C` = `0x80428734`), `u16`.
const G_SCENE_OLDER: usize = SDA_BASE - 0x6F4C;

/// One-byte "transition in progress / already swapped" flag
/// (`r13 - 0x6F4A` = `0x80428736`).
const G_SCENE_TRANSITION_FLAG: usize = SDA_BASE - 0x6F4A;

/// Auxiliary id A (`r13 - 0x6F48` = `0x80428738`), `u16`.
///
/// Consumed by [`scene_base_update`] when the scene reports finished,
/// together with [`G_SCENE_PREVIOUS`].  `// inferred:` a fade-out scene id;
/// the only caller found in the export, `0x8015C4C0`, writes `0x1E` (30) here.
const G_SCENE_AUX_A: usize = SDA_BASE - 0x6F48;

/// Auxiliary id B (`r13 - 0x6F46` = `0x8042873A`), `u16`.
///
/// Consumed by [`scene_base_update`] on the alternate branch.  `// inferred:`
/// fade duration/id paired with [`G_SCENE_AUX_A`].
const G_SCENE_AUX_B: usize = SDA_BASE - 0x6F46;

/// "A scene has finished" flag (`r13 - 0x6F44` = `0x8042873C`), byte.
const G_SCENE_FINISHED_FLAG: usize = SDA_BASE - 0x6F44;

/// Last argument passed to [`scene_request`] (`r13 - 0x5638` = `0x8042A048`).
const G_SCENE_ARGUMENT: usize = SDA_BASE - 0x5638;

/// Scene-manager singleton pointer (`r13 - 0x5240` = `0x8042A440`).
///
/// The pointee's first word is its vtable; `vtable + 0x10` is the no-argument
/// query used by [`scene_base_update`].
const G_SCENE_MANAGER_PTR: usize = SDA_BASE - 0x5240;

/// Special-case flag (`r13 - 0x5463` = `0x8042A21D`), byte.
///
/// Set when leaving scene `5` for any other scene; `// unknown:` consumer.
const G_SCENE_LEAVE_FIVE_FLAG: usize = SDA_BASE - 0x5463;

/// Scene reference/count word (`r13 - 0x53A0` = `0x8042A2E0`), `s32`.
const G_SCENE_REFCOUNT: usize = SDA_BASE - 0x53A0;

/// Bytes cleared by [`scene_base_construct`] (`r13 - 0x5787`,
/// `r13 - 0x5988..0x5985`).
const G_SCENE_FLAG_5787: usize = SDA_BASE - 0x5787; // 0x80429EF9
const G_SCENE_FLAG_5988: usize = SDA_BASE - 0x5988; // 0x80429CF8

/// `dScBoot_c`'s base-object vtable installed at `this + 0x60`.
///
/// `// observed:` `0x8015C130` calls [`scene_base_construct`] and immediately
/// overwrites this slot with its derived vtable, which proves the base layout.
const VTABLE_BASE_SCENE: u32 = 0x8031_AFC0;

// ---------------------------------------------------------------------------
// Boot/save object globals.
// ---------------------------------------------------------------------------

/// Current save-task object (`r13 - 0x56C8` = `0x80429FB8`).
const G_SAVE_TASK: usize = SDA_BASE - 0x56C8;

/// Source save manager (`r13 - 0x5640` = `0x8042A040`).
///
/// [`save_task_construct`] copies its `+0x20` header and two arrays of three
/// 0x980-byte records out of it.
const G_SAVE_SOURCE: usize = SDA_BASE - 0x5640;

/// Resource manager pointer (`r13 - 0x5648` = `0x8042A038`); the archive handle
/// is `*(ptr) + 4`.
const G_RESOURCE_MANAGER: usize = SDA_BASE - 0x5648;

/// Save-manager value published by [`save_manager_init`]
/// (`r13 - 0x59C0` = `0x80429CC0`).
///
/// `0x800CE110` returns it and the init routine stores it once.  `// unknown:`
/// the producer's role; it is not read anywhere in this band.
const G_SAVE_MANAGER_STATE: usize = SDA_BASE - 0x59C0;

/// Five 0x10-byte lookup tables filled by [`save_manager_init`].
///
/// `// observed:` the loop writes four entries into each table; the first is a
/// byte array indexed by the *record* index (`0x800CE070`), the other four are
/// word arrays indexed by the loop counter.
const SAVE_TABLE_SLOT_ORDER: usize = 0x8035_4E60;
const SAVE_TABLE_A: usize = 0x8035_4E70;
const SAVE_TABLE_B: usize = 0x8035_4E80;
const SAVE_TABLE_C: usize = 0x8035_4E90;
const SAVE_TABLE_D: usize = 0x8035_4EA0;

/// Scene/sound helper tables (`DAT_802f2f08`, `0x802f2f30`, `0x802f2f58`,
/// `0x802f2f80`).
///
/// Each is indexed by a small scene/sound id; the values are `// unknown:`
/// engine ids forwarded to `0x800B3540`/`0x80060690`.
const SOUND_TABLE_CUE: usize = 0x802F_2F08;
const SOUND_TABLE_SMALL: usize = 0x802F_2F30;
const SOUND_TABLE_REVERB: usize = 0x802F_2F58;
const SOUND_TABLE_CUE_B: usize = 0x802F_2F80;

/// Third banner-name pointer (`r13 - 0x7A18` = `0x80427C68`).
const G_BANNER_NAME_ALT: usize = SDA_BASE - 0x7A18;

/// Byte used by the save-file check (`r13 - 0x7A14..0x7A11` =
/// `0x80427C6C..0x80427C6F`).
///
/// `// unknown:` `0x800CF590` compares these four bytes against the first four
/// bytes of the save buffer, so they are probably a format/version stamp.
const G_SAVE_STAMP: usize = SDA_BASE - 0x7A14;

/// Global 0x3FA0-byte save file image in BSS.
const G_SAVE_BUFFER: usize = 0x8035_DCC0;

/// Banner staging area used by [`save_build_banner_buffer`].
const G_BANNER_WORK: usize = 0x8036_1C60;

/// First banner image destination inside [`G_BANNER_WORK`] (`+0xA0`).
const G_BANNER_IMAGE: usize = 0x8036_1D00;

/// Second banner image destination (`DAT_80367D00`).
const G_BANNER_ICON: usize = 0x8036_7D00;

/// Header field `+0x08` of [`G_BANNER_WORK`] (`DAT_80361C68`).
const G_BANNER_HEADER_FLAGS: usize = 0x8036_1C68;

/// Save-file path strings already in the image.
const STR_SAVE_FILE: u32 = 0x802F_118C; // "wiimj2d.sav"
const STR_BANNER_FILE: u32 = 0x802F_1180; // "banner.bin"
const STR_BANNER_TMP: u32 = 0x802F_1170; // "/tmp/banner.bin"
const STR_SAVE_BANNER: u32 = 0x8031_79E4; // "save_banner"
const STR_SAVE_BANNER_BTI: u32 = 0x8031_79D0; // "save_banner_US.bti"

// ---------------------------------------------------------------------------
// Local signature helper.
//
// `crate::platform` exposes returning variants for 1..7 arguments, but not a
// zero-argument function that returns a value.  The task/allocator locks and
// the base-object queries below return `u32`, so they need one.
// ---------------------------------------------------------------------------

/// Calls a PowerPC function with no arguments and returns its `r3`.
///
/// # Original
/// Not present in the binary: local signature helper for the no-argument
/// calls whose Ghidra prototypes return a value, e.g. `FUN_802B9170` (task
/// allocator lock) at `0x800CFAB0` and `FUN_8006C540` at `0x800E1AA0`.
///
/// # Arguments
/// * `addr` - virtual address of the callee.
///
/// # Returns
/// The callee's `r3`.
///
/// # Side effects
/// Whatever the callee does.
///
/// # Safety
/// `addr` must point at a PowerPC function with this ABI and the caller must be
/// on a PowerPC target.
///
/// # Confidence
/// high - mechanical wrapper, same pattern as `crate::platform::call0`.
#[inline]
unsafe fn call0_ret(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

// ===========================================================================
// Boot/save worker: task object and NAND hand-off.
// ===========================================================================

/// Constructs the save worker task and seeds the global save image.
///
/// In plain English: allocate the task's sync objects, install its vtable, then
/// copy the live save manager's header and its two arrays of three course
/// records into the 0x3FA0-byte global save image.
///
/// # Original
/// - Ghidra: `FUN_800cec10` at `0x800CEC10` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cec10_FUN_800cec10.c`
/// - Caller: `0x800CFAB0` ([`save_task_create`])
/// - Removed: the modelled `0x802DCD74`/`0x802DCDC0` calls. They are the
///   `_savegpr_27`/`_restgpr_27` compiler helpers (register spill/restore
///   only) at `0x800CEC2C`/`0x800CED14`; `task` is the incoming `r3` and the
///   constructor's fourth argument is the incoming `r4`.
///
/// # Arguments
/// * `memory` - the 0x80-byte task block from `0x802B9210` (`r3`), which the
///   constructor initialises in place.
/// * `priority` - value from [`save_task_create`] (`r4`); forwarded as the
///   fourth argument of the constructor at `0x802BA3B0`.
/// * `arg` - third argument from [`save_task_create`] (`0`); goes to `r6` of the
///   task constructor at `0x802BA3B0`.
///
/// # Returns
/// The task object pointer (the original returns `r3`).
///
/// # Side effects
/// * initialises a sync object at `task + 0x54` and `task + 0x6C`;
/// * stores the task in `0x80429FB8` (`r13 - 0x56C8`);
/// * copies 0x3FA0 bytes of save data into `0x8035DCC0`.
///
/// # Safety
/// Must run once, before any save request.  `G_SAVE_SOURCE` must already point
/// at a live save manager.
///
/// # Confidence
/// high for the field offsets and copies; `medium` for the meaning of the two
/// sync objects (`// inferred:` queue/event pair).
pub unsafe fn save_task_construct(memory: usize, priority: u32, arg: u32) -> usize {
    let task = memory;

    // 0x800CEC18: task constructor.  `0x4000` is the worker stack size.
    platform::call5(
        0x802B_A3B0,
        task as u32,
        0x4000,
        0,
        priority,
        arg,
    );

    // 0x800CEC24..0x800CEC38: vtable plus two sync objects.
    platform::write_u32(task, 0x8031_7A30); // vtable A
    platform::write_u32(task + 0x50, 0x8031_7A54); // vtable B
    platform::call1(0x801B_2E20, (task + 0x54) as u32);
    platform::write_u32(task + 0x50, 0x8031_7A48);
    platform::call1(0x801B_3140, (task + 0x6C) as u32); // thunk_FUN_801b4b20
    platform::write_u32(task + 0x74, 0);

    let source = platform::read_u32(G_SAVE_SOURCE) as usize;
    platform::write_u32(G_SAVE_TASK, task as u32);

    // The original stack layout is one contiguous 0x3FC0-byte image:
    //   +0x000  header (0x6A0)
    //   +0x6A0  three 0x980-byte records from FUN_800E0380
    //   +0x2320 three 0x980-byte records from FUN_800E03B0
    //   +0x3FC0 end
    // Only the first 0x3FA0 bytes are copied to the global buffer (the last
    // 0x20 bytes are stack padding).
    let mut image = crate::platform::GuestLocal::new( [0u8; 0x3FC0]);

    // 0x800CEC4C: header from save manager + 0x20.
    platform::call3(
        0x8000_4364, // memcpy
        image.as_mut_ptr() as u32,
        (source + 0x20) as u32,
        0x6A0,
    );

    // 0x800CEC64..0x800CECB4: two arrays of three records.
    let mut index = 0u32;
    while index < 3 {
        let a = platform::call2(0x800E_0380, source as u32, index);
        platform::call3(
            0x8000_4364,
            image.as_mut_ptr().add(0x6A0 + index as usize * 0x980) as u32,
            a,
            0x980,
        );
        let b = platform::call2(0x800E_03B0, source as u32, index);
        platform::call3(
            0x8000_4364,
            image.as_mut_ptr().add(0x2320 + index as usize * 0x980) as u32,
            b,
            0x980,
        );
        index += 1;
    }

    // 0x800CECC4: publish to the global save image.
    platform::call3(
        0x8000_4364,
        G_SAVE_BUFFER as u32,
        image.as_ptr() as u32,
        0x3FA0,
    );

    task
}

/// Releases the save task's factory allocation.
///
/// # Original
/// - Ghidra: `FUN_800ced70` at `0x800CED70` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800ced70_FUN_800ced70.c`
///
/// # Arguments
/// * `task` - task pointer; `0` is accepted and returns immediately.
/// * `free` - `>0` asks `0x802B9280` to free the block.
///
/// # Returns
/// `task`, so the caller can chain the release.
///
/// # Side effects
/// Possible `0x802B9280(task)` free.
///
/// # Safety
/// `task` must be a live task or `0`.
///
/// # Confidence
/// high - two-instruction body plus one guarded call.
pub unsafe fn save_task_release(task: usize, free: i32) -> usize {
    if task != 0 && free > 0 {
        platform::call1(0x802B_9280, task as u32);
    }
    task
}

/// Clears the global save-task pointer and destroys the task.
///
/// # Original
/// - Ghidra: `FUN_800cedb0` at `0x800CEDB0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cedb0_FUN_800cedb0.c`
///
/// # Arguments
/// * `task` - task pointer; `0` is accepted.
/// * `free` - `>0` frees the block after teardown.
///
/// # Returns
/// `task`.
///
/// # Side effects
/// * writes `0` to `0x80429FB8` (`r13 - 0x56C8`);
/// * tears the task down through `0x802BA500(task, 0)`;
/// * optionally frees `task`.
///
/// # Safety
/// `task` must be live or `0`.
///
/// # Confidence
/// high - every store is visible.
pub unsafe fn save_task_destroy(task: usize, free: i32) -> usize {
    if task != 0 {
        platform::write_u32(G_SAVE_TASK, 0);
        platform::call2(0x802B_A500, task as u32, 0);
        if free > 0 {
            platform::call1(0x802B_9280, task as u32);
        }
    }
    task
}

/// Wakes the save worker when a request is queued.
///
/// In plain English: if the task's message queue is non-empty, reset the error
/// state, mark the operation "write" (mode 1), drain the queue and resume the
/// worker thread.  This is the hand-off point used by `dScBoot_c` at
/// `0x8015D330`.
///
/// # Original
/// - Ghidra: `FUN_800cee20` at `0x800CEE20` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cee20_FUN_800cee20.c`
/// - Caller: `0x8015D330` in the boot scene (`dScBoot_c`)
///
/// # Arguments
/// * `task` - live save task.
///
/// # Returns
/// `true` when a request was pending and the worker was resumed.
///
/// # Side effects
/// Writes `state = 0` (`+0x78`), `result flag = 0` (`+0x7C`), `mode = 1`
/// (`+0x74`); resets the queue at `+0x54`; resumes the thread at `+0x6C`.
///
/// # Safety
/// `task` must be the live save task.
///
/// # Confidence
/// high for the control flow; `medium` for the field names (inferred from the
/// state steps that read them).
pub unsafe fn save_task_wait_and_start(task: usize) -> bool {
    let queued = platform::call1(0x801B_3080, (task + 0x54) as u32) != 0;
    if queued {
        platform::write_u32(task + 0x78, 0);
        platform::write_u8(task + 0x7C, 0);
        platform::write_u32(task + 0x74, 1);
        platform::call1(0x801B_2F40, (task + 0x54) as u32);
        platform::call1(0x801B_3230, (task + 0x6C) as u32); // thunk_FUN_801b5d80
    }
    queued
}

/// Submits a completed save image to the worker.
///
/// In plain English: if a request is queued, copy the caller's 0x3FA0-byte save
/// image into the global buffer, reset the queue and wake the worker.
///
/// # Original
/// - Ghidra: `FUN_800cf080` at `0x800CF080` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf080_FUN_800cf080.c`
/// - Caller: `0x800E0540` (save-image builder) passes its stack image.
///
/// # Arguments
/// * `task` - live save task.
/// * `buffer` - source 0x3FA0-byte save image.
///
/// # Returns
/// `true` when a request was queued and the copy happened.
///
/// # Side effects
/// Writes `state = 0`, `mode = 4`; copies to `0x8035DCC0`; drains the queue;
/// resumes the thread.
///
/// # Safety
/// `buffer` must be readable for 0x3FA0 bytes.
///
/// # Confidence
/// high - the copy length and both sync calls are explicit.
pub unsafe fn save_task_submit(task: usize, buffer: usize) -> bool {
    let queued = platform::call1(0x801B_3080, (task + 0x54) as u32) != 0;
    if queued {
        platform::write_u32(task + 0x78, 0);
        platform::write_u32(task + 0x74, 4);
        platform::call3(
            0x8000_4364,
            G_SAVE_BUFFER as u32,
            buffer as u32,
            0x3FA0,
        );
        platform::call1(0x801B_2F40, (task + 0x54) as u32);
        platform::call1(0x801B_3230, (task + 0x6C) as u32); // thunk_FUN_801b5d80
    }
    queued
}

// ---------------------------------------------------------------------------
// 0x800CEF70 / 0x800CF520 / 0x800CF8F0 - save-operation selectors
//
// Three identical selectors driven by the worker-side tail stubs
// `0x8015D460`, `0x800E1680` and `0x800180E0`, which load the global save task
// (`0x80429FB8`) and branch here. Each resets the error word `+0x78`, sets the
// operation word `+0x74`, drains the request queue at `+0x54` and resumes the
// worker at `+0x6C`; the value is `1` only when a request was queued.
//
// The operation word is read by the worker dispatcher `0x800CF9D0`, whose arms
// are `1 -> 0x800CEE90` ([`save_check_files`]), `2 -> 0x800CEFE0`
// ([`save_check_nand_space`]), `3 -> 0x800CF960`, `4 -> 0x800CF110`
// ([`save_read`]), `5 -> 0x800CF590` ([`save_verify`]) and
// `6 -> 0x800CFA88` (finish).
// ---------------------------------------------------------------------------

/// Selects save operation `2` (the NAND-space check).
///
/// In plain English: when the task's queue at `+0x54` is non-empty, clear the
/// error word, mark operation `2`, drain the queue and resume the worker.
///
/// # Original (untabled)
/// - Ghidra: none - no functions.tsv row for 0x800CEF70.
/// - Extent: `0x800CEF70..0x800CEFDC` (108 bytes, 25 instructions), ending at
///   the `blr` at `0x800CEFD8`; one zero word follows at `0x800CEFDC` and the
///   next prologue is at `0x800CEFE0`.
/// - Reach: the four-byte main-image tail branch `b 0x800CEF70` at
///   `0x8015D464` (`lwz r3,-0x56C8(r13)` immediately before it); no data word
///   names it.
/// - Table: none.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `mr r31,r3`;
///   `addi r3,r3,0x54`; `bl 0x801B3080`; `neg r0,r3`; `or r0,r0,r3`;
///   `rlwinm. r0,r0,1,31,31`; `bc 12,2` (`bt cr0.eq`) to the `li r3,0` exit;
///   `li r3,0`; `li r0,2`; `stw r3,0x78(r31)`; `addi r3,r31,0x54`;
///   `stw r0,0x74(r31)`; `bl 0x801B2F40`; `addi r3,r31,0x6C`;
///   `bl 0x801B3230`; `li r3,1`; `b`; the other arm `li r3,0`; epilogue;
///   `4E800020 bclr 20,0` (blr).
/// - Polarity: `neg`/`or`/`rlwinm ...,1,31,31` is the unsigned non-zero idiom
///   `(x | -x) >> 31`, and `4182002C` is `bt cr0.eq` - so a **zero** queue
///   result takes the `return 0` arm and a non-zero queue takes the work arm.
///
/// # Arguments
/// - `task`: the global save task (`0x80429FB8`).
///
/// # Returns
/// `1` when a request was queued and operation `2` was armed, else `0`.
///
/// # Side effects
/// Writes `+0x74`/`+0x78`, drains `+0x54` and resumes the worker at `+0x6C`.
///
/// # Safety
/// `task` must be the live save task; `+0x54` and `+0x6C` must be live
/// synchronisation objects.
///
/// # Confidence
/// high - the queue test, both stores and the two sync calls are literal; the
/// operation number is the dispatcher's arm index.
pub unsafe fn fn_800cef70(task: usize) -> u32 {
    if platform::call1(0x801B_3080, (task + 0x54) as u32) == 0 {
        return 0;
    }
    platform::write_u32(task + 0x78, 0);
    platform::write_u32(task + 0x74, 2);
    platform::call1(0x801B_2F40, (task + 0x54) as u32);
    platform::call1(0x801B_3230, (task + 0x6C) as u32);
    1
}

/// Selects save operation `5` (the verification step).
///
/// The byte-for-byte twin of [`fn_800cef70`] with operation `5`, whose handler
/// is `0x800CF590` ([`save_verify`]).
///
/// # Original (untabled)
/// - Ghidra: none - no functions.tsv row for 0x800CF520.
/// - Extent: `0x800CF520..0x800CF58C` (108 bytes, 25 instructions), ending at
///   the `blr` at `0x800CF588`; one zero word follows at `0x800CF58C` and the
///   next prologue is at `0x800CF590`.
/// - Reach: the four-byte main-image tail branch `b 0x800CF520` at
///   `0x800E1684` (preceded by `lwz r3,-0x56C8(r13)`); no data word names it.
/// - Table: none.
/// - Observed: the body of [`fn_800cef70`] with `li r0,5` instead of `li r0,2`
///   (the compare at `0x800CF544`, the `bt cr0.eq` exit at `0x800CF548` and the
///   two sync calls are identical).
/// - Polarity: identical to [`fn_800cef70`] - a zero queue result returns `0`.
///
/// # Arguments
/// - `task`: the global save task (`0x80429FB8`).
///
/// # Returns
/// `1` when a request was queued and operation `5` was armed, else `0`.
///
/// # Side effects
/// Writes `+0x74`/`+0x78`, drains `+0x54` and resumes the worker at `+0x6C`.
///
/// # Safety
/// Same contract as [`fn_800cef70`].
///
/// # Confidence
/// high - identical structure to [`fn_800cef70`], operation number aside.
pub unsafe fn fn_800cf520(task: usize) -> u32 {
    if platform::call1(0x801B_3080, (task + 0x54) as u32) == 0 {
        return 0;
    }
    platform::write_u32(task + 0x78, 0);
    platform::write_u32(task + 0x74, 5);
    platform::call1(0x801B_2F40, (task + 0x54) as u32);
    platform::call1(0x801B_3230, (task + 0x6C) as u32);
    1
}

/// Selects save operation `3` (the `0x800CF960` reporting arm).
///
/// The third copy of [`fn_800cef70`], with operation `3`, whose handler is
/// `0x800CF960` (it reports through `0x800CFB30`).
///
/// # Original (untabled)
/// - Ghidra: none - no functions.tsv row for 0x800CF8F0.
/// - Extent: `0x800CF8F0..0x800CF95C` (108 bytes, 25 instructions), ending at
///   the `blr` at `0x800CF958`; one zero word follows at `0x800CF95C` and the
///   next prologue is at `0x800CF960`.
/// - Reach: the four-byte main-image tail branch `b 0x800CF8F0` at
///   `0x800180E4` (preceded by `lwz r3,-0x56C8(r13)`); no data word names it.
/// - Table: none.
/// - Observed: the body of [`fn_800cef70`] with `li r0,3` instead of `li r0,2`.
/// - Polarity: identical to [`fn_800cef70`] - a zero queue result returns `0`.
///
/// # Arguments
/// - `task`: the global save task (`0x80429FB8`).
///
/// # Returns
/// `1` when a request was queued and operation `3` was armed, else `0`.
///
/// # Side effects
/// Writes `+0x74`/`+0x78`, drains `+0x54` and resumes the worker at `+0x6C`.
///
/// # Safety
/// Same contract as [`fn_800cef70`].
///
/// # Confidence
/// high - identical structure to [`fn_800cef70`], operation number aside.
pub unsafe fn fn_800cf8f0(task: usize) -> u32 {
    if platform::call1(0x801B_3080, (task + 0x54) as u32) == 0 {
        return 0;
    }
    platform::write_u32(task + 0x78, 0);
    platform::write_u32(task + 0x74, 3);
    platform::call1(0x801B_2F40, (task + 0x54) as u32);
    platform::call1(0x801B_3230, (task + 0x6C) as u32);
    1
}

/// Creates the save worker thread and resumes it.
///
/// In plain English: take the task allocator lock, allocate 0x80 bytes, build
/// the task at the calling thread's base priority minus one, release the lock
/// and resume the worker stored at `task + 8`.
///
/// # Original
/// - Ghidra: `FUN_800cfab0` at `0x800CFAB0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cfab0_FUN_800cfab0.c`
/// - Caller: `0x800E48D0` (the `dSys` start-up, called from
///   [`crate::engine::startup::game_startup`]).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates and initialises the save task, publishes it through
/// [`save_task_construct`], and resumes the OS thread.
///
/// # Safety
/// Must run once during engine start-up on the game thread.
///
/// # Confidence
/// high for the call order; `medium` for "thread at `task + 8`" (inferred from
/// the task constructor and the `OSResumeThread`-shaped call).
pub unsafe fn save_task_create() {
    let lock = call0_ret(0x8016_E4F0); // thunk_FUN_802b9170 (allocator lock)
    // unknown: Ghidra prints no argument for the lock call; the thunk stores
    // its r3 into 0x80423890-related state and returns the previous value.
    let memory = platform::call1(0x802B_9210, 0x80);
    let mut task = 0usize;
    if memory != 0 {
        let current = platform::read_u32(0x8000_00E4);
        let priority = platform::call1(0x801B_5F70, current);
        task = save_task_construct(memory as usize, priority.wrapping_sub(1), 0);
    }
    platform::call1(0x8016_E4F0, lock); // unlock
    platform::call1(0x801B_5860, platform::read_u32(task + 8));
}

// ===========================================================================
// Boot/save state steps.  These all read/write the state object at +0x78.
// ===========================================================================

/// Maps an SDK/NAND result code to the boot scene's state number.
///
/// The state object stores the mapped value at `+0x78`; `0` means "success,
/// keep going" and non-zero is an error state that the boot scene handles.
///
/// # Original
/// - Ghidra: `FUN_800cfb30` at `0x800CFB30` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cfb30_FUN_800cfb30.c`
///
/// # Arguments
/// * `task` - state object.
/// * `result` - raw SDK result (`0`, `-2..-15`).
///
/// # Returns
/// Nothing; the mapped state is written to `task + 0x78`.
///
/// # Side effects
/// One `u32` store at `task + 0x78`.
///
/// # Safety
/// `task` must be writable.
///
/// # Confidence
/// high - the switch is fully visible.  The *names* of the error states are
/// `// unknown:`; only the numeric mapping is observed.
pub unsafe fn save_result_to_state(task: usize, result: u32) {
    let state: u32 = match result as i32 {
        -15 | -5 => 6, // 0xfffffff1 / 0xfffffffb
        -12 | -6 | 0 => 0, // 0xfffffff4 / 0xfffffffa / success
        -11 => 3,      // 0xfffffff5
        -9 => 2,       // 0xfffffff7
        -4 => 1,       // 0xfffffffc
        -3 | -2 => 4,  // 0xfffffffd / 0xfffffffe
        _ => 5,
    };
    platform::write_u32(task + 0x78, state);
}

/// Checks that both save files exist.
///
/// In plain English: ask the NAND API whether `wiimj2d.sav` exists, then whether
/// `banner.bin` exists; when both do, set the "ready" byte at `+0x7C`.
///
/// # Original
/// - Ghidra: `FUN_800cee90` at `0x800CEE90` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cee90_FUN_800cee90.c`
///
/// # Arguments
/// * `task` - state object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801DAE70` twice; updates `+0x78` through
/// [`save_result_to_state`]; may write `+0x7C = 1`.
///
/// # Safety
/// `task` must be writable.
///
/// # Confidence
/// high - both path strings and the two-result count are explicit.
pub unsafe fn save_check_files(task: usize) {
    let mut status = crate::platform::GuestLocal::new( [0u8; 12]);
    let mut found = 0u32;

    let mut result = platform::call2(
        0x801D_AE70,
        STR_SAVE_FILE,
        status.as_mut_ptr() as u32,
    );
    save_result_to_state(task, result);

    if platform::read_u32(task + 0x78) == 0 {
        if result == 0 && status[0] == 1 {
            found = 1;
        }

        result = platform::call2(
            0x801D_AE70,
            STR_BANNER_FILE,
            status.as_mut_ptr() as u32,
        );
        save_result_to_state(task, result);

        if platform::read_u32(task + 0x78) == 0 {
            if result == 0 && status[0] == 1 {
                found += 1;
            }
            if found == 2 {
                platform::write_u8(task + 0x7C, 1);
            }
        }
    }
}

/// Checks NAND free space before creating the save file.
///
/// # Original
/// - Ghidra: `FUN_800cefe0` at `0x800CEFE0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cefe0_FUN_800cefe0.c`
///
/// # Arguments
/// * `task` - state object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801DB140(3, 2, &out)`; maps the result; sets state `7` when the
/// low-space bit is set (`out & 5`) and state `8` when `out & 10` is set.
///
/// # Safety
/// `task` must be writable.
///
/// # Confidence
/// medium - the call and masks are observed; the meaning of `3`/`2` and of the
/// bit pairs is `// unknown:`.
pub unsafe fn save_check_nand_space(task: usize) {
    // Original local is `uint local_18[4]`, first word preset to -1.
    let mut info = crate::platform::GuestLocal::new( [0xFFFF_FFFFu32; 4]);
    let result = platform::call3(
        0x801D_B140,
        3,
        2,
        info.as_mut_ptr() as u32,
    );
    save_result_to_state(task, result);

    if platform::read_u32(task + 0x78) == 0 && result == 0 {
        if (info[0] & 5) == 0 {
            if (info[0] & 10) != 0 {
                platform::write_u32(task + 0x78, 8);
            }
        } else {
            platform::write_u32(task + 0x78, 7);
        }
    }
}

/// Reads `wiimj2d.sav` into the global save image.
///
/// In plain English: open the save, read it, decode it into `0x8035DCC0`, close
/// it, and on a size/format rejection fall through to the banner step.
///
/// # Original
/// - Ghidra: `FUN_800cf110` at `0x800CF110` (380 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf110_FUN_800cf110.c`
///
/// # Arguments
/// * `task` - state object.
///
/// # Returns
/// `2` when the close/finish call itself failed, otherwise `1` or the result of
/// [`save_read_banner`].
///
/// # Side effects
/// Opens, reads and closes `wiimj2d.sav`; decodes 0x3FA0 bytes into
/// `0x8035DCC0`; updates `+0x78`.
///
/// # Safety
/// The global save buffer must be writable; this must be the only save
/// operation in flight.
///
/// # Confidence
/// high for the control flow; the `// unknown:` `0x801D9950` mode `2` is the
/// SDK's read mode.
pub unsafe fn save_read(task: usize) -> u32 {
    let mut buffer = crate::platform::GuestLocal::new( [0u8; 0x90]); // original local auStack_98[144]

    // unknown: 0x3C is the SDK open-mode/argument passed to 0x801D84E0.
    let mut result = platform::call3(0x801D_84E0, STR_SAVE_FILE, 0x3C, 0);
    save_result_to_state(task, result);
    if platform::read_u32(task + 0x78) != 0 {
        return 1;
    }

    result = platform::call5(
        0x801D_9950,
        STR_SAVE_FILE,
        buffer.as_mut_ptr() as u32,
        2,
        0x8035_9CC0, // metadata block (DAT_80359cc0)
        0x4000,      // 16 KiB read buffer
    );
    save_result_to_state(task, result);

    if platform::read_u32(task + 0x78) != 0 {
        if platform::read_u32(task + 0x78) == 6 {
            let close = platform::call1(0x801D_9F60, buffer.as_mut_ptr() as u32);
            save_result_to_state(task, close);
        }
        return 1;
    }

    // Decode the file image into the global buffer.
    let decoded = platform::call3(
        0x801D_8AE0,
        buffer.as_mut_ptr() as u32,
        G_SAVE_BUFFER as u32,
        0x3FA0, // size of the save image
    ) as i32;

    if decoded < 0 {
        save_result_to_state(task, decoded as u32);
        if platform::read_u32(task + 0x78) != 0 {
            if platform::read_u32(task + 0x78) == 6 {
                let close = platform::call1(0x801D_9F60, buffer.as_mut_ptr() as u32);
                save_result_to_state(task, close);
                if platform::read_u32(task + 0x78) != 0 {
                    return 2;
                }
            }
            return 1;
        }
    }

    // Finish the read.
    let finished = platform::call1(0x801D_9D10, buffer.as_mut_ptr() as u32);
    save_result_to_state(task, finished);
    if platform::read_u32(task + 0x78) != 0 {
        return 1;
    }

    // Fall through to the banner read.
    save_read_banner(task)
}

/// Reads and validates the temporary banner file.
///
/// In plain English: open `/tmp/banner.bin`, read it, build the banner image,
/// then write it back in the format the save file expects.
///
/// # Original
/// - Ghidra: `FUN_800cf290` at `0x800CF290` (376 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf290_FUN_800cf290.c`
/// - Called by [`save_read`] when the save read succeeds.
///
/// # Arguments
/// * `task` - state object.
///
/// # Returns
/// `1` on success, `0` when the final banner write left an error state.
///
/// # Side effects
/// Opens/reads/writes `/tmp/banner.bin`; builds banner data via
/// [`save_build_banner_buffer`]; updates `+0x78`.
///
/// # Safety
/// `task` must be writable; only one save operation may run.
///
/// # Confidence
/// medium - one error-path call lost its second argument in the decompile
/// (`FUN_800cfb30(param_1)`), so the error code is inferred as
/// the just-computed `0x72A0` mismatch value.
pub unsafe fn save_read_banner(task: usize) -> u32 {
    let mut buffer = crate::platform::GuestLocal::new( [0u8; 0x90]);

    // unknown: same SDK open mode 0x3C as save_read.
    let mut result = platform::call3(0x801D_84E0, STR_BANNER_TMP, 0x3C, 0);
    save_result_to_state(task, result);
    if platform::read_u32(task + 0x78) != 0 {
        return 1;
    }

    result = platform::call3(
        0x801D_95B0,
        STR_BANNER_TMP,
        buffer.as_mut_ptr() as u32,
        2,
    );
    save_result_to_state(task, result);
    if platform::read_u32(task + 0x78) != 0 {
        return 1;
    }

    let built = save_build_banner_buffer(task, buffer.as_mut_ptr() as usize);
    if built != 0x72A0 {
        // Error path: Ghidra dropped r4 here; the
        // value still in r4 is the just-compared size.  `// partial:`
        save_result_to_state(task, built);
        return 1;
    }

    result = platform::call2(0x801D_9850, buffer.as_mut_ptr() as u32, 0x72A0); // banner block size
    save_result_to_state(task, result);
    if platform::read_u32(task + 0x78) != 0 {
        return 1;
    }

    // 16-word banner header, all zero.
    let mut header = crate::platform::GuestLocal::new( [0u32; 16]);
    result = platform::call1(0x801D_AAF0, header.as_mut_ptr() as u32);
    save_result_to_state(task, result);
    if platform::read_u32(task + 0x78) != 0 {
        return 1;
    }

    result = platform::call2(
        0x801D_8FD0,
        STR_BANNER_TMP,
        header.as_mut_ptr() as u32,
    );
    save_result_to_state(task, result);

    // `(-state | state) >> 31`: 1 when the final state is non-zero.
    (platform::read_u32(task + 0x78) != 0) as u32
}

/// Builds the save banner image in place.
///
/// In plain English: initialise the banner work area, load two model resources
/// (`save_banner` / `save_banner_US.bti`), copy the 0x6000-byte and 0x1200-byte
/// payloads, set header flags, and encode the 0x72A0-byte banner block into the
/// caller's scratch buffer.
///
/// # Original
/// - Ghidra: `FUN_800cf410` at `0x800CF410` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf410_FUN_800cf410.c`
/// - Removed: the modelled `0x802DCD74`/`0x802DCDC0` calls. They are the
///   `_savegpr_27`/`_restgpr_27` compiler helpers (register spill/restore
///   only) at `0x800CF420`/`0x800CF504`.
///
/// # Arguments
/// * `_task` - state object; **not read** by the original body.
/// * `scratch` - the caller's 0x72A0-byte banner buffer (`r4` at entry; Ghidra
///   prints it as `extraout_r4` in the decode call).
///
/// # Returns
/// The value produced by `0x801D8AE0` (the 0x72A0 size on success); this is the
/// value [`save_read_banner`] compares against `0x72A0`.
///
/// # Side effects
/// * writes the banner header at `0x80361C60`;
/// * copies models into `0x80361D00` (0x6000) and `0x80367D00` (0x1200);
/// * ORs `2` into the low nibble of `0x80361C68`;
/// * encodes 0x72A0 bytes into `scratch`.
///
/// # Safety
/// `scratch` must be writable for 0x72A0 bytes.
///
/// # Confidence
/// medium - every store is visible, but Ghidra lost the argument list, so the
/// `scratch`/`r4` binding is `// inferred:` from the caller `0x800CF290`.
pub unsafe fn save_build_banner_buffer(_task: usize, scratch: usize) -> u32 {
    let a = platform::call2(0x800C_DC40, 0x15F91, 1); // unknown: image format id
    let b = platform::call2(0x800C_DC40, 0x15F91, 0);
    platform::call4(0x801D_AFA0, G_BANNER_WORK as u32, 0, a, b);

    let manager = platform::read_u32(G_RESOURCE_MANAGER);
    let model = platform::call3(
        0x800D_F180,
        manager + 4,
        STR_SAVE_BANNER,
        STR_SAVE_BANNER_BTI,
    );
    let data = model + platform::read_u32(model as usize + 0x1C);
    platform::call3(
        0x8000_4364,
        G_BANNER_IMAGE as u32,
        data,
        0x6000, // banner image payload size
    );

    let alt_name = platform::read_u32(G_BANNER_NAME_ALT);
    let icon = platform::call3(0x800D_F180, manager + 4, STR_SAVE_BANNER, alt_name);
    let icon_data = icon + platform::read_u32(icon as usize + 0x1C);
    platform::call3(
        0x8000_4364,
        G_BANNER_ICON as u32,
        icon_data,
        0x1200, // icon payload size
    );

    let flags = platform::read_u32(G_BANNER_HEADER_FLAGS);
    platform::write_u32(G_BANNER_HEADER_FLAGS, (flags & 0xFFF0) | 2); // format field 2

    let encoded = platform::call3(
        0x801D_8AE0,
        scratch as u32,
        G_BANNER_WORK as u32,
        0x72A0,
    );
    encoded
}

/// Verifies the save file against the global image.
///
/// In plain English: open the save, check its size is 0x3FA0, decode it, compare
/// the decode result with the global image, then compare the four-byte format
/// stamp and the per-slot checksums.
///
/// # Original
/// - Ghidra: `FUN_800cf590` at `0x800CF590` (644 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf590_FUN_800cf590.c`
///
/// # Arguments
/// * `task` - state object.
///
/// # Returns
/// `2` if the final close failed, `1` for most outcomes, `0` when the slot
/// comparison found a mismatch.
///
/// # Side effects
/// Opens/reads/closes `wiimj2d.sav`; decodes into `0x8035DCC0`; may set state
/// `6`; calls [`save_compare_slots`].
///
/// # Safety
/// `task` must be writable; only one save operation may run.
///
/// # Confidence
/// high for the control flow; `medium` for the stamp/checksum meaning
/// (`// inferred:` format stamp).
pub unsafe fn save_verify(task: usize) -> u32 {
    let mut buffer = crate::platform::GuestLocal::new( [0u8; 0x90]);
    let mut size = crate::platform::GuestLocal::<i32>::new( 0);

    let mut result = platform::call5(
        0x801D_9950,
        STR_SAVE_FILE,
        buffer.as_mut_ptr() as u32,
        1,
        0x8035_9CC0,
        0x4000,
    );
    save_result_to_state(task, result);

    if platform::read_u32(task + 0x78) != 0 {
        if platform::read_u32(task + 0x78) == 6 {
            let close = platform::call1(0x801D_9F60, buffer.as_mut_ptr() as u32);
            save_result_to_state(task, close);
        }
        return 1;
    }

    result = platform::call2(
        0x801D_9040,
        buffer.as_mut_ptr() as u32,
        (size.as_mut_ptr() as *mut i32) as u32,
    );
    save_result_to_state(task, result);

    if platform::read_u32(task + 0x78) != 0 {
        if platform::read_u32(task + 0x78) == 6 {
            let close = platform::call1(0x801D_9F60, buffer.as_mut_ptr() as u32);
            save_result_to_state(task, close);
        }
        return 1;
    }

    if *size == 0x3FA0 {
        let compared = platform::call3(
            0x801D_89F0,
            buffer.as_mut_ptr() as u32,
            G_SAVE_BUFFER as u32,
            0x3FA0,
        ) as i32;

        if compared < 0 {
            save_result_to_state(task, compared as u32);
            if platform::read_u32(task + 0x78) != 0 {
                if platform::read_u32(task + 0x78) == 6 {
                    let close = platform::call1(0x801D_9F60, buffer.as_mut_ptr() as u32);
                    save_result_to_state(task, close);
                    if platform::read_u32(task + 0x78) != 0 {
                        return 2;
                    }
                }
                return 1;
            }
        }

        let finished = platform::call1(0x801D_9D10, buffer.as_mut_ptr() as u32);
        save_result_to_state(task, finished);
        if platform::read_u32(task + 0x78) != 0 {
            return 1;
        }

        // Four-byte format stamp.
        let stamp_a = platform::read_u8(G_SAVE_STAMP);
        let stamp_b = platform::read_u8(G_SAVE_STAMP + 1);
        let stamp_c = platform::read_u8(G_SAVE_STAMP + 2);
        let stamp_d = platform::read_u8(G_SAVE_STAMP + 3);
        let save_a = platform::read_u8(G_SAVE_BUFFER);
        let save_b = platform::read_u8(G_SAVE_BUFFER + 1);
        let save_c = platform::read_u8(G_SAVE_BUFFER + 2);
        let save_d = platform::read_u8(G_SAVE_BUFFER + 3);

        if save_a == stamp_a && save_b == stamp_b && save_c == stamp_c {
            if save_d == stamp_d {
                let same = save_compare_slots();
                if same == 0 {
                    platform::write_u32(task + 0x78, 6);
                    return 1;
                }
                return 0;
            }
            platform::write_u32(task + 0x78, 6);
            return 1;
        }
        platform::write_u32(task + 0x78, 6);
        return 1;
    }

    // Wrong size.
    let finished = platform::call1(0x801D_9D10, buffer.as_mut_ptr() as u32);
    save_result_to_state(task, finished);
    if platform::read_u32(task + 0x78) != 0 {
        platform::write_u32(task + 0x78, 6);
    }
    1
}

/// Compares the three save slots against the checksums embedded in the image.
///
/// # Original
/// - Ghidra: `FUN_800cf820` at `0x800CF820` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf820_FUN_800cf820.c`
/// - Called by [`save_verify`] when the size is `0x3FA0`.
///
/// # Arguments
/// None; it reads the global `0x8035DCC0` image.
///
/// # Returns
/// `1` when the header checksum and all six slot checksums match, `0` otherwise.
///
/// # Side effects
/// Only reads and calls the checksum function `0x8015F130` (the
/// `thunk_FUN_801b8620` wrapper around `0x801B8620`).
///
/// # Safety
/// The global save image must be initialised.
///
/// # Confidence
/// high - the layout is proven by [`save_task_construct`]:
/// `+0x000` header (0x6A0), `+0x6A0` and `+0x2320` two arrays of three
/// 0x980-byte records, each with a 0x97C-byte payload and a `u32` checksum.
pub unsafe fn save_compare_slots() -> u32 {
    let base = G_SAVE_BUFFER;

    // Header checksum: +4 for 0x698 bytes, stored at +0x69C.
    let header_checksum = platform::call2(0x8015_F130, (base + 4) as u32, 0x698); // thunk_FUN_801b8620
    if header_checksum != platform::read_u32(base + 0x69C) {
        return 0;
    }

    let mut index = 0usize;
    while index < 3 {
        let offset = index * 0x980;
        let a = platform::call2(
            0x8015_F130,
            (base + 0x6A0 + offset) as u32,
            0x97C,
        );
        if a != platform::read_u32(base + 0x101C + offset) {
            return 0;
        }

        let b = platform::call2(
            0x8015_F130,
            (base + 0x2320 + offset) as u32,
            0x97C,
        );
        if b != platform::read_u32(base + 0x2C9C + offset) {
            return 0;
        }
        index += 1;
    }
    1
}

/// Deletes the save and banner files.
///
/// # Original
/// - Ghidra: `FUN_800cf960` at `0x800CF960` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800cf960_FUN_800cf960.c`
///
/// # Arguments
/// * `task` - state object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801D87E0` for `banner.bin` and, if the first delete did not set an
/// error state, for `wiimj2d.sav`; updates `+0x78`.
///
/// # Safety
/// `task` must be writable.
///
/// # Confidence
/// high - both path strings are explicit.
pub unsafe fn save_delete_files(task: usize) {
    let mut result = platform::call1(0x801D_87E0, STR_BANNER_FILE);
    save_result_to_state(task, result);
    if platform::read_u32(task + 0x78) == 0 {
        result = platform::call1(0x801D_87E0, STR_SAVE_FILE);
        save_result_to_state(task, result);
    }
}

/// Loads the save manager's four per-record fields into the global tables.
///
/// In plain English: reset the save header at `manager + 0x20`, take the record
/// array, ask `0x800CE110` for one shared value, then for each of the four
/// records store its slot order and its three `u8` fields into the four global
/// tables.  Finally run the two scene-side hooks.
///
/// # Original
/// - Ghidra: `FUN_800e1840` at `0x800E1840` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1840_FUN_800e1840.c`
/// - Sibling: [`save_record_copy`] (`0x800E1940`) operates on the same object.
///
/// # Arguments
/// * `manager` - save-manager object; the header starts at `+0x20`.
/// * `arg` - second argument, forwarded verbatim to `0x800CE5F0`
///   (`// inferred:` from `extraout_r4`; Ghidra typed the function `void`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * calls `0x800CE5F0`, `0x800E0380`, `0x800CE110`, `0x800CE070`,
///   `0x800CE090`, `0x800CE0B0`, `0x800CE0D0`, `0x800CE0F0`;
/// * writes `0x80429CC0` and the five tables at `0x80354E60..0x80354EB0`;
/// * calls `0x80060EF0` and `0x80102A80`.
///
/// # Safety
/// `manager` must be a live save manager; the global tables are BSS.
///
/// # Confidence
/// medium - every store and call is visible; the `arg` binding is recovered
/// from `extraout_r4` and the table meanings are `// unknown:`.
pub unsafe fn save_manager_init(manager: usize, arg: i32) {
    // 0x800E1850: header reset on the sub-object at +0x20.
    platform::call2(0x800C_E5F0, (manager + 0x20) as u32, arg as u32);

    // 0x800E1858: the record array (`0xFFFFFFFF` = "all records").
    let record = platform::call2(0x800E_0380, manager as u32, 0xFFFF_FFFF);

    // 0x800E1860: shared value published in BSS.
    let shared = call0_ret(0x800C_E110);
    platform::write_u32(G_SAVE_MANAGER_STATE, shared);

    let mut i: u32 = 0;
    while i < 4 {
        // Slot order: the first table is a byte array indexed by the value the
        // record itself returns.
        let slot = platform::call2(0x800C_E070, record, i);
        platform::write_u8(SAVE_TABLE_SLOT_ORDER + slot as usize, i as u8);

        let a = platform::call2(0x800C_E090, record, i);
        platform::write_u32(SAVE_TABLE_A + i as usize * 4, a & 0xFF);

        let c = platform::call2(0x800C_E0B0, record, i);
        platform::write_u32(SAVE_TABLE_C + i as usize * 4, c & 0xFF);

        let b = platform::call2(0x800C_E0D0, record, i);
        platform::write_u32(SAVE_TABLE_B + i as usize * 4, b & 0xFF);

        // Observed: `char` result sign-extended into the word table.
        let d = platform::call2(0x800C_E0F0, record, i) as i8;
        platform::write_i32(SAVE_TABLE_D + i as usize * 4, d as i32);

        i += 1;
    }

    platform::call0(0x8006_0EF0);
    platform::call0(0x8010_2A80);
}

/// Copies one 0x980-byte save record over another.
///
/// In plain English: reset the header at `manager + 0x20`, fetch record `-1`
/// from both accessors (`0x800E03B0` source, `0x800E0380` destination) and copy
/// 0x980 bytes.
///
/// # Original
/// - Ghidra: `FUN_800e1940` at `0x800E1940` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1940_FUN_800e1940.c`
///
/// # Arguments
/// * `manager` - save-manager object.
/// * `arg` - forwarded to `0x800CE5F0` (the function's second argument).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies 0x980 bytes into the record returned by `0x800E0380`.
///
/// # Safety
/// Both records must be live and 0x980 bytes long.
///
/// # Confidence
/// high - the copy direction and length are explicit; the meaning of `arg` is
/// `// unknown:` but its position is observed.
pub unsafe fn save_record_copy(manager: usize, arg: i32) {
    platform::call2(0x800C_E5F0, (manager + 0x20) as u32, arg as u32);
    let src = platform::call2(0x800E_03B0, manager as u32, 0xFFFF_FFFF);
    let dst = platform::call2(0x800E_0380, manager as u32, 0xFFFF_FFFF);
    platform::call3(0x8000_4364, dst, src, 0x980);
}

// ===========================================================================
// Base scene object.
// ===========================================================================

/// Constructs the base scene object.
///
/// In plain English: run the base-object constructor, install the base vtable
/// at `+0x60`, set the "active/visible" bits in `+0x0F`, clear the state pointer
/// at `+0x70`, and clear the four global scene flags.
///
/// # Original
/// - Ghidra: `FUN_800e19b0` at `0x800E19B0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e19b0_FUN_800e19b0.c`
/// - Caller: `0x8015C130` (the `dScBoot_c` constructor) calls it before
///   overwriting the vtable with the derived one at `+0x60`.
///
/// # Arguments
/// * `this` - scene object; returned unchanged.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes the vtable at `+0x60`, ORs `5` into `+0x0F`, clears `+0x70`, clears
/// `0x80429EF9` and `0x80429CF8..0x80429CFB`, and calls the base constructor
/// helper `0x8006C420` plus `0x800B3A20`.
///
/// # Safety
/// `this` must be a writable scene object of at least 0x71 bytes.
///
/// # Confidence
/// high - every store is explicit and the two helper receivers are visible in
/// the instruction stream: `0x8006C420` gets `r3 = this` (saved in `r30` at
/// `0x800E19C4`), and `0x800B3A20` gets `r3 = 0x8031AFC0` (built at
/// `0x800E19D0..D4`, the same word stored at `+0x60`).
pub unsafe fn scene_base_construct(this: usize) -> usize {
    platform::call1(0x8006_C420, this as u32);
    platform::write_u32(this + 0x60, VTABLE_BASE_SCENE);
    let flags = platform::read_u8(this + 0x0F) | 5;
    platform::write_u8(this + 0x0F, flags);
    platform::write_u32(this + 0x70, 0);
    platform::write_u8(G_SCENE_FLAG_5787, 0);
    platform::call1(0x800B_3A20, VTABLE_BASE_SCENE);
    platform::write_u8(G_SCENE_FLAG_5988, 0);
    platform::write_u8(G_SCENE_FLAG_5988 + 1, 0);
    platform::write_u8(G_SCENE_FLAG_5988 + 2, 0);
    platform::write_u8(G_SCENE_FLAG_5988 + 3, 0);
    this
}

/// Destroys the base scene object.
///
/// # Original
/// - Ghidra: `FUN_800e1a20` at `0x800E1A20` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1a20_FUN_800e1a20.c`
///
/// # Arguments
/// * `this` - scene object or `0`.
/// * `free` - `>0` asks `0x80162920` to free the object.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// * restores the base vtable at `+0x60`;
/// * decrements the global scene reference count (`0x8042A2E0`) when positive;
/// * calls `0x80069C50`, `0x8006C490(this, 0)` and optionally `0x80162920`.
///
/// # Safety
/// `this` must be live or `0`.
///
/// # Confidence
/// high for the stores and the guarded free.
pub unsafe fn scene_base_destroy(this: usize, free: i32) -> usize {
    if this != 0 {
        platform::write_u32(this + 0x60, VTABLE_BASE_SCENE);
        let count = platform::read_i32(G_SCENE_REFCOUNT);
        if count > 0 {
            platform::write_u32(G_SCENE_REFCOUNT, count.wrapping_sub(1) as u32);
        }
        platform::call0(0x8006_9C50);
        platform::call2(0x8006_C490, this as u32, 0);
        if free > 0 {
            platform::call1(0x8016_2920, this as u32);
        }
    }
    this
}

/// Reports whether the base scene object has finished.
///
/// # Original
/// - Ghidra: `FUN_800e1aa0` at `0x800E1AA0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1aa0_FUN_800e1aa0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `true` when `0x8006C540` returns non-zero; also sets the global finished
/// flag at `0x8042873C`.
///
/// # Side effects
/// May write `1` to `0x8042873C`.
///
/// # Safety
/// None beyond the target ABI.
///
/// # Confidence
/// high - two stores/calls, fully visible.
pub unsafe fn scene_base_is_finished() -> bool {
    let finished = call0_ret(0x8006_C540) != 0;
    if finished {
        platform::write_u8(G_SCENE_FINISHED_FLAG, 1);
    }
    finished
}

/// Calls the scene manager's no-argument query at `vtable + 0x10`.
///
/// # Original
/// Not present in the binary: local model of the indirect call repeated at
/// `0x800E1BE0` (`(**(code **)(**(int **)(r13-0x5240) + 0x10))()`).
///
/// # Returns
/// The callee's `r3`.
///
/// # Safety
/// `0x8042A440` must hold a live scene-manager pointer.
///
/// # Confidence
/// medium - the double dereference is observed; whether the callee takes the
/// manager as `r3` is not visible (`// unknown:`).
#[inline]
unsafe fn scene_manager_query() -> u32 {
    let manager = platform::read_u32(G_SCENE_MANAGER_PTR) as usize;
    let vtable = platform::read_u32(manager) as usize;
    call0_ret(platform::read_u32(vtable + 0x10) as usize)
}

/// Per-frame update of the base scene object.
///
/// In plain English: if the scene has a sub-state table, run it; when it is
/// finished (runner returned `2`) or there is no sub-state, drive the global
/// transition state.  If no transition is pending (`current == 0x2EE`), either
/// let the manager finish the fade and then start the next scene, or clear the
/// "busy" bits when the scene itself is done.  While a transition *is* pending,
/// this is the code that asks the manager for the next step.
///
/// # Original
/// - Ghidra: `FUN_800e1be0` at `0x800E1BE0` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1be0_FUN_800e1be0.c`
/// - Called from the base scene vtable; the frame loop reaches it through
///   `0x800E4E40` (`FUN_8015F620`).
///
/// # Arguments
/// * `this` - live scene object.
///
/// # Returns
/// `1` when the transition was started/advanced, `0` otherwise.
///
/// # Side effects
/// Reads/writes the transition globals, may call `0x800B0A20`, `0x8006A2B0`,
/// `0x800B0A90`, `0x80162510`, and clears bits `0x05` of `this + 0x0F`.
///
/// # Safety
/// `this` must be live; the scene-manager singleton must be initialised.
///
/// # Confidence
/// medium - every branch is visible, but two vtable helpers are shown by
/// Ghidra as no-argument calls; their receiver is `// unknown:`.
pub unsafe fn scene_base_update(this: usize) -> u32 {
    if call0_ret(0x8006_C5C0) == 0 {
        return 0;
    }

    let substate = platform::read_u32(this + 0x70) as usize;
    if substate != 0 {
        let state = platform::call2(0x8015_F620, substate as u32, this as u32);
        if state != 2 {
            return 0;
        }
    }

    if platform::read_u16(G_SCENE_CURRENT) == 0x2EE {
        if (platform::read_u8(this + 0x0F) & 1) == 0 {
            let allowed = scene_manager_query();
            // The original tests countLeadingZeros(query) >> 5 != 0, i.e. query == 0.
            if allowed == 0 && platform::read_u8(G_SCENE_FINISHED_FLAG) != 0 {
                platform::call1(
                    0x800B_0A20,
                    platform::read_u16(G_SCENE_AUX_A) as u32,
                );
                platform::call1(
                    0x8006_A2B0,
                    platform::read_u16(G_SCENE_PREVIOUS) as u32,
                );
            }
            1
        } else {
            let done = platform::call1(0x8016_2A20, this as u32);
            if done == 0 {
                let flags = platform::read_u8(this + 0x0F) & 0xFA;
                platform::write_u8(this + 0x0F, flags);
            }
            0
        }
    } else {
        let step = scene_manager_query();
        if step != 1 {
            let state = scene_manager_query();
            // countLeadingZeros(state) >> 5 != 0 means state == 0.
            if state == 0 {
                platform::call1(0x8016_2510, this as u32);
            }
        } else {
            platform::call1(
                0x800B_0A90,
                platform::read_u16(G_SCENE_AUX_B) as u32,
            );
        }
        0
    }
}

// ===========================================================================
// Scene transition globals.
// ===========================================================================

/// Resets the transition ids and the global scene id.
///
/// # Original
/// - Ghidra: `FUN_800e1db0` at `0x800E1DB0` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1db0_FUN_800e1db0.c`
/// - Caller: `0x800E48D0` (`dSys` start-up).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears current/previous/older ids and the transition flag, then calls
/// `0x8006A620(0, 0)` to reset the global scene id.
///
/// # Safety
/// Must run during start-up before any scene request.
///
/// # Confidence
/// high - five stores and one call.
pub unsafe fn scene_transition_reset() {
    platform::write_u16(G_SCENE_CURRENT, 0);
    platform::write_u16(G_SCENE_PREVIOUS, 0);
    platform::write_u16(G_SCENE_OLDER, 0);
    platform::write_u8(G_SCENE_TRANSITION_FLAG, 0);
    platform::call2(0x8006_A620, 0, 0);
}

/// Commits a requested scene id and frees the transition slot.
///
/// In plain English: if the transition flag is clear and the current slot holds
/// a requested id, validate it through `0x80162B20` (called at the
/// `0x8006C6C0` thunk); on success, rotate
/// older/previous/current and park the current slot at `0x2EE`.
///
/// # Original
/// - Ghidra: `FUN_800e1de0` at `0x800E1DE0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1de0_FUN_800e1de0.c`
/// - Caller: `0x800E4E40` (one game frame), only when the scene state chain
///   returns `2`.
///
/// # Arguments
/// None.
///
/// # Returns
/// The non-zero value from `0x80162B20` on success, `0` when there was nothing
/// to commit or validation failed.
///
/// # Side effects
/// Rotates the three scene-id globals, sets the flag, updates the current slot.
///
/// # Safety
/// Must run once per frame on the game thread.
///
/// # Confidence
/// high - the rotation and sentinel are explicit.
pub unsafe fn scene_transition_step() -> i32 {
    if platform::read_u8(G_SCENE_TRANSITION_FLAG) != 0
        || platform::read_u16(G_SCENE_CURRENT) == 0x2EE
    {
        return 0;
    }

    let result = platform::call3(
        0x8006_C6C0, // thunk_FUN_80162b20
        platform::read_u16(G_SCENE_CURRENT) as u32,
        platform::read_u32(G_SCENE_ARGUMENT),
        1,
    ) as i32;

    if result == 0 {
        return 0;
    }

    platform::write_u16(G_SCENE_OLDER, platform::read_u16(G_SCENE_PREVIOUS));
    platform::write_u16(G_SCENE_PREVIOUS, platform::read_u16(G_SCENE_CURRENT));
    platform::write_u16(G_SCENE_CURRENT, 0x2EE);
    platform::write_u8(G_SCENE_TRANSITION_FLAG, 1);
    result
}

/// Requests a scene change.
///
/// In plain English: only while the transition slot is free (`current ==
/// 0x2EE`), check whether the fade manager is busy; unless forced, refuse the
/// request while a fade is in progress.  On acceptance, remember the argument,
/// set the global scene id and park the requested id in the current slot so
/// [`scene_transition_step`] can commit it on the next frame.
///
/// # Original
/// - Ghidra: `FUN_800e1e60` at `0x800E1E60` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1e60_FUN_800e1e60.c`
/// - Callers: `0x800BB780`, `0x801017D0`, `0x801018C0`, `0x80102260`,
///   `0x80102690` (gameplay / world-map code).
///
/// # Arguments
/// * `scene_id` - requested scene id (`3`, `6`, `7`, `8`, `9`, `0x2C7`, ... at
///   the observed call sites).
/// * `argument` - value stored at `0x8042A048` for the new scene.
/// * `force` - when non-zero, bypass the fade-busy test.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * may set `0x8042A21D` when leaving scene `5`;
/// * calls `0x8006A620(scene_id, argument)`;
/// * writes current id and argument globals.
///
/// # Safety
/// Must run on the game thread; the fade manager (`0x80107F10`) must be live.
///
/// # Confidence
/// high - every branch and store is visible; the `1`/`5` fade-state values are
/// `// inferred:` from the manager object.
pub unsafe fn scene_request(scene_id: i32, argument: u32, force: i32) {
    if platform::read_u16(G_SCENE_CURRENT) != 0x2EE {
        return;
    }

    let fade = call0_ret(0x8010_7F10) as usize;
    let fade_b = platform::read_i32(fade + 0x08);
    let fade_c = platform::read_i32(fade + 0x0C);

    let in_fade = if fade_c == 1 || fade_b == 1 {
        true
    } else {
        fade_c == 5 || fade_b == 5
    };

    if !in_fade || force != 0 {
        if platform::read_u16(G_SCENE_PREVIOUS) == 5 && scene_id != 5 {
            platform::write_u8(G_SCENE_LEAVE_FIVE_FLAG, 1);
        }
        platform::call2(0x8006_A620, scene_id as u32, argument);
        platform::write_u16(G_SCENE_CURRENT, scene_id as u16);
        platform::write_u32(G_SCENE_ARGUMENT, argument);
    }
}

/// Writes auxiliary scene id A.
///
/// # Original
/// - Ghidra: `FUN_800e1f50` at `0x800E1F50` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1f50_FUN_800e1f50.c`
/// - Caller: `0x8015C4C0` writes `0x1E`.
///
/// # Arguments
/// * `id` - 16-bit value.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0x80428738`.
///
/// # Safety
/// None beyond the target address space.
///
/// # Confidence
/// high for the store; `low` for the meaning (`// inferred:` fade-out id).
pub unsafe fn scene_set_aux_id_a(id: u16) {
    platform::write_u16(G_SCENE_AUX_A, id);
}

/// Writes auxiliary scene id B.
///
/// # Original
/// - Ghidra: `FUN_800e1f60` at `0x800E1F60` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1f60_FUN_800e1f60.c`
/// - Caller: `0x8015C4C0` writes `0x1E`.
///
/// # Arguments
/// * `id` - 16-bit value.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0x8042873A`.
///
/// # Safety
/// None beyond the target address space.
///
/// # Confidence
/// high for the store; `low` for the meaning (`// inferred:` fade-in id).
pub unsafe fn scene_set_aux_id_b(id: u16) {
    platform::write_u16(G_SCENE_AUX_B, id);
}

// ===========================================================================
// Scene lifecycle helpers (`0x800E1D20..0x800E1DD0`).
//
// These sit between the base-object methods above and the transition globals:
// they release a scene, report whether a transition request is outstanding and
// ask for the default scene.  They are part of the same scene cluster.
// ===========================================================================

/// Tears a scene object down, optionally finishing the fade first.
///
/// In plain English: when a non-zero second argument is passed, cancel/stop the
/// fade machinery (`0x80107F10`, `0x80108260`); then hand the object and the
/// same flag to the generic scene destructor thunk `0x80162110`.
///
/// # Original
/// - Ghidra: `FUN_800e1d20` at `0x800E1D20` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1d20_FUN_800e1d20.c`
///
/// # Arguments
/// * `this` - scene object.
/// * `free` - `0` skips the fade teardown; non-zero runs it.
///
/// # Returns
/// Nothing (the thunk's result is not used in the C).
///
/// # Side effects
/// Calls `0x80107F10`, `0x80108260` and `0x80162110(this, free)`.
///
/// # Safety
/// `this` must be a live scene object; the fade manager must be initialised.
///
/// # Confidence
/// medium - the branches and calls are explicit, but both fade calls are shown
/// with no arguments, so their receiver is `// unknown:`.
pub unsafe fn scene_teardown(this: usize, free: i32) {
    if free != 0 {
        platform::call0(0x8010_7F10); // fade manager accessor (result unused)
        platform::call0(0x8010_8260);
    }
    platform::call2(0x8016_2110, this as u32, free as u32);
}

/// Reports whether `0x8006C610` returned non-zero.
///
/// `// observed:` the original computes `(-value | value) >> 31`, which is `1`
/// exactly when `value != 0`.
///
/// # Original
/// - Ghidra: `FUN_800e1d70` at `0x800E1D70` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1d70_FUN_800e1d70.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x8006C610` is non-zero, `0` otherwise.
///
/// # Side effects
/// Only whatever `0x8006C610` does (a scene-manager query).
///
/// # Safety
/// None beyond the target ABI.
///
/// # Confidence
/// high - the bit trick is fully visible; the query's role is `// unknown:`.
pub unsafe fn scene_is_transition_pending() -> u32 {
    let value = call0_ret(0x8006_C610);
    (value.wrapping_neg() | value) >> 31
}

/// Requests the default scene with `force = 1`.
///
/// # Original
/// - Ghidra: `FUN_800e1dd0` at `0x800E1DD0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e1dd0_FUN_800e1dd0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801017D0(0, 1)` - the engine wrapper that ends in
/// [`scene_request`].
///
/// # Safety
/// Must run on the game thread with the transition slot free.
///
/// # Confidence
/// high - one call, both arguments visible.
pub unsafe fn scene_request_default() {
    platform::call2(0x8010_17D0, 0, 1);
}

// ===========================================================================
// Scene sound dispatch (`0x800E2220`, `0x800E2340`).
//
// The two routines below turn a scene/sound id into a cue: both resolve the id
// through `0x8005FB90`, fall back to id `0` when the object rejects it, and then
// pass a table entry to the sequencer (`0x800B3540`) plus a secondary sound
// command.  They are the "scene lifecycle ends -> play sound" hooks.
// ===========================================================================

/// Starts a scene sound cue with a caller-supplied receiver and id.
///
/// In plain English: unless the id is `-1`, look it up; if the object exists
/// and its vtable slot `+0xE0` rejects it, use id `0` instead.  Then call the
/// sequencer with the table entry for `index`, and finally either the
/// per-index small command (`index < 5`) or the generic cue command.
///
/// # Original
/// - Ghidra: `FUN_800e2220` at `0x800E2220` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2220_FUN_800e2220.c`
/// - Sibling: `0x800E22F0` wraps this routine with a position vector.
///
/// # Arguments
/// * `_unused` - first argument; **not read** by the body.
/// * `receiver` - object passed to `0x800B3540` in `r3`.
/// * `index` - table index (`< 5` selects the small command path).
/// * `cue` - sound/scene id, `-1` means "no cue".
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8005FB90`, one vtable slot, `0x800B3540`, and
/// `0x80060690`/`0x80060460`; reads the two `0x802F2F..` tables.
///
/// # Safety
/// `receiver` must be acceptable to `0x800B3540`; `index` must be in range for
/// the tables (the original does not check).
///
/// # Confidence
/// medium - all branches are visible; the vtable call is printed with no
/// argument, so its receiver is `// unknown:`.
pub unsafe fn scene_play_sound(_unused: u32, receiver: u32, index: u32, cue: i32) {
    if cue == -1 {
        return;
    }

    let mut resolved = cue;
    let object = platform::call1(0x8005_FB90, cue as u32) as usize;
    if object != 0 {
        // `(**(code **)(*(int *)(object + 0x60) + 0xE0))()`: no visible args.
        let vtable = platform::read_u32(object + 0x60) as usize;
        let slot = platform::read_u32(vtable + 0xE0) as usize;
        if call0_ret(slot) != 0 {
            resolved = 0;
        }
    }

    // 0x800E2288: table entry for this index; third argument is the id.
    let entry = platform::read_u32(SOUND_TABLE_CUE + index as usize * 4);
    platform::call4(0x800B_3540, receiver, entry, resolved as u32, 0);

    if index < 5 {
        let small = platform::read_u32(SOUND_TABLE_SMALL + index as usize * 4);
        platform::call2(0x8006_0690, small, resolved as u32);
    } else {
        platform::call3(0x8006_0460, resolved as u32, 1, 1);
    }
}

/// Starts a scene sound cue by table index only.
///
/// In plain English: id `8` takes the special path (`0x800B35B0` plus a
/// constant cue `1`); every other id resolves the object, reverb entry and cue
/// entry from the two `0x802F2F..` tables and passes them to the sequencer.
///
/// # Original
/// - Ghidra: `FUN_800e2340` at `0x800E2340` (128 bytes)
/// - Extent: `0x800E2340..0x800E23C0` (128 bytes), closing `blr` at `0x800E23BC`.
/// - Evidence: `work/ghidra/main-gb/export/800e2340_FUN_800e2340.c`
///
/// # Arguments
/// * `_unused` - first argument; **not read** by the body.
/// * `receiver` - object passed to `0x800B3540` in `r3`.
/// * `index` - table index; `8` is the special case.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800B35B0`, `0x80060590`, `0x80060690` and `0x800B3540`.
///
/// # Safety
/// `receiver` must be acceptable to `0x800B3540`; `index` must index both
/// tables.
///
/// # Confidence
/// high - every branch, table base and constant is visible; table value
/// meanings are `// unknown:`.
pub unsafe fn scene_play_sound_index(_unused: u32, receiver: u32, index: i32) {
    if index == 8 {
        platform::call1(0x800B_35B0, receiver);
        platform::call1(0x8006_0590, 1);
        return;
    }

    let offset = index as u32 as usize * 4;
    let small = platform::read_u32(SOUND_TABLE_CUE_B + offset);
    // Observed: the small command runs first and passes -1 as its second word.
    platform::call2(0x8006_0690, small, 0xFFFF_FFFF);
    let reverb = platform::read_u32(SOUND_TABLE_REVERB + offset);
    platform::call4(0x800B_3540, receiver, reverb, 0xFFFF_FFFF, 0);
}


// ===========================================================================
// Wave 277 lane E (`_E277`) - gap batch 7, scene-band body
// ===========================================================================
//
// One untabled gap body from the wave-266 candidate set, re-verified against
// `work/dol/main.elf` with the current documented extents. Selection:
// `work/wave277/e_pick.mjs`; ELF re-walk and disassembly: `e_verify.mjs` and
// `e_verify_rest.txt`.

/// Releases a task's factory allocation (third release slot of the array).
///
/// Same body as [`save_task_release`] (`0x800CED70`, which this file already
/// documents). `0x800CED30` is the third 12-byte `{code, 0, 0}` descriptor of
/// the array at `0x80317A44`; the entries are `0x800CFBC0`, `0x800CED70`,
/// `0x800CED30` and `0x800D09D0`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800CED30`.
/// - Extent: `0x800CED30..0x800CED70` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x800CED6C`; two zero words precede it at
///   `0x800CED28..0x800CED30` and [`save_task_release`] starts at `0x800CED70`.
/// - Reach: no `b` or `bl` in the executable segments targets it. The one data
///   word naming it, `0x80317A5C`, is a descriptor word (`{code, 0, 0}`), not a
///   pointer-run interior.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`; `bc 4,1,+0x08` (ble);
///   `bl 0x802B9280`; `mr r3,r31`; `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`;
///   `mtlr r0`; `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `task`: task pointer; `0` is accepted and returns immediately.
/// - `free`: `>0` asks `0x802B9280` to free the block.
///
/// # Returns
/// `task`, so the caller can chain the release.
///
/// # Side effects
/// Possible `0x802B9280(task)` free.
///
/// # Safety
/// `task` must be a live task or `0`.
///
/// # Confidence
/// high - the body is byte-identical to [`save_task_release`] apart from the
/// `bl` displacement, and the descriptor slot proves the release role.
pub unsafe fn fn_800ced30(task: usize, free: i32) -> usize {
    if task != 0 && free > 0 {
        platform::call1(0x802B_9280, task as u32);
    }
    task
}

// ---------------------------------------------------------------------------
// Wave 281 lane B - the `dScene_c` base slots of the `dScBoot_c` vtable
// (`_B281`)
//
// `0x80328180` is the `dScBoot_c` primary vtable installed at `+0x60` by
// `dsc_boot_construct` (`0x8015C130`). Slots 2, 4, 5 and 11 of that table still
// point at the base scene table `0x8031AFC0`, and those base bodies were the
// four open words in the wave-281 census of `0x80328188`. They are translated
// here, next to the rest of the `0x800E18xx..0x800E1Fxx` scene cluster.
// ---------------------------------------------------------------------------

/// `r13 - 0x6F4A` -> `0x80428736`: scene byte cleared by [`fn_800e1b80`].
///
/// It sits two bytes below the finished flag (`0x8042873C`); `// inferred:` it
/// is a second scene-state latch because the same block is cleared by the base
/// scene's teardown path.
const G_SCENE_FLAG_6F4A: usize = SDA_BASE - 0x6F4A;

/// Prepares the scene banks and enters the stage-bank transition.
///
/// In plain English: when the caller passes mode `2`, cancel the current fade,
/// build the scene's three effect objects and create scene resource `0x2ED`
/// with `this` as owner; every mode then forwards to `0x8006C570`, which is a
/// tail jump to the stage-bank transition `0x80161DA0`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row and no export file for
///   `0x800E1AE0`; the body was read from `work/dol/main.elf`.
/// - Extent: `0x800E1AE0..0x800E1B48` (104 bytes, 26 instructions), ending at
///   the `blr` at `0x800E1B44`. The preceding word at `0x800E1ADC` is zero and
///   `0x800E1AD0..0x800E1AD8` is the previous function's epilogue.
/// - Table: `0x80328188` slot 2 (`0x80328190`), the `dScBoot_c` vtable; the
///   same word is slot 2 of the base table `0x8031AFC0` (`0x8031AFD0`).
/// - Observed: `cmpi cr0,0,r4,2`; `bc 4,2` (`bne`) around `bl 0x80107F10`,
///   `bl 0x80108230`, `bl 0x80069820`, `li r3,749`, `mr r4,r30`, `li r5,0`,
///   `li r6,0`, `bl 0x80162B00`; then `mr r3,r30`; `mr r4,r31`;
///   `bl 0x8006C570`.
///
/// # Arguments
/// * `this` - the scene object (`r3`).
/// * `mode` - change mode; `2` selects the resource/bank creation branch.
///
/// # Returns
/// The value returned by `0x8006C570` (`b 0x80161DA0`).
///
/// # Side effects
/// On mode `2`: one fade-context read, one fade phase advance, three effect
/// objects created, and one resource `0x2ED` creation with `this` as owner.
/// Then the stage-bank transition always runs.
///
/// # Safety
/// `this` must be a live scene object and the fade/resource/task systems must
/// be initialised.
///
/// # Confidence
/// high - the mode test, the two fade calls, the resource id `749`, the owner
/// register and the tail call are literal. `// inferred:` `749` is a resource
/// id because `0x80162B00` is the resource creator.
pub unsafe fn fn_800e1ae0(this: usize, mode: i32) -> u32 {
    if mode == 2 {
        // 0x800E1B04: the fade context is returned in r3, then consumed by
        // 0x80108230 (it loads `r3 + 8`).
        let fade = call0_ret(0x8010_7F10);
        let _ = platform::call1(0x8010_8230, fade);
        platform::call0(0x8006_9820);
        // 0x800E1B20: resource creator `(id, owner, 0, 0)`; r4 = r30 = this.
        let _ = platform::call4(0x8016_2B00, 0x2ED, this as u32, 0, 0);
    }
    // 0x800E1B2C: `bl 0x8006C570`, itself a `b 0x80161DA0`.
    platform::call2(0x8006_C570, this as u32, mode as u32)
}

/// Reports whether the scene's resource/detach queue is ready.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800E1B50`.
/// - Extent: `0x800E1B50..0x800E1B7C` (44 bytes, 11 instructions), ending at
///   the `blr` at `0x800E1B78`.
/// - Table: `0x80328188` slot 4 (`0x80328198`), the `dScBoot_c` vtable; the
///   same word is slot 4 of the base table `0x8031AFC0` (`0x8031AFD8`).
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `bl 0x8006C580`; `neg r0,r3`; `or r0,r0,r3`;
///   `rlwinm r3,r0,1,31,31`; epilogue; `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x8006C580` returns non-zero, `0` otherwise.
///
/// # Side effects
/// Only whatever `0x8006C580` does (it wraps `0x80161EF0`, the queue gate).
///
/// # Safety
/// None beyond the target ABI; the queue object must be initialised for the
/// callee to read it.
///
/// # Confidence
/// high - the boolean idiom is `(value != 0)` and the body is byte-identical
/// to the already-translated `0x8006C540` wrapper shape.
pub unsafe fn fn_800e1b50() -> u32 {
    // 0x800E1B5C: `bl 0x8006C580`; the callee already returns 0/1.
    call0_ret(0x8006_C580)
}

/// Detaches the scene's stage banks when the scene is being torn down.
///
/// In plain English: when the caller passes mode `2`, clear the scene byte at
/// `0x80428736`, cancel the current fade and release its controller phase; every
/// mode then forwards to `0x8006C5B0`, which is a tail jump to the record detach
/// `0x80161F60`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800E1B80`.
/// - Extent: `0x800E1B80..0x800E1BD8` (88 bytes, 22 instructions), ending at
///   the `blr` at `0x800E1BD4`.
/// - Table: `0x80328188` slot 5 (`0x8032819C`), the `dScBoot_c` vtable; the
///   same word is slot 5 of the base table `0x8031AFC0` (`0x8031AFDC`).
/// - Observed: `cmpi cr0,0,r4,2`; `bc 4,2` (`bne`) around
///   `li r0,0`; `stb r0,-0x6F4A(r13)`; `bl 0x80107F10`; `bl 0x801082D0`;
///   then `mr r3,r30`; `mr r4,r31`; `bl 0x8006C5B0`.
///
/// # Arguments
/// * `this` - the scene object (`r3`).
/// * `mode` - change mode; `2` selects the detach branch.
///
/// # Returns
/// The value returned by `0x8006C5B0` (`b 0x80161F60`).
///
/// # Side effects
/// On mode `2`: clears one SDA byte, reads the fade context and releases its
/// controller phase. Then the record detach always runs.
///
/// # Safety
/// `this` must be a live scene object and the fade system must be initialised.
///
/// # Confidence
/// high - the mode test, the SDA byte store, both calls and the tail call are
/// literal.
pub unsafe fn fn_800e1b80(this: usize, mode: i32) -> u32 {
    if mode == 2 {
        platform::write_u8(G_SCENE_FLAG_6F4A, 0);
        // 0x800E1BAC: r3 is the fade context for 0x801082D0 (`r3 + 8`).
        let fade = call0_ret(0x8010_7F10);
        let _ = platform::call1(0x8010_82D0, fade);
    }
    // 0x800E1BBC: `bl 0x8006C5B0`, itself a `b 0x80161F60`.
    platform::call2(0x8006_C5B0, this as u32, mode as u32)
}

/// Scene virtual slot 11 - a register-transparent no-op.
///
/// In plain English: the original is the three-word jump chain
/// `0x800E1DA0: b 0x8006C650`, `0x8006C650: b 0x80162210`,
/// `0x80162210: blr`. Nothing is stored or called, so the incoming `r3` is
/// returned unchanged and every other register is preserved.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800E1DA0`.
/// - Extent: `0x800E1DA0..0x800E1DA4` (4 bytes, 1 instruction: `b 0x8006C650`).
/// - Table: `0x80328188` slot 11 (`0x803281B4`), the `dScBoot_c` vtable; the
///   same word is slot 11 of the base table `0x8031AFC0` (`0x8031AFF4`).
/// - Observed: `4BF8A8B0 b 0x8006C650`; the target is `480F5BC0 b 0x80162210`
///   and `0x80162210` is `4E800020 blr`.
///
/// # Arguments
/// * `value` - the incoming `r3`.
///
/// # Returns
/// `value`, unchanged.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the original only jumps and returns.
///
/// # Confidence
/// high - all three instructions were read from the image.
pub unsafe fn fn_800e1da0(value: u32) -> u32 {
    value
}
