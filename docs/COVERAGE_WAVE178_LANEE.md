# Coverage - Wave 178 lane E (`game::bases`, four 484-492 byte bodies)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 4 | 1,952 | 3 | 1 |

Four bodies from the REL `d_basesNP`: the world-select carousel step, the
sand-pillar smoke-ring update, the `cobKoopaShip` ride state machine, and one
pass of the four-entry `+0x7BC` record sort. Source of the bodies:
`work/wave178/laneE_block.rs`; per-function ledger:
`ledger/agents/wave178-lanee.csv`.

## What this pass covers

- **World-select carousel step (1 body).** `0x807BC7E0` is the direct
  continuation of the already translated [`fn_807bb840`]/[`fn_807bb9f0`]/
  [`fn_807bbca0`] trio and shares the field set of [`fn_807bc310`]. Every call
  adds the signed `+0x2A6` delta to each pane's `+0x2C6` angle; every fifth
  call decrements the hold counter `+0x284`, moves `+0x288` by `+1`/`-1`
  depending on the sign of the delta, wraps it through `fn_807bb840`, and
  publishes `extsh((i16)+0x2A4 * wrapped)` (positively for a negative delta,
  negated otherwise) through `fn_807bb9f0`. At `+0x284 == 0` it publishes
  `+0x280` into `+0x27C` and then either calls `fn_807bbca0` (confirm bit `8`
  when `+0x2B4 == 0`, bit `4` otherwise) or re-arms `0x80834278` with a
  `+0x290 = 4` delay.
