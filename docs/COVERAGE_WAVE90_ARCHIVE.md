# Coverage - Wave 90 (resource::archive, `0x802b0000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 28 bodies are appended after the wave-89
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 28 | 2056 | 19 | 9 |

The file now holds 408 translated bodies: 380 before this pass plus the 28
below. The assigned `0x802b0000` bucket had 94 untranslated functions of 306
when this lane started; 28 of them are taken here.

## What this pass covers

The remaining `0x802B0000` band is mostly RLYT polygon/layout data, GX draw
setup and small NW4R screen-effect objects. This pass takes the bodies whose
field offsets, masks, constants and callee addresses are all visible in the
exports:

1. **Record copy and the twelve `+` helpers** - `0x802B2A20` and
   `0x802B2A80..0x802B2B40` (13 bodies). One more four-byte copy in the same
   band as `0x802B2980`/`0x802B2A50`, then twelve identical one-instruction
   additions that continue the compiler-generated `record + offset` family
   already partly taken by wave 79.
2. **List insert and polygon payload decode** - `0x802B49E0`, `0x802B4C60`
   (2 bodies). The insert thunk mirrors `fn_80268d50` with the offsets shifted
   to `+0x04`/`+0x08`; the decoder fills the record whose `+0x18` bitfield is
   the one `fn_802b4c40` uploads, including the `0x6000` field and the
   two-bit second-descriptor value.
3. **GX attribute-table setup and the header thunk** - `0x802B6B90`,
   `0x802B9700` (2 bodies). Attribute 9, optional attribute `0xB` and the
   `0xD + i` loop, each enabled through `0x801C37C0` and formatted through
   `0x801C40B0`; the thunk returns the word twelve bytes before its argument.
4. **Effect-object construction and stream-ring state** - `0x802BAC70`,
   `0x802BBDD0`, `0x802BC270`, `0x802BC350`, `0x802BC440`, `0x802BC560`,
   `0x802BC6F0` (7 bodies). The constructor's vtable plus grey/white defaults,
   the two-call wrapper, the interrupt-masked ring insert with its two count
   bytes, the cursor advance and release pair, the `0x10`-byte block allocator
   and the two float range ladders.
5. **Child-field copy, SDA resets and the 3x4 matrix** - `0x802BD1E0`,
   `0x802BD360`, `0x802BD3F0`, `0x802BE540` (4 bodies). The `+0x24..+0x2C`
   copy plus the `+0x38`/`+0x3C` snapshot through `fn_802bc8c0`, the two SDA
   word clears, the empty body, and the twelve-word fill whose diagonal comes
   from the second `r2` constant.

### Shared layout notes

`// inferred:` the records are RLYT polygon payloads and NW4R screen-effect
objects; the exports prove the offsets, the masks, the constants and the callee
addresses only.

- the polygon record keeps its mode bits at `+0x18`; `fn_802b4c40` reads
  `>> 0xD & 3` and the mask `0x0FFF9FFF` clears the `0x6000` field;
- the effect list is `{ head, cursor, pending, live count, total count }` with
  the two byte counters at `+0x0C`/`+0x0D` and the ring links at `+0x0C`/`+0x10`
  of each node;
- the stream object's rejected/pending node is at `+0x08` and its `+0x08` state
  word distinguishes `0` (free), `1` (playing) and `2` (pending);
