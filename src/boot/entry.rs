//! Boot entry and CRT (C runtime) register setup for the DOL image (the Wii
//! executable container that `sys/main.dol` stores).
//!
//! # What this module covers
//!
//! Original address range `0x80004000..=0x8000432F`, the first `0x330` bytes of
//! `.text`. These routines run before any engine code:
//!
//! | Address  | Item | Role |
//! |----------|------|------|
//! | `0x80004000` | [`fn_80004000`] | conditional call to the obsolete `OSResetSystem` path |
//! | `0x80004030` | [`fn_80004030`] | set the boot flag byte at `r13 - 0x4F48` |
//! | `0x80004040` | [`fn_80004040`] | read that boot flag byte |
//! | `0x80004050` | [`entry`] | DOL entry point (`__start`; `// inferred:` SDK role) |
//! | `0x800041C0` | [`init_registers`] | zero the GPRs, install r1 / r2 / r13 |
//! | `0x80004300` | [`fn_80004300`] | enable `MSR[FP]`, then call three init routines |
//!
//! The table copy/clear pass that `entry` runs is in
//! [`crate::boot::init_tables`]; the primitives it uses are in
//! [`crate::boot::mem`] and [`crate::boot::cache`].
//!
//! # Why it exists
//!
//! The Wii loader jumps to `0x80004050` with no useful register state. This
//! module establishes the stack, the two small-data bases (r2/r13) used by all
//! CodeWarrior-compiled code, copies the early data tables, relocates the
//! module list that the loader found in the boot-info block, and finally hands
//! control to the thread starter at `0x800C9F90` (see
//! [`crate::engine::startup`]) or to the MetroTRK debugger path.
//!
//! Terms: *MEM1* is the Wii's 24 MiB main RAM; *small data* is the r2/r13-
//! relative region CodeWarrior uses to reach globals with one instruction;
//! *GPR* means general-purpose register.
//!
//! # Start here
//!
//! [`entry`]. Every other function in this file is called by it (directly or
//! through the boot-type dispatch).
//!
//! # Dependencies
//!
//! * [`crate::platform`] - volatile big-endian memory access and indirect calls.
//! * [`crate::boot::init_tables`] - copies/clears the linker tables at
//!   `0x80006618` and `0x8000669C`.
//!
//! # Evidence
//!
//! Ghidra (Gekko_Broadway) output under `work/ghidra/main-gb/export/`:
//! `80004000_FUN_80004000.c`, `80004030_FUN_80004030.c`,
//! `80004040_FUN_80004040.c`, `80004050_entry.c`,
//! `800041c0_FUN_800041c0.c`, `80004300_FUN_80004300.c`.
//! Where the Ghidra C loses information (r13, 16-bit vs 32-bit loads, the
//! `FUN_80004250` return value) the raw `sys/main.dol` instruction stream is
//! cited inline as `// observed:`.

use crate::boot::init_tables;
use crate::platform;

// ---------------------------------------------------------------------------
// Register bases established by `init_registers` (0x800041C0).
// ---------------------------------------------------------------------------

/// Initial stack pointer: `lis r1,0x8043 / ori r1,r1,0xFC40` at `0x80004234`.
///
/// `0x8043FC40` is above the DOL image end (`0x8042FC3C`) and below the
/// exception handlers that the OS installs later.
pub const STACK_TOP: usize = 0x8043_FC40;

/// Small-data area 2 base (PPC `r2`): `lis r2,0x8043 / ori r2,r2,0x3080`
/// at `0x8000423C`.
pub const SDA2_BASE: usize = 0x8043_3080;

/// Small-data area 1 base (PPC `r13`): `lis r13,0x8042 / ori r13,r13,0xF680`
/// at `0x80004244`.
///
/// This is the base every `r13 - N` access in the shipped code uses. The ELF
/// import does not model `r13`, which is why Ghidra prints `unaff_r13` for
/// `0x80004030` / `0x80004040`; the value is recovered from the `init_registers`
/// instruction stream, not from a symbol table.
pub const SDA1_BASE: usize = 0x8042_F680;

/// Byte flag addressed as `r13 - 0x4F48` by [`fn_80004030`] / [`fn_80004040`].
///
/// `0x4F48` is the magnitude of the signed 16-bit displacement `0xB0B8`
/// (`stb r0,-0x4F48(r13)` encodes `0xB0B8`). With [`SDA1_BASE`] this is
/// `0x8042A738`.
pub const G_BOOT_FLAG: usize = SDA1_BASE - 0x4F48;

