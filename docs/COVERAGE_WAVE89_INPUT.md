# Coverage — Wave 89 (`runtime::input`, WUD patch staging, device database, BTM/HID glue)

## How to read this file

1. **Summary** gives the per-file totals for this pass.
2. **What this pass covers** describes the five clusters of code that were
   added and how they connect to the wave-74 bodies they sit between.
3. The **function table** lists all 27 bodies in address order with their role,
   the byte count re-read from `work/ghidra/main-gb/export/functions.tsv` and
   the confidence carried by the doc block.
4. **Corrections and caveats** records every place where the export needed an
   inference, and **Deliberate skips** says what was left out and why.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for; all 27 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and repeat the `// observed:` /
`// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 27 | 8,260 | 13 | 13 | 1 |

Band inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the band | 268 |
| Untranslated before this pass | 105 |
| Selected and translated by this pass | **27** |
| Untranslated in the band after this pass | 78 |

## What this pass covers

The pass fills the two layers that wave 74 stepped over: the BCM2045 firmware
patch chain that feeds the `0x801F4350` bring-up, and the WUD device database
that the bring-up re-populates. Five shapes of code repeat:

- **Firmware patch staging (`0x801F3CE0`–`0x801F4250`).** Five entry points
  share one state: the destination address at `r13-0x4B28`, the length at
  `r13-0x4B20`, the number of bytes already sent at `r13-0x4B24`, the record
  count at `r13-0x4B2A` and the install cursor at `r13-0x4B2B`.
  `fn_801f4250` fills the address/length pair from the two words at
  `0x803460A0`/`0x803460A4` and picks the remove path (`0xFC4F`, one staged
  byte) or the SuperPeekPoke path (`0xFC0A`, nine bytes at `0x80346094`).
  `fn_801f4030` sends the first `0xFB`-byte download slice, `fn_801f3dd0`
  continues with the rest and, once `length == position`, resets the cursor and
  starts installing, and `fn_801f3ce0` installs `0x0D`-byte records in batches
  of at most `0x13`, naming itself as the continuation each time.
- **WUD bring-up and database builders (`0x801F4350`–`0x801F47A0`).**
  `fn_801f4350` publishes the local name and device class from the `r2`
  small-data area, installs the status, device-event and power-management
  callbacks, applies three BTM configuration values, re-adds every device
  record whose `+0x59` state byte is `1`, then sets the link state to `3`.
  `fn_801f44c0` and `fn_801f4620` are the add/remove pair: each resolves the
  record through `fn_801f47a0`, drives `BTA_DmAddDevice`/`BTA_DmRemoveDevice`,
  adds or removes the HID-host handle for a Wii Remote (or a Balance Board when
  `r13-0x4B38` is set) and moves one of the two counters at `0x803A3132` /
  `0x803A3133` according to the record's `+0x5B` class byte.
- **The two in-use link lists (`0x801F4880`–`0x801F5000`).** Six routines
  over one data structure: a 12-byte slot descriptor per entry (node pointer at
  `+0`, neighbour links at `+4` and `+8`), six slots in the first bank
  (`0x803A313C`, head/tail `0x803A3134`/`0x803A3138`) and ten in the second
  (`0x803A318C`, head/tail `0x803A3184`/`0x803A3188`). Two of them move a
  device to the head or the tail, four splice a node in front of a chosen node
  and differ only in the selection rule and the bank.
- **BTM link-key and power events (`0x801F5CD0`, `0x801F6030`).**
  `fn_801f5cd0` is the five-way event handler for the stored-link-key
  operations: event `1` walks `(BD_ADDR, 16-byte key)` pairs from the event
  record, refreshes known records, allocates a free one from the ten- or
  six-entry table otherwise, and copies the last address to `0x803A381C`;
  events `2`..`4` only trace and clear the outstanding-request byte.
  `fn_801f6030` maps a power-management event onto record state `8` (event `0`)
  or `9` (event `2`), falling back to the default record at `0x803A3978`.
- **DM disable timer and HID-host security/teardown (`0x801FAC44`–`0x801FE0D0`,
  `0x801FC880`/`0x801FC8EC`).** `fn_801fac44` starts the DM disable sequence
  and arms the `0x803D1888` timer with 1 s or 5 s; `fn_801facf4` is the timer
  callback: it completes the disable, or stores the link key of every record in
  the `0x803D1814` search area. `fn_801fdc34`/`fn_801fdd2c` release one of the
  three 0x20-byte security entries at `0x803D18AC`; `fn_801fde58`,
  `fn_801fdfcc` and `fn_801fe0d0` reset and tear the HID host down through the
  sixteen 0x20-byte control blocks at `0x803D1958`. The two
  `0x801FC8xx` bodies forward one inquiry result to the BTM search callback.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---:|---|---|---|
| `0x801F3CE0` | `fn_801f3ce0` | installs one batch of `0x0D`-byte patch records | 240 | translated | high | `work/ghidra/main-gb/export/801f3ce0_FUN_801f3ce0.c` |
| `0x801F3DD0` | `fn_801f3dd0` | patch download continuation, then install start | 596 | translated | high | `work/ghidra/main-gb/export/801f3dd0_FUN_801f3dd0.c` |
| `0x801F4030` | `fn_801f4030` | first `0xFB`-byte patch slice | 436 | translated | high | `work/ghidra/main-gb/export/801f4030_FUN_801f4030.c` |
| `0x801F41F0` | `fn_801f41f0` | closes the SuperPeekPoke exchange | 92 | translated | high | `work/ghidra/main-gb/export/801f41f0_FUN_801f41f0.c` |
| `0x801F4250` | `fn_801f4250` | loads the patch header, picks remove or peek/poke | 252 | translated | high | `work/ghidra/main-gb/export/801f4250_FUN_801f4250.c` |
| `0x801F4350` | `fn_801f4350` | `wudInitSub`: callbacks, config, re-add devices | 368 | translated | medium | `work/ghidra/main-gb/export/801f4350_FUN_801f4350.c` |
| `0x801F44C0` | `fn_801f44c0` | adds one device to both databases | 340 | translated | medium | `work/ghidra/main-gb/export/801f44c0_FUN_801f44c0.c` |
| `0x801F4620` | `fn_801f4620` | removes one device and clears its record | 384 | translated | high | `work/ghidra/main-gb/export/801f4620_FUN_801f4620.c` |
| `0x801F47A0` | `fn_801f47a0` | record lookup by `BD_ADDR` | 216 | translated | high | `work/ghidra/main-gb/export/801f47a0_FUN_801f47a0.c` |
| `0x801F4880` | `fn_801f4880` | moves a linked device to the six-slot head | 276 | translated | medium | `work/ghidra/main-gb/export/801f4880_FUN_801f4880.c` |
| `0x801F49A0` | `fn_801f49a0` | moves a linked device to the six-slot tail | 276 | translated | medium | `work/ghidra/main-gb/export/801f49a0_FUN_801f49a0.c` |
| `0x801F4AC0` | `fn_801f4ac0` | splices ahead of the first idle six-slot node | 360 | translated | medium | `work/ghidra/main-gb/export/801f4ac0_FUN_801f4ac0.c` |
| `0x801F4C30` | `fn_801f4c30` | moves a linked device to the ten-slot head | 276 | translated | medium | `work/ghidra/main-gb/export/801f4c30_FUN_801f4c30.c` |
| `0x801F4D50` | `fn_801f4d50` | moves a linked device to the ten-slot tail | 276 | translated | medium | `work/ghidra/main-gb/export/801f4d50_FUN_801f4d50.c` |
| `0x801F4E70` | `fn_801f4e70` | splices ahead of the first idle ten-slot node | 400 | translated | medium | `work/ghidra/main-gb/export/801f4e70_FUN_801f4e70.c` |
| `0x801F5000` | `fn_801f5000` | splices relative to the ten-slot tail | 408 | translated | medium | `work/ghidra/main-gb/export/801f5000_FUN_801f5000.c` |
| `0x801F5CD0` | `fn_801f5cd0` | stored-link-key event handler, events `1`..`4` | 852 | translated | high | `work/ghidra/main-gb/export/801f5cd0_FUN_801f5cd0.c` |
| `0x801F6030` | `fn_801f6030` | power-management event to record state | 300 | translated | high | `work/ghidra/main-gb/export/801f6030_FUN_801f6030.c` |
| `0x801FAC44` | `fn_801fac44` | starts DM disable, arms 1 s/5 s timer | 176 | translated | medium | `work/ghidra/main-gb/export/801fac44_FUN_801fac44.c` |
| `0x801FACF4` | `fn_801facf4` | DM disable timer callback | 188 | translated | medium | `work/ghidra/main-gb/export/801facf4_FUN_801facf4.c` |
| `0x801FC880` | `fn_801fc880` | inquiry result -> search callback, type `1` | 108 | translated | high | `work/ghidra/main-gb/export/801fc880_FUN_801fc880.c` |
| `0x801FC8EC` | `fn_801fc8ec` | inquiry result -> search callback, type `2` | 108 | translated | medium | `work/ghidra/main-gb/export/801fc8ec_FUN_801fc8ec.c` |
| `0x801FDC34` | `fn_801fdc34` | releases one security entry, posts `0x10A` | 248 | translated | high | `work/ghidra/main-gb/export/801fdc34_FUN_801fdc34.c` |
| `0x801FDD2C` | `fn_801fdd2c` | drops the security entry of a disconnecting device | 288 | translated | low | `work/ghidra/main-gb/export/801fdd2c_FUN_801fdd2c.c` |
| `0x801FDE58` | `fn_801fde58` | HID host reset and block/map rebuild | 372 | translated | medium | `work/ghidra/main-gb/export/801fde58_FUN_801fde58.c` |
| `0x801FDFCC` | `fn_801fdfcc` | HID host teardown, or deferred per block | 260 | translated | high | `work/ghidra/main-gb/export/801fdfcc_FUN_801fdfcc.c` |
| `0x801FE0D0` | `fn_801fe0d0` | unconditional HID host teardown | 164 | translated | high | `work/ghidra/main-gb/export/801fe0d0_FUN_801fe0d0.c` |

Totals: **27 translated**, 0 partial, 0 stubs; 13 high / 13 medium / 1 low;
8,260 bytes. Address range `0x801F3CE0`–`0x801FE0D0`.

## Corrections and caveats

1. **`fn_801f47a0` argument.** Ghidra prints the signature as `void` and
   recovers the address from `0x802DCD70` inside the body, but all four call
   sites (`0x801F44C0`, `0x801F4620`, `0x801F5CD0`, `0x801F6030`) pass their own
   pointer, and `fn_801f5cd0` must look up each entry of a returned array. The
   body therefore takes `bd_addr` and the doc says why.
2. **`fn_801f4350` small-data base.** The three boot fields are loaded through
   a base register Ghidra reports as `unaff_r2`; the body uses `SDA2_BASE` and
   keeps the original relative layout (halfword, byte, word) because the two
   callees take different slots of it. The word field is handed to
   [fn_801fcf68] as a NUL-terminated name, so it is read as an inline string
   rather than as a number. `medium`.
3. **`fn_801f44c0` parameter block.** The `BTA_HhAddDev` block starts with a
   halfword `0x00D9` whose low half is never written in the export
   (`CONCAT22(0xd9, local_20._2_2_)`). The Rust writes only that halfword and
   the `0x80345FB8` pointer at `+4`; the unknown bytes stay zero. `medium`.
4. **`fn_801f3ce0` / `fn_801f3dd0` / `fn_801f4030` copies.** The export contains
   the compiler's eight-way unrolled `memcpy` inlining at three sizes (`0x0D`,
   `0x10` and `0xFB`). Each Rust body uses a byte loop covering exactly the same
   source and destination ranges, in the same order.
5. **Little-endian patch address.** `fn_801f3dd0` and `fn_801f4030` break the
   patch address word into low byte first, while every other word in this layer
   is big-endian. The stores are literal and marked `// observed:`.
