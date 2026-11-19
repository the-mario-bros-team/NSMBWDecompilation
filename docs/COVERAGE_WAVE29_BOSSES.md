# Coverage - Wave 29, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 24 bodies added to `src/game/bosses.rs` by the wave-29 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier work is
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE28_BOSSES.md`;
  this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader chooses a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8015F7C0`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented; `partial` means the
  visible call is exact but one ABI operand is not representable.
- **Confidence** rates the translation, not the guessed role. The
  twenty-two factory bodies are one constant return each (`high`); the two
  adapters are exact apart from the hidden `r12` register (`medium`).
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. State names are quoted from the data-section strings
  that the owning registrar passes to `0x8015F7C0`.
- Start with the **summary**, then **why this selection**, then **cluster 1**
  (Roy), **cluster 2** (Wendy) and **cluster 3** (the Larry pair), then the
  **function table**. Read **remaining candidates** before treating this pass
  as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 24 |
| text bytes added | 360 |
| status `translated` | 22 |
| status `partial` | 2 |
| status `stub` | 0 |
| confidence `high` | 22 |
| confidence `medium` | 2 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 558 |
| cumulative bytes in `game::bosses` | 124,200 |
| translated share of module bodies | 72.47% |
| translated share of module text bytes | 34.85% |

This pass completes the Roy state-id table and Wendy's first factory block,
and translates the Larry teardown pair:

- the seventeen Roy entries called by the registrar `0x80C49540`, which
  completes that registrar's table;
- the five Wendy entries at `0x80C4A8F0..0x80C4A930`, the first of that
  registrar's two factory blocks;
- the Larry `0x802DCBFC` adapter pair at `0x80C34C70`/`0x80C34CA0`.

## Why this selection

The wave-28 close-out reported 236 untranslated bodies. Their composition was:

| remaining kind | count | sizes | complete control flow? |
|---|---|---:|---|
| state-id factories | 197 | 12 B each | yes |
| `0x802DCBFC` destructor adapters | 25 | 48 B each | visible call yes, hidden `r12` no |
| truncated or thunk exports | 14 | 4-72 B | no |

One correction to that inventory: the 48-byte body at `0x80C38230` is not a
`0x802DCBFC` adapter. Its export is
`(**(code **)(*(int *)(param_1 + 0x394) + 0x10))(); return 1;`, a virtual call
through the object's `+0x394` block. There are therefore 24 real adapters,
not 25.

The owner-proven factory sets remain the cleanest bodies, so this pass takes
two complete, contiguous blocks that a single registrar call list closes:

| owner | registrar | states | factories in this pass |
|---|---|---|---:|
| `daEnBossRoy_c` | `0x80C49540` | 17 | 17 |
| `daEnBossWendy_c` | `0x80C4E010` | 17 | 5 |

The Roy block is the whole table, so no Roy factory remains. The Wendy block
is the five-entry run of twelve-byte factories at `0x80C4A8F0..0x80C4A930`;
the other twelve Wendy addresses at `0x80C4F000..0x80C4F0B0` are left as a
single contiguous follow-up.

The two adapters are the Larry pair immediately before the `0xE98`-byte Larry
actor factory `0x80C34CD0` and the Larry constructor `0x80C34D90`. They are
the same repeated forwarder shape as the six Morton/Roy/Wendy adapters that
wave 28 translated, so the body translation is proven even though the hidden
`r12` operand stays unmodelled.

