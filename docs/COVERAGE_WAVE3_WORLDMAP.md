# Wave 3 WorldMap second pass

## How to read this file

This page records the twenty-five `main.dol` functions promoted into
`src/game/worldmap.rs` by the wave 3 second pass. They were selected from the
`0x80100000..0x80180000` export range and checked against
`ledger/functions.csv`, `ledger/agents/r80100_80140.csv` and
`ledger/agents/r80140_80180.csv`; none had a previous ledger row.

Each table row has:

- **address** - virtual address in `main.dol`;
- **name** - Rust symbol in `src/game/worldmap.rs`;
- **role** - what the function does in one sentence;
- **status** - `translated` means the recorded control flow is represented;
- **confidence** - `high` when every branch/store is explicit, `medium` when a
  callee argument or a field role is inferred;
- **evidence** - the Ghidra C file used for the translation.

The source module also keeps the original `# Original` block on every
function. `// observed:`, `// inferred:` and `// unknown:` comments mark the
boundary between evidence and naming guesses. The machine-readable copy of this
table is `ledger/agents/wave3-worldmap.csv` (module `game::worldmap`).

## What this pass adds

| cluster | functions | why it is one unit |
|---|---:|---|
| effect slot allocator/spawn | 6 | `0x80103220..0x80103580` completes the `0x55`-slot pool with allocation, slot spawn, name lookup and the two spawn wrappers |
| effect manager/motion | 4 | `0x80103B40..0x801040D0` covers the motion step, its weight wrapper, global manager teardown and slot release |
| Wm actor animation/state | 12 | `0x80142510`, `0x80143790..0x801472D0` covers the animation table selector, reset path, animation frame update, sound-id tables, table pointers, mode byte and action words |
| Wm controller/target | 3 | `0x8014F3D0..0x8014FD60` covers the target-state clear/update and facing reset |

