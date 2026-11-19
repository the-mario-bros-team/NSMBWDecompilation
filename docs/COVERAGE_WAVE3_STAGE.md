# Coverage: wave 3 stage/course second pass

## How to read this file

This document covers the **second-pass translation** of stage, BG-manager,
BG-scene and course-data code from `main.dol` into:

- `src/game/stage.rs` — BG manager/scene lifecycle and the tile-map object;
- `src/resource/course.rs` — bgdat layer access plus the course section
  lookups and object-id remapping.

Every row describes one Ghidra entry point:

- **address** — virtual address of the entry in `main.dol` (image base
  `0x80004000`, so these are not file offsets);
- **name** — the Rust function in the destination file;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present;
  `partial` means the logic is present but one original detail (paired-single
  scaffolding) is not reproduced;
- **confidence** — `high` when every load/store/branch was read directly from
  the Ghidra C; `medium` when a field name, object type or hidden argument is
  inferred from call sites;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust files, `// observed:` marks facts read from the Ghidra output,
`// inferred:` marks roles reconstructed from call sites, and `// unknown:`
marks an unresolved detail. `# Original` in each function doc repeats the
Ghidra name, address, size and evidence path.

This pass did **not** run Ghidra, install tools, or touch `lib.rs`, `mod.rs`,
`Cargo.toml`, `platform.rs`, or any other lane's files. The skip address
`0x800C9F90` is outside this range (`0x800C9F90 >= 0x800C0000`) and was not
visited.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated in this pass | 25 |
| Bytes of Ghidra function size | 3,020 |
| `translated` | 24 |
| `partial` | 1 |
| `stub` | 0 |
| Confidence `high` | 12 |
| Confidence `medium` | 13 |
| Confidence `low` | 0 |
| Destination `src/game/stage.rs` | 11 functions |
| Destination `src/resource/course.rs` | 14 functions |

Selection rule: complete clusters adjacent to the existing stage/course
translations rather than isolated addresses. The two clusters are:

1. **BG manager → scene → tile map** (`0x80081070..0x800826E0`). This closes the
   constructor that `create_bg_scene` previously called by address, and adds
   the scene direction/wrap helpers and the 0x44-byte tile-map object used by
   `0x80077EB0` / `0x80082740`.
2. **Course section and object-id lookups** (`0x80081AB0`, `0x80082140`,
   `0x800822E0`, `0x8008E310..0x8008E800`). These are the readers/writers that
   `0x80081C40` and the actor spawn path use on the seven section tables built
   by `parse_course_file`.

## BG manager / scene / tile map (`src/game/stage.rs`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80081070` | [`bg_manager_construct`] | installs the manager vtable and clears the 0x328-byte manager body | translated | high | `80081070_FUN_80081070.c` |
| `0x80081C40` | [`bg_scene_camera_entry`] | selects a stage section entry from the camera position and a mode | translated | medium | `80081c40_FUN_80081c40.c` |
| `0x80082160` | [`bg_scene_clear`] | clears the four dwords at scene `+0x84` | translated | high | `80082160_FUN_80082160.c` |
| `0x80082180` | [`bg_scene_update_x_direction`] | classifies scene x origin against camera `+0x8FEA8` | translated | high | `80082180_FUN_80082180.c` |
| `0x800821E0` | [`bg_scene_update_z_direction`] | classifies scene z origin against camera `+0x8FEAC` | translated | high | `800821e0_FUN_800821e0.c` |
| `0x80082240` | [`bg_scene_wrap_x`] | wraps a scene x edge across the camera width in mode `2` | partial | medium | `80082240_FUN_80082240.c` |
| `0x80082310` | [`bg_tilemap_construct`] | constructs the 0x44-byte BG tile-map object | translated | high | `80082310_FUN_80082310.c` |
| `0x80082410` | [`bg_tilemap_init`] | fills dimensions, allocates and zeroes the two cell arrays | translated | medium | `80082410_FUN_80082410.c` |
| `0x80082560` | [`bg_tilemap_release`] | frees the two arrays and detaches the buffer owner | translated | high | `80082560_FUN_80082560.c` |
| `0x800826C0` | [`bg_tilemap_set`] | writes one `u16` cell through the `+0x28` array | translated | high | `800826c0_FUN_800826c0.c` |
| `0x800826E0` | [`bg_scene_camera_limits`] | returns the camera x/z wrap limits as two floats | translated | medium | `800826e0_FUN_800826e0.c` |

Key offsets established by this cluster:

- manager: vtable `+0`, layer handles `+4 + stage*0xC`, model handles
  `+0x34 + stage*0xC`, texture handles `+0x64 + stage*0x10`, mesh pointers
  `+0xA4 + stage*0x10`, names `+0xE4 + stage*0x80 + unit*0x20`, archive
  `+0x2E4`, resources `+0x2E8 + stage*0x10`;
- scene: x origin `+0x38`, z origin `+0x3C`, direction bytes `+0x80`/`+0x81`,
  cleared window `+0x84..+0x90`;
- tile map: vtable `0x8030FE40`, embedded buffer owner `+8`, array pointers
  `+0x28`/`+0x2C`, width `+0x30`, height `+0x32`, ids `+0x24`/`+0x26`,
  extra id `+0x34`, three cleared dwords `+0x38`.

## Course sections and object ids (`src/resource/course.rs`)

