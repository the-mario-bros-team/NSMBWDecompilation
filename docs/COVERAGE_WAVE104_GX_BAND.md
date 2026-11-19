# Coverage - Wave 104 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 104 pass over the `0x80280000..0x8028FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unrelocated `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The machine-readable
mirror is `ledger/agents/wave104-gx-band.csv` (same seven rows).

The bucket holds 158 Ghidra entries. 138 were already translated by waves 12,
18, 95, 96, 98 and 100; this pass re-read the remaining 20 and adds seven. The
thirteen still untranslated entries are listed with their exact blocker under
"Deliberate skips".

`status` is `translated` for six rows and `partial` for `0x8028E310`, whose two
GQR-quantised input arms are not modelled. `confidence` follows each body's
`# Confidence` line.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 7 | 4,656 | 1 | 6 |

## What this pass covers

- **Pool free-list allocate and sorted insert** (`0x802830A0`, 444 bytes). The
  free-list header is `+0x10` (tail at `+0x14`); when it is empty the head chunk
  at `+0x08` is recycled through the two release passes `0x80281390`/
  `0x802810A0` (now in-file) and its `+0x94` callback. Otherwise the tail node
  is initialised by `0x80280E60`, its level byte at `+0xA8` is set, the node is
  removed from the free list and inserted into the active list at `+0x04` with
  the same descending-level loop already proven for `0x80283420`. The walk
  refreshes neighbours through `0x802818C0` and reports the chunk to the
  manager. The two halves of the `0x802DCD64` stub are simply the incoming
  `r3`/`r4` (`this`, `rows`), which is what the callers `0x80268A50` and
  `0x8027DDD0` pass.
- **0x4C-byte record expansion** (`0x80283BD0`, 460 bytes). The six-word header
  (kind, present byte, count, `byte<<16 | u16`, two `0x80262750` ids) is
  followed by `count` rows. The first two rows copy a 0x30-byte payload as eight
  words plus seven half-words; every row stores the `0x80283DA0` relocation
  result in its first word. The kind table and the overlapping half-word copy
  are literal.
- **Squared-distance fade** (`0x80288390`, 460 bytes). The squared distance from
  `0x801C1650` (the already-translated runtime helper) is turned into a length
  with the `0x80237BA0` reciprocal square root, then mapped through two linear
  clamps into `[0, 1]`. All four endpoints/slopes are `f32` arguments; the two
  `0x8042EBB4`/`0x8042EBD4` words are 0.0 and 1.0.
- **Particle age accumulator** (`0x80288490`, 1,104 bytes). The whole
  `unaff_GQR0` pair is a dead entry/exit spill. The body maintains the `+0xE0`
  delay counter, the `+0x38` accumulated value, the `+0xEC` LCG and the
  `+0x2C`/`+0x34` rates, optionally modulates by the distance fade through the
  `+0x90` matrix and the `+0xB8` descriptor, then dispatches the resolved object
  at `+0xF0` either directly through vtable slot 8 or through the controller
  `+0x44` hook first. The hook packet is the export's six pointers
  (`count`, `flags`, six `+0x4C` words, descriptor `u16`, angle/100, twelve
  caller words). Whole units are removed from `+0x38` after the dispatch.
- **Strip-key walk** (`0x802888E0`, 436 bytes). The `+0xB8` descriptor is a
  three-level self-relative table (`u32[+4]` header, header word, key array at
  `+4`). For each key the code reads the `u16` limit at `count*8+4`, the record
  pointer at `count*8+key*4+8`, filters on record byte `0xAB`/`0xAC` and on
  `record[4] & 8`, and calls `0x8028A6F0` with `(record, this+record[1]+0x20,
  key, u16[+0xEA], limit)`. A record byte above `0x6F` marks the node for
  `0x802891C0`.
