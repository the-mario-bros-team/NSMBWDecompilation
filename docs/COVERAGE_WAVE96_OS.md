# Coverage - Wave 96 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records wave 96 of the `main.dol` reconstruction for the
`runtime::os` lane: the render-state record layer of the `0x80240000` bucket.
The bucket holds 137 inventory entries; when the lane started 77 of them were
already in `ledger/functions.csv` and 60 were untranslated. 25 of those 60 are
reconstructed here.

- **Status** - `translated` means every guard, store and call printed in the
  Ghidra export has a counterpart in `src/runtime/os.rs`. All 25 rows are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export.
  `medium` means the flow and the offsets are explicit but at least one callee
  role, field meaning or parameter is `// unknown:` / `// inferred:` in the
  body. `low` marks a body whose *instruction sequence* is certain but whose
  folded scratch layout (a heavily unrolled cache search, a named stack-slot
  transpose) had to be interpreted.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes were re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave96-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 25 | 10,960 | 1 | 22 | 2 |

Status split: 25 `translated`, 0 `partial`, 0 `stub`.

## What this pass covers

- **Cluster A - curve-sampled readers, `0x80241020`/`0x80241710` (2 bodies).**
  `0x80241020` maps a two-bit reader selector in the record's `+0x04` flag word
  onto the three curve entries `0x802403C0`/`0x80240240`/`0x80240100`, and
  honours the "already a literal float" bits `0x80000`/`0x100000`/`0x200000`.
  `0x80241710` normalises a unit vector and derives yaw/pitch/roll through
  `0x80237BA0` (square root) and two `0x80237D10` angle calls.
- **Cluster B - slot constructors, `0x80242C30`, `0x80242D80`, `0x802481B0`,
  `0x80248990` (4 bodies).** `0x80242C30` publishes `count * 2 + 0x30` bytes and
  installs `0x8034E1E4` → `0x8034E1A0`; `0x802481B0` publishes
  `align4(count * 2 + 0x27) + count * 8` and installs
  `0x8034E690` → `0x8034E638` → `0x8034E5E0`; `0x80248990` publishes
  `selected * 0x4C + count * 2 + 0x34` and installs `0x8034E690` → `0x8034E588`.
  `0x80242D80` is the three-instruction "run the `+0x28` kernel, store at
  `+0x18`, notify through vtable slot `+0x0C`" wrapper.
- **Cluster C - the four identical three-word refreshers, `0x80243F10`,
  `0x802456F0`, `0x80246CF0`, `0x80248B80` (4 bodies).** Same code, four entry
  types: recompute the width with the kernel at `+0x28`, store it at `+0x18`,
  then walk the `u16` index array at `+0x14` and forward every entry whose bit
  15 is clear to `0x8023F370`/`0x8023F5A0`/`0x8023FA30`/`0x80241690` with
  strides `0x5C`/`0x44`/`0xE8`/`0x4C`.
- **Cluster D - the record caches, `0x8024B720`..`0x8024B1E0` (5 bodies).**
  `0x8024B720` compares eight eight-word records against `0x803DEFC0` and
  republishes the `0x803DEF50` pair through `0x801C7200`/`0x801C71E0`;
  `0x8024B990` does the three-word variant against `0x803DF0E0` with the
  halfword mask `0x803DF140` and invalidates the matching `0x803DF0C0` bit;
  `0x8024B1E0` resolves one material's channel bitmask and colour bytes;
  `0x8024B0E0` re-initialises the eight-entry channel table.
- **Cluster E - the packed-state detector and the per-channel passes,
  `0x8024BAE0`..`0x8024CDC0` (8 bodies).** `0x8024BAE0` is the five-byte
  change detector (`0xFFFFFFFC` for bytes 0..1, `0xFFFFFFFE` for bytes 2..4);
  `0x8024BCB0`, `0x8024C0E0`, `0x8024CC10` and `0x8024CDC0` replay the
  `0x803DEF50` pair, push the Wave 92 packed word, and forward records to
  `0x8023C950`/`0x8023C610`/`0x8023C6F0`; `0x8024C2B0` is the vertex/colour/
  texcoord FIFO writer; `0x8024C960` rebuilds the eight channel matrices from
  `0x803DF180`.
- **Cluster F - the matrix-slot selectors, `0x8024AAC0`, `0x8024D2C0`,
  `0x8024D420` (3 bodies).** Three views of the same 32-slot cache: install from
  `0x803E29E8`, select/clear with the `0x803DF980` latch, and copy into the four
  parallel slot arrays.

## What the pass adds beyond the raw bodies

1. **The `0x30`-byte record layout is now closed.** Twelve bodies read the same
   fields in the same order: vtable `+0x00`, class word `+0x04`, heap `+0x08`,
   count `+0x10`, index table `+0x14`, width `+0x18`, scale floats
   `+0x20`/`+0x24`, width kernel `+0x28`, matrix `+0x2C`, entry array `+0x30`
   and sample count `+0x34`. `0x80248990` adds the `0x4C`-strided entry arrays
   and `0x802481B0` the `align4(count * 2 + 0x27)` split between the `u16` index
   array and the second `count`-word array.
2. **`0x80243F10` settles the entry header.** Bit 15 of the first halfword means
   "hidden" and the low 14 bits are an index into the entry array; the four
   sibling bodies differ only in the stride, so the four entry types are
   `0x44`, `0x4C`, `0x5C` and `0xE8` bytes.
