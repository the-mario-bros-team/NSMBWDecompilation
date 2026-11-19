# Coverage - Wave 187 (`game::bases` 128-192 band, part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 8,016 | 48 | 2 |

First wave in the 128-192 band (the 64-128 band is now down to three
stragglers, folded into this selection).

## Highlights

- **Layout/animation families.** `0x80600D40` (`charaChangeSelectArrow_02`
  loader), `0x806010F0`, `0x80624880`/`0x80624A20`/`0x80624AD0`/`0x80624BE0`
  (out-animation swaps), `0x80627240` (model-play-date restart),
  `0x80627A80` (`modelPlayGuide`).
- **State descriptors.** `0x80613530`, `0x806148F0`, `0x806296A0`
  (KeyWait / CoinCountDown / WipeWait), `0x8063F060`, `0x8063D4E0`,
  `0x80643260`.
- **Lift helpers.** `0x80649190`, `0x80649380`, `0x80649560`.
- **Actor bodies.** `0x8064A150` (`block_arrow`, vtable `0x807E39AC`),
  `0x8064A820`, `0x80656610`, `0x80656EB0`, `0x80658080`
  (`launch_bomb` clip, `+0x500 = 800`).
- **Pane/save helpers.** `0x8063A680`, `0x8063A8C0`, `0x8063ABE0`,
  `0x80638AD0`, `0x80628810` (8x13 save grid), `0x806288B0`.

## Recoveries and corrections

- `0x806148F0`'s export truncates with "bad instruction data"; rebuilt from
  `0x806148F0..0x8061497C` - the rotate sequence is a sign test and
  `bclr 4,1` gives the `<= 0x384`/`<= 0x960` returns.
- `0x80643260`'s `halt_baddata` is a `b 0x80195380` tail call.
- `0x806245F0` returns `f32`, not the C's `double` (GQR0 `lfs` + `fmr f1,f31`).
- `0x8063A680` and `0x80638680` regain `this` from `_savegpr` thunks that
  preserve `r3`.
- `0x8064DA10` computes `A*cos - B*sin` / `A*sin + B*cos` with `A = 0.0`,
  `B = 8.0`, angle scaled by `1/256`.
- `0x80649380`'s `+0x1A4EE` is `u16`, not Ghidra's `(short)`.
- `0x8062D120`'s `void` export hides a consumed `r3` (recovered `screen`), so it
  is **medium**.
- `0x8063A8C0` is **medium**: the three `rlwimi` inserts are folded to
  `>>0/1/2 & 1`; the grouping tables fix member order but shift counts were not
  force-decoded bit-for-bit.
- `0x80613290`, `0x80627160`, `0x8062C800` have no Rust bodies yet and go
  through `abi_call*!`.

## Verification

- `cargo check` succeeds with 4,668 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave187`: 50 rows, 50 unique addresses.
- All 50 evidence exports exist; 8,016 bytes total.
