# Coverage - Wave 113 (`runtime::bt`, the text appender)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave113-bt.csv` names the exact Ghidra C export.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles the evidence does not prove are marked rather
than named. The `0x80220000` bucket is shared with the SDP, IPC/EXI, render and
text code documented by waves 60, 69, 84, 97, 102 and 108; this pass only
touches the one entry those passes left open.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 1 | 2052 | 0 | 1 | 0 |

## What this pass covers

The `0x80220000` bucket holds 216 functions. When this pass started exactly two
were untranslated, both of them deliberate skips inherited from wave 108:
`0x8022FA70` (the text appender) and `0x80228644` (the EXI block worker). The
appender reconstructed cleanly in this pass; the EXI worker did not, so the
bucket is left with one open entry (215 of 216 translated).

### Text appender (`0x8022FA70`, 2052 bytes)

This is the draw/append core that `fn_8022eaf0`, `fn_8022eed0` and
`fn_8022f010` reach through [`TEXT_OUT_8022FA70`]. It is the draw-side twin of
the layout/measure pass `fn_8022f350` (wave 108):

1. `0x80230280` (translated as `resource::bres::fn_80230280`, called here by
   address) measures the buffer and adjusts the starting pen for the anchor mode
   in the object's `+0x5C` word (`0x30` for x, `0x300` for y).
2. The encoded buffer is walked one character at a time. Codes below `0x20` are
   glyphs: the font object at `+0x60` is asked to measure the glyph through
   virtual slot `+0x10` when a limited object must know whether the glyph still
   fits, then to draw it through slot `+0x0C`, which advances the
   `{object, cursor}` pair.
3. Codes at or above `0x20` are spacing. The inner object at `+0x48` supplies
   the advance through slot `+0x48` (scaled by `+0x24`) or the fixed `+0x44`
   value, and the line starts through `fn_8022d5f0`.
4. The draw slot returns a status: `1` resets the wrap flag, `2` sets it, `3`
   asks for a re-layout through `fn_8022f350` (over a sentinel-seeded copy of the
   object), and `4` jumps straight to the finish block.
5. The finish block reports `max(measured, pen_x - initial_x)` and restores,
   advances or re-anchors the pen according to the `+0x5C` anchor bits and the
   `newline` argument.

The body keeps every branch, mask, store and virtual call literal. The two
17-word stack snapshots are exact copies of the object with only `+0x2C`/`+0x30`
replaced by the `SDA2_F32_4C34` sentinel, which is what `fn_8022f350` itself
does. The spacing fields (`+0x24` glyph scale, `+0x28` line height, `+0x44`
fixed advance, `+0x4C` limit, `+0x50` line step) are named from the arithmetic;
the two virtual slot roles stay `// inferred:` because the font and inner
objects have no symbol names.

As with `fn_8022f350`, the opening `ldexpf`/`CONCAT` block is the compiler's
paired-single save of `f22..f31` (`GQR0` spill) and is not data flow, so the
body does not reproduce it. The result is returned as an `f64`; the existing
formatter wrappers that call it by address still declare a `u64`/`u32` result,
which is unchanged by this pass.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8022FA70` | `fn_8022fa70` | 2052 | medium |

## Deliberate skips

- `0x80228644` (744 bytes) - the EXI block worker. Its export carries twelve
  `Removing unreachable block` warnings (`0x802287E0..0x80228910`), every loop
  body was removed, and what remains after the `0x802DCD70` prologue is a
  countdown prelude followed by `do { } while (true)`. Ghidra did not recover
  the transfer blocks, so any translation would be invented. It remains a call
  by address for the wave-92 EXI helpers and gets no CSV row.

## Verification

- Every address and byte size matches `functions.tsv` (`8022fa70`, 2052 bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/8022fa70_FUN_8022fa70.c`.
- The row has exactly one `pub unsafe fn fn_8022fa70` body in
  `src/runtime/bt.rs`.
- `0x8022FA70` was not in `ledger/functions.csv` before this pass; it came from
  the bucket's untranslated list.
- `cargo check --message-format=short` reports no error in
  `src/runtime/bt.rs`; the remaining crate errors belong to other Wave 113
  lanes. The only `bt.rs` diagnostics are the pre-existing
  `unnecessary parentheses` warnings in the untouched L2CAP code.
- `node tools/check-lane-csv.mjs wave113` reports this lane clean.
