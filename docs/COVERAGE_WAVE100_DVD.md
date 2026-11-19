# Coverage - Wave 100 (runtime::dvd, the `0x801C0000` page: matrix/quaternion maths, GX draw state and bring-up)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801C0000` 64 KiB page of `main.dol`,
which is wider than the disc band (`0x801CA640`-`0x801D6780`). Everything below is
SDK code that happens to share the page, and `src/runtime/dvd.rs` is the only
writable file assigned to the bucket, so it is reconstructed there. Each body in
that file states which routine it is, which fields it touches and how confident
the reconstruction is; this document only summarises the pass.

`high` means every store, mask and call in the body is literal in the export and
its role is proven; `medium` means the instruction sequence is literal but at
least one field role, callee identity or recovered argument is inferred; `low`
means only the store/instruction sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 24 | 9084 | 2 | 21 | 1 |

## What this pass covers

The bucket still held 37 untranslated addresses. This pass takes 24 of them in
four groups spread over `0x801C0020`-`0x801C99C0`; none is disc code.

- **Matrix and quaternion library, `0x801C0540`-`0x801C1850` (6 bodies, 1700
  bytes).** This finishes the slice Waves 90/95 started:
  [`fn_801c0540`] is the 3x4 concatenation (the export's paired stores leave in
  the order `(0,1) (4,5) (2,3) (6,7) (8,9) (10,11)`, and the two `.sdata2`
  words at `0x804295A0`/`0x804295A4` scale the first matrix' translation
  column); [`fn_801c07a0`] and [`fn_801c08a0`] are the affine and 3x3 inverses
  (column-0 cofactors build the determinant, then one Newton step
  `2*i - i*i*det` refines the reciprocal); [`fn_801c0aa0`] builds the axis-angle
  rotation matrix with the same Newton step on the reciprocal square root;
  [`fn_801c1680`] extracts a quaternion through the largest-diagonal branch
  (the three-word index table at `0x802F6AD0` is read, not assumed) and
  [`fn_801c1850`] is the slerp, with the linear blend above `r2-0x4EFC`.
- **Video-mode and bring-up, `0x801C0020`-`0x801C1DB0` (2 bodies, 2480
  bytes).** [`fn_801c0020`] is the linear twelve-block video command sequence
  (`0x801BE860(0xE0, ...)` + `0x801BE480(2)`), built in one stack frame whose
  `local_XX` slots map to `frame[0x68 - XX]`; [`fn_801c1db0`] is the GX
  bring-up: it installs the four register windows (`0xCC000000`, `0xCC001000`,
  `0xCC003000`, `0xCC004000`), initialises the channel/texcoord/indirect
  tables and ends with the `0x69000400` clock word.
- **Draw state and FIFO payloads, `0x801C1B10`-`0x801C99C0` (10 bodies, 3024
  bytes).** [`fn_801c1b10`] is the retrace latch plus its teardown branch,
  [`fn_801c1c80`] the channel-field mask writer, [`fn_801c2b80`] the
  pending-bit service that reaches the completion callback, [`fn_801c2e70`] and
  [`fn_801c3050`] the GP and CP token publishers, [`fn_801c78d0`] the TEV-stage
  refresh, [`fn_801c8cb0`] the literal `0xE8` emitter and
  [`fn_801c97f0`]/[`fn_801c99c0`]/[`fn_801c9940`] the three FIFO float
  emitters.
