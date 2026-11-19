# Coverage - Wave 26, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 24 functions added to `src/game/bosses.rs` by the wave-26
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions) and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE25_BOSSES.md`;
  this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8015F7C0`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented.
- **Confidence** rates the translation, not the guessed role. `high` means the
  body is exact; here every body is a single constant return, so all 24 rows
  are `high`.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then **why this selection**, then the **two
  clusters**, then the **function table**. Read the **remaining candidates**
  before treating this pass as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 24 |
| text bytes added | 288 |
| status `translated` | 24 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 24 |
| confidence `medium` | 0 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 490 |
| cumulative bytes in `game::bosses` | 122,952 |
| translated share of module bodies | 63.64% |
| translated share of module text bytes | 34.50% |

This pass translates the two complete state-id factory tables: the twelve
Castle Iggy entries (`0x80C04790..0x80C04840`) and the twelve Castle Roy
entries (`0x80C15880..0x80C15930`). Every body is exactly twelve bytes and is
one `return` of a constant; the owning registrar calls each address exactly
once and stores the result in the state's `+0x30` word.

## Why this selection

The wave-25 close-out reported 304 untranslated bodies, and a fresh diff of
`functions.tsv` against `ledger/functions.csv` still gives exactly 304. Their
composition is:

| remaining kind | count | sizes | complete control flow? |
|---|---:|---|---|
| state-id factories | 255 | 12 B each | yes |
| `0x802DCBFC` destructor adapters | 35 | 48 B each | yes, but hidden `r12` is unmodelled |
| import thunks with bad instruction data | 4 | 4 B each | no statements recovered |
| exports that stop at `halt_baddata()` | 5 | 40-72 B | no |
| other bad-instruction exports | 4 | 8-28 B | no |
| one truncated byte-store export | 1 | 16 B | no |

No untranslated body with complete control flow in the constructor,
phase/attack, collision/damage or aim/heading categories remains in this REL:
the exported functions in those categories were consumed by waves 3-25. The
only bodies with complete control flow and no missing ABI operand left are the
state-id factories, so this pass takes two that form complete
tables rather than a scattered sample. After this pass 280 of the 304 remain:
231 factories, 35 adapters and 14 truncated or thunk exports, all listed under
**remaining candidates**.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |
| BSS end (exclusive) | `0x80C7117C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it.

## Cluster 1 - Castle Iggy state-id factory table (144 bytes)

| address | size | Rust | returns | status | confidence |
|---|---:|---|---|---|---|
| `0x80C04790` | 12 | `fn_80c04790` | `0x80358704` | translated | high |
| `0x80C047A0` | 12 | `fn_80c047a0` | `0x803586C4` | translated | high |
| `0x80C047B0` | 12 | `fn_80c047b0` | `0x803576F4` | translated | high |
| `0x80C047C0` | 12 | `fn_80c047c0` | `0x803583C4` | translated | high |
| `0x80C047D0` | 12 | `fn_80c047d0` | `0x80358484` | translated | high |
| `0x80C047E0` | 12 | `fn_80c047e0` | `0x80358444` | translated | high |
| `0x80C047F0` | 12 | `fn_80c047f0` | `0x80358404` | translated | high |
| `0x80C04800` | 12 | `fn_80c04800` | `0x80358384` | translated | high |
| `0x80C04810` | 12 | `fn_80c04810` | `0x80358344` | translated | high |
| `0x80C04820` | 12 | `fn_80c04820` | `0x80358304` | translated | high |
| `0x80C04830` | 12 | `fn_80c04830` | `0x803582C4` | translated | high |
| `0x80C04840` | 12 | `fn_80c04840` | `0x80358284` | translated | high |

What is proven:

- Each export body is exactly `return <constant>;` with no load or store.
- The Iggy registrar [`fn_80c03880`] calls all twelve addresses. For the 11th
  entry the C reads `uVar13 = FUN_80c04790();`, copies the nine descriptor
  words into the state object at `0x80C6CCD4`, writes the vtable
  `0x80C5A1CC`, and then stores the factory result at `0x80C6CD04`, which is
  state `+0x30`.
- The same one-call/one-store shape repeats for the other eleven entries; the
  destination state base advances by `0x40` per state.
- The return values are opaque ids in `main.dol` data (`0x8035....`). They are
  stored, never called or dereferenced.

## Cluster 2 - Castle Roy state-id factory table (144 bytes)

| address | size | Rust | returns | status | confidence |
|---|---:|---|---|---|---|
| `0x80C15880` | 12 | `fn_80c15880` | BSS address `0x80C70984` | translated | high |
| `0x80C15890` | 12 | `fn_80c15890` | `0x80358404` | translated | high |
| `0x80C158A0` | 12 | `fn_80c158a0` | `0x803583C4` | translated | high |
| `0x80C158B0` | 12 | `fn_80c158b0` | `0x803585C4` | translated | high |
| `0x80C158C0` | 12 | `fn_80c158c0` | BSS address `0x80C70C44` | translated | high |
| `0x80C158D0` | 12 | `fn_80c158d0` | `0x80358544` | translated | high |
| `0x80C158E0` | 12 | `fn_80c158e0` | `0x80358244` | translated | high |
| `0x80C158F0` | 12 | `fn_80c158f0` | BSS address `0x80C709C4` | translated | high |
| `0x80C15900` | 12 | `fn_80c15900` | `0x803581C4` | translated | high |
| `0x80C15910` | 12 | `fn_80c15910` | `0x80358744` | translated | high |
| `0x80C15920` | 12 | `fn_80c15920` | `0x80358704` | translated | high |
| `0x80C15930` | 12 | `fn_80c15930` | `0x803586C4` | translated | high |

What is proven:

- Each export body is exactly `return <constant>;`. Four bodies print
  `return &DAT_80c7....;`; their value is an address in this REL's BSS, so the
  Rust bodies return the address as a `u32` and never read the word.
- The Roy registrar [`fn_80c146b0`] calls all twelve addresses. For the first
  entry the C reads `uVar10 = FUN_80c15880();`, copies the descriptor into the
  state object at `0x80C6DE2C`, writes the vtable `0x80C5E19C`, and stores the
  result at `0x80C6DE5C`, which is state `+0x30`.
- Two ids are shared with the Iggy table: `0x80358704` and `0x803586C4`. The
  factories are separate bodies; only the returned word is equal.
- All four BSS addresses lie inside the documented REL BSS window
  `0x80C6C900..0x80C7117C`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C04790` | `FUN_80c04790` | `fn_80c04790` | Iggy state-id factory | translated | high | `80c04790_FUN_80c04790.c` |
