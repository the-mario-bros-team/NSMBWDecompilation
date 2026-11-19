# Coverage - Wave 93 (`runtime::os`, `0x801b0000` bucket)

## How to read this file

This page records wave 93 of the `main.dol` reconstruction for the
`runtime::os` lane: a second pass over the `0x801b0000` bucket, which holds 227
inventory entries. When the lane started, 147 of them were already in
`ledger/functions.csv` and 80 were untranslated; 26 of those 80 are
reconstructed here.

- **Status** - `translated` means every guard, store and call printed in the
  Ghidra export has a counterpart in `src/runtime/os.rs`. All 26 rows are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export.
  `medium` means the flow and the offsets are explicit but at least one callee
  role, field meaning or parameter is `// unknown:` / `// inferred:` in the
  body.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes were re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave93-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 26 | 6,504 | 2 | 24 |

Status split: 26 `translated`, 0 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **Cluster A - the reset entries, `0x801B3240..0x801B3790` (6 bodies).**
  `0x801B3240` lowers both protection windows and enters the loader with
  `entry | 0x80000000`; `0x801B32B0` is the two-word saved-value getter;
  `0x801B3410` is the callback sweep that stops every device (two phases,
  `cntlzw` folding, then cancels every ready or waiting thread); and
  `0x801B3590`/`0x801B3670`/`0x801B3790` are the three reset wrappers that go
  through the NAND boot-info record and end in `0x801AD4E0("OSReset.c")`.
- **Cluster B - the SRAM block, `0x801B3CE0..0x801B4820` (6 bodies).** The
  write pass, the checksum commit, the three single-word commands and the
  handler slot. These bodies are what pins `0xA0000000` as the SRAM window and
  `0x8038E8E0` as its `0x40`-byte cached copy.
- **Cluster C - the EXI channel layer, `0x801B8E50..0x801B9E70` (9 bodies).**
  The DMA start, the attach callback swap, the attach probe, the transfer
  start/stop pair, the two completion handlers, the controller initialiser,
  the lock and the unlock.
- **Cluster D - play-time, CRC-32 and SI startup, `0x801B7220..0x801BAEF0`
  (5 bodies).** The play-time clear, the expiry thread, the alarm arm from the
  disc record, the nibble-table CRC-32 and the serial-interface startup.

## What the pass adds beyond the raw bodies

1. **The EXI register map is named.** `0x801B8E50` writes the DMA address at
   `channel * 0x14 + 0xCD006804`, the length at `+0x08` and the control word at
   `+0x0C`; `0x801B8F50` reads the data register at `+0x10`. Those five names
   (`EXI_CSR`/`MAR`/`LENGTH`/`CR`/`DATA`) are declared once at the top of the
   Wave 93 section and used by every body.
2. **The `0x8038F710` control record is now readable.** The nine EXI bodies
   use seven of its fields: `+0x00` attach callback, `+0x04` DMA completion,
   `+0x08` immediate completion, `+0x0C` flags, `+0x10`/`+0x14` pending
   length/buffer, `+0x18` selected device and `+0x24`/`+0x28` the lock queue.
   `0x801B9E70` proves the queue entries are 8 bytes (`id`, `callback`) by the
   `(count - 1) << 3` compaction it asks `0x802DEF74` to perform.
   `docs/COVERAGE_WAVE71_OS.md` had left this block as "64-byte records with an
   unknown register layout"; that gap is closed.
3. **The 0x8038E8E0 SRAM block is pinned.** `0x801B4020` sums eight halfwords
   at `+0x0C..+0x1B` into `+0x00` and the complements into `+0x02`, so the
   first four bytes of the block are a two's-complement checksum pair and the
   payload starts at `+0x0C`. The same body clamps `0x8038E920` and clears the
   `0x8038E91C` status halfword when it still holds `0x5000`/`0xC0`-shaped
   unformatted values.
4. **`0x801B8620` is a genuine CRC-32**, not a checksum walk: the
   `0x802F6A90` table is indexed by `(crc ^ nibble) & 0xF` after every
   `crc >> 4`, which is the standard reflected nibble algorithm, and the
   result is complemented at the end.
