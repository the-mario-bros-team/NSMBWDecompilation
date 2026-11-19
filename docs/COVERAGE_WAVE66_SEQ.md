# Coverage - Wave 66 (audio::seq, resolver tail and sound-system archive consumer)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means every load, guard, store
and call is literal in that export; `medium` means at least one forwarded
argument, field index or role is inferred and marked as such in the function's
doc block.

One convention matters for this block. Ghidra prints the CodeWarrior
register-save stubs `0x802DCD64`/`68`/`6C`/`70`/`74` as function calls that
return a 64-bit value, and the bare-return stubs `0x802DCDB0`/`B4`/`B8`/`BC`/
`C0` as calls that discard their argument. Neither is a real callee: the first
group only preserves registers, so the "returned pair" is the caller's own
incoming `r3`/`r4`, and the second group only leaves its argument in `r3`. The
bodies below take those values from their own parameters and return the relayed
word directly instead of jumping through the stubs.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 28 | 7428 | 14 | 14 |

Bytes, High and Medium counts cover only the 28 bodies added in this pass.

## What this pass covers

- **Resolver tail, `0x80276CE0..0x80276F50` (3 bodies).** This finishes the
  id-resolver run that wave 52 stopped one entry short of. `0x80276CE0` is the
  `(id, sub-id)` two-word form, `0x80276F50` is the eight-byte descriptor
  decoder with the same `+0x101` version split as its siblings, and
  `0x80276E00` is the opposite direction: a 20-byte-node trie that consumes
  bits of the name string (`key >> 3` byte, `7 - (key & 7)` bit) until a node
  whose flag has `0x10000` set, then `strcmp`s the leaf name and returns the
  stored id. The last one is the piece that lets a caller go from a name back to
  an id without scanning.
- **Archive loading, `0x80275850` and `0x802770D0`.** These are the two ends of
  the same load path: `0x80275850` resolves a record, optionally resolves a
  second stream through `0x80276CE0`, and calls one of the two loader vtable
  slots; `0x802770D0` is the full loader, taking the sound system's mutex,
  opening the loader object returned by `0x80275A80`/`0x80275BA0`, reserving
  room in a caller-supplied sink and reading the payload whole or in
  `chunk`-sized pieces. `0x80277090` is its free half.
- **Sound-system object lifecycle, `0x802776B0..0x802779D0` (7 bodies).**
  `0x802776B0` builds the object: six vtable stores, three self-referential
  list sentinels, three mutex headers (`+0x54`, `+0x7C`, `+0xA4`) and its
  registration in the global list. `0x802779D0` reverses it.
  `0x802777D0`/`0x80277810` and `0x80277870`/`0x802778F0`/`0x80277970` are
  five deleting destructors of its sub-objects; three of them additionally run
  the intrusive-list teardown `0x80229030`.
- **Table sizing and carving, `0x80277A90..0x80278290` (8 bodies).** This is the
  cluster that explains the archive numbers. `0x80277CF0` computes the exact
  byte budget (100 bytes per bank, 8-byte id arrays, and the four blocks scaled
  by the seven-word info header as `0x4B4`, `0x10B4`, `0x234` and `0xD0`);
  `0x80277E40` is the `word[4] * 0xA000` term; `0x80277AB0`, `0x80277E90`,
  `0x80278290` and `0x802781C0` lay the tables out in
  the caller's buffer with explicit bounds checks, and `0x80277BC0` tears them
  down, unlinking every registry node that points into the block.
- **Bank registry lookups, `0x80278780..0x8027AFA0` (7 bodies).** `0x80278780`
  addresses one 100-byte slot; `0x80278790`/`0x80278920` are the two-column
  lookups that consult a bridge object, the archive and two cached tables
  before walking the sub-table; `0x80279D50` is the presence test; `0x8027A560`
  constructs a slot; and `0x8027AD90`/`0x8027AFA0` register a bank and attach
  descriptors under the sequence-manager mutex.

