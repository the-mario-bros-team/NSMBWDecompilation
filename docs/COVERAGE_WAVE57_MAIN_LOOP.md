# Coverage - Wave 57 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the MetroTRK debugger module. Wave 36 translated
the debugger's core `0x802E8E54..0x802E9E84`; this pass adds the debug port's
UART ring buffer (`0x802E886C..0x802E8D14`), the three TRK message buffers at
`0x80425720` (`0x802EA820..0x802EA9E4`) and the small SDA knob getters /
setters used by the interrupt path (`0x802EB090`, `0x802ED2E4..0x802ED9CC`).
Each body cites its Ghidra name, the `functions.tsv` byte size and the exact
export path in that order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 30 | 2064 | 23 | 7 |

Before this pass the file held 30 bodies (28 translated + 2 partial boot
entries); it now holds 60.

## What this pass covers

- **UART ring buffer, control block** (`0x802E8BE8`, `0x802E8BE0`,
  `0x802E8C0C`, `0x802E8D14`, `0x802E8BA8`, `0x802E8BD8`): the six-word
  constructor at `0x804256B0` (storage `0x804251B0`, capacity `0x500`), the
  fill counter at `+0x10`, and the wrapping append / drain routines that
  bracket their memcpy with the `0x801B1140` / `0x801B1180` interrupt
  mask pair.
- **UART entry points** (`0x802E892C`, `0x802E8A50`, `0x802E8AC4`,
  `0x802E8B0C`, `0x802E8B80`, `0x802E899C`): port setup, the chunked
  blocking write, the flush / reinit wrappers, and the two blocking reads
  that stage through the export's 1284-byte stack buffer.
- **String compare** (`0x802E886C`): the font-table translated `strcmp`
  used by the debugger's command matcher.
- **Message buffers** (`0x802EA820`, `0x802EA884`, `0x802EA8A4`,
  `0x802EA940`, `0x802EA964`, `0x802EA98C`, `0x802EA9B4`, `0x802EA9E4`):
  the three-slot 0x88C-byte pool at `0x80425720`, its sequence stamping
  transmit path, allocation scan, release, reset and bounded append.
- **SDA knobs** (`0x802EB090`, `0x802ED2E4`, `0x802ED4E8`, `0x802ED4F8`,
  `0x802ED508`, `0x802ED91C`, `0x802ED92C`, `0x802ED9C4`, `0x802ED9CC`):
  the `0x8042B054` word, the `0x804272C0` selector, the `0x80427230`
  request-path enable word, the `0x80427238` handler pointer, the
  `| 0x80000000` address tag, and the `0x8042B060` byte flag.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E886C` | `fn_802e886c` | 160 | medium |
| `0x802E892C` | `fn_802e892c` | 60 | medium |
| `0x802E899C` | `fn_802e899c` | 180 | medium |
| `0x802E8A50` | `fn_802e8a50` | 112 | medium |
| `0x802E8AC4` | `fn_802e8ac4` | 36 | high |
| `0x802E8B0C` | `fn_802e8b0c` | 116 | high |
| `0x802E8B80` | `fn_802e8b80` | 36 | high |
| `0x802E8BA8` | `fn_802e8ba8` | 48 | medium |
| `0x802E8BD8` | `fn_802e8bd8` | 8 | medium |
| `0x802E8BE0` | `fn_802e8be0` | 8 | high |
| `0x802E8BE8` | `fn_802e8be8` | 40 | high |
| `0x802E8C0C` | `fn_802e8c0c` | 264 | high |
| `0x802E8D14` | `fn_802e8d14` | 264 | high |
| `0x802EA820` | `fn_802ea820` | 100 | high |
| `0x802EA884` | `fn_802ea884` | 32 | high |
| `0x802EA8A4` | `fn_802ea8a4` | 156 | high |
| `0x802EA940` | `fn_802ea940` | 36 | high |
| `0x802EA964` | `fn_802ea964` | 40 | high |
| `0x802EA98C` | `fn_802ea98c` | 36 | high |
| `0x802EA9B4` | `fn_802ea9b4` | 48 | high |
| `0x802EA9E4` | `fn_802ea9e4` | 164 | high |
| `0x802EB090` | `fn_802eb090` | 8 | medium |
| `0x802ED2E4` | `fn_802ed2e4` | 16 | high |
| `0x802ED4E8` | `fn_802ed4e8` | 16 | high |
| `0x802ED4F8` | `fn_802ed4f8` | 16 | high |
| `0x802ED508` | `fn_802ed508` | 24 | high |
| `0x802ED91C` | `fn_802ed91c` | 16 | high |
| `0x802ED92C` | `fn_802ed92c` | 8 | high |
| `0x802ED9C4` | `fn_802ed9c4` | 8 | high |
| `0x802ED9CC` | `fn_802ed9cc` | 8 | high |

## Deliberate skips

- `0x802E8AE8` (36 B, wrapper over `0x8022863C`): identical shape to
  `0x802E8AC4`, left out because it is one of a pair of 4-byte thunk
  callees and adds nothing to the port story.
- `0x802E90B8` (296 B, `memset`) and `0x802E8F7C` (316 B, `memcpy`): the
  unaligned inline loops are called by address from the new bodies; a byte
  loop would change the literal access pattern, so they stay for a
  memory-lane pass.
- `0x802E9EA0` (500 B, multi-packet send) and `0x802EA094`
  (296 B, request-send validation): they interleave with the skipped
  `memset` / reply reader and with `0x802EBxxx` host handlers; the buffer
  primitives they need are now translated, so they are the obvious next
  step.
- `0x802EA6BC` (8 B): two byte stores at `0x804270C8` / `0x804270C9`, but
  the field roles are unknown and it belongs to the interrupt cluster
  around `0x802ECCB4`, which needs inline assembly (`returnFromInterrupt`).
- `0x802ECCB4` (280 B): exception prologue saving every GPR/SPR and ending
  in `returnFromInterrupt`; not representable without inline assembly.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error or warning in
  `src/engine/main_loop.rs`.
- `node tools/check-lane-csv.mjs wave57` is clean for
  `wave57-main-loop.csv`.
