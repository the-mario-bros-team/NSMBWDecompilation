# Coverage - Wave 97 (resource::registry)

## How to read this file

Every row below is one real `pub unsafe fn` body in
`src/resource/registry.rs`. The address and byte count come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export that was read before the body was written. `confidence` matches the
`# Confidence` line of the function's doc block, and the role is the same
plain-English summary as the module-header table.

Three conventions need a short explanation:

- **Float ABI.** `crate::platform` exposes `call0..call7`, which only fill
  integer registers. These bodies call callees that read `f1..f3` or return a
  float, so the section declares local `extern "C"` transmute helpers
  (`w97_call_f32x2_u32`, `w97_call_f64_f64`, …). This is the same device
  `src/engine/dsys.rs` and `src/audio/ax.rs` already use for the PowerPC EABI,
  whose float and integer register files are separate. Each helper states the
  argument shape it serves.
- **GQR noise.** Ghidra wraps several of these bodies in
  `if ((unaff_GQR0 & 0x3f00) != 0) ldexpf(...)` blocks. `unaff_GQR0` and
  `unaff_GQR3` are never assigned by the function, so those branches are dead
  and are not reproduced. The one place a quantised read is real
  (`0x8016EBF0`/`0x8016ED10`/`0x8016EE30`) says so with `// inferred:`.
- **`r2` constants.** `r2` is `SDA2_BASE` = `0x80433080`; every `r2 - 0xNNNN`
  word cited below was read back from `work/dol/main.elf` at
  `SDA2_BASE - 0xNNNN`, and the constant comment gives both the address and the
  value found there.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 25 | 3832 | 5 | 20 |

Address range: `0x80160D10..0x8016EE30`.

## What this pass covers

The `0x80160000` bucket held 48 untranslated bodies when the pass started (421
total, 373 already translated). Wave 93 had documented all 48 as skips for one
reason: every one of them touches the floating-point register file. This pass
takes the 25 that are reconstructible once that ABI is written out, and leaves
23 documented skips.

- **Step and angle pair** (`0x80160D10`, `0x80160E80`, `0x80160EA0`, 3 bodies).
  The step helper measures the `current - target` vector with `0x801C14C0`,
  snaps on `FLT_EPSILON` or on arrival, and otherwise subtracts
  `step / length` of the delta. The two reporters compute the horizontal
  `x`/`z` angle and the vertical `dy` over `sqrt(dx² + dz²)` angle for
  `0x801611C0`.
- **Random-rate scale leaves** (`0x801613A0`, `0x80161400`, 2 bodies). Both
  multiply by the `f32` that `0x80161390` tail-calls out of `0x80161490`; the
  first converts its `u32` with the `2^52 + 2^31` bias, the second works in
  `f32` throughout.
- **Render-state commit family** (`0x801654B0`, `0x80165AE0`, `0x801666E0`,
  `0x801672D0`, `0x80167E50`, `0x80168F10`, `0x80168FE0`, `0x80169020`,
  `0x801690E0`, 9 bodies). `0x80168F10` is the shared push: it clamps the
  incoming value at `0.0`, stores `+0x28`/`+0x2C`/`+0x34`/`+0x30` and drives
  the `+0x04` object's `+0x1C` and `+0x28` slots. The five commit wrappers
  differ only in which element field holds the reference count (`+0x20` or
  `+0x30`) and which holds the `leading_zeros` word (`+0x24`, `+0x28`,
  `+0x38`); all pass `1.0` and `-1.0` from their own `r2` trio. The two
  predicates read the same `+0x04` object through slots `+0x20` and `+0x2C`.
- **Record and scale leaves** (`0x80163760`, `0x801643B0`, `0x80168BD0`,
  3 bodies). The animation-record fill with its `0.0` clamp, the
  `0x80163FF0` lookup that feeds it, and the scale record whose `+0x0C`
  becomes `1.0 / scale`.
- **Texture builders** (`0x80169290`, `0x801693F0`, `0x80169440`, 3 bodies).
  The `w x h` halving, the allocator's `+0x14` reservation, the
  `0x801C6D90`/`0x801C6FF0` draw pair, the `0x801C5960`/`0x801C59D0` present
  pair, the `0x801C6290` texture bind and the final blend rebuild from the
  global display object at `r13 - 0x5200`.
