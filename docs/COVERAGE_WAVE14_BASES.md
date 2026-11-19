# Coverage - wave 14 (`d_basesNP` course, menu and result screen actors)

## How to read this file

This page records the fourteenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: ten `StateID` registration tables, their registered
   destructors, and ten in-place constructors.
2. **Why these functions?** - *Selection* explains why this pass closed ten
   complete table/destructor/constructor triples instead of starting the two
   remaining giant tables.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the remaining tables, handler bodies and every role that is only
   inferred.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a class attribution is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006xxxx`, `0x800Cxxxx`, `0x802Dxxxx`, `0x8032xxxx`, `0x80329958`) are
imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 14 |
|---|---:|
| New translated functions | 30 |
| State registration tables | 10 |
| Registered states described | 80 |
| Actor constructors (in-place) | 10 |
| Registered-state destructors | 10 |
| Selected bytes | 18,784 |
| Translated functions in `src/game/bases.rs` (cumulative) | 316 |
| Cumulative selected bytes | 106,740 (9.62 % of indexed bytes) |

The 30 new bodies select 18,784 of the 1,109,860 indexed bytes (1.69 %) and 30
of the 5,409 eligible entries (0.55 %). Confidence is **20 high** and **10
medium**; there are no `partial` or `stub` rows. The total is 316 function
bodies in `src/game/bases.rs`.

## Selection

The pass stayed evidence-first and lifecycle-first:

1. **Started from `functions.tsv`, the existing `bases.rs` and
   `ledger/functions.csv`.** All 286 addresses already translated by waves
   2-13 were removed before selection; none of the 30 new addresses appears in
   an earlier ledger row.
2. **Scanned every remaining untranslated C file for the state-name
   constructor `func_0x8015f7c0` and the registry call `0x80600160`.** This
   left 21 untranslated tables. Ten were taken as complete families, each with
   its registered destructor and an exported in-place constructor in a tight
   window, between `0x8060B680` and `0x806393F0`.
3. **Closed complete class families.** Each selected table was taken with its
   registered destructor and the in-place constructor that installs the class
   vtable. This avoids leaving a table whose lifecycle calls point at
   untranslated neighbours.
4. **Reused and re-verified the descriptor template.** All 80 descriptors build
   a 0x30-byte object at stride 0x40, read nine parameter words, install one
   shared vtable, and call [`fn_80600160`](../src/game/bases.rs) with the link
   node. The Rust helper `register_state_desc` covers every descriptor; the
   nine store offsets and their source words were checked against all ten C
   files (720 stores, 0 mismatches).
5. **Preserved the two preludes and one one-time guard.** `dCourseSelectMenu_c`
   and `dOtehonClear_c` copy main.dol/REL data words and link a class-name node
   before their first descriptor; `dModeSelect_c` runs a guarded one-time
   initialiser. All three are translated verbatim.
6. **Read the real state strings.** Names were extracted from the REL data
   image at the referenced addresses, not guessed from the truncated Ghidra
   symbols (`..._807d33dc`).
7. **Rejected one false constructor.** `0x807AA310`, which follows the
   `dOtasukeInfo_c` destructor, is a global initialisation routine (a chain of
   system setup calls plus per-channel loops), not an in-place class
   constructor, so that family was left for a later pass.

## Clusters

### Continue screen (`0x8060B680..0x8060C000`)

- `dContinue_c` (`0x8060B680`): eleven states (`StartWait`,
  `BlackInAnimeEndWait`, `OpenAnimeEndWait`, `ContinueCountUpWait`,
  `ContinueCountUp`, `RestCountUpWait`, `RestCountUp`, `ColorRecover`,
  `EndWait`, `ExitAnimeEndWait`, `BlackExitAnimeEndWait`), vtable `0x807D3588`,
  destructor `0x8060BEE0`.
- Constructor `0x8060C000` installs class vtable `0x807D38F0`, builds the
  layout at `+0x70` and a `0x802D7AA0` interface sub-object at `+0x208`, and
  wraps the interface group at `+0x330..+0x33C`.

### Course-select menu (`0x8060DE60..0x8060E4B0`)

- `dCourseSelectMenu_c` (`0x8060DE60`): six states (`InitWait`,
  `OpenAnimeEndWait`, `ButtonChangeAnimeEndWait`, `PauseDisp`,
  `HitAnimeEndWait`, `ClouseAnimeEndWait`; the `Clouse` spelling is in the
  original string), vtable `0x807D42A8`, destructor `0x8060E390`.
- The prelude writes `0x807D3C24`/`0x807D3C30..0x807D3C38` and links the
  class-name node `0x8060E370` at `0x808236B0`; `0x808236BC` receives one more
  main.dol word.
- Constructor `0x8060E4B0` installs class vtable `0x807D43D0` and the final
  interface vtable `0x807D4420`.

### Opening title (`0x80614F40..0x80615460`)

- `dEventOpeningTitle_c` (`0x80614F40`): five states (`WipeOutEndWait`,
  `OpenAnimeEndWait`, `PressButtonDispWait`, `PressButtonExitWait`,
  `PressButtonExitAnimeEndWait`), vtable `0x807D5F58`, destructor
  `0x80615340`.
- Constructor `0x80615460` installs class vtable `0x807D6750` and derives
  `+0x2D4` from the low four bits of the object's `+0x04` word.

### Letter window (`0x8061D110..0x8061D6E0`)

- `dLetterWindow_c` (`0x8061D110`): six states (`StartWait`,
  `StartAnimeEndWait`, `ButtonOnStageAnimeEndWait`, `ButtonInputWait`,
  `HitAnimeEndWait`, `ExitAnimeEndWait`), vtable `0x807D8368`, destructor
  `0x8061D5C0`.
- Constructor `0x8061D6E0` installs class vtable `0x807D84B0`, wraps the
  interface group at `+0x70..+0x7C`, and builds the layout at `+0xAC`.

### Mini-game cannon (`0x8061FB10..0x806203D0`)

- `dMiniGameCannon_c` (`0x8061FB10`): ten states (`TitleOpenWait`,
  `TitleOpenAnimeEndWait`, `TitleDisp`, `TitleExitAnimeEndWait`, `StartWait`,
  `OpenAnimeEndWait`, `NowDisp`, `ResultDispAnimeEndWait`, `ResultNowDisp`,
  `ExitAnimeEndWait`), vtable `0x807D9070`, destructor `0x806202B0`.
- Constructor `0x806203D0` installs class vtable `0x807D98B0` and clears
  `+0x378`, `+0x379`, `+0x37A` and `+0x37C`.

### Mode select (`0x80623AB0..0x806240A0`)

- `dModeSelect_c` (`0x80623AB0`): six states (`StartWait`,
  `OpenAnimeEndWait`, `ButtonChangeAnimeEndWait`, `SelectWait`,
  `HitAnimeEndWait`, `ExitAnimeEndWait`), vtable `0x807DA4A8`, destructor
  `0x80623F80`.
- The function first runs a one-time initialiser guarded by `0x808256F0`,
  which copies the word at `0x807C6EF0` to `0x80825570` (the raw instructions
  copy a 32-bit float with `lfs`/`stfs`; the Rust copies the same four bytes).
  The first registry node is `descriptor - 0x0C`; the other five are
  `descriptor - 0x10`.
- Constructor `0x806240A0` installs class vtable `0x807DA630` and final
  interface vtable `0x807DA680`.

### Model-play arrow and base (`0x80624EE0..0x80626810`)

- `dModelPlayArrow_c` (`0x80624EE0`): six states (`StartWait`, `ActionWait`,
  `OnStageAnimeEndWait`, `HitAnimeEndWait`,
  `LeftOrRightExitAnimeEndWait`, `ExitAnimeEndWait`), vtable `0x807DA978`,
  destructor `0x80625390`; constructor `0x806254B0` clears `+0x27C`.
- `dModelPlayBase_c` (`0x80626240`): six states (`StartWait`,
  `OnStageAnimeEndWait`, `ActionWait`, `DateScroll`,
  `CollectionCoinCount`, `ExitAnimeEndWait`), vtable `0x807DAF88`,
  destructor `0x806266F0`; constructor `0x80626810` clears `+0x278`.

### Otehon clear (`0x80636130..0x80636900`)

- `dOtehonClear_c` (`0x80636130`): eight states (`OtehonCheck`,
  `YesNoWindowOnStageWait`, `YesNoWindowSelectWait`,
  `YesNoWindowHitAnimeEndWait`, `ClearConfirmationWindowOnStageWait`,
  `ClearConfirmationSelectWait`, `ClearConfirmationHitAnimeEndWait`,
  `SceneChangeWait`), vtable `0x807DE998`, destructor `0x806367E0`.
- The prelude writes `0x807DE564`/`0x807DE570..0x807DE578` and links the
  class-name node `0x806367C0` at `0x80826880`; `0x8082688C` receives one more
  main.dol word. The first registry node is `descriptor - 0x0C`.
- Constructor `0x80636900` builds a four-element array at `+0x208` with
  `0x802DC9A0` (element constructor `0x8060F590`, handler `0x8060C0F0`,
  stride `0x128`) and wraps the interface group at `+0x6A8..+0x6B4`.

### Point-result date file (`0x806393F0..0x8063A120`)

- `dPointResultDateFile_c` (`0x806393F0`): sixteen states (`Initial`,
  `OnSatgeWait`, `OnSatgeAnimeEndCheck`, `ScoreCalcStartWait`, `CoinAdd`,
  `StarCoinToScoreStartWait`, `StarCoinAdd`, `StarCoinAnimeEndWait`,
  `RankChangeStartWait`, `RankExitAnimeEndWait`, `RankChange`,
  `BigSmallAnimeEndWait`, `RankOnStageAnimeEndWait`, `StarDispWait`,
  `StarOnStage`, `GoalDeMoEndWait`; the `Satge` spelling is in the original
  strings), vtable `0x807DFB30`, destructor `0x8063A000`.
- Constructor `0x8063A120` installs class vtable `0x807DFF60` and final
  interface vtable `0x807DFFB0`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave14-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8060B680` | `fn_8060b680` | register the eleven `dContinue_c` states | translated | high | `8060b680_FUN_8060b680.c` |