Total: **25 functions, 3,420 bytes**.

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80103220` | `wm_effect_slot_spawn` | Refresh one effect slot and call its spawn method | translated | high | `work/ghidra/main-gb/export/80103220_FUN_80103220.c` |
| `0x80103310` | `wm_effect_spawn_transform` | Build a stack transform then spawn a named effect | translated | high | `work/ghidra/main-gb/export/80103310_FUN_80103310.c` |
| `0x80103390` | `wm_effect_spawn_named` | Validate an effect name and call the global spawner | translated | medium | `work/ghidra/main-gb/export/80103390_FUN_80103390.c` |
| `0x80103420` | `wm_effect_slot_alloc_default` | Allocate a slot with zeroed placement fields | translated | high | `work/ghidra/main-gb/export/80103420_FUN_80103420.c` |
| `0x80103440` | `wm_effect_slot_alloc` | Reserve a free `0x150`-byte slot and store its fields | translated | high | `work/ghidra/main-gb/export/80103440_FUN_80103440.c` |
| `0x80103580` | `wm_effect_name` | Map a `Wm_*` effect id to its string name | translated | high | `work/ghidra/main-gb/export/80103580_FUN_80103580.c` |
| `0x80103B40` | `wm_effect_motion_step` | Advance a damped/zoomed motion value | translated | medium | `work/ghidra/main-gb/export/80103b40_FUN_80103b40.c` |
| `0x80103D30` | `wm_effect_weight` | Forward a record's vector table to the weight scan | translated | medium | `work/ghidra/main-gb/export/80103d30_FUN_80103d30.c` |
| `0x80103F10` | `wm_effect_manager_destroy` | Release the four children and four shared objects | translated | high | `work/ghidra/main-gb/export/80103f10_FUN_80103f10.c` |
| `0x801040D0` | `wm_effect_slot_release` | Run the slot destructor and optionally free the base | translated | high | `work/ghidra/main-gb/export/801040d0_FUN_801040d0.c` |
| `0x80142510` | `wm_anim_table_entry` | Index the player animation table by player and kind | translated | high | `work/ghidra/main-gb/export/80142510_FUN_80142510.c` |
| `0x80143790` | `wm_actor_reset_effects` | Reset the actor's timer and effect fields | translated | high | `work/ghidra/main-gb/export/80143790_FUN_80143790.c` |
| `0x80143880` | `wm_countdown_byte` | Decrement a byte without underflow | translated | high | `work/ghidra/main-gb/export/80143880_FUN_80143880.c` |
| `0x80145CC0` | `wm_anim_frame_update` | Drive long-animation counters and shared audio | translated | medium | `work/ghidra/main-gb/export/80145cc0_FUN_80145cc0.c` |
| `0x80145E40` | `wm_set_flag_c1_if_active` | Set status flag `0xC1` for an active, grounded actor | translated | high | `work/ghidra/main-gb/export/80145e40_FUN_80145e40.c` |
| `0x80145E90` | `wm_set_sound_by_mode` | Choose the per-player voice/sound id by mode | translated | medium | `work/ghidra/main-gb/export/80145e90_FUN_80145e90.c` |
| `0x801466D0` | `wm_set_scale_constants` | Copy four scale/limit constants into the actor | translated | high | `work/ghidra/main-gb/export/801466d0_FUN_801466d0.c` |
| `0x80146700` | `wm_set_anim_table_pointers` | Point `+0x1060`/`+0x1064` at per-player tables | translated | high | `work/ghidra/main-gb/export/80146700_FUN_80146700.c` |
| `0x80146740` | `wm_set_motion_table` | Point `+0x1068` at the normal or special motion table | translated | high | `work/ghidra/main-gb/export/80146740_FUN_80146740.c` |
| `0x80146770` | `wm_get_motion_table` | Return the normal or special motion table | translated | medium | `work/ghidra/main-gb/export/80146770_FUN_80146770.c` |
| `0x80147250` | `wm_set_mode_byte` | Set `+0x153C`, forcing `1` when the vtable vetoes | translated | high | `work/ghidra/main-gb/export/80147250_FUN_80147250.c` |
| `0x801472D0` | `wm_set_action_and_sound` | Store two action words and trigger the sound state | translated | high | `work/ghidra/main-gb/export/801472d0_FUN_801472d0.c` |
| `0x8014F3D0` | `wm_clear_target_state` | Clear the target-state word | translated | high | `work/ghidra/main-gb/export/8014f3d0_FUN_8014f3d0.c` |
| `0x8014F540` | `wm_update_target_state` | Advance the target state and select the next action | translated | medium | `work/ghidra/main-gb/export/8014f540_FUN_8014f540.c` |
| `0x8014FD60` | `wm_reset_facing_and_notify` | Reset facing when idle and notify the scene manager | translated | high | `work/ghidra/main-gb/export/8014fd60_FUN_8014fd60.c` |

## Evidence notes

### Compiler save/restore helpers

`0x80103440` starts with `FUN_802DCD6C()` and ends with `FUN_802DCDB8()` in the
Ghidra listing. Those addresses are the EABI `_savegpr`/`_restgpr` helpers, not
game calls. The first preserves the incoming `r3`/`r4` pair, which Ghidra
exposes as an `undefined8`. The Rust signature therefore reads
`(tag, base, ...)`: `tag` is checked for the empty marker `0x2F`, and `base` is
the slot-table pointer passed to `fn_80103500`.

### Resolved constants

- `0x802F57C0..CC` - `0`, `20.0`, `15.0`, `22.0` (`main.elf`).
- `0x802F57D0..DC` - `1`, `1`, `1`, `0`.
- `0x802F5618` / `0x802F5660` - motion tables whose first floats are `-0.34`
  and `-0.09`.
- `0x8042D1CC..E0` - `-2.0`, `2.0`, `0.05`, `0.01`, `9999.0` and the
  `0x4330000080000000` integer-to-double bias used by `0x80103B40`.
- `0x80429070` (`r13 - 0x6610`) - empty string; confirmed in `main.elf`.

## Unknowns and inferences

- `wm_effect_spawn_named` (`0x80103390`) - Ghidra shows `FUN_80103580()` with
  no arguments. The `base`/`tag` forwarding is inferred from the wrapper
  `0x80103310` and from `0x80103220`, which passes `(base, slot+0x04)`.
- `wm_effect_weight` (`0x80103D30`) - Ghidra shows only one argument to
  `0x80106C70`; the second pointer `record + 0x1C` is inferred from the only
  caller, `0x80103B40`.
- `wm_get_motion_table` (`0x80146770`) - the actor parameter is not shown by
  Ghidra; it is inferred because `0x80056BD0` reads `actor + 0x1090`.
- `wm_update_target_state` (`0x8014F540`) - the first call's actor argument is
  dropped by Ghidra; it is inferred from the surrounding target-state code.
- `wm_anim_frame_update` (`0x80145CC0`) - virtual calls show no explicit
  `this`; the controller and actor pointers are inferred from the field layout.
- `wm_set_sound_by_mode` (`0x80145E90`) - the three three-word tables are
  lazily written; their exact voice meaning is unknown.
- `wm_effect_slot_spawn` / `wm_effect_slot_alloc` - the field roles at
  `+0x08`, `+0x0C`, `+0x10`, `+0x14`, `+0x18` are inferred from the producer
  (`0x80103440`) and consumer (`0x80103220`) only; no type metadata exists.
- `wm_effect_manager_destroy` (`0x80103F10`) - the manager type at
  `0x8042A27C` is unknown; only its child offsets and vtable slot `+0x08` are
  proven.

## Verification

- `cargo check` passes from the repository root (exit 0).
- A temporary crate containing only `src/platform.rs`, `src/game/worldmap.rs`
  and the `SDA_BASE` constant also passes `cargo check` (exit 0), confirming
  this lane's file is self-consistent.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml` or `src/platform.rs` change
  was made.
