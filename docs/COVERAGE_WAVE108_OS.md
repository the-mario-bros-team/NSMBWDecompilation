# Coverage - Wave 108 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records wave 108 of the `main.dol` reconstruction for the
`runtime::os` lane, run on the `0x80240000` bucket. The bucket holds 137
inventory entries; 130 were already merged when this pass started and 7 were
open. Four of those 7 are reconstructed here
(`0x80240100`..`0x8024FEF0`); the other three are listed under
[Deliberate skips](#deliberate-skips).

- **Status** - every row is `translated`: each guard, store, call and arithmetic
  expression printed in the Ghidra export has a counterpart in
  `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. Three rows are
  `medium` (a field role, a dropped argument or the `psq_l` element type is
  `// inferred:`), one is `low` (`0x802405B0`, where the incoming float argument
  and the epilogue `r3` are not declared by the export).
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave108-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 4 | 4,176 | 0 | 3 |

Status split: 4 `translated`, 0 `partial`, 0 `stub`; 1 of the 4 is `low`.

## What this pass covers

- **The GQR question is answered from the boot chain.** Waves 96, 100 and 104
  left the three curve readers open because every key/value access in their
  exports is expanded into three `GQR` arms. `init_hardware_paired_single`
  (`0x801AB150`, translated in `src/runtime/init.rs`) clears `GQR0`..`GQR7`
  after enabling paired-single mode, and no other translated code writes a
  `GQR`. The live arm is therefore "signed 16-bit, zero fraction" for the
  halfword fields, which makes the index hint a plain
  `(u16)(rate * (time - start) * count)` conversion. This is the same reading
  the lane already uses for `0x8023F000` ("the target also reads a GQR word in
  `r4`, which every caller leaves at its startup value").
- **`0x80240100` is a twin of `0x8023F000`.** The two exports are the same
  308-byte routine: `u16 count`, `f32 rate` at `+4`, 12-byte keys
  `{f32 time, f32 value, f32 tangent}` at `+8`, an index hint from the rate,
  two bracket-search loops, and the Hermite expression
  `value + u^2*(3u-2)*(value-next) + (time-key)*(u-1)*((u-1)*tan + u*next_tan)`.
  Only the three small-data slots moved (`-0x4AE4/-0x4AE0/-0x4ADC` here versus
  `-0x4B28/-0x4B24/-0x4B20` there). `resource::bres::fn_8023f000` is the
  independently reviewed transcription, so this body has no guessed field.
- **`0x80240240` is the 16-bit variant of the same algorithm.** Header:
  `u16 count`, pad, `f32 rate` at `+4`, `f32 scale` at `+8`, `f32 bias` at
  `+12`. Keys are 6 bytes `{s16 time, s16 value, s16 tangent}`. The equality
  arm decodes the value at `key+2`; the interpolation arm decodes value/tangent
  at `key+2`/`key+4` and the next pair at `key+8`/`key+10`, all as
  `scale * raw + bias`. That the two `+4` reads are the tangents is proved by
  their only use, the `(u-1)*tangent + u*next_tangent` term.
- **`0x8024FEF0` is the per-instance palette builder.** The `0x802DCD40` entry
  it calls is a callee-saved-register save thunk, so the two "unused" leading
  arguments are really the caller's `r3`/`r4`: a `count * 0x30` composed-matrix
  array and a `count * 0x24` normal-matrix array. The body chains the source
  matrices through `0x801C0540`/`0x801C0610` into the first array, then per
  element either multiplies the node's `0x30`-byte scratch matrix in (flag low
  byte `0`) or dispatches `0x802F8ED0[flag & 0xFF]` and multiplies in the
  inverse of the element matrix (`0x80242A40`). A second loop writes the
  `0x24` normal matrices: the 9-float inverse `0x80242840` when bit
  `0x20000000` is clear, the packed linear part `0x802380D0` when it is set.
  The closing `0x801AC4D0` calls are `dc_flush_range_nosync`, not frees.
- **`0x802405B0` is the three-channel animation evaluator.** Bits 27..29 of
  `record+4` select raw floats, the three keyed-track decoders, or a
  byte/halfword/float lookup table; bits `0x10000`/`0x20000`/`0x40000` mark a
  channel whose source word is already a value. The three results are scaled by
  `sda2(-0x4AD8)`, passed to the `0x802382B0` orientation kernel (the same
  trampoline Wave 92 already uses) and stored at the first argument. This also
  fixes the argument shape the Wave 104 helpers only modelled as inert
  registers: `out`, `matrix`, `curves`, `record`, `values`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80240100` | `fn_80240100` | 308 | medium |
| `0x80240240` | `fn_80240240` | 380 | medium |
| `0x802405B0` | `fn_802405b0` | 2,660 | low |
| `0x8024FEF0` | `fn_8024fef0` | 828 | medium |

## Deliberate skips

Three addresses were read again and left alone:

1. **`0x802403C0` (492).** The 8-bit curve reader. Its search compares a byte
   at entry `+0`, but its value is re-extracted as bits 12..23 of the same
   32-bit word and its sign as bits 0..3 (`(x << 20) >> 28`). Those three
   extractions cannot describe one entry layout at once; Ghidra is merging
   several `psq_l` lanes of a packed 4-byte key. Writing one interpretation
   would be a guess. `0x802405B0` calls it by address, so the missing body does
   not hold up the evaluator.
2. **`0x802471E0` (428).** The matrix-slot quantiser. Its incoming `f1`/`ps1`
   pair (`in_f31`/`in_ps31_1`) is spilled to the last two words of the stack
   buffer and never read again, while the buffer itself is re-used as the
   argument of `0x80250D40` (identity matrix), `0x8023DAA0` (packed matrix) and
   `0x80247140` (record copy). The maximum-absorber reads offsets
   `0/4/0xC/0x10/0x14/0x1C`, but `0x8023DAA0` encodes offsets
   `0/4/8/0x10/0x14/0x18`; the two disagree, so the local matrix layout cannot
   be settled from the C alone.
3. **`0x8024E180` (2,212).** The vertex/record emitter. It reads eight incoming
   `f23`..`f31` doubles and eight `ps23_1`..`ps31_1` lanes; the register file is
   the whole argument list and Ghidra prints the function as `void`. This is
   the same situation Wave 104 rejected, and there is still no honest
   signature to write.

No address in this pass was already present in `ledger/functions.csv`; the 130
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (4 of 4; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build errors belong to other lanes
  (`src/ui/menu.rs`, `src/runtime/bt.rs`).
- `node tools/check-lane-csv.mjs wave108` prints `OK`, with `wave108-os.csv`
  counted at 4 rows.
