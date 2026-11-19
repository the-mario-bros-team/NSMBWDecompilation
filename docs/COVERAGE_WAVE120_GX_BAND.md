# Coverage - Wave 120 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 120 pass over the last open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave120-gx-band.csv` (one row,
`partial`).

The bucket held three untranslated entries when this pass started:

| address | bytes | state after this pass |
|---|---:|---|
| `0x80292C90` | 5,464 | open - text/`pcVar` dispatch body, see "Deliberate skips" |
| `0x80299840` | 6,316 | **cloud half landed here**, grid half open |
| `0x8029F900` | 3,392 | open - two-branch basis builder, see "Deliberate skips" |

## The finding that re-opened `0x80299840`

Two defects in the Ghidra C were corrected from `work/dol/main.elf` before the
body was written:

- **The cloud/grid selector is confirmed.** `0x802999B8` is
  `rlwinm r0,r7,0,14,14` followed by `bc 12,2,0x8029A768`, so a clear
  `flags & 0x00020000` jumps forward to the cloud loop at `0x8029A768`. The
  cloud loop counts against `r29`, which `0x80299904` sets to `r6` (`count`);
  the grid loop counts against `r23` ( `mullw r23,r6,r6` at `0x80299A20`), i.e.
  `count * count`. The export's `for (iVar10 = 0; iVar10 < param_4; ...)` in
  the cloud half is therefore correct, and the grid half's loop bound is the
  squared one.
- **The export prints the `0x8029A824` arms in the wrong order.** The raw
  sequence is `fcmpu 0,f18,f29` with `f18 = 1.0` and `f29 = 0.01*params[3]`,
  then `bc 4,2,0x8029AAB4`. `BO = 4`, `BI = 2` branches on `CR0[EQ]`, so
  `0.01*params[3] == 1.0` takes the **ellipsoid** block at `0x8029AAB4`; the
  fall-through at `0x8029A828` is the six-face block. Ghidra's C puts the
  six-face block on the equality arm. The body here follows the raw stream and
  leaves a comment at the site.
- The first arm (`previous == k`, the three independent draws) is reachable
  only when `0.01*params[3]` underflows to zero. It is kept because the raw
  compare at `0x8029A790` is `0.0 == f29` and the block at `0x8029A798` is
  live for that input.
- The `f1` argument of every `0x80299510` call is the incoming `f1`: the
  entry sequence stores it with `fmr f23,f1` at `0x802998E4` and every call
  site restores it with `fmr f1,f23` (for example `0x80299C9C` and
  `0x8029AFB0`). `0x80299510` is already translated in this file, which pins
  the argument order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 6,316 | 0 | 1 |

## What this pass covers

- **The cloud mode of the sphere/density emitter `0x80299840`**. The body
  reconstructs the whole non-grid branch:
  - three radius scales from `params[0..3]`, each clamped to `1.0e-5`
    (`0x8042EF08`), and the `flags & 0x02000000` collapse to `params[0]`;
  - thickness `k = 0.01*params[3]` (or `1.0e-5` when `params[3] == 0`) and
    the radius `node[0x78]`, nudged off `pi` by `3.7450704e-7`
    (`0x8042EF10`);
  - three point generators: three independent `[-1,1]` draws (the
    underflow arm), the ellipsoid projection (four LCG words, with the
    `k`/`1-k` scaling chain of `0x8029AAB4..0x8029AB00`), and the six box
    faces selected by `(seed >> 16) % 6`;
  - the pole selector (`0x8029AD18` onward), the radius scaling, and the
    `length / sin(max*node[0x78])` normaliser with the single-precision
    reciprocal plus one Newton step (`0x8029AF10..0x8029B034`);
  - the `0x80299510(scale, ctx, vec, dir, node, emitter, p9, p10)` call.
  The LCG word is `node+0xEC`, the draw id and the transform are left to
  `0x80299510`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80299840` | `fn_80299840` | 6316 | medium (partial) |

## Deliberate skips

### `0x80299840` grid mode (`flags & 0x00020000`)

The grid half (`0x802999C0..0x8029A760`, about 870 instructions) is not in the
body. The export interleaves the spiral state machine (`r19` phase, `r20`/`r22`
coordinates, `r17` stride, `r23 = count*count`) with four inner loops that the
decompiler prints under the same `uVar12`/`uVar13` names. The raw stream shows
the state machine running at `0x80299A5C..0x80299AB8` and the first inner loop
starting at `0x80299D3C` with its own counter, but reconstructing the exact
nesting and the four mirrored face formulas needs a control-flow rebuild rather
than a line-by-line reading of the export. Writing the four loops from the
printed C would silently drop the state update, so the body returns before the
grid path and the ledger row is `partial`.

### `0x80292C90` (5,464 bytes)

The 3,425-line export keeps 106 removed blocks spread across the live body,
not just the prologue, and 38 `pcVar` temporaries. The virtual slots are read
through offsets Ghidra never resolves, so the dispatch chain cannot be
attributed without guessing slot numbers. It needs a raw-stream transcription
of the whole dispatch, not a late-pass add-on.

### `0x8029F900` (3,392 bytes)

This pass corrects the wave-118 reason. The six indirect calls are **not**
unresolved: the export prints the slots directly as `*(*piVar17 + 0x18)`
(`0x8029FBD8`), `+ 0x10` (`0x8029FE10`) and `+ 0x14` (`0x8029FE30`/`0x8029FE48`),
and the returned pointers are called as `(*pcVar20)(param_3)` and
`(*pcVar21)(param_3, node)`. What blocks the body is the two-branch basis
builder at `0x8029FC10..0x8029FD58`: both arms concatenate a matrix, clear one
row with `0x801C9940`, then normalise a three-float record whose stack slots
(`local_3B0`/`local_3AC`/`local_3A8`) are re-used by the GQR ladders for two
different source triples. The loop half is a straight particle draw
(`0x8029BAB0`, the vtable callback, two `0x801C1530` transforms and one or two
`0x8029F4A0` quads), but the axis handed to the callback is produced by that
ambiguous builder, so a body written from the export alone would invent the
basis. It remains open.

## Verification

- Every address and byte size matches `functions.tsv` (one row, 6,316 bytes).
- Every evidence file exists at the cited path:
  `work/ghidra/main-gb/export/80299840_FUN_80299840.c`.
- Every row has exactly one `pub unsafe fn fn_80299840` body in
  `src/render/gx_band.rs`; the module header now carries item 31 and the
  running total is 339 reconstructed entries (one `partial`).
- `0x80299840` was not in `ledger/functions.csv` before the pass; the whole
  `0x80290000` bucket was re-read (85 entries, three open).
- `node tools/check-lane-csv.mjs wave120` reports `OK` for this lane's CSV.
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`.
- The cloud branch was checked against `work/dol/main.elf` at the addresses
  cited above; the grid branch and the two other functions are documented as
  skips rather than guessed.
