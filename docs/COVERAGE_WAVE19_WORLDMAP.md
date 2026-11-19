# Coverage - wave 19 WorldMap spin, selection and transform helpers

## How to read this file

This page records the twenty-five `main.dol` functions added to
[`src/game/worldmap.rs`](../src/game/worldmap.rs) by the wave 19 pass. They were
selected from `0x80100000..0x80140000` after removing every address already
present in `ledger/functions.csv`, every address already translated in
`src/game/worldmap.rs`, and every entry smaller than 16 bytes.

- **address** is the entry virtual address from
  `work/ghidra/main-gb/export/functions.tsv`; it is not a file offset.
- **name** is the Rust symbol in `src/game/worldmap.rs`.
- **role** is the behaviour in one sentence.
- **status** is `translated` when the whole recorded control flow is
  represented. This pass has no `partial` or `stub` rows.
- **confidence** is `high` when every branch, call and store is explicit;
  `medium` when one argument, table stride or field role is inferred.
- **evidence** is the Ghidra C file used for the translation.

Every Rust body keeps its `# Original` block with the Ghidra name, address, byte
size and evidence path. The machine-readable copy of the table is
`ledger/agents/wave19-worldmap.csv` (module `game::worldmap`).

The range is a WorldMap lane: the `0x80106xxx` pair closes the transform-record
cluster, the `0x80129xxx..0x8012Dxxx` group are player action gates and target
selectors that read the controller mode word at `controller+0x154`, and the
`0x8013Dxxx..0x8013Exxx` group is the spin/landing state machine that the wave
12 effect helpers call. The spin code names its effects through the already
translated `Wm_mr_*` helpers; this pass covers the surrounding state logic.

## Scope and counts

| item | count |
|---|---:|
| functions in this pass | 25 |
| bytes in this pass | 2,648 |
| `high` confidence | 16 |
| `medium` confidence | 9 |
| `partial` | 0 |
| `stub` | 0 |
| new staging files | 0 |

## Selection

Three clusters were taken:

1. **Transform/renderer resets (`0x80106FC0`, `0x80107020`).** The first
   applies a 100-byte scene record and commits the inline transform at
   `object+0x1C`; the second restores the renderer light/channel state. They
   sit directly after wave 4's `0x80106E60` and use the same transform-record
   conventions.
2. **Player action gates and target selectors (`0x801298B0..0x8012DE20`, 15
   functions).** These read the controller mode, the spin timer, the status-bit
   words and the selection handle at `+0x2A78`, then resolve the selected
   object through `0x80162D00`. `0x8012D200` copies a controller transform into
   the actor's effect block.
3. **Spin/landing state (`0x80134AB0`, `0x8013D1E0..0x8013E470`, 8
   functions).** Entry animation, sub-object release, the four-player exit
   choice, per-player flag clearing, the idle reset gate, and the landing
   condition word. Wave 12's `wm_spinjump_effect` already called
   `0x8013D3B0`/`0x8013D8F0`/`0x8013D9F0`; they now resolve to bodies in the
   same module.

