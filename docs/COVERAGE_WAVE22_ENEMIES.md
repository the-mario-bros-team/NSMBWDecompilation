# Coverage - wave 22 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-second translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found:
   the tail of the `0x80A610C0..0x80A63410` `gesso` block plus the complete
   `gessoMini` child family at `0x80A65350..0x80A66C40`.
3. **What is still missing?** - *Deferred neighbours* names the GQR-damaged
   `gesso` handlers, the two forwarders, and the rest of the `gessoMini`
   block that wave 23 should take.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  receiver is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 22 | Wave 22 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 478 | 30 | 508 |
| Of those, `translated` | 463 | 30 | 493 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| `daEnGesso_c` tail entries | - | 10 | - |
| `gessoMini` registration callbacks | - | 4 | - |
| `gessoMini` constructors / binder / destructor | - | 5 | - |
| `gessoMini` state helpers and walkers | - | 4 | - |
| `gessoMini` frame/clip/dispose handlers | - | 4 | - |
| Constant producers | - | 3 | - |
| Selected bytes | - | 3,736 | - |

The 30 new bodies cover 3,736 bytes (0.67 % of the 558,252 bytes that were
still untranslated before this pass; 3,736 / 558,252 = 0.67 %) and 30 of the
3,077 remaining function entries (0.98 %). Confidence is **23 high** and
**7 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 508 bodies, 493 `translated`, 15 `partial`, 3,047
function entries and 554,516 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Finished the `gesso` block before opening a new class.** Wave 21 closed
   at `0x80A63410` and left two clean entries on the table
   (`0x80A61180`, `0x80A61D60`) plus the clean handlers from `0x80A63480`
   through `0x80A64180`. Those ten entries are the first part of this pass.
3. **Took the `gessoMini` block from its start.** `0x80A65850` binds the
   model `gessoMini` (`0x80B48C68`) from `g3d/gessoMini.brres`
   (`0x80B48C54`) with the `swim` (`0x80B48C74`) and `dead` (`0x80B48C7C`)
   clips, using the exact binder sequence of wave 21's `fn_80a61830`. The
   class label is `// inferred:` from that model; the block owns no
   `StateID` strings in the export, so the handlers keep `fn_<address>`
   names.
4. **Took the whole small-function run `0x80A65350..0x80A66C40`.** It is
   contiguous except for the two `0x802DCBFC` forwarders (`0x80A656A0`,
   `0x80A656D0`), which wave 22 again skips for their hidden `r12` operand,
   and the unrelated large helpers after `0x80A673D0`.
5. **Excluded every export with damaged control flow.** `0x80A61E10` still
   ends in `halt_baddata()`; `0x80A628F0`, `0x80A62A30`, `0x80A635D0`,
   `0x80A637E0`, `0x80A638F0`, `0x80A63A30`, `0x80A63FF0` and `0x80A641F0`
   contain GQR/`ldexpf` artefacts in their angle or ballistic steps. None were
   guessed.
6. **Verified the shared helper ABIs against their exports.** `0x8015F4F0`
   is a `s16` angle-approach helper returning `1` on arrival; `0x80095130` is
   the base release; `0x80162920` wraps the free; `0x80162510` is the retire
   walk used earlier in the file. `0x802DC9A0` takes
   `(this, ctor, dtor, element_size, count)`.