- **Sand-pillar smoke ring (1 body).** `0x806952D0` completes the
  [`fn_80695170`]/[`fn_80695260`]/[`fn_806955d0`]/[`fn_80695580`] group: it
  walks the ten `0x154`-byte records at `+0x548`, ages each record (`+0x0C`
  grows by `0.3`, clamped to the `2.0` at `+0x10`), runs `0x800B2D20` on the
  record, and retires the resolved target either through `fn_806955d0` when
  `dy >= +0x530 - 2.0` or through `fn_80695580` when `|dx| >= 30.72` and the
  `0x80070BA0` ground query at the pillar edge (`x - 48` for `dx < 0`, `x + 48`
  otherwise, query `y` = the record's cached `+0x08`) sets bit `0`.
- **Ship ride state machine (1 body).** `0x807BF930` is the state-`0/1/2`
  predecessor of the already translated [`fn_807bfe20`] (`0x807BFE20`). It
  shares that function's constants and shape but has two distinct `0x8015F140`
  tuples (`0.03/0.07/0.015` for states 0/1, `0.05/0.09/0.03` for state 2), and
  only state 2 republishes the speed
  `+0x5EC = +0x61C + +0x60C * (2750.0 - +0x61C)`.
- **Record sort pass (1 body).** `0x80637120` is the sorting neighbour of
  [`fn_80637380`] (which reads the same table with the same `0x3C` stride).
  Starting at record `index`, it runs `3 - index` passes; each pass compares the
  key word `+0x04` of record `k` with record `k + 1`, swaps them field by field
  when `key(k) < key(k + 1)` or when `+0x29(k) == 0 && +0x29(k + 1) != 0`, and
  otherwise only raises `+0x28` of the follower when the keys are equal.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807BC7E0` | `fn_807bc7e0` | 484 | high |
| `0x806952D0` | `fn_806952d0` | 488 | medium |
| `0x807BF930` | `fn_807bf930` | 488 | high |
| `0x80637120` | `fn_80637120` | 492 | high |

## Deferred

None.

## Corrections and notes

- **`0x806952D0` takes one argument.** The export's `void FUN_806952d0(void)`
  and its `func_0x802dcd70()` result are the `_savegpr_26` prologue; the raw
  words keep `r3` in `r26` and address the ring as `r3 + 0x548`, exactly like
  the translated `fn_80695170`/`fn_80695260`. Neither `0x802DCD70` nor
  `0x802DCDBC` is emitted.
- **`0x806952D0`'s ground-bit test is inverted in the export.** `0x8069541C` is
  `rlwinm rc r0,r3,0,31,31` (i.e. `r3 & 1`) followed by `bc 4,2` at
  `0x80695420`. `BO = 4` branches on the *false* value of the selected CR bit,
  so the `0x80695580` retire runs when the bit is **set**, not when it is
  clear. The `bc`/`BO` sense was cross-checked on the `0x80695580` twin at
  `0x80695588`/`0x80695594` (`beq` = `bc 12,2`, `bne` = `bc 4,2`).
- **`0x806952D0`'s `fcmpo`/`cror` folds are `>=` tests.** `0x806953BC` and
  `0x806953E4` both use `cror 2,1,2` (`cr0.EQ = cr0.GT | cr0.EQ`) followed by
  `bc 4,2`, so the export's `<=`/`<` renderings are the branch polarity of the
  fold, not the source relation; the translation uses `dy >= top` and
  `|dx| >= 30.72`. `0x80695378` is a plain `bc 4,1` (skip when the new age is
  not greater than the limit), confirming the `0.3` clamp is a `min`.
- **`0x806952D0`'s `0x80070BA0` probe has two live floats.** The dead `stfs`
  pair at `0x8069540C`/`0x80695414` writes the record's `+0x04`/`+0x08` to
  `r1+8`/`r1+0xC`, but no call takes that address; the live operands are
  `f1 = axis_x ± 48` (from the `fadds`/`fsubs` at `0x806953F8`/`0x80695400`)
  and `f2 = record[+0x08]` (reloaded at `0x80695404`), which is why the probe is
  a `call_f32x2_u32`. `0x800B2D20` is `// unknown:` apart from taking the record
  pointer in `r3`.
- **`0x807BF930` swaps the two `0x8015F140` tuples.** The export prints the
  state-2 tuple (`0.05/0.09/0.03`, words `0x807D0B24`/`0x807D0B28`/`0x807D0B14`)
  in the state-0/1 arm; the raw loads at `0x807BF9A4..0x807BF9B4` are
  `0x807D0AC8`/`0x807D0B14`/`0x807D0B18`/`0x807D0B1C`
  (`1.0/0.03/0.07/0.015`) and the ones at `0x807BFA78..0x807BFA88` are the
  state-2 tuple.
- **`0x807BF930`'s only shared tail is state 2's.** States 0/1 run
  `0x801610B0` and the `|+0x60C - 1.0| <= FLT_EPSILON` test but never touch
  `+0x5EC`; state 2 runs the same call plus the speed republication. The
  former `FUN_807bf560` state-0 prelude, the `2750.0` sentinel
  (`0x8082155C`) and the `1 / (int)30` ramp (`0x8082156C`, `fctiwz`) are
  literal; `0x80821568` is the byte that selects the tint path.
- **`0x80637120` has two arguments and a real loop counter.** The export's
  `func_0x802dcd6c()`/`3 - iVar4` arithmetic is `_savegpr_27` plus
  `subfic r0,r4,3` loaded into the decrementer; `0x802DCDB8` is `_restgpr_27`.
  The record base is `this + 0x7BC + 0x3C * index`, not the `param_1`-relative
  offsets the export shows, and the guard at `0x806372E0` re-uses the key word
  already loaded in `r3`/`r4`.
- **`0x80637120`'s `+0x28` byte is not swapped.** Both records are set to `1`
  in the swap arm (the saved old `+0x28` byte is written to the follower at
  `0x80637278` and immediately overwritten by `stb r25,0x820` at `0x806372D8`),
  so the translation sets it on both sides. The `+0x00` byte is copied with
  `lbz`/`stb` only, while `+0x2C..+0x38` are copied with `lfs`/`stfs`.

## Verification

- The lane-E block type-checks standalone (`rustc --edition 2021
  --emit=metadata --crate-type lib`) against stubbed copies of the real helper
  ABIs (`platform::read_*`/`write_*`/`call1`/`call3`, `abi_call1!`/`abi_call3!`,
  `call_f32x2_u32`, `call_f32_u32x3`, `call_f32x4_u32`, `call_f64_u32`,
  `vmethod1`, `fn_807bb840`, `fn_807bb9f0`, `fn_807bbca0`, `fn_80695580`,
  `fn_806955d0`, `fn_807bf560`); the harness lives outside the repo in the
  session temp directory.
- All four evidence exports exist under `work/ghidra/rel-d_basesNP/export/`;
  every constant, branch and field offset was re-read from
  `work/rels/elf/d_basesNP.elf` with a raw-word dumper
  (`0x807D09F8` = int-to-double magic, `0x807C9E90` = `0.0`,
  `0x807C9E94` = `2.0`, `0x807C9F14` = `0.3`, `0x807C9F18` = `30.72`,
  `0x807C9F1C` = `48.0`, `0x807D0AC8` = `1.0`, `0x807D0B10` = `1.1920929e-7`,
  `0x807D0B14` = `0.03`, `0x807D0B18` = `0.07`, `0x807D0B1C` = `0.015`,
  `0x807D0B24` = `0.05`, `0x807D0B28` = `0.09`, `0x807D0B20` = `0.2`,
  `0x8082155C` = `2750.0`, `0x8082156C` = `30.0`, `0x80821568` = `1`).
- Only helpers that already exist in `src/game/bases.rs` are called
  (`fn_807bb840`, `fn_807bb9f0`, `fn_807bbca0`, `fn_80695580`, `fn_806955d0`,
  `fn_807bf560`, `vmethod1`, `call_f32x2_u32`, `call_f32_u32x3`,
  `call_f32x4_u32`, `call_f64_u32`); `0x801953A0`, `0x80162D00`,
  `0x800B2D20`, `0x80070BA0`, `0x8015F140`, `0x801610B0`, `0x807BE3D0` and
  `0x807BF5A0` go through `abi_callN!` or the typed `call_*` helpers.
- No `psq_l`/`psq_st`, GQR or `_savegpr`/`_restgpr` artifact is emitted; the
  exports' `unaff_GQR0`/`ldexpf` blocks in `0x806952D0` are compiler noise
  (`GQR0 = 0`).
- `src/game/bases.rs` was not modified by this lane.
