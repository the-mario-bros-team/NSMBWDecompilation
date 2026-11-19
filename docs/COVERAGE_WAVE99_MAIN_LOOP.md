# Coverage - Wave 99 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Before this pass the bucket had 164 of its 203 functions
translated; Wave 99 adds 20 more and leaves 19.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path. "Confidence" is copied from the body's `# Confidence` line.
The body is `pub unsafe fn fn_<address>` in `src/engine/main_loop.rs`; the
address is always lowercase hex in the file name and in the ledger row.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 20 | 6148 | 5 | 13 |

Two of the 20 bodies carry `low` confidence (`0x802E16E8`, `0x802E17DC`), so
the medium column is 13 and `high + medium + low = 20`.

Before this pass the file held 164 bodies; it now holds 184.

## What this pass covers

- **Sine / cosine / tangent** (`0x802E7FBC`, `0x802E84C4`, `0x802E8590`): the
  three `double` wrappers around the `__kernel_cos` / `__kernel_sin` /
  `__kernel_tan` pair. Each keeps the `|x| < 0x3FE921FC` fast path, the
  `|x| < 0x7FF00000` reduction and the four-quadrant sign selection.
- **Range reduction** (`0x802E5DB8`): `__ieee754_rem_pio2`. It keeps the
  two-chunk `pi/2` band, the `npio2_hw` exact-multiple table at `0x802FE138`,
  the three decreasing-precision correction steps and the Payne-Hanek
  hand-off to `0x802E624C` for arguments above `2^20 * pi/2`.
- **Scaling** (`0x802E4660`): `scalbn`, i.e. `frexp` into a 4-byte stack slot,
  the exponent add, and `ldexp`.
- **Inverse tangent family** (`0x802E496C`, `0x802E4C04`): the two-branch
  `atan` (odd rational series below `0.5`, `sqrt` reduction above it) and the
  `atan2` quadrant reconstruction with its `0` / `pi/2` / `pi` / `pi/4`
  special-case tables.
- **Remainder** (`0x802E4E5C`): `fmod`, the bit-by-bit subtract-and-shift
  loop, including the `0x802FDFF0` signed-zero table.
- **Four-byte tails** (`0x802E8608`, `0x802E860C`, `0x802E8610`,
  `0x802E8614`): the `acos` / `atan` / `atan2` / `fmod` entry points, each a
  `b <target>` that is now a direct call to the body in this file.
- **Unaligned block copy** (`0x802EC2A8`): the byte-at-a-time
  read-modify-write word copy with the two MSR installs and the final restore.
- **Mapped block transfer** (`0x802EC374`): the `fn_802e9430` address map, the
  `fn_802ec1b0` cache-window test, the forward copy plus the two
  `0x802E8E1C` flushes and the reverse copy.
- **Patch builder** (`0x802ED6EC`): the ten-word `mfspr` / `mtspr` template
  read from `0x802FE320..0x802FE33C` and `0x802FE368..0x802FE38C`, with the
  `spr == 0x20` (`mffs`) and `spr == 0x21` (FPSCR) special cases.
- **Per-index transfers** (`0x802EC5BC`, `0x802EC85C`): the GPR range
  (`< 0x22`) and SPR range (`< 0x20`) eight-byte-group transfers, including the
  `0x80351620..0x8035162C` save/restore bracket and the `0x702` sticky flag.
- **Variadic format entries** (`0x802E16E8`, `0x802E17DC`): the `sprintf` /
  `snprintf` shapes that build the `va_list` (register word, overflow area,
  register save area) and call the MSL formatter `0x802E0BF0`.
