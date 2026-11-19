# Coverage - Wave 127 (`game::enemies`, the `0x80A50000` bucket tail)

## How to read this file

This page records the Wave 127 `game::enemies` lane: 30 functions of the
`0x80A50000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 108 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave127-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / the module's `read_f64`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 5,396 | 27 | 3 |

The bucket holds 181 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 52 were untranslated when
this pass started. This pass claims 30 of them and leaves 22.

## What this pass covers

- **The `center_firebar` actor (`0x80A55CB0..0x80A56DB0`, 7 bodies).** The
  binder `0x80A564A0` loads `g3d/center_firebar.brres` and picks
  `center_firebar` or `center_firebar_2unit` from the table at `0x80B46470`
  through the variant word `+0x247C`; the placement step reads the variant
  model from `+0x540`, normalises `(+0xAC, +0xB0 + 8.0, +0xB4 - 32.0)` through
  `0x80064AF0`/`0x801C0BD0`, and applies it at `+0x7C`. The remaining clean
  helpers are the `+0x5EC` sample emitter (class `0x6B`), the `0x6E` timer seed
  at `+0x6DC`, the facing-sign word at `+0x2482` (`0x100`/`0xC0`, negated by
  flag bit `0x1000`), and the two 4-byte empty hooks.
- **The `daEnFireSnake_c` state tail (`0x80A585D0..0x80A59F00`, 23 bodies).**
  The class is fixed by `register_fire_snake_states` at `0x80A5A350` (wave 4)
  and by the clips bound here: `R_jump_st`/`L_jump_st` (`0x80B46800`),
  `R_jump`/`L_jump` (`0x80B46818`) and `R_jump_ed`/`L_jump_ed`
  (`0x80B46838`). The run adds the `0x800B2EE0(3)` predicate, the
  `+0x164..+0x170` quad writer, the ten-effect ice-hit binder (two head
  effects at `+0xAC` and eight body effects at the four `0x260`-byte elements
  from `+0x7B8`), the state-reset helper with its gravity `-384.0`/`4500.0`
  choice and `12.0` divisors, the `wait`/`jump_st`/`jump`/`jump_ed` entries
  and their frame steps, and the two sound calls `0x1B6`/`0x1B7`. The state
  descriptors `0x80B71748..0x80B71A08` live in BSS and are cited by address.

### Why three rows are `medium`

- **A promoted-`f32` call shape.** [`fn_80a58a30`] calls `0x800E20A0` and
  `0x800E23C0` with two promoted `f32` values, but the export types the
  sibling globals as overlapping `f64` words. The reading follows the wave-96
  sibling at `0x80A72500`, which is the only translated call site of the same
  pair.
- **A GQR-damaged predicate.** [`fn_80a59230`] branches on `0x80A589A0`, whose
  own export is quantiser-damaged; the call is kept by analysis address and
  the predicate's arithmetic is marked `// unknown:`.
- **An unprinted target layout.** [`fn_80a594f0`] receives an object from
  virtual slot `+0x2C` of `+0x394` and then calls slot `+0x14` on it; the
  export does not print that object's fields.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A55CB0` | `fn_80a55cb0` | 80 | high |
| `0x80A55DE0` | `fn_80a55de0` | 56 | high |
| `0x80A56470` | `fn_80a56470` | 48 | high |
| `0x80A564A0` | `fn_80a564a0` | 184 | high |
| `0x80A56560` | `fn_80a56560` | 160 | high |
| `0x80A56B00` | `fn_80a56b00` | 4 | high |
| `0x80A56DB0` | `fn_80a56db0` | 4 | high |
| `0x80A585D0` | `fn_80a585d0` | 52 | high |
| `0x80A58610` | `fn_80a58610` | 56 | high |
| `0x80A58800` | `fn_80a58800` | 112 | high |
| `0x80A58870` | `fn_80a58870` | 232 | high |
| `0x80A58A30` | `fn_80a58a30` | 308 | medium |
| `0x80A58C80` | `fn_80a58c80` | 136 | high |
| `0x80A58EB0` | `fn_80a58eb0` | 176 | high |
| `0x80A58F80` | `fn_80a58f80` | 224 | high |
| `0x80A590C0` | `fn_80a590c0` | 204 | high |
| `0x80A59190` | `fn_80a59190` | 52 | high |
| `0x80A59230` | `fn_80a59230` | 204 | medium |
| `0x80A59330` | `fn_80a59330` | 240 | high |
| `0x80A59420` | `fn_80a59420` | 180 | high |
| `0x80A594F0` | `fn_80a594f0` | 396 | medium |
| `0x80A59680` | `fn_80a59680` | 272 | high |
| `0x80A597A0` | `fn_80a597a0` | 236 | high |
| `0x80A59890` | `fn_80a59890` | 152 | high |
| `0x80A59940` | `fn_80a59940` | 148 | high |
| `0x80A599E0` | `fn_80a599e0` | 232 | high |
| `0x80A59AE0` | `fn_80a59ae0` | 376 | high |
| `0x80A59C60` | `fn_80a59c60` | 196 | high |
| `0x80A59D40` | `fn_80a59d40` | 408 | high |
| `0x80A59F00` | `fn_80a59f00` | 268 | high |

## Deliberate skips

- **GQR-damaged firebar emitters.** `0x80A56620` (736), `0x80A56920` (476)
  and `0x80A56B10` (672) build their node angles from `unaff_GQR0`/`unaff_GQR3`
  quantiser fiction and `ldexpf` calls that are not in the original code, so
  their bodies cannot be typed honestly. They are the three remaining
  functions of the `center_firebar` block.
- **GQR-damaged fire-snake helpers.** `0x80A584E0` (232, the
  `0x80064740`/`0x801611C0` angle writer at `+0x780..+0x784`), `0x80A589A0`
  (144, the four-player distance query called by [`fn_80a59230`]) and
  `0x80A58D70` (308, the jump arc with the `x8000` half-turn) remain
  untranslated for the same reason. The first two were already recorded as
  GQR-damaged in wave 106.
- **`0x80A53850` (368).** The `dossun`-family fall-speed function whose
  `extraout_f1` clamp is unbound in the export; it stays skipped exactly as in
  wave 106.
- **Clean bodies left for a later pass.** `0x80A5AEF0` (12), `0x80A5B180`
  (48), `0x80A5B1B0` (48), `0x80A5BB30` (48), `0x80A5BB60` (48),
  `0x80A5C660` (436), `0x80A5D2D0` (48), `0x80A5D300` (48), `0x80A5D9D0`
  (68), `0x80A5DBC0` (212), `0x80A5E1A0` (136), `0x80A5E590` (252),
  `0x80A5EB00` (448), `0x80A5F7B0` (48) and `0x80A5F7E0` (48) are readable
  but were not needed once the 30-body target was met. Nothing was written for
  them, so no ledger row claims them. With them, 22 functions of the bucket
  remain untranslated.
- **No address already in `ledger/functions.csv` was touched**, and no body
  from another lane's file was edited.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  5,396 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was in
  `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave127` reports
  `wave127-enemies.csv: 30 rows`, 30 unique addresses and `OK`.
- The module header was updated: cluster item 108, the cluster count (one
  hundred and seven to one hundred and eight), the function totals (2,895 to
  2,925 public translations, 2,824 to 2,854 `fn_<addr>` bodies), the "start
  here" list and the records list.
