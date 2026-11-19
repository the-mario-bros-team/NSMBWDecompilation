# Audit - Wave 230 (ledger polish + pointer-table batch 3)

## Ledger polish

| Item | Result |
|---|---|
| Rows with no usable evidence path | **54 / 54 fixed** in `boot.csv`, `engine.csv`, `engine-frame.csv`, `oslink.csv`, `scene.csv` |
| Ghidra-style `FUN_<addr>` names | **621 normalised** to `fn_<addr>` across 27 agent CSVs |
| Malformed name | `fn_8060180` -> `fn_80600180` |
| Dead intra-doc `fn_<addr>` links | **5 repointed** (`cargo doc` unresolved links 46 -> 43) |

All edits went into `ledger/agents/*.csv` (the source of truth) and doc comments,
never into `ledger/functions.csv`. The 54 evidence fields keep their original
notes after `"; "`, and the special export names (`80004050_entry.c`,
`800e4820_thunk_FUN_8006a320.c`) are cited verbatim.

Two follow-up classes were found and left for later:

- **69 rows** cite `Ghidra main-gb/export/….c` without the `work/ghidra/`
  prefix (they resolve as basenames, so they were outside the audit's 54).
- **2,447 "evidence missing" complaints** in historical lane CSVs, plus 355
  `0x`-prefixed addresses and 3,232 "already in ledger" notices. An untouched
  control wave fails the same way, so these are legacy CSV formatting, not
  regressions from this wave.

Rustdoc still has 43 unrelated unresolved links (e.g. `player.rs:6175`
`fn_80601be0`, `stage.rs:69228` `fn_8007b96a0`) outside the audit list.

## Pointer-table batch 3

| Lane | Table | Bodies | Bytes |
|---|---|---:|---:|
| C | `.ctors` slots 29-42 | 12 | 2,572 |
| D | `.ctors` slots 43-56 | 11 | 5,052 |
| E | world-map vtable `0x80325420` | 5 | 1,848 |
| F | `dEn_c` vtable `0x80314060` | 10 | 764 |

**38 bodies / 10,236 bytes.** Slots already translated were referenced rather
than duplicated throughout (`.ctors` 31/40/47/54/55; world-map 1/2/4/5/7;
`dEn_c` 2/10).

## Structural finding: several "state tables" are vtables

Three of the wave-227 tier-3 entries are not state tables at all:

- **`0x80314068`** is the method array of the vtable at **`0x80314060`**
  (`{0,0}` header, installed at object `+0x60`, dispatched as
  `[[object+0x60] + 8 + 4n]`). Callers name `0x80314060`, two words before the
  scanned base - which is exactly why the audit called it unreferenced. True
  count: **306 unique targets, 281 untabled**, not "59 entries".
- **`0x80325428`** is the method array of the WorldMap map-object vtable at
  **`0x80325420`**, installed at `+0x60` by `fn_801442b0`. A full
  `lis`/`addi`/`ori` + data-word scan found **zero** references to
  `0x80325428` itself; every caller uses the vtable base. Slot `n` is offset
  `8 + 4n`.
- **`0x80343E90`** (wave 228) was a genuine table, but its tier-1 status came
  from `lis 0x8034` strings for KDAP diagnostics rather than table loads.

Consequence: the wave-227 tier list over-counts "unreferenced" groups and
under-counts entries per group, because it scanned for the first method word
rather than the vtable header two words earlier. A re-scan anchored on
`base - 8` would shrink the backlog estimate and should be done before the next
pointer-table wave.

## Extent-measurement lesson

Lane E caught a bogus extent: slot 5 of the world-map vtable is a **4-byte
branch**, but a naive first-`blr` scan read 176 B because it crossed 12 padding
bytes into the unrelated `0x800643F0` body. Lane A's wave-229 note applies
again: use recursive descent, stop on `BO=20` returns and unconditional `b`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `merge-ledger` OK: 20,250 rows, 20,250 unique, 0 duplicates.
- `scripts/check.ps1` exit 0.
- Calibration: the already-translated `.ctors` slots 47/54/55 matched their
  `functions.tsv` sizes exactly (308/1,500/2,316), independently validating the
  measurement method.
