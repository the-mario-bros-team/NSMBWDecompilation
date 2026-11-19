# Coverage - Wave 100 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`. Every row below is one literal
reconstruction of a Ghidra body; the doc block above the Rust body names the
original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file. Offsets are
relative to the object named in the row.

The `0x80230000` bucket is shared with several subsystems. Waves 85, 90, 95 and
98 closed the BRRES material band, the debug-console band, the exception report,
the message queues, the text-box measures and the vector-store members of the
packed GQR group. Wave 98 left 34 addresses untranslated; this pass takes 24 of
them and leaves 10, each for a reason recorded under **Deliberate skips**.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - a field role or callee identity had to be reconstructed from a
  sibling, or the live GQR word is an explicit argument, or a prologue result is
  supplied by the caller.
- **low** - only the instruction sequence is certain; a `va_list` descriptor, an
  FPR argument block, the hardware `fsqrt` or an asymmetric export branch had to
  be modelled.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 24 | 7,728 | 0 | 15 |

The file now holds **251** translated bodies (227 before this pass). Nine of the
24 new rows are `low`; six are `partial` (five console entry points plus the
byte-colour lerp) because a caller-built descriptor or a split-helper result is
an explicit argument rather than something the crate can construct.

## What this pass covers

- **Angle and logarithm kernels** (`0x80237A60..0x80237ED0`): the sine-plus-
  offset kernel, the exponent/mantissa log2 helper, the Newton reciprocal square
  root, the two low-byte table reducers, the sine/cosine pair, and the two
  octant-reduced arctangent forms (the second returns a packed `u16`). These are
  the bodies wave 98 refused because they store through a GQR format and read a
  lane back as an index.
- **3-D math** (`0x80238200`, `0x802385D0`, `0x80239780`, `0x80239380`,
  `0x802394E0`): the 3x4 matrix-times-vector transform, the plane-from-three-
  points builder, the six-plane box classifier, and the pair of shape/segment
  clips. The clips share [`fn_80237ba0`] as their square root.
- **Key sampling** (`0x8023F000..0x8023FE50`): the complete five-function band —
  Hermite key evaluation, the fixed-point byte-colour lerp, the key bracketing
  search, the two-mask key applicator, and the three-component element reader.
- **Console band** (`0x802337B0`, `0x80234A80`, `0x80236230`, `0x802365E0`,
  `0x80236FC0`, `0x80237370`): the 5x7 glyph blitter, the exception report sink,
  and the four queue entry points (two per heap; the pair on each heap is
  byte-identical apart from its address).

## The read-back model

Wave 98's objection was that bodies such as `0x80237BD0` store a quantised value
to a stack slot and then read a *different-width* lane back, so the export mixes
defined and undefined bytes. This pass restates the sequence with
[`psq_round`]:

```
store the float through the low half of gqr into a 4-byte slot
read the raw 4-byte word back          -> bits  (table index)
reload the lane through the high half  -> value (interpolation base)
```

`psq_lane(bits, gqr)` returns the lane the store actually wrote (first byte for
format 4/6, first half for 5/7, the whole word otherwise). Where the export's
alternative branch would read a byte the store never touched, the doc block
records that the natural lane is used. This is the only modelling choice added
to an otherwise literal transcription.

Two smaller notes apply to the math band:

- the multiply is done in `f32` where the original widens to `f64` first, so a
  value exactly on a truncation boundary can move by one unit;
