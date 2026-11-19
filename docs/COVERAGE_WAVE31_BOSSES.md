# Coverage - Wave 31, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 23 bodies added to `src/game/bosses.rs` by the wave-31 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier work is
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE30_BOSSES.md`;
  this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. Every `0x80C...` value below is an
  analysis address, not a runtime address. Calls into `main.dol` (for example
  `0x802DCBFC`) are absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented; `partial` means the
  visible call is exact but one ABI operand is not representable.
- **Confidence** rates the translation, not the guessed role. The thirteen
  factory bodies are one constant return each (`high`); the ten adapters are
  exact apart from the hidden `r12` register (`medium`).
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. State words are quoted from the constants the exports
  return.
- Start with the **summary**, then **cluster 1** (the factory run), then
  **cluster 2** (the last adapter pairs), then the **function table**. Read
  **remaining candidates** before treating this pass as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 23 |
| text bytes added | 636 |
| status `translated` | 13 |
| status `partial` | 10 |
| status `stub` | 0 |
| confidence `high` | 13 |
| confidence `medium` | 10 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 605 |
| cumulative bytes in `game::bosses` | 125,556 |
| translated share of module bodies | 78.57% |
| translated share of module text bytes | 35.23% |

This pass takes the last two homogeneous groups in the module:

- the thirteen state-id factories at `0x80C1C2E0..0x80C1C3A0`, one contiguous
  run of 12-byte constant returns;
- the five remaining `0x802DCBFC` destructor pairs at `0x80C18530`,
  `0x80C21560`, `0x80C2A330`, `0x80C3D540` and `0x80C4F340` (ten 48-byte
  forwarders). With these ten bodies every adapter of that shape in the
  module is translated.

## Inventory correction for the factory run

Wave 30 reported 163 untranslated state-id factories and 10 adapters. The
fresh pass over `functions.tsv` and the export bodies gives:

| item | count |
|---|---:|
| 12-byte untranslated factories before this pass | 163 |
| factories translated in this pass | 13 |
| factories untranslated after this pass | 150 |
| 48-byte `0x802DCBFC` adapters before this pass | 10 |
| adapters translated in this pass | 10 |
| adapters untranslated after this pass | 0 |

The 188 untranslated bodies reported by wave 30 therefore decompose after this
pass as 150 factories + one non-adapter 48-byte body (`0x80C38230`) + 14
truncated/thunk exports = 165.

## Why this selection

Wave 30 already noted that the clean gameplay handlers are exhausted, and the
remaining small, complete bodies are the state-id factories and the
`0x802DCBFC` forwarders. This pass finishes the forwarder family and takes one
complete factory run:

| group | site / shape | bodies |
|---|---|---:|
| factories `0x80C1C2E0..0x80C1C3A0` | contiguous run between `0x80C1C280` and `0x80C1C490` | 13 |
| adapter pairs at the five sites below | five `0x802DCBFC` pair sites still open after wave 30 | 10 |

The factory run is taken whole rather than sampled, so the table row count and
the byte count are exact. The five adapter pairs are the last ones in the
module; taking all of them leaves no adapter deferred.

## Cluster 1 - the thirteen `0x80C1C2E0` factories (156 bytes)

| address | size | Rust | returns | status | confidence |
|---|---:|---|---|---|---|
| `0x80C1C2E0` | 12 | `fn_80c1c2e0` | `0x80358804` | translated | high |
| `0x80C1C2F0` | 12 | `fn_80c1c2f0` | `0x803587C4` | translated | high |
| `0x80C1C300` | 12 | `fn_80c1c300` | `0x80358784` | translated | high |
| `0x80C1C310` | 12 | `fn_80c1c310` | `0x80358744` | translated | high |
| `0x80C1C320` | 12 | `fn_80c1c320` | `0x80358704` | translated | high |
| `0x80C1C330` | 12 | `fn_80c1c330` | `0x80358204` | translated | high |
| `0x80C1C340` | 12 | `fn_80c1c340` | `0x803581C4` | translated | high |
| `0x80C1C350` | 12 | `fn_80c1c350` | `0x80358184` | translated | high |
| `0x80C1C360` | 12 | `fn_80c1c360` | `0x80358144` | translated | high |
| `0x80C1C370` | 12 | `fn_80c1c370` | `0x80358584` | translated | high |
| `0x80C1C380` | 12 | `fn_80c1c380` | `0x80358304` | translated | high |
| `0x80C1C390` | 12 | `fn_80c1c390` | `0x80358344` | translated | high |
| `0x80C1C3A0` | 12 | `fn_80c1c3a0` | `0x803576F4` | translated | high |

What is proven:

- Every body is exactly `return <word>;` with no load or store. The raw text
  is `lis r3,0` / `addi r3,r3,0` / `blr`; the immediates are filled by REL
  relocations, which Ghidra has already applied.
- The run is contiguous: each next function starts `0x10` bytes after the
  previous one, and the run ends immediately before the gap that leads to
  `0x80C1C490`.
- The words are in the `main.dol` address range (`0x8035....`). Ten of the
  thirteen are also returned by Wendy factories translated in wave 30; the
  three exceptions are `0x803581C4`, `0x80358144` and `0x80358304`. The overlap
  shows the words are shared state descriptors rather than per-class values.
- Neither a decompiled caller nor a relocation from this module's data section
  names the run. The owning registrar is therefore not recoverable from this
  module alone, and the Rust doc says so.

## Cluster 2 - the last five `0x802DCBFC` adapter pairs (480 bytes)

| address | size | Rust | hidden `r12` | adjacent translated site | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C18530` | 48 | `fn_80c18530` | `param_1 + 0x18` | `0xE80` actor constructor `0x80C18590` | partial | medium |
| `0x80C18560` | 48 | `fn_80c18560` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C21560` | 48 | `fn_80c21560` | `param_1 + 0x18` | `0x15C8` actor constructor `0x80C215C0` | partial | medium |
| `0x80C21590` | 48 | `fn_80c21590` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C2A330` | 48 | `fn_80c2a330` | `param_1 + 0x18` | `0x780` actor constructor `0x80C2A390` | partial | medium |
| `0x80C2A360` | 48 | `fn_80c2a360` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C3D540` | 48 | `fn_80c3d540` | `param_1 + 0x18` | `0x1118` actor constructor `0x80C3D5A0` | partial | medium |
| `0x80C3D570` | 48 | `fn_80c3d570` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C4F340` | 48 | `fn_80c4f340` | `param_1 + 0x18` | effect-slot publisher `0x80C4F3A0` | partial | medium |
| `0x80C4F370` | 48 | `fn_80c4f370` | `param_1 + 0x24` | same site | partial | medium |

