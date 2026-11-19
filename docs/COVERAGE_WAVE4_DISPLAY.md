# Coverage - wave 4 display/pause and home-button dialog flow

## How to read this file

This document covers the fourth translation pass over the 2-D display and
home-button-menu bands of `sys/main.dol`. It follows the semantic modules
created by waves 1-3 and does **not** introduce a new staging range.

Read it in this order:

1. **Scope** - which destination modules received code and how much.
2. **Function table** - one row per translated function with address, Rust
   name, plain-English role, status, confidence and evidence file.
3. **What each module now covers** - the pause animation cycle, the display
   value helpers and the dialog confirm/dismiss/vertex-buffer model.
4. **Unknowns and limits** - what is inferred rather than observed.
5. **Verification** - the two checks that were run.

Status and confidence follow `docs/READING_GUIDE.md`:

| status | meaning |
|---|---|
| `translated` | full control flow, every call accounted for |
| `partial` | flow and major effects present; one call operand is not literal |
| `stub` | address/role only |

All addresses are virtual (image base `0x80004000`), not file offsets.
Evidence paths are relative to the repository root and point at
`work/ghidra/main-gb/export/<addr>_<name>.c`.

**Start here:** [`pausewindow_start_open_animation`](../src/engine/display.rs)
(`0x8015AD80`) -> [`pausewindow_start_button_animation`](../src/engine/display.rs)
(`0x8015AE70`) -> [`pausewindow_update_hint_animation`](../src/engine/display.rs)
(`0x8015A910`). For the menu side start at
[`menu_confirm_pending`](../src/ui/menu.rs) (`0x8017CCA0`) and
[`dialog_begin_dismiss`](../src/ui/menu.rs) (`0x8017D1D0`).

## Scope

| module | file | functions added | bytes | translated | partial |
|---|---|---:|---:|---:|---:|
| `engine::display` | `src/engine/display.rs` | 12 | 2,348 | 12 | 0 |
| `ui::menu` | `src/ui/menu.rs` | 10 | 1,944 | 8 | 2 |
| **total** | | **22** | **4,292** | **20** | **2** |

The pass added 22 functions. Every body keeps its `# Original` block, cites the
Ghidra export, and uses `crate::platform` for target-memory access and indirect
calls. No save-banner body was moved here: the save-banner/boot-scene cluster
(`0x8015BE60`, `0x8015D480`, `0x8015C130` ...) already belongs to
`game::boot_scene` and is outside this lane's two destination files.

## Function table

### `engine::display` - pause animation and 2-D values

| address | Rust name | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8015A910` | `pausewindow_update_hint_animation` | choose the controller hint frame from `DAT_80315B9D` | translated | medium | `8015a910_FUN_8015a910.c` |
| `0x8015AB20` | `pausewindow_update_button_slots` | show one button row and start its selection animation | translated | high | `8015ab20_FUN_8015ab20.c` |
| `0x8015AD80` | `pausewindow_start_open_animation` | mark the layout ready and seek the open animations | translated | high | `8015ad80_FUN_8015ad80.c` |
| `0x8015AE70` | `pausewindow_start_button_animation` | seek the previous/current button animations | translated | high | `8015ae70_FUN_8015ae70.c` |
| `0x8015AC30` | `pausewindow_play_selected_button` | play the animation of the selected button | translated | medium | `8015ac30_FUN_8015ac30.c` |
| `0x8015A470` | `fn_8015a470` | conditional release of a display object | translated | high | `8015a470_FUN_8015a470.c` |
| `0x8015A4B0` | `fn_8015a4b0` | release the embedded sub-object, then the storage | translated | high | `8015a4b0_FUN_8015a4b0.c` |
| `0x8015A510` | `fn_8015a510` | release the embedded sub-object, then the storage | translated | high | `8015a510_FUN_8015a510.c` |
| `0x80159AC0` | `display_set_transform_value` | set one value word and rebuild its transform | translated | medium | `80159ac0_FUN_80159ac0.c` |
| `0x80159CB0` | `display_set_large_value` | debounce and clamp a large display value | translated | medium | `80159cb0_FUN_80159cb0.c` |
| `0x80159F10` | `dgame_display_register_states` | register the four `dGameDisplay_c` state descriptors | translated | high | `80159f10_FUN_80159f10.c` |
| `0x8015B840` | `pausewindow_state_release` | destructor callback registered by the pause descriptors | translated | high | `8015b840_FUN_8015b840.c` |

### `ui::menu` - confirm, dismiss and vertex buffers

| address | Rust name | role | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8017CCA0` | `menu_confirm_pending` | activate the pending menu entries and selection panes | translated | medium | `8017cca0_FUN_8017cca0.c` |
| `0x8017D1D0` | `dialog_begin_dismiss` | dismiss/finish a dialog and choose its timer path | translated | medium | `8017d1d0_FUN_8017d1d0.c` |
| `0x8017D700` | `dialog_panel_construct` | construct a small dialog panel object | translated | medium | `8017d700_FUN_8017d700.c` |
| `0x8017DFE0` | `dialog_find_named_entry` | recursively find a named entry in the parsed document | translated | medium | `8017dfe0_FUN_8017dfe0.c` |
| `0x8017E5F0` | `dialog_vertex_buffer_free` | free the vertex-row buffer | partial | medium | `8017e5f0_FUN_8017e5f0.c` |
| `0x8017E640` | `dialog_vertex_buffer_reserve` | grow the vertex-row buffer to `count` rows | partial | medium | `8017e640_FUN_8017e640.c` |
| `0x8017E870` | `dialog_record_has_value` | test whether a four-word record is unset | translated | high | `8017e870_FUN_8017e870.c` |
| `0x8017F6E0` | `dialog_list_insert` | insert a node after the embedded list head | translated | high | `8017f6e0_FUN_8017f6e0.c` |
| `0x8017F720` | `dialog_find_child_by_name` | find a child node by 16-byte name | translated | high | `8017f720_FUN_8017f720.c` |
| `0x8017F940` | `dialog_group_construct` | construct a dialog group with an embedded list | translated | high | `8017f940_FUN_8017f940.c` |

