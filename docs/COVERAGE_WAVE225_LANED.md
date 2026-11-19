# Wave 225 lane D coverage (`game::stage`)

Slice: the last two `game::stage` bodies of the `0x800A_BC30..0x800A_C078` run,
the paired model-animation rebuilds of one actor class. Two functions, two
translated, none deferred; 320 bytes.

| address | size | body |
| --- | --- | --- |
| `0x800A_BC30` | 148 | `fn_800abc30` - guard on the `+8` handle of the `+0x754` model table; `0x8023A0B0(+0x600, handle)` -> `0x801651F0(+0x644, +0x604, &entry, 1)` -> `0x80169F80(0.0f, +0x604, +0x644)` -> `0x801680E0([+0xACC], +0x644)`; then `+0xE8`/`+0xEC`/`+0xF0` = `0.0f` and `+0x348 = 0x80096240(+0xAC)` |
| `0x800A_BFD0` | 172 | `fn_800abfd0` - the same pass guarded on the `+0x10` handle of the `+0x754` table, then `+0xE8`/`+0xEC`/`+0xF0` = `0.0f` and virtual slots `+0x530` then `+0x560` of the `+0x60` vtable, each with `this` |

## Byte extents

Both sizes were re-derived from the instruction stream (`ppc159.js` over
`work/dol/main.elf`); neither is a prologue artefact, and both match
`functions.tsv` exactly:

* `0x800A_BC30` + `0x94` -> `bclr 20,0` at `0x800A_BCC0` (148). The three
  `0x00000000` words at `0x800A_BCC4..0x800A_BCCC` are alignment padding before
  the 4-byte stub whose `bclr` sits at `0x800A_BCD0`.
* `0x800A_BFD0` + `0xAC` -> `bclr 20,0` at `0x800A_C078` (172). The single
  `0x00000000` word at `0x800A_C07C` is padding before `0x800A_C080`.

Both prologues are the plain `stwu r1,-0x20` / `mfspr r0,256` / `stw r0,0x24(r1)`
/ `stw r31,0x1c(r1)` form, so no saved-register helper hides part of the body.

## Ghidra corrections applied

* Both exports print the name resolver as
  `FUN_8023a0b0(param_1 + 0x600)` with a single argument. The raw stream loads
  `r4 = *(*(this + 0x754) + 8)` (resp. `+ 0x10`), tests it against zero and
  leaves it in `r4` across the `bl 0x8023a0b0`; `0x8023A0B0` reads that register
  (`mr r31,r4` at `0x8023A0D0`, forwarded to `0x80239C40` at `0x8023A0FC`). The
  bodies therefore pass the handle explicitly.
* Both exports print `unaff_r2` because the register-saving prologue is inlined
  rather than split out; `r2 = 0x80433080`, so `lfs ...,0xffff9394(r2)` is
  `0x8042_C414`, whose image word is `0x00000000` (`0.0f`).
* `0x800A_BFD0` re-reads `[this + 0x60]` before the second virtual call
  (`0x800A_C054`), after the `+0x530` slot has been given the chance to replace
  the vtable; the body keeps both loads instead of hoisting one.

## Reuse

No new constant, helper, `DAT_*` or vtable address was introduced. The bodies
reuse `FN_8023A0B0_A223`, `FN_801651F0_A223`, `FN_80169F80_A223`,
`FN_801680E0_A223`, the `f32`-exact call shims `w223b_call_f32_ptr` /
`w223b_call_f32_ptr2`, `W224D_F_ZERO` (`r2 - 0x6C6C`) and
[`crate::game::stage::fn_80096240`], all already in `src/game/stage.rs`.

## Verification

Scratch copy of `src/`, `Cargo.toml` and `Cargo.lock` under
`%TEMP%\opencode\w225laneD`, with this lane's block appended to
`src/game/stage.rs`: `cargo check --lib` exits 0 (26 pre-existing warnings only,
none from the two new bodies).
