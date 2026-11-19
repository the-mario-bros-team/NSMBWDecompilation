# Coverage - Wave 69 (enemies: `0x80AC4410..0x80AC47D0`, `0x80AC6BA0..0x80AC6F80`, `0x80ACD5E0..0x80ACD9A0`)

## How to read this file

This page records the Wave 69 `game::enemies` lane: three contiguous runs in the
`0x80AC0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real `pub unsafe fn`
body in that file, and the `evidence` column names the Ghidra C export the body
was reconstructed from. The CSV that mirrors this table is
`ledger/agents/wave69-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 2,588 | 14 | 15 |

## What this pass covers

The bucket `0x80AC0000` holds 177 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 29 of them were already in
`ledger/functions.csv` when this lane started (the wave-55 run
`0x80AC0280..0x80AC1ED0` plus one address claimed by an earlier wave). 148 were
untranslated. This pass claims 29 of those and leaves 119.

- **`0x80AC4410` skeleton (`0x80AC4410..0x80AC47A0`).** Another copy of the
  five-entry state-descriptor shape already seen in waves 46, 54, 55 and 68: a
  normaliser pair ([`fn_80ac4410`], [`fn_80ac4470`]), two constant producers
  ([`fn_80ac44d0`] = `0x80357BF4`, [`fn_80ac44e0`] = `0x80352DB4`), the
  `+0x30` chain walker [`fn_80ac45d0`], the `:`-suffix name predicate
  [`fn_80ac46b0`] and the two 48-byte deleting-destructor forwarders
  [`fn_80ac4770`]/[`fn_80ac47a0`].
- **`0x80AC47D0` constructor.** The `0x880`-byte actor that this skeleton
  fronts: vtable `0x80B592A8`, the non-standard base constructor `0x80037EF0`
  (the one [`alloc_toge_tekkyu_actor`] uses) and the identical
  `+0x7DC`/`+0x7F8`/`+0x7FC`/`+0x83C` model layout. The binder at `0x80AC4B50`
  opens `g3d/met.brres` and the `met`-family node names into those holders, so
  this is a smaller sibling of the `0x9E0`-byte `daEnTogeTekkyu_c`.
- **`0x80AC6BA0` skeleton (`0x80AC6BA0..0x80AC6F50`).** Another copy of the
  skeleton, this one with **four** constant producers
  ([`fn_80ac6c60`] = `0x80353AE4`, [`fn_80ac6c70`] = `0x80353B24`,
  [`fn_80ac6c80`] = `0x80353AA4`, [`fn_80ac6c90`] = `0x80353A64`), plus the same
  normaliser pair, walker, predicate and two forwarders. Two of those four
  descriptors (`0x80353AA4`, `0x80353B24`) are also compared by the
  `Wm_en_quicksand` block, so they are shared base states.
- **`0x80AC6F80` constructor.** The `0x770`-byte `g3d/huhu.brres` actor:
  temporary vtable `0x80304E10` rewritten to `0x80B598C8`, holders at `+0x524`,
  `+0x544`, `+0x584` (`0x80B330E0`), a `0x803295D8`/`0x80329548` pair at
  `+0x5C0`, the `0x80329958` helper at `+0x5F0`, and the flag/counter block at
  `+0x704..+0x714`. The binder at `0x80AC82B0` loads `g3d/huhu.brres` into
  exactly those holders and attaches a `dead_02` clip at `+0x5BC`.
- **`0x80ACD5E0` skeleton (`0x80ACD5E0..0x80ACD970`).** A further copy with two
  producers: `0x80357BF4` (shared with [`fn_80ac44d0`] and [`fn_80ac0370`]) and
  `0x80357BB4` (shared with [`fn_80ac0360`]).
- **`0x80ACD9A0` constructor.** The `0x608`-byte sibling of
  [`alloc_rail_poly_child_actor`] (`0x80ACE540`, `0x600` bytes,
  `daEnRailPolyChild_c`): the store sequence is identical word for word and
  differs only in the allocation size and the final vtable (`0x80B5A808`
  instead of `0x80B5ABC0`). The neighbouring handlers bind the
  `net_walk`/`net_turn` clips (`0x80ACDB00` binds `net_walk2` at `+0x544`), the
  same animation family as the `0x80ACE540` actor.

### Identity evidence

