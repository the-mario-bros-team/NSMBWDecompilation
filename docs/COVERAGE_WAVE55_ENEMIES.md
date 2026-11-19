# Coverage - Wave 55 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records one lane of Wave 55: the opening of the `0x80AC0000` bucket
of REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row
below corresponds to a real `pub unsafe fn` body in that file; the `evidence`
column of the lane CSV names the Ghidra C export each body was reconstructed
from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 3,552 | 14 | 14 |

## What this pass covers

The pass is two runs inside `0x80AC0000`, both read straight from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

- **Shared five-entry skeleton, `0x80AC0280..0x80AC0630`.** The
  registration/normaliser pair (`0x80AC0280`, `0x80AC02E0`), four 12-byte
  constant producers (`0x80358004`, `0x80358084`, `0x80357BB4`, `0x80357BF4`),
  the `+0x30` chain walker, the `:`-suffix name predicate, and the two 48-byte
  deleting-destructor forwarders. This is the fifth copy of a shape already
  documented at waves 46, 53 and 54; the bodies are byte-for-byte siblings.
- **`lift_maruta` lift actor, `0x80AC0660..0x80AC0E00`.** The `0x5E8`-byte
  constructor (vtable `0x80B58720`, twelve-slot table at `+0x550`, latch at
  `+0x5E2`), the second normaliser copy, the `g3d/lift_maruta.brres` model
  binder, the slot clear/test helpers, the first-free-slot allocator, and the
  per-frame speed step that walks all twelve slots and emits sound event
  `0x2B2`.
- **World-map `mecha_koopa` actor, `0x80AC16D0..0x80AC1ED0`.** The
  `g3d/mecha_koopa.brres` variant binder (two clip-driven sub-objects at
  `+0x588` and `+0x5C0`), the `+0x7C` matrix rebuild from the owner at
  `+0x394`, the variant word copy to `+0x164`, the `Wm_en_comattack` and
  `Wm_mr_hardhit` effect spawners, both attack-record initialisers, the
  `0x80064740` forwarder, the main frame step (sound `0x20B`) and the
  countdown step (sound `0x20C`, phase-2 bind at `0x61`).

The class label `mecha_koopa` and the two effect names are surviving string
evidence. `lift_maruta` is likewise named by the archive and node strings.
Every other role is `// inferred:` where the export does not name it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AC0280` | `fn_80ac0280` | 88 | high |
| `0x80AC02E0` | `fn_80ac02e0` | 92 | high |
| `0x80AC0340` | `fn_80ac0340` | 12 | high |
| `0x80AC0350` | `fn_80ac0350` | 12 | high |
| `0x80AC0360` | `fn_80ac0360` | 12 | high |
| `0x80AC0370` | `fn_80ac0370` | 12 | high |
| `0x80AC0460` | `fn_80ac0460` | 224 | medium |
| `0x80AC0540` | `fn_80ac0540` | 136 | medium |
| `0x80AC0600` | `fn_80ac0600` | 48 | medium |
| `0x80AC0630` | `fn_80ac0630` | 48 | medium |
| `0x80AC0660` | `fn_80ac0660` | 148 | medium |
| `0x80AC0710` | `fn_80ac0710` | 64 | high |
| `0x80AC0A90` | `fn_80ac0a90` | 172 | high |
| `0x80AC0D60` | `fn_80ac0d60` | 36 | high |
| `0x80AC0D90` | `fn_80ac0d90` | 56 | high |
| `0x80AC0DD0` | `fn_80ac0dd0` | 44 | medium |
| `0x80AC0E00` | `fn_80ac0e00` | 300 | medium |
| `0x80AC16D0` | `fn_80ac16d0` | 300 | high |
| `0x80AC1920` | `fn_80ac1920` | 440 | medium |
| `0x80AC1AE0` | `fn_80ac1ae0` | 28 | high |
| `0x80AC1B00` | `fn_80ac1b00` | 140 | high |
| `0x80AC1B90` | `fn_80ac1b90` | 84 | high |
| `0x80AC1BF0` | `fn_80ac1bf0` | 224 | medium |
| `0x80AC1CD0` | `fn_80ac1cd0` | 36 | medium |
| `0x80AC1D00` | `fn_80ac1d00` | 76 | medium |
| `0x80AC1D50` | `fn_80ac1d50` | 192 | medium |
| `0x80AC1E10` | `fn_80ac1e10` | 180 | medium |
| `0x80AC1ED0` | `fn_80ac1ed0` | 348 | medium |

## Deliberate skips

- **`0x80AC0750` (240 bytes).** The Ghidra export contains unresolved
  `unaff_GQR0` paired-single conversion artifacts and duplicated `ldexpf`
  guards, so the float maths and the angle clamp cannot be reconstructed
  honestly. [`fn_80ac0dd0`] keeps the raw call address with a note.
- **`0x80AC0840` (396 bytes).** Same problem with `unaff_GQR0` and
  `unaff_GQR3`; the export also shows unreachable-block warnings. Left
  untranslated and called by address from [`fn_80ac0e00`].
- **`0x80AC1800` (276 bytes).** Same GQR artifacts; it is the variant binder
  called by [`fn_80ac1d00`] and [`fn_80ac1e10`], so those two bodies keep the
  call but not the binder body.
- **The rest of the `0x80AC0000` bucket.** 148 functions remain untranslated
  outside these runs. The next contiguous candidates are the
  `0x80AC2220..0x80AC3A60` block; it was left untouched to keep this lane
  reviewable.
- **Hidden `r12` this-adjust operands.** The two deleting-destructor
  forwarders are `translated`, but their `0x802DCBFC(param_2, param_2,
  param_1)` call cannot model the hidden register; each doc block says so.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  3,552 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`; `0x80ACE540`
  (`alloc_rail_poly_child_actor`) is the only translated address in this
  bucket and is not repeated here.
- `cargo check --message-format=short` reports no error for
  `src/game/enemies.rs` (the only remaining error names
  `src/game/worldmap.rs`, another Wave 55 lane).
- `node tools/check-lane-csv.mjs wave55` is expected to print `OK` with 28
  rows for `wave55-enemies.csv`.
