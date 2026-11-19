# Coverage - display, boot scene and home-button menu

## How to read this file

This document covers the semantic modules that were promoted from the
display/boot/menu clusters of the `main.dol` address lane
`0x80140000..0x80180000`:

| module | what it owns | evidence band |
|---|---|---|
| `src/engine/display.rs` (`engine::display`) | `Pausewindow_c` state machine and the `Wm_2d_*` effects | G, `0x80154E50..0x8015B320` |
| `src/game/boot_scene.rs` (`game::boot_scene`) | save banner/icon and the first `dScBoot_c` handlers | H, `0x8015B840..0x8015DDD0` |
| `src/ui/menu.rs` (`ui::menu`) | home-button button-name lookup, dialog data model, event/colour helpers | N/O/P, `0x8016FD30..0x8017F980` |

Read it in this order:

1. **Scope and counts** - how many entries the three bands contain and how
   many are translated.
2. **Promotion summary** - the 25 bodies that moved, grouped by destination.
3. **Translated functions** - one row per function with address, role, status,
   confidence and the exact Ghidra evidence file.
4. **What each module now covers** - the state-machine or data-model shape the
   Rust implements.
5. **Evidence quality notes** and **Uncertainties** - what is observed, what is
   inferred, and what remains unresolved.
6. **Next targets** - the next bodies to translate from the same bands.

Status and confidence follow `docs/READING_GUIDE.md`:

| status | meaning |
|---|---|
| `translated` | full control flow, every call accounted for |
| `partial` | flow and major effects present; one call's operands are unrecovered |
| `stub` | address/role only |

All addresses are virtual (image base `0x80004000`), not file offsets. Evidence
paths are relative to the repository root.

**Start here:** `pausewindow_construct` (`0x8015A370`) ->
`pausewindow_begin_open` (`0x8015AC70`) -> `pausewindow_wait_open_animation`
(`0x8015ADF0`) -> `pausewindow_wait_button_animation` (`0x8015AEE0`) ->
`pausewindow_update_display` (`0x8015AF60`) in
[`src/engine/display.rs`](../src/engine/display.rs). That is the complete
pause-window cycle. For the boot/save path start at `dsc_boot_construct`
(`0x8015C130`) in [`src/game/boot_scene.rs`](../src/game/boot_scene.rs); for the
menu data model start at `menu_button_index` (`0x8017CF10`) in
[`src/ui/menu.rs`](../src/ui/menu.rs).

## Scope and counts

| band | addresses | all entries | eligible (>=16 B) | bytes (eligible) | translated | translated bytes |
|---|---|---:|---:|---:|---:|---:|
| G - 2-D display/pause | `0x80154E50..0x8015B320` | 62 | 62 | 14,004 | 10 | 1,772 |
| H - save banner/boot scene | `0x8015B840..0x8015DDD0` | 59 | 59 | 10,956 | 7 | 1,360 |
| N - `hmMenu` | `0x8016FD30..0x80175510` | 100 | 87 | - | 0 | 0 |
| O - home-button dialogs/battery | `0x80175D70..0x8017CF0F` | 19 | 19 | - | 0 | 0 |
| P - dialog/menu input helpers | `0x8017CF10..0x8017F980` | 44 | 42 | - | 8 | 644 |
| **total** | | **284** | **269** | | **25** | **3,776** |

The dash means the lane coverage doc does not report a per-band byte total for
N/O/P; only the entry counts are recorded there. This pass translated 25
functions from bands G, H and P. Bands N and O remain untranslated; their
strings (`hmMenu_strt`, `P1_Def_brlyt`, `btryPwr_00_0`, `T_Dialog`,
`T_msg_00/01`) are the anchor for the next pass.

## Promotion summary

| module | functions | bytes | status |
|---|---:|---:|---|
| `engine::display` | 10 | 1,772 | translated |
| `game::boot_scene` | 7 | 1,360 | translated |
| `ui::menu` | 8 | 644 | translated |

No function was left in `src/subsystems/r80140_80180.rs` for these bodies: each
definition and its `# Original` block was written directly into the semantic
module. The staging file is owned by another lane and was not edited.

## Translated functions

All evidence paths are under `work/ghidra/main-gb/export/`.

### `engine::display` - 2-D display and pause

