# Coverage - wave 13 (`d_basesNP` course, result and stock-item state actors)

## How to read this file

This page records the thirteenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: ten `StateID` registration tables, their registered
   destructors, nine in-place constructors, and the `dStockItem_c` layout
   loader.
2. **Why these functions?** - *Selection* explains why this pass closed a
   family of small and medium course/result tables instead of starting the
   large `dFileSelect_c` table.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the larger tables and every helper whose role is inferred.

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
(`0x8006xxxx`, `0x800Cxxxx`, `0x8015xxxx`, `0x8035xxxx`, `0x8042xxxx`) are
imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 13 |
|---|---:|
| New translated functions | 30 |
| State registration tables | 10 |
| Registered states described | 49 |
| Actor constructors (in-place) | 9 |
| Registered-state destructors | 10 |
| Layout/model loaders | 1 |
| Selected bytes | 12,984 |
| Translated functions in `src/game/bases.rs` (cumulative) | 286 |
| Cumulative selected bytes | 87,956 (7.92 % of indexed bytes) |

The 30 new bodies select 12,984 of the 1,109,860 indexed bytes (1.17 %) and 30
of the 5,409 eligible entries (0.55 %). Confidence is **21 high** and **9
medium**; there are no `partial` or `stub` rows. The total is 286 function
bodies in `src/game/bases.rs`.

## Selection

The pass stayed evidence-first and lifecycle-first:

1. **Started from `functions.tsv`, the existing `bases.rs` and
   `ledger/functions.csv`.** All 256 addresses already translated by waves
   2-12 were removed before selection; none of the 30 new addresses appears in
   an earlier ledger row.
2. **Scanned every untranslated C file for the state-name constructor
   `func_0x8015f7c0` and the registry call `0x80600160`.** This leaves 34
   untranslated tables; ten of them form small/medium course, result and stock
   families between `0x80627660` and `0x807B48F0`.
3. **Closed complete class families.** Each selected table was taken with its
   registered destructor and, where an exported body exists, the in-place
   constructor that installs the class vtable. The only missing constructor in
   the selection is `dTheEnd_c`; its table and destructor are still complete.
4. **Added the stock-item layout loader.** `0x80644B30` is the only exported
   function that binds the `stockItemShadow` archive, layout and pane set for
   the `dStockItem_c` constructor, so it was included as the lifecycle
   completion of that family.
5. **Reused the verified registration template.** Every state builds a
   0x30-byte descriptor at stride 0x40, reads nine parameter words, installs a
   shared state vtable, and calls [`fn_80600160`](../src/game/bases.rs) with the
   `descriptor - 0x10` link node. `register_state_desc` covers all 49
   descriptors selected here.
6. **Read the real state strings.** Names were extracted from the REL data
   image at the referenced addresses, not guessed from truncated Ghidra
   symbols. Constructors carry no class string, so their attribution is marked
   `// inferred:` from the vtable family and adjacency to the table.

## Clusters

### Model play date (`0x80627660..0x806279C0`)

- `dModelPlayDate_c` (`0x80627660`): two states (`ActionWait`,
  `AnimeEndWait`), vtable `0x807DB458`, destructor `0x80627820`.
- Constructor `0x80627940` installs class vtable `0x807DB4F0` and initialises
  the layout at `+0x70`.

### Multi-course select contents (`0x80631950..0x80632100`)

- `dMultiCourseSelectContents_c` (`0x80631950`): seven states (`DispWait`,
  `ActionWait`, `OnAnimeEndWait`, `HitAnimeEndWait`, `PauseAnimeEndWait`,
  `OffAnimeEndWait`, `ClearAnimeEndWait`), vtable `0x807DDA28`, destructor
  `0x80631EC0`.
- Constructor `0x80631FE0` installs class vtable `0x807DDFD0`, builds four
  sub-objects at `+0xB4`, `+0x188`, `+0x25C` and `+0x330`, and clears 16 panel
  words.

### Point-result date file free (`0x8063AF10..0x8063B420`)

