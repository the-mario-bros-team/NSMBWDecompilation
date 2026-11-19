# Coverage - Wave 110 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 110 pass over the last open entries of the
`0x80280000..0x8028FFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave110-gx-band.csv` (three rows).

`status` is `translated` for all three rows. `confidence` is `medium`: every
guard, offset, constant and call was re-read from `work/dol/main.elf`, but the
four roles inside the sampler (`point_to`, `point_from`, `delta`, `direction`)
are inferred from instruction order rather than from a symbol, and the matrix
helper `0x801C1160` stays opaque.

## Why these three could be translated now

The bucket had ten open entries after wave 109. Five of them are the shape
samplers that `0x80290CF0` dispatches to through `state+0x29`; wave 109 had
called all five "opaque". They are not opaque: they are the same body as the
two entries wave 109 translated, and their paired-single code only ever touches
GQR0.

The check is mechanical. Decoding every `psq_l` (`opcode 56`) and `psq_st`
(`opcode 60`) operand in the ten open entries gives:

| entry | `psq_l` | `psq_st` | `I` values | verdict |
|---|---:|---:|---|---|
| `0x80287C10` | 6 | 4 | 0 | GQR0 only -> float |
| `0x80288E30` | 4 | 4 | 0 | GQR0 only -> float |
| `0x80289410` | 0 | 1 | 0 | GQR0 only -> float |
| `0x8028A6F0` | 0 | 1 | 0 | GQR0 only -> float |
| `0x8028B6C0` | 0 | 1 | 0 | GQR0 only -> float |
| `0x8028E890` | 25 | 13 | 0 | GQR0 only -> **translated here** |
| `0x8028ED30` | 25 | 13 | 0 | GQR0 only -> **translated here** |
| `0x8028F2D0` | 25 | 13 | 0 | GQR0 only -> **translated here** |
| `0x8028F860` | 43 | 27 | 0 | GQR0 only -> float |
| `0x8028FEA0` | 49 | 33 | 0 | GQR0 only -> float |

`0x801AB150` leaves GQR0 at plain float (type `0`, scale `0`), so every
`bVar == 4/6` (char) and `bVar == 5/7` (short) ladder in these exports is
unreachable and the `else` arm is the whole translation. The `ldexpf(...)`
calls Ghidra prints are the same quantiser expression with a scale of one; the
`mtspr` in every epilogue decodes to `mtlr r0`, not to a GQR write.

The constants the family uses are all in the same `sdata2` window and were
dumped from `main.elf` with `r2 = 0x80433080`:

| address | value | role |
|---|---|---|
| `0x8042EC28` | `0.0` | sign probe in the crossing test |
| `0x8042EC2C` | `2.0` | "no hit" sentinel returned in `f1` |
| `0x8042EC30` | `1.1920929e-7` (`FLT_EPSILON`) | division guard / epsilon nudge |
| `0x8042EC34` | `1.0000001192` (`1 + FLT_EPSILON`) | unit-square limit (`0x8028ED30`) |
| `0x8042EC38` | `1.0000002384` (`1 + 2*FLT_EPSILON`) | unit-disc limit (`0x8028F2D0`) |
| `0x8042EC20` | `-1.0` | mirror factor for flag `2` |

A count of the `lfs` displacements inside each body backs this up: `0x8028ED30`
loads `0x8042EC34` exactly six times and `0x8028F2D0` loads `0x8042EC38` exactly
six times - one per compute arm (three axes times the two crossing
directions) - and neither loads any other limit.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 3 | 4,020 | 0 | 3 |

## What this pass covers

- **The shared sampler skeleton** (`sample_shape`). The three bodies are one
  function. Each call: reads the axis selector at `state+0x2A` (values above 2
  return the `2.0` sentinel), maps it to the tested component
  `axis = (selector + 1) % 3`, rejects the endpoint pair unless
  `FLT_EPSILON <= |from[axis]|` or `FLT_EPSILON <= |to[axis]|`, accepts only a
  sign change (`from > 0` with `to < eps`, or `from < 0` with `to > eps`) and
  otherwise returns `2.0`, computes `t = -from[axis] / delta[axis]`, lerps all
  three components of the `point_from -> point_to` segment with that single
  parameter, stores them into the caller's three-float sample buffer, applies
  its in-plane rejection, mirrors `direction[axis]` and `delta[axis]` by `-1.0`
  and nudges the hit point by one epsilon step when `state+0x2C & 2` is set,
  and finally calls `0x801C1160(matrix, sample, out)`.
- **Selector 0, `0x8028E890`.** The plain member: no in-plane rejection, so the
  matrix call always runs when the segment crosses the plane.
- **Selector 1, `0x8028ED30`.** Adds the unit-square test
  `1 + FLT_EPSILON <= |sample[a]| || 1 + FLT_EPSILON <= |sample[b]|`, where
  `(a, b)` are the two components in the plane of the crossing axis
  (`(1,2)`, `(0,2)`, `(0,1)` for selectors `0`, `1`, `2`). A rejected sample
  returns `2.0` and never reaches `0x801C1160`.
