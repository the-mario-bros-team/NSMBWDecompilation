# Coverage - wave 10 (`d_basesNP` boss props, lift props, train and mini-game gun actors)

## How to read this file

This page records the tenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 29 functions appended to
   `src/game/bases.rs`: ten `StateID` registration tables, eleven registered
   state destructors, four actor constructors, and four initialiser/model-load
   routines.
2. **Why these functions?** - *Selection* explains why the pass took complete
   table/destructor pairs first and only then the adjacent constructors and
   initialisers that have explicit control flow.
3. **What is still missing?** - *Deferred candidates* records the neighbouring
   classes that were deliberately left alone, including the paired-single
   `daLiftScalesStep_c` constructor. *Unknowns preserved* lists every unresolved
   callback instead of guessing it.

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

| Metric | Wave 7 | Wave 9 | Wave 10 | Cumulative (`game/bases.rs`) |
|---|---:|---:|---:|---:|
| Translated functions in `src/game/bases.rs` | 30 | 24 | 29 | 196 |
| Actor classes with a complete table+destructor+constructor triple | 10 | 8 | 4 | 22 since wave 7 |
| State registrations covered | 40 | 10 | 39 | 89 since wave 7 |
| Selected bytes | 11,060 | 4,940 | 11,612 | 27,612 since wave 7 |

The 29 new bodies select 11,612 of the 1,109,860 indexed bytes (1.05 %) and
29 of the 5,409 eligible entries (0.54 %). Confidence is **21 high** and
**8 medium**; there are no `partial` or `stub` rows. The total is 196 function
bodies in `src/game/bases.rs`.

## Selection

This pass stayed evidence-first and lifecycle-first:

1. **Started from `functions.tsv` and the existing `bases.rs`.** Every address
   already translated in waves 2-9 was removed from the candidate set before
   selection.
2. **Preferred complete pairs.** Ten `StateID` tables were chosen because each
   one is a self-contained registration body and its callback is also the
   registered-state destructor: `daBossKoopaJrDemo_c`,
   `daBossKoopaRubble_c`, `daBossKoopaScaffold_c`, `daFireBlitz_c`,
   `daLiftObjbgVMove_c`, `daLiftScalesStep_c`, `daLiftZenShiroYogan_c`,
   `daLineTrain_c`, `daMiniGameGunBatteryMgrObj_c`, and
   `daWmSandPillar_c`.
3. **Added only constructors/initialisers with complete flow.** The four
   constructors (`0x8067BA70`, `0x806CE9A0`, `0x806E7870`, `0x806EE630`) and
   the four mini-game initialisers (`0x806F9D50`, `0x806F9E20`, `0x806FA270`,
   `0x807796C0`) contain only explicit calls, stores and constant loads. The
   paired-single `daLiftScalesStep_c` constructor remains deferred.
4. **Reused the verified registration template.** Every table builds 0x30-byte
   descriptors at stride 0x40, reads nine parameter words, installs the shared
   state vtable, and calls [`fn_80600160`](../src/game/bases.rs) with the
   `descriptor - 0x10` link node. The existing `register_state_desc` helper
   covers all 39 descriptors selected here.
5. **Verified class names against the state strings.** The `StateID` strings
   were read from the REL data section, not guessed. Constructors carry no class
   string, so their attribution is marked `// inferred:` from the vtable they
   install and from adjacency to the table.

## Clusters

### Koopa boss props (`0x80664BA0..0x806674A0`)

- `daBossKoopaJrDemo_c` (`0x80664BA0`): five states (`Rea...`, three `Bat...`,
  `Dem...`), vtable `0x807E7C68`/`0x807E7C9C`, destructors `0x806650C0` and
  `0x80665060`, plus a class-name node callback `0x80665040`.
- `daBossKoopaRubble_c` (`0x80666260`): four states (`Wa...`, `Jum...`,
  `Fal...`, `Cra...`), vtable `0x807E7FD8`, destructor `0x80666720`, and a
  one-shot timing/scale block at `0x807E7D3C..0x807E7D9C`.
