# Coverage - Wave 95 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 95 pass over the first 64 KiB bucket of the
GX/render lane (`0x80280000..0x8028FFFF`). It lists what was added, the exact
evidence for each body, and what was deliberately left alone. Addresses are
virtual addresses of the unrelocated `main.dol` (image base `0x80004000`); byte
counts come from `work/ghidra/main-gb/export/functions.tsv`, never from prose.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 28 | 5008 | 13 | 15 |

## What this pass covers

- **Draw-queue completion** (`0x802802D0`). The task-removal path that closes
  cluster A: the `queue+0x24` current-task compare, the three-list unlink via
  `0x802290C0`, and the two task virtual slots (`+0x14`, `+0x10`).
- **Texture grid** (`0x80281EF0`, `0x80282FE0`). The release callback stored at
  `object+0x94` (per-cell handle release plus optional `fn_802810a0`), and the
  `mulli r3,r4,0x104` chunk-stride helper that matches the `0x104`-byte scene
  object from cluster B.
- **Clamp/index helpers** (`0x80283760`, `0x80283940`, `0x802839D0`). Three
  small "clamp, then index a fixed table" routines: a `[-90.4, 6.0]` float ramp,
  a five-field ten-byte record, and a `0..=0x7F` record selector.
- **Fixed-point filter** (`0x80285660`). Three-tap accumulation plus
  `(x + 1) >> 1` rounding and the 16-bit clamp; the two history words shift
  after every call.
- **Node factories and list walkers** (`0x802858E0`, `0x80285B50`,
  `0x80286350`, `0x802864A0`, `0x80286B70`, `0x802874E0`, `0x802877C0`,
  `0x802891C0`, `0x802892D0`, `0x80289320`). These use the `+0x1C` embedded
  vtable convention already established by `call_state_node`, plus one new
  four-argument helper (`call_state_node3`) for the create call.
- **Unit-box classifiers** (`0x8028E430`, `0x8028E510`, `0x8028E5E0`,
  `0x8028E6F0`, `0x8028E7E0`). Five small predicates over a three-float vector,
  all reading the `sdata2` limits `-1.0` (`0x8042EC20`) and `1.0`
  (`0x8042EC24`).
- **One-store accessors** (`0x80283BC0`, `0x80284B00`, `0x80284B10`,
  `0x802862A0`, `0x802862B0`, `0x80286630`). Six 8-12 byte functions that are
  real single loads/stores, not veneers. The 16-byte heuristic is waived for
  them for the same reason as the earlier `0x8029F8F0` accessor.

## Functions

| Address | Function | Bytes | Confidence |
|---|---|---:|---|
| `0x802802D0` | `fn_802802d0` | 240 | medium |
| `0x80281EF0` | `fn_80281ef0` | 332 | medium |
| `0x80282FE0` | `fn_80282fe0` | 8 | high |
| `0x80283760` | `fn_80283760` | 88 | medium |
| `0x80283940` | `fn_80283940` | 132 | medium |
| `0x802839D0` | `fn_802839d0` | 84 | high |
| `0x80283BC0` | `fn_80283bc0` | 8 | high |
| `0x80284B00` | `fn_80284b00` | 8 | high |
| `0x80284B10` | `fn_80284b10` | 8 | high |
| `0x80285660` | `fn_80285660` | 140 | high |
| `0x802858E0` | `fn_802858e0` | 308 | medium |
| `0x80285B50` | `fn_80285b50` | 244 | medium |
| `0x802862A0` | `fn_802862a0` | 8 | high |
| `0x802862B0` | `fn_802862b0` | 8 | high |
| `0x80286350` | `fn_80286350` | 216 | medium |
| `0x802864A0` | `fn_802864a0` | 244 | medium |
| `0x80286630` | `fn_80286630` | 12 | high |
| `0x80286B70` | `fn_80286b70` | 160 | medium |
| `0x802874E0` | `fn_802874e0` | 736 | medium |
| `0x802877C0` | `fn_802877c0` | 368 | medium |
| `0x802891C0` | `fn_802891c0` | 272 | high |
| `0x802892D0` | `fn_802892d0` | 72 | high |
| `0x80289320` | `fn_80289320` | 228 | medium |
| `0x8028E430` | `fn_8028e430` | 224 | high |
| `0x8028E510` | `fn_8028e510` | 200 | high |
| `0x8028E5E0` | `fn_8028e5e0` | 260 | medium |
| `0x8028E6F0` | `fn_8028e6f0` | 232 | medium |
| `0x8028E7E0` | `fn_8028e7e0` | 168 | medium |

