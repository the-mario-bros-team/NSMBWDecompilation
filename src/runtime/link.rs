//! REL dynamic linker: `OSLink`, `OSLinkFixed`, `OSUnlink`, and the two
//! relocation engines behind them.
//!
//! # What lives here
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x801b1910` | `fn_801b1910` | empty `blr` hook called at the end of `link` |
//! | `0x801b1920` | `fn_801b1920` | empty `blr` hook called by `OSUnlink` |
//! | `0x801b1930` | `fn_801b1930` | empty `blr` hook called when an import entry matches |
//! | `0x801b1940` | `fn_801b1940` | empty `blr` hook called at the end of a relocation stream |
//! | `0x801b1950` | [`os_set_string_table`] | store the module-name string-table base |
//! | `0x801b1960` | [`relocate`] | apply one importer's relocations against one provider |
//! | `0x801b1c30` | [`link`] | version/alignment checks, list insert, pointer fixups, BSS zero |
//! | `0x801b1f20` | [`os_link`] | public `OSLink(module, bss)` wrapper |
//! | `0x801b1f30` | [`os_link_fixed`] | public `OSLinkFixed(module, bss)` wrapper |
//! | `0x801b1f50` | [`undo`] | remove one importer's relocations against one provider |
//! | `0x801b2190` | [`os_unlink`] | public `OSUnlink(module)` |
//! | `0x801b2370` | [`os_module_init`] | clear the module list and string table |
//!
//! # Original address range
//!
//! `0x801b1910..0x801b2387` in `main.dol` text. The four 4-byte hooks and the
//! 12-byte string-table setter sit immediately before the linker; the module
//! list resetter follows `OSUnlink`. Evidence is the Ghidra C under
//! `work/ghidra/main-gb/export/` plus the raw big-endian instruction words in
//! `work/dol/main.elf` (the saved register helpers at `0x802dcd40` make the
//! Ghidra argument mapping unreliable, so the assembly was used to confirm
//! argument order and the `unaff_r28` relocation cursor).
//!
//! # Why it exists
//!
//! A Wii **REL** ([glossary](../../docs/GLOSSARY.md)) is a relocatable module
//! like a DLL. It holds section bytes and a table of places that must be
//! patched once the module knows where it was placed. [`os_link`] is the SDK
//! entry point that performs those patches. NSMBW ships four RELs:
//! `d_profileNP`, `d_basesNP`, `d_enemiesNP`, and `d_en_bossNP`.
//!
//! # What the linker does, step by step
//!
//! 1. **Validate.** Version greater than 3 fails. Versions 2 and 3 must have
//!    `module % align == 0` and `bss % bss_align == 0` when those header fields
//!    are non-zero. Versions 0 and 1 skip the alignment test.
//!    `// observed:` the raw header fields are used as byte multiples, not as
//!    exponents (the shipped values are `align = 16`, `bssAlign = 8`).
//! 2. **Insert** the module at the tail of the global module list at
//!    `0x800030c8` / `0x800030cc`.
//! 3. **Make the header absolute.** `sectionInfoOffset`, `relOffset`, and
//!    `impOffset` gain the module base. For version 3, `fixSize` gains it too.
//! 4. **Patch the section table.** Every non-zero section offset becomes
//!    `base + offset`; the low bit (executable flag) is preserved. A section
//!    with offset zero and size non-zero becomes the BSS section: its offset is
//!    set to the caller's `bss` buffer and its index is stored in `bssSection`.
//! 5. **Patch the import table.** Each 8-byte entry's relocation-stream offset
//!    becomes an absolute address.
//! 6. **Patch entry points.** `prolog`, `epilog`, and `unresolved` gain the
//!    address of their section (`section` field selects the section).
//! 7. **Patch the module name.** If `0x800030d0` holds a string-table base, it
//!    is added to `nameOffset`. The base is installed by
//!    [`os_set_string_table`] when the game loads `WIIMJ2DNP.str`.
//! 8. **Relocate.** The module's imports from `main.dol` are applied first
//!    (module id 0). Then, for every module already in the list, its imports
//!    from the new module are applied and the new module's imports from it are
//!    applied. This is the `Relocate(provider, importer)` pass.
//! 9. **Optional import cut (`OSLinkFixed`).** Cut `impSize` at the first
//!    import entry whose module id is 0 or the module's own id. This is what
//!    makes the `fixSize` region reusable.
//! 10. **Zero BSS** with the CRT `memset` wrapper at `0x800046b4`, then call
//!     the link-notification hook.
//!
//! # What the linker does *not* do
//!
//! `OSLink` does **not** read the REL file, decompress it, allocate memory, or
//! copy section bytes. The caller must already have placed the complete file
//! image at `newModule`; the "base" is simply that pointer. There is no base
//! field in the REL header. [`recover_base`] recovers the base from an
//! already-linked image, and the section table is the only place the loader
//! learns each section's address.
//!
//! # Why synthetic bases exist
//!
//! The four analysis images use `0x80500000`, `0x80600000`, `0x80A00000`, and
//! `0x80C00000` (`docs/REL.md`, `docs/REL2ELF.md`). Those values are **not**
//! read from the REL and are **not** chosen by `OSLink`: they are fixed,
//! non-overlapping slots chosen so the four modules can be loaded together in
//! one static analysis database. In the game the real address comes from
//! `dDyl`'s allocator and is passed to `OSLink` as the module pointer. The
//! loader only checks that the address satisfies the header's alignment field.
//! Because every cross-module pointer and branch displacement depends on the
//! chosen base, a synthetic-base image is correct only for that slot; see
//! `docs/REL2ELF.md` "Synthetic bases are not runtime addresses".
//!
//! For an already-linked module in memory the real base is
//! `sectionInfoOffset - 0x4c`, because all four shipped RELs place the section
//! table immediately after the 0x4c-byte header. [`recover_base`] performs that
//! check and validates the module id and section count.
//!
//! # REL / `OSModuleHeader` layout (big-endian, from `docs/REL.md`)
//!
//! | offset | width | field |
//! |---:|---:|---|
//! | `0x00` | 4 | `id` (0 = `main.dol`) |
//! | `0x04` | 4 | `next` module (runtime) |
//! | `0x08` | 4 | `prev` module (runtime) |
//! | `0x0c` | 4 | `numSections` |
//! | `0x10` | 4 | `sectionInfoOffset` |
//! | `0x14` | 4 | `nameOffset` (into the separate string table) |
//! | `0x18` | 4 | `nameSize` |
//! | `0x1c` | 4 | `version` (0..3 accepted here) |
//! | `0x20` | 4 | `bssSize` |
//! | `0x24` | 4 | `relOffset` |
//! | `0x28` | 4 | `impOffset` |
//! | `0x2c` | 4 | `impSize` |
//! | `0x30` | 1 | `prologSection` |
//! | `0x31` | 1 | `epilogSection` |
//! | `0x32` | 1 | `unresolvedSection` |
//! | `0x33` | 1 | `bssSection` (written by this loader) |
//! | `0x34` | 4 | `prolog` offset |
//! | `0x38` | 4 | `epilog` offset |
//! | `0x3c` | 4 | `unresolved` offset |
//! | `0x40` | 4 | `align` (version 2+) |
//! | `0x44` | 4 | `bssAlign` (version 2+) |
//! | `0x48` | 4 | `fixSize` (version 3) |
//!
//! Section table entries are 8 bytes: `{ raw_offset, size }`. Bit 0 of
//! `raw_offset` is the executable flag; the upper bits are the file offset.
//! Import table entries are 8 bytes: `{ module_id, relocation_stream_offset }`.
//! Relocation records are 8 bytes: `{ u16 delta, u8 type, u8 section, u32 addend }`.
//!
//! # Relocation types
//!
//! Let `S` be the resolved symbol address and `P` the address being patched.
//! For module id 0, `S = 0` and the addend is already the absolute `main.dol`
//! address, so every write uses `A` directly.
//!
//! | type | name | value written |
//! |---:|---|---|
//! | 0 | `R_PPC_NONE` | nothing |
//! | 1 | `R_PPC_ADDR32` | `S + A` |
//! | 2 | `R_PPC_ADDR24` | `(S + A) & 0x03fffffc \| old & 0xfc000003` |
//! | 3 | `R_PPC_ADDR16` | `S + A` (low 16) |
//! | 4 | `R_PPC_ADDR16_LO` | `S + A` (low 16) |
//! | 5 | `R_PPC_ADDR16_HI` | `(S + A) >> 16` |
//! | 6 | `R_PPC_ADDR16_HA` | `((S + A) >> 16) + (((S + A) >> 15) & 1)` |
//! | 7-9 | `R_PPC_ADDR14*` | `(S + A) & 0x0000fffc \| old & 0xffff0003` |
//! | 10 | `R_PPC_REL24` | `((S + A) - P) & 0x03fffffc \| old & 0xfc000003` |
//! | 11-13 | `R_PPC_REL14*` | `((S + A) - P) & 0x0000fffc \| old & 0xffff0003` |
//! | 201 | `R_DOLPHIN_NOP` | nothing; the delta still advances `P` |
//! | 202 | `R_DOLPHIN_SECTION` | select a target section in the importer |
//! | 203 | `R_DOLPHIN_END` | stop this stream |
//!
//! The four shipped RELs exercise only types 1, 4, 6, and 10 (198,456
//! relocations; see `docs/REL2ELF.md`). Types 2, 3, 5, 7-9, and 11-13 are
//! translated from the same dispatch even though these files do not use them.
//! Unknown types print `OSLink: unknown relocation type %3d` (the format
//! string is at `0x80333a98`; the reporter is `0x8015f730`) and continue.
//!
//! # Dependencies
//!
//! * [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! * [`crate::runtime::os::dc_flush_range`] - `0x801ac470`, the `dcbf` range
//!   flush used after patching an executable section.
//! * `0x801ac560` - `ICInvalidateRange` (`icbi` loop plus `sync`/`isync`),
//!   translated in [`crate::runtime::os::ic_invalidate_range`]; this module
//!   delegates to it through [`ic_invalidate_range`].
//! * [`crate::boot::mem::memset_wrapper`] - `0x800046b4`, the CRT zero-fill
//!   used for BSS.
//!
//! # Start here
//!
//! [`os_link`] (public entry), then [`link`] (the actual work) and [`relocate`]
//! (the relocation engine). [`os_unlink`] is the inverse lifecycle.
//!
//! # Unknowns
//!
//! * The four empty hooks at `0x801b1910`-`0x801b1940` are real functions with
//!   a single `blr`; their roles are inferred from call position only. The SDK
//!   names `OSNotifyLink` / `OSNotifyUnlink` fit `0x801b1910` / `0x801b1920`,
//!   but the two per-stream hooks have no name evidence.
//! * `fixSize` is only observed to become absolute. How the caller reuses the
//!   region after `OSLinkFixed` is outside this cluster.
//! * No direct call site for `0x801b1f20` or `0x801b1f30` exists in the
//!   `main.dol` export; they are public entry points reached indirectly (the
//!   `dDyl` loader is in a different lane).
//! * `docs/SYMBOLS.md` labelled `0x801b1960` as `OSLink` because that is where
//!   the "OSLink: unknown relocation type" string is referenced. The body is
//!   the SDK's static `Relocate` helper; the public `OSLink` is the 8-byte
//!   wrapper at `0x801b1f20`.
//! * `Relocate`, `Undo`, `Link`, and `__OSModuleInit` are SDK-conventional
//!   names. The export contains only `FUN_*` symbols and the two format
//!   strings, so every descriptive name here is an inference from the function
//!   shape, call position, and the public API it serves.

