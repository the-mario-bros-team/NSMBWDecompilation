# Coverage - Wave 27, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 24 bodies added to `src/game/bosses.rs` by the wave-27 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE25_BOSSES.md`
  and `docs/COVERAGE_WAVE26_BOSSES.md`; this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8015F7C0`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented; `partial` means the
  visible call is exact but one ABI operand is not representable.
- **Confidence** rates the translation, not the guessed role. The twenty
  factory bodies are one constant return each (`high`); the four adapters are
  exact apart from the hidden `r12` register (`medium`).
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. State names are quoted from the data-section strings
  that the owning registrar passes to `0x8015F7C0`.
- Start with the **summary**, then **why this selection**, then the four
  **clusters**, then the **function table**. Read **remaining candidates**
  before treating this pass as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 24 |
| text bytes added | 432 |
| status `translated` | 20 |
| status `partial` | 4 |
| status `stub` | 0 |
| confidence `high` | 20 |
| confidence `medium` | 4 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 514 |
| cumulative bytes in `game::bosses` | 123,384 |
| translated share of module bodies | 66.75% |
| translated share of module text bytes | 34.62% |

The pass closes three owner-proven state-id factory sets and translates two
`0x802DCBFC` destructor pairs:

- the eight remaining Castle Roy entries, which complete the twenty-state
  table the wave-26 pass started;
- all nine Larry entries, in the two blocks its registrar uses;
- all three Koopa demo-cage entries;
- four 48-byte `0x802DCBFC` forwarders: the Castle Roy pair and the Larry pair.

## Why this selection

The wave-26 close-out reported 280 untranslated bodies. Their composition was:

| remaining kind | count | sizes | complete control flow? |
|---|---|---:|---|
| state-id factories | 231 | 12 B each | yes |
| `0x802DCBFC` destructor adapters | 35 | 48 B each | visible call yes, hidden `r12` no |
| import thunks with bad instruction data | 4 | 4 B each | no statements recovered |
| exports that stop at `halt_baddata()` | 5 | 40-72 B | no |
| other bad-instruction exports | 4 | 8-28 B | no |
| one truncated byte-store export | 1 | 16 B | no |

The owner-proven factory sets are the only remaining clean bodies. This pass
takes the three sets whose owning registrar and per-state names can be shown
end to end, and the two adapter pairs that bracket them:

| owner | registrar | states | factories in this pass |
|---|---|---:|---:|
| `daEnBossCastleRoy_c` | `0x80C146B0` | 20 | 8 (the wave-26 pass supplied the other 12) |
| `daEnBossLarry_c` | `0x80C37030` | 9 | 9 |
| `daEnBossKoopaDemoCage_c` | `0x80C29CC0` | 3 | 3 |
| Castle Roy teardown pair | - | - | 2 adapters |
| Larry teardown pair | - | - | 2 adapters |

After this pass 256 of the original 304 bodies remain: 211 factories, 31
adapters and the same 14 truncated or thunk exports. The remaining factory
sets include the fourteen Morton entries at `0x80C42CE0..0x80C42D60` and
`0x80C46510..0x80C46550`, the seventeen Roy entries at
`0x80C468E0..0x80C46960` and `0x80C4A510..0x80C4A580`, and the seventeen Wendy
entries at `0x80C4A8F0..0x80C4A930` and `0x80C4F000..0x80C4F0B0`.

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

## Cluster 1 - Castle Roy: the eight remaining factories (96 bytes)

| address | size | Rust | returns | state registered by `0x80C146B0` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C11DF0` | 12 | `fn_80c11df0` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Enter` | translated | high |
| `0x80C11E00` | 12 | `fn_80c11e00` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Hide` | translated | high |
| `0x80C11E10` | 12 | `fn_80c11e10` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Peek` | translated | high |
| `0x80C11E20` | 12 | `fn_80c11e20` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Peek_Wait` | translated | high |
| `0x80C11E30` | 12 | `fn_80c11e30` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Feint` | translated | high |
| `0x80C11E40` | 12 | `fn_80c11e40` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Exit` | translated | high |
| `0x80C11E50` | 12 | `fn_80c11e50` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_Drop` | translated | high |
| `0x80C11E60` | 12 | `fn_80c11e60` | `0x80377A04` | `daEnBossCastleRoy_c::StateID_ShellAtk_Ed` | translated | high |

What is proven:

- Every body is exactly `return 0x80377a04;` with no load or store.
- The registrar `0x80C146B0` calls each address once. In every block the C
  reads `uVar10 = FUN_80c11dxx();`, passes the quoted `StateID` string to
  `0x8015F7C0`, copies the nine descriptor words, writes the vtable, and then
  stores `uVar10` at the state's `+0x30` word.
- The eight calls at `0x80C11DF0..0x80C11E60` and the twelve at
  `0x80C15880..0x80C15930` (wave 26) are the complete twenty-call set made by
  this registrar, so the twenty-state table is now fully translated.
- The eight new entries all return the same word. The wave-26 twelve return
  eight distinct `0x8035....` ids and four BSS addresses, so a shared id is a
  property of these eight states, not a decode artifact.

## Cluster 2 - Larry: the nine-state table (108 bytes)

| address | size | Rust | returns | state registered by `0x80C37030` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C34D30` | 12 | `fn_80c34d30` | `0x80377A04` | `daEnBossLarry_c::StateID_Dance` | translated | high |
| `0x80C34D40` | 12 | `fn_80c34d40` | `0x80377A04` | `daEnBossLarry_c::StateID_DemoFall` | translated | high |
| `0x80C34D50` | 12 | `fn_80c34d50` | `0x80377A04` | `daEnBossLarry_c::StateID_DemoFallEd` | translated | high |
| `0x80C34D60` | 12 | `fn_80c34d60` | `0x80377A04` | `daEnBossLarry_c::StateID_DemoEscape_Ready` | translated | high |
| `0x80C34D70` | 12 | `fn_80c34d70` | `0x80377A04` | `daEnBossLarry_c::StateID_DemoEscape_Jump` | translated | high |
| `0x80C34D80` | 12 | `fn_80c34d80` | `0x80377A04` | `daEnBossLarry_c::StateID_ShellAtk_Ed` | translated | high |
| `0x80C379A0` | 12 | `fn_80c379a0` | `0x80358584` | `daEnBossLarry_c::StateID_ShellAtk` | translated | high |
| `0x80C379B0` | 12 | `fn_80c379b0` | `0x80358804` | `daEnBossLarry_c::StateID_DemoEscape_St` | translated | high |
| `0x80C379C0` | 12 | `fn_80c379c0` | `0x803576F4` | `daEnBossLarry_c::StateID_DieFumi` | translated | high |

