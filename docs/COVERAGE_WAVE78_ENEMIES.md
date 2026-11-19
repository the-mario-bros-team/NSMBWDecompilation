# Coverage - Wave 78 (enemies: `0x80A50000` bucket, part 3)

## How to read this file

This page records the Wave 78 `game::enemies` lane: a third pass over the
`0x80A50000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave78-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 2,728 | 10 | 11 |

The bucket holds 181 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 44 were already in
`ledger/functions.csv` (the `daEnFireSnake_c` registration/callback block, the
`daEnFixationIcicle_c` and `daEnIcicle_c` constructors and contacts, the
`daEnGabon_c` registration callbacks and constructor, and the `daEnGabon_c`
state steps), leaving 137. This pass claims 27 and leaves 110. Six of the new
rows are `low` confidence - all six are the 48-byte `0x802DCBFC` forwarders,
whose hidden-`r12` this-adjust is not modelled; the rest are `high` or
`medium`.

## What this pass covers

Four short, contiguous runs chosen for a shared field layout.

- **Run 1 (`0x80A50430..0x80A50780`, seven bodies).** A five-entry
  state-descriptor skeleton plus the actor constructor it fronts: the deleting
  destructor (`0x8015F800` then `0x802B9280` when the mode is positive), the
  constant producer returning `0x80358084`, the five-step `+0x30` owner-chain
  walker, the `:`-suffix name predicate (`0x802E1C40`/`0x802E1AB4`), the two
  `0x802DCBFC` forwarders and the `0x6E8`-byte constructor (intermediate vtable
  `0x80303F48`, model block `+0x524`, cleared word `+0x540`, owner object
  `+0x544`, animation holder `+0x584` with `0x80B330E0`, `+0x600` block,
  `+0x6D8` sub-object with the `+0x6DC` back-pointer and class vtable
  `0x80B4463C`).
- **Run 2 (`0x80A50F30..0x80A51280`, seven bodies).** A byte-identical copy of
  Run 1 whose only difference is the final class vtable, `0x80B44A14`.
- **Run 3 (`0x80A52630..0x80A52980`, seven bodies).** A third copy of the
  skeleton whose constructor builds a `0x5E0`-byte object: intermediate vtable
  `0x80303658`, the same `+0x524`/`+0x544`/`+0x584` holder chain, a cleared
  `+0x5D0` word and class vtable `0x80B451E8`.
- **Run 4 (`0x80A52AE0..0x80A52D60`, six bodies).** That `0x5E0`-byte actor's
  helpers: the `s_obj_doorA` clip binder (`0x8023A0B0` lookup, `0x801651F0`
  attach with weight 1, `0x80169F80`/`0x801680E0` at `0.0f`), the `1.0f` rate
  setter that also writes the `+0x5B8` phase byte 3, the `0.0f` rate/blend
  reset, the `0x28D` sound request through `0x80198C30`, the `0x80353090`
  class test through the `+0x394 -> +0x28 -> +0x14` virtual chain, and the full
  destructor (`0x80168D80`, `0x80169D20(-1)`, `0x80069060(-1)`,
  `0x80095130(actor, 0)`, then `0x80162920` when the mode is positive).

### Identity evidence

- **All three skeletons return the same record.** The 12-byte producers at
  `0x80A50490`, `0x80A50F90` and `0x80A52690` all return `0x80358084`, the
  same word returned by `0x80AC0350` in an unrelated bucket. It is therefore a
  shared base-class `fState` record and does **not** name any of the three
  classes. No class name is invented for them.
- **The `0x5E0` constructor and its helpers agree on the object.** The
  constructor clears `+0x5D0`, the binder touches `+0x540`/`+0x544`/`+0x584`,
  the rate setter writes `+0x5B8`, and the destructor releases `+0x524`,
  `+0x544` and `+0x584`. That layout is the same holder chain the `0x6E8`
  constructors build, so the field roles carry over; only the object size and
  the class vtables differ.
- **The class test is a duplicate.** `0x80A52D10` has the same
  `+0x394 -> vtable +0x28 -> object vtable +0x14` shape and the same
  `0x80353090` argument as the already-translated `0x80AB0310`, so its doc
  points at that body instead of inventing a new role.
- **The float words were read, not guessed.** `0x80B2CCA4` is `0.0f` and
  `0x80B2CCA8` is `1.0f` in the REL image; the bodies load them through
  `read_f32` and the docs record the observed value.
- **The 48-byte forwarders** are the same `0x802DCBFC(param_2, param_2,
  param_1)` bodies already documented in this module; their hidden `r12`
  this-adjust is not modelled and each doc says so.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A50430` | `fn_80a50430` | 92 | high |
