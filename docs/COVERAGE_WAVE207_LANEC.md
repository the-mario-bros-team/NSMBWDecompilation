# Coverage - Wave 207, lane C (`ui::hud`, `0x80037220..0x80037D18`)

## How to read this file

This page records the Wave 207 lane C pass over `src/ui/hud.rs`: ten bodies of
`main.dol` in the run `0x80037220..0x80037D18` (2,516 bytes by the verified
extents). The block is staged in `work/wave207/laneC_block.rs` for the merge
pass; it is appended verbatim at the end of `src/ui/hud.rs`. One row per body
is mirrored in `ledger/agents/wave207-laneC.csv`.

Every address is a **runtime address** of `sys/main.dol`, the same address the
Ghidra exports in `work/ghidra/main-gb/export/` use, and each function doc
carries `# Original` (Ghidra name, address, byte count, evidence path) and
`# Confidence` plus an argument/return/side-effect/safety block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 2,516 | 10 | 0 |

All ten rows are `translated`; none is `partial` or `stub`. Covered range:
`0x80037220..0x80037D18`.

Raw words were decoded with `node $env:TEMP\opencode\ppc159.js work/dol/main.elf
<addr>`, and each body was followed from its entry to its terminal
`bclr`. Verified extents: `0x84`, `0x84`, `0x7C`, `0x7C`, `0x7C`, `0x7C`,
`0x7C`, `0x48`, `0x50`, `0x5C8` (132, 132, 124, 124, 124, 124, 124, 72, 80,
1,480 bytes). All ten equal their `functions.tsv` size; this run has no
prologue artefact and no alignment padding.

## What this pass covers

The run is the state machinery of `daEnObjCoinBlock_c` - the actor whose seven
state descriptors the last body builds. The strings at `0x803057D8`,
`0x80305800`, `0x8030582C`, `0x8030585C`, `0x80305888`, `0x803058BC` and
`0x803058E4` name the states in image order: `StateID_SerchRailID`,
`StateID_SerchObjPolID`, `StateID_SerchWaterMoveID`,
`StateID_SerchSpinLiftID`, `StateID_SerchSpinLiftChildID`, `StateID_CoinWait`
and `StateID_BlockWait`.

- **`0x80037220`, `0x800372B0` - record resolvers (132 bytes each).** Both walk
  `0x80162D50` (`0x1F7` / `0x1F8`), match `+0x71C` against the record word at
  `+0x6F8` / `+0x6FC`, then copy the record handle (`lwz`/`stw`) into
  `+0x6BC` and the three `lfs` floats `+0x62C..+0x634` into
  `+0x700..+0x708`. They return `1` on a hit and `0` when the list ends.

- **`0x80037360`, `0x80037400`, `0x800374A0` - probe/dispatch states (124
  bytes each).** Each calls a profile probe with `this`
  (`0x80037060` for `0x1E9`, `0x80037110` for `0x1C0`, `0x80037190` for
  `0x25E`) and, on a non-zero answer, dispatches through owner slot `+0xD4`
  with `this` and `0x80353970` (`BlockWait`) when `+0x710 == 1`, else
  `0x80353930` (`CoinWait`).

- **`0x80037540`, `0x800375E0` - the same shape over the two resolvers (124
  bytes each).** `0x80037540` reuses [`fn_80037220`] and `0x800375E0` reuses
  [`fn_800372b0`]; both were translated in this lane, so the bodies call them
  directly instead of by address. `r3` still holds `this` at the `bl`, which
  is why Ghidra prints both callees without operands.

- **`0x80037690` - body-step gate (72 bytes).** `0x80036A70(this)` selects
  `0x800369D0(this)` when non-zero and `0x80036E20(this)` when zero.

- **`0x80037700` - step, teardown, validator (80 bytes).**
  `0x80036AB0(this)`, `0x80036E20(this)`, then in coin mode (`+0x710 == 0`)
  `0x8007F900(this + 0x534)`, then [`fn_80021780`] (already translated in this
  module) whose result is discarded.

