# Coverage - Wave 183 lane D (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 916 | 10 | 0 |

Sub-300-byte sweep of the `0x806B_1380..0x806B_E7E0` band of REL `d_basesNP`.
Source of the bodies: `work/wave183/laneD_block.rs`. Every address is the REL
analysis address with the synthetic base `0x8060_0000`; these are not runtime
addresses.

## What this pass covers

- **Deleting destructors (7 bodies).** `0x806B1380`, `0x806B46F0`,
  `0x806B8CF0`, `0x806BA820`, `0x806BE780`, plus the 92-byte `0x806BC2B0` and
  `0x806BE7E0`. All seven run `0x8015_F800(this, 0)` on a non-null object and
  call `0x802B_9280(this)` when the caller's free flag is positive, returning
  `this`. They are the same shape as the already-translated
  [`fn_806511d0`]; each is kept as its own body because each class registers
  its own destructor. The two 92-byte variants carry a second, redundant
  `beq` on `r3` before the teardown call and are otherwise identical.
- **Aim reset + side notify (1 body).** `0x806B4B20` is the twin of
  [`fn_806c3710`]: gate on `*(u8*)(other + 0x38C) == 1`, call
  [`fn_806b4ed0`]`(this, 1)`, compute `delta = this.x - other.x` and forward to
  [`fn_806b4ba0`].
- **Target-vector reset (1 body).** `0x806B4ED0` clears the float at `+0x6DC`
  to `0.0`, re-runs `0x800E_81C0` on that address with `1.0`/`0`, and stores
  the 16-bit argument at `+0x6D6`, all behind the `+0x6EC` enable byte.
- **Actor finaliser (1 body).** `0x806B9CB0` calls [`fn_806b9720`], then
  virtual slot `+0xE0` of the `+0x60` vtable, then `0x8016_2510(this)`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806B1380` | `fn_806b1380` | 88 | high |
| `0x806B46F0` | `fn_806b46f0` | 88 | high |
| `0x806B4B20` | `fn_806b4b20` | 124 | high |
| `0x806B4ED0` | `fn_806b4ed0` | 96 | high |
| `0x806B8CF0` | `fn_806b8cf0` | 88 | high |
| `0x806B9CB0` | `fn_806b9cb0` | 72 | high |
| `0x806BA820` | `fn_806ba820` | 88 | high |
| `0x806BC2B0` | `fn_806bc2b0` | 92 | high |
| `0x806BE780` | `fn_806be780` | 88 | high |
| `0x806BE7E0` | `fn_806be7e0` | 92 | high |

## Deferred

None. All ten bodies in the lane were translated.

## Notes and residual risk

- **`0x806B4B20` subtraction direction and flag.** Ghidra prints
  `*(f32*)(param_2 + 0xAC) - *(f32*)(param_1 + 0xAC)`, but the raw
  `lfs f2,0xac(r30)` / `lfs f1,0xac(r31)` / `fsubs f1,f2,f1` loads `r30`
  (the first argument) into `fD`, so the delta is `this.x - other.x`. The
  compare block (`0xFC010040` `fcmpu`, `0x7C000026` `mfcr`, `0x540017FE`
  `rlwinm …,2,31,31`, `0x7C000034` `cntlzw`, `0x5405DE3E` `rlwinm …,27,24,31`)
  isolates the CR0 **GT** bit, so Ghidra's `countLeadingZeros(0.0 < delta) >> 5`
  is `1` exactly when `delta <= 0.0`. The existing [`fn_806c3710`] doc says the
  same idiom is `delta >= 0.0`; the two differ only at `delta == 0.0`, and this
  body follows the raw CR-bit extraction.
- **`0x806B4B20` third argument is dead.** `0x806B4B20` sets `r5` to the flag
  above before `bl 0x806b4ba0`, but the whole 456-byte body of
  [`fn_806b4ba0`] uses only `r3`, `r4`, `r12`, `r0` and the float registers, so
  the flag is unobservable. `fn_806b4ba0` has no third Rust parameter, so the
  value is computed and dropped with a comment instead of being invented as a
  new parameter.
- **`0x806B4ED0` call width.** The store at `+0x6DC` is `stfsu` of
  `0x807C_AA64` (`0.0f`), and the following call loads `0x807C_AA68`
  (`1.0f`) with `lfs`; Ghidra's `(double)` cast on that constant is its usual
  double-printing artefact. `0x800E_81C0` is emitted through
  `call_f32_u32x2`, the same helper and operand order used at the
  already-translated `fn_806bf2c0` sites.
- **`0x806B9CB0` first call.** Ghidra prints `FUN_806b9720()` with no argument,
  but the raw words are `mr r31,r3` in the prologue and `mr r3` is still the
  actor at the `bl`, so [`fn_806b9720`]`(this)` is called.