7. **Read every numeric constant from the REL image.** `0x80B2D3E8 = 2.0`,
   `0x80B2D46C = 0.0625`, `0x80B2D480 = 0.5`, `0x80B2D47C = 48.0`,
   `0x80B2D2E8 = 4.0`, `0x80B2D4B8 = {0.0625, -0.0625}`,
   `0x80B2D4C0 = {1.0, -1.0}`, `0x80B2D4C8 = 0.0`, `0x80B2D4D8 = 10.0` and
   `0x80B2D4E8 = 1.0` were read as big-endian `f32` values. The Rust keeps
   the data addresses instead of baking the values in.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A61180` | `fn_80a61180` | Constant producer `0x80377BB4` | translated | high | `80a61180_FUN_80a61180.c` |
| `0x80A61D60` | `fn_80a61d60` | Store the snapshot forward/height offsets at `+0x164`/`+0x168` | translated | high | `80a61d60_FUN_80a61d60.c` |
| `0x80A63480` | `fn_80a63480` | Grow `+0x708` and the three scale words while attack input is held | translated | high | `80a63480_FUN_80a63480.c` |
| `0x80A63B70` | `fn_80a63b70` | Turn `+0x104` towards `+0x728`, leave for `0x80B72290` | translated | medium | `80a63b70_FUN_80a63b70.c` |
| `0x80A63C10` | `fn_80a63c10` | Forward `0.5` step with `+0x71C` clamp | translated | medium | `80a63c10_FUN_80a63c10.c` |
| `0x80A63CE0` | `fn_80a63ce0` | Backward `0.5` step with `+0x71C` clamp | translated | medium | `80a63ce0_FUN_80a63ce0.c` |
| `0x80A63DD0` | `fn_80a63dd0` | `+0x733` countdown, leave for `0x80B720D0` | translated | high | `80a63dd0_FUN_80a63dd0.c` |
| `0x80A63E60` | `fn_80a63e60` | Turn `+0x104`, leave for `0x80B72310` | translated | medium | `80a63e60_FUN_80a63e60.c` |
| `0x80A63EE0` | `fn_80a63ee0` | State entry: bind clip 4, seed `+0x10C`/`+0x70C`, arm flags | translated | high | `80a63ee0_FUN_80a63ee0.c` |
| `0x80A64180` | `fn_80a64180` | State entry: bind clip 5 and reset `+0x360` | translated | high | `80a64180_FUN_80a64180.c` |
| `0x80A65350` | `fn_80a65350` | `gessoMini` registration callback, first variant | translated | high | `80a65350_FUN_80a65350.c` |
| `0x80A653B0` | `fn_80a653b0` | `gessoMini` registration callback, second variant | translated | high | `80a653b0_FUN_80a653b0.c` |
| `0x80A65410` | `fn_80a65410` | Constant producer `0x80377BF4` | translated | high | `80a65410_FUN_80a65410.c` |
| `0x80A65500` | `fn_80a65500` | Five-link `+0x30` chain walker | translated | medium | `80a65500_FUN_80a65500.c` |
| `0x80A655E0` | `fn_80a655e0` | Compare the state-name suffix after the last `:` | translated | medium | `80a655e0_FUN_80a655e0.c` |
| `0x80A65700` | `fn_80a65700` | Allocate/construct the `0x5C8`-byte `gessoMini` actor | translated | high | `80a65700_FUN_80a65700.c` |
| `0x80A65850` | `fn_80a65850` | Load `gessoMini` and bind the body/`swim` clips | translated | high | `80a65850_FUN_80a65850.c` |
| `0x80A65CE0` | `fn_80a65ce0` | Copy a spawn position with a per-index z offset | translated | high | `80a65ce0_FUN_80a65ce0.c` |
| `0x80A65D40` | `fn_80a65d40` | Bind the `dead` clip at frame `0.0` | translated | high | `80a65d40_FUN_80a65d40.c` |
| `0x80A65E80` | `fn_80a65e80` | Bind the `swim` clip and raise `+0x5BD` | translated | high | `80a65e80_FUN_80a65e80.c` |
| `0x80A65F90` | `fn_80a65f90` | Main frame handler: drift tables, angle return, lifetime timer | translated | medium | `80a65f90_FUN_80a65f90.c` |
| `0x80A66110` | `fn_80a66110` | Release the four holders, optionally free | translated | high | `80a66110_FUN_80a66110.c` |
| `0x80A665C0` | `fn_80a665c0` | `gessoMini` registration callback, third variant | translated | high | `80a665c0_FUN_80a665c0.c` |
| `0x80A66620` | `fn_80a66620` | `gessoMini` registration callback, fourth variant | translated | high | `80a66620_FUN_80a66620.c` |
| `0x80A66680` | `fn_80a66680` | Constant producer `0x80377C74` | translated | high | `80a66680_FUN_80a66680.c` |
| `0x80A66690` | `fn_80a66690` | Constant producer `0x80377BF4` | translated | high | `80a66690_FUN_80a66690.c` |
| `0x80A66980` | `fn_80a66980` | Allocate/construct the `0x528`-byte family actor | translated | high | `80a66980_FUN_80a66980.c` |
| `0x80A66A70` | `fn_80a66a70` | In-place composite constructor with five holders | translated | high | `80a66a70_FUN_80a66a70.c` |
| `0x80A66C00` | `fn_80a66c00` | Dispose callback: `0x802B9280` when `mode > 0` | translated | high | `80a66c00_FUN_80a66c00.c` |
| `0x80A66C40` | `fn_80a66c40` | Construct one embedded animation holder | translated | high | `80a66c40_FUN_80a66c40.c` |

## Deferred neighbours

### Inside the touched ranges

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A61410` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A61440` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A61E10` | 64 B | Export is `halt_baddata()` after the partial `+0x736` update |
| `0x80A628F0` | 292 B | GQR/`ldexpf` artefacts and a removed unreachable block |
| `0x80A62A30` | 312 B | GQR/`ldexpf` artefacts and a removed unreachable block |
| `0x80A635D0` | 488 B | GQR/`ldexpf` artefacts in the angle step |
| `0x80A637E0` | 236 B | GQR/`ldexpf` artefacts in both angle branches |
| `0x80A638F0` | 284 B | GQR/`ldexpf` artefacts in both angle branches |
| `0x80A63A30` | 284 B | GQR/`ldexpf` artefacts in both angle branches |
| `0x80A63FF0` | 392 B | GQR/`ldexpf` artefacts in the ballistic step |
| `0x80A641F0` | 380 B | GQR/`ldexpf` artefacts in the ballistic step |
| `0x80A656A0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A656D0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |

### After the selected range (`gessoMini` remainder)

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A673D0` | 1,080 B | Large multi-branch handler, needs a dedicated pass |
| `0x80A67A60` | 120 B | Clean helper left for wave 23 |
| `0x80A67AE0` | 96 B | Clean helper left for wave 23 |
| `0x80A67B40` | 520 B | Large branch table, needs a dedicated pass |
| `0x80A67D50` | 280 B | Clean handler left for wave 23 |
| `0x80A67E70` | 196 B | Clean handler left for wave 23 |
| `0x80A67F40` | 552 B | Large handler left for wave 23 |

