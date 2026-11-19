# Coverage - Wave 40 (resource::archive)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 18 | 1448 | 9 | 5 |

9 + 5 = 14; the remaining four bodies are `partial`/`low` (the LZ completion
tests at `0x802B8450..0x802B8570`) and are listed in neither the `High` nor the
`Medium` column, because they omit arguments the export does not show. The file
now holds 122 bodies.

## What this pass covers

The assigned bucket `0x802B0000` has 225 untranslated functions of 306. Most of
the band is **not** archive code: `0x802B0000..0x802B1B00` and
`0x802B1C90..0x802B6C80` are the NW4R text/font renderer and the RLYT layout
object, `0x802B6AE0..0x802B7310` are its colour/GX helpers, and
`0x802BAC30..0x802BE600` is the EGG display/controller family. This pass takes
the three archive-owned clusters:

1. **U8 member table** — `0x802B63B0..0x802B6860` (12 bodies). The object scans
   a U8 archive for a named member: `0x802B6410` opens the scan root through
   `0x8019FF00` (translated as
   [`crate::resource::bres::arc_open_dir`]), reads entries through `0x8019FF80`
   ([`crate::resource::bres::arc_read_dir`]), descends with `0x8019FEA0` and
   compares names with `0x802E886C`; `0x802B6680` builds the 0x1C-byte U8 handle
   through `0x8019F660` ([`crate::resource::bres::arc_init_handle`]), stores the
   image pointer at `+0x20` and copies up to `0x7F` name bytes to `+0x30`. The
   rest is the object's constructor (`0x802B6620`, vtable `0x8034FBB8`, empty
   list at `+0x28`), the take/clear accessor (`0x802B6700`), the list lookup
   (`0x802B6740`, key `+0x08`, payload `+0x88`), the two bounded name compares
   (`0x802B67B0`/`67E0`, `strncmp` 0x802E1BD0 with 0x10/0x14), a magic +
   `0xFEFF` header test (`0x802B6810`), the header clear (`0x802B6840`), the
   child-heap release (`0x802B6860`, vtable-slot call `0x801D5550` on the EGG
   root heap at `r13 - 0x48B8`) and the guarded delete (`0x802B63B0`).
2. **LZ decoder bindings** — `0x802B8420..0x802B8570` (5 bodies), sitting
   between the translated member loader `0x802B8150` and the translated Yaz0
   stream decoder `0x802B85D0`. `0x802B8420` stores an output range at
   `+0x04`/`+0x08` and initialises the decoder state at `+0x0C` through
   `0x801D6880`; the four 44-byte bodies call one decoder each
   (`0x801D6B60`, `0x801D68C0`, `0x801D6F60`, `0x801D7D50`) and reduce its
   result with the `countLeadingZeros(x) >> 5` idiom, i.e. `result == 0`.
3. **EGG root heap** — `0x802BA870` (372 bytes). This is the initialiser that
   the whole archive allocator rests on: it reads four arena bounds through
   `0x801ABE60/70/90/A0`, aligns and records them in the root descriptor, calls
   the four setters, initialises the heap list through `0x802B8B40`, splits the
   last 64 MiB of MEM2 off as the debug heap when MEM2 is large enough, builds
   the MEM1/MEM2/debug heaps through `0x802B9360`, tags each with
   `s_EGGRootMEM1/MEM2/Debug`, builds the system heap through `0x802B9430`,
   tags it `s_EGGSystem` and makes it current through
   [`arc_heap_exchange_current`].

It reuses the field layout already documented for the heap factories: the name
pointer lives at `+0x34` of every heap object, exactly as
[`fn_802b9360`] / [`fn_802b9430`] already assume.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B63B0` | `fn_802b63b0` | 64 | high |
| `0x802B63F0` | `fn_802b63f0` | 16 | high |
| `0x802B6410` | `fn_802b6410` | 204 | high |
| `0x802B6620` | `fn_802b6620` | 84 | medium |
| `0x802B6680` | `fn_802b6680` | 124 | high |
| `0x802B6700` | `fn_802b6700` | 20 | high |
| `0x802B6740` | `fn_802b6740` | 112 | medium |
| `0x802B67B0` | `fn_802b67b0` | 44 | high |
| `0x802B67E0` | `fn_802b67e0` | 44 | high |
| `0x802B6810` | `fn_802b6810` | 40 | high |
| `0x802B6840` | `fn_802b6840` | 20 | high |
| `0x802B6860` | `fn_802b6860` | 80 | medium |
| `0x802B8420` | `fn_802b8420` | 48 | medium |
| `0x802B8450` | `fn_802b8450` | 44 | low |
| `0x802B84B0` | `fn_802b84b0` | 44 | low |
| `0x802B8510` | `fn_802b8510` | 44 | low |
| `0x802B8570` | `fn_802b8570` | 44 | low |
| `0x802BA870` | `fn_802ba870` | 372 | medium |

## Cluster notes

- `0x802B6410` is recursive. The export's `do { while (1) { ... } } while
  (strcmp != 0)` is kept: the inner loop reads entries, descends into
  directories (restoring the parent path afterwards) and breaks out on the
  first file, which is then compared; on inequality the outer loop reads the
  next entry. `-1` is the "not found" sentinel and the end-of-directory path
  returns the last recursion result.
- `0x802B6680` calls the U8 handle builder with `(data, this + 4)`, so the
  0x1C-byte handle sits at `+0x04` while the image pointer is stored separately
  at `+0x20`; the name copy is bounded at `0x7F` and the byte at `+0xAF` is
  forced to NUL.
- `0x802B6810` compares the incoming magic with the header's first word and the
  halfword at `+0x04` with `-0x101` (unsigned `0xFEFF`). Which container that
  magic belongs to is not visible in the export, so it is marked `// unknown:`.
