# Coverage - Wave 58 (`runtime::bt`, RFCOMM PORT cluster)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. All addresses are inside the lane's `0x80210000` bucket, which
held 198 functions, 171 of them untranslated when this pass started.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 28 | 3620 | 21 | 7 |

## What this pass covers

- **DLC state callbacks** (`0x8021C6D8`-`0x8021D3DC`, 18 bodies). The block
  starts with the empty callback `fn_8021c6d8` and the buffer-release callback
  `fn_8021c6dc`, then the two teardown paths that clear/set the RFCOMM-wide
  control flag at `0x803D548C` and report the port state through `0x8021982C`.
  `fn_8021c7ac` is the stop-flag setter that drains the send queue through
  [`fn_8021ebc8`]. `fn_8021c834` copies the optional port-settings bytes out of
  a received parameter block under a nine-bit mask. The middle of the block is
  the frame machinery: two guarded wrappers around the by-address builder
  `0x8021D270` and the translated builder [`fn_8021d630`], four short UIH
  frame builders (`0x73`, `0x0F`/`0x1F`, `0x53`, plus the two 2-byte responses
  `0xA1`/`0x61`), and the shared finisher [`fn_8021d0d8`] that rewrites the
  header/length/type fields, appends the two-byte checksum and routes the
  buffer to either the RFCOMM queue or the L2CAP data-write entry.
- **Port credit and timer routines** (`0x8021E484`-`0x8021EB8C`, 10 bodies).
  The receive-side checksum test (`0xCF` residue over the shared table at
  `0x802F7098`), the retry-timer arm/cancel pair (`0x0B`) and the port-timer
  arm/cancel pair (`0x0C`) at `+0x70`, the 61-entry idle scan at `+0x24` that
  re-arms the retry timer or issues DLC command `8`, the window-state event
  forwarder, and the two credit-counter updates on the `u16` at `+0x98` that
  gate on peer mode `2`.

The blocks are contiguous, share the DLC/port field layout, and were selected
because each export is complete: no decompiler-dropped arguments and no
`extraout_*` register values.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8021C6D8` | `fn_8021c6d8` | 4 | high |
| `0x8021C6DC` | `fn_8021c6dc` | 8 | high |
| `0x8021C6E4` | `fn_8021c6e4` | 100 | high |
| `0x8021C748` | `fn_8021c748` | 100 | high |
| `0x8021C7AC` | `fn_8021c7ac` | 136 | high |
| `0x8021C834` | `fn_8021c834` | 164 | high |
| `0x8021C8D8` | `fn_8021c8d8` | 44 | high |
| `0x8021CABC` | `fn_8021cabc` | 36 | high |
| `0x8021CBA0` | `fn_8021cba0` | 28 | high |
| `0x8021CEB4` | `fn_8021ceb4` | 176 | medium |
| `0x8021CF64` | `fn_8021cf64` | 196 | medium |
| `0x8021D028` | `fn_8021d028` | 176 | medium |
| `0x8021D0D8` | `fn_8021d0d8` | 408 | high |
| `0x8021D358` | `fn_8021d358` | 132 | high |
| `0x8021D3DC` | `fn_8021d3dc` | 132 | high |
| `0x8021D460` | `fn_8021d460` | 288 | medium |
| `0x8021D580` | `fn_8021d580` | 176 | high |
| `0x8021D630` | `fn_8021d630` | 284 | high |
| `0x8021E484` | `fn_8021e484` | 76 | medium |
| `0x8021E6B0` | `fn_8021e6b0` | 116 | high |
| `0x8021E724` | `fn_8021e724` | 88 | high |
| `0x8021E77C` | `fn_8021e77c` | 128 | high |
| `0x8021E7FC` | `fn_8021e7fc` | 88 | high |
| `0x8021E854` | `fn_8021e854` | 196 | high |
| `0x8021E918` | `fn_8021e918` | 56 | high |
| `0x8021E954` | `fn_8021e954` | 84 | high |
| `0x8021EB00` | `fn_8021eb00` | 140 | medium |
| `0x8021EB8C` | `fn_8021eb8c` | 60 | medium |

## Deliberate skips

These neighbours were read but cannot be reconstructed honestly from the
export, so they stay untranslated and are called by address where a landed
body needs them:

- `0x8021C904`, `0x8021C96C`, `0x8021C9DC`, `0x8021CAE4`, `0x8021CBC0`,
  `0x8021CC64`, `0x8021CD08`, `0x8021CDA0`, `0x8021CDCC` - nine DLC
  callbacks whose exports call `FUN_80219E40()` with **no argument list**.
  That routine needs `(channel, index)` and indexes the 61-entry table at
  `+0x24`; Ghidra dropped the register pair at these call sites, so the two
  arguments are unknowable. Guessing them would silently build the wrong
  frame.
- `0x8021D270` - the six-argument frame builder. It consumes `extraout_r4`
  (the second half of the `0x802DCD70` lock result) and stores it into the
  packet at `+0x16`, so omitting or guessing it changes the payload. It is the
  same defect wave 53 documented for `0x8020CD34`/`0x8020CDF4`.
- `0x8021E4D0` - the 344-byte `rfc_timer_start/stop` wrapper. Its export is
  marked "Heritage AFTER dead removal" and contains `extraout_r4` written to
  `0x803D5831`.
- `0x8021E628` - the port teardown. The export pops a node with
  `FUN_801F78F0` but then calls `FUN_801F7240()` with **no argument**, so the
  freed pointer is not represented.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  total 3620 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn` body in `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 28 were absent).
- `cargo check --message-format=short` is free of errors; the remaining
  warnings are pre-existing ones in `src/runtime/bt.rs` (lines 4833-5475) and
  in other lanes' files.
- `node tools/check-lane-csv.mjs wave58` reports `OK` for `wave58-bt.csv`
  (28 rows, 28 unique addresses, 21 high / 7 medium).
