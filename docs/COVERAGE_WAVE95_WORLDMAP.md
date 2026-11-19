# Coverage - Wave 95 (`game::worldmap`, `0x80140000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); every `r2` constant below is read from `work/dol/main.elf` at the
absolute address recorded in its `W95_R2_*` comment. The export never names the
classes involved, so the bodies keep literal offsets and the two groups are
labelled by address.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 27 | 5,412 | 9 | 17 |

One body (`0x8014ECE0`) is `low` confidence because its export carries an
unrecovered jump table; no body is `stub` or `partial`.

## What this pass covers

- **`0x80146D00..0x80147B60` - the player action gates and pickup tracker.**
  `0x80146D00` starts or continues the landing action around the `+0x10D4`
  action bits, `0x80146E20` stores the two motion values and refreshes `+0x348`,
  and `0x80146F00`/`0x80146FA0` are the two `countLeadingZeros` kind classifiers
  that read `+0x1090` and the `+0x3CC` slot. `0x80147040`/`0x80147100` finish
  actions `0x8A` and `0x87`, `0x801474B0`/`0x801474F0` own the `0x5A` latch, and
  `0x80147560` posts the two `Wm_mr_itemget` effects on the `+0x2804`/`+0x292C`
  records. `0x80147680`, `0x80147A80` and `0x80147B60` gate item pickup, while
  `0x80147820` tracks the target distance with the `r2 - 0x56A4`/`0x56A8`
  thresholds and the `0x1E0`/`0xF0` cooldowns.
- **`0x8014CD70..0x8014ECE0` - the slide/skid actors.** The group owns the
  `+0x152C`/`+0x152D` pose bytes (`0x8014D070`), the `+0x14D8` sound/effect
  controller (`0x8014CEB0`, `0x8014E7B0`), the `Wm_mr_yssweat` spawner
  (`0x8014CFD0`), the two effect records at `+0x1694`/`+0x2804`/`+0x292C`
  (`0x8014CFD0`, `0x80147560`, `0x801474F0`) and the action wrappers
  `0x8014CD70`, `0x8014D2D0`, `0x8014D720`, `0x8014DAB0`, `0x8014DEF0`,
  `0x8014DF60`, `0x8014E0E0`, `0x8014E130`, `0x8014E910` and `0x8014ECE0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80146D00` | `fn_80146d00` | 288 | medium |
| `0x80146E20` | `fn_80146e20` | 108 | high |
| `0x80146F00` | `fn_80146f00` | 152 | high |
| `0x80146FA0` | `fn_80146fa0` | 156 | medium |
| `0x80147040` | `fn_80147040` | 188 | medium |
| `0x80147100` | `fn_80147100` | 324 | medium |
| `0x801474B0` | `fn_801474b0` | 20 | high |
| `0x801474F0` | `fn_801474f0` | 108 | medium |
| `0x80147560` | `fn_80147560` | 276 | medium |
| `0x80147680` | `fn_80147680` | 124 | medium |
| `0x80147820` | `fn_80147820` | 608 | medium |
| `0x80147A80` | `fn_80147a80` | 212 | medium |
| `0x80147B60` | `fn_80147b60` | 96 | medium |
| `0x8014CD70` | `fn_8014cd70` | 84 | high |
| `0x8014CEB0` | `fn_8014ceb0` | 284 | medium |
| `0x8014CFD0` | `fn_8014cfd0` | 148 | medium |
| `0x8014D070` | `fn_8014d070` | 596 | medium |
| `0x8014D2D0` | `fn_8014d2d0` | 460 | medium |
| `0x8014D720` | `fn_8014d720` | 56 | high |
| `0x8014DAB0` | `fn_8014dab0` | 244 | medium |
| `0x8014DEF0` | `fn_8014def0` | 104 | high |
| `0x8014DF60` | `fn_8014df60` | 216 | medium |
| `0x8014E0E0` | `fn_8014e0e0` | 76 | high |
| `0x8014E130` | `fn_8014e130` | 312 | medium |
| `0x8014E7B0` | `fn_8014e7b0` | 100 | high |
| `0x8014E910` | `fn_8014e910` | 56 | high |
| `0x8014ECE0` | `fn_8014ece0` | 16 | low |

## Deliberate skips

- **`0x801468D0`.** The export forwards `extraout_f1` into the `+0x400` slot and
  prints the receiver as `undefined8`; the lost argument cannot be reconstructed
  honestly, so no body was written.
- **`0x801469D0`, `0x801472F0`, `0x80147700`, `0x8014CDD0`.** Each export carries
  `unaff_GQR0` float loads and `ldexpf` reconstruction blocks, so the float
  constants cannot be recovered from the C alone. `0x80147700` is still called
  by raw address from [`fn_80147820`].
- **The remaining `0x80140000` entries.** This pass selected the two contiguous
  runs described above. Entries outside them (the GQR records listed by wave 90,
  `0x80144110..0x801459B0`, `0x8014A340..0x8014B040`, `0x8014F630..0x80150830`,
  `0x80151320..0x8015F7B0`) stay untranslated for a later pass.

## Verification

- Every address and byte size matches `functions.tsv`; the 27 sizes sum to
  5,412 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 27 were untranslated
  entries of the `0x80140000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs`. At the end of the pass the only remaining crate errors
  were in `src/audio/seq.rs` (another parallel lane).
- `node tools/check-lane-csv.mjs wave95` reports `OK`.
