# Coverage — wave 16 engine display/frame

## How to read this file

This is the completion record for the wave-16 "engine display/frame" lane. It
answers, for a reader with no context:

1. **What was added?** Two tables list every function body written in this
   pass: the pause-menu controller and the `gameScene` world-map HUD.
2. **Where is it?** Every new body is in `src/engine/display.rs`
   (`engine::display`). `src/engine/frame.rs` (`engine::frame`) was checked and
   left unchanged; the reason is in **Selection** below.
3. **How sure is it?** `confidence` measures the evidence, not the code style:
   `high` = every branch, store and call argument is explicit in the Ghidra C;
   `medium` = the flow is complete but a callee operand/return or a data-table
   meaning had to be recovered from the call site.
4. **What is still unknown?** See **Uncertainties** and **Next targets**.

Addresses are **plain lowercase hex** virtual image addresses (image base
`0x80004000`), not file offsets. Evidence is the per-function Ghidra C file
under `work/ghidra/main-gb/export/`. Every Rust function keeps an
`# Original` block with the same address, size and evidence path.

Status follows `docs/RUST_STYLE.md`: `translated` = the full control flow and
every call are represented; `partial` = control flow is represented but one
operand or callee body is missing; `stub` = address/role only. Markers inside
the Rust code: `// observed:`, `// inferred:`, `// unknown:`.

Terms: **DOL** is the Wii executable container; **r13** is the PowerPC
small-data base (`0x8042F680`); **r2** is the second small-data base
(`0x80433080`); **SDA/SDA2** are those small-data areas; **NW4R / lyt** is
NintendoWare's layout library (`.brlyt` layouts, `.brlan` animations). See
`docs/GLOSSARY.md`.

## Scope and result

| Item | Count |
|---|---:|
| New function bodies in this pass | **25** |
| Bytes represented | **6,392** |
| `engine::display` functions / bytes | 25 / 6,392 |
| `engine::frame` functions / bytes | 0 / 0 |
| Status `translated` | 25 |
| Status `partial` | 0 |
| Confidence `high` / `medium` | 16 / 9 |
| Invented symbol names | 0 |

Running totals for the two destination files after this pass:

| File | Functions | Bytes | Ranges covered |
|---|---:|---:|---|
| `src/engine/display.rs` | 47 | 10,512 | `800d0940..800d11f0`, `801568d0..8015b840` |
| `src/engine/frame.rs` | 4 | 384 | `800e4e40` plus three 8-byte getters (unchanged) |

### Selection

The lane brief names the engine band `0x800C0000..0x80100000` and the themes
"display/pause/save-banner, frame dispatch, scene transition, dSys helpers".
The two destinations were checked before selecting:

* `src/engine/frame.rs` already holds the only frame body in that band
  (`800e4e40`) and its three getters.
* The rest of the frame neighbourhood is already owned: `800e45f0..800e5480`
  by `engine::dsys`, `800e19b0..800e1f68` by `game::scene`, and
  `800c9f10..800c9f90` by `engine::startup`.
* The band does contain a pause cluster: `800d0940..800d11f0`. It is the
  per-frame driver that writes the `Pausewindow_c` selection and flags, so it
  belongs next to the state machine in `engine::display`, not in
  `engine::frame`.

The remaining bodies continue coverage band G
(`0x80154E50..0x8015B320`): the `gameScene` HUD sits between the translated
`Wm_2d_*` effects (`801590b0`, `80159960`) and the pause window, so the whole
`801568d0..8015a250` slice now reads as one HUD. No address in this pass
appeared in `ledger/functions.csv` before it was written.

## New bodies — pause-menu controller (`800d0940..800d11f0`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `800d0940` | `pause_menu_release` | Frees a pause-support object when the caller owns it. | translated | medium | `800d0940_FUN_800d0940.c` |
| `800d0b20` | `pause_menu_set_lock` | Sets/clears the controller's input-lock bit. | translated | high | `800d0b20_FUN_800d0b20.c` |
| `800d0b50` | `pause_menu_lock` | Returns the input-lock bit. | translated | high | `800d0b50_FUN_800d0b50.c` |
| `800d0b60` | `pause_menu_begin` | Starts the pause window from the stored player index. | translated | high | `800d0b60_FUN_800d0b60.c` |
| `800d0c00` | `pause_menu_select_button` | Clamps and stores the selected pause button. | translated | high | `800d0c00_FUN_800d0c00.c` |
| `800d0cb0` | `pause_menu_update` | Per-frame pause input: move, confirm, cancel. | translated | medium | `800d0cb0_FUN_800d0cb0.c` |
| `800d0e90` | `pause_menu_end` | Leaves the pause menu and marks all four slots. | translated | high | `800d0e90_FUN_800d0e90.c` |
| `800d11f0` | `pause_menu_request_player` | Opens the menu for one player and raises the request byte. | translated | high | `800d11f0_FUN_800d11f0.c` |

