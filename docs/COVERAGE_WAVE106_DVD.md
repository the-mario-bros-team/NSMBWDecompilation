# Coverage - Wave 106 (runtime::dvd, the nine addresses Wave 105 skipped)

## How to read this file

The `runtime::dvd` lane owns the whole `0x801C0000` 64 KiB page of `main.dol`,
which is much wider than the disc band. Every body below is SDK GX code that
happens to share the page; `src/runtime/dvd.rs` is the only writable file
assigned to the bucket, so the bodies live there.

`high` means every store, mask and branch in the body is literal and its role is
proven; `medium` means the instruction sequence is literal but at least one
field role, operand binding or callee identity is inferred; `low` means only the
store sequence is certain.

This pass exists because Wave 105 left nine addresses unclaimed on the theory
that their Ghidra exports did not describe the code at the address. Re-reading
all nine against the raw instruction stream in `work/dol/main.elf` shows the
theory was only half right: eight exports are **truncated** or inflated by the
paired-single save/restore prologue, not wrong, and those eight are translated
here. The ninth (`0x801C90A0`) is still unclaimed, for a concrete reason given
under "Deliberate skips".

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 8 | 2884 | 6 | 2 | 0 |

## What this pass covers

The bucket held nine untranslated addresses when this pass started. Eight are
translated, in ascending address order; none is disc code.

- **Matrix concat array, `0x801C0610` (392 bytes).** The array form of the
  already-translated [`fn_801c0540`]: a fixed left 3x4 matrix is multiplied into
  `count` matrices at a `0x30` stride. The body reuses the same three-term
  products, the same backward column order and the same
  `0x804295A0`/`0x804295A4` fourth-column constants; the caller at `0x8024FEF0`
  proves the `(m, src[], dst[], count)` argument order by routing `count < 2` to
  `0x801C0540`.
- **The two angle wrappers, `0x801C0970`/`0x801C0B50` (124 bytes each).** Both
  save `r3`/`r4`, call `0x802E84C4` and `0x802E7FBC`, narrow the results with
  `frsp`, and forward the pair to a matrix builder. The two entries are the
  `double` sine and cosine: their small-argument arms call the polynomial
  kernels `0x802E7904` (`x + x^3*...`) and `0x802E613C` (`1 - x^2*...`)
  respectively, which fixes the order as `(sin, cos)`. `0x801C0970` forwards an
  axis character to [`fn_801c09f0`]; `0x801C0B50` forwards an axis vector to
  [`fn_801c0aa0`].
- **The axis-character rotation builder, `0x801C09F0` (176 bytes).** The
  selector is `axis | 0x20`, so `0x78`/`0x79`/`0x7A` cover both cases of
  `'X'`/`'Y'`/`'Z'`. Each arm is a 3x4 rotation with the fourth column zero:
  three rows built from `ps_merge00` pairs whose fields are the SDA2 one
  (`r2-0x4F50`) and zero (`r2-0x4F4C`). The listing resolves the pair stores
  that the export prints as a merged single store: the `W` bit at instruction
  bit 15 distinguishes a one-float store (offsets 0 and 44 in the X arm) from an
  eight-byte pair store (offsets 4, 12, 20, 28, 36).
- **The copy-filter writer, `0x801C5F40` (484 bytes).** Four BP registers
  (`0x01`-`0x04`) carry six 4-bit sample offsets each; `0x53`/`0x54` carry the
  seven 6-bit vertical-filter taps (four + three). The all-`0x66` sample
  default and the `{0, 0, 21, 22, 21, 0, 0}` tap default are literal
  immediates (`0x01666666`..`0x04666666`, `0x15`/`0x16`). Callers at
  `0x80169440`, `0x801C22C0`, `0x802BAED0` and `0x802C02C0` confirm the
  `(enable, 24-byte table, use-filter, 7-byte taps)` argument order. The
  `0x801C22C0` call site in this file now calls the body directly.
- **The three-arm BP block emitter, `0x801C7A40` (848 bytes).** Each arm is a
  literal sixteen-word table for registers `0x8C`-`0x93` then `0xAC`-`0xB3`,
  emitted as `0x61` + word. The arms differ only in the low bits of the second
  block (`0x8400`-`0xDC00` for the else arm, `0x8800`-`0xD800` plus
  `0xE000`/`0xE800`/`0xF000`/`0xF800` tails for `mode == 1`, and the `0xC400`/
  `0xCC00`/`0xD400` tails for `mode == 2`). `0x801C1DB0` calls it with `2`.
