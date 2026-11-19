# Coverage - Wave 75 (`game::stage`, the `0x800Bxxxx` course-manager runs)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave75-stage.csv` points at the Ghidra C export each
body was written from.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 27 | 2936 | 16 | 11 |

No body in this pass is `low` confidence and none is `partial`.

## What this pass covers

Four untranslated runs inside the `0x800Bxxxx` band, all between the
fukidashi object (wave 45) and the course manager (wave 10). They are wired
together by two layouts that the stores make explicit:

* **Handle array**: count at `+0x00`, live handles at `+0x34` (a `4`-stride
  array). Every handle is resolved with `0x80162D00`, which the export usually
  prints with no argument - the handle itself is recovered as `r3` and marked
  `inferred` in each body.
* **Node list**: count at `+0x1C`, head at `+0x20`, next pointer at `+0x88`,
  `+0x24` optional owner-record, `+0x14` generation word.

The four runs:

- **Status word and colour tables** (`0x800B3750..0x800B3C50`). The scene
  forward `0x800B3750`, the clear/mask pair on the wave-62 status word
  (`0x800B3A20`/`0x800B3B50`), and `0x800B3C50`, which lazily fills two
  four-entry colour tables at `0x80358A58`/`0x80358A68` from sixteen literals
  each and then writes the selected entry into a record obtained from vtable
  slot `+0x78`.
- **Four-slot state list** (`0x800B58E0..0x800B5C60`). Locked allocation
  (`0x800B58E0`), the list itself (`0x800B5930`, vtable `0x80315600`, four
  `0x94`-byte children, published at `r13-0x5730`), the child constructor
  (`0x800B5B00`, vtable `0x80315610`), the 38-store child field initialiser
  (`0x800B5B90`, three `r2` words at `0x8042C5B0`/`0x8042C5B4`/`0x8042C5B8`),
  the four-slot destructor walk (`0x800B5AB0`) and the `u16` at `+0x8E`
  shifted into the high halfword (`0x800B5C50`).
- **Input-word translator** (`0x800B6080`/`0x800B63A0`). `0x800B6080` maps a
  raw pad/flag word onto the packed state bits and only runs while `+0x08` is
  `1` or `2`; `0x800B63A0` fixes up that mode from the descriptor's `+0x10`,
  translates the actor row's `+0x1C` word and tests it against a mask. Two
  small object helpers follow (`0x800B6450`, `0x800B64E0`).
- **Actor walkers and node list** (`0x800B8A20..0x800B9530`, completed by
  `0x800BAAC0`/`0x800BADC0`/`0x800BD1E0`). Four sweeps of the `+0x34` handle
  array (match, set `+0x5DC`, clear `+0x5DC`, release), the event-plus-vtable
  release pass `0x800B8C30` (event `0x26C` carrying a transform built from
  `+0x64 + 0xAC`), the `0x88`-linked node list and its four per-node passes
  (`0x800B90C0..0x800B9530`), the detach/free pair, the draw-list refresh
  `0x800BADC0`, and the three-way record selector `0x800BD1E0`.

