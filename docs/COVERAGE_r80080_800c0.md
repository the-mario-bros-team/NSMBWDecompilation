# Coverage — `r80080_800c0` (main.dol `0x80080000..0x800C0000`)

## How to read this file

This document describes one address-range lane of the reconstruction. The lane
owns main.dol entry addresses in `[0x80080000, 0x800C0000)`. **All 20 translated
functions have been promoted out of staging:** the stage runtime lives in
`src/game/stage.rs`, the course-data/bgdat decoders in `src/resource/course.rs`.
`src/subsystems/r80080_800c0.rs` is now an empty-lane notice that maps every
address to its new home, so the staging module stays resolvable and no old link
dangles.

* **Address** — virtual address of the *entry point* in `main.dol` (image base
  `0x80004000`, so these are not file offsets).
* **Rust name** — the name of the translated function. It is descriptive only
  when the evidence supports the role; the original Ghidra name (`FUN_…`) is in
  the function's `# Original` doc block.
* **Role** — one sentence, in plain English.
* **Status** — `translated` (all control flow and calls accounted for),
  `partial` (entry/exit and the field effects are reproduced, but some callees
  are called by address without being reconstructed), or `stub` (address/role
  only).
* **Confidence** — strength of the *evidence*, not the quality of the Rust:
  `high` = every store/load is visible in the Ghidra C; `medium` = layout
  observed but the meaning of a field/table is inferred; `low` = mostly
  unknown.
* **Home** — the semantic Rust file that now owns the function; the staging
  range module only holds the address→home table.
* **Evidence** — the Ghidra file the translation was read from, under
  `work/ghidra/main-gb/export/`.

To check a translation, open the Rust function, read its `# Original` block,
then open the evidence `.c` file. If the two disagree, the C wins. Offsets in
this document (`+0x2E4`, `r13-0x57E8`, …) are byte offsets from the function's
first argument or from the small-data base, exactly as printed by Ghidra.

## Scope and counts

| Item | Count |
|---|---|
| Function entries with an entry address in range | 780 |
| Excluded: smaller than 16 bytes (one- or two-instruction thunks) | 93 |
| Eligible (`>= 16` bytes) | 687 |
| Translated in this lane (this pass) | 20 |
| Of those, `partial` | 1 (`0x80081D20`) |
| Promoted to `src/game/stage.rs` | 15 |
| Promoted to `src/resource/course.rs` | 5 |
| Still staged in `src/subsystems/r80080_800c0.rs` | 0 |

The promotion is a **move**, not a copy: the bodies, their `# Original`
evidence blocks and the lane's data-model docs now live in the semantic
modules, and the ledger rows name those files instead of the staging range.

`0x800C9F90` is called out as "skip" in the lane brief; it is **above** this
range (`0x800C9F90 >= 0x800C0000`) and belongs to the engine lane, so it never
appears here.

## What this lane does

It is the course-resource lane: it turns a course number into the data a stage
needs.

* `course/course<N>_bgdatL{0,1,2}.bin` — the three tile/background layers.
* `BG_unt_<name>{,_hd}.bin` — the background-unit meshes and textures, one set
  per stage/unit, named by strings the stage data provides.
* `course/course<N>.bin` — the object list and the per-section tile tables,
  parsed into a 0x3B0-byte stage object.

The four stage slots are the reason every loop in this lane runs `0..4` (stage)
and `0..4` or `0..3` (unit): the game keeps the current stage and its
neighbours resident.

## Recovered machinery (needed to read the C)

Ghidra shows many operands as `unaff_r13` / `unaff_r2` because the ELF import
does not set the PowerPC small-data base registers. Both bases are in the image
prologue inside `__init_registers` (`0x800041C0`), so they are **observed**, not
guessed:

| Register | Value | Evidence |
|---|---|---|
| `r1` (stack) | `0x8043FC40` | `lis r1,0x8043` / `ori r1,r1,0xFC40` at `0x80004234`/`0x80004238` |
| `r2` (small-data-2) | `0x80433080` | `lis r2,0x8043` / `ori r2,r2,0x3080` at `0x8000423C`/`0x80004240` |
| `r13` (small-data) | `0x8042F680` | `lis r13,0x8042` / `ori r13,r13,0xF680` at `0x80004244`/`0x80004248` |

`r13` is imported from `crate::engine::startup::SDA_BASE` (same constant, owned
by the engine lane); `r2` is defined locally in `game/stage.rs` because only
this lane needs it. With these, every `r13 - 0xNNNN` in this lane resolves to a
concrete BSS address, listed in the constant docs.

