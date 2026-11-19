# Coverage - Wave 115 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 115 pass over the last open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave115-gx-band.csv` (two rows).

The bucket held 85 entries, six of them untranslated, when this pass started.
Two are landed here; the other four are documented under "Deliberate skips"
with the concrete blocker that remains for each.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 708 | 0 | 2 |

## The method that re-opened these two entries

Wave 112 translated `0x80296160`/`0x802961E0` by decoding the raw
`psq_l`/`psq_st` operands instead of trusting the export, and left the two
larger table builders open because their lane algebra was still opaque.

This pass finished that job for the two builders by decoding the **paired-single
opcodes** rather than only the GQR selectors. The export of
`0x80296260`/`0x802963B0` prints 25 and 29 `unaff_GQRx` ladders but only 11 and
12 dropped "unreachable" arms, all of which are the unused type arms of the
same ladders - the value flow is complete. With the opcode map from the PowerPC
paired-single encoding (`ps_muls0/1` = `ps_mul` + lane-zero/one broadcast,
`ps_madds0/1`, `ps_abs`/`ps_neg`, `ps_merge00/01/10`, `ps_sum0`) every lane of
both functions decodes to a plain `f32` expression, so the bodies below are
transcriptions of the instruction stream, not of the export's collapsed
`double` arithmetic.

Two constants and the table geometry were read straight out of
`work/dol/main.elf` (`r2 = 0x80433080`):

| constant | value | role |
|---|---|---|
| `0x8042ECF0` | `65536.0` | modulo period for the table coordinate |
| `0x8042ECF8` | `40.7436638` = `256/(2*pi)` | radians -> 1/256-turn units |
| `0x802F74A0` row 0 | `(0.0, 1.0, 0.024541, -0.000301)` | `(sin, cos, dsin, dcos)` |

Both functions convert an angle with `angle * 256/(2*pi)`, fold it to its
absolute value (`ps_abs` for the x/y pair, `fabs` for the scalar third angle),
reduce it modulo `65536.0` by repeated subtraction, quantise the integer part
through `GQR3`, keep only its low byte as the row number and use the fractional
part for the linear interpolation between that row and the next. That is the
same sample that [`sincos_table`] already performs for `0x80296160`, including
the `Trunc`-then-`0..=255` clamp.

One correction to the wave-112 note: in these two functions the `GQR3`
element is **not** a byte. Both use `lwz`/`lhz` reloads followed by
`rlwinm rD,rS,20,20,27` / `rlwinm rD,rS,4,20,27`, which only recovers the row
number it just stored when the pair occupies two halfwords per lane. The width
is therefore marked `inferred:` as an unsigned halfword with zero scale; the
row mask is `& 0xFF` because the `rlwinm` mask is eight bits wide.

## What this pass covers

- **The three-angle vector builder `0x80296260`** (332 bytes, `medium`). Reads
  three `f32` table coordinates from `r3`, samples each one, and combines the
  six interpolated values into three floats written to `r4` (returned in `r3`).
  The sign fold is `ps_neg` + `ps_merge01`, which negates lane 0 when the
  scaled angle is negative, i.e. the stored sine is the sine of the input angle
  (the modulo loops above worked on its absolute value). The composition is `ps_muls1` (both lanes times `cos_x`), `ps_muls0`
  (both lanes times `sin_x`), one `ps_merge10` lane swap, one `ps_neg` and one
  `ps_madds0`; the last store is a single-lane `psq_st` that takes lane 1 of
  the final merge. The export renders the same chain as
  `dVar22*dVar17*dVar13 + -(dVar14*dVar21)` and `dVar24*(dVar23-dVar26)+dVar18`,
  which is the collapsed form of those five instructions. The result is *not*
  normalised, so the body makes no claim about what the vector is used for
  (`// unknown:` in the doc block).