// ---------------------------------------------------------------------------
// Low-memory globals used by `entry`.
// ---------------------------------------------------------------------------

/// Low-memory word cleared before the boot-type dispatch (`stw r0,0x44(0x8000)`).
///
/// `// unknown:` its consumer is outside this lane. It lives in the first page
/// of MEM1, next to the loader's boot-info pointers.
const G_LOW_WORD_44: usize = 0x8000_0044;

/// Pointer to the boot-info block (BI2, the second boot-information block the
/// loader passes in memory).
/// `// inferred:` from the offsets read: `+0x8` is a byte offset to a pointer
/// list and `+0xC` is a small integer boot type.
const G_BOOT_INFO: usize = 0x8000_00F4;

/// Word that receives the 32-byte-aligned module-list pointer; when non-zero
/// it is also an "already relocated" flag for the boot-type dispatch.
const G_MODULE_TABLE: usize = 0x8000_0034;

/// Second copy of the aligned module-list pointer (`stw r7,0x3110(0x8000)`).
const G_MODULE_TABLE_2: usize = 0x8000_3110;

/// 32-bit boot-type value used when [`G_BOOT_INFO`] is null.
const G_BOOT_TYPE: usize = 0x8000_30E8;

/// 16-bit boot flags checked after the OS init calls. Bit 15 and the low 15
/// bits (`& 0x7FFF`) are tested separately in the original.
const G_BOOT_FLAGS_16: usize = 0x8000_30E6;

/// 16-bit platform/debug flags tested against `0x0EEF` by [`fn_80004000`].
const G_PLATFORM_FLAGS_16: usize = 0x8000_30E4;

// ---------------------------------------------------------------------------
// Call targets outside this lane.
// ---------------------------------------------------------------------------

/// `__init_registers` is the first call; `0x80004300` enables `MSR[FP]`.
/// The three `0x801Axxxx` calls below are unclassified in this lane.
const FN_801B_B930: usize = 0x801B_B930;
const FN_801A_A800: usize = 0x801A_A800;
const FN_801B_8970: usize = 0x801B_8970;
const FN_801B_89E0: usize = 0x801B_89E0;

/// MetroTRK boot paths, translated in [`crate::engine::main_loop`] as
/// `fn_802e9300(hardware_id)` / `fn_802e9398()`. Called here by address to
/// keep this lane free of cross-module ordering assumptions.
const FN_802E_9300: usize = 0x802E_9300;
const FN_802E_9398: usize = 0x802E_9398;

/// `OSResetSystem`-era stub: prints "OSResetSystem() is obsoleted ..." through
/// `0x801AD4E0` (`s_OSReset_c_80333af8`, line `0x4A1`).
///
/// Ghidra: `801b3cc0_FUN_801b3cc0.c`
const FN_801B_3CC0: usize = 0x801B_3CC0;

/// Thread starter that never returns; see [`crate::engine::startup`]
/// (`start_main_thread`). `// inferred:` from its call site at `0x800041B4`.
const FN_800C_9F90: usize = 0x800C_9F90;

// ---------------------------------------------------------------------------
// Functions.
// ---------------------------------------------------------------------------