## New bodies — `gameScene` world-map HUD (`801568d0..8015a250`)

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `801568d0` | `player_slot_count` | Counts the connected player slots. | translated | high | `801568d0_FUN_801568d0.c` |
| `80158100` | `gamescene_hud_position_panes` | Writes eight pane positions from the layout offsets. | translated | medium | `80158100_FUN_80158100.c` |
| `801582b0` | `gamescene_hud_offset_pane` | Tilts one pane's hit rectangle from its corner flag. | translated | medium | `801582b0_FUN_801582b0.c` |
| `80158480` | `gamescene_hud_offset_panes` | Updates the three tilts and the selected pane's edge. | translated | medium | `80158480_FUN_80158480.c` |
| `80158580` | `gamescene_hud_restore_panes` | Restores the pane positions of active players. | translated | high | `80158580_FUN_80158580.c` |
| `801586f0` | `gamescene_hud_mount` | Mounts `gameScene/gameScene.arc` and binds its panes. | translated | high | `801586f0_FUN_801586f0.c` |
| `80158890` | `gamescene_hud_request_close` | Latches a close request and plays the close animation. | translated | high | `80158890_FUN_80158890.c` |
| `80158910` | `gamescene_hud_hit_test` | Tests the four player cursors against three rectangles. | translated | high | `80158910_FUN_80158910.c` |
| `80158a90` | `gamescene_hud_update_counters` | Animates the three counter values and their panes. | translated | high | `80158a90_FUN_80158a90.c` |
| `80158c50` | `gamescene_hud_commit_decrement` | Commits a whole counter decrement in one step. | translated | high | `80158c50_FUN_80158c50.c` |
| `80158d00` | `gamescene_hud_fade_decrement` | Fades a partial counter decrement over frames. | translated | medium | `80158d00_FUN_80158d00.c` |
| `80158f70` | `gamescene_hud_tick` | Ticks the counter and leaves the state when done. | translated | high | `80158f70_FUN_80158f70.c` |
| `80159250` | `gamescene_hud_hide_panes` | Saves pane positions/sets the shared colour before hiding. | translated | medium | `80159250_FUN_80159250.c` |
| `801594e0` | `gamescene_hud_show_panes` | Restores the saved pane colours and positions. | translated | medium | `801594e0_FUN_801594e0.c` |
| `801597a0` | `gamescene_hud_spawn_star_coin` | Spawns `Wm_2d_starcoinget` at a pane. | translated | medium | `801597a0_FUN_801597a0.c` |
| `80159af0` | `gamescene_hud_select_player_pane` | Chooses the star-coin pane of each of three rows. | translated | high | `80159af0_FUN_80159af0.c` |
| `8015a250` | `dgame_display_state_release` | Destructor callback of the four `dGameDisplay_c` descriptors. | translated | high | `8015a250_FUN_8015a250.c` |

Sizes: 64 + 40 + 12 + 144 + 168 + 404 + 144 + 100 = **1,076** (pause) and
88 + 420 + 452 + 244 + 356 + 412 + 124 + 376 + 436 + 172 + 272 + 224 + 656 +
332 + 220 + 444 + 88 = **5,316** (HUD), for **6,392** bytes total.

## What the pause-menu controller does

The controller is the object at `0x80429FD8`. It survives a single pause: the
outside caller `0x8005F5C0` invokes `pause_menu_set_lock` every frame, and the
controller keeps the last player and button so a reopened menu resumes where it
left off.

```text
pause_menu_request_player(player)          sound 0x7c, +0x1b = 1, request byte = 1
   -> pause_menu_begin()                   phase = 1, +0x18 |= 1, Pausewindow+0x291 = 1,
                                           Pausewindow+0x28c = 0x80354E60[player]
   -> pause_menu_update() every frame
        hold 0x800B3B50(-6) / missing player / Pausewindow+0x294  -> no input
        left/right                        -> pause_menu_select_button()
        confirm or locked view            -> pause_menu_end(); Pausewindow+0x293 = 1
        cancel                            -> sound 0x7a, Pausewindow+0x292 = 1
pause_menu_end()                           phase = 3, four slots -> 0x800DC990
```

