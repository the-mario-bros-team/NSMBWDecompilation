# Coverage: wave 15 stage band (`0x80080000..0x800C0000`)

## How to read this file

This document covers the wave-15 batch of the stage/course lane. All 27 new
bodies live in `src/game/stage.rs`; no other Rust file was changed.

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** — the Rust function in `src/game/stage.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present, with
  calls outside the batch left as absolute-address calls;
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field meaning, ABI detail or local descriptor
  is inferred from call sites or from the surrounding code;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the Ghidra
name, address, size and evidence path.

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, any
`mod.rs`, `Cargo.toml`, `platform.rs`, or another lane's files. The only files
written are `src/game/stage.rs`, this document and
`ledger/agents/wave15-stage.csv`.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this batch | 27 |
| Bytes of Ghidra function size | 7,256 |
| `translated` | 27 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 15 |
| Confidence `medium` | 12 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 27 |

The batch was selected by walking `work/ghidra/main-gb/export/functions.tsv`
over `0x80080000..0x800C0000` and crossing it with `ledger/functions.csv`. The
band is dominated by actors, bosses, the world map and the save/scene engine,
which belong to other lanes. The entries below are the remaining functions that
operate on the BG tile stream, the stage texture manager, the stage environment
object, the dDyl stage-resource table or the stage block object.

## BG tile pipeline (`0x80083D00..0x80086D10`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80083D00` | [`bg_unit_tile_layer_upload`] | walk a bgdat layer and hand each run to a tile-run builder | translated | medium | `80083d00_FUN_80083d00.c` |
| `0x80085780` | [`bg_tile_run_fill`] | build the runs of a layer that starts with a separator or `0xFF` | translated | medium | `80085780_FUN_80085780.c` |
| `0x800859F0` | [`bg_tile_run_fill_wide`] | build the runs of a layer from its `0x80..0x93` direction command | translated | medium | `800859f0_FUN_800859f0.c` |
| `0x80086D10` | [`bg_unit_tile_pick_four`] | pick a tile id that differs from all four neighbours | translated | medium | `80086d10_FUN_80086d10.c` |

`0x80083D00` reads the command stream from its second argument and the stage
from `model+0xC20`. It copies the last four command words into a local
pseudo-model descriptor at `+0xC16..+0xC1C` and calls one of the two run
builders, which read the descriptor exactly like the real model fields at
`+0xC00..+0xC1C`. The descriptor is a stack mirror; it is documented at the
call site rather than promoted to a Rust struct.

`0x80086D10` is the four-neighbour sibling of the already-translated
`0x80086EC0`/`0x80086FE0` tile pickers. The prologue helper is read as
`(r3, r4)`: `r3` is the palette owner and `r4` the tile model.

## Stage texture manager (`0x80087380..0x80087B60`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80087380` | [`stage_tex_manager_construct`] | install the `0x80310A18` vtable and construct the `+0x08` subobject | translated | high | `80087380_FUN_80087380.c` |
| `0x80087410` | [`stage_tex_manager_destroy`] | release the animation list and three slots, then free the manager | translated | medium | `80087410_FUN_80087410.c` |
| `0x80087510` | [`stage_tex_manager_init`] | load all BG textures and register the fixed texture animations | translated | medium | `80087510_FUN_80087510.c` |
| `0x800878E0` | [`stage_tex_anim_apply_all`] | walk the animation list inside the scene begin/end pair | translated | high | `800878e0_FUN_800878e0.c` |
| `0x80087A40` | [`stage_tex_anim_apply_entry`] | bind one animation entry to the scene | translated | high | `80087a40_FUN_80087a40.c` |
| `0x80087AF0` | [`stage_tex_anim_insert`] | allocate, initialise and prepend one 0x18-byte animation entry | translated | medium | `80087af0_FUN_80087af0.c` |
| `0x80087B60` | [`stage_tex_anim_add`] | load one BG animation file and insert it | translated | medium | `80087b60_FUN_80087b60.c` |

The manager is the object built at `stage+0x90A50` by `0x80077B40`; the
constructor installs the `0x80310A18` vtable and the animation list root is at
`+0x34`. The 0x18-byte entries are the same records translated as
[`stage_anim_entry_init`] in the wave-7 section.

`0x80087510` keeps the original `dBgTexMng::m_allocator` name and all six
belt-conveyor animation registrations. The first path name is built through a
format string that lives at `r13-0x7C84` and is filled at boot; the Rust passes
that address rather than inventing the string.

## Stage environment (`0x80088CF0..0x80089DC0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80088CF0` | [`stage_effect_position`] | move the camera to a wrapped tile and spawn effect `0x1D4` | translated | medium | `80088cf0_FUN_80088cf0.c` |
| `0x800890F0` | [`stage_env_gx_pair_a`] | restore the two GX state bits | translated | high | `800890f0_FUN_800890f0.c` |
| `0x80089120` | [`stage_env_gx_pair_b`] | second entry point with the same two GX calls | translated | high | `80089120_FUN_80089120.c` |
| `0x80089150` | [`stage_env_construct`] | install the `0x80310B70` vtable and clear the child slots | translated | high | `80089150_FUN_80089150.c` |
| `0x80089210` | [`stage_env_destroy`] | release two children and the `+0x04` subobject | translated | high | `80089210_FUN_80089210.c` |
| `0x80089B80` | [`stage_env_release_children`] | release every child object of the environment | translated | high | `80089b80_FUN_80089b80.c` |
| `0x80089DC0` | [`stage_env_subobject`] | return one child's `+0x14` word plus `0x20` | translated | high | `80089dc0_FUN_80089dc0.c` |

