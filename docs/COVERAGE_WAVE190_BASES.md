# Coverage - Wave 190 (`game::bases` 128-192 band, part 4)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 8,144 | 48 | 2 |

Fourth wave in the 128-192 band. **Overall coverage crossed 90%.**

## Highlights

- **`daWmSandPillar_c` family (10).** `0x80776C80`, `0x80777BA0`, `0x80777D20`,
  `0x80778100`, `0x80778200`, `0x807787E0`, `0x80778880`, `0x807789A0`,
  `0x80779770`, `0x80779E40`.
- **Ship/route bodies (6).** `0x8076FB00`, `0x8076FBA0`, `0x8076FDA0`
  (`shipRoute` stop list, now called directly by `fn_806fc60`), `0x8076D140`,
  `0x8076D1F0`, `0x80774FA0`, plus `0x8079BB60`/`0x8079B5B0`/`0x8079B660`.
- **World/scene bodies (5).** `0x807ABD60` and `0x807ADA10` (instruction-for-
  instruction twins binding `Env_world` blight/blmap/bfog), `0x807ABE20`
  (`MainSelect` starter), `0x807AB3A0`, `0x807AB9D0`.
- **Transform twins.** `0x8078FAD0` is character-for-character `fn_8075aa50`
  with the animation object at `+0x1A4`; `0x80776390` is `fn_8076d7f0` with
  `+0x1A0`.
- **The rest** are node constructors, route samplers and small state helpers.

## Corrections and recoveries

- `0x80776C80` has no integer `param_4/param_5`: three `fmr`s prove `f1/f2/f3`
  are the args, part object at `this + part*0x38 + 0x1E4`.
- `0x80777D20`/`0x80779770`/`0x80779E40`: the `0x08/0x0C/0x10` stack slots are
  the lost `0x8016EF50` angle pointers and `0x1C/0x20/0x24` the lost
  `0x801C0BD0` args.
- `0x807787E0`'s `fcmpo`+`cror`+`bc 4,2` folds to `height <= limit`.
- `0x8078DAFC` `beq` and `0x8078F114` `bc 4,1` (which is `<= 0`) both invert
  Ghidra's naive reading and were checked against raw words.
- `0x8079BB60`'s `r4` is dead (reloaded with the scene pointer); `r3` is dead in
  `0x807ABD60`/`0x807ABE20`/`0x807ADA10` (named `_unused`).
- `0x807AC260/270/280` drop the resource argument in their existing Rust
  signatures, so binding uses `vmethod1(obj, 8, resource)`.
- `0x8076D140`'s `+0x84` call needs a float-arg virtual (`call_f32_f32`).

## Verification

- `cargo check` succeeds with 4,818 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave190`: 50 rows, 50 unique addresses,
  48 high + 2 medium.
- All 50 evidence exports exist; 8,144 bytes total.
