# Coverage - Wave 97 (`game::worldmap`, `0x80100000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`). The two `W97_R2_*` floats below were decoded from
`work/dol/main.elf` at the absolute address recorded in each constant's
comment.

The group is the `dGoalManager_c` screen and the five-record `0x264`-stride
layout controller that wave 92 already documented. The class name is not
printed by the export on the handlers, but the neighbouring initialiser
`0x8010B890` names its records `dGoalManager_c::StateID_StartWait`,
`KiMePose`, `MessageOpen`, `ScoreCalc`, `MessageDone`, `MessageEnd` and
`ClearWait`. The `+0x70`/`+0x74` fields hold NW4R `lyt` objects; their
constructor `0x800C8950` and prepare pass `0x800C9560` are documented in
`src/engine/display.rs`, and the destructor `0x800C89A0` in
`src/game/actor.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 3,368 | 21 | 9 |

Every row is `translated`; no body is `stub` or `partial`, and no body is
`low` confidence.

## What this pass covers

- **`0x8010A9B0..0x8010ACC0` - the scene draw pair and two GX helpers.**
  `0x8010A9B0` registers all nine scene sub-objects (`0x80014380` with mask
  `0x1FF`), runs the stack-record draw `0x8010A3E0`, builds a `0x400 x 0x400`
  texture of format `5` with the eight-argument `0x801C6D90`, and binds it to
  map slot `0` with `0x801C74C0`. `0x8010AA30` is the matching `0x80014480`
  teardown; `0x8010AA40` is a one-line GX attribute writer
  (`0x80 = (value & 0x3FFF) << 2`); `0x8010AA90` is a bare `blr`;
  `0x8010ACC0` releases the scene, selects texture stage `0` and flushes.
- **`0x8010AF70..0x8010BF30` - the goal-manager controller and its free
  wrappers.** `0x8010AF70` installs vtable `0x803222F0` at `+0x60`, the five
  interface tables at `+0x74..+0x80`, back-links `+0x84`, builds the state
  manager at `+0x8C` from the `0x80374C90` table and publishes the object at
  `r13 - 0x53BC`. The four state handlers `0x8010B2C0`/`0x8010B370`/
  `0x8010B420`/`0x8010B570` scan the four player controllers through
  `0x800B4710` (or the three stored animation indices through `0x800C9610`)
  and post `KiMePose` (`0x80374CD0`), `MessageOpen` (`0x80374D50`),
  `ScoreCalc` (`0x80374D90`) and `ClearWait` (`0x80374E50`) on the `+0x74`
  interface slot `0x18`. `0x8010B5F0` selects course mode `1`/`3` through
  `0x8005B7D0`/`0x800B4FB0` and binds the four panes from `0x80354E60`/
  `0x80354E90`. The free/destroy wrappers `0x8010B010`, `0x8010B050`,
  `0x8010B090`, `0x8010B0F0`, `0x8010BF30`, `0x8010EE80` and the interface
  dispatch `0x8010B1F0` complete the group.
- **`0x8010C050..0x8010C7F0` - the `0x264`-record controller.** `0x8010C050`
  builds the five-record array at `+0x144` with `0x802DC9A0`, using
  `0x8010C0C0`/`0x8010C110` as the per-record constructor/destructor pair,
  and publishes itself at `r13 - 0x53B8`. `0x8010C510` lays out the five child
  quads of one record (centre from `+0x384`/`+0x388` plus the `+0x390`/
  `+0x394` offsets, mirrored by the `+0x39C`/`+0x39D` axis bytes, corners
  scaled by `0.5f` from `0x8042D30C`, timer `-0x1A`). `0x8010C7F0`
  initialises a record from a placement source: name copy through `0x802E1938`
  into `+0x305`, transform words `+0x36C..+0x394`, the `+0xBA` byte split into
  `+0x39C = byte % 3` and `+0x39D = byte / 3`, the `0xFF` timer and the
  `0x93`/`0x98`/`0x0E` style bytes.
- **`0x8010DB30..0x8010F3A0` - the state/overlay helpers.** `0x8010DB30`
  opens the `+0xB0C` state object with event `0x80374FC8`; `0x8010E670`
  validates a candidate through its vtable slot `0x0C` before storing it at
  `+0x0C`; `0x8010EFA0` constructs the small overlay (vtable `0x80322DD0`,
  layout at `+0x70`); `0x8010F340` prepares that layout and dispatches slot
  `0x10`; `0x8010F3A0` draws it through `0x80163850`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8010A9B0` | `fn_8010a9b0` | 124 | high |
