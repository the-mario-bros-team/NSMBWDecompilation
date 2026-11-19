# Coverage - Wave 98 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`. Every function below is a literal
reconstruction of one Ghidra body; the doc block above each body names the
original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv` and the evidence file. Offsets are
relative to the object named in the row.

The `0x80230000` bucket is shared with several subsystems. Waves 85, 90 and 95
closed the BRRES material band, the debug-console band, the exception report and
the message queues. This pass finishes the plainly reconstructible remainder of
the console band and takes the seven *vector-store* members of the packed GQR
group that all three earlier waves had refused.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - the export loses an argument to the Metrowerks prologue, or a
  field role / callee identity had to be reconstructed from a sibling, or the
  live GQR word is supplied as an explicit argument.
- **low** - only the instruction sequence is certain; a varargs descriptor had
  to be modelled rather than reproduced.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 11 | 1,576 | 0 | 10 |

The file now holds **227** translated bodies (216 before this pass). One of the
11 new rows is `low`; no body in the file is a stub.

## What this pass covers

- **Text-box measures** (`0x80230F50`, `0x802310D0`): the two entry points that
  feed the wave-95 measure `0x802322C0`. `0x80230F50` formats the caller's
  arguments into the shared console line buffer at `r13 - 0x4A28`, clamps the
  write count to `capacity - 1`, measures the line on a 0x64-byte snapshot of
  the stream and disposes the snapshot. `0x802310D0` does the same with an
  explicit character run and returns `x1 - x0` in `f1`; its three callers in
  `0x800E5AD0`/`0x800E6680` pass `(stream, index, count)` and use the result as
  a text advance.
- **Console print sinks** (`0x80233480`, `0x802344E0`): the `printf`-shaped
  half of the debug console. `0x80233480` gates on the `0x803DA120` console
  block, formats into a 0x100-byte local and blits the text with `0x80233570`.
  `0x802344E0` is its non-variadic core: console mutex protocol (the
  `countLeadingZeros(state) >> 5` "never contended" test when no threads run,
  otherwise `0x801B2E60`/`0x801B2F40`), a 0x400-byte format into `0x803DA160`,
  an optional echo through `0x8015F730` and an optional print through the
  wave-90 `0x80234120`, selected by two mask bits.
- **Quantised vector stores** (`0x802380A0`, `0x802380D0`, `0x80238110`,
  `0x80238130`, `0x802381A0`, `0x802384B0`, `0x802384F0`): the six bodies that
  only *write* through the paired-single quantisation format, plus the
  3x4 matrix scale. They read `unaff_GQR0` only to select a lane width
  (8-bit, 16-bit or raw single) and a power-of-two scale; both are passed in as
  the `gqr` argument and interpreted by the new `psq_store`/`psq_load` helpers,
  which restate the Ghidra idiom once instead of forty times.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80230F50` | `fn_80230f50` | 384 | medium |
| `0x802310D0` | `fn_802310d0` | 324 | medium |
| `0x80233480` | `fn_80233480` | 228 | low |
| `0x802344E0` | `fn_802344e0` | 216 | medium |
| `0x802380A0` | `fn_802380a0` | 36 | medium |
| `0x802380D0` | `fn_802380d0` | 52 | medium |
| `0x80238110` | `fn_80238110` | 32 | medium |
| `0x80238130` | `fn_80238130` | 100 | medium |
| `0x802381A0` | `fn_802381a0` | 84 | medium |
| `0x802384B0` | `fn_802384b0` | 52 | medium |
| `0x802384F0` | `fn_802384f0` | 68 | medium |

## The `gqr` argument

Every quantised body in this bucket carries the same four lines, repeated per
lane:

```
if ((unaff_GQR0 & 0x3f00) == 0) dVar = 1.0; else dVar = ldexpf((gqr >> 8) & 0x3f);
if (fmt == 4 || fmt == 6) *(char  *)p = (char )(dVar * v);
else if (fmt == 5 || fmt == 7) *(short *)p = (short)(dVar * v);
else *p = v;
```

`unaff_GQR0` is not an argument of the function - the caller loads it with
`mtgqr0` before the call - so the export cannot show its value. Earlier waves
read that as "cannot be reconstructed" and skipped the group. The value is the
caller's business, not the body's, so this pass makes it an explicit `gqr`
parameter and transcribes the branches. `psq_store` covers the store direction
(format in the low half, exponent in bits 8..13) and `psq_load` the mirror
(format in bits 16..18, reciprocal exponent in bits 24..29).

Two consequences are recorded in every affected doc block:

- the multiply is done in `f32` here while the original widens to `f64` first,
  so a value sitting exactly on a truncation boundary can move by one unit;
