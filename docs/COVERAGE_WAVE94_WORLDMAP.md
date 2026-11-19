# Coverage - Wave 94 (`game::worldmap`, `0x80110000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); every `r2` float below was read from `work/dol/main.elf` at the
absolute address recorded in its `W94_R2_*` constant comment. The export never
names the classes involved, so the bodies keep literal offsets and the two
groups are labelled by address.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 28 | 7,500 | 5 | 23 |

No body is `low` confidence.

## What this pass covers

- **`0x80114C40..0x80117C50` - the ice archive binders and course-grid
  controller.** `0x80114C40` and `0x8011B9F0` load `g3d_ice_brres`, create the
  `ice_A1` model and layout selected by the `+0x04` nibble, and finish the
  `+0x3D0` node. `0x80116110` splits the variant into `+0x4E4`, classifies the
  three grid id halfwords into `+0x4E0`, records the `+0x480`/`+0x484` course
  extents, resolves the `0xD3` event object and seeds the 100-entry `+0x658`
  halfword table. `0x80117AC0` allocates the `0x8C50`-byte tile array at
  `+0x3B8` (34 by 22 records of `0x30` bytes, `-7000.0f`/`-8000.0f` height
  limits), and `0x80117C50` sets the `A..F` bits found in the layout archive's
  child names.
- **`0x801183B0..0x8011AE90` - extents, ground and placement helpers.**
  `0x801183B0` reads a six-float extent record and eases `+0xDC`/`+0xE0`;
  `0x80118860` compares the two three-float spans of every child against the
  `0x802F8FF0` filter; `0x801189E0` subtracts one `0xC`-stride tile position
  from another; `0x80119060` builds the three course layout names and returns
  their joint success; `0x80119140` projects the scene's half extents.
  `0x8011A100`, `0x8011A800`, `0x8011A8B0` and `0x8011AE90` are the
  position/probe/timer helpers that feed the tile fields.
- **`0x8011B500..0x8011BE70` - the `0xE70`-byte ice object.** The constructor
  installs eight embedded vtable sub-objects and their `+0xE48..+0xE64` link
  table; `0x8011B770` runs the `+0x614` name/state ladder, and `0x8011BE70`
  rebuilds the `+0x7C` layout from the `+0x59C`/`+0x100` pairs.
- **`0x8011C190..0x8011EA00` - tile placement and sound slots.** The
  `+0x394`-name-gated ground alignment (`0x8011C190`), the `+0x61C` child/size
  ladder (`0x8011CEB0`), the `r13 - 0x5770` scene-slot placement (`0x8011D050`)
  and the four `+0x134`/`0x8009F6xx` sound-slot variants (`0x8011D100`,
  `0x8011D1E0`, `0x8011D2B0`, `0x8011D380`) close the run with the
  `0x8011EA00` `+0x2D == 4` child comparison.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80114C40` | `fn_80114c40` | 292 | medium |
| `0x80115100` | `fn_80115100` | 220 | medium |
| `0x80116110` | `fn_80116110` | 548 | medium |
| `0x80117AC0` | `fn_80117ac0` | 312 | high |
| `0x80117C50` | `fn_80117c50` | 368 | medium |
| `0x801183B0` | `fn_801183b0` | 160 | medium |
| `0x80118860` | `fn_80118860` | 376 | medium |
| `0x801189E0` | `fn_801189e0` | 60 | high |
| `0x80119060` | `fn_80119060` | 216 | medium |
| `0x80119140` | `fn_80119140` | 52 | high |
| `0x8011A100` | `fn_8011a100` | 160 | medium |
| `0x8011A1E0` | `fn_8011a1e0` | 252 | high |
| `0x8011A800` | `fn_8011a800` | 176 | medium |
| `0x8011A8B0` | `fn_8011a8b0` | 180 | high |
| `0x8011AA50` | `fn_8011aa50` | 292 | medium |
| `0x8011AE90` | `fn_8011ae90` | 132 | medium |
| `0x8011B500` | `fn_8011b500` | 624 | medium |
| `0x8011B770` | `fn_8011b770` | 640 | medium |
| `0x8011B9F0` | `fn_8011b9f0` | 308 | medium |
| `0x8011BE70` | `fn_8011be70` | 308 | medium |
| `0x8011C190` | `fn_8011c190` | 284 | medium |
| `0x8011CEB0` | `fn_8011ceb0` | 408 | medium |
| `0x8011D050` | `fn_8011d050` | 172 | medium |
| `0x8011D100` | `fn_8011d100` | 224 | medium |
| `0x8011D1E0` | `fn_8011d1e0` | 204 | medium |
| `0x8011D2B0` | `fn_8011d2b0` | 204 | medium |
| `0x8011D380` | `fn_8011d380` | 204 | medium |
| `0x8011EA00` | `fn_8011ea00` | 168 | medium |

## Deliberate skips

- **`0x8011C100`.** `src/game/worldmap.rs` already contained a complete
  `fn_8011c100` body (the eight-float placement writer used by `fn_8011c040`),
  so this pass did not write a second one and did not add a ledger row. The
  address left the untranslated set before this lane started.
- **`0x8011CA90`.** A GQR-reconstruction record, already listed as a skip by
  wave 89. It stays untranslated but is called by raw address from
  `fn_8011c190` with the export's `(object, float*)` shape.
- **`0x80110B60`, `0x80110FA0`, `0x80118DC0`, `0x80112DB0`, `0x80112E80`,
  `0x80113D90`, `0x80118450`, `0x80118760`, `0x80119240`, `0x801194A0`,
  `0x80119C50`, `0x8011FCF0`.** Each export carries `unaff_GQR0` float loads
  and/or `ldexpf` reconstruction blocks, so the float constants cannot be
  recovered from the C alone.
- **The remaining `0x80110000` entries.** This pass selected the two contiguous
  runs described above. Entries outside them (for example `0x80111850`,
  `0x801121B0`, `0x801126A0`, `0x80112F50`, `0x80114050`, `0x80116430`,
  `0x80116D20..0x801177B0`, `0x80117DC0` and the `0x8011C750`/`0x8011C8B0`
  GQR records) stay untranslated for a later pass.

## Verification

- Every address and byte size matches `functions.tsv`; the 28 sizes sum to
  7,500 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 28 were untranslated
  entries of the `0x80110000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave94` reports `OK`.
