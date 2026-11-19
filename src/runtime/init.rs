//! OS initialisation: the single-shot startup sequence and the CPU/cache
//! bring-up steps the DOL entry point performs before the game runs.
//!
//! **What lives here** (all called from `entry` at `0x80004050` unless noted):
//! - `0x801bb930` - writes a small hand-off record into low memory.
//! - `0x801aa800` - the big OS init: boot-info capture, arena bounds,
//!   exception handlers, IPC, thread state, error screens, final interrupts-on.
//! - `0x801ab150` - enables paired-single mode and clears the GQRs
//!   (reached through `__init_hardware` at `0x80004300`).
//! - `0x801a9d40` - enables the FPU and fills the floating-point registers.
//! - `0x801ac990` - enables the L1/L2 caches and installs the machine-check
//!   handler.
//! - `0x801ae0a0` - snapshots the 0x1C-byte boot/loader record (or clears the
//!   destination's first word when none is published).
//!
//! **Why it exists.** Stage 2 of `PLAN.md`: "boot and CRT: entry -> `__start`,
//! stack/BSS, `OSInit`, ctors, `main()`". The `boot` lane owns the CRT; this
//! module owns everything the CRT calls into the OS/runtime before the game's
//! own startup.
//!
//! **Dependencies.** `crate::platform` for volatile memory access and indirect
//! calls; `super::os` for the CPU register and cache primitives and for the
//! recovered `r13` small-data base. Callees that are large or belong to another
//! lane are called by absolute address and named in comments.
//!
//! **Start here.** `os_init()`. Everything else in this file is either a step
//! of it or is called immediately around it by the entry point.

use crate::boot::mem::memcpy;
use crate::platform::{call0, call1, call2, call3, read_u8, read_u32, write_u8, write_u32};

use super::os;
use super::os::{call0_ret, sda, sda2};

// ---------------------------------------------------------------------------
// r13-relative globals used by this module
// ---------------------------------------------------------------------------
//
// `sda(offset)` is `r13 + offset`; see `os::SDA_BASE` for how r13 was
// recovered. Names are descriptive only where the surrounding code proves the
// role; otherwise the offset is part of the name.

/// Non-zero once `OSInit` has run. Tested and set by `0x801aa800`.
const SLOT_OS_INITIALIZED: isize = -0x503c;

/// First word of the 64-bit "time at OS init" stamp (`0x801b5fb0` result).
const SLOT_BOOT_TIME: isize = -0x5008;

/// Holds the boot-info pointer: set to `0x80000000` here, and read back as a
/// structure whose `+0x30` / `+0x34` words override the arena bounds.
const SLOT_BOOT_INFO: isize = -0x5010;

/// Holds a pointer into the boot parameters; `[0]` is the boot type
/// (`< 2` selects the alternate stack-top default).
const SLOT_BOOT_PARAMS: isize = -0x5014;

/// Two-word scratch that `SLOT_BOOT_PARAMS` can point at (boot type + flag).
const SLOT_BOOT_PARAMS_LOCAL: isize = -0x5018;

/// Second word of the boot type / apploader descriptor.
const SLOT_BOOT_AUX: isize = -0x4c18;

/// OS init stage counter: 0 while capturing boot info, 1 once captured.
const SLOT_INIT_STAGE: isize = -0x6078;

/// Boot-mode flags. `0` = normal disc boot; non-zero selects the reduced
/// initialisation path (IPL/debug style boot). Written outside this module.
const SLOT_FLAG_5048: isize = -0x5048;
const SLOT_FLAG_5044: isize = -0x5044;
const SLOT_FLAG_4FD4: isize = -0x4fd4;
const SLOT_FLAG_5040: isize = -0x5040;

/// MEM1 arena bounds (type-1 memory, starts at `0x80000000`).
/// Low/high order is inferred from the defaults: low = end of the loaded image,
/// high = top of MEM1; the kernel banner at `0x801aa550` prints the pair.
const SLOT_MEM1_ARENA_LO: isize = -0x6180;
const SLOT_MEM1_ARENA_HI: isize = -0x4fe8;

/// MEM2 arena bounds (type-2 memory, starts at `0x90000000`).
const SLOT_MEM2_ARENA_LO: isize = -0x617c;
const SLOT_MEM2_ARENA_HI: isize = -0x4fe4;

/// Default MEM1 arena low bound when boot info provides none: `0x80441c40`,
/// i.e. the initial stack pointer (`0x8043fc40`) plus `0x2000`.
const DEFAULT_MEM1_ARENA_LO: u32 = 0x8044_1c40;

/// Alternate MEM1 low bound used when the boot parameters say "no apploader
/// relocation": exactly the initial stack pointer set by `__init_registers`.
const STACK_TOP_MEM1_ARENA_LO: u32 = 0x8043_fc40;

/// Default MEM1 arena high bound: `0x81800000` (24 MiB of MEM1) minus 1 MiB,
/// leaving room for the SDK's reserved block.
const DEFAULT_MEM1_ARENA_HI: u32 = 0x8170_0000;

/// Default MEM2 arena low bound: base of MEM2 (`0x90000000`) plus 2 KiB.
const DEFAULT_MEM2_ARENA_LO: u32 = 0x9000_0800;

/// Absolute slot holding the pointer to the current 0x1C-byte boot/loader
/// record. It lives in BSS, so it is 0 until `0x801ae840` publishes a record.
const SLOT_BOOT_RECORD: u32 = 0x8000_30f0;

/// Size of the boot/loader record: seven words, four of which are caller
/// parameters plus a tick stamp and a size.
const BOOT_RECORD_LEN: u32 = 0x1c;

/// Lowest value `SLOT_BOOT_RECORD` must have to be usable as a pointer: the
/// record is always a MEM1 heap allocation, i.e. at or above `0x80000000`.
const BOOT_RECORD_MIN_PTR: u32 = 0x8000_0000;

