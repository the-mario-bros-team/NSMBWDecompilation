# Coverage - wave 26 HUD/menu helpers (`0x80019A40..0x801750E0`)

## How to read this file

- **Scope** - the ninth UI translation pass. It adds 25 functions to the two
  existing semantic modules `src/ui/hud.rs` and `src/ui/menu.rs`; no staging
  module and no new `.rs` file is created.
- **Addresses** are virtual `sys/main.dol` addresses (image base `0x80004000`),
  not file offsets. The CSV uses the plain eight-digit hex form.
- **Evidence** is the Ghidra C export under
  `work/ghidra/main-gb/export/`; every row names its file.
- **Status** follows `docs/RUST_STYLE.md`:
  `translated` = full control flow and every call accounted for;
  `partial` = flow and major effects present, one operand not literal;
  `stub` = address/role only.
- **Confidence** measures the evidence, not code quality. `medium` marks a body
  whose flow and stores are exact but whose virtual-call operand, argument
  split or data role was inferred from the surrounding bodies.
- Start with the **summary**, then the **function tables**. The **cluster
  notes** explain how the nunchuk teardown bodies connect to the warning scene
  and how the widget setters connect to the wave-21 tables.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/hud.rs` | 7 |
| functions added to `src/ui/menu.rs` | 18 |
| functions added in total | 25 |
| bytes translated | 1,512 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 20 |
| medium confidence | 5 |
| new staging modules | 0 |
| files written | `src/ui/hud.rs`, `src/ui/menu.rs`, this document, `ledger/agents/wave26-ui.csv`, `ledger/functions.csv` |

Address span: `0x80019A40..0x801750E0`. The HUD half closes the nunchuk warning
scene: two deleting destructors, the `+0x18` release path, the close transition,
the sorted-list insert, the layout teardown and the start/sound marker. The menu
half adds the grid and widget setters, the two list helpers around the widget
table, the controller-hook installer, the animation-record speed updater and the
first three pause/home-button wrappers.

## Function table

### `ui::hud` - nunchuk release, close and start bodies

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80019A40` | `fn_80019a40` | conditional delete of a nunchuk record | translated | high | `80019a40_FUN_80019a40.c` |
| `0x80019A80` | `fn_80019a80` | twin conditional delete | translated | high | `80019a80_FUN_80019a80.c` |
| `0x80019B20` | `fn_80019b20` | release the `+0x18` sub-object and delete | translated | high | `80019b20_FUN_80019b20.c` |
| `0x80019D70` | `fn_80019d70` | close the open transition and layout | translated | medium | `80019d70_FUN_80019d70.c` |
| `0x80019DE0` | `fn_80019de0` | insert into the sorted list when open | translated | high | `80019de0_FUN_80019de0.c` |
| `0x80019E00` | `fn_80019e00` | tear down the layout at `+4` | translated | high | `80019e00_FUN_80019e00.c` |
| `0x80019FC0` | `fn_80019fc0` | mark started, hide layout, play sound `0x82` | translated | high | `80019fc0_FUN_80019fc0.c` |

### `ui::menu` - grid, widget, record and wrapper bodies

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8016FFC0` | `fn_8016ffc0` | store the grid's `+4` word | translated | high | `8016ffc0_FUN_8016ffc0.c` |
| `0x80170630` | `fn_80170630` | create the grid buffer object | translated | medium | `80170630_FUN_80170630.c` |
| `0x80170CD0` | `fn_80170cd0` | destroy a grid object and optionally free it | translated | high | `80170cd0_FUN_80170cd0.c` |
| `0x80171DE0` | `fn_80171de0` | read one list entry's value word | translated | high | `80171de0_FUN_80171de0.c` |
| `0x80172000` | `fn_80172000` | close every pane in the list | translated | medium | `80172000_FUN_80172000.c` |
| `0x80172180` | `fn_80172180` | store one byte at `+0x24` | translated | high | `80172180_FUN_80172180.c` |
| `0x80172E70` | `fn_80172e70` | store the widget's `+4` word | translated | high | `80172e70_FUN_80172e70.c` |
| `0x80172E90` | `fn_80172e90` | store the widget's `+0x24` word | translated | high | `80172e90_FUN_80172e90.c` |
| `0x801731F0` | `fn_801731f0` | install the two channel hooks and connection byte | translated | high | `801731f0_FUN_801731f0.c` |
| `0x801734A0` | `fn_801734a0` | clear three widget words | translated | high | `801734a0_FUN_801734a0.c` |
| `0x801734C0` | `fn_801734c0` | copy the two default words into a widget | translated | high | `801734c0_FUN_801734c0.c` |
| `0x801734E0` | `fn_801734e0` | reset a record and install its callback | translated | high | `801734e0_FUN_801734e0.c` |
| `0x80173610` | `fn_80173610` | clear the widget hold bytes | translated | high | `80173610_FUN_80173610.c` |
| `0x80174CC0` | `fn_80174cc0` | update both record pools from a level value | translated | medium | `80174cc0_FUN_80174cc0.c` |
| `0x80174F30` | `fn_80174f30` | release-build print, menu reset and pane build | translated | medium | `80174f30_FUN_80174f30.c` |
| `0x80174FD0` | `fn_80174fd0` | apply the compact/full button mode | translated | high | `80174fd0_FUN_80174fd0.c` |
| `0x801750A0` | `fn_801750a0` | store the menu timer word | translated | high | `801750a0_FUN_801750a0.c` |
| `0x801750E0` | `fn_801750e0` | initialise the sound/menu manager | translated | high | `801750e0_FUN_801750e0.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Nunchuk teardown and start (`0x80019A40..0x80019FC0`)

