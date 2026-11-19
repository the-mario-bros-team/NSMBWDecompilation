# Wave 219, lane C - `src/resource/archive.rs`, addresses `0x802C0010..0x802C4D80`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x802C0000`
band of the `resource::archive` ledger module. All ten are translated; none
deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C0010` | 44 | 44 (`blr` at `0x802C0038`, no frame) | corner pair -> width/height/aspect |
| `0x802C0240` | 124 | 124 (`blr` at `0x802C02B8`) | initialise the 0x13-byte state record |
| `0x802C05E0` | 1104 | 1104 (`blr` at `0x802C0A2C`) | two colour runs plus an eased gradient |
| `0x802C1710` | 168 | 168 (`blr` at `0x802C17B4`) | build one texture object, fill one quad |
| `0x802C17D0` | 8 | 8 (`blr` at `0x802C17D4`) | TEV constant-select word |
| `0x802C18C0` | 364 | 364 (`blr` at `0x802C1A28`) | full render-state reset |
| `0x802C2F00` | 104 | 104 (`blr` at `0x802C2F64`) | rebind one GX descriptor pair |
| `0x802C31A0` | 88 | 88 (`blr` at `0x802C31F4`) | two palette/format selectors |
| `0x802C3570` | 64 | 64 (`blr` at `0x802C35AC`) | free when the count argument is positive |
| `0x802C4D80` | 548 | 548 (`blr` at `0x802C4FA0`) | one scaled, textured sprite quad |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` in `work/dol/main.elf`, and in every
case the `blr` ends exactly at `address + tsv_size`; whatever follows up to the
next entry is four-byte `blr`-padding before that function's `stwu`.
Bytes translated: **2616**.

## What the run is

All ten belong to the immediate-mode render layer rather than to the archive
readers that dominate this file: the callees are the `0x801C0000`-band GX
wrappers that write the FIFO register `0xCC008000` (`_DAT_cc008000`) and the GX
state block at `*(r2 - 0x4EF8)`, plus the `0x802C1F80..0x802C2700` chain of the
same band. `// inferred:` the owners are render/texture records and one sprite
record; the exports prove only the offsets, the masks, the constants and the
callee addresses.

1. **Float-record normalisers.** `0x802C0010` turns the two corner pairs at
   `+0x00..+0x0C` into `width`, `height` and the raw `width / height`
   quotient at `+0x10..+0x18`; it has no frame and no guard, so a zero height
   produces `inf`/`nan` exactly as the shipped `fdivs` does. `0x802C4D80` is
   the consumer: it scales the record's two extents at `+0x64`/`+0x68` by
   `source_size / target_size`, where the source sizes are the `u16` pair at
   `+0x04`/`+0x06` of the record reached through `+0x88` and the target sizes
   are `(int)*(float *)(target + 0x08) & 0xFFFF` /
   `(int)*(float *)(target + 0x0C) & 0xFFFF` of the object behind
   `r13 - 0x47B0`. The five-float array passed to `0x801C7E00` is
   `{ sx, 0, 0, 0, sy }`.

2. **Colour ramp.** `0x802C05E0` fills `count` four-byte entries:
   `0 ..= first` with the bytes at `from`, `last - 1 .. count - 1` with the
   bytes at `to`, and the gap between them with
   `0x802C0450((i - first) / (last - first), entry, from, to)`. The first
   argument selects the easing: `t^2`, `t^4`, `t^8`, `t^16`,
   `1 - (1 - t)^2`, `1 - (1 - t)^4`, and the identity for every other value.
   The two fills are the compiler's 8-entry unrolled `lbz`/`stb` copies; the
   export's `unaff_GQR0`/`ldexpf` lines are the unrelated paired-single
   save/restore prologue, and `bl 0x802DCD64` is the Metrowerks
   `__savegprlr_23` helper, which is why the export believes the first
   argument and the destination arrive in one `u64`. The body is modelled as
   byte loops plus the exact single-precision easing chain.

