# Coverage - Wave 102 (`runtime::os`, `0x801B0000` bucket)

## How to read this file

This page records wave 102 of the `main.dol` reconstruction for the
`runtime::os` lane: the last open run of the `0x801B0000` bucket. The bucket
holds 227 inventory entries; 198 were already merged when this pass started,
29 were open, and 25 of those are reconstructed here (`0x801B06A0`..`0x801BFD40`).

- **Status** - every row is `translated`: each guard, store and call printed in
  the Ghidra export has a counterpart in `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. `high` marks bodies
  whose whole instruction sequence is literal (two of them are four-byte branch
  stubs into bodies this file already had). `medium` marks the bodies whose
  field roles or callee identities are `// inferred:`. `low` marks `0x801BD000`
  alone, whose third argument the only caller never sets.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave102-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 25 | 17,048 | 9 | 15 |

Status split: 25 `translated`, 0 `partial`, 0 `stub`. The twenty-fifth row,
`0x801BD000`, is `low`; it is still a complete translation, the low mark only
records that its third argument cannot be verified.

## What this pass covers

- **Cluster A - the video interface, `0x801BC480`-`0x801BDF80`.** The 25 open
  addresses are dominated by the piece of the OS that programs the VI register
  file at `0xCC002000`. `0x801BC480` is the fifteen-case TV-format record
  selector, `0x801BC5A0` copies one record into about twenty VI halfwords,
  `0x801BC7A0` is the one-shot initialiser (record selection, the whole
  `0x8038FAE0`..`0x8038FB34` scan window, the retrace-handler install and the
  three region timeouts), `0x801BD280`/`0x801BD950` rebuild that window when the
  TV format or the visible size changes, and `0x801BC5A0`'s two consumers
  `0x801BCD50`/`0x801BD0E0` publish the `XFB` addresses and the four quad
  corners.
- **Cluster B - the scan-out shadow protocol.** `0x8038FA68` holds *pending*
  values and `0x8038F9F0` the shadow the retrace handler uploads (both already
  named `W98_SI_SCAN_VALUE`/`W98_SI_SCAN` by Wave 98, which reached them from
  the SI side). Every producer here writes one halfword and sets one bit of the
  two pending words `sda(-0x4E84)`/`sda(-0x4E88)`; `0x801BDCB0` folds those bits
  into the upload pair `sda(-0x4E74)`/`sda(-0x4E78)` while copying the values
  into the shadow, and `0x801BBBD0` drains the upload pair into
  `0xCC002000 + index * 2`. That closes the loop Wave 98 left open.
- **Cluster C - the front-panel GPIO layer, `0x801BE480`-`0x801BFD40`.**
  `0x801BE480` is the bit-clock delay (elapsed ticks, `* 8`, modulo `0x1E6`),
  `0x801BE510` clocks one byte out of `0xCD8000C0` bit `0x8000`, `0x801BEFC0`
  selects one of thirteen 26-byte records for the current TV format, and
  `0x801BFD40` rebroadcasts a `0x22`-byte record. This is the same `0xE0`
  channel the Wave 86 senders use, and `0x801BEFC0` is now the callee those
  senders never explained.
- **Cluster D - the retrace handler, `0x801BBBD0`.** The interrupt entry that
  acknowledges the four VI interrupt words, uploads the shadow, republishes the
  clock bit through `0x801BEDA0`, maps the TV region to its two timeout triples
  and drains the pending command mask `sda(-0x4E08)` through the eight `0xE0`
  senders.
- **Cluster E - reset, disc and text, `0x801B06A0`-`0x801B7810`.** The
  character decoder `0x801B06A0` (five encoding modes plus the two font bases),
  the `OSReturnToMenu`/`OSHotReset` ladder `0x801B3870`, the EXI device probe
  `0x801B3E20`, the title/disc state machine `0x801B6D60` and the drive-settle
  report `0x801B7810`.

## What the pass adds beyond the raw bodies

1. **The VI register file is now mapped.** The section header names
   `0x2000`/`0x2002` (vertical timing and display control), `0x2004`..`0x200A`
   (horizontal timing), `0x200C`..`0x201A` (`XFB` window), `0x202C`/`0x202E`
   (beam position, `& 0x7FF` in every reader), `0x2030`..`0x203C` (the four
   interrupt words, bit `0x8000`), `0x2046`..`0x2074` (burst/blanking and the
   clock pair) and `0x2072`/`0x2074`.
2. **The pending-bit convention is written down.** The export prints each
   producer's bit as a bare OR into `sda(-0x4E88)`; the drain loop in
   `0x801BDCB0` shows that the bit is the field index counted from the top of
   the high word, so the eight literal masks in `0x801BCD50` and the five in
   `0x801BD0E0` can be checked against `0x8038FA68 + index * 2`.
