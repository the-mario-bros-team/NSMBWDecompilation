# Coverage - Wave 219, lane B, `src/resource/archive.rs`

## How to read this file

- **Scope** - the ten bodies added to `src/resource/archive.rs` by wave 219,
  lane B, module `resource::archive`: the `0x802C0110..0x802C4770` band.
  Earlier work in this file is documented in the lane files of the other
  waves; this file does not repeat it.
- **Addresses are analysis addresses.** They are absolute `main.dol`
  addresses, as printed by Ghidra and `work/dol/main.elf`.
- **Evidence** is the Ghidra C under `work/ghidra/main-gb/export/`; the exact
  file is named in every table row. Sizes come from
  `work/ghidra/main-gb/export/functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented.
- **Confidence** rates the translation, not the guessed role. The six GX
  wrappers, the deleter and the UV wrap are literal (`high`); the gradient
  fill and the two-pass draw discard compiler artefacts and read one indirect
  callee slot, so they are `medium`.
- **Names** stay `fn_802cxxxx`. The shipped image has no symbol table, so no
  Nintendo symbol is invented. New constants carry the lane `_B` suffix.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 10 |
| text bytes represented | 3096 |
| status `translated` | 10 |
| status `partial` | 0 |
| status `deferred` | 0 |
| confidence `high` | 8 |
| confidence `medium` | 2 |

## Size verification

Every `functions.tsv` size was checked against the prologue and the closing
`blr` in `work/dol/main.elf`. All ten values are exact; there is no
prologue-artefact (16/20-byte) row in this lane.

| address | tsv | real extent | note |
|---|---:|---:|---|
| `0x802C0110` | 52 | 52 | ends at the `blr` at `0x802C0140` |
| `0x802C02C0` | 388 | 388 | ends at the `blr` at `0x802C0440` |
| `0x802C0BA0` | 128 | 128 | ends at the `blr` at `0x802C0C1C` |
| `0x802C0C70` | 448 | 448 | ends at the `blr` at `0x802C0E2C` |
| `0x802C20E0` | 8 | 8 | two instructions, tail `b` (no `blr`) |
| `0x802C2250` | 100 | 100 | ends at the `blr` at `0x802C22B0` |
| `0x802C30C0` | 88 | 88 | ends at the `blr` at `0x802C3114` |
| `0x802C35B0` | 64 | 64 | ends at the `blr` at `0x802C35EC` |
| `0x802C38E0` | 156 | 156 | leaf, no prologue, ends at the `blr` at `0x802C3978` |
| `0x802C4700` | 1664 | 1664 | ends at the `blr` at `0x802C4D7C` |

## Function table

| address | bytes | confidence | what the body does |
|---|---:|---|---|
| `0x802C0110` | 52 | high | resets the descriptor block, then stores the caller word |
| `0x802C02C0` | 388 | high | GX viewport/scissor/blend/texture setup from a sprite descriptor |
| `0x802C0BA0` | 128 | high | two GX texture-attribute calls (`0x801C6D90`, `0x801C6FF0`) |
| `0x802C0C70` | 448 | medium | fills a radial-gradient render target, one texel at a time |
| `0x802C20E0` | 8 | high | tail call to `0x801C8110(0)` |
| `0x802C2250` | 100 | high | mode-2 GX state plus two record setters |
| `0x802C30C0` | 88 | high | GX depth function and copy-scale state |
| `0x802C35B0` | 64 | high | guarded default delete, returns its argument |
| `0x802C38E0` | 156 | high | advances and wraps one scrolled UV pair |
| `0x802C4700` | 1664 | medium | two-pass screen quad draw and gradient-strip rebuild |

## Notes on the two `medium` bodies

- `0x802C0C70` - the Ghidra export opens with five `GQR0`/`CONCAT` blocks.
  The disassembly shows those are only the `psq_st` saves of `f27..f31` in the
  prologue (and their `psq_l` restores in the epilogue), so they carry no
  arithmetic and are dropped. Everything else is literal: the rounded centre
  `(n + (n & 1)) >> 1`, the `sqrt(r2 - d2)` emitted by the compiler as
  `(r2 - d2) * invsqrt(r2 - d2)`, and the byte rotation from colour byte 3 to
  byte 0 before `0x802C1280`.
- `0x802C4700` - same `GQR0` artefact. The two indirect calls read their
  target from an object reached through the draw record (mode 0:
  `[frame + 0x88] + 0x0C`; mode 0 tail: `[object + 0x80] + 0x28`), which the
  export cannot name. The 0x1D8-byte scratch frame stands in for the shipped
  stack frame so the offsets in the export remain valid verbatim.

## Deferrals

None.
