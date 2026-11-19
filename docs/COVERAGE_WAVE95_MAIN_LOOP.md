# Coverage - Wave 95 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Wave 95 adds 25 bodies in three clusters: the MSL math
kernels (`0x802E46A0..0x802E8620`), the remaining TRK cache / transfer entry
points (`0x802EC014..0x802ED5FC`) and two four-byte tails.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path. "Confidence" is copied from the body's `# Confidence` line.
The body is `pub unsafe fn fn_<address>` in `src/engine/main_loop.rs`; the
address is always lowercase hex in the file name.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 25 | 7324 | 7 | 16 |

Two of the 25 bodies carry `low` confidence (`0x802E46A0`, `0x802E79C4`), so
the medium column is 16 and `high + medium + low = 25`.

Before this pass the file held 139 bodies; it now holds 164.

## What this pass covers

- **Elementary `copysign` / rounding** (`0x802E7F90`, `0x802E7E44`,
  `0x802E8084`): the 44-byte sign transplant and the `ceil` / `floor` twins.
  Both rounding bodies keep the four exponent ranges (`< 0.5`, `< 2^20`,
  `< 2^52`, infinity) and the `+/-0.0` / `+/-1.0` edge results.
- **Exponent split / scale / split** (`0x802E81D4`, `0x802E825C`,
  `0x802E83C8`): `frexp` writes the exponent through its second argument and
  pre-scales subnormals by `2^54`; `ldexp` folds the exponent into the field
  and uses the MSL over/underflow constants for the out-of-range cases; `modf`
  stores the integral part as two big-endian words and returns the fraction.
- **Logarithms** (`0x802E51DC`, `0x802E5490`): the `log1p` kernel evaluates
  the `atanh` series for the near-zero and away-from-zero mantissas and adds
  the exponent-scaled `ln 2`; `log` normalizes to `[1,2)` and calls it. Both
  keep the MSL `errno = 0x21` zero / negative handling.
- **Inverse tangent family** (`0x802E613C`, `0x802E7904`, `0x802E79C4`,
  `0x802E7C14`): the tail polynomial, the two-mode `atan` body, the `atan2`
  quadrant reconstruction and the four-octant body with its
  `0x802FE270` / `0x802FE290` offset tables. These are the exact call targets
  that the Wave 91 `acos` / `asin` wrappers use.
- **Square root and arccosine** (`0x802E8620`, `0x802E46A0`): the
  digit-by-digit restoring square root over 52 fraction bits, and the `acos`
  body that reduces around `+/-1` and calls the square root.
- **Transport guards** (`0x802EC014`, `0x802EC07C`, `0x802EC0F4`): the two
  `0x802ED9CC` checks that forward to the `0x802EC0F4` trap-stub transfer, and
  that transfer's `0x802EB090` gate plus its `0/1/2` byte-result mapping.
- **Status replies** (`0x802ECDCC`, `0x802ECE28`): the request-type 3/4
  trigger and the full `0x4E8`-byte `0x90` reply — a `0x40`-byte header, the
  32 saved GPRs, the five status words and the `0x400`-byte code window.
- **Register transfers** (`0x802EC4C4`, `0x802EC6F8`): the GPR range
  (`< 0x25`) and SPR range (`< 0x61`) copies, including the
  `0x80351620..0x8035162C` save/restore bracket, the byte-1 sticky-status
  test and the `0x804270C8` / `0x804270C9` overlap flags.
- **Instruction patch builders** (`0x802ED520`, `0x802ED5FC`): the ten-word
  and nine-word `mfspr` / `mtspr` blocks, with the constant words read from
  their real image addresses `0x802FE320` / `0x802FE344`.
