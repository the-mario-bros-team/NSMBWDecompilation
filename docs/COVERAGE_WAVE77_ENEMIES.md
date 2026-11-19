# Coverage - Wave 77 (enemies: `0x80A40000` bucket, part 2)

## How to read this file

This page records the Wave 77 `game::enemies` lane: a second pass over the
`0x80A40000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave77-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 3,360 | 14 | 10 |

The bucket holds 189 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 51 were already in
`ledger/functions.csv` (`register_coin_states`, the coin/coin-floor/coin-jugem
families, the `0x8E8`/`0xAE8`/`0xAE0` actor block, `0x80A48390`,
`register_crasher_states` and `alloc_crasher_actor`), leaving 138. This pass
claims 30 and leaves 108. Six of the new rows are `low` confidence - all six
are the 48-byte `0x802DCBFC` forwarders, whose hidden-`r12` this-adjust is not
modelled; the rest are `high` or `medium`.

## What this pass covers

Four short, contiguous runs chosen for shared field layouts.

- **Run 1 (`0x80A401D0..0x80A40740`, seven bodies).** The tail of an enemy
  actor whose state block begins at `+0xA54` and whose model holder is `+0x534`.
  It contributes the holder/motion initialiser, the guard-wrapped idle frame
  step, the ammo-drop step (`+0xA94` counter, per-ammo tables at `0x80B2C610`
  and `0x80B2C61C`, effect `0x191`), the `+0xA54` object lookup that can force
  the state `0x80B708A8` through the `+0xD4` vtable slot, the `+0x60C`/`+0xA86`
  timer step, the ride step over the `+0xA99` slot (position integrator
  `+0xA70`, effect pair `0x80060250`/`0x800E20A0`), and the speed reset that
  picks the `+0x348`-facing entry of `0x80B2C628` and re-arms the motion.
  No class name survives around this run, so the functions are described by
  offset only.
- **Run 2 (`0x80A46D10..0x80A47450`, six bodies).** A second actor: the frame
  pair (`0x8007FBF0` placement of the `+0x534` holder, the seven-word limit
  record at `+0xAC4`/`+0xAD0` installed through `0x8006D090`, the `+0xADC`
  two-record probe, the damped `+0xE8` speed and the `0x800`-incremented
  `+0x102` yaw), its deleting destructor, both 48-byte forwarders, and the
  `0x6A8`-byte constructor (class vtable `0x80B42868`, render holder `+0x524`,
  embedded object `+0x544`, animation holder `+0x584` with `0x80B330E0`, and
  the owner pair `+0x5BC` built from `0x803295D8` then `0x80329518` around the
  `+0x5C8` sub-object).
- **Run 3 (`0x80A4A370..0x80A4A700`, eight bodies).** The five-entry
  state-descriptor skeleton that sits directly in front of the translated
  `alloc_crasher_actor` (`0x80A4A730`): two 88/92-byte deleting destructors,
  two 12-byte constant producers, the `+0x30` owner-chain walker, the
  `:`-suffix name predicate and the two 48-byte forwarders.
- **Run 4 (`0x80A4C7E0..0x80A4CBA0`, nine bodies).** A second, byte-for-byte
  similar skeleton that closes the bucket: two deleting destructors, two
  constant producers, the chain walker, the name predicate, two forwarders and
  a 76-byte constructor that allocates `0x760` bytes and installs the class
  vtable `0x80B4338C` at `+0x60`.

### Identity evidence

- **Runs 3 and 4 return the same two state descriptors.** Both producers
  pairs return `0x80357BF4` and `0x80357BB4`. Those two addresses are returned
  by a dozen unrelated blocks of the export (`0x80A059D0`, `0x80A0D050`,
  `0x80A0EF30`, `0x80A15930`, `0x80A2F740`, `0x80A35BE0`, `0x80A38D50`) and
  are passed as parameters at `0x80A365B0`, so they are shared base-class
  `fState` records. They do **not** name either class, and no name is invented
  for either skeleton.
- **Run 3's position is the only class link.** Its last forwarder
  (`0x80A4A700`) ends 0x30 bytes before `alloc_crasher_actor` (`0x80A4A730`),
  which allocates `0x718` bytes, binds `g3d/crow.brres` and installs
  `0x80B42E70`. The run is therefore described as the crasher family's
  descriptor skeleton and nothing stronger is claimed.
- **Runs 1 and 2 share the `+0x534` model holder** with the already-translated
  coin block (`0x80A42C20`, `0x80A42CC0`) and use the same
  `0x8007FBF0`/`0x80027B00`/`0x800B2F10` sequence, so those field roles carry
  over. The two actors differ in size (state block at `+0xA54` versus a
  `0x6A8`-byte object), so they are not the same class.
- **The 48-byte forwarders** are the same `0x802DCBFC(param_2, param_2,
  param_1)` bodies already documented in this module; their hidden `r12`
  this-adjust is not modelled and each doc says so.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A401D0` | `fn_80a401d0` | 56 | medium |
