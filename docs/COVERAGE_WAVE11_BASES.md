# Coverage - wave 11 (`d_basesNP` castle-demo, Lemmy-ball, clown, coin and wire-mesh actors)

## How to read this file

This page records the eleventh translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/bases.rs`: five `StateID` registration tables, their registered
   destructors and state-object helpers, three actor constructors, and the
   constant handler getters used by the tables.
2. **Why these functions?** - *Selection* explains why the pass closed the four
   remaining `da*` class-named state tables plus the adjacent
   `dCollectionCoin_c` menu table.
3. **What is still missing?** - *Deferred candidates* records the larger menu
   tables and the paired-single code left untouched. *Unknowns preserved* lists
   every unresolved callback instead of guessing it.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or class
  attribution is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8000xxxx`, `0x8032xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 9 | Wave 10 | Wave 11 | Cumulative (`game/bases.rs`) |
|---|---:|---:|---:|---:|
| Translated functions in `src/game/bases.rs` | 24 | 29 | 30 | 226 |
| Actor classes with a complete table+destructor+constructor triple | 8 | 4 | 3 | 25 since wave 7 |
| State registrations covered | 10 | 39 | 30 | 119 since wave 7 |
| Selected bytes | 4,940 | 11,612 | 7,800 | 35,412 since wave 7 |

The 30 new bodies select 7,800 of the 1,109,860 indexed bytes (0.70 %) and
30 of the 5,409 eligible entries (0.55 %). Confidence is **27 high** and
**3 medium**; there are no `partial` or `stub` rows. The total is 226 function
bodies in `src/game/bases.rs`.

## Selection

This pass stayed evidence-first and lifecycle-first:

1. **Started from `functions.tsv` and the existing `bases.rs`.** Every address
   already translated in waves 2-10 was removed from the candidate set before
   selection (196 ledger rows for `game::bases`).
2. **Closed the remaining class-named `da*` tables.** A scan of every
   untranslated export for `s_da<Class>_c__` strings found exactly four:
   `daCastleKokoopaDemo1st_c` (`0x8066F000`), `daCastleLemmyBall_c`
   (`0x80672500`), `daJrClownC_c` (`0x806A0B30`) and
   `daMiniGameWireMeshMgrObj_c` (`0x806FED80`).
3. **Added the adjacent menu table.** `dCollectionCoin_c` (`0x806083B0`) was
   the smallest remaining menu table with a complete named state table and a
   local destructor, so it was taken with its inferred constructor.
4. **Included every registered lifecycle helper.** For each table the exported
   destructor, the two destructor variants (where the table registers two), the
   array element constructor/destructor, and the constant handler getters were
   translated instead of being left as address-only calls.
5. **Reused the verified registration template.** Every state builds a
   0x30-byte descriptor at stride 0x40, reads nine parameter words, installs the
   shared state vtable, and calls [`fn_80600160`](../src/game/bases.rs) with the
   `descriptor - 0x10` link node. The existing `register_state_desc` helper
   covers all 30 descriptors selected here; the extra handler word at
   `descriptor + 0x30` is written explicitly after each registration.
6. **Verified class names against the state strings.** The `StateID` strings
   were read from the REL data section, not guessed. Constructors carry no class
   string, so their attribution is marked `// inferred:` from the vtable they
   install and from adjacency to the table.

## Clusters

### Castle demo (`0x8066F000..0x8066F760`)

- `daCastleKokoopaDemo1st_c` (`0x8066F000`): three states, vtable `0x807E9B44`,
  destructor `0x8066F390`, handler getters `0x8066F3F0`/`0x8066F400`/
  `0x8066F410`, and a class-name node callback `0x8066F310`.
- Constructor `0x8066F700` (`0x440` bytes, class vtable `0x807E9BC0`).
- `0x8066F760` returns the class-name constant `0x80377A04`.

### Lemmy ball (`0x80672500..0x80672D30`)

- `daCastleLemmyBall_c` (`0x80672500`): a context object at `0x80829A04`, a
  six-element `0x28`-byte array at `0x80829A30` built by `0x802DC9A0`, and seven
  states (`Dem`, two `Fil`, `Wai`, `Den`, two `Jum`). Destructors `0x80672B20`,
  `0x80672C60`, `0x80672CC0`; element helpers `0x80672B90`/`0x80672BE0`;
  handler getters `0x80672D20`/`0x80672D30`.
- The class-name callback `0x80672C40` has no exported body.

### Jr clown (`0x806A0B30..0x806A12D0`)

