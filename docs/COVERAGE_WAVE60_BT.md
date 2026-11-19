# Coverage - Wave 60 (`runtime::bt`, SDP layer)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. All addresses are inside this lane's `0x80220000` bucket,
which held 216 functions, 166 of them untranslated when this pass started.

The block is the Bluetooth **SDP** (Service Discovery Protocol) layer that runs
over L2CAP PSM 1, not RFCOMM and not gameplay. Layer naming is `// inferred:`
from the trace strings (`SDP_Registration_failed`,
`SDP__Rcvd_L2CAP_disc_cfm__CID__0x`, ...) and from field use; the shipped image
has no symbol table.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 26 | 7464 | 16 | 9 |

One further body is `low` confidence, so the confidence split is
16 high / 9 medium / 1 low.

## What this pass covers

- **Client PDU machinery** (`0x802202D0`-`0x802211B4`, 11 bodies). Two
  UUID-list builders that emit the `0x19` two-byte element, the attribute-list
  encoder (`0x35` list, kinds `0x19`/`0x1A`/`0x1C`), the service-search
  response builder, the deferred-request helper, the PDU dispatcher keyed on
  type `5`/`3`/`7` and the client state byte at `+0x474`, the handle collector
  that clamps to the `0x15` service cap, the fragment reassembler with its
  1000-byte buffer, and the attribute-response parser that inserts records
  through `0x80221440`.
- **Database and lifecycle** (`0x802213A0`-`0x80222544`, 12 bodies). The
  `0x14`-byte service-record allocator with its `BD_ADDR` copy and list
  append, the one-shot init that clears the `0x4634`-byte context, registers
  the `Service_Discovery` security entries, writes the response caps
  (`0xF0`/`0x15`) and installs the nine-slot L2CAP callback table, the five
  L2CAP event handlers (connect indication/confirm, configuration
  indication/confirm, disconnect indication/confirm, data and timeout), the
  originate call, the abort helper and the server request dispatcher.
- **CCB pool** (`0x80223254`-`0x80223358`, 3 bodies). Lookup by local CID over
  four `0x478`-byte blocks at `0x803D587C`, the allocator that clears a block
  and stores its self-pointer at `+0x18`, and the release path.

### Decompiler note

The exports of `0x8022084C`, `0x80220C38`, `0x80220EE0` and `0x802211B4` open
with `uVar = FUN_802DCD6C()` and close with `FUN_802DCDB8(...)`. Those two
addresses are the compiler's non-volatile register save/restore thunks (their
exports spill `r25..r31` and return immediately), not game calls — the same
finding recorded in `src/game/worldmap.rs`. Ghidra folded the two real incoming
arguments into the thunk's fake 64-bit result, so it printed several of these
functions as taking no parameters. The bodies here use ordinary parameters and
do not call the thunks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802202D0` | `fn_802202d0` | 216 | medium |
| `0x802203A8` | `fn_802203a8` | 380 | medium |
| `0x80220760` | `fn_80220760` | 236 | high |
| `0x8022084C` | `fn_8022084c` | 304 | medium |
| `0x8022097C` | `fn_8022097c` | 52 | high |
| `0x802209B0` | `fn_802209b0` | 320 | high |
| `0x80220AF0` | `fn_80220af0` | 328 | high |
| `0x80220C38` | `fn_80220c38` | 680 | medium |
| `0x80220EE0` | `fn_80220ee0` | 720 | medium |
| `0x802211B4` | `fn_802211b4` | 492 | medium |
| `0x802213A0` | `fn_802213a0` | 156 | high |
| `0x802219D8` | `fn_802219d8` | 416 | medium |
| `0x80221B78` | `fn_80221b78` | 196 | medium |
| `0x80221C3C` | `fn_80221c3c` | 400 | high |
| `0x80221DCC` | `fn_80221dcc` | 316 | high |
| `0x80221F08` | `fn_80221f08` | 380 | high |
| `0x80222084` | `fn_80222084` | 248 | low |
| `0x8022217C` | `fn_8022217c` | 212 | high |
| `0x80222250` | `fn_80222250` | 276 | high |
| `0x80222364` | `fn_80222364` | 156 | high |
| `0x80222400` | `fn_80222400` | 188 | high |
| `0x802224BC` | `fn_802224bc` | 136 | high |
| `0x80222544` | `fn_80222544` | 340 | high |
| `0x80223254` | `fn_80223254` | 144 | high |
| `0x802232E4` | `fn_802232e4` | 116 | medium |
| `0x80223358` | `fn_80223358` | 56 | high |

## Deliberate skips

These neighbours were read and left untranslated. None of them has a row in
`ledger/agents/wave60-bt.csv`.

- `0x80220524` (680 bytes) - service-record compaction over the table at
  `0x803D6A64`. The export aliases two pointers (`piVar2`/`piVar13`) across a
  `0xa6`-word stride, updates 16-bit offsets with `piVar14[1] - (...) & 0xffff`,
  and mixes a nested search loop with in-place compaction. Reproducing that
  walk without the disassembly would be a guess.
- `0x80221440` (1424 bytes) - the service-database insert. Not attempted in
  this pass because of its size; it is called by address from
  [`fn_802211b4`](src/runtime/bt.rs) and
  [`fn_80220ee0`](src/runtime/bt.rs) and is the natural first target of the
  next pass over this cluster.
- `0x8022D2F0` (196), `0x8022D3C0` (120), `0x8022D440` (96), `0x8022D4A0` (96),
  `0x8022D500` (96), `0x8022D560` (96), `0x8022DF30` (124) - a reciprocal /
  fixed-point helper family. Every export depends on `unaff_r2` (the
  `-0x4C50` double constant) and `unaff_GQR0` (the paired-single conversion
  register), so the arguments and the scale factor are not represented. Same
  defect class as the `extraout_r4` skips documented in
  `docs/COVERAGE_WAVE58_BT.md`.
- `0x8022C980` (64) and `0x8022DE40` (88) are clean but belong to a different
  cluster (`0x8022C...`); they are left for a later pass so that this lane
  stays a contiguous SDP run.

`0x80222084` is landed with `low` confidence: the export renders the callback
argument as `~((int)~(*pbVar1 - 3 | 3 - *pbVar1) >> 0x1f) & 0xfff1`, a sign
trick that collapses to `0` for every state as printed. The body uses the
state-`3` reading because the sibling handlers `0x80222364` and `0x802224BC`
pass `0xFFF1` on failure and `0` on success; the doc block states this
explicitly.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  total 7464 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn` body in `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 26 were absent; the
  bucket had 50 prior translations, none in `0x802202D0..0x80223358`).
- `cargo check --message-format=short` reports no errors; the only warnings in
  `src/runtime/bt.rs` are the ten pre-existing
  `unnecessary parentheses around function argument` warnings at lines
  4833-5475.
- `node tools/check-lane-csv.mjs wave60` reports `OK` with
  `wave60-bt.csv: 26 rows` (26 unique addresses).
