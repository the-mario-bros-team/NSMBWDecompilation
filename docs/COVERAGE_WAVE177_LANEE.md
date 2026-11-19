# Coverage - Wave 177 lane E (`game::bases`, four 456-byte bodies)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 4 | 1,824 | 2 | 2 |

Four bodies, all exactly 456 bytes: a slope/launch reaction pair, a child-chain
status tally, the five-state slot machine, and the world-map screen template
rebuild. Source of the bodies: `work/wave177/laneE_block.rs`; per-function
ledger: `ledger/agents/wave177-lanee.csv`.

## What this pass covers

- **Reaction/launch pair (1 body).** `0x806B4BA0` is the sibling of the already
  translated [`fn_806c3790`]: same `0x806B4ED0(this, 0)`, same `0x80170980`
  normalise of `(other - this)` in the XZ plane with the vertical component
  folded to `max(dy, 0.0)`, same `+0x2A0` gain applied to
  `(other.y - this[0x6C4])`, same `+0x294` scale, same `+0x2B8` dispatch and
  same `0x806B4F40`/virtual-`+0x3FC` tail. It has **no** `+0x898` ground-band
  step, and it adds the mode-dependent limit `other + 0x1090 == 3 ? +0x29C :
  +0x298` clamped with `min` onto the vertical component.
- **Child-chain status tally (1 body).** `0x806CFB30(this, out) -> u32` walks
  the `+0x3E0` chain exactly like [`fn_806d3680`]: kind `1`/`2` children are
  counted and contribute the `+0x10D4` bits `0x2`/`0x20`/`0x40` to `out` as
  `1`/`2`/`4` (the `0x2` bit is additionally gated on `+0x665` having its sign
  bit clear); kind `3` children that answer the `0x8006EF40(+0x1EC, 0)` probe
  get the `node + 0x24 - 92.0 < 0.0` latch written to `+0x459`, then either
  `+0x34A |= 4` (id `0x85`) or `0x800656C0(actor, slot, flag)` with
  `flag = 1` for id `0x193`/`0x191`/`0x3C`; kind-`0`/id-`0x19B` children that
  pass the same probe get `+0x34A |= 4`.
- **Slot appear/wait/fade machine (1 body).** `0x807630C0` dispatches on the
  state word `+0x25C` over the record table `0x8081_5494 + (this[4] & 0xFF) *
  0x18` (`+0x10` `f32` amplitude = `1.0` for all six slots, `+0x14`/`+0x16`
  `s16` counter bounds, `+0x18` `s16` fade length `30/15/15/15/15/15`). State
  `2` emits sound `0x2D` at `+0xAC`, seeds the timer with `high - low` and
  writes the amplitude; state `3` ramps `0.0 -> -amplitude` over the last
  `fade` frames; states `0` and `4` call the class virtual `+0x68`, state `4`
  first resetting the animation value to `0.0`.
- **World-map screen rebuild (1 body).** `0x807B7D60` is the alternate arm of
  the already translated [`fn_807b7f30`] (`0x807B7F30` calls it as
  `0x807B7D60(this)` when `0x80315B98 != 0`). It copies the template at
  `0x8035_8D54` into `*(0x8042_9F7C)` field by field, fills the slot tables
  `0x8035_4E60`/`0x8035_4E50` from the four indices at template `+0x48..+0x58`
  and the four bytes at `+0x68..+0x6C`, block-copies four `0x10`-byte records
  to `0x8035_4E70`/`+0x80`/`+0x90`/`+0xA0`, mirrors `0x8042A179`/`0x8042A181`
  into `0x8042A178`/`0x8042A180`, and clears `screen + 0x6C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806B4BA0` | `fn_806b4ba0` | 456 | medium |
| `0x806CFB30` | `fn_806cfb30` | 456 | high |
| `0x807630C0` | `fn_807630c0` | 456 | medium |
| `0x807B7D60` | `fn_807b7d60` | 456 | high |

## Deferred

None.

## Corrections and notes

- **`0x806B4BA0` drops a dead stack block.** `0x806B4C98..0x806B4CBC` builds a
  `(other.x + vx, other.y + 1.0, other.z)` triple at `r1+0x08` and a second
  `(other.x, other.y + 1.0, other.z)` triple at `r1+0x14`. No call between the
  stores and the epilogue takes `r1+8` as an argument (the only calls use
  `r3 = this`, `r3 = other`, or `r3 = r1+0x20`), and the epilogue only reloads
  `r1+0x38..0x54`, so both triples are dead. The `0x807CAA68` (`1.0`) constant
  is consumed only by those stores, which is why it does not appear in the
  translation. `0x807CAA64` is `0.0`.