- `daJrClownC_c` (`0x806A0B30`): eight states (`Move`, `DemoWait`, `DemoAwake`,
  `BombardRea`, `LargeBomba`, `Bombard`, `Damaged`, `Fire`), class vtable
  `0x807F2204`, bombardment vtable `0x807F2238`, two destructors
  `0x806A11D0`/`0x806A1230`, and five handler getters
  `0x806A1290..0x806A12D0`.

### Collection coin (`0x806083B0..0x80608B00`)

- `dCollectionCoin_c` (`0x806083B0`): seven states (`Initi`, `OnSta`, `OpenA`,
  `KeyWa`, two `World`, `ExitA`), vtable `0x807D2770`, destructor `0x806089E0`,
  and class-name callback `0x80608980`.
- Constructor `0x80608B00` (class vtable `0x807D28E0`, layout at `+0xA0`).

### Mini-game wire mesh (`0x806FED80..0x806FF900`)

- `daMiniGameWireMeshMgrObj_c` (`0x806FED80`): five states with truncated
  `Stat` names, vtable `0x80803704`, destructor `0x806FF180`; constructor
  `0x806FF7B0` (`0x3D0` bytes, class vtable `0x80803930`); class-name getter
  `0x806FF900`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave11-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8066F000` | `fn_8066f000` | register the three `daCastleKokoopaDemo1st_c` states | translated | high | `8066f000_FUN_8066f000.c` |
| `0x8066F390` | `fn_8066f390` | destroy a registered Kokoopa demo state | translated | high | `8066f390_FUN_8066f390.c` |
| `0x8066F3F0` | `fn_8066f3f0` | return handler constant `0x80352730` | translated | high | `8066f3f0_FUN_8066f3f0.c` |
| `0x8066F400` | `fn_8066f400` | return handler constant `0x803526F0` | translated | high | `8066f400_FUN_8066f400.c` |
| `0x8066F410` | `fn_8066f410` | return handler constant `0x803526B0` | translated | high | `8066f410_FUN_8066f410.c` |
| `0x8066F700` | `fn_8066f700` | construct the `0x440`-byte castle-demo actor | translated | medium | `8066f700_FUN_8066f700.c` |
| `0x8066F760` | `fn_8066f760` | return class-name constant `0x80377A04` | translated | high | `8066f760_FUN_8066f760.c` |
| `0x80672500` | `fn_80672500` | register the seven `daCastleLemmyBall_c` states and array | translated | high | `80672500_FUN_80672500.c` |
| `0x80672B20` | `fn_80672b20` | destroy the Lemmy-ball context object | translated | high | `80672b20_FUN_80672b20.c` |
| `0x80672B90` | `fn_80672b90` | construct one `0x28`-byte Lemmy array element | translated | high | `80672b90_FUN_80672b90.c` |
| `0x80672BE0` | `fn_80672be0` | destroy one Lemmy array element | translated | high | `80672be0_FUN_80672be0.c` |
| `0x80672C60` | `fn_80672c60` | destroy a registered Lemmy state | translated | high | `80672c60_FUN_80672c60.c` |
| `0x80672CC0` | `fn_80672cc0` | destroy a registered Lemmy demo/wait state | translated | high | `80672cc0_FUN_80672cc0.c` |
| `0x80672D20` | `fn_80672d20` | return shared data block `0x8082CE04` | translated | high | `80672d20_FUN_80672d20.c` |
| `0x80672D30` | `fn_80672d30` | return shared data block `0x8082CC04` | translated | high | `80672d30_FUN_80672d30.c` |
| `0x806A0B30` | `fn_806a0b30` | register the eight `daJrClownC_c` states | translated | high | `806a0b30_FUN_806a0b30.c` |
| `0x806A11D0` | `fn_806a11d0` | destroy a bombardment clown state | translated | high | `806a11d0_FUN_806a11d0.c` |
| `0x806A1230` | `fn_806a1230` | destroy a movement/fire clown state | translated | high | `806a1230_FUN_806a1230.c` |
| `0x806A1290` | `fn_806a1290` | return `Fire` handler `0x80357F5C` | translated | high | `806a1290_FUN_806a1290.c` |
| `0x806A12A0` | `fn_806a12a0` | return `Damaged` handler `0x80357E9C` | translated | high | `806a12a0_FUN_806a12a0.c` |
| `0x806A12B0` | `fn_806a12b0` | return `DemoAwake` handler `0x80357E1C` | translated | high | `806a12b0_FUN_806a12b0.c` |
| `0x806A12C0` | `fn_806a12c0` | return `DemoWait` handler `0x80357DDC` | translated | high | `806a12c0_FUN_806a12c0.c` |
| `0x806A12D0` | `fn_806a12d0` | return `Move` handler `0x80357E5C` | translated | high | `806a12d0_FUN_806a12d0.c` |
| `0x806083B0` | `fn_806083b0` | register the seven `dCollectionCoin_c` states | translated | high | `806083b0_FUN_806083b0.c` |
| `0x806089E0` | `fn_806089e0` | destroy a registered collection-coin state | translated | high | `806089e0_FUN_806089e0.c` |
| `0x80608B00` | `fn_80608b00` | construct the collection-coin actor in place | translated | medium | `80608b00_FUN_80608b00.c` |
| `0x806FED80` | `fn_806fed80` | register the five `daMiniGameWireMeshMgrObj_c` states | translated | high | `806fed80_FUN_806fed80.c` |
| `0x806FF180` | `fn_806ff180` | destroy a registered wire-mesh state | translated | high | `806ff180_FUN_806ff180.c` |
| `0x806FF7B0` | `fn_806ff7b0` | construct the `0x3D0`-byte wire-mesh manager | translated | medium | `806ff7b0_FUN_806ff7b0.c` |
| `0x806FF900` | `fn_806ff900` | return class-name constant `0x80377A04` | translated | high | `806ff900_FUN_806ff900.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 27 high confidence, 3 medium
(the three constructors whose class attribution is inferred from adjacency).

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x80618D00` | `dFileSelect_c` state table, 44 `StateID` references and 8,344 bytes; still the largest single table in the module. |
| `0x8063D7A0`, `0x8062A2C0`, `0x8062F050` | 20-, 19- and 18-state menu tables; same mechanical family, better handled with a table generator. |
| `0x80622010`, `0x806393F0`, `0x807BD110`, `0x8060B680`, `0x807AC320` | 17-, 16-, 13-, 11- and 11-state menu/world-map tables. |
| `0x806A0290` | `daJrClownC_c` per-frame handler; explicit virtual dispatch and float state math, not part of the table/constructor lifecycle slice. |
| `0x8066FA10` | castle-demo update; rejected again because Ghidra reconstructs paired-single loads with `unaff_GQR0`/`ldexpf`. |
| `0x806FF3F0`, `0x806FF700`, `0x806FF4C0`, `0x806FFB50` | wire-mesh lifecycle handlers after the table; the constructor and destructor are the complete slice translated here. |
| `0x80608C40`, `0x80608D10`, `0x80608DF0` | collection-coin post-construction handlers; they build layouts and need the screen-object model first. |

## Unknowns preserved

1. **Constructor class attribution.** None of the three constructors contains a
   class string. Each is tied to its class only by the vtable it installs and by
   adjacency to the table; those rows are `medium` confidence and the Rust
   carries `// inferred:` notes.
