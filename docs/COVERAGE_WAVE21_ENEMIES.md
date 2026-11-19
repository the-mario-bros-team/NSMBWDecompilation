# Coverage - wave 21 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-first translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found:
   the `0x80A610C0..0x80A63410` block that builds and drives the `gesso`
   actor, including its `0x738`-byte constructor, its first six state-entry
   helpers and six per-frame handlers.
3. **What is still missing?** - *Deferred neighbours* names the seven
   entries inside the range that were left out, and the rest of the class
   block that wave 22 should take.

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
`main.dol` addresses (`0x8000xxxx`, `0x803xxxxx`, `0x8042xxxx`) are imported
absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 21 | Wave 21 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 448 | 30 | 478 |
| Of those, `translated` | 433 | 30 | 463 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| `daEnGesso_c` registration callbacks | - | 2 | - |
| `daEnGesso_c` constructor / model binder | - | 2 | - |
| Chain walker / name predicate | - | 2 | - |
| Constant producer | - | 1 | - |
| Animation, matrix and physics helpers | - | 11 | - |
| State-entry helpers | - | 6 | - |
| Per-frame handlers | - | 6 | - |
| Selected bytes | - | 4,656 | - |

The 30 new bodies cover 4,656 bytes (0.83 % of the 562,908 bytes that were
still untranslated before this pass; 4,656 / 562,908 = 0.83 %) and 30 of the
3,107 remaining function entries (0.97 %). Confidence is **22 high** and
**8 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 478 bodies, 463 `translated`, 15 `partial`, 3,077
function entries and 558,252 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added. The matrix call sequence in `fn_80a61c00` is
   the same one already used by the wave-11 `fn_80a00940` helper, but the
   `gesso` functions themselves had no Rust body.
2. **Started from the class identity.** `0x80A61470` allocates `0x738`
   bytes, installs vtable `0x80B48348` and initialises the same `+0x524`
   model holder, `+0x544` body holder and `+0x584` animation holder used by
   the wave-20 `daEnGabon_c` block. The binder `0x80A61830` names the model
   `gesso` (`0x80B48340`) and the path `g3d/gesso.brres` (`0x80B48330`).
   The `daEnGesso_c::StateID_*` strings begin at `0x80B488EC` in the same
   data section as the vtable and model names. The class label is therefore
   `// inferred:` from the model plus the adjacent state-name pool.
3. **Took thirty entries from the head of the block.** The range
   `0x80A610C0..0x80A63410` contains 37 function entries. Thirty were
   translated. Seven were left: five have damaged control flow or a hidden
   ABI operand, and two clean entries (`0x80A61180`, `0x80A61D60`) are held
   for wave 22 so this pass stays inside the 30-body budget.
4. **Kept the state machine's targets by address.** The handlers call the
   class vtable slot `+0xD4` with the BSS records at `0x80B71ED0`,
   `0x80B71F10`, `0x80B71F90`, `0x80B71FD0`, `0x80B72010`, `0x80B72050`
   and `0x80B7238C`. Those records are built at run time, so the state names
   cannot be matched one-to-one from this export; the Rust keeps the
   descriptor addresses and says so.
5. **Excluded every export with damaged control flow.** `0x80A61E10` ends in
   `halt_baddata()` after a partial `+0x736` increment. `0x80A628F0` and
   `0x80A62A30` contain GQR/`ldexpf` artefacts and an export-removed
   unreachable block in the middle of their arithmetic. None were guessed.
   The two `0x802DCBFC` forwarders at `0x80A61410` and `0x80A61440` were
   deferred for the same hidden `r12` operand as the earlier waves.
6. **Verified the shared helper ABIs against both modules.** `0x80064740`
   prints as `void` but every caller consumes `r3` (the nearest participant
   returned by its tail query `0x80064790`). `0x80168080` returns `f1` but
   prints as `void` because of an unrecovered jump table. Both are modelled
   from the caller shape and marked `// inferred:` where relevant.
