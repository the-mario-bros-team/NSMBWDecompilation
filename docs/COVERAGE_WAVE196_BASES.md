# Coverage - Wave 196 (`game::bases` 256-300 band, part 2, complete)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 8,152 | 28 | 1 |

The final translatable bodies. After this wave every `game::bases` function at
or above 64 bytes is translated.

## Highlights

- **Lane A (10).** `0x80768470` (nearest `0x275` actor), `0x8076EE60` (rider
  matrix), `0x80774BD0` (route key scan), `0x80782C40` (route-node destructor),
  `0x80786180` (3-state rail hand-over), `0x80790280` (collision-clip refresh),
  `0x80797190` (`W1X0` node init), `0x80799B00` (route-ring neighbour walk,
  `-> i32`), `0x8079E1E0`, `0x8079E550`.
- **Lane B (10).** `0x8079EF20` (egg re-arm), `0x8079F390` (course lookup),
  `0x8079F900` (save commit), `0x807A58E0` (spawn cursor), `0x807AD590`
  (dual-path dispatch / slot tables), `0x807B5D20` (demo-movie light trio),
  `0x807B73D0` (world-map screen ctor), `0x807BA920` (unlock ladder),
  `0x807BAB20` (`0x1A6` item scan), `0x807BB8D0` (cursor step).
- **Lane C (9).** `0x807BDF20` (`worldSelectMapGuide.arc`), `0x807BEAF0`
  (ten-way state switch from jump table `0x8082187C`), `0x807BF5A0`,
  `0x807BFD00`, `0x807C07A0`, `0x807C0C10` (line-crossing solve, returns
  `z - +0x4B8`), `0x807C0EB0`, `0x807C1220` (speed ramp), `0x807C3540`
  (pad-edge poll + `0x900` alloc).

## Call sites wired

`0x80799B00` (4 sites), `0x80797190` (2 sites) now call the real bodies.

## Corrections and recoveries

- **`0x807AD590` was not psq-heavy at all** — the shipped 300 bytes contain no
  `psq_*`, only `_savegpr_27`/`_restgpr_27`, so the old deferral was mistaken.
  Its 16-byte record map matches `fn_8079f790` byte-for-byte (tag `0xFF`).
- `0x807BEAF0`'s ten-way jump table `0x8082187C` was read directly.
- `0x80799B00`'s export is `void`; the real signature is
  `fn_80799b00(this, node: u32, kind: u32) -> i32`.
- `0x80774BD0` and `0x80790280` recover arguments hidden by `_savegpr`.
- `0x807BF5A0`'s GQR lines are register saves, not sample dequantisation.
- `0x807C0C10` returns `f32` (caller `fn_807bffa0` uses `call_f32_1`).

## Completion status

`game::bases` is now complete for every translatable function:

| Class | Count |
|---|---:|
| Translated | 5,072 |
| `0x802DCBFC` forwarders (no shipped target) | 472 |
| Documented deferrals (read unwritten state) | 2 (`0x806E2BF0`, `0x80697090`) |
| Sub-64-byte stragglers remaining | ~69 |

## Verification

- `cargo check` succeeds; the ledger holds 5,072 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave196`: 29 rows, 29 unique addresses,
  28 high + 1 medium.
- All 29 evidence exports exist; 8,152 bytes total.
