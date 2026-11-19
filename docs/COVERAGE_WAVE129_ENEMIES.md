# Coverage - Wave 129 (`game::enemies`, wave-128 specification and `0x80B20000` tail)

## How to read this file

This page records the Wave 129 `game::enemies` lane: 33 functions of REL
module 3, `d_enemiesNP`, appended to `src/game/enemies.rs` as sections 110 and
111. Every row of the machine-readable mirror
(`ledger/agents/wave129-enemies.csv`) is one real `pub unsafe fn fn_<addr>`
body, and the `evidence` column names the Ghidra C export each body was
reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values. The wave-128 specification's
constant table was re-checked word by word against
`work/rels/elf/d_enemiesNP.elf` at the mapping in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`); all 42 float constants
of that specification matched, as did the `1.0` word at `0x80B32A9C` used by
the two lift clamps.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 33 | 10,620 | 24 | 9 |

The lane has two halves:

- **12 bodies** are the "ready" and "conditional" rows of
  `docs/SPEC_WAVE128_ENEMIES.md`. They were written from that page's
  instruction-level decode and re-verified against the Ghidra C export.
- **21 bodies** are the smallest still-open entries of the planner bucket
  `0x80B20000`. 49 of that bucket's 139 functions were open when this pass
  started; this pass takes 21 and leaves 28.

## What this pass covers

- **The twelve specification bodies (`0x80A36C10..0x80AEFCF0`).** Three
  matrix emitters (`0x80AAC360`, `0x80AAC610`, `0x80AAC870`) that rebuild the
  `+0x58C`/`+0x5CC`/`+0x60C` model nodes from the `0xD50..0xD60` angle table;
  the per-part builder `0x80A7C7D0` over the 16-entry table at `0x80B2DFB0`;
  two state machines (`0x80A37830` drop/bounce and `0x80A36C10` Choropu
  appear/escape) and the `+0x8E3` timer machine `0x80AFFC90`; the bubble step
  `0x80AD3D50`; the two-actor bounce exchange `0x80AB5710`; the `fishbone`
  and `red_ring` constructors `0x80AB1510`/`0x80AEFCF0`; and the node-list
  contact dispatcher `0x80AA8470`.
- **The `0x80B20000` tail (21 bodies, `0x80B22B40..0x80B29DD0`).** Five
  `0x802DCBFC` argument-swapping forwarders (`0x80B22B40`, `0x80B22B70`,
  `0x80B28AC0`, `0x80B28AF0`, `0x80B29D70`); the `+0x72C`/`+0x74x` state
  tails of one controller actor (`0x80B25D90`, `0x80B265A0`, `0x80B265C0`,
  `0x80B26A10`); the `+0x720/+0x724` lift clamps and their state entries
  (`0x80B26620`, `0x80B26660`, `0x80B266A0`, `0x80B266E0`); the four-slot
  `+0x8C4` per-player refresh `0x80B28310`; two record-dispose forwarders
  (`0x80B289A0`, `0x80B29C50`); the class-3 `+0x440` triple copy
  `0x80B29100`; the `+0x6B8` node slot `0x80B28F30`; the `0x550`-byte
  constructor `0x80B29DD0`; and the `kanaami`/`box_wood` model binders
  `0x80B27C10`/`0x80B292D0`.
- **Three section-local adapters** make the register shapes explicit:
  `w129_kind_test` (the two-call `+0x394` vtable chain used twelve times by
  `0x80AB5710` and once by `0x80AD3D50`), `w129_slot_trans_yaw` and
  `w129_slot_grounded` (the model-node matrix recipes), and
  `w129_call_f32_6` (the `+0xA0` appear-effect call of `0x80A36C10`).

### Two corrections to the wave-128 specification

The specification is instruction-exact everywhere it was re-checked, but two
statements were overtaken by evidence:

- **`0x80AD3D50`'s contact record.** The spec lists
  `fn_80087F40(mgr, &a[0], *(u8*)(this+0x694), 8.0f)`. The callee's own export
  (`work/ghidra/main-gb/export/80087f40_FUN_80087f40.c`) reads a single
  14-byte record: two floats at `+0`, three bytes at `+8..+0xB`, and a `u16`
  at `+0xC`. The body here therefore builds that record and passes it as one
  pointer, which is what the caller's stack layout shows. The three
  `0x80B30920`-relative floats and the two flag bytes are unchanged.
- **`0x80AD3D50`'s `0x800D0470` argument order.** The spec's prose block puts
  the `3` in `r5` and the scene byte in `r6`; its own call table and the
  export agree on the reverse (`r5` = scene byte, `r6` = `3`/`0`). The body
  follows the export and the call table.

### Why nine rows are `medium`

- **Five `0x802DCBFC` forwarders.** The register shuffle is literal, but the
  hidden `r12` this-adjust pointer cannot be modelled, exactly as wave 128
  recorded for `0x80A83D80`/`0x80A83DB0`.
- **Three `main.dol`-dependent bodies.** `0x80AFFC90` reads seven trajectory
  floats from `0x80B781C8..0x80B781EC`; `0x80A36C10` reads three globals and
  calls two `main.dol` tile helpers; `0x80AA8470` folds `0x8042A310` into its
  hard-hit `f1`. The control flow and every offset are exact; only the
  external values are outside this REL.
- **One inferred receiver.** `0x80B28F30`'s indirect call prints no argument,
  so the receiver is `this` because `r3` is not reloaded. That is the only
  reading consistent with the 48-byte body, but it stays `medium`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A37830` | `fn_80a37830` | 1148 | high |
