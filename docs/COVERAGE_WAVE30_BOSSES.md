# Coverage - Wave 30, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 24 bodies added to `src/game/bosses.rs` by the wave-30 pass
  over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier work is
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` and
  `docs/COVERAGE_WAVE14_BOSSES.md` through `docs/COVERAGE_WAVE29_BOSSES.md`;
  this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader chooses a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8015F7C0` or `0x802DCBFC`)
  are absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and every observable store are represented; `partial` means the
  visible call is exact but one ABI operand is not representable.
- **Confidence** rates the translation, not the guessed role. The twelve
  factory bodies are one constant return each (`high`); the twelve adapters
  are exact apart from the hidden `r12` register (`medium`).
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. State names are quoted from the data-section strings
  that the owning registrar passes to `0x8015F7C0`.
- Start with the **summary**, then the **inventory correction**, then
  **cluster 1** (Wendy factories) and **cluster 2** (adapter pairs), then the
  **function table**. Read **remaining candidates** before treating this pass
  as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 24 |
| text bytes added | 720 |
| status `translated` | 12 |
| status `partial` | 12 |
| status `stub` | 0 |
| confidence `high` | 12 |
| confidence `medium` | 12 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 582 |
| cumulative bytes in `game::bosses` | 124,920 |
| translated share of module bodies | 75.58% |
| translated share of module text bytes | 35.05% |

This pass completes the Wendy state-id table and clears twelve more
`0x802DCBFC` destructor adapters:

- the twelve Wendy entries at `0x80C4F000..0x80C4F0B0`, which close the
  seventeen-state table registered by `0x80C4E010`;
- six adapter pairs at `0x80C1C630`, `0x80C29560`, `0x80C2C2F0`,
  `0x80C2E430`, `0x80C30050` and `0x80C322F0` (`0x18`/`0x24` hidden-`r12`
  adjustments in each pair).

## Inventory correction for adapter bodies

Wave 29 reported 24 remaining `0x802DCBFC` adapters. A fresh pass over
`functions.tsv` and the export bodies gives the exact figures:

| item | count |
|---|---:|
| 48-byte exports whose whole body is `func_0x802dcbfc(param_2,param_2,param_1)` | 44 |
| translated before this pass (ledger `game::bosses`) | 22 |
| untranslated before this pass | 22 |
| translated in this pass | 12 |
| untranslated after this pass | 10 |

The untranslated count of 212 before this pass therefore decomposes as
175 state-id factories + 22 adapters + one non-adapter 48-byte body
(`0x80C38230`) + 14 truncated/thunk exports = 212. Wave 29's "24 adapters"
was two too high.

## Why this selection

The clean gameplay handlers are exhausted. The remaining small, complete
bodies are the state-id factories and the `0x802DCBFC` forwarder pairs, so
this pass takes two complete, contiguous groups:

| group | owner / site | bodies |
|---|---|---:|
| Wendy factories `0x80C4F000..0x80C4F0B0` | `daEnBossWendy_c`, registrar `0x80C4E010` | 12 |
| adapter pairs at the six sites below | six translated factory/constructor sites | 12 |

The Wendy block is a single contiguous run of twelve-byte factories, and the
registrar `0x80C4E010` calls all twelve exactly once. The six adapter pairs
are the ones that sit immediately before factory/constructor sites already
translated in earlier waves, so the ownership context is recoverable from the
surrounding code rather than guessed from a distant address.

## Cluster 1 - Wendy: the twelve remaining factories (144 bytes)

