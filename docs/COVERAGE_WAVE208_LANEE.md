# Coverage - Wave 208 lane E (`ui::hud`, main.dol)

> File name note: the shared instructions name this document
> `docs/COVERAGE_WAVE205_LANE<X>.md`, but that path is owned by the wave-205
> lane E report. This lane follows the `COVERAGE_WAVE208_LANEB.md` precedent and
> lands as `docs/COVERAGE_WAVE208_LANEE.md` instead of overwriting unrelated
> work.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 8 | 2500 | 2 | 6 |

Addresses `0x8003_EDD0`-`0x8003_F8B0`: the eight per-frame bodies that continue
the shared enemy-actor band the wave-207 lane-E block stopped at (`0x8003A0C0`).
The run is one generic frame, one walking-state arming body, three landing /
airborne frames, a spin-down frame, a static spin factory and the spin-up frame.
No deferrals.

## What landed

- **`0x8003EDD0` (568) - generic frame.** Base call chain
  (`0x8006CD90`/`0x8006CE70`/`0x800955F0`/`0x8003E850`/`0x80096F60`/
  `0x8006EF40`/`0x80097050`), then: release the `+0x614` signed contact flag
  (clearing bit 31 of `+0x614/+0x620/+0x630` and reinstalling through
  `0x8006D090`, or just clearing `+0x274`); zero `+0xEC` while any
  `0x3C000000` bit is set; while a `0x15 << side` bit is set run `0x8003E900`
  and, if the `+0x1EC` record has no `0x1FE000` field, flip `+0x348` and
  multiply `+0xE8` by `-0.5`; then either arm the airborne timer `+0x500 = 12`
  or run `0x800957B0(0.1875, 0.875, 0.5)`, the `0.25` landing branch
  (`0x8003EB50(-8.0)`, `0x80070720 == 3` resets `+0xEC`) and the one-shot
  `0x1BA` sound plus `+0x102 = {12000, -12000}[side]`.
- **`0x8003F010` (344) - walking-state arming.** Three `+0x18` descriptor
  tests on the `+0x394` owner chain (`0x80353DA8`, `0x80353B24`, `0x80353AE4`)
  gate the `"walk"` clip bind on `+0x7F8`/`+0x7FC`/`+0x83C`, then `+0x77C = 1`,
  `+0x114 = -0.1875`, `+0x102`/`+0xE8` from the side tables, `+0xF0 = 0.0`,
  `+0xF8 = -4.0`, `+0x898 = 0`, `+0x894 = 2`.
- **`0x8003F180` (476) - landing frame.** `+0x7FC` virtual `+0x1C` first; the
  `+0x45D`-gated reset of `+0x114/+0xF4/+0xF8/+0xFC`; the two `+0x1EC` queries
  bracket the base chain so the `+0x44C += +0x310` branch only runs on the
  ground-to-ground transition; the airborne arm sets `+0xEC = 0.0`, tests
  `0x80353D68` through `+0x14`, notifies `0x80353DA8` on a side mismatch, and
  otherwise installs the side speed and `+0x898 = 1` (with `1.85` when
  `0x8006A200(1)` answers non-zero); the final `0x15 << side` test notifies
  again.
- **`0x8003F360` (180) - stop-and-turn.** `0x80353E24` through `+0x14` gates
  the `"walk"` rebind, then both `+0xE8` and `+0xF0` are zeroed and `+0x348` is
  flipped.
- **`0x8003F430` (252) - airborne frame.** The same `+0x45D` reset, five base
  calls, the `1.85`/`0.0` rate choice on `0x8006A200(1)`, and virtual `+0x2E4`
  deciding whether `0x80353CA8` is sent to `+0xD4`.
- **`0x8003F5E0` (204) - spin-down.** `0x800957B0(0.1875, 1.0, 0.5)`, the
  `0x3C000000`/`0x3F` flag work, then `+0x884 -= 0x800` and
  `+0x100 -= 0x800`; `0x80353CE8` is notified when the counter lands on `0`.
