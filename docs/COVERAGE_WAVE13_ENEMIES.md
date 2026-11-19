# Coverage - wave 13 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the thirteenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 27 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   `daEnGuraRock_c`/`daEnGuraYuka_c` lift block
   (`0x80A6C810..0x80A6E680`) that sits between the two already-translated
   state tables.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   helper inside the run and the surrounding actor blocks that the next pass
   should take.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand or
  unresolved callee prevents a complete model; `stub` = address only.
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

| Metric | Before wave 13 | Wave 13 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 245 | 27 | 272 |
| Of those, `translated` | 245 | 22 | 267 |
| Of those, `partial` | 0 | 5 | 5 |
| Functions from the lift block | 0 | 27 | - |
| Constructors | 0 | 3 | - |
| Model/animation binders | 0 | 3 | - |
| State callbacks | 0 | 2 | - |
| Selected bytes | 0 | 4,236 | - |

The 27 new bodies cover 4,236 bytes (0.71 % of the 595,832 bytes that the gap
report listed as untranslated before this pass) and 27 of the 3,310 remaining
function entries (0.82 %). Confidence is **16 high** and **11 medium**. The
five 48-byte forwarders are `partial`: their visible body is complete, but the
hidden `r12` operand of `0x802DCBFC` cannot be modelled by
`platform::call3`. There are no `stub` rows.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 27 bodies were written before
   any ledger row was added.
2. **Took the lift block as one contiguous run.** The two wave-3 state tables
   bracket it: `register_gura_rock_states` at `0x80A6D180` registers `Wait`,
   `Fall` and `Still` with callback `0x80A6D400`, and
   `register_gura_yuka_states` at `0x80A6E240` registers `Wait` with callback
   `0x80A6E350`. The untranslated functions immediately before, between and
   after those tables form one actor family.
3. **Confirmed the owners from the asset strings.** `0x80A6C8C0` binds
   `g3d/lift_guraIwa.brres` with the model `lift_guraIwa`; `0x80A6D600`
   binds `g3d/lift_guraYuka.brres` with `lift_guraYuka`; `0x80A6E680` binds
   `g3d/guru.brres` with `guru` and the clip `rotate`. All three strings were
   read from `work/rels/d_enemiesNP.rel` at `address - 0x80A00000`.
4. **Confirmed the shared layout.** Both lift constructors initialise the
   same `+0x3D4` model holder, `+0x3F0` model handle, `+0x3F4`/`+0x400`
   animation holders and `+0x40C` auxiliary object. Both position/attitude
   handlers read `+0xAC..+0xB4` and `+0x100..+0x104`; `fn_80a6de90` commits a
   matrix to both holders.
5. **Excluded one neighbour on evidence grounds.** `0x80A6C9A0` (220 bytes)
   has a removed unreachable block and GQR/`ldexpf` artefacts in the export,
   so it is not presented as a complete body. `fn_80a6cf40` still calls it
   by address with the receiver inferred and records that gap.
6. **Cross-checked the data constants.** The `f32`/`f64` values referenced by
   the handlers were read from `work/rels/d_enemiesNP.rel`; the Rust keeps the
   original data addresses instead of baking the values in.

### Lift block (`0x80A6C810..0x80A6E680`, 27 functions)

