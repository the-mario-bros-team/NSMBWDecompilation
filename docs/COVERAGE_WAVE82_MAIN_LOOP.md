# Coverage - Wave 82 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the `0x802E0000` bucket. Wave 82 adds 25 bodies
inside it, in four clusters: the unaligned storage primitives that the earlier
TRK bodies already called by address, three leftover C-runtime routines, the
MetroTRK message handlers and the breakpoint / watchpoint knobs at the top of
the bucket.

Each row below cites the Ghidra name, the `functions.tsv` byte size and the
exact export path, in that order. "Confidence" is copied from the body's
`# Confidence` line.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 25 | 6848 | 15 | 10 |

Before this pass the file held 89 bodies; it now holds 114.

## What this pass covers

- **Storage primitives** (`0x802E8F7C`, `0x802E90B8`): the unaligned byte copy
  and byte fill that every TRK payload writer calls by address. Both exports
  unroll four bytes at a time with read-modify-write word accesses; the Rust
  bodies keep the same byte order and touch the same bytes, collapsing only the
  word-sized addressing. `0x802E8F7C` is now also used directly by
  [`fn_802eabe8`] instead of `platform::call3`.
- **C runtime leftovers** (`0x802E1938`, `0x802E2108`, `0x802E21D4`): `strcpy`
  (its aligned fast path writes the identical bytes, the same collapse Wave 68
  applied to `strcmp`), `wmemchr`, and the `atoi` wrapper that runs the shared
  parser `0x802E1CF4` in base 10 with a `LONG_MAX` clamp and stores `ERANGE`
  (`0x22`) in `errno` at `0x8042AFE8`.
- **Message layer** (`0x802E91E0`, `0x802EABE8`, `0x802EA4C4`,
  `0x802EB268..0x802EBF48`): the command dispatcher (12 cases, default `0x500`),
  the 64-bit payload append, the interrupt teardown, and the four block
  request handlers plus the continue / breakpoint / status / serial-option
  handlers. All of them build the same `0x40`-byte header the Wave 68
  notification senders documented: `+0x00` total length, `+0x04` type `0x80`
  or `0x91`, `+0x08` reply code, `+0x0C` sequence.
- **High end** (`0x802EC1B0`, `0x802ED028`, `0x802ED0C4`, `0x802ED1E8`,
  `0x802ED270`, `0x802ED2F4`, `0x802ED8B4`, `0x802ED934`): the cached-window
  containment test, the `0x91` support reply, the three watchpoint knobs and
  the pending-request state machine (D0/D1 block transfer, D2/D3/D4 single
  word, otherwise post a 4-byte event), the `blr`-patch-and-call helper and
  the thread-queue index walk.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E1938` | `fn_802e1938` | 192 | high |
| `0x802E2108` | `fn_802e2108` | 196 | medium |
| `0x802E21D4` | `fn_802e21d4` | 40 | high |
| `0x802E8AE8` | `fn_802e8ae8` | 36 | medium |
| `0x802E8F7C` | `fn_802e8f7c` | 316 | high |
| `0x802E90B8` | `fn_802e90b8` | 296 | high |
| `0x802E91E0` | `fn_802e91e0` | 288 | high |
| `0x802EA4C4` | `fn_802ea4c4` | 52 | high |
| `0x802EABE8` | `fn_802eabe8` | 244 | high |
| `0x802EB268` | `fn_802eb268` | 564 | medium |
| `0x802EB49C` | `fn_802eb49c` | 528 | medium |
| `0x802EB6AC` | `fn_802eb6ac` | 508 | medium |
| `0x802EB8A8` | `fn_802eb8a8` | 672 | medium |
| `0x802EBB48` | `fn_802ebb48` | 196 | high |
| `0x802EBC0C` | `fn_802ebc0c` | 644 | medium |
| `0x802EBE90` | `fn_802ebe90` | 184 | high |
| `0x802EBF48` | `fn_802ebf48` | 204 | high |
| `0x802EC1B0` | `fn_802ec1b0` | 248 | high |
| `0x802ED028` | `fn_802ed028` | 156 | medium |
| `0x802ED0C4` | `fn_802ed0c4` | 292 | high |
| `0x802ED1E8` | `fn_802ed1e8` | 132 | high |
| `0x802ED270` | `fn_802ed270` | 112 | high |
| `0x802ED2F4` | `fn_802ed2f4` | 500 | medium |
| `0x802ED8B4` | `fn_802ed8b4` | 104 | high |
| `0x802ED934` | `fn_802ed934` | 144 | medium |

## Deliberate skips

- **`0x802EC374`, `0x802EC4C4`, `0x802EC5BC`, `0x802EC6F8`, `0x802EC85C`** - the
  register/memory block transfers. Every one of them writes
  `DAT_8035162c._1_3_ = (uint3)DAT_8035162c._2_2_;` at entry and then tests
  `_1_1_`. Reproducing that byte-wide shuffle from the C export would mean
  guessing which of three stack/global bytes the original clears, and the
  interrupt handler may set it asynchronously; they are still called by
  address from the new handlers. `0x802EC5BC` / `0x802EC85C` additionally lose
  the second argument (`extraout_r4`) in the export.
- **`0x802EC014`, `0x802EC07C`, `0x802EC0E4`, `0x802EC0EC`, `0x802EC0F4`** - the
  cache-flush wrappers. They forward to the 8-byte thunks whose fifth argument
  (`r7`) is undefined at the call site, and the inner call target
  `0x802E890C` is a bare `trapWord(0x1f, ...)` that the runtime patches. A
  faithful call would need the missing argument.
- **`0x802EC2A8`** - byte copy bracketed by `FUN_802EC1A0` / `FUN_802EC1A8` and
  `sync(0)`. The barrier hooks are `blr`-only stubs and the `sync` instruction
  has no Rust equivalent in `src/platform.rs`, so the body would be a partial
  translation; left out rather than misrepresented.
- **`0x802E9CC8`** - sets the wire byte order (`r13 - 0x4648`) to 1 and returns
  0, but Ghidra removed three unreachable blocks, so the visible body is not
  the whole function.
- **`0x802ED520`, `0x802ED5FC`, `0x802ED6EC`, `0x802ED6A4`, `0x802ED6C8`** -
  the instruction-patch builders. Their bodies copy constant tables from
  `0x802FE320` / `0x802FE344` / `0x802FE368`, which are not in this export and
  whose bytes would have to be invented. `0x802ED6A4` / `0x802ED6C8` also read
  the uninitialised `GQR0` pseudo-register.
- **`0x802E9D3C`**, **`0x802EA1BC`**, **`0x802EA2D4`**, **`0x802EA3B8`** - still
  skipped for the reasons Wave 68 documented (lost operands in the reply
  reader and the three request senders). They are called by address here.
- **`0x802E8E1C`, `0x802ECCB4`** and the `0x802E8608..0x802E8620` thunks - cache
  control, exception prologue and pure branches; unchanged from earlier waves.
- The remaining Libm bodies (`0x802E7904..0x802E8620`) were left for a dedicated
  floating-point pass: their arguments live in `f1..f8` and Ghidra renders the
  constant pool as `*(double *)(unaff_r2 - N)`, so the argument order is not
  provable from the C export.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`; the file now holds 114 bodies and no duplicates.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/engine/main_loop.rs` (the only build errors come from other Wave 82
  lanes' files).
- `node tools/check-lane-csv.mjs wave82` reports `OK` for
  `ledger/agents/wave82-main-loop.csv` with 25 rows.
