# Coverage - Wave 63 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer. Every function below is a literal reconstruction of one
Ghidra body; the doc block above each body names the original address, the
byte count from `work/ghidra/main-gb/export/functions.tsv` and the evidence
file. Offsets in this document are relative to the object named in the row.
"high" confidence means every store, branch and call is visible in the export;
"medium" means a virtual-call receiver, a forwarding operand or a link/field
role had to be inferred from a sibling function or call site.

This pass only edits `src/engine/display.rs`; the ledger is merged by the
coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 30 | 2344 | 22 | 8 |

## What this pass covers

The `0x800D0000` bucket still holds 160 untranslated of its 251 functions.
This pass takes four coherent slices of the same engine band and leaves the
paired-single bodies alone:

- **The `0x80303408` actor wrapper** (`0x800D54A0..0x800D5790`, 6 bodies).
  [`fn_800d54a0`] constructs it, [`fn_800d54e0`] destroys it with the two
  array descriptors `+0x190` (6 x `0xC` bytes) and `+0x28` (2 x `0x38` bytes)
  plus the embedded object at `+0x98`, [`fn_800d5580`] opens it, and
  [`fn_800d5680`] / [`fn_800d56f0`] / [`fn_800d5790`] copy, rebuild and read
  its transform.
- **The `0x80318A80`/`0x80318A90` node wrappers**
  (`0x800D6E00..0x800D70C0`, 5 bodies). A node is an actor pointer at `+0x04`
  plus a ready word at `+0x08`; the bodies forward to the cluster above
  ([`fn_800d6eb0`] to [`fn_800d5680`], [`fn_800d6f40`] to
  [`fn_800d56f0`]) so the two clusters are read together.
- **The spin-jump counter object** (`0x800D85D0..0x800D8840`, 10 bodies).
  A single instance is published at `r13 - 0x5880` by [`fn_800d85d0`]. It
  keeps a three-slot activity mask at `+0x04` and three frame counters at
  `+0x08/+0x0C/+0x10`; [`fn_800d8680`] is the tick and plays sound `0xAB`
  below `0xB5` frames or `0xAA` at or above on every whole 60-frame boundary,
  and sends signal `8` through `0x8019BD20` when a counter expires.
  [`fn_800d8640`] / [`fn_800d8780`] mirror the four saved words at
  `+0x84..+0x90`.
- **The embedded actor search context** (`0x800D9860..0x800D9AD0`, 9 bodies).
  `0x80064110` constructs it inside the actor at `+0x2DC`; [`fn_800d98c0`]
  binds it to its owner, [`fn_800d9940`] searches the global list at
  `r13 - 0x5664` and stores the accepted actor in the collaborator's `+0x14`,
  and [`fn_800d9a30`] confirms that candidate. [`fn_800d9920`] exposes the
  target's `+0x3C` flag word through a caller-supplied mask.

