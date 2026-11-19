# Coverage - Wave 105 (`game::stage`, the `0x80090000` remainder)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave105-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

Four conventions matter for this bucket:

* **The `__savegprlr` pack.** `0x802DCD74` returns the caller's `r3:r4` pair, so
  a body that opens with `iVar1 = FUN_802dcd74();` is reading its own first
  argument out of the low half. `0x800925F0` goes one step further: its two
  words are handed to `0x80092370` in the order `(r4, set, r3, r5, r6)`, which
  the body spells out as `(key, set, owner, id, mode)`.
* **The GQR prologues are dead.** Every `ldexpf`/`unaff_GQR0` block in these
  exports only rescales `in_f30`/`in_f31` into stack slots that are never read
  afterwards, and the `CONCAT44(0x43300000, x ^ 0x80000000) - 2^52` chains are
  the signed-int-to-double idiom. They are reproduced as `i32`/`i16`/`i8` casts,
  not as paired-single state.
* **The `Wm` actor layout.** `0x524`-byte object built by `0x80094E80`:
  `+0x60` vtable, `+0xAC`/`+0xB0`/`+0xB4` world position, `+0x1EC` collision
  sub-object, `+0x38F`/`+0x390`/`+0x391` layer/side bytes, `+0x440..+0x454`
  motion accumulators, `+0x45A..+0x45E` state bytes, `+0x510` mode word. The
  `Wm_jr` sub-class (wave 92) adds `+0x600` model set, `+0x604` animation node
  and the `+0x644`/`+0x67C`/`+0x700`/`+0x72C` channels.
* **The effect tables.** `0x80093530`/`0x80093A50` sample the camera once
  (`0x80164B30` → `0x80253990`/`0x802540B0`/`0x80253E60`) and then submit the
  same eight-float matrix under sixteen and twelve ids respectively.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 17 | 5772 | 2 | 10 |

Five rows are `low` confidence (the four vtable tail stubs and the one body whose
probe point is dropped by the export); none is `partial`. The pass covers
`0x80090440..0x8009BAD0`.

## What this pass covers

- **The animated block's segment rebuild** (`0x80090440`). One loop over the
  `+0x2D0` segments rotates the `+0x230 + i*0x1C` position by the `+0x34`
  triple, builds the translation matrix with `0x801C0BD0`, applies the three
  axis rotations (`0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0`), binds it to the
  `+0x80 + i*0x40` model and, for variants 4/5, stores three words per segment
  from the model's 0x30-byte output block.
- **The stage-effect animation set constructor** (`0x800925F0`). It allocates
  0x18C bytes, installs `0x8031133C`/`0x80311368`, appends the link at `+0x54`
  to the `r13-0x5790` list, builds the three counted arrays (`0x24` words at
  `+0x60`, `0x1C` at `+0xF0`, `8` at `+0x16C`) and registers the set through
  `0x80092370`, returning its truth value through `0x802DCDC0`.
- **The two effect-table builders** (`0x80093530`, `0x80093A50`). Both sample
  the camera block once and then rebuild the 48-byte transform for every
  instance; `0x80093530` also gates ids `1`/`0`/`5` on `r13-0x5986` bit 0 and
  each `0xB`/`7` slot on `0x80061080` returning zero, while `0x80093A50` gates
  ids `1`/`0` on the `+0x394` byte behind `r13-0x5704` and skips id `5`.
- **The `Wm` motion and query helpers** (`0x800955F0`, `0x800957B0`,
  `0x800963B0`, `0x800976B0`, `0x800978C0`, `0x80097A20`). These are the orbit
  step with its freeze branch and three velocity accumulators, the damped-speed
  pair, the collision-side/layer push (including three stores the export never
  reads back), the airborne rest-word clear, the four-slot proximity box test
  and the player-bone target point.
- **The vtable tail stubs and the wrapper constructor** (`0x80099F80`,
  `0x80099FA0`, `0x80099FE0`, `0x8009A000`, `0x8009AEC0`). The four 16-byte
  bodies are the `+0x16C`/`+0x170` slots of two sibling classes; Ghidra prints
  them as unrecovered jump tables, so the dispatch is reproduced as a
  one-argument call with `this` still in `r3`.
