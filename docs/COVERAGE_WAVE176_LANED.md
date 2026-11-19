# Coverage - Wave 176 lane D (`game::bases`, 424/428-byte classes)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 5 | 2,132 | 5 | 0 |

Five bodies: two 424-byte and three 428-byte functions. Source of the bodies:
`work/wave176/laneD_block.rs`.

## What this pass covers

- **Select-screen slot highlight, four-slot variant (1 body).** `0x8063F120` is
  the sibling of the already translated [`fn_8063f300`] and shares its tail
  (state `0x80827560` on a changed index, view slot `+0x20`/`0x900` plus sound
  `0x323`, or `0x800B53A0(0)` plus sound `0x7A` and state `0x80827660`). Its
  extra branch is the zero-index case, which restores `+0x27C` when flag bit 0
  is set, and the `& 2` case, which latches the current index into `+0x27C`.
- **Three-phase highlight fade with a 3-slot ring (1 body).** `0x80733F70`
  drives `+0x64C` through states `1`/`2`, stepping `+0x61C` towards `1.5`
  (step `0.08`) and `+0x618` towards `0.75` (step `0.04`) with
  [`fn_80733f10`], snapping to `1.15`/`1.075`, stepping back to `1.0`, and
  publishing the result into a rotating `+0x620` ring indexed by `+0x65D`.
- **World-slot reservation (1 body).** `0x80746C20` quantises a stage
  rectangle to `1/32`-unit slots, collects the free ones in a 50-word list,
  and either delegates to [`fn_80746f90`] or claims one at random
  (`0x800B2EE0`) and writes its centre `y - 32 * index - 16` to `out`.
- **`bros` model + clip-pair constructor (1 body).** `0x8078C500` is the
  `+0x6E8`/`+0x704`/`+0x744`/`+0x770` twin of [`fn_80790840`]: it mounts
  `g3d/bros.brres` (`0x8081A264`) from the `bros` sub-archive (`0x8081A274`),
  binds the model with size `0x224`, attaches `walk` (`0x8081A254`) and a
  second `bros` clip (`0x8081A260`), applies `1.0`/`0.0`/`3.0`, and finishes
  with `character_SV` (`0x8081A27C`) / `g3d/model.brres` (`0x8081A290`).
- **Per-variant palm model loader (1 body).** `0x8078D560` is the twin of
  [`fn_80779ca0`]: it formats `CS_W%d`/`CS_W%d_Palm`
  (`0x8081A3F0`/`0x8081A408`) with `byte[0x8042A24D] + 1` and mounts
  `g3d/model.brres` (`0x8081A3F8`) at `+0x18C`/`+0x1AC`/`+0x1B8`. Variant `1`
  mounts the base name and then binds the `_Palm` clip; every other variant
  mounts `_Palm` directly.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8063F120` | `fn_8063f120` | 428 | high |
| `0x80733F70` | `fn_80733f70` | 428 | high |
| `0x80746C20` | `fn_80746c20` | 424 | high |
| `0x8078C500` | `fn_8078c500` | 424 | high |
| `0x8078D560` | `fn_8078d560` | 428 | high |

## Deferred

None.

## Corrections and notes

- `0x80746C20` carries the usual `unaff_GQR0`/`ldexpf` guards. Its only
  GQR-0 accesses are the `psq_st f31,0x118(r1),0,0` save and reload, so the
  guards are compiler noise and are not emitted.
- Two words Ghidra prints as `F63_8`/`F63_12`/`F63_15` are ordinary
  double-precision opcode-63 forms, not paired-single maths: `0xFC400A10` is
  `fabs f2,f1` (10-bit XO `264`), `0xFC401018` is `frsp f2,f2` (XO `12`) and
  `0xFC00001E`/`0xFC20081E` are `fctiwz f0,f0`/`fctiwz f1,f1` (XO `15`). The
  `fabs` operand is `f31 - 0x800B30E0()`, which matches Ghidra's
  `ABS(dVar13 - dVar12)`.
- `0x80746C20`'s `xoris rX,rX,0x8000` + `0x43300000` pair subtracts
  `0x807CE470 = 0x4330000080000000` (`2^52 + 2^31`), i.e. the signed
  int-to-`f64` idiom, so `candidates[pick] as f32` is exact.
- `0x80746C20` keeps 50 candidate words, the same reservation as
  [`fn_80746f90`]'s own list; neither body bounds-checks the count, so the
  translation keeps the fixed 50-word array.
- `0x8063F120`'s `bc 12,6` at `0x8063_F174` tests `cr1`, set only by the
  `cmpi cr1,6,r6,0` at `0x8063_F14C`. That branch is inside the `r6 != 0` arm,
  so the test is always false and the `+0x27C` store is unconditional, exactly
  as the export prints it.
- `0x8078C500` and `0x8078D560` recover their receiver through the
  `0x802DCD6C`/`0x802DCD60` register-save thunks (`mr r25,r3` / `mr r29,r3`);
  Ghidra prints no parameters. The same recovery is used by [`fn_80790840`]
  and [`fn_80779ca0`].
- `0x8078C500` and `0x8078D560` pass the model word and the clip word as the
  two halves of one two-word stack pair to `0x801650D0`/`0x80166830`, so the
  blocks keep them in a single `[clip, model]` array and pass `&mut pair[1]` /
  `&mut pair[0]` in that order (the same idiom as [`fn_806f9e20`]).
- `0x8078C500` reads `0x8081A254`/`0x8081A260` as pointer words, not strings:
  they resolve to `"walk"` (`0x8081A24C`) and `"bros"` (`0x8081A258`). The two
  identical `"bros"` strings live at different addresses and are kept as the
  loader takes them.

## Verification

- The lane-D block type-checks standalone against stubbed helper signatures in
  a temporary `rustc --crate-type lib` harness outside the repo; no errors.
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch, offset and
  string was re-read from `work/rels/elf/d_basesNP.elf`.
- Only helpers that already exist in `src/game/bases.rs` are called
  (`platform::read_*`/`write_*`, `call0_f32`, `call_f32_u32`,
  `call_f32_u32x2`, `call_f32x2_u32x2`, `vmethod1`, `abi_call1!`..
  `abi_call6!`); every other callee (`0x8008E890`, `0x800B30E0`,
  `0x800B2EE0`, `0x80746F90`, `0x8019_5380`, `0x802E_17DC`, `0x800DF180`,
  the `0x8016_*` and `0x800F_*` helpers) has no Rust body in `bases.rs`.
