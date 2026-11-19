# Coverage - Wave 115 (`game::stage`, closing the `0x80070000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave115-stage.csv` is the Ghidra C export each body was
written from.

The bucket held 192 entries and 189 already had `game::stage` rows, so wave 115
starts from three open entries. Two are landed here and one is deliberately
skipped, which closes the bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 2 | 3728 | 0 | 2 | 0 |

Both rows are `translated`; none is `partial` or `stub`. The pass covers
`0x800777B0..0x8007AC40`.

## What this pass covers

- **The cell re-decode wrapper** (`0x800777B0`, 96 bytes). It forwards its four
  arguments to the already-translated [`fn_80077730`], which fills three `u16`
  stack slots (layer tile, mask value, flat cell index), ORs the first two and
  writes the result at the flat index of the current stage's `layer` through
  `0x80082140(manager, stage, index, layer, value)`. The two call argument lists
  are missing from the export, so they were recovered from the code bytes:
  `r7/r8/r9 = r1+0xA / r1+8 / r1+0xC` before the first call, then
  `r5 = lha r1+0xC`, `r7 = (s16)[r1+0xA] | (s16)[r1+8]`, the `r13-0x5880` scene
  word and the `r13-0x54B8` stage byte before the second. The `stb` of the OR-ed
  byte back into the first output slot has no reader and is noted, not
  reproduced.
- **The background-camera window rebuild** (`0x8007AC40`, 3632 bytes). This is
  the per-frame z/window solve: the `+0x9000C`/`+0x90028` zooms become the two
  depth reciprocals, `ABS(x0-x1)/[r13-0x5594]` and
  `ABS(y1-y0)/[r13-0x5590]` become the spans, and `0x8007BD40` returns the
  `(horizontal, vertical)` fit pair. The seven fit readers
  (`0x80079250`/`0x80079380`/`0x800794B0`/`0x800795E0` and the three
  `0x80078860` wrappers) narrow the spans per mode, the `+0x90094` 0/1/2 state
  machine latches the layer state, two `0x8015F140` curve calls build the
  `+0x90088`/`+0x9009C` z window, and the result is written to
  `+0x8FE80/+0x8FE84`, `+0x8FE90/+0x8FE94`, `+0x8FE98`, `+0x900A4/+0x900A8`,
  `+0x900AC/+0x900B0` and `+0x8FE60`. The single value wave 109 could not
  recover — the second float of `FUN_8007bd40(dVar25, ?, &local_118, …)` — is
  the export's own `fVar3` (`r2-0x7274` = 1.2f when `+0x900FE` is 1, otherwise
  `r2-0x7270` = 0.9f): the code bytes at `0x8007AD8C..0x8007ADB0` load it into
  `f2`, keep it there across the `fmuls f21,f21,f2` and pass it unchanged, so
  the body uses exactly that word and invents nothing. `0x802DCD74` is
  `__savegprlr_27` rather than a value-returning call, which is why the export's
  `uVar29/iVar10/pfVar12` are simply the incoming `r3`/`r4` (`this`, `rect`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800777B0` | `fn_800777b0` | 96 | medium |
| `0x8007AC40` | `fn_8007ac40` | 3632 | medium |

Address range: `0x800777B0..0x8007AC40` (2 bodies, 3728 bytes).

## Deliberately skipped

- **`0x800776F0`** (`thunk_FUN_80077650`, 4 bytes). Its export is the *target*
  body, the same precedent as `0x800B4600`/`0x800B54B0` in wave 114 and the
  thunks of waves 84 and 98-109: the 4-byte entry branches into `0x80077650`,
  which already owns a `game::stage` row (`high`), so a second body here would
  double-count the same code. `functions.tsv` lists it at 4 bytes, and the
  decompiled text is the full `0x80077650` body Ghidra copied through the
  branch.

Two landed + one skipped = the three open entries of the `0x80070000` bucket,
which is now closed (192/192).

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 3728 bytes;
  `0x800777B0` = 96, `0x8007AC40` = 3632).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the two were
  chosen from the three open entries of the `0x80070000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave115` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-115 paragraph naming
  both bodies (3728 bytes) and the one skip, plus a wave-115 line in the
  original-range list; no earlier count became stale.
