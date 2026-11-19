# Coverage - Wave 109 (`game::enemies`, the `0x80A40000` bucket)

## How to read this file

This page records the Wave 109 `game::enemies` lane: 26 functions of the
`0x80A40000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 90 of that file. Every row of the machine
readable mirror (`ledger/agents/wave109-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The run is one class. `daEnCrasher_c` is identified by its own state table
([`register_crasher_states`], `0x80A49C60`): the nine `fState` records it
builds are named `daEnCrasher_c::StateID_*` verbatim, and each record's
parameter block in `.data` (`0x80B42B00..0x80B42BF8`) holds the three callback
pointers of that state. This pass translated the entry and frame callbacks
those blocks point at, plus the helpers they share. No handler role is guessed.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 5,732 | 18 | 8 |

## What this pass covers

- **The state table as evidence.** Record -> `StateID` -> callbacks:
  `0x80B70DC8` `Walk` (`0x80A48B20` / `0x80A48BA0`), `0x80B70E08` `Turn`
  (`0x80A48E40`), `0x80B70E48` `Wait` (`0x80A48F20` / `0x80A48F90`),
  `0x80B70E88` `AttackJump` (`0x80A49230` / `0x80A492D0`),
  `0x80B70EC8` `AttackWalk` (`0x80A493F0` / `0x80A49490`),
  `0x80B70F08` `ChangeWalkWait` (`0x80A49750` / `0x80A497A0`) and
  `0x80B70F48` `Damege` (`0x80A49860` / `0x80A498C0`). The third callback of
  every record (the exit/leave tail) was already translated by earlier waves.
- **`0x80A47B90..0x80A47EB0`, the child/attack bridge.** The `0x1A2` event
  responder that pushes the event owner by half the hit value, raises sound
  `0x2D1` on a strike frame and re-arms the actor (`0x80A47B90`); the `dead`
  clip thunk (`0x80A47CB0`); the eight-word attack record copied from the child
  into `+0x414..+0x432` (`0x80A47CC0`); the child release that either bounces
  back to `Damege` for the first two hits or finalises the record and binds the
  `dead` clip (`0x80A47D90`); and the one-line frame hook (`0x80A47EB0`).
- **`0x80A48520..0x80A48970`, the shared helpers.** The five-clip binder over
  the names `walk`, `wait`, `run`, `damage`, `dead` with the `4.0` walk/run
  switch blend (`0x80A48520`); the once-only nine-entry sound table
  `0x174..0x17A, 0x17A, 0x17A` and its per-child-animation player
  (`0x80A48620`); the frame-`0`/frame-`12` sound tick `0x218` (`0x80A48730`);
  the attack-state strike-frame predicate (`0x80A487E0`); the `256.0` x `80.0`
  volume query (`0x80A488B0`); and the pending side-angle test (`0x80A48970`).
- **`0x80A48A20..0x80A498C0`, the callbacks.** The tilt/aim step
  (`0x80A48A20`), the two-phase walk-blend scalar (`0x80A48A80`), the seven
  entry initialisers and the five frame handlers: ground probes, the `48.0`
  lateral clamp, the `0x20..0x5F` wander timer, the four-phase `0x400`-step
  yaw sweep around `+0x680`/`+0x682`, the `-2.0` attack recoil, the walk-blend
  values `0.7`/`1.0`, the `AttackWalk` side-change debounce, and the packaged
  record id `0x11804101`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A47B90` | `fn_80a47b90` | 284 | medium |
| `0x80A47CB0` | `fn_80a47cb0` | 8 | high |
| `0x80A47CC0` | `fn_80a47cc0` | 204 | medium |
| `0x80A47D90` | `fn_80a47d90` | 288 | high |
| `0x80A47EB0` | `fn_80a47eb0` | 36 | high |
| `0x80A48520` | `fn_80a48520` | 244 | high |
| `0x80A48620` | `fn_80a48620` | 260 | high |
| `0x80A48730` | `fn_80a48730` | 164 | high |
| `0x80A487E0` | `fn_80a487e0` | 208 | high |
| `0x80A488B0` | `fn_80a488b0` | 132 | medium |
| `0x80A48970` | `fn_80a48970` | 168 | high |
| `0x80A48A20` | `fn_80a48a20` | 92 | medium |
| `0x80A48A80` | `fn_80a48a80` | 148 | high |
| `0x80A48B20` | `fn_80a48b20` | 112 | high |
| `0x80A48BA0` | `fn_80a48ba0` | 596 | medium |
| `0x80A48E40` | `fn_80a48e40` | 220 | medium |
| `0x80A48F20` | `fn_80a48f20` | 88 | high |
| `0x80A48F90` | `fn_80a48f90` | 672 | medium |
| `0x80A49230` | `fn_80a49230` | 140 | high |
| `0x80A492D0` | `fn_80a492d0` | 288 | high |
| `0x80A493F0` | `fn_80a493f0` | 128 | high |
| `0x80A49490` | `fn_80a49490` | 692 | medium |
| `0x80A49750` | `fn_80a49750` | 56 | high |
| `0x80A497A0` | `fn_80a497a0` | 192 | high |
| `0x80A49860` | `fn_80a49860` | 80 | high |
| `0x80A498C0` | `fn_80a498c0` | 232 | high |

## Deliberate skips

- **`0x80A48940` (`FUN_80a48940`, 48 bytes).** The export ends in
  `/* WARNING: Bad instruction - Truncating control flow here */` followed by
  `halt_baddata()`. Only the first half of the body survives (`+0x674 += delta`
  clamped to `1.0`); the rest of the function is missing from the export, so no
  body was written. The two callers reach it through the address constant
  `W109_FN_80A48940`.
- **The rest of the bucket.** 52 of the bucket's 189 functions remain
  untranslated after this pass. The nearest unfinished neighbours are
  `0x80A45DB0`, `0x80A46200` and `0x80A46BF0` (earlier state handlers of the
  same bucket) and the `crow` block `0x80A4A8D0..0x80A4BD70`, whose exports
  name `g3d/crow.brres` and the `fly`/`attack1`/`attack2` clips.
- **`0x80A4B0B0`, `0x80A4B140`, `0x80A4B420`, `0x80A4B740`, `0x80A4BBD0`.**
  Five functions of the `crow` block carry `unaff_GQR0` conversion artefacts
  around the whole body. They were outside the chosen run and were not
  attempted.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  5,732 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 26 was in
  `ledger/functions.csv` (111 of the bucket's 189 rows were already translated,
  78 were not).
- `node tools/check-lane-csv.mjs wave109` prints
  `wave109-enemies.csv: 26 rows` and `OK`.
- `cargo check --message-format=short` emits no error naming
  `src/game/enemies.rs`; the errors it still prints come from
  `src/game/player.rs`, another lane's file.
- The module header was updated: a new cluster item 90, the cluster count
  (eighty-nine to ninety), the function totals (2,387 to 2,413 public
  translations, 2,316 to 2,342 `fn_<addr>` bodies) and the records list.
