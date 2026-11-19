# Coverage - Wave 98 (runtime::dvd, the `0x801D0000` page: range decoder and the ISFS wrappers)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801D0000` 64 KiB page of `main.dol`,
which is wider than the disc band (`0x801CA640`-`0x801D6780`). Everything below
is SDK code that happens to share the page, and `src/runtime/dvd.rs` is the only
writable file assigned to the bucket, so it is reconstructed there. Each section
of the Rust file states plainly which routine it is, which fields it touches and
how confident the reconstruction is; this document only summarises the pass.

The `Functions` table lists the address, the Rust symbol, the byte size from
`work/ghidra/main-gb/export/functions.tsv` and the confidence recorded in the
body's `# Confidence` block. `high` means every store, mask and call in the body
is literal in the export and its role is proven; `medium` means the instruction
sequence is literal but at least one field role, callee identity or recovered
argument is inferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 25 | 8000 | 7 | 18 |

## What this pass covers

The bucket still held 47 untranslated addresses. This pass takes 25 of them in
three groups spread over `0x801D7760`-`0x801DAB90`; none is disc code.

- **Range decoder, `0x801D7760`-`0x801D82E0` (6 bodies, 2996 bytes).**
  A probability model is a 0x10-byte record `{freq: *u32, cum: *u32, total: u32,
  bits: u8}` and the coder state is the triple `{low, range, code}`. The
  evidence fixes both layouts without guessing: [`fn_801d7d50`] passes exactly
  `state + 0x0C`/`+0x80C`/`+0x100C`/`+0x500C` as the two models' arrays and
  `+0x9014`/`+0x9018`/`+0x901C` as the coder words, while [`fn_801d7b00`]
  initialises them. [`fn_801d7760`] credits one
  symbol and rebuilds the cumulative array every time the total reaches
  `0x10000`; [`fn_801d7950`] is the bounded binary search plus the linear
  correction loop; [`fn_801d79f0`] narrows the interval and renormalises eight
  bits at a time, returning a negative byte count when the input runs dry;
  [`fn_801d7d50`] is the header prefill plus the literal/match byte loop, with
  the four negative return codes `-1`, `-3`, `-4`; [`fn_801d82e0`] validates the
  distance bit-tree against a 0x80-byte visited bitmap.
- **ISFS wrappers, `0x801D84E0`-`0x801DAB90` (13 bodies, 3360 bytes).** The
  attribute/delete/rename/open twins that Wave 91 left out. Every body builds
  the full path with [`fn_801da6e0`], guards on [`isfs_ready`], and forwards to
  one of the `0x80226xxx`/`0x80227xxx` cores; the sync/async pairs are literal
  in the exports (`0x80227040`/`0x80227150` for rename, `0x80226BF0`/`0x80226D50`
  for attributes, `0x80226940`/`0x80226AA0` for open). The `0x802DCD74` prologue
  pairs in [`fn_801d8a60`]-[`fn_801d8d20`] and [`fn_801dab90`] are recovered as
  their two visible parameters, per the note above the Wave 91 section.
