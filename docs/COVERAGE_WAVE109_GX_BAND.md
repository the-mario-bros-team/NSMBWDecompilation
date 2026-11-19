# Coverage - Wave 109 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 109 pass over the last open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane (`src/render/gx_band.rs`).
Addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`); byte counts come from `work/ghidra/main-gb/export/functions.tsv`,
never from prose. The machine-readable mirror is
`ledger/agents/wave109-gx-band.csv` (two rows).

`status` is `translated` for both rows. `confidence` is `medium`: every guard,
offset, store and call is raw-verified against `work/dol/main.elf`, but field
roles are inferred from call shape, and the samplers both entries call are
`GQR3`-dependent and stay opaque.

## The finding that unblocked this bucket

Wave 108 closed this bucket with one body and listed the other twelve entries as
"live GQR type-switch samplers". That description is too wide. The PSQ
quantisation registers are written in only a handful of places in the image:

| site | writes | value |
|---|---|---|
| `0x801AB150` | `GQR0..GQR7` | all `0` (type 0 = float, scale 0) |
| `0x801ACE88` | `GQR1..GQR7` | restored from `+0x1A8..+0x1C0` of a GX save block |
| `0x8025EEC0` | `GQR2..GQR7` | fixed-point types (`0x00050005`, `0x05070507`, ...) |
| `0x802AB50C`, `0x802C2770` | `GQR2..GQR5` | loaded from a format record |
| `0x802EA6FC` | `GQR0..GQR7` | save-area restore inside the interrupt path |

`GQR0` is therefore `0` — i.e. **plain float** — at every point in this band: the
init path at `0x801AB150` zeroes it and its only other writer,
`0x802EA6FC`, is the matching restore. The instruction encoding tells which GQR
each site uses: `psq_l`/`psq_st` take a `W` bit (single-element access) and an
`I` field (the GQR index). Counting the `I` values per entry gives a hard split:

| entry | `psq` sites | `I` values | verdict |
|---|---:|---|---|
| `0x80290690` | 50 | 0 | GQR0 only → float |
| `0x80290CF0` | 18 | 0 | GQR0 only → **translated here** |
| `0x80291100` | 21 | 0 | GQR0 only → float |
| `0x80292C90` | 142 | 0 | GQR0 only → float |
| `0x80295C20` | 80 | 0 | GQR0 only → float |
| `0x80296160` | 6 | 0, 3 | reads **GQR3** → deferred |
| `0x802961E0` | 7 | 0, 3 | reads **GQR3** → deferred |
| `0x80296260` | 12 | 0, 3 | reads **GQR3** → deferred |
| `0x802963B0` | 17 | 0, 3 | reads **GQR3** → deferred |
| `0x80296820` | 17 | 0 | GQR0 only → **translated here** |
| `0x80299840` | 36 | 0 | GQR0 only → float |
| `0x8029F900` | 14 | 0 | GQR0 only → float |

For a `GQR0`-only entry every `if (type == 4) ... else if (type == 5) ... else
(float)` ladder in the Ghidra C collapses to its final `float` arm, which is why
these bodies can be written exactly. For the four `I = 3` entries the byte width
and scale come from the vertex-format record that `GXSetVtxAttrFmt`-style code
has installed in `GQR3` at the time of the call, so any transcription would have
to invent both. They stay open.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 2,248 | 0 | 2 |

## What this pass covers

- **Hit/threshold shape dispatcher** (`0x80290CF0`, 1,032 bytes). A dispatch byte
  at `state+0x29` selects one of six unit-box classifiers (`0x8028E7E0`,
  `0x8028E6F0`, `0x8028E5E0`, `0x8028E430`, `0x8028E310`, `0x8028E510`) and then
  one of six samplers (`0x8028E890`, `0x8028ED30`, `0x8028F2D0`, `0x80290690`,
  `0x8028F860`, `0x8028FEA0`). The samplers get
  `(matrix_world, state, point_to, point_from, point_to-point_from, direction,
  channel, source)` and return a fraction in `f1`. The body continues only when
  that fraction is `<= -1.0` (`fcmpu` plus a branch on the GT bit, so an
  unordered result also continues) and returns `1`; otherwise it returns `0`
  without writing anything. On the hit path it always runs
  `matrix_local * direction`, and either `matrix_world * point_to` (flag bits 2
  and 8 both clear) or the scaled variant: `transformed` is multiplied by
  `state+0x30..+0x38`, `(0 - fraction) * delta` is transformed by
  `matrix_local` and scaled the same way, and `out_vector = source + that`.
  Flag `0x40` then overwrites `out_vector` with `source` and sets the byte at
  `flag_out`; flags `0x10`/`0x20` dispatch to `0x80296DD0`/`0x80296F50` through
  the `owner+0xC8 -> +0x20 -> +0xBC -> +0x20 -> +0x10` chain.
- **Six-term particle transform** (`0x80296820`, 1,216 bytes). Each term is
  gated by its own scalar in the emitter record and evaluated in the original
  order: `+0x68` replaces the vector with `position*scale` (or zeroes it),
  `+0x6C` adds `axis*scale`, `+0x70` adds the three-angle random spin built from
  three LCG draws and three `0x802961E0` samples, `+0x74` adds `ring*scale`,
  `+0x7C` adds the matrix-rotated unit Y axis (fixed angles `+0x84/+0x88/+0x8C`,
  or a random spin composed with them when `+0x80` is set), and a non-zero
  signed byte at `+0x66` scales the result by
  `1 - 0.01*byte*(2r - 1)`. The three LCG advances of the spin term store only
  the last seed, exactly like the original's single `stw`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80290CF0` | `fn_80290cf0` | 1032 | medium |