A second trap in the Ghidra output: functions that begin with
`iVar = FUN_802dcd5c()`, `FUN_802dcd64()`, `FUN_802dcd68()`, `FUN_802dcd70()`
or `FUN_802dcd74()` are calling the Metrowerks register-save helpers
(`__savegprlr_21/23/24/26/27` at `0x802DCD5C…0x802DCD74`), and the matching
`FUN_802dcda8(1)` / `FUN_802dcdb0()` / `FUN_802dcdb4()` / `FUN_802dcdbc()` at
the end are the restore helpers. These are prologue/epilogue, **not** logic, and
they preserve `r3`. That is why three functions here (`0x80081410`,
`0x800814d0`, `0x800816a0`) were printed as `void …(void)` but really take the
BG manager pointer in `r3`; all three call sites in `0x80081800` pass it.

## Translated and promoted functions

Every row is promoted; the `Home` column is the semantic file that now owns the
function (the staging file only references it).

| Address | Rust name | Role | Status | Confidence | Home | Evidence |
|---|---|---|---|---|---|---|
| `0x800813F0` | `unit_name` | Address of one 0x20-byte BG-unit name inside the manager | translated | high | `game/stage.rs` | `800813f0_FUN_800813f0.c` |
| `0x80081410` | `copy_unit_names` | `strncpy` the four unit names of each stage into the manager | translated | high | `game/stage.rs` | `80081410_FUN_80081410.c` |
| `0x800814D0` | `load_bg_units` | Loads `course<N>_bgdatL{0,1,2}.bin` + `BG_unt_<name>{,_hd}.bin` | translated | medium | `game/stage.rs` | `800814d0_FUN_800814d0.c` |
| `0x800816A0` | `create_bg_unit_models` | Creates the three BG-unit models per stage | translated | medium | `game/stage.rs` | `800816a0_FUN_800816a0.c` |
| `0x80081780` | `refresh_bg_unit_models` | Rebuilds those models after a layer change | translated | high | `game/stage.rs` | `80081780_FUN_80081780.c` |
| `0x80081800` | `init_stage_resources` | Entry point: archive handle → names → layers → models | translated | high | `game/stage.rs` | `80081800_FUN_80081800.c` |
| `0x80081850` | `bg_unit_model` | Model handle accessor (`+0x34`) | translated | high | `game/stage.rs` | `80081850_FUN_80081850.c` |
| `0x80081870` | `bg_unit_u16` | `u16` read through the `+0x64` texture block | translated | high | `game/stage.rs` | `80081870_FUN_80081870.c` |
| `0x80081890` | `bg_unit_mesh_offset` | `+0xA4` mesh base plus a byte offset | translated | high | `game/stage.rs` | `80081890_FUN_80081890.c` |
| `0x800818B0` | `bg_layer_data` | L1 layer handle accessor (`+4`) | translated | high | `resource/course.rs` | `800818b0_FUN_800818b0.c` |
| `0x800818E0` | `object_id_table_entry` | `u16` id lookup in the table at `0x802EF934` | translated | medium | `resource/course.rs` | `800818e0_FUN_800818e0.c` |
| `0x80081900` | `object_id_in_list` | Linear search of the `0xFFFF`-terminated id list | translated | medium | `resource/course.rs` | `80081900_FUN_80081900.c` |
| `0x80081BA0` | `is_special_object_id` | "id 0x2E remaps to 0x2F" test | translated | medium | `resource/course.rs` | `80081ba0_FUN_80081ba0.c` |
| `0x80081BE0` | `is_current_stage_unit` | World/stage/area compare against the stage-info object | translated | medium | `game/stage.rs` | `80081be0_FUN_80081be0.c` |
| `0x80081D20` | `create_bg_scene` | BG scene constructor: defaults, tile buffers, manager | partial | medium | `game/stage.rs` | `80081d20_FUN_80081d20.c` |
| `0x80081F40` | `alloc_tile_buffers` | Allocates and zeroes 12 × 64 KiB tile buffers | translated | high | `game/stage.rs` | `80081f40_FUN_80081f40.c` |
| `0x80082050` | `free_tile_buffers` | Frees every non-null tile buffer | translated | high | `game/stage.rs` | `80082050_FUN_80082050.c` |
| `0x800820D0` | `free_tile_buffer` | Frees one tile buffer and nulls its slot | translated | high | `game/stage.rs` | `800820d0_FUN_800820d0.c` |
| `0x80082120` | `bg_layer_u16` | `u16` read through the `+4` layer handle | translated | high | `resource/course.rs` | `80082120_FUN_80082120.c` |
| `0x8008DFF0` | `parse_course_file` | Parses `course/course<N>.bin` into the stage object | translated | medium | `game/stage.rs` | `8008dff0_FUN_8008dff0.c` |

## Clusters

Destination by cluster: bootstrap, manager accessors, scene/tile buffers and the
parser → `game/stage.rs`; the bgdat layer readers and the object-id tables →
`resource/course.rs`. The cluster order below is still the reading order across
both files.

