# Wave 220, lane C - `src/resource/archive.rs`, addresses `0x802C60D0..0x802C8AC0`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x802C0000`
band of the `resource::archive` ledger module. All ten are translated; none
deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C60D0` | 156 | 156 (`blr` at `0x802C6168`) | guarded field stores through `*out` |
| `0x802C6550` | 128 | 128 (`blr` at `0x802C65CC`) | construct the 0x20-byte slot array |
| `0x802C6A30` | 100 | 100 (`blr` at `0x802C6A90`) | projection object constant block |
| `0x802C6E50` | 44 | 44 (`blr` at `0x802C6E78`) | derive then submit the matrix |
| `0x802C70B0` | 224 | 224 (`blr` at `0x802C718C`) | derive the seven projection floats |
| `0x802C78B0` | 64 | 64 (`blr` at `0x802C78EC`) | `(pointer, count)` guarded free |
| `0x802C7B20` | 12 | 12 (`blr` at `0x802C7B28`) | byte-table lookup |
| `0x802C7EF0` | 272 | 272 (`blr` at `0x802C7FFC`) | allocate and bind one play object per slot |
| `0x802C8840` | 640 | 640 (`blr` at `0x802C8ABC`) | re-apply both archive tables to the slots |
| `0x802C8AC0` | 60 | 60 (`blr` at `0x802C8AF8`) | pack slot word 0 into the caller array |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` in `work/dol/main.elf`, and in every
case the `blr` ends exactly at `address + tsv_size`; whatever follows up to the
next entry is four-byte padding before that function's prologue (the two frameless
bodies `0x802C6A30` and `0x802C7B20` are the same shape: `blr` + 4 bytes of
`0x00000000`). Bytes translated: **1700**.

## What the run is

These ten sit directly after the wave-219 lane C block and belong to one object
family: a slot-table owner that keeps

* a table of 12-byte slots at `+0x0C` with its count byte at `+0x05`, and
* a table of 164-byte records at `+0x08` with its count byte at `+0x04`,

and resolves the underlying data through a pointer-to-pointer at the caller's
`container` argument. The pointed-to container exposes counts at `+0x3C`,
`+0x3E` and `+0x40`, relative offsets at `+0x14`, `+0x18` and `+0x1C`, and
records of `0x4C`, `0x1C` and `0x5C` bytes respectively - the shape of the
sound-archive index blocks that the `0x8026Fxxx` reader in this file already
documents. `// inferred:` the owner is the NW4R sound-archive player layer; the
exports prove only the offsets, the strides, the constants and the callee
addresses.

1. **Slot record copy.** `0x802C60D0` reloads `*out` before each field, so a
   null descriptor skips only that field. `+0x00` is the `+0x1C`-bit-0
   selected word (0 or the word at `+0x18`), `+0x04`/`+0x08` are the two floats
   at `+0x08`/`+0x0C`, `+0x14..+0x17` are the colour word at `+0x04` spilled
   and re-read byte by byte (most significant byte first), and `+0x18` is bit 1
   of the byte at `+0x1C`.

2. **Slot array.** `0x802C6550` installs `0x80350318`, stores the count at
   `+0x08` and `+0x0A`, allocates `(count & 0xFFFF) * 0x20 + 0x10` through
   `0x802B9250` and hands that block to `0x802DC7E0` together with the element
   constructor `0x802C6040` (which installs `0x803502F8`) and the element
   deleter `0x802C64F0` (the same guarded free as `0x802C78B0`).
   `0x802C7EF0` fills the slots: per index it allocates one play object with
   `0x8025EFB0(owner, scratch, 0x802C8150, 1, 0, 0)`, links the object back to
   its slot at `+0xF0`, attaches the shared 80-byte descriptor filled by
   `0x801C04D0` through `0x80257570`, stamps the index at `+0x08`, the owner at
   `+0x00` and the literal `3` at `+0x0A`, then binds the completion callback
   with `0x802C8000(this, index, index & 0xFF, 1)`. `owner == 0` selects the
   default at `r13 - 0x47B8`.

3. **Projection math.** `0x802C6A30` installs `0x80350338` at `+0x38` and the
   constant block: `45.0f` at `+0x10`, `2.4142134f` (`1 + sqrt(2)`) at `+0x14`,
   `0.0f` at `+0x20`/`+0x24`, `1.0f` at `+0x28..+0x30`, the two `f64` bounds
   narrowed to `+0x18`/`+0x1C` and the enable word `1` at `+0x34`.
   `0x802C70B0` asks `0x802C7190` for four bounds and derives
   `{ 1, 2/(hi-lo), (hi+lo scaled), 2/(c-d), (c+d scaled), -1/(h-w),
   -h/(h-w) }` - seven floats, no zero guard, so a degenerate extent yields
   `inf`/`nan` exactly as the shipped `fdivs` does. `0x802C6E50` is the
   two-call wrapper that derives the block into a 40-byte stack record and
   submits it through `0x802D31D0`.

