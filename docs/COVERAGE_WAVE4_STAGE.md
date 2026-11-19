# Coverage: wave 4 stage/course band (`0x8006E000..0x800A8000`)

## How to read this file

This document covers the wave-4 translation batch for the stage/course band of
`main.dol`. All 25 new functions live in `src/game/stage.rs`; no other Rust file
was changed.

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** — the Rust function in `src/game/stage.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present, with
  calls outside the batch left as absolute-address calls;
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a role, object type or hidden argument is
  inferred from call sites;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the
Ghidra name, address, size and evidence path.

The batch was selected by walking `ledger/functions.csv` and the function table
`work/ghidra/main-gb/export/functions.tsv` in `0x8006E000..0x800A8000`, then
reading the untranslated entries around the existing stage/course clusters.
Chosen clusters: the BG-unit model (`0x80083380..0x80083BD0`), the camera edge
tests and position-keyed list (`0x8008D3D0..0x8008DDB0`), the section-4B field
accessors and rectangle helpers (`0x8008E530..0x8008EAF0`), and the stage-object
teardown (`0x8008EBE0`).

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, any
`mod.rs`, `Cargo.toml`, `platform.rs`, or another lane's files.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this batch | 25 |
| Bytes of Ghidra function size | 2,876 |
| `translated` | 25 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 18 |
| Confidence `medium` | 7 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 25 |

## BG-unit model and tile map (`0x80083380..0x80083BD0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80083380` | [`bg_tilemap_set_at`] | write a tile by absolute coordinate, returning the old value | translated | high | `80083380_FUN_80083380.c` |
| `0x80083560` | [`bg_unit_model_create`] | allocate or refresh a BG-unit model | translated | medium | `80083560_FUN_80083560.c` |
| `0x80083660` | [`bg_unit_model_clear`] | clear the model's tile-id and pointer tables | translated | high | `80083660_FUN_80083660.c` |
| `0x800837D0` | [`bg_unit_model_alloc_palette`] | allocate and fill one 256-entry palette block | translated | high | `800837d0_FUN_800837d0.c` |
| `0x80083AD0` | [`bg_unit_model_bind_heap`] | attach or swap the model's buffer heap | translated | high | `80083ad0_FUN_80083ad0.c` |
| `0x80083B40` | [`bg_unit_model_tile_at`] | locate one 2-byte tile cell in a model | translated | high | `80083b40_FUN_80083b40.c` |
| `0x80083BD0` | [`bg_unit_model_resolve_names`] | resolve three unit-name strings to table indices | translated | medium | `80083bd0_FUN_80083bd0.c` |
| `0x80083F90` | [`bg_unit_model_tile_index`] | map a world tile coordinate to a tile-table index | translated | high | `80083f90_FUN_80083f90.c` |

## Camera edges and position-keyed list (`0x8008D3D0..0x8008DDB0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008D3D0` | [`segment_side_x`] | side of an x-ordered 2-D segment | translated | medium | `8008d3d0_FUN_8008d3d0.c` |
| `0x8008D820` | [`segment_side_y`] | side of a y-ordered 2-D segment | translated | medium | `8008d820_FUN_8008d820.c` |
| `0x8008DCF0` | [`spatial_list_release`] | free every entry of the 0x1C-byte list | translated | high | `8008dcf0_FUN_8008dcf0.c` |
| `0x8008DDB0` | [`spatial_entry_store`] | insert or update a position-keyed list entry | translated | medium | `8008ddb0_FUN_8008ddb0.c` |

## Course-section accessors (`0x8008E530..0x8008EAF0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008E530` | [`section4b_byte_e`] | byte `+0xE` of a section-4B entry | translated | high | `8008e530_FUN_8008e530.c` |
| `0x8008E560` | [`section4b_byte_f`] | byte `+0xF` of a section-4B entry | translated | high | `8008e560_FUN_8008e560.c` |
| `0x8008E590` | [`section4b_byte_10`] | byte `+0x10` of a section-4B entry | translated | high | `8008e590_FUN_8008e590.c` |
| `0x8008E5C0` | [`section4b_byte_16`] | byte `+0x16` of a section-4B entry | translated | high | `8008e5c0_FUN_8008e5c0.c` |
| `0x8008E5F0` | [`section4b_byte_17`] | byte `+0x17` of a section-4B entry | translated | high | `8008e5f0_FUN_8008e5f0.c` |
| `0x8008E620` | [`section4b_byte_11`] | byte `+0x11` of a section-4B entry | translated | high | `8008e620_FUN_8008e620.c` |
| `0x8008E6B0` | [`section4b_u16_8`] | `u16` at `+8` of a section-4B entry | translated | high | `8008e6b0_FUN_8008e6b0.c` |
| `0x8008E710` | [`section4b_byte_14`] | byte `+0x14` of a section-4B entry | translated | high | `8008e710_FUN_8008e710.c` |
| `0x8008E740` | [`section4b_u16_4`] | `u16` at `+4` of a section-4B entry | translated | high | `8008e740_FUN_8008e740.c` |
| `0x8008E950` | [`rect_contains_point`] | point-in-rectangle test with the 0x80 margin | translated | medium | `8008e950_FUN_8008e950.c` |
| `0x8008EA20` | [`rect_overlaps`] | centre-distance rectangle overlap test | translated | high | `8008ea20_FUN_8008ea20.c` |
| `0x8008EAF0` | [`section5b_entry_by_index`] | `0x14`-byte section-5B entry by index | translated | medium | `8008eaf0_FUN_8008eaf0.c` |

## Stage-object teardown (`0x8008EBE0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008EBE0` | [`stage_objects_release`] | destroy and free the four stage objects | translated | high | `8008ebe0_FUN_8008ebe0.c` |

## Recovered data model

The 0xC2C-byte BG-unit model (constructed by `0x80083560`):

| offset | meaning | evidence |
|---|---|---|
| `+0x00` | vtable `0x80310910` | `0x80083560` |
| `+0x04` | 256-entry pointer array, one block per tile index | `0x800837D0`, `0x80083B40` |
| `+0x408` | 0x800-byte tile → palette-block byte table (0x800 entries) | `0x80083660`, `0x80083F90` |
| `+0xC08` | `u16` palette-block count | `0x80083660`, `0x800837D0` |
| `+0xC0C` | first name-family index, initialised to `-1` | `0x80083BD0` |
| `+0xC10` | terrain ("chika") name index | `0x80083BD0` |
| `+0xC14` | ice ("kori") name index | `0x80083BD0` |
| `+0xC18` | rail name index | `0x80083BD0` |
| `+0xC1C` | unit slot | `0x80083560` |
| `+0xC20` | stage slot | `0x80083560` |
| `+0xC24` | bound buffer/heap | `0x80083560`, `0x80083AD0` |

The position-keyed list at `r13-0x5808` = **`0x80429E78`** stores a `{head,
tail}` pair. Nodes are 0x1C bytes:

| offset | meaning | evidence |
|---|---|---|
| `+0x00` | previous node | `0x80160FE0` / `0x80160F60` set it |
| `+0x04` | next node | same |
| `+0x08` | self-pointer, used as the free target | `0x8008DDB0`, `0x8008DCF0` |
| `+0x0C` | `f32` x key | `0x8008DDB0` |
| `+0x10` | `f32` y key | `0x8008DDB0` |
| `+0x14` | `f32` z payload | `0x8008DDB0` |
| `+0x18` | `u16` value | `0x8008DDB0` |
| `+0x1A` | key byte | `0x8008DDB0` |

`0x8008EAF0` is section 5B: base `+0x34`, row count `+0xA4`, stride `0x14`.
That naming follows the seven pointer/count pairs already documented for
`parse_course_file`; it is a neutral label, not a recovered Nintendo name.

## Uncertainties carried forward

- `0x80083560`: the allocator object and its virtual slot `+0x28` are inferred
  from the call shape; the `-0x100` subtraction on the resource size is
  observed but its purpose is unknown. `0x80083870` (rebuild, 608 bytes) and
  `0x80083D00` (upload, 284 bytes) are called by address and were not
  translated in this batch.
- `0x80083560`/`0x80083BD0`: Ghidra prints a `void` signature because the
  Metrowerks prologue helpers hide `r3`; the helpers preserve `r3`, and the
  call sites pass the model there.
- `0x80083BD0`: the three name families are read directly from the Ghidra
  string labels (`Pa1_chika`, `Pa2_kori`, `Pa3_rail`); their gameplay meaning
  is unknown. `+0xC0C` starts at `-1` and is not written again in this function.
- `0x8008D3D0`/`0x8008D820`: the arithmetic and the `-1`/`2`/`3` results are
  observed; "segment side" is inferred from the four call sites in
  `0x8008D460`/`0x8008D8B0`, which pass pairs of region corners. Those two
  callers are large paired-single functions and remain untranslated.
- `0x8008DCF0`/`0x8008DDB0`: the list layout is certain, but what the list is
  for is unknown. The names describe the structure only. The original's first
  argument to `0x8008DDB0` is never read and is kept as `_unused`.
- `0x8008E950`/`0x8008EA20`: the rectangle fields are inferred to be
  `[x, y, width, height]`; the `0x80` margin unit is unknown. `0x8008E950`
  wraps x through `0x80101AA0` (outside this lane) and reproduces the original
  sign-bit expression literally rather than simplifying it.
- All nine `0x8008E530..0x8008E740` accessors inherit the original's missing
  null check: if no section-4B entry matches, they read through address
  `0xE`/`0xF`/etc.
- `0x8008EAF0`: section-5B field roles beyond the pointer/count/stride are
  unknown.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus, where relevant, `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original argument
  registers; none was "simplified" away.
- The section-4B accessors call the already-promoted
  `crate::resource::course::section4b_entry_by_id` rather than duplicating the
  scan.
- `cargo check` passes and `node tools/doc-lint.mjs src` passes (68 files,
  0 violations) at the end of this batch. Other lanes were not edited.
