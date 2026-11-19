# Coverage - Wave 39 (`resource::registry`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 13 | 1064 | 10 | 3 |

Range covered: `0x8016A480..0x8016A9B0` — the stream/frame-heap wrapper family
that the mount stream base and the wave-38 heap layer drive. Total bytes:
1064.

## What this pass covers

- **Stream singleton (3 bodies):**
  `fn_8016a480` (constructs the EGG stream base over `0x802BA2D0` and
  publishes the singleton in `r13 - 0x5298`), `fn_8016a700` (allocates the
  `0x38`-byte object through `0x802B9240` and constructs the singleton; the
  `dSys` initialisation at `0x800E48D0` calls this with `0x80377C48`), and
  `fn_8016a5d0` (reads the singleton).
- **Heap-view attach/detach (6 bodies):**
  `fn_8016a5e0` / `fn_8016a740` (the two view constructors, both rooted in the
  LZ construction `0x802B8A70`, installing the `0x803296BC` / `0x803296A8`
  vtables), `fn_8016a630` / `fn_8016a780` (their deleting destructors),
  `fn_8016a690` (attaches an EGG heap at `+0x14` via the heap's `+0x10` slot),
  and `fn_8016a880` (detaches the heap back to the singleton via its `+0x1C`
  slot).
- **Frame-heap lifecycle (4 bodies):**
  `fn_8016a7f0` (creates a frame heap through [`fn_8016e630`] and attaches
  it), `fn_8016a960` (opens the frame heap, then records the old current
  heap with [`fn_8016e970`] and sets the view's heap current through
  `0x802B9170`), `fn_8016a8e0` (kind-tests the heap's `+0x0C` slot and
  adjusts through `0x8016E760`), and `fn_8016a9b0` (restores with
  [`fn_8016e980`] and adjusts). The shipped game callers (`0x80044460`,
  `0x80115D10`) pass `(view, base, heap, name, size, flags)` and always pair
  the open with the close.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8016A480` | `fn_8016a480` | 72 | high |
| `0x8016A5D0` | `fn_8016a5d0` | 8 | high |
| `0x8016A5E0` | `fn_8016a5e0` | 76 | high |
| `0x8016A630` | `fn_8016a630` | 88 | high |
| `0x8016A690` | `fn_8016a690` | 80 | high |
| `0x8016A700` | `fn_8016a700` | 56 | high |
| `0x8016A740` | `fn_8016a740` | 60 | high |
| `0x8016A780` | `fn_8016a780` | 108 | high |
| `0x8016A7F0` | `fn_8016a7f0` | 132 | medium |
| `0x8016A880` | `fn_8016a880` | 92 | medium |
| `0x8016A8E0` | `fn_8016a8e0` | 116 | high |
| `0x8016A960` | `fn_8016a960` | 76 | medium |
| `0x8016A9B0` | `fn_8016a9b0` | 100 | high |

## Deliberate skips and why

- **The mount island `0x8016B090..0x8016BF70` is already fully translated**
  (waves 15/24), so there was nothing left to extend there except the
  4-byte call thunks `0x8016B950`/`0x8016BCB0`, which stay below the size
  floor (their targets already live in this file). The stream base
  `0x802BA2B0..0x802BA800` (wave 30) is likewise complete; `0x802BA870` on
  is EGG root-heap / GX-object setup owned by other lanes.
- **The `0x8016A000..0x8016A3A0` AX/voice object family** (vtable
  `0x80329670`, calls into `0x80257xxx`/`0x8025axxx`) is audio, not the
  registry — untouched here.
- **GQR matrix helpers** (`0x8016EBF0`, `0x8016ED10`, `0x8016E3A0`,
  `0x8016EE30`, plus the `0x8016E230` geometry test) render as
  `ldexpf`/`unaff_GQR0` and are not transportable through the platform u32
  harness; skipped per brief, as wave 38 did.
- **The `0x8016C400..0x8016C7F0` layout-object manager** (list at
  `0x80377C3C`, pruning through `0x8016D120`) and the `0x8016Fxxx` input
  pad table belong to `ui::lyt` / input lanes.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path
  (`work/ghidra/main-gb/export/<file>.c`).
- Every row has exactly one body (`pub unsafe fn fn_<addr>`) in
  `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs` (only
  pre-existing warnings in `runtime/bt.rs`, `runtime/cpp.rs`,
  `runtime/input.rs` remain).
- `node tools/check-lane-csv.mjs wave39` reports no problem naming
  `ledger/agents/wave39-registry.csv`.