- **Selector 2, `0x8028F2D0`.** Same, but the test is the squared in-plane
  length `1 + 2*FLT_EPSILON <= a*a + b*b`, i.e. the unit disc instead of the
  unit square.

The only differences between the three exports after normalising the
`GQR` ladders are exactly those guard lines and the constant they load; a
token-level diff of `0x8028ED30` and `0x8028F2D0` reports six differing
statements, the six `if (fVar4 <= ...)` tests.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8028E890` | `fn_8028e890` | 1172 | medium |
| `0x8028ED30` | `fn_8028ed30` | 1436 | medium |
| `0x8028F2D0` | `fn_8028f2d0` | 1412 | medium |

## Deliberate skips

Seven entries of the bucket stay untranslated. They are kept separate by
reason so a later pass knows which ones are cheap.

### Deferred: sphere samplers, two roots and a cap fallback

| address | bytes | why |
|---|---:|---|
| `0x8028F860` | 1,600 | selector `4`. Solves `|p4 + t*p5|^2 = 1` (`disc = 4*dot^2 - 4*|p5|^2*(|p4|^2 - 1)`; the root uses `0x80237BA0`, the reciprocal-square-root helper, whose product with the discriminant is the square root), accepts a root only when its two in-plane components satisfy `< 1 + 2*FLT_EPSILON` and when component 1 passes the `state+0x2A` cap test, then keeps the smaller accepted root. A later arm builds a tangent frame out of the sample, rounds it through `0x801C07A0` and re-transforms the direction; the cap byte and the frame layout are not yet proved. |
| `0x8028FEA0` | 2,024 | selector `5`. Same quadratic, solved once per plane (`0x80237BA0` three times, always multiplied by its argument) with the smallest valid `t` kept across all three axes, plus the same tangent-frame tail. |

Both bodies are reconstructible in principle - they are GQR0-only, so no
paired-single knowledge is missing - but the root-pair selection, the per-axis
acceptance and the tangent-frame tail need a careful read that this pass did
not have the budget for. They remain callable by address (both already have
constants in `src/render/gx_band.rs`) and their `# Original` evidence is
`work/ghidra/main-gb/export/8028f860_FUN_8028f860.c` and
`work/ghidra/main-gb/export/8028fea0_FUN_8028fea0.c`.

### Deferred: keyframe track samplers

| address | bytes | why |
|---|---:|---|
| `0x80289410` | 4,792 | six-channel byte keyframe sampler. The frame-index half is exactly [`track_frame_index`], but the body then runs a binary search over the key-table offsets, an LCG dither seeded from a hash of `(index, descriptor, channel)`, and three per-key interpolation modes (linear, cubic, step). The `0x3f81f635`/`0x30a74193`/`0x7b929`/`0x371097e7` hash multipliers and the `0x343FD`/`0x269EC3` LCG are literal, but the mode-1 cubic and the two-frame combination still need a full pass. |
| `0x8028A6F0` | 4,012 | same family, another channel width; same open questions. |
| `0x8028B6C0` | 4,280 | same family, another channel width; same open questions. |

Their shared descriptor layout is already documented on [`track_frame_index`]
and their range driver `0x8028CEE0` is translated, so a later pass can start
from the existing helpers.

### Deferred: call-heavy state emitters

| address | bytes | why |
|---|---:|---|
| `0x80287C10` | 1,908 | particle spawn path. It reads a 3x2 fixed-point position pair out of `param_4+0xAC` and `+0xB8`, but then spends most of its body on virtual slots (`+0x1C -> +0x18/+0x24`, `+0xC8` chain), the `0x802891C0` dirty walk and five `0x802963B0`/`0x80295A90` calls. Ghidra also drops the third argument of several `0x80238200` calls, so reproducing the call sequence exactly needs the raw instructions. |
| `0x80288E30` | 572 | 3x4 matrix compose: identity (`0x801C04D0`), optional scale/rotate parts via `0x80295C00`, `0x80295970`, `0x80295C20`, `0x80296530`, and a mode byte at `param_5`. Three `0x80238200` calls print with only two of their arguments, and the `param_5 == 'd'` arm is a straight 11-word copy, so the function's real signature is not yet proved. |

`0x80288E30` is the cheapest of the seven to finish: it is short, has no
paired-single data path beyond two register saves, and only needs the
`0x80238200` signature recovered from the raw words.

## Verification

- Every address and byte size matches `functions.tsv` (3 rows, 4,020 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 321 translated entries).
- None of the three addresses was in `ledger/functions.csv` before the pass.
- `node tools/check-lane-csv.mjs wave110` reports `OK` with
  `wave110-gx-band.csv: 3 rows`.
- `cargo check --message-format=short` reports no error and no warning from this
  pass. The three `unused_assignments` warnings at `src/render/gx_band.rs`
  lines 23216, 23263 and 23361 belong to the wave-107 body `fn_8028cee0` and
  predate it.
- The `psq` operand inventory above was produced by decoding every word of the
  ten entries: opcodes 56/57/60/61, with `I = (word >> 12) & 7`.
- The constant table was dumped from `work/dol/main.elf` at
  `0x8042EC20..0x8042EC38`, and the `lfs` load counts per body were read from
  the same image.
