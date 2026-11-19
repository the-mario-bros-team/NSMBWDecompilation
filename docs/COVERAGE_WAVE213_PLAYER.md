# Coverage - Wave 213 (`game::player`, main.dol part 4)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 60 | 8,252 | 53 | 7 |

Fourth 60 `game::player` bodies, `0x800693E0`-`0x8006C870`.

## What landed

- **Nearest-player and masks (lane A).** `0x800693E0`, `0x80069510`,
  `0x80069520`, `0x80069530`, four 8-byte tail branches (`0x80069580`,
  `0x80069590`, `0x800695F0`, `0x80069600`), `0x80069610`, `0x80069720`.
- **Effect records (lane B).** `0x80069820`, `0x80069A00`, `0x80069B70`,
  `0x80069C50`, `0x80069CF0`, `0x80069F30`, `0x8006A110`, `0x8006A130`,
  `0x8006A1C0`, `0x8006A1F0`.
- **SDA accessors and rect maths (lane C).** `0x8006A200`, six 8-byte
  setters/getters (`0x8006A230`-`0x8006A270`), `0x8006A280`, `0x8006A2B0`,
  `0x8006A320`, `0x8006A3F0`.
- **Projection and request machine (lane D).** `0x8006A4B0`, `0x8006A5E0`,
  `0x8006A620`, `0x8006A6A0`, `0x8006A6F0`, `0x8006A730`, `0x8006A760`,
  `0x8006A7D0`, `0x8006A7F0`, `0x8006A830`.
- **Sound-slot helpers (lane E).** `0x8006A9A0`, `0x8006C0E0`, `0x8006C120`,
  `0x8006C420`, `0x8006C490`, `0x8006C500`, `0x8006C540`, `0x8006C570`,
  `0x8006C580`, `0x8006C5B0`.
- **`Base_c` family (lane F).** `0x8006C5C0`, `0x8006C600`, `0x8006C610`,
  `0x8006C660`, `0x8006C690`, `0x8006C6B0`, `0x8006C6C0`, `0x8006C6D0`,
  `0x8006C7F0`, `0x8006C870`.

## Corrections and recoveries

- **Branch polarity.** `0x800693E0`'s `fcmpu`/`cror 2,0,2` + `bc 4,2` is
  `bne` (BO=4 branches on a *clear* CR bit), so the nearest-player update runs
  on `distance <= bound`. Reading it as "branch if equal" inverts the search.
  Cross-checked against the `bc 4,1` (`ble`) in the already-translated
  `0x80069060`.
- **`f32` versus `f64` helpers.** `0x80082240` is `(f32,u32)->f32` and
  `0x800B30E0` is `()->f32`; a wave-98 helper models them as `f64`, so lane C
  added correct `f32` thunks rather than reusing it.
- `0x8006A4B0` is a world-to-screen projection, not the two-word record builder
  its earlier call sites suggested.
- `0x8006A620` -> `0x8006A6A0` forwards `r3` from `0x80069610`, and
  `0x8006A760` -> `0x8016B7E0` receives `r3 = handle`; Ghidra omits both.
- `0x8006C660` returns the class descriptor `0x8030F1B8`, whose block holds the
  Shift-JIS name `その他(dBase_c継承の非アクター)`; the base vtable is
  `0x8030F1D8` (slots `+0x0C/+0x10/+0x18/+0x1C/+0x48/+0x4C`) and the derived one
  `0x8030F228` (`+0xC/+0x48/+0x4C`).
- `0x8006C570`'s raw word `0x480F5830` confirms the wave-211 chain
  `0x8006C8A0 -> 0x8006C570 -> 0x80161DA0`; `0x8006C5B0` is `b 0x80161F60`.
- `0x80069610`'s self-branch (`b 0x80069718`) is kept as a literal spin loop for
  kinds 1, 2 and >`0x2C7` rather than inventing returns.
- `0x8006A7D0`/`0x8006A7F0` reproduce PowerPC `slw` semantics (low 6 bits,
  >=32 yields 0).

## Merge note

Lane F's block contained eight bodies although its CSV listed ten: the two
4-byte thunks `0x8006C6B0` and `0x8006C6C0` were missing. They were decoded from
the raw words (`0x480F6450` -> `b 0x80162B00`, `0x480F6460` -> `b 0x80162B20`)
and written by the coordinator against the already-translated
`resource::registry::fn_80162a70`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave213`: 60 rows, 60 unique, 53 high + 7 medium.
- Ledger: 19,617 rows (96.86%); `game::player` 777/818 (94.99%).
- `scripts/check.ps1` exit 0.

## Next

53 `game::player` bodies remain, then `src/engine/dsys.rs` (248),
`src/resource/archive.rs` (198), `src/game/stage.rs` (132) and the two
`ui::hud` stragglers.