- `daBossKoopaScaffold_c` (`0x806671D0`): two states, vtable `0x807E8290`,
  destructor `0x806674A0`, and a ten-word tuning block at
  `0x807E80CC..0x807E80F0`.

### Fire prop (`0x8067B5B0..0x8067BA70`)

- `daFireBlitz_c` (`0x8067B5B0`): four states (`Move`, `Vanish`, `EatIn`,
  `EatNow`), vtable `0x807EC358`, destructor `0x8067B8F0`; constructor
  `0x8067BA70` (`0x558` bytes, class vtable `0x807EC3B0`).

### Lift props (`0x806CE660..0x806E76F0`)

- `daLiftObjbgVMove_c` (`0x806CE660`): two states (`NoDi...`, `Move...`),
  vtable `0x807FA598`, destructor `0x806CE820`; constructor `0x806CE9A0`
  (`0x668` bytes, class vtable `0x807FA668`).
- `daLiftScalesStep_c` (`0x806DBED0`): four states (`Hold...`, `Rele...`,
  `NoDi...`, `Disp...`), vtable `0x807FBD80`, destructor `0x806DC210`.
- `daLiftZenShiroYogan_c` (`0x806E7530`): two states, vtable `0x807FDA5C`,
  destructor `0x806E76F0`; constructor `0x806E7870` (`0x890` bytes, class
  vtable `0x807FDAC8`).

### Line train (`0x806EE170..0x806EE630`)

- `daLineTrain_c` (`0x806EE170`): four states (`Wait...`, `Search...`,
  `WaitParen...`, `Move...`), vtable `0x807FE794`, destructor `0x806EE4B0`;
  constructor `0x806EE630` (`0x6F8` bytes, vtables `0x807F7D88` and
  `0x807FE8B8`).

### Mini-game gun battery (`0x806F98C0..0x806FA270`)

- `daMiniGameGunBatteryMgrObj_c` (`0x806F98C0`): three truncated state names,
  vtable `0x80802864`, destructor `0x806F9B40`.
- `0x806F9D50` initialises the movement/limit fields and dispatches virtual
  slot `0xD4`; `0x806F9E20` loads `g3d/MG_kinopio.brres` and the
  `KDB`/`KDH`/`P_rcha`/`PH_wait` parts; `0x806FA270` finishes the motion and
  sound setup.

### World-map sand pillar (`0x80778C20..0x807796C0`)

