# Coverage - wave 9 WorldMap map object and select/attach flow

## How to read this file

This page records the twenty-five `main.dol` functions added to
[`src/game/worldmap.rs`](../src/game/worldmap.rs) by the wave 9 pass. They were
selected from the `0x80140000..0x80180000` lane after removing every address
already present in `ledger/functions.csv`; the checks covered
`ledger/agents/r80140_80180.csv`, `ledger/agents/wave3-worldmap.csv`,
`ledger/agents/wave4-worldmap.csv` and `ledger/agents/wave5-worldmap.csv`.

The page answers:

1. **What was added?** - the table lists the 25 new bodies with their role,
   status, confidence and Ghidra evidence file.
2. **Why these functions?** - *Selection* explains why the `0xD70` map object
   and the attach hand-off were taken together.
3. **What remains unknown?** - *Unknowns and inferences* records every name or
   field role that is inferred instead of proven.

Column meanings:

- **address** - virtual address in `main.dol`.
- **name** - Rust symbol in `src/game/worldmap.rs`.
- **role** - what the function does in one sentence.
- **status** - `translated` means the recorded control flow is represented;
  there are no `partial` or `stub` rows in this pass.
- **confidence** - `high` when every branch, call and store is explicit;
  `medium` when one return type or field role is inferred.
- **evidence** - the Ghidra C file used for the translation.

The source module keeps the original `# Original` block on every function and
uses `// observed:`, `// inferred:` and `// unknown:` comments at the points
where evidence ends and naming begins. The machine-readable copy of this table
is `ledger/agents/wave9-worldmap.csv` (module `game::worldmap`).

## Scope and counts

| item | count |
|---|---:|
| functions in this pass | 25 |
| bytes in this pass | 5,276 |
| `high` confidence | 24 |
| `medium` confidence | 1 |
| `partial` | 0 |
| `stub` | 0 |

The pass contributes 25 rows and 5,276 bytes to the `game::worldmap` total. It
adds no staging file and changes no other module.

## Selection

The two clusters are adjacent in the lane and share the same object fields:

1. **Map-object lifecycle (`0x8014A260..0x8014AFC0`, 9 functions).** These
   helpers either release the child object (`0x8014A260`, `0x8014A2C0`), walk
   its five-link child chain (`0x8014A4F0`), compare its name (`0x8014A5D0`),
   construct and refresh the `0xD70`-byte object (`0x8014A6F0`, `0x8014A960`),
   or run its exit path (`0x8014AE30`, `0x8014AEF0`, `0x8014AFC0`).
2. **Secondary-target select/attach (`0x8014E4E0..0x80150AB0`, 16 functions).**
   This is the `+0x1520` handle / `+0x1524` candidate slot flow that the
   interaction module starts. The bodies cover the event gate, candidate copy,
   variant selection, confirmation, lift timer, commitment, settlement, range
   test, begin/release and secondary-target update.

