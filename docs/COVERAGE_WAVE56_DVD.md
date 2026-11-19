# Coverage - Wave 56 (runtime::dvd, GX channel and raster helpers in the 0x801C0000 bucket)

## How to read this file

The `0x801C0000` bucket is a whole 64 KiB page of `main.dol`; the DVD band
itself starts at `0x801CA640`. Wave 56 continues the practice of earlier passes:
it takes small, self-contained GX render-state routines that live in the page
before the DVD band and writes them into `src/runtime/dvd.rs`, the only writable
file assigned to the bucket. Every doc block says plainly that the routine is GX
code, not disc code.

All 28 routines read the GX state base from the SDA2 word at `r2 - 0x4EF8`
(the pointer documented in `src/render/gx.rs`). Most emit one word to the GX
write-gather FIFO at `0xCC008000` and then write an acknowledgement to
`state + 2`. Field roles that the export does not prove remain `inferred:` in
their doc blocks; every shift, mask and store is literal.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 28 | 2164 | 10 | 18 |

## What this pass covers

- Channel/register builder run `0x801C7D90`-`0x801C88F0` (20 bodies). The
  shared ten-field packer `fn_801c7d90` is translated first, then called
  directly by `fn_801c8130` and `fn_801c8180`, so their argument derivation is
  visible instead of hidden behind an absolute call. The run continues with the
  `+0x174` byte writer, the per-index `+0x180`/`+0x1C0`/`+0x200` nibble and
  selector setters, the `0xE1`/`0xF3`/`0xF5` command emitters, and the `+0x254`
  scale writers.
- Raster/offset helper run `0x801C9B40`-`0x801C9D90` (8 bodies). It covers the
  six-float block at `+0x544` (one unconditional writer and one with the
  `r2 - 0x4DD0` bias), the dot-product emit from `+0x558`/`+0x55C`/`+0x560`,
  the scissor words `+0x148`/`+0x14C` together with the decoder that recovers
  left/top/width/height, and two emitters that intentionally set the marker to
  one instead of zero.
- Both runs are contiguous address ranges, chosen so that adjacent bodies share
  field offsets and confirm each other (for example `fn_801c82f0` and
  `fn_801c8330`, or `fn_801c9c60` and `fn_801c9cd0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C7D90` | `fn_801c7d90` | 104 | medium |
| `0x801C8110` | `fn_801c8110` | 32 | medium |
| `0x801C8130` | `fn_801c8130` | 72 | high |
| `0x801C8180` | `fn_801c8180` | 84 | high |
| `0x801C81E0` | `fn_801c81e0` | 4 | high |
| `0x801C81F0` | `fn_801c81f0` | 48 | medium |
| `0x801C82F0` | `fn_801c82f0` | 64 | medium |
| `0x801C8330` | `fn_801c8330` | 64 | medium |
| `0x801C8370` | `fn_801c8370` | 88 | medium |
| `0x801C83D0` | `fn_801c83d0` | 88 | medium |
| `0x801C8430` | `fn_801c8430` | 96 | medium |
| `0x801C8490` | `fn_801c8490` | 100 | medium |
| `0x801C8500` | `fn_801c8500` | 92 | medium |
| `0x801C8560` | `fn_801c8560` | 80 | medium |
| `0x801C85B0` | `fn_801c85b0` | 80 | medium |
| `0x801C8600` | `fn_801c8600` | 60 | medium |
| `0x801C8640` | `fn_801c8640` | 124 | medium |
| `0x801C86C0` | `fn_801c86c0` | 56 | high |
| `0x801C8700` | `fn_801c8700` | 132 | high |
| `0x801C88F0` | `fn_801c88f0` | 36 | medium |
| `0x801C9B40` | `fn_801c9b40` | 144 | medium |
| `0x801C9BD0` | `fn_801c9bd0` | 60 | medium |
| `0x801C9C10` | `fn_801c9c10` | 44 | high |
| `0x801C9C60` | `fn_801c9c60` | 104 | medium |
| `0x801C9CD0` | `fn_801c9cd0` | 72 | high |
| `0x801C9D20` | `fn_801c9d20` | 60 | high |
| `0x801C9D60` | `fn_801c9d60` | 40 | high |
| `0x801C9D90` | `fn_801c9d90` | 136 | high |

## Deliberate skips

- `0x801C97F0`, `0x801C98B0`, `0x801C9900`, `0x801C9940`, `0x801C99C0`,
  `0x801C9A80`, `0x801C9C40`: these are the GQR-format conversion routines.
  Their exports depend on `unaff_GQR0`, a register the decompiler could not
  assign, and their three-way `4/5/6/7` byte/half-word/float branches would
  require inventing a GQR argument that the callers do not pass explicitly.
  They were not reconstructed.
- `0x801C9470` and `0x801C9530` (180 and 176 bytes): save/restore the whole
  `0x600`-byte state block through `FUN_80004364` and call the untranslated
  scheduler hooks `FUN_802dcd70`/`FUN_802dcd74`/`FUN_802dcdbc`/`FUN_802dcdc0`.
  The surrounding state ownership is not established, so the pair was left for
  a pass that also takes those hooks.
- `0x801C0020` (1196 bytes) and `0x801C9E20` (2076 bytes): still the two
  largest state machines in the page; they need a dedicated frame-by-frame
  read.
- `0x801C92xx`-`0x801C9Axx` builders outside the chosen tail slice, and the
  `0x801C6470`-`0x801C7D90` mid-page builders, remain for later passes; this
  lane took only the two contiguous runs it could justify at `high`/`medium`.
- No address already present in `ledger/functions.csv` was re-claimed; the 117
  previously translated addresses in this bucket are untouched.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no errors or warnings in
  `src/runtime/dvd.rs`.
- `node tools/check-lane-csv.mjs wave56` prints `OK` for
  `ledger/agents/wave56-dvd.csv` (28 rows, 28 unique addresses).
