# Coverage - Wave 49 (resource::archive, 0x80260000 bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 30 bodies are appended after the wave-46
section at the end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 30 | 1972 | 21 | 9 |

The file now holds 208 translated bodies: 178 before this pass plus the 30
below. The assigned `0x80260000` bucket still has 162 untranslated functions of
260; 68 were already translated before this pass and 30 are added here.

## What this pass covers

The `0x80260000` bucket is NW4R sound-library infrastructure. Wave 39 took the
DVD reader, buffer cache and list primitives; wave 41 took the first four
member-object classes (`0x8026A670..0x8026BED0`). This pass closes the rest of
that same class family plus the field accessors on the sound-handle object:

1. **Class D** - `0x8026B640..0x8026B740` (3 bodies). Constructor, two-set size
   query and guard-mutex initialiser for vtable `0x8034EB68`.
2. **Class E constructor** - `0x8026BCF0` (1 body). Vtable `0x8034EB90`; its
   size query, mutex and release helpers were already translated in wave 41 as
   `fn_8026bdc0`/`fn_8026be00`/`fn_8026bed0`.
3. **Class F** - `0x8026C2F0..0x8026CA60` (7 bodies). Constructor, size query,
   guard mutex, release, manager-count dispatch and deleting destructor for
   vtable `0x8034EBB8`.
4. **Class G** - `0x8026CAE0..0x8026CCC0` (4 bodies). Constructor, size query,
   guard mutex and release for vtable `0x8034EBE0`.
5. **Class H** - `0x8026D0B0..0x8026D330` (4 bodies). Constructor, size query,
   guard mutex and release for vtable `0x8034EC08`; its dispatch and deleting
   destructor are the already-translated `fn_8026d770`/`fn_8026d7e0`.
6. **Sound-handle field accessors** - `0x80267300..0x802674D0` (12 bodies).
   Four paired word setter/clearer methods, a byte getter, two float setters
   and one word setter over the same handle object.

### Shared layout of classes D..H

All five constructors have the same shape:

- install the class vtable at `+0x00`;
- clear `+0x04`, `+0x08` and the active byte at `+0x0C`;
- copy an opaque default block from `r2` small data
  (`SDA2_BASE = 0x80433080`) into the parameter fields at `+0x18..+0x44`;
- call the class's separate float-parameter store, which is kept as an address
  in this pass.

`// unknown:` the defaults are copied bit-for-bit; the export never prints their
numeric meaning, so no float interpretation is claimed. The constructors'
confidence is `medium` for that reason; the stores themselves are literal.

The size queries all return the larger of two AX voice-set sizes, each rounded
up with `(size + 0x87) & 0xFFFF_FFE0`, and the release helpers all take the
`+0x0C` flag under the guard mutex established by `fn_8026a810`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8026B640` | `fn_8026b640` | 140 | medium |
| `0x8026B6D0` | `fn_8026b6d0` | 84 | high |
| `0x8026B740` | `fn_8026b740` | 8 | high |
| `0x8026BCF0` | `fn_8026bcf0` | 208 | medium |
| `0x8026C2F0` | `fn_8026c2f0` | 208 | medium |
| `0x8026C3C0` | `fn_8026c3c0` | 84 | high |
| `0x8026C430` | `fn_8026c430` | 8 | high |
| `0x8026C570` | `fn_8026c570` | 124 | medium |
| `0x8026C9F0` | `fn_8026c9f0` | 112 | high |
| `0x8026CA60` | `fn_8026ca60` | 124 | medium |
| `0x8026CAE0` | `fn_8026cae0` | 196 | medium |
| `0x8026CBB0` | `fn_8026cbb0` | 44 | high |
| `0x8026CBF0` | `fn_8026cbf0` | 8 | high |
| `0x8026CCC0` | `fn_8026ccc0` | 100 | medium |
| `0x8026D0B0` | `fn_8026d0b0` | 196 | medium |
| `0x8026D180` | `fn_8026d180` | 84 | high |
| `0x8026D1F0` | `fn_8026d1f0` | 8 | high |
| `0x8026D330` | `fn_8026d330` | 124 | medium |
| `0x80267300` | `fn_80267300` | 8 | high |
| `0x80267310` | `fn_80267310` | 12 | high |
| `0x80267320` | `fn_80267320` | 8 | high |
| `0x80267330` | `fn_80267330` | 12 | high |
| `0x80267340` | `fn_80267340` | 8 | high |
| `0x80267350` | `fn_80267350` | 12 | high |
| `0x80267360` | `fn_80267360` | 8 | high |
| `0x80267370` | `fn_80267370` | 12 | high |
| `0x80267380` | `fn_80267380` | 8 | high |
| `0x802674B0` | `fn_802674b0` | 8 | high |
| `0x802674C0` | `fn_802674c0` | 8 | high |
| `0x802674D0` | `fn_802674d0` | 8 | high |

## Unknowns

1. **Class roles.** The exports install five distinct vtables but never name the
   class. The pass therefore calls them D..H, following wave 41's A..C naming,
   and does not invent a NintendoWare class name.
2. **The default blocks.** `r2 - 0x4768..-0x46E4` are copied as opaque words.
   Whether they are volume, pitch, pan or distance constants is not shown.
3. **The `0x8034EBC8` slot argument.** `0x8026CA60` calls the slot with no
   arguments in C; the body forwards `this`, matching the idiom used by
   `fn_8026d7e0` and marked `// inferred:`.
4. **Sound-handle field roles.** `+0x04`, `+0x10`, `+0x14`, `+0x18`, `+0xB4`,
   `+0xBC`, `+0xC8` and `+0x99` are described only by offset; the sibling
   readers (`0x802666c0`, `0x802667d0`) do not name them either.

## Deliberate skips

- **The five class parameter stores** - `0x8026B900` (604 B), `0x8026BF40`
  (716 B), `0x8026C5F0` (852 B), `0x8026CD30` (664 B), `0x8026D3B0` (796 B) -
  are called by address from the constructors but not translated. Each is a
  long float clamp chain over `r2` constants; folding them into this pass would
  risk a silent transcription error, so they are left to a dedicated pass.
  The same applies to the three wave-41 stores `0x8026A880`, `0x8026ADD0` and
  `0x8026B370`, which remain address-only constants.
- **`0x8026A2A0`, `0x8026A350`, `0x8026A420`** - sequence-list allocation and
  resize helpers. They allocate through the `0x801D4C40` family, not the archive
  heap, and belong to the sequence player.
- **`0x80269B50..0x80269F00`** - sound priority selection and list ordering;
  the `0x8027B5F0` subsystem owns the state they drive.
- **`0x80269660..0x80269990`** - volume/velocity conversion tables; float
  semantics that the audio mixer owns.
- **`0x8026DBA0..0x8026DF50`** - stream time and `RSEQ` offset helpers already
  reached through `audio::seq`.
- **`0x8026A650`** and the remaining one-field accessors outside
  `0x80267300..0x802674D0` - 8-12 byte helpers whose owners are not resolved;
  adding more offsets without a class would be padding.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/8026xxxx_FUN_8026xxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; the 30 addresses are all
  from the untranslated part of the `0x80260000` bucket.
- `node tools/check-lane-csv.mjs wave49` reports `wave49-archive.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/resource/archive.rs`. The 11 errors emitted in the shared run are all in
  `src/game/bases.rs` (another wave-49 lane); no archive line appears.
