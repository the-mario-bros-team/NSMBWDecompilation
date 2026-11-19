# Coverage - wave 19 HUD/menu helpers (`0x80016430..0x80175500`)

## How to read this file

- **Scope** - the sixth UI translation pass. It adds 25 functions to the two
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
  function whose guards, stores and calls are exact but whose dropped call
  operands were reconstructed from the surrounding code.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the new bodies connect to the earlier HUD and
  home-button-menu code already in the two destination files.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/hud.rs` | 9 |
| functions added to `src/ui/menu.rs` | 16 |
| functions added in total | 25 |
| bytes translated | 3,676 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 20 |
| medium confidence | 5 |
| new staging modules | 0 |
| files written | `src/ui/hud.rs`, `src/ui/menu.rs`, this document, `ledger/agents/wave19-ui.csv`, `ledger/functions.csv` |

Address span: `0x80016430..0x80175500`. The selected bodies are a further set
of untranslated HUD/menu helpers in the two bands: the battery per-frame update
and state tables, the error-window pane binding, several record destructors and
teardown thunks, the home-menu record helpers, and the small dialog/menu entry
points. Large state machines such as `0x80018330` remain listed as next
targets.

## Function table

### `ui::hud` - battery frame, state tables, error binding and teardown

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80016430` | `fn_80016430` | per-frame battery pane update | translated | high | `80016430_FUN_80016430.c` |
| `0x80016A20` | `fn_80016a20` | unresolved indirect-jump thunk | translated | medium | `80016a20_FUN_80016a20.c` |
| `0x80016A50` | `fn_80016a50` | unresolved indirect-jump thunk | translated | medium | `80016a50_FUN_80016a50.c` |
| `0x80016A80` | `fn_80016a80` | build the five battery state descriptors | translated | high | `80016a80_FUN_80016a80.c` |
| `0x80017100` | `fn_80017100` | battery record destructor | translated | high | `80017100_FUN_80017100.c` |
| `0x80017630` | `fn_80017630` | bind the error answer panes and animations | translated | high | `80017630_FUN_80017630.c` |
| `0x80018D90` | `fn_80018d90` | nunchuk record destructor | translated | medium | `80018d90_FUN_80018d90.c` |
| `0x80018E30` | `fn_80018e30` | nunchuk record destructor with `+0x18` sub-object | translated | high | `80018e30_FUN_80018e30.c` |
| `0x80019110` | `fn_80019110` | tear down the nunchuk layout | translated | high | `80019110_FUN_80019110.c` |

### `ui::menu` - home-menu records and entry points

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x80171400` | `menu_item_recompute_current` | copy the mode-selected item value into the live slot | translated | high | `80171400_FUN_80171400.c` |
| `0x80173880` | `fn_80173880` | start one entry action | translated | medium | `80173880_FUN_80173880.c` |
| `0x80173920` | `fn_80173920` | submit one entry action | translated | high | `80173920_FUN_80173920.c` |
| `0x801739E0` | `fn_801739e0` | read a per-player menu byte | translated | high | `801739e0_FUN_801739e0.c` |
| `0x80173A10` | `fn_80173a10` | clear a per-player menu byte | translated | high | `80173a10_FUN_80173a10.c` |
| `0x80173CA0` | `fn_80173ca0` | reset the four entry records | translated | high | `80173ca0_FUN_80173ca0.c` |
| `0x80173F20` | `fn_80173f20` | clear the block timer and four record timers | translated | high | `80173f20_FUN_80173f20.c` |
| `0x80173F80` | `fn_80173f80` | install the one-argument callback | translated | high | `80173f80_FUN_80173f80.c` |
| `0x80174090` | `fn_80174090` | install the four-argument callback | translated | high | `80174090_FUN_80174090.c` |
| `0x801742E0` | `fn_801742e0` | test an entry's enabled word | translated | high | `801742e0_FUN_801742e0.c` |
| `0x80174300` | `fn_80174300` | test enabled word plus selection id | translated | high | `80174300_FUN_80174300.c` |
| `0x801744F0` | `fn_801744f0` | notify/reset the active menu manager | translated | high | `801744f0_FUN_801744f0.c` |
| `0x80174F60` | `fn_80174f60` | open a dialog through the live menu | translated | medium | `80174f60_FUN_80174f60.c` |
| `0x80175040` | `fn_80175040` | apply the three dialog-mode bytes | translated | high | `80175040_FUN_80175040.c` |
| `0x801754B0` | `fn_801754b0` | allocate and construct the menu object | translated | high | `801754b0_FUN_801754b0.c` |
| `0x80175500` | `fn_80175500` | return the live menu object | translated | high | `80175500_FUN_80175500.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Battery frame and state tables (`0x80016430..0x80017100`)

