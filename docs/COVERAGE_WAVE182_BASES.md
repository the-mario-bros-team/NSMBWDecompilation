# Coverage - Wave 182 (`game::bases` sub-300 sweep, part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 4,692 | 46 | 4 |

Second sub-300 wave, 10 bodies per lane. Covers the 64-128 byte band from
`0x806249B0` to `0x806842D0`.

## Composition

- **Destructors (10).** `0x806269A0`, `0x80627E10`, `0x8062B4C0`, `0x80669040`,
  `0x806691D0`, `0x80674840`, `0x8067ACD0`, `0x80656B70`, `0x806626A0`,
  `0x8067C340`/`0x8067D140`/`0x8067D1A0`/`0x8067F350`. The
  `0x805F9F0`-vs-`0x8015F800`-vs-`0x800DAE00` teardown callee varies by class.
- **Factories (3).** `0x8067D4F0` (`0x3F8`, vtable `0x807EC830`), `0x806677E0`
  (`0x398`, vtable `0x807E8304`), `0x806842D0` (`0x8A8`, vtable `0x807ED83C`).
- **State gates (6).** `0x806249B0`/`0x80624B60`/`0x80624C80`/`0x806387A0`,
  `0x8063D260`, `0x80656570`.
- **Collision/probe (2).** `0x8067BC30`, `0x8067CD10`.
- **Sound/effect (4).** `0x80653B60`, `0x80653BC0`, `0x80658610`, `0x8067A900`.
- **Everything else** is small arithmetic, layout and record helpers.

## Boundary removals

- `0x806505B0` is now a real body (`3600.0 / (u16)`), and all four
  `call_f32_0(0x8065_05B0)` sites in `fn_8064eeb0`/`fn_8064f390`/
  `fn_8064f510`/`fn_8064f820` call it directly.
- `0x80640AC0` (previously "bad instruction data") is translated, and
  `fn_80640b40` now calls `fn_80640ac0(this, slot)`; its doc moved to `high`.
- `fn_80684a30`'s doc no longer claims the `0x806842D0` wrapper runs it - the
  raw branch proves it runs the twin `0x80683750`.

## Corrections and cross-checks

- `0x80640AC0`'s raw words are a plain `cmpi` chain with a seat permutation
  `0/1/3/2`.
- `0x80673600`'s `lfs`/`lfd` stack dance means the operands are `x` and
  `176.0f`, not the double Ghidra implies. **medium**.
- `0x8064B660`'s `0x802DCBFC` dispatch needs `r12 = this + 0x42C`, which Rust
  cannot set, so it stays an address-only call. **medium**.
- `0x8064D150`, `0x806564A0` (**medium**, `0x80A46F60` by address),
  `0x80675040` and `0x8067B160` had `halt_baddata`/`bcctr` tails and were
  decoded word by word.
- `0x80656570`'s tags needed explicit `u32` suffixes; they had been inferred as
  `i32` and overflowed.

## Verification

- `cargo check` succeeds with 4,419 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave182`: 50 rows, 50 unique addresses,
  46 high + 4 medium.
- All 50 evidence exports exist and every size matches `functions.tsv`
  (4,692 bytes total).
