# Wave 225 lane E coverage (`game::stage`)

Slice: the last two unclaimed bodies before `0x800AD000` in the `Wm` actor
vtable band - the two animation-set actions at `0x800AC310` and `0x800ACF20`.
Two functions, two translated, none deferred; 304 bytes of code (the
`functions.tsv` sizes total 312 because both entries count four trailing
alignment bytes).

| address | size | body |
| --- | --- | --- |
| `0x800AC310` | 116 | `fn_800ac310` - guarded (`[[this+0x758]] != 0`) entry lookup of the `+0x758` key through `0x8023A0B0(this+0x600; key)`, bind `0x801651F0(this+0x644; this+0x604; entry; 1)`, rate `0x80169F80(0.0f; +0x604; +0x644)`, rate `0x801680E0([this+0xACC]; +0x644)` |
| `0x800ACF20` | 196 | `fn_800acf20` - guarded (`[[this+0x75C]+4] != 0`) entry lookup of the `+0x75C` key, bind with flag `0`, rates `0.0f`/`1.0f`; then virtual slot `0x98` of the `+0x60` vtable, `0x8008C380(this+0x79C)`, `+0x114 = -0.275f`, `+0xE8 = 0.0f`, `+0xEC = 6.5f`, `+0xF0 = 0.0f`, clear bit 3 of `+0x120`, `+0x100 = 0` |

## Byte extents

Both sizes were re-derived from the instruction stream (`ppc159.js` over
`work/dol/main.elf`); each differs from `functions.tsv` by exactly four bytes,
the same tail artefact on both bodies:

* `0x800AC310` - `stwu r1,-0x20` ... `bclr` at `0x800AC380` = **112** bytes.
  The table value of 116 reaches `0x800AC384`, which is four zero bytes
  (`OP_0`) that precede the 4-byte slot function at `0x800AC390`.
* `0x800ACF20` - `stwu r1,-0x20` ... `bclr` at `0x800ACFE0` = **192** bytes.
  The table value of 196 reaches `0x800ACFE4`, four `OP_0` bytes before the
  4-byte slot function at `0x800ACFF0`.

Neither is a 16/20-byte prologue artefact: both bodies start with the full
four-instruction `stwu`/`mfspr`/`stw`/`stw` frame and end on their own `bclr`.

## Ghidra corrections applied

* `0x800AC310` - the export prints `local_18[0] = FUN_8023a0b0(param_1 + 0x600);`
  with a single argument. The instruction stream loads the key into `r4` at
  `0x800AC328` (`lwz r4,0x758(r3)` then `lwz r4,0x0(r4)`) and never rewrites
  `r4` before `bl 0x8023a0b0` at `0x800AC338` (only `r3` is re-derived by
  `addi r3,r3,1536`). `0x8023A0B0` is the two-argument `(holder, key)` lookup
  (`work/ghidra/main-gb/export/8023a0b0_FUN_8023a0b0.c`: `FUN_80239c40(&local_18,
  param_2)`), so the key is passed in `fn_800ac310`. This is the only inferred
  element in the lane, hence `medium` confidence for that body.
* `0x800ACF20` - the export names the float operands through `unaff_r2`
  because `r2` was folded away. The `lfs` displacements are `0xFFFF9394`
  (`r2 - 0x6C6C` -> `0x8042C414` = `0.0f`) and `0xFFFF9390`
  (`r2 - 0x6C70` -> `0x8042C410` = `1.0f`); the two state stores use
  `0xFFFF93DC` (`r2 - 0x6C24` -> `0x8042C45C` = `-0.275f`) and
  `0xFFFF93E0` (`r2 - 0x6C20` -> `0x8042C460` = `6.5f`). All four were read
  back from the image at those virtual addresses.
* `0x800ACF20` - the store order in the export (`+0x114`, then
  `+0xE8`/`+0xEC`/`+0xF0`) mirrors the `stfs` order; the `li r0,0` at
  `0x800ACFA4` is a separate, later load from the `lwz r3,0x120(r31)` at
  `0x800ACFA0`, so the mask store and the `+0x100` halfword store are
  independent (the body keeps them separate).

## Reused symbols

`platform::call1/call2/call4`, `read_u32`, `read_f32`, `write_f32`,
`write_u16`, `call1_f64_ptr`, `call1_f64_ptr2`, `COLLISION_AREA_UNREGISTER`
(`0x8008C380`), `FN_8023A0B0_A223`, `FN_801651F0_A223`, `FN_80169F80_A223`,
`FN_801680E0_A223`, `W224D_F_ZERO` (`r2 - 0x6C6C`), `W224D_F_ONE`
(`r2 - 0x6C70`). New constants this lane adds, both `_E`-suffixed:
`W225E_F_M0275` (`r2 - 0x6C24`) and `W225E_F_65` (`r2 - 0x6C20`).

## Verification

Scratch crate: `src/`, `Cargo.toml` and `Cargo.lock` copied to
`%TEMP%\wave225-laneE-crate`, `laneE_block.rs` appended to the copy's
`src/game/stage.rs`, `cargo check --lib` -> exit code **0**, 26 warnings (the
pre-existing baseline count; none from this lane).
