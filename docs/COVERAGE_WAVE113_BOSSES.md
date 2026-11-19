# Coverage - Wave 113 (Bosses)

## How to read this file

- **Scope.** Two bodies added to `src/game/bosses.rs` for the boss-actor REL
  `d_en_bossNP` (REL module 4). They are the last two untranslated entries of
  the `0x80C40000` bucket, and both are entries whose Ghidra C export stops at
  `halt_baddata()`. Earlier work is documented in
  `docs/COVERAGE_WAVE107_BOSSES.md` and the wave-2 through wave-34 boss
  coverage files; this note does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x8016F640`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence.** The Ghidra C export path is cited in every row and every
  `# Original` block. Because both exports truncate, each body was decoded
  instruction by instruction from the intact raw REL text
  (`work/rels/d_en_bossNP.rel`, text section 1, file offset
  `address - 0x80C00000`) and each branch target was read from the relocated
  branch field of `work/rels/elf/d_en_bossNP.elf`, then cross-checked against
  the `main.dol` exports of the callees.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 2 | 20 | 1 | 1 |

Cumulative after this pass: 766 functions in `src/game/bosses.rs` (765
`pub unsafe fn` plus the private empty hook `fn_80c001e0`). The 20 bytes are
`4 + 16`, and both sizes were re-read from
`work/ghidra/rel-d_en_bossNP/export/functions.tsv`.

## What this pass covers

This pass takes the two remaining untranslated entries of bucket `0x80C40000`
(all 130 other bucket members were already translated):

1. **The `0x800AD4F0` thunk `0x80C4C160` (4 bytes).** The raw slab is a single
   `b 0x800AD4F0`. `0x800AD4F0` has no recovered body in the `main.dol`
   inventory, so Ghidra truncates the export at `halt_baddata()`. The only
   in-module branch to the thunk is at `0x80C16EA4`, inside a small unlisted
   helper that sets `r3` from its own first argument and nothing else, so the
   body is modelled as `0x800AD4F0(this)` with one forwarded argument. Marked
   `medium` because the callee's full argument list cannot be proven.
2. **The `+0x2E1` clear / mode reset `0x80C4FDE0` (16 bytes).** Four
   instructions: `li r0,0`, `stb r0,0x2E1(r3)`, `li r3,0`, `b 0x8016F640`.
   The third instruction discards the incoming actor pointer, so the tail call
   always receives zero. `0x8016F640(0)` writes the `r13 - 0x5208` mode word
   (`0x8042A478`) and, because the value is zero, runs `0x8016F550`, the
   four-iteration clear of the `0x18`-byte records at `0x80377D08`. This is the
   not-ready fallback of the phase selector `0x80C504E0`, whose export stores
   phase `7` after calling `0x80C4FDE0`; the sibling `fn_80c4ff10` sets the same
   byte to one when it arms the phase. Marked `high` because all four
   instructions and both `main.dol` callee bodies are literal.

The `# Original` blocks still cite the Ghidra export (the required evidence
path), while the body text states that the instructions were recovered from the
raw REL because the export truncates.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C4C160` | `fn_80c4c160` | 4 | medium |
| `0x80C4FDE0` | `fn_80c4fde0` | 16 | high |

## Deliberately skipped

- **Nothing in this bucket.** `0x80C4C160` and `0x80C4FDE0` are the whole
  `0x80C40000` remainder, and both could be reconstructed from the raw REL text.
- **Four `halt_baddata()` exports outside this bucket remain untranslated**
  (`0x80C0B510`, `0x80C228A0`, `0x80C512F0`, `0x80C51430`). They belong to the
  `0x80C00000`, `0x80C20000` and `0x80C50000` buckets, which are outside this
  lane's assigned remainder, so they were not touched. Their raw text is intact
  in the same REL and is recoverable by the same method.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_en_bossNP/export/functions.tsv`; the two sizes are 4 and
  16.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_en_bossNP/export/`.
- Each body was decoded from the raw REL text at text offset
  `address - 0x80C00000`; `0x80C4FDE0` agrees instruction-for-instruction with
  the partial Ghidra export where it overlaps (the `+0x2E1` store).
- Every branch target was computed from the relocated branch field in
  `work/rels/elf/d_en_bossNP.elf` and checked against the `main.dol` exports
  `8016f640_FUN_8016f640.c`, `8016f550_FUN_8016f550.c` and
  `8016f500_FUN_8016f500.c`; `0x800AD4F0` has no `main.dol` export.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `node tools/check-lane-csv.mjs wave113` reports `OK`, with no problem naming
  `wave113-bosses.csv`.
- `cargo check --message-format=short` reports no error in
  `src/game/bosses.rs`.
- The module header count was updated from 764 to 766 and the wave-113 bullet
  was added.
- No files outside `src/game/bosses.rs`, this document and
  `ledger/agents/wave113-bosses.csv` were edited.

## Module state after this pass

`d_en_bossNP` now has 766 functions (765 `pub unsafe fn` bodies plus the
private empty hook). Every entry of the `0x80C40000` bucket is translated.
Across the whole module four truncated exports remain (listed above); a
re-decompile that resolves them is the only way past 766 without the raw-text
method used here.