| address | size | Rust | returns | state registered by `0x80C4E010` | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C4F000` | 12 | `fn_80c4f000` | `0x80358584` | `daEnBossWendy_c::StateID_ShellAtk` | translated | high |
| `0x80C4F010` | 12 | `fn_80c4f010` | `0x80358544` | `daEnBossWendy_c::StateID_ShellAtk_St` | translated | high |
| `0x80C4F020` | 12 | `fn_80c4f020` | `0x80358804` | `daEnBossWendy_c::StateID_DemoEscape_St` | translated | high |
| `0x80C4F030` | 12 | `fn_80c4f030` | `0x803587C4` | `daEnBossWendy_c::StateID_DemoIkaku_Wait` | translated | high |
| `0x80C4F040` | 12 | `fn_80c4f040` | `0x80358784` | `daEnBossWendy_c::StateID_DemoIkaku` | translated | high |
| `0x80C4F050` | 12 | `fn_80c4f050` | `0x80358744` | `daEnBossWendy_c::StateID_DemoAwake_Wait` | translated | high |
| `0x80C4F060` | 12 | `fn_80c4f060` | `0x80358704` | `daEnBossWendy_c::StateID_DemoAwake` | translated | high |
| `0x80C4F070` | 12 | `fn_80c4f070` | `0x80358204` | `daEnBossWendy_c::StateID_BigJump` | translated | high |
| `0x80C4F080` | 12 | `fn_80c4f080` | `0x80358184` | `daEnBossWendy_c::StateID_Jump` | translated | high |
| `0x80C4F090` | 12 | `fn_80c4f090` | `0x80358344` | `daEnBossWendy_c::StateID_Attack` | translated | high |
| `0x80C4F0A0` | 12 | `fn_80c4f0a0` | `0x803585C4` | `daEnBossWendy_c::StateID_ShellOut` | translated | high |
| `0x80C4F0B0` | 12 | `fn_80c4f0b0` | `0x803576F4` | `daEnBossWendy_c::StateID_DieFumi` | translated | high |

What is proven:

- Every body is exactly `return <word>;` with no load or store. The raw text
  is `lis r3,0` / `addi r3,r3,0` / `blr` in all twelve bodies; the immediates
  are filled by REL relocations, which Ghidra has already applied.
- The registrar `0x80C4E010` calls each of the twelve addresses exactly once,
  in descending address order, passes the quoted state name to `0x8015F7C0`,
  copies the nine descriptor words, installs the Wendy vtable `0x80C6B8AC`,
  and stores the factory result at that state's `+0x30` word.
- All twelve returns are distinct. Together with the five wave-29 entries
  (`0x80377A04` each), they account for the registrar's seventeen states.
- `DieFumi` is registered from `0x80C6B623 + 1`; byte `0x80C6B623` is a stray
  backtick, so the readable text is quoted above.

## Cluster 2 - six `0x802DCBFC` adapter pairs (576 bytes)

| address | size | Rust | hidden `r12` | adjacent translated site | status | confidence |
|---|---:|---|---|---|---|---|
| `0x80C1C630` | 48 | `fn_80c1c630` | `param_1 + 0x18` | 0x1238-byte actor factory `0x80C1C690`, constructor `0x80C1C740` | partial | medium |
| `0x80C1C660` | 48 | `fn_80c1c660` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C29560` | 48 | `fn_80c29560` | `param_1 + 0x18` | constructor `0x80C295C0` (allocation `0x6B0`) | partial | medium |
| `0x80C29590` | 48 | `fn_80c29590` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C2C2F0` | 48 | `fn_80c2c2f0` | `param_1 + 0x18` | constructor `0x80C2C350` (allocation `0x768`) | partial | medium |
| `0x80C2C320` | 48 | `fn_80c2c320` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C2E430` | 48 | `fn_80c2e430` | `param_1 + 0x18` | W4 factory `0x80C2E490` (`0xD98`), constructor `0x80C2E4D0` | partial | medium |
| `0x80C2E460` | 48 | `fn_80c2e460` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C30050` | 48 | `fn_80c30050` | `param_1 + 0x18` | W6 factory `0x80C300B0` (`0xD78`), constructor `0x80C30150` | partial | medium |
| `0x80C30080` | 48 | `fn_80c30080` | `param_1 + 0x24` | same site | partial | medium |
| `0x80C322F0` | 48 | `fn_80c322f0` | `param_1 + 0x18` | W8 factory `0x80C32350` (`0xCB8`), constructor `0x80C32380` | partial | medium |
| `0x80C32320` | 48 | `fn_80c32320` | `param_1 + 0x24` | same site | partial | medium |

What is proven:

- The C body of all twelve is exactly
  `func_0x802dcbfc(param_2,param_2,param_1);`.
- The raw PowerPC text repeats the earlier adapter shape exactly:
  `stwu r1,-0x10(r1)` / `mflr r0` / `mr r5,r3` / `mr r3,r4` /
  `stw r0,0x14(r1)` / `addi r12,r5,0x18` or `0x24` / `bl 0x802DCBFC` / `nop` /
  epilogue. Since `r5` receives the first C argument, the hidden register is
  `param_1 + 0x18` in the first body of each pair and `param_1 + 0x24` in the
  second.
- Each pair is exactly `0x60` bytes before a factory/constructor site whose
  bodies were translated earlier: the first adapter is at `site - 0x60`, the
  second at `site - 0x30`. The six sites are the 0x1238-byte actor library,
  the `0x6B0` and `0x768` actor constructors, and the W4/W6/W8 flying-boss
  factories and constructors.
- `platform::call3` cannot load `r12`, so the visible call is translated but
  every body is `partial`; the exact adjustment is recorded in each doc
  comment.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C4F000` | `FUN_80c4f000` | `fn_80c4f000` | Wendy state-id factory (`ShellAtk`) | translated | high | `80c4f000_FUN_80c4f000.c` |
