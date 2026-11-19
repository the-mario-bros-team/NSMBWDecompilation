# Coverage - Wave 48 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 48: the remaining parts of the
`0x80AF0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row corresponds to a real `pub unsafe fn` body in
that file; the `evidence` column of the lane CSV names the Ghidra C export each
body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). The REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 2,312 | 13 | 14 |

Six of the fourteen `medium` rows are `partial`: the 48-byte
deleting-destructor forwarders at `0x80AF4D10`, `0x80AF4D40`, `0x80AF62B0`,
`0x80AF62E0`, `0x80AF7750` and `0x80AF7780` keep their visible call but cannot
model the hidden `r12` this-adjust operand (the same limitation the earlier
`0x80A11500` and `0x80B12F20` forwarders carry).

## What this pass covers

- **`obj_tobira` door block, `0x80AF3540` and `0x80AF4BF0..0x80AF5280`.** The
  `0x528`-byte constructor at `0x80AF4D70` installs class vtable `0x80B63180`;
  `0x80AF4F90` resolves the `Sample_tobira.bti` texture and binds it into the
  model holder at `+0x540`; `0x80AF5040` reads the controller pointer table at
  `0x80377C88`, copies the door position onto the interacting actor and flips
  the open byte at `+0x38F`; `0x80AF5280` releases the holder when its owner
  word no longer matches the scene handle. `0x80AF3540` shifts the four-word
  interaction-history ring at `+0x778..+0x784`. `// inferred:` - the export
  prints no class name; the label `obj_tobira` rests on the `Sample_tobira.bti`
  texture and the door-shaped `+0x38F` toggle.
- **`daEnRotBlock_c` lift helpers, `0x80AF57A0..0x80AF63D0`.** The constructor
  body at `0x80AF57A0` installs vtable `0x80B63988`, and `0x80AF5DA0` binds
  `g3d/test_lift.brres` plus its `test_lift` node. `0x80AF5BE0` finds the
  matching player, snapshots its position and enters the
  `daEnRotBlock_c::StateID_Move` descriptor at `0x80B77BB0`. Three 12-byte
  extractors decode the spawn word at `+0x04`; `0x80AF6310` is the adjacent
  `0x588`-byte constructor with vtable `0x80B63CD8`; `0x80AF63D0` is an empty
  hook.
- **`daEnRotarionGhost_c` helpers, `0x80AF7750..0x80AF7BA0`.** The `0x7D8`-byte
  constructor at `0x80AF77B0` installs the class vtable `0x80B643F8` and the two
  `0x803295D8`-rooted helper pairs; `0x80AF78A0` is a constant producer
  (`0x80377A04`); `0x80AF7B20` dispatches to the owner selected by `+0x7D1`;
  `0x80AF7B70` releases the move helper; `0x80AF7BA0` rebuilds the model matrix
  at `+0x7C` from the pose and forwards it to the active owner. `// inferred:`
  - the grouping rests on the shared `+0x6B8`/`+0x6F8` holders with the
  `block_roulette` binder at `0x80AF7CA0` and on its position inside the
  `daEnRotarionGhost_c` registration block; the export prints no class name.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AF3540` | `fn_80af3540` | 36 | high |
| `0x80AF4BF0` | `fn_80af4bf0` | 88 | high |
| `0x80AF4D10` | `fn_80af4d10` | 48 | medium (partial) |
| `0x80AF4D40` | `fn_80af4d40` | 48 | medium (partial) |
| `0x80AF4D70` | `fn_80af4d70` | 76 | high |
| `0x80AF4F90` | `fn_80af4f90` | 168 | medium |
| `0x80AF5040` | `fn_80af5040` | 220 | medium |
| `0x80AF5280` | `fn_80af5280` | 88 | medium |
| `0x80AF57A0` | `fn_80af57a0` | 104 | high |
| `0x80AF5BE0` | `fn_80af5be0` | 92 | medium |
| `0x80AF5D70` | `fn_80af5d70` | 12 | high |
| `0x80AF5D80` | `fn_80af5d80` | 12 | high |
| `0x80AF5D90` | `fn_80af5d90` | 12 | high |
| `0x80AF5DA0` | `fn_80af5da0` | 164 | high |
| `0x80AF5E50` | `fn_80af5e50` | 140 | medium |
| `0x80AF5EE0` | `fn_80af5ee0` | 104 | medium |
| `0x80AF62B0` | `fn_80af62b0` | 48 | medium (partial) |
| `0x80AF62E0` | `fn_80af62e0` | 48 | medium (partial) |
| `0x80AF6310` | `fn_80af6310` | 100 | high |
| `0x80AF63D0` | `fn_80af63d0` | 4 | high |
| `0x80AF7750` | `fn_80af7750` | 48 | medium (partial) |
| `0x80AF7780` | `fn_80af7780` | 48 | medium (partial) |
| `0x80AF77B0` | `fn_80af77b0` | 228 | high |
| `0x80AF78A0` | `fn_80af78a0` | 12 | high |
| `0x80AF7B20` | `fn_80af7b20` | 80 | medium |
| `0x80AF7B70` | `fn_80af7b70` | 40 | high |
| `0x80AF7BA0` | `fn_80af7ba0` | 244 | medium |

## Deliberate skips

- **The rest of the `0x80AF0000` bucket.** 174 other functions remain for later
  passes, including the `MG_remo_cannon` block at
  `0x80AF20F0..0x80AF2810`, the `teresa` block at `0x80AF6840..0x80AF7330`,
  the `block_roulette` model binder at `0x80AF7CA0` and the
  `daEnSakasaTogezo_c` / `daEnSanboEl_c` tails at `0x80AF9170..0x80AFFFB0`.
- **`0x80AF3840`.** The export is a truncated `halt_baddata()` block with no
  reachable instructions, so no body can be reconstructed.
- **The `0x80AF5C60` step.** Its Ghidra output depends on uninitialised `GQR0`
  and `GQR3` register state (`ldexpf` of the graphics-quantiser fields), which
  cannot be expressed honestly in Rust; it is left for a pass that can read the
  paired quantiser setup.
- **Opaque callees.** `0x8015F800`, `0x802B9280`, `0x802DCBFC`, `0x80094E80`,
  `0x80069020`, `0x80169CD0`, `0x8016A2F0`, `0x8016A3A0`, `0x8016A5D0`,
  `0x8016A5E0`, `0x8007F7A0`, `0x8007F950`, `0x8008F1B0`, `0x8008F310`,
  `0x800077A0`, `0x8006CED0`, `0x80064AF0`, `0x801C0BD0`, `0x8016EDD0`,
  `0x80162D50`, `0x801C1650` and `0x802BE470` are kept by address, with their
  argument counts taken from the Ghidra call sites.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  2,312 bytes.
- Every evidence file exists at the cited repo-relative path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`; the `0x80AF0000` bucket
  held seven earlier rows (`0x80AF1C00`, `0x80AF5FD0`, `0x80AF7520`,
  `0x80AF7630`, `0x80AF98A0`, `0x80AFE3C0`, `0x80AFEFD0`), none of which is
  reused.
- `cargo check --message-format=short` reports no error for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave48` prints `OK` for this lane's CSV.
