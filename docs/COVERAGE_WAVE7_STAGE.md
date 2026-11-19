# Coverage: wave 7 stage band (`0x80070000..0x80090000`)

## How to read this file

This document covers the wave-7 batch for the stage/course band of `main.dol`.
All 29 new functions live in `src/game/stage.rs`; no other Rust file was
changed. The batch is a continuation of the wave-5 stage work
(`docs/COVERAGE_WAVE5_STAGE.md`) and of the course-resource lane
(`docs/COVERAGE_r80080_800c0.md`).

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** — the Rust function in `src/game/stage.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present, with
  calls outside the batch left as absolute-address calls; `partial` means the
  same, but at least one call cannot be reproduced exactly (here: the `r12`
  argument of `0x802DCBFC`);
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a role, register pairing or object type is
  inferred from call sites or from the instruction stream;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the
Ghidra name, address, size and evidence path.

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, any
`mod.rs`, `Cargo.toml`, `platform.rs`, or another lane's files. The only files
written are `src/game/stage.rs`, this document and
`ledger/agents/wave7-stage.csv`.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this batch | 29 |
| Bytes of Ghidra function size | 3,760 |
| `translated` | 28 |
| `partial` | 1 (`0x80086B20`) |
| `stub` | 0 |
| Confidence `high` | 23 |
| Confidence `medium` | 6 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 29 |

The batch was selected by walking `ledger/functions.csv` and
`work/ghidra/main-gb/export/functions.tsv` in `0x80070000..0x80090000`, then
reading the untranslated entries adjacent to the existing stage clusters. The
module header of `src/game/stage.rs` already reserved the BG-unit tile and dDyl
slot clusters for a "wave 6" batch; that batch was never written, so wave 7
lands those two clusters and three smaller stage groups:

1. BG-unit tile selection — `0x80085E40..0x80086FE0`;
2. camera depth selectors — `0x800872E0..0x80087360`;
3. BG-unit cell accessors — `0x80077470..0x80077490`;
4. stage animation list — `0x80087890..0x80087C30`;
5. stage helpers — `0x80088D90..0x80089030`;
6. dDyl resource slots — `0x8008F110..0x8008F870`.

## BG-unit tile selection (`0x80085E40..0x80086FE0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80085E40` | [`bg_unit_tile_style_id`] | mask the command to a tile id and remove the layer offset | translated | high | `80085e40_FUN_80085e40.c` |
| `0x80085E80` | [`bg_unit_tile_pick_style_a`] | id-family decision tree for the `0x10x`/`0x11x`/`0x12x`/`0x14x` tables | translated | high | `80085e80_FUN_80085e80.c` |
| `0x80085FF0` | [`bg_unit_tile_pick_style_b`] | id-family decision tree for the `0x10Ax`/`0x11Ax`/`0x12Ax` tables | translated | high | `80085ff0_FUN_80085ff0.c` |
| `0x80086A60` | [`bg_unit_tile_pick_style_c`] | adds the `0x1A0..0x1A5` and `0x1B0..0x1B5` families | translated | high | `80086a60_FUN_80086a60.c` |
| `0x80086B20` | [`bg_unit_tile_write_cell`] | method-record dispatch and the final cell write-back | partial | medium | `80086b20_FUN_80086b20.c` |
| `0x80086EC0` | [`bg_unit_tile_pick_left_right`] | draw an id that differs from the left/right neighbours | translated | medium | `80086ec0_FUN_80086ec0.c` |
| `0x80086FE0` | [`bg_unit_tile_pick_up_down`] | draw an id that differs from the up/down neighbours | translated | medium | `80086fe0_FUN_80086fe0.c` |

The command word is a `u16` at `model+0xC10`: bits `0..9` are the tile id and
bits `8..9` are the layer offset. Each id family is a six-entry `u16` table
(`0x803103E8`, `0x803103F4`, `0x80310400`, `0x8031040C`, `0x80310418`,
`0x80310424`, `0x803108F8`, `0x80310904`) whose contents are exactly the id
range the decision tree allows, e.g. `0x803103E8 = 0x102..0x107`. The
four-entry tables at `0x80427958`, `0x80427960`, `0x80427968` and `0x80427970`
hold the range-ending ids `0x110/0x120/0x130/0x140`, their `+1` variants, and
the `0x118`/`0x119` families.

`0x80086B20` reads one of three 12-byte "method record" tables
(`0x803104D8` for the `0x1xx` layer, `0x803106B8` for `0x2xx` and the BSS table
`0x80355F60` for `0x3xx`), tests the record with `0x802DCB90` (any of its three
words non-zero) and dispatches it through `0x802DCBFC` when `owner+0xC28` is
set. The default record comes from `0x802FD328` and is `{0,0,0}`. The recorded
handlers are `0x80085E80`, `0x800861D0`, `0x80086160` (low layer) and
`0x80086A60` (mid layer).

## Camera depth selectors (`0x800872E0..0x80087360`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x800872E0` | [`bg_camera_clip_depth`] | `2500.0f` or `3600.0f`, chosen by the camera mode and owner byte | translated | high | `800872e0_FUN_800872e0.c` |
| `0x80087340` | [`bg_camera_near_plane`] | near plane `5.0f` / `0.0f` from the owner `+0x54` byte | translated | high | `80087340_FUN_80087340.c` |
| `0x80087360` | [`bg_camera_far_plane`] | far plane `0.0f` / `-3500.0f` from the owner `+0x54` byte | translated | high | `80087360_FUN_80087360.c` |