The pass extends the module header's data model with the sound-system layout
(`+0x08` vtables through `+0x21C` active loader) so the offsets used above are
readable without cross-referencing every body.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80275850` | `fn_80275850` | 552 | medium |
| `0x80276CE0` | `fn_80276ce0` | 288 | high |
| `0x80276E00` | `fn_80276e00` | 328 | medium |
| `0x80276F50` | `fn_80276f50` | 256 | medium |
| `0x80277090` | `fn_80277090` | 64 | high |
| `0x802770D0` | `fn_802770d0` | 1492 | medium |
| `0x802776B0` | `fn_802776b0` | 288 | medium |
| `0x802777D0` | `fn_802777d0` | 64 | high |
| `0x80277810` | `fn_80277810` | 64 | high |
| `0x80277870` | `fn_80277870` | 88 | high |
| `0x802778F0` | `fn_802778f0` | 88 | high |
| `0x80277970` | `fn_80277970` | 88 | high |
| `0x802779D0` | `fn_802779d0` | 184 | high |
| `0x80277A90` | `fn_80277a90` | 24 | medium |
| `0x80277AB0` | `fn_80277ab0` | 260 | medium |
| `0x80277BC0` | `fn_80277bc0` | 304 | medium |
| `0x80277CF0` | `fn_80277cf0` | 336 | medium |
| `0x80277E40` | `fn_80277e40` | 76 | medium |
| `0x80277E90` | `fn_80277e90` | 816 | medium |
| `0x802781C0` | `fn_802781c0` | 204 | high |
| `0x80278290` | `fn_80278290` | 300 | medium |
| `0x80278780` | `fn_80278780` | 16 | high |
| `0x80278790` | `fn_80278790` | 400 | medium |
| `0x80278920` | `fn_80278920` | 400 | medium |
| `0x80279D50` | `fn_80279d50` | 68 | high |
| `0x8027A560` | `fn_8027a560` | 140 | high |
| `0x8027AD90` | `fn_8027ad90` | 136 | high |
| `0x8027AFA0` | `fn_8027afa0` | 104 | high |

## Deliberate skips

- **`0x80277050`.** The constructor stores `+0x18`, zeroes `+0x21C` and then
  calls `0x801B2E20`, the mutex initialiser, but Ghidra drops the argument
  register and no other function in the range reveals which mutex of the object
  it receives. Its free half `0x80277090` is translated; the constructor itself
  is left out rather than guessing an offset.
- **`0x802700F0..0x80270C20`** (`0x802700F0`, `0x80270100`, `0x80270110`,
  `0x80270310`, `0x80270920`, `0x80270AD0`, `0x80270C20`). These operate on the
  four-slot JAS voice band at `0x803FC9C8` with an `unaff_r13` sentinel, and
  `ledger/functions.csv` already records the surrounding `0x80270000` block as
  `game::player`. They belong to the AX/voice and player lanes, not here.
- **`0x80271E30`, `0x802720A0`, `0x80273560`, `0x80273DD0`, `0x802741E0`,
  `0x80274320`, `0x80274480`, `0x80274630`, `0x802746D0`, `0x80274A10`,
  `0x80274B90`, `0x80274CE0`.** These remain from wave 45/52's skip list: large
  dispatchers or paired-single audio-rate arithmetic whose fixed-point scaling
  cannot be reconstructed honestly without the GQR instructions.
- **`0x802783C0`** (956 bytes). The three-way 128-bucket rebuild of the three
  registries is understood — `0x802DC9A0`/`0x802DCA98` build and drop a
  128-entry table of 0xC-bytes entries, `0x802290C0`/`0x80229160` move nodes —
  but it repeats the same block three times and would push this lane past the
  requested 25-30 bodies. Left for a later pass.
- **`0x80278xxx` beyond the two lookups** (`0x80279510`, `0x80279880`,
  `0x80279A30`, `0x80279B70`, `0x80279CE0`, and the `0x8027A1xx..0x8027A9xx`
  group loaders) and the whole **`0x8027Bxxx` manager block** are untouched.
  The `0x8027Bxxx` block owns the sequence-manager singleton and the timer and
  bank-manager classes; those belong to an `audio::ax`/manager pass, and the
  `0x80277xxx`/`0x80278xxx` functions translated here touch them only through
  the documented `0x8027B5F0` singleton calls.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors and warnings for
  `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave66` reports `OK` with this CSV clean.
