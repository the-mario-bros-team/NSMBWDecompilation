# Coverage - Wave 111 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records wave 111 of the `main.dol` reconstruction for the
`runtime::os` lane. The `0x80240000` bucket holds 137 inventory entries; 134
were already merged when this pass started and three were open
(`0x802403C0`, `0x802471E0`, `0x8024E180`). Two are reconstructed here; the
third is deliberately left open and the reason is recorded under "Deliberate
skips".

- **Status** - both rows are `translated`: every guard, store, call and
  arithmetic expression printed in the Ghidra export has a counterpart in
  `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. Both rows are
  `medium`. `0x802403C0`'s key layout is read from the extraction instructions
  themselves, and its last 88 bytes are instruction-for-instruction the
  `0x80240240` tail already translated in Wave 108; only the GQR lanes and the
  `scale * raw + bias` decode are `// inferred:` from the export.
  `0x802471E0`'s loop, mask walk, six store slots and two calls are literal;
  the purpose of two of the stores and the `0x8023DAA0` stage number stay
  `// unknown:`.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave111-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 2 | 920 | 0 | 2 |

Status split: 2 `translated`, 0 `partial`, 0 `stub`; 0 of the 2 are `low`.

## What this pass covers

- **`0x802403C0` is the packed-key twin of `0x80240240`.** Same 16-byte header
  (`u16 count`, pad, `f32` rate at `+4`, `f32` scale at `+8`, `f32` bias at
  `+0x0C`), same fixed-point index hint `rate * (t - first) * count`, same
  one-step neighbour search, same Hermite expression. Only the key stride and
  the key layout differ: four bytes per key, and the word packs three fields.
  Wave 108 rejected this body because the C prints the value at bits 12..23 and
  the tangent at bits 8..11 (`(x << 20) >> 28`), and a third extraction at bits
  0..11, which cannot all describe one word. The instruction stream settles it:
  the search compares the **top byte** with `lbz`/`cmplw`, the value comes from
  `rlwinm rD,rS,20,20,31` (keeps bits 0..11) fed to the 16-bit GQR lane, and the
  tangent from `rlwinm rD,rS,20,0,11` followed by `srawi rD,rD,20`
  (sign-extends bits 12..23) fed to the signed-16-bit lane. So the layout is
  `time` in bits 24..31, `tangent` signed in bits 12..23, `value` unsigned in
  bits 0..11 - 8 + 12 + 12 bits. The exact-hit path returns the decoded value;
  the two range ends return the first/last key's value.
- **`0x802471E0` is the per-slot quantiser between the channel record and the
  packed-stage encoder.** `0x802DCD74`'s pair supplies the word whose value is
  copied through `0x80247140` and the structure holding the nibble mask at
  `+0x04` and the `0x14`-strided slot array at `+0xAC`. Every set nibble
  identity-initialises its slot through `0x80250D40`, reduces the six
  magnitudes at `+0x00`, `+0x04`, `+0x0C`, `+0x10`, `+0x14`, `+0x1C` to one
  maximum (seeded from `sda2(-0x4A50)`, which the image holds as
  `0x219392EF`, about `1.0e-18`), derives `2^(126 - exponent(max))` through
  `0x802E825C` (`ldexp`) and scales the slot in place. The store slots are
  `+0x00`, `+0x04`, `+0x08`, `+0x10`, `+0x14`, `+0x18`: the two `+0x0C`/`+0x1C`
  values land in the `+0x08`/`+0x18` slots. Then
  `0x8023DAA0(record, stage, slot, exponent)` encodes the slot with the stage
  counter starting at 8. Wave 108 rejected this body because the reducer reads
  `0/4/0xC/0x10/0x14/0x1C` while `0x8023DAA0` encodes `0/4/8/0x10/0x14/0x18`;
  both are correct - the six `stfs` really do write the second set - so the
  apparent disagreement is the function's actual behaviour and is now recorded
  verbatim instead of blocking the translation.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802403C0` | `fn_802403c0` | 492 | medium |
| `0x802471E0` | `fn_802471e0` | 428 | medium |

## Deliberate skips

**`0x8024E180` (2,212 bytes) is left open.** The body is a draw emitter whose
calls are all readable (`0x802DCD44`, `0x8024DCE0`, `0x8023E370`, `0x8023D460`,
`0x8023DC70`, `0x8023DCB0`, `0x8023D1B0`, `0x8023D8B0`, `0x8024CC10`,
`0x80249D30`, `0x8024D5E0`, `0x8023C220`, `0x8023D240`, `0x802E861C` (`pow`),
`0x801C8500`, `0x801C7E00`, `0x8023E9A0`, `0x8024CDC0`, `0x8023E6C0`,
`0x8023E7A0`, `0x8023E880`, `0x802DCD90`), but the vertex words it builds are
stitched byte by byte across three overlapping stack scratch areas
(`r1+0x68`, `r1+0x158`..`r1+0x174` and `r1+0xB0`) with `fctiwz` conversions and
`stb`/`stw` pairs, and the export's `CONCAT13`/`CONCAT12` nesting does not name
the bytes those stores actually write. The register-save prologue
(`psq_st`/`psq_l` around `f23`..`f31`, restored before `0x802DCD90`) is a
callee-save idiom, not an argument list, so the `void` signature is not the
obstacle; the packed words are. Reproducing them would require asserting byte
positions the evidence does not fix, so no body was written. This matches the
decisions of Waves 104 and 108 for the same address.

No other skip. The other 134 entries in `0x80240000` were already present in
`ledger/functions.csv` and were filtered out before any body was written; no
address was duplicated and no body was invented.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (2 of 2; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build warnings belong to other lanes
  (`src/runtime/bt.rs`, `src/runtime/cpp.rs`, `src/render/gx_band.rs`,
  `src/runtime/input.rs`, `src/ui/menu.rs`).
- `node tools/check-lane-csv.mjs wave111` is run for the lane; problems naming
  another lane's CSV belong to that lane.
