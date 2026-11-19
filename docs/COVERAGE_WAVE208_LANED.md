# Coverage - Wave 208 lane D (`ui::hud`, main.dol)

> File name note: the shared instructions name this document
> `docs/COVERAGE_WAVE205_LANE<X>.md`, but that path is owned by the wave-205
> lane reports. This lane follows the `COVERAGE_WAVE207_LANEB.md` /
> `COVERAGE_WAVE208_LANEB.md` precedent and lands as
> `docs/COVERAGE_WAVE208_LANED.md` instead of overwriting unrelated work.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1672 | 7 | 3 |

Addresses `0x8003_DFD0`-`0x8003_EC38`: the `Wm_en_quicksand` pose, terrain
probe and effect band. All ten bodies are translated; nothing in this lane is
deferred. The band sits between the scene driver `0x8003_D890..0x8003_DCD0`
(not in this lane) and the per-frame driver `0x8003_EDD0` (not in this lane),
which calls three of the translated bodies.

## Scope

| address | size | name | confidence | role |
|---|---|---:|---|---|
| `0x8003DFD0` | 244 | `fn_8003dfd0` | high | build the `+0x878` pose from the position/angle triple |
| `0x8003E0D0` | 316 | `fn_8003e0d0` | high | build the `+0x7FC` pose with two `+/-6` lifts |
| `0x8003E210` | 16 | `fn_8003e210` | medium | tail-forward slot `+0x14` of the `+0x7FC` node |
| `0x8003E220` | 16 | `fn_8003e220` | medium | tail-forward slot `+0x14` of the `+0x878` node |
| `0x8003E350` | 144 | `fn_8003e350` | high | sinking-sand terrain probe |
| `0x8003E3E0` | 120 | `fn_8003e3e0` | medium | spawn `"Wm_en_quicksand"` through `+0x89C` |
| `0x8003E6E0` | 356 | `fn_8003e6e0` | high | build the 15-word record for `0x800B8700` |
| `0x8003E850` | 172 | `fn_8003e850` | high | advance the `+0x88C` angle accumulator into `+0x100` |
| `0x8003E900` | 56 | `fn_8003e900` | high | flip the facing byte, angle and two scales |
| `0x8003EB50` | 232 | `fn_8003eb50` | high | landing-smoke probe and effect spawn |

Total: 1672 bytes, 10/10 translated, 0 deferred.

## Size checks

`functions.tsv` reports `244/316/16/16/144/120/356/172/56/232` bytes. Every value
was re-checked against the prologue and the terminal branch and matches the real
extent exactly, so no row is a prologue artefact:

| address | terminal branch | real extent |
|---|---|---|
| `0x8003DFD0` | `0x8003E0C0` `bclr` | 244 (`0xF4`) |
| `0x8003E0D0` | `0x8003E208` `bclr` | 316 (`0x13C`) |
| `0x8003E210` | `0x8003E21C` `bcctr` | 16 |
| `0x8003E220` | `0x8003E22C` `bcctr` | 16 |
| `0x8003E350` | `0x8003E3DC` `bclr` | 144 (`0x90`) |
| `0x8003E3E0` | `0x8003E454` `bclr` | 120 (`0x78`) |
| `0x8003E6E0` | `0x8003E840` `bclr` | 356 (`0x164`) |
| `0x8003E850` | `0x8003E8F8` `bclr` | 172 (`0xAC`) |
| `0x8003E900` | `0x8003E934` `bclr` | 56 (`0x38`) |
| `0x8003EB50` | `0x8003EC34` `bclr` | 232 (`0xE8`) |

The two 16-byte rows are genuine frame-less tail jumps, not truncated
prologues: `lwzu r12,0x7FC(r3)` / `lwzu r12,0x878(r3)`, `lwz r12,0x14(r12)`,
`mtspr ctr,r12`, `bcctr`.

## What the cluster is

The class identity comes from the shipped strings, not from RTTI:

- `0x8003E3E0` passes `"Wm_en_quicksand"` (`0x803063C0`) to virtual slot
  `+0xA0` of its `+0x89C` sub-object;
- `0x8003EB50` chooses between `"Wm_en_sndlandsmk_s"` (`0x803063D0`) and
  `"Wm_en_landsmoke_s"` (`0x803063E4`) for `0x8016C890`.

