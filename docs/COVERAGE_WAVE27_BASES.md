# Coverage — wave 27 (`d_basesNP`: `remo_lift_up` actor and twin lift manager)

## How to read this file

This page records the twenty-seventh translation pass over `d_basesNP`, the
base gameplay REL (`game::bases`). It answers:

1. **What was added?** — *Translated functions* lists the 28 new bodies in
   `src/game/bases.rs`: the twin `0x9B38`-byte manager constructor, the
   `0x9B18`-byte `remo_lift_up` actor constructor/destructor, its BRRES model
   loader, state initialiser, pose selectors, three part-attachment helpers,
   parameter tween, rider collector and the achievement/effect/retarget
   helpers.
2. **Why these functions?** — *Selection* explains how the pass continued the
   address-ordered sweep above the wave-26 window and which decompiler outputs
   were skipped.
3. **What is still missing?** — *Deferred candidates* records the paired-single
   bodies, the two truncated `halt_baddata` bodies and the two release helpers.

Column meanings:

- **Status** — `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** — strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, table meaning or
  called body is inferred; `low` = mostly guesswork.
- **Evidence** — file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8004xxxx`, `0x8006xxxx`, `0x8016xxxx`, `0x8019xxxx`, `0x8023xxxx`,
`0x8042xxxx`, `0x807Cxxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output (or in the instruction stream where the C drops an argument);
`// inferred:` = role guessed from callers/shape; `// unknown:` = unresolved;
`// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 27 |
|---|---:|
| New translated functions | 28 |
| Twin `0x9B38` manager constructor (`0x806CD210`) | 1 |
| `remo_lift_up` constructors/lifecycle (`0x806CDE60`, `0x806CE5A0`) | 2 |
| `remo_lift_up` state and pose bodies (`0x806CD950`, `0x806CE290`..`0x806CEB40`) | 6 |
| `remo_lift_up` part attachment and model loader (`0x806CEFC0`..`0x806CF880`) | 7 |
| `remo_lift_up` rider, tween, effect and retarget (`0x806CF6A0`..`0x806D0820`) | 12 |
| Selected bytes | 5,564 |
| Translated functions in `src/game/bases.rs` (cumulative) | 669 (667 translated, 2 partial) |
| Cumulative selected bytes | 215,468 (19.41 % of indexed bytes) |

The 28 new bodies select 5,564 of the 1,109,860 indexed bytes (0.50 %) and 28 of
the 5,409 eligible entries (0.52 %). 22 rows are `high` confidence; 6 are
`medium` (`0x806CEB40`, `0x806CF250`, `0x806CF700`, `0x806CF880`, `0x806CFD00`,
`0x806D06A0`) because a called body or a stack record is only partly recovered.
No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2–26 were removed before selection. Wave 26 stopped at
`0x806CD720`; this pass takes the twin constructor still below that window
(`0x806CD210`) and then sweeps `0x806CD760..0x806D0820` in address order.

1. **Take the `remo_lift_up` actor as a lifecycle.** `0x806CDE60` is the
   `0x9B18`-byte constructor (vtable `0x807FA3C4`); `0x806CE5A0` is its
   destructor; `0x806CEB40` is the state initialiser that copies the spawn
   point, builds the collision/bounds records and calls the model loader.
2. **Take the model loader and part binders.** `0x806CF880` opens
   `g3d/remo_lift_up.brres` and builds the visible, box and line parts;
   `0x806CEFC0`, `0x806CF060`, `0x806CF160`, `0x806CF250` and `0x806CF350`
   place those parts from the actor, its target and the cached scale block.
3. **Take the rider and achievement helpers.** `0x806CF6A0` spawns the
   `Wm_en_burst_s` burst, `0x806CF700` tweens the rider index, `0x806CFD00`
   collects which player slots still hold the lift, and
   `0x806D0510..0x806D0820` run the two-phase timer, the achievement sound,
   the counter and the retarget.
4. **Take the twin manager.** `0x806CD210` repeats the `0x9B38`-byte manager
   shape of [`fn_806cd050`] with the callback block at `+0x9A0C`.
5. **Rejected or deferred decompiler failures.** `0x806CF400`, `0x806CFB30`
   and `0x806CFFD0` are paired-single reconstructions (`ldexpf`,
   `unaff_GQR0`). `0x806D0130`, `0x806D02A0`, `0x806D0350`, `0x806D0410` and
   `0x806D09B0` belong to the same paired-single family or call it.
   `0x806D0590` and `0x806D0780` decode to `halt_baddata`.

## Clusters

### Twin `0x9B38` manager (`0x806CD210`)

- `0x806CD210` is byte-for-byte the same shape as the wave-26 constructor
  `0x806CD050`: `0x9B38` bytes, vtable `0x807FA1A8`, the `0x60 x 0x100` array
  at `+0x3F0`, the `0x0C x 0x400` array at `+0x63F4` and the callback block at
  `+0x9A0C`.

### `remo_lift_up` actor (`0x806CD760..0x806D0820`)

- `0x806CDE60` allocates `0x9B18` bytes, installs vtable `0x807FA3C4`, the
  model accessor `+0x3D4`, the two arrays and the callback block `+0x9A10`.
- `0x806CEB40` copies the spawn point to `+0x5F0..+0x5F8`, chooses the height
  from `+0x04 & 0x10000000`, writes the colour `+0xDC..+0xE4`, the collision
  block `+0x1EC`, the five-float bounds `+0x318..+0x344`, the eight-halfword
  record `+0x5D8` and then calls `0x806CF880`, `0x806CF350`, `0x806CF250`
  before selecting `0x8082D4D0`, `0x8082D510` or `0x8082D550`.
- `0x806CF880` loads `g3d/remo_lift_up.brres` (`0x807FA60C`) with the
  `remo_lift_up` pane `0x807FA624`, binds `+0x488`/`+0x548`, then the
  `remo_lift_up_box` pane `0x807FA640` into `+0x4C8`/`+0x5AC`, and the
  `remo_lift_up_line` pane `0x807FA654` into `+0x508`.
- `0x806CFD00` walks the rider list at `+0x3E0`, reads each entity's status
  byte `+0x38C` and index virtual `+0x6C`, and mirrors the six-word slot array
  into the two `+0x618`/`+0x61C` pairs.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave27-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806CD210` | `fn_806cd210` | allocate the twin `0x9B38`-byte manager | translated | high | `806cd210_FUN_806cd210.c` |
