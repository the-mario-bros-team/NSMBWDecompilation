//! Engine start-up: how the boot thread hands the machine over to the game thread.
//!
//! ## What this module covers
//!
//! Original address range `0x800C9F10..0x800CA05C` - four short functions that
//! sit next to each other at the end of the game's `main` translation unit:
//!
//! | address | size | name here | what it does |
//! |---|---|---|---|
//! | `0x800C9F90` | 204 B | [`start_main_thread`] | creates the game thread, then parks the boot thread |
//! | `0x800C9F60` | 40 B | [`game_main`] | thread entry: start the engine, then loop frames forever |
//! | `0x800C9F10` | 40 B | [`game_startup`] | runs the engine's `dSys` initialiser |
//! | `0x800C9F40` | 20 B | [`frame_loop`] | `do { one_game_frame(); } while (true)` |
//!
//! ## Why it exists
//!
//! `0x80004050` (the DOL entry - see `docs/BOOT_CHAIN.md`) never calls a game
//! `main` directly. After the CRT, the OS init and the boot-info relocation it
//! calls `0x800C9F90`; that function spawns the real game thread and suspends
//! the boot thread. This module records that hand-off.
//!
//! ## The question this lane was asked
//!
//! `0x800C9F90` **is not the game main**. It is a thread bootstrap/hand-off.
//! The game main is `0x800C9F60`, the entry point of the thread it creates.
//! The evidence for that conclusion is in `docs/ENGINE_STARTUP.md` (§2).
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! * Runtime lane (OS), called by absolute address, **not** translated here:
//!   `0x801B5F80` `OSGetTime`, `0x801B4B30` `OSGetCurrentThread`,
//!   `0x801B5F70` thread base-priority read, `0x801B5130` `OSCreateThread`,
//!   `0x801B5860` `OSResumeThread`, `0x801B5E80` `OSSetThreadPriority`,
//!   `0x801B5B00` `OSSuspendThread`.
//! * Engine subsystems, called by absolute address, **not** translated here:
//!   `0x800E51A0` arena bounds, `0x800E48D0` / `0x800E47A0` `dSys` start-up,
//!   `0x800E4E40` one game frame.
//!
//! ## Start here
//!
//! Read [`start_main_thread`], then [`game_main`] and [`frame_loop`].
//!
//! ## Glossary (first use)
//!
//! * **DOL** - the Wii executable container (`sys/main.dol`); its entry point is
//!   `0x80004050`.
//! * **r13 / SDA** - the PowerPC small-data base register and the globals
//!   addressed through it. `0x800041C0` sets `r13 = 0x8042F680`, so
//!   "`r13 - 0x56E8`" below means the absolute address `0x80429F98`.
//! * **MEM1** - the 24 MB of main RAM at `0x80000000`; **MEM2** is the extra 64 MB.
//! * **OSThread** - the SDK's thread object. The fields used here are
//!   `+0x2D4` (base priority) and `+0x304`/`+0x308` (stack top / stack bottom).
//! * **MetroTRK** - Nintendo's target-resident debugger. It is *not* part of
//!   this module; see [`crate::engine::main_loop`].

use crate::platform;

/// Small-data anchor: the value `0x800041C0` loads into `r13`.
///
/// Every `r13 ± offset` global in this file is resolved against it, which
/// replaces the `unaff_r13` placeholders in the Ghidra output.
///
/// # Original
/// - Ghidra: `FUN_800041c0` at `0x800041c0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800041c0_FUN_800041c0.c`
/// - Observed: `0x80004244 lis r13,0x8042` / `0x80004248 ori r13,r13,0xF680`
pub const SDA_BASE: usize = 0x8042_F680;

/// Absolute address of the boot time-base snapshot (`r13 - 0x56E8` in the
/// original).
///
/// Holds the **high** 32 bits of the `u64` written by [`start_main_thread`];
/// the low half is at [`G_BOOT_TIME_LO`], so the value is a big-endian `u64` at
/// this address.
///
/// # Original
/// - Ghidra: `FUN_800c9f90` at `0x800C9f90`
/// - Evidence: `work/ghidra/main-gb/export/800c9f90_FUN_800c9f90.c`
/// - Observed: `0x800C9FD4 stw r3,-0x56E8(r13)`
pub const G_BOOT_TIME_HI: usize = SDA_BASE - 0x56E8; // 0x80429F98