The four constants are read from `r2`-relative storage: `0x8042BF18 = 2500.0f`,
`0x8042BF1C = 3600.0f`, `0x8042BF20 = 5.0f`, `0x8042BF24 = 0.0f` and
`0x8042BF28 = -3500.0f`. The owner pointer is `*(r13-0x5870)` for the planes
and `*(r13-0x56E0)` for the clip depth.

## BG-unit cell accessors (`0x80077470..0x80077490`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80077470` | [`bg_unit_cell_pair`] | two words of one 8-byte cell, returned as a `u64` pair | translated | high | `80077470_FUN_80077470.c` |
| `0x80077490` | [`bg_unit_cell_address`] | address of one 8-byte cell | translated | high | `80077490_FUN_80077490.c` |

Both index the array at `*(owner+0x70)` with `index & 0xFFFF`. The original
returns the pair in `r3:r4` (`CONCAT44`), which the Rust signature reproduces.

## Stage animation list (`0x80087890..0x80087C30`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80087890` | [`stage_anim_step_all`] | advance every entry of the `+0x34` list | translated | high | `80087890_FUN_80087890.c` |
| `0x80087940` | [`stage_anim_entry_init`] | fill one 0x18-byte entry and find its last frame | translated | high | `80087940_FUN_80087940.c` |
| `0x800879A0` | [`stage_anim_entry_step`] | advance the frame index and timer in either direction | translated | high | `800879a0_FUN_800879a0.c` |
| `0x80087C30` | [`stage_anim_list_release`] | free every entry and clear the head | translated | high | `80087c30_FUN_80087c30.c` |

An entry is 0x18 bytes: `+0x00` next, `+0x04` `u16` id, `+0x06` frame index,
`+0x07` last frame, `+0x08` timer, `+0x0C` resource handle, `+0x10` frame-table
pointer, `+0x14` direction byte. The frame table is a byte array of durations
terminated by a zero byte at `last+1`. `0x80087AF0` (not translated here) is the
allocator that links an entry into `+0x34`.

## Stage helpers (`0x80088D90..0x80089030`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80088D90` | [`stage_timer_advance`] | add the frame delta to the float at `+0x2C` | translated | high | `80088d90_FUN_80088d90.c` |
| `0x80088FD0` | [`stage_aux_object_destroy`] | deleting destructor of the `0x80310A78` class | translated | medium | `80088fd0_FUN_80088fd0.c` |
| `0x80089030` | [`stage_type_code`] | map a stage section kind to a `0..=5` code | translated | high | `80089030_FUN_80089030.c` |

`0x80088D90` widens both floats to `double`, adds, then narrows back, which is
reproduced so the rounding matches. `0x80089030` selects one of the four
0x3B0-byte stage objects with `stage_info+0x120E`, uses the section id
`stage_info+0x120F`, and treats a null `stage+0x2C` field as "no stage object".

