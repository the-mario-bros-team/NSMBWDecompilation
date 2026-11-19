# Coverage - Wave 118 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 118 pass over the last open entries of the
`0x80290000..0x8029FFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave118-gx-band.csv` (one row).

The bucket held 85 entries, four of them untranslated, when this pass started
(`0x80292C90`, `0x80295C20`, `0x80299840`, `0x8029F900`). One is landed here;
the other three are documented under "Deliberate skips" with the evidence that
is still missing for each. The lane as a whole now holds 336 translated entries.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 1 | 1,232 | 0 | 1 |

## What this pass covers

- **The basis-length extractor `0x80295C20`** (1,232 bytes, `medium`). Reads the
  three basis columns of a row-major 4x4 (`(m[0], m[4], m[8])`,
  `(m[1], m[5], m[9])`, `(m[2], m[6], m[10])`), runs Gram-Schmidt over them and
  writes three floats: `|col0|`, `|col1 - n0*(n0.col1)|` and
  `|col2 - n1*(n1.col2) - n0*(n0.res2)|`. A residual whose squared length is not
  greater than `FLT_EPSILON` (`0x8042ED00`) stores `0.0` (`0x8042ED04`) instead.
  Only the fully non-degenerate path reaches the handedness test: it calls
  `0x801C1530` with `(n1, res2)`, and when `n0 . cross(n1, res2) < 0` it
  multiplies all three stored values by `-1.0` (the double at `0x8042ED10`).
  **The matrix is never written**: of the 71 stores in the entry, none uses
  `r27` (the input pointer) as its base, so the wave-115 note that called this
  entry a "basis orthogonaliser" writing a matrix was wrong. The only caller,
  `0x80288E30`, passes the result to the matrix scale helper `0x80296530`,
  which reads exactly three floats, so `out` is a 3-float scale vector.

### Why this entry needed the raw stream

The Ghidra export of `0x80295C20` cannot be read as shipped:

1. The prologue ends in `bl 0x802DCD74`, which is a register-save helper
   (`__savegprlr_27`-shaped; its export stores `r27..r31` at `r11-0x14..r11-4`
   and returns nothing). Ghidra models the call as a `longlong` return and then
   uses the high half as the input pointer and the low half as the output
   pointer (`pfVar15` / `pfVar16`).
2. It removes 66 blocks as "unreachable". All of them are unused arms of the
   `psq_l`/`psq_st` type ladders: every live operand in the entry carries
   `I = 0`, i.e. GQR0, which the init path at `0x801AB150` pins to float.

With those two artefacts removed, the export's arithmetic collapses to the
Gram-Schmidt sequence above, and the raw stream
(`work/dol/main.elf` at `0x80295C20..0x802960F0`) gives the operand-level
detail the body cites:

| step | raw evidence |
|---|---|
| column 0 length | `ps_mul`/`ps_madd`/`ps_sum0` at `0x80295C68..0x80295C70`, compared with `0x8042ED00` at `0x80295C74` |
| `n0` | `bl 0x80237BA0` + `fres` + `ps_add`/`ps_mul`/`ps_nmsub` at `0x80295C7C..0x80295CC0` |
| `dot(n0, col1)` | `0x80295DB8..0x80295DC8` |
| `res2` re-projection | `ps_sub f0,f2,f1` at `0x80295D1C` and the second `dot(n0, res2)` at `0x80295E04..0x80295E0C` |
| handedness | `bl 0x801C1530` at `0x80295E8C`, `fcmpo` + `bc 4,0` at `0x80295EB8`, three `fmul` by `-1.0` at `0x80295ED0..0x80295EF0` |

Two details are modelled rather than copied literally:

- `fres` (the hardware reciprocal *estimate*) has no host equivalent, so
  `out[0]`/`out[1]` use an exact `1.0 / inv` seed and then the same Newton step
  `2a - len2*a*a` the stream performs. `out[2]` uses the plainer
  `len2 * rsqrt(len2)` the raw stream emits at `0x80295E70`, and the
  `0x801C14C0` call in the first degenerate arm computes the same quantity.