- **Texture, fog and depth helpers, `0x801C19F0`-`0x801C8B50` (6 bodies, 1880
  bytes).** [`fn_801c19f0`] selects a texture block by index modulo eight,
  [`fn_801c6fa0`] is the post-init store that clears the tile-size flag,
  [`fn_801c6470`] is the seven-arm fog selector, [`fn_801c6760`] writes a
  normalised direction plus a scaled point, [`fn_801c8b50`] builds ten 12-bit
  depth words and [`fn_801c5bc0`] is the 9-bit scale search.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C0020` | `fn_801c0020` | 1196 | medium |
| `0x801C0540` | `fn_801c0540` | 204 | medium |
| `0x801C07A0` | `fn_801c07a0` | 248 | medium |
| `0x801C08A0` | `fn_801c08a0` | 200 | medium |
| `0x801C0AA0` | `fn_801c0aa0` | 176 | medium |
| `0x801C1680` | `fn_801c1680` | 460 | medium |
| `0x801C1850` | `fn_801c1850` | 412 | medium |
| `0x801C19F0` | `fn_801c19f0` | 228 | medium |
| `0x801C1B10` | `fn_801c1b10` | 368 | medium |
| `0x801C1C80` | `fn_801c1c80` | 304 | high |
| `0x801C1DB0` | `fn_801c1db0` | 1284 | medium |
| `0x801C2B80` | `fn_801c2b80` | 352 | medium |
| `0x801C2E70` | `fn_801c2e70` | 480 | medium |
| `0x801C3050` | `fn_801c3050` | 656 | medium |
| `0x801C5BC0` | `fn_801c5bc0` | 548 | medium |
| `0x801C6470` | `fn_801c6470` | 412 | medium |
| `0x801C6760` | `fn_801c6760` | 268 | medium |
| `0x801C6FA0` | `fn_801c6fa0` | 72 | medium |
| `0x801C78D0` | `fn_801c78d0` | 356 | medium |
| `0x801C8B50` | `fn_801c8b50` | 352 | medium |
| `0x801C8CB0` | `fn_801c8cb0` | 272 | high |
| `0x801C97F0` | `fn_801c97f0` | 68 | medium |
| `0x801C9940` | `fn_801c9940` | 80 | low |
| `0x801C99C0` | `fn_801c99c0` | 88 | medium |

## Notes on the recovered arguments and stores

- **GQR0 arms.** As in the Wave 90/95 sections, the paired-single access arms
  (`4`/`5`/`6`/`7` of GQR0) are not reproduced; every body here is the plain
  `f32` arm and says so in `# Confidence`. This is why the matrix, fog and
  quaternion bodies are `medium` even where every term is literal.
- **`0x802DCD6C`/`0x802DCD70`/`0x802DCD74`.** These prologue fragments appear
  again as fake 64-bit returns. Per the Wave 91/98 note, the high word is the
  first incoming argument and the low word the second; the bodies take them as
  ordinary parameters (`fn_801c1680`, `fn_801c2e70`, `fn_801c3050`,
  `fn_801c1db0`) and never call the helper.
- **FIFO port aliases.** The export splits the write-gather port into
  `_DAT_cc008000`/`_DAT_cc008002`/`_DAT_cc008004`. The three float emitters
  ([`fn_801c97f0`], [`fn_801c99c0`], [`fn_801c9940`]) keep those addresses and
  widths because the payload-before-opcode order and the 16-bit stores are part
  of the observed sequence.
- **`0x43300000` idioms.** [`fn_801c8b50`] and [`fn_801c5bc0`] show
  `CONCAT44(0x43300000, x) - r2-0x4Exx`: that pair is the compiler's
  `u32 -> f64` conversion (the SDA slot holds `2^52`), so the bodies use plain
  `as f64`/`as f32` conversions and say so.
- **`fn_801c9940` is `low`.** Its 16-bit stores carry `SUB42(float, 0)` - the
  high half of the float's bit image - which is how Ghidra renders a quantised
  attribute store. The store addresses, widths and order are literal; the value
  semantics are not, so the row is `low` rather than `medium`.

## Deliberate skips

The remaining 13 addresses in the bucket were not claimed:

- `0x801C0610` (392): the concat-with-count. Its loop advances the destination
  by `0x30` bytes per iteration while writing `0x60` bytes, so the destination
  element mapping is not recoverable from the export.
- `0x801C0970` (124) and `0x801C0B50` (124): wrappers over
  `0x801C09F0`/`0x801C0AA0` whose first argument arrives as a paired-single
  pair and whose sine call has no recovered operand.
- `0x801C09F0` (176): the X/Y/Z rotation selector. Its Y arm stores the
  `r2-0x4F4C` word where the identity term belongs, so the export is internally
  inconsistent and the body would be a guess.
- `0x801C22C0` (2240) and `0x801C9E20` (2076): the large table-driven XF/CP
  register emitters (dozens of literal command words each). Not attempted this
  pass; they need a dedicated transcription run.
- `0x801C5570` (332) and `0x801C56C0` (216): the two FIFO padding writers whose
  final store merges the written half-word with the write-only FIFO's previous
  content, which cannot be read back. Same reason Wave 80 left them.
- `0x801C5F40` (484) and `0x801C7A40` (356): both decompile to four stores
  while `functions.tsv` gives 484 and 356 bytes respectively, so a body would be
  a silent `partial` with no view of the missing code.
- `0x801C8920` (556): the `0x802DCC4C` conversion result is discarded in the
  export and the two loops compare SDA doubles that are never initialised in the
  body.
- `0x801C90A0` (976): the GX mesh/EFB setup takes sixteen `f64` arguments
  through the register file (`in_f19`-`in_f31`); the argument list is not
  recoverable.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (24 rows, 9084 bytes, 2 high,
  21 medium, 1 low).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave100` reports `OK` for
  `ledger/agents/wave100-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
