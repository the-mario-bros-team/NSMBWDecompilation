# Coverage - Wave 53 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 53: the `0x80B10000` bucket of REL module 3,
`d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below corresponds
to a real `pub unsafe fn` body in that file; the `evidence` column of the lane
CSV names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 25 | 2,816 | 7 | 18 |

The two `partial` rows (`0x80B188E0`, `0x80B18910`) are counting in the
`medium` column: their visible call is exact, but the hidden `r12` this-adjust
operand of `0x802DCBFC` cannot be modelled by `platform::call3`.

## What this pass covers

The export names no class in this bucket, so the roles below are `// inferred:`
from each function's own instruction stream. The pass is one contiguous run,
`0x80B18580..0x80B196A0`, which breaks into three short actor blocks:

- **duplicate callback/helper pair, `0x80B18580..0x80B18910`.** The two
  registration/dispose callbacks (same shape as the `iron_ball` pair translated
  in wave 47), two 12-byte constant producers returning `0x80357BF4` and
  `0x80358004`, the unrolled `+0x30` chain walker, the `:`-suffix name
  predicate and the two 48-byte deleting-destructor forwarders.
- **`0x3C0`-byte actor, `0x80B18940..0x80B18D90`.** The constructor installs
  class vtable `0x80B6A63C`, a `u16` timer at `+0x392`, a step counter at
  `+0x394`, a face flag at `+0x3AC`, an animation index at `+0x3A8` and a
  12-byte vector at `+0x3B0`. The two effect spawners use the direction byte
  `+0x348` to select a per-direction offset row and gate on the effect
  manager's `+0x150`/`+0x14C` counters. The timer step reloads from the
  six-entry table at `0x80B323A8`. The block ends with a player-side query and
  the window writer that copies the four flags from
  `0x80B323EC/B323F0/B323FC/B32400`.
- **`+0x52C` window actor, `0x80B19140..0x80B196A0`.** An embedded controller
  at `+0x394`, an x window `[+0x52C, +0x534]` with a y window at
  `+0x530/+0x538`, a timer `+0x528`, a live projectile slot `+0x524` (release
  flag `+0x5EC`) and the three state descriptors
  `0x80B79038`/`0x80B79078`/`0x80B790B8`. The block contains the controller
  step, the camera-slot window refresh, the four-player search, the absolute
  elapsed-time helpers, the `0xC0` projectile spawner and the
  release/launch/idle steps.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B18580` | `fn_80b18580` | 88 | high |
| `0x80B185E0` | `fn_80b185e0` | 92 | high |
| `0x80B18640` | `fn_80b18640` | 12 | high |
| `0x80B18650` | `fn_80b18650` | 12 | high |
| `0x80B18740` | `fn_80b18740` | 224 | medium |
| `0x80B18820` | `fn_80b18820` | 136 | medium |
| `0x80B188E0` | `fn_80b188e0` | 48 | medium (partial) |
| `0x80B18910` | `fn_80b18910` | 48 | medium (partial) |
| `0x80B18940` | `fn_80b18940` | 124 | high |
| `0x80B189C0` | `fn_80b189c0` | 148 | medium |
| `0x80B18A60` | `fn_80b18a60` | 148 | medium |
| `0x80B18B00` | `fn_80b18b00` | 192 | medium |
| `0x80B18CC0` | `fn_80b18cc0` | 84 | medium |
| `0x80B18D90` | `fn_80b18d90` | 84 | medium |
| `0x80B19140` | `fn_80b19140` | 104 | medium |
| `0x80B191B0` | `fn_80b191b0` | 148 | medium |
| `0x80B19250` | `fn_80b19250` | 156 | medium |
| `0x80B192F0` | `fn_80b192f0` | 92 | medium |
| `0x80B19350` | `fn_80b19350` | 52 | high |
| `0x80B19390` | `fn_80b19390` | 8 | high |
| `0x80B193A0` | `fn_80b193a0` | 268 | medium |
| `0x80B194B0` | `fn_80b194b0` | 72 | medium |
| `0x80B19510` | `fn_80b19510` | 124 | medium |
| `0x80B195B0` | `fn_80b195b0` | 204 | medium |
| `0x80B196A0` | `fn_80b196a0` | 148 | medium |

## Deliberate skips

- **`0x80B18F20` (148 bytes).** The overlap search over object `0x19B`. Ghidra
  declares a single `param_1` but the recursive call is
  `FUN_80b18f20(param_1, iVar2)`, and one caller passes `(this, 0)`. The second
  operand is therefore a scan cursor whose contract the C export never shows.
  Writing a loop from that would invent the iteration rules, so the body is
  left for a later pass that can read the assembly. `0x80B18D90` still calls it
  by address.
- **The rest of the `0x80B10000` bucket.** 155 functions remain after this
  pass; the next contiguous run starts at `0x80B19A20`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 25 sizes sum to
  2,816 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave53` prints `OK` for this lane's CSV.
