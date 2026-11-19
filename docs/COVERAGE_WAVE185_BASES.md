# Coverage - Wave 185 (`game::bases` sub-300 sweep, part 5)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 4,820 | 43 | 7 |

Fifth sub-300 wave, 10 bodies per lane; 64-128 byte band, `0x80722560`
through `0x80778770`.

## Composition

- **Destructors/release wrappers (10).** `0x80722560`, `0x807225C0`,
  `0x807238F0`, `0x8072C830`, `0x80757E10`, `0x8073B440`, `0x8073C120`,
  `0x80777990`, `0x807779F0`, plus `0x80778A50`'s insert predicate.
- **Ride/ship bodies (8).** `0x80757280`, `0x8075AEF0`, `0x8075C010`,
  `0x8075C1C0`, `0x8076B560`, `0x8076C090`, `0x8076D3B0`, `0x8076FF10`.
- **Collision rebuilds (2).** `0x807242C0` (wide) and `0x80724320` (narrow),
  both recovered from `halt_baddata` as `b 0x8006D090` tail calls.
- **Wave helpers (10).** `0x8073F000`, `0x8074DB30`, `0x80751C90`, `0x80754620`
  and the `0x807786xx` rate family.
- **The rest** are small predicates, ctors and transform copies.

## Deferral retired

- `0x8075F9E0` was deferred by **wave 106**. The raw words show `r3` still
  holding the `0x80162D50` result when `bl 0x8075FA40` runs, so the "missing
  actor pointer" is that lookup. Now translated.

## Corrections and cross-checks

- `0x807242C0`/`0x80724320`: Ghidra truncates both to `halt_baddata`; the raw
  words end in a `b 0x8006D090` tail call.
- `0x80733F10`'s two `double` parameters are `f32`: `f1` is only compared,
  `f2` only added/subtracted.
- `0x80770BE0`'s export is truncated; the body is a frameless leaf ending in a
  `b 0x80105440` tail call.
- `0x807754D0` forwards the live `r3` (the `0x80761310` probe result) as the
  actor argument Ghidra drops.
- `0x80776750` forwards `r4` (the `0x80069020` vtable descriptor) as arg 2.
- `0x807786xx`'s index is `(type & 0xFF) * 4`; `0x80778770`'s `lfs` targets
  `f0` and does not clobber the index.
- `0x8074DB30` passes `this` (not the float) to `0x8074db10`.
- `0x8073B1B0` is **medium**: its `+0xC` callee takes a 12-byte position by
  value in the EABI outgoing parameter area, which no existing helper can
  express, so it is modelled receiver-only.
- `0x8076FE70`/`0x8076FF10` are **medium** (`0x80106B40`/`0x801004B0`/
  `0x801006C0` contracts unestablished).

## Verification

- `cargo check` succeeds with 4,568 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave185`: 50 rows, 50 unique addresses,
  43 high + 7 medium.
- All 50 evidence exports exist; 4,820 bytes total.