| address | name | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x80081AB0` | [`object_id_remap`] | remaps ids per two version flags and a camera-owner byte | translated | medium | `80081ab0_FUN_80081ab0.c` |
| `0x80082140` | [`bg_layer_write_u16`] | writes one `u16` through a loaded bgdat layer | translated | high | `80082140_FUN_80082140.c` |
| `0x800822E0` | [`object_tile_id`] | optionally remaps an id, then masks it to 10 bits | translated | medium | `800822e0_FUN_800822e0.c` |
| `0x8008E310` | [`section1_entry_by_id`] | finds a 0x18-byte section-1A entry by `(u16@+0x10) & 0xFF` | translated | medium | `8008e310_FUN_8008e310.c` |
| `0x8008E350` | [`section2_entry_by_id`] | finds a 0x18-byte section-2A entry by leading `u16` | translated | medium | `8008e350_FUN_8008e350.c` |
| `0x8008E390` | [`section2b_entry_by_id`] | finds a 0x18-byte section-2B entry by leading `u16` | translated | medium | `8008e390_FUN_8008e390.c` |
| `0x8008E3D0` | [`section3_entry_by_id`] | finds a 0x14-byte section-3A entry by byte `+8` | translated | medium | `8008e3d0_FUN_8008e3d0.c` |
| `0x8008E410` | [`section4b_entry_by_id`] | finds a 0x18-byte section-4B entry by byte `+0xC`, optional xyz w output | translated | medium | `8008e410_FUN_8008e410.c` |
| `0x8008E4D0` | [`section4b_byte_c`] | returns byte `+0xC` of a section-4B entry | translated | high | `8008e4d0_FUN_8008e4d0.c` |
| `0x8008E500` | [`section4b_byte_d`] | returns byte `+0xD` of a section-4B entry | translated | high | `8008e500_FUN_8008e500.c` |
| `0x8008E650` | [`section4b_byte_12`] | returns byte `+0x12` of a section-4B entry | translated | high | `8008e650_FUN_8008e650.c` |
| `0x8008E680` | [`section4b_byte_13`] | returns byte `+0x13` of a section-4B entry | translated | high | `8008e680_FUN_8008e680.c` |
| `0x8008E770` | [`section4b_entry_at_point`] | returns the section-4B area containing a float point | translated | medium | `8008e770_FUN_8008e770.c` |
| `0x8008E800` | [`section4b_entry_overlapping`] | returns the section-4B area overlapping a short box | translated | medium | `8008e800_FUN_8008e800.c` |

The section table indices come from the layout written by
`parse_course_file` (`0x8008DFF0`): section `k` has pointers at
`+0x08 + 8k` / `+0x0C + 8k`, counts at `+0x40 + 8k` / `+0x44 + 8k` and row
counts at `+0x78 + 8k` / `+0x7C + 8k`. The lookups use:

| lookup | count | base | stride | id field |
|---|---|---|---|---|
| `section1_entry_by_id` | `+0x80` (1A rows) | `+0x10` (1A ptr) | `0x18` | `u16@+0x10` low byte |
| `section2_entry_by_id` | `+0x88` (2A rows) | `+0x18` (2A ptr) | `0x18` | `u16@+0` |
| `section2b_entry_by_id` | `+0x8C` (2B rows) | `+0x1C` (2B ptr) | `0x18` | `u16@+0` |
| `section3_entry_by_id` | `+0x90` (3A rows) | `+0x20` (3A ptr) | `0x14` | `u8@+8` |
| `section4b_entry_by_id` | `+0x9C` (4B rows) | `+0x2C` (4B ptr) | `0x18` | `u8@+0xC` |
| `section4b_entry_at_point` | `+0x9C` | `+0x2C` | `0x18` | rectangle contains point |
| `section4b_entry_overlapping` | `+0x9C` | `+0x2C` | `0x18` | rectangle overlaps box |

## The object-id remap

`object_id_remap` is the only place in this batch that changes an id:

```text
flags = *(r13-0x5680)
if flags+4 != 0:
    if flags+4 & 1:
        0x1E -> 0x30, 0x30 -> 0x1E, 0x2E -> 0x2F
    if flags+4 & 2:
        0x3B -> 0x37, 0x3C -> 0x37
if (*(r13-0x58B0))->0x90AA8 != 0:
    0x3B -> 0x37, 0x3C -> 0x37
```

The two flag bits and the camera byte are observed; their **meaning**
(version, region, or a scene setting) is unknown. `object_tile_id` then masks
the result to 10 bits, matching the tile/object id range used by the actor
spawner.

## Unknowns carried forward

- `0x80082240`: the Ghidra body is wrapped in paired-single `GQR0`/`ldexpf`
  scaffolding. The real float comparisons and the `right - left` adjustment
  are translated; the paired-single register-pair handling is not reproduced.
  The status is `partial` for this reason.
- `0x80082410`: the second argument is printed as `extraout_r4` because the
  prologue helper hides `r4`. The `u16` store at `+0x24` identifies it; the
  allocator object type (`&DAT_80377C48`) and its virtual `+0x14` slot are
  inferred from the call site.
- `0x800826E0`: four callers pass a tile-map pointer, but the body never reads
  it. The function is kept as a no-argument getter and returns the `r3:r4`
  float pair as `[x, z]`.
- `0x80082310`: the class that owns vtable `0x8030FE40` is not named; the
  constructor shape (base ctor, embedded buffer owner, three clears) is
  certain.
- `0x80081070`: bytes `+0x324..+0x327` of the manager are not written by the
  constructor; their use is unknown.
- Section names `1A`, `2A`, `2B`, `3A`, `4B` are neutral labels for the seven
  pointer/count pairs, not recovered Nintendo names. The 4B rectangle units
  and the meaning of bytes `+0xC..+0x13` are unknown.
- The two section-search helpers `0x8008E950` (point-in-rectangle) and
  `0x8008EA20` (rectangle overlap) are called by address and are not
  translated in this pass.
