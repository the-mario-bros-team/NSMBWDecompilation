# Coverage - Wave 92 (resource::archive, `0x80260000` bucket)

## How to read this file

Every row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, its exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The 25 bodies are appended after the wave-90
section at the end of `src/resource/archive.rs`, in address order.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 25 | 8884 | 4 | 21 |

The file now holds 433 translated bodies: 408 before this pass plus the 25
below. The assigned `0x80260000` bucket had 75 untranslated functions of 260
when this lane started; 25 of them are taken here.

## What this pass covers

The remaining `0x80260000` band is NW4R sound-manager infrastructure: the
resident initialiser, the member-record upload helpers, the descriptor lookup
chain, the intrusive list teardown pair and the five-float voice parameter
stores. This pass takes the bodies whose offsets, masks, constants and callee
addresses are all visible in the exports:

1. **Resident init and per-slot release** - `0x80260950`, `0x80261640`,
   `0x80265100` (3 bodies). The once-only initialiser clears two global blocks
   and writes six SDA link words; the per-slot release walks an intrusive list
   at `+0xB8` and resets the matching voice channel; the queue drain releases
   every resident record and lazily registers its three list heads.
2. **Mode and cursor arithmetic** - `0x80261E70`, `0x80262540` (2 bodies). The
   pair tests whether the child's 32-bit cursor lies in the mode's
   `width..edge` window, then reports its distance from the lower edge. Both
   share the `mode == 3 ? width * 2 + 2 : mode == 2 ? width : mode == 1 ?
   width >> 1 : 0` ladder.
3. **Member-record uploads** - `0x802628A0`, `0x80262E90`, `0x80263B90`,
   `0x80264400`, `0x80264760` (5 bodies). One snapshot of twelve halfwords,
   one 0x28-byte mode-sized parameter block, one duration split through
   `0x801A3290`, and the two large block writers that pair 24 resp. 16
   halfwords with a packed channel flag word.
4. **Descriptor lookup chain** - `0x80265780`, `0x80265A30`, `0x80265890`
   (3 bodies). The index resolver, the kind-2/kind-3 table walker it calls
   twice, and the record filler that copies the descriptor into the caller's
   0x1C-byte record, including the `0x101` version split.
5. **Large field initialisers** - `0x80265FF0`, `0x802660D0`, `0x80267E30`
   (3 bodies). The vtable/field block, the doubled counter-and-range ladder,
   and the stream-record constructor that initialises the block at `+0x1C`.
6. **Intrusive list and node helpers** - `0x80269F00`, `0x8026A010`,
   `0x8026A2A0`, `0x8026A350` (4 bodies). The inner-list drain and free pair,
   the payload-node allocator, and the heap-grow helper that returns
   `count - 1`.
7. **Voice parameter stores** - `0x8026A880`, `0x8026ADD0`, `0x8026B370`,
   `0x8026B900` (4 bodies). The constructor-called five-float stores for the
   first four member classes. Each clamps every parameter between two `r2`
   small-data floats and then runs the class's voice-set dirty test.
8. **Layered offset query** - `0x8026DF50` (1 body). Three nested reader
   lookups (`0x80275840`, `0x80275800`, `0x80275810`) summed with the
   `+0x108` data-cursor base.

### Shared layout notes

`// inferred:` the owners are the NW4R sound manager at `0x803FC810` and its
member objects; the exports prove the offsets, the masks, the constants and the
callee addresses only.

- the manager keeps its record-list head at `+0x18` and current node at
  `+0x1C`; member lists live at `+0xB4 + slot * 0xC` with links at `+0xB8`;