/// DOL entry point (`__start`): sets up the CRT, copies the boot tables,
/// dispatches on the boot type, relocates the module list and hands control to
/// the main thread.
///
/// # Original
/// - Ghidra: `entry` at `0x80004050` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004050_entry.c`
///
/// # Arguments
/// None. The loader's register state is not used.
///
/// # Returns
/// Never, in the original: the last instruction is `b 0x801B89E0`, a tail
/// branch into the module-init/OS-exit path. The Rust function returns only
/// because the crate is not itself the boot image.
///
/// # Side effects
/// * [`init_registers`] sets the CRT register bases (documented, not executed).
/// * [`fn_80004300`] enables the FPU and runs three init routines.
/// * [`init_tables::init_tables`] copies the linker tables at `0x80006618` and
///   zeroes the BSS pairs at `0x8000669C`.
/// * clears `0x80000044`, may write `0x80000034` / `0x80003110` (module table).
/// * writes boot-type-dependent state: the MetroTRK path or [`G_BOOT_FLAG`].
/// * calls `0x801BB930`, `0x801AA800`, `0x801B8970`, `0x800C9F90`, `0x801B89E0`.
///
/// # Safety
/// Must run on the real target with MEM1 valid and with the linker tables
/// present at their link-time addresses. Calling it on a host would execute
/// PowerPC code through [`platform::call0`].
///
/// # Confidence
/// high - control flow was checked against the DOL instruction stream. The
/// only approximations are the stack sentinel (Rust owns the frame) and the
/// dead `r3`/`r4` arguments of the MetroTRK tail call (see [`trace_boot`]).
pub unsafe fn entry() {
    init_registers();
    fn_80004300();

    // observed: 0x80004058 `li r0,-1`, 0x8000405C `stwu r1,-8(r1)`,
    // 0x80004060 `stw r0,4(r1)`, 0x80004064 `stw r0,0(r1)`.
    // This is the CodeWarrior stack sentinel (0xFFFFFFFF in the two words
    // below the caller's SP) consumed by the MetroTRK debugger.
    // unknown: Rust owns the stack frame here, so the sentinel is not written.

    init_tables::init_tables();

    // observed: 0x8000406C..0x80004078 zero the word at 0x80000044.
    platform::write_u32(G_LOW_WORD_44, 0);

    // --- boot-type dispatch (0x8000407C..0x800040F0) ----------------------
    let boot_info = platform::read_u32(G_BOOT_INFO);
    let mut boot_type: Option<u32> = None;
    if boot_info != 0 {
        // observed: `lwz r7,0xC(r6)` - boot type lives in the boot-info block.
        boot_type = Some(platform::read_u32(boot_info as usize + 0x0C));
    } else if platform::read_u32(G_MODULE_TABLE) != 0 {
        // observed: only reached when the module table was already relocated;
        // the type then comes from the low-memory copy at 0x800030E8.
        boot_type = Some(platform::read_u32(G_BOOT_TYPE));
    }

    if let Some(kind) = boot_type {
        // observed: the 2/3 arms end in `mtlr`/`blr` (tail branch) into
        // 0x802E9300, which does not return on hardware. The reconstruction
        // falls through as Ghidra's C does, so the remaining boot steps are
        // still visible in source form.
        match kind {
            // observed: 0x800040BC `cmpwi r7,2`; `li r5,0`; branch to the tail call.
            2 => trace_boot(0),
            // observed: 0x800040C4 `cmpwi r7,3`; `li r5,1` before the branch.
            3 => trace_boot(1),
            // observed: 0x800040D0 `cmpwi r7,4`; `li r5,2`; `bl 0x80004030`.
            // The callee ignores its arguments and only sets the boot flag byte.
            4 => fn_80004030(),
            _ => {}
        }
    }

    // --- module-list relocation (0x800040F4..0x8000416C) -------------------
    //
    // The boot-info block holds `+0x8` = byte offset (relative to the block
    // base) of a table whose first word is the entry count and whose following
    // words are pointers to be relocated by the block base.
    let mut module_count: u32 = 0; // r14
    let mut module_table: usize = 0; // r15 (first relocated pointer)
    let boot_info = platform::read_u32(G_BOOT_INFO); // observed: re-loaded
    if boot_info != 0 {
        let list_offset = platform::read_u32(boot_info as usize + 0x08);
        if list_offset != 0 {
            let list = (boot_info as usize).wrapping_add(list_offset as usize);
            let count = platform::read_u32(list);
            if count != 0 {
                module_count = count;
                module_table = list + 4;
                let mut p = list; // r6 walks the pointer words
                let mut left = count;
                while left != 0 {
                    p += 4;
                    // observed: `lwz r7,0(r6); add r7,r7,r5; stw r7,0(r6)`
                    // where r5 is the boot-info base.
                    let relocated = platform::read_u32(p).wrapping_add(boot_info);
                    platform::write_u32(p, relocated);
                    left -= 1;
                }
                // observed: `rlwinm r7,r15,0,0,26` = clear the low 5 bits
                // (32-byte alignment) before publishing the list.
                let aligned = module_table & 0xFFFF_FFE0;
                platform::write_u32(G_MODULE_TABLE, aligned as u32);
                platform::write_u32(G_MODULE_TABLE_2, aligned as u32);
            }
        }
    }

    // --- OS / runtime bring-up (0x8000416C..0x800041B8) --------------------
    platform::call0(FN_801B_B930);
    platform::call0(FN_801A_A800);

    // observed: 0x80004174 `lhz r3,0x30E6(0x8000)`; bit 15 clear or
    // `(& 0x7FFF) == 1` both enter the 0x80004000 call. Ghidra models this
    // halfword as a 32-bit DAT; the load is 16-bit.
    let boot_flags = platform::read_u16(G_BOOT_FLAGS_16) as u32;
    if (boot_flags & 0x8000) == 0 || (boot_flags & 0x7FFF) == 1 {
        fn_80004000();
    }

    // observed: 0x80004198 `bl 0x80004040`; `cmpwi r3,1`; conditional
    // `bl 0x802E9398` (MetroTRK, hardware id 2).
    if fn_80004040() == 1 {
        platform::call0(FN_802E_9398);
    }

    platform::call0(FN_801B_8970);
    // observed: 0x800041AC..0x800041B4 `mr r3,r14; mr r4,r15; bl 0x800C9F90`.
    // r14 = module count, r15 = aligned module table (0/0 when absent).
    platform::call2(FN_800C_9F90, module_count, module_table as u32);

    // observed: 0x800041B8 `b 0x801B89E0` - a tail branch, not a call. The
    // target walks the table at 0x802EDCC0 and never returns to `entry`.
    platform::call0(FN_801B_89E0);
}

