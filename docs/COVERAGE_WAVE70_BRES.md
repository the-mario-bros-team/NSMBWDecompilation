# Coverage - Wave 70 (resource::bres)

## How to read this file

This document covers the **wave-70 group** of BRRES work in
`src/resource/bres.rs`. Waves 4, 5, 7, 9 and 18 already translated the U8
archive path, the BRRES dictionaries, the typed section accessors, the MDL0
payload accessors, the deferred bind pass and the material/texture state
payload. Wave 70 fills the **cached-block publish helpers and the object-cache
accessors** that sit immediately around that payload, in the range
`0x8023C610..0x8023EAC0`:

1. the two **GX block-word emitters** that push a record's cached block size to
   the FIFO (`0x8023C610`, `0x8023C6F0`);
2. the **self-relative 0x200-byte block copy** (`0x8023C9B0`);
3. the eight **fixed-size flush/store publishers** (`0x8023CA20..0x8023CB30`);
4. the three **fixed-size cache-block copies** (`0x8023CBA0..0x8023CC20`);
5. the remaining **texture/palette slot accessors** (`0x8023CC90`,
   `0x8023CDC0`);
6. the **record header readers** and the block-size setter
   (`0x8023D0A0..0x8023D190`);
7. the **0x40 present-gated emitter** (`0x8023D5B0`);
8. the **texcoord variant readers** (`0x8023DC70`, `0x8023DCB0`);
9. the **count-selected publisher** (`0x8023DDD0`);
10. the six **per-entry cache thunks** (`0x8023EA20..0x8023EAC0`).

Each row has:

- **address** - the Ghidra entry address (virtual address of `sys/main.dol`);
- **function** - the Rust function in `src/resource/bres.rs`;
- **bytes** - the byte count from `work/ghidra/main-gb/export/functions.tsv`;
- **confidence** - `high` when offsets, guards and constants were read directly
  from the Ghidra C; `medium` when a block or record role had to be inferred
  from the surrounding family;
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 30 | 1,724 | 15 | 15 |

## What this pass covers

- **Block-word emitters** (`0x8023C610`, `0x8023C6F0`): two descriptors on the
  same record. The first lives at `+0x18`/`+0x1C`/`+0x20`, the second at
  `+0x24`/`+0x28`/`+0x2C`; both write the descriptor's word count through the
  checked FIFO writer when `sync != 0` and straight to `0xCC008000` otherwise.
  The first has a `skip_header` variant that moves the block on by 0x20 bytes
  and subtracts 0x20 from the count.
- **Self-relative block copy** (`0x8023C9B0`): copies 0x200 bytes, rebases the
  word at `+4` by the byte distance the block moved, then cache-stores `*(dst)`
  bytes.
- **Fixed-size publishers** (`0x8023CA20`, `0x8023CA60`, `0x8023CA80`,
  `0x8023CAA0`, `0x8023CAC0`, `0x8023CAE0`): the "sync path or plain store"
  pair over 0x20, 0x40, 0x80 and 0xA0 bytes and over a size read from the
  block's first word.
- **GX word emitters** (`0x8023CB00`, `0x8023CB30`): push the 0x20/0x80 block
  count to the FIFO, with the checked writer optional.
- **Cache-block copies** (`0x8023CBA0`, `0x8023CBE0`, `0x8023CC20`): 0x80, 0x40
  and 0xA0 bytes through `0x8024A530`.
- **Slot accessors** (`0x8023CC90`, `0x8023CDC0`): bounds-checked slot
  payload pointers with 0x20 (texture) and 0x0C (palette) strides.
- **Record header readers** (`0x8023D0A0`, `0x8023D0C0`, `0x8023D100`,
  `0x8023D190`): size-word store, 8-byte header copy, unsigned byte 0 and
  signed byte at `+2`.
- **Present-gated 0x40 emitter** (`0x8023D5B0`): a zero presence flag suppresses
  the command.
- **Texcoord variant readers** (`0x8023DC70`, `0x8023DCB0`): the four selector
  bytes of a `+0x14`-strided pair of variants and the packed decode of the word
  at `+0xC`/`+0x10` into six optional outputs.
- **Count-selected publisher** (`0x8023DDD0`): `<2`/`<4`/`<8`/else selects
  0x20/0x40/0x80/0xA0 before the same sync-or-store emission.