- member records keep the child handle at `+0x00`, the changed-flag word at
  `+0x04`, the cursor at `+0x08` (mirrored to the child's `+0x92`/`+0x94`),
  the ready word at `+0x10` and the mode at `+0x14`;
- stream records keep the open byte at `+0x0C`, the mode word at `+0x10`,
  the block at `+0x1C`, the state bytes at `+0x35..+0x37` and the stream
  handle at `+0xD0`;
- every voice parameter store clamps with the same ladder,
  `v <= upper ? (v < lower ? lower : v) : upper`, which is factored into
  `clamp_param_f32` so the four bodies cannot drift apart;
- the `r2` bounds for those stores are read from `0x80433080` (the `r2` base
  documented in `engine::globals`), slot by slot; no value is invented.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80260950` | `fn_80260950` | 296 | medium |
| `0x80261640` | `fn_80261640` | 244 | medium |
| `0x80261E70` | `fn_80261e70` | 296 | medium |
| `0x80262540` | `fn_80262540` | 424 | medium |
| `0x802628A0` | `fn_802628a0` | 336 | medium |
| `0x80262E90` | `fn_80262e90` | 328 | medium |
| `0x80263B90` | `fn_80263b90` | 400 | medium |
| `0x80264400` | `fn_80264400` | 632 | high |
| `0x80264760` | `fn_80264760` | 492 | high |
| `0x80265100` | `fn_80265100` | 360 | medium |
| `0x80265780` | `fn_80265780` | 268 | medium |
| `0x80265890` | `fn_80265890` | 416 | medium |
| `0x80265A30` | `fn_80265a30` | 284 | medium |
| `0x80265FF0` | `fn_80265ff0` | 224 | high |
| `0x802660D0` | `fn_802660d0` | 340 | high |
| `0x80267E30` | `fn_80267e30` | 296 | medium |
| `0x80269F00` | `fn_80269f00` | 264 | medium |
| `0x8026A010` | `fn_8026a010` | 412 | medium |
| `0x8026A2A0` | `fn_8026a2a0` | 168 | medium |
| `0x8026A350` | `fn_8026a350` | 204 | medium |
| `0x8026A880` | `fn_8026a880` | 420 | medium |
| `0x8026ADD0` | `fn_8026add0` | 508 | medium |
| `0x8026B370` | `fn_8026b370` | 492 | medium |
| `0x8026B900` | `fn_8026b900` | 604 | medium |
| `0x8026DF50` | `fn_8026df50` | 176 | medium |

## Unknowns

1. **The `r2` clamp bounds.** `0x8026A880`, `0x8026ADD0`, `0x8026B370` and
   `0x8026B900` read two floats per parameter from `r2 - 0x47AC..-0x4738`.
   The bodies read the slots by address; their values, units and the
   "parameter" meaning of each of the five entries are not recovered.
2. **The `0x801A0xxx` channel bindings.** `fn_80260950` calls
   `0x801A0DF0`/`0x801A0E10`/`0x801A0E30` with `(this + 0xD8..+0xE0,
   this + 0xE4..+0xEC)` and `0x801A0CA0`/`0x801A0D10`/`0x801A0D80` with
   `(0, 0)`. The exports prove the argument lists only; those bodies remain
   untranslated in `audio::ax`.
3. **`0x802DCD60`/`0x802DCD70`/`0x802DCD74`.** The register-save thunks are
   modelled as extra leading arguments, following the convention the wave-36
   and wave-79 sections established. `fn_80261640` therefore takes
   `(this, slot)`, `fn_80269f00` takes `(this, free_flag)`, `fn_8026a010`
   takes `(this, base, extra)` and `fn_8026a2a0` takes
   `(this, size, first, second)`.
4. **Dropped `this` in two calls.** `fn_80260950` forwards the frame callback
   `0x80261190` as a bare address; `fn_80267E30`'s `FUN_802696b0` export drops
   its `this`, reconstructed as the block at `+0x1C` that `fn_8026db50`
   initialises on the next line.
5. **`fn_80262e90`'s uninitialised block.** The original leaves the stack
   block untouched for modes outside 1..3. The body zero-fills it so the host
   run is deterministic; this is noted at the store site.
6. **`fn_80264400`'s second flag.** The export assigns the literal `5` when
   `values[1] != 0` instead of OR-ing a bit. The body transcribes the
   assignment literally rather than repairing it.

## Deliberate skips

- **`0x80261190`, `0x80263D20`, `0x80263F50`, `0x80268910`, `0x80269740`,
  `0x80269B50`, `0x8026A420`** - these reach through `(**(code **)...)()`
  with a dropped receiver, an `undefined8` float pair (`extraout_f1`), or a
  16-argument callee (`0x80264950`) that the `call0..call5` helpers cannot
  express honestly.
- **`0x802605D0`, `0x80260950`'s callback, `0x80260AF0`, `0x80260FA0`,
  `0x802610C0`, `0x80261190`, `0x802612F0`, `0x80261740`, `0x80261BF0`,
  `0x802622A0`, `0x802629F0`, `0x802631F0`, `0x802638A0`, `0x802654F0`,
  `0x80266250`, `0x80266440`, `0x802666C0`, `0x802667F0`, `0x80266DC0`,
  `0x80267190`, `0x80267F40`** - the `CONCAT44(0x43300000, ...)` and GQR
  families. They need the double-precision constant materialisation and the
  `ldexpf` scale path that `src/platform.rs` does not model.
- **`0x8026B900`'s sibling `0x8026BF40` and the class F/G/H stores
  `0x8026C5F0`/`0x8026CD30`/`0x8026D3B0`** - the same clamp shape as the four
  bodies taken here, but each is 664-852 bytes with a further voice-set pair;
  left for a focused pass rather than transcribed at lower review quality.
- **`0x8026A420`, `0x8026A880`'s caller-side allocation, `0x80268790`** -
  teardown paths whose `0x802DCD74` pair carries both a count and a pointer
  and whose virtual slots are called with a receiver the export strips.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; all 25 addresses came from
  the untranslated part of the `0x80260000` bucket.
- The module header gained item 13 and its body count is 25.
- `node tools/check-lane-csv.mjs wave92` reports 25 rows for
  `wave92-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`; the remaining errors name other lanes' files.
