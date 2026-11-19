# Coverage - Wave 110 (`game::enemies`, the `0x80B20000` bucket)

## How to read this file

This page records the Wave 110 `game::enemies` lane: 29 functions of the
`0x80B20000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 91 of that file. Every row of the machine
readable mirror (`ledger/agents/wave110-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The pass covers two world-map actors whose helpers sit in two dense runs:

- **`Wm_en_pakkun`** (`0x80B22A20..0x80B23F00`), named from its effect strings
  only (no `StateID` table for it is in this export). The name survives in two
  effect names,
  `Wm_en_pakkun_foo` (`0x80B6CA3C`) and `Wm_en_pakkunsweat` (`0x80B6CA50`),
  and in the `g3d/pakkun_water.brres` archive string (`0x80B6CA10`).
- **`block_jump`** (`0x80B25830..0x80B264C0`), named by
  `g3d/block_jump.brres` (`0x80B6D1F0`) and its four-clip table
  `block_jumpA..` (`0x80B6D1E8`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 6,128 | 21 | 8 |

## What this pass covers

- **`0x80B22A20..0x80B22E00`, construction.** The null-guarded release helper
  (`0x80B22A20`), the `0x858`-byte allocating constructor with the class vtable
  at `+0x060` and both effect sub-objects at `+0x600`/`+0x728`
  (`0x80B22BA0`), and the `g3d/pakkun_water.brres` + `pakkun_water`/`foo`
  binder (`0x80B22E00`).
- **`0x80B23540..0x80B23720`, the effect helpers.** The `0x14D` spawn at
  `y + 48.0` that back-links the actor at effect `+0x570` (`0x80B23540`), the
  `Wm_en_pakkun_foo` placement with the `0.2 + 0.8 * dy / 96.0` height blend
  (`0x80B23650`), and the four-name contact predicate over the unnamed
  `main.dol` BSS objects `0x80357BF4`, `0x8082AAF0`, `0x8082AAB0` and
  `0x80358004` (`0x80B23720`).
- **`0x80B23960..0x80B23F00`, the state machine.** Four frame handlers that
  share the `0x8006CD90` / `0x800955F0` / `0x80B23480` / `fn_80b23650` tail and
  differ in their target test, countdown (`+0x5C4` or `+0x5C8`), state record
  and name gate (`0x80B23960`, `0x80B23AB0`, `0x80B23BE0`, `0x80B23D30`); the
  state-3 entry initialiser (`0x80B23E30`); the `Wm_en_pakkunsweat` re-arm
  (`0x80B23E80`); and the state-2 entry that seeds the side-dependent speed
  and pitch `0.5`/`-0.5`, `-0.075`, `-2.0` (`0x80B23F00`).
- **`0x80B25830..0x80B25960`, the `block_jump` placement and binder.** The
  matrix rebuild with the `8.0` lift (`0x80B25830`), the one-line `+0x534`
  reset (`0x80B25930`) and the `g3d/block_jump.brres` binder whose clip is
  selected by the `+0x740` variant from the `block_jumpA..` table
  (`0x80B25960`).
- **`0x80B25A20..0x80B25E20`, the per-player arming.** The 640-byte arming
  routine that maps the `+0x38C` character id onto the parallel masks at
  `+0x72D`, `+0x72E`, `+0x72F`, `+0x730` and `+0x734`, probes the player's
  ability handle with ids `0x1C`/`0x2B` and sets `+0x731`/`+0x736`
  (`0x80B25A20`); the child-added and child-removed callbacks
  (`0x80B25CA0`, `0x80B25D10`); the fourteen-byte record clear
  (`0x80B25DE0`); and the four-player mounted-object scan that sets `+0x732`
  (`0x80B25E20`).
- **`0x80B25EB0..0x80B26030`, the rate tables and effects.** The
  fall-height-band selector over the four tables `0x80B32AF0`, `0x80B32AB0`,
  `0x80B32B30` and `0x80B32B70` (`0x80B25EB0`), the `0x279` flatten variant
  (`0x80B25F40`), the direct `0x80146D00` bounce (`0x80B25FE0`) and the `0x278`
  flatten whose rate is `5.2` from `0x80B32BB0` (`0x80B26030`).
- **`0x80B260D0..0x80B264C0`, the two update passes.** The first pass over the
  candidate/done masks `+0x735`/`+0x734`, which latches `+0x74C` and starts the
  nine-frame `+0x73C` timer (`0x80B260D0`); the second pass without the effect
  or timer (`0x80B26280`); the landing retirement pass (`0x80B263E0`); and the
  three-part land condition with the `0x2B`-ability exclusion
  (`0x80B264C0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B22A20` | `fn_80b22a20` | 88 | high |
| `0x80B22BA0` | `fn_80b22ba0` | 236 | high |
| `0x80B22E00` | `fn_80b22e00` | 228 | high |
| `0x80B23540` | `fn_80b23540` | 132 | high |
| `0x80B23650` | `fn_80b23650` | 200 | high |
| `0x80B23720` | `fn_80b23720` | 364 | medium |
| `0x80B23960` | `fn_80b23960` | 304 | medium |
| `0x80B23AB0` | `fn_80b23ab0` | 248 | medium |
| `0x80B23BE0` | `fn_80b23be0` | 304 | medium |
| `0x80B23D30` | `fn_80b23d30` | 248 | medium |
| `0x80B23E30` | `fn_80b23e30` | 64 | high |
| `0x80B23E80` | `fn_80b23e80` | 128 | medium |
| `0x80B23F00` | `fn_80b23f00` | 316 | medium |
| `0x80B25830` | `fn_80b25830` | 256 | high |
| `0x80B25930` | `fn_80b25930` | 40 | high |
| `0x80B25960` | `fn_80b25960` | 184 | high |
| `0x80B25A20` | `fn_80b25a20` | 640 | high |
| `0x80B25CA0` | `fn_80b25ca0` | 100 | high |
| `0x80B25D10` | `fn_80b25d10` | 124 | high |
| `0x80B25DE0` | `fn_80b25de0` | 64 | high |
| `0x80B25E20` | `fn_80b25e20` | 144 | high |
| `0x80B25EB0` | `fn_80b25eb0` | 144 | high |
| `0x80B25F40` | `fn_80b25f40` | 152 | high |
| `0x80B25FE0` | `fn_80b25fe0` | 68 | high |
| `0x80B26030` | `fn_80b26030` | 156 | high |
| `0x80B260D0` | `fn_80b260d0` | 420 | high |
| `0x80B26280` | `fn_80b26280` | 340 | high |
| `0x80B263E0` | `fn_80b263e0` | 216 | high |
| `0x80B264C0` | `fn_80b264c0` | 220 | medium |

## Deliberate skips

- **`0x80B23480` (`FUN_80b23480`, 180 bytes).** The shared `Wm_en_pakkun`
  sway/height stepper. Its export is built around an unexplained GQR
  conversion block (`ldexpf` on the quantisation register, `unaff_GQR3`) and
  two constants that are only reached through that conversion, so the value
  fed to `0x80237BD0` cannot be proven. The four frame handlers call it by
  address (`W110_FN_80B23480`) and say so in their docs.
- **`0x80B23890` (`FUN_80b23890`, 156 bytes).** The `Wm_en_pakkun`
  state/animation selector. It computes the per-state table entry at
  `0x80B328B0`, but the export then drops that pointer from the call
  arguments, so the clip that is being bound cannot be named. Callers reach it
  by address (`W110_FN_80B23890`) with the observed `(f64, actor, state)`
  shape.
- **`0x80B250F0` (56 bytes).** The export ends in
  `halt_baddata()` after a bad instruction, so no honest body is possible.
- **`0x80B25280`, `0x80B253D0` (328 bytes each).** Six GQR conversions each;
  both bodies are dominated by the same quantisation artefacts as
  `0x80B23480`.
- **`0x80B265A0..0x80B26A10` (the `block_jump` record helpers).** These are
  contiguous with this pass but call `0x80B25D90`, whose export is two
  unrecoverable jump tables. They were left for a later pass so that no body
  in this lane points at an untranslated switch.
- **The rest of the bucket.** 49 of the bucket's 139 functions remain
  untranslated after this pass (78 before it). The nearest unfinished
  neighbours are the `0x80B228..`/`0x80B234..` GQR-heavy helpers, the
  `0x80B27980..0x80B28710` and `0x80B292D0..0x80B2A360` actors
  (`box_wood`-style world-map props) and the `0x80B265A0..0x80B26A10` tail
  noted above.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  6,128 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 29 was in
  `ledger/functions.csv` (61 of the bucket's 139 rows were already translated,
  78 were not).
- `node tools/check-lane-csv.mjs wave110` prints
  `wave110-enemies.csv: 29 rows` and `OK`.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`; the errors it still prints come from
  `src/engine/dsys.rs`, another lane's file.
- The module header was updated: a new cluster item 91, the cluster count
  (ninety to ninety-one), the function totals (2,413 to 2,442 public
  translations, 2,342 to 2,371 `fn_<addr>` bodies) and the records list.