The seven HUD bodies finish the scene whose constructor (`0x80018CA0`), layout
builder (`0x80018F20`), state release (`0x800198A0`) and animation pair
(`0x80019E30`/`0x80019EF0`) were translated in waves 12, 19 and 21.

`0x80019A40` and `0x80019A80` are byte-identical deleting destructors for the
two record sub-objects: a non-null guard, a positive delete-flag guard and one
`0x802B9280(this)` call. `0x80019B20` is the variant that first releases the
`+0x18` sub-object through `0x8015F9F0` and preserves the original's redundant
`this + 0x18 != 0` comparison.

`0x80019D70` is the close transition. Its `+0x1E5` open byte gates three calls:
virtual slot `+0x10` of the object at `+0x19C` (printed with no operand), the
layout teardown `0x800C9560(this + 4)`, and virtual slot `+0x10` of the object
at `+0x0C` with `this + 4` as the only argument.

`0x80019DE0` inserts `this + 4` into the sorted list through `0x80163850` only
while `+0x1E5` is set. `0x80019E00` is the unconditional layout teardown
`0x800C9930(this + 4)`. `0x80019FC0` sets the start byte `+0x1E6`, puts the
layout in mode `2` with `0x800C92F0(this + 4, 2, 0)` and plays sound `0x82`
with flag `1` through the global audio handle at `SDA_BASE - 0x51F8`.

### Grid and widget setters (`0x8016FFC0..0x80173610`)

The grid descriptor documented in wave 7 is `{ width, height, cell_w, cell_h,
area, columns, rows, pages, buffer }` at `+0..+0x24`. `0x8016FFC0` stores the
`+4` word; `0x80170630` creates the buffer object with `0x801D4380(a, b, 0)`
and stores it at `+0`; `0x80170CD0` runs `0x8016DCA0(this, 0)` for every
non-null object and frees it only when the delete flag is positive.

The widget table is `0x80377F88[4]`, with the record layout documented in wave
21. The new setters write the `+4` speed word (`0x80172E70`) and the `+0x24`
callback word (`0x80172E90`); `0x801734A0` clears `+0x10/+0x14/+0x18`,
`0x801734C0` copies the two defaults from `0x802F5A6C/0x802F5A70`, and
`0x80173610` clears the hold bytes `+0x42/+0x43`.

`0x801731F0` is the important body. It selects channel `0`, installs
`fn_80172ea0` through `0x801E1060` and stores the handle at `+0x24`; selects
channel `1`, installs `fn_80172fb0` through `0x801E10F0` and stores the handle
at `+0x28`; sets the enabled byte `+0x44`; and calls `0x801E0F40` to derive the
connection byte `+0x40` (`0` -> `1`, `-1` -> `0`, other results leave it alone).

### Record and menu wrappers (`0x80174CC0..0x801750E0`)

`0x80174CC0` is the animation-record speed updater. A live manager at
`0x80377FA0` and a level above `0x802F5AA0` select
`scale_a * scale_b * curve(level)`; otherwise the offset is the fixed `-0x388`.
The result is clamped to `[-0x388, 0x3C]` and applied to every active record in
the four-record pool at `+0` and the three-record pool at `+0xB8B0` through
`0x801948C0`. The active bytes are `+0x2E1C` and `+0xE6CC` (the latter is
`0xB8B0 + 0x2E1C`).

