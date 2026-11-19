# Coverage - Wave 110 (resource::registry)

## How to read this file

Every row below is one real `pub unsafe fn` body in
`src/resource/registry.rs`. The address and byte count come from
`work/ghidra/main-gb/export/functions.tsv` (re-read this pass, not copied from
earlier prose); the evidence path is the Ghidra C export that was read before
the body was written. `confidence` matches the `# Confidence` line of the
function's doc block.

This pass is the last one for the `0x80160000` page. Wave 103 closed nineteen
of the twenty-three wave-97 skips and left four addresses; they are the four
bodies below, so the bucket now has no untranslated address.

Three reading rules made the four possible, and a later pass should keep them:

- **Tail branches carry the arguments.** `0x801611C0` is not called by `bl`
  from `0x80160E80`; it is reached by `b`, with `f1`/`f2` holding that
  function's two `fsubs` results. Ghidra types the pair as `double`, but the
  operator that produces them is single precision. The body therefore takes
  two `f32`s, and every `FUN_80161180(...)` call the export prints with one or
  zero arguments is a two-register call whose second operand the decompiler
  dropped; the listing names it in each of the eight branches.
- **`unaff_r13` is always the same SDA page.** `r13` is
  `SDA_BASE = 0x8042F680`, fixed by the ABI and never assigned in these
  bodies. Ghidra prints `unaff_r13` because the prologue does not write it.
  Every offset was resolved to an absolute address and is a named constant in
  the file. The same rule holds for `unaff_r2` (`SDA2_BASE = 0x80433080`).
- **A slot's meaning comes from its other readers.** `r13 - 0x5208` is written
  by wave 44's `fn_8016f640` and read by `fn_8016f660`, which just store and
  return a word. `fn_8016f220` compares it with `1` and `3` and wraps a
  companion counter at it, so the slot is a small count. No comment was
  rewritten in the older bodies; `G_8016F640_INSTANCE` keeps its `// unknown:`
  note, and the wave-110 body says which use it relies on.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 4 | 2404 | 1 | 3 |

Address range: `0x801611C0..0x8016FA10`.

## What this pass covers

- **Quadrant table** (`0x801611C0`, 1 body). The two `FLT_EPSILON`/`0.0f`
  guards, the eight octant branches and the exact operand order of every
  `0x80161180` lookup, with the two `cror` guards folded into the plain
  comparison.
- **Box rebuild** (`0x8016DEC0`, 1 body). The eight corners from the six
  extents (`y` = `f1`/`f2`, `x` = `f3`/`f4`, `z` = `-f5`/`-f6`), the
  `param_6 / param_5` taper path, the ten `0x802385D0` face directions around
  the eight-iteration in-place `0x801C1160` loop, and the `0x80238690`
  bounding box at `+0x78`.
- **Player frame** (`0x8016F220`, 1 body). The frame counter, the manager's
  `+0x10 -> +0x08` virtual slot, the per-player active bit at `+0xB1C`, the
  six-float motion record (`x`, `y`, `dx`, `dy`, `ddx`, `ddy`), the retire
  path, the mode/slot selector and the published "current player" pointer.
- **Text page** (`0x8016FA10`, 1 body). The two-pass glyph draw with both
  colour words, the signed-halfword position conversion, the `+0x43` alpha
  byte, the `+0x44` `0.8 * scale` field, the `0.1 * scale` shift and the
  `+0x04` countdown into `0x8016FE40`.

## Functions

| address | name | role | bytes | status | confidence | evidence |
|---|---|---|---|---:|---|---|---|
| `0x801611C0` | `fn_801611c0` | quadrant angle from two direction components | 436 | translated | high | `work/ghidra/main-gb/export/801611c0_FUN_801611c0.c` |
| `0x8016DEC0` | `fn_8016dec0` | rebuild a box's corners, face directions and bounds | 880 | translated | medium | `work/ghidra/main-gb/export/8016dec0_FUN_8016dec0.c` |
| `0x8016F220` | `fn_8016f220` | advance the four-player table one frame | 484 | translated | medium | `work/ghidra/main-gb/export/8016f220_FUN_8016f220.c` |
| `0x8016FA10` | `fn_8016fa10` | draw and age one text page | 604 | translated | medium | `work/ghidra/main-gb/export/8016fa10_FUN_8016fa10.c` |

All four rows are `translated`; none is a stub and none is `partial`.

## What is deliberately not modelled

These are reductions inside otherwise complete bodies, not missing code:

- **NaN ordering.** The `fcmpo` + `mfcr` + `cror` sequence the compiler emits
  before each of the eight tests compares `cr0[LT]` after folding the unordered
  case. The bodies use the plain `<`/`<=`, which matches every ordered input.
  The doc blocks say so.
- **Dead stack stores.** `0x8016F220` copies `ddx`, `ddy`, `delta x`, `delta
  y`, `x` and `y` into its outgoing-argument area at `r1 + 8..0x28` and never
  reads them back; `0x8016FA10` stores the `page + 4` word at `r1 + 0x58` the
  same way. Nothing observes those slots, so they are not reproduced and each
  body carries a comment naming the omission.
- **The virtual slot's prototype.** `0x8016F220` calls
  `(*(manager + 0x10) + 8)()` with the old frame count still in `r4`. Whether
  the callee reads that register cannot be decided from one call site, so the
  body calls it as `call0` and the doc block records the fact.

## Verification

- Every address and byte size matches `functions.tsv` (re-read this pass).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/registry.rs`; the file now has 423 bodies.
- No address was already in `ledger/functions.csv`; the four rows are exactly
  the bucket's remaining addresses, and the bucket (`0x80160000..0x8016FFFF`,
  421 functions) now has no untranslated entry.
- `cargo check --message-format=short` reports no diagnostic for
  `src/resource/registry.rs`.
- `node tools/check-lane-csv.mjs wave110` reports
  `wave110-registry.csv: 4 rows` and an overall `OK`.