5. **`0x801B84C0` settles the play-time unit.** The timeout is
   `(seconds + 20) * (0x800000F8 / 4)` ticks, i.e. milliseconds scaled by the
   bus-clock-derived tick, and the high/low halves are passed as the second
   and fourth arguments of the five-argument `0x801AB540` alarm install.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B3240` | `fn_801b3240` | 108 | medium |
| `0x801B32B0` | `fn_801b32b0` | 20 | medium |
| `0x801B3410` | `fn_801b3410` | 384 | medium |
| `0x801B3590` | `fn_801b3590` | 224 | medium |
| `0x801B3670` | `fn_801b3670` | 284 | medium |
| `0x801B3790` | `fn_801b3790` | 212 | medium |
| `0x801B3CE0` | `fn_801b3ce0` | 312 | medium |
| `0x801B4020` | `fn_801b4020` | 736 | medium |
| `0x801B4310` | `fn_801b4310` | 292 | medium |
| `0x801B4560` | `fn_801b4560` | 284 | medium |
| `0x801B4680` | `fn_801b4680` | 272 | medium |
| `0x801B4820` | `fn_801b4820` | 112 | high |
| `0x801B7220` | `fn_801b7220` | 84 | medium |
| `0x801B8250` | `fn_801b8250` | 160 | medium |
| `0x801B84C0` | `fn_801b84c0` | 344 | medium |
| `0x801B8620` | `fn_801b8620` | 308 | high |
| `0x801B8E50` | `fn_801b8e50` | 248 | medium |
| `0x801B91D0` | `fn_801b91d0` | 132 | medium |
| `0x801B9500` | `fn_801b9500` | 184 | medium |
| `0x801B95C0` | `fn_801b95c0` | 300 | medium |
| `0x801B96F0` | `fn_801b96f0` | 252 | medium |
| `0x801B97F0` | `fn_801b97f0` | 184 | medium |
| `0x801B9AD0` | `fn_801b9ad0` | 208 | medium |
| `0x801B9BA0` | `fn_801b9ba0` | 452 | medium |
| `0x801B9E70` | `fn_801b9e70` | 216 | medium |
| `0x801BAEF0` | `fn_801baef0` | 192 | medium |

## Deliberate skips

The other 54 untranslated addresses in the bucket were left alone. Several are
the ones Waves 71 and 86 already documented as unprovable; their reasons still
hold and are not repeated in full here. The ones read for this pass and
rejected:

1. **`0x801B3010` (108 bytes).** It walks the thread queue at `+0x2F4` and then
   calls `0x801B5D80` with **no** argument in the export, although that callee
   clearly consumes a queue pointer. Choosing one would be a guess, so the body
   was not written. This is the same reason Wave 86 recorded.
2. **`0x801B9260` (384 bytes).** The body depends on `extraout_r4`, a register
   the callee `0x802DCEC4` leaves behind; the comparison
   `(extraout_r4 + 1) - saved < 3` cannot be expressed without knowing which
   register that is.
3. **`0x801B93E0` (280 bytes).** Ghidra drops the argument of its first
   `0x801B9260` call. The channel is in scope and the neighbouring calls all
   pass it, but a reconstruction would rest on that inference alone, so the
   body was left out rather than shipped as a guess. `0x801B9BA0` covers the
   same channel state from the initialisation side.
4. **`0x801B8DA0` (164 bytes).** The export ignores both leading parameters and
   takes `(channel, address)` from `0x802DCD74`, so the four-argument signature
   the callers use cannot be reconciled with the body without assuming which
   register the helper reads.
5. **`0x801B98B0` (536 bytes).** It repeats the byte-lane copy of `0x801B8F50`
   in full plus a context save/switch pair; reconstructing it would duplicate
   an already untranslated helper and add no new structure.
6. **`0x801B3E20` (512 bytes) and `0x801B3870` (648 bytes).** Both are long
   reset/read wrappers that mix the SRAM sequence with five direct stores into
   `0x8038E8Fx`. Their flow is readable, but almost every step is already
   covered by a cluster-B or cluster-A body; writing them would mostly mirror
   the same calls a fourth time. They stay untranslated so the next pass can
   handle them together with `0x801B4020`'s claim protocol.
7. **The large workers** `0x801B6D60` (1,204), `0x801BBBD0` (2,060),
   `0x801BC7A0` (1,352), `0x801BD280` (1,740), `0x801BE860` (1,332) and
   `0x801BEFC0` (3,448) were not attempted; their helper tables are still
   undocumented and Wave 71 already deferred them.

No address in this pass was already present in `ledger/functions.csv`; the 147
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes.
- `node tools/check-lane-csv.mjs wave93` prints `OK` for `wave93-os.csv`
  (26 rows).
