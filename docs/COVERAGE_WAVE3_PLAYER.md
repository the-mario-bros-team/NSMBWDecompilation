# Wave 3 — player module coverage

## How to read this file

This is the lane report for the second player pass. It lists the 24 functions
added to `src/game/player.rs` in wave 3: 17 gameplay-actor functions from
`main.dol` and 7 character-select lifecycle functions from REL `d_basesNP`.
Every row names the Ghidra evidence file, the Rust symbol, and a confidence
level. "Translated" means the recorded control flow is represented; names and
field roles marked as inferred are still unproven.

REL rows use the canonical synthetic base **`0x80600000`** assigned by
`tools/rel2elf.mjs`. That is a static-analysis address, not the address the Wii
REL loader chooses at runtime.

## Summary

| Source | Range | Functions | Translated | Partial | Stub |
|---|---|---:|---:|---:|---:|
| `main.dol` | `0x8004B290..0x8004D5A0` | 17 | 17 | 0 | 0 |
| `d_basesNP` (REL) | `0x80601BE0..0x80602FF0` | 7 | 7 | 0 | 0 |
| **Total** | | **24** | **24** | **0** | **0** |

Confidence: **14 high**, **10 medium**, **0 low**.

## Gameplay player — motion, animation rate and effects

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8004B290` | `fn_8004b290` | fast-animation predicate plus vtable `+0x398` | translated | medium | `8004b290_FUN_8004b290.c` |
| `0x8004B2F0` | `fn_8004b2f0` | gated variant of the fast-animation predicate | translated | high | `8004b2f0_FUN_8004b2f0.c` |
| `0x8004B380` | `fn_8004b380` | movement-state / animation composite predicate | translated | high | `8004b380_FUN_8004b380.c` |
| `0x8004B460` | `fn_8004b460` | signed animation rate (`±4.0`) | translated | high | `8004b460_FUN_8004b460.c` |
| `0x8004B4C0` | `fn_8004b4c0` | animation-rate table `0x802EE978` | translated | high | `8004b4c0_FUN_8004b4c0.c` |
| `0x8004B580` | `fn_8004b580` | animation-rate table `0x802EE9F0` | translated | high | `8004b580_FUN_8004b580.c` |
| `0x8004B5E0` | `fn_8004b5e0` | animation-rate table `0x802EEA2C` | translated | high | `8004b5e0_FUN_8004b5e0.c` |
| `0x8004B640` | `fn_8004b640` | clamp horizontal speed from the movement state | translated | high | `8004b640_FUN_8004b640.c` |
| `0x8004B9A0` | `fn_8004b9a0` | apply the movement scale to vertical speed | translated | medium | `8004b9a0_FUN_8004b9a0.c` |
| `0x8004BA50` | `fn_8004ba50` | choose the horizontal-speed branch | translated | medium | `8004ba50_FUN_8004ba50.c` |
| `0x8004C4F0` | `fn_8004c4f0` | choose the landing-speed branch | translated | high | `8004c4f0_FUN_8004c4f0.c` |
| `0x8004CD80` | `fn_8004cd80` | landing effect dispatch | translated | medium | `8004cd80_FUN_8004cd80.c` |
| `0x8004CE00` | `fn_8004ce00` | alternate landing effect dispatch | translated | medium | `8004ce00_FUN_8004ce00.c` |
| `0x8004CE70` | `fn_8004ce70` | foot-water effect | translated | medium | `8004ce70_FUN_8004ce70.c` |
| `0x8004CFD0` | `fn_8004cfd0` | brake-smoke effect | translated | medium | `8004cfd0_FUN_8004cfd0.c` |
| `0x8004D210` | `fn_8004d210` | one-shot interface teardown | translated | medium | `8004d210_FUN_8004d210.c` |
| `0x8004D5A0` | `fn_8004d5a0` | preview-id lookup | translated | high | `8004d5a0_FUN_8004d5a0.c` |

Evidence root: `work/ghidra/main-gb/export/`.

## Character-change select lifecycle (REL `d_basesNP`)

Synthetic base `0x80600000`; evidence root
`work/ghidra/rel-d_basesNP/export/`.

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80601BE0` | `fn_80601be0` | delete-with-flag helper | translated | high | `80601be0_FUN_80601be0.c` |
| `0x80601CD0` | `fn_80601cd0` | array-element delete helper | translated | high | `80601cd0_FUN_80601cd0.c` |
| `0x80601EF0` | `fn_80601ef0` | delete helper with a member at `+0x18` | translated | high | `80601ef0_FUN_80601ef0.c` |
| `0x80602C20` | `fn_80602c20` | apply the selected character preview | translated | medium | `80602c20_FUN_80602c20.c` |
| `0x80602EE0` | `fn_80602ee0` | final slot confirmation | translated | medium | `80602ee0_FUN_80602ee0.c` |
| `0x80602FB0` | `fn_80602fb0` | start the confirm animation | translated | high | `80602fb0_FUN_80602fb0.c` |
| `0x80602FF0` | `fn_80602ff0` | notify when the layout animation finished | translated | high | `80602ff0_FUN_80602ff0.c` |

