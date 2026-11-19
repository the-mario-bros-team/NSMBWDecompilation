# Coverage - Wave 32, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 22 bodies added to `src/game/bosses.rs` by the wave-32 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier work is
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE31_BOSSES.md`;
  this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x802DCBFC`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented.
- **Confidence** rates the translation, not the guessed role. All 22 bodies
  are one constant return each, so every row is `high`.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. State words are quoted from the constants the exports
  return.
- Start with the **summary**, then **cluster 1** (the thirteen-body
  `0x80C3D1F0` run), then **cluster 2** (the nine-body `0x80C37D60` run), then
  the **function table**. Read **remaining candidates** before treating this
  pass as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 22 |
| text bytes added | 264 |
| status `translated` | 22 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 22 |
| confidence `medium` | 0 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 627 |
| cumulative bytes in `game::bosses` | 125,820 |
| translated share of module bodies | 81.43% |
| translated share of module text bytes | 35.30% |

This pass takes two complete constant-return runs:

- thirteen bodies at `0x80C3D1F0..0x80C3D2B0`, the run between the 92-byte
  runner `0x80C3D190` and the handler family at `0x80C3D3A0`;
- nine bodies at `0x80C37D60..0x80C37DE0`, the run immediately before the
  Larry actor constructor `0x80C37DF0`.

The runs are taken whole rather than sampled, so the table row count and the
byte count are exact. Together they cover every contiguous factory body in
their two address windows; no member of either run is deferred.

## Why this selection

Wave 31 finished the clean gameplay handlers, the destructor-adapter family and
one factory run. The remaining complete bodies are the 150 state-id factories.
This pass takes two whole runs that fit the 20-25 body budget, using the same
selection rule as wave 31:

| group | site / shape | bodies |
|---|---|---:|
| factories `0x80C3D1F0..0x80C3D2B0` | contiguous run after `0x80C3D190`, before `0x80C3D3A0` | 13 |
| factories `0x80C37D60..0x80C37DE0` | contiguous run after `0x80C37CB0`, before constructor `0x80C37DF0` | 9 |

Choosing whole runs avoids a partial sample inside a table; the nine-body run
is safe to take alone because its last body ends exactly where the Larry
constructor begins.

## Cluster 1 - the thirteen `0x80C3D1F0` factories (156 bytes)

| address | size | Rust | returns | status | confidence |
|---|---:|---|---|---|---|
| `0x80C3D1F0` | 12 | `fn_80c3d1f0` | `0x80358804` | translated | high |
| `0x80C3D200` | 12 | `fn_80c3d200` | `0x803587C4` | translated | high |
| `0x80C3D210` | 12 | `fn_80c3d210` | `0x80358784` | translated | high |
| `0x80C3D220` | 12 | `fn_80c3d220` | `0x80358744` | translated | high |
| `0x80C3D230` | 12 | `fn_80c3d230` | `0x80358704` | translated | high |
| `0x80C3D240` | 12 | `fn_80c3d240` | `0x803586C4` | translated | high |
| `0x80C3D250` | 12 | `fn_80c3d250` | `0x803576F4` | translated | high |
| `0x80C3D260` | 12 | `fn_80c3d260` | `0x803585C4` | translated | high |
| `0x80C3D270` | 12 | `fn_80c3d270` | `0x80358584` | translated | high |
| `0x80C3D280` | 12 | `fn_80c3d280` | `0x80358384` | translated | high |
| `0x80C3D290` | 12 | `fn_80c3d290` | `0x80358344` | translated | high |
| `0x80C3D2A0` | 12 | `fn_80c3d2a0` | `0x80358304` | translated | high |
| `0x80C3D2B0` | 12 | `fn_80c3d2b0` | `0x803582C4` | translated | high |

What is proven:

- Every body is exactly `return <word>;` with no load or store. The raw text
  is `lis r3,0` / `addi r3,r3,0` / `blr`; the immediates are filled by REL
  relocations, which Ghidra has already applied.
