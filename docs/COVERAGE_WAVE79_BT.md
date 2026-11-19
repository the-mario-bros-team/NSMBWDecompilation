# Coverage - Wave 79 (`runtime::bt`, HCI command/event core)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. All addresses are inside this lane's `0x80200000` 64 KiB
bucket, which held 256 functions with 124 untranslated when this pass started.

The module is *not* gameplay code: it is Broadcom's BTE host stack plus the
platform glue that shipped in the same address lane. This pass translates the
part of the stack that actually speaks HCI - the command-complete / command-
status matchers, the opcode dispatch tree they feed, and the BTM handlers that
those dispatches above land in.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 26 | 5720 | 10 | 16 |

## What this pass covers

- **HID-host slot helpers** (`0x802011D8..0x80201514`, 4 bodies). The
  disconnect-reason getter, the `BD_ADDR`-to-slot-handle lookup and the `u16`
  slot store that shares the four-slot `0x11C`-stride table with the wave 5
  function `fn_8020028c`, plus the role-change completion block that clears
  `G_ROLE_CALLBACK` before invoking it.
- **ACL payload-size table** (`0x80201570`). A nested packet-type bit test that
  maps a link's mask (`slot+2`, or the global mask when the address is the local
  device) to one of eleven classic ACL payload sizes and clamps the largest to
  the controller's reported buffer size.
- **BTM command-complete handlers** (`0x80201AA4..0x802057E8`, 4 bodies). The
  read-RSSI and read-link-quality completions (timer cancel, single-shot
  callback, nine-byte event block, peer-address copy), the `0x0406` disconnect
  request for a remote address, and the inquiry-complete teardown.
- **Small BTM helpers** (`0x80205FEC..0x802089A4`, 4 bodies). The `0x22`-byte
  SCO pool record clear, the four-word default link-parameter copy, the
  security-mode store with its `0xFF` key scratch, and the gate that re-sends
  the two link-policy builders when the security mode is `3`.
- **HCI command/event core** (`0x8020AE04..0x8020BAC4`, 10 bodies). This is the
  hub of the host stack:
  - four payload parsers (`0x8020AE04`, `0x8020AEE0`, `0x8020AF6C`,
    `0x8020B06C`) that unpack a handle, a length, a reversed six-byte address
    and a small option block, then call the L2CAP connection dispatcher
    `0x80206E50` or the QoS completion handler `0x80201754`,
  - the opcode dispatch tree `fn_8020b14c`: `0x0C03`, `0x080C`, `0x0C0D`,
    `0x0C11`, `0x0C12`, `0x1001`/`0x1003`/`0x1005`/`0x1009`, `0x1403`,
    `0x1405` and the `0xFC00` vendor range,
  - the command-complete matcher `fn_8020b2a4` and the command-status
    matcher `fn_8020b5bc`, which pop the queued command, verify the opcode,
    arm or cancel the retry timer and free the buffer,
  - the status-aware dispatcher `fn_8020b3bc` (`0x0801`/`0x0803..0x0806`
    link-control group, `0x0419` remote name, `0x0401`, `0x080B` role switch,
    `0x0405`, `0x0428`),
  - the timeout redispatch `fn_8020b6e4` that replays the timed-out opcode
    with status `0x1F`,
  - the link-key notification splitter `fn_8020b83c`.
