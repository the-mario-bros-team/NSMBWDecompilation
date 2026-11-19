# Coverage — wave 24 resource registry and archive additions

## How to read this file

This pass continues the archive-resource work from
`docs/COVERAGE_WAVE4_REGISTRY.md` and `docs/COVERAGE_WAVE15_RESOURCES.md`. It is
organised by the flow, not by address:

1. **Scope** — what was added, where it lives, and how big it is.
2. **Function table** — one row per new body: address, Rust name,
   plain-English role, status, confidence and Ghidra evidence file.
3. **Cluster notes** — the four flows the rows belong to:
   - the mount-archive table and lookup (`0x8016B090..0x8016B180`),
   - the threaded stream manager and mount executor
     (`0x8016B1C0..0x8016B2A0`),
   - the path probe and extension fallback
     (`0x8016B400`, `0x8016BAA0`, `0x8016BC10..0x8016BC30`),
   - the stateful Yaz0 decoder (`0x802B85D0..0x802B89A0`).
4. **Unknowns** — object types, virtual slots and helpers called by address.
5. **Verification** — compiler, doc linter and ledger.

Addresses are **virtual addresses of the unrelocated `main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. Status and confidence follow
`docs/READING_GUIDE.md`: `translated` means the complete recorded control flow
is present; `high` confidence means the bytes and call order are explicit, while
`medium` means one callee identity, virtual `this` or hidden argument is
inferred.

## Scope

| metric | value |
|---|---:|
| functions added | 23 |
| bytes of Ghidra function size | 2,600 |
| `translated` / `partial` / `stub` | 23 / 0 / 0 |
| high confidence | 21 |
| medium confidence | 2 |
| low confidence | 0 |
| new staging modules | 0 |
| files written | `src/resource/registry.rs`, `src/resource/archive.rs`, this document, `ledger/agents/wave24-resources.csv` |

The selection covers two address bands:

- `0x8016B090..0x8016BC30` — the missing middle of the mount command stack:
  the table that maps a tag byte to an archive manager, the threaded stream
  object that owns the read thread, the executor that drives the wave-15 EGG
  member reader, and the path probe that turns `/<folder>/<name>.arc` into a DVD
  entry number;
- `0x802B85D0..0x802B89A0` — the stateful Yaz0 decoder used when the executor
  must decompress a member instead of returning raw bytes.

The wave-15 queue (`0x8016B440..0x8016BF70`) and the EGG member-read bodies
(`0x802B7ED0..0x802B8150`) are not repeated here; this pass only adds the
functions those bodies called by address.

## Function table

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8016B090` | `mount_archive_table_set` | Publish the archive-table bounds | translated | high | `8016b090_FUN_8016b090.c` |
| `0x8016B0A0` | `mount_archive_table_init` | Install the shipped one-slot table | translated | high | `8016b0a0_FUN_8016b0a0.c` |
| `0x8016B0B0` | `mount_archive_find` | Find an archive manager by tag byte | translated | high | `8016b0b0_FUN_8016b0b0.c` |
| `0x8016B0F0` | `mount_archive_acquire` | Run the found manager's acquire slot | translated | medium | `8016b0f0_FUN_8016b0f0.c` |
| `0x8016B130` | `mount_archive_release` | Run the found manager's release slot | translated | medium | `8016b130_FUN_8016b130.c` |
| `0x8016B170` | `mount_thread` | Read the mount worker thread | translated | high | `8016b170_FUN_8016b170.c` |
| `0x8016B180` | `mount_default_heap` | Read the default mount heap | translated | high | `8016b180_FUN_8016b180.c` |
| `0x8016B1C0` | `mount_stream_construct` | Construct the threaded read stream | translated | high | `8016b1c0_FUN_8016b1c0.c` |
| `0x8016B220` | `mount_stream_manager_init` | Create the stream and resume its thread | translated | high | `8016b220_FUN_8016b220.c` |
| `0x8016B2A0` | `mount_execute` | Read one entry, optionally through a decoder | translated | high | `8016b2a0_FUN_8016b2a0.c` |
| `0x8016B400` | `mount_entry_lookup` | Probe a path without a tag output | translated | high | `8016b400_FUN_8016b400.c` |
| `0x8016B800` | `mount_call_cmd_destroy` | Wait for a call command, then free | translated | high | `8016b800_FUN_8016b800.c` |
| `0x8016B860` | `mount_call_cmd_construct` | Construct a "call one function" command | translated | high | `8016b860_FUN_8016b860.c` |
| `0x8016B8D0` | `mount_call_cmd_create` | Create and queue that command | translated | high | `8016b8d0_FUN_8016b8d0.c` |
| `0x8016BAA0` | `mount_probe_path_variants` | Try every registered extension | translated | high | `8016baa0_FUN_8016baa0.c` |
| `0x8016BC10` | `mount_set_path_fallback` | Enable/disable the fallback probe | translated | high | `8016bc10_FUN_8016bc10.c` |
| `0x8016BC20` | `mount_path_fallback` | Read the fallback flag | translated | high | `8016bc20_FUN_8016bc20.c` |
| `0x8016BC30` | `mount_probe_path` | Path to DVD entry, then fallback | translated | high | `8016bc30_FUN_8016bc30.c` |
| `0x802B85D0` | `lz_stream_init` | Initialise the decoder state | translated | high | `802b85d0_FUN_802b85d0.c` |
| `0x802B8600` | `lz_stream_parse_header` | Verify `Yaz0` and read the size | translated | high | `802b8600_FUN_802b8600.c` |
| `0x802B8700` | `lz_stream_decode` | Consume one source chunk | translated | high | `802b8700_FUN_802b8700.c` |
| `0x802B8970` | `lz_stream_object_init` | Bind output buffer and size | translated | high | `802b8970_FUN_802b8970.c` |
| `0x802B89A0` | `lz_stream_object_feed` | Feed a chunk, report completion | translated | high | `802b89a0_FUN_802b89a0.c` |

