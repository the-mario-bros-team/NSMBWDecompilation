# Coverage - Wave 100 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records wave 100 of the `main.dol` reconstruction for the
`runtime::os` lane: the matrix and record half of the `0x80240000` bucket. That
bucket holds 137 inventory entries; 35 were untranslated when this pass
started, and 14 of them are reconstructed here.

- **Status** - every row is `translated`: each guard, store and call printed in
  the Ghidra export has a counterpart in `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. `medium` means the
  flow, offsets and calls are explicit but at least one callee role or field
  meaning is inferred; `low` means only the instruction sequence is certain,
  usually because the two halves of a four-byte `0x802DCDxx` handoff have no
  declared parameters.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave100-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 14 | 5,692 | 4 | 3 |

Status split: 14 `translated`, 0 `partial`, 0 `stub`. The remaining seven rows
are `low`: `0x80241C70`, `0x80243000`, `0x802430C0`, `0x80243B30`,
`0x80244190`, `0x80248E00` and `0x8024EAF0`.

## What this pass covers

- **Cluster A - the three 3x4 inverses, `0x80242840`..`0x80242A40`.**
  `0x80242840` and `0x80242930` expand the determinant along the first column
  through the three cofactors, refine `1/det` with the compiler's Newton step
  (`r = -(r*det*r - (r+r))`) and write the nine cofactors in the order
  `C00, C01, C02, C10, C11, C12, C20, C21, C22`; the first writes nine floats,
  the second a twelve-float record with `+0x0C`, `+0x1C` and `+0x2C` zeroed.
  `0x80242A40` is the full affine inverse: the same nine values, laid out as
  rows, plus `-(row . translation)` in the three translation slots. All three
  compare `|det|` against the `f32` at `sda(-0x1098)` and fall back to a copy
  (or, for `0x80242A40`, to no store at all).
- **Cluster B - the two projected inverses, `0x8024EA30`/`0x8024EAF0`.**
  Both reuse the same determinant and then dot the caller's two-row record
  against the first cofactor row. `0x8024EA30` zeroes the third output
  unconditionally; `0x8024EAF0` fills it with the printed expression whose
  second addend repeats the same scale factor.
- **Cluster C - the keyed slot sweeps, `0x80241C70`..`0x80248E00`.**
  The object header is `*holder`; `+0x14` is a relocated key table, `+0x18`
  a `u16` id and `+0x3C` an array of `0x10`-byte entries. `0x80241C70`
  re-resolves every missing key through `0x802422A0`/`0x80242360`/`0x802424A0`
  and reports the all-resolved flag through `0x802DCDB4`; `0x80244190` and
  `0x80248E00` walk the `+0x2C` sub-object and write the `0x8023B7B0` /
  `0x8023B250` id into the `u16` map before calling `0x80249C60(this,4,1)`;
  `0x80243000` mirrors the `+0x38` vtable answer into bit `0x100` of each slot
  record, `0x802430C0` gathers one byte per slot, and `0x80243B30` builds the
  keyed table object itself (size `(count*2 + 0x23) & ~3 + extra*4`, vtable
  words `0x8034E300` -> `0x8034E2B8` -> `0x8034E270`).
- **Cluster D - `0x80244360`, the channel-field writer.** Selector bits `0..3`
  write four words of the object OR-ed with the value they replace; bits
  `4..10` drive the seven channel registers through `0x8023D300`/`0x8023D3C0`
  (three channels) and `0x8023D460`/`0x8023D520` (four channels), then
  `0x8023CA80(channel, 0)` latches. This is the only body in the pass with a
  fully declared argument list.
- **Cluster E - `0x8024DCE0`, the render-state installer.** Twelve steps in a
  fixed order, each taking an override from `overrides[n]` when it is non-zero
  and otherwise deriving the pointer from the header. It is the caller that
  finally ties the Wave 92 flush wrappers (`0x8024B6B0`, `0x8024B990`,
  `0x8024B720`, `0x8024BAE0`, `0x8024BCB0`, `0x8024BEC0`, `0x8024BF70`,
  `0x8024C020`, `0x8024C0E0`, `0x8024C2B0`, `0x8024C8D0`, `0x8024C960`)
  together, which is why it is translated even though it is the largest body
  here.
- **Cluster F - `0x8024EE30`, the orthonormalising rebuild.** `t` is the
  negated translation, `u = column1 x t`, `v = t x u`; the three are written
  back as the matrix columns, scaled by the source record's column lengths
  (one shared length when `mode != 0`). Both guards test every component
  against `sda2(-0x49B4)`, and the degenerate path falls through to the
  `0x80238110` constant filler.

## What the pass adds beyond the raw bodies

1. **The matrix layout is column-major 3x4.** `0x80242A40` reads `+0x00`,
   `+0x10`, `+0x20` as the three columns and `+0x0C`, `+0x1C`, `+0x2C` as the
   translation; `0x8024EE30` writes the same shape and `0x80242840` /
   `0x80242930` read it. The nine output slots of `0x80242840` and
   `0x80242930` are the cofactor matrix in `C00..C22` order, which is the
   transpose-inverse the callers want for a rigid rotation.
2. **The determinant expansion is shared.** All four inverse bodies compute
   `c2x*(c0y*c1z - c1y*c0z) + c1x*(c2y*c0z - c0y*c2z) + c0x*(c1y*c2z -
   c2y*c1z)`, i.e. the scalar triple product of the three columns. The
   remaining cofactors are then formed pairwise from the same nine reads.
3. **`0x802DCD60`..`0x802DCD74` are register saves, not functions.** Their own
   exports store `r26`..`r31` into `r11 - k` and fall through, so the
   `undefined8` a caller receives is the live `r3`/`r4` pair. The Wave 100
   bodies read it with `wave100_pair` and name each half where it is used.
4. **The `GQR` arms are unreachable.** Every float body in this bucket came out
   of Ghidra with `ldexpf(GQR >> 8 & 0x3f)` scale blocks and
   `psq_st`-shaped char/short stores. Those are the decompiler admitting it
   cannot prove the quantisation-register contents; the real stores are
   single-precision floats, so the bodies keep `f32` arithmetic and `write_f32`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80241C70` | `fn_80241c70` | 364 | low |
