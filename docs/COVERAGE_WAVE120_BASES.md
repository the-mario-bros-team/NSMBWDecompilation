# Coverage - Wave 120 (`game::bases`)

## How to read this file

This page documents the Wave 120 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`** produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv`.

The `0x8065` bucket holds 185 functions. Before this pass 81 were already
translated, leaving 104. This pass takes 28 of them, spread over
`0x80655DA0..0x8065C210` in five clusters that share field offsets rather than
one long contiguous run.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a helper's
semantics had to be inferred. Comments in the bodies keep `// observed:`,
`// inferred:`, `// partial:` and `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 2,884 | 17 | 11 |

All 28 rows are `translated`; none is `partial`.

## What this pass covers

- **Destructors and constant getters, 10 bodies.** Five
  `0x8015F800`-style destructor twins [`fn_80655da0`], [`fn_8065a6f0`],
  [`fn_8065b160`], [`fn_8065b1c0`], [`fn_8065bb40`]; the `0x802BE7C0` element
  destructor [`fn_806571a0`]; and the four constant getters
  [`fn_80656f50`] and [`fn_8065b590`] (`0x80377A04`), [`fn_8065b220`]
  (`0x80352730`) and [`fn_8065b230`] (`0x803526B0`).
- **`+0x394` interface helpers, 4 bodies.** The `+0x10` notify-and-return-one
  [`fn_80656040`] (byte-identical to the already-translated
  [`fn_80651470`]), the `+0x3E4` sample accumulator [`fn_806561b0`], the
  `+0xD4` descriptor dispatch [`fn_80657390`] and the two-name query against
  the same interface [`fn_806573b0`].
- **`+0x30` chain and name helpers, 4 bodies.** The child-walk twins
  [`fn_8065b320`] / [`fn_8065bc80`] (Ghidra unrolls five probes and then
  recurses; both bodies fold that into one loop) and the colon-trimmed
  class-name compares [`fn_8065b400`] / [`fn_8065bd60`] (`0x802E1C40` +
  `0x802E1AB4`).
- **Transform and sub-object pair, 4 bodies.** The notify-plus-rebuild pair
  [`fn_8065b7d0`] / [`fn_8065b820`], the `+0x3F0` slot wrapper
  [`fn_8065b8c0`] and the `f64` lerp clamp [`fn_8065c210`].
- **Constructors and array elements, 6 bodies.** The `1000`-byte constructor
  [`fn_8065a870`], the `0x400`-byte constructor [`fn_8065b520`], the
  `0x7B0`-byte in-place constructor [`fn_80656f60`], the `0x128`-byte element
  destructor [`fn_80657200`] registered by [`fn_8065be80`], and the
  `0x9CE8`-byte manager constructor [`fn_8065be80`] with its `0x128`-byte
  element constructor [`fn_8065bf50`].

Shared fields: class vtable `+0x60`, interface object `+0x394`, transform block
`+0x7C`, model holder `+0x3F0`, interface byte `+0x5F8`, and the `+0x30` child
pointer of the chain nodes.

Start with [`fn_8065b820`] (the transform writer), [`fn_8065be80`] (the
`0x9CE8`-byte manager constructor) and [`fn_80656f60`] (the `0x7B0`-byte
in-place constructor).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80655DA0` | `fn_80655da0` | 88 | high |
| `0x80656040` | `fn_80656040` | 48 | high |
| `0x806561B0` | `fn_806561b0` | 92 | medium |
| `0x80656F50` | `fn_80656f50` | 12 | high |
| `0x80656F60` | `fn_80656f60` | 264 | medium |
| `0x806571A0` | `fn_806571a0` | 88 | high |
| `0x80657200` | `fn_80657200` | 120 | medium |
| `0x80657390` | `fn_80657390` | 24 | medium |
| `0x806573B0` | `fn_806573b0` | 200 | medium |
| `0x8065A6F0` | `fn_8065a6f0` | 88 | high |
| `0x8065A870` | `fn_8065a870` | 84 | high |
| `0x8065B160` | `fn_8065b160` | 88 | high |
| `0x8065B1C0` | `fn_8065b1c0` | 92 | high |
| `0x8065B220` | `fn_8065b220` | 12 | high |
| `0x8065B230` | `fn_8065b230` | 12 | high |
| `0x8065B320` | `fn_8065b320` | 224 | medium |
| `0x8065B400` | `fn_8065b400` | 136 | medium |
| `0x8065B520` | `fn_8065b520` | 100 | high |
| `0x8065B590` | `fn_8065b590` | 12 | high |
| `0x8065B7D0` | `fn_8065b7d0` | 72 | high |
| `0x8065B820` | `fn_8065b820` | 160 | high |
| `0x8065B8C0` | `fn_8065b8c0` | 48 | high |
| `0x8065BB40` | `fn_8065bb40` | 92 | high |
| `0x8065BC80` | `fn_8065bc80` | 224 | medium |
| `0x8065BD60` | `fn_8065bd60` | 136 | medium |
| `0x8065BE80` | `fn_8065be80` | 208 | medium |
| `0x8065BF50` | `fn_8065bf50` | 88 | high |
| `0x8065C210` | `fn_8065c210` | 72 | medium |

