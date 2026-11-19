# Coverage - Wave 112 (`game::enemies`, the `0x80A00000` bucket)

## How to read this file

This page records the Wave 112 `game::enemies` lane: 29 functions of the
`0x80A00000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 93 of that file. Every row of the machine
readable mirror (`ledger/agents/wave112-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 4,936 | 15 | 13 |

The remaining body is `low`: `0x80A0DB40` ends in an indirect jump that the
export cannot recover, so its row is `partial`.

## What this pass covers

- **`0x80A00200..0x80A00240`, two deleting helpers of the base enemy
  framework.** Both run the heap release `0x802B9280` for a positive mode and
  return the pointer so the caller can chain the free. `0x80A00200` is also the
  per-element hook that [`fn_80a70b00`] hands to the `0x802DC7E0` container.
- **`0x80A08CB0..0x80A09880`, ten steps of a rider-carrying actor.** The list
  head is `+0x588` (payload at `+0x04`, next at `+0x0C`); a rider counts while
  its `+0x38C` byte is `1`. The block is the `+0x578` sub-object release, the
  rider probe that adopts the rider's player slot (`+0x567`/`+0x38D`), the
  four-bit player mask at `+0x568`, and the ride-height state machine over
  `+0xEC` (offset), `+0x550` (ride height), `+0x554`/`+0x558` (steps),
  `+0x566` (rider count), `+0x56C` and `+0x5B4` (flags), with the nine state
  descriptors `0x80B6EA18..0x80B6EC18` and the rider-impact effect through the
  rider's `vtable +0x3FC`. Ghidra prints the shared probe `0x80A08DC0` without
  arguments at several call sites; its own body reads `this` in `r3`, so every
  call passes the actor.
- **`0x80A0D5B0..0x80A0E3B0`, seventeen steps of the `kuriboMini` (Mini Goomba)
  actor.** The binder loads `g3d/kuriboMini.brres`, resolves the `kuriboMini`
  node and fills the `+0x524`/`+0x540`/`+0x544`/`+0x584` holders that the
  `0x618`-byte constructor of wave 83 ([`fn_80a0d350`]) creates, so the block
  and that constructor belong to one class. Covered here: the `+0x7C` matrix
  refresh from the `+0x100`/`+0x102`/`+0x104` angle triple and the `+0x464`
  scale triple; five sound wrappers for the events `0x1F9` and `0x1FA`; the
  contact handler with its double vtable query; the contact callback that plays
  `0x1FB` and reads the attacker height; the attacker-record copy into
  `+0x414`; the six-button stomp counter at `+0x5F8..+0x608`; the `+0x1EC`
  ground query; and the turn, walk and idle frame steps that emit
  `0x80B6EE18`/`0x80B6EE58`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A00200` | `fn_80a00200` | 64 | high |
| `0x80A00240` | `fn_80a00240` | 64 | high |
| `0x80A08CB0` | `fn_80a08cb0` | 40 | high |
| `0x80A08DC0` | `fn_80a08dc0` | 124 | medium |
| `0x80A08E40` | `fn_80a08e40` | 152 | high |
| `0x80A08FF0` | `fn_80a08ff0` | 116 | high |
| `0x80A090A0` | `fn_80a090a0` | 180 | medium |
| `0x80A091B0` | `fn_80a091b0` | 248 | medium |
| `0x80A09310` | `fn_80a09310` | 288 | medium |
| `0x80A09460` | `fn_80a09460` | 348 | medium |
| `0x80A09650` | `fn_80a09650` | 284 | medium |
| `0x80A09880` | `fn_80a09880` | 300 | medium |
| `0x80A0D5B0` | `fn_80a0d5b0` | 268 | high |
| `0x80A0D8A0` | `fn_80a0d8a0` | 80 | high |
| `0x80A0D8F0` | `fn_80a0d8f0` | 80 | high |
| `0x80A0D940` | `fn_80a0d940` | 80 | high |
| `0x80A0D990` | `fn_80a0d990` | 80 | high |
| `0x80A0D9E0` | `fn_80a0d9e0` | 80 | high |
| `0x80A0DA30` | `fn_80a0da30` | 144 | medium |
| `0x80A0DAC0` | `fn_80a0dac0` | 116 | high |
| `0x80A0DB40` | `fn_80a0db40` | 68 | low (partial) |
| `0x80A0DCC0` | `fn_80a0dcc0` | 264 | medium |
| `0x80A0DDD0` | `fn_80a0ddd0` | 276 | high |
| `0x80A0DEF0` | `fn_80a0def0` | 16 | medium |
| `0x80A0DF00` | `fn_80a0df00` | 484 | medium |
| `0x80A0E0F0` | `fn_80a0e0f0` | 100 | high |
| `0x80A0E190` | `fn_80a0e190` | 116 | high |
| `0x80A0E220` | `fn_80a0e220` | 328 | medium |
| `0x80A0E3B0` | `fn_80a0e3b0` | 148 | medium |

## Deliberate skips

37 addresses of the bucket stay untranslated. The ones this pass examined and
rejected are:

- **`0x80A0E160` (36 bytes), `0x80A01700` (80 bytes).** The export ends both in
  `halt_baddata()` after a bad instruction, so no honest body exists.
- **`0x80A0E450` (276 bytes).** Every path of this ride entry calls
  `0x80A0E160`, the truncated function above, so the state it hands over to
  cannot be reconstructed.
- **`0x80A0E580` (968 bytes).** Wrapped in a GQR conversion block
  (`unaff_GQR3`, `ldexpf`) whose quantisation is part of the value written back
  to the actor, exactly like the `0x80B23480` skip recorded in
  `docs/COVERAGE_WAVE110_ENEMIES.md`.
- **`0x80A00D00` (424 bytes).** Its four arguments arrive only through the GQR
  block (`unaff_GQR0`, `ldexpf`), so the values the four-rider follow step
  computes cannot be expressed without inventing the quantisation.
- **`0x80A06220`, `0x80A06760`, `0x80A06800`, `0x80A06B00`, `0x80A07080`,
  `0x80A07160`.** These neighbours of the `gessoMini` and BaramakiGesso blocks
  are decompiled with dropped arguments (`FUN_80a06190()`, `func_0x80162d00()`,
  `FUN_80a0b7f0()` without a `this`), so the callee identity and the argument
  order would both be guesses.
- **`0x80A0B8A0`, `0x80A0BD20`.** The recursive chain walker at `0x80A0B8A0` is
  declared with one argument but recurses with two, and the neighbouring
  `0x80A0BD20` calls it with a node where the declared body expects the actor;
  the pair is left for a pass that can establish the walker's real prototype.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  4,936 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 29 was in
  `ledger/functions.csv` (66 of the bucket's 212 rows were untranslated before
  this pass, 37 after it).
- `node tools/check-lane-csv.mjs wave112` prints `wave112-enemies.csv: 29 rows`
  and `OK`.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 93, the cluster count
  (ninety-two to ninety-three), the function totals (2,471 to 2,500 public
  translations, 2,400 to 2,429 `fn_<addr>` bodies) and the records list.