/// Tail-branches into the MetroTRK boot path (`0x802E9300`) with a hardware id.
///
/// # Original
/// - Ghidra: part of `entry` at `0x80004050` (block `0x800040E4..0x800040F0`)
/// - Evidence: `work/ghidra/main-gb/export/80004050_entry.c`
///
/// # Arguments
/// * `hardware_id` - `r5`: `0` for boot type 2, `1` for boot type 3. Ghidra's
///   C shows `FUN_802e9300(uVar1,(int)uVar8,uVar3)`; the callee's only consumed
///   argument is this third one (`0x802E96E4(hardware_id)`).
///
/// # Returns
/// Never in the original (the block ends in `mtlr` + `blr`); the callee resets
/// the console or enters the TRK loop.
///
/// # Side effects
/// Whatever the MetroTRK boot path does; it saves the full GPR/SPR context at
/// `0x80427240`/`0x804273E8`.
///
/// # Safety
/// Same constraints as [`entry`].
///
/// # Confidence
/// high for the target and `r5`; medium for `r3`/`r4` (documented below).
unsafe fn trace_boot(hardware_id: u32) {
    // observed: `lis r6,0x802F / addi r6,r6,-0x6D00` assembles
    // 0x802F0000 - 0x6D00 = 0x802E9300; `mtlr r6; blr`.
    // unknown: r3/r4 at this point are dead leftovers from `init_tables`
    // (that function returns void; Ghidra's 64-bit "return value" is an
    // artifact of the decompiler pairing r3:r4). They are forwarded as 0.
    platform::call3(FN_802E_9300, 0, 0, hardware_id);
}

/// Zeroes the general-purpose registers (GPRs) and installs the three static
/// bases.
///
/// # Original
/// - Ghidra: `FUN_800041c0` at `0x800041C0` (144 bytes); the exported C is an
///   empty stub because the body only writes registers.
/// - Evidence: `work/ghidra/main-gb/export/800041c0_FUN_800041c0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing (leaf function, plain `blr`).
///
/// # Side effects
/// Register-only, no memory writes:
/// * `r0`, `r3`..`r12`, `r14`..`r31` = 0
/// * `r1` = [`STACK_TOP`], `r2` = [`SDA2_BASE`], `r13` = [`SDA1_BASE`]
///
/// # Safety
/// No memory access.
///
/// # Confidence
/// high for the values (read directly from `0x80004234..0x8000424C`);
/// `// partial:` the register assignment cannot be executed from Rust without
/// corrupting the compiler's reserved registers, so the function body is empty
/// and the constants above are the authoritative record.
pub unsafe fn init_registers() {
    // observed: `li r0,0` / `li r3,0` .. `li r31,0` (r13 skipped),
    // then the three `lis`/`ori` pairs listed on the constants.
    //
    // partial: the register assignment is not executed; STACK_TOP,
    // SDA2_BASE and SDA1_BASE above are the authoritative record.
    // unknown: no portable Rust equivalent; see the doc comment.
}

