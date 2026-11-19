# Coverage: wave 7 scene/state/task cluster

## How to read this file

This pass continues the scene framework in `docs/COVERAGE_SCENE.md` and the
task/state base in `docs/COVERAGE_r80140_80180.md`. It adds **20 function
bodies** in two existing modules:

- `src/game/scene.rs` - 7 functions: the save-record helpers
  `0x800E1840`/`0x800E1940`, the scene lifecycle helpers
  `0x800E1D20`/`0x800E1D70`/`0x800E1DD0`, and the scene sound dispatch
  `0x800E2220`/`0x800E2340`;
- `src/game/task.rs` - 13 functions: the fixed-step movers
  `0x8015F340..0x8015F4F0`, the task-list header/callback pair
  `0x8015F600`/`0x8015F6D0`/`0x8015F6E0`, and the state-descriptor/state-object
  constructors and releases `0x8015F7C0..0x8015F9F0`.

Column meanings:

- **address** - virtual address of the entry point in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** - the Rust function in the `rust_file` column;
- **role** - one plain sentence;
- **status** - `translated` means all recorded branches, stores and calls are
  present, with callees outside the batch left as absolute-address calls;
  `partial` would mean a call or argument cannot be reproduced exactly (there
  are none in this pass);
- **confidence** - `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when an argument binding, register pair or cast chain
  had to be inferred from the prototype or the call sites;
- **evidence** - the Ghidra `.c` file under
  `work/ghidra/main-gb/export/`.

Notation inside the Rust bodies follows `docs/RUST_STYLE.md`: `// observed:`
facts from the Ghidra output, `// inferred:` roles reconstructed from call
sites, `// unknown:` unresolved details. Every function keeps an `# Original`
block naming the Ghidra function, its address, its size and its evidence file.

Two register conventions are reused from the earlier documents:

1. `r13 = 0x8042F680` (`0x800041C0`, `__init_registers`), so `r13 - N` is the
   absolute BSS address `0x8042F680 - N`.
2. `r2 = 0x80433080`; the only `r2`-relative operand in this batch is
   `r2 - 0x5488 = 0x8042DBF8`, the float zero constant used by
   `0x8015F480`.

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, any
`mod.rs`, `Cargo.toml`, `platform.rs`, generated documents or another lane's
files. Files written: `src/game/scene.rs`, `src/game/task.rs`, this document and
`ledger/agents/wave7-scene.csv`.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this batch | 20 |
| Ghidra function bytes | 1,588 |
| `translated` | 20 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 14 |
| Confidence `medium` | 6 |
| Confidence `low` | 0 |
| Destination `src/game/scene.rs` | 7 |
| Destination `src/game/task.rs` | 13 |

Every address was checked against `ledger/functions.csv` (absent before this
pass) and against the two Rust files (no pre-existing `fn_<addr>` body), so
each ledger row describes a body that now exists.

## Cluster 1 - save-record helpers (`0x800E1840`, `0x800E1940`)

These two extend the boot/save cluster translated in `docs/COVERAGE_SCENE.md`.
Both operate on a save-manager object whose header starts at `+0x20`, and both
use the same two record accessors that `save_task_construct` already calls
(`0x800E0380` for the destination array, `0x800E03B0` for the source array),
so the record layout stays consistent with the 0x980-byte records documented
for the save image.

`0x800E1840` is the "publish the per-record fields" step: it fills five BSS
tables at `0x80354E60..0x80354EB0` and stores the value returned by
`0x800CE110` in `0x80429CC0`. `0x800E1940` is the copy step: 0x980 bytes from
the `0x800E03B0` record to the `0x800E0380` record.

## Cluster 2 - scene lifecycle (`0x800E1D20..0x800E1DD0`)

The three helpers sit between `scene_base_is_finished` (`0x800E1AA0`) and the
transition globals (`0x800E1DB0`):

