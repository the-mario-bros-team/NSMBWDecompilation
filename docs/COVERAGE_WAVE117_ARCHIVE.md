# Coverage - Wave 117 (resource::archive, `0x802B0000` bucket close-out)

## How to read this file

Each row names one Ghidra function by its shipped-image address, the Rust body
that replaces it in `src/resource/archive.rs`, the exact size from
`work/ghidra/main-gb/export/functions.tsv`, and how confident the
reconstruction is. `address` is a virtual address in `sys/main.dol` (image base
`0x80004000`), not a file offset. The single body is appended after the
wave-112 section at the end of `src/resource/archive.rs`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 1 | 1844 | 0 | 1 |

The file now holds 549 translated bodies: 548 before this pass plus the 1
below. The assigned `0x802B0000` bucket held exactly one untranslated function
when this lane started, so the bucket is now closed (306 of 306).

## What this pass covers

1. **Four-face glyph-quad builder** - `0x802B0680` (1844 bytes). Walks the
   descriptor array at `faces + 0x00..0x1F` in the export's order `+0x04`,
   `+0x0C`, `+0x1C`, `+0x14`. Each descriptor is a pane-index byte followed by
   a texture-object pointer. When the top nibble of the face's `+0x3C` word is
   non-zero the body calls vtable slot `+0x0C` with `(face, mode, alpha)`,
   builds four UV corners from one 10-byte template of `0x8034F9B8`, enables
   the GX attribute table with `0x802B6B90(handle, 1)` and submits the strip
   through `0x802B71E0`. `mode` is the local [`fn_802b6ae0`] result for
   `(null, alpha)`, i.e. `0` only for the `0xFF` sentinel.

### Shared layout notes

- The 10-byte template is indexed by the face's pane byte. Its `+0x08`/`+0x09`
  bytes select the u and v lanes; the lane bytes then address the eight corner
  bytes at `+0x00..+0x07`. The four blocks differ only in which corner pair is
  the minimum and which is the maximum, so the eight-float corner array is
  always `[u0,v0,u1,v1,u2,v2,u3,v3]`.
- The export's `0x43300000` bias pairs are the compiler's `u32 -> f64`
  conversions and the `r2 - 0x3FD0`/`0x3FD8` doubles are the `2^52` and
  `2^52 + 2^31` biases. They are written as plain `as f32` casts, the same
  policy [`fn_802b4ae0`] documents.
- The six-register call to `0x802B71E0` is issued with `platform::call6` and
  the two position pairs, the corner array, the colour block and `alpha` as
  explicit arguments. This resolves the wave-105 skip: that pass could not map
  the registers onto the local [`fn_802b71e0`] stub, which only models the
  colour copy.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802B0680` | `fn_802b0680` | 1844 | medium |

## Unknowns

1. **The corner-template semantics.** `0x8034F9B8` is 10 bytes per record with
   two selector bytes and eight corner bytes; the export proves the indexing
   and the arithmetic, but not what each corner byte means (glyph atlas UVs vs.
   layout edges). The `// inferred:` comment in the constant doc records this.
2. **The face-object class.** Each face is a RLYT-style record with a vtable at
   `+0x00`, a `+0x3C` packed count word and a `+0x40` section base, because the
   body calls [`fn_802b4ae0`] on it. The export does not name the class.
3. **The `alpha`/`mode` split.** `alpha` (`r7`) is both the colour scale and
   the input of `0x802B6AE0`, which returns `0` only for `0xFF`. The export
   shows the use, not the intended meaning of the sentinel.
4. **The lane values.** The selector bytes are used as indexes into an
   eight-float block and are expected to be `0`/`1`; any other value would step
   outside it, exactly as in the export.

## Deliberate skips

None. The one function left in the bucket was reconstructible once the
six-register draw call was issued by address with explicit arguments; see
`docs/COVERAGE_WAVE105_ARCHIVE.md` for the earlier, now-resolved skip note.

## Verification

- The address and byte size match `work/ghidra/main-gb/export/functions.tsv`
  (read from the TSV, not from prose): `802b0680` is 1844 bytes.
- The evidence file exists at
  `work/ghidra/main-gb/export/802b0680_FUN_802b0680.c`.
- The row has exactly one `pub unsafe fn fn_802b0680` body in
  `src/resource/archive.rs` with a complete `# Original` / `# Arguments` /
  `# Returns` / `# Side effects` / `# Safety` / `# Confidence` doc block.
- No address was already in `ledger/functions.csv`; `0x802B0680` was the last
  open address in the `0x802B0000` bucket.
- The module header gained item 20, item 18's stale skip sentence now points at
  it, and the file total is 549.
- `node tools/check-lane-csv.mjs wave117` reports 1 row for
  `wave117-archive.csv` and `OK`.
- `cargo check --message-format=short` reports no error for
  `src/resource/archive.rs`.
