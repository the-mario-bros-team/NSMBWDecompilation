# Wave 225 lane F coverage (`game::stage`)

Slice: the two highest-address bodies still open in `game::stage` — the
`0x800A_D090` track/collision teardown and the `0x800A_DB00` per-frame vertical
update. Two functions, both translated, none deferred; 188 + 468 =
**656 bytes**. `0x800A_DB00` is the highest remaining address of `main.dol`, so
this lane closes the `game::stage` ledger.

| address | size | body |
| --- | --- | --- |
| `0x800A_D090` | 188 | `fn_800ad090` - when `[[this+0x75C]+4] != 0`: resolve `[this+0x75C][0]` through `0x8023A0B0(this+0x600, name)`, file it with `0x801651F0(this+0x644, this+0x604, &handle, 0)`, detach with `0x80169F80(0.0f, +0x604, +0x644)` and `0x801680E0(1.0f, +0x644)`; then virtual slot `0x98`, `0x8008C380(this+0x79C)`, `+0x114 = -0.275f`, `+0xE8 = +0xF0 = 0.0f`, `+0xEC = 6.5f`, `+0x120 &= ~8` |
| `0x800A_DB00` | 468 | `fn_800adb00` - `0x8006CD90(this)` + `0x8006CDE0(this)`, sample `+0xAC`, `fn_800955f0(this)`, `0x80072BC0(this+0x1EC)` zeroing `+0xEC`, `+0x348 = (+0xE8 < 0.0f)`, `+0x102 += +0xAC0`, `s16` step `±910` from `0x8042C47C[+0x348]` into `0x8015F4F0(this+0x104, step, 0x80)`, virtual slots `0x570`/`0x544`/`0x548`, two-sided `+0xAC4` crossing test, `+0xAC8--`, bounce (`+0xF4 = -+0xF4`) or the zero-counter block (`0x80096240(this, +0xAC)`, slot `0x520` × `0x8042C1A0[+0x348]` into `+0x102`, `+0x104 = 0`, slot `0xD4` with `0x803585C4`) |

## Byte extents

Both sizes were re-derived from the instruction stream (`ppc159.js` over
`work/dol/main.elf`), not copied from `functions.tsv`, and both match the
reference table exactly:

* `0x800A_D090` + `0xBC` → `stwu r1,-0x20` at `0x800A_D090`, `bclr 20,0` at
  `0x800A_D148` (188). Four zero bytes and a bare `bclr` at `0x800A_D150` follow
  as padding with no `functions.tsv` entry.
* `0x800A_DB00` + `0x1D4` → `stwu r1,-0x20` at `0x800A_DB00`, `bclr 20,0` at
  `0x800A_DCD0` (468); four zero bytes at `0x800A_DCD4` separate it from the
  next body at `0x800A_DCE0`.

Neither is a prologue artefact (16/20 bytes): both bodies open with a real
`stwu r1,-0x20` frame and end in `bclr`, and `address + size` lands exactly on
the following alignment boundary.

## Ghidra corrections applied

* `0x800A_D090` - the export types the `0x80169F80` / `0x801680E0` scale
  arguments as `double`; both are `lfs` loads of `f32` literals handed to
  single-precision `f1` parameters, so the body widens the `f32` load to `f64`
  for the call (`w91_call_f64_u32_u32` / `w59_call_f64_u32`).
* `0x800A_D090` - `unaff_r2` is the small-data-2 base. `r2 - 0x6C6C` →
  `0x8042C414 = 0.0f`, `r2 - 0x6C70` → `0x8042C410 = 1.0f`,
  `r2 - 0x6C24` → `0x8042C45C = 0xBE8CCCCD = -0.275f` and
  `r2 - 0x6C20` → `0x8042C460 = 0x40D00000 = 6.5f`.
* `0x800A_DB00` - the `unaff_GQR0` / `ldexpf` noise at both ends of the export
  is the decompiler misreading the `stfd f31` / `psq_st f31` save and its
  restore. No fixed-point maths is involved and the block is elided.
* `0x800A_DB00` - the export prints `FUN_8006cd90()` with no argument; `r3`
  still holds `this` at the `bl` (`0x800A_DB1C`) and the callee's own export
  (`8006cd90_FUN_8006cd90.c`) declares `FUN_8006cd90(int param_1)` and reads
  `+0xE8`/`+0xF4`/`+0x11C`. The argument is passed.
