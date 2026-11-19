# Coverage - wave 23 stage band (`0x80080000..0x800C0000`)

## How to read this file

This document records the wave-23 batch of the stage/course lane. All 25 new
bodies live in `src/game/stage.rs`; no other Rust file was changed. The batch
continues the main.dol band `0x80080000..0x800C0000` after the wave-18 stage
pass and fills the BG-unit actor, stage-block and collision clusters that still
had no Rust body.

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets); written with the `0x` prefix
  here and in the Rust docs, plain hex in the CSV;
- **name** - the Rust function in `src/game/stage.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field meaning, an unrecovered indirect call or
  a register-split is inferred from call sites or surrounding code;
- **bytes** - Ghidra's recorded function size;
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the
Ghidra name, address, size and evidence path.

The only files written by this pass are `src/game/stage.rs`, this document and
`ledger/agents/wave23-stage.csv`. `lib.rs`, every `mod.rs`, `Cargo.toml`,
`platform.rs`, generated documents and other lanes' files were not touched.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 25 |
| Bytes of Ghidra function size | 6,580 |
| `translated` | 25 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 12 |
| Confidence `medium` | 13 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 25 |
| New staging modules | 0 |

The selection was made by walking
`work/ghidra/main-gb/export/functions.tsv` over `0x80080000..0x800C0000`,
subtracting every address already in `ledger/functions.csv`, and then checking
each candidate against the destination file. Every address below was absent
from both. The batch deliberately avoids the paired-single bodies
(`0x80088100`, `0x800883E0`, `0x80088970`, `0x80088B30`, `0x80088E80`,
`0x800892F0`, `0x8008A380`, `0x8008AC60`), which need an instruction-level
pass first.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x80080290` | [`stage_circle_wrap`] | wrap a position around a circular stage boundary | translated | medium | 348 | `80080290_FUN_80080290.c` |
| `0x80080670` | [`bg_unit_contains_point`] | point-in-region test for a BG-unit actor | translated | medium | 520 | `80080670_FUN_80080670.c` |
| `0x800878D0` | [`fn_800878d0`] | forward to virtual slot `+0x14` | translated | medium | 16 | `800878d0_FUN_800878d0.c` |
| `0x80087CA0` | [`bg_unit_texture_alloc`] | allocate the 0x30 x 0x30 BG-unit texture | translated | high | 76 | `80087ca0_FUN_80087ca0.c` |
| `0x80087CF0` | [`bg_unit_texture_init`] | initialise that texture and blit its first cells | translated | high | 116 | `80087cf0_FUN_80087cf0.c` |
| `0x80087D70` | [`bg_unit_tile_blit`] | copy a 2 x 2 grid of 0x60-byte tile cells | translated | high | 348 | `80087d70_FUN_80087d70.c` |
| `0x80087ED0` | [`bg_unit_flag_refresh`] | refresh the BG unit's current-stage byte | translated | high | 84 | `80087ed0_FUN_80087ed0.c` |
| `0x80087F40` | [`bg_unit_effect_spawn_a`] | queue the `0x47` stage effect from a BG-unit record | translated | medium | 228 | `80087f40_FUN_80087f40.c` |
| `0x80088030` | [`bg_unit_effect_spawn_b`] | spawn a stage block at the biased record position | translated | medium | 208 | `80088030_FUN_80088030.c` |
| `0x80088210` | [`bg_unit_kind_code`] | map a kind byte to `0/1/2/4` | translated | high | 84 | `80088210_FUN_80088210.c` |
| `0x80088270` | [`bg_unit_available_count`] | count present-but-unplaced BG units | translated | medium | 104 | `80088270_FUN_80088270.c` |
| `0x800890E0` | [`stage_block_data_getter`] | return the static block descriptor `0x80310A94` | translated | high | 16 | `800890e0_FUN_800890e0.c` |
| `0x80089B60` | [`stage_block_release_check`] | run the `+0x3C` release hook | translated | high | 16 | `80089b60_FUN_80089b60.c` |
| `0x80089EC0` | [`stage_block_vtable_call8`] | forward to virtual slot `+0x08` | translated | medium | 16 | `80089ec0_FUN_80089ec0.c` |
| `0x8008A2E0` | [`stage_block_bind_resource`] | create a render buffer and bind two slots | translated | high | 144 | `8008a2e0_FUN_8008a2e0.c` |
| `0x8008A5E0` | [`stage_block_bind_pair`] | create a buffer and bind two selector-dependent slots | translated | medium | 172 | `8008a5e0_FUN_8008a5e0.c` |
| `0x8008A6D0` | [`stage_block_init`] | initialise the block texture and coin model | translated | high | 156 | `8008a6d0_FUN_8008a6d0.c` |
| `0x8008A780` | [`stage_block_actor_setup`] | build the block matrix and program four slots | translated | medium | 344 | `8008a780_FUN_8008a780.c` |
| `0x8008A9C0` | [`stage_block_gx_state`] | program the fixed GX state block | translated | high | 672 | `8008a9c0_FUN_8008a9c0.c` |
| `0x8008B030` | [`stage_block_spawn`] | create a stage block from a location and float | translated | medium | 180 | `8008b030_FUN_8008b030.c` |
| `0x8008BBD0` | [`stage_block_model_dispatch`] | choose the block model and render mode | translated | medium | 656 | `8008bbd0_FUN_8008bbd0.c` |
| `0x8008BE60` | [`stage_block_resources_ready`] | bind the block resource when it is loaded | translated | high | 112 | `8008be60_FUN_8008be60.c` |
| `0x8008BED0` | [`stage_block_render_setup`] | install the model dispatcher as callback | translated | high | 100 | `8008bed0_FUN_8008bed0.c` |
| `0x8008D460` | [`stage_area_overlap_x`] | x-ordered collision-area overlap resolver | translated | medium | 952 | `8008d460_FUN_8008d460.c` |
| `0x8008D8B0` | [`stage_area_overlap_y`] | y-ordered collision-area overlap resolver | translated | medium | 912 | `8008d8b0_FUN_8008d8b0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. BG-unit actor (`0x800878D0..0x80088270`)

The object is the same 0x100-byte BG-unit actor already described by the
wave-4 BG-unit model code. Its placement record is 0x10 bytes: two floats at
`+0`/`+4`, four bytes at `+8..+0xB`, a `u16` at `+0xC`. The signed byte at
`+0xB` is `-1` when the record has no placement index; that bit is packed into
the effect word at bit 18.

`0x80087CA0` allocates the texture and returns `texture + 0x20`, exactly the
pixel pointer that `stage_block_texture_init` (wave 18) stores. `0x80087CF0`
initialises the same object and calls `0x80087D70`, the unrolled tile blit.
The blit walks six 0x20-byte rows and four 24-halfword runs per cell; its
source address formula is kept verbatim.

`0x80087F40` and `0x80088030` are the two effect spawners. They differ in how
they bias the source position (`0` versus `(8, -16)`) and in which central
spawn they call (`0x80064610` effect versus `0x800925F0` stage block), but both
end with the same `0x8006A4B0` + `0x80198C30(manager, 0x274, ...)` pair.

`0x80088210` is a pure literal mapping; `0x80088270` counts the difference
between two global counters and is used only by the bigger stage spawner
`0x800883E0` (not in this batch).

### B. Stage-block render/readiness (`0x800890E0..0x8008BED0`)

These are the helpers called from `0x800892F0` (the stage-environment setup,
not in this batch). The block object is the 0x18C-byte record created by
`0x800925F0`:

| offset | meaning |
|---|---|
| `+0x08` | render mode (`1` = A, `2` = B) |
| `+0x0C` | interface pointer used by `0x8008A780` |
| `+0x14` | texture descriptor from `stage_block_texture_init` |
| `+0x18` | texture size (`0x20` for the block) |
| `+0x24` | coin/block model index |
| `+0xD0` | type byte (`0x7F` enters the model dispatcher) |
| `+0xE8` | stage resource / block name base |

`0x8008BBD0` is the only function here with a name-driven cascade. It
compares the block resource name with `"block_"` through `0x802DF098`, then
probes `"MH_model"`, `"SMH_model"`, `"LH_model"` and `"SLH_model"` with
`0x802E1AB4`, selecting `"polygon0"` or `"polygon3"` through the
`r13-0x5998` variant flag and toggling bit 1 of the node's `+0x34` flags.
The three other string pointers (`0x802F8FD8`, `0x802F8FF0`) are runtime
type-name descriptors, empty in the shipped ELF.

`0x8008BED0` installs `0x8008BBD0` as the `+0x1C` callback of the global
buffer owner returned by `0x80164B10`; `0x8008BE60` and `0x8008A2E0` are the
two "create buffer, then bind" wrappers around `0x80164FD0` and
`0x8016A2A0`.

### C. Collision-area overlaps (`0x8008D460` / `0x8008D8B0`)

The pair resolves two overlapping collision-area rectangles. Both areas have
their x/y bounds at `+0x20..+0x28`, a second set of corridor offsets at
`+0x40..+0x4C`, an owner id byte at `+0x2C`, and two `4 x 4` contact tables at
`+0x50`/`+0x70`. The functions reuse the wave-4 accessors
(`collision_area_center_x`, `collision_area_min_y`, ...) and the two
segment-side classifiers (`segment_side_x` at `0x8008D3D0`,
`segment_side_y` at `0x8008D820`).

The `x` version tests the y extents first, the `y` version the x extents.
Each then runs three corridor tests and two `side != 3` (x) / `side != 0` (y)
pairs before writing both centres back. The zero written to the two contact
tables is the `r2-0x7080` `0.0f` word.

### D. Point tests (`0x80080290` / `0x80080670`)

`0x80080670` is the BG-unit containment test: circle (`+0xC8 == 1`), rotated
rectangle (`s16` angle at `[+0xBC]`), or axis-aligned rectangle. The object
stores integer coordinates; Ghidra prints the conversions as
`(float)param_2[...]`, and the Rust keeps them as `read_i32(...) as f32`.

`0x80080290` is the circular wrap: it converts the position offset to polar
coordinates, adds the stored `u16` angle at `+0xC4`, and writes the point back
on the circle. The four `sin`/`cos` calls include two whose results are
discarded; they are kept because the original emits them.

## Uncertainties carried forward

- `0x800878D0` / `0x80089EC0`: Ghidra reports "could not recover jumptable";
  the slot is explicit, but the callee prototype is unknown. The Rust passes
  the object in `r3`, which is the only unchanged register.
- `0x80087F40`: the second argument of `0x80060690` is not printed by Ghidra;
  the Rust calls the address with `10` only.
- `0x80088030`: the `CONCAT21` packing is reproduced as
  `(u16@+0x0A << 8) | u8@+0x08`; the meaning of the two halves is unknown.
- `0x8008A780`: the matrix calls use the documented GX matrix ABI
  (`0x801C0BD0` translation, `0x8016EDD0` rotation, `0x802574E0` bind).
  The `+0xDC..+0xE4` fields are set to `1.5f`; their role is inferred.
- `0x8008A9C0`: the GX state names behind the twenty calls are unknown; the
  body is reproduced call-for-call, including the `0xCC008000` FIFO write.
- `0x8008B030`: the float argument arrives in `f1` and is not in Ghidra's
  parameter list (printed as `extraout_f1`).
- `0x8008BBD0`: the `+0xC8` interface descriptor and the node `+0x34` bit 1
  are inferred from the matching set/clear pair.
- `0x8008D460` / `0x8008D8B0`: the `3` and `2` / `0` and `1` side constants
  come from the documented return values of the two segment classifiers.
- `0x8008A090` (block transform) was **not** selected: its 25-word stack
  descriptor is only partially initialised in the Ghidra output, so a faithful
  body cannot be written without an instruction-level pass.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original
  argument registers; none was "simplified" away.
- In-batch calls are direct Rust calls: [`bg_unit_texture_init`] ->
  [`bg_unit_tile_blit`], [`stage_block_init`] -> [`stage_block_texture_init`]/
  [`stage_block_coin_models_load`], [`bg_unit_flag_refresh`] ->
  [`is_current_stage_unit`], and [`stage_block_model_dispatch`] ->
  [`stage_render_mode_a`]/[`stage_render_mode_b`].
- `cargo check` reports no diagnostics for `src/game/stage.rs`.
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