use crate::boot::mem::memset_wrapper;
use crate::platform::{call2, read_u8, read_u16, read_u32, write_u16, write_u32, write_u8};
use crate::runtime::os::dc_flush_range;

// ---------------------------------------------------------------------------
// OSModuleHeader field offsets
// ---------------------------------------------------------------------------

/// `id`: unique module number; 0 means `main.dol`.
const MODULE_ID: usize = 0x00;
/// `next`: next module in the global list (runtime pointer).
const MODULE_NEXT: usize = 0x04;
/// `prev`: previous module in the global list (runtime pointer).
const MODULE_PREV: usize = 0x08;
/// `numSections`: number of 8-byte section-table entries.
const MODULE_NUM_SECTIONS: usize = 0x0c;
/// `sectionInfoOffset`: section table, absolute after [`link`].
const MODULE_SECTION_INFO: usize = 0x10;
/// `nameOffset`: offset into the separate module-name string table.
const MODULE_NAME_OFFSET: usize = 0x14;
/// `version`: REL format version (0..3 accepted by this loader).
const MODULE_VERSION: usize = 0x1c;
/// `bssSize`: bytes to zero at the caller-provided BSS address.
const MODULE_BSS_SIZE: usize = 0x20;
/// `relOffset`: start of all relocation data, absolute after [`link`].
const MODULE_REL_OFFSET: usize = 0x24;
/// `impOffset`: import table, absolute after [`link`].
const MODULE_IMP_OFFSET: usize = 0x28;
/// `impSize`: import table size in bytes; shrinkable by `OSLinkFixed`.
const MODULE_IMP_SIZE: usize = 0x2c;
/// `prologSection`: section index for the module entry point.
const MODULE_PROLOG_SECTION: usize = 0x30;
/// `epilogSection`: section index for the module exit point.
const MODULE_EPILOG_SECTION: usize = 0x31;
/// `unresolvedSection`: section index for the unresolved-function handler.
const MODULE_UNRESOLVED_SECTION: usize = 0x32;
/// `bssSection`: section index of the BSS entry; written by [`link`].
const MODULE_BSS_SECTION: usize = 0x33;
/// `prolog`: entry-point offset inside `prologSection`.
const MODULE_PROLOG: usize = 0x34;
/// `epilog`: exit-point offset inside `epilogSection`.
const MODULE_EPILOG: usize = 0x38;
/// `unresolved`: unresolved-handler offset inside `unresolvedSection`.
const MODULE_UNRESOLVED: usize = 0x3c;
/// `align`: module alignment constraint (version 2+), treated as bytes.
const MODULE_ALIGN: usize = 0x40;
/// `bssAlign`: BSS alignment constraint (version 2+), treated as bytes.
const MODULE_BSS_ALIGN: usize = 0x44;
/// `fixSize`: version-3 `OSLinkFixed` reserve boundary.
const MODULE_FIX_SIZE: usize = 0x48;

/// Size of the fixed part of `OSModuleHeader`, i.e. where the shipped RELs
/// place their section table. See [`recover_base`].
const MODULE_HEADER_SIZE: usize = 0x4c;

