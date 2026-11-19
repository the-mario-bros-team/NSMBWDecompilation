# Coverage - Wave 80 (runtime::dvd, the GX command pipeline in the `0x801C0000` page)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801C0000` 64 KiB page of `main.dol`.
Only part of that page is disc code: the DVD/FST band is
`0x801CA640`-`0x801D6780`. Everything below `0x801CA640` is other SDK code that
lives in the same page, and `src/runtime/dvd.rs` is the only writable file
assigned to the bucket, so those routines are reconstructed there too. Each
function doc says plainly which routine it is and which fields it touches.

This pass covers the GX command pipeline at `0x801C2CE0`-`0x801C56C0`. It is
the block immediately below the Wave 67 draw-token work and shares its `r13`
flags (`-0x4DC7`/`-0x4DC8`), its 16-bit mirror block (`-0x4DEC`) and its two
nine-word FIFO control blocks at `0x80390200` (CP) and `0x80390224` (GP). Word 0
of each block is the write pointer and word 1 the read pointer, which is what
`fn_801c2d70` compares; `+0x20` is a control word whose spare bytes carry the
"busy"/"active" markers.

The GX state block itself is the SDA2 pointer at `r2 - 0x4EF8`, as documented in
`src/render/gx.rs`; `+0x14`/`+0x18` hold the texgen and texcoord bit fields,
`+0x1C`/`+0x3C`/`+0x5C` the per-stage records, `+0x520`/`+0x524`/`+0x525` the
CLUT format and its two dirty bytes, `+0x5F9`/`+0x5FA` the indirect-stage flags
and `+0x5FC` the per-feature dirty mask.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 25 | 7676 | 12 | 13 |

## What this pass covers

- FIFO control `0x801C2CE0`-`0x801C35C0` (4 bodies, 1060 bytes). The eight-word
  record initialiser [`fn_801c2ce0`] (write limit, length, half length, and the
  pointer triple stored under the `0x801B1140`/`0x801B1180` interrupt lock), the
  CP/GP write-pointer comparison [`fn_801c2d70`] with its two `0x8015F730`
  reports, the token refresh [`fn_801c32e0`] that Wave 67's [`fn_801c3440`]
  already calls, and the guarded GP commit [`fn_801c35c0`], which mirrors eight
  half-words into the status block and drains through `0x801A9B90`.
- Texturing selector set `0x801C37C0`-`0x801C4070` (7 bodies, 2244 bytes). One
  21-case switch writes `+0x14`/`+0x18` (plus the CLUT words `+0x520`/`+0x524`/
  `+0x525`) in two forms - the immediate entry point [`fn_801c37c0`] and the
  table-driven [`fn_801c3a30`] - and [`fn_801c3e20`] reads the same fields back.
  [`fn_801c3fe0`] materialises the whole `(id, value)` table. Around them sit the
  derived `0x21` command emitter [`fn_801c3c40`] (two `countLeadingZeros`
  fields), the size-code summation [`fn_801c3cf0`] over the three `r13` byte
  tables at `-0x60C8`/`-0x60CC`/`-0x60D0`, and the reset [`fn_801c4070`].
- Per-stage twin `0x801C40B0`-`0x801C4780` (6 bodies, 1780 bytes). The
  `+0x1C`/`+0x3C`/`+0x5C` triple written by [`fn_801c40b0`] and
  [`fn_801c4250`], read back by [`fn_801c44a0`] and tabulated by
  [`fn_801c4700`]; plus the `+0x5FB` stage flush [`fn_801c4420`] and the raw
  FIFO store [`fn_801c4780`]. Both setters set bit 4 of `+0x5FC` and the
  stage's bit in `+0x5FB`, so the flush emitter and the flush dispatcher agree
  on the mask layout.
- Texcoord/indirect setup and the retrace pair `0x801C47D0`-`0x801C5200`
  (7 bodies, 1960 bytes). [`fn_801c47d0`] maps the texcoord type id to a size
  code, stores `+0xC8`/`+0xE8` and packs the six-bit bias fields into
  `+0x80`/`+0x84`; [`fn_801c4a50`] writes the count word and the two
  indirect-stage bytes; [`fn_801c4b70`]/[`fn_801c4ce0`] run the two-sample VI
  synchronisation and pulse `0xCC003018`; [`fn_801c4ea0`] flushes `0x45000002`
  and sleeps on the GX wait queue; [`fn_801c5120`]/[`fn_801c5200`] are the two
  retrace handlers that set bits 2 and 3 of the IRQ block's `+0xA`.
