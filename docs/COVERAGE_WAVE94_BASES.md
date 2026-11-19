# Coverage - Wave 94 (`game::bases`, `0x806D` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image.
Cross-module pointers (`0x8042xxxx`, `0x807Fxxxx`) are `main.dol` data addresses
imported by the module and keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants and
call arguments are literal, and `medium` when the role of a field, a global, a
callee or a dropped argument had to be inferred (those bodies carry
`// observed:` / `// inferred:` / `// unknown:` notes). No body in this pass is
`low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 29 | 5,608 | 13 | 16 |

## What this pass covers

The `0x806d` bucket holds 219 functions; 88 were already in
`ledger/functions.csv` when this pass began, leaving 131. This pass takes 29 of
them from three adjacent copies of the rideable-lift island,
`0x806D9810..0x806DA9B0`, `0x806DB6E0..0x806DBBD0` and
`0x806DC5B0..0x806DCE40`.

- **First copy, `0x806D9810..0x806DA9B0` (14 bodies).** A lift actor with three
  rig records at `+0x53C`/`+0x548`/`+0x554`, three matrix sources at
  `+0x560`/`+0x56C`/`+0x578`, per-player flags at `+0x584..+0x589`, rider
  counters at `+0x590` and five `0x40`-stride NW4R holders (`+0x3F0`, `+0x430`,
  `+0x470`, `+0x4B0`, `+0x4F0`). It contributes the ramp-length rebuild
  [`fn_806d9810`], the per-player model placement [`fn_806d9870`] with its
  two-slot wrapper [`fn_806d9990`], the two-pass part placement
  [`fn_806d99d0`], the three-rig rebuild [`fn_806d9bf0`], the contact-volume
  requests [`fn_806d9e80`], the rider drop/tick pair
  [`fn_806d9f70`]/[`fn_806da050`], the load classifier [`fn_806da120`], the four
  state handlers [`fn_806da220`]/[`fn_806da2e0`]/[`fn_806da410`]/[`fn_806da890`]
  that switch between `0x8082D9E8`, `0x8082DA28`, `0x8082DA68`, `0x8082DAA8` and
  `0x8082DB28`, and the teardown [`fn_806da9b0`].
- **Second copy, `0x806DB6E0..0x806DBBD0` (8 bodies).** The same family with the
  animation frames `+0x4C8`/`+0x4DC` instead of the rig records. It contributes
  the rider recount and board resize [`fn_806db6e0`] (child list at `+0x500`,
  per-player flags, `0x55C`/`0x560`/`0x564`/`0x568` history), the four flag
  clears [`fn_806db9d0`], the player frame step [`fn_806db9f0`], the presence
  query [`fn_806dba30`], the detach body [`fn_806dbae0`], the motion
  stop/restart pair [`fn_806dbb20`]/[`fn_806dbb80`] and the heading copy
  [`fn_806dbbd0`].
- **Third copy, `0x806DC5B0..0x806DCE40` (7 bodies).** The `test_lift` variant
  that adds the `+0x4A4`/`+0x4A5` steering pair and the `+0x440` contact list.
  It contributes the `g3d/test_lift.brres` loader [`fn_806dc5b0`], the
  contact-scan steering vote [`fn_806dc660`], the constant restore
  [`fn_806dc7e0`], the steering clamp [`fn_806dc850`], the model matrix rebuild
  [`fn_806dc9e0`], the release wrapper [`fn_806dccc0`] and the `0x728`-byte
  constructor [`fn_806dce40`].

Start with [`fn_806da120`] for the rider-count arithmetic and
[`fn_806db6e0`] for the board-resize state machine; [`fn_806dc5b0`] is the
clearest model loader of the pass.

## Address range

`0x806D9810..0x806DA9B0` (first copy, 14 bodies),
`0x806DB6E0..0x806DBBD0` (second copy, 8 bodies) and
`0x806DC5B0..0x806DCE40` (third copy, 7 bodies). After this pass the `0x806d`
bucket still has 102 untranslated functions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806D9810` | `fn_806d9810` | 96 | high |
| `0x806D9870` | `fn_806d9870` | 276 | medium |
| `0x806D9990` | `fn_806d9990` | 60 | high |
| `0x806D99D0` | `fn_806d99d0` | 516 | medium |
| `0x806D9BF0` | `fn_806d9bf0` | 648 | medium |
| `0x806D9E80` | `fn_806d9e80` | 240 | medium |
| `0x806D9F70` | `fn_806d9f70` | 76 | medium |
| `0x806DA050` | `fn_806da050` | 76 | medium |
| `0x806DA120` | `fn_806da120` | 204 | medium |
| `0x806DA220` | `fn_806da220` | 128 | high |
| `0x806DA2E0` | `fn_806da2e0` | 268 | medium |
| `0x806DA410` | `fn_806da410` | 236 | medium |
| `0x806DA890` | `fn_806da890` | 172 | medium |
| `0x806DA9B0` | `fn_806da9b0` | 200 | high |
| `0x806DB6E0` | `fn_806db6e0` | 752 | medium |
| `0x806DB9D0` | `fn_806db9d0` | 24 | high |
| `0x806DB9F0` | `fn_806db9f0` | 60 | high |
| `0x806DBA30` | `fn_806dba30` | 48 | high |
| `0x806DBAE0` | `fn_806dbae0` | 64 | medium |
| `0x806DBB20` | `fn_806dbb20` | 56 | high |
| `0x806DBB80` | `fn_806dbb80` | 56 | high |
| `0x806DBBD0` | `fn_806dbbd0` | 72 | medium |
| `0x806DC5B0` | `fn_806dc5b0` | 164 | high |
| `0x806DC660` | `fn_806dc660` | 228 | medium |
| `0x806DC7E0` | `fn_806dc7e0` | 88 | medium |
| `0x806DC850` | `fn_806dc850` | 372 | medium |
| `0x806DC9E0` | `fn_806dc9e0` | 136 | high |
| `0x806DCCC0` | `fn_806dccc0` | 88 | high |
| `0x806DCE40` | `fn_806dce40` | 204 | high |

