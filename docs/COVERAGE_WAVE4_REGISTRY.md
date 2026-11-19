# Coverage — wave 4 archive-resource registry (`0x800DE970..0x800DF7D0`)

## How to read this file

This document covers the **archive-resource registry** reconstructed in
`src/resource/registry.rs`. It is organised the way the code is:

1. **Scope** — what this pass translated, and why the module is called
   `registry` even though the original lane brief named it `bcsv`.
2. **Function table** — one row per function: address, Rust name, plain-English
   role, status, confidence and evidence file.
3. **Cluster notes** — the object model and the four flows (mount, acquire,
   member access, singleton) that the rows implement.
4. **Already translated elsewhere** — the three `dRes_c` helpers that live in
   `src/resource/bres.rs`.
5. **Unknowns / next targets / verification**.

Addresses are **virtual addresses of the unrelocated `main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`.

Status follows `docs/READING_GUIDE.md`:

- `translated` — full recorded control flow and every call is accounted for;
- `partial` — entry/exit and major effects present, one operand unresolved;
- `stub` — address and role only.

Confidence measures the **evidence**, not the code style: `high` means the
bytes and control flow are explicit and cross-checked; `medium` means the flow
is explicit but one callee identity or argument is inferred.

## Scope

| metric | value |
|---|---:|
| functions added to `src/resource/registry.rs` | 20 |
| bytes of Ghidra function size | 2,924 |
| `translated` / `partial` / `stub` | 20 / 0 / 0 |
| high confidence | 17 |
| medium confidence | 3 |
| low confidence | 0 |
| new staging modules | 0 |
| files written | `src/resource/registry.rs`, this document, `ledger/agents/wave4-registry.csv` |

Address span: `0x800DE970..0x800DF7D0`. All 20 entries are at least 16 bytes
and are the complete `dRes_c` / `info_c` implementation: the registry header,
holder lookup, mount completion, member-table build, typed member accessors and
the `dResMng_c` singleton.

### Why this module is `registry.rs`

Wave 4 originally scoped this file as a BCSV/table parser and named it
`bcsv.rs`. A check of the shipped build found **no BCSV container or parser**:

- `sys/main.dol` contains no `BCSV` magic (`42 43 53 56`) at any offset;
- `work/arcs/index.json` lists 21 `.csv` members, but all are plain text
  (WorldMap route/point tables and `HomeButton2/home.csv`), not BCSV rows;
- no `.bcsv` literal appears in `work/dol/strings.txt`.

The table implementation that actually exists in this address band is the
archive-resource registry. It has the same shape the BCSV brief described —
header, fixed-stride rows, name lookup, typed accessors and row iteration — so
it is reconstructed here rather than inventing a parser that is not in the
binary.

