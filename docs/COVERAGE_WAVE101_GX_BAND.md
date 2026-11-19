# Coverage - Wave 101 (GX / render band, `0x802A0000` bucket)

## How to read this file

This page covers the Wave 101 pass over the `0x802A0000..0x802AFFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses
of the unrelocated `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose.

`status` is `translated` for every row; `confidence` follows the body's
`# Confidence` line. Every row cites the exact export file it was read from.
The bucket held 102 entries and 70 were already translated; this pass adds 25
of the remaining 32, leaving 7 explicit skips at the end.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 25 | 17,760 | 1 | 24 |

## What this pass covers

- **Float-vector kernels** (`0x802A08A0`, `0x802A0910`, `0x802A09F0`,
  `0x802A6F80`, `0x802A8930`, `0x802A8960`). The NW4R `VEC3` normalise, dot,
  subtract and add helpers. Their `psq_l`/`psq_st` operands are `float *`
  records; the same callers read and write neighbouring components with plain
  `lfs`/`stfs` (`fn_802a35c0` stores `+0xCC..+0xD4` with `stfs`), so only the
  type-0/scale-1 arm of the GQR ladder can be live. The same test was applied
  to `fn_8029f640` in wave 98.
- **Ramp builders** (`0x802A47C0`, `0x802A9640`). Both build the transient
  `+0xD4` sine ramp from `u8[desc+0x14C]` with the angle
  `0.7111111 * (360/count) * i` and the two `0x80237CA0` outputs (endpoints
  `[0,1]` in `0x802A47C0`, `[1,0]` in `0x802A9640`), then dispatch mode
  `1`/`2`/other with a stage select, an optional two-vertex `GXBegin(0x98,0,8)`
  and two draw calls.
- **Range emitters** (`0x802A2DE0`, `0x802A3E00`, `0x802A40C0`, `0x802A7CA0`,
  `0x802A8020`, `0x802A8440`, `0x802A8A10`, `0x802A8D10`). The child-range
  draws. They share the `0x802A3120` count, the `0x802A8000` format word
  (`0x40` forces `1.0f`/`0.5f` steps), the `0x802A7FF0` edge flag and the
  `0x802A3140` step converter. The rings rotate three or five `0x1C`/`0x28`
  byte records and blend them through `0x802A6CB0`/`0x802A6FB0`/`0x802A71D0`;
  `0x802A8440` additionally builds the first two-point strip from
  `ctx+0xA4 - node+0xAC`.
- **Billboard record pair** (`0x802A35C0`, `0x802A6850`). Two identical bodies
  with different `sdata2` windows: the callback reference vector is multiplied
  by `0x801C1530`, normalised through `0x80237BA0`, composed with the node's
  `+0x30..+0x3C` scales, rotated by `0x802961E0` and packed by `0x801C0540`
  into the ten-float node record.
- **Segment blends** (`0x802A6CB0`, `0x802A6FB0`). The quadratic and fixed
  `0.5/0/0.5` blends of the `+0x14` column fields into the GX FIFO.
- **Matrix transform pair** (`0x802A5810`, `0x802AA830`). The mode-selected
  `4x3` column (`8`, `0x10`, `0x18`, other) or the two `0x80238420` table
  compositions, followed by the `0x80237BA0` normalise with the
  `+0x98/+0x9C/+0xA0` fallback.
- **Scalar helpers** (`0x802A0660`, `0x802A0A40`, `0x802AE9C0`). The
  `0x30`-byte block copy with the `+0x88` last-row offset, the guarded
  `sqrt(v)`, and the text-run bounds walk with the `0x8022F910`/`0x802DCBFC`
  state machine.

All eighteen `unaff_GQR0` bracketing blocks in the range-draw exports are dead
entry/exit register spills; their `ldexpf` results are never read. The
remaining live-`psq` functions are listed under "Deliberate skips".

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802A0660` | `fn_802a0660` | 464 | medium |
| `0x802A08A0` | `fn_802a08a0` | 112 | medium |
| `0x802A0910` | `fn_802a0910` | 32 | medium |
| `0x802A09F0` | `fn_802a09f0` | 76 | medium |
| `0x802A0A40` | `fn_802a0a40` | 80 | high |
| `0x802A2DE0` | `fn_802a2de0` | 820 | medium |
| `0x802A35C0` | `fn_802a35c0` | 1108 | medium |
| `0x802A3E00` | `fn_802a3e00` | 696 | medium |
| `0x802A40C0` | `fn_802a40c0` | 828 | medium |
| `0x802A47C0` | `fn_802a47c0` | 1964 | medium |
| `0x802A5810` | `fn_802a5810` | 684 | medium |
| `0x802A6850` | `fn_802a6850` | 1108 | medium |
| `0x802A6CB0` | `fn_802a6cb0` | 716 | medium |
| `0x802A6F80` | `fn_802a6f80` | 36 | medium |
| `0x802A6FB0` | `fn_802a6fb0` | 540 | medium |
| `0x802A7CA0` | `fn_802a7ca0` | 836 | medium |
| `0x802A8020` | `fn_802a8020` | 1048 | medium |
| `0x802A8440` | `fn_802a8440` | 1180 | medium |
| `0x802A8930` | `fn_802a8930` | 36 | medium |
| `0x802A8960` | `fn_802a8960` | 36 | medium |
| `0x802A8A10` | `fn_802a8a10` | 756 | medium |
| `0x802A8D10` | `fn_802a8d10` | 936 | medium |
| `0x802A9640` | `fn_802a9640` | 1992 | medium |
| `0x802AA830` | `fn_802aa830` | 684 | medium |
| `0x802AE9C0` | `fn_802ae9c0` | 992 | medium |

## Deliberate skips

Seven entries in the bucket stay untranslated.

- **`0x802A4400`** (952 bytes). The x/z strip pair. Its quantised arms write
  the computed vertex bytes, so the live GQR type changes the stored value;
  the same blocker as the wave-100 `0x8028E310`/`0x802888E0` entries.
- **`0x802A71D0`** (2,748 bytes). Thirty-six quantised stores into packed
  vertex records. A faithful body needs the raw `psq` pass.
- **`0x802A90C0`** (1,396 bytes). Reads two local vector arrays through the
  GQR ladder and stores the packed result; the live quantiser type decides the
  output.
- **`0x802A3A20`** (988 bytes). The per-pair record blend. Its `+0x14` reads
  and `_DAT_cc008000` stores pass through the full GQR ladder; kept at
  address-only use.
- **`0x802A3160`** (1,092 bytes) and **`0x802A64B0`** (928 bytes). Standalone
  billboard builders whose live `psq` operands are stack scratch written by
  `0x801C1530` and the callback. Unlike the `float *` node fields above, the
  scratch type is not proven from any neighbouring plain-float access, so the
  type-0 assumption is not available. They are kept at address-only use.
- **`0x802A1090`** (1,588 bytes). Entry/exit spills only, so the arithmetic is
  plain; it is deferred because its `0x801C0540`/`0x8029CFC0` chain aliases
  five 48- and 192-byte stack blocks and the aliasing was not proven inside
  this pass's budget.

## Verification

- Every address and byte size matches `functions.tsv` (25 rows, 17,760 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 291
  translated entries and this pass added 25.
- `node tools/check-lane-csv.mjs wave101` prints `OK` (25 rows for this lane,
  no duplicate, malformed or already-merged row).
- `cargo check --message-format=short` reports no error or warning in
  `src/render/gx_band.rs`.
- The `sdata2` constants used by this pass were read from `work/dol/main.elf`
  through the ELF program headers (`r2 = 0x80433080`); the values are listed
  in section 14 of the module header.