- `dPointResultDateFileFree_c` (`0x8063AF10`): four states (`Initial`,
  `OnSatgeWait`, `OnSatgeAnimeEndCheck`, `GoalDeMoEndWait`; the `Satge`
  spelling is in the original strings), vtable `0x807E0220`, destructor
  `0x8063B250`.
- Constructor `0x8063B370` installs class vtable `0x807E0580` and clears
  `+0x91C`.

### Staff credit score (`0x806415D0..0x80642000`)

- `dStaffCreditScore_c` (`0x806415D0`): eight states (`OnStageWait`,
  `OnStageAnimeEndWait`, `NowDisp`, `HighScoreCheck`,
  `HighScoreAnimeEndWait`, `No1PlayerKiMeStartWait`, `No1PlayerKiMe`,
  `EndWait`), vtable `0x807E2098`, destructor `0x80641C00`.
- Constructor `0x80641D20` installs class vtable `0x807E2660`, builds a
  4-element array at `+0x32C` through `0x802DC9A0`, and constructs the element
  at `+0x77C`.

### Stock item (`0x806441E0..0x806455E0`)

- `dStockItem_c` (`0x806441E0`): eight states (`WindowOpenInit`,
  `WindowOpenAnimeEndWait`, `ButtonChangeAnimeEndWait`, `ItemSelectWait`,
  `SuccessionItemSelectWait`, `WindowCloseAnimeEndWait`, `HitAnimeEndWait`,
  `WindowCloseWait`), vtable `0x807E2AF8`, destructor `0x806448C0`.
- The table is the only selected function with a class-name/configuration
  prelude: `0x80600160(0, 0x806448A0, 0x808278B0)` links the class-name node,
  and a one-time guard fills `0x808278C0/C4` and the byte
  `0x808278D6 = 0x78`.
- Constructor `0x806449E0` installs class vtable `0x807E2D58`.
- Layout loader `0x80644B30` loads `stockItemShadow/stockItemShadow.arc`,
  `stockItemShadow_00.brlyt`, seven `T_itemRest_07` animations and seven
  `P_buttonBaseS_00` panes.

### The end and time up (`0x80645820..0x80646930`)

- `dTheEnd_c` (`0x80645820`): four states (`OnStageWait`,
  `OnStageAnimeEndWait`, `NowDisp`, `ExitAnimeEndWait`), vtable
  `0x807E30B0`, destructor `0x80645B60`. No constructor body is exported.
- `dTimeUp_c` (`0x806465F0`): four states (`DispWait`, `OpenAnimeEndWait`,
  `DispEndWait`, `ExitAnimeEndWait`), vtable `0x807E344C`, destructor
  `0x80646930`.
- Constructor `0x80645C80` installs class vtable `0x807E31D8` and clears
  `+0x258`.

### Result screens (`0x807AD000..0x807B48F0`)

- `dScMulti_c` (`0x807ADFA0`): four states (`FadeInEndWait`,
  `SelectCourse`, `ReturnTitleFinalConfirm`,
  `ReturnTitleHitAnimrEndWait`; the `Animr` spelling is in the original
  string), vtable `0x8081E364`, destructor `0x807AE2E0`.
- Constructor `0x807AD000` installs class vtable `0x8081E0D0`, builds a
  ten-entry callback table at `+0xB0`, and publishes the screen at
  `0x80833890`.
- `dScResult_c` (`0x807AF240`): two states (`ResultStartWait`,
  `ResultEndWait`), vtable `0x8081E624`, destructor `0x807AF400`.
- Constructor `0x807AE720` installs class vtable `0x8081E430`, builds a
  seven-entry callback table at `+0xB0`, and publishes the screen at
  `0x80833990`.
- `dScGameOver_c` (`0x807B4440`): six states (`LoGoStartWait`,
  `DrawDispEndWait`, `MultiCourseSelectSceneWait`, `GameOverDispEndWait`,
  `ContinueEndWait`, `SaveEndWait`), vtable `0x8081F684`, destructor
  `0x807B48F0`.