- **Text object factory** (`0x80164770`, 1 body). The two-table slot guard,
  the `0x802B9170` heap switch around a `0x28`-byte allocation, the
  `0x80164990` slot bind and the `+0x18` deleting destructor on failure.
- **GX reset** (`0x80163220`, 1 body). `0x801C5860(0)`, `0x801C8E70(0,7,0)`
  and `0x801C8920` with four `0.0` components plus the BSS word at
  `r2 - 0x3470`.
- **Axis rotation matrices** (`0x8016EBF0`, `0x8016ED10`, `0x8016EE30`,
  3 bodies). One signed halfword angle, scaled by `1/256` (`r2 - 0x5360`,
  `0x8042DD20`), through `0x80237C40` (cosine) and `0x80237BD0` (sine), then
  twelve float stores forming the X, Y and Z rotations.

The four commit twins are the `0x80165AE0`/`0x801666E0`/`0x801672D0`/
`0x80167E50` addresses wave 93's layout-group bodies still call directly, so
those constants now name functions in this same file.

## Functions

| address | name | role | bytes | status | confidence | evidence |
|---|---|---|---:|---|---|---|
| `0x80160D10` | `fn_80160d10` | step a position toward a target | 368 | translated | medium | `work/ghidra/main-gb/export/80160d10_FUN_80160d10.c` |
| `0x80160E80` | `fn_80160e80` | horizontal angle between two points | 28 | translated | high | `work/ghidra/main-gb/export/80160e80_FUN_80160e80.c` |
| `0x80160EA0` | `fn_80160ea0` | vertical angle between two points | 104 | translated | medium | `work/ghidra/main-gb/export/80160ea0_FUN_80160ea0.c` |
| `0x801613A0` | `fn_801613a0` | scale a word by the random rate | 88 | translated | high | `work/ghidra/main-gb/export/801613a0_FUN_801613a0.c` |
| `0x80161400` | `fn_80161400` | scale a float by the random rate | 56 | translated | high | `work/ghidra/main-gb/export/80161400_FUN_80161400.c` |
| `0x80163220` | `fn_80163220` | reset the GX clear colour | 88 | translated | medium | `work/ghidra/main-gb/export/80163220_FUN_80163220.c` |
| `0x80163760` | `fn_80163760` | fill one animation record | 100 | translated | high | `work/ghidra/main-gb/export/80163760_FUN_80163760.c` |
| `0x801643B0` | `fn_801643b0` | look up an entry and start its record | 176 | translated | medium | `work/ghidra/main-gb/export/801643b0_FUN_801643b0.c` |
| `0x80164770` | `fn_80164770` | create one text object | 308 | translated | medium | `work/ghidra/main-gb/export/80164770_FUN_80164770.c` |
| `0x801654B0` | `fn_801654b0` | base element commit | 104 | translated | medium | `work/ghidra/main-gb/export/801654b0_FUN_801654b0.c` |
| `0x80165AE0` | `fn_80165ae0` | first-class element commit | 104 | translated | medium | `work/ghidra/main-gb/export/80165ae0_FUN_80165ae0.c` |
| `0x801666E0` | `fn_801666e0` | second-class element commit | 104 | translated | medium | `work/ghidra/main-gb/export/801666e0_FUN_801666e0.c` |
| `0x801672D0` | `fn_801672d0` | third-class element commit | 104 | translated | medium | `work/ghidra/main-gb/export/801672d0_FUN_801672d0.c` |
| `0x80167E50` | `fn_80167e50` | four-argument element commit | 104 | translated | medium | `work/ghidra/main-gb/export/80167e50_FUN_80167e50.c` |
| `0x80168BD0` | `fn_80168bd0` | reset a scale record | 60 | translated | medium | `work/ghidra/main-gb/export/80168bd0_FUN_80168bd0.c` |
| `0x80168F10` | `fn_80168f10` | push one element's render state | 196 | translated | medium | `work/ghidra/main-gb/export/80168f10_FUN_80168f10.c` |
| `0x80168FE0` | `fn_80168fe0` | run the `+0x1C` slot, store a scale | 60 | translated | medium | `work/ghidra/main-gb/export/80168fe0_FUN_80168fe0.c` |
| `0x80169020` | `fn_80169020` | end-of-range predicate | 192 | translated | medium | `work/ghidra/main-gb/export/80169020_FUN_80169020.c` |
| `0x801690E0` | `fn_801690e0` | range-containment predicate | 312 | translated | medium | `work/ghidra/main-gb/export/801690e0_FUN_801690e0.c` |
| `0x80169290` | `fn_80169290` | create a texture and its draw block | 268 | translated | medium | `work/ghidra/main-gb/export/80169290_FUN_80169290.c` |
| `0x801693F0` | `fn_801693f0` | present a texture from its record | 68 | translated | high | `work/ghidra/main-gb/export/801693f0_FUN_801693f0.c` |
| `0x80169440` | `fn_80169440` | draw and present one texture | 276 | translated | medium | `work/ghidra/main-gb/export/80169440_FUN_80169440.c` |
| `0x8016EBF0` | `fn_8016ebf0` | X-axis rotation matrix | 188 | translated | medium | `work/ghidra/main-gb/export/8016ebf0_FUN_8016ebf0.c` |
| `0x8016ED10` | `fn_8016ed10` | Y-axis rotation matrix | 188 | translated | medium | `work/ghidra/main-gb/export/8016ed10_FUN_8016ed10.c` |
| `0x8016EE30` | `fn_8016ee30` | Z-axis rotation matrix | 188 | translated | medium | `work/ghidra/main-gb/export/8016ee30_FUN_8016ee30.c` |

