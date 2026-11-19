# Coverage - Wave 46 (resource::archive, 0x802B0000 bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. Bodies are appended in address order at the
end of the file (wave-46 section) so the run can be read top to bottom.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 28 | 1152 | 26 | 2 |

The file now holds 178 bodies: 150 before this pass plus the 28 below.

## What this pass covers

The assigned bucket `0x802B0000` still has 207 untranslated functions of 306.
Wave 40 finished the genuinely archive-owned clusters (U8 member table, LZ
bindings, EGG root heap) and left the rest to the subsystems that own the
layout/text/controller objects. This pass adds the two remaining blocks that
read or write **archive-band state directly**, so no other module is
duplicated:

1. **RLYT header accessors and deleting destructor** - `0x802B2720..0x802B2BC0`
   (25 bodies). The `RLYT` layout-header parser `0x802B20B0` constructs an
   object with vtable `0x8034FB38` and calls every one of these accessors while
   it walks a `.brlyt` member. The block is one shared header: a packed flag
   word at `+0x18` (bits 12, 13..14, 15..17, 18..22, 23, 24, 25, 27), packed
   bytes at `+0x02`/`+0x03` (2-bit and 3-bit fields), and small copies used for
   the per-axis records and colour quads. `0x802B2BC0` is the matching deleting
   destructor; it restores the vtable, frees the `+0x40` block through the EGG
   root heap, tears down the `0x802DCA98` array at `+0x28`, unlinks `+0x04`
   through `0x80229030` and optionally frees the object through `0x802B9280`,
   the same release path the heap factories already use.
2. **32-byte default-record array** - `0x802B68B0..0x802B6A40` (3 bodies). A
   small vector of `0x20`-byte records: byte capacity at `+0x00`, byte count at
   `+0x01`, record pointer at `+0x04`. `0x802B68B0` grows it on the EGG root
   heap (`r13 - 0x48B8`, [`EGG_ROOT_HEAP`]), `0x802B6950` fills new records
   from the eight shared defaults at `0x80423840..0x8042385C` (lazy-init byte
   at `r13 - 0x4890`, sources `r2 - 0x3F70`/`r2 - 0x3F6C`), and `0x802B6A40`
   copies records in and raises the count. The `0x80423840` defaults sit in the
   same archive small-data band as `G_ARC_HEAP_LIST` (`0x80423880`), which is
   why this block is filed here.

### Small-data addresses

`r13 = 0x8042F680` (`crate::engine::startup::SDA_BASE`) and
`r2 = 0x80433080` (`SDA2_BASE` in this file). The wave-46 bodies resolve:

| body | expression | absolute |
|---|---|---|
| `0x802B68B0`, `0x802B2BC0` | `r13 - 0x48B8` | `0x8042ADC8` (EGG root heap) |
| `0x802B6950` | `r13 - 0x4890` | `0x8042ADF0` (defaults-ready byte) |
| `0x802B6950` | `r2 - 0x3F70` / `r2 - 0x3F6C` | `0x8042F110` / `0x8042F114` |
| `0x802B6950` | `0x80423840..0x8042385C` | eight default words |

The two `r2` words are copied bit-for-bit; their numeric meaning is not printed
by the export, so no float interpretation is claimed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B2720` | `fn_802b2720` | 36 | high |
| `0x802B2750` | `fn_802b2750` | 12 | high |
| `0x802B2760` | `fn_802b2760` | 12 | high |
| `0x802B2770` | `fn_802b2770` | 12 | high |
| `0x802B2780` | `fn_802b2780` | 12 | high |
| `0x802B2790` | `fn_802b2790` | 12 | high |
| `0x802B27A0` | `fn_802b27a0` | 12 | high |
| `0x802B27B0` | `fn_802b27b0` | 12 | high |
| `0x802B27C0` | `fn_802b27c0` | 12 | high |
| `0x802B27D0` | `fn_802b27d0` | 12 | high |
| `0x802B27E0` | `fn_802b27e0` | 12 | high |
| `0x802B27F0` | `fn_802b27f0` | 12 | high |
| `0x802B2800` | `fn_802b2800` | 12 | high |
| `0x802B2810` | `fn_802b2810` | 20 | high |
| `0x802B2830` | `fn_802b2830` | 12 | high |
| `0x802B2840` | `fn_802b2840` | 12 | high |
| `0x802B2850` | `fn_802b2850` | 20 | high |
| `0x802B2870` | `fn_802b2870` | 20 | high |
| `0x802B2890` | `fn_802b2890` | 20 | high |
| `0x802B28B0` | `fn_802b28b0` | 36 | high |
| `0x802B28E0` | `fn_802b28e0` | 36 | high |
| `0x802B2910` | `fn_802b2910` | 36 | high |
| `0x802B2940` | `fn_802b2940` | 12 | high |
| `0x802B2950` | `fn_802b2950` | 44 | high |
| `0x802B2BC0` | `fn_802b2bc0` | 172 | medium |
| `0x802B68B0` | `fn_802b68b0` | 148 | medium |
| `0x802B6950` | `fn_802b6950` | 236 | high |
| `0x802B6A40` | `fn_802b6a40` | 148 | high |

## Cluster notes

- The accessors at `0x802B2760..0x802B2800` are eleven independent masks over
  the **same** word: `0xF`, `>>4 & 0xF`, `>>8 & 0xF`, `>>0x19 & 1`,
  `>>0x1B & 1`, `>>0xC & 1`, `>>0x17 & 1`, `>>0x18 & 1`, `>>0xD & 3`,
  `>>0xF & 7`, `>>0x12 & 0x1F`. Each is reproduced with its literal shift and
  mask; no field name is asserted because the export never prints one.
- `0x802B2810` and `0x802B2850` are the only read-modify-writes. Both preserve
  every bit outside their mask: `0xF87FFFFF` clears bits 23..26 and
  `0xFF87FFFF` clears bits 19..22. The first stores `param_2` at bits 25..26
  and `param_3` at bits 23..24; the second stores `param_2` (3 bits) at bits
  20..22 and `param_3` (1 bit) at bit 19.
- `0x802B2870`/`0x802B2890` add one before masking, so the stored 3-bit value
  wraps `7 -> 0` and the 1-bit value is the inverse of the source bit.
- `0x802B28B0`, `0x802B28E0` and `0x802B2910` are three separate byte-for-byte
  copies of one four-byte field; all three exist as distinct call targets in
  the parser, so all three are kept rather than folded into one helper.
- `0x802B2950` copies five words, not four or eight. The export shows five
  loads followed by five stores and no loop, so the odd length is literal.
- `0x802B68B0` only grows. It compares the byte capacity at `+0x00` with the
  requested count, frees the old block when one exists, allocates
  `count * 0x20` through `0x801D5540(root_heap, size)` and only then updates
  the capacity byte. A zero allocation leaves the capacity unchanged.
- `0x802B6950` refuses to write when the pointer is null or the capacity is too
  small; it still stores the new count afterwards. The defaults block is
  initialised once from two `r2` words: index 0,1,3,4 receive the first word
  and index 2,5,6,7 the second, exactly as the eight assignments appear.
- `0x802B6A40` never lowers the count (`max(current, count)`) and performs no
  capacity check, unlike its sibling. The copy is eight words per record.

## Unknowns

1. **Field names in the RLYT header.** The 24 accessors are structurally
   certain but the export prints no field names, so each doc block uses
   `// unknown:` and describes only the bit positions. No `RLYT` class or
   member name is invented.
