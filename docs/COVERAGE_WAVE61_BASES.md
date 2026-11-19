# Coverage - Wave 61 (`game::bases`, `0x8076` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address here is a static analysis
address, not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the cited evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, call arguments and constants are literal, and
`medium` when a callee identity, a field role or a lost argument had to be
inferred (`// inferred:` / `// unknown:` in the body).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 5208 | 13 | 16 |

## What this pass covers

- The killer-launcher throw tail at `0x80769620..0x80769FF0`. It selects the
  projectile slot from the variant byte ([`fn_80769620`]), resets the ten-slot
  array and advances the armed slot ([`fn_80769640`]), gates arming on a
  six-float zone record ([`fn_807698a0`]) and a projectile distance check
  ([`fn_80769e90`]), and drives the four-state throw meter
  ([`fn_807699d0`]) with its 16-bit accumulator ([`fn_80769c70`]). The meter is
  read in `0x807699D0` from the constant block `0x80816042..0x8081604C`, the
  arming record lives at `0x80832720..0x8083273C`, and the button handler is
  [`fn_80769cb0`] (buttons `0x56`, `0x59`, `0x9A`). The pickup effect spawn
  ([`fn_80769f00`]) and charge abort ([`fn_80769ff0`]) close the cluster.
- The `cobKinopio` Toad carrier at `0x8076A270..0x8076B4A0`, identified by the
  `cobKinopio` / `g3d/model.brres` / `PTR_s_cobKinopio` strings at
  `0x8081627C..0x80816290`: constructor [`fn_8076a270`] (vtable `0x808162A0`),
  start-up [`fn_8076a3a0`], and the `cobKinopio.brres` model/clip/material
  loader [`fn_8076a560`]. The object owns an animation binding at `+0x1A8`, a
  model at `+0x1A4` and a scale/clip sub-object at `+0x1B4`; the transform
  writer [`fn_8076a6a0`] binds them through `0x801C0BD0` / `0x8016EF50`.
- The variant and respawn helpers: [`fn_8076a750`] selects the variant colour
  and scale from `(+0x04 >> 16) & 0xFF`, [`fn_8076a890`] clears the pickup flag
  `+0x1F0`, and [`fn_8076a8b0`] places the actor at the current player's spawn
  point (or clears it for character byte `0x2A`) using the `0x80429F7C` player
  table. [`fn_8076aa40`] plays the release sound and enters state `2`;
  [`fn_8076ae20`] restores the player's character byte.
- The type-walk helpers: [`fn_8076ae80`] finds the first actor of type `0x2A7`
  with a given kind, [`fn_8076aee0`] releases the first three kinds, and
  [`fn_8076af60`] repositions the first actor of the second trio.
- The charge-effect pair [`fn_8076afe0`] / [`fn_8076b040`], the all-carrier cue
  reset [`fn_8076b0a0`], and the sibling constructor/destructor trio
  [`fn_8076b210`] / [`fn_8076b3f0`] / [`fn_8076b4a0`] (vtables `0x80816418`
  and `0x80816538`, interface vtable `0x807FEF40` at `+0x258`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80769620` | `fn_80769620` | 28 | high |
| `0x80769640` | `fn_80769640` | 136 | medium |
| `0x807696D0` | `fn_807696d0` | 464 | medium |
| `0x807698A0` | `fn_807698a0` | 304 | medium |
| `0x807699D0` | `fn_807699d0` | 668 | medium |
| `0x80769C70` | `fn_80769c70` | 64 | high |
| `0x80769CB0` | `fn_80769cb0` | 468 | medium |
| `0x80769E90` | `fn_80769e90` | 104 | medium |
| `0x80769F00` | `fn_80769f00` | 232 | medium |
| `0x80769FF0` | `fn_80769ff0` | 156 | medium |
| `0x8076A270` | `fn_8076a270` | 124 | high |
| `0x8076A3A0` | `fn_8076a3a0` | 136 | high |
| `0x8076A530` | `fn_8076a530` | 48 | high |
| `0x8076A560` | `fn_8076a560` | 308 | medium |
| `0x8076A6A0` | `fn_8076a6a0` | 176 | high |
| `0x8076A750` | `fn_8076a750` | 308 | medium |
| `0x8076A890` | `fn_8076a890` | 12 | high |
| `0x8076A8B0` | `fn_8076a8b0` | 220 | medium |
| `0x8076AA40` | `fn_8076aa40` | 104 | high |
| `0x8076AE20` | `fn_8076ae20` | 88 | medium |
| `0x8076AE80` | `fn_8076ae80` | 92 | medium |
| `0x8076AEE0` | `fn_8076aee0` | 116 | medium |
| `0x8076AF60` | `fn_8076af60` | 120 | medium |
| `0x8076AFE0` | `fn_8076afe0` | 92 | high |
| `0x8076B040` | `fn_8076b040` | 72 | high |
| `0x8076B0A0` | `fn_8076b0a0` | 132 | medium |
| `0x8076B210` | `fn_8076b210` | 68 | high |
| `0x8076B3F0` | `fn_8076b3f0` | 176 | high |
| `0x8076B4A0` | `fn_8076b4a0` | 192 | high |

## Address range

`0x80769620..0x8076B4A0`: the launcher throw tail and the complete `cobKinopio`
carrier with its sibling constructor/destructor trio. The run is contiguous
except for `0x8076B090`, skipped below.

## Deliberate skips

- `0x8076B090` (4 bytes) is an empty function (`return;` only, a bare `blr`).
  There is no behavior to reconstruct, so it was left out rather than recorded
  as a body that does nothing.
- `0x80769E90` contains paired-single `GQR` lines in the export. The null guard,
  the two calls and the `<` threshold compare are literal, but the argument
  passing to `0x802BE470` is not recoverable; the body keeps that call as a
  no-argument function pointer with a `// unknown:` note and is marked `medium`.
- The rest of the `0x8076` bucket above `0x8076B4A0` remains for later waves,
  including the large bodies `0x80760700` (896), `0x80760B90` (932),
  `0x80763290` (780), `0x80763E50` (1056), `0x80766F20` (752), `0x8076C900`
  (2100), `0x8076DA30` (2656) and `0x8076EF80` (2544).
- The 32 addresses already in `ledger/functions.csv` were not duplicated:
  `0x80764270`, `0x80765C10`, and the whole wave-47 window
  `0x80767D60..0x80769600`.

At lane start the `0x8076` bucket held 203 functions, 32 of them already
translated. This pass adds 29, leaving 142 for later waves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave61` reports `OK` for
  `ledger/agents/wave61-bases.csv` (29 rows, 13 high / 16 medium).
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
