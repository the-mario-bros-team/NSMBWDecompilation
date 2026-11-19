# Coverage - Wave 188 (`game::bases` 128-192 band, part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 7,700 | 40 | 10 |

Second wave in the 128-192 band.

## Highlights

- **Name-suffix comparators (8).** `0x80673C80`, `0x8067C570`, `0x8067D3D0`,
  `0x80699190`, `0x806A5FC0`, `0x806A95B0`, `0x806AB560` and `0x806628D0`'s
  five siblings are the same `0x802E1C40`/`0x802E1AB4` comparator after the
  first `':'`.
- **Gate bodies (2).** `0x80697D00` and `0x80699C00` walk the `0x12B` list and
  forward `this` into `0x80697DC0`/`0x80699CC0`. Their callers `fn_80697f10`
  and `fn_80699e30` now call the real functions instead of `call0_ret`.
- **Ring/segment math (4).** `0x806E3EB0` rewrites four `0x60`-stride ring
  segments from `+0x104`; `0x806D4D30`/`0x806D7D60` are the `+0x58C`/`+0xB0`
  twins; `0x806EC380` gates speed inside `+/-16384.0`.
- **Actor/effect bodies (6).** `0x8067ECC0` (variant kill), `0x806860E0`,
  `0x80686350`, `0x806947C0`, `0x806954C0`, `0x806E7AA0`.
- **The rest** are state entries, transforms and small helpers.

## Corrections and recoveries

- `0x80686350`'s Ghidra `double` is really `f1` from `lfs` (proved from caller
  `0x80685090`); `0x806954C0`'s `ldexpf` noise is plain `psq_l`/`psq_st` f32
  frame traffic.
- `0x806E3EB0` and `0x806ED260` take `this` in `r3`; their `_savegpr` prologues
  hide it.
- `0x80662F40`'s `xoris`/`0x43300000` block is dead (no `lfd` reads it), so the
  player count stays `int`.
- `0x806ABCD0`'s last slot takes only `this + 0x394`; the stale `r4` is
  call-clobbered.
- `0x80696EA0`'s two parameter stores are `stfs`, not `undefined4`.
- `0x806E7F50` now has a real body and is called directly by `fn_806e7ff0`.

## Medium rows (10)

Mostly the `0x806ABCD0`/`0x80697D00`/`0x80699C00` gates (callee contracts
inferred) and small state helpers whose callee roles are not established.

## Verification

- `cargo check` succeeds with 4,718 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave188`: 50 rows, 50 unique addresses,
  40 high + 10 medium.
- All 50 evidence exports exist; 7,700 bytes total.