* `0x800A_DB00` - the export types the `0x520` slot result as `short` and the
  table byte as `char`. The instructions are `extsb r0,r5`
  (`0x800A_DC98`), `mullw r0,r0,r3` (`0x800A_DC9C`) on the **full** return word
  and `sth r0,0x102` (`0x800A_DCA4`); the body multiplies the full word with
  `wrapping_mul` and truncates only the product.
* `0x800A_DB00` - `addic. r0,r3,-1` (`0x800A_DC30`) + `bc 4,1` (`0x800A_DC38`)
  negate `+0xF4` only when the decremented counter is **strictly positive**, not
  merely non-zero; the zero/negative case runs the long block.
* `0x800A_DB00` - the two threshold tests are ordered single-precision
  `fcmpo` pairs (`0x800A_DBF8`/`0x800A_DC04` and `0x800A_DC14`/`0x800A_DC20`)
  completed by `cror 2,1,2` / `cror 2,0,2`, giving
  `(previous < t && t <= current) || (t < previous && current <= t)` with
  `previous` sampled **before** the frame's helper calls (`lfs f31,0xac(r31)`)
  and `current` re-read after them. Reproduced with `f32` comparisons.
* `0x800A_DB00` - `lha`/`add`/`sth` for `+0x102 += +0xAC0` make the addition a
  16-bit wrap; reproduced with `u16::wrapping_add` (identical in two's
  complement to the signed form).
* `0x800A_DB00` - `0x80096240` really takes two register arguments
  (`r3` context, `r4` point); the module's Rust wrapper `fn_80096240(point)`
  carries only one. The call is therefore made by address with both words
  explicit, so the point still lands in `r4`.

## Shared constants reused

`SDA2_BASE`, `W224D_F_ZERO` (`r2-0x6C6C`, `0.0f`), `W224D_F_ONE`
(`r2-0x6C70`, `1.0f`), `W69_SIDE_TABLE` (`0x8042_C1A0`),
`COLLISION_AREA_UNREGISTER` (`0x8008_C380`), `FN_8023A0B0_A223`,
`FN_801651F0_A223`, `FN_80169F80_A223`, `FN_801680E0_A223`,
`FN_8006CDE0_A223`, `FN_8015F4F0_F`, `call4`, `w91_call_f64_u32_u32`,
`w59_call_f64_u32`, `read_f32`/`write_f32`, `read_u8`/`write_u8`,
`read_u16`/`write_u16`, `read_u32`/`write_u32` and
[`crate::game::stage::fn_800955f0`] already existed; no existing name was
redefined. New slots are suffixed `_F`: `W225F_F_M0275_F` (`r2-0x6C24`),
`W225F_F_6P5_F` (`r2-0x6C20`), `W225F_SIGN_STEP_F` (`r2-0x6C04`, `{910, -910}`),
`W225F_DAT_803585C4_F`, `FN_8006CD90_F` and `FN_80096240_F`.

`0x803585C4` lives in the `0x80351680..0x80427680` segment, which has no file
image (`.bss`); it reads as zeros at reset, which is why the record is only
referenced by address.

## Deliberate call-by-address sites

`0x8023_A0B0` (name-to-handle), `0x8016_51F0` (registry bind), `0x8016_9F80`
and `0x8016_80E0` (track detach), `0x8008_C380` (collision unlink),
`0x8006_CD90` / `0x8006_CDE0` (height and descent ticks), `0x8007_2BC0`
(ground probe), `0x8009_6240` (point probe), `0x8015_F4F0` (`s16` approach) and
the virtual slots `0x98`, `0xD4`, `0x520`, `0x544`, `0x548` and `0x570` of the
`+0x60` vtable. No `r2`/`r13` slot is inlined as a number and no table entry is
hard-coded as a literal instead of read from the image.

## Verification

The block (`work/wave225/laneF_block.rs`, 290 lines) was appended to a scratch
copy of the crate (`%TEMP%\opencode\wave225-lanef-crate\src\game\stage.rs`,
starting at line 77316) and `cargo check --lib` exits `0`. The log contains no
diagnostic naming any line of the appended region, and the warning count is
unchanged from the untouched baseline (26). The real `src/game/stage.rs` was
not touched.
