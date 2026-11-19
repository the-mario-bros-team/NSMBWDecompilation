# Coverage - Wave 176 lane B (`game::bases`, 416-420 byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 5 | 2,092 | 4 | 1 |

Five bodies from the 416-420 byte band. Source of the bodies:
`work/wave176/laneB_block.rs`; per-function ledger:
`ledger/agents/wave176-laneb.csv`.

## What this pass covers

- **Course-entry hand-over (1 body).** `0x8079F5F0` is the body that follows the
  `0x3388`/`0x8081B738` + `"F7C0"` gate already documented for `fn_80799f10`.
  It rejects with `0` unless both tests match the `this+0x230` record, then marks
  course `+0x3B3`, publishes `0x80100D90(this)`'s two floats into
  `[0x80429C30] + 0x1B4/+0x1B8`, raises `+0x1C2`, requests the scene exit through
  `0x800B0DB0(5)`/`0x800B0A20(0x3C)`, binds the entry's `"W7C0"` model through
  `0x801002C0`, copies the entry's three floats into `0x80C71158..0x80C71160`,
  lowers every `+0x184` node's `+0xB0` by `1000.0`, and pushes
  `fn_8079f790(this, 6, 0, 4)`.
- **Stick-driven aim pair (1 body).** `0x807C0600` rejects the frame with
  `0x8019CC70([0x8042A4A8])`/`1` on pad status bits `0x800`/`0x200`; otherwise
  `+0x6AC` moves by `11.0` (`4.0` in mode `6`) on buttons `0x2`/`0x1`, `+0x6A4`
  by `10.0` on `0x8`/`0x4`, `+0x6A8` is refreshed from `0x807C1130(this)` when
  `+0x66C != +0x660`, and `0x807C08B0(this, 0xF)` lands in
  `[0x80834470] + 0x4FC`.
- **Pendulum pivot frame (1 body).** `0x80683200` is the sibling of the already
  translated `fn_80680fd0`: `fn_80682f40(this)` + `0x806830D0(this)`, the signed
  `+0xAB2` step added to `+0x104`/`+0xAB4`/`+0xAB6`/`+0xAB8` (subtracted when
  `+0x348` is set), the `+0x92C` node placed with `(400.0, 0.0, -400.0, 0.0,
  -400.0)` and angle `16099 + yaw` through `0x800DB7D0` plus its `+0x0C`/`+0x10`
  slots, three `0xE4` blocks at `+0x3D4` and three at `+0x680` flushed through
  `0x8007FDA0`, and the four `+0xABC + i*4` channels pushed into live players
  through `0x80057820` when `+0xAA8` is set.
- **`lift_zen_kaihei` constructor (1 body).** `0x806E5370` unpacks the spawn word
  `+0x4` into the `0x800D77D0` argument set, runs `fn_806e52a0`, writes
  `+0x318 = 48.0`, `+0x31C = 544.0 + 16.0*index`,
  `+0x320 = 24.0`, `+0x324 = -16.0 -/+ 8.0*index`, `+0xD0..+0xD8 =
  24.0/-16.0/0.0`, builds the `+0x4A0` animation with
  `0.0/0.0/48.0/-32.0` and callbacks `0x806E4CC0`/`0x806E4D10`, sets
  `+0x4EC`/`+0x4F0`/`+0x4F4`/`+0x570`, and finishes with
  `fn_806e5300(this, flags)` + `fn_806e52d0(this)`, returning `1`.
