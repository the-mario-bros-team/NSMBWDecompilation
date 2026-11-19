# Coverage - Wave 97 (`game::enemies`, the `0x80B00000` bucket)

## How to read this file

This page records the Wave 97 `game::enemies` lane: 30 functions of the
`0x80B00000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 78 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave97-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u8`, never transcribed as invented values. The integer and
float payloads quoted below (`16.0`, `-16.0`, `0.0`, `24.0`, the `-8.0`..`8.0`
rectangle) were dumped from `work/rels/elf/d_enemiesNP.elf` at the cited
synthetic addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 3,220 | 19 | 7 |

Four rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80B09550`, `0x80B09580`, `0x80B0A3B0` and `0x80B0A3E0`, whose hidden-`r12`
this-adjust pointer is not modelled (the same limitation recorded for the
`0x802DCBFC` copies in waves 56, 75, 80, 81 and 82).

The bucket holds 207 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 114 were already in
`ledger/functions.csv` before this pass, 30 are claimed here and 63 remain.

## What this pass covers

Three families of the same actor shell, in address order.

**Family 1 - the `0x6A8` actor, `0x80B091F0..0x80B099C0` plus `0x80B09DD0`.**

- The five-entry descriptor skeleton: two freeing callbacks
  (`0x80B091F0`/`0x80B09250`), the constant producers `0x80B092B0`/`0x80B092C0`
  (`0x80353C64` / `0x80353BE4`), the five-link `+0x30` chain walker
  `0x80B093B0`, the `:`-suffix name predicate `0x80B09490` and the two 48-byte
  `0x802DCBFC` forwarders `0x80B09550`/`0x80B09580`.
- `0x80B095B0` allocates `0x6A8` bytes and installs class vtable `0x80B671AC`.
  Its store list is identical to [`fn_80b08730`] (the wave-82 star-coin
  sibling, vtable `0x80B66DB4`): the `+0x524` model block, the `+0x544`
  holder, the `+0x594`/`+0x5C4` sub-objects and the `+0x5F0` controller.
- `0x80B09690` is the entry step: heading `0x4000`, the `+0x148`/`+0x5F0`
  installer pair seeded from `0x80B31C70` and `0x80B31C94`, the four floats
  `16.0`, `-16.0`, `1.0`, `-1.0` at `+0x630..+0x63C`, `+0x68E = 2` and the
  `+0x588` animation phase restarted at `1.0`.
- `0x80B097D0`, `0x80B09870` and `0x80B099C0` are the three sound wrappers
  (`0x2E9`, `0x2EA`, `0x2EC`) built on the random image-plane position pair
  `0x800B30C0`/`0x800B30E0` and the actor's `+0x2B0` virtual, exactly the shape
  wave 82 documented for `0x80B08950`/`0x80B089F0`/`0x80B08B40`.
- `0x80B09DD0` releases the five embedded holders with `-1`, resets the base
  actor and, for a positive mode, releases the heap block.

**Family 2 - the `0x5E0` actor, `0x80B0A050..0x80B0A4B0`.**

- The same skeleton: free pair `0x80B0A050`/`0x80B0A0B0`, producers
  `0x80B0A110`/`0x80B0A120`, walker `0x80B0A210`, predicate `0x80B0A2F0`,
  forwarders `0x80B0A3B0`/`0x80B0A3E0`.
- `0x80B0A410` allocates `0x5E0` bytes. It installs `0x80303658` first, then
  the `+0x584` sub-object word `0x80B330E0`, and finishes with class vtable
  `0x80B675AC`.
- `0x80B0A4B0` is the frame helper. When the `0x80429D19` byte is set and the
  word at `0x80429D18` does not intersect the high half of the `0x8042A078`
  mask, `+0x5DC` is cleared and the `+0x148` transform is left alone; otherwise
  `+0x5DC` becomes `1` and `0x8008C330` commits the transform. Both paths then
  write `0.0` and `24.0` at `+0x320`/`+0x324`.

**Family 3 - the `0x528` shell, `0x80B0B450..0x80B0B6F0`.**

- The free pair `0x80B0B450`/`0x80B0B4B0`, the two BSS descriptor producers
  `0x80B0B510` (`0x80B6E994`) and `0x80B0B520` (`0x80B6E954`), the chain walker
  `0x80B0B610` and the name predicate `0x80B0B6F0`.
- The producers return the same two BSS addresses as
  [`fn_80ab7630`]/[`fn_80ab7640`] in the `0x80AB0000` bucket; those addresses
  live above the `0x80B6E500` BSS start, so their contents are filled at run
  time and are entered by address only.

### Identity evidence

- **The class names do not survive.** Each constructor is referenced only from
  its class vtable (`0x80B671A0`, `0x80B675A0`, `0x80B67CA0` point at
  `0x80B095B0`, `0x80B0A410`, `0x80B0B810`), and the descriptor records at
  `0x80B67140`/`0x80B67540` hold engine callbacks (`0x8015F840`, `0x8015F850`,
  `0x8015F8C0`, `0x8015F8D0`, `0x8015F920`, `0x8015F930`) rather than
  printable names. No `daEn*_c` string can be tied to them, so the three
  families are described by size, vtable and field offsets only.
- **The `0x6A8` constructor is a proven sibling.** `0x80B08730` (wave 82) and
  `0x80B095B0` have the same allocation size, the same eleven stores and the
  same `0x80305F10` / `0x803295D8` / `0x80329578` / `0x80329548` words; only
  the final class vtable differs (`0x80B66DB4` vs `0x80B671AC`).
- **Every constant was dumped, not assumed.** `0x80B31C70` is four zero words
  (transform seed); `0x80B31C94` is `-8.0, 0.0, 8.0, 8.0`; `0x80B31CBC..CC8`
  is `16.0, -16.0, 1.0, -1.0`; `0x80B31D00`/`0x80B31D04` are `0.0` and `24.0`.
  `0x80B31C70`/`0x80B31C94` are the same record pair that `0x80B31BE0`/
  `0x80B31C04` provide for the wave-82 sibling `0x80B08810`.
- **The two BSS producers are already documented.** `0x80AB7630` and
  `0x80AB7640` return `0x80B6E994` and `0x80B6E954`; wave 56 recorded that the
  export does not name their contents.

### Deliberate skips

- **The rest of the `0x528` family.** `0x80B0B7B0` and `0x80B0B7E0` (48 bytes
  each, the two `0x802DCBFC` forwarders) and `0x80B0B810` (76 bytes, the
  constructor that allocates `0x528` bytes and installs `0x80B67CB0`) close
  the run but fall outside this lane's 30-body budget. They are the natural
  next entries; because the constructor is not translated, the `0x528` family
  here is explicitly a partial family.
- **The larger actor above the run.** `0x80B0A8D0` (56 bytes, a
  `0x802DCBFC`-style call with the record at `0x80B31D50`), `0x80B0ABF0`
  (88 bytes, the `+0x99C`/`+0x9A0`/`+0x9A4` fraction helper),
  `0x80B0AC50` (104 bytes, the kind-`0x99` search), `0x80B0AF60` (76 bytes,
  its caller) and `0x80B0B020` (372 bytes, the destructor of a `0xAE0`-field
  actor) belong to a different, much larger class. Its constructor is not in
  this run, so the field roles would be inferred only from the destructor; they
  were left for a pass that can read the constructor first.
- **Two wave-82 skips are still open.** `0x80B07C90` (36 bytes) is a
  `halt_baddata()` body and `0x80B07D70` (252 bytes) decodes with removed
  unreachable blocks and `GQR` paired-single artefacts; neither can be
  reconstructed honestly, exactly as wave 82 recorded.
- **The rest of the bucket: 63 functions.** The untouched runs are
  `0x80B04A90..0x80B05B80` (6), the two undecodable bodies above, and
  `0x80B0A8D0..0x80B0FF50`, including `0x80B0C540` (848 bytes) and
  `0x80B0C900` (1,140 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B091F0` | `fn_80b091f0` | 88 | high |
