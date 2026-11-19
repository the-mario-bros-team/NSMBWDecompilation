# Coverage - Wave 174 lane E (`game::bases`, 384/388-byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 5 | 1,928 | 2 | 3 |

Three 384-byte bodies and two 388-byte bodies. Source of the bodies:
`work/wave174/laneE_block.rs`.

## What this pass covers

- **Cart-chain builder (1 body).** `0x806EECA0` allocates the `daLineTrain_c`
  chain object (`0x80064680` id `0x116`) plus `count` children (id `0x115`,
  `0x400` bytes). `count` is the sign-extended halfword at `+0x16`, the spacing
  step is `(s16)(+0x14) / count` truncated with `extsh`, and each child receives
  a `0x20`-byte record through `0x806EE8F0` before its first word is stored at
  `+0x398 + i*4`. Sits next to the existing constructor `fn_806eebc0`, which
  clears the same `+0x394..+0x3BC` block.
- **Pad-class mode mapper (1 body).** `0x8078FF30` reads the pad manager at
  `0x80429F7C`, queries the slot through `0x800BBC80`/`0x800BBC70`, and writes
  the mode word `+0x214` plus latch `+0x218`. Same global, slot source and
  ladder style as the adjacent `fn_8078fc80`.
- **World-map floor re-seed (1 body).** `0x807C0D30` fires audio event `0x6B`,
  mirrors `+0x4B0..+0x4B8` into `+0x710..+0x718` when the two probe words are
  equal, otherwise takes `0x807C0C10` as the height and builds a signed
  `+0x6F8` spin from three `0x801C1650`/`0x802BE470` distance pairs. Ends by
  re-arming `0x80834470` through `0x807C3A70`/`0x807C3B00`/`0x807C3AF0`.
- **Slope reaction (1 body).** `0x806C3790` runs only when the other actor's
  `+0x38C` byte is `1`. It normalises `(dx, max(dy, 0), 0)` through `0x80170980`,
  adds `virtual +0x2A0 * (other.y - this[+0x6C4])`, scales by virtual `+0x294`,
  applies the `+0x898` ground push and the `10.0` clamp, then calls the other
  actor's virtual `+0x3FC(f1 = y, f2 = x, 1, 0x806B4F40(this), 0)`.
- **Pipe transform rebuild (1 body).** `0x80718CF0` takes the slot
  `(flags & 0xF)`, selects `[6, 10, 14, 18, 22]` from `0x808083D4`, and seeds
  position, `+0xD0..+0xE4`, the `+0x318..+0x324` band and `+0x5B0` range from the
  `0x807CCDF8` constant block, the `0x8082FF10..0x8082FF18` origin and the
  `0x808083xx` field constants; returns `1`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806EECA0` | `fn_806eeca0` | 384 | high |
| `0x8078FF30` | `fn_8078ff30` | 384 | high |
| `0x807C0D30` | `fn_807c0d30` | 384 | medium |
| `0x806C3790` | `fn_806c3790` | 388 | medium |
| `0x80718CF0` | `fn_80718cf0` | 388 | medium |

## Deferred

None.

## Corrections and notes

- The Ghidra exports of `0x806C3790` and `0x806EECA0` are both wrapped in
  `ldexpf`/`psq_st` guards. Per the established GQR0 rule these are compiler
  noise: `0x806EECA0` only stages `0x807C_BFCC` (0.0) and `0x807C_BFD0` (4.0)
  into `f30`/`f31` and both stores are overwritten before `0x806EE8F0` reads the
  record, so the loads are dropped.
- `0x806C3790`'s export reads several A-form products with the wrong operand.
  Re-reading the words gives `fmuls fD,fA,FRC` (FRC = bits 21-25), while
  `fadds`/`fsubs` use `FRB` (bits 16-20); e.g. `0xEC2100B2` is
  `fmuls f1,f1,f2`, `0xEC2400B2` is `fmuls f1,f4,f2`, and `0xEC00082A` is
  `fadds f0,f0,f1`. The body follows the words, which also reproduces the
  export's `local_28 * this[+0x898]` test exactly.
- `0x807C0D30`: the export prints `fcmpo`, but `0xFC1EF840` is a plain `fcmpu`
  (`XO = 0`). The branch is `bc 4,1`, i.e. taken when `f30 > f31` is false, so
  `+0x6F8` uses `+1.0` when the first distance exceeds the second.
- `0x80718CF0` calls `0x8007FA40` through `call_f64x4_u32x8`, matching the four
  existing `0x8007FA40` sites in this file; the 4th float slot is the `0.0`
  staged into `f4` from `0x807CCE00` by the surrounding stores.