| address | Rust name | role (plain English) | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8015A370` | `pausewindow_construct` | build the pause window, layout and state manager | translated | high | `8015a370_FUN_8015a370.c` |
| `0x8015A8D0` | `pausewindow_close` | close the layout and clear the live instance | translated | high | `8015a8d0_FUN_8015a8d0.c` |
| `0x8015AC70` | `pausewindow_begin_open` | start the open animation and enter `OpenAnime` | translated | high | `8015ac70_FUN_8015ac70.c` |
| `0x8015ADF0` | `pausewindow_wait_open_animation` | wait, then enter `ButtonChange` | translated | high | `8015adf0_FUN_8015adf0.c` |
| `0x8015AEE0` | `pausewindow_wait_button_animation` | wait, then enter `PauseDisp` | translated | high | `8015aee0_FUN_8015aee0.c` |
| `0x8015AF60` | `pausewindow_update_display` | choose button/hit/close transition | translated | high | `8015af60_FUN_8015af60.c` |
| `0x8015B050` | `pausewindow_wait_hit_animation` | wait, then enter `CloseAni` | translated | high | `8015b050_FUN_8015b050.c` |
| `0x8015B0E0` | `pausewindow_wait_close_animation` | wait, then return to `InitWait` | translated | high | `8015b0e0_FUN_8015b0e0.c` |
| `0x801590B0` | `wm_spawn_1up_effects` | spawn `Wm_2d_1up01/02` for one player | translated | medium | `801590b0_FUN_801590b0.c` |
| `0x80159960` | `wm_set_coin100_display` | update the `Wm_2d_coin100` effect | translated | medium | `80159960_FUN_80159960.c` |

### `game::boot_scene` - save banner and boot scene

| address | Rust name | role (plain English) | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8015BE60` | `load_save_banner` | mount the US save-banner resource | translated | high | `8015be60_FUN_8015be60.c` |
| `0x8015D480` | `load_save_icon` | load `save_icon.bti` or enter the error fade | translated | medium | `8015d480_FUN_8015d480.c` |
| `0x8015C130` | `dsc_boot_construct` | construct the boot scene and two state managers | translated | high | `8015c130_FUN_8015c130.c` |
| `0x8015C5A0` | `dsc_boot_update` | run the task list and tick both interfaces | translated | high | `8015c5a0_FUN_8015c5a0.c` |
| `0x8015C710` | `dsc_boot_begin_reset` | enter the reset/wait phase | translated | high | `8015c710_FUN_8015c710.c` |
| `0x8015C770` | `dsc_boot_update_reset` | wait for controller, then fade or finish | translated | high | `8015c770_FUN_8015c770.c` |
| `0x8015C8D0` | `dsc_boot_update_controller_info` | route the controller-information states | translated | medium | `8015c8d0_FUN_8015c8d0.c` |

### `ui::menu` - home-button menu and dialogs

| address | Rust name | role (plain English) | status | conf. | evidence |
|---|---|---|---|---|---|
| `0x8017CF10` | `menu_button_index` | resolve a pane name to a `B_btnL_00`/`B_btn_00` index | translated | medium | `8017cf10_FUN_8017cf10.c` |
| `0x8017CFC0` | `menu_dialog_result` | return the result word for dialog type `0x12` | translated | high | `8017cfc0_FUN_8017cfc0.c` |
| `0x8017DFA0` | `menu_find_list_node` | find a list node by id | translated | high | `8017dfa0_FUN_8017dfa0.c` |
| `0x8017E270` | `menu_bind_message` | bind a message object and copy its text | translated | high | `8017e270_FUN_8017e270.c` |
| `0x8017E310` | `menu_find_named_value` | find a named node and return its value | translated | high | `8017e310_FUN_8017e310.c` |
| `0x8017E560` | `menu_event_is_button` | test the fixed `0xFEFF/8` button event | translated | high | `8017e560_FUN_8017e560.c` |
| `0x8017E590` | `menu_event_matches` | test an event id against the fixed button event | translated | high | `8017e590_FUN_8017e590.c` |
| `0x8017E8E0` | `menu_scale_color_alpha` | copy a colour and scale its alpha byte | translated | high | `8017e8e0_FUN_8017e8e0.c` |

## What each module now covers

### Pause window (`engine::display`)

The six `Pausewindow_c` descriptors are registered by `0x8015B320` at
`0x80377180` (`InitWait`), `0x803771C0` (`OpenAnime`), `0x80377200`
(`ButtonChange`), `0x80377240` (`PauseDisp`), `0x80377280` (`HitAnimeEnd`) and
`0x803772C0` (`CloseAni`). The interface object at `Pausewindow_c + 0x208`
changes state through vtable slot `+0x18`, passing the descriptor address.

