# Coverage - Wave 121 (`game::enemies`, the `0x80A70000` bucket)

## How to read this file

This page records the Wave 121 `game::enemies` lane: 28 functions of the
`0x80A70000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 102 of that file. Every row of the machine
readable mirror (`ledger/agents/wave121-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 4,636 | 19 | 9 |

The bucket holds 195 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 137 were already in
`ledger/functions.csv`, leaving 58. This pass claims 28 of them in four
clusters and leaves 30 untouched.

## What this pass covers

- **The two chain actors' walk steps (`0x80A70E20..0x80A71F40`, 7 bodies).**
  The `0x868`/`0x988`-byte segmented actors keep their extra-segment count at
  `+0x540`, the first segment at `+0x544` and the next segment at `+0x238`.
  `0x80A715E0`/`0x80A71700` start the `walk_normal`/`walk_angly` clips on every
  segment and are byte-for-byte the shape of the wave-85 `fn_80a10730`, which
  fixes both the clip names (`0x80B35600`/`0x80B355F4`) and the five
  bias-encoded frame words. `0x80A70F40` rebuilds the speed pair from the
  heading, `0x80A70E20` is a stage x-band test, and `0x80A71AA0`/`0x80A71F40`
  are the two per-frame handlers (heading, ground probe, side flip, `+0x28`
  segment slot, `+0x88` drop). `0x80A71C60` spawns `Wm_en_hanapetal` and scales
  the turn word and timer by `1.5`.
- **The `daEnHimanBros_c` jump-clip block (`0x80A74570..0x80A754B0`, 8
  bodies).** Wave 16 left the shared clip player untranslated and called it by
  address; [`fn_80a74570`] is now a body, so the four arguments of every call
  site are explicit. The seven entries that follow play the class clips read
  from the REL image: `{throw_L, throw_R}` (`0x80B4B788`),
  `{jump_throw_L, jump_throw_R}` (`0x80B4B7B0`), `jump_st` (`0x80B4B830`),
  `jump_ed`, `jump_attack` and `jump_attack_ed`, plus the `walk` restart of
  `0x80A74600` and its `Turn`-descriptor query against `0x80B72D68`.
- **The `0xEB0` actor's placement helpers (`0x80A7A170..0x80A7AEF0`, 6
  bodies).** `0x80A7A170` refreshes the eight placement rows at `+0x628` from
  the two four-node groups of `0x80B2E068` (`{11,14,17,20}`, `{1,4,7,10}`),
  adds the `0x801C1160` transform with the `5.0` scale at `+0x688`, reads node
  `0x15` into `+0x61C` and stores the three deltas at `+0xD0..+0xD8`.
  `0x80A7A2C0`/`0x80A7A360`/`0x80A7A3C0` are the clip-rate speed and countdown
  helpers, `0x80A7A860` the per-side placement offset and `0x80A7AEF0` the
  `0x16`-node x-extent probe against `16.0`.
- **The `daEnIcicle_c` descriptor skeleton (`0x80A7D030..0x80A7DCC0`, 7
  bodies).** The two constant producers (`0x80357BF4`, `0x80357BB4`), the four
  48-byte `0x802DCBFC` state-destructor forwarders that front and follow the
  `0x628`-byte constructor `0x80A7DCF0`, and the class's three-phase throw
  handler `0x80A7D760`.

### State entry names

The `daEnHimanBros_c` table at `0x80A75800` (documented in section 18 of the
file) names the descriptors this block uses: `0x80B72D68` is `Turn`,
`0x80B72B68` `Attack`, `0x80B72BE8` `Jump_St`, `0x80B72C28` `JumpAttack` and
`0x80B72C68` `JumpAttack_Ed`. The jump entries translated here are the
entry-side bodies of those records.

### Why nine rows are `medium`

- **Bias-encoded constants.** The export prints the integer-to-double bias
  (`2^52 + 2^31`) as a magic `4503601774...` literal. The bias values were read
  out of `work/rels/elf/d_enemiesNP.elf` (high word `0x43300000`, low word
  `0x80000000`), so `4503601774854148.0 - bias` is exactly `4.0` and
  `4503601774854144.0 - bias` exactly `0.0`; the bodies state that in comments
  instead of repeating the artefact.
- **Dropped arguments.** `0x80A7A170` recovers its actor through the
  `0x802DCD68` prologue while every caller passes it; the Rust takes it as an
  argument, following the precedent of `fn_80a77820`. `0x80A7D760` prints one
  `0x801680E0` call with no `double`; the adjacent calls pass `0x80B2E198`
  (`0.0`), which is forwarded and marked `// inferred:`.
- **Inferred field roles.** The `+0x2CC`/`+0x2F0` class slots of
  `0x80A7D760`, the `+0x28` segment slot, the `+0x64C`/`+0x67C` placement pair
  and the `+0xEC` jump speeds are named from their position in the state
  machine, not from a symbol.
- **Sign/cosine assignment.** `0x80A70F40` uses the cosine helper `0x80237BD0`
  for `+0xE8` and the sine helper `0x80237C40` for `+0xF0`; the roles are the
  same inference already recorded for `fn_80b10be0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A70E20` | `fn_80a70e20` | 140 | medium |
