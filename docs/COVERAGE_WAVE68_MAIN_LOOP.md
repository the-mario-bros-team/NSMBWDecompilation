# Coverage - Wave 68 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket: statically linked C
runtime helpers at the low end, MetroTRK in the middle, and the interrupt /
exception knobs at the high end. Earlier waves translated the debug-boot
entries, the MetroTRK core, the UART ring buffer, the message-buffer pool and
the SDA platform knobs. This pass adds the C string routines and the
message-payload accessor set. Every row below cites the Ghidra name, the
`functions.tsv` byte size and the exact export path in that order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 29 | 3116 | 25 | 4 |

Before this pass the file held 60 bodies; it now holds 89.

## What this pass covers

- **Byte strings** (`0x802E19F8..0x802E1C88`): the MSL `strncpy` /
  `strcat` / `strncat` / `strcmp` / `strncmp` / `strchr` / `strrchr` /
  `strstr` family. `strcmp`'s aligned word-at-a-time fast path is collapsed to
  the same byte comparisons; the result is identical for every valid C string.
- **Wide strings** (`0x802E4324..0x802E44CC`): the append callback for the wide
  format driver (`{base, limit, cursor}` state), the bounded wide formatter
  that drives `0x802E3998` with that callback, and `wcslen` / `wcscpy` /
  `wcsncpy` / `wcscmp` / `wcschr`. The shared copy helper `0x802E21CC` turns
  an element count into `count << 1` bytes for `0x80004364`.
- **Narrow append callback** (`0x802E14F4`): the byte twin of the wide append
  callback, used by the byte formatter `0x802E0BF0`.
- **TRK payload accessors** (`0x802EAA88..0x802EAFA8`): read bytes out, append
  a single word / two words / byte string / word array in wire byte order, and
  read a 64-bit field / byte stream / word array. All of them use the
  message-buffer layout Wave 57 documented: `+0x04` valid length, `+0x08` read
  cursor, `+0x0C` payload, `0x880`-byte capacity. The little-endian path is
  selected by `r13 - 0x4648` (`0x8042B038`).
- **Notification senders** (`0x802EB098`, `0x802EB108`, `0x802EB190`,
  `0x802EB1FC`): the four `0x80` message senders that build the 0x40-byte
  header, stamp `old_sequence + 1` at `+0x0C`, bump `r13 - 0x4630`
  (`0x8042B050`) by two, and then either set / clear the `0x8042B054` request
  flag, post event 1, re-enter `0x802E9614`, or flush code with `0x802E9498`.
- **Event release** (`0x802E9BE4`): frees the message-buffer slot named by the
  third word of a task event; this is the call already visible in
  `fn_802e8e90`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E14F4` | `fn_802e14f4` | 108 | high |
| `0x802E19F8` | `fn_802e19f8` | 68 | high |
| `0x802E1A3C` | `fn_802e1a3c` | 44 | high |
| `0x802E1A68` | `fn_802e1a68` | 76 | high |
| `0x802E1AB4` | `fn_802e1ab4` | 284 | high |
| `0x802E1BD0` | `fn_802e1bd0` | 64 | high |
| `0x802E1C10` | `fn_802e1c10` | 48 | high |
| `0x802E1C40` | `fn_802e1c40` | 72 | high |
| `0x802E1C88` | `fn_802e1c88` | 104 | high |
| `0x802E21CC` | `fn_802e21cc` | 8 | high |
| `0x802E4324` | `fn_802e4324` | 108 | high |
| `0x802E4390` | `fn_802e4390` | 140 | high |
| `0x802E441C` | `fn_802e441c` | 28 | high |
| `0x802E4438` | `fn_802e4438` | 28 | high |
| `0x802E4454` | `fn_802e4454` | 68 | high |
| `0x802E4498` | `fn_802e4498` | 52 | high |
| `0x802E44CC` | `fn_802e44cc` | 44 | high |
| `0x802E9BE4` | `fn_802e9be4` | 8 | high |
| `0x802EAA88` | `fn_802eaa88` | 144 | high |
| `0x802EAB18` | `fn_802eab18` | 208 | high |
| `0x802EACDC` | `fn_802eacdc` | 100 | high |
| `0x802EAD40` | `fn_802ead40` | 240 | high |
| `0x802EAE30` | `fn_802eae30` | 224 | high |
| `0x802EAF10` | `fn_802eaf10` | 152 | high |
| `0x802EAFA8` | `fn_802eafa8` | 232 | high |
| `0x802EB098` | `fn_802eb098` | 112 | medium |
| `0x802EB108` | `fn_802eb108` | 136 | medium |
| `0x802EB190` | `fn_802eb190` | 108 | medium |
| `0x802EB1FC` | `fn_802eb1fc` | 108 | medium |

## Deliberate skips

- `0x802E149C`, `0x802E1560`, `0x802E1664`, `0x802E16E8`, `0x802E17DC`: the
  narrow `printf` / `fprintf` wrappers. The first two return a caller-supplied
  stream word whose identity is unknown, and the last three are variadic with
  a Ghidra `va_list` reconstruction that could not be verified against the
  register save area. The shared append callback `0x802E14F4` is translated
  instead.
- `0x802E44F8`, `0x802E4570`, `0x802E4660`: wide-format state helpers. `44F8`
  reads undocumented bits of `+0x04`; `4570` gates on an `r13 - 0x4690` flag
  and calls an unexplained `0x80227C70(0xE100)`; `4660` feeds mixed integer /
  double locals to `0x802E81D4` and `0x802E825C`, so the argument split is not
  provable.
- `0x802EA1BC`, `0x802EA2D4`, `0x802EA3B8`: the `0xD2` / `0xD3` / `0xD4` file
  request senders. In each export the reply handle is dereferenced outside the
  success branch (Ghidra shows `param_2 + 0x1C` even when the send failed),
  which cannot be reproduced without guessing the missing branch.
- `0x802E9D3C`: reply reader. The second payload read passes only a stack
  buffer, and the release call uses the allocation status word rather than the
  returned index; the export lost both operands.
- `0x802E8E1C`: cache range flush. It is `dcbst` / `dcbf` / `sync` / `icbi` /
  `isync`, which need cache-control intrinsics that this crate does not have.
- `0x802ECCB4`: exception prologue that saves every GPR/SPR and ends in
  `returnFromInterrupt`; not representable without inline assembly.
- The four-byte thunks `0x802E8608..0x802E8620`, `0x802E8868`, `0x802E942C`
  and `0x802E981C` are unconditional branches to already-translated bodies; a
  ledger row for each would only restate the target.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors and warnings for
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave68` reports `OK` with 29 rows claimed by
  `wave68-main-loop.csv`.
