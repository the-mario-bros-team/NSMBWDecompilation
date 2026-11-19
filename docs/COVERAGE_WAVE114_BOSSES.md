# Coverage - Wave 114 (Bosses)

## How to read this file

- **Scope.** Two bodies added to `src/game/bosses.rs` for the boss-actor REL
  `d_en_bossNP` (REL module 4). They are the last two untranslated entries of
  the `0x80C50000` bucket, and both are entries whose Ghidra C export stops at
  `halt_baddata()`. Earlier work is documented in
  `docs/COVERAGE_WAVE113_BOSSES.md` and the wave-2 through wave-113 boss
  coverage files; this note does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x80106B40`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence.** The Ghidra C export path is cited in every row and every
  `# Original` block. Because both exports truncate, each body was decoded
  instruction by instruction from the intact raw REL text
  (`work/rels/d_en_bossNP.rel`, text section 1, file offset
  `address - 0x80C00000`), and the three relocated
  fields were resolved from the REL import/relocation stream with the same
  rule `tools/rel2elf.mjs` uses (`cursor` accumulated from the 16-bit deltas,
  `R_RVL_SECT` / `R_RVL_NONE` / `R_RVL_STOP` control entries, module-0 addends
  taken as absolute `main.dol` addresses). Every computed target was then
  cross-checked in the converted image `work/rels/elf/d_en_bossNP.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 2 | 44 | 1 | 1 |

Cumulative after this pass: 768 functions in `src/game/bosses.rs` (767
`pub unsafe fn` plus the private empty hook `fn_80c001e0`). The 44 bytes are
`20 + 24`, and both sizes were re-read from
`work/ghidra/rel-d_en_bossNP/export/functions.tsv`.

## What this pass covers

This pass takes the two remaining untranslated entries of bucket `0x80C50000`
(all 50 other bucket members were already translated). They sit at the head of
the Kameck route cluster (`0x80C51460`, `0x80C51520`, `0x80C516E0`) and both
tail-branch into `main.dol`.

1. **The `+0x14` slot reloader `0x80C512F0` (20 bytes).** Five instructions:
   `rlwinm r0,r4,2,0,29`, `li r4,-1`, `add r3,r3,r0`, `lwz r3,0x14(r3)` and
   `b 0x80103DB0`. The index is scaled by four and added to `r3`; the loaded
   word is the slot at `this + 4*index + 0x14`; `r4` is then overwritten with
   `-1`, so the callee receives `(slot_word, -1)`. The only caller, the
   unlisted state-enter routine at `0x80C50BE0`, passes index `0` and then
   writes `+0x24 = 13`, `+0x2C = -1` and `+0x30 = 0.0f` on the same object. The
   `+0x0C`/`+0x14` word pairs are released as heap objects by the destructor
   `fn_80c50b10`, the `+0x14` ones through vtable
   slot `+8`. Marked `medium` because `0x80103DB0` sits in the `main.dol` gap
   between `FUN_80103d90` and `FUN_80103e20` and has no recovered body, so only
   the argument values — not their meaning — are proven.
2. **The route sampler `0x80C51430` (24 bytes).** Six instructions:
   `rlwinm r0,r5,2,0,29`, `lis r5,0x80C6`, `add r4,r4,r0`,
   `lfs f1,-0x6D60(r5)`, `lwz r4,0xC(r4)` and `b 0x80106B40`. The relocated
   `lis`/`lfs` pair resolves to the module-image constant at `0x80C592A0`,
   whose word is `0x00000000` (`0.0f`); `r4` becomes the route word at
   `owner + 4*index + 0x0C`; `r3` is the three-float output vector and is never
   written by this body — its only caller sets `r3 = r1 + 0x38` at
   `0x80C50C8C` immediately before the branch.
   - The EABI point matters here: Ghidra prints the callee as
     `FUN_80106b40(double, undefined4, int *)`. Because the first argument is a
     `float`, it occupies `f1` and the two integer arguments land in `r3` and
     `r4`; the `r5` that this body reuses as the float-literal base is *not* an
     argument. That is proven by the callee's second call site, in `d_basesNP`
     at `0x8076FEB4`, which sets `f1`, `r3 = &local_18` and
     `r4 = DAT_808327E0` and then reads three floats back from `local_18` into
     `+0xAC/+0xB0/+0xB4`. The callee's own export confirms the shape: it reads
     `route[2]` as a segment count and `*route` as the segment table before
     forwarding to `0x80106620`, which writes three floats to `out`.
   - Marked `high`: all six instructions decode, the constant load is
     relocation-backed, and the argument order is proven by an independent
     decompiled call site.

The two `# Original` blocks still cite the Ghidra export (the required evidence
path), while the body text states that the instructions were recovered from the
raw REL because the export truncates.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C512F0` | `fn_80c512f0` | 20 | medium |
| `0x80C51430` | `fn_80c51430` | 24 | high |

## Deliberately skipped

- **Nothing in this bucket.** `0x80C512F0` and `0x80C51430` are the whole
  `0x80C50000` remainder, and both could be reconstructed from the raw REL text.
- **Two `halt_baddata()` exports remain in the module**, both outside this
  lane's bucket: `0x80C0B510` (8 bytes, `0x80C00000` bucket) and `0x80C228A0`
  (40 bytes, `0x80C20000` bucket). Their raw text is intact in the same REL and
  is recoverable by the same method, but they belong to buckets assigned to
  other passes.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_en_bossNP/export/functions.tsv`; the two sizes are 20 and
  24.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_en_bossNP/export/`.
- Each body was decoded from the raw REL text; the three relocated fields are
  `0x80C51300` (`REL24`, module 0, addend `0x80103DB0`), `0x80C51436`/`0x80C5143E`
  (`ADDR16_HA`/`ADDR16_LO` to data `0x80C592A0`) and `0x80C51444` (`REL24`,
  module 0, addend `0x80106B40`), all read back from the REL relocation stream.
- Every branch target in the converted ELF matches those addends, and the
  callee `0x80106B40` was checked against
  `work/ghidra/main-gb/export/80106b40_FUN_80106b40.c` as well as its other
  call site in `d_basesNP`; `0x80103DB0` has no `main.dol` export, which is why
  the first row is `medium`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `node tools/check-lane-csv.mjs wave114` reports `OK`, with no problem naming
  `wave114-bosses.csv`.
- `cargo check --message-format=short` reports no error in
  `src/game/bosses.rs`.
- The module header count was updated from 766 to 768 and the wave-114 bullet
  and "start here" entry were added.
- No files outside `src/game/bosses.rs`, this document and
  `ledger/agents/wave114-bosses.csv` were edited.

## Module state after this pass

`d_en_bossNP` now has 768 functions (767 `pub unsafe fn` bodies plus the
private empty hook). Every entry of the `0x80C50000` bucket is translated.
Across the whole module two truncated exports remain (`0x80C0B510`,
`0x80C228A0`); a re-decompile that resolves them is the only way past 768
without the raw-text method used here.