The map-object constructor was the wave 5 "Next targets" item 1 because its
`+0x3B4`/`+0x508` layout fields are used by `wm_player_update_transform`.
`0x8014A960` was item 2. `0x801502E0` (item 1 of the lane document) was **not**
taken: its Ghidra body contains `unaff_GQR0`/`ldexpf` reconstruction noise and
two "Removing unreachable block" warnings, so it is not a complete control-flow
record. Its dependencies (`0x80150090`, `0x8014FDB0`, `0x8014FF80`) were taken
instead where they are clean.

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8014A260` | `wm_map_object_free` | release a child object and optionally free it | translated | high | `work/ghidra/main-gb/export/8014a260_FUN_8014a260.c` |
| `0x8014A2C0` | `wm_map_object_free_b` | second, identical release entry point | translated | high | `work/ghidra/main-gb/export/8014a2c0_FUN_8014a2c0.c` |
| `0x8014A4F0` | `wm_map_child_chain` | walk up to five child links and return the first non-empty one | translated | high | `work/ghidra/main-gb/export/8014a4f0_FUN_8014a4f0.c` |
| `0x8014A5D0` | `wm_map_object_name_matches` | compare the object's name after the last `:` separator | translated | high | `work/ghidra/main-gb/export/8014a5d0_FUN_8014a5d0.c` |
| `0x8014A6F0` | `wm_map_object_init` | construct the `0xD70`-byte WorldMap map object | translated | high | `work/ghidra/main-gb/export/8014a6f0_FUN_8014a6f0.c` |
| `0x8014A960` | `wm_map_object_refresh` | rebuild the map object's matrix, model and sound state | translated | high | `work/ghidra/main-gb/export/8014a960_FUN_8014a960.c` |
| `0x8014AE30` | `wm_map_object_exit_effect` | play the exit cue, post a sound event and release the owner | translated | high | `work/ghidra/main-gb/export/8014ae30_FUN_8014ae30.c` |
| `0x8014AEF0` | `wm_map_object_exit_effect_b` | same exit path plus the mode-3 reset | translated | high | `work/ghidra/main-gb/export/8014aef0_FUN_8014aef0.c` |
| `0x8014AFC0` | `wm_map_object_exit_if_flagged` | run the exit path only when flags `0xE` are set | translated | high | `work/ghidra/main-gb/export/8014afc0_FUN_8014afc0.c` |
| `0x8014E4E0` | `wm_attach_event_gate` | gate the attach event on state `0xA7` and two flag words | translated | high | `work/ghidra/main-gb/export/8014e4e0_FUN_8014e4e0.c` |
| `0x8014EA30` | `wm_attach_complete` | copy the candidate into the secondary slot and play the attach cue | translated | high | `work/ghidra/main-gb/export/8014ea30_FUN_8014ea30.c` |
| `0x8014EB80` | `wm_attach_cancel` | cancel the attach, clear the variant and reset the secondary slot | translated | high | `work/ghidra/main-gb/export/8014eb80_FUN_8014eb80.c` |
| `0x8014EC40` | `wm_set_attach_variant` | store the variant byte and its mirrored actor field | translated | high | `work/ghidra/main-gb/export/8014ec40_FUN_8014ec40.c` |
| `0x8014EE10` | `wm_attach_confirm` | confirm an attach target and raise the matching event | translated | high | `work/ghidra/main-gb/export/8014ee10_FUN_8014ee10.c` |
| `0x8014F0C0` | `wm_attach_choose_result` | choose the attach outcome from two angle conversions | translated | high | `work/ghidra/main-gb/export/8014f0c0_FUN_8014f0c0.c` |
| `0x8014F200` | `wm_attach_clear_flags` | clear four action bits and release the target's lift flag | translated | high | `work/ghidra/main-gb/export/8014f200_FUN_8014f200.c` |
| `0x8014F290` | `wm_attach_target_lift` | raise and restore the target while the lift timer runs | translated | high | `work/ghidra/main-gb/export/8014f290_FUN_8014f290.c` |
| `0x8014F470` | `wm_attach_committed` | mark the attach committed, seed the target and play the cue | translated | high | `work/ghidra/main-gb/export/8014f470_FUN_8014f470.c` |
| `0x8014F590` | `wm_attach_settle` | settle the attached target and set the retry flags | translated | high | `work/ghidra/main-gb/export/8014f590_FUN_8014f590.c` |
| `0x8014F840` | `wm_attach_follow` | follow the attached target and swap in the next handle | translated | medium | `work/ghidra/main-gb/export/8014f840_FUN_8014f840.c` |
| `0x8014FAD0` | `wm_attach_range_check` | finish the attach by range or select the next candidate | translated | high | `work/ghidra/main-gb/export/8014fad0_FUN_8014fad0.c` |
| `0x8014FDB0` | `wm_attach_begin` | begin a new attach phase and seed the target's owner word | translated | high | `work/ghidra/main-gb/export/8014fdb0_FUN_8014fdb0.c` |
| `0x8014FF80` | `wm_attach_release` | clear the target's lift bit and release the handle | translated | high | `work/ghidra/main-gb/export/8014ff80_FUN_8014ff80.c` |
| `0x80150090` | `wm_attach_finish` | finish the attach when the target is within eight units | translated | high | `work/ghidra/main-gb/export/80150090_FUN_80150090.c` |
| `0x80150AB0` | `wm_attach_secondary_update` | run the secondary-target confirm/retry branch | translated | high | `work/ghidra/main-gb/export/80150ab0_FUN_80150ab0.c` |

## Field layout used by this pass

The offsets below are the ones the translated bodies read and write. They are
`// observed:` from the Ghidra bodies; the semantic labels are `// inferred:`
unless the neighbouring interaction code already uses the same name.