/// One section-table entry: `{ raw_offset, size }`.
const SECTION_ENTRY_SIZE: usize = 8;
/// One import-table entry: `{ module_id, relocation_stream_offset }`.
const IMPORT_ENTRY_SIZE: usize = 8;
/// One relocation record: `{ u16 delta, u8 type, u8 section, u32 addend }`.
const RELOCATION_SIZE: usize = 8;

// ---------------------------------------------------------------------------
// OS low-memory globals used by the linker
// ---------------------------------------------------------------------------

/// Head of the global `__OSModuleInfoList` (`0x800030c8`).
const MODULE_LIST_HEAD: usize = 0x8000_30c8;
/// Tail of the global `__OSModuleInfoList` (`0x800030cc`).
const MODULE_LIST_TAIL: usize = 0x8000_30cc;
/// `__OSStringTable` (`0x800030d0`), installed by [`os_set_string_table`].
const STRING_TABLE_BASE: usize = 0x8000_30d0;

/// `OSLink: unknown relocation type %3d` in `main.dol` data.
const STR_OSLINK_UNKNOWN: usize = 0x8033_3a98;
/// `OSUnlink: unknown relocation type %3d` in `main.dol` data.
const STR_OSUNLINK_UNKNOWN: usize = 0x8033_3ac0;
/// Varargs reporter at `0x8015f730` (first vararg arrives in `r4`).
const REPORT: usize = 0x8015_f730;
/// `ICInvalidateRange` at `0x801ac560`; translated by
/// [`crate::runtime::os::ic_invalidate_range`], which owns this address.
const IC_INVALIDATE_RANGE: usize = 0x801a_c560;

// ---------------------------------------------------------------------------
// Relocation control codes
// ---------------------------------------------------------------------------

/// `R_PPC_NONE` (0): no write; the delta still advances the cursor.
const R_PPC_NONE: u8 = 0;
/// `R_PPC_ADDR32` (1): full 32-bit absolute address.
const R_PPC_ADDR32: u8 = 1;
/// `R_PPC_ADDR24` (2): 24-bit absolute field inside an instruction.
const R_PPC_ADDR24: u8 = 2;
/// `R_PPC_ADDR16` (3): low 16 bits of an absolute address.
const R_PPC_ADDR16: u8 = 3;
/// `R_PPC_ADDR16_LO` (4): low 16 bits of an absolute address.
const R_PPC_ADDR16_LO: u8 = 4;
/// `R_PPC_ADDR16_HI` (5): high 16 bits of an absolute address.
const R_PPC_ADDR16_HI: u8 = 5;
/// `R_PPC_ADDR16_HA` (6): high 16 bits, adjusted by the low half's sign bit.
const R_PPC_ADDR16_HA: u8 = 6;
/// `R_PPC_ADDR14` (7): 14-bit absolute field.
const R_PPC_ADDR14: u8 = 7;
/// `R_PPC_ADDR14_BRTAKEN` (8): same field patch as type 7 here.
const R_PPC_ADDR14_BRTAKEN: u8 = 8;
/// `R_PPC_ADDR14_BRNTAKEN` (9): same field patch as type 7 here.
const R_PPC_ADDR14_BRNTAKEN: u8 = 9;
/// `R_PPC_REL24` (10): 24-bit PC-relative branch field.
const R_PPC_REL24: u8 = 10;
/// `R_PPC_REL14` (11): 14-bit PC-relative branch field.
const R_PPC_REL14: u8 = 11;
/// `R_PPC_REL14_BRTAKEN` (12): same field patch as type 11 here.
const R_PPC_REL14_BRTAKEN: u8 = 12;
/// `R_PPC_REL14_BRNTAKEN` (13): same field patch as type 11 here.
const R_PPC_REL14_BRNTAKEN: u8 = 13;

/// `R_DOLPHIN_NOP` (201): advance the cursor without writing.
const R_DOLPHIN_NOP: u8 = 201;
/// `R_DOLPHIN_SECTION` (202): select the importer's target section.
const R_DOLPHIN_SECTION: u8 = 202;
/// `R_DOLPHIN_END` (203): end this relocation stream.
const R_DOLPHIN_END: u8 = 203;

// ---------------------------------------------------------------------------
// Small accessors
// ---------------------------------------------------------------------------

/// Returns the address of section-table entry `index` of a linked module.
///
/// # Safety
/// `module` must point to a linked module whose `sectionInfoOffset` is absolute.
#[inline]
unsafe fn section_entry(module: usize, index: u8) -> usize {
    read_u32(module + MODULE_SECTION_INFO) as usize + index as usize * SECTION_ENTRY_SIZE
}

/// Reads the relocated address of section `index` (executable bit cleared).
///
/// # Safety
/// See [`section_entry`].
#[inline]
unsafe fn section_address(module: usize, index: u8) -> u32 {
    read_u32(section_entry(module, index)) & !1
}

/// Flushes one executable section after its code has been patched.
///
/// The loader calls `DCFlushRange` (`0x801ac470`) and `ICInvalidateRange`
/// (`0x801ac560`) with the section start and size. `section_entry` is the
/// address of the 8-byte section-table entry, not the section itself.
///
/// # Original
/// - Inline in `FUN_801b1960` at `0x801b1b7c..0x801b1b98` and
///   `0x801b1bdc..0x801b1bf8`; the same pair appears in `FUN_801b1f50`.
/// - Evidence: `801b1960_FUN_801b1960.c` lines 104-108 and 122-126.
///
/// # Safety
/// The section entry must belong to a linked module and describe mapped,
/// executable memory.
unsafe fn flush_section(section_entry: usize) {
    let start = (read_u32(section_entry) & !1) as usize;
    let size = read_u32(section_entry + 4);
    dc_flush_range(start, size);
    ic_invalidate_range(start, size);
}

/// Invalidates the instruction cache over `[start, start + size)`.
///
/// # Original
/// - Ghidra: `FUN_801ac560` at `0x801ac560` (52 bytes), an `icbi` block loop
///   followed by `sync` and `isync`.
/// - Evidence: `801ac560_FUN_801ac560.c`; instruction words at `0x801ac57c`
///   (`0x7c001fac`, `icbi`) and `0x801ac588` (`0x7c0004ac`, `sync`).
///
/// # Arguments
/// * `start` - first byte of the range.
/// * `size` - length in bytes; zero is a no-op.
///
/// # Side effects
/// Discards cached instruction lines. The caller must be past the patched code.
///
/// # Safety
/// The range must be mapped executable memory.
///
/// # Confidence
/// high - the instruction sequence is the canonical SDK `ICInvalidateRange`.
/// Delegates to [`crate::runtime::os::ic_invalidate_range`], the canonical owner of this address.
unsafe fn ic_invalidate_range(start: usize, size: u32) {
    // canonical: crate::runtime::os::ic_invalidate_range owns this address.
    crate::runtime::os::ic_invalidate_range(start, size);
}

/// Reports an unknown relocation type and continues.
///
/// # Original
/// - Inline in `FUN_801b1960` at `0x801b1bb8` and `FUN_801b1f50` at
///   `0x801b2124`: `li r3,<format>; crxor 6,6,6; bl 0x8015f730`. The type is
///   still in `r4` from the stream load, so it becomes the first vararg.
///
/// # Arguments
/// * `format` - virtual address of the `%3d` format string.
/// * `reloc_type` - the unrecognised type byte.
///
/// # Safety
/// The reporter is a varargs function; this call matches its `(format, value)`
/// shape.
unsafe fn report_unknown(format: usize, reloc_type: u8) {
    let _ = call2(REPORT, format as u32, reloc_type as u32);
}

// ---------------------------------------------------------------------------
// Empty hooks
// ---------------------------------------------------------------------------

/// Empty hook called at the end of [`link`].
///
/// # Original
/// - Ghidra: `FUN_801b1910` at `0x801b1910` (4 bytes)
/// - Evidence: `801b1910_FUN_801b1910.c` is an empty body; the instruction word
///   at `0x801b1910` is `0x4e800020` (`blr`).
///
/// # Unknown
/// The SDK names this slot `OSNotifyLink` by position, but the shipped body is
/// a no-op and there is no string evidence.
///
/// # Safety
/// No memory access.
unsafe fn fn_801b1910() {}

/// Empty hook called by [`os_unlink`] after the relocation undo pass.
///
/// # Original
/// - Ghidra: `FUN_801b1920` at `0x801b1920` (4 bytes)
/// - Evidence: `801b1920_FUN_801b1920.c`; body word `0x4e800020` (`blr`).
///
/// # Unknown
/// Probably `OSNotifyUnlink`, but the body is empty and the name is inferred.
///
/// # Safety
/// No memory access.
unsafe fn fn_801b1920() {}

/// Empty hook called when [`relocate`] or [`undo`] finds a matching import.
///
/// # Original
/// - Ghidra: `FUN_801b1930` at `0x801b1930` (4 bytes)
/// - Evidence: `801b1930_FUN_801b1930.c`; body word `0x4e800020` (`blr`).
///
/// # Unknown
/// No name evidence; the original passes the two module pointers in `r3`/`r4`,
/// which this no-op ignores.
///
/// # Safety
/// No memory access.
unsafe fn fn_801b1930() {}

/// Empty hook called when a relocation stream reaches `R_DOLPHIN_END`.
///
/// # Original
/// - Ghidra: `FUN_801b1940` at `0x801b1940` (4 bytes)
/// - Evidence: `801b1940_FUN_801b1940.c`; body word `0x4e800020` (`blr`).
///
/// # Unknown
/// No name evidence; called with the two module pointers in `r3`/`r4`, ignored.
///
/// # Safety
/// No memory access.
unsafe fn fn_801b1940() {}

// ---------------------------------------------------------------------------
// String table
// ---------------------------------------------------------------------------

/// Installs the base of the separate module-name string table.
///
/// REL modules store `nameOffset` relative to a string table that lives in a
/// different file (`WIIMJ2DNP.str` in NSMBW). Before a module is linked, the
/// game must tell the OS where that table was loaded. [`link`] and
/// [`os_unlink`] then add/subtract this base from `nameOffset`.
///
/// # Original
/// - Ghidra: `FUN_801b1950` at `0x801b1950` (12 bytes), the SDK's
///   `OSSetStringTable`.
/// - Evidence: `801b1950_FUN_801b1950.c` (`_DAT_800030d0 = param_1`), and the
///   instruction words `lis r4,0x8000; stw r3,0x30d0(r4); blr`.
///
/// # Arguments
/// * `table` - virtual address of the loaded string table; 0 clears it.
///
/// # Side effects
/// Writes the global at `0x800030d0`.
///
/// # Safety
/// `table` is only stored, not dereferenced here; callers must pass a mapped
/// table before linking modules whose `nameOffset` is non-zero.
///
/// # Confidence
/// high - the function body and the SDK `OSSetStringTable` shape are identical.
/// The caller `0x8008f640` loads `WIIMJ2DNP.str` and passes its buffer.
pub unsafe fn os_set_string_table(table: usize) {
    write_u32(STRING_TABLE_BASE, table as u32);
}

// ---------------------------------------------------------------------------
// Relocation engine
// ---------------------------------------------------------------------------

/// Finds the import-table entry of `importer` that names `module_id`.
///
/// # Arguments
/// * `table` - absolute import-table address.
/// * `end` - one byte past the table (from `impSize`).
/// * `module_id` - module id to search for (0 = `main.dol`).
///
/// # Returns
/// The entry address, or `None`.
///
/// # Original
/// - Inline in `FUN_801b1960` at `0x801b1990..0x801b19cc` and in
///   `FUN_801b1f50` at `0x801b1f64..0x801b1fac`.
/// - Evidence: `801b1960_FUN_801b1960.c` lines 31-45; the loop is
///   `count = (impSize + 7) >> 3` with a `cmplw` guard.
///
/// # Safety
/// `table..end` must be readable; `end` must not wrap.
unsafe fn find_import_entry(table: usize, end: usize, module_id: u32) -> Option<usize> {
    if table >= end {
        return None;
    }

    // The SDK rounds impSize up to whole 8-byte entries and then runs that
    // count without re-checking the end pointer. Keep the same shape: the
    // shipped tables are exact multiples of 8, so this only matters for a
    // malformed header.
    let count = end.wrapping_sub(table).wrapping_add(IMPORT_ENTRY_SIZE - 1) / IMPORT_ENTRY_SIZE;
    let mut entry = table;
    let mut i = 0usize;
    while i < count {
        if read_u32(entry) == module_id {
            return Some(entry);
        }
        entry = entry.wrapping_add(IMPORT_ENTRY_SIZE);
        i += 1;
    }
    None
}

/// Applies one non-control relocation record to `place`.
///
/// The caller has already resolved the symbol base from the provider's section
/// table (or zero for module 0) and advanced `place` by the record's delta.
///
/// # Original
/// - Inline switch in `FUN_801b1960` at `0x801b1a1c..0x801b1bc0`.
/// - Evidence: `801b1960_FUN_801b1960.c` lines 51-117. Type 6's two-instruction
///   high-adjusted sequence is at `0x801b1ae4..0x801b1af8`; type 10's
///   `subf`/mask sequence is at `0x801b1b1c..0x801b1b34`.
///
/// # Arguments
/// * `reloc_type` - record type byte.
/// * `symbol` - provider section address, or 0 for module id 0.
/// * `place` - address being patched.
/// * `addend` - record addend (absolute for module 0).
///
/// # Side effects
/// Writes 2 or 4 bytes at `place`; unknown types only report.
///
/// # Safety
/// `place` must be writable for the width implied by `reloc_type`.
unsafe fn apply_relocation(reloc_type: u8, symbol: u32, place: usize, addend: u32) {
    let value = symbol.wrapping_add(addend);
    match reloc_type {
        R_PPC_NONE => {}
        R_PPC_ADDR32 => write_u32(place, value),
        R_PPC_ADDR24 => {
            let old = read_u32(place);
            write_u32(place, (value & 0x03ff_fffc) | (old & 0xfc00_0003));
        }
        R_PPC_ADDR16 | R_PPC_ADDR16_LO => write_u16(place, value as u16),
        R_PPC_ADDR16_HI => write_u16(place, (value >> 16) as u16),
        R_PPC_ADDR16_HA => {
            // Add the sign bit of the low half before taking the high half.
            let adjusted = (value >> 16).wrapping_add((value >> 15) & 1);
            write_u16(place, adjusted as u16);
        }
        R_PPC_ADDR14 | R_PPC_ADDR14_BRTAKEN | R_PPC_ADDR14_BRNTAKEN => {
            let old = read_u32(place);
            write_u32(place, (value & 0x0000_fffc) | (old & 0xffff_0003));
        }
        R_PPC_REL24 => {
            let old = read_u32(place);
            let displacement = value.wrapping_sub(place as u32);
            write_u32(place, (displacement & 0x03ff_fffc) | (old & 0xfc00_0003));
        }
        R_PPC_REL14 | R_PPC_REL14_BRTAKEN | R_PPC_REL14_BRNTAKEN => {
            let old = read_u32(place);
            let displacement = value.wrapping_sub(place as u32);
            write_u32(place, (displacement & 0x0000_fffc) | (old & 0xffff_0003));
        }
        _ => report_unknown(STR_OSLINK_UNKNOWN, reloc_type),
    }
}

