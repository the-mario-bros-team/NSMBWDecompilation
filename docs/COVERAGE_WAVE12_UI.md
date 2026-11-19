# Coverage - wave 12 HUD/menu helpers (`0x800151F0..0x8017E380`)

## How to read this file

- **Scope** - the fifth UI translation pass. It adds 25 functions to the two
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
  function whose bytes and control flow are exact but whose callee roles,
  string tables or object layout were inferred from the surrounding code.
- Start with the **summary**, then the **function table**. The **cluster
  notes** explain how the new bodies connect to the wave-1..7 HUD and
  home-button-menu code already in the two destination files.

## Summary

| metric | value |
|---|---:|
| functions added to `src/ui/hud.rs` | 15 |
| functions added to `src/ui/menu.rs` | 10 |
| functions added in total | 25 |
| bytes translated | 3,012 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 17 |
| medium confidence | 8 |
| new staging modules | 0 |
| files written | `src/ui/hud.rs`, `src/ui/menu.rs`, this document, `ledger/agents/wave12-ui.csv`, `ledger/functions.csv` |

Address span: `0x800151F0..0x8017E380`. The selected bodies are the remaining
small counter, battery, error-window, nunchuk-warning and dialog/menu helpers;
no large state-machine or jump-table body was guessed.

## Function table

### `ui::hud` - counter, battery, error and nunchuk helpers

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x800151F0` | `fn_800151f0` | destroy the `pointGet` layout at `+4` | translated | high | `800151f0_FUN_800151f0.c` |
| `0x80015200` | `fn_80015200` | refresh one counter digit colour | translated | medium | `80015200_FUN_80015200.c` |
| `0x80015780` | `fn_80015780` | move and flash the coin-pop-up pane | translated | high | `80015780_FUN_80015780.c` |
| `0x80015F20` | `fn_80015f20` | release one battery record | translated | high | `80015f20_FUN_80015f20.c` |
| `0x800162E0` | `fn_800162e0` | destroy the `batteryInfo` layout at `+8` | translated | high | `800162e0_FUN_800162e0.c` |
| `0x80016300` | `fn_80016300` | refresh the four per-player battery slots | translated | medium | `80016300_FUN_80016300.c` |
| `0x80017910` | `fn_80017910` | choose the next error-window animation | translated | high | `80017910_FUN_80017910.c` |
| `0x80017A10` | `fn_80017a10` | start the error button/player animations | translated | high | `80017a10_FUN_80017a10.c` |
| `0x80017A90` | `fn_80017a90` | notify when the button animation ends | translated | high | `80017a90_FUN_80017a90.c` |
| `0x80017B20` | `fn_80017b20` | show the selected answer button | translated | high | `80017b20_FUN_80017b20.c` |
| `0x80017D60` | `fn_80017d60` | start the confirm animation and timer | translated | medium | `80017d60_FUN_80017d60.c` |
| `0x80017DE0` | `fn_80017de0` | advance the error window after animation | translated | high | `80017de0_FUN_80017de0.c` |
| `0x80018060` | `fn_80018060` | close the error window | translated | high | `80018060_FUN_80018060.c` |
| `0x80018CA0` | `fn_80018ca0` | construct the nunchuk warning scene | translated | medium | `80018ca0_FUN_80018ca0.c` |
| `0x80018F20` | `fn_80018f20` | build the nunchuk warning layout | translated | medium | `80018f20_FUN_80018f20.c` |

### `ui::menu` - animation records, pane list and dialog helpers

| address | Rust function | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x801713C0` | `fn_801713c0` | configure one animation record | translated | high | `801713c0_FUN_801713c0.c` |
| `0x80171570` | `fn_80171570` | construct the dialog pane-list object | translated | high | `80171570_FUN_80171570.c` |
| `0x801715D0` | `fn_801715d0` | activate/deactivate every pane in the list | translated | medium | `801715d0_FUN_801715d0.c` |
| `0x80172CE0` | `fn_80172ce0` | read the list-entry flag bit | translated | high | `80172ce0_FUN_80172ce0.c` |
| `0x80172CF0` | `fn_80172cf0` | read the child pointer | translated | high | `80172cf0_FUN_80172cf0.c` |
| `0x80173860` | `fn_80173860` | query one list entry's enabled byte | translated | high | `80173860_FUN_80173860.c` |
| `0x801738F0` | `fn_801738f0` | close one active list entry | translated | high | `801738f0_FUN_801738f0.c` |
| `0x80174330` | `fn_80174330` | convert an enabled byte to `0/1` | translated | high | `80174330_FUN_80174330.c` |
| `0x8017D520` | `fn_8017d520` | evaluate one channel of a cubic key table | translated | medium | `8017d520_FUN_8017d520.c` |
| `0x8017E380` | `fn_8017e380` | construct an alternate dialog list node | translated | medium | `8017e380_FUN_8017e380.c` |

