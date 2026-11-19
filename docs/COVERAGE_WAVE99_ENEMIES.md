# Coverage - Wave 99 (`game::enemies`, the `0x80A30000` bucket)

## How to read this file

This page records the Wave 99 `game::enemies` lane: the `ankou` and `chorobon`
actor shells of the `0x80A30000` bucket of REL module 3, `d_enemiesNP`,
promoted to `src/game/enemies.rs` (section 80 of that file). Every row below is
one real `pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave99-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The
`ankou` and `chorobon` names come from the model strings in the module data
section (`g3d/ankou.brres`, `ankou`, `ankou_fog`, `g3d/chorobon.brres`,
`chorobon`), not from a surviving class symbol.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,184 | 13 | 12 |

Five rows are `low` confidence: the four 48-byte argument-swapping forwarders
`0x80A32EB0`, `0x80A32EE0`, `0x80A349E0` and `0x80A34A10`, whose hidden-`r12`
this-adjust pointer is not modelled (the same limitation recorded for the
`0x802DCBFC` copies in waves 56, 75, 80, 81, 82, 97 and 98), and the leash test
`0x80A339F0`, whose sign term uses the compiler's
`CONCAT44(0x43300000, x) - 2^52` integer-to-double conversion that the export
prints with only half of the `f64` constant.

The bucket holds 149 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 57 were already in
`ledger/functions.csv` before this pass, 30 are claimed here and 62 remain.

## What this pass covers

Two complete actor shells, in address order, `0x80A32D90..0x80A35680`:

- **`ankou` (class vtable `0x80B3D558`, allocation `0x6E0`).** The constructor
  `0x80A32F10` builds the model block `+0x524`, the `+0x544` model holder, the
  `+0x584` animation holder, the two `+0x604`/`+0x630` controller pairs and the
  `+0x65C..+0x6B8` state tail. `0x80A33230` binds `g3d/ankou.brres`, the
  `ankou` node and the `swim` clip, then hands the `+0x5BC` fog animator to
  `0x80A34240`, which binds `ankou_fog` twice (one `0x32C`-frame holder and two
  clip records). `0x80A343B0` rebuilds the fog object's `+0x4` transform from a
  world position.
- **The `ankou` per-frame steps.** `0x80A33350` rewrites the two side tables
  `0x80B3D510` (`0.5`/`-0.5`) and `0x80B3D518` (`0.25`/`-0.25`) and dispatches
  on bit 28 of the spawn word; `0x80A33B20` re-places the walk clip and the
  `+0xE8..+0xF0` offset pair; `0x80A33BD0` normalises the recorded direction
  and calls `0x80A33970`; `0x80A33C70` re-tests the `0x80B70218` state through
  the `+0x394` holder and stores the `16.0`-scaled angle speeds
  `+0x668`/`+0x66C`; `0x80A33DC0` turns the actor at the two per-side bounds;
  `0x80A33EF0` switches on the range test; `0x80A33F90` places the `wait` clip;
  `0x80A34000` only steps the three holders.
- **The `ankou` helpers.** `0x80A33920` is the `{±10000}`/`{±1024}` range test,
  `0x80A339F0` the `96.0`-reach leash test that retires the actor,
  `0x80A33470` copies the joint-3 and joint-6 matrices into `+0x6BC`/`+0x6CC`
  and `0x80A337E0` rearms the `+0x67C` timer.
- **`chorobon` (class vtable `0x80B3D998`, allocation `0x6C0`).** The
  constructor `0x80A34A40`, the model binder `0x80A34C40`
  (`g3d/chorobon.brres`, the `chorobon` node and the `run` clip), the
  view-change rebind `0x80A34F00` and the spawn placement `0x80A350E0`, whose
  speed is entry `((word >> 2) & 0x1C) + (word & 1) * 0x20` of the eight-float
  table `0x80B2C0A8` (`1.0`, `1.5`, `2.0`, `3.2` and their negatives).
- **The teardown shell.** `0x80A32D90` and its copy `0x80A348C0` are the two
  deleting destructors; `0x80A32EB0`, `0x80A32EE0`, `0x80A349E0` and
  `0x80A34A10` are the four `0x802DCBFC` forwarders. `0x80A35470` steps the
  `chorobon` holder before the untranslated follow helper, and `0x80A35680` is
  the contact callback that runs the class `+0x3FC` slot with `3.0`.

### Identity evidence

- **No class name survives.** `0x80B3D558` and `0x80B3D998` are referenced only
  by their constructors; no `daEn*_c` string can be tied to them. The actors are
  described by size, vtable and field offsets only.
- **The strings are literal.** `g3d/ankou.brres`, `ankou`, `swim`, `wait` and
  `ankou_fog` were read from `work/rels/elf/d_enemiesNP.elf` at
  `0x80B3D520..0x80B3D548`; `g3d/chorobon.brres`, `chorobon` and `run` at
  `0x80B3D974..0x80B3D994`.
- **Every constant was dumped, not assumed.** `0x80B3D510..0x80B3D51C` is
  `0.5`, `-0.5`, `0.25`, `-0.25`; `0x80B2BF50..0x80B2BFD4` holds `0.0`, `0.5`,
  `0.25`, `0.0`, `16.0`, `96.0` and `1.0`; `0x80B2C014`/`0x80B2C018` are
  `0.5`/`0.03125`; `0x80B2C0A8..0x80B2C0E4` is `1.0`, `1.5`, `2.0`, `3.2`,
  `-1.0`, `-1.5`, `-2.0`, `-3.2`; `0x80B2C0E8` is `3.0`; `0x80B2BF90` is the
  halfword pair `0x0400`, `0xFC00`. The `f64` constant whose high word sits at
  `0x80B2C020` is `4503599627370496.0` (`2^52`), not the `176.0` the export
  prints.
- **The state words are BSS.** `0x80B70218`, `0x80B70258` and
  `0x80B701C0..0x80B701C8` lie in the module BSS segment
  (`0x80B6E500..0x80B799E0`, `p_filesz = 0`), so they are entered by address
  and their contents are not claimed.

### Deliberate skips

- **`0x80A334D0` (352 bytes) and `0x80A33630` (380 bytes).** Both export with a
  bogus `unaff_GQR0` register and `ldexpf` fragments from the paired-single
  quantisation code; the actual arithmetic cannot be read from that text.
- **`0x80A337B0` (4 bytes) and `0x80A337C0` (32 bytes).** `functions.tsv` lists
  them, but both export as `halt_baddata()` ("control flow encountered bad
  instruction data"), so no body can be reconstructed honestly.
- **`0x80A34F80` (100 bytes).** The effect registration helper calls
  `0x800B8700` on `+0x484` and `0x802DCA98(&local, 0x80A0B5A0, 0x3C, 1)`; it
  belongs to the effect cluster rather than the actor shell and was left out of
  this 30-body pass.
- **`0x80A35170` (464 bytes), `0x80A35370` (232 bytes) and `0x80A35570`
  (28 bytes).** `0x80A35170` is a long counter/effect handler,
  `0x80A35370` needs the `0x80168FE0` rate conversion reconstructed from a
  `CONCAT44` expression, and `0x80A35570` exports as an unrecovered jump table
  ("Treating indirect jump as call"). They are named in the file as call sites
  only (see `W99_FN_80A35170`) and left for a pass with the disassembly.
- **The rest of the bucket: 62 functions.** Of those, 7 are this run's skips
  above, and the nearest untranslated blocks are
  `0x80A35ED0..0x80A38050` (25 functions, including the 1,148-byte
  `0x80A37830` and the 608-byte `0x80A36C10`), `0x80A390F0..0x80A398C0`
  (6), `0x80A3A4A0` (2,620 bytes), `0x80A3CA80` (288), `0x80A3DBE0` (284),
  `0x80A3DEA0` (2,940 bytes) and the dense tail
  `0x80A3F280..0x80A3FFF0` (28 functions).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A32D90` | `fn_80a32d90` | 88 | high |
