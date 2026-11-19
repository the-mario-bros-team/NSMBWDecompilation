# Coverage: wave 31 stage band (`0x80080000..0x800C0000`)

## How to read this file

This document covers the wave-31 batch of the stage/course lane. All 25 new
bodies live in `src/game/stage.rs`; no other Rust source file was changed. The
batch closes the gaps that wave 29 left in the `0x8008xxxx` stage code
(`0x80080900`, `0x80083870`, `0x80088100`, `0x80088970..0x80088E80`,
`0x800892F0`, `0x8008A380`, `0x8008AC60`) and translates the early
`0x8009xxxx` stage-block and stage-effect helpers that follow the block
spawners (`0x80090600`, `0x80091040..0x80091100`, `0x80092D50`,
`0x800931F0..0x80094DE0`).

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** - the Rust function in `src/game/stage.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field meaning, an indirect call, a table value
  or a hardware/effect role is inferred from call sites;
- **bytes** - Ghidra's recorded function size;
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the Ghidra
name, address, size and evidence path.

The only files written by this pass are `src/game/stage.rs`, this document and
`ledger/agents/wave31-stage.csv`. `lib.rs`, every `mod.rs`, `Cargo.toml`,
`platform.rs`, generated documents and other lanes' files were not touched.
`ledger/functions.csv` is unchanged: the agent CSV is the merge input.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 25 |
| Bytes of Ghidra function size | 8,840 |
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
subtracting every address already in `ledger/functions.csv` and every address
already present in `src/game/stage.rs`. Every address below was absent from
both. The `0x8008xxxx` entries are the last untranslated BG-unit, stage-camera
and stage-environment functions in the band; the `0x8009xxxx` entries are the
stage-block model helper and the stage-effect pool/removal cluster that follows
the block spawners documented by wave 29. `0x80081960` (pad/mode selection) and
`0x80082BE0`/`0x80082ED0` (heavily paired-single render loops whose ABI is not
recoverable from the Ghidra C) were deliberately left for an instruction-level
pass.

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x80080900` | [`bg_scene_edge_snap`] | snap a point onto a BG-scene rectangle edge and derive an angle | translated | medium | 1024 | `80080900_FUN_80080900.c` |
| `0x80083870` | [`bg_unit_collision_scan`] | find the first BG-unit cell overlapping a section-5 rectangle | translated | medium | 608 | `80083870_FUN_80083870.c` |
| `0x80088100` | [`bg_unit_priority_check`] | compare a BG-unit record's height against its limit | translated | medium | 264 | `80088100_FUN_80088100.c` |
| `0x80088970` | [`stage_actor_place_effect`] | place the effect of one stage actor record | translated | medium | 440 | `80088970_FUN_80088970.c` |
| `0x80088B30` | [`stage_actor_mirror_effect`] | probe the camera and spawn the mirror/impact effect | translated | medium | 436 | `80088b30_FUN_80088b30.c` |
| `0x80088E80` | [`stage_aux_object_animate`] | advance the auxiliary object's two-phase animation | translated | medium | 328 | `80088e80_FUN_80088e80.c` |
| `0x800892F0` | [`stage_capture_setup`] | build the course capture manager and its subobjects | translated | medium | 2056 | `800892f0_FUN_800892f0.c` |
| `0x8008A380` | [`stage_capture_render`] | submit the split-screen capture viewport | translated | medium | 604 | `8008a380_FUN_8008a380.c` |
| `0x8008AC60` | [`stage_block_render_apply`] | build and apply the stage block's two render matrices | translated | medium | 684 | `8008ac60_FUN_8008ac60.c` |
| `0x80090600` | [`stage_big_renga_block_setup`] | load the big-renga block models for the stage type | translated | medium | 464 | `80090600_FUN_80090600.c` |
| `0x80091040` | [`stage_block_model_destroy`] | unlink a block model and free it when sized | translated | high | 116 | `80091040_FUN_80091040.c` |
| `0x800910C0` | [`stage_block_model_free`] | free a block model when the size is positive | translated | high | 64 | `800910c0_FUN_800910c0.c` |
| `0x80091100` | [`stage_block_four_segment_construct`] | initialise the four-segment block from its origin | translated | high | 108 | `80091100_FUN_80091100.c` |
| `0x80092D50` | [`stage_block_bind_buffer_store`] | create a block buffer and store one extra word | translated | medium | 68 | `80092d50_FUN_80092d50.c` |
| `0x800931F0` | [`stage_effect_pool_init`] | create the fourteen-entry effect pool | translated | medium | 304 | `800931f0_FUN_800931f0.c` |
| `0x80093320` | [`stage_effect_entry_pool`] | allocate effect entries in the global manager | translated | high | 160 | `80093320_FUN_80093320.c` |
| `0x800933C0` | [`stage_effect_grid_reset`] | program the effect grid with half-screen sizes | translated | high | 116 | `800933c0_FUN_800933c0.c` |
| `0x80094370` | [`stage_effect_coinkira_remove`] | remove the coin-sparkle effect | translated | high | 28 | `80094370_FUN_80094370.c` |
| `0x80094390` | [`stage_effect_bluecoinkira_remove`] | remove the blue coin-sparkle effect | translated | high | 28 | `80094390_FUN_80094390.c` |
| `0x800943B0` | [`stage_effect_icemisshit_remove`] | remove the ice-miss-hit effect | translated | high | 28 | `800943b0_FUN_800943b0.c` |
| `0x800943D0` | [`stage_effect_vshit_remove`] | remove the v-shit enemy effect | translated | high | 28 | `800943d0_FUN_800943d0.c` |
| `0x800943F0` | [`stage_effect_quicksand_remove`] | remove the quicksand effect | translated | high | 36 | `800943f0_FUN_800943f0.c` |
| `0x80094560` | [`stage_effect_iceball_remove`] | remove both iceball effects | translated | high | 112 | `80094560_FUN_80094560.c` |
| `0x80094660` | [`stage_effect_screen_matrix`] | build the 12-word effect quad matrix | translated | medium | 648 | `80094660_FUN_80094660.c` |
| `0x80094DE0` | [`stage_effect_entry_destroy`] | destroy one effect entry and optionally free it | translated | high | 88 | `80094de0_FUN_80094de0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. BG-scene edge snap (`0x80080900`)

`point` is six floats: the current triple at `+0`, the result triple at
`+0xC`. The area descriptor at `area` selects the mode through
`+0xC8`. Mode `1` builds a three-float anchor from the two integer fields at
`+0x58`/`+0x5C` plus `point.z`, calls `0x802394E0` for the segment parameter,
moves the result triple along the segment, and writes the angle
`atan2(dx, -dy) - 0x8000`. Every other mode builds four corner triples from the
eight integer fields at `area+0x60..0x7C` (all z = `point.z`), tests the four
edges with `0x802390D0`, snaps to the first edge closer than `1.0f`, and writes
`atan2(dx, -dy) + 0x4000`. `area+0xBC` points at a `u16` whose negated value
selects the one or two skipped edges when `mode == 1`; `point.y < point[4]`
flips that index by two.

### B. BG-unit collision and priority (`0x80083870`, `0x80088100`)

`0x80083870` rebuilds the model tile tables, then walks the 2048-cell grid
(64 × 32 cells of 16 units) against stage-section 5 (`+0x2C` pointer,
`+0x9C` count, 12 `u16` values = `0x18` bytes per entry). The low nibble of the
entry width/height adds one unit before the rectangle test. A hit allocates that
cell's palette through `0x800837D0`. The final `+0xC28` byte is the negated result of
`0x80081BE0(2,0x18,1)`, i.e. "this is not world 2 / stage 0x18 / area 1".

`0x80088100` compares `record+4` against `record+8`. Kind `3` at `+0xD` uses
the sub-kind at `+0xC`; all other kinds with a non-zero `+0xE` compare the two
heights directly, adding the `float` at `0x8042BF48 + byte[*record+0x348]*4`
when the owner's `+0x1090` byte is zero and the sub-kind is `2`. In game mode
`1` both heights are first passed through `0x80101AA0`.

### C. Stage actor helpers (`0x80088970..0x80088E80`)

`0x80088970` reuses its 16-byte effect record as the input block of
`0x80087F40`/`0x80088030`. The shape selector is `record+0xC` (`0` -> shape
`3`, `2` -> `0x802F0050[record[8]]`); shape `4` takes the extended
`0x80077860` camera path with type `0xB`.

`0x80088B30` rounds `record.x`, negates `record.y`, probes the camera through
`0x80070BA0`/`0x80070BF0`, and spawns effect `0x197` (class `+8 == 4`) or
`0x191|9` with the shape bits from `+9` (`-1` maps to `3`).

`0x80088E80` keeps two `s16` phases at `+0x18`/`+0x38` with steps
`+0x26`/`+0x56`, a count/index pair at `+0x3A`/`+0x3B`, and a speed at `+0x44`.
The first phase advances through `0x800DF890`, `+0x30` is smoothed by
`0x8015F140`, and `0x80237BD0` supplies the two sine values written to
`+0x2C`/`+0x34`.

### D. Course capture (`0x800892F0`, `0x8008A380`, `0x8008AC60`)

`0x800892F0` builds the 0xA0-byte capture manager: an embedded 0x4000-byte
allocator at `+4`, two effect subobjects at `+0x20`/`+0x98`, two stage blocks
at `+0x24`/`+0x28`, spawn blocks at `+0x34`/`+0x38`, a 0x90-byte capture
object at `+0x3C`, twelve effect wrappers at `+0x40`, and two depth buffers at
`+0x70`/`+0x74`. The depth-of-field file is selected by
`stage_type_code` (`0x80089030`): Sabaku, Kaigan, Water, Fire2, Koopa, with
type `0` using `7750.0f` and the camera pointer instead. The second block set
(`+0x80..+0x94`) only exists when `is_current_stage_unit(1,0x2A,1)` is true.

`0x8008A380` converts the screen size into the capture viewport: the 16:9 path
scales the width by `832.0f` and uses `0x1C8` as the height, the normal path
copies width/height, and split modes `1`/`2` halve the appropriate edges before
`0x801C5960`/`0x801C59D0` are emitted.

`0x8008AC60` snapshots the camera through `0x8008A230`, builds the projection
matrix from the signed screen x, the viewport height plus `32.0f`, `3600.0f`
and `20000.0f`, then builds the colour matrix from the BSS vectors at
`0x80377DC8`/`0x80377DE0`/`0x80377DEC`. The two words at `0x8042BFE0` are
selected by `this+0x24` and handed to the block GX state; the same colour
matrix feeds `0x8008A780`/`0x8008A8E0` before the camera is restored.

### E. Stage block model helpers (`0x80090600`, `0x80091040..0x80092D50`)

`0x80090600` is the big-renga factory. The stage type at `+0x2D8` indexes the
name tables `0x80310FF8`/`0x803110B8`, the count is at `+0x2D0`, models are
built at `+0x80` (stride `0x40`) and meshes at `+0x180` (stride `0x2C`).
Types `7..9` additionally load `boss_kameck_block_parts` and select the model
scale `2.0f`, `1.0f` or `0.0f`.

`0x80091040`/`0x800910C0` are the two model deleters: the first unlinks the
object from the intrusive list rooted 100 bytes into its key object, the second
only frees a positive-sized allocation. `0x80091100` initialises a
four-segment block (`1.0f` velocities, `0xF0` angle limit, `-0.1875f` and
`-4.0f`) and calls the three wave-29 helpers. `0x80092D50` creates a render
buffer through `0x80092C00` and stores the untouched `r7` word at `+8`.

### F. Stage effects (`0x800931F0..0x80094DE0`)

`0x800931F0` creates the fourteen-entry effect pool (0x20-byte entries from
`0x802DA310`), registers the four stage effect names, and posts the effect draw
object through `0x80169290`. `0x80093320` is the small entry pool (0x18-byte
entries, vtable `0x80311B7C`). `0x800933C0` reprograms the effect grid with the
half-size screen dimensions.

The six removal wrappers all forward one argument to `0x8016D8E0` or
`0x8016D640` with a fixed handle slot and, for quicksand/iceball, the effect
name. They are kept separate because each slot is a distinct global address.

`0x80094660` builds the effect quad: without a BG scene it is the full screen
(half width/height, `-0.5..0.5`); with a scene it narrows the edges by
`0.11403508f`, or scales through the letterbox object at `r13-0x47B0`.
`0x80094DE0` destroys one effect entry through `0x802D7B20` and optionally
frees it.

## Uncertainties carried forward

- `0x80080900`: Ghidra prints the anchor and corner reads as integer
  conversions (`(float)piVar10[...]`), but the shipped instructions are
  `lfs 0x58(r3)`/`lfs 0x5C(r3)` and eight `lfs` loads at `area+0x60..0x7C`, so
  the Rust reads them as `f32`. The psq_l/GQR guards around the point vector
  are treated as load-format artefacts, as in wave 29.
- `0x80083870`: `0x800B3100` is declared with four pointers but the call site
  passes an extra `f1`; that value is ignored by the helper and is therefore
  omitted. The section-5 entry geometry (width/height in units of 16) is
  inferred from the shift by four.
- `0x80088100`: the limit table at `0x8042BF48` starts with
  `2.0f, -2.0f, 176.0f, …`; the table is indexed by a flexible object byte, so
  only the first entries are known to be floats.
- `0x80088970` / `0x80088B30`: the two effect spawners were translated by
  wave 23; their record layout is inferred from those bodies. The
  `0x80070BA0`/`0x80070BF0` result words are read from `r3` although Ghidra
  types the probes as `void`.
- `0x800892F0`: `0x800E28A0` returns a 64-bit value; the float conversion of
  its low word and the two `0x80169290` arguments (`r10` and the high word) are
  unresolved. The `0x802C3630` constructor is called with the new object in
  `r3` although Ghidra prints no parameters.
- `0x8008A380`: the split parameter is `param_2` (the low half returned by the
  register-save helper); its values `1`/`2` are the only ones tested.
- `0x8008AC60`: the three vectors at `0x80377DC8`/`0x80377DE0`/`0x80377DEC`
  are BSS and initialised outside the lane; their values are read at runtime.
  The third argument of `0x8008A9C0` is a matrix pointer in the original call
  even though the callee passes it to a GX layer call.
- `0x80092D50`: the four forwarded arguments are reconstructed from
  `0x80092C00`'s signature because the wrapper is printed with no parameters.
- `0x800931F0`: the `r10` word forwarded to `0x80169290` is whatever the caller
  left in the register; it is exposed as an explicit parameter. The fourteen
  0x20-byte entries and their `0x802DA310` handles are otherwise literal.
- `0x80094660`: the letterbox fields `+8`/`+0xC`/`+0x28`/`+0x2C` are inferred
  from the arithmetic; the three BSS colour vectors are read at runtime.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus `# Safety`.