`0x80174F30` is the release-build path: it prints the `"<< RVL_SDK - HBM
release build: ..."` string at `0x8032A300`, fetches the live menu through
`0x80175500` and calls the still-untranslated pane builder `0x80175F70`. The
three wrappers `0x80174FD0`, `0x801750A0` and `0x801750E0` fetch the live menu
and forward the argument to `0x8017CFE0`, `0x8017D3D0` and `0x801748E0`
respectively.

### Small list helpers (`0x80171DE0..0x80172000`)

`0x80171DE0` resolves `0x8018A6E0(this + 8, index)` and returns the entry's
`+4` word. `0x80172000` walks the `u16` count at `+0x10`, resolves each entry
the same way and calls virtual slot `+0x10` of the object stored at `entry + 4`.

## Unknowns and limits

1. **Virtual calls without operands.** `0x80019D70` and `0x80172000` contain
   virtual calls that Ghidra printed with no arguments. They are translated as
   zero-argument calls, exactly as printed, and marked `medium`. If a later
   caller proves that `r3` carried the object pointer, the operand should be
   added without changing the call order.
2. **Curve identity.** `0x80174CC0` calls the 4-byte thunk `0x802E8618`, whose
   body is `0x802E5490` (276 bytes, exponent split plus an SDA2 polynomial).
   The Rust body calls `0x802E5490` directly with the recovered `f64` argument
   and labels the routine `medium`; the control flow, clamps and both loops are
   literal.
3. **Grid resource arguments.** `0x80170630` forwards two opaque words to
   `0x801D4380`; the export does not name them, so the Rust signature keeps
   `a`/`b` and the role is marked `medium`.
4. **Setters' object roles.** The offsets of `0x8016FFC0`, `0x80172180`,
   `0x80172E70`, `0x80172E90` and `0x801734A0..0x80173610` are observed; the
   labels "grid", "widget" and "hold byte" follow the neighbouring wave-7 and
   wave-21 bodies, not a recovered header.
5. **Debug string.** The release-build string at `0x8032A300` is read from the
   binary image; its exact format is data and is not interpreted.
6. **Callee bodies.** `0x8015F9F0`, `0x802B9280`, `0x800C9560`, `0x800C9930`,
   `0x800C92F0`, `0x80195380`, `0x80163850`, `0x8016DCA0`, `0x8018A6E0`,
   `0x801D4380`, `0x801948C0`, `0x801E0F40`, `0x801E1060`, `0x801E10F0`,
   `0x801E5450`, `0x801AB1C0`, `0x802E5490` and `0x80175F70` are called by
   address; their bodies are outside this pass.

## Verification

```text
cargo check          # exits 0; no diagnostic references hud.rs or menu.rs
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

The crate check was rerun after the concurrent edits in other lanes settled;
it exits 0 and no diagnostic references `src/ui/hud.rs` or `src/ui/menu.rs`.
`doc-lint` passes with 0 violations.

Only `src/ui/hud.rs`, `src/ui/menu.rs`, this document,
`ledger/agents/wave26-ui.csv` and the 25 matching rows appended to
`ledger/functions.csv` were written. `src/lib.rs`, every `mod.rs`,
`Cargo.toml`, `src/platform.rs`, the generated coverage files and the other
lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x80175510` | 2,144 B | creates the full menu viewport and its panes; the wave-21 constructor only reaches it | `80175510_FUN_80175510.c` |
| 2 | `0x80175F70` | 1,184 B | home-button battery/controller pane builder called by `0x80174F30` | `80175f70_FUN_80175f70.c` |
| 3 | `0x801769B0` | 7,420 B | the large dialog layout body | `801769b0_FUN_801769b0.c` |
| 4 | `0x80172BE0` | 256 B | remaining per-player widget helper between the widget table and the entry points | `80172be0_FUN_80172be0.c` |
| 5 | `0x8017DCC0` | 728 B | dialog animation task; consumes the wave-12 evaluator | `8017dcc0_FUN_8017dcc0.c` |
| 6 | `0x8017EA10` | 1,376 B | dialog close/confirm body | `8017ea10_FUN_8017ea10.c` |
| 7 | `0x8016FD30` | 228 B | grid object constructor next to the wave-26 setters | `8016fd30_FUN_8016fd30.c` |
