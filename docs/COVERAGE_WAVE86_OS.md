# Coverage - Wave 86 (`runtime::os`, `0x801b0000` bucket)

## How to read this file

This page records wave 86 of the `main.dol` reconstruction for the
`runtime::os` lane: a pass over the `0x801b0000` bucket, which holds 227
inventory entries. 108 of them were still untranslated when the lane started;
the other 119 are already merged, a handful of them by `runtime::link` and
`runtime::init`, which share this address range.

- **Status** - `translated` means every branch, store and call in the Ghidra
  export has a counterpart in `src/runtime/os.rs`. All 28 rows here are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export
  itself. `medium` means the flow is explicit but at least one callee is
  unnamed and its role is read from its argument shape or from an adjacent
  string.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes come from
  `work/ghidra/main-gb/export/functions.tsv`, re-read for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave86-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 28 | 2,712 | 18 | 10 |

Status split: 28 `translated`, 0 `partial`, 0 `stub`. No `low`.

## What this pass covers

- **Group 1 - text, NAND and net helpers, `0x801B63B0..0x801BCCF0`
  (10 bodies).** The UTF-8 decoder at `0x801B63B0` (lead-byte length table,
  continuation prefix test, three overlong bounds, surrogate rejection); the
  two ISFS record checks `0x801B7C30` (`shared2/sys/NANDBOOTINFO`, eight-byte
  read, `?` marker, error `-0xC`, create with mode `0x3F`) and `0x801B8020`
  (`shared2/expired`, 184-byte record, zeroed 32-byte console id, `-8` on a
  short read); the `/dev/net/kd/time` ioctl wrapper `0x801B7AE0`; the two boot
  flag leaves `0x801B7E40`/`0x801B8140`; the display-mask programmer
  `0x801B8A30`; the `0x801B9F80` forwarder; the `0x8038F7D0` table close; and
  the `sda(-0x4E14)` wait loop `0x801BCCF0`, which reuses the already-merged
  `sleep_thread` (`0x801B5C90`).
- **Group 2 - boot words and TV format, `0x801BDE40..0x801BE470` (7 bodies).**
  Two one-load returns (`sda(-0x4E58)`, `sda(-0x4E14)`), the TV-format mapper
  `0x801BE010`, the masked `0xCC00206E` bit read, the guarded read/replace of
  `sda(-0x4E20)`, and the two "clear and report success" leaves.
- **Group 3 - the `0xE0` channel senders, `0x801BEDA0..0x801BFFD0`
  (11 bodies).** Every body builds a two- to five-byte stack request, calls
  `0x801BE860(0xE0, request, len)` and then `0x801BE480(2)`. The requests are
  the payloads: the video-mode announcement (`0x801BEDA0`, which also maps
  `0x800000CC` onto the `sda(-0x4DF4)` mode word), the two-byte `0x6E` command,
  three colour/brightness triples, the four 7-bit values under `0x7A`, the
  transition flag, the `0x0A` mode-conditional request and the `{1, 3}`
  "mode 3" pair. Two neighbours in the same run, `0x801BFEA0`/`0x801BFEB0`,
  pass fixed records to `0x801BFD40` and `0x801BFFC0` sets bit `0x80` of
  `sda(-0x4E08)`.

## What the pass adds beyond the raw bodies

1. **The request layer is a single shape.** Eleven exports collapse onto one
   helper, `wave86_send_channel_request`, because all eleven end with the same
   two calls and the same `0xE0` channel id. The differences between them are
   only the request bytes, so the settings they carry are now readable as data
   instead of as eleven stack-frame layouts.
2. **The `sda(-0x4E14)` pair is closed.** `0x801BCCF0` sleeps until that word
   changes and `0x801BDED0` reads it, so the two halves of the handshake no
   longer need an export to be read together.
3. **The `0x801B1510`/`0x801B1590` mask pair is now used from the video path.**
   `0x801B8A30` is a caller well outside the exception layer: masks
   `0x410000`, `0x80000` and `0x40` are set or cleared per video mode, which
   fixes the role of both helpers without reading their bodies again.
