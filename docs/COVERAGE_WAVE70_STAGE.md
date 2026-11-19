# Coverage - Wave 70 (`game::stage`, the `0x8007xxxx` actor-manager and camera-scroll gaps)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column of `ledger/agents/wave70-stage.csv` points at the Ghidra C export the
body was written from.

This pass closes two gaps inside the `0x8007xxxx` band that wave 33 (the BG
scroll-layer object) and wave 61 (the map-cell probes) left open. Both clusters
are object layouts read from the stores themselves:

* **`dBgActorManager`** (vtable `0x8030FD58`, built by `0x8007E180`): base
  object at `+0x04` (`0x80069020`), entry array pointer at `+0x38` with count at
  `+0x3C`, `0x0C`-byte entries (`+0` descriptor index, `+2`/`+4` cell x/y,
  `+6` state byte, `+8` resource handle), the `0x10000`-byte resource block at
  `+0x04` created by `0x8016A960`, and the current descriptor table in
  `r13-0x58A4`. Descriptor records are `0x20` bytes: `+4` kind word, `+8`/`+0xC`/
  `+0x10` position triple, `+0x14`/`+0x18` extent pair, `+0x1C` flag word.
* **Camera/scene owner** (built by `0x80077060`, `r13-0x58B0`): scroll window at
  `+0x900AC..+0x900FD`, twenty effect slots at `+0x90130..+0x901E4`, per-frame
  accumulators at `+0x8FE18`/`+0x8FE44`/`+0x8FE48`, and the lazily built heap in
  `r13-0x58B4` whose size lives in `r13-0x58B8`.

Every offset below is literal in the export. Class names come from the
`dBgActorManager::m_allocator` string (`0x8030FD38`) and the `Pa3_*` rail-name
comparisons (`0x8030FCF4..0x8030FD20`); no symbol table was consulted.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 27 | 3528 | 18 | 9 |

No body in this pass is `low` confidence and none is `partial`.

## What this pass covers

Three connected clusters, all inside `0x80075090..0x8007EBF0`:

- **Scroll-layer insert** (`0x80075090`) - the missing half of the wave-33
  attach pair: pushes the node onto both parents' `+0x0C`/`+0x10` chains, saves
  the previous heads at `+0x6C`/`+0x70`, clears `+0x74..+0x84`, re-parents the
  two children at `+0x64`/`+0x68` through `0x800751C0`, then publishes through
  `0x80075070` (already translated as `fn_80075070`).
- **Camera/scene owner scroll helpers** (`0x80076DC0..0x80079B60`) - the
  100-entry map probe (`0x80076DC0`), the lazy actor-heap create/destroy pair
  (`0x80076FD0`/`0x80077020`), the array-element deleter and owner destructor
  (`0x800773A0`/`0x800773E0`), the cell-flag resolver (`0x800774A0`), the
  layer-presence test (`0x80077980`), the effect-slot claimer and visibility
  byte (`0x80078410`/`0x800784D0`), the per-frame scroll chain
  (`0x80079710`) with its two clamps (`0x800799E0`/`0x80079AA0`), and the
  scroll-window seed (`0x80079B60`).