| `0x80A50490` | `fn_80a50490` | 12 | high |
| `0x80A50580` | `fn_80a50580` | 224 | medium |
| `0x80A50660` | `fn_80a50660` | 136 | medium |
| `0x80A50720` | `fn_80a50720` | 48 | low |
| `0x80A50750` | `fn_80a50750` | 48 | low |
| `0x80A50780` | `fn_80a50780` | 168 | medium |
| `0x80A50F30` | `fn_80a50f30` | 92 | high |
| `0x80A50F90` | `fn_80a50f90` | 12 | high |
| `0x80A51080` | `fn_80a51080` | 224 | medium |
| `0x80A51160` | `fn_80a51160` | 136 | medium |
| `0x80A51220` | `fn_80a51220` | 48 | low |
| `0x80A51250` | `fn_80a51250` | 48 | low |
| `0x80A51280` | `fn_80a51280` | 168 | medium |
| `0x80A52630` | `fn_80a52630` | 92 | high |
| `0x80A52690` | `fn_80a52690` | 12 | high |
| `0x80A52780` | `fn_80a52780` | 224 | medium |
| `0x80A52860` | `fn_80a52860` | 136 | medium |
| `0x80A52920` | `fn_80a52920` | 48 | low |
| `0x80A52950` | `fn_80a52950` | 48 | low |
| `0x80A52980` | `fn_80a52980` | 156 | medium |
| `0x80A52AE0` | `fn_80a52ae0` | 120 | high |
| `0x80A52BC0` | `fn_80a52bc0` | 64 | high |
| `0x80A52C00` | `fn_80a52c00` | 76 | high |
| `0x80A52CC0` | `fn_80a52cc0` | 80 | medium |
| `0x80A52D10` | `fn_80a52d10` | 80 | medium |
| `0x80A52D60` | `fn_80a52d60` | 136 | high |

## Deliberate skips

- **The fourth skeleton copy, `0x80A51A30..0x80A51D80`** (seven bodies:
  92/12/224/136/48/48/168). It is byte-identical to Runs 1 and 3 apart from
  its class vtable `0x80B44DEC`, so it adds no new evidence. It was left for
  the next pass to keep this lane inside its 25-30 body budget.
- **`0x80A521F0`** (140 bytes) is a clean export, but it is the only function
  in the bucket that reads the `+0x5E8`/`+0x5EC` fields. The object that owns
  those fields has no other claimed body in this run, so its role (a
  stack-vector offset correction through `0x80152480`) would be guesswork. It
  is recorded here rather than claimed with an invented class.
- **The rest of the bucket from `0x80A52D70` onward** (`0x80A53710..0x80A54120`,
  `0x80A54C00..0x80A55DE0`, `0x80A56470..0x80A56DB0`,
  `0x80A570A0..0x80A59F00`, and the isolated `0x80A5C660`, `0x80A5D2D0`,
  `0x80A5D300`, `0x80A5DBC0`, `0x80A5E1A0`, `0x80A5EB00`, `0x80A5F7B0` and
  `0x80A5F7E0`) remains untranslated. Those are ordinary clean exports; they
  were outside this lane's budget and are left for a later pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  2,728 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 44 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave78` reports `wave78-enemies.csv: 27 rows`,
  27 unique addresses and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`; the only error is in `src/game/bases.rs`, another
  lane.
