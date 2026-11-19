# Coverage - Wave 66 (`runtime::bt`, BTM/HCI continuation)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. All addresses are inside this lane's `0x80200000` 64 KiB
bucket, which held 256 functions with 154 untranslated when this pass started.

The block continues the BTM/HCI code that waves 53 and 58 started rather than
the `0x80220000` SDP layer: connection-mode command builders, the BTM
connection-slot and inquiry tables, the security application registration
slots, and two pool helpers. Layer names are `// inferred:` from the trace
strings each export passes (`BTM_SetPinType`, `BTM_CancelRemoteDeviceName`,
`BTM_InqDbRead`, ...) and from the HCI opcodes in the packet headers; the
shipped image has no symbol table.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 30 | 3908 | 25 | 5 |

## What this pass covers

- **HCI connection-mode builders** (`0x8020DBD0..0x8020E33C`, 8 bodies). Six
  allocate their own size-class `2` buffer, two take a caller buffer. The
  headers are the same shape already documented for the wave-5 builder family
  (`+2` tag, `+4` zero, `+8` little-endian opcode, `+0xA` length, `+0xB`
  params), with opcodes `0x042A`, `0x0801`, `0x0804`, `0x0805`, `0x0806`,
  `0x0C03`, `0x0C0A` and `0x0C0D`. Two builders reverse the six-byte
  `BD_ADDR`.
- **BTM connection slots and security** (`0x802070A8..0x80207818`, 9 bodies).
  The three `0x34`-byte records at `0x803D33CC` (already named
  [`CONN_SLOT_TABLE`](src/runtime/bt.rs)) drive the occupied-slot release by
  `BD_ADDR`, the active-link count, the handle lookup, the connected-slot
  lookup and the removal path. The same block adds the six-word security
  application registration at `0x803D3484`, its three-handle table at
  `0x803D34A4`, and the PIN setter.
- **BTM inquiry and mode control** (`0x80204008..0x80204B6C`, 8 bodies). The
  SDP discovery-database builder and its teardown, the three connection-mode
  setters (capability bits `0x10`/`0x20`/`0x40`), the inquiry-state getter,
  the remote-name cancel and the twelve-record inquiry database reader.
- **BTM bring-up** (`0x80205B94..0x80205C30`, 3 bodies). Two table resets, the
  `0x27C4`-byte global clear with the re-init chain, and the three-slot
  service registration that stores a context word and a mode byte.
- **Pool helpers** (`0x8020FDCC..0x8020FE10`, 2 bodies). The `0x404`-byte
  clear and the enable/store path.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80204008` | `fn_80204008` | 120 | high |
| `0x80204080` | `fn_80204080` | 184 | high |
| `0x802043C0` | `fn_802043c0` | 176 | high |
| `0x80204470` | `fn_80204470` | 176 | high |
| `0x80204520` | `fn_80204520` | 152 | high |
| `0x80204740` | `fn_80204740` | 16 | high |
| `0x80204AF4` | `fn_80204af4` | 120 | high |
| `0x80204B6C` | `fn_80204b6c` | 192 | high |
| `0x80205B94` | `fn_80205b94` | 64 | medium |
| `0x80205BD4` | `fn_80205bd4` | 92 | medium |
| `0x80205C30` | `fn_80205c30` | 188 | high |
| `0x802070A8` | `fn_802070a8` | 132 | medium |
| `0x8020712C` | `fn_8020712c` | 276 | medium |
| `0x802075A4` | `fn_802075a4` | 112 | high |
| `0x80207614` | `fn_80207614` | 96 | high |
| `0x80207674` | `fn_80207674` | 136 | high |
| `0x802076FC` | `fn_802076fc` | 120 | high |
| `0x80207774` | `fn_80207774` | 80 | high |
| `0x802077C4` | `fn_802077c4` | 84 | high |
| `0x80207818` | `fn_80207818` | 176 | high |
| `0x8020DBD0` | `fn_8020dbd0` | 96 | high |
| `0x8020DC30` | `fn_8020dc30` | 180 | high |
| `0x8020DDB0` | `fn_8020ddb0` | 132 | high |
| `0x8020DE34` | `fn_8020de34` | 180 | high |
| `0x8020DEE8` | `fn_8020dee8` | 132 | high |
| `0x8020E0B0` | `fn_8020e0b0` | 92 | high |
| `0x8020E2C8` | `fn_8020e2c8` | 116 | high |
| `0x8020E33C` | `fn_8020e33c` | 96 | high |
| `0x8020FDCC` | `fn_8020fdcc` | 68 | high |
| `0x8020FE10` | `fn_8020fe10` | 124 | medium |

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave66-bt.csv`.

- `0x80203FE8` (28 bytes) - a single indirect dispatch through
  `DAT_803D20E0` with no argument list; Ghidra also reports an unrecovered
  jump table. The body would be a guess about which registers are forwarded.
- `0x80204138` (4 bytes) - an empty `blr` slot; no behaviour to reconstruct.
- `0x80204750` (264 bytes) - the cancel-inquiry path. The export calls
  `FUN_801F7240()` with no argument even though every other caller passes a
  block address, so the free target cannot be reconstructed honestly.
- `0x80204858` (436 bytes) - `BTM_StartInquiry`. The mode byte is built with a
  sign trick (`~((*p - 1 >> 24) | (1 - *p >> 24)) >> 7`) and the function
  mutates its input buffer (`param_1[3] = 0`); reproducing it without the
  disassembly would mix inference into observable stores.
- `0x80204C2C` (184) and `0x80204CE4` (316) - both return a 64-bit
  `CONCAT44(pointer, index)` pair. The decompiler's return convention is not
  the C ABI used elsewhere in this module, so the two results would have to be
  guessed.
- `0x802072F8` (424), `0x802074A0` (260), `0x802078C8` (552), `0x80207AF0`
  (196) - the eSCO change handlers. Their exports open with the
  `FUN_802DCDxx` non-volatile save/restore thunks and consume the folded
  64-bit result as two real arguments; the register pairing cannot be proven
  from the C alone.
- `0x8020D9EC` (252), `0x8020DAE8` (232), `0x8020DCE4` (204), `0x8020E39C`
  (448) - HCI builders whose arguments arrive through the same thunk or an
  `extraout` register.
- `0x8020FE8C` (228) and `0x8020FF70` (400) - pool teardown and lookup. The
  first is readable but needs the same thunk handling as its neighbours; the
  second depends on `extraout_r4` for the handle written at `+0x18`.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  total 3908 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 30 were absent from the
  `0x80200000` bucket's 154 untranslated entries).
- `cargo check --message-format=short` reports no errors in
  `src/runtime/bt.rs`; the only warnings in the file are the ten pre-existing
  `unnecessary parentheses around function argument` warnings.
- `node tools/check-lane-csv.mjs wave66` reports `OK` with
  `wave66-bt.csv: 30 rows` (30 unique addresses).
