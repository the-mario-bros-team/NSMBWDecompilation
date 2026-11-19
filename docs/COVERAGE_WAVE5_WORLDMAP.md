# Wave 5 WorldMap fourth pass

## How to read this file

This page records the twenty-two `main.dol` functions added to
[`src/game/worldmap.rs`](../src/game/worldmap.rs) by the wave 5 pass. They were
selected from the `0x80100000..0x80180000` WorldMap/UI band and checked against
`ledger/functions.csv`, `ledger/agents/r80100_80140.csv`,
`ledger/agents/r80140_80180.csv`, `ledger/agents/wave3-worldmap.csv` and
`ledger/agents/wave4-worldmap.csv`; none had a previous ledger row.

Each table row has:

- **address** - virtual address in `main.dol`;
- **name** - Rust symbol in `src/game/worldmap.rs`;
- **role** - what the function does in one sentence;
- **status** - `translated` means the recorded control flow is represented;
- **confidence** - `high` when every branch/store is explicit, `medium` when a
  field role or a dropped call argument is inferred;
- **evidence** - the Ghidra C file used for the translation.

The source module keeps the original `# Original` block on every function.
`// observed:`, `// inferred:` and `// unknown:` comments mark the boundary
between evidence and naming guesses. The machine-readable copy of this table is
`ledger/agents/wave5-worldmap.csv` (module `game::worldmap`).

## What this pass adds

| cluster | functions | why it is one unit |
|---|---:|---|
| actor vector helper | 1 | `0x80145980` reads the same `+0x1544` scalar and `+0xDC..+0xE4` axis fields used by the player states |
| player construction/release | 4 | `0x8014B2F0` plus the three deleting-destructor entry points `0x8014B470`, `0x8014B4B0` and `0x8014B510` |
| transform/layout glue | 3 | `0x8014B060`, `0x8014C330` and `0x8014C3A0` copy position/angles into the NW4R layout and emit effects |
| player state handlers | 14 | `0x8014B850..0x8014C690`: ready, move, land, approach, attached, turn, update and exit states of the WorldMap player |

