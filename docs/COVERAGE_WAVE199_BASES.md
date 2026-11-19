# Coverage - Wave 199 (`game::bases` very large bodies, part 2, complete)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 12 | 13,684 | 9 | 3 |

The final giant bodies, all merged on the first pass. This closes every
translatable function in `game::bases`.

## Merged

- **Lane A.** `0x807826B0` (516, `cobHatenaSwitch` accessor/model/animation
  setup) and the 4,232-byte `0x8079C330`, a `switch (state - 6)` over states
  `6..=0xA4` with the 0x9F-entry table at `0x8081B884` - every arm translated.
  **medium** (state-arm roles).
- **Lane B.** `0x807B8CB0` (516, course-scene setup with the
  `.bfog`/`.blmap`/`.blight` entry tables at `0x807D08E0`/`0x807D0884`/
  `0x807D0828`; **medium**) and the 4,076-byte `0x8077B450` (99-entry jump
  table at `0x808181F8`, states `10..=0xA2`, plus the 26-arm decision tree at
  `0x8077BD3C`; all 1,019 instructions).
- **Lane C.** `0x807B7B40` (524, world-map mode switch on `0x8042900C`) and
  `0x8079D3C0` (3,176, 157-entry dispatch at `0x8081BB00`, 40 case blocks
  translated arm by arm).
- **Lane D.** `0x80731D10` (540, phase machine at `+0x9E4`) and `0x8078DC70`
  (2,576, 30-arm jump-table pass at `0x8081A580`, all 169 entries dumped).
- **Lane E.** `0x807962B0` (1,428, course-record start-up) and `0x8076EF80`
  (2,544, two-phase table with lazy `0x80832800`/`0x80832880` fills).
- **Lane F.** `0x807740D0` (1,656) and `0x80773210` (2,096).

## Corrections and recoveries

- `0x8079C330`'s Ghidra `void(void)` is a `_savegpr` i64-return artifact; the
  real signature is `(this, state)`.
- `0x8079C330` has two arms dispatching through vtable pointers **not at object
  offset 0** (`obj+0` slot `0x5C`, `handle+4` slot `0x38`), so they load the
  slot address and call it directly.
- Its export misprints FPRs as `rN`; every float load/store was re-read from raw
  words (changing state `0x25`'s y-delta and distinguishing `0x1A` vs `0x23`).
- Three states branch on the `r3` result of `0x8079F500`, which the existing Rust
  body discards, so they use `abi_call1!(0x8079_F500, this)`.
- `0x8079D3C0`'s dispatch table is `0x8081BB00`, not `0x8082BB00` (an `addi`
  sign error in the earlier read); all 157 words were read in full.
- Lane B: globals are `0x8043xxxx`, and `0x800F69F0` takes 3 floats + **five**
  ints (`call_f32x3_u32x5`).
- Lane D: the jump table base is `0x8081A580` (`addi -0x5A80`); `bc 12,n`
  branches when set and `bc 4,n` when clear.
- Lane C: mode `0` of `0x807B7B40` gates `0x800FB9E0` on `(x,y)==(2,3)`.

## Completion status (`game::bases`)

| Class | Count |
|---|---:|
| Translated | 5,134 |
| `0x802DCBFC` forwarders (no shipped target) | 472 |
| Documented deferrals (read unwritten state) | 2 |
| 32-48 byte body remaining | 1 |

## Verification

- `cargo check` succeeds; the ledger holds 5,134 `game::bases` rows.
- `node tools/check-lane-csv.mjs wave199`: 12 rows, 12 unique addresses,
  9 high + 3 medium.
- All 12 evidence exports exist; 13,684 bytes translated in this wave.