All evidence paths are relative to `work/ghidra/main-gb/export/`.

## Cluster notes

### Counter and battery (`0x800151F0..0x80016300`)

`0x800151F0` and `0x800162E0` are the two eight-byte teardown thunks for the
`pointGet` layout at `+4` and the `batteryInfo` layout at `+8`; both call the
NW4R layout teardown `0x800C9930`. `0x80015200` forwards the animation object at
`+0x1E4` and the selected digit index to `0x800B4730`, completing the colour
path started in wave 7. The second argument is not printed by Ghidra in this
eight-byte body, but the only caller (`0x800154E0`) passes it, so the Rust
signature preserves it.

`0x80015780` is the per-frame position update: it writes
`base + offset` into the selected pane and toggles the direction byte at
`+0x220` every ten ticks. The step is the `f32` word at `r2 - 0x7EC0`
(`0x8042B1C0`), read by address.

`0x80016300` refreshes the four battery slots. It hides each pane at
`+0x1E0 + i*4`, checks controller presence with `0x800B4710`, and copies the
level word at `+0x50` of the SDA controller record into `+0x294 + i*4`. A
negative level aborts before the shared animation handle is shown.

`0x80015F20` is the battery-record release: it destroys the optional
`+0x18` sub-object and frees the record when the flag is positive.

### Error window (`0x80017910..0x80018060`)

These five bodies complete the `errorInfo` state machine around the wave-4
`menu_confirm_pending`/`dialog_begin_dismiss` and wave-7 open/close helpers:

- `0x80017910` waits for the layout animation and selects the member
  notification from the button panes `+0x204`/`+0x208`.
- `0x80017A10` starts the previous/current button animations, or animation `6`
  in the alternate mode (`+0x22F`).
- `0x80017A90` notifies the member when the button animation ends.
- `0x80017B20` shows the selected answer button and its animation.
- `0x80017D60` plays sound `0x79`, starts the confirm animation and arms the
  thirty-tick timer at `+0x21C`.
- `0x80017DE0` advances the window after the animation: timer decrement,
  selected-answer `3` handling, or the generic follow-up.
- `0x80018060` clears the open flag and hides the animation handle.

The member object is the same vtable-bearing sub-object at `+0x1A0` used by
`0x80017340`; virtual slot `+0x18` receives the data objects
`0x80352038/0x803520F8/0x80352138/0x803521B8/0x80352238/0x80352278/0x803522B8`.

### Nunchuk warning (`0x80018CA0..0x80018F20`)

`0x80018CA0` is a third warning-scene constructor shaped like `errorInfo`:
layout at `+4`, member vtables at `+0x19C..+0x1A8`, base sub-object at
`+0x1B4`, build flag at `+0x1E4`. `0x80018F20` builds the layout on first use:
loads `extensionControllerNunchuk_...`, binds the `_09` layout and three
window animations, resolves the animation handle at `+0x1D8` and the
`T_player` pane at `+0x1DC`, binds five `T_nunchukS_00` texts and one nunchuk
pane, then advances and hides the layout. All string/table addresses are used
verbatim; their contents are data and are not interpreted.

