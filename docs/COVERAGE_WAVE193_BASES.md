# Coverage - Wave 193 (`game::bases` 192-256 band, part 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 50 | 11,232 | 48 | 2 |

Second wave in the 192-256 band.

## Highlights

- **Seven more callees became real, with all 8 call sites wired.**
  `0x806A9F50` (rider effect table), `0x806CB140` (follower copy + rotation),
  `0x806D0130` (lowest player height, returns `f32` in `f1`), `0x806EC5F0`
  (seed pair), `0x806ED160` (rider-list mask), `0x806FE310` (player-slot scan)
  and `0x8070F9C0` (grid snap, `(f32, usize, usize) -> f32`).
- **Node/plank rebuilds (5).** `0x806E26B0` (four `0xE0`-stride plank nodes),
  `0x806E33A0`, `0x806F6700`, `0x806F7830`, `0x80706110`.
- **Chain walkers (6).** `0x806990B0`, `0x806A5EE0`, `0x806AB480`,
  `0x806ADF30`, `0x806AEFB0`, `0x806C7040` - the five-unrolled-level walker
  family, now with `0x806E9C10`/`0x806F6040` as further members.
- **Lift/boat bodies (8).** `0x806D0410`, `0x806D4A20`, `0x806D7A30`,
  `0x80703940`, `0x80703A40`, `0x80704790`, `0x807057C0`, `0x806FFC50`.
- **The rest** are constructors, destructors and small state helpers.

## Corrections and recoveries

- `0x806F8160`'s two arms are swapped in the Ghidra C: the raw `beq` at
  `0x806F818C` selects the subtract arm, so `+0x574 != 0` is the direct copy.
- `0x80703940`'s `*(int *)uVar9` is really the `r4` out-pointer, hidden by
  `_savegpr_24`.
- `0x80703A40`'s `cntlzw(count-1)` folds to `count == 1`.
- `0x806F8870`'s recursive tail uses the fourth node's `+0x30`.
- `0x806CC530`'s table index is `((w>>16)&1)*0x18 + ((w>>24)&0xF)*4`.
- `0x806CB140` is single-precision throughout; GQR3 makes its angle an
  unsigned-16 sample.
- `0x8070F9C0`'s real signature is `(f1: f32, anchor: usize, step: usize) -> f32`;
  Ghidra folds `f1` into a `double`.
- `0x800D2610` needs 8 register + 1 stack argument, so it uses `call_u32x9`.
- `0x800B_FC60` needs a 5-int prototype with no matching helper, so it is
  marshalled through an inline `transmute`.
- `0x80699940` is **medium**: window structure is literal, `sinf` is inferred,
  and boundary strictness follows Ghidra.

## Verification

- `cargo check` succeeds; the ledger now holds 4,943 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave193`: 50 rows, 50 unique addresses,
  48 high + 2 medium.
- All 50 evidence exports exist; 11,232 bytes total.
