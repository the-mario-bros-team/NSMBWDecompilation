# Coverage - Wave 58 (enemies: opening of the `0x80A20000` bucket)

## How to read this file

This page records the Wave 58 enemies lane: three consecutive actor blocks at
the start of the `0x80A20000` bucket of REL module 3, `d_enemiesNP`, promoted
to `src/game/enemies.rs`. Every row below corresponds to a real
`pub unsafe fn` body in that file; the `evidence` column of the lane CSV names
the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,624 | 11 | 13 | 6 |

## What this pass covers

The pass is the contiguous run `0x80A200D0..0x80A22B50`, read from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. The bucket holds 171
functions and 3 of them were already in `ledger/functions.csv`
(`0x80A2A350`, `0x80A2CC40`, `0x80A2D2D0`, all far above this run); none of
those addresses is repeated here.

The run contains three actor blocks with the same skeleton: a deleting
destructor, two 48-byte argument-swapping forwarders, an allocating
constructor, then a series of matrix/model helpers. They share offsets
(`+0x60` vtable, `+0xAC` position, `+0x100` angles, `+0x394` owner,
`+0x534` animation holder), which is why they are translated together.

- **Class X (`0x80A200D0..0x80A20E70`).** An unnamed `0x708`-byte actor
  (vtable `0x80B39054`). Covers the two forwarders, the constructor, the
  four-slot player wake scan, the two-float angle helper, the dismiss request
  and its retire path, the contact-driven two-clip switch, the state entry
  that resets six words and the dismiss mask, and the per-frame step that
  calls the untranslated effect/scan body `0x80A20900`.
- **Class Y (`0x80A21220..0x80A21A30`).** The `remo_lift_soroban` actor
  (`0x798` bytes, vtable `0x80B39478`): deleting destructor, two forwarders,
  constructor, the spawn/setup step that installs six callbacks and the
  `0x260` flag word, the holder step, the position/angle matrix rebuild, the
  `g3d/remo_lift_soroban.brres` + `change_mark_A` binder, the
  `Wm_en_burst_s` costume effect, and the blend gate.
- **Class Z (`0x80A21F90..0x80A22B50`).** The `staffRollBlock` actor
  (`0x808` bytes, vtable `0x80B399B4`, identified from the untranslated model
  binder at `0x80A22820`): deleting destructor, two forwarders, constructor,
  the main per-frame handler, the two-holder selector, the matrix/colour
  commit step, the per-node colour patch, the free-only release and the
  collision/timer gate.
- **Constants.** `0x80B2B75C = 8.0`, `0x80B2B784 = 1.0`, `0x80B2B788 = -8.0`,
  `0x80B2B78C = 8.0`, `0x80B2B798 = 0.0`, `0x80B2B79C = 5500.0` and the
  four-word table at `0x80B2B7C0` (`0xFFFFFFFF`, `0x000000FF`, `0xFFFFFF00`,
  `0x00000000`) were read byte-for-byte from the REL image
  (`work/rels/elf/d_enemiesNP.elf`). The `0x43300000`/`0x80000000` pair at
  `0x80B2B790` is the MWCC signed-int-to-double bias, not a game constant.
