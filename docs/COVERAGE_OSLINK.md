# Coverage — `OSLink` and the REL loader (`0x801B1910..0x801B2387`)

## How to read this file

- **Scope** — the `main.dol` cluster that links the four NSMBW REL modules:
  `OSLink`, `OSLinkFixed`, `OSUnlink`, the two relocation engines, the
  string-table setter, the module-list initialiser, and four empty SDK hooks.
  It does **not** cover the `dDyl` layer that allocates the module memory,
  decompresses the LZ11 file, or calls the linker.
- **Addresses** are virtual `main.dol` addresses, not file offsets. The image
  base is `0x80004000`.
- **Evidence** means the Ghidra C under `work/ghidra/main-gb/export/`, plus the
  raw big-endian instruction words read from `work/dol/main.elf` through its
  ELF program headers. The raw words were needed because Ghidra models the
  `__savegprlr_*` helpers as value-returning calls; that obscures which
  physical register is which argument. The bodies and addresses come only from
  the shipped binary. The public API spellings (`OSLink`, `OSLinkFixed`,
  `OSUnlink`, `OSSetStringTable`, `__OSModuleInit`) and the static helper names
  (`Relocate`, `Undo`) are cross-checked against the SDK's documented API, not
  read from a symbol table in this image.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete control flow
  and all calls accounted for; `partial` = entry/exit and major effects; `stub`
  = address/role only.
- **Confidence** is strength of evidence for the translation, not code quality.
  `medium` marks an exact body whose public symbol spelling is inferred.
- **Rust file** for every row is `src/runtime/link.rs`. The ledger companion is
  `ledger/agents/oslink.csv`.

## Summary

| metric | value |
|---|---:|
| functions in cluster | 12 |
| bytes of translated code (`functions.tsv` sizes) | 2,576 |
| address span (with alignment padding) | `0x801b1910..0x801b2387` |
| named entry points | 7 (5 public: `OSLink`, `OSLinkFixed`, `OSUnlink`, `OSSetStringTable`, `__OSModuleInit`; 2 static: `Relocate`, `Undo`) |
| relocation types implemented | 14 (`0..13`) plus 3 control codes |
| relocation types exercised by the four RELs | 4 (`1`, `4`, `6`, `10`) |
| relocations applied by these RELs | 198,456 |
| unknown relocation types in these RELs | 0 |
| empty `blr` hooks | 4 |
| unresolved symbol names | 4 hooks (`fn_801b1910..fn_801b1940`) |

The cluster is small but central: the relocated cross-module pointers and
inter-module branches in `d_basesNP`, `d_enemiesNP`, `d_en_bossNP`, and
`d_profileNP` are produced by `relocate` and removed by `undo`.

## Function table

All evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | Ghidra / SDK name | role | status | confidence | evidence |
|---|---:|---|---|---|---|---|---|
| `0x801B1910` | 4 | `fn_801b1910` | `FUN_801b1910` (probably `OSNotifyLink`) | empty hook at end of `link` | translated | high (body) / low (role) | `801b1910_FUN_801b1910.c`; word `0x4E800020` (`blr`) |
| `0x801B1920` | 4 | `fn_801b1920` | `FUN_801b1920` (probably `OSNotifyUnlink`) | empty hook in `OSUnlink` cleanup | translated | high (body) / low (role) | `801b1920_FUN_801b1920.c`; word `0x4E800020` |
| `0x801B1930` | 4 | `fn_801b1930` | `FUN_801b1930` | empty hook when an import entry matches | translated | high (body) / low (role) | `801b1930_FUN_801b1930.c`; word `0x4E800020` |
| `0x801B1940` | 4 | `fn_801b1940` | `FUN_801b1940` | empty hook at `R_DOLPHIN_END` | translated | high (body) / low (role) | `801b1940_FUN_801b1940.c`; word `0x4E800020` |
| `0x801B1950` | 12 | `os_set_string_table` | `OSSetStringTable` | store the name-string-table base at `0x800030D0` | translated | high | `801b1950_FUN_801b1950.c`; `lis r4,0x8000; stw r3,0x30D0(r4); blr` |
| `0x801B1960` | 708 | `relocate` | `Relocate(newModule, module)` | apply one importer's relocations against one provider | translated | high | `801b1960_FUN_801b1960.c`; raw words `0x801b19d4..0x801b1c20` |
| `0x801B1C30` | 740 | `link` | `Link` (static) | validate, register, fix up, relocate, zero BSS | translated | high | `801b1c30_FUN_801b1c30.c`; raw words `0x801b1c30..0x801b1f14` |
| `0x801B1F20` | 8 | `os_link` | `OSLink(module, bss)` | public link wrapper (`fixed = false`) | translated | high | `801b1f20_FUN_801b1f20.c`; `li r5,0; b 0x801b1c30` |
| `0x801B1F30` | 28 | `os_link_fixed` | `OSLinkFixed(module, bss)` | public version-3 link wrapper (`fixed = true`) | translated | medium (name) / high (body) | `801b1f30_FUN_801b1f30.c`; version check then `li r5,1` |
| `0x801B1F50` | 576 | `undo` | `Undo(newModule, module)` | remove one importer's relocations against one provider | translated | high | `801b1f50_FUN_801b1f50.c`; raw words `0x801b1fb4..0x801b218c` |
| `0x801B2190` | 464 | `os_unlink` | `OSUnlink(module)` | dequeue module, undo relocations, restore header | translated | high | `801b2190_FUN_801b2190.c`; raw words `0x801b2190..0x801b235c` |
| `0x801B2370` | 24 | `os_module_init` | `__OSModuleInit` | clear list head/tail and string-table base | translated | high | `801b2370_FUN_801b2370.c`; three stores to `0x800030CC/C8/D0` |

### Address-order note

`0x801B2370` is not part of the loader body proper. It is included because it
defines the initial state of the three globals that `link` and `os_unlink`
read and write; without it, the list and string-table behaviour is ambiguous.

## Call graph

```text
os_link            (0x801B1F20) ──> link (0x801B1C30)
os_link_fixed      (0x801B1F30) ──> link (0x801B1C30)          [version == 3]
link               (0x801B1C30) ──> relocate (0x801B1960)      [provider 0, then every module]
                               ──> memset_wrapper (0x800046B4) [BSS zero]
                               ──> fn_801b1910
relocate           (0x801B1960) ──> find_import_entry
                               ──> apply_relocation
                               ──> flush_section ──> dc_flush_range  (0x801AC470)
                                                ──> ic_invalidate_range (0x801AC560)
                               ──> fn_801b1930 / fn_801b1940
                               ──> report_unknown ──> 0x8015F730
os_unlink          (0x801B2190) ──> undo (0x801B1F50)         [each remaining module]
                               ──> fn_801b1920
undo               (0x801B1F50) ──> find_import_entry
                               ──> undo_relocation
                               ──> flush_section
                               ──> fn_801b1930 / fn_801b1940
                               ──> report_unknown ──> 0x8015F730
os_set_string_table (0x801B1950)  independent; writes 0x800030D0
os_module_init      (0x801B2370)  independent; clears 0x800030C8/CC/D0
```

## What `OSLink` actually does

### 1. The caller places the module; `OSLink` does not allocate

The REL header has no load-address field. The first argument of `OSLink` is the
address where the caller has already placed the file image. `link` uses that
pointer as the base and adds it to every section offset:

```text
section[i].address = newModule + (section[i].raw_offset & ~1)
```

Therefore:

- `OSLink` does not read the REL file, decompress LZ11, or choose a heap.
- `OSLink` does not copy section bytes. The caller must have copied the file
  image to `newModule` first.
- The only bulk write in this cluster is the BSS zero-fill
  `memset_wrapper(bss, 0, bssSize)` at `0x801B1EDC..0x801B1EEC`.

The section-copy step is outside this cluster. The nearest `dDyl` setup
functions are `0x8008F640`, `0x8008F760`, and `0x8008F870`, but the actual
allocate/decompress/copy path is still untranslated and is not claimed here.

### 2. Validation and registration

`link` first clears `bssSection` (header `+0x33`), then:

| check | condition | result |
|---|---|---|
| version upper bound | `version > 3` | fail |
| module alignment (v2/v3) | `align != 0 && newModule % align != 0` | fail |
| BSS alignment (v2/v3) | `bssAlign != 0 && bss % bssAlign != 0` | fail |

