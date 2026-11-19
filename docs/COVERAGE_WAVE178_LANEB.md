# Wave 178 lane B coverage (`game::bases`)

Four bodies translated (1,848 bytes) from REL `d_basesNP`; nothing deferred.
All four were read from the raw instruction stream and from the Ghidra export.
No edits were made to `src/game/bases.rs`; the bodies are in
`work/wave178/laneB_block.rs`.

| Address | Bytes | Body |
| --- | --- | --- |
| `0x807ADC00` | 460 | `fn_807adc00` - pane rect pair at `+0xD8`/`+0x1C0` and `6000.0` scale block |
| `0x807AEEA0` | 460 | `fn_807aeea0` - pane rect pair at `+0xD4`/`+0x1BC` and `10000.0` scale block |
| `0x80731380` | 464 | `fn_80731380` - `houdai` gun-part dispatcher (`houdai_rot_large_gun`, `gun_upper/left/right`) |
| `0x80758880` | 464 | `fn_80758880` - `Wm_mr_` aim-angle sweep with barrel rotation latching |

## Reuse of already-merged neighbours

- `fn_807ADC00` / `fn_807AEEA0` are the same body as the merged
  [`fn_807b08b0`] (`0x807B08B0`, 460 bytes) with one extra argument-shift:
  `0x807ADC00` moves every offset down one word (`+0xD8`/`+0x138`/`+0x19C`/
  `+0x1C0`), `0x807AEEA0` keeps the `0x807B08B0` offsets exactly
  (`+0xD4`/`+0x134`/`+0x198`/`+0x1BC`) and only swaps the `+0x1A0` constant to
  `10000.0`. The three bodies share the `0x80164B20(1)` record, the
  `[0x8042A480]+6`/`+4` dimension reads, the `0x800B54B0`/`0x802DCC4C` height
  branch, both `0x802BF420`/`0x802BFFF0` rect calls, the nine-word store block,
  `0x802C0150` and the `+0x[V][0x28]` / `+0x[V+0x60][0x14]` / `0x802BE880`
  tail, so the merged body was copied and only the literals were changed.
- `fn_80731380` reuses the resource probe idiom of the merged
  [`fn_80722180`]/[`fn_807836f0`] (`0x8023B300` + `+8`, `0x802E1AB4` compare) and
  the `0x8024` `1710`/`1AA0`/`1A00` decompose/recompose/translate trio used by
  [`fn_807268c0`], including its `mulli 90` / `srawi 14` / `addze` truncation.
- `fn_80758880` is the last member of the wave-132 `Wm_mr_` cluster
  (`0x80758A70..0x80759860`): it uses the same
  `*(*(this+0x128)+4)` model pointer, the merged `vmethod_f32x3_u32` for slot
  `+0x5C` with three floats plus a selector, `vmethod1(this+0x12C, 0x18, ...)`
  for the pane message, and the same `0x807CEA58..0x807CEAD4` constant pool.

## Notes taken while translating

- `0x802DCD74` / `0x802DCDC0` are `_savegpr_27` / `_restgpr_27` compiler
  helpers, not semantic calls; `r3` survives them, which is why Ghidra prints
  `FUN_80731380(undefined4 param_1, ...)` and then uses `iVar3` as `param_1`.
  `xoris ...,0x8000` plus the `0x43300000`/`0x807CD748` pair is just the signed
  `int -> double` conversion; the Rust uses plain integer division, which has
  the same truncation-toward-zero semantics as `srawi`+`addze`.
- `FUN_807ADC00` never writes `+0x198` (the store ladder starts at `+0x19C`),
  unlike `FUN_807B08B0`, which starts at `+0x198`. This is not a Ghidra
  artifact: `stfs r2,0x19c(r30)` is the first store in the raw words.
- `FUN_80758880`'s `psq_l ...,1,3` guards are Ghidra noise; the actual
  sample load is a `psq_l` with GQR3 = `0x00050005`, i.e. unsigned 16-bit, so
  the dequantise is `angle as u16 as f32` (same idiom as [`fn_80669560`]).
  Ghidra's `(longlong)(double)cStack_46` ladder is wrong.
- `FUN_80758880` caveat: the multiplier is loaded into `f30` only inside the
  `+0x23C == 2` and `== 3` arms (`lfs f30,0xb0(r30)` twice) but is used
  unconditionally at `0x807589CC`/`0x807589D0`. On any other phase the raw
  stream would read an uninitialised non-volatile `f30`, which cannot be
  intentional, so the body is taken to be reachable only in those two phases
  and the literal `8.0` is used on every path. This is the only reason the
  body is marked `medium`.
- Constants read from the ELF: `0x807D0768 = 0.0`, `0x807D0770 = 1.0`,
  `0x807D077C = 6000.0`, `0x807D0780`/`0x807D07B0`/`0x807CD748` = the `2^52`
  magic, `0x807D0794 = 0.0`, `0x807D079C = 1.0`, `0x807D07A8 = 10000.0`,
  `0x807CD710 = 1.0`, `0x807CEA5C = 0.0`, `0x807CEAC0 = 8.0`,
  `0x807CEAC4 = 3.0`, `0x807CEAC8 = 0.00390625`,
  `0x807CEAB0 = [100.0, 200.0, 300.0, 400.0]`.
- Strings verified in the REL: `0x8080D044 = "houdai_rot_large_gun"`,
  `0x8080D05C = "gun_upper"`, `0x8080D068 = "gun_left"`,
  `0x8080D074 = "gun_right"`, `0x80832380 = ""`.

## Deferred

None.