| `0x80A70F40` | `fn_80a70f40` | 148 | medium |
| `0x80A715E0` | `fn_80a715e0` | 280 | high |
| `0x80A71700` | `fn_80a71700` | 280 | high |
| `0x80A71AA0` | `fn_80a71aa0` | 444 | medium |
| `0x80A71C60` | `fn_80a71c60` | 240 | high |
| `0x80A71F40` | `fn_80a71f40` | 520 | medium |
| `0x80A74570` | `fn_80a74570` | 136 | high |
| `0x80A74600` | `fn_80a74600` | 176 | medium |
| `0x80A74A20` | `fn_80a74a20` | 132 | high |
| `0x80A74DA0` | `fn_80a74da0` | 112 | high |
| `0x80A74F20` | `fn_80a74f20` | 140 | high |
| `0x80A75190` | `fn_80a75190` | 112 | high |
| `0x80A752E0` | `fn_80a752e0` | 104 | high |
| `0x80A754B0` | `fn_80a754b0` | 88 | high |
| `0x80A7A170` | `fn_80a7a170` | 324 | medium |
| `0x80A7A2C0` | `fn_80a7a2c0` | 156 | medium |
| `0x80A7A360` | `fn_80a7a360` | 96 | high |
| `0x80A7A3C0` | `fn_80a7a3c0` | 104 | high |
| `0x80A7A860` | `fn_80a7a860` | 88 | medium |
| `0x80A7AEF0` | `fn_80a7aef0` | 200 | medium |
| `0x80A7D030` | `fn_80a7d030` | 12 | high |
| `0x80A7D040` | `fn_80a7d040` | 12 | high |
| `0x80A7D2D0` | `fn_80a7d2d0` | 48 | high |
| `0x80A7D300` | `fn_80a7d300` | 48 | high |
| `0x80A7D760` | `fn_80a7d760` | 400 | medium |
| `0x80A7DC90` | `fn_80a7dc90` | 48 | high |
| `0x80A7DCC0` | `fn_80a7dcc0` | 48 | high |

## Deliberate skips

- **The other 30 addresses of the bucket were not attempted.** They lie
  outside the four clusters this pass targeted and are, in ascending order:
  `0x80A707E0` (652), `0x80A778F0` (116), `0x80A77A50` (340),
  `0x80A77BB0` (124), `0x80A77C30` (548), `0x80A78350` (616),
  `0x80A78700` (64), `0x80A79630` (236), `0x80A79750` (188),
  `0x80A79810` (360), `0x80A7A460` (312), `0x80A7A8C0` (320),
  `0x80A7AA00` (292), `0x80A7AB90` (272), `0x80A7ACA0` (252),
  `0x80A7ADA0` (220), `0x80A7B7C0` (184), `0x80A7BD30` (372),
  `0x80A7BEC0` (1,464), `0x80A7C600` (184), `0x80A7C7D0` (628),
  `0x80A7E630` (8), `0x80A7E640` (212), `0x80A7EB60` (280),
  `0x80A7ECA0` (84), `0x80A7ED20` (176), `0x80A7EE20` (128),
  `0x80A7F690` (48), `0x80A7F6C0` (48), `0x80A7FB50` (392).
  Nothing was written for them, so no ledger row claims them.
- **`0x80A7A8C0` and `0x80A7AA00` are skipped on purpose.** Both are the two
  collision-row scans behind `0x80A7AB30`, but their exports take their
  arguments from paired-single registers: the actor arrives through the
  `0x802DCD70` prologue while the coordinate pair is read from the same
  register as a float, and Ghidra prints the pair as `f29`/`ps29` spills. The
  caller `0x80A7AB30` passes `(actor, &pair)`, which contradicts the export's
  `((float *)value)[0]` reads. Rather than pick one of the two readings, no
  body was written.
- **`0x80A7E630` (8 bytes) is skipped.** Its export is `halt_baddata()`
  ("control flow encountered bad instruction data"), so there is no
  instruction sequence to reconstruct.
- **`0x80A7AB90`/`0x80A7ACA0` are skipped.** They call `0x80A7AB30` and use its
  `r3` residue as an `int` result, while the already-merged body of
  `0x80A7AB30` is typed `-> ()`. Reconstructing the residue convention would
  require changing an existing row outside this pass's claim.
- **`0x80A74570` and `0x80A71700` are no longer referenced as bare addresses.**
  Their constants in sections 15 and 18 were kept (callers still pass the
  address) but their doc comments now point at the two new bodies instead of
  saying "not translated".
- **No `StateID` table is claimed here.** The bucket holds no state table of
  its own; the records used by this pass (`0x80B72D68`, `0x80352CB4`) are
  referenced from the already-translated tables.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  4,636 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave121` reports the lane CSV clean.
- `cargo check --message-format=short` emits no error naming
  `src/game/enemies.rs`.
- The module header was updated: cluster item 102, the cluster count (one
  hundred and one to one hundred and two), the function totals (2,727 to 2,755
  public translations, 2,656 to 2,684 `fn_<addr>` bodies), the "start here"
  list and the records list.
