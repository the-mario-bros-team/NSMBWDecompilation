# Coverage - Wave 175 lane D (`game::bases`, 404/408-byte classes)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 5 | 2,028 | 3 | 2 |

Five bodies: three 404-byte and two 408-byte functions. Source of the bodies:
`work/wave175/laneD_block.rs`.

## What this pass covers

- **Pose-slider angle/transform updater (1 body).** `0x80762010` normalises the
  clip slider `0x80168080(+0x22C)` against the length `+0x254 - 1.0`, scales it
  by the per-variant `8`-byte entry at `0x808151D4` and adds the result to Y.
  It recomputes the ratio, replaces it with `2.0 - ratio` when
  `0x801680C0(+0x22C)` is not positive, and truncates
  `180.0 * ratio * 0x8042DCDC` into the source angle `+0x102`. The shared tail
  (`0x801C0BD0`, `0x8016EF50`, `0x8016A170/A150/A1A0`) is the same as the
  existing [`fn_8075aa50`].
- **Per-variant cloud model loader (1 body).** `0x80779CA0` formats
  `"CS_W%d"` (`0x80817EF0`) with `byte[0x8042A24D] + 1`, mounts
  `"g3d/model.brres"` (`0x80817EF8`) and then builds the model/animation pair at
  `+0x1A8`/`+0x1AC`/`+0x1B8` from the descriptor tables at `0x80817EE0` and the
  lazily built table at `0x80832BB0`. Structurally identical to the existing
  [`fn_80760070`].
- **Seven-state cloud interaction machine (1 body).** `0x8078E9A0` runs phases
  `0..6` on `+0x1C8`: sound `0x76`/`0x77` plus `0x8079BC10`, a `0x5A`-frame
  wait polled with [`fn_8079bd40`], [`fn_8078e680`]/[`fn_8078e690`] around the
  player latch `+0x547`, cue `0xE`/`0x15` into the player `+0x55C`, and finally
  [`fn_8079bce0`] plus the object's virtual `+0x68`.
- **Duplicated ground-probe span snap (1 body).** `0x8064DCC0` scans down and
  then up in `16.0` steps with `0x80070BF0(x, y, byte[+0x38F])`, skipping answer
  types `4`/`5`, and writes two quantised 2-float endpoints (`0.0625` scale,
  i.e. `v as i32 as f32 * 16.0`); the second endpoint is `14.0` below the first
  on Y.
- **Lift height clamp with sound and re-aim (1 body).** `0x806DFDB0` clamps
  `+0xB0` to `+0x624 +/- +0x648` (the sign is chosen by `+0x658`), raises
  `+0x6A9 = 2` and switches the interface to `0x8082DF68`; otherwise it spawns
  sound `0x2C0`/`0x2C1` and, when the virtual `+0xE4` answered `2`, re-aims
  through [`fn_806dfc40`] with the negated `+0x6AA` angle.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80762010` | `fn_80762010` | 404 | medium |
| `0x80779CA0` | `fn_80779ca0` | 404 | high |
| `0x8078E9A0` | `fn_8078e9a0` | 404 | high |
| `0x8064DCC0` | `fn_8064dcc0` | 408 | medium |
| `0x806DFDB0` | `fn_806dfdb0` | 408 | high |

## Deferred

None.

## Corrections and notes

- All five exports carry `unaff_GQR0`/`ldexpf` guards. In every body the only
  GQR-0 accesses are plain scalar `psq_st`/`psq_l` of `f29..f31`, so the guards
  are compiler noise and are not emitted.
- `0x80762010` and `0x8064DCC0` keep their float locals in undefined stack
  slots; the export prints them as Ghidra `double` temporaries. The raw words
  are `lfs`/`stfs`, so the reconstruction keeps `f32` throughout.
- `0x8064DCC0`'s `0x43300000` + `xoris rX,rX,0x8000` sequence is the plain
  signed-`int`-to-`f64` idiom: the subtraction constant at `0x807C86E0` is
  `0x4330000080000000` = `2^52 + 2^31` (not the `2^52` value used elsewhere),
  so `(v as i32) as f32` is exact. Ghidra's `CONCAT44(...) ^ 0x80000000`
  rendering is the same operation.
- `0x806DFDB0`'s two float window tests fold `fcmpo` + `cror` exactly as Ghidra
  prints them (`sum <= height` for variant `0`, `height <= diff` otherwise);
  the `fcmpo`/`cror`/`bc 4,2` triples at `0x806DFE0C..0x806DFE1C` and
  `0x806DFE30..0x806DFE3C` agree.
- `0x806DFDB0` builds two different stack vectors for the sound call (16 bytes
  for variant `0`, 8 bytes for variant `1`); both are kept as written.
- `0x806DFDB0` takes its first call (`0x8006CD40`) with no recoverable argument
  in the export; the existing [`fn_806b2010`] calls it the same way
  (`platform::call0`), so the `r3 = this` value is not forwarded.
- `0x80779CA0`'s `0x802DCD6C` save thunk hides the real `r3` receiver; the
  parameter is recovered as `this`, exactly as in the existing
  [`fn_80760070`].
- `0x8078E9A0` state `3` waits with `cmpi r4,0` + `bc 4,1`, i.e. `<= 0`
  (`BI=1` is `CR0[GT]`), which matches Ghidra's `< 1`.

## Verification

- The lane-D block type-checks standalone against stubbed helper signatures
  (temporary `cargo check` harness with the real helper ABIs); the harness lives
  outside the repo.
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was re-read from `work/rels/elf/d_basesNP.elf`.
- Only helpers that already exist in `src/game/bases.rs` are called
  ([`fn_8078e680`], [`fn_8078e690`], [`fn_8079bd40`], [`fn_806dfc40`],
  `call_f32_1`, `call_f32_u32`, `call_f32x2_u32`, `call_f32x3_u32`,
  `vmethod0`, `vmethod1`); `0x8079BC10`, `0x8079BCE0`, `0x8010_5240`,
  `0x802E_17DC`, `0x800D_F180`, `0x8016_*` and `0x8019_9390` have no Rust body
  and go through the `abi_callN!` macros.
