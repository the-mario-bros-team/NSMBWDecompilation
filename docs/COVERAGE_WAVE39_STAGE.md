# Coverage - Wave 39 (`game::stage`, course-manager cell accessors)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 13 | 1020 | 11 | 2 |

## What this pass covers

This pass closes the course-manager neighbourhood opened by
[`course_manager_construct`] (`0x800BB090`) in wave 10. The bucket
`0x800B0000` is a mixed program-flow band (title/UI, dialogue,
actors, lifts, resource bring-up), so only the helpers that share the
course-manager object (`*(r13 - 0x5704)`) and its cell tables are
translated here:

- **stage-slot byte getters** - the id/mode bytes at `0x80315B9C` /
  `0x80315B9D` (`0x800B2350` / `0x800B2360`), the same globals
  [`course_manager_set_course`] writes.
- **rectangle overlap** - `0x800B3100`, the helper
  [`bg_unit_collision_scan`] already calls by address. It normalises
  both pairs with two compares per axis and then runs the shipped
  nested inclusive-overlap test, reproduced literally.
- **slot lookup check** - `0x800B4710`, the other called-by-address
  helper this module uses: `countLeadingZeros(*(owner + index * 4 +
  0x384) - 3) >> 5` over the course manager at `*(r13 - 0x5704)`.
- **tile-table pair** - `0x800B4FB0` raises one entry of the strided
  tables at `manager + idx * 8 + 0x404` / `+ 0x774` (selected by
  `G_COURSE_MODE` bit `0x40`), and `0x800B5040` decrements it when it
  is `1` or `3`. Both use the clamped index `clamped(+0x3E4, 0, 10) +
  clamped(+0x3E8, 0, 9) * 10` and are gated on mode bit `0x10`. The
  tables are the ones [`course_manager_clear_tile_lists`] clears, so
  the roles are inferred and the bodies are `medium`.
- **cell and indexed-table accessors** - the cell address helper
  (`0x800BBBB0`), the two-word and one-word cell stores
  (`0x800BBBD0` / `0x800BBBF0`), the cell load (`0x800BBC10`), and
  the indexed stores/loads at `+0x320` / `+0x348`
  (`0x800BBC50` / `0x800BBC60` / `0x800BBC70`). All use the same
  `this + x * 0x40 + y * 0x10` base as
  [`course_manager_set_tile_entry`].

Deliberately skipped:

- The `ldexpf` / `unaff_GQR0` carriers in the bucket
  (`0x800B2F10`, `0x800B3000`, `0x800B33D0`, `0x800B38D0`,
  `0x800B6830`, `0x800B6E50`, `0x800B8CF0`, `0x800B9300`,
  `0x800B95E0`, `0x800B96A0`, `0x800B9A90`, `0x800B9FC0`,
  `0x800BAE60`, `0x800BBE10`, `0x800BC2E0`, `0x800BC4C0`,
  `0x800BC5D0`, `0x800BC6B0`, `0x800BD7A0`, `0x800BD970`,
  `0x800BE000`, `0x800BE390`, `0x800BEC00`, `0x800BEF00`,
  `0x800BFE90`): paired-single code the brief requires skipping.
- Dialogue/fukidashi (`0x800B1760..0x800B28F0`, including the
  `0x800B2230` / `0x800B2370` / `0x800B24B0` handlers that also write
  course-manager bytes), font setup (`0x800B1330` and neighbours),
  the fixed colour tables (`0x800B1BD0`, `0x800B3C50`,
  `0x800B4730`), the audio latch batches (`0x800B3E50..0x800B42B0`),
  the pane helpers (`0x800B44D0..0x800B4650`) and the RNG accessors
  (`0x800B2EB0..0x800B2F00`): other systems with no course-manager
  cell state.
- Engine frame resource services: the `0x8042A440` object dispatchers
  (`0x800B0A20` / `0x800B0A90` / `0x800B0B00` / `0x800B0B20`), the
  bring-up pair (`0x800B0B50` / `0x800B0DB0`), the four-view family
  (`0x800B58E0..0x800B63A0`, including `0x800B5C60`, whose block
  tables `0x803741D0` / `0x80377C88` are held by the `engine::globals`
  and player lanes) and the heap initialisers
  (`0x800B6450..0x800B64E0`).
- Save/course-scene glue owned by the fellow `game::scene` lane and
  the display lanes: the save-record checks (`0x800B4DE0` /
  `0x800B4E80` / `0x800B5400`), the world-map object (`0x800BB3C0`)
  and the course transition reset (`0x800B2FB0`, which also calls
  the save-image refresh and the global reset).
- Actor/lift systems: the Iggy chain-link family
  (`0x800BA5E0..0x800B90C0`, including the list walkers
  `0x800B9170..0x800B94C0`), the kanaami resource load (`0x800BBD30`,
  whose kanaami actors live in `game::bases`), the lift matrix family
  (`0x800BE690..0x800BFD80`, including the wrappers `0x800BFA40` /
  `0x800BFAA0` whose callee argument lists Ghidra dropped) and the
  actor sub-object (`0x800B8440..0x800B8EE0`, embedded at `+0x484` by
  the actor base constructor `0x80094E80`).
- Scene-graph / camera helpers left for `game::scene`: the BG-scene
  wrap tests (`0x800B55F0` / `0x800B56E0`), the BG-scene derived
  getters (`0x800B30C0` / `0x800B30E0`), the direction-matrix builder
  (`0x800BC140` with its `0x800BBDD0` perspective helper), the
  `+0xCC` buffer loop (`0x800BC460`) and the orphan buffer writers
  (`0x800BCA40` / `0x800BD1E0`).
- Trivial empty stubs not worth a body: `0x800BBF00` (4 bytes),
  `0x800B81B0` (4 bytes) and the 4-byte thunks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B2350` | `fn_800b2350` | 16 | high |
| `0x800B2360` | `fn_800b2360` | 16 | high |
| `0x800B3100` | `fn_800b3100` | 468 | high |
| `0x800B4710` | `fn_800b4710` | 32 | high |
| `0x800B4FB0` | `fn_800b4fb0` | 144 | medium |
| `0x800B5040` | `fn_800b5040` | 168 | medium |
| `0x800BBBB0` | `fn_800bbbb0` | 24 | high |
| `0x800BBBD0` | `fn_800bbbd0` | 28 | high |
| `0x800BBBF0` | `fn_800bbbf0` | 24 | high |
| `0x800BBC10` | `fn_800bbc10` | 52 | high |
| `0x800BBC50` | `fn_800bbc50` | 16 | high |
| `0x800BBC60` | `fn_800bbc60` | 16 | high |
| `0x800BBC70` | `fn_800bbc70` | 16 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/stage.rs`.
