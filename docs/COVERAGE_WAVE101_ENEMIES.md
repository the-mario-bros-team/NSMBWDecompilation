# Coverage - Wave 101 (`game::enemies`, the `0x80AB0000` bucket tail)

## How to read this file

This page records the Wave 101 `game::enemies` lane: two contiguous runs of the
`0x80AB0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 82 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave101-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The bucket holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 99 were already in
`ledger/functions.csv` before this pass, 30 are claimed here and 59 remain. The
previous pass over this bucket (`docs/COVERAGE_WAVE72_ENEMIES.md`) claimed
`0x80AB7810..0x80AB9030` and `0x80ABB3D0..0x80ABBCF0`; this pass starts where
that one stopped.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 4,104 | 15 | 9 |

Six rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80AB99D0`, `0x80AB9A00`, `0x80ABC490`, `0x80ABC4C0`, `0x80ABCFA0` and
`0x80ABCFD0`, whose hidden-`r12` this-adjust pointer is not modelled (the same
limitation recorded for the `0x802DCBFC` copies in waves 56, 72, 75, 80, 81, 82,
97, 98 and 99).

## What this pass covers

Two runs, both chosen because the exports are complete (no `unaff_GQR*`
quantiser pseudo-code and no `halt_baddata`).

- **`0x80AB98B0..0x80ABAD60`, the `lift_trampoline`/`test_lift` lift pair
  (17 functions).** The only surviving identity evidence is the resource names
  read from the REL data section:
  `g3d/lift_trampoline.brres` (`0x80B56954`), `lift_trampoline`
  (`0x80B56918`/`0x80B56970`), `ridden` (`0x80B56928`), and the long variants
  `lift_trampoline_long` (`0x80B56930`) / `ridden_long` (`0x80B56948`); then
  `g3d/test_lift.brres` (`0x80B56A64`) and `test_lift` (`0x80B56A78`).
  - The `0x548`-byte actor: the release forwarder `0x80AB98B0`, the two
    `0x802DCBFC` forwarders, the constructor `0x80AB9A30` (class vtable
    `0x80B56980`, holders `+0x394`, `+0x3B4`, `+0x3F4` with vtable
    `0x80B330E0`, `+0x42C`/`+0x438`, animator `+0x458`), the record binder
    `0x80AB9C10` (the two twelve-byte records of the `0x80B2FF20` table carry
    `{model, node, extent}` = `64.0` and `128.0`), the matrix placement
    `0x80AB9F10`, the animator tail `0x80AB9FB0` and the four-bit player mask
    `0x80AB9FE0` over the `+0x464` list.
  - The `test_lift` sibling: `0x80ABA290` creates it in place through the
    section prologue `0x802DCD60`, installs class vtable `0x80B56A88` and two
    four-word controller records at `+0x620`/`+0x65C`; `0x80ABA3C0` is its
    destructor, `0x80ABA8A0` the `test_lift` binder, `0x80ABA9F0` the
    `0x1A3`/`0x1A4` actor search, `0x80ABAA80`/`0x80ABAAD0` the two frame
    entries, `0x80ABAC40` the `+0x5E4` snapshot with the `sqrt` distance and
    `atan2` heading, and `0x80ABAD60` its destructor.
  - Constants were dumped, not assumed: `0x80B2FF88` is `16.0`, `0x80B2FF8C`
    `20.0`, `0x80B2FF90` `0.5`, `0x80B2FF98` the `f64` `2^52`, `0x80B2FFA4`
    `0.0`, `0x80B2FFA8` `8.0`, `0x80B2FFAC` `-1.0`, `0x80B2B4C4` a twelve-byte
    zero block.
- **`0x80ABC370..0x80ABD000`, the `daEnLiftRotHalf_c` tail (13 functions).**
  This is the class whose constructor is `0x80ABC4F0` (already translated as
  `alloc_lift_rot_half_actor`) and whose two `Create`/`Move` records are
  installed by `register_lift_rot_half_states` (`0x80ABB210`).
  - `0x80ABC370` is the release forwarder that table names, followed by the two
    `0x802DCBFC` forwarders `0x80ABC490`/`0x80ABC4C0`.
  - `0x80ABC610` binds the `+0x72C` player link through `0x80162D00`,
    `0x80161800` and `0x80161750`; `0x80ABC9A0` returns that link;
    `0x80ABC7E0` is the main per-frame step (controller `+0x394`, model refresh
    `0x80ABC940`, holder `+0x5BC`, and the linked player's `+0x614` active
    mask); `0x80ABCA30` starts the `body_wait` clip; `0x80ABCAB0` installs the
    zero pose record at `+0x148` and refreshes `+0x1E7` from `0x8042BB18`;
    `0x80ABCB10` is the two-phase grow step (rate pairs `1.25`/`0.125` and
    `1.0`/`0.0625`); `0x80ABCE80`/`0x80ABCFA0`/`0x80ABCFD0` close the
    descriptor block; and `0x80ABD000` is the next `0x528`-byte constructor
    (class vtable `0x80B57AA8`).
  - The rate constants were dumped: `0x80B30010` `0.0`, `0x80B30060` `1.0`,
    `0x80B30064` `1.25`, `0x80B30068` `0.125`, `0x80B3006C` `0.0625`.