All 25 rows are `translated`; none is a stub and none is `partial`.

## Deliberate skips

23 bodies in the bucket stay untranslated. The groups below are exact: together
they are the 23 addresses that are neither in `ledger/functions.csv` nor in
this pass's CSV.

- **4-byte thunks (8)** `0x80160AD0`, `0x80161740`, `0x80162110`, `0x801644F0`,
  `0x8016B950`, `0x8016BCB0`, `0x8016E4F0`, `0x8016E8E0`: below the size floor.
  Four are literally empty bodies and four are single branches; none is a
  meaningful reconstruction at four bytes.
- **`0x801611C0` (436)** the quadrant table wrapper. The eight branches, the
  `0x4000`/`0x8000` quadrant offsets and every constant are visible, but
  Ghidra prints several `0x80161180` calls as `FUN_80161180()` or
  `FUN_80161180(param_2,param_1)`, i.e. the numerator/denominator routing is
  lost per branch. Reproducing `atan2` from convention would be invention, so
  the two angle reporters above still reach it by address.
- **`0x80161CC0` (180)** `0x802DCBFC` is called with zero, one and two
  arguments from one body and `r3` is used as both input and output — the
  Metrowerks static-initialisation pattern, with no recoverable argument list.
- **`0x80162A70` (132)** drops the four arguments of `0x80162A50` and the
  receiver of the table entry's virtual call.
- **`0x801645F0` (380)** builds a nine-argument call around a stack struct that
  its first callee only partly initialises; the two halfword locals it forwards
  read uninitialised stack.
- **`0x80165280` (548)** consumes `in_f31`, `unaff_r28` and `unaff_r29`,
  register values the decompiler never assigns.
- **`0x80169600` (1068)**, **`0x8016AD30` (532)**, **`0x8016DEC0` (880)**,
  **`0x8016E3A0` (268)**, **`0x8016F220` (484)**, **`0x8016FA10` (604)**: the
  paired-single quantisation is real here, but the `psq_l` type word lives in a
  `GQR` register set outside the function, and each of these bodies mixes
  byte/short/float reads of the *same* block. The export's float branch reads
  uninitialised stack for several fields, so which width belongs to which field
  is not recoverable. This is the one place where the three rotation-matrix
  builders above could be distinguished: there the only non-garbage branch is
  the signed-16 one and the argument is declared `short *`.
- **`0x80169B20` (244)**, **`0x80169D90` (340)**: consume `in_f31`/`in_ps31_1`,
  i.e. elided paired-single inputs no integer helper can supply.
- **`0x8016EBA0` (68)**: forwards eight doubles in `f1..f8` plus four stack
  floats that the export never writes.
- **`0x8016FD30` (228)**: consumes `extraout_r4` and `extraout_f1`, register
  values the decompiler could not recover at all.

## Verification

- Every address and byte size matches `functions.tsv` (re-read this pass, not
  copied from earlier prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/resource/registry.rs` (remaining errors at report time are in
  `src/resource/archive.rs` and `src/runtime/bt.rs`).
- `node tools/check-lane-csv.mjs wave97` reports
  `wave97-registry.csv: 25 rows` and `OK`.