The public entry points are [`fn_800b3c50`], [`fn_800b5930`],
[`fn_800b6080`] and [`fn_800b90c0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B3750` | `fn_800b3750` | 40 | high |
| `0x800B3A20` | `fn_800b3a20` | 12 | high |
| `0x800B3B50` | `fn_800b3b50` | 12 | high |
| `0x800B3C50` | `fn_800b3c50` | 508 | high |
| `0x800B58E0` | `fn_800b58e0` | 72 | medium |
| `0x800B5930` | `fn_800b5930` | 124 | high |
| `0x800B5AB0` | `fn_800b5ab0` | 80 | high |
| `0x800B5B00` | `fn_800b5b00` | 72 | medium |
| `0x800B5B90` | `fn_800b5b90` | 180 | high |
| `0x800B5C50` | `fn_800b5c50` | 12 | high |
| `0x800B6080` | `fn_800b6080` | 280 | high |
| `0x800B63A0` | `fn_800b63a0` | 176 | medium |
| `0x800B6450` | `fn_800b6450` | 16 | high |
| `0x800B64E0` | `fn_800b64e0` | 52 | high |
| `0x800B8A20` | `fn_800b8a20` | 144 | medium |
| `0x800B8AB0` | `fn_800b8ab0` | 128 | medium |
| `0x800B8B30` | `fn_800b8b30` | 128 | medium |
| `0x800B8BB0` | `fn_800b8bb0` | 116 | medium |
| `0x800B8C30` | `fn_800b8c30` | 184 | medium |
| `0x800B90C0` | `fn_800b90c0` | 44 | medium |
| `0x800B9170` | `fn_800b9170` | 96 | high |
| `0x800B91D0` | `fn_800b91d0` | 96 | high |
| `0x800B94C0` | `fn_800b94c0` | 92 | high |
| `0x800B9530` | `fn_800b9530` | 48 | high |
| `0x800BAAC0` | `fn_800baac0` | 48 | medium |
| `0x800BADC0` | `fn_800badc0` | 132 | medium |
| `0x800BD1E0` | `fn_800bd1e0` | 44 | high |

## Deliberately skipped

Skipped because the body could not be reconstructed honestly:

- `0x800B9300` (440 bytes) - the list builder. Its floats are produced and
  consumed through `GQR0`-guarded `ldexpf` blocks and the stack values are
  stored as `(float)` of a `double` loaded from `r2-0x69F4`. The paired-single
  quantisation cannot be stated from the export, so the whole builder is left
  as an address call in [`fn_800b90c0`].
- `0x800B9230` (196 bytes) - the refresh pass. `iVar1 = FUN_8016a5d0();` is
  printed without arguments and its result is compared against `+0x14`; the
  argument cannot be recovered, so the guard would have to be invented.
- `0x800B9570`, `0x800B95E0` (56 and 180 bytes) - a `CONCAT44(0x43300000, n)`
  unsigned-int-to-`double` conversion over `+0x1C` and a length/ratio helper
  built from `+0x630..+0x640` floats. The `r2` double constants and the
  `GQR0` rounding order are not visible, so neither scale nor clamp can be
  named.
- `0x800B39C0` (92 bytes) - forwards `extraout_var`, a `short` Ghidra lost
  from the callee, into `0x800B38D0`. Writing it would require inventing the
  second argument.
- `0x800B6460` (124 bytes) - three of its five calls are printed with no
  arguments at all (`FUN_80007520()`, `FUN_800c0ae0()`, and a `0x800B6450`
  call whose receiver is dropped); only the two `param_2`/`param_3` calls
  survive.

Also left untranslated this pass, but reconstructable in a later one:

- `0x800B5C60` (1056 bytes) - the child refresh/registration routine. It is
  mechanical (about forty field copies, a `0x80377C88` row read, the packed
  nibble loop and a length via `0x802E8620`) but long enough that a partial
  transcription would be worse than none.
- `0x800BABB0` (280 bytes) and `0x800BACD0` (236 bytes) - the per-node mesh
  binder and the colour-record writer. Both index the `r13-0x7AD8` table and
  the `r2` float block; they need a pass that can name the BRRES/model tables.
- `0x800B61A0`, `0x800B6250` (180 and 328 bytes) - the `+0x8E` halfword
  updater and the contact/counter state machine. Both compare float fields in
  `+0x34..+0x50` against `r2` thresholds; the comparisons are readable but
  were left to keep this pass on integer-verifiable bodies.
- `0x800B37E0`, `0x800B38D0`, `0x800B33D0`, `0x800B3000` - the
  `CONCAT44`/`r2`-double camera-scale helpers, same reason as `0x800B9570`.

The remaining `0x800Bxxxx` gaps outside these four runs
(`0x800B04A0..0x800B3000`, `0x800B3E50..0x800B58E0`, `0x800B6500..0x800B8A20`,
`0x800B9600..0x800BAAB0`, `0x800BB000..0x800BDD00` and up) were not touched
by this lane.

## Verification

- Every address and byte size matches `functions.tsv` (27 rows, 2936 bytes);
  `tools/check-lane-csv.mjs` cross-checks each `# Original` size.
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave75` prints `OK`; the wave-75 stage rows
  are clean.
