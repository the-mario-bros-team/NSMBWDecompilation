# Coverage: wave 7 BRRES model payload and deferred bind

## How to read this file

This document covers the **third group** of BRRES work in
`src/resource/bres.rs`. Wave 4 translated the U8 archive path and the root
dictionary; wave 5 translated the dictionary internals, the typed section
accessors and the first MDL0 accessors. Wave 7 adds:

1. the remaining MDL0 sub-dictionary accessors (materials, shaders, texture
   references, palettes, polygons and the unclassified `+0x2C`/`+0x30`
   dictionaries);
2. the deferred bind pass that turns the texture and palette **names** stored
   in a model material or in an `AnmTexPat` animation into cached TEX0/palette
   pointers;
3. the small TEX0/palette bind and revision helpers that the wave-5
   `brres_bind_all` / `brres_check_revision` previously called by address only.

Every row has:

- **address** — the Ghidra entry address (virtual address of `sys/main.dol`);
- **name** — the Rust function in `src/resource/bres.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present;
- **confidence** — `high` when offsets, calls and return values were read
  directly from the Ghidra C and cross-checked against a shipped model;
  `medium` when an argument, return value or callee role had to be recovered
  from a caller or from the outlined `__savegprlr_*` prologue;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions added in this wave | 22 |
| Bytes of Ghidra function size | 2,488 |
| `translated` | 22 |
| `partial` | 0 |
| `stub` | 0 |
| High confidence | 14 |
| Medium confidence | 8 |
| Low confidence | 0 |
| Destination | `src/resource/bres.rs` |
| Total functions in the module after this wave | 72 |

## MDL0 header offsets verified against a shipped model

The wave-5 code already used `+0x10` (node tree), `+0x14` (nodes) and `+0x18`
(materials). Wave 7 adds the rest. The offsets below were read from
`MovieDemo/Baloon/g3d/model.brres` (extracted under `work/arcs/extracted/`)
and the dictionary keys were printed from the file itself:

| Offset | Dictionary keys observed | Role used here |
|---|---|---|
| `+0x10` | `NodeTree`, `DrawOpa` | node tree (wave 5) |
| `+0x14` | `skl_root`, `door_01`, ... | bones/nodes (wave 5) |
| `+0x18` | `balloon_model__mat_*` | materials (wave 5) |
| `+0x1C` | `balloon_model__mat_*` | shaders; callers index it with the material shader slot |
| `+0x20` | `balloon_model__mat_*` | texture references; one array per material |
| `+0x24` | `#0`..`#6` | palettes |
| `+0x28` | (zero) | not used |
| `+0x2C` | (zero) | unclassified accessor `0x8023B700` |
| `+0x30` | `mat_balloon_*` (short names) | unclassified; owns 0x34-byte material records |
| `+0x34` | `mat_balloon_*` | not translated in this wave |
| `+0x38` | `polygon0`..`polygon5` | polygons |
| `+0x3C` | texture names | not translated in this wave |

A scan of all 518 extracted BRRES files (918 MDL0 bodies) found `+0x30` and
`+0x38` present in every model, `+0x2C` zero in every model, and the `+0x30`
keys always matching material names.

The `+0x30` entry layout used by the bind pass is:

| Offset | Meaning |
|---|---|
| `+0x2C` | number of 0x34-byte material records |
| `+0x30` | offset from the entry base to the record table |
| `+0x40` | texture bitmask pointer word |
| `+0x144` | palette bitmask pointer word |

`// unknown:` the G3D name of the `+0x2C` and `+0x30` sections. They are
described by offset, not by an invented symbol name.

## The deferred bind path

```text
brres_bind_check(brres_word, brres_word)
  ├─ 3DModels(NW4R), per model:
  │    mdl_bind_check(model_word, brres_word)
  │      └─ model+0x30 entry, per entry:
  │           mdl_material_bind_check(entry_word, brres_word)
  │             └─ 0x8023E390 per 0x34-byte record   [opaque, 8 args]
  └─ AnmTexPat(NW4R), per animation:
       anm_tex_pat_bind(animation_word, brres_word)
         ├─ brres_get_texture_record   (wave 5)
         └─ brres_get_palette_record   (wave 5)
```

`0x8023E390` is the only call that cannot be expressed with
`crate::platform`: it takes eight arguments and the helper set stops at
`call7`. It is called by address and its signature is documented in the Rust
comment.

## Functions