/// Absolute address of the low 32 bits of the boot time-base snapshot
/// (`r13 - 0x56E4` in the original).
///
/// # Original
/// - Ghidra: `FUN_800c9f90` at `0x800C9f90`
/// - Evidence: `work/ghidra/main-gb/export/800c9f90_FUN_800c9f90.c`
/// - Observed: `0x800C9FD0 stw r4,-0x56E4(r13)`
pub const G_BOOT_TIME_LO: usize = SDA_BASE - 0x56E4; // 0x80429F9C

/// The `OSThread` object the game main runs on.
///
/// `// inferred:` from the call shape: `0x801B5130` writes an `OSThread`
/// (`+0x2C8` state, `+0x2CC` suspend count, `+0x2D0/+0x2D4` priority,
/// `+0x304/+0x308` stack) at this address. It is in BSS
/// (`0x80351680..0x8042FC3C`), so the object is zero at boot.
///
/// # Original
/// - Ghidra: `FUN_800c9f90` at `0x800C9f90` (`DAT_80359938` in the C output)
/// - Observed: `0x800C9FF4 lis r30,0x8036` / `0x800CA00C addi r3,r30,-0x66C8`
pub const G_MAIN_THREAD: usize = 0x8035_9938;

/// Entry point of [`G_MAIN_THREAD`] - the game main, [`game_main`].
///
/// # Original
/// - Ghidra: `FUN_800c9f60` at `0x800C9f60`
/// - Evidence: `work/ghidra/main-gb/export/800c9f60_FUN_800c9f60.c`
/// - Observed: `0x800CA004 addi r4,r4,-0x60A0` with `r4 = 0x800D0000`
pub const GAME_MAIN_ENTRY: usize = 0x800C_9F60;

/// Size of the game main's stack, in bytes: `0xF000` = 61 440 = 60 KiB.
///
/// The stack is a local array inside [`start_main_thread`]'s own frame, so the
/// boot thread reserves the memory before the game thread exists.
///
/// # Original
/// - Ghidra: `FUN_800c9f90` at `0x800C9f90` (argument 5 of `OSCreateThread`)
/// - Observed: `0x800C9FF8 lis r5,0x1` / `0x800CA008 addi r7,r5,-0x1000`
pub const MAIN_STACK_SIZE: usize = 0xF000;

/// `OSGetTime` - returns the 64-bit time base, high word in `r3`, low in `r4`.
///
/// The body is the standard stable read: sample TBU, TBL, TBU again and retry
/// until the two TBU samples match. Ghidra renders it as an empty function
/// because the body is three `mftb` instructions.
///
/// # Original
/// - Ghidra: `FUN_801b5f80` at `0x801b5f80` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b5f80_FUN_801b5f80.c`
/// - Observed: `mftb r3,TBU / mftb r4,TBL / mftb r5,TBU / cmp / bne -0x10 / blr`
const OS_GET_TIME: usize = 0x801B_5F80;

/// `0x800E51A0` - guarantees the OS heap arena covers `[0x806E0000, 0x817F0000]`.
///
/// It reads `OSGetArenaHi` (`0x801ABE90`) / `OSGetArenaLo` (`0x801ABE60`), stores
/// them plus the two fixed bounds in SDA, and raises/lowers the arena through
/// `0x801ABEF0` / `0x801ABEC0` when the current bounds fall short. This runs
/// before the game thread carves the MEM1/MEM2 root heaps out of the arena.
///
/// # Original
/// - Ghidra: `FUN_800e51a0` at `0x800e51a0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e51a0_FUN_800e51a0.c`
const ARENA_INIT: usize = 0x800E_51A0;

