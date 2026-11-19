# Coverage - Wave 124 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 124 pass over the `0x80290000..0x8029FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses
of the unloaded `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave124-gx-band.csv` (one row,
`partial`, `medium`).

The bucket held one untranslated entry when this pass started:

| address | bytes | state after this pass |
|---|---:|---|
| `0x80292C90` | 5,464 | **landed (partial)** - per-node keyframe/trace walk |

The `0x80290000` bucket is now 85 of 85 (83 full, two partial: `0x80299840`
and `0x80292C90`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 5,464 | 0 | 1 |

## What this pass covers

- **The scene/node driver.** `0x80228FF0(this+0x38, this[0x54])` yields the
  first node; the scene callback at `*(this[0x20]+0xBC)+0x48` is called with
  `(this, this+0x38, node)`; the optional `this[0x8A]` block rebuilds the local
  matrix through `0x80289070` + `0x80288E30` and clears the flag.
- **The matrix context.** `this+0x8C` (twelve words) is inverted with
  `0x801C07A0`, the ambient matrix comes from `0x80289070(this[0x20])`, and
  `0x801C0540` concatenates the inverse with it. The body keeps the inverse of
  that product plus translation-zeroed copies of both matrices; the `0x340`
  inverse is written and never read, exactly as in the raw stream.
- **The node gate and cache.** Per node the body checks `+0xC == 1` and clears
  `+0xD8`, optionally sets bit `0x10000` in `*(owner+0x50)`, computes
  `delta = old +0xAC..+0xB4 - old +0xB8..+0xC0`, then copies
  `+0xAC..+0xB4` into `+0xB8..+0xC0`.
- **The key list.** The list header is `this[0x24]`; the stride comes from the
  header and the start index from either zero or the `u16` at the block header
  `+6`. Each key is skipped when bit `0x8` of `key[4]` is set, and only tags
  `0xAA`/`0xAB` are accepted.
- **The sampler dispatch.** Byte `key[2]` selects:
  `0` -> `0x80289410`, `3` -> `0x8028A6F0`, `4` -> `0x8028C7A0`,
  `5` -> `0x8028CEE0`, `6` -> `0x8028B6C0`. The index/salt/frames triple is
  either `node[0xDC..0xDE..0xE0]` or the owner record at
  `*(node+0xC8)+0x20` (`+0xE4`, `+0xEA`, `+0x3C`/`-1`).
- **The kind-2 trace carrier.** Its `0x24`-byte record is copied out of
  `key + key[0x18] + key[0x14] + key[0x10] + key[0xC] + 0x20`; when
  `key[0x18] > 4` the gate is the word at
  `tail + 4*u16(record+0x46) + 4`. The presence of this key arms
  `0x80291100`, which receives `(node, record_copy, record, gate, ambient,
  inverse_context, node+0xAC, axis_b, node[0xA0..A8]+axis_c, &hit)`.
- **The kind-7 record and direction builders.** The `0x1C`-byte record at
  `0x14C` is copied, optionally sampled into its first float, and then
  `key[1]` selects builders `0`, `1`, `2`, `3`, `6`, `8` (table rotation,
  anchor scale, two delta/normalise arms, axis rotation, owner translation
  difference). Builders `4` and `7` are the partial arms below.
- **The shared tail.** The record's first byte selects
  `0x801C1160` against the inverse/concatenated matrices (`1` -> inverse
  concatenated, `0` -> inverse context, `3` -> scale by `1/3` first), and the
  record's second byte accumulates the tangent into `0x110` (byte `0`) or
  `0x104` (byte `1`).
- **The post-trace block.** A non-zero trace result adds the direction to
  `node[0xA0..0xA8]` and folds the two axis vectors into
  `node[0xAC..0xB4]`; when `record[0x48] & 1` is set the record's scale,
  three angles and translation rebuild a matrix, the node's local point is
  wrapped into `[-1,1]` with `fmod(..., 2.0)`, and the result is applied back.
  The no-trace arm (`0x9408C`) performs the equivalent update directly.
- **The epilogue.** `0x80229010(this+0x38, 0)` refreshes `this+0x54` and the
  scene's `+0x4C` callback receives the first node.

## The export defects this pass resolves

- **The pseudo-return is `__savegprlr`.** `bl 0x802DCD40` at `0x80292D00` only
  stores the non-volatile GPRs; it never writes `r3`. The export's
  `FUN_802dcd40()` result is an artefact and the first argument survives as
  `this`.
- **Every `psq` operand is GQR0.** All 142 `psq_l`/`psq_st` sites in
  `0x80292C90..0x802941E4` decode with `I = 0`, and `0x801AB150` leaves GQR0 at
  plain float, so the 147 `unaff_GQR` references and the 106 removed blocks are
  dead helper ladders. The bodies were transcribed from `work/dol/main.elf`.
- **The switch table is `0x8034F5F0`.** The export's `switch (uVar30)` has nine
  live arms (`0..8`); the raw table at `0x8034F5F0` lists
  `0x80293450`, `0x80293428`, `0x8029395C`, `0x802939B0`, `0x80293A40`,
  `0x80293BB0`, `0x802938F0`, `0x80293494`, `0x80293B40` for indices `0..8`.
- **Constant block.** With `r2 = 0x80433080`:
  `0x8042ECA8 = 1.0`, `0x8042ECAC = 0.0`, `0x8042ECB0 = 0.57735026`,
  `0x8042ECB4 = 32768.0`, `0x8042ECB8 = 65535.0`, `0x8042ECBC = 2.0`,
  `0x8042ECC0 = pi`, `0x8042ECC4 = 40.743664`, `0x8042ECC8/0x8042ECD8/0x8042ECE0`
  are the `2^52` double biases and `0x8042ECD0 = -1.0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80292C90` | `fn_80292c90` | 5464 | medium (partial) |

## Deliberate skips

### `0x80292C90` builder `4` (`0x93A40..0x93B3C`)

The arm builds a basis from record floats 3..5 and the first axis, then merges
three `ps_*` results into one tangent. The raw lane order of the six
`ps_sum0`/`ps_madd` sites is literal, but the export re-uses the same
`local_4d4`/`local_4cc` slots for two different source triples, so the
intermediate that feeds the final `0x802960F0` normalise cannot be attributed
without a lane-level rebuild. The arm therefore leaves the previous tangent in
place and the row is `partial`.

### `0x80292C90` builder `7` (`0x93550..0x938BC`)

The hash gate (`0x93494..0x93550`) **is** reproduced: the 32-bit mix of
`sample_index`, `salt` and `key[6]` uses the `0x343FD`/`0x269EC3` LCG, the
byte-fold, and the `sample_index % (u16(record+0xC)+1)` test. What is not
reproduced is the frame construction below it: two dither stages
(`0x93614`/`0x936E4`), the anchor/delta selection, `0x80295860`, the two
`0x80296160`/`0x802961E0` angle samples, and the component masks in
`0x938BC..0x938EC`. As with builder `4`, the previous tangent survives.

All other arms, the record/trace plumbing and both post-processing paths are
transcribed from `work/dol/main.elf`.

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 5,464 bytes).
- The evidence file exists at the cited path:
  `work/ghidra/main-gb/export/80292c90_FUN_80292c90.c`.
- Every row has exactly one `pub unsafe fn fn_80292c90` body in
  `src/render/gx_band.rs`; the module now holds 344 reconstructed entries
  (342 full, two partial).
- `0x80292C90` was not in `ledger/functions.csv` before the pass; the whole
  `0x80290000` bucket was re-read (85 entries, this one open).
- `node tools/check-lane-csv.mjs wave124` reports `OK` for this lane's CSV.
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings in
  `fn_8028cee0`/neighbours predate this pass and no new warning comes from
  `fn_80292c90`.
- The body was transcribed from `work/dol/main.elf`; every cited branch,
  constant, store and call argument was checked against the raw stream, and the
  register map (`r15` scene, `r21` node, `r20` key index, `r22` trace flag,
  `r23` key, `r31` builder, `r18`/`r16`/`r17` sampler triple, `r19` trace
  record) was read from the prologue and loops.
