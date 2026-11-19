# Coverage - Wave 106 (`game::worldmap`, `0x80110000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); offsets this pass introduces are declared with a `W106_` prefix
so they cannot collide with earlier passes. `// observed:` marks a literal from
the export, `// inferred:` a role guessed from call sites and `// unknown:` an
unresolved role.

The `0x80110000` bucket had 17 untranslated entries when wave 100 finished.
Fourteen are written here; the three deliberate skips are listed below.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 14 | 7,780 | 2 | 12 |

All fifteen rows are `translated`; none is a `stub` or `partial`.

## What this pass covers

- **`0x80110B60`/`0x80110FA0` - the `Fg3d_balloon` gate and contact.** The gate
  probes `(x, y + h)`, `(x, y - h)`, `(0, y)` and `(2x, y)` around
  `+0xAC/+0xB0`, writes `+0x1E7` and uses `+0x821` as the success byte. The
  contact response measures the horizontal separation, normalises it with
  `0x80170980`, and either doubles it and posts `0x80375188` through slot
  `+0xD4`, or stores it unscaled.
- **`0x80111850` - the balloon model sets.** Opens the `+0x524` display range,
  loads `Fg3d_balloon.brres`, then binds `balloon`/`float`,
  `balloon_back`/`float_back` and, when `+0x7EC == 1`, the
  `g3d_I_kinoko.brres`/`I_kinoko`/`wait` trio into `+0x544/+0x5C4/+0x634`,
  `+0x584/+0x5FC/+0x660` and `+0x6B8/+0x6F8`.
- **`0x801126A0` - the 4x2 ground probe fan.** Walks the four `0x802F4B20`
  records, skips a group whose mask is already set, accumulates the
  `1`/`2`/`4`/`8` mask and upgrades the result to `1` on a `3`/`4` contact from
  `0x80075270`.
- **`0x80116430`/`0x801177B0`/`0x80117DC0` - the grid builder trio.** The first
  builds one `0x240`-byte tile record (model, effect model, two extra layouts,
  `Effect0_d` index list at `+0x5F4`); the second lays out one `0x42`-cell row
  from the `r13 - 0x5380`/`r13 - 0x5388` cell sizes; the third allocates the
  27-tile `3 x 9` grid, adds the optional tenth tile and seeds the
  `+0x438..+0x478` interpolation table through the signed-to-double bias idiom.
- **`0x80118760` - the scene-extent quad.** Builds the four corners from
  `scene+0x40`/`scene+0x44`, the `0x80164B20` handle and a 60-byte matrix
  scratch, then emits through `0x8016DEC0`.
- **`0x8011C750`/`0x8011C8B0`/`0x8011CA90` - the grid box tests.** The pad-quad
  band test, the `16.0`-step box sweep through `0x800709C0` and the
  `8.0`-step height sampler through `0x800757B0`.
- **`0x8011D930`/`0x8011DA30`/`0x8011DAB0` - the ice-object spin handlers.**
  The state-`5` phase step, the per-player `-512`/`+512` offset and the speed
  clamp with the `0x80237BD0`/`0x80237C40` rotation pair.

## Paired-single notes

`GQR0` is cleared by `0x801AB170..0x801AB190` and restored only from the saved
thread context at `0x802EA6FC`; no instruction loads a non-zero value into it.
Every `if ((unaff_GQR0 & 0x3f00) != 0) { ldexpf(...); }` block in these exports
therefore discards its result, and the plain `r2` float operands are the body.

`GQR3` was the reason the `0x8011D930`/`0x8011DA30`/`0x8011DAB0` records stayed
untranslated in wave 100. The image writes `0x00050005` to `SPR 915` from
`0x8025EEB0`, `0x802AB4F0` and `0x802C2750` (`li r3,5` / `oris r3,r3,5`), i.e.
type `5` (unsigned 16-bit) with scale `0`; the only other writes are the boot
clear at `0x801AB174` and the context restore at `0x802EA708`. The three
`psq_l f1,0(r5),1,3` idioms are consequently plain `(u16)` loads, which the
bodies use.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80110B60` | `fn_80110b60` | 312 | medium |
| `0x80110FA0` | `fn_80110fa0` | 528 | medium |
| `0x80111850` | `fn_80111850` | 1,044 | medium |
| `0x801126A0` | `fn_801126a0` | 356 | medium |
| `0x80116430` | `fn_80116430` | 1,280 | medium |
| `0x801177B0` | `fn_801177b0` | 592 | medium |
| `0x80117DC0` | `fn_80117dc0` | 1,512 | medium |
| `0x80118760` | `fn_80118760` | 244 | medium |
| `0x8011C750` | `fn_8011c750` | 348 | medium |
| `0x8011C8B0` | `fn_8011c8b0` | 472 | high |
| `0x8011CA90` | `fn_8011ca90` | 484 | high |
| `0x8011D930` | `fn_8011d930` | 236 | medium |
| `0x8011DA30` | `fn_8011da30` | 120 | medium |
| `0x8011DAB0` | `fn_8011dab0` | 252 | medium |

## Deliberate skips

Three entries remain untranslated. Two are matrix-driven records whose Ghidra
exports invert branch polarity relative to the raw condition-register bits and
drop matrix-call operands; the third is GQR-dominated.

- **`0x80119240` (608).** The export opens with four `unaff_GQR0`/`ldexpf`
  lane conversions and reads `unaff_r2`/`unaff_r13` small-data words with no
  declared arguments; the nearest-cell arithmetic cannot be recovered without
  inventing the GQR type/scale and the incoming float pairs. It was claimed in
  the first pass but its body did not survive the file recovery, and the export
  does not support an honest replacement, so the row is withdrawn.

- **`0x801194A0` (1,960).** The export prints three one-argument
  `FUN_801c0bd0(...)` calls and three branch pairs whose `if`/`else` blocks are
  swapped against the `cror`-extracted CR bit in the raw listing
  (`0x80119698`..`0x801196B0`, `0x80119828`, `0x80119860`). Reproducing the
  translation-matrix chain from the C would require guessing the operands of
  six calls, so the body is left out rather than invented.
- **`0x80119C50` (1,200).** The same `cror`/branch inversion appears in the
  `+0x5F0` gate, and the 13-word matrix copy at `0x80119E20`..`0x80119E9C`
  replaces the `+0x608`/`+0x618` translation slots with computed positions
  whose source registers are not printed. The per-cell probe and the
  `0x8016A170`/`0x8016A130`/`0x8016A1A0` tail are literal, but the matrix chain
  is not.

Both were re-read from the raw ELF (`work/dol/main.elf`) before being skipped;
neither is omitted because it is long.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the 14 sizes sum to 7,780 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 14 were untranslated
  entries of the `0x80110000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave106` reports `OK` and
  `wave106-worldmap.csv: 14 rows`.