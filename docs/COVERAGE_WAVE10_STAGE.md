# Coverage: wave 10 stage band (`0x800A0000..0x80100000`)

## How to read this file

This document covers the wave-10 batch for the stage/course lane inside the
next `main.dol` band. All 28 new bodies live in `src/game/stage.rs`; no other
Rust file was changed.

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** — the Rust function in `src/game/stage.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present, with
  calls outside the batch left as absolute-address calls; `partial` would mean
  a call cannot be reproduced exactly (none here); `stub` is not used;
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field meaning, ABI detail or lookup table is
  inferred from call sites or from the surrounding code;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks unresolved details. Each function's `# Original` block repeats the Ghidra
name, address, size and evidence path.

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, any
`mod.rs`, `Cargo.toml`, `platform.rs`, or another lane's files. The only files
written are `src/game/stage.rs`, this document and
`ledger/agents/wave10-stage.csv`.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this batch | 28 |
| Bytes of Ghidra function size | 4,260 |
| `translated` | 28 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 22 |
| Confidence `medium` | 6 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 28 |

The batch was selected by walking `work/ghidra/main-gb/export/functions.tsv`
over `0x800A0000..0x80100000` (1,215 entries of at least 16 bytes) and crossing
it with `ledger/functions.csv`. The band is dominated by actors, bosses, the
world map and the save/scene engine, which belong to other lanes; the four
clusters below are the entries that use the stage-object table
(`r13-0x57E8`), the stage-info object (`r13-0x54B8`), the course manager
(`r13-0x5704`) or the camera owner (`r13-0x56E0`).

The course-manager identification is not a guess: the already-translated
`stage_slot_refresh` (`0x8008EE00`) reads `*(r13-0x5704)+0x3B3`, and
`course_manager_construct` / `course_manager_clear_entries` are the only
writers of that field in the shipped image. The stage slot constructed at
`+0x08` is the same 0x34-byte record used by `0x8008EBA0..0x8008F000`.

## Course manager (`0x800BB090..0x800BBC90`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x800BB090` | [`course_manager_construct`] | install the vtable, construct the `+0x08` stage slot and register the global owner | translated | high | `800bb090_FUN_800bb090.c` |
| `0x800BB130` | [`course_manager_set_defaults`] | initialise the object-order table and four layer words | translated | high | `800bb130_FUN_800bb130.c` |
| `0x800BB170` | [`course_manager_clear_entries`] | clear the six 8-entry object groups and the per-stage bookkeeping | translated | high | `800bb170_FUN_800bb170.c` |
| `0x800BB400` | [`course_manager_reset`] | reset the manager, its stage slot and its tile/object lists | translated | high | `800bb400_FUN_800bb400.c` |
| `0x800BB560` | [`course_manager_clear_tile_lists`] | clear the two strided pointer tables and twenty 8-byte records | translated | high | `800bb560_FUN_800bb560.c` |
| `0x800BB780` | [`course_manager_set_course`] | apply one 0x10-byte course descriptor and request stage scene `7` | translated | medium | `800bb780_FUN_800bb780.c` |
| `0x800BB8F0` | [`course_manager_load_course`] | copy the manager into the course-state block and rebuild the id tables | translated | medium | `800bb8f0_FUN_800bb8f0.c` |
| `0x800BBB70` | [`course_manager_set_tile_entry`] | write one 0x10-byte record into the `(x, y)` object grid | translated | high | `800bbb70_FUN_800bbb70.c` |
| `0x800BBC90` | [`course_manager_global_init`] | fill the course-info globals and construct the global stage slot | translated | high | `800bbc90_FUN_800bbc90.c` |

The manager is at least 0xB5C bytes (the highest byte written is `+0xB5B`).
Its observed layout is documented in the section comment of `stage.rs` and in
the module header. The most useful anchors are:

- `+0x008` embedded stage slot (vtable `0x80310E30`);
- `+0x380` "marked" byte, `+0x384` object-order word;
- `+0x388`/`+0x38C`/`+0x390` layer words;
- `+0x395..+0x3A0` and `+0x3A9..+0x3B4` twelve per-stage bytes;
- `+0x3B3` the byte read by `stage_slot_refresh`;
- `+0xAF4..+0xB5B` reload bookkeeping.

The shared course-state block at `0x80358D58` is not part of the manager: the
manager copies its `+0x0C..+0x38` fields into it and fills the four
id-indexed tables at `+0x44`, `+0x54`, `+0x68`, `+0x78` and `+0x88`. The
four-word order tables are `0x80354E50`/`0x80354E60`; the per-id words come
from `0x80354E70/80/90/A0`.

