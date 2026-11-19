# Coverage - Wave 126 (`game::enemies`, the `0x80A40000` bucket)

## How to read this file

This page records the Wave 126 `game::enemies` lane: 29 functions of the
`0x80A40000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 107 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave126-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 3,880 | 19 | 6 |

The bucket holds 189 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 52 were untranslated when
this pass started. This pass claims 29 of them and leaves 23. The four `low`
rows not counted in the two right-hand columns are the `0x802DCBFC` forwarders.

## What this pass covers

Three runs chosen for shared field layouts and one shared model block.

- **The re-arm actor tail (`0x80A407D0..0x80A40F30`, 7 bodies).** The
  `+0xA54`-state actor whose earlier frame steps were translated in wave 77 and
  whose local helpers were translated in wave 117. This pass adds the two
  countdown variants (`+0xA8C`/`+0xA8E` with the `0x80027B00` request and
  `+0xAA8` with sound request `0x238`), the two speed-table initialisers
  (`0x80B2C638`/`0x80B2C658` and `0x80B2C690`, both bent by `0x80B2C5FC` and
  closed by the `0x80B2C684` x scale and `+0xA74`/`+0xA88`/`+0x74A` tail), the
  `+0x554`-flag class-list scan that re-parents the actor to the scene object
  `0x80429DD0`, and the `+0xA54` handle setter consumed by
  [`fn_80a404a0`].
- **The descriptor furniture (`0x80A41E20..0x80A42150`,
  `0x80A43150..0x80A43660`, 10 bodies).** Two record teardowns (the
  `0x8015F800`/`0x802B9280` pair), two base-class constant producers
  (`0x80357D34` and `0x80358104`), four `0x802DCBFC` argument-swapping
  forwarders, the `+0x394` holder release that returns `1`, and the
  class-`0x261` registry walk that matches `+0x912` against each entry's
  `+0x4F4` byte. The `0x80358104` word is the same base-class record returned
  by the coin block's producer, which ties the skeleton to that base class
  without inventing a symbol.
- **The `daEnCrow_c` model block (`0x80A4A8D0..0x80A4BD70`, 12 bodies).** The
  binder loads `g3d/crow.brres` (`0x80B42E10`) into `+0x540`/`+0x544` and the
  body clip into `+0x584`. The block continues with the 52-byte `+0x544`
  sample merged into `+0x5E0`, five clip entries (`attack1` `0x80B42E60`,
  `attack2` `0x80B42E58`, body `0x80B42E28`), the dive-level clamp `0..7` with
  sound request `0x1EF`, the `0x80B2CA38` facing-gated swoop window that
  enters `0x80B71070`, and the two `+0x100..+0x104` steering steps that convert
  the signed side table `0x8042C1A0` through the compiler's `2^52 + 2^31`
  integer-to-double idiom. The state strings
  `daEnCrow_c::StateID_Appear..StateID_DieFall` at
  `0x80B43227..0x80B43304` sit in the same data block as the crow model
  strings, so the class name is not guessed.

### Why six rows are `medium`

- **An untranslated callee.** `0x80A4B0B0` (animation bind/blend) and
  `0x80A4B140` (lateral query) are themselves GQR-damaged in the export, so
  the crow bodies call them by analysis address and mark the role
  `// unknown:`. See [`fn_80a4b2b0`] and the five clip entries.
- **A result code is inferred.** In [`fn_80a40d90`] the class-list walk
  produces `2`/`1`/`0`; only the two consumers establish what those mean.
- **A slot argument is implicit.** In [`fn_80a435c0`] and
  [`fn_80a4b2b0`] the `+0x10`/`+0x1C` and `+0x6C` virtual slots print no
  argument; the receiver is inferred from the live `r3`.