4. **Table and free helpers.** `0x802C7B20` is a two-instruction `lbzx` into
   the ten-byte table at `0x8042F3E0` (`1E 21 24 27 2A 2D 30 33 3F 80`);
   `0x802C78B0` frees only when the pointer is non-zero and the count is
   positive (signed compare) and returns the pointer unchanged.

5. **Table re-apply.** `0x802C8840` is the largest body. With `flags & 2` it
   sets `+0x1E` bit `0x10`, writes `*container` to `+0x20` and the low half of
   `value` to `+0x24`. With `flags & 1` it clears the `+0xA0` bit of every
   164-byte record (`0x802C97F0`), walks the slots up to
   `min(+0x05, *(u16 *)(*container + 0x3E))`, and for the one record selected
   by `0x802425E0` (or all slots when that lookup returns 0) copies the
   `0x80242630` lookup record into the slot with `0x802C8C30` and sets slot
   bit 0; then walks the records up to the `0x802425E0` record's `+0x1A` byte
   count, clears each record's `+0x68` bit 0 and, when `0x80242680` resolves
   the index and the record is not already marked, calls `0x802CA3E0` with the
   previous record (only when the lookup record has `& 8` and its `+0x14`
   index is below `min(+0x04, *(u16 *)(*container + 0x40) + *(u16 *)(*container
   + 0x36))`) and sets the record's `+0xA0` bit 0. The index array it walks is
   the `u16` sequence at `lookup + i * 2 + 0x3C`.

6. **Packing.** `0x802C8AC0` copies word 0 of each 12-byte slot into
   `*(target + 0x2880)` with a 4-byte stride, `*(u8 *)(this + 5)` times.

## Two export quirks, recorded in the bodies

* `0x802C8840` keeps its first argument in `f1` (`fmr f31,f1`) and forwards it
  to both apply helpers. Ghidra prints the two calls through a phantom `u64`
  return from the register-save helper `0x802DCD68`, so the float argument
  disappears from the printed parameter list of `0x802C8840`, `0x802C8C30` and
  `0x802CA3E0`. The signatures used here are the register ones (`f1` plus
  `r3..r6` / `r3..r5`), supplied by the lane-local helpers
  `call_f32_u32x3_c` / `call_f32_u32x4_c`. Neither callee reads `f1`; the value
  is passed because the call sites arm it.
* `0x802C7EF0` has the same phantom-`u64` artefact; its real registers are
  `r3 = this`, `r4 = count`, `r5 = owner`.

## New names

Added, all suffixed `_C`: `VTABLE_80350318_C`, `VTABLE_80350338_C`,
`FN_802C6040_C`, `FN_802C64F0_C`, `FN_802C70B0_C`, `FN_802C7190_C`,
`FN_802C8000_C`, `FN_802C8150_C`, `FN_802C97F0_C`, `FN_802C8C30_C`,
`FN_802CA3E0_C`, `FN_802DC7E0_C`, `FN_802D31D0_C`, `FN_8025EFB0_C`,
`FN_80257570_C`, `FN_802425E0_C`, `FN_80242630_C`, `FN_80242680_C`,
`SDA2_C_ONE_3CC8`, `SDA2_C_45_3CC4`, `SDA2_C_TAN_3CC0`, `SDA2_C_ZERO_3CBC`,
`SDA2_C_TWO_3CB4`, `SDA2_C_MINUS_ONE_3CB0`, `SDA2_C_BYTE_TABLE_3CA0`,
`G_C_DEFAULT_OWNER_47B8`, plus the two local call helpers `call_f32_u32x3_c`
and `call_f32_u32x4_c`.

Reused from the module: `SDA_BASE`, `SDA2_BASE`, `HEAP_DELETE`, `FN_801C04D0`
and `fn_802b9250` (the 12-byte trampoline that allocates `(size, 4, 0)`).

## Constant values

Read from the image (`work/dol/main.elf`, `r2 = 0x80433080`): `r2 - 0x3CC8` =
`1.0f`, `r2 - 0x3CC4` = `45.0f`, `r2 - 0x3CC0` = `2.4142134f`, `r2 - 0x3CBC` =
`0.0f`, `r2 - 0x3CB4` = `2.0f`, `r2 - 0x3CB0` = `-1.0f`; `r2 - 0x3CA0` starts
the byte table `1E 21 24 27 2A 2D 30 33 3F 80`. `r13 - 0x47B8` is in the
zero-filled small-data segment, so its image value is not mapped.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to the copy's `src/resource/archive.rs`, and
`cargo check --lib` there exits `0` with no diagnostic pointing at
`src/resource/archive.rs`.
