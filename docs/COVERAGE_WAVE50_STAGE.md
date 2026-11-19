# Coverage - Wave 50 (`game::stage`, transition/archive helpers and font slots)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body.

This pass fills the two untranslated runs that surround the wave-45 fukidashi
object in the `0x800B0000` bucket. The first run
(`0x800B0440..0x800B1410`) is the resource/transition side of a scene change:

- Six `r13` words hold archive handles created by `0x800B0B50` and polled by
  `0x800B0DB0`.
- The current scene object at `r13-0x5240` receives the transition value
  through `0x8016DCE0` and then one of its vtable slots (`+0x14`, `+0x18`,
  `+0x1C`).
- The four font records at `0x80358848` (stride `0x1C`, each with a loader
  record at `0x803588B8` and a handle word at `+0xC0 + slot*4`) are built by
  `0x800B11D0`, destroyed by `0x800B1250`, filled by `0x800B1330` and queried
  by `0x800B12D0`/`0x800B1310`/`0x800B1410`.
- `0x800B0FD0` is a 32-entry × 16-byte lookup table; `0x800B0690` compares the
  tails of two `:`-separated paths; `0x800B05B0` walks at most five `+0x30`
  parent links.

The second run (`0x800B2E70..0x800B30E0`) is the transition teardown plus the
two camera-scale readers that the stage scroll conversion multiplies by. The
camera layout is the same `r13-0x5880` object wave 48 used: `+0x38`/`+0x3C`
position, `+0x40`/`+0x44` step.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 25 | 3020 | 17 | 7 |

One further body (`0x800B0B00`) is `low` confidence; it is the only row that is
neither high nor medium.

## What this pass covers

- **Object teardown and path helpers** (`0x800B0440`, `0x800B05B0`,
  `0x800B0690`, `0x800B0750`, `0x800B0780`, `0x800B07B0`, `0x800B09E0`) - the
  two-stage release helper, the parent-chain walk, the `:`-tail comparison, the
  duplicated `0x802DCBFC` forwarding pair and the three-way float selector.
- **Transition state forwarding** (`0x800B0A20`, `0x800B0A90`, `0x800B0B00`,
  `0x800B0B20`) - the `0x0400_0000` forced-path bit, the `0x8016DCE0` publish
  and the `+0x14`/`+0x18`/`+0x1C` vtable slots, plus the two-call cleanup.
- **Resource group and its poll** (`0x800B0B50`, `0x800B0DB0`) - six handles
  (`0x1BC` ×4, `0x1C4`, `0x18`, `0x58`), six loaders, and the six-state
  readiness selector that also writes the layout byte `+0x1C1`.
- **Lookup and font slots** (`0x800B0FD0`, `0x800B11D0`, `0x800B1250`,
  `0x800B12D0`, `0x800B1310`, `0x800B1330`, `0x800B1410`) - the 32-entry table,
  the two four-slot containers, the four font loads under the `0x802DCD70`
  lock, and the index/record accessors.
- **Transition teardown and camera scale** (`0x800B2E70`, `0x800B2F10`,
  `0x800B2FB0`, `0x800B30C0`, `0x800B30E0`) - the resource-list rebuild, the
  factor multiply, the two releases and flag clears, and the two interpolated
  camera coordinates.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B0440` | `fn_800b0440` | 92 | high |
| `0x800B05B0` | `fn_800b05b0` | 224 | medium |
| `0x800B0690` | `fn_800b0690` | 136 | high |
| `0x800B0750` | `fn_800b0750` | 48 | high |
| `0x800B0780` | `fn_800b0780` | 48 | high |
| `0x800B07B0` | `fn_800b07b0` | 80 | high |
| `0x800B09E0` | `fn_800b09e0` | 64 | medium |
| `0x800B0A20` | `fn_800b0a20` | 108 | medium |
| `0x800B0A90` | `fn_800b0a90` | 108 | medium |
| `0x800B0B00` | `fn_800b0b00` | 20 | low |
| `0x800B0B20` | `fn_800b0b20` | 36 | high |
| `0x800B0B50` | `fn_800b0b50` | 604 | high |
| `0x800B0DB0` | `fn_800b0db0` | 240 | high |
| `0x800B0FD0` | `fn_800b0fd0` | 300 | high |
| `0x800B11D0` | `fn_800b11d0` | 124 | high |
| `0x800B1250` | `fn_800b1250` | 128 | high |
| `0x800B12D0` | `fn_800b12d0` | 52 | medium |
| `0x800B1310` | `fn_800b1310` | 20 | high |
| `0x800B1330` | `fn_800b1330` | 224 | high |
| `0x800B1410` | `fn_800b1410` | 124 | medium |
| `0x800B2E70` | `fn_800b2e70` | 56 | high |
| `0x800B2F10` | `fn_800b2f10` | 60 | medium |
| `0x800B2FB0` | `fn_800b2fb0` | 68 | high |
| `0x800B30C0` | `fn_800b30c0` | 28 | high |
| `0x800B30E0` | `fn_800b30e0` | 28 | high |

## Deliberately skipped

- `0x800B04A0`, `0x800B04B0`, `0x800B04C0` (12 bytes each) - three bare
  `return &DAT_...` global getters, below the module's 16-byte floor; the
  globals they return are already reachable from the translated siblings.
- `0x800B19B0` (8 bytes), `0x800B2EB0`/`0x800B2EC0` (8 bytes),
  `0x800B2ED0`/`0x800B2EE0` (12 bytes), `0x800B2F00` (8 bytes) - below the
  same floor; `0x800B2EB0` is still called by address from `fn_800b2e70`.
- `0x800B3000` (192 bytes) - the export is dominated by GQR/paired-single
  reconstruction noise (`ldexpf` blocks, unreachable-block warnings) with the
  real argument registers lost; a body would be guesswork, so it is left.
- `0x800B33D0` and the following run - another paired-single export with the
  `r2`/`r13` operands only partly recovered; it belongs to the next cluster,
  not this one.
- `0x800B3540`, `0x800B3600` and the rest of the bucket - outside the two runs
  this pass owns; they need their own field map (course-manager slot stride
  `0x23C`) and are better handled together.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave50` reports `OK` for the stage CSV.
