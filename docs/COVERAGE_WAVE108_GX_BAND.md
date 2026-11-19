# Coverage - Wave 108 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 108 pass over the last open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane (`src/render/gx_band.rs`).
Addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`); byte counts come from `work/ghidra/main-gb/export/functions.tsv`,
never from prose. The machine-readable mirror is
`ledger/agents/wave108-gx-band.csv` (one row).

`status` is `translated` for the row; `confidence` follows the body's
`# Confidence` line (`medium`, because the per-attribute component counts are
inferred from GX's compCnt encoding while every guard, call and shipped store is
raw-verified).

Before this pass the bucket held 85 Ghidra entries, 72 of them already in
`ledger/functions.csv`. The thirteen that were left were re-read in full. Twelve
are live GQR type-switch samplers and are listed under "Deliberate skips"; the
remaining one, `0x8029BAB0`, contains no paired-single code at all and is
translated here: one body, 1,484 bytes.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 1,484 | 0 | 1 |

## What this pass covers

- **Material-state flush plus degenerate strip** (`0x8029BAB0`, 1,484 bytes).
  The entry compares the material id byte `material+0x97` with the render
  object's cached copy at `+0xB0`; on a miss (or when `force != 0`) it stores the
  id into `+0xB0`/`+0xB4` and emits one packed GX register word through
  `0x801C86C0` built from `mask+2`, the id, `mask+4`, `mask+3` and
  `material+0x98`. It then runs the TEV colour-channel sync `0x8029C080` and the
  per-channel texgen builder `0x8029C820` — both are `void` in this module, but
  the original tests `r3` afterwards, so the body calls them by address through
  `call4`/`call5` and keeps the register. When either reported work, the texgen
  builder reported `0`, bit 3 of `*mask` is clear and the flag four pointer hops
  from `material+0xC8` (`+0x20`, `+0xBC`, `+0x20`, `+0xC064`) is set, the
  function reads the five VAT descriptors (9 = POS, 10 = NRM, 11 = CLR0,
  12 = CLR1, 13 = TEX0) from the GX state cache, issues
  `GXBegin(0x98, GX_VTXFMT0, 8)` and writes eight all-zero vertices laid out
  exactly as the current vertex format describes (2 or 3 position elements, 3
  normal elements, 3 or 4 colour bytes per colour, 1 or 2 tex elements, each in
  the VAT component type).

The Ghidra export collapses every run of same-address FIFO stores into one
`_DAT_cc008000 = _DAT_cc008000 & 0xffffff` (or `& 0xffff`) statement, which hides
both the store width and the store count. The real instructions were read from
`work/dol/main.elf`: `stb r4,-32768(r3)`, `sth r4,-32768(r3)` and
`stfs f0,-32768(r3)` with `r3 = 0xCC018000 - 0x8000 = 0xCC008000`, `r4 = 0` and
`f0` loaded from sdata2 `0x8042EF1C` (= `+0.0`). That is why the body's helper
counts components explicitly instead of copying the collapsed masks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8029BAB0` | `fn_8029bab0` | 1484 | medium |

## Deliberate skips

Twelve entries of the bucket stay untranslated after a full read. All twelve are
**live GQR type-switch samplers**: they read `unaff_GQR0`/`unaff_GQR3` and pick
the stored element width from the register's type field
(`(GQR >> 16) & 7` = 0 → float, 4/6 → 8-bit, 5/7 → 16-bit stores), while the
value comes from a live `ldexpf`/`psq_st` pair. The export records neither the
GQR configuration nor the resulting byte width, so any transcription would have
to guess the layout of every stored vertex component.

| address | bytes | GQR type-switch sites | what it does |
|---|---:|---:|---|
| `0x80290690` | 1628 | 30 | per-channel sampler ladder |
| `0x80290CF0` | 1032 | 17 | per-channel sampler ladder |
| `0x80291100` | 2208 | 19 | per-channel sampler ladder |
| `0x80292C90` | 5464 | 130 | largest sampler in the bucket |
| `0x80295C20` | 1232 | 79 | sampler with LCG dither |
| `0x80296160` | 116 | 6 | angle → (sin, cos) table sampler, two floats out |
| `0x802961E0` | 120 | 7 | same sampler, the pair split across two pointers |
| `0x80296260` | 332 | 13 | 3-angle matrix builder, returns the matrix |
| `0x802963B0` | 376 | 18 | 3-angle rotation-matrix writer, 12 floats |
| `0x80296820` | 1216 | 13 | particle modifier: three `0x802961E0` samplers and two `0x802963B0` matrix calls on LCG values |
| `0x80299840` | 6316 | 18 | per-vertex attribute emitter |
| `0x8029F900` | 3392 | 40 | per-vertex attribute emitter |

`0x80296160` and `0x802961E0` are worth one more sentence because they look
small enough to fake: both read `unaff_GQR3`/`unaff_GQR0` twice — once to scale
the incoming value (`ldexpf(bVar & 0x3f)`) and once to de-scale a table row
before the lerp — and both store the result through `char`/`short`/`float`
switches driven by `(GQR >> 16) & 7`. Their table rows
(`DAT_802F74A0 + 0x10 * index`) hold GQR-quantised pairs as well, so the element
width and the scale change together.

`0x80296820` is the caller that proves the block is live: it feeds
`0x802961E0` with three values derived from the `0x343FD`/`0x269EC3` LCG at
`param_3+0xEC`, then blends the six outputs back into `param_2`. Reproducing that
without the GQR configuration would change every byte it writes.

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 1,484 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 316 translated entries).
- No address was already in `ledger/functions.csv`; `0x8029BAB0` was one of the
  thirteen still untranslated in this bucket before the pass.
- The twelve skipped addresses are still absent from `ledger/functions.csv`, so
  a later pass can claim them once the GQR configuration is known.
- `node tools/check-lane-csv.mjs wave108` reports no problem naming
  `wave108-gx-band.csv`.
- `cargo check --message-format=short` reports no error in
  `src/render/gx_band.rs`. The three `unused_assignments` warnings at lines
  23188, 23235 and 23333 belong to the wave-107 body `fn_8028cee0` and predate
  this pass; the wave-108 body adds none.
- The one `sdata2` constant used by this pass was read from
  `work/dol/main.elf` (`r2 = 0x80433080`): `0x8042EF1C` = `0.0`, the word every
  float arm of the strip stores (`lfs f0,-0x4164(r2)`).
- The prologue helper `0x802DCD70` (whose Ghidra C looks like a 64-bit getter)
  was disassembled and is `__savegprlr_26`: it saves `r26..r31` at `r11-24..-4`
  and returns, so Ghidra's `iVar3`/`iVar4` pair is just `param_1`/`param_2`.
- The epilogue helper `0x802DCDBC` is the matching `__restgprlr_26`.