- `daWmSandPillar_c` (`0x80778C20`): nine states (`Ready`, two `Bottom`, two
  `TopWai...`, `MoveRe...`, `MoveUp`, `MoveDo...`, and a third `TopWai...`),
  vtable `0x80817D48`, destructor `0x80779380`, five configuration copies, and
  a class-name node callback `0x80779360`; `0x807796C0` binds the
  `cobSunkenShip` model at `+0x1A4`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave10-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80664BA0` | `fn_80664ba0` | register the five `daBossKoopaJrDemo_c` states and config | translated | high | `80664ba0_FUN_80664ba0.c` |
| `0x80665060` | `fn_80665060` | destroy a registered `daBossKoopaJrDemo_c` demo state | translated | high | `80665060_FUN_80665060.c` |
| `0x806650C0` | `fn_806650c0` | destroy one of the four shared battle states | translated | high | `806650c0_FUN_806650c0.c` |
| `0x80666260` | `fn_80666260` | register the four `daBossKoopaRubble_c` states and timing block | translated | high | `80666260_FUN_80666260.c` |
| `0x80666720` | `fn_80666720` | destroy a registered rubble state | translated | high | `80666720_FUN_80666720.c` |
| `0x806671D0` | `fn_806671d0` | register the two `daBossKoopaScaffold_c` states and tuning block | translated | high | `806671d0_FUN_806671d0.c` |
| `0x806674A0` | `fn_806674a0` | destroy a registered scaffold state | translated | high | `806674a0_FUN_806674a0.c` |
| `0x8067B5B0` | `fn_8067b5b0` | register the four `daFireBlitz_c` states | translated | high | `8067b5b0_FUN_8067b5b0.c` |
| `0x8067B8F0` | `fn_8067b8f0` | destroy a registered fire-blitz state | translated | high | `8067b8f0_FUN_8067b8f0.c` |
| `0x8067BA70` | `fn_8067ba70` | construct the `0x558`-byte fire-blitz actor | translated | medium | `8067ba70_FUN_8067ba70.c` |
| `0x806CE660` | `fn_806ce660` | register the two `daLiftObjbgVMove_c` states | translated | high | `806ce660_FUN_806ce660.c` |
| `0x806CE820` | `fn_806ce820` | destroy a registered moving-lift state | translated | high | `806ce820_FUN_806ce820.c` |
| `0x806CE9A0` | `fn_806ce9a0` | construct the `0x668`-byte moving lift | translated | medium | `806ce9a0_FUN_806ce9a0.c` |
| `0x806DBED0` | `fn_806dbed0` | register the four `daLiftScalesStep_c` states | translated | high | `806dbed0_FUN_806dbed0.c` |
| `0x806DC210` | `fn_806dc210` | destroy a registered scales-step state | translated | high | `806dc210_FUN_806dc210.c` |
| `0x806E7530` | `fn_806e7530` | register the two `daLiftZenShiroYogan_c` states | translated | high | `806e7530_FUN_806e7530.c` |
| `0x806E76F0` | `fn_806e76f0` | destroy a registered Zen-Shiro lift state | translated | high | `806e76f0_FUN_806e76f0.c` |
| `0x806E7870` | `fn_806e7870` | construct the `0x890`-byte Zen-Shiro lift | translated | medium | `806e7870_FUN_806e7870.c` |
| `0x806EE170` | `fn_806ee170` | register the four `daLineTrain_c` states | translated | high | `806ee170_FUN_806ee170.c` |
| `0x806EE4B0` | `fn_806ee4b0` | destroy a registered line-train state | translated | high | `806ee4b0_FUN_806ee4b0.c` |
| `0x806EE630` | `fn_806ee630` | construct the `0x6F8`-byte line train | translated | medium | `806ee630_FUN_806ee630.c` |
| `0x806F98C0` | `fn_806f98c0` | register the three mini-game gun battery states | translated | high | `806f98c0_FUN_806f98c0.c` |
| `0x806F9B40` | `fn_806f9b40` | destroy a registered gun-battery state | translated | high | `806f9b40_FUN_806f9b40.c` |
| `0x806F9D50` | `fn_806f9d50` | initialise the gun-battery movement/limit fields | translated | medium | `806f9d50_FUN_806f9d50.c` |
| `0x806F9E20` | `fn_806f9e20` | load the gun-battery `MG_kinopio` model parts | translated | medium | `806f9e20_FUN_806f9e20.c` |
| `0x806FA270` | `fn_806fa270` | finish the gun-battery motion and sound setup | translated | medium | `806fa270_FUN_806fa270.c` |
| `0x80778C20` | `fn_80778c20` | register the nine `daWmSandPillar_c` states and config | translated | high | `80778c20_FUN_80778c20.c` |
| `0x80779380` | `fn_80779380` | destroy a registered sand-pillar state | translated | high | `80779380_FUN_80779380.c` |
| `0x807796C0` | `fn_807796c0` | load the `cobSunkenShip` sand-pillar model | translated | medium | `807796c0_FUN_807796c0.c` |

**Totals:** 29 translated, 0 partial, 0 stub. 21 high confidence, 8 medium
(the four constructors and four initialiser/model-load routines).

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x806DB340` | `daLiftScalesStep_c` constructor; still a Ghidra paired-single reconstruction (`unaff_GQR0`, `ldexpf`). The table and destructor are translated here; the constructor waits for a paired-single pass. |
| `0x806DC5B0` | `daLiftScalesStep_c` model loader (`test_lift` BRRES); belongs with the deferred constructor. |
| `0x806E2180`, `0x806E3090` | `daLiftWaterWheel_c` execute/update routines; they carry float state math and no class string beyond the existing table. |
| `0x806EE8D0`, `0x806EE8F0`, `0x806EEBC0`, `0x806EECA0` | `daLineTrain_c` state handlers and post-construction update code; the table, destructor and constructor are the complete lifecycle slice translated here. |
| `0x806FA310`, `0x806FA3D0`, `0x806FA500` | remaining mini-game gun battery handlers; they are state-machine tails after the initialisers translated here. |
| `0x80672500`, `0x80672B20`, `0x80672BE0`, `0x80672C60`, `0x80672CC0` | `daCastleLemmyBall_c` six-state table, its array builder and three destructors; the table also initialises an element array, so it needs an array-model pass. |
| `0x806A0B30`, `0x806A11D0`, `0x806A1230` | `daJrClownC_c` seven-state table and destructors; the table has two configuration copies, so it was left for a batch that can share the guard-block helper. |
| `0x806083B0`, `0x8060CBE0`, `0x8060F010`, `0x80618D00` | menu state tables (`dCollectionCoin_c`, `dCourseClear_c`, `dCourseTimeUp_c`, `dFileSelect_c`); the 44-reference `dFileSelect_c` table is the largest remaining single table. |

