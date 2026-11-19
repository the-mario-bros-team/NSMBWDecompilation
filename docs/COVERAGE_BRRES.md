# Coverage: BRRES loader (`src/resource/bres.rs`)

## How to read this file

This document covers the **loader side** of BRRES in `sys/main.dol`. It is
organized by the path a resource takes, not by address order:

1. the game finds a named resource slot and its mounted archive;
2. the archive object finds a member path inside the U8 (`.arc`) container;
3. the U8 reader walks its node table and returns the member's data pointer;
4. the BRRES root dictionary turns the file bytes plus a name into a resource
   pointer.

Each row in the table has:

- **address** — the Ghidra entry address (virtual address of `sys/main.dol`);
- **name** — the Rust function in `src/resource/bres.rs`;
- **role** — one plain sentence;
- **status** — `translated` means the full recorded control flow is present;
- **confidence** — `high` when strings/offsets/call order were verified in the
  image, `medium` when one argument or helper identity is inferred;
- **evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C or image bytes, `// inferred:` marks a role/argument reconstructed from call
sites, and `// unknown:` marks an unresolved detail. Format parsing of `MDL0`,
`TEX0`, `CHR0`, etc. is **not** part of this coverage.

## Scope and counts

| Metric | Count |
|---|---:|
| Functions translated | 25 |
| Bytes of Ghidra function size | 4,252 |
| `translated` | 25 |
| `partial` | 0 |
| `stub` | 0 |
| Opaque/helper callee groups documented below | 9 |
| Destination | `src/resource/bres.rs` |
| Staging leftovers | 0 |

The selection is the complete call chain needed to go from an actor's
`g3d/*.brres` path to a BRRES member pointer, plus the acquire/release pair for
the mounted archive object. The survey found 35 Ghidra C files mentioning
`brres` and 11 mentioning `brlyt`; most are actor call sites that only carry a
path string. The functions below are the loader/registry implementations those
call sites actually reach.

Nearby functions that are *not* loader code
(`0x802B60B0..0x802B6C80` animation binding, `0x802B7B00..0x802B7F80` DVD
reader, `0x80239A80`/`0x80239CC0`/`0x80239C40` dictionary internals) are left
opaque or for another lane. The thin enumeration wrappers
`0x802B7830`/`0x802B78A0` call `0x802B7A40` with fixed callbacks; they are
call-site helpers for `0x800DEDB0`, not part of the path from a resource slot
to a BRRES pointer.

## The loader path

```text
fn_800df180(table, slot_name, path)
  └─ fn_800df640(table, slot_name)
       └─ fn_800df570(table, slot_name)      resource-slot scan
            slot+0x28                        mounted archive object
                 └─ fn_802b76c0(archive, path, size_out)
                      ├─ arc_open(archive+0x18, path, info)
                      │    └─ arc_find_index(handle, path)
                      │         └─ U8 node-table walk (case-insensitive)
                      └─ arc_file_data(info) = archive_base + member_offset
                           = pointer to the BRRES bytes

fn_80239e30(file_ptr, name)
  └─ BRRES root dict at file+0x18
       ├─ 0x80239CC0  section "3DModels(NW4R)"   [opaque helper]
       └─ 0x80239C40  named resource in section  [opaque helper]
```

Mount and release:

```text
fn_802b75b0(data, heap, flags)
  ├─ fn_802b74c0(data)          already-mounted scan
  ├─ fn_802b7380(object)        construct + list-append
  └─ fn_802b7460(object,data,1) -> arc_init_handle(data, object+0x18)

fn_802b7680(object)             refcount--; 0 -> vtable+0x08 deleting dtor
```

## BRRES container facts verified here

The following was cross-checked against `scene/scene.brres` extracted from
`Env/Env_course.arc` (6,272 bytes) and against the tag bytes at `0x8034DF60`:

