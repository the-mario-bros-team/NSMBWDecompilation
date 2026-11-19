# Wave 221, lane C - `src/resource/archive.rs`, addresses `0x802C97F0..0x802CE290`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x802C0000`
band of the `resource::archive` ledger module. All ten are translated; none
deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C97F0` | 28 | 28 (`blr` at `0x802C9808`) | clear one record's `+0xA0` bit 0 |
| `0x802C9A20` | 8 | 8 (`blr` at `0x802C9A24`) | constant 2 |
| `0x802C9AF0` | 272 | 272 (`blr` at `0x802C9BFC`) | reset one 0xA4-byte record |
| `0x802CA3E0` | 216 | 216 (`blr` at `0x802CA4B4`) | apply a lookup record to a record pair |
| `0x802CA6D0` | 168 | 168 (`blr` at `0x802CA774`) | copy an expanded scratch record in |
| `0x802CBE20` | 80 | 80 (`blr` at `0x802CBE6C`) | reset one pane record |
| `0x802CBE70` | 388 | 388 (`blr` at `0x802CBFF0`) | install the tile grid and draw the quad |
| `0x802CC570` | 444 | 444 (`blr` at `0x802CC728`) | map a tile index to pixels and emit |
| `0x802CD9F0` | 144 | 144 (`blr` at `0x802CDA7C`) | accumulate descriptor sizes and count |
| `0x802CE010` | 656 | 656 (`blr` at `0x802CE29C`) | dispatch a descriptor by mode |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` in `work/dol/main.elf`, and in every
case `prologue_offset + 4` equals the tsv size and the `blr` word ends exactly
at that boundary (`0x802C9A20` is the two-instruction frameless case, `li r3,2`
plus `blr`). Bytes translated: **2404**.

## What the run is

These ten are the record/tile half of the `0x80350370`-class layout object the
neighbouring lanes documented. The object holds a table of 0xA4-byte records at
`+0x08` (count byte at `+0x04`) and a table of 12-byte slots at `+0x0C`
(count byte at `+0x05`); the record fields this lane touches are
`+0x04` (kind/id word), `+0x2C..+0x2F` (a four-byte colour/opacity word),
`+0x38..+0x3B` (the default colour `00 00 00 FF`), `+0x40`/`+0x44`/`+0x48`
(angle and scale pair), `+0x3C..+0x3F` (four flag bytes), `+0x50..+0x64` (the
step block) and the `u16` flag halfword at `+0x68`. `// inferred:` the owner is
the RLYT/scene layout record plus the global tile atlas in the `r13` small-data
area; the exports prove only the offsets, the constants and the callee
addresses.

1. **Record flags.** `0x802C97F0` clears bit 0 of a record's `+0xA0` halfword.
   This is the exact helper `fn_802c8840` (wave 220, lane C) already calls
   before it re-walks the second archive table, and it re-sets the bit
   afterwards.

2. **Record reset.** `0x802C9AF0` is the per-record initialiser the existing
   `fn_802c84f0` (wave 220, lane E) calls once per `+0x04` count with one
   record pointer. It first hands two default blocks to `0x802CA8A0` - the
   3-float `{-10000, 10000, 10000}` and the 6-float `{0, 0, 0, -10000, 10000,
   10000}` whose leading triple comes from the zero words at `0x802FD1D0` -
   then writes `FF FF FF FF` at `+0x2C`, `00 00 00 FF` at `+0x38`, `1.0` at
   `+0x30`, `0`/`1` at `+0x3C`/`+0x3D`, `90.0` at `+0x40`, `0.5` at
   `+0x44`/`+0x48`, `0` at `+0x3E`/`+0x3F`, `16.0` at `+0x64`, `1.0` at
   `+0x58`/`+0x4C`, `0.0` at `+0x60`/`+0x54`/`+0x5C`/`+0x50` and `0x661` at
   `+0x68`.

