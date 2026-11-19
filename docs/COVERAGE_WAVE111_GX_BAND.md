# Coverage - Wave 111 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 111 pass over the last ten open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane (`src/render/gx_band.rs`).
Addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave111-gx-band.csv` (two rows).

`status` is `translated` for both rows and `confidence` is `medium`: every
guard, offset, constant and call was re-read from `work/dol/main.elf`, but the
roles of the vector pointers are inferred from instruction order, and the two
matrix helpers `0x801C07A0` / `0x801C0C10` stay opaque.

## What this pass covers

Both translated entries sit on the shape-hit path that
[`fn_80290cf0`](../src/render/gx_band.rs) drives, and both are provably
`GQR0`-only.

- **Unit-box face sampler `0x80290690`** (selector `3` of the dispatcher). Wave
  109 described this entry as "clamp `(c - *param_4) / *param_5` and emit
  per-channel attributes". That is only the first line of it: the body walks
  all six faces of the axis-aligned unit box. For each axis it evaluates the
  `+1` face first and the `-1` face second (order `+x`, `-x`, `+y`, `-y`, `+z`,
  `-z`), computes `t = (level - from[axis]) / delta[axis]`, accepts only
  `0.0 < t <= 1 + FLT_EPSILON`, lerps all three components of the segment,
  rejects the face unless both in-plane components stay within
  `1 + FLT_EPSILON` (pairs `(1,2)`, `(0,2)`, `(0,1)` for axes `0`, `1`, `2`),
  records the face parameter and keeps the smallest one. On a hit it lerps the
  winning point into `sample`, calls `0x801C1160(matrix, sample, out)` and, when
  `state+0x2C & 2` is set, mirrors `direction`/`delta` on every axis whose face
  parameter is within `FLT_EPSILON` of the winner and snaps `sample[axis]` to
  `+1.0`, then `-1.0` (so the `-1` face wins a tie). A miss returns the `2.0`
  sentinel untouched.
- **Particle step `0x80291100`** (ten arguments). Identity, row scale from the
  nine-float transform record, three-angle rotation, translation, an optional
  second translation gated by `state+0x2C & 1`, and the parent matrix produce
  the emitter's world matrix; `0x801C07A0` produces its inverse. The probe point
  `param_7` is moved into that space, classified with the same six-classifier
  family as the dispatcher (selector `state+0x2A` in `r4`), and the result is
  written to `entity+0x9D`. The bounds rule at `state+0x24`/`+0x28` then runs on
  `direction`. Unless the path byte `state+0x2B` is `1` or `2`, the body then
  traces at most five segments: the target is
  `position + entity+0xC4 * (velocity + direction)` and every iteration calls
  `0x80290CF0` for the segment `point -> target`, skipping the call when the two
  points are closer than `2^-46` (squared, `0x8042EC54`). A hit copies the
  sampler's world and local vectors into `entity+0xAC` and `entity+0xA0`, sets
  bit `0x80` of `entity+0x9D`, promotes the sampler's sample buffer to the next
  `point` and re-derives `target`/`heading`. A miss re-applies the bounds rule
  to `entity+0xA0`; the first miss also runs the two tail dispatches when the
  classifier transition looks like a fresh entry.

The two bodies are also a matched pair: `0x80290690` is the sampler that fills
the `channel_arg` buffer that `0x80291100` promotes to the next segment
endpoint, which is why the former's `sample` pointer and the latter's
`channel_arg` slot had to be identified together.

## Evidence method

Wave 109's split by `psq` operand `I` field was re-run over the ten entries and
reproduced; the two translated entries decode as follows from
`work/dol/main.elf`:

| entry | `psq_l` | `psq_st` | `I` values | `bl` | verdict |
|---|---:|---:|---|---:|---|
| `0x80290690` | 31 | 19 | `0` | 1 | GQR0 only -> translated |
| `0x80291100` | 13 | 8 | `0` | 30 | GQR0 only -> translated |

The `lfs` displacement counts pin the body shapes. `0x80290690` loads
`1.0` (`0x8042EC24`) six times, `0.0` (`0x8042EC28`) six times,
`1 + FLT_EPSILON` (`0x8042EC34`) six times, `-1.0` (`0x8042EC20`) nine times,
`FLT_EPSILON` (`0x8042EC30`) nine times and the `2.0` sentinel
(`0x8042EC2C`) twice: six faces, six range tests, six in-plane tests, six
recorded parameters, three mirror pairs, three `-1.0` coordinate snaps and the
two sentinel initialisations. `0x80291100` loads only `0.0` three times and
`0x8042EC54` once (the `2^-46` minimum-step guard).

The argument registers of the `0x80290CF0` call inside `0x80291100` were read
from the raw stream at `0x802916F0..0x8029173C`: `r3 = r1+0x150` (world
matrix), `r4 = r1+0xF0` (its 3x4 copy), `r8 = r1+0x74` (point_from),
`r9 = r1+0x68` (point_to), `r10 = r1+0x20` (direction), and the five trailing
arguments in the parameter save area at `r1+8..r1+24`. The classifier switch
at `0x802912DC` starts with `lbz r0, 41(r24)` and `lbz r4, 42(r24)`, so all six
classifiers share the `state+0x2A` byte even though Ghidra prints only one
argument for four of them.

Constants dumped from `main.elf` with `r2 = 0x80433080`:

| address | value | role |
|---|---|---|
| `0x8042EC20` | `-1.0` | `-1` face level / mirror factor / `-1` snap |
| `0x8042EC24` | `1.0` | `+1` face level / `+1` snap |
| `0x8042EC28` | `0.0` | lower `t` bound / zero vector fill |
| `0x8042EC2C` | `2.0` | "no hit" sentinel |
| `0x8042EC30` | `FLT_EPSILON` | mirror / snap epsilon test |
| `0x8042EC34` | `1 + FLT_EPSILON` | upper `t` bound and unit-square limit |
| `0x8042EC54` | `2^-46` | squared minimum step between probe points |

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 3,836 | 0 | 2 |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80290690` | `fn_80290690` | 1628 | medium |
| `0x80291100` | `fn_80291100` | 2208 | medium |

