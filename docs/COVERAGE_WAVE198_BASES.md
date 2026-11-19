# Coverage - Wave 198 (`game::bases` very large bodies, part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 15 | 17,600 | 9 | 6 |

First wave of the very large bodies (516-3,788 bytes). All fifteen merged,
including lane B after its block was re-verified.

## Merged

- **Lane A (3, 5,468 bytes).** `0x80605BA0` (pane-bank visibility),
  `0x8061F2F0` (winner panes), and the 3,788-byte `0x8065CB20`
  (`(this, max_out, out, x, y, radius)`) which samples the `(2n+1)^2` stage
  grid, marks radius hits, repairs edge bits, picks variant words from
  `0x807E703C`/`0x807E7240`, paints both layers and emits 12-byte spawn
  triples.
- **Lane B (3, 6,976 bytes).** `0x8065DD50`, `0x8065F090`, `0x80661070`.
  The lane's first pass carried ~20 mechanical `u32`/`usize` errors and was
  un-merged; after fixes the block verified clean in a scratch `cargo check`.
  The fixes included a real behavioural one: the timer gates read `+0x3D0`
  and `+0x9CCC` as `u32`, but the raw words use signed `cmp`, so `timer >= 0`
  was always true and the shipped `-1` sentinel semantics were lost. They are
  now `read_i32`.
- **Lane C (3, 2,668 bytes).** `0x806A24F0` (1,624-byte `Wm_jr` pair
  separation), `0x806D5FB0` (nine-segment ridden trail), `0x806D62C0`
  (eight `48.0` bands, class `1`/`2` bucketing).
- **Lane D (3, 1,584 bytes).** `0x806D6680` (number-panel redraw),
  `0x806D6950` (rider-tree walk), `0x806E0110` (`0x760`-byte flipper frame).
- **Lane E (3, 1,556 bytes).** `0x806F2100` / `0x806F2710` (byte-identical
  ring collectors) and `0x80702FD0` (boat-light setup).

## Corrections and recoveries

- `0x8065CB20`'s real signature is `(this, max_out: i32, out, x, y, radius)`;
  Ghidra's `iVar11` is a `_savegpr_15` artifact and the array base is `r3`.
- `0x806A24F0`'s `in_f30`/`in_f31` and GQR0 blocks are `stfd`/`psq_st` noise,
  so the signature is `(this, other)`.
- `0x806D63D0`'s `mfcr`/`rlwinm r0,r0,1,31,31` ladder extracts LT and its
  `.`-form sets EQ for the following branch; all `fcmpo` folds match Ghidra.
- `0x806D5FB0`'s node table (`02 04 05 07 08 0A 0B`) was read from the ELF.
- `0x806D62C0`'s `lbz`/`extsb`/`cmpli` id test equals `read_u8(id) < 4`.
- `0x806F2100`/`0x806F2710`'s int->f32 bias is `0x4330000080000000`
  (`2^52+2^31`, signed), not plain `2^52`.
- Pane ids `4..7` are read at runtime from `0x807CC57D..0x807CC580`.
- Lane B's timer gates needed `read_i32` (signed `cmp`), fixing an
  always-true comparison.

## Verification

- `cargo check` succeeds; the ledger holds 5,122 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave198`: 15 rows, 15 unique addresses,
  9 high + 6 medium (the "already in ledger" notes are post-merge only).
- All 15 evidence exports exist; 17,600 bytes translated in this wave.