3. **The two masks in `0x8024B720`/`0x8024B990` are complementary.**
   `0x803DF0C0` is a *byte* mask for the eight-word records and `0x803DF140` a
   *halfword* mask for the three-word records; `0x8024B990` clears the byte bit
   of the record it just cached, which is why the two passes cannot be merged.
4. **`0x8024BAE0` fixes the packed-state byte widths.** Bytes 0..1 invalidate
   both latch bits (`& 0xFFFFFFFC`), bytes 2..4 only bit 0 (`& 0xFFFFFFFE`), and
   byte 4 falls back to the value `3`.
5. **The `0x803DEF50` pair table is a real pair.** `0x8024B720` stores the two
   halfwords returned by `0x801C7200`/`0x801C71E0` at `+0x00`/`+0x02` of a
   four-byte slot, and `0x8024BCB0`/`0x8024CC10`/`0x8024CDC0` replay them into
   `0x803DEF70 + channel * 8` before calling `0x80249D60`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80241020` | `fn_80241020` | 608 | medium |
| `0x80241710` | `fn_80241710` | 404 | medium |
| `0x80242C30` | `fn_80242c30` | 336 | medium |
| `0x80242D80` | `fn_80242d80` | 100 | medium |
| `0x80243F10` | `fn_80243f10` | 192 | medium |
| `0x802456F0` | `fn_802456f0` | 192 | medium |
| `0x80246CF0` | `fn_80246cf0` | 192 | medium |
| `0x80247140` | `fn_80247140` | 156 | medium |
| `0x802481B0` | `fn_802481b0` | 372 | medium |
| `0x80248990` | `fn_80248990` | 496 | medium |
| `0x80248B80` | `fn_80248b80` | 192 | medium |
| `0x8024AAC0` | `fn_8024aac0` | 272 | medium |
| `0x8024B0E0` | `fn_8024b0e0` | 244 | medium |
| `0x8024B1E0` | `fn_8024b1e0` | 1,044 | low |
| `0x8024B720` | `fn_8024b720` | 616 | medium |
| `0x8024B990` | `fn_8024b990` | 332 | medium |
| `0x8024BAE0` | `fn_8024bae0` | 460 | high |
| `0x8024BCB0` | `fn_8024bcb0` | 528 | medium |
| `0x8024C0E0` | `fn_8024c0e0` | 464 | medium |
| `0x8024C2B0` | `fn_8024c2b0` | 1,556 | medium |
| `0x8024C960` | `fn_8024c960` | 684 | low |
| `0x8024CC10` | `fn_8024cc10` | 420 | medium |
| `0x8024CDC0` | `fn_8024cdc0` | 752 | medium |
| `0x8024D2C0` | `fn_8024d2c0` | 184 | medium |
| `0x8024D420` | `fn_8024d420` | 164 | medium |

## Deliberate skips

The other 35 untranslated addresses in the bucket were left alone. The ones
read for this pass and rejected:

1. **`0x80240100` (308), `0x80240240` (380), `0x802403C0` (492) and the
   `0x80242xxx` matrix family (`0x80242840`, `0x80242930`, `0x80242A40`).**
   Every store is selected at run time by an untyped `unaff_GQR0`/`unaff_GQR3`
   gather-register word (three quantised forms plus the float form). Reproducing
   only the float form would drop observable stores, so the bodies stay open
   until the gather setup itself is reconstructed.
2. **`0x80241EB0` (1000)** depends on `extraout_f1`, a register the callee
   `0x8023F000` leaves behind. `0x80241C70` (364) and `0x80248E00` (260) were
   already recorded as absent by the Wave 92 pass for the same `0x802DCDxx`
   half-ordering reason, and their sibling `0x80244190` (248) repeats
   `0x80248E00` byte for byte, so none of the four is repeated here.
3. **`0x80243000` (188) and `0x802430C0` (140)** are listed by Wave 92 as
   "deliberately absent" because the `undefined8` halves they split are not
   provably ordered. That reason still holds and is not re-litigated.
4. **`0x8024A610` (204), `0x8024EA30` (192), `0x8024EAF0` (216), `0x8024EE30`
   (872) and `0x8024FEF0` (828).** All five write their results through the
   GQR-selected `char`/`short`/`float` store triple. The control flow is clear
   but a faithful body would have to model the gather register, which no
   translated body in this file does yet.
5. **`0x8024DCE0` (1172).** The dispatcher that ties this cluster together calls
   `0x8024D2C0` with **no** argument in the export although that callee consumes
   the index returned by `0x8023D190`. The argument is deducible but not printed,
   so the body was left for a pass that can confirm the return register.
6. **`0x8024A6E0` (984).** It interpolates with four `sda2` float constants and
   an orientation kernel at `0x8025F6B0` whose output layout is still
   undocumented, and `0x8024B0E0` already covers the same table from the
   initialisation side.
7. **The large workers `0x802405B0` (2,660), `0x80241020`'s sibling
   `0x80241690` (1,124), `0x80244360` (980), `0x80245B40` (500), `0x802471E0`
   (428), `0x80249740` (808) and `0x8024ABD0` (1,088), plus
   `0x8024E180` (2,212)** were not attempted: they either mix GQR stores with
   undocumented helper tables or duplicate a body above at three times the size.

No address in this pass was already present in `ledger/functions.csv`; the 77
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (25 of 25; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes.
- `node tools/check-lane-csv.mjs wave96` prints `OK` for `wave96-os.csv`
  (25 rows).
