# Coverage - Wave 92 (`game::player`, the `0x80040000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 27 | 6120 | 10 | 17 |

## How to read this file

This document covers the Wave-92 slice of the `0x80040000` bucket of
`work/ghidra/main-gb/export/functions.tsv`. Every body lives in
`src/game/player.rs`; the ledger rows are in `ledger/agents/wave92-player.csv`.
The module now holds four hundred thirty-nine translated functions.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - all rows are `translated`: the whole control flow and every call
  are present.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a callee
  argument had to be inferred.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. Every `r2 - k` below is resolved against that base and
named `W92_*`. The shared float block keeps the wave-91 meanings
(`0x8042B874` = `1.0f`, `0x8042B878` = `0.0f`), so the bodies use the existing
`ACTOR_ONE` / `ACTOR_ZERO` constants rather than new ones.

## What this pass covers

- **`daFireBall` state machinery (`0x80040DE0..0x800414E0`, 5 bodies)** -
  [`fn_800414e0`] is the centre of the pass: it installs the four
  `daFireBall_Base_c` state descriptors (`Move`, `Kill`, `EatIn`, `EatNo`) at
  `0x80353F0C..0x8035404C` from four nine-word blocks at `0x80306B60`, with the
  compiler's rotated copy order reproduced through `W92_STATE_PERMUTATION`.
  [`fn_80040de0`] is the two-stage hit test, [`fn_80040e70`] the animation
  binder, [`fn_80040ec0`] the `Wm_mr_fireball_hit` spawner and
  [`fn_80040f70`] the position snapshot handed to the `+0x3EC` interface.
- **dokan / lift-ice sub-object builders (`0x80042480..0x80045710`, 4
  bodies)** - [`fn_80042650`] builds the `lift_ice_ashiba` model,
  [`fn_80045710`] the `obj_dokan` graph with its A/B shape nodes and four
  primitives, [`fn_80042480`] the grounded-event forwarder and
  [`fn_800437d0`] the three-word clear plus angle read.
- **`+0x30` chain and track leaves (`0x80044460..0x80045D00`, 7 bodies)** -
  [`fn_80044460`] builds the `0x2000`-byte scene arena and the `0x80`-byte copy
  buffer, [`fn_80044ff0`] repeats wave 72's five-probe chain walk,
  [`fn_800450d0`] repeats the `strrchr`-based name compare, and
  [`fn_800459f0`] / [`fn_80045b10`] / [`fn_80045d00`] are the three speed and
  track leaves. [`fn_80045590`] unpacks the two fields of the `+0x04` word and
  tail-calls the `0x80045A10` builder.
- **Actor constructor / destructor / mode hook
  (`0x800460D0..0x80046FC0`, 4 bodies)** - the largest reconstruction in this
  pass. [`fn_800460d0`] is the whole constructor (base class, class vtable,
  seven sub-records at `+0x5B0` then `0x128` stride, the `+0xDEC` effect
  record with its periodic timer, four `+0x1164` effect records, the two
  interfaces at `+0x1418`/`+0x1464`, and the owner bytes from the local-player
  record); [`fn_800467c0`] is the same field set torn down in reverse;
  [`fn_80046490`] is the timer's delete callback; [`fn_80046fc0`] is the
  mode-2 event reset.
- **Shared-handler getters (`0x80044940`, `0x80047270`, `0x80047380`,
  `0x8004D6F0`, 4 bodies)** - one body from the `0x80044940` pair and both ends
  of the `0x80047270..0x80047380` run plus the first of the
  `0x8004D6F0..0x8004D810` run. All are twelve bytes and return `0x80377A04`;
  the other thirty-five are byte-identical and are listed as a bulk item below.