## What each module now covers

### `engine::display` - pause animation and display values

The wave-4 bodies complete the animation half of `Pausewindow_c`:

* `0x8015AD80` marks `+0x294` ready, advances all bound animations through
  `0x800C9500`, seeks slots `3` and `4`, then starts animation `0`.
* `0x8015AE70` performs the same end-state advance, seeks `+0x288 + 7` when the
  previous index is non-negative, copies `+0x284` into `+0x288`, and starts
  `+0x284 + 3`.
* `0x8015AC30` resolves `TABLE[+0x284]` from `0x8042DBD8` and calls
  `0x800B44D0(pane, 0, 0)` on `+0x244 + index*4`.
* `0x8015A910` is the hint switch. It clamps `DAT_80315B9C` to `9`, writes
  `display + 0x3B4 = value + 1`, starts frame `0x51` on pane `+0x268`, and maps
  `DAT_80315B9D` cases `0x14..0x26`, `0x28` and `default` to hint frames.
  The default arm starts the left pane `+0x26C` with frame `0x52` and advances
  `display + 0x3B8`; every other arm starts the right pane `+0x270`.
* `0x8015AB20` enables the three button panes `+0x278`/`+0x27C`/`+0x280`, then
  either selects the row (`+0x26C`/`+0x250`, frame `0x52`) or delegates to
  `0x8015A910`.
* `0x80159AC0`/`0x80159CB0` are the two value setters: the first is a direct
  compare/store/call, the second adds the one-shot `+0x434` debounce and the
  `0x5F5E0CD`/`0x5F5E0CE` clamp.
* `0x80159F10` registers the four `dGameDisplay_c` descriptors at
  `0x80377078`, `0x803770B8`, `0x803770F8` and `0x80377138`. Each block reads
  nine ROM words, calls the descriptor constructor `0x8015F7C0` with its
  `StateID_ProcMain`/`StateID_ProcGoal` name, writes the common vtable
  `0x80327980` and the nine words at `+0xC..+0x2C`, then registers the
  descriptor with destructor `0x8015A250`.
* `0x8015A470`, `0x8015A4B0`, `0x8015A510` and `0x8015B840` are the
  display/state release helpers. The two middle bodies also destroy the
  embedded sub-object at `+0x18` through `0x8015F9F0`.

### `ui::menu` - confirm, dismiss and vertex buffers

* `0x8017CCA0` walks the pending-entry flags at `+0x20 + i*4`, resolves each
  entry's pane through the `0x802F5AB8` `(entry, 2)` table, and then handles the
  two global pending slots at `+0x20 + count*4` / `+0x24 + count*4`. The second
  slot selects the `0x802F5B18` group mode `0x14` or `3` before starting the
  pane at `+0x2F0 + index*4`.
