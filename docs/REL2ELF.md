# REL to ELF adapter

## In plain English

A Wii **REL** is a relocatable module, roughly the console's equivalent of a
DLL. The game's loader decides where to place it in memory at runtime; that
load address is **not stored in the REL**. The REL instead contains:

- a section table (code, initialized data, and a zero-initialized BSS area),
- an import table that says which module each relocation list refers to,
- delta-encoded relocation lists that say which 32-bit or 16-bit fields must
  be fixed up once a load address is known.

`tools/rel2elf.mjs` does not run the game or reproduce the real loader. It
chooses a fixed, documented **analysis address** for each of the four NSMBW
RELs, applies every relocation against those addresses, and writes an ELF32
big-endian `EM_PPC` executable image. The image is intended for static
analysis in Ghidra (or another ELF reader). It is structurally loadable, but
the addresses are synthetic and are not runtime addresses.

In `--all` mode the converter also writes `work/rels/elf/summary.json`, which
records the section layout, program headers, relocation counts, failures, and
validation results for every module.

## Quick start

```powershell
# Convert all four modules and write summary.json
node tools/rel2elf.mjs --all

# --all is the default when no arguments are supplied
node tools/rel2elf.mjs

# Convert one module; the default output is work/rels/elf/<name>.elf
node tools/rel2elf.mjs work/rels/d_profileNP.rel

# Convert one module to an explicit path
node tools/rel2elf.mjs work/rels/d_profileNP.rel work/rels/elf/custom.elf
```

Requirements: Node 24, plain ESM, no third-party packages. The script resolves
the workspace root from its own location, so it can be started from any
directory. Exit status is 0 when every conversion and validation passes; it is
1 if a module fails or an ELF validation check fails. A single-module run
prints that module's JSON summary; `--all` prints and writes the combined
summary.

## Canonical analysis bases

These are the slots defined in `docs/REL.md`. They are deliberately spaced far
apart so the four images can be imported together without overlapping memory.
They are **not** the addresses the NSMBW loader would choose.

| Module | REL module ID | Canonical base | Address range used by the emitted image |
|---|---:|---:|---:|
| `d_profileNP` | 1 | `0x80500000` | `0x805000E0` .. `0x80503B08` |
| `d_basesNP` | 2 | `0x80600000` | `0x806000F0` .. `0x80834D84` |
| `d_enemiesNP` | 3 | `0x80A00000` | `0x80A000F0` .. `0x80B799E0` |
| `d_en_bossNP` | 4 | `0x80C00000` | `0x80C000F0` .. `0x80C7117C` |
| `main.dol` | 0 | `0x80004000` (context only) | imports use absolute addends, not this base |

Module 0 is special: a relocation against module 0 already contains an
absolute `main.dol` address in its addend, so the converter does not add
`0x80004000`. The main.dol base is listed only because it is the fixed base in
the workspace documentation.

## What the converter does, step by step

1. **Read the REL header** at offsets `0x00` through `0x48` and the section
   table at `sectionInfoOffset`.
2. **Clear the executable bit** from each section's 32-bit file offset. The
   low bit is the executable flag; the upper bits are the file offset.
3. **Find BSS.** A section table entry whose offset is zero and whose size is
   greater than zero is the uninitialized BSS section. Its size comes from the
   header's `bssSize`, not from the section table, and it is placed after all
   initialized sections (see below). Section zero is the mandatory empty entry
   and is skipped; any section with size zero is skipped.
4. **Assign canonical virtual addresses.** For every initialized section,
   `VA = canonical base + section file offset`. BSS is the exception: it is
   placed at `align_up(max_end_of_initialized_sections, 1 << bssAlign)`.
   `bssAlign` is an exponent, so the observed value 8 means 256-byte alignment,
   not 8-byte alignment.
5. **Emit PT_LOAD headers.** Each non-empty initialized section becomes one
   `PT_LOAD` segment: executable sections get read+execute (`5`), data sections
   get read+write (`6`), and BSS gets a read+write `PT_LOAD` with
   `p_filesz = 0` and `p_memsz = bssSize`. Section 0 and empty sections do not
   produce segments. There is no PT_LOAD overlap in either file or memory.
