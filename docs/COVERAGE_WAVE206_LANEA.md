# Coverage - Wave 206 lane A (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1488 | 10 | 0 |

Addresses `0x8003_01B0`-`0x8003_10C0`, the ten bodies that continue the
`0x80030xxx` block that wave 205 lane F opened at `0x8002_EF60..0x8003_0140`.
Lane A covers `0x8003_01B0` through `0x8003_10C0`; wave 206 lane B takes over
at `0x8003_1180`, so there is no overlap with lanes B-F.

## What landed

- **Freeze latch.** `0x800301B0` binds `"freeze"` through
  `0x80030140(0.0f, this, "freeze", 1)`, zeroes the `+0x584` holder scale via
  `0x801680E0`, then latches the sign of `+0x102` into `+0x348` and reloads
  `+0x102` from `0x8042C1A4` (`{8192, -8192}`), i.e. the angle halfword keeps
  only its sign.
- **Death effect stamp.** `0x80030310` builds the effect point
  `(+0xAC + +0xD0, +0xB0 + +0xD4, 5500.0)`, hands it to `0x800943D0` for mode
  `0`/`1`, posts the death-sound table entry `0x80357FD0[+0x154]` through
  `0x800A7C60`, advances that counter with `0x800A7C00`, calls
  `0x8009F6F0(+0x510, +0x154)` for its side effect only, and then stamps the
  static block `0x802EE5E8` onto `+0x414..+0x432`.
- **Death reply.** `0x80030440` binds `"dead"` at blend `1.0` with loop flag
  `1`, runs slots `+0x98` and `+0x2B0` of the `+0x60` core, and picks
  `0x800E20A0` (type `0..3`) or `0x800E23C0` (type `0x32`) for the contact
  effect before latching `+0x360 = 1`.
- **Sound post.** `0x80030540` is the minimal `0x8006A4B0` projection of
  `+0xAC` followed by `0x80197F00(*(r13 - 0x5924), 0x1A8, record, 0)`.
- **Ice death.** `0x80030680` binds `"dead_ice"`, runs slot `+0x98`, copies
  `+0x414`/`+0x418`/`+0x41C` into `+0xE8`/`+0xEC`/`+0xF8`, zeroes
  `+0xF0`/`+0xF4`/`+0xFC`/`+0x11C`, copies `+0x430` to `+0x459`, copies
  `+0x420` to `+0x114`, repeats the `+0x102` latch, spawns the `0x800E20A0`
  effect for types `0..3`, and finally writes `+0xB4` as `-384.0` when
  `+0x391 == 1` and `4500.0` otherwise.
- **Frame helpers.** `0x80030910` is the two-call pair `fn_8002f3f0(this)` +
  `fn_8002f540(this)`, both already translated in this file by wave 205
  lane F.
- **Record release.** `0x80030E90` is the same shape as `fn_80015e80`:
  `0x8015F800(this, 0)` for a non-null pointer, `0x802B9280` only for a
  strictly positive flag, and `this` returned in every case.
- **Singleton address.** `0x80030EF0` returns `0x80357D74`, the object the
  death stamp writes into `+0x424`.
- **Child-chain walk.** `0x80030FE0` tests `*(node + 0x30)` through slot
  `+0x0C` up to five times, returns the parent on the first non-zero answer and
  otherwise recurses on the fifth accepted child.
- **Name-tail compare.** `0x800310C0` is a byte-for-byte duplicate of
  `0x8002EE30`: `0x802E1C40` finds the first `':'` in the key and in virtual
  slot `+0x20`, and `0x802E1AB4` compares the two tails.

## Corrections and recoveries

- **All ten sizes match `functions.tsv`.** The terminal `bclr`s are at
  `0x8003_0228`, `0x8003_043C`, `0x8003_04F4`, `0x8003_0588`, `0x8003_0798`,
  `0x8003_0940`, `0x8003_0EE8`, `0x8003_0EF8`, `0x8003_10BC` and
  `0x8003_1144`, i.e. exactly 124, 304, 184, 76, 284, 52, 92, 12, 224 and 136
  bytes. No prologue-artefact correction was needed for this lane.
- **`0x80030310`'s `+0x428` is not the `0x8009F6F0` result.** Ghidra folds
  `uVar7 = FUN_8009f6f0(...)` into the store at `+0x428`. The disassembly
  reloads `r3` with `lfs r3,0x18(r1)` - the block word at `0x802EE5E8 + 0x00`
  - at `0x8003_03D4`, before the `stw r3,0x428(r31)` at `0x8003_0404`, so the
  result is discarded and `+0x428` receives the block's `0`. The `+0x428 >= 0`
  guards in `0x80030440`/`0x80030680` and the `-1` value lane E's sibling
  block (`0x802EE538`) carries at the position `0x8002DE80` stores into
  `+0x428` both fit a signed "kind" index that the death stamp resets to `0`.
- **`0x80030680`'s `+0x274` is the word form of `+0x41C`.** Ghidra prints
  `+0x274 = 0`; the disassembly is `lfs f3,0x41C(r31)` then
  `stw r3,0x274(r31)`, so the field receives `0xC0800000` (`-4.0f`). The body
  stores it as a raw word, as reconstructed.
- **`+0x41C`/`+0x420` do not come from the static block.** Both death stamps
  load them from SDA2 (`r2 - 0x6DA8` = `-4.0f`, `r2 - 0x6DB0` = `-0.24375f`),
  while `+0x414`/`+0x418` come from block words `0`/`1`. The same
  `-4.0`/`-0.24375` pair appears in wave 206 lane B's `0x80031B70` hit record,
  so the values are shared constants and not per-class ones.
- **The effect calls pass the kind and type words in `r5`/`r6`.** Both
  `0x80030440` and `0x80030680` reach `0x800E20A0` with `r5 = *(+0x428)` and
  `r6 = extsb(*(+0x431))`, matching the six-argument form already documented
  for `fn_8002fa50`. `0x800E23C0` takes only `f1`, `f2`, `r3`, `r4`, so lane A
  adds one four-argument wrapper (`w206a_call_f64x2_u32x2`) because the
  existing `w205f_*` wrappers cover one double or two doubles plus four words.
- **`0x80030FE0`'s failure return is the parent.** After the fifth successful
  test the original recurses on the child it just accepted while `r31` still
  holds that child's parent, so the loop keeps `node` at the parent of the
  tested child instead of advancing on the last iteration. Both `return`
  paths (`0x8003_10A0`, `0x8003_10A8`) load `r31`, confirming this.
- **`0x800310C0` is a duplicate, not a new shape.** It is instruction for
  instruction `fn_8002EE30`, including the asymmetry that the key pointer only
  advances when `0x802E1C40` found a `':'` while the name tail advances
  unconditionally. The body is written out rather than delegated so the two
  addresses stay independently checkable.
- **Line-1 context.** `0x800301B0`'s `f1` argument is an `lfs` of SDA2 `0.0f`
  handed to `fn_80030140`, which copies `f1` into `f31` (`stfd`/`fmr`) and
  treats it as a double. The call is reconstructed as
  `read_f32(...) as f64`, so the blend stays `0.0`; the upper half of `f1` is
  not definable from the call site and does not affect `0.0`.
