# Coverage - Wave 105 (resource::archive, `0x802B0000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 15 bodies are appended after the wave-102
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 15 | 8304 | 2 | 12 | 1 |

The file now holds 546 translated bodies: 531 before this pass plus the 15
below. The assigned `0x802B0000` bucket held exactly 16 untranslated functions
when this lane started; 15 are taken here and 1 is skipped with a reason (see
deliberate skips), so the bucket is complete apart from that one.

## What this pass covers

1. **RLYT header parser and section table** - `0x802B20B0` and `0x802B2C70`
   (2 bodies, 3312 bytes). `0x802B20B0` is the `.brlyt` header parser named in
   the wave-46 module header: it installs the `0x8034FB38` vtable, builds the
   `0x802AD910` message queue at `+0x28`, copies three 8-byte words and four
   words out of the source header, decodes the eleven packed counts of
   `source + 0x3C` through the already-translated `0x802B2760..0x802B2800`
   bitfield getters, clamps them through `0x802B2B10`, and then fills the
   texture, material, SRT, texcoord, colour and palette tables.
   `0x802B2C70` is the allocator behind it: it compares every packed field,
   releases the old block through `0x801D5550`, takes the new block from the
   EGG root heap through `0x801D5540`, repacks `+0x38`/`+0x3C` and seeds the
   five texture defaults.
2. **Section-entry collectors and window records** - `0x802B5CC0`,
   `0x802B5DA0`, `0x802B5E50`, `0x802B60B0` (4 bodies, 1416 bytes). The first
   counts a collector's accepted entries through vtable slots `+0x3C`/`+0x40`,
   the second sums that count over a manager list, the third builds the
   `0x28`-byte window record from two section scans, and the fourth binds the
   record into the `0x10`-byte slot array through `0x802AC080`/`0x802B49E0`.
3. **GX emitters and the overlay setup** - `0x802B6C80`, `0x802BBA30` and the
   two 4-byte thunks `0x802BE530`/`0x802BE5D0` (4 bodies, 2304 bytes). The first
   writes one position strip into the write-gather FIFO at `0xCC008000`, the
   second programs the whole overlay pipeline with 34 literal GX calls, and the
   thunks forward to the matrix helpers at `0x801C08A0`/`0x801C9940`.
4. **Controller and child-list helpers** - `0x802B7310`, `0x802BBFD0`,
   `0x802BC980`, `0x802BC9A0`, `0x802BD4A0` (5 bodies, 1272 bytes). These are
   the manager-list dispatcher, the ten-entry controller-layout ladder, the
   start pair for the `+0xB48` child list, and the EGG controller-manager
   bootstrap.

### Shared layout notes

`// inferred:` the owners are the RLYT layout header at `0x8034FB38` and the EGG
controller manager behind `r13 - 0x47FC`; the exports prove only the offsets,
the packed bitfields, the constants and the callee addresses.

- the `0x802DCD40..0x802DCD74` family are the Metrowerks register-save stubs;
  this file's established convention is **high word = incoming `r3`, low word =
  incoming `r4`**. `0x802BBFD0` and `0x802BD4A0` use the low word as the object,
  `0x802B7310`/`0x802B5DA0`/`0x802B5E50`/`0x802B60B0` use the high word as the
  record and the low word as the collector/owner. That split is what the exports
  show and is stated per body.
- `0x802B2C70`'s packed word at `+0x38` is the same count word the wave-54,
  wave-66 and wave-90 sections documented; the new bodies name each field after
  the bit positions they occupy (`state_count` = 28..31, `count_18` = 24..27,
  `count_14` = 20..23, `count_09` = 9..13, `flag_0e`, `count_0f`, `count_12`,
  and the four one-bit flags at 8..5).
- the two default words of the record fill are `r2 - 0x3FC0` and `r2 - 0x3FBC`,
  already named `SDA2_SECTION_DEFAULT`/`SDA2_802B1E70_LIMIT` in this file.
- `0x800046B4` is the memset wrapper and `0x80004364` the block copy, both
  already documented in this file.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B20B0` | `fn_802b20b0` | 1636 | medium |
| `0x802B2C70` | `fn_802b2c70` | 1676 | medium |
| `0x802B5CC0` | `fn_802b5cc0` | 216 | medium |
| `0x802B5DA0` | `fn_802b5da0` | 104 | medium |
| `0x802B5E50` | `fn_802b5e50` | 608 | medium |
| `0x802B60B0` | `fn_802b60b0` | 488 | medium |
| `0x802B6C80` | `fn_802b6c80` | 1376 | medium |
| `0x802B7310` | `fn_802b7310` | 112 | medium |
| `0x802BBA30` | `fn_802bba30` | 920 | medium |
| `0x802BBFD0` | `fn_802bbfd0` | 328 | medium |
| `0x802BC980` | `fn_802bc980` | 16 | medium |
| `0x802BC9A0` | `fn_802bc9a0` | 16 | low |
| `0x802BD4A0` | `fn_802bd4a0` | 800 | medium |
| `0x802BE530` | `fn_802be530` | 4 | high |
| `0x802BE5D0` | `fn_802be5d0` | 4 | high |

## Unknowns

1. **The `r2` window constants.** `0x802BBA30` reads `r2 - 0x3F38`,
   `r2 - 0x3F40` and `r2 - 0x3F48`; the first is used as the alpha scale of
   every setup call, the second is the `2^52` bias of the int-to-double idiom
   and the third is forwarded as the matrix shear. Their numeric values are not
   recovered, only their roles.
2. **`0x802BBA30`'s unused registers.** The export declares eight parameters and
   reads only `param_2`, `param_3`, `param_5`, `param_7` and `param_8`;
   `param_1`, `param_4` and `param_6` are never touched and are kept as explicit
   `unused4`/`unused6`/`arg1` arguments so the register order stays visible.
3. **`0x802BC9A0`'s step.** `0x802BDFC0` takes its step in `f1`, and the export
   prints no arguments at all, so the body issues a zero-argument call and is
   marked `low`. The guard and the callee address are certain.
4. **`0x802BD4A0`'s owner register.** The export declares the function `void`
   while every store goes through the low word of the save pair; the body takes
   that word as an explicit `this` argument and documents the split.
5. **`0x802B6C80`'s vertex stride.** The export unrolls the emit loop eight
   times; the body writes the equivalent `0x20` stride starting at `+0x14`. The
   three count-only scaffolding loops of the export touch no memory and are not
   reproduced.

## Deliberate skips

- **`0x802B0680` (1844 bytes)** - the four-corner textured-quad setup. Its four
  blocks compute a perspective-corrected glyph quad and then call
  `0x802B71E0` with six register arguments: `(&xy_a, &xy_b, 1, uv_array,
  colour_block_or_null, scale)`. The translated `0x802b71e0` body in this file
  only consumes `r7`/`r8` (the colour block and the scale) and forwards the
  remaining registers to `0x802B6C80` through the compiler's untouched `r3..r6`,
  so the per-corner UV array and the two position pairs cannot be attributed to
  a callee argument without inventing a calling convention. Ghidra's argument
  list for that call is a register snapshot, not a prototype, and the same
  shape appears in `0x802AFF60`. No body was written rather than guessing which
  registers the emitter reads.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose); the 15 sizes sum to 8304 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 15 addresses came from
  the untranslated part of the `0x802B0000` bucket.
- The module header gained item 18; its body count is 15 and the file total is
  546.
- `node tools/check-lane-csv.mjs wave105` reports 15 rows for
  `wave105-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`; the remaining errors name other lanes' files from
  the same wave, which are being edited in parallel.