- **Stopped-motion and contact gates (`0x80047630..0x80047DC0`, 3 bodies)** -
  [`fn_80047630`] is the three-way stop gate, [`fn_80047c40`] the contact-flag
  and hop-decay hook, and [`fn_80047dc0`] the brake/idle rate selector.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80040DE0` | `fn_80040de0` | 144 | medium |
| `0x80040E70` | `fn_80040e70` | 80 | medium |
| `0x80040EC0` | `fn_80040ec0` | 76 | high |
| `0x80040F70` | `fn_80040f70` | 128 | medium |
| `0x800414E0` | `fn_800414e0` | 888 | high |
| `0x80042480` | `fn_80042480` | 128 | medium |
| `0x80042650` | `fn_80042650` | 164 | medium |
| `0x800437D0` | `fn_800437d0` | 72 | medium |
| `0x80044460` | `fn_80044460` | 240 | medium |
| `0x80044940` | `fn_80044940` | 12 | high |
| `0x80044FF0` | `fn_80044ff0` | 224 | medium |
| `0x800450D0` | `fn_800450d0` | 136 | high |
| `0x80045590` | `fn_80045590` | 108 | medium |
| `0x80045710` | `fn_80045710` | 732 | medium |
| `0x800459F0` | `fn_800459f0` | 20 | medium |
| `0x80045B10` | `fn_80045b10` | 60 | high |
| `0x80045D00` | `fn_80045d00` | 20 | medium |
| `0x800460D0` | `fn_800460d0` | 828 | medium |
| `0x80046490` | `fn_80046490` | 88 | medium |
| `0x800467C0` | `fn_800467c0` | 652 | high |
| `0x80046FC0` | `fn_80046fc0` | 452 | high |
| `0x80047270` | `fn_80047270` | 12 | high |
| `0x80047380` | `fn_80047380` | 12 | high |
| `0x80047630` | `fn_80047630` | 348 | medium |
| `0x80047C40` | `fn_80047c40` | 196 | medium |
| `0x80047DC0` | `fn_80047dc0` | 288 | medium |
| `0x8004D6F0` | `fn_8004d6f0` | 12 | high |

Byte total: 6120. Address range covered: `0x80040DE0..0x8004D6F0`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket still holds 49 untranslated functions after this pass.

* **The thirty-five remaining shared-handler getters** - `0x80044950`,
  `0x80047280..0x80047370` (16) and `0x8004D700..0x8004D810` (18). All are
  twelve bytes and byte-identical to the four bodies translated here
  (`return &DAT_80377a04`). Translating them would add thirty-five rows of no
  new information; the four representatives give a later mechanical pass the
  exact shape to copy.
* **Eight forward-call stubs** - `0x800408F0`, `0x80041BA0`, `0x80041BD0`,
  `0x80043770`, `0x800437A0`, `0x80045190`, `0x800451C0` (48 B each) and
  `0x8004DB00` (52 B). The first seven all decompile to the same
  `FUN_802DCFBC(param_2, param_2, param_1)` shape, which no call site supports
  (48 bytes is far too large for that single call, so the export has lost the
  real prologue); `0x8004DB00` is the two-argument variant
  `FUN_802DCFBC(param_1, &DAT_80308300)`. There is no honest body to write
  without first reconstructing `0x802DCFBC`'s true prototype.
* **Five `GQR`-artefact bodies** - `0x800410F0` (212 B), `0x80044550` (520 B),
  `0x80045A10` (244 B), `0x80047F10` (360 B) and `0x80048630` (704 B). Each
  decompile is polluted by `ldexpf` on the paired-single quantisation registers
  (`unaff_GQR0`/`unaff_GQR3`) and by unreachable-block warnings, so the float
  conversions in the export are not the real arithmetic. They need the paired
  single instruction stream, which this export does not carry.
* **`0x800441E0` (432 B)** - the per-frame attachment-mask driver that would
  complete wave 72's `+0x442` table. It reads its owner from the *high* word of
  the `0x802DCD68` outlined-prologue artefact and writes through the *low* word
  as a pointer, a pair this module has not seen before. Guessing which half is
  the owner would have produced a wrong signature for a body that mutates the
  live attachment table, so it is left for a pass that can read the
  instructions.

## Verification

- Every address and byte size matches `functions.tsv` (checked row by row).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors and of warnings for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave92` reports `OK`.