Object layout used here:

| Offset | Width | Meaning | Confidence |
|---|---:|---|---|
| `+0x04` | word | menu phase (`1` opening, `3` closing) | observed/inferred |
| `+0x08` | word | selected button (`0..1`) | observed |
| `+0x0c` | word | player index | observed |
| `+0x10` | word | previous selection / debounce word | observed |
| `+0x18` | byte | flags: bit 0 active, bit 1 locked, bit 2 closed | observed |
| `+0x19` | byte | confirm latch | observed |
| `+0x1a` | byte | cleared on begin | observed |
| `+0x1b` | byte | "opened at least once" | observed |
| `+0x1d` | byte | set by the HUD close path | observed |

The controller writes only two `Pausewindow_c` fields directly (`+0x284`
button and `+0x28c` animation), plus the four transition flags
`+0x291..+0x293` that the pause state machine already consumes. That is why it
is documented in this file: it is the input half of the same state machine.

## What the `gameScene` HUD does

The HUD owns the `gameScene/gameScene.arc` layout with four pane groups:
`N_otasukeInfo_00` (0x14 panes), `P_collectOff_00` (10), `T_left_00` (0xB) and
`T_otaChuS_00` (4). Mounting binds all four tables and leaves five panes
visible. The runtime then runs three rectangles, three counter values and a
four-player pane restore.

Object layout used here (all offsets observed; the names are inferred from the
direction each value moves):

| Offset | Width | Meaning |
|---|---:|---|
| `+0x70` | object | NW4R layout; vtable pointer stored at `+0x78` |
| `+0x390` | object | `dGameDisplay_c` state interface (vtable `+0x18` changes state) |
| `+0x3dc` | word | displayed large counter, stepped down by 3 |
| `+0x3ec` | word | per-row star-coin state (`0` or `2`), stride 4 |
| `+0x400/+0x404` | word | counter tick and its limit |
| `+0x40c/+0x410` | word | decrement step count and step scale |
| `+0x414` | word | selected `P_collectOff_00` index (`6..9`) |
| `+0x418..+0x420` | word | three animated counter values |
| `+0x449/+0x44a` | byte | close request / closing latch |
| `+0x44d..+0x44f` | byte | per-rectangle hit latches |
| `+0x450` | byte | shared "hit" byte |
| `+0x453` | byte | star-coin pane update gate |
| `+0x454` | byte | per-player hidden flag (stride 1) |
| `+0x458..+0x484` | f32 | three hit rectangles: xlo,ylo,xhi,yhi |
| `+0x490` | word | root pane |
| `+0x494` | word[10] | `P_collectOff_00` panes |
| `+0x4bc` | word[0xB] | `T_left_00` panes |
| `+0x4e8` | word[0x14] | `N_otasukeInfo_00` panes |
| `+0x500/+0x504/+0x508` | word | the three panes whose `+0xb8` frame is animated |
| `+0x520/+0x524/+0x528` | word | effect panes positioned by `80158100` |

The state machine mirrors the pause window: `gamescene_hud_tick` ends in
`change_state(+0x390, 0x80377138)`, which is the fourth descriptor installed by
`dgame_display_register_states` (`80159f10`). The event is the same vtable
`+0x18` call the pause transitions use.

## Evidence quality notes

* The `802dcd5c`, `802dcd70`, `802dcd74`, `802dcda8` and `802dcdbc` calls that
  Ghidra prints inside `80158580`, `80159250` and `80159af0` are CodeWarrior
  `__savegprlr_*` / `__restgprlr_*` prologue helpers. They are not game logic
  and are omitted; the preserved `r3` (`hud`) is written as an explicit
  argument. This follows the treatment already recorded in
  `docs/COVERAGE_DISPLAY.md`.
* `globals::fn_80101960` (address `80101960`) was already translated in
  `engine::globals`. The pause controller calls it instead of repeating its
  body; the address and return meaning match that module.
* `r2` constants are read from their absolute addresses:
  `8042dbc0`, `8042dbc4`, `8042dbc8`, `8042dbcc`, `8042dbd0`, `8042dbb4` and
  `8042fc08`. No float value is invented.
* The pane visibility bit is the same `pane + 0xbb` bit 0 used by wave 4; the
  new code calls the existing `pane_set_flag0` helper through
  `hud_set_pane_visible`.
