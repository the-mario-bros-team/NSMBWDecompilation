# Coverage - Wave 75 (`game::bases`, `0x8077` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers (`0x8042xxxx`,
`0x8047xxxx`, `0x8037xxxx`) are `main.dol` addresses imported by the REL and
keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. In the table below,
"role" is the plain-English behaviour, and confidence is `high` when guards,
stores, constants, strings and call arguments are literal, `medium` when the
role of a field or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 5104 | 12 | 13 |

One body is `low` confidence (`fn_80770b20`), so the medium column plus the
high column plus that body accounts for all 26 rows.

## What this pass covers

The `0x8077` bucket held 182 functions at lane start; 31 were already in
`ledger/functions.csv` (all from the wave-56 manager and `cobPeachCastle`
cluster), leaving 151. This pass fills two clusters:

- **Ship actor tail, `0x807700F0..0x807713B0`** - eighteen bodies of the actor
  whose panes are `propellerFront` and `cobKoopaShip` and whose model loader
  binds `g3d/togezo.brres`:
  - the position step [`fn_807700f0`] (six-float frame, `0x800F6410` move) and
    the pane rebuild [`fn_807701c0`];
  - the state machine at `+0x478` [`fn_807702a0`] (states `1`, `8`, `9`, `10`,
    `0x0B`, `0x0C`) and the second machine at `+0x4A0` [`fn_807706b0`]
    (states `2`, `3`, `6`, `7`, `8`), with the frame-count helper
    [`fn_80770620`] and the re-arm [`fn_80770680`];
  - the arrival/warp machine at `+0x4B0` [`fn_80770840`] plus its reset
    [`fn_80770830`] and the camera-side test [`fn_80770970`];
  - the `cobKoopaShip` hand-over effect [`fn_807709f0`], its broadcast over the
    `0x284` actor chain [`fn_80770aa0`], the five-type re-activation
    [`fn_80770b20`], the published-position/flag refresh [`fn_80770c50`] and
    the range test [`fn_80770d90`];
  - the constructor [`fn_80770fd0`], the `togezo` model loader
    [`fn_80771240`], the matrix writer [`fn_807712f0`] and the scale reset
    [`fn_807713b0`].

- **Forest manager dial lists, `0x80772AD0..0x807731E0`** - eight members of the
  `0x5C7F4`-byte manager whose constructor is the wave-56 [`fn_80771540`]. The
  manager owns three `0xA8`-byte dial lists at `+0x3180`, `+0x3228` and
  `+0x32D0` (four `0x28`-byte entries, then counts at `+0xA0`/`+0xA4`):
  - the half-span wrap [`fn_80772ad0`], the reset [`fn_80772d90`] and the
    ten-byte append [`fn_80772e60`];
  - the three-pass rebuild from the `0x16518`-stride clip table
    [`fn_80772eb0`], gated by the `0xBF8`-stride model table;
  - the empty test [`fn_807730e0`], the clip bind [`fn_80773120`], the
    model-table probe [`fn_807731a0`] and the pending-count publish
    [`fn_807731e0`].

