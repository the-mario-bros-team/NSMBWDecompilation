# Coverage - Wave 180 lane D (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 1 | 320 | 1 | 0 |

Single body from the 300-332 byte slice of the band. Source of the body:
`work/wave180/laneD_block.rs`.

## What this pass covers

- **Random clear-slot picker (1 body).** `0x8074B8D0` reads the camera band
  behind the pointer at `0x8042_9E00` (`low = +0x38`, `span = +0x40`), forms
  `base = low + 0.25 * span` and `range = (low + span - 32.0) - base`, and
  truncates `range / 48.0` (all constants are `f32`: `0x807CE5F8` = `0.25`,
  `0x807CE5FC` = `32.0`, `0x807CE5E8` = `48.0`). The truncated count plus one is
  the exclusive bound of the RNG at `0x800B_2EE0`; each of at most 50 draws
  probes `24.0 + (base + 48.0 * slot)` and the first candidate that clears the
  actor's recorded `+0x3E4` distances is returned. The probe is exactly
  [`fn_8074ba10`], already translated in this module, whose half-window
  `0x807CE600 * 0x807CE5E8` is the `24.0` cell centre offset added here. After
  50 collisions the band midpoint `base + 0.5 * range` is returned.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8074B8D0` | `fn_8074b8d0` | 320 | high |

## Deferred

None.

## Corrections and notes

- The prologue `psq_st f27..f31` / epilogue `psq_l f27..f31` pairs are scalar
  `f32` spills of the callee-saved FPRs on GQR0 (no `psq_l` type-6 pair, no
  quantisation); they carry no semantics and are not modelled. The five
  `ldexpf` blocks and the `unaff_GQR0` bit tests in the Ghidra export are the
  established GQR0 compiler noise.
- The Ghidra export types the body `void(void)` only because it cannot see
  through the `0x802DCD74` register-save thunk (which preserves `r3`); the raw
  words pass `r3` straight to `fn_8074ba10`, and the body returns through `f1`
  on both paths (`lfs f1,0x8(r1)` and `fadds f1,f28,f0`), hence
  `fn(this) -> f32`.
- `0x8074B9C4`..`0x8074B9CC` is `f0 = 0.5 * f27; f1 = f28 + f0`: the fallback
  uses the very same `range` the RNG bound was derived from, so the midpoint is
  `base + 0.5 * range`.
- The `stw 0x43300000` / `xoris r0,r3,0x8000` pair with the `lfd` and
  `fsubs ... ,[0x807CE608]` is the unsigned `u32 -> float` idiom: the double at
  `0x807CE608` is `4503601774854144.0` (`2^52 + 2^31`), exactly representable
  in single precision, so the subtraction cancels the bias bit-for-bit and
  leaves the RNG word. The chain is modelled as `slot as f32`; every later
  operation is `fmuls`/`fadds` (opcode-59 single precision), decoded from the
  raw words with the per-XO operand rule (`fmuls f0,f29,f0` uses FRC at bits
  21-25: `0xEC1D0032`, `0xEC0006F2`).
- `0x8042_9E00` is dereferenced (`lwz r4,0xffff9e00(r4)` then `lfs 0x38(r4)`),
  so the body loads the pointer and reads the fields off it rather than
  treating the global as the struct itself.

## Verification

- The body was type-checked standalone against the real helper signatures
  (`platform::read_u32`/`read_f32`, `abi_call1!`, `fn_8074ba10`) with
  `rustc --emit=metadata`: no errors, no warnings. `src/game/bases.rs` was not
  modified; the coordinator's `merge.cjs` appends the block.
- The evidence export exists at
  `work/ghidra/rel-d_basesNP/export/8074b8d0_FUN_8074b8d0.c`; the body is
  exactly `0x8074B8D0..0x8074BA10` (320 bytes), every constant was read from
  `work/rels/elf/d_basesNP.elf`, and every branch (`beq` on the
  `fn_8074ba10` result, `blt` for the 50-iteration back edge) was verified
  against the raw words.