| `0x806CD760` | `fn_806cd760` | test the actor against the height probe | translated | high | `806cd760_FUN_806cd760.c` |
| `0x806CD950` | `fn_806cd950` | choose the next pose and default state | translated | high | `806cd950_FUN_806cd950.c` |
| `0x806CDCE0` | `fn_806cdce0` | release the lift and optionally free it | translated | high | `806cdce0_FUN_806cdce0.c` |
| `0x806CDE60` | `fn_806cde60` | allocate and construct the `0x9B18`-byte lift | translated | high | `806cde60_FUN_806cde60.c` |
| `0x806CE290` | `fn_806ce290` | keep the model idle until active | translated | high | `806ce290_FUN_806ce290.c` |
| `0x806CE2D0` | `fn_806ce2d0` | advance the model and callback block | translated | high | `806ce2d0_FUN_806ce2d0.c` |
| `0x806CE430` | `fn_806ce430` | reset the activation flag and colour | translated | high | `806ce430_FUN_806ce430.c` |
| `0x806CE4B0` | `fn_806ce4b0` | choose the sibling pose and state | translated | high | `806ce4b0_FUN_806ce4b0.c` |
| `0x806CE5A0` | `fn_806ce5a0` | destroy the lift and free it in delete mode | translated | high | `806ce5a0_FUN_806ce5a0.c` |
| `0x806CEB40` | `fn_806ceb40` | initialise the lift, load parts and select the state | translated | medium | `806ceb40_FUN_806ceb40.c` |
| `0x806CEFC0` | `fn_806cefc0` | build the default transform and bind `+0x488` | translated | high | `806cefc0_FUN_806cefc0.c` |
| `0x806CF060` | `fn_806cf060` | place `+0x4C8` between actor and target | translated | high | `806cf060_FUN_806cf060.c` |
| `0x806CF160` | `fn_806cf160` | place `+0x508` from the scale block | translated | high | `806cf160_FUN_806cf160.c` |
| `0x806CF250` | `fn_806cf250` | rebuild the visible rope/cart node | translated | medium | `806cf250_FUN_806cf250.c` |
| `0x806CF350` | `fn_806cf350` | compute the settled attach position and scale | translated | high | `806cf350_FUN_806cf350.c` |
| `0x806CF6A0` | `fn_806cf6a0` | spawn the `Wm_en_burst_s` burst | translated | high | `806cf6a0_FUN_806cf6a0.c` |
| `0x806CF700` | `fn_806cf700` | tween the rider index and its animations | translated | medium | `806cf700_FUN_806cf700.c` |
| `0x806CF880` | `fn_806cf880` | load `remo_lift_up` and bind three parts | translated | medium | `806cf880_FUN_806cf880.c` |
| `0x806CFD00` | `fn_806cfd00` | collect the occupied rider slots | translated | medium | `806cfd00_FUN_806cfd00.c` |
| `0x806CFF80` | `fn_806cff80` | store the target actor pointer | translated | high | `806cff80_FUN_806cff80.c` |
| `0x806CFF90` | `fn_806cff90` | find the actor of type `0x1CF` | translated | high | `806cff90_FUN_806cff90.c` |
| `0x806D0230` | `fn_806d0230` | read the target height minus the offset | translated | high | `806d0230_FUN_806d0230.c` |
| `0x806D0510` | `fn_806d0510` | advance the two-phase timer | translated | high | `806d0510_FUN_806d0510.c` |
| `0x806D05C0` | `fn_806d05c0` | gate the ride and play the achievement sound | translated | high | `806d05c0_FUN_806d05c0.c` |
| `0x806D06A0` | `fn_806d06a0` | post the second achievement sound | translated | medium | `806d06a0_FUN_806d06a0.c` |
| `0x806D0720` | `fn_806d0720` | count two ticks and release the pose | translated | high | `806d0720_FUN_806d0720.c` |
| `0x806D0820` | `fn_806d0820` | retarget the lift and pick its state | translated | high | `806d0820_FUN_806d0820.c` |