- **Four-byte tails** (`0x802E8618`, `0x802E8868`): branches to the newly
  translated `0x802E5490` and `0x802E8620`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E46A0` | `fn_802e46a0` | 716 | low |
| `0x802E51DC` | `fn_802e51dc` | 692 | medium |
| `0x802E5490` | `fn_802e5490` | 276 | medium |
| `0x802E613C` | `fn_802e613c` | 272 | medium |
| `0x802E7904` | `fn_802e7904` | 192 | medium |
| `0x802E79C4` | `fn_802e79c4` | 592 | low |
| `0x802E7C14` | `fn_802e7c14` | 560 | medium |
| `0x802E7E44` | `fn_802e7e44` | 328 | medium |
| `0x802E7F90` | `fn_802e7f90` | 44 | high |
| `0x802E8084` | `fn_802e8084` | 332 | medium |
| `0x802E81D4` | `fn_802e81d4` | 136 | medium |
| `0x802E825C` | `fn_802e825c` | 364 | medium |
| `0x802E83C8` | `fn_802e83c8` | 252 | high |
| `0x802E8620` | `fn_802e8620` | 584 | medium |
| `0x802EC014` | `fn_802ec014` | 104 | high |
| `0x802EC07C` | `fn_802ec07c` | 104 | high |
| `0x802EC0F4` | `fn_802ec0f4` | 172 | medium |
| `0x802EC4C4` | `fn_802ec4c4` | 248 | medium |
| `0x802EC6F8` | `fn_802ec6f8` | 356 | medium |
| `0x802ECDCC` | `fn_802ecdcc` | 92 | high |
| `0x802ECE28` | `fn_802ece28` | 512 | medium |
| `0x802ED520` | `fn_802ed520` | 220 | medium |
| `0x802ED5FC` | `fn_802ed5fc` | 168 | medium |
| `0x802E8618` | `fn_802e8618` | 4 | high |
| `0x802E8868` | `fn_802e8868` | 4 | high |

## Deliberate skips

- **`0x802E0454`, `0x802E0BF0`, `0x802E16E8`, `0x802E17DC`, `0x802E21FC`,
  `0x802E2C8C`, `0x802E3208`, `0x802E3998`** - the MSL format driver,
  variadic entries, format-spec parser, `strtod`-sized parsers and bounded
  output engine. Each is a multi-hundred-line state machine and remains a
  dedicated pass, unchanged from Wave 91.
- **`0x802E496C`, `0x802E4C04`, `0x802E4E5C`, `0x802E55A4`, `0x802E5DB8`,
  `0x802E624C`** - the remaining large math bodies. `0x802E4C04` and
  `0x802E496C` have calls whose argument lists Ghidra dropped
  (`FUN_802e7c14()` and `thunk_FUN_802e8620(...)` with placeholder arguments),
  so their reduced-argument values are not provable from the export. The
  `fmod` (`0x802E4E5C`) and `rem_pio2` (`0x802E5DB8`) bodies are complete but
  were left for a dedicated pass rather than risk a rushed 175-line
  translation.
- **`0x802E8E1C`** - `dcbst` / `sync` / `icbi`; still needs cache intrinsics
  that this `#![no_std]` crate does not have. Callers keep using the absolute
  address.
- **`0x802E9CC8`, `0x802EA6BC`** - both own far more bytes than their visible
  body (116 bytes for two identical stores; 356 bytes for two byte stores)
  because Ghidra removed unreachable blocks. Writing only the visible stores
  would misrepresent the function.
- **`0x802EA4F8`** - the SPR dump; every store reads an `mfspr`-only register.
- **`0x802EC2A8`, `0x802EC374`, `0x802EC5BC`, `0x802EC85C`** - the remaining
  block transfers. `0x802EC374` / `0x802EC2A8` depend on `0x802EC1A0` /
  `0x802EC1A8`, which Ghidra renders as empty `return` although both callers
  use their result, so the value is not recoverable. `0x802EC5BC` and
  `0x802EC85C` have `extraout_r4` placeholder arguments in their
  `FUN_802eabe8` calls.
- **`0x802EC9CC`, `0x802ECCB4`** - the exception prologue / epilogue; register
  save/restore and `returnFromInterrupt` need inline assembly.
- **`0x802ED6A4`, `0x802ED6C8`, `0x802ED6EC`** - the FPSCR / GQR patch helpers.
  `0x802ED6A4` reads thirty FPSCR bit fields that only `mffs` can produce, and
  `0x802ED6EC` calls it without the output pointer Ghidra dropped.
- **`0x802E890C`** - the eight-byte TRK trap stub (`trapWord`); no inline
  assembly. It stays a call-by-address target of `fn_802ec0f4`.
- **The 4-byte branch thunks** `0x802E8608..0x802E8614` and `0x802E861C` -
  their targets are the large math bodies skipped above, so a forward would
  only duplicate a signature this pass cannot prove. `0x802E8618` and
  `0x802E8868` are translated because their targets are now bodies in this
  file.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/` (the tails cite the
  `802e8618_thunk_FUN_802e5490.c` / `802e8868_thunk_FUN_802e8620.c` names
  Ghidra actually produced).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 164 bodies and no duplicates.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave95` is clean for
  `ledger/agents/wave95-main-loop.csv`.
