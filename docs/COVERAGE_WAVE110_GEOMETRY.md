# Coverage - Wave 110 (geometry GX ribbons and particle quads)

## How to read this file

This is the Wave 110 record for the last four untranslated entries of the
`0x80000000` bucket in [`src/render/geometry.rs`](../src/render/geometry.rs).
It answers four questions for a reader with no context:

1. **What was added?** Four `pub unsafe fn` bodies: the 200-quad billboard
   emitter, the four-corner ribbon strip, its six-vertex variant, and the long
   ribbon strip with a rounded end cap.
2. **Where is it?** Only in [`src/render/geometry.rs`](../src/render/geometry.rs).
   No other Rust file, `lib.rs`, `mod.rs`, `Cargo.toml` or `platform.rs` was
   edited.
3. **Why were they missing?** Every earlier pass deferred them because Ghidra's
   C export loses whole blocks inside their FIFO loops: the `psq_l`/`psq_st`
   prologues become `ldexpf`/`unaff_GQR` noise and the vertex loops show one
   FIFO word per iteration where the raw stream writes four or six vertices.
   This pass decoded the raw instruction words instead.
4. **What is still missing?** Nothing in this bucket. The four entries were the
   only addresses below `0x80100000` that had no row in
   [`ledger/functions.csv`](../ledger/functions.csv) when the pass started.

Addresses are plain lowercase hex **virtual** addresses (image base
`0x80004000`), not file offsets. Evidence paths are relative to the repository
root and live under `work/ghidra/main-gb/export/`. Raw instruction checks used
`work/dol/main.elf` (the DOL converted by `tools/dol2elf.mjs`).

Terms: **GX** is the Wii GPU; **FIFO** is the GX write-gather stream at
`0xCC008000`; **GQR** is a paired-single quantisation register; **r2** is the
second small-data base (`0x80433080`); **TEV** is the GX texture environment.
See `docs/GLOSSARY.md`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/geometry.rs` | `render::geometry` | 4 | 6,260 | 0 | 4 |

The bucket had exactly four untranslated entries out of 92 at the start of this
pass (the `0x80007DE0..0x8000E83C` addresses below). All four are landed, so
the bucket is now closed: 92 of 92.

`src/render/geometry.rs` now holds 50 `pub unsafe fn` bodies: 45 address-named
`fn_<addr>` functions and the five descriptive helpers
([`vec2_sub`], [`vec2_differs_from`], [`vec2_length`],
[`store_four_lane_vertices`], [`copy_12_floats`]).

## What this pass covers

- **Billboard particle quads** - [`fn_80007de0`] walks the 200 records at
  `+0x780` (three `f32` each) and emits one `GXBegin(0x80, 0, 4)` quad per
  record. The radius is `record.z * (this[+0x68] + 0.1 * (i & 7))`; the first
  eleven records use the fixed radius `150.0` and alpha `30`; records from
  `190` on multiply the radius by `4.0`. The four corners are the four
  quarter-turn phases of the `u16` angle at `+0x1270`, with the quadrant UVs
  `(0,0)`, `(1,0)`, `(1,1)`, `(0,1)`.
- **Four-corner ribbon** - [`fn_8000c5c0`] draws `count` quads from the four
  parallel `Vec3` arrays at `+0x48`, `+0x33D8`, `+0x6768` and `+0x9AF8`. Each
  vertex is `xyz` plus two `(s16, s16)` texture coordinates; the last two
  corners drop their `y` by `32.0`, which folds the ribbon into a box section.
- **Six-vertex ribbon** - [`fn_8000ca50`] is the taller sibling: one
  `GXBegin(0x98, 0, 6)` triangle strip per record, with an RGBA colour per
  vertex, a `170 * (i % 6)` horizontal texture step, and a far pair lowered by
  `8.0` with alpha `0`.
- **Ribbon with end cap** - [`fn_8000e060`] opens a single
  `GXBegin(0x98, 0, (count + 9) * 2)` strip, writes one cross-section per
  record (the centre advances by each record's length along the angle), adds
  eight rings of a rounded end cap, and closes with one pair. The longitudinal
  texture coordinate is the remaining rail length times `512.0`, saturated by
  `0x802DCC4C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80007DE0` | `fn_80007de0` | 1,632 | medium |
| `0x8000C5C0` | `fn_8000c5c0` | 1,168 | medium |
| `0x8000CA50` | `fn_8000ca50` | 1,444 | medium |
| `0x8000E060` | `fn_8000e060` | 2,016 | medium |

All four also have a row in
[`ledger/agents/wave110-geometry.csv`](../ledger/agents/wave110-geometry.csv).
The address range is `0x80007DE0..0x8000E83C` and is not contiguous: seven
already-translated entries and the `0x8000B...`/`0x8000C...` gap separate them.

## Raw-word checks

Each body was built from the instruction stream, not from the Ghidra C. The
sizes above were re-read from
`work/ghidra/main-gb/export/functions.tsv` (1,632 / 1,168 / 1,444 / 2,016
bytes, i.e. 408 / 292 / 361 / 504 instructions).

| address | key raw facts used by the translation |
|---|---|
| `0x80007DE0` | `bl 0x802DCD40` is `__savegprlr_14`, so `r15 = r3` is `this`, not a return value. The `psq_st` prologue spills `f20..f31`/`ps20..ps31`; it is not data. The 200-record loop starts at `0x800080DC`; the four corners and the 16 FIFO stores per iteration are at `0x80008378..0x800083C0`. |
| `0x8000C5C0` | `bl 0x802DCD60` is `__savegprlr_22` (`r22 = this`). The loop body at `0x8000C91C` computes the primitive/count through a shift chain that reduces to `(0x80, 4)` for every index below `2^28`, then writes four 20-byte vertices (`0x8000C944..0x8000CA04`). |
| `0x8000CA50` | `bl 0x802DCD54` is `__savegprlr_19` (`r19 = this`). The `0x2AAAAAAB` multiply at `0x8000CE00` is `index % 6`; `mulli r19,r0,170` gives the texture step. Six 24-byte vertices are written at `0x8000CE28..0x8000CFBC`. |
| `0x8000E060` | `bl 0x802DCD50` is `__savegprlr_18` (`r28 = this`). The guard at `0x8000E0CC` rejects counts below two. One `GXBegin(0x98, 0, (count+9)*2)` at `0x8000E3AC`, then the segment loop (`0x8000E420..0x8000E53C`), the eight-ring cap (`0x8000E5A4..0x8000E6CC`) and the closing pair (`0x8000E7A8..0x8000E7CC`). |

## Uncertainties and limitations

1. **Callee GX API names.** Every `0x801Cxxxx` call is another lane's GX
   helper. The Rust calls it by absolute address and documents only what the
   file observes (argument registers, FIFO reads/writes). Names such as
   "TEV combiner" or "vertex attribute descriptor" are `// inferred:` from the
   literal operands and from the callee exports; they are not symbol-table
   facts.