- All 22 selected bodies were checked byte-for-byte in
  `work/rels/d_en_bossNP.rel`: every 12-byte slot is
  `3C600000 38630000 4E800020` before relocation.
- The run is contiguous: each next function starts `0x10` bytes after the
  previous one, and the next function starts `0xF0` bytes after the last
  member at `0x80C3D3A0`.
- Nine of the thirteen words also occur in the wave-31 run at
  `0x80C1C2E0..0x80C1C3A0`. The overlap shows that the words are shared state
  descriptors rather than per-class values.
- Neither a decompiled caller nor a relocation from this module's data section
  names the run. The owning registrar is therefore not recoverable from this
  module alone, and the Rust docs say so.

## Cluster 2 - the nine `0x80C37D60` factories (108 bytes)

| address | size | Rust | returns | status | confidence |
|---|---:|---|---|---|---|
| `0x80C37D60` | 12 | `fn_80c37d60` | `0x80377A04` | translated | high |
| `0x80C37D70` | 12 | `fn_80c37d70` | `0x80377A04` | translated | high |
| `0x80C37D80` | 12 | `fn_80c37d80` | `0x80377A04` | translated | high |
| `0x80C37D90` | 12 | `fn_80c37d90` | `0x80377A04` | translated | high |
| `0x80C37DA0` | 12 | `fn_80c37da0` | `0x80377A04` | translated | high |
| `0x80C37DB0` | 12 | `fn_80c37db0` | `0x80377A04` | translated | high |
| `0x80C37DC0` | 12 | `fn_80c37dc0` | `0x80377A04` | translated | high |
| `0x80C37DD0` | 12 | `fn_80c37dd0` | `0x80377A04` | translated | high |
| `0x80C37DE0` | 12 | `fn_80c37de0` | `0x80377A04` | translated | high |

What is proven:

- Every body is the same `lis`/`addi`/`blr` constant return, verified in the
  raw REL text.
- The run is contiguous and ends `0x10` bytes before the Larry actor
  constructor `0x80C37DF0`, which fixes both boundaries.
- All nine entries return `0x80377A04`, the same word returned by the nine
  Morton factories of wave 28. A repeated word cannot identify an owner.
