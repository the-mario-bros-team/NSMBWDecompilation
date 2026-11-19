# Coverage - wave 10 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the tenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the three blocks were
   found (the deferred penguin handlers, the `block_snake_ice` rail actor and
   two `daEnIcicle_c` helpers) and why each is a complete control-flow unit.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   wrappers and the local helpers that remain address-based calls.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` =
  address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  class ownership is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry point
is `0x80A000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8000xxxx`, `0x8032xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 10 | Wave 10 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Translated functions in `src/game/enemies.rs` | 162 | 30 | 192 |
| Untranslated penguin functions in `0x80B00C70..0x80B02860` | 15 | 15 | 0 |
| Untranslated `block_snake_ice` functions in `0x80B03ED0..0x80B04880` | 13 | 13 | 0 |
| New `daEnIcicle_c` helpers | 0 | 2 | - |
| Selected bytes | 0 | 6,704 | - |

The 30 new bodies cover 6,704 bytes (1.10 % of the 609,548 bytes that the gap
report listed as untranslated before this pass) and 30 of the 3,393 remaining
function entries (0.88 %). Confidence is **27 high** and **3 medium**; there
are no `partial` or `stub` rows.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Finished the penguin state machine.** Wave 9 stopped at
   `0x80B01350` and explicitly deferred `0x80B00C70`, `0x80B012E0`,
   `0x80B017A0`, `0x80B01DB0`, `0x80B01E70`, `0x80B01F20`, `0x80B01F80`.
   The run `0x80B01DB0..0x80B02860` is contiguous in `functions.tsv`, so the
   whole gap between those handlers was taken; no function in the run is left
   as a stub.
3. **Took the `block_snake_ice` actor from its asset strings.** The binder at
   `0x80B03ED0` loads `block_snake_ice` and `g3d/block_snake_ice.brres`
   (`0x80B66468`, `0x80B6644C`). The surrounding run
   `0x80B03ED0..0x80B04880` is that actor's model holder, follow controller,
   rail step, contact response and frame dispatcher.
4. **Added two `daEnIcicle_c` helpers.** `0x80A7F570` is the per-state callback
   installed by the already-translated `daEnIcicle_c` state table at
   `0x80A7EF40`; `0x80A7F930` is a model binder whose asset strings
   (`kuribo_iga`, `g3d/kuribo_iga.brres`, `wakeUp`) make its role explicit.
5. **Checked two disputed call sites against the instruction stream.** The
   `0x8007FA40` call in `0x80B04270` has exactly four float and eight integer
   arguments (Ghidra prints three trailing dead words), and the prologue thunk
   in `0x80B017A0` hides `r3` = penguin, `r4` = other participant. Both facts
   are recorded in the function docs.

### Penguin handlers (`0x80B00C70..0x80B02860`, 15 functions)

The block now contains the ground probe `0x80B00C70`, the sweat effect helper
`0x80B012E0`, the collision responder `0x80B017A0`, and the state handlers
`0x80B01DB0`, `0x80B01E70`, `0x80B01F20`, `0x80B01F80`, `0x80B02040`,
`0x80B023A0`, `0x80B02430`, `0x80B025C0`, `0x80B02630`, `0x80B026C0`,
`0x80B027D0` and `0x80B02860`. `0x80B02040` is the main frame handler and
calls the other new helpers.

### `block_snake_ice` rail actor (`0x80B03ED0..0x80B04880`, 13 functions)

`0x80B03ED0` builds the model and the `ridden` chain; `0x80B04060`,
`0x80B040D0`, `0x80B04120` and `0x80B041A0` reset, rebind and place the model
holders. `0x80B04270`/`0x80B04340` install and remove the follow controller.
`0x80B043F0` initialises the direction table at `0x80B78438` and advances along
the byte rail; `0x80B04680` probes the next segment, `0x80B04740` commits it,
`0x80B04580` handles a player contact and `0x80B04880` is the frame dispatcher.

### `daEnIcicle_c` helpers (2 functions)