6. **Link roles are inferred.** The six list routines read and write three
   words per slot, two of which are always used as neighbour nodes (`+4` and
   `+8`). Which one is "next" is never stated, so the bodies keep the literal
   offsets and the doc marks the role as inferred. All six are `medium`.
7. **`fn_801fdd2c` is `low`.** It reads the device address at record `+8`,
   while [fn_801f4880] reads the same getter (`0x802DCD74`) at `+0x40`. One of
   the two must be a decompiler artifact; both bodies keep the offsets exactly
   as the export shows them, and this one is marked `low` because its record
   identity cannot be proven from the export alone.
8. **`fn_801fc8ec` payload offset.** The twin of [fn_801fc880] stores the
   record's `+2` byte in a separate stack slot (`-0x10` instead of `-0x11`), so
   the payload is modelled as nine bytes. `medium`.
9. **`fn_801fac44` label target.** The idle branch stores `0x801FC864`, which
   Ghidra names only as a label inside `0x801FC5E4`. It is written as an address
   constant with a comment; no body is claimed for it.
10. **No staging.** All 27 bodies live in `src/runtime/input.rs`; no other Rust
    file, generated doc or `ledger/functions.csv` was touched.

## Deliberate skips

- **`0x801F7DF0` (356 B) and `0x801F8040` (332 B).** Both call `0x802DCD70`,
  but one treats the result as a small index and the other as a 64-bit split
  return (`extraout` pattern). That is the same live-register family the
  wave-74 doc already skips; the argument list cannot be recovered.