The shipped RELs use `version = 3`, `align = 16`, `bssAlign = 8`. The loader
divides by the raw field, so these are 16-byte and 8-byte byte-multiple checks,
not exponent checks. `docs/REL2ELF.md` describes `bssAlign = 8` as 256-byte
alignment because the converter places BSS on an exponent-derived boundary;
that is a stricter analysis layout, not what `link` checks.

On success the module is appended to `__OSModuleInfoList`:

| global | meaning |
|---|---|
| `0x800030C8` | list head |
| `0x800030CC` | list tail |
| `0x800030D0` | module-name string-table base |

### 3. Header and table fixups

In this order:

1. `sectionInfoOffset` (`+0x10`), `relOffset` (`+0x24`), `impOffset` (`+0x28`)
   gain the module base.
2. For version 3, `fixSize` (`+0x48`) gains the module base.
3. Section table entries `1..numSections-1`:
   - non-zero raw offset → `offset + base`;
   - zero raw offset and non-zero size → BSS: `offset = bss` and
     `bssSection = i`.
4. Import entries `{module_id, stream_offset}`: `stream_offset += base`.
5. `prolog`/`epilog`/`unresolved` (`+0x34/+0x38/+0x3C`) gain the address of
   their section, selected by the byte fields at `+0x30/+0x31/+0x32`.
6. If `0x800030D0` is non-zero, `nameOffset` (`+0x14`) gains it.

Section index 0 is never patched; it is the mandatory empty entry.

### 4. Import walk and relocation streams

Every REL import entry is 8 bytes:

| offset | field |
|---:|---|
| `+0x00` | module id (`0` = `main.dol`) |
| `+0x04` | offset of the relocation stream for that module |

`Relocate(provider, importer)` scans the **importer's** table for the
**provider's** id. If there is no entry, it returns `false` and writes nothing.
If there is, it calls `fn_801b1930`, follows the stream, and returns `true` at
`R_DOLPHIN_END` (type 203).

Each stream record is 8 bytes:

| offset | field |
|---:|---|
| `+0x00` | `u16 delta` |
| `+0x02` | `u8 type` |
| `+0x03` | `u8 section` |
| `+0x04` | `u32 addend` |

The running target cursor starts uninitialised in the original. Every stream in
the four shipped RELs begins with `R_DOLPHIN_SECTION` (202) — all 15 import
lists were checked — which sets the cursor to the start of one of the
importer's sections. The translation starts the cursor at zero and documents
the dependency; a malformed stream therefore faults near address 0 instead of
using a stale caller register.

For a normal record, `section` selects a section of the **provider**. For
module id 0 (`main.dol`), no section base is added: the addend is already the
absolute DOL address.

### 5. Cache maintenance

After patching an executable section, the loader flushes the section range:

```text
DCFlushRange(section.address & ~1, section.size)   // 0x801AC470, dcbf loop + sc
ICInvalidateRange(section.address & ~1, section.size) // 0x801AC560, icbi loop + sync/isync
```

The flush happens at the next `R_DOLPHIN_SECTION` (for the previous section)
and at `R_DOLPHIN_END` (for the last section). Only sections whose raw offset
has bit 0 set are remembered; data sections are not flushed.

### 6. `OSLinkFixed`, `fixSize`, and the import cut

`os_link_fixed` accepts only `version == 3` and calls `link` with
`fixed = true`. After all relocation passes, `link` scans its own import table
and sets `impSize` to the byte offset of the first entry whose module id is `0`
or its own id. This drops that entry and every entry after it. The tail of the
import table — and the `fixSize` region after it — can then be reused by the
caller (typically for BSS).

The shipped RELs are version 3 and have `fixSize` values:

| module | `fixSize` |
|---|---:|
| `d_profileNP` | `0x4F84` |
| `d_basesNP` | `0x223198` |
| `d_enemiesNP` | `0x16E780` |
| `d_en_bossNP` | `0x6D170` |

The linker only makes `fixSize` absolute; it does not itself reuse the region.

## Relocation coverage

