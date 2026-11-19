# Wave 222, lane C - `src/resource/archive.rs`, addresses `0x802CEC60..0x802CEE70`

Three entries of `work/ghidra/main-gb/export/functions.tsv` at the tail of the
`0x802C0000` band of the `resource::archive` ledger module. All three are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802CEC60` | 288 | 288 (`blr` at `0x802CED7C`) | lerp four byte components |
| `0x802CED80` | 228 | 228 (`blr` at `0x802CEE60`) | build the 3x4 texgen/rotation matrix |
| `0x802CEE70` | 244 | 244 (`blr` at `0x802CEF60`) | fill and flush one mode-2 draw context |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` in `work/dol/main.elf`, and in every
case `prologue_offset + 4` equals the tsv size and the `blr` word ends exactly
on that boundary. Bytes translated: **760**.

## What the run is

These three are the small helpers the effect/render path of the band calls just
before the `0x802CEF70` strip builder (wave 221's neighbours):

1. **Colour lerp.** `0x802CEC60(t, out, from, to)` interpolates four bytes and
   truncates. Its four callers in `0x802CFD00` pass a pair of goal colours and
   the result goes straight into `0x802C1280(slot, x, side, &colour)`, so the
   four bytes are one colour word.
2. **Texgen matrix.** `0x802CED80(this, out)` derives an angle from
   `*(f32 *)(this + 0x24)`, calls `0x80237CA0` for the sine/cosine pair and
   writes twelve floats - a rotation with a `0.5` bias on the third column, the
   shape GX projected texcoords need. `0x802CF1E0` builds it on the stack and
   hands it to `0x801C4A20`.
3. **Mode-2 context.** `0x802CEE70(source)` lazily creates the two global
   effect vectors, builds a `0x88`-byte `0x802D0E50` context on the stack,
   forces mode `1`, writes the identity scale and the `(0, 0)` texcoord offset,
   and flushes it with `0x802D0F40`. The two existing call sites in the target
   file (`FN_802CEE70_A`, `FN_802CEE70_B`) pass the HUD object and the live
   render context respectively, which is the one-argument shape translated here.

`// inferred:` the owner is the effect/particle renderer at the end of the layout
band; the exports prove the offsets, the constants, the callee addresses and the
argument order.

## Constants

Every scalar was read out of the image at its `r2`-relative address
(`SDA2_BASE` = `0x80433080`), so the values quoted in the bodies are shipped
words: `1.0f` at `0x8042F4C4`, `0.0f` at `0x8042F4C0`, `0.5f` at `0x8042F4E8`,
`3.14159274f` at `0x8042F4E0` and `40.7436638f` at `0x8042F4E4` (which multiply
to exactly `128.0f`), plus the two integer-to-double magic words
`4503599627370496.0` (`0x8042F4C8`) and `4503601774854144.0` (`0x8042F4D8`).

## Notes

* `0x802CEC60` uses the `xoris`/`CONCAT44(0x43300000, x)` idiom, so the delta is
  the signed `to - from`; for `u8` operands the magic-double cancellation is
  exact and the body writes the same `from + t * (to - from)` expression. The
  only deviation is Rust's saturating float-to-int cast, which can differ from
  `fctiwz` only outside `i32` range (unreachable for two `u8` endpoints).
* `0x802CED80` keeps the compiler's redundant `* 1.0` and `fneg` because they
  are visible in the shipped words, and it keeps the two angle factors separate
  rather than folding them into `128`, because the rounding order is part of the
  original expression.
* `0x802CEE70` zero-initialises its stack context; the original leaves the bytes
  the constructor does not write indeterminate. The context size (`0x88`) was
  derived from the `0x90` frame and the `r1 + 8` base and cross-checked against
  `0x802D0E50`'s highest write (`+0x84`).

## Verification

Copied `src/`, `Cargo.toml` and `Cargo.lock` to a scratch directory, appended
`work/wave222/laneC_block.rs` to that copy's `src/resource/archive.rs` and ran
`cargo check --lib`: **exit 0** (`Finished dev profile`).
