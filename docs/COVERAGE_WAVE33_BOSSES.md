# Coverage - Wave 33 (`d_en_bossNP`, `game::bosses`)

## How to read this file

- **Scope** - the 45 bodies added to `src/game/bosses.rs` by the wave-33 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier work is
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE32_BOSSES.md`;
  this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x8015F7C0`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented.
- **Confidence** rates the translation, not the guessed role. All 45 bodies
  are one constant return each, so every row is `high`.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then **what this pass covers**, then the
  **function table**. Read **deliberately skipped** before treating the
  `0x80C20000` bucket as complete.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bosses.rs` | `game::bosses` | 45 | 540 | 45 | 0 |

Cumulative after this pass: 672 REL bodies in `src/game/bosses.rs`
(671 `pub unsafe fn` plus the private empty hook `fn_80c001e0`).

## What this pass covers

This pass takes the whole `0x80C20000` bucket: every factory run in the
`0x80C2xxxx` window, leaving only the one truncated non-factory export. The
six runs are contiguous and each is bounded by a non-factory neighbour:

| run | site | bodies | returns | bounded before / after |
|---|---|---:|---|---|
| 1 | `0x80C211E0..0x80C212D0` | 16 | mixed `0x8035...` ids | `fn_80c21180` (92 B) / `fn_80c213c0` (224 B), `0xF0` gap |
| 2 | `0x80C218D0..0x80C219C0` | 16 | all `0x80377A04` | `fn_80c21840` (140 B) / `fn_80c21bc0` (1056 B), `0x200` gap |
| 3 | `0x80C292D0` | 1 | `0x803576B4` | `fn_80c29270` (92 B) / `fn_80c293c0` (224 B), `0xF0` gap |
| 4 | `0x80C2A4A0..0x80C2A4D0` | 4 | all `0x80377A04` | `fn_80c2a460` (60 B) / `fn_80c2a600` (556 B), `0x130` gap |
| 5 | `0x80C2C430..0x80C2C480` | 6 | all `0x80377A04` | `fn_80c2c350` (216 B) / `fn_80c2c5d0` (276 B), `0x150` gap |
| 6 | `0x80C2FDB0..0x80C2FDC0` | 2 | `0x80357AB4`, `0x803578B4` | `fn_80c2fd50` (92 B) / `fn_80c2feb0` (224 B), `0xF0` gap |

What is proven for every one of the 45 bodies:

- The export body is literally `return 0x........;` with no load, store or
  call. Every evidence file is 65 bytes.
- The `functions.tsv` size is 12 bytes for every row, matching the
  `lis` / `addi` / `blr` three-instruction slab used by the factory family
  documented in waves 26-32.
- Each run is contiguous: every next member starts `0x10` bytes after the
  previous one, and the member after the run is a non-factory function at a
  larger alignment gap, which fixes the run boundary from both sides.
- `0x80377A04` is returned by 26 of the 45 entries (runs 2, 4 and 5). It is the
  same word returned by the wave-28 Morton entries and the wave-32
  `0x80C37D60` run, so the value cannot identify an owner.

The owning registrars are not recoverable from this module alone: no
decompiled caller and no relocated data word in the export names these slots.
The bodies are translated body-for-body and the wrapper docs say the owner is
unknown rather than guessing a class.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80C211E0` | `fn_80c211e0` | 12 | high |
| `0x80C211F0` | `fn_80c211f0` | 12 | high |
| `0x80C21200` | `fn_80c21200` | 12 | high |
| `0x80C21210` | `fn_80c21210` | 12 | high |
| `0x80C21220` | `fn_80c21220` | 12 | high |
| `0x80C21230` | `fn_80c21230` | 12 | high |
| `0x80C21240` | `fn_80c21240` | 12 | high |
| `0x80C21250` | `fn_80c21250` | 12 | high |
| `0x80C21260` | `fn_80c21260` | 12 | high |
| `0x80C21270` | `fn_80c21270` | 12 | high |
| `0x80C21280` | `fn_80c21280` | 12 | high |
| `0x80C21290` | `fn_80c21290` | 12 | high |
| `0x80C212A0` | `fn_80c212a0` | 12 | high |
| `0x80C212B0` | `fn_80c212b0` | 12 | high |
| `0x80C212C0` | `fn_80c212c0` | 12 | high |
| `0x80C212D0` | `fn_80c212d0` | 12 | high |
| `0x80C218D0` | `fn_80c218d0` | 12 | high |
| `0x80C218E0` | `fn_80c218e0` | 12 | high |
| `0x80C218F0` | `fn_80c218f0` | 12 | high |
| `0x80C21900` | `fn_80c21900` | 12 | high |
| `0x80C21910` | `fn_80c21910` | 12 | high |
| `0x80C21920` | `fn_80c21920` | 12 | high |
| `0x80C21930` | `fn_80c21930` | 12 | high |
| `0x80C21940` | `fn_80c21940` | 12 | high |
| `0x80C21950` | `fn_80c21950` | 12 | high |
| `0x80C21960` | `fn_80c21960` | 12 | high |
| `0x80C21970` | `fn_80c21970` | 12 | high |
| `0x80C21980` | `fn_80c21980` | 12 | high |
| `0x80C21990` | `fn_80c21990` | 12 | high |
| `0x80C219A0` | `fn_80c219a0` | 12 | high |
| `0x80C219B0` | `fn_80c219b0` | 12 | high |
| `0x80C219C0` | `fn_80c219c0` | 12 | high |
| `0x80C292D0` | `fn_80c292d0` | 12 | high |
| `0x80C2A4A0` | `fn_80c2a4a0` | 12 | high |
| `0x80C2A4B0` | `fn_80c2a4b0` | 12 | high |
| `0x80C2A4C0` | `fn_80c2a4c0` | 12 | high |
| `0x80C2A4D0` | `fn_80c2a4d0` | 12 | high |
| `0x80C2C430` | `fn_80c2c430` | 12 | high |
| `0x80C2C440` | `fn_80c2c440` | 12 | high |
| `0x80C2C450` | `fn_80c2c450` | 12 | high |
| `0x80C2C460` | `fn_80c2c460` | 12 | high |
| `0x80C2C470` | `fn_80c2c470` | 12 | high |
| `0x80C2C480` | `fn_80c2c480` | 12 | high |
| `0x80C2FDB0` | `fn_80c2fdb0` | 12 | high |
| `0x80C2FDC0` | `fn_80c2fdc0` | 12 | high |