Total: **22 functions, 3,940 bytes**.

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80145980` | `wm_actor_scaled_vector` | Scale the actor's three axis values by `+0x1544` | translated | high | `work/ghidra/main-gb/export/80145980_FUN_80145980.c` |
| `0x8014B060` | `wm_player_update_transform` | Copy position/angles into the layout transform and commit it | translated | high | `work/ghidra/main-gb/export/8014b060_FUN_8014b060.c` |
| `0x8014B2F0` | `wm_player_init` | Construct the WorldMap player actor and its two sub-objects | translated | high | `work/ghidra/main-gb/export/8014b2f0_FUN_8014b2f0.c` |
| `0x8014B470` | `wm_player_free` | Free the actor when the delete flag is positive | translated | high | `work/ghidra/main-gb/export/8014b470_FUN_8014b470.c` |
| `0x8014B4B0` | `wm_player_dispose_a` | Destroy the `+0x18` sub-object and optionally free | translated | high | `work/ghidra/main-gb/export/8014b4b0_FUN_8014b4b0.c` |
| `0x8014B510` | `wm_player_dispose_b` | Second, identical destructor variant | translated | high | `work/ghidra/main-gb/export/8014b510_FUN_8014b510.c` |
| `0x8014B850` | `wm_player_release_secondary_target` | Release `+0x1524` then run two movement helpers | translated | medium | `work/ghidra/main-gb/export/8014b850_FUN_8014b850.c` |
| `0x8014B8C0` | `wm_player_wait_counter` | One-shot / countdown wait gate | translated | high | `work/ghidra/main-gb/export/8014b8c0_FUN_8014b8c0.c` |
| `0x8014BA20` | `wm_player_update_facing_angle` | Ease the facing angle toward the requested halfword | translated | high | `work/ghidra/main-gb/export/8014ba20_FUN_8014ba20.c` |
| `0x8014BAE0` | `wm_player_state_ready` | Enter state 0/1 and seed the target position | translated | medium | `work/ghidra/main-gb/export/8014bae0_FUN_8014bae0.c` |
| `0x8014BBF0` | `wm_player_state_move` | Main movement state: speed, turn and target hand-off | translated | medium | `work/ghidra/main-gb/export/8014bbf0_FUN_8014bbf0.c` |
| `0x8014BE50` | `wm_player_state_land` | Clear flags, reset speeds and notify the target | translated | medium | `work/ghidra/main-gb/export/8014be50_FUN_8014be50.c` |
| `0x8014BFB0` | `wm_player_clear_move_flags` | Clear six movement/action status bits | translated | high | `work/ghidra/main-gb/export/8014bfb0_FUN_8014bfb0.c` |
| `0x8014C020` | `wm_player_state_approach` | Approach a target over states 0/1/2 | translated | high | `work/ghidra/main-gb/export/8014c020_FUN_8014c020.c` |
| `0x8014C170` | `wm_player_state_attached` | Release the target layout and play attach feedback | translated | high | `work/ghidra/main-gb/export/8014c170_FUN_8014c170.c` |
| `0x8014C200` | `wm_player_clear_flag_84` | Clear status `0x84` and the speed field | translated | high | `work/ghidra/main-gb/export/8014c200_FUN_8014c200.c` |
| `0x8014C240` | `wm_player_turn` | Set turn speed and flip facing when the flags allow | translated | high | `work/ghidra/main-gb/export/8014c240_FUN_8014c240.c` |
| `0x8014C330` | `wm_player_sweat_effect` | Emit `Wm_mr_yssweatrun` from the `+0x156C` sub-object | translated | high | `work/ghidra/main-gb/export/8014c330_FUN_8014c330.c` |
| `0x8014C3A0` | `wm_player_idle_sound` | Clear `0x7A` and play the idle voice cue | translated | high | `work/ghidra/main-gb/export/8014c3a0_FUN_8014c3a0.c` |
| `0x8014C410` | `wm_player_update` | Per-frame update: speed, target checks and voice | translated | high | `work/ghidra/main-gb/export/8014c410_FUN_8014c410.c` |
| `0x8014C500` | `wm_player_maybe_dismount` | Start the dismount event when the speed test passes | translated | high | `work/ghidra/main-gb/export/8014c500_FUN_8014c500.c` |
| `0x8014C690` | `wm_player_state_exit` | Dispatch the exit slot and clear two status bits | translated | high | `work/ghidra/main-gb/export/8014c690_FUN_8014c690.c` |

## Evidence notes

### Actor identity

The cluster is the WorldMap player actor, not a generic stage object:

- `main.elf` contains the strings `WM_PLAYER` and `WM_SUBPLAYER` in the same
  `WM_*` object-name table as `WM_COURSE`, `WM_CANNON`, `WM_DIRECTOR` and
  `WM_CLOUD`.
- `wm_player_init` constructs two sub-objects with vtable `0x80329958`, the same
  vtable used by the `0x150`-byte effect-slot sub-object in
  `src/game/worldmap.rs` (cluster D).
- `wm_player_sweat_effect` emits the literal effect name `Wm_mr_yssweatrun`
  from `0x80326714`.

The functions sit between the Wm actor/effect band already covered in
`0x80140590..0x80147B60` and the Wm controller/target helpers in
`0x8014F3D0..0x8014FD60`. The `+0x14D8` target object, `+0x1520` attach handle,
`+0x1524` secondary target and `+0x14A4` state word are the same fields used by
`src/game/interaction.rs`.

### Field layout used by the handlers

| offset | width | meaning | confidence |
|---|---|---|---|
| `+0x60` | word | primary vtable | observed |
| `+0x348` | byte | facing (`0`/`1`) | inferred |
| `+0x36D` | byte | candidate/sub-kind byte (same as interaction lane) | inferred |
| `+0x38C` | byte | target "ready" byte | inferred |
| `+0x38E` | byte | mode byte set to `4` by the constructor | inferred |
| `+0x7C` | 0x... | NW4R layout transform | observed |
| `+0xAC/B0/B4` | 3 floats | world position | observed |
| `+0xB8/BC/C0` | 3 floats | previous position | observed |
| `+0xDC/E0/E4` | 3 floats | axis vector scaled by `+0x1544` | observed |
| `+0x10C` | float | speed | inferred |
| `+0x110` | float | speed factor | inferred |
| `+0x114/118/11C` | 3 words | velocity vector | inferred |
| `+0x14A4` | word | state selector | observed |
| `+0x14A8` | word | sub-state/readiness word | inferred |
| `+0x14D8` | pointer | target/attached object | observed |
| `+0x1520` | word | attach handle passed to `0x80162D00` | inferred |
| `+0x1524` | word | secondary target slot | inferred |
| `+0x1528` | float | previous height | inferred |
| `+0x1544` | float | axis/height scale | inferred |
| `+0x1548` | signed word | wait counter | observed |
| `+0x156C` | 0x128-byte object | effect/layout sub-object with vtable `0x80329958` | observed |
| `+0x1694` | 0x128-byte object | second sub-object with the same vtable | observed |
| `+0x10D4` | word | status flags | observed |
| controller `+0x60` | word | actor vtable used by virtual slots `+0x294`, `+0x380..0x448` | observed |

### Resolved constants

All r2-relative values were read from `main.elf` at `r2 = 0x80433080`:

| symbolic name | absolute address | value |
|---|---:|---:|
| `r2 - 0x5668` | `0x8042DA18` | `0.0f` |
| `r2 - 0x567C` | `0x8042DA04` | `-4.0f` |
| `r2 - 0x781C` | `0x8042B864` | `-4.0f` |
| `r2 - 0x5648` | `0x8042DA38` | `3072.0f` |
| `r2 - 0x5634` | `0x8042DA4C` | `2.0f` |
| `r2 - 0x5658` | `0x8042DA28` | `0.5f` |
| `r2 - 0x5620` | `0x8042DA60` | `-0.5f` |
| `r2 - 0x5624` | `0x8042DA5C` | `4.5f` |
| `r2 - 0x5628` | `0x8042DA58` | `0.01f` |
| `r2 - 0x562C` | `0x8042DA54` | `10.0f` |
| `r2 - 0x5630` | `0x8042DA50` | `0.3f` |
| `r2 - 0x5638` | `0x8042DA48` | `-3.0f` |
| `r2 - 0x5640` | `0x8042DA40` | per-facing byte table `0xA3`, `0xA2` |
| `r2 - 0x5664` | `0x8042DA1C` | `20.0f` |
| `r2 - 0x7838` | `0x8042B848` | `3.628f` |

Other tables:

- `0x80370DCC` - BSS vector table, three floats per entry. It is read at runtime,
  not embedded, because the binary initialises it during startup.
- `0x8042B840` - per-facing multiplier table: `1.0`, `-1.0`.
- `0x80329958` - sub-object vtable shared with the effect-slot cluster.

### Dropped arguments restored from raw instructions

Ghidra dropped several arguments. The Rust uses the register state at the call
site, verified from `main.elf`:

- `0x8014B850`: `0x80162510` is printed with no arguments, but `r3` still holds
  the result of `0x80162D00` at `0x8014B894`; the Rust passes that target.
- `0x8014B8C0`, `0x8014C410`, `0x8014C500`, `0x8014C690`: `0x80046AB0`,
  `0x8004C480`, `0x8004BA50`, `0x80047800` and `0x80048A60` are printed with no
  arguments but every callee declares `int param_1`; `r3` is still `this`.
- `0x8014BBF0`: the three `+0x5C` virtual calls print only part of their
  arguments. Raw instructions show `r3` = target object, `r4` = `0`, `6`, `7`,
  or the unmasked entry from `0x8042DA40`, and `f1/f2/f3` from the
  `0x80370DCC` vector table.
- `0x8014B2F0`: `0x800460D0` is printed as nullary but `r3` is the constructor
  object; the Rust passes `this`.
- `0x8014BBF0`: both speed-easing calls target the 4-byte thunk
  `0x8015F470`, which branches to the `0x8015F480` body. The Rust calls the
  thunk address exactly as the original does.

## Unknowns and inferences

- The state numbers, facing byte, speed factor, target handle and effect slot
  names are inferred from use sites; only the offsets, branches, stores and
  call order are proven.
- `wm_player_init`'s six child/task fields are not decoded beyond their vtable
  addresses. `0x80377A04` is passed to `0x8015FC90`; its layout is unknown.
- `wm_player_update_transform` uses NW4R layout functions `0x8016ECB0`,
  `0x8016EDD0`, `0x8016EEF0`, `0x8016A150`, `0x8016A170` and `0x8016A1A0`.
  Their names are not recovered; only the call order is proven.
- `wm_player_state_move_tail` is an inlined label (`LAB_8014BE00`) of
  `0x8014BBF0`, split out so both paths share the exact same code.
- `0x8014B4B0` and `0x8014B510` are byte-equivalent apart from a redundant
  null test. Both entry points are kept because both are addressable.
- The `Wm_mr_yssweatrun` buffer is sized from the `FUN_800D5790` call (`0x14`
  bytes) and the six-argument virtual call at `0x8014C330`.
- The virtual slot numbers (`+0x84`, `+0x294`, `+0x380`, `+0x388`, `+0x390`,
  `+0x3C4`, `+0x3FC`, `+0x414`, `+0x444`, `+0x448`, `+0x5C`, `+0xA0`) are
  observed but their method names are not recovered.

## Verification

- `cargo check` exits 0 from the repository root.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml` or `src/platform.rs` change
  was made by this pass.
- Every function cites its Ghidra address, byte size and evidence file in the
  `# Original` block.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---|---|---|
| 1 | `0x8014A6F0` | 624 B | The `0xD70`-byte WorldMap map-object constructor; would name the `+0x3B4`/`+0x508` layout fields used by `wm_player_update_transform`. | `8014a6f0_FUN_8014a6f0.c` |
| 2 | `0x8014A960` | 396 B | Map-object matrix/scaling update next to the constructor. | `8014a960_FUN_8014a960.c` |
| 3 | `0x80114AC0` | 224 B | `0x6B8`-byte object constructor with vtable `0x80323890` and a `0x80329958` sub-object; likely a WorldMap node/decoration. | `80114ac0_FUN_80114ac0.c` |
| 4 | `0x8011B770` | 640 B | `0xE70`-byte object with a switch on the `+4` nibble and per-case virtual dispatch; candidate WorldMap node/path object. | `8011b770_FUN_8011b770.c` |
| 5 | `0x8014A260..0x8014B510` | 800 B | Remaining lifecycle helpers around the same map object. | `8014a260_FUN_8014a260.c` |