// ---------------------------------------------------------------------------
// Local helper: print through the SDK formatter
// ---------------------------------------------------------------------------

/// Formats and emits one debug line through `0x8015f730`.
///
/// # Original
/// - Ghidra: `FUN_8015f730` - not translated (belongs to the boot/CRT lane)
/// - Evidence: used with a format string plus zero or more arguments in every
///   diagnostic path of `0x801aa800` and in `0x801aa550`.
///
/// # Arguments
/// - `format`: virtual address of a NUL-terminated format string
/// - `args`: up to three integer arguments consumed by the format
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Console/debug output. In this release build `os::os_report` (the other
/// reporter) is compiled out, so this is the path that actually prints.
///
/// # Safety
/// `format` must point at a valid string and its conversions must match the
/// number of entries in `args`.
///
/// # Confidence
/// high for the calling convention; the formatter's internals are out of scope.
unsafe fn sdk_printf(format: u32, args: &[u32]) {
    match args.len() {
        0 => {
            call1(0x8015_f730, format);
        }
        1 => {
            call2(0x8015_f730, format, args[0]);
        }
        _ => {
            call3(0x8015_f730, format, args[0], args[1]);
        }
    }
}

/// Raises an SDK panic/assert screen and never returns.
///
/// # Original
/// - Ghidra: `FUN_801ad4e0` at `0x801ad4e0` (300 bytes) - not translated
/// - Evidence: `801ad4e0_FUN_801ad4e0.c` prints "in %s on line %d" and walks
///   the thread back-chain before halting; every call site is a fatal path.
///
/// # Arguments
/// - `file`: virtual address of the source-file string
/// - `line`: source line number
/// - `message`: virtual address of the message string
///
/// # Returns
/// Nothing (it never returns on target).
///
/// # Safety
/// Must only be called on unrecoverable paths (it does not return on target).
///
/// # Confidence
/// high for the message strings and the terminal loop, but this body is only a
/// forwarder to `0x801ad4e0`: the 300-byte variadic panic path (thread back-chain
/// walk, "in %s on line %d" formatting, halt) is not translated, so the row stays
/// `stub`.
unsafe fn sdk_panic(file: u32, line: u32, message: u32) {
    // adapter: three-argument forwarder to FUN_801AD4E0; the 300-byte panic
    // path itself is not translated (ledger row stays `stub`).
    call3(0x801a_d4e0, file, line, message);
}

// ---------------------------------------------------------------------------
// 0x801bb930 - low-memory hand-off record
// ---------------------------------------------------------------------------

/// Writes a three-word record into the OS low-memory area before `OSInit`.
///
/// The record lives at `0x80000040`; a pointer to it is published in the
/// `r13-0x4ec8` slot. Word `+0` is left alone, word `+4` is cleared by the
/// entry point just before this call, word `+8` receives the *physical* form
/// (`0x001bb9b0`) of the code address `0x801bb9b0`, and `r13-0x4ec4` is set
/// to 1 to mark the record valid.
///
/// # Original
/// - Ghidra: `FUN_801bb930` at `0x801bb930` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801bb930_FUN_801bb930.c`; instruction
///   bytes: `lis 0x8000; addi 0x40` → `stw r13-0x4ec8`; `lis 0x801c;
///   addi -18000; addis 0x8000` (clears bit 31) → `stw 0x80000048`;
///   `li 1` → `stw r13-0x4ec4`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `*(u32*)0x80000040` becomes the record pointer's target
/// - `*(u32*)0x80000048 = 0x001bb9b0`
/// - `*(u32*)(r13-0x4ec4) = 1`
///
/// # Unknown
/// The record's purpose. Evidence that constrains it:
/// - `0x801bb9c0` reads `record[1] & (1 << index)` - a bitmask of "reasons".
/// - `0x801bb9b0` is a four-instruction trampoline
///   (`mfmsr; ori 0x30; mtmsr; b 0x801bb960`) that switches on address
///   translation (`MSR[IR]`/`MSR[DR]`, mask `0x30`) and jumps into code that
///   scans `0x80340050`-style boot strings.
/// - Storing the *physical* address means the record is meant to be usable
///   while translation is off, i.e. a real-mode entry point.
///
/// # Safety
/// Writes fixed low-memory addresses; must run once, before anything reads `0x80000040`.
///
/// # Confidence
/// high for the stores; low for any naming beyond "hand-off record".
pub unsafe fn fn_801bb930() {
    write_u32(sda(-0x4ec8), 0x8000_0040);
    write_u32(0x8000_0048, 0x001b_b9b0);
    write_u32(sda(-0x4ec4), 1);
}

// ---------------------------------------------------------------------------
// 0x801ae0a0 - boot/loader record snapshot
// ---------------------------------------------------------------------------

/// Copies the current 0x1C-byte boot/loader record to `dest`, or zeroes only
/// the first word of `dest` when no record has been published yet.
///
/// # Original
/// - Ghidra: `FUN_801ae0a0` at `0x801ae0a0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801ae0a0_FUN_801ae0a0.c`; instruction
///   bytes `3ca08000` (`lis r5,0x8000`), `808530f0` (`lwz r4,0x30f0(r5)`),
///   `7c042840` (`cmplw cr0,r4,r5`), `4180000c` (`blt +0xc`), `38a0001c`
///   (`li r5,0x1c`), `4be562b0` (`bl 0x80004364`), `38000000` (`li r0,0`),
///   `90030000` (`stw r0,0(r3)`), `4e800020` (`blr`).
/// - Caller: `os_init` at `0x801aa800`, with `dest = 0x8038e520`.
///
/// # Arguments
/// - `dest`: address of a writable 0x1C-byte buffer
///
/// # Returns
/// Nothing. `// observed:` `r3` is left holding `dest` (the `memcpy` result on
/// the copy path, its own value on the zero path) and the caller drops it.
///
/// # Side effects
/// - record present (`SLOT_BOOT_RECORD >= 0x80000000`): `dest[0..0x1C]` becomes
///   a byte-for-byte copy of the record
/// - record absent: only `dest[0]` is cleared; `dest[1..0x1C]` keeps whatever
///   the caller left there
///
/// # Unknown
/// What the record means. Evidence that constrains it:
/// - `0x801ae840` builds it: `0x801abf20(0x1c, 1)` allocates it, word 0 is
///   forced to 1, word 1 receives a 64-bit tick value, word 2 accumulates the
///   loaded size in words (`+0x14 + 0x20 >> 2` per section), words 3-5 are the
///   caller's parameters, word 6 is a per-section handle.
/// - The same routine republishes the slot after each loader pass and just
///   before `0x801b1140` / `0x801ae050`, so the record tracks the most recent
///   apploader/relocation run. `inferred:` it is the "currently loaded image"
///   descriptor that the OS layer hands to late startup code.
///
/// # Safety
/// `dest` must point at 0x1C writable bytes when a record is published. The
/// zero path writes a single word.
///
/// # Confidence
/// high for the branch shape, the length and the destination handling; low for
/// the name and for the record's semantics.
pub unsafe fn fn_801ae0a0(dest: u32) {
    let record = read_u32(SLOT_BOOT_RECORD as usize);
    // observed: cmplw/blt - only a pointer-shaped value takes the copy path.
    if record >= BOOT_RECORD_MIN_PTR {
        memcpy(dest as usize, record as usize, BOOT_RECORD_LEN);
    } else {
        // observed: li r0,0; stw r0,0(r3) - first word only.
        write_u32(dest as usize, 0);
    }
}

// ---------------------------------------------------------------------------
// 0x801aa800 - OSInit
// ---------------------------------------------------------------------------

/// Initialises the OS and the runtime once per boot.
///
/// Runs only if the `r13-0x503c` flag is clear, sets it immediately, and then:
/// captures the boot time and boot info, seeds the memory arenas, clears the
/// performance counters, installs exception handlers, initialises IPC/DSP/
/// thread state and the caches, prints the kernel banner, optionally boots the
/// apploader, and finally enables interrupts.
///
/// # Original
/// - Ghidra: `FUN_801aa800` at `0x801aa800` (1364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801aa800_FUN_801aa800.c`
/// - Caller: `entry` at `0x80004050`, immediately after `0x801bb930`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing; on a normal boot it falls off the end and the entry point
/// continues to `0x800c9f90` (the game's startup).
///
/// # Side effects
/// - `r13-0x503c`: 0 → 1 (single-shot latch)
/// - `r13-0x5008`: 64-bit boot timestamp
/// - `0x800030e8` / `0x800030e9`: boot type bytes copied out of the boot info
/// - `r13-0x5010` / `-0x5014` / `-0x4c18` / `-0x6078`: boot-info pointers/stage
/// - `r13-0x6180` / `-0x4fe8` / `-0x617c` / `-0x4fe4`: arena bounds
/// - performance counters `MMCR0/1`, `PMC1..4`: zeroed
/// - `HID0` bit `0x200`: set; SPR 1011: `0x83900000`
/// - `HID2`: bit `0x40000000` cleared
/// - `0x800030e6`: forced to `0xffff9000` when `r13-0x5040` is set
/// - exception handlers, IPC, DSP, thread state, caches: see the call map below
/// - `MSR[EE]`: enabled near the end
///
/// # Safety
/// Must be called exactly once, early, with the MMU/caches already set up by
/// `__init_hardware` (`0x80004300`). It programs hardware registers and calls
/// firmware through memory-mapped I/O.
///
/// # Confidence
/// high for the sequence and the memory effects; medium for individual
/// subsystem names, which are marked `inferred:` inline.
pub unsafe fn os_init() {
    // ---------------------------------------------------------------------
    // 1. Single-shot latch, boot timestamp, interrupt mask.
    // ---------------------------------------------------------------------
    if read_u32(sda(SLOT_OS_INITIALIZED)) != 0 {
        return;
    }
    write_u32(sda(SLOT_OS_INITIALIZED), 1);

    // observed: `FUN_801b5fb0` = time base + 64-bit offset at 0x800030d8; the
    // result is stored as one 8-byte write at r13-0x5008.
    let boot_time = os::get_time();
    write_u32(sda(SLOT_BOOT_TIME), (boot_time >> 32) as u32);
    write_u32(sda(SLOT_BOOT_TIME + 4), boot_time as u32);

    // observed: `FUN_801b1140` = OSDisableInterrupts; the returned mask is
    // dropped here (the final `enable_interrupts` re-enables unconditionally).
    os::disable_interrupts();

    // observed: 36-byte helper that copies the 0x1C-byte boot/loader record
    // from 0x800030f0 into 0x8038e520 when the slot looks like a pointer
    // (> 0x7fffffff), otherwise zeroes just the first word. inferred: snapshot
    // of the boot/version record (see `fn_801ae0a0`).
    fn_801ae0a0(0x8038_e520);

    // observed: six `mtspr` leaves clear MMCR0/MMCR1/PMC1..PMC4.
    os::clear_performance_monitor();

    // observed: SPR 1011 = 0x83900000 (top bit forced on), then HID0 |= 0x200,
    // then 0x801a9ce0, a single `mtfsf` that installs an FPSCR image from f29.
    // unknown: the FPSCR image is implicit (no argument in r3..r10).
    os::set_spr_1011(0x8390_0000);
    os::or_hid0_0x200();
    call0(0x801a_9ce0);

    // ---------------------------------------------------------------------
    // 2. Boot information.
    // ---------------------------------------------------------------------
    // observed: 0x80000000 is the address the boot info is aliased at; the
    // fields read below are +0x30 (MEM1 arena low override) and +0x34 (MEM1
    // arena high override).
    write_u32(sda(SLOT_BOOT_INFO), 0x8000_0000);
    write_u32(sda(SLOT_BOOT_PARAMS), 0);
    write_u32(sda(SLOT_INIT_STAGE), 0);

    let boot_info = read_u32(0x8000_00f4); // boot info block, 0 when absent
    if boot_info == 0 {
        // observed: 0x80000034 is set by the CRT when it relocated a segment.
        if read_u32(0x8000_0034) != 0 {
            write_u32(
                sda(SLOT_BOOT_PARAMS_LOCAL),
                read_u8(0x8000_30e8) as u32,
            );
            write_u32(sda(SLOT_BOOT_PARAMS), sda(SLOT_BOOT_PARAMS_LOCAL) as u32);
            write_u32(sda(SLOT_BOOT_AUX), read_u8(0x8000_30e9) as u32);
        }
    } else {
        let base = boot_info as usize;
        // observed: +0x0c is the boot type word. inferred: the 16-byte header
        // is the "boot parameters" block later handed to the apploader.
        write_u32(sda(SLOT_BOOT_PARAMS), (base + 0x0c) as u32);
        write_u32(sda(SLOT_BOOT_AUX), read_u32(base + 0x24));
        write_u8(0x8000_30e8, read_u32(base + 0x0c) as u8);
        write_u8(0x8000_30e9, read_u32(sda(SLOT_BOOT_AUX)) as u8);
    }
    write_u32(sda(SLOT_INIT_STAGE), 1);

    // ---------------------------------------------------------------------
    // 3. Arena bounds (MEM1 first, then MEM2).
    // ---------------------------------------------------------------------
    // observed: 0x8000310c / 0x80003110 override the MEM1 low/high bounds and
    // 0x80003124 / 0x80003128 the MEM2 ones; the setters write r13 slots:
    //   0x801abef0 -> r13-0x6180 (MEM1 lo)   0x801abec0 -> r13-0x4fe8 (MEM1 hi)
    //   0x801abf00 -> r13-0x617c (MEM2 lo)   0x801abed0 -> r13-0x4fe4 (MEM2 hi)
    let mut value = read_u32(0x8000_310c);
    if value == 0 {
        let info = read_u32(sda(SLOT_BOOT_INFO)) as usize;
        value = read_u32(info + 0x30);
        if value == 0 {
            value = DEFAULT_MEM1_ARENA_LO;
        }
        // observed: with no override, a boot type below 2 narrows the arena to
        // the initial stack pointer (0x8043fc40).
        if read_u32(info + 0x30) == 0
            && read_u32(sda(SLOT_BOOT_PARAMS)) != 0
            && read_u32(read_u32(sda(SLOT_BOOT_PARAMS)) as usize) < 2
        {
            value = STACK_TOP_MEM1_ARENA_LO;
        }
    }
    call1(0x801a_bef0, value);

    value = read_u32(0x8000_3110);
    if value == 0 {
        value = read_u32(read_u32(sda(SLOT_BOOT_INFO)) as usize + 0x34);
        if value == 0 {
            value = DEFAULT_MEM1_ARENA_HI;
        }
    }
    call1(0x801a_bec0, value);

    value = read_u32(0x8000_3124);
    if value != 0 {
        // observed: the MEM2 base is 0x90000000; anything inside the first
        // 2 KiB is bumped past the SDK's reserved block.
        if value > 0x8fff_ffff && value < 0x9000_0800 {
            value = DEFAULT_MEM2_ARENA_LO;
        }
        call1(0x801a_bf00, value);
    }
    value = read_u32(0x8000_3128);
    if value != 0 {
        call1(0x801a_bed0, value);
    }

    // ---------------------------------------------------------------------
    // 4. Subsystem initialisation, in the original order.
    // ---------------------------------------------------------------------
    // observed: copies 0x80003130/0x80003134 into r13-0x6148 / r13-0x4f70
    // (two boot words kept for later queries).
    call0(0x801b_6620);
    // observed: reports "Installing OSDBIntegrator..." then
    // "Exceptions initialized..."; installs the exception vectors and the
    // debug-integrator hook. inferred: __OSInitException.
    call0(0x801a_ad60);
    // observed: copies a 0x1c-byte stub to 0x80000c00, then invalidates and
    // re-enables the instruction cache over 0x100 bytes. inferred: installs the
    // low-memory handler trampolines.
    call0(0x801b_47b0);
    // observed: reserves handler slot 8 of the table at r13-0x5038 and reports
    // through 0x801b32d0 when the slot was already taken.
    call0(0x801a_b280);
    // observed: zeroes 0x800030c8 / 0x800030cc / 0x800030d0 (three boot words).
    call0(0x801b_2370);
    // observed: clears the mask words 0x800000c4/0x800000c8, programs PI
    // (0xcc003004 = 0xf0) and DI (0xcd000034 = 0x40000000), installs handler 4
    // after draining pending bits. inferred: interrupt-controller init.
    call0(0x801b_11e0);
    // observed: reports "FPU unavailable handler installed". inferred:
    // installs exception 0x10 (FPU unavailable).
    call0(0x801a_d360);
    // Cache bring-up, translated in this file; also installs the machine-check
    // handler that decodes locked-cache DMA errors.
    init_cache();
    // observed: programs 0xcd006800-0xcd006834, installs handlers 9..0x10,
    // queries IOS version through 0x801b9f80. inferred: IPC init.
    call0(0x801b_9ba0);
    // observed: SISetSamplingRate(0) - the body uses the TV format from
    // 0xcc00206c to pick the SI sampling table.
    call0(0x801b_aef0);
    // observed: probes the boot device / IPC state and stores the result in
    // 0x8038e924..0x8038e92c. inferred: disc/backup-unit state query.
    call0(0x801b_3e20);
    // observed: initialises the thread/context bookkeeping: _DAT_800000d8 and
    // _DAT_800000e4 point at 0x8038e938, stack bounds 0x8038ec3c = 0x8043fc40
    // and 0x8038ec40 = 0x8042fc3c, and the 0xdeadbabe stack sentinel is
    // written at 0x8042fc3c. inferred: thread/stack init.
    call0(0x801b_4890);
    // observed: copies 0x80 bytes between the arena low bound and 0x81000000,
    // then flushes the copy. inferred: relocation/segment snapshot support.
    call0(0x801a_c180);

    // observed: mfhid2; clear bit 0x40000000 (locked-cache DMA enable).
    let hid2 = os::get_hid2();
    os::set_hid2(hid2 & 0xBFFF_FFFF);

    // observed: one-shot DSP bring-up (0xcc004010 / 0xcc004020), gated by
    // r13-0x5048 and r13-0x4fd4 in the caller.
    if read_u32(sda(SLOT_FLAG_5048)) == 0 {
        call0(0x801b_2d50);
    }

    // observed: prints the kernel banner ("Revolution OS", build date
    // "Feb 27 2009 10:04:29", console type, firmware, memory size and the two
    // arena pairs). The banner function takes no arguments.
    call0(0x801a_a550);

    // observed: format string is the buffer at r13-0x61a4, argument is the
    // word at r13-0x61b8 (filled by the banner/print helpers).
    sdk_printf(sda(-0x61a4) as u32, &[read_u32(sda(-0x61b8))]);

    // observed: when the boot parameters hold a type above 1, call the thunk at
    // 0x802e9874 (its target 0x802e9614 panics with "OSResetSystem obsoleted").
    // unknown: why only non-zero boot types take this path.
    let params = read_u32(sda(SLOT_BOOT_PARAMS));
    if params != 0 && read_u32(params as usize) > 1 {
        call0(0x802e_9874);
    }

    // observed: 0x801aa140 / 0x801aa320 flush and invalidate the free ends of
    // the MEM1 / MEM2 arenas (0x40000-byte granularity) before the heap is
    // handed out. Skipped in the reduced boot mode.
    if read_u32(sda(SLOT_FLAG_5044)) == 0 && read_u32(sda(SLOT_FLAG_4FD4)) == 0 {
        call0(0x801a_a140);
        call0(0x801a_a320);
    }

    // observed: OSEnableInterrupts (MSR[EE] = 1).
    os::enable_interrupts();

    // observed: one-shot 4 KiB allocator + exception 0x1b handler + callbacks;
    // returns a status that the caller ignores. inferred: secondary heap setup.
    call0(0x8022_47d0);

    // ---------------------------------------------------------------------
    // 5. Storage device (STM) and network shutdown registration.
    // ---------------------------------------------------------------------
    if read_u32(sda(SLOT_FLAG_5044)) == 0 && read_u32(sda(SLOT_FLAG_4FD4)) == 0 {
        // observed: opens "/dev/stm/immediate" and "/dev/stm/eventhook".
        call0(0x801b_6820);
        call0(0x801d_b840);
        // observed: polls a state byte until it stops returning 1.
        while call0_ret(0x801d_b8e0) == 1 {}
        // observed: reports "Failed to register network shutdown ...",
        // "Failed to suspend the WiiConnect ...", "Failed to synchronize time
        // with ...". inferred: WiiConnect24 shutdown hooks.
        call0(0x801b_7690);
    }

    // ---------------------------------------------------------------------
    // 6. Production-only error screens and menu return.
    // ---------------------------------------------------------------------
    if read_u32(sda(SLOT_FLAG_5048)) == 0 {
        // observed: 0x81 (-0x7f) in these boot bytes means "not found".
        if read_u8(0x8000_315c) == 0x81 {
            sdk_printf(0x8033_24a8, &[]); // "OS_ERROR : boot program is not found"
            sdk_panic(sda(-0x61a0) as u32, 0x481, 0x8033_24f8);
        }
        if read_u8(0x8000_315d) == 0x81 {
            sdk_printf(0x8033_2510, &[]); // "OS_ERROR : apploader ... is not found"
            sdk_panic(sda(-0x61a0) as u32, 0x493, 0x8033_24f8);
        }

        // observed: 0xc0003140 / 0xc0003144 are the console's build date/time
        // words. The routine packs the four date bytes, then BCD month, day and
        // a 2000-based year, into the eight bytes that follow two argument
        // words on its stack; the words are read from r2-0x4fb8 / r2-0x4fb4.
        let date_word = read_u32(0xC000_3140);
        let time_word = read_u32(0xC000_3144);
        let month = ((time_word >> 16) & 0xf) + ((time_word >> 20) & 0xf) * 10;
        let day = ((time_word >> 8) & 0xf) + ((time_word >> 12) & 0xf) * 10;
        let year = 2000 + (time_word & 0xf) + ((time_word >> 4) & 0xf) * 10;

        // inferred: the two words and the date form one contiguous 16-byte
        // stack record; only the first two words are passed as pointers to
        // 0x801af5d0.
        let mut record = [0u8; 16];
        record[0..4].copy_from_slice(&read_u32(sda2(-0x4fb8)).to_be_bytes());
        record[4..8].copy_from_slice(&read_u32(sda2(-0x4fb4)).to_be_bytes());
        record[8] = (date_word >> 24) as u8;
        record[9] = (date_word >> 16) as u8;
        record[10] = (date_word >> 8) as u8;
        record[11] = date_word as u8;
        record[12] = month as u8;
        record[13] = day as u8;
        record[14] = (year >> 8) as u8;
        record[15] = year as u8;

        // observed: compares the installed firmware date with 0x80003188; a
        // shorter/older date shows the "incompatible firmware" error screen.
        let required = read_u32(0x8000_3188);
        if (date_word >> 16) & 0xff != (required >> 16) & 0xff
            || (date_word & 0x00ff_ffff) < (required & 0x00ff_ffff)
        {
            sdk_printf(0x8033_2568, &[]); // "OS_ERROR : This firmware is an incompatible version"
            call3(
                0x801a_f5d0,
                record.as_ptr() as u32,
                (record.as_ptr() as usize + 4) as u32,
                0x8033_25c8, // "ERROR #002: An error has occurred ..."
            );
            sdk_panic(sda(-0x61a0) as u32, 0x4d4, 0x8033_24f8);
        }

        // observed: static-init style one-shot (r13-0x4d24) that brings up the
        // IOS/console layer, calls 0x801d1230 and the 0x801d62xx-0x801d6550
        // device paths, then 0x801ca650 / 0x801cfa70. inferred: IOS device
        // initialisation (large; body not translated).
        call0(0x801c_ad30);

        if read_u32(sda(SLOT_FLAG_5040)) == 0 {
            // observed: when 0x800030e6 == 0, invalidate a 0x20-byte block at
            // 0x8038e540 and hand 0x8038e560 to 0x801cedb0 with the callback
            // 0x801aa510. inferred: alarm/timer callback setup.
            if read_u32(0x8000_30e6) == 0 {
                os::dc_invalidate_range(0x8038_e540, 0x20);
                call3(0x801c_edb0, 0x8038_e560, 0x8038_e540, 0x801a_a510);
            }
        } else {
            // observed: -0x7000 as a 32-bit store.
            write_u32(0x8000_30e6, 0xffff_9000);
        }

        // observed: 0x80003184 is the "console mode" byte; 0x40 is forced when
        // the reduced boot flag is set. 0x80 selects the menu-return path,
        // which is skipped when the debugger/loader state is present.
        let mut console_mode = read_u8(0x8000_3184);
        if read_u32(sda(SLOT_FLAG_5048)) != 0 {
            console_mode = 0x40;
        }
        if console_mode == 0x80
            && read_u32(sda(SLOT_FLAG_4FD4)) == 0
            && call0_ret(0x801d_0980) == 0
        {
            // observed: 0x801b3b00 reports "OSReturnToMenu() : Failed to boot
            // ..." through the OSReset.c panic path.
            call0(0x801b_3b00);
        }
    }

    // observed: play-time limit check ("OSPlayTime.c", "PlayTime = %d seconds
    // left"); only in normal boot.
    if read_u32(sda(SLOT_FLAG_5048)) == 0 && read_u32(sda(SLOT_FLAG_5044)) == 0 {
        call0(0x801b_84c0);
    }

    // observed: clears r13-0x4f38/-0x4f40/-0x6140/-0x4f3c/-0x4f30/-0x4f34/
    // -0x4f2c and calls 0x801b6d60(0,0) when 0x801da860 reports no device.
    // inferred: resets the "extra" peripheral state.
    if read_u32(sda(SLOT_FLAG_5048)) == 0
        && read_u32(sda(SLOT_FLAG_5044)) == 0
        && read_u32(sda(SLOT_FLAG_4FD4)) == 0
    {
        call0(0x801b_7220);
    }
}

// ---------------------------------------------------------------------------
// 0x801ab150 - paired-single / GQR bring-up
// ---------------------------------------------------------------------------

/// Enables paired-single mode, invalidates the instruction cache and clears
/// the eight graphics quantisation registers.
///
/// # Original
/// - Ghidra: `FUN_801ab150` at `0x801ab150` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801ab150_FUN_801ab150.c`; instruction
///   bytes: `mfhid2; oris 0xa000; mthid2` (set `WPE|PSE`), then a call to
///   `0x801ac5a0` (`HID0[ICFI]`), `sync`, `li r3,0` and eight
///   `mtspr 912+i, r3`.
/// - Caller: `__init_hardware` at `0x80004300`, before `0x801a9d40`.
///
/// # Arguments
/// None.
///
/// # Returns
/// 0 (the value is discarded by the caller).
///
/// # Side effects
/// - `HID2[WPE]` (`0x80000000`, write-gather pipe enable) is set
/// - `HID2[PSE]` (`0x20000000`, paired-single enable) is set
/// - the L1 instruction cache is invalidated
/// - `GQR0`-`GQR7` (SPR 912-919) are cleared, selecting the default
///   paired-single formats
///
/// The bit *names* come from the Broadway register documentation; only the bit
/// values are visible in this binary.
///
/// # Safety
/// Must run before any paired-single instruction executes, i.e. before
/// `0x801a9d40` and before the first C++ initialiser.
///
/// # Confidence
/// high - every bit and SPR number is decoded from the instruction bytes.
pub unsafe fn init_hardware_paired_single() -> u32 {
    let hid2 = os::get_hid2();
    // observed: 0xa0000000 = WPE (bit 31) | PSE (bit 29).
    os::set_hid2(hid2 | 0xA000_0000);
    os::invalidate_icache();
    os::sync();
    for index in 0..8 {
        os::set_gqr(index, 0);
    }
    0
}

// ---------------------------------------------------------------------------
// 0x801a9d40 - FPU enable and register fill
// ---------------------------------------------------------------------------

/// Enables the floating-point unit and loads a default value into every
/// floating-point register, in both paired-single and scalar halves.
///
/// # Original
/// - Ghidra: `FUN_801a9d40` at `0x801a9d40` (296 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801a9d40_FUN_801a9d40.c`. The
///   decompiler models a `ldexpf` call and a `u64` return; the instruction
///   bytes show neither:
///   `mfmsr; ori 0x2000; mtmsr` (set `MSR[FP]`), `mfhid2`, a test of
///   `HID2[PSE]`, then `psq_l f0` from `r13-0x5030`, thirty-one
///   `ps_mr fN,f0`, `lfd f0,-0x5028(r13)`, thirty-one `fmr fN,f0`, `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing usable. Ghidra reports a `u64` because the tail load targets `f0`;
/// `r3` is left holding the `HID2` image and is ignored by the caller.
///
/// # Side effects
/// - `MSR[FP]` is set (floating point enabled)
/// - `f1`-`f31` are overwritten with the 64-bit value at `r13-0x5028`
///   (`0x8042a658`) when the paired-single path is inactive, and with the
///   paired-single quad at `r13-0x5030` (`0x8042a650`) as well when `HID2[PSE]`
///   is set
/// - both slots live in BSS, so at boot the registers are filled with zeroes
///
/// # Safety
/// Requires `MSR[FP]`/`HID2` writes to be legal, i.e. after
/// [`init_hardware_paired_single`].
///
/// # Confidence
/// medium - control flow and register usage are certain from the bytes, but the
/// purpose ("clear the FPU register file") is inferred from the BSS-zero source
/// values, and `// partial:` the 62 register moves are recorded rather than
/// executed.
pub unsafe fn init_fpu_registers() {
    // observed: mfmsr; ori 0x2000; mtmsr - 0x2000 is MSR[FP].
    let msr = os::get_msr();
    os::set_msr(msr | 0x0000_2000);

    // observed: the paired-single source is the quad at r13-0x5030.
    let paired = (read_u32(sda(-0x5030)), read_u32(sda(-0x502c)));
    let hid2 = os::get_hid2();
    if hid2 & 0x2000_0000 != 0 {
        // observed: psq_l f0,0(r13-0x5030),0,0 then ps_mr f1..f31,f0.
        fill_float_registers(paired, None);
    }

    // observed: the scalar source is the double at r13-0x5028; `lfd` reads
    // both words, so the high word is at -0x5024.
    let scalar = (read_u32(sda(-0x5028)), read_u32(sda(-0x5024)));
    fill_float_registers(paired, Some(scalar));

    // partial: the 31 `ps_mr` + 31 `fmr` moves are not emitted; see
    // `fill_float_registers`.
}

/// Placeholder for the unrolled floating-point register fill.
///
/// # Original
/// - Ghidra: inlined into `FUN_801a9d40` at `0x801a9d40`
/// - Evidence: `801a9d40_FUN_801a9d40.c`; encodings `0x10200090`-`0x13e00090`
///   (`ps_mr f1..f31,f0`) and `0xfc200090`-`0xffe00090` (`fmr f1..f31,f0`).
///
/// # Arguments
/// - `paired`: the 8 bytes loaded by `psq_l` (`f0.ps0`, `f0.ps1`)
/// - `scalar`: the 8 bytes loaded by `lfd`, or `None` while only the
///   paired-single half is being filled
///
/// # Returns
/// Nothing.
///
/// # Unknown
/// Host Rust cannot write the PowerPC FPR file, so this keeps the source values
/// visible (and their BSS addresses checkable) without pretending to execute
/// the moves.
///
/// # Safety
/// No memory preconditions; `unsafe` only models privileged FPR writes.
///
/// # Confidence
/// high for the register range; high for the "all 31 registers" count.
unsafe fn fill_float_registers(paired: (u32, u32), scalar: Option<(u32, u32)>) {
    let _ = (paired, scalar);
}

// ---------------------------------------------------------------------------
// 0x801ac990 - cache initialisation
// ---------------------------------------------------------------------------

/// Enables the L1 instruction/data caches, brings up the L2 cache, and installs
/// the machine-check handler for locked-cache DMA errors.
///
/// Each step is skipped when the hardware already reports it as enabled, so the
/// function is safe to call after a warm reset.
///
/// # Original
/// - Ghidra: `FUN_801ac990` at `0x801ac990` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801ac990_FUN_801ac990.c`; the report
///   strings are at `0x803329a4`, `0x803329c0`, `0x803327f0`, `0x803329dc` and
///   `0x803329f4`. Caller: `os_init` (step 4) and `__init_hardware`
///   (`0x80004300`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `HID0[ICE]` / `HID0[DCE]` set when the corresponding L1 cache was off
/// - L2 control register (SPR 1017): disable, invalidate, wait for idle,
///   re-enable; the L2 is left enabled with the "initialised" bit (`0x80000000`)
///   set
/// - while the L2 invalidate runs, address translation is turned off
///   (`MSR[IR]`/`MSR[DR]`, mask `0x30`) and restored afterwards
/// - exception 1 (machine check, vector `0x100`) is pointed at `0x801ac840`
/// - one report line per step; the reporter at `0x801bb9e0` is compiled out in
///   this build, so nothing is printed in practice
///
/// # Safety
/// Must run with caches off or already coherent; it invalidates the L2 and
/// executes with translation disabled, which is only valid before any
/// translated address is live.
///
/// # Confidence
/// high - the decompiled control flow and the instruction bytes agree on every
/// SPR access.
pub unsafe fn init_cache() {
    // ---- L1 instruction cache ------------------------------------------
    // observed: test HID0 bit 16 (0x00008000 = ICE).
    if os::get_hid0() & 0x0000_8000 == 0 {
        os::enable_icache();
        os::os_report(0x8033_29a4); // "L1 i caches initialized"
    }

    // ---- L1 data cache --------------------------------------------------
    // observed: test HID0 bit 17 (0x00004000 = DCE).
    if os::get_hid0() & 0x0000_4000 == 0 {
        os::enable_dcache();
        os::os_report(0x8033_29c0); // "L1 d caches initialized"
    }

    // ---- L2 cache -------------------------------------------------------
    // observed: test the "L2 enabled" bit (0x80000000) of SPR 1017.
    if os::get_l2_control() & 0x8000_0000 == 0 {
        let saved_msr = os::get_msr();
        os::sync();
        // observed: 0x30 = MSR[IR] | MSR[DR]; the invalidate sequence runs
        // untranslated, so all addresses must be physical-equivalent.
        os::set_msr(0x30);
        os::sync();
        os::sync();

        // observed: clear the enable bit twice, then request invalidate.
        os::set_l2_control(os::get_l2_control() & 0x7fff_ffff);
        os::sync();
        os::sync();
        os::set_l2_control(os::get_l2_control() & 0x7fff_ffff);
        os::sync();
        // observed: 0x00200000 is the invalidate request bit.
        os::set_l2_control(os::get_l2_control() | 0x0020_0000);

        // observed: wait while the busy bit (0x1) is still set.
        while os::get_l2_control() & 1 != 0 {}
        os::set_l2_control(os::get_l2_control() & 0xFFDF_FFFF);

        // observed: second wait loop; the report is unreachable in practice
        // ("the invalidate should never still be running").
        while os::get_l2_control() & 1 != 0 {
            os::os_report(0x8033_27f0);
        }

        os::set_msr(saved_msr);
        // observed: clear the request bit and set "L2 enabled/initialised".
        os::set_l2_control((os::get_l2_control() & 0xFFDF_FFFF) | 0x8000_0000);
        os::os_report(0x8033_29dc); // "L2 cache initialized"
    }

    // ---- Machine-check handler ------------------------------------------
    // observed: `FUN_801ad610(1, 0x801ac840)`; exception 1 is the PowerPC
    // machine check at vector 0x100, and 0x801ac840 decodes the locked-cache
    // DMA error bits of HID2 before halting.
    os::set_exception_handler(1, MACHINE_CHECK_HANDLER);
    os::os_report(0x8033_29f4); // "Locked cache machine check handler installed"
}

/// Address of the locked-cache / DMA machine-check handler installed by
/// [`init_cache`].
///
/// # Original
/// - Ghidra: `FUN_801ac840` at `0x801ac840` - body not translated (belongs to
///   the exception lane)
/// - Evidence: `801ac840_FUN_801ac840.c` reports "Machine check received",
///   prints `HID2`/`SRR1`, decodes the four DMA error bits and calls
///   `0x801ad070` plus `0x801a9ba0` (halt) when the cause is not a locked-cache
///   DMA error.
///
/// # Confidence
/// high - the strings and bit tests are explicit.
const MACHINE_CHECK_HANDLER: usize = 0x801a_c840;

// ===========================================================================
// Wave 279 lane E (`_E279`) - gap batch 9, real-mode exception hand-off
// ===========================================================================
//
// One untabled gap body from the wave-266 candidate set, re-verified against
// `work/dol/main.elf`. Selection: `work/wave279/e_pick.mjs`; ELF re-walk and
// disassembly: `e_verify.mjs` and `e_verify_stdout.txt`. It starts with
// `9421FFF0`, its recursive descent ends on exactly one `blr`, it overlaps no
// documented extent and no pointer-run word names it.

/// Fatal `DBExceptionDestination` report: prints the destination, dumps the
/// exception context and halts.
///
/// `r3` holds the boot string at `0x80340050`, `"DBExceptionDestination\n"`.
/// The destination argument is the low-memory physical word at `0xC0` with
/// `0x80000000` added by `addis r4,r4,0x8000`, and `creqv 6,6,6` sets `CR6`
/// before the varargs report. `0x8015F730` prints the line, `0x801AD070` dumps
/// the context snapshot it fetches through `0x802DCD6C` (GPRs, LR/CR,
/// SRR0/SRR1, GQRs, FPRs, PSFs and the back chain), and `0x801A9BA0`
/// (`sync` plus an endless loop) halts. The body never returns.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801BB960`.
/// - Extent: `0x801BB960..0x801BB9A8` (72 bytes, 17 instructions), ending at
///   the `blr` at `0x801BB9A4`; two zero words precede it at
///   `0x801BB958..0x801BB960` and the next body starts at `0x801BB9C0`.
/// - Reach: one static `b` enters it, the word `4BFFFFA4` at `0x801BB9BC`;
///   that instruction is the last word of the four-instruction real-mode
///   trampoline at `0x801BB9B0` (`mfmsr`; `ori 0x30`; `mtmsr`; `b`) and its
///   target is this body. No data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `3C608034 lis r3,0x8034`;
///   `stw r0,0x14(r1)`; `38630050 addi r3,r3,0x50` (`0x80340050`);
///   `stw r31,0x0C(r1)`; `808000C0 lwz r4,0xC0` (absolute low memory);
///   `3FE48000 addis r31,r4,0x8000`; `4CC63182 creqv 6,6,6`;
///   `4BFA3DAD bl 0x8015F730`; `mr r3,r31`;
///   `4BFF16E5 bl 0x801AD070`; `4BFEE211 bl 0x801A9BA0`; epilogue;
///   `4E800020 bclr 20,0` (blr, never reached on the target).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing - `0x801A9BA0` never returns on the target.
///
/// # Side effects
/// Prints the exception destination and the full register context, then halts.
///
/// # Safety
/// Must only run on the fatal exception path; it does not return.
///
/// # Confidence
/// high - every address, the string, the low-memory load and the halt helper
/// are literal. The `mr r3,r31` before `0x801AD070` is retained for
/// instruction-level fidelity even though that dump reads its snapshot through
/// `0x802DCD6C`.
pub unsafe fn fn_801bb960() -> ! {
    let destination = read_u32(0xC0).wrapping_add(0x8000_0000);
    call2(0x8015_F730, 0x8034_0050, destination);
    call1(0x801A_D070, destination);
    call0(0x801A_9BA0);
    loop {}
}