6. **Set the entry point.** `e_entry` is
   `base + section[prologSection].offset + prolog`, provided the prolog section
   exists, is executable, is not BSS, and `prolog < section.size`. Otherwise
   the converter falls back to the first executable section and records why.
   All four modules use section 1 and a prolog offset of zero.
7. **Apply relocations** while walking each import list. Details and the type
   table follow below.
8. **Re-parse the emitted ELF** and validate its header, program headers,
   section coverage, entry point, alignment congruence, and overlap rules.
   The results are written into the summary and printed.

### BSS layout used here

`max_end_of_initialized_sections` is the largest
`section offset + section size` among initialized sections. `1 << bssAlign`
is 256 for all four modules.

| Module | End of initialized sections | BSS offset from base | BSS virtual address | BSS size |
|---|---:|---:|---:|---:|
| `d_profileNP` | `0x3884` | `0x3900` | `0x80503900` | `0x208` |
| `d_basesNP` | `0x2228B8` | `0x222900` | `0x80822900` | `0x12484` |
| `d_enemiesNP` | `0x16E438` | `0x16E500` | `0x80B6E500` | `0xB4E0` |
| `d_en_bossNP` | `0x6C8A8` | `0x6C900` | `0x80C6C900` | `0x487C` |

## REL fields, one by one

### Header fields

Offsets are from the start of the REL. All values are big-endian 32-bit unless
noted.

| Offset | Field | Meaning | How the converter uses it |
|---:|---|---|---|
| `0x00` | `id` | Module ID; unique within a game, never 0 in a valid module. | Selects the canonical base (1..4). |
| `0x04` | `next` | Runtime pointer to the next loaded module. | Ignored; not a file offset. |
| `0x08` | `prev` | Runtime pointer to the previous loaded module. | Ignored; not a file offset. |
| `0x0C` | `numSections` | Number of section table entries. | Bounds the section table parse. |
| `0x10` | `sectionInfoOffset` | File offset of the section table. | Read and used. |
| `0x14` | `nameOffset` | Offset into the separate `.str` debug file. | Ignored; the embedded name is unavailable. |
| `0x18` | `nameSize` | Length in the separate `.str` file. | Ignored for the same reason. |
| `0x1C` | `version` | REL format version. | Recorded only; observed value is 3. |
| `0x20` | `bssSize` | Size of the uninitialized BSS area in bytes. | Used for BSS `p_memsz`, BSS placement, and BSS symbol bounds. |
| `0x24` | `relOffset` | File offset of the first relocation list. | Recorded for provenance; streams are found through the import table. |
| `0x28` | `impOffset` | File offset of the import table. | Read and used. |
| `0x2C` | `impSize` | Byte size of the import table. | Read and used. |
| `0x30` | `prologSection` (u8) | Section index for the module entry point. | Used for `e_entry`. |
| `0x31` | `epilogSection` (u8) | Section index for the module unload entry. | Ignored; not the ELF entry point. |
| `0x32` | `unresolvedSection` (u8) | Section index for the unresolved-function handler. | Ignored. |
| `0x33` | `bssSection` (u8) | Section index of BSS, filled in by the runtime loader. | Ignored because it is zero/not reliable in these files; BSS is detected by `offset == 0 && size > 0`. |
| `0x34` | `prolog` | Offset of the entry function within `prologSection`. | Added to the section VA to form `e_entry`. |
| `0x38` | `epilog` | Offset of the unload function. | Ignored. |
| `0x3C` | `unresolved` | Offset of the unresolved-function handler. | Ignored. |
| `0x40` | `align` | Load-address alignment as a power of two. | Ignored; the canonical bases are already chosen and aligned. |
| `0x44` | `bssAlign` | BSS alignment as a power of two. | Used as `1 << bssAlign`; value 8 gives 256 bytes. |
| `0x48` | `fixSize` | OSLinkFixed reserve boundary (version 3). | Recorded only; this converter uses ordinary OSLink semantics. |

