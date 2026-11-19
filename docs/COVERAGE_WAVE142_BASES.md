# Coverage - Wave 142 (`game::bases`)

## How to read this file

This page documents the Wave 142 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a virtual dispatch
had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 17 | 9,160 | 7 | 10 |

All 17 rows are `translated`; none is `partial`. The module header now reports
3603 bodies (3586 + 17).

## What this pass covers

- **The four "ready" bodies of `docs/SPEC_WAVE141_BASES.md`** (8,136 bytes).
  Each was re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and cross-checked against the export:

  | cluster | body |
  |---|---|
  | `dWorldSelect_c` constructor | `0x807BD110`: 13 `StateID` objects, one-time 17-`f32` / 9-`u16` layout block, 13 x 9-word payload rotation, `0x8015F7C0` + `0x80600160` per state |
  | world-select cell initialiser | `0x8062C8E0`: `+0x70` event sub-object, 9 x 38 grid scan, ten slot records, bank snapshot and write-back, packer `0x8062C800` |
  | world-9 water level | `0x8071D500`: six states, `Wm_we_water`/`...sign`/`...upsign`/`...up` creation, `0x550`/`0x551`/`0x552` sound ids |
  | water-surface actor | `0x8067FC80`: packed parameter word, eight 0x74-byte segments, eight 0x88-byte node records, 24 render-node pairs from the 24 x 3 table at `0x807ECB48` |

- **Thirteen `0x8077` planner-bucket bodies** (1,024 bytes), taking that bucket
  from 82 to 69 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | course-object spawner family | `0x80774750` (`"W<S><n>S"` name, profile `0x279`), `0x80773A40` (zone-record tail call), `0x80773A60` (zone step result) |
  | zone-table getters | `0x80775030` (`+0x30 + index*4`), `0x80775100` (`+0x48` or `-1`), `0x80775150` (`+0x50`) |
  | course-side construction | `0x807760B0` (`0x808173C8` descriptor, `+0x184`/`+0x1A0` sub-objects) |
  | small actor helpers | `0x80776360` (pose reset), `0x80777F60` (clamped stepper), `0x80779F90` (`+0x1F4` clear), `0x80774090` (kind `== 2` test), `0x80777910`/`0x80777950` (twin deleting destructors) |

## Where the binary differed from the wave-141 prose

1. **The cell banks of `0x8062C8E0` live in the course object, not in `this`.**
   `0x8062CAFC` is `add r3,r30,r0` with `r30 = *(void**)0x80429F7C`, so the
   snapshot/clear pass touches `course + 0x400 + idx*8` or
   `course + 0x770 + idx*8`, and the write-back goes to `course + 0x720`
   (bank A) or `course + 0xA90` (bank B). The prose had `this + ...` for all
   three. The snapshot source is the fixed range `+100..+109`; the selected
   cell index is only used to re-resolve `this[0x4F4]`.
2. **`+0x3724` in `0x8067FC80` is `400.0 - 48.0`.** The load at `0x8067FE5C` is
   `fsubs f0,f31,f0` with `f31 = 400.0` and `f0 = 48.0`, not the vertical
   offset `d - 48.0` the prose specified. The altitude ring is
   `-(((word >> 24) & 0xF) << 12)`; `rlwinm r6,r9,20,16,19` extracts bits
   24-27, not bits 12-15.