- Loads/stores go through `crate::platform` (and the module's `read_f32` /
  `write_f32` wrappers) so big-endian semantics do not depend on the host.
- Calls outside the batch are absolute-address calls with the original
  argument registers; none was "simplified" away.
- In-batch calls are direct Rust calls: [`bg_scene_edge_snap`] uses the
  existing `0x801611C0` helper, [`bg_unit_collision_scan`] calls
  [`bg_unit_model_resolve_names`]/[`bg_unit_model_clear`]/
  [`bg_unit_model_alloc_palette`], [`stage_actor_place_effect`] calls
  [`bg_unit_effect_spawn_a`]/[`bg_unit_effect_spawn_b`],
  [`stage_block_four_segment_construct`] calls the three wave-29 four-segment
  helpers, [`stage_capture_setup`] calls
  [`stage_block_bind_pair`]/[`stage_block_spawn`]/[`stage_block_init`]/
  [`stage_block_resources_ready`]/[`stage_block_bind_resource`]/
  [`is_current_stage_unit`]/[`bg_camera_clip_depth`]/[`stage_type_code`], and
  [`stage_block_render_apply`] calls [`stage_camera_submit`]/
  [`stage_block_gx_state`]/[`stage_block_actor_setup`]/
  [`stage_block_fill_rect`]/[`stage_camera_apply`].
- `cargo check` passes; it reports no diagnostics for `src/game/stage.rs` (the
  crate keeps 11 pre-existing warnings in `runtime/bt.rs` and
  `runtime/input.rs`).
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
