# Coverage - Wave 103 (`runtime::input`, IOS USB transfer layer, KPAD init and aim path)

## How to read this file

1. **Summary** gives the per-file totals and the inventory of the
   `[0x801E0000, 0x801F0000)` bucket.
2. **What this pass covers** describes the two clusters this pass closed and how
   they connect to the wave 61/74/89/91/96/97/102 bodies around them.
3. The **function table** lists all 14 bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** lists the ten bucket entries this pass did not take.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. All 14 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and repeat the
`// observed:` / `// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 14 | 10,564 | 2 | 12 |

Bucket inventory (`[0x801E0000, 0x801F0000)`):

| Item | Count |
|---|---:|
| Functions in the bucket | 103 |
| Untranslated before this pass | 24 |
| Selected and translated by this pass | **14** |
| Deliberately skipped | 10 |
| Untranslated in the bucket after this pass | 10 |

## What this pass covers

The twenty-four entries left in the bucket split into two unrelated layers: the
IOS USB client at the bottom of the band and the KPAD per-channel status/aim
path in the middle. Two shapes repeat:

- **IOS USB transfer layer (`0x801EEB00`-`0x801EFE60`, 9 bodies).** One heap
  (`r13-0x5DE0`, created by [`fn_801eeb00`] from a `0x4000`-byte arena slice),
  one completion callback ([`fn_801eec10`], which understands the `0x00`,
  `0x02`..`0x04` and `0x07` completion kinds and frees the buffer array the
  senders park at `+0x14`), and one transfer-record shape: a `0x80`-byte block
  whose `+0` is a callback, `+8` a callback argument, `+0x14`.. the buffers and
  `+0x34` the kind. On top of it sit `OpenDevice` ([`fn_801eedb0`],
  `"/dev/usb/%s/%x/%x"` through `0x802E16E8`), `CloseDeviceAsync`
  ([`fn_801eef00`]), the long-bulk sender ([`fn_801ef030`], IOS request `10`),
  the interrupt sender ([`fn_801ef390`], caller request code `1`/`2`), the
  control-transfer builder ([`fn_801ef8b0`], six setup fields
  `1,1,2,2,2,1` plus the data buffer) and its async entry point
  ([`fn_801efe60`]). [`fn_801eec00`] is the `0`-returning probe stub.
- **KPAD init and aim path (`0x801EB5C0`-`0x801ED3B0`, 5 bodies).** The four
  `0x578`-byte records at `0x803A1AC8` are created by [`fn_801edaf0`] (clear,
  callbacks, limits, status banks `+0x4BC`/`+0x4C0`, clock scale, shared
  `0x803A30A8` cos/sin basis). The sensor-bar path then runs per sample:
  [`fn_801ec2f0`] converts the four packed pairs into the `0xF0` slots, marks
  out-of-window and duplicate points, counts the survivors at `+0x138`, asks the
  four wave-97 pair matchers for a verdict and rewrites the sensor-bar direction
  `+0x4C4`..`+0x4F0`; [`fn_801ebd20`] applies that verdict to the aim, scroll
  and camera-point filters; [`fn_801ecad0`] maps the extension report
  (types `1`, `2`, `0x10`, `0x11` and the balance board `3`) into the point,
  tilt and trigger words `+0x60`..`+0x80`. [`fn_801ed3b0`] is the 8-byte
  forwarder into the still-untranslated status-ring writer `0x801ED3C0`.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---:|---|---|---|
| `0x801EBD20` | `fn_801ebd20` | aim / scroll / camera-point filter step | 1,484 | translated | medium | `work/ghidra/main-gb/export/801ebd20_FUN_801ebd20.c` |
| `0x801EC2F0` | `fn_801ec2f0` | sensor-bar matcher and direction update | 1,148 | translated | medium | `work/ghidra/main-gb/export/801ec2f0_FUN_801ec2f0.c` |
| `0x801ECAD0` | `fn_801ecad0` | extension-report mapper (incl. balance board) | 2,256 | translated | medium | `work/ghidra/main-gb/export/801ecad0_FUN_801ecad0.c` |
| `0x801ED3B0` | `fn_801ed3b0` | forwarder into `0x801ED3C0` | 8 | translated | medium | `work/ghidra/main-gb/export/801ed3b0_FUN_801ed3b0.c` |
| `0x801EDAF0` | `fn_801edaf0` | KPAD record initialiser | 1,092 | translated | medium | `work/ghidra/main-gb/export/801edaf0_FUN_801edaf0.c` |
| `0x801EEB00` | `fn_801eeb00` | IOS heap bring-up | 248 | translated | high | `work/ghidra/main-gb/export/801eeb00_FUN_801eeb00.c` |
| `0x801EEC00` | `fn_801eec00` | `0`-returning stub | 8 | translated | high | `work/ghidra/main-gb/export/801eec00_FUN_801eec00.c` |
| `0x801EEC10` | `fn_801eec10` | IOS transfer completion | 412 | translated | medium | `work/ghidra/main-gb/export/801eec10_FUN_801eec10.c` |
| `0x801EEDB0` | `fn_801eedb0` | `OpenDevice` | 324 | translated | medium | `work/ghidra/main-gb/export/801eedb0_FUN_801eedb0.c` |
| `0x801EEF00` | `fn_801eef00` | `CloseDeviceAsync` | 292 | translated | medium | `work/ghidra/main-gb/export/801eef00_FUN_801eef00.c` |
| `0x801EF030` | `fn_801ef030` | long bulk request (IOS `10`) | 852 | translated | medium | `work/ghidra/main-gb/export/801ef030_FUN_801ef030.c` |
| `0x801EF390` | `fn_801ef390` | interrupt request (IOS `1`/`2`) | 856 | translated | medium | `work/ghidra/main-gb/export/801ef390_FUN_801ef390.c` |
| `0x801EF8B0` | `fn_801ef8b0` | control transfer builder | 1,444 | translated | medium | `work/ghidra/main-gb/export/801ef8b0_FUN_801ef8b0.c` |
| `0x801EFE60` | `fn_801efe60` | control transfer async entry point | 140 | translated | medium | `work/ghidra/main-gb/export/801efe60_FUN_801efe60.c` |

Totals: **14 translated**, 0 partial, 0 stubs; 2 high / 12 medium / 0 low;
10,564 bytes. Address range `0x801EBD20`-`0x801EFE60`.

## Corrections and caveats

1. **The `0x802DCDxx` split returns.** Every function in both clusters receives
   its first two words through one of the tiny `0x802DCD58`/`5C`/`60`/`64`/
   `68`/`70`/`74` accessors, which the export prints as a 64-bit value whose
   high half is `r3` and low half `r4`. The Rust keeps the established wave-97
   convention: the first two declared parameters are used directly
   (`fn_801ef390(fd, tag, ...)`, `fn_801eedb0(name, vendor, ...)`) and the
   epilogue twin (`0x802DCDA4`/`A8`/`AC`/`B0`/`B8`/`BC`/`C0`/`8C`) is called
   with the status word. Where the export reads the pair only to store it
   (`fn_801eec10`, `fn_801eef00`), `call_u64` is used exactly like
   `fn_801ef6f0` already does.
2. **`fn_801edaf0` takes two arguments.** The export declares the body `void`,
   but `0x802BD4A0` calls it as `FUN_801edaf0(record + 0x20, 0x40)` and the
   body reads both words through the `0x802DCD40` split; the quarter of the
   size word (`0x10`) is the status-slot count published at `+0x4C0`, and the
   pointer is the slot bank published at `+0x4BC`. The Rust signature is
   `(status_bank, size)`, with `medium` confidence for that recovery.
3. **Status banks.** `+0x4BC` is `status_bank + count * channel * 0x38` and the
   count is `size / 4`. Slots `0`..`0xF` live inline at `+0x13C` (status byte
   `+0x29` of each `0x38`-byte slot) and the overflow slots live in the
   caller's bank; the init clears the status byte of both sets to `0xFF`.
4. **`fn_801ef030` / `fn_801ef390` vector shape.** The ioctl vector table is
   three `(address, length)` pairs — `(tag block, 1)`, `(length block, 4 or 2)`
   and `(caller buffer, length)` — with `num_in = 2`, `num_out = 1`. The
   length block is filled with the raw length word, so the third pair repeats
   it as the output vector; that duplication is literal in the export.
5. **`fn_801ef8b0` setup layout.** The six single/word blocks are
   `bmRequestType(1)`, `bRequest(1)`, `wValue(2)`, `wIndex(2)`,
   `wLength(2)`, a zero padding byte `(1)`; the two 16-bit fields are stored
   byte-swapped (`param >> 8 | param << 8` in the export). The trailing pair
   `+0x30`/`+0x34` is the caller's data buffer and length, so the transfer is
   request `0`, `num_in = 6`, `num_out = 1`.
6. **`fn_801efe60` argument recovery.** The forwarder rebuilds its first two
   arguments from the `0x802DCD64` split and passes the same nine words plus
   the async flag `1`; the callers at `0x801F92C0` and `0x801F9840` confirm the
   order `(handle, 0x20, 0, 0, 0, length, buffer, callback, context)`.
7. **Trace calls.** The IOS layer is threaded with `0x801EE9A0`/`0x801EEA50`
   reports; both are gated variadic printers whose printer is a no-op in this
   build, so the calls are omitted and the guard conditions are kept in
   comments. `fn_801eeb00` keeps the `0xFFFFFFEA` return and the early exit of
   the "arena too small" path.
8. **`fn_801ecad0` field roles.** Only extension type `3` is proven (the
   balance-board `0x801DDAB0`/`0x801DDAC0`/`0x801DDB20` calls). Types `1`,
   `2`, `0x10` and `0x11` are described by their report ids and the axis shape
   (`1,1,2,2,2,1`), not by a proven controller name. The whole balance-board
   ladder is unreachable in this build because `0x801DDAB0` returns `0`
   unconditionally, and the export is transcribed anyway so the body stays
   correct if that stub is ever filled in. `0x801DDAC0`/`0x801DDB20` are the
   translated dummies, which ignore their arguments.
9. **`fn_801ecad0` axis mapper.** The mapper is `0x801EC770` by default and
   `fn_801ec8a0` when `r13-0x4B90` is non-zero; both are called by address with
   the same five words, because the choice is a runtime branch and
   `0x801EC770` is still untranslated.
10. **`fn_801ec2f0` matcher results.** The four matchers (`0x801EB5C0`,
    `0x801EB7B0`, `0x801EB9E0`, `0x801EBBA0`) are translated bodies that return
    their verdict through the `0x802DCDB8` epilogue helper, so the result is
    read with `call0_ret` at the address rather than from the `()`-returning
    Rust functions.
11. **`fn_801ebd20` constants.** `r2-0x4D34` is read but never named by an
    earlier pass; its value is not recovered, so the code reads the constant
    from `SDA2_BASE - 0x4D34` and the doc marks it `// unknown`. The
    `r13-0x4B80`/`-0x4B7C` pair is the existing wave-97 default vector.