What is proven:

- The C body of all ten is exactly
  `func_0x802dcbfc(param_2,param_2,param_1);`.
- The raw PowerPC text is byte-identical across the ten bodies apart from the
  `r12` immediate: `stwu r1,-0x10(r1)` / `mflr r0` / `mr r5,r3` /
  `mr r3,r4` / `stw r0,0x14(r1)` / `addi r12,r5,0x18` or `0x24` /
  `bl <0x802DCBFC through the import slot>` / `nop` / epilogue. The immediates
  were read from `work/rels/d_en_bossNP.rel`, not inferred from the C.
- Each pair is exactly `0x60` bytes before a site: the first adapter is at
  `site - 0x60`, the second at `site - 0x30`.
- `platform::call3` cannot load `r12`, so the visible call is translated but
  every body is `partial`; the exact adjustment is recorded in each doc
  comment.
- Five pairs use a constructor as their following site. The last pair
  (`0x80C4F340`/`0x80C4F370`) precedes the effect-publication method
  `0x80C4F3A0`, so its owning class is marked unknown instead of guessed.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C1C2E0` | `FUN_80c1c2e0` | `fn_80c1c2e0` | state-id factory | translated | high | `80c1c2e0_FUN_80c1c2e0.c` |
| `0x80C1C2F0` | `FUN_80c1c2f0` | `fn_80c1c2f0` | state-id factory | translated | high | `80c1c2f0_FUN_80c1c2f0.c` |
| `0x80C1C300` | `FUN_80c1c300` | `fn_80c1c300` | state-id factory | translated | high | `80c1c300_FUN_80c1c300.c` |
| `0x80C1C310` | `FUN_80c1c310` | `fn_80c1c310` | state-id factory | translated | high | `80c1c310_FUN_80c1c310.c` |
| `0x80C1C320` | `FUN_80c1c320` | `fn_80c1c320` | state-id factory | translated | high | `80c1c320_FUN_80c1c320.c` |
| `0x80C1C330` | `FUN_80c1c330` | `fn_80c1c330` | state-id factory | translated | high | `80c1c330_FUN_80c1c330.c` |
| `0x80C1C340` | `FUN_80c1c340` | `fn_80c1c340` | state-id factory | translated | high | `80c1c340_FUN_80c1c340.c` |
| `0x80C1C350` | `FUN_80c1c350` | `fn_80c1c350` | state-id factory | translated | high | `80c1c350_FUN_80c1c350.c` |
| `0x80C1C360` | `FUN_80c1c360` | `fn_80c1c360` | state-id factory | translated | high | `80c1c360_FUN_80c1c360.c` |
| `0x80C1C370` | `FUN_80c1c370` | `fn_80c1c370` | state-id factory | translated | high | `80c1c370_FUN_80c1c370.c` |
| `0x80C1C380` | `FUN_80c1c380` | `fn_80c1c380` | state-id factory | translated | high | `80c1c380_FUN_80c1c380.c` |
| `0x80C1C390` | `FUN_80c1c390` | `fn_80c1c390` | state-id factory | translated | high | `80c1c390_FUN_80c1c390.c` |
| `0x80C1C3A0` | `FUN_80c1c3a0` | `fn_80c1c3a0` | state-id factory | translated | high | `80c1c3a0_FUN_80c1c3a0.c` |
| `0x80C18530` | `FUN_80c18530` | `fn_80c18530` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c18530_FUN_80c18530.c` |
| `0x80C18560` | `FUN_80c18560` | `fn_80c18560` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c18560_FUN_80c18560.c` |
| `0x80C21560` | `FUN_80c21560` | `fn_80c21560` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c21560_FUN_80c21560.c` |
| `0x80C21590` | `FUN_80c21590` | `fn_80c21590` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c21590_FUN_80c21590.c` |
| `0x80C2A330` | `FUN_80c2a330` | `fn_80c2a330` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c2a330_FUN_80c2a330.c` |
| `0x80C2A360` | `FUN_80c2a360` | `fn_80c2a360` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c2a360_FUN_80c2a360.c` |
| `0x80C3D540` | `FUN_80c3d540` | `fn_80c3d540` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c3d540_FUN_80c3d540.c` |
| `0x80C3D570` | `FUN_80c3d570` | `fn_80c3d570` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c3d570_FUN_80c3d570.c` |
| `0x80C4F340` | `FUN_80c4f340` | `fn_80c4f340` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c4f340_FUN_80c4f340.c` |
| `0x80C4F370` | `FUN_80c4f370` | `fn_80c4f370` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c4f370_FUN_80c4f370.c` |

## Unknowns

- **Owner of the `0x80C1C2E0` run.** No decompiled caller and no relocated
  data word in this module points at these thirteen factories. They are
  translated body-for-body, but the registrar that consumes them lives outside
  the reach of this module's exports.
- **Meaning of `r12` at `0x802DCBFC`.** The instruction that loads it is exact
  (`param_1 + 0x18` or `param_1 + 0x24`), but how the imported stub consumes
  it is not proven. The bodies stay `partial` because `platform::call3` cannot
  set it.
- **Adapter ownership.** No C function references the ten adapters; their
  vtable slots are written through data initializers. The four constructor
  assignments follow from the adjacent constructor address, not from a named
  caller. The last pair's owner is explicitly unknown.
- **Shared state words.** The same `0x8035....` words appear in more than one
  factory table. Their numeric pattern is not explained by the exports; only
  the constant return is proven.

## Remaining candidates

| address / family | size | why deferred |
|---|---|---|
| 150 state-id factories | 12 B each | complete bodies; the largest contiguous runs are `0x80C3D620` (17), `0x80C211E0` (16), `0x80C218D0` (16), `0x80C3D1F0` (13), `0x80C37D60` (9) |
| `0x80C38230` | 48 B | not an adapter: virtual call through `*(param_1 + 0x394)`, returns `1` |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

After this pass 165 of the 770 module functions remain untranslated: 150
factories + the non-adapter `0x80C38230` + 14 truncated or thunk exports.

## Verification

- `cargo check` exits 0; no diagnostic names `src/game/bosses.rs`. The eleven
  remaining warnings are in `src/runtime/bt.rs` and `src/runtime/input.rs`,
  which belong to other lanes and were not edited.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 23 evidence files were read in full: thirteen are one constant `return`,
  ten are the single `0x802DCBFC` call.
- Each selected body's raw bytes were checked in
  `work/rels/d_en_bossNP.rel`: the thirteen factories are
  `lis`/`addi`/`blr`, and each adapter contains `addi r12,r5,0x18` or `0x24`
  in the expected slot.
- The file now defines 605 REL bodies (604 `pub unsafe fn` plus the private
  empty hook `fn_80c001e0`); the module header count was updated from 582 to
  605.
- The 23 rows in `ledger/agents/wave31-bosses.csv` match the Rust functions
  one to one: 13 `translated`/`high` and 10 `partial`/`medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-31 ledger
  were edited.