- **`0x8003F6B0` (252) - static spin factory.** `0x802DCD74` supplies the
  record; it is reset (`+0x7C4 = 0`, `+0xEC = 2.0`, `+0x100 = -0x8000`,
  `+0x884 = 0x8000`, `+0xF4 = 0.0`, `+0x118 = -4.0`, `+0x77C = 1`), sound
  `0x1BB` is posted, the eleven contact words `+0x614..+0x63C` are installed
  through `0x8006D090`, `+0x168 = 6.0`/`+0x16C = 8.0` follow, and
  `0x802DCDC0` closes.
- **`0x8003F7D0` (224) - spin-up.** The same blend and flag work, then
  `+0x884 -= 0xC00` and `+0x100 += 0xC00`; `0x80353CA8` is notified once the
  counter is above `0x8000` unsigned.

## Corrections and recoveries

- **All eight `functions.tsv` sizes are exact.** Each body was walked to its
  terminal `bclr`: `0x238`, `0x158`, `0x1DC`, `0xB4`, `0xFC`, `0xCC`, `0xFC`,
  `0xE0`. The gaps between them hold code `functions.tsv` does not list as
  functions: the `blr`-only `0x8003F170` (between `0x8003F010` and
  `0x8003F180`) and `0x8003F420` (between `0x8003F360` and `0x8003F430`), the
  two small `0x8003F530`/`0x8003F550` bodies after `0x8003F430`, and the
  `0x8003F8B0`/`0x8003F8C0` tail branches after `0x8003F7D0`. None of them is
  in this lane, so none is translated or counted here.
- **The exports print `f` registers as `r`.** `lfs r0,...` / `stfs r0,...` in
  the disassembly are float loads and stores. The consequence that mattered is
  `0x8003F010`: the export merges `lfs f3, r2-0x797C` with the following
  `li r3,0` into one variable, which appears to store `-0.1875` at `+0x898` and
  `0` at `+0x114`. The machine code stores `-0.1875` at `+0x114` and `0` at
  `+0x898`; the body follows the machine code.
- **`0x8003EDD0`'s "lift above `0.25`" test is strict `>`.** The sequence is
  `fcmpo cr0,f1,f0` (with `f1 = +0xEC`, `f0 = 0.25`) + `ble` past the block, so
  the landing branch runs when `+0xEC > 0.25`. The export prints the mirrored
  `const < +0xEC`, which is the same predicate; the body writes it as `>`.
- **`0x8003F180`'s "on the ground" test is `<=`, not `<`.** The machine code
  is `fcmpo` + `cror 2,0,2` + `bne`, i.e. `+0xEC <= 0.0`. The export prints a
  strict `<`.
- **`0x8003F180`'s two `+0x1EC` queries are distinct.** The first result is
  kept in `r31` across `0x80096F60`/`0x8006EF40`/`0x80097050`, the second is
  taken after them; only the pair proves the "was airborne, now grounded"
  transition that gates the `+0x44C` accumulator. The body models it as
  `was_airborne` plus a second query.
- **`0x8003F5E0`/`0x8003F7D0` counter arithmetic is unsigned with wrap.** The
  `+0x884` word is an `s32` in the DWARF-free sense but both bodies use
  `addi`/`addic.`, and `0x8003F7D0` compares with `cmpli` + `ble` (`> 0x8000`
  unsigned). The bodies use `wrapping_sub`/`wrapping_add`, and the `+0x100`
  halfword update is reproduced as `u16` wrap in both.
- **`0x8003F5E0` notifies on `== 0`, not `< 0`.** `addic.` sets the flag from
  the subtract and the branch is `bne` past the notify, so the notify is on
  exactly zero.
- **`0x8003F6B0`'s descriptor word is `0xFFFFC000`.** The export prints
  `0xffffc000`; the `li r11,-16384` produces the sign-extended value, so the
  body stores `0xFFFF_C000` into `+0x624` rather than `0x4000`.
- **`0x8003F6B0`'s sound record is the two-float `0x8006A4B0` output.**
  `0x8006A4B0` writes only `+0x00` and `+0x04`, so the stack buffer is the
  same `[u32; 2]` the wave-207 lane-E bodies use, not the 16/40 bytes the
  exports' `auStack_*` sizes suggest.
