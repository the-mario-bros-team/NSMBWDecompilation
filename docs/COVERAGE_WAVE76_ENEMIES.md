# Coverage - Wave 76 (enemies: `0x80A20000` bucket, part 2)

## How to read this file

This page records the Wave 76 `game::enemies` lane: the second pass over the
`0x80A20000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave76-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Float
constants quoted below were read from the REL image
(`work/rels/elf/d_enemiesNP.elf`) at the cited synthetic address, not guessed
from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,008 | 14 | 16 |

The bucket holds 171 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 33 were already in
`ledger/functions.csv` (the wave-58 opening plus `register_bomhei_states`,
`register_bounce_ball_states` and `alloc_bounce_ball_actor`), leaving 138.
This pass claims 30 and leaves 108.

## What this pass covers

The claimed bodies form four runs chosen for a shared shape: each fronts the
five-entry skeleton (deleting destructor, two 48-byte `0x802DCBFC` forwarders,
allocating constructor) that the wave-58 block already established, so the
field roles carry over between runs.

- **Block A (`0x80A23180..0x80A24040`, eleven bodies).** A `0x700`-byte actor,
  vtable `0x80B39D9C`, inferred `daEnBlockSwich_c`. It contributes the third
  deleting destructor of the family, its two forwarders, the constructor (two
  holders `+0x534`/`+0x698`, child word `+0x6BC`), the per-frame step that
  seats the model and retires the actor, the release step that pops the
  child's render request from `+0x698`, and the costume effect that raises
  `Wm_en_burst_s` with effect id `0x80B2B7F0[+0x6F8]` and sound `0x289`. The
  run ends with the second destructor/forwarder/constructor quartet, whose
  constructor allocates `0x720` bytes and installs vtable `0x80B3A154`.
- **Block B (`0x80A24490..0x80A248E0`, five bodies).** The `0x720`-byte
  sibling (inferred `daEnBlockWater_c`): the `+0x698` matrix placement with the
  negated `+0x104` roll, the `+0x714` turn band and its `0x10`-step integrator
  with the side byte `+0x716`, the `+0x67D` vanish predicate, and the
  item/clip state step that selects `0x80B6FA28` or `0x80B6FA68` and arms the
  `+0x6A0` timer with `0x32`.
- **Block C (`0x80A24F20..0x80A25A20`, eleven bodies).** The blue-coin
  collectible (`0x940` bytes, inferred `daEnBlueCoin_c`): its state
  destructor, two forwarders, the three-stage constructor (base table
  `0x80301EA8`, coin table `0x80B3A828`, second base table `0x80302D78`,
  class table `0x80B3A53C`, eleven holders), the deleting destructor of the
  coin base class, the release step, the `g3d/obj_coin.brres` /
  `obj_coin_blue` binder, the two-list owner credit scan, the scene-armed
  collect step, the motion seed around sound `0x237`, and the frame step that
  spins the yaw, counts `+0x93C` down and pays the player.
- **Block D (`0x80A25DB0..0x80A25F30`, three bodies).** The opening of a
  `0x540`-byte actor, vtable `0x80B3AC00`: its state destructor, one
  forwarder, and the short constructor that allocates `0x540` bytes and
  initialises the render-list holder `+0x524`. No name is printed for this
  class, so none is invented.

### Identity evidence

- **The class names are inferred, never asserted.** Each vtable sits in the
  data section the same distance in front of its class's `StateID` strings as
  the already-translated `daEnBlockStaffRoll_c` vtable `0x80B399B4` does
  (`0x334` bytes before `daEnBlockStaffRoll_c::StateID_Wait`). That layout
  places `0x80B39D9C` in front of `daEnBlockSwich_c::StateID_Wait` /
  `_EndWait` (`0x332`), `0x80B3A154` in front of
  `daEnBlockWater_c::StateID_Wait` / `_HitWait` (`0x334`) and `0x80B3A828` in
  front of `daEnBlueCoin_c::StateID_Wait` / `_DemoUp` (`0x34E`). The strings
  themselves are at `0x80B3A0CE`, `0x80B3A488` and `0x80B3AB76`.
- **The vtables tie the bodies to the runs.** `0x80B39D9C` holds
  `0x80A23850` at slot `+0x14` and `0x80A23740` at slot `+0x20`;
  `0x80B3A828` holds `0x80A251C0` at slot `+0x48`; `0x80B3A53C` holds
  `0x80A256D0` at slot `+0x14`. Those three slot roles were already
  established for the wave-58 classes.
- **The blue coin is named by its own binder.** `0x80A25730` loads
  `g3d/obj_coin.brres`, looks up the `obj_coin` model and the `obj_coin_blue`
  node — the exact three calls of the translated red-coin binder
  [`fn_80af1500`](COVERAGE_WAVE73_ENEMIES.md), which binds `obj_coin_red`.
- **The 48-byte forwarders** are the same
  `0x802DCBFC(param_2, param_2, param_1)` bodies already documented in this
  module; their hidden `r12` this-adjust is not modelled and each doc says so.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A23180` | `fn_80a23180` | 88 | high |