| `0x80A40220` | `fn_80a40220` | 100 | medium |
| `0x80A40390` | `fn_80a40390` | 236 | medium |
| `0x80A404A0` | `fn_80a404a0` | 84 | high |
| `0x80A40530` | `fn_80a40530` | 152 | medium |
| `0x80A40630` | `fn_80a40630` | 272 | medium |
| `0x80A40740` | `fn_80a40740` | 124 | high |
| `0x80A46D10` | `fn_80a46d10` | 208 | high |
| `0x80A46DF0` | `fn_80a46df0` | 364 | high |
| `0x80A472D0` | `fn_80a472d0` | 88 | high |
| `0x80A473F0` | `fn_80a473f0` | 48 | low |
| `0x80A47420` | `fn_80a47420` | 48 | low |
| `0x80A47450` | `fn_80a47450` | 184 | high |
| `0x80A4A370` | `fn_80a4a370` | 88 | high |
| `0x80A4A3D0` | `fn_80a4a3d0` | 92 | high |
| `0x80A4A430` | `fn_80a4a430` | 12 | high |
| `0x80A4A440` | `fn_80a4a440` | 12 | high |
| `0x80A4A530` | `fn_80a4a530` | 224 | medium |
| `0x80A4A610` | `fn_80a4a610` | 136 | medium |
| `0x80A4A6D0` | `fn_80a4a6d0` | 48 | low |
| `0x80A4A700` | `fn_80a4a700` | 48 | low |
| `0x80A4C7E0` | `fn_80a4c7e0` | 88 | high |
| `0x80A4C840` | `fn_80a4c840` | 92 | high |
| `0x80A4C8A0` | `fn_80a4c8a0` | 12 | high |
| `0x80A4C8B0` | `fn_80a4c8b0` | 12 | high |
| `0x80A4C9A0` | `fn_80a4c9a0` | 224 | medium |
| `0x80A4CA80` | `fn_80a4ca80` | 136 | medium |
| `0x80A4CB40` | `fn_80a4cb40` | 48 | low |
| `0x80A4CB70` | `fn_80a4cb70` | 48 | low |
| `0x80A4CBA0` | `fn_80a4cba0` | 76 | medium |

## Deliberate skips

- **Broken evidence inside the claimed runs.** `0x80A40290` (232 bytes), the
  second body of run 1, is built from `unaff_GQR3` quantiser fiction and
  `ldexpf` calls that are not in the original code, so its two angle writes
  cannot be recovered; it is the one gap inside an otherwise contiguous run.
- **The remaining 16 damaged exports in the bucket.** `0x80A40CA0` (208),
  `0x80A40FA0` (28), `0x80A429C0` (476), `0x80A436C0` (328), `0x80A43810`
  (252), `0x80A443A0` (24), `0x80A46200` (300), `0x80A47B90` (284),
  `0x80A48520` (244), `0x80A48940` (48), `0x80A4B0B0` (136), `0x80A4B140`
  (236), `0x80A4B420` (548), `0x80A4B740` (604) and `0x80A4BBD0` (304) carry
  `unaff_` pseudo-registers or `halt_baddata` truncation. None is claimed.
- **Clean bodies left for the next pass.** The `0x80A4B010..0x80A4BD70`
  object family, the `0x80A4F250`/`0x80A4F4B0` quadrant and mirror helpers,
  the `0x80A4F8E0..0x80A4FC30` skeleton and the `0x80A4C9A0`-style closing
  blocks after `0x80A4D590` were left untouched to keep the lane inside its
  30-body budget. They are ordinary clean exports, so their omission is
  recorded here rather than hidden.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  3,360 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 51 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave77` reports
  `wave77-enemies.csv: 30 rows`, 30 unique addresses and no problem naming
  this CSV (the only reported problem names `wave77-bases.csv`, another lane).
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`; the remaining errors are in `src/runtime/cpp.rs` and
  `src/game/worldmap.rs`, both other lanes.
