# Coverage - wave 190, lane E

Module: `game::bases` (REL `d_basesNP`, synthetic base `0x80600000`).
Band: 128-192 bytes. All ten assigned functions are translated; nothing deferred.

## Bodies

| Address | Size | Name | Summary |
| --- | --- | --- | --- |
| `0x8079BB60` | 168 | `fn_8079bb60` | Samples a route node's position through `0x80100200(out, scene, node)` and adds the `15.0` lift constant `0x807D0164` to Y when `0x800F57A0(record, node, 0x800)` hits. |
| `0x8079BD50` | 148 | `fn_8079bd50` | Unrolled three-way mirror of `*(0x80429F7C)+0x388/+0x38C/+0x390` into `0x80832EB8[index[i]*4]` using the `0x80354E64/68/6C` slot indices; writes `3` or `0`. |
| `0x8079C230` | 144 | `fn_8079c230` | Copy-initialiser: `+0x2E0 = 0x12`, position `+0xAC..+0xB4`, three `s16` angles `+0x100..+0x104`, then re-samples `*(+0x22C)` into `+0x1FC..+0x204`. |
| `0x8079EE70` | 140 | `fn_8079ee70` | `handle = 0x80100AA0(this)`, `0x80103390(effectMgr, handle, first, 0, 0)`, then `0x80105060(0.0, kind-8 manager, 8, second, third)`. |
| `0x807A7C40` | 164 | `fn_807a7c40` | Builds the `cameraA` name (8 bytes), patches byte 6 with the `extsb` of `+0x1B8`, hands `+0x1C0` to `0x801006E0`, and arms camera fields `+0x604/+0x5F4/+0x5F0/+0x624/+0x608`. |
| `0x807AB3A0` | 152 | `fn_807ab3a0` | Same `clz(result-1)>>5` mode gate as `fn_807ab010`; selects `0x80833620` when `+0x108 == 0`, `0x808336E0` otherwise, pushed through view `+0x74` slot `+0x18`. |
| `0x807AB9D0` | 132 | `fn_807ab9d0` | If `0x800E0450(0x8042A040) == 0`, consumes `*(0x80429FB8)+0x78`: non-zero stores `+0xB8F = 1` / `+0xB50 = choice` in `0x8042A2D8`, zero pushes `0x80833820`. |
| `0x807ABD60` | 188 | `fn_807abd60` | Binds `Env_world` `light/Layout3D.blight`, `.blmap`, `fog/Layout3D.bfog` from archive `+4` and hands each to slot `+0x08` of the world, its `+0x14` lightmap child and the fog manager. |
| `0x807ABE20` | 188 | `fn_807abe20` | Resolves `MainSelect` out of `scene/scene.brres`, pairs it with `0x802426D0`, starts the world through `0x802C8840(0.0, world, &entry, -1, 3)`, and recurses into `fn_807abd60` when the course argument is `1`. |
| `0x807ADA10` | 188 | `fn_807ada10` | Instruction-for-instruction twin of `fn_807abd60` with the string block at `0x8081E000` (`Env_world` `0x8081E0A4`, blight/ blmap/ bfog at `+0x60/+0x78/+0x90`). |

## Reused neighbours

- `fn_807ab010` (`0x807AB010`) pins the `0x8042A440` vtable slot `+0x10` call
  and the `clz(x - 1) >> 5` equality idiom; `fn_807ab3a0` is the same skeleton
  plus the `+0x108` descriptor select, and calls `vmethod1(screen + 0x74, 0x18,
  ...)` exactly as that neighbour does.
- `fn_807ac320` (`0x807AC320`) already registers `0x80833620` and `0x80833820`
  as `dScGameSetup_c` state descriptors, which confirms the two addresses pushed
  by `fn_807ab3a0`/`fn_807ab9d0`.
- `fn_807ac260`/`fn_807ac270`/`fn_807ac280` exist in `bases.rs` but their
  signatures take only `this`; the raw bodies are
  `lwz r12,0(r3); lwz r12,8(r12); mtctr; bctr`, i.e. the caller's `r4` (the
  BRRES pointer) is still live. `fn_807abd60`/`fn_807ada10` therefore use
  `vmethod1(obj, 8, resource)` rather than the one-argument helpers.
- `0x80100200` (scene position into a float triad) and `0x800F57A0` (record
  lookup with a mask) are already used with the same argument shapes, e.g.
  `abi_call3!(0x8010_0200, out, scene, node)` and
  `abi_call3!(0x800F_57A0, record, node, 0x800)` in `bases.rs`.

## Notes / inferred

- `fn_8079bb60`'s incoming `r4` is dead: `0x8079BB8C` reloads it with the scene
  pointer `0x8042A18C` before any use. The live arguments are `r3 = out` and
  `r5 = node`, so the third parameter is kept and named `_unused` to preserve
  the ABI slot.
- `fn_8079ee70`'s incoming `r3` *is* live: it is still in `r3` at the
  `bl 0x80100AA0`, so the routine forwards its receiver to that handle getter.
  `0x807D0160` is `0.0f32`; like the other translated `0x80105060` sites the
  scalar is passed as the promoted `f64` (`call_f64_u32x4`).
- `fn_807a7c40`'s `0x802E16E8` is the varargs name formatter; the shipped call
  passes only `(buffer, 8, "cameraA")`, and the `extsb` of `+0x1B8` is added to
  byte 6, turning `cameraA` into `cameraB`/`cameraC`/...
- `fn_807abd60` and `fn_807ada10` both read their `course` argument from `r4`
  while `r3` is never touched, which is why the first parameter is `_unused`.
  `fn_807abe20` calls `fn_807abd60` with `cmpi r28,1` (unsigned compare of the
  `r4` argument), confirming that the second slot is the live one.
- `0x802DCD6C`/`0x802DCD74`/`0x802DCDB8`/`0x802DCDC0` in the exports are the
  save/restore compiler helpers and are intentionally not emitted.
- `0x807D0738`, the `0x802C8840` scalar, reads as `0.0f32` in the shipped REL.

## Deferred

None.
