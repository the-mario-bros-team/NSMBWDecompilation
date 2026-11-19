# Coverage - Wave 195 (`game::bases` 256-300 band, part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 13,940 | 43 | 7 |

First wave in the 256-300 band; these are the largest bodies translated so far
(248-296 bytes each).

## Highlights

- **Shared helpers now real.** `0x806601B0`
  (`fn(position, low, high) -> f32`, `p == high` returns 0.0), `0x8068C5A0`
  (`fn(height: f32, this) -> f32`), `0x80693DD0` (`(this, candidate) -> u32`).
- **Results/save board (10).** `0x80626B80` (`dModelPlayDate_c` loader),
  `0x80628610` (ten-pane broadcast), `0x8062BCF0` (save-board stepping),
  `0x8062CFF0` (coin-pane hiding), `0x80636F30`, `0x80637450` (self-recursive
  promotion copy), `0x80638440`, `0x80638C80`, `0x8063A740`, `0x8064AAC0`.
- **Path/lift bodies (8).** `0x8064D650`, `0x8064E060`, `0x8064E1D0`,
  `0x806DEFD0`, `0x806E2E40`, `0x806E8340`, `0x806ED340`, `0x806F1210`.
- **Actor/model bodies (10).** `0x806F5640`, `0x80705600`, `0x80707F40`,
  `0x807103D0`, `0x80733B00`, `0x80736890`, `0x80737270`, `0x80738460`,
  `0x807435A0`, `0x80746400`.
- **The rest** are distance ladders, matrix sweeps and small state helpers.

## Corrections and recoveries

- `0x806601B0` is integer-window, not double: the `CONCAT44` ladder carries the
  `2^52 + 2^31` bias at `0x807C8AF8`; `p == high` returns `0.0`, not `1.0`.
- `0x8064D650`'s `lfs 0x38(r31)` is dead; `+0xB4` receives the following
  `lwz r0,0xC(r1)` word.
- `0x8064E1D0`'s export loses the float args; the raw passes `(dy, dx)` and the
  six spilled stack floats are never read.
- `0x80705600`'s `addic. r4,r1,200` is the `&local != NULL` idiom hiding the
  third matrix argument (`PSMTXCopy`/`PSMTXConcat`).
- `0x80707F40`'s `+0x3D4`/`+0x3D8`/`+0x3DC`/`+0x3E0` arithmetic is dead; the
  final loads re-read `+0xAC`/`+0xB0`.
- `0x806F1210`'s `0x43300000`/`0x80000010` pair folds to `16.0`.
- `0x806830D0`'s `angle + 0x4000` comes from `addis -1`/`addi 0x4000` truncated
  by `sth`.
- `0x80651A60`-style `_savegpr` `r3` survival applies to the four Ghidra-`void`
  bodies here as well.
- `0x807103D0` is **medium**: the `0x8082F760`/`764`/`774` cells live in
  main.dol and are read at runtime.

## Verification

- `cargo check` succeeds; the ledger holds 5,043 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave195`: 50 rows, 50 unique addresses,
  43 high + 7 medium.
- All 50 evidence exports exist; 13,940 bytes total.