/// Applies every relocation that `importer` has for the module `provider`.
///
/// This is the SDK's static `Relocate(newModule, module)`. In this translation
/// the two roles are named by what they do:
/// * `provider` - the module that defines the symbols. Pass `0` for `main.dol`;
///   its addends are already absolute and no section base is added.
/// * `importer` - the module whose section bytes are patched. Its import table
///   must contain an entry for `provider`'s module id.
///
/// The function walks the importer's import table for the matching entry, then
/// follows the delta-encoded stream:
/// * a normal record advances the cursor by `delta` and patches the field;
/// * `R_DOLPHIN_SECTION` selects one of the importer's sections as the new
///   target and flushes the previous executable section;
/// * `R_DOLPHIN_END` flushes the last section, calls the end hook, and returns
///   success;
/// * no matching import entry returns failure without touching memory.
///
/// # Original
/// - Ghidra: `FUN_801b1960` at `0x801b1960` (708 bytes), the SDK's `Relocate`.
/// - Evidence: `801b1960_FUN_801b1960.c`; raw words at
///   `0x801b19d4..0x801b1c20`.
///
/// # Arguments
/// * `provider` - module that supplies section bases, or 0 for `main.dol`.
/// * `importer` - linked module whose relocations are applied.
///
/// # Returns
/// `true` when an import entry was found and the stream reached
/// `R_DOLPHIN_END`; `false` when the importer has no entry for the provider.
///
/// # Side effects
/// Patches code/data words in the importer; flushes data and instruction
/// caches for executable sections; may report an unknown relocation type.
///
/// # Safety
/// Both pointers must reference linked modules with absolute header fields and
/// mapped section tables. The relocation stream must be well formed; there is
/// no bounds check on `delta` or the target address.
///
/// # Confidence
/// high - the argument order, cursor initialization, and every type arm were
/// confirmed against the raw instruction words (Ghidra's `unaff_r28` is the
/// running target cursor, initialized by the first `R_DOLPHIN_SECTION` entry).
pub unsafe fn relocate(provider: usize, importer: usize) -> bool {
    let provider_id = if provider != 0 {
        read_u32(provider + MODULE_ID)
    } else {
        0
    };

    let table = read_u32(importer + MODULE_IMP_OFFSET) as usize;
    let end = table.wrapping_add(read_u32(importer + MODULE_IMP_SIZE) as usize);
    let Some(entry) = find_import_entry(table, end, provider_id) else {
        return false;
    };

    fn_801b1930();

    let mut cursor = read_u32(entry + 4) as usize;
    // The original keeps the running target in r28, which the compiler never
    // initializes because every stream in the shipped RELs starts with a
    // SECTION record. A zero start makes a malformed stream fault at a low
    // address instead of silently using a stale caller register.
    let mut place: usize = 0;
    // Section-table entry of the last executable section, or 0 if none.
    let mut previous_section: usize = 0;

    loop {
        let reloc_type = read_u8(cursor + 2);

        if reloc_type == R_DOLPHIN_END {
            if previous_section != 0 {
                flush_section(previous_section);
            }
            fn_801b1940();
            return true;
        }

        place = place.wrapping_add(read_u16(cursor) as usize);

        // The original resolves the provider section base before the type
        // dispatch, so even NOP/SECTION records perform this read. The value is
        // only used by the address-writing types; keep it here to match the
        // original's memory-access order.
        let symbol = if provider_id == 0 {
            0
        } else {
            section_address(provider, read_u8(cursor + 3))
        };

        match reloc_type {
            R_DOLPHIN_NOP => {}
            R_DOLPHIN_SECTION => {
                let index = read_u8(cursor + 3);
                let entry = section_entry(importer, index);
                let raw = read_u32(entry);
                place = (raw & !1) as usize;

                if previous_section != 0 {
                    flush_section(previous_section);
                }

                // Only executable sections need the instruction-cache flush.
                previous_section = if raw & 1 != 0 { entry } else { 0 };
            }
            _ => apply_relocation(reloc_type, symbol, place, read_u32(cursor + 4)),
        }

        cursor = cursor.wrapping_add(RELOCATION_SIZE);
    }
}

/// Removes the relocation effects that [`relocate`] applied to `importer`.
///
/// This is the SDK's static `Undo(newModule, module)`. It follows the same
/// import table and stream as [`relocate`], but:
/// * `ADDR32`/`ADDR16*` fields are zeroed;
/// * `ADDR24`/`ADDR14*`/`REL14*` fields keep their opcode bits and lose the
///   address/displacement bits;
/// * `REL24` is pointed at the importer's `unresolved` handler when the header
///   has an `unresolvedSection`, otherwise its 24-bit field is cleared.
///
/// # Original
/// - Ghidra: `FUN_801b1f50` at `0x801b1f50` (576 bytes), the SDK's `Undo`.
/// - Evidence: `801b1f50_FUN_801b1f50.c`; raw words at
///   `0x801b1fb4..0x801b218c`.
///
/// # Arguments
/// * `provider` - module that supplied the symbols (non-zero).
/// * `importer` - linked module whose patches are removed.
///
/// # Returns
/// `true` when the stream reached `R_DOLPHIN_END`; `false` when no import
/// entry matched.
///
/// # Side effects
/// Clears or rewrites fields in the importer; flushes executable sections.
///
/// # Safety
/// `provider` must be non-zero and both pointers must reference linked modules.
/// A malformed stream can write at an arbitrary address.
///
/// # Confidence
/// high for the field clearing and the REL24 special case; the arg order is the
/// same as [`relocate`] and was confirmed from the raw words.
pub unsafe fn undo(provider: usize, importer: usize) -> bool {
    let provider_id = read_u32(provider + MODULE_ID);

    let table = read_u32(importer + MODULE_IMP_OFFSET) as usize;
    let end = table.wrapping_add(read_u32(importer + MODULE_IMP_SIZE) as usize);
    let Some(entry) = find_import_entry(table, end, provider_id) else {
        return false;
    };

    fn_801b1930();

    let mut cursor = read_u32(entry + 4) as usize;
    let mut place: usize = 0;
    let mut previous_section: usize = 0;

    loop {
        let reloc_type = read_u8(cursor + 2);

        if reloc_type == R_DOLPHIN_END {
            if previous_section != 0 {
                flush_section(previous_section);
            }
            fn_801b1940();
            return true;
        }

        place = place.wrapping_add(read_u16(cursor) as usize);

        match reloc_type {
            R_DOLPHIN_NOP => {}
            R_DOLPHIN_SECTION => {
                let index = read_u8(cursor + 3);
                let entry = section_entry(importer, index);
                let raw = read_u32(entry);
                place = (raw & !1) as usize;

                if previous_section != 0 {
                    flush_section(previous_section);
                }

                previous_section = if raw & 1 != 0 { entry } else { 0 };
            }
            _ => undo_relocation(reloc_type, place, importer),
        }

        cursor = cursor.wrapping_add(RELOCATION_SIZE);
    }
}

