# Coverage - Wave 100 (`game::worldmap`, `0x80110000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); every `r2` float used below was resolved against
`work/dol/main.elf` and is spelled as a `W100_R2_*` constant that names the
value.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 25 | 10,928 | 4 | 21 |

Every row is `translated`; no body is `stub`, `partial` or `low` confidence.

## What this pass covers

- **`0x80110B10..0x80112F50` - rail geometry and contact.** `0x80110B10` is the
  `[0, 8192)` box gate that forwards the `+0x38F` ground kind to `0x800767F0`.
  `0x80110CA0` snapshots the actor position into the controller through
  `0x801117A0` once the `+0x38F`/`+0x821`/`+0x10D8` gate opens, then clamps the
  X to the collider entry `- 16.0`. `0x80111F00` projects `+0xAC` with
  `0x80082240` and returns the camera X while the projection stays inside the
  scene's `+0x40` half-width. `0x80112DB0`/`0x80112E80` are the narrow and wide
  rail boxes (`24`/`16` and `32`/`48` margins).
- **`0x801121B0` - the collision sweep.** Saves the twelve box words
  `+0x75C..+0x788` and the position, installs a `dx`/`dz << 12` box, runs the
  ceiling/floor/left/right probes through `0x8006DB30`, `0x8006EF40` and
  `0x8006FA40`, classifies each with `0x80070720`/`0x80070730`/`0x80070740`,
  and restores everything. `mask` disables individual probes.
- **`0x80112810..0x80113D90` - side and steering selectors.**
  `0x80112810` folds `0.5 * scene[+0x44] * 0.125` into two `6.0`/`7.0` bands,
  toggles `+0x81E`, seeds `+0x7CC` with `+-0.4` and stores `100`/`0x96` in
  `+0x502`. `0x801129C0` uses the same band shape on `0.5 * scene[+0x40] / 6`
  and writes `+0x10C`. `0x80112C10` clamps the camera/rail offset to `+-7.0`,
  snaps `+0xAC` to either rail edge and forces `+-0.4` when the offset is small.
  `0x80113AE0` applies the per-side clamp and the `+0xF4`/`+0xF0` refresh, and
  `0x80113D90` re-enables layout channel `+0x148` and aims `+0x10C`.
- **`0x80112F50` - lane follower.** Steps the `+0xE8`/`+0x10C` and
  `+0xEC`/`+0x7CC` pairs toward the probed actor with `0.45`/`1.8`/`0.3`/`0.9`,
  or `0.63`/`2.52`/`0.42`/`1.26` when bit `0x40` of `0x80429F80` is set.
- **`0x80113790..0x80114050` - checkpoint, lane timer and landing.**
  `0x80113790` copies the position to `+0x798`, arms `+0x500`, seeds the speed
  pair, takes the lane index from `+0x110` (wrapping at four) into `+0x7F0` and
  derives `+0x348` from camera `+0x90079`. `0x80113950` runs the variant-`2`
  clamp, and when the timer expires pushes the actor `32.0` off the rail and
  posts `0x80375148`. `0x80114050` caches `+0x884`, folds the `0x80113260`,
  `0x80112B30` and `0x80112C10` results, sweeps with `0x801121B0`, then either
  commits the contact (balloon burst) or posts `0x80375188`.
- **`0x80116D20..0x801175D0` - grid row builders.** Six shapes of the same
  cell writer: the array at `object+0x3B8` holds `0x10`-byte cells in
  `0x420`-byte rows. X comes from `0x80116C00`, Y from the `0x80116B10` lift
  plus/minus the row pitch, Z from `-7000.0` (variant `0`) or `-8000.0` minus
  the `0.1` inset. The flag word is `0`, `1` or `2` depending on the row.
- **`0x80118450` - quad rebuilder.** Uses `0x80119140`'s scene centre, the
  mode-dependent X (`camera +0x8FE64`, the `+0x4E8` accumulation, or the mode-2
  spacing loop), the `0x80116BB0` side offset, and writes three records of two
  quads each with the `+0x438..+0x448` weights, folding X back inside
  `720 + 100` in mode two.
- **`0x80118DC0` - tile animation walk.** 3 x 9 records at `+0x3C4`: binds the
  parent through `0x80168FE0`/`0x801680E0`, advances `+0x638` by `+0x634` and
  clamps against `0x80167B80`, then closes each record with the `+0x14`
  virtuals. The `+0x430` record follows.
- **`0x8011C310`/`0x8011FCF0` - ice/grid object.** `0x8011C310` is the
  constructor: `+0x38F` copy, `0x8011D450`, the `0x802F4CB0` collision shape with
  the twelve-argument `0x8007FA40`, the `+0x164..+0x170` extents, the
  `+0x544..+0x550` frame seeds, the `+0x530` flags and the `0x8042BB18`
  name-kind byte. `0x8011FCF0` is its ground contact, driving `+0x5F4` and
  posting `0x803757B8`/`0x80375738`/`0x803757F8`.
- **Shared notation.** The `W100_R2_*` constants are the small-data floats
  resolved from `main.elf` (for example `0x5CF8` = `6.0`, `0x5CE8` = `8192.0`,
  `0x5C08` = the `0x4330000080000000` bias double). `W100_SDA_637C_0_1` is the
  `0.1` grid inset, and the `0x802DCD48..0x802DCD68` helpers are CodeWarrior
  register-save prologues, not calls, so the bodies take their arguments
  directly. The `ldexpf` blocks are paired-single save/restore noise and are not
  reproduced.
