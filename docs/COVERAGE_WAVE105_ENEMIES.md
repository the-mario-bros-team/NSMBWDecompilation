# Coverage - Wave 105 (`game::enemies`, the `0x80A20000` bucket, `daEnBomhei_c`)

## How to read this file

This page records the Wave 105 `game::enemies` lane: 28 functions of the
`0x80A20000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 86 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave105-enemies.csv`) names the Ghidra
C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 6,828 | 16 | 12 |

No row is `low` confidence. The twelve `medium` rows fall into four groups: the
transform and probe helpers whose holder roles come from the identical
`+0x548`/`+0x588` sequence in the already-translated `daEnBomhei_c` binder; the
two contact-record functions whose field roles are taken from the twin
`fn_80ac2a50` of wave 84; the state queries whose `0x80B6Fxxx` records are named
by `register_bomhei_states`; and the placement helpers whose `+0x5F0`/`+0x604`
field names are inferred from their use.

## What this pass covers

- **Identity of the block.** `register_bomhei_states` (`0x80A2A350`, already
  translated in wave 92) names the fourteen `daEnBomhei_c::StateID_*` records.
  `0x80A263C0` loads `g3d/bombhei.brres`, binds the node `bombhei` and the clip
  `walk`, which fixes the class for the whole run. The records used here are
  `0x80B6FB28` (`Walk`), `0x80B6FB68` (`Sleep`), `0x80B6FBA4` (`Carry`),
  `0x80B6FCE8` (`Explode`), `0x80B6FD28` (`Turn`), `0x80B6FD68`/`0x80B6FDA8`
  (both `CannonHop` entries) and `0x80B6FDE8` (`AfterIce`).
- **`0x80A263C0..0x80A26B30`, the model and transform layer.** The
  `0x20`-entry model record at `+0x528`, the model at `+0x544`, the `0x128`
  matrix holder at `+0x548`, the clip holder at `+0x588`, the clamp holder at
  `+0x5C0`, the `+0x14` release, the `+0x7C` transform rebuild (position, then
  yaw/roll/pitch around `(0, ±8, 0)` and `(0, ±16, 0)` translations), the
  `skl_root` probe and the two-endpoint ground probe that writes
  `+0x164..+0x170`.
- **`0x80A277A0..0x80A28030`, the contact layer.** The pair-contact record
  builder (`+0x414..+0x432`, the `0x80026C00` target branch or sound `0x17B`) is
  the `daEnBomhei_c` twin of wave 84's `fn_80ac2a50`; the partner grab forces
  `0x80358004` (`InIceLump`) with the `+0x5EC` countdown at `-1`; the
  `Explode` request plays sound `0x194`; and the responder separates the
  `0x6A`/`0x6B` fireball partners (timer `0xF0`, `Wm_mr_fireball_hit`) from the
  `0x200`-flag branch that marks `+0xA1 |= 2` and switches to `Explode`.
- **`0x80A28170..0x80A285D0`, the placement layer.** The `0x8006D090` probe
  record re-seat, the fixed `Carry` record built from the constant block at
  `0x80B2B9B0` (`0`, `3.0`, `-4.0`, `-0.1875`, `0x80357BF4`, `-1`, `-1`), the
  two `0x800768D0` footprints, the nine-cell grid offset
  (`0x80B2B9E8`/`0x80B2BA0C` with the 16-unit snap) and the `stop` binder that
  also writes `+0xB4 = 1500.0 + 16 * ((spawn >> 24) & 0xF)`.
- **`0x80A28760..0x80A28B60`, the state steps.** The `Turn` pair (the
  `0x15 << side` button mask, the `±0x400` yaw ramp and the `0x8042C1A4`
  clamp), the `stop` rebinds, the first `Explode` step with the
  `Wm_en_bombignition` effect and the `+0x5EC` countdown, and the `Carry`
  entry that copies the `0x8042BB18` blend byte and saves
  `+0x164..+0x170` into `+0x610..+0x61C`.