- **Identity evidence.** `0x80A217B0` loads
  `g3d/remo_lift_soroban.brres` and the `remo_block`/`change_mark_A` names;
  `0x80A21960` spawns the `Wm_en_burst_s` effect. The class-Z name comes from
  the `g3d/staffRollBlock.brres` loader at `0x80A22820`, which is cited as
  context but not translated.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A200D0` | `fn_80a200d0` | 48 | low |
| `0x80A20100` | `fn_80a20100` | 48 | low |
| `0x80A20130` | `fn_80a20130` | 116 | high |
| `0x80A20770` | `fn_80a20770` | 104 | medium |
| `0x80A207E0` | `fn_80a207e0` | 44 | high |
| `0x80A20B10` | `fn_80a20b10` | 180 | medium |
| `0x80A20BD0` | `fn_80a20bd0` | 400 | medium |
| `0x80A20D60` | `fn_80a20d60` | 204 | high |
| `0x80A20E70` | `fn_80a20e70` | 76 | medium |
| `0x80A21220` | `fn_80a21220` | 88 | high |
| `0x80A21340` | `fn_80a21340` | 48 | low |
| `0x80A21370` | `fn_80a21370` | 48 | low |
| `0x80A213A0` | `fn_80a213a0` | 208 | high |
| `0x80A21470` | `fn_80a21470` | 348 | medium |
| `0x80A21640` | `fn_80a21640` | 48 | high |
| `0x80A21670` | `fn_80a21670` | 40 | high |
| `0x80A216A0` | `fn_80a216a0` | 260 | medium |
| `0x80A217B0` | `fn_80a217b0` | 424 | medium |
| `0x80A21960` | `fn_80a21960` | 196 | medium |
| `0x80A21A30` | `fn_80a21a30` | 108 | medium |
| `0x80A21F90` | `fn_80a21f90` | 88 | high |
| `0x80A220B0` | `fn_80a220b0` | 48 | low |
| `0x80A220E0` | `fn_80a220e0` | 48 | low |
| `0x80A22110` | `fn_80a22110` | 232 | high |
| `0x80A223F0` | `fn_80a223f0` | 208 | medium |
| `0x80A224C0` | `fn_80a224c0` | 164 | medium |
| `0x80A225A0` | `fn_80a225a0` | 360 | medium |
| `0x80A22710` | `fn_80a22710` | 260 | medium |
| `0x80A22AE0` | `fn_80a22ae0` | 64 | high |
| `0x80A22B50` | `fn_80a22b50` | 116 | high |

## Deliberate skips

- **`0x80A20810` (228 bytes) and `0x80A20900` (520 bytes).** Both exports
  carry `unaff_GQR0`/`unaff_GQR3` conversion artifacts, duplicated `ldexpf`
  guards and merged float paths. `0x80A20900` is still called by
  [`fn_80a20e70`] as `0x80A20900`.
- **`0x80A22820` (700 bytes).** The class-Z model binder. Its export mixes a
  `CONCAT44`/`ldexpf` block-index conversion with a match on
  `block_00`/`block_broken` material names; the body was not reconstructed,
  so class Z is documented through its constructor, frame step and holder
  helpers instead.
- **`0x80A22B20` (44 bytes).** The export ends in `halt_baddata()` after a
  bad-instruction warning, so there is no body to reconstruct.
  [`fn_80a223f0`] keeps the call as `0x80A22B20`.
- **Hidden argument drops.** `0x80094E80` (base constructor) and
  `0x802B9280`/`0x802DCBFC` appear with no printed arguments at several call
  sites. The bodies pass the value that is still in `r3` (`this`), mark the
  inference in a `// inferred:` comment and are scored `medium`/`low`.
- **Forwarder identity.** The six 48-byte forwarders (`0x80A200D0`,
  `0x80A20100`, `0x80A21340`, `0x80A21370`, `0x80A220B0`, `0x80A220E0`)
  reproduce the literal `(second, second, first)` call to `0x802DCBFC`. The
  target's decompiled body reads `r12`-relative state, so neither its role nor
  the two words is presented as fact; they are scored `low`.
- **`fn_80a22710` byte insert.** The export renders the third conditional
  channel as `CONCAT21(local_24._0_2_, patch[2]) << 8`, which drops the top
  byte. The body performs the masked insert that matches the two preceding
  channels, and the deviation is recorded in the function doc; the row stays
  `medium`.
- **BSS descriptors.** `0x80B6F868`, `0x80B6F8A8`, `0x80B6F8E8` and
  `0x80B6F968` are `fState`-style objects in BSS, and `0x80352AF4` /
  `0x80352B34` are `main.dol` BSS clip objects built at runtime. No export
  prints their names, so the docs cite the addresses only.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,624 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice.
- No address was already in `ledger/functions.csv` (the bucket's three
  existing rows are `0x80A2A350`, `0x80A2CC40` and `0x80A2D2D0`).
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/enemies.rs`; the warnings in the tree belong to
  `src/runtime/bt.rs`, `src/runtime/cpp.rs` and `src/runtime/input.rs`.
- `node tools/check-lane-csv.mjs wave58` is clean for
  `wave58-enemies.csv`.
