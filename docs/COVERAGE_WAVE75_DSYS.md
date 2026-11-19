# Coverage - Wave 75 (engine::dsys)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 30 | 2596 | 26 | 4 |

The file already held the 18 `dSys_c` start-up bodies from an earlier wave;
this pass adds 30 more in three adjacent clusters, for 48 bodies total.

## What this pass covers

- **Cluster A - message-string builder, `0x800E6930..0x800E71A0` (14 bodies).**
  The family that renders ids, decimal counters and mode switches into the
  0x10-slot wide string at `+4` of a message object. `0x800E6970` is the shared
  slot clear, `0x800E6930` the digit-count helper, `0x800E69C0` the encoder
  core, and the rest are thin wrappers that pick a style from a byte.
- **Cluster B - tween helpers and widget floats, `0x800E7C70..0x800E8120`
  (12 bodies).** A guarded free, a clamped inverse lerp, a delta stepper, and
  seven accessors for the fields `+0x38`, `+0x3C`, `+0x58`, `+0x5C`, `+0x60`,
  `+0x78`, `+0x7C` of the object pointed at by `r13 - 0x5880` (`0x80429E00`).
  The paired fields are differenced in `fn_800e80d0`, which is why the cluster
  is read as a tween/scale record.
- **Cluster C - display metrics and node chain, `0x800E8AB0..0x800E8F10`
  (4 bodies).** `fn_800e8ab0` is the display-metric capture the `dSys` header
  already refers to: it caches `dSys + 0x44` in `0x8042A480` and fills the
  metric block at `0x8042A0E0`. `fn_800e8dc0` allocates the 0x58-byte node
  chain (head in `+0x24`, index at `+0x30`, back link `+0x50`, forward link
  `+0x54`), and `fn_800e8e90` / `fn_800e8f10` walk it.

## Field notes

- `0x802DCD70` / `0x802DCD74` / `0x802DCDBC` / `0x802DCDC0` are the Metrowerks
  `__savegprlr` / `__restgprlr` register-save entries, not high-level calls.
  Where Ghidra prints their fake 64-bit "return" and then uses one half as an
  object pointer (`fn_800e69c0`, `fn_800e8dc0`), the pointer is reconstructed
  from the matching caller and marked `// inferred:`.
- `0x802E441C` is a `u16`-string length (`wcslen`); several bodies call it on
  the string they just built and discard the result.
- The `CONCAT44(0x43300000, x)` / `2^52` pairs in `fn_800e8ab0` and
  `fn_800e8f10` are the CodeWarrior unsigned-to-double idiom; the Rust keeps the
  equivalent integer arithmetic and documents the artefact.
- The `ldexpf` / `unaff_GQR0` lines that Ghidra inserts around paired-single
  loads (see `docs/QUALITY.md` section 5.3) do not appear in the chosen bodies.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E6930` | `fn_800e6930` | 60 | high |
| `0x800E6970` | `fn_800e6970` | 72 | high |
| `0x800E69C0` | `fn_800e69c0` | 208 | medium |
| `0x800E6A90` | `fn_800e6a90` | 188 | high |
| `0x800E6B50` | `fn_800e6b50` | 96 | high |
| `0x800E6BB0` | `fn_800e6bb0` | 40 | high |
| `0x800E6C30` | `fn_800e6c30` | 12 | high |
| `0x800E6C40` | `fn_800e6c40` | 160 | high |
| `0x800E6D40` | `fn_800e6d40` | 112 | high |
| `0x800E6DB0` | `fn_800e6db0` | 80 | high |
| `0x800E6E00` | `fn_800e6e00` | 128 | medium |
| `0x800E6E80` | `fn_800e6e80` | 108 | high |
| `0x800E7190` | `fn_800e7190` | 12 | high |
| `0x800E71A0` | `fn_800e71a0` | 32 | high |
| `0x800E7C70` | `fn_800e7c70` | 64 | high |
| `0x800E7CB0` | `fn_800e7cb0` | 160 | high |
| `0x800E7D50` | `fn_800e7d50` | 144 | high |
| `0x800E7DE0` | `fn_800e7de0` | 64 | medium |
| `0x800E7ED0` | `fn_800e7ed0` | 12 | high |
| `0x800E7F80` | `fn_800e7f80` | 12 | high |
| `0x800E8010` | `fn_800e8010` | 12 | high |
| `0x800E8020` | `fn_800e8020` | 12 | high |
| `0x800E80C0` | `fn_800e80c0` | 12 | high |
| `0x800E80D0` | `fn_800e80d0` | 60 | high |
| `0x800E8110` | `fn_800e8110` | 12 | high |
| `0x800E8120` | `fn_800e8120` | 12 | high |
| `0x800E8AB0` | `fn_800e8ab0` | 124 | high |
| `0x800E8DC0` | `fn_800e8dc0` | 200 | medium |
| `0x800E8E90` | `fn_800e8e90` | 116 | high |
| `0x800E8F10` | `fn_800e8f10` | 272 | high |

## Deliberately skipped

- `0x800E5820`, `0x800E5AD0`, `0x800E5DD0`, `0x800E6090`, `0x800E63A0`,
  `0x800E6680`, `0x800E71C0`, `0x800EC3B0` - the large bodies in the bucket.
  Several carry the paired-single `ldexpf`/`unaff_GQR0` mis-render and
  "removing unreachable block" warnings, so their argument registers and stack
  slots cannot be trusted from the export. Left for a pass that reads the raw
  disassembly.
- `0x800EA060` - 1236-byte global-table restore that starts with the
  `0x802DCD74` register-save helper and then copies a run of `DAT_8031B1C0..`
  words; the table's layout is unknown, so no honest body was written.
- `0x800E54E0`, `0x800E55F0`, `0x800E5790` - font/console setup whose six-float
  calls (`0x801C8920(d,d,d,d,0,ptr)`) and GX register writes would need a
  float-argument call helper that `crate::platform` does not provide yet.
- `0x800E69C0`'s callees are translated as calls, not reconstructed; the
  encoder `0x800CDC40` returns a pointer through a Ghidra `void` signature, so
  that body stays outside this lane.
- `0x800EA000` / `0x800EA030` - 48-byte bodies that only forward to the
  unrecovered jump table at `0x802DCBFC`; no honest body can be written from the
  export.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors (and free of diagnostics) for
  `src/engine/dsys.rs`.
- `node tools/check-lane-csv.mjs wave75` reports `OK` for
  `ledger/agents/wave75-dsys.csv`.