/// Reverses one non-control relocation record.
///
/// # Original
/// - Inline switch in `FUN_801b1f50` at `0x801b1fd4..0x801b2130`.
/// - Evidence: `801b1f50_FUN_801b1f50.c` lines 44-107. The `REL24` arm reads
///   `unresolvedSection` (+0x32) and `unresolved` (+0x3c) at
///   `0x801b2098..0x801b20b8`.
///
/// # Arguments
/// * `reloc_type` - record type byte.
/// * `place` - address to clear or rewrite.
/// * `importer` - linked module that owns `place` (for the `REL24` target).
///
/// # Side effects
/// Writes 2 or 4 bytes; unknown types only report.
///
/// # Safety
/// `place` must be writable for the width implied by `reloc_type`.
unsafe fn undo_relocation(reloc_type: u8, place: usize, importer: usize) {
    match reloc_type {
        R_PPC_NONE => {}
        R_PPC_ADDR32 => write_u32(place, 0),
        R_PPC_ADDR24 => {
            let old = read_u32(place);
            write_u32(place, old & 0xfc00_0003);
        }
        R_PPC_ADDR16 | R_PPC_ADDR16_LO | R_PPC_ADDR16_HI | R_PPC_ADDR16_HA => {
            write_u16(place, 0)
        }
        R_PPC_ADDR14 | R_PPC_ADDR14_BRTAKEN | R_PPC_ADDR14_BRNTAKEN => {
            let old = read_u32(place);
            write_u32(place, old & 0xffff_0003);
        }
        R_PPC_REL24 => {
            let old = read_u32(place);
            let displacement = if read_u8(importer + MODULE_UNRESOLVED_SECTION) != 0 {
                read_u32(importer + MODULE_UNRESOLVED).wrapping_sub(place as u32)
            } else {
                0
            };
            write_u32(place, (displacement & 0x03ff_fffc) | (old & 0xfc00_0003));
        }
        R_PPC_REL14 | R_PPC_REL14_BRTAKEN | R_PPC_REL14_BRNTAKEN => {
            let old = read_u32(place);
            write_u32(place, old & 0xffff_0003);
        }
        _ => report_unknown(STR_OSUNLINK_UNKNOWN, reloc_type),
    }
}

// ---------------------------------------------------------------------------
// Module list / header fixups
// ---------------------------------------------------------------------------

/// Adds `base` to one of the three section-relative entry-point fields.
///
/// # Original
/// - Inline in `FUN_801b1c30` at `0x801b1da4..0x801b1e18`; the same subtraction
///   appears in `FUN_801b2190`.
///
/// # Safety
/// `module` must be linked and `section_field` must hold a valid section index
/// or zero.
unsafe fn add_section_base(module: usize, section_field: usize, offset_field: usize) {
    let index = read_u8(module + section_field);
    if index == 0 {
        return;
    }
    let value = read_u32(module + offset_field).wrapping_add(section_address(module, index));
    write_u32(module + offset_field, value);
}

/// Subtracts `base` from one of the three section-relative entry-point fields.
///
/// # Original
/// - Inline in `FUN_801b2190` at `0x801b2228..0x801b229c`.
///
/// # Safety
/// `module` must be linked and `section_field` must hold a valid section index
/// or zero.
unsafe fn sub_section_base(module: usize, section_field: usize, offset_field: usize) {
    let index = read_u8(module + section_field);
    if index == 0 {
        return;
    }
    let value = read_u32(module + offset_field).wrapping_sub(section_address(module, index));
    write_u32(module + offset_field, value);
}

// ---------------------------------------------------------------------------
// Public linker entry points
// ---------------------------------------------------------------------------

/// Links a placed REL module: validates it, registers it, fixes every header
/// offset to an absolute address, applies all relocations, and zeroes BSS.
///
/// This is the SDK's static `Link` routine shared by [`os_link`] and
/// [`os_link_fixed`]. The `fixed` flag selects the `OSLinkFixed` behaviour:
/// after all relocations are applied, cut `impSize` at the first import entry
/// whose module id is 0 or the module's own id, so the tail of the import table
/// (and the `fixSize` region after it) can be reused.
///
/// # Original
/// - Ghidra: `FUN_801b1c30` at `0x801b1c30` (740 bytes)
/// - Evidence: `801b1c30_FUN_801b1c30.c`; raw words at
///   `0x801b1c30..0x801b1f14`. The version/alignment gates are at
///   `0x801b1c5c..0x801b1cac`, the list insert at `0x801b1cb4..0x801b1ce0`,
///   and the optional import cut at `0x801b1e7c..0x801b1ed8`.
///
/// # Arguments
/// * `module` - address where the caller placed the complete REL image. This
///   is the module's runtime base; the REL stores no base of its own.
/// * `bss` - address of the zero-initialised buffer for the BSS section.
/// * `fixed` - `true` for `OSLinkFixed` (cut the import table), `false` for
///   ordinary `OSLink`.
///
/// # Returns
/// `true` on success. `false` when the version is greater than 3, or when the
/// version is 2/3 and the module or BSS address violates the header alignment.
///
/// # Side effects
/// Appends to the global module list, rewrites header and section/import table
/// entries, patches relocated code, flushes caches, and zeroes `bssSize` bytes
/// at `bss`.
///
/// # Safety
/// `module` must point to a mapped REL image whose header and tables are
/// readable; `bss` must be writable for `bssSize` bytes. The section bytes must
/// already be at `module + file_offset`; this function does not copy them.
///
/// # Confidence
/// high - every step is visible in the Ghidra body and the raw words. The name
/// `Link` is the SDK's static routine; `link` is used here to avoid a keyword
/// clash with Rust concepts.
pub unsafe fn link(module: usize, bss: usize, fixed: bool) -> bool {
    // The loader clears bssSection before it validates the header, so a failed
    // link still leaves this byte zero. Preserve that ordering.
    write_u8(module + MODULE_BSS_SECTION, 0);

    let version = read_u32(module + MODULE_VERSION);
    if version > 3 {
        return false;
    }

    // Versions 0 and 1 have no alignment fields; versions 2 and 3 do. The
    // original divides by the raw header value, so treat them as byte counts.
    if version >= 2 {
        let align = read_u32(module + MODULE_ALIGN);
        if align != 0 && (module as u32) % align != 0 {
            return false;
        }
        let bss_align = read_u32(module + MODULE_BSS_ALIGN);
        if bss_align != 0 && (bss as u32) % bss_align != 0 {
            return false;
        }
    }

    // Tail-append to __OSModuleInfoList.
    let tail = read_u32(MODULE_LIST_TAIL) as usize;
    if tail == 0 {
        write_u32(MODULE_LIST_HEAD, module as u32);
    } else {
        write_u32(tail + MODULE_NEXT, module as u32);
    }
    write_u32(module + MODULE_PREV, tail as u32);
    write_u32(module + MODULE_NEXT, 0);
    write_u32(MODULE_LIST_TAIL, module as u32);

    // Turn the three header file offsets into absolute pointers.
    let section_info = read_u32(module + MODULE_SECTION_INFO).wrapping_add(module as u32);
    let rel_offset = read_u32(module + MODULE_REL_OFFSET).wrapping_add(module as u32);
    let imp_offset = read_u32(module + MODULE_IMP_OFFSET).wrapping_add(module as u32);
    write_u32(module + MODULE_SECTION_INFO, section_info);
    write_u32(module + MODULE_REL_OFFSET, rel_offset);
    write_u32(module + MODULE_IMP_OFFSET, imp_offset);

    if version >= 3 {
        let fix_size = read_u32(module + MODULE_FIX_SIZE).wrapping_add(module as u32);
        write_u32(module + MODULE_FIX_SIZE, fix_size);
    }

    // Section table: absolute offsets, and BSS gets the caller's buffer.
    let num_sections = read_u32(module + MODULE_NUM_SECTIONS);
    let table = section_info as usize;
    let mut i = 1u32;
    while i < num_sections {
        let entry = table + i as usize * SECTION_ENTRY_SIZE;
        let raw = read_u32(entry);
        if raw != 0 {
            write_u32(entry, raw.wrapping_add(module as u32));
        } else {
            let size = read_u32(entry + 4);
            if size != 0 {
                write_u8(module + MODULE_BSS_SECTION, i as u8);
                write_u32(entry, bss as u32);
            }
        }
        i += 1;
    }

    // Import table: each relocation-stream offset becomes absolute.
    let mut entry = imp_offset as usize;
    let import_end = entry.wrapping_add(read_u32(module + MODULE_IMP_SIZE) as usize);
    while entry < import_end {
        let stream = read_u32(entry + 4).wrapping_add(module as u32);
        write_u32(entry + 4, stream);
        entry = entry.wrapping_add(IMPORT_ENTRY_SIZE);
    }

    // Entry points: prolog/epilog/unresolved are section-relative.
    add_section_base(module, MODULE_PROLOG_SECTION, MODULE_PROLOG);
    add_section_base(module, MODULE_EPILOG_SECTION, MODULE_EPILOG);
    add_section_base(module, MODULE_UNRESOLVED_SECTION, MODULE_UNRESOLVED);

    // Module name: relative to the separately loaded string table.
    let string_table = read_u32(STRING_TABLE_BASE);
    if string_table != 0 {
        let name = read_u32(module + MODULE_NAME_OFFSET).wrapping_add(string_table);
        write_u32(module + MODULE_NAME_OFFSET, name);
    }

    // Relocate against main.dol first, then against every loaded module.
    relocate(0, module);
    let mut iter = read_u32(MODULE_LIST_HEAD) as usize;
    while iter != 0 {
        relocate(module, iter);
        if iter != module {
            relocate(iter, module);
        }
        iter = read_u32(iter + MODULE_NEXT) as usize;
    }

    // OSLinkFixed: drop the self/DOL import entries so the tail of the import
    // table (and the fixSize region) can be reused by the caller.
    if fixed {
        let import_start = read_u32(module + MODULE_IMP_OFFSET) as usize;
        let import_limit = import_start.wrapping_add(read_u32(module + MODULE_IMP_SIZE) as usize);
        let module_id = read_u32(module + MODULE_ID);
        let mut p = import_start;
        while p < import_limit {
            let import_id = read_u32(p);
            if import_id == 0 || import_id == module_id {
                write_u32(module + MODULE_IMP_SIZE, (p - import_start) as u32);
                break;
            }
            p = p.wrapping_add(IMPORT_ENTRY_SIZE);
        }
    }

    let bss_size = read_u32(module + MODULE_BSS_SIZE);
    let _ = memset_wrapper(bss, 0, bss_size);

    fn_801b1910();
    true
}