```text
scene_teardown(this, free)        0x800E1D20
  free != 0 -> 0x80107F10, 0x80108260
  -> 0x80162110(this, free)       generic scene destructor thunk

scene_is_transition_pending()     0x800E1D70
  -> 0x8006C610() != 0            (-v | v) >> 31

scene_request_default()           0x800E1DD0
  -> 0x801017D0(0, 1)             engine wrapper around scene_request
```

`0x80107F10` is the same gate used by `scene_request` (`0x800E1E60`) to read
the fade object, so the pair inside `scene_teardown` is `// inferred:` a
fade-stop/fade-release step; the Ghidra output shows both calls with no
arguments and their results discarded.

## Cluster 3 - scene sound dispatch (`0x800E2220`, `0x800E2340`)

Both routines translate a small scene/sound id into a sequencer call. Helper
`0x800E22F0` (not in this batch) wraps `0x800E2220` with a position vector, so
the "play at position" path is the caller's job.

```text
scene_play_sound(_, receiver, index, cue)       0x800E2220
  cue == -1 -> nothing
  0x8005FB90(cue) != 0
    vtable[+0xE0]() != 0 -> cue = 0
  0x800B3540(receiver, 0x802F2F08[index], cue, 0)
  index < 5 -> 0x80060690(0x802F2F30[index], cue)
  else       -> 0x80060460(cue, 1, 1)

scene_play_sound_index(_, receiver, index)      0x800E2340
  index == 8 -> 0x800B35B0(receiver), 0x80060590(1)
  else       -> 0x80060690(0x802F2F80[index], -1)
                0x800B3540(receiver, 0x802F2F58[index], -1, 0)
```

The table bases and the `-1`/`1` constants are `// observed:`; the meaning of
each table entry is `// unknown:`.

## Cluster 4 - fixed-step movers (`0x8015F340..0x8015F4F0`)

Five variants of one idea: walk a value toward a target by a signed step and
return `1` once it is there.

| address | Rust name | value type | notes |
|---|---|---|---|
| `0x8015F340` | `step_short_clamped` | `i16` | target is `i16`; step is truncated to `i16` before the add |
| `0x8015F3C0` | `step_int_clamped` | `i32` | `0x8015F3B0` is a 4-byte thunk here |
| `0x8015F420` | `step_int_clamped_b` | `i32` | byte-identical body; `0x8015F410` is a 4-byte thunk here |
| `0x8015F480` | `step_float_clamped` | `f32` | arguments are two `double`s in `f1`/`f2` plus the pointer in `r3` |
| `0x8015F4F0` | `step_short_clamped_wrap` | `i16` | arrival test uses the wide target; the stored step is `(i16)` truncated |

All five return `0` when the step is zero and the value is not already at the
target. The `i32` variants clamp by testing `step * (next - target) >= 0`; the
`i16` variants use the same product after the truncating add. The original
multiplies integers, so the Rust uses `wrapping_mul` to keep the PowerPC
overflow semantics.

## Cluster 5 - task registration (`0x8015F600..0x8015F6E0`)

`0x8015F600` is the initialiser that `0x8015F620` (`run_task_list`) needs:

```text
task_list_init(header, table, count)   0x8015F600
  header[0]  = table
  header[4]  = (u16) count
  header[6]  = 0
```

`0x8015F6D0` reads the global callback pointer `0x80429408` (`r13 - 0x6278`);
`0x8015F6E0` loads it and calls it with the two arguments unchanged. No writer
of the pointer exists in this band, so its installer is `// unknown:`; the
original does not null-check it, and neither does the Rust.

## Cluster 6 - state descriptors and state objects (`0x8015F7C0..0x8015F9F0`)

| address | Rust name | role |
|---|---|---|
| `0x8015F7C0` | `state_descriptor_init` | vtable `0x80328A18` at `+0`, name at `+4`, unique id at `+8` |
| `0x8015F800` | `state_descriptor_release` | optional `0x802B9280` free, returns `this` |
| `0x8015F930` | `state_descriptor_id` | reads `+8` |
| `0x8015F9A0` | `state_object_init` | vtable `0x80328A40` at `+0`, name `+4`, value `+8`, five flag bytes, sub-object `+0x14..+0x20` |
| `0x8015F9F0` | `state_object_release` | optional `0x802B9280` free, returns `this` |