2. **The `psq_l` phase conversion.** All four emitters store a `u16` angle with
   `sth` and read it back through `psq_l` with GQR3. Ghidra models that as
   `ldexpf(...)` multiplication, i.e. a signed 16-bit value times a
   quantisation scale. The Rust reads the same `u16` and multiplies it by the
   image's `1/256` constant (`0x8042B0BC` for `0x80007DE0`,
   `0x8042B108` for `0x8000E060`), exactly as the earlier
   [`fn_8000d740`] translation does. The static image does not record GQR3's
   runtime value, so the effective radians-per-count scale is `// inferred:`.
3. **`fn_80007de0`'s mode branch.** The raw compares the remainder to `0`, then
   `1`, then `2`, but the compare order maps `0 -> +0x28`, `1 -> +0x48`,
   `2 -> +0x08`. Ghidra prints the un-rotated mapping (`0 -> +8`); the Rust
   follows the raw words.
4. **`fn_8000c5c0`'s shift chain.** The image computes the primitive word and
   the vertex count from the loop index with four shifts. For every index below
   `2^28` the chain reduces to `0x80` and `4`, which is what Ghidra prints. The
   Rust keeps the literal arithmetic so the reduction is visible; the stored
   texture coordinate is truncated to `u16` exactly as the `sth` stores do.
5. **`fn_8000e060`'s ring count.** The eight cap rings are a fixed count; the
   `0x1000` step and the `0x7000` start are literal. There is no data-driven
   tessellation parameter in the function.
6. **No runtime validation.** `cargo check` proves the module type-checks; it
   does not execute the PowerPC code, and no render output was compared against
   the console.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (1,632 / 1,168 / 1,444 / 2,016 bytes).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/render/geometry.rs`.
- No address was already in `ledger/functions.csv`; the bucket's four open
  entries were re-filtered before writing (92 total, 4 open).
- `cargo check --message-format=short` reports no errors and no warnings in
  `src/render/geometry.rs`.
- `node tools/doc-lint.mjs src` reports `{"ok": true, "files": 68, "violations": 0}`.
- `node tools/check-lane-csv.mjs wave110` reports `OK` for this lane's CSV.
