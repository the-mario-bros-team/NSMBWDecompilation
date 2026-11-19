# Coverage: wave 18 BRRES/MDL0 material and texture state payload

## How to read this file

This document covers the **wave-18 group** of BRRES work in
`src/resource/bres.rs`. Waves 4, 5, 7 and 9 translated the U8 archive path,
the BRRES dictionaries, the typed section accessors, the deferred bind pass and
the first MDL0 payload accessors. Wave 18 continues inside the same address
neighbourhood, `0x8023CB60..0x8023EE90`, and adds the material/draw-state
payload that sits between those earlier groups:

1. the eight-entry **texture and palette object caches** (present bitsets and
   compact copies);
2. the per-material **texture-matrix and texgen setters**;
3. the **texture-coordinate and colour state** getters and BP emitters;
4. the **texture/palette cache publish** pair and the indexed texture
   display-list emitter;
5. the **animation frame-ratio** helper;
6. the two **material channel-state** writers, plus the record/sub-record
   helpers that sit between them.

Each row has:

- **address** - the Ghidra entry address (virtual address of `sys/main.dol`);
- **name** - the Rust function in `src/resource/bres.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the recorded control flow is present;
- **confidence** - `high` when offsets, guards and call order were read
  directly from the Ghidra C; `medium` when a role or field meaning had to be
  inferred from a caller or from the surrounding accessor family;
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C or the ELF, `// inferred:` marks a role reconstructed from call sites, and
`// unknown:` marks an unresolved field.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions added in this wave | 25 |
| Bytes of Ghidra function size | 3,228 |
| `translated` | 25 |
| `partial` | 0 |
| `stub` | 0 |
| High confidence | 14 |
| Medium confidence | 11 |
| Low confidence | 0 |
| Destination | `src/resource/bres.rs` |
| Total translated Ghidra functions in the module after this wave | 121 |

## Structures used by this wave

The cache accessors all take `this` as the address of a word that holds the
cache base pointer, matching the pointer-to-pointer convention already used by
the earlier BRRES accessors.

| Cache | Header | Slot stride | Slots | Used by |
|---|---|---|---|---|
| texture objects | present bitset word at `+0x00` | `0x20` | 8 | `res_texture_cache_copy_present` |
| palette objects | present bitset word at `+0x00` | `0x0C` | 8 | `res_palette_cache_present` |
| palette block | present bitset word at `+0x00` | 0x20/0x40/0x60 total | - | `res_palette_cache_copy` |
| state table | first two words copied | `0x14` words / `0xD` bytes | 8 | `res_palette_table_copy` |

The 0x34-byte material record already documented in
`docs/COVERAGE_WAVE9_BRRES.md` is reused here:

| Offset | Meaning in this wave |
|---|---|
| `+0xA8` | texgen source byte (0..0x1F, 0xFF = none) |
| `+0xA9` | texgen kind byte (0..0x7F, 0xFF = none) |
| `+0xAA` | texgen selector byte (0..0xFF) |
| `+0xAB` bit 0 | identity-matrix flag |
| `+0xAC` | 3x4 texture matrix, written by `mdl_texmtx_set` |

The `r2` constants used by `anm_frame_ratio` and the two channel writers were
resolved from the ELF: `__init_registers` loads `r2 = 0x80433080`, so the
negative offsets in the Ghidra output name `0x8042E510..0x8042E568`. The values
are `f32` 0.0, the `f64` 2^52 conversion bias, the `f64` easing exponent
0.6000000238418579, `f32` 1.0, `f32` 0.7111111283302307 and `f32` 0.0.

A material channel-state object (`*this` in the last two functions) uses:

| Offset | Meaning in this wave |
|---|---|
| `+0x14` | channel flags: bit 1, 2, 3, 4, 5, 6 select payload fields |
| `+0x20` / `+0x24` / `+0x28` | first three-component colour |
| `+0x2C` / `+0x30` / `+0x34` | three scale values, multiplied by 0.7111111 |
| `+0x38` / `+0x3C` / `+0x40` | second three-component colour |

The output state block is indexed as words: `[0]` mask, `[1..3]` first colour,
`[4..6]` scale, `[7..]` 3x4 matrix, `[10]`/`[0xE]`/`[0x12]` second colour.

## Functions