3. **State resets.** `0x802C18C0` re-programs the whole path: the two mode
   bytes through `0x802D2F20`/`0x802D2F80`, the `0x802C1F80` /
   `0x802C2060` / `0x802C20E0` passes, two `0xFFFFFFFF` fill words from
   `r2 - 0x3E3C`, the colour word, then the TEV stage, blend and z-mode calls
   with literal arguments. One divergence from the export is recorded in the
   body's doc block: the `r13 - 0x4744 == 1` arm stores the argument into the
   *high* byte of the stack word (`value << 24`) and the other arm into the
   low byte (`value`); the export names the two slots the other way round.
   `0x802C0240` initialises the 0x13-byte record those bodies read and is the
   only body here that copies from four different small-data widths
   (`r2 - 0x3450` bytes, `r2 - 0x3E78` word, `r2 - 0x3E74` halfword,
   `r2 - 0x3E72` byte; the image word is `0x13F`). `0x802C31A0` selects the
   palette pair (`2`/`3` vs `1`/`1`) and `0x802C2F00` clears one GX object
   twice and then loads the `index * 8` entry of the table at `0x804245B8`.

4. **Accessor and guarded free.** `0x802C17D0` is a single `lwz` of
   `r13 - 0x5B40` (`0x80429B40`, image word `1`); `fn_802c1f80` forwards that
   word as the `param_5` konstant-colour selector of `0x801C6AF0`, whose body
   packs `(v & 0xF0) << 7 | (v & 0xF) << 2`, which is why the constant is
   named after the TEV konstant select. `0x802C3570` frees through
   `0x802B9280` only when the pointer is non-zero and the count argument is
   positive, then returns the pointer unchanged.

## Reuse and new names

Reused from the module: `SDA_BASE`, `SDA2_BASE`, `G_ARC_HEAP_CURRENT`
(`r13 - 0x4870`, the default record source of `0x802C1710`), `HEAP_DELETE`,
`call8` and `call_mixed9` (the eight-word and three-`f64`-plus-six-word
helpers of the wave-99/wave-105 sections), and the callee constants
`FN_801C04D0`, `FN_801C4A20`, `FN_801C47D0_105`, `FN_801C5860`,
`FN_801C6900`, `FN_801C69E0`, `FN_801C6D90`, `FN_801C6FF0`,
`FN_801C74C0_105`, `FN_801C7E00`, `FN_801C8110`, `FN_801C8130`,
`FN_801C82F0_105`, `FN_801C8330_105`, `FN_801C8370_105`,
`FN_801C83D0_105`, `FN_801C8430_105`, `FN_801C86C0`, `FN_801C8790`,
`FN_801C88F0`, `FN_801C8DC0`, `FN_801C9940_105`.

Added, all suffixed `_C`: `FN_802C0B30_C`, `FN_802C0450_C`,
`FN_802C2700_C`, `FN_802C1F80_C`, `FN_802C2060_C`, `FN_802C20E0_C`,
`FN_802D2F20_C`, `FN_802D2F80_C`, `FN_801C99C0_C`, `FN_801C95E0_C`,
`FN_801C0C60_C`, `FN_801C9A80_C`, `FN_801C8040_C`, `FN_801C7F40_C`,
`FN_801C8180_C`, `G_C_802C2F00_TABLE`, `G_C_TEX_OBJ_80424678`,
`G_C_TEX_SRC_80350120`, `G_C_QUAD_80424648`, `G_TEV_KONST_SELECT_5B40_C`,
`G_RENDER_FORMAT_4744_C`, `G_PALETTE_MODE_4731_C`, `G_SCREEN_SIZE_47B0_C`,
`SDA2_C_REC_BYTES_3450`, `SDA2_C_REC_WORD_3E78`, `SDA2_C_REC_HALF_3E74`,
`SDA2_C_REC_BYTE_3E72`, `SDA2_C_ZERO_3E28`, `SDA2_C_FILL_3E3C`,
`SDA2_C_ZERO_3D58`, `SDA2_C_ONE_3D40`, and the two local call helpers
`copy_rgba_c`, `call_f64_ptr3_c`, `call_f64x3_ptr_c`.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to the copy's `src/resource/archive.rs`, and
`cargo check --lib` there exits `0` with no diagnostic pointing at
`src/resource/archive.rs`.
