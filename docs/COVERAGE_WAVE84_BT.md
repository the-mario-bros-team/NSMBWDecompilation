# Coverage - Wave 84 (`runtime::bt`, SDP data elements and the IPC request service)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the Ghidra
C export. All addresses are inside this lane's `0x80220000` bucket, which held
216 functions with 111 untranslated when this pass started.

This pass finishes two clusters that earlier waves had opened but not closed:
the SDP data-element codec immediately after the CCB pool (wave 60 stopped at
`0x80223358`) and the IPC request service between the arena bootstrap and the
`0x140`-byte completion helpers (wave 69 translated the builders but not the
machinery they call).

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 25 | 9408 | 13 | 11 | 1 |

## What this pass covers

- **SDP data-element codec** (`0x80223390..0x80223F40`, 8 bodies). Every SDP
  service record is a tree of "data elements", each a one-byte type/size header
  followed by a payload. This run supplies the whole codec:
  - `0x80223390` builds a sequence element holding 16-bit attribute ids, using
    `0x35` (one length byte) or `0x36` (two length bytes) and the wildcard
    element `0x0000FFFF` when the list pointer is null;
  - `0x80223560` encodes one attribute pair: the id as the fixed `0x09` uint16
    element, then the value with the caller's type nibble and the compact size
    index (or the length-byte form for types `4` and `6..8`);
  - `0x802236b4` is `sdpu_build_n_send_error_response` (the trace string names
    it): it allocates a `2`-class buffer, fills PDU id `1` and the transaction
    id, appends the message and sends the PDU on the CCB's channel;
  - `0x802237f0` and `0x80223b40` are the two parsers for the request side: a
    sequence of 2/4/16-byte UUIDs into 16 `0x12`-byte entries, and a sequence
    of `u16`/`u32` attribute ids into 16 two-word entries;
  - `0x80223d90` decodes a size index into a width or an inline value;
  - `0x80223e60` tests the fixed twelve-byte tail of the Bluetooth base UUID,
    and `0x80223f40` is the general 2/4/16-byte UUID comparator that expands a
    short form into the base UUID at offset `2` (16-bit) or `0` (32-bit).
- **IPC reply and interrupt handling** (`0x802243C0`, `0x80224620`). The reply
  handler repairs the bus addresses inside a block (`kind 3` one buffer,
  `kind 6` two buffers, `kind 7` a vector table), invalidates exactly the bytes
  the device wrote, runs the completion callback and retires the block. The
  interrupt handler tests the two published status bit pairs and forwards to
  the reply handler or dequeues one ring entry.
- **IPC submit** (`0x80224910`). The single funnel every builder ends in: cache
  write-back, the 16-entry completion ring with its monotonic counters, the
  32-slot wait table, the doorbell write, and the synchronous wait/free path.
- **Request builders** (`0x80224B50`, `0x80225410..0x802259A0`, 7 bodies):
  the asynchronous kind-1 open, the synchronous kind-5 request, and the
  kind-6/kind-7 two-buffer and vector-table forms with and without callbacks.
- **The `0xBABE0000` heap** (`0x80225D80`, `0x80225EB0`, `0x802260C0`): attach
  one of eight heaps, the first-fit aligned allocator that splits blocks and
  marks alignment pads, and the free path with pad redirect and forward/backward
  coalescing.
- **Tables and completion** (`0x802262B0`, `0x80226370`, `0x802265C0`) and the
  **second-arena setup** (`0x80226490`).