### Section table entry (8 bytes)

| Offset | Field | Meaning |
|---:|---|---|
| `0x0` | `rawOffset` (u32) | Bit 0 is the executable flag. Bits 1..31 are the file offset. The converter clears bit 0 before using the offset. |
| `0x4` | `size` (u32) | Section size in bytes. Size zero means the entry is empty and is skipped. |

A non-zero size with a zero file offset denotes BSS. The first table entry is
the mandatory empty section zero. In these four modules the BSS section is
index 6; its table size equals the header `bssSize` (the tool warns if a future
file disagrees, and uses the header value).

### Import table entry (8 bytes)

| Offset | Field | Meaning |
|---:|---|---|
| `0x0` | `moduleId` (u32) | 0 means `main.dol`; 1..4 mean the corresponding REL module. |
| `0x4` | `relOffset` (u32) | File offset of the first 8-byte relocation entry for this module. |

### Relocation entry (8 bytes)

| Offset | Field | Meaning |
|---:|---|---|
| `0x0` | `delta` (u16) | Number of bytes from the previous relocation to this one. After a section switch (`202`) it is relative to the new section's start. |
| `0x2` | `type` (u8) | Relocation type or control code. See the type table. |
| `0x3` | `section` (u8) | For normal relocations, the **symbol's** section in the module named by the import entry. For type `202`, this is the **target** section in the module being converted. |
| `0x4` | `addend` (u32) | Symbol offset from the start of its section. For module 0 imports this is already an absolute `main.dol` address. |

The running cursor is per relocation list. A normal entry adds `delta` to the
cursor, resolves the symbol, and writes at
`current module base + target section offset + cursor`. Type `201` adds its
delta without writing, which is how a list skips a gap larger than the 16-bit
delta limit; the observed value `0xFFFF` is simply the largest u16 delta.
Type `202` switches the target section and resets the cursor to zero; its
delta field is ignored (all observed values are zero). Type `203` ends the
list.

## Relocation types

Let:

- `S` = symbol address after resolution,
- `P` = address of the field being relocated (the target address),
- `A` = the entry's addend.

For module 0 imports, `S = A` (the addend is an absolute `main.dol` address).
For REL modules 1..4, `S = target module canonical base + symbol section
canonical offset + A`. If the symbol section is BSS, the canonical offset is
the computed BSS offset, not zero.

The emitted values are:

| Type | Name | Width | Value written | Observed in these modules |
|---:|---|---:|---|---:|
| 1 | `R_PPC_ADDR32` | 4 | `S` | 91,863 |
| 2 | `R_PPC_ADDR24` | 4 | `S` masked to the 24-bit instruction field (`S & 0x03FFFFFC`), preserving the other instruction bits | 0 |
| 3 | `R_PPC_ADDR16` | 2 | low 16 bits of `S`; fit-checked to 16 bits | 0 |
| 4 | `R_PPC_ADDR16_LO` | 2 | `S & 0xFFFF` | 30,772 |
| 5 | `R_PPC_ADDR16_HI` | 2 | `(S >> 16) & 0xFFFF` | 0 |
| 6 | `R_PPC_ADDR16_HA` | 2 | `((S + 0x8000) >> 16) & 0xFFFF`; the `0x8000` compensates for the sign extension of the low half in paired `addi`/`ori` code | 27,246 |
| 7-9 | `R_PPC_ADDR14`, `_BRTAKEN`, `_BRNTAKEN` | 4 | `S` masked to the 14-bit instruction field (`S & 0x0000FFFC`), preserving the other instruction bits | 0 |
| 10 | `R_PPC_REL24` | 4 | `(S - P)` masked to the 24-bit branch field (`& 0x03FFFFFC`), preserving opcode/AA/LK bits | 48,575 |
| 11-13 | `R_PPC_REL14`, `_BRTAKEN`, `_BRNTAKEN` | 4 | `(S - P)` masked to the 14-bit branch field (`& 0x0000FFFC`), preserving opcode/AA/LK bits | 0 |
| 0 | `R_PPC_NONE` | 0 | no write; the delta is still accumulated | 0 |
| 201 | `R_RVL_NONE` | 0 | no write; add `delta` to the running cursor | 56 |
| 202 | `R_RVL_SECT` | 0 | set target section from the `section` byte; reset cursor to zero | 38 |
| 203 | `R_RVL_STOP` | 0 | stop this relocation list | 15 |
| 204 | `R_DOLPHIN_MRKREF` | - | not implemented; counted as unknown and bytes left unchanged | 0 |

