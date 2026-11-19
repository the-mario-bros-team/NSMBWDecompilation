# Coverage - Wave 95 (runtime::dvd, the `0x801C0000` page: matrix, texture-object and raster state)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801C0000` 64 KiB page of `main.dol`,
which is much wider than the disc band (`0x801CA640`-`0x801D6780`). Everything
below is SDK code that happens to share the page, and `src/runtime/dvd.rs` is
the only writable file assigned to the bucket, so it is reconstructed there.
Each function doc in that file states plainly which routine it is, which state
fields it touches and how confident the reconstruction is; this document only
summarises the pass.

The `Functions` table lists the address, the Rust symbol, the byte size from
`work/ghidra/main-gb/export/functions.tsv` and the confidence recorded in the
body's `# Confidence` block. `high` means every store, mask and call in the
body is literal in the export; `medium` means the instruction sequence is
literal but at least one field role, callee identity or paired-single arm is
inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 29 | 5992 | 7 | 22 |

## What this pass covers

The bucket still held 66 untranslated addresses. This pass takes 29 of them in
four groups spread over `0x801C0BD0`-`0x801C9530`; none is disc code.

- **Matrix/projection tail, `0x801C0BD0`-`0x801C1260` (8 bodies, 988 bytes).**
  The constructors Wave 90's matrix pass stopped short of: the translation
  builder [`fn_801c0bd0`], the translated copy [`fn_801c0c10`], the two scale
  builders [`fn_801c0c60`]/[`fn_801c0c90`], the quaternion-to-rotation
  [`fn_801c0cf0`] and the matrix/vector multiply [`fn_801c1160`]. The two
  projection builders [`fn_801c0fd0`]/[`fn_801c1260`] take `1/tan` on both
  diagonals through `0x802E8590`. The quaternion role is proven algebraically:
  with `t = 2/(x^2+y^2+z^2+w^2)` the twelve entries reduce exactly to the
  standard `1-t(y^2+z^2)`, `t(xy-zw)`, `t(xz+yw)`, … form with a zero last
  column.
- **Texture-object layer, `0x801C5B30`-`0x801C6FF0` (9 bodies, 2412 bytes).**
  The two texel/mip estimators [`fn_801c5b30`]/[`fn_801c5df0`] share the
  `r2-0x4ED0` ratio and the `0x400` clamp; [`fn_801c5df0`] also emits
  `0x4E000000|texels` and clears bit 10 of `+0x23C` unless the count is exactly
  `0x100`. The compare-mode emitters [`fn_801c6150`]/[`fn_801c6290`] write the
  `0x52004000`/`0x52000000` words at `+0x23C`/`+0x24C` and flush `+0x220` and
  `+0x22C` on the exported predicates. [`fn_801c6880`] streams the twelve
  floats at `+0x10` and is the one emitter that sets the `+2` marker to one.
  [`fn_801c69e0`] patches one byte of the two TMEM colour words,
  [`fn_801c6ba0`] is the format-switched texel counter, [`fn_801c6d90`] is the
  32-byte object initialiser and [`fn_801c6ff0`] its filter/bias setter.
- **Raster/channel state, `0x801C7260`-`0x801C8790` (10 bodies, 2236 bytes).**
  The packed-descriptor decoder [`fn_801c7260`], the channel colour writer
  [`fn_801c7350`] (seven `r13` byte tables, the `+0x51C` object patch, the
  `+0x564`/`+0x584` state arrays) with its two exchange entry points
  [`fn_801c74c0`]/[`fn_801c7550`], the texcoord packer [`fn_801c75f0`], the
  three-group command emitter [`fn_801c7e00`], the stage and two-bit setters
  [`fn_801c7f40`]/[`fn_801c8040`], the texcoord source copier [`fn_801c8250`]
  and the texcoord-generator writer [`fn_801c8790`].