What is proven:

- Every body is one constant return. Six entries return `0x80377A04`; the
  three in the `0x80C379A0..0x80C379C0` block return distinct words.
- `0x80C37030` makes exactly nine factory calls, one per registered state,
  split 6 + 3 across the two blocks. It is the only caller of all nine.
- `0x803576F4` is also returned by the Castle Iggy factory `fn_80c047b0`
  (wave 26). The factories are separate twelve-byte bodies; only the value is
  shared.

## Cluster 3 - Koopa demo cage: the three-state table (36 bytes)

| address | size | Rust | returns | state registered by `0x80C29CC0` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C29650` | 12 | `fn_80c29650` | `0x80377A04` | `daEnBossKoopaDemoCage_c::StateID_Wait` | translated | high |
| `0x80C29660` | 12 | `fn_80c29660` | `0x80377A04` | `daEnBossKoopaDemoCage_c::StateID_Magic` | translated | high |
| `0x80C29670` | 12 | `fn_80c29670` | `0x80377A04` | `daEnBossKoopaDemoCage_c::StateID_Open` | translated | high |

What is proven:

- Three constant returns, all `0x80377A04`.
- `0x80C29CC0` makes exactly three factory calls (`0x80C29650`, `0x80C29660`,
  `0x80C29670`) and links three states, so this table is complete.
- The first state-name symbol carries a stray backtick byte at its printed
  address (`0x80C62C2B`); the readable text begins one byte later and is
  `daEnBossKoopaDemoCage_c::StateID_Wait`. The Rust doc quotes the readable
  text, not the stray byte.

## Cluster 4 - the two `0x802DCBFC` adapter pairs (192 bytes)

| address | size | Rust | hidden `r12` | status | confidence |
|---|---:|---|---|---|---|
| `0x80C15BC0` | 48 | `fn_80c15bc0` | `param_1 + 0x18` | partial | medium |
| `0x80C15BF0` | 48 | `fn_80c15bf0` | `param_1 + 0x24` | partial | medium |
| `0x80C37C50` | 48 | `fn_80c37c50` | `param_1 + 0x18` | partial | medium |
| `0x80C37C80` | 48 | `fn_80c37c80` | `param_1 + 0x24` | partial | medium |

What is proven:

- The C body of all four is exactly `func_0x802dcbfc(param_2,param_2,param_1)`.
- The raw PowerPC text adds the detail wave 25 could not show. Each body is
  `stwu r1,-0x10(r1)` / `mflr r0` / `mr r5,r3` / `mr r3,r4` /
  `stw r0,0x14(r1)` / `addi r12,r5,0x18` or `0x24` / `bl 0x802DCBFC` / `nop` /
  epilogue. Since `r5` receives the first C argument, the hidden register is
  `param_1 + 0x18` in the first body of each pair and `param_1 + 0x24` in the
  second.
- The two words of each pair are adjacent in the REL data section: ELF offsets
  `0x5F1C8`/`0x5F1CC` for the Castle Roy pair and `0x66EE0`/`0x66EE4` for the
  Larry pair, next to the translated comparators `fn_80c15b00` and
  `fn_80c37b90`. The ownership is therefore inferred from the address
  neighborhood and the data layout; the exports themselves do not name it.
- `platform::call3` cannot load `r12`, so the visible call is translated but
  the body is `partial`; the exact adjustment is recorded in each doc comment.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C11DF0` | `FUN_80c11df0` | `fn_80c11df0` | Castle Roy state-id factory (`Enter`) | translated | high | `80c11df0_FUN_80c11df0.c` |
