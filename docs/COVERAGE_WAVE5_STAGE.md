# Coverage: wave 5 stage/course band (`0x8006E000..0x800A8000`)

## How to read this file

This document covers the wave-5 translation batch for the stage/course band of
`main.dol`. All 28 new functions live in `src/game/stage.rs`; no other Rust file
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
  inferred from call sites or from the instruction stream;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the
Ghidra name, address, size and evidence path.

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, any
`mod.rs`, `Cargo.toml`, `platform.rs`, or another lane's files.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this batch | 28 |
| Bytes of Ghidra function size | 2,976 |
| `translated` | 28 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 20 |
| Confidence `medium` | 8 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 28 |

The batch was selected by walking `ledger/functions.csv` and
`work/ghidra/main-gb/export/functions.tsv` in `0x8006E000..0x800A8000`, then
reading the untranslated entries adjacent to the existing stage/course
clusters. Chosen clusters: the 2-D collision-area object
(`0x8008C200..0x8008CA50`), the stage-slot record
(`0x8008EBA0..0x8008F000`), and the BG-unit tile/palette helpers
(`0x800833E0..0x80083EE0`).

## Collision areas (`0x8008C200..0x8008CA50`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008C200` | [`collision_area_construct`] | install the vtable and clear the list/contact state | translated | high | `8008c200_FUN_8008c200.c` |
| `0x8008C260` | [`collision_area_destroy`] | unlink and optionally free one area | translated | high | `8008c260_FUN_8008c260.c` |
| `0x8008C2C0` | [`collision_area_reset`] | clear the overlap flags and separation accumulators | translated | high | `8008c2c0_FUN_8008c2c0.c` |
| `0x8008C330` | [`collision_area_register`] | push an area onto the global list | translated | high | `8008c330_FUN_8008c330.c` |
| `0x8008C380` | [`collision_area_unregister`] | unlink an area from the global list | translated | high | `8008c380_FUN_8008c380.c` |
| `0x8008C3E0` | [`collision_area_set_descriptor`] | copy a 0x24-byte descriptor into the area | translated | high | `8008c3e0_FUN_8008c3e0.c` |
| `0x8008C440` | [`collision_area_init`] | descriptor copy plus the `+0x9F` path flag | translated | medium | `8008c440_FUN_8008c440.c` |
| `0x8008C4D0` | [`collision_area_max_x`] | right edge from centre and half extent | translated | high | `8008c4d0_FUN_8008c4d0.c` |
| `0x8008C4F0` | [`collision_area_min_x`] | left edge from centre and half extent | translated | high | `8008c4f0_FUN_8008c4f0.c` |
| `0x8008C510` | [`collision_area_center_x`] | centre x plus the owner offset | translated | high | `8008c510_FUN_8008c510.c` |
| `0x8008C530` | [`collision_area_max_y`] | top edge from centre and half extent | translated | high | `8008c530_FUN_8008c530.c` |
| `0x8008C550` | [`collision_area_min_y`] | bottom edge from centre and half extent | translated | high | `8008c550_FUN_8008c550.c` |
| `0x8008C570` | [`collision_area_center_y`] | centre y plus the owner offset | translated | high | `8008c570_FUN_8008c570.c` |
| `0x8008C6E0` | [`collision_area_overlap`] | shape-dispatched overlap test and mask update | translated | medium | `8008c6e0_FUN_8008c6e0.c` |
| `0x8008CA50` | [`collision_area_resolve`] | push two overlapping areas apart along their centre delta | translated | high | `8008ca50_FUN_8008ca50.c` |

The shared object has fields through `+0xA2` (0xA8 is the inferred aligned
size) and vtable `0x80310DF0`. The list is rooted at
`r13-0x5820` (head) and `r13-0x581C` (tail), i.e. `0x80429E60`/`0x80429E64`.
The `0x80310DB0` table selects a handler from the two `+0x9E` shape bytes.
The push-out constants are `0.5f` at `r2-0x7078` and `0.0f` at `r2-0x7080`.

## Stage-slot record (`0x8008EBA0..0x8008F000`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x8008EBA0` | [`stage_object_delete`] | per-element deleting destructor for the stage-object array | translated | high | `8008eba0_FUN_8008eba0.c` |
| `0x8008ED70` | [`stage_slot_construct`] | install the stage-slot vtable and reset it | translated | high | `8008ed70_FUN_8008ed70.c` |
| `0x8008EDB0` | [`stage_slot_reset`] | reset the 0x34-byte slot record to idle | translated | high | `8008edb0_FUN_8008edb0.c` |
| `0x8008EE00` | [`stage_slot_refresh`] | keep or reset a slot based on mode/id globals | translated | medium | `8008ee00_FUN_8008ee00.c` |
| `0x8008EEC0` | [`stage_slot_pair_check`] | test the two resource state words | translated | medium | `8008eec0_FUN_8008eec0.c` |
| `0x8008F000` | [`stage_slot_matches`] | match a slot and id pair against the record | translated | high | `8008f000_FUN_8008f000.c` |