**Totals:** 28 bodies, 2,884 bytes (17 high, 11 medium).

## Deliberate skips

Read and rejected, with the reason:

- **`0x802DCBFC` thunks.** `0x8065B4C0`, `0x8065B4F0`, `0x8065BE20` and
  `0x8065BE50` only forward two unnamed arguments to the unresolved
  indirect-jump stub `0x802DCBFC`. Earlier waves already translated two of that
  family as `partial`; repeating a body whose receiver is a hidden `r12`
  adjustment would add no evidence.
- **Six 12-byte vtable getters.** `0x8065BFB0`, `0x8065BFC0`, `0x8065BFD0`,
  `0x8065BFE0`, `0x8065BFF0` and `0x8065C000` are six identical
  `return 0x80377A04;` bodies. They are genuine translations, but six more
  rows of the same two-instruction stub would pad the lane rather than extend
  coverage; [`fn_80656f50`] and [`fn_8065b590`] already record that exact body
  and constant.
- **Paired-single / GQR reconstruction failures.** `0x80656260`, `0x80656310`,
  `0x8065C100` and `0x8065C260` carry Ghidra's `unaff_GQR0` / `ldexpf`
  reconstruction and unreachable-block warnings. The control flow and call
  sites are visible, but the float register slots cannot be rebuilt without
  guessing which paired-single slot holds which value.
- **Bad-instruction truncation.** `0x80651590` ends in `halt_baddata()` after
  the `+0x3D8` test, and `0x80652310` truncates the same way after a `+0xB0`
  height test. Both would be `partial` bodies with an invented tail, so neither
  was written.
- **Jump-table / event bodies left for a later sweep.** `0x80656D20` (392
  bytes) dispatches two unprototyped virtual calls and a `0x801C14C0` query
  whose roles stay unknown, and `0x8065DA00` / `0x8065DD50` / `0x8065F090` /
  `0x8065F910` sit far outside this pass's window.
- **The rest of the bucket.** The 76 other untranslated addresses in `0x8065`
  were not translated in this pass; they remain for a later sweep.

## Notes on inference

- The `0x9CE8`-byte manager `0x8065BE80` and the `0x400`-byte object
  `0x8065B520` publish different vtables (`0x807E7410` and `0x807E6C88`), so
  they are two classes, not two constructors of one. Neither class name
  survives in the exports and both stay `// unknown:`.
- [`fn_8065b320`] / [`fn_8065bc80`] return the **parent** whose child answered
  non-zero, exactly as the C assigns `param_1` before recursing. The loop fold
  is behaviourally identical; the probe slot `+0x0C` and the `+0x30` link are
  literal.
- [`fn_8065b400`] / [`fn_8065bd60`] preserve the C's `strchr(...) + 1` on a
  null result, which makes the compare start at address `1`; the body keeps
  that instead of "fixing" it.
- `0x802E1C40` and `0x802E1AB4` are called with `':'` and with two string
  pointers, so they are `// inferred:` as `strchr` and `strcmp`.
- [`fn_806561b0`] calls `0x800B2EE0` with the caller's sample and with the
  literal `4`; the routine converts its argument into the time base cached at
  `+0x3E4`, but nothing in the export names it.
- [`fn_80657200`] keeps the compiler's second null test, which is always true
  after the outer guard.
- [`fn_8065c210`] runs the lerp in `f64` because the C widens the parameter and
  the subtraction result before multiplying; the result is rounded back to
  `f32`, so the Rust return type is exact.
- [`fn_8065b820`] passes the single `0x807C8AD0` float three times to
  `0x8016A130`; Ghidra prints `(double)` only because the callee is
  unprototyped.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 2,884 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 81 translated
  addresses were re-read first and none is duplicated here. The file now holds
  3,022 bodies.
- `node tools/check-lane-csv.mjs wave120` reports `OK` and names no problem in
  `wave120-bases.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`; diagnostics raised by other lanes' files are ignored.
