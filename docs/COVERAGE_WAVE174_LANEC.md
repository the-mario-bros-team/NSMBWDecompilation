# Coverage - Wave 174, lane C (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane C block) | `game::bases` | 5 | 1,892 | 5 | 0 |

Lane C of wave 174: five REL bodies spanning four address ranges. All five are
translated; nothing is deferred.

## What this pass covers

- **Boss-block tilt/height body (1 body).** `0x80713B00` advances the `+0x6D6`
  halfword by `0xB6` towards the interval anchored at `+0x6D4`, calling the
  signed sampler [`fn_807137a0`] (endpoint `0x4000`) when
  `(value + 0x4000) <= 0x7FFF`, i.e. the advanced value sits in the half-window
  centred on zero, and the unsigned [`fn_80713690`] (endpoint `0xC000`)
  otherwise; both arms set the same crossing flag. It then refreshes the
  `+0x584` clip with
  `0x801680E0`, dispatches slot `+0x1C` of the `+0x544` interface, and writes
  `BSS[0x8082F8C4] + (y - 160.0) + (32.0 - |y|) * sin(angle / 256.0)` to `+0xB0`.
  This is the `0x80713250` sibling from the same GQR family; wave 110 deferred
  both as "GQR bodies", which the current decoder disproves.
- **Area element-map rebuild (1 body).** `0x80773D70` walks every
  `0x16518`-stride area row and all `0xC0` element slots, priming
  `0x800FB040`/`0x800FAE40`, resolving each record with
  `0x800F56B0`/`0x800F9530`, and calling the row writer
  `0x80773EF0(1.0, this, row, slot, side)` for both "word set but slot clear"
  cases and for `0x800FD310`. The `0x800E0380(current player, -1)` call is the
  same idiom used throughout `bases.rs`.
- **Manager id gate (1 body).** `0x8078EB40` tests `manager[+0x54]` /
  `manager[+0x50]` against the `(7, 0x17)` pair and the `0x17`/`0x25` special
  cases, with `0x800FC970(7)` as the outer reject and the `+0x60 == 2` mode word
  and `+0x6D` byte as the secondary guards.
- **Five-element transform push (1 body).** `0x80764C80` builds the
  `+0xDF8..+0xE00` position plus three per-element biases (selected by a
  `cntlzw` "first element" flag) into `+0x7C` with `0x801C0BD0`, applies the
  `+0x100`/`+0x104` angles and the `+0xE08` low halfword with `0x8016EF50`, and
  copies the transform and `+0xDC` colour into the five panes at `+0x38C` and
  `+0x1A8 + (i-1) * 0x40`. This is the same call sequence as
  [`fn_80768e40`], extended to five elements.
- **Page-arrow channel push (1 body).** `0x8062D660` belongs to the
  `dMultiCourseSelect_c` family: it selects the running arrow clip among
  `0x0F`/`0x10`/`0x11`, reads that channel's `+0x08` float, and pushes it into
  the left/right arrow channel (`0x258`/`0x2A8`, `0x280`/`0x2D0`) behind the
  `+0x513`/`+0x514` latches and the mirrored `+0x520` flag — the same layout as
  [`fn_8062d370`] and [`fn_8062de90`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80713B00` | `fn_80713b00` | 376 | high |
| `0x80773D70` | `fn_80773d70` | 376 | high |
| `0x8078EB40` | `fn_8078eb40` | 376 | high |
| `0x80764C80` | `fn_80764c80` | 380 | high |
| `0x8062D660` | `fn_8062d660` | 384 | high |

## Corrections and notes

- `0x80713B00`: the export's `unaff_GQR0`/`unaff_GQR3` "dequantise" blocks are
  the prologue `psq_st` spill pair and the `psq_l` of the `+0x6D6` halfword
  through GQR3 (type 5, unsigned 16-bit). The `fabs`/`frsp`/`fsubs` triple at
  `0x8071_3BDC..0x8071_3BF8` is a plain `|y|` and `32.0 - |y|`. The two
  `fmuls` words are `0xEC200072` = `fmuls f1,f0,f1` and `0xEC1E0072` =
  `fmuls f0,f30,f1` (A-form: destination `(w>>21)&31`, first source
  `(w>>16)&31`, third source `(w>>6)&31`), i.e. `1/256 * angle` and
  `(32 - |y|) * sin`. The `+0xB0` expression therefore keeps the
  `sin(angle / 256.0)` product; reading the third source from the `(w>>11)` slot
  yields `f0` in both words and silently drops it, which is the artefact that
  left this body untranslated in wave 110.
- `0x80713B00`: `0x8082_F8C4` is a `f32` in the REL's BSS workspace block at
  `0x8082_F8C0` (segment 5, `filesz 0`), not a ROM constant. It is read at run
  time like the other `0x8082_Fxxx` state descriptors already used in
  `bases.rs`; its runtime value is not recoverable from this REL.
- `0x80773D70`: Ghidra declares the body `void` because the first call is the
  `_savegpr_22` helper `0x802DCD60`; `r3` is the real argument and is
  `mr`'d into `r22` right after the helper returns. The save/restore pair is
  not emitted. `0x800F9530` is called with the record returned by
  `0x800F56B0` in `r3`, following the `r3` pass-through chain already documented
  for [`fn_80772bd0`].
- `0x80764C80`: the `i == 0` selector is `cntlzw` + `rlwinm 27,5,31`
  (`0x8076_4CBC..0x8076_4CC4`), giving `1.0` for element `0` and `0.0` for the
  rest; it scales three biases at `0x8081_599C`/`0x8081_59A0`/`0x8081_59A4`,
  which are zero in the REL image. The `lfd` of `0x807C_EF18` is the `2^52`
  int-to-double magic the compiler uses for that selector, not a real constant.
- `0x8078EB40`: `0x800FC0A0` has no recoverable prototype; the raw words leave
  `r3` at the zero returned by the preceding `0x800FC970(7)` probe, so the body
  forwards that zero explicitly instead of using a zero-argument call.
- `0x8062D660`: `0x801637D0` writes one float into an arrow animation channel
  and `0x801644D0` commits the channel that owns it (both roles are inferred at
  every call site of this class); the loaded channel word is passed to
  `0x801637D0` and the `arrows + offset` address to `0x801644D0`, exactly as
  [`fn_8062de90`] does.

## Deferred

None.