The record is 0x34 bytes with vtable `0x80310E30`. `+0x04` is the asynchronous
state (`-1` idle, `4` empty in the two slots), `+0x08`/`+0x0C` are the loader
object/zero, `+0x14` is the source object, `+0x18`/`+0x19` are the requested
ids, and `+0x20..+0x30` are five resource state words. The refresh gate reads
the byte at `0x80315B9D`, the `u32` at `0x80315B98`, the byte at `0x80315B9C`
and the byte at `*(r13-0x5704)+0x3B3`.

## BG-unit tile/palette helpers (`0x800833E0..0x80083EE0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x800833E0` | [`bg_buffer_configure`] | bind the unit buffer and set two tile-map flags | translated | medium | `800833e0_FUN_800833e0.c` |
| `0x80083440` | [`bg_buffer_release`] | detach the unit buffer and restore two GX bits | translated | medium | `80083440_FUN_80083440.c` |
| `0x80083490` | [`palette_fill_value`] | read one `u16` from the palette fill table | translated | high | `80083490_FUN_80083490.c` |
| `0x800834B0` | [`tile_mask_value`] | read one `u16` from the tile mask table | translated | high | `800834b0_FUN_800834b0.c` |
| `0x80083E20` | [`bg_unit_tile_probe`] | probe one tile cell and select its palette block | translated | medium | `80083e20_FUN_80083e20.c` |
| `0x80083EC0` | [`bg_unit_tile_command`] | copy a two-byte tile command into the model | translated | high | `80083ec0_FUN_80083ec0.c` |
| `0x80083EE0` | [`bg_unit_tile_collision`] | remap a tile id when the layer mask says solid | translated | medium | `80083ee0_FUN_80083ee0.c` |

The model fields used here are the wave-4 BG-unit model: `+0xC00` command
table, `+0xC0A` tile id, `+0xC0C`/`+0xC0E` tile coordinates, `+0xC10` command
value, `+0xC1C` unit slot, `+0xC20` stage slot, `+0xC1E` in-cell nibbles and
`+0x408` the per-cell palette byte. The two `u16` tables are
`0x802EF998` (masks) and `0x802EF9B8` (palette fill).

## Uncertainties carried forward

- The collision-area class is unnamed. The layout through `+0xA2`, the mask bits and
  the shape index are inferred from `0x8008C200..0x8008CA50`; no symbol table
  confirms them. `collision_area_overlap` is `medium` for that reason.
- `0x8008C440`: Ghidra prints `FUN_8008c3e0()` without arguments because the
  callers preserve `r3`/`r4`/`r5`; the three forwarded arguments are taken from
  the unchanged registers.
- `0x8008EE00`: the meanings of `0x80315B9D` (mode), `0x80315B98` (active
  suppressor), `0x80315B9C` (id) and the owner byte `+0x3B3` are unknown. The
  comparisons and the reset paths are observed.
- `0x8008EEC0`: `FUN_80060110(value)` returns a value that is forwarded to
  `FUN_800B4710(result)`; Ghidra prints both calls with no arguments. The
  forwarding is read from the unchanged `r3`.
- `0x800833E0`: Ghidra prints only `owner`; the call site at `0x80077B40`
  passes two more words, which are forwarded to `FUN_8000F270`.
- `0x80083440`: the owner is hidden by the Metrowerks prologue helper but is
  still in `r3` when `FUN_8000F2E0` is called.
- `0x80083E20`: the call sites pass a separate `table_base` and `model`; when
  the table byte is zero the allocation helper expects the same object in
  `r3`. The tile-bit meaning is inferred.
- `0x80083EE0`: the split between the palette model (`+0xC1C`) and the tile
  model (`+0xC0C`/`+0xC0E`/`+0xC20`) is inferred from `0x80086B20`.
- `0x80083D00` (284 bytes, the bgdat upload) is still untranslated: its
  0xC10-byte stack scratch object is read by `0x80085780`/`0x800859F0` and its
  tail layout is not proven. It remains called by address.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus, where relevant, `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original argument
  registers; none was "simplified" away.
- `bg_unit_tile_probe`, `bg_unit_tile_command` and `bg_unit_tile_collision`
  reuse the wave-4 model accessors already in `src/game/stage.rs`;
  `bg_layer_u16` and `object_id_table_entry` are reused from
  `src/resource/course.rs`.
- `cargo check` passes (exit 0) with no `stage.rs` diagnostics.
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