The `0x80A66A70` composite object's exact storage size is not printed by the
export, so its Rust doc states the requirement instead of inventing a number.

## Unknowns preserved in code

1. **Class label of the `gessoMini` block.** The model name and path are
   explicit in `fn_80a65850`, but no `StateID` string is referenced by the
   block. The label is `// inferred:` and every handler keeps its address
   name.
2. **Handler-to-state ownership.** The `gesso` records entered by the tail
   handlers (`0x80B720D0`, `0x80B72290`, `0x80B72310`) are BSS records built
   elsewhere, so the transitions stay by address.
3. **`0x8015F4F0` role.** The export shows a `s16` approach-and-clamp helper
   returning `1` on arrival; the name "angle approach" is inferred from the
   `+0x104`/`+0x728` call shape.
4. **Direction bytes `+0x5BE`/`+0x5BF`.** Their values are table indices 0/1;
   which axis each one drives is inferred from the `+0xE8`/`+0xEC` speed
   words they gate.
5. **Spawn index `+0x5C0`.** The byte is only used by `fn_80a65ce0` as the
   multiplier in `10.0 * (index + 1)`; its source is outside the block.
6. **Ownership of `0x80B49110` and `0x80B49418`.** Both are class tables
   installed by constructors with no model binder in their own body, so only
   the adjacent `gessoMini` block supports the family label.
7. **`0x80A00360`.** It is the destructor passed to the `+0x60C` container.
   Its body is already translated in this file, but it is a generic dispose
   callback, not a `gessoMini`-specific one.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` reports no error or warning in
`src/game/enemies.rs`, and `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave22-enemies.csv`.