* `0x8017D1D0` is the dismiss path: it resets the four widgets at
  `+0x2AC + i*4`, clears the four state bytes `+0x95/+0x96/+0x98/+0x99`, then
  routes on `+0x14` into the immediate release, the timer arm, or the deferred
  release. The timer is `+0x5E8` and is armed with
  `(_DAT_800000F8 / 4000) * 100` ticks and callback `0x80178B40`. The tail
  writes `+0x14 = 0x13`, `+0x45C = 1`, `+0xB8 = 2`, converts `+0x458` into the
  `f32` at `+0x79C`, and invokes the interface callback `(3, value)`.
* `0x8017D700`/`0x8017F940` are the two small constructors: a panel with
  vtable `0x8032AFF8` and a group with vtable `0x8032B238` plus a self-
  referential list head at `+8`.
* `0x8017DFE0` recursively walks the parsed document. `0x8019FF00` fills a
  four-word iterator, `0x8019FF80` fills a four-word node record, and the `+8`
  flag selects a nested descent; otherwise `strcmp` compares the node name.
* `0x8017E5F0`/`0x8017E640` own the vertex-row buffer (`+0` capacity, `+1`
  used count, `+4` pointer). The reserve path allocates
  `(count & 0xFF) * 32` bytes and stores the truncated capacity.
* `0x8017E870`, `0x8017F6E0` and `0x8017F720` are the record test, the list
  insert wrapper and the child-by-name search.

## Unknowns and limits

1. **`0x8015A910` / `0x8017CCA0` prologues.** Ghidra prints no parameter list
   because the bodies start with `__savegprlr_*` helpers (`0x802DCD74` and
   `0x802DCD6C`). The `this` argument is reconstructed from the callers
   `0x8015AB20` and `0x801769B0`, which pass their own object pointer, and from
   the offsets used by the body.
2. **Display state words.** `0x80429F7C` is read as an opaque pointer. Its
   `+0x3B4`, `+0x3B8`, `+0x3E4` and `+0x3E8` words are named only by their
   role in these two update functions; the export does not recover a class
   name.
3. **SDA2 tables and constants.** `0x8042DBD8` is indexed as a table of button
   slots; `0x8042DBB8` and `0x8042DBBC` are passed as source-word addresses to
   `0x800B3B60`. Their contents are data and are not interpreted.
4. **`0x8017CCA0` tables.** The two tables at `0x802F5AB8` and `0x802F5B18`
   are scanned exactly, but their entry/pane semantics are inferred from the
   surrounding menu state.
5. **`0x8017D1D0` conversion.** The original uses the
   `CONCAT44(0x43300000, value ^ 0x80000000)` unsigned-to-double idiom with the
   bias at `0x802F5DC0`. The Rust code uses `value as f64`, which is the same
   numeric conversion; the comment records the original idiom.
6. **`0x8017DFE0` iterator layout.** The 16-byte iterator and four-word node
   record are reconstructed from the access offsets in `0x8019FF00`/
   `0x8019FF80`. `0x8019FEA0`'s definition prints one parameter, but both call
   sites pass `(document, name)`; the second operand is preserved.
7. **`0x801D5550` free operand.** Ghidra prints only `DAT_80378008` in the two
   buffer helpers. The pointer operand is reconstructed from the value read at
   `this + 4` immediately before the call; both rows are marked `partial`.

## Verification

```text
cargo check          # exits 0
node tools/doc-lint.mjs src
# {"ok": true, "files": 68, "violations": 0, "byRule": {}}
```

Only `src/engine/display.rs`, `src/ui/menu.rs`,
`docs/COVERAGE_WAVE4_DISPLAY.md` and `ledger/agents/wave4-display.csv` were
written. `src/lib.rs`, every `mod.rs`, `Cargo.toml`, `src/platform.rs` and the
other lanes were left untouched.

## Next targets

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x8015B320` | 1,276 B | `Pausewindow_c` six-state descriptor registration; pairs with the wave-4 `dGameDisplay_c` initialiser | `8015b320_FUN_8015b320.c` |
| 2 | `0x80159AF0` | 444 B | three-player indicator update loop that uses the display object | `80159af0_FUN_80159af0.c` |
| 3 | `0x8017E5D0` | 20 B | clear the vertex-row counters; pairs with the wave-4 buffer helpers | `8017e5d0_FUN_8017e5d0.c` |
| 4 | `0x8017D520` | 452 B | spline/keyframe interpolation used by the dialog animation | `8017d520_FUN_8017d520.c` |
| 5 | `0x8017DCC0` | 728 B | animation-task update that consumes the dialog key tables | `8017dcc0_FUN_8017dcc0.c` |