- **The fog setter, `0x801C8920` (556 bytes).** Bit 2 of the type word selects
  the orthographic family; the perspective family normalises
  `b = farz/(farz-nearz)` into `[0.5, 1)` while counting `fsel`, divides it by
  `1 << (fsel + 1)` and emits the `0xEF` parameter (through the `0x802DCC4C`
  double-to-int helper) and the `0xF0` range word. Every arm then emits `0xEE`
  (the `sp+12` coefficient truncated to its top 20 bits), `0xF1` (the `sp+8`
  coefficient plus the orthographic bit at bit 20 and the mode at bits 21-23),
  `0xF2` (`color >> 8`) and clears `state + 2`.
- **The CP matrix loader, `0x801C9A80` (180 bytes).** The `0x10` command byte,
  then `(count - 1) << 16 | address` where `address` is `id * 4` below `0x40`
  and `(id - 0x40) * 4 + 0x500` above it, then eight (`kind == 1`, the 2x4
  form) or twelve floats. This is the same `0x10` command shape that
  [`fn_801c5570`] closes with its trailing byte/halfword pair.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C0610` | `fn_801c0610` | 392 | medium |
| `0x801C0970` | `fn_801c0970` | 124 | high |
| `0x801C09F0` | `fn_801c09f0` | 176 | high |
| `0x801C0B50` | `fn_801c0b50` | 124 | high |
| `0x801C5F40` | `fn_801c5f40` | 484 | high |
| `0x801C7A40` | `fn_801c7a40` | 848 | high |
| `0x801C8920` | `fn_801c8920` | 556 | medium |
| `0x801C9A80` | `fn_801c9a80` | 180 | high |

## Notes on the recovered operands and stores

- **Instruction-stream check.** All nine functions end in `blr`
  (`0x4E800020`) exactly at `address + size`, so the `functions.tsv` sizes are
  the real body lengths, not gaps to the next symbol.
- **Paired-single save frames.** Several of these bodies open with
  `stfd fN,..` immediately followed by `psq_st fN,..` for `f19`-`f31`. That is
  the compiler's save of a 64-bit FPR (both halves), not a GX quantisation
  block; no `GQR0` logic is involved in `0x801C0610`'s or `0x801C90A0`'s save
  frames.
- **Copy-filter bit layout.** `rlwimi r0,r8,0,28,31` packs the low nibble of a
  sample byte, `rlwimi ...,4,24,27` the next, and so on; the all-`6` default
  table therefore assembles to `0x01666666`, which is the check that fixed the
  field order.
- **Fog divisor.** The `0x43300000`/`xoris 0x8000` pair is the integer-to-double
  magic; subtracting the `0x4330000080000000` constant yields
  `1 << (fsel + 1)` exactly. The `0xEF`/`0xF0` words are emitted before the
  `0xEE`/`0xF1`/`0xF2` group, and only on the perspective arm, matching the
  listing's two separate `stb 0x61` groups.
- **Fog caveat.** The listing's normalisation loop names one FPR in both the
  `fcmpu` and the `fmuls`; the export reads it as `b * 0.5` / `b * 2.0` and its
  constant addresses (`r2-0x4E38` = 1.0, `r2-0x4E40` = 0.5, `r2-0x4E30` = 2.0,
  `r2-0x4E28` = 0.5) exist and are used as the export says. The body follows
  the export on those four operands and says so in `# Confidence`; everything
  else in the fog body is literal.

## Deliberate skips

One address is not claimed.

- `0x801C90A0` (976 bytes) - a texture-coordinate / indirect stage emitter that
  loops over two caller counts and streams twelve floats per inner iteration to
  the FIFO through [`fn_801c5570`]'s `0x98` header. The export is not usable as
  a transcription: its inner loop contains only two `0xCC008000` stores
  (`_DAT_cc008000 = dVar11` and the ratio expression) while the listing writes
  twelve `stfs` per iteration, and the remaining ten were folded into
  `uStack_fc`/`uStack_f4` bookkeeping that does not correspond to any store.
  Reconstructing those ten floats would mean deriving the caller's FPU register
  map (`f19`-`f31` are unnamed incoming values in the export) and inventing the
  conditional four-float group, so it is left out rather than guessed.

All nine addresses in the bucket are therefore accounted for: eight translated
above, one skipped for the reason just given.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (8 rows, 2884 bytes, 6 high,
  2 medium, 0 low).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_<name>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`; all eight were the bucket's
  remaining untranslated set.
- `node tools/check-lane-csv.mjs wave106` reports no problem for
  `ledger/agents/wave106-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
