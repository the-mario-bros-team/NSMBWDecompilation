# Coverage - Wave 121 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 121 pass over the last two open entries of the
`0x80280000..0x8028FFFF` bucket of the GX/render lane (`src/render/gx_band.rs`).
Addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave121-gx-band.csv` (two rows, both
`translated`).

The bucket held two untranslated entries when this pass started:

| address | bytes | state after this pass |
|---|---:|---|
| `0x8028F860` | 1,600 | **landed** - sphere sampler (dispatch selector `4`) |
| `0x8028FEA0` | 2,024 | **landed** - clipped-cylinder sampler (dispatch selector `5`) |

The `0x80280000` bucket is now closed (158 of 158 entries translated).

## The finding that re-opened the bucket

Wave 119 left both entries open because "the export drops flow". That is an
export artefact, not missing information. Re-reading `work/dol/main.elf`
instruction by instruction settles everything:

- **The pseudo-return is the `__savegprlr_24` thunk.** Both exports call
  `FUN_802dcd68()` and split its 64-bit result into the matrix and the state
  pointer. `0x802DCD68` is the CodeWarrior register-save entry: it only stores
  `r24..r31` and returns through `blr`, so `r3` (matrix) and `r4` (state) are
  untouched. The bodies below use the incoming arguments directly.
- **Every paired-single operand is GQR0.** All 70 `psq_l`/`psq_st` sites in
  `0x8028F860..0x8028FEA0` and all 82 in
  `0x8028FEA0..0x80290688` carry `I = 0`; `0x801AB150` leaves GQR0 at plain
  float, so each pair is a two-lane `f32` register and every `ldexpf`/`CONCAT`
  ladder in the exports is dead.
- **The two entries are the sphere/cylinder pair.** The quadratic of
  `0x8028F860` is built from `|P|^2`, `|D|^2` and `P.D` with
  `c = |P|^2 - 1`, i.e. the unit sphere. `0x8028FEA0` uses only the `x`/`z`
  components, i.e. the infinite cylinder `x*x + z*z = 1`, and then clips it
  with `y = +-1` cap arms. This matches the classifier pair it is dispatched
  with: `0x8028E310` (unit-sphere classifier, selector `4`) and
  `0x8028E510` (unit-cylinder classifier, selector `5`).
- **The plane sampler family's hit contract is `t <= 1.5`, not `t <= -1`.**
  `fn_80290cf0`'s threshold was transcribed from the wrong SDA2 slot
  (`0x8042EBD0`, `-1.0`). The raw compare at `0x80290F0C` loads
  `0x8042EC50` (`1.5`) and the `bc 4,1` arm is `ble`, so a sampler hit is
  `result <= 1.5`; `2.0` (`0x8042EC2C`) is the miss sentinel. The threshold and
  the doc block are corrected in this pass (see "Corrections").
- **The constant block used by both bodies** (dumped from `main.elf`,
  `r2 = 0x80433080`): `0x8042EC20 = -1.0`, `0x8042EC24 = 1.0`,
  `0x8042EC28 = 0.0`, `0x8042EC2C = 2.0`, `0x8042EC30 = FLT_EPSILON`,
  `0x8042EC34 = 1 + FLT_EPSILON`, `0x8042EC38 = 1 + 2*FLT_EPSILON`,
  `0x8042EC3C = 4.0`, `0x8042EC40 = -2.0`, `0x8042EC44 = -FLT_EPSILON`,
  `0x8042EC48 = 1.00001`, `0x8042EC4C = 0.99999`, `0x8042EC50 = 1.5`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 3,624 | 0 | 2 |

## What this pass covers

- **The unit-sphere sampler `0x8028F860`** (1,600 bytes, `medium`). It solves
  `|P + tD|^2 = 1` with `disc = 4b^2 - 4a(|P|^2 - 1)`, returns `2.0` for a
  negative discriminant and evaluates `t0 = (-2b - sqrt)/(2a)` before
  `t1 = (-2b + sqrt)/(2a)`. Each root needs `0 < t < 1 + FLT_EPSILON` and a hit
  point on the `state+0x2A` side (`1` = `y >= -eps`, `2` = `y <= +eps`); `t1`
  must also beat the current best. When the selector is `1` or `2` and the
  segment is not parallel to `y = 0`, the body also intersects the `y = 0`
  plane, accepting `t < 2.0` and `x*x + z*z < 1 + 2*FLT_EPSILON`, with a
  `+eps` nudge for a `+y -> -y` crossing and `-eps` for the reverse. On a hit
  and `state+0x2C & 2`, a local `4x3` frame is built from the hit's `z`,
  inverted in place (`0x801C07A0`), and `delta`/`direction` are projected
  through it and reduced by twice the hit component before `sample` is scaled
  by `0.99999` (`t <= 1.0`) or `1.00001` and transformed by `0x801C1160`.
- **The clipped-cylinder sampler `0x8028FEA0`** (2,024 bytes, `medium`). The
  quadratic is the `xz` projection of the same solve. Side roots need
  `0 < t < 1 + FLT_EPSILON` and `|sample.y| < 1 + FLT_EPSILON`; the two cap
  arms then test `y = +1` (skipped when both endpoints already sit on that
  plane) and `y = -1`, each with one arm for a crossing from above and one from
  below, both requiring `t < best` and `x*x + z*z < 1 + 2*FLT_EPSILON`. A cap
  hit returns immediately after the `0x801C1160` transform. On a side hit with
  `state+0x2C & 2`, the body projects `delta` and `direction` onto the
  normalised `(x, 0, z)` basis of the hit, subtracting twice, then applies the
  same `0.99999`/`1.00001` scale. `0x801C1470` is the shipped three-float
  normaliser; the preceding `0x80237BA0` rsqrt call's result is unused in the
  original stream and is preserved as an observable call.

The two entries are a pair: the sphere sampler is selector `4` of the
`0x80290CF0` dispatch, the cylinder sampler selector `5`, and both share the
`(matrix, state, point_to, point_from, delta, direction, sample, out)` call
shape of the three plane samplers translated in wave 110.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8028F860` | `fn_8028f860` | 1600 | medium |
| `0x8028FEA0` | `fn_8028fea0` | 2024 | medium |