Two structural facts are worth recording because several bodies depend on
them. First, the `0x802DCD6C` and `0x802DCD74` helpers return an eight-byte
value whose *high* word Ghidra uses as a collaborator pointer; `0x800D9940`
and `0x800D70CD0` both pass the *low* word to `0x802DCBFC`. Second, the same
`0x802DCBFC` routine is reached from the `0x800D9xxx` search cluster and from
the already-translated `0x800D16B0` cursor helper, so its argument contract is
still only partly known and stays `// unknown:` in the doc blocks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D54A0` | `fn_800d54a0` | 60 | medium |
| `0x800D54E0` | `fn_800d54e0` | 156 | high |
| `0x800D5580` | `fn_800d5580` | 168 | high |
| `0x800D5680` | `fn_800d5680` | 112 | medium |
| `0x800D56F0` | `fn_800d56f0` | 64 | medium |
| `0x800D5790` | `fn_800d5790` | 96 | medium |
| `0x800D6E00` | `fn_800d6e00` | 132 | high |
| `0x800D6EB0` | `fn_800d6eb0` | 132 | medium |
| `0x800D6F40` | `fn_800d6f40` | 180 | high |
| `0x800D70A0` | `fn_800d70a0` | 24 | high |
| `0x800D70C0` | `fn_800d70c0` | 28 | high |
| `0x800D85D0` | `fn_800d85d0` | 20 | high |
| `0x800D85F0` | `fn_800d85f0` | 72 | high |
| `0x800D8640` | `fn_800d8640` | 40 | high |
| `0x800D8680` | `fn_800d8680` | 256 | high |
| `0x800D8780` | `fn_800d8780` | 40 | high |
| `0x800D87B0` | `fn_800d87b0` | 20 | high |
| `0x800D87D0` | `fn_800d87d0` | 20 | high |
| `0x800D87F0` | `fn_800d87f0` | 16 | high |
| `0x800D8820` | `fn_800d8820` | 24 | high |
| `0x800D8840` | `fn_800d8840` | 16 | high |
| `0x800D9860` | `fn_800d9860` | 28 | high |
| `0x800D9880` | `fn_800d9880` | 64 | high |
| `0x800D98C0` | `fn_800d98c0` | 40 | high |
| `0x800D98F0` | `fn_800d98f0` | 12 | high |
| `0x800D9900` | `fn_800d9900` | 28 | high |
| `0x800D9920` | `fn_800d9920` | 32 | high |
| `0x800D9940` | `fn_800d9940` | 236 | medium |
| `0x800D9A30` | `fn_800d9a30` | 136 | medium |
| `0x800D9AD0` | `fn_800d9ad0` | 92 | medium |

Byte total: `656 + 496 + 524 + 668 = 2344` across the four clusters.

## Deliberate skips

- **Argument-less virtual calls** `0x800D6E90` (slot `+0x18`), `0x800D7000`
  (slot `+0x20`) and `0x800D7020` (slot `+0x24`) were left out. Their bodies
  are 20-32 bytes of `(**(code **)(...))();` with no receiver and no
  arguments in the export, so the operand list cannot be recovered honestly.
  The three wrappers are still reachable by address.
- **`0x800D6DF0`** (8 bytes) calls `0x800D5580` with only the actor operand
  visible; the three remaining argument registers are not recoverable, so the
  body was not written rather than padded with invented zeros.
- **`0x800D5730`** (80 bytes) reads a vtable slot `+0x28` without operands and
  forwards its result to `0x80168180`; the receiver is inferred but the return
  use is not, so it is deferred.
- **`0x800D9AC0`** (12 bytes) is a single `+0x14 = 0` store. It is trivially
  honest, but it was dropped to keep the pass inside the 25-30 body budget;
  [`fn_800d9a30`] performs the same clear on its failure path.
- **`0x800D8850`** (268 bytes) mixes two float pointer tables
  (`0x80318D58`, `0x80318D38`) with `G_PLAYER_SLOTS`; the table entries are
  pointer+float pairs, so the body needs table typing that the export does not
  provide. Deferred rather than guessed.
- **Paired-single / GQR bodies** stay out of scope as in wave 38:
  `0x800D6460`, `0x800D6550`, `0x800D65E0` and the large `0x800D57F0`
  (`ldexpf`/`unaff_GQR0`) cannot be reconstructed from the export.
  `0x800D57F0` is still called by address from [`fn_800d56f0`].
- **Identical 48-byte forwarding thunks** `0x800DD1C0`, `0x800DD1F0`,
  `0x800DD220`, `0x800DD250`, `0x800DD280`, `0x800D7C20` and `0x800D7C50`
  all decompile to the same `FUN_802dcbfc(param_2, param_2, param_1)` body.
  They are most likely a jump-table/thunk bank; shipping seven copies would
  add rows without adding understanding.
- The `0x800D85D0` counter tick calls `0x8019BD20`, and [`fn_800d9940`] calls
  `0x802DCD6C`/`0x802DCDB8`; those three callees remain `// unknown:` in the
  doc blocks instead of being given invented names.

## Verification

- Every address and byte size matches `functions.tsv` (checked mechanically:
  zero mismatches over the new section).
- Every evidence file exists at the cited path and every row has exactly one
  `pub unsafe fn fn_<addr>` body in `src/engine/display.rs`.
- `ledger/functions.csv` was read only; none of the 30 addresses was present
  before this pass, and `ledger/agents/wave63-display.csv` has 30 unique rows.
- `node tools/check-lane-csv.mjs wave63` reports `OK`.
- `cargo check --message-format=short` reports no errors and no warnings for
  `src/engine/display.rs`.
