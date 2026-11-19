# Wave 172, lane E (`game::bases`, REL `d_basesNP`)

Four bodies, all translated from raw ELF words plus the Ghidra exports.
Evidence root: `work/ghidra/rel-d_basesNP/export/`.

| Address | Size | Rust body | Description |
| --- | --- | --- | --- |
| `0x806D16B0` | 332 | `fn_806d16b0` | Variant-selects the collision box height (`0x1FD` at `+8` -> `148.0`, else `116.0`), seeds colour/flags, installs `+0x3D0` via `0x800DB180`, sets `+0x434/+0x438/+0x43C`, switches virtual `+0x394` slot `0x18`, and lifts `+0x340/+0x344` by the main.dol float `0x8042BB08`. |
| `0x80734E30` | 332 | `fn_80734e30` | Four-slot audio fade: clears `+0x518` and `+0x524..+0x530`, then per latched-but-not-pending slot computes `0.5 * (160.0 * [+0x520]) * sin([+0x51C] as u16 / 256)`, negated when `+0x348` is zero, and finally shifts `+0x534..+0x537` into `+0x538..+0x53B`. |
| `0x80775BB0` | 336 | `fn_80775bb0` | Walks up to 30 clip ids at `+0x6C470`, copies the position from `0x801006C0`, samples the walk table at `0x8031CA18`, resolves the part handle and clip object, and starts/stops the clip with `0x800DFF30` per the `0x800FE040` sphere test. |
| `0x80788B50` | 348 | `fn_80788b50` | Places an effect: copies the target position, starts animation slot `3` via `0x80789B80` at rate `5.0`, forwards the absolute or origin-relative position to `0x800F6410`, caches `+0x3C4/+0x3C8/+0x3CC`, the target position and a cleared `+0x338`. |

## Deferred

None.

## Verification notes

- **Constant pool.** `0x806D16B0` builds its pool with `lis r31,0x807D` /
  `addi r31,r31,-0x4B20`, i.e. `0x807CB4E0`; the constants used are
  `0x807CB510`=1.0, `0x807CB514`=148.0, `0x807CB518`=116.0, `0x807CB51C`=0.0,
  `0x807CB520`=8.0, `0x807CB524`=32.0, `0x807CB528`=96.0, `0x807CB52C`=80.0
  (all re-read from `work/rels/elf/d_basesNP.elf`).
- **`0x806D16B0` branch.** The raw test is `lhz r0,0x8(r3)` + `cmpli 0x1FD`
  (Ghidra printed `short`/`sVar2`; the load is unsigned, which does not change an
  equality test against `0x1FD`). The `lfs` of `148.0` sits on the taken side of
  `bc 4,2` when the flag register is `1`, so the mapping is
  `== 0x1FD -> 148.0`, else `116.0`.
- **`0x806D16B0` call.** `f1/f2 = 0.0`, `f3 = 8.0`, `f4/f5 = +/- height`,
  `r3 = this+0x3D0`, `r4 = this`, `r5/r6/r7 = 0/1/0`, matching the existing
  `call_f64x5_u32x5` shim. `0x800DB180` has no export, so its meaning stays
  `unknown`.
- **`_savegpr`/`_restgpr`.** `0x80734E30` opens with `0x802DCD74` and closes
  with `0x802DCDC0`; `0x80775BB0` opens with `0x802DCD68` and closes with
  `0x802DCDB4`. Neither helper is emitted and `r3` survives as the entry
  argument, so both bodies take `this`.
- **GQR3 sample.** `0x80734E30` does `lha r0,0x51C` -> `sth r0,0x8(r1)` ->
  `psq_l f0,0x0(r30),1,3`. With GQR3 = `0x00050005` the reload is an unsigned
  16-bit sample, so the sign extension of `lha` is discarded:
  `read_u16(this + 0x51C) as f32`.
- **`0x80237BD0`.** Called from `0x80734E30` with only `f1` live and used as
  `f28 * f1` afterwards, so it is the established `f32 -> f32` sine helper and
  is invoked through `f32_of_f32`. `r3` at that point holds `this + slot`, a
  loop scratch value, not an argument.
- **`0x80789B80`.** Its own raw words (`rlwinm r0,r4,2,0,29`,
  `lwz r5,0x438(r3)`, `addi r3,r3,416`) show `(f1 = rate, r3 = object,
  r4 = slot)`; Ghidra's `(double, int, int)` prototype is wrong. The call in
  `0x80788B50` is therefore `call_f32_u32x2(0x8078_9B80, 5.0, this, 3)`.
- **`0x80788B50` position reads.** The stack copy at `r1+0x14` is only the
  argument of `0x800F6320`; the `fcmpu` pair (`lfs f1,0x0(r30)` / `0x8(r30)`),
  the subtracted delta (`r1+0x08..0x10`) and the three `+0x40C..+0x414` stores
  all re-read the caller's `position` pointer (`r30 = r4`).
- **`0x80788B50` unused registers.** `f1` is overwritten by the `5.0` load
  before any read and `r5` is never referenced; both are kept in the signature
  so the argument order matches the existing `call_f32x3_u32x4` call sites at
  `src/game/bases.rs:141779` and `:186288`.
- **`0x80775BB0` handle.** `0x80168400`'s `r3` is
  `this + [+0x338C] * 0xBF8 + 0x1A8`, its result is stored to a four-byte stack
  local and that local's address is passed to `0x8023B2A0`; modelled as
  `[u32; 1]`, the same shape as `fn_807603d0`.
- **`0x80775BB0` table.** The walk-table read is
  `*(f32 *)(0x8031C9B0 + slot * 8 + [+0x3388] * 0xF0 + 0x68)`; the `+0x68`
  is folded into the literal `0x8031CA18`. The table lives in `main.dol`
  (below the REL's synthetic base), so its contents were not inspectable; only
  the address arithmetic is claimed.
- **`0x80775BB0` re-read.** The id word is re-loaded from
  `this + 0x6C470 + slot * 4` for the `0x800DFF30` call, after `0x801006C0`
  and `0x800FE040`, which is why the body re-reads it instead of reusing `id`.
