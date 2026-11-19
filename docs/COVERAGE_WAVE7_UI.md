# Coverage - wave 7 HUD/menu helpers (`0x80014F80..0x8017E920`)

## How to read this file

- **Scope** - the fourth UI translation pass. It adds 23 functions to the two
  existing semantic modules `src/ui/hud.rs` and `src/ui/menu.rs` and creates no
  staging module.
- **Addresses** are virtual `sys/main.dol` addresses (image base
  `0x80004000`), not file offsets.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; each row names its file.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = full control flow and every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a
  function whose bytes and control flow are exact but whose table bounds,
  callee roles or object layout were inferred from the surrounding code.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the new bodies connect to the wave-1..4 HUD and
  home-button-menu code already in the two destination files.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/hud.rs` | 9 |
| functions added to `src/ui/menu.rs` | 14 |
| functions added in total | 23 |
| bytes translated | 2,896 |
| `translated` / `partial` / `stub` | 23 / 0 / 0 |
| high confidence | 20 |
| medium confidence | 4 |
| new staging modules | 0 |
| files written | `src/ui/hud.rs`, `src/ui/menu.rs`, this document, `ledger/agents/wave7-ui.csv` |

Address span: `0x80014F80..0x8017E920`. The selected bodies are the remaining
small counter, layout, dialog-widget and release helpers around the already
translated scenes; no large state-machine body was guessed.

## Function table

### `ui::hud` - coin counter and warning-window helpers

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80014F80` | `fn_80014f80` | release a `pointGet` scene | translated | high | `80014f80_FUN_80014f80.c` |
| `0x80015210` | `fn_80015210` | write the coin-counter colour pair through virtual slot `+0x28` | translated | medium | `80015210_FUN_80015210.c` |
| `0x800153B0` | `fn_800153b0` | write one digit colour into its record | translated | medium | `800153b0_FUN_800153b0.c` |
| `0x800154E0` | `fn_800154e0` | advance the counter tick and select the next digit slot | translated | high | `800154e0_FUN_800154e0.c` |
| `0x80015580` | `fn_80015580` | flash the coin-counter colour pair | translated | high | `80015580_FUN_80015580.c` |
| `0x80015CE0` | `fn_80015ce0` | open the battery warning at a position and mode | translated | high | `80015ce0_FUN_80015ce0.c` |
| `0x80016E80` | `fn_80016e80` | release an `errorInfo` record | translated | high | `80016e80_FUN_80016e80.c` |
| `0x80017440` | `fn_80017440` | close the error window when it is open | translated | high | `80017440_FUN_80017440.c` |
| `0x800174C0` | `fn_800174c0` | play the error open animation for the selected button | translated | medium | `800174c0_FUN_800174c0.c` |

### `ui::menu` - home-menu grid and dialog widgets

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8016FE40` | `fn_8016fe40` | release one grid entry | translated | high | `8016fe40_FUN_8016fe40.c` |
| `0x8016FE90` | `fn_8016fe90` | create the grid resource | translated | high | `8016fe90_FUN_8016fe90.c` |
| `0x80170110` | `fn_80170110` | release every grid entry and reset the list | translated | high | `80170110_FUN_80170110.c` |
| `0x80170670` | `fn_80170670` | compute grid dimensions, page counts and cell area | translated | high | `80170670_FUN_80170670.c` |
| `0x801706E0` | `fn_801706e0` | page number for a coordinate | translated | high | `801706e0_FUN_801706e0.c` |
| `0x80170700` | `fn_80170700` | in-page cell offset | translated | high | `80170700_FUN_80170700.c` |
| `0x80170730` | `fn_80170730` | global cell index with bounds check | translated | high | `80170730_FUN_80170730.c` |
| `0x80170830` | `fn_80170830` | write one four-bit cell value | translated | high | `80170830_FUN_80170830.c` |
| `0x80170900` | `fn_80170900` | flush the whole grid buffer | translated | high | `80170900_FUN_80170900.c` |
| `0x8017D3C0` | `fn_8017d3c0` | store the three dialog-mode bytes | translated | high | `8017d3c0_FUN_8017d3c0.c` |
| `0x8017D3D0` | `fn_8017d3d0` | store the dialog timer | translated | high | `8017d3d0_FUN_8017d3d0.c` |
| `0x8017D6F0` | `fn_8017d6f0` | read the current message id | translated | high | `8017d6f0_FUN_8017d6f0.c` |
| `0x8017E5D0` | `fn_8017e5d0` | clear the vertex-row counters | translated | high | `8017e5d0_FUN_8017e5d0.c` |
| `0x8017E920` | `fn_8017e920` | enable and commit the dialog pane group | translated | high | `8017e920_FUN_8017e920.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### `pointGet` counter (`0x80014F80..0x80015580`)

The counter reuses the `pointGet` scene layout already documented in
`src/ui/hud.rs`. `fn_80015210` and `fn_80015580` own the two colour-pair
tables: five RGBA words at `0x80351EA8`/`0x80351EBC`, and two four-byte pairs
in the SDA at `r13 - 0x5A70` / `r13 - 0x5A60`. Each table is built byte by
byte behind a one-byte guard and then read back as words, which is why the
Rust code stores bytes and loads `u32`.