- **`dBgActorManager`** (`0x8007E180..0x8007EBF0`) - the constructor and
  destructor, the entry-array builder (`0x8007E3D0`/`0x8007E430`), the free-slot
  claimer (`0x8007E7F0`), the four rail-name comparisons (`0x8007E2C0`), the
  entry reset/fill pair (`0x8007EA30`/`0x8007EA60`), the descriptor-driven
  creation (`0x8007EA80`), the entry release (`0x8007EB70`) and the two
  descriptor readers (`0x8007EBC0`/`0x8007EBF0`). `0x8007CA70` (a two-word swap
  used by the camera code) rides along with this cluster.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80075090` | `fn_80075090` | 272 | high |
| `0x80076DC0` | `fn_80076dc0` | 496 | high |
| `0x80076FD0` | `fn_80076fd0` | 68 | medium |
| `0x80077020` | `fn_80077020` | 64 | high |
| `0x800773A0` | `fn_800773a0` | 64 | high |
| `0x800773E0` | `fn_800773e0` | 144 | high |
| `0x800774A0` | `fn_800774a0` | 120 | medium |
| `0x80077980` | `fn_80077980` | 116 | high |
| `0x80078410` | `fn_80078410` | 88 | high |
| `0x800784D0` | `fn_800784d0` | 64 | high |
| `0x80079710` | `fn_80079710` | 108 | medium |
| `0x800799E0` | `fn_800799e0` | 188 | medium |
| `0x80079AA0` | `fn_80079aa0` | 180 | medium |
| `0x80079B60` | `fn_80079b60` | 144 | high |
| `0x8007CA70` | `fn_8007ca70` | 20 | high |
| `0x8007E180` | `fn_8007e180` | 80 | high |
| `0x8007E1D0` | `fn_8007e1d0` | 236 | medium |
| `0x8007E2C0` | `fn_8007e2c0` | 264 | medium |
| `0x8007E3D0` | `fn_8007e3d0` | 88 | high |
| `0x8007E430` | `fn_8007e430` | 176 | high |
| `0x8007E7F0` | `fn_8007e7f0` | 108 | medium |
| `0x8007EA30` | `fn_8007ea30` | 32 | high |
| `0x8007EA60` | `fn_8007ea60` | 20 | high |
| `0x8007EA80` | `fn_8007ea80` | 240 | medium |
| `0x8007EB70` | `fn_8007eb70` | 68 | high |
| `0x8007EBC0` | `fn_8007ebc0` | 44 | high |
| `0x8007EBF0` | `fn_8007ebf0` | 36 | high |

## Deliberately skipped

- `0x80077860` (568 bytes) - the tile-edit helper. It takes `param_3`/`param_4`
  from its caller, discards `param_1`/`param_2`, and then uses the two halves of
  the 64-bit `0x802DCD70` lock result as the owner and the cell index. Which
  half is which cannot be proven from the export, so no body was written.
- `0x8007E4E0` (780 bytes) - the actor placement pass. The body is dominated by
  `ldexpf`/`unaff_GQR0` paired-single blocks that feed `local_8`/`local_18`/
  `local_28` from register-only `in_f29..in_f31`/`in_ps29_1..in_ps31_1`
  arguments, so the three floats it publishes cannot be recovered.
- `0x8007E860` (464 bytes) - the entry-table scan that `0x8007E430` calls in
  modes `0` and `1`. The grid walk itself is readable, but it consumes three
  `r2` floats and hands `FUN_8007E7F0` a stack-packed argument list whose order
  the export does not print; `0x8007E430` keeps calling it by address.
- `0x8007E0E0` (136 bytes) - the scenery-height sum. Its three operands are
  paired-single loads guarded by `ldexpf` blocks whose quantisation factors are
  not visible, and the `r13-0x5590` operand is a small-data float whose role is
  not established, so the arithmetic cannot be stated honestly.
- `0x80076530`, `0x800766F0` and `0x80077A00` (924 bytes) - the remaining
  neighbours of the camera owner. The first is dominated by a paired-single
  `GQR0` fragment, the second uses the same two `0x802DCD70` halves as
  `0x80077860`, and the third needs the `0x8006C420` owner layout plus the
  `0x80077xxx` argument lists that the export does not print.
- `0x8007E2C0`'s `+0x2C == 0` branch is reproduced exactly as printed (the same
  idiom as `copy_unit_names`), even though the following `+0x8` load can only
  succeed on the non-null path; the body is `medium` for that reason.
- The rest of the bucket was left for later waves: the `0x80070130..0x80076530`
  runs, the `0x8007A6B0..0x8007CA90` block and the `0x8007FB10..0x8007FFA0` tail.
  Their exports either lose register-only arguments (`FUN_800775a0`,
  `FUN_80077700`, `FUN_800777b0` print no parameter list) or need the
  `0x8006C420` owner layout that the earlier waves did not map. The two runs
  chosen here are the ones whose field map is complete.

## Verification

- Every address and byte size matches `functions.tsv` (27 rows, 3528 bytes).
- Every evidence file exists at the cited path under `work/ghidra/main-gb/`.
- Every row has exactly one body in `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave70` reports `OK` (27 rows in
  `wave70-stage.csv`).
