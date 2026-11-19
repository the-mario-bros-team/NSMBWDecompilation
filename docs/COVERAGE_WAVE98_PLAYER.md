# Coverage - Wave 98 (`game::player`, the `0x80050000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 26 | 5812 | 7 | 19 |

## How to read this file

This document covers the Wave-98 slice of the `0x80050000` bucket of
`work/ghidra/main-gb/export/functions.tsv`: 26 of the 49 untranslated functions
in that bucket. Every body lives in `src/game/player.rs`; the ledger rows are in
`ledger/agents/wave98-player.csv`. The bucket still holds 23 untranslated
functions, listed under "Deliberate skips" below.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - all rows are `translated`.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a callee
  argument had to be inferred.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. Every float in this pass was re-read from
`work/dol/main.elf` rather than copied from prose: `0x8042B9BC` = `24.0f`,
`0x8042B9E0` = `+0.01f`, `0x8042B9E4` = `128.0f`, `0x8042B9F0` = `-0.01f`,
`0x8042B9F4` = `96.0f`, `0x8042B9F8` = `192.0f`, `0x8042BA48` = `-1.0f`,
`0x8042BA4C` = `1.0f`, `0x8042BA50` = `0.6f`. The zero used by the clamp pair is
the wave-87 `W87_ZERO_8042B878` (`0.0f`) and the one added to the right edge is
the wave-3 `ACTOR_ONE` (`1.0f` at `0x8042B874`).

## What this pass covers

- **The `0x800556F0..0x80055EF0` edge/ceiling band (8 bodies)** - one terrain
  probe and the per-frame band logic that keeps the camera-followed actor inside
  the walkable strip. [`fn_800556f0`] lifts the actor point by `128.0f` and asks
  `0x800757B0` for the surface under it, then raises bits `0x08000000`,
  `0x10000000` and `0x20000000` of `+0x10D4` for the body, effect point and
  adjusted body. [`fn_80055ae0`] computes the two bounds as
  `0x80082240(x, scene) + 0x3EC` and
  `1.0f + (that + scene+0x40 - 0x3EC)`, keeping a per-side engaged byte
  (`+0x140D` left, `+0x140C` right) and an accumulated overshoot
  (`+0x1414`/`+0x1410`). [`fn_80055a00`] / [`fn_80055a70`] are the two clamps the
  band calls: they set bit `0x80000` / `0x100000`, install the target through
  `0x80055900`, and clamp the sign of `+0x10C` to `-0.01f` / `+0.01f` (or `0.0f`
  when the facing byte `+0x348` says the other way). [`fn_80055d00`] latches the
  same two flags without pushing, [`fn_80055db0`] drops the actor when the scene
  depth `+0x3C - +0x44 - 24.0f` is too small, and [`fn_80055ef0`] clamps `+0xB0`
  under the 96 or 192 ceiling and refreshes the rest of the band.
- **The ending celebration, `0x8005C2B0..0x8005C6D0` (3 bodies)** - the firework
  volley and its driver. [`fn_8005c2b0`] reads the ring table at `0x8030962C`,
  spawns `Wm_ob_fireworks_1up` (cue `0x251`) on ring 0 and one of
  `Wm_ob_fireworks_{y,b,g,p}` (cue `0x250`, rotated by
  `(byte + 0x800B2EE0(6) + 1) & 3`) afterwards. [`fn_8005c410`] is the four-state
  machine (wait, one firework per 28 frames, notice/bonus check, cue) and
  [`fn_8005c6d0`] averages the positions of the `0x65`-flagged players into
  `+0x30..+0x38`.
- **The `0x8005D100..0x8005DDC0` frame-record compiler (4 bodies)** -
  [`fn_8005d100`] removes one id from the four-word table and pads with `-1`,
  [`fn_8005d880`] copies one whole `0xF0`-byte record out of `0x80354C30`,
  [`fn_8005db00`] seeds a fresh frame object (`-1.0f`, seven `1.0f`/`0.6f`, the
  `0xFF` byte) and [`fn_8005ddc0`] loads one of the three constant sub-records
  from the `0x10`/`0x20`-stride tables at `0x802EF0B8`.
- **Two 4-byte thunks (2 bodies)** - `0x8005E620` and `0x8005E680` tail-jump to
  the already-translated `0x8005E590` / `0x8005E5D0`; the exports name the
  targets and repeat the same `0x400`/`0x200` masks over `+0x06` and `+0x04`.
- **The spawn-slot leaves, `0x8005ED90..0x8005FE30` (9 bodies)** -
  [`fn_8005ed90`] turns a stage-area record into a placement point using the
  `0xC`-stride nudge tables at `0x802EF188`, [`fn_8005ee90`] returns the current
  area record's `+0x0B` type byte, [`fn_8005f4d0`] hands the first free slot to
  `0x8005EEE0`, [`fn_8005f570`] writes the presence bit, handle `1`, character
  flag `8` and the SDA handle word, and [`fn_8005fdb0`] / [`fn_8005fe30`] are the
  reserve/release pair around vtable slot `+0xE0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800556F0` | `fn_800556f0` | 228 | medium |
