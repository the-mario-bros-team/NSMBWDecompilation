# Coverage - Wave 192 (`game::bases` 192-256 band, part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 49 | 11,248 | 42 | 7 | 1 |

First wave in the 192-256 band (~230 bytes per body).

## Highlights

- **Real bodies for long-standing callees.** `0x8064EC70` (called three times
  per branch by `fn_8064eeb0`), `0x80648470` (`(f32, usize)`), `0x8062C800`
  (now called directly by `fn_8062d120`), `0x80613290(this, slot)` and
  `0x80627160(this)`.
- **`d*_c` loaders (4).** `dCollectionCoinDate_c` (`worldCollectionCoinDate_12`),
  `dCourseClear_c`, `dGameOver_c`, `dMessageWindow_c` (`messageWindow.arc`) plus
  `dModelPlayArrow_c` (`modelPlayArrow.arc`).
- **Results machinery (6).** `0x80629A30` (descriptor publish), `0x8062C270`
  (20-pane float copy), `0x8062EA90` (page/row mirror), `0x8062ED90` (label
  resolver), `0x80637610` (record rebind), `0x80637ED0`.
- **Boss/ball bodies (5).** `0x80671510` (`wait_ball_big`), `0x80671A80`
  (`1.2`/`0.8`, step `0.013333334`), `0x80675960` (flag-pole sea pose),
  `0x80678300` (`32.0*h + 8.0` rescale), plus the six `+0x30` chain walkers.
- **Camera/lift bodies (6).** `0x80649090`, `0x80649280`, `0x80649460`,
  `0x8064A9E0`, `0x8064EB50`, `0x8064EC70`.

## Deferred

- **`0x80697090` (248 bytes).** The target `(x, y)` is read from stack slots the
  body never writes (`lfs f2,0x18(r1)` / `0x14(r1)`), and `0x18` is later
  rewritten as `slot - 16.0`. Both the move direction and the `13.0` arrival
  test depend on stale stack contents, and no field or parameter exposes the
  target, so any body would invent a source.

## Corrections and recoveries

- `0x8064EC70`'s exact signature is `fn_8064ec70(this: usize)` with no hidden
  float.
- `0x80648470`'s signature is `(value: f32, this: usize)`; Ghidra's `double` is
  unprototyped-callee noise.
- Six bodies (`0x8062C800`, `0x8062ED90`, `0x8063xxxx`) recover `r3`/`r4` from
  save-thunk artifacts.
- `0x8063C090`'s incoming `r3` is the table base that Ghidra drops.
- `0x807C9110 = 4503601774854144.0` proves the `xoris 0x8000` pair is signed
  int->f32; `0x807C927C` is `60.0f`; `0x80829E98` reads as an empty string.
- `0x8062C800` merged code replaced the placeholder `abi_call3!` in
  `fn_8062d120`.

## Verification

- `cargo check` succeeds with 4,893 defined bodies and no duplicates (anchored
  scan); every wave-192 address except the deferred one is present.
- `node tools/check-lane-csv.mjs wave192`: 49 rows, 49 unique addresses,
  42 high + 7 medium.
- All 49 evidence exports exist; 11,248 bytes total.