## Corrections made in this pass

- `fn_80290cf0`: the hit guard now reads `0x8042EC50` (`1.5`) instead of
  `0x8042EBD0` (`-1.0`). The raw compare at `0x80290F0C..0x80290F20` is
  `lfs f0,-0x4430(r2)` + `fcmpo cr0,f31,f0` + `bc 4,1` (`ble`), which is
  `result <= 1.5 -> hit`. The export prints the same `-0x4430` slot. Without
  this fix the two new samplers (which return positive parameters) would never
  be treated as hits. The `# Returns` and `# Confidence` blocks were updated.
- The two selector constants `FN_8028F860` / `FN_8028FEA0` no longer say
  "still untranslated"; the wave-111 note that selector `5` "solves the same
  quadratic three times" is corrected: it solves the `xz` quadratic once and
  then walks two cap planes.

## Deliberate skips

None in this bucket. Both remaining entries were reconstructible from
`work/dol/main.elf` and are landed. The lane's open remainder is outside this
bucket: `0x80292C90`, `0x8029F900`, `0x802A71D0` and `0x802A90C0` (the first
three were documented in waves 120/119; `0x802A90C0` in wave 119).

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 3,624 bytes).
- Every evidence file exists at the cited path:
  `work/ghidra/main-gb/export/8028f860_FUN_8028f860.c` and
  `work/ghidra/main-gb/export/8028fea0_FUN_8028fea0.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`; the module now holds 341 reconstructed entries
  (340 full, one partial).
- Neither address was in `ledger/functions.csv` before the pass (the whole
  `0x80280000` bucket was re-read: 158 entries, 156 already translated, these
  two open).
- `node tools/check-lane-csv.mjs wave121` reports no problem naming this CSV.
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings in the
  wave-107 body `fn_8028cee0` predate this pass.
- Both bodies were transcribed from `work/dol/main.elf`; every cited branch,
  constant and store was checked against the raw stream, and the register map
  (`r27` matrix, `r24` state, `r25` point_to, `r26` point_from, `r28` delta,
  `r29` direction, `r30` sample, `r31` out) was read from the two prologues.