After this pass 212 of the original 304 bodies remain: 175 factories, 24
adapters, the non-adapter 48-byte body `0x80C38230` and the same 14 truncated
or thunk exports.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |
| BSS end (exclusive) | `0x80C7117C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it.

## Cluster 1 - Roy: the seventeen remaining factories (204 bytes)

| address | size | Rust | returns | state registered by `0x80C49540` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C468E0` | 12 | `fn_80c468e0` | `0x80377A04` | `daEnBossRoy_c::StateID_AirSpin` | translated | high |
| `0x80C468F0` | 12 | `fn_80c468f0` | `0x80377A04` | `daEnBossRoy_c::StateID_AirWait` | translated | high |
| `0x80C46900` | 12 | `fn_80c46900` | `0x80377A04` | `daEnBossRoy_c::StateID_QuakeFall` | translated | high |
| `0x80C46910` | 12 | `fn_80c46910` | `0x80377A04` | `daEnBossRoy_c::StateID_Quake` | translated | high |
| `0x80C46920` | 12 | `fn_80c46920` | `0x80377A04` | `daEnBossRoy_c::StateID_DemoFall` | translated | high |
| `0x80C46930` | 12 | `fn_80c46930` | `0x80377A04` | `daEnBossRoy_c::StateID_DemoFallEd` | translated | high |
| `0x80C46940` | 12 | `fn_80c46940` | `0x80377A04` | `daEnBossRoy_c::StateID_DemoEscape_St` | translated | high |
| `0x80C46950` | 12 | `fn_80c46950` | `0x80377A04` | `daEnBossRoy_c::StateID_DemoEscape_Ready` | translated | high |
| `0x80C46960` | 12 | `fn_80c46960` | `0x80377A04` | `daEnBossRoy_c::StateID_DemoEscape_Jump` | translated | high |
| `0x80C4A510` | 12 | `fn_80c4a510` | `0x80358584` | `daEnBossRoy_c::StateID_ShellAtk` | translated | high |
| `0x80C4A520` | 12 | `fn_80c4a520` | `0x80358804` | `daEnBossRoy_c::StateID_ShellAtk_Ed` | translated | high |
| `0x80C4A530` | 12 | `fn_80c4a530` | `0x803587C4` | `daEnBossRoy_c::StateID_DemoIkaku_Wait` | translated | high |
| `0x80C4A540` | 12 | `fn_80c4a540` | `0x80358784` | `daEnBossRoy_c::StateID_DemoIkaku` | translated | high |
| `0x80C4A550` | 12 | `fn_80c4a550` | `0x80358744` | `daEnBossRoy_c::StateID_DemoAwake_Wait` | translated | high |
| `0x80C4A560` | 12 | `fn_80c4a560` | `0x80358704` | `daEnBossRoy_c::StateID_DemoAwake` | translated | high |
| `0x80C4A570` | 12 | `fn_80c4a570` | `0x80358204` | `daEnBossRoy_c::StateID_BigJump` | translated | high |
| `0x80C4A580` | 12 | `fn_80c4a580` | `0x803576F4` | `daEnBossRoy_c::StateID_DieFumi` | translated | high |

What is proven:

- Every body is exactly `return <word>;` with no load or store. The raw text
  is `lis r3,0` / `addi r3,r3,0` / `blr` in all seventeen bodies; the
  immediates are filled by REL relocations, which Ghidra has already applied.
- The registrar `0x80C49540` makes exactly seventeen factory calls and links
  seventeen states. In every block the C reads
  `uVar10 = FUN_80c468xx();` (or `FUN_80c4a5xx()`), passes the quoted
  `StateID` string to `0x8015F7C0`, copies the nine descriptor words, writes
  the vtable `0x80C6A684`, and then stores `uVar10` at the state's `+0x30`
  word.
- Nine consecutive entries (`0x80C468E0..0x80C46960`) share `0x80377A04`;
  the eight entries at `0x80C4A510..0x80C4A580` return eight distinct words.
- The name strings are contiguous in the data section at file offsets
  `0x6A423..0x6A660`. The `AirSpin` string carries a stray backtick byte
  before `da`; the registrar passes `...+ 1`, so the Rust doc quotes the
  readable text.

## Cluster 2 - Wendy: the first five factories (60 bytes)