`0x80016430` is the `batteryInfo` per-frame body that earlier passes listed as
still staged. It locks the controller-info record with `0x802DCD70`/`0x802DCDBC`,
hides two blocks of ten bar panes at `+0x204..+0x250`, and then walks the four
controller slots (`0x800B4710`). For each connected player it shows the
`N_batt` pane, copies the position words from `+0x254/+0x258/+0x25C` (advancing
by `0x0C` per present player), starts animation `player + 1`, and shows the bar
panes selected by the level word at `+0x294`. The two tables are read by
address: `0x802EDE90` chooses the shared pane by the number of present players,
and `0x802EDEA8` maps each player's level to the five bar-pane indices.

`0x80016A80` builds the five `dWarningBattery_c` state descriptors at
`0x80351EF8`, `0x80351F38`, `0x80351F78`, `0x80351FB8` and `0x80351FF8`.
Every block calls `0x8015F7C0` with its state name, installs the shared vtable
`0x802FFD84`, copies nine words from the adjacent `.data` table into
`+0x0C..+0x2C`, and registers with `0x802DC780` and the release callback
`0x80016E80`. The copy order is preserved; the source words are read from
fixed addresses, not transcribed.

`0x80017100` is the matching record destructor: it releases the sub-object at
`+0x18` through `0x8015F9F0` and optionally frees the record with
`0x802B9280`.

### Error-window binding (`0x80017630`)

`0x80017630` completes the answer-pane path used by the wave-12 bodies
`0x80017B20`/`0x80017D60`/`0x80017DE0`. For the selected answer (`+0x220`) it
copies five visibility bytes from `0x802EDF80` into the panes at
`+0x204..+0x214`, then reads the 12-byte animation record at
`0x802EDFA4 + answer*12`:

- word `+0` starts panes `+0x1EC/+0x1F0` when non-zero;
- word `+4` starts panes `+0x1F4/+0x1F8`;
- word `+8` either clears the alternate flag `+0x22F` or starts panes
  `+0x1FC/+0x200` and sets it.

When the first visibility byte is non-zero, the function instead starts the
`+0x1E8` animation with frame limit `0xC9` and argument `+0x218 + 1`. Finally
it starts animation `0x10`, or `3`, or the pair `1/2`, according to the
visibility of the two player-button panes `+0x204/+0x208`.

### Nunchuk destructors (`0x80018D90..0x80019110`)

`0x80018D90` is the plain `operator delete` guard; `0x80018E30` also releases
the `+0x18` sub-object; `0x80019110` tears down the embedded layout at `+4`
through `0x800C9930`. These are the teardown twins of the constructors added in
wave 12 (`0x80018CA0`/`0x80018F20`).

### Home-menu records (`0x80171400..0x80174300`)

`menu_item_recompute_current` is the 32-byte body that copies `+4` (mode `1`)
or `+8` (every other mode) into the live `+0x0C` slot; it complements the
wave-12 animation-record setter `0x801713C0`.

The four 0x68-byte records are shared with `fn_801738f0` and `0x80174330`.
`0x801742E0` is the strict enabled-word test (`+0x50 != 0`), while `0x80174300`
adds the selection-id compare at `+0x58`. `0x80173880`/`0x80173920` start and
submit one entry; both bound the index to `0..3` and use the `0x801742E0`
predicate. `0x80173CA0` resets the enabled/id pairs of the four records, and
`0x80173F20` clears the block timer at `+0x1A0` plus the four per-record
timers.