7. **Read every numeric constant from the REL image.** `0x80B2D2C0 = 0.06`,
   `0x80B2D2C4 = 0.125`, `0x80B2D2D4 = 0.5`, `0x80B2D3EC = 4096.0`,
   `0x80B2D438 = 16.0`, `0x80B2D448 = 20.0`, `0x80B2D44C = 100.0` and
   `0x80B2D450 = 6.0` were read as big-endian `f32` values. The Rust keeps
   the data addresses instead of baking the values in.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A610C0` | `fn_80a610c0` | `daEnGesso_c` registration callback, first variant | translated | high | `80a610c0_FUN_80a610c0.c` |
| `0x80A61120` | `fn_80a61120` | `daEnGesso_c` registration callback, second variant | translated | high | `80a61120_FUN_80a61120.c` |
| `0x80A61270` | `fn_80a61270` | Five-link `+0x30` chain walker | translated | medium | `80a61270_FUN_80a61270.c` |
| `0x80A61350` | `fn_80a61350` | Compare the state-name suffix after the last `:` | translated | medium | `80a61350_FUN_80a61350.c` |
| `0x80A61470` | `alloc_gesso_actor` | Allocate/construct the `0x738`-byte `gesso` actor | translated | high | `80a61470_FUN_80a61470.c` |
| `0x80A61540` | `fn_80a61540` | Constant producer `0x80377A04` | translated | high | `80a61540_FUN_80a61540.c` |
| `0x80A61830` | `fn_80a61830` | Load `gesso` and bind the body/break clips | translated | high | `80a61830_FUN_80a61830.c` |
| `0x80A61A40` | `fn_80a61a40` | Run class slot `+0x294` and return 1 | translated | medium | `80a61a40_FUN_80a61a40.c` |
| `0x80A61A80` | `fn_80a61a80` | Bind an indexed break clip and two blend rates | translated | high | `80a61a80_FUN_80a61a80.c` |
| `0x80A61B20` | `fn_80a61b20` | Advance the `+0x737` timer and the `+0x102` angle | translated | high | `80a61b20_FUN_80a61b20.c` |
| `0x80A61B60` | `fn_80a61b60` | Seed the `+0x737` timer from the input query | translated | medium | `80a61b60_FUN_80a61b60.c` |
| `0x80A61BB0` | `fn_80a61bb0` | Snapshot clip matrix 0 into `+0x710` | translated | high | `80a61bb0_FUN_80a61bb0.c` |
| `0x80A61C00` | `fn_80a61c00` | Rebuild the matrix from position, yaw and three angles | translated | high | `80a61c00_FUN_80a61c00.c` |
| `0x80A61D90` | `fn_80a61d90` | Convert the height difference into the `+0x704` angle words | translated | medium | `80a61d90_FUN_80a61d90.c` |
| `0x80A61E60` | `fn_80a61e60` | Damp the y speed towards its target | translated | high | `80a61e60_FUN_80a61e60.c` |
| `0x80A61EB0` | `fn_80a61eb0` | Test whether a participant is `48.0` above the actor | translated | medium | `80a61eb0_FUN_80a61eb0.c` |
| `0x80A61F20` | `fn_80a61f20` | Same test with the `8.0` tolerance | translated | medium | `80a61f20_FUN_80a61f20.c` |
| `0x80A61F90` | `fn_80a61f90` | Select the side/variant from the two-bit phase counter | translated | medium | `80a61f90_FUN_80a61f90.c` |
| `0x80A62750` | `fn_80a62750` | State entry A: bind clip 2 and clear the motion words | translated | high | `80a62750_FUN_80a62750.c` |
| `0x80A627F0` | `fn_80a627f0` | Frame A: frame-100 turn rate and state change | translated | high | `80a627f0_FUN_80a627f0.c` |
| `0x80A62B70` | `fn_80a62b70` | State entry B: bind clip 5 and set the turn speeds | translated | high | `80a62b70_FUN_80a62b70.c` |
| `0x80A62C10` | `fn_80a62c10` | Frame B: speed thresholds and state choice | translated | high | `80a62c10_FUN_80a62c10.c` |
| `0x80A62D70` | `fn_80a62d70` | State entry C: bind clip 6 and set the downward speed | translated | high | `80a62d70_FUN_80a62d70.c` |
| `0x80A62E20` | `fn_80a62e20` | Frame C: `+0x733` timer and landing branch | translated | high | `80a62e20_FUN_80a62e20.c` |
| `0x80A62F80` | `fn_80a62f80` | Frame D: clip and `+0x2A0` predicate | translated | high | `80a62f80_FUN_80a62f80.c` |
| `0x80A630D0` | `fn_80a630d0` | Frame E: target-height and clip tests | translated | high | `80a630d0_FUN_80a630d0.c` |
| `0x80A631E0` | `fn_80a631e0` | State entry D: bind clip 0 and seed `+0x733` | translated | high | `80a631e0_FUN_80a631e0.c` |
| `0x80A63270` | `fn_80a63270` | Frame F: two-phase `+0x360` timer | translated | high | `80a63270_FUN_80a63270.c` |
| `0x80A633A0` | `fn_80a633a0` | State entry E: clip 6 with a loiter timer | translated | high | `80a633a0_FUN_80a633a0.c` |
| `0x80A63410` | `fn_80a63410` | State entry F: reset the three scale words | translated | high | `80a63410_FUN_80a63410.c` |

## Deferred neighbours

### Inside the selected range (7)

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A61180` | 12 B | Clean constant producer (`0x80357BB4`), held for wave 22 |
| `0x80A61410` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A61440` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A61D60` | 48 B | Clean two-store height helper, held for wave 22 |
| `0x80A61E10` | 64 B | Export is `halt_baddata()` after the partial `+0x736` update |
| `0x80A628F0` | 292 B | GQR/`ldexpf` artefacts and a removed unreachable block |
| `0x80A62A30` | 312 B | GQR/`ldexpf` artefacts and a removed unreachable block |

