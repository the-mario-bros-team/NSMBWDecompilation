# Coverage - wave 12 WorldMap effect and transition helpers

## How to read this file

This page records the twenty-four `main.dol` functions added to
[`src/game/worldmap.rs`](../src/game/worldmap.rs) by the wave 12 pass. They were
selected from `0x80100000..0x80140000` after removing every address already
present in `ledger/functions.csv` and every address already translated in
`src/game/worldmap.rs` or `src/engine/globals.rs`.

- **address** is the entry virtual address from
  `work/ghidra/main-gb/export/functions.tsv`; it is not a file offset.
- **name** is the Rust symbol in `src/game/worldmap.rs`.
- **role** is the behaviour in one sentence.
- **status** is `translated` when the whole recorded control flow is
  represented. This pass has no `partial` or `stub` rows.
- **confidence** is `high` when every branch, call and store is explicit;
  `medium` when one conversion, return value or field role is inferred.
- **evidence** is the Ghidra C file used for the translation.

Every Rust body keeps its `# Original` block with the Ghidra name, address, byte
size and evidence path. The machine-readable copy of the table is
`ledger/agents/wave12-worldmap.csv` (module `game::worldmap`).

The range is a WorldMap lane: the early `0x80101xxx` helpers drive the map
scene's model/effect transition, and the `0x8011xxxx..0x8013xxxx` group are
one-shot effect helpers on the WorldMap player actor. Their effect names all
carry the `Wm_` WorldMap prefix (`Wm_mr_*`, `Wm_en_*`, `Wm_bg_*`), read from
`main.elf`.

## Scope and counts

| item | count |
|---|---:|
| functions in this pass | 24 |
| bytes in this pass | 5,836 |
| `high` confidence | 20 |
| `medium` confidence | 4 |
| `partial` | 0 |
| `stub` | 0 |
| new staging files | 0 |

## Selection

Three clusters were taken:

1. **Scene/model transition (`0x80101170`, `0x80101230`).** The first helper
   switches the active model record and repacks its fourth word; the second is
   the `0xF` effect spawn and the two record plays it triggers. They are a
   closed pair: `0x80101170` calls `0x80101230` when the mode byte is not `3`.
2. **Object release (`0x80106190`).** A complete release helper next to the
   transform-record cluster; it releases `+8` and `+0x10` and optionally frees
   the object.
3. **WorldMap actor effects (`0x80113490..0x8013E170`, 21 functions).** Every
   function in this group either names a `Wm_*` effect directly or selects one
   from a table, and most read the actor's common `+0xAC..+0xB4` position and
   `+0x38D` player index. The group covers balloon burst, volcano, fireball hit,
   magma wave, spin-down, wall slip, wall kick, half spin, water/swim, landing
   splash, swim paddle, flutter kick, sea-land smoke, move-cliff, spin smoke,
   spin depart, spin jump, magma transition and atiti smoke.

Functions with incomplete Ghidra records were deliberately skipped:

| address | size | reason |
|---|---:|---|
| `0x80106620` | 648 | seven `Removing unreachable block` warnings; the C is not a complete control-flow record. |
| `0x801068B0` | 608 | five `Removing unreachable block` warnings. |
| `0x8012FB10` | 480 | `unaff_GQR0`/`ldexpf` reconstruction plus one unreachable block. |
| `0x80132340` | 244 | the call is printed with an extra `in_r10` and duplicated float arguments; the ABI is not recoverable from the C alone. |