**Totals:** 28 translated, 0 partial, 0 stub (22 high, 6 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806CD020`, `0x806CDE00`, `0x806CDE30`, `0x806CE940`, `0x806CE970` | 48 each | `0x802DCBFC` thunks with no independent behaviour. |
| `0x806CEAC0`, `0x806CEB00` | 64 each | Release helpers with the same shape as [`fn_806cdce0`] but no delete branch. |
| `0x806CF400` | 452 | **Rejected**: paired-single reconstruction (`ldexpf`, `unaff_GQR0`). |
| `0x806CFB30` | 456 | **Rejected**: paired-single reconstruction; also returns through `0x802DCDC0`. |
| `0x806CFFD0` | 264 | **Rejected**: paired-single reconstruction; reads `extraout_f1`. |
| `0x806D00E0` | 68 | Two `double` parameters plus a dropped word; needs an ABI review. |
| `0x806D0130`, `0x806D02A0` | 244, 176 | **Rejected**: paired-single reconstruction; `0x806D02A0` reads `unaff_GQR0`. |
| `0x806D0350`, `0x806D0410` | 188, 256 | Callers of the rejected `0x806D0130`/`0x806D02A0`; their own flow is only partly recoverable. |
| `0x806D0590`, `0x806D0780` | 36, 120 | **Rejected**: `halt_baddata` in the export. |
| `0x806D08D0` | 184 | Calls the rejected `0x806CFB30`; wait for the paired-single pass. |
| `0x806D09B0` | 1,508 | **Rejected**: `unaff_GQR0` paired-single body with unreachable blocks. |

## Unknowns preserved

1. **`0x802DCD74` / `0x802DCDC0`.** The `remo_lift_up` callbacks fetch the
   actor through `0x802DCD74` and return the next state through `0x802DCDC0`.
   The Rust models both as `call0_ret`/`abi_call1` and marks the dispatcher
   role `// unknown:`; `0x806CEB40` is the only body that passes a real value
   (`1` or `2`).
2. **`0x800DB180`.** [`fn_806ceb40`] passes five values that the C prints as
   `(double)` plus five words. The export does not include the callee, so the
   Rust uses a `f64`-by-five wrapper. The operation (bounds/scale install) is
   `// unknown:`.
3. **`0x80043A30` and `0x8005F900`.** Both print no arguments in this window.
   The Rust calls `0x80043A30` with no argument and passes the rider byte to
   `0x8005F900`, cross-checked against `0x806517C0`, where the same routine is
   called with slot indices `0..3`.
4. **Dropped receivers.** `0x806CFD00`'s virtual `+0x6C` call prints no
   receiver; the Rust passes the `+0x60` sub-object. [`fn_806cf250`]'s type
   test uses the same sub-object shape.
5. **Stack records.** The C of [`fn_806cf880`] overlaps `local_2C`/`local_30`
   and `local_34`/`local_38`; the Rust reconstructs them as adjacent word
   pairs. The six-word `local_28` of [`fn_806cfd00`] is cleared completely even
   though the C only clears four words.
6. **Class identity.** `remo_lift_up` is the BRRES/pane name; no class string
   appears in this window, so the class stays unnamed. The twin `0x9B38`
   manager also remains unnamed.
7. **Flag fields.** The `+0x04` word supplies the height choice
   (`0x10000000`), the variant (`bits 4..5`) and the `+0x62C` flag (`bit 8`);
   each meaning is `// inferred:` from this one body.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 669 functions after this pass (667 `translated`,
2 pre-existing `partial`); this pass adds 28, all with `# Original` blocks and
the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0.