- No decompiled caller or data relocation names the run. The adjacency to the
  Larry constructor suggests a Larry-family table but does not prove it, so the
  Rust docs mark the owner unknown.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C3D1F0` | `FUN_80c3d1f0` | `fn_80c3d1f0` | state-id factory | translated | high | `80c3d1f0_FUN_80c3d1f0.c` |
| `0x80C3D200` | `FUN_80c3d200` | `fn_80c3d200` | state-id factory | translated | high | `80c3d200_FUN_80c3d200.c` |
| `0x80C3D210` | `FUN_80c3d210` | `fn_80c3d210` | state-id factory | translated | high | `80c3d210_FUN_80c3d210.c` |
| `0x80C3D220` | `FUN_80c3d220` | `fn_80c3d220` | state-id factory | translated | high | `80c3d220_FUN_80c3d220.c` |
| `0x80C3D230` | `FUN_80c3d230` | `fn_80c3d230` | state-id factory | translated | high | `80c3d230_FUN_80c3d230.c` |
| `0x80C3D240` | `FUN_80c3d240` | `fn_80c3d240` | state-id factory | translated | high | `80c3d240_FUN_80c3d240.c` |
| `0x80C3D250` | `FUN_80c3d250` | `fn_80c3d250` | state-id factory | translated | high | `80c3d250_FUN_80c3d250.c` |
| `0x80C3D260` | `FUN_80c3d260` | `fn_80c3d260` | state-id factory | translated | high | `80c3d260_FUN_80c3d260.c` |
| `0x80C3D270` | `FUN_80c3d270` | `fn_80c3d270` | state-id factory | translated | high | `80c3d270_FUN_80c3d270.c` |
| `0x80C3D280` | `FUN_80c3d280` | `fn_80c3d280` | state-id factory | translated | high | `80c3d280_FUN_80c3d280.c` |
| `0x80C3D290` | `FUN_80c3d290` | `fn_80c3d290` | state-id factory | translated | high | `80c3d290_FUN_80c3d290.c` |
| `0x80C3D2A0` | `FUN_80c3d2a0` | `fn_80c3d2a0` | state-id factory | translated | high | `80c3d2a0_FUN_80c3d2a0.c` |
| `0x80C3D2B0` | `FUN_80c3d2b0` | `fn_80c3d2b0` | state-id factory | translated | high | `80c3d2b0_FUN_80c3d2b0.c` |
| `0x80C37D60` | `FUN_80c37d60` | `fn_80c37d60` | state-id factory | translated | high | `80c37d60_FUN_80c37d60.c` |
| `0x80C37D70` | `FUN_80c37d70` | `fn_80c37d70` | state-id factory | translated | high | `80c37d70_FUN_80c37d70.c` |
| `0x80C37D80` | `FUN_80c37d80` | `fn_80c37d80` | state-id factory | translated | high | `80c37d80_FUN_80c37d80.c` |
| `0x80C37D90` | `FUN_80c37d90` | `fn_80c37d90` | state-id factory | translated | high | `80c37d90_FUN_80c37d90.c` |
| `0x80C37DA0` | `FUN_80c37da0` | `fn_80c37da0` | state-id factory | translated | high | `80c37da0_FUN_80c37da0.c` |
| `0x80C37DB0` | `FUN_80c37db0` | `fn_80c37db0` | state-id factory | translated | high | `80c37db0_FUN_80c37db0.c` |
| `0x80C37DC0` | `FUN_80c37dc0` | `fn_80c37dc0` | state-id factory | translated | high | `80c37dc0_FUN_80c37dc0.c` |
| `0x80C37DD0` | `FUN_80c37dd0` | `fn_80c37dd0` | state-id factory | translated | high | `80c37dd0_FUN_80c37dd0.c` |
| `0x80C37DE0` | `FUN_80c37de0` | `fn_80c37de0` | state-id factory | translated | high | `80c37de0_FUN_80c37de0.c` |

## Unknowns

- **Owners of both runs.** No decompiled caller and no relocated data word in
  this module points at either run. The bodies are translated body-for-body,
  but the registrars that consume them live outside the reach of the exports.
- **Shared state words.** `0x80377A04` is returned by all nine entries of the
  second run and by nine Morton entries; nine words of the first run also
  occur in the wave-31 run. The exports prove the constant return only; they
  do not explain the numeric pattern.
- **Larry adjacency.** The second run ends immediately before the Larry actor
  constructor `0x80C37DF0`, which is suggestive but not proof of ownership.

## Remaining candidates

| address / family | size | why deferred |
|---|---|---|
| 128 state-id factories | 12 B each | complete bodies; the largest contiguous runs are `0x80C3D620` (17), `0x80C211E0` (16), `0x80C218D0` (16), `0x80C118F0` (10), `0x80C0DFE0` (9) |
| `0x80C38230` | 48 B | not a factory: virtual call through `*(param_1 + 0x394)`, returns `1` |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

After this pass 143 of the 770 module functions remain untranslated: 128
factories + the non-factory `0x80C38230` + 14 truncated or thunk exports.

## Verification

- `cargo check` exits 0; the 11 remaining warnings are in `src/runtime/bt.rs`
  and `src/runtime/input.rs`, which belong to other lanes and were not edited.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 22 evidence files were read in full; each is a single constant `return`.
- All 22 raw REL bodies were checked at their file offsets: every one is
  `3C600000 38630000 4E800020` (`lis` / `addi` / `blr`) before relocation.
- The file now defines 627 REL bodies (626 `pub unsafe fn` plus the private
  empty hook `fn_80c001e0`); the module header count was updated from 605 to
  627.
- The 22 rows in `ledger/agents/wave32-bosses.csv` match the Rust functions
  one to one, all `translated`/`high`.
- No files outside `src/game/bosses.rs`, this document and the wave-32 ledger
  were edited.