| address | size | Rust | returns | state registered by `0x80C4E010` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C4A8F0` | 12 | `fn_80c4a8f0` | `0x80377A04` | `daEnBossWendy_c::StateID_DemoFall` | translated | high |
| `0x80C4A900` | 12 | `fn_80c4a900` | `0x80377A04` | `daEnBossWendy_c::StateID_DemoFallEd` | translated | high |
| `0x80C4A910` | 12 | `fn_80c4a910` | `0x80377A04` | `daEnBossWendy_c::StateID_DemoEscape_Ready` | translated | high |
| `0x80C4A920` | 12 | `fn_80c4a920` | `0x80377A04` | `daEnBossWendy_c::StateID_DemoEscape_Jump` | translated | high |
| `0x80C4A930` | 12 | `fn_80c4a930` | `0x80377A04` | `daEnBossWendy_c::StateID_ShellAtk_Ed` | translated | high |

What is proven:

- Each body is one constant return, with the same `lis`/`addi`/`blr` text as
  the Roy block.
- `0x80C4E010` calls each of these five addresses exactly once and stores the
  result at the corresponding state's `+0x30` word, following the same
  nine-word copy and the Wendy vtable `0x80C6B8AC`.
- All five return `0x80377A04`. The registrar's `DieFumi` string (used by the
  deferred block at `0x80C4F0B0`) carries a stray backtick byte; none of the
  five strings in this block does.

## Cluster 3 - the Larry `0x802DCBFC` pair (96 bytes)

| address | size | Rust | hidden `r12` | inferred family | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C34C70` | 48 | `fn_80c34c70` | `param_1 + 0x18` | Larry | partial | medium |
| `0x80C34CA0` | 48 | `fn_80c34ca0` | `param_1 + 0x24` | Larry | partial | medium |

What is proven:

- The C body of both is exactly `func_0x802dcbfc(param_2,param_2,param_1)`.
- The raw PowerPC text repeats the Morton/Roy/Wendy adapter shape exactly:
  `stwu r1,-0x10(r1)` / `mflr r0` / `mr r5,r3` / `mr r3,r4` /
  `stw r0,0x14(r1)` / `addi r12,r5,0x18` or `0x24` / `bl 0x802DCBFC` / `nop` /
  epilogue. Since `r5` receives the first C argument, the hidden register is
  `param_1 + 0x18` in the first body and `param_1 + 0x24` in the second.
- The pair sits immediately before the `0xE98`-byte Larry actor factory
  `0x80C34CD0` and the Larry constructor `0x80C34D90`; `0x80C34F10` is the
  Larry model loader. No C function references the adapters, so the family
  assignment is inferred from this layout, not from a named caller.