Unknown relocation types are counted in `unknownTypes` and their target bytes
are left unchanged. Their delta still advances the running cursor, so later
entries in the same list keep the offsets the REL intended.

The 24-bit mask `0x03FFFFFC` covers bits 2 through 25 of a PowerPC instruction:
the 24-bit branch/address field. The 14-bit mask `0x0000FFFC` covers bits 2
through 15: the 14-bit conditional-branch field. The corresponding preserved
masks are `0xFC000003` (opcode plus AA and LK bits) and `0xFFFF0003` (everything
outside the 14-bit field). This is why branch relocation does not destroy the
instruction opcode or the link bit.

`R_PPC_ADDR16_HA` is the "high adjusted" value: adding `0x8000` before shifting
rounds up when the low 16 bits are at least `0x8000`, which is what paired
`lis`/`addi` sequences expect.

### Fit checks and failures

The converter checks that every write lies completely inside the target
section and inside the emitted file. For the absolute field types it also
checks the field width:

- type 2 must be word-aligned and no greater than `0x03FFFFFC`,
- type 3 must fit in 16 bits,
- types 7-9 must be word-aligned and no greater than `0x0000FFFC`,
- types 10 and 11-13 must have a displacement in the signed range of their
  24-bit or 14-bit field.

A failed check is counted by type, left unchanged in the image, and recorded
with the import module, stream offset, entry offset, target section, target
offset, and reason. The summary keeps the first 100 failure details and the
exact total failure count.

## Results for the four modules

Run `node tools/rel2elf.mjs --all` to regenerate these files under
`work/rels/elf/`.

### Layout and entry

| Module | ID | Base | Entry | Text VA + size | BSS VA + size | ELF bytes | PT_LOADs |
|---|---:|---:|---:|---|---|---:|---:|
| `d_profileNP` | 1 | `0x80500000` | `0x805000E0` | `0x805000E0` + `0x1FB8` | `0x80503900` + `0x208` | 18,564 | 6 |
| `d_basesNP` | 2 | `0x80600000` | `0x806000F0` | `0x806000F0` + `0x1C6004` | `0x80822900` + `0x12484` | 2,242,744 | 6 |
| `d_enemiesNP` | 3 | `0x80A00000` | `0x80A000F0` | `0x80A000F0` + `0x12A428` | `0x80B6E500` + `0xB4E0` | 1,504,312 | 6 |
| `d_en_bossNP` | 4 | `0x80C00000` | `0x80C000F0` | `0x80C000F0` + `0x5702C` | `0x80C6C900` + `0x487C` | 448,680 | 6 |

Each module has one text segment, four initialized data segments (sections
2-5), and one BSS segment. The exact per-section addresses and file offsets
are in `summary.json`.

### Relocations applied

| Module | Import lists | Entries incl. control | Relocations | Applied | Failed | Unknown | Type 1 ADDR32 | Type 4 LO16 | Type 6 HA16 | Type 10 REL24 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `d_profileNP` | 5 | 1,217 | 1,203 | 1,203 | 0 | 0 | 977 | 56 | 47 | 123 |
| `d_basesNP` | 4 | 97,582 | 97,524 | 97,524 | 0 | 0 | 39,650 | 16,743 | 14,819 | 26,312 |
| `d_enemiesNP` | 3 | 79,648 | 79,624 | 79,624 | 0 | 0 | 42,079 | 10,522 | 9,396 | 17,627 |
| `d_en_bossNP` | 3 | 20,118 | 20,105 | 20,105 | 0 | 0 | 9,157 | 3,451 | 2,984 | 4,513 |
| **Total** | **15** | **198,565** | **198,456** | **198,456** | **0** | **0** | **91,863** | **30,772** | **27,246** | **48,575** |

