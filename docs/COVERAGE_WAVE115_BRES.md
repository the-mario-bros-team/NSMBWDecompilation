# Coverage - Wave 115 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs` and adds one body to it. The row
below is one literal reconstruction of a Ghidra body: the doc block above the
Rust body names the original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file. Offsets are
relative to the 240-byte record the function fills.

The `0x80230000` bucket is shared with several subsystems. The Ghidra inventory
lists **251** addresses in it. Wave 112 left the bucket at **250 of 251** with
only `0x802389F0` open. **Wave 115 takes `0x802389F0`**, so the bucket stands at
**251 of 251**, and `src/resource/bres.rs` now holds **261** translated bodies.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - a field role or callee identity had to be reconstructed from a
  sibling, a value had to be recovered from a prologue stub, or a return value
  is inferred from two call sites.
- **low** - only the instruction sequence is certain; three lost argument lists,
  a control-flow restructure across `goto`s or a table layout had to be
  modelled.

Inside the Rust file, `// observed:` marks facts read directly from the raw
instruction stream, `// inferred:` marks a role reconstructed from neighbours,
and `// unknown:` marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 1 | 1752 | 0 | 1 |

Address range: `0x802389F0`.

## What this pass covers

- **Beam / shadow-volume record builder** (`0x802389F0`, 1 752 bytes). The
  record is 0xF0 bytes: the source 3x4 matrix at `+0x00`, four planes at
  `+0x30/+0x40/+0x50/+0x60`, the negated depths at `+0x70/+0x74`, the min/max
  box at `+0x78`, and six more planes at `+0x90..+0xE0`.
- The body inverts the source matrix (`0x801C07A0`), copies the matrix into the
  record (`0x801C0500`), builds eight corners of a near/far quad, cuts four
  planes through the origin, transforms the eight corners in place through the
  inverse (`0x801C1160`, eight iterations), expands the box
  (`0x80238690`), and cuts six more planes around the transformed origin.
- Every plane is the same pattern already documented for `fn_802385d0`:
  `cross(first - base, second - base)` through `0x801C1530`, `0x801C1470`
  normalises into the plane slot, and `d = -dot(normal, base)` is a plain
  `stfs`. The new local helper `build_plane` performs exactly those three
  steps.

### The decoding facts this pass added

1. **The export's 3 030 lines are a regular 438-instruction body.** All 126
   "removing unreachable block" markers come from the paired-single save and
   restore ring (`stfd`/`psq_st` pairs at `0x80238A00..0x80238A2C` and
   `0x80239080..0x802390AC`), not from the arithmetic. The body itself is
   `bl 0x802DCD64` (the register-save thunk), one inverse, one copy, 24 corner
   stores, four plane builds, an eight-iteration loop, the box call, six plane
   builds, and `bl 0x802DCDB0`.
2. **The GQR soup is plain `W=0`/`W=1` float traffic.** Every `ldexpf` /
   `CONCAT` block in the export is Ghidra's model of the paired-single format
   field. Each vector is stored as an `(x, y)` pair plus a `W=1` single for `z`;
   the `W=1` lane 1 is the implicit `1.0`, which the dot product discards when
   `ps_sum0` adds lane 0 of the madd to lane 1 of the product.
3. **`ps_sub` uses `FRB`, `ps_mul`/`ps_madd` use `FRC`.** `ps_sub fD,fA,fB`
   subtracts the third encoded field (`FRB`), while `ps_mul fD,fA,fC` and
   `ps_madd fD,fA,fC,fB` multiply `FRA` by `FRC`. Reading the `ps_sub` blocks
   with `FRC` turns every second edge vector into `x - x = 0` and makes the
   cross products degenerate; re-reading them against `FRB` reproduces the two
   distinct edge vectors the export's C shows.
4. **The first four planes measure from the zero point, not the origin.**
   `0x80238A74..0x80238A98` stores `r2 - 0x4B9C`, which is `0x8042E4E4 =
   0.0f`, three times at `sp+0x170`. The four pre-transform plane blocks use
   that triple as their `base`, so their fourth component is `-0.0`. The loop
   then overwrites `sp+0x170` with the inverse's translation column, and the
   last six planes use that as their base.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802389F0` | `fn_802389f0` | 1752 | medium |

## Deliberate skips

- None. The one open address in the bucket is translated.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (re-read from the TSV, not copied
  from prose): 1752.
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 261 such bodies.
- No address in `ledger/agents/wave115-bres.csv` was already in
  `ledger/functions.csv` (`0x802389F0` was the only address left in the
  `0x80230000` bucket after wave 112, recomputed from `functions.tsv` minus the
  ledger).
- `cargo check --message-format=short` reports no error and no warning naming
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave115` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and the paired-single lane behaviour, and must only execute on a
target-compatible runtime with the original memory image mapped.
