# Coverage - wave 27 WorldMap action and landing helpers

## How to read this file

This page records the twenty-five `main.dol` functions added to
[`src/game/worldmap.rs`](../src/game/worldmap.rs) by the wave 27 pass. They
were selected from `0x80100000..0x80140000` after removing every address
already present in `ledger/functions.csv`, every address already translated in
`src/game/worldmap.rs`, and every entry whose Ghidra C record has a removed
block or a GQR reconstruction.

- **address** is the entry virtual address from
  `work/ghidra/main-gb/export/functions.tsv`; it is not a file offset.
- **name** is the Rust symbol in `src/game/worldmap.rs`.
- **role** is the behaviour in one sentence.
- **status** is `translated` when the whole recorded control flow is
  represented. This pass has no `partial` or `stub` rows.
- **confidence** is `high` when every branch, call and store is explicit;
  `medium` when one callee argument, field role or BSS value is inferred.
- **evidence** is the Ghidra C file used for the translation.

Every Rust body keeps its `# Original` block with the Ghidra name, address, byte
size and evidence path. The machine-readable copy of the table is
`ledger/agents/wave27-worldmap.csv` (module `game::worldmap`).

The range is a WorldMap lane. The first group (`0x80129A00..0x8012ADA0`)
contains player action begin/cancel/update helpers that share the
`+0x14A4`/`+0x14A8` timer pair, the `+0xEA4` event receiver and the four-word
parameter record consumed by `0x80127FC0`. The second group
(`0x8013D010..0x8013F100`) is the landing/spin half of the same actor: it
drives the `+0x1458`/`+0x145C` two-word timer and calls the wave 19 spin
helpers that already live in this module.

## Scope and counts

| item | count |
|---|---:|
| functions in this pass | 25 |
| bytes in this pass | 4,816 |
| `high` confidence | 14 |
| `medium` confidence | 11 |
| `partial` | 0 |
| `stub` | 0 |
| new staging files | 0 |

## Selection

Two clusters were taken:

1. **Player action helpers (`0x80129A00..0x8012ADA0`, 9 functions).** The
   group alternates begin/cancel/update triples. Both `begin` bodies build a
   four-word record for `0x80127FC0`; the `cancel` bodies clear long lists of
   action bits and post events `3`/`4`; the `update` bodies run the shared
   movement helpers (`0x8004C480`, `0x8004B730`, `0x8004C310`, `0x801278E0`)
   and gate on the action timer. `0x8012AC90` saves the height and starts the
   water timer; `0x8012ADA0` clamps the horizontal speed to the side table.
2. **Landing/spin helpers (`0x8013D010..0x8013F100`, 16 functions).** This
   group completes the `+0x1458`/`+0x145C` state machine around wave 19's
   `wm_spin_enter`, `wm_spin_release`, `wm_spin_exit_choose`,
   `wm_spin_clear_player_flags`, `wm_spin_reset_if_idle` and
   `wm_land_condition_set`. It adds the two `+0x1418` sub-object gate tests,
   the three-state `0x8013E4B0` dispatcher, the sound/speed steps and the
   final `0x8013F100` predicate.