- **`0x80A28F20..0x80A295D0`, the remaining steps.** The `Carry` frame step
  with the `0.0625` word at `+0x11C`, the two `Explode` frame steps (the second
  one adds the `Wm_en_landsmoke_s` puff and the `0.5` mirrored x speed), the
  `CannonHop` entry/release/air trio and the `Wakidashi` entry/release/two-phase
  wait on `+0x360`.

### Deliberate skips

- **`0x80A281C0` (32 bytes).** The export is a truncated
  `halt_baddata` stub ("Control flow encountered bad instruction data"), so no
  body can be reconstructed. Its only caller inside this run,
  [`fn_80a29010`], keeps the call by address with an `// unknown:` note, exactly
  as the earlier waves handled unresolvable callees.
- **The four functions before the block.** `0x80A25F00` is a 48-byte
  `0x802DCBFC` forwarder, but it belongs to the `0x540`-byte actor that
  `0x80A25F30` allocates, not to `daEnBomhei_c`; it was left to a later pass
  together with `0x80A20810`, `0x80A20900`, `0x80A22820` and `0x80A22B20`.
- **The remaining functions of the bucket.** 54 of the bucket's 171 functions
  are still untranslated after this pass. The obvious next candidates are the
  rest of this same class (`0x80A296E0..0x80A2A180`), then the
  `0x80A2B490..0x80A2C2A0` pair-contact/impact cluster, the
  `0x80A2CDE0..0x80A2F170` bounce-ball support objects and the
  `0x80A2F670..0x80A2FD40` tail.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A263C0` | `fn_80a263c0` | 356 | high |
| `0x80A26740` | `fn_80a26740` | 48 | high |
| `0x80A26770` | `fn_80a26770` | 496 | medium |
| `0x80A26AC0` | `fn_80a26ac0` | 112 | medium |
| `0x80A26B30` | `fn_80a26b30` | 356 | medium |
| `0x80A277A0` | `fn_80a277a0` | 440 | medium |
| `0x80A27AE0` | `fn_80a27ae0` | 292 | medium |
| `0x80A27D90` | `fn_80a27d90` | 184 | medium |
| `0x80A28030` | `fn_80a28030` | 316 | medium |
| `0x80A28170` | `fn_80a28170` | 80 | high |
| `0x80A281E0` | `fn_80a281e0` | 224 | high |
| `0x80A282C0` | `fn_80a282c0` | 244 | medium |
| `0x80A284D0` | `fn_80a284d0` | 188 | medium |
| `0x80A285D0` | `fn_80a285d0` | 380 | medium |
| `0x80A28760` | `fn_80a28760` | 204 | high |
| `0x80A28860` | `fn_80a28860` | 280 | high |
| `0x80A28980` | `fn_80a28980` | 152 | high |
| `0x80A28A30` | `fn_80a28a30` | 304 | high |
| `0x80A28B60` | `fn_80a28b60` | 260 | medium |
| `0x80A28F20` | `fn_80a28f20` | 228 | high |
| `0x80A29010` | `fn_80a29010` | 76 | medium |
| `0x80A29060` | `fn_80a29060` | 476 | high |
| `0x80A29240` | `fn_80a29240` | 132 | high |
| `0x80A292D0` | `fn_80a292d0` | 52 | high |
| `0x80A29310` | `fn_80a29310` | 484 | high |
| `0x80A29500` | `fn_80a29500` | 140 | high |
| `0x80A29590` | `fn_80a29590` | 52 | high |
| `0x80A295D0` | `fn_80a295d0` | 272 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  6,828 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave105` reports
  `wave105-enemies.csv: 28 rows` and `OK` with no problem naming this CSV.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`. The remaining errors at the time of this pass belong
  to other lanes' files.
- The module header was updated: a new cluster item 86, the cluster count
  (eighty-five to eighty-six), the function totals (2,274 to 2,302 public
  translations, 2,203 to 2,231 `fn_<addr>` bodies) and the record list.
