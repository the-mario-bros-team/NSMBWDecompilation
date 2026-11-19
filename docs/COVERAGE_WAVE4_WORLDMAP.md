# Wave 4 WorldMap third pass

## How to read this file

This page records the twenty-five `main.dol` functions added to
[`src/game/worldmap.rs`](../src/game/worldmap.rs) by the wave 4 pass. They were
selected from the `0x80100000..0x80180000` export range and checked against
`ledger/functions.csv`, `ledger/agents/r80100_80140.csv`,
`ledger/agents/r80140_80180.csv` and `ledger/agents/wave3-worldmap.csv`; none
had a previous ledger row.

Each table row has:

- **address** - virtual address in `main.dol`;
- **name** - Rust symbol in `src/game/worldmap.rs`;
- **role** - what the function does in one sentence;
- **status** - `translated` means the recorded control flow is represented;
- **confidence** - `high` when every branch/store is explicit, `medium` when a
  callee argument, a buffer size or a field role is inferred;
- **evidence** - the Ghidra C file used for the translation.

The source module keeps the original `# Original` block on every function.
`// observed:`, `// inferred:` and `// unknown:` comments mark the boundary
between evidence and naming guesses. The machine-readable copy of this table is
`ledger/agents/wave4-worldmap.csv` (module `game::worldmap`).

## What this pass adds

| cluster | functions | why it is one unit |
|---|---:|---|
| `0x34`-byte effect pool | 7 | `0x80104140` and `0x80105410..0x801055F0` are the constructor, two preset wrappers, two allocators, the release byte and the free-entry scan of one pool |
| effect id tables | 6 | `0x801056C0..0x80105C10` map one effect id to a resource id, a dispatch kind, a voice id, a slot index, a channel and a mode-dependent id bank |
| record dispatch | 6 | `0x80104CC0..0x80105240` plus `0x80105EE0` build a record transform, select a renderer and query the resource manager |
| effect manager init | 3 | `0x801025C0`, `0x80102690` and `0x80102C10` clear the effect bit table, post a packed request and install the projection globals |
| transform records | 3 | `0x80106B10`, `0x80106DF0` and `0x80106E60` are the transform-record helpers adjacent to the shared transform-math cluster already in `worldmap.rs`; their WorldMap membership is **not** proven |

Total: **25 functions, 5,948 bytes**.

> The three transform helpers are grouped here because they are numerically
> and structurally adjacent to cluster E (`0x80106C70..0x80106FA0`), which was
> already labelled "shared transform math" in this module. `0x80106DF0` is
> called by the engine frame initialiser `0x800E48D0`, so it may belong to a
> generic animation layer rather than the WorldMap subsystem. They are kept in
> the clearly labelled section instead of a speculative new module.