* `80159250` and `801594e0` open with `(**(code **)(...+0x68))()` and then read
  or write `+0x18..+0x1e` of the result. The Rust treats that result as a
  0x20-byte draw record. The save/restore pair proves the layout, but the
  callee's real return type is not named in the export.

## Uncertainties

1. **`800d0940` release operand.** Ghidra prints `FUN_802b9280()` with no
   operand. The two guards (`object != 0`, `free > 0`) and every sibling
   destructor in this file pass the object pointer, so the Rust passes
   `object`; marked `// partial:` in the doc.
2. **`800d0cb0` view return word.** `800b53a0` is decompiled as `void` and
   tail-calls a view vtable slot. The caller tests `r3`; the Rust reads
   `platform::call1`'s result and marks it inferred.
3. **Pause phase numbers.** `1` and `3` are proven by the stores and by
   `0x8005F5C0`'s reads; `2` never appears in the export, so no name is
   invented for it.
4. **`800b3b50` mask `-6`.** The callee is a single `flag_word & mask`; bit 0
   and bit 2 are excluded by the call. What those bits mean is unknown.
5. **`80159250` colour record.** The four halfword stores at `result + 0x18`
   are literal; whether the record is a pane, a draw object or a colour quad is
   inferred from the matching restore function.
6. **SDA2 float meanings.** The seven constants above are scale/offset values in
   the pane placement; their numeric values are not recovered and they are read
   from memory at run time.
7. **`804293b0` / `804293b4` / `8042fc08`.** The three data addresses passed to
   `800c8fb0` / `800c90f0` during the mount are only addresses in the export;
   their contents are not decoded here.
8. **`80159880` is not translated.** Its C compares `this + 0x3cc` inside a
   loop whose base pointer also advances; the export folds the loop index into
   the printed offset, so a literal transcription would be wrong. It is left
   for a pass that can check the instruction words.
9. **`80159eb0`, `80159ee0`, `8015b2c0`, `8015b2f0`.** These four 48-byte
   bodies are printed as three-argument tail calls to `802dcbfc` with duplicated
   operands; they are constructor thunks, not enough evidence for a body.

## Next targets

Ordered by value per effort, with the evidence file to read first.

| Priority | Target | Size | Why | First evidence |
|---:|---|---:|---|---|
| 1 | `8015b320` | 1,276 B | `Pausewindow_c` descriptor registration; completes the pause side | `8015b320_FUN_8015b320.c` |
| 2 | `80159880` | 224 B | four-player star-coin count update; needs the instruction check noted above | `80159880_FUN_80159880.c` |
| 3 | `80156ed0` + `80157060` + `80157120` | 104 + 108 + 124 B | construct/step/query of the 4 x 0x23c effect container | `80156ed0_FUN_80156ed0.c` |
| 4 | `801571a0`, `801571b0` | 16 + 108 B | per-slot seek and state-select helpers of the same container | `801571a0_FUN_801571a0.c` |
| 5 | `80157310`, `80157360`, `80157710` | 72 + 72 + 244 B | remaining container helpers | `80157310_FUN_80157310.c` |
| 6 | `80157850`, `80157890`, `801578f0` | 64 + 96 + 100 B | container flag/visibility helpers | `80157850_FUN_80157850.c` |
| 7 | `80156930`, `80156ab0` | 372 B each | player-in-rectangle predicates; carry GQR/`ldexpf` artefacts, check the instructions first | `80156930_FUN_80156930.c` |
| 8 | `80156c30`, `80156d70` | 312 + 132 B | related predicates without GQR artefacts | `80156c30_FUN_80156c30.c` |
| 9 | `800d0340`, `800d0470` | 96 + 360 B | drain/spawn of the 0x24-byte effect records next to the pause controller | `800d0340_FUN_800d0340.c` |
| 10 | `800d1510`, `800d1640`, `800d1f10` | 300 + 104 + 108 B | pointer/cursor selection class; needs a field model first | `800d1510_FUN_800d1510.c` |

Do not start with `800d0610` (656 B) or `800d19e0` (976 B): both carry
paired-single/GQR artefacts and need the 3-D view field model before a
translation can be faithful.

## Ledger

`ledger/agents/wave16-engine.csv` has one row per function above with the
seven-column header from `docs/RUST_STYLE.md`
(`module,address,name,status,confidence,evidence,rust_file`). The `module`
column is `engine::display` for every row and matches the `rust_file` path
`src/engine/display.rs`. The same 25 rows were appended to
`ledger/functions.csv`.
