# Coverage - Wave 211 (`game::player`, main.dol part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 60 | 9,748 | 48 | 12 |

Second 60 `game::player` bodies, `0x80063200`-`0x80066630`.

## What landed

- **Model rebuild and anchor drive (lane A).** `0x80063200` (despawn/track
  gate), the **932-byte** `0x80063270` (main model + `count-2` parts + cap along
  the heading), `0x800636E0`, `0x80063820`, `0x80063880`, `0x80063890`,
  `0x80063900`, `0x80063960`, `0x80063AC0`, `0x80063E00`.
- **Track/step bodies (lane B).** `0x80063F20`, `0x80063F50`, `0x80063F80`,
  `0x80064030`, `0x80064110`, `0x800642B0`, the 4-byte `0x80064380`,
  `0x800643F0`, `0x80064490`, `0x80064540`.
- **Spawn-policy hooks (lane C).** `0x80064600` (8-byte SDA tag setter),
  `0x80064610`, `0x80064680`, `0x800646E0`, `0x80064720`, `0x80064740`,
  `0x80064790` (12-byte `bcctr` tail jump), `0x800647A0`, `0x80064A10`,
  `0x80064A30` (float-relay tail jump).
- **Hit tests and local thunks (lane D).** `0x80064A50`, `0x80064AF0`,
  `0x80064B00`, four 8-byte `li r4,0; b` forwarders (`0x80064BD0`-`0x80064C00`),
  `0x80064D10`, `0x80064DA0`, `0x80064F50`.
- **Cull and cue bodies (lane E).** `0x80065080`, `0x80065110`, the **700-byte**
  `0x800651C0`, `0x80065480`, `0x80065A00`, `0x80065A40`, `0x80065AC0`,
  `0x80065B80`, `0x80065CB0`, `0x80065FC0`.
- **Spawn-list and stage bodies (lane F).** `0x80066120`, `0x800661C0`,
  `0x800663E0`, `0x80066420`, `0x80066490`, `0x800664F0`, `0x80066540`,
  `0x80066570`, `0x800665D0`, the **272-byte** `0x80066630`.

## Corrections and recoveries

- **Queue rebuilt from `main-gb`.** The per-file inventory had been derived from
  the smaller `main` export, so 52 canonical functions were never queued,
  including two `ui::hud` bodies (`0x80026CA0`, `0x8002D260`).
  `work/rebuild-missing.cjs` now re-derives it: dsys 248, player 233,
  archive 198, stage 132, hud 2.
- **Size corrections** (the brief's values came from the `main` export's
  prologue artefacts): `0x80063270` = 932 (not 20), `0x800651C0` = 700 (not 16),
  `0x80066630` = 272 (not 28), `0x80064A50` = 156 (not 16).
- `0x80064030`: Ghidra swaps the inequality operands. The first site is
  `fcmpo` + `cror 2,0,2` (`target <= current`) + `bc 4,2`, so the `fsubs`
  arm runs only when `target > current`; the second is `cror 2,1,2`
  (`value >= current`) + `bc 4,2`. The body follows the instructions.
- `0x80064380` is a 4-byte `b` beginning a three-hop chain
  (`0x8006C8A0 -> 0x8006C570 -> 0x80161DA0`); translated as a forwarder to the
  real target.
- `0x80064A50` is genuinely asymmetric: `rlwinm 1,31,31` extracts `LT` and
  shift 2 extracts `GT`, so the compare is `diff > tol` when `diff >= 0` and
  `diff >= -tol` otherwise.
- `0x80065080` returns `u32` although Ghidra says `void`; the `0x800B55F0` cull
  result stays in `r3`.
- `0x80065A00` is medium: the `xoris`/`0x4330` pair is the unsigned-to-double
  bias, but the machine reads the double's high word with `fsubs`/`fdivs`, which
  literally collapses to `0.0`; the body keeps the only reading in which the
  `0.8` step exists.
- `0x800661C0` has a real fifth parameter in `r7` that the export dropped.
- `0x800665D0` returns `f32`; the export's `double` is the `0x43300000` bias
  pair.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave211`: 60 rows, 60 unique, 48 high + 12 medium.
- Ledger: 19,497 rows (96.27%); `game::player` 657/818 (80.32%).
- `scripts/check.ps1` exit 0.

## Next

161 `game::player` bodies remain, then `src/engine/dsys.rs` (248),
`src/resource/archive.rs` (198), `src/game/stage.rs` (132) and the two
`ui::hud` stragglers.
