# Coverage - Wave 98 (`runtime::os`, `0x801B0000` bucket)

## How to read this file

This page records wave 98 of the `main.dol` reconstruction for the
`runtime::os` lane: the serial-interface (SI) layer, the play-time state
machine and the two neighbours those paths use. The `0x801B0000` bucket holds
227 inventory entries; 54 were untranslated when this pass started and 25 of
them are reconstructed here.

- **Status** - every row is `translated`: each guard, store and call printed in
  the Ghidra export has a counterpart in `src/runtime/os.rs`.
- **Confidence** - strength of the evidence, not of the Rust. `medium` means
  the flow, offsets and calls are explicit but at least one callee role, field
  meaning or register half is `// unknown:`/`// inferred:` in the body.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; byte sizes were
  re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave98-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 25 | 11,676 | 1 | 24 |

Status split: 25 `translated`, 0 `partial`, 0 `stub`.

## What this pass covers

- **Cluster A - the SI transfer core, `0x801B8B20`..`0x801B9F80` (7 bodies).**
  `0x801B8B20` queues an up-to-four-byte command into the EXI data register and
  stores the pending length/buffer; `0x801B8DA0` splits a byte count into
  four-byte chunks; `0x801B8F50` waits for the start bit, unpacks the reply and
  applies the short-answer check; `0x801B9260` polls the `0x800`/`0x1000`
  status bits with a three-unit elapsed gate; `0x801B93E0` starts a queued
  transfer; `0x801B98B0` maps `(high - 10) / 3` back to an EXI channel and
  switches context around the installed callback; `0x801B9D70` selects a
  device or appends to the `+0x28` lock queue; `0x801B9F80` performs the full
  select/probe/two-round read and publishes the answer.
- **Cluster B - the SI command helpers, `0x801BA330`..`0x801BBA30` (8 bodies).**
  `0x801BA330` (`0x20011300` command), `0x801BA670` (byte/half/word-swapped
  value), `0x801BA4B0` (accepted-answer set), `0x801BA800` (receive from
  `0xCD006480`), `0x801BAFB0` (start, status word `0x80000001`), `0x801BB250`
  (descriptor store plus alarm deadline), `0x801BB3C0` (completion states and
  the four callbacks), `0x801BB680` (50/75-tick timeout), `0x801BBA30`
  (software service of the two pending masks) and `0x801BAB00` (the interrupt
  handler that ties `0x801BA800`, the descriptor search and the four channel
  stamps together).
- **Cluster C - the play-time and boot neighbours, `0x801B3010`..`0x801B8760`
  (7 bodies).** The thread-list detach at `0x801B3010`; the state machine
  `0x801B7280`; the `/dev/net/kd` open `0x801B7750`; the NANDBOOTINFO
  checksum/read `0x801B7CE0`; the scanline expander `0x801B7E60`; the
  play-time record decoder `0x801B82F0`; and the disc-worker boot
  `0x801B8760`.

## What the pass adds beyond the raw bodies

1. **The SI descriptor is fixed at `0x8038F7D0`, stride `0x20`.** `+0x00` is
   the `0x802DCD5C` pair, `+0x08` the send length, `+0x0C` the receive buffer,
   `+0x10` the receive length, `+0x14` the callback and `+0x18` the 64-bit
   deadline. `0xFFFFFFFF` marks a free descriptor. `0x801BB250` writes every
   field and `0x801BA800` reads it back through `0x8033FF98`/`0x8033FF9C`.
2. **The SI status word is `channel << 1 | receive << 8 | send << 16 |
   callback << 30 | 0x80000001`.** `0x801BAFB0` builds it; `0x801BA800`
   decodes the per-channel byte from `0xCD006438` and `0x801BAB00` clears that
   byte with `0x0F000000 >> (channel * 8)`.
3. **The four globals are `0x8033FF90`..`0x8033FFA8`.** `-1` in `0x8033FF90`
   means idle; `0x8033FFA8` is one word per channel and `0x80` is the "reply
   pending" value. The stamp arrays are `0x8038F850` and `0x8038F870` (four
   64-bit entries each, eight bytes apart).
4. **The SI timeouts share one unit.** `0x801BB250`, `0x801BB680` and
   `0x801BAB00` compute `(bus / 100000) * 64 + bus / 1000000`; the first two
   then multiply the 50/75 units by `bus / 4000`. Both divisions are literal in
   the exports (`0x431BDE83 >> 41` and `/ 0x1E848 >> 3`).