| `0x80B09250` | `fn_80b09250` | 92 | high |
| `0x80B092B0` | `fn_80b092b0` | 12 | high |
| `0x80B092C0` | `fn_80b092c0` | 12 | high |
| `0x80B093B0` | `fn_80b093b0` | 224 | high |
| `0x80B09490` | `fn_80b09490` | 136 | medium |
| `0x80B09550` | `fn_80b09550` | 48 | low |
| `0x80B09580` | `fn_80b09580` | 48 | low |
| `0x80B095B0` | `fn_80b095b0` | 224 | high |
| `0x80B09690` | `fn_80b09690` | 168 | high |
| `0x80B097D0` | `fn_80b097d0` | 148 | medium |
| `0x80B09870` | `fn_80b09870` | 164 | medium |
| `0x80B099C0` | `fn_80b099c0` | 164 | medium |
| `0x80B09DD0` | `fn_80b09dd0` | 156 | high |
| `0x80B0A050` | `fn_80b0a050` | 88 | high |
| `0x80B0A0B0` | `fn_80b0a0b0` | 92 | high |
| `0x80B0A110` | `fn_80b0a110` | 12 | high |
| `0x80B0A120` | `fn_80b0a120` | 12 | high |
| `0x80B0A210` | `fn_80b0a210` | 224 | high |
| `0x80B0A2F0` | `fn_80b0a2f0` | 136 | medium |
| `0x80B0A3B0` | `fn_80b0a3b0` | 48 | low |
| `0x80B0A3E0` | `fn_80b0a3e0` | 48 | low |
| `0x80B0A410` | `fn_80b0a410` | 160 | high |
| `0x80B0A4B0` | `fn_80b0a4b0` | 152 | medium |
| `0x80B0B450` | `fn_80b0b450` | 88 | high |
| `0x80B0B4B0` | `fn_80b0b4b0` | 92 | high |
| `0x80B0B510` | `fn_80b0b510` | 12 | high |
| `0x80B0B520` | `fn_80b0b520` | 12 | high |
| `0x80B0B610` | `fn_80b0b610` | 224 | high |
| `0x80B0B6F0` | `fn_80b0b6f0` | 136 | medium |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  3,220 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv` (the bucket's 114 existing rows are
  untouched).
- `node tools/check-lane-csv.mjs wave97` reports `OK` for this lane.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace build itself is currently stopped by
  parse/type errors in `src/engine/dsys.rs` and `src/resource/archive.rs`
  (other Wave 97 lanes), which do not name this file.