/// `OSGetCurrentThread` - the thread pointer cached at `0x800000E4`.
///
/// Kept for documentation only: the translation reads `0x800000E4` directly,
/// because [`crate::platform::call0`] cannot return a value.
///
/// # Original
/// - Ghidra: `FUN_801b4b30` at `0x801b4b30` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b4b30_FUN_801b4b30.c`
const OS_GET_CURRENT_THREAD: usize = 0x801B_4B30;

/// Field `+0x2D4` of `OSThread`: the thread's **base** priority, i.e. the value
/// `OSSetThreadPriority` would restore. `0x801B5F70` is just `lwz r3,0x2D4(r3)`.
///
/// # Original
/// - Ghidra: `FUN_801b5f70` at `0x801b5f70` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b5f70_FUN_801b5f70.c`
const OS_THREAD_BASE_PRIORITY: usize = 0x801B_5F70;

/// `OSCreateThread(thread, func, val, stack, stack_size, prio, attr)`.
///
/// `// inferred:` role proven by the body: it rejects priorities outside
/// `0..=31`, fills the `OSThread` fields listed on [`G_MAIN_THREAD`], points the
/// new context at the thread trampoline `0x801B53A0`, stores the stack top at
/// `+0x304`, `stack_top - size` at `+0x308`, and writes the `0xDEADBABE`
/// overflow guard word at that low address. The guard proves that the `stack`
/// argument is the **top** of the area, not the bottom.
///
/// # Original
/// - Ghidra: `FUN_801b5130` at `0x801b5130` (620 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b5130_FUN_801b5130.c`
/// - Observed: `0x801B5144` priority range check, `0x801B51E0`
///   `lis 0xDEAE / addi -17730` = `0xDEADBABE` stored at `stack_top - size`
const OS_CREATE_THREAD: usize = 0x801B_5130;

/// `OSResumeThread(thread)` - decrements `+0x2CC` (the suspend count) and, when
/// it reaches 0, puts the thread on the ready queue for its priority.
///
/// # Original
/// - Ghidra: `FUN_801b5860` at `0x801b5860` (660 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b5860_FUN_801b5860.c`
const OS_RESUME_THREAD: usize = 0x801B_5860;

/// `OSSetThreadPriority(thread, prio)` - rejects `prio > 31`, updates `+0x2D4`
/// and re-queues the thread if it is not suspended.
///
/// # Original
/// - Ghidra: `FUN_801b5e80` at `0x801b5e80` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b5e80_FUN_801b5e80.c`
/// - Observed: `0x801B5E88 cmpi r4,0` / `0x801B5EA8 cmpi r4,31` / `+0x2D4`
const OS_SET_THREAD_PRIORITY: usize = 0x801B_5E80;

/// `OSSuspendThread(thread)` - increments `+0x2CC`, removes the thread from the
/// ready queue, and requests a reschedule (`0x801B4EA0`, a full context switch)
/// when it was running. Returns the previous suspend count.
///
/// # Original
/// - Ghidra: `FUN_801b5b00` at `0x801b5b00` (392 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801b5b00_FUN_801b5b00.c`
const OS_SUSPEND_THREAD: usize = 0x801B_5B00;

/// Lowest thread priority in the SDK (0 = highest, 31 = lowest).
///
/// `// inferred:` the value the boot thread demotes itself to, so that the game
/// thread it just created runs first.
///
/// # Original
/// - Observed: `0x800CA02C li r4,0x1F` before `OSSetThreadPriority`
const THREAD_PRIO_LOWEST: u32 = 0x1F;

/// `OSThread` field holding the current thread pointer (`lwz r3,0xE4(0x80000000)`).
///
/// # Original
/// - Ghidra: `FUN_801b4b30` at `0x801b4b30`
/// - Observed: `0x801B4B30 lis r3,0x8000` / `0x801B4B34 lwz r3,0xE4(r3)`
const OS_CURRENT_THREAD_PTR: usize = 0x8000_00E4;

// ---------------------------------------------------------------------------
// Local call helpers.
//
// `crate::platform` exposes `call0..call3` only. `OSCreateThread` takes seven
// arguments and `OSGetTime` returns 64 bits, so those two calls need their own
// signatures. `platform.rs` is outside this lane and must not change.
// ---------------------------------------------------------------------------