- `platform::call3` cannot load `r12`, so the visible call is translated but
  the body is `partial`; the exact adjustment is recorded in each doc comment.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C468E0` | `FUN_80c468e0` | `fn_80c468e0` | Roy state-id factory (`AirSpin`) | translated | high | `80c468e0_FUN_80c468e0.c` |
| `0x80C468F0` | `FUN_80c468f0` | `fn_80c468f0` | Roy state-id factory (`AirWait`) | translated | high | `80c468f0_FUN_80c468f0.c` |
| `0x80C46900` | `FUN_80c46900` | `fn_80c46900` | Roy state-id factory (`QuakeFall`) | translated | high | `80c46900_FUN_80c46900.c` |
| `0x80C46910` | `FUN_80c46910` | `fn_80c46910` | Roy state-id factory (`Quake`) | translated | high | `80c46910_FUN_80c46910.c` |
| `0x80C46920` | `FUN_80c46920` | `fn_80c46920` | Roy state-id factory (`DemoFall`) | translated | high | `80c46920_FUN_80c46920.c` |
| `0x80C46930` | `FUN_80c46930` | `fn_80c46930` | Roy state-id factory (`DemoFallEd`) | translated | high | `80c46930_FUN_80c46930.c` |
| `0x80C46940` | `FUN_80c46940` | `fn_80c46940` | Roy state-id factory (`DemoEscape_St`) | translated | high | `80c46940_FUN_80c46940.c` |
| `0x80C46950` | `FUN_80c46950` | `fn_80c46950` | Roy state-id factory (`DemoEscape_Ready`) | translated | high | `80c46950_FUN_80c46950.c` |
| `0x80C46960` | `FUN_80c46960` | `fn_80c46960` | Roy state-id factory (`DemoEscape_Jump`) | translated | high | `80c46960_FUN_80c46960.c` |
| `0x80C4A510` | `FUN_80c4a510` | `fn_80c4a510` | Roy state-id factory (`ShellAtk`) | translated | high | `80c4a510_FUN_80c4a510.c` |
| `0x80C4A520` | `FUN_80c4a520` | `fn_80c4a520` | Roy state-id factory (`ShellAtk_Ed`) | translated | high | `80c4a520_FUN_80c4a520.c` |
| `0x80C4A530` | `FUN_80c4a530` | `fn_80c4a530` | Roy state-id factory (`DemoIkaku_Wait`) | translated | high | `80c4a530_FUN_80c4a530.c` |
| `0x80C4A540` | `FUN_80c4a540` | `fn_80c4a540` | Roy state-id factory (`DemoIkaku`) | translated | high | `80c4a540_FUN_80c4a540.c` |
| `0x80C4A550` | `FUN_80c4a550` | `fn_80c4a550` | Roy state-id factory (`DemoAwake_Wait`) | translated | high | `80c4a550_FUN_80c4a550.c` |
| `0x80C4A560` | `FUN_80c4a560` | `fn_80c4a560` | Roy state-id factory (`DemoAwake`) | translated | high | `80c4a560_FUN_80c4a560.c` |
| `0x80C4A570` | `FUN_80c4a570` | `fn_80c4a570` | Roy state-id factory (`BigJump`) | translated | high | `80c4a570_FUN_80c4a570.c` |
| `0x80C4A580` | `FUN_80c4a580` | `fn_80c4a580` | Roy state-id factory (`DieFumi`) | translated | high | `80c4a580_FUN_80c4a580.c` |
| `0x80C4A8F0` | `FUN_80c4a8f0` | `fn_80c4a8f0` | Wendy state-id factory (`DemoFall`) | translated | high | `80c4a8f0_FUN_80c4a8f0.c` |
| `0x80C4A900` | `FUN_80c4a900` | `fn_80c4a900` | Wendy state-id factory (`DemoFallEd`) | translated | high | `80c4a900_FUN_80c4a900.c` |
| `0x80C4A910` | `FUN_80c4a910` | `fn_80c4a910` | Wendy state-id factory (`DemoEscape_Ready`) | translated | high | `80c4a910_FUN_80c4a910.c` |
| `0x80C4A920` | `FUN_80c4a920` | `fn_80c4a920` | Wendy state-id factory (`DemoEscape_Jump`) | translated | high | `80c4a920_FUN_80c4a920.c` |
| `0x80C4A930` | `FUN_80c4a930` | `fn_80c4a930` | Wendy state-id factory (`ShellAtk_Ed`) | translated | high | `80c4a930_FUN_80c4a930.c` |
| `0x80C34C70` | `FUN_80c34c70` | `fn_80c34c70` | Larry teardown forwarder (`r12=+0x18`) | partial | medium | `80c34c70_FUN_80c34c70.c` |
| `0x80C34CA0` | `FUN_80c34ca0` | `fn_80c34ca0` | Larry teardown forwarder (`r12=+0x24`) | partial | medium | `80c34ca0_FUN_80c34ca0.c` |

## Unknowns

- **Shared word `0x80377A04`.** All nine `0x80C468E0..0x80C46960` bodies and all
  five Wendy bodies store it at `+0x30`, as do eight Castle Roy states, six
  Larry states and all three cage states. The exports return the value and
  never dereference or call it, so the Rust returns it as a `u32`. Whether it
  is a sentinel, a common vtable, or a data address is not proven.
- **Why nine Roy states share an id.** Nine consecutive Roy entries share one
  word while the other eight use distinct ids. The registrar stores the values
  but the export does not explain the split.
- **Larry adapter ownership.** No C function references the two adapters;
  their vtable slots are written through data initializers. The Larry
  ownership is inferred from the address neighborhood, not from a named
  caller.
- **Meaning of `r12` at `0x802DCBFC`.** The instruction that loads it is exact
  (`param_1 + 0x18` or `+0x24`), but how the imported stub consumes it is not
  proven. The bodies stay `partial` because `platform::call3` cannot set it.
- **State-name strings.** The `daEnBoss..._c::StateID_...` names are data, not
  code symbols. They are quoted only to tie each factory to the state its
  registrar builds. Roy's `AirSpin` and Wendy's `DieFumi` begin with a stray
  backtick byte; the doc quotes the readable text.

## Remaining candidates

| address / family | size | why deferred |
|---|---|---|
| 175 state-id factories | 12 B each | complete bodies; Wendy's 12 at `0x80C4F000..0x80C4F0B0` are the next contiguous block |
| 24 `0x802DCBFC` adapters | 48 B each | hidden `r12`; this pass selected one pair |
| `0x80C38230` | 48 B | not an adapter: virtual call through `*(param_1 + 0x394)`, returns `1` |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

The 175 remaining factories group into these contiguous runs (count in
parentheses): `0x80C06820` (8), `0x80C09D80` (7), `0x80C0DFE0` (9),
`0x80C118F0` (10), `0x80C18230` (8), `0x80C185F0` (5), `0x80C1C2E0` (13),
`0x80C1C6C0` (8), `0x80C211E0` (16), `0x80C218D0` (16), `0x80C292D0` (1),
`0x80C2A4A0` (4), `0x80C2C430` (6), `0x80C2FDB0` (2), `0x80C32050` (2),
`0x80C349B0` (4), `0x80C37D60` (9), `0x80C3D1F0` (13), `0x80C3D620` (17),
`0x80C42950` (5) and `0x80C4F000` (12).

Each of those blocks is a sequence of twelve-byte constant returns. The next
coherent moves are Wendy's twelve-entry block at `0x80C4F000..0x80C4F0B0`,
then the seventeen-entry Ludwig block at `0x80C3D620..0x80C3D720` and the
sixteen-entry blocks at `0x80C211E0` and `0x80C218D0`.

## Verification

- `cargo check` (via `%USERPROFILE%\.cargo\bin\cargo.exe`) exits 0; no
  diagnostic names `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 24 evidence files were read in full: twenty-two are one constant
  `return`, two are the single `0x802DCBFC` call.
- The Roy registrar `0x80C49540` was parsed call-by-call: it makes exactly the
  seventeen selected factory calls, one per `func_0x8015f7c0` state name, and
  stores each result at `+0x30`.
- The Wendy registrar `0x80C4E010` was parsed the same way: the five selected
  addresses are each called once, in the order listed in cluster 2.
- The raw text of every selected body was checked in
  `work/rels/d_en_bossNP.rel`: the factories are `lis`/`addi`/`blr`, and each
  adapter contains `addi r12,r5,0x18` or `0x24`.
- The full state-name strings were read from the data section at file offsets
  `0x6A423..0x6A660` and `0x6B770..0x6B884`; the two leading backtick bytes
  are recorded above.
- The 24 rows in `ledger/agents/wave29-bosses.csv` match the Rust functions
  one to one: 22 `translated`/`high` and 2 `partial`/`medium`.
- `src/game/bosses.rs` now defines 558 REL bodies; the module header count was
  updated from 534 to 558.
- No files outside `src/game/bosses.rs`, this document and the wave-29 ledger
  were edited.