3. **`0x8071D500`'s slope is `-1.0 / 120.0`.** The magic pair at `0x807CD028`
   is the **double** `0x4330000080000000` (`2^52 + 2^31`), so the `fsubs`
   against the `0x43300000 | (120 ^ 0x80000000)` word yields `120.0`, not
   `120.0 - 176.0`. The `+0xB0` timer is `f32@0x8082FFE4` (BSS) plus the zero
   word at `0x80809628`, not `+ 15`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8062C8E0` | `fn_8062c8e0` | 1,804 | medium |
| `0x8067FC80` | `fn_8067fc80` | 1,676 | medium |
| `0x8071D500` | `fn_8071d500` | 1,748 | medium |
| `0x80773A40` | `fn_80773a40` | 20 | medium |
| `0x80773A60` | `fn_80773a60` | 108 | medium |
| `0x80774090` | `fn_80774090` | 60 | high |
| `0x80774750` | `fn_80774750` | 292 | medium |
| `0x80775030` | `fn_80775030` | 76 | medium |
| `0x80775100` | `fn_80775100` | 72 | medium |
| `0x80775150` | `fn_80775150` | 56 | medium |
| `0x807760B0` | `fn_807760b0` | 76 | medium |
| `0x80776360` | `fn_80776360` | 48 | high |
| `0x80777910` | `fn_80777910` | 64 | high |
| `0x80777950` | `fn_80777950` | 64 | high |
| `0x80777F60` | `fn_80777f60` | 76 | high |
| `0x80779F90` | `fn_80779f90` | 12 | high |
| `0x807BD110` | `fn_807bd110` | 2,908 | high |

**Totals:** 17 bodies, 9,160 bytes. The address window runs from
`0x8062C8E0` to `0x807BD110`.

## Notes on inference

- The ten `medium` rows each call at least one unclaimed sibling or main.dol
  address (`0x8015F7C0`, `0x80600160`, `0x800C9500`, `0x800C93D0`, `0x800CE600`,
  `0x8062C770`, `0x8062C800`, `0x806601B0`, `0x8007FAD0`, `0x8007FD80`,
  `0x8007F900`, `0x800F32F0`, `0x800F5FD0`, `0x80069020`, `0x8016A2F0`,
  `0x80100530`, `0x801005C0`, `0x801006C0`, `0x800F29F0`, `0x800F9270`,
  `0x800F9970`, `0x801002C0`, `0x800F92C0`, `0x80773AD0`). Their argument
  shapes are fixed by the instruction stream; their bodies are not claimed.
- Three helper wrappers were added with the bodies because the shapes did not
  exist yet: `worldmap_create` (`+0xA0` create slot), `play_worldmap_sound`
  (`0x8006A3F0` + `0x80199390`), `call_f32x5_u32x7` (`0x800DB740`). Each carries
  its own `# Original` / `# Confidence` block.
- `0x80774750` reads `record + 8` **without** a null guard at `0x807747E4`;
  the body reproduces that and the doc block says so.
- `0x80775030`/`0x80775100`/`0x80775150` take the zone index from `+0x338C` and
  index the `0x5C7F4` table with stride `0x3F08`, matching the arithmetic in
  `0x807740D0`'s specification.

## Deliberate skips

Read and rejected, or deferred with the reason:

- **`0x8065DD50` (1,772)**, **`0x8065F090` (1,772)**, **`0x807740D0` (1,656)**
  and **`0x806A24F0` (1,624)** are the four "conditional" rows of
  `docs/SPEC_WAVE141_BASES.md`. Each still needs the external fact the spec
  names (the `0x80828D5C` `StateID`, the four `0x80C6Exxx` name strings, the
  `+0x3388` area-mode meaning plus `0x800F53A0`'s id space, and the actor
  vtable slots `+0x2D4/+0x2F0/+0x314/+0x31C/+0x320/+0x3B4`). They remain open.
- **`0x8077A000` (40)** is the only bucket body rejected on evidence grounds:
  its export is `halt_baddata()` ("bad instruction data"), so no honest body
  can be written. `0x80773200` (4) and `0x80776C00` (4) are single-instruction
  `thunk_` aliases; `0x8077D660` (4) and `0x8077D670` (8) are too small to be
  interesting and were left for a later sweep.
- The remaining `0x8077` bucket rows above 400 bytes (`0x80772370`,
  `0x807727D0`, `0x80775030`'s neighbours `0x80775190`/`0x80775D00`,
  `0x8077A4B0`, `0x8077AC30`, `0x8077C670`, `0x8077DC80` and the 4,076-byte
  `0x8077B450`) were not started: each needs its own multi-phase call graph and
  the lane stopped at the verified small-body cluster instead.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (17 rows, 9,160 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 17 were checked against
  the ledger's address column before being written, and none is duplicated.
- Each body's last word was re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000`; 16 of the 17 end in `0x4E800020` (`blr`)
  at `entry + size - 4`, and `0x80773A40` ends in the documented tail branch
  `b 0x80773AD0` (`0x48000080`). No `functions.tsv` row starts inside any body.
- `node tools/check-lane-csv.mjs wave142` reports `OK`: 17 rows, 17 unique
  addresses, all `translated`.
- `cargo check --message-format=short` finishes with no error naming
  `src/game/bases.rs`.