- **Exception flag clear** (`0x802EA6BC`): the two byte stores that reset
  `0x804270C8` / `0x804270C9` before `returnFromInterrupt`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E4660` | `fn_802e4660` | 64 | high |
| `0x802E4C04` | `fn_802e4c04` | 592 | medium |
| `0x802E4E5C` | `fn_802e4e5c` | 896 | medium |
| `0x802E496C` | `fn_802e496c` | 664 | medium |
| `0x802E5DB8` | `fn_802e5db8` | 900 | medium |
| `0x802E7FBC` | `fn_802e7fbc` | 200 | medium |
| `0x802E84C4` | `fn_802e84c4` | 204 | medium |
| `0x802E8590` | `fn_802e8590` | 120 | medium |
| `0x802E8608` | `fn_802e8608` | 4 | high |
| `0x802E860C` | `fn_802e860c` | 4 | high |
| `0x802E8610` | `fn_802e8610` | 4 | high |
| `0x802E8614` | `fn_802e8614` | 4 | high |
| `0x802E16E8` | `fn_802e16e8` | 244 | low |
| `0x802E17DC` | `fn_802e17dc` | 212 | low |
| `0x802EA6BC` | `fn_802ea6bc` | 356 | medium |
| `0x802EC2A8` | `fn_802ec2a8` | 204 | medium |
| `0x802EC374` | `fn_802ec374` | 336 | medium |
| `0x802EC5BC` | `fn_802ec5bc` | 316 | medium |
| `0x802EC85C` | `fn_802ec85c` | 368 | medium |
| `0x802ED6EC` | `fn_802ed6ec` | 456 | medium |

## Deliberate skips

Nineteen addresses in the bucket stay untranslated. Each one has a concrete
reason, not a shortage of time.

- **`0x802E0454`, `0x802E0BF0`, `0x802E21FC`, `0x802E2C8C`, `0x802E3208`,
  `0x802E3998`** - the MSL format family. The two drivers (`0x802E0BF0` and
  `0x802E3998`) dispatch the float conversions to `0x802DFF64` and
  `0x802E0454`, which take the value in `f1`; this crate has no helper that
  can pass a floating-point argument to an absolute address, so those cases
  cannot be expressed without translating callees outside this lane. The
  remaining four are the digit packers and the wide spec parser, which sit
  behind the same drivers.
- **`0x802E55A4` (`pow`)** - it calls `thunk_FUN_802E8620` (the software
  square root) with an argument list Ghidra dropped and re-uses the result as
  the mantissa; the reduced-argument value is not provable from the export.
  Its tail [`fn_802e861c`] therefore also stays untranslated.
- **`0x802E624C`** - the 5816-byte Payne-Hanek tail. It is called from
  [`fn_802e5db8`] by absolute address with the six register arguments the
  classic `__kernel_rem_pio2` takes; translating the body itself is a
  dedicated pass.
- **`0x802E890C`, `0x802E8E1C`** - the eight-byte `trapWord` stub and the
  `dcbst` / `dcbf` / `sync` / `icbi` flush loop. Both are pure inline
  assembly (`locks`), which this `#![no_std]` crate does not have. Callers
  keep using the absolute addresses.
- **`0x802EC1A0`, `0x802EC1A8`** - the two eight-byte `mfmsr` / `mtmsr`
  leaves. Ghidra prints an empty body for both because the privileged SPR
  instructions produce no decompiled operation; they stay call-by-address
  targets of [`fn_802ec2a8`] / [`fn_802ec374`].
- **`0x802ED6A4`, `0x802ED6C8`** - the FPSCR dump helpers. Every operand is an
  `mffs`-only pseudo-register (`in_fp_fx`, `in_fp_vxvc`, ...), so the 32-bit
  result word cannot be formed without reading the FPSCR.
- **`0x802E9CC8`** - 116 bytes of extent for two identical stores and a `0`
  return, with three blocks removed as unreachable. Writing one store would
  misrepresent the function.
- **`0x802EA4F8`** - the SPR dump; every store reads an `mfspr`-only
  register.
- **`0x802EC9CC`, `0x802ECCB4`** - the exception prologue and epilogue.
  Register save/restore plus `returnFromInterrupt` need inline assembly.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/` (the tails cite the
  `802e8608_thunk_FUN_802e46a0.c` / `802e860c_thunk_FUN_802e496c.c` /
  `802e8610_thunk_FUN_802e4c04.c` / `802e8614_thunk_FUN_802e4e5c.c` names
  Ghidra actually produced).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 184 bodies and no duplicates.
- No address was already in `ledger/functions.csv` (it held 164 of the 203
  bucket entries before this pass).
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave99` reports `OK` for
  `ledger/agents/wave99-main-loop.csv`.
