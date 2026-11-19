# Coverage - Wave 184 (`game::bases` sub-300 sweep, part 4)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 49 | 4,692 | 46 | 3 | 1 |

Fourth sub-300 wave, 10 bodies per lane. Covers `0x806CBAF0` through
`0x807209B0`.

## Composition

- **Destructors / release wrappers (15).** `0x806CBAF0`, `0x806CCED0`,
  `0x806D26C0`, `0x806E9AC0`, `0x806F86C0`, `0x806F8720`, `0x806FC840`,
  `0x80705B00`, `0x80706BD0`, `0x80708160`, `0x807082D0`, `0x8070F0F0`,
  `0x80717960`, `0x80719B80`, `0x8071EAA0`.
- **Player-slot gates (3).** `0x806D0780`, `0x806D22B0`, `0x806D3AC0` - same
  template with pending fields `+0x648`/`+0x53C`/`+0x75C` and ids `0xC`/`0`/`0xD`.
- **Sink clamps (2).** `0x807169E0` (`+0x670`), `0x80716A70` (`+0x690`). These
  are siblings of `fn_80716860`/`fn_807168f0`, but with the sink-direction
  `bc 4,1` test rather than the raise `bc 4,0`.
- **Factories (4).** `0x806E1780` (`0x3A8`, vtable `0x807FC9E0`),
  `0x806FCA20`, `0x80706D50` (`0x3A0`, vtable `0x8080481C`), plus
  `0x806E0EA0`'s partner bind.
- **Everything else** is small state, timer, layout and arithmetic helpers.

## Deferred

- **`0x806E2BF0` (96 bytes, lane B).** The raw words loop the four
  `lift_suisha` planks (`+0x430..+0x7B0`, stride `0xE0`), load
  `+0xCC/+0xD0/+0xD4/+0xD8`, add `8.0` (`0x807CBBD4`) to two of them, and store
  **nothing** back into any plank - the only stores are six `stfs` into the
  function's own `0x50`-byte frame. Ghidra shows only the counter loop. The
  `ori r0,r0,0` nop at `0x806E2C04` suggests a post-link patch, and the
  destination field is unrecoverable, so no body was emitted.

## Corrections and cross-checks

- `0x806E1780`'s vtable is `0x807FC9E0` (`lis 0x8080` + `addi -13856`), not
  Ghidra's `0x8080C9E0`.
- The three `halt_baddata` player-slot bodies are `b 0x800B3600` tail branches;
  emitted as `abi_call2!` with a `void` C signature.
- The slot gates dereference `0x8042A320` before adding the `0x23C` stride.
- `0x806E5580`/`0x806E7350` are byte-identical twins; `0x806F3010`/`0x806F3080`
  share the `0x803295D8` base with different derived vtables.
- `0x8071D120` is single-precision throughout; Ghidra's `double` parameter is
  wrong, `t: f32`, and the cast truncates (`as i32`).
- `0x80718960` is 100 bytes (epilogue at `0x807189C3`), not the 92 in
  `functions.tsv`.
- `0x807209B0`'s `_DAT_80428730` is 16-bit (`lhz`), so `read_u16`.
- `0x80706230`'s `xoris` magic pair minus `0x807CC6D8` (`2^52+2^31`) recovers
  the `lha 0x3FC` word; `0x807CC6E0 = 0.00067138671875`.
- `0x80706D50`'s constructor takes `r3` (fresh allocation), though the C export
  shows no argument.
- `0x806F3010`/`0x806F3080` vtables are `0x80329578`/`0x80329518` on the
  `0x803295D8` base.

## Verification

- `cargo check` succeeds with 4,518 public bodies and no duplicate names.
- `node tools/check-lane-csv.mjs wave184`: 49 rows, 49 unique addresses,
  46 high + 3 medium.
- All 49 evidence exports exist and the recorded sizes match the code extents
  (4,692 bytes total).
