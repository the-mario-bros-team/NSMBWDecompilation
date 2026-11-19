# Coverage - Wave 103 (`game::stage`, the `0x80070000` contact and tile-cell band)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave103-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

Two conventions matter for this band:

* Every body starts after the `__savegprlr_NN` register-save helper that Ghidra
  prints as a function call (`0x802DCD5C..0x802DCD74`). Those helpers preserve
  `r3`/`r4`, so the `extraout_r4` a body reads is the caller's own second
  argument, not a callee result. This pass relies on that same reading that
  `fn_800722c0` and `fn_80071060` already use.
* Every `ldexpf` block in the prologue writes a dead stack local. The data-free
  GQR0 branches are therefore not reproduced.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 9 | 6260 | 0 | 9 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`. The pass covers `0x80070130..0x8007E4E0`.

## What this pass covers

- **The contact-query family** (`0x80073290`, `0x80073BC0`, `0x80073670`).
  `0x80073290` tests one scroll-layer entry three ways: the `+0xC8 == 1` circle
  arm (radius `+0x90` around `+0x58`, height from `0x80237BA0`, bearing from
  `0x801611C0`), the `kind == 0` box arm, and the slope arm that rewrites the
  caller's probe in place before `0x80080880`/`0x80080670` and interpolates the
  edge row. `0x80073BC0` is its walking twin: the same circle arm, then a
  `0x8006D120` cursor march that keeps the lowest normal y. `0x80073670` is the
  caller: it decodes the trigger record with `0x80072DB0`, sweeps `r13-0x58A0`
  with `0x80080E40`, and runs the full `+0xD0` attribute ladder with
  `0x80070760`, `0x80081030`, `0x80070600`, `0x800731E0` and the two virtual
  hooks (`[player+0x60]+0x68` and `entry[0x10]`).
- **The cell-walk pair** (`0x80070130`, `0x80071A60`). `0x80070130` classifies
  the point with `0x80070BA0`/`0x80070BF0`, then runs the four `0x8006Dxxx`
  tests, nudges the actor x by up to `0x2000` of a cell and sets the
  direction/mode bits before `0x8006D460`. `0x80071A60` orders the trigger
  record's `+4`/`+8` words, probes the two grid points, matches the
  `0x18`/`0x1A` kind row and snaps the coordinates to nibbles before
  `0x800CFEA0` looks the stage object up.
- **The slide and terrain updates** (`0x80071210`, `0x8007E4E0`).
  `0x80071210` rebuilds the `+0x2C`/`+0x30` deltas, clamps the y delta, merges
  `0x80070E70`/`0x80070EF0` and applies the `sin/cos` slope nudge. `0x8007E4E0`
  copies the scene frame into `+0x20..+0x34` and settles every `0xC`-byte
  terrain entry through `0x8007EBC0`/`0x8007EBF0`/`0x800B3100`.
- **The BG tile decode pair** (`0x80077730`, `0x80077810`). `0x80077730` turns
  a packed `x` word and tile token into the flat layer index, the layer value
  (`bg_layer_u16`) and `tile_mask_value`; `0x80077810` is the `(tile & mask) != 0`
  predicate built on it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80070130` | `fn_80070130` | 916 | medium |
| `0x80071210` | `fn_80071210` | 408 | medium |
| `0x80071A60` | `fn_80071a60` | 860 | medium |
| `0x80073290` | `fn_80073290` | 984 | medium |
| `0x80073670` | `fn_80073670` | 1104 | medium |
| `0x80073BC0` | `fn_80073bc0` | 1020 | medium |
| `0x80077730` | `fn_80077730` | 120 | medium |
| `0x80077810` | `fn_80077810` | 68 | medium |
| `0x8007E4E0` | `fn_8007e4e0` | 780 | medium |

Address range: `0x80070130..0x8007E4E0` (9 bodies, 6260 bytes).

## Deliberately skipped

The bucket held 25 open entries. Nine are landed above; the other sixteen are
skipped on purpose.

- **Already translated targets behind branch thunks** (`0x800776F0`, 4 bytes).
  It is a `b 0x80077650` thunk and `fn_80077650` already has a ledger row, so a
  row here would double-count the body. Same precedent as waves 84 and 98-102.
- **Wrappers whose return value is a leftover register** (`0x80077700`,
  `0x800777B0`, `0x8007FA40`). `0x80077700` returns `extraout_r4` after calling
  the void `fn_80077650`; `0x800777B0` leaves the 5th argument of `0x80082140`
  unprinted; `0x8007FA40` passes `in_r10` to `0x8007FBF0` and takes its first
  float from `extraout_f1`. In each case the value is not defined by the export,
  so writing it would be invention.
- **Live GQR3 quantisation** (`0x800728C0`, `0x8007FFA0`). `0x800728C0` folds
  the result of a `ps_mul` through
  `(double)(CONCAT44(0x43300000, x) & 0xffffffff0000ffff ^ 0x80000000)` and
  `& 0xffffffff0000fff0`, i.e. the mask is applied to the *low* word of the
  double idiom. `0x8007FFA0` converts a `s16` angle through a live `GQR3`
  whose quantisation mode is a register, and the decompiler prints stack garbage
  as the low half. Both are the shape wave 102 deferred.
- **Large stage-block handlers, not attempted this pass** (`0x80074470`,
  `0x80074AF0`, `0x80075820`, `0x80075CA0`, `0x80075FD0`, `0x80079E80`,
  `0x8007A6C0`, `0x8007AC40`, `0x8007CD70`). These are 812-3632-byte bodies
  with five to eight arguments that drive several still-untranslated `0x8008xxxx`
  callees. They look reconstructible in principle, but this pass did not have
  the budget to read and land them carefully, so no row is claimed for them.
  `0x8007A6C0` additionally consumes the `f1` result of `fn_80078eb0`, whose
  existing Rust signature returns nothing.

None of the sixteen skipped entries is counted as translated; 9 landed + 16
skipped = the 25 open entries of the `0x80070000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (9 rows, 6260 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the nine were
  chosen from the 25 open entries of the `0x80070000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the remaining errors belong to other lane files).
- `node tools/check-lane-csv.mjs wave103` prints `OK`; the wave-103 stage rows
  are clean.