Control entries: 56 `201`, 38 `202`, 15 `203`; no type 0 entries. Every import
list had a `203` terminator. The only real types encountered were 1, 4, 6, and
10. There were no target-BSS writes, no out-of-bounds writes, and no
address-fit failures.

### Validation

Run `node tools/rel2elf.mjs --all`, then inspect
`work/rels/elf/summary.json`: `validation.ok` is the overall result,
`totals.relocationsFailed` and `totals.relocationsUnknown` must be zero, and
each module has its own `validation.ok`, `validation.errors`,
`validation.memoryOverlaps`, and `validation.fileOverlaps`. The summary also
contains each ELF's SHA-256. All four modules currently report
`validation.ok: true`.

The per-module checks are:

- ELF magic, class 32, big-endian, version 1, `ET_EXEC`, `EM_PPC` (machine 20).
- Program header table within the file; `p_filesz <= p_memsz`; every segment
  within the 32-bit address space and within the file.
- No overlapping `PT_LOAD` memory ranges and no overlapping file ranges.
- File-backed segments satisfy `p_offset` and `p_vaddr` congruence modulo
  `p_align` (4 KiB).
- `e_entry` is inside a file-backed executable segment; a non-word-aligned
  entry is reported as a warning rather than a hard error.
- Every non-empty REL section is covered by exactly one `PT_LOAD`, with the
  expected flags; BSS has `p_filesz = 0` and the expected `p_memsz`.
- The canonical ranges of the four modules do not overlap each other.

In addition, an independent byte-level audit recomputed every one of the
198,456 patched fields from the REL streams and compared it with the emitted
ELF, including the preservation of opcode/AA/LK bits for type 10. All 198,456
matched.

## Emitted ELF fields

### ELF header

| Field | Value | Reason |
|---|---|---|
| `e_ident` class/data/version | 1 / 2 / 1 | ELF32, big-endian, current version. |
| `e_type` | 2 | `ET_EXEC`; a fixed-address image, not a relocatable object. |
| `e_machine` | 20 | `EM_PPC`; the Wii CPU is PowerPC. |
| `e_version` | 1 | ELF version 1. |
| `e_entry` | canonical entry VA | Computed from `prologSection` and `prolog`; see above. |
| `e_phoff` | 52 | The ELF32 header is 52 bytes (`0x34`), so the first program header follows it. |
| `e_shoff` | 0 | No section header table is emitted. |
| `e_flags` | 0 | No PowerPC ELF flags are needed for this static image. |
| `e_ehsize` | 52 | ELF32 header size. |
| `e_phentsize` | 32 | ELF32 program header size. |
| `e_phnum` | 6 | One text + four data + one BSS segment per module. |
| `e_shentsize` | 40 | Standard ELF32 section header size; `e_shnum` is zero, so no section headers are present. |
| `e_shnum` | 0 | Nothing to describe with section headers. |
| `e_shstrndx` | 0 | No section-name string table. |

### PT_LOAD program headers

| Field | Value | Reason |
|---|---|---|
| `p_type` | 1 | `PT_LOAD`. |
| `p_offset` | File offset, page-congruent with `p_vaddr` | File layout is an implementation detail; it is not a virtual address. |
| `p_vaddr` / `p_paddr` | Canonical section VA | Both are set to the same analysis address. |
| `p_filesz` | Section size, or 0 for BSS | BSS has no bytes in the file. |
| `p_memsz` | Section size, or `bssSize` for BSS | In-memory extent. |
| `p_flags` | 5 for text, 6 for data and BSS | read+execute (`PF_R` and `PF_X`) = 5; read+write (`PF_R` and `PF_W`) = 6. |
| `p_align` | `0x1000` | 4 KiB. File offsets are chosen so `p_offset` and `p_vaddr` are congruent modulo this value, as ELF loaders expect. |

