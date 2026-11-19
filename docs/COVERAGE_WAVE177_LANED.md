# Coverage - Wave 177 lane D (`game::bases`, 452/456-byte classes)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 4 | 1,812 | 3 | 1 |

Four bodies: three 452-byte and one 456-byte function. Source of the bodies:
`work/wave177/laneD_block.rs`.

## What this pass covers

- **`cobTsuta` pose rebuild (1 body).** `0x807946D0` is the build step that
  [`fn_80794540`] was already calling through `abi_call1!`. The low byte of
  `+0x04` selects a record: the name from `0x8081B4D4[index]` is resolved with
  `0x800F9530` and the per-class byte `0x807D0038[index]` chooses the
  `0x800FAD60`/`0x800FAE80` pair (value `1`) or the
  `0x800FADB0`/`0x800FAF70` pair (value `0`). A hit replaces the default
  `0.0` (`0x807D0058`) with the ramp mean [`fn_80794c00`] and the growth height
  [`fn_80794bf0`]; the body then drives the `+0x264` pane through
  `0x80168FE0`/`0x801680E0`, calls the `+0x1EC` vtable slot `+0x1C`, toggles
  `+0x340` around [`fn_80794480`], and repeats the pane writes for `+0x22C`
  (ramp) and `+0x29C` (growth).
- **Position snapshot + effect spawn (1 body).** `0x8079E310` copies
  `+0xAC..+0xB4` to `+0x264..+0x26C`, then fills `+0x1FC..+0x204`: the closed
  branch samples the `0x800FC480` handler (scale `0x8031D39C + 0x210`,
  position `+0xAC`, handle `+0x240`), the open branch samples the scene node
  `0x80100200` and normalises the difference with `0x80170980`. It publishes
  pose `9` through `0x80799960`, runs `0x800FDC80`, and ends in the
  `0x800F69F0` spawn (`this`, `this + 0x1FC`, `0`, `0x8031D39C + 0x1B0`,
  `this - snapshot`) with the two `0x8031D39C + 0x1B4`/`+0x1B8` scales.
- **Scene shutdown / render-state restore (1 body).** `0x807B90F0` rebinds the
  `0x80169220` slot to `0x8042A284 + 0x20C`, releases the ten `0x8016xxxx`
  layers, flushes the effect manager `0x8042A234` through `0x807BA750`, clears
  font slots `3`/`2`, and - when `0x80429F7C + 0x394` is set - runs the full
  second display-list pass, including the `+0xAFC` font loops `0xB..0xE` and
  `0x7..0xA`, and flushes the effect manager again.