- Constructor `0x807B3C70` installs class vtable `0x8081F350`, creates the
  six-entry callback table at `+0x84`, and stores its pointer at `+0x70`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave13-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80627660` | `fn_80627660` | register the two `dModelPlayDate_c` states | translated | high | `80627660_FUN_80627660.c` |
| `0x80627820` | `fn_80627820` | destroy a registered model-play-date state | translated | high | `80627820_FUN_80627820.c` |
| `0x80627940` | `fn_80627940` | construct the model-play-date object in place | translated | medium | `80627940_FUN_80627940.c` |
| `0x80631950` | `fn_80631950` | register the seven `dMultiCourseSelectContents_c` states | translated | high | `80631950_FUN_80631950.c` |
| `0x80631EC0` | `fn_80631ec0` | destroy a registered multi-course-contents state | translated | high | `80631ec0_FUN_80631ec0.c` |
| `0x80631FE0` | `fn_80631fe0` | construct the multi-course-contents object in place | translated | medium | `80631fe0_FUN_80631fe0.c` |
| `0x8063AF10` | `fn_8063af10` | register the four `dPointResultDateFileFree_c` states | translated | high | `8063af10_FUN_8063af10.c` |
| `0x8063B250` | `fn_8063b250` | destroy a registered point-result-free state | translated | high | `8063b250_FUN_8063b250.c` |
| `0x8063B370` | `fn_8063b370` | construct the point-result-free object in place | translated | medium | `8063b370_FUN_8063b370.c` |
| `0x806415D0` | `fn_806415d0` | register the eight `dStaffCreditScore_c` states | translated | high | `806415d0_FUN_806415d0.c` |
| `0x80641C00` | `fn_80641c00` | destroy a registered staff-credit state | translated | high | `80641c00_FUN_80641c00.c` |
| `0x80641D20` | `fn_80641d20` | construct the staff-credit object in place | translated | medium | `80641d20_FUN_80641d20.c` |
| `0x806441E0` | `fn_806441e0` | register the eight `dStockItem_c` states and class node | translated | high | `806441e0_FUN_806441e0.c` |
| `0x806448C0` | `fn_806448c0` | destroy a registered stock-item state | translated | high | `806448c0_FUN_806448c0.c` |
| `0x806449E0` | `fn_806449e0` | construct the stock-item object in place | translated | medium | `806449e0_FUN_806449e0.c` |
| `0x80644B30` | `fn_80644b30` | load and bind the stock-item layout | translated | high | `80644b30_FUN_80644b30.c` |
| `0x80645820` | `fn_80645820` | register the four `dTheEnd_c` states | translated | high | `80645820_FUN_80645820.c` |
| `0x80645B60` | `fn_80645b60` | destroy a registered the-end state | translated | high | `80645b60_FUN_80645b60.c` |
| `0x806465F0` | `fn_806465f0` | register the four `dTimeUp_c` states | translated | high | `806465f0_FUN_806465f0.c` |
| `0x80646930` | `fn_80646930` | destroy a registered time-up state | translated | high | `80646930_FUN_80646930.c` |
| `0x80645C80` | `fn_80645c80` | construct the time-up object in place | translated | medium | `80645c80_FUN_80645c80.c` |
| `0x807ADFA0` | `fn_807adfa0` | register the four `dScMulti_c` states | translated | high | `807adfa0_FUN_807adfa0.c` |
| `0x807AE2E0` | `fn_807ae2e0` | destroy a registered multi-result state | translated | high | `807ae2e0_FUN_807ae2e0.c` |
| `0x807AD000` | `fn_807ad000` | construct the multi-result screen in place | translated | medium | `807ad000_FUN_807ad000.c` |
| `0x807AF240` | `fn_807af240` | register the two `dScResult_c` states | translated | high | `807af240_FUN_807af240.c` |
| `0x807AF400` | `fn_807af400` | destroy a registered result state | translated | high | `807af400_FUN_807af400.c` |
| `0x807AE720` | `fn_807ae720` | construct the result screen in place | translated | medium | `807ae720_FUN_807ae720.c` |
| `0x807B4440` | `fn_807b4440` | register the six `dScGameOver_c` states | translated | high | `807b4440_FUN_807b4440.c` |
| `0x807B48F0` | `fn_807b48f0` | destroy a registered game-over state | translated | high | `807b48f0_FUN_807b48f0.c` |
| `0x807B3C70` | `fn_807b3c70` | construct the game-over screen in place | translated | medium | `807b3c70_FUN_807b3c70.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 21 high confidence, 9 medium.
The medium rows are the nine constructors (`0x80627940`, `0x80631FE0`,
`0x8063B370`, `0x80641D20`, `0x806449E0`, `0x80645C80`, `0x807AD000`,
`0x807AE720`, `0x807B3C70`), whose class attribution is inferred from the
adjacent table and vtable family.

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x806043C0` | `dCharacterChangeSelectBase_c` state table, 20 states and 3,860 bytes; still a mechanical generator candidate. |
| `0x80618D00` | `dFileSelect_c` state table, 44 `StateID` references and 8,344 bytes; still the largest single table in the module. |
| `0x80622010`, `0x807AC320`, `0x807BD110`, `0x807C3D70` | 17-, 11-, 13- and 35-state menu/select tables; best handled with a table generator. |
| `0x8062A2C0`, `0x8062F050`, `0x806393F0`, `0x8063D7A0` | Remaining 18-20-state course/result tables in the same family. |
| `0x807A9BC0`, `0x807B0C50`, `0x807B3140` | Remaining 8-10-state result/world tables; left so this pass stays within 30 rows. |
| `0x80645F90`, `0x80646180`, `0x806463B0` | `dTheEnd_c`/`dTimeUp_c` state handlers; the table-and-destructor slice is closed first. |
| `0x80673F50` | `daCastleLemmyThrowBall_c` spawn handler; still rejected because Ghidra reconstructs paired-single loads with `unaff_GQR0`/`ldexpf`. |

## Unknowns preserved

1. **Constructor class attribution.** None of the nine constructors contains a
   class string. Each is tied to its class only by the vtable family and
   adjacency to the table; those rows are `medium` confidence and the Rust
   carries `// inferred:` notes.
