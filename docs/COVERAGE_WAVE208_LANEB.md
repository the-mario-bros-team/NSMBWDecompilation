# Coverage - Wave 208 lane B (`ui::hud`, main.dol)

> File name note: the shared instructions name this document
> `docs/COVERAGE_WAVE205_LANE<X>.md`, but that path is owned by the wave-205
> lane B report. This lane follows the `COVERAGE_WAVE207_LANEB.md` precedent and
> lands as `docs/COVERAGE_WAVE208_LANEB.md` instead of overwriting unrelated
> work.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 528 | 7 | 3 |

Addresses `0x8003_C6F0`-`0x8003_CA20`: the ten bodies between the translated
`0x8003C690` release wrapper and the `0x8003CCC0`/`0x8003CE20` bodies that
follow. The run is entirely accessors, one chain walk and three string/thunk
helpers; it contains no scene or state-machine logic.

## What landed

- **Two singleton accessors.** `0x8003C6F0` returns `0x80357BF4` and
  `0x8003C700` returns `0x80352DB4`; both are the literal
  `lis r3,0x8035` / `addi r3,r3,<disp>` / `blr` shape. `0x8003C6F0` is
  byte-identical to the already-translated `0x800268B0`, and `0x8003C700`
  materialises the same descriptor that `0x80031B70`'s cluster hands to virtual
  slot `+0x14`.
- **Child-chain walk.** `0x8003C7F0` unrolls five `child = *(node + 0x30)` /
  virtual `+0x0C` calls and then tail-recurses on the sixth child; a non-zero
  answer returns the parent node. The body is instruction-for-instruction the
  same as `0x80023A60` and `0x80030FE0`, which are already in this module.
- **Name-tail compare.** `0x8003C8D0` splits both the incoming name and the
  object's own name (virtual slot `+0x20`) at `':'` (`0x3A`) through
  `0x802E1C40` and compares the tails with `0x802E1AB4`. It is byte-identical
  to `0x8001D930`.
- **Two forwarding thunks.** `0x8003C990` (`r12 = r5 + 0x18`) and `0x8003C9C0`
  (`r12 = r5 + 0x24`) forward `(b, b, a)` to the unresolved indirect-jump stub
  `0x802DCBFC`; they are the `+0x18`/`+0x24` copies of `0x8001D9F0`/
  `0x8001DA20`.
- **Four `0x80377A04` accessors.** `0x8003C9F0`, `0x8003CA00`, `0x8003CA10`
  and `0x8003CA20` all return `0x80377A04` (fifth to eighth copies of
  `0x8001CBB0`).

## Corrections and recoveries

- **All ten `functions.tsv` sizes are exact.** Each body was walked to its
  terminal `bclr`: `0x0C`, `0x0C`, `0xE0`, `0x88`, `0x30`, `0x30`, `0x0C`,
  `0x0C`, `0x0C`, `0x0C`. Six four-byte zero-padding slots follow the 12-byte
  accessors (`0x8003C6FC`, `0x8003C70C`, `0x8003C9FC`, `0x8003CA0C`,
  `0x8003CA1C`, `0x8003CA2C`); the padding is not code and is not reproduced.
  No prologue artefact appears in this run.
- **`0x8003C7F0` has no trailing accessor call.** The neighbouring
  `0x8003C710` (same `0xE0` size, not in this lane) ends with
  `mr r3,r31` / `bl 0x8015F930`; `0x8003C7F0`'s two return arms
  (`0x8003C8B0`, `0x8003C8B8`) both fall straight into the epilogue at
  `0x8003C8BC`. The translation therefore returns the node itself, not
  `*(node + 8)`.
- **The `r12` sub-object pointers in the two thunks are dead.** Both bodies
  compute `addi r12,r5,0x18` / `addi r12,r5,0x24` and never read `r12`; the
  `bl 0x802DCBFC` takes `r3 = b`, `r4 = b` (Ghidra's `param_2,param_2,
  param_1`) and `r5 = a`. The reconstruction keeps the three-operand call and
  documents the unused offset rather than inventing a fourth argument.
- **`0x8003C8D0` advances the object-side pointer unconditionally.** The
  original does `bl 0x802E1C40` and then `addi r3,r3,1` without a null test on
  the object side, while the name side (`r31 = r3 + 1`) is guarded by
  `cmpi`/`bc 12,2`. The body keeps that asymmetry, matching `0x8001D930`.
- **No new constants were needed.** Every literal this lane touches already
  exists at module scope (`DAT_80357BF4`, `DAT_80352DB4`, `DAT_80377A04`,
  `FN_802DCBFC`, `FN_802E1C40`, `FN_802E1AB4`) and the helpers
  `vtable_method`/`p32` are reused, so the block adds no `_B`-suffixed
  constant and cannot collide with another lane.
- **The `+0x0C` predicate and the two singletons stay unnamed.** No body in
  this run proves what virtual slot `+0x0C` tests, nor what lives at
  `0x80357BF4`/`0x80352DB4`; both are marked `unknown`/`inferred:` in the doc
  blocks rather than guessed.

## Verification

- Scratch build: `src/`, `Cargo.toml` and `Cargo.lock` copied to
  `%TEMP%\opencode\w208b`, `work/wave208/laneB_block.rs` appended to that
  copy's `src/ui/hud.rs`, `cargo check --lib` exits `0`; no warning mentions
  the appended block (hud.rs lines > 29825).
- Ledger: `ledger/agents/wave208-laneB.csv` (10 rows, all `translated`).
- This document: `docs/COVERAGE_WAVE208_LANEB.md`.