`0x80A7F570` is the shared `fState` callback (`0x8015F800`/`0x802B9280`).
`0x80A7F930` binds the `kuribo_iga` model and four clips. Their state table and
constructor were already translated as `register_icicle_states` and
`alloc_icicle_actor`.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80B00C70` | `fn_80b00c70` | Four-step ground probe ahead of the penguin | translated | medium | `80b00c70_FUN_80b00c70.c` |
| `0x80B012E0` | `fn_80b012e0` | Sweat effect from animation matrix 0 | translated | high | `80b012e0_FUN_80b012e0.c` |
| `0x80B017A0` | `fn_80b017a0` | Contact response: state test, turn and dive commit | translated | high | `80b017a0_FUN_80b017a0.c` |
| `0x80B01DB0` | `fn_80b01db0` | Edge test and attack-entry switch | translated | high | `80b01db0_FUN_80b01db0.c` |
| `0x80B01E70` | `fn_80b01e70` | Dive attack setup | translated | high | `80b01e70_FUN_80b01e70.c` |
| `0x80B01F20` | `fn_80b01f20` | Dive animation advance and timer | translated | high | `80b01f20_FUN_80b01f20.c` |
| `0x80B01F80` | `fn_80b01f80` | Dive entry without the vertical kick | translated | high | `80b01f80_FUN_80b01f80.c` |
| `0x80B02040` | `fn_80b02040` | Main walking/ice frame handler | translated | medium | `80b02040_FUN_80b02040.c` |
| `0x80B023A0` | `fn_80b023a0` | Ice-ball state: animation 2 and effect release | translated | high | `80b023a0_FUN_80b023a0.c` |
| `0x80B02430` | `fn_80b02430` | Sliding/approach per-frame handler | translated | high | `80b02430_FUN_80b02430.c` |
| `0x80B025C0` | `fn_80b025c0` | Animation 1 plus facing pitch | translated | high | `80b025c0_FUN_80b025c0.c` |
| `0x80B02630` | `fn_80b02630` | Launch state: animation 5 and facing speed | translated | high | `80b02630_FUN_80b02630.c` |
| `0x80B026C0` | `fn_80b026c0` | Airborne handler with trail and state switch | translated | high | `80b026c0_FUN_80b026c0.c` |
| `0x80B027D0` | `fn_80b027d0` | Landing state: animation 5 and cooldown reset | translated | high | `80b027d0_FUN_80b027d0.c` |
| `0x80B02860` | `fn_80b02860` | Walking/turning per-frame handler | translated | high | `80b02860_FUN_80b02860.c` |
| `0x80A7F570` | `fn_80a7f570` | `daEnIcicle_c` common state callback | translated | high | `80a7f570_FUN_80a7f570.c` |
| `0x80A7F930` | `fn_80a7f930` | `kuribo_iga` model and clip binder | translated | high | `80a7f930_FUN_80a7f930.c` |
| `0x80B03ED0` | `fn_80b03ed0` | `block_snake_ice` model and `ridden` binder | translated | high | `80b03ed0_FUN_80b03ed0.c` |
| `0x80B04060` | `fn_80b04060` | Reset three model holders (`+0x10`) | translated | high | `80b04060_FUN_80b04060.c` |
| `0x80B040D0` | `fn_80b040d0` | Release two clip holders (`+0x14`) | translated | high | `80b040d0_FUN_80b040d0.c` |
| `0x80B04120` | `fn_80b04120` | Rebind the body clip on `+0x48` | translated | medium | `80b04120_FUN_80b04120.c` |
| `0x80B041A0` | `fn_80b041a0` | Place the model at position plus offset | translated | high | `80b041a0_FUN_80b041a0.c` |
| `0x80B04270` | `fn_80b04270` | Install the follow controller | translated | high | `80b04270_FUN_80b04270.c` |
| `0x80B04340` | `fn_80b04340` | Remove the follow controller | translated | high | `80b04340_FUN_80b04340.c` |
| `0x80B043C0` | `fn_80b043c0` | Clamp the vertical step | translated | high | `80b043c0_FUN_80b043c0.c` |
| `0x80B043F0` | `fn_80b043f0` | Initialise the direction table and step the rail | translated | high | `80b043f0_FUN_80b043f0.c` |
| `0x80B04580` | `fn_80b04580` | Player contact and state switch | translated | high | `80b04580_FUN_80b04580.c` |
| `0x80B04680` | `fn_80b04680` | Probe the next rail segment | translated | high | `80b04680_FUN_80b04680.c` |
| `0x80B04740` | `fn_80b04740` | Commit the next rail position | translated | high | `80b04740_FUN_80b04740.c` |
| `0x80B04880` | `fn_80b04880` | Rail actor frame dispatcher | translated | high | `80b04880_FUN_80b04880.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A7F690` | 48 B | State-vtable wrapper: tail-calls `0x802DCBFC` |
| `0x80A7F6C0` | 48 B | State-vtable wrapper: tail-calls `0x802DCBFC` |
| `0x80B04C90` | 112 B | Rail helper called by `fn_80b04880`; not yet translated |
| `0x80B04E10` | 124 B | Rail helper called by `fn_80b04880`; not yet translated |
| `0x80B055D0` | 68 B | Rail helper called by `fn_80b04880`; not yet translated |
| `0x80B05670` | 160 B | Rail retire test called by `fn_80b04880`; not yet translated |

These remain address-based calls in the Rust; none is presented as a new
symbol. `functions.tsv` contains no function entries between `0x80B029D8`
(end of `0x80B02860`) and `0x80B03780`, so no penguin handler was skipped in
that gap.

## Unknowns preserved in code

1. **Class labels.** The asset strings prove the models but the export carries
   no class symbol. The penguin handlers keep `fn_<address>` names; the
   `block_snake_ice` block is named from its asset string only.
2. **State-name objects.** The addresses `0x80B782C0`, `0x80B78300`,
   `0x80B78280`, `0x80B78498` and `0x80B784D8` are state-name objects in the
   data section. Their `StateID` text is outside the exported C, so they stay
   addresses.
3. **Flag meanings in `fn_80b02040`.** `+0x10B7`, `+0x10B8`, `+0x10B9`,
   `+0x10BA`, `+0x10BB` and `+0x10BD` are named from their use in this block
   and are marked `// inferred:`-style in the doc table; no external symbol
   confirms them.
4. **`fn_80b04120` clip name.** The `0x8023A190` call shows only one argument,
   unlike the same helper in `fn_80b03ed0`. The missing name pointer is marked
   `// unknown:` in the body.
5. **Rail table meaning.** `0x80B78438` (initialised by `fn_80b043f0`) and
   `0x80B78460` (read by `fn_80b04680`/`fn_80b04740`) are five-entry `(x, y)`
   direction tables. Their per-entry meaning (compass directions in actor
   space) is inferred from the arithmetic.
6. **`0x80070BA0` ABI.** The paired-single noise in `0x80B00C70` hides whether
   the probe takes `f32` or `f64`; the Rust uses the same adapter as
   `fn_80b00be0` and marks the function `medium`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` succeeds and
`node tools/doc-lint.mjs src` reports 0 violations across 68 files. This lane
modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave10-enemies.csv`.
