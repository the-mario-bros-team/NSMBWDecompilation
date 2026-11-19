# Coverage - Wave 107 (`runtime::os`, `0x80250000` bucket)

## How to read this file

This page records wave 107 of the `main.dol` reconstruction for the
`runtime::os` lane, run on the `0x80250000` bucket. The bucket holds 163
inventory entries; 154 were already merged when this pass started and 9 were
open. Eight of those 9 are reconstructed here (`0x80250E70`..`0x8025CA20`); the
ninth is listed under [Deliberate skips](#deliberate-skips).

- **Status** - every row is `translated`: each guard, store and call printed in
  the Ghidra export has a counterpart in `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. Four rows are
  `high`, three `medium` (one field role or one dropped argument is
  `// inferred:`/`// unknown:`), one is `low` (`0x80251930`, whose callees are
  printed without arguments).
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave107-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 8 | 11,720 | 4 | 3 |

Status split: 8 `translated`, 0 `partial`, 0 `stub`; 1 of the 8 is `low`.

## What this pass covers

- **Cluster A - the node-tree binding walk (`0x80250E70`).** Walks a `0x05`-byte
  command stream over the per-node matrix (`0x30` stride), flag-word and
  three-word vector arrays. Tag `0x02` resolves the command's `u16` key through
  `0x8023B300` and rebuilds the destination entry with one of the three
  vertex-format kernels (`0x80254A70`, `0x80255750`, `0x80256350`), selected by
  the "count == 1" test and by bit `0x400` of the material record; any other tag
  copies one entry over another. A second pass translates every destination by
  its vector through `0x802381A0` unless the vector still holds the
  `sda2(-0x49A8)` seed. The optional hook record gets three callbacks
  (vtable slots `+0x0C`, `+0x10`, `+0x14`) gated by bits 0, 1 and 2 of its `+4`
  byte.
- **Cluster B - the matrix blender (`0x80251440`).** The matrix twin of cluster
  A. Tag `0x03` blends up to four sources into one destination matrix: each
  source is a `u16` matrix index plus an `f32` weight, the destination flag word
  becomes the AND of the source flag words, and the twelve floats are accumulated
  pairwise. The other tag multiplies through `0x801C0540`. A trailing mode word
  (`root[+0x6F]`) then ORs `0x30000000` or ANDs `0xDFFFFFFF` into the flag words
  of the last `count - first` instances.
- **Cluster C - the instance emitter (`0x80251930`).** Reads three channel words
  from `instance + 0x1C` and emits each live channel through the `0x8024D210`
  list object. The first channel builds the shared matrix (default
  `0x8024D140` row, or `0x8024D140(selector)` multiplied by the `0x8023B2A0`
  record, or the `0x8024D4D0` matrix), zeroes the three `w` elements and
  normalises the three basis columns. Each channel's offset vector comes from
  `0x8025F690`/`0x8025F6B0` under bits 1/2 of the descriptor flags, with the
  `sda2(-0x499C)` gain and `sda2(-0x4998)` bias applied to the third component.
- **Cluster D - the block layout builder (`0x8025CA20`).** Computes one aligned
  block for a whole node tree from a flag mask: eleven counted arrays plus three
  packed sub-record arrays whose sizes are the sums of the aligned entries
  `0x8023B400`/`0x8023B500`/`0x8023B600` return. Then one `0x801D5540`
  allocation, the per-entry variant resolution (`0x8023C100`, `0x8023C150`,
  `0x8023C1B0`), the `0x8025C030` construction of the `0x8034E8F0` object, the
  `0x4E..0x61` field block and `0x8025E060`.
- **Cluster E - the two MSL `qsort` pairs (`0x802529F0`/`0x80252FB0` and
  `0x80259760`/`0x80259AE0`).** The sorters behind the four range-sort wrappers
  `0x80252710`, `0x80259680`, `0x80259F60` and `0x8025A0E0`. All four are the
  same `qsort`: insertion sort below 21 elements, a two-element shuffle picked
  by a rotating `sda` counter, a Hoare partition around the last element, then
  recurse into the smaller half and loop on the larger. `0x80252FB0` and
  `0x80259AE0` own the loop (comparator address behind a pointer); `0x802529F0`
  and `0x80259760` keep the comparator in a stack word and forward, exactly as
  their exports do.

## What the pass adds beyond the raw bodies

1. **The `0x802DCDxx` half order is now proved the same way three times.** The
   `0x802DCD40` (heap/size-out), `0x802DCD6C`/`0x802DCD70` (begin/end) and
   `0x802DCD60` (instance/secondary) pairs all put the first register argument
   in the **high** half of `call_u64`'s value. `0x80259AE0` proves it
   arithmetically (it subtracts the pair halves to get a positive element
   count), and the call sites in `0x80259680`/`0x80259F60` prove it textually
   (`(base, base + count * 4)`).
2. **The two `qsort` specialisations are one algorithm, not four.** Recording
   that `0x80259760` forwards into `0x80259AE0` (and `0x802529F0` into
   `0x80252FB0`) keeps a single transcription of a 90-line loop instead of four,
   which is what the four exports themselves do.
3. **The command streams are now enumerated.** Cluster A/B show that the same
   node-tree record drives both a vector/matrix bind (`0x05`-byte commands) and
   a matrix blend (`0x04`/`0x05`/`0x0C` commands), both keyed by the same
   `0x8023B300` node lookup, which pins the `+0x10`/`+0x18`/`+0x0C` roles of the
   node record.
4. **The layout chain of `0x8025CA20` is written down.** Eleven counted arrays
   and three packed arrays, each with its own alignment rule, in the order the
   object's `0x148..0x180` pointer block stores them. That block is what
   `0x8025E060` then fills.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80250E70` | `fn_80250e70` | 1,476 | medium |
| `0x80251440` | `fn_80251440` | 1,256 | medium |
| `0x80251930` | `fn_80251930` | 1,308 | low |
| `0x802529F0` | `fn_802529f0` | 1,468 | high |
| `0x80252FB0` | `fn_80252fb0` | 1,468 | high |
| `0x80259760` | `fn_80259760` | 896 | high |
| `0x80259AE0` | `fn_80259ae0` | 896 | high |
| `0x8025CA20` | `fn_8025ca20` | 2,952 | medium |

## Deliberate skips

One of the 9 open addresses was read and left alone:

1. **`0x8025FE30` (1,940).** The skinning pass. For each entry of
   `0x8023B460()` it resolves a record through `0x80249720` and then runs three
   weighted vertex blends (bit 1 from `record[+8]`, bit 4 from the `param_4`
   table, bit 8 from the `param_5` table). The export cannot be turned into a
   signature: it reads an undeclared `extraout_r4` as the root argument, calls
   `FUN_8023b460()` with **no** arguments where every other call site passes the
   root, and reads three undeclared incoming `f29`/`f30`/`f31` lane pairs. The
   whole register file is the argument list, which is the same situation Wave
   104 rejected for `0x8024E180`. The three blends, the `0x8025F690`/`0x8025F6B0`
   offset pair and the GQR scale/round arms are all visible, so a later pass
   with the instruction stream can finish it without touching this lane.

No address in this pass was already present in `ledger/functions.csv`; the 154
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (8 of 8; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build diagnostic belongs to another lane
  (`src/game/player.rs`, a `worldmap` symbol).
- `node tools/check-lane-csv.mjs wave107` prints `OK`, with `wave107-os.csv`
  counted at 8 rows.
