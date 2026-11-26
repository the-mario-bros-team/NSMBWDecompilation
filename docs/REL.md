# REL modules

Decompressed from `files/rels/*.rel.LZ` (LZ11) into `work/rels/*.rel`.
Tool: `tools/relinfo.mjs` (JSON per module).

## Header notes (Wii REL)

- `id`, `numSections`, `sectionInfoOffset`, `bssSize`, `relOffset`,
  `impOffset`, `impSize`, `align`, `bssAlign`, `fixSize`
- `nameOffset`/`nameSize` point into a separate `.str` debug file, **not** the
  REL; the embedded name is unavailable. Use the filename.
- Section table entry: offset has LSB = executable flag; clear it for the file
  offset. `offset == 0 && size > 0` marks the uninitialized BSS section.
- Relocations are delta-encoded: `u16 delta, u8 type, u8 section, u32 addend`.
  Types 201/202/203 are NOP/SECTION/STOP control entries; real types observed:
  1 (ADDR32), 4 (ADDR16_LO), 6 (ADDR16_HA), 10 (REL24).

## Summary

| Module | ID | Sections | Text | Data | BSS | Imports | Relocations |
|---|---:|---:|---:|---:|---:|---:|---:|
| d_profileNP | 1 | 18 | 1 | 5 | 0x208 | 5 | 1,203 |
| d_basesNP | 2 | 19 | 1 | 5 | 0x12484 | 4 | 97,524 |
| d_enemiesNP | 3 | 19 | 1 | 5 | 0xB4E0 | 3 | 79,624 |
| d_en_bossNP | 4 | 19 | 1 | 5 | 0x487C | 3 | 20,105 |

`d_basesNP` section map (canonical):

| # | File offset | Size | Role |
|---:|---:|---:|---|
| 0 | 0 | 0 | null |
| 1 | 0x00F0 | 0x1C6004 | text (executable) |
| 2 | 0x1C60F4 | 0x04FC | data |
| 3 | 0x1C65F0 | 0x0C | data |
| 4 | 0x1C6600 | 0xA5FC | data |
| 5 | 0x1D0C00 | 0x51CB8 | data |
| 6 | 0 | 0x12484 | BSS |

## Linker behavior (recovered)

`OSLink` (`0x801B1F20`) / `OSLinkFixed` (`0x801B1F30`) call the static `Link`
(`0x801B1C30`). The caller must place the file image at the destination module
base; `Link` does not copy sections. It validates the module, inserts it in the
module list, fixes section/import tables, applies relocations, and zeroes BSS
through the `memset` wrapper.

Real base recovery: a linked module's section table remains at file offset
`0x4C`, so `base = sectionInfoOffset - 0x4C`. The four canonical bases in this
workspace are synthetic analysis slots, not loader output.

`Relocate` (`0x801B1960`) is the per-importer relocation helper; `Undo`
(`0x801B1F50`) is its inverse. Do not label either one `OSLink`/`OSUnlink`.

## Address lookup

Use `node tools/rel2elf.mjs --address-layout <module.rel|module.rel.LZ>...`
to print validated section metadata as JSON. Save the JSON and pass it to
`node tools/rel2elf.mjs --resolve-address <summary.json> <0xaddress>...`
to recover module IDs, original section indices, and section-relative offsets.
The lookup rejects addresses outside the sections and ambiguous ranges.

## Converted images

`tools/rel2elf.mjs` now produces analysis-ready ELF32 BE `EM_PPC` images:

| Module | Base | Relocations | ELF |
|---|---|---:|---|
| d_profileNP | `0x80500000` | 1,203 | `work/rels/elf/d_profileNP.elf` |
| d_basesNP | `0x80600000` | 97,524 | `work/rels/elf/d_basesNP.elf` |
| d_enemiesNP | `0x80A00000` | 79,624 | `work/rels/elf/d_enemiesNP.elf` |
| d_en_bossNP | `0x80C00000` | 20,105 | `work/rels/elf/d_en_bossNP.elf` |

All relocated with 0 failures/0 unknown types; see `docs/REL2ELF.md` and
`work/rels/elf/summary.json` for validation details. Entry points:
`0x805000E0`, `0x806000F0`, `0x80A000F0`, `0x80C000F0`.