`S` is the resolved provider symbol address and `P` the patch address. For
module id 0, the write is just the addend (`S = 0`).

| type | name | `relocate` writes | `undo` writes | observed in NSMBW |
|---:|---|---|---|---:|
| 0 | `R_PPC_NONE` | nothing | nothing | 0 |
| 1 | `R_PPC_ADDR32` | `S + A` | `0` | 91,863 |
| 2 | `R_PPC_ADDR24` | `(S+A) & 0x03FFFFFC \| old & 0xFC000003` | `old & 0xFC000003` | 0 |
| 3 | `R_PPC_ADDR16` | low 16 of `S + A` | `0` | 0 |
| 4 | `R_PPC_ADDR16_LO` | low 16 of `S + A` | `0` | 30,772 |
| 5 | `R_PPC_ADDR16_HI` | `(S+A) >> 16` | `0` | 0 |
| 6 | `R_PPC_ADDR16_HA` | `((S+A) >> 16) + (((S+A) >> 15) & 1)` | `0` | 27,246 |
| 7 | `R_PPC_ADDR14` | `(S+A) & 0x0000FFFC \| old & 0xFFFF0003` | `old & 0xFFFF0003` | 0 |
| 8 | `R_PPC_ADDR14_BRTAKEN` | same as 7 | same as 7 | 0 |
| 9 | `R_PPC_ADDR14_BRNTAKEN` | same as 7 | same as 7 | 0 |
| 10 | `R_PPC_REL24` | `((S+A)-P) & 0x03FFFFFC \| old & 0xFC000003` | `(unresolved-P) & 0x03FFFFFC \| old & 0xFC000003` when `unresolvedSection != 0`, else masked zero | 48,575 |
| 11 | `R_PPC_REL14` | `((S+A)-P) & 0x0000FFFC \| old & 0xFFFF0003` | `old & 0xFFFF0003` | 0 |
| 12 | `R_PPC_REL14_BRTAKEN` | same as 11 | same as 11 | 0 |
| 13 | `R_PPC_REL14_BRNTAKEN` | same as 11 | same as 11 | 0 |
| 201 | `R_DOLPHIN_NOP` | nothing; delta advances `P` | nothing | 56 |
| 202 | `R_DOLPHIN_SECTION` | select importer section; flush previous executable section | same | 38 |
| 203 | `R_DOLPHIN_END` | flush last section; call hook; return `true` | same | 15 |
| other | unknown | report `OSLink: unknown relocation type %3d`; continue | report `OSUnlink: unknown relocation type %3d`; continue | 0 |

Notes:

- Types 3 and 4 both store the low 16 bits in the shipped loader; the original
  dispatch has two arms with the same `sth`.
- Types 8/9 and 12/13 are field-only variants; the loader does not synthesize
  branch-prediction hints.
- The `REL24` undo target is the importer's `unresolved` handler, not the
  original symbol. This is how an unlinked branch is made harmless.
- The 24-bit mask covers instruction bits 2..25; the 14-bit mask covers bits
  2..15. The preserved masks keep the opcode plus the AA/LK bits.

### Per-module exercise

Counts are from `docs/REL2ELF.md` and match a fresh scan of the four RELs.

| module | id | import lists | relocations | type 1 | type 4 | type 6 | type 10 |
|---|---:|---:|---:|---:|---:|---:|---:|
| `d_profileNP` | 1 | 5 | 1,203 | 977 | 56 | 47 | 123 |
| `d_basesNP` | 2 | 4 | 97,524 | 39,650 | 16,743 | 14,819 | 26,312 |
| `d_enemiesNP` | 3 | 3 | 79,624 | 42,079 | 10,522 | 9,396 | 17,627 |
| `d_en_bossNP` | 4 | 3 | 20,105 | 9,157 | 3,451 | 2,984 | 4,513 |
| **total** | | **15** | **198,456** | **91,863** | **30,772** | **27,246** | **48,575** |

Control entries: 56 × 201, 38 × 202, 15 × 203. No type 0, no unknown types.

## Real bases versus synthetic bases

This is the part that is easiest to get wrong. `OSLink` **does not choose the
base**. The flow is:

```text
dDyl: allocate memory, decompress REL, copy file image
   │
   ├─ newModule = allocation address        (the runtime base)
   ├─ bss       = BSS buffer address
   └─ OSLink(newModule, bss)
          └─ section[i].address = newModule + file_offset
```

The REL header stores only file offsets. The module's runtime base exists only
as the pointer the caller passes.

The four synthetic bases (`0x80500000`, `0x80600000`, `0x80A00000`,
`0x80C00000`) are **analysis slots**:

- They are not stored in the REL files.
- They are not produced by `OSLink`.
- They are spaced far enough apart that all four ELFs can be loaded into one
  static analysis database without overlapping.
- Every cross-module pointer and branch displacement in those ELFs is valid
  only for those slots.

The real bases are determined by the `dDyl` allocator and the load order, not
by anything in this cluster. No direct call site for `0x801B1F20` or
`0x801B1F30` exists in the `main.dol` export; the public wrappers are reached
indirectly from the module-loader lane.

### Recovering a base from a linked module

All four shipped RELs place their section table at file offset `0x4C`, directly
after the `0x4C`-byte header. After `link`:

```text
sectionInfoOffset = base + 0x4C
base              = sectionInfoOffset - 0x4C
```

`recover_base` performs that subtraction and then checks:

- the candidate is at least `0x80000000` (MEM1/MEM2);
- the candidate is 16-byte aligned (the shipped `align` value);
- the candidate's module id is non-zero;
- the candidate's `numSections` is plausible (1..=64).

This is a property of these four files, not a guarantee of the general REL
format. An unlinked file has `sectionInfoOffset = 0x4C` and yields base 0,
which the check rejects.

## Corrections to earlier notes

| earlier note | correction | evidence |
|---|---|---|
| `docs/SYMBOLS.md`: `0x801B1960` is `OSLink` | It is the static `Relocate(newModule, module)` helper. The public `OSLink` is the 8-byte wrapper at `0x801B1F20`. | `801b1f20` tail-calls `801b1c30`; `801b1960` only applies one provider/importer pair. |
| `docs/SYMBOLS.md`: `0x801B1F50` is `OSUnlink` | It is the static `Undo(newModule, module)` helper. The public `OSUnlink` is `0x801B2190`. | `0x801B2190` dequeues the module and calls `0x801B1F50` once per remaining module. |
| `docs/ARCHITECTURE.md`: "real bases are chosen by `OSLink`" | `OSLink` consumes a caller-chosen pointer. The allocation/placement is in `dDyl`; the REL carries no base. | `link` adds the first argument to every section offset; it has no allocation call. |
| `docs/REL2ELF.md`: `bssAlign = 8` means 256-byte alignment | The converter uses the exponent interpretation. The loader compares the raw field as a byte multiple (`bss % 8 == 0`). | `0x801B1C9C..0x801B1CA8`: `divwu`/`mullw`/`subf.` with the raw `+0x44` value. |

The string evidence in `docs/SYMBOLS.md` was real: the "OSLink: unknown
relocation type" string is referenced from `0x801B1960`. That is the *helper*
that detects unknown types, which is why the string is there rather than in the
8-byte public wrapper.

## Unknowns and limits

1. **Four empty hooks.** `0x801B1910`, `0x801B1920`, `0x801B1930`, and
   `0x801B1940` are genuine 4-byte `blr` functions. The first two fit the SDK
   `OSNotifyLink` / `OSNotifyUnlink` positions; the last two have no name
   evidence. All four are translated as no-ops, not invented behaviour.
2. **`fixSize` reuse.** The linker makes `fixSize` absolute and, for
   `OSLinkFixed`, truncates the import table. It does not itself replace the
   dropped region with BSS or anything else; that is the caller's decision.
3. **No direct callers.** The `main.dol` export contains no `bl` to
   `0x801B1F20` / `0x801B1F30`. The linker is reached indirectly from the
   `dDyl` lane. The exact call site is still unknown.
4. **Unexercised relocation types.** Types 2, 3, 5, 7-9, and 11-13 are
   implemented from the loader's own dispatch and the PowerPC ELF ABI, but the
   four shipped RELs do not contain them. They are `high` confidence in shape,
   not in field-exercise.