| `0x80242840` | `fn_80242840` | 228 | high |
| `0x80242930` | `fn_80242930` | 264 | high |
| `0x80242A40` | `fn_80242a40` | 260 | high |
| `0x80243000` | `fn_80243000` | 188 | low |
| `0x802430C0` | `fn_802430c0` | 140 | low |
| `0x80243B30` | `fn_80243b30` | 308 | low |
| `0x80244190` | `fn_80244190` | 248 | low |
| `0x80244360` | `fn_80244360` | 980 | high |
| `0x80248E00` | `fn_80248e00` | 260 | low |
| `0x8024DCE0` | `fn_8024dce0` | 1,172 | medium |
| `0x8024EA30` | `fn_8024ea30` | 192 | medium |
| `0x8024EAF0` | `fn_8024eaf0` | 216 | low |
| `0x8024EE30` | `fn_8024ee30` | 872 | medium |

## Deliberate skips

The other 21 addresses in the bucket were left alone. Eleven were read and
rejected:

1. **`0x80240100` (308), `0x80240240` (380), `0x802403c0` (492).** The three
   spline evaluators quantise the segment index through
   `psq_st`/`psq_l` pairs and Ghidra prints the index as
   `(uint)CONCAT11(cStack_4, uStack_3)` and `CONCAT13(...)` byte merges. The
   index width and the `GQR3`/`GQR6`/`GQR7` scales cannot be separated from the
   C, so the table search would be a guess.
2. **`0x802405b0` (2,660) and `0x8024FEF0` (828).** Both take an
   `undefined8` first argument and forward it to `0x801C0540`/`0x80241020`
   with argument lists that differ between call sites (five and six words for
   the same callee). The register split of the 64-bit parameter is not pinned.
3. **`0x80241690` (1,124).** The seven-case component dispatcher itself is
   clear, but `0x802405B0` is called with five arguments in one arm and six in
   another, and the `0x80241020` triple's third word prints as
   `in_stack_ffffffd0` (a caller-frame slot). Two of the three dispatch arms
   would ship with an invented register.
4. **`0x8024A610` (756).** The fill loop's guards are
   `count = param_2 >> 5` then `param_2 >>= 8` and only loop when the second is
   non-zero, so for `param_2` in `0xE0` the export writes nothing at all. That
   is either a decompiler artefact or a real early-out; the byte range cannot
   decide it, and a loop that silently writes nothing is not worth shipping.
5. **`0x80245B40` (500) and `0x802471E0` (428).** Both pass eight and nine
   arguments to `0x801C6D90`/`0x801C6FA0`/`0x801C6FF0` and take an untyped
   paired-single argument (`in_f31`/`in_ps31_1`) from the caller. The pair
   packing is a `GQR` type selection, so the argument order is a guess.
6. **`0x8024A6E0` (984) and `0x8024ABD0` (1,088).** The two basis builders are
   the only bodies in this bucket that chain *four* untyped `0x801C05xx`
   routines per call, and the second one's `0x801C1570` output is consumed by
   an `0x801C0540` whose first argument is the destination; two different
   readings of that routine's parameter order both reproduce the C. They need a
   pass that owns `0x801C0500`..`0x801C15xx`.

The remaining ten (`0x80241EB0`, `0x80243D20`,
`0x80245300`, `0x80245500`, `0x80245970`, `0x80246900`, `0x80246B00`,
`0x80246F70`, `0x80249740`, `0x8024E180`) were not opened, so no claim is made
about them.

No address in this pass was already present in `ledger/functions.csv`; the 102
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (14 of 14; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes.
- `node tools/check-lane-csv.mjs wave100` prints `OK` with `wave100-os.csv`
  counted at 14 rows.