- the `2^n` factor is built with `f32::from_bits((127 + n) << 23)` instead of a
  `ldexpf` call, which is what Metrowerks emits for the same constant.

## Deliberate skips

34 of the 45 untranslated addresses in the bucket stay untranslated. Every one
of them was read in the export before being skipped.

- **Bodies that read a quantised index back as raw bits**
  (`0x80237A60`, `0x80237B00`, `0x80237BD0`, `0x80237C40`, `0x80237CA0`,
  `0x80237D10`, `0x80237ED0`, `0x8023F000`, `0x8023F140`, `0x8023F4E0`,
  `0x8023FA30`, `0x8023FE50`, plus `0x80239780` and `0x802385D0`): these write
  a value through the GQR store format and then use the stored bytes as an
  index into a coefficient or colour table. In the 8-bit branch Ghidra writes
  one byte and reads two (`local_28._0_2_`); in the 16-bit branch it writes the
  high half and reads the low half. Only the 16-bit reading is coherent, so
  reconstructing the other branches would mean inventing the undefined half.
  These are the angle reducers, the colour-key sampler and the animation-key
  samplers.
- **`0x80237BA0` (36 bytes):** a Newton refinement of `1 / sqrt(x)`. The export
  renders the square root as the `SQRT` intrinsic and this `no_std` crate has no
  `f64` square root and no libm dependency. Substituting `pow` would change the
  rounding, so the body stays untranslated rather than approximating it.
- **Full matrix products `0x80238200` and `0x802382B0`:** both pair up two
  independent output lanes in one `ps_mul`/`ps_sum0` sequence, and Ghidra's
  scalar emulation crosses those lanes - `0x80238200` stores `dst[3]`, `dst[7]`
  and `dst[11]` twice, once with a product and once with the accumulated dot
  product. Transcribing the C literally would reproduce Ghidra's crossed
  lanes, not the original result, so the bodies are left as address calls.
- **Large packed-vector bodies** (`0x802389F0` 1 752 bytes, `0x802390D0`,
  `0x80239380`, `0x802394E0`): the same read-back objection, at a size where a
  single wrong shift silently corrupts the whole result.
- **`0x80238930`:** needs a call whose four `f64` arguments live in `f1..f4`
  and whose remaining argument is an `undefined8` of unknown type; the callee
  `0x802389F0` is skipped anyway, so only a shell could be produced.
- **`0x80231CF0` (1 468 bytes) and `0x802323E0` (2 088 bytes):** the text
  layout engine and the per-character renderer. Both decode a packed glyph
  record through `unaff_GQR0`, and both call the skipped `0x80231CF0` chain, so
  they inherit the read-back objection.
- **`0x80230280` / `0x80232C10` (1 240 bytes each):** the paired-single
  text/box transforms. Every branch quantises and immediately dequantises with
  `unaff_GQR0`, and the two bodies hand a `f64` through `FUN_802DCDC0`, which
  cannot be modelled without the register.
- **`0x802337B0` (glyph blitter, 704 bytes):** still skipped. The 5x7 font
  expansion builds each 16-bit pixel pair from four rotated bits of a 30-bit
  font row and renormalises through a three-lookup `0x8034CF30` chain. The
  export does not contain the font table.
- **`0x802348F0` (console frame loop, 400 bytes):** still skipped. Its argument
  is read from `FUN_802DCD64` (`_savegpr0_23`) although the body is declared
  `void f(void)`, and its tail is an unmodelled `do { } while (true)` loop over
  `_DAT_800000F8 / 4000` timebase ticks.
- **`0x80235AE0` (script parser, 716 bytes):** still skipped. The decompiler
  could not structure it - `unaff_r30` is reused as a loop counter across three
  `goto` targets and three calls have empty argument lists.
- **Variadic queue entry points `0x80236230`, `0x802365E0`, `0x80236FC0`,
  `0x80237370`** (288 bytes each): they build a `va_list` descriptor whose type
  word `0x05000000` includes an FPR save area and whose first saved "GPR" slot
  is Ghidra's `extraout_f1`, i.e. `f1`. There is no honest integer-only model of
  that descriptor, and the FPR half cannot be reconstructed from the export.
  `0x80233480`, which this pass *did* translate, uses the GPR-only word
  `0x04000000` and is therefore not affected.
- **`0x80234A80` (the shared report sink, 224 bytes):** every exception-dump
  body calls it, and it accepts a format plus a descriptor, but its own body
  also formats into the `0x803DA8A4` record and calls the `0x8015F6E0` thunk
  with a descriptor whose `0x01000000` type word the export does not explain.
  It stays an address call.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read from the TSV, not copied from prose).
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 227 such bodies.
- No address in `ledger/agents/wave98-bres.csv` was already in
  `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave98` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