/// Calls a PowerPC function with seven integer arguments (`r3..r9`).
///
/// # Original
/// Not present in the binary: this is a local signature helper for the
/// `OSCreateThread` call at `0x800CA01C`, whose arguments are set up in
/// `r3..r9` between `0x800C9FE8` and `0x800CA018`.
///
/// # Arguments
/// * `addr` - virtual address of the callee.
/// * `a1..a7` - values for `r3..r9`.
///
/// # Returns
/// The callee's `r3`.
///
/// # Side effects
/// Whatever the callee does.
///
/// # Safety
/// `addr` must point at a PowerPC function with this exact ABI and the caller
/// must be running on a PowerPC target. This module is a source
/// reconstruction, not a host build.
///
/// # Confidence
/// high - the argument order is read directly from the `r3..r9` setup at
/// `0x800C9FE8..0x800CA018`.
unsafe fn call7(
    addr: usize,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32) -> u32);
    f(a1, a2, a3, a4, a5, a6, a7)
}

/// Calls a PowerPC function that returns a 64-bit value.
///
/// # Original
/// Not present in the binary: this is a local signature helper for the
/// `OSGetTime` call at `0x800C9FCC`, which returns the time base in `r3:r4`
/// (Metrowerks EABI returns `u64` in a register pair; `extern "C"` agrees on
/// 32-bit PowerPC).
///
/// # Arguments
/// * `addr` - virtual address of the callee.
///
/// # Returns
/// The callee's 64-bit result.
///
/// # Side effects
/// Whatever the callee does.
///
/// # Safety
/// `addr` must point at a PowerPC function returning `u64`, and the caller must
/// be running on a PowerPC target.
///
/// # Confidence
/// high - `0x801B5F80` ends with `r3 = TBU`, `r4 = TBL`, and the two stores at
/// `0x800C9FD0`/`0x800C9FD4` treat the pair as one big-endian 64-bit value.
unsafe fn call_u64(addr: usize) -> u64 {
    let f = crate::platform::guest_fn!(addr, fn() -> u64);
    f()
}

// ---------------------------------------------------------------------------
// 0x800C9F90
// ---------------------------------------------------------------------------

