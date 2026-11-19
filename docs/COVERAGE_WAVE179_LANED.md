# Coverage - Wave 179 lane D (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 3 | 1,512 | 2 | 1 |

Third batch from the 500-508 byte slice of the 300-512 byte band. Source of the
bodies: `work/wave179/laneD_block.rs`.

## What this pass covers

- **World-map aim state machine (1 body).** `0x807C0400` is the eight-arm state
  machine behind `+0x6A0`, built on the aim-object family already translated as
  `fn_807c0d30`/`fn_807c0600`/`fn_807c0fd0`-callers. It seeds the aim vector
  `+0x6B0..+0x6B8` from `0x80377DC8..0x80377DD0`, waits on the four existing
  helpers, tests the five manager bytes `+0x511..+0x515`, derives the `+0x73C`
  tolerance from `|2.0 * +0x6FC / |(+0x6A4,+0x6A8,+0x6AC)||`, resets the
  singleton through `fn_807c3a10`/`fn_807c3a90`, and runs a `20`-frame delay at
  `+0x720`. It reuses `fn_807c0d30`, `fn_807c0600`, `fn_807c3a10` and
  `fn_807c3a90`; the four un-translated callees go through `abi_call1!` and the
  aim-length helper through `call_f32_1`.
- **Unnamed `0x1A500`-byte UI actor projection (1 body).** `0x80648060` is the
  sibling of the already-translated `fn_80648260`/`fn_80648300`/`fn_806483b0`/
  `fn_80648440` block. It snapshots the marker height, picks a depth of `4.0`,
  `7100.0` or `6016.0` from `+0x1A4FA`, publishes the shared screen height,
  builds the `(+0xAC,+0xB0,+0xB4)` stack vector shared by both projection calls,
  computes `+0x1A4C0 = base+0x8FE68 - base+0x8FE64` and
  `+0x1A4C4 = y - (base+0x8FE70 - 16.0)`, then submits through `0x800EAA00`
  and `0x800EBBE0`.
- **Marker-record bound step (1 body).** `0x8073FF60` walks the four `0x20`-byte
  records at `+0x480..+0x680` and re-arms their `0x200` limit halfwords. For the
  records whose `+0x1D` slot is `0..3`, the pad status word `+0x14` (bits `0x8`
  and `0x4`) nudges the signed `+0x00` position by `8` or flags `+0x06 = 4`; the
  position is then clamped against `+0x0E` using the sign of `+0x02` and
  `0x800DF890(record)`. It is the body already called by the translated
  `fn_807401c0` via `abi_call1!(0x8073_FF60, actor)` (that call site can now be
  turned into a direct call by a later pass).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807C0400` | `fn_807c0400` | 500 | high |
| `0x80648060` | `fn_80648060` | 504 | medium |
| `0x8073FF60` | `fn_8073ff60` | 508 | high |

## Deferred

None.

## Corrections and notes

- `0x807C0400`'s dispatch is a jump table at `0x808218A4` with eight entries
  (`0x807C043C`, `0x807C046C`, `0x807C0484`, `0x807C04D4`, `0x807C04FC`,
  `0x807C0570`, `0x807C0590`, `0x807C05D0`). The guard is `cmpli r0,7` +
  `ble`, so states above `7` take the `0` return. Case `0`'s three floats live
  in main.dol (`0x80377DC8..0x80377DD0`), so they are read at runtime, the same
  convention as `fn_807c19d0`/`fn_80682f40`.
- `0x807C0400` case `4` prints the aim-length helper as `(double)`; the raw
  words are `lfs f2,0xac4` (`2.0`), `fmuls`, `fdivs f0,f0,f1`, `fabs`, `frsp`,
  so it is an `f32` division and the helper is `call_f32_1`.
- `0x80648060`'s fifth argument to `0x800EAA00` is an uninitialised `r29` for
  state bytes `>= 7`: the compare chain jumps straight to the call with no
  assignment. The body uses `0` and marks the arm `// unknown:`; every
  documented state (`0..6`) is fully literal. This is the only `medium` in the
  batch.
- `0x80648060` passes one stack vector to both `0x800EAA00` and `0x800EBBE0`
  (the body subtracts `8.0` from its `+4` slot before the first call), so the
  Rust body keeps a single mutable `[f32; 3]` and passes the same address twice.
- `0x8073FF60` is typed `void(void)` by Ghidra only because its prologue calls
  the `0x802DCD68` register-save thunk (which preserves `r3`); the raw words
  immediately use `r3 + 0x480`, and `fn_807401c0` already calls it with
  `actor`, so the body takes `this`.
- `0x8073FF60`'s `CONCAT44(0x43300000, x) - 2^52` double dance only feeds the
  two `fcmpo` tests against `0.0` (`0x807CDFC0`; the magic at `0x807CDFC8` is
  `0x43300000` = `2^52`), so the body compares the signed halfwords directly.
  The `ldexpf` guards and `psq_l`/`psq_st` pair are the established GQR0
  compiler noise.
- `0x8073FF60`'s `+0x1D` test is `lbz` + `extsb` + `cmpli ...,4`, i.e. an
  unsigned compare that rejects negative slot bytes; the body expresses it as
  `kind >= 0 && kind < 4`.

## Verification

- The lane-D block was appended to a backed-up copy of `src/game/bases.rs`,
  `cargo check --lib` finished with no errors and no new warnings from the
  block, and the original file was restored byte-for-byte (the block was then
  removed again; the coordinator's `merge.cjs` appends it).
- All three evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was read from the raw words of `work/rels/elf/d_basesNP.elf`.