## Deliberate skips

- **Every `G` entry of the bucket (23 addresses).** `0x80280910`, `0x80280BA0`,
  `0x80281C90`, `0x802821E0`, `0x802850F0`, `0x80286CA0`, `0x80287C10`,
  `0x80288390`, `0x80288490`, `0x80288E30`, `0x80289410`, `0x8028A6F0`,
  `0x8028B6C0`, `0x8028C7A0`, `0x8028CEE0`, `0x8028DC70`, `0x8028DFC0`,
  `0x8028E310`, `0x8028E890`, `0x8028ED30`, `0x8028F2D0`, `0x8028F860`,
  `0x8028FEA0`. Their Ghidra C contains `ldexpf(...)`/`unaff_GQR0`; a faithful
  body needs a GQR-aware reconstruction, which this pass does not have.
- **`0x80282040`** (416 bytes). Parametric curve evaluator with ten `sdata2`
  constants and interleaved `double`/`float` casts. The constants are readable,
  but the intermediate rounding cannot be reproduced honestly from the C alone,
  so it is left for a raw-instruction pass.
- **`0x80283420`** (264 bytes). The list insert/sort path takes two live
  pointers in `r3`/`r4`, but Ghidra declares no parameters and reads the second
  one out of the register-save stub's `r4`. The role of that pointer is not
  provable from the export, so no body was written.
- **`0x802837C0` and `0x802838A0`** (224 and 148 bytes). Both index a table
  through `PTR_DAT_8034f288[word[object]]` and then re-clamp against `[0, 2]`,
  which does not fit the `[-1, 1]` convention of their neighbours. The table
  role and the extra clamp are unresolved.
- **`0x80283EC0`** (160 bytes). The virtual call at `*piVar1+0x20` is printed
  with no arguments although the caller receives seven; forwarding the exact
  register set would be a guess.
- **`0x802830A0`** (444 bytes). Pool user that first drains the current grid and
  then re-allocates through `fn_80280e60`; its `r4` slot is read out of the
  save stub and its `+0x10/+0x14` link words are not established, so the two
  entry states cannot be named honestly.
- **`0x802835B0`** (696 bytes). A `pow(1.018, 0x3FF - x)` style float helper
  with two 8-way unrolled multiply chains and a `double`/`float` mix that the C
  renders ambiguously; a body would not be verifiable without raw decoding.
- **`0x80283BD0`** (460 bytes), **`0x80284700`** (412 bytes), **`0x80284F30`**
  (440 bytes), **`0x80286D80`** (384 bytes), **`0x80287930`** (732 bytes),
  **`0x802888E0`** (436 bytes). These depend on multi-level descriptor walks or
  on lock/handle APIs (`0x8027B5F0`, `0x80268A50`, `0x802698xx`) whose roles and
  argument counts are not yet established; a body would be a transcription
  without meaning.
- **`0x80284040` and `0x80284050`** (8 bytes each). Pure tail-call forwarders to
  `0x80284B00`/`0x80284B10`; translating them would only duplicate the callee.
  The other sub-16-byte entries of the bucket are the shared-`blr` helpers
  already classified as `S` in `docs/COVERAGE_GX_BAND.md`. The six accessors
  listed above were admitted because each has a real load/store body.

## Verification

- Every address and byte size matches `functions.tsv` (28 rows, 5,008 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn` body in `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 202
  translated entries and this pass added 28.
- `node tools/check-lane-csv.mjs wave95` prints `OK` for
  `ledger/agents/wave95-gx-band.csv`.
- `cargo check` is free of errors for `src/render/gx_band.rs`; the file was also
  compiled in isolation as a `no_std` crate with `platform.rs` to confirm the
  new helpers and constants stand on their own. (The workspace build was
  blocked by an in-progress syntax error in another lane's file, which this
  lane does not touch.)
