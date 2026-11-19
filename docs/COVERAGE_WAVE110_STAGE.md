# Coverage - Wave 110 (`game::stage`, the `0x800B0000` bucket tail)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave110-stage.csv` is the Ghidra C export each body was
written from.

Two conventions matter for this tail of the bucket:

* The `0x802DCD40..0x802DCD74` helpers are the Metrowerks `__savegprlr_NN`
  prologues. The `u64` Ghidra prints as their return value is the caller's own
  `r3:r4` pair, so `packed >> 32` is `r3` and `packed as u32` is `r4`. Both new
  builders read `this` from the high half and the draw-list owner from the low
  half.
* The `ldexpf`/`CONCAT13`/`CONCAT22` blocks in the lifted prologues only write
  stack words that nothing reads. They are recorded in the body docs as dead
  and are not reproduced; the surviving code is plain `f32`/`f64` arithmetic
  plus the `0x43300000` signed/unsigned-int idioms, which are written as casts.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 3 | 1532 | 0 | 3 |

All three rows are `translated` and `medium`; none is `partial`. The pass covers
`0x800B9A90..0x800BC5D0`.

## What this pass covers

- **The lift follow pass** (`0x800B9A90`). Reads the source record at `this+0x24`
  and the `+0x1C` count / `+0x20` head of the node list. `fn_800b95e0` supplies
  the ratio that fixes how many trailing nodes are blended toward the source
  anchor at `+0x62C`/`+0x630`; each node's end point is the `r2-0x69D0`
  interpolation from the current start to the previous end, clamped against
  `r2-0x69D4 + [source+0x78C]` and `r2-0x69E0`, then normalised to
  `r2-0x69B8` before `fn_800bae60` rebuilds its derived fields.
- **The animated three-node builder** (`0x800BC2E0`). For each of the three
  `0x40`-stride nodes it loads a model through `0x80239E30`, builds it with
  `0x80169D90` and the three `0x802F0D50` flag words, closes it with
  `0x80064BF0`, loads the clip named by the `r2-0x6950` byte table + 4, and
  binds the pair to the `0xCC + slot*0x2C` sub-object through
  `0x80166830`/`0x80166BD0`/`0x80166EF0`. The shared half-extent is stored at
  `+0xC0` and split by `fn_800bc5a0`.
- **The plain three-node builder** (`0x800BC5D0`). Same three `0x40`-stride
  nodes and the same model/flag/build/finish sequence, but no animation pair;
  the three flag words come from `0x802F0D60`. It stores the same `+0xC0`
  half-extent and calls `fn_800bc5a0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B9A90` | `fn_800b9a90` | 936 | medium |
| `0x800BC2E0` | `fn_800bc2e0` | 384 | medium |
| `0x800BC5D0` | `fn_800bc5d0` | 212 | medium |

Address range: `0x800B9A90..0x800BC5D0` (3 bodies, 1532 bytes).

## Deliberately skipped

The bucket held nine open entries. Three are landed above; the other six are
skipped on purpose.

- **Branch thunks into already-translated bodies** (`0x800B4600`,
  `0x800B54B0`, 4 bytes each). Their exports are `thunk_FUN_800b4610` and
  `thunk_FUN_801dd1d0`; both targets already have ledger rows (`game::stage`
  `0x800B4610` and `runtime::dvd` `0x801DD1D0`), so a row here would
  double-count the body. Same precedent as waves 84 and 98-109.
- **The stack-local template** (`0x800BC140`, 344 bytes). It probes the four
  `0x802F0D40` type records, then reads `local_44`/`local_34`/`local_24`
  before any store writes them. The export's only other stack writer is the
  later `0x801C0C60` matrix build, so those three words are values the callee
  never defines; writing the transform would be invention. Wave 104 skipped
  the same body for the same reason.
- **The per-strip grid pass** (`0x800BD970`, 1680 bytes). Its body consumes
  `this`/`slot` from `0x802DCD40`'s `r3:r4` pair, but also a column count, a
  variant index and a mode that are not all present in the five arguments the
  only caller (`0x800BE000`) prints. The printed callee prototype assigns
  `param_3..param_5` to registers that the printed call does not pass, so the
  order of the hidden `r6`/`r7` words (count versus variant) cannot be proven.
  Writing the stores against a guessed order would put the mirror table on the
  wrong index, so the body is left out.
- **The per-cell grid pass** (`0x800BEF00`, 1260 bytes). The argument layout is
  recoverable (`this, first, second, variant`, same four words as
  `0x800BF3F0`), but the export reuses `local_64`/`local_68` as both the cell
  geometry and the submitted translation vector, and the `FUN_800be7b0`
  argument groups (`&local_5c`, `&local_74`, ...) read slots that earlier
  blocks had already overwritten with a different meaning. The five submitted
  vectors therefore cannot be stated without guessing.
- **The GQR-quantised stagger pass** (`0x800B96A0`, 1008 bytes). Its
  `0x80237BD0`/`0x80237C40` calls take a `u16` angle that the export only
  materialises through a live `GQR3` conversion. The `else` arm of that
  conversion reads `auStack_1b4`, which nothing writes, and the second value
  collapses to the constant `0.0004891455173492432`. Since the two trig
  arguments are not both defined, the pass cannot be reconstructed.

Three landed + six skipped = the nine open entries of the `0x800B0000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (3 rows, 1532 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the three were
  chosen from the nine open entries of the `0x800B0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave110` prints `OK` for this CSV.
- The module header of `src/game/stage.rs` gained a wave-110 paragraph naming
  all three bodies and the six skips, plus the wave-110 run list in the
  "Original range and ownership" section.
