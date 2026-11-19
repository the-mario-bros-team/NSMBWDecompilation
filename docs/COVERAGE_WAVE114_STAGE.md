# Coverage - Wave 114 (`game::stage`, closing the `0x800B0000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave114-stage.csv` is the Ghidra C export each body was
written from.

The bucket opened with 235 entries and 231 already had `game::stage` rows, so
wave 114 starts from four open entries. Two are landed here and two are
deliberately skipped, which closes the bucket.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 2 | 1352 | 0 | 2 | 0 |

Both rows are `translated`; none is `partial` or `stub`. The pass covers
`0x800B96A0..0x800BC140`.

## What this pass covers

- **The node-chain layout pass** (`0x800B96A0`, 1008 bytes). A method of the
  lift element (one `r3` argument; the `0x802DCD6C` save helper only preserves
  `r25..r31`, which is why Ghidra declared it `(void)` and then reused the
  register as `iVar9`). It walks `this+0x20` for `this+0x1C` nodes over the
  `+0x88` link and rewrites each node's `+0x68..+0x70` (running point) and
  `+0x74..+0x7C` (next point), then calls [`fn_800bae60`]. The geometry comes
  from the `+0x24` source record: anchor `+0x62C/+0x630`, walk start
  `+0x638/+0x63C/+0x640`, and the `r2-0x69E0` plane height that every node
  receives in z. One rotation matrix (from the `u16` angle picked by the
  `[+0x62C] <= [+0x638]` flag in the `r2-0x69E4` pair) is concatenated with a
  `(0, -sin, -cos)` translation built from the fixed `0x3A00` angle and
  decomposed by `0x8016F130` into the running offset. Each iteration then takes
  one of three paths:
  - the alternating branch re-places both ends on either side of the anchor x
    by `r2-0x69D8 * 1.0`;
  - the extend branch moves the point by the current offset;
  - the bend branch (taken when the point would cross the anchor y) derives
    `d = (P.y - A.y) / 1.0`, takes `r2-0x69E8 - d*d` (through `0x80237BA0`
    when positive) as the curvature, rebuilds the offset twice and marks the
    walk as bent.
  The final `(P.x - A.x) * (Q.x - A.x) < 0.0` test latches the alternating
  branch.
- **The named-node re-anchor** (`0x800BC140`, 344 bytes). It resolves
  `record`/`index` through `0x8023B300`, turns the `+8` word into a name string
  and compares that name against the four pointers at `0x802F0D40`; any other
  name returns immediately. On a match it copies the caller's 0x30-byte matrix
  (the second argument of `0x801C0500` is dropped by the export; the stack
  buffer is the block whose translation column is `local_44/34/24`), subtracts
  the `this+4` node position, perspective-projects the delta with
  [`fn_800bbdd0`], adds the node position back into the matrix translation,
  builds a uniform scale matrix with the projection factor `0x800BBDD0` leaves
  in `f1` (`0x801C0C60(s, s, s, &m)`) and multiplies it in before copying the
  result back. The `addic. r4,r1,0x60 / beq` pair Ghidra prints as
  `if (&stack0x00000000 != (undefined1 *)0x50)` is a dead null test on the
  local destination and is noted, not reproduced as a branch.

The two `psq_st`/`psq_l` runs over `f25..f31` in `0x800B96A0` are the usual
frame-save artefact (nothing reads the spilled slots), and the fixed `0x3A00`
angle is reproduced as the word Ghidra prints for the `GQR3` dequantisation.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B96A0` | `fn_800b96a0` | 1008 | medium |
| `0x800BC140` | `fn_800bc140` | 344 | medium |

Address range: `0x800B96A0..0x800BC140` (2 bodies, 1352 bytes).

## Deliberately skipped

Two of the four open rows are not landed. Both are 4-byte branch thunks whose
exports are the *target* body, the same precedent as `0x80096700` in wave 113
and the thunks of waves 84 and 98-112:

- **`0x800B4600`** (`thunk_FUN_800b4610`). Its export is the 4-byte branch into
  `0x800B4610`, which already owns a `game::stage` row (`fn_800b4610`,
  `high`). A second body would double-count the same code.
- **`0x800B54B0`** (`thunk_FUN_801dd1d0`). Same shape: it branches to
  `0x801DD1D0`, which is already translated as `runtime::dvd::fn_801dd1d0`
  (`high`) and is called by address from several stage bodies.

Two landed + two skipped = the four open entries of the `0x800B0000` bucket,
which is now closed.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 1352 bytes;
  `0x800B96A0` = 1008, `0x800BC140` = 344).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the two were
  chosen from the four open entries of the `0x800B0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave114` reports this CSV clean.
- The module header of `src/game/stage.rs` gained a wave-114 paragraph naming
  both bodies (1352 bytes) and the two skips, plus a wave-114 line in the
  original-range list; no earlier count became stale.