- `+0x00` magic `bres` (`0x62726573`);
- `+0x04` `u16` version (`0xFEFF` in the sampled file);
- `+0x08` total file size;
- `+0x0C` `u16` root-section offset (`0x10` in the sampled file);
- `+0x10` root section: four-byte tag `root`, size, then dictionary metadata;
- root dictionary at **file `+0x18`**;
- dictionary nodes are 16 bytes: `u16` hash/bit index, `u16` pad, `u16` left,
  `u16` right, `u32` name-record offset, `u32` data offset;
- node 0 is the sentinel whose `left` word is the root node index;
- type tags are length-prefixed strings: `3DModels(NW4R)` at `0x8034DF60`,
  `Palettes(NW4R)` at `0x8034DF80`, `Textures(NW4R)` at `0x8034DFA0`,
  `AnmTexPat(NW4R)` at `0x8034E020`, `AnmTexSrt(NW4R)` at `0x8034E040`.

`fn_80239e30` uses the `3DModels(NW4R)` tag and stops at the resource record.
It does not parse the `MDL0` payload, its bones, materials, or textures.

## U8 / ARC facts verified here

- header: magic `55 AA 38 2D`, `+0x04` node-table offset, `+0x08` data offset,
  `+0x0C` string-table offset;
- 12-byte node: `+0x00` top byte type + low 24-bit name offset, `+0x04` data
  offset (file) or parent index (directory), `+0x08` size (file) or
  one-past-children index (directory);
- root node index 0, its `+0x08` word equals the total node count;
- in-memory handle: base, node table, string table, count, node-table end,
  data offset, current directory (0 = root).

The `ARCInitHandle` panic is grounded in the image: `arc.c` at `0x804294A8`,
line 74, message `ARCInitHandle: bad archive format` at `0x8032A3E0`. The
miss warning is `Warning: ARCOpen(): file '%s' was not found under %s in the
archive.` at `0x8032E404`.

## Functions