| `0x80C4F010` | `FUN_80c4f010` | `fn_80c4f010` | Wendy state-id factory (`ShellAtk_St`) | translated | high | `80c4f010_FUN_80c4f010.c` |
| `0x80C4F020` | `FUN_80c4f020` | `fn_80c4f020` | Wendy state-id factory (`DemoEscape_St`) | translated | high | `80c4f020_FUN_80c4f020.c` |
| `0x80C4F030` | `FUN_80c4f030` | `fn_80c4f030` | Wendy state-id factory (`DemoIkaku_Wait`) | translated | high | `80c4f030_FUN_80c4f030.c` |
| `0x80C4F040` | `FUN_80c4f040` | `fn_80c4f040` | Wendy state-id factory (`DemoIkaku`) | translated | high | `80c4f040_FUN_80c4f040.c` |
| `0x80C4F050` | `FUN_80c4f050` | `fn_80c4f050` | Wendy state-id factory (`DemoAwake_Wait`) | translated | high | `80c4f050_FUN_80c4f050.c` |
| `0x80C4F060` | `FUN_80c4f060` | `fn_80c4f060` | Wendy state-id factory (`DemoAwake`) | translated | high | `80c4f060_FUN_80c4f060.c` |
| `0x80C4F070` | `FUN_80c4f070` | `fn_80c4f070` | Wendy state-id factory (`BigJump`) | translated | high | `80c4f070_FUN_80c4f070.c` |
| `0x80C4F080` | `FUN_80c4f080` | `fn_80c4f080` | Wendy state-id factory (`Jump`) | translated | high | `80c4f080_FUN_80c4f080.c` |
| `0x80C4F090` | `FUN_80c4f090` | `fn_80c4f090` | Wendy state-id factory (`Attack`) | translated | high | `80c4f090_FUN_80c4f090.c` |
| `0x80C4F0A0` | `FUN_80c4f0a0` | `fn_80c4f0a0` | Wendy state-id factory (`ShellOut`) | translated | high | `80c4f0a0_FUN_80c4f0a0.c` |
| `0x80C4F0B0` | `FUN_80c4f0b0` | `fn_80c4f0b0` | Wendy state-id factory (`DieFumi`) | translated | high | `80c4f0b0_FUN_80c4f0b0.c` |
| `0x80C1C630` | `FUN_80c1c630` | `fn_80c1c630` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c1c630_FUN_80c1c630.c` |
| `0x80C1C660` | `FUN_80c1c660` | `fn_80c1c660` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c1c660_FUN_80c1c660.c` |
| `0x80C29560` | `FUN_80c29560` | `fn_80c29560` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c29560_FUN_80c29560.c` |
| `0x80C29590` | `FUN_80c29590` | `fn_80c29590` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c29590_FUN_80c29590.c` |
| `0x80C2C2F0` | `FUN_80c2c2f0` | `fn_80c2c2f0` | teardown forwarder (`r12=+0x18`) | partial | medium | `80c2c2f0_FUN_80c2c2f0.c` |
| `0x80C2C320` | `FUN_80c2c320` | `fn_80c2c320` | teardown forwarder (`r12=+0x24`) | partial | medium | `80c2c320_FUN_80c2c320.c` |
| `0x80C2E430` | `FUN_80c2e430` | `fn_80c2e430` | W4 teardown forwarder (`r12=+0x18`) | partial | medium | `80c2e430_FUN_80c2e430.c` |
| `0x80C2E460` | `FUN_80c2e460` | `fn_80c2e460` | W4 teardown forwarder (`r12=+0x24`) | partial | medium | `80c2e460_FUN_80c2e460.c` |
| `0x80C30050` | `FUN_80c30050` | `fn_80c30050` | W6 teardown forwarder (`r12=+0x18`) | partial | medium | `80c30050_FUN_80c30050.c` |
| `0x80C30080` | `FUN_80c30080` | `fn_80c30080` | W6 teardown forwarder (`r12=+0x24`) | partial | medium | `80c30080_FUN_80c30080.c` |
| `0x80C322F0` | `FUN_80c322f0` | `fn_80c322f0` | W8 teardown forwarder (`r12=+0x18`) | partial | medium | `80c322f0_FUN_80c322f0.c` |
| `0x80C32320` | `FUN_80c32320` | `fn_80c32320` | W8 teardown forwarder (`r12=+0x24`) | partial | medium | `80c32320_FUN_80c32320.c` |