2. **Class-name node callbacks.** `0x8066F310`, `0x80608980` and `0x80672C40`
   are stored through `fn_80600160` but have no exported function body. They are
   kept by address with `// unknown:` notes.
3. **Constant handler getters.** `0x8066F3F0`/`0x8066F400`/`0x8066F410`,
   `0x806A1290..0x806A12D0` and the two Lemmy data-block getters return
   main.dol/REL constants. They are translated as direct returns, and the table
   stores their results at `descriptor + 0x30`.
4. **Truncated state strings.** Ghidra truncates the Kokoopa names to
   `State_...`/`StateI...`, the collection-coin names to three or five
   characters (`Initi`, `OnSta`, ...), and the wire-mesh names to `Stat`. The
   Rust names only the visible prefix and does not invent the missing
   characters.
5. **Array element helpers.** `0x80672B90`/`0x80672BE0` are identified as the
   constructor/destructor pair only by their position in the `0x802DC9A0` call;
   the element layout (`+0` vtable, `+4`/`+8` words, `+0x0C` sub-object) is
   observed, not documented.
6. **Imported sub-object routines.** `0x800E8340`, `0x802BE7C0`,
   `0x800C8950`, `0x801628C0`, `0x802DC9A0` and `0x8006C420` are main.dol
   routines whose roles are inferred from their call positions. They are emitted
   by address, never renamed.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 226 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are in `src/runtime/bt.rs` and `src/runtime/input.rs`, and
`src/game/bases.rs` itself produces no errors or warnings.