## Function table

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80104140` | `wm_effect_pool_init` | Fill 30 `0x34`-byte entries and create six render children | translated | high | `work/ghidra/main-gb/export/80104140_FUN_80104140.c` |
| `0x80105410` | `wm_effect_pool_alloc_preset_a` | Allocate one entry with the first preset field layout | translated | high | `work/ghidra/main-gb/export/80105410_FUN_80105410.c` |
| `0x80105440` | `wm_effect_pool_alloc_preset_b` | Allocate one entry with the second preset field layout | translated | high | `work/ghidra/main-gb/export/80105440_FUN_80105440.c` |
| `0x80105480` | `wm_effect_pool_alloc_a` | Reserve an entry and store a three-field transform plus links | translated | high | `work/ghidra/main-gb/export/80105480_FUN_80105480.c` |
| `0x80105530` | `wm_effect_pool_alloc_b` | Reserve an entry and store a three-field transform | translated | high | `work/ghidra/main-gb/export/80105530_FUN_80105530.c` |
| `0x801055D0` | `wm_effect_pool_release` | Clear one entry's active byte | translated | high | `work/ghidra/main-gb/export/801055d0_FUN_801055d0.c` |
| `0x801055F0` | `wm_effect_pool_find_free` | Scan 30 active bytes for a free entry | translated | high | `work/ghidra/main-gb/export/801055f0_FUN_801055f0.c` |
| `0x801056C0` | `wm_effect_resource_id` | Map an effect id to its resource id | translated | high | `work/ghidra/main-gb/export/801056c0_FUN_801056c0.c` |
| `0x80105A20` | `wm_effect_kind` | Map an effect id to a dispatch kind `0..=7` | translated | high | `work/ghidra/main-gb/export/80105a20_FUN_80105a20.c` |
| `0x80105AB0` | `wm_effect_voice_id` | Map an effect id to a voice/sound id | translated | high | `work/ghidra/main-gb/export/80105ab0_FUN_80105ab0.c` |
| `0x80105B40` | `wm_effect_slot_index` | Map an id to render slot `0..3` or `5` | translated | high | `work/ghidra/main-gb/export/80105b40_FUN_80105b40.c` |
| `0x80105B90` | `wm_effect_channel` | Map an id to an audio channel `0..6` | translated | high | `work/ghidra/main-gb/export/80105b90_FUN_80105b90.c` |
| `0x80105C10` | `wm_effect_mode_resource_id` | Mode-dependent ids for effect ids `0x4E..0x53` | translated | high | `work/ghidra/main-gb/export/80105c10_FUN_80105c10.c` |
| `0x80104CC0` | `wm_effect_record_dispatch` | Build a transform and dispatch one record | translated | medium | `work/ghidra/main-gb/export/80104cc0_FUN_80104cc0.c` |
| `0x80104FD0` | `wm_effect_record_play` | Set the channel and play one record's id | translated | high | `work/ghidra/main-gb/export/80104fd0_FUN_80104fd0.c` |
| `0x80105060` | `wm_effect_record_play_position` | Play an id with a position and slot overrides | translated | medium | `work/ghidra/main-gb/export/80105060_FUN_80105060.c` |
| `0x801051D0` | `wm_effect_record_play_transform` | Build a transform then submit it | translated | high | `work/ghidra/main-gb/export/801051d0_FUN_801051d0.c` |
| `0x80105240` | `wm_effect_record_submit` | Dispatch one record to its renderer | translated | medium | `work/ghidra/main-gb/export/80105240_FUN_80105240.c` |
| `0x80105EE0` | `wm_effect_resource_missing` | Query whether an effect resource is absent | translated | medium | `work/ghidra/main-gb/export/80105ee0_FUN_80105ee0.c` |
| `0x801025C0` | `wm_effect_bit_table_init` | Clear the 15-word effect bit table | translated | high | `work/ghidra/main-gb/export/801025c0_FUN_801025c0.c` |
| `0x80102690` | `wm_effect_request` | Pack an effect request and post it to the scene | translated | high | `work/ghidra/main-gb/export/80102690_FUN_80102690.c` |
| `0x80102C10` | `wm_projection_globals_init` | Install the projection globals and the register callback | translated | high | `work/ghidra/main-gb/export/80102c10_FUN_80102c10.c` |
| `0x80106B10` | `wm_transform_record_copy` | Copy a 3-word record from `base + (index+1)*0xC` | translated | high | `work/ghidra/main-gb/export/80106b10_FUN_80106b10.c` |
| `0x80106DF0` | `wm_anim_transform_create` | Allocate and bind a 0x24-byte transform object | translated | high | `work/ghidra/main-gb/export/80106df0_FUN_80106df0.c` |
| `0x80106E60` | `wm_anim_transform_bind` | Bind a source object into an allocated transform | translated | high | `work/ghidra/main-gb/export/80106e60_FUN_80106e60.c` |

## Evidence notes

### The `0x34`-byte pool

This is a second WorldMap effect pool, distinct from the `0x55`-entry,
`0x150`-byte pool translated in waves 1 and 3:

| property | value | evidence |
|---|---:|---|
| entry stride | `0x34` | `0x80105480`, `0x80105530`, `0x801055D0`, `0x801055F0` |
| entry count | `0x1E` (30) | `0x801055F0` (three blocks of ten) |
| free tag | `0x79` | `0x80104140` writes it, both allocators refuse it |
| entry data start | pool base `+0x24` | `0x80104140`; allocators write `tag` at `+0x24` |
| active byte | entry base `+0x50` | `0x801055F0`, `0x801055D0`, both allocators |
| transform seed | `0x80377DC8/CC/D0` | `0x80104140` |

`0x80104140` is written as ten iterations of three unrolled entry blocks
(`+0x24`, `+0x58`, `+0x8C`, then `+= 0x9C`). The Rust keeps that grouping via
`init_effect_pool_entry`, so the store order inside an entry still matches the
`lfs`/`stfs` sequence in the raw instructions.

### Save/restore helpers, not calls

`0x80104140`, `0x80105480`, `0x80105530`, `0x80105060` and `0x80105240` start
with calls Ghidra prints as `FUN_802DCD58`, `FUN_802DCD68`, `FUN_802DCD74` or
`FUN_802DCD6C`, and end with `FUN_802DCDB4` / `FUN_802DCDB8` /
`FUN_802DCDC0`. Those are the CodeWarrior EABI `_savegpr0_N` / `_restgpr0_N`
helpers. The `undefined8` Ghidra invents for them is the preserved incoming
`r3:r4` pair, and on PowerPC a 64-bit value is returned high word in `r3`, low
word in `r4`.

This pass checked the raw instructions instead of trusting the decompiler's
`tag`/`base` order:

- `0x80103450` calls `0x80103500` with the incoming `r3`, and `0x80103484`
  compares the incoming `r4` with `0x2F`;
- `0x801054B4` calls `0x801055F0` with the incoming `r3`, and `0x801054C8`
  compares the incoming `r4` with `0x79`;
- `0x80105494..0x801054B0` save `r3..r10` as `base, tag, p3..p7, ptr`.

Consequently the two pool families take `(base, tag, ...)`, not
`(tag, base, ...)`.

### Correction to wave 3

`wm_effect_slot_alloc` (`0x80103440`) and its wrapper
`wm_effect_slot_alloc_default` (`0x80103420`) were declared `(tag, base, ...)`
in wave 3. The raw instructions above prove the opposite order, so wave 4
swapped the two parameters. The body, the stores and the `0x2F` check are
unchanged; only the Rust parameter order now matches the binary. The wave-3
coverage page still describes the old order and should be treated as
superseded on this point.

### Resolved constants

- `0x8042D150/154/158` - `2160.0`, `-30.0`, `-478.0` (verified in `main.elf`).
- `0x8042CF6C` (`r2 - 0x6114`) - `23`; `0x8042CF84` (`r2 - 0x60FC`) - `38`.
- `0x8042D1F0` (`r2 - 0x5E90`) - raw word `0x00000001`; `0x8042D1F4`
  (`r2 - 0x5E8C`) - `0`.
- `0x80377DC8/CC/D0` - the three transform seed words copied into every pool
  entry.
- `0x8042A488` (`r13 - 0x51F8`) - effect/render manager pointer.
- `0x8042A24D` (`r13 - 0x5433`) - mode byte.
- `0x8042A288` (`r13 - 0x53F8`) and `0x8042A290` (`r13 - 0x53F0`) - current and
  bound anim-transform pointers.
- `0x8042A230` (`r13 - 0x5450`) - word set to `38` by
  `wm_projection_globals_init`; role unknown.

### Local projection buffers

`0x8006A5E0(dst, src)` ends in `0x800F9DB0`, which writes exactly two floats
(`param_1[0]`, `param_1[1]`). The Rust therefore uses a `[u32; 2]` buffer for
the projected transform in `wm_effect_record_dispatch` and
`wm_effect_record_submit`. `0x800FC4F0(dst, a, b)` writes three words, matching
the three-word record transform at `+0x34..+0x3C` and the `[u32; 3]` buffer.

## Unknowns and inferences

- `wm_effect_pool_init` (`0x80104140`) - the six child objects are identified
  only by their allocation sizes (`0xB8` x5, `0xAC`, `0x17C` x2), vtables
  (`0x80321B70`, `0x8032E038`, `0x8032E000`, `0x8032E078`) and the
  `0x80105B40` tag stored at `+0xAC`. Their gameplay roles are unknown.
- The scale word at `0x8042D1F0` is `0x00000001`. Interpreted as an `f32` it is
  a denormal, so the Rust copies the raw bits; the intended value is unknown.
- `wm_effect_record_dispatch` (`0x80104CC0`) - the three-word transform buffer
  and the two-float projection are sized from the callee write patterns, not
  from a recovered type.
- `wm_effect_record_play_position` (`0x80105060`) - the position argument is an
  `f64` in `f1` per the Ghidra prototype and the saved `f31`; no runtime test
  exists.
- `wm_effect_record_submit` (`0x80105240`) - the local projection is again
  sized from `0x8006A5E0`; the vtable slot meanings (`+0x20`, `+0x24`) are
  inferred from the neighbouring dispatcher.
- `wm_effect_resource_missing` (`0x80105EE0`) - Ghidra declares a nullary
  function because `r3`/`r4` pass straight into `wm_effect_resource_id`; the
  `(base, tag)` parameter list is reconstructed from that pass-through.
- `wm_anim_transform_create` (`0x80106DF0`) - the value passed as the allocator
  tag is also used as the source object for the vtable call. The Rust keeps one
  `source` parameter for both roles.
- The names `resource id`, `kind`, `voice id`, `slot index`, `channel` and
  `mode` are inferred from how the values are used; only the numeric tables are
  proven.

## Verification

- `cargo check` passes from the repository root (exit 0).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- The only pre-existing behaviour change is the wave-3 parameter-order
  correction documented above.
- No `src/lib.rs`, `src/game/mod.rs`, `Cargo.toml` or `src/platform.rs` change
  was made.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---|---|---|
| 1 | `0x80102A80` | 400 B | Loads the 10-row, 4-column stage table that feeds the effect manager. Uses `0x800CE4xx` accessors and three SDA globals. | `80102a80_FUN_80102a80.c` |
| 2 | `0x80104140` callers | - | Find the call sites that allocate the pool object so its header layout is proven rather than inferred. | search for `0x80104140` |
| 3 | `0x80107050`, `0x80107170` | 280 B, 320 B | The `0x1C4`-byte object constructors next to the transform helpers; they use `__va_argsave`-shaped stack frames and float clamps. | `80107050_FUN_80107050.c`, `80107170_FUN_80107170.c` |
| 4 | `0x80102DB0` | 76 B | The remaining r2/GQR division wrapper in the same manager band; constants are now resolved. | `80102db0_FUN_80102db0.c` |
| 5 | `0x80103220` callers | - | The `0x150`-byte pool and the `0x34`-byte pool never call each other; a shared caller would prove whether they are two stages of one effect system. | search for `0x80103220` / `0x80105480` |
