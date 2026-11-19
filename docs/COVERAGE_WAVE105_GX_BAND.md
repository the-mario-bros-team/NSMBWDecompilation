# Coverage - Wave 105 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 105 pass over the `0x80290000..0x8029FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unrelocated `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave105-gx-band.csv` (the same five
rows).

Wave 102 translated nine entries of this bucket and deferred eighteen. Wave 105
re-read all eighteen and adds the five whose `f14..f31` paired-single ladders are
**dead stack spills** - values written by `psq_st` and never read again, which
Ghidra renders as `ldexpf`/`CONCAT` noise. Five bodies, 6,004 bytes. The
thirteen still untranslated entries are listed with their exact blocker under
"Deliberate skips"; ten of them write GQR-quantised bytes/halfwords to caller
memory and cannot be reproduced without knowing the configured GQR contents.

`status` is `translated` for all five rows; `confidence` follows each body's
`# Confidence` line (all `medium`, because the shared draw callees
`0x80296820`/`0x80296CE0` and the GX helpers outside this band are called
opaquely).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 5 | 6,004 | 0 | 5 |

## What this pass covers

- **Particle ring emitter** (`0x80298720`, 1,120 bytes). Three or four LCG
  advances per particle feed the distance factor (a plain lerp, or its square
  root through `0x80237BA0` when bit `0x01000000` is set), the angle offset and
  the `2r-1` vertical term. The `(x,0,z)` ring vector and the `(x,y,z)` middle
  vector are normalised separately and handed to `0x80296820` with the
  `(sin,0,-cos)` axis; the 16-bit descriptor is shrunk by `0x80296CE0` and the
  emitter's vtable slot `+0x14` draws the particle.
- **Ring/sweep driver** (`0x80298B80`, 848 bytes). Clamps the three `params`
  radii to `FLT_EPSILON`, picks the base angle from `params[2]` or one LCG draw,
  and either calls the ring emitter once or emits `count` rings whose angular
  step is `(params[3]-params[2])/u16[node+0x32]` - divided by `n-1` when the
  `fmod`-shaped reduction lands within one `2*pi/32768` step of the wrap
  boundary. The y spread is `2*radius_z*(i/(count-1)-0.5)`.
- **Grid/spiral emitter** (`0x80298ED0`, 1,596 bytes). The same family with two
  trig evaluations per particle: the ring angle advances by `2*pi/count` per row
  of `count` particles (or the outer count is `count*count`), the ellipse
  compression is `(100-params[1])/(100+params[1])`, and the `[-1,1]` shape terms
  build three vectors that `0x80296820` folds into the draw record.
- **Double-sided spawn helper** (`0x80299510`, 816 bytes). Normalises `dir`,
  copies `vec` as the offset, replaces it with three `[-1,1]` LCG draws when
  `vec` is zero, builds the `(x,0,z)` axis the same way (two draws), calls
  `0x80296820`, shrinks `r8` through `0x80296CE0` and draws through the vtable.
