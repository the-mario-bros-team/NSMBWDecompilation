# Coverage: wave 5 BRRES dictionary and MDL0 accessors

## How to read this file

This document covers the **second half** of the BRRES loader reconstruction in
`src/resource/bres.rs`. Wave 5 added the NW4R `ResDict` implementation, the
typed BRRES section accessors, the revision check, the bind pass, and the MDL0
payload accessors.

Addresses are **virtual addresses of `sys/main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. Status follows `docs/READING_GUIDE.md`:
`translated` means the recorded control flow and every observable call are
present; `partial` and `stub` are not used here.

Confidence measures the evidence, not the code style. `high` means the flow,
offsets and calls were read directly from the Ghidra C and cross-checked
against a shipped `.brres` file. `medium` means one argument, return type or
callee role had to be recovered from call sites or the outlined
`__savegprlr`/`__restgprlr` prologue.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions added in this wave | 25 |
| Bytes of Ghidra function size | 5,620 |
| `translated` | 25 |
| `partial` | 0 |
| `stub` | 0 |
| High confidence | 20 |
| Medium confidence | 5 |
| Low confidence | 0 |
| Destination | `src/resource/bres.rs` |
| Total functions in the module after this wave | 50 |

The selection is the dictionary machinery that the wave-4 loader called
through `0x80239CC0`/`0x80239C40`, plus the public section accessors that
actor code uses to reach models, textures, palettes and animations. The last
group exposes the MDL0 sub-dictionaries for node trees, bones and materials.

## Dictionary model

The root dictionary starts at BRRES file `+0x18`. Its header is:

| Offset | Meaning |
|---|---|
| `+0x00` | `u32` size of the dictionary body |
| `+0x04` | `u32` entry count (sentinel excluded) |
| `+0x08` | node 0, the sentinel |
| `+0x0C` | `u16` root node index |

Every node is 16 bytes:

| Offset | Meaning |
|---|---|
| `+0x00` | `u16` hash / bit index |
| `+0x02` | `u16` padding |
| `+0x04` | `u16` left child index |
| `+0x06` | `u16` right child index |
| `+0x08` | `u32` offset to the name string (length word is four bytes earlier) |
| `+0x0C` | `u32` offset to the resource body |

The tree walk uses the hash as a bit index into the name. A set bit selects
the right child; a clear bit or an out-of-range index selects the left child.
The candidate is accepted only when the name record matches exactly.

This model was verified against `Env/Env_course/scene/scene.brres` and
`MovieDemo/Baloon/g3d/model.brres`; the latter supplied the `NodeTree`,
`DrawOpa`, `skl_root`, `mat_*` and polygon names used to identify the MDL0
sub-dictionary offsets.

## Functions

| Address | Rust | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80239930` | `res_dict_compare` | Compare two length-prefixed name records | translated | high | `80239930_FUN_80239930.c` |
| `0x80239A80` | `res_dict_find` | Walk the dictionary tree for a name-record key | translated | high | `80239a80_FUN_80239a80.c` |
| `0x80239B60` | `res_dict_find_name` | Walk the tree for a C-string name and length | translated | high | `80239b60_FUN_80239b60.c` |
| `0x80239C40` | `res_dict_get_named` | Named resource body from a dictionary | translated | high | `80239c40_FUN_80239c40.c` |
| `0x80239CC0` | `res_dict_get_section` | Section/resource body from a tag record | translated | high | `80239cc0_FUN_80239cc0.c` |
| `0x80239D30` | `res_dict_name_index` | Zero-based index of a named resource | translated | high | `80239d30_FUN_80239d30.c` |
| `0x80239DC0` | `res_dict_tag_index` | Zero-based index of a tag-record resource | translated | high | `80239dc0_FUN_80239dc0.c` |
| `0x80239EA0` | `brres_get_palette_record` | Palette entry by length-prefixed name | translated | high | `80239ea0_FUN_80239ea0.c` |
| `0x80239F20` | `brres_get_texture` | Texture entry by C-string name | translated | high | `80239f20_FUN_80239f20.c` |
| `0x80239F90` | `brres_get_texture_record` | Texture entry by length-prefixed name | translated | high | `80239f90_FUN_80239f90.c` |
| `0x8023A010` | `brres_texture_by_index` | Texture entry by index | translated | high | `8023a010_FUN_8023a010.c` |
| `0x8023A0B0` | `brres_get_anm_chr` | `AnmChr(NW4R)` entry by name | translated | high | `8023a0b0_FUN_8023a0b0.c` |
| `0x8023A120` | `brres_get_anm_vis` | `AnmVis(NW4R)` entry by name | translated | high | `8023a120_FUN_8023a120.c` |
| `0x8023A190` | `brres_get_anm_clr` | `AnmClr(NW4R)` entry by name | translated | high | `8023a190_FUN_8023a190.c` |
| `0x8023A200` | `brres_get_anm_tex_pat` | `AnmTexPat(NW4R)` entry by name | translated | high | `8023a200_FUN_8023a200.c` |
| `0x8023A270` | `brres_get_anm_tex_srt` | `AnmTexSrt(NW4R)` entry by name | translated | high | `8023a270_FUN_8023a270.c` |
| `0x8023A590` | `brres_bind_all` | Bind every model, texture and palette | translated | medium | `8023a590_FUN_8023a590.c` |
| `0x8023A860` | `brres_check_revision` | Validate every present section revision | translated | medium | `8023a860_FUN_8023a860.c` |
| `0x8023B1B0` | `mdl_get_node_tree` | Named `NodeTree`/`DrawOpa` lookup | translated | medium | `8023b1b0_FUN_8023b1b0.c` |
| `0x8023B200` | `mdl_get_node` | Named bone/node lookup | translated | medium | `8023b200_FUN_8023b200.c` |
| `0x8023B250` | `mdl_get_node_record` | Bone/node by length-prefixed name | translated | medium | `8023b250_FUN_8023b250.c` |
| `0x8023B2A0` | `mdl_node_by_index` | Bone/node by index | translated | high | `8023b2a0_FUN_8023b2a0.c` |
| `0x8023B300` | `mdl_node_by_index_alt` | Duplicate bone/node-by-index entry point | translated | high | `8023b300_FUN_8023b300.c` |
| `0x8023B360` | `mdl_node_count` | Bone/node dictionary count | translated | high | `8023b360_FUN_8023b360.c` |
| `0x8023B3A0` | `mdl_material_by_index` | Material by index | translated | high | `8023b3a0_FUN_8023b3a0.c` |