5. **Cursor initialisation.** The original's target cursor (`r28`) is
   uninitialised by the compiler; the first `R_DOLPHIN_SECTION` record sets it.
   A stream that starts with a normal record would fault. The translation uses
   a documented zero start.
6. **Alignment field meaning.** The loader treats `align`/`bssAlign` as byte
   counts. The file-format documentation calls them powers of two. The shipped
   values (`16`, `8`) and the loader's `divwu` sequence are the evidence here.
7. **Multiple BSS-like sections.** `link` overwrites `bssSection` for every
   zero-offset/non-zero-size entry, so the last one wins. The four shipped RELs
   have exactly one.
8. **BSS size source.** The zero-fill length is the header `bssSize` (`+0x20`),
   not the section table's size field. They agree in the four shipped files.
9. **Host execution.** `relocate` and `undo` patch real memory and call
   PowerPC cache instructions through `0x801AC470` / `0x801AC560`; they are not
   host-runnable without an emulator or a mock memory map.

## Next targets

| address | size | why |
|---|---:|---|
| `0x8008F640` | 268 | loads `WIIMJ2DNP.str` and calls `OSSetStringTable`; part of the `dDyl` setup |
| `0x8008F760` | 68 | passes `0x8008F640` to `0x8016B8D0`; shows when the string table becomes valid |
| `0x8008F870` | 60 | `dDyl` constructor; the REL-loading vtable/object |
| `0x801B2390` | 96 | next OS object after `__OSModuleInit`; initialises two queues and two words (thread/mutex-shaped, not linker) |
| `0x801B23F0` | 200 | next function in the OS cluster; check whether it searches the module list |

The high-value unknown is not another relocation type; it is the `dDyl` caller.
That is where the real bases and the LZ11 decompression are decided, and where
the section bytes must be placed before `OSLink` runs. Until it is translated,
the four synthetic slots remain the only addresses this reconstruction can use.

## Verification notes

- **doc-lint.** `node tools/doc-lint.mjs src` reports
  `{"ok": true, "files": 68, "violations": 0}` after this lane's edits.
- **crate check.** `cargo check` exits 0 with no errors or warnings. Earlier in
  this pass the crate stopped on errors in `src/engine/display.rs` and
  `src/game/boot_scene.rs`; those lanes were fixed while this lane was being
  verified, and no diagnostic ever pointed at `src/runtime/link.rs`.
- **isolated check.** Before those cross-lane fixes landed, `src/runtime/link.rs`
  was copied into a temporary no-std crate together with `src/platform.rs` and
  signature-compatible stubs for `memset_wrapper` and `dc_flush_range`.
  `cargo check` finished with no errors or warnings for the linker file.
- **relocation replay.** A standalone scan replayed the translated algorithm
  (provider selection, 202 cursor switches, delta accumulation, types 1/4/6/10)
  over all 15 import streams and compared every patched field with the audited
  `work/rels/elf/*.elf` images: 198,456 of 198,456 fields match, 0 mismatches.
  The same scan confirmed that every stream begins with a 202 record, which is
  what justifies the documented zero cursor start.
- **argument order.** `0x802DCD64` is `__savegprlr_27` (saves `r27`-`r31`).
  It does not initialise `r28`, which is why Ghidra prints the target cursor as
  `unaff_r28`. The raw words `0x801B1974..0x801B19E0` show `r3`/`r4` saved to
  `r26`/`r27`, and the import scan reads `0x28(r27)` — that is, the second
  argument's import table. This fixed Ghidra's swapped `piVar3`/`iVar6`
  rendering.
- **no invented symbols.** The four hooks keep `fn_801b19x0` names. The only
  descriptive names are the SDK/API names supported by the call shape
  (`OSLink`, `OSLinkFixed`, `OSUnlink`, `OSSetStringTable`, `__OSModuleInit`,
  plus the static `Relocate`/`Undo` spellings); the binary itself has only
  `FUN_*` symbols and the two format strings.
- **files touched.** `src/runtime/link.rs`, this document,
  `ledger/agents/oslink.csv`. No `lib.rs`, `mod.rs`, `Cargo.toml`,
  `platform.rs`, or other lane file was changed.
