# Coverage - Wave 103 (`game::worldmap`, `0x80100000` bucket)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the offsets this pass introduces are declared in a `W103_`
block so they cannot collide with earlier passes. `// observed:` marks a
literal from the export, `// inferred:` a role guessed from call sites and
`// unknown:` an unresolved role.

The `0x80100000` bucket had 50 untranslated entries when wave 102 finished.
Twenty-five are written here; the twenty-five deliberate skips are listed below.
One of the 50 (`0x8011C100`) already had a complete body in the Rust file from
wave 89 but no merged ledger row, so this pass did not re-claim it and removed
the duplicate it had started to write.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 25 | 7,416 | 5 | 19 | 1 |

Every row is `translated`; no body is `stub` or `partial`.

## What this pass covers

- **`0x80102DB0` and `0x8010E7D0`/`0x8010E800` - small shared helpers.**
  `0x80102DB0` is the `value / fn_80102e00(mode)` ratio with the f32 rounding
  the export prints. The `0x8010E7D0`/`0x8010E800` pair is byte-identical and
  calls `0x802DCBFC` with the first two registers duplicated.
- **`0x8010F600` - the warning-state machine.** Keeps the level byte at
  `r13-0x5394` (`0xFF` inside the limits, decay by ten outside), records the
  accepted width/height at `r13-0x5390`/`-0x538C`, and refreshes `+0x144` and
  `+0x1DC` through `0x80006870` whenever the level changes.
- **`0x80109780` - the scratch-record submit.** Fills a `0x38`-byte stack
  record with `0x802D0CC0`, copies the two SDA words at `r2-0x5DF0`/`-0x5DEC`
  into `+0x18`/`+0x1C`, and submits through `0x802D0F40`.
- **`0x8010F980`/`0x8010FB50` - the transform and coin packets.** `0x8010F980`
  builds the six-word `+0xDC..+0xB4` block and drives the three
  `0x8016ECB0`/`0x8016EDD0`/`0x8016EEF0` halfword setters. `0x8010FB50` creates
  the `0x20`-byte GX range, loads `obj_coin.brres` through `0x800DF180` and
  resolves the style model through `0x80239E30`.
- **`0x8010F0C0` - the `WiiStrap` archive loader.** Opens `WiiStrap.arc`,
  appends the language name and message file, binds the font resource and
  finishes the archive. The six language branches use the exact SDA string
  addresses the export prints.
- **`0x80101AA0` - the camera-window wrap.** Shifts a height by
  `r2-0x5F50` while it lies outside the `0x800789A0`/`0x80078B40` bounds, with
  the two-iteration cap and the single-screen early-out.
- **`0x8011EC10`/`0x8011F1D0` - the player contact probes.** `0x8011EC10`
  runs the four `0x800767F0` ground probes around `+0x5B4`; `0x8011F1D0`
  resolves the grab position through `0x8012DE80`, two matrix products and
  `0x8016F130`, falling back to `+0xAC..+0xB4`.
- **`0x8010DC80`/`0x8010DE30`/`0x8010FF70` - the selection queue.** The two
  overlay functions rotate the four `+0xB54`/`+0xB7C` slots, play the UI sound
  and commit the selected handler; `0x8010FF70` walks six `+0x30` links and
  tests vtable slot `0x0C`.
- **`0x80106620`/`0x801068B0` - the spline helpers.** The first interpolates a
  four-control-point point from the two `0x802E55A4` curve weights; the second
  rebuilds the current point from its neighbours and appends the rotated
  cross-product offset to the next slot.
