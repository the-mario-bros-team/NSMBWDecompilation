# Coverage - Wave 49 (game::player — scratch-word rotate, interface wrappers, counter bytes, table initialisers, movement-bit leaves)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 30 | 1736 | 23 | 7 |

## How to read this file

This document covers the Wave-49 slice of the gameplay player actor from the
`0x80050000` bucket of `work/ghidra/main-gb/export/functions.tsv`. Every body
lives in `src/game/player.rs`; the ledger rows are in
`ledger/agents/wave49-player.csv`. The module now holds two hundred forty
translated functions.

* **Address** — virtual entry point in the DOL image (not a file offset).
* **Name** — the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** — one plain sentence. Field roles marked "inferred" below come from
  the shared offsets, not from a recovered symbol.
* **Status** — all rows are `translated`: the full control flow and every call
  are present.
* **Confidence** — strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = some field or callee roles guessed.
* **Evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Bodies use the same helpers as the rest of the module. The `0x8042B878`
constant behind the `r2 - 0x7808` load in `fn_80057d70` is `0.0f`, as recorded
in `docs/COVERAGE_WAVE39_PLAYER.md`; `0x8019A6D0` is called with the `f64`
prototype already used in `src/game/worldmap.rs`.

## What this pass covers

- **Scratch-word / event-flag cluster (`0x80057C80..0x80058110`)** — the
  `+0x1088` word rotate with its `0xAC` flag clear, the `+0x1088` getter, the
  `0x53`/`0x64` clear-pair predicate, the `0x58`/`0x59` flag-and-speed gate, the
  `0x75`/`0xB5` playback predicate, and the four `+0xDEC` interface dispatch
  wrappers (slots `+0x24`, `+0x28`, `+0x30`, `+0x34`) plus the `0x8019A6D0`
  effect spawn that uses the absolute `+0x10C` speed. Start at `fn_80057c80`.
- **Counter-byte cluster (`0x800581A0..0x80058260`)** — the two saturating
  signed counter bytes at `+0x1080` and `+0x1082`, each with a clear and a
  compare-and-increment pair.
- **Slot-writer and sub-object comparators (`0x80056370`, `0x800565A0`,
  `0x800565E0`)** — the `+0x3B0` presence-bit writer with its indexed `+0x3B8`
  value slot, and the two `+0xE0`-word sub-object comparisons over
  `+0x244`/`+0x248` and `+0x254`/`+0x250`.
- **Track-entry table cluster (`0x8005DBE0..0x8005DD80`)** — the conditional
  release helper, the four-float component selector, the 12-byte entry seeder
  fed from the `0x24`-byte `0x802F1364` records, the 177-entry clear-and-reseed
  pair, and the slot-`0` constructor wrapper.