12. **Module header.** The sixteenth-pass bullet and the promotion-note
    sentence were appended; no earlier count was stale (the previous text named
    no total).

## Deliberate skips

- **`0x801E1E20` (5,628 B).** The WPAD command-record writer. Its body builds
  `0x30`-byte records out of stack unions (`local_58`, `local_b8`, `local_118`,
  ...) whose upper halves Ghidra leaves undefined (`CONCAT22(local_70._0_2_,1)`
  where `local_70` is never written). The record contents therefore cannot be
  recovered from the export, and any body would be a guess. Its callers keep
  calling the address.
- **`0x801E39C0` (4,224 B).** The second WPAD command writer; same undefined
  stack-union defect, with `local_84`/`local_264`/`local_504` in the same role.
- **`0x801E55C0` (2,792 B).** The paired-single scale routine. The export is a
  chain of `unaff_GQR0`, `in_ps31_1`, `ldexpf` and
  "removing unreachable block" warnings; the paired-single semantics cannot be
  expressed without inventing the GQR mode at each site.
- **`0x801E9730` (1,776 B) and `0x801E9E20` (1,784 B).** The two random
  self-test report builders. Both read `extraout_var`/`extraout_var_00` (values
  the decompiler could not place in a register) and index the two `0x100`-entry
  tables at `0x80344400`/`0x80344D30`, so the reported values are not
  recoverable.