| `0x80055810` | `fn_80055810` | 236 | medium |
| `0x80055A00` | `fn_80055a00` | 112 | medium |
| `0x80055A70` | `fn_80055a70` | 112 | medium |
| `0x80055AE0` | `fn_80055ae0` | 532 | medium |
| `0x80055D00` | `fn_80055d00` | 164 | medium |
| `0x80055DB0` | `fn_80055db0` | 296 | medium |
| `0x80055EF0` | `fn_80055ef0` | 256 | medium |
| `0x80058280` | `fn_80058280` | 32 | high |
| `0x800582A0` | `fn_800582a0` | 68 | medium |
| `0x800582F0` | `fn_800582f0` | 168 | medium |
| `0x8005C2B0` | `fn_8005c2b0` | 348 | high |
| `0x8005C410` | `fn_8005c410` | 692 | medium |
| `0x8005C6D0` | `fn_8005c6d0` | 344 | medium |
| `0x8005D100` | `fn_8005d100` | 380 | high |
| `0x8005D880` | `fn_8005d880` | 628 | medium |
| `0x8005DB00` | `fn_8005db00` | 220 | medium |
| `0x8005DDC0` | `fn_8005ddc0` | 212 | medium |
| `0x8005E620` | `fn_8005e620` | 4 | high |
| `0x8005E680` | `fn_8005e680` | 4 | high |
| `0x8005ED90` | `fn_8005ed90` | 220 | medium |
| `0x8005EE90` | `fn_8005ee90` | 80 | medium |
| `0x8005F4D0` | `fn_8005f4d0` | 156 | high |
| `0x8005F570` | `fn_8005f570` | 80 | high |
| `0x8005FDB0` | `fn_8005fdb0` | 116 | medium |
| `0x8005FE30` | `fn_8005fe30` | 124 | medium |

Byte total: 5812. Address range covered: `0x800556F0..0x8005FE30`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket now holds 23 untranslated functions. Two groups:

**Checked and left alone - the C form is not the real logic.**

* **`0x80055900` (252 B)** - the "install a target heading" callee of both
  clamps. Its whole trigonometric body is expressed through
  `unaff_GQR0`/`unaff_GQR3` `ldexpf` guards plus two `FUN_80237BD0` /
  `FUN_80237C40` calls, so the 16-bit angle conversion cannot be recovered from
  the C: the scale factor lives in a GQR register the decompiler never binds.
  The two clamps call it by address with a `// unknown:` note.
* **`0x80059260` and `0x80059290` (48 B each)** - both decompile to
  `FUN_802DCBFC(param_2, param_2, param_1)` with no readable prologue. That is
  the same outlined-register-save artefact wave 97 recorded for
  `0x800408F0..0x8004DB00`; writing a body would invent the prototype.
* **`0x80058B20` (236 B) and `0x80058EE0` (332 B)** - the pair that starts with
  `FUN_802DCD74` and then uses the **high** half of the returned `u64` as the
  object and the **low** half as a value compared against `record+0x9C`. Choosing
  which half is the receiver would produce a wrong signature for code that
  inserts into a live slot array, so both stay for a pass that can read the two
  instructions at the call site.
* **`0x8005C830` (256 B)** - the digit check divides by ten twice through
  Ghidra's signed-division rewrite (`iVar4 - (iVar4 >> 31)` after
  `iVar5 / 10 + (iVar5 >> 31)`); the emitted C is not the source expression, so
  the tens/units extraction would be a guess.

**Not attempted - budget.** The remaining 17 are bodies that were not read
past their headers in this pass and are therefore **not** claimed to be
unreconstructable: `0x800544F0` (2540 B), `0x80054F80` (820 B), `0x80055330`
(512 B), `0x80055FF0` (880 B), `0x80056620` (720 B), `0x800569C0` (496 B),
`0x80056E40` (888 B), `0x800571C0` (552 B), `0x800573F0` (600 B),
`0x800583A0` (1200 B), `0x800592C0` (7620 B), `0x8005B8A0` (1100 B),
`0x8005BFD0` (192 B), `0x8005D280` (1064 B), `0x8005E040` (624 B),
`0x8005EF50` (1408 B) and `0x8005F5C0` (696 B). One of them, `0x8005E040`, is
close to done: its two interleaved 20-word history windows
(`+0x0D..+0x12` and `+0x34..+0x3A`, indexed by `*param_1 * 10`) need a second
read to confirm the source order before it can be written honestly.

## Verification

- Every address and byte size matches `functions.tsv` (checked row by row, and
  the sizes are the ones printed in each `# Original` block).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/game/player.rs`.
- No address was already in `ledger/functions.csv` (all 26 were confirmed
  untranslated before a body was written).
- `cargo check --message-format=short` is free of errors and of warnings for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave98` reports `OK` (26 rows in this lane, 26
  unique addresses, no overlap with the merged ledger).