`0x80089B80` is the shutdown counterpart of the constructor: it invokes the
virtual `+0x14` destructor on the two primary children, the camera-only slots,
the twelve `+0x40` slots and the six tail slots guarded by `+0x94` and
`DAT_80310A91`. The null checks are exactly those of the original (the primary
pair is not checked).

## dDyl stage-resource readiness (`0x8008F460..0x8008F760`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008F460` | [`ddyl_resource_table_build`] | allocate the id table and build one record per descriptor | translated | high | `8008f460_FUN_8008f460.c` |
| `0x8008F640` | [`ddyl_resource_complete`] | finish the asynchronous resource load and set the ready word | translated | medium | `8008f640_FUN_8008f640.c` |
| `0x8008F760` | [`ddyl_resource_start`] | build the 0x2EE-slot table and start the async load | translated | high | `8008f760_FUN_8008f760.c` |

The table size is `count*0x10 + extra*0x48`; the shipped call passes
`count = 0x2EE` slots and `extra = 1` descriptor. The id array is
`count*4` bytes and is indexed by the descriptor's `u16` id. The outer loop is
over `extra`, not `count`; this was checked against the shipped instructions
(`0x8008F55C` compares the inner counter with `r13-0x57C4`, while
`0x8008F5B4` compares the outer counter with `r13-0x57CC`).

## Stage block object (`0x80090590..0x800919A0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80090590` | [`stage_block_extras_release`] | release the `+0x2A0` edge models of kinds 4/5 | translated | high | `80090590_FUN_80090590.c` |
| `0x800907D0` | [`stage_block_destroy`] | destroy four arrays, the `+0x60` subobject and the list link | translated | high | `800907d0_FUN_800907d0.c` |
| `0x800908B0` | [`stage_block_element_delete`] | deleting destructor for one array element | translated | medium | `800908b0_FUN_800908b0.c` |
| `0x800908F0` | [`stage_block_release`] | reinstall the vtable, unlink and optionally free | translated | high | `800908f0_FUN_800908f0.c` |
| `0x800912A0` | [`ice_piece_models_load`] | load the ice-piece BRRES and create four models | translated | medium | `800912a0_FUN_800912a0.c` |
| `0x800919A0` | [`wanwan_models_load`] | load `g3d_wanwan.brres` and create six models | translated | high | `800919a0_FUN_800919a0.c` |

The block object uses vtable `0x8031133C` and keeps its list node at `+0x54`.
The four arrays destroyed by `0x800907D0` are the same regions populated by
the block creation path at `0x80090600` (`+0x80`, `+0x180`, `+0x230`,
`+0x2A0`), which is not part of this batch. `0x800912A0` and `0x800919A0` are
the two model-loading variants used by the block variants.

## Uncertainties carried forward

- `0x80085780`: the first branch (the "empty layer" path) is only reachable
  when the stream's first byte is `0xFF`. The original compares the byte at the
  separator pointer, not the advanced index; the Rust keeps a comment next to
  that check.
- `0x800859F0`: the three scratch tables at `+0x400`/`+0x800` are inferred from
  the pointer writes in the second pass. The direction byte meanings are
  observed but not named.
- `0x80083D00`: the pseudo-model descriptor is a stack mirror. The low byte of
  the two-byte texel pair is loaded by the original but never consumed by the
  two run builders; it is reproduced as an ignored load.
- `0x80087380`/`0x80087410`: the three `.data` words at `0x80377DC8..D0` are
  copied verbatim; their meanings are unknown.
- `0x80087510`: the first path format string is filled at `r13-0x7C84` during
  boot and is not decoded here. `PTR_s_BG_tex_belt_conveyor_*` entries are
  pointer variables, so the Rust loads them with `read_u32`.
- `0x80087AF0`: the allocation-failure path dereferences null in the original;
  it is reproduced literally.
- `0x80088CF0`: the third float of the effect block comes from
  `r2-0x7148`; its value is unknown.
- `0x8008F640`: the leading-zero poll is an unbounded loop when the polled word
  is zero, exactly as printed; the object/allocator field types are opaque.
- `0x800912A0`/`0x800919A0`: `0x800690C0` takes the **value** at
  `0x80377C48` (loaded with `lwz` in the shipped code), not the address of the
  slot.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original
  argument registers; none was "simplified" away.
- In-batch calls are direct Rust calls: `bg_unit_tile_layer_upload` →
  `bg_tile_run_fill`/`bg_tile_run_fill_wide`, `stage_tex_manager_init` →
  `stage_tex_anim_add` → `stage_tex_anim_insert` →
  `stage_anim_entry_init`, `stage_tex_anim_apply_all` →
  `stage_tex_anim_apply_entry`, `stage_env_*` → `stage_env_subobject`,
  `ddyl_resource_start` → `ddyl_resource_table_build` and
  `ddyl_resource_complete`, and `stage_block_destroy` →
  `stage_block_element_delete`.
- `cargo check` reports **no diagnostics for `src/game/stage.rs`**. The rest of
  the tree keeps its 11 pre-existing warnings.
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