## Function table

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x800DE970` | `res_info_destroy` | Wait for the DVD command, clean up, optionally free the holder | translated | high | `800de970_FUN_800de970.c` |
| `0x800DE9F0` | `res_info_cleanup` | Adopt the command's archive, free heap/binaries, unmount | translated | high | `800de9f0_FUN_800de9f0.c` |
| `0x800DEAD0` | `res_info_set` | Build `/<folder>/<name>.arc` and start a mount | translated | medium | `800dead0_FUN_800dead0.c` |
| `0x800DEBA0` | `res_info_set_res` | Finish a mount, create the data heap and member table | translated | medium | `800deba0_FUN_800deba0.c` |
| `0x800DEDB0` | `res_info_load_res` | Allocate `mpFiles` and enumerate the archive | translated | high | `800dedb0_FUN_800dedb0.c` |
| `0x800DEF50` | `dres_construct` | Zero the three-word registry header | translated | high | `800def50_FUN_800def50.c` |
| `0x800DEF70` | `dres_destroy` | Destroy the holder array and free the registry | translated | high | `800def70_FUN_800def70.c` |
| `0x800DEFF0` | `dres_init` | Allocate and construct the holder array | translated | high | `800deff0_FUN_800deff0.c` |
| `0x800DF080` | `dres_set_res` | Find-or-create a holder and bump its refcount | translated | high | `800df080_FUN_800df080.c` |
| `0x800DF1E0` | `dres_get_res` | Member pointer + size by path, logging open | translated | high | `800df1e0_FUN_800df1e0.c` |
| `0x800DF270` | `dres_get_res_compressed` | Member pointer + size/compression, `.LZ` fallback | translated | high | `800df270_FUN_800df270.c` |
| `0x800DF3A0` | `dres_copy_res` | Copy one uncompressed member | translated | high | `800df3a0_FUN_800df3a0.c` |
| `0x800DF3B0` | `dres_copy_res_compressed` | Copy or LZ-decompress one member | translated | medium | `800df3b0_FUN_800df3b0.c` |
| `0x800DF3C0` | `dres_get_res_silently` | Member pointer by path, no warning | translated | high | `800df3c0_FUN_800df3c0.c` |
| `0x800DF440` | `dres_get_res_silently_size` | Member pointer + size, no warning | translated | high | `800df440_FUN_800df440.c` |
| `0x800DF4E0` | `dres_sync_all_res` | Advance one pending mount and report progress | translated | high | `800df4e0_FUN_800df4e0.c` |
| `0x800DF600` | `dres_new_res_info` | First free holder (refcount zero) | translated | high | `800df600_FUN_800df600.c` |
| `0x800DF680` | `dres_mng_construct` | Construct the singleton and install 300 holders | translated | high | `800df680_FUN_800df680.c` |
| `0x800DF6E0` | `dres_mng_destroy` | Clear the singleton and destroy the registry | translated | high | `800df6e0_FUN_800df6e0.c` |
| `0x800DF7D0` | `dres_mng_set_res_array` | Schedule an array of archive names | translated | high | `800df7d0_FUN_800df7d0.c` |

## Cluster notes

### Registry model

`dRes_c` is a three-word header:

| offset | field |
|---|---|
| `+0x00` | `mpArcInfos`, pointer to the first holder |
| `+0x04` | `u16 mNumArcs` |
| `+0x08` | `mpCallback` |

Each holder is a 0x40-byte `info_c`:

| offset | field |
|---|---|
| `+0x00` | `char mName[0x20]` |
| `+0x20` | `u16 mRefCount` |
| `+0x24` | pending `mDvd_mountMemArchive_c *` |
| `+0x28` | mounted `EGG::Archive *` |
| `+0x2C` | mount heap |
| `+0x30` | raw archive bytes |
| `+0x34` | archive size |
| `+0x38` | frame heap holding loaded members |
| `+0x3C` | `u8 **mpFiles` |

`dres_init` allocates `count * 0x40 + 0x10` bytes (the extra 0x10 is the
Metrowerks array cookie) and uses `__construct_array` with the unexported
in-object constructor `0x800DE940` and the holder destructor `0x800DE970`.
`dres_destroy` reverses that with `__destroy_arr`.

### Mount lifecycle

```text
dres_set_res(table, arc, folder, allocDir, heap)
  ├─ bres::fn_800df570(table, arc)          existing holder?
  │    └─ hit: refcount++
  └─ miss:
       ├─ dres_new_res_info(table)          refcount == 0 row
       ├─ res_info_set(row, arc, folder, allocDir, heap)
       │    ├─ snprintf("/%s/%s.arc", folder, arc)
       │    ├─ mDvd_mountMemArchive_c::create(path, allocDir, heap)
       │    └─ strncpy(row->mName, arc, 0x20)
       └─ on mount failure: res_info_cleanup(row)

dres_sync_all_res(table)
  └─ res_info_set_res(row, callback)
       ├─ command not done  -> 1
       ├─ adopt archive/binary/heap/size, destroy command
       ├─ create frame heap "dRes_c::info_c::mDataHeap"
       ├─ res_info_load_res(row, callback)
       │    ├─ mpFiles = new[count]
       │    └─ getFileArray(...) or searchInside(callback context)
       ├─ restore heap + adjust frame heap
       └─ DCStoreRangeNoSync(dataHeap, end - dataHeap)
