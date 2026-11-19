# Coverage - Wave 182 lane D (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 876 | 9 | 1 |

Sub-300-byte sweep of the `0x8066_9040..0x8067_B380` band of REL `d_basesNP`.
Source of the bodies: `work/wave182/laneD_block.rs`. Every address is the REL
analysis address with the synthetic base `0x8060_0000`; these are not runtime
addresses, and cross-module words (`0x8042_xxxx`) are `main.dol` addresses kept
as they appear in the instruction stream.

## What this pass covers

- **Deleting destructors (4 bodies).** `0x80669040` is byte-identical to the
  already-translated [`fn_80614470`] (`0x8015_F9F0(this + 0x18, 0)` then
  `0x802B_9280` for `free > 0`). `0x806691D0`, `0x80674840` and `0x8067ACD0`
  are the same shape with a different teardown entry (`0x800D_AE00` and
  `0x8015_F800` respectively); the last two are byte-identical twins, kept as
  two bodies because each class registers its own destructor.
- **Countdown + effect (1 body).** `0x80675040` decrements the signed counter
  at `+0x4D4` and, when it hits zero, falls through into a tail branch to
  `0x800E20A0(*(0x8042A050), this, 5, 16.0f, 32.0f, (s8)*(u8*)(this + 0x38D))`.
  Ghidra truncates this body at `halt_baddata` after the two `bclr` returns;
  the arm was read from the raw words inside the declared 68-byte extent. The
  same class as the adjacent [`fn_80675090`] (both drive `+0x38D`).
- **Interval rebuild (1 body).** `0x80678400` runs `0x8067_8300(this)` and then
  writes the symmetric pair `(-16*s - 4, 4 + 16*s)` through `0x800D_B2B0` into
  `this + 0x4CC`, with `s = *(f32*)(this + 0xDC)`.
- **Sound + release (1 body).** `0x8067A900` calls [`fn_8067a790`], converts
  `this + 0xAC` with `0x8006_A4B0`, queues sound `0x51E` through `0x8019_8C30`
  with the manager at `0x80429D60`, and then `0x8016_2510(this)`.
- **Position snapshot + virtual forward (1 body).** `0x8067B160` copies
  `+0xAC..+0xB4` into `+0x3F0..+0x3F8`, stores `120.0` at `+0x3FC`, and
  tail-calls slot `+0xC` of the vtable held at `+0x3EC` with `this + 0x3EC`
  as the callee receiver and the three floats in `f1..f3`.
- **Hit report (1 body).** `0x8067B380` spawns the `"Wm_mr_fireball_hit"`
  effect (`0x807E_C140`) through `0x8008_F9F0` and sets `+0x40C = 1`.
- **Numeric leaf (1 body).** `0x80673600` returns
  `(i16)(-(i32)(x * (x - 176.0f)))` for `x = *(f32*)(this + 0xE8)`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80669040` | `fn_80669040` | 100 | high |
| `0x806691D0` | `fn_806691d0` | 88 | high |
| `0x80673600` | `fn_80673600` | 76 | medium |
| `0x80674840` | `fn_80674840` | 88 | high |
| `0x80675040` | `fn_80675040` | 68 | high |
| `0x80678400` | `fn_80678400` | 100 | high |
| `0x8067A900` | `fn_8067a900` | 100 | high |
| `0x8067ACD0` | `fn_8067acd0` | 88 | high |
| `0x8067B160` | `fn_8067b160` | 92 | high |
| `0x8067B380` | `fn_8067b380` | 76 | high |

## Deferred

None. All ten bodies in the lane were translated.

## Notes and residual risk

- `0x80673600` is the only medium: Ghidra renders the operand as
  `4503601774856448.0 - DAT_807c91c8`, but the raw stream stores the
  `lfs f0,0xE8(r3)` bits as the **high** word of a stack double and then loads
  it back with `lfd` before a *single-precision* `fsubs`. Single-precision
  arithmetic uses the high word, which is `x`, and the high word of
  `0x807C_91C8` (`0x43300000_80000000`) is `176.0f`; the `li 2304`/`xoris`
  low word and the dead `lis r0,0x4330` never reach the result. `fmuls`,
  `fctiwz`, `neg r0,r0` and `extsh r3,r0` were re-decoded from the raw words
  (`0x7C0000D0`, `0x7C030734`), because Ghidra's `(int)(short)-(short)(int)`
  spelling is ambiguous about the destination register.
- `0x80675040` and `0x8067B160`: Ghidra's decompiler mistrusts both tails
  (`halt_baddata` after the `bclr`s, `bcctr` for the virtual). The tails were
  decoded instruction by instruction, and the dead `stfs f1..f3,8(r1)` spills
  in `0x8067B160` are noted in the body's doc block.
- `0x8067A900` uses a 16-byte scratch block (Ghidra's `auStack_18[16]`); the
  sibling [`fn_806b2930`] sizes the same sample as 12 bytes, so the declared
  extent is kept and flagged in the doc block.
- `0x800E20A0` is called with `lfs` floats here but with `lfd` doubles from
  `0x8042CC88`/`0x8042CC8C` in [`fn_8066cb40`] and [`fn_8068bc40`]; the
  two-float/four-integer helper is used for this site, the `f64` form for
  those, each following its own load width.
- `0x8067_8300` has no Rust body in the module, so it is called by address.