## What was reused instead of duplicated

The new public functions call the translated dictionary helpers directly:

- section accessors use `brres_section_value`, then `res_dict_get_named_value`;
- indexed accessors use `brres_section_entry` / `mdl_section_entry`;
- the revision check and bind pass share `brres_section_count` and
  `brres_section_entry`;
- the existing wave-4 `fn_80239e30` now calls `res_dict_get_record_value` and
  `res_dict_get_named_value` instead of the two old opaque addresses.

`src/resource/registry.rs` already calls `bres::fn_800df570`,
`bres::fn_800df640`, `bres::fn_802b76c0`, `bres::fn_802b7760` and
`bres::fn_802b7770`; those entry points are unchanged. Archive-object release
remains the wave-4 `fn_802b7680` pair; this wave adds the per-resource bind
pass that pairs with the engine's G3D state.

## Opaque callees

These are called by absolute address and are deliberately outside this lane:

| Callee | Role used here | Why opaque |
|---|---|---|
| `0x8023BB60` | bind one MDL0 model | G3D model-side binding |
| `0x8023ECE0` | bind one TEX0 texture | G3D texture-side binding |
| `0x8023EAE0` | bind one palette | G3D palette-side binding |
| `0x8023ECB0` | validate one TEX0 entry | texture validation |
| `0x8023EB00` | validate one palette entry | palette validation |
| `0x802DC69C` | `strlen` | libc |
| `0x802E1AB4` | `strcmp` | libc |

## Unknowns

- `// inferred:` `brres_bind_all` and `brres_check_revision` have no Ghidra
  parameters because `r3` is preserved across the outlined prologue at
  `0x802DCD74`. They are modelled with an explicit `this` argument.
- `// inferred:` `mdl_get_node_tree`, `mdl_get_node` and `mdl_get_node_record`
  lose their name argument (or return value) in the Ghidra output; call sites
  supply the missing piece.
- `// unknown:` the five binding/validation routines above are not translated;
  their exact side effects are G3D-engine state, not BRRES container state.

## Verification

- `cargo check` — no diagnostics for `src/resource/bres.rs`; the workspace
  still has transient errors from other lanes' files, which are outside this
  lane. A standalone `rustc` harness over `bres.rs` plus the `platform` API
  compiles cleanly.
- `node tools/doc-lint.mjs src` — `ok: true`, 0 violations.
- Ledger: `ledger/agents/wave5-brres.csv` (25 rows, module `resource::bres`).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic is modelled on the Wii's 32-bit
big-endian PowerPC. The functions must only be executed on a target-compatible
runtime with the original memory image mapped.