- Already-translated neighbours are reused by name instead of by address:
  [`fn_801117a0`], [`fn_80112080`], [`fn_80112440`], [`fn_80112b30`],
  [`fn_80113260`], [`fn_80113470`], [`fn_80113500`], [`fn_80113600`],
  [`fn_80116b10`], [`fn_80116bb0`], [`fn_80116c00`], [`fn_80119140`],
  [`fn_8011d450`], [`fn_8011dce0`], [`fn_8011ee80`],
  [`wm_balloon_burst_effect`] and [`fn_801474f0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80110B10` | `fn_80110b10` | 72 | medium |
| `0x80110CA0` | `fn_80110ca0` | 316 | medium |
| `0x80111F00` | `fn_80111f00` | 136 | medium |
| `0x801121B0` | `fn_801121b0` | 652 | medium |
| `0x80112810` | `fn_80112810` | 428 | medium |
| `0x801129C0` | `fn_801129c0` | 356 | medium |
| `0x80112C10` | `fn_80112c10` | 408 | medium |
| `0x80112DB0` | `fn_80112db0` | 200 | medium |
| `0x80112E80` | `fn_80112e80` | 204 | medium |
| `0x80112F50` | `fn_80112f50` | 776 | high |
| `0x80113790` | `fn_80113790` | 424 | high |
| `0x80113950` | `fn_80113950` | 348 | high |
| `0x80113AE0` | `fn_80113ae0` | 412 | medium |
| `0x80113D90` | `fn_80113d90` | 156 | high |
| `0x80114050` | `fn_80114050` | 736 | medium |
| `0x80116D20` | `fn_80116d20` | 452 | medium |
| `0x80116EF0` | `fn_80116ef0` | 440 | medium |
| `0x801170B0` | `fn_801170b0` | 432 | medium |
| `0x80117260` | `fn_80117260` | 432 | medium |
| `0x80117410` | `fn_80117410` | 440 | medium |
| `0x801175D0` | `fn_801175d0` | 480 | medium |
| `0x80118450` | `fn_80118450` | 768 | medium |
| `0x80118DC0` | `fn_80118dc0` | 664 | medium |
| `0x8011C310` | `fn_8011c310` | 788 | medium |
| `0x8011FCF0` | `fn_8011fcf0` | 408 | medium |

## Deliberate skips

The bucket had 45 untranslated entries at the start of the pass. Twenty-five
were written; the remaining twenty are listed here.

- **Paired-single / GQR records: `0x80110B60` (312), `0x80110FA0` (528),
  `0x80111850` (1,044), `0x80117DC0` (1,512), `0x80119240` (608),
  `0x801194A0` (1,960), `0x80119C50` (1,200), `0x8011C750` (348),
  `0x8011C8B0` (472), `0x8011CA90` (484), `0x8011D930` (236),
  `0x8011DAB0` (252), `0x8011EC10` (248), `0x8011F1D0` (268).** These exports
  quantise incoming paired-single registers inside the body
  (`CONCAT13`/`CONCAT22` over `GQR0`), so the operands cannot be recovered from
  the C alone. `0x8011DAB0` is still called by raw address from
  [`fn_8011fcf0`]; `0x8011CA90` is still called by raw address from
  `0x8011C190`.
- **`0x801126A0` (356).** The export reads `0x8005F900` with no argument in one
  branch and hands the `0x802F4B20` probe table to `0x801121B0`; the probe
  offsets and the player index are not recoverable without the table contents.
- **`0x80116430` (1,280).** The effect/resource loader opens an archive, builds
  four `0x8023xxxx` iterators and allocates `0x80303408` records; the call
  sequence is literal but the handle roles are not, so no body was written.
- **`0x8011Axxx`/`0x8011Bxxx`/`0x8011Dxxx` neighbours** listed above were
  already translated by waves 76, 89 and 94 (or are the same GQR records those
  waves skipped), and no address is duplicated here.
- `0x8011C100` already had a body in `src/game/worldmap.rs` from wave 94 and is
  not duplicated; it has no ledger row, so no row was created for it.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the 25 sizes sum to 10,928 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 25 were untranslated
  entries of the `0x80110000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error naming
  `src/game/worldmap.rs`. The remaining crate errors at the end of the pass were
  in `src/render/gx_band.rs` and `src/resource/bres.rs` (other parallel lanes).
- `node tools/check-lane-csv.mjs wave100` reports `OK`.

[`fn_801117a0`]: crate::game::worldmap::fn_801117a0
[`fn_80112080`]: crate::game::worldmap::fn_80112080
[`fn_80112440`]: crate::game::worldmap::fn_80112440
[`fn_80112b30`]: crate::game::worldmap::fn_80112b30
[`fn_80113260`]: crate::game::worldmap::fn_80113260
[`fn_80113470`]: crate::game::worldmap::fn_80113470
[`fn_80113500`]: crate::game::worldmap::fn_80113500
[`fn_80113600`]: crate::game::worldmap::fn_80113600
[`fn_80116b10`]: crate::game::worldmap::fn_80116b10
[`fn_80116bb0`]: crate::game::worldmap::fn_80116bb0
[`fn_80116c00`]: crate::game::worldmap::fn_80116c00
[`fn_80119140`]: crate::game::worldmap::fn_80119140
[`fn_8011d450`]: crate::game::worldmap::fn_8011d450
[`fn_8011dce0`]: crate::game::worldmap::fn_8011dce0
[`fn_8011ee80`]: crate::game::worldmap::fn_8011ee80
[`wm_balloon_burst_effect`]: crate::game::worldmap::wm_balloon_burst_effect
[`fn_801474f0`]: crate::game::worldmap::fn_801474f0