`0x8013E850` (248 bytes) is called from `0x8013E920` but is **not** translated
in this pass: its Ghidra record still contains a removed unreachable block and
`unaff_GQR0`/`ldexpf` noise. The call is kept by address.

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80129A00` | `wm_slide_begin` | cancel eight bits, seed the timer and the `0x80127FC0` record | translated | medium | `work/ghidra/main-gb/export/80129a00_FUN_80129a00.c` |
| `0x80129B30` | `wm_slide_cancel` | cancel twelve bits and stop events `3`/`4` | translated | high | `work/ghidra/main-gb/export/80129b30_FUN_80129b30.c` |
| `0x80129C20` | `wm_slide_update` | run the four movement helpers and drop two bits | translated | medium | `work/ghidra/main-gb/export/80129c20_FUN_80129c20.c` |
| `0x80129DA0` | `wm_swim_cancel` | cancel thirteen bits and stop events `3`/`4` | translated | high | `work/ghidra/main-gb/export/80129da0_FUN_80129da0.c` |
| `0x8012A000` | `wm_swim_begin` | cancel eight bits and seed the `0x80127FC0` record | translated | medium | `work/ghidra/main-gb/export/8012a000_FUN_8012a000.c` |
| `0x8012A170` | `wm_swim_update` | run the movement helpers and the surface gate | translated | medium | `work/ghidra/main-gb/export/8012a170_FUN_8012a170.c` |
| `0x8012A920` | `wm_swim_drift_update` | movement helpers plus the three-way state request | translated | medium | `work/ghidra/main-gb/export/8012a920_FUN_8012a920.c` |
| `0x8012AC90` | `wm_water_begin` | save height, set speeds and start the water timer | translated | medium | `work/ghidra/main-gb/export/8012ac90_FUN_8012ac90.c` |
| `0x8012ADA0` | `wm_water_cancel` | cancel seven bits and clamp the horizontal speed | translated | high | `work/ghidra/main-gb/export/8012ada0_FUN_8012ada0.c` |
| `0x8013D010` | `wm_landing_sound_stop` | stop the landing sound on actor and target | translated | high | `work/ghidra/main-gb/export/8013d010_FUN_8013d010.c` |
| `0x8013D400` | `wm_landing_commit` | optional ground probe, sound `0x54` and speed reset | translated | medium | `work/ghidra/main-gb/export/8013d400_FUN_8013d400.c` |
| `0x8013D4E0` | `wm_landing_state_sound` | choose sound `0x2F`/`0x29`/`0x2A`/`0x2B` and motion values | translated | high | `work/ghidra/main-gb/export/8013d4e0_FUN_8013d4e0.c` |
| `0x8013D600` | `wm_landing_speed_scale` | scale the descent by four thresholds | translated | high | `work/ghidra/main-gb/export/8013d600_FUN_8013d600.c` |
| `0x8013D7E0` | `wm_spin_setup` | assignment flag, events, mode `3` and timer `0x1E` | translated | medium | `work/ghidra/main-gb/export/8013d7e0_FUN_8013d7e0.c` |
| `0x8013DE40` | `wm_landing_enter` | enter the landing and gate state `0x13B` by range | translated | medium | `work/ghidra/main-gb/export/8013de40_FUN_8013de40.c` |
| `0x8013DF50` | `wm_landing_update` | two-state landing timer | translated | high | `work/ghidra/main-gb/export/8013df50_FUN_8013df50.c` |
| `0x8013E400` | `wm_landing_finish` | condition check then clear the motion fields | translated | high | `work/ghidra/main-gb/export/8013e400_FUN_8013e400.c` |
| `0x8013E4B0` | `wm_landing_state_machine` | dispatch states `0..3` | translated | medium | `work/ghidra/main-gb/export/8013e4b0_FUN_8013e4b0.c` |
| `0x8013E5C0` | `wm_landing_idle_release` | idle test plus the two release calls | translated | high | `work/ghidra/main-gb/export/8013e5c0_FUN_8013e5c0.c` |
| `0x8013E6C0` | `wm_landing_gate_test` | test `0x803767F0` and start state `3` | translated | high | `work/ghidra/main-gb/export/8013e6c0_FUN_8013e6c0.c` |
| `0x8013E7C0` | `wm_landing_gate_timeout` | test `0x803767F0` and start state `2` | translated | high | `work/ghidra/main-gb/export/8013e7c0_FUN_8013e7c0.c` |
| `0x8013E920` | `wm_landing_recover` | sound `0x85`, height and state hand-off | translated | medium | `work/ghidra/main-gb/export/8013e920_FUN_8013e920.c` |
| `0x8013EA50` | `wm_landing_release` | cancel `0xBB` and seed state `4` | translated | high | `work/ghidra/main-gb/export/8013ea50_FUN_8013ea50.c` |
| `0x8013ED10` | `wm_landing_setup_speed` | seed the side-table speed and sound `5` | translated | high | `work/ghidra/main-gb/export/8013ed10_FUN_8013ed10.c` |
| `0x8013F100` | `wm_landing_gate_passed` | report the five-part gate result | translated | high | `work/ghidra/main-gb/export/8013f100_FUN_8013f100.c` |

## Data used by this pass

The `r2` base is `0x80433080` and the `r13` base is `0x8042F680`, both verified
in earlier passes. The float values below were read from `work/dol/main.elf` at
the absolute address shown.

| r2 offset | absolute | value | first use |
|---|---|---:|---|
| `-0x56FC` | `0x8042D984` | `4.2` | `0x8013ED10` |
| `-0x5700` | `0x8042D980` | `-2000.0` | `0x8013E920` |
| `-0x5704` | `0x8042D97C` | `2800.0` | `0x8013E920` |
| `-0x5710` | `0x8042D970` | `-0.1` | `0x8013D600` |
| `-0x5714` | `0x8042D96C` | `-0.047` | `0x8013D600` |
| `-0x5718` | `0x8042D968` | `-0.06` | `0x8013D600` |
| `-0x575C` | `0x8042D924` | `96.0` | `0x8013DE40` |
| `-0x57C4` | `0x8042D8BC` | `-0.25` | `0x8013D600` |
| `-0x5868` | `0x8042D818` | `5.0` | `0x8013ED10` |
| `-0x587C` | `0x8042D804` | `2.5` | `0x8013D600` |
| `-0x58AC` | `0x8042D7D4` | `-3.0` | `0x8012AC90` |
| `-0x58B4` | `0x8042D7CC` | `-0.5` | `0x8013D600` |
| `-0x58B8` | `0x8042D7C8` | `2.85` | `0x80129A00` |
| `-0x58BC` | `0x8042D7C4` | `-1.0` | `0x80129A00` |
| `-0x5904` | `0x8042D77C` | `3.0` | `0x8012A000` |
| `-0x5918` | `0x8042D768` | `2.0` | `0x8013D400`, `0x8012AC90` |
| `-0x591C` | `0x8042D764` | `0.5` | `0x8013D600`, `0x8013ED10` |

The following words are BSS in `main.elf` and are therefore read at runtime:

| address | width | meaning |
|---|---|---|
| `0x803089B0` | data | first word of the `0x80127FC0` record; address is passed, not read |
| `0x8035421C` | data | event object passed to slot `+0x294` by `0x8013D7E0` |
| `0x8035476C` | data | object compared by `0x8013F100` |
| `0x80370E08` | float | controller sound parameter for `0x8013ED10` |
| `0x803711B0`/`B4` | float | controller sound parameters for `0x8013D4E0` |
| `0x803711BC`/`C0` | float | controller sound parameters for `0x8013D400` |
| `0x80371348`/`4C` | float | controller sound parameters for `0x8012AC90` |
| `0x80371408`/`0C` | float | controller sound parameters for `0x8013E920` |
| `0x80375C2C` | data | event object passed to slot `+0x294` by `0x8013E920` |
| `0x80375CAC` | data | event object passed to slot `+0x294` by the swim helpers |
| `0x803767F0` | data | object compared by the two `+0x1418` gate helpers |
| `r13 - 0x51D8` | word | shared audio object (`0x8042A4A8`) |
| `r13 - 0x59E0` | word | assignment word tested by `0x8013D7E0` |
| `r13 - 0x59B8` | word | player count word used by `0x8013D4E0` |

Actor fields read or written by this pass:

| offset | width | meaning | confidence |
|---|---|---|---|
| `+0x348` | byte | surface/side selector into `0x8042B840` | observed |
| `+0x38D` | byte | signed player index | observed |
| `+0x38F` | byte | branch selector in `0x8013E920` | observed |
| `+0x43C` | word | speed/limit word cleared by `wm_water_cancel` | unknown |
| `+0xEA4` | byte block | event receiver for events `1`/`3`/`4` | observed |
| `+0x100C` | word | stored actor handle resolved by `0x80162D00` | inferred |
| `+0x10D4` | word | status flags; bit `2` gates the swim request | observed |
| `+0x10DC` | word | status flags; bit `0` gates the landing report | observed |
| `+0x1418` | object | inline sub-object used by both gate helpers | observed |
| `+0x1454` | word | action state selecting the landing sound | observed |
| `+0x1458` / `+0x145C` | words | landing state and timer | observed |
| `+0x14A4` / `+0x14A8` | words | action state and timer | observed |
| `+0x154C` | word | cue word written with `0xFF` | unknown |
| `+0x1554` | word | spin-assignment gate | observed |
| `+0x156C` | float | vertical speed seeded by `wm_water_begin` | observed |
| `+0x27CC` | byte | previous surface byte | inferred |
| `+0x2A64` | word | controller object; `+0x160` bit `2`, vtable `+0x5C` | observed |
| `+0x2A80` | word | landing condition word from wave 19 | observed |
| `+0x2A84` | halfword | water timer/value | unknown |

## Unknowns and inferences

1. **Roles are inferred from field use.** No string names these functions.
   `slide`, `swim` and `water` describe the motion fields and bit lists the
   bodies share; they do not claim a proven gameplay state.
2. **Dropped callee arguments.** Ghidra prints `FUN_8004C480()`,
   `FUN_8013D7E0()`, `FUN_8013E380()`, `FUN_8013D670()`, `FUN_8013D9F0()`,
   `FUN_8013D8F0()`, `FUN_8013E5C0()`, `FUN_8013D600()` and
   `FUN_80162D00()` without arguments at some call sites. Each callee declares
   an `int param_1` and the actor is still in `r3`, so the Rust passes `actor`.
3. **`0x80162D00` argument in `0x8013E4B0`.** The C prints a nullary call.
   The value tested immediately before is `+0x100C`, and `0x8013E850` stores a
   handle into that same field, so the Rust passes `read_u32(actor + 0x100C)`.
4. **`0x80082240` prototype.** Ghidra declares
   `double FUN_80082240(double, int)`, but the body returns a promoted float.
   [`wm_landing_enter`] calls it through `call_f64_u32_f32`, which keeps the
   `f64` argument and `f32` result. The `96.0` range expression keeps both
   `f64` intermediates of the original.
5. **Controller sound parameters.** The `0x8037xxxx` floats are zero in the
   file image of `main.elf`; they are installed at start-up and are read with
   `read_f32`.
6. **`0x8013F100` height test.** Ghidra prints
   `((int)*(float *)(param_1 + 0xb0) & 0xfU) == 0`, which is a float-to-integer
   truncation followed by a mask, not a bit reinterpretation. The Rust keeps
   `read_f32(...) as i32 as u32 & 0xF`.
7. **`0x8013E400` calls `wm_land_condition_set` then re-tests `+0x2A80`.**
   The condition word is the wave 19 output; a non-zero value runs the full
   spin setup before the motion fields are cleared.
8. **`0x8013E850` is still untranslated.** It is called from `0x8013E920` by
   absolute address; its Ghidra record has one removed block and GQR noise.
9. **`0x8004B460` is declared `double` by Ghidra but returns a float.**
   The Rust uses the existing `call_u32_f32` helper for it.

## Verification

- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- `cargo check` was run from the repository root. At the time of this pass the
  workspace does **not** build because of pre-existing errors in
  `src/game/bases.rs` and `src/runtime/bt.rs` (other lanes); no error or warning
  points at `src/game/worldmap.rs`.
- To isolate this file, a temporary crate under
  `%TEMP%/opencode/wm27check` compiled `src/game/worldmap.rs` with the real
  `src/platform.rs` and two small stubs (`engine::globals::SDA_BASE`,
  `game::scene::scene_request`); `cargo check` finished with exit code 0 and no
  diagnostics for this module.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml`, `src/platform.rs` or other
  lane's file was modified, and no staging file was added.
- Every function cites its Ghidra address, byte size and evidence file in the
  `# Original` block.
- No ledger row was added for a function that does not have a Rust body.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---|---:|---|
| 1 | `0x8013EF50` | 172 B | Companion release/cue helper with only two BSS reads. | `8013ef50_FUN_8013ef50.c` |
| 2 | `0x8012A210` | 348 B | Full swim-start body: sound, event, angle clamp and four position copies. | `8012a210_FUN_8012a210.c` |
| 3 | `0x8013EE00` | 324 B | Continues the `0x8013E920..0x8013F100` landing sequence. | `8013ee00_FUN_8013ee00.c` |
| 4 | `0x8013EA90` | 636 B | Large landing sub-state; needs a block-order check first. | `8013ea90_FUN_8013ea90.c` |
| 5 | `0x8013F1C0` | 388 B | Next gate in the same `+0x1458` family. | `8013f1c0_FUN_8013f1c0.c` |