- `0x80AC4B50` is literal: it calls `0x800DF180` with `s_g3d_met_brres` and
  `0x8023A0B0` with the node-name word at `0x80B5926C`; the actor it fills is
  the one built by [`fn_80ac47d0`].
- `0x80AC82B0` is literal: `s_g3d_huhu_brres_80b59c7c`, the node word at
  `0x80B59C94` and the `s_dead_02` clip at `0x80B59CA0`, all into the offsets
  [`fn_80ac6f80`] initialises.
- `0x80ACDB00` and `0x80ACE900`/`0x80ACEB70`/`0x80ACED20` are literal:
  `s_net_walk2`, `s_net_walk1`, `s_net_turn_st` and `s_net_turn_ed`.
- The class names `daEnTogeTekkyu_c` and `daEnRailPolyChild_c` are **not**
  printed in this run. They come from the already-translated constructors
  `0x80B146E0` and `0x80ACE540`, which share the base constructor and the exact
  store sequence; both docs say `// inferred:` where that matters.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AC4410` | `fn_80ac4410` | 88 | high |
| `0x80AC4470` | `fn_80ac4470` | 92 | high |
| `0x80AC44D0` | `fn_80ac44d0` | 12 | high |
| `0x80AC44E0` | `fn_80ac44e0` | 12 | high |
| `0x80AC45D0` | `fn_80ac45d0` | 224 | medium |
| `0x80AC46B0` | `fn_80ac46b0` | 136 | medium |
| `0x80AC4770` | `fn_80ac4770` | 48 | medium |
| `0x80AC47A0` | `fn_80ac47a0` | 48 | medium |
| `0x80AC47D0` | `fn_80ac47d0` | 132 | medium |
| `0x80AC6BA0` | `fn_80ac6ba0` | 88 | high |
| `0x80AC6C00` | `fn_80ac6c00` | 92 | high |
| `0x80AC6C60` | `fn_80ac6c60` | 12 | high |
| `0x80AC6C70` | `fn_80ac6c70` | 12 | high |
| `0x80AC6C80` | `fn_80ac6c80` | 12 | high |
| `0x80AC6C90` | `fn_80ac6c90` | 12 | high |
| `0x80AC6D80` | `fn_80ac6d80` | 224 | medium |
| `0x80AC6E60` | `fn_80ac6e60` | 136 | medium |
| `0x80AC6F20` | `fn_80ac6f20` | 48 | medium |
| `0x80AC6F50` | `fn_80ac6f50` | 48 | medium |
| `0x80AC6F80` | `fn_80ac6f80` | 248 | medium |
| `0x80ACD5E0` | `fn_80acd5e0` | 88 | high |
| `0x80ACD640` | `fn_80acd640` | 92 | high |
| `0x80ACD6A0` | `fn_80acd6a0` | 12 | high |
| `0x80ACD6B0` | `fn_80acd6b0` | 12 | high |
| `0x80ACD7A0` | `fn_80acd7a0` | 224 | medium |
| `0x80ACD880` | `fn_80acd880` | 136 | medium |
| `0x80ACD940` | `fn_80acd940` | 48 | medium |
| `0x80ACD970` | `fn_80acd970` | 48 | medium |
| `0x80ACD9A0` | `fn_80acd9a0` | 204 | medium |

## Deliberate skips

- **The actor bodies behind each constructor.** `0x80AC47D0`'s siblings
  (`0x80AC4B50..0x80AC5D00`), `0x80AC6F80`'s siblings
  (`0x80AC7790..0x80AC8A90`) and `0x80ACD9A0`'s siblings
  (`0x80ACDB00..0x80ACEE40`) are the model binders, contact tests and per-frame
  steps of the three classes. They are clean exports, but the lane budget of
  25-30 bodies was spent on the descriptor skeletons, which are shared by the
  whole module and were the blocking gap for this bucket. No address in those
  ranges is claimed here.
- **`0x80AC0840` (396 bytes) and the head of the bucket**
  (`0x80AC0750`, `0x80AC1800`, `0x80AC2220` and the `0x80AC2370..0x80AC3A60`
  run) belong to other actor blocks and were left for a later contiguous run.
- **`0x80ACD9A0`'s `+0x5F0` field.** It is not written by the constructor; the
  handlers read it as a mode flag. The sub-object installed at `+0x5D0` by
  `0x8016A5E0` owns that storage, but which component writes it is not
  established by any export, so the body does not guess.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  2,588 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave69` reports `OK` with
  `wave69-enemies.csv: 29 rows`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
