# Coverage - Wave 105 (geometry view transform and rail polyline)

## How to read this file

This is the Wave 105 record for the remaining reconstructible entries of the
`0x80000000` bucket in [`src/render/geometry.rs`](../src/render/geometry.rs).
It answers four questions for a reader with no context:

1. **What was added?** Eight `pub unsafe fn` bodies: one view-transform writer,
   its wrapper, one quad-matrix builder, one lazy-init quad driver, two
   20-byte virtual-slot forwarders and the rail tessellator plus its wrapper.
2. **Where is it?** Only in [`src/render/geometry.rs`](../src/render/geometry.rs).
   No other Rust file, `lib.rs`, `mod.rs`, `Cargo.toml` or `platform.rs` was
   edited.
3. **How sure is it?** `high` means the raw instruction stream and the Ghidra C
   agree on every store and call. `medium` means the control flow is literal
   but at least one callee (sine/cosine pair, tessellator, float-to-int
   converter) is another lane's function, so its role is marked
   `// inferred:`.
4. **What is still missing?** `0x80007DE0`, `0x8000C5C0`, `0x8000CA50` and
   `0x8000E060` are deliberately skipped (see "Deliberate skips").

Addresses are plain lowercase hex **virtual** addresses (image base
`0x80004000`), not file offsets. Evidence paths are relative to the repository
root and live under `work/ghidra/main-gb/export/`. Raw instruction checks used
`work/dol/main.elf`.

Terms: **GX** is the Wii GPU; **SDA** is the `r13` small-data area
(`0x8042F680`); **r2** is the second small-data base (`0x80433080`); **GQR** is
a paired-single quantisation register; **FIFO** is the GX write-gather stream at
`0xCC008000`. See `docs/GLOSSARY.md`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/geometry.rs` | `render::geometry` | 8 | 1,848 | 5 | 3 |

The bucket started this pass with 12 untranslated entries out of 92. Eight are
landed here; four remain and are listed under "Deliberate skips".

## What this pass covers

- **View transform** - `fn_8000d740` converts the biased 16-bit angle through
  the image `1/256` scale, calls the two table evaluators at `0x80237C40` and
  `0x80237BD0`, and writes the six words at `+0xD0C..+0xD28`.
  `fn_8000d820` forwards `f1`/`r4` to it and then drives the rail builder.
- **Quad matrix** - `fn_8000e910` runs the fixed GX preamble, lazily initialises
  the `0x80351690` fill block and the `r13-0x5AB0` pair, fills and submits one
  draw descriptor, then composes the 48-byte matrix from the global render
  object's `u16` fields and the caller's two scale/offset pairs.
- **Quad driver** - `fn_8000edf0` lazily initialises `r13-0x5AA0` to `1.0` and
  `r13-0x5A90` to `0.0`, then calls `fn_8000e910`, the gx-lane corner emitter
  `fn_8000ebf0` and the flush forwarder `fn_8000edd0`.
- **Virtual-slot forwarders** - `fn_8000edd0` dispatches through
  `[r13-0x47B0]+0x38` then slot `+0x0C`; `fn_8000f2e0` dispatches through
  `[this+0x24]` then slot `+0x18`. Both are five raw instructions.
- **Rail polyline** - `fn_8000f950` tessellates the curve through
  `begin`/`begin+0x18` into `this+0xC38`, accumulates 2D segment lengths into
  `this+0xC80`, and emits `{f32 x, f32 y, u16 height}` records. `fn_8000fba0`
  wraps it and hands the result to `crate::game::path::fn_8000df60`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8000D740` | `fn_8000d740` | 160 | medium |
| `0x8000D820` | `fn_8000d820` | 72 | high |
| `0x8000E910` | `fn_8000e910` | 732 | medium |
| `0x8000EDD0` | `fn_8000edd0` | 20 | high |
| `0x8000EDF0` | `fn_8000edf0` | 192 | high |
| `0x8000F2E0` | `fn_8000f2e0` | 20 | high |
| `0x8000F950` | `fn_8000f950` | 580 | medium |
| `0x8000FBA0` | `fn_8000fba0` | 72 | high |

All eight also have a row in
[`ledger/agents/wave105-geometry.csv`](../ledger/agents/wave105-geometry.csv).
The address range is `0x8000D740..0x8000FBA0` (not contiguous; the four skipped
entries sit inside it).

## Deliberate skips

These four are in the bucket but were not translated. Each has a Ghidra export,
but the export loses unreachable blocks *inside* its GX FIFO loop, so a C-level
translation would silently drop visible writes:

| address | bytes | why skipped |
|---|---:|---|
| `0x80007DE0` | 1,632 | GQR/paired-single effect setup; the export replaces the arithmetic with `ldexpf`/`unaff_GQR0` and drops blocks. |
| `0x8000C5C0` | 1,168 | The raw loop emits four vertices per `GXBegin(0x80,0,4)`; the export shows one `CONCAT22` FIFO word per iteration. |
| `0x8000CA50` | 1,444 | Same quad-emission shape as `0x8000C5C0`, four vertices lost in the export. |
| `0x8000E060` | 2,016 | Large GX effect loop with the same export damage; needs a full raw FIFO decode. |

Translating any of them from the current Ghidra C would invent behavior. They
are left for a raw-instruction GX pass; the bodies above already document the
calling conventions that pass will need.

## Uncertainties and limitations

1. **`fn_8000d740` evaluator identity.** `0x80237C40` and `0x80237BD0` are two
   adjacent 256-entry table evaluators called with the same phase. The pair is
   `// inferred:` to be sine/cosine from the shared argument and the
   matrix-shaped output, but neither address has a symbol.
2. **The angle unit.** The original stores the biased angle with `sth` and reads
   it back through `psq_l` with GQR0 in unsigned-16-bit mode, so the Rust uses
   `u16`; Ghidra declares the parameter `short` because it only sees the store.
   The pre-multiplier is the image constant `1/256` at `0x8042B0E0`.
3. **`fn_8000e910`/`fn_8000f950` callees.** `0x802D0E50`/`0x802D0F40` (draw
   descriptor), `0x801C0C60`/`0x801C0C10` (matrix compose) and `0x800E2F10`
   (curve tessellator) are owned by other lanes; their roles are inferred from
   the argument registers, so both functions are `medium`.
4. **`fn_8000f950` closing record.** The raw code decrements `end` by one
   8-byte record and enters both emission loops as do/while loops, so the loop
   stops before the final point and the single closing record at `0xFB18`
   emits it. The Rust keeps that literal pointer and loop shape; it is not a
   simplification.
5. **`fn_8000edf0`'s `f1`.** The wrapper never writes `f1`; it forwards the
   caller's value to `fn_8000e910` as the `z` translation. Ghidra prints a
   two-argument prototype because it does not model the pass-through.
6. **No runtime validation.** `cargo check` proves the module type-checks; it
   does not execute the PowerPC code.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (160, 72, 732, 20, 192, 20, 580 and 72 bytes).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/render/geometry.rs`.
- No address was already in `ledger/functions.csv`; the bucket's 12 open
  entries were re-filtered before writing.
- `cargo check --message-format=short` reports no errors or warnings in
  `src/render/geometry.rs`.
- `node tools/check-lane-csv.mjs wave105` reports `OK` for this lane's CSV.
