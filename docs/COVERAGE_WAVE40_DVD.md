# Coverage - Wave 40 (runtime::dvd, 0x801C0000 bucket)

## How to read this file

The Wave 40 `0x801C0000` bucket is a whole 64 KiB page, not just the DVD band.
It contains the early VI/video-register, SI/controller and GX command code that
precedes the DVD/FST routines at `0x801CA640`. This pass takes a coherent slice
of the small, literal accessors in that page plus the three remaining
untranslated addresses inside the DVD band. The bodies live in
`src/runtime/dvd.rs` because it is the single writable file assigned to the
bucket; each doc block says explicitly when a register or field role is
`inferred:`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 28 | 600 | 9 | 19 |

## What this pass covers

- VI timing-register setters (`0x801C4FA0`-`0x801C50D0`, 9 bodies): the
  half-word packers at offsets `+0`, `+2`, `+4`, `+6` and `+8` of the block
  pointed to by the SDA slot `r13-0x4DE8`, plus the interrupt-locked word swap
  of `r13-0x4D98`. The offsets match the VI VTR/DCR/HTR/VBE/DPR layout, but
  that identity is inferred, so the confidence is `medium` except for the
  lock/swap function.
- Three-vector transform accessors (`0x801C66E0`-`0x801C6870`, 5 bodies): the
  `+0x28`/`+0x2C`/`+0x30` setter and getter, the negated `+0x34`/`+0x38`/
  `+0x3C` setter and getter, and the single-word copy into `+0xC`. The
  four-byte stores in a 16-byte body prove the parameters are `f32` with no
  `frsp` conversion.
- Texture/EFB descriptor accessors (`0x801C7100`-`0x801C7340`, 11 bodies):
  the packed-word field replacer, the `+0x10`/`+0x14`/`+0x18` word
  accessors, the ten-bit width/height extractors, the two-bit format
  extractors and the `+0x1F` flag bit.
- DVD-band leftovers (`0x801CA640`, `0x801CFA50`, `0x801CFA60`, 3 bodies):
  the half-word store into the `r13-0x4DEC` block, the constant-`1` return and
  the empty function.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C4FA0` | `fn_801c4fa0` | 16 | medium |
| `0x801C4FB0` | `fn_801c4fb0` | 24 | medium |
| `0x801C4FD0` | `fn_801c4fd0` | 20 | medium |
| `0x801C4FF0` | `fn_801c4ff0` | 92 | medium |
| `0x801C5050` | `fn_801c5050` | 20 | medium |
| `0x801C5070` | `fn_801c5070` | 24 | medium |
| `0x801C5090` | `fn_801c5090` | 20 | medium |
| `0x801C50B0` | `fn_801c50b0` | 28 | medium |
| `0x801C50D0` | `fn_801c50d0` | 68 | high |
| `0x801C66E0` | `fn_801c66e0` | 16 | medium |
| `0x801C66F0` | `fn_801c66f0` | 28 | medium |
| `0x801C6710` | `fn_801c6710` | 28 | medium |
| `0x801C6730` | `fn_801c6730` | 40 | medium |
| `0x801C6870` | `fn_801c6870` | 12 | high |
| `0x801C7100` | `fn_801c7100` | 20 | medium |
| `0x801C7120` | `fn_801c7120` | 8 | high |
| `0x801C7160` | `fn_801c7160` | 8 | high |
| `0x801C7170` | `fn_801c7170` | 8 | high |
| `0x801C71E0` | `fn_801c71e0` | 20 | medium |
| `0x801C7200` | `fn_801c7200` | 20 | medium |
| `0x801C7220` | `fn_801c7220` | 8 | high |
| `0x801C7230` | `fn_801c7230` | 12 | medium |
| `0x801C7240` | `fn_801c7240` | 12 | medium |
| `0x801C7250` | `fn_801c7250` | 12 | medium |
| `0x801C7340` | `fn_801c7340` | 8 | high |
| `0x801CA640` | `fn_801ca640` | 16 | medium |
| `0x801CFA50` | `fn_801cfa50` | 8 | high |
| `0x801CFA60` | `fn_801cfa60` | 4 | high |

## Deliberate skips

- `0x801C0020` (1196 bytes) and `0x801C9E20` (2076 bytes): the two largest
  routines in the page. Their exports are large state machines with many
  unresolved callees; translating them without a frame-by-frame read would
  pad the lane with guesses, so they are left for a dedicated pass.
- `0x801C4F70` and `0x801C5120`-`0x801C66D0`: the SI/controller
  sampling block. The functions mix `r13` register windows with calls to
  `0x801A9B90`/`0x801B4B30` whose roles are not established in this lane;
  only the standalone VI setters in that run were taken.
- `0x801C6880`-`0x801C70F0`: GX command builders bound to the `r2-0x4EF8`
  render state. They are valid candidates but belong to the render band, not
  to the DVD module; a later GX/display pass can take them with the correct
  state-structure names.
- `0x801C7260`-`0x801C9E20`: mixed descriptor decoders, DSP/CARD wrappers and
  the remaining GX write helpers. They need the `r13`/`r2` constant tables
  resolved before a `high` or `medium` row can be justified.
- No address already present in `ledger/functions.csv` was re-claimed; the
  59 DVD-band bodies from Waves 5/13/36/39 are untouched.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no errors in
  `src/runtime/dvd.rs`.
- `node tools/check-lane-csv.mjs wave40` prints `OK` for
  `ledger/agents/wave40-dvd.csv` (28 rows, 28 unique addresses).
