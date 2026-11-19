# Coverage - Wave 96 (`game::enemies`, the `0x80B10000` bucket)

## How to read this file

This page records the Wave 96 `game::enemies` lane: 30 functions of the
`0x80B10000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 77 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body in that file, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave96-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The float
constants cited below were read out of `work/rels/elf/d_enemiesNP.elf` at the
cited synthetic addresses, never guessed from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 7,180 | 16 | 14 |

The bucket holds 211 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 115 were already in
`ledger/functions.csv` before this pass. 30 are claimed here and 66 remain.

## What this pass covers

Two runs.

**Run 1 - the `daEnTogeMet_c` trail walker, `0x80B10110..0x80B11F80`.**

- `0x80B10110`/`0x80B10120` are 16-byte holder steppers: they load the vtable
  of the object stored at `+0x544` (resp. `+0x5BC`), tail-call its slot `+0x14`,
  and leave `r3` pointing at the actor.
- `0x80B102A0` is the contact dispatcher. A hit record whose kind halfword is
  `0x3B` takes the class slot `0x12C`; otherwise the `+0x394` holder's object is
  asked through two name tests (`0x80B78B28`, then `0x80B78BE4`) and the
  `+0x5C9` node byte picks the plane - nodes `1`/`3` compare `+0xEC` with the
  other actor's `+0x7C`, all other nodes compare `+0xE8` with `+0x5C`. Both
  comparisons are "opposite sides of zero".
- `0x80B10440` scans kind-`0x3B` actors and then kind-`0x47` actors for one
  whose box `(x - 8, y)`..`(x + 8, y + 16)` overlaps the probe rectangle built
  from the four `+0x148` accessors (`0x8008C4F0`, `0x8008C550`, `0x8008C4D0`,
  `0x8008C530`), each shifted by `1.0`.
- `0x80B10940` is the death step: it lazily fills `0x80B78C60`, appends the
  position to the global frame manager (`+0x158`, clamped to `8`), spawns the
  `24.0` effect, writes the `+0x5CB` side from the fractional part of `x / 16`,
  sets `+0xE8` from the signed `0x8042C1A0` table, `+0xEC` to `3.0` inside the
  export's wrapping angle range, and enters `0x80B78BA8`.
- `0x80B10B20`/`0x80B10BE0` are the two ground probes: the first walks the lazy
  four-direction table at `0x80B78C88`, the second a `sin`/`cos` step of the
  `+0x100 - 0x4000` heading at `±8.0`. Both test bits `0x803D`.
- `0x80B10CE0` reloads `+0xE8`/`+0xEC` from `0x80B78AB8`, negated for the
  `+0x348` side.
- `0x80B10D50` refills the 64-record trail at `+0x5EC` as one straight line,
  starting from the actor position plus one offset of `0x80B78CB0` and one of
  the two-row table at `0x80B78CD8`, stepping by that row entry times `0.03125`.
- `0x80B10F80` derives the heading from the trail ends: the `0x80B31FF8` table
  selects record `0` or record `63`, the x difference is mirrored by `+0x348`,
  and `0x801611C0` writes the `+0x100` angle. `0x80B11020` writes the
  perpendicular `8.0` tangent into `+0x164`/`+0x168`.
- `0x80B110F0` advances the `+0x5C9` direction by the `±1` step at
  `0x80B32040[+0x5C8]` (negated for lane `1`) after probing both sides of the
  head record, and writes the straight-ahead point.
- `0x80B11300` pushes that point at the end `+0x5CA` selects:
  `0x80B11360` shifts 126 words down-to-up and inserts at `+0x5EC`,
  `0x80B11480` shifts 126 words up-to-down and inserts at `+0x7E4`.
- `0x80B11780` builds the oriented box `+0x5D0..+0x5E4` from the four rotated
  corners `(-10, 18)`, `(10, 18)`, `(-10, -2)` and `(10, -2)`.
- `0x80B11A30` returns the first kind-`0x79` actor's `+0x394` float.
  `0x80B11A70` is the hit reaction (damage report, snap to the box centre,
  sound `0x17B`, `DieBreak`); `0x80B11BF0` rebinds the `walk` node at scale
  `2.0`; `0x80B11CF0` is the main step with its `0.525` speed threshold;
  `0x80B11EC0`/`0x80B11F80` are the two wind-up turn steps.

**Run 2 - the `wanwan` chain actor, `0x80B1F970..0x80B1FF30`.**

- `0x80B1F970`/`0x80B1F9D0` are the pair of disposal callbacks
  (`0x8015F800(state, 0)`, then `0x802B9280` for a positive mode).
- `0x80B1FA30`/`0x80B1FA40` return the fixed pointers `0x80357BF4` and
  `0x80358104`.
- `0x80B1FB30` is the `+0x30` chain walker over vtable slot `+0xC`;
  `0x80B1FC10` is the `:`-suffix name predicate.
- `0x80B1FD30` constructs the `0x660`-byte actor: vtable `0x80B6BC58`, the
  `+0x524` model block, `+0x544`/`+0x550` placements, `+0x568 = 3` and the
  `+0x56C` rectangle animator.
- `0x80B1FF30` binds `g3d/wanwan.brres` and the `wanwan_kui` /
  `wanwan_chain_ring` nodes to the two placements.

### Identity evidence

- **The `daEnTogeMet_c` name is in the image.** `0x80B68BB4` holds
  `g3d/met_toge.brres`, `0x80B68BD0` `met_toge`, `0x80B68BDC` `walk` and
  `0x80B68BE8` `met_toge_shell`; `0x80B68F64..0x80B6901E` holds
  `daEnTogeMet_c::StateID_Walk`, `Turn`, `WindTurn`, `DieBreak`, `Ice` and
  `DieFall`. The 11-slot record list at `0x80B68E94` names this lane's
  `0x80B11CE0`, `0x80B11CF0`, `0x80B11BF0`, `0x80B11EA0`, `0x80B11EC0`,
  `0x80B11E90`, `0x80B11F70`, `0x80B11F80`, `0x80B11F60`, `0x80B12460` and
  `0x80B12470` in that order.
- **No state name is attached to a body.** The six StateID strings have no
  absolute pointer anywhere in the text section, and four of the eleven record
  slots are 12/16-byte thunks that `functions.tsv` does not list separately, so
  the pairing between the six names and the six listed bodies is not provable
  and is not claimed.
- **`wanwan` is named by its own binder.** `0x80B6BC1C` =
  `g3d/wanwan.brres`, `0x80B6BC30` = `wanwan`, `0x80B6BC38` = `wanwan_kui`,
  `0x80B6BC44` = `wanwan_chain_ring`; `0x80B6BC58` is the class vtable installed
  by the constructor.
- **Every float was dumped, not assumed.** `0x80B32004` = `1.0f`,
  `0x80B32008` = `0.0f`, `0x80B3200C` = `16.0f`, `0x80B32010` = `3.0f`,
  `0x80B32014` = `8.0f`, `0x80B32018` = `-8.0f`, `0x80B3201C` = `24.0f`,
  `0x80B32020` = `0.0625f`, `0x80B32028` = `0x4330000080000000` (the
  `2^52 + 2^31` bias), `0x80B32030` = `-24.0f`, `0x80B32034` = `0.00390625f`,
  `0x80B32038` = `0.03125f`, `0x80B32040` = `-1, +1`,
  `0x80B32048` = `10.0f`, `0x80B32050` = `1.15f`, `0x80B32054` = `-10.0f`,
  `0x80B32058` = `18.0f`, `0x80B3205C` = `-2.0f`, `0x80B32060` = `0.5f`,
  `0x80B32064` = `32.0f`, `0x80B32068` = `5500.0f`, `0x80B3206C` = `2.0f`,
  `0x80B32070` = `0, 63`, `0x80B32078` = `0.525f`, `0x80B3207C` = `0.25f`.
- **The `0x80B78xxx` tables are BSS.** `0x80B78AB8`, `0x80B78C58`,
  `0x80B78C60`, `0x80B78C80`, `0x80B78C88`, `0x80B78CA8`, `0x80B78CB0`,
  `0x80B78CD0`, `0x80B78CD8`, `0x80B78D18`, `0x80B78D20`, `0x80B78D40` and
  `0x80B78D48` all sit above `0x80B6E500`, so the bodies lazily initialise them
  and no image content is claimed for them. The four state names
  (`0x80B78AE8`, `0x80B78B28`, `0x80B78B68`, `0x80B78BA8`, `0x80B78BE4`) are BSS
  too and are entered by address only.

### Deliberate skips

- **`0x80B115A0` (480 bytes).** The export passes `&local_88` to `0x800B8700`
  for a 60-byte record but also fills `local_d4`/`local_d0`/`local_cc` with
  `1.15`, which Ghidra's frame layout puts outside that record. Without the
  disassembly the block boundaries cannot be settled, so no body is claimed.
- **`0x80B1FCD0` and `0x80B1FD00` (48 bytes each).** Both are byte-identical
  `0x802DCBFC(second, second, first)` forwarders; the shape is already covered
  elsewhere in this bucket and neither target role is established.
- **The rest of the bucket: 66 functions.** The largest untouched runs after
  this pass are `0x80B123D0..0x80B12470`, `0x80B147D0`, `0x80B15C60..0x80B17CB0`,
  `0x80B18F20`, `0x80B19EE0`, `0x80B19FA0`, `0x80B1AD30`, `0x80B1B0E0`,
  `0x80B1B3F0` and `0x80B1CC50..0x80B1EF80`.
- **Two exports cannot be reconstructed on their own.** `0x80B14D80` (28
  bytes) and `0x80B1B0E0` (32 bytes) are
  `/* WARNING: Control flow encountered bad instruction data */` plus
  `halt_baddata()`, the same failure mode the wave-95 lane hit at
  `0x80ADA6E0`, so neither is claimed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B10110` | `fn_80b10110` | 16 | medium |