| `0x80A32EB0` | `fn_80a32eb0` | 48 | low |
| `0x80A32EE0` | `fn_80a32ee0` | 48 | low |
| `0x80A32F10` | `fn_80a32f10` | 308 | medium |
| `0x80A33230` | `fn_80a33230` | 288 | medium |
| `0x80A33350` | `fn_80a33350` | 152 | medium |
| `0x80A33470` | `fn_80a33470` | 96 | high |
| `0x80A337E0` | `fn_80a337e0` | 24 | high |
| `0x80A33920` | `fn_80a33920` | 72 | medium |
| `0x80A33970` | `fn_80a33970` | 124 | medium |
| `0x80A339F0` | `fn_80a339f0` | 192 | low |
| `0x80A33AB0` | `fn_80a33ab0` | 100 | medium |
| `0x80A33B20` | `fn_80a33b20` | 160 | high |
| `0x80A33BD0` | `fn_80a33bd0` | 160 | medium |
| `0x80A33C70` | `fn_80a33c70` | 312 | medium |
| `0x80A33DC0` | `fn_80a33dc0` | 232 | high |
| `0x80A33EF0` | `fn_80a33ef0` | 160 | high |
| `0x80A33F90` | `fn_80a33f90` | 84 | high |
| `0x80A34000` | `fn_80a34000` | 100 | high |
| `0x80A34240` | `fn_80a34240` | 368 | medium |
| `0x80A343B0` | `fn_80a343b0` | 136 | high |
| `0x80A348C0` | `fn_80a348c0` | 88 | high |
| `0x80A349E0` | `fn_80a349e0` | 48 | low |
| `0x80A34A10` | `fn_80a34a10` | 48 | low |
| `0x80A34A40` | `fn_80a34a40` | 140 | high |
| `0x80A34C40` | `fn_80a34c40` | 228 | high |
| `0x80A34F00` | `fn_80a34f00` | 100 | medium |
| `0x80A350E0` | `fn_80a350e0` | 136 | medium |
| `0x80A35470` | `fn_80a35470` | 68 | medium |
| `0x80A35680` | `fn_80a35680` | 76 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,184 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv` (the bucket's 57 existing rows are
  untouched).
- `node tools/check-lane-csv.mjs wave99` reports `OK`; this lane's CSV has 30
  rows, 7 fields each.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace build itself is currently stopped by
  errors in other Wave 99 lanes (`src/audio/seq.rs`, `src/resource/archive.rs`
  and others), which do not name this file.