3. **Apply pair.** `0x802CA3E0` expands the lookup record into a scratch block
   with `0x80241EB0` (the caller stamps scratch `+0x20` and `+0x38` with `-1`
   first) and, when `mode == 0` or scratch bit 2 is set, either clears the
   `+0x68` bit 0 of the record and its predecessor, or runs `0x802CA560` on the
   record and `0x802CA6D0` on the predecessor, copying `+0x04` to
   `predecessor + 6` and setting `predecessor + 0x68` bit 1 when that reports
   success. `0x802CA6D0` is the copy half: on scratch bit 3 it clears the
   record via `0x802CA4C0`, copies the scratch colour bytes `+0x38..+0x3B` to
   `+0x2C..+0x2F`, hands `scratch + 8` and `scratch + 0x14` to `0x802CA8A0`,
   sets `+0x68` bit `0x800` and copies the scratch `f32` at `+0x3C` to
   `+0x64`; on a clear bit it clears `+0x68` bit 0 and answers `0`.
   `0x802CA3E0` is called with the frame time in `f1` by `fn_802c8840` but
   never reads it, so the parameter is kept only for the register shape.

4. **Pane reset.** `0x802CBE20` runs the render-state initialiser `fn_802c0240`
   (already in the module), the `0x802C1270` tail thunk of `fn_802c11e0` with
   selection `0`, zeroes `+0x0A`/`+0x0B` and clears bit 4 of `+0x18`.

5. **Tile grid.** `0x802CBE70` builds a 0x44-byte blit record with
   `0x801C1350` from the four arguments (six `f32`: `param_4`, `0`, `0`,
   `param_3`, `0`, `1`), submits it with `0x802D31A0`, uploads the rect
   `(param_1, param_2, param_3, param_4, 0, 1)` as six floats through
   `0x802D3200` and the same four words through `0x802D3230`, closes with
   `0x802D3270(0, 0)`, runs the channel-9/13 GX ladder
   (`0x801C4070`, `0x801C37C0`, `0x801C4780` with the `0x8042F480` word pair,
   `0x801C40B0` twice), publishes the grid (`tile = *(u16 *)(r13 - 0x5B18)` to
   both step words, the four `i16` argument copies, then
   `param_3 / tile` and `param_4 / tile`) and finally installs the fixed render
   state with `0x802CB410`.

6. **Tile emission.** `0x802CC570` turns a tile index into the pane's pixel
   rectangle: `row = index / across`, `+0x7A = row_step * (index - row *
   across)`, `+0x7C = row * col_step`, `+0x78 = 0`. It then runs `0x802C0B60`,
   fills a 0x30-byte quad with `0x801C04D0`, submits
   `(row_step, col_step, 1.0)` through `0x801C0C60`, patches the quad words at
   `+0x0C` and `+0x1C` with `+0x7A` and `span_y - (+0x7C + col_step)`, draws
   it through `0x801C9A60`/`0x801C9940`/`0x801C9A80`, opens the 4-vertex
   texcoord stream with `0x801C5570(0x80, 0, 4)` and writes the four
   `(vertex, s, t)` triples `(0,0,1) (1,0,0) (2,1,0) (3,1,1)` to the
   `0xCC008000` FIFO. The `0x802D2FE0` depth latch is saved first and restored
   last.

7. **Descriptor walkers.** `0x802CD9F0` walks `*(u16 *)(owner + 6)` entries,
   sizes each through `0x802CDB70(entry + 0x2C, out + 0x20)`, adds the
   returned `+0x04` size into `out + 0x04`, advances the scratch pointer by it
   and stores the count at `out + 0x10`. `0x802CE010` is the matching
   dispatcher: it guards on `param_3`/`param_4`, brackets the work with the
   `r13 - 0x5B08` active word, and branches on `target + 4` into the player
   path (`0x802CE6B0` descriptor, vtable slot `+0x0C` selectors 6/7, or
   `0x8025BFE0` slots 2/3 plus `0x80252710`), the full-screen path
   (`0x802CE6F0` descriptor, `0x8024D0B0`, `0x8024E180` with the
   `+0xE8`/`+0xEC` pair) and the controller path (flag fold `2`/`4` into
   `1`/`2`, then the nested `+0xF0` slot through `+0x08 -> +0x10`).

## Two export quirks, recorded in the bodies

* `0x802CD9F0` and `0x802CE010` open with `bl 0x802DCD70` / `bl 0x802DCD74`.
  Those are `__savegprlr_26` / `__savegprlr_27`: six `stw` instructions and a
  `blr`, no result. Ghidra models them as returning a `u64` and then reads
  `r3`/`r4` out of that "return", which is why both exports print `(void)` and
  route the first arguments through `iVar1`/`piVar8`. The real live-in
  registers are `r3`/`r4` (and, for `0x802CE010`, `r3..r7`), and the lane
  bodies take them as parameters. The stubs themselves and the closing
  `0x802DCDBC` / `0x802DCDC0` restores are omitted, as in the earlier lanes.