| `0x8060BEE0` | `fn_8060bee0` | destroy a registered continue-screen state | translated | high | `8060bee0_FUN_8060bee0.c` |
| `0x8060C000` | `fn_8060c000` | construct the continue screen in place | translated | medium | `8060c000_FUN_8060c000.c` |
| `0x8060DE60` | `fn_8060de60` | register the six `dCourseSelectMenu_c` states and class node | translated | high | `8060de60_FUN_8060de60.c` |
| `0x8060E390` | `fn_8060e390` | destroy a registered course-select-menu state | translated | high | `8060e390_FUN_8060e390.c` |
| `0x8060E4B0` | `fn_8060e4b0` | construct the course-select menu in place | translated | medium | `8060e4b0_FUN_8060e4b0.c` |
| `0x80614F40` | `fn_80614f40` | register the five `dEventOpeningTitle_c` states | translated | high | `80614f40_FUN_80614f40.c` |
| `0x80615340` | `fn_80615340` | destroy a registered opening-title state | translated | high | `80615340_FUN_80615340.c` |
| `0x80615460` | `fn_80615460` | construct the opening-title screen in place | translated | medium | `80615460_FUN_80615460.c` |
| `0x8061D110` | `fn_8061d110` | register the six `dLetterWindow_c` states | translated | high | `8061d110_FUN_8061d110.c` |
| `0x8061D5C0` | `fn_8061d5c0` | destroy a registered letter-window state | translated | high | `8061d5c0_FUN_8061d5c0.c` |
| `0x8061D6E0` | `fn_8061d6e0` | construct the letter window in place | translated | medium | `8061d6e0_FUN_8061d6e0.c` |
| `0x8061FB10` | `fn_8061fb10` | register the ten `dMiniGameCannon_c` states | translated | high | `8061fb10_FUN_8061fb10.c` |
| `0x806202B0` | `fn_806202b0` | destroy a registered mini-game-cannon state | translated | high | `806202b0_FUN_806202b0.c` |
| `0x806203D0` | `fn_806203d0` | construct the mini-game cannon in place | translated | medium | `806203d0_FUN_806203d0.c` |
| `0x80623AB0` | `fn_80623ab0` | register the six `dModeSelect_c` states and one-time word | translated | high | `80623ab0_FUN_80623ab0.c` |
| `0x80623F80` | `fn_80623f80` | destroy a registered mode-select state | translated | high | `80623f80_FUN_80623f80.c` |
| `0x806240A0` | `fn_806240a0` | construct the mode-select screen in place | translated | medium | `806240a0_FUN_806240a0.c` |
| `0x80624EE0` | `fn_80624ee0` | register the six `dModelPlayArrow_c` states | translated | high | `80624ee0_FUN_80624ee0.c` |
| `0x80625390` | `fn_80625390` | destroy a registered model-play-arrow state | translated | high | `80625390_FUN_80625390.c` |
| `0x806254B0` | `fn_806254b0` | construct the model-play arrow in place | translated | medium | `806254b0_FUN_806254b0.c` |
| `0x80626240` | `fn_80626240` | register the six `dModelPlayBase_c` states | translated | high | `80626240_FUN_80626240.c` |
| `0x806266F0` | `fn_806266f0` | destroy a registered model-play-base state | translated | high | `806266f0_FUN_806266f0.c` |
| `0x80626810` | `fn_80626810` | construct the model-play base in place | translated | medium | `80626810_FUN_80626810.c` |
| `0x80636130` | `fn_80636130` | register the eight `dOtehonClear_c` states and class node | translated | high | `80636130_FUN_80636130.c` |
| `0x806367E0` | `fn_806367e0` | destroy a registered Otehon-clear state | translated | high | `806367e0_FUN_806367e0.c` |
| `0x80636900` | `fn_80636900` | construct the Otehon-clear screen in place | translated | medium | `80636900_FUN_80636900.c` |
| `0x806393F0` | `fn_806393f0` | register the sixteen `dPointResultDateFile_c` states | translated | high | `806393f0_FUN_806393f0.c` |
| `0x8063A000` | `fn_8063a000` | destroy a registered point-result-date-file state | translated | high | `8063a000_FUN_8063a000.c` |
| `0x8063A120` | `fn_8063a120` | construct the point-result-date-file screen in place | translated | medium | `8063a120_FUN_8063a120.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 20 high confidence, 10 medium.
The medium rows are the ten constructors, whose class attribution is inferred
from the adjacent table and vtable family.

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x80618D00` | `dFileSelect_c` state table, 44 `StateID` references and 8,344 bytes; still the largest single table in the module. |
| `0x807C3D70` | `dCourseSelectManager_c` state table, 6,744 bytes; near the end of the text section and needs its own table pass. |
| `0x8063D7A0`, `0x8062A2C0`, `0x8062F050` | 20-, 19- and 18-state course/result tables; each is larger than any table taken here. |
| `0x80622010` | `dMiniGameWire_c` state table, 17 states and 3,268 bytes. |
| `0x807BD110`, `0x807AC320`, `0x807B3140`, `0x807B0C50` | `dWorldSelect_c`, `dScGameSetup_c`, `dScCrsin_c` and `dScWorld9DeMo_c` tables; each needs its own constructor/table pass. |
| `0x807A9BC0` | `dOtasukeInfo_c` table and destructor `0x807AA1F0`, but no in-place constructor: the next function `0x807AA310` is a global initialisation routine. |
| `0x8060B680..0x8063A120` | The state handler bodies for the ten selected classes; the tables and lifecycles are closed first. |
| `0x80645F90`, `0x80646180`, `0x806463B0` | `dTheEnd_c`/`dTimeUp_c` state handlers, still deferred from wave 13. |