## Deliberate skips

The bodies below were read and rejected rather than guessed; the rest of the
bucket was simply not reached.

- `0x806D9FC0`, `0x806DA520` and `0x806DB340` are paired-single
  reconstructions: each export contains `unaff_GQR0`, two `ldexpf` lane
  extractions and at least one `WARNING: Removing unreachable block`, so the
  register class and the float lanes cannot be proven. The three are nevertheless
  needed by the handlers above, so `0x806D9FC0` is invoked by address
  (`abi_call1!(0x806D_9FC0, this)`) with an explicit `// unknown:` note and
  `0x806DA520`/`0x806DB340` are not called at all.
- `0x806D5230`, `0x806D5260`, `0x806DBE70`, `0x806DBEA0`, `0x806DCDE0` and
  `0x806DCE10` are one-line tails that call `0x802DCBFC`, the unresolved
  indirect-jump stub documented in the wave-3 note. They carry no
  reconstructable behaviour.
- `0x806DB2C0`, `0x806DB300`, `0x806DBAE0`'s neighbour `0x806DBA30` aside: the
  two release wrappers `0x806DB2C0`/`0x806DB300` print `func_0x802B9280()` with
  no arguments and no other evidence of the callee prototype, so the placement
  of the object pointer could not be proven; they were left for a later pass.
- `0x806DC660`'s two thresholds (`0x807CB950`, `0x807CB980`) are carried as
  literal reads with `// inferred:` near/far roles; no function was skipped for
  that reason.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (29 rows, 5,608 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 88 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave94` reports `wave94-bases.csv: 29 rows` and
  `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/bases.rs` (the crate still fails on other lanes' files, which are
  outside this lane).