- **A scratch record's layout is not printed.** [`fn_80a4ac00`] hands a
  52-byte stack record to `0x8016F130` without exposing its fields.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A407D0` | `fn_80a407d0` | 164 | high |
| `0x80A40880` | `fn_80a40880` | 320 | high |
| `0x80A409D0` | `fn_80a409d0` | 140 | high |
| `0x80A40A60` | `fn_80a40a60` | 304 | high |
| `0x80A40BA0` | `fn_80a40ba0` | 148 | medium |
| `0x80A40D90` | `fn_80a40d90` | 364 | medium |
| `0x80A40F30` | `fn_80a40f30` | 8 | high |
| `0x80A41E20` | `fn_80a41e20` | 92 | high |
| `0x80A41E80` | `fn_80a41e80` | 12 | high |
| `0x80A41E90` | `fn_80a41e90` | 12 | high |
| `0x80A42120` | `fn_80a42120` | 48 | low |
| `0x80A42150` | `fn_80a42150` | 48 | low |
| `0x80A43150` | `fn_80a43150` | 88 | high |
| `0x80A43270` | `fn_80a43270` | 48 | low |
| `0x80A432A0` | `fn_80a432a0` | 48 | low |
| `0x80A435C0` | `fn_80a435c0` | 48 | medium |
| `0x80A43660` | `fn_80a43660` | 92 | high |
| `0x80A4A8D0` | `fn_80a4a8d0` | 228 | high |
| `0x80A4AC00` | `fn_80a4ac00` | 68 | medium |
| `0x80A4B010` | `fn_80a4b010` | 160 | high |
| `0x80A4B230` | `fn_80a4b230` | 108 | high |
| `0x80A4B2B0` | `fn_80a4b2b0` | 252 | medium |
| `0x80A4B3B0` | `fn_80a4b3b0` | 96 | high |
| `0x80A4B650` | `fn_80a4b650` | 216 | high |
| `0x80A4B9A0` | `fn_80a4b9a0` | 112 | high |
| `0x80A4BA20` | `fn_80a4ba20` | 316 | medium |
| `0x80A4BB60` | `fn_80a4bb60` | 88 | high |
| `0x80A4BD00` | `fn_80a4bd00` | 88 | high |
| `0x80A4BD70` | `fn_80a4bd70` | 164 | high |

## Deliberate skips

- **GQR-damaged callees of this run.** `0x80A4B0B0` (136), `0x80A4B140`
  (236), `0x80A4B420` (548), `0x80A4B740` (604) and `0x80A4BBD0` (304) carry
  `unaff_GQR0` conversion artefacts and `ldexpf` calls that are not in the
  original code, so their bodies cannot be typed honestly. They are called by
  address from the new bodies; none is claimed here. This is the same
  limitation recorded in waves 77 and 109.
- **Truncated exports.** `0x80A40FA0` (28), `0x80A429C0` (476), `0x80A436C0`
  (328), `0x80A43810` (252), `0x80A443A0` (24), `0x80A46200` (300) and
  `0x80A48940` (48) end in `halt_baddata` or lose their control flow; no body
  was written. `0x80A40FA0` is the contact-effect callee already documented as
  truncated in wave 117.
- **GQR-damaged frame steps of the re-arm actor.** `0x80A40290` (232) and
  `0x80A40CA0` (208) build their angle writes from `unaff_GQR0`/`unaff_GQR3`
  quantiser fiction, so they remain untranslated exactly as recorded in
  wave 77.
- **Clean bodies left for a later pass.** `0x80A45DB0` (500), `0x80A46BF0`
  (172), `0x80A441E0` (116), `0x80A442D0` (208), `0x80A446B0` (88),
  `0x80A43DC0` (88), `0x80A43EE0` (48), `0x80A43F10` (48) and
  `0x80A4EC90` (12) are readable but were not needed once the 25-30 body
  target was met. Nothing was written for them, so no ledger row claims them.
- **No address already in `ledger/functions.csv` was touched**, and no body
  from another lane's file was edited.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  3,880 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 29 was in
  `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` emits no diagnostic naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave126` reports
  `wave126-enemies.csv: 29 rows`, 29 unique addresses and `OK`.
- The module header was updated: cluster item 107, the cluster count (one
  hundred and six to one hundred and seven), the function totals (2,866 to
  2,895 public translations, 2,795 to 2,824 `fn_<addr>` bodies), the "start
  here" list and the records list.
