# Coverage - Wave 89 (`game::worldmap`, `0x80110000` bucket)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the course-background object. `r2` is `0x80433080` and `r13` is `0x8042F680`;
the `r2`- and `r13`-relative floats were read from `work/dol/main.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 29 | 3620 | 19 | 10 |

## What this pass covers

- **`0x80114BA0`..`0x80115D10` - background setup and screen extents.**
  `0x80114BA0` resets the `+0x434` placement record, applies `1.0` through the
  double-first `0x80169F80`, sets `+0x584` from status bit `8` and posts the
  `0x80375340` event through the `+0x394` component. `0x80115030` and
  `0x801150A0` divide the display extents by the shared integer counter, with
  `100.0` substituted when the counter is zero. `0x80115A90` initialises the
  `0x80`-byte screen/matrix record, including the `0xFFFF` sentinel, the two
  packed words at `+0x34`/`+0x38` and two `0x20`-byte blocks filled with `1`.
  `0x80115D10` publishes the `720x768` screen globals and their halves, then
  starts the `0x8016A960` transition and the four init steps.
- **`0x80116050`..`0x80116C00` - tile and grid configuration.** `0x80116050`
  builds a 48-byte tile scratch record and gates the `0x80118860` setup and the
  tile vtable slot `+0x14` on two `0x8037545F`/`0x80375462` per-variant bytes.
  `0x80116340` formats the `bgA_%04X` / `g3d_bgA_%04X.brres` (or `bgB`)
  resource names. `0x80116930` is the pure three-id classifier (`0..=7`),
  `0x80116A30` installs the two per-player grid records, `0x80116A70`/
  `0x80116AC0` read the `0x802F4BB8` table from grid `+2`/`+4`, and
  `0x80116B10`/`0x80116BB0` map the `+0x4E0` configuration to a height and a
  size class. `0x80116C00` places one cell from that configuration and writes
  the `+0x5F8` halfword cursor.
- **`0x80117A10`..`0x80119230` - layer counting, grid selectors, tile vtable.**
  `0x80117A10` divides the world extents by the screen extents, rounds each
  quotient up to an odd number and adds the two border cells. `0x80118D80`
  returns the `+0x4E4`-indexed grid record and `0x80118DA0` returns the other
  player's record. `0x801191E0` dispatches a `0x114`-stride tile entry's vtable
  slot `+0xB0` and stores the result at `base + index + 0x12`; `0x80119230` is
  the three-argument tail call to `0x80168230`. `0x80115910` and `0x80117C10`
  are the two identical release wrappers for this group.
- **`0x8011A1A0`..`0x8011C6E0` - size class, chain walk and placement.**
  `0x8011A1A0` reads the `0x802F4BF8` table with the grid's `+0x12` size class.
  `0x8011B300` walks up to five `+0x30` child links, testing each child's vtable
  slot `+0xC`, then recurses into the sixth link. `0x8011C040` copies an
  eight-word placement record and posts two `+0x620` layout events;
  `0x8011C100` applies a three-float position through the per-player
  `0x803755A0` scale vectors; `0x8011C2B0` classifies the `+0x5B4*+0x5B8`
  extent into `+0x618`; and `0x8011C6E0` combines the `0x8011C750` primary test
  with the optional `0x8011C8B0` fallback.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80114BA0` | `fn_80114ba0` | 148 | medium |
| `0x80115030` | `fn_80115030` | 100 | high |
| `0x801150A0` | `fn_801150a0` | 96 | high |
| `0x80115910` | `fn_80115910` | 88 | high |
| `0x80115A30` | `fn_80115a30` | 48 | high |
| `0x80115A60` | `fn_80115a60` | 48 | high |
| `0x80115A90` | `fn_80115a90` | 392 | high |
| `0x80115D10` | `fn_80115d10` | 172 | medium |
| `0x80116050` | `fn_80116050` | 184 | medium |
| `0x80116340` | `fn_80116340` | 232 | medium |
| `0x80116930` | `fn_80116930` | 256 | high |
| `0x80116A30` | `fn_80116a30` | 64 | high |
| `0x80116A70` | `fn_80116a70` | 80 | high |
| `0x80116AC0` | `fn_80116ac0` | 80 | high |
| `0x80116B10` | `fn_80116b10` | 156 | high |
| `0x80116BB0` | `fn_80116bb0` | 80 | high |
| `0x80116C00` | `fn_80116c00` | 276 | medium |
| `0x80117A10` | `fn_80117a10` | 176 | medium |
| `0x80117C10` | `fn_80117c10` | 64 | high |
| `0x80118D80` | `fn_80118d80` | 20 | high |
| `0x80118DA0` | `fn_80118da0` | 28 | high |
| `0x801191E0` | `fn_801191e0` | 80 | medium |
| `0x80119230` | `fn_80119230` | 16 | high |
| `0x8011A1A0` | `fn_8011a1a0` | 64 | high |
| `0x8011A410` | `fn_8011a410` | 64 | high |
| `0x8011B300` | `fn_8011b300` | 224 | medium |
| `0x8011C040` | `fn_8011c040` | 180 | medium |
| `0x8011C2B0` | `fn_8011c2b0` | 96 | high |
| `0x8011C6E0` | `fn_8011c6e0` | 108 | medium |

## Deliberate skips

- **GQR-reconstruction records `0x8011C750`, `0x8011C8B0` and `0x8011CA90`.**
  Each export opens with the `unaff_GQR0`/`ldexpf` prologue and several removed
  unreachable blocks, so the float loads cannot be recovered without the live
  `GQR0` base. [`fn_8011c6e0`] therefore calls the first two by raw address and
  uses only their return booleans; the third is left untouched. Same policy as
  waves 60/64/86.
- **Other GQR records elsewhere in the bucket.** `0x80111F00`, `0x80112DB0`,
  `0x80112E80`, `0x80113D90`, `0x80119240`, `0x8011D930` and `0x8011FCF0`
  also carry the removed GQR prologue and are not claimed by this pass.
- **The rest of the `0x80110000` bucket.** `0x80110B10`..`0x80114A90`, the
  `0x80117AC0` grid-record allocator, the `0x801194A0`/`0x80119C50`
  resource/loader records, the
  `0x8011A800`/`0x8011A8B0` collision probes, `0x8011AA50`, `0x8011AE90`, the
  `0x8011B500`/`0x8011B770`/`0x8011B9F0`/`0x8011BE70` cluster and the
  `0x8011D050`..`0x8011E420` sound/state nodes remain untranslated. They are
  valid follow-up targets for a later `0x80110000` pass.

## Verification

- Every address and byte size matches `functions.tsv`; the 29 sizes sum to
  3,620 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 29 were untranslated
  entries of the `0x80110000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error naming
  `src/game/worldmap.rs` (the build still stops on other lanes' errors).
- `node tools/check-lane-csv.mjs wave89` reports the lane clean.