### After the selected range (the rest of `daEnGesso_c`)

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A63480` | 216 B | Clean frame handler left for wave 22 |
| `0x80A635D0` | 488 B | GQR/`ldexpf` artefacts in the angle step |
| `0x80A637E0` | 236 B | GQR/`ldexpf` artefacts in both angle branches |
| `0x80A638F0` | 284 B | GQR/`ldexpf` artefacts in both angle branches |
| `0x80A63A30` | 284 B | GQR/`ldexpf` artefacts in both angle branches |
| `0x80A63B70` | 116 B | Clean frame handler left for wave 22 |
| `0x80A63C10` | 172 B | Clean frame handler left for wave 22 |
| `0x80A63CE0` | 172 B | Clean frame handler left for wave 22 |
| `0x80A63DD0` | 112 B | Clean frame handler left for wave 22 |
| `0x80A63E60` | 116 B | Clean frame handler left for wave 22 |
| `0x80A63EE0` | 172 B | Clean state entry left for wave 22 |
| `0x80A63FF0` | 392 B | GQR/`ldexpf` artefacts in the ballistic step |
| `0x80A64180` | 96 B | Clean state entry left for wave 22 |
| `0x80A641F0` | 380 B | GQR/`ldexpf` artefacts in the ballistic step |

The `gessoMini` child block begins at `0x80A65350` (registration callback,
constant producer, binder and its own handlers) and is untouched.

## Unknowns preserved in code

1. **Class label of the `0x738` object.** The vtable `0x80B48348`, the
   `gesso` model names and the `daEnGesso_c::StateID_*` pool agree, but no
   `StateID` string is referenced by this block's code. The name
   `alloc_gesso_actor` is descriptive only and documented as inferred.
2. **Handler-to-state ownership.** The descriptors at
   `0x80B71ED0..0x80B7238C` are BSS records built elsewhere. The handlers
   are named `fn_<address>` and the transition targets stay by address.
3. **`0x80064740` return type.** The `main.dol` export declares the helper
   `void`, but `FUN_80064790` tail-calls the spatial query and every caller
   reads `r3`. The Rust models it as returning a participant pointer.
4. **`0x80168080` return type.** Ghidra prints `void` because of an
   unrecovered jump table, but three callers consume `f1` as the clip frame.
5. **`0x800B2EE0(2)`.** The phase-3 branch of `fn_80a61f90` calls this
   unknown helper; its result is returned as the side/variant value.
6. **Input masks at `0x8042A078`.** Used only by the deferred
   `0x80A63480` handler, so no mask constants were added in this pass.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` reports no warning or error in
`src/game/enemies.rs`, and `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave21-enemies.csv`.