The band is the quicksand/sinking-sand actor's pose and effect layer. It shares
its scene shape with the surrounding `0x8003_xxxx` actors (`+0x60` vtable,
`+0xAC..+0xB4` position, `+0x100..+0x104` angles, `+0x7C` pose matrix,
`+0x464..+0x46C` source vector) and with the bodies already in `src/ui/hud.rs`
(`fn_80038360`, `fn_800275f0`, `fn_80024de0`).

### Object map used here

| offset | type | role |
|---|---:|---|
| `+0x7C` | 12 `f32` | pose matrix written by `0x801C0BD0`/`0x801C0540` |
| `+0xAC..+0xB4` | 3 `f32` | scene position |
| `+0xD0..+0xD8` | 3 `f32` | offset triple added by `0x8006CED0` |
| `+0xE8`, `+0xF4` | `f32` | two signed scales |
| `+0x100..+0x104` | 3 `u16` | rotation triple (Z, Y, X) |
| `+0x1EC` | record | terrain/collision sub-object |
| `+0x348` | `u8` | facing toggle |
| `+0x38F` | `u8` | cell-lookup selector |
| `+0x464..+0x46C` | 3 `f32` | sprite source vector |
| `+0x484` | record | spawn emitter |
| `+0x77C` | `u32` | spawn-record index |
| `+0x7FC`, `+0x878` | node | sprite records with vtable slot `+0x14` |
| `+0x88C` | `i16` | angle accumulator |
| `+0x89C` | object | effect/sound dispatcher (`+0xA0`) |

## Notable decisions

- **Reused helpers, no new call wrappers.** The band calls
  `0x801C0BD0`, `0x801C0540`, `0x80064AF0`, the three single-angle helpers,
  `0x8016A170`/`0x8016A150`/`0x8016A1A0`, `0x8006CED0`, `0x80070BA0`/`0x80070BF0`,
  `0x802DE514`, `0x80070720`, `0x8016C890` and `0x802DCA98`, all of which
  already have module-scope constants. The float helpers `call_3f_1u` and
  `call_2f_1u` already exist in `src/ui/hud.rs` and are reused unchanged.
- **New constants are `_D`-suffixed.** `S_WM_EN_QUICKSAND_D`,
  `S_WM_EN_SNDLANDSMK_D`, `S_WM_EN_LANDSMOKE_D`, `DAT_80353ED8_D`,
  `DAT_802EE834_D`, `G_SDA_5A10_D`, the nine `F32_SDA2_79xx_D` values,
  `TABLE_SDA2_ANGLE_PAIR_D`, `FN_80070CB0_D`, `FN_800B8700_D` and
  `FN_80024C20_D`. The `0x803063xx` string copies are used at their own
  addresses rather than the already-named `0x803023xx`/`0x803049xx` copies.
- **`0x80064AF0` mutates the angle array in place.** Both pose builders read the
  angles back after the normalisation call, exactly as the original does; the
  translation does not cache the pre-call values.
- **`0x8003E850` keeps the original's two-path step.** When `step < |old|` the
  accumulator grows by `0x20` and the *old* accumulator value is added to
  `+0x100`; otherwise the accumulator is cleared and `step` is added. Both
  branches were read from the `cmp`/`bc 4,1` pair, not from the C ordering.
- **`0x8003EB50` takes an `f32`.** Ghidra types the first parameter `double`
  because of its float/double modelling, but the raw code is
  `fadds f4,f0,f1`; the translation keeps `f32` (the neighbouring `fn_80024de0`
  is the same effect pair without the probe).
- **`0x8003E210`/`0x8003E220` return the forwarded `u32`.** Ghidra prints them
  `void` because it models `bcctr` as an indirect call, but the slot's `r3`
  survives the jump; the result is kept and the uncertainty in slot `+0x14`
  is marked `medium`.
- **Nothing is deferred.** All ten bodies have a literal instruction-for-
  instruction counterpart; the unknown parts (slot `+0x14` role, the three
  zero operands of the `+0xA0` dispatch, the emitter's record contract) are
  documented as `unknown`/`inferred:` in the doc blocks rather than guessed.

## Verification

- Scratch build: `src/`, `Cargo.toml` and `Cargo.lock` copied to
  `%TEMP%\opencode\w208d`, `work/wave208/laneD_block.rs` appended to that
  copy's `src/ui/hud.rs`, `cargo check --lib` exits `0`; the appended block
  produces no warning.
- Ledger: `ledger/agents/wave208-laneD.csv` (10 rows, all `translated`).
- This document: `docs/COVERAGE_WAVE208_LANED.md`.
