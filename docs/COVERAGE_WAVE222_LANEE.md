# Coverage: wave 222 lane E (`resource::archive`, `0x802CF6F0..0x802CFD00`)

## How to read this file

This document covers lane E of wave 222. All three bodies live in
`src/resource/archive.rs`; no source file was edited by this pass. The bodies
are staged in `work/wave222/laneE_block.rs` for the merge, and the three rows
are in `ledger/agents/wave222-laneE.csv`.

## Module completion

These are the last three open bodies of the `resource::archive` ledger module.
With this block **`resource::archive` reaches completion**: every entry the
canonical `main-gb` listing assigns to the module now has a translated
counterpart in `src/resource/archive.rs`, and no lane work remains for the
module. The three bodies close the `0x802CE740..0x802CFD00` draw-record family
that wave 222 lane A opened with the base initialiser `0x802CE740`; nothing
else in the module's address set is left deferred or stubbed.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 3 |
| Bytes of `functions.tsv` size | 1,268 |
| `translated` | 3 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 3 |
| Confidence `medium` | 0 |
| Deferrals | 0 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---:|---:|---|
| `0x802CF6F0` | [`fn_802cf6f0`] | initialise a `0x1C0`-byte draw record: vtable, `0xFFFFFFFF`, `0.0`/`1.0`, one flag | translated | high | 100 | `802cf6f0_FUN_802cf6f0.c` |
| `0x802CFBA0` | [`fn_802cfba0`] | construct the small draw record owning four `0x18`-byte texture panes | translated | high | 352 | `802cfba0_FUN_802cfba0.c` |
| `0x802CFD00` | [`fn_802cfd00`] | paint three channel panes and the fourth pane with two colour gradients | translated | high | 816 | `802cfd00_FUN_802cfd00.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing `blr`, not just
against the `functions.tsv` column:

| address | `functions.tsv` | ending | `end + 4` | note |
|---|---:|---|---|---|
| `0x802CF6F0` | 100 | `blr` at `0x802CF750` | `0x802CF754` | exact; two pad words before `0x802CF760` |
| `0x802CFBA0` | 352 | `blr` at `0x802CFCFC` | `0x802CFD00` | exact; next function starts immediately |
| `0x802CFD00` | 816 | `blr` at `0x802D002C` | `0x802D0030` | exact; next function starts immediately |

No row is a `psq_st` prologue artefact. `0x802CFD00` does save f27..f31 with
`psq_st`, but the Ghidra "unreachable block" warnings at
`0x802CFFF4..0x802D0014` are the matching `psq_l` restores in its epilogue, not
dead code; the body between the real prologue at `0x802CFD00` and the `blr` at
`0x802D002C` is fully covered by the translation.

## What the three bodies do

`0x802CF6F0` runs the base initialiser `0x802CE740` (wave 222 lane A), installs
the `0x80350480` vtable at `+0x28`, clears `+0x2C`, stores the `sdata2` word
`0x8042F244` (`0xFFFFFFFF`) at `+0x30`, writes the `f32` defaults `0.0`/`1.0`
from `0x8042F520`/`0x8042F524` at `+0x34`/`+0x38`, sets the `+0x1BC` flag and
returns `this`. Ghidra prints the two `stfs` stores as integer stores; the image
words `d03f0034`/`d01f0038` make them `f32`.

`0x802CFBA0` initialises the same base, installs the `0x803504A0` vtable, and
writes its parameter block: byte `+0x2C = 8`, `+0x44 = 0.5f`,
`+0x48 = +0x50 = +0x54 = 0.0f`, `+0x4C = 1.0f`, and the two integer `2`s at
`+0x5C`/`+0x60`. It then builds three `0x18`-byte panes at `+0x30..+0x38` with
`0x802C0A80(block, 0x40, 2, 6)` and one at `+0x3C` with
`0x802C0A80(block, 0x40, 0x40, 6)`, dispatches each pane's vtable slot `+0x0C`,
rebinds it with `0x802C1270(pane, 0)`, clears its `+0x0A`/`+0x0B` bytes, runs
`0x802CFD00` over the record and finally clears `*this`.

`0x802CFD00` is the painter. For each of the three channel panes it walks
`i in [0, width/2)`, forms `t = 1 - i/(width/2 - 1)`, raises it to `t`, `t^2`
or `t^4` on channels `1`/`2`, interpolates two colour words through
`0x802CEC60` and writes them into the texels `(i, 0)`, `(i, 1)`,
`(width - 1 - i, 0)` and `(width - 1 - i, 1)` with `0x802C1280`. For the fourth
pane it walks `row in [0, height)` and `k in [0, width/2)`, forms
`t = clamp((k/(width/2)) / (1 - row/height), 0, 1)`, raises it to the fourth
power and writes the mirrored texels `((width/2 - 1) - k, row)` and
`(width/2 + k, row)`. Each pane is published through `0x802C0C20`.

Both colour pairs start at the word returned by `0x802D0820` and end at
`0x802D0800` (bytes `00 FF 00 FF`, green) for the first word and `0x802D0810`
(bytes `FF 00 00 00`, red) for the second. `0x802CEC60(dst, from, to, t)` writes
one interpolated RGBA word into the four-byte cell the caller passes.

## Reconstruction notes

- **Register-save thunks.** `0x802CFD00` opens with `bl 0x802DCD68` and closes
  with `bl 0x802DCDB4`. The disassembly shows they only spill/restore `r24..r31`
  in the caller's `r11 - 0x20` area; `0x802DCD68` leaves `r3` unchanged, which
  is why Ghidra modelled `this = FUN_802dcd68()`. Both are omitted from the
  body, as in the other lanes' bodies, and `this` is the `r3` argument.
- **`u16`-to-`f64` bias.** Ghidra's `ldexpf`, `GQR0` and
  `CONCAT44(local_80, ...)` arms are the compiler's `0x43300000` `2^52` bias
  conversion. `0x8042F540` holds `2^52` and `0x8042F548` holds `2^52 + 2^31`
  (the companion that cancels the `xoris 0x8000` bias applied to `width/2 - 1`).
  The translation states the same value as a plain integer-to-`f32` conversion;
  the divisions are single precision in both phases (`fdivs`/`fsubs`), which is
  what the `f32` arithmetic below reproduces.
- **Null panes.** The shipped body guards only the pane *construction* against a
  failed `0x802B9210`; the vtable dispatch and `0x802C1270` run
  unconditionally. The translation keeps that order and documents the null
  hazard in the `# Safety` section instead of inventing a guard.
- **Call shapes.** `crate::platform` has no `f32`-argument call shape, so this
  lane adds one local helper, `call_u32x3_f32_E222`, for
  `0x802CEC60(dst, from, to, t)`.
- **Reuse.** `fn_802b9210`, `fn_802c0a80`, `fn_802c1270`, `fn_802c1280` and
  `fn_802c0c20` are called directly; `0x802CE740` is lane A's `fn_802ce740`,
  reached through the local address constant `FN_802CE740_E222` so this block
  compiles on its own (the same pattern `fn_802c0e30` uses for `0x802C0C20`).
  New constants carry the `_E222` suffix. `SDA2_BASE` (`0x80433080`) is the
  module's existing const.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, this
block was appended to the copy's `src/resource/archive.rs`, and
`cargo check --lib` there finished with exit code 0 and no new warnings. The
files in the repository were not modified by that check.

## Deferrals

None. All three bodies are complete; no control-flow arm, call or store was
dropped, and every callee outside this batch is called by absolute address.