3. **The field/clamp idioms are named.** The `x & ((int)x >> 31)` pairs the
   export prints throughout `0x801BC7A0`, `0x801BD280` and `0x801BD950` are the
   compiler's `MAX(x, 0)`; they are written as `.max(0)` with a `// observed:`
   note, which is what makes the aspect-scale arithmetic readable.
4. **The `0x802DCDxx` handoffs stayed handoffs.** As in waves 98/101 the entries
   at `0x802DCD4C`..`0x802DCD74` are callee-saved-register saves, so the
   `r3`/`r4` pair they "return" is the incoming argument pair. Bodies here take
   those halves as ordinary parameters and name the half each one came from;
   where the export prints a *stale* register (`extraout_r4`) the value is
   marked `// unknown:` rather than guessed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B06A0` | `fn_801b06a0` | 436 | medium |
| `0x801B3140` | `fn_801b3140` | 4 | high |
| `0x801B3230` | `fn_801b3230` | 4 | high |
| `0x801B3870` | `fn_801b3870` | 648 | medium |
| `0x801B3E20` | `fn_801b3e20` | 512 | medium |
| `0x801B6D60` | `fn_801b6d60` | 1,204 | medium |
| `0x801B7810` | `fn_801b7810` | 148 | medium |
| `0x801BBBD0` | `fn_801bbbd0` | 2,060 | medium |
| `0x801BC480` | `fn_801bc480` | 280 | high |
| `0x801BC5A0` | `fn_801bc5a0` | 512 | medium |
| `0x801BC7A0` | `fn_801bc7a0` | 1,352 | medium |
| `0x801BCD50` | `fn_801bcd50` | 680 | medium |
| `0x801BD000` | `fn_801bd000` | 224 | low |
| `0x801BD0E0` | `fn_801bd0e0` | 404 | medium |
| `0x801BD280` | `fn_801bd280` | 1,740 | medium |
| `0x801BD950` | `fn_801bd950` | 852 | medium |
| `0x801BDCB0` | `fn_801bdcb0` | 276 | high |
| `0x801BDDD0` | `fn_801bddd0` | 108 | high |
| `0x801BDE50` | `fn_801bde50` | 120 | high |
| `0x801BDF80` | `fn_801bdf80` | 144 | medium |
| `0x801BE0B0` | `fn_801be0b0` | 564 | medium |
| `0x801BE480` | `fn_801be480` | 136 | high |
| `0x801BE510` | `fn_801be510` | 840 | medium |
| `0x801BEFC0` | `fn_801befc0` | 3,448 | high |
| `0x801BFD40` | `fn_801bfd40` | 352 | high |

## Deliberate skips

Four of the 29 open addresses were read and left alone. All four are honest
skips, not partial rows:

1. **`0x801B0170` (784).** The body copies `0x4D000` or `0x3000` bytes in
   `0x100`-byte chunks through `0x801B4310`, checks the `"Yay"` magic, and then
   writes four halfwords into a texture buffer whose address arithmetic is the
   paired-single/GQR form the earlier waves rejected. The two block ranges and
   the magic test are readable, but the four destination offsets depend on
   `GQR`-shaped index scaling, so the stores would be guesses.
2. **`0x801B0860` (648).** The glyph-bitmap packer walks the same texture as
   `0x801B0170` and ORs nibbles into it through the same `GQR` index form, with
   the tile origin in `in_r13` slots. The loop is literal but the bit offset
   combines two `>>`-rounded quotients whose roles are not separable from the C.
3. **`0x801B6080` (808).** A ten-word calendar decomposition. It divides the
   tick count with `0x802DD0E0`/`0x802DCEC4` and takes each quotient from
   `extraout_r4`, i.e. the decompiler prints the same call as both a 64-bit
   return and a separate remainder. Which half is the quotient cannot be
   settled from the C, and getting it backwards would silently shift every
   field, so no row is claimed.
4. **`0x801BE860` (1,332).** The byte-at-a-time `0xE0` request sender that the
   Wave 86 section calls by address. Its buffer pointer is recovered from the
   `0x802DCD60` fold and its channel argument is re-used as the
   brightness/clock mask of `0x801BE510`; the body is a fourteen-wait
   bit-bang, and the pointer/argument split cannot be proven from the C. Wave
   86 already documents the call shape it uses, so nothing regresses by leaving
   the body open.

No address in this pass was already present in `ledger/functions.csv`; the 198
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (25 of 25; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes.
- `node tools/check-lane-csv.mjs wave102` prints `OK` with `wave102-os.csv`
  counted at 25 rows.