- `r13 - 0x4870` is [`G_ARC_HEAP_CURRENT`], the same archive heap the wave-36
  section documents; `r13 - 0x4820`, `-0x4810` and `-0x480C` are effect-object
  slots introduced here.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B2A20` | `fn_802b2a20` | 36 | high |
| `0x802B2A80` | `fn_802b2a80` | 8 | high |
| `0x802B2A90` | `fn_802b2a90` | 8 | high |
| `0x802B2AA0` | `fn_802b2aa0` | 8 | high |
| `0x802B2AB0` | `fn_802b2ab0` | 8 | high |
| `0x802B2AC0` | `fn_802b2ac0` | 8 | high |
| `0x802B2AD0` | `fn_802b2ad0` | 8 | high |
| `0x802B2AE0` | `fn_802b2ae0` | 8 | high |
| `0x802B2AF0` | `fn_802b2af0` | 8 | high |
| `0x802B2B00` | `fn_802b2b00` | 8 | high |
| `0x802B2B20` | `fn_802b2b20` | 8 | high |
| `0x802B2B30` | `fn_802b2b30` | 8 | high |
| `0x802B2B40` | `fn_802b2b40` | 8 | high |
| `0x802B49E0` | `fn_802b49e0` | 52 | medium |
| `0x802B4C60` | `fn_802b4c60` | 208 | medium |
| `0x802B6B90` | `fn_802b6b90` | 228 | medium |
| `0x802B9700` | `fn_802b9700` | 4 | high |
| `0x802BAC70` | `fn_802bac70` | 128 | medium |
| `0x802BBDD0` | `fn_802bbdd0` | 84 | medium |
| `0x802BC270` | `fn_802bc270` | 224 | high |
| `0x802BC350` | `fn_802bc350` | 232 | medium |
| `0x802BC440` | `fn_802bc440` | 288 | medium |
| `0x802BC560` | `fn_802bc560` | 104 | medium |
| `0x802BC6F0` | `fn_802bc6f0` | 172 | medium |
| `0x802BD1E0` | `fn_802bd1e0` | 116 | high |
| `0x802BD360` | `fn_802bd360` | 16 | high |
| `0x802BD3F0` | `fn_802bd3f0` | 4 | high |
| `0x802BE540` | `fn_802be540` | 64 | high |

## Unknowns

1. **The `r2` range constants.** `r2 - 0x3F20`, `-0x3F1C`, `-0x3F18` and
   `-0x3F14` are read at runtime; `fn_802bc6f0` reads the slots but does not
   invent values or names.
2. **The matrix constants.** `fn_802be540` reads `r2 - 0x3EF0` and
   `r2 - 0x3EEC`; the export proves only that indices 0, 5 and 10 receive the
   second word.
3. **`0x802281D0`, `0x802282F0`, `0x80229160`, `0x802331A0`, `0x801BC3E0`,
   `0x801BC7A0`, `0x802BBE30`, `0x801C4070`, `0x801C37C0`, `0x801C40B0`,
   `0x801C8E70`, `0x801C6150`, `0x801BDDD0`, `0x80233420`.** The exports prove
   the argument lists only; they are released as raw call targets.
4. **`0x802BC350`'s interrupt mask.** The original calls `0x801B1140` without
   saving the result and still calls `0x801B1180` with no argument. The body
   transcribes that literally rather than repairing it.
5. **`0x802BC560`'s third argument.** It calls `0x802BC670(iVar1, size, heap)`,
   while `0x802BC670`'s own export drops the heap. The body calls
   [`fn_802bc670`] with the two arguments its Rust signature documents and
   records the discrepancy.
6. **`0x802B4C60`'s pair layout.** The two payload pointers come back from
   `0x802282F0` as `{first, second}`; the `+0x04`/`+0x08`/`+0x16` field roles
   are inferred from the sibling uploader `fn_802b4c40`.

## Deliberate skips

- **The `CONCAT44(0x43300000, ...)` and GQR families** - `0x802BBF40`,
  `0x802B4AE0`, `0x802B4DF0`, `0x802BAF70`, `0x802BB030`, `0x802BB840`,
  `0x802BB9A0`, `0x802BE470..0x802BE500`, `0x802BE640`/`0x802BE6B0`,
  `0x802BE5D0`, `0x802BF540`, `0x802BF650`, `0x802BF930`, `0x802BFDA0`,
  `0x802BFE90`, `0x802BFF70`, `0x802B1D60`. These need double-precision
  constant materialisation and GQR scale factors that `src/platform.rs` does
  not model; transcribing them would invent the `ldexpf` path.
- **Virtual-call bodies reached through the `r13 - 0x4838` hook** -
  `0x802B4940`, `0x802BAC30`, `0x802BAED0`, `0x802BC1A0`, `0x802BC7A0`,
  `0x802BC980..0x802BC9C0`, `0x802BD310`, `0x802BD370`, `0x802BD420`,
  `0x802BD460`, `0x802BD8C0`, `0x802BB840`. The exports drop the receiver
  (`(**(code **)(...))()`), so the argument lists would be invented.
- **Multi-level virtual dispatch with aliased locals** - `0x802B5CC0`,
  `0x802B5DA0`, `0x802B5E50`, `0x802B60B0`, `0x802B7310`, `0x802BE880`. The
  dispatch slots `+0x3C`/`+0x40`/`+0x44`/`+0x6C` and the `undefined8` return
  pair are not recoverable from the export.
- **Constructors whose export lost an argument** - `0x802B1C90`,
  `0x802BF3D0`, `0x802BE580`, `0x802BE540`'s caller side. Each shows a bare
  `FUN_xxxxxxxx()` where the callee clearly consumes `this`; the bodies would
  rest on an invented register.
- **Large literal initialisers and renderers** - `0x802B0680`, `0x802B1E70`,
  `0x802B20B0`, `0x802B2C70`, `0x802B6C80`, `0x802BBA30`, `0x802BC9E0`,
  `0x802BD4A0`, `0x802BD970`, `0x802BDE60`, `0x802BEA00`, `0x802BED10`,
  `0x802BEF00`, `0x802BF080`. These are 0.4-1.8 KB bodies left for a dedicated
  pass.
- **The remaining bucket** - `0x802B1D60..0x802B20B0`, `0x802B5E50..0x802B60B0`,
  `0x802BB030..0x802BBDD0`, `0x802BC9E0..0x802BD310`, `0x802BD4A0..0x802BDE60`
  and `0x802BEA00..0x802BF080` hold the bodies above plus further float and
  render code.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 28 addresses came from
  the untranslated part of the `0x802b0000` bucket.
- The module header gained item 12 and its body count is 28.
- `node tools/check-lane-csv.mjs wave90` reports 28 rows for
  `wave90-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`.