- **HCI builders and pool teardown** (`0x8020D244`, `0x8020FE8C`). The
  `0x040C` six-byte-address command builder, and the teardown that walks all
  sixteen `0x34`-byte HID device records, cancels their timers, clears both
  channel ids and deregisters the L2CAP PSM pair.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802011D8` | `fn_802011d8` | 16 | high |
| `0x802011E8` | `fn_802011e8` | 156 | high |
| `0x80201284` | `fn_80201284` | 156 | medium |
| `0x80201514` | `fn_80201514` | 92 | medium |
| `0x80201570` | `fn_80201570` | 416 | medium |
| `0x80201AA4` | `fn_80201aa4` | 284 | high |
| `0x80201BC0` | `fn_80201bc0` | 280 | high |
| `0x80201CD8` | `fn_80201cd8` | 188 | medium |
| `0x802024E8` | `fn_802024e8` | 408 | medium |
| `0x802057E8` | `fn_802057e8` | 260 | medium |
| `0x80205FEC` | `fn_80205fec` | 80 | high |
| `0x802068C4` | `fn_802068c4` | 68 | medium |
| `0x80208988` | `fn_80208988` | 28 | high |
| `0x802089A4` | `fn_802089a4` | 64 | high |
| `0x8020AE04` | `fn_8020ae04` | 220 | medium |
| `0x8020AEE0` | `fn_8020aee0` | 140 | medium |
| `0x8020AF6C` | `fn_8020af6c` | 256 | medium |
| `0x8020B06C` | `fn_8020b06c` | 224 | medium |
| `0x8020B14C` | `fn_8020b14c` | 340 | high |
| `0x8020B2A4` | `fn_8020b2a4` | 280 | medium |
| `0x8020B3BC` | `fn_8020b3bc` | 512 | medium |
| `0x8020B5BC` | `fn_8020b5bc` | 296 | medium |
| `0x8020B6E4` | `fn_8020b6e4` | 344 | medium |
| `0x8020B83C` | `fn_8020b83c` | 224 | medium |
| `0x8020D244` | `fn_8020d244` | 160 | high |
| `0x8020FE8C` | `fn_8020fe8c` | 228 | high |

## Why so many are `medium`

- `0x8020B2A4` and `0x8020B5BC` are printed as parameterless because they open
  with the `0x802DCD74` register-save prologue and Ghidra assigns the call's
  result to the local that holds the incoming `r3`. The thunk only stores
  `r27..r31` to the frame, so `r3` survives it unchanged; every use of the
  recovered value in both bodies is a byte load, which confirms the recovery.
  The same pattern is already documented for waves 60, 68 and 69.
- `0x8020AE04`, `0x8020AEE0`, `0x8020AF6C` and `0x8020B06C` unpack fields whose
  roles are not proven: the three-byte tag, the two "halves" and the four
  option bytes are stored exactly where the callee reads them, but no export in
  this lane names them.
- `0x802057E8` calls `FUN_801F7240()` with no operand in the export; the freed
  block is inferred to be `BTM_INQ_BUFFER_3254`, which is tested for non-zero
  and cleared one statement later, and the body says so.
- `0x802024E8` and `0x802068C4` minimise/copy a `u32` whose "load" and "link
  parameter" meanings are inferred from use, not from a symbol.
- `0x80201570`'s individual packet-type bits are only identified by their
  payload size, so the branch tests keep the raw masks instead of names.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave79-bt.csv`.

- `0x80201320` (500 bytes, role-switch event) - opens with `FUN_802DCD6C` and
  then uses the low half of the fake 64-bit result as a pointer and the high
  half as a status byte. Which register is `r3` cannot be settled from the C,
  so no honest signature exists.
- `0x80201D94` (544 bytes, SCO packet-type restore) - `FUN_802DCD68()` returns
  a value that is compared against zero, but the export never shows whether
  that is the function's own first argument or a value produced by the thunk.
- `0x80203FE8` (28 bytes, ready-callback trampoline) - the body calls the
  installed callback with no visible argument, while its only caller puts `2`
  in `r3`. Forwarding that value would be a guess; the trampoline is still
  called by address from `fn_8020b6e4`.
- `0x80205F74` (116 bytes, BTM queue drain) - indexes 8-byte entries from
  `0x803D20CC`, a region that also holds five unrelated BTM callback words
  (`0x803D20D8`..`0x803D20E4`). The table shape cannot be recovered from the C.
- `0x8020A060` (80), `0x8020A68C` (52), `0x802092FC` (64) - each calls
  `FUN_802023A8()`/`FUN_80202460()` with the argument list dropped by Ghidra,
  even though those callees take a handle or an address. The value passed is
  unrecoverable.
- `0x8020CD34` (192), `0x8020CDF4` (224), `0x8020DCE4` (204) - HCI builders that
  consume `extraout_r4`, i.e. an operand the decompiler lost.
- `0x8020D9EC` (252), `0x8020DAE8` (232) - take the six-byte address in
  registers through the multi-register prologue result; which half of the fake
  value is which register is not consistent between the two exports.
- `0x80204138`, `0x80205088`, `0x802072F4` (4 bytes each) and the remaining
  4-byte branch thunks - single `b` instructions to already documented callees,
  so a body here would be a duplicate.
- The `0x80207D90..0x8020A6C0` control-block cluster, the
  `0x8020F538..0x8020FF70` pool code and the `0x8020E39C` multi-key builder
  were not opened this pass.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  total 5720 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 26 were absent from the
  bucket's 124 untranslated entries).
- `cargo check --message-format=short` reports no errors and no new warnings in
  `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave79` reports `wave79-bt.csv: 26 rows` with
  no problem naming that file.
