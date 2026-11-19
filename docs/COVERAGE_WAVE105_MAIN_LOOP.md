# Coverage - Wave 105 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Before this pass the bucket had 189 of its 203 functions
translated; Wave 105 adds four more and leaves ten.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path. "Confidence" is copied from the body's `# Confidence` line.
The body is `pub unsafe fn fn_<address>` in `src/engine/main_loop.rs`; the
address is always lowercase hex in the file name and in the ledger row.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 4 | 5092 | 0 | 3 |

One of the four bodies carries `low` confidence (`0x802E9CC8`), so
`high + medium + low = 4`. Before this pass the file held 189 bodies; it now
holds 193.

## What this pass covers

- **Wide format-spec parser** (`0x802E21FC`): the `%` field parser behind the
  wide `printf`. The flag loop (`-`, `+`, space, `#`, `0`), the `*` and digit
  width / precision loops, the `h` / `hh` / `l` / `ll` / `L` / `j` / `t` / `z`
  modifiers and the per-conversion rejection table that writes `0xFFFF` into
  the conversion slot (`%a` does not take `j` / `z` / `t` / `ll`, `%c` and `%s`
  only accept the `l` form, `%p` becomes `%#x` with precision 8, ...).
- **Wide `%a` / `%A` converter** (`0x802E2C8C`): the hexadecimal-float twin of
  [`fn_802e0454`]. `FUN_802DE370` is used only to classify the value (sign,
  `0`, `INF`, `NAN`); a finite value is rebuilt from the raw IEEE-754 bits one
  nibble per requested precision digit and the binary exponent is printed in
  decimal through [`fn_802e278c`]. The eight special-case wide strings at
  `0x803512D8..0x8035133C` are the same ones [`fn_802e3208`] already cites.
- **Wide `printf` driver** (`0x802E3998`): the wide twin of [`fn_802e0bf0`].
  It finds `%` with [`fn_802e44cc`], parses with [`fn_802e21fc`], dispatches
  to the wide converters (now the bodies in this file) and applies the same
  width / zero-pad / sign-prefix / left-justify rules in `u16` units. `%c`
  without `l` and `%s` without `l` widen their argument through `mbtowc` /
  `mbstowcs`.
- **MetroTRK reply flag** (`0x802E9CC8`): two identical stores of `1` to
  `0x8042B038` (`r13 - 0x4648`) and a zero return.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E21FC` | `fn_802e21fc` | 1424 | medium |
| `0x802E2C8C` | `fn_802e2c8c` | 1108 | medium |
| `0x802E3998` | `fn_802e3998` | 2444 | medium |
| `0x802E9CC8` | `fn_802e9cc8` | 116 | low |

## Deliberate skips

Ten addresses in the bucket stay untranslated. Each one has a concrete reason,
not a shortage of time.

- **`0x802E624C`** - the 5816-byte Payne-Hanek tail (`__kernel_rem_pio2`). The
  export is an eight-way unrolled loop in which every integer/`double`
  conversion is the raw `CONCAT44(0x43300000, x) ^ 0x80000000 - 2^52` magic
  number trick, with roughly sixty live locals. A faithful transliteration is
  a dedicated pass; a partial rewrite would silently change the reduction.
  [`fn_802e5db8`] keeps calling it by absolute address.
- **`0x802EA4F8`** - the SPR dump. Every one of its 113 stores reads an
  `mfspr`-only pseudo-register (`in_sr0`, `in_IBAT0U`, `in_HID0`, ...); Ghidra
  prints no source for those operands and this `#![no_std]` crate has no
  `mfspr` helper. [`fn_802ec9cc`] keeps calling it by absolute address.
- **`0x802EC9CC`, `0x802ECCB4`** - the exception prologue and epilogue. They
  save or restore every GPR/SPR from pseudo-registers, run `sync`, and end in
  `returnFromInterrupt(uVar8, param_2)`, which needs inline assembly.
- **`0x802ED6A4`, `0x802ED6C8`** - the FPSCR dump helpers. The 32-bit result is
  assembled from `mffs`-only sub-fields (`in_fp_fx`, `in_fp_vxvc`, ...) and
  both bodies also read the `GQR0` quantization register, none of which this
  crate can read.
- **`0x802E890C`** - eight bytes of `trapWord(0x1f, r0, 0)`, i.e. a `tw`
  instruction. Pure inline assembly.
- **`0x802E8E1C`** - the `dcbst` / `dcbf` / `sync` / `icbi` flush loop. Pure
  inline assembly; callers already use the absolute address.
- **`0x802EC1A0`, `0x802EC1A8`** - the two eight-byte `mfmsr` / `mtmsr` leaves.
  Ghidra prints an empty body for both because the privileged SPR instructions
  produce no decompiled operation.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 193 bodies and no duplicates.
- No address was already in `ledger/functions.csv` (it held 189 of the 203
  bucket entries before this pass; 189 + 4 translated + 10 skipped = 203).
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/main_loop.rs` (the errors that remain in the workspace are in
  other lanes' files).
- `node tools/check-lane-csv.mjs wave105` reports `OK` for
  `ledger/agents/wave105-main-loop.csv`.
