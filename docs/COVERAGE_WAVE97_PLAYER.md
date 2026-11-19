# Coverage - Wave 97 (`game::player`, the `0x80040000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 38 | 1236 | 35 | 3 |

## How to read this file

This document covers the Wave-97 slice of the `0x80040000` bucket of
`work/ghidra/main-gb/export/functions.tsv`: the 38 of the 49 untranslated
functions that could be reconstructed honestly. Every body lives in
`src/game/player.rs`; the ledger rows are in
`ledger/agents/wave97-player.csv`. The module now holds four hundred
seventy-seven translated functions, and the bucket has 11 untranslated
functions left (all listed under "Deliberate skips" below).

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
and `r13 = 0x8042F680`. Every `r2 - k` / `r13 - k` below is resolved against
those bases and named `W97_*`. The shared float block keeps the wave-91
meanings (`0x8042B874` = `1.0f`, `0x8042B878` = `0.0f`), so the bodies reuse the
existing `ACTOR_ONE` / `ACTOR_ZERO` constants, and `0x8042B828` keeps its
wave-92 name `W92_SPEED_BIAS_8042B828`.

## What this pass covers

- **The thirty-five remaining shared-handler getters (`0x80044950`,
  `0x80047280..0x80047370`, `0x8004D700..0x8004D810`)** - this closes the whole
  family of thirty-nine twelve-byte getters that wave 92 left as a mechanical
  bulk item. Every one of the 35 exports was re-read and is byte-identical to
  the four wave-92 anchors: `return &DAT_80377a04`. The run boundaries are
  `0x80044940..0x80044950` (2), `0x80047270..0x80047380` (18) and
  `0x8004D6F0..0x8004D810` (19); each body cites its own export file.
- **`0x80045A10`, the fireball/track builder (244 B)** - the seven-argument
  builder that `fn_80045590` tail-calls. It stores the speed at `+0x5C8`,
  clears `+0xDC..+0xE4`, copies the selected twelve-byte record's `+0x08`
  halfword to `+0x104`, builds the `+0x4E4` key through the twelve-argument
  `0x8007FA40`, stores the caller's word at `+0x5B0` and finishes with
  `0x8007F900`, [`fn_80045710`] and [`fn_800459f0`]. The export's entry/exit
  `GQR0` guards only cover the two `f64` -> `f32` conversions.
- **`0x800410F0`, the `0x284` effect driver (212 B)** - copies `+0xAC`/`+0xB0`
  into a two-word block, binds that block to descriptor `0x284`, posts a
  seven-word record built from the `0x8042B748` word, plays the `0x12` sound at
  the same position and raises `+0x408`.
- **`0x80047F10`, the two-point terrain probe (360 B)** - the predicate that
  reads class vtable slots `+0xEC` (record with two word offsets) and `+0xF8`
  (an angle) and runs `0x80075CA0` on `+0xAC + offset / 0x8042B8B0`, comparing
  the returned surface with `(y + angle) - 1.0f`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800410F0` | `fn_800410f0` | 212 | medium |
| `0x80044950` | `fn_80044950` | 12 | high |
| `0x80045A10` | `fn_80045a10` | 244 | medium |
| `0x80047280` | `fn_80047280` | 12 | high |
| `0x80047290` | `fn_80047290` | 12 | high |
| `0x800472A0` | `fn_800472a0` | 12 | high |
| `0x800472B0` | `fn_800472b0` | 12 | high |
| `0x800472C0` | `fn_800472c0` | 12 | high |
| `0x800472D0` | `fn_800472d0` | 12 | high |
| `0x800472E0` | `fn_800472e0` | 12 | high |
| `0x800472F0` | `fn_800472f0` | 12 | high |
| `0x80047300` | `fn_80047300` | 12 | high |
| `0x80047310` | `fn_80047310` | 12 | high |
| `0x80047320` | `fn_80047320` | 12 | high |
| `0x80047330` | `fn_80047330` | 12 | high |
| `0x80047340` | `fn_80047340` | 12 | high |
| `0x80047350` | `fn_80047350` | 12 | high |
| `0x80047360` | `fn_80047360` | 12 | high |
| `0x80047370` | `fn_80047370` | 12 | high |
| `0x80047F10` | `fn_80047f10` | 360 | medium |
| `0x8004D700` | `fn_8004d700` | 12 | high |
| `0x8004D710` | `fn_8004d710` | 12 | high |
| `0x8004D720` | `fn_8004d720` | 12 | high |
| `0x8004D730` | `fn_8004d730` | 12 | high |
| `0x8004D740` | `fn_8004d740` | 12 | high |
| `0x8004D750` | `fn_8004d750` | 12 | high |
| `0x8004D760` | `fn_8004d760` | 12 | high |
| `0x8004D770` | `fn_8004d770` | 12 | high |
| `0x8004D780` | `fn_8004d780` | 12 | high |
| `0x8004D790` | `fn_8004d790` | 12 | high |
| `0x8004D7A0` | `fn_8004d7a0` | 12 | high |
| `0x8004D7B0` | `fn_8004d7b0` | 12 | high |
| `0x8004D7C0` | `fn_8004d7c0` | 12 | high |
| `0x8004D7D0` | `fn_8004d7d0` | 12 | high |
| `0x8004D7E0` | `fn_8004d7e0` | 12 | high |
| `0x8004D7F0` | `fn_8004d7f0` | 12 | high |
| `0x8004D800` | `fn_8004d800` | 12 | high |
| `0x8004D810` | `fn_8004d810` | 12 | high |

Byte total: 1236. Address range covered: `0x800410F0..0x8004D810`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket now holds 11 untranslated functions.

* **Eight outlined-prologue stubs** - `0x800408F0`, `0x80041BA0`,
  `0x80041BD0`, `0x80043770`, `0x800437A0`, `0x80045190`, `0x800451C0`
  (48 B each) and `0x8004DB00` (52 B). The first seven decompile to the single
  call `FUN_802DCFBC(param_2, param_2, param_1)` and the last to
  `FUN_802DCFBC(param_1, &DAT_80308300)`; the call has lost the outlined
  register-save prologue that every 48-byte body needs, so there is no honest
  body to write without first recovering that stub's real prototype. This is
  the same finding wave 92 recorded, re-checked against the exports.
* **`0x800441E0` (432 B)** - the per-frame attachment-mask driver that would
  complete wave 72's `+0x442` table. It takes its owner from the *high* word of
  the `0x802DCD68` outlined-prologue artefact and writes through the *low*
  word, a pair this module has not seen before. Choosing which half is the
  owner would produce a wrong signature for a body that mutates the live
  attachment table, so it is left for a pass that can read the instructions.
* **`0x80044550` (520 B)** - a ring/spiral builder whose arithmetic is
  expressed entirely through `unaff_GQR0`/`unaff_GQR3` `ldexpf` guards and
  eight "removing unreachable block" warnings; the C form of the
  paired-single quantisation is not the real arithmetic.
* **`0x80048630` (704 B)** - the velocity driver that scales `+0x10C` and
  `+0xEC` through the same paired-single quantisation (three `GQR` blocks and
  three unreachable-block warnings around the `+0x112A` halfword), mixed with
  the `FUN_80237C40`/`FUN_80237BD0` trig helpers. The branch tree is legible
  but the float conversions are not, so a full body would have to guess.

## Verification

- Every address and byte size matches `functions.tsv` (checked row by row).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/game/player.rs`.
- No address was already in `ledger/functions.csv` (all 38 were confirmed
  untranslated before a body was written).
- `cargo check --message-format=short` is free of errors and of warnings for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave97` reports `OK` (38 rows, 38 unique
  addresses).