## Camera globals (`0x800CD230..0x800CD3E0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x800CD230` | [`camera_global_f32`] | return camera global `0x80317894` | translated | high | `800cd230_FUN_800cd230.c` |
| `0x800CD240` | [`camera_mode_flag`] | return the byte at `0x803178B0` | translated | high | `800cd240_FUN_800cd240.c` |
| `0x800CD250` | [`camera_mode_is_mid`] | test the camera mode byte against `0x20..=0x30` | translated | high | `800cd250_FUN_800cd250.c` |
| `0x800CD2C0` | [`camera_mode_is_low`] | test the camera mode byte against `0x10..=0x1F` | translated | high | `800cd2c0_FUN_800cd2c0.c` |
| `0x800CD330` | [`camera_value_store`] | copy camera word `0x803178B4` to the caller | translated | high | `800cd330_FUN_800cd330.c` |
| `0x800CD350` | [`camera_byte_b2`] | return the byte at `0x803178B2` | translated | high | `800cd350_FUN_800cd350.c` |
| `0x800CD360` | [`camera_f32_b8`] | return camera `0x803178B8` as a float | translated | high | `800cd360_FUN_800cd360.c` |
| `0x800CD370` | [`camera_f32_bc`] | return camera `0x803178BC` as a float | translated | high | `800cd370_FUN_800cd370.c` |
| `0x800CD380` | [`camera_word_c0`] | return camera word `0x803178C0` | translated | high | `800cd380_FUN_800cd380.c` |
| `0x800CD390` | [`camera_manager_construct`] | install the `0x803178E0` vtable and register the camera manager | translated | high | `800cd390_FUN_800cd390.c` |
| `0x800CD3E0` | [`camera_manager_destroy`] | reinstall the vtable, detach the subobject and free the manager | translated | medium | `800cd3e0_FUN_800cd3e0.c` |

The owner pointer is `*(r13-0x56E0)`, which is the same object
`bg_camera_clip_depth` (`0x800872E0`) reads. The two predicates are called by
the stage environment code at `0x800872E0`, `0x800892F0` and `0x80089B80`,
which is why they are part of this lane rather than the camera lane. The
parameters at `0x80317890..0x803178C0` are read as words, bytes and floats
exactly as Ghidra prints them; the `lfs`/`lwz`/`lbz` instructions were
cross-checked in the shipped ELF.

## Stage-object section helpers (`0x800CD470..0x800D90C0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x800CD470` | [`stage_point_section_code`] | look up the section at the camera's left edge and store its code | translated | medium | `800cd470_FUN_800cd470.c` |
| `0x800CFD00` | [`stage_section3_copy`] | copy a section-3 entry and refresh its slot byte | translated | high | `800cfd00_FUN_800cfd00.c` |
| `0x800CFDE0` | [`stage_section3_changed`] | test whether a section-3 record no longer matches | translated | high | `800cfde0_FUN_800cfde0.c` |
| `0x800CFEA0` | [`stage_object_tile_lookup`] | find the object entry whose world cell contains `(x, y)` | translated | medium | `800cfea0_FUN_800cfea0.c` |
| `0x800D90C0` | [`stage_object_section_entry`] | find an 8-byte list entry by its first byte | translated | high | `800d90c0_FUN_800d90c0.c` |

All five select the stage object as
`*(r13-0x57E8) + stage_info[0x120E] * 0x3B0`, then treat `stage+0x2C == 0` as
"no stage object". Section-3 entries come from
`crate::resource::course::section3_entry_by_id`; point lookup uses
`section4b_entry_at_point` and `section4b_byte_11` from the same file.

`0x800CFEA0` scans the `stage+0x20` list (`stage+0x90` entries of 0x14 bytes).
Each entry has `u16` cell x/y at `+0x00/+0x02`, a class byte at `+0x0B`, a
signed id byte at `+0x08` and a flags word at `+0x10` (bit `0x80` = skip).
The three `r2` extent constants at `0x8042C9C0/BC/B8` are combined by the
class byte through the original chained assignment; the Rust reproduces the
assignment tree exactly because the unsigned subtractions wrap for classes
below `0x10` and produce non-obvious pairs for `5`/`6`.