- **Pitch ramp step (1 body).** `0x80648AA0` is the direction-latched sibling of
  the already translated [`fn_806485c0`], without its `silent` parameter. It
  runs [`fn_80648770`] on direction `0`, adds the step `+0x1A4B4` and clamps at
  `|+0x1A4B0|` on direction `1`, and subtracts the step and clamps at
  `0x807C850C` (`0.0`) otherwise, releasing [`fn_80648770`] once the latch
  clears; both moving directions post cue `0x305`/`0x306` while `+0x1A4F9` is
  clear, then the tail writes `+0x1A49C`, runs `0x80648470` and forwards the
  height pair to `0x800E4550`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80648AA0` | `fn_80648aa0` | 456 | high |
| `0x807946D0` | `fn_807946d0` | 452 | high |
| `0x8079E310` | `fn_8079e310` | 452 | medium |
| `0x807B90F0` | `fn_807b90f0` | 452 | high |

## Deferred

None.

## Corrections and notes

- `0x807946D0` carries the usual `unaff_GQR0`/`ldexpf` guards; its only GQR-0
  accesses are `psq_st f31,0x38(r1)`/`psq_st f30,0x28(r1)` plus their reloads,
  so the guards are compiler noise and are not emitted. The two constants used
  by the body were read from the REL: `0x807D0058 = 0.0` and
  `0x807D0038 = 00 01 01 00 ...`, i.e. `index 0 -> 0`, `index 1 -> 1`.
- `0x807946D0` was listed as deferred in the wave-90 header note ("the
  paired-single bodies of the same window ... were not attempted"); the raw
  words show no paired-single maths at all, only the two `f32` spills.
- `0x8079E310`'s open branch is where Ghidra's reconstruction breaks: the
  `psq_l`/`ps_sub`/`psq_st` group at `0x8079_E364..0x8079_E3A0` fills
  `sp+0x44..0x4C` with `node - position`, `0x80170980` normalises it in place,
  and the scaled results plus `position + scaled` are then stored only to
  `sp+0x2C..0x34` and `sp+0x20..0x28`, which nothing reads. The machine code
  stores the raw `+0xAC..+0xB4` copy into `+0x1FC..+0x204` (`stfs f0/f1/f2`
  from the three `lfs` at `0x8079_E3C4..E3D4`). The body keeps the two
  `sp+0x44`-normalised values as a local `delta` so the `0x80100200`/
  `0x80170980` calls stay in place, and stores the raw position. This is the
  only reason the body is rated medium: the dead stores suggest the original
  source computed a projected point that the optimiser dropped.
- `0x8079E310`'s spread arguments are `f1 = 0x8031D39C + 0x1AC`,
  `f2 = (0x8031D39C + 0x1B4) * (0x8031D39C + 0x18)`,
  `f3 = (0x8031D39C + 0x1B8) * (0x8031D39C + 0x18)`, then `r3 = this`,
  `r4 = this + 0x1FC`, `r5 = 0`, `r6 = (s16)[0x8031D39C + 0x1B0]` and
  `r7 = &delta`; Ghidra's parameter list is shifted by one because it models
  `r3` as the first float. `0x8031D39C` and its fields live in the main DOL, so
  they are read at run time and no value is asserted here.
- `0x800FC480` is called as `(out, f1 = scale, r4 = position, r5 = handle)`.
  No existing helper models that order (`call_f32_u32x2` would put the float
  first), so the body declares a local `extern "C"` pointer, the same idiom
  [`fn_80648550`] uses for its 64-bit mask query.
- `0x807B90F0`'s no-argument callees use `platform::call0` and the rest use
  `abi_call1!`/`abi_call2!`/`abi_call3!`; the call order, both guards and the
  two four-iteration font loops are literal. `0x807BA750` takes the word at
  `0x8042A234`, not its address.
- `0x80648AA0`'s height argument to the driver slot is
  `camera[0x4C] - (this[0x1A49C] - this[0x1A4BC])` where the camera is the
  pointer stored at `0x80429E00` (`lwz r5,0xffff9e00(r4)` then `lfs 0x4C(r5)`),
  truncated to `s16` by `extsh`. [`fn_806485c0`] reads the same address as an
  inline struct (`0x8042_9E00 + 0x4C`); this body follows the machine code and
  notes the divergence rather than copying the sibling.
- `0x80648AA0`'s last call passes `f1 = +0xB0`, `f2 = +0x1A4B8`, then the five
  words `[0x8042A078]`, `+0x1A4F7`, `(s8)+0x1A4F6`, `this + 0x20000`,
  `+0x1A490`, `+0x1A494`. Ghidra prints `0x8042A078` as a third double, but the
  code does `lwz r3,0xffffa078(r3)` - it is the low half of the
  `0x8042A078`/`0x8042A07C` mask already used by [`fn_80648550`] and is the
  first integer argument (`call_f32x2_u32x6`).
- `0x80648AA0` has no `silent` parameter: the cue is gated only by
  `+0x1A4F9`. The falling clamp is `fallen <= 0.0` (the sibling uses the same
  `frsp`/`fcmpo` pair).

## Verification

- The lane-D block type-checks standalone against stubbed helper signatures in
  a temporary `rustc --crate-type lib` harness outside the repo; no errors.
- All four evidence exports exist under `work/ghidra/rel-d_basesNP/export/`
  and every branch was re-read from `work/rels/elf/d_basesNP.elf`
  (`0x80648AA0`, `0x807946D0`, `0x8079E310`, `0x807B90F0`).
- Only helpers that already exist in `src/game/bases.rs` are called
  (`platform::read_*`/`write_*`/`call0`, `call0_ret`, `call_f32_u32`,
  `call_f32x2_u32x6`, `call_f32x3_u32x5`, `vmethod0`, `abi_call1!`..
  `abi_call4!`) plus the translated neighbours [`fn_80794480`],
  [`fn_80794bf0`], [`fn_80794c00`] and [`fn_80648770`]; every other callee
  (`0x800F9530`, `0x800FAD60`, `0x800FAE80`, `0x800FADB0`, `0x800FAF70`,
  `0x80168FE0`, `0x801680E0`, `0x800FC810`, `0x800FC480`, `0x80100200`,
  `0x80170980`, `0x80799960`, `0x800FDC80`, `0x800F69F0`, the `0x8016xxxx`
  layer calls, `0x801C4EA0`, `0x801C8E70`, `0x800067B0`/`0x800067D0`,
  `0x8006CAE0`/`0x8006CB40`, `0x80093F10`, `0x807BA750`, `0x800E4550`,
  `0x80648470`) has no Rust body in `bases.rs`.
- `src/game/bases.rs` was not edited; the coordinator appends
  `work/wave177/laneD_block.rs`.