## Unknowns preserved

1. **Constructor class attribution.** None of the four constructors and none of
   the four mini-game initialisers contains a class string. Each is tied to its
   class only by the vtable it installs and by adjacency to the StateID table;
   those eight rows are therefore `medium` confidence and the Rust carries
   `// inferred:` notes.
2. **Dropped `this` arguments.** `0x80066FC0` is a base constructor called by
   address; Ghidra drops its `this` argument. The Rust passes the newly
   allocated object and documents the omission. `0x806F9E20` is worse: Ghidra
   lost `this` through the `0x802DCD74` register-save thunk and used the thunk's
   return value, so the Rust restores the `actor` parameter and documents it.
3. **Class-name node callbacks.** `0x80665040` (`daBossKoopaJrDemo_c`) and
   `0x80779360` (`daWmSandPillar_c`) are stored through `fn_80600160` but have
   no exported function body. They are kept by address with `// unknown:` notes.
4. **Constant handler getters.** `0x80665120`/`0x80665130`/`0x80665140`/
   `0x80665150`, `0x806654F0`..`0x80665520` and `0x80666AD0`/`0x80666AE0` are
   12-byte getters that return main.dol handler objects. The Rust calls them
   through `call0_ret` and records the returned constant in a comment.
5. **Sub-object initialisers.** `0x800DB130`, `0x80168D10`, `0x80169CD0`,
   `0x8016A2F0`, `0x8016A5E0`, `0x802D7AA0`, `0x800690C0`, `0x800690E0`,
   `0x80064BC0`, `0x80169D90`, `0x8016A000`, `0x8016A3A0`, `0x801650D0`,
   `0x801651F0`, `0x80166830`, `0x80166EF0`, `0x80166F10`, `0x80166F40`,
   `0x80169F80`, `0x801680E0`, `0x8019BED0` and `0x800F2A20` are main.dol
   routines whose roles are inferred from their call positions. They are
   emitted by address, never renamed.
6. **Array element constructors.** The Zen-Shiro lift calls the array builder
   `0x802DC9A0` with the element constructor/destructor pair
   `0x8007F7A0`/`0x8007F810`; the pair is kept by address with an `// unknown:`
   note.
7. **Truncated state strings.** Ghidra truncates long state names (`NoDi...`,
   `Rele...`, `MoveRe...`, `TopWai...`). The Rust names only the visible prefix
   and does not invent the missing characters. The mini-game gun battery names
   are truncated to a bare address, so no prefix is claimed there.
8. **Two-byte tuning stores.** The rubble block writes 600 and 0x5DC at
   two-byte intervals (`0x807E7D50/52/54`, `0x807E7D8C/8E/90`). The Rust uses
   `write_u16` and marks the width as observed; the values fit both a u16 and a
   small integer field.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 196 translated functions; this pass adds 29,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are in `src/runtime/bt.rs` and `src/runtime/input.rs`, and
`src/game/bases.rs` itself produces no errors or warnings.