4. **`0x800000CC` is a mode selector, not a pointer.** `0x801BEDA0` compares it
   against `1`, `2` and `5` only; that is the whole recovered meaning, so it is
   documented as three values rather than as a named hardware register.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B63B0` | `fn_801b63b0` | 276 | high |
| `0x801B7AE0` | `fn_801b7ae0` | 292 | medium |
| `0x801B7C30` | `fn_801b7c30` | 176 | medium |
| `0x801B7E40` | `fn_801b7e40` | 24 | high |
| `0x801B8020` | `fn_801b8020` | 276 | medium |
| `0x801B8140` | `fn_801b8140` | 24 | high |
| `0x801B8A30` | `fn_801b8a30` | 232 | medium |
| `0x801B9F50` | `fn_801b9f50` | 40 | medium |
| `0x801BB1C0` | `fn_801bb1c0` | 140 | medium |
| `0x801BCCF0` | `fn_801bccf0` | 84 | high |
| `0x801BDE40` | `fn_801bde40` | 8 | high |
| `0x801BDED0` | `fn_801bded0` | 8 | high |
| `0x801BE010` | `fn_801be010` | 96 | high |
| `0x801BE070` | `fn_801be070` | 60 | medium |
| `0x801BE2F0` | `fn_801be2f0` | 84 | medium |
| `0x801BE450` | `fn_801be450` | 20 | high |
| `0x801BE470` | `fn_801be470` | 16 | high |
| `0x801BEDA0` | `fn_801beda0` | 144 | medium |
| `0x801BEE40` | `fn_801bee40` | 64 | high |
| `0x801BEE80` | `fn_801bee80` | 88 | high |
| `0x801BEEE0` | `fn_801beee0` | 100 | high |
| `0x801BEF50` | `fn_801bef50` | 108 | high |
| `0x801BFEA0` | `fn_801bfea0` | 16 | high |
| `0x801BFEB0` | `fn_801bfeb0` | 24 | medium |
| `0x801BFED0` | `fn_801bfed0` | 92 | high |
| `0x801BFF30` | `fn_801bff30` | 132 | high |
| `0x801BFFC0` | `fn_801bffc0` | 16 | high |
| `0x801BFFD0` | `fn_801bffd0` | 72 | high |

## Unknowns

1. **`0x801BE860` and `0x801BE480` are unnamed.** Every body in group 3 calls
   them with a literal argument list (`0xE0`, pointer, length) and (`2`), so
   the calls are recorded faithfully, but what the channel does and what the
   `2` means are `// unknown:`.
2. **`0xCC00206E` bit 0.** The read is literal; the register sits in the
   drive-interface window at `0xCC002000` and nothing in the export names it.
3. **`0x800000CC`.** Only three comparison values (`1`, `2`, `5`) appear, so the
   Rust documents the mapping, not a register name.
4. **The `0x801BFF30` shift.** `(sda(-0x4E00) << 1) | 1` is truncated to eight
   bits before the request is sent; whether the word can exceed `0x7F` is not
   visible in this window.
5. **`0x8038F7D0` table length.** `0x801BB1C0` divides the key by `0x30` and
   indexes a `0x20`-byte entry array without clamping; every caller is outside
   this window.
6. **`0x801B7C30`'s `-0xC`.** It is only used as "file does not exist"; the
   matching ISFS error constant name is `// inferred:`.

## Deliberate skips

- **`0x801B3010`** (108 bytes). Read and left out: it walks a link list at
  `+0x2F4` and then calls `FUN_801b5d80()` with **no** arguments in the export,
  while that callee's own body clearly consumes a queue pointer. The argument
  is missing, and choosing one (`param_1`? the popped entry?) would be a guess.
- **`0x801B3140`** (4 bytes) and **`0x801B3230`** (4 bytes). Read and left out:
  both are thunks. The first is `FUN_801b4b20`'s two-zero-stores body, the
  second is the 244-byte `FUN_801b5d80` body, printed only because Ghidra
  followed the branch. Their real bodies are one instruction each, so a row
  here would duplicate `init_thread_queue`/`wake_all`, which are already in
  `ledger/functions.csv`.
- **`0x801B3240`** (108 bytes). Read and left out: it stores `_DAT_80003194`
  through `unaff_r13 - 0x4FE0` and builds a stack frame for `0x801AEFA0` whose
  fourth argument is a local array Ghidra never initialises beyond `local_18[0]`.