- Dirty-state dispatch `0x801C52F0` (1 body, 632 bytes). [`fn_801c52f0`] reads
  the `+0x5FC` mask and emits the TEV colour words (`+0xA8`-`+0xB4`), the
  `+0xB8`/`+0xE8` per-stage loops, the derived texture command, the stage flush
  and the three late flush steps, then clears the mask.

The slice is contiguous apart from the addresses already landed by Wave 40/67
(`0x801C33E0`-`0x801C3540`, `0x801C3760`/`0x801C37B0`, `0x801C47C0`,
`0x801C4A20`, `0x801C4AD0`/`0x801C4B30`, `0x801C4F70`-`0x801C50D0`,
`0x801C51B0`, `0x801C5280`); none of them is re-claimed here.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C2CE0` | `fn_801c2ce0` | 140 | high |
| `0x801C2D70` | `fn_801c2d70` | 248 | medium |
| `0x801C32E0` | `fn_801c32e0` | 256 | medium |
| `0x801C35C0` | `fn_801c35c0` | 416 | medium |
| `0x801C37C0` | `fn_801c37c0` | 612 | high |
| `0x801C3A30` | `fn_801c3a30` | 524 | high |
| `0x801C3C40` | `fn_801c3c40` | 176 | medium |
| `0x801C3CF0` | `fn_801c3cf0` | 300 | medium |
| `0x801C3E20` | `fn_801c3e20` | 436 | high |
| `0x801C3FE0` | `fn_801c3fe0` | 144 | high |
| `0x801C4070` | `fn_801c4070` | 52 | high |
| `0x801C40B0` | `fn_801c40b0` | 416 | high |
| `0x801C4250` | `fn_801c4250` | 452 | high |
| `0x801C4420` | `fn_801c4420` | 128 | medium |
| `0x801C44A0` | `fn_801c44a0` | 604 | high |
| `0x801C4700` | `fn_801c4700` | 116 | high |
| `0x801C4780` | `fn_801c4780` | 64 | high |
| `0x801C47D0` | `fn_801c47d0` | 580 | medium |
| `0x801C4A50` | `fn_801c4a50` | 128 | medium |
| `0x801C4B70` | `fn_801c4b70` | 356 | medium |
| `0x801C4CE0` | `fn_801c4ce0` | 432 | medium |
| `0x801C4EA0` | `fn_801c4ea0` | 204 | medium |
| `0x801C5120` | `fn_801c5120` | 132 | medium |
| `0x801C5200` | `fn_801c5200` | 128 | medium |
| `0x801C52F0` | `fn_801c52f0` | 632 | high |

## Deliberate skips

- `0x801C2E70` (480 bytes) and `0x801C3050` (656 bytes): the CP/GP block
  loaders. Both exports read their nine-word source record from the "return
  value" of `0x802DCD74`, which the Ghidra export shows is one of the
  Metrowerks register-save stubs (`_savegpr`) and has no return value at all.
  The record's real origin therefore cannot be established from the export, and
  inventing one would be worse than leaving the pair for a pass with the
  disassembly in hand. The neighbouring [`fn_801c2ce0`] and
  [`fn_801c35c0`] cover the same record layout from the other direction.
- `0x801C5570` (332 bytes) and `0x801C56C0` (216 bytes): the two FIFO padding
  writers. Ghidra models the padding stores as 16-bit writes to
  `0xCC008002` and then emits the final word as `CONCAT22(half, _DAT_cc008002)`,
  i.e. it merges a write-only FIFO register's previous 16-bit content into the
  last store. The high half is known but the low half is unrecoverable from the
  export, so neither body can be written honestly in full.
- `0x801C35C0`'s callers outside the slice, and the untranslated callees
  `0x801C78D0`, `0x801C81E0`, `0x801C9D90`, `0x801C9B40`, `0x801C97F0`,
  `0x801C1C80`: called by absolute address with a comment, not translated.
- No address already present in `ledger/functions.csv` was re-claimed. After
  this pass the `0x801C0000` bucket holds 95 untranslated addresses (120 before
  the pass).

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (25 rows, 7676 bytes total).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/runtime/dvd.rs`; the outstanding errors in the workspace are all in the
  parallel `src/audio/seq.rs` lane.
- `node tools/check-lane-csv.mjs wave80` prints `OK` for
  `ledger/agents/wave80-dvd.csv` (25 rows, 25 unique addresses, 12 high and 13
  medium).
