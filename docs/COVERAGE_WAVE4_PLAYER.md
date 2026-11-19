# Wave 4 — player module coverage

## How to read this file

This is the lane report for the fourth player pass. It lists the 23 functions
added to `src/game/player.rs`: 14 movement/animation/effect functions from
`main.dol` (`0x8004BAC0..0x8004D510`) and 9 shared actor-field accessors
(`0x80056C70..0x80057AD0`). Every row gives the original address, the Ghidra
name, a one-sentence role, the status, the confidence, and the evidence file.
"Translated" means the recorded control flow is represented in Rust; roles and
field names marked as inferred are still unproven. The source of truth is the
Ghidra C under `work/ghidra/main-gb/export/`; where an argument was missing from
the C, the instruction stream in `work/dol/main.elf` was read as well.

Addresses are DOL virtual addresses (image base `0x80004000`), not file
offsets. `r2` is the SDA2 base `0x80433080` and `r13` the SDA base
`0x8042F680`; every `r2 - k` / `r13 - k` constant below was resolved with those
bases.

## Summary

| Source | Range | Functions | Translated | Partial | Stub |
|---|---|---:|---:|---:|---:|
| `main.dol` | `0x8004BAC0..0x8004D510` | 14 | 14 | 0 | 0 |
| `main.dol` | `0x80056C70..0x80057AD0` | 9 | 9 | 0 | 0 |
| **Total** | `0x8004BAC0..0x80057AD0` | **23** | **23** | **0** | **0** |

Confidence: **13 high**, **10 medium**, **0 low**.

## Movement, animation and landing effects (`0x8004BAC0..0x8004D510`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8004BAC0` | `fn_8004bac0` | horizontal-speed dispatcher: fast-animation gate, direction sign, movement entry, model `+0x18` fallback | translated | medium | `8004bac0_FUN_8004bac0.c` |
| `0x8004C0D0` | `fn_8004c0d0` | copies one of six four-float pose blocks selected by vtable `+0x3D4` and the pose state | translated | high | `8004c0d0_FUN_8004c0d0.c` |
| `0x8004C1E0` | `fn_8004c1e0` | scales the animation rate by `0.375`, `0.25` or `0.004` from the pose state and speed | translated | high | `8004c1e0_FUN_8004c1e0.c` |
| `0x8004C480` | `fn_8004c480` | chooses `0x8004C4F0` or the model's first float when event flag `0x98` is clear | translated | high | `8004c480_FUN_8004c480.c` |
| `0x8004C560` | `fn_8004c560` | landing-speed table with five thresholds at `+0x04+i*4` and speeds at `+0x30+i*4` | translated | high | `8004c560_FUN_8004c560.c` |
| `0x8004C620` | `fn_8004c620` | alternate landing-speed table (`+0x2C` default, `+0x18+i*4` speeds) with `0x8005E460` mode `4` | translated | high | `8004c620_FUN_8004c620.c` |
| `0x8004C700` | `fn_8004c700` | two-stage signed speed-ramp transfer curve (`0.1`, `0.9`, `1.11111`) | translated | high | `8004c700_FUN_8004c700.c` |
| `0x8004CB30` | `fn_8004cb30` | landing-smoke dispatcher by state; state 7 delegates to `0x8004CD20` | translated | medium | `8004cb30_FUN_8004cb30.c` |
| `0x8004CC10` | `fn_8004cc10` | `Wm_mr_cmnlandsmoke` / `Wm_mr_cmnsndlandsmk` with terrain scale from `0x80370DA8` | translated | medium | `8004cc10_FUN_8004cc10.c` |
| `0x8004CF20` | `fn_8004cf20` | slip-smoke effect; state 5 delegates to `0x8004CE70` | translated | medium | `8004cf20_FUN_8004cf20.c` |
| `0x8004D040` | `fn_8004d040` | turn sound/effect and lazy 13-byte turn-sound table at `0x803546F8` | translated | medium | `8004d040_FUN_8004d040.c` |
| `0x8004D260` | `fn_8004d260` | terrain foot-effect spawner for states 1–3, 5, 7 and 9 | translated | medium | `8004d260_FUN_8004d260.c` |
| `0x8004D350` | `fn_8004d350` | `Wm_mr_sandsplash`, `Wm_mr_quicksand` and `Wm_mr_sanddive*` effects | translated | medium | `8004d350_FUN_8004d350.c` |
| `0x8004D510` | `fn_8004d510` | spawns `Wm_mr_sanddive_s` when the actor is below the sand surface | translated | high | `8004d510_FUN_8004d510.c` |

## Shared actor accessors (`0x80056C70..0x80057AD0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80056C70` | `fn_80056c70` | sets bit `flag` of the event-flag array at `+0x1014` | translated | high | `80056c70_FUN_80056c70.c` |
| `0x80056CB0` | `fn_80056cb0` | clears bit `flag` of the event-flag array at `+0x1014` | translated | high | `80056cb0_FUN_80056cb0.c` |
| `0x80056CF0` | `fn_80056cf0` | tests bit `flag` of the event-flag array at `+0x1014` | translated | high | `80056cf0_FUN_80056cf0.c` |
| `0x80057650` | `fn_80057650` | returns `+0x1060` or `+0x1064` according to vtable `+0x3D4` | translated | high | `80057650_FUN_80057650.c` |
| `0x800577D0` | `fn_800577d0` | sets event flag `0x99` and stores the actor Z position (`+0xB4`) | translated | high | `800577d0_FUN_800577d0.c` |
| `0x80057820` | `fn_80057820` | clamps a value to `[-2.0, 2.0]` and stores it at `+0x14C4` unless flag bit 0 is set | translated | high | `80057820_FUN_80057820.c` |
| `0x800578D0` | `fn_800578d0` | slip predicate: movement state, `abs(speed) > 1.3`, `+0x10E4`/`+0x10E8`, `0x800579C0` | translated | medium | `800578d0_FUN_800578d0.c` |
| `0x800579C0` | `fn_800579c0` | shared slip/skid starter; special case `0x80047B70`, otherwise vtable `+0x294` | translated | medium | `800579c0_FUN_800579c0.c` |
| `0x80057AD0` | `fn_80057ad0` | slip availability: flag bit 0, clear `+0x10D8` bit 8, `0x800579C0` | translated | medium | `80057ad0_FUN_80057ad0.c` |

