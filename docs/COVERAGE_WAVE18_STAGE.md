# Coverage — wave 18 stage band (`0x80080000..0x800C0000`)

## How to read this file

This document records the wave-18 batch of the stage/course lane. All 29 new
bodies live in `src/game/stage.rs`; no other Rust file was changed. The batch
continues the main.dol band `0x80080000..0x800C0000` after the wave-15 stage
pass and fills four clusters that still had no Rust body.

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets); written with the `0x` prefix
  here and in the Rust docs, plain hex in the CSV;
- **name** — the Rust function in `src/game/stage.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field meaning, an unrecovered indirect call or
  a register-split is inferred from call sites or surrounding code;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the
Ghidra name, address, size and evidence path.

The only files written by this pass are `src/game/stage.rs`, this document and
`ledger/agents/wave18-stage.csv`. `lib.rs`, every `mod.rs`, `Cargo.toml`,
`platform.rs`, generated documents and other lanes' files were not touched.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 29 |
| Bytes of Ghidra function size | 4,444 |
| `translated` | 29 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 17 |
| Confidence `medium` | 12 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 29 |
| New staging modules | 0 |

The selection was made by walking
`work/ghidra/main-gb/export/functions.tsv` over `0x80080000..0x800C0000`,
subtracting every address already in `ledger/functions.csv`, and then checking
each candidate against the destination file. Every address below was absent
from both. The remaining untranslated functions in the band are either other
lanes' subjects (actors, bosses, world map, save/scene engine) or paired-single
bodies that need an instruction-level pass (for example `0x8008D460` /
`0x8008D8B0`, `0x800892F0`).

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---:|---:|---:|---|
| `0x80080170` | [`bg_scene_follow_layers`] | advance every scroll layer and clear the two accumulators | translated | medium | 288 | `80080170_FUN_80080170.c` |
| `0x800803F0` | [`bg_scene_layer_add_a`] | insert a handle into the scene's `+0x18` four-slot array | translated | high | 156 | `800803f0_FUN_800803f0.c` |
| `0x80080530` | [`bg_scene_layer_add_b`] | insert a handle into the scene's `+0x28` four-slot array | translated | high | 156 | `80080530_FUN_80080530.c` |
| `0x800805D0` | [`bg_scene_layer_clear`] | clear the per-layer handles and one camera/tile word | translated | high | 88 | `800805d0_FUN_800805d0.c` |
| `0x80080880` | [`bg_scene_span`] | test whether a biased span covers `[lo, hi]` | translated | medium | 128 | `80080880_FUN_80080880.c` |
| `0x80080D00` | [`bg_scene_layer_mode_a`] | flag/vtable gate for a BG layer | translated | medium | 52 | `80080d00_FUN_80080d00.c` |
| `0x80080D40` | [`bg_scene_layer_mode_b`] | same gate with the `0x100000` flag and `+0x50` callback | translated | medium | 52 | `80080d40_FUN_80080d40.c` |
| `0x80080D80` | [`bg_scene_layer_mode_c`] | same gate with the `0x200000` flag and `+0x54` callback | translated | medium | 52 | `80080d80_FUN_80080d80.c` |
| `0x80080E40` | [`bg_scene_object_filter`] | full collision-filter chain for a candidate stage object | translated | medium | 484 | `80080e40_FUN_80080e40.c` |
| `0x80081030` | [`bg_scene_scroll_allowed`] | reject the two flag combinations that disable scrolling | translated | high | 52 | `80081030_FUN_80081030.c` |
| `0x80089ED0` | [`stage_camera_defaults`] | compute the camera extent and install one-shot camera defaults | translated | medium | 308 | `80089ed0_FUN_80089ed0.c` |
| `0x8008A230` | [`stage_camera_submit`] | submit the projection and viewport pointers to GX | translated | high | 72 | `8008a230_FUN_8008a230.c` |
| `0x8008A280` | [`stage_camera_apply`] | submit six view floats and four viewport words to GX | translated | high | 92 | `8008a280_FUN_8008a280.c` |
| `0x8008A8E0` | [`stage_block_fill_rect`] | draw the stage block's filled rectangle | translated | high | 224 | `8008a8e0_FUN_8008a8e0.c` |
| `0x8008AF10` | [`stage_block_texture_init`] | allocate and initialise the block texture descriptor | translated | medium | 108 | `8008af10_FUN_8008af10.c` |
| `0x8008AF80` | [`stage_block_coin_models_load`] | load the selected coin BRRES and bind its model | translated | high | 168 | `8008af80_FUN_8008af80.c` |
| `0x8008B830` | [`stage_render_mode_a`] | program the first stage render-mode GX state block | translated | medium | 452 | `8008b830_FUN_8008b830.c` |
| `0x8008BA00` | [`stage_render_mode_b`] | program the second stage render-mode GX state block | translated | medium | 456 | `8008ba00_FUN_8008ba00.c` |
| `0x8008F810` | [`ddyl_path_delete`] | release a dDyl path record and optionally free it | translated | high | 88 | `8008f810_FUN_8008f810.c` |
| `0x8008F920` | [`ddyl_path_build`] | build a dDyl resource path from a source name | translated | high | 204 | `8008f920_FUN_8008f920.c` |
| `0x8008F9F0` | [`ddyl_path_invoke`] | build a path and forward it to the dDyl loader | translated | high | 108 | `8008f9f0_FUN_8008f9f0.c` |
| `0x8008FA60` | [`ddyl_path_vtable_call`] | build a path and invoke the owner's `+0xA0` method | translated | medium | 116 | `8008fa60_FUN_8008fa60.c` |
| `0x8008FAE0` | [`ddyl_path_vtable_call_b`] | twin of the `+0xA0` path call | translated | medium | 116 | `8008fae0_FUN_8008fae0.c` |
| `0x8008FB60` | [`ddyl_loader_call`] | set the current-path id around a loader call | translated | high | 64 | `8008fb60_FUN_8008fb60.c` |
| `0x8008FBA0` | [`ddyl_loader_vtable_call`] | set the current-path id around a `+0xA0` call | translated | high | 80 | `8008fba0_FUN_8008fba0.c` |
| `0x8008FBF0` | [`ddyl_loader_vtable_call_b`] | twin of the current-path `+0xA0` call | translated | high | 80 | `8008fbf0_FUN_8008fbf0.c` |
| `0x8008FC40` | [`ddyl_loader_path_call`] | set the current-path id around [`ddyl_path_invoke`] | translated | high | 64 | `8008fc40_FUN_8008fc40.c` |
| `0x8008FC80` | [`ddyl_loader_path_vtable_call`] | set the current-path id around [`ddyl_path_vtable_call`] | translated | high | 68 | `8008fc80_FUN_8008fc80.c` |
| `0x8008FCD0` | [`ddyl_loader_path_vtable_call_b`] | set the current-path id around [`ddyl_path_vtable_call_b`] | translated | high | 68 | `8008fcd0_FUN_8008fcd0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### A. BG-scene scroll layers (`0x80080170..0x80081030`)