The implemented cycle is:

```text
construct -> begin_open -> OpenAnime
OpenAnime  --wait_open_animation--> ButtonChange
ButtonChange --wait_button_animation--> PauseDisp
PauseDisp  --update_display--> ButtonChange | HitAnimeEnd | CloseAni
HitAnimeEnd --wait_hit_animation--> CloseAni
CloseAni   --wait_close_animation--> InitWait
```

`0x8015AC70` also starts four animation panes and applies the colour animation
when `+0x295` is set. `0x8015AF60` plays sound `0x78` when the selected button
index changed (`+0x288 != +0x284`).

The two `Wm_2d_*` functions build the same 15-word transform from a source
matrix at `+0x84..+0xB0`, call `0x800B37E0` to rescale the first two `f32`
components, and spawn the named effect through `0x8016C890`. `0x801590B0`
spawns `Wm_2d_1up01` and `Wm_2d_1up02`; `0x80159960` spawns `Wm_2d_coin100`.

### Boot scene (`game::boot_scene`)

`dScBoot_c` has two state managers:

* `+0x8C`, constructed with interfaces `+0x78`/`+0x7C` and the `FadeOutWait`
  descriptor `0x803774C8`;
* `+0xC8`, constructed with interfaces `+0xB4`/`+0xB8` and the `ResetWait`
  descriptor `0x80377408`.

The translated handlers cover the save resources (`US/save_banner`,
`save_banner`, `save_icon.bti`), the per-frame task-list tick, the reset start,
the controller-ready wait and the two controller-information event queries.
The unused transitions are listed in **Next targets**.

### Home-button menu (`ui::menu`)

The menu module now has the data-model half of the overlay:

* `menu_button_index` walks `B_btnL_00` (count from `table + 0xC`) and then the
  ten `B_btn_00` entries of `0x8032ACA0`;
* `menu_dialog_result` exposes the result word for dialog type `0x12`;
* `menu_find_list_node` / `menu_find_named_value` walk the two intrusive lists
  (`+4` by id, `+0x28` by name);
* `menu_bind_message` stores the message object, copies up to `0x7F` bytes of
  text and clears the `+0xAF` flag;
* the two event predicates test the fixed `0xFEFF/8` event;
* `menu_scale_color_alpha` performs the exact signed `alpha * scale / 0xFF`
  arithmetic.

The layout/animation construction (`0x80175510`, `0x80175F70`, `0x801769B0`)
and the battery-pane visibility helpers (`0x80176740`, `0x801786B0`,
`0x80178C50`, `0x80179E60`) remain untranslated.

## Evidence quality notes

* Every one of the 25 functions has a matching `.c` file in
  `work/ghidra/main-gb/export`; none carries a `Could not recover jumptable`,
  `ldexpf`/GQR pattern, or a hard decode failure. The only warnings in the
  translated set are the compiler prologue helpers described below.
* The `0x802DCDxx` calls in `0x8015C130` and `0x8017CF10` are CodeWarrior
  `__savegprlr_N` stubs, not game logic. Ghidra prints them as calls and, in
  `0x8017CF10`, as a 64-bit "return" that is really the preserved `r3`/`r4`
  argument pair. The Rust signatures make those arguments explicit and document
  the inference.
* `r13` is the SDA base `0x8042F680`, so every `unaff_r13 - k` in these bodies
  is a known absolute address and is written as such. `r2` is the SDA2 base
  `0x80433080`; the only unresolved reads are the two float/word constants at
  `r2 - 0x54C0` and `r2 - 0x54CC`.
* The state descriptors are data records, not code. The Strings in
  `0x8015B320` and `0x8015DDD0` provide the `StateID_*` names; the Rust code
  keeps the descriptor addresses and marks the truncated names with `...`.

## Uncertainties

1. **SDA2 transform constant.** `0x8042DBC0` (`r2 - 0x54C0`) is the third word
   of both effect transforms. Its value is not recovered; the Rust code reads
   the absolute address instead of guessing `0.0`, `1.0` or a scale.
2. **`0x800B3B60` source operand.** `wm_set_coin100_display` passes
   `r2 - 0x54CC` as the second argument. The call only needs the address, so the
   word at `0x8042DBB4` is not interpreted.