/// Creates the game's main thread and then parks the thread that called it.
///
/// In plain English: snapshot the time base into a global, make sure the heap
/// arena covers the expected MEM1 window, create a thread that starts at
/// `0x800C9F60` on a 60 KiB stack carved out of this function's own frame at
/// the *caller's* priority, resume it, then drop the caller to the lowest
/// priority and suspend it. From that moment the game thread owns the machine.
///
/// # Original
/// - Ghidra: `FUN_800c9f90` at `0x800C9f90` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800c9f90_FUN_800c9f90.c`
/// - Call site: `0x800041B4` in `80004050_entry.c`
/// - Instruction check: `work/dol/main.elf`, `0x800C9F90..0x800CA058`
///
/// # Arguments
/// * `_module_count` - `r3` at entry. The DOL entry passes `r14`, the count from
///   the boot-info module list (`0` when there is none). **Never read.**
/// * `_module_list` - `r4` at entry. The DOL entry passes `r15`, the
///   32-byte-aligned module-list pointer (`0` when there is none).
///   **Never read.** Both parameters are kept so the call site stays faithful.
///
/// # Returns
/// Nothing. `// inferred:` control only reaches the epilogue if the boot thread
/// is resumed later; `OSSuspendThread` on the running thread normally
/// reschedules away first.
///
/// # Side effects
/// * writes an 8-byte big-endian time base to `0x80429F98` ([`G_BOOT_TIME_HI`]);
/// * indirectly clamps the OS arena (`0x800E51A0`);
/// * initialises the `OSThread` at `0x80359938` ([`G_MAIN_THREAD`]) and makes
///   `0x800C9F60` ([`GAME_MAIN_ENTRY`]) runnable;
/// * sets the calling thread's priority to 31 and suspends it.
///
/// # Safety
/// Must run once, on the initial boot thread, before the scheduler has other
/// runnable work. It suspends the caller, so any code after it in the caller
/// runs only if that thread is resumed; the stack it hands out must stay alive
/// (here: it is a local array, exactly as in the original).
///
/// # Confidence
/// high - all 204 bytes are accounted for; only the *name* is descriptive.
pub unsafe fn start_main_thread(_module_count: u32, _module_list: u32) {
    // Step 1 - 0x800C9FCC: `bl 0x801B5F80` = OSGetTime().
    // r3 = TBU (high word), r4 = TBL (low word).
    let boot_time = call_u64(OS_GET_TIME);

    // 0x800C9FD0 / 0x800C9FD4: two 32-bit stores, low word first in address
    // order, so the 8 bytes at 0x80429F98 are a big-endian u64.
    platform::write_u32(G_BOOT_TIME_HI, (boot_time >> 32) as u32);
    platform::write_u32(G_BOOT_TIME_LO, boot_time as u32);

    // Step 2 - 0x800C9FD8: `bl 0x800E51A0`, arena bounds maintenance.
    platform::call0(ARENA_INIT);

    // Step 3 - 0x800C9FDC..0x800C9FE4:
    //   `bl 0x801B4B30` -> OSGetCurrentThread() (a load from 0x800000E4, inlined
    //                     here because platform::call0 cannot return a value)
    //   `bl 0x801B5F70` -> *(thread + 0x2D4), the caller's base priority
    let current = platform::read_u32(OS_CURRENT_THREAD_PTR);
    let caller_priority = platform::call1(OS_THREAD_BASE_PRIORITY, current);

    // Step 4 - the stack, and why it lives here.
    // Original prologue, 0x800C9F90..0x800C9FC8:
    //   rlwinm r11,r1,0,27,31   r11 = r1 & 0x1F      (SP misalignment)
    //   mr     r12,r1           r12 = caller SP
    //   subfic r11,r11,-64      r11 = -64 - (r1 & 0x1F)
    //   stwux  r1,r1,r11        open a small frame, 32-byte aligned + 0x40 pad
    //   ... save LR / r31 / r30 / r29 at caller SP +4 / -4 / -8 / -12 ...
    //   stwux  r0,r1,-0xF020    open the big frame: 0xF020 = 0xF000 + 0x20
    // The 60 KiB stack is that big frame. Its top is `r31 + 0x20`, which is what
    // the call below passes; the 0x20 is the pad between the array and the top.
    let mut stack = crate::platform::GuestLocal::new( [0u8; MAIN_STACK_SIZE]);
    let frame_top = (stack.as_mut_ptr() as usize + MAIN_STACK_SIZE) & !0x1F; // ~ r31, 32-byte aligned
    let stack_top = frame_top + 0x20; // observed: r6 = r31 + 0x20

    // Step 5 - 0x800C9FE8..0x800CA01C, argument setup then `bl 0x801B5130`:
    //   r3 = &0x80359938 (thread)   r4 = 0x800C9F60 (entry)
    //   r5 = 0 (thread argument)    r6 = stack_top
    //   r7 = 0xF000 (stack size)    r8 = caller_priority
    //   r9 = 0 (attribute: not detached)
    call7(
        OS_CREATE_THREAD,
        G_MAIN_THREAD as u32,
        GAME_MAIN_ENTRY as u32,
        0,
        stack_top as u32,
        MAIN_STACK_SIZE as u32,
        caller_priority,
        0,
    );

    // Step 6 - 0x800CA020..0x800CA024: make the new thread runnable.
    platform::call1(OS_RESUME_THREAD, G_MAIN_THREAD as u32);

    // Step 7 - 0x800CA028..0x800CA030: `mr r3,r29` (current thread), `li r4,31`
    // then `bl 0x801B5E80`; drop the boot thread to the lowest priority so the
    // game thread (same priority as before) is scheduled first.
    platform::call2(OS_SET_THREAD_PRIORITY, current, THREAD_PRIO_LOWEST);

    // Step 8 - 0x800CA034..0x800CA038: `mr r3,r29` then `bl 0x801B5B00`.
    // `// inferred:` suspending the running thread deschedules through
    // 0x801B4EA0, so the DOL entry's tail (0x801B8970 / 0x801B89E0) is reached
    // only if this thread is ever resumed again.
    platform::call1(OS_SUSPEND_THREAD, current);
}