`fn_800153B0` uses a four-entry digit table at `0x80351ED0` and writes the
four selected bytes as four zero-extended halfwords at `+0x18..+0x1E` of the
record returned by virtual slot `+0x68`. `fn_800154E0` is the ten-tick
counter: after the tick wraps it scans the four state slots at
`+0x388 + i*4` of the object pointed to by `r13 - 0x5704`, looking for the
value `3`, then refreshes the digit through `0x80015200`.

### Battery and error windows (`0x80015CE0..0x800174C0`)

`fn_80015CE0` hides the battery pane, honours the debug gate in
`r13 - 0x5700` bit `0x40`, copies the two-word position, and clears the three
interpolation words to the SDA2 zero float at `F32_SDA2_ZERO`
(`0x8042B1A8`). The three release bodies `fn_80016e80`, `fn_80017440` and
`fn_80014f80` keep the original destructor/thunk order.

`fn_800174C0` builds a 40-byte NW4R animation argument block with
`0x800CDDB0`, starts two keyframed animations from `0x802EDF48`/`0x802EDF64`,
advances the layout to frame `0x10`, and hides the selected pane at `+0x208`.

### Home-menu grid (`0x8016FE40..0x80170900`)

These bodies describe one grid descriptor:
`{ width, height, cell_w, cell_h, area, columns, rows, pages, buffer }` at
`+0..+0x24`. `fn_80170670` computes `area = cell_w * cell_h`,
`columns = ceil(width / cell_w)`, `rows = ceil(height / cell_h)` and
`pages = columns * rows`; the two zero-dimension guards match the original
(and leave the corresponding field untouched).

`fn_801706e0`/`fn_80170700` map a pixel coordinate to a page and an in-page
offset; `fn_80170730` combines them into a global index and returns `-1` out
of bounds. `fn_80170830` packs two four-bit cells per byte with the
`((index + 1) * 4) & 4` shift, and `fn_80170900` flushes
`height * rows * columns * width / 2` bytes.

### Dialog widgets (`0x8017D3C0..0x8017E920`)

`fn_8017d3c0` and `fn_8017d3d0` are the two setters used by the existing
dismiss path; `fn_8017d6f0` reads the message id from `*(+0xC) + 8`.
`fn_8017e5d0` clears the vertex-row record that
`dialog_init_vertex_rows`/`dialog_copy_vertex_rows` fill.

`fn_8017e920` enables pane `9`, optional pane `0xB` (when the secondary
object is non-null) and panes `0xD..0xD + count - 1`, then commits the same
groups through `0x801C40B0`. This is the pane-visibility batch that the
dialog `bind` path calls before drawing the message.

## Unknowns and limits

1. **Colour-table bounds.** The five-word and four-entry bounds are inferred
   from the number of bytes initialised in each lazy block; the export does
   not carry the array declarations.
2. **SDA flags.** `r13 - 0x5A78`, `-0x5A74`, `-0x5A73`, `-0x5A72`,
   `-0x5A71`, `-0x5A68` and `-0x5700` are read/written exactly, but their
   names are not recovered. The `-0x5700` bit `0x40` is treated as a debug
   gate because of its use in `0x80015CE0`.
3. **`r2` floats.** `fn_80015CE0` copies the word at `F32_SDA2_ZERO`; the
   value is not interpreted as a float. `fn_800174C0` does not read `r2`.
4. **Grid descriptor roles.** The field meanings (width/height/cell/pages)
   are inferred from the arithmetic in `0x80170670` and the bound checks in
   `0x80170730`. The buffer field at `+0x24` and its nibble packing are
   observed.
5. **Callee bodies.** `0x80015200`, `0x8015F800`, `0x800CDDB0`,
   `0x80228F80`, `0x801D4710`, `0x801D4380`, `0x80228FF0`, `0x80228DD0`,
   `0x801705E0`, `0x801AC500`, `0x801C4070`, `0x801C37C0` and `0x801C40B0`
   are called by address; their bodies are outside this pass.
6. **No new staging.** Every selected body already had a complete Ghidra
   export; no skeleton or placeholder function was created.

## Verification

```text
cargo check          # exits 0
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Only `src/ui/hud.rs`, `src/ui/menu.rs`, this document and
`ledger/agents/wave7-ui.csv` were written. `src/lib.rs`, every `mod.rs`,
`Cargo.toml`, `src/platform.rs`, the generated coverage files and the other
lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x80015C10` | 208 B | coin motion update; completes the `pointGet` per-frame path | `80015c10_FUN_80015c10.c` |
| 2 | `0x80016430` | 596 B | battery per-frame body | `80016430_FUN_80016430.c` |
| 3 | `0x80017630` | 560 B | error state-machine body | `80017630_FUN_80017630.c` |
| 4 | `0x8017D520` | 452 B | dialog spline/keyframe interpolation | `8017d520_FUN_8017d520.c` |
| 5 | `0x8017DCC0` | 728 B | dialog animation-task update | `8017dcc0_FUN_8017dcc0.c` |
