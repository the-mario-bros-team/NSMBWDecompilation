# Coverage - Wave 100 (`game::enemies`, the `0x80A10000` bucket)

## How to read this file

This page records the Wave 100 `game::enemies` lane: the `pakkun_black`-model
actor pair of the `0x80A10000` bucket of REL module 3, `d_enemiesNP`, promoted
to `src/game/enemies.rs` (section 81 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave100-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The
`pakkun_black` label comes from the model strings in the module data section
(`g3d/pakkun_black.brres`, `pakkun_black`, `attack`), not from a surviving class
symbol.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 3,576 | 10 | 13 |

Four rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80A1AD40`, `0x80A1AD70`, `0x80A1C080` and `0x80A1C0B0`, whose hidden-`r12`
this-adjust pointer is not modelled (the same limitation recorded for the
`0x802DCBFC` copies in waves 56, 75, 80, 81, 82, 97, 98 and 99).

The bucket holds 209 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 120 were already in
`ledger/functions.csv` before this pass, 27 are claimed here and 62 remain
(including the 20-byte `0x80A1B2E0`, which cannot be reconstructed).

## What this pass covers

One contiguous run, `0x80A1A9F0..0x80A1C0B0`, in address order:

- **The `0x630`-byte object (class vtable `0x80B37D08`).** Three normaliser
  copies (`0x80A1A9F0`, `0x80A1AA50`, `0x80A1BF60`) built from
  `0x8015F800`/`0x802B9280`, the constant producer `0x80A1AAB0` returning
  `0x80357BF4`, the five-link `+0x30` chain walker `0x80A1ABA0`, the
  `:`-suffix name predicate `0x80A1AC80`, two `0x802DCBFC` forwarders, four
  byte-identical release wrappers and the constructor `0x80A1ADA0`
  (`0x630` bytes, `+0x524` transform from `0x800C0D70`, state word `+0x610`).
- **The `0x630` spawn cycle.** `0x80A1AF40` copies `+0xAC..+0xB4` to
  `+0x624..+0x62C`, writes `16.0 * (((word >> 8) & 0xF) + 1)` to `+0x620`,
  then runs `0x80A1B060` (the sixteen-entry `0x80B2B540` scale table
  `1.0, 1.5, 2.0, 3.0, 0.25, 0.5, 0.75, 2.5` and their negatives, placed with
  `0x800C10F0`/`0x800C1580`) and `0x80A1B0D0` (the two `0xFFFF` active-slot
  masks at `+0x614`/`+0x616`). `0x80A1B280` reads the transform back into
  `+0xAC`/`+0xB0`; `0x80A1B300` is the three-state spawn timer and
  `0x80A1B3F0` the spawner (object id `0x8E`, cached point `+0x624`, state
  copied to the new object's `+0x820`).
- **The `0x600`-byte object (class vtable `0x80B37FC8`).** The constructor
  `0x80A1B560` builds the usual `+0x524` loader, `+0x544` model, `+0x584`
  animation (vtable `0x80B330E0`) and `+0x5BC`/`+0x5C8` owner interfaces.
  `0x80A1B910` seeds the animation frame from `0x8042A200` modulo the `+0x5AC`
  period; `0x80A1B980` binds `g3d/pakkun_black.brres`, the `pakkun_black` node
  and the `attack` clip, with the holder id `0x20`/`0x23` selected by the
  `+0x5F4` variant and a second animation object at `+0x5BC` for variant `1`.
  `0x80A1BB30` emits the `0x94` stage probe, `0x80A1BBD0` is the per-frame
  player-list step that switches to the `Move` descriptor `0x80B6F658`, and
  `0x80A1BD10` is the teardown chain.
- **The following class's helpers.** `0x80A1BF60` is the callback that the
  already-translated `daEnBlackPakkun_c` table at `0x80A1BDA0` names; the two
  48-byte forwarders `0x80A1C080`/`0x80A1C0B0` sit directly in front of that
  class's `0x828` constructor at `0x80A1C0E0` (translated in an earlier wave)
  and are grouped with it by address order only (`// inferred:`).

### Identity evidence

- **No class name survives.** `0x80B37D08` and `0x80B37FC8` are referenced only
  by their constructors. The objects are described by size, vtable and field
  offsets only.
- **The strings are literal.** `g3d/pakkun_black.brres`, `pakkun_black` and
  `attack` were read from `work/rels/elf/d_enemiesNP.elf` at
  `0x80B37F94..0x80B37FBC`.
- **Every constant was dumped, not assumed.** `0x80B2B530` is `16.0`;
  `0x80B2B538` is the high word of the `f64` `2^52`; `0x80B2B540..0x80B2B57C`
  is the sixteen-float scale table; `0x80B2B580` is a zero word; `0x80B2B58C`
  is `0.0`; `0x80B2B590` is `1.0`; `0x80B2B598` is the `2^52` high word;
  `0x80B2B5A0` is the word table `0x20`, `0x23`; `0x80B2B5A8`/`0x80B2B5AC`
  are `16.0`/`8.0`.
- **The state descriptors are BSS.** `0x80B6F658` lies in the module BSS
  segment, so it is entered by address and its contents are not claimed.

### Deliberate skips

- **`0x80A1B2E0` (20 bytes).** `functions.tsv` lists it, but the export is
  `countLeadingZeros(+0x616 ^ 0xFFFF)` followed by `halt_baddata()` ("control
  flow encountered bad instruction data"), so no body can be reconstructed
  honestly.
- **The rest of the bucket: 62 functions.** Of those, one is the skip above;
  the others form these untranslated blocks: `0x80A10220` (484 bytes),
  `0x80A10880` (940), `0x80A12110` (8), `0x80A18730..0x80A18770` (three),
  `0x80A18E30`, `0x80A197F0`/`0x80A19820`, `0x80A19A70..0x80A1A3A0` (ten,
  including the 352-byte `0x80A19A70`), `0x80A1C790..0x80A1DAD0` (22, including
  the 636-byte `0x80A1C8F0`), `0x80A1E100..0x80A1EF30` (eleven, including the
  716-byte `0x80A1E2A0`) and `0x80A1F390..0x80A1FFB0` (nine). They are left for
  a pass with the disassembly.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A1A9F0` | `fn_80a1a9f0` | 88 | high |
| `0x80A1AA50` | `fn_80a1aa50` | 92 | high |
| `0x80A1AAB0` | `fn_80a1aab0` | 12 | high |
| `0x80A1ABA0` | `fn_80a1aba0` | 224 | medium |
| `0x80A1AC80` | `fn_80a1ac80` | 136 | medium |
| `0x80A1AD40` | `fn_80a1ad40` | 48 | low |
| `0x80A1AD70` | `fn_80a1ad70` | 48 | low |
| `0x80A1ADA0` | `fn_80a1ada0` | 92 | high |
| `0x80A1AE40` | `fn_80a1ae40` | 64 | medium |
| `0x80A1AE80` | `fn_80a1ae80` | 64 | medium |
| `0x80A1AEC0` | `fn_80a1aec0` | 64 | medium |
| `0x80A1AF00` | `fn_80a1af00` | 64 | medium |
| `0x80A1AF40` | `fn_80a1af40` | 136 | medium |
| `0x80A1B060` | `fn_80a1b060` | 112 | medium |
| `0x80A1B0D0` | `fn_80a1b0d0` | 432 | high |
| `0x80A1B280` | `fn_80a1b280` | 84 | high |
| `0x80A1B300` | `fn_80a1b300` | 240 | medium |
| `0x80A1B3F0` | `fn_80a1b3f0` | 140 | medium |
| `0x80A1B560` | `fn_80a1b560` | 184 | high |
| `0x80A1B910` | `fn_80a1b910` | 104 | medium |
| `0x80A1B980` | `fn_80a1b980` | 432 | high |
| `0x80A1BB30` | `fn_80a1bb30` | 116 | medium |
| `0x80A1BBD0` | `fn_80a1bbd0` | 272 | medium |
| `0x80A1BD10` | `fn_80a1bd10` | 144 | high |
| `0x80A1BF60` | `fn_80a1bf60` | 88 | high |
| `0x80A1C080` | `fn_80a1c080` | 48 | low |
| `0x80A1C0B0` | `fn_80a1c0b0` | 48 | low |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  3,576 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave100` reports `OK`; this lane's CSV has 27
  rows, 7 fields each.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace build itself is currently stopped by
  errors in other Wave 100 lanes (`src/render/gx_band.rs`,
  `src/resource/bres.rs`, `src/game/bases.rs`, `src/game/stage.rs`), which do
  not name this file.
