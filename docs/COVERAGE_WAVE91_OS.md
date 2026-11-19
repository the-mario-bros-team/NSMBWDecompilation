# Coverage - Wave 91 (`runtime::os`, `0x80250000` bucket)

## How to read this file

This page records wave 91 of the `main.dol` reconstruction for the
`runtime::os` lane: a pass over the `0x80250000` bucket, which holds 163
inventory entries. 74 of them were already merged when the lane started (the
Wave 81 and Wave 83 passes plus a handful of `runtime::bt` neighbours); the
other 89 were untranslated and 29 of them are reconstructed here.

- **Status** - `translated` means every guard, store and call printed in the
  Ghidra export has a counterpart in `src/runtime/os.rs`. All 29 rows are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export.
  `medium` means the flow is explicit but at least one callee argument list or
  field role is not recoverable and is marked `// unknown:` / `// inferred:` in
  the body.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes were re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.
- `f32`/`f64` arguments reach the matrix kernels through the floating-point
  registers. The bodies model those calls with host-side `extern "C"`
  trampolines (`wave91_matrix_*`), the same convention `src/audio/ax.rs` uses.

The machine-readable companion is `ledger/agents/wave91-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 29 | 3,424 | 14 | 15 |

Status split: 29 `translated`, 0 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **Cluster 1 - matrix initialisation, `0x80250CB0..0x80250DF0` (8 bodies).**
  Two bodies run a shape helper and identity-initialise the caller's 3x4 matrix
  only when it succeeded and the caller's flag word is non-zero: `0x80250CB0`
  picks between `0x802556B0`, `0x802562B0` and `0x80256F90` from a mode word,
  `0x80250D40` uses the single helper `0x80254910`. Four leaves return one of
  two fixed data addresses (`0x803E4500` four times, `0x803EA500` once), and
  `0x80250DF0` writes three floats into a destination matrix and then sets one
  of three flag patterns (`0x40000000`, `0x10000000` plus a `30..31` clear, or a
  `28..31` clear) from the three-way equality test and the `sda2(-0x49A8)`
  constant.
- **Cluster 2 - default state and the six matrix dispatchers,
  `0x802537E0..0x802543D0` (8 bodies).** `0x80253830` rewrites the whole
  `+0x70..+0x108` block of the render node: flag word `0x21`, eight class
  constants read from `sda2(-0x4990)..sda2(-0x4974)`, and four of its six
  arguments converted through the standard `0x43300000` `u32`-to-`double`
  bias. `0x802537E0` is its only caller and feeds it six halfwords of the
  `0x803E43E8` record. `0x802540B0` publishes the node matrix behind the
  `+0x70` bit-8 "stale" check; `0x80254110`/`0x802543D0` share one three-way
  flag tree over `0x801C1260`/`0x801C11C0`/`0x801C1350` and end with the
  publisher `0x802384F0` or the commit `0x801C9840`; `0x802541F0` uses the same
  tree over `0x801C0FD0`/`0x801C0F20`/`0x801C10D0`; `0x802542C0` resets a
  matrix to identity and overlays six words from `+0xCC`/`+0xD4`/`-+0xD0`/
  `+0xD8` and two class constants.
- **Cluster 3 - the two strided getters, `0x80258BD0..0x80258C80` (3 bodies).**
  The node owns 32 records of `0x10C` bytes at `+0xF8` and 32 records of `0x30`
  bytes at `+0x2278`. Two bodies take a signed index (bound `0..0x1F`), one
  takes the byte at `+0xF4` with no bound, and all three materialise the record
  address through the one-word leaves `0x802537D0`/`0x8025F2B0` before reading
  word 0.
- **Cluster 4 - event forwards, `0x80259110`, `0x80259280`, `0x802592E0`
  (3 bodies).** `0x80259110` fetches the `+0xF4` slot, copies its matrix and
  reports event `4`; `0x80259280` runs the slot object's `+0x14` pre-pass and
  reports `0x10004`; `0x802592E0` refreshes the node, reports through vtable
  slot `+0x20` and closes with `0x8024D5E0(4)`.
- **Cluster 5 - the sort comparators, `0x80259FC0`, `0x8025A050` (2 bodies).**
  Identical bodies over different key bytes (`+0xD0`/`+0xD1`) and vtable slots
  (`+0x28`/`+0x2C`); the unequal-key path is the branch-free
  `((diff >> 1) - (diff & b_key)) >> 31` sign trick, reproduced bit for bit.
- **Cluster 6 - five leaves, `0x8025A290`, `0x8025BE30`, `0x8025C8E0`,
  `0x8025C960`, `0x8025EEB0` (5 bodies).** A positive-size allocator release
  that returns its pointer, a five-slot virtual sweep over `+0x124..+0x134`, a
  lazy boolean setter that raises bit 0 of `+0x13C` only on a state change, a
  child cursor built from the `0x8023B300` lookup and the `+0x148` base, and
  the TV-format resource selector that reuses the already-translated
  [`fn_801be010`](../src/runtime/os.rs) and hands one of four tables to
  `0x8024D4F0`.

## What the pass adds beyond the raw bodies

1. **The render node is now readable end to end.** Wave 81 documented the
   transform node's three matrices at `+0x0C`; this pass shows that the object
   they hang from keeps its *display* matrix at `+0x30`, its flags at `+0x70`
   and its float fields at `+0xAC..+0xF0`, and that every matrix kernel is
   selected by the same `0x10`/`0x40`/`0x80` bit trio.
2. **Two data constants close a loop.** `sda2(-0x4990)` and `sda2(-0x4988)` are
   written into the node's scalar block by `0x80253830` and into the last two
   words of the matrix built by `0x802542C0`. That cross-reference is what
   proves the matrix in `0x802542C0` is a node-derived one and not a scratch
   buffer.
3. **`0x801BE010` is confirmed as the TV-format accessor.** `0x8025EEB0` tests
   its result against `1`, `2` and `5` only and maps everything else to the same
   table, which matches the pass-through arms of the accessor exactly.
4. **The `0x8023B300` lookup shape is now pinned twice.** `0x8025C960` and the
   Wave 81 `0x80258680` both pass a `+0xE8` word plus a key, and the second body
   adds the `+0x148` base of the child cursor.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80250CB0` | `fn_80250cb0` | 136 | medium |
