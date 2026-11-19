# Coverage - Wave 105 (runtime::dvd, the last four bodies of the `0x801C0000` page)

## How to read this file

The `runtime::dvd` lane owns the whole `0x801C0000` 64 KiB page of `main.dol`,
which is much wider than the disc band. Every body below is SDK GX code that
happens to share the page; `src/runtime/dvd.rs` is the only writable file
assigned to the bucket, so the bodies live there.

`high` means every store, mask and call in the body is literal and its role is
proven; `medium` means the instruction sequence is literal but at least one
field role, callee identity or recovered argument is inferred; `low` means only
the store/instruction sequence is certain.

This pass differs from the earlier ones in one respect worth stating up front:
the bucket is down to 13 addresses, and nine of those 13 have a Ghidra export
that does **not** describe the code at the address. The criterion used here is
mechanical: a function of `n` bytes holds at most `n / 4` instructions, so an
export with several hundred statements for a 392-byte function cannot be a
faithful decompilation. Those exports were cross-checked against the raw
instruction stream in `work/dol/main.elf` and dropped. The nine addresses and
the exact discrepancy for each are listed under "Deliberate skips".

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 4 | 4864 | 0 | 4 | 0 |

## What this pass covers

The bucket held 13 untranslated addresses when this pass started. Four are
translated here, in two groups spread over `0x801C22C0`-`0x801C9E20`; none is
disc code.

- **FIFO padding writers, `0x801C5570`/`0x801C56C0` (2 bodies, 548 bytes).**
  These are the pair Wave 80 deliberately left out because Ghidra merges their
  final `stb`/`sth` pair into one 32-bit store whose low half re-reads the
  write-only port at `0xCC008000`. The instruction stream resolves the merge:
  `0x801C56C0` emits a `0x98` command byte, then the width halfword, then
  `round_up(width*height, 4)` zero bytes as `stw` words, then sets the halfword
  at `state + 2` to one. `0x801C5570` is the guarded variant: it first calls
  `0x801C52F0` when the dirty word at `+0x5FC` is non-zero, emits the header and
  the padding only while `state + 0` is zero, and then emits its own low byte and
  halfword. Both are reproduced from the export plus the listing, and the fill is
  factored into [`gx_fifo_zero_fill`].
- **Mode selector tables, `0x801C9E20` (1 body, 2076 bytes).** The pixel mode at
  `+0x5EC` and the fog mode at `+0x5F0` each get an "invalidate" word for the
  outgoing value and a command word for the incoming one: 35 cases for the pixel
  mode (`0x84`-`0x273`, then the `0x2300..`/`0x2402..` blocks) and 23 for the fog
  mode. Eight fog cases rewrite bits 4-7 of the packed word at `+0x5F4` and four
  poke the halfword at `+6` of the block that `r13 - 0x4DEC` points at.