The scene passed to these functions is the BG manager documented at the top of
`src/game/stage.rs`. Its first word points at the camera owner; the camera
position is at owner `+0xAC..+0xB4`, matching the collision-area edge accessors.

`0x80080170` walks three collections in order: the `+0x6C`-linked list rooted
at scene `+0x0C` with selector `0`, the four handles at scene `+0x18..+0x24`
with selector `1`, and the four handles at `+0x28..+0x34` with selector `2`.
Every entry is passed to `0x8007FFA0(scene, layer, &position, selector)`. The
local position is `camera - origin + accumulator` for x/y and `camera - origin`
for z. The two accumulators at `+0xB0`/`+0xB4` are then cleared from the
`r2-0x71F8` zero.

`0x800803F0` and `0x80080530` are the two insertion helpers for those
four-slot arrays. They return `1` for "already present or inserted" and `0`
when the array is full, exactly like the unrolled C.

`0x80080880` and `0x80080E40` are the two predicates. The first is a numeric
span test driven by the `u16` at `*(owner+0xBC)` and the two `.rodata` offset
tables `0x802EF8C0`/`0x802EF8D0`. The second is the long collision-filter chain
that combines the `+0xE5`/`+0xDD` lane mask, the owner/parent/`+0x54`
exclusions, the `+0xE8` family byte, two per-lane mask tables and the layer
mode at `+0xCC`.

`0x80080D00`, `0x80080D40` and `0x80080D80` are three copies of the same
flag-or-callback gate with masks `0x80000`/`0x100000`/`0x200000` and callback
slots `+0x4C`/`+0x50`/`+0x54`. Ghidra could not recover the indirect jumps, so
the Rust calls the slot with the object's first word, which is the only
argument the original has in `r3`.

### B. Stage camera (`0x80089ED0..0x8008A280`)

`0x80089ED0` computes the camera extent from the two `f64` values at
`0x8042BF90`/`0x8042BF88` and the `f32` at `0x8042BF98`, stores it at
`0x80429E20`, and on the first call fills the camera object at `0x80310A88`.
The first-call flag is the byte at `0x80429E28`.

`0x8008A230` and `0x8008A280` are the two GX submission helpers. The first
passes three camera field addresses to `0x801C9C40`/`0x801C9CD0`; the second
loads six floats at `+0x1C..+0x30` and four words at `+0x34..+0x40` into
`0x801C9C10`/`0x801C9C60`. Both are thin wrappers, but they are the only
places where the stage camera object is handed to GX in this band.