| `0x80250D40` | `fn_80250d40` | 84 | medium |
| `0x80250DA0` | `fn_80250da0` | 12 | high |
| `0x80250DB0` | `fn_80250db0` | 16 | high |
| `0x80250DC0` | `fn_80250dc0` | 12 | high |
| `0x80250DD0` | `fn_80250dd0` | 12 | high |
| `0x80250DE0` | `fn_80250de0` | 12 | high |
| `0x80250DF0` | `fn_80250df0` | 128 | high |
| `0x802537E0` | `fn_802537e0` | 80 | high |
| `0x80253830` | `fn_80253830` | 284 | medium |
| `0x802540B0` | `fn_802540b0` | 96 | medium |
| `0x80254110` | `fn_80254110` | 220 | high |
| `0x802541F0` | `fn_802541f0` | 208 | high |
| `0x802542C0` | `fn_802542c0` | 128 | medium |
| `0x80254340` | `fn_80254340` | 132 | medium |
| `0x802543D0` | `fn_802543d0` | 200 | high |
| `0x80258BD0` | `fn_80258bd0` | 92 | high |
| `0x80258C30` | `fn_80258c30` | 60 | high |
| `0x80258C80` | `fn_80258c80` | 92 | high |
| `0x80259110` | `fn_80259110` | 120 | medium |
| `0x80259280` | `fn_80259280` | 96 | medium |
| `0x802592E0` | `fn_802592e0` | 116 | medium |
| `0x80259FC0` | `fn_80259fc0` | 144 | medium |
| `0x8025A050` | `fn_8025a050` | 144 | medium |
| `0x8025A290` | `fn_8025a290` | 64 | medium |
| `0x8025BE30` | `fn_8025be30` | 184 | medium |
| `0x8025C8E0` | `fn_8025c8e0` | 124 | high |
| `0x8025C960` | `fn_8025c960` | 188 | medium |
| `0x8025EEB0` | `fn_8025eeb0` | 240 | medium |

## Deliberate skips

The second half of the bucket is dominated by seven very large bodies
(`0x8025CA20` at 2,952 bytes, `0x8025FE30` at 1,940, `0x8025B500` at 1,552,
`0x80250E70`/`0x802529F0`/`0x80252FB0` at ~1,470 each and `0x80250710` at
1,436). Each one is a state machine whose helper table is not yet recovered, so
they were left for a later pass rather than padded with guesses.

Four medium-sized bodies were also skipped because the export itself is
ambiguous:

1. `0x802593E0` (252 bytes) — its `FUN_802DCD68()` result is read as two
   pointers from one `undefined8` (`(u64) >> 32` for the first argument, the low
   word for the constructor's second argument). Modelling that register pair as
   either a pointer pair or a struct return is not provable from the export, so
   no body was written.
2. `0x80259E60` (256 bytes) — same `undefined8` split (`0x802DCD74`) plus an
   eleven-branch swap tree on two comparator results; the pointer halves are not
   recoverable.
3. `0x8025EFB0` (272 bytes) and `0x8025F6D0` (1,020 bytes) — the same
   `undefined8` split around `0x802DCD6C`, with a placement-new style
   constructor.
4. `0x80259190` (236 bytes) — its two virtual calls print no argument lists and
   the body also journals a stack pointer through `sda(-0x49B0)`; the forward at
   `0x80259110` covers the same event path with a fully recoverable argument
   list, so this one was left out rather than half-modelled.

Every skipped address stays untranslated in `ledger/functions.csv` and can be
revisited together with the `0x802DCDxx` allocator family.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `cargo check --message-format=short` reports no diagnostic for
  `src/runtime/os.rs`; the only build errors at the time of writing belong to
  another lane's `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave91` prints `OK` with `wave91-os.csv: 29 rows`.