| `0x80296820` | `fn_80296820` | 1216 | medium |

## Deliberate skips

Ten entries stay untranslated. Two different reasons, kept separate so a later
pass knows which ones are cheap.

### Deferred: `GQR3`-dependent samplers (not reconstructible as shipped)

| address | bytes | why |
|---|---:|---|
| `0x80296160` | 116 | writes the wrapped angle through `psq_st ...,1,3`, reads it back through `psq_l ...,1,3`, then indexes the 16-entry table `0x802F74A0` by the low byte; width and scale both come from live `GQR3` |
| `0x802961E0` | 120 | same sampler with the pair split across the two output pointers |
| `0x80296260` | 332 | three-angle matrix builder; quantises its angles and table rows through `GQR3` before the lerp |
| `0x802963B0` | 376 | three-angle rotation-matrix writer; same `GQR3` quantisation of the input angles |

`0x802961E0` and `0x802963B0` remain callable by address (both are used in the
`0x80296820` body, and `0x802963B0` already had a constant). Their own bodies
cannot be written without the runtime `GQR3` value: Ghidra proves this by
choosing between `char`, `short` and `float` stores from the same word.

### Deferred: reconstructible but outside this pass's budget

These six use only `GQR0`, so the float arms of their ladders are the whole
body. They are large straight-line emitters and none of them needs a new
insight, only the translation work:

| address | bytes | what it does |
|---|---:|---|
| `0x80290690` | 1,628 | sampler 3 of the dispatcher: clamp `(c - *param_4) / *param_5` and emit per-channel attributes |
| `0x80291100` | 2,208 | 10-argument, 0x1B0-byte-frame emitter |
| `0x80292C90` | 5,464 | largest sampler in the bucket, 142 `psq` sites |
| `0x80295C20` | 1,232 | sampler with LCG dither |
| `0x80299840` | 6,316 | per-vertex attribute emitter |
| `0x8029F900` | 3,392 | per-vertex attribute emitter |

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 2,248 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 318 translated entries).
- Neither address was in `ledger/functions.csv` before the pass.
- `node tools/check-lane-csv.mjs wave109` reports `OK` with
  `wave109-gx-band.csv: 2 rows`.
- `cargo check --message-format=short` reports no error and no warning from this
  pass. The three `unused_assignments` warnings at `src/render/gx_band.rs`
  lines 23202, 23249 and 23347 belong to the wave-107 body `fn_8028cee0` and
  predate it.
- Constants were read from `work/dol/main.elf` with `r2 = 0x80433080`:
  `0x8042EBBC` = `0.0`, `0x8042EBA4` = `0.0`, `0x8042EBD0` = `-1.0`,
  `0x8042ED30` = `0.0`, `0x8042ED34` = `65536.0`, `0x8042ED38` = `2.0`,
  `0x8042ED3C` = `pi`, `0x8042ED40` = `1.0`, `0x8042ED44` = `0.01`.
- The GQR inventory above was produced by scanning every `mtspr` to SPR
  `912..919` in `main.dol` and decoding each `psq_l`/`psq_st` operand of the
  twelve entries.
- `0x80290CF0`'s prologue was decoded to recover the real signature: five
  argument words are read from the caller's frame at `+8..+24`, which is why
  Ghidra shows thirteen parameters; the two `0x802DCD54`/`0x802DCDA0` helpers it
  prints as a value-returning call are `__savegprlr_19`/`__restgprlr_19`.
