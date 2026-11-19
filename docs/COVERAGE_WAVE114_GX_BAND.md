# Coverage - Wave 114 (GX / render band, `0x802A0000` bucket)

## How to read this file

This page covers the Wave 114 pass over the last open entries of the
`0x802A0000..0x802AFFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave114-gx-band.csv` (three rows).

The bucket had seven untranslated entries when this pass started. Three are
landed here; the other four are documented under "Deliberate skips" with the
concrete blocker that remains for each.

## The finding that re-opened this bucket

Wave 101 deferred all seven entries as "live quantised" code because their
exports contain `unaff_GQR0` type ladders. That description was too wide. A raw
scan of the 20 `psq_l`/`psq_st` operands in `0x802A1090` (all stack spills), the
18 in `0x802A4400` and the 18 in `0x802A64B0` shows every operand carries
`I = 0`, i.e. it addresses `GQR0`.
`0x801AB150` zeroes `GQR0..GQR7` (type 0 = float, scale 0) and the only other
`GQR0` writer, `0x802EA6FC`, is the matching save-area restore; none of the
three entries writes a GQR itself. Every one of those ladders therefore
collapses to its final `float` arm, and the `ldexpf` text left in the export is
either an entry/exit register spill or a plain float-pair move.

This mirrors the Wave 109 finding for the `0x80290000` bucket. For the three
entries landed here the collapse is exact: every field they read is a plain
`float` in the neighbouring already-translated record builders, and every
`psq_st` outside the prologue is a float-pair store to a stack temporary or to
the 0x28-byte record.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 3 | 3,468 | 0 | 3 |

## What this pass covers

- **The particle node-list drawer `0x802A1090`** (1,588 bytes, `medium`). It
  resolves the GX material for `descriptor` (`0x802A16D0`), copies the context
  matrix (`0x802A0660`) and expands it into the four matrices consumed by the
  per-node callback (`0x8029CFC0`). The three virtual slots are factories: slot
  `+0x18` is called once with `(scene, descriptor)` and its result is the
  per-node callback `(out_vector, matrices, node)`; slot `+0x10` is called with
  `(scene, flag, 100.0, phase_x/100, phase_y/100)` and returns the first-node
  iterator; slot `+0x14` returns the next-node iterator. Both protocols are
  visible in the raw stream at `0x802A1254..0x802A12B4`.
  For each node the body walks the `+0xCC` length guard, computes the two
  volume factors `0x802A0950`/`0x802A0970`, optionally enlarges the scale by
  half the chord length over the second factor, emits the material state
  (`0x8029BAB0`, first call forced) and builds the oriented rectangle
  (`0x802A0A90` + `0x802A0E50`). The callback vector is transformed by the world
  matrix (`0x80238420`), the tangent basis comes either from `node+0xCC` or from
  the `(-x*z, -y*z, x*x + y*y)` cross product, and the resulting 3x4 matrix is
  projected into three corner vectors. Two `0x8029F640` quad emissions follow,
  the second only when `u8[source+0xAD] == 1`.
- **The `1/n`-stepped strip driver `0x802A4400`** (952 bytes, `medium`). This is
  the `+0x14E & 7 == 2` strip mode dispatched by `0x802A47C0`. It reads the
  descriptor from `*ctx`, the render source from `descriptor+0x24`, the phase
  bytes at `source+0x145`/`+0x146` (scaled by `0.01`) and the frame count at
  `descriptor+0x50`. When the format byte is not `0x40` the step is
  `-(n*(1/n)^2 - (2/n))`, which reduces to `1/n`; otherwise it is the `1.0f` at
  `0x8042EFEC`. The direction and start index come from bit 11 of
  `u16[source+0x9C]` (`frames` when set, `0` otherwise). One
  [`fn_802a35c0`] record is built, its `+0x24` phase is advanced once, it is
  copied, and the copy is translated by
  `(ctx+0xA4, ctx+0xA8, ctx+0xAC) - (tail+0xAC, tail+0xB0, tail+0xB4)`, where
  `tail` is the last child whose `+0x0C` type word is not `1`. Every node then
  builds one more record and the pair is handed to `0x802A3A20`, which emits
  `GXBegin(0x98)` plus the vertices. The vtable argument shape follows the
  already-translated sibling [`fn_802a3e00`], which uses the same three slots.