| `0x80C047A0` | `FUN_80c047a0` | `fn_80c047a0` | Iggy state-id factory | translated | high | `80c047a0_FUN_80c047a0.c` |
| `0x80C047B0` | `FUN_80c047b0` | `fn_80c047b0` | Iggy state-id factory | translated | high | `80c047b0_FUN_80c047b0.c` |
| `0x80C047C0` | `FUN_80c047c0` | `fn_80c047c0` | Iggy state-id factory | translated | high | `80c047c0_FUN_80c047c0.c` |
| `0x80C047D0` | `FUN_80c047d0` | `fn_80c047d0` | Iggy state-id factory | translated | high | `80c047d0_FUN_80c047d0.c` |
| `0x80C047E0` | `FUN_80c047e0` | `fn_80c047e0` | Iggy state-id factory | translated | high | `80c047e0_FUN_80c047e0.c` |
| `0x80C047F0` | `FUN_80c047f0` | `fn_80c047f0` | Iggy state-id factory | translated | high | `80c047f0_FUN_80c047f0.c` |
| `0x80C04800` | `FUN_80c04800` | `fn_80c04800` | Iggy state-id factory | translated | high | `80c04800_FUN_80c04800.c` |
| `0x80C04810` | `FUN_80c04810` | `fn_80c04810` | Iggy state-id factory | translated | high | `80c04810_FUN_80c04810.c` |
| `0x80C04820` | `FUN_80c04820` | `fn_80c04820` | Iggy state-id factory | translated | high | `80c04820_FUN_80c04820.c` |
| `0x80C04830` | `FUN_80c04830` | `fn_80c04830` | Iggy state-id factory | translated | high | `80c04830_FUN_80c04830.c` |
| `0x80C04840` | `FUN_80c04840` | `fn_80c04840` | Iggy state-id factory | translated | high | `80c04840_FUN_80c04840.c` |
| `0x80C15880` | `FUN_80c15880` | `fn_80c15880` | Roy state-id factory (BSS address) | translated | high | `80c15880_FUN_80c15880.c` |
| `0x80C15890` | `FUN_80c15890` | `fn_80c15890` | Roy state-id factory | translated | high | `80c15890_FUN_80c15890.c` |
| `0x80C158A0` | `FUN_80c158a0` | `fn_80c158a0` | Roy state-id factory | translated | high | `80c158a0_FUN_80c158a0.c` |
| `0x80C158B0` | `FUN_80c158b0` | `fn_80c158b0` | Roy state-id factory | translated | high | `80c158b0_FUN_80c158b0.c` |
| `0x80C158C0` | `FUN_80c158c0` | `fn_80c158c0` | Roy state-id factory (BSS address) | translated | high | `80c158c0_FUN_80c158c0.c` |
| `0x80C158D0` | `FUN_80c158d0` | `fn_80c158d0` | Roy state-id factory | translated | high | `80c158d0_FUN_80c158d0.c` |
| `0x80C158E0` | `FUN_80c158e0` | `fn_80c158e0` | Roy state-id factory | translated | high | `80c158e0_FUN_80c158e0.c` |
| `0x80C158F0` | `FUN_80c158f0` | `fn_80c158f0` | Roy state-id factory (BSS address) | translated | high | `80c158f0_FUN_80c158f0.c` |
| `0x80C15900` | `FUN_80c15900` | `fn_80c15900` | Roy state-id factory | translated | high | `80c15900_FUN_80c15900.c` |
| `0x80C15910` | `FUN_80c15910` | `fn_80c15910` | Roy state-id factory | translated | high | `80c15910_FUN_80c15910.c` |
| `0x80C15920` | `FUN_80c15920` | `fn_80c15920` | Roy state-id factory | translated | high | `80c15920_FUN_80c15920.c` |
| `0x80C15930` | `FUN_80c15930` | `fn_80c15930` | Roy state-id factory | translated | high | `80c15930_FUN_80c15930.c` |