- **Per-entry thunks** (`0x8023EA20..0x8023EAC0`): six byte-identical
  sync-or-store wrappers; each is a separate symbol and keeps its own body.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8023C610` | `mdl_buffer_word_emit` | 216 | medium |
| `0x8023C6F0` | `mdl_aux_buffer_word_emit` | 100 | medium |
| `0x8023C9B0` | `res_block_copy_200` | 100 | medium |
| `0x8023CA20` | `res_cache_publish` | 24 | medium |
| `0x8023CA60` | `res_cache_publish_20` | 28 | high |
| `0x8023CA80` | `res_cache_publish_80` | 28 | high |
| `0x8023CAA0` | `res_cache_publish_40` | 28 | high |
| `0x8023CAC0` | `res_cache_publish_a0` | 28 | high |
| `0x8023CAE0` | `res_cache_publish_alt` | 24 | medium |
| `0x8023CB00` | `res_gx_word_emit_20` | 48 | high |
| `0x8023CB30` | `res_gx_word_emit_80` | 48 | high |
| `0x8023CBA0` | `res_block_copy_80` | 60 | medium |
| `0x8023CBE0` | `res_block_copy_40` | 60 | medium |
| `0x8023CC20` | `res_block_copy_a0` | 60 | medium |
| `0x8023CC90` | `res_texture_object_ptr_alt` | 44 | high |
| `0x8023CDC0` | `res_palette_object_ptr_alt` | 44 | high |
| `0x8023D0A0` | `res_cache_set_size` | 20 | high |
| `0x8023D0C0` | `res_record_copy_8` | 52 | high |
| `0x8023D100` | `res_record_first_byte` | 28 | medium |
| `0x8023D190` | `res_record_selector_byte` | 32 | high |
| `0x8023D5B0` | `res_cache_emit_40` | 52 | high |
| `0x8023DC70` | `res_texcoord_variant_copy` | 56 | high |
| `0x8023DCB0` | `res_texcoord_variant_fields` | 192 | high |
| `0x8023DDD0` | `res_cache_publish_counted` | 208 | high |
| `0x8023EA20` | `res_entry_cache_sync_0` | 24 | medium |
| `0x8023EA40` | `res_entry_cache_sync_1` | 24 | medium |
| `0x8023EA60` | `res_entry_cache_sync_2` | 24 | medium |
| `0x8023EA80` | `res_entry_cache_sync_3` | 24 | medium |
| `0x8023EAA0` | `res_entry_cache_sync_4` | 24 | medium |
| `0x8023EAC0` | `res_entry_cache_sync_5` | 24 | medium |

## Opaque callees reused by this pass

| Callee | Role used here | Why opaque |
|---|---|---|
| `0x801AC4A0` | data-cache block store plus `syscall` | `// observed:` loops `dataCacheBlockStore` over 0x20-byte blocks; Ghidra hides its arguments at the `mode != 0` call sites, so those branches call it with none, exactly as the export shows |
| `0x801AC500` | data-cache block store | `// observed:` same loop without the trailing `syscall` |
| `0x801C95E0` | write one GX FIFO word with stream-state checks | body tests the stream object at `r2 - 0x4EF8` and stores at `0xCC008000` |
| `0x8024A530` | copy whole 32-byte units | block-copy primitive documented in wave 9 |
| `0xCC008000` | GX write-gather FIFO | direct store is a GX command word |

## Unknowns and deliberate skips

- `// unknown:` the concrete record types behind the `+0x18`/`+0x24` block
  descriptors and the six `0x8023EA20..0x8023EAC0` thunks. Their bodies are
  literal; only their owners are inferred from the surrounding material bind
  family.
- `// inferred:` the 0x20/0x40/0x80/0xA0 sizes are the one-, two-, four- and
  five-plus-slot texture cache block sizes; the counts come from the thresholds
  in `0x8023DDD0`, not from a symbol table.
- **Skipped because another lane already owns them:** `0x8023CC60`,
  `0x8023CCC0` and `0x8023CD90` were merged into `ledger/functions.csv` by
  `runtime::bt` (`src/runtime/bt.rs`) while this lane was running. Only the
  still-free twins `0x8023CC90` and `0x8023CDC0` are claimed here; the same
  bodies in `bres.rs` were removed so no address is translated twice.
- **Skipped as out of scope for this pass:** the large material binder at
  `0x8023C280` (908 bytes) and the polygon/draw-state mother functions at
  `0x8023BB60`, `0x8023D8B0`, `0x8023DEA0` and `0x8023E390` remain
  untranslated; they need the full draw-state record, which is not reconstructed
  yet. The key-sampling trio `0x8023F000`, `0x8023F4E0`, `0x8023F5A0` stays
  untranslated for the reason already recorded in
  `docs/COVERAGE_WAVE18_BRRES.md` (caller-set GQR quantization). `0x8023FA30`
  and `0x8023FE50` remain for a later pass.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (re-read from the TSV, not copied from prose).
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn` body in `src/resource/bres.rs`
  (151 bodies in the file after this pass).
- No address in `ledger/agents/wave70-bres.csv` was already in
  `ledger/functions.csv`; the three that were, were dropped and replaced.
- `cargo check --message-format=short` exits 0 with no diagnostics for
  `src/resource/bres.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 0 violations.
- `node tools/check-lane-csv.mjs wave70` reports `OK` for
  `ledger/agents/wave70-bres.csv` (30 rows, 7 fields each).

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic is modelled on the Wii's 32-bit
big-endian PowerPC and must only execute on a target-compatible runtime with the
original memory image mapped.