- **`0x80037750` - state-table initialiser (1,480 bytes).** Seven mechanically
  identical blocks build the descriptors at `0x803537F0`, `0x80353830`,
  `0x80353870`, `0x803538B0`, `0x803538F0`, `0x80353930` and `0x80353970`
  (stride `0x40`) from parameter records at `0x803056D8 + 0x24 * i`
  (stride `0x24`). Each block: nine `lwz` loads staged on the stack,
  `0x8015F7C0(desc, name)`, vtable `0x8030590C` into `+0x00`, nine `stw` into
  `+0x0C..+0x2C`, then `0x802DC780(desc, 0x80037D20, desc - 0x10)`. The copy
  order is the same one the module documents for [`fn_80016a80`]:
  params `+0x18..+0x20` to `+0x0C..+0x14`, params `+0x0C..+0x14` to
  `+0x18..+0x20`, params `+0x00..+0x08` to `+0x24..+0x2C`. The tail then writes
  two four-float vectors: `(a, b, b, c)` at `0x803539A0` and `(a, d, b, e)` at
  `0x803539BC`, where `a..e` are the `lfs` values from `SDA2 - 0x7AF0`,
  `-0x7AF8`, `-0x7AE4`, `-0x7AEC` and `-0x7AE8`.

## Notes and limitations

- **New constants are `_C`-suffixed.** `FN_800369D0_C`, `FN_80036A70_C`,
  `FN_80036AB0_C`, `FN_80036E20_C`, `FN_80037060_C`, `FN_80037110_C`,
  `FN_80037190_C`, `FN_80037D20_C`, `FN_8007F900_C`, `VT_COINBLOCK_STATE_C`,
  `DAT_803056D8_C`, the seven `STATE_COINBLOCK_*_C` descriptors, the seven
  `S_STATEID_*_C` names, `DAT_803539A0_C`/`DAT_803539BC_C` and the five
  `F32_SDA2_7Axx_C` anchors. `FN_80162D50`, `FN_8015F7C0`, `FN_802DC780` and
  `fn_80021780` already exist in the module and were reused unchanged.

- **One host-side helper.** `w207c_build_state(desc, params, name)` collapses
  the seven identical 9-word copies. It keeps the shipped order (all nine
  loads before the name call, all nine stores after it) and derives the
  registration event as `desc - 0x10`, which holds for every block
  (`0x803537E0`/`0x80353820`/.../`0x80353960`).

- **Ghidra's `lfs`-as-`undefined4` rendering.** Both the resolvers and the
  initialiser tail are single-precision in the raw stream (`lfs`/`stfs`), but
  the export prints integer copies; the reconstruction uses `f32` throughout,
  matching the existing convention in this module.

- **`unaff_r2` in `FUN_80037750`.** Ghidra lost track of `r2` in the tail and
  printed `*(undefined4 *)(unaff_r2 - 0x7AF0)`. The raw words are
  `lfs r4,-0x7af0(r2)` etc. against the SDA2 base `0x80433080`, so the five
  anchors are expressed as `SDA2_BASE - offset`.

- **Compiler save/restore helpers.** `0x802DCD74` / `0x802DCDC0` in the
  initialiser are the register save/restore helpers for the `0x140`-byte
  frame; they have no observable effect and are omitted, exactly as in
  [`fn_80016a80`].

- **Unknowns.** The roles of the two float vectors in `0x803539A0` /
  `0x803539BC`, and the semantic difference between the `0x1F7`/`0x1F8`
  record kinds (beyond their match words), are not established by these
  bodies. The `+0x710` mode word is only inferred to mean block form (`1`)
  versus coin form (`0`) from the `BlockWait`/`CoinWait` selection and the
  `+0x534` teardown, which happens in coin mode. The seven state methods
  above `0x80037750` are not mapped one-to-one onto the seven state names;
  the name order is documented, not the dispatch order.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory and
the block appended to the copy's `src/ui/hud.rs`; `cargo check --lib` exits `0`
with no warning or error that mentions `src/ui/hud.rs` (the 23 pre-existing
warnings all come from other modules).

`node tools/check-lane-csv.mjs wave207` reports only the expected pre-merge
"no body in `src/ui/hud.rs`" rows; the lane C header, addresses, evidence
paths and byte sizes all pass.
