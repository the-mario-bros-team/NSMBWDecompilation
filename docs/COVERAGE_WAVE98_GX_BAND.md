# Coverage - Wave 98 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 98 pass over the `0x80290000..0x8029FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unrelocated `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The module already
held 229 translated entries from waves 1-96; this pass adds 19 more (248).

`status` is `translated` for every row; `confidence` follows the body's
`# Confidence` line. Every row cites the exact export file it was read from.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 19 | 4556 | 9 | 10 |

## What this pass covers

- **Leaf helpers and empty hooks** (`0x802946E0`, `0x80294B20`, `0x80295C00`,
  `0x80296750`, `0x80296DC0`, `0x8029F5E0`, `0x8029F630`). Three bare `blr`
  entries, the fixed-address getter `0x80423170`, the three-word copy that loads
  `+0x1C`/`+0x2C` before the first store, and the two small zero-fill helpers
  used by the `0x30`-byte command records.
- **Vtable store and release wrapper** (`0x802919A0`, `0x802919B0`). A single
  `0x8034F5D8` store at `+0x1C`, and the `object != 0 && count > 0` release
  guard around `0x802B9280` that returns the object.
- **Text-run copy-constructor** (`0x80291A10`). The `0x7E`-byte block copy plus
  the three per-axis phase advances driven by the `+0xEC` RNG; each axis is
  quantised through `0x802E7E44` after the `128/pi * phase - 0.5` scaling.
- **Gradient ramp** (`0x802925D0`). The five-mode alpha/colour ramp on the
  context bytes `+0x10D..+0x111`. The period is `u16[+0x10E]` widened by
  `u8 * u16 * s8 / 0x319C`, the phase is `(u16[this+0xDC] - 1) mod period`, and
  mode 5 is `128 + u8[+0x111] * sin(40.74366f * 2pi * phase / period)`.
- **Child spawn helper** (`0x80292B60`). The factory probe at virtual `+0x4C`
  and the create call at the node's virtual `+0x10` (float + eight words),
  then the `+0xE2` credit bump, the `manager+0x38` list insert, the `+0x50`
  live count and `node+0x0C = 1`.
- **Draw-state packet submit** (`0x802941F0`). A `0x128`-byte stack packet is
  filled from `r4` (words `+0x00..+0x54`, the `+0x60`/`+0x6C` bytes widened to
  `+0xC4`/`+0xA8`, then the remaining words), eight words at `+0xA0..+0xC0` are
  re-pointed at copied fields, and `*(this+0x2C)`'s virtual `+0x0C` slot is
  called with `(receiver, block, this)`. A non-zero scene scale at
  `state+0x150` refills a `3x4` matrix through `0x80289070` first.
- **Material-name resolve family** (`0x802951E0`, `0x80295300`, `0x80295400`).
  Two self-relative name tables walked with length-prefixed `u16` entries and
  resolved through `0x80294E90` / `0x80294CA0`, plus the per-index driver that
  patches the three slots at `payload+0x74`, dispatches sub-records with
  `+2 == 4` / `+2 == 5 or 2` to the two resolvers, and toggles bit
  `0x40000000` of `record+8`.
- **Billboard basis build** (`0x80295860`). The `1 - |z|` guard, the
  `d * rsqrt(d)` length through `0x80237BA0`, and the twelve-float store with a
  zero homogeneous column.
- **Euler extraction** (`0x80295A90`). Three column lengths through
  `0x802966F0`, `asin(-m[2][0] / len0)` clamped to `-1..1`, and the
  `0x80237C40` branch that selects either one `atan2` (planar case) or two.
- **Single-vertex GX emitter** (`0x8029F640`). `GXBegin(GX_QUADS, 0, 4)`, one
  `stfs` of `a[+8] + b[+8]` to the FIFO at `0xCC008000`, and the optional
  matrix-index byte `3`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802919A0` | `fn_802919a0` | 16 | high |
| `0x802919B0` | `fn_802919b0` | 64 | high |
| `0x80291A10` | `fn_80291a10` | 792 | medium |
| `0x802925D0` | `fn_802925d0` | 564 | medium |
| `0x80292B60` | `fn_80292b60` | 300 | medium |
| `0x802941F0` | `fn_802941f0` | 592 | medium |
| `0x802946E0` | `fn_802946e0` | 4 | high |
| `0x80294B20` | `fn_80294b20` | 12 | high |
| `0x802951E0` | `fn_802951e0` | 288 | medium |
| `0x80295300` | `fn_80295300` | 252 | medium |
| `0x80295400` | `fn_80295400` | 572 | medium |
| `0x80295860` | `fn_80295860` | 260 | medium |
| `0x80295A90` | `fn_80295a90` | 364 | medium |
| `0x80295C00` | `fn_80295c00` | 28 | high |
| `0x80296750` | `fn_80296750` | 16 | high |
| `0x80296DC0` | `fn_80296dc0` | 12 | high |
| `0x8029F5E0` | `fn_8029f5e0` | 4 | high |
| `0x8029F630` | `fn_8029f630` | 4 | high |
| `0x8029F640` | `fn_8029f640` | 412 | medium |

## Deliberate skips

The bucket still contains 27 untranslated entries. Each was read in full before
being rejected.

- **Paired-single arithmetic that reaches the live values**
  (`0x80290690`, `0x80290CF0`, `0x80291100`, `0x80292C90`, `0x802946F0`,
  `0x80295C20`, `0x802960F0`, `0x80296160`, `0x802961E0`, `0x80296260`,
  `0x802963B0`, `0x80296530`, `0x802965B0`, `0x80296640`, `0x802966F0`,
  `0x80296820`, `0x80298720`, `0x80298ED0`, `0x80299510`, `0x8029C820`,
  `0x8029CFC0`). In these the `ldexpf`/`unaff_GQR0` arms *decide* the stored
  value (a byte/halfword fixed-point quantity or a scaled float), so the live
  quantiser type is part of the semantics. `0x80296820` is the clearest loss:
  its second `0x802963B0` call is printed with only two of the three scale
  factors, so a faithful body cannot be written from this export.
- **`0x80298B80`** (848 bytes). Same blocker as `0x80296820`: its
  `0x802963B0` call is printed with a different argument count than the
  three-scale prototype, so one factor is missing from the export.
- **`0x8029BAB0`** (1,484 bytes). No GQR at all, but the 69 FIFO writes are
  printed as `DAT_cc008000 & 0xffffff` / `& 0xffff` expressions that no store
  width can produce; the real per-attribute packing is not recoverable from the
  export.
- **`0x80294B30`** (352 bytes). A pure pointer-fixup walk over nested
  self-relative word offsets; the export renders it with overlapping
  `ushort *`/`ushort **` casts whose exact aliasing cannot be proven without
  the raw instruction stream.
- **Large remaining GQR functions** (`0x80299840`, `0x8029F900`). 6,316 and
  3,392 bytes of vertex/emitter code whose entire value path is paired-single.
  They stay deferred until a raw `psq` pass exists.

## Verification

- Every address and byte size matches `functions.tsv` (19 rows, 4,556 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 248
  translated entries and this pass added 19.
- `node tools/check-lane-csv.mjs wave98` prints `OK` (19 rows for this lane, no
  duplicate or malformed row).
- `cargo check --message-format=short` reports no error or warning in
  `src/render/gx_band.rs`. The remaining workspace errors at the time of
  writing are in `src/runtime/os.rs`, owned by another Wave 98 lane.
- Constants used by this pass were read from `work/dol/main.elf` with the
  documented mapping (`0x8042B080` at file `0x350200`, `r2 = 0x80433080`); the
  values are listed in the module header.
