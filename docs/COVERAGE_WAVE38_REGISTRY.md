# Coverage - Wave 38 (`resource::registry`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 14 | 1044 | 10 | 4 |

Range covered: `0x8016E500..0x8016EB50` (mount / archive heap layer).
Total bytes: 1044.

## What the 0x80160000 band actually contains

The band `0x80160000..0x8016FFFF` holds **421 functions** in `functions.tsv`
(352 still untranslated at the start of this wave). Sampling 30+ exports
across the band shows it is **not** a single subsystem; it is at least five
lanes' worth of code interleaved:

1. **`0x8016B090..0x8016BF70` — the genuinely registry-owned island (wave 15/24,
   already in this file).** Mount-command queue + mount-archive table +
   threaded stream manager/executor + DVD path probe. This is the only slice
   of the band that is unambiguously the archive registry, and it was already
   translated in earlier registry waves.
2. **`0x80161AD0..0x80163xxx` — actor/object base machinery.** `0x801628C0`
   is the shared allocator every `game::bases`/`game::enemies` factory calls
   (`ledger/functions.csv` shows 40+ actor factories citing it), and
   `0x80161AD0..0x80163180` is its vtable/list base. **Not** the registry —
   deliberately left out of this wave (see §3).
3. **`0x80164xxx..0x8016Axxx` — audio/stream.** Voice and sound objects
   (the `+0x38`-stride object family whose slot getters call
   `802B8A70/802B8AD0`). A different lane's work, untouched here.
4. **`0x8016Cxxx..0x8016Exxx` — GX + heap internals, jointly owned by
   `ui::lyt` in the merged ledger** (`0x8016C730..0x8016Dxx`, `0x8016E4B0`,
   `0x8016E610`, `0x8016E760`, `0x8016E7D0`, `0x8016E8F0`, `0x8016ECB0`,
   `0x8016EF50` — 27 rows). The exceptions are the mount heap publishers
   taken by this wave (`0x8016E970..0x8016EB50`).
5. **`0x8016Fxxx` — input pad table.** The 4-slot table at `0x80377D08`
   (`0x8016F500` clears slot `i`, `0x8016F220` computes rumble from the pad
   floats). Engine/input lane, **not** the registry — untouched here.

So the practical consequence: a *new* registry pass in this band must
concentrate on the heap plumbing that the already-translated mount code
calls but never translated — the `0x8016E500..0x8016EB50` mount/archive heap
layer, which is exactly the callees the `0x8016Bxxx` bodies use
(`HEAP_*` consts `0x8016E740`/`0x8016E760`/`0x8016E980`/`0x8016E9B0`, and the
`G_MOUNT_DVD_HEAP`/`G_GAME_HEAP_DEFAULT` globals).

## What this pass covers

- **Frame-heap lifecycle (7 bodies):**
  `fn_8016e970` (save current heap), `fn_8016e980` (restore + clear),
  `fn_8016e9b0` (`0x8016E9B0` create-to-current wrapper, used by
  `res_info_set_res`), `fn_8016e740` (frame-heap destroy through vtable
  `+0x1C`), `fn_8016e500`/`fn_8016e630`/`fn_8016e7f0` (allocate-inside-current-
  EGG-heap constructors; `E630` is the create the frame-heap wrapper calls).
- **Game-heap table and wrappers (4 bodies):**
  `fn_8016e9f0` (create game heap 1/2 into `0x80377C48` via `0x8016E8F0`,
  honouring the `r13 - 0x61F0` current-index byte), `fn_8016eaa0` /
  `fn_8016eab0` (index-1 / index-2 shims).
- **Named mount-heap slots (3 bodies):**
  `fn_8016eac0` / `fn_8016eaf0` / `fn_8016eb20` (publish the
  `0x80329B58/90/C0` heaps into `G_HEAP_SLOT_5234/5230/522C`),
  `fn_8016eb50` (create the mount DVD heap through
  `0x802BA2B0`/`0x802BA000` and publish it into `G_MOUNT_DVD_HEAP`, the
  heap `mount_stream_manager_init` installs on the mount stream).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8016E500` | `fn_8016e500` | 268 | medium |
| `0x8016E630` | `fn_8016e630` | 268 | medium |
| `0x8016E740` | `fn_8016e740` | 24 | high |
| `0x8016E7F0` | `fn_8016e7f0` | 240 | medium |
| `0x8016E970` | `fn_8016e970` | 12 | high |
| `0x8016E980` | `fn_8016e980` | 44 | high |
| `0x8016E9B0` | `fn_8016e9b0` | 64 | medium |
| `0x8016E9F0` | `fn_8016e9f0` | 172 | high |
| `0x8016EAA0` | `fn_8016eaa0` | 16 | high |
| `0x8016EAB0` | `fn_8016eab0` | 16 | high |
| `0x8016EAC0` | `fn_8016eac0` | 44 | high |
| `0x8016EAF0` | `fn_8016eaf0` | 44 | high |
| `0x8016EB20` | `fn_8016eb20` | 44 | high |
| `0x8016EB50` | `fn_8016eb50` | 80 | high |

## Deliberate skips (from the same band) and why

- **Paired-single / `ldexpf`/`unaff_GQR` exports** are skipped per brief:
  `0x8016E230` (4x4 point test) is fine but is geometry, not registry;
  `0x8016E3A0`, `0x8016EBF0`, `0x8016ED10`, `0x8016EE30` are GQR matrix
  helpers; `0x8016EBA0` passes twelve floats in fpr/stack registers that the
  `platform` u32-call harness cannot transport. All four remain untranslated.
- **Already owned by `ui::lyt` in the merged ledger:** `0x8016E4B0` (flag
  mask — called by this pass's bodies), `0x8016E610`/`0x8016E7D0`
  (alignment cursors — also called here), `0x8016E760` (frame-heap adjust),
  `0x8016E8F0` (EGG heap factory — `fn_8016e9f0`/`fn_8016eac0`/`fn_8016eaf0`/
  `fn_8016eb20` call it). This pass calls them by their translated names
  instead of re-typing bodies.
- **Not registry:** the `0x80161AD0..0x80163180` actor base (leaves for a
  `game::bases` lane — `0x801628C0` is its allocator), the `0x8016CXX..0x8016DXX`
  GX command list (prunes through `0x8016D120`), the 4-slot input pad table
  (`0x8016Fxxx`), the audio/stream objects at `0x80164xxx..0x8016Axxx`, and
  the 4-byte call thunks `0x8016B950`/`0x8016BCB0` (below the file's size
  floor: the target bodies already live here as `mount_call_cmd_create` /
  `mount_probe_path`).

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path
  (`work/ghidra/main-gb/export/<file>.c`).
- Every row has exactly one body (`pub unsafe fn fn_<addr>`) in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs` (only
  pre-existing warnings in `runtime/bt.rs`, `runtime/cpp.rs`,
  `runtime/input.rs` remain).
- `node tools/check-lane-csv.mjs wave38` reports no problem naming
  `ledger/agents/wave38-registry.csv`.
