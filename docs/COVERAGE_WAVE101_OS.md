# Coverage - Wave 101 (`runtime::os`, `0x80250000` bucket)

## How to read this file

This page records wave 101 of the `main.dol` reconstruction for the
`runtime::os` lane: the per-instance record layer of the `0x80250000` bucket.
The bucket holds 163 inventory entries; 129 were already merged when this pass
started, 34 were open, and 25 of those are reconstructed here.

- **Status** - every row is `translated`: each guard, store and call printed in
  the Ghidra export has a counterpart in `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. `low` marks the
  bodies whose `0x802DCD40`..`0x802DCD74` handoff halves are not declared by
  the export; their roles are inferred from the sizes allocated to them or
  from how the caller consumes them.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave101-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 25 | 18,820 | 1 | 21 |

Status split: 25 `translated`, 0 `partial`, 0 `stub`. The remaining three rows
are `low`: `0x80250230`, `0x80250710` and `0x802525A0` (plus `0x80252710`,
which shares `0x802525A0`'s handoff). All four are still complete translations;
the low mark only records the inferred handoff halves.

## What this pass covers

- **Cluster A - the tagged-code gather, `0x802525A0`/`0x80252710`.** The code
  list is an array of eight-byte tags (`0x01` terminates); `0x802525A0`
  resolves each `+0x05` code through `0x8023B2A0`, tests the entry's visibility
  (either its own `+0x14` bit 8 or the caller's `+0x04` mask indexed by
  `+0x0C`), and writes a twelve-byte record `{scale, byte7, code, tag+1/tag+2,
  tag+3..4}`. `0x80252710` is the driver: it runs the cache round-trip, picks
  the flat (`0x80252020`/`0x80252160`) or sorted-record
  (`0x802525A0` + `0x802529F0` + `0x80252330`/`0x80252440`) path per list, and
  closes with `0x802DCDBC`.
- **Cluster B - the two comparator kernels, `0x80253570`/`0x80259E60`.** Both
  are the same Bentley-McIlroy compare ladder: `cmp(left, high) == 0` means
  "not smaller", so the kernel conditionally exchanges `high`/`low` and then
  `record` with one of them. The first moves three-word records, the second
  single words.
- **Cluster C - the event fan-out, `0x80257A40`-`0x8025B500`.** The node's
  `+0xD4` child receives the event before and after the node's own handler;
  the `+0xD8` bits choose which of the child's `+0x0C`, `+0x10` and `+0x14`
  slots run. `0x80257FC0` also forwards the event to every node in the
  `+0xDC` child array. `0x8025B500` is the five-class attach ladder: each class
  descriptor is type-tested through the child's `vtable+0x08`, then
  `0x80249C90`, then installed into `+0x124`..`+0x134`.
- **Cluster D - the activation layer, `0x8025C030`-`0x8025EC70`.** The
  constructor at `0x8025C030` initialises the vtable `0x8034E8A8`, the four
  named sub-objects and the three buffers. `0x8025C660` gathers the thirteen
  words of one activation record from the eleven tables at
  `+0x14C`..`+0x174`. `0x8025E400` installs the selected bit attributes,
  `0x8025E060` rebuilds them all, `0x8025D5B0` flushes the pending bits in the
  export's order, and `0x8025E8B0`/`0x8025EC70` detach and re-upload.
- **Cluster E - the palette and draw paths, `0x80250230`-`0x8025FC30`.**
  `0x802544F0` rebuilds a 3x4 matrix from three half-angles (or from two axis
  vectors when bit 2 of `+0x70` is set), and `0x80258CE0`/`0x8025F6D0` fill the
  `0x30`-byte matrix and `0x44`-byte palette records. `0x80250230` and
  `0x80250710` are the chunked draw emitters: they allocate the three vertex
  rings, walk `0x28` entries at a time, dispatch each format code through
  `0x802F8ED0[code]`, optionally invert the matrix through `0x80242A40`, and
  publish the ring slices with `0x801AC740`.

## What the pass adds beyond the raw bodies

1. **The node layout is now explicit.** `+0xD4` is the child handler, `+0xD8`
   the per-event mask, `+0xDA` the event-class mask, `+0xDC`/`+0xE4` the child
   array and its count, `+0xE8` the source list, `+0xEC` the matrix buffer,
   `+0x104` the activation flags, and `+0x124`..`+0x134` the five attached
   children.
2. **The eleven activation tables are named by offset and stride.** `+0x14C`
   is `0x104`-strided, `+0x150` `0x64`, `+0x154` `0x248`, `+0x158` `0x28`,
   `+0x15C` `8`, `+0x160` `0x0C`, `+0x164` `0x20`, `+0x168` `0x80`, `+0x16C`
   `0x40`, `+0x170` `0xA0` and `+0x174` `0x200`. Every installer in this pass
   uses the same list, which is why `0x8025E060` and `0x8025E400` are nearly
   interchangeable.
3. **The `0x802DCDxx` handoffs are still register saves.** `0x802DCD40`,
   `0x802DCD48`, `0x802DCD5C`, `0x802DCD60`, `0x802DCD64` and `0x802DCD70`
   are all prologue/epilogue saves, so the pair a caller receives is whichever
   live registers the compiler happened to leave there. The bodies read the
   pair with [`wave100_pair`]/[`wave97_pair`] and mark each inferred role.
4. **The `GQR` arms are unreachable again.** Every float body in the bucket
   came out of Ghidra with `ldexpf(GQR >> 8 & 0x3f)` scale blocks and
   `psq_st`-shaped byte/halfword stores. As in waves 92/96/97/100, the bodies
   keep single-precision arithmetic and `write_f32`/`write_u8`; the GQR arms
   are the decompiler admitting it cannot prove the quantisation registers.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80250230` | `fn_80250230` | 1,244 | low |
