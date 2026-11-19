# Wave 223 lane D coverage (`game::stage`)

Slice: ten bodies of the `Wm`-family actor state publishers plus three teardown
and dispatch helpers in the `0x800A0000` band
(`0x800A_02D0..0x800A_7110`, 1756 bytes). Ten functions, ten translated, none
deferred.

| address | size | body |
| --- | --- | --- |
| `0x800A_02D0` | 228 | `fn_800a02d0` - proximity pass: two `fadds` position pairs into `0x80064A30`, `+0x6C` virtual slot byte, `0x8009F5F0`/`0x8009F710` frame index, guarded `0x800E20A0`, `0x194` effect at `this+0xAC`, `0x800A0FB0` |
| `0x800A_0550` | 432 | `fn_800a0550` - height-mode selection (`+0xE8 < r2-0x6D88`), `0x8006CED0`/`0x80066630` point emission, `0x800954D0` with the `+0x90/+0x94` pair, `0x80065B80` mode from `other+0x510 == 2`, `+0x134` counter clamp `8`, eight-word `0x802F0660` record at `+0x414..+0x432` |
| `0x800A_0A30` | 252 | `fn_800a0a30` - `+0x490` guard, `+0x480` height flag, four-slot `0x8005F900`/`0x80162D00` player rebind via `0x8012E510`, sign-extended `+0x6C` byte into `+0x4B4`, `[this+0x394] + 0x30` call with `&DAT_80358004` |
| `0x800A_0C70` | 328 | `fn_800a0c70` - mode-`0`/`1` `0x800943D0` resample and scale pick (`r2-0x6D78`/`r2-0x6D74`), `r13-0x5770` owner with `0x800A7C60`/`0x800A7C00`, `0x8009F6F0` frame id, eight-word `0x802F06C0` record |
| `0x800A_12A0` | 208 | `fn_800a12a0` - `0x800A8170(this)` (`0x800B8850(this+0x484)`), variant byte `+0x459` into `0x8042C1B0[variant]`, six-word `0x802F0784..0x802F0798` record, `+0x430`/`+0x431` bytes |
| `0x800A_2BB0` | 12 | `fn_800a2bb0` - `return &DAT_80377A04` |
| `0x800A_4270` | 60 | `fn_800a4270` - `0x800A42B0(&block, out)` then `0x8016F130(&block, out)` over a `0x34`-byte stack block |
| `0x800A_43E0` | 124 | `fn_800a43e0` - `+0xE8 *= 0x802F080C`, guarded `+0xEC` decay above `r2-0x6D44`, `+0x744` `i16` advanced by `0x802F0810` through `fctiwz` |
| `0x800A_5440` | 20 | `fn_800a5440` - `+0xD38 = 2`, `+0x788 = 0`, tail call `0x800A4AC0(this)` |
| `0x800A_7110` | 92 | `fn_800a7110` - `0x8015F800(this, 0)` and `0x802B9280(this)` when `release > 0`; returns `this` |

## Byte extents

Every size was re-derived from the prologue and the `blr` (not copied from
`functions.tsv`) and all ten match the table exactly:

* `0x800A_02D0` + `0xE4` → `blr` at `0x800A_03B0`;
* `0x800A_0550` + `0x1B0` → `blr` at `0x800A_06FC`;
* `0x800A_0A30` + `0xFC` → `blr` at `0x800A_0B28`;
* `0x800A_0C70` + `0x148` → `blr` at `0x800A_0DB4`;
* `0x800A_12A0` + `0xD0` → `blr` at `0x800A_136C`;
* `0x800A_2BB0` + `0x0C` → `blr` at `0x800A_2BB8`;
* `0x800A_4270` + `0x3C` → `blr` at `0x800A_42A8`;
* `0x800A_43E0` + `0x7C` → `blr` at `0x800A_4458`;
* `0x800A_5440` + `0x14` → tail branch `b 0x800A4AC0` at `0x800A_5450`;
* `0x800A_7110` + `0x5C` → `blr` at `0x800A_7168`.

## Ghidra corrections applied

