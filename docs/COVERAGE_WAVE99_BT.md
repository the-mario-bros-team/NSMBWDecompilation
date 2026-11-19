# Coverage - Wave 99 (`runtime::bt`, the BTM inquiry and SCO/eSCO tail)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave99-bt.csv` names the exact Ghidra C export. All addresses
are in the `0x80200000` bucket, which holds 256 functions and had 47
untranslated entries when this pass started. The pass claims 27 of them.

The bucket still mixes three unrelated subsystems (see the module header): the
Broadcom BTE Bluetooth host stack, the Hollywood IPC/EXI service and a handful
of render helpers. This pass stays inside the Bluetooth half: it is the
inquiry/remote-name code and the SCO/eSCO link layer that waves 53, 58, 66,
79, 82, 89, 91, 93 and 96 had left between their clusters.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 27 | 8884 | 7 | 20 |

## What this pass covers

### Inquiry request machinery (`0x80204750..0x80205350`, 9 bodies)

`0x80204750` cancels a query, `0x80204858` starts one from a ten-byte request
block, `0x80204A0C` requests a remote name. The three database scans are
`0x80204C2C` (first occupied record), `0x80204CE4` (next record from a pointer)
and `0x80204E20` (wipe by address or all); `0x80204EF0` is the full teardown.
`0x8020526C` builds the HCI event-filter command for one of three selectors and
`0x80205350` is the state machine that drives filter-then-inquiry.

They share the twelve `0x1C`-byte records at [`BTM_INQ_DB`] that wave 79 named
and add the request block `0x803D33AC..0x803D33BF`:

| Offset | Meaning |
|---|---|
| `0x803D33AC` | request mode (`0` general, `1` limited) |
| `0x803D33AD` | id byte forwarded to the HCI builders |
| `0x803D33AE` | result limit, clamped to `0xC` |
| `0x803D33AF` | event-filter selector |
| `0x803D33B0` | six-byte `BD_ADDR` |
| `0x803D33BC` | event-filter command outstanding |
| `0x803D33BD` | cancels deferred behind the filter command |

### SCO/eSCO link layer (`0x80205EBC..0x80207AF0`, 14 bodies)

The five `0x22`-byte packet-type records at [`SCO_POOL_803D2044`] and the three
`0x34`-byte connection slots at [`CONN_SLOT_TABLE`] are driven by:

| Address | Role |
|---|---|
| `0x80205EBC` | SCO state byte for a `BD_ADDR` |
| `0x80205F74` | release the active slot and notify its service callback |
| `0x8020603C` | merge two packet-type range descriptors (`0x802F7040` table) |
| `0x80206258` | merge the two registered service ranges with the slot's own |
| `0x802063B8` | apply a merged range and dispatch one packet-type builder |
| `0x802065EC` | finish the active command, update state/range, notify callbacks |
| `0x8020669C` | mode-change completion (unsniff gate, then `0x040F`/`0x0413`/`0x080B`) |
| `0x80206908` | eSCO connection response (reject and accept) |
| `0x80206AE4` | unpark a slot in state `6` with a matching handle |
| `0x80206C70` | SCO connection request; claim a slot or reject |
| `0x80206E50` | connection-response dispatcher, slot `+0`/`+4` callbacks |
| `0x80206FF0` | close one slot through HCI `0x0406` |
| `0x802074A0` | changed eSCO parameters, completion callback |
| `0x80207AF0` | complete or fail a pending connection setup |

The two 8-byte service slots at [`SCO_SERVICE_TABLE`] (`0x803D20CC`) receive the
state changes: `+0` callback word, `+4` flags byte, bit 0 "packet-type
service", bit 1 "state-change service". Both exports that walk them
(`0x80206258` and the already-translated `0x80205C30`) agree on a stride of `8`
and a count of two.

### Three isolated bodies

- `0x80207BB4` finishes a security authorization step and runs the untranslated
  security executor `0x8020A0B0`.
- `0x8020BB44` is the GAP connection indication over the eight `0x60`-byte
  records at `0x803D43EC` that wave 89 documented end to end.
- `0x8020013C` is a 84-byte lookup from a packet-type code to a payload offset
  through the `r13` table pointer at `0x804298F8`.