```

### Typed member accessors

| Rust | Open path | Returns |
|---|---|---|
| `dres_get_res` | `fn_802b76c0` (`getFile`, logs on miss) | data + `FileInfo.size` |
| `dres_get_res_compressed` | `fn_802b7760`/`fn_802b7770`, then `.LZ` retry | data + size + compression nibble |
| `dres_get_res_silently` | `fn_802b7760`/`fn_802b7770` | data only, no warning |
| `dres_get_res_silently_size` | same, with `FileInfo` | data + size |

The LZ suffix comes from the literal `"%s.LZ"` at `0x80427F78`. The compression
nibble is `data[0] & 0xF0`; the uncompressed size comes from
`CXGetUncompressedSize` (`0x801D8150`). `dres_copy_res_compressed` dispatches to
`memcpy` (`0x80004364`) for type 0 and `CXUncompressLZ` (`0x801D8190`)
otherwise.

### Singleton

`dres_mng_construct` installs the manager vtable `0x80319550`, the embedded
`dRes_c` at `+0x04`, and the callback vtable `0x80319560` at `+0x10`, stores
itself at `0x8042A038` (`r13 - 0x5648`), and calls `dres_init(300)`.
`dres_mng_destroy` clears that global, destroys the embedded registry with the
non-deleting flag `-1`, and frees the manager only when its deleting flag is
positive.

## Already translated elsewhere

These three functions are the same table family and stay in
`src/resource/bres.rs`; this module calls them rather than duplicating them:

| Address | Rust | Role |
|---|---|---|
| `0x800DF570` | `bres::fn_800df570` | `getResInfo`: refcount > 0 + `strcmp` scan |
| `0x800DF640` | `bres::fn_800df640` | `getResInfoLoaded`: slot with non-null archive |
| `0x800DF180` | `bres::fn_800df180` | `getRes` wrapper for `g3d/*.brres` |

## Unknowns and next targets

1. **Class names are inferred.** The image has no symbol table. `dRes_c`,
   `info_c`, `dResMng_c` and the field names follow the call shapes and the
   vtable at `0x80319550`; they are descriptive, not claimed Nintendo symbols.
2. **EGG helper identities are inferred from the address map.**
   `0x802B7680` is `EGG::Archive::unmount`, `0x802B7830`/`0x802B78A0`/
   `0x802B7A40` are the archive enumeration trio, and `0x802B8F70`/`0x802B9250`/
   `0x802B9280` are the heap layer. The call shapes and the P1/E1 address delta
   (`-0x140` in this band) agree, but no symbol table is present.
3. **`0x800DE940` is not an exported function.** It is passed to
   `__construct_array` as the element constructor. Ghidra produced no C file for
   it; it is referenced by address exactly as the caller does.
4. **`0x800DEDA0` is not an exported function.** It is the static
   `searchCallback_c` thunk handed to `searchInside`; again, the caller supplies
   it by address.
5. **`res_info_set_res` status values.** The C returns `-1` (error), `1`
   (DVD command still running) and `0` (loaded/success). `dres_sync_all_res`
   stops on a positive value; both are reproduced as observed, not renamed.
6. **Next targets in the same family:**
   - `0x800DF840` (28 B) — thin `dResMng_c::setRes(path, name, heap)` wrapper;
   - `0x800DF760` (56 B) — vtable-only `resCallback_c::execute` that checks
     `folderSig == 'g3d '` and runs `ResFile::CheckRevision/Init/Bind`; it has no
     Ghidra C export, so it needs a raw-disassembly pass;
   - `0x800DF750` (4 B) — empty `resCallback_c::init` thunk, below the
     size floor.

## Verification

- `cargo check` — exit 0. (An intermediate run hit
  `src/game/bases.rs:4210` in another lane; that lane has since fixed it, and
  the final run is green.)
- `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
- Ledger: `ledger/agents/wave4-registry.csv` (20 rows, module `resource::registry`).

## Promotion status

All 20 functions are already in their semantic home, `src/resource/registry.rs`.
No row lives in `src/subsystems/`. No `lib.rs`, `mod.rs`, `Cargo.toml`,
`platform.rs`, generated doc or other lane file was changed.