- **`0x8003F010`/`0x8003F360` `0x801651F0` order.** Both call sites load
  `r3 = this + 0x83C` and `r4 = this + 0x7FC`, and the following
  `0x80169F80(blend, +0x7FC, +0x83C)` / `0x801680E0(blend, +0x83C)` confirm
  that `+0x83C` is the clip holder and `+0x7FC` the part object. The bodies keep
  that order, which is the reverse of the `+0x540`/`+0x544` pairs elsewhere in
  this module.
- **`0x80169F80`/`0x801680E0` receive the `lfs` pattern widened.** Both
  callees read `f1` as a double while the callers use `lfs`; the bodies follow
  the documented convention of `fn_80031df0` (`read_f32(..) as f64`).
- **`0x8023A0B0`'s name argument is `r13 - 0x7DA8`.** The ELF bytes at
  `0x804278D8` are `77 61 6C 6B 00`, so the name is `"walk"`; the neighbouring
  string is `"paipo"`. It is a different `"walk"` copy from the
  `0x80427898` one already used by `fn_80033310`.
- **Descriptors stay unnamed.** `0x80353AE4`, `0x80353B24`, `0x80353CA8`,
  `0x80353CE8`, `0x80353D68`, `0x80353DA8` and `0x80353E24` are `.bss`
  objects of the same family as the existing `DAT_80353A64`/`0x80352DB4`
  (unmapped in `main.elf`, so their contents cannot be read). Each is kept as
  its own `_E` constant and marked `unknown`.

## New constants (all `_E`-suffixed)

`FN_8006CD90_E`, `FN_8006CE70_E`, `FN_8003E850_E`, `FN_80096F60_E`,
`FN_80097050_E`, `FN_8003E900_E`, `FN_800957B0_E`, `FN_8003EB50_E`,
`FN_80096240_E`, `FN_8006A200_E`, `FN_802DCD74_E`, `FN_802DCDC0_E`,
`DAT_80353AE4_E`, `DAT_80353B24_E`, `DAT_80353CA8_E`, `DAT_80353CE8_E`,
`DAT_80353D68_E`, `DAT_80353DA8_E`, `DAT_80353E24_E`, `S_WALK_7DA8_E`,
`F32_SDA2_79C0_E`, `F32_SDA2_79AC_E`, `F32_SDA2_79B0_E`, `F32_SDA2_79B4_E`,
`F32_SDA2_79BC_E`, `F32_SDA2_799C_E`, `F32_SDA2_7974_E`, `F32_SDA2_7970_E`,
`F32_SDA2_796C_E`, `F32_SDA2_7968_E`, `F32_SDA2_7964_E`, `F32_SDA2_7960_E`,
`F32_SDA2_797C_E`, `F32_SDA2_7978_E`, `TABLE_F32_79D0_E`, `TABLE_S16_79C4_E`.

Reused from earlier lanes: `FN_8006CDE0`, `FN_800955F0`, `FN_8006EF40`,
`FN_8006D090`, `FN_8006A4B0`, `FN_80070CB0_B`, `FN_80070720`,
`FN_80197F00_D`, `FN_801651F0_A`, `FN_80169F80`, `FN_801680E0`,
`FN_8023A0B0`, `G_SDA_5924_F`, plus `call0_r`, `call_3f_1u`,
`call_f32_u32`, `call_f64_u32`, `w205f_call_f64_u32x2`, `p32`,
`vtable_method`, `read_f32`, `write_f32`, `SDA_BASE`, `SDA2_BASE`.

## Verification

- Scratch build: `src/`, `Cargo.toml` and `Cargo.lock` copied to
  `%TEMP%\opencode\w208e`, `work/wave208/laneE_block.rs` appended to that
  copy's `src/ui/hud.rs`, `cargo check --lib --offline` exits `0`; no warning
  mentions `hud.rs`.
- Ledger: `ledger/agents/wave208-laneE.csv` (8 rows, all `translated`).
- This document: `docs/COVERAGE_WAVE208_LANEE.md`.