- **The twelve-float block builder `0x802963B0`** (376 bytes, `medium`). The
  same conversion, sampling and sign fold for three scalar angles taken in
  `f1`/`f2`/`f3`, expanded through `ps_muls0`/`ps_muls1`/`ps_madds0`/
  `ps_merge00`/`ps_merge10` into twelve floats stored at `+0x00`, `+0x04`,
  `+0x0C`, `+0x14`, `+0x1C`, `+0x24` and `+0x2C` of `r3`. Three of those stores
  are `psq_st ...,1,0`, i.e. one lane only, which is why the block has three
  zero entries (the homogeneous column). The export's `dVar12 = dVar17 - dVar17`
  is the `ps_sub f7, f0, f0` zero pair; the remaining stores map one-for-one to
  the export's `param_4[i]` assignments. The block is reproduced literally and
  its role is `unknown:` - with all three angles zero it is not orthonormal, so
  calling it a rotation matrix would be a guess.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80296260` | `fn_80296260` | 332 | medium |
| `0x802963B0` | `fn_802963b0` | 376 | medium |

## Deliberate skips

Four entries of the bucket stay untranslated. Each was re-read from
`functions.tsv`, from its export and from `work/dol/main.elf` before being left
open. None of them is blocked by the GQR type question any more; all four are
cases where the shipped export no longer describes the function.

| address | bytes | export lines | dropped blocks | why |
|---|---:|---:|---:|---|
| `0x80292C90` | 5,464 | 3,425 | 106 | 48 `in_fN` placeholders and 38 `pcVar` indirect-call temporaries: the export invents the callees, so the dispatch chain cannot be attributed to real slots. |
| `0x80295C20` | 1,232 | 1,747 | 66 | Prologue spills `in_f31`/`in_ps31_1` and models `bl 0x802DCD74` as returning a 64-bit value that it then splits into two *pointers* (`or r27,r3` / `or r28,r4`). |
| `0x80299840` | 6,316 | 1,404 | 18 | 127 `in_fN` placeholders plus 3 `extraout_` values: the entry register set is lost, and 0x228 bytes of stack locals are read before any proven write. |
| `0x8029F900` | 3,392 | 1,424 | 38 | 56 `in_fN` placeholders and 9 `pcVar` temporaries; the export also allocates a 0x458-byte frame it never types. |

### `0x80292C90` - the `0x80290000` dispatch body

The largest unclaimed entry in the bucket (5,464 bytes, running from
`0x80292C90` to the already-translated `0x802941F0`). The export contains 147
`unaff_GQR` references, 38 `pcVar`-typed temporaries and 166 `undefined4`
locals. Its first block alone reads a descriptor, walks a `+0x24` sub-object
and dispatches through two indirect slots whose targets the export never
resolves. Reconstructing it would mean inventing both the slot numbers and the
callee identities, which is exactly what the lane brief forbids.

### `0x80295C20` - the basis orthogonaliser

Wave 113 cited this address by name (the "basis orthogonaliser" called from
`0x80288E30`), so the raw prologue was re-read this pass:

```
80295c20: stwu  r1,-112(r1)
80295c38: bl    0x802DCD74
80295c3c: lfs   f0,0(r3)        ; lfs f1,16(r3) ... lfs f0,32(r3)
80295c48: or    r27,r3,r3
80295c50: or    r28,r4,r4
80295c5c: lfs   f3,-17280(r2)   ; 0x8042ED00 = FLT_EPSILON
80295c74: fcmpo 0,f1,f3
80295c78: bc    4,1,0x80295FBC  ; early out when the first column is tiny
```

The shape is a Gram-Schmidt orthonormalisation of a 4x4 matrix column triple,
but the export loses the function signature: it spills `in_f31`/`in_ps31_1`
(registers it cannot name) and rewrites the `bl` as a `longlong` return whose
high half it uses as the input pointer and low half as the output pointer. Sixty
six "removing unreachable block" warnings remove the lane-split arms that carry
the actual orthogonalisation, so the body cannot be reconstructed without
guessing.

### `0x80299840` and `0x8029F900` - the two large emitters

Wave 111 deferred both as "large per-vertex emitters" and nothing in this pass
changed that. Their exports keep the call sequence but lose the entry register
set: `0x80299840` opens a `0x228`-byte frame and reads 127 untracked float
inputs, `0x8029F900` opens a `0x458`-byte frame, reads 56 untracked float
inputs and keeps nine `pcVar` indirect calls. A body would have to guess the
argument registers, which the evidence does not support.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 708 bytes:
  332 + 376).
- Every evidence path exists under `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 332 translated entries) plus the
  two shared helpers `band_reduce_period` and `band_table_sincos`.
- Neither address was in `ledger/functions.csv` before the pass; both were
  confirmed absent.
- `node tools/check-lane-csv.mjs wave115` reports `OK` for
  `wave115-gx-band.csv` (2 rows).
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings at lines
  23277/23324/23422 belong to the wave-107 body `fn_8028cee0` and predate this
  pass.
- Constants were read from `work/dol/main.elf` with `r2 = 0x80433080`:
  `0x8042ECF0` = `65536.0`, `0x8042ECF8` = `40.7436638`, `0x802F74A0` row 0 =
  `(0.0, 1.0, 0.024541, -0.000301)`.
- The opcode decode used the PowerPC paired-single encoding: exact `SUBOP10`
  entries for `ps_neg` (40), `ps_abs` (264), `ps_nabs` (136), `ps_mr` (72),
  `ps_merge00/01/10/11` (528/560/592/624) and `ps_cmpu0/1`, `ps_cmpo0/1`; and
  the `(fill << 5) + op` forms for `ps_muls0/1` (12/13), `ps_madds0/1` (14/15),
  `ps_sub` (20), `ps_add` (21), `ps_rsqrte` (26), `ps_msub` (28), `ps_madd`
  (29). Every instruction in both new bodies was checked against that map, which
  is why the lane pairing (`ps_muls0` multiplying by lane 0, `ps_madds1` by
  lane 1) is stated as fact rather than inferred.
