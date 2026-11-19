# Coverage - Wave 109 (audio::seq)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count is the size column
of `work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the
Ghidra C export the body was reconstructed from. `high` means every load, guard,
store and call is literal in that export; `medium` means at least one field role
or callee identity is inferred and marked as such in the function's doc block;
`low` means only the instruction sequence is certain.

This lane closed the module's `0x80270000` bucket. The bucket held 276
functions; 270 were already in `ledger/functions.csv`, and the six that remained
were the six entries wave 104 had left open. Four of them are below; the other
two are the paired-single skips documented at the end.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 4 | 2504 | 0 | 4 |

## What this pass covers

- **Voice start-up, `0x80273DD0` (1032 bytes, 1 body).** The largest of the six.
  It takes the sequence-manager mutex, converts the request level as
  `level * this[+0x89] / 127`, and re-uses or destroys the voice already at
  `this + 0xC8` depending on the `+0x27` flag. Without a live voice it fills the
  nine-word request block that starts from the static template at `0x802FA788`
  (`this + 0x54`, the id, the converted level, `value` or `-1`,
  `this + 0x8B`, `this[+0xC4][+0xE9] + this[+0x8D]`, `this[+0xC4][+0xE4]`, the
  template's callback word, `this`) and hands it to `0x80272340`. The accepted
  voice retires the same-key nodes of the `+0xC8` list, is spliced in at
  `+0xD4`, then receives the five guarded band setters, the two `0x802688E0`
  key-velocity calls and the `+0xA0..+0xA4` interpolation. The export is fully
  integer/scalar, so the whole control flow and every store are literal.
- **Node distance ramp, `0x802741E0` (316 bytes, 1 body).** One of the four mix
  helpers `fn_80274ce0` calls. The node's cached position at `+0x30..+0x38` is
  subtracted from the three-float `vector`, the difference is measured by
  `0x801C14C0`, and `node[+0x4C]`/`node[+0x50]` turn it into the level. The byte
  at `vector + 0x1C` picks the power curve (`0x802E861C`, the `pow` thunk), the
  linear curve, or no attenuation; the second output negates
  `(1 - level) * this[+0x1C]`. The three `psq` instructions in the original are
  a pair load, a pair subtraction and a single-element load/store — all plain
  `f32` elements of the same struct, which the export prints as the float path.
- **Node matrix fade, `0x80274320` (352 bytes, 1 body).** The other mix input.
  `0x801C1160` transforms the node position through `vector` into a local
  `0x30`-byte block, `0x801C14C0` measures it, and the `0x802610B0` mode selects
  `fn_802746d0` (modes 1/2, ten-segment fade) or `fn_80274a10` (mode 0,
  seven-segment fade); any other mode writes the small-data zero to both
  outputs. The `stfd`/`psq_st` ladder that opens and closes the function is only
  the `f25..f31` register save/restore — the earlier pass read it as a
  quantisation problem, but the stacked values are never read.
- **Per-element voice burst, `0x8027E080` (804 bytes, 1 body).** The routine the
  wave-104 element ticks drive. It early-outs on the enable byte at `spec + 0`,
  derives the gain from `spec + 8 / 0.0078125` and `spec + 0x30`, the level from
  `spec + 0x34` and the `(spec + 9 - 63) / 63` (or `- 64`) term, gathers three
  running values through `0x80265F60` and four words through `0x80265FA0`, then
  applies ten `0x802814A0..0x80281710` setters, re-sends both arrays and walks
  the `obj + 0x50` row table (`0x18` bytes per row, count at `obj + 0x814`)
  while the sound state lock is held. Its `f24..f31` prologue/epilogue is again
  a pure register save.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80273DD0` | `fn_80273dd0` | 1032 | medium |
| `0x802741E0` | `fn_802741e0` | 316 | medium |
| `0x80274320` | `fn_80274320` | 352 | medium |
| `0x8027E080` | `fn_8027e080` | 804 | medium |

## Deliberate skips

- **`0x80273560` (1400 bytes).** Its export is dominated by `in_f21..in_f31` and
  `in_ps21_1..in_ps31_1`: eleven paired-single pairs that the ABI never
  materialises as arguments (the four in-module call sites pass only `this`).
  The body rewrites the voice list from those registers plus a dozen `r2`
  small-data words. Any Rust signature would invent eleven inputs, so no body is
  claimed. The existing call sites keep the documented address stub.
- **`0x80274480` (432 bytes).** This one is *live* paired-single code: a
  rotated dot product (`ps_sub`, `ps_mul`, `ps_madd`, `ps_sum0`, and a
  `psq_l ...,1,0` single load) over the normalised triple and two `0x3C..0x44`
  vectors, followed by a two-edge gain formula (`65535.0` when the second edge
  is behind the listener). The element rotation depends on `ps_sum0`/`ps_mul`
  element semantics that Ghidra renders as `CONCAT` noise and drops from the
  scalar path, so the reduction would be a guess. The `0x80274480` address stub
  in `fn_80274ce0` is unchanged.

## Verification

- Every address and byte size matches `functions.tsv` (1032 / 316 / 352 / 804).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`; the six candidates were
  re-derived from the inventory, not copied from prose.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave109` reports no problem for
  `wave109-seq.csv`.