| Address | Rust | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8023A350` | `brres_bind_check` | Run the model and `AnmTexPat` bind passes | translated | medium | `8023a350_FUN_8023a350.c` |
| `0x8023B460` | `mdl_material_count` | Material dictionary count | translated | high | `8023b460_FUN_8023b460.c` |
| `0x8023B4A0` | `mdl_shader_by_index` | Shader by index | translated | high | `8023b4a0_FUN_8023b4a0.c` |
| `0x8023B560` | `mdl_shader_count` | Shader dictionary count | translated | high | `8023b560_FUN_8023b560.c` |
| `0x8023B5A0` | `mdl_texture_by_index` | Texture reference by index | translated | high | `8023b5a0_FUN_8023b5a0.c` |
| `0x8023B660` | `mdl_texture_count` | Texture-reference count | translated | high | `8023b660_FUN_8023b660.c` |
| `0x8023B6A0` | `mdl_palette_by_index` | Palette by index | translated | high | `8023b6a0_FUN_8023b6a0.c` |
| `0x8023B700` | `mdl_section_2c_by_index` | Entry of the unknown `+0x2C` dictionary | translated | medium | `8023b700_FUN_8023b700.c` |
| `0x8023B760` | `mdl_section_30_by_name` | `+0x30` entry by C-string name | translated | medium | `8023b760_FUN_8023b760.c` |
| `0x8023B7B0` | `mdl_section_30_by_record` | `+0x30` entry by name record | translated | medium | `8023b7b0_FUN_8023b7b0.c` |
| `0x8023B800` | `mdl_section_30_by_index` | `+0x30` entry by index | translated | high | `8023b800_FUN_8023b800.c` |
| `0x8023B8C0` | `mdl_section_30_count` | `+0x30` dictionary count | translated | high | `8023b8c0_FUN_8023b8c0.c` |
| `0x8023B900` | `mdl_polygon_by_name` | Polygon by C-string name | translated | high | `8023b900_FUN_8023b900.c` |
| `0x8023B950` | `mdl_polygon_by_index` | Polygon by index | translated | high | `8023b950_FUN_8023b950.c` |
| `0x8023BA10` | `mdl_polygon_count` | Polygon dictionary count | translated | high | `8023ba10_FUN_8023ba10.c` |
| `0x8023BA50` | `mdl_bind_check` | Check every `+0x30` entry of a model | translated | medium | `8023ba50_FUN_8023ba50.c` |
| `0x8023E250` | `mdl_material_bind_check` | Check every 0x34-byte material record | translated | medium | `8023e250_FUN_8023e250.c` |
| `0x8023EAE0` | `res_palette_bind` | Cache-publish one palette | translated | medium | `8023eae0_FUN_8023eae0.c` |
| `0x8023EB00` | `res_palette_revision_ok` | Accept palette revision 1 or 3 | translated | high | `8023eb00_FUN_8023eb00.c` |
| `0x8023ECB0` | `res_texture_revision_ok` | Accept texture revision 1 or 3 | translated | high | `8023ecb0_FUN_8023ecb0.c` |
| `0x8023ECE0` | `res_texture_bind` | Cache-flush one texture | translated | high | `8023ece0_FUN_8023ece0.c` |
| `0x8023F720` | `anm_tex_pat_bind` | Resolve texture/palette names of one `AnmTexPat` | translated | medium | `8023f720_FUN_8023f720.c` |

## What was reused instead of duplicated

- All indexed accessors call the existing `mdl_section_entry`; all count
  accessors call the new internal `mdl_section_count`.
- Named and record lookups call `res_dict_get_named_value` /
  `res_dict_get_record_value`, so no dictionary walk is repeated.
- `anm_tex_pat_bind` calls the wave-5 `brres_get_texture_record` and
  `brres_get_palette_record`.
- `brres_bind_check` calls `brres_section_count` / `brres_section_entry` and
  the translated `mdl_bind_check` / `anm_tex_pat_bind`, not raw addresses.

## Opaque callees

| Callee | Role used here | Why opaque |
|---|---|---|
| `0x8023E390` | bind one 0x34-byte material record | eight arguments; helper set stops at `call7` |
| `0x801AC4A0` | cache-block store + `syscall` | PowerPC cache primitive |
| `0x801AC4D0` | cache-block flush | PowerPC cache primitive |
| `0x801AC500` | cache-block store | PowerPC cache primitive |
| `0x8023BB60` | bind one full MDL0 model | G3D model-side binding, outside this lane |

## Unknowns

- `// unknown:` the role of the MDL0 dictionaries at `+0x2C` and `+0x30`.
  The accessor formulas are literal; the semantic names are not invented.
- `// inferred:` `brres_bind_check` receives the BRRES-word address in both
  `r3` and `r4`; the child bind functions dereference it once before calling
  the BRRES lookups.
- `// unknown:` the `mode != 0` branch of `res_palette_bind` calls
  `0x801AC4A0` with no arguments in the Ghidra output, although the callee
  itself takes `(data, size)`. The Rust body keeps the no-argument call.

## Verification

- `cargo check` — exit 0; no diagnostics for `src/resource/bres.rs`. The
  remaining warnings are in other lanes (`src/runtime/bt.rs`,
  `src/runtime/input.rs`).
- `node tools/doc-lint.mjs src` — `ok: true`, 0 violations.
- Ledger: `ledger/agents/wave7-brres.csv` (22 rows, module `resource::bres`).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic is modelled on the Wii's 32-bit
big-endian PowerPC and must only execute on a target-compatible runtime with
the original memory image mapped.
