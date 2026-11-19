# Coverage - Wave 122 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 122 pass over the `0x80290000..0x8029FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unloaded `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave122-gx-band.csv` (one row,
`translated`, `medium`).

The bucket held two untranslated entries when this pass started:

| address | bytes | state after this pass |
|---|---:|---|
| `0x80292C90` | 5,464 | open - see "Deliberate skips" |
| `0x8029F900` | 3,392 | **landed** - per-node billboard drawer |

The `0x80290000` bucket is now 84 of 85 translated (the remaining entry is
`0x80292C90`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 3,392 | 0 | 1 |

## What this pass covers

- **The per-node billboard drawer `0x8029F900`** (3,392 bytes, `medium`). It is
  the per-vertex painter paired with [`fn_802a1090`]: `0x802A16D0`'s GX state
  block runs first (inlined in the original, called through the existing body
  here), then the effect record is expanded by `0x8029CFC0`, and the scene
  vtable yields three factories - slot `+0x18` the per-node draw callback,
  `+0x10` the first-node iterator and `+0x14` the next-node iterator. The node
  list at `descriptor+0x3C` is walked with the `u16` stride at `descriptor+0x42`.
- **Size selection.** Per node the drawer builds
  `scale_x = node[0x30] * node[0x38] * object[0x58]` and a second factor
  selected by `(*(object+0x24))[0x9C] & 0x6000`: `0x4000` uses
  `node[0x34]*node[0x38]`, `0x2000` uses `node[0x30]*node[0x3C]`, `0x6000`
  reuses `node[0x30]*node[0x38]` (the earlier product is still live), and every
  other value uses `node[0x34]*node[0x3C]`. Either factor below
  `FLT_EPSILON` skips the node.
- **Optional chord scale.** When `source[0x14C]` is set, the length of
  `node[0xAC..0xB4] - node[0xB8..0xC4]` is computed with the same
  `ps_mul`/`ps_madd`/`ps_sum0` dot used elsewhere in the module; an exact length
  of `1.0` skips `0x80237BA0`, otherwise `length2 * rsqrt(length2)` is used.
  The scale becomes `1.0 + 0.5 * length / scale_y`.
- **Material and Euler matrix.** `0x8029BAB0(scene, node, source+0x9C,
  context, first, 0)` emits the material state and clears the `first` flag. The
  phase bytes `source[0x145]`/`source[0x146]` are signed and divided by `100.0`.
  The three position/angle sums (`node[0x40..0x48]`, `byte * 0.024543693` for
  `node[0x9A..0x9C]`, then `object[0x60..0x68]`) feed `0x802A0A90` with the
  `source[0x14B]` axis selector - that block is the inlined body of the existing
  [`fn_802a0a90`].
- **Side matrix.** Two arms selected by `source[0x14D]` build a 3x4 side matrix
  from the Euler matrix and the two scale factors; the `0` arm puts the track
  scale on the y slot (`scale_y * scale`) and the `1` arm uses
  `scale_y * phase_y` and folds `-phase_y` into entry `[11]`.
- **Basis rebuild and quads.** The draw callback writes a three-float
  reference. `0x801C1530` crosses the node basis with it; a squared length
  below `FLT_EPSILON` falls back to the **normalised z column** (stack `0xB0`),
  not to the raw y column. A second cross with the reference gives the third
  axis, which is written back to `node+0xCC..+0xD4`. The local matrix rows are
  `(axis2, reference, axis0, node[0xAC..0xB4])`; `0x801C0540` composes it with
  the side matrix and `0x8029F4A0` emits the quad with the `0x802FCC30` point
  table, or also with `0x802FCC60` when `source[0xAD] == 1`.

## The export defects this pass resolves

The wave-120 note left `0x8029F900` open because of a "two-branch basis builder"
whose stack slots were re-used by the GQR ladders. Reading
`work/dol/main.elf` settles all three defects:

- **The pseudo-return is `__savegprlr_18`.** `bl 0x802DCD50` at `0x8029F980`
  only stores `r18..r31`; it never writes `r3..r5`. The export's 64-bit
  `FUN_802dcd50()` result and its `piVar17`/`puVar22` split are artefacts. The
  real arguments are `r3` = scene, `r4` = context, `r5` = descriptor, and the
  body uses them directly.
- **The basis builder is a matrix load, not an ambiguous normaliser.**
  `0x8029FC04..0x8029FC1C` is `PSMTXConcat(matrix, record+0x38, dst)` followed
  by `0x801C9940(dst, 0)` (`GXLoadPosMtx`). The `y`/`z` columns are then read
  from `record+0x6C/+0x7C/+0x8C` and `record+0x70/+0x80/+0x90` (mode `7`), or
  from `(...+4/+0x14/+0x24)` and `(...+8/+0x18/+0x28)` of the parent matrix
  produced by `0x80289070` (all other modes). Only the `z` column is
  normalised.
- **Every `psq` operand is GQR0.** All 54 `psq_l`/`psq_st` sites in
  `0x8029F900..0x802A063C` decode with `I = 0`, and `0x801AB150` leaves GQR0 at
  plain float, so each pair is two `f32` lanes and every `ldexpf`/`CONCAT`
  ladder in the export is dead.

The constant block used by the body (`r2 = 0x80433080`): `0x8042EF78 = 0.0`,
`0x8042EF7C = FLT_EPSILON`, `0x8042EF80 = 1.0`, `0x8042EF84 = 100.0`,
`0x8042EF88 = 0.5`, `0x8042EF8C = 0.024543693` (`2*pi/256`),
`0x8042EF90 = 2^52` (signed conversion) and `0x8042EF98 = 2^52` (unsigned).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8029F900` | `fn_8029f900` | 3392 | medium |

## Deliberate skips

### `0x80292C90` (5,464 bytes)

Still open. The 3,425-line export keeps 106 removed blocks spread across the
live body (not just the prologue) and 38 `pcVar` temporaries; the raw prologue
alone calls `0x802DCD40` and then a list lookup at `0x80228FF0(this+0x38,
this[0x54])`, with the first live branch at `0x80294170`. The body is a
per-vertex sampler plus a virtual-slot dispatch, not a leaf like
`0x8029F900`, so closing it needs a full raw control-flow rebuild (~1,366
instructions) rather than resolving one ambiguous block. It is documented here
as a skip rather than guessed; the lane's open remainder is this entry plus
`0x802A71D0` and `0x802A90C0` (outside the `0x80290000` bucket).

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 3,392 bytes).
- The evidence file exists at the cited path:
  `work/ghidra/main-gb/export/8029f900_FUN_8029f900.c`.
- Every row has exactly one `pub unsafe fn fn_8029f900` body in
  `src/render/gx_band.rs`; the module now holds 342 reconstructed entries
  (341 full, one partial).
- `0x8029F900` was not in `ledger/functions.csv` before the pass; the whole
  `0x80290000` bucket was re-read (85 entries, these two open).
- `node tools/check-lane-csv.mjs wave122` reports `OK` for this lane's CSV.
- `cargo check --message-format=short` reports no error and no new warning from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings in
  `fn_8028cee0` predate this pass.
- The whole body was transcribed from `work/dol/main.elf`; every cited branch,
  constant and store was checked against the raw stream, and the register map
  (`r20` scene, `r21` context, `r22` descriptor, `r23` node, `r24` first flag,
  `r25` next factory, `r27` draw callback, `r28` `source+0x9C`) was read from
  the raw prologue and loop.
