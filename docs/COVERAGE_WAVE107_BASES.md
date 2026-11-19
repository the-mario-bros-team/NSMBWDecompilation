# Coverage - Wave 107 (`game::bases`, `0x8079` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address on this page is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (globbed by address
prefix, never assumed). "Confidence" is `high` when guards, stores, constants
and call arguments are literal, and `medium` when a callee identity, a field
role or a dropped argument had to be inferred; those bodies carry
`// observed:` / `// inferred:` notes. No body in this pass is `low`.

The bucket holds 184 functions; 65 were already in `ledger/functions.csv` when
this pass began, leaving 119. This pass takes 30 of them, in three clusters
inside `0x807900B0..0x8079A870`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4,276 | 11 | 19 |

## What this pass covers

- **The scene effect tables (`0x807900B0..0x807901E0`, 3 bodies).** The bind
  pass [`fn_807900b0`] walks the `0x44`-byte record selected by `+0x214` and
  fills the `+0x21C` handles, [`fn_80790160`] releases all `0x11` of them, and
  [`fn_807901e0`] fills the `0x15`-entry `+0x260` table. All three share the
  marker compare `0x802E1AB4(name, 0x8081AB94)`.
- **The shared animation pair (`0x80791CE0..0x80794590`, 6 bodies).** The
  `0x29E` actor probe [`fn_80791ce0`], the `skl_root` effect arming
  [`fn_807923f0`], the clip constant [`fn_807939d0`], the `+0x70C`/`+0x784`
  rebuild [`fn_807939e0`], the finished-clip copy into `+0x74C`
  [`fn_80793a60`], and the `+0x32C` clear [`fn_80794590`].
- **The course-tree query family (`0x807995A0..0x8079A870`, 21 bodies).** The
  node probes `0x80775100`/`0x80775150` ([`fn_807995a0`], [`fn_807995c0`]), the
  `0x14`-step two-hop walk [`fn_807995e0`], the classification gate
  [`fn_80799750`], the `Fk`/`Ka` prefix test [`fn_80799860`], the model/child
  reset [`fn_807999d0`], the sibling scan [`fn_80799a50`], the `0x278` spawn
  gate [`fn_80799c90`], the three flag tests ([`fn_80799d10`],
  [`fn_80799eb0`], [`fn_8079a750`]), the `F`/`W` cursor update
  [`fn_80799f10`], the position getters [`fn_8079a310`]/[`fn_8079a330`], the
  name compare [`fn_8079a350`], the six-way direction classifier
  [`fn_8079a3e0`], the five-bit and six-slot tests
  ([`fn_8079a4b0`], [`fn_8079a5d0`]), the digit-code test [`fn_8079a690`], the
  combined test [`fn_8079a7d0`] and the map rebuild [`fn_8079a870`].

Start with [`fn_8079a310`] for the `+0x29C` position record and
[`fn_80799860`] for the name-prefix convention the rest of the family uses.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807900B0` | `fn_807900b0` | 164 | medium |
| `0x80790160` | `fn_80790160` | 116 | high |
| `0x807901E0` | `fn_807901e0` | 148 | medium |
| `0x80791CE0` | `fn_80791ce0` | 60 | high |
| `0x807923F0` | `fn_807923f0` | 104 | medium |
| `0x807939D0` | `fn_807939d0` | 16 | medium |
| `0x807939E0` | `fn_807939e0` | 128 | medium |
| `0x80793A60` | `fn_80793a60` | 184 | medium |
| `0x80794590` | `fn_80794590` | 12 | high |
| `0x807995A0` | `fn_807995a0` | 20 | medium |
| `0x807995C0` | `fn_807995c0` | 32 | high |
| `0x807995E0` | `fn_807995e0` | 360 | medium |
| `0x80799750` | `fn_80799750` | 264 | medium |
| `0x80799860` | `fn_80799860` | 248 | medium |
| `0x807999D0` | `fn_807999d0` | 128 | medium |
| `0x80799A50` | `fn_80799a50` | 172 | medium |
| `0x80799C90` | `fn_80799c90` | 128 | medium |
| `0x80799D10` | `fn_80799d10` | 84 | high |
| `0x80799EB0` | `fn_80799eb0` | 84 | high |
| `0x80799F10` | `fn_80799f10` | 292 | medium |
| `0x8079A310` | `fn_8079a310` | 32 | high |
| `0x8079A330` | `fn_8079a330` | 32 | high |
| `0x8079A350` | `fn_8079a350` | 144 | medium |
| `0x8079A3E0` | `fn_8079a3e0` | 204 | medium |
| `0x8079A4B0` | `fn_8079a4b0` | 276 | high |
| `0x8079A5D0` | `fn_8079a5d0` | 188 | high |
| `0x8079A690` | `fn_8079a690` | 188 | medium |
| `0x8079A750` | `fn_8079a750` | 124 | high |
| `0x8079A7D0` | `fn_8079a7d0` | 152 | medium |
| `0x8079A870` | `fn_8079a870` | 192 | medium |

**Totals:** 30 translated, 0 partial, 0 stub (11 high, 19 medium, 4,276 bytes).

## Deliberate skips

Read and rejected, with the reason:

- `0x80790280` (268 bytes) and `0x80799D70` (304 bytes) open with
  `unaff_GQR0` paired-single brackets and `ldexpf` calls. The incoming GQR
  state cannot be reconstructed from the export, so the argument scaling is
  lost; the same reason is recorded for `0x80754430`/`0x80754620` in
  `docs/COVERAGE_WAVE106_BASES.md`.
- `0x80799B00` (288 bytes) recovers its two arguments through the
  `0x802DCD70` save/restore thunk as one 64-bit value, and then feeds the low
  and high halves into different queries. The register pair cannot be ordered
  honestly from the export, so the body is left untranslated.
- `0x8079B110` (16 bytes) and `0x8079B1C0` (12 bytes) are `halt_baddata()`
  bodies; Ghidra recovered no instruction stream for them.
- `0x80793BC0` (92 bytes) decompiles as a search loop whose result is
  discarded (`FUN_80793BC0` returns `void` although every caller compares the
  result), so the callee's return register is not recoverable.
- `0x80793E50` (64 bytes) calls `0x802B9280` with no arguments at all even
  though the enclosing body's two parameters are otherwise unused; the real
  argument list is lost.

The remaining untranslated functions of the bucket were not opened in this pass
(budget, not doubt); the pass deliberately stops at `0x8079A870`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 4,276 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 65 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,654 bodies.
- `node tools/check-lane-csv.mjs wave107` reports
  `wave107-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
