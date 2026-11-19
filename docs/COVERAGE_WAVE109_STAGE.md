# Coverage - Wave 109 (`game::stage`, the `0x80070000` bucket tail)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave109-stage.csv` is the Ghidra C export each body was
written from.

Two conventions matter for this band:

* **The save-helper pairs.** `0x802DCD50`/`0x802DCD70`/`0x802DCD74` return the
  incoming `r3`/`r4` pair as one `u64`. The bodies below therefore read their
  first argument(s) back from that pair; `0x802DCCE0` (the `f28..f31` saver) and
  `0x802DCD6C` leave `r3` untouched, so the layer/scene pointer survives the
  call and is used directly.
* **The paired-single `GQR`/`ldexpf` prologues are dead.** Every `0x80075820`,
  `0x80075FD0`, `0x8007A6C0`, `0x8007AC40` and `0x80079E80` opens with a block of
  `ldexpf` dequantisations that only write stack slots nobody reads; they are
  not reproduced. The `0x43300000`/`2^52 + 2^31` sequences that *are* read are
  the compiler's signed-word-to-double idiom and are written with
  [`w101_bias`], the existing helper for that exact bit pattern.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 7 | 8064 | 0 | 4 |

Four rows are `medium` (`0x80075820`, `0x80077700`, `0x8007A6C0`, `0x8007FA40`)
and three are `low` (`0x80075FD0`, `0x80079E80`, `0x8007CD70`); none is
`partial`. The pass covers `0x80075820..0x8007FA40`.

## What this pass covers

- **The cell-request wrapper** (`0x80077700`). Forwards `(owner, x, y, mode)` to
  `0x80077650` and returns the low word of its `r3:r4` pair; the callers
  `0x80070C60` and `0x801260F0` confirm the four-argument shape.
- **The layer bootstrap** (`0x8007FA40`). One 132-byte body that initialises a
  scroll layer through `0x8007F9B0`, hands the four doubles to `0x8007FBF0`,
  clears the layer's `+0xC8` timer and rebuilds the masks with `0x8007FDA0`.
- **The scroll-layer refresh** (`0x8007A6C0`) and **its clamp** (`0x8007CD70`).
  The first tracks the camera zoom (`+0x9000C`), converts the two BG colours
  into the layer spans, picks the x target from the `0x80078A70`/`0x80078C10`/
  `0x800789A0` bounds and the z target from `0x8007E0E0`, then sorts both row
  tables and calls the clamp. The clamp copies the ten-word rectangle, applies
  the `0x8007BD40`/`0x8007BC40` edge tests, runs the four `0x8015F140` corner
  smoothers and publishes `camera+0x90128`/`+0x9012C`.
- **The two ground sweeps** (`0x80075820`, `0x80075FD0`). Both seed a hit with
  `0x800766F0`; `0x80075820` drops in `8.0` steps and resolves the `0x20` slope
  bit through the `0x802EF3F0` table and `0x802EF548` tile ids, while
  `0x80075FD0` walks in `1.0` steps with the `/8` and `/16` probes and the
  `0x802390D0` slope intersection.
- **The stage-scroll tile walk** (`0x80079E80`). Reads the section record through
  `0x800D90C0`, advances the tile index with the `0x14D` guard, derives the two
  `0x801611C0` angles from the normalised corner segment, smooths them with
  `0x8015F200`, applies the `0x80237C40`/`0x80237BD0` step, and rebuilds the
  depth window and BG spans.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80075820` | `fn_80075820` | 1144 | medium |
| `0x80075FD0` | `fn_80075fd0` | 1368 | low |
| `0x80077700` | `fn_80077700` | 40 | medium |
| `0x80079E80` | `fn_80079e80` | 2072 | low |
| `0x8007A6C0` | `fn_8007a6c0` | 1200 | medium |
| `0x8007CD70` | `fn_8007cd70` | 2108 | low |
| `0x8007FA40` | `fn_8007fa40` | 132 | medium |

Address range: `0x80075820..0x8007FA40` (7 bodies, 8064 bytes).

## Deliberately skipped

The bucket held ten open entries. Seven are landed above; the other three are
skipped on purpose.

- **A branch thunk into an already-translated body** (`0x800776F0`, 4 bytes).
  It is `b 0x80077650`, and `fn_80077650` already has a ledger row in
  `src/game/stage.rs`, so a row here would double-count the target. Same
  precedent as waves 84 and 98-108.
- **The cell-copy wrapper** (`0x800777B0`, 96 bytes). Its second call,
  `FUN_80082140(manager, stage, local_14, in_r6)`, prints only four
  arguments; the fifth (`param_5`, the `u16` value written into the layer) is
  absent from the export, so no honest body can say what is stored. Wave 107
  skipped wrappers with dropped registers for the same reason.
- **The camera depth body** (`0x8007AC40`, 3632 bytes). Every store and branch
  is literal except one: its `FUN_8007bd40(dVar25, &local_118, iVar10, pfVar12)`
  call prints four arguments although `FUN_8007BD40` takes
  `(double, double, float *, int, float *)`. The second double (the vertical
  fit scale used by `fn_8007bd40`) is not in the export and has no other call
  site to recover it from, so the body is left untranslated rather than
  inventing that multiplier.

Seven landed + three skipped = the 10 open entries of the `0x80070000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (7 rows, 8064 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the seven were
  chosen from the 10 open entries of the `0x80070000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`; the remaining crate errors are in other lanes' files.
- `node tools/check-lane-csv.mjs wave109` prints `OK` for this CSV.
- The module header of `src/game/stage.rs` gained a wave-109 paragraph naming
  all seven bodies and the three skips, plus the wave-109 run list in the
  "Original range and ownership" section.