No section headers, symbol table, dynamic section, or debug information are
emitted. Ghidra reads the `PT_LOAD` segments directly, as it does for the
existing `main.elf` conversion.

## Unresolved types and limitations

- **No unresolved relocation types in these four files.** Types 2, 3, 5, 7-9,
  and 11-13 are implemented from the PowerPC ELF ABI but were not exercised by
  the observed streams; their fit rules are documented above. Any other type,
  including 204, is counted in `unknownTypes` and left unchanged.
- **Synthetic bases are not runtime addresses.** Cross-module pointers and
  branch displacements are correct only for the canonical slots in this
  document. Do not present them as addresses used by the retail game. If the
  real loader places a module elsewhere, every cross-module value changes.
- **No symbol names.** The REL has no symbol table in these files. The ELF has
  no `.symtab`, no section headers, and no function names; Ghidra must discover
  functions and references heuristically.
- **main.dol imports are trusted as absolute addresses.** The addend is used
  verbatim. The tool only applies a rough sanity warning if a non-zero addend
  falls outside a wide window around the known main.dol image; it does not
  validate the address against `main.elf` or a main.dol section map.
- **Cross-module section tables come from the canonical files in `work/rels/`.**
  A different build or region of a REL can move sections and change every
  imported symbol address. Convert matching REL versions together.
- **BSS has no file bytes.** Relocations that target BSS are rejected
  (`target-section-bss`) because there is nowhere to store the patched value in
  a `p_filesz = 0` segment. None occur in these four modules. BSS contents are
  also not initialized by the ELF.
- **The branch-prediction variants are field patches only.** Types 8, 9, 12,
  and 13 are treated as their ADDR14/REL14 counterparts: the 14-bit field is
  written and the rest of the instruction is preserved. No prediction-hint bit
  is synthesized. None of these types occur in these modules.
- **Absolute 24-bit and 14-bit fields are fit-checked.** If an address does not
  fit the field, the relocation is counted as a failure and the bytes are left
  unchanged rather than silently truncated.
- **`bssAlign` is interpreted as a power-of-two exponent.** The observed value
  8 therefore means 256-byte alignment. If another REL stores a direct
  alignment value instead, its BSS placement would need a different rule.
- **No missing terminators in these files.** The converter counts an import
  list with no `203` in `controls.noStopStreams`; none of the four has one.
- **Malformed layouts are rejected rather than guessed.** Multiple BSS-like
  sections, overlapping initialized section ranges, section data outside the
  REL file, and a BSS placement that would overlap initialized data all cause
  an error for that module.
- **One image per module.** The tool does not merge the RELs into a single
  address space. Import all four ELFs into Ghidra with their canonical bases if
  cross-module references need to resolve.
- **Not intended for native execution.** The use of high analysis addresses and
  the absence of a runtime loader mean the ELF is an analysis artifact, not a
  runnable Wii program.
- **Ghidra was not invoked as part of this conversion.** Validation is
  structural and byte-level. The images are shaped for Ghidra's ELF loader but
  were not imported in this task.

## Files produced

| Path | Contents |
|---|---|
| `work/rels/elf/d_profileNP.elf` | ELF32 BE `EM_PPC` image, base `0x80500000`. |
| `work/rels/elf/d_basesNP.elf` | ELF32 BE `EM_PPC` image, base `0x80600000`. |
| `work/rels/elf/d_enemiesNP.elf` | ELF32 BE `EM_PPC` image, base `0x80A00000`. |
| `work/rels/elf/d_en_bossNP.elf` | ELF32 BE `EM_PPC` image, base `0x80C00000`. |
| `work/rels/elf/summary.json` | Per-module layout, segment, relocation, failure, hash, and validation records. |

The summary also includes a SHA-256 for each ELF and the cross-module range
check, so an external consumer can verify the exact artifacts that were
validated.