| offset | width | meaning |
|---|---|---|
| `+0x24` | word | target's attach-copy word |
| `+0x60` | word | primary vtable (event slots `+0x294`, `+0x3F4`, `+0x404`, `+0x444`, `+0x448`) |
| `+0x102` | halfword | saved actor angle |
| `+0x17C` | word | target lift/attach bit word |
| `+0x284` | byte | mirrored variant byte |
| `+0x294` | word | target attach state |
| `+0x298` | word | target attach timer/height word (`70.0f` bits) |
| `+0x348` | byte | facing |
| `+0x36C` | byte | target attach kind |
| `+0x36D` | byte | candidate/attach kind |
| `+0x368` | word | target owner word |
| `+0x38D` | byte | actor variant |
| `+0x38E` | byte | actor mode |
| `+0x4FC` | word | map-object exit flags |
| `+0x508..+0x510` | 3 floats | map-object scale |
| `+0x514` | word | map-object model id (`0x568`) |
| `+0x520` | word | map-object sound/state flags |
| `+0x9CC..` | objects | embedded `0x128`-byte sub-objects with vtable `0x80329958` |
| `+0xD48..+0xD68` | 9 words | self/sub-object link table |
| `+0xEA4` | word | action/state word |
| `+0x10D4` | word | status flags |
| `+0x10D8` | word | secondary status flags |
| `+0x151C` | word | attachment phase |
| `+0x1520` | word | attach handle consumed by `0x80162D00` |
| `+0x1524` | word | secondary candidate slot |
| `+0x1530` | word | attach timer |
| `+0x1534` | word | attach finished flag |
| `+0x1538` | word | target-ready word |
| `+0x1540` | byte | saved facing for the angle slot |
| `+0x1541` | byte | saved variant |
| `+0x1548` | word | retry counter |
| `+0x154C` | word | lift timer |
| `+0x1550` | word | lift state (`0..2`) |
| `+0x1554` | float | saved target height |
| `+0x17BC` | word | effect timer cleared on the chosen attach result |

## Evidence notes

### The map object is the `0xD70` object used by the player transform

`wm_map_object_init` allocates `0xD70` bytes, installs vtable `0x80326634` at
`+0x60`, and builds the embedded sub-objects with vtable `0x80329958` (the same
sub-object vtable used by the WorldMap player and by the effect-slot code).
The `0x80329958` family begins at `+0x65C`, `+0x1570`, `+0x1698`, `+0x17D4`,
`+0x1A28`, `+0x1B50`, `+0x1CA0`, `+0x1DF8`, `+0x1F20`, `+0x2074`, `+0x219C`,
`+0x22C4` and `+0x23EC`; the adjacent objects at `+0x18FC`, `+0x2538`, `+0x264C`
and `+0x2A94` use vtable `0x80308A80`, and the two at `+0x2804`/`+0x292C` use
`0x80310ED8`. `wm_map_object_refresh` then reads the `+0x3B4` layout, the
`+0x3F8` model, the `+0x424` NW4R matrix builder and the `+0x508` scale fields
that `wm_player_update_transform` uses. That is why the constructor and
refresh helper are grouped with the player transform code rather than with the
unrelated lifecycle helpers.

### `0x8014A960` is a real matrix construction, not noise

The Ghidra body reads two global floats at `0x80358B6C`/`0x80358B70` and the
`1.1`/`0.5` constants, then passes two pointers into one eight-float stack
block to `0x8007FAD0`. `0x8007FAD0` reads `param_3[0..1]` and
`param_4[0..1]`, so the Rust keeps the same eight-float block and passes
`&values[2]` and `&values[0]` in that order. It does not collapse the block
into a named matrix because the original stack layout is the evidence.