| `0x80C11E00` | `FUN_80c11e00` | `fn_80c11e00` | Castle Roy state-id factory (`Hide`) | translated | high | `80c11e00_FUN_80c11e00.c` |
| `0x80C11E10` | `FUN_80c11e10` | `fn_80c11e10` | Castle Roy state-id factory (`Peek`) | translated | high | `80c11e10_FUN_80c11e10.c` |
| `0x80C11E20` | `FUN_80c11e20` | `fn_80c11e20` | Castle Roy state-id factory (`Peek_Wait`) | translated | high | `80c11e20_FUN_80c11e20.c` |
| `0x80C11E30` | `FUN_80c11e30` | `fn_80c11e30` | Castle Roy state-id factory (`Feint`) | translated | high | `80c11e30_FUN_80c11e30.c` |
| `0x80C11E40` | `FUN_80c11e40` | `fn_80c11e40` | Castle Roy state-id factory (`Exit`) | translated | high | `80c11e40_FUN_80c11e40.c` |
| `0x80C11E50` | `FUN_80c11e50` | `fn_80c11e50` | Castle Roy state-id factory (`Drop`) | translated | high | `80c11e50_FUN_80c11e50.c` |
| `0x80C11E60` | `FUN_80c11e60` | `fn_80c11e60` | Castle Roy state-id factory (`ShellAtk_Ed`) | translated | high | `80c11e60_FUN_80c11e60.c` |
| `0x80C34D30` | `FUN_80c34d30` | `fn_80c34d30` | Larry state-id factory (`Dance`) | translated | high | `80c34d30_FUN_80c34d30.c` |
| `0x80C34D40` | `FUN_80c34d40` | `fn_80c34d40` | Larry state-id factory (`DemoFall`) | translated | high | `80c34d40_FUN_80c34d40.c` |
| `0x80C34D50` | `FUN_80c34d50` | `fn_80c34d50` | Larry state-id factory (`DemoFallEd`) | translated | high | `80c34d50_FUN_80c34d50.c` |
| `0x80C34D60` | `FUN_80c34d60` | `fn_80c34d60` | Larry state-id factory (`DemoEscape_Ready`) | translated | high | `80c34d60_FUN_80c34d60.c` |
| `0x80C34D70` | `FUN_80c34d70` | `fn_80c34d70` | Larry state-id factory (`DemoEscape_Jump`) | translated | high | `80c34d70_FUN_80c34d70.c` |
| `0x80C34D80` | `FUN_80c34d80` | `fn_80c34d80` | Larry state-id factory (`ShellAtk_Ed`) | translated | high | `80c34d80_FUN_80c34d80.c` |
| `0x80C379A0` | `FUN_80c379a0` | `fn_80c379a0` | Larry state-id factory (`ShellAtk`) | translated | high | `80c379a0_FUN_80c379a0.c` |
| `0x80C379B0` | `FUN_80c379b0` | `fn_80c379b0` | Larry state-id factory (`DemoEscape_St`) | translated | high | `80c379b0_FUN_80c379b0.c` |
| `0x80C379C0` | `FUN_80c379c0` | `fn_80c379c0` | Larry state-id factory (`DieFumi`) | translated | high | `80c379c0_FUN_80c379c0.c` |
| `0x80C29650` | `FUN_80c29650` | `fn_80c29650` | cage state-id factory (`Wait`) | translated | high | `80c29650_FUN_80c29650.c` |
| `0x80C29660` | `FUN_80c29660` | `fn_80c29660` | cage state-id factory (`Magic`) | translated | high | `80c29660_FUN_80c29660.c` |
| `0x80C29670` | `FUN_80c29670` | `fn_80c29670` | cage state-id factory (`Open`) | translated | high | `80c29670_FUN_80c29670.c` |
| `0x80C15BC0` | `FUN_80c15bc0` | `fn_80c15bc0` | Castle Roy teardown forwarder (`r12=+0x18`) | partial | medium | `80c15bc0_FUN_80c15bc0.c` |
| `0x80C15BF0` | `FUN_80c15bf0` | `fn_80c15bf0` | Castle Roy teardown forwarder (`r12=+0x24`) | partial | medium | `80c15bf0_FUN_80c15bf0.c` |
| `0x80C37C50` | `FUN_80c37c50` | `fn_80c37c50` | Larry teardown forwarder (`r12=+0x18`) | partial | medium | `80c37c50_FUN_80c37c50.c` |
| `0x80C37C80` | `FUN_80c37c80` | `fn_80c37c80` | Larry teardown forwarder (`r12=+0x24`) | partial | medium | `80c37c80_FUN_80c37c80.c` |

