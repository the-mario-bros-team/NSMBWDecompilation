# Coverage - Wave 102 (`runtime::input`, HID-host receive path, security tables, WUD steps)

## How to read this file

1. **Summary** gives the per-file totals and the bucket inventory for
   `[0x801F0000, 0x80200000)`.
2. **What this pass covers** describes the four clusters this pass closed and
   how they connect to the wave 61/74/89/91/96/97 bodies around them.
3. The **function table** lists all 24 bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** lists the two bucket entries this pass did not take.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. All 24 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and repeat the
`// observed:` / `// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 24 | 13,040 | 7 | 17 |

Bucket inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the bucket | 268 |
| Untranslated before this pass | 26 |
| Selected and translated by this pass | **24** |
| Deliberately skipped | 2 |
| Untranslated in the bucket after this pass | 2 |

## What this pass covers

The twenty-six entries left in the bucket were the receive side of the HID
host, the BTA security/ACL bookkeeping under it, the four WUD connection/save
steps and five GKI task stubs. Four shapes repeat:

- **HID-host report path (`0x801F8BC0`, `0x801F8F70`-`0x801F9FA0`, 8 bodies).**
  The `0x803CDDE0` state block is now complete: `0x801F9490` clears it and opens
  the two IOS endpoints, `0x801F96A0` moves it from state `4` (connected) to
  `2` (receiving) and arms both pumps, `0x801F8F70` and `0x801F9120` are the
  `0x1000` and `0x1100` pump bodies, `0x801F9840` is the per-chunk feeder,
  `0x801F8BC0` re-chunks an ACL packet, `0x801F9FA0` drains the frame and timer
  queues, and `0x801F36F0` arms the WUD connection alarm. The two dynamic pool
  ids (`0x803CDDFC`/`0x803CDDFD`) are created by `0x801F9490` through the
  wave-96 pool builder.
- **HID state machine (`0x801FF970`).** The `0x802F7030` per-state transition
  table and the `0x802F6FB8` action table were already named by wave 61; this
  pass finally gives the machine its body, using the existing
  [`bta_hh_state_name`] helper and replacing the old private forwarder.
- **GKI stubs (`0x801F3300`, `0x801F8630`-`0x801F87F0`, 7 bodies).** Five are
  literal `blr` no-ops (`8630` yield, `8640` enter critical, `8650` exit
  critical, `8660` delay, `87B0` pool error), `87F0` is the `App_MEMfree`
  thunk and `3300` is the `0x14A78` buffer constant.
- **BTA security/ACL tables (`0x801FC958`-`0x801FD83C`, 5 bodies).** The
  five-slot discovery table at `0x803D1814`, the nine-byte HID service table at
  `0x803D1919`, the three `0x20`-byte security timers at `0x803D18AC` and the
  `r13-0x5DA0`/`r13-0x5D9C` link/service tables are now all read and written by
  Rust bodies with the same offsets.
- **WUD steps (`0x801F0B10`, `0x801F1F70`, `0x801F2650`).** These are the three
  long bodies wave 74/89/96 deferred: the `0x00`..`0xFF` pairing state machine,
  the device-table migration out of the two link banks, and the WiiFit
  save-data ladder with its two checksum loops.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---:|---|---|---|
| `0x801F0B10` | `fn_801f0b10` | WUD pairing state machine (`0x803A312C`) | 2,788 | translated | medium | `work/ghidra/main-gb/export/801f0b10_FUN_801f0b10.c` |
| `0x801F1F70` | `fn_801f1f70` | WUD device-table migration | 1,592 | translated | medium | `work/ghidra/main-gb/export/801f1f70_FUN_801f1f70.c` |
| `0x801F2650` | `fn_801f2650` | WiiFit save-data ladder + checksums | 1,328 | translated | medium | `work/ghidra/main-gb/export/801f2650_FUN_801f2650.c` |
| `0x801F3300` | `fn_801f3300` | `0x14A78` buffer constant thunk | 4 | translated | high | `work/ghidra/main-gb/export/801f3300_thunk_FUN_801f8ec0.c` |
| `0x801F36F0` | `fn_801f36f0` | WUD connect arming | 332 | translated | medium | `work/ghidra/main-gb/export/801f36f0_FUN_801f36f0.c` |
| `0x801F8630` | `fn_801f8630` | GKI yield (no-op) | 4 | translated | high | `work/ghidra/main-gb/export/801f8630_FUN_801f8630.c` |
| `0x801F8640` | `fn_801f8640` | GKI enter critical (no-op) | 4 | translated | high | `work/ghidra/main-gb/export/801f8640_FUN_801f8640.c` |
| `0x801F8650` | `fn_801f8650` | GKI exit critical (no-op) | 4 | translated | high | `work/ghidra/main-gb/export/801f8650_FUN_801f8650.c` |
| `0x801F8660` | `fn_801f8660` | GKI delay (no-op) | 4 | translated | high | `work/ghidra/main-gb/export/801f8660_FUN_801f8660.c` |
| `0x801F87B0` | `fn_801f87b0` | GKI pool error (no-op) | 4 | translated | high | `work/ghidra/main-gb/export/801f87b0_FUN_801f87b0.c` |
| `0x801F87F0` | `fn_801f87f0` | `App_MEMfree` thunk | 4 | translated | high | `work/ghidra/main-gb/export/801f87f0_thunk_FUN_801eff50.c` |
| `0x801F8BC0` | `fn_801f8bc0` | ACL re-chunk into fragments | 428 | translated | medium | `work/ghidra/main-gb/export/801f8bc0_FUN_801f8bc0.c` |
| `0x801F8F70` | `fn_801f8f70` | `0x1000` receive pump | 424 | translated | medium | `work/ghidra/main-gb/export/801f8f70_FUN_801f8f70.c` |
| `0x801F9120` | `fn_801f9120` | `0x1100` transmit pump | 412 | translated | medium | `work/ghidra/main-gb/export/801f9120_FUN_801f9120.c` |
| `0x801F9490` | `fn_801f9490` | HID-host transport bring-up | 520 | translated | medium | `work/ghidra/main-gb/export/801f9490_FUN_801f9490.c` |
| `0x801F96A0` | `fn_801f96a0` | connected -> receiving, arm pumps | 396 | translated | medium | `work/ghidra/main-gb/export/801f96a0_FUN_801f96a0.c` |
| `0x801F9840` | `fn_801f9840` | per-chunk IOS feeder | 524 | translated | medium | `work/ghidra/main-gb/export/801f9840_FUN_801f9840.c` |
| `0x801F9FA0` | `fn_801f9fa0` | frame + timer event pump | 892 | translated | medium | `work/ghidra/main-gb/export/801f9fa0_FUN_801f9fa0.c` |
| `0x801FC958` | `fn_801fc958` | L2CAP compressed-link callback | 228 | translated | medium | `work/ghidra/main-gb/export/801fc958_FUN_801fc958.c` |
| `0x801FCC74` | `fn_801fcc74` | discovery-slot link-value update | 264 | translated | medium | `work/ghidra/main-gb/export/801fcc74_FUN_801fcc74.c` |
| `0x801FD16C` | `fn_801fd16c` | AFH channel-map builder | 240 | translated | medium | `work/ghidra/main-gb/export/801fd16c_FUN_801fd16c.c` |
| `0x801FD4BC` | `fn_801fd4bc` | security event: service slot update | 896 | translated | medium | `work/ghidra/main-gb/export/801fd4bc_FUN_801fd4bc.c` |
| `0x801FD83C` | `fn_801fd83c` | security event: quality/timer decision | 896 | translated | medium | `work/ghidra/main-gb/export/801fd83c_FUN_801fd83c.c` |
| `0x801FF970` | `fn_801ff970` | BTA HID state machine execute | 852 | translated | medium | `work/ghidra/main-gb/export/801ff970_FUN_801ff970.c` |

Totals: **24 translated**, 0 partial, 0 stubs; 7 high / 17 medium / 0 low;
13,040 bytes. Address range `0x801F0B10`-`0x801FF970`.

## Corrections and caveats

1. **`fn_801ff970` replaces a forwarder.** An earlier pass had a private
   `fn_801ff970` that only called `0x801FF970` by address. It was replaced by
   the real body; the signature is unchanged, so the eight existing call sites
   keep compiling.
2. **IOS sender status.** The translated `fn_801ef6f0`/`fn_801ef770`/
   `fn_801ef810` bodies return `()` because their exports are `void`, but the
   pump exports test the sender's result (`if (iVar4 != 0) free`). The three
   pump bodies therefore call the sender addresses through `call6` so the
   status survives; `fn_801f9840` uses the existing nine-argument `call9` for
   `0x801EFE60`.
3. **`fn_801f9840` argument order.** Its own export prints three parameters
   but reads the real `r3`/`r4` through the `0x802DCD6C` prologue artifact. The
   caller `0x801F8BC0` passes `(channel, source, length)`, which fixes the
   order: channel `0` is the receive path (pool `0x803CDDFC`, receive queue) and
   channel `2` the transmit path (pool `0x803CDDFD`, transmit queue). The same
   convention applies to `0x801FD4BC`, `0x801FD83C` and `0x801FCC74`; where the
   export compares the whole 64-bit `uVar9` against a single table byte, the
   Rust compares its low word (the `r4` handle), which is the only value such a
   byte can equal.
4. **Pool naming in the state block.** `0x803CDDFC` and `0x803CDDFD` are the
   two dynamic pool ids created by `0x801F9490`; the export uses `DDFC` for the
   `0x1000`/receive pump and `DDFD` for the `0x1100`/transmit pump, which is the
   opposite of the intuitive "TX/RX" reading, so the constants are named
   `HID_POOL_A`/`HID_POOL_B` and the doc blocks state the pump each serves.
5. **`fn_801f9fa0` dispatch shape.** The Ghidra body nests a `while` around a
   `goto` label; it is reproduced as an outer `loop` plus an inner timer `loop`
   with `continue` for the event-`10` case. Only `1`, `5`, `8`, `0x3C`,
   `0x16` and `0x42` have direct handlers; `2..=4` and `0x0B..=0x15` use the
   shared helpers and every other value falls through to the two-entry
   `0x803D1650` table.
6. **`fn_801f36f0` second argument.** `0x803A3808` is written from
   `extraout_r4`, the established "lost second argument" artifact, so the Rust
   names it `mode`. The sign test for `0x803A3807` is the shift/XOR sequence of
   the export, i.e. `sign_source < 0`.
7. **`fn_801F0B10` warning path.** Unrecognised states fall through to the
   `"WARNING: Illigal status"` trace, whose printer is a no-op in this build;
   the trace call is therefore omitted and the state byte is kept. The device
   scan uses the `0x60` stride this function shows (`0x803A325D + i * 0x60`),
   which differs from the `0x18` stride of the sibling `0x801F1F70` scan and is
   reproduced as printed in each. The ten-byte request record built in the
   state-`2` step has four initialised bytes in the export; its untouched tail
   is zero-filled here rather than left undefined.
8. **`fn_801F1F70` bank layout.** Bank A is `0x803A39D8` (count at `+0`,
   ten `0x46`-byte records with the address at `+1` and the name at `+7`); bank
   B is `0x803A5080` (count at `+0`, six `0x56`-byte records with the address
   at `+1`, the name at `+7` and the key at `+0x47`). The export walks B
   backwards from `+0x1AF`, which is record 5 of that layout.
9. **`fn_801F2650` checksum.** The stored value is
   `(sum & 0xFFFF) << 16 | folded`, where `sum` is the 16-bit sum of the first
   `0x4AF18` bytes and `folded` the folded one's-complement sum; the second
   loop then extends both over the `0x80`-byte calibration block. The WiiFit
   adoption test keeps Ghidra's polarity: adopt when the local key matches, or
   when the stored 16-byte link key differs.
10. **`fn_801FD83C` scratch frame.** The 13 bytes Ghidra splits into
    `local_48`/`local_47`/`auStack_44`/`local_3c` are modelled as one `[u8; 16]`
    buffer (`0x80205EBC` writes at `+1` or `+0`, `0x80205CEC` at `+4`).
11. **Module header.** The fifteenth-pass bullet and the promotion-note
    sentence were appended; no earlier count was stale (the previous text named
    no total).

## Deliberate skips

- **`0x801F8820` (920 B).** The HID report dispatcher. Its state-block pointer
  and channel index are read only through the `0x802DCD40` prologue stub
  (`iVar9` = low half, `uVar3` = high half), while its sole exported caller
  `0x801F8E80` passes a single length word. The pointer/index pairing cannot be
  proven without guessing, so the body is left for a pass that can resolve the
  HCI control block. `0x801F8E80` remains translated and still calls the
  address.
- **`0x801F9C50` (180 B).** The sixteen-parameter variadic trace forwarder.
  Its body only builds a `va_list` frame (`local_58`..`local_20`,
  `local_18 = 0x02000000`) and calls `0x802E1664`/`0x802E17DC`/`0x8015F730`.
  `platform::call7` cannot express sixteen arguments and the frame layout is
  Ghidra's reconstruction of the saved GPR/FPR area, so any body would be a
  guess. Its seven callers keep calling the address.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 13,040
  bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs` (checked by name for all 24 addresses).
- No address was already in `ledger/functions.csv`; the 26-entry untranslated
  list for the bucket was derived from it first.
- `node tools/check-lane-csv.mjs wave102` prints `OK` and reports
  `wave102-input.csv: 24 rows`.
- `cargo check --message-format=short` reports no error in
  `src/runtime/input.rs`; the only `input.rs` output is the pre-existing
  warnings from earlier passes (two "direct cast of function item into an
  integer" and the unused-constant set). While the parallel `runtime::os` lane
  was mid-write and `runtime::os` did not resolve, the file was also
  type-checked in a scratch copy of the crate with a six-item `os` stub
  (`SDA_BASE`, `call0_ret`, `disable_interrupts`, `restore_interrupts`,
  `read_time_base`, `get_time`): it compiled with no errors and no new
  warnings.

## Ledger

Machine-readable rows: `ledger/agents/wave102-input.csv` (24 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