### The attach flow uses two different vtable locations

The attach controller and actor keep their event vtable at `+0x60`; the target
object keeps its vtable at `+0`. The Rust therefore has two helper families:
`vcall60_*` for the `+0x60` slots and `vcall_*` for the target's `+0`
slots. This distinction is easy to miss: `wm_attach_event_gate` calls
`+0x294` through `+0x60`, while `wm_attach_target_lift` calls `+0x8C`/`+0x90`
through the target's `+0`.

### SDA and SDA2 reads

`r13 = 0x8042F680` is `SDA_BASE` from `src/engine/globals.rs`; `r2 =
0x80433080` is `SDA2_BASE`, already defined in this module. The two BSS words
at `SDA_BASE - 0x5678` and `SDA_BASE - 0x5920` are read at runtime because
they are installed during startup. Every SDA2 float used here was read from
`main.elf` and is documented at its `const` declaration.

## Unknowns and inferences

1. `0x8014A260` and `0x8014A2C0` are byte-identical. Both entry points are
   kept because both are addressable; `wm_map_object_free_b` is not a renamed
   copy of a distinct behaviour.
2. `wm_map_object_init` passes the new object to `0x80064110` even though
   Ghidra prints the call as nullary: every other constructor in the binary
   passes its new object in `r3`, and `0x80064110` declares one `int` argument.
3. `wm_map_object_refresh` passes `object` to `0x8014AAF0` for the same reason:
   the callee declares `int param_1` and `r3` still holds `object` at the call.
4. `wm_attach_follow` is `medium`: `0x80064610` is printed as `void`, but every
   caller uses its result as a pointer, so the Rust keeps the `r3` return.
5. The two `0x80358B6C`/`0x80358B70` floats and the
   `0x803716E4..0x803716EC`/`0x8037124C` floats live in BSS/read-only data and
   are read at runtime rather than baked in.
6. The names "candidate", "secondary slot", "variant", "lift" and "commit"
   are inferred from the call shape and from the field names already used by
   `src/game/interaction.rs`. Only the offsets, branches, stores and call order
   are proven.
7. `0x801502E0`, `0x8014F630`, `0x80145650` and `0x8014AC10` were not taken:
   their Ghidra bodies carry `unaff_GQR0`/`ldexpf`/unreachable-block noise, so
   they are not complete control-flow records. They remain future targets.

## Verification

- `cargo check` exits 0 after the 25 bodies were added.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml` or `src/platform.rs` change
  was made by this pass.
- Every function cites its Ghidra address, byte size and evidence file in the
  `# Original` block.
- No ledger row was added for a function that does not have a Rust body.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---|---|---|
| 1 | `0x801502E0` | 256 B | consumes the `+0x1558` candidate and calls `0x8014ED90`; closes the flow started by the new attach helpers. Needs a raw-instruction pass because Ghidra emits `unaff_GQR0`. | `801502e0_FUN_801502e0.c` |
| 2 | `0x8014F630` | 528 B | attachment phase machine that sets `+0x151C` to 2/3/4 and is called by `wm_attach_begin`; raw-instruction pass needed for the same reason. | `8014f630_FUN_8014f630.c` |
| 3 | `0x8014AAF0` | 252 B | the map object's resource/model setup, called first by `wm_map_object_refresh`. Uses `r13` and the `g3d_ice.brres` string. | `8014aaf0_FUN_8014aaf0.c` |
| 4 | `0x8014AC10` | 412 B | map-object per-frame motion next to the refresh helper; Ghidra emits `unaff_GQR0` and one unreachable block. | `8014ac10_FUN_8014ac10.c` |
| 5 | `0x8014A960` callbacks | 3 × 32 B | `0x8014B000`, `0x8014B020`, `0x8014B040` are the matrix callbacks passed to `0x8007FAD0`; all three are tiny and still untranslated. | `8014b000_FUN_8014b000.c` |