### Menu animation/list and dialog (`0x801713C0..0x8017E380`)

`0x801713C0` configures one 0x68-byte animation record. `0x80171570` and
`0x801715D0` are the pane-list constructor and its activation walk; the latter
calls virtual slot `+0x58` on every child and, in mode `1`, virtual slot
`+0x0C` once. The four helpers `0x80172CE0`, `0x80172CF0`, `0x80173860`,
`0x801738F0` and `0x80174330` are the small state/list queries used by the
home-menu state machine.

`0x8017D520` is the dialog keyframe evaluator called by `0x8017DCC0`. It
binary-searches the `(time, value, slope)` triples, snaps to a key inside the
tolerance pair `0x802F5E10/14`, and otherwise evaluates the cubic blend using
the four coefficients at `0x802F5E18/1C/20/24`. `0x8017E380` constructs one
alternate dialog list node and installs vtable `0x8032B0F8`.

## Unknowns and limits

1. **Counter digit argument.** `0x80015200` is eight bytes; Ghidra prints one
   argument for it and one for its `0x800B4730` call. The only caller
   (`0x800154E0`) passes the selected digit index, so both the signature and
   the forwarded second argument are marked `medium`.
2. **Pane-list virtual call.** In `0x801715D0` the virtual call in the mode
   branch has no formal arguments in the export. Passing `this` is inferred
   from the method shape and is the only non-literal operand; the body is
   otherwise exact, so the row is `translated`/`medium`.
3. **Spline coefficients.** `0x8017D520` reads four `f32` coefficients and the
   tolerance pair from fixed addresses; the values are preserved exactly but
   are not interpreted. The export also contains one unreachable-block warning
   after the closing branch.
4. **Battery lock and SDA record.** `0x80016300` calls `0x802DCD70`/
   `0x802DCDBC` (lock/unlock shape) and reads the controller record through
   `r13 - 0x5658`; the names are not recovered.
5. **Nunchuk strings/tables.** `0x80018F20` uses eight string/table addresses
   (`0x80300668..0x80300754`, `0x802EDFF8`, `0x802EE004`, `r13 - 0x7EF8`)
   exactly as printed; the full text of the archive/layout paths is not
   reproduced.
6. **Vtable identities.** The seven nunchuk/menu vtable addresses are stored
   verbatim; their class names are inferred only from the neighbouring
   constructors.
7. **Callee bodies.** `0x800B44D0`, `0x800B4730`, `0x800C8DD0`, `0x800C9930`,
   `0x80107F10`, `0x801083C0`, `0x80174330`, `0x801836B0`, `0x80195380`,
   `0x801E14E0`, `0x802DCD70` and `0x802DCDBC` are called by address; their
   bodies are outside this pass.
8. **No new staging.** Every selected body already had a complete Ghidra
   export; no skeleton or placeholder function was created.

## Verification

```text
cargo check          # exits 0 (warnings only, none from the two destination files)
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Only `src/ui/hud.rs`, `src/ui/menu.rs`, this document,
`ledger/agents/wave12-ui.csv` and the 25 matching rows appended to
`ledger/functions.csv` were written. `src/lib.rs`, every `mod.rs`,
`Cargo.toml`, `src/platform.rs`, the generated coverage files and the other
lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x80017EE0` | 116 B | checks the per-player yes/no result through vtable `+0x20` | `80017ee0_FUN_80017ee0.c` |
| 2 | `0x80017FD0` | 112 B | timer-only error-window state | `80017fd0_FUN_80017fd0.c` |
| 3 | `0x80017D00` | 96 B | start/stop the answer animation | `80017d00_FUN_80017d00.c` |
| 4 | `0x801705E0` | 72 B | grid-entry release duplicate of `0x8016FE40` | `801705e0_FUN_801705e0.c` |
| 5 | `0x8017DCC0` | 728 B | dialog animation-task update; consumes [`fn_8017d520`] | `8017dcc0_FUN_8017dcc0.c` |