Recurring layout of the ship actor: two state words at `+0x478` and `+0x4A0`
(the arrival machine uses `+0x4B0`), one shared countdown at `+0x498`, the
active byte at `+0x124`, the kind word at `+4`, the position at
`+0xAC..+0xB4`, the angle triple at `+0x100..+0x104`, the model at `+0x1A0`,
the matrix at `+0x7C`, the scale at `+0xDC` and the spawn/effect handles at
`+0x1E0`/`+0x4B4`. The dial lists are reached only through the current-object
helpers `0x802DCD6C`/`0x802DCD74`, exactly like the wave-56 collision queries.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x807700F0` | `fn_807700f0` | 204 | medium | position step and republish |
| `0x807701C0` | `fn_807701c0` | 216 | high | `propellerFront`/`cobKoopaShip` pane rebuild |
| `0x807702A0` | `fn_807702a0` | 892 | medium | six-state approach machine `+0x478` |
| `0x80770620` | `fn_80770620` | 88 | medium | frame-count to per-frame distance |
| `0x80770680` | `fn_80770680` | 36 | high | re-arm of the `+0x4A0` machine |
| `0x807706B0` | `fn_807706b0` | 384 | medium | five-state machine `+0x4A0` |
| `0x80770830` | `fn_80770830` | 12 | high | clears the arrival state |
| `0x80770840` | `fn_80770840` | 296 | medium | arrival/warp machine `+0x4B0` |
| `0x80770970` | `fn_80770970` | 128 | medium | camera-side limit test |
| `0x807709F0` | `fn_807709f0` | 172 | medium | `cobKoopaShip` hand-over effect |
| `0x80770AA0` | `fn_80770aa0` | 116 | high | broadcast over the `0x284` chain |
| `0x80770B20` | `fn_80770b20` | 188 | low | five-type re-activation |
| `0x80770C50` | `fn_80770c50` | 312 | medium | published position and state flag |
| `0x80770D90` | `fn_80770d90` | 216 | medium | rider/carrier range test |
| `0x80770FD0` | `fn_80770fd0` | 116 | high | constructor (holder + group + pool) |
| `0x80771240` | `fn_80771240` | 172 | high | `g3d/togezo.brres` loader |
| `0x807712F0` | `fn_807712f0` | 188 | medium | matrix and rotation writer |
| `0x807713B0` | `fn_807713b0` | 36 | high | scale triple reset |
| `0x80772AD0` | `fn_80772ad0` | 252 | medium | dial half-span wrap |
| `0x80772D90` | `fn_80772d90` | 208 | high | dial list reset |
| `0x80772E60` | `fn_80772e60` | 72 | high | ten-byte dial append |
| `0x80772EB0` | `fn_80772eb0` | 548 | medium | three-pass dial rebuild |
| `0x807730E0` | `fn_807730e0` | 52 | high | empty-list test |
| `0x80773120` | `fn_80773120` | 120 | medium | clip bind per dial entry |
| `0x807731A0` | `fn_807731a0` | 56 | high | model-table probe |
| `0x807731E0` | `fn_807731e0` | 24 | high | pending-count publish |

## Address range

`0x807700F0..0x807731E0`, in two runs:

- `0x807700F0..0x807713B0` - eighteen bodies with a single gap at the rejected
  `0x80770BE0`.
- `0x80772AD0..0x807731E0` - eight bodies, unbroken. They sit between the
  wave-56 manager code (`0x80772180..0x80772300`) and the `cobPeachCastle`
  actors (`0x807769F0..0x80777670`), which were already translated.

This pass adds 26 bodies (5,104 bytes), leaving 125 of the 151 remaining
`0x8077` functions for later waves.

## Deliberate skips

- `0x80770BE0` (112 bytes) - its export ends in `halt_baddata()` after a
  "Control flow encountered bad instruction data" warning, so the tail cannot
  be reconstructed honestly. Same rejection class as `0x806148F0` and
  `0x80613290`. Because of that, [`fn_80770b20`] calls it with the literal
  no-argument shape the export shows and is marked `low`.
- `0x807727D0` (764) and `0x80772BD0` (348) - both exports read hidden
  registers (`unaff_r21`, `unaff_r31`) or a split `undefined8` return from
  `0x802DCD5C`/`0x802DCD68`, i.e. decompiler failures; their real argument and
  return values are unrecoverable.
- `0x80773200` (4 bytes, `thunk_FUN_80773210`) - a compiler jump thunk with no
  Rust-visible behaviour, like the other skipped `0x802DCxxx` stubs.
- The rest of the bucket (`0x80772370`, `0x80773A40..0x807740D0`,
  `0x80774750..0x80776750`, `0x80777910..0x8077C440`, `0x8077CD80..0x8077E760`
  and `0x8077F0C0..0x8077FD80`) is untouched, not rejected. Several of those
  bodies are paired-single heavy and will need the same treatment earlier waves
  gave `0x80651640` and `0x806148F0`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (26 rows, 5,104 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 31 pre-existing rows of
  this bucket were checked before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave75` reports `OK` with
  `wave75-bases.csv: 26 rows`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`. (The only build error in the tree during this lane was
  `src/game/enemies.rs`, another parallel lane's file.)
