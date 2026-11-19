# Coverage - Wave 114 (`game::enemies`, the `0x80AC0000` bucket)

## How to read this file

This page records the Wave 114 `game::enemies` lane: 27 functions of the
`0x80AC0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 95 of that file. Every row of the machine
readable mirror (`ledger/agents/wave114-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 4,384 | 19 | 8 |

Four of the eight `medium` rows are `partial`: the 48-byte
`0x802DCBFC` destructor forwarders carry a hidden `r12` this-adjust pointer
that neither `platform::call3` nor the Ghidra export can express. They are
recorded as `partial/medium` exactly as wave 13 recorded the five identical
forwarders at `0x80A6C810..0x80A6E4A0`.

## What this pass covers

- **`0x80AC9C20..0x80ACA1D0`, the huhu damage states.** The wave-102 huhu actor
  (section 83) owns five hurt/death handlers and this pass finishes four of
  them plus their two frame tails: the `dead_01` entry with its
  `Wm_en_huhudamage01` effect and sound `0x223`, the `dead_02` entry that
  copies the contact record (`+0x414`/`+0x418`/`+0x41C`/`+0x420`) into the
  motion words and re-derives the side from the sign of `+0x102`, the second
  `dead_02` entry with the integer animation rate, and the two frame steps that
  step `+0x102` towards the per-side `0x80B30600`/`0x80B30610` limits with a
  `0x600` step before retiring through `0x80162510` / `0x80064D10`.
- **`0x80ACA9A0..0x80ACB990`, the `mouse` chain actor.** `0x80ACAB20`
  allocates the `0x5D8`-byte object (vtable `0x80B5A320`), `0x80ACACE0` binds
  `g3d/mouse.brres`, resolves the `mouse` node and starts the default clip, and
  `0x80ACADD0` is the clip binder every later entry calls (`+0x5D7` index
  cache, `0x80B30628` name table). On top of that: the shared ground-tilt step
  `0x80ACAE70`, the four-name predicate `0x80ACAED0`, the two `+0x5C4`/`+0x5C8`
  list walks and the release helper (`0x80ACB430`, `0x80ACB540`,
  `0x80ACCF90`), the seven-word contact template with sound `0x17B`
  (`0x80ACB690`) and the main step `0x80ACB990`.
- **`0x80ACC150..0x80ACCCB0`, the family's clip and contact entries.** The
  contact dispatcher (`0x80ACC150`) forwards contact classes `1` and `3` to the
  hit responder with the `+0xE8`/`0xEC` velocity pair, and four entries bind
  clip `0`/`1` through `0x80ACADD0` with the two shared `0x80B30644`/`0x80B30648`
  blend constants.
- **`0x80ACE3C0..0x80ACECB0`, the closing block of the next family.** A second
  copy of the `0x8015F800`/`0x802B9280` detach/re-arm helper, its two 48-byte
  destructor forwarders, and the `+0x548` holder step that switches to state
  `0x80B762F0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AC9C20` | `fn_80ac9c20` | 200 | high |
| `0x80AC9D00` | `fn_80ac9d00` | 84 | high |
| `0x80AC9D60` | `fn_80ac9d60` | 408 | medium |
| `0x80AC9F10` | `fn_80ac9f10` | 212 | high |
| `0x80AC9FF0` | `fn_80ac9ff0` | 460 | medium |
| `0x80ACA1D0` | `fn_80aca1d0` | 140 | high |
| `0x80ACA9A0` | `fn_80aca9a0` | 88 | high |
| `0x80ACAAC0` | `fn_80acaac0` | 48 | medium |
| `0x80ACAAF0` | `fn_80acaaf0` | 48 | medium |
| `0x80ACAB20` | `fn_80acab20` | 132 | high |
| `0x80ACACE0` | `fn_80acace0` | 232 | high |
| `0x80ACADD0` | `fn_80acadd0` | 156 | high |
| `0x80ACAE70` | `fn_80acae70` | 96 | high |
| `0x80ACAED0` | `fn_80acaed0` | 300 | high |
| `0x80ACB430` | `fn_80acb430` | 128 | high |
| `0x80ACB540` | `fn_80acb540` | 132 | high |
| `0x80ACB690` | `fn_80acb690` | 348 | high |
| `0x80ACB990` | `fn_80acb990` | 276 | medium |
| `0x80ACC150` | `fn_80acc150` | 172 | medium |
| `0x80ACC4B0` | `fn_80acc4b0` | 96 | high |
| `0x80ACCB80` | `fn_80accb80` | 112 | high |
| `0x80ACCDB0` | `fn_80accdb0` | 96 | high |
| `0x80ACCF90` | `fn_80accf90` | 132 | high |
| `0x80ACE3C0` | `fn_80ace3c0` | 88 | high |
| `0x80ACE4E0` | `fn_80ace4e0` | 48 | medium |
| `0x80ACE510` | `fn_80ace510` | 48 | medium |
| `0x80ACECB0` | `fn_80acecb0` | 104 | high |

## Deliberate skips

The bucket had 63 untranslated addresses before this pass and 36 after it.
The neighbours this pass examined and rejected are:

- **`0x80AC9AA0` (240 bytes) and `0x80AC95D0` (540 bytes).** Both are huhu
  frame steps that call the attack wrappers `0x80AC9230`/`0x80AC9310` with a
  `double` rate Ghidra drops from the call site. Wave 102 skipped them for that
  reason and this pass keeps that decision: the only value the export offers is
  the `0.0` the sibling entries use, and the real rate cannot be recovered.
- **`0x80AC9870` (464 bytes).** The body is bracketed by a mutex pair
  (`0x802DCD74`/`0x802DCDC0`) whose arguments and return halves are printed
  without operands; the actor it locks is therefore unknown.
- **`0x80ACB000` (180 bytes).** `func_0x80162d00()` is printed with no
  argument, although every other call site of that routine passes an actor id,
  so the predicate cannot be reconstructed. `0x80ACB430` and `0x80ACB540`
  therefore call `0x80ACB000` through its raw address; the call sites
  themselves are exact.
- **`0x80ACB130` (196 bytes), `0x80ACB4B0` (140 bytes), `0x80ACB5D0` (188
  bytes).** All three take a 64-bit mutex/lock result and use only one half,
  which the export renders as an untyped word pair. `0x80ACB990` calls
  `0x80ACB5D0` through its raw address instead.
- **`0x80ACB280` (172 bytes).** The export stores the actor's first word (its
  vtable pointer) into `+0x5C0` and calls `0x80ACB000` with no argument, so the
  slot bookkeeping cannot be trusted.
- **`0x80ACB330` (252 bytes), `0x80ACB690`'s neighbour `0x80ACC520` (492
  bytes), `0x80ACC740` (388 bytes), `0x80ACC8F0` (652 bytes), `0x80ACDB00`
  (148 bytes).** Every one is wrapped in a GQR conversion block
  (`unaff_GQR0`, `ldexpf`) whose quantisation feeds the values written back to
  the actor, exactly like the five skips recorded in
  `docs/COVERAGE_WAVE113_ENEMIES.md`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  4,384 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave114` prints
  `wave114-enemies.csv: 27 rows` and `OK` for this lane.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 95, the cluster count
  (ninety-four to ninety-five), the function totals (2,527 to 2,554 public
  translations, 2,456 to 2,483 `fn_<addr>` bodies) and the records list.