- **`0x801ED3C0` (1,836 B).** The KPAD status-ring writer the two forwarders
  jump into. Its first act copies `0x58` bytes from the caller's record into a
  stack image, then a rotate loop walks a 17..`block+0x4C0` ring whose slot
  pointer is chosen between the inline bank (`+0x13C + slot * 0x38`) and the
  external `0x803A1F84` bank, and the export reuses `iVar18` first as the ring
  index and then as the record count (and `iVar24` first as the channel and then
  as the count). The register-level data flow cannot be disambiguated without
  the caller's record layout, so the body is left for a pass that owns
  `0x802D71F0`. [`fn_801ed3b0`] and all callers keep calling the address.
- **`0x801EE230` (1,896 B).** The KPAD connect callback. Like `0x801E55C0` it
  opens with a GQR/paired-single prologue (`in_f31`, `in_ps31_1`, six
  `ldexpf` guards) before the load-cell code, so the entry state cannot be
  reconstructed honestly.
- **`0x801EE9A0` (168 B) and `0x801EEA50` (172 B).** The sixteen-parameter
  variadic trace forwarders. They build a `va_list` frame
  (`local_78`..`local_5c`, `local_18 = 0x1000000`) and call `0x802E1560`;
  `platform::call7` cannot express the frame, and the frame layout is Ghidra's
  reconstruction of the saved argument area. This is the same shape wave 102
  skipped at `0x801F9C50`; every caller keeps calling the address.
- **`0x801ED3A0` (12 B).** The sibling forwarder into `0x801ED3C0`. It is four
  bytes longer than the `8`-byte `0x801ED3B0` but the export shows the same
  single call, so the extra instruction is not visible; only the pure
  call+return sibling is translated.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 10,564
  bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs` (checked by name for all 14 addresses).
- No address was already in `ledger/functions.csv`; the 24-entry untranslated
  list for the bucket was derived from it first.
- `node tools/check-lane-csv.mjs wave103` prints `OK` and reports
  `wave103-input.csv: 14 rows`; the other lane CSVs present in the same run are
  clean too, and no address is duplicated across lanes.
- `cargo check --message-format=short` reports no error in
  `src/runtime/input.rs`; the only `input.rs` output is the two pre-existing
  "direct cast of function item into an integer" warnings recorded by wave 102.

## Ledger

Machine-readable rows: `ledger/agents/wave103-input.csv` (14 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