- **Channel key resolver** (`0x8028C7A0`, 1,468 bytes). The five selector arms
  are the reversed, wrap, ping-pong and fractional variants of the track
  sampler, including the `cntlzw(step & 1)` even-step zero. The selected
  `0x10`-byte entry is located by the export's binary search and either supplies
  the fixed `+0xC..+0xE` fields or a hash-driven pick from
  `desc+0x20 + u32[+0x10] + u32[+0xC]`. The hash fold is the literal
  `0x3F81F635`/`0x30A74193`/`0x7B929`/`0x371097E7` mix. The result updates
  `out+0x88`/`+0x8C`/`+0x90` and the per-channel nibble/bit fields at
  `out+0x94`/`+0x96`.
- **Unit-box classifier** (`0x8028E310`, 284 bytes, `partial`). Two `y` mask
  bits, three plain range tests and the `z*z + x*x + y*y` radius produce
  `1`/`2`/`3`. The GQR type-0 float arm is the live one: the caller
  `0x80291100` fills the same stack vector through `0x801C1160` and reads it
  with the other five unit-box classifiers as plain `f32`. The packed-byte and
  packed-halfword arms are not reproduced.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802830A0` | `fn_802830a0` | 444 | medium |
| `0x80283BD0` | `fn_80283bd0` | 460 | medium |
| `0x80288390` | `fn_80288390` | 460 | high |
| `0x80288490` | `fn_80288490` | 1104 | medium |
| `0x802888E0` | `fn_802888e0` | 436 | medium |
| `0x8028C7A0` | `fn_8028c7a0` | 1468 | medium |
| `0x8028E310` | `fn_8028e310` | 284 | medium (partial) |

## Deliberate skips

Thirteen entries of the bucket stay untranslated after a full read. They fall
into three groups.

1. **Live GQR-quantised stores** (8 entries): `0x802850F0`, `0x80287C10`,
   `0x80288E30`, `0x8028E890`, `0x8028ED30`, `0x8028F2D0`, `0x8028F860`,
   `0x8028FEA0`. In these the `ldexpf` result is not discarded: it feeds a
   `CONCAT`/store ladder that writes packed vertex or stack records, and the
   `2^scale` factor of the configured GQR is not printed. Guessing it changes
   every stored value.
2. **Large multi-track evaluators** (4 entries): `0x80289410` (4,792 bytes),
   `0x8028A6F0` (4,012 bytes), `0x8028B6C0` (4,280 bytes) and `0x8028CEE0`
   (3,468 bytes). Their `ldexpf` pairs are dead, but their Ghidra C is a
   600-880-line projection of packed per-channel descriptors: overlapping
   `byte`/`ushort` aliases on the same fields, two binary searches, and an LCG
   hash/dither whose fold is printed as four views of one stack slot. A faithful
   body needs the raw instruction stream; an approximation would silently change
   which key wins. `0x8028A6F0` is called opaquely by [`fn_802888e0`].
3. **Register-file-sized textured emitter** (1 entry): `0x802821E0` (2,988
   bytes). All 32 `ldexpf` blocks are dead spills, but the live data path takes
   `f18..f27` as inputs and projects twenty-eight stack locals and three
   10-argument UV-ramp calls. The export does not prove which float register
   maps to which ramp argument, so the driver cannot be written honestly.

## Verification

- Every address and byte size matches `functions.tsv` (7 rows, 4,656 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 307
  translated entries and this pass added 7 (6 `translated`, 1 `partial`).
- `node tools/check-lane-csv.mjs wave104` reports no problem naming this lane's
  CSV.
- `cargo check --message-format=short` reports no error or warning in
  `src/render/gx_band.rs`; the only errors in the tree are in
  `src/runtime/bt.rs`, another lane's file.
- The `sdata2` constants used by this pass were read from
  `work/dol/main.elf` (`r2 = 0x80433080`): `0x8042EBB0` = 100.0,
  `0x8042EBB4` = 0.0, `0x8042EBD4` = 1.0, `0x8042EBD8` = 65536.0,
  `0x8042EBDC` = 2.0, `0x8042EC20` = -1.0, `0x8042EC24` = 1.0 and
  `0x8042EC28` = 0.0. They are listed in section 16 of the module header.
