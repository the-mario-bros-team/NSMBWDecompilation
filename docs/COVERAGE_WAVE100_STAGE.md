# Coverage - Wave 100 (`game::stage`, the `0x800B0000` lift/actor tail)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave100-stage.csv` is the Ghidra C export each body was
written from. Field roles the export only hints at stay marked `observed`,
`inferred` or `unknown` in the body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 25 | 6680 | 7 | 18 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **The `dIggyWanKusari_c` state registrar** (`0x800BA5E0`). It is the same
  five-step block wave 99 found for `dEnBoss`: nine `.rodata` words are copied
  into `record + 0xC..+0x2C` with the permutation `+0x18, +0x1C, +0x20, +0x0C,
  +0x10, +0x14, +0x00, +0x04, +0x08`; the name comes from `0x8015F7C0`; `+0x00`
  receives the state vtable `0x80315AD0`; and `0x802DC780` chain-registers the
  record with `0x800BAAC0` and the predecessor `record - 0x10`. Six records,
  `0x40` stride, `0x24`-stride sources at `0x80315928`. This body also fixes the
  one global `s16` angle written at `r13-0x5710`.
- **The lift node list** (`0x800B9300`, `0x800B9FC0`, `0x800BAE50`,
  `0x800BAE60`). `0x800B9300` allocates `0x8C`-byte nodes and links them through
  the `+0x84` previous / `+0x88` next words that wave 88 used, re-tagging the
  `+0x14` interface from `0x803295D8` to `0x80329578`. `0x800B9FC0` pulls each
  node's end point toward the `+0x24` source at a fixed normalised distance,
  `0x800BAE50` ticks the node interface, and `0x800BAE60` rebuilds its heading,
  midpoint and derived words.
- **The lift cell grid** (`0x800BEC00`, `0x800BF6E0`, `0x800BF7D0`,
  `0x800BF970`, `0x800BFB10`, `0x800BFD80`). Cells are `0x84` bytes with a
  twelve-word matrix at `+0x6C`; the grid size is always derived through
  `0x800BE910` (columns) and `0x800BE930` (rows). `0x800BFB10` builds one
  translate+rotate matrix and feeds the two writers; `0x800BFD80` runs the whole
  placement at `+0x1C` and, in game mode `1`, the mirrored one at `+0x22AC`.
- **Cell placement and record fillers** (`0x800B6830`, `0x800B6E50`,
  `0x800B8700`, `0x800B95E0`). `0x800B6830` builds the rotated matrix and writes
  the two `+0xAC`/`+0xB8` translation vectors, `0x800B6E50` drives it, and
  `0x800B8700` fills one render item per `0x3C`-byte source record (including
  the six-word block order `[4],[5],[6],[1],[2],[3]`).
- **The sprite animation and small helpers** (`0x800BFE90`, `0x800BC4C0`,
  `0x800BC5A0`, `0x800B53A0`, `0x800B6460`, `0x800B38D0`, `0x800BFA40`,
  `0x800BFAA0`, `0x800B81B0`, `0x800BBF00`). These are the three-state vertical
  animation, the two effect-node draws, the slot-priority raiser, the
  load-result registration, the signed remainder scaler, the two forwarding
  wrappers and the two empty overrides.