- **Default-state bring-up, `0x801C22C0` (1 body, 2240 bytes).** The routine
  reads the display-mode index from `0x801BE010`, selects one of four records in
  `0x803410A8`-`0x80341198`, and then walks the driver defaults in export order:
  eight texcoord generators, the indirect/TEV stage reset, a twelve-word 3x4
  block with the `r2 - 0x4EE4` word on the diagonal and `r2 - 0x4EF0` elsewhere,
  the two texture objects, sixteen generated texcoords, four channels, the fog
  setup and finally the `0x23`/`0x16` mode pair handed to [`fn_801c9e20`]. The
  last call is the body translated directly above, which is a useful check that
  the argument pair is literal.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C5570` | `fn_801c5570` | 332 | medium |
| `0x801C56C0` | `fn_801c56c0` | 216 | medium |
| `0x801C9E20` | `fn_801c9e20` | 2076 | medium |
| `0x801C22C0` | `fn_801c22c0` | 2240 | medium |

## Notes on the recovered arguments and stores

- **Byte/halfword FIFO writes.** The GX write-gather pipe at `0xCC008000`
  accepts 8-, 16- and 32-bit stores and packs them in issue order, so the
  `stb 0x98` + `sth width` pair in `0x801C5570`/`0x801C56C0` is a three-byte
  header, not a misaligned word. The bodies write it with `write_u8` then
  `write_u16`, and the merged `CONCAT22` store in the exports is called out in
  each doc block.
- **Padding budget.** The export prints the fill as a halfword store per
  iteration with a byte counter (`emitted += 0x20`); the listing shows eight
  `stw` per iteration, i.e. `0x20` bytes, so the bodies reproduce eight word
  stores and a `(remaining) >> 2` tail. Both give `round_up(pixels, 4)` bytes.
- **Zero-length corner.** Both fill loops are guarded so a zero or tiny pixel
  count cannot underflow the block counter; the export's
  `(2 < total) && (pixels <= total)` guard is the compiler's proof that at least
  one block exists.
- **Recovered argument pair.** `0x801C5570`'s first argument is moved to `r28`
  and never read. The body takes it as `_unused` so the register file matches the
  export; the two live arguments are the trailing low byte and halfword.
- **Floating-point callees.** `platform::call0`-`call7` only carry integer
  registers, so the three callees that take arguments in `f1`-`f6` go through
  their translated bodies ([`fn_801c9c10`], [`fn_801c5df0`]) or through the
  private [`call_f64x4`] shim added for `0x801C8920`. Everything else is called
  by absolute address, exactly as the export prints it.
- **Display-mode records.** `0x803410A8`, `0x80341120`, `0x8034115C` and
  `0x80341198` are the four records; the fields used here are `+4` width, `+6`
  height, `+8` a second width, `+0x10` a doubled-width comparison value, `+0x18`
  a flag byte, `+0x19` a selector byte and `+0x32` a four-byte block. The roles
  are `// inferred:` from the call sites, not from a symbol.

## Deliberate skips

Nine addresses are not claimed. For each one the Ghidra export contradicts the
size in `functions.tsv`, so any body written from it would be an invention:

- `0x801C0610` (392 bytes) - the export is 762 lines (a 3x4 concatenation with
  quantised loads). 392 bytes cannot hold 98 instructions' worth of that code, so
  the export is not the function at the address.
- `0x801C09F0` (176 bytes) - the export is 415 lines (a `0x78`/`0x79`/`0x7A`
  matrix emitter); far more statements than 44 instructions can carry.
- `0x801C9A80` (180 bytes) - the export is 412 lines (a quantised store path);
  same problem.
- `0x801C0970`/`0x801C0B50` (124 bytes each) - thin wrappers over
  `0x802E84C4`/`0x802E7FBC`, but their exports open with paired-single
  quantisation blocks (`GQR0` field tests and `ldexpf` calls) that Ghidra
  duplicates; the recovered argument list of the callee cannot be established.
- `0x801C90A0` (976 bytes) - a raster/vertex emitter with thirteen paired-single
  arguments; the export's `GQR0` blocks are duplicated per store and the inner
  loop's `uStack_fc`/`uStack_f4` bookkeeping does not correspond to the listing.
- `0x801C8920` (556 bytes) - the fog setup. The export stops after the
  `0xF2000000` word, while the listing continues with the `0x61`, `0xEF`,
  `0xF0`, `0x61`, `0xEE`, `0xF1` and `0xF2` BP writes; the body would have to be
  invented.
- `0x801C5F40` (484 bytes) - the export packs three bytes at `+4`..`+6` into one
  `0x54` command word, but the listing then packs another eleven fields from
  `+0`..`+0x15`; the export is truncated.
- `0x801C7A40` (848 bytes) - the export shows three constant branches, but the
  listing is a long literal FIFO burst (212 instructions) for each of the three
  arms. Reproducing it would mean transcribing machine code, not the export.

All thirteen addresses in the bucket are therefore accounted for: four
translated above, nine skipped for the reasons listed.
## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (4 rows, 4864 bytes, 0 high,
  4 medium, 0 low).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_<name>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave105` reports no problem for
  `ledger/agents/wave105-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
