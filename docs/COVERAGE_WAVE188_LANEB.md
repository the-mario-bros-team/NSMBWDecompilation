# Coverage - Wave 188 lane B (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 10 | 1,516 | 7 | 3 | 0 |

Ten bodies, all from this lane's 128-192 byte band. Source of the bodies:
`work/wave188/laneB_block.rs`; per-function ledger:
`ledger/agents/wave188-laneb.csv`.

## What this pass covers

- **Two class-update tails (2 bodies).** `0x80696EA0` (176) runs the shared
  `0x8006CDE0`/`0x8006CD40` prelude, calls the already translated probe
  [`fn_80696e00`] and, when `+0x443` is set and `+0x442 != 0xFF`, converts
  `+0x430`/`-+0x434` to 16-bit screen coordinates for `0x800777B0` and applies
  the `0x8082B3A0` block through the `+0x394` slot `+0x18`. `0x80696F70` (132)
  is the same prelude plus a `+0x102` spin of `0x2000` and a `+0xEC <= 0.0`
  window that snaps `+0xB4` to `9000.0`, clears `+0xEC` and applies
  `0x8082B3E0`. Both are the same `+0x394`/`+0x18` parameter-block dispatch as
  `fn_80705080`.
- **Two proximity gates (2 bodies).** `0x80697D00` (184) and `0x80699C00` (184)
  walk the `0x12B` (299) actor chain, build `[x, y - radius, z]` from each
  candidate's `+0xAC..+0xB4` and its own `+0x314` radius, and forward
  `(this, point, candidate + 0xE8)` to `0x80697DC0` / `0x80699CC0`. They are the
  gates already called by the translated `fn_80697f10` and `fn_80699e30`; see
  the wiring note below.
- **One two-stage gate (1 body).** `0x80699B60` (160) walks the same chain and
  returns `1` for the first candidate that fails the [`fn_8069a6e0`] proximity
  test and passes [`fn_806a3d60`].
- **Four `':'`-suffix name comparators (4 bodies).** `0x80699190`, `0x806A5FC0`,
  `0x806A95B0` and `0x806AB560` (136 each) are instruction-for-instruction the
  same body as the already translated `fn_806b6cc0`: `0x802E1C40` finds the
  first `':'` in each name, `0x802E1AB4` compares the two suffixes. Only the
  `bl` displacements differ between them.
- **One state-entry method (1 body).** `0x806ABCD0` (136) dispatches the `+0x60`
  vtable slots `+0x2A4` (stop) and `+0x2A8` (reset), installs the `0x8082BD04`
  `+0xD4` callback, runs the `+0x394` slot `+0x1C` and writes `1` to `+0x360`.

## Signature notes for the two gates (caller wiring)

The callers `fn_80697f10` and `fn_80699e30` currently emit
`call0_ret(0x8069_7D00)` / `call0_ret(0x8069_9C00)`, which assume no argument.
The raw words show the gates take `this` in `r3` and pass it straight to
`0x80697DC0`/`0x80699CC0` as the query object (which then reads its own
position at `+0xE8`/`+0xEC`/`+0xF0`):

- `fn_80697d00(this: usize) -> u32`
- `fn_80699c00(this: usize) -> u32`

At both call sites `r3` still holds the caller's `param_1` (`mr r30,r3` saves
it and nothing reloads `r3` before the `bl`), so the correct wiring is
`fn_80697d00(this)` / `fn_80699c00(this)`. The existing `call0_ret` form leaves
`r3` undefined by contract even though it happens to work in place.

## Notes

- `0x8006CDE0` is printed without an argument by Ghidra in both `0x80696EA0`
  and `0x80696F70`; the raw words (`bl` with `r3 == param_1`, `mr r31,r3`
  before it) restore the `this` argument, matching `fn_8068d510`.
- `0x807C9F90 = 0.0` and `0x807C9FA4 = 9000.0` were read from the REL.
  `0x80696F70`'s two `stfs` (`0x80696FC4`, `0x80696FCC`) are what Ghidra printed
  as `undefined4` stores, and the `+0x102` step is `lha`/`sth`, i.e. a
  sign-extended halfword add emitted as `read_u16().wrapping_add(0x2000)`.
- `0x80697DC0` and `0x80699CC0` are still untranslated, so they are called by
  address. Their export prototypes are `(int, float *, float *)`, which matches
  the point-pointer call shape used by `fn_806a4110`.
- `0x806ABCD0`'s fourth call is `vmethod0(this + 0x394, 0x1C)`: `r4` still holds
  the `0x8082BD04` left over from the preceding `+0xD4` call at the `bcctr`, but
  it is call-clobbered and therefore not a real second argument.
- Constants used: `0x8082B3A0`, `0x8082B3E0`, `0x8082BD04` (descriptor blocks),
  `0x80429DD0` (effect manager pointer), list id `299` (`0x12B`).

## Verification

- `work/wave188/laneB_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- Every body's raw words were re-read from `work/rels/elf/d_basesNP.elf`
  (`ppc159.js`) and each call target, offset and constant was checked against
  the export.
- All ten evidence exports exist under `work/ghidra/rel-d_basesNP/export/`.
- `node tools/check-lane-csv.mjs wave188` sees the 10 rows; the "no body in
  src/game/bases.rs" lines it prints are the expected pre-merge state, since the
  coordinator appends the blocks.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
