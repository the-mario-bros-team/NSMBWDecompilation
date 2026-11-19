# Inventory — NSMBW (SMNE01, r37675)

Source: `C:\Users\WitherULTRA7\Downloads\mariobroswii` (read-only)
Manifest: `orig/manifest.sha256.json` — 1,195 files, 335.8 MiB

## Disc layout

| Path | Contents |
|---|---|
| `disc/header.bin` | game code `SMNE01`, title "New SUPER MARIO BROS. Wii" |
| `disc/region.bin` | 32-byte region blob (0x...01) |
| `sys/main.dol` | 3,491,808 bytes, entry `0x80004050` |
| `sys/fst.bin` | 35,628 bytes |
| `sys/apploader.img` | 241,412 bytes |
| `sys/boot.bin`, `sys/bi2.bin` | boot metadata |
| `files/` | 1,195-file FST tree |
| `cert.bin`, `ticket.bin`, `tmd.bin`, `h3.bin` | partition metadata (extraction already decrypted) |

## Content by directory

| Dir | Files | Bytes |
|---|---:|---:|
| Sound | 89 | 192,031,296 |
| Object | 432 | 70,265,024 |
| Replay | 212 | 19,785,676 |
| WorldMap | 56 | 17,357,409 |
| MovieDemo | 33 | 11,345,024 |
| Stage | 214 | 8,789,420 |
| Layout | 63 | 8,612,556 |
| US | 32 | 8,848,720 |
| Effect | 30 | 7,105,684 |
| rels | 4 | 2,209,385 |
| Env | 3 | 962,496 |
| HomeButton2 | 7 | 447,561 |

## Archives (U8)

Index: `work/arcs/index.json`

- 820 `.arc` archives, 2,933 members, 115.3 MiB of member data, 0 parse errors
- Magic `55 AA 38 2D`; header 0x20, nodes 12 bytes, string table after node table
- Root node `size` is the 1-based last node index (= node count); data offset absolute

Known member types (heuristic scan of first 4 bytes):

| Type | Count |
|---|---:|
| TPL | 713 |
| BRRES | 518 |
| BRLAN (`RLAN`) | 448 |
| BRLYT | 94 |
| LZ11-ish | 67 |
| LZ10-ish | 252 |
| MThd (MIDI) | 22 |
| BRFNT | 3 |
| BRSAR / BRSTM / BCSV / BREFF | outside arcs (Sound/) |

Raw `.bin` payloads (course data, bgdat) have arbitrary first words; the
"unknown hex" histogram is expected and will be refined when formats are
parsed in Stage 4.

## Extracted assets

`work/arcs/extracted/` mirrors the archive tree (820 arcs, 2,933 files,
120.9 MB, 0 errors). Known payloads: 518 BRRES, 713 TPL, 94 BRLYT, 448 RLAN,
3 BRFNT. Raw `.bin` payloads remain as-is; no decompression is applied to
nested LZ payloads yet.


## REL modules

`work/rels/*.rel` (LZ11-decompressed):

| Module | ID | Sections | Decompressed |
|---|---:|---:|---:|
| d_profileNP.rel | 1 | 18 | 24,244 |
| d_basesNP.rel | 2 | 19 | 3,019,336 |
| d_enemiesNP.rel | 3 | 19 | 2,137,424 |
| d_en_bossNP.rel | 4 | 19 | 605,552 |

## main.dol → main.elf

`work/dol/main.elf` (ELF32 BE, EM_PPC, entry `0x80004050`)

- 13 PT_LOAD segments, no address overlaps, image ends `0x8042FC3C`
- NSMBW quirk: initialized data sections 6–7 live inside the nominal BSS range
  (`0x80351680 + 0xDE5BC`). The converter splits BSS around them, so those
  bytes are preserved as initialized data and the remaining BSS is zero-filled.
  Consequence: analysis sees the data sections at their real addresses; BSS
  coverage is split, which is recorded as a known limitation.
