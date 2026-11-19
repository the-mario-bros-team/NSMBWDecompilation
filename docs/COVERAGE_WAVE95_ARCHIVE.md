# Coverage - Wave 95 (resource::archive, `0x802B0000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 25 bodies are appended after the wave-92
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 25 | 4848 | 5 | 20 |

The file now holds 458 translated bodies: 433 before this pass plus the 25
below. The assigned `0x802B0000` bucket had 66 untranslated functions when this
lane started; 25 of them are taken here.

## What this pass covers

This pass takes the `0x802B0000`-band scene/effect helpers whose offsets,
masks, constants and callee addresses are all visible in the exports, plus the
two GX setup bodies at the end of the band:

1. **Small vtable objects** - `0x802B1C90`, `0x802BB030`, `0x802BC7A0`,
   `0x802BF3D0` (4 bodies). Each installs a literal vtable (`0x8034FA80`,
   `0x8034FED8`, `0x80350030`, `0x803500D8`), clears a handful of fixed fields
   and runs one initialiser whose receiver is the same object.
2. **Float table and search helpers** - `0x802B4AE0`, `0x802B4DF0`,
   `0x802B1E70` (3 bodies). The first reads an `(x, y)` pair out of a
   `0x1C`-byte entry table with a default fallback; the second binary-searches
   an eight-byte-stride `(float, u16)` table using the two `r2 - 0x3F90` /
   `r2 - 0x3F8C` tolerance bounds; the third rescales six floats by powers of
   two and forwards the exponent to `0x801C7E00`.
3. **Manager and texture wrappers** - `0x802BAED0`, `0x802BC120`, `0x802BC1A0`
   (3 bodies). They read the singleton at `r13 - 0x4838`, enqueue one texture
   colour, and build the two-key record whose payload is allocated through
   `0x802B9270(size, heap, 0x20)`.
4. **Controller array and draw state** - `0x802BC800`, `0x802BD370`,
   `0x802BD8C0`, `0x802BD420`, `0x802BD460` (5 bodies). The initialiser clears
   the five-word colour block, both 3x4 matrices and the `+0xB44`/`+0xB45`
   draw-state bytes; the other four are the guarded child read, the child
   release loop, the handler forwarder and the optional two-word callback.
5. **Effect-object bootstrap** - `0x802BD310` (1 body). Lazily allocates
   `0xE60` bytes and stores the constructed object in both `r13 - 0x4810`
   slots.
6. **Effect transform and normalisers** - `0x802BC9E0`, `0x802BE580`,
   `0x802BE640`, `0x802BE6B0`, `0x802BEA00` (5 bodies). The twelve-float copy
   is a pure register/stack move; the two normalisers call `0x802BE470` with
   the squared length and divide by its result; the transform rebuild snapshots
   the old matrix and writes three normalised basis rows plus their translation
   terms.
7. **GX setup** - `0x802BED10`, `0x802BEF00` (2 bodies). One programs the 2D
   blit pipeline through the `0x801C8xxx`/`0x801C4xxx` call chain; the other
   writes one spliced descriptor word to the `0xCC008000` FIFO port.
8. **Small accessors and forwarders** - `0x802B4940`, `0x802BE880` (2 bodies).
   A virtual `+0x14` forward and a virtual `+0x08` result forwarded to
   `0x80253BE0`.

### Shared layout notes

`// inferred:` the owners are RLYT/scene records plus one screen-effect object;
the exports prove only the offsets, the masks, the constants and the callee
addresses.

- the controller at `0x802BC7A0`/`0x802BC800` keeps its child count at `+0x18`,
  an intrusive list header at `+0x14`, the four-word child array at `+0x1C`,
  the buffer block at `+0xB20..+0xB34`, two 3x4 matrices at `+0xB4C` and
  `+0xB7C`, and the draw-state bytes at `+0xB44`/`+0xB45`;
- the effect object is `0xE60` bytes, with the transform at `+0x04..+0x30`,
  its previous copy at `+0x34..+0x60`, and the input vectors at
  `+0x64..+0x84`;
- the normalisers read the same two `r2` bound floats, `0x8042F198` and
  `0x8042F19C` (`SDA2_BASE - 0x3EE8` / `- 0x3EE4`); neither value is invented;