/// Public `OSLink`: links a module at a caller-chosen address.
///
/// The 8-byte body at `0x801b1f20` loads `r5 = 0` and tail-branches to
/// [`link`]; the return value is that call's result.
///
/// # Original
/// - Ghidra: `FUN_801b1f20` at `0x801b1f20` (8 bytes), the SDK's `OSLink`.
/// - Evidence: `801b1f20_FUN_801b1f20.c`; raw words
///   `0x38a00000` (`li r5,0`) and `0x4bfffd0c` (tail `b 0x801b1c30`).
///
/// # Arguments
/// * `module` - address of the placed REL image (its runtime base).
/// * `bss` - writable BSS buffer.
///
/// # Returns
/// See [`link`].
///
/// # Side effects
/// See [`link`] with `fixed = false`.
///
/// # Safety
/// See [`link`].
///
/// # Confidence
/// high - the wrapper is unambiguous; the SDK API name is `OSLink`.
pub unsafe fn os_link(module: usize, bss: usize) -> bool {
    link(module, bss, false)
}

/// Public `OSLinkFixed`: links a version-3 module and frees its import tail.
///
/// The body checks `version == 3` and tail-calls [`link`] with `fixed = true`;
/// every other version returns `false` without touching the module. This is the
/// entry point that makes the header's `fixSize` region reusable.
///
/// # Original
/// - Ghidra: `FUN_801b1f30` at `0x801b1f30` (28 bytes)
/// - Evidence: `801b1f30_FUN_801b1f30.c`; raw words at `0x801b1f30..0x801b1f48`
///   (`lwz r0,0x1c(r3); cmplwi r0,3; bne ...; li r5,1; b 0x801b1c30`).
///
/// # Arguments
/// * `module` - address of the placed version-3 REL image.
/// * `bss` - writable BSS buffer.
///
/// # Returns
/// `true` on success, `false` when the version is not 3 or [`link`] fails.
///
/// # Side effects
/// See [`link`] with `fixed = true`.
///
/// # Safety
/// See [`link`].
///
/// # Confidence
/// high for the body; medium for the symbol spelling, which comes from
/// `docs/REL2ELF.md` ("`fixSize` - OSLinkFixed reserve boundary") and the
/// SDK's documented `BOOL OSLinkFixed(OSModuleInfo*, void*)`.
pub unsafe fn os_link_fixed(module: usize, bss: usize) -> bool {
    if read_u32(module + MODULE_VERSION) == 3 {
        link(module, bss, true)
    } else {
        false
    }
}