/// Enables the floating-point unit and runs three boot-time init routines.
///
/// # Original
/// - Ghidra: `FUN_80004300` at `0x80004300` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004300_FUN_80004300.c`
///
/// # Arguments
/// None. The three callees receive whatever argument registers are live; the
/// caller sets nothing.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * sets `MSR[FP]` (`0x2000`) through `mfmsr`/`mtmsr`
/// * calls `0x801AB150`, `0x801A9D40`, `0x801AC990` (`// unknown:` roles)
///
/// # Safety
/// `mtmsr` is privileged; only valid on the target.
///
/// # Confidence
/// high for the translation; the three callees are outside this lane.
pub unsafe fn fn_80004300() {
    enable_fpu();
    platform::call0(0x801A_B150);
    platform::call0(0x801A_9D40);
    platform::call0(0x801A_C990);
}

/// Sets `MSR[FP]` (bit `0x2000`) so floating-point instructions become legal.
///
/// # Original
/// - observed: `0x80004300 mfmsr r0`; `0x80004304 ori r0,r0,0x2000`;
///   `0x80004308 mtmsr r0`
/// - Evidence: `work/ghidra/main-gb/export/80004300_FUN_80004300.c` (the
///   decompiler drops both privileged instructions)
///
/// # Arguments
/// None; both instructions work on the machine-state register.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the machine-state register on PowerPC (sets `MSR[FP]`, bit `0x2000`).
///
/// # Safety
/// Privileged; must not run in user mode.
///
/// # Confidence
/// high on PowerPC; the non-PowerPC build is a documented no-op so the crate
/// can still be `cargo check`ed on the host.
#[cfg(target_arch = "powerpc")]
unsafe fn enable_fpu() {
    // `MSR[FP]` - floating-point available.
    const MSR_FP: u32 = 0x2000;
    let mut msr: u32;
    core::arch::asm!("mfmsr {0}", out(reg) msr);
    msr |= MSR_FP;
    core::arch::asm!("mtmsr {0}", in(reg) msr);
}

/// Host-only stub for the PowerPC [`enable_fpu`]; intentionally empty.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn enable_fpu() {
    // unknown: MSR does not exist on the host; nothing is emulated.
    //
    // This variant exists only so the crate type-checks on the development
    // host; it has no target behaviour.
}

/// If the platform flags are fully set, calls the obsolete `OSResetSystem`
/// stub; otherwise returns immediately.
///
/// # Original
/// - Ghidra: `FUN_80004000` at `0x80004000` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004000_FUN_80004000.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing; the call at the end is a tail branch.
///
/// # Side effects
/// `0x801B3CC0` reports "OSResetSystem() is obsoleted ..." (`0x801AD4E0`,
/// file `s_OSReset_c_80333af8`, line `0x4A1`) and does not return.
///
/// # Safety
/// Same constraints as [`entry`].
///
/// # Confidence
/// high. `// observed:` the flag load is `lhz` (16-bit) although Ghidra models
/// `_DAT_800030e4` as 32-bit; the mask is `0x0EEF`.
/// `// unknown:` what the four `0x0EEF` flag bits stand for; only the "all set"
/// comparison is visible here.
pub unsafe fn fn_80004000() {
    let platform_flags = platform::read_u16(G_PLATFORM_FLAGS_16) as u32;
    if (platform_flags & 0x0EEF) != 0x0EEF {
        return;
    }
    // observed: `li r3,0; li r4,0; li r5,0; b 0x801B3CC0`.
    platform::call3(FN_801B_3CC0, 0, 0, 0);
}

/// Sets the one-byte boot flag at [`G_BOOT_FLAG`] (originally `r13 - 0x4F48`).
///
/// # Original
/// - Ghidra: `FUN_80004030` at `0x80004030` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004030_FUN_80004030.c`
///
/// # Arguments
/// None (the call site passes dead registers; the body ignores them).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `*(u8*)0x8042A738 = 1`.
///
/// # Safety
/// Requires [`init_registers`] to have established r13; the absolute address is
/// derived from [`SDA1_BASE`], not from a symbol.
///
/// # Confidence
/// high for the store; medium for the flag's meaning (`// unknown:`).
pub unsafe fn fn_80004030() {
    platform::write_u8(G_BOOT_FLAG, 1);
}

/// Reads the one-byte boot flag at [`G_BOOT_FLAG`].
///
/// # Original
/// - Ghidra: `FUN_80004040` at `0x80004040` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004040_FUN_80004040.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The byte value (0 or 1 in practice).
///
/// # Side effects
/// None.
///
/// # Safety
/// Same r13 requirement as [`fn_80004030`].
///
/// # Confidence
/// high.
pub unsafe fn fn_80004040() -> u8 {
    platform::read_u8(G_BOOT_FLAG)
}
