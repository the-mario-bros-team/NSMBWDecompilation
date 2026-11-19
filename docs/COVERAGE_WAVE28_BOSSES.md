# Coverage - Wave 28, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 20 bodies added to `src/game/bosses.rs` by the wave-28 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier work is
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE27_BOSSES.md`;
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
- **Confidence** rates the translation, not the guessed role. The fourteen
  factory bodies are one constant return each (`high`); the six adapters are
  exact apart from the hidden `r12` register (`medium`).
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. State names are quoted from the data-section strings
  that the owning registrar passes to `0x8015F7C0`.
- Start with the **summary**, then **why this selection**, then the two
  **clusters**, then the **function table**. Read **remaining candidates**
  before treating this pass as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 20 |
| text bytes added | 456 |
| status `translated` | 14 |
| status `partial` | 6 |
| status `stub` | 0 |
| confidence `high` | 14 |
| confidence `medium` | 6 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 534 |
| cumulative bytes in `game::bosses` | 123,840 |
| translated share of module bodies | 69.35% |
| translated share of module text bytes | 34.75% |

The pass closes the Morton state-id table and translates three
`0x802DCBFC` destructor pairs:

- the fourteen Morton entries called by the registrar `0x80C45810`, which
  completes that registrar's table;
- the Morton, Roy and Wendy teardown pairs at `0x80C42C20`/`0x80C42C50`,
  `0x80C467E0`/`0x80C46810` and `0x80C4A810`/`0x80C4A840`.

## Why this selection

The wave-27 close-out reported 256 untranslated bodies. Their composition was:

| remaining kind | count | sizes | complete control flow? |
|---|---|---:|---|
| state-id factories | 211 | 12 B each | yes |
| `0x802DCBFC` destructor adapters | 31 | 48 B each | visible call yes, hidden `r12` no |
| truncated or thunk exports | 14 | 4-72 B | no |

The owner-proven factory sets are the only remaining clean bodies. The wave-27
report named Morton, Roy and Wendy as the next whole blocks and recorded that
each address is called exactly once by its registrar. This pass takes the
complete Morton block, because its fourteen calls can be shown end to end, and
the three adapter pairs that sit next to the Morton, Roy and Wendy actor
factories and constructors:

| owner | registrar | states | factories in this pass |
|---|---|---:|---:|
| `daEnBossMorton_c` | `0x80C45810` | 14 | 14 |
| Morton teardown pair | - | - | 2 adapters |
| Roy teardown pair | - | - | 2 adapters |
| Wendy teardown pair | - | - | 2 adapters |

After this pass 236 of the original 304 bodies remain: 197 factories, 25
adapters and the same 14 truncated or thunk exports. The remaining factory
blocks include Roy's seventeen entries at `0x80C468E0..0x80C46960` and
`0x80C4A510..0x80C4A580`, and Wendy's seventeen entries at
`0x80C4A8F0..0x80C4A930` and `0x80C4F000..0x80C4F0B0`.

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

## Cluster 1 - Morton: the fourteen remaining factories (168 bytes)

| address | size | Rust | returns | state registered by `0x80C45810` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C42CE0` | 12 | `fn_80c42ce0` | `0x80377A04` | `daEnBossMorton_c::StateID_AirSpin` | translated | high |
| `0x80C42CF0` | 12 | `fn_80c42cf0` | `0x80377A04` | `daEnBossMorton_c::StateID_AirWait` | translated | high |
| `0x80C42D00` | 12 | `fn_80c42d00` | `0x80377A04` | `daEnBossMorton_c::StateID_QuakeFall` | translated | high |
| `0x80C42D10` | 12 | `fn_80c42d10` | `0x80377A04` | `daEnBossMorton_c::StateID_Quake` | translated | high |
| `0x80C42D20` | 12 | `fn_80c42d20` | `0x80377A04` | `daEnBossMorton_c::StateID_ShellAtk_Ed` | translated | high |
| `0x80C42D30` | 12 | `fn_80c42d30` | `0x80377A04` | `daEnBossMorton_c::StateID_DemoFall` | translated | high |
| `0x80C42D40` | 12 | `fn_80c42d40` | `0x80377A04` | `daEnBossMorton_c::StateID_DemoFallEd` | translated | high |
| `0x80C42D50` | 12 | `fn_80c42d50` | `0x80377A04` | `daEnBossMorton_c::StateID_DemoEscape_Ready` | translated | high |
| `0x80C42D60` | 12 | `fn_80c42d60` | `0x80377A04` | `daEnBossMorton_c::StateID_DemoEscape_Jump` | translated | high |
| `0x80C46510` | 12 | `fn_80c46510` | `0x80358804` | `daEnBossMorton_c::StateID_DemoEscape_St` | translated | high |
| `0x80C46520` | 12 | `fn_80c46520` | `0x80358584` | `daEnBossMorton_c::StateID_ShellAtk` | translated | high |
| `0x80C46530` | 12 | `fn_80c46530` | `0x80358204` | `daEnBossMorton_c::StateID_BigJump` | translated | high |
| `0x80C46540` | 12 | `fn_80c46540` | `0x80358184` | `daEnBossMorton_c::StateID_Jump` | translated | high |
| `0x80C46550` | 12 | `fn_80c46550` | `0x803576F4` | `daEnBossMorton_c::StateID_DieFumi` | translated | high |