## Unknowns preserved

1. **Constructor class attribution.** None of the ten constructors contains a
   class string. Each is tied to its class by the vtable family and adjacency
   to the table; those rows are `medium` confidence.
2. **Class-name node callbacks.** `0x8060E370` (`dCourseSelectMenu_c`) and
   `0x806367C0` (`dOtehonClear_c`) are stored through `0x80600160` but have no
   exported function body. They are kept by address with `// unknown:` notes.
3. **Shared interface-context addresses.** The constructors pass an interface
   context that is also used as the first state descriptor by a neighbouring
   class: `0x80825708` (`dModelPlayArrow_c`) in `fn_806240a0`, `0x80825888`
   (`dModelPlayBase_c`) in `fn_806254b0`, `0x80825A08` (`dModelPlayDate_c`) in
   `fn_80626810`, `0x80826AA0` (`dPointResultDateFile_c`) in `fn_80636900`,
   and `0x80825580` (`dModeSelect_c`) in the untranslated `dMiniGameWire_c`
   constructor `0x80622E00`. The stores are translated by address; whether the
   screens deliberately share a dispatcher object or the BSS slots are
   mutually exclusive is unresolved.
4. **One-time initialiser.** `dModeSelect_c` copies the word at `0x807C6EF0`
   to `0x80825570` under the `0x808256F0` guard. The consumer is outside this
   slice; the Rust preserves the guard and store.
5. **`0x80329958`.** The imported main.dol pointer stored at `+0x208`/`+0x20C`
   by the `dContinue_c` and `dCourseSelectMenu_c` constructors is kept by
   address with an `// unknown:` note.
6. **Mini-game array builder.** `0x802DC9A0`, element constructor
   `0x8060F590` and handler `0x8060C0F0` are emitted by address; their
   prototypes come from the call shape only.
7. **Main.dol and NW4R helpers.** `0x8006C420` (actor base constructor),
   `0x800C8950` (layout constructor), `0x802D7AA0` (interface sub-object),
   `0x8015FC90` (interface wrapper), `0x8015F800`/`0x8015F7C0` (state name /
   teardown), `0x802B9280` (state release) and `0x802DC9A0` (array builder) are
   emitted by address, never renamed.
8. **Compiler thunks.** The `0x802DCD74`/`0x802DCDC0` save/restore thunks that
   bracket the tables are omitted because they have no Rust-visible effect; see
   `docs/COVERAGE_r80260_802ed.md`.
9. **In-place constructors.** The constructors take the object as their first
   argument; the allocator and factory are outside the translated slice, so
   their callers and allocated sizes are not claimed.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 316 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are outside `src/game/bases.rs`, which produces no errors or
warnings.
