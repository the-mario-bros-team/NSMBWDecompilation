# Coverage — wave 7 archive-resource registry and U8/EGG archive

## How to read this file

This pass continues the archive-resource work from
`docs/COVERAGE_WAVE4_REGISTRY.md` and `docs/COVERAGE_BRRES.md`. It is organised
by the code path, not by address:

1. **Scope** — what was added, where it lives, and how big it is.
2. **Function table** — one row per new body: address, Rust name, plain-English
   role, status, confidence and Ghidra evidence file.
3. **Cluster notes** — the three flows the rows belong to:
   - manager → registry (`dResMng_c`),
   - member-table enumeration and the EGG archive object,
   - the remaining sound-archive reader/block helpers.
4. **Unknowns** — callbacks and helper bodies that are called by address only.
5. **Verification** — compiler, doc linter and ledger.

Addresses are **virtual addresses of the unrelocated `main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. Status and confidence follow
`docs/READING_GUIDE.md`: `translated` means the complete recorded control flow
is present; `high` confidence means the bytes and call order are explicit, while
`medium` means one callee identity or hidden argument is inferred.

## Scope

| metric | value |
|---|---:|
| functions added | 25 |
| bytes of Ghidra function size | 2,212 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 19 |
| medium confidence | 6 |
| low confidence | 0 |
| new staging modules | 0 |
| files written | `src/resource/registry.rs`, `src/resource/archive.rs`, this document, `ledger/agents/wave7-resources.csv` |

The selection covers five address bands:

- `0x800DF840` — the one-call `dResMng_c` wrapper that completes the registry
  singleton family in `registry.rs`;
- `0x8019FE70..0x8019FE90` — the three U8 file-info accessors/close stub that
  `src/resource/bres.rs` previously had to call by address;
- `0x802B7830..0x802B7E50` — member enumeration wrappers and the EGG archive
  object lifecycle (construct, reset, open, close, wait, destroy);
- `0x8026F440..0x8026FF30` — the remaining sound-archive bodies around the
  reader already translated in wave 5: block-length escape, constructors,
  destructors, header loader, second block reader and write-buffer cleanup;
- `0x801D8150..0x801D8190` — the Nintendo LZ size decoder and stream expander
  that the registry's compressed-member accessors call by address.

## Function table

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x800DF840` | `dres_mng_set_res` | Forward one folder/name pair to the embedded registry | translated | high | `800df840_FUN_800df840.c` |
| `0x8019FE70` | `arc_file_info_offset` | Return the member offset from a U8 file-info block | translated | high | `8019fe70_FUN_8019fe70.c` |
| `0x8019FE80` | `arc_file_info_size` | Return the member size from a U8 file-info block | translated | high | `8019fe80_FUN_8019fe80.c` |
| `0x8019FE90` | `arc_close_file_stub` | No-op U8 close; always returns 1 | translated | high | `8019fe90_FUN_8019fe90.c` |
| `0x802B7830` | `arc_count_files` | Count archive members through the fixed count callback | translated | medium | `802b7830_FUN_802b7830.c` |
| `0x802B78A0` | `arc_get_file_array` | Fill a caller array with member pointers | translated | medium | `802b78a0_FUN_802b78a0.c` |
| `0x802B7AC0` | `arc_object_list_init` | Lazy-init the global archive-object list | translated | high | `802b7ac0_FUN_802b7ac0.c` |
| `0x802B7B00` | `arc_object_construct` | Write the vtable and reset a new archive object | translated | high | `802b7b00_FUN_802b7b00.c` |
| `0x802B7B50` | `arc_object_destroy` | Run the base destructor and optionally free | translated | medium | `802b7b50_FUN_802b7b50.c` |
| `0x802B7BC0` | `arc_object_reset` | Initialise self-pointer, mutexes, conditions and counters | translated | high | `802b7bc0_FUN_802b7bc0.c` |
| `0x802B7C30` | `arc_object_open` | `DVDFastOpen` an entry, set the flag and append to the list | translated | high | `802b7c30_FUN_802b7c30.c` |
| `0x802B7D10` | `arc_object_close` | `DVDClose`, clear the flag and remove from the list | translated | high | `802b7d10_FUN_802b7d10.c` |
| `0x802B7E50` | `arc_object_wait` | Wait on the object condition and clear `+0xC4` | translated | high | `802b7e50_FUN_802b7e50.c` |
| `0x8026F440` | `fn_8026f440` | Decode a `0xFE`-escaped block length | translated | high | `8026f440_FUN_8026f440.c` |
| `0x8026F480` | `fn_8026f480` | Construct the small sound object (vtable `0x8034EEE0`) | translated | medium | `8026f480_FUN_8026f480.c` |
| `0x8026F620` | `fn_8026f620` | Copy `0xD0`-stride records into `this + 8` | translated | high | `8026f620_FUN_8026f620.c` |
| `0x8026F690` | `fn_8026f690` | Construct the sound-archive reader object | translated | medium | `8026f690_FUN_8026f690.c` |
| `0x8026F6E0` | `fn_8026f6e0` | Close, reset and destroy the reader | translated | high | `8026f6e0_FUN_8026f6e0.c` |
| `0x8026FB20` | `fn_8026fb20` | Release a block list and optionally free it | translated | high | `8026fb20_FUN_8026fb20.c` |
| `0x8026FB90` | `fn_8026fb90` | Read and install the 0x40-byte archive header | translated | high | `8026fb90_FUN_8026fb90.c` |
| `0x8026FD00` | `fn_8026fd00` | Read the `+0x118`/`+0x11C` block into the caller buffer | translated | high | `8026fd00_FUN_8026fd00.c` |
| `0x8026FF00` | `fn_8026ff00` | Construct the write-buffer object | translated | high | `8026ff00_FUN_8026ff00.c` |
| `0x8026FF30` | `fn_8026ff30` | Flush the live range and destroy the write buffer | translated | medium | `8026ff30_FUN_8026ff30.c` |
| `0x801D8150` | `lz_get_uncompressed_size` | Decode the LZ header's uncompressed size | translated | high | `801d8150_FUN_801d8150.c` |
| `0x801D8190` | `lz_uncompress` | Expand one LZ stream into the destination | translated | high | `801d8190_FUN_801d8190.c` |