- the manager singleton read as `r13 - 0x4838` is the same pointer the wave-79
  and wave-90 sections already used.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B1C90` | `fn_802b1c90` | 60 | medium |
| `0x802B1E70` | `fn_802b1e70` | 512 | medium |
| `0x802B4940` | `fn_802b4940` | 20 | high |
| `0x802B4AE0` | `fn_802b4ae0` | 112 | high |
| `0x802B4DF0` | `fn_802b4df0` | 232 | medium |
| `0x802BAED0` | `fn_802baed0` | 156 | medium |
| `0x802BB030` | `fn_802bb030` | 172 | medium |
| `0x802BC120` | `fn_802bc120` | 116 | medium |
| `0x802BC1A0` | `fn_802bc1a0` | 100 | medium |
| `0x802BC7A0` | `fn_802bc7a0` | 96 | medium |
| `0x802BC800` | `fn_802bc800` | 180 | high |
| `0x802BC9E0` | `fn_802bc9e0` | 1012 | medium |
| `0x802BD310` | `fn_802bd310` | 72 | medium |
| `0x802BD370` | `fn_802bd370` | 56 | medium |
| `0x802BD420` | `fn_802bd420` | 56 | high |
| `0x802BD460` | `fn_802bd460` | 60 | high |
| `0x802BD8C0` | `fn_802bd8c0` | 168 | medium |
| `0x802BE580` | `fn_802be580` | 68 | medium |
| `0x802BE640` | `fn_802be640` | 108 | medium |
| `0x802BE6B0` | `fn_802be6b0` | 132 | medium |
| `0x802BE880` | `fn_802be880` | 68 | medium |
| `0x802BEA00` | `fn_802bea00` | 604 | medium |
| `0x802BED10` | `fn_802bed10` | 488 | medium |
| `0x802BEF00` | `fn_802bef00` | 132 | medium |
| `0x802BF3D0` | `fn_802bf3d0` | 68 | medium |

## Unknowns

1. **The `r2` float pairs.** The normalisers and the search tolerance read
   `f32` values from `SDA2_BASE - 0x3EE8..- 0x3F8C`. The bodies read the slots by
   address; their values, units and the exact "length limit" meaning are not
   recovered.
2. **Dropped receivers.** Several exports print calls with no arguments where
   the receiver is still in `r3` (`0x802B1C90`'s base constructor, `0x802BC7A0`'s
   `0x802BC800`, `0x802BE880`'s virtual `+0x08`, `0x802BF3D0`'s `0x802BF540`).
   The bodies pass the obvious object and say so at the call site.
3. **`0x802BE470`.** Both normalisers pass the squared length to it and divide
   by the returned value; the body is called by address through the PPC double
   ABI, and the "root" role is inferred from the divide, not from its own
   export.
4. **`0x802BC9E0`'s dead copies.** The original copies the first basis into
   `local_1C`/`local_18`/`local_14` before normalising; those copies are never
   read on any reachable path, so the body omits them and says so here.
5. **`0x802BEF00`'s first argument.** The export takes an `undefined4` that is
   never read; it is kept as `_context` for call-site parity.

## Deliberate skips

- **`0x802B4B50`, `0x802B4EE0`, `0x802BBF40`, `0x802BDE60`, `0x802BF540`,
  `0x802BFDA0`, `0x802BFF70`** - the `CONCAT44(0x43300000, ...)`,
  `unaff_GQR0`/`ldexpf` and GX-register families. They need the double-precision
  constant materialisation and quantisation-scale path that `src/platform.rs`
  does not model.
- **`0x802B5CC0`, `0x802B5DA0`, `0x802B7310`, `0x802B5E50`, `0x802B60B0`,
  `0x802BBFD0`, `0x802BD4A0`, `0x802B6C80`, `0x802B2C70`, `0x802B20B0`,
  `0x802B0680`** - the `FUN_802DCDxx` register-pair thunks. Their real
  signatures return an `(r3, r4)` pair, which the `call0..call7` helpers cannot
  express honestly.
- **`0x802BC980`, `0x802BC9A0`, `0x802BC9C0`** - each checks a `+0xB48` field
  and then tail-calls a four-argument list helper (`0x802BDF70`/`0x802BDFC0`/
  `0x802BDDF0`) with every argument stripped. The forwarding cannot be proved
  from the export.
- **`0x802BBFA0`, `0x802BB580`, `0x802BF930`** - wrappers whose calls drop
  either the receiver or a float argument that the callee actually uses.
- **`0x802BF650`, `0x802BFE90`, `0x802B4B50`** - the callee takes `double`
  arguments in `f1..f8`; no `call*` helper passes float registers, and modelling
  a private float ABI would be a guess.
- **`0x802BE470`, `0x802BE4A0`, `0x802BE4D0`, `0x802BE500`** - three-line
  wrappers that call an unknown `0x802E84xx`/`0x802E86xx` double function with
  no visible arguments and narrow its result to `f32`.
- **`0x802BE530`, `0x802BE5D0`** - four-byte thunks whose targets are exported
  as 250-330 line inlined matrix/FIFO bodies; deferred rather than padded.
- **`0x802BB9A0`, `0x802BBDD0`-family neighbours, `0x802BD970`** - eight
  argument call sites and multi-list update loops left for a dedicated pass.
- **`0x802BB840`, `0x802BD8C0`'s siblings above `0x802BD970`** - controller
  commit/draw paths whose virtual slots are called with stripped receivers and
  whose arguments are not recoverable from the export.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 25 addresses came from
  the untranslated part of the `0x802B0000` bucket.
- The module header gained item 14 and its body count is 25.
- `node tools/check-lane-csv.mjs wave95` reports 25 rows for
  `wave95-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`; the remaining errors name other lanes' files from
  the same wave, which are being edited in parallel.