| Address | Rust | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x800DF570` | `fn_800df570` | Scan the 0x40-byte resource slots for a name | translated | high | `800df570_FUN_800df570.c` |
| `0x800DF640` | `fn_800df640` | Return the named slot only if it has an archive | translated | medium | `800df640_FUN_800df640.c` |
| `0x800DF180` | `fn_800df180` | Load `g3d/*.brres` from the slot's archive | translated | high | `800df180_FUN_800df180.c` |
| `0x8019F660` | `arc_init_handle` | Validate U8 magic and build the 0x1C-byte handle | translated | high | `8019f660_FUN_8019f660.c` |
| `0x8019F9B0` | `arc_file_info_by_index` | Node index -> file-info block (reject dirs) | translated | high | `8019f9b0_FUN_8019f9b0.c` |
| `0x8019FA00` | `arc_find_index` | Resolve a `/` path to a U8 node index | translated | high | `8019fa00_FUN_8019fa00.c` |
| `0x8019F700` | `arc_open` | Find member, fill file-info, warn on miss | translated | high | `8019f700_FUN_8019f700.c` |
| `0x8019FC80` | `arc_build_dir_path` | Build a directory's slash path by parent chain | translated | high | `8019fc80_FUN_8019fc80.c` |
| `0x8019FEA0` | `arc_set_current_dir` | Set handle's current directory if path is a dir | translated | high | `8019fea0_FUN_8019fea0.c` |
| `0x8019FF00` | `arc_open_dir` | Open an iterator over a directory path | translated | high | `8019ff00_FUN_8019ff00.c` |
| `0x8019FF80` | `arc_read_dir` | Read the next entry, skip `.` | translated | high | `8019ff80_FUN_8019ff80.c` |
| `0x8019FE50` | `arc_file_data` | `archive_base + info[1]` | translated | high | `8019fe50_FUN_8019fe50.c` |
| `0x802B7380` | `fn_802b7380` | Construct and register an archive object | translated | medium | `802b7380_FUN_802b7380.c` |
| `0x802B7460` | `fn_802b7460` | Mount U8 data into `object+0x18` | translated | high | `802b7460_FUN_802b7460.c` |
| `0x802B74C0` | `fn_802b74c0` | Find a mounted object by its archive base | translated | medium | `802b74c0_FUN_802b74c0.c` |
| `0x802B7540` | `fn_802b7540` | Lazy-init and append to the global list | translated | high | `802b7540_FUN_802b7540.c` |
| `0x802B75A0` | `fn_802b75a0` | Remove from the global list | translated | high | `802b75a0_FUN_802b75a0.c` |
| `0x802B75B0` | `fn_802b75b0` | Acquire: find or allocate+mount, else refcount++ | translated | medium | `802b75b0_FUN_802b75b0.c` |
| `0x802B7680` | `fn_802b7680` | Release; destroy at refcount zero | translated | high | `802b7680_FUN_802b7680.c` |
| `0x802B76C0` | `fn_802b76c0` | Open member by path, return data pointer + size | translated | high | `802b76c0_FUN_802b76c0.c` |
| `0x802B7760` | `fn_802b7760` | Find member node index by path | translated | high | `802b7760_FUN_802b7760.c` |
| `0x802B7770` | `fn_802b7770` | Open member by index, return data pointer + size | translated | high | `802b7770_FUN_802b7770.c` |
| `0x802B78F0` | `fn_802b78f0` | Recursive directory walk with 4-arg callback | translated | medium | `802b78f0_FUN_802b78f0.c` |
| `0x802B7A40` | `fn_802b7a40` | Enumerate from `/` with a callback | translated | high | `802b7a40_FUN_802b7a40.c` |
| `0x80239E30` | `fn_80239e30` | `3DModels(NW4R)` section -> named resource | translated | high | `80239e30_FUN_80239e30.c` |

## Unknowns and opaque callees

These functions are called by absolute address and are deliberately not
translated in this lane:

| Callee | Role used here | Why opaque |
|---|---|---|
| `0x80239CC0` | find a section by type tag in the BRRES root dict | dictionary internals; shared with other resource types |
| `0x80239C40` | find a named resource inside a section dict | dictionary internals |
| `0x80239A80` | binary name-tree search | callee of `0x80239CC0`; generic NW4R `ResDict` code |
| `0x80239930` | compare two length-prefixed name records | callee of `0x80239A80` |
| `0x80228DD0/8DF0/8F80/8FF0` | NW4R list init/append/remove/next | utility library, not BRRES-specific |
| `0x8015F730` | report/printf sink | OS/debug layer |
| `0x801AD4E0` | fatal assert/panic | OS/debug layer |
| `0x802B9240` | archive-object allocator | allocator layer |
| `0x802BA2D0` | archive-object base constructor | C++ runtime/base class |

Open questions kept in the Rust comments:

- `// unknown:` the archive object's `+0x10` mode word is only proven to be
  `1` when `ARCInitHandle` succeeded; other values are not observed.
- `// inferred:` `fn_800df640`'s two arguments are recovered from the caller,
  because Ghidra shows no parameters and no call arguments.
- `// inferred:` `fn_802b78f0`'s first two arguments come from the outlined
  `__savegprlr` prologue at `0x802DCD60`; the decompiler renders the `r3:r4`
  pair as a `u64`.
- `// inferred:` the list helpers `0x80228xxx` are NW4R `ut::List` operations;
  their names are inferred from their bodies, not from a symbol table.

## Promotion status

All 25 rows are already in their semantic home, `src/resource/bres.rs`. No row
lives in `src/subsystems/`. No `lib.rs`, `mod.rs`, `Cargo.toml`, `platform.rs`
or other lane file was changed.

## Verification

- `cargo check` — exit 0 (no warnings in this file).
- `node tools/doc-lint.mjs src` — `ok: true`, 0 violations.
- Ledger: `ledger/agents/brres.csv` (25 rows, module `resource::bres`).

## Caveats

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic and call ABI are modeled on the Wii's
32-bit big-endian PowerPC. On a 64-bit host, the numeric addresses fit in
`u32`; the functions must only be executed on a target-compatible runtime.
