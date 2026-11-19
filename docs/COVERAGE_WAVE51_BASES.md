# Coverage - Wave 51 (bases)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`). Every address below is a static analysis
address, not a runtime address. Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores and call targets are literal, `medium` when a
callee role, a field role or an ABI argument is inferred from surrounding code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 2844 | 24 | 6 |

## What this pass covers

- The `0xF10`-byte number-panel lift actor: its heap constructor
  [`fn_806d5290`] (base constructor `0x80066FC0`, vtable `0x807FB0E8`, three
  NW4R holders, the nine-element `0x5C` array at `+0xBB4` built by
  `0x802DC9A0`) and its destructor tail [`fn_806d5110`].
- The three-byte digit records at `+0x784` (sixteen of them, stride 3):
  clear [`fn_806d53f0`], arm [`fn_806d5400`], validity test [`fn_806d5420`],
  the roll step [`fn_806d5450`], the "first free record" armer with its sound
  `0x2AA` [`fn_806d5c60`], and the two array walks [`fn_806d5bb0`],
  [`fn_806d5c00`].
- The eight-byte ridden-slot ring at `+0x7B4` (128 slots): clear
  [`fn_806d5540`], age [`fn_806d5550`], claim [`fn_806d5570`], expired test
  [`fn_806d5580`], refresh-by-id [`fn_806d5da0`], claim-first-free
  [`fn_806d5e30`], live count [`fn_806d5eb0`] and the two ring walks
  [`fn_806d5d00`], [`fn_806d5d50`].
- The `+0x3F4` model path: the transform writer [`fn_806d58b0`] (the same
  `0x80064AF0`/`0x801C0BD0`/`0x8016EEF0` sequence as the `kazan_rock` actor),
  the animation binder [`fn_806d5810`], the two virtual-slot drivers
  [`fn_806d6610`], [`fn_806d6e00`] and the per-frame dispatcher
  [`fn_806d6d50`].
- The nine `0x5C` motion records at `+0xBB4`: their initialiser
  [`fn_806d5f20`], reset [`fn_806d61c0`], virtual teardown [`fn_806d6210`],
  release [`fn_806d6270`] and child-list query [`fn_806d6890`].
- Two bounds/query helpers: the X/Y visibility test [`fn_806d64d0`] and the
  word-array id search [`fn_806d68e0`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806D5110` | `fn_806d5110` | 88 | high |
| `0x806D5290` | `fn_806d5290` | 276 | high |
| `0x806D53F0` | `fn_806d53f0` | 16 | high |
| `0x806D5400` | `fn_806d5400` | 20 | high |
| `0x806D5420` | `fn_806d5420` | 36 | high |
| `0x806D5450` | `fn_806d5450` | 236 | medium |
| `0x806D5540` | `fn_806d5540` | 16 | high |
| `0x806D5550` | `fn_806d5550` | 24 | high |
| `0x806D5570` | `fn_806d5570` | 16 | high |
| `0x806D5580` | `fn_806d5580` | 20 | high |
| `0x806D5810` | `fn_806d5810` | 152 | medium |
| `0x806D58B0` | `fn_806d58b0` | 180 | high |
| `0x806D5BB0` | `fn_806d5bb0` | 72 | high |
| `0x806D5C00` | `fn_806d5c00` | 88 | high |
| `0x806D5C60` | `fn_806d5c60` | 160 | high |
| `0x806D5D00` | `fn_806d5d00` | 72 | high |
| `0x806D5D50` | `fn_806d5d50` | 72 | high |
| `0x806D5DA0` | `fn_806d5da0` | 132 | high |
| `0x806D5E30` | `fn_806d5e30` | 124 | high |
| `0x806D5EB0` | `fn_806d5eb0` | 100 | high |
| `0x806D5F20` | `fn_806d5f20` | 136 | high |
| `0x806D61C0` | `fn_806d61c0` | 72 | high |
| `0x806D6210` | `fn_806d6210` | 84 | high |
| `0x806D6270` | `fn_806d6270` | 72 | high |
| `0x806D64D0` | `fn_806d64d0` | 88 | high |
| `0x806D6610` | `fn_806d6610` | 100 | medium |
| `0x806D6890` | `fn_806d6890` | 68 | high |
| `0x806D68E0` | `fn_806d68e0` | 100 | medium |
| `0x806D6D50` | `fn_806d6d50` | 176 | medium |
| `0x806D6E00` | `fn_806d6e00` | 48 | medium |

## Deliberate skips

- `0x806D5970` (276 bytes), `0x806D5A90` (276 bytes), `0x806D5FB0` (516 bytes)
  and `0x806D62C0` (528 bytes) decompile with an unrecovered `GQR0` register and
  `ldexpf` bookkeeping around their paired-single moves. The register pair that
  feeds each body is not visible in the export, so their float semantics cannot
  be settled and no body was invented. They are still called by fixed address
  from [`fn_806d6d50`].
- `0x806D6530` (104 bytes) and `0x806D65A0` (112 bytes) have the same `GQR3`
  problem on a signed halfword counter; the increment/pack sequence is readable
  but the float conversion is not. Also called by address from
  [`fn_806d6d50`].
- `0x806D5230` and `0x806D5260` are 48-byte adjustor thunks whose whole body is
  `0x802DCBFC(a2, a2, a1)`. They carry no recoverable object role, so they were
  skipped rather than padded into rows.
- The rest of the `0x806D` bucket is outside this lane: 190 entries were
  untranslated before this pass and 160 remain after it. The chosen window is
  otherwise contiguous, apart from the six skipped bodies above.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave51` reports `OK` for
  `wave51-bases.csv`.