- **`0x806B4BA0`'s two folds have opposite senses.** `0x806B4BFC`
  (`fc000840`-class `fcmpo` of `1.0`-slot `0.0` against `dy`) plus
  `bc 4,0` is the `max(dy, 0.0)` select at `0x806B4C10`; `0x806B4CF4`
  (`fc000840` of `vector[1]` against the virtual limit) plus `bc 4,1`
  (`fcmr` on the greater-than bit) is the `min(vector[1], limit)` select. Both
  were re-read from the words rather than trusted from the export.
- **`0x806CFB30` has two arguments, not zero.** The export's
  `void FUN_806cfb30(void)` and its `uVar12 = func_0x802dcd74()` split are the
  `_savegpr_26` prologue: the raw words immediately do `mr r27,r3`,
  `mr r28,r4` and `stw r30,0x0(r4)`. `0x802DCDC0` is `_restgpr_26`, which keeps
  the `mr r3,r30` counter, so the body returns the count. Neither helper is
  emitted.
- **`0x806CFB30`'s bit tests are `rlwinm` masks.** `0x5400CFFF`
  (`rlwinm 25,31,31`) is bit 7 of the `+0x665` byte, and the `rc` form makes
  `bc 4,2` skip the block when the bit is *clear*. `MB 30`/`MB 26`/`MB 25`
  select `0x2`/`0x20`/`0x40` of `+0x10D4`.
- **`0x807630C0`'s case-3 constant is `0.0`, not `1.0`.** Ghidra prints
  `DAT_807cee64 - ratio`; the raw word is `0xC024EE5C`, i.e. `lfs f1,
  -0x11A4(r4)` with `r4 = 0x807D0000` = `0x807CEE5C`, which is the `0.0f`
  that case `4` also loads (`0x807CEE64` is the neighbouring `1.0f`). The ramp
  is therefore `(0.0 - ratio) * amplitude`. The `+0x18` and counter conversions
  use the `0x43300000`/`xoris 0x8000` pair with the `0x807CEE68` magic
  double, so the ratio is computed in `f64` and narrowed once.
- **`0x807B7D60` takes no arguments.** `fn_807b7f30` forwards `this`, but the
  body executes `lis r3,0x8043` before any read of `r3`, so the parameter is
  discarded. Ghidra's `void FUN_807b7d60(void)` is right.
- **`0x807B7D60`'s template copy is field-level, not a block move.** The five
  bytes at template `+0x18..+0x1C` are `lbz`/`stb` pairs into
  `screen + 0x20..0x24`, so `screen + 0x25..0x27` are left untouched; a plain
  `0x30`-byte copy would have overwritten them.
- **`0x807B7D60`'s `ctr = 2` loop is two slots per pass.** Each pass stores the
  running counter and the matching `+0x68 + k` byte as **words** (`stwx`) into
  `0x8035_4E60`/`0x8035_4E50` indexed by `template[+0x48 + 4k]`, and advances
  the destination pointers by `8`, which makes the four `0x10`-byte record
  copies contiguous (`+0x58`, `+0x7C`, `+0x8C`, `+0x6C`). It is modelled as the
  equivalent `k = 0..4` pass; `0x8035_4E60` is the same table
  [`fn_806027e0`] reads as `0x8035_4E60 + slot * 4`.
- No `psq_l`/`psq_st`, GQR or `_savegpr`/`_restgpr` artifact is emitted; the
  exports' `unaff_GQR0`/`ldexpf` blocks in `0x806B4BA0`/`0x806CFB30` are
  compiler noise (`GQR0 = 0`).

## Verification

- The lane-E block type-checks standalone (`rustc --edition 2021
  --emit=metadata`) against stubbed copies of the real helper ABIs
  (`platform::read_*`/`write_*`/`callN`, `abi_call1..4!`, `actor_method0`,
  `actor_method0_f32`, `call_f32_u32`, `call_f32x2_u32x4`, `fn_806b4f40`);
  the harness lives outside the repo in the session temp directory.
- All four evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was re-read from `work/rels/elf/d_basesNP.elf` with a raw-word dumper
  (`0x807CAA64` = `0.0`, `0x807CAA68` = `1.0`, `0x807CB450` = `0.0`,
  `0x807CB45C` = `-92.0`, `0x807CEE5C` = `0.0`, `0x807CEE64` = `1.0`,
  `0x807CEE68` = the int-to-double magic, and the six
  `0x8081_5494 + i * 0x18` records).
- Only helpers that already exist in `src/game/bases.rs` are called
  ([`fn_806b4f40`], `actor_method0`, `actor_method0_f32`, `call_f32_u32`,
  `call_f32x2_u32x4`); `0x806B4ED0`, `0x80105240`, `0x801680E0`,
  `0x80170980` and `0x800656C0`/`0x8006EF40` go through `abi_callN!` or the
  typed `call_f32_*` helpers.
- `src/game/bases.rs` was not modified by this lane.
