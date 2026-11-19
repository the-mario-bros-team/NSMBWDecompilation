# Coverage - Wave 102 (GX / render band, `0x80290000` bucket)

## How to read this file

This page covers the Wave 102 pass over the `0x80290000..0x8029FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unrelocated `main.dol` (image base `0x80004000`). Byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose; `float` constants
were read from `work/dol/main.elf` through the ELF program headers with
`r2 = 0x80433080`.

The bucket holds 85 Ghidra entries. 58 were already translated by waves 12, 18,
96 and 98 (and are listed in `ledger/functions.csv`), which left 27. This pass
adds nine of those 27. The eighteen that remain are the paired-single core of
the particle and G3D mesh emitters; they are listed with the exact reason under
"Deliberate skips".

`status` is `translated` for every row; `confidence` follows the body's
`# Confidence` line.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 9 | 2,792 | 1 | 8 |

## What this pass covers

- **Fog colour blend** (`0x802946F0`, 1,052 bytes). The one entry in the bucket
  whose colour math is byte/integer only. The current fog object selects mode
  `1` (tint by the four bytes at `+0x6D..+0x70`, rounded `>> 8`) or mode `2`
  (transform `instance+0xAC` by the global work matrix at `.bss` `0x80423140`,
  subtract the camera position at `+0x7C..+0x84`, take the vector length
  through `0x801C14C0`, then blend the near bytes at `+0x71..+0x74` towards the
  far bytes with an `8.8` factor and `>> 16`). The two register-restore halves
  of `0x802DCD74` are read as (view instance, fog object), the convention this
  module already uses for the `0x802DCDxx` stubs.
- **Name-table pointer install** (`0x80294B30`, 352 bytes). The classic
  BRRES/G3D "offset word -> pointer" pass: the first word of the resource is
  the table offset, each entry carries its own relative offset plus a second
  relative offset that leads to a `u16`-sized pointer array, the delta array
  that follows it, and a second array further into the same block. Both arrays
  are filled with running pointers. No floating point at all.
- **Float vector kernels** (`0x802960F0`, `0x80296640`, `0x802966F0`). Three
  small helpers whose `psq_l`/`psq_st` operands are plain `float *` records:
  the `VEC3` normalise returning `0`/`1` (`0x802960F0`), the reciprocal of a
  3-vector followed by one `0x802381A0` call (`0x80296640`), and the length of a
  `0x10`-byte-strided 3-component column (`0x802966F0`, already used by the
  translated Euler extraction `0x80295A90`). The same test that wave 101
  applied to `0x802A08A0`/`0x802A8930` applies here: every caller
  (`0x80287930`, `0x80289070`, `0x80291100`, `0x80299510`, `0x80298ED0`,
  `0x8029F900`) reads the destination back with a plain `lfs`, so only the
  type-0/scale-1 arm of the GQR ladder can be live.
- **Matrix scale pair** (`0x80296530`, `0x802965B0`). Two 3x4 scaling kernels
  used on the `0x110`-byte and stack matrices: `0x80296530` scales the three
  rows by `(x, y, z, 1)` (the `w` component is multiplied by the literal `1.0`),
  while `0x802965B0` scales each whole row by one factor. Both substitute
  `1.0e-5f` for a zero factor. `0x80289070` calls the first as
  `(this+0x124, this+0x124, this+0x9C)`, so both operands are provably `float`.
- **Random fixed-point shrink** (`0x80296CE0`, 212 bytes). Advances the
  generator at `context+0xEC` (`seed*0x343FD + 0x269EC3`), turns its high half
  into `r/65536`, multiplies by the `f1` scale and clamps the result to
  `1..=0xFFFF`. Plain integer/float code, no `psq` at all.
- **Billboard axis record** (`0x8029CFC0`, 556 bytes). The helper that fills the
  `+0x98..+0xB8` block of the particle node used by `0x8029F900`: matrix fetch
  `0x80289070`, world matrix `0x80294440`, inverse `0x801C07A0` (with
  `0x801C04D0` as fallback), then two transformed and normalised axes, plus a
  third axis when the descriptor mode byte is `5` or `7`. All `psq` operands are
  `float` columns of that node.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802946F0` | `fn_802946f0` | 1052 | medium |
| `0x80294B30` | `fn_80294b30` | 352 | medium |
| `0x802960F0` | `fn_802960f0` | 100 | medium |
| `0x80296530` | `fn_80296530` | 128 | medium |
| `0x802965B0` | `fn_802965b0` | 136 | medium |
| `0x80296640` | `fn_80296640` | 168 | high |
| `0x802966F0` | `fn_802966f0` | 88 | medium |
| `0x80296CE0` | `fn_80296ce0` | 212 | medium |
| `0x8029CFC0` | `fn_8029cfc0` | 556 | medium |

## Deliberate skips

Eighteen entries in the bucket stay untranslated. They fall into five groups.

1. **Quantised stores whose GQR type/scale is not in the export** (10 entries).
   `0x80296160`, `0x802961E0`, `0x80296260`, `0x802963B0`, `0x80295C20`,
   `0x80296820`, `0x80298720`, `0x80299840`, `0x8029C820`, `0x8029F900`. Each
   writes packed vertex records through the byte/halfword/float ladder. The
   sine/cosine family (`0x80296160`/`0x802961E0`/`0x80296260`/`0x802963B0`)
   additionally uses a quantised angle index into the 256-entry table at
   `0x802F74A0`, and the `2^s` scale factor of GQR3 is not printed; guessing it
   changes every stored value.
2. **Drivers that forward eight float register arguments** (3 entries).
   `0x80298B80`, `0x80298ED0` and `0x80299510` call `0x80298720` with eight
   `f1..f8` values plus eight integer arguments. `0x80298720` is itself in
   group 1, so the drivers cannot be validated and their call sites cannot be
   written honestly.
3. **Particle motion curves** (3 entries). `0x80290690`, `0x80290CF0` and
   `0x80291100` read their caller's `float *` positions through the GQR ladder,
   store the interpolated result back through it, and dispatch into the five
   unit-box classifiers plus the group-1 evaluators. The output quantiser, not
   the arithmetic, is the blocker.
4. **Register-file-sized body** (1 entry). `0x80292C90` (5,464 bytes) takes
   `f14..f31` live-in and its Ghidra C is a 3,400-line projection of the
   paired-single register file.
5. **Export defect** (1 entry). `0x8029BAB0` prints twenty masked
   *read-modify-write* accesses to the write-only GX FIFO at `0xCC008000`
   (`_DAT_cc008000 = _DAT_cc008000 & 0xffffff`). The FIFO cannot be read, so the
   real instruction is a masked store whose operand Ghidra lost; reproducing the
   printed lines would mean inventing the value being written.

## Verification

- Every address and byte size matches `functions.tsv` (9 rows, 2,792 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`.
- No address was already in `ledger/functions.csv`; the module now holds 300
  translated entries and this pass added 9.
- `node tools/check-lane-csv.mjs wave102` reports no problem naming this lane's
  CSV.
- `cargo check --message-format=short` reports no error or warning in
  `src/render/gx_band.rs`.
- The `sdata2`/`.data` constants used by this pass are listed in section 15 of
  the module header (`0x8042ECE8`, `0x8042ECEC`, `0x8042ED04`, `0x8042ED18`,
  `0x8042ED1C`, `0x8042ED24`, `0x8042ED28`, `0x8042ED2C`, `0x8042ED30`,
  `0x8042ED34`, `0x8042ED58`, `0x8042EF18`, `0x8042EF1C`, `0x8042EF40`,
  `0x80429A80`, `0x80429A84`).