- **Draw-frame pair, `0x801C9470`/`0x801C9530` (2 bodies, 356 bytes).** The
  begin and end of the token buffer. The only caller, `0x802C2700`, passes
  `(stack buffer, 0x4000)`, which is what fixes the two arguments of
  [`fn_801c9470`]; the pair fills the nine-word block at `0x803904D0`, shadows
  0x600 bytes of GX state at `0x80390550`, and restores it under the interrupt
  lock on the way out.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C0BD0` | `fn_801c0bd0` | 52 | medium |
| `0x801C0C10` | `fn_801c0c10` | 76 | medium |
| `0x801C0C60` | `fn_801c0c60` | 40 | medium |
| `0x801C0C90` | `fn_801c0c90` | 88 | medium |
| `0x801C0CF0` | `fn_801c0cf0` | 164 | medium |
| `0x801C0FD0` | `fn_801c0fd0` | 248 | medium |
| `0x801C1160` | `fn_801c1160` | 84 | medium |
| `0x801C1260` | `fn_801c1260` | 236 | medium |
| `0x801C5B30` | `fn_801c5b30` | 136 | medium |
| `0x801C5DF0` | `fn_801c5df0` | 200 | medium |
| `0x801C6150` | `fn_801c6150` | 316 | medium |
| `0x801C6290` | `fn_801c6290` | 344 | medium |
| `0x801C6880` | `fn_801c6880` | 128 | medium |
| `0x801C69E0` | `fn_801c69e0` | 212 | high |
| `0x801C6BA0` | `fn_801c6ba0` | 292 | high |
| `0x801C6D90` | `fn_801c6d90` | 524 | medium |
| `0x801C6FF0` | `fn_801c6ff0` | 260 | medium |
| `0x801C7260` | `fn_801c7260` | 220 | medium |
| `0x801C7350` | `fn_801c7350` | 356 | medium |
| `0x801C74C0` | `fn_801c74c0` | 84 | medium |
| `0x801C7550` | `fn_801c7550` | 148 | medium |
| `0x801C75F0` | `fn_801c75f0` | 168 | high |
| `0x801C7E00` | `fn_801c7e00` | 308 | high |
| `0x801C7F40` | `fn_801c7f40` | 256 | high |
| `0x801C8040` | `fn_801c8040` | 200 | high |
| `0x801C8250` | `fn_801c8250` | 148 | medium |
| `0x801C8790` | `fn_801c8790` | 348 | medium |
| `0x801C9470` | `fn_801c9470` | 180 | high |
| `0x801C9530` | `fn_801c9530` | 176 | medium |

## Paired-single treatment

The matrix and texcoord routines were compiled with paired singles. Ghidra
renders each `psq_l`/`psq_st` as a switch over GQR0 whose arms are the 8-bit,
16-bit and `f32` forms. GQR0 cannot be read from Rust and this title's callers
hand these routines plain `f32` arrays, so the bodies reproduce the `f32` arm
and say so in `# Confidence`, exactly as Wave 90 did for the adjacent slice.

## Deliberate skips

- `0x801C0AA0` (176 bytes), `0x801C0B50` (124 bytes): the matrix-product core
  and its thin wrapper. `0x801C0AA0` accumulates twelve outputs through f64
  temporaries plus the `SQRT` helper `0x802E8620`; `0x801C0B50` calls
  `0x802E84C4` with no visible argument, so its callee role is unresolved and
  no body is claimed for either.
- `0x801C0020` (1196 bytes), `0x801C1DB0` (1284), `0x801C22C0` (2240),
  `0x801C3050` (656), `0x801C7A40` (848), `0x801C8920` (556), `0x801C90A0`
  (976), `0x801C9E20` (2076): the large remaining routines of the page. None
  was attempted.
- `0x801C0540`-`0x801C09F0` (6 bodies, 124-392 bytes) and
  `0x801C1680`-`0x801C2B80` (6 bodies, 228-460 bytes): the two matrix/vector
  slices on either side of this pass. Not read line by line; left for a later
  pass rather than guessed at. `0x801C2E70` (480 bytes), the token-block
  submission that [`fn_801c9470`]/[`fn_801c9530`] call, stays untranslated and
  is called by absolute address.
- `0x801C5570`/`0x801C56C0` (332/216 bytes): the two FIFO padding writers that
  Wave 80 already deferred. Ghidra merges their final store with the FIFO's
  previous 16-bit content, so the low half of the emitted word is not
  recoverable from the export.
- `0x801C5BC0` (548), `0x801C5F40` (484): the remaining texture-stage arms; the
  state fields they write overlap the ones taken here but the bodies are long
  and were not attempted.
- `0x801C6470` (412) and `0x801C6760` (268): the fog/parameter table and the
  sphere/light vector setup. Both read a dozen distinct unnamed SDA2 floats
  (`r2-0x4E00`-`r2-0x4EC0`); only the arithmetic is recoverable, so no body.
- `0x801C6FA0` (72 bytes): calls `0x801C6D90` with the argument list Ghidra
  dropped entirely, and takes its own ninth argument from the caller's stack
  slot. The callee arguments cannot be reconstructed honestly.
- `0x801C78D0` (356 bytes): entered through the `0x802DCD64` register-save
  helper, whose saved registers are not surfaced as parameters, so the body
  would silently drop inputs.
- `0x801C8B50` (352 bytes): the ten-entry LOD table. The loop shape is clear,
  but its scale constants (`r2-0x4E00`, `-0x4E04`, `-0x4E08`, `-0x4E10`,
  `-0x4E30`, `-0x4E44`) are unnamed and two of them are `f64` sentinels.
- `0x801C8CB0` (272 bytes according to `functions.tsv`): the export is a single
  two-store emitter (one FIFO word plus the `+2` marker). The cited size and the
  body disagree, so the byte count cannot be quoted honestly and no row is
  claimed.
- `0x801C97F0`/`0x801C9940`/`0x801C99C0`/`0x801C9A80` (68-180 bytes): the
  immediate-mode position emitters. Ghidra renders their `psq_st` pairs as
  stores to `0xCC008002`/`0xCC008004`, so the number of words emitted per
  vertex is not recoverable from the export; left out rather than guessed.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (29 rows, 5992 bytes, 7 high and
  22 medium).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave95` prints `OK` and lists
  `wave95-dvd.csv: 29 rows`.
- `cargo check --message-format=short` is free of errors naming
  `src/runtime/dvd.rs`.
