# Coverage - Wave 176 lane A (`game::bases`, 416-byte band)

## Summary

| Lane | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| A | `game::bases` | 5 | 2,080 | 4 | 1 | 0 |

All five functions are exactly 416 bytes in `functions.tsv`; every body is in
`work/wave176/laneA_block.rs` and is appended to `src/game/bases.rs` by the
coordinator.

## Bodies

- `0x80629240` (`fn_80629240`) - board cursor repeat/debounce. Same cluster as
  `fn_80628f70`, but reads pad nibble `+0x14` instead of `+0x1C`, compares it
  with the latch `+0xAB8` and honours the repeat counter `+0xAB4` before
  stepping. **high**.
- `0x80693F00` (`fn_80693f00`) - two-kind dust spawner. Builds the 18-float
  jitter table at `0x8082B1C8` from `-10.0`/`10.0`/`0.0`, then spawns actor
  `0x85` or `0x59` through `0x80064610`. **high**.
- `0x80701390` (`fn_80701390`) - four-slot player pickup scan on the
  `[0x80429C94]` gate. **medium**: the receiver is recovered from
  `_savegpr_23` at `0x802DCD60` (the export declares `void f(void)`), and
  `0x80051E70` is called with the live `r3` handle although its export prints
  no parameters.
- `0x8071D1A0` (`fn_8071d1a0`) - rebuilds the `+0x1A4C0`/`+0x1A500` part
  transforms of the `+0x1A4xx` effect block. **high**.
- `0x8073F440` (`fn_8073f440`) - three-state `0x8015F140` fade driver, cues
  driver slot `+0x2C` with `0x30B` and refreshes `+0x394`. **high**.

## Deferred

None.

## Corrections and cross-checks

- `0x80701390`: raw words at `0x807013dc..0x807013f4` load
  `[0x8042A078]`, then `+0x00`/`+0x04` from that pointer; the translation
  dereferences before masking. This matches the `0x8042A078` note in the wave
  175 coverage doc, but differs from the direct
  `read_u32(0x8042_A078)`/`read_u32(0x8042_A07C)` form used by the older
  `fn_80703840`, which has the same raw instruction idiom. Flagged, not
  changed: `fn_80703840` is outside this lane.
- `0x80701390`'s `and r0,r4,r0` / `and r3,r5,r3` / `or. r0,r3,r0`
  (`0x7c800038`, `0x7ca31838`, `0x7c600379`) are genuine mask-and-combine
  ops, not self-`and` dead code; the field decode (rS/rA/rB) was re-checked
  because `ppc159.js` prints the A-form operands in `rS,rA,rB` order.
- `0x80693F00`'s `psq_l`/`lfs` pair at `0x80693FDC`/`0x80693FE0` loads table
  entry `jitter*2` and `jitter*2+1`: on `GQR0 = 0` the paired-single form is
  two plain `f32` words, which is why the table is stored as 18 floats;
  `fsubs f2,f2,f0` uses `fB` (bits 11-15), matching Ghidra.
- `0x80701390` and `0x8071D1A0` both open with `GQR0` "dequantise" blocks in
  the export; those are the `psq_st`/`psq_l` `f31` save slots in the prologue
  and epilogue, not vector code, and are dropped.
- `0x8073F440`'s `0x807CDEF8..0x807CDF20` pool was read out of the ELF:
  `0.1, 0.4, 0.6, 1.0, -1.0, 0.0, 0.5, 0.01, 0.025, 0.001, 1000.0`; only the
  eleven values used by the three states and the `180`-frame budget appear in
  the body. `0x7c050734` is `extsh r5,r0`, so the cued height is
  `(f32 as i32) as i16`.
- `0x8071D1A0`'s `stb` at `0x8071d308` writes `r1+0xF`, the low byte of the
  `0x8000D500` word at `r1+0xC`, so the patch is
  `(word & 0xFFFFFF00) | 0x7F`.

## Verification

- Block compiles standalone against stubbed `platform`/`abi_call*` helpers
  (`rustc --edition 2021 --crate-type lib check.rs`), so every macro, helper
  name and signature resolves against the existing `src/game/bases.rs` set.
- `node tools/check-lane-csv.mjs wave176`: lane A reports 5 rows, 5 unique
  addresses, all evidence present and no byte-size mismatch (every doc says
  `416 bytes`, matching `functions.tsv`); the only lane-A problem is the
  expected `has no body ... in src/game/bases.rs` line, because the coordinator
  has not appended the block yet (lane E is in the same state).
- `ledger/functions.csv` does not contain any of the five addresses, so no
  claim collides with a merged row.
