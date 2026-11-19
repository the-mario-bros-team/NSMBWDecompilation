# Coverage - Wave 46 (runtime::dvd, GX state setters in the 0x801C0000 bucket)

## How to read this file

The `0x801C0000` bucket is a whole 64 KiB page of `main.dol`, much wider than
the DVD band that starts at `0x801CA640`. Wave 40 took the VI/vector/texture
accessors in the low part of that page; this pass takes a further contiguous
slice of the **GX render-state setter band** that Wave 40 deliberately left for
a later pass. The bodies live in `src/runtime/dvd.rs` because it is the single
writable file assigned to the bucket, so each doc block states plainly that the
routine is GX code and not disc code.

All 30 routines read the GX state base from the SDA2 word at `r2 - 0x4EF8`
(the same pointer documented in `src/render/gx.rs`) and most emit one word to
the GX write-gather FIFO at `0xCC008000`. Field roles that the export does not
prove stay `inferred:`/`unknown:` in the doc block; the byte packing itself is
literal.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 30 | 1420 | 6 | 24 |

## What this pass covers

- GX state packers `0x801C57A0`-`0x801C5B00` (12 bodies): the `+0x7C` texture
  words, the per-index `+0x108` flag pair, the `+0x230`/`+0x234` and
  `+0x240`/`+0x244` size pairs, the `+0x238` shifted field, and the
  `+0x23C`/`+0x24C`/`+0x254`/`+0x5FC` field updates. The `+0x...` offsets and
  their masks are literal in the exports; the field names are inferred.
- Object field writers `0x801C6430`-`0x801C6460` (3 bodies): six `f32` stores
  at `+0x10`..`+0x24`, then the matching three-store halves. The four-byte
  stores fix the parameter width at `f32` even though Ghidra prints `double`
  sources.
- GX state exchange accessors `0x801C7710`/`0x801C7730` (2 bodies): return the
  old `+0x518`/`+0x51C` word and store the new one.
- GX command helpers `0x801C76A0`, `0x801C8220`, `0x801C6130`, `0x801C63F0`
  (4 bodies): the biased-position `0x65` word builder, the `+0x174` re-emit, the
  `+0x23C` two-bit setter and the constant `0x560003FF` emit.
- GX state packers `0x801C8E10`-`0x801C9030` (9 bodies): the `+0x220` bit
  setters (bits 2, 3, 4), the `+0x228`/`+0x224` two-field packers, the `+0x22C`
  bit-6 setter, the table-indexed `0x801C8EE0` transform selector, the `0x44`
  two-bit emit, and the `+0x7C` bit-22 / twin `0x68` command pair.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C57A0` | `fn_801c57a0` | 52 | medium |
| `0x801C57E0` | `fn_801c57e0` | 52 | medium |
| `0x801C5820` | `fn_801c5820` | 60 | medium |
| `0x801C5860` | `fn_801c5860` | 40 | medium |
| `0x801C5890` | `fn_801c5890` | 28 | medium |
| `0x801C58B0` | `fn_801c58b0` | 60 | medium |
| `0x801C58F0` | `fn_801c58f0` | 36 | high |
| `0x801C5920` | `fn_801c5920` | 64 | medium |
| `0x801C5960` | `fn_801c5960` | 64 | medium |
| `0x801C59A0` | `fn_801c59a0` | 36 | medium |
| `0x801C5AE0` | `fn_801c5ae0` | 32 | medium |
| `0x801C5B00` | `fn_801c5b00` | 40 | medium |
| `0x801C6130` | `fn_801c6130` | 20 | medium |
| `0x801C63F0` | `fn_801c63f0` | 56 | high |
| `0x801C6430` | `fn_801c6430` | 28 | medium |
| `0x801C6450` | `fn_801c6450` | 16 | medium |
| `0x801C6460` | `fn_801c6460` | 16 | medium |
| `0x801C76A0` | `fn_801c76a0` | 32 | medium |
| `0x801C7710` | `fn_801c7710` | 20 | high |
| `0x801C7730` | `fn_801c7730` | 20 | high |
| `0x801C8220` | `fn_801c8220` | 36 | high |
| `0x801C8E10` | `fn_801c8e10` | 44 | medium |
| `0x801C8E40` | `fn_801c8e40` | 44 | medium |
| `0x801C8E70` | `fn_801c8e70` | 52 | medium |
| `0x801C8EB0` | `fn_801c8eb0` | 48 | medium |
| `0x801C8EE0` | `fn_801c8ee0` | 168 | medium |
| `0x801C8F90` | `fn_801c8f90` | 44 | medium |
| `0x801C8FC0` | `fn_801c8fc0` | 48 | medium |
| `0x801C8FF0` | `fn_801c8ff0` | 52 | high |
| `0x801C9030` | `fn_801c9030` | 112 | medium |

## Deliberate skips

- `0x801C0020` (1196 bytes) and `0x801C9E20` (2076 bytes): the two largest
  routines in the page. Their exports are large state machines with unresolved
  callees; translating them without a frame-by-frame read would pad the lane
  with guesses.
- `0x801C5120`-`0x801C56C0`: the SI/controller run before the taken GX setters.
  The exports mix `r13` register windows with calls to `0x801A9B90`/
  `0x801B4B30` whose roles are still not established, so the lane takes only the
  standalone GX routines.
- `0x801C5B30`-`0x801C5F40`: the larger transform/lighting builders between the
  two taken setter runs; they call unresolved helpers and were not
  reconstructed.
- `0x801C6150`-`0x801C63F0`: the transform/state helpers around the taken
  `0x801C63F0` constant emit; the neighbouring bodies depend on unresolved
  callees.
- `0x801C6470`-`0x801C76A0`: the mid-page GX builders, decoders and descriptor
  helpers (including the 524-byte `0x801C6D90`); they need the state-structure
  names from `src/render/gx.rs` before a `high`/`medium` row can be justified.
- `0x801C76C0`-`0x801C8DC0`: the remaining render-state and command builders in
  the page; only the literal setters listed above were taken.
- `0x801C90A0`-`0x801C9E20`: the remaining large transform/decoder routines,
  including `0x801C90A0` (976 bytes), which needs a dedicated frame read.
- `0x801C6FA0` was skipped deliberately: it calls the untranslated
  `0x801C6D90` and reads a seventh argument from the caller's stack
  (`in_stack_00000008`), so a faithful Rust signature cannot be justified from
  this export alone.
- No address already present in `ledger/functions.csv` was re-claimed; the 87
  previously translated addresses in this bucket (including the 28 Wave 40
  bodies) are untouched.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no errors in
  `src/runtime/dvd.rs`.
- `node tools/check-lane-csv.mjs wave46` prints `OK` for
  `ledger/agents/wave46-dvd.csv` (30 rows, 30 unique addresses).
