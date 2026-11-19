# Coverage - Wave 33 (Bluetooth local-device bring-up and vendor-specific command manager)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 29 | 5,604 | 15 | 14 |

The 29 bodies form one contiguous call-graph cluster inside the Bluetooth band
`0x80200000..0x80230000`: the BTM local-device bring-up sequence and the
vendor-specific (`OGF 0xFC`) command manager. They all share the 240-byte
control block at `0x803D20E0..0x803D21CF` (`btm_cb`), which is what makes the
field offsets consistent across every function in this pass.

## What this pass covers

- **Bring-up state machine** - `fn_80202720` tears down the pending-callback
  slots, `fn_802027d0`/`fn_8020286c` request controller resets,
  `fn_80202914` reports "bring-up done", and `fn_80202cb8` handles the HCI
  reset command complete that starts the first step.
- **HCI command-complete handlers** - `fn_80202e7c` (`0x1005`,
  read-buffer-size), `fn_80202f44` (`0x1001`, read-local-version),
  `fn_80203000` (`0x1003`, read-local-features), `fn_802035b4` (`0x0C14`,
  read-local-name), `fn_80203680` (`0x1009`, read-`BD_ADDR`), and
  `fn_80203d1c`/`fn_80203d98`/`fn_80203df4` (the stored-link-key replies
  tagged `2`/`3`/`4`).
- **Local identity** - `fn_802034fc` writes the local name, `fn_80203640`
  hands out the local `BD_ADDR`, `fn_802036c8` copies the cached version
  words, `fn_80203714` sets the class of device, `fn_80203794` returns the
  cached class bytes, and `fn_802037b4` installs the device-ready callback.
- **Vendor-specific command manager** - `fn_802037cc` sends one `OGF 0xFC`
  command, `fn_802038c8` delivers its command complete, `fn_80203934`
  installs the vendor event callback, and `fn_8020396c` dispatches vendor
  events.
- **Link-key storage** - `fn_80203a04` writes the page timeout,
  `fn_80203aa4` requests stored link keys, `fn_80203b7c` writes up to eleven
  keys back to the controller, `fn_80203c4c` deletes stored keys, and
  `fn_80203e5c` handles the "return link keys" event, byte-swapping every key
  in place.

Every function is reached from, or is a direct callee of, the HCI
command-complete dispatcher at `0x8020B14C`; the opcode-to-handler mapping
above is taken from that dispatcher's own body, not guessed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80202720` | `fn_80202720` | 176 | high |
| `0x802027D0` | `fn_802027d0` | 156 | medium |
| `0x8020286C` | `fn_8020286c` | 168 | medium |
| `0x80202914` | `fn_80202914` | 28 | high |
| `0x80202930` | `fn_80202930` | 240 | high |
| `0x80202CB8` | `fn_80202cb8` | 432 | medium |
| `0x80202E7C` | `fn_80202e7c` | 180 | medium |
| `0x80202F44` | `fn_80202f44` | 168 | high |
| `0x80203000` | `fn_80203000` | 1,276 | medium |
| `0x802034FC` | `fn_802034fc` | 184 | high |
| `0x802035B4` | `fn_802035b4` | 140 | high |
| `0x80203640` | `fn_80203640` | 64 | high |
| `0x80203680` | `fn_80203680` | 72 | high |
| `0x802036C8` | `fn_802036c8` | 76 | high |
| `0x80203714` | `fn_80203714` | 128 | high |
| `0x80203794` | `fn_80203794` | 16 | high |
| `0x802037B4` | `fn_802037b4` | 24 | high |
| `0x802037CC` | `fn_802037cc` | 252 | medium |
| `0x802038C8` | `fn_802038c8` | 108 | medium |
| `0x80203934` | `fn_80203934` | 56 | high |
| `0x8020396C` | `fn_8020396c` | 152 | medium |
| `0x80203A04` | `fn_80203a04` | 160 | high |
| `0x80203AA4` | `fn_80203aa4` | 216 | medium |
| `0x80203B7C` | `fn_80203b7c` | 208 | high |
| `0x80203C4C` | `fn_80203c4c` | 208 | medium |
| `0x80203D1C` | `fn_80203d1c` | 124 | medium |
| `0x80203D98` | `fn_80203d98` | 92 | medium |
| `0x80203DF4` | `fn_80203df4` | 104 | medium |
| `0x80203E5C` | `fn_80203e5c` | 396 | medium |

## Deliberate skips

These addresses sit in the same band and share the same control block but were
not translated because the evidence is not sufficient to reconstruct them
honestly:

- `0x80202680` - the export reads one of its inputs from `unaff_r13` as a
  three-byte data source; the register is not set up in the visible body, so
  the argument cannot be recovered.
- `0x80202A20` - Ghidra removed two unreachable blocks inside the state
  machine, leaving the fall-through path ambiguous.
- `0x80203FE8` - 28 bytes that end in an unrecovered jump table; the indirect
  tail dispatch is not in the export.
- `0x80204008`, `0x80204080` - these operate on the *next* state block
  (`0x803D21D0`), not the `btm_cb` block shared by this cluster, so they belong
  to a later pass.

`0x802DCD74`, called from `fn_802037cc`, is not translated here: its two
halves are just register saves whose values are the caller's own `r3`/`r4`
arguments, so the body is modelled without a named callee and the elision is
documented in that function's doc block.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path
  (`work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/runtime/bt.rs`, and `node tools/doc-lint.mjs src` reports zero
  violations.
