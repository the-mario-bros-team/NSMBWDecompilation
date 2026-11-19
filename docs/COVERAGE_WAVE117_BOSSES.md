# Coverage - Wave 117 (Bosses)

## How to read this file

- **Scope.** One body added to `src/game/bosses.rs` for the boss-actor REL
  `d_en_bossNP` (REL module 4). It is the last untranslated entry of the
  `0x80C00000` bucket, and it is an entry whose Ghidra C export stops at
  `halt_baddata()`. Earlier work is documented in
  `docs/COVERAGE_WAVE114_BOSSES.md` and the wave-2 through wave-114 boss
  coverage files; this note does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x80162D00`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence.** The Ghidra C export path is cited in the row and in the
  `# Original` block. Because the export truncates, the body was decoded
  instruction by instruction from the intact raw REL text
  (`work/rels/d_en_bossNP.rel`, text section 1), and the one relocated field
  was resolved from the REL relocation stream (module 0, `R_PPC_REL24`, addend
  `0x80162D00` at `0x80C0B514`) and then cross-checked in the converted image
  `work/rels/elf/d_en_bossNP.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 1 | 8 | 1 | 0 |

Cumulative after this pass: 769 functions in `src/game/bosses.rs` (768
`pub unsafe fn` plus the private empty hook `fn_80c001e0`). The 8 bytes were
re-read from `work/ghidra/rel-d_en_bossNP/export/functions.tsv`.

## What this pass covers

This pass takes the one remaining untranslated entry of bucket `0x80C00000`
(all 153 other bucket members were already translated). It is the only
`0x80C00000` entry whose Ghidra export truncates.

1. **The `+0x770` handle resolver `0x80C0B510` (8 bytes).** Two instructions:
   `lwz r3,0x770(r3)` and `b 0x80162D00`. The branch field carries the module-0
   `R_PPC_REL24` relocation with addend `0x80162D00`, so the loaded word is the
   single argument of the `main.dol` bank lookup `FUN_80162d00`: that function
   selects its bucket with `(id & 7) * 0x20` from `0x80377A70`, walks the chain
   through `0x80162C30` and returns the node's `+0x08` descriptor word, or zero
   on a miss. The Rust body therefore reads `this + 0x770` and forwards it
   through `platform::call1(0x8016_2D00, ..)`.
   - The field is used as a handle, not a pointer. The four call sites in this
     REL (`0x80C0B194`, `0x80C0C24C`, `0x80C0C2CC`, `0x80C0C3E4`) all reach
     this body with `bl`; the first one clears `+0x770` immediately after the
     lookup, and two of them pass the result together with `this + 0x774` to
     `0x806EEE20`, so `+0x770` and the floats at `+0x774..+0x77C` form a
     handle/vector pair. The same read-then-lookup chain is written inline in
     `fn_80c191a0` and `fn_80c43670`, which is independent evidence for the
     shape.
   - Marked `high`: both instructions decode, the branch target is
     relocation-backed, and the callee has a recovered `main.dol` body.

The `# Original` block still cites the Ghidra export (the required evidence
path), while the body text states that the instructions were recovered from the
raw REL because the export truncates.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C0B510` | `fn_80c0b510` | 8 | high |

## Deliberately skipped

- **Nothing in this bucket.** `0x80C0B510` is the whole `0x80C00000`
  remainder and it could be reconstructed from the raw REL text plus the
  relocation stream.
- **One `halt_baddata()` export remains in the module**, outside this lane's
  bucket: `0x80C228A0` (40 bytes, `0x80C20000` bucket). Its raw text is intact
  in the same REL and is recoverable by the same method, but that bucket is not
  assigned to this pass.

## Verification

- The address and byte size match
  `work/ghidra/rel-d_en_bossNP/export/functions.tsv`; the size is 8.
- The evidence file exists at the cited path under
  `work/ghidra/rel-d_en_bossNP/export/`.
- The body was decoded from the raw REL text; the single relocated field is
  `0x80C0B514` (`R_PPC_REL24`, module 0, addend `0x80162D00`), read back from
  the REL relocation stream.
- The branch target was checked in the converted ELF and against
  `work/ghidra/main-gb/export/80162d00_FUN_80162d00.c`.
- The row has exactly one `pub unsafe fn fn_80c0b510` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `node tools/check-lane-csv.mjs wave117` reports `OK`, with no problem naming
  `wave117-bosses.csv`.
- `cargo check --message-format=short` reports no error in
  `src/game/bosses.rs`.
- The module header count was updated from 768 to 769 and the wave-117 bullet
  and "start here" entry were added.
- No files outside `src/game/bosses.rs`, this document and
  `ledger/agents/wave117-bosses.csv` were edited.

## Module state after this pass

`d_en_bossNP` now has 769 functions (768 `pub unsafe fn` bodies plus the private
empty hook). Every entry of the `0x80C00000` bucket is translated. Across the
whole module one truncated export remains (`0x80C228A0`, in the `0x80C20000`
bucket).