`0x80173F80`/`0x80174090` fetch the active manager with `0x801ABA80` and install
the callbacks `0x80173FB0`/`0x801740C0` in slots `1` and `4`.

### Menu object and dialog entry points (`0x801744F0..0x80175500`)

`0x801754B0` allocates `0x7A0` bytes with `0x80174DF0`, constructs the menu
with `0x80175330`, and stores the result in `0x80377FB8`; `0x80175500` is the
getter for that global. `0x80175040` applies the three dialog-mode bytes to the
live menu through the already-translated `0x8017D3C0`.

`0x801744F0` resets the active manager widget at `+0x14680` and calls the
optional callback at `+0x14348`. `0x80174F60` opens the large dialog body
`0x801769B0` with `(menu, argument)` and then queries the dialog result. The
result query has no printed argument in the export; the Rust body feeds the
second getter result, matching the `r3` retention at the call site.

## Unknowns and limits

1. **Indirect jump stubs.** `0x802DCBFC`, used by `0x80016A20`/`0x80016A50`,
   still has no recovered jump table. The three operands are preserved exactly
   and both rows are `medium`.
2. **Dropped operands.** `0x80018D90` prints `0x802B9280()` without operands;
   the Rust body passes `this`, following the twin destructors that do print
   it. `0x80174F60` prints the final result query without operands; the second
   menu-getter result is passed by inference. `0x80173880` prints only one
   operand on the `0x801742E0` call; the index is supplied from `*this`.
3. **State names.** The five battery state names are string addresses in the
   image (`0x802FFCAC`, `0x802FFCD0`, `0x802FFCFC`, `0x802FFD30`,
   `0x802FFD58`). They are used verbatim; their full text is not reproduced.
4. **Table contents.** The battery pane maps (`0x802EDE90`, `0x802EDEA8`) and
   the error answer tables (`0x802EDF80`, `0x802EDFA4`) are observed bytes.
   Their contents are documented in the Rust comments but not interpreted
   beyond the indexing used by the two bodies.
5. **`0x801739B0`.** The callback passed to `0x801E12C0` is a code block that
   is not a separate `functions.tsv` entry; it is used by its raw address.
6. **Callee bodies.** `0x8015F7C0`, `0x802DC780`, `0x801E12C0`, `0x801E14E0`,
   `0x801E1E20`, `0x801ABA80`, `0x801B23F0`, `0x801769B0`, `0x80175330` and
   the two `0x802DCD70`/`0x802DCDBC` helpers are called by address; their
   bodies are outside this pass.

## Verification

```text
cargo check          # exits 0 (warnings only, none from the two destination files)
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Only `src/ui/hud.rs`, `src/ui/menu.rs`, this document,
`ledger/agents/wave19-ui.csv` and the 25 matching rows appended to
`ledger/functions.csv` were written. `src/lib.rs`, every `mod.rs`,
`Cargo.toml`, `src/platform.rs`, the generated coverage files and the other
lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x80018330` | 2,168 B | `dWarningErrorInfo_c` state-descriptor builder; completes the error scene next to `0x80017630` | `80018330_FUN_80018330.c` |
| 2 | `0x80016A80` neighbour `0x80016A20`/`0x80016A50` remain `medium` until `0x802DCBFC` is decoded | 48 B each | unresolved indirect jump | `80016a20_FUN_80016a20.c` |
| 3 | `0x8017DCC0` | 728 B | dialog animation-task update; consumes the wave-12 `0x8017D520` evaluator | `8017dcc0_FUN_8017dcc0.c` |
| 4 | `0x80171680` | 452 B | home-menu pane/animation constructor next to the wave-12 list helpers | `80171680_FUN_80171680.c` |
| 5 | `0x80174AF0` | 452 B | home-menu initialisation body next to the new `0x801754B0` constructor | `80174af0_FUN_80174af0.c` |
