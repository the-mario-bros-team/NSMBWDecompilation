# Coverage - Wave 45 (`game::stage`, fukidashi speech-balloon object)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line in the body.

The band `0x800B0000` is the mixed program-flow bucket described in
[`docs/COVERAGE_WAVE39_STAGE.md`](COVERAGE_WAVE39_STAGE.md). This pass takes one
contiguous run inside it: the **fukidashi** (speech-balloon) object that hangs
off the course manager. The balloon layout is inferred from the bodies below:

- `+0x04` NW4R layout wrapper, `+0x0C` layout interface, `+0x19C` state
  interface, `+0x1A0`/`+0x1A4` value objects, `+0x1B4` state triplet.
- `+0x1D8` optional three-float override position, `+0x1DC..+0x1E4` per-frame
  offsets, `+0x1E8`/`+0x1EC` animation values, `+0x1F0` tracked owner id.
- `+0x1F4` root pane, `+0x1FC..+0x208` four colour panes, `+0x20C` text
  holder, `+0x210` translation triple, `+0x214` secondary pane, `+0x218`
  text pane.
- `+0x21C` character id, `+0x220` balloon kind, `+0x224` timer, `+0x228`
  timer limit, `+0x22C` step count, `+0x230` forced timer, `+0x234` timer
  step, `+0x238`/`+0x239`/`+0x23A` created/active/exit flags.

The course-manager side of the link is the existing
[`G_STAGE_SLOT_OWNER`](../src/game/stage.rs) object (`r13-0x5704`) and its
`+0xAFE..+0xB38` visibility grid and `+0x3D9` byte.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 26 | 5244 | 18 | 8 |

## What this pass covers

- **Constructor / destructor pair** (`0x800B14E0`, `0x800B16E0`) - the
  constructor builds the layout wrapper and the state triple and registers the
  `StateID_StartWait` record; the destructor clears the `r13-0x5748` global and
  releases the `+0x1B4` triplet and the `+4` layout.
- **Lifetime helpers** (`0x800B15D0`, `0x800B1610`, `0x800B1670`,
  `0x800B2C30`) - the release bodies used by the balloon and by the
  `dfukidashiInfo_c` state records. `0x800B1610` and `0x800B1670` are the same
  body at two sizes; `0x800B2890` and `0x800B28C0` are likewise a duplicate
  `0x802DCBFC` wrapper pair.
- **Layout build** (`0x800B1760`) - `fukidashiInfo_16.brlyt`, the
  `inFukidashi` animation, the `A00_fukidashi` material animation, the
  `P_afterImageUD1` and second pane bindings, the `+0x1F4` root-pane cache and
  the default timing block.
- **Owner and timer handlers** (`0x800B19C0`, `0x800B2230`, `0x800B2370`,
  `0x800B24B0`, `0x800B2450`, `0x800B2180`, `0x800B1FD0`) - owner-id tracking
  through `r13-0x5730`, the `+0x224` countdown, the course-manager grid writes,
  and the `StateID_StartWait` / `OnStage` / `Move` / `Exit` transitions.
- **Position and colour** (`0x800B1A50`, `0x800B1A90`, `0x800B1BD0`,
  `0x800B1EB0`, `0x800B2070`, `0x800B1F80`, `0x800B1910`, `0x800B1990`) - the
  two-level motion table, the subject/override position blend through
  `0x800B33D0`, the three global `RGBA8` tables, the `0x43300000` integer-to-
  float conversion, and the kind-driven pane visibility switch.
- **Spawn** (`0x800B2550`) - the kind-`8` id rewrite, the `0x8005F900` subject
  lookup and the complete `0x22C` step switch for kinds `0..0x15`.
- **State records** (`0x800B28F0`) - `StateID_StartWait`, `OnStage`, `Move`
  and `Exit`, built from the shipped rows at `0x80315478`, `0x8031549C`,
  `0x803154C0` and `0x803154E4` under the `0x802DCD74` / `0x802DCDC0` lock
  pair.

Deliberately skipped:

- `0x800B19B0` (8 bytes) and `0x800B2EB0..0x800B2F00` (8-12 byte getters) -
  below the module's stated 16-byte floor; the role of their globals is already
  visible through the translated callers.
- `0x800B2F10` and the remaining `ldexpf` / `unaff_GQR0` carriers in the
  bucket - paired-single code, consistent with the wave-39 and wave-41 skips.
- The rest of the bucket (dialogue/fukidashi text outside this run, font setup,
  save/course-scene glue, Iggy actors, lift matrices, scene graph) - separate
  systems, not part of this cluster.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800B14E0` | `fn_800b14e0` | 172 | high |
| `0x800B15D0` | `fn_800b15d0` | 64 | medium |
| `0x800B1610` | `fn_800b1610` | 96 | high |
| `0x800B1670` | `fn_800b1670` | 100 | high |
| `0x800B16E0` | `fn_800b16e0` | 128 | high |
| `0x800B1760` | `fn_800b1760` | 428 | medium |
| `0x800B1910` | `fn_800b1910` | 112 | medium |
| `0x800B1990` | `fn_800b1990` | 20 | high |
| `0x800B19C0` | `fn_800b19c0` | 132 | medium |
| `0x800B1A50` | `fn_800b1a50` | 56 | high |
| `0x800B1A90` | `fn_800b1a90` | 320 | medium |
| `0x800B1BD0` | `fn_800b1bd0` | 724 | medium |
| `0x800B1EB0` | `fn_800b1eb0` | 200 | medium |
| `0x800B1F80` | `fn_800b1f80` | 72 | high |
| `0x800B1FD0` | `fn_800b1fd0` | 72 | high |
| `0x800B2070` | `fn_800b2070` | 272 | medium |
| `0x800B2180` | `fn_800b2180` | 88 | high |
| `0x800B2230` | `fn_800b2230` | 280 | high |
| `0x800B2370` | `fn_800b2370` | 196 | high |
| `0x800B2450` | `fn_800b2450` | 96 | high |
| `0x800B24B0` | `fn_800b24b0` | 156 | high |
| `0x800B2550` | `fn_800b2550` | 452 | high |
| `0x800B2890` | `fn_800b2890` | 48 | high |
| `0x800B28C0` | `fn_800b28c0` | 48 | high |
| `0x800B28F0` | `fn_800b28f0` | 824 | high |
| `0x800B2C30` | `fn_800b2c30` | 88 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave45` reports the stage CSV as clean.

## Known limits

- The eight `medium` bodies are the ones whose field roles are inferred rather
  than proved: `0x800B1760`/`0x800B1910`/`0x800B19C0`/`0x800B1A90` name the
  layout, owner and subject fields; `0x800B1BD0` infers the pane colour slot
  order; `0x800B1EB0` infers the `+0x94..+0xA4` animation roles; `0x800B2070`
  infers the `+0x210` triple. The instruction sequences and constants are
  literal in every case.
- Two calls print no operands in the export: the `+0x19C` slot `+0x10` call in
  `0x800B1910` and the `+0x68` colour getter in `0x800B1BD0`. Both are
  translated exactly as printed (zero-argument calls) and flagged in their
  bodies.
- `0x800B15D0`'s allocator call has its argument dropped by Ghidra; the body
  restores `this` from the three sibling release helpers and is marked
  `medium`.