1. **Stage-resource bootstrap** — `0x80081410`, `0x800814D0`, `0x800816A0`,
   `0x80081780`, `0x80081800`. All five operate on the same 0x328-byte BG
   manager and run back to back from `init_stage_resources`. Read them in that
   order.
2. **Manager accessors** — `0x800813F0`, `0x80081850`, `0x80081870`,
   `0x80081890`, `0x800818B0`, `0x80082120`. Pure offset arithmetic over the
   manager; they are the map of its layout. The two that expose a loaded bgdat
   layer (`0x800818B0`, `0x80082120`) sit in `resource/course.rs` with the
   layer decoding; the rest stay in `game/stage.rs`.
3. **Object-id tables** — `0x800818E0`, `0x80081900`, `0x80081BA0`, now in
   `resource/course.rs`. The object ids placed by the course file (section
   records) are translated through these tables before spawning. The tables
   themselves are `.rodata` at `0x802EF900`/`0x802EF934`.
4. **Scene construction and tile buffers** — `0x80081D20`, `0x80081F40`,
   `0x80082050`, `0x800820D0`, now in `game/stage.rs`. `create_bg_scene` owns
   the 3 × 4 grid of 0x10000-byte buffers; the other three allocate/free it.
5. **Course-file parser** — `0x8008DFF0`, now in `game/stage.rs`. Standalone:
   reads `course/course<N>.bin`, fills the 0x3B0-byte stage object, then buckets
   its 0x10-byte object entries by id byte into 64 lists. It is the only writer
   of the pointers that `copy_unit_names` reads back through `[slot+8]`.

## Uncertainties

* `0x80081D20` is `partial`: the vtable slots and the two constructors
  (`0x80081070` BG manager, `0x80082160` window clear) are invoked by address,
  not reconstructed here. Every field store in the body is reproduced.
* The BG-manager field map (`+4` = L1, `+8` = L2, `+0xC` = L0; `+0x64` =
  texture; `+0xA4` = mesh) is inferred from the writers in `0x800814D0` and the
  readers in `0x80081850`/`0x80081870`/`0x80081890`/`0x800818B0`/`0x80082120`.
  There is no symbol table to confirm the names.
* `FUN_8016E630(arg1 = 0x20F2C)` is described as a resource-heap/file loader
  because of how `0x80081D20` uses its result as a heap for `operator new`; its
  own body is out of this lane.
* `SECTION_DIVISORS` (`0x802F00F8`) is a seven-pair table whose unit is
  unknown. The divisions in `0x8008DFF0` are reproduced as-is; the table values
  are non-zero (checked in the ELF), so the original never divides by zero.
* The two `BG` resource names at `0x8030FDE0` and `0x8030FE08` are Shift-JIS and
  were not decoded.
* `0x8008DFF0`'s "two pointers + two counts per section" split (which of the
  pair is the visible layer and which is the collision/behind layer) is not
  proven.

Each of these is repeated in the destination docs, next to the code that depends
on it: the `# Confidence` blocks in `game/stage.rs` / `resource/course.rs`, and
the BG-manager data-model table in the `game/stage.rs` module header.

## Remaining in staging

**None.** `src/subsystems/r80080_800c0.rs` no longer contains a function, only
the moved-table notice; the ledger rows for all 20 addresses point at
`src/game/stage.rs` (15) or `src/resource/course.rs` (5) with module `game` /
`resource`. Nothing was left behind for lack of a role, so there is no
`// unknown:` stub to finish in this lane.

## Next targets in this range

Highest value first, all with existing Ghidra C:

1. `0x80081070` — BG manager constructor (0x328 bytes of initialisers). Needed
   to make `create_bg_scene` complete instead of partial.
2. `0x80081C40` — picks the current stage slot and drives the
   `0x8008E770`/`0x8008E650`/`0x8008E350` path; the natural caller of
   `parse_course_file`.
3. `0x8008E310`/`0x8008E350`/`0x8008E390` — the small stage-slot helpers used
   by that path (also `0x8008E650`, `0x8008E680`, `0x8008E770`).
4. `0x80083560` — BG-unit model factory; would let `create_bg_unit_models` and
   `refresh_bg_unit_models` describe their model argument instead of passing it
   through.
5. `0x800892F0` — large "course environment" setup (0xA0 `dCaptureMng` object,
   depth-of-field selection by stage type). Evidence is readable but it is
   heavily paired-single (`ps_*`) code that Ghidra prints as GQR bit tricks, so
   it needs an instruction-level pass first.

## Method notes

* Every Rust function cites `/// Ghidra: <name> <address>` style evidence in its
  `# Original` block and points at the exact `.c` file.
* Reads/writes go through `crate::platform` so the big-endian semantics do not
  depend on the host.
* Calls into code outside the lane are absolute-address calls with the original
  argument registers; none of them was "simplified" away.