## Unknowns

- **Shared word `0x80377A04`.** Eight Castle Roy states, six Larry states and
  all three cage states store it at `+0x30`. The exports return the value and
  never dereference or call it, so the Rust returns it as a `u32`. Whether it
  is a sentinel, a common vtable, or a data address is not proven; the module
  header records it as unknown.
- **Why the eight Castle Roy states share an id.** The registrar stores the
  same word for eight consecutive states, while the other twelve Castle Roy
  states store distinct ids. The export does not explain the split.
- **Adapter ownership.** No C function references the four adapters; their
  vtable slots are written through data initializers. The Castle Roy/Larry
  ownership is inferred from the address neighborhood and from the adjacent
  data words, not from a named caller.
- **Meaning of `r12` at `0x802DCBFC`.** The instruction that loads it is exact
  (`param_1 + 0x18` or `+0x24`), but how the imported stub consumes it is not
  proven. The bodies stay `partial` because `platform::call3` cannot set it.
- **State-name strings.** The `daEnBoss..._c::StateID_...` names are data, not
  code symbols. They are quoted only to tie each factory to the state its
  registrar builds.

## Remaining candidates

| address / family | size | why deferred |
|---|---:|---|
| 211 state-id factories | 12 B each | complete bodies, same template; not selected in this pass |
| 31 `0x802DCBFC` adapters | 48 B each | hidden `r12`; selected pairs are translated as `partial` |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

The next factory passes can take whole owner-proven blocks: Morton
(`0x80C42CE0..0x80C42D60`, `0x80C46510..0x80C46550`), Roy
(`0x80C468E0..0x80C46960`, `0x80C4A510..0x80C4A580`) and Wendy
(`0x80C4A8F0..0x80C4A930`, `0x80C4F000..0x80C4F0B0`). Each block is twelve-byte
constant returns and each address is called exactly once by its registrar
(`0x80C45810`, `0x80C49540`, `0x80C4E010`).

## Verification

- `cargo check` (via `%USERPROFILE%\.cargo\bin\cargo.exe`) exits 0; none of
  the crate's warnings name `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 24 evidence files were read in full; each is one constant `return` or
  the single `0x802DCBFC` call, and `functions.tsv` records size 12 for every
  factory and 48 for every adapter.
- The three registrars were re-read to confirm the one-call/one-`+0x30`-store
  pattern and the state names quoted above.
- The four adapters were checked against the raw text: each has the
  `addi r12,r5,0x18`/`0x24` instruction described in Cluster 4.
- The 24 rows in `ledger/agents/wave27-bosses.csv` match the Rust functions
  one to one: 20 `translated`/`high` and 4 `partial`/`medium`.
- `src/game/bosses.rs` now defines 514 REL bodies; the module header count was
  updated from 490 to 514.
- No files outside `src/game/bosses.rs`, this document and the wave-27 ledger
  were edited; `ledger/functions.csv` and `ledger/SUMMARY.md` were regenerated
  with `node tools/merge-ledger.mjs`.