## Cluster notes

### `dResMng_c` wrapper

`0x800DF840` is the last public method of the registry manager. Its argument
order is the trap: the shipped call site `0x8015BE60` passes
`(singleton, "__US_save_banner", "save_banner", 0)`, while
[`dres_set_res`] takes `(registry, name, folder, alloc_dir, heap)`. The wrapper
therefore forwards `param_3` as the name and `param_2` as the folder:

```text
dres_mng_set_res(manager, folder, name, heap)
  └─ dres_set_res(manager + 4, name, folder, 0, heap)
```

### U8 member table and EGG archive object

`0x800DEDB0` (`res_info_load_res`) already allocates `count << 2` bytes and calls
`0x802B7830` / `0x802B78A0` by address. Wave 7 gave those addresses bodies:

```text
0x800DEDB0 (registry)
  ├─ 0x802B7830  arc_count_files(archive)
  │    └─ bres::fn_802b7a40(archive, 0x802B7810, &count)
  └─ 0x802B78A0  arc_get_file_array(archive, files, count)
       └─ bres::fn_802b7a40(archive, 0x802B7870, &{count, files, capacity})
```

The EGG archive object is the long-lived owner of a mounted disc entry:

```text
arc_object_list_init()           one-shot ut::List(0x80423870, 200)
arc_object_construct(obj)        vtable 0x8034FBE0, then arc_object_reset
arc_object_reset(obj)            +0x78 self, two mutexes, two conditions
arc_object_open(obj, entry)      DVDFastOpen -> flag +4 -> list append
arc_object_close(obj)            DVDClose -> clear +4 -> list remove
arc_object_wait(obj)             lock +8, wait +0xA0, clear +0xC4, unlock
arc_object_destroy(obj, del)     slot 0x8034FBF0, optional operator delete
```