## Unknowns

- **Constant meanings.** The `0x8035....` words are opaque ids resident in
  `main.dol` data; the export does not say what object or state each names.
  The four BSS addresses are equally opaque: the export returns the address,
  not the word's contents, and the contents are outside the C image.
- **Factory role.** The role is inferred from the owners: each address is
  called exactly once by its registrar and the result is stored in state
  `+0x30`, the field the module header already describes as an optional
  state-id/callback result. No factory is used as a function pointer.
- **BSS layout.** The four Roy return addresses (`0x80C70984`, `0x80C70C44`,
  `0x80C709C4`) are inside the module's BSS but their neighbours are not
  otherwise referenced by this pass.
- **Shared ids.** `0x80358704` and `0x803586C4` are each produced by two
  different factories (one Iggy, one Roy). Whether the game treats the shared
  value as the same state family is not proven.

## Remaining candidates

| address / family | size | why deferred |
|---|---:|---|
| 231 state-id factories | 12 B each | complete bodies, same template; not selected in this pass |
| 35 `0x802DCBFC` adapters | 48 B each | hidden `r12` this-adjust operand is not modelable with `platform::call3` |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

The 231 factories are the only clean, complete bodies left; they are
mechanically identical apart from the returned word. The 35 adapters and the
14 truncated/thunk exports need either the hidden `r12` ABI or a better
disassembly before they can be called `translated`.

## Verification

- `cargo check` (via `%USERPROFILE%\.cargo\bin\cargo.exe`) exits 0; the crate's
  11 warnings belong to other lanes and none name `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 24 evidence files were read in full; each contains exactly one constant
  `return` and `functions.tsv` records size 12 for all of them.
- The two registrars were re-read to confirm the one-call/one-`+0x30`-store
  pattern for the table heads (`0x80C6CD04` for Iggy, `0x80C6DE5C` for Roy).
- The 24 rows in `ledger/agents/wave26-bosses.csv` match the Rust functions
  one to one: 24 `translated`, 24 `high`.
- `src/game/bosses.rs` now defines 490 REL bodies; the module header count was
  updated from 466 to 490.
- No files outside `src/game/bosses.rs`, this document and the wave-26 ledger
  were edited.