* `0x802CA3E0` is called by `fn_802c8840` through the local float-first helper,
  so its signature here is `(f32, r3..r6)` even though the callee never reads
  the float.

## New names

Added, all suffixed `_C221` to avoid the wave-220 `_C` names already in the
module: `DAT_802FD1D0_C221`, `DAT_8042F480_C221`,
`SDA2_F32_MIN_10000_C221`, `SDA2_F32_MAX_10000_C221`,
`SDA2_F32_ZERO_3C50_C221`, `SDA2_F32_ONE_3C48_C221`, `SDA2_F32_90_3C44_C221`,
`SDA2_F32_HALF_3C40_C221`, `SDA2_F32_16_3C3C_C221`,
`SDA2_F32_ZERO_3C20_C221`, `SDA2_F32_ONE_3C1C_C221`,
`SDA2_BYTES_3E40_C221`, `SDA2_BYTES_3E3C_C221`,
`G_TILE_SIZE_5B18_C221`, `G_TILE_SPAN_X_47A0_C221`, `G_TILE_SPAN_Y_479E_C221`,
`G_TILE_ORIGIN_X_479C_C221`, `G_TILE_ORIGIN_Y_479A_C221`,
`G_TILE_ROW_STEP_4798_C221`, `G_TILE_COL_STEP_4796_C221`,
`G_TILE_ACROSS_4794_C221`, `G_TILE_DOWN_4792_C221`,
`G_DESCRIPTOR_ACTIVE_5B08_C221`, `GX_FIFO_C221`, plus the callee constants
`FN_80241EB0_C221`, `FN_802CA560_C221`, `FN_802CA4C0_C221`,
`FN_802CA6D0_C221`, `FN_802CA8A0_C221`, `FN_802D31A0_C221`,
`FN_802D3200_C221`, `FN_802D3230_C221`, `FN_802D3270_C221`,
`FN_802CB410_C221`, `FN_802CDB70_C221`, `FN_802CE6B0_C221`,
`FN_802CE6F0_C221`, `FN_8024D5E0_C221`, `FN_8024D0B0_C221`,
`FN_8024E180_C221`, `FN_80252710_C221`, `FN_8025BFE0_C221`, and the three
helpers `call_f32x6_C221`, `vcall_void4_C221`, `vcall_inner3_C221`.

Reused from the module: `SDA_BASE`, `SDA2_BASE`, `platform::call0..call8`,
`FN_801C04D0`, `FN_801C4070`, `FN_801C37C0`, `FN_801C40B0`,
`FN_801C4780_E`, `FN_801C5570`, `FN_801C9A60`, `FN_801C9940_105`,
`FN_801C9A80_C`, `FN_801C0C60_A`, `FN_802D2FE0_A`, `call_blit7`,
`call_f32x3_ptr_a`, and the two module bodies `fn_802c0240` / `fn_802c11e0`.

## Constant values

Read from the image (`work/dol/main.elf`, `r2 = 0x80433080`, `r13 =
0x8042F680`): `r2 - 0x3C58` = `-10000.0f`, `r2 - 0x3C54` = `10000.0f`,
`r2 - 0x3C50` = `0.0f`, `r2 - 0x3C48` = `1.0f`, `r2 - 0x3C44` = `90.0f`,
`r2 - 0x3C40` = `0.5f`, `r2 - 0x3C3C` = `16.0f`, `r2 - 0x3C20` = `0.0f`,
`r2 - 0x3C1C` = `1.0f`; `r2 - 0x3E40` = `00 00 00 FF`, `r2 - 0x3E3C` =
`FF FF FF FF`; `r2 - 0x3C00` = `00000001 01010100`; `0x802FD1D0` = three
zero words. The `r2 - 0x3BF8` double used by the `u32 -> f32` idiom is
`2^52` and the `r2 - 0x3BE8` double used by the signed conversion is
`2^52 + 2^31`; the bodies replace both idioms with plain Rust casts, which
compute the same values.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to the copy's `src/resource/archive.rs`, and
`cargo check --lib` there exits `0` with no diagnostic pointing at
`src/resource/archive.rs`.
