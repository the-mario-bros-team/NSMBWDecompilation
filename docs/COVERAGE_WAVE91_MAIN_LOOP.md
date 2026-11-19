# Coverage - Wave 91 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: the statically linked C
runtime at the low end, MetroTRK in the middle and the interrupt / exception
knobs at the top. Wave 91 adds 25 bodies in two clusters: the MSL
`printf` / `strtol` support code (`0x802E032C..0x802E4660`) and the MetroTRK
request / reply path (`0x802E942C..0x802EC0EC`).

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path, in that order. "Confidence" is copied from the body's
`# Confidence` line. The body is `pub unsafe fn fn_<address>` in
`src/engine/main_loop.rs`; the address is always lowercase hex in the file name.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 25 | 6308 | 7 | 18 |

Before this pass the file held 114 bodies; it now holds 139.

## What this pass covers

- **MSL integer parser** (`0x802E1CF4`): the shared `strtol` engine that the
  Wave 82 `atoi` wrapper [`fn_802e2108`] already calls by address. The body
  keeps the seven-state machine, the ctype class tests (`+8` map) and the
  lowercase -> uppercase map (`+0xC`), the two overflow tests and the
  push-back call at the terminator.
- **Number formatters** (`0x802E278C`, `0x802E29D0`, `0x802E032C`,
  `0x802E30E0`): the 32-bit and 64-bit radix converters write wide digits
  backwards from `end-2` and then prepend sign, `0x` prefix and zero padding;
  the 64-bit twin computes the same quotient and remainder inline because
  `src/platform.rs` cannot forward a 64-bit result from `0x802DCFFC` /
  `0x802DCDD8`. `0x802E30E0` is byte-identical to `0x802E032C` (only the
  function name differs in the export), so it is a full copy of the same body.
- **`printf` entry points** (`0x802E149C`, `0x802E1560`, `0x802E15DC`,
  `0x802E1664`): the stream append callback over the `fwrite` core, the
  `stdout` wrapper, the bounded `snprintf` and the unbounded `sprintf`. All
  four build the same three-word `{pointer, length, 0}` descriptor whose
  address is the append context handed to `0x802E0BF0`.
- **Stream / cache helpers** (`0x802E44F8`, `0x802E4570`, `0x802E4648`): the
  MSL stream orientation get/set (bits `0x1C00000` of the flag word), the
  "reserve the `0xE100`-byte DMA pool once, then flush" wrapper, and the
  optional flush callback stored at `r13 - 0x4680`.
- **Devkit selection** (`0x802E96E4`): the string triage plus the nine-entry
  NDEV handler table at `0x804256D0..0x804256F4`. [`trk_boot`] now calls this
  body directly instead of `platform::call1(0x802E96E4, id)`.
- **Segment writeback** (`0x802E9498`): walks the fifteen-entry segment table
  at `0x803513B8`, translates each cached address to its cached / physical
  alias when it falls outside the third BAT window, copies 256 bytes from the
  image at `0x800046E4 + address` and invalidates the line through
  `0x802E8E1C`. [`fn_802eb1fc`] now calls it directly.
- **TRK reply path** (`0x802E99C0`, `0x802E9D3C`, `0x802EA094`,
  `0x802E9EA0`, `0x802EA1BC`, `0x802EA2D4`, `0x802EA3B8`): the reply
  dispatcher, the header + payload reply reader, the send / poll loop with its
  four header checks, the D0/D1 chunked block transfer with `0x800` byte
  chunks, and the D2/D3/D4 single-register senders. [`fn_802ed2f4`] now calls
  the four senders directly instead of through `platform::call6/4/2`.