`0x80A6C840` constructs the `0x488`-byte `daEnGuraRock_c`; `0x80A6C8C0`
binds its model and two clips. `0x80A6D580`/`0x80A6D600` do the same for the
`0x4A0`-byte `daEnGuraYuka_c`. The attitude half (`0x80A6CA80`,
`0x80A6CAB0`, `0x80A6CF40`, `0x80A6D050`, `0x80A6D6E0`, `0x80A6DA70`,
`0x80A6DAE0`, `0x80A6DB60`) derives tilt/roll, probes the ground, runs the
vertical spring and enters `Fall`/`Still`. `0x80A6D950` plus its recursive
helper `0x80A6D780` sample the neighbouring lift actors; `0x80A6E080` is the
main per-frame update. `0x80A6E1B0` releases a lift. The `guru` actor is the
small tail (`0x80A6E4D0`, `0x80A6E680`) plus two forwarders.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A6C810` | `fn_80a6c810` | Forward a state object's destructor through `0x802DCBFC` | partial | medium | `80a6c810_FUN_80a6c810.c` |
| `0x80A6C840` | `fn_80a6c840` | Allocate/construct a `0x488`-byte `daEnGuraRock_c` | translated | high | `80a6c840_FUN_80a6c840.c` |
| `0x80A6C8C0` | `fn_80a6c8c0` | Bind `lift_guraIwa` model and two clips | translated | high | `80a6c8c0_FUN_80a6c8c0.c` |
| `0x80A6CA80` | `fn_80a6ca80` | Derive the base tilt from the roll | translated | high | `80a6ca80_FUN_80a6ca80.c` |
| `0x80A6CAB0` | `fn_80a6cab0` | Ground probe and landing offset | translated | medium | `80a6cab0_FUN_80a6cab0.c` |
| `0x80A6CF40` | `fn_80a6cf40` | Clamp attitude bias and enter `Fall` | translated | medium | `80a6cf40_FUN_80a6cf40.c` |
| `0x80A6D050` | `fn_80a6d050` | `Wait` base update and ground-to-`Still` transition | translated | medium | `80a6d050_FUN_80a6d050.c` |
| `0x80A6D400` | `fn_80a6d400` | `daEnGuraRock_c` state callback (three records) | translated | high | `80a6d400_FUN_80a6d400.c` |
| `0x80A6D520` | `fn_80a6d520` | Second destructor forwarder | partial | medium | `80a6d520_FUN_80a6d520.c` |
| `0x80A6D550` | `fn_80a6d550` | Third destructor forwarder | partial | medium | `80a6d550_FUN_80a6d550.c` |
| `0x80A6D580` | `fn_80a6d580` | Allocate/construct a `0x4A0`-byte `daEnGuraYuka_c` | translated | high | `80a6d580_FUN_80a6d580.c` |
| `0x80A6D600` | `fn_80a6d600` | Bind `lift_guraYuka` model and two clips | translated | high | `80a6d600_FUN_80a6d600.c` |
| `0x80A6D6E0` | `fn_80a6d6e0` | Convert tilt to the clamped roll feedback | translated | high | `80a6d6e0_FUN_80a6d6e0.c` |
| `0x80A6D710` | `fn_80a6d710` | Test membership in a fixed word array | translated | high | `80a6d710_FUN_80a6d710.c` |
| `0x80A6D780` | `fn_80a6d780` | Recursive lift/neighbour sampler | translated | medium | `80a6d780_FUN_80a6d780.c` |
| `0x80A6D950` | `fn_80a6d950` | Sample the `+0x41C` list and update the attitude | translated | medium | `80a6d950_FUN_80a6d950.c` |
| `0x80A6DA70` | `fn_80a6da70` | Decay the wobble and step the tilt to zero | translated | high | `80a6da70_FUN_80a6da70.c` |
| `0x80A6DAE0` | `fn_80a6dae0` | Damp the spring speed and return the offset | translated | high | `80a6dae0_FUN_80a6dae0.c` |
| `0x80A6DB60` | `fn_80a6db60` | Integrate the vertical spring and write Y | translated | high | `80a6db60_FUN_80a6db60.c` |
| `0x80A6DE90` | `fn_80a6de90` | Build and commit both model matrices | translated | high | `80a6de90_FUN_80a6de90.c` |
| `0x80A6E080` | `fn_80a6e080` | Main lift frame update | translated | medium | `80a6e080_FUN_80a6e080.c` |
| `0x80A6E1B0` | `fn_80a6e1b0` | Release one lift actor and optionally free it | translated | high | `80a6e1b0_FUN_80a6e1b0.c` |
| `0x80A6E350` | `fn_80a6e350` | `daEnGuraYuka_c::StateID_Wait` callback | translated | high | `80a6e350_FUN_80a6e350.c` |
| `0x80A6E470` | `fn_80a6e470` | Fourth destructor forwarder | partial | medium | `80a6e470_FUN_80a6e470.c` |
| `0x80A6E4A0` | `fn_80a6e4a0` | Fifth destructor forwarder | partial | medium | `80a6e4a0_FUN_80a6e4a0.c` |
| `0x80A6E4D0` | `fn_80a6e4d0` | Allocate/construct a `0x5D8`-byte `guru` actor | translated | high | `80a6e4d0_FUN_80a6e4d0.c` |
| `0x80A6E680` | `fn_80a6e680` | Bind the `guru` model and `rotate` clip | translated | high | `80a6e680_FUN_80a6e680.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A6C9A0` | 220 B | Removed unreachable block plus GQR/`ldexpf` artefacts; called by address from `fn_80a6cf40` |

`0x80A6C9A0` is the only untranslated text entry inside the selected range.
The two state-object parameter blocks belong to the data section and are
already referenced by the wave-3 tables. No `StateID` table names a `guru`
state machine. Both gaps remain address-based or table-based; no behaviour is
invented for them.

## Unknowns preserved in code

1. **Lift gameplay meaning.** The archive names (`lift_guraIwa`,
   `lift_guraYuka`) and the height probe calls identify the actors as lifts.
   Which field is a rider slot, a chain link or a visual offset is inferred
   from the surrounding calls, not from symbols.
2. **`+0x41C` neighbour list.** `fn_80a6d950` walks a linked list whose links
   are `{next at +0xC, node at +4}`; `fn_80a6d780` then walks rider lists at
   `+0x460`/`+0x4CC`. The object graph is observed, the gameplay meaning is
   not.
3. **Five 48-byte forwarders.** `0x80A6C810`, `0x80A6D520`, `0x80A6D550`,
   `0x80A6E470` and `0x80A6E4A0` all call `0x802DCBFC`, which dispatches
   through a hidden `r12` this-adjust pointer. The Rust keeps the visible
   three arguments and marks the omission.
4. **Sound event `0x2C6`.** `fn_80a6e080` plays it when `+0xC8 <= -0.125`;
   the event id and trigger are observed, the sound's role is inferred.
5. **`guru` actor ownership.** `0x80A6E4D0` builds a `0x5D8`-byte actor with
   the `bakubaku` holder layout and `0x80A6E680` binds `g3d/guru.brres`. No
   `StateID` table names it, so the class label is inferred from the model
   string only.
6. **Dropped receivers.** Ghidra prints `0x8006CDE0` with no argument in
   `fn_80a6d050` and `0x80A6C9A0` with no argument in `fn_80a6cf40`. Both
   receive the only actor in scope; both call sites are marked.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. `cargo check` was run from the lane; it reports no error in
`src/game/enemies.rs`. This lane modified only `src/game/enemies.rs`, this
page and `ledger/agents/wave13-enemies.csv`.