- **Movement-bit leaves (`0x8005E460..0x8005E610`)** — bit clear on the `+0x12`
  halfword, the four-halfword/two-byte reset, the `0xFFF3` mask clear on
  `+0x04`/`+0x06`, and the `0x4`/`0x100`/`0x200` tests.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80057C80` | `fn_80057c80` | 64 | high |
| `0x80057CC0` | `fn_80057cc0` | 8 | high |
| `0x80057CD0` | `fn_80057cd0` | 88 | high |
| `0x80057D70` | `fn_80057d70` | 144 | medium |
| `0x80057E00` | `fn_80057e00` | 104 | high |
| `0x80057E70` | `fn_80057e70` | 96 | high |
| `0x80057EE0` | `fn_80057ee0` | 112 | high |
| `0x80057F60` | `fn_80057f60` | 96 | high |
| `0x80057FD0` | `fn_80057fd0` | 112 | high |
| `0x80058110` | `fn_80058110` | 96 | medium |
| `0x800581A0` | `fn_800581a0` | 12 | high |
| `0x800581B0` | `fn_800581b0` | 32 | high |
| `0x80058250` | `fn_80058250` | 12 | high |
| `0x80058260` | `fn_80058260` | 32 | high |
| `0x80056370` | `fn_80056370` | 68 | medium |
| `0x800565A0` | `fn_800565a0` | 64 | medium |
| `0x800565E0` | `fn_800565e0` | 64 | medium |
| `0x8005DBE0` | `fn_8005dbe0` | 64 | high |
| `0x8005DC40` | `fn_8005dc40` | 60 | medium |
| `0x8005DCA0` | `fn_8005dca0` | 40 | medium |
| `0x8005DCD0` | `fn_8005dcd0` | 96 | high |
| `0x8005DD30` | `fn_8005dd30` | 80 | high |
| `0x8005DD80` | `fn_8005dd80` | 52 | high |
| `0x8005E460` | `fn_8005e460` | 32 | high |
| `0x8005E480` | `fn_8005e480` | 32 | high |
| `0x8005E4A0` | `fn_8005e4a0` | 28 | high |
| `0x8005E560` | `fn_8005e560` | 12 | high |
| `0x8005E570` | `fn_8005e570` | 12 | high |
| `0x8005E580` | `fn_8005e580` | 12 | high |
| `0x8005E610` | `fn_8005e610` | 12 | high |

Byte total: 1736. Address range covered: `0x80056370..0x8005E610`
(non-contiguous; the group gaps are the deliberate skips below).

## Deliberate skips and why

The bucket still holds 161 untranslated functions. The ones examined and
skipped in this pass:

* `0x80053940`, `0x80053AA0`, `0x80053AD0`, `0x80053C80`, `0x80053DF0`,
  `0x80053E60`, `0x80057860` — paired-single code rendered as
  `ldexpf`/`unaff_GQR0`; the float lanes cannot be reconstructed honestly from
  the C export alone.
* `0x80058280`, `0x800582A0`, `0x800582F0` — read `r13`-based globals
  (`unaff_r13 - 0x5678`, `- 0x5658`) or the unresolved `r2 - 0x77BC` constant,
  so the call seams cannot be justified.
* `0x8005D7E0`, `0x8005D830` — share the `r13 - 0x59E8` counter whose identity
  is not resolved; `0x8005E590`, `0x8005E5D0` depend on `r13 - 0x5730` and the
  table entry reached through it.
* `0x8005DC80`, `0x8005DDC0` — paired-single code in the export. Their exact
  call shapes are still used from `fn_8005dcd0` and `fn_8005dd80` via
  `0x8005DC80`/`0x8005DDC0`; only the callee bodies are skipped.
* `0x80056C30` — the signed `param_2 / 32` bit-array expression is emitted as a
  single convoluted shift; it is reconstructible only after re-reading the
  instruction stream, which was out of scope for this pass.
* `0x800544F0` (2540 bytes) and `0x800592C0` (7620 bytes) — far beyond a single
  honest body for this pass.
* `0x80054F80`, `0x80055330`, `0x800556F0`, `0x80055810`, `0x80055900`,
  `0x80055A00`, `0x80055A70`, `0x80055DB0`, `0x80055EF0` — call `r13`-based
  globals or hide float arguments, as already recorded in
  `docs/COVERAGE_WAVE39_PLAYER.md`.
* `0x8005E4E0` and the `0x8005E5xx`/`0x8005E6xx`/`0x8005E7xx` variants — the
  same trivial movement-bit shapes, but the lane target was met at 30 bodies,
  so the rest (including the `r13`-dependent `0x8005E590`/`0x8005E5D0`) is
  left as clean future work rather than padding.
* The remaining `0x80055AE0..0x80055FF0`, `0x80056620..0x80057CC0` (except the
  leaves above), `0x800583A0..0x80059290`, `0x8005B0F0..0x8005D0D0` (except the
  two track leaves) and `0x8005DFD0..0x80060000` groups — different object
  clusters than this pass's player-actor state block; not worked here.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave49` reports `OK`.
