# Coverage - Wave 112 (audio::seq)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count is the size column
of `work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the
Ghidra C export the body was reconstructed from. `high` means every load, guard,
store and call is literal in that export; `medium` means at least one field role,
argument source or callee identity is inferred and marked as such in the
function's doc block; `low` means only the instruction sequence is certain.

This pass closes the module's `0x80270000` bucket. The bucket held 276
functions; 274 were already in `ledger/functions.csv`, and the two that remained
were exactly the entries wave 109 had documented as skips. Both are landed
below, so the bucket now has no untranslated function.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 2 | 1832 | 0 | 2 |

## What this pass covers

- **Per-track mixer refresh, `0x80273560` (1400 bytes, 1 body).** The routine
  [`fn_80272e30`] calls while tearing a track down. Under the sequence mutex it
  early-outs on the track's `+5` enable byte and on an empty `+0xC8` voice list,
  then derives the whole mixer state from the track and its `+0xC4` element:
  the four envelope counters (`+0x70`, `+0x76`, `+0x7C`, `+0x82`, each a `from`
  byte, a `to` byte, a signed `i16` limit and a signed `i16` position), the
  gain (three squared `byte / 127` levels times two linear gains), the pitch and
  surround offsets (`/63`, clamped to `-1..1` and `0..2`), the spread byte
  (`+0x8A * trim / 32`), the element-overridden pan byte and value, and the
  three running values plus four words read through `0x80265F60` /
  `0x80265FA0`. Every voice of the list then receives nineteen fields and the
  element's `0x18`-byte row table through `0x802817D0`.
  The nineteen stores, the four interpolations and both clamps are literal. The
  only inference is the argument source: the export declares the routine `void`
  and takes its object from `iVar10 = FUN_802dcd70()`, whose own export only
  spills `r26..r31` — the ABI register-save thunk — so that word is the
  untouched incoming `r3`, which the in-module call site fills with the track.
- **Directional cone gain, `0x80274480` (432 bytes, 1 body).** The fourth mix
  helper [`fn_80274ce0`] calls (output word `out + 4`). `this + 0x24` is the
  cone limit; zero means full gain `1.0`. Otherwise the delta from the node's
  cached position `+0x30..+0x38` to the request block's three-float position is
  measured by `0x801C14C0` and normalised, scaled by the block's `+0x1E` byte
  over 32, and reduced to two negated dot products: one against the block's
  axis at `+0x0C..+0x14`, one against the node's normal at `+0x3C..+0x44`. A
  node sitting on the listener substitutes the two vector lengths for those
  dots instead of dividing by zero. The gain is then a two-edge ramp:
  `0.0` above the limit, `(limit - normal) / (limit - axis)` between the two
  edges, and `65535.0` when the axis edge is behind the listener.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80273560` | `fn_80273560` | 1400 | medium |
| `0x80274480` | `fn_80274480` | 432 | medium |

## Deliberate skips

None. Both remaining addresses in the bucket are reconstructed.

The two reasons wave 109 gave for skipping them were re-checked against the
exports and do not hold:

- `0x80273560`: the twenty-two `f21..f31` / `ps21_1..ps31_1` stores are the
  epilogue register save (Ghidra emitted them at the top of the body, and the
  `Removing unreachable block` warnings list their addresses). No remaining
  load in the export reads those stack slots, so they carry no program state.
- `0x80274480`: the three-way load ladder is not runtime branching, it is one
  `psq_l` per field printed once per GQR quantisation class Ghidra cannot
  resolve statically. The `f32` arm is the one the neighbouring helpers
  [`fn_802741e0`] and [`fn_80274630`] take for the same loads in the same
  structures, so the delta vector and the two dot products are plain floats.

## Verification

- Every address and byte size matches `functions.tsv` (1400 / 432).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`; the two candidates were
  re-derived from the inventory, not copied from prose.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave112` reports no problem for
  `wave112-seq.csv`.
