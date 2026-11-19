# Coverage - Wave 76 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. Every function below is a literal reconstruction of one
Ghidra body; the doc block above each body names the original address, the byte
count from `work/ghidra/main-gb/export/functions.tsv` and the evidence file.
Offsets are relative to the object named in the row. "high" confidence means
every store, branch and call is visible in the export; "medium" means a callee
operand, an SDA2 constant value or a field role had to be inferred from a
sibling function or call site.

This pass only edits `src/engine/display.rs`; the ledger is merged by the
coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 30 | 3256 | 19 | 11 |

## What this pass covers

The `0x800D0000` bucket entered this pass with 130 untranslated of its 251
functions. This pass takes four coherent slices of the same engine band. It
deliberately stays out of every paired-single body.

- **The rest of the `0x80303408` actor wrapper** (`0x800D5B90..0x800D6D10`,
  9 bodies). [`fn_800d5b90`] switches the display handle from the mode at
  `+0x180`, [`fn_800d5be0`] reports a pending action with the `-1` sentinel at
  `+0x158`, [`fn_800d5d00`] advances the interface at `+0x60`, and
  [`fn_800d5d10`] buckets a frame count into `+0x168`. [`fn_800d6840`] copies
  the 0x20-byte 2-jump record for kind 5/8/9, [`fn_800d6a30`] interpolates
  `+0x184` toward `+0x188` over `+0x18C` frames, and [`fn_800d6ab0`] tests the
  action at `+0x154` against eight SDA2 animation windows. [`fn_800d6cc0`] and
  [`fn_800d6d10`] construct the `0x80318A80` node and its actor for modes
  `0..1`, `2..4` and `5`.
- **The `Wm_mr_spinjump` display object** (`0x800D7480..0x800D8510`, 9 bodies).
  [`fn_800d81e0`] constructs it with vtable `0x80318C20` and the embedded
  interface `0x80329958`; [`fn_800d8280`] binds the owner and publishes the
  object at `owner + 0x388`; [`fn_800d82a0`] applies one of six motion presets
  to `+0x0C/+0x10/+0x13C`; [`fn_800d8420`] spawns the model by name through
  interface slot `+0xA0` or the player-aware `0x8008FBA0`. [`fn_800d84f0`] and
  [`fn_800d8510`] are the mode equality test and the halfword timer.
  [`fn_800d7480`], from the neighbouring integer-motion object, clamps its
  `+0x5C` value into the interval spanned by `+0x3C`/`+0x4C`.
- **The intrusive actor list node** (`0x800D9AC0..0x800DAED0`, 8 bodies). The
  node is constructed by [`fn_800dadb0`], reset by [`fn_800dae60`], unlinked by
  [`fn_800daed0`] against the global head/tail pair at `r13 - 0x5660` and
  `r13 - 0x5664`, and destroyed by [`fn_800dae00`]. [`fn_800d9ac0`],
  [`fn_800d9b60`], [`fn_800d9d00`] and [`fn_800d9d40`] are the four small
  target/flag helpers that sit immediately before the list in the same band.
- **Three standalone helpers** (`0x800DA650`, `0x800DC990`,
  `0x800DF860..0x800DF890`). [`fn_800da650`] forms `sqrt(a*a - b*b)` through the
  `0x80237BA0` inverse square root. [`fn_800dc990`] is the wave-38 route-flag
  store at slot `+0x55`. [`fn_800df860`] permutes eight halfwords and
  [`fn_800df890`] is a pure integer oscillator step over seven halfwords plus a
  flag word.