/// Public `OSUnlink`: removes a module from the list and undoes its relocations.
///
/// Steps, in order:
/// 1. Unlink the module from `__OSModuleInfoList` (head `0x800030c8`, tail
///    `0x800030cc`).
/// 2. Walk the remaining modules and call [`undo`] for this module against each
///    one.
/// 3. Call the unlink-notification hook.
/// 4. Subtract the string-table base from `nameOffset`.
/// 5. Subtract the section address from `prolog`/`epilog`/`unresolved`.
/// 6. Subtract the module base from every import-stream offset.
/// 7. Clear the BSS entry and subtract the module base from every non-zero
///    section offset.
/// 8. Subtract the module base from `relOffset`, `impOffset`, and
///    `sectionInfoOffset`, restoring the file-relative header.
///
/// The function always returns `true`.
///
/// # Original
/// - Ghidra: `FUN_801b2190` at `0x801b2190` (464 bytes), the SDK's `OSUnlink`.
/// - Evidence: `801b2190_FUN_801b2190.c`; raw words at
///   `0x801b2190..0x801b235c`. The list removal is
///   `next == 0 ? tail = prev : next->prev = prev` plus the mirror case.
///
/// # Arguments
/// * `module` - a module previously linked by [`os_link`] or
///   [`os_link_fixed`].
///
/// # Returns
/// Always `true`.
///
/// # Side effects
/// Removes the module from the global list, clears/rewrites relocated fields,
/// flushes executable sections, and restores the header's relative offsets.
///
/// # Safety
/// `module` must be currently linked and its neighbours consistent. Undoing a
/// module twice corrupts the list and the header.
///
/// # Confidence
/// high - the body matches the SDK `DEQUEUE_INFO` macro and the inverse of
/// [`link`] step for step.
pub unsafe fn os_unlink(module: usize) -> bool {
    let next = read_u32(module + MODULE_NEXT) as usize;
    let prev = read_u32(module + MODULE_PREV) as usize;

    // DEQUEUE_INFO: fix the two neighbours (or the head/tail) first.
    if next == 0 {
        write_u32(MODULE_LIST_TAIL, prev as u32);
    } else {
        write_u32(next + MODULE_PREV, prev as u32);
    }
    if prev == 0 {
        write_u32(MODULE_LIST_HEAD, next as u32);
    } else {
        write_u32(prev + MODULE_NEXT, next as u32);
    }

    // The module is already out of the list, so this walks only the modules
    // that remain and removes this module's patches from each of them.
    let mut iter = read_u32(MODULE_LIST_HEAD) as usize;
    while iter != 0 {
        undo(module, iter);
        iter = read_u32(iter + MODULE_NEXT) as usize;
    }

    fn_801b1920();

    let string_table = read_u32(STRING_TABLE_BASE);
    if string_table != 0 {
        let name = read_u32(module + MODULE_NAME_OFFSET).wrapping_sub(string_table);
        write_u32(module + MODULE_NAME_OFFSET, name);
    }

    sub_section_base(module, MODULE_PROLOG_SECTION, MODULE_PROLOG);
    sub_section_base(module, MODULE_EPILOG_SECTION, MODULE_EPILOG);
    sub_section_base(module, MODULE_UNRESOLVED_SECTION, MODULE_UNRESOLVED);

    // Import streams: absolute -> file-relative again.
    let mut entry = read_u32(module + MODULE_IMP_OFFSET) as usize;
    let import_end = entry.wrapping_add(read_u32(module + MODULE_IMP_SIZE) as usize);
    while entry < import_end {
        let stream = read_u32(entry + 4).wrapping_sub(module as u32);
        write_u32(entry + 4, stream);
        entry = entry.wrapping_add(IMPORT_ENTRY_SIZE);
    }

    // Section table: undo the base, and forget the runtime BSS placement.
    let table = read_u32(module + MODULE_SECTION_INFO) as usize;
    let num_sections = read_u32(module + MODULE_NUM_SECTIONS);
    let bss_section = read_u8(module + MODULE_BSS_SECTION);
    let mut i = 1u32;
    while i < num_sections {
        let entry = table + i as usize * SECTION_ENTRY_SIZE;
        if i as u8 == bss_section {
            write_u8(module + MODULE_BSS_SECTION, 0);
            write_u32(entry, 0);
        } else {
            let raw = read_u32(entry);
            if raw != 0 {
                write_u32(entry, raw.wrapping_sub(module as u32));
            }
        }
        i += 1;
    }

    let rel_offset = read_u32(module + MODULE_REL_OFFSET).wrapping_sub(module as u32);
    let imp_offset = read_u32(module + MODULE_IMP_OFFSET).wrapping_sub(module as u32);
    let section_info = read_u32(module + MODULE_SECTION_INFO).wrapping_sub(module as u32);
    write_u32(module + MODULE_REL_OFFSET, rel_offset);
    write_u32(module + MODULE_IMP_OFFSET, imp_offset);
    write_u32(module + MODULE_SECTION_INFO, section_info);

    true
}

/// Clears the global module list and the string-table base.
///
/// This is the SDK's `__OSModuleInit`. It runs before any REL is linked, so the
/// linker starts from an empty list and no string table. The write order is
/// tail, head, string table; the list head is at `0x800030c8`, the tail at
/// `0x800030cc`, and the string-table base at `0x800030d0`.
///
/// # Original
/// - Ghidra: `FUN_801b2370` at `0x801b2370` (24 bytes), the SDK's
///   `__OSModuleInit`.
/// - Evidence: `801b2370_FUN_801b2370.c` (`_DAT_800030cc = 0;
///   _DAT_800030c8 = 0; _DAT_800030d0 = 0`).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears three OS globals. Any linked module becomes unreachable through the
/// list, so this is only safe at OS initialisation time.
///
/// # Safety
/// Must run before the first link and while no module list is in use.
///
/// # Confidence
/// high - the three stores are the whole function.
pub unsafe fn os_module_init() {
    write_u32(MODULE_LIST_TAIL, 0);
    write_u32(MODULE_LIST_HEAD, 0);
    write_u32(STRING_TABLE_BASE, 0);
}

// ---------------------------------------------------------------------------
// Real-base recovery
// ---------------------------------------------------------------------------

/// Recovers a linked module's runtime base from its section-table pointer.
///
/// The REL header has no base field, so the base is not stored anywhere by the
/// format. [`link`] turns `sectionInfoOffset` into an absolute pointer; all
/// four NSMBW RELs place their section table immediately after the 0x4c-byte
/// header, so an already-linked module satisfies
/// `base = sectionInfoOffset - 0x4c`.
///
/// This is the only statically checkable way to recover the base from a live
/// module without the original file. It cannot recover a base from an unlinked
/// REL image (whose `sectionInfoOffset` is still `0x4c`, giving base 0) and it
/// does not apply to a REL that stores its table at a different file offset.
///
/// # Original
/// - Derived, not an original function. The 0x4c header size is the end of the
///   `fixSize` field at `0x48`; `link` adds the base to `sectionInfoOffset` at
///   `0x801b1ce8..0x801b1d00`.
/// - Evidence: all four headers in `work/rels/*.rel` report
///   `sectionInfoOffset = 0x4c` (checked with the REL header reader), and
///   `docs/REL.md` lists the same four modules.
///
/// # Arguments
/// * `module` - a module that has been through [`link`].
///
/// # Returns
/// The base if `sectionInfoOffset - 0x4c` is a plausible MEM1/MEM2 address,
/// 16-byte aligned (the shipped `align` value), with a non-zero module id and a
/// sane section count; otherwise `None`.
///
/// # Side effects
/// Reads the candidate header only; writes nothing.
///
/// # Safety
/// `module` must be readable for the header (0x4c bytes). The candidate base is
/// also read, so it must be mapped; the sanity checks cannot prevent a fault on
/// a wild pointer.
///
/// # Confidence
/// medium - the 0x4c placement is verified for the four shipped RELs, but it is
/// a property of those files, not a guarantee of the general format.
pub unsafe fn recover_base(module: usize) -> Option<usize> {
    let section_info = read_u32(module + MODULE_SECTION_INFO) as usize;
    if section_info < MODULE_HEADER_SIZE {
        return None;
    }
    let base = section_info - MODULE_HEADER_SIZE;

    // Real modules live in MEM1 (0x80000000+) or MEM2 (0x90000000+); the four
    // shipped files use align = 16, which the loader checks as a byte multiple.
    if base < 0x8000_0000 || base & 0xf != 0 {
        return None;
    }

    let id = read_u32(base + MODULE_ID);
    let num_sections = read_u32(base + MODULE_NUM_SECTIONS);
    if id == 0 || num_sections == 0 || num_sections > 64 {
        return None;
    }

    Some(base)
}