- **The billboard record builder `0x802A64B0`** (928 bytes, `medium`). The
  callback at `r8` produces a three-float reference vector for `(ctx, node)`.
  `0x801C1530` multiplies it by the node's `+0xCC` basis (or `ctx+0xC8` when
  bit 3 of `mode` is set), `0x80237BA0` normalises it (fallback
  `ctx+0xBC..+0xC4`), and a second product builds the third axis, which is
  normalised and written back to `node+0xCC..+0xD4`. The record matrix is
  `[axis0, reference, axis2, vector]` in three rows. The per-axis phases at
  `node+0x40..+0x48` are advanced by the `+0x9A..+0x9C` counters scaled by
  `0.024543693` (`0x8042F024`) and by the transform at `node+0xC8`; the scale is
  `node+0x30 * node+0x38 * transform+0x58`. A `0x802961E0` sample of
  `centre[1]` (raw-verified at `0x802A6740`) builds the side matrix, which
  `0x801C0540` concatenates with the record matrix. Two `0x801C1160` calls
  transform the caller's two points into `out` and `out+0x0C`, and the second
  float argument is stored at `out+0x18` (28 bytes total).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802A1090` | `fn_802a1090` | 1588 | medium |
| `0x802A4400` | `fn_802a4400` | 952 | medium |
| `0x802A64B0` | `fn_802a64b0` | 928 | medium |

## Deliberate skips

Four entries of the bucket stay untranslated. Each was re-read from
`functions.tsv` and from `work/dol/main.elf` before being left open. The
blocker is no longer the GQR type; it is the paired-single *lane* semantics
that the Ghidra C flattens.

### Live vector-lane emitters

| address | bytes | why |
|---|---:|---|
| `0x802A3160` | 1,092 | Standalone quad emitter. The export prints one `_DAT_cc008000` write at the tail, but the raw stream at `0x802A352C..0x802A3574` writes three floats for each of two transformed points and two more gated by bit 0 of `param_5`. Reproducing only the printed write would be wrong; the lane split of the two `0x801C1160` outputs is not proved. |
| `0x802A3A20` | 988 | Per-pair vertex emitter (`GXBegin(0x98)` plus `2*count+2` vertices). 34 psq pairs feed a software-pipelined loop: `psq_l ...,1,0` leaves the other lane live from the previous iteration, and the export keeps only one of the three FIFO component writes (`0x802A3B48..0x802A3B64`). The raw pass needed to prove the pipeline was outside this pass's budget. |
| `0x802A71D0` | 2,748 | Three-record bilinear blend called by the already-translated ring drawers. 197 psq sites, 181 of them against record pointers; the export collapses the lane-wise weights into scalar `double` expressions. A faithful body needs a full lane-by-lane transcription. |
| `0x802A90C0` | 1,396 | Three-record ring driver. The record array and the `+0x800` reversed path are visible, but the export leaves the ring index `iVar29` uninitialised when the pre-block is skipped, and the second edge block copies a ten-float record through two overlapping stack blocks. Reproducing it would require guessing the initial index. |

## Verification

- Every address and byte size matches `functions.tsv` (3 rows, 3,468 bytes).
- Every evidence path exists under `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 330 translated entries).
- None of the three addresses was in `ledger/functions.csv` before the pass;
  all were confirmed absent.
- `node tools/check-lane-csv.mjs wave114` reports `OK` for
  `wave114-gx-band.csv` (3 rows).
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings at lines
  23260/23307/23405 belong to the wave-107 body `fn_8028cee0` and predate this
  pass.
- The `psq` inventory was produced by decoding the `W`/`I` fields of every
  `psq_l`/`psq_st` in `0x802A1090..0x802A16CC`, `0x802A4400..0x802A47B8` and
  `0x802A64B0..0x802A6850`; all 56 operands carry `I = 0`, and the 20 in
  `0x802A1090` are stack spills.
- Constants were read from `work/dol/main.elf` with `r2 = 0x80433080`:
  `0x8042EF7C` = `1.1920929e-7`, `0x8042EFE8` = `0.01`, `0x8042EFEC` = `1.0`,
  `0x8042F020` = `1.1920929e-7`, `0x8042F024` = `0.024543693`,
  `0x8042F028` = `0.0`, `0x8042F02C` = `1.0`.