## Stage-resource readiness (`0x800E3640..0x800E3AB0`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x800E3640` | [`stage_resource_table_entry`] | select one 8-byte resource-table header by id | translated | high | `800e3640_FUN_800e3640.c` |
| `0x800E3660` | [`stage_resources_ready`] | test every dDyl id of the selected table through `0x8008F620` | translated | medium | `800e3660_FUN_800e3660.c` |
| `0x800E3AB0` | [`stage_slot_construct_thunk`] | one-argument wrapper around `stage_slot_construct` | translated | high | `800e3ab0_FUN_800e3ab0.c` |

The table base is `r13-0x6F34` (`0x8042874C`). Id `1` is replaced by the byte
at `r13-0x5618`; the selector byte read by `0x800E3660` is at `r13-0x5617`.
Each header is `{u16 count; u32 entry_list}` and each list entry is
`{i16 type; u16 id_count; u32 ids}`. Only type `3` entries are checked.
`0x8008F620` ignores the id argument and tests the single global ready word at
`r13-0x57BC`, so the per-id loop is reproduced literally even though every
iteration observes the same word.

## Uncertainties carried forward

- `0x800BB780`: Ghidra warns that globals starting with `_` overlap smaller
  symbols at `0x80315B9C`. The shipped instructions
  (`0x800BB7C0..0x800BB834`) show one `stw` of the `+0xC` word, a `lbz` of
  its byte 1 for the mode test, and two `stb`s to bytes `+2`/`+3` in the
  `0x26` branch. The Rust follows the instructions. The meaning of bytes
  `+2`/`+3` is unknown; the 32-bit `id - 1` index into `0x802F0D28` is
  reproduced with `wrapping_sub`/`wrapping_mul`.
- `0x800BB8F0`: the two-pair table loop and the five id-indexed destination
  tables are observed, but the fields they represent are unnamed. The two
  byte copies from `r13-0x5508`/`-0x5500` to `-0x5507`/`-0x54FF` are literal.
- `0x800BB170`: the `i*2+1 == 4` branch can never be taken because `i*2` is
  even. The comparison is kept so the original structure is visible.
- `0x800CD3E0`: Ghidra prints the delete call without the size argument; the
  size is forwarded in `r4`, following the same convention as
  `stage_object_delete`.
- `0x800CD470`: Ghidra prints `FUN_800789a0()` with no argument although the
  body reads `param_1+0x8FE64`; the camera owner (`*(r13-0x58B0)`) is passed,
  matching the other call sites of `0x800789A0` in this file. The stack point
  has three floats: `x`, `y` and the z word from `r2-0x66F0`.
- `0x800CFEA0`: `param_3` is accepted but never read by the original. The
  three extent constants and the class-byte tree are observed; their unit and
  the meaning of the class values are unknown.
- `0x800E3660`: Ghidra declares the function `void`, but it computes a 0/1
  accumulator and passes it to the restore helper `0x802DCDB8`. The Rust
  returns that accumulator; if the shipped caller discards `r3` this is
  equivalent. `0x8008F620` ignores the id argument.
- The course-manager size `0xB5C` is inferred from the highest written byte
  `+0xB5B`; there is no allocation-size call in this batch to confirm it.
- `0x80315BA0` (course-manager vtable) and `0x803178E0` (camera vtable) live
  in `.data`; the `0x80354E50..0x80354EA0` order/id tables and the whole
  `0x80358Dxx` course-state block are `.bss`, so they are zero at boot and
  filled at runtime. Their contents are read at runtime, not baked into the
  Rust.

## Method notes

- Every function cites its Ghidra name, address, size and exact `.c` file in
  `# Original`, and carries `# Confidence` plus `# Safety`.
- Loads/stores go through `crate::platform` so big-endian semantics do not
  depend on the host.
- Calls outside the batch are absolute-address calls with the original
  argument registers; none was "simplified" away.
- In-batch calls are direct Rust calls: `course_manager_construct` →
  `course_manager_set_defaults`, `course_manager_reset` →
  `course_manager_clear_entries`/`stage_slot_reset`/
  `course_manager_clear_tile_lists`, `course_manager_set_course` →
  `course_manager_load_course`, `course_manager_load_course` →
  `stage_slot_refresh`, `course_manager_global_init` →
  `stage_slot_construct_thunk`, `stage_slot_construct_thunk` →
  `stage_slot_construct`, `camera_mode_*` → `camera_mode_flag`,
  `stage_resources_ready` → `stage_resource_table_entry`, and
  `stage_point_section_code` → `section4b_byte_11`.
- `cargo check` reports **no diagnostics for `src/game/stage.rs`**. The rest of
  the tree keeps its 12 pre-existing warnings.
- `node tools/doc-lint.mjs src` reports 68 files and 0 violations.