`0x8019FE70`/`0x8019FE80` expose the two words that `0x8019F9B0` writes into a
file-info block (`+0x04` offset, `+0x08` size). `0x8019FE90` is the no-op close
stub that `fn_802b76c0` already called by address.

### Sound-archive additions

These bodies fill the gaps around the reader translated in wave 5:

```text
fn_8026f440       0xFE + u16 block-length framing
fn_8026f690       reader constructor   (vtable 0x8034EF68, block +0x108, open byte +0x1D8)
fn_8026f6e0       reader destructor    (close +0x14C, path reset, block release)
fn_8026fb90       read 0x40-byte header, install block +0x108
fn_8026fd00       read block +0x118/+0x11C, publish through 0x80275E20
fn_8026ff00/ff30  write-buffer constructor and flush/destroy
```

The header loader's lazy global is `r13 - 0x4960` (`0x8042AD20`), with its
one-shot byte at `r13 - 0x495C` (`0x8042AD24`); both are defined from
`SDA_BASE` in `archive.rs`.

### Compressed member path

The registry's two compressed accessors previously called the LZ pair by
address. Wave 7 supplies both bodies in `archive.rs`:

```text
dres_get_res_compressed   -> 0x801D8150 lz_get_uncompressed_size
  short header: (w << 24 | (w & 0xFF00) << 8 | (w >> 8) & 0xFF00) >> 8
  long header : full 32-bit byte swap of the next word

dres_copy_res_compressed  -> 0x801D8190 lz_uncompress
  tag byte -> eight literal / back-reference steps
  length nibble: 0 -> 0x11 escape, 1 -> 0x111 escape, else nibble + 3
  distance     : ((word & 0xF) << 8 | next) + 1 bytes behind the output cursor
```

Both routines are byte-oriented and have no output bound checks; the caller
must supply a buffer of `lz_get_uncompressed_size(src)` bytes.

## Unknowns

1. **Enumeration callbacks are unexported.** `0x802B7810` and `0x802B7870` are
   labels inside the EGG archive code with no Ghidra C file. The wrappers'
   user-context layouts are explicit, but the callbacks themselves are called by
   address. `arc_count_files`/`arc_get_file_array` are marked `medium` for that
   reason.
2. **EGG and OS helper identities are inferred.** `0x80228DD0`/`DF0`/`F80` are
   NW4R list init/append/remove, `0x801B2390`/`24C0`/`2E20`/`2E60`/`2F40` are
   OS condition/mutex primitives, and `0x802B9280` is the default deallocator.
   Their names come from their bodies and call shapes; the image has no symbol
   table.
3. **DVD helpers are identified from their bodies.** `0x801CA990` fills a
   0x3C-byte file-info block from a disc entry, `0x801CAA00` calls the DVD close
   path, and `0x801CAB20` contains the `DVDRead(): specified area is out of
   bounds` assertion string.
4. **`0x8034FBF0` is a function-pointer slot.** The body behind the archive
   object's base destructor is not a separate export; `arc_object_destroy` reads
   and calls the slot exactly as the original does.
5. **Sound flush semantics are inferred.** `0x80268D90`/`0x80268EC0` are called
   with `(buffer, start, length, 0)`; their exact cache-flush split is not
   proven, so `fn_8026ff30` is `medium`.
6. **No Nintendo class symbols are claimed.** `dResMng_c`, `info_c`,
   `EGG::Archive` and the reader names are descriptive, cross-checked against
   call sites and vtables.

## Verification

- `cargo check` — exit 0 (no warnings in these two files).
- `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
- Ledger: `ledger/agents/wave7-resources.csv` (25 rows: 1 `resource::registry`,
  24 `resource::archive`).

## Promotion status

All 25 bodies are already in their semantic homes, `src/resource/registry.rs`
and `src/resource/archive.rs`. No row lives in `src/subsystems/`. No `lib.rs`,
`mod.rs`, `Cargo.toml`, `platform.rs`, generated doc or other lane file was
changed.
