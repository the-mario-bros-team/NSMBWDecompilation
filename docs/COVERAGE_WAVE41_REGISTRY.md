# Coverage - Wave 41 (resource::registry, 0x80160000 bucket)

## How to read this file

Each row is one real `pub unsafe fn` body in `src/resource/registry.rs`. The
address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 30 | 1716 | 21 | 9 |

## What this pass covers

- Handle lifecycle and vtable use (`0x80160020`-`0x801602D0`, 6 bodies): the
  flag probe that dispatches vtable slot `+0x20`, the 0x48-byte constructor
  that installs `0x80328AE8` at `+0x0C`, the destructor, the publish/allocate
  routine that fills the five `r13 - 0x52xx` scratch slots, the minima updater
  and the `u16` checksum loop.
- Handle lock helpers (`0x80160930`-`0x801609D0`, 3 bodies): the pointer-taking
  close path used by the destructor, the lock acquire that stores at `+0x00`,
  and the matching release that clears it.
- Intrusive doubly-linked list primitives (`0x80160F10`-`0x80161020`, 4
  bodies): insert-after, unlink, append-at-tail and insert-at-head for the
  `{prev, next}` node shape already used by the wave-40 sound-bank tables.
  These are the routines wave 40 called only by absolute address.
- Live-context facade (`0x80164B10`-`0x80164D50`, 10 bodies): read the object
  published in `r13 - 0x52B4`, read a state by id, read the current state and
  slot byte, expose `+0x2878`, index the per-slot object array, close one slot
  (vtable `+0x28` plus clearing aux bit 2), key lookup in a slot map, slot and
  context shutdown, and the teardown request.
- `0x38`-stride element dispatch trampolines (`0x801662E0`-`0x80166360`, 7
  bodies): the per-index forwarders into the wave-40 accessor set
  (`fn_80168080`, `fn_801680c0`, `fn_801680e0`) plus the value setter, range
  test and the `+0x34` byte / `+0x28` float accessors.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80160020` | `fn_80160020` | 36 | medium |
| `0x80160080` | `fn_80160080` | 132 | high |
| `0x80160110` | `fn_80160110` | 104 | high |
| `0x80160190` | `fn_80160190` | 156 | medium |
| `0x80160240` | `fn_80160240` | 132 | medium |
| `0x801602D0` | `fn_801602d0` | 40 | high |
| `0x80160930` | `fn_80160930` | 96 | high |
| `0x80160990` | `fn_80160990` | 52 | medium |
| `0x801609D0` | `fn_801609d0` | 56 | high |
| `0x80160F10` | `fn_80160f10` | 80 | high |
| `0x80160F60` | `fn_80160f60` | 128 | high |
| `0x80160FE0` | `fn_80160fe0` | 64 | high |
| `0x80161020` | `fn_80161020` | 64 | high |
| `0x80164B10` | `fn_80164b10` | 8 | high |
| `0x80164B20` | `fn_80164b20` | 12 | high |
| `0x80164B30` | `fn_80164b30` | 8 | high |
| `0x80164B40` | `fn_80164b40` | 12 | high |
| `0x80164B60` | `fn_80164b60` | 12 | high |
| `0x80164B70` | `fn_80164b70` | 16 | high |
| `0x80164B90` | `fn_80164b90` | 124 | medium |
| `0x80164C50` | `fn_80164c50` | 128 | medium |
| `0x80164CD0` | `fn_80164cd0` | 128 | medium |
| `0x80164D50` | `fn_80164d50` | 8 | high |
| `0x801662E0` | `fn_801662e0` | 16 | high |
| `0x801662F0` | `fn_801662f0` | 16 | medium |
| `0x80166300` | `fn_80166300` | 16 | high |
| `0x80166310` | `fn_80166310` | 16 | high |
| `0x80166320` | `fn_80166320` | 16 | medium |
| `0x80166340` | `fn_80166340` | 20 | high |
| `0x80166360` | `fn_80166360` | 20 | high |

## Deliberate skips

- `0x80167B10`-`0x80167BA0` (8 bodies, 124 bytes): a second, byte-identical
  copy of the `0x801662E0`-`0x80166360` trampoline set (same callees, same
  offsets, same sizes). One copy is translated; the duplicate vtable's copy is
  left for a later pass rather than padding the lane.
- `0x80160D10` (368 bytes) and `0x80160EA0` (104 bytes): floating-point
  helpers whose exports are polluted by GQR/`ldexpf` artifacts and unrecovered
  `unaff_r2` globals; the control flow cannot be stated honestly from the C.
- `0x801611C0` (436 bytes): a large quadrant-based angle routine whose
  branches depend on four unprinted `r2`-relative floats; only the branch tree
  is visible, so it was not guessed.
- The remaining 268 untranslated entries in the bucket are smaller leaf
  wrappers and unanalysed sound/scene code; they are outside this pass's
  chosen contiguous runs.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave41` reports `OK` for
  `ledger/agents/wave41-registry.csv` (30 rows, 30 unique addresses).
- `cargo check --message-format=short` reports no errors in
  `src/resource/registry.rs`.
- Module header cluster table and function count were updated (125 -> 155).
