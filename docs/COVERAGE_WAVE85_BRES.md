# Coverage - Wave 85 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`. Every function below is a literal
reconstruction of one Ghidra body; the doc block above each body names the
original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file. Offsets are
relative to the object named in the row.

- **high** - every load, store, branch and call is visible in the export; the
  only inference is the plain-English role sentence.
- **medium** - a field role, a constant from the small-data area or a callee's
  identity had to be reconstructed from a sibling or from the encoder that
  round-trips the same record.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 15 | 6,232 | 5 | 10 |

## What this pass covers

The `0x80230000` bucket entered this pass with 110 untranslated functions. Its
BRRES/g3d share was the tail of the material band, `0x8023C280..0x8023F5A0`,
plus four self-contained NW4R math helpers. This pass takes:

- **Polygon display-list builder** (`0x8023C280`): resolves the polygon's
  material (`+0x48`), palette (`+0x4A`), two texture slots (`+0x4C`) and eight
  texture slots (`+0x50`), then rewrites the 6-byte command records at
  `0xA0`/`0xB0` plus the slot number and publishes the `0xE0`-byte block.
- **Texgen record pair** (`0x8023C760`, `0x8023C830`): the first reads one
  0x30-byte stage record and its table-selected word; the second clears the old
  source byte, stores the new one and merges the packed descriptor into the
  BP command through `0x802399B0`.
- **Packed 11-bit matrix codec** (`0x8023D5F0`, `0x8023D8B0`, `0x8023DAA0`):
  three 32-bit groups hold two signed 11-bit fields and two exponent bits each.
  The exponent is biased by `0x11`; the field scale is `1/1024`, recovered from
  the encoder's reciprocal.
- **Attribute record pair** (`0x8023DEA0`, `0x8023E050`): decodes one
  0x12-byte record into a texture-format family/selector, and writes the two
  9-byte command records with the mode/family/scale switch.
- **Material texture/palette binder** (`0x8023E390`): the eight-argument
  `GXInitTexObj`/`GXInitTexObjCI` + `GXInitTexObjLOD` path for TEX0 and the
  `GXInitTlutObj` path for PLT0, with the 0x20/0x0C object caches and their
  present bitsets.
- **Per-entry animation resolvers** (`0x8023F370`, `0x8023F5A0`): the eleven
  colour-key tracks and the eight texture/eight palette handle tables.
- **NW4R G3D math** (`0x80238420`, `0x80238690`, `0x80238750`, `0x80239710`):
  matrix-times-vector, box expansion over a point array, box transform and box
  overlap.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80238420` | `fn_80238420` | 144 | high |
| `0x80238690` | `fn_80238690` | 188 | high |
| `0x80238750` | `fn_80238750` | 472 | high |
| `0x80239710` | `fn_80239710` | 112 | high |
| `0x8023C280` | `fn_8023c280` | 908 | medium |
| `0x8023C760` | `fn_8023c760` | 204 | high |
| `0x8023C830` | `fn_8023c830` | 284 | medium |
| `0x8023D5F0` | `fn_8023d5f0` | 696 | medium |
| `0x8023D8B0` | `fn_8023d8b0` | 488 | medium |
| `0x8023DAA0` | `fn_8023daa0` | 360 | medium |
| `0x8023DEA0` | `fn_8023dea0` | 428 | medium |
| `0x8023E050` | `fn_8023e050` | 500 | medium |
| `0x8023E390` | `fn_8023e390` | 700 | medium |
| `0x8023F370` | `fn_8023f370` | 364 | medium |
| `0x8023F5A0` | `fn_8023f5a0` | 384 | medium |

## Deliberate skips

- **Key sampling with caller-set GQR:** `0x8023F000`, `0x8023F140`,
  `0x8023F4E0`, `0x8023FA30` and `0x8023FE50` stay untranslated. Their Ghidra
  bodies decode key counts, indices and times through the paired-single
  quantisation register, which the caller sets; the exports discard the scale
  factor (`ldexpf` with an unused result) or read the raw 16-bit halves of a
  float. Reconstructing them would require guessing GQR, which the lane brief
  forbids. `0x8023F370` and `0x8023F5A0` still call `0x8023F140` and
  `0x8023F4E0` by absolute address, so the call graph stays explicit.
- **The rest of the bucket is another subsystem:** the untranslated
  `0x80230280..0x80239780` band is the engine console/framebuffer code, the
  Broadcom Bluetooth utilities and the NW4R math library. Most of those bodies
  depend on `unaff_r13` (unknown small-data base), on GQR, or on paired-single
  registers; the console block is already represented by
  `runtime::bt`'s `0x80233420` console-config note. They are left for the
  module that owns that subsystem rather than being relabelled as BRRES.
- **`0x8023C280` was already described by a constant:** the old
  `MDL_POLYGON_BUILD` constant documented it as an address-only callee. The
  body is now real, so the constant's call site in [`mdl_model_bind`] resolves
  to [`fn_8023c280`].

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read from the TSV, not copied from prose).
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 166 bodies.
- No address in `ledger/agents/wave85-bres.csv` was already in
  `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors and of diagnostics for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave85` reports `OK` (101 rows across the four
  lanes, of which 15 are this lane).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