- The redundant second `fcmpo` against `0.0` (`0x80295E60`, `0x80295F90`) is
  unreachable-false after the epsilon test, so the body keeps only the epsilon
  test and says so.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80295C20` | `fn_80295c20` | 1,232 | medium |

## Deliberate skips

Three entries of the bucket stay untranslated. This pass re-read all four open
entries from `functions.tsv`, from their exports and from the raw stream, and it
corrects the wave-115 rationale for all three of the remaining ones: the
`in_fN` / `in_psN` placeholders in those exports are **not** a lost argument
set. They are the prologue save of the callee-saved floats. Every read of them
is inside the `psq_l` ladder that stores `f14..f31` (for example
`80299840_FUN_80299840.c` lines 160-514, before the first live statement at
line 515), and Ghidra lists them as inputs because `stfd`/`psq_st` read the
registers they save. No live value is missing from those three exports.

| address | bytes | removed blocks | indirect calls | why it stays open |
|---|---:|---:|---:|---|
| `0x80292C90` | 5,464 | 106 | 7 `bctrl` | 106 removed blocks spread across the live body (not just the GQR arms), and 38 `pcVar` temporaries: the virtual slots are read through offsets the export never resolves, so the dispatch chain cannot be attributed without guessing slot numbers. |
| `0x80299840` | 6,316 | 18 | 0 | In principle reconstructible: no indirect calls, and all 18 removed blocks are the epilogue restore at `0x8029B048..0x8029B0D4` (verified in the raw stream), which is harmless. The only cost is volume: the live body runs from `0x802998E4` to `0x8029B048`, i.e. about 1,500 instructions (890 export lines after the save prologue). That is a budget decision, not an export defect - the entry is left for a later pass rather than transcribed unverified. |
| `0x8029F900` | 3,392 | 38 | 6 `bctrl` | 38 removed blocks and six indirect calls through `pcVar` temporaries; the same unresolved-slot problem as `0x80292C90`, on a smaller body. |

### `0x80295C20` and the wave-115 record

`docs/COVERAGE_WAVE115_GX_BAND.md` described this entry as a Gram-Schmidt
"basis orthogonaliser" whose export splits a `bl` return into two pointers.
The second half of that description is correct and is quoted above; the first
half is not: the entry writes three lengths, never a matrix. The wave-113
inline comment in `fn_80288e30`, which repeated the "writes the orthonormal
result to `r4`" claim, has been corrected in place.

## Verification

- Every address and byte size matches `functions.tsv` (1 row, 1,232 bytes for
  `0x80295C20`; the next entry, `0x802960F0`, starts exactly at
  `0x80295C20 + 0x4D0`).
- Every evidence file exists at the cited path:
  `work/ghidra/main-gb/export/80295c20_FUN_80295c20.c`.
- The only row has exactly one `pub unsafe fn fn_80295c20` body in
  `src/render/gx_band.rs`; the module now holds 336 translated entries.
- The address was not in `ledger/functions.csv` before the pass (re-read for the
  whole `0x80290000` bucket: the only open entries were the four listed above).
- `node tools/check-lane-csv.mjs wave118` reports `OK`; the gx-band lane CSV has
  one row.
- `cargo check --message-format=short` reports no error and no new warning from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings now at lines
  23311/23358/23456 belong to the wave-107 body `fn_8028cee0` and predate this
  pass.
- The store-base histogram of the new body was re-read from the raw stream
  (`stw`/`stfs`/`psq_st` bases): 31 stores to `r1`, 13 to `r28` (the output),
  and none to `r27` (the input), which is the evidence for "the matrix is never
  written".
- Constants were read from `work/dol/main.elf` with `r2 = 0x80433080`:
  `0x8042ED00` = `FLT_EPSILON` (u32 `0x34000000`), `0x8042ED04` = `0.0f`,
  `0x8042ED10` = `-1.0` as a double (`0xBFF0000000000000`).
