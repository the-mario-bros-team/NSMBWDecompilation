# Coverage: wave 9 BRRES/MDL0 payload and material accessors

## How to read this file

This document covers the **fourth group** of BRRES work in
`src/resource/bres.rs`. Wave 4 translated the U8 archive path and the root
dictionary, wave 5 the dictionary internals and typed section accessors, and
wave 7 the deferred texture/palette bind pass. Wave 9 fills the remaining
payload-adjacent gap between `0x8023B400` and `0x8023F890`:

1. the **alternate indexed accessors** for materials, shaders, texture
   references, the `+0x30` dictionary and polygons;
2. the **MDL0 model binder** at `0x8023BB60`, which walks eight model
   dictionaries in a fixed order;
3. the **polygon-entry accessors** that resolve a polygon's material, shader,
   texture reference and `+0x2C` entry through its owning model;
4. the **`+0x30` entry helpers** and the per-material **texture-state
   accessors** used by the GX draw path;
5. the two **TEX0 descriptor readers** (`0x8023EB30`/`0x8023EBF0`);
6. the **`AnmTexPat` slot clear** at `0x8023F890`.

Every row has:

- **address** — the Ghidra entry address (virtual address of `sys/main.dol`);
- **name** — the Rust function in `src/resource/bres.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the recorded control flow is present;
- **confidence** — `high` when offsets, guards and call order were read
  directly from the Ghidra C; `medium` when a role or field meaning had to be
  inferred from a caller or from the GX call shape;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read directly from the
Ghidra C, `// inferred:` marks a role reconstructed from call sites, and
`// unknown:` marks an unresolved field.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions added in this wave | 24 |
| Bytes of Ghidra function size | 3,648 |
| `translated` | 24 |
| `partial` | 0 |
| `stub` | 0 |
| High confidence | 15 |
| Medium confidence | 9 |
| Low confidence | 0 |
| Destination | `src/resource/bres.rs` |
| Total translated Ghidra functions in the module after this wave | 96 |

## Structures used by this wave

The model header offsets are the same ones documented in
`docs/COVERAGE_WAVE7_BRRES.md`. Wave 9 additionally proves that the model
binder walks the `+0x28` and `+0x2C` words as dictionaries even though both
are zero in every sampled shipped model.

The `+0x30` entry and its 0x34-byte material records are used as follows:

| Offset | Meaning in this wave |
|---|---|
| entry `+0x04` | entry count (shared with the dictionary header) |
| entry `+0x28` | offset to a transform/state block (binder input) |
| entry `+0x34` | offset to a second draw-state block |
| entry `+0x3C` | offset to a 0x180-byte draw cache |
| material `+0x08` | TEX0 pointer or offset, written by `0x8023E390` |
| material `+0x0C` | palette pointer, written by `0x8023E390` |
| material `+0x10` | texture slot index (0..7) |
| material `+0x1C` / `+0x1D` | state bytes forwarded to the GX FIFO |

A polygon entry (from the model `+0x38` dictionary, resolved by `0x8023B9B0`
or `0x8023B950`) references its owning model and the material records it uses:

| Offset | Meaning in this wave |
|---|---|
| polygon `+0x04` | offset from the polygon entry to its owning model |
| polygon `+0x48` | material index |
| polygon `+0x4A` | shader index (`-1` = none) |
| polygon `+0x4C` / `+0x4E` | two texture-reference indices (`-1` = none) |
| polygon `+0x62` | `+0x2C` dictionary index (`-1` = none) |

`// unknown:` the G3D-side names of the `+0x28` and `+0x2C` dictionaries and
the purpose of the 0x180-byte block. The accessor formulas are literal; the
semantic names are not invented.

## The bind order in `mdl_model_bind`

```text
mdl_model_bind(model)
  ├─ +0x30 entries -> mdl_entry_clear_draw_cache (0x8023E320)
  ├─ +0x38 entries -> 0x8023C280 (polygon display-list builder)
  ├─ +0x18 entries -> 0x8023EA20 (cache publish)
  ├─ +0x1C entries -> 0x8023EA40
  ├─ +0x20 entries -> 0x8023EA60
  ├─ +0x24 entries -> 0x8023EA80
  ├─ +0x28 entries -> 0x8023EAA0
  └─ +0x2C entries -> 0x8023EAC0
```

Each loop reads the count at `section + 0x04`, resolves entry `i` with
`section + (i + 1) * 0x10 + 0x14`, and preserves the original order.
`brres_bind_all` now calls `mdl_model_bind` directly instead of `0x8023BB60`
by address.

## Functions