// ---------------------------------------------------------------------------
// 0x800C9F60 - the game main, and its two small callees.
// Translated here only because they identify the thread entry that
// 0x800C9F90 starts.
// ---------------------------------------------------------------------------

/// Entry point of the game thread: run the one-time `dSys` start-up, then loop
/// frames forever.
///
/// In plain English: "initialise the engine, then draw/update a frame until the
/// console is switched off."
///
/// # Original
/// - Ghidra: `FUN_800c9f60` at `0x800C9f60` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800c9f60_FUN_800c9f60.c`
/// - Observed: `0x800C9F6C bl 0x800C9F10`, `0x800C9F70 bl 0x800C9F40`
///
/// # Arguments
/// * `r3` at entry is the thread argument, which `OSCreateThread` was given as
///   `0`; the body ignores it.
///
/// # Returns
/// `0` in `r3` per the epilogue (`0x800C9F78 li r3,0`), which is unreachable
/// because [`frame_loop`] never returns.
///
/// # Side effects
/// All of [`game_startup`], then everything [`frame_loop`] does.
///
/// # Safety
/// Runs on the game thread with the 60 KiB stack from [`start_main_thread`].
///
/// # Confidence
/// high - `0x800E48D0` carries the `dSys_c::RootHeapMEM1/2` strings and
/// `0x800E4E40` bumps a frame counter, so "engine start-up + frame loop" is
/// observed, not guessed.
#[allow(unreachable_code)] // the original still emits `li r3,0` after the never-returning loop
pub unsafe fn game_main() -> u32 {
    game_startup(); // 0x800C9F6C
    frame_loop(); // 0x800C9F70 (never returns)
    0 // 0x800C9F78: the original still emits `li r3,0`
}

/// One-time engine start-up: `dSys` initialisation plus one follow-up call.
///
/// # Original
/// - Ghidra: `FUN_800c9f10` at `0x800C9f10` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800c9f10_FUN_800c9f10.c`
/// - Observed: `FUN_800e48d0(); FUN_800e47a0(0);`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x800E48D0` creates the MEM1/MEM2 root heaps and brings the engine
/// subsystems up; `0x800E47A0(0)` is a follow-up in the same subsystem (not
/// analysed in this lane).
///
/// # Safety
/// Must run once, before the first frame, on the game thread.
///
/// # Confidence
/// high for the call order; the *content* of `0x800E48D0` belongs to the engine
/// subsystem lanes.
pub unsafe fn game_startup() {
    platform::call0(0x800E_48D0); // dSys init: root heaps MEM1/MEM2, subsystems
    platform::call1(0x800E_47A0, 0); // follow-up, argument 0 (see 800e47a0_FUN_800e47a0.c)
}

/// Runs game frames forever: `do { frame(); } while (true)`.
///
/// # Original
/// - Ghidra: `FUN_800c9f40` at `0x800C9f40` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800c9f40_FUN_800c9f40.c`
/// - Observed: `0x800C9F4C bl 0x800E4E40`, `0x800C9F50 b 0x800C9F4C`
///
/// # Arguments
/// None.
///
/// # Returns
/// Never. The `stwu`/`mfspr LR` frame in the original is dead code.
///
/// # Side effects
/// Each iteration runs `0x800E4E40`, which drives the render/audio/input/actor
/// steps and increments a frame counter at `r13 - 0x5318`.
///
/// # Safety
/// Never returns, so anything after it is unreachable - same as the original.
///
/// # Confidence
/// high - the loop back-edge is a plain `b` to the call.
pub unsafe fn frame_loop() -> ! {
    loop {
        platform::call0(0x800E_4E40); // one game frame
    }
}