- **Per-channel texgen builder** (`0x8029C820`, 1,624 bytes). Three channels of
  cached-object + wrap-bit comparison, texture-object rebuild
  (`0x801C6D90`/`0x801C7520`/`0x801C7550`/`0x801C6FA0`), LOD bias
  `-1.0 + u8[object+0x14]`, identity-plus-rotation matrix with the two scale
  terms `0.5+cos*x-sin*y` / `0.5+sin*x+cos*y`, four texgen sign/bit flags, then
  either a direct `0x801C9A80` submit or a `0x801C0540` scene multiply first.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80298720` | `fn_80298720` | 1120 | medium |
| `0x80298B80` | `fn_80298b80` | 848 | medium |
| `0x80298ED0` | `fn_80298ed0` | 1596 | medium |
| `0x80299510` | `fn_80299510` | 816 | medium |
| `0x8029C820` | `fn_8029c820` | 1624 | medium |

## Deliberate skips

Thirteen entries of the bucket stay untranslated after a full read. They fall
into four groups.

1. **Live GQR-quantised stores** (10 entries): `0x80290690`, `0x80290CF0`,
   `0x80291100`, `0x80292C90`, `0x80296160`, `0x802961E0`, `0x80296260`,
   `0x802963B0`, `0x80296820`, `0x8029F900`. In these the `ldexpf`/`CONCAT`
   arms are not dead: the result is stored through `char*`/`short*` into vertex,
   keyframe or stack records, so the lane's `psq_st` type and scale decide every
   written byte. `0x80296160`/`0x802961E0`/`0x80296260`/`0x802963B0` additionally
   read the integer part back out of the low byte of that store to index the
   `0x802F74A0` key table, which only works for the quantised arm. Guessing the
   GQR would change the table index, not just the rounding. `0x80296820` is the
   shared transform emitter called opaquely by four of this pass's bodies.
2. **Dense GQR scratch** (1 entry): `0x80295C20` (1,232 bytes, `void`). Every
   live value is packed with `psq_st` into a stack temporary and unpacked again
   with `psq_l`; there is no plain-float read of the same slot to prove the type,
   so the whole body depends on the GQR contents.
3. **Giant multi-group driver** (1 entry): `0x80299840` (6,316 bytes). Seven
   repeated 0x80299510 emission groups plus a GQR argument marshalling layer;
   the export is 1,400 lines and the group parameters are only visible through
   the packed records. A faithful body needs the raw instruction stream.
4. **Export defect** (1 entry): `0x8029BAB0` (1,484 bytes). All TEV FIFO writes
   print as `_DAT_cc008000 = _DAT_cc008000 & 0xffffff` / `& 0xffff`; the word
   actually constructed from the five channel descriptors is not recovered, so
   the emitter cannot be written honestly.

## Verification

- Every address and byte size matches `functions.tsv` (5 rows, 6,004 bytes:
  1,120 + 848 + 1,596 + 816 + 1,624).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 312 translated entries).
- No address was already in `ledger/functions.csv`; all five were still
  untranslated before this pass.
- `node tools/check-lane-csv.mjs wave105` reports `OK` and lists
  `wave105-gx-band.csv: 5 rows`.
- `cargo check --message-format=short` reports no error in
  `src/render/gx_band.rs`.
- The `sdata2` constants used by this pass were read from
  `work/dol/main.elf` (`r2 = 0x80433080`): `0x8042EE50` = 65536.0,
  `0x8042EE54` = 100.0, `0x8042EE58` = 0.0, `0x8042EE5C` = 1.0,
  `0x8042EE60` = 2.0, `0x8042EE64` = 0.01, `0x8042EE78`/`0x8042EE98` =
  `FLT_EPSILON`, `0x8042EE7C`/`0x8042EEA8` = pi, `0x8042EE80` = `2*pi` as
  `f64`, `0x8042EE88`/`0x8042EEB8` = 0.0001917476, `0x8042EE8C`/`0x8042EEBC` =
  6.2829938, `0x8042EE90` = 0.5, `0x8042EEA4` = 2.0, `0x8042EEB0` = 2.392699 as
  `f64`, `0x8042EEC0` = 100.0, `0x8042EEC4` = 1.0, `0x8042EEC8` = 0.01,
  `0x8042EECC` = `2*pi` as `f32`, `0x8042EEE0` = `FLT_MIN`, `0x8042EEE4` =
  65536.0, `0x8042EEE8` = 2.0, `0x8042EEEC` = 1.0, `0x8042EEF0` = 0.0,
  `0x8042EEF4` = 0.01, `0x8042EF18` = 1.0, `0x8042EF1C` = 0.0,
  `0x8042EF24` = -1.0 and `0x8042EF28` = 0.5.
- The `f1` scale argument that Ghidra prints as a spurious `undefined8` was
  verified from the raw stream: `fmr f24,f1` at `0x802987C4`, `fmr f26,f1` at
  `0x80298BE4`, `fmr f31,f1` at `0x80298F74` and `0x8029952C`, each forwarded to
  the shared callee at `0x80298A00`, `0x80298DD4`, `0x80299350` and `0x80299758`.
- The missing `f1` angle of the `0x802961E0` call in `0x8029C820` was verified
  from the raw stream: `lfs f1,100(r28)` at `0x8029CC0C`, i.e.
  `f32[source+0x64]` of the per-channel record.