5. **The play-time record is `0xD8` bytes with eight `{kind, value}` pairs at
   `+0x98`.** `0x801B82F0` reports kind `1` immediately, otherwise the last
   non-zero kind as `4`/`9`; `0x801B8760` consumes exactly that shape.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B3010` | `fn_801b3010` | 108 | medium |
| `0x801B7280` | `fn_801b7280` | 492 | medium |
| `0x801B7750` | `fn_801b7750` | 184 | medium |
| `0x801B7CE0` | `fn_801b7ce0` | 344 | medium |
| `0x801B7E60` | `fn_801b7e60` | 444 | medium |
| `0x801B82F0` | `fn_801b82f0` | 460 | medium |
| `0x801B8760` | `fn_801b8760` | 516 | medium |
| `0x801B8B20` | `fn_801b8b20` | 636 | medium |
| `0x801B8DA0` | `fn_801b8da0` | 164 | high |
| `0x801B8F50` | `fn_801b8f50` | 636 | medium |
| `0x801B9260` | `fn_801b9260` | 384 | medium |
| `0x801B93E0` | `fn_801b93e0` | 280 | medium |
| `0x801B98B0` | `fn_801b98b0` | 536 | medium |
| `0x801B9D70` | `fn_801b9d70` | 252 | medium |
| `0x801B9F80` | `fn_801b9f80` | 936 | medium |
| `0x801BA330` | `fn_801ba330` | 376 | medium |
| `0x801BA4B0` | `fn_801ba4b0` | 444 | medium |
| `0x801BA670` | `fn_801ba670` | 392 | medium |
| `0x801BA800` | `fn_801ba800` | 756 | medium |
| `0x801BAFB0` | `fn_801bafb0` | 432 | medium |
| `0x801BB250` | `fn_801bb250` | 364 | medium |
| `0x801BB3C0` | `fn_801bb3c0` | 692 | medium |
| `0x801BB680` | `fn_801bb680` | 436 | medium |
| `0x801BBA30` | `fn_801bba30` | 408 | medium |
| `0x801BAB00` | `fn_801bab00` | 1004 | medium |

## Deliberate skips

The other 29 addresses in the bucket were left alone. Nine were read and
rejected:

1. **`0x801B0170` (784).** The glyph writer's pixel address is built from the
   compiler's negative-shift idiom (`uVar8 * -8`, `(3 - iVar9) * 8`); the
   individual scanline terms cannot be separated honestly from the C.
2. **`0x801B06A0` (436) and `0x801B0860` (648).** The text-convert dispatcher
   merges three SDA contexts and calls `0x801B6540`/`0x801B65C0` without the
   value they test; both belong to the text lane.
3. **`0x801B3140` and `0x801B3230` (4 bytes each).** Ghidra prints the
   destination body for these thunks, so the four bytes of evidence cannot
   prove the forwarded argument list.
4. **`0x801B3870` (648).** The reset path builds three stack records whose
   fields are consumed by `0x801B7470`/`0x801B4680`; their offsets are not
   pinned by the export.
5. **`0x801B3E20` (512).** The SRAM probe's second mask block assigns
   `puVar8 = NULL` and then reads `puVar8[0x14]`, so the block pointer for the
   ISR-claimed branch is ambiguous.
6. **`0x801B6080` (808).** The tick-to-calendar converter chains four
   `extraout_r4` remainders and the two month tables at `0x80333D60`/
   `0x80333D90`; it needs its own field-level pass.
7. **`0x801B6D60` (1204).** The play-time record state machine is a
   twelve-case switch over `sda(-0x6140)` with a 64-bit tick window; only its
   caller was reconstructed here.
8. **`0x801B7810` (148).** The elapsed value is the unnamed second return
   register of `0x802DCEC4` (`extraout_r4`), not a value the export defines.
9. **`0x801BBBD0` (2060).** The 2 KB SI variant; not opened this pass.

The remaining 19 (`0x801BC480`, `0x801BC5A0`, `0x801BC7A0`, `0x801BCD50`,
`0x801BD000`, `0x801BD0E0`, `0x801BD280`, `0x801BD950`, `0x801BDCB0`,
`0x801BDDD0`, `0x801BDE50`, `0x801BDF80`, `0x801BE0B0`, `0x801BE480`,
`0x801BE510`, `0x801BE860`, `0x801BEFC0`, `0x801BFD40`) were not opened, so
no claim is made about them.

No address in this pass was already present in `ledger/functions.csv`; the 173
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (25 of 25; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes.
- `node tools/check-lane-csv.mjs wave98` prints `OK` with `wave98-os.csv`
  counted at 25 rows.
