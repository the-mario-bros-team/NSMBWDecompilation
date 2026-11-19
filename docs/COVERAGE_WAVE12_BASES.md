# Coverage - wave 12 (`d_basesNP` course/menu state actors)

## How to read this file

This page records the twelfth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: eight `StateID` registration tables, their registered
   destructors and in-place constructors, two array-element helpers, and one
   throw-ball actor table with its destructor, handler getter and constructor.
2. **Why these functions?** - *Selection* explains why this pass closed a
   contiguous family of course/menu tables instead of starting the large
   `dFileSelect_c` table.
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
(`0x8006xxxx`, `0x800Cxxxx`, `0x8015xxxx`, `0x8042xxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 12 |
|---|---:|
| New translated functions | 30 |
| State registration tables | 9 |
| Actor constructors (in-place or allocating) | 9 |
| Registered-state destructors | 8 |
| Array-element constructor/destructor pairs | 1 |
| Handler/data getters | 1 |
| Selected bytes | 11,004 |
| Translated functions in `src/game/bases.rs` (cumulative) | 256 |

The 30 new bodies select 11,004 of the 1,109,860 indexed bytes (0.99 %) and
30 of the 5,409 eligible entries (0.55 %). Confidence is **21 high** and
**9 medium**; there are no `partial` or `stub` rows. The total is 256 function
bodies in `src/game/bases.rs`.

## Selection

The pass stayed evidence-first and lifecycle-first:

1. **Started from `functions.tsv`, the existing `bases.rs` and
   `ledger/functions.csv`.** All 226 addresses already translated by waves
   2-11 were removed before selection; none of the 30 new addresses appears in
   an earlier ledger row.
2. **Closed a contiguous course/menu family.** A scan of the remaining 79
   `StateID` tables sorted by size found eight 4-7-state tables between
   `0x8060CBE0` and `0x8061E1B0` with the same shape: a nine-word parameter
   block per state, a shared vtable, one standard destructor, and an in-place
   constructor exactly `0x2C0` bytes after the table.
3. **Added the smallest remaining actor table.** `daCastleLemmyThrowBall_c`
   (`0x80673830`) was the only remaining single-state class table. It reuses
   the wave-11 Lemmy context object and array helpers, so it was taken with its
   destructor, data getter and constructor.
4. **Included every registered lifecycle helper.** Each table's destructor and
   adjacent in-place constructor were translated. The two `dCourseTimeUp_c`
   array-element helpers (`0x8060F590`/`0x8060C0F0`) were included because the
   constructor passes both to `0x802DC9A0`.
5. **Reused the verified registration template.** Every state builds a
   0x30-byte descriptor at stride 0x40, reads nine parameter words, installs a
   shared state vtable, and calls [`fn_80600160`](../src/game/bases.rs) with the
   `descriptor - 0x10` link node. `register_state_desc` covers all 29
   descriptors selected here; `daCastleLemmyThrowBall_c` additionally stores
   the getter result at `descriptor + 0x30`.
6. **Read the real state strings.** Names were extracted from the REL data
   image at the referenced addresses, not guessed from truncated Ghidra
   symbols. Constructors carry no class string, so their attribution is marked
   `// inferred:` from the vtable family and adjacency to the table.

## Clusters

### Course clear (`0x8060CBE0..0x8060D1B0`)

- `dCourseClear_c` (`0x8060CBE0`): five states (`OnSatgeSetup`,
  `OnSatgeAnimeEndCheck`, `Disp`, `ExitAnimeEndCheck`, `GoalDeMoEndWait`),
  vtable `0x807D3BD0`, destructor `0x8060CFE0`.
- Constructor `0x8060D100` installs class vtable `0x807D3F58` and the
  interface vtable `0x807D3FA8`.

### Course time-up (`0x8060C0F0..0x8060F590`)

- `dCourseTimeUp_c` (`0x8060F010`): four states (`DispWait`,
  `OpenAnimeEndWait`, `DispEndWait`, `ExitAnimeEndWait`), vtable `0x807D4664`,
  destructor `0x8060F370`. A one-time guard writes `0x50` to `0x80823840`.
- Constructor `0x8060F490` builds two five-element arrays of `0x128`-byte
  elements at `+0x20C` and `+0x7D4`.
- Element constructor `0x8060F590` and destructor `0x8060C0F0` share vtable
  `0x80329958`.

### Date file (`0x806108B0..0x80610FF0`)

- `dDateFile_c` (`0x806108B0`): seven states (`InitialSetup`, `ActionWait`,
  `OnAnimeEndWait`, `HitAnimeEndWait`, `InterruptSaveMarkExit`,
  `SelectCopyFileAnimeStart`, `OffAnimeEndWait`), vtable `0x807D4D48`,
  destructor `0x80610E20`.
- Constructor `0x80610F40` installs class vtable `0x807D4E68` and interface
  vtable `0x807D4EB8`.

### Demo message (`0x80611960..0x80611EB0`)

- `dDemoMessage_c` (`0x80611960`): four states (`StartWait`,
  `StartAnimeEndWait`, `ExitWait`, `ExitAnimeEndWait`), vtable `0x807D50F8`,
  destructor `0x80611CE0`. A one-time block copies five configuration words
  from `0x807C6A20`.
- Constructor `0x80611E00` installs class vtable `0x807D51F8` and interface
  vtable `0x807D5248`.

### Draw game (`0x80612740..0x80612C50`)

- `dDrawGame_c` (`0x80612740`): four states (`FadeInEndWait`,
  `OnStageAnimeEndWait`, `DispEndWait`, `ExitAnimeEndWait`), vtable
  `0x807D5484`, destructor `0x80612A80`.
- Constructor `0x80612BA0` installs class vtable `0x807D5740` and interface
  vtable `0x807D5790`.

### Easy pairing (`0x80613CA0..0x806143E0`)

- `dEasyPairing_c` (`0x80613CA0`): seven states (`DispWait`,
  `OpenAnimeEndWait`, `StartWait`, `keyWait`, `StopStartWait`, `StopEndWait`,
  `ExitAnimeEndWait`), vtable `0x807D5AA8`, destructor `0x80614210`.
- Constructor `0x80614330` installs class vtable `0x807D5C40` and interface
  vtable `0x807D5C90`.

### Game over (`0x8061BCE0..0x8061C200`)

- `dGameOver_c` (`0x8061BCE0`): four states (`FadeInEndWait`,
  `OnStageAnimeEndWait`, `DispEndWait`, `ExitAnimeEndWait`), vtable
  `0x807D7D64`, destructor `0x8061C0A0`. The prelude copies main.dol/REL
  configuration words and links the class-name node with callback `0x8061C080`
  (no exported body).
- Constructor `0x8061C1C0` installs class vtable `0x807D7E90`.

### Message window (`0x8061E1B0..0x8061E6D0`)

- `dMessageWindow_c` (`0x8061E1B0`): four states (`StartWait`,
  `StartAnimeEndWait`, `ExitWait`, `ExitAnimeEndWait`), vtable `0x807D8740`,
  destructor `0x8061E4F0`.
- Constructor `0x8061E610` installs class vtable `0x807D8BE8` and interface
  vtable `0x807D8C38`.

### Lemmy throw ball (`0x80673830..0x80673E20`)

- `daCastleLemmyThrowBall_c` (`0x80673830`): one state (`Move`), vtable
  `0x807EA988`, destructor `0x80673A50`. The body rebuilds the wave-11 Lemmy
  context object at `0x80829CEC` and a six-element `0x28`-byte array at
  `0x80829D18`.
- Handler getter `0x80673AB0` returns the shared block `0x8082C47C`.
- Constructor `0x80673DA0` allocates `0x3C0` bytes and installs class vtable
  `0x807EAA24`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave12-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8060CBE0` | `fn_8060cbe0` | register the five `dCourseClear_c` states | translated | high | `8060cbe0_FUN_8060cbe0.c` |
| `0x8060CFE0` | `fn_8060cfe0` | destroy a registered course-clear state | translated | high | `8060cfe0_FUN_8060cfe0.c` |
| `0x8060D100` | `fn_8060d100` | construct the course-clear screen in place | translated | medium | `8060d100_FUN_8060d100.c` |
| `0x8060F010` | `fn_8060f010` | register the four `dCourseTimeUp_c` states | translated | high | `8060f010_FUN_8060f010.c` |
| `0x8060F370` | `fn_8060f370` | destroy a registered time-up state | translated | high | `8060f370_FUN_8060f370.c` |
| `0x8060F490` | `fn_8060f490` | construct the time-up screen and two arrays | translated | medium | `8060f490_FUN_8060f490.c` |
| `0x8060F590` | `fn_8060f590` | construct one time-up array element | translated | high | `8060f590_FUN_8060f590.c` |
| `0x8060C0F0` | `fn_8060c0f0` | destroy one time-up array element | translated | high | `8060c0f0_FUN_8060c0f0.c` |
| `0x806108B0` | `fn_806108b0` | register the seven `dDateFile_c` states | translated | high | `806108b0_FUN_806108b0.c` |
| `0x80610E20` | `fn_80610e20` | destroy a registered date-file state | translated | high | `80610e20_FUN_80610e20.c` |
| `0x80610F40` | `fn_80610f40` | construct the date-file screen in place | translated | medium | `80610f40_FUN_80610f40.c` |
| `0x80611960` | `fn_80611960` | register the four `dDemoMessage_c` states | translated | high | `80611960_FUN_80611960.c` |
| `0x80611CE0` | `fn_80611ce0` | destroy a registered demo-message state | translated | high | `80611ce0_FUN_80611ce0.c` |
| `0x80611E00` | `fn_80611e00` | construct the demo-message screen in place | translated | medium | `80611e00_FUN_80611e00.c` |
| `0x80612740` | `fn_80612740` | register the four `dDrawGame_c` states | translated | high | `80612740_FUN_80612740.c` |
| `0x80612A80` | `fn_80612a80` | destroy a registered draw-game state | translated | high | `80612a80_FUN_80612a80.c` |
| `0x80612BA0` | `fn_80612ba0` | construct the draw-game screen in place | translated | medium | `80612ba0_FUN_80612ba0.c` |
| `0x80613CA0` | `fn_80613ca0` | register the seven `dEasyPairing_c` states | translated | high | `80613ca0_FUN_80613ca0.c` |
| `0x80614210` | `fn_80614210` | destroy a registered easy-pairing state | translated | high | `80614210_FUN_80614210.c` |
| `0x80614330` | `fn_80614330` | construct the easy-pairing screen in place | translated | medium | `80614330_FUN_80614330.c` |
| `0x8061BCE0` | `fn_8061bce0` | register the four `dGameOver_c` states | translated | high | `8061bce0_FUN_8061bce0.c` |
| `0x8061C0A0` | `fn_8061c0a0` | destroy a registered game-over state | translated | high | `8061c0a0_FUN_8061c0a0.c` |
| `0x8061C1C0` | `fn_8061c1c0` | construct the game-over screen in place | translated | medium | `8061c1c0_FUN_8061c1c0.c` |
| `0x8061E1B0` | `fn_8061e1b0` | register the four `dMessageWindow_c` states | translated | high | `8061e1b0_FUN_8061e1b0.c` |
| `0x8061E4F0` | `fn_8061e4f0` | destroy a registered message-window state | translated | high | `8061e4f0_FUN_8061e4f0.c` |
| `0x8061E610` | `fn_8061e610` | construct the message-window screen in place | translated | medium | `8061e610_FUN_8061e610.c` |
| `0x80673830` | `fn_80673830` | register `daCastleLemmyThrowBall_c::StateID_Move` | translated | high | `80673830_FUN_80673830.c` |
| `0x80673A50` | `fn_80673a50` | destroy a registered throw-ball state | translated | high | `80673a50_FUN_80673a50.c` |
| `0x80673AB0` | `fn_80673ab0` | return shared block `0x8082C47C` | translated | high | `80673ab0_FUN_80673ab0.c` |
| `0x80673DA0` | `fn_80673da0` | construct the `0x3C0`-byte throw-ball actor | translated | medium | `80673da0_FUN_80673da0.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 21 high confidence, 9 medium.
The medium rows are the nine constructors (`0x8060D100`, `0x8060F490`,
`0x80610F40`, `0x80611E00`, `0x80612BA0`, `0x80614330`, `0x8061C1C0`,
`0x8061E610`, `0x80673DA0`), whose class attribution is inferred from the
adjacent table and vtable family.

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x80618D00` | `dFileSelect_c` state table, 44 `StateID` references and 8,344 bytes; still the largest single table in the module. |
| `0x806043C0`, `0x8063D7A0`, `0x8062A2C0`, `0x8062F050` | 20-, 20-, 19- and 18-state menu tables; same mechanical family, better handled with a table generator. |
| `0x80622010`, `0x806393F0`, `0x807BD110`, `0x8060B680`, `0x807AC320` | 17-, 16-, 13-, 11- and 11-state menu/world-map tables. |
| `0x80631950`, `0x806415D0`, `0x806441E0` | remaining 7-8-state select/result tables in the same family; left so this pass stays within 30 rows. |
| `0x80645820`, `0x806465F0`, `0x80627660`, `0x807AF240`, `0x807ADFA0` | 2-4-state end/result tables adjacent to the `dTimeUp_c`/`dTheEnd_c` constructors; the constructors are not contiguous, so they need a separate lookup. |
| `0x80673F50` | `daCastleLemmyThrowBall_c` spawn handler; rejected because Ghidra reconstructs paired-single loads with `unaff_GQR0`/`ldexpf`. |

## Unknowns preserved

1. **Constructor class attribution.** None of the nine constructors contains a
   class string. Each is tied to its class only by the vtable family and
   adjacency to the table; those rows are `medium` confidence and the Rust
   carries `// inferred:` notes.
2. **Class-name node callbacks.** `0x8061C080` (`dGameOver_c`) and `0x806739D0`
   (`daCastleLemmyThrowBall_c`) are stored through `fn_80600160` but have no
   exported function body. They are kept by address with `// unknown:` notes.
3. **One-time initialisers.** `0x80823840 = 0x50` in the time-up table and the
   five-word copy in the demo-message table have no named consumer; the Rust
   preserves the writes and marks them `// unknown:` where the role is unclear.
4. **Main.dol helpers.** `0x8006C420` (base constructor), `0x800C8950`
   (layout), `0x8015FC90` (NW4R interface wrapper), `0x801628C0` (allocator),
   `0x80064110` (base constructor), `0x8016D4B0`/`0x802D7B20` (element
   teardown), `0x802D7AA0` (element base constructor) and `0x802DC9A0`
   (array builder) are emitted by address, never renamed.
5. **In-place constructors.** The nine constructors take the object as their
   first argument; the allocator is outside the translated slice. The eight
   `d*` constructors are reached from an untranslated factory, so their caller
   and allocated size are not claimed.
6. **Truncated Ghidra names.** The state strings were read from the REL data
   image, so the Rust comments use the full names; the Ghidra symbols remain
   truncated at 32 characters in the evidence files.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 256 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are in `src/runtime/bt.rs` and `src/runtime/input.rs`, and
`src/game/bases.rs` itself produces no errors or warnings.
