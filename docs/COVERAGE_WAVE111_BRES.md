# Coverage - Wave 111 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs` and adds one body to it. The row
below is one literal reconstruction of a Ghidra body: the doc block above the
Rust body names the original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file. Offsets are
relative to the matrix destination named in the row.

The `0x80230000` bucket is shared with several subsystems. The Ghidra inventory
lists **251** addresses in it; wave 109 left it at **248 of 251** translated.
**Wave 111 takes one of the three that remained** (`0x802382B0`) and leaves
`0x802389F0` and `0x802390D0`, each with its reason under *Deliberate skips*.
The bucket now has **249 of 251** translated, and the file as a whole holds
**259** translated bodies.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - a field role or callee identity had to be reconstructed from a
  sibling, a value had to be recovered from a prologue stub, or a return value
  is inferred from two call sites.
- **low** - only the instruction sequence is certain; three lost argument lists,
  a control-flow restructure across `goto`s or a table layout had to be
  modelled.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 1 | 360 | 0 | 1 |

## What this pass covers

- **3x4 rotation-matrix builder** (`0x802382B0`, 360 bytes): the routine the
  wave-18 material path already calls by the name `MTX34_SCALE`. It reduces each
  of three angles modulo the `f32` 65536 at `0x8042E4D8` (`r2 - 0x4BA8`),
  interpolates the 256-entry sine/cosine table at `0x802F74A0`, folds the sign
  of each angle into its sine only, and writes the row-major XYZ product to
  twelve floats.

### The decoding facts this pass added

1. **The raw instructions are paired single, not scalar doubles.** The export
   presents every table read and matrix store as a four-way branch on
   `unaff_GQR0`/`unaff_GQR3`. The raw words at `0x802382B0..0x80238418` show
   `psq_l d(rA),0,0` (two floats), `psq_l d(rA),1,0` (one float),
   `psq_st d(rA),0,0` (two floats) and `psq_st d(rA),1,3` (the scratch word).
   The four-way branches are Ghidra printing the quantisation formats as if
   they were runtime tests; the format is the instruction's `I` field selecting
   GQR0 or GQR3. Both words are now explicit arguments, the same convention
   wave 98 used for the vector stores.
2. **The index comes from the raw scratch word, not from the float value.**
   `0x80238310` stores the reduced angle through GQR3, `0x8023831C` reloads the
   raw word with `lwz`, and `0x80238338` (`rlwinm r5,r0,20,20,27`) extracts
   `(stored_low_byte & 0xFF) * 0x10`. That is exactly the sequence
   `fn_80237ca0` already reconstructs, so the new body calls it once per
   angle instead of duplicating the interpolator.
3. **Element `[8]` is `-sin_y`, not zero.** The export prints
   `dVar18 = dVar14 - dVar14` and stores that zero into both `[7]` and `[8]`.
   The raw store at `0x80238404` is `psq_st f6,28(r3)` where `f6` is
   `ps_merge00(0, -f1)` built at `0x802383FC..0x80238400`; `f1` is the Y pair.
   The reconstruction keeps `-sin_y`, which is also what the sibling combined
   arm of `fn_802a0a90` writes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802382B0` | `fn_802382b0` | 360 | medium |

## Deliberate skips

Two addresses remain. Both were re-read in the export and in the raw image
before being skipped again.

- **`0x802389F0` (1 752 bytes):** the billboard/beam matrix builder that
  `fn_80238930` calls. The export is **3 034 lines** with **126
  "removing unreachable block" markers**, six paired-single input pairs and four
  twelve-float scratch matrices. Its declared shape is two pointer words plus
  five doubles, and the body is a faithful-transcription problem far beyond this
  pass's size. It stays an address call; `fn_80238930` documents the argument
  shape it needs.
- **`0x802390D0` (688 bytes):** the closest-approach/segment kernel. The raw
  body reads two 24-byte segment records (start `+0..+8`, end `+0xC..+0x14`),
  builds `d1`, `d2` and `r`, and computes the squared distance plus the two
  segment parameters. Unlike `0x802382B0`, the paired-single arithmetic is not a
  straight-line expansion: the export interleaves several dot products through
  reused scratch slots (`local_10`, `local_1c`, `local_8`, `local_14`,
  `local_28`, `local_20`, `local_58`, `local_34`), and the clamp tail at
  `0x80239220..0x802392B4` tests a `frsqrtes`-derived parallel-vector guard
  before choosing between four branches. Reproducing that guard and the exact
  clamp order would require guessing which lane each reused slot holds, so the
  body stays untranslated rather than presenting an approximation as the
  original.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (re-read from the TSV, not copied
  from prose): 360.
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 259 such bodies.
- No address in `ledger/agents/wave111-bres.csv` was already in
  `ledger/functions.csv` (`0x802382B0` was absent before this pass).
- `cargo check --message-format=short` reports no error and no warning for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave111` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