2. **`dStockItem_c` class-name callback.** `0x806448A0` is stored through
   `fn_80600160` but has no exported function body. It is kept by address with
   an `// unknown:` note.
3. **`dTheEnd_c` constructor.** No exported body references the table's
   descriptor block, so the constructor is not claimed.
4. **One-time initialisers.** The `dStockItem_c` prelude writes
   `0x807E20EC..0x807E2100` and fills `0x808278C0/C4` plus the byte
   `0x808278D6 = 0x78`; the consumers are outside this slice. The Rust
   preserves the writes and marks the role unclear where needed.
5. **Main.dol and NW4R helpers.** `0x8006C420` (actor base constructor),
   `0x800E19B0` (screen base constructor), `0x800C8950` (layout),
   `0x8015FC90` (interface wrapper), `0x8015F600` (callback table),
   `0x800C8CB0`/`0x800C8EB0`/`0x800C8F20`/`0x800C8E40` (layout binders),
   `0x802DC9A0` (array builder), `0x802D7AA0` (element constructor) and
   `0x802BF3D0`/`0x802BFF30` (sub-object constructors) are emitted by
   address, never renamed.
6. **In-place constructors.** The nine constructors take the object as their
   first argument; the allocator and factory are outside the translated slice,
   so their callers and allocated sizes are not claimed.
7. **Compiler thunks.** The `0x802DCD74`/`0x802DCDC0` save/restore thunks that
   bracket several table functions are omitted because they have no
   Rust-visible effect; see `docs/COVERAGE_r80260_802ed.md`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 286 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are in `src/runtime/bt.rs`, `src/runtime/input.rs` and
`src/game/enemies.rs`, and `src/game/bases.rs` itself produces no errors or
warnings.