- `0x80201754` delivers a completed QoS setup to its callback.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8020013C` | `fn_8020013c` | 84 | high |
| `0x80201754` | `fn_80201754` | 216 | medium |
| `0x80204750` | `fn_80204750` | 264 | high |
| `0x80204858` | `fn_80204858` | 436 | medium |
| `0x80204A0C` | `fn_80204a0c` | 232 | medium |
| `0x80204C2C` | `fn_80204c2c` | 184 | high |
| `0x80204CE4` | `fn_80204ce4` | 316 | medium |
| `0x80204E20` | `fn_80204e20` | 208 | medium |
| `0x80204EF0` | `fn_80204ef0` | 408 | high |
| `0x8020526C` | `fn_8020526c` | 228 | medium |
| `0x80205350` | `fn_80205350` | 544 | medium |
| `0x80205EBC` | `fn_80205ebc` | 184 | high |
| `0x80205F74` | `fn_80205f74` | 116 | medium |
| `0x8020603C` | `fn_8020603c` | 540 | high |
| `0x80206258` | `fn_80206258` | 352 | medium |
| `0x802063B8` | `fn_802063b8` | 564 | medium |
| `0x802065EC` | `fn_802065ec` | 176 | medium |
| `0x8020669C` | `fn_8020669c` | 552 | medium |
| `0x80206908` | `fn_80206908` | 476 | medium |
| `0x80206AE4` | `fn_80206ae4` | 396 | medium |
| `0x80206C70` | `fn_80206c70` | 480 | medium |
| `0x80206E50` | `fn_80206e50` | 416 | medium |
| `0x80206FF0` | `fn_80206ff0` | 184 | high |
| `0x802074A0` | `fn_802074a0` | 260 | medium |
| `0x80207AF0` | `fn_80207af0` | 196 | medium |
| `0x80207BB4` | `fn_80207bb4` | 476 | medium |
| `0x8020BB44` | `fn_8020bb44` | 396 | medium |

## Why the twenty `medium` rows are `medium`

- Fourteen bodies open with a `0x802DCD58..0x802DCD74` register-save thunk. As
  waves 60/68/92/97 established, those are the function's own `stmw`, not a
  call: Ghidra folds the incoming `r3`/`r4` into a fake 64-bit result. Each doc
  block names the half it recovered (`high` = `r3`, `low` = `r4`). Where a
  function only reads one half, the body takes one argument and says so.
- `0x8020526C` and `0x80205350` reproduce two decompiler artefacts verbatim:
  the four-way `switch` that Ghidra printed as a chain of `longlong`
  comparisons, and the `unaff_r2 - 0x4C88`/`-0x4C84` accesses, which are the
  two three-byte inquiry access codes at `0x8042E3F8`/`0x8042E3FC`.
- `0x80204CE4` keeps the compiler's signed index sequence
  (`(ptr - 0x803D3264) / 0x1C`, sign correction, `+ 1`, `& 0xFFFF`), including
  its consequence that record `0` maps to index `1`.
- `0x8020669C` translates `countLeadingZeros(x) >> 5` as `x == 0 ? 1 : 0`,
  which is what the sequence computes.
- The five packet-type builders called from `0x802063B8`
  (`0x8020DC30`, `0x8020DDB0`, `0x8020DCE4`, `0x8020DE34`, `0x8020DEE8`) are
  translated bodies; the four still untranslated callees (`0x8020CFE8`,
  `0x8020D2E4`, `0x8020D4C8`, `0x80212AD4`) are called by address with the
  literal argument order.
- `0x80207BB4` and `0x8020BB44` name their record fields from the wave-89/93
  layouts; the two nested `+0x7F`/`+0` tests in `0x80207BB4` are marked
  `// inferred:` as the queued-procedure handshake.

## Deliberate skips

These are the 20 addresses still untranslated in the bucket. None has a row in
`ledger/agents/wave99-bt.csv`.

- `0x80200080` (188) - the SCO record setup stores `extraout_r4` at `+0x16`;
  that argument never appears in the export's signature and is not recoverable.
- `0x80200190` (224) - the HID device dump indexes the same sixteen records
  with both a `0x10`-element (`u16`) and a `0x20`-byte stride, so the record
  layout cannot be pinned down; the three log calls are the only effect.
- `0x8020413C` (644) - the discoverability setter mixes three `r2` small-data
  buffers through `unaff_r2` and one `extraout_r4`; the final class-update byte
  arithmetic would be guesswork.
- `0x80205570` (632) - the inquiry-result parser; its `0x802DCD48` prologue
  carries four arguments and the result record is not modelled.
- `0x80205A3C` (344) - the remote-name timeout copies from `extraout_r4`.
- `0x80205CEC` (464) - the packet-type request reads two six-byte `r2` tables
  through `unaff_r2` and copies from `extraout_r4`.
- `0x802078C8` (552) - security service registration; twenty-four `0x3C`-byte
  records plus two `0x15`-byte names, with one argument lost to the prologue.
- `0x80207D90` (972), `0x8020815C` (1108), `0x802085C0` (668),
  `0x8020A6C0` (1464) - the four large security-manager bodies. They are the
  next targets of the wave-93 cluster; each needs its own pass.
- `0x802090C4` (252) - reads six `r2` bytes through `unaff_r2` as a default
  `BD_ADDR`; the offsets resolve but the table's contents are unknown.
- `0x8020A0B0` (992) - the security procedure executor, still called by address
  as [`BTM_SEC_EXECUTE_8020A0B0`] (its per-service capability bitmap is not
  proven).
- `0x8020F538` (472), `0x8020F710` (344), `0x8020F97C` (1104), `0x8020FF70`
  (400) - the HID/SCO bring-up tail. `0x8020F710`/`0x8020FF70` are mostly
  literal but depend on `0x8020F538` and on the timer callback `0x8020F868`,
  which are not translated; reconstructing one without the others would leave
  the state block half-documented.
- `0x80204138`, `0x80205088` and `0x802072F4` (4 bytes each) - tail-call
  thunks; a one-instruction bounce is not a translation.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 27 bodies total
  8884 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path (checked
  case-sensitively against the directory listing).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all 27 came from the
  bucket's untranslated list, and no address collides with another Wave 99
  lane.
- `cargo check --message-format=short` exits `0`; `src/runtime/bt.rs` produces
  no error and no new warning. The only `bt.rs` warnings in the run are the ten
  pre-existing `unnecessary parentheses` diagnostics at lines `5140..5782`
  (wave-6 L2CAP code, untouched by this pass).
- `node tools/check-lane-csv.mjs wave99` prints `wave99-bt.csv: 27 rows`,
  `total rows: 244, unique addresses: 244` and `OK`.