What is proven:

- Every body is exactly `return <word>;` with no load or store. The raw text
  is `lis r3,hi` / `addi r3,r3,lo` / `blr` in all fourteen bodies.
- The registrar `0x80C45810` makes exactly fourteen factory calls and links
  fourteen states. In every block the C reads
  `uVar10 = FUN_80c42cxx();` (or `FUN_80c465xx()`), passes the quoted
  `StateID` string to `0x8015F7C0`, copies the nine descriptor words, writes
  the vtable, and then stores `uVar10` at the state's `+0x30` word.
- The calls at `0x80C42CE0..0x80C42D20` and `0x80C42D30..0x80C42D60` share
  `0x80377A04`; the five calls at `0x80C46510..0x80C46550` return five
  distinct words.
- The registrar uses the name strings at data offsets `0x6928F` through
  `0x69474`. The first symbol carries a stray backtick byte at its printed
  address; the Rust doc quotes the readable text, not the stray byte.

## Cluster 2 - the three `0x802DCBFC` adapter pairs (288 bytes)

| address | size | Rust | hidden `r12` | inferred family | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C42C20` | 48 | `fn_80c42c20` | `param_1 + 0x18` | Morton | partial | medium |
| `0x80C42C50` | 48 | `fn_80c42c50` | `param_1 + 0x24` | Morton | partial | medium |
| `0x80C467E0` | 48 | `fn_80c467e0` | `param_1 + 0x18` | Roy | partial | medium |
| `0x80C46810` | 48 | `fn_80c46810` | `param_1 + 0x24` | Roy | partial | medium |
| `0x80C4A810` | 48 | `fn_80c4a810` | `param_1 + 0x18` | Wendy | partial | medium |
| `0x80C4A840` | 48 | `fn_80c4a840` | `param_1 + 0x24` | Wendy | partial | medium |

What is proven:

- The C body of all six is exactly `func_0x802dcbfc(param_2,param_2,param_1)`.
- The raw PowerPC text repeats the Castle Roy and Larry shape exactly:
  `stwu r1,-0x10(r1)` / `mflr r0` / `mr r5,r3` / `mr r3,r4` /
  `stw r0,0x14(r1)` / `addi r12,r5,0x18` or `0x24` / `bl 0x802DCBFC` / `nop` /
  epilogue. Since `r5` receives the first C argument, the hidden register is
  `param_1 + 0x18` in the first body of each pair and `param_1 + 0x24` in the
  second.
- The two words of each pair are adjacent in the REL data section: ELF
  offsets `0x694B0`/`0x694B4` and `0x694E4`/`0x694E8` for Morton,
  `0x6A4CC`/`0x6A4D0` and `0x6A500`/`0x6A504` for Roy, and
  `0x6B6B0`/`0x6B6B4` and `0x6B6E4`/`0x6B6E8` for Wendy.
- The Morton pair sits immediately before the `0xEA0`-byte factory
  [`fn_80c42c80`] and the Morton constructor [`fn_80c42d70`]; the Roy pair
  sits before the base constructor [`fn_80c46840`] and the
  `0x80C468E0` factory block; the Wendy pair sits before the base constructor
  [`fn_80c4a870`] and the `0x80C4A8F0` factory block. No C function references
  the adapters, so the family assignment is inferred from this layout.
- `platform::call3` cannot load `r12`, so the visible call is translated but
  the body is `partial`; the exact adjustment is recorded in each doc comment.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C42CE0` | `FUN_80c42ce0` | `fn_80c42ce0` | Morton state-id factory (`AirSpin`) | translated | high | `80c42ce0_FUN_80c42ce0.c` |