## dDyl resource slots (`0x8008F110..0x8008F870`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008F110` | [`ddyl_global_init`] | run the three dDyl global initialisers | translated | high | `8008f110_FUN_8008f110.c` |
| `0x8008F140` | [`ddyl_slot_construct`] | install the slot vtable and clear four fields | translated | high | `8008f140_FUN_8008f140.c` |
| `0x8008F170` | [`ddyl_slot_delete`] | deleting destructor of a slot | translated | medium | `8008f170_FUN_8008f170.c` |
| `0x8008F1B0` | [`ddyl_slot_acquire`] | start or finish a loader and return its result handle | translated | medium | `8008f1b0_FUN_8008f1b0.c` |
| `0x8008F2B0` | [`ddyl_slot_poll`] | release a finished load and rewind the slot | translated | high | `8008f2b0_FUN_8008f2b0.c` |
| `0x8008F310` | [`ddyl_slot_poll_release`] | as above plus the vtable slot `+0xC` | translated | high | `8008f310_FUN_8008f310.c` |
| `0x8008F3F0` | [`ddyl_slot_release_if_loaded`] | guarded acquire, then the vtable slot `+0xC` | translated | high | `8008f3f0_FUN_8008f3f0.c` |
| `0x8008F620` | [`ddyl_slot_is_ready`] | branch-free `word != 0` test of the ready word | translated | high | `8008f620_FUN_8008f620.c` |
| `0x8008F7B0` | [`ddyl_slot_finish`] | release the global loader once its completion byte is set | translated | high | `8008f7b0_FUN_8008f7b0.c` |
| `0x8008F870` | [`ddyl_loader_construct`] | base constructor plus the loader vtable at `+0xC` | translated | high | `8008f870_FUN_8008f870.c` |

A slot is 0x14 bytes: `+0x00` vtable `0x80310E40`, `+0x04` id (`-1` idle),
`+0x08` loader handle, `+0x0C` loader object, `+0x10` result handle. The loader
wrapper installs `0x80310E98` at `+0x0C`. The supporting globals are
`r13-0x57D8` (completion callback), `r13-0x57BC` (ready word) and `r13-0x57B8`
(global loader handle).

## Uncertainties carried forward

- `0x80086B20` is `partial`: the original also places the address of the
  three-word method record in `r12` before calling `0x802DCBFC`, and the
  dispatcher reads `in_r12[0..2]`. A Rust call cannot set `r12`, so the stub is
  entered with the same `r3..r5` argument registers only. For the shipped
  records (`{0, -1, handler}`) the dispatcher takes its direct-call branch, but
  a record with a non-negative slot would need the missing register.
- `0x80086EC0`/`0x80086FE0` begin with a Metrowerks prologue helper
  (`0x802DCD70`) whose result Ghidra models as a 64-bit pair. The pairing used
  here (`high = r3`, `low = r4`) is the one that makes the bodies and the
  `0x80083FC0` call sites agree; it is not proven from a symbol table.
- The output pointer of `0x80083B40` is called with five arguments at these
  sites (`model, x, y, 0, 1`) while its Ghidra body only reads four. The fifth
  register is not read by the body, so the extra argument is harmless; it is
  kept to match the call site.
- `0x80355F60` (the `0x3xx` method-record table) is BSS in the shipped image, so
  every record is `{0,0,0}` there and the dispatch is skipped. Its runtime
  writers are outside this batch.
- `0x800890E0` (`FUN_800890e0`, 16 bytes) returns `&DAT_80310A94`; it is a
  one-instruction constant getter and was not worth a separate body. The bytes
  at that address are zero in the shipped image.
- `0x800861D0` and `0x80086160`, the other two low-layer handlers referenced by
  `0x803104D8`, are still untranslated. They sit in the gap between
  `0x80085FF0` and `0x80086A60`.
- `0x80086D10` (the helper that avoids all four direct neighbours) is still
  untranslated; it is called by address from the two decision trees.
- The remaining dDyl debug wrappers `0x8008F920..0x8008FCD0` are untranslated.
  They are five- to seven-argument print/format wrappers arming a global
  `r13-0x7C48` slot; they have no effect on the slot record above.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus, where relevant, `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original argument
  registers; none was "simplified" away.
- Three in-batch calls are direct Rust calls (`bg_unit_tile_pick_style_c` →
  `bg_unit_tile_pick_style_a`, the two poll functions →
  `ddyl_slot_release_if_loaded`, `stage_anim_step_all` →
  `stage_anim_entry_step`); they preserve the original argument order.
- `cargo check` reports **no diagnostics for `src/game/stage.rs`**. The
  workspace build is currently red because of two calls in
  `src/engine/globals.rs` (another lane); a scratch copy of the tree with only
  that unrelated call arity patched compiles with exit 0.
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