- **Small ends** (`0x802E942C`, `0x802E981C`, `0x802EC0E4`, `0x802EC0EC`): the
  four-byte thunk to the breakpoint reset (now a direct call to
  [`fn_802e9874`]), the empty `blr`-only helper at `0x802E981C`, and the two
  eight-byte forwarders into the `0x802EC0F4` cache-flush trampoline.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E032C` | `fn_802e032c` | 296 | medium |
| `0x802E149C` | `fn_802e149c` | 88 | medium |
| `0x802E1560` | `fn_802e1560` | 124 | medium |
| `0x802E15DC` | `fn_802e15dc` | 136 | high |
| `0x802E1664` | `fn_802e1664` | 132 | medium |
| `0x802E1CF4` | `fn_802e1cf4` | 1044 | medium |
| `0x802E278C` | `fn_802e278c` | 580 | medium |
| `0x802E29D0` | `fn_802e29d0` | 700 | medium |
| `0x802E30E0` | `fn_802e30e0` | 296 | high |
| `0x802E44F8` | `fn_802e44f8` | 120 | high |
| `0x802E4570` | `fn_802e4570` | 208 | medium |
| `0x802E4648` | `fn_802e4648` | 20 | medium |
| `0x802E942C` | `fn_802e942c` | 4 | high |
| `0x802E9498` | `fn_802e9498` | 308 | medium |
| `0x802E96E4` | `fn_802e96e4` | 312 | high |
| `0x802E981C` | `fn_802e981c` | 4 | high |
| `0x802E99C0` | `fn_802e99c0` | 144 | high |
| `0x802E9D3C` | `fn_802e9d3c` | 204 | medium |
| `0x802E9EA0` | `fn_802e9ea0` | 500 | medium |
| `0x802EA094` | `fn_802ea094` | 296 | medium |
| `0x802EA1BC` | `fn_802ea1bc` | 280 | medium |
| `0x802EA2D4` | `fn_802ea2d4` | 228 | medium |
| `0x802EA3B8` | `fn_802ea3b8` | 268 | medium |
| `0x802EC0E4` | `fn_802ec0e4` | 8 | medium |
| `0x802EC0EC` | `fn_802ec0ec` | 8 | medium |

## Deliberate skips

- **`0x802E16E8`, `0x802E17DC`** - the variadic `printf` / `wprintf`
  counterparts. Ghidra renders their argument list as eight `undefined8`
  parameters plus the `in_cr1` register-save flag, so which slot holds the
  format string and which the `va_list` is not provable from the export; the
  non-variadic wrappers above cover the entry points that a caller can reach.
- **`0x802E0BF0`, `0x802E21FC`, `0x802E3998`** - the 2220 / 1424 / 2444-byte
  format driver, format-spec parser and bounded output engine. Each is a
  multi-hundred-line state machine over bitfield-packed format records; they
  are called by address and are a dedicated pass of their own.
- **`0x802EA4F8`** - the SPR dump. Every statement stores a register that only
  `mfspr` can read (`in_sr0..in_sr15`, `in_HID0`, `in_DBAT3U`, ...), and this
  crate has no inline assembly, so there is no honest body to write. The
  destination layout is already documented on [`TRK_SPR_CONTEXT`].
- **`0x802EA6BC`, `0x802E9CC8`** - both own far more bytes than their visible
  body (`0x802EA6BC` is 356 bytes for two byte stores; `0x802E9CC8` is 116
  bytes for two identical stores) because Ghidra removed unreachable blocks.
  Writing the visible stores would misrepresent the function.
- **`0x802EC374`, `0x802EC4C4`, `0x802EC5BC`, `0x802EC6F8`, `0x802EC85C`** -
  the register / memory block transfers; still skipped for the byte-wide
  `DAT_8035162c` shuffle reason Wave 82 documented. They are called by address
  from the D2/D3/D4 senders and the state reply.
- **`0x802ECE28`** - the 512-byte `0x90` status reply builder. It walks the
  saved GPR / SPR frame and posts the `0x4E8`-byte message; it is called by
  address from [`fn_802e99c0`] and left for the next pass.
- **`0x802EC014..0x802EC0F4`** - the cache-flush wrappers. Wave 82 skipped
  `0x802EC0F4` because its fifth argument (`r7`) is undefined at the call site;
  the two eight-byte forwarders into it are translated, the callee is not.
- **`0x802EC9CC`, `0x802ECCB4`** - the exception prologue / epilogue. They
  save and restore every GPR, the MSR, the CR fields and SPRG1..3 and finish
  with `returnFromInterrupt`, none of which is representable without inline
  assembly.
- **`0x802ED520..0x802ED6EC`** - the instruction-patch builders. Their bodies
  copy constant tables from `0x802FE320` / `0x802FE344` / `0x802FE368`, which
  are not in this export, and `0x802ED6A4` / `0x802ED6C8` also read the
  uninitialised `GQR0` pseudo-register.
- **`0x802E7F90..0x802E8620`** (the Libm cluster) - their arguments live in
  `f1..f8` and Ghidra renders the constant pool as
  `*(double *)(unaff_r2 - N)`, so the argument order is not provable from the C
  export. Unchanged from Wave 82.
- **`0x802EC1A0`, `0x802EC1A8`** - Ghidra renders both as a bare `return`
  although each occupies 8 bytes, i.e. the real opcode is most likely a
  barrier that the decompiler did not decode. No body was written.
- **The 4-byte branch thunks** `0x802E8608..0x802E8620`, `0x802E8868` and
  `0x802E942C`'s siblings - `0x802E942C` is translated because its target
  [`fn_802e9874`] is already in this module; the rest target Libm or
  untranslated bodies and would only duplicate a signature this pass cannot
  prove.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/` (the thunk row cites
  `802e942c_thunk_FUN_802e9874.c`, the name Ghidra actually produced).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 139 bodies and no duplicates.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/engine/main_loop.rs` (the only build errors come from other Wave 91
  lanes' files).
- `node tools/check-lane-csv.mjs wave91` reports `OK` for
  `ledger/agents/wave91-main-loop.csv` with 25 rows.