| `0x80C42CF0` | `FUN_80c42cf0` | `fn_80c42cf0` | Morton state-id factory (`AirWait`) | translated | high | `80c42cf0_FUN_80c42cf0.c` |
| `0x80C42D00` | `FUN_80c42d00` | `fn_80c42d00` | Morton state-id factory (`QuakeFall`) | translated | high | `80c42d00_FUN_80c42d00.c` |
| `0x80C42D10` | `FUN_80c42d10` | `fn_80c42d10` | Morton state-id factory (`Quake`) | translated | high | `80c42d10_FUN_80c42d10.c` |
| `0x80C42D20` | `FUN_80c42d20` | `fn_80c42d20` | Morton state-id factory (`ShellAtk_Ed`) | translated | high | `80c42d20_FUN_80c42d20.c` |
| `0x80C42D30` | `FUN_80c42d30` | `fn_80c42d30` | Morton state-id factory (`DemoFall`) | translated | high | `80c42d30_FUN_80c42d30.c` |
| `0x80C42D40` | `FUN_80c42d40` | `fn_80c42d40` | Morton state-id factory (`DemoFallEd`) | translated | high | `80c42d40_FUN_80c42d40.c` |
| `0x80C42D50` | `FUN_80c42d50` | `fn_80c42d50` | Morton state-id factory (`DemoEscape_Ready`) | translated | high | `80c42d50_FUN_80c42d50.c` |
| `0x80C42D60` | `FUN_80c42d60` | `fn_80c42d60` | Morton state-id factory (`DemoEscape_Jump`) | translated | high | `80c42d60_FUN_80c42d60.c` |
| `0x80C46510` | `FUN_80c46510` | `fn_80c46510` | Morton state-id factory (`DemoEscape_St`) | translated | high | `80c46510_FUN_80c46510.c` |
| `0x80C46520` | `FUN_80c46520` | `fn_80c46520` | Morton state-id factory (`ShellAtk`) | translated | high | `80c46520_FUN_80c46520.c` |
| `0x80C46530` | `FUN_80c46530` | `fn_80c46530` | Morton state-id factory (`BigJump`) | translated | high | `80c46530_FUN_80c46530.c` |
| `0x80C46540` | `FUN_80c46540` | `fn_80c46540` | Morton state-id factory (`Jump`) | translated | high | `80c46540_FUN_80c46540.c` |
| `0x80C46550` | `FUN_80c46550` | `fn_80c46550` | Morton state-id factory (`DieFumi`) | translated | high | `80c46550_FUN_80c46550.c` |
| `0x80C42C20` | `FUN_80c42c20` | `fn_80c42c20` | Morton teardown forwarder (`r12=+0x18`) | partial | medium | `80c42c20_FUN_80c42c20.c` |
| `0x80C42C50` | `FUN_80c42c50` | `fn_80c42c50` | Morton teardown forwarder (`r12=+0x24`) | partial | medium | `80c42c50_FUN_80c42c50.c` |
| `0x80C467E0` | `FUN_80c467e0` | `fn_80c467e0` | Roy teardown forwarder (`r12=+0x18`) | partial | medium | `80c467e0_FUN_80c467e0.c` |
| `0x80C46810` | `FUN_80c46810` | `fn_80c46810` | Roy teardown forwarder (`r12=+0x24`) | partial | medium | `80c46810_FUN_80c46810.c` |
| `0x80C4A810` | `FUN_80c4a810` | `fn_80c4a810` | Wendy teardown forwarder (`r12=+0x18`) | partial | medium | `80c4a810_FUN_80c4a810.c` |
| `0x80C4A840` | `FUN_80c4a840` | `fn_80c4a840` | Wendy teardown forwarder (`r12=+0x24`) | partial | medium | `80c4a840_FUN_80c4a840.c` |