Functions with incomplete Ghidra records were deliberately skipped (for
example `0x80106620`, `0x801068B0`, `0x80106B40`, `0x8012FB10` and
`0x80132340`, all listed in the wave 12 and range coverage pages).

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80106FC0` | `wm_transform_scene_apply` | build and commit a 100-byte scene record | translated | medium | `work/ghidra/main-gb/export/80106fc0_FUN_80106fc0.c` |
| `0x80107020` | `wm_render_light_reset` | restore the default light/channel state | translated | medium | `work/ghidra/main-gb/export/80107020_FUN_80107020.c` |
| `0x801298B0` | `wm_action_allowed` | gate the spin action on controller mode `0x154` | translated | high | `work/ghidra/main-gb/export/801298b0_FUN_801298b0.c` |
| `0x80129910` | `wm_spin_timer_value` | return the signed spin timer after the gate | translated | medium | `work/ghidra/main-gb/export/80129910_FUN_80129910.c` |
| `0x80129950` | `wm_spin_timer_start` | seed the timer and force the half-spin | translated | high | `work/ghidra/main-gb/export/80129950_FUN_80129950.c` |
| `0x801299C0` | `wm_spin_timer_advance` | advance the 16-bit timer by `0x1400` | translated | high | `work/ghidra/main-gb/export/801299c0_FUN_801299c0.c` |
| `0x8012A0C0` | `wm_clear_action_bits` | cancel eleven landing/slide bits | translated | high | `work/ghidra/main-gb/export/8012a0c0_FUN_8012a0c0.c` |
| `0x8012A370` | `wm_clamp_slide_speed` | clamp `+0x10C` and cancel six bits | translated | medium | `work/ghidra/main-gb/export/8012a370_FUN_8012a370.c` |
| `0x8012A8D0` | `wm_clear_idle_bits` | cancel two idle bits and send event `3` | translated | high | `work/ghidra/main-gb/export/8012a8d0_FUN_8012a8d0.c` |
| `0x8012D0C0` | `wm_clear_sound_bits` | cancel two bits and stop the `+0x494` sub-object | translated | high | `work/ghidra/main-gb/export/8012d0c0_FUN_8012d0c0.c` |
| `0x8012D200` | `wm_copy_controller_transform` | copy the controller transform and submit it | translated | medium | `work/ghidra/main-gb/export/8012d200_FUN_8012d200.c` |
| `0x8012DBE0` | `wm_find_actor_state_one` | select the handle's actor when type `+0x38C` is `1` | translated | high | `work/ghidra/main-gb/export/8012dbe0_FUN_8012dbe0.c` |
| `0x8012DC20` | `wm_find_actor_id_19d` | select the handle's actor when id `0x19D` matches | translated | high | `work/ghidra/main-gb/export/8012dc20_FUN_8012dc20.c` |
| `0x8012DC60` | `wm_find_selected_actor` | select a type-`1`/state-`4` or id-`0x19D` actor | translated | high | `work/ghidra/main-gb/export/8012dc60_FUN_8012dc60.c` |
| `0x8012DCC0` | `wm_find_actor_id_19c_range` | select an actor with id `0x19C` or `0x19D` | translated | high | `work/ghidra/main-gb/export/8012dcc0_FUN_8012dcc0.c` |
| `0x8012DDB0` | `wm_action_finish_check` | report whether the current action still runs | translated | medium | `work/ghidra/main-gb/export/8012ddb0_FUN_8012ddb0.c` |
| `0x8012DE20` | `wm_scale_by_id_table` | scale `+0x27DC` by the id-selected table entry | translated | medium | `work/ghidra/main-gb/export/8012de20_FUN_8012de20.c` |
| `0x80134AB0` | `wm_clear_move_bits` | cancel three movement bits and run `+0x430` | translated | medium | `work/ghidra/main-gb/export/80134ab0_FUN_80134ab0.c` |
| `0x8013D1E0` | `wm_spin_enter` | request state `0x11A` and raise `+0x1A10` | translated | high | `work/ghidra/main-gb/export/8013d1e0_FUN_8013d1e0.c` |
| `0x8013D3B0` | `wm_spin_release` | release the `+0x18FC` sub-object and clear state | translated | high | `work/ghidra/main-gb/export/8013d3b0_FUN_8013d3b0.c` |
| `0x8013D670` | `wm_spin_exit_choose` | choose the spin-exit event via the four-player test | translated | high | `work/ghidra/main-gb/export/8013d670_FUN_8013d670.c` |
| `0x8013D8F0` | `wm_spin_clear_player_flags` | clear the four players' spin flags | translated | medium | `work/ghidra/main-gb/export/8013d8f0_FUN_8013d8f0.c` |
| `0x8013D9F0` | `wm_spin_reset_if_idle` | reset when the global player slot is unassigned | translated | high | `work/ghidra/main-gb/export/8013d9f0_FUN_8013d9f0.c` |
| `0x8013E380` | `wm_land_condition_set` | set `+0x2A80` from state `0xC` or slot `+0x3D4` | translated | high | `work/ghidra/main-gb/export/8013e380_FUN_8013e380.c` |
| `0x8013E470` | `wm_stop_two_states` | cancel states `0xBB` and `4` | translated | high | `work/ghidra/main-gb/export/8013e470_FUN_8013e470.c` |

## Data used by this pass

| address | width | meaning |
|---|---|---|
| `0x80424558` | object | shared renderer/scene object passed to `0x801C9940` |
| `0x80325364` | float[3] records | id-selected scale table; only the first float of each 12-byte record is read |
| `0x80354E50` | word[4] | player-presence words, one per controller slot |
| `0x80354E60` | word[4] | player-slot indices into `0x80354E90` |
| `0x80354E90` | word[4] | argument words selected by `0x80354E60` |
| `r13 - 0x59EC` | word | global player object (`0x80429C94`); `+0x84` is the assigned slot |
| `r13 - 0x5988` | byte | per-player update flag byte (`0x80429CF8`) |

The three `0x80354xxx` tables are runtime-initialised; `0x80325364` is zero in
the file image of `main.elf` and filled during start-up, so all four are read
with `platform::read_u32`/`read_f32` instead of being baked in.

Actor fields read or written by this pass:

| offset | width | meaning | confidence |
|---|---|---|---|
| `+0x100` | halfword | animation/state halfword, cleared by three helpers | observed |
| `+0x10C` | float | horizontal slide speed, clamped into `[-limit, limit]` | observed |
| `+0x494` | word | sub-object stopped by `wm_clear_sound_bits` | observed |
| `+0x5AC` | byte | sub-object active flag | observed |
| `+0xE0` (vtable) | word | slot used as the "busy" test in five helpers | observed |
| `+0xEA4` | byte block | sub-object that receives events `1`, `3`, `4` | observed |
| `+0x1090` | word | player/character variant index | inferred |
| `+0x1454` | word | action state (`0xC` selects the landing branch) | inferred |
| `+0x1554` | word | spin-exit gate | observed |
| `+0x18FC` | object | inline spin sub-object | observed |
| `+0x1A10` / `+0x1A14` | words | spin state words | observed |
| `+0x2534` | halfword | signed spin timer | observed |
| `+0x27DC` | float | scale multiplied by the id table | observed |
| `+0x2A64` | word | controller object; mode at `+0x154`, probe object at `+0x28` | observed |
| `+0x2A78` | word | target-selection handle consumed by `0x80162D00` | observed |
| `+0x2A80` | word | landing condition word | observed |
| `+0x38C` | byte | selected-actor type test (`1`) | observed |
| `+0x38D` | byte | signed player/controller index | observed |
| `+0x38E` | byte | per-player status flags (bit `1` cleared) | observed |

## Unknowns and inferences

1. **Roles are inferred from shape and adjacency.** No string names these
   functions. `wm_transform_scene_apply`, `wm_render_light_reset`,
   `wm_clear_move_bits` and the `wm_action_*`/`wm_find_*` names describe the
   call pattern; they do not claim a proven gameplay state.
2. **Dropped callee arguments.** Ghidra prints `FUN_80057650()` in
   `0x8012A370` and `FUN_801298B0()` in `0x80129910`; both callees declare
   `int param_1` and the actor is still in `r3`, so the Rust passes `actor`.
   Same convention for the virtual slot `+0x454` in `0x8012DDB0`, which is
   printed as `(param_1)` and is treated as `this`-only.
3. **A `void` callee whose result is used.** `0x8005F900` is declared `void` in
   the Ghidra C, but `0x8013D8F0` tests its `r3` result. The Rust keeps the
   word through `platform::call1`; this is the same convention as the wave 12
   ground probe.
4. **Table stride.** `0x80325364` is indexed as `(id & 0xFF) * 3` into a float
   array. Reading `main.elf` shows the first initialised record at `+0x20` as
   `-3.0, 3.0, 3.2`, which confirms 12-byte records; only the first float is
   used by `0x8012DE20`.
5. **Four-player tables.** `0x80354E50` is the same presence table used by
   `src/display.rs`; `0x80354E60`/`0x80354E90` are the index/argument pair
   already documented in `src/engine/globals.rs`. The spin-exit test treats a
   zero presence word or a zero argument word as "slot absent".
6. **`0x800D5790` third argument.** The callee is declared with two
   parameters, but `0x8012D200` passes `(controller, buffer, 7)`. The Rust
   keeps the call as printed and marks the translation `medium`.
7. **`0x80106FC0` record layout.** The 100-byte record is opaque: the body
   only shows the constructor, two resets and the commit call. The field
   layout belongs to the `0x802BF3D0..0x802BF650` helpers and is not asserted
   here.
8. **Inline vtable call in `0x8013D3B0`.** The sub-object lives at
   `actor+0x18FC`, so its vtable word is read from that address and slot
   `+0x18` is called with `this = actor+0x18FC`. This differs from the usual
   `+0x60` actor vtable and is kept literal.

## Verification

- `cargo check` exits 0; the only warnings are pre-existing ones in
  `src/runtime/*.rs`, outside this lane.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml` or `src/platform.rs` change
  was made by this pass, and no staging file was added.
- Every function cites its Ghidra address, byte size and evidence file in the
  `# Original` block.
- No ledger row was added for a function that does not have a Rust body.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---|---|---|
| 1 | `0x8013D400` | 216 B | Complements `wm_spin_enter`/`wm_spin_release`; calls `0x80145650` and uses the r2 zero constant. | `8013d400_FUN_8013d400.c` |
| 2 | `0x8013E400` | 104 B | Calls `0x8013E380` and clears the spin transform fields; completes the landing branch. | `8013e400_FUN_8013e400.c` |
| 3 | `0x801285D0` | 252 B | Neighbour of the wall-slip helper; ground probe plus two effect names. | `801285d0_FUN_801285d0.c` |
| 4 | `0x801286D0` | 216 B | Companion reset that clears four bits and seeds `+0x114`/`+0x110`/`+0x10C`/`+0x11C`. | `801286d0_FUN_801286d0.c` |
| 5 | `0x8013D7E0` | 272 B | Spin-exit setup called from the wave 12 magma/landing path. | `8013d7e0_FUN_8013d7e0.c` |