- **Close state machines and path helpers, `0x801D9D10`-`0x801DA7B0` (6 bodies,
  1644 bytes).** The two state chains walk `+0x89` down through `7 -> 0`,
  `6 -> 2`, `5 -> 4`, `4 -> 2` and `2 -> 0` around the close and delete cores;
  the four helpers copy the directory part, split at the first slash, take the
  last component, and default to the small-data root.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D7760` | `fn_801d7760` | 488 | high |
| `0x801D7950` | `fn_801d7950` | 148 | medium |
| `0x801D79F0` | `fn_801d79f0` | 272 | medium |
| `0x801D7B00` | `fn_801d7b00` | 580 | high |
| `0x801D7D50` | `fn_801d7d50` | 1008 | medium |
| `0x801D82E0` | `fn_801d82e0` | 500 | medium |
| `0x801D84E0` | `fn_801d84e0` | 264 | medium |
| `0x801D85F0` | `fn_801d85f0` | 240 | medium |
| `0x801D86E0` | `fn_801d86e0` | 256 | medium |
| `0x801D87E0` | `fn_801d87e0` | 176 | medium |
| `0x801D8890` | `fn_801d8890` | 152 | high |
| `0x801D8A60` | `fn_801d8a60` | 120 | medium |
| `0x801D8B50` | `fn_801d8b50` | 120 | medium |
| `0x801D8C70` | `fn_801d8c70` | 168 | medium |
| `0x801D8D20` | `fn_801d8d20` | 256 | medium |
| `0x801D8E20` | `fn_801d8e20` | 424 | medium |
| `0x801D91C0` | `fn_801d91c0` | 340 | medium |
| `0x801D97D0` | `fn_801d97d0` | 120 | high |
| `0x801D9D10` | `fn_801d9d10` | 476 | high |
| `0x801D9F60` | `fn_801d9f60` | 428 | high |
| `0x801DA110` | `fn_801da110` | 212 | high |
| `0x801DA1F0` | `fn_801da1f0` | 212 | medium |
| `0x801DA2D0` | `fn_801da2d0` | 152 | medium |
| `0x801DA7B0` | `fn_801da7b0` | 164 | medium |
| `0x801DAB90` | `fn_801dab90` | 724 | medium |

## Notes on the recovered arguments

- **`0x802DCD74`/`0x802DCDC0`.** These are the Metrowerks callee-save/restore
  fragments, not calls. Ghidra prints their `r3:r4` pair as a 64-bit return; on
  this big-endian target the high word is the first incoming argument and the
  low word the second. Seven bodies here (`0x801D86E0`, `0x801D8A60`,
  `0x801D8B50`, `0x801D8C70`, `0x801D8D20`, `0x801DA1F0`, `0x801DAB90`) take
  those words as ordinary parameters and never call the helper.
- **Dropped mapper arguments.** Four bodies (`0x801D8890`, `0x801D8A60`,
  `0x801D8B50`, `0x801D8C70`, plus the state-machine branches) show
  `FUN_801da570()` with no operand. On those paths `r3` still holds the
  preceding call result, which is what the body passes.
- **Small-data strings.** `r13 - 0x5FF4`, `-0x6010` and `-0x600C` are unnamed
  text in the exports. They are declared as [`SDA_ISFS_ROOT`],
  [`SDA_ISFS_RENAME_PREFIX`] and [`SDA_ISFS_RENAME_FORMAT`] and marked
  `// unknown:` at the declaration; only their use is transcribed.
- **`FUN_801d79f0`'s return type.** The export declares `void`, but the caller
  compares its result with `0x100` and the value leaves in `r3` through the
  `0x802DCDB8` epilogue, so the body returns `u32` and says so.

## Deliberate skips

The remaining 22 addresses in the bucket were not claimed:

- `0x801D68C0` (660), `0x801D6B60` (940), `0x801D6F60` (2036): the three
  byte-stream/RLE decoder variants of the NAND reader. Their bit-prefill loops
  are readable but long, and the three differ only in tag nibbles and match
  modes; left for a dedicated pass rather than half-translated.
- `0x801D9950` (948): the NAND `open` core with the full mode state machine.
  Not attempted; the surrounding wrappers are translated and call it by
  absolute address.
- `0x801DA990` (204): a tick-based wait loop that packs `FUN_801B5F80` into a
  64-bit divide and compares it against `0x800000F8 / 4000`. The unit of the
  low-memory divisor is not established, so the arithmetic is not reproduced.
- `0x801DA370` (316): the recursive relative-path resolver. It formats through
  three unnamed small-data strings and recurses on a directory component; it is
  called by address from [`fn_801dab90`] instead of guessed at.
- `0x801DB2B0` (248) and `0x801DB3B0` (1156): the formatted error-log writer and
  its main body. Both take `va_list` through the calling convention, which the
  exports reconstruct as sixteen `undefined8` parameters, so the real argument
  list is not recoverable.
- `0x801DBEB0` (584), `0x801DC280` (404), `0x801DC420` (632), `0x801DCC70`
  (540), `0x801DCE90` (88): the SYSCONF staging/commit group. `0x801DCC70` and
  `0x801DCE90` read a second global set at `0x803962C4`-`0x803962CC` whose role
  conflicts with the `0x8039629C`-`0x803962BC` set the already-translated
  `0x801DBD70` uses (state, completion, result, chunk size), and the `0x801DBExx`
  pair manipulates 16-bit list entries with negative offsets. No honest mapping
  yet.
- `0x801DE010` (2296) and `0x801DEB70` (2524): the two large settings-store
  routines. Not attempted.
- `0x801DFAC0`, `0x801DFAD0`, `0x801DFAF0`, `0x801DFB00`, `0x801DFB10`,
  `0x801DFB20`, `0x801DFB30` (4 bytes each): plain branch trampolines into
  `0x801F####`. A tail-call with the register file intact cannot be expressed
  with `platform::call0`-`call7`, so no body is claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (25 rows, 8000 bytes, 7 high and
  18 medium).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave98` prints `OK` and lists
  `wave98-dvd.csv: 25 rows`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