Two structural facts carry across several bodies. First, the `0x80303408`
wrapper and the `0x80318C20` display object share the animation interface at
`+0x28`/`+0x14`, so `0x801680E0`, `0x801690E0` and `0x8015F480` appear in both
clusters. Second, the list node uses `+0x08`/`+0x0C` as previous/next while the
wave-63 search context uses `+0x0C`/`+0x10`; they are different objects and the
offset tables in the module header keep them apart.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D5B90` | `fn_800d5b90` | 48 | high |
| `0x800D5BE0` | `fn_800d5be0` | 40 | high |
| `0x800D5D00` | `fn_800d5d00` | 8 | high |
| `0x800D5D10` | `fn_800d5d10` | 184 | medium |
| `0x800D6840` | `fn_800d6840` | 240 | high |
| `0x800D6A30` | `fn_800d6a30` | 124 | medium |
| `0x800D6AB0` | `fn_800d6ab0` | 336 | medium |
| `0x800D6CC0` | `fn_800d6cc0` | 76 | high |
| `0x800D6D10` | `fn_800d6d10` | 212 | high |
| `0x800D7480` | `fn_800d7480` | 64 | high |
| `0x800D80B0` | `fn_800d80b0` | 28 | high |
| `0x800D80D0` | `fn_800d80d0` | 244 | medium |
| `0x800D81E0` | `fn_800d81e0` | 160 | medium |
| `0x800D8280` | `fn_800d8280` | 28 | high |
| `0x800D82A0` | `fn_800d82a0` | 376 | medium |
| `0x800D8420` | `fn_800d8420` | 208 | medium |
| `0x800D84F0` | `fn_800d84f0` | 20 | high |
| `0x800D8510` | `fn_800d8510` | 40 | high |
| `0x800D9AC0` | `fn_800d9ac0` | 12 | high |
| `0x800D9B60` | `fn_800d9b60` | 28 | high |
| `0x800D9D00` | `fn_800d9d00` | 64 | high |
| `0x800D9D40` | `fn_800d9d40` | 48 | high |
| `0x800DADB0` | `fn_800dadb0` | 68 | medium |
| `0x800DAE00` | `fn_800dae00` | 96 | medium |
| `0x800DAE60` | `fn_800dae60` | 32 | high |
| `0x800DAED0` | `fn_800daed0` | 80 | high |
| `0x800DA650` | `fn_800da650` | 88 | medium |
| `0x800DC990` | `fn_800dc990` | 12 | high |
| `0x800DF860` | `fn_800df860` | 40 | high |
| `0x800DF890` | `fn_800df890` | 252 | medium |

Byte total: `1268` (actor wrapper) + `1168` (`Wm_mr_spinjump` and the
`0x800D7480` clamp) + `428` (list node) + `392` (standalone helpers) = `3256`.

## Deliberate skips

- **`0x800D5C10` / `0x800D5C90`** (100 bytes each). Both forward an `undefined8`
  first parameter to a virtual slot (`+0x88` / `+0x90`) and to `0x80168FE0` /
  `0x801680E0`. The export drops the double operand at the first call and never
  states the register class of the eight-byte value, so the operand order
  cannot be recovered honestly.
- **`0x800D5C80`** (8 bytes) calls `0x80168FE0(object + 0x60)` with only the
  integer operand visible; the callee's first (double) argument is dropped.
- **`0x800D6940` / `0x800D69B0`** (100 + 116 bytes). The three eight-byte
  operands are reordered between the caller and the callee, and `0x800D69B0`
  receives five arguments while the export declares four. The register classes
  of those values are not recoverable from the export.
- **`0x800D5730`, `0x800D6DF0`, `0x800D6E90`, `0x800D7000`, `0x800D7020`** stay
  skipped for the reasons wave 63 recorded: argument-less virtual calls or a
  single visible operand.
- **Paired-single / GQR bodies** `0x800D57F0`, `0x800D5DD0`, `0x800D6460`,
  `0x800D6550`, `0x800D65E0`, `0x800D9D70`, `0x800DAA10` and `0x800DAC40` were
  left out: Ghidra emits `unaff_GQR0`/`ldexpf` blocks whose lane scaling cannot
  be reconstructed from the C export. `0x800DAB40` is the non-GQR sibling of
  `0x800DA650`, but it only feeds the GQR search at `0x800DAA10` and was left
  with that cluster.
- **`0x800D8850`** mixes the `0x80318D58`/`0x80318D38` pointer-plus-float tables
  with player slots and needs a table type the export does not provide.
- **`0x800DFBE0`, `0x800DFC70`, `0x800DFD20`, `0x800DFDD0`, `0x800DFE60`,
  `0x800DFF30`** are the name/locale table helpers. Their bodies call
  `0x8023B2A0` and several virtual slots with every operand dropped
  (`FUN_8023b2a0()` with no arguments), so they were deferred rather than
  padded with invented operands.
- **`0x800DD1C0`, `0x800DD1F0`, `0x800DD220`, `0x800DD250`, `0x800DD280`,
  `0x800D7C20`, `0x800D7C50`** are the identical 48-byte forwarding thunks
  already documented as a thunk bank in wave 63.
- **`0x800DC800` and `0x800DCED0`** are genuine empty 4-byte bodies. They are
  honest but carry no information, so they were left for a bulk pass.

## Verification

- Every address and byte size matches `functions.tsv` (checked mechanically:
  zero mismatches over the 30 rows, total 3256 bytes).
- Every evidence file exists at the cited path and every row has exactly one
  `pub unsafe fn fn_<addr>` body in `src/engine/display.rs`.
- `ledger/functions.csv` was read only; none of the 30 addresses was present
  before this pass, and `ledger/agents/wave76-display.csv` has 30 unique rows.
- `node tools/check-lane-csv.mjs wave76` reports `OK`: this lane's CSV passes
  the header, field-count, quoting, evidence-path, duplicate-address and
  byte-size checks.
- `cargo check --message-format=short` reports no errors and no warnings for
  `src/engine/display.rs`.