| `0x80B10120` | `fn_80b10120` | 16 | medium |
| `0x80B102A0` | `fn_80b102a0` | 408 | medium |
| `0x80B10440` | `fn_80b10440` | 560 | medium |
| `0x80B10940` | `fn_80b10940` | 476 | medium |
| `0x80B10B20` | `fn_80b10b20` | 184 | high |
| `0x80B10BE0` | `fn_80b10be0` | 244 | medium |
| `0x80B10CE0` | `fn_80b10ce0` | 100 | high |
| `0x80B10D50` | `fn_80b10d50` | 556 | high |
| `0x80B10F80` | `fn_80b10f80` | 152 | medium |
| `0x80B11020` | `fn_80b11020` | 200 | medium |
| `0x80B110F0` | `fn_80b110f0` | 524 | medium |
| `0x80B11300` | `fn_80b11300` | 92 | high |
| `0x80B11360` | `fn_80b11360` | 288 | high |
| `0x80B11480` | `fn_80b11480` | 288 | high |
| `0x80B11780` | `fn_80b11780` | 680 | high |
| `0x80B11A30` | `fn_80b11a30` | 64 | high |
| `0x80B11A70` | `fn_80b11a70` | 372 | medium |
| `0x80B11BF0` | `fn_80b11bf0` | 228 | medium |
| `0x80B11CF0` | `fn_80b11cf0` | 404 | medium |
| `0x80B11EC0` | `fn_80b11ec0` | 152 | medium |
| `0x80B11F80` | `fn_80b11f80` | 248 | high |
| `0x80B1F970` | `fn_80b1f970` | 88 | high |
| `0x80B1F9D0` | `fn_80b1f9d0` | 92 | high |
| `0x80B1FA30` | `fn_80b1fa30` | 12 | high |
| `0x80B1FA40` | `fn_80b1fa40` | 12 | high |
| `0x80B1FB30` | `fn_80b1fb30` | 224 | medium |
| `0x80B1FC10` | `fn_80b1fc10` | 136 | high |
| `0x80B1FD30` | `fn_80b1fd30` | 140 | high |
| `0x80B1FF30` | `fn_80b1ff30` | 224 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  7,180 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave96` prints
  `wave96-enemies.csv: 30 rows` and `OK` for this lane.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. During this pass the workspace build was stopped by a
  parse error in `src/runtime/bt.rs` (another Wave 96 lane), so the file was
  additionally checked on its own with `platform.rs` in a scratch crate, where
  it compiles with no errors and no warnings.