## Unknowns

- **Wendy ids.** The twelve returns are opaque words. The registrar stores
  them at `+0x30`; the exports never dereference them. Their numeric pattern
  (`0x80358184..0x80358804`) is not explained by the exports.
- **Meaning of `r12` at `0x802DCBFC`.** The instruction that loads it is exact
  (`param_1 + 0x18` or `param_1 + 0x24`), but how the imported stub consumes
  it is not proven. The bodies stay `partial` because `platform::call3` cannot
  set it.
- **Adapter ownership.** No C function references the twelve adapters; their
  vtable slots are written through data initializers. The W4/W6/W8 assignment
  follows from the adjacent factory and constructor addresses, not from a
  named caller.
- **`DieFumi` prefix byte.** `0x80C6B623` is a backtick in the data section and
  the registrar compensates with `+ 1`. The cause is not known; the Rust doc
  quotes the readable name.

## Remaining candidates

| address / family | size | why deferred |
|---|---|---|
| 163 state-id factories | 12 B each | complete bodies; the largest contiguous runs are `0x80C3D620` (17), `0x80C211E0` (16), `0x80C218D0` (16), `0x80C1C2E0` (13), `0x80C3D1F0` (13) |
| 10 `0x802DCBFC` adapters | 48 B each | hidden `r12`; five pairs remain at `0x80C18530`, `0x80C21560`, `0x80C2A330`, `0x80C3D540`, `0x80C4F340` |
| `0x80C38230` | 48 B | not an adapter: virtual call through `*(param_1 + 0x394)`, returns `1` |
| `0x80C389B0`, `0x80C38A00`, `0x80C3DFE0`, `0x80C3E030` | 72 B each | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 B | export stops after the first load |
| `0x80C4FDE0` | 16 B | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 B | bad instruction data; no statements recovered |
| `0x80C512F0`, `0x80C51430` | 20/24 B | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 B | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 B each | import thunks with bad instruction data |

After this pass 188 of the 770 module functions remain untranslated:
163 factories + 10 adapters + the non-adapter `0x80C38230` + 14 truncated or
thunk exports.

## Verification

- `cargo check` exits 0; no diagnostic names `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 24 evidence files were read in full: twelve are one constant `return`,
  twelve are the single `0x802DCBFC` call.
- The Wendy registrar `0x80C4E010` was parsed call-by-call: the twelve selected
  addresses are each called once, in descending address order, and every call
  is followed by the matching `func_0x8015f7c0` state string.
- Each selected body's raw bytes were checked in `work/rels/d_en_bossNP.rel`:
  the twelve factories are `lis`/`addi`/`blr`, and each adapter contains
  `addi r12,r5,0x18` or `0x24` followed by the branch to `0x802DCBFC`.
- The 44-body adapter inventory comes from `functions.tsv` filtered to size 48
  and from a body-text match for
  `func_0x802dcbfc(param_2,param_2,param_1)`; the ledger was then used to
  separate the 22 previously translated bodies from the 22 untranslated ones.
- The 24 rows in `ledger/agents/wave30-bosses.csv` match the Rust functions
  one to one: 12 `translated`/`high` and 12 `partial`/`medium`.
- `src/game/bosses.rs` now defines 582 REL bodies; the module header count was
  updated from 558 to 582.
- No files outside `src/game/bosses.rs`, this document and the wave-30 ledger
  were edited.
