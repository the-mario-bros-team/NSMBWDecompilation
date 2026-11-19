# Coverage - wave 190 lane A (`game::bases`, `0x8076D140..0x80776390`)

Lane A of wave 190 translated ten bodies from `rel/d_basesNP.elf` (bucket
`0x80600000`, synthetic analysis base; not a runtime address). Nine are
`translated` at `high` confidence and one (`0x8076FDA0`) at `medium`. Nothing
was deferred.

Address span: `0x8076D140..0x80776440`. Block:
`work/wave190/laneA_block.rs`. Ledger rows:
`ledger/agents/wave190-lanea.csv`.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | --- | --- | --- | --- |
| `0x8076D140` | 176 | `fn_8076d140` | In the `2..=9` state window, calls the sub-object's `+0x84` virtual with `0.0f` when `0x80168080(obj+0x28) == *(f32*)(obj+0x50) - 1.0`; wraps `+0x1AC` at `1000`. | high |
| `0x8076D1F0` | 132 | `fn_8076d1f0` | Re-submits a sound/effect handle: `0x801006E0` position lookup, then `0x800F69F0(f1=rec+4, f2/f3 = 1.9 * rec[+0xC]/[+0x10], this, pos, 0, (i16)rec+8, 0x80377DD4)`; clears `+0x1B4`. | high |
| `0x8076D7F0` | 176 | `fn_8076d7f0` | Halfword angle vectors `+0x100`/`+0x102`/`+0x104` through `0x8016EF50`, then copies matrix `+0x7C` and colour `+0xDC` to the animation object at `+0x1A8`. | high |
| `0x8076E6C0` | 160 | `fn_8076e6c0` | Constructor: base `0x800F5FD0`, vtable `0x80816ED8` at `+0x60`, model `+0x184`, animation `+0x1A0`, effect `+0x1EC`, `0x38`-byte array at `+0x3D8`, vtables `0x803295D8` then `0x807FEF40` at `+0x448`; returns `this`. | high |
| `0x8076FB00` | 152 | `fn_8076fb00` | Sets frame index `+0x102` inside `current < frame <= current + step`, advances it by `+0x494` when unchanged, mirrors it into `+0x108`, then `0x8006CC00`/`0x8006CD40`; returns the accept flag. | high |
| `0x8076FBA0` | 188 | `fn_8076fba0` | Seeds the `(0,0,1)` up vector, `0x800F6410`, stores `0x80770620(this, frames)` into `+0x10C`, `0x10000 / frames` into `+0x494`, and snaps `+0xAC`/`+0xB0` to `+0x1E0` minus `153.0` / plus `166.0`. | high |
| `0x8076FDA0` | 156 | `fn_8076fda0` | When the controller at `0x808327E0` holds more than two entries, appends every `"shipRoute"` record through `0x801004B0`/`0x801006C0`/`0x80106280`, then commits with `0x801062B0`. | medium |
| `0x80774FA0` | 132 | `fn_80774fa0` | Resolves the element at `this + index * 0x3F08 + 0x5C7F4` (`index = +0x338C`), runs `0x800F33C0` twice and returns the record's `+0x4C`, or `-1`. | high |
| `0x807762B0` | 172 | `fn_807762b0` | `0x800690C0(this+0x184, -1, *(0x80377C48), 0, 0x20)`, loads `"g3d_model.brres"` via `0x800DF180`, rebinds it with `0x80239E30` and writes it into `0x8016A3A0(this+0x1A0, ...)`; then `0x800F2A20`/`0x800690E0`. | high |
| `0x80776390` | 176 | `fn_80776390` | Byte-identical to `fn_8076d7f0` except that the animation target is `+0x1A0` instead of `+0x1A8`. | high |

## Caller wiring note (`0x8076FDA0`)

The already-translated `fn_8076fc60` ends with

```rust
abi_call1!(0x8076_FDA0, this);
```

That call is now backed by a real body with the matching exact signature:

```rust
pub unsafe fn fn_8076fda0(_this: usize)
```

The body never reads `r3` (the export is `void FUN_8076fda0(void)`), so the
argument is kept only so the existing call site type-checks; the parameter is
named `_this` to record that it is unused. No change to `fn_8076fc60` is
needed.

## Evidence and decoding notes

- **`0x8076D7F0` / `0x80776390` reuse `fn_8075aa50`.** Both are the same body
  already translated at `0x8075AA50`: three `lfs` of `+0xAC..+0xB4` are the
  position arguments Ghidra drops from `0x801C0BD0`, and the `lha`/`sth`/
  `lhz`/`sth` chain is a two-halfword vector copy that only fills element 0.
  The only difference is `addi r3,r31,424` (`+0x1A8`) versus
  `addi r3,r31,416` (`+0x1A0`) for the three `0x8016A1xx` calls.
- **`0x8076D140` float comparison.** `lfs r0,0xfffff158` is `1.0` and `fc010000`
  is `fcmpu cr0,f1,f0` with `bc 4,2` branching when the values differ, so the
  guard is exact float equality `0x80168080(obj+0x28) == *(f32*)(obj+0x50) -
  1.0`. The reload `lfs r1,0xfffff150` (`0.0`) immediately before `bctrl` is the
  argument of the `+0x84` virtual, so it goes through `call_f32_f32`, not
  `vmethod0`.
- **`0x8076FBA0` hidden constant base.** Ghidra prints `DAT_80816bf0` /
  `DAT_80816bf4` without the base; the raw words load them off
  `lis r3,0x8081; addi r3,r3,27624` (`0x80816BE8`), i.e. `+0x8 = 166.0` and
  `+0xC = 153.0`. The subtraction is `x - 153.0` and the addition is
  `y + 166.0`, matching Ghidra's field assignment order.
- **`0x80774FA0` matches `fn_80772280`.** `addis r3,r3,0x6` plus
  `mulli r0,r0,16136` plus `addi r3,r3,-14348` is exactly
  `this + index * 0x3F08 + 0x5C7F4`, the third inline array of the
  `0x5C7F4`-byte manager. Note that several existing call sites pass only two
  arguments (`abi_call2!(0x8077_4FA0, scene, id)`); the callee's third
  parameter (`r5`) is unused by the body, so those sites stay correct.
- **`0x807762B0` passes a stack slot by address.** `addi r3,r1,12` holds the
  `0x800DF180` result and `0x80239E30` reads that address, so the Rust keeps a
  one-word local and passes `binding.as_mut_ptr()`; the follow-up
  `0x8016A3A0` receives the address of the `0x80239E30` result.
- **`0x8076E6C0` leftover `r4`.** `r4` still holds `0x80816ED8` when
  `0x80069020` is called, exactly as in the translated `fn_8075a6c0`
  (`r4 = 0x80814310`); that precedent passes only the object address, and this
  body does the same.
- **`0x8076FDA0` prologue/epilogue.** `bl 0x802dcd74` and `bl 0x802dcdc0` are
  the `_savegpr`/`_restgpr` compiler helpers and are not emitted. The
  `"shipRoute"` argument is `lwz r4,0x6d44(r31)`, a load of the relocated
  string pointer at `0x80816D44` (not the address itself), which is how the
  existing `fn_8076ff10` also reads it.

## Deferred

None.
