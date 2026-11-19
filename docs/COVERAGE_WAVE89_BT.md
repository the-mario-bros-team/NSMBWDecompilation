# Coverage - Wave 89 (`runtime::bt`, BTM role/record helpers and the GAP connection block)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. All addresses are inside this lane's `0x80200000` bucket,
which held 256 functions with 98 untranslated when this pass started.

This pass closes holes inside three areas that waves 66, 79 and 82 had
already opened:

- the HID-host role-switch completion code immediately after the slot helpers
  of wave 79,
- the BTM device-record/inquiry helpers between the connection database and
  the bring-up state machine,
- the GAP connection-slot block (`0x803D43EC`) that feeds the L2CAP
  configuration and disconnect primitives translated in wave 82.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 26 | 9332 | 9 | 17 | 0 |

## What this pass covers

- **HID-host role switch** (`0x80200968`, `0x80201320`). The first completes or
  aborts a switch selected by connection handle: the slot's `+0x11B` marker
  moves between `2`, `3` and `0`, HCI `0x080B` is queued through
  `fn_8020df6c`, and the pending `0x803D21A4` callback receives the eight-byte
  role block. The second is the controller-event side: it stores the new role
  at `0x803D219C`, copies the negotiated remote timeout to the peer slot,
  advances marker `3` to `4` through HCI `0x0413`, or clears the marker and
  fires the callback.
- **BTM RSSI / link-quality request twins** (`0x8020182C`, `0x80201968`). Both
  gate on a pending word (`0x803D2158` / `0x803D2174`), find the slot by
  `BD_ADDR`, arm their own timer, and queue HCI `0x05`/`0x03` through
  `0x8020ED2C`/`0x8020ECB0`; result codes are `1`, `2`, `3` and `7`.
- **eSCO packet-type repair** (`0x80201D94`, `0x802072F8`). The first scans the
  active slots and either applies a QoS window (`0x80205CEC`) or recomputes the
  `0xCC18`/`0x3306` packet mask and queues `0x040F`. The second updates one
  `0x803D33CC` connection slot: the short form writes `mask & 7` to `+0x20`;
  the extended form widens it with `0x3C0` and calls the seven-argument
  `0x8020D9EC` with the stored bandwidth and latency words.
- **BTM device records** (`0x80201FB4..0x802023A8`). Four bodies cover creating
  or updating a record (three-byte class at `+0x22`, `0x41`-byte name at
  `+0x35`, eight feature bytes at `+0x77`, link key at `+0x25`), the free-list
  scan and "least loaded" overflow of `0x80202288`, the name-buffer lookup at
  `+0x35`, and the `u16` handle lookup at `+0x18`.
- **BTM defaults and retry** (`0x80202680`, `0x80202A20`). The first restores
  the local COD, page timeout, packet masks, `0xFF` defaults and state byte,
  then sends `0x0C03`. The second is the retry timer behind that reset: it
  counts `0x803D21C7` down, re-queues reset/class/name/PIN commands and
  cancels the pending name callback on timer id `2`.
- **Mode, inquiry and remote-name helpers** (`0x80203FE8`, `0x802045B8`,
  `0x8020508C`, `0x80205168`, `0x802058EC`). These are the device-ready
  forwarder, the connectability setter, the fixed inquiry-result buffer
  insert, the twelve-record inquiry-database allocator with its eviction
  callback, and the remote-name request gate with its pending mask and timer.
- **GAP connection block** (`0x8020BCD0..0x8020CA18`, 9 bodies). The eight
  `0x60`-byte records at `0x803D43EC` are resolved by CID and driven through
  configuration request/response, disconnect, congestion, discovery-complete
  and remote-name-complete paths. Three of them share the queue drain
  (`+0x14`), the token copy (`+0x10`) and the re-resolution scan that returns
  the index of the surviving record or the `0x80212810` deregister result.

The GAP record layout used by the nine bodies:

| Offset | Field |
|---:|---|
| `+0x00` | state byte (`0` free, `2` configuring, `3` configured, `4` open) |
| `+0x01` | flags (`0x02` config sent, `0x04` ready) |
| `+0x04` | `u16` connection handle |
| `+0x06` | `u16` local CID |
| `+0x08` | "any address" byte |
| `+0x09` | six-byte `BD_ADDR` |
| `+0x10` | `u16` token used to find the surviving record |
| `+0x12` | `u16` configuration result |
| `+0x14` | pending-frame queue head |
| `+0x20` | per-connection callback |
| `+0x24` | spare word passed to the configuration request |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80200968` | `fn_80200968` | 364 | medium |
| `0x80201320` | `fn_80201320` | 500 | medium |
| `0x8020182C` | `fn_8020182c` | 316 | high |
| `0x80201968` | `fn_80201968` | 316 | high |
| `0x80201D94` | `fn_80201d94` | 544 | medium |
| `0x80201FB4` | `fn_80201fb4` | 392 | medium |
| `0x802021E8` | `fn_802021e8` | 160 | high |
| `0x80202288` | `fn_80202288` | 288 | medium |
| `0x802023A8` | `fn_802023a8` | 184 | high |
| `0x80202680` | `fn_80202680` | 160 | high |
| `0x80202A20` | `fn_80202a20` | 644 | medium |
| `0x80203FE8` | `fn_80203fe8` | 28 | medium |
| `0x802045B8` | `fn_802045b8` | 392 | medium |
| `0x8020508C` | `fn_8020508c` | 220 | medium |
| `0x80205168` | `fn_80205168` | 260 | high |
| `0x802058EC` | `fn_802058ec` | 336 | medium |
| `0x802072F8` | `fn_802072f8` | 424 | medium |
| `0x8020BCD0` | `fn_8020bcd0` | 476 | high |
| `0x8020BEAC` | `fn_8020beac` | 316 | medium |
| `0x8020BFE8` | `fn_8020bfe8` | 492 | high |
| `0x8020C1D4` | `fn_8020c1d4` | 504 | medium |
| `0x8020C3CC` | `fn_8020c3cc` | 272 | medium |
| `0x8020C4DC` | `fn_8020c4dc` | 308 | high |
| `0x8020C610` | `fn_8020c610` | 472 | medium |
| `0x8020C800` | `fn_8020c800` | 536 | medium |
| `0x8020CA18` | `fn_8020ca18` | 428 | medium |

## Why the seventeen `medium` rows are `medium`

- `0x80200968`, `0x80201320`, `0x80201D94`, `0x80201FB4`, `0x802045B8`,
  `0x802058EC` and `0x802072F8` each open with a `0x802DCD64..74`
  register-save prologue. Ghidra prints no parameter list; the two halves of
  the prologue result are the incoming `r3`/`r4`, which the bodies take as
  ordinary arguments. The stores and branches are literal.
- `0x80202288` and `0x80202A20` call builders whose arguments Ghidra dropped
  (`FUN_8020EC28()`, `FUN_80212D1C()` and the two `FUN_801F7240` frees). The
  bodies supply the argument from the allocation immediately above the call
  and say so inline. The retry state machine itself is exact.
- `0x80203FE8` shows an indirect jump with no visible parameter setup, so the
  forwarded callback's argument count is marked `// unknown:`; the guard is
  literal.
- `0x8020508C` returns its `1`/`0` through the register-restore epilogue
  rather than the C return value. The gate, the `0x0C` stride walk and the
  append are literal.
- `0x8020BEAC`, `0x8020C1D4`, `0x8020C3CC`, `0x8020C610`, `0x8020C800` and
  `0x8020CA18` all reproduce the exported text, including the decompiler's
  initialised-pointer return (`0x803D4340`), the constant-masked
  `0x07FC2BCC` comparison and the dropped queue-push buffer argument. Those
  quirks are documented at the site instead of being "cleaned up".

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave89-bt.csv`.

- `0x8020BB44` (396) - the GAP connection-indication entry. It consumes both
  halves of the `0x802DCD70` prologue as a CID (`r3` low half) and a request
  block (`r4` high half), then hands them to `0x80212AD4`/`0x80212C64`. The
  half order is only visible through the two calls, and the export dereferences
  a null `uVar1` in the log path; it needs the same per-register review as
  `0x8020D9EC` below.
- `0x8020413C` (644) - the discoverability twin of `0x802045B8`. It reads a
  third prologue argument through `unaff_r2 + -0x4C88` (a stack slot) that the
  decompiler does not name, so the two address arguments cannot be recovered
  honestly from this export.
- `0x8020526C` (344) - the three-byte/three-byte address splitter. Its control
  flow branches on a 64-bit comparison of the prologue pair and then on
  `lVar5 < 0x100000000` / `0x300000000`; the packed argument layout is not
  proven.
- `0x80205350` (544) and `0x80205570` (632) - the inquiry start and result
  handlers around `0x802058EC`. Both index `unaff_r2 + -0x4C88`, the same
  unnamed stack slot as `0x8020413C`, for the address passed to
  `0x8020CC2C`/`0x8020CD34`.
- `0x802074A0` (260), `0x802078C8` (552) - the eSCO connection-change callback
  and the security-registration writer. Both build stack event blocks from
  unnamed padding and the latter branches on a `0x802DCD58` half pair whose
  order is not visible; they belong with `0x8020BB44`.
- `0x8020D9EC` (252) - the seven-argument extended eSCO builder called by
  `fn_802072f8`. Its own export needs the same prologue-argument review and was
  left outside this pass; the call in `fn_802072f8` keeps the literal address.
- `0x80204138` (4) and `0x80205088` (4) - four-byte branch thunks. An alias
  body would add nothing; callers already use the real targets.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 26 bodies total
  9332 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 26 were among the
  bucket's 98 untranslated entries).
- `cargo check --message-format=short` reports no errors in
  `src/runtime/bt.rs`; the remaining errors come from `src/runtime/input.rs`,
  which belongs to another Wave 89 lane.
- `node tools/check-lane-csv.mjs wave89` reports `wave89-bt.csv: 26 rows` and
  `OK`.