| `0x80250710` | `fn_80250710` | 1,436 | low |
| `0x802525A0` | `fn_802525a0` | 356 | low |
| `0x80252710` | `fn_80252710` | 552 | low |
| `0x80253570` | `fn_80253570` | 608 | medium |
| `0x80253ED0` | `fn_80253ed0` | 476 | medium |
| `0x802544F0` | `fn_802544f0` | 1,052 | medium |
| `0x80257A40` | `fn_80257a40` | 732 | medium |
| `0x80257FC0` | `fn_80257fc0` | 1,120 | medium |
| `0x80258CE0` | `fn_80258ce0` | 552 | medium |
| `0x80258F10` | `fn_80258f10` | 392 | medium |
| `0x80259E60` | `fn_80259e60` | 256 | medium |
| `0x8025A520` | `fn_8025a520` | 600 | medium |
| `0x8025A7C0` | `fn_8025a7c0` | 844 | medium |
| `0x8025B500` | `fn_8025b500` | 1,552 | medium |
| `0x8025C030` | `fn_8025c030` | 536 | medium |
| `0x8025C660` | `fn_8025c660` | 640 | high |
| `0x8025D5B0` | `fn_8025d5b0` | 1,404 | medium |
| `0x8025E060` | `fn_8025e060` | 924 | medium |
| `0x8025E400` | `fn_8025e400` | 820 | medium |
| `0x8025E8B0` | `fn_8025e8b0` | 504 | medium |
| `0x8025EC70` | `fn_8025ec70` | 564 | medium |
| `0x8025F6D0` | `fn_8025f6d0` | 1,020 | medium |
| `0x8025FAD0` | `fn_8025fad0` | 348 | medium |
| `0x8025FC30` | `fn_8025fc30` | 288 | medium |

## Deliberate skips

Nine of the 34 open addresses were left alone. Eight were read and rejected:

1. **`0x80250E70` (1,468).** The eight-argument emitter forwards an untyped
   paired-single argument and builds a local record through `0x8023ECF0`/
   `0x8023EE90`; the pair packing is a `GQR` type selection, so the argument
   order would be a guess.
2. **`0x80251440` (1,256).** The vertex-palette tag walk blends two matrices
   whose indices come from `0x80250DD0` and from the `0x802DCD6C` handoff. The
   export multiplies both through the same twelve accumulators, but which
   buffer is the accumulator and which is the source is not separable from the
   C, and the blend weights arrive as paired-single bytes.
3. **`0x80251930` (1,308).** The body takes `in_f29`/`in_f30`/`in_f31` and their
   paired-single halves live from its caller (`0x80252020` passes only three
   integer key pointers through `call3`). The float register handoff is not
   declared, so the six argument roles cannot be pinned.
4. **`0x802529F0` (1,468) and `0x80252FB0` (1,468).** The record-sort twins.
   Their pivot index is `count/4 + counter mod 5` and
   `count*3/4 + (counter+1) mod 5`, where `counter` is the persistent rotating
   word at `sda(-0x5C60)`/`sda(-0x5C5C)` and the export prints the modulo with
   the compiler's floor-division correction. Reproducing the ladder is
   possible, but one wrong signed step would silently change the sort order.
5. **`0x80259760` (896) and `0x80259AE0` (896).** The word-record sort twins,
   same rotating counter at `sda(-0x5C50)` and the same floor-division pivot
   arithmetic. Rejected for the same reason.
6. **`0x8025FE30` (1,940).** Three near-identical skinning loops over
   sixty-four-entry pointer arrays, with `GQR2` conversions and a local matrix
   accumulator that the export shares between the second and third arm. The
   loop bodies are literal but the buffer roles are not; shipping three
   guessed arms would be worse than the skip.

The ninth, **`0x8025CA20` (2,952)**, was not opened beyond its declaration, so
no claim is made about it.

No address in this pass was already present in `ledger/functions.csv`; the 129
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
- `node tools/check-lane-csv.mjs wave101` prints `OK` with `wave101-os.csv`
  counted at 25 rows.
