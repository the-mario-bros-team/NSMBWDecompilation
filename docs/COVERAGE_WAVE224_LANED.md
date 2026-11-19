# Wave 224 lane D coverage (`game::stage`)

Slice: ten bodies of the `0x800A_7690..0x800A_A620` band — one position
submitter, one state clear, five byte-identical operand getters, and three
actor frame/track helpers. Ten functions, ten translated, none deferred;
652 bytes.

| address | size | body |
| --- | --- | --- |
| `0x800A_7690` | 76 | `fn_800a7690` - builds `{position[0], position[1], 480.0f}` on the frame and submits it as `0x80064610(0x237, 1, &block, 0, 0)` |
| `0x800A_7BF0` | 12 | `fn_800a7bf0` - `[this + 0x154] = 0` |
| `0x800A_7DE0` | 12 | `fn_800a7de0` - `return &DAT_80377A04` |
| `0x800A_8770` | 12 | `fn_800a8770` - `return &DAT_80377A04` |
| `0x800A_87C0` | 12 | `fn_800a87c0` - `return &DAT_80377A04` |
| `0x800A_8830` | 12 | `fn_800a8830` - `return &DAT_80377A04` |
| `0x800A_8880` | 12 | `fn_800a8880` - `return &DAT_80377A04` |
| `0x800A_99D0` | 188 | `fn_800a99d0` - `+0x6A4`/`+0x3C` guarded track stop at `+0x67C`, virtual slot `0x98`, `0x8008C380(this+0x79C)`, `0x800E23C0(0.0f, 24.0f, r13-0x5630, this, 5)`, `+0x770` lookup through `0x80162D00` fed to `0x80162510`, virtual slot `0xD4` with `&DAT_803584C4` |
| `0x800A_A150` | 208 | `fn_800aa150` - `+0xE8` sign select, `0.9f - sign` truncation index, `+0x840[index] + 0x800B30C0()`, steered `+0xAC` raise/drop by `1.0f` |
| `0x800A_A620` | 108 | `fn_800aa620` - virtual slot `0x520` speed times `0x8042C1A0[+0x348]`, sign-extended and handed to the `s16` approach `0x8015F4F0(this+0x102, target, step)` |

## Byte extents

Every size was re-derived from the instruction stream (`ppc159.js` over
`work/dol/main.elf`), not copied from `functions.tsv`, and all ten match the
reference table exactly:

* `0x800A_7690` + `0x4C` → `blr` at `0x800A_76D8` (76);
* `0x800A_7BF0` + `0x0C` → `blr` at `0x800A_7BF8` with no prologue (12);
* `0x800A_7DE0` + `0x0C` → `lis`/`addi`/`bclr` (12);
* `0x800A_8770` + `0x0C` → `bclr` at `0x800A_8778` (12);
* `0x800A_87C0` + `0x0C` → `bclr` at `0x800A_87C8` (12);
* `0x800A_8830` + `0x0C` → `bclr` at `0x800A_8838` (12);
* `0x800A_8880` + `0x0C` → `bclr` at `0x800A_8888` (12);
* `0x800A_99D0` + `0xBC` → `bclr` at `0x800A_9A88` (188);
* `0x800A_A150` + `0xD0` → `bclr` at `0x800A_A21C` (208);
* `0x800A_A620` + `0x6C` → `bclr` at `0x800A_A688` (108).

None of these ten is a prologue artefact: each body has a real `stwu`/`blr`
pair (or, for the twelve-byte getters, no prologue at all) and the next
function starts exactly at address + size.

## Ghidra corrections applied

* `0x800A_99D0` - the export prints `iVar1 = FUN_80162d00();` and
  `if (iVar1 != 0) FUN_80162510();` with no arguments. The instruction stream
  loads `r3 = [this + 0x770]` at `0x800A_9A38` for the lookup and leaves the
  **lookup result** in `r3` for the second `bl` at `0x800A_9A58`, so both
  arguments are explicit in the body. The export also prints the track stop as
  `FUN_801662f0((double)*…, this+0x67c, 1)`; `0x801662F0`'s own body only reads
  `r3`/`r4`, so the `f1 = 0.0f` load at `0x800A_99F4` is a dead register set —
  it is still performed (through `call1_f64_ptr2`) so the call site matches the
  original register state.
* `0x800A_A150` - the export widens the coefficient sum to `double`
  (`fVar1 = (float)((double)… + dVar3)`); the instruction is
  `fadds f2,f3,f1`, so the add is single precision. `0x800B30C0` returns its
  result in `f1` as an `f32` (its body ends in `fadds`); the already-translated
  [`crate::game::stage::fn_800b30c0`] widens it to `f64` exactly, so narrowing
  it back reproduces the original addend. The `fctiwz` truncation is
  reproduced with `as i32`; the only reachable inputs are `-0.1f` and `1.9f`,
  giving indices `0` and `1`.
* `0x800A_A150` - the height is read twice in the original: once before the
  `0x800B30C0` call (index selection and the `== 0.0f` override) and once after
  it (`0x800AA1B4`, the two steered-branch tests). Both reads are kept.
* `0x800A_A620` - the export types the multiply operand as `short`. The
  instruction is `mullw r0,r0,r3` on the *full* return word; only the product
  is truncated by `extsh` at `0x800AA66C`, and the table byte is signed by
  `extsb` at `0x800AA660`. The body multiplies the full word and truncates the
  product.

## Shared constants reused

`SDA2_BASE`, `W84_TYPE_INFO` (`0x8037_7A04`), `W69_SIDE_TABLE`
(`0x8042_C1A0`), `W69_G_NOTICE_OWNER` (`r13-0x5630`),
`COLLISION_AREA_UNREGISTER` (`0x8008_C380`), `FN_800E23C0_F`,
`FN_80162D00_F`, `FN_801662F0_A223`, `FN_8015F4F0_F` and
[`crate::game::stage::fn_800b30c0`] already existed; no existing name was
redefined. New slots are suffixed `_D`: `W224D_F_480` (`r2-0x6CA8`),
`W224D_F_ZERO` (`r2-0x6C6C`), `W224D_F_24` (`r2-0x6C64`), `W224D_F_ONE`
(`r2-0x6C70`), `W224D_F_MONE` (`r2-0x6C5C`), `W224D_F_09` (`r2-0x6C48`),
`W224D_DAT_803584C4`, `FN_80064610_D` and `FN_80162510_D`, plus the local ABI
helper `w224d_call2_f64_ptr3_D` for the five-argument `0x800E23C0` call site.

`0x803584C4` lives in the `0x80351680..0x80427680` segment, which has no file
image (`.bss`); it reads as zeros at reset, which is why the record is only
referenced by address.

## Deliberate call-by-address sites

`0x8006_4610` (position submitter), `0x800E_23C0` (kind-5 notice),
`0x8016_62F0` (track stop), `0x8008_C380` (collision unlink), `0x8016_2D00`
(id lookup), `0x8016_2510` (destroy walk), `0x8015_F4F0` (`s16` approach) and
the three virtual slots `0x98`, `0xD4` (`+0x60` of the actor) and `0x520`.
No `r13`/`r2` slot is inlined as a number.

## Verification

The block (`work/wave224/laneD_block.rs`) was appended to a scratch copy of the
crate (`%TEMP%\opencode\wave224-laned-crate\src\game\stage.rs`) and
`cargo check --lib` exits `0` with no diagnostic naming a line below
`74303` (the appended region starts after the 74302-line original). The real
`src/game/stage.rs` was not touched.