| `0x80AB5710` | `fn_80ab5710` | 1064 | high |
| `0x80AAC870` | `fn_80aac870` | 900 | high |
| `0x80AFFC90` | `fn_80affc90` | 788 | medium |
| `0x80AD3D50` | `fn_80ad3d50` | 688 | high |
| `0x80AAC360` | `fn_80aac360` | 680 | high |
| `0x80A7C7D0` | `fn_80a7c7d0` | 628 | high |
| `0x80AB1510` | `fn_80ab1510` | 624 | high |
| `0x80A36C10` | `fn_80a36c10` | 608 | medium |
| `0x80AEFCF0` | `fn_80aefcf0` | 600 | high |
| `0x80AAC610` | `fn_80aac610` | 596 | high |
| `0x80AA8470` | `fn_80aa8470` | 592 | medium |
| `0x80B22B40` | `fn_80b22b40` | 48 | medium |
| `0x80B22B70` | `fn_80b22b70` | 48 | medium |
| `0x80B28AC0` | `fn_80b28ac0` | 48 | medium |
| `0x80B28AF0` | `fn_80b28af0` | 48 | medium |
| `0x80B29D70` | `fn_80b29d70` | 48 | medium |
| `0x80B25D90` | `fn_80b25d90` | 68 | high |
| `0x80B265A0` | `fn_80b265a0` | 24 | high |
| `0x80B265C0` | `fn_80b265c0` | 84 | high |
| `0x80B26620` | `fn_80b26620` | 64 | high |
| `0x80B26660` | `fn_80b26660` | 60 | high |
| `0x80B266A0` | `fn_80b266a0` | 64 | high |
| `0x80B266E0` | `fn_80b266e0` | 80 | high |
| `0x80B26A10` | `fn_80b26a10` | 168 | high |
| `0x80B28310` | `fn_80b28310` | 96 | high |
| `0x80B289A0` | `fn_80b289a0` | 88 | high |
| `0x80B29C50` | `fn_80b29c50` | 88 | high |
| `0x80B29100` | `fn_80b29100` | 52 | high |
| `0x80B28F30` | `fn_80b28f30` | 48 | medium |
| `0x80B29DD0` | `fn_80b29dd0` | 100 | high |
| `0x80B27C10` | `fn_80b27c10` | 196 | high |
| `0x80B292D0` | `fn_80b292d0` | 184 | high |

## Deliberate skips

- **`0x80B23480` (180 bytes) and `0x80B23890` (156 bytes).** The first
  builds its angle from `unaff_GQR3` quantiser fiction (`ldexpf` calls that
  are not in the original code). The second loses the clip-name argument of
  `0x8023A0B0`, so the `g3d` lookup cannot be reproduced honestly.
- **`0x80B250F0` (56 bytes).** The export ends in `halt_baddata` after the
  `0x8042A200 & 1` guard; the control flow past the truncation is not
  recoverable.
- **`0x80B28F90` (356 bytes).** The GQR0 prologue plus an unproven `f1` on the
  `0x80064AF0` call make the float path a guess; skipped rather than guessed.
- **`0x80B28080` (120 bytes) and the remaining medium-sized bucket rows.**
  They are readable and were left for a later pass once the 33-body target was
  met. Nothing was written for them, so no ledger row claims them.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 33 sizes sum to
  10,620 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 33 was in
  `ledger/functions.csv` before this pass.
- All 42 float constants quoted in the wave-128 specification were re-read
  from the ELF and matched, along with the `1.0` word at `0x80B32A9C`; the
  in-REL strings used by the two constructors and the two binders were
  re-read the same way.
- `cargo check --message-format=short` emits no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave129` reports
  `wave129-enemies.csv: 33 rows`, 33 unique addresses and `OK`.
- The module header was updated: cluster items 110 and 111, the function
  totals (2,954 to 2,987 public translations, 2,883 to 2,916 `fn_<addr>`
  bodies), the "start here" list and the records list.