This pass does not add gameplay code; it is platform glue. It stays in this
module for the same reason wave 69 did: the globals are part of the original
image layout and the layer documents what the game expects from the transport.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80223390` | `fn_80223390` | 464 | high |
| `0x80223560` | `fn_80223560` | 340 | high |
| `0x802236B4` | `fn_802236b4` | 316 | medium |
| `0x802237F0` | `fn_802237f0` | 848 | high |
| `0x80223B40` | `fn_80223b40` | 592 | high |
| `0x80223D90` | `fn_80223d90` | 208 | high |
| `0x80223E60` | `fn_80223e60` | 224 | high |
| `0x80223F40` | `fn_80223f40` | 628 | high |
| `0x802243C0` | `fn_802243c0` | 600 | medium |
| `0x80224620` | `fn_80224620` | 428 | medium |
| `0x80224910` | `fn_80224910` | 576 | medium |
| `0x80224B50` | `fn_80224b50` | 280 | medium |
| `0x80225410` | `fn_80225410` | 232 | high |
| `0x80225500` | `fn_80225500` | 312 | medium |
| `0x80225640` | `fn_80225640` | 304 | medium |
| `0x80225770` | `fn_80225770` | 316 | medium |
| `0x802258B0` | `fn_802258b0` | 228 | medium |
| `0x802259A0` | `fn_802259a0` | 220 | medium |
| `0x80225D80` | `fn_80225d80` | 304 | high |
| `0x80225EB0` | `fn_80225eb0` | 508 | medium |
| `0x802260C0` | `fn_802260c0` | 484 | high |
| `0x802262B0` | `fn_802262b0` | 184 | high |
| `0x80226370` | `fn_80226370` | 132 | high |
| `0x80226490` | `fn_80226490` | 296 | low |
| `0x802265C0` | `fn_802265c0` | 384 | high |

## Why the eleven `medium` rows are `medium`

- `0x802236B4` takes its CCB pointer from the low half of the
  `FUN_802dcd68()` register-save prologue and its transaction id from Ghidra's
  `extraout_r4`. Both values are certain; only their position in the original
  signature is recovered from use.
- `0x802243C0` and `0x80224620` are literal, but the meaning of the
  `0xCD000030` write and of the two unnamed guard calls around the completion
  callback is not visible in this lane. The callback slot is read through
  `&DAT_80000020 + bus`, which is `block + 0x20` after the MEM2 bias.
- `0x80224910` reproduces the export's bit-twiddled ring guard exactly. Its
  value is zero while fewer than fifteen of the sixteen slots are used, so the
  ring can never hold sixteen requests; the source expression is kept so the
  behaviour is not "improved" by accident.
- `0x80224B50`, `0x80225500`, `0x80225640`, `0x80225770`, `0x802258B0` and
  `0x802259A0` each start with a `FUN_802DCD64..74` prologue whose 64-bit
  "return" is the function's own first two arguments. The bodies take those as
  ordinary parameters; the remaining stores are literal.
- `0x80225EB0` prints its selected block twice: the loop condition keeps a
  shadow copy (`puVar3 = puVar2`) one iteration behind the body's `best`. The
  body uses `best` (the only reading under which the allocator works) and says
  so. The split path also never writes the new block's `prev` word; that is
  reproduced as printed.

## Why the one `low` row is `low`

- `0x80226490` is literal in every store and call, but the two-argument call to
  `0x802E1938` is unnamed and the following kind-1 request builder is handed
  the raw arena base as its "string". The only coherent reading is that
  `0x802E1938` leaves the device name at that address, and the export does not
  prove it. The body keeps both calls at their documented addresses and marks
  the inference in the doc block.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave84-bt.csv`.

- `0x80220524` (572) - `sdpu_delete_attr`: the database attribute deleter. The
  two prologue halves are identifiable from the log string (attribute id = low
  half, handle = high half), but the routine compacts the record in place: it
  treats word `+4` of each record as the payload byte count and `+0x138` as the
  start of the payload area. Both roles are visible only in this one function,
  so the body was left for the pass that also takes the insert `0x80221440`.
- `0x80221440` (1424), `0x80222698` (1040), `0x80222AA8` (912),
  `0x80222E38` (1052) - the SDP database insert and the three server request
  handlers. They are large and were left for a dedicated pass; the new codec
  bodies document the element layout they consume.
- `0x80225A80` (764) - the synchronous kind-7 submit. It re-implements the ring
  push, the doorbell write and the wait/free sequence of `0x80224910` inline,
  and its final free call prints only the pool argument, so the 764-byte routine
  needs its own review rather than being half-translated here.
- `0x802260B0` (4) - the branch thunk to `0x80225EB0`. A four-byte alias adds
  nothing; callers use the allocator directly.
- `0x80226400`/`0x80226410` are already translated (wave 69).
- `0x80226740..0x80227730` (16 functions) - the callback-carrying wrappers over
  the same `0x140`-byte blocks. They all open with a register-save prologue and
  several read an `extraout_r4` that is never printed as a parameter; they need
  the same per-register treatment `0x80225A80` needs.
- `0x802281D0` (276) and `0x8022852C..0x80228F80` - the texture-header fixup
  and the EXI cluster. `0x80228644` (744 bytes) decompiles to an empty
  `do {} while (true)` with twelve "unreachable block" warnings, so the
  cluster cannot be read from the export; the rest were not attempted without
  it.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 25 bodies total
  9408 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 25 were among the
  bucket's 111 untranslated entries).
- `cargo check --message-format=short` reports no errors and no warnings in
  `src/runtime/bt.rs` for the new code.
- `node tools/check-lane-csv.mjs wave84` reports `wave84-bt.csv: 25 rows` and
  `OK`.