- **Screen-space cue bind (1 body).** `0x807583D0` builds the two-float record at
  `r1+0x18` from the actor position minus `0.5 * (width / (width/608.0))` and
  minus `0.5 * edge`, where `edge` is `0.5480769 * width` when
  `0x800B54B0(screen)` is zero and the screen height otherwise, then binds
  `table[(+0x4 & 0xF) + (4 when +0x238 == 3)]` and the `id` argument into the
  interface at `+0x168` through `0x8006A570` + `0x8019A490`. It is the callee of
  the five already-translated `fn_80758xxx` bodies, which pass `0`/`0x80832570`/
  `0x80832598` and ids `0x8E`/`0x8F`/`0x90`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8079F5F0` | `fn_8079f5f0` | 416 | high |
| `0x807C0600` | `fn_807c0600` | 416 | high |
| `0x80683200` | `fn_80683200` | 420 | high |
| `0x806E5370` | `fn_806e5370` | 420 | high |
| `0x807583D0` | `fn_807583d0` | 420 | medium |

## Deferred

None. All five assigned bodies are translated.

## Corrections and notes

- **Three dropped `this` arguments recovered.** `0x8079F5F0` and `0x806E5370`
  open with `bl 0x802DCD74`/`bl 0x802DCD70` and `0x807583D0` with
  `bl 0x802DCD70`; those are `_savegpr_*` helpers whose `r3` survives, so the
  exports' `void` signatures are wrong. The raw `mr r31,r3` / `lwz r30,0x4(r3)`
  restores the argument and the three bodies take `this`. `0x8079F5F0` and
  `0x806E5370` also return a live `r3` (`fn_8079f790`'s result / `li r3,1`), so
  they are emitted as `-> u32`.
- **The shared decoder mislabels `fmr`/`fneg`.** `0x80683200`'s three register
  copies print as `F63_8 f3,f0,f1` etc. because `ppc159.js` reads only the
  5-bit XO field. The 10-bit XO values are `40` (`0xFC600850` → `fneg f3,f1`)
  and `72` twice (`0xFC801090` → `fmr f4,f2`, `0xFCA01890` → `fmr f5,f3`), so the
  box is `(400.0, 0.0, -400.0, 0.0, -400.0)` — the same argument set
  `fn_80680fd0` uses. `0x806E5370`'s `0xFC400890` is `fmr f2,f1`.
- **`0x8079F5F0` table index is provably zero.** `r27` is initialised to `-1`,
  only the matching gate arm stores `0`, and the body is entered only when
  `r27 == 0` (`bc 12,0` = `blt` and `bc 4,0` = `bge` both leave). The `mulli
  r27,r27,36` therefore addresses entry 0 of the stride-`0x24` table at
  `0x8081B738`, which reads as `{6, "F7C0", 6, 0, 4, "W7C0", 0.0, 0.0, 0.0}`.
- **`0x8079F5F0` dead stack copies.** The `+0x184` walk copies `+0xAC`/`+0xB0`/
  `+0xB4` into `r1+0x10`/`+0x14`/`+0x18` on every pass and never reads them back
  (the frame has no other user); only the `+0xB0 -= 1000.0` store is kept.
- **`0x807C0600` `FUN_807c1130` is an `f32` source.** The export types it
  `void` and the caller stores `f1` (`stfs f1,0x6a8(r31)`) after the `bl`; the
  live `f1` is the `0x807C1190` result (or the `0x807D0A6C` seed on the guard
  edge), so it is called with `call_f32_1`. `0x807C1130` is therefore the
  `try`-style wrapper, not an argument-taking helper.
- **`0x806E5370` `2^52` idiom.** The `int -> float` conversions appear as
  `(double)CONCAT44(0x43300000, index) - 2^52` with an `fsubs`: the operands are
  doubles, the subtraction is exact and the result is rounded to single, so the
  value is `index as f32`. `0x807CBCB0` holds the `0x4330000000000000` double and
  `0x807CBCC0` is *not* string data — the nearby `dModelPlayer_Manager:StateID_*`
  text starts one page later at `0x807DBCA0`.
- **`0x806E5370` `0x320` slot.** Ghidra prints `*(iVar8 + 800)` as a
  `undefined4` store; the raw `d0bb0320 stfs` is a float store of `24.0`, so the
  body uses `write_f32`.
- **`0x807583D0` ratio chain is literal, not simplified.** X uses
  `(width / (width/608.0)) * 0.5` and Y uses `height / (height/edge)`, both read
  from the `+4`/`+6` samples of `[[0x8042A480]]`; the two divisions are kept so
  the shipped zero-width `NaN` behaviour is preserved.
- **`0x807583D0` export `in_f31`/`CONCAT13` blocks are GQR0 noise.** The
  `psq_st f31,0x58(r1)` save plus its `ldexpf` guards produce the pseudo
  `local_8 = (float)CONCAT13(...)` prologue; there is no paired-single data path
  and no incoming float argument. The `local_48`/`local_44` pair is the two-float
  record at `r1+0x18`, and `r1+0x08`/`r1+0x10` are the two `0x8006A570`
  destinations.

## Verification

- `work/wave176/laneB_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch, field offset and
  call target was re-read from the raw words of `work/rels/elf/d_basesNP.elf`.
- The lane CSV is written against `src/game/bases.rs` as the brief requires; the
  bodies live in `work/wave176/laneB_block.rs` until the coordinator appends
  them.
