# Coverage - Wave 115 (engine::display: the last `0x80150000` body)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D presentation
layer: the `Pausewindow_c` state machine, the `dGameDisplay_c` descriptors, the
`Wm_2d_*` effects and the engine band `0x800D0000..0x80200000`. This pass works
the last open bucket, `0x80150000`: it entered with one untranslated address of
its 209 and leaves none.

"Bodies" counts `pub unsafe fn` functions; "bytes" is the Ghidra size re-read
from `work/ghidra/main-gb/export/functions.tsv` for this pass.

"medium" means one operand shape or field role had to be recovered from the
instruction words behind the export, and that recovery is spelled out in the
body's `# Confidence` block.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's row.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 1 | 128 | 0 | 1 |

## What this pass covers

One cluster, the single open address `0x8015F730` (128 bytes):

- **The variadic report entry** (`0x8015F730`). [`fn_8015f730`] is the last body
  of the `0x80150000` bucket and the `printf`-shaped front end that every
  `OSReport` call site in the game reaches (for example `0x801AA550`'s boot
  banner and `0x801AB1C0`'s single-value report). It is the missing caller of
  [`fn_8015f7b0`] (`0x8015F7B0`), a four-byte branch to the callback trampoline
  `0x8015F6E0` documented in `game::task`, so this lane completes both halves of
  the report chain inside this crate.
- **The `va_list` frame**: `stwu r1,-0x80(r1)` allocates 0x80 bytes. The eight
  integer argument registers are stored at `+0x08..+0x24` (`r3` = format, then
  `r4..r10`), the eight float argument registers at `+0x28..+0x60` under
  `bne cr1,+0x24` (the EABI variadic "floating-point arguments passed" flag),
  and the twelve-byte `va_list` at `+0x68`: the tag `0x01000000`, the overflow
  area `r1 + 0x88` (the caller's first stack word) and the register save area
  `r1 + 0x08`.
- **The dispatch**: `bl 0x8015F7B0` from `0x8015F79C` with `r3` (the format
  pointer) untouched and `r4 = r1 + 0x68`; the export prints the same call, but
  as `thunk_FUN_8015f6e0`, because `0x8015F7B0` is itself a four-byte thunk.
- **The Ghidra shape**: the export types the float saves as eight `undefined8`
  parameters guarded by `in_cr1` and the integer saves as `param_9..16`, because
  it spends its whole parameter budget on the float pairs. The body recovers the
  real split from the instruction words, which are cited per offset.

The `in_cr1` guard is not expressible in Rust, so the eight doubles are always
stored; that is equivalent, because the guard only skips stores of floats the
caller never passed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8015F730` | `fn_8015f730` | 128 | medium |

Byte total: `128`.

## Deliberate skips

None. The only open address in the `0x80150000` bucket was `0x8015F730`, and it
is translated. Two readings are inferred rather than literal in the C export and
are flagged in the body: the `0x01000000` tag is read as GPR index `1` / FPR
index `0` (the same byte-pair convention as the `0x03000000` tag documented for
`0x802E16E8`), and the caller's stack argument area is modelled as an explicit
`stack_args` parameter because Rust has no way to name the caller's `sp + 8`.

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`
  (`8015f730`, `128` bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/8015f730_FUN_8015f730.c`.
- The row has exactly one `pub unsafe fn fn_8015f730` body in
  `src/engine/display.rs`.
- `0x8015F730` was absent from `ledger/functions.csv` before this pass; the
  bucket had 1 untranslated function before it and 0 after it.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/engine/display.rs`.
- `node tools/check-lane-csv.mjs wave115` reports this lane clean.
