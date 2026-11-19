# Coverage - Wave 115 (`game::worldmap`, `0x80100000` bucket close-out)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export the body was transcribed from. `r2` is `SDA2_BASE` (`0x80433080`) and
`r13` is `SDA_BASE` (`0x8042F680`). The body reuses the module's existing
`G_80424558`, `FUN_802BF3D0`, `FUN_802BF650`, `FUN_801C5860`, `FUN_802BED10`
and `FUN_80107850` constants and adds only the callees and `r2` floats listed
below.

The bucket had exactly one untranslated entry at the start of this pass. That
entry is written here, which closes the `0x80100000` bucket (280 of 280
entries in `ledger/functions.csv`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 1 | 580 | 0 | 1 |

The row is `translated`/`medium`; no part of the body is a stub or partial.

## What this pass covers

Wave 111 added the two bounded-format text builders (`0x80107050`,
`0x80107170`) and the layout-record quad draw (`0x801097F0`) and left
`0x80107460` open because the export appears to read the virtual slot at
`local_E8 + 0x18` before writing `local_E8`. The frame layout resolves that
defect: `local_E8` is `local_100 + 0x18`, the exact word [`fn_80106ee0`]
fills with the `0x80321BD8` vtable. That vtable's `+0x18` entry is the
`0x801072D0` float setter, so the indirect call is reconstructible.

- **`0x80107460` - the message manager's draw pass (580 bytes).** Gated on the
  manager byte returned by `0x80107850` (`+0x04`). It builds a `0x20`-byte
  render-mode record (`0x802BFF30`), a camera record (`0x802BF3D0`, allocated
  here as `0x48` bytes because the constructor writes `+0x44` and the original
  frame recycles that word for `local_3C`), sets the viewport to
  `(0, 0, 640, 456)` (`0x802BFF70`) and commits it (`0x802C0110`). It then
  applies the camera (`0x802BF650`),
  restores the default light state (`0x802BED10`), clears the deferred render
  flag (`0x801C5860(0)`), and loads the shared scene matrix through the
  `0x802BE5D0` thunk to `0x801C9940`.
- **Backdrop.** The four floats at `r2 - 0x5E28`/`-0x5E24` are `-400.0f` and
  `400.0f`, so the quad is `(-400, -400)..(400, 400)`; the colour quad is
  `(0, 0, 0, 0xFF)`. `0x802BEF00` emits both triangles. The export shows only
  the four colour byte stores and drops every float read; the instruction
  stream at `0x802BEF00` reads the four floats and writes two triangles with a
  `0.0f` Z term.
- **Text selection.** `+0x1B0` is the raw column selector and `+0x1B4` the
  latched one. Values `4`, `0x0B` and `6` map to columns `0`, `1`, `2`; every
  other value keeps the default `4`, which [`fn_801079b0`] turns into the
  shared language-table pointer.
- **Text build and draw.** [`fn_80106ee0`] constructs the `0x80`-byte record
  (including the `0x64`-byte nested text object at `+0x1C`),
  [`fn_80106f80`] stores `this[0]` at `+0x64`, and
  [`wm_transform_scene_apply`] commits the transform. The record's first four
  bytes become white, the anchor is `(24.0f, -24.0f)`, and
  [`fn_80107170`] measures the line twice: the first call prints no variadic
  words, the second prints the line width in `f1`. The virtual `+0x18` setter
  then receives `-0.5 * width` and `0.5 * (bottom - top)`;
  [`fn_80107050`] formats and commits the text, [`wm_render_light_reset`]
  restores the channel state, and `0x8022DE40` destroys the nested text object.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80107460` | `fn_80107460` | 580 | medium |

## Confidence and the two export defects

The body is `medium`, not `high`, because the `4`/`6`/`0xB` selector roles and
the manager byte roles are inferred from the neighbouring message functions
(`0x80107340..0x80107C90`). Every call, offset, comparison, float operation and
constant is literal.

Two defects in the C export required the instruction stream in
`work/dol/main.elf`; neither changes the control flow:

1. **Dropped 0x802BEF00 arguments.** The export prints
   `FUN_802bef00(&local_130,&local_154)` as two opaque pointers and shows only
   the final FIFO colour store. The callee's instructions load four floats from
   `r3`, build the `(x0, y0)`, `(x1, y0)`, `(x1, y1)`, `(x0, y1)` vertices with
   a `0.0f` Z term, and write the four `r4` colour bytes before each vertex.
   The Rust body models the call as `(quad, colour)`.
2. **Thunk name.** The export renders the call at `0x801074E4` as
   `thunk_FUN_801c9940`; the instruction stream shows a branch to the
   four-byte thunk at `0x802BE5D0`, which branches to `0x801C9940`. The body
   names `0x802BE5D0` and keeps the target in its doc comment.

Three other observations are annotated `// observed:` in the body:

- `FUN_80107850` is called with no printed argument because `r3` still holds
  the incoming `this`; the body forwards `this`.
- The store of the mode pointer into the dead frame word `local_3C` is kept as
  a comment, not as a write to a named record.
- The first [`fn_80107170`] call has no variadic words in either the export or
  the instruction stream (`crxor` marker only), so the model supplies empty
  GPR/FPR arrays.

## Verification

- The address and 580-byte size match `work/ghidra/main-gb/export/functions.tsv`.
- The evidence file exists at
  `work/ghidra/main-gb/export/80107460_FUN_80107460.c`.
- The row has exactly one `pub unsafe fn fn_80107460` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; `0x80107460` was the
  remaining untranslated entry of the `0x80100000` bucket.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave115` reports `wave115-worldmap.csv: 1
  rows` and `OK`.

## Deliberate skips

None. The `0x80100000` bucket is closed by this pass.

[`fn_80106ee0`]: crate::game::worldmap::fn_80106ee0
[`fn_80106f80`]: crate::game::worldmap::fn_80106f80
[`fn_80107050`]: crate::game::worldmap::fn_80107050
[`fn_80107170`]: crate::game::worldmap::fn_80107170
[`fn_801079b0`]: crate::game::worldmap::fn_801079b0
[`wm_transform_scene_apply`]: crate::game::worldmap::wm_transform_scene_apply
[`wm_render_light_reset`]: crate::game::worldmap::wm_render_light_reset
