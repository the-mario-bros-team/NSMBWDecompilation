# Coverage - Wave 68 (`runtime::bt`, HID-host device table, RFCOMM LCID dispatch, SDP database)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the Ghidra
C export. All addresses are inside this lane's `0x80210000` 64 KiB bucket, which
held 198 functions with 143 untranslated when this pass started (55 already
merged in `ledger/functions.csv`, all of them either in the L2CAP channel code at
`0x80212648..0x80213220` or in the RFCOMM `0x80214AA8..0x8021EBC8` range).

The pass fills three table-centred gaps rather than the RFCOMM/L2CAP state
machines that neighbour them:

- the HID-host device table at `0x803D4838` (the region wave 66 only cleared),
- the RFCOMM port send path and the LCID callback/block pair at
  `0x803D543C` / `0x803D5464`,
- the SDP service-database records at `0x803D6A64`, whose address range ends
  exactly where the wave-60 SDP client code starts (`0x802202D0`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 28 | 4668 | 12 | 16 |

## What this pass covers

- **HID-host device table** (`0x80210100..0x8021254C`, 6 bodies). Sixteen
  `0x34`-byte records live at [`BTM_POOL`](src/runtime/bt.rs); `16 * 0x34 =
  0x340` ends exactly at the owner callback word `0x803D4B78`, which is what
  makes wave 66's "`0x404`-byte pool" that length. The pass adds the release
  path (`fn_80210100`), the originate path (`fn_802101c8`, `fn_8021254c`), the
  disconnect path (`fn_8021036c`, `fn_8021078c`) and the PSM `0x11`/`0x13`
  registration with its 16-pair record clear (`fn_80210634`).
- **RFCOMM port send path** (`0x8021995C..0x80219F8C`, 5 bodies). The send-queue
  drain with its `0x4000`/`0x4004` status words, the frame-size and credit
  selection that derives `8000/5000/12000` credit limits from the baseband
  payload size, the five-record port scan with odd-LCID promotion, the idle
  transition reporter and the modem-status-to-flag-mask converter.
- **RFCOMM LCID dispatch** (`0x8021A1C0..0x8021AAD8`, 8 bodies). The ten-word
  L2CAP callback block at `0x803D543C` and the ten-slot block table at
  `0x803D5464`; six handlers share the inlined `rfc_find_lcid_mcb` lookup, now
  written once as a helper, and drive DLC commands `9`, `0x0B`, `0x0C`, `0x0E`.
  Slot `[7]` of the callback block points at `0x8021A5DC`, the shared epilogue
  of `fn_8021a5e0`.
- **SDP service database** (`0x8021EFA4..0x8021FE24`, 9 bodies). CCB claim with
  and without the defer byte, the linked-list `u16` lookup, the trace-level
  accessor, the recursive attribute matcher, record lookup by handle, the
  attribute-range scan, the append path (`0x10000 + n` handles) and the
  compaction path that also moves every attribute value pointer down one
  record.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80210100` | `fn_80210100` | 200 | high |
| `0x802101C8` | `fn_802101c8` | 96 | medium |
| `0x8021036C` | `fn_8021036c` | 160 | high |
| `0x80210634` | `fn_80210634` | 344 | medium |
| `0x8021078C` | `fn_8021078c` | 164 | medium |
| `0x8021254C` | `fn_8021254c` | 252 | high |
| `0x8021995C` | `fn_8021995c` | 248 | medium |
| `0x80219BE8` | `fn_80219be8` | 196 | high |
| `0x80219E94` | `fn_80219e94` | 136 | high |
| `0x80219F1C` | `fn_80219f1c` | 112 | high |
| `0x80219F8C` | `fn_80219f8c` | 104 | high |
| `0x8021A1C0` | `fn_8021a1c0` | 124 | medium |
| `0x8021A23C` | `fn_8021a23c` | 140 | medium |
| `0x8021A2C8` | `fn_8021a2c8` | 260 | medium |
| `0x8021A3CC` | `fn_8021a3cc` | 264 | medium |
| `0x8021A4D4` | `fn_8021a4d4` | 264 | medium |
| `0x8021A5E0` | `fn_8021a5e0` | 292 | medium |
| `0x8021A9A4` | `fn_8021a9a4` | 308 | high |
| `0x8021AAD8` | `fn_8021aad8` | 28 | high |
| `0x8021EFA4` | `fn_8021efa4` | 92 | medium |
| `0x8021F000` | `fn_8021f000` | 100 | medium |
| `0x8021F064` | `fn_8021f064` | 40 | high |
| `0x8021FA70` | `fn_8021fa70` | 36 | high |
| `0x8021FBD0` | `fn_8021fbd0` | 212 | medium |
| `0x8021FCA8` | `fn_8021fca8` | 92 | high |
| `0x8021FD04` | `fn_8021fd04` | 64 | medium |
| `0x8021FD44` | `fn_8021fd44` | 64 | medium |
| `0x8021FE24` | `fn_8021fe24` | 276 | medium |

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave68-bt.csv`.

- `0x80219A54` (404 bytes) - the RFCOMM port-closed handler. Its export calls
  `FUN_80219F8C(param_1)` with one argument, but that callee's own export takes
  three (`port`, `current`, `new` modem-status bits) and XORs the last two. The
  two status arguments are simply not present in the C, and inventing them would
  change the observable flag mask.
- `0x8021A704` (672 bytes) - the buffer-data indication. It is otherwise
  readable, but on the credit branch it calls `FUN_8021EB00(port)` although that
  callee takes `(port, i16 delta)` and adds the delta to the credit counter. The
  delta is lost, so the credit update cannot be reproduced honestly.
- `0x8021FF38` (920 bytes) - the SDP record-attribute insert. Its bulk move is an
  8-way unrolled word copy driven by a `FUN_802dcd64()` register pair, and the
  length clamp writes two bytes outside the copied range. Translating it would
  have to guess which half of the pair is the attribute id; left for a pass that
  reads the disassembly.

The same reasoning as wave 66 applies to the rest of the bucket: the exports
that open with `FUN_802dcd68()`/`FUN_802dcd70()` and consume the folded 64-bit
result as two real arguments are translated here only where the register pair is
unambiguous (the first two arguments of the routine itself). Where a *later* call
also loses an argument, the body was skipped instead.

## Left for a later pass

The remaining 115 addresses in this bucket are the HID data paths
(`0x8021040C..0x802122B0`, `0x80213240` and its 4.9 KB body), the L2CAP channel
helpers (`0x80214578..0x802159BC`, `0x8021675C..0x8021982C`), and the RFCOMM DLC
machines (`0x8021A704`, `0x8021AAF4`, `0x8021B738`, `0x8021D8B4`, `0x8021DE28`,
`0x8021E9A8`, `0x8021EC2C`). Two SDP attribute-search variants (`0x8021F08C`,
`0x8021F158`) were read but not landed; they duplicate the class-3/class-6 walk
of [`fn_8021fbd0`](src/runtime/bt.rs) with a list walk on top.

## Verification

- Every address and byte size matches `functions.tsv` (re-read for this pass;
  total 4668 bytes).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 28 were absent from the
  bucket's 143 untranslated entries).
- `cargo check --message-format=short` reports no errors in `src/runtime/bt.rs`;
  the only warnings in the file are the ten pre-existing
  `unnecessary parentheses around function argument` warnings, and this pass
  added none.
- `node tools/check-lane-csv.mjs wave68` reports `OK` with
  `wave68-bt.csv: 28 rows`.
