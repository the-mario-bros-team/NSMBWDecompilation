# Coverage - Wave 96 (GX / render band, `0x802A0000` bucket)

## How to read this file

This page covers the Wave 96 pass over the `0x802A0000..0x802AFFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). It lists exactly what was added,
the evidence for each body, and what was deliberately left alone. Addresses are
virtual addresses of the unrelocated `main.dol` (image base `0x80004000`); byte
counts come from `work/ghidra/main-gb/export/functions.tsv`, never from prose.
The module already held 202 translated entries from waves 1-95.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 27 | 5480 | 12 | 15 |

## What this pass covers

- **Empty hooks** (`0x802A0650`, `0x802A35B0`, `0x802A7C90`, `0x802A8010`).
  Four entries are a bare `blr`. `0x802A8010` is still called with stack
  buffers by `0x802A7CA0`, so the empty body is the real behaviour, not a
  missing translation.
- **Field accessors** (`0x802A0930`, `0x802A0940`, `0x802A3120`,
  `0x802A3130`, `0x802A7FF0`, `0x802A8000`, `0x802AF070`). Six single
  loads/masks plus the unscaled table index at `0x802A0940`. The
  `0x802A3130`/`0x802A8000` pair is the same `u8[+0xB2] & 0xC0` alignment
  query at two addresses.
- **Matrix packers** (`0x802A0E00`, `0x802A0E50`). The twelve-float store and
  the two-mode 3x4 composition. The five scalars arrive in `f1..f5` (Ghidra
  prints them as `double`), the destination in `r3`, the mode in `r4` and the
  nine source floats in `r5`; the call shape is confirmed by `0x802AF240`.
- **Reciprocal refinement** (`0x802A3140`). One Newton step around a
  single-precision reciprocal, which the text emitters use to turn
  `count - 1` into `1 / (count - 1)`.
- **Display-list footer** (`0x802AB4F0`). Clears the word at `r13-0x5B88`
  (`0x80429AF8`) through `0x801AB1C0` and returns the constant `0x00070007`.
- **Scene list/tag helpers** (`0x802AB680`, `0x802AC8B0`, `0x802ACA70`). The
  list destructor with its `+0xD2` ownership byte, the three-level tag
  propagation keyed on the address `0x8042ADD8`, and the `0x14`-byte name
  record scan that can follow the `+0x0C` parent chain.
- **Name-record builder** (`0x802AC550`). Vtable `0x8034F868`, a self-linked
  header at `+0x10`, and one `0x0C`-byte node per non-zero virtual `+0x3C`
  result.
- **`txt1` constructor** (`0x802ADEA0`). Vtable `0x8034F930`, the five-argument
  delegate at `+0xDC`, the `+0xD8` ring, the `+0x100` modulo-3 re-encoding, the
  optional `0x8022C300` font helper and the `0x5C`-byte material at `+0x28`.
- **`wnd1` constructor** (`0x802AF6F0`). Vtable `0x8034F9F4`, the four-slot
  delegate at `+0xE8`, the `+0xF8` slot array, and the `+0x100`/`+0x104`
  child array with one `0x5C`-byte material per child.
- **Ring buffer pair** (`0x802AEDA0`, `0x802AEFA0`). The `u16` ring at `+0xD8`
  with capacity at `+0xFC` and cursor at `+0xFE`: resize (release through
  virtual `+0x78`, allocate, zero) and append (clamp to capacity, `memcpy`
  through `0x80004364`, write the terminator).
- **Glyph attach** (`0x802AF080`). Detach path (virtual `+0x08`, free through
  `0x801D5550`, clear `+0x101` bit 7) and the cached extents at
  `+0xE8`/`+0xEC`.
- **Rectangle helpers** (`0x802AF240`, `0x802AF4F0`, `0x802AFF60`). The
  rectangle build (`0x8022D2F0` + bounds + `0x802307E0` + the
  `0x802AC340` anchors), the per-segment min/max accumulator, and the window
  quad submission that passes two overlapping halves of its buffer to
  `0x802B71E0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802A0650` | `fn_802a0650` | 4 | high |
| `0x802A0930` | `fn_802a0930` | 8 | high |
| `0x802A0940` | `fn_802a0940` | 12 | high |
| `0x802A0E00` | `fn_802a0e00` | 68 | high |
| `0x802A0E50` | `fn_802a0e50` | 504 | medium |
| `0x802A3120` | `fn_802a3120` | 8 | high |
| `0x802A3130` | `fn_802a3130` | 12 | high |
| `0x802A3140` | `fn_802a3140` | 24 | medium |
| `0x802A35B0` | `fn_802a35b0` | 4 | high |
| `0x802A7C90` | `fn_802a7c90` | 4 | high |
| `0x802A7FF0` | `fn_802a7ff0` | 12 | high |
| `0x802A8000` | `fn_802a8000` | 12 | high |
| `0x802A8010` | `fn_802a8010` | 4 | high |
| `0x802AB4F0` | `fn_802ab4f0` | 84 | medium |
| `0x802AB680` | `fn_802ab680` | 316 | medium |
| `0x802AC550` | `fn_802ac550` | 276 | medium |
| `0x802AC8B0` | `fn_802ac8b0` | 428 | medium |
| `0x802ACA70` | `fn_802aca70` | 200 | medium |
| `0x802ADEA0` | `fn_802adea0` | 748 | medium |
| `0x802AEDA0` | `fn_802aeda0` | 200 | medium |
| `0x802AEFA0` | `fn_802aefa0` | 196 | medium |
| `0x802AF070` | `fn_802af070` | 8 | high |
| `0x802AF080` | `fn_802af080` | 292 | medium |
| `0x802AF240` | `fn_802af240` | 676 | medium |
| `0x802AF4F0` | `fn_802af4f0` | 468 | medium |
| `0x802AF6F0` | `fn_802af6f0` | 644 | medium |
| `0x802AFF60` | `fn_802aff60` | 268 | medium |

## Deliberate skips

The bucket still contains 32 untranslated entries. Each was read in full before
being rejected; none is a bookkeeping omission.

- **Paired-single vector arithmetic** (`0x802A08A0`, `0x802A0910`,
  `0x802A09F0`, `0x802A0A40`, `0x802A6F80`, `0x802A8930`, `0x802A8960`).
  These are 2- or 3-float adds/subtracts/dot products whose every operand goes
  through a `psq_l`/`psq_st` arm that branches on the live GQR0 value. A Rust
  body would have to assume a quantizer type that the C does not record. This
  is the same reason the module has skipped its GQR entries since wave 1.
- **GX vertex emitters that mix GQR arms with live vector math**
  (`0x802A1090`, `0x802A3160`, `0x802A35C0`, `0x802A3A20`, `0x802A3E00`,
  `0x802A40C0`, `0x802A4400`, `0x802A47C0`, `0x802A5810`, `0x802A64B0`,
  `0x802A6850`, `0x802A6CB0`, `0x802A6FB0`, `0x802A71D0`, `0x802A7CA0`,
  `0x802A8020`, `0x802A8440`, `0x802A8A10`, `0x802A8D10`, `0x802A90C0`,
  `0x802A9640`, `0x802AA830`, `0x802AE9C0`). In these the `ldexpf` blocks are
  inside the arithmetic path (not just the prologue), and several also split a
  `FUN_802DCDxx` save pair whose second half is dereferenced as a float array.
  The vertex paths need a raw `psq` pass first.
- **`0x802A0660`** (464 bytes). The billboard/text copy reads `local_2c[0]` as
  an integer immediately after `0x801C9900` fills that buffer with floats, and
  one arm reads `local_38`/`local_34`/`local_30` before any visible store. The
  branch selector is a decompiler artefact, so a faithful body cannot be
  proven from this export.
- **`0x802A2DE0`** (820 bytes). The text-run emitter uses the GQR arms for its
  float-to-fixed conversions; only the loop skeleton is certain, and a body
  without the conversion would misrepresent the draw call.

## Verification

- Every address and byte size matches `functions.tsv` (27 rows, 5,480 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 229
  translated entries and this pass added 27.
- `node tools/check-lane-csv.mjs wave96` prints `OK` (27 rows for this lane,
  no duplicate or malformed row).
- `cargo check --message-format=short` reports no error or warning in
  `src/render/gx_band.rs`. The only remaining workspace error at the time of
  writing is in `src/runtime/os.rs`, owned by another Wave 96 lane.
