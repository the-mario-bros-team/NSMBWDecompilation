# Coverage - Wave 194 (`game::bases` 192-256 band, part 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 11,132 | 43 | 7 |

Third wave in the 192-256 band.

## Highlights

- **Eight call sites wired to new bodies.** `0x8075FA40` (`(this, f32, f32)`),
  `0x80789990` (2 sites, `(this, mode, point, extra)`), `0x80798E20` (2 sites,
  `(this, mode)`) and `0x8075BE90` (2 sites, `(this, first, second)`).
- **Director bodies (5).** `0x8078DB80`, `0x8078ED10`, `0x8078EDF0`,
  `0x8078F350`, `0x8078FB80`.
- **Ride/rail bodies (6).** `0x8071E060`, `0x8071EEE0`, `0x80730360`,
  `0x80734240`, `0x80734B00`, `0x8074BB00`.
- **Transform/bounds bodies (8).** `0x8073AF20`, `0x8073C730`, `0x8073F810`,
  `0x80740A60`, `0x8074EFF0`, `0x80752AC0`, `0x807537D0`, `0x80783300`.
- **Chain walkers (5).** `0x807203D0`, `0x80722700`, `0x80723A30`,
  `0x807433A0`, `0x8076E490` - `0x80722700`/`0x80723A30` are byte-identical to
  `0x807203D0`.
- **The rest** are factories, destructors and small state helpers.

## Corrections and recoveries

- **Wave-101 deferrals retired.** `0x807382F0`/`0x80738DB0` were clean; their
  `unaff_GQR0`/`ldexpf` output is GQR0/`_savegpr` noise.
- `0x807436C0`'s `0x8042BB0x` operands are really `0x8043BB08/0x8043BB0C`
  (main.dol floats kept as runtime reads); its four parameter `stfs` are dead.
- `0x807538E0`'s guard is `(+0x358 | +0x35C) == 0`; the export's masked double
  test is wrong (the `0x8042A078` loads are dead).
- `0x8074BB00` needs `0x80064610`'s return register, which the void wrapper
  `fn_8074b880` discards, so the call is emitted inline to keep the
  `cmpwi`/`beq` gate.
- `fn_80774d00`/`fn_80775730` return `u32` despite Ghidra's `void` (tested as
  booleans at `0x80772370`/`0x807953B8`).
- `fn_80762fe0`, `fn_80774d00`, `fn_80775730`, `fn_80783300` take `this` from
  an `r3` preserved by `_savegpr_27`.
- `0x8078FB80` keeps `abi_call1!(0x807900B0, this)` because the existing
  `fn_807900b0()` wrapper recovers `r3` through `_savegpr_24`, so a bare Rust
  call would drop the argument.
- `0x8074A4F0`'s shift is `-160.0` when `0x8005FEF0 > 1`, else `0.0`.
- `0x8075BE90`'s `r3` is never read (overwritten after `_savegpr_24`); only
  `first`/`second` reach `0x800EF570`.

## Verification

- `cargo check` succeeds; the ledger holds 4,993 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave194`: 50 rows, 50 unique addresses,
  43 high + 7 medium.
- All 50 evidence exports exist; 11,132 bytes total.