## Unknowns

- **Shared word `0x80377A04`.** Nine Morton states store it at `+0x30`, as do
  eight Castle Roy states, six Larry states and all three cage states. The
  exports return the value and never dereference or call it, so the Rust
  returns it as a `u32`. Whether it is a sentinel, a common vtable, or a data
  address is not proven.
- **Why nine Morton states share an id.** Five consecutive Morton entries use
  five distinct ids while the other nine share one. The registrar stores the
  values but the export does not explain the split.
- **Adapter ownership.** No C function references the six adapters; their
  vtable slots are written through data initializers. The Morton/Roy/Wendy
  ownership is inferred from the address neighborhood and the adjacent data
  words, not from a named caller.
- **Meaning of `r12` at `0x802DCBFC`.** The instruction that loads it is exact
  (`param_1 + 0x18` or `+0x24`), but how the imported stub consumes it is not
  proven. The bodies stay `partial` because `platform::call3` cannot set it.
- **State-name strings.** The `daEnBoss..._c::StateID_...` names are data, not
  code symbols. They are quoted only to tie each factory to the state its
  registrar builds.

## Remaining candidates

| address / family | size | why deferred |
|---|---|---|
| 197 state-id factories | 12 B each | complete bodies; includes Roy's 17 and Wendy's 17 |
| 25 `0x802DCBFC` adapters | 48 B each | hidden `r12`; this pass selected three pairs |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

The next factory passes can finish the remaining whole tables: Roy
(`0x80C468E0..0x80C46960`, `0x80C4A510..0x80C4A580`) and Wendy
(`0x80C4A8F0..0x80C4A930`, `0x80C4F000..0x80C4F0B0`). Each block is twelve-byte
constant returns and each address is called exactly once by its registrar
(`0x80C49540`, `0x80C4E010`).

## Verification

- `cargo check` (via `%USERPROFILE%\.cargo\bin\cargo.exe`) exits 0; no
  diagnostic names `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 20 evidence files were read in full: fourteen are one constant
  `return`, six are the single `0x802DCBFC` call.
- The Morton registrar `0x80C45810` was re-read to confirm the fourteen
  one-call/one-`+0x30`-store blocks and the state names quoted above.
- The raw text of every selected body was checked in
  `work/rels/elf/d_en_bossNP.elf`: the factories are
  `lis`/`addi`/`blr`, and each adapter contains `addi r12,r5,0x18` or `0x24`.
- The six adapter data words were located in the REL data section at the ELF
  offsets quoted in Cluster 2.
- The 20 rows in `ledger/agents/wave28-bosses.csv` match the Rust functions
  one to one: 14 `translated`/`high` and 6 `partial`/`medium`.
- `src/game/bosses.rs` now defines 534 REL bodies; the module header count was
  updated from 514 to 534.
- No files outside `src/game/bosses.rs`, this document and the wave-28 ledger
  were edited.
