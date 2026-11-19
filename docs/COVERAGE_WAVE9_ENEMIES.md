# Coverage - wave 9 enemy handlers (`d_enemiesNP`, penguin2 block)

## How to read this file

This page records the ninth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the block was found
   from the `penguin2` model strings and why it forms a complete gameplay
   slice (frame callbacks, model, hitbox, effects and state setup).
3. **What is still missing?** - *Deferred neighbours* names the GQR-mangled
   function and the adjacent handlers left for a later pass; *Unknowns
   preserved* lists the facts that stay address-based.

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

| Metric | Before wave 9 | Wave 9 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Translated functions in `src/game/enemies.rs` | 132 | 30 | 162 |
| Functions in this block (`0x80B00050..0x80B01350`) | 0 | 30 | 30 |
| Actor model blocks identified by strings | - | 1 | - |
| Selected bytes | 0 | 4,620 | - |

The 30 new bodies cover 4,620 bytes (0.75 % of the 614,168 bytes that the gap
report listed as untranslated before this pass) and 30 of the 3,423 remaining
function entries (0.88 %). Confidence is **24 high** and **6 medium**; there
are no `partial` or `stub` rows.

## Selection

This pass was evidence-first and cluster-first:

1. **Started from `functions.tsv` and the existing `enemies.rs`.** Every
   address already translated in waves 1-7 was removed from the candidate
   set before selection.
2. **Found the class from its asset strings.** The model setup at
   `0x80B002D0` loads `penguin2`, `penguin` and `g3d/penguin2.brres` from the
   REL data section. No other function in the export references those
   strings, so the surrounding `0x80B00050..0x80B01F80` run is one actor's
   implementation.
3. **Took the complete clean sub-run.** `0x80B00050..0x80B01350` contains the
   frame callbacks, model builder, animation binder, transform composer,
   hitbox installer, hit resolver, effect lifetime helpers, list queries and
   the state initialiser. The run ends at the state initialiser so that every
   body's local callees are translated in the same pass.
4. **Rejected GQR bodies.** `0x80B00C70` contains paired-single quantisation
   noise in the Ghidra output. It is not translated and remains an address
   callee.

### Frame callbacks and ice sub-object (`0x80B00050..0x80B00280`)

The ice sub-object at `+0x5C4` has a small four-value state word at `+0x8E0`
and a state byte at `+0x8E3`. `fn_80b00050` enters state 1;
`fn_80b00070` and `fn_80b00090` advance states 1/2. The three callbacks
`fn_80b001d0`, `fn_80b00180` and `fn_80b00280` are the ones installed by
`fn_80b00f20`; they first try the attack response `fn_80b000c0` and otherwise
fall back to the `0x69` event handler `fn_80b00140`.

### Model and hitbox (`0x80B002D0..0x80B00880`)

`fn_80b002d0` builds the `penguin` model and the `+0x548`/`+0x588` animation
holders. `fn_80b003c0` selects a clip from the table at `0x80B318CC`.
`fn_80b00460` composes the world transform from five matrices.
`fn_80b005e0`, `fn_80b006f0`, `fn_80b00730` and `fn_80b00880` install and
resolve the nine-word attack block at `+0x414..+0x432`.

### Effects, probes and state setup (`0x80B009E0..0x80B01350`)

