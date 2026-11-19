# Coverage - Wave 118 (Bosses)

## How to read this file

- **Scope.** One body added to `src/game/bosses.rs` for the boss-actor REL
  `d_en_bossNP` (REL module 4). It is the last untranslated entry of the
  `0x80C20000` bucket, and it is the last remaining entry whose Ghidra C export
  stops at `halt_baddata()`. Earlier work is documented in
  `docs/COVERAGE_WAVE117_BOSSES.md` and the wave-2 through wave-117 boss
  coverage files; this note does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x80064610`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence.** The Ghidra C export path is cited in the row and in the
  `# Original` block. Because the export truncates, the body was decoded
  instruction by instruction from the intact raw REL text
  (`work/rels/d_en_bossNP.rel`, text section 1, file offset
  `address - 0x80C00000`). The tail branch at `0x80C228C4` was resolved from
  the module-0 `R_PPC_REL24` entry in the REL relocation stream at `0x7EA60`
  (addend `0x80064610`) and cross-checked in the converted image
  `work/rels/elf/d_en_bossNP.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 1 | 40 | 1 | 0 |

Cumulative after this pass: 770 functions in `src/game/bosses.rs` (769
`pub unsafe fn` plus the private empty hook `fn_80c001e0`). The 40 bytes were
re-read from `work/ghidra/rel-d_en_bossNP/export/functions.tsv`.

## What this pass covers

This pass takes the one remaining untranslated entry of bucket `0x80C20000`
(all 126 other bucket members were already translated). It sits between the
clamped-ratio helper `0x80C22580` (`fn_80c22580`) and the wide castle runner
`0x80C22CA0` (`fn_80c22ca0`).

1. **The position-effect spawner `0x80C228A0` (40 bytes).** Ten instructions:

   ```
   lwz    r0,0x76C(r3)
   mr     r5,r4
   li     r3,0x105
   li     r6,0
   cntlzw r0,r0
   li     r7,0
   rlwinm r0,r0,27,31,31
   neg    r4,r0
   addi   r4,r4,0x11
   b      0x80064610
   ```

   The `cntlzw`/`rlwinm`/`neg`/`addi` chain is the compiler's
   `word == 0 ? 0x10 : 0x11`: `cntlzw` yields `32` for zero and `0..31`
   otherwise, the rotate keeps one bit of that count, and the negated value
   added to `0x11` gives `0x10` only when `+0x76C` is zero. The tail branch
   leaves `r3 = 0x105`, `r4 = 0x10` or `0x11`, `r5 = param_2`, `r6 = 0`,
   `r7 = 0`.
   - The branch field carries a module-0 `R_PPC_REL24` relocation with addend
     `0x80064610`, so the tail target is the `main.dol` effect factory
     `0x80064610`. Its export forwards the first four arguments to
     `0x8006CA80`, which stores `param_3`/`param_4` through `0x8006CA70` and
     spawns `param_1`. This is the same `(id, variant, position, 0, 0)` shape
     already translated in `fn_80c32660`, `fn_80c30760` and `fn_80c30820`.
   - The `main.dol` export is typed `void`, but the chain ends in `0x80162B00`,
     which returns its `0x80162A70` result in `r3`. Both call sites in this REL
     consume that word as a handle, so the Rust body returns it.
   - Both call sites are now direct: the two aim scripts `fn_80c27230` and
     `fn_80c27860` call [`fn_80c228a0`] with the actor and the position vector
     they sampled through `0x800E7880`, and pass the returned handle together
     with the clamped heading to `0x806BAEB0`.
   - `// inferred:` `+0x76C` is the castle phase word keyed by
     [`fn_80c23fd0`]; the body only distinguishes zero from nonzero, and the
     `0x105` effect id and the `0x10`/`0x11` variants remain opaque object
     kinds, as recorded in the wave-16 note in the module header.
   - Marked `high`: all ten instructions decode, the tail target is
     relocation-backed, the callee signature is literal in the `main.dol`
     export, and both call sites agree on `(actor, position) -> handle`.

The `# Original` block still cites the Ghidra export (the required evidence
path), while the body text states that the instructions were recovered from the
raw REL because the export truncates.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C228A0` | `fn_80c228a0` | 40 | high |

## Deliberately skipped

- **Nothing.** `0x80C228A0` is the whole `0x80C20000` remainder and it could be
  reconstructed from the raw REL text plus the relocation stream.
- After this pass **no untranslated inventory entry remains** in the module.
  The other four `halt_baddata()` exports were closed out by waves 107
  (`0x80C30000`), 113 (`0x80C40000`), 114 (`0x80C50000`) and 117
  (`0x80C00000`).

## Verification

- The address and byte size match
  `work/ghidra/rel-d_en_bossNP/export/functions.tsv`; the size is 40.
- The evidence file exists at the cited path under
  `work/ghidra/rel-d_en_bossNP/export/`.
- The body was decoded from the raw REL text at text offset
  `0x80C228A0 - 0x80C00000`; all ten instructions decode and the first one
  (`lwz r0,0x76C(r3)`) agrees with the `countLeadingZeros` fragment that Ghidra
  printed before truncating.
- The branch target was computed from the module-0 `R_PPC_REL24` entry
  (`target 0x227D4`, addend `0x80064610`) and checked against the `main.dol`
  exports `80064610_FUN_80064610.c`, `8006ca80_FUN_8006ca80.c`,
  `8006ca70_FUN_8006ca70.c` and `80162b00_FUN_80162b00.c`.
- The row has exactly one `pub unsafe fn fn_80c228a0` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `node tools/check-lane-csv.mjs wave118` reports `OK`, with no problem naming
  `wave118-bosses.csv`.
- `cargo check --message-format=short` reports no error in
  `src/game/bosses.rs`.
- The module header count was updated from 769 to 770 and the wave-118 bullet,
  "start here" entry and unknown-list entry were added.
- No files outside `src/game/bosses.rs`, this document and
  `ledger/agents/wave118-bosses.csv` were edited.

## Module state after this pass

`d_en_bossNP` now has 770 functions (769 `pub unsafe fn` bodies plus the
private empty hook). Every inventory entry in every bucket
(`0x80C00000`, `0x80C10000`, `0x80C20000`, `0x80C30000`, `0x80C40000`,
`0x80C50000`) is translated, and no `halt_baddata()` export remains.