`0x80106B40` (292 bytes) was also left out: its Ghidra body orders the inner
loop after an unconditional `return`, so the printed control flow is not a
complete record even though the intended loop is visible.

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80101170` | `wm_model_record_apply` | switch the active model record and apply its packed flag word | translated | medium | `work/ghidra/main-gb/export/80101170_FUN_80101170.c` |
| `0x80101230` | `wm_effect_transition_spawn` | spawn the `0xF` transition effect and two records | translated | high | `work/ghidra/main-gb/export/80101230_FUN_80101230.c` |
| `0x80106190` | `wm_object_release` | release two owned resources and optionally free the object | translated | high | `work/ghidra/main-gb/export/80106190_FUN_80106190.c` |
| `0x80113490` | `wm_balloon_burst_effect` | play `Wm_mr_balloonburst` and mark the object finished | translated | high | `work/ghidra/main-gb/export/80113490_FUN_80113490.c` |
| `0x80119180` | `wm_volcano_effect_apply` | call a sub-object's `+0xA0` slot with `Wm_bg_volcano` | translated | high | `work/ghidra/main-gb/export/80119180_FUN_80119180.c` |
| `0x8011A680` | `wm_fireball_hit_update` | apply the fireball hit reaction and spawn its effect/sound | translated | high | `work/ghidra/main-gb/export/8011a680_FUN_8011a680.c` |
| `0x8011F430` | `wm_magma_wave_effect` | play the common or normal magma-wave effect | translated | high | `work/ghidra/main-gb/export/8011f430_FUN_8011f430.c` |
| `0x801268F0` | `wm_spindown_effects` | play both spin-down effects | translated | high | `work/ghidra/main-gb/export/801268f0_FUN_801268f0.c` |
| `0x80128830` | `wm_wallslip_effect` | play a side-selected wall-slip effect | translated | medium | `work/ghidra/main-gb/export/80128830_FUN_80128830.c` |
| `0x80128B00` | `wm_wallkick_effect` | snap to the wall side and play the wall kick | translated | medium | `work/ghidra/main-gb/export/80128b00_FUN_80128b00.c` |
| `0x80129220` | `wm_halfspin_effect` | start or poll the half-spin effect | translated | high | `work/ghidra/main-gb/export/80129220_FUN_80129220.c` |
| `0x8012BBA0` | `wm_water_effect` | choose the water/swim effect by `+0x111C` | translated | high | `work/ghidra/main-gb/export/8012bba0_FUN_8012bba0.c` |
| `0x8012FCF0` | `wm_land_effect` | play the landing splash and its scale ramp | translated | high | `work/ghidra/main-gb/export/8012fcf0_FUN_8012fcf0.c` |
| `0x8012FE50` | `wm_swimpaddle_effects` | play both swim-paddle effects | translated | high | `work/ghidra/main-gb/export/8012fe50_FUN_8012fe50.c` |
| `0x8012FF40` | `wm_waterswim_effect` | gate the water-swim effect on the surface height | translated | high | `work/ghidra/main-gb/export/8012ff40_FUN_8012ff40.c` |
| `0x80130040` | `wm_flutterkick_effects` | play both flutter-kick effects | translated | high | `work/ghidra/main-gb/export/80130040_FUN_80130040.c` |
| `0x80130A00` | `wm_sealand_smoke_effect` | play a table-selected sea-land smoke | translated | high | `work/ghidra/main-gb/export/80130a00_FUN_80130a00.c` |
| `0x80134C80` | `wm_movecliff_effect_a` | play the move-cliff effect for modes `4`/`7` | translated | high | `work/ghidra/main-gb/export/80134c80_FUN_80134c80.c` |
| `0x80134D30` | `wm_movecliff_effect_b` | play the move-cliff effect for modes `0xB`/`0xE` | translated | high | `work/ghidra/main-gb/export/80134d30_FUN_80134d30.c` |
| `0x8013D060` | `wm_spinsmoke_effect` | probe the ground and play `Wm_mr_spinsmoke` | translated | medium | `work/ghidra/main-gb/export/8013d060_FUN_8013d060.c` |
| `0x8013D140` | `wm_spindepart_effect` | play `Wm_mr_spindepart` while spinning | translated | high | `work/ghidra/main-gb/export/8013d140_FUN_8013d140.c` |
| `0x8013D230` | `wm_spinjump_effect` | advance the spin-jump state machine | translated | high | `work/ghidra/main-gb/export/8013d230_FUN_8013d230.c` |
| `0x8013E000` | `wm_magmawave_transition` | run the magma transition and atiti tail | translated | high | `work/ghidra/main-gb/export/8013e000_FUN_8013e000.c` |
| `0x8013E170` | `wm_atitismoke_effect` | run the three-state atiti-smoke action | translated | high | `work/ghidra/main-gb/export/8013e170_FUN_8013e170.c` |

## Data used by this pass

| address | width | meaning |
|---|---|---|
| `r2 - 0x5708` | float | `3000.0` used by the atiti `+0x428` slot |
| `r2 - 0x570C` | float | `6500.0`, magma transition Z |
| `r2 - 0x571C` | float | `-0.125` atiti X speed |
| `r2 - 0x5720` | float | `3.25` atiti Y speed |
| `r2 - 0x5724` | float | `5000.0` atiti `+0x428` argument |
| `r2 - 0x5728` | float | `2000.0`, spin-state Z override |
| `r2 - 0x5764` | float | `64.0`, spin-smoke ground margin |
| `r2 - 0x57AC` | float | `24.0`, atiti floor margin |
| `r2 - 0x58A8` | float | `0.8` landing scale |
| `r2 - 0x58D0` | float | `8.0`, wall-kick probe offset |
| `r2 - 0x58FC` | float | `1.0` landing scale |
| `r2 - 0x5900` | float | `10.0`, spin-smoke height offset |
| `r2 - 0x5914` | float | `4.0`, water-surface margin |
| `r2 - 0x5928` | float | `0.0` |
| `r2 - 0x5AC4` | float | `6500.0`, magma-wave Z |
| `r2 - 0x5B80` | float | `-3.0` |
| `r2 - 0x5B84` | float | `5500.0`, fireball Z |
| `r2 - 0x5B88` | float | `4.0`, fireball hit height |
| `r2 - 0x5B90` | float[2] | `3.625`, `-3.625` side table |
| `r2 - 0x5BA0` | float | `0.0` |
| `r2 - 0x5F90` | float | `50.0`, transition height |
| `r2 - 0x5F9C` | float | `0.0`, transition time |
| `r2 - 0x7954` | float | `-0.4375` |
| `r2 - 0x7958` | float | `-4.0` |
| `r13 - 0x5404` | word | effect-record pool |
| `r13 - 0x5414` | word | effect-table base |
| `r13 - 0x5433` | byte | active model mode |
| `r13 - 0x54B8` | word | current WorldMap player object |
| `r13 - 0x5678` | word | shared sound-state object |
| `r13 - 0x5690` | word | effect channel word |
| `r13 - 0x5704` | word | model manager pointer |
| `r13 - 0x5880` | word | scene pointer (`+0x3C`/`+0x44` floor) |
| `r13 - 0x58B0` | word | shared scale/parameter word |
| `r13 - 0x5920` | word | sound/event manager |
| `0x80370DA8` | word[?] | BSS table indexed by `controller+0x152` |
| `0x80371198`, `0x8037119C` | float | BSS spin angles passed to the controller |
| `0x8042B840` | float[8] | side table `1.0`, `-1.0`, `3.628`, `3.5`, `2.3`, `0.5625`, `1.125`, `1.25` |
| `0x802F4E58` | float[?] | water colour scalars indexed by `+0xE8` |

The `0x8042B840` and `0x802F4E58` tables were read directly from
`work/dol/main.elf`. The two BSS spin angles and `0x80370DA8` are read at
runtime because their contents are installed during startup.

## Unknowns and inferences

1. **Roles are inferred from the effect names.** `Wm_` is the WorldMap prefix,
   and the `Wm_mr_*`/`Wm_en_*`/`Wm_bg_*` strings come from `main.elf`. The
   function names describe the effect that is spawned; they do not claim a
   proven gameplay state.
2. **`0x80101170`'s fourth word packing.** Ghidra prints
   `CONCAT13(1, (int3)record[3])`, which the Rust reconstructs as a leading `1`
   byte followed by the low three bytes of the word. The exact callee contract
   of `0x800BBB70` is outside this lane.
3. **Signed conversions in `0x80128830`/`0x80128B00`.** The paired-single-looking
   `CONCAT44(0x43300000, x ^ 0x80000000)` sequence is the PowerPC compiler's
   signed `u32 -> f64` conversion; the Rust uses `read_u32(..) as i32 as f32`
   and divides by `4096.0`. The `0x43300000 80000000` double constant was read
   from `main.elf` at `r2 - 0x58E0`.
4. **Two `void` declarations whose result is used.** Ghidra declares
   `0x800757B0` and `0x800CD2C0` as `void`, but the call sites use `r3` as a
   condition. `wm_spinsmoke_effect` keeps the `r3` return through `call5` and
   `call0_u32`; this is the same convention used by wave 9's
   `wm_map_object_refresh`.
5. **`0x80106190` is a generic release helper.** Its `+8` and `+0x10` fields
   are proven by the two release calls, but the object type is not named by any
   string; its placement in this module is by address adjacency to the
   transform cluster.
6. **Six-effect limit.** The wall-slip and wall-kick helpers only spawn when
   the computed table index is below `6`; the table contents beyond that bound
   were not inspected because the branch is unreachable there.
7. **`0x80129220` recursion.** The recursive tail call receives the object
   returned by `0x8012DBE0`. That callee is `FUN_8012dbe0(int)`, so the Rust
   passes `actor` explicitly even though Ghidra prints a nullary call.
8. **`0x8013D7E0`/`0x8013D600`/`0x8013D400`/`0x8013D8F0`/`0x8013D670`/
   `0x8013D9F0`.** Ghidra prints several of these calls without arguments
   because `r3` still holds the actor; the Rust passes `actor` explicitly after
   checking the callee bodies, which declare `int param_1`.
9. **BSS reads.** `0x80371198`, `0x8037119C` and `0x80370DA8` are zero in
   `main.elf`'s file image. They are loaded at runtime and therefore read with
   `platform::read_f32`/`read_u32`, not baked in.

## Verification

- `cargo check` exits 0 after the 24 bodies were added. The only warnings are
  pre-existing ones in `src/runtime/*.rs`, outside this lane.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml` or `src/platform.rs` change
  was made by this pass.
- Every function cites its Ghidra address, byte size and evidence file in the
  `# Original` block.
- No ledger row was added for a function that does not have a Rust body.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---|---|---|
| 1 | `0x80106620` | 648 B | Builds the large effect/transform record around the `0x801068B0` cluster; needs a raw-instruction pass because Ghidra removed seven unreachable blocks. | `80106620_FUN_80106620.c` |
| 2 | `0x801068B0` | 608 B | Complements `0x80106620`; same unreachable-block limitation. | `801068b0_FUN_801068b0.c` |
| 3 | `0x80106B40` | 292 B | Path interpolation across the `0x14`-byte records; the printed loop order is broken, so recover it from the instruction words. | `80106b40_FUN_80106b40.c` |
| 4 | `0x80132340` | 244 B | Ivy effect that combines two controller transforms; the extra `in_r10` arguments must be resolved first. | `80132340_FUN_80132340.c` |
| 5 | `0x8012FB10` | 480 B | Float-splash state with `unaff_GQR0` noise; only the GQR-save restoration prevents a literal translation. | `8012fb10_FUN_8012fb10.c` |