| Address | Rust | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8023C950` | `res_texcoord_command_emit` | Emit the table-selected command word for a texcoord record | translated | medium | `8023c950_FUN_8023c950.c` |
| `0x8023CA40` | `res_subresource_ptr` | Resolve the offset at `record + 4` against the record base | translated | high | `8023ca40_FUN_8023ca40.c` |
| `0x8023CB60` | `res_texture_object_copy` | Copy the 0x20-byte texture object referenced by `*this` | translated | high | `8023cb60_FUN_8023cb60.c` |
| `0x8023CD00` | `res_texture_cache_copy_present` | Copy every present texture object into a compact array | translated | high | `8023cd00_FUN_8023cd00.c` |
| `0x8023CDF0` | `res_palette_cache_present` | Test one palette-cache present bit | translated | high | `8023cdf0_FUN_8023cdf0.c` |
| `0x8023CE30` | `res_palette_cache_copy` | Copy the palette block selected by the present bitset | translated | medium | `8023ce30_FUN_8023ce30.c` |
| `0x8023CEC0` | `res_palette_table_copy` | Copy the eight-entry state table from the saved GX state | translated | medium | `8023cec0_FUN_8023cec0.c` |
| `0x8023CFA0` | `mdl_texmtx_set` | Set or clear one material's 3x4 texture matrix | translated | high | `8023cfa0_FUN_8023cfa0.c` |
| `0x8023D020` | `mdl_texgen_set` | Store a material's three texgen selector bytes | translated | high | `8023d020_FUN_8023d020.c` |
| `0x8023D120` | `res_texcoord_set_index` | Store the clamped index byte of a texcoord record | translated | high | `8023d120_FUN_8023d120.c` |
| `0x8023D150` | `res_texcoord_index` | Return the signed index byte of a texcoord record | translated | high | `8023d150_FUN_8023d150.c` |
| `0x8023D1B0` | `res_texcoord_pair` | Read one of three indexed byte pairs | translated | medium | `8023d1b0_FUN_8023d1b0.c` |
| `0x8023D200` | `res_texcoord_copy` | Copy the 12-byte texcoord record header | translated | high | `8023d200_FUN_8023d200.c` |
| `0x8023D240` | `anm_frame_ratio` | Return the normalized animation frame position | translated | medium | `8023d240_FUN_8023d240.c` |
| `0x8023D300` | `res_texcoord_get` | Read one 0x14-byte texcoord record into four bytes | translated | high | `8023d300_FUN_8023d300.c` |
| `0x8023D3C0` | `res_texcoord_emit` | Emit four GX BP commands for a texcoord record | translated | medium | `8023d3c0_FUN_8023d3c0.c` |
| `0x8023D460` | `res_color_get` | Read one 10-byte colour record into four bytes | translated | high | `8023d460_FUN_8023d460.c` |
| `0x8023D520` | `res_color_emit` | Emit two GX BP commands for a colour record | translated | medium | `8023d520_FUN_8023d520.c` |
| `0x8023DD70` | `res_record_copy_40` | Copy ten words out of the record referenced by `*this` | translated | high | `8023dd70_FUN_8023dd70.c` |
| `0x8023E7E0` | `res_texture_get_data_byte` | Read a texture data pointer and `+0x1D` byte for display list 8 | translated | high | `8023e7e0_FUN_8023e7e0.c` |
| `0x8023E820` | `res_texture_publish` | Copy a texture image and run the cache-block store | translated | medium | `8023e820_FUN_8023e820.c` |
| `0x8023E900` | `res_palette_publish` | Copy a palette image and run the cache-block store | translated | medium | `8023e900_FUN_8023e900.c` |
| `0x8023E9A0` | `res_texture_emit_indexed` | Emit display list 9 for one indexed texture entry | translated | high | `8023e9a0_FUN_8023e9a0.c` |
| `0x8023ECF0` | `mdl_material_channel_sync` | Merge one material's channel state into a dirty-mask block | translated | medium | `8023ecf0_FUN_8023ecf0.c` |
| `0x8023EE90` | `mdl_material_channel_apply` | Write one material's channel state into a fresh block | translated | medium | `8023ee90_FUN_8023ee90.c` |

## Opaque callees

| Callee | Role used here | Why opaque |
|---|---|---|
| `0x8024A530` | copy whole 32-byte units | block-copy primitive already documented in wave 9 |
| `0x801AC4A0` | cache-block store plus sync | PowerPC cache primitive |
| `0x801C04D0` | write a 3x4 identity matrix | GX matrix primitive |
| `0x801C0500` | copy a 3x4 matrix | GX matrix primitive |
| `0x802382B0` | build a scaled 3x4 matrix from three doubles | matrix helper shared with the GX path |
| `0x80239980` | write one five-byte GX BP command | GX command primitive; body stores `0x61` then the word |
| `0x801C4780` | write one value to the GX FIFO | PowerPC/GX primitive |
| `0x801C95E0` | write one value through the synchronous GX FIFO writer | GX primitive; checks the stream object and flushes when needed |
| `0x801A9B90` | execute a bare `syscall` | PowerPC primitive |
| `0x802F84D0` | read-only command table for `res_texcoord_command_emit` | data table, not a function |
| `0x802DCD70` / `0x802DCDBC` | save / restore GX state | GX state pair; the `u64` result splits into two pointers |
| `0x802E55A4` | `pow(base, exponent)` for `f64` | math library routine reached through a Ghidra thunk |

## Unknowns

- `// unknown:` the official NW4R names of the texture/palette cache records;
  the "texture object" and "palette object" labels are `// inferred:` from the
  0x20/0xC strides and the present bitsets.
- `// unknown:` the exact BP register roles selected by the 0xE0/0xE1 bytes in
  the texcoord and colour emitters. The command words are reproduced literally.
- `// inferred:` `anm_frame_ratio` is an animation frame helper; its callers
  are `0x80244360` and `0x8024E180`, and mode 1 applies a 0.6 easing exponent.
- `// inferred:` the `+0x14` flag bits in the material channel state select
  colour, matrix and second-colour payloads; only their effects are observed.
- The three very large key-sampling functions at `0x8023F000`, `0x8023F4E0`
  and `0x8023F5A0` are deliberately **not** part of this wave: their Ghidra
  output depends on caller-set GQR quantization, which the current platform
  helpers cannot model faithfully. They remain untranslated and are not in the
  ledger.

## Verification

- `cargo check` - exit 0; no diagnostics for `src/resource/bres.rs`.
- `node tools/doc-lint.mjs src` - `ok: true`, 0 violations.
- Ledger: `ledger/agents/wave18-brres.csv` (25 rows, module `resource::bres`).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic is modelled on the Wii's 32-bit
big-endian PowerPC and must only execute on a target-compatible runtime with
the original memory image mapped.