* `0x800A_0550` - Ghidra prints `uVar13 = FUN_802dcd74()` as an `undefined8`
  and then uses `(int)uVar13` and `(ulonglong)uVar13 >> 0x20` as a pointer and
  a word. `0x802D_CD74` is the five-instruction save thunk
  (`stw r27..r31` at `r11-0x14..r11-4`, then `blr`), so `r3`/`r4` still hold
  `param_1`/`param_2`; the body uses `this` and `other` and never models the
  fake 64-bit return.
* `0x800A_0550` / `0x800A_0A30` - both height tests are the
  `fcmpu cr0,f1,f0` + `cror cr0[EQ],cr0[GT],cr0[EQ]` fold, i.e.
  `height >= X`; the branch `bc 4,2`/the `cntlzw`+`rlwinm 27,24,31` fold both
  invert it, so the stored flag is `height < X`. This matches Ghidra's `uVar11`
  in `0x800A_0550` and its `e8 < X` in `0x800A_0A30`; no correction was needed
  beyond reading `cror` operand `1` as `GT` (not `LT`).
* `0x800A_0C70` - the `unaff_GQR0`/`ldexpf` guards, `dVar7` and the
  `local_48`/`local_44` split are artefacts of the `stfd f31`/`psq_st f31`
  save pair and of the FPR/GPR numbering; the raw words use `lfs`/`frsp` and
  the body keeps plain single-precision values. `0x800A7C00` declares one
  parameter, so the `li r4,0` before the call is dead and is not passed.
* `0x800A_43E0` - the `lha`/`xoris 0x8000`/`ldfd 0x43300000_xxxxxxxx`/
  `fsubs` sequence is the signed-short-to-double idiom; `read_u16(..) as i16 as
  f32` reproduces the exact value, including negative counters.
* `0x800A_4270` - the export shows `FUN_800a42b0(auStack_38)` with one argument,
  but the words also load `r4 = param_1` (`mr r31,r3` / `mr r4,r31`), and
  `0x800A42B0` reads `[r4+0x60]+0x2F8`; both arguments are kept.
* `0x800A_0A30` - the `+0x4B4` store is `lbz` + `extsb`, so the byte is
  sign-extended; the body stores `read_u8(..) as i8 as i32`.

## Shared constants reused

`W84_TYPE_INFO` (`0x8037_7A04`), `W84_VARIANT_TABLE` (`0x8042_C1B0`),
`W92_F_SAMPLE_X` (`r2-0x63F8`), `W69_F_SAMPLE_Y` (`r2-0x63F4`),
`W69_G_NOTICE_OWNER` (`r13-0x5630`), `W69_G_EVENT_MANAGER` (`r13-0x5924`) and
`DELETE_ONE` (`0x802B_9280`) already existed in `stage.rs`. New slots are
suffixed `_D`: `F_HEIGHT_6D88_D`, `F_POINT_Z_6DA0_D`, `F_CAM_Z_6D78_D`,
`F_CAM_Z_6D74_D`, `F_THRESH_6D44_D`, `G_CTRL_5770_D`, `G_EMITTER_5940_D`,
`REC_A_802F0660_D`, `REC_B_802F06C0_D`, `REC_C_802F0780_D`,
`F_SCALE_802F080C_D`, `F_STEP_802F0810_D`, `DESC_80358004_D`, plus the local
ABI helper `call_f32x2_D` for the `0x80064A30` dispatch.

## Deliberate call-by-address sites

`0x8006_4A30` (float dispatcher), `0x8006_A4B0`, `0x8006_CED0`, `0x8006_5B80`,
`0x8006_6630`, `0x8009_43D0`, `0x8009_54D0`, `0x8009_F5F0`, `0x8009_F6F0`,
`0x8009_F710`, `0x8019_7F00`, `0x800E_20A0`, `0x8012_E510`, `0x8016_2D00`,
`0x8005_F900`, `0x800A_0FB0`, `0x800A_42B0`, `0x800A_4AC0`, `0x800A_7C00`,
`0x800A_7C60`, `0x800A_8170`, `0x8016_F130`, `0x8015_F800`, `0x802B_9280`.
No vtable slot or `r13`/`r2` slot is inlined as a number.

## Verification

The block was appended to a scratch copy of the crate
(`%TEMP%\opencode\wave223-laned-crate\src\game\stage.rs`) and
`cargo check --lib` exits `0` with no diagnostic naming a line above `65911`
(the appended region). The real `src/game/stage.rs` was not touched.