Evidence root: `work/ghidra/main-gb/export/`.

## Argument and constant checks against `main.elf`

The Ghidra C drops some arguments and types. These were resolved by reading the
instruction stream:

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x8004BAC0` | `FUN_8004B580(param_1)` | `r4` is the direction byte `(speed < 0.0)` before the call; the byte is also passed to `0x8004B520`. |
| `0x8004C0D0` | vtable `+0x3D4` with no argument | `r3` is still `player` at the indirect call; the result is tested for non-zero. |
| `0x8004C560` / `0x8004C620` | vtable `+0x3C4` with no argument | every indirect call sets `r3 = player` first; the returned pointer is `base`. |
| `0x8004CB30` | `FUN_8004CD20()` | `r3` is still `player`; the sibling translation passes `player`. |
| `0x8004CC10` | `FUN_800CD2C0()` | `r3` is still `player` at the call. |
| `0x8004D040` | interface `+0xB0` with `(iface, stack, 0, 0)` | `r3 = player + 0x494`, `r4 = stack`, `r5 = r6 = 0`. |
| `0x800577D0` / `0x80057820` | `(double value, int player)` | the object is in `r3` and the float in `f1`; the real order is `(player, value)`. |
| `0x800579C0` | vtable `+0x294(player, 0x8035429C, block)` | the four-word block is `{0x803089B0, 0.0f, param_3, param_2}`. |

Constants verified in `main.elf` (`r2 = 0x80433080`):

| address | value | role |
|---|---|---|
| `0x8042B904` | `0.05` | [`fn_8004bac0`] rate when flag `0x31` is clear |
| `0x8042B908` | `0.09` | rate when flag `0x31` is set |
| `0x8042B8A0` | `3.0` | rate boost while flag `0x89` is set |
| `0x8042B914` | `0.375` | main damping factor |
| `0x8042B918` | `0.25` | low-speed damping factor |
| `0x8042B910` | `0.004` | near-stop rate |
| `0x8042B8D0` | `0.1` | ramp knee |
| `0x8042B91C` | `0.9` | ramp span |
| `0x8042B920` | `1.11111` | ramp output scale |
| `0x8042BA10` | `1.3` | slip minimum speed |
| `0x8042B92C` / `0x8042B930` | `0.6` / `0.8` | landing-smoke scales (water / ground) |
| `0x8042B934` | `3700.0` | special landing-smoke Z |
| `0x8042B87C` | `16.0` | sand-dive vertical window |
| `0x8042B938` / `0x8042B93C` | `8.0` / `58.0` | sand surface offsets |
| `0x8042B8E0` / `0x8042B8D8` | `-4.0` / `-2.0` | sand threshold / float clamp |
| `0x8042B968` | `2.0` | upper float clamp |

Tables read by these functions:

| address | contents | role |
|---|---|---|
| `0x802EEAC0` | `1.1, 0.9, 1.0` | animation-rate column scale |
| `0x802EEAD8` / `0x802EEAE4` | `0.5, 0.8, 1.0` | effect scale by vtable `+0xE8` |
| `0x802F55B8` | six 16-byte pose blocks | [`fn_8004c0d0`] |
| `0x80307D8C` | `Wm_mr_landsmoke*`, `Wm_mr_watersplash` | [`fn_8004cb30`] |
| `0x80307F18` | `Wm_mr_slipsmoke*` | [`fn_8004cf20`] |
| `0x80308188` | `Wm_mr_turn_*` | [`fn_8004d040`] |
| `0x80308248` | `Wm_mr_foot_*` | [`fn_8004d260`] |
| `0x803082B4` | `Wm_mr_sanddive`, `..._m`, `..._s` | [`fn_8004d350`] |
| `0x80370DA8` | runtime `f32` scale table (BSS) | [`fn_8004cc10`] |
| `0x803546F8` | 13 runtime sound ids (BSS) | [`fn_8004d040`] |

## Unknowns and inferred pieces

* `// unknown:` `0x8004BC80` (the `param_2 == 0` branch of [`fn_8004bac0`]) is
  still untranslated. Its call is preserved at the same point.
* `// unknown:` `0x80370DA8` is in BSS, so its values are runtime data; only
  its use as a one-word scale (replicated three times) is inferred.
* `// unknown:` the exact length and layout of the `0x800D5790` result; the
  translation reserves 16–24 bytes of aligned stack per call, matching the
  Ghidra stack slots.
* `// inferred:` state `5` in [`fn_8004d040`] / [`fn_8004d260`] and the
  `Wm_mr_foot_*` grouping come from the strings and shared field use, not from
  a recovered enum.
* `// inferred:` [`fn_800578d0`] / [`fn_80057ad0`] are described as slip
  predicates because they gate the effect path behind `0x800579C0`; the
  original names are not present in the image.
* All table indices are unchecked in the original; the Rust preserves that and
  documents the range at each constant.

## Gates

* `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
* `cargo check` — exit `0` (no errors, no warnings).