## Deliberately skipped

| address | size | reason |
|---|---:|---|
| `0x80C228A0` | 40 B | Export is truncated: it recovers only `countLeadingZeros(*(param_1 + 0x76C))` and then `halt_baddata();`. No complete behaviour is recoverable, so no body was written. It stays a raw-address call site in its two callers, as recorded in the module's unknowns. |

`0x80C228A0` is the only member of the `0x80C20000` bucket that this pass did
not close; the bucket's other 45 entries are all translated above. The
`0x80C00000`, `0x80C10000`, `0x80C30000` and `0x80C40000` buckets were not
touched by this pass.

## Verification

- Every address and byte size matches `work/ghidra/rel-d_en_bossNP/export/functions.tsv`
  (all 45 rows are 12 bytes).
- Every evidence file exists at the cited path; each was read in full and
  contains exactly one constant `return`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bosses.rs`.
- No address was already in `ledger/functions.csv` (checked before writing).
- `cargo check --message-format=short` exits 0 with no diagnostic naming
  `src/game/bosses.rs`; `node tools/doc-lint.mjs src` reports `ok: true`.
- The 45 rows in `ledger/agents/wave33-bosses.csv` match the Rust functions one
  to one, all `translated`/`high`.
- The module header count and the wave-33 "start here" entry were updated
  (627 -> 672 bodies).
- No files outside `src/game/bosses.rs`, this document and the wave-33 ledger
  were edited.
