# Coverage - Wave 123 (GX / render band, `0x802A0000` bucket)

## How to read this file

This page covers the Wave 123 pass over the `0x802A0000..0x802AFFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unloaded `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave123-gx-band.csv` (one row,
`translated`, `medium`).

The bucket held two untranslated entries when this pass started:

| address | bytes | state after this pass |
|---|---:|---|
| `0x802A71D0` | 2,748 | open - see "Deliberate skips" |
| `0x802A90C0` | 1,396 | **landed** - forward/reverse three-record ring driver |

The `0x802A0000` bucket is now 101 of 102 translated (the remaining entry is
`0x802A71D0`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 1,396 | 0 | 1 |

## What this pass covers

- **The ring driver `0x802A90C0`** (1,396 bytes, `medium`). It is the sibling of
  the already-landed [`fn_802a8a10`] and [`fn_802a8d10`] in this module: the
  scene vtable yields the draw callback at `+0x18` and the two iterator
  getters at `+0x10`/`+0x14`, and each of the three 0x28-byte records is built
  by [`fn_802a6850`] (the `+0x18` callback result is its last argument).
- **The direction switch.** `source[0x9C] & 0x800` is passed to both iterator
  getters. When set, the ring starts at `index = u16[descriptor+0x50]` with
  `direction = -1`; when clear it starts at `0` with `direction = +1`.
- **The step.** `source[0x14E] & 0xC0 == 0x40` selects `1.0f`
  (`0x8042F02C`); every other value uses `1/count`, computed with `fres` plus
  one Newton step. The unrefined `r*r` and `2*r` values are the two floats the
  first iterator is called with, exactly as in [`fn_802a8a10`].
- **The offset vector.** The leading/trailing seam records are shifted by
  `ctx[0xA4..0xAC] - node[0xAC..0xB4]`, where `node` is the first entry in the
  `descriptor+0x3C` list whose `+0x0C` word is not `1`. The list stride is the
  `u16` at `descriptor+0x42`.
- **The forward seam.** On the clear arm the first record is shifted by the
  offset vector and its unshifted copy (with the next step) is emitted as the
  leading `(first, first, edge)` triple.
- **The reverse seam.** On the set arm the record one slot ahead is rebuilt
  from the current record plus the offset vector, the current record's
  `+0x0C..+0x14` is written from its `+0x18..+0x20`, and the trailing
  `(previous, current, next)` triple is emitted.
- **The slot math.** Inside the loop the slot is `(i + 1) % 3` (the
  `0x55555556` `mulhw` sequence at `0x802A93E0..0x802A943C`), the new record
  is written to that slot, and the triple is
  `(previous_previous, previous, current)`. The closing call repeats the last
  record twice.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802A90C0` | `fn_802a90c0` | 1396 | medium |

## Export defects this pass resolves

- **The `0x802DCD40` pseudo-return.** The export turns the register-save helper
  into a 64-bit return and splits `r3`/`r4` out of it. `0x802DCD40` only stores
  the non-volatile GPRs, so the incoming `r3` (scene) and `r4` (context)
  survive untouched; the body uses them directly.
- **Every paired-single operand is GQR0.** All 26 `psq_l`/`psq_st` sites in
  `0x802A90C0..0x802A9634` carry `I = 0`; `0x801AB150` leaves GQR0 at float, so
  the export's `ldexpf`/`CONCAT` ladders are dead and each pair is two `f32`
  lanes. The `fsubs`/`fmuls` operand fields were read from the raw stream (the
  float-multiply form uses the `frC` slot, which the export flattens).
- **Two write-only stack records.** The export keeps a 0x28-byte record at
  `r1+0x48` (forward arm) and one at `r1+0x20` (reverse arm); every store to
  them is verified against the raw stream and no later instruction reads either
  block, so they are compiler spills and are not reproduced.
- **Constant block.** With `r2 = 0x80433080`: `0x8042F028 = 0.0f`,
  `0x8042F02C = 1.0f`, `0x8042F038 = 0.5f`, `0x8042F03C = -1.0f`,
  `0x8042F048 = 0.01f`, and `0x8042F040 = 2^52 + 2^31` is the signed
  integer-to-double bias.

## Deliberate skips

### `0x802A71D0` (2,748 bytes)

Still open. This is a `(r4 + 1) x (r5 + 1)` patch tessellator: a 976-byte frame
holding a matrix of ten-float records, an outer loop over the `r4` counter
(`0x802A7C18..0x802A7C34`), an inner loop over the `r5` counter
(`0x802A7928..0x802A7A88` and its mirrored twin `0x802A7AB4..0x802A7C14`), and
three-float `stfs` writes to `0xCC008000` with an optional two-float UV pair.
The body is reconstructible only after modelling two things the export does not
pin:

1. **The blend parameter.** The entry block multiplies by the SDA constant at
   `0x8042F028` (`0.0f`), and the outer loop then builds cubic weights from the
   raw row counter (`0x802A754C..0x802A758C`) without ever applying the
   `1/r4` reciprocal it computes at `0x802A729C..0x802A731C` (that `f28` value
   is never read again). Whether the counter is a pre-scaled float, a step
   count, or a raw index is not settled by the in-lane call sites: the driver
   passes `r4 = param` (a forwarded draw word) and `r5 = source[0x14C]` (a
   byte), and no caller supplies the float register the export names `f1`.
2. **The lane split of the two mirrored loops.** Both loops write only three
   FIFO words plus an optional `(u, v)` pair per vertex, while the export keeps
   a single `local_3a8`/`local_36c`/`local_330` triple and 197 `ldexpf` calls.
   Every `psq` operand is GQR0, but the `ps_muls0` lane selection and the
   overlapping 0x28-byte stack records still have to be transcribed lane by
   lane; reproducing the export's scalar arms would be wrong.

Landing it needs a full raw control-flow rebuild (687 instructions) plus an
argument model for `r4`/`r5`; that is outside this pass's budget, so it is
documented here rather than guessed. The lane's open remainder is this entry
plus `0x80292C90` (outside this bucket).

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 1,396 bytes).
- The evidence file exists at the cited path:
  `work/ghidra/main-gb/export/802a90c0_FUN_802a90c0.c`.
- Every row has exactly one `pub unsafe fn fn_802a90c0` body in
  `src/render/gx_band.rs`; the module now holds 343 reconstructed entries
  (342 full, one partial).
- `0x802A90C0` was not in `ledger/functions.csv` before the pass; the whole
  `0x802A0000` bucket was re-read (102 entries, 100 already translated, these
  two open).
- `node tools/check-lane-csv.mjs wave123` reports no problem naming this CSV.
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings in
  `fn_8028cee0` predate this pass and no new warning comes from
  `fn_802a90c0`.
- The body was transcribed from `work/dol/main.elf`; every cited branch,
  constant, store and call argument was checked against the raw stream, and the
  register map (`r19` scene, `r20` context, `r21` param, `r22` mode, `r24`/`r23`
  ring slots, `r26` index, `r25` direction, `r27` next iterator, `r28` callback)
  was read from the prologue and loop.
