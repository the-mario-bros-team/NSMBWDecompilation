# Coverage - Wave 109 (`audio::ax`)

## How to read this file

This is the status page for the wave-109 pass over the `audio::ax` lane of the
`main.dol` reconstruction. The `0x801A0000` bucket holds 271 inventory entries.
After wave 108 only six of them had no byte-level body, and this pass lands all
six: the performance-monitor special-register leaves at
`0x801A9BC0..0x801A9C10`.

The sections answer, in order:

1. **What did this pass translate?** - *Functions* lists the six bodies with
   their addresses, Rust names, byte sizes and confidence.
2. **Why were they still open?** - *What this pass covers* explains the
   `runtime::os` range row that previously stood in for these bytes and why the
   individual bodies are still useful.
3. **Was anything left out?** - *Deliberate skips* is empty for this pass.

**Virtual addresses.** All addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets.

**Evidence.** The Ghidra C is under `work/ghidra/main-gb/export/`; the function
index is `work/ghidra/main-gb/export/functions.tsv`. Inline Rust markers follow
`docs/RUST_STYLE.md`: `// observed:`, `// inferred:` and `// unknown:`.

**Ledger.** The machine-readable rows are `ledger/agents/wave109-ax.csv`
(module `audio::ax`, 6 rows, all `translated`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/ax.rs` | `audio::ax` | 6 | 48 | 6 | 0 |

Status split: 6 `translated`, 0 `partial`, 0 `stub`. Address range:
`0x801A9BC0..0x801A9C10`.

## What this pass covers

- **The six performance-monitor leaves, `0x801A9BC0..0x801A9C10`.** Every
  Ghidra export in this range decompiles to an empty `return;`, because the only
  non-branch word is an `mtspr`. The words are the SPR-encoded forms of
  `mtspr <spr>, r3`, each followed by `blr`:

  | address | instruction word | SPR | register |
  |---|---|---:|---|
  | `0x801A9BC0` | `0x7C78EBA6` | 952 | `MMCR0` |
  | `0x801A9BD0` | `0x7C7CEBA6` | 956 | `MMCR1` |
  | `0x801A9BE0` | `0x7C79EBA6` | 953 | `PMC1` |
  | `0x801A9BF0` | `0x7C7AEBA6` | 954 | `PMC2` |
  | `0x801A9C00` | `0x7C7DEBA6` | 957 | `PMC3` |
  | `0x801A9C10` | `0x7C7EEBA6` | 958 | `PMC4` |

  Each body is a single `asm!` write; `value` arrives in `r3`, exactly as in the
  original. The byte readings are those recorded in
  `docs/COVERAGE_WAVE108_AX.md`; the SPR field of each word was re-decoded here
  (`952 = 0x3B8`, low five bits `0x18`, high five bits `0x1D`, and so on) and
  matches the register in the table, so the write is the whole effect.
- **Relationship to `runtime::os::clear_performance_monitor`.** The
  `runtime::os` lane owns the range row `801a9bc0-801a9c10`, and its
  `clear_performance_monitor` body calls these six addresses with `0`. That
  helper reaches the leaves as opaque callees; it does not translate the six
  instruction words themselves. This pass supplies the byte-level bodies, so a
  reader can now follow the call from `clear_performance_monitor` into a real
  `mtspr`. The range row and these six rows cover the same 48 bytes from two
  different levels; the coordinator may retire the range row, but this lane does
  not edit `ledger/functions.csv` or `src/runtime/os.rs`.
- **Bucket status.** With these six bodies the `0x801A0000` bucket has an
  individual-body entry for all 271 inventory functions. There is no remaining
  reconstructible address in the bucket.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801A9BC0` | `fn_801a9bc0` | 8 | high |
| `0x801A9BD0` | `fn_801a9bd0` | 8 | high |
| `0x801A9BE0` | `fn_801a9be0` | 8 | high |
| `0x801A9BF0` | `fn_801a9bf0` | 8 | high |
| `0x801A9C00` | `fn_801a9c00` | 8 | high |
| `0x801A9C10` | `fn_801a9c10` | 8 | high |

Totals: **6 bodies**; **6 high, 0 medium**; **48 bytes**.

## Deliberate skips

None. All six remaining entries in the bucket were reconstructible.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (`8` bytes each, `FUN_801a9bc0`
  through `FUN_801a9c10`).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/audio/ax.rs`, and its doc block carries the same confidence.
- No individual address in this CSV is present in `ledger/functions.csv`; the
  pre-existing `801a9bc0-801a9c10` range row is discussed above.
- `cargo check --message-format=short` is free of errors for
  `src/audio/ax.rs`.
- `node tools/check-lane-csv.mjs wave109` reports `wave109-ax.csv: 6 rows` and
  `OK` for this lane.
