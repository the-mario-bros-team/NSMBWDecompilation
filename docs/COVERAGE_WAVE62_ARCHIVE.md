# Coverage - Wave 62 (resource::archive, 0x80260000 bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 28 bodies are appended after the wave-54
section at the end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 28 | 1452 | 24 | 4 |

The file now holds 264 translated bodies: 236 before this pass plus the 28
below. The assigned `0x80260000` bucket still has 134 untranslated functions of
260; 98 were already translated before this pass and 28 are added here.

## What this pass covers

The `0x80260000` bucket is NW4R sound/stream infrastructure. Waves 39 and 41
took the archive-facing reader and the member-object classes; this pass takes
four remaining groups that read or write the same handle record and its lists:

1. **Float and byte accessors** - `0x80265F40..0x80265FB0` (8 bodies). A
   forwarder into the `this + 4` block, stores and loads over the four-word
   arrays at `+0x34` and `+0x40`, a paired float/byte store at `+0x18`/`+0x1C`,
   the byte setter at `+0x1D`, and a guarded free that returns `this`.
2. **Wave-49 handle continuation** - `0x80267390..0x802675F0` (9 bodies). The
   mode byte at `+0x98` plus the child notify and `+0x2C` slot, three clamped
   float setters (`+0xB0`, `+0xCC`, `+0xE0[]`), three virtual property setters
   through the `+0x24` slot (`+0x2C`, `+0x30`, `+0x1D`), and one record binder
   that fills `+0x1C..+0x2C` and caps the channel count at 4.
3. **Guarded frees and list helpers** - `0x80267B40..0x80267D30` (5 bodies).
   The guarded-free instantiation at `0x80267B40`, the lazy 0x18-byte list
   header at `0x803FC958`, its element deleter, the `(count + 1) * 0xE0` row
   size and the guarded `+0x10` record-list filler under the interrupt mask.
4. **Small field and size helpers** - `0x802627E0`/`0x80262840`,
   `0x802643D0`, `0x80264C00`, `0x80266230`/`0x802666A0` (6 bodies). Two
   interrupt-masked child halfword stores with owner flag bits, a five-word
   descriptor initialiser, one `(count + 0x10) * 0x48` size query, the `+0x84`
   ready setter and the `+0x90` word store with its `+0x86` sign flag.

### Shared layout of the handle cluster

`// inferred:` the object is the sound-handle/player record that the wave-49
accessors already write. The exports prove only offsets and stores:

- `+0x00` is the vtable word; slots `+0x24` and `+0x2C` callers appear
  in clusters 1 and 2, and the binder also calls `+0x0C` and `+0x14`.
- `+0x10` is a child pointer, `+0x14`/`+0x18`/`+0x1C` are the record-list
  head and its two range words, `+0x1C..+0x2C` are five bound words.
- `+0x34` and `+0x40` are four-word float arrays; `+0xB0`, `+0xCC` and
  `+0xE0[]` are single floats clamped against `r2 - 0x4834`.
- `+0x84`/`+0x85`/`+0x86` are state bytes, `+0x90` a word, `+0x98`/`+0x99`
  the mode and enable bytes, `+0xC8` a copied child word.

The clamp value is read from `0x8042E84C` (`r2 - 0x4834`). The list header is
`0x803FC958`, its registration link is `0x803FC948` and its ready byte is
`r13 - 0x4978` (`0x8042A208`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802627E0` | `fn_802627e0` | 96 | high |
| `0x80262840` | `fn_80262840` | 96 | high |
| `0x802643D0` | `fn_802643d0` | 44 | high |
| `0x80264C00` | `fn_80264c00` | 12 | high |
| `0x80265F40` | `fn_80265f40` | 8 | high |
| `0x80265F50` | `fn_80265f50` | 16 | high |
| `0x80265F60` | `fn_80265f60` | 16 | high |
| `0x80265F70` | `fn_80265f70` | 12 | high |
| `0x80265F80` | `fn_80265f80` | 8 | high |
| `0x80265F90` | `fn_80265f90` | 16 | high |
| `0x80265FA0` | `fn_80265fa0` | 16 | high |
| `0x80265FB0` | `fn_80265fb0` | 64 | high |
| `0x80266230` | `fn_80266230` | 24 | high |
| `0x802666A0` | `fn_802666a0` | 24 | high |
| `0x80267390` | `fn_80267390` | 88 | high |
| `0x80267400` | `fn_80267400` | 24 | high |
| `0x802674E0` | `fn_802674e0` | 16 | high |
| `0x802674F0` | `fn_802674f0` | 24 | high |
| `0x80267510` | `fn_80267510` | 32 | high |
| `0x80267530` | `fn_80267530` | 64 | medium |
| `0x80267570` | `fn_80267570` | 60 | medium |
| `0x802675B0` | `fn_802675b0` | 60 | medium |
| `0x802675F0` | `fn_802675f0` | 196 | high |
| `0x80267B40` | `fn_80267b40` | 64 | high |
| `0x80267C40` | `fn_80267c40` | 116 | high |
| `0x80267CC0` | `fn_80267cc0` | 96 | high |
| `0x80267D20` | `fn_80267d20` | 12 | high |
| `0x80267D30` | `fn_80267d30` | 148 | medium |

## Unknowns

1. **The handle class name.** The exports never name the owner of the fields
   between `+0x00` and `+0xE0`; the doc blocks describe literal offsets and mark
   the sound-handle reading `// inferred:`.
2. **The dropped virtual arguments.** `0x80267530`, `0x80267570` and
   `0x802675b0` print their `+0x24` slot call with no arguments, while the slot
   is an instance method. `this` is forwarded by inference, so those three rows
   are `medium`.
3. **The restore argument.** `0x80267D30` prints the else-arm restore as
   `FUN_801b1180()`; the saved mask from `FUN_801b1140` is forwarded by
   inference, hence `medium`.
4. **The `0x80265DB0` and `0x80004364` callees.** Both are called by address
   only; their bodies are outside this pass and their roles stay
   `// unknown:`.

## Deliberate skips

- **`0x80267B80`, `0x80267BC0`, `0x80267C00`** - byte-identical copies of
  `fn_80267b40`'s guarded free. They are real functions but would only repeat
  the same four instructions; `fn_80267b40` is bound and the three siblings are
  left for a batch pass.
- **`0x80265DB0`/`0x80265E80`** - the 196-byte block initialiser and its
  184-byte constructor call it. `fn_80265f40` forwards into `0x80265DB0`, but
  the initialiser's 40-word default block was not translated here.
- **`0x80265FF0`, `0x802660D0`** - 224- and 340-byte constructor blobs that
  write most of the same record. They need a full field-by-field pass.
- **`0x802667F0`, `0x80267F40`** - 1412- and 1756-byte parsers; too large for a
  small-helper lane.
- **`0x80268D80`** - a four-byte thunk to `0x80229190`, not translated.
- **`0x8026A880` and the `0x8026C5F0..0x8026D3B0` managers** - long voice-set
  dispatchers that already have address-only constants in this file.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/802xxxxx_FUN_802xxxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 28 addresses came from
  the untranslated part of the `0x80260000` bucket.
- `node tools/check-lane-csv.mjs wave62` reports
  `wave62-archive.csv: 28 rows`, 28 unique addresses and `OK`.
- `cargo check --message-format=short` reports no error and no diagnostic for
  `src/resource/archive.rs`; the remaining crate errors are in another lane's
  `src/game/stage.rs`.