2. **`0x801D5550`'s freed pointer.** In `0x802B68B0` and `0x802B2BC0` the
   export passes only the EGG root heap, never the block at `+0x04`/`+0x40`.
   Both bodies reproduce the observed call and mark the missing argument.
3. **The base-destructor slot.** `0x802B2BC0` calls `PTR_FUN_8034FB50` with no
   argument in the export. The body forwards `this`, matching the idiom used by
   the wave-41 deleting destructors; it is marked `// inferred:`.
4. **The two `r2` defaults.** `0x8042F110`/`0x8042F114` are copied without
   interpretation; whether they are source rectangles, colours or floats is
   not shown.

## Deliberate skips

- **`0x802B20B0` (1636 B)** - the RLYT parser itself. It is a large
  resource-specific state machine with `lyt`-owned sections (`timg` lookup
  through a vtable at `+0x0C`, `FUN_802B35E0` table builds) and belongs to
  `ui::lyt`, not to the archive reader. Only the accessors it calls are
  adopted here.
- **`0x802B2C70` (1676 B)** - the matching RLYT constructor. Same reason; the
  accessors and destructor are enough to explain the header layout without
  pulling the whole layout object into this file.
- **`0x802B2980..0x802B2B50`** - the remaining RLYT field accessors
  (`0x802B2980`, `0x802B29B0`, `0x802B2A20..0x802B2B50`). They are 8-44 byte
  getters over neighbouring offsets whose owning sub-object is not resolved by
  the export; the wave-46 block already covers the parser's hot path, and
  adding unresolved offsets would be padding rather than reconstruction.
- **`0x802B4AE0..0x802B4DF0`** - glyph width tables and GX text drawing; text
  renderer, not archive.
- **`0x802B5AF0..0x802B6320`** - the `RLAN` layout-animation reader. It is a
  resource decoder, but its object graph (parameter/pane/material records and
  `0x802B67E0` name compares) belongs to `ui::lyt`; translating it here would
  duplicate that module's ownership.
- **`0x802B6AE0..0x802B7310`** - colour scaling and GX display-list emission
  for the text renderer.
- **`0x802B0680..0x802B1B00`, `0x802B1C90..0x802B2070`** - NW4R text/font
  rendering and the layout constructor: float/paired-single code (`CONCAT44`,
  `ldexpf`, `GQR0`) plus `r2`-relative constants whose absolute address is not
  recoverable from the C export.
- **`0x802B3300..0x802B3940`, `0x802B4940..0x802B49E0`, `0x802B50A0..0x802B50D0`**
  - glyph tables, small GX helpers and the two list wrappers; not archive.
- **`0x802BAC30..0x802BFxxx`** - EGG display/controller objects. They do call
  the archive heap (`0x802B8CC0`, `0x802B8A70`, `0x802B9280` in
  `0x802BC670`/`0x802BD4A0`/`0x802BE100`), but they are window/controller code,
  not archive readers; `0x802BC670`'s `0x801C1DB0` callee is the GX
  initialiser, exactly as wave 40 recorded.
- **`0x802B9700`, `0x802BE530`, `0x802BE5D0`** - 4-byte thunks; skipped per the
  brief.

## Verification

- Every address and byte size matches `functions.tsv` (read from the TSV, not
  from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/802bxxxx_FUN_802bxxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; none of the 28 overlaps the
  wave 24/30/36/39/40/41 archive ranges.
- `node tools/check-lane-csv.mjs wave46` reports `wave46-archive.csv: 28 rows`
  and `OK` with no problem naming this file.
- `cargo check --message-format=short` is free of errors for
  `src/resource/archive.rs`; the 13 warnings are pre-existing and live in
  `runtime::bt`, `runtime::cpp` and `runtime::input`.