## Deliberate skips

Eight entries of the bucket stay untranslated, for three different reasons.

### Blocked: `GQR3`-dependent samplers (not reconstructible as shipped)

| address | bytes | why |
|---|---:|---|
| `0x80296160` | 116 | writes the wrapped angle through `psq_st ...,1,3`, reads it back through `psq_l ...,1,3`, then indexes the 16-entry table `0x802F74A0` by the low byte; width and scale both come from live `GQR3` |
| `0x802961E0` | 120 | same sampler with the pair split across the two output pointers |
| `0x80296260` | 332 | three-angle matrix builder; quantises its angles and table rows through `GQR3` before the lerp |
| `0x802963B0` | 376 | three-angle rotation-matrix writer; same `GQR3` quantisation of the input angles |

These were re-checked in wave 109 and again here: the `I = 3` operands are real,
`GQR3` is installed per vertex format at run time (`0x8025EEC0`, `0x802AB50C`,
`0x802C2770`), and Ghidra itself cannot choose between a `char`, a `short` and a
`float` store for the same word. Any transcription would have to invent the
width and scale. Both `0x802961E0` and `0x802963B0` remain callable by address
(they are used by `0x80291100` and `0x80296820` respectively).

### Deferred: the 3x4 basis orthogonaliser

| address | bytes | why |
|---|---:|---|
| `0x80295C20` | 1,232 | takes a 3x4 matrix in `r3` and writes an orthonormal basis to `r4`: it measures the squared length of the first column (`m[0]`, `m[4]`, `m[8]`) against `FLT_EPSILON` (`0x8042ED00`), zeroes the column when it is degenerate (`0x8042ED04` = `0.0`), otherwise normalises it through `0x80237BA0`/`0x801C14C0`, and rebuilds the second and third columns with two nested arms (a `sqrt` path and a `1/sqrt` path). It is `GQR0`-only and reconstructible, but the column order, the degenerate thresholds and the `0x801C14C0` call shape need a full pass that this one did not have the budget for. |

### Deferred: the large per-vertex emitters

| address | bytes | what it does |
|---|---:|---|
| `0x80292C90` | 5,464 | the largest sampler in the bucket (142 `psq` sites). A per-vertex walk over a `0x40`-byte record: it calls `0x80288E30` for the local matrix, `0x80289070` for the ambient matrix, all three keyframe samplers (`0x8028B6C0`, `0x80289410`, `0x8028A6F0`), `0x80296260`/`0x80296160`/`0x802961E0` for the tangent frame, and finally `0x80291100` for the trace. It cannot be written before the three keyframe samplers are. |
| `0x80299840` | 6,316 | nested sphere sampler: `param_4 * param_4` loops over a latitude/longitude grid with six near-identical side blocks, each re-deriving a normal through `0x802E8590`/`0x80237BA0` and calling `0x80299510`. Reconstructible but long, and five of its local vectors are unnamed in the export. |
| `0x8029F900` | 3,392 | per-vertex attribute emitter: a `do` loop over the vertex list with a `GX` format ladder on `u16 & 0x6000`, four virtual slots on the material, and `0x8029BAB0` / `0x8029F4A0` FIFO writes. The format ladder decides which attributes exist, so it needs the raw stream rather than the `ldexpf` export. |

Their evidence files already exist at
`work/ghidra/main-gb/export/80295c20_FUN_80295c20.c`,
`80292c90_FUN_80292c90.c`, `80299840_FUN_80299840.c` and
`8029f900_FUN_8029f900.c`, so a later pass can start from them directly.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 3,836 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one body in `src/render/gx_band.rs` (the module now
  holds 323 translated entries).
- Neither address was in `ledger/functions.csv` before the pass; both were
  confirmed absent from it and from the wave-111 sibling CSVs.
- `node tools/check-lane-csv.mjs wave111` reports `OK` for
  `wave111-gx-band.csv` (2 rows).
- `cargo check --message-format=short` reports no error and no warning from this
  pass. The three `unused_assignments` warnings at `src/render/gx_band.rs` lines
  23216, 23263 and 23361 belong to the wave-107 body `fn_8028cee0` and predate
  it.
- The `psq` operand inventory and the `lfs` counts above were produced by
  decoding every word of the two entries in `work/dol/main.elf` (opcodes
  56/57/60/61 with `I = (word >> 12) & 7`, and opcode 48 with `rA = 2`).