The id comes from the `s32` counter at `0x8042A360` (`r13 - 0x5320`) and is
only incremented when the name pointer is non-null; an unnamed descriptor keeps
id `0`. `state_object_init` writes `DAT_80377A04` into the embedded
sub-object's `+0x18` slot and the caller's `sub` value into `+0x14` and `+0x1C`.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Home | Evidence |
|---|---|---|---|---|---|---|
| `0x800E1840` | `save_manager_init` | Resets the save header, publishes the shared value and fills five per-record tables | translated | medium | `game/scene.rs` | `800e1840_FUN_800e1840.c` |
| `0x800E1940` | `save_record_copy` | Copies one 0x980-byte save record over another | translated | high | `game/scene.rs` | `800e1940_FUN_800e1940.c` |
| `0x800E1D20` | `scene_teardown` | Stops the fade (when asked) and forwards to the generic scene destructor thunk | translated | medium | `game/scene.rs` | `800e1d20_FUN_800e1d20.c` |
| `0x800E1D70` | `scene_is_transition_pending` | Returns `1` when `0x8006C610` is non-zero | translated | high | `game/scene.rs` | `800e1d70_FUN_800e1d70.c` |
| `0x800E1DD0` | `scene_request_default` | Requests the default scene with `force = 1` | translated | high | `game/scene.rs` | `800e1dd0_FUN_800e1dd0.c` |
| `0x800E2220` | `scene_play_sound` | Resolves a cue id and drives the sequencer plus a per-index command | translated | medium | `game/scene.rs` | `800e2220_FUN_800e2220.c` |
| `0x800E2340` | `scene_play_sound_index` | Drives the sequencer from two id-indexed tables, with id `8` special-cased | translated | high | `game/scene.rs` | `800e2340_FUN_800e2340.c` |
| `0x8015F340` | `step_short_clamped` | Moves an `i16` toward a `i16` target by a `i16` step | translated | high | `game/task.rs` | `8015f340_FUN_8015f340.c` |
| `0x8015F3C0` | `step_int_clamped` | Moves an `i32` toward a target by a step | translated | high | `game/task.rs` | `8015f3c0_FUN_8015f3c0.c` |
| `0x8015F420` | `step_int_clamped_b` | Second `i32` entry with the same body | translated | high | `game/task.rs` | `8015f420_FUN_8015f420.c` |
| `0x8015F480` | `step_float_clamped` | Moves an `f32` toward a `double` target by a `double` step | translated | medium | `game/task.rs` | `8015f480_FUN_8015f480.c` |
| `0x8015F4F0` | `step_short_clamped_wrap` | `i16` mover with a wide target and a truncated step | translated | high | `game/task.rs` | `8015f4f0_FUN_8015f4f0.c` |
| `0x8015F600` | `task_list_init` | Seeds table/count/index of a task-list header | translated | high | `game/task.rs` | `8015f600_FUN_8015f600.c` |
| `0x8015F6D0` | `task_callback` | Reads the global task callback pointer | translated | high | `game/task.rs` | `8015f6d0_FUN_8015f6d0.c` |
| `0x8015F6E0` | `invoke_task_callback` | Calls the global task callback with two arguments | translated | high | `game/task.rs` | `8015f6e0_FUN_8015f6e0.c` |
| `0x8015F7C0` | `state_descriptor_init` | Constructs a descriptor, assigning a counter id when named | translated | high | `game/task.rs` | `8015f7c0_FUN_8015f7c0.c` |
| `0x8015F800` | `state_descriptor_release` | Frees a descriptor when asked; returns it | translated | medium | `game/task.rs` | `8015f800_FUN_8015f800.c` |
| `0x8015F930` | `state_descriptor_id` | Reads the descriptor id at `+8` | translated | high | `game/task.rs` | `8015f930_FUN_8015f930.c` |
| `0x8015F9A0` | `state_object_init` | Constructs the larger state object and its embedded sub-object | translated | high | `game/task.rs` | `8015f9a0_FUN_8015f9a0.c` |
| `0x8015F9F0` | `state_object_release` | Frees a state object when asked; returns it | translated | medium | `game/task.rs` | `8015f9f0_FUN_8015f9f0.c` |

