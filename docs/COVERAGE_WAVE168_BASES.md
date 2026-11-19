# Coverage - Wave 168 (`game::bases` 300-332 byte band, part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 7 | 2,252 | 6 | 1 |

This is the first batch from the 300-512 byte band. The band contains 226
untranslated bodies (87,856 bytes); this pass takes seven readable bodies from
its smallest sub-band.

## What this pass covers

- **Colour-table renderer helpers (2 bodies).** `0x8062C360` lazily fills the
  two 12-byte `.bss` colour tables at `0x80825F58`/`0x80825F68` and pushes one
  row to the `+0x40C` renderer with selectors `0` and `2`. `0x8062C4B0` sets up
  the `0x80429F7C` context, plays one of three sound ids and calls
  `0x8062C360` with the matching row.
- **Padded box test (1 body).** `0x80697DC0` compares a point against the actor
  centre `+0xE8`/`+0xEC` and half extents `+0xAC`/`+0xB0`, with `20.0`, `16.0`,
  `36.0`, `24.0` and `0.5` constants from `0x807CA004..0x807CA050`.
- **State callbacks (3 bodies).** `0x807C25B0` dispatches one of five
  `0x80834640..0x80834A80` descriptors from `+0x550`. `0x807C2A00` and
  `0x807C2E50` are the same body with descriptors `0x80834740` and
  `0x808348C0`; the shared Rust helper is `fn_807c2a_like`.
- **Lane transform spawn (1 body).** `0x80637910` selects one of the four
  `+0x6E8` records from the `+0x8D0` lane tag, copies twelve words into a
  transform, runs `0x800B37E0` and spawns `0x807DF460` through `0x8016C890`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8062C360` | `fn_8062c360` | 328 | high |
| `0x8062C4B0` | `fn_8062c4b0` | 312 | high |
| `0x80697DC0` | `fn_80697dc0` | 328 | high |
| `0x807C25B0` | `fn_807c25b0` | 300 | high |
| `0x807C2A00` | `fn_807c2a00` | 328 | high |
| `0x807C2E50` | `fn_807c2e50` | 328 | high |
| `0x80637910` | `fn_80637910` | 300 | medium |

## Corrections and notes

- `0x80637910`'s Ghidra expression
  `((int)~(tag - 2 | 2 - tag) >> 0x1f) + 8` hides the raw sequence: the `nor`
  result is dead, and `srawi` reads `2 - tag`. That maps tag `2` to index `8`,
  tags `>= 3` to index `7`. The transform's `+0x0C` word is never written by the
  original body; Rust uses `0` for determinism.
- `0x807C2A00`/`0x807C2E50` test world flags `0x8` and `0x4`; the `0x8` arm
  stores `1`, the `0x4` arm stores `0`, and the free arm allocates `0x900`
  through the `0x8042A468` vtable and falls back to `0x800B53A0`.
- `0x807C25B0` loads its callback vtable from the embedded object at `+0x70`
  (the object's first word), so `vmethod1(this + 0x70, 0x18, descriptor)` is the
  correct shape.

## Verification

- `cargo check` succeeds.
- `node tools/check-lane-csv.mjs wave168` validates the seven-row lane CSV.
- All seven evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`.
