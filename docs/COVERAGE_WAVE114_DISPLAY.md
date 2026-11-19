# Coverage - Wave 114 (engine::display: the last `0x800D0000` element bind/attach pass)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D presentation
layer: the `Pausewindow_c` state machine, the `dGameDisplay_c` descriptors, the
`Wm_2d_*` effects and the engine band `0x800D0000..0x80200000`. This pass works
the last open bucket, `0x800D0000`: it entered with one untranslated address of
its 251 and leaves none.

"Bodies" counts `pub unsafe fn` functions; "bytes" is the Ghidra size re-read
from `work/ghidra/main-gb/export/functions.tsv` for this pass.

"medium" means one operand shape or field role had to be recovered from the
export plus the surrounding call sites, and that recovery is spelled out in the
body's `# Confidence` block.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's row.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 1 | 312 | 0 | 1 |

## What this pass covers

One cluster, the single open address `0x800D43E0` (312 bytes):

- **The element bind/attach pass** (`0x800D43E0`). [`fn_800d43e0`] is the last
  body of the `0x800D0000` bucket and the twin of `0x800FEC10` (outside this
  lane), which runs the same sequence on a different marker class. It is a
  `void` routine with three float operands (`f1..f3`), the marker object in
  `r3`, a source record in `r4` and a mode word in `r5`.
- **Model lookup**: the marker keeps the current view index at `+0x770` and a
  name-pointer table at `+0x778`; the third word of the view entry is the model
  name. The name is resolved against the BRRES archive field at `+0x20C`
  through [`crate::resource::bres::fn_80239e30`], whose own documentation
  defines the argument as "address of the word that holds the BRRES file
  pointer". The returned resource pointer is not stored, exactly as in the
  export.
- **Element filing**: the first word of the source record is passed by address
  to `0x801651F0` together with the 0x38-byte element at `object + 0x18`, the
  receiver returned by the marker's vtable slot `+0x28` and the mode word.
  `0x801651F0` runs the element's `+0x0C` slot, installs the value through
  `0x80168520` and rebuilds the keyed entry with `0x80165280`.
- **Animation and attach**: `0x801680E0` hands the first float to the element's
  `+0x04` object slot `+0x28`; `0x80168FE0` stores the second float at element
  `+0x30`; a second `+0x28` receiver is then combined with the third float and
  the descriptor at `object + 0x26` in `0x80169F80`, which attaches the element
  when its `+0x0C` slot reports "not attached".
- **The halfword conversion**: the `local_50 = 0x43300000` /
  `uStack_4c = *(ushort *)(*param_5 + 0x20) ^ 0x80000000` pair is the standard
  PowerPC signed-halfword to double conversion. The converted value is the
  float operand of the second `+0x28` invocation; Ghidra prints the SDA
  conversion constant `0x4330000000000000` in that argument slot instead, which
  is noted in the body as `// inferred:`.

The `ldexpf(GQR0)` block at the head of the export is the decompiler's failed
model of a paired-single save, the same artifact documented for the wave-105,
wave-106 and wave-108 bodies in this file. It is not a call and is omitted.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800D43E0` | `fn_800d43e0` | 312 | medium |

Byte total: `312`.

## Deliberate skips

None. The only open address in the `0x800D0000` bucket was `0x800D43E0`, and it
is now translated. Two operand shapes are inferred rather than literal and are
flagged in the body: the three `undefined8` parameters are the `f1..f3` float
arguments (the same callees are called as `(float, element)` from `0x800F0630`
and `0x800EF230`), and the `f1` operand of the second vtable `+0x28` call is
printed as an SDA constant instead of the converted halfword.

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`
  (`800d43e0`, `312` bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/800d43e0_FUN_800d43e0.c`.
- The row has exactly one `pub unsafe fn fn_800d43e0` body in
  `src/engine/display.rs`.
- `0x800D43E0` was absent from `ledger/functions.csv` before this pass; the
  bucket had 1 untranslated function before it and 0 after it.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/engine/display.rs`; the remaining crate errors name other lanes' files
  that were being edited in parallel.
- `node tools/check-lane-csv.mjs wave114` reports this lane clean.
