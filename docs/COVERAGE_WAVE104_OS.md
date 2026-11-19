# Coverage - Wave 104 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records wave 104 of the `main.dol` reconstruction for the
`runtime::os` lane, run on the `0x80240000` bucket. The bucket holds 137
inventory entries; 116 were already merged when this pass started and 21 were
open. Fourteen of those 21 are reconstructed here
(`0x80241690`..`0x8024ABD0`); the other seven are listed under
[Deliberate skips](#deliberate-skips).

- **Status** - every row is `translated`: each guard, store and call printed in
  the Ghidra export has a counterpart in `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. All fourteen rows
  are `medium`: the control flow, offsets, strides and vtable literals are
  literal, but at least one field role, constant or call arity is
  `// inferred:`/`// unknown:` and is marked as such in the body.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave104-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 14 | 8,308 | 0 | 14 |

Status split: 14 `translated`, 0 `partial`, 0 `stub`.

## What this pass covers

- **Cluster A - the element dispatcher and its sample reader
  (`0x80241690`, `0x80241EB0`).** Both walk the record shape that the already
  translated `0x80241020` and the Wave 96 curve readers use: the record root's
  `+0x10` word is a self-relative offset to an element section, and element `n`
  is the self-relative `+0x14` word at `section + (n + 1) * 0x10`.
  `0x80241690` switches on bits 20..24 of the element's `+0x04` word and fills a
  `0x4C`-byte slot: `0` = identity basis, `4` = three raw floats, `8`/`0xC` =
  one `0x8023FE50` read plus a `0x802405B0` evaluation, `0x10`/`0x14`/`0x1C` =
  one `0x80241020` read, and the default = both. `0x80241EB0` evaluates one
  record's keyframe tracks into a `0x40`-byte block: one clamp
  (`[sda2(-0x4AA8), u16(record+0x34)]`), one animation-frame bit test and one
  `0x8023F000`/`0x8023F140` call per track.
- **Cluster B - the entry constructors and their index arrays
  (`0x80243D20`, `0x80245300`, `0x80245500`, `0x80245970`, `0x80246900`,
  `0x80246B00`, `0x80246F70`).** The three constructors are one body at three
  strides (`0x5C`, `0x44`, `0xE8`) and three fill kernels
  (`0x8023F370`, `0x8023F5A0`, `0x8023FA30`), exactly like the already
  translated `0x802481B0` (`0x2C`) and `0x80248990` (`0x4C`). Each publishes
  `selected * stride + count * 2 + 0x34` through the low half of `0x802DCD6C`,
  builds the index array at `+0x34`, the entries after it, and installs two
  vtables around one virtual-slot `+0x34` call. `0x80245300`/`0x80246900` are
  the plain `u32`-per-entry holders (`align4(count * 2 + 0x23) + entries * 4`)
  for the two vtable families. `0x80245970`/`0x80246F70` walk the record's
  `+0x32`/`+0x22` entries, look each key up through `0x8023B7B0` and publish
  the found index into the `u16` array at `*(context + 0x14)`.
- **Cluster C - the texture-stream passes (`0x80245B40`, `0x80249740`).**
  `0x80245B40` walks the two bit masks at a texture/material record's `+0`/`+1`
  bytes: for each set bit of the first it decodes the packed vertex descriptor
  (`0x801C7260`), reads the TEX0 header through `0x8023EB30`/`0x8023EBF0` and
  programs the object with `0x801C6D90`/`0x801C6FA0`/`0x801C6FF0`; the second
  loop emits one `0x64` command header per set bit through `0x801C7520`.
  `0x80249740` re-indexes a `0x44`-stride array: it clears bit 2 of every entry,
  resolves each target through virtual slots `+0x7C`/`+0x78` and re-emits it
  through `+0x6C`, after two counted publish passes through
  `+0x64`/`+0x68`.
- **Cluster D - the fill loop and the two basis builders (`0x8024A610`,
  `0x8024A6E0`, `0x8024ABD0`).** `0x8024A610` writes the constant at
  `sda2(-0x4A08)` over `size / 4` floats (`size >> 8` blocks of 64, then
  `(size >> 5) & 7` blocks of 8). The two builders compose a 3x4 matrix from
  either the current angle slot (`0x803E29E8 + angle * 0x30`) or a `0x44`-byte
  colour record (`0x803DFFB4 + code * 0x44`), using the same
  normalise/cross/concatenate sequence and finishing with `0x801C0540` against
  either the constant basis `0x802F8E90` (`0x8024A6E0`) or the current slot in
  `0x803E3DE8` (`0x8024ABD0`).

## What the pass adds beyond the raw bodies

1. **The element-section convention is now written down.** Three independent
   bodies (`0x80241690`, `0x80241EB0`, both index fillers) resolve an element as
   `root + *(root + 0x10)`, then `section + *(section + (n + 1) * 0x10 + 0x14)`.
   This is the same shape the Wave 96 `0x8023F370`/`0x8023F5A0`/`0x8023FA30`
   fill kernels use, so the convention can be cross-checked in four places.
2. **The three entry constructors are now complete as a family.** With this
   pass the `0x2C` (`0x802481B0`), `0x44` (`0x80245500`), `0x4C`
   (`0x80248990`), `0x5C` (`0x80243D20`) and `0xE8` (`0x80246B00`) strides all
   have bodies, and the two `u32`-table holders (`0x80245300`/`0x80246900`)
   explain the `0x8034E390`/`0x8034E4B0` classes that the record table indexes.
3. **The `0x802DCDxx` handoffs were used the same way as in Waves 98/101/102.**
   The low half of each call is the out-pointer/size slot and the high half the
   heap handle or context, exactly as `ledger/functions.csv` records for those
   four-byte stubs.
4. **The floating-point call shapes are explicit.** The exports print
   `0x8023FE50`, `0x802405B0` and `0x80241020` with fewer register arguments
   than they read, because their curve parameter arrives in `f1`. The bodies
   therefore call them through typed `extern "C"` trampolines with the `f64`
   first, and the two `0x802405B0` arities are documented at their helpers.
5. **The `0x802DCD6C`/`0x802DCD70` size words are proved.** `0x80245300` and
   `0x80246900` write `align4(count * 2 + 0x23) + entries * 4`, and
   `0x80243D20` writes `selected * 0x5C + count * 2 + 0x34`; both land in the
   low half of the handoff, which pins the sign of that convention.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80241690` | `fn_80241690` | 1,124 | medium |
| `0x80241EB0` | `fn_80241eb0` | 1,000 | medium |
| `0x80243D20` | `fn_80243d20` | 496 | medium |
| `0x80245300` | `fn_80245300` | 308 | medium |
| `0x80245500` | `fn_80245500` | 496 | medium |
| `0x80245970` | `fn_80245970` | 248 | medium |
| `0x80245B40` | `fn_80245b40` | 500 | medium |
| `0x80246900` | `fn_80246900` | 308 | medium |
| `0x80246B00` | `fn_80246b00` | 496 | medium |
| `0x80246F70` | `fn_80246f70` | 248 | medium |
| `0x80249740` | `fn_80249740` | 808 | medium |
| `0x8024A610` | `fn_8024a610` | 204 | medium |
| `0x8024A6E0` | `fn_8024a6e0` | 984 | medium |
| `0x8024ABD0` | `fn_8024abd0` | 1,088 | medium |

## Deliberate skips

Seven of the 21 open addresses were read and left alone. All seven are honest
skips, not partial rows:

1. **`0x80240100` (308), `0x80240240` (380), `0x802403C0` (492).** The three
   curve readers. Every field access in them is expanded by the decompiler into
   three runtime `GQR` arms (`bVar3 == 4 || 6`, `5 || 7`, else) with
   `ldexpf` scale factors taken from the `unaff_GQRn` registers. That is the
   decompiler simulating a `psq_l`/`lfs` whose element type is set at startup,
   not real branches; without the instruction stream the actual access width
   cannot be recovered, so writing one arm would be a guess. The already
   translated `0x80241020` calls them through
   `wave96_curve(entry, t, curve)`, so the call shape is preserved and nothing
   regresses by leaving the bodies open. This matches the Wave 102 decision on
   `0x801B0170`.
2. **`0x802405B0` (2,660).** The anim evaluator the cluster-A dispatcher calls.
   It reads only its third, fourth and fifth parameters and expands the same
   `GQR` store/load arms for the `0x20000000`..`0x28000000` source formats; the
   `extraout_f1` curve parameter is also never declared in the export. The
   caller-side shape is captured in `w104_eval_curve5`/`w104_eval_curve6` with
   the used registers pinned, so a later pass can fill the body without
   changing this lane.
3. **`0x802471E0` (428).** Reads `in_f31`/`in_ps31_1` (an incoming `f1`/`ps1`
   pair the export never declares) before the first call, then folds a `0x14`-
   strided record into `0x8023DAA0`. The incoming vector and the `0x49F4`
   epsilon are not separable from the C.
4. **`0x8024E180` (2,212).** The vertex/record emitter with eight incoming
   `f23`..`f31` doubles and eight `ps23_1`..`ps31_1` lanes. The register file is
   the whole argument list and Ghidra prints it as `void`, so there is no
   honest signature to write.
5. **`0x8024FEF0` (828).** The per-instance draw loop. Same undeclared
   `in_f31`/`in_ps31_1` pair, plus a function-pointer table at `0x802F8ED0`
   whose entries take five arguments and one variadic tail; the export cannot
   separate the table index from the source word.

No address in this pass was already present in `ledger/functions.csv`; the 116
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (14 of 14; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes
  (`src/runtime/bt.rs`).
- `node tools/check-lane-csv.mjs wave104` prints `OK`, with `wave104-os.csv`
  counted at 14 rows.
