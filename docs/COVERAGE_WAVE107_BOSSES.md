# Coverage - Wave 107 (Bosses)

## How to read this file

- **Scope.** Eight bodies added to `src/game/bosses.rs` for the boss-actor REL
  `d_en_bossNP` (REL module 4), all of them in the `0x80C30000` bucket.
  Earlier work is documented in `docs/COVERAGE_WAVE34_BOSSES.md` and the
  wave-2 through wave-34 boss coverage files; this note does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x8006D090`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence.** The Ghidra C export path is cited in every row and every
  `# Original` block. For these eight entries the C export stops at
  `halt_baddata()`, so each body was decoded instruction by instruction from
  the intact raw REL text (`work/rels/d_en_bossNP.rel`, text section 1, file
  offset `address - 0x80C00000`) and each branch target was read from the
  relocated branch field of `work/rels/elf/d_en_bossNP.elf` (REL24 entries in
  the module-0 import stream), then cross-checked against the `main.dol`
  exports of the callees.
- **Status:** all eight rows are `translated`; every instruction of each slab
  is represented (the thunks are one branch each).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 8 | 328 | 6 | 2 |

Cumulative after this pass: 764 functions in `src/game/bosses.rs` (763
`pub unsafe fn` plus the private empty hook `fn_80c001e0`). The 328 bytes
are `28 + 4 + 72 + 72 + 4 + 4 + 72 + 72`, which matches the `328`-byte total
that `docs/GAP_REPORT.md` recorded for this bucket before the pass, and every
size was re-read from
`work/ghidra/rel-d_en_bossNP/export/functions.tsv`.

## What this pass covers

This pass takes the eight entries of bucket `0x80C30000` that were still
untranslated. They are exactly the eight bucket members whose Ghidra export
truncates at `halt_baddata()`; the other 153 entries of the bucket were already
translated in earlier waves and were not touched.

Three clusters:

1. **One spawn helper (`0x80C32660`, 28 bytes).** Seven instructions:
   `mr r4,r3`, then `r3 = 0x12F`, `r5 = this + 0x788`, `r4/r6/r7 = 0`, then
   `b 0x80064610`. The `main.dol` export of `0x80064610` takes five arguments
   and forwards the first four to `0x8006CA80`, so this is the same
   `(id, resource, out, 0, 0)` factory shape already translated in
   `game::bases`.
2. **Four 72-byte field/binder initializers (`0x80C389B0`, `0x80C38A00`,
   `0x80C3DFE0`, `0x80C3E030`).** Each one copies four float words from a
   module-image table into `+0x164`, `+0x168`, `+0x16C` and `+0x170`, then
   tail-branches to `0x8006D090` with `r3 = this + 0x1EC`, `r4 = this` and
   three static descriptor addresses in `r5`/`r6`/`r7`. The `main.dol` export
   of `0x8006D090` stores its third, fourth and fifth arguments at
   `+0x08/+0x0C/+0x10` of its first argument and links the record back to the
   owner through `+0x14..+0x1C`, which is why the three words are described as
   descriptors bound into the `+0x1EC` record.
   - `0x80C389B0`: floats `0x80C6632C..0x80C66338`, descriptors
     `0x80C584D0`/`0x80C584C0`/`0x80C584E8`.
   - `0x80C38A00`: floats `0x80C6633C..0x80C66348`, descriptors
     `0x80C58500`/`0x80C584F4`/`0x80C58510`.
   - `0x80C3DFE0`: floats `0x80C675AC..0x80C675B8`, descriptors
     `0x80C68758`/`0x80C68748`/`0x80C68770`.
   - `0x80C3E030`: floats `0x80C675BC..0x80C675C8`, descriptors
     `0x80C68788`/`0x80C6877C`/`0x80C687A0`.
3. **Three four-byte tail branches (`0x80C336F0`, `0x80C3AF50`,
   `0x80C3AF60`).** Each raw slab is a single `b` instruction.
   `0x80C336F0` branches to `0x80663290`, which is already translated as
   `crate::game::bases::fn_80663290`; that function reads no argument register,
   so the thunk forwards by calling it directly and returns its word.
   `0x80C3AF50`/`0x80C3AF60` branch to `0x800AE500`/`0x800AE510`, which have no
   recovered bodies in the `main.dol` inventory; they are modelled as
   no-argument calls, and the two other call sites of `0x800AE500` in this
   module (`fn_80c02b20`, `fn_80c102a0`) are also exported as argument-less
   calls.

The `# Original` blocks therefore still cite the Ghidra export (the required
evidence path), while the body text states that the instructions were recovered
from the raw REL because the export truncates.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C32660` | `fn_80c32660` | 28 | high |
| `0x80C336F0` | `fn_80c336f0` | 4 | high |
| `0x80C389B0` | `fn_80c389b0` | 72 | high |
| `0x80C38A00` | `fn_80c38a00` | 72 | high |
| `0x80C3AF50` | `fn_80c3af50` | 4 | medium |
| `0x80C3AF60` | `fn_80c3af60` | 4 | medium |
| `0x80C3DFE0` | `fn_80c3dfe0` | 72 | high |
| `0x80C3E030` | `fn_80c3e030` | 72 | high |

## Deliberately skipped

- **Nothing in this bucket.** No `functions.tsv` entry in `0x80C30000` remains
  untranslated: the eight rows above are the whole bucket remainder, and all
  eight could be reconstructed from the raw REL text.
- **Six `halt_baddata()` exports outside this bucket remain untranslated**
  (`0x80C0B510`, `0x80C228A0`, `0x80C4C160`, `0x80C4FDE0`, `0x80C512F0`,
  `0x80C51430`). They belong to the `0x80C00000`, `0x80C20000`, `0x80C40000`
  and `0x80C50000` buckets, which were outside this lane's assigned remainder,
  so they were not touched. Their raw text is intact in the same REL and is
  recoverable by the same method.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_en_bossNP/export/functions.tsv`; the eight sizes are 28, 4,
  72, 72, 4, 4, 72 and 72.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_en_bossNP/export/`.
- Each body was decoded from the raw REL text at text offset
  `address - 0x80C00000`; the four 72-byte bodies agree instruction-for-
  instruction with the partial Ghidra export where it overlaps (the four
  `lfs`/`stfs` pairs).
- Every branch target was computed from the relocated branch field in
  `work/rels/elf/d_en_bossNP.elf` and checked against the `main.dol` exports
  `80064610_FUN_80064610.c` and `8006d090_FUN_8006d090.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `node tools/check-lane-csv.mjs wave107` reports `OK`, with no problem naming
  `wave107-bosses.csv`.
- `cargo check --message-format=short` reports no error in
  `src/game/bosses.rs` (the only errors are in other lanes' files, which were
  being written in parallel).
- The module header count was updated from 756 to 764 and the wave-107 bullet
  and "start here" entry were added.
- No files outside `src/game/bosses.rs`, this document and
  `ledger/agents/wave107-bosses.csv` were edited.

## Module state after this pass

`d_en_bossNP` now has 764 functions (763 `pub unsafe fn` bodies plus the
private empty hook). Every entry of the `0x80C30000` bucket is translated.
Across the whole module six truncated exports remain (listed above); a
re-decompile that resolves them is the only way past 764 without the raw-text
method used here.
