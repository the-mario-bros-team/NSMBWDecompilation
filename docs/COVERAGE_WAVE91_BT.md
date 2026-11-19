# Coverage - Wave 91 (`runtime::bt`, HID-host L2CAP events and the L2CAP/RFCOMM close paths)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column of
`ledger/agents/wave91-bt.csv` names the Ghidra C export. All addresses are in
the `0x80210000` bucket, which held 198 functions with 87 untranslated when this
pass started (450 `runtime::bt` rows were already in `ledger/functions.csv`).

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 29 | 12420 | 21 | 8 | 0 |

## What this pass covers

- **HID-host security and connection setup** (`0x8021040C`, `0x80210830`,
  `0x80210960`, `0x80210BB8`, `0x80210D04`). `0x8021040C` registers the six
  security-service entries (ids `0x20`, `0x21`, `0x22`, both roles, PSM `0x11`
  and `0x13`). The other four drive the control channel from idle to accepted:
  the security result moves the record to state `2`, the connect indication
  stores both channel ids and the setup byte at `+0x12`, the originate path
  clears the ids and queues PSM `0x11` through `0x802128A0`, and the interrupt
  security result issues PSM `0x13`.
- **HID-host channel callbacks** (`0x80210F18..0x80211F18`). The connection,
  configuration (indication and confirmation), disconnect (indication and
  confirmation), congestion and data handlers all share one sixteen-record scan
  over `0x803D4838` (stride `0x34`) and one owner callback
  (`0x803D4B78`) called as `(index, event, param, 0)`. The data handler decodes
  the HID report kind from the high nibble and either forwards the buffer or
  frees it.
- **HID PSM registration** (`0x80212674`). The ten-word callback table is
  validated (`+0x0C`, `+0x10`, `+0x14`, `+0x20` non-zero, `psm & 0x101 == 1`),
  matched against the PSM table, allocated through `0x80217FE4` when absent and
  copied in.
- **L2CAP connection request/complete** (`0x8021471C`, `0x80214870`). The first
  reuses or allocates a `0x5C`-byte channel block for a `BD_ADDR` and either
  re-sends, rejects (`0x15`) or starts the connection; the second stores the
  handle, moves state `3` to `4`, notifies the HID host through `0x80200310`,
  flushes the pending list and re-arms the channel timer.
- **L2CAP signal builders** (`0x80216D94`, `0x80216F30`, `0x80217928`,
  `0x80217A98`, `0x80217C34`). Four-byte connect-request, eight-byte
  connect-response, echo request (plain and limit-clamped) and four-byte
  disconnect-request signals, all allocated through `0x80216BD4` and submitted
  through `0x80214F04`.
- **L2CAP channel control** (`0x80217CD8`, `0x80218554`, `0x802186B4`). The CCB
  free-list allocator hands out one `0x7C`-byte record, assigns local channel
  id `index + 0x40` and splices it into the owner list; the connect-start path
  merges the inquiry page-scan fields and arms a 60-second timer; the busy test
  walks four `0x5C`-byte blocks.
- **RFCOMM port close and flow control** (`0x8021894C`, `0x80219A54`,
  `0x80219D64`, `0x80219FF4`, `0x8021B554`, `0x8021B648`). Port start
  indication, the two close paths, the double queue drain, the credit/strict
  receive-window accounting and the two multiplexer configuration handlers.

The RFCOMM port block layout used by these six bodies:

| Offset | Field |
|---:|---|
| `+0x02` | DLC state (`1` opening, `2` configured) |
| `+0x0C` | open flag |
| `+0x0D` | LCID byte |
| `+0x0E` | close reason |
| `+0x0F` | line-status bits |
| `+0x18` | send queue |
| `+0x28` | queued byte count |
| `+0x30` | receive queue |
| `+0x38` | queued-frame count (`u16`) |
| `+0x3C`/`+0x3D` | window-full / flow-pause flags |
| `+0x40` | buffered byte count |
| `+0x5F` | modem-signal byte |
| `+0x68` | event state (`u16`) |
| `+0x6A` | MTU (`u16`) |
| `+0x6C` | owning DLC pointer |
| `+0x6E`/`+0x6F` | configuration flags |
| `+0x70` | timer object |
| `+0x71` | flow byte |
| `+0x88` | status mask |
| `+0x8C`/`+0x90`/`+0x94` | event, close and data callbacks |
| `+0x98` | credit counter |
| `+0x9A`/`+0x9C`/`+0x9E`/`+0xA0` | transmit credit and the three credit limits |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8021040C` | `fn_8021040c` | 552 | high |
| `0x80210830` | `fn_80210830` | 304 | high |
| `0x80210960` | `fn_80210960` | 600 | medium |
| `0x80210BB8` | `fn_80210bb8` | 332 | high |
| `0x80210D04` | `fn_80210d04` | 532 | medium |
| `0x80210F18` | `fn_80210f18` | 840 | high |
| `0x80211260` | `fn_80211260` | 680 | high |
| `0x80211508` | `fn_80211508` | 780 | medium |
| `0x80211814` | `fn_80211814` | 708 | medium |
| `0x80211AD8` | `fn_80211ad8` | 584 | high |
| `0x80211D20` | `fn_80211d20` | 504 | high |
| `0x80211F18` | `fn_80211f18` | 920 | high |
| `0x80212674` | `fn_80212674` | 412 | high |
| `0x8021471C` | `fn_8021471c` | 340 | high |
| `0x80214870` | `fn_80214870` | 568 | high |
| `0x80216D94` | `fn_80216d94` | 196 | high |
| `0x80216F30` | `fn_80216f30` | 192 | high |
| `0x80217928` | `fn_80217928` | 368 | high |
| `0x80217A98` | `fn_80217a98` | 412 | medium |
| `0x80217C34` | `fn_80217c34` | 164 | high |
| `0x80217CD8` | `fn_80217cd8` | 268 | medium |
| `0x80218554` | `fn_80218554` | 208 | medium |
| `0x802186B4` | `fn_802186b4` | 232 | high |
| `0x8021894C` | `fn_8021894c` | 156 | high |
| `0x80219A54` | `fn_80219a54` | 404 | medium |
| `0x80219D64` | `fn_80219d64` | 220 | high |
| `0x80219FF4` | `fn_80219ff4` | 460 | high |
| `0x8021B554` | `fn_8021b554` | 244 | high |
| `0x8021B648` | `fn_8021b648` | 240 | high |

## Why the eight `medium` rows are `medium`

- `0x80210960`, `0x80211508` and `0x80211814` open with the
  `0x802DCD68`/`0x802DCD74` register-save thunk. Ghidra prints no parameter
  list; the two halves of the thunk result are the incoming `r3`/`r4`, taken
  here as ordinary arguments. Every store and branch is literal.
- `0x80210D04` has no printed record argument; the decompiler uses
  `extraout_r4`, which is the preserved incoming `r4` and is recovered as the
  `record` argument.
- `0x80217A98` clamps the echo payload with a `< 0x294` test on the two globals
  `0x803D16CC`/`0x803D16CE`, whose owners are unknown; the clamp arithmetic and
  the copy loop are literal.
- `0x80217CD8` restores the compiler's `(ccb - 0x803D4DB8) / 0x7C` index
  computation and the unnamed default field values; every store is literal.
- `0x80218554` reads an unnamed feature byte from `0x802037A4` and uses only
  its bit 5; the rest of the body is literal.
- `0x80219A54` calls `FUN_80219F8C` with no printed arguments. The old and new
  modem-signal bytes are reloaded from `+0x5F` (the export shows the store that
  clears bits 1 and 3 between them); the call's semantics match the three-
  argument form used by `0x80219188`.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave91-bt.csv`.

- `0x8021022C` (320) and `0x802122B0` (668) - the HID send wrapper and the
  L2CAP PDU splitter. The splitter is driven by the `0x802DCD4C` register-save
  pair and the export drops the packet-type/register arguments of the nested
  builder calls, so the six-argument mapping cannot be proven from this export.
- `0x80212AD4` (400) - the connect-accept primitive called by
  [`fn_80210830`]. Its export branches on unnamed `0x802DCD6C` halves and the
  result-buffer layout needs the same per-register review that wave 89 deferred
  for `0x8020BB44`.
- `0x80213120` (256), `0x8021471C`'s neighbour `0x80214F04` (716) and
  `0x802151D0` (408) - the first indexes the stack slot `unaff_r2 - 0x4C70`,
  which the decompiler does not name (the same reason wave 89 skipped
  `0x8020413C`); the second is a large congestion/flush routine whose
  connection-block argument is another dropped register; the third reads
  `0x803D53F8` and four priority bytes without a proven owning block.
- `0x80215368` (228) and `0x80215460` (464) - the segment-complete and
  reassembly paths. Both consume a packet pointer from the low half of a
  register-save thunk and walk variable-length `u16` fields; the field widths
  are clear but the ownership of the reassembly buffer at `0x803D53FC` is not
  proven for these two entry points.
- `0x802159BC` (816), `0x80215CEC` (2672) and `0x802167AC` (300) - the big
  connection/reassembly routines. Their exports dereference offsets that only
  make sense with the per-connection callback table at `+0x30`, and
  `0x80215CEC` is a 2.6 KB dispatcher whose jump targets are not all in this
  bucket.
- `0x80217FE4` (268) is called by [`fn_80212674`] by address: the PSM slot
  allocator was left for the L2CAP lane because its size class and free-list
  head are shared with `0x80217F88`, which wave 82 already translated.
- `0x80218B98`, `0x80218C9C`, `0x80218DDC`, `0x80218F20`, `0x80219070`,
  `0x80219188`, `0x80219300`, `0x802193C4`, `0x8021948C`, `0x802194F4` and
  `0x8021982C` - the remaining PORT callbacks. Most call `FUN_80219E40()` with
  no argument list; wave 58 documented the same defect. The two arguments are
  `(dlc, lcid)` but their order is not visible in these exports.
- `0x8021B738` (2476) and `0x8021C0EC` (232)/`0x8021C64C` (140) - the RFCOMM
  event dispatcher and two callback wrappers. The dispatcher's switch arms
  build frames whose argument registers are dropped; the wrappers call
  `FUN_8021E7FC()` without the port argument.
- `0x8021D7F4` (192), `0x8021E4D0` (344), `0x8021E9A8` (344) and
  `0x8021F158` (264) - the credit-frame builder and three timer/database
  helpers outside the close-path clusters; they are honest next targets but
  were not needed to close the two runs above.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 29 bodies total
  12420 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 29 were among the
  bucket's 87 untranslated entries).
- `cargo check --message-format=short` reports no errors or warnings in
  `src/runtime/bt.rs`; the only remaining errors come from `src/runtime/os.rs`,
  which belongs to another Wave 91 lane.
- `node tools/check-lane-csv.mjs wave91` reports `wave91-bt.csv: 29 rows` and
  `OK`.