### Identity evidence

- **No class name survives** for either family. `0x80B56980`, `0x80B56A88` and
  `0x80B57AA8` are referenced only by their constructors, so the objects are
  described by size, vtable and field offsets only. The `lift_trampoline`,
  `test_lift` and `body_wait` labels are literal archive/clip strings, not class
  names.
- **The names are literal.** They were read from
  `work/rels/elf/d_enemiesNP.elf` at `0x80B56918..0x80B56980` and
  `0x80B56A64..0x80B56A80`.
- **The state descriptors are BSS.** `0x80B75590`, `0x80B755D0` and
  `0x80B6F558` lie in the module BSS segment, so they are entered by address
  and their contents are not claimed.
- **Two helpers keep `fn_<address>` names.** `0x80A19DF0` and `0x80A19EE0` are
  called from `0x80ABC7E0` and `0x80ABCB10`; they lie outside this lane's runs
  and are still untranslated, so they are called by address.

### Deliberate skips

- **`0x80ABA080` (308 bytes), `0x80ABA950` (156) and `0x80ABAB50` (236).** Their
  exports are dominated by `unaff_GQR0`/`unaff_GQR3` quantiser pseudo-code and
  `ldexpf` fiction (the same nine-export hazard recorded in
  `docs/COVERAGE_WAVE72_ENEMIES.md`), so the real float arithmetic cannot be
  recovered from the C.
- **`0x80AB9AF0` (284 bytes).** The `test_lift` spawn initialiser. It is
  reconstructible in principle - the twelve-argument call is the same
  `0x8007FA40` animator install already modelled by `call12_f64x4` - but its
  three `f64` arguments come from a `GQR`-folded two-record table that the
  export does not fully preserve. Left for a pass with the disassembly.
- **The other 55 functions of the bucket.** They are the actors between
  `0x80AB0570` and `0x80ABF650` that neither this pass nor wave 72 touched; the
  largest clean candidates left are `0x80AB0570` (664 bytes), `0x80AB0EE0`
  (640) and `0x80AB5710` (1064, QGPR-damaged).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB98B0` | `fn_80ab98b0` | 88 | high |
| `0x80AB99D0` | `fn_80ab99d0` | 48 | low |
| `0x80AB9A00` | `fn_80ab9a00` | 48 | low |
| `0x80AB9A30` | `fn_80ab9a30` | 192 | high |
| `0x80AB9C10` | `fn_80ab9c10` | 356 | high |
| `0x80AB9F10` | `fn_80ab9f10` | 160 | high |
| `0x80AB9FB0` | `fn_80ab9fb0` | 40 | high |
| `0x80AB9FE0` | `fn_80ab9fe0` | 148 | medium |
| `0x80ABA290` | `fn_80aba290` | 304 | medium |
| `0x80ABA3C0` | `fn_80aba3c0` | 140 | high |
| `0x80ABA630` | `fn_80aba630` | 244 | medium |
| `0x80ABA8A0` | `fn_80aba8a0` | 164 | high |
| `0x80ABA9F0` | `fn_80aba9f0` | 140 | medium |
| `0x80ABAA80` | `fn_80abaa80` | 56 | high |
| `0x80ABAAD0` | `fn_80abaad0` | 96 | high |
| `0x80ABAC40` | `fn_80abac40` | 288 | medium |
| `0x80ABAD60` | `fn_80abad60` | 208 | high |
| `0x80ABC370` | `fn_80abc370` | 88 | high |
| `0x80ABC490` | `fn_80abc490` | 48 | low |
| `0x80ABC4C0` | `fn_80abc4c0` | 48 | low |
| `0x80ABC610` | `fn_80abc610` | 136 | medium |
| `0x80ABC7E0` | `fn_80abc7e0` | 348 | medium |
| `0x80ABC9A0` | `fn_80abc9a0` | 8 | high |
| `0x80ABCA30` | `fn_80abca30` | 128 | high |
| `0x80ABCAB0` | `fn_80abcab0` | 88 | medium |
| `0x80ABCB10` | `fn_80abcb10` | 232 | medium |
| `0x80ABCE80` | `fn_80abce80` | 88 | high |
| `0x80ABCFA0` | `fn_80abcfa0` | 48 | low |
| `0x80ABCFD0` | `fn_80abcfd0` | 48 | low |
| `0x80ABD000` | `fn_80abd000` | 76 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  4,104 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave101` reports `wave101-enemies.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace build is currently stopped by errors in
  other Wave 101 lanes (`src/game/stage.rs`, `src/runtime/bt.rs`,
  `src/runtime/os.rs`), which do not name this file.
