# Coverage - Wave 112 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 112 pass over the last open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave112-gx-band.csv` (two rows).

Both rows are `translated` with `confidence = medium`. Every guard, constant,
table offset and store was re-read from the raw instruction words in
`work/dol/main.elf`; the single inference is the width of the `GQR3`
quantiser that produces the table index (see "The one inference" below).

## What this pass covers

The bucket had eight untranslated entries when this pass started, all of them
listed as deliberate skips by Wave 111. Two of them are plain table lookups and
are landed here; the other six are still open.

- **Sine/cosine table sample `0x80296160`** (116 bytes). The angle is scaled by
  `sdata2` `0x8042ECF8` = `256 / (2*pi)` into "table units" (256 units per full
  turn), reduced by `65536.0` (`0x8042ED20`) while it is larger than that, and
  then split into a row index and a `0..1` interpolation weight. Row `i` of the
  table at `0x802F74A0` is sixteen bytes and holds
  `(sin_i, cos_i, sin_i+1 - sin_i, cos_i+1 - cos_i)`; row 0 is
  `(0.0, 1.0, 0.024541, -0.000301)`, i.e. the values at `0` and `1/256` turn.
  The result is `row[2]*frac + row[0]` / `row[3]*frac + row[1]`
  (`ps_madds0 f1, f0, f2, f3` with `f2` = the fraction broadcast). A negative
  input angle flips the sine only: `ps_neg` produces `(-sin, -cos)` and
  `ps_merge01 f1, f0, f1` keeps lane 1, which is why the cosine stays even.
  The final `psq_st f1, 0(r3), 0, 0` writes the pair as two floats.
- **Split-output twin `0x802961E0`** (120 bytes). Identical arithmetic; the
  only differences are the output shape and the order. After the interpolation
  `ps_merge10 f2, f0, f0` swaps the lanes so that `psq_st f2, 0(r4), 1, 0`
  writes the cosine to the second pointer, then the conditional `ps_neg` runs
  on the sine and `psq_st f0, 0(r3), 1, 0` writes it to the first pointer. The
  cosine therefore never sees the sign fix, exactly as in `0x80296160`.

The two bodies share one private helper, [`sincos_table`], because the raw
streams are the same code with a different tail; the module already documents
such matched pairs this way (for example `fn_802960f0`).

## The one inference

The table index is rounded through `GQR3`: `psq_st f1, 0(r3), 1, 3` followed by
`lhz` + `rlwinm r0, r0, 4, 20, 27` and `psq_l f2, 0(r3), 1, 3`. The `I = 3`
operands are real, and `GQR3` is installed per vertex format at run time, so its
width and scale are not in the image. What *is* in the image pins the
assignment anyway: one byte of the quantised value is multiplied by the
sixteen-byte row stride to select a table row, and the value read back through
`psq_l` is subtracted from the reduced coordinate to form the interpolation
weight (`fsubs f2, f1, f2`). Both operations are only consistent with an
unsigned eight-bit, zero-scale `GQR3` — any other width would either overflow
the row index or make the "weight" an integer multiple of the row step. The
Rust body therefore applies `trunc` plus a `0..=255` clamp and marks that step
`// inferred:`; everything else in the two bodies is `// observed:`.

The `psq_st ...,1,3` stores that precede the read-back are scratch: the closing
`psq_st ...,0,0` overwrites the same word with the float pair, so only the
final write is reproduced. This is called out in both doc blocks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80296160` | `fn_80296160` | 116 | medium |
| `0x802961E0` | `fn_802961e0` | 120 | medium |

## Deliberate skips

Six entries of the bucket stay untranslated.

### Deferred: the two three-angle matrix builders

| address | bytes | why |
|---|---:|---|
| `0x80296260` | 332 | The same table machinery three times over (all three angles are reduced and looked up through the same `GQR3` byte), then two `ps_merge10` lane swaps and a chain of `ps_mul`/`ps_madds0`/`ps_muls1` products that build three output floats. Ghidra re-uses one `double` per sub-expression across 300 lines, so the register-level dataflow has to be walked by hand; the two extractions `rlwinm r0, r0, 20, 20, 27` and `rlwinm r0, r0, 4, 20, 27` select two different bytes of the packed pair and the pairing of row/weight has to be decided before the algebra can be named. Left for a pass with room to verify the result numerically against the table. |
| `0x802963B0` | 376 | Three-angle 3x4 rotation-matrix writer built from the same lookups. It writes twelve floats, two of which are the literal `0.0` from `fsubs f17, f17`, and mixes `ps_merge01` sign fixes with scalar `fmuls`. Same blocker as `0x80296260`. |

### Deferred: the large reconstruction jobs

| address | bytes | what it does |
|---|---:|---|
| `0x80295C20` | 1,232 | 3x4 basis orthogonaliser. It measures the squared length of the first column against `FLT_EPSILON` (`0x8042ED00`), zeroes the column when degenerate, otherwise normalises it through `0x80237BA0` (the reciprocal-square-root helper this module already calls from `fn_802960f0`) and rebuilds the second and third columns with two nested arms. It is `GQR0`-only, so the export is usable, but the body is 308 instructions of paired-single stack shuffling (`psq_st`/`psq_l` pairs through `r1+8`..`r1+56`) and needs a full pass of its own. |
| `0x80292C90` | 5,464 | The largest sampler in the bucket (142 `psq` sites). A per-vertex walk that calls the local/ambient matrix helpers, all three keyframe samplers, the tangent-frame builders and finally `0x80291100`. It cannot be written before the three keyframe samplers are translated. |
| `0x80299840` | 6,316 | Nested sphere sampler: `param_4 * param_4` loops over a latitude/longitude grid with six near-identical side blocks. Reconstructible but very long; five of its local vectors are unnamed in the export. |
| `0x8029F900` | 3,392 | Per-vertex attribute emitter: a loop over the vertex list with a GX format ladder on `u16 & 0x6000`, four virtual slots on the material and `0x8029BAB0`/`0x8029F4A0` FIFO writes. The ladder decides which attributes exist, so it needs the raw stream rather than the `ldexpf` export. |

Their evidence files already exist at
`work/ghidra/main-gb/export/80296260_FUN_80296260.c`,
`802963b0_FUN_802963b0.c`, `80295c20_FUN_80295c20.c`,
`80292c90_FUN_80292c90.c`, `80299840_FUN_80299840.c` and
`8029f900_FUN_8029f900.c`, so a later pass can start from them directly.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 236 bytes).
- Every evidence path exists under `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 325 translated entries).
- Neither address was in `ledger/functions.csv` before the pass; both were
  confirmed absent from it.
- `node tools/check-lane-csv.mjs wave112` reports `OK` for
  `wave112-gx-band.csv` (2 rows).
- `cargo check --message-format=short` reports no error from this pass; the
  three `unused_assignments` warnings in `src/render/gx_band.rs` belong to the
  wave-107 body `fn_8028cee0` and predate it.
- The constants and the table were dumped from `main.elf` with `r2 =
  0x80433080`: `0x8042ECF8` = `40.7436638` (`256 / 2*pi`), `0x8042ED20` =
  `65536.0`, and `0x802F74A0` = `(0.0, 1.0, 0.024541, -0.000301)`.
