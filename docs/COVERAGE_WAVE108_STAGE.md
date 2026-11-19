# Coverage - Wave 108 (`game::stage`, the `0x80080000` bucket tail)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave108-stage.csv` is the Ghidra C export each body was
written from.

Two conventions matter for this band:

* **The `__savegprlr_NN` prologues.** `0x802DCD48..0x802DCD74` return their
  saved `r3`/`r4` pair through a `u64`; the bodies below therefore read their
  first two arguments back from that pair. Because the pair is the function's
  own argument pair, `fn_80082740`, `fn_80082be0`, `fn_80082ed0`, `fn_80083fc0`
  and `fn_800883e0` name the two words directly instead of calling the helper.
  The matching `0x802DCD94..0x802DCDBC` epilogues are still called, because the
  export delivers the result through them.
* **The paired-single `GQR0` prologues are dead.** `0x80082BE0` and
  `0x80082ED0` open with a 22-float `ldexpf` dequantisation of `f21..f31`; the
  resulting stack slots are never read, so, as in wave 107, they are not
  reproduced. The `0x43300000` / `2^52 + 2^31` folds that *are* read are the
  compiler's signed-word-to-double idiom and are written as `word as i32`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 11 | 7060 | 2 | 8 |

Eight rows are `medium`, two (`0x800818D0`, `0x8008F610`) are `high` and one
(`0x80082ED0`) is `low`; none is `partial`. The pass covers
`0x800818D0..0x8008F610`.

## What this pass covers

- **The heap size and the state dispatcher** (`0x800818D0`, `0x80081960`). The
  first returns the `0x20F2C` bytes `load_bg_units` multiplies by twelve; the
  second rewrites the `(status, handle)` pair from the `0x800D87D0`/`0x800D87B0`
  bank tests, the `0x35..0x37` handle kinds and the `0x3B`/`0x3C` camera-owner
  case.
- **The three scene draw bodies** (`0x80082740`, `0x80082BE0`, `0x80082ED0`).
  `0x80082740` converts the camera rectangle into a clamped tile window
  (`+0x30`/`+0x32` extents), builds the twelve-word colour block from
  `0x802EF968`, and runs `0x80082BE0`/`0x80082ED0` once per layer.
  `0x80082BE0` walks `rows × columns` cells of the `+0x909C8` layer mode's
  table (`0`, `0x4E`, `0x10E`, `0x1CA`) and emits each surviving `0x3FF`-masked
  tile; `0x80082ED0` walks the `+0x3C` node list and composes four billboard
  variants through the `0x801Cxxxx` matrix chain.
- **The tile-run builder** (`0x80083FC0`). Three 3-byte-stride scans classify
  the run (`first`/`second`/`end`), then the negative-count arm walks left from
  `first - 3` and the positive-count arm walks right, each with two wrap
  targets; the descriptor cursors `+0xC0C`/`+0xC0E` are written on every step
  and the start cell is updated to the `0xFE` terminator.
- **The tile-driven actor spawn** (`0x800883E0`). Wraps `x` through
  `0x80101AA0`, queries the `(x, -y)` tile, resolves the actor class through
  `0x8005F900` and the two virtual slots `+0xE0`/`+0x3D4`, and dispatches on
  the `0x802F0010`/`0x802EFF20`/`0x802EFFD0` band tables.
- **The stage-object draw record** (`0x8008A090`). Three stack blocks (0x88,
  0x40 and 0x70 bytes) are filled in order by `0x802BF930`/`0x802BEA00`,
  `0x802BFDA0`/`0x802BFE10`, `0x802BFE90`/`0x802BE880` and the
  `0x80253D00`/`0x802541F0`/`0x802540B0` projection.
- **The stage-block spawn wrappers** (`0x8008B2E0`, `0x8008B500`). Both are
  12-byte tail calls into [`stage_block_spawn`] with `f1 = 0.0f`; every other
  register is forwarded unchanged.
- **The dDyl predicate** (`0x8008F610`). A literal `return 1;`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800818D0` | `fn_800818d0` | 12 | high |
| `0x80081960` | `fn_80081960` | 332 | medium |
| `0x80082740` | `fn_80082740` | 1176 | medium |
| `0x80082BE0` | `fn_80082be0` | 752 | medium |
| `0x80082ED0` | `fn_80082ed0` | 1192 | low |
| `0x80083FC0` | `fn_80083fc0` | 1732 | medium |
| `0x800883E0` | `fn_800883e0` | 1420 | medium |
| `0x8008A090` | `fn_8008a090` | 412 | medium |
| `0x8008B2E0` | `fn_8008b2e0` | 12 | medium |
| `0x8008B500` | `fn_8008b500` | 12 | medium |
| `0x8008F610` | `fn_8008f610` | 8 | high |

Address range: `0x800818D0..0x8008F610` (11 bodies, 7060 bytes).

## Deliberately skipped

The bucket held 13 open entries. Eleven are landed above; the other two are
skipped on purpose.

- **A branch thunk into an already-translated body** (`0x80083B30`, 4 bytes).
  It is `b 0x8016E980`, and `fn_8016e980` already has a ledger row in
  `src/resource/registry.rs`, so a row here would double-count the target. Same
  precedent as waves 84, 98-107.
- **The filled-tile rectangle** (`0x80084690`, 4328 bytes). The body is the
  `0x80..0x93` dispatcher that fills a rectangle of cells, and two of its call
  sites into `0x80083FC0` print only three arguments
  (`FUN_80083fc0(owner, desc, local_28)` at export lines 58 and 171), leaving
  the run-length register `r6` at whatever the preceding block left there.
  Unlike `0x80083FC0`'s own callees, those two sites do not define the fourth
  argument, so writing a count would be invention. The body stays untranslated
  and uncounted.

Eleven landed + two skipped = the 13 open entries of the `0x80080000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (11 rows, 7060 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the eleven were
  chosen from the 13 open entries of the `0x80080000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`; the remaining crate errors are in other lanes' files.
- `node tools/check-lane-csv.mjs wave108` prints `OK` for this CSV.
- The module header of `src/game/stage.rs` gained a wave-108 paragraph naming
  all eleven bodies and the two skips, plus the wave-108 run list in the
  "Original range and ownership" section.