| `0x80A232A0` | `fn_80a232a0` | 48 | medium |
| `0x80A232D0` | `fn_80a232d0` | 48 | medium |
| `0x80A23300` | `fn_80a23300` | 116 | high |
| `0x80A23740` | `fn_80a23740` | 272 | medium |
| `0x80A23850` | `fn_80a23850` | 84 | medium |
| `0x80A238B0` | `fn_80a238b0` | 308 | medium |
| `0x80A23EC0` | `fn_80a23ec0` | 88 | high |
| `0x80A23FE0` | `fn_80a23fe0` | 48 | medium |
| `0x80A24010` | `fn_80a24010` | 48 | medium |
| `0x80A24040` | `fn_80a24040` | 116 | high |
| `0x80A24490` | `fn_80a24490` | 112 | high |
| `0x80A24500` | `fn_80a24500` | 124 | high |
| `0x80A24580` | `fn_80a24580` | 128 | high |
| `0x80A24600` | `fn_80a24600` | 156 | medium |
| `0x80A248E0` | `fn_80a248e0` | 496 | medium |
| `0x80A24F20` | `fn_80a24f20` | 88 | high |
| `0x80A25040` | `fn_80a25040` | 48 | medium |
| `0x80A25070` | `fn_80a25070` | 48 | medium |
| `0x80A250A0` | `fn_80a250a0` | 288 | high |
| `0x80A251C0` | `fn_80a251c0` | 124 | high |
| `0x80A256D0` | `fn_80a256d0` | 84 | medium |
| `0x80A25730` | `fn_80a25730` | 172 | high |
| `0x80A257E0` | `fn_80a257e0` | 160 | medium |
| `0x80A258A0` | `fn_80a258a0` | 168 | medium |
| `0x80A25950` | `fn_80a25950` | 180 | high |
| `0x80A25A20` | `fn_80a25a20` | 148 | medium |
| `0x80A25DB0` | `fn_80a25db0` | 88 | high |
| `0x80A25ED0` | `fn_80a25ed0` | 48 | medium |
| `0x80A25F30` | `fn_80a25f30` | 84 | high |

## Deliberate skips

- **`0x80A24AD0` (312 bytes).** Block B's per-frame integrator is built from
  `unaff_GQR0`/`unaff_GQR3` quantiser fiction and `ldexpf` calls that are not
  in the original code, so the band/height arithmetic cannot be recovered.
  `0x80A24580` (translated) is still called from it by address.
- **`0x80A25F00` (48 bytes).** The second `0x802DCBFC` forwarder of block D is
  byte-identical to the translated `0x80A25ED0`; it was left out to keep the
  lane inside its 30-function budget. It is the only skip in the run that is
  not damaged evidence, and the omission is recorded here rather than hidden.
- **Other damaged exports in the bucket.** `0x80A20810` (228), `0x80A20900`
  (520), `0x80A22820` (700), `0x80A2BC50` (456), `0x80A2EA60` (224),
  `0x80A2EB40` (188) and `0x80A2EC00` (472) carry `unaff_` pseudo-registers;
  `0x80A22B20` (44), `0x80A281C0` (32) and `0x80A2D460` (20) are truncated by
  `halt_baddata`. None is claimed.
- **The rest of the bucket.** 108 functions of `0x80A20000` remain, including
  the `bombhei` family from `0x80A263C0` onward. The largest clean candidates
  for the next pass are `0x80A2DFE0` (940), `0x80A2E450` (704), `0x80A2EF70`
  (512), `0x80A29060` (476), `0x80A29310` (484) and `0x80A26770` (496).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,008 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 33 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave76` prints
  `wave76-enemies.csv: 30 rows`, `unique addresses: 117` and `OK`.
- `cargo check --message-format=short` finishes with no diagnostic naming
  `src/game/enemies.rs`; the remaining warnings are in `src/runtime/*`.
