# Coverage - Wave 222 (`resource::archive` close + `ui::hud` stragglers)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/archive.rs` | `resource::archive` | 18 | 4,944 | 17 | 1 |
| `src/ui/hud.rs` | `ui::hud` | 2 | 440 | 2 | 0 |

Both modules are now complete: **`resource::archive` 747/747** and
**`ui::hud` 542/542**.

## What landed

- **Flag decoders and record initialisers (lane A).** `0x802CE6B0`,
  `0x802CE6F0`, `0x802CE740`, `0x802CE850`, `0x802CE940`.
- **GX latches and colour scaling (lane B).** `0x802CE9B0`, `0x802CE9C0`,
  the 11-arm jump table `0x802CEA40`, `0x802CEAF0`.
- **Colour lerp, texgen matrix, mode-2 context (lane C).** `0x802CEC60`,
  `0x802CED80`, `0x802CEE70`.
- **Arc records and slice emitter (lane D).** `0x802CEF70`, `0x802CF050`,
  the 1,296-byte `0x802CF1E0`.
- **Draw records and pane painting (lane E).** `0x802CF6F0`, `0x802CFBA0`,
  the 816-byte `0x802CFD00`.
- **Hud stragglers (lane F).** `0x80026CA0`, `0x8002D260`.

## Corrections and recoveries

- **Dangling reference closed.** `0x8002D260` is the tail that
  `fn_8002d0c0` already called through `FN_8002D260_D`; the two now agree.
- `0x80026CA0` slots into the wave-204 `0x80026B50..0x800272C0` family and
  reuses the `Wm_en_hit` spawner `0x800954D0`; its polarity is kept as shipped
  and the unknown latch semantics are flagged rather than "fixed".
- **`0x802CE740` operand trap.** The disassembler prints both `lfs f0` and
  `stw r0` as `"r0"`, but the `stw` group writes the GPR zero from `li r0,0`,
  so Ghidra's `*param_1 = 0` is correct.
- `0x802CEA40` is an 11-arm jump table at `0x80350418`: modes 0/1/2/3/5/7/10
  call `0x802C3020` with `(0x0E,0,2,4,5,3,7)`, while 4/6/8/9 call
  `0x801C8DC0` with `(1,2,3,0)`, `(1,3,3,0)`, `(1,5,1,0)`, `(3,4,5,0)`.
- `0x802CEAF0`'s export types its first argument `double`, but every multiply
  is `fmuls`; the real signature is `(scale: f1, dst: r3, src: r4, flag: r5)`.
- `0x802CED80`'s angle is `40.7436638f * (pi * *(this+0x24))`, exactly
  `128 * +0x24`; the compiler's `*1.0`, `fneg` and `0.5` bias terms and the
  store order are preserved.
- `0x802CEE70` lazily seeds `0x80424700 = 1.0f x3` and `0x8042AF00 = 0.0f x2`
  behind two SDA gates, then builds a `0x88`-byte mode-2 context.
- `0x802CF050`'s chunk bound is `srawi` + `addze` (a count of 0 still runs one
  pass), reproduced exactly rather than as a plain shift.
- `0x802CF1E0` medium: the `0x80237CA0` sine/cosine output order and the GX
  setter names (derived by shape from their own exports) are marked
  `// partial:`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave222`: 20 rows, 20 unique, 19 high + 1 medium.
- Ledger: 20,118 rows (99.34%); `resource::archive` 747/747, `ui::hud` 542/542.
- `scripts/check.ps1` exit 0.

## Next

`src/game/stage.rs` (132 bodies), then the untabled bodies found in dsys and
archive, and the audit items accumulated over waves 214-221.