| `0x8010AA30` | `fn_8010aa30` | 12 | high |
| `0x8010AA40` | `fn_8010aa40` | 20 | high |
| `0x8010AA90` | `fn_8010aa90` | 4 | high |
| `0x8010ACC0` | `fn_8010acc0` | 52 | high |
| `0x8010AF70` | `fn_8010af70` | 160 | high |
| `0x8010B010` | `fn_8010b010` | 64 | high |
| `0x8010B050` | `fn_8010b050` | 64 | high |
| `0x8010B090` | `fn_8010b090` | 96 | high |
| `0x8010B0F0` | `fn_8010b0f0` | 100 | high |
| `0x8010B1F0` | `fn_8010b1f0` | 48 | medium |
| `0x8010B2C0` | `fn_8010b2c0` | 144 | medium |
| `0x8010B370` | `fn_8010b370` | 132 | medium |
| `0x8010B420` | `fn_8010b420` | 152 | medium |
| `0x8010B570` | `fn_8010b570` | 104 | medium |
| `0x8010B5F0` | `fn_8010b5f0` | 168 | medium |
| `0x8010B830` | `fn_8010b830` | 48 | high |
| `0x8010B860` | `fn_8010b860` | 48 | high |
| `0x8010BF30` | `fn_8010bf30` | 88 | high |
| `0x8010C050` | `fn_8010c050` | 112 | high |
| `0x8010C0C0` | `fn_8010c0c0` | 68 | high |
| `0x8010C110` | `fn_8010c110` | 88 | high |
| `0x8010C510` | `fn_8010c510` | 624 | medium |
| `0x8010C7F0` | `fn_8010c7f0` | 396 | medium |
| `0x8010DB30` | `fn_8010db30` | 32 | medium |
| `0x8010E670` | `fn_8010e670` | 96 | high |
| `0x8010EE80` | `fn_8010ee80` | 88 | high |
| `0x8010EFA0` | `fn_8010efa0` | 76 | high |
| `0x8010F340` | `fn_8010f340` | 96 | high |
| `0x8010F3A0` | `fn_8010f3a0` | 64 | high |

## Deliberate skips

- **`0x8010AAA0`, `0x8010AD00`.** Both build their arguments with the
  `0x43300000` integer-to-double trick and `CONCAT44`, and read their floats
  from unnamed `r2 - 0x5Dxx` slots; the GEOMETRY/texcoord values cannot be
  recovered from the C alone, so no body was written. `0x8010AD00` also writes
  the `0xCC008000` FIFO word directly.
- **`0x8010B890` (1,652 bytes).** The `dGoalManager_c` state-table
  initialiser copies roughly thirty words from BSS globals
  (`DAT_80322450..DAT_80322470` and four neighbours) into seven state records
  and registers each with `0x802DC780`. The copied words are not initialised
  in the export and their roles cannot be named, so the table is left
  unexpanded. Its state names are quoted in the section above.
- **`0x8010F600`.** Compares the player's `+0x8`/`+0xC` coordinates against
  four `r2` float limits (`r2 - 0x5D24`, `-0x5D28`, `-0x5D2C`, `-0x5D30`) that
  were not decoded, and drives the `r13 - 0x5394` countdown; submitting a
  guess for the limits would misstate the spawn region.
- **`0x8010DC80`, `0x8010DE30`, `0x8010E830`, `0x8010F0C0`.** The remaining
  large neighbours of the `0x8010E/Fxxx` ranges are deferred to a later pass;
  they are not part of this screen cluster.
- **`0x8010E7D0`, `0x8010E800`.** Two 48-byte thunks that are byte-identical
  to `0x8010B830`/`0x8010B860`; they were skipped to keep this pass on the
  distinct bodies and can be transcribed mechanically later.
- **`0x8010F470`, `0x8010F7A0`, `0x8010F980`, `0x8010FB50`, `0x8010FE20`,
  `0x8010FE80`, `0x8010FF70`.** Small neighbours after `0x8010F3A0`; left for
  the next pass.
- **Earlier `0x8010xxxx` gaps.** `0x80101AA0`, `0x80102DB0`, `0x80103410`,
  `0x80106620`, `0x801068B0`, `0x80107050`, `0x80107170`, `0x80107460`,
  `0x80108F40`, `0x80108F70` and the wave-81 stack-record graphics bodies
  `0x80109780`/`0x801097F0`/`0x8010A3E0` stay untranslated. `0x8010A3E0` is
  called by raw address from [`fn_8010a9b0`].

## Verification

- Every address and byte size matches `functions.tsv`; the 30 sizes sum to
  3,368 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 30 were untranslated
  entries of the `0x80100000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs`. The only remaining crate errors at the end of the
  pass were in `src/resource/archive.rs` (another parallel lane).
- `node tools/check-lane-csv.mjs wave97` reports `OK`.