- **`0x8010A3E0`/`0x80108F70`/`0x8010AD00`/`0x8010AAA0` - the GX setup
  group.** These rebuild the ortho/TEV/scissor/viewport state, upload the
  colour and texture records, and write the `0xCC008000` FIFO register. The
  `0x43300000` blocks are the compiler's unsigned-short-to-double idiom and are
  reproduced through `unsigned_to_double`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80101AA0` | `fn_80101aa0` | 288 | medium |
| `0x80102DB0` | `fn_80102db0` | 76 | medium |
| `0x80106620` | `fn_80106620` | 648 | medium |
| `0x801068B0` | `fn_801068b0` | 608 | medium |
| `0x80108F40` | `fn_80108f40` | 48 | high |
| `0x80108F70` | `fn_80108f70` | 752 | medium |
| `0x80109780` | `fn_80109780` | 100 | medium |
| `0x8010A3E0` | `fn_8010a3e0` | 724 | medium |
| `0x8010AAA0` | `fn_8010aaa0` | 532 | medium |
| `0x8010AD00` | `fn_8010ad00` | 492 | medium |
| `0x8010DC80` | `fn_8010dc80` | 408 | medium |
| `0x8010DE30` | `fn_8010de30` | 400 | medium |
| `0x8010E7D0` | `fn_8010e7d0` | 48 | medium |
| `0x8010E800` | `fn_8010e800` | 48 | medium |
| `0x8010F0C0` | `fn_8010f0c0` | 552 | medium |
| `0x8010F470` | `fn_8010f470` | 108 | high |
| `0x8010F600` | `fn_8010f600` | 300 | medium |
| `0x8010F7A0` | `fn_8010f7a0` | 20 | high |
| `0x8010F980` | `fn_8010f980` | 240 | medium |
| `0x8010FB50` | `fn_8010fb50` | 180 | medium |
| `0x8010FE20` | `fn_8010fe20` | 92 | high |
| `0x8010FE80` | `fn_8010fe80` | 12 | high |
| `0x8010FF70` | `fn_8010ff70` | 224 | low |
| `0x8011EC10` | `fn_8011ec10` | 248 | medium |
| `0x8011F1D0` | `fn_8011f1d0` | 268 | medium |

## Deliberate skips

Twenty-five entries of the 50 remain untranslated. Each one either loses an
operand in the C export or was already rejected by an earlier pass for the same
reason.

- **`0x80103410` (16).** A tail thunk onto `0x80103440`; Ghidra prints no
  arguments at all, so the allocator's `base`/`tag` pair cannot be stated
  without inventing a signature.
- **`0x80107050` (280), `0x80107170` (320) and `0x80107460` (580).** The two
  formatters read the caller's varargs from `stack0x00000008` (`in_cr1` guard
  and `local_434`/`local_430` pointers); the Rust body cannot reconstruct that
  home area. `0x80107460` calls both of them, so it inherits the loss.
- **`0x801097F0` (344).** Packs the result of `0x802DCC4C` into the low byte of
  a word whose upper three bytes are set to `0xFF`, while the surrounding stack
  record is passed to `0x802D1850`; the return register width is not recoverable
  from the C, so the packed word cannot be reproduced honestly.
- **`0x80118760` (244).** Writes a four-byte result into `local_a8[0]` but never
  initialises `local_a8[1]`, then hands the pair to `0x802540B0`; the missing
  word would have to be invented.
- **`0x801177B0` (592).** Uses the high word of the `_savegpr0` return as an
  object pointer and the low word as a loop count, then scales through the
  `GQR0` scratch. The object/count split is not provable from the export.
- **`0x8011DA30` (120).** The `GQR3` pair (`bVar1`/`lVar2`) reaches the angle
  value and `0x80162590` is called with no arguments, so both the scale and the
  actor operand are lost.
- **`0x8010B890` (1,652) and `0x8010E830` (1,616).** These are large
  state-manager registration tables: dozens of absolute pointer words copied
  into the `0x80374XXX` globals. There is no arithmetic to translate, only a
  data dump the C export does not prove complete.
- **The wave-100 GQR/probe group.** `0x80110B60`, `0x80110FA0`, `0x80111850`,
  `0x801126A0`, `0x80116430`, `0x80117DC0`, `0x80119240`, `0x801194A0`,
  `0x80119C50`, `0x8011C750`, `0x8011C8B0`, `0x8011CA90`, `0x8011D930` and
  `0x8011DAB0` were already left untranslated by wave 100 because the `GQR0`/
  `GQR3` scale reaches a value or because a loader/probe argument is dropped.
  This pass did not re-litigate those rejections.
- **`0x8011C100` (132).** Already translated in `src/game/worldmap.rs` by wave
  89 (`fn_8011c100`, same address), but the address is absent from
  `ledger/functions.csv`. Writing it again produced a duplicate symbol, so the
  duplicate was removed and no row was claimed. The coordinator may want to
  merge that existing body as its own ledger row.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the 25 sizes sum to 7,416 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 25 were untranslated
  entries of the `0x80100000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave103` reports `OK` for
  `ledger/agents/wave103-worldmap.csv`.