## Cluster notes

### Mount archive table and lookup (`0x8016B090..0x8016B180`)

`0x8016B090` stores two SDA words: the first slot and the one-past-last slot of
a pointer table. `0x8016B0A0` points those bounds at the static slot at
`0x80429464`; the slot contains `0x80377AF0`, and the image's `".szs"` literal
sits at `0x80429470`. Each manager object has:

| offset | meaning (`// inferred:` from the lookup bodies) |
|---|---|
| `+0x00` | vtable |
| `+0x04` | tag byte |
| `+0x08` | extension C-string pointer |
| `+0x0C` | acquire virtual slot |
| `+0x10` | release virtual slot |

`0x8016B0B0` is a read-only 4-byte-stride scan comparing `+0x04`.
`0x8016B0F0` and `0x8016B130` call slots `+0x0C` and `+0x10` on the matching
object. `0x8016B170`/`0x8016B180` expose the worker thread and default heap
that `0x8016B220` installs.

### Stream manager and mount executor (`0x8016B1C0..0x8016B2A0`)

`0x8016B220` is the initialiser reached from `0x8008F110`. It stores two heaps,
allocates a 0x4C-byte stream through `0x802B9240(size, heap, alignment)`, lets
`0x8016B1C0` construct it (the base constructor `0x802BA3B0` allocates a
0x4000-byte buffer and an OS thread), copies the thread pointer at `+0x08` to
the global, optionally applies the global DVD heap through `0x802BA670`, and
calls `OSResumeThread` (`0x801B5860`).

`0x8016B2A0` is the executor the wave-15 memory-mount command already calls:

```text
mount_execute(entry, dst, heap, direction, offset, length_out, size_out, mode)
  ├─ mode == 0 -> archive::arc_object_open_entry(entry, dst, heap, ...)
  └─ mode != 0
       ├─ mount_archive_acquire(mode & 0xFF)                  -> decoder
       ├─ lazy arc_object_construct(0x80377B60)
       │    └─ 0x802DC780(global, 0x802B7B50, 0x80377B50)     (destructor)
       ├─ arc_object_open(global, entry)
       ├─ arc_object_load_member(global, decoder, dst, heap,
       │                         direction, offset, 0,
       │                         0x10000, 0, 0)
       ├─ mount_archive_release(mode & 0xFF)
       ├─ size = *(global + 0x70)
       ├─ arc_object_close(global)
       ├─ 0x801AC500(dst, 0x801D4A90(dst))                    (cache write-back)
       └─ *size_out = size; *length_out = size
```