3. **`menu_button_index` arguments.** Ghidra prints `void FUN_8017cf10(void)`.
   The two arguments are reconstructed from the prologue: `r3` (the high word
   of Ghidra's `u64`) is the button-table header whose `+0xC` is the
   left-button count, and `r4` (the low word) is the menu object whose name
   buffer starts at `+0xB4`. The loops and the index arithmetic are observed.
   This register order is confirmed by `0x8000B370`, where the same prologue
   pattern yields the first argument as a pointer and the second as a small
   enum.
4. **`0x8017CF10` return value.** The C initialises the result to `-1` and
   overwrites it with the first matching left index; the second loop can
   overwrite it with `j + left_count`. The Rust keeps that exact order,
   including the case where the first loop matched and the second does not.
5. **`dsc_boot_update_controller_info`.** The two event queries pass through
   virtual interfaces and their return words are only tested for zero. The
   event descriptors (`0x80377548`, `0x80377688`) and the target descriptors
   (`0x80377608`, `0x803776C8`) are exact; the opaque state object's type is not.
6. **`load_save_icon` archive argument.** `0x800DF180` receives
   `archive_manager + 4`; the role of the `+4` handle is inferred from the stage
   loader's use of the same global, not from a named structure.
7. **`0x8015CB20`.** `dsc_boot_update_reset` calls this boot-state helper, but
   its body is not translated in this pass. It is called by address with an
   `// unknown:` comment; no behaviour is invented.
8. **Pause flag names.** The bytes `+0x290..+0x295` are named by the transitions
   they cause (open/close/hit/ready) because the export does not recover their
   field names. The stores and tests themselves are exact.
9. **Object extents.** The constructor requires `0x290` writable bytes for
   `Pausewindow_c` and `0x110` for `dScBoot_c`; those are lower bounds derived
   from the highest offset written in the translated bodies, not from a
   recovered class size.

## Next targets

Ordered by value per effort, with the evidence file to read first.

| priority | target | size | why | first evidence |
|---|---|---:|---|---|
| 1 | `0x8015A910` | 524 B | pause-display switch that chooses the controller hint animation (cases `0x14..0x28`) | `8015a910_FUN_8015a910.c` |
| 2 | `0x8015AB20` | 268 B | pause button/slot update that calls `0x8015A910` | `8015ab20_FUN_8015ab20.c` |
| 3 | `0x8015AD80`, `0x8015AE70` | 104 B, 108 B | remaining open/button-change animation setup | `8015ad80_FUN_8015ad80.c`, `8015ae70_FUN_8015ae70.c` |
| 4 | `0x80159F10` | 824 B | `dGameDisplay_c` 4-state descriptor initialiser; completes the display class | `80159f10_FUN_80159f10.c` |
| 5 | `0x8015B320` | 1,276 B | `Pausewindow_c` descriptor registration; the six names are already mapped here | `8015b320_FUN_8015b320.c` |
| 6 | `0x8015C4C0` | 148 B | boot task-list gate that returns `1` only after both controller checks | `8015c4c0_FUN_8015c4c0.c` |
| 7 | `0x8015C680`, `0x8015CAB0` | 100 B, 104 B | reset-fade-out and reset-wait transitions | `8015c680_FUN_8015c680.c`, `8015cab0_FUN_8015cab0.c` |
| 8 | `0x8015CD40` | 80 B | fade timer constants `0x4B0`/`0x3C` | `8015cd40_FUN_8015cd40.c` |
| 9 | `0x8015DDD0` | 4,716 B | the full 24-state `dScBoot_c` descriptor list; mechanical once one descriptor is decoded | `8015ddd0_FUN_8015ddd0.c` |
| 10 | `0x80179E60` | 700 B | button visibility loop over the `B_btn_00` table | `80179e60_FUN_80179e60.c` |
| 11 | `0x8017CFE0`, `0x8017D1D0` | 292 B, 488 B | dialog button state set / full dismiss path | `8017cfe0_FUN_8017cfe0.c`, `8017d1d0_FUN_8017d1d0.c` |
| 12 | `0x80175510` | 2,144 B | `hmMenu` layout/animation setup; string-rich but large | `80175510_FUN_80175510.c` |
| 13 | `0x80175F70` | 1,184 B | `T_Dialog` layout setup | `80175f70_FUN_80175f70.c` |
| 14 | `0x801769B0` | 7,420 B | largest menu/dialog initialiser; read the string references before attempting | `801769b0_FUN_801769b0.c` |

Do not start with `0x80178C50` (2,388 B) or `0x8017A120`/`0x8017AF70`/
`0x8017BA40` (3,660/2,764/4,692 B): they are the battery/menu update bodies and
need the pane-field model from `0x80179E60` and the dialog state model first.
