# Coverage - Wave 107 (`game::stage`, the `0x80070000` actor/ground-trace band)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave107-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

Three conventions matter for this band:

* **The `__savegprlr_NN` prologues.** `0x802DCD50..0x802DCD74` return their
  saved `r3`/`r4` pair through a `u64`, so a body that opens with
  `uVar = FUN_802dcdNN()` is reading its own first two arguments back. The
  angle arrives separately in `f1` and is printed as `extraout_f1`. Every body
  below keeps that reading, the same one `fn_800722c0`, `fn_80071060`,
  `fn_80073b40` and `fn_800743d0` already use.
* **The paired-single prologue is dead.** The `GQR0`/`ldexpf` blocks at the top
  of `0x800728C0`, `0x80074470`, `0x80074AF0`, `0x80075CA0` and `0x8007FFA0`
  only store the non-volatile `f14..f31` registers to the stack and reload them
  at the exit. Their locals are never read by the body, so they are not
  reproduced.
* **The fixed-point folds are literal.** `CONCAT44(0x43300000, w) - 2^52(+2^31)`
  and the `& 0xFFFF` / `& 0xFFF0` variants are the compiler's integer-to-double
  idiom around a raw word. They are reproduced as `(word as i32) as f32`,
  `(word & 0xffff) as f32` and `(word & 0xfff0) as f32`, exactly as
  `fn_80075560` and `fn_800714d0` already do.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 6 | 6016 | 0 | 5 |

Five rows are `medium` and one (`0x8007FFA0`) is `low`; none is `partial`. The
pass covers `0x800728C0..0x8007FFA0`.

## What this pass covers

- **The feet probe** (`0x800728C0`). One `0x80072790` query seeds the search
  (`feet + r2-0x73A8`, or the returned segment height), then the loop samples
  `r2-0x743C` apart with `0x80070BA0`/`0x80070BF0`. A `0x40` kind is a slope
  tile built from the `0x802EF488` normal pair, a `0x4899` kind is snapped by
  the `0x80070A40` answer (`row + 8` / `row + 0x10`), and the found height is
  clamped up to the feet before it is written out.
- **The two world scans** (`0x80073FC0`, `0x80074AF0`). These are the callees
  that `fn_80073b40` and `fn_800743d0` already reach through
  `W91_CALL_80073FC0` / `W91_CALL_80074AF0`, so the signatures are fixed by
  those call sites: one `f64` angle plus two (or three) integer words. Both
  rebase the entry's decoded triplet on the actor's `+0x14` position, walk the
  `r13-0x58A0` scroll list through `0x80080E40`, and run the `+0xD0` attribute
  ladder (`0x200`, `0x100`, `0x80`, `0x400`, plus the lane-scaled bits in the
  `0x80074AF0` variant). A kind of `0`..`2` then runs the two virtual hooks
  (`[player+0x60]+0x68` and `[entry+0x44]`/`[entry+0x48]`). The `0x80073FC0`
  variant finally pulls the actor y to at most one step below its old value;
  the `0x80074AF0` variant keeps the actor x within `1.0f` of its wrapped
  position.
- **The segment height solver** (`0x80074470`). Three arms: the `+0xC8 == 1`
  circle arm (probe stepped by `0x8006D120` between `point[1]` and
  `point[2]`, radius remainder from `0x80237BA0`), the `+0xBC` node word `== 0`
  box arm (three coordinate windows against `+0x60..+0x74`), and the segmented
  arm, whose node word selects one of the four `0x802EF648` offsets and whose
  interpolation is kept when the `0x80237ED0` bearing falls outside the
  `0x638E` window.
- **The vertical trace** (`0x80075CA0`). The same slope/snap ladder as the feet
  probe, but with the start clamped to `0.0f` and the `0x800766F0` confirmation
  replacing it before the walk.
- **The facing-relative step** (`0x8007FFA0`). Scales the input delta by
  `+0xD8` in mode `0`, rotates it by `base + atan2(+0x34/+0x3C)` when the
  owner's `+0xC2` angle is set, wraps the x through `0x80101AA0` and mirrors
  the delta into the body's `+0x310`/`+0x314`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800728C0` | `fn_800728c0` | 768 | medium |
| `0x80073FC0` | `fn_80073fc0` | 1040 | medium |
| `0x80074470` | `fn_80074470` | 1656 | medium |
| `0x80074AF0` | `fn_80074af0` | 1280 | medium |
| `0x80075CA0` | `fn_80075ca0` | 812 | medium |
| `0x8007FFA0` | `fn_8007ffa0` | 460 | low |

Address range: `0x800728C0..0x8007FFA0` (6 bodies, 6016 bytes).

## Deliberately skipped

The bucket held 16 open entries. Six are landed above; the other ten are
skipped on purpose.

- **A branch thunk into an already-translated body** (`0x800776F0`, 4 bytes).
  It is `b 0x80077650`, and `fn_80077650` already has a ledger row, so a row
  here would double-count the target. Same precedent as waves 84 and 98-106.
- **Three wrappers whose values are undefined in the export** (`0x80077700`,
  `0x800777B0`, `0x8007FA40`). `0x80077700` returns `extraout_r4` after calling
  the void `fn_80077650`; `0x800777B0` leaves the fifth argument of `0x80082140`
  unprinted and its `local_14` is never written; `0x8007FA40` passes `in_r10` to
  `0x8007FBF0` and takes its first float from `extraout_f1`. In each case the
  value is not defined by the export, so writing it would be invention.
- **Two sweeps whose area and out registers the export conflates**
  (`0x80075820`, `0x80075FD0`). In both, Ghidra prints `FUN_80070a40(x, y,
  param_3)` — using the *out float pointer* as the area byte — while the same
  `param_3` receives `*param_3 = ...` stores later in the body. The plane block
  that `0x802390D0` receives in `0x80075FD0` is built from those same aliased
  locals. The correct split of `r5..r8` (area, mask, out float, out word) cannot
  be stated from this export without inventing it.
- **Four large state machines, not attempted this pass** (`0x80079E80`,
  `0x8007A6C0`, `0x8007AC40`, `0x8007CD70`). These are 1200-3632-byte bodies
  driving the `0x8008xxxx` stage-block helpers. `0x8007A6C0` additionally
  consumes the `f1` result of `fn_80078eb0`, whose Rust signature in this file
  returns nothing, and `0x8007AC40`/`0x8007CD70` save fourteen to eighteen
  float registers and use them as scratch. They look reconstructible in
  principle, but this pass did not have the budget to read and land them
  carefully, so no row is claimed for them.

None of the ten skipped entries is counted as translated; 6 landed + 10
skipped = the 16 open entries of the `0x80070000` bucket.

## Verification

- Every address and byte size matches `functions.tsv` (6 rows, 6016 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the six were
  chosen from the 16 open entries of the `0x80070000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the one remaining crate error is in `src/game/player.rs`,
  which belongs to another wave-107 lane).
- `node tools/check-lane-csv.mjs wave107` prints `OK`; the wave-107 stage rows
  are clean.
- The module header of `src/game/stage.rs` gained a wave-107 paragraph naming
  all six bodies and the ten skips.