## What is proven, what is inferred

**Observed (high):**

- the five BSS tables filled by `0x800E1840` and their widths: one `u8` array
  indexed by the record's own slot value, four `u32` arrays indexed `0..4`;
- the 0x980-byte record copy in `0x800E1940` and the accessor directions
  (`0x800E03B0` source, `0x800E0380` destination), matching
  `save_task_construct`;
- the `(-v | v) >> 31` non-zero test in `0x800E1D70`;
- the call `0x801017D0(0, 1)` in `0x800E1DD0`;
- the sound tables `0x802F2F08`, `0x802F2F30`, `0x802F2F58`, `0x802F2F80` and
  the `index == 8` / `index < 5` split;
- the task-list header layout `u32 table / u16 count / u16 index` (also
  confirmed by `0x8015F600` itself);
- the descriptor vtable `0x80328A18`, the state-object vtable `0x80328A40`, the
  counter `0x8042A360`, and the `name == 0 -> id 0` bypass;
- every field offset in `state_object_init` (`+0x0C..+0x10` bytes,
  `+0x14/+0x18/+0x1C/+0x20` words).

**Inferred (medium):**

- `0x800E1840`'s second argument: Ghidra typed the function `void` and passed
  `extraout_r4` to `0x800CE5F0`, the same pattern as `0x800CF410`; the Rust
  takes `arg: i32` and documents it.
- `0x800E1D20`'s two fade calls: no arguments are shown, so their receiver is
  `// unknown:`; the Rust calls them with none.
- `0x800E2220`'s vtable call at `*(object + 0x60) + 0xE0`: shown with no
  argument; the Rust calls the slot with none and marks the receiver unknown.
- `0x8015F480`'s float constant: `r2 - 0x5488` is read at run time; `0.0f` is
  the inferred value, and the comparison would still match whatever the image
  holds.
- `0x8015F800`/`0x8015F9F0`: Ghidra prints `FUN_802b9280()` with no argument,
  so the freed pointer (`this`, still in `r3`) is an inference. The same
  convention is already used by `save_task_release` in `game/scene.rs`.

## Unknowns / open items

1. **Table meanings.** The five `0x80354E60..` tables and the four `0x802F2F..`
   tables have no names; only their widths and indices are proven.
2. **Global callback installer.** Nothing in this band writes `0x80429408`, so
   the installer of the task callback is unknown.
3. **Scene/sound id names.** Ids `5`, `8`, `9` and the `-1` sentinel appear in
   the bodies, but their game meanings are not established.
4. **`0x800CE110`.** The producer of the value stored at `0x80429CC0` is
   outside this batch.
5. **Two identical `i32` movers.** `0x8015F3C0` and `0x8015F420` have the same
   body; why the original keeps two entries (overloads in different
   translation units) is not proven.

## Next targets in these clusters

1. `0x800E1F80` / `0x800E20A0` / `0x800E22F0` - the scene sound entry points
   above `scene_play_sound`; `0x800E20A0` reads the player object and needs a
   careful paired-single pass.
2. `0x8015F730` - the variadic diagnostic sink (15 integer/float arguments);
   it needs the `savegpr`/varargs convention spelled out first.
3. `0x80160020` - a state-object virtual dispatch whose jump table Ghidra could
   not recover ("too many branches"), so it needs an instruction-level pass.
4. `0x80160080` - the base constructor of the loader wrapper (`0x8015FEF0`
   base call plus eleven field stores); its four-byte neighbour is already
   referenced from `game/stage.rs`.

## Verification

- `cargo check --lib` was run from the repository root and completes with
  `Finished` / exit code 0; `src/game/scene.rs` and `src/game/task.rs` produce no
  errors and no new warnings. (One intermediate run failed in a concurrent
  lane's `src/ui/menu.rs` on a missing `p32` import; that file is outside this
  pass and was not touched here.)
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