`0x801D4A90` reads the allocation-size word at `dst - 0x0C`, so the write-back
covers the whole allocator block, not only the decoded bytes.

### Path probe (`0x8016B400`, `0x8016BAA0`, `0x8016BC10..0x8016BC30`)

`0x8016BC30` calls the translated DVD path converter
`crate::runtime::dvd::dvd_convert_path_to_entrynum_801ca680`. On a direct hit it
clears the optional tag byte to 0. On a miss, if `0x8016BC20` reports that the
fallback is enabled, it calls `0x8016BAA0`.

`0x8016BAA0` copies the path into a 0x100-byte buffer. The first pass appends
each table entry's `+0x08` extension at the end of the path. The second pass
truncates at the last `/` when present, finds the last `.` with `0x802E1C40`
(`strrchr`), and replaces the extension in place. Every lookup uses the buffer
base, not the append cursor. The matched object's `+0x04` tag is written to
`tag_out` only on a hit.

`0x8016B400` is the same probe with a null tag output.
`0x8016BC10`/`0x8016BC20` are the one-byte enable flag; `0x8008F110` sets it to
1 after the stream manager is created.

### Yaz0 stream decoder (`0x802B85D0..0x802B89A0`)

The state block is described at [`lz_stream_init`]. The decoder is stateful: it
keeps the current tag byte and bit mask, the pending match phase and the
back-distance between calls. `0x802B8600` validates the four magic bytes `Y`,
`a`, `z`, `0` and accumulates the eight-byte big-endian uncompressed size into
`+0x04`, clamped to `+0x08` when that is nonzero. `0x802B8700` consumes one
source chunk and returns the remaining output count, or `-1`/`-3`/`-4`.

The wrappers at `0x802B8970`/`0x802B89A0` bind a destination buffer and size to
that state; the feed wrapper returns 1 when the decoder reports zero remaining.

## Unknowns

1. **Archive-manager type and slots are inferred.** The table, tag byte and
   extension pointer are explicit in the lookup bodies. The two virtual slots
   `+0x0C`/`+0x10` are named "acquire"/"release" from their use in the executor
   and from the extension table; the class itself is not symbolized. The static
   object pointer is `0x80377AF0` and the `".szs"` literal is at `0x80429470`.
2. **`0x802BA3B0` and `0x802BA670` are outside this lane.** They construct the
   EGG stream base and set its heap override. Only their call order and argument
   shapes are used; their bodies are not claimed.
3. **The global archive field at `+0x70` is inferred.** `0x8016B2A0` publishes
   that word as both output lengths. The field's exact name is unknown; the
   code calls it the decoded size.
4. **`0x8016B960` is not an exported function.** It is the execute slot of the
   `0x80329768` vtable: it reads `+0x0C` as a function pointer, passes `+0x10`,
   stores the result at `+0x14`, and marks the command done. Only that contract
   is documented; the body is not translated here.
5. **No Nintendo class symbols are claimed.** `mount_*`, `lz_stream_*` and the
   existing `arc_object_*` names are address-anchored descriptions. `Yaz0` is
   the format name proven by the four magic bytes in `0x802B8600`.
6. **Two original quirks are reproduced, not fixed.** `lz_stream_decode`
   subtracts a negative header result exactly as the C does, and
   `mount_stream_manager_init` has no null check before reading `stream + 8`.

## Verification

- `cargo check` — exit 0. No diagnostic points at either file in this lane.
- `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
- Ledger: `ledger/agents/wave24-resources.csv` (23 rows: 18
  `resource::registry`, 5 `resource::archive`).
- Body count: `src/resource/registry.rs` 42 -> 60 public bodies;
  `src/resource/archive.rs` 33 -> 38. New bodies: **23**.

## Promotion status

All 23 bodies are in their semantic homes, `src/resource/registry.rs` and
`src/resource/archive.rs`. No row lives in `src/subsystems/`. No `lib.rs`,
`mod.rs`, `Cargo.toml`, `platform.rs`, generated doc or other lane file was
changed.
