# Coverage - Wave 96 (`game::stage`, the `0x80070000` bucket)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the `function`
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The `evidence`
column of `ledger/agents/wave96-stage.csv` is the Ghidra C export each body was
written from. Roles the export only hints at stay marked `inferred` or `unknown`
in the body.

This pass works the open entries of the `0x80070000` bucket: the BG-scene
camera bounds, the ground/slope scans that use them, and the per-area scene
caches. Two record layouts carry the pass:

* the **BG scene** (`+0x8FE48..+0x90A1C`): the camera/zoom fields at
  `+0x8FE4C..+0x8FEAC`, the area selector bytes `+0x90024`/`+0x90079`/
  `+0x9007A`/`+0x900C6`/`+0x900FE`, the view scale `+0x9000C`, the effect
  height `+0x900B8` and the per-cell tables cleared by [`fn_8007bef0`];
* the **area record** sampled through `0x8005ED90` and resolved through
  `0x8008E310`: the min/max x/z window that [`fn_80079780`] folds into
  `+0x8FE54`/`+0x8FE58`.

[`fn_800789a0`] and [`fn_80078b40`] are the anchors: the wave-5
`bg_scene_wrap_x` already called those two addresses through `call1_f32`, which
is what fixes their single-precision return convention. The four sibling
getters then make [`fn_8007ba70`], [`fn_8007bc40`], [`fn_8007bd40`] and
[`fn_8007c7d0`] readable, and [`fn_8007bef0`] supplies the scene state those
helpers consume.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 20 | 8200 | 4 | 16 |

No body in this pass is `low` confidence and none is `partial`; every row is
`translated`.

## What this pass covers

- **Camera bounds** (`0x800789A0`, `0x80078A70`, `0x80078B40`, `0x80078C10`).
  Four 200-byte getters that seed from `+0x8FE64`/`+0x90030`/`+0x8FE68`/
  `+0x90034` and widen or narrow the bound by
  `scale * ratio * BG_colour_A` while the zoom is under `FUN_80079060`'s
  ratio. `+0x900FE` mode `1`/`4` skips the adjustment.
- **Slope and ceiling helpers** (`0x8007BA70`, `0x8007BC40`, `0x8007BD40`,
  `0x8007C7D0`). The trigger-height clamp around `+0x8FE90`, the rectangle
  depth test, the two-scale rectangle solve with the `+0x90038`/`+0x9003C`
  limits, and the two-player depth nudge.
- **Column and height scans** (`0x80072180`, `0x80075560`, `0x80076530`,
  `0x800775A0`). The packed 20.12 x-span probe, the upward `0x800EB930` walk
  with its `0x200` downward arm, the `0x800757B0` ground fallback and the
  four-tier height classifier, and the single-cell tile getter with its
  optional `0x80081AB0` conversion.
- **Scene integration** (`0x800714D0`, `0x80079780`, `0x80079BF0`,
  `0x8007BEF0`, `0x8007E0E0`). The collision-cell validation in front of an
  actor, the four-sample area window, the area-effect height and curve
  selection, the full scene/table rebuild, and the corrected camera z.
- **Small helpers** (`0x80072BC0`, `0x80072CB0`, `0x8007FBF0`). The
  `Wm_en_spsmoke` trigger effect, the per-frame position mirror, and the
  four scaled rectangle writes.

The public entry points are [`fn_800714d0`] (collision landing test),
[`fn_80075560`] (vertical scan), [`fn_8007ba70`] (trigger height clamp) and
[`fn_8007bef0`] (scene rebuild).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800714D0` | `fn_800714d0` | 716 | medium |
| `0x80072180` | `fn_80072180` | 316 | medium |
| `0x80072BC0` | `fn_80072bc0` | 240 | medium |
| `0x80072CB0` | `fn_80072cb0` | 76 | medium |
| `0x80075560` | `fn_80075560` | 488 | medium |
| `0x80076530` | `fn_80076530` | 248 | medium |
| `0x800775A0` | `fn_800775a0` | 100 | medium |
| `0x800789A0` | `fn_800789a0` | 200 | medium |
| `0x80078A70` | `fn_80078a70` | 200 | high |
| `0x80078B40` | `fn_80078b40` | 200 | high |
| `0x80078C10` | `fn_80078c10` | 200 | high |
| `0x80079780` | `fn_80079780` | 596 | medium |
| `0x80079BF0` | `fn_80079bf0` | 644 | medium |
| `0x8007BA70` | `fn_8007ba70` | 304 | medium |
| `0x8007BC40` | `fn_8007bc40` | 256 | medium |
| `0x8007BD40` | `fn_8007bd40` | 432 | medium |
| `0x8007BEF0` | `fn_8007bef0` | 2272 | medium |
| `0x8007C7D0` | `fn_8007c7d0` | 372 | medium |
| `0x8007E0E0` | `fn_8007e0e0` | 136 | medium |
| `0x8007FBF0` | `fn_8007fbf0` | 204 | high |

Address range: `0x800714D0..0x8007FBF0` (twelve runs, 20 bodies, 8200 bytes).

## Deliberately skipped

The bucket still holds 42 untranslated functions after this pass. Skipped on
purpose:

- **GX-list split-return family** (35 entries). Every one opens with a
  `0x802DCD50`..`0x802DCD74` helper and then consumes one or both halves of
  the helper's `undefined8` result as incoming `r3`/`r4`. The export does not
  recover a consistent signature across the family (some read the low half as
  a record, some the high half, some both), so a body would have to invent the
  argument list. The affected addresses are `0x80070130`, `0x80070EF0`,
  `0x80071060`, `0x800717A0`, `0x80071A60`, `0x80071DC0`, `0x800722C0`,
  `0x80072440`, `0x80072790`, `0x800728C0`, `0x80073290`, `0x80073670`,
  `0x80073BC0`, `0x80073FC0`, `0x80074470`, `0x80074AF0`, `0x80075820`,
  `0x80075CA0`, `0x80075FD0`, `0x800766F0`, `0x800768D0`, `0x80077B40`,
  `0x80077EB0`, `0x80078300`, `0x80078670`, `0x80079E80`, `0x8007A6C0`,
  `0x8007AC40`, `0x8007CA90`, `0x8007CD70`, `0x8007E4E0`, `0x8007FA40`,
  `0x8007FFA0`, plus `0x80077730`/`0x80077860`, which call the same helpers
  and then read stack halfwords the export never writes.
- **Dropped or unrecoverable argument lists** (`0x80070530`, `0x800709C0`,
  `0x80071210`, `0x800776F0`, `0x80077700`, `0x800777B0`, `0x80077810`).
  `0x80070530` calls `0x8005F900()` with its player index dropped, so the
  height branch cannot be fixed. `0x800709C0` takes two `undefined8` pairs and
  calls `0x80070BA0()` with no arguments. `0x80071210` keeps its sine/cosine
  input in a GQR3 stack halfword (`uStack_34`) that the export never writes;
  it was drafted and then removed rather than guessed. `0x800776F0` is a
  4-byte thunk. `0x80077700` returns `extraout_r4` from `0x80077650`, a value
  that callee does not return. `0x800777B0` and `0x80077810` call
  `0x80077730` with its argument list dropped and then read uninitialised
  stack halfwords.

None of these is counted as translated in the CSV.

## Verification

- Every address and byte size matches `functions.tsv` (20 rows, 8200 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/stage.rs`.
- No address in this CSV was already in `ledger/functions.csv`; the 20
  addresses were chosen from the 62 open entries of the `0x80070000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave96` prints `OK`; the wave-96 stage rows
  are clean.