## Shared evidence and constants

The wave-3 functions reuse the actor offsets already documented in
`src/game/player.rs` (`+0x10C` speed, `+0x10D4`/`+0x10D8` flags, `+0x1EC`
animation controller, `+0xEA4` movement state, `+0x38D` owner index, `+0xAC`
position). The `main.elf` data sections were read directly to verify:

| address | value | role |
|---|---|---|
| `0x8042B840` | `1.0, -1.0, 3.628, 3.5, ...` | 16-entry animation direction/scale table |
| `0x8042B8AC` | `4.0` | animation-rate multiplier |
| `0x8042B8A4` | `10.0` | foot-water Y offset |
| `0x8042B8C4` | `0.5` | half-speed multiplier |
| `0x802EEACC` | `0.5, 0.8, 1.0, ...` | nine-entry foot-water scale table |
| `0x802EEAF0` | `0, 1, 2, 3, ...` | preview-id int table |

## Unknowns and inferred pieces

* `0x8004B290`, `0x8004B2F0`: Ghidra prints the `+0x398` vtable call without
  arguments; the translation passes `player`, which is still in `r3` at the
  call site.
* `0x8004CD80`: Ghidra prints `0x8004D510()` without an argument; the
  translation passes `player`. The callee's own C declaration takes one
  argument.
* `0x8004CE00`, `0x8004CE70`, `0x8004D210`: argument-less virtual calls are
  given the object address (actor or interface) as their `this` argument. The
  original `r3` value is not printed by Ghidra.
* `0x8004CE70`, `0x8004CFD0`: the `0x8008FBA0` / `0x8008FC80` owner argument and
  the effect-name pointer table are observed; their semantic roles are
  inferred from the shared effect-spawner shape.
* `0x8004B9A0`, `0x8004BA50`: the two model floats (`+0x00`/`+0x08`) and the
  transform word at `+0x1C` are inferred to be scale/speed inputs.
* `0x80602C20`: the block at `+0x2A4` and the actor field at `+0x25C` are
  inferred color/transform fields; the copy direction is observed.
* `0x80602EE0`: vtable slot `+0x20` on the player object and sound id `0x79`
  are observed; their semantic roles are inferred.
* `0x80601BE0`, `0x80601CD0`, `0x80601EF0`: `0x8015F800` / `0x8015F9F0` and
  `0x802B9280` are inferred to be base-cleanup and free helpers from their
  bodies in `main.dol` (`8015f800_FUN_8015f800.c`, `802b9280_FUN_802b9280.c`).
* `0x8004B4C0` / `0x8004B580`: Ghidra drops the `direction` argument of
  `0x80070820`; the second parameter is preserved, following
  `0x8004B520` and caller `0x8004BAC0`.
* All table indices are unchecked in the original; the Rust preserves that and
  documents the range at each constant.

## Gates

* `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
* `cargo check` — exit `0` (no errors, no warnings for `src/game/player.rs`).