`fn_80b009e0` creates effect `199` and stores its handle at `+0x5C0`;
`fn_80b00a50` copies animation matrix 3 onto that effect; `fn_80b00b10` and
`fn_80b00b90` release it. `fn_80b00be0` probes the ground ahead,
`fn_80b00d70` aligns pitch, `fn_80b00dd0`/`fn_80b00e60` walk the two actor
lists, and `fn_80b00ea0` refreshes the hitbox when a player is present.
`fn_80b00f20` installs the frame callbacks; `fn_80b00fc0` emits the ice
trail; `fn_80b010b0` chooses the facing pitch; `fn_80b01290` enables the
renderer and switches state; `fn_80b01350` resets physics, installs the
model and selects the first state.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80B00050` | `fn_80b00050` | Ice sub-object: enter state 1, set `0x186` | translated | high | `80b00050_FUN_80b00050.c` |
| `0x80B00070` | `fn_80b00070` | Ice sub-object: state 1/2 -> 4 | translated | high | `80b00070_FUN_80b00070.c` |
| `0x80B00090` | `fn_80b00090` | Ice sub-object: state 1/2 -> 5, timer 100 | translated | high | `80b00090_FUN_80b00090.c` |
| `0x80B000C0` | `fn_80b000c0` | Attack response: mode 1 + `+0x3D4` predicate | translated | high | `80b000c0_FUN_80b000c0.c` |
| `0x80B00140` | `fn_80b00140` | Event `0x69` handler for the ice sub-object | translated | high | `80b00140_FUN_80b00140.c` |
| `0x80B00180` | `fn_80b00180` | Frame callback: attack or event | translated | medium | `80b00180_FUN_80b00180.c` |
| `0x80B001D0` | `fn_80b001d0` | Frame callback: body hit, reverse speed | translated | medium | `80b001d0_FUN_80b001d0.c` |
| `0x80B00280` | `fn_80b00280` | Frame callback: attack or event | translated | medium | `80b00280_FUN_80b00280.c` |
| `0x80B002D0` | `fn_80b002d0` | Build `penguin` model and animation holders | translated | high | `80b002d0_FUN_80b002d0.c` |
| `0x80B003C0` | `fn_80b003c0` | Apply one attack animation | translated | high | `80b003c0_FUN_80b003c0.c` |
| `0x80B00460` | `fn_80b00460` | Compose position and four rotations | translated | high | `80b00460_FUN_80b00460.c` |
| `0x80B005E0` | `fn_80b005e0` | Install default attack block | translated | high | `80b005e0_FUN_80b005e0.c` |
| `0x80B006F0` | `fn_80b006f0` | Virtual `+0x15C` refresh + hitbox | translated | high | `80b006f0_FUN_80b006f0.c` |
| `0x80B00730` | `fn_80b00730` | Hit-type-dependent attack block | translated | high | `80b00730_FUN_80b00730.c` |
| `0x80B00880` | `fn_80b00880` | Resolve hit against a victim | translated | medium | `80b00880_FUN_80b00880.c` |
| `0x80B009E0` | `fn_80b009e0` | Spawn effect `199`, store handle | translated | high | `80b009e0_FUN_80b009e0.c` |
| `0x80B00A50` | `fn_80b00a50` | Copy animation matrix 3 to effect | translated | high | `80b00a50_FUN_80b00a50.c` |
| `0x80B00B10` | `fn_80b00b10` | Set effect facing and clear handle | translated | high | `80b00b10_FUN_80b00b10.c` |
| `0x80B00B90` | `fn_80b00b90` | Retire effect handle | translated | medium | `80b00b90_FUN_80b00b90.c` |
| `0x80B00BE0` | `fn_80b00be0` | Ground probe with facing offset | translated | high | `80b00be0_FUN_80b00be0.c` |
| `0x80B00D70` | `fn_80b00d70` | Align or clear the actor pitch | translated | high | `80b00d70_FUN_80b00d70.c` |
| `0x80B00DD0` | `fn_80b00dd0` | Walk `+0xEBC` list for player flags | translated | high | `80b00dd0_FUN_80b00dd0.c` |
| `0x80B00E60` | `fn_80b00e60` | Walk `+0xEB8` list for a live player | translated | high | `80b00e60_FUN_80b00e60.c` |
| `0x80B00EA0` | `fn_80b00ea0` | Refresh hitbox when a player is nearby | translated | high | `80b00ea0_FUN_80b00ea0.c` |
| `0x80B00EF0` | `fn_80b00ef0` | Actor position + `+0xB0` offset | translated | high | `80b00ef0_FUN_80b00ef0.c` |
| `0x80B00F20` | `fn_80b00f20` | Install three frame callbacks | translated | high | `80b00f20_FUN_80b00f20.c` |
| `0x80B00FC0` | `fn_80b00fc0` | Ice trail effects `0x26E`/`0x26F` | translated | high | `80b00fc0_FUN_80b00fc0.c` |
| `0x80B010B0` | `fn_80b010b0` | Facing pitch or dive pitch | translated | high | `80b010b0_FUN_80b010b0.c` |
| `0x80B01290` | `fn_80b01290` | Enable renderer, switch state | translated | high | `80b01290_FUN_80b01290.c` |
| `0x80B01350` | `fn_80b01350` | State initialiser: physics, model, spawn | translated | medium | `80b01350_FUN_80b01350.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80B00C70` | 244 B | GQR/paired-single mangling in the export |
| `0x80B012E0` | 112 B | sweat effect helper; depends on renderer state |
| `0x80B017A0` | 636 B | collision responder with two prologue-lost parameters |
| `0x80B01DB0` | 192 B | nearby-player state switch |
| `0x80B01E70` | 156 B | attack-state reset helper |
| `0x80B01F20` | 96 B | attack timer |
| `0x80B01F80` | 156 B | attack-state entry |

These remain address-based calls or simply untranslated; none is presented as
a new symbol.

## Unknowns preserved in code

1. **Class name.** The strings prove a penguin actor, but the export carries no
   class symbol. Every function keeps `fn_<address>` and the docs say so.
2. **Hitbox semantics.** The nine words at `+0x414..+0x432` are copied
   verbatim from engine data and tables. Only `+0x432` (active flag) and
   `+0x431` (type byte) are named; the other fields stay positional.
3. **Prologue-lost parameters.** Ghidra shows `0x802DCD70` as a returning
   function in `fn_80b005e0`, `fn_80b00730`, `fn_80b00880` and
   `fn_80b01350`-related handlers. The Rust omits the save block and restores
   the incoming actors from the call sites, marked `// inferred:`.
4. **Frame-callback arguments.** `fn_80b00180` and `fn_80b00280` call
   `fn_80b000c0` without printed arguments; the two parameters are inferred
   from the sibling callback and the callee body.
5. **Effect ids.** `199`, `0x26E` and `0x26F` are passed through by number;
   their visual meaning is outside this lane.
6. **Data constants.** Facing/speed constants are read from their synthetic
   data addresses rather than baked in, preserving the original indirection.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` succeeds and
`node tools/doc-lint.mjs src` reports 0 violations across 68 files. This lane
modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave9-enemies.csv`.