| Address | Rust | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8023B400` | `mdl_material_by_index_alt` | Alternate material-by-index entry point | translated | high | `8023b400_FUN_8023b400.c` |
| `0x8023B500` | `mdl_shader_by_index_alt` | Alternate shader-by-index entry point | translated | high | `8023b500_FUN_8023b500.c` |
| `0x8023B600` | `mdl_texture_by_index_alt` | Alternate texture-reference accessor | translated | high | `8023b600_FUN_8023b600.c` |
| `0x8023B860` | `mdl_section_30_by_index_alt` | Alternate `+0x30` entry accessor | translated | high | `8023b860_FUN_8023b860.c` |
| `0x8023B9B0` | `mdl_polygon_by_index_alt` | Alternate polygon-by-index accessor | translated | high | `8023b9b0_FUN_8023b9b0.c` |
| `0x8023BB60` | `mdl_model_bind` | Bind every payload dictionary of one model | translated | medium | `8023bb60_FUN_8023bb60.c` |
| `0x8023C100` | `mdl_polygon_material` | Resolve and return a polygon's material | translated | medium | `8023c100_FUN_8023c100.c` |
| `0x8023C150` | `mdl_polygon_shader` | Resolve a polygon's shader from `+0x4A` | translated | high | `8023c150_FUN_8023c150.c` |
| `0x8023C1B0` | `mdl_polygon_texture` | Resolve a polygon's texture reference | translated | high | `8023c1b0_FUN_8023c1b0.c` |
| `0x8023C220` | `mdl_polygon_section_2c` | Resolve a polygon's `+0x2C` entry | translated | medium | `8023c220_FUN_8023c220.c` |
| `0x8023E320` | `mdl_entry_clear_draw_cache` | Clear an entry's 0x180-byte draw cache | translated | medium | `8023e320_FUN_8023e320.c` |
| `0x8023E350` | `mdl_entry_section_28` | Return the `+0x28` data pointer | translated | medium | `8023e350_FUN_8023e350.c` |
| `0x8023E370` | `mdl_entry_section_34` | Return the `+0x34` data pointer | translated | medium | `8023e370_FUN_8023e370.c` |
| `0x8023E650` | `mdl_material_reset_binding` | Clear texture/palette pointers and bits | translated | high | `8023e650_FUN_8023e650.c` |
| `0x8023E6C0` | `mdl_material_emit_texture_9` | Emit display-list 9 texture state | translated | medium | `8023e6c0_FUN_8023e6c0.c` |
| `0x8023E700` | `mdl_material_get_texture_9` | Read texture data and `+0x1D` byte | translated | high | `8023e700_FUN_8023e700.c` |
| `0x8023E740` | `mdl_material_publish` | Copy a record's bytes and cache-store | translated | medium | `8023e740_FUN_8023e740.c` |
| `0x8023E7A0` | `mdl_material_emit_texture_10` | Emit display-list 10 texture state | translated | medium | `8023e7a0_FUN_8023e7a0.c` |
| `0x8023E880` | `mdl_material_emit_texture_indexed` | Emit display-list 11/12 texture state | translated | high | `8023e880_FUN_8023e880.c` |
| `0x8023E8C0` | `mdl_material_get_texture_indexed` | Read texture data and `+0x1C` byte | translated | high | `8023e8c0_FUN_8023e8c0.c` |
| `0x8023E960` | `mdl_material_get_texture_10` | Read texture data and `+0x1D` byte | translated | high | `8023e960_FUN_8023e960.c` |
| `0x8023EB30` | `res_texture_descriptor` | Read a TEX0 header with flag bit 0 clear | translated | high | `8023eb30_FUN_8023eb30.c` |
| `0x8023EBF0` | `res_texture_descriptor_flag_set` | Read a TEX0 header with flag bit 0 set | translated | high | `8023ebf0_FUN_8023ebf0.c` |
| `0x8023F890` | `anm_tex_pat_clear_slots` | Zero an `AnmTexPat`'s cached slots | translated | high | `8023f890_FUN_8023f890.c` |

## Opaque callees

| Callee | Role used here | Why opaque |
|---|---|---|
| `0x8023C280` | build one polygon's GX display lists | large GX command builder, outside this lane |
| `0x8023EA20`..`0x8023EAC0` | cache-publish one payload entry | six identical wrappers; Ghidra passes the entry word as both cache arguments |
| `0x801C4780` | write one value to the GX FIFO | PowerPC/GX primitive |
| `0x8024A530` | copy whole 32-byte units | block-copy primitive |
| `0x801AC4A0` / `0x801AC500` | cache-block store / store-plus-sync | PowerPC cache primitives |
| `0x8023E390` | bind one 0x34-byte material record | eight arguments; `platform::call0..call7` stops at seven |

## Unknowns

- `// unknown:` the G3D names of the `+0x28` and `+0x2C` model dictionaries;
  both are zero in every sampled shipped model.
- `// unknown:` the purpose of the 0x180-byte block cleared by `0x8023E320`.
- `// unknown:` why `0x8023E740` uses the material record's first word as the
  byte count instead of `*(record + 0x04)`.
- `// inferred:` the `+0x04` word of a polygon entry is an offset to its owning
  model; this makes the `mdl_polygon_*` accessors agree with the model
  dictionary accessors they call.
- `// observed:` the `+0x04` word of a 0x34-byte material record is the palette
  pointer, because `0x8023E390` tests it before choosing `0x8023EBF0`.
- `// inferred:` the display-list numbers 9, 10, 11 and 12 in the
  `0x8023E6C0`..`0x8023E960` group come from the first argument passed to
  `0x801C4780`, whose body only stores the third argument.

## Verification

- `cargo check` — exit 0; no diagnostics for `src/resource/bres.rs`. The
  remaining warnings are in other lanes (`src/runtime/bt.rs`,
  `src/runtime/input.rs`, `src/runtime/...`).
- `node tools/doc-lint.mjs src` — `ok: true`, 0 violations.
- Ledger: `ledger/agents/wave9-brres.csv` (24 rows, module `resource::bres`).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic is modelled on the Wii's 32-bit
big-endian PowerPC and must only execute on a target-compatible runtime with
the original memory image mapped.