- **`0x801F8820` (920 B), `0x801F8BC0` (428 B), `0x801F9490` (520 B),
  `0x801F96A0` (396 B), `0x801F9840` (524 B), `0x801F9FA0` (892 B).** The
  HID-host receive path; every one reads a live register pair out of the
  `0x802DCD6C`/`0x802DCD70`/`0x802DCD74` family, exactly the split-frame
  artifacts the wave-50/61/74 docs skip.
- **`0x801F9C50` (180 B).** Sixteen eight-byte parameters plus an `in_cr1`
  branch and a variadic `0x8015F730` tail: the arity is not recoverable.
- **`0x801F8F70` (424 B), `0x801F9120` (412 B), `0x801F9D10`–`0x801F9DD0`
  (7 × 20 B), `0x801F9830` (8 B), `0x801F8E80` (60 B), `0x801FF808` (168 B),
  `0x801FF970` (852 B), `0x801FFEC4` (316 B), `0x801F6A00` (56 B).** Carried
  over from the wave-61/74 skip lists; the reasoning there still holds
  (variadic forwarding with no recoverable arity, single-`return` artifacts and
  an unprovable global width).
- **The BTA HID/SDP state handlers between `0x801FB0C4` and `0x801FD16C`,
  including the `0x801FD4BC`/`0x801FD83C` pair.** Larger, split-frame-heavy and
  mostly call into that same untranslated pair; they remain for a later pass.
- **The 4-byte thunks (`0x801F3300`, `0x801F8630`–`0x801F87F0`).** Pure tail
  jumps to already-translated bodies with no independent control flow; left to
  the ledger merge rather than padded into this lane.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 8,260 bytes).
  `0x801F4620` is 384 bytes in the inventory, not the 368 the first draft of
  the doc block carried.
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- No address was already in `ledger/functions.csv`; the 105-entry untranslated
  list for the band was derived from it first.
- `cargo check --message-format=short` reports no errors in
  `src/runtime/input.rs`; the only warnings in this file are the two
  pre-existing "direct cast of function item into an integer" sites (lines
  5286 and 10482), which this pass did not touch.
- `node tools/check-lane-csv.mjs wave89` prints `OK` and reports
  `wave89-input.csv: 27 rows`.

## Ledger

Machine-readable rows: `ledger/agents/wave89-input.csv` (27 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
