# Coverage - Wave 97 (`runtime::os`, `0x80250000` bucket)

## How to read this file

This page records wave 97 of the `main.dol` reconstruction for the
`runtime::os` lane: the model-instance loaders, the transform kernels and the
vertex-format object of the `0x80250000` bucket. The bucket holds 163 inventory
entries; when the lane started 103 were already in `ledger/functions.csv` and
60 were untranslated. 26 of those 60 are reconstructed here.

- **Status** - `translated` means every guard, store and call printed in the
  Ghidra export has a counterpart in `src/runtime/os.rs`. All 26 rows are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export.
  `medium` means the flow and the offsets are explicit but at least one callee
  role, field meaning or parameter is `// unknown:` / `// inferred:` in the
  body. `low` marks a body whose instruction sequence is certain but whose call
  shapes had to be interpreted.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes were re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave97-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 26 | 7,648 | 13 | 12 | 1 |

Status split: 26 `translated`, 0 `partial`, 0 `stub`.

## What this pass covers

- **Cluster A - model-instance loaders, `0x80251E50`..`0x80252440` (5 bodies).**
  `0x80251E50` fills the fourteen-word instance block from a shared offset table
  (eleven `base + index * stride` slots with strides `0x104`, `0x64`, `0x248`,
  `0x28`, `0x08`, `0x0C`, `0x20`, `0x80`, `0x40`, `0xA0`, `0x200`, plus the
  three-word tail guarded by bit 0 of the table's first word). The four walkers
  iterate either the eight-byte tag array terminated by a `0x01` byte
  (`0x80252020`, `0x80252160`) or a counted run of `0x0C`-stride records
  (`0x80252330`, `0x80252440`), and hand a five-word key to `0x80251930` before
  the `0x8024E180` emitter.
- **Cluster B - transform dispatch tables, `0x802556B0`/`0x802562B0`/`0x80256F90`
  (3 bodies).** Each selects one of seven kernels with `(format >> 1) & 7` and
  returns `0` for selector `7`. The six tables are contiguous seven-entry
  arrays at `0x802F8EF0`, `0x802F8F0C`, `0x802F8F28`, `0x802F8F44`,
  `0x802F8F60` and `0x802F8F7C`; each wrapper owns one pair.
- **Cluster C - transform kernels, `0x80254910`..`0x80256990` (8 bodies).**
  Three `0x30`-byte emitters (`0x80254A70`, `0x80255750`, `0x80256350`) that
  branch on record flags 1/2/5, scale the translation column (or the vertex
  words) and republish the rewritten format; three two-column matrix builders
  (`0x80254EC0`, `0x80255AA0`, `0x80255BC0`); the 3x4 builder/updater
  `0x80254910`; and the in-place 2x4 rotation `0x80256990`. All of them take
  their angle from `src + 0x08` and their sine/cosine pair from `0x80237CA0`.
- **Cluster D - vertex-format object, `0x8025A380`..`0x8025C250` (5 bodies).**
  The allocator `0x8025A380` sizes the object from the model record
  (`+0x68` vertices, `+0x6C` second array, `+0x6D` extra block, `+0x70`
  format-word index) and calls `0x8025C030`; `0x8025B420` picks one matrix by
  key and index; `0x8025BB10` detaches one of the five owned slots;
  `0x8025BC90` returns one slot after a `+0x38` virtual call; `0x8025C250` is
  the destructor.
- **Cluster E - render objects and node entries, `0x80257E60`..`0x8025EFB0`
  (5 bodies).** `0x802589E0` allocates the `0x288C`-header object
  (`0x8034E828`) and `0x802593E0` initialises it (`0x8034E858`, the `0x2878`
  sub-object, `0x20` matrix slots at `+0xF8` and `0x20` quads at `+0x2278`);
  `0x80257E60` is the two-bit child traversal; `0x80259190` reports one
  transformed sub-node; `0x8025EFB0` constructs the `align4(payload) + 0xF4`
  node with the `0x8034E980` vtable.

## What the pass adds beyond the raw bodies

1. **The five-slot family at `+0x124`..`+0x134` is closed.** `0x8025BB10`
   (detach), `0x8025BC90` (get) and `0x8025C250` (destructor) read exactly the
   five words that `0x8025C030`'s constructor zeroes, and `0x8025B420` reads
   the two matrix arrays those slots publish (`+0xEC`, `+0xF4`). The layer
   geometry is `u8(+0x101) * align32(u16(+0x102) * 0x30)`.
2. **`0x801C0540`'s argument order is settled.** Its own export writes all
   twelve results through its third parameter, so the order is `(a, b, dst)`,
   not `(dst, a, b)`. The Wave 91 body `fn_802570e0` documented the opposite
   order; its code was already faithful, and only its parameter name and doc
   were corrected in this pass.
3. **The six gather-format tables form one contiguous array.** `0x802F8EF0`,
   `0x802F8F0C`, `0x802F8F28`, `0x802F8F44`, `0x802F8F60` and `0x802F8F7C` are
   seven words apart, i.e. six tables of seven entries. Two of the six
   (`0x802F8F0C`/`0x802F8EF0` and `0x802F8F44`/`0x802F8F28`) differ in the
   `LAB`/`FUN` typing Ghidra gave the entries, one pair
   (`0x802F8F7C`/`0x802F8F60`) does not.
4. **The `0x802DCDxx` entries are register handoffs.** Every entry used in
   this section is four bytes wide in `functions.tsv` yet stores seven to
   fourteen registers. Waves 92 and 96 already modelled them as opaque
   64-bit-returning calls; this pass keeps that convention and names the half
   each call site reads, because `0x8025A380`/`0x802589E0`/`0x8025EFB0` use the
   low half as a size-out pointer while `0x80255750`/`0x80256350`/`0x80254A70`
   use it as the vertex write pointer.
5. **The record layout of the emitters is fixed.** `+0x00` flags, `+0x04`
   `+0x08` `+0x0C` the three vertex words, `+0x1C` the matrix, and
   `+0x28`/`+0x38`/`+0x48` the three vector words; the counted and tag-driven
   loaders read the same three fields as `u16` at `+0x06`/`+0x08`/`+0x0A` of
   their `0x0C`-stride records.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80251E50` | `fn_80251e50` | 456 | high |
| `0x80252020` | `fn_80252020` | 320 | medium |
| `0x80252160` | `fn_80252160` | 452 | medium |
| `0x80252330` | `fn_80252330` | 272 | medium |
| `0x80252440` | `fn_80252440` | 352 | medium |
| `0x80254910` | `fn_80254910` | 348 | high |
| `0x80254A70` | `fn_80254a70` | 360 | medium |
| `0x80254EC0` | `fn_80254ec0` | 220 | high |
| `0x802556B0` | `fn_802556b0` | 156 | high |
| `0x80255750` | `fn_80255750` | 404 | medium |
| `0x80255AA0` | `fn_80255aa0` | 200 | high |
| `0x80255BC0` | `fn_80255bc0` | 200 | high |
| `0x802562B0` | `fn_802562b0` | 156 | high |
| `0x80256350` | `fn_80256350` | 436 | medium |
| `0x80256990` | `fn_80256990` | 292 | high |
| `0x80256F90` | `fn_80256f90` | 156 | high |
| `0x80257E60` | `fn_80257e60` | 280 | medium |
| `0x802589E0` | `fn_802589e0` | 312 | medium |
| `0x80259190` | `fn_80259190` | 236 | low |
| `0x802593E0` | `fn_802593e0` | 252 | medium |
| `0x8025A380` | `fn_8025a380` | 404 | medium |
| `0x8025B420` | `fn_8025b420` | 224 | high |
| `0x8025BB10` | `fn_8025bb10` | 376 | high |
| `0x8025BC90` | `fn_8025bc90` | 224 | high |
| `0x8025C250` | `fn_8025c250` | 288 | high |
| `0x8025EFB0` | `fn_8025efb0` | 272 | medium |

## Deliberate skips

The other 34 untranslated addresses in the bucket were left alone. Eight were
read for this pass and rejected:

1. **`0x802525A0` (356) and `0x80252710` (552).** They are the pair that ties
   the four loaders together, but the count they exchange is written through
   the *high* half of the `0x802DCD60` pair in one body and read from a stack
   word in the other. Every other call site of that entry in this bucket uses
   the low half as the size-out pointer, so the count's provenance is not
   provable from the exports.
2. **`0x8025C030` (536).** The vertex-format constructor. Ghidra prints
   `FUN_80257100()` with no arguments even though that callee takes
   `(object, heap)` everywhere else, and the object pointer itself comes from
   the undocumented `0x802DCD60` half. The ten-argument signature is known from
   its caller [`fn_8025a380`], but the body's own two initialiser calls cannot
   be written faithfully yet.
3. **`0x80253ED0` (476).** Calls `FUN_802544F0()` with no arguments and then
   passes six `auStack_*` slots to `0x80253E60` whose assignment order the
   export does not pin down; the colour/double conversions are also folded
   through the `0x43300000` bias.
4. **`0x80259760` (896) and `0x80259AE0` (896).** The introsort driver and its
   partner: the recursion state is carried in a single `CONCAT44` word whose
   halves Ghidra reinterprets between iterations (`CONCAT44(puVar10,puVar12)`
   then `CONCAT44(puVar12,puVar11)`), so the two live ranges cannot be
   separated honestly from the C.
5. **`0x80259E60` (256).** The comparator used by that sort. Its four swap
   branches are literal, but it calls through a bare `code *` argument with
   the two operands in both orders, so it belongs with the sort driver above.
6. **`0x8025FAD0` (348) and `0x8025FC30` (288).** Both take the 64-bit register
   pair and use *both* halves as object pointers (a source and a destination
   slot array) while the stores themselves are GQR-quantised. No translated
   body in this file models that store form yet.

The remaining 25 addresses were not opened this pass. They are the large
workers of the bucket (`0x80250230`, `0x80250710`, `0x80250E70`, `0x80251440`,
`0x80251930`, `0x802529F0`, `0x80252FB0`, `0x80253570`, `0x802544F0`,
`0x80257A40`, `0x80257FC0`, `0x80258CE0`, `0x80258F10`, `0x8025A520`,
`0x8025A7C0`, `0x8025B500`, `0x8025C660`, `0x8025CA20`, `0x8025D5B0`,
`0x8025E060`, `0x8025E400`, `0x8025E8B0`, `0x8025EC70`, `0x8025F6D0` and
`0x8025FE30`); none of them was read, so no claim is made about them here.

No address in this pass was already present in `ledger/functions.csv`; the 103
translated entries in the bucket were filtered out before any body was written.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (26 of 26; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/runtime/os.rs`; the remaining build diagnostics belong to other lanes.
- `node tools/check-lane-csv.mjs wave97` prints `OK` with `wave97-os.csv`
  counted at 26 rows.
