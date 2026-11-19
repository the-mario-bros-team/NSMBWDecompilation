# Coverage - Wave 172 lane D (`game::bases` 344-348 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 5 | 1,736 | 5 | 0 |

Fourth batch from the 300-512 byte band; all five bodies are in the 344-348 byte
sub-band. Source of the bodies: `work/wave172/laneD_block.rs`.

## What this pass covers

- **Pose re-apply (1 body).** `0x8077E410` looks up the `0xC`-stride record at
  `*(this+0x294) + index*0xC`, early-outs when the resolved mode still matches
  `+0x1F0` *and* `record[+0x29C] == 0x801680C0(model+0x28)`, otherwise stores
  the mode and drives the model's virtual `+0x5C` (two record floats plus `0.0`)
  and `+0x84` (the frame value). `index == 0` clears the mode when `+0x2F8` is
  set and re-derives the frame value from `model[+0x50] - 1.0f`.
- **`BGM_anim_dummy` direction switch, `+0x5BC` (1 body).** `0x8064F390` waits
  for the `+0x584` clip to reach `(+0x5AC - 1.0f) - 10.0f`, rebinds the
  `+0x5F8` clip into the `+0x5BC` mover, stores `(+0x5E4 - 1.0f) / 0x806505B0()`
  into `+0x5FC`, latches `+0x3F4`/`+0x3F5` on the `0x808283E8` record and
  dispatches the `0xD4` callback with `0x808282B8`.
- **`BGM_anim_dummy` direction switch, `+0x584` (1 body).** `0x8064F6A0` is the
  mirror image: gate on `+0x5E4`/`+0x5BC`, clip `+0x5F4` into `+0x584`, speed
  from `+0x5AC`, *clears* `+0x3F4`/`+0x3F5` and dispatches `0x80828278`.
- **Player-count frame rebuild (1 body).** `0x80675090` seeds `+0x38D` to
  `0xFF`, then either copies `0x8008F070`'s byte or clamps `0x800B0FD0`'s lane
  id to `0..3`; for lanes `<= 3` it converts the player's `+0x14D4` count `+ 1`
  to `f32` through the `2^52` magic and re-drives `+0x46C`.
- **Unused-name picker (1 body).** `0x80772BD0` re-rolls
  `0x800B2EE0(count)` from the scene's `0x16518`-stride table until the id
  survives the `0x800F9970` verdict, the `0x800F9530`/`0x800F9840` kind test,
  `*(0x80429F7C)+0x44`, the caller's `excluded` id and a full `0x28F`-actor
  walk over vtable slots `0x70`/`0x74`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8077E410` | `fn_8077e410` | 344 | high |
| `0x8064F390` | `fn_8064f390` | 348 | high |
| `0x8064F6A0` | `fn_8064f6a0` | 348 | high |
| `0x80675090` | `fn_80675090` | 348 | high |
| `0x80772BD0` | `fn_80772bd0` | 348 | high |

## Deferred

None.

## Corrections and notes

- `0x8077E410`'s first argument is **single precision**. Every caller puts it in
  `f1` with `lfs` (`0x8077E584` loads `0x807CF698`, `0x8077C6D0` the same), and
  the body only ever forwards it with `fmr`. Ghidra renders it as a `double`
  because of the pairing in `fmr`, so the existing
  `fn_8077e570`/`fn_8077e5e0` translations call it through `call_f64_u32x3`.
  The lane-D body takes `f32`; the call site is harmless because the argument is
  `0.0`, but it should switch to a single-precision helper when the coordinator
  next touches those two bodies.
- `0x8064F390`/`0x8064F6A0` pass the freshly computed quotient to `0x801680E0`
  in `f1`; Ghidra's export prints no argument at all. The lane-D bodies use
  `call_f32_u32` so the value is not dropped. `fn_8064eeb0` (wave 171) uses
  `abi_call1!` at the same call site and therefore loses it.
- The gate in both direction switches is two rounded subtractions,
  `(length - 1.0f) - 10.0f`, not a folded `- 11.0f`; the raw words emit
  `fsubs` into `f2` and then `fsubs` into `f0`. `fcmpo` + `cror 2,1,2` + `bne`
  skips on `current < limit`, so an unordered pair also skips.
- `fn_8064eeb0`'s mode mapping is confirmed by the two new bodies:
  mode `2` <-> `+0x584` <-> `0x80828278` <-> `+0x3F4`/clip `+0x5F4`,
  mode `3` <-> `+0x5BC` <-> `0x808282B8` <-> `+0x3F5`/clip `+0x5F8`.
  `0x8064F390` therefore ends in mode `3` and `0x8064F6A0` in mode `2`.
- `0x80675090` keeps the `extsb` in front of `0x8005F900`; the lane byte is
  sign-extended, while the `0x800B1100` index is masked to `0xFFFF`.
- `0x8077E410`'s `cmpwi r5,2` at `0x8077E474` is a real dead compare: nothing
  consumes `cr0` before the next instruction overwrites it, so the body only
  branches on `mode == -1`.
- `0x80772BD0` returns `r26` in `r3` across the `0x802DCDB4` restore helper,
  which preserves it; the export types the function `void`.
- The `r3` chain `0x800F56B0 -> 0x800F9530 -> 0x800F9840` in `0x80772BD0` is
  preserved explicitly; `0x800F9970` is called with `call0_ret` like the
  existing `fn_80799750`.

## Verification

- The lane-D block type-checks standalone against stubbed helper signatures
  (`rustc --emit=metadata`).
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was read from the raw words of `work/rels/elf/d_basesNP.elf`.
- `node tools/check-lane-csv.mjs wave172` reports only the expected pre-merge
  "no body in `src/game/bases.rs`" entries for the three addresses that do not
  yet appear in the merged file.
