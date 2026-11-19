# Coverage - Wave 110 (`runtime::os`, `0x801B0000` bucket)

## How to read this file

This page records wave 110 of the `main.dol` reconstruction for the
`runtime::os` lane. The `0x801B0000` bucket holds 227 inventory entries; 223
were already merged when this pass started and four were open. All four are
reconstructed here, so the bucket is closed in this lane.

- **Status** - every row is `translated`: each guard, store, call and
  arithmetic expression printed in the Ghidra export has a counterpart in
  `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. All four rows are
  `medium`: the instruction-level arithmetic, call order and constants are
  literal, but in `0x801B0170`/`0x801B0860` the roles of the font-record fields
  (`+0x10`..`+0x24`, `+0x2C`) are `// inferred:` from the two bodies plus the
  Wave 38 glyph readers, and in `0x801B6080` the ten output fields follow the
  SDK `OSCalendarTime` layout rather than a recovered symbol.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave110-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 4 | 3,572 | 0 | 4 |

Status split: 4 `translated`, 0 `partial`, 0 `stub`; 0 of the 4 are `low`.

## What this pass covers

- **`0x801B0170` is the font loader behind `0x801B0480`.** It reads `0x4D000`
  bytes from device address `0x1AFF00` (mode `1`) or `0x3000` bytes from
  `0x1FCF00` (other modes) into the incoming `r3` buffer, in `0x100`-byte
  chunks through the SRAM command+DMA wrapper `0x801B4310`, retrying each chunk
  until it reports success. A `"Yay0"` header then makes the word at `+4` the
  decompressed size; `0x801AFEF0` expands the payload into the incoming `r5`
  buffer and the size is published through the `0x802DCDB8` close entry - which
  is exactly the value `0x801B0480` adds to its second font base. Mode `1`
  also rewrites four halfwords of glyph `0x54`: the glyph offset from
  `0x801AFDB0(1, 0x54)` is split into page/cell row/cell column with the
  record's grid, and the two cached low-memory words `sda(-0x4F70)` /
  `sda(-0x4F6C)` (`0x80003134`/`0x80003138`) are stored big-endian at the
  fourth pixel slot of the cell. That is the "version text drawn from RAM"
  glyph the boot code fills before the font is used.
- **`0x801B0860` is the glyph blitter.** It reuses the Wave 38 mode word and
  decoder pointer (the initialise block is byte-for-byte
  `wave38_text_mode_ensure`), decodes the text byte through the installed
  decoder, and then copies the cell pixel by pixel. The source is a 2 bpp,
  8x8-blocked atlas: the block index is `stride * ceil(y/8) + 0x10 *
  ceil(x/8)`, the byte within a block is `(y%8)*2 + (x%8)/4`, and the two-bit
  lane is selected by `x % 4`. The two-bit value indexes the four-byte palette
  at `font + 0x2C`; the resulting byte is OR-ed into the destination nibble
  (high nibble for even columns, low for odd). The destination is also tiled
  in 8x8 blocks, but 0x20 bytes each because it is 4 bpp; its blocks per row
  come from the rotate `stride << 2 | stride >> 30`. The final optional word
  is the glyph's width byte from the table at `font + +0x22`. This is the
  missing `0x802DCD4C` consumer of the text decoder, and it confirms the
  `+0x10`/`+0x12` (cell size) versus `+0x1A`/`+0x1C` (page grid) split used by
  `0x801B0170`.
- **`0x801B6080` is the tick-to-calendar decoder.** The tick rate is
  `0x800000F8 >> 2`. `0x802DD0E0` returns `time % rate` (`__moddi3` shape: it
  returns the dividend unchanged when the dividend is smaller than the
  divisor), `0x802DCEC4` returns the quotient (`__divdi3` shape). The remainder
  becomes microseconds (`* 8 / (rate / 125000)`) and milliseconds
  (`/ (rate / 1000)`, both reduced modulo 1000); `time - remainder` divided by
  the rate is whole seconds, then days and seconds-within-day. The civil-date
  part is the March-based 400-year algorithm with the `0x0B2575` (five
  400-year cycle) shift, and the month comes from the 12-entry cumulative
  tables at `0x80333D90` (leap) and `0x80333D60` (common) - note the bases are
  `-0x7FCCC270` and `-0x7FCCC2A0` in the export, and the common table ends
  exactly where the `0x80333DC0` text table begins. The ten field order is the
  SDK `OSCalendarTime` layout.
- **`0x801BE860` is the multi-byte GPIO sender.** It is the twin of
  `fn_801be510`: the first byte goes through `0x801BE510` itself (whose return
  value is read from `r3` after the `0x802DCDB8` close), then each byte of the
  buffer is shifted out of `GPIO_OUT` bit `0x8000` with bit `0x4000` as the
  clock, three `fn_801be480(2)` waits per bit, and the two closing clock pulses
  per byte. The `sda(-0x60F8)` polarity inversion and the
  `GPIO_IN` bit `0x8000` read-back abort test are the same as in the single
  byte sender. The result (`1` finished, `0` aborted) goes to `0x802DCDAC`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B0170` | `fn_801b0170` | 784 | medium |
| `0x801B0860` | `fn_801b0860` | 648 | medium |
| `0x801B6080` | `fn_801b6080` | 808 | medium |
| `0x801BE860` | `fn_801be860` | 1,332 | medium |

## Deliberate skips

None. Every one of the four open entries in the bucket was reconstructed and
landed. The other 223 entries in `0x801B0000` were already present in
`ledger/functions.csv` and were filtered out before any body was written; no
address was duplicated and no body was invented.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (4 of 4; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build errors belong to other Wave 110
  lanes (`src/runtime/input.rs`, `src/engine/dsys.rs`, `src/game/enemies.rs`).
- `node tools/check-lane-csv.mjs wave110` is run for the lane; problems naming
  another lane's CSV belong to that lane.