### C. Stage block rendering and coin models (`0x8008A8E0..0x8008BA00`)

`0x8008A8E0` draws the block's filled rectangle: the texture handle is at
`+0x14`, the `u16` size at `+0x18`, and the two float coordinates at
`+0x1C`/`+0x20` are truncated to 16 bits before the GX rect call.

`0x8008AF10` allocates the block's texture object from the same `u16` size and
stores the 0x20-byte descriptor at `+0x14`. `0x8008AF80` loads the coin model
selected by `+0x24`: the three overlapping pointer arrays at `0x802F00D0` are
one `{name, brres, model}` triple per index, and the model is bound to the
J3D object at `+0x08`.

`0x8008B830` and `0x8008BA00` are the two stage render-mode GX state blocks.
They differ in exactly three selector words and in the final value passed to
`0x801C8430` (`0xFFFFFF40` versus `0xFF`). The dispatcher at `0x8008BBD0`
chooses between them from the model name (`MH_model`/`SMH_model`, …), so the
Rust names stay neutral: `stage_render_mode_a` and `stage_render_mode_b`.

### D. dDyl resource paths (`0x8008F810..0x8008FCD0`)

The dDyl path object is a temporary record: `0x8008F810` releases it with
`0x80160110(this, 0)` and frees it when the size is positive.

`0x8008F920` is the shared path builder. It first does
`0x802E19F8(dst, src, 0x1E)` (a padded `strncpy`), then, depending on the game
mode returned by `0x8005FFB0`, replaces the first six bytes with `"Wm_01_"`,
`"Wm_02_"` or `"Wm_03_"` (at `0x80427A10`/`18`/`20`) and copies the 0x18-byte
tail from `src+3` with `0x80004364` (the game's `memmove`). The three prefixes
were read directly from the shipped image.

`0x8008F9F0` builds a 40-byte path and calls `0x8016C890(path, a2..a5)`.
`0x8008FA60`/`0x8008FAE0` build a 56-byte path and call the owner's `+0xA0`
virtual method with `(owner, path, a3..a6)`. The remaining six functions set
the "current path id" at `0x80427A38` to their first argument, run one of those
calls, and restore the global to `-1`.

## Uncertainties carried forward

- `0x80080170`: the C applies `(float)` to integer fields at `+0x94..+0x9C` and
  `+0xB0`/`+0xB4`; the Rust keeps the signed-int-to-float conversions. The
  camera-owner/-layer naming is inferred from the matching collision-area
  accessors.
- `0x80080880`: the two `.rodata` tables at `0x802EF8C0`/`0x802EF8D0` are
  copied as raw `int` offsets; their units and the `+0x60` object fields are
  unknown.
- `0x80080D00`/`0x80080D40`/`0x80080D80`: Ghidra reports "could not recover
  jumptable"; the single argument is taken from the unchanged `r3`.
- `0x80080E40`: the per-lane masks at `0x802EF8E0`/`0x802EF8F0` and the meaning
  of each bit in `+0xD0`/`flags` are inferred from the call shape.
- `0x80089ED0`: the two `f64` loads are paired-single loads that Ghidra
  widened; the Rust reads the same eight bytes through a local `read_f64`.
  The camera field meanings are inferred.
- `0x8008AF10`: the roles of `0x801C6BA0`, `0x802B9260` and `0x8000EFC0` are
  inferred from the GX texture-object shape (`tex+0x20` and the 0x20-byte
  descriptor).
- `0x8008B830`/`0x8008BA00`: the GX state names behind the twenty-three calls
  are unknown; the two bodies are reproduced call-for-call.
- `0x8008F920`: `0x8005FFB0` is named `GAME_MODE_CODE` from the three
  comparisons only. The three prefix strings are observed ASCII
  (`Wm_01_`/`Wm_02_`/`Wm_03_`), but their meaning is not decoded.
- `0x8008FA60`/`0x8008FAE0`: the register split (`r3` = name, `r4` = owner) is
  recovered from the unchanged registers at the call site; the virtual slot's
  first argument is the owner.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original
  argument registers; none was "simplified" away.
- In-batch calls are direct Rust calls: [`ddyl_path_invoke`] →
  [`ddyl_path_build`], [`ddyl_loader_path_call`] → [`ddyl_path_invoke`],
  [`ddyl_loader_path_vtable_call`] → [`ddyl_path_vtable_call`],
  [`ddyl_loader_path_vtable_call_b`] → [`ddyl_path_vtable_call_b`].
- `cargo check` reports no diagnostics for `src/game/stage.rs`. At the time of
  this pass the crate still has two unrelated errors in `src/game/player.rs`
  (`W18_TEN` and `fn_80052500` unresolved) from another lane; they are not
  touched here.
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