- `0x802B8420`'s call to `0x801D6880` only shows one argument; the callee's
  signature is `(state, output)` and `r4` still holds the first argument at the
  call, so `output` is forwarded. This is the only inferred argument in the
  body.
- `0x802BA870` starts with the CodeWarrior register-save stub `0x802DCD70`,
  whose result is the incoming `r3`; the reconstructed signature therefore
  takes the root descriptor as its argument, matching the idiom already used by
  [`fn_802b9360`] and `resource::registry`.

## Unknowns

1. **Two path strings.** `0x802B6410` opens `r13 - 0x5B78` and restores
   `r13 - 0x5B74` after descending. The strings live in the data section and
   are not printed by the export; they are named `G_U8_SCAN_ROOT` and
   `G_U8_SCAN_PARENT` with `// unknown:`/`// inferred:` comments.
2. **Arena getter/setter names.** `0x801ABE60/70/90/A0` and the four setters
   `0x801ABEC0/BED0/BEF0/BF00` have no symbols. Their pairing (both setters of
   a pair receive the same bound) is literal; which bound is low and which is
   high is inferred.
3. **`0x801D5550`'s freed pointer.** In `0x802B6860` the export passes only the
   root heap to `0x801D5550`, which calls vtable slot `+4` with one argument;
   the block at `+0x04` is not forwarded. The body reproduces the observed call
   and says so.
4. **The four LZ decoder arguments.** `0x801D6B60`, `0x801D68C0` and
   `0x801D7D50` take `(state, src, len)` but their wrappers show only the state
   in `r3`; `0x801D6F60` is exported as `void`, so the value tested by
   `countLeadingZeros` is whatever it leaves in `r3`. These four bodies are
   `partial`/`low` for that reason; the call, the state offset and the
   `== 0` reduction are certain.
5. **No Nintendo class symbols are claimed.** Vtables are given as
   `0x8034FBxx` addresses and the U8 helpers are referenced through their
   `resource::bres` names rather than invented container classes.

## Deliberate skips

- `0x802B0680..0x802B1B00`, `0x802B1D60..0x802B1E70`, `0x802B2070..0x802B2C70`
  — NW4R text/font rendering and the RLYT layout constructor: float/paired-
  single code (`CONCAT44`, `ldexpf`, `GQR0`) plus `r2`-relative constants whose
  absolute address is unrecoverable from the C export.
- `0x802B2720..0x802B2B50` — RLYT header accessors (12/20/36-byte bitfield
  getters and copies) belonging to the object whose destructor is
  `0x802B2BC0`; different subsystem, no archive field.
- `0x802B3300..0x802B3940`, `0x802B4AE0..0x802B4DF0`, `0x802B4EE0..0x802B50D0`
  — glyph width tables and GX text drawing.
- `0x802B5AF0..0x802B5E50`, `0x802B60B0..0x802B6320` — the RLAN layout
  animation reader (`0x524C414E`, `pat1`/`pah1`/`pai1`) and its list helpers;
  a resource decoder, not an archive reader.
- `0x802B68B0`/`0x802B6950`/`0x802B6A40` — heap-backed 0x20-byte record arrays
  called only from the text band (`0x802AD920`, `0x802AF6F0`); heap-allocated
  but not archive-owned.
- `0x802B6AE0..0x802B7310` — colour scaling and GX display-list emission for the
  text renderer.
- `0x802B8420`'s neighbours `0x802B8600..0x802B89A0` were already translated in
  wave 24 and were re-read first, not repeated.
- `0x802B9700` — 4-byte `thunk_FUN_801d4a90`; thunks are skipped per the brief.
- `0x802BAC30..0x802BE600` — EGG display/controller objects (`0x8034FEB8`,
  `0x8034FF00`, `0x80350004`, `0x80350030` vtables), including
  `EGG::CoreControllerMgr` at `0x802BD4A0` and the `0x802BE100` delete
  trampoline. The only heap contact is the buffer object at `0x802BC670`,
  whose `0x801C1DB0` callee turned out to be the 163-line GX initialiser, so it
  is not an archive buffer and was left alone.

## Verification

- Every address and byte size matches `functions.tsv` (read from the TSV, not
  from prose); `node tools/check-lane-csv.mjs wave40` cross-checks all
  eighteen `(N bytes)` claims and reports no problem naming
  `wave40-archive.csv`.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/802bxxxx_FUN_802bxxxx.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; none of the eighteen is in
  the wave 24/30/36/39 ranges.
- `cargo check --message-format=short` is free of errors for
  `src/resource/archive.rs` and adds no warning in that file.
