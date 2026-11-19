# Coverage - Wave 109 (runtime::dvd, the NAND error-log pair and the record comparator)

## How to read this file

The `runtime::dvd` lane owns the whole `0x801D0000` 64 KiB page of `main.dol`,
which is much wider than the disc band. Every body below is non-disc code that
happens to share the page; `src/runtime/dvd.rs` is the only writable file
assigned to the bucket, so the bodies live there.

`high` means every store, mask and branch in the body is literal and its role is
proven; `medium` means the instruction sequence is literal but at least one
field role or callee identity is inferred; `low` means only the store sequence
is certain.

This pass started from the six addresses still unclaimed in the bucket. Four are
translated here; the remaining two are skipped for the concrete reasons under
"Deliberate skips". The four bodies are the NAND error-log writer pair and the
record comparator, all in the `0x801DA990`-`0x801DE010` range.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 4 | 3904 | 0 | 4 | 0 |

## What this pass covers

- **The bounded IOS wait, `0x801DA990` (204 bytes).** Entered only with
  `(0, 2)`. It captures the 64-bit tick, submits IOS command `0x0D` through
  `0x80227AE0` with the local latch callback at `0x801DAA60`, and spins until
  the latch is non-zero or `500` scaled periods have elapsed. The period comes
  from the exact `rlwinm`/`mulhwu` sequence at `0x801DA9FC`-`0x801DAA14`: the
  `mulhwu` high word with the low six bits cleared. Ghidra prints the function
  as `void` because the `0x802DCD70` register-save thunk hides the incoming
  `r3`/`r4`, but the listing tests `r3 == 0` and `r4 == 2` at
  `0x801DA9A4`/`0x801DA9AC`.
- **The NAND error-log front end, `0x801DB2B0` (248 bytes).** Claims the
  one-shot latch [`fn_801db250`], saves `r3`-`r10` and `f1`-`f8` into the
  `vsnprintf` `va_list`, formats into `0x80395E40`, stores the callback at
  `r13-0x4C38` and step `1` at `r13-0x4C34`, then opens
  `"/shared2/test2/nanderr.log"` with mode `3` and completion
  [`fn_801db3b0`]. The export's sixteen parameter slots are the variadic save
  area, not sixteen real arguments: both call sites in the image pass only the
  two fixed words.
- **The nine-step completion, `0x801DB3B0` (1156 bytes).** The three 0x100-byte
  buffers are consecutive at `0x80395E40`: the first is the formatter output,
  the second holds the previous on-flash image and the third the CRLF-terminated
  line. The steps open the file, seek to `0`, read the previous image, seek to
  `0`, build the calendar line (`"%d %04d/%02d/%02d %02d:%02d:%02d %s %s"` plus
  the home-directory suffix), write it, parse the previous image, seek by
  `value << 8`, read it back, close, and finally store `0xFFFFFF01` and report
  `1`.
- **The record comparator, `0x801DE010` (2296 bytes).** Two 0x60-byte records
  are compared with fixed thresholds. The `0xC` group covers `+0x04`/`+0x02`/
  `+0x06`, the `2` group covers eight words from `+0x08` to `+0x22`, and the
  per-state rule is selected by `context + 0x8B8` (`3`-`5`, `6`-`8`/`0xB`/`0xF`,
  `0xA`, `0xC`, `0xD`/`0xE`). Each group has a mismatch counter and a decay
  counter (`+0x8F8`/`+0x8FE`, `+0x8FA`/`+0x900`, `+0x8FC`/`+0x902`). The
  compiler's `((x ^ k) >> 1) - ((x ^ k) & x)` masks reduce to
  `abs(difference) > k`, which is what the body writes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801DA990` | `fn_801da990` | 204 | medium |
| `0x801DB2B0` | `fn_801db2b0` | 248 | medium |
| `0x801DB3B0` | `fn_801db3b0` | 1156 | medium |
| `0x801DE010` | `fn_801de010` | 2296 | medium |

## Notes on the recovered operands and stores

- **Instruction-stream check.** All four bodies end in `blr` exactly at
  `address + size`, so the `functions.tsv` sizes are the real body lengths.
- **The wait period.** `rlwinm r0,r0,30,2,30` is `(bus >> 2) & 0x3FFFFFFF`;
  `mulhwu` takes the high word of the product with `0x10624DD3`; and
  `rlwinm r6,r0,26,6,30` keeps bits 6..30 of the rotated high word. The body
  spells the three steps out instead of importing the decompiler's
  `_DAT_800000f8 / 4000` shorthand.
- **The NAND log buffers.** `0x80395E40`, `0x80395F40` and `0x80396040` are
  consecutive 0x100-byte buffers. `0x8039613E`/`0x8039613F` in the export are
  the last two bytes of the third buffer, and the body writes the `0x0D`/`0x0A`
  pair there directly.
- **The variadic save area.** `0x801DB2B0` stores the `va_list` word
  `0x0200_0000`, which means "the next integer argument is `r5`". The body
  builds the same three-word `va_list` and the same `r3`-`r10` / `f1`-`f8` save
  area. The observed callers pass no conversions, so the tail is zeroed rather
  than guessed.
- **The comparator masks.** The repeated expression
  `((int)(d ^ k) >> 1) - ((d ^ k) & d)` is negative exactly when `d > k` for a
  non-negative magnitude `d`; the `(d ^ 1)` variant is the `d != 0` test. The
  body states the comparisons that way and keeps the original counter updates.

## Deliberate skips

Two addresses are not claimed.

- `0x801D6F60` (2036 bytes) - the range-decoder entry for the compressed NAND
  image. The body reads a 32-bit symbol, then runs the input through the
  four-instruction `rlwinm`/`rlwimi` network at `0x801D7040`-`0x801D7050`
  before testing the result against `0x40`, and only then stores a 24-bit
  value. The export prints the network as a single `uVar7 << 24 | ...`
  expression, which does not preserve the bit positions the listing uses, and
  the symbol tables at `+0x0C`/`+0x80C` are consumed by a second tree walk with
  its own byte-at-a-time bit reader. Reconstructing the header tag and the two
  table layouts from the export would mean inventing the bit order, so it is
  left out.
- `0x801DEB70` (2524 bytes) - the per-frame Wi-Fi reconnect/update tick. The
  two slot loops are readable, but the idle path at `0x801DF348` builds a
  twelve-word request descriptor on the stack and assigns only five of its
  words (`0x10`/`0x10`/low-24-bit field/one half-word), copying the rest from
  uninitialized stack slots. The export also prints the hidden `r4` half of
  `0x802DCEC4` as `extraout_r4` in both timeout branches. The descriptor
  field order and the timeout value cannot be recovered from the export alone,
  so it is left out.

Both skips are recorded here rather than padded with a guess.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (4 rows, 3904 bytes, 0 high,
  4 medium, 0 low).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`; all four were the bucket's
  remaining untranslated set.
- `node tools/check-lane-csv.mjs wave109` reports no problem for
  `ledger/agents/wave109-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs` (the parallel lanes' errors are in their own files).