- **`0x801B32B0`** (20 bytes). Read and left out: two `r13`-relative loads
  (`-0x4F98`, `-0x4F94`) are exact, but the function is called with two output
  pointers that the export does not otherwise constrain; the two slots are never
  written anywhere in this window, so the role would be invented.
- **`0x801BDDD0`** (108 bytes) and **`0x801BDE50`** (120 bytes). Read and left
  out: the first needs a five-pointer call and the second a nine-argument call,
  and neither the extra arguments nor the callees' roles are recoverable here.
- **`0x801BDF80`** (144 bytes), **`0x801B7E60`** (444 bytes),
  **`0x801B7CE0`** (344 bytes), **`0x801B8250`** (160 bytes). Read and left
  out: each mixes `r13`/`r2` small-data reads with either `double` arithmetic
  (`0x801BDF80`, `0x801B7E60`) or a checksum walk whose accumulator Ghidra
  splits across two overlapping pointer expressions (`0x801B7CE0`), or a
  function-pointer slot and a seven-argument thread create (`0x801B8250`).
- **`0x801BE480`** (136 bytes). Read and left out: the 64-bit time-base
  subtraction, the `0x801DCE C4` scaling call and the loop condition are printed
  as `undefined8` shuffles that cannot be written without guessing which half
  is which.
- **Bodies not attempted** - the other 68 entries still untranslated in the
  bucket after this pass: `0x801B0170` (784), `0x801B06A0` (436),
  `0x801B0860` (648), `0x801B3410` (384), `0x801B3590` (224),
  `0x801B3670` (284), `0x801B3790` (212), `0x801B3870` (648),
  `0x801B3CE0` (312), `0x801B3E20` (512), `0x801B4020` (736),
  `0x801B4310` (292), `0x801B4560` (284), `0x801B4680` (272),
  `0x801B4820` (112), `0x801B6080` (808), `0x801B6D60` (1,204),
  `0x801B7220` (84), `0x801B7280` (492), `0x801B7750` (184),
  `0x801B7810` (148), `0x801B82F0` (460), `0x801B84C0` (344),
  `0x801B8620` (308), `0x801B8760` (516), `0x801B8B20` (636),
  `0x801B8DA0` (164), `0x801B8E50` (248), `0x801B8F50` (636),
  `0x801B91D0` (132), `0x801B9260` (384), `0x801B93E0` (280),
  `0x801B9500` (184), `0x801B95C0` (300), `0x801B96F0` (252),
  `0x801B97F0` (184), `0x801B98B0` (536), `0x801B9AD0` (208),
  `0x801B9BA0` (452), `0x801B9D70` (252), `0x801B9E70` (216),
  `0x801B9F80` (936), `0x801BA330` (376), `0x801BA4B0` (444),
  `0x801BA670` (392), `0x801BA800` (756), `0x801BAB00` (1,004),
  `0x801BAEF0` (192), `0x801BAFB0` (432), `0x801BB250` (364),
  `0x801BB3C0` (692), `0x801BB680` (436), `0x801BBA30` (408),
  `0x801BBBD0` (2,060), `0x801BC480` (280), `0x801BC5A0` (512),
  `0x801BC7A0` (1,352), `0x801BCD50` (680), `0x801BD000` (224),
  `0x801BD0E0` (404), `0x801BD280` (1,740), `0x801BD950` (852),
  `0x801BDCB0` (276), `0x801BE0B0` (564), `0x801BE510` (840),
  `0x801BE860` (1,332), `0x801BEFC0` (3,448), `0x801BFD40` (352). Each was
  read only far enough to confirm it was not a two-line leaf; their callees are
  no better named after this pass.
- The 119 addresses already merged in this bucket are not repeated here; they
  are listed in `docs/COVERAGE_WAVE83_OS.md` and in the `runtime::link` /
  `runtime::init` CSVs. Every row in this CSV was checked against
  `ledger/functions.csv` before it was written.

Remaining untranslated in the bucket after this wave: 108 - 28 = 80 entries.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read for this pass, not copied from prose).
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every CSV row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/os.rs`; no address appears twice.
- No address in the CSV was already present in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave86` ->
  `wave86-os.csv: 28 rows`, `total rows: 28, unique addresses: 28`,
  `translated/high=18 translated/medium=10`, `OK`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/runtime/os.rs`; the 14 warnings the crate emits are in `runtime::bt`,
  `runtime::cpp` and `runtime::input`.