- `sqrt64` replaces the hardware `fsqrt` with an exponent-halving seed and five
  Newton steps. It reaches full double precision but is not guaranteed to be
  correctly rounded in the last bit; that is why [`fn_80237ba0`] and the two
  clips are marked `low`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802337B0` | `fn_802337b0` | 704 | medium |
| `0x80234A80` | `fn_80234a80` | 224 | low |
| `0x80236230` | `fn_80236230` | 288 | low |
| `0x802365E0` | `fn_802365e0` | 288 | low |
| `0x80236FC0` | `fn_80236fc0` | 288 | low |
| `0x80237370` | `fn_80237370` | 288 | low |
| `0x80237A60` | `fn_80237a60` | 156 | low |
| `0x80237B00` | `fn_80237b00` | 152 | medium |
| `0x80237BA0` | `fn_80237ba0` | 36 | low |
| `0x80237BD0` | `fn_80237bd0` | 112 | medium |
| `0x80237C40` | `fn_80237c40` | 96 | medium |
| `0x80237CA0` | `fn_80237ca0` | 112 | medium |
| `0x80237D10` | `fn_80237d10` | 440 | medium |
| `0x80237ED0` | `fn_80237ed0` | 464 | medium |
| `0x80238200` | `fn_80238200` | 108 | medium |
| `0x802385D0` | `fn_802385d0` | 180 | medium |
| `0x80239380` | `fn_80239380` | 344 | medium |
| `0x802394E0` | `fn_802394e0` | 548 | medium |
| `0x80239780` | `fn_80239780` | 428 | low |
| `0x8023F000` | `fn_8023f000` | 308 | medium |
| `0x8023F140` | `fn_8023f140` | 260 | medium |
| `0x8023F4E0` | `fn_8023f4e0` | 180 | medium |
| `0x8023FA30` | `fn_8023fa30` | 1044 | medium |
| `0x8023FE50` | `fn_8023fe50` | 680 | low |

## Deliberate skips

10 of the 34 addresses wave 98 left stay untranslated. Every one was re-read in
the export before being skipped.

- **`0x80230280` / `0x80232C10` (1 240 bytes each):** the paired-single text/box
  transforms. Their signatures take two quantised FPR pairs
  (`in_f30`/`in_f31` and their `ps` lanes), copy a 0x60-byte opaque console
  record and hand it to `0x8022F960`/`0x8022F350`. The FPR pair is the same
  read-back problem at a wider width, and the record layout is not described by
  any body in this file, so only a shell could be produced.
- **`0x80231CF0` (1 468 bytes) and `0x802323E0` (2 088 bytes):** the text layout
  engine and the per-character renderer. Both consume five FPR inputs plus an
  opaque record and both call the `0x80231CF0` chain, so they inherit the same
  objection at a size where a single wrong lane silently shifts an entire line.
- **`0x802348F0` (400 bytes):** the console frame pump. Its argument is read
  from `FUN_802DCD64` (`_savegpr0_23`), and its final loop is an unmodelled
  `do { } while (true)` over `_DAT_800000F8 / 4000`; the two timing helpers
  return `u64` and `crate::platform` exposes only 32-bit returns, so the frame
  deadline cannot be expressed without changing the helper layer.
- **`0x80235AE0` (716 bytes):** the script parser. The decompiler could not
  structure it — `unaff_r30` is reused as a loop counter across three `goto`
  targets, three calls have empty argument lists, and the final string-copy loop
  is emitted after the return.
- **`0x802382B0` (360 bytes):** the Euler matrix builder. Its callers pass
  `(ax, ay, az, dst)` — three `double` angles and one pointer — but the export
  fuses the three angle parameters with two `float *` source vectors
  (`param_1[0..5]`, `param_2[0..5]`) whose storage does not exist in the call
  sites. Assigning those pointers would be a guess, so the body stays skipped.
- **`0x80238930` (180 bytes):** the instancing wrapper. It forwards six doubles
  to `0x802389F0`; `crate::platform` passes only 32-bit GPR words, so the FPR
  argument list cannot be reproduced without editing the shared platform layer.
- **`0x802389F0` (1 752 bytes):** the billboard/particle matrix builder. Same
  six-double FPR objection, plus six paired-single inputs; it is the callee that
  makes `0x80238930` a shell.
- **`0x802390D0` (688 bytes):** the closest-approach/segment kernel. The export
  is 839 lines of scalar emulation with more than thirty scratch slots reused
  across GQR lanes; reconstructing it without introducing a wrong lane would be
  guesswork rather than translation.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (re-read from the TSV, not copied
  from prose).
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 251 such bodies.
- No address in `ledger/agents/wave100-bres.csv` was already in
  `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave100` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