- **The `Wm_jr` model and aim helpers** (`0x8009B070`, `0x8009BAD0`). The first
  loads `g3d/koopaJr.brres` into `+0x600`, builds the `0x16B` node, rejects a
  non-matching `0x802F8FF0` parent, installs the `damage_s` texture and then
  builds the optional second model with its `+0x72C`/`+0x700` blocks. The second
  samples the player twice through slot `+0x70`, derives the two aim angles with
  `0x802BE470`/`0x801611C0`, reduces them by both objects' bone angles and
  clamps them to `[slot +0x400, 0x2000]` and to the `+0x348`-selected pair of
  the `r2-0x6EE0` direction table.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80090440` | `fn_80090440` | 332 | medium |
| `0x800925F0` | `fn_800925f0` | 268 | medium |
| `0x80093530` | `fn_80093530` | 1312 | medium |
| `0x80093A50` | `fn_80093a50` | 1020 | medium |
| `0x800955F0` | `fn_800955f0` | 444 | medium |
| `0x800957B0` | `fn_800957b0` | 212 | medium |
| `0x800963B0` | `fn_800963b0` | 176 | medium |
| `0x800976B0` | `fn_800976b0` | 76 | low |
| `0x800978C0` | `fn_800978c0` | 236 | high |
| `0x80097A20` | `fn_80097a20` | 264 | medium |
| `0x80099F80` | `fn_80099f80` | 16 | low |
| `0x80099FA0` | `fn_80099fa0` | 16 | low |
| `0x80099FE0` | `fn_80099fe0` | 16 | low |
| `0x8009A000` | `fn_8009a000` | 16 | low |
| `0x8009AEC0` | `fn_8009aec0` | 60 | high |
| `0x8009B070` | `fn_8009b070` | 864 | medium |
| `0x8009BAD0` | `fn_8009bad0` | 444 | medium |

Address range: `0x80090440..0x8009BAD0` (17 bodies, 5772 bytes).

## Deliberately skipped

The bucket held 22 open entries. Seventeen are landed above; the other five are
skipped on purpose.

- **Already-translated target behind a 4-byte branch thunk** (`0x80096700`).
  Its export is `thunk_FUN_800967f0` and `0x800967F0` already has a ledger row in
  `game::stage`, so a row here would double-count that body. Same precedent as
  waves 84, 98-104.
- **Paired-single body with a dropped register** (`0x80093F10`). The body opens
  with two `GQR0` dequantisation blocks that only fill `local_8`/`local_18`, and
  the surviving code closes with `FUN_8016c850(&local_e0,
  (int)((ulonglong)uVar12 >> 0x20), iVar8)`, where the high half of the
  `__savegprlr` return is a second entry register the export never defines. Both
  `iVar8` (the player index used by the `switch`) and that high word are needed
  for the call, so the argument list cannot be stated without inventing one.
- **Forwarder whose two callees are only reachable through lost registers**
  (`0x80097170`, `0x80097250`). `0x80097170` is called as
  `(f64, this, this+0xAC)` by `0x8002E440`, but its own body calls
  `FUN_80097250()` with no argument list, and `0x80097250` then dereferences
  `param_3` (a pointer the frame never sets) and forwards an eight-byte
  `param_1` to three vtable slots. Reproducing the chain would require guessing
  `r5`/`r6`, which is exactly the case wave 104 declined for `0x800B96A0`.
- **Body that mixes a lost register with `extraout_f1`** (`0x80098900`). The
  `FUN_80196660` chain carries an `extraout_f1` that Ghidra never sources, and
  the entry pair is `(r3 = mode 2, r4 = this)` - a call shape with no other
  evidence in the tree - so neither the signature nor the float lineage can be
  written honestly.

None of the five is counted as translated; 17 landed + 5 skipped = the 22 open
entries of the `0x80090000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (17 rows, 5772 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the seventeen
  were chosen from the 22 open entries of the `0x80090000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the remaining errors belong to other lane files).
- `node tools/check-lane-csv.mjs wave105` prints `OK`; the wave-105 stage rows
  are clean.