Address range: `0x800B38D0..0x800BFE90` (25 bodies, 6680 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B38D0` | `fn_800b38d0` | 164 | medium |
| `0x800B53A0` | `fn_800b53a0` | 92 | medium |
| `0x800B6460` | `fn_800b6460` | 124 | medium |
| `0x800B6830` | `fn_800b6830` | 252 | medium |
| `0x800B6E50` | `fn_800b6e50` | 160 | medium |
| `0x800B81B0` | `fn_800b81b0` | 4 | high |
| `0x800B8700` | `fn_800b8700` | 332 | medium |
| `0x800B9300` | `fn_800b9300` | 440 | medium |
| `0x800B95E0` | `fn_800b95e0` | 180 | medium |
| `0x800B9FC0` | `fn_800b9fc0` | 608 | medium |
| `0x800BA5E0` | `fn_800ba5e0` | 1236 | high |
| `0x800BAE50` | `fn_800bae50` | 16 | medium |
| `0x800BAE60` | `fn_800bae60` | 240 | medium |
| `0x800BBF00` | `fn_800bbf00` | 4 | high |
| `0x800BC4C0` | `fn_800bc4c0` | 212 | medium |
| `0x800BC5A0` | `fn_800bc5a0` | 40 | medium |
| `0x800BEC00` | `fn_800bec00` | 756 | medium |
| `0x800BF6E0` | `fn_800bf6e0` | 236 | high |
| `0x800BF7D0` | `fn_800bf7d0` | 408 | medium |
| `0x800BF970` | `fn_800bf970` | 208 | high |
| `0x800BFA40` | `fn_800bfa40` | 88 | medium |
| `0x800BFAA0` | `fn_800bfaa0` | 104 | medium |
| `0x800BFB10` | `fn_800bfb10` | 328 | high |
| `0x800BFD80` | `fn_800bfd80` | 200 | high |
| `0x800BFE90` | `fn_800bfe90` | 248 | medium |

## Deliberately skipped

The bucket still holds 23 untranslated functions after this pass (48 at the
start). Skipped on purpose:

- **GQR-quantised math bodies** (`0x800B3000`, `0x800B33D0`, `0x800B8CF0`,
  `0x800B9A90`, `0x800B96A0`, `0x800BBE10`). Each passes `s16`/`u16` values
  through `unaff_GQR0`/`unaff_GQR3` `ldexpf` branches that the export prints as
  unreachable blocks, or through the `CONCAT44(0x43300000, …)` word-to-double
  idiom. The quantisation order and the `r2` constants are not visible, so no
  honest body can be written.
- **The `0x802DCD5x..0x802DCD7x` half-return families** (`0x800B39C0`,
  `0x800BC2E0`, `0x800BC5D0`, `0x800BC6B0`, `0x800BD7A0`, `0x800BD970`,
  `0x800BE000`, `0x800BE390`). These consume `extraout_f1` or both register
  halves of a save helper in an order the C text does not pin down; the
  positions and angles are then read from stale locals.
- **The `extraout_f1` lift fillers** (`0x800BE690`, `0x800BE950`, `0x800BEA00`,
  `0x800BEB50`). The real callee takes seven words: the float argument is
  printed as `extraout_f1` after the call, so the first argument of
  `FUN_80169D90`/`FUN_80166EF0` cannot be grounded. They are still called by
  address from `0x800BFA40`.
- **`0x800BEF00`.** The four-branch `+0x207C` writer mixes the `0x43300000`
  conversions with `DAT_80358DF0..DF8` stores whose block layout is not
  recoverable from the text; it is called by address from `0x800BFAA0`.
- **`0x800BF3F0`.** The four bit-selected `+0x20E8..+0x2288` blocks are literal
  but were not attempted in this pass; `0x800BFAA0` calls it by address.
- **`0x800BC140`.** The matrix body uses `local_44`/`local_34`/`local_24`
  before they are written, so the translation and rotation arguments would have
  to be invented.
- **`0x800B4600`.** A 4-byte thunk of `0x800B4610`, which is already
  translated; per the wave-84/98/99 precedent thunks of translated bodies are
  not counted twice.
- **`0x800B54B0`.** A 4-byte thunk of `0x801DD1D0`, which already has a ledger
  row.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (25 rows, 6680 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 25
  addresses were chosen from the 48 open entries of the `0x800B0000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs` (the errors it reports are in other lanes' files:
  `runtime::cpp`, `resource::bres`, `runtime::dvd`, `render::gx_band` and
  `game::worldmap`).
- `node tools/check-lane-csv.mjs wave100` prints `OK`; the wave-100 stage rows
  are clean.
