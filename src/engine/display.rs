//! 2-D display effects and the pause window (`dGameDisplay_c`, `Pausewindow_c`).
//!
//! # What lives here
//!
//! This module is the semantic home for the engine-level 2-D presentation layer:
//! the `Pausewindow_c` state machine and the `Wm_2d_*` coin/1-up effects that
//! share the same display object. Four hundred and two functions are translated
//! here: the original ten pause/effect bodies, twelve wave-4 bodies that close the
//! pause animation, display-value and `dGameDisplay_c` descriptor gaps,
//! twenty-five wave-16 bodies that add the pause-menu controller of the engine
//! band and the `gameScene` world-map HUD around the `Wm_2d_*` effects,
//! twenty-two wave-20 bodies that add the pause cursor, the water-effect pool,
//! the save image and one display-object helper, seventeen wave-26 bodies
//! that add the cursor value helpers, the `Wm_mr_water*` actor and the
//! save-banner slot helper, thirty wave-38 bodies that add the
//! pad-button triples and the four-slot menu state container at
//! `0x800DB450..0x800DCDD0`, thirty wave-63 bodies that add the
//! `0x80303408` actor wrapper, the `0x80318A80`/`0x80318A90` node wrappers,
//! the spin-jump counter object at `r13 - 0x5880` and the embedded actor
//! search context of `0x800D9860..0x800D9AD0`, and thirty wave-76 bodies that
//! finish the actor-wrapper methods, build the `Wm_mr_spinjump` display
//! object, add the intrusive actor list node and four standalone helpers, and
//! twenty-eight wave-78 bodies that add the boot-screen readiness queries, the
//! four transition-scene gates, three state-object destructors and the
//! `dScBoot_c` strap/controller-info handlers of `0x8015BB90..0x8015DAA0`, and
//! twenty-nine wave-89 bodies that add the actor motion controller around
//! `0x800D7780..0x800D7FC0`, the course-cursor advance pair
//! `0x800D9290..0x800D96D0`, the embedded-interface shims of
//! `0x800D5C80..0x800DCF00` and the packet-record copy at `0x800DE290`, and
//! thirty wave-90 bodies that add the 4 x 0x23C record container of
//! `0x80156ED0..0x80157360`, the scene constructor and its two release helpers
//! at `0x80157710..0x80157890`, the boot-state tail of
//! `0x8015D1D0..0x8015DAF0` and the intrusive list plus state objects of
//! `0x8015F930..0x8015FF40`, and twenty-eight wave-93 bodies that add the
//! player-pad effect selectors of `0x800D0340..0x800D8EA0`, the two
//! state-registration tables at `0x800D7C80` and `0x800DE510`, and the
//! replay/insert helpers of `0x800DD770..0x800DFE60`, and twenty-six wave-95
//! bodies that add the scene-layout gate pair of `0x801501B0..0x80150830`, the
//! state handlers of `0x80151320..0x80152B30`, the allocation thunk, release
//! helper and player-bit resolver of `0x801533A0..0x80156D70`, and the
//! boot-screen and stepping helpers of `0x8015C4C0..0x8015F140`, and
//! twenty-five wave-99 bodies that add the actor teardown of `0x800D38A0`,
//! the interpolated-value and star-sprite helpers of
//! `0x800D4CD0..0x800D5170`, the state-selector family of
//! `0x800D5DD0..0x800D69B0`, the 0x80-byte command-packet builder at
//! `0x800D9760`, the two ground-collision filters of `0x800DAB40..0x800DAC40`,
//! the six `dRemoconMng_c` registration thunks and their descriptor bank of
//! `0x800DD1C0..0x800DD2B0`, and the release/interface helpers of
//! `0x800DD7D0..0x800DFCE0`, and twenty-five wave-100 bodies that add the
//! `daYoshi_c` state bank of `0x80153400` and its eleven table getters, the
//! `Pausewindow_c` registration at `0x8015B320`, the `dScBoot_c` state bank of
//! `0x8015DDD0`, the player-attribute rebind and route reset of
//! `0x80151630..0x80154740`, and six release thunks, and seven wave-105 bodies
//! that work the `0x800D0000` bucket: the effect-record list of `0x800D0470`,
//! the ten-slot water-pool spawn of `0x800D2CF0`, the four-slot record bands of
//! `0x800D8B00..0x800D8C60`, the replay-interface pair of
//! `0x800DFA40..0x800DFF30` and the `0x800D8680` thunk, and seven wave-106
//! bodies that reduce the `0x80150000` bucket from seventeen open addresses to
//! ten: the course reference-height getter of `0x80151A10`, the four-slot
//! region counter of `0x80156C30`, the per-slot effect switch of `0x80159880`
//! and the four `0x8015F3B0..0x8015F7B0` step/callback thunks, and ten
//! wave-108 bodies that reduce the `0x800D0000` bucket to one: the four-view
//! effect transform of `0x800D0610`, the cursor/effect grid of
//! `0x800D19E0..0x800D27B0`, the four-player star-colour layout of
//! `0x800D3380`, the table-selected effect dispatch of `0x800D6550`, the
//! ground sampler and shadow placement of `0x800D9D70..0x800DAA10`, and the
//! indexed record walk of `0x800DFDD0`, and nine wave-109 bodies that reduce the
//! `0x80150000` bucket from ten open addresses to one: the pending-item display
//! pass of `0x801502E0`, the interaction request handler of `0x80151E50`, the
//! committed-target selector of `0x80152CF0`, the two route-HUD layout passes of
//! `0x801548A0..0x80154A70`, the band-selected player mask of `0x80154E50`, the
//! two player-band scans of `0x80156930..0x80156AB0` and the four-byte CRC-32
//! thunk at `0x8015F130`, and one wave-114 body that closes the `0x800D0000`
//! bucket: the element bind/attach pass of `0x800D43E0`, and one wave-115 body
//! that closes the `0x80150000` bucket: the variadic report entry of
//! `0x8015F730`.
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8015A370` | [`pausewindow_construct`] | construct the pause window and its state managers |
//! | `0x8015A8D0` | [`pausewindow_close`] | close the layout and clear the global instance |
//! | `0x8015AC70` | [`pausewindow_begin_open`] | start the open animation and enter `OpenAnime` |
//! | `0x8015ADF0` | [`pausewindow_wait_open_animation`] | wait, then enter `ButtonChange` |
//! | `0x8015AEE0` | [`pausewindow_wait_button_animation`] | wait, then enter `PauseDisp` |
//! | `0x8015AF60` | [`pausewindow_update_display`] | choose changed-button, hit or close transition |
//! | `0x8015B050` | [`pausewindow_wait_hit_animation`] | wait, then enter `CloseAni` |
//! | `0x8015B0E0` | [`pausewindow_wait_close_animation`] | wait, then return to `InitWait` |
//! | `0x801590B0` | [`wm_spawn_1up_effects`] | spawn `Wm_2d_1up01/02` for one player |
//! | `0x80159960` | [`wm_set_coin100_display`] | update the `Wm_2d_coin100` effect |
//!
//! Wave 4 adds the two animation starters, the hint/button update pair, the
//! selected-button animation, the display-value setters and the release
//! helpers:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8015A910` | [`pausewindow_update_hint_animation`] | choose the controller hint frame for the current pause state |
//! | `0x8015AB20` | [`pausewindow_update_button_slots`] | show one button row and start its selection animation |
//! | `0x8015AD80` | [`pausewindow_start_open_animation`] | mark the layout ready and seek the open animations |
//! | `0x8015AE70` | [`pausewindow_start_button_animation`] | seek the previous/current button animations |
//! | `0x8015AC30` | [`pausewindow_play_selected_button`] | play the animation of the selected button |
//! | `0x8015A470` | [`fn_8015a470`] | conditional release of a display object |
//! | `0x8015A4B0` | [`fn_8015a4b0`] | release the embedded sub-object, then the storage |
//! | `0x8015A510` | [`fn_8015a510`] | release the embedded sub-object, then the storage |
//! | `0x80159AC0` | [`display_set_transform_value`] | set one value word and rebuild its transform |
//! | `0x80159CB0` | [`display_set_large_value`] | debounce and clamp a large display value |
//! | `0x80159F10` | [`dgame_display_register_states`] | register the four `dGameDisplay_c` state descriptors |
//! | `0x8015B840` | [`pausewindow_state_release`] | destructor callback registered by the pause descriptors |
//!
//! Wave 16 adds the pause-menu controller that lives in the engine band
//! `0x800C0000..0x80100000` and the `gameScene` world-map HUD that was still
//! untranslated between the two existing effect bodies:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x800D0940` | [`pause_menu_release`] | release a pause-support object when owned |
//! | `0x800D0B20` | [`pause_menu_set_lock`] | set/clear the pause-menu lock bit |
//! | `0x800D0B50` | [`pause_menu_lock`] | read the pause-menu lock bit |
//! | `0x800D0B60` | [`pause_menu_begin`] | start the pause window from the controller |
//! | `0x800D0C00` | [`pause_menu_select_button`] | clamp and store the selected pause button |
//! | `0x800D0CB0` | [`pause_menu_update`] | per-frame pause input: move, confirm, cancel |
//! | `0x800D0E90` | [`pause_menu_end`] | leave the pause menu and release the slots |
//! | `0x800D11F0` | [`pause_menu_request_player`] | open the pause menu for one player |
//! | `0x801568D0` | [`player_slot_count`] | count the connected player slots |
//! | `0x80158100` | [`gamescene_hud_position_panes`] | write eight pane positions from the layout offsets |
//! | `0x801582B0` | [`gamescene_hud_offset_pane`] | tilt one pane edge from its corner flag |
//! | `0x80158480` | [`gamescene_hud_offset_panes`] | update the three tilts and the selected pane |
//! | `0x80158580` | [`gamescene_hud_restore_panes`] | restore the pane positions of active players |
//! | `0x801586F0` | [`gamescene_hud_mount`] | mount `gameScene/gameScene.arc` and bind its panes |
//! | `0x80158890` | [`gamescene_hud_request_close`] | latch a close request and play the close animation |
//! | `0x80158910` | [`gamescene_hud_hit_test`] | test the four player cursors against three rectangles |
//! | `0x80158A90` | [`gamescene_hud_update_counters`] | animate the three counter values and panes |
//! | `0x80158C50` | [`gamescene_hud_commit_decrement`] | commit a whole counter decrement |
//! | `0x80158D00` | [`gamescene_hud_fade_decrement`] | fade a partial counter decrement |
//! | `0x80158F70` | [`gamescene_hud_tick`] | tick the counter and leave when it finishes |
//! | `0x80159250` | [`gamescene_hud_hide_panes`] | save the pane positions and apply the shared colour |
//! | `0x801594E0` | [`gamescene_hud_show_panes`] | restore the saved pane colours and positions |
//! | `0x801597A0` | [`gamescene_hud_spawn_star_coin`] | spawn `Wm_2d_starcoinget` |
//! | `0x80159AF0` | [`gamescene_hud_select_player_pane`] | choose the star-coin pane of each player |
//! | `0x8015A250` | [`dgame_display_state_release`] | destructor callback registered by `0x80159F10` |
//!
//! Wave 20 continues the engine band past the pause-menu controller. The
//! cursor cluster follows the controller; the water-effect pool is the
//! ten-slot container for the `Wm_mr_watersplash` spawn at `0x800D2AC0`; the
//! `0x800E01xx` cluster builds the in-memory save image that
//! [`crate::game::scene`] reads as `0x8035DCC0`.
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x800D14E0` | [`cursor_selection_active`] | read the cursor's selection-valid flag |
//! | `0x800D1500` | [`cursor_set_target`] | store a target pointer and its flag |
//! | `0x800D1510` | [`cursor_acquire_target`] | latch the first accepted target and clamp its position |
//! | `0x800D1640` | [`cursor_tick_lock`] | age the target debounce counters |
//! | `0x800D1F10` | [`cursor_begin_track`] | start the target-tracking animation |
//! | `0x800D2200` | [`cursor_unlink`] | remove the cursor from its list |
//! | `0x800D29F0` | [`water_effect_construct`] | construct one 0x13C-byte water effect |
//! | `0x800D2A50` | [`water_effect_release`] | release one water effect |
//! | `0x800D2C20` | [`water_effect_pool_construct`] | construct the ten-slot pool |
//! | `0x800D2C80` | [`water_effect_pool_release`] | destroy the pool |
//! | `0x800D2D80` | [`water_effect_pool_update`] | tick all ten elements |
//! | `0x800D3030` | [`water_node_construct`] | construct one 0x80-byte grid node |
//! | `0x800D3070` | [`water_node_release`] | release one grid node |
//! | `0x800E0180` | [`save_image_create`] | allocate and construct the 0x3FC0-byte save image |
//! | `0x800E01D0` | [`save_image_construct`] | build header plus two three-record arrays |
//! | `0x800E02D0` | [`save_image_refresh`] | copy the on-disc image into the live object |
//! | `0x800E0380` | [`save_image_record_a`] | select one record of array A |
//! | `0x800E03B0` | [`save_image_record_b`] | select one record of array B |
//! | `0x800E0430` | [`save_image_flag_get`] | read (and maybe clear) the dirty byte |
//! | `0x800E0470` | [`save_image_update_header_checksum`] | hash the header |
//! | `0x800E04B0` | [`save_image_update_checksums`] | hash the header and all six records |
//! | `0x800C9B00` | [`display_object_mark_dirty`] | set the display object's dirty byte when a resource resolves |
//!
//! # Original address range
//!
//! The pause-window cluster is coverage band G, `0x80154E50..0x8015B320`
//! (62 entries, 14,004 bytes). The three functions around `0x80159F10` are the
//! `dGameDisplay_c` state descriptors; the six `0x80377180..0x803772C0`
//! descriptors registered at `0x8015B320` are the `Pausewindow_c` states. The
//! `Wm_2d_*` effects are the `0x801590B0..0x80159960` slice of the same band.
//! Wave 4 also covers the pause-state destructor at `0x8015B840`, the first
//! entry of band H, because `0x8015B320` registers it as a `Pausewindow_c`
//! state callback.
//!
//! Wave 16 adds two neighbouring clusters. The pause-menu controller
//! `0x800D0940..0x800D11F0` sits in the engine band
//! `0x800C0000..0x80100000` (documented in `docs/COVERAGE_ENGINE_FRAME.md`);
//! it is the per-frame driver that writes `Pausewindow_c + 0x284..+0x293` and
//! therefore belongs to this state machine, not to `engine::frame`. The
//! `gameScene` world-map HUD `0x801568D0..0x8015A250` is the untranslated
//! middle of band G between the `Wm_2d_*` effects and the pause window. Both
//! clusters are tracked in `docs/COVERAGE_WAVE16_ENGINE.md`.
//!
//! Wave 20 covers three more slices of the same engine band, all tracked in
//! `docs/COVERAGE_WAVE20_ENGINE.md`:
//!
//! * `0x800D14E0..0x800D2200` - the cursor/target class that consumes the
//!   pause-menu selection;
//! * `0x800D29F0..0x800D3070` - the ten-slot water-effect pool (element stride
//!   `0x13C`, matching the fields of the `Wm_mr_watersplash` spawn at
//!   `0x800D2AC0`);
//! * `0x800E0180..0x800E04B0` - the 0x3FC0-byte save image (header `0x6A0`,
//!   two arrays of three `0x980`-byte records) and one display-object helper at
//!   `0x800C9B00`.
//!
//! Wave 63 covers three separate slices of the same engine band, all tracked
//! in `docs/COVERAGE_WAVE63_DISPLAY.md`:
//!
//! * `0x800D54A0..0x800D5790` - the `0x80303408` actor wrapper (constructor,
//!   destructor, open, matrix copy, transform rebuild and transform read);
//! * `0x800D6E00..0x800D70C0` - the `0x80318A80`/`0x80318A90` node wrappers
//!   that forward to that actor wrapper;
//! * `0x800D85D0..0x800D8840` - the single spin-jump counter object and its
//!   three-slot accessors, including the `0x800D8680` tick;
//! * `0x800D9860..0x800D9AD0` - the embedded actor search context, from its
//!   constructor to the `0x800D9940` list search and the `0x800D9A30`
//!   confirmation.
//!
//! Wave 76 covers four more slices of the same engine band, all tracked in
//! `docs/COVERAGE_WAVE76_DISPLAY.md`:
//!
//! * `0x800D5B90..0x800D6D10` - the remaining `0x80303408` actor-wrapper
//!   methods (display switch, pending-action test, animation bucket, 2-jump
//!   record copy, frame interpolation and action-window test) plus the two
//!   `0x80318A80` node constructors;
//! * `0x800D7480..0x800D8510` - the `Wm_mr_spinjump` display object:
//!   constructor, owner binding, six motion presets, model spawn, mode equality
//!   and the halfword timer;
//! * `0x800D9AC0..0x800DAED0` - the intrusive actor list node that the
//!   `0x800D9940` search walks, with its three candidate filters;
//! * `0x800DA650`, `0x800DC990` and `0x800DF860..0x800DF890` - a square-root
//!   helper, the wave-38 route-flag store and two table/oscillator helpers.
//!
//! Wave 78 covers the untranslated middle of coverage band H,
//! `0x8015BB90..0x8015DAA0` (28 bodies, 2,316 bytes), tracked in
//! `docs/COVERAGE_WAVE78_DISPLAY.md`. It is the boot-screen half of the band:
//! four readiness queries and the `wipe*` layout gates, three state-object
//! destructors, and the `dScBoot_c` Wii-strap/controller-info state handlers.
//! The scene-side functions of the same class stay in
//! [`crate::game::boot_scene`]; this file only owns the addresses that fall in
//! the `0x80150000` bucket.
//!
//! Wave 93 covers three slices of the same `0x800D0000` bucket, all tracked in
//! `docs/COVERAGE_WAVE93_DISPLAY.md`:
//!
//! * `0x800D0340..0x800D8EA0` - the per-player effect selectors, the
//!   course-marker geometry pair and the two `0x80303408`-neighbour display
//!   setters that share the `G_PLAYER_SLOTS` table;
//! * `0x800D7C80` and its sibling `0x800DE510` - the two four-state descriptor
//!   tables (`dPosGoAndComeExeFrm_c` and `dReplayPlay_c`);
//! * `0x800DD770..0x800DFE60` - the `dRemoconMng_c::dConnect_c` constructor and
//!   release shims, the replay HUD flag word and the event-stream decoder.
//!
//! Wave 100 finishes the `0x80150000` bucket, tracked in
//! `docs/COVERAGE_WAVE100_DISPLAY.md`:
//!
//! * `0x80153400` - the nineteen `daYoshi_c` state descriptors, installed from
//!   the 0x24-byte records at `0x80326D08..0x80326FB4`; the eleven
//!   riding/physics states also store the table pointer returned by the
//!   `0x801543C0..0x80154460` getters at `+0x30`;
//! * `0x8015B320` - the six `Pausewindow_c` descriptors plus the four globals
//!   seeded from `r2 - 0x54A0..0x5498` and `r2 - 0x6114`;
//! * `0x8015DDD0` - the twenty-four `dScBoot_c` descriptors, the two actor
//!   lists and the two SDA tables of the boot scene;
//! * `0x80151630..0x80154740` - the player attribute rebind, the route reset
//!   with its two pad masks, and the container release at `0x801578F0`;
//! * the six remaining `0x802DCBFC` release thunks
//!   (`0x801533D0`, `0x80159EB0`, `0x80159EE0`, `0x8015B2C0`, `0x8015B2F0`,
//!   `0x8015DDA0`).
//!
//! The address lane `0x80140000..0x80180000` is documented in
//! `docs/COVERAGE_r80140_80180.md`; the wave-4 part of this file is tracked in
//! `docs/COVERAGE_WAVE4_DISPLAY.md`.
//!
//! # Why it exists
//!
//! The 2-D layer is engine presentation, not the NW4R layout library:
//! `Pausewindow_c` owns the pause-menu state machine and `dGameDisplay_c` owns
//! the in-game HUD/effect transitions. Keeping them apart from `ui/lyt.rs`
//! means the layout primitives stay reusable and this file can be read as one
//! state machine.
//!
//! # Pause-window state machine
//!
//! The six descriptors are registered by `0x8015B320`; each is 0x40 bytes.
//! The state interface is a C++ object embedded at `Pausewindow_c + 0x208`;
//! its vtable slot `+0x18` changes the state and takes the descriptor address.
//!
//! ```text
//! InitWait (0x80377180)
//!    -- pausewindow_begin_open (0x8015AC70)
//!           -- OpenAnime (0x803771C0)
//!                  -- pausewindow_wait_open_animation (0x8015ADF0)
//!                         -- ButtonChange (0x80377200)
//!                                -- pausewindow_wait_button_animation (0x8015AEE0)
//!                                       -- PauseDisp (0x80377240)
//!                                              -- pausewindow_update_display (0x8015AF60)
//!                                                   -- changed button -> ButtonChange
//!                                                   -- hit flag      -> HitAnimeEnd (0x80377280)
//!                                                   -- close flag    -> CloseAni (0x803772C0)
//! HitAnimeEnd - pausewindow_wait_hit_animation (0x8015B050) - CloseAni
//! CloseAni   - pausewindow_wait_close_animation (0x8015B0E0) - InitWait
//! ```
//!
//! # Object layout used here (observed)
//!
//! Offsets are relative to the `Pausewindow_c` object unless stated otherwise.
//!
//! | offset | width | meaning | confidence |
//! |---|---|---|---|
//! | `+0x060` | word | base-class vtable pointer | observed |
//! | `+0x070` | word | embedded NW4R layout object | observed |
//! | `+0x208` | word | pause state interface; vtable slot `+0x18` changes state | observed |
//! | `+0x20C/+0x210` | word | interfaces passed to the state-manager constructor | observed |
//! | `+0x220` | object | state manager built by `0x8015FC90` | observed |
//! | `+0x250/+0x254` | word | layout-pane handles used by the pause controls | inferred |
//! | `+0x258..+0x264` | word | four animation panes started by `0x8015AC70` | inferred |
//! | `+0x284` | word | selected button index | observed |
//! | `+0x288` | word | previously selected button index | observed |
//! | `+0x28C` | word | animation handle passed to `0x800B3C50` | inferred |
//! | `+0x290..+0x295` | byte | state flags: close/hit/anim-ready/etc. | observed names unknown |
//!
//! Wave 4 adds the pane handles `+0x250`, `+0x268..+0x280` and the ready byte
//! `+0x294` to the same object. It also reads a separate display object through
//! the global `0x80429F7C`; that object's `+0x3B4` (player count), `+0x3B8`
//! (hint state), `+0x3E4` (gate) and `+0x3E8` (slot) words are documented in
//! the wave-4 constants block near the bottom of the file.
//!
//! Wave 16 adds two more layouts. The pause-menu controller at
//! `0x80429FD8` has `+0x04` (menu phase), `+0x08` (selected button), `+0x0C`
//! (player index), `+0x18` (flag byte: bit 0 = active, bit 1 = locked, bit 2 =
//! closed), `+0x19`, `+0x1A`, `+0x1B` (request/dirty bytes) and `+0x1D`. The
//! `gameScene` HUD object owns the layout at `+0x70`, its vtable at `+0x78`,
//! the state interface at `+0x390`, the two counters `+0x400/+0x404`, the
//! step/step-scale `+0x40C/+0x410`, the selected index `+0x414`, the counter
//! values `+0x418..+0x420`, the rects `+0x458..+0x484`, the pane arrays
//! `+0x494` (10 panes), `+0x4BC` (11 panes) and `+0x4E8` (0x14 panes), and the
//! position offsets `+0x544..+0x5A0`. All offsets are observed; the field
//! *names* are inferred from the direction each value moves.
//!
//! Wave 38 adds two more layouts. The three-byte pad latch at
//! `0x800DBCC0` has byte `+0` (blocked), `+1` (held) and `+2` (hold
//! counter); four latches are cleared by [`fn_800dbe30`] and
//! updated by [`fn_800dbe80`]. The four-slot container has its
//! vtable at `+0x00` and slot pointers at `+0x04..+0x10`. One slot
//! object has the index at `+0x04`, the second interface at
//! `+0x0C`, the state managers at `+0x2C` and `+0x74`, the lookup
//! result at `+0x50`, the ready byte at `+0x54`, the route flag at
//! `+0x55`, the latch at `+0x58` and the active interface at
//! `+0x5C`. All offsets are observed; the field *names* are
//! inferred from the calls each field feeds.
//!
//! Wave 63 adds four more layouts, all observed offsets with inferred names.
//! The `0x80303408` actor wrapper keeps its list word at `+0x04`, two
//! `0x38`-byte elements at `+0x28`, an embedded object at `+0x98`, a
//! twelve-word matrix at `+0xD8`, a configuration byte at `+0x150`, a word at
//! `+0x180` and six `0xC`-byte elements at `+0x190` (both array shapes come
//! from the destructor's descriptors). The `0x80318A80` node wrapper is only
//! an actor pointer at `+0x04` plus a ready word at `+0x08`. The single
//! `0x80318C30` spin-jump object keeps a three-slot mask at `+0x04`, three
//! frame counters at `+0x08/+0x0C/+0x10` and a saved copy of them at
//! `+0x84..+0x90`. The `0x80318E70` search context keeps its owner at `+0x04`,
//! the owner transform at `+0x08`, an intrusive link at `+0x0C`, the current
//! and fallback link words at `+0x10/+0x14`, a shadow word at `+0x28`, a
//! group byte at `+0x2E` and two flag bytes at `+0x30/+0x31`.
//!
//! Wave 76 adds three more layouts, again observed offsets with inferred names.
//! The same `0x80303408` actor wrapper grows the action words `+0x154/+0x158`,
//! the mask words `+0x160/+0x164`, the mode words `+0x168/+0x16C`, the display
//! mode at `+0x180`, the interpolated float pair `+0x184/+0x188` and the frame
//! counter `+0x18C`. The `0x80318C20` `Wm_mr_spinjump` display object keeps its
//! owner at `+0x04`, a mode word at `+0x08`, a halfword timer at `+0x0C`,
//! motion floats at `+0x10..+0x14`, an animation interface at `+0x14` and the
//! two target floats at `+0x13C`. The `0x80318E58` actor list node keeps its
//! previous/next links at `+0x08/+0x0C`, a word at `+0x10`, the linked flag at
//! `+0x44` and the reset bytes at `+0x43/+0x4A`.
//!
//! Wave 78 uses the `dScBoot_c` layout that
//! [`crate::game::boot_scene`] documents: the scene-state interface at `+0x74`
//! (slot `+0x18` changes state, slot `+0x28` returns the descriptor object and
//! its slot `+0x14` answers an event query), the second interface at `+0xB0`,
//! the resource handles `+0xF0/+0xF4`, the save/controller object at `+0xFC`,
//! the byte latches `+0x101/+0x103` and the timer pair `+0x108/+0x10C`. The
//! `wipe*` gates read four adjacent SDA slots at `0x80429C30..0x80429C48`; their
//! owner is not named in the export and they are kept as
//! `W78_G_TRANSITION_SLOT_0..3`.
//!
//! # Dependencies
//!
//! [`crate::platform`] supplies the volatile loads/stores and the indirect
//! calls. The NW4R layout entry points (`0x800C8950`, `0x800C92F0`,
//! `0x800C93D0`, `0x800C9500`, `0x800C9610`, `0x800C9640`, `0x800C9930`,
//! `0x800C9A60`, `0x800CDC60`) are called by address. Their reconstructed
//! documentation lives in [`crate::ui::lyt`], which is owned by another lane and
//! is therefore not imported here.
//!
//! Wave 16 also calls [`crate::engine::globals::fn_80101960`], the already
//! translated getter for the live `dGameDisplay_c` object, instead of repeating
//! its body here.
//!
//! # Start here
//!
//! Read [`pausewindow_construct`] for the object layout, then
//! [`pausewindow_begin_open`] and the four `wait_*` functions in order. The
//! `Wm_2d_*` pair at the bottom shows the display-effect call shape:
//! copy a 15-word transform, scale its first two components with
//! `0x800B37E0`, then spawn the effect by name through `0x8016C890`.
//! Wave 4 continues from [`pausewindow_begin_open`] into
//! [`pausewindow_start_open_animation`] and
//! [`pausewindow_start_button_animation`]; the hint switch lives in
//! [`pausewindow_update_hint_animation`]. For the display class itself, read
//! [`dgame_display_register_states`] as the four-descriptor counterpart of
//! the pause-window registration at `0x8015B320`.
//!
//! Wave 16 starts at [`pause_menu_update`] (`0x800D0CB0`) for the pause input
//! loop and at [`gamescene_hud_mount`] (`0x801586F0`) for the world-map HUD.
//! The HUD state machine mirrors the pause window: [`gamescene_hud_tick`]
//! (`0x80158F70`) ends in `change_state(+0x390, 0x80377138)`, one of the
//! `dGameDisplay_c` descriptors installed by [`dgame_display_register_states`].
//!
//! Wave 20 starts at [`cursor_acquire_target`] (`0x800D1510`) for the cursor
//! flow, [`water_effect_pool_construct`] (`0x800D2C20`) for the effect pool and
//! [`save_image_construct`] (`0x800E01D0`) for the save image.
//!
//! Wave 26 continues the same band:
//!
//! * [`cursor_target_value`] (`0x800D2270`) and [`cursor_position_sum`]
//!   (`0x800D2510`) close the two cursor value helpers that follow the wave-20
//!   target selection;
//! * [`water_effect_spawn`] (`0x800D2AC0`) and [`water_effect_update`]
//!   (`0x800D2B90`) fill the two water-effect bodies the wave-20 pool called by
//!   address;
//! * [`water_actor_construct`] (`0x800D2DD0`) and its release/update/model
//!   helpers (`0x800D2FF0..0x800D4660`) cover the `Wm_mr_water*` actor that
//!   owns the effect pool;
//! * [`save_banner_set_slot_visible`] (`0x800E0070`), [`save_record_release`]
//!   (`0x800E0290`) and [`save_manager_busy`] (`0x800E0450`) finish the
//!   save-banner/save-image neighbourhood.
//!
//! Wave 38 covers the next coherent block of the same engine band,
//! `0x800DB450..0x800DCDD0` (30 bodies, 4,044 bytes), tracked in
//! `docs/COVERAGE_WAVE38_DISPLAY.md`: the four pad-button triples
//! (`0x800DBCC0..0x800DBE80`) and the four-slot menu state container
//! published in the global at `r13 - 0x5658` (already
//! [`G_PLAYER_SLOTS`]), whose slot constructor
//! [`fn_800dc570`] builds two state managers and whose route
//! helpers (`0x800DC6F0`, `0x800DCA20..0x800DCDD0`) dispatch on the
//! route byte at view `+0x74` against the manager table at
//! `+0xE2C`. Read [`fn_800dbf50`] for the container, then
//! [`fn_800dc570`] for the slot, then [`fn_800dca20`] for the
//! dispatch.
//!
//! Wave 63 starts at [`fn_800d54a0`] for the `0x80303408` actor wrapper and
//! [`fn_800d8640`] / [`fn_800d8680`] for the spin-jump counter object. The
//! embedded search context is easiest to follow from [`fn_800d98c0`] (bind),
//! [`fn_800d9940`] (search) and [`fn_800d9a30`] (confirm).
//!
//! Wave 76 continues the actor wrapper from [`fn_800d5b90`] and starts the
//! `Wm_mr_spinjump` display object at [`fn_800d81e0`] (constructor),
//! [`fn_800d82a0`] (motion presets) and [`fn_800d8420`] (model spawn). The
//! intrusive list node reads best from [`fn_800dadb0`] (construct),
//! [`fn_800daed0`] (unlink) and [`fn_800d9d00`] (candidate filter).
//!
//! Wave 78 starts at [`fn_8015bb90`] for the readiness queries, at
//! [`fn_8015bf10`] for the four `wipe*` gates and at [`fn_8015cb20`] for the
//! `dScBoot_c` error/timer routing. The strap flow reads in order as
//! [`fn_8015ccc0`] (enter the key wait), [`fn_8015cdf0`] (wait or skip),
//! [`fn_8015ce80`] (fade out) and [`fn_8015d030`] (finish and drop the scene
//! reference).
//!
//! Wave 93 starts at [`fn_800d3ce0`] for the instance -> animation-name
//! switch, at [`fn_800d8850`] / [`fn_800d8bb0`] for the two per-player pad
//! effect selectors and at [`fn_800d7c80`] / [`fn_800de510`] for the two
//! four-state descriptor tables.
//!
//! Wave 95 starts at [`fn_80150700`] for the three `+0x1544` re-entry wrappers,
//! at [`fn_80151ad0`] / [`fn_80151c30`] for the two marker placements and at
//! [`fn_80151d00`] for the placement-row selector. The boot-screen tail reads
//! best from [`fn_8015c680`] (fade out) and [`fn_8015d7c0`] (resource grant).
//!
//! # Glossary
//!
//! * **2-D display** - the overlay layer that draws HUD and pause elements on
//!   top of the 3-D scene; not the NW4R layout library itself.
//! * **state descriptor** - a 0x40-byte record naming one state and the
//!   interface callbacks used while it is active.
//! * **state manager** - the object built by `0x8015FC90`; it owns the current
//!   descriptor and dispatches the state callbacks.
//! * **NW4R / lyt** - NintendoWare's layout system; `.brlyt` layouts and
//!   `.brlan` animations are addressed by pane handles and animation indices.
//! * **SDA / SDA2** - the PowerPC small-data areas based at `r13 = 0x8042F680`
//!   and `r2 = 0x80433080`; Ghidra prints them as `unaff_r13`/`unaff_r2`.
//!
//! # Confidence summary
//!
//! Eight of the original bodies are `high`: every store, branch and call is
//! visible. The two `Wm_2d_*` effects are `medium` because their transform reads
//! one SDA2 constant at `r2-0x54C0` whose value is not recovered; the constant
//! is read from its absolute address instead of being guessed.
//!
//! In wave 4, [`pausewindow_update_button_slots`],
//! [`pausewindow_start_open_animation`], [`pausewindow_start_button_animation`],
//! [`fn_8015a470`], [`fn_8015a4b0`], [`fn_8015a510`],
//! [`dgame_display_register_states`] and [`pausewindow_state_release`] are
//! `high`; the remaining four are `medium` because their SDA2 tables or the
//! meaning of the display-state words are inferred from the call sites.
//!
//! In wave 16, the eight pause-menu functions are `high` except
//! [`pause_menu_release`] and [`pause_menu_update`], which are `medium` because
//! one callee operand and one callee return word are dropped by the export.
//! [`player_slot_count`], [`gamescene_hud_mount`], [`gamescene_hud_hit_test`],
//! [`gamescene_hud_update_counters`], [`gamescene_hud_request_close`],
//! [`dgame_display_state_release`] and [`gamescene_hud_show_panes`] are `high`;
//! the pane-position and colour functions are `medium` because their float
//! constants live in SDA2 and the layout slot `+0x68` result layout is inferred.
//!
//! In wave 20, the save-image, pool-construction and list functions are `high`;
//! [`cursor_acquire_target`], [`cursor_begin_track`], [`water_effect_release`],
//! [`water_node_construct`] and [`display_object_mark_dirty`] are `medium`
//! because one callee operand, the five SDA2 clamp floats or the class names
//! are inferred from the call sites.
//!
//! In wave 63, the spin-jump counter accessors and the small wrapper bodies
//! are `high`; [`fn_800d54a0`], [`fn_800d5680`], [`fn_800d56f0`],
//! [`fn_800d5790`], [`fn_800d6eb0`], [`fn_800d9940`], [`fn_800d9a30`] and
//! [`fn_800d9ad0`] are `medium` because the export drops the receiver or the
//! forwarding operands of a virtual call, because an eight-byte return holds
//! the search collaborator, or because a list/link field role is inferred from
//! a sibling function rather than from the body itself.
//!
//! In wave 76, nineteen bodies are `high`: the display switch, pending-action
//! test, 2-jump record copy, the node constructors, the spin-jump bind/timer
//! bodies, the actor list node and the two table helpers. Eleven are `medium`:
//! [`fn_800d5d10`], [`fn_800d6a30`], [`fn_800d6ab0`], [`fn_800d80d0`],
//! [`fn_800d81e0`], [`fn_800d82a0`], [`fn_800d8420`], [`fn_800dadb0`],
//! [`fn_800dae00`], [`fn_800da650`] and [`fn_800df890`] because a callee
//! operand is dropped, an SDA2 constant's value is not recovered, or a field
//! role comes from a sibling function.
//!
//! In wave 78, nineteen bodies are `high`: the four readiness queries, the
//! scene-reference pair, the transition helper, two of the three destructors,
//! the player-count/state transitions, the display-flag updates and the timer
//! helpers. Nine are `medium`: [`fn_8015bf10`], [`fn_8015bf50`],
//! [`fn_8015bf90`] and [`fn_8015bfd0`] because the export drops the receiver
//! operand of the `wipe*` setup call, [`fn_8015c260`] because the free call's
//! receiver is dropped, [`fn_8015ca50`] and [`fn_8015cd40`] because the SDA2
//! float and the descriptor field roles are not recovered, and
//! [`fn_8015cdf0`] and [`fn_8015ced0`] because the event id, the view slot role
//! and the `+0x208/+0x209` byte pair are inferred from sibling handlers.
//!
//! In wave 89, ten bodies are `high`: the record-pointer reset, the four-call
//! animation seed, the node and actor release shims, the wrapper destructor,
//! the cursor advance step, the proximity test, the record copy and the two
//! empty hooks. Nineteen are `medium`: [`fn_800d3a30`], [`fn_800d4670`],
//! [`fn_800d4c10`], [`fn_800d5730`], [`fn_800d5c10`], [`fn_800d5c90`],
//! [`fn_800d6e90`], [`fn_800d7000`], [`fn_800d7020`], [`fn_800d7780`],
//! [`fn_800d7820`], [`fn_800d79b0`], [`fn_800d7c20`], [`fn_800d7c50`],
//! [`fn_800d9370`], [`fn_800d9560`], [`fn_800dc970`], [`fn_800dcee0`] and
//! [`fn_800dcf00`] because a virtual receiver or float operand is dropped by
//! the export, an SDA slot's owner is not recovered, the record/step field
//! roles are inferred from the sibling motion bodies, or the dispatched member's
//! role is not recovered.
//!
//! In wave 90, sixteen bodies are `high`: the record-container step, readiness
//! and select helpers, the two per-player table writers, the save-image rebuild,
//! six of the boot-state handlers and the intrusive list insert/unlink pair.
//! Fourteen are `medium`: [`fn_80156ed0`] and [`fn_80157710`] because the C++
//! base constructors are printed without a receiver, [`fn_80157850`] for the same
//! reason, [`fn_8015d1d0`], [`fn_8015d930`] and [`fn_8015d9f0`] because the
//! player/view queries lose their receiver and the `countLeadingZeros` idioms
//! only hold for the `0`/`1` cases, [`fn_8015d2a0`], [`fn_8015d3a0`] and
//! [`fn_8015daf0`] because the save-manager and boot-object field roles are
//! inferred, [`fn_8015dd70`] because the dispatched member's role is not
//! recovered,
//! [`fn_8015fa30`], [`fn_8015fb80`] and [`fn_8015ff40`] because a holder slot is
//! printed with no explicit receiver, and [`fn_8015fc90`] because its four
//! operands are recovered from its three call sites rather than from its own
//! export.
//!
//! In wave 93, six bodies are `high`: the instance-to-name switch
//! [`fn_800d3ce0`], the sweep step [`fn_800d74c0`], the replay flag update
//! [`fn_800ddc50`], the two release shims and the list counter
//! [`fn_800dfd20`]. Twenty-two are `medium`: [`fn_800d0340`],
//! [`fn_800d36e0`], [`fn_800d3ef0`], [`fn_800d4ad0`], [`fn_800d4d80`],
//! [`fn_800d5330`], [`fn_800d6940`], [`fn_800d70e0`], [`fn_800d7350`],
//! [`fn_800d7c80`], [`fn_800d8850`], [`fn_800d8bb0`], [`fn_800d8e40`],
//! [`fn_800d8ea0`], [`fn_800dd950`], [`fn_800ddab0`], [`fn_800de020`],
//! [`fn_800de510`], [`fn_800df990`], [`fn_800dfb70`], [`fn_800dfc70`] and
//! [`fn_800dfe60`] because a virtual receiver, a 64-bit debug-context word or a
//! callee operand is dropped by the export, an SDA2 float's value is not
//! recovered, or the `0x14`-byte effect-record stride is inferred from the
//! sibling [`fn_800d8e40`].
//!
//! In wave 95, twelve bodies are `high`: the three `+0x1544` re-entry wrappers,
//! the placement-table getter, the three small channel wrappers, the allocation
//! thunk, the release helper, the boot-state handshake, the fade-out handler
//! and the two-word clear. Fourteen are `medium`: [`fn_801501b0`],
//! [`fn_801504d0`], [`fn_80150540`], [`fn_80151320`], [`fn_80151ad0`],
//! [`fn_80151c30`], [`fn_80151d00`], [`fn_80152020`], [`fn_801527a0`],
//! [`fn_80152b30`], [`fn_80156820`], [`fn_80156d70`], [`fn_8015d7c0`] and
//! [`fn_8015f140`] because the export drops a virtual receiver or a callee
//! operand, because an SDA2 float's value is not recovered, or because the
//! `0x802DCD74` debug context and the `0x802F5918` byte-table layout are only
//! partially visible.

use crate::game::bases::interface_call;
use crate::game::bases::{interface_call2, interface_call_desc5, Descriptor};
use crate::engine::globals;
use crate::platform;

// ---------------------------------------------------------------------------
// Small-data anchors
// ---------------------------------------------------------------------------

/// PowerPC `r13` small-data base, set by `__init_registers` at `0x80004244`.
///
/// observed: `lis r13,0x8042` / `ori r13,r13,0xF680`; the same constant is
/// recorded in [`crate::ui::lyt`] and [`crate::engine::startup`].
const SDA_BASE: usize = 0x8042_F680;

/// PowerPC `r2` small-data-2 base, set at `0x8000423C`.
///
/// observed: `lis r2,0x8043` / `ori r2,r2,0x3080`. Only the `-0x54C0` float is
/// read by this module.
const SDA2_BASE: usize = 0x8043_3080;

/// `r2 - 0x54C0` -> `0x8042DBC0`: one `f32` used as the third word of the
/// effect transform.
///
/// `// unknown:` the value is not recovered from the export. The original
/// loads it with `lfs`; reading the absolute address preserves the operand
/// instead of inventing a float.
const F32_SDA2_54C0: usize = SDA2_BASE - 0x54C0;

/// `r13 - 0x5344` -> `0x8042A33C`: pointer to the live `Pausewindow_c`.
///
/// observed: written by `0x8015A370`, cleared by `0x8015A8D0`.
const G_PAUSEWINDOW: usize = SDA_BASE - 0x5344;

/// `r13 - 0x5240` -> `0x8042A440`: pointer to the object whose vtable slot
/// `+0x10` returns the current player/controller count.
///
/// `// inferred:` the returned value is tested as "at least one player" before
/// spawning the 1-up and coin effects.
const G_PLAYER_MANAGER: usize = SDA_BASE - 0x5240;

/// `r13 - 0x569E` -> `0x80429FE2`: byte that suppresses the 1-up effect.
///
/// `// unknown:` the owner of the flag is not named in the export.
const G_SUPPRESS_1UP: usize = SDA_BASE - 0x569E;

/// `r13 - 0x5460` -> `0x8042A220`: word tested as zero before the 1-up spawn.
///
/// `// unknown:` the field's owner is not recovered; the 1-up path only runs
/// while it is zero.
const G_1UP_GATE: usize = SDA_BASE - 0x5460;

/// `r13 - 0x51F8` -> `0x8042A488`: sound-system handle passed to
/// `0x80195380(sound, 0x78, 1)` when the pause selection changes.
const G_SOUND_SYSTEM: usize = SDA_BASE - 0x51F8;

// ---------------------------------------------------------------------------
// Pausewindow state descriptors (observed in `0x8015B320`)
// ---------------------------------------------------------------------------

/// `0x80377180`: `Pausewindow_c::StateID_InitWait`.
const PAUSE_STATE_INIT_WAIT: u32 = 0x8037_7180;

/// `0x803771C0`: `Pausewindow_c::StateID_OpenAnime`.
const PAUSE_STATE_OPEN_ANIME: u32 = 0x8037_71C0;

/// `0x80377200`: `Pausewindow_c::StateID_ButtonChange`.
const PAUSE_STATE_BUTTON_CHANGE: u32 = 0x8037_7200;

/// `0x80377240`: `Pausewindow_c::StateID_PauseDisp`.
const PAUSE_STATE_PAUSE_DISP: u32 = 0x8037_7240;

/// `0x80377280`: `Pausewindow_c::StateID_HitAnimeEnd`.
const PAUSE_STATE_HIT_ANIME_END: u32 = 0x8037_7280;

/// `0x803772C0`: `Pausewindow_c::StateID_CloseAni`.
const PAUSE_STATE_CLOSE_ANI: u32 = 0x8037_72C0;

// ---------------------------------------------------------------------------
// Vtable/interface words observed in the constructor `0x8015A370`
// ---------------------------------------------------------------------------

/// `0x80327D30`: `Pausewindow_c` base vtable written at `+0x60`.
const VTABLE_PAUSE_BASE: u32 = 0x8032_7D30;

/// `0x80327DB0`: state-interface vtable written at `+0x208` before the state
/// managers are built.
const VTABLE_PAUSE_IFACE_PRE: u32 = 0x8032_7DB0;

/// `0x80327D80`: final state-interface vtable written at `+0x208`.
const VTABLE_PAUSE_IFACE: u32 = 0x8032_7D80;

/// `0x80327DE0`: first interface table stored at `+0x210`.
const VTABLE_PAUSE_IFACE_210: u32 = 0x8032_7DE0;

/// `0x80327DF8`: second interface table stored at `+0x214`.
const VTABLE_PAUSE_IFACE_214: u32 = 0x8032_7DF8;

/// `0x802FEAF8`: shared interface table stored at `+0x20C`.
const VTABLE_SHARED_IFACE: u32 = 0x802F_EAF8;

// ---------------------------------------------------------------------------
// Named callees (called by address; definitions live outside this module)
// ---------------------------------------------------------------------------

/// `0x8006C420`: C++ base-object constructor, called by the pause-window
/// constructor.
const FN_8006C420: usize = 0x8006_C420;

/// `0x800C8950`: NW4R layout constructor (`layout` at `this + 0x70`).
const FN_800C8950: usize = 0x800C_8950;

/// `0x8015FC90`: state-manager constructor. Its real signature has four
/// arguments `(manager, interface_a, interface_b, initial_state)`; Ghidra
/// prints only one because it inlines through `0x8015F9A0`.
const FN_8015FC90: usize = 0x8015_FC90;

/// `0x800C9930`: closes/frees the layout and returns non-zero when it is gone.
const FN_800C9930: usize = 0x800C_9930;

/// `0x800C9640`: returns non-zero while any layout animation is still active.
const FN_800C9640: usize = 0x800C_9640;

/// `0x800C9610`: returns the "animation finished" byte for an animation index;
/// index `-1` means the current animation.
const FN_800C9610: usize = 0x800C_9610;

/// `0x800C9500`: advances every bound animation to its end state.
const FN_800C9500: usize = 0x800C_9500;

/// `0x800C93D0`: layout playback helper `(layout, slot, seek)`.
const FN_800C93D0: usize = 0x800C_93D0;

/// `0x800C92F0`: starts/stops animation `index` `(layout, index, seek)`.
const FN_800C92F0: usize = 0x800C_92F0;

/// `0x800C9A60`: plays an animation `(pane, handle, start, end, 0)`.
const FN_800C9A60: usize = 0x800C_9A60;

/// `0x800CDC60`: returns the global layout animation handle.
const FN_800CDC60: usize = 0x800C_DC60;

/// `0x800B3C50`: applies a colour/material animation pair.
const FN_800B3C50: usize = 0x800B_3C50;

/// `0x800B3B60`: builds a 2-D transform from a source word and writes it to a
/// destination word `(dst, src, animation, flag)`.
const FN_800B3B60: usize = 0x800B_3B60;

/// `0x800B37E0`: rescales the first two `f32` components of a transform.
const FN_800B37E0: usize = 0x800B_37E0;

/// `0x8016C890`: spawns a named 2-D effect `(name, 0, transform, 0, 0)`.
const FN_8016C890: usize = 0x8016_C890;

/// `0x80195380`: plays a sound effect through the sound system.
const FN_80195380: usize = 0x8019_5380;

/// `0x80315B98`: game/scene mode byte tested by the effect paths.
const G_SCENE_MODE_80315B98: usize = 0x8031_5B98;

/// `0x8032769C`: `"Wm_2d_1up01"` effect name.
const S_WM_2D_1UP01: usize = 0x8032_769C;

/// `0x8032_76A8`: `"Wm_2d_1up02"` effect name.
const S_WM_2D_1UP02: usize = 0x8032_76A8;

/// `0x8032_76E0`: `"Wm_2d_coin100"` effect name.
const S_WM_2D_COIN100: usize = 0x8032_76E0;

/// `0x8042DBB4` (`r2 - 0x54CC`): source word passed to `0x800B3B60` by
/// [`wm_set_coin100_display`].
///
/// `// unknown:` the word is an SDA2 constant; the call only needs its address.
const W_SDA2_54CC: usize = SDA2_BASE - 0x54CC;

// ---------------------------------------------------------------------------
// Shared memory and call helpers
// ---------------------------------------------------------------------------

/// Reads the big-endian word at `base + offset`.
#[inline]
unsafe fn read_word(base: usize, offset: usize) -> u32 {
    platform::read_u32(base + offset)
}

/// Writes the big-endian word `value` at `base + offset`.
#[inline]
unsafe fn write_word(base: usize, offset: usize, value: u32) {
    platform::write_u32(base + offset, value);
}

/// Reads the byte at `base + offset`.
#[inline]
unsafe fn read_byte(base: usize, offset: usize) -> u8 {
    platform::read_u8(base + offset)
}

/// Writes the byte `value` at `base + offset`.
#[inline]
unsafe fn write_byte(base: usize, offset: usize, value: u8) {
    platform::write_u8(base + offset, value);
}

/// Reads the big-endian word at an absolute global address.
#[inline]
unsafe fn read_global_word(addr: usize) -> u32 {
    platform::read_u32(addr)
}

/// Writes the big-endian word `value` at an absolute global address.
#[inline]
unsafe fn write_global_word(addr: usize, value: u32) {
    platform::write_u32(addr, value);
}

/// Reads the byte at an absolute global address.
#[inline]
unsafe fn read_global_byte(addr: usize) -> u8 {
    platform::read_u8(addr)
}

/// Calls a no-argument function that returns `u32`.
///
/// `crate::platform::call0` drops `r3`; the animation and manager helpers in
/// this module return handles or counts, so the return value is preserved.
#[inline]
unsafe fn call0_r(addr: usize) -> u32 {
    let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
    f()
}

/// Calls the virtual method at `this_arg`'s vtable slot `slot`.
#[inline]
unsafe fn vcall0(this_arg: usize, slot: usize) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call1(entry, this_arg as u32)
}

/// [`vcall0`] with one integer argument (`r4`).
#[inline]
unsafe fn vcall1(this_arg: usize, slot: usize, arg: u32) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call2(entry, this_arg as u32, arg)
}

/// `true` when `value > 0` for the Ghidra idiom
/// `countLeadingZeros(value - 1) >> 5 != 0`.
///
/// The idiom appears in `0x801590B0`, `0x80159960` and the boot controller
/// checks. It is kept as an explicit helper so the translation matches the
/// observed branch, including the `0x80000000` wrap case.
#[inline]
fn positive_test(value: u32) -> bool {
    value.wrapping_sub(1).leading_zeros() >> 5 != 0
}

/// Changes the state of the interface at `iface` through vtable slot `+0x18`.
///
/// observed: every pause/boot transition is
/// `(**(code **)(*(int *)(iface) + 0x18))(iface, descriptor)`.
#[inline]
unsafe fn change_state(iface: usize, descriptor: u32) {
    let _ = vcall1(iface, 0x18, descriptor);
}

/// Reads a big-endian `u16` at `base + offset`.
///
/// Wave 16 needs halfword access because `0x80159250` / `0x801594E0` copy each
/// `f32` as two halfwords, exactly as the PowerPC `lha`/`sth` pairs do.
#[inline]
unsafe fn read_half(base: usize, offset: usize) -> u16 {
    platform::read_u16(base + offset)
}

/// Writes a big-endian `u16` at `base + offset`.
#[inline]
unsafe fn write_half(base: usize, offset: usize, value: u16) {
    platform::write_u16(base + offset, value);
}

/// Reads an `f32` at `base + offset`.
#[inline]
unsafe fn read_float(base: usize, offset: usize) -> f32 {
    platform::read_f32(base + offset)
}

/// Writes an `f32` at `base + offset`.
#[inline]
unsafe fn write_float(base: usize, offset: usize, value: f32) {
    platform::write_f32(base + offset, value);
}

/// Reads the `f32` stored at an absolute SDA2 address.
#[inline]
unsafe fn read_const_f32(addr: usize) -> f32 {
    platform::read_f32(addr)
}

/// Writes a byte at an absolute global address.
#[inline]
unsafe fn write_global_byte(addr: usize, value: u8) {
    platform::write_u8(addr, value);
}

/// Calls the vtable entry `slot` of the layout object with no extra argument.
///
/// observed in `0x80158100`: the layout keeps its vtable pointer at
/// `this + 0x78` and passes `this + 0x70` as the receiver, so the usual
/// [`vcall0`] shape (vtable at `this + 0`) does not apply.
#[inline]
unsafe fn vcall_layout0(layout: usize, slot: usize) -> u32 {
    let vtable = read_word(layout, 0x78) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call1(entry, (layout + 0x70) as u32)
}

/// [`vcall_layout0`] with two integer arguments (`r4`, `r5`).
#[inline]
unsafe fn vcall_layout2(layout: usize, slot: usize, a: u32, b: u32) -> u32 {
    let vtable = read_word(layout, 0x78) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call3(entry, (layout + 0x70) as u32, a, b)
}

// ===========================================================================
// Pausewindow_c lifecycle
// ===========================================================================

/// Constructs the pause window, its embedded layout and its state managers.
///
/// # Original
/// - Ghidra: `FUN_8015a370` at `0x8015A370` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a370_FUN_8015a370.c`
///
/// # Arguments
/// * `pausewindow` - storage for the `Pausewindow_c` object. The caller owns
///   the memory; this function only initialises it.
///
/// # Returns
/// The same `pausewindow` pointer, matching the original `return param_1`.
///
/// # Side effects
/// Writes the base vtable at `+0x60`, constructs the NW4R layout at `+0x70`,
/// initialises the state interface at `+0x208`, builds the state manager at
/// `+0x220` with the `InitWait` descriptor, clears `+0x290`, stores the object
/// in the global at `0x8042A33C`, and returns the pointer.
///
/// # Safety
/// `pausewindow` must point to at least `0x290` writable bytes and the
/// constructors at `0x8006C420` / `0x800C8950` / `0x8015FC90` must be callable
/// on the target.
///
/// # Confidence
/// high - every store and constructor argument is visible in the export.
pub unsafe fn pausewindow_construct(pausewindow: usize) -> usize {
    // observed: FUN_8006c420(param_1); *(param_1 + 0x60) = &DAT_80327d30;
    //           FUN_800c8950(param_1 + 0x70);
    let _ = platform::call1(FN_8006C420, pausewindow as u32);
    write_word(pausewindow, 0x60, VTABLE_PAUSE_BASE);
    let _ = platform::call1(FN_800C8950, (pausewindow + 0x70) as u32);

    // observed: the pre-construction state interface words.
    write_word(pausewindow, 0x208, VTABLE_PAUSE_IFACE_PRE);
    write_word(pausewindow, 0x20C, VTABLE_SHARED_IFACE);
    write_word(pausewindow, 0x210, VTABLE_PAUSE_IFACE_210);
    write_word(pausewindow, 0x214, VTABLE_PAUSE_IFACE_214);
    write_word(pausewindow, 0x218, pausewindow as u32);
    write_word(pausewindow, 0x21C, 0);

    // observed: state-manager constructor with the InitWait descriptor.
    let _ = platform::call4(
        FN_8015FC90,
        (pausewindow + 0x220) as u32,
        (pausewindow + 0x20C) as u32,
        (pausewindow + 0x210) as u32,
        PAUSE_STATE_INIT_WAIT,
    );

    // observed: final interface vtable, byte flag and global instance.
    write_byte(pausewindow, 0x290, 0);
    write_word(pausewindow, 0x208, VTABLE_PAUSE_IFACE);
    write_global_word(G_PAUSEWINDOW, pausewindow as u32);
    pausewindow
}

/// Closes the pause window and clears the global live-instance pointer.
///
/// # Original
/// - Ghidra: `FUN_8015a8d0` at `0x8015A8D0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a8d0_FUN_8015a8d0.c`
///
/// # Arguments
/// * `pausewindow` - the object built by [`pausewindow_construct`].
///
/// # Returns
/// `true` when `0x800C9930` reports that the layout is closed, otherwise
/// `false`. The global pointer is cleared only in the `true` case.
///
/// # Side effects
/// Calls the layout teardown at `0x800C9930` and writes `0` to `0x8042A33C`
/// when it succeeds.
///
/// # Safety
/// `pausewindow + 0x70` must be a live layout object.
///
/// # Confidence
/// high - the body is a single call, one conditional store and a return.
pub unsafe fn pausewindow_close(pausewindow: usize) -> bool {
    // observed: iVar1 = FUN_800c9930(param_1 + 0x70);
    let closed = platform::call1(FN_800C9930, (pausewindow + 0x70) as u32) != 0;
    if closed {
        // observed: *(undefined4 *)(unaff_r13 + -0x5344) = 0;
        write_global_word(G_PAUSEWINDOW, 0);
    }
    closed
}

/// Starts the opening animation and transitions to `OpenAnime`.
///
/// # Original
/// - Ghidra: `FUN_8015ac70` at `0x8015AC70` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ac70_FUN_8015ac70.c`
///
/// # Arguments
/// * `pausewindow` - the live pause-window object.
///
/// # Side effects
/// Clears the selected index `+0x284`, sets the "opening" byte `+0x291`, applies
/// the colour animation, starts four animation panes when `+0x295` is set, and
/// requests `OpenAnime`.
///
/// # Safety
/// The pane words `+0x254`, `+0x258`..`+0x264` and the state interface at
/// `+0x208` must be live.
///
/// # Confidence
/// high - all arguments and the transition descriptor are visible.
pub unsafe fn pausewindow_begin_open(pausewindow: usize) {
    // +0x284: selected button index; +0x291: "opening" flag.
    write_word(pausewindow, 0x284, 0);
    write_byte(pausewindow, 0x291, 1);

    // observed: FUN_800b3c50(*(param_1 + 0x254), *(param_1 + 0x28c));
    // +0x254 is a pane handle, +0x28c a material/animation handle.
    let _ = platform::call2(
        FN_800B3C50,
        read_word(pausewindow, 0x254),
        read_word(pausewindow, 0x28C),
    );

    if read_byte(pausewindow, 0x295) != 0 {
        let handle = call0_r(FN_800CDC60);
        // observed: four calls to FUN_800c9a60(pane, handle, 0x32, 0x17/0x18, 0)
        // 0x32 is the start frame; 0x17/0x18 are the two end frames.
        let _ = platform::call5(
            FN_800C9A60,
            read_word(pausewindow, 0x258),
            handle,
            0x32,
            0x17,
            0,
        );
        let _ = platform::call5(
            FN_800C9A60,
            read_word(pausewindow, 0x25C),
            handle,
            0x32,
            0x17,
            0,
        );
        let _ = platform::call5(
            FN_800C9A60,
            read_word(pausewindow, 0x260),
            handle,
            0x32,
            0x18,
            0,
        );
        let _ = platform::call5(
            FN_800C9A60,
            read_word(pausewindow, 0x264),
            handle,
            0x32,
            0x18,
            0,
        );
    }

    change_state(pausewindow + 0x208, PAUSE_STATE_OPEN_ANIME);
}

/// Waits for the opening layout animation, then enters `ButtonChange`.
///
/// # Original
/// - Ghidra: `FUN_8015adf0` at `0x8015ADF0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015adf0_FUN_8015adf0.c`
///
/// # Arguments
/// * `pausewindow` - the live pause-window object.
///
/// # Side effects
/// When `0x800C9640` reports that no layout animation is active, requests the
/// `ButtonChange` descriptor.
///
/// # Safety
/// `pausewindow + 0x70` and the state interface at `+0x208` must be live.
///
/// # Confidence
/// high - one query, one branch and one transition.
pub unsafe fn pausewindow_wait_open_animation(pausewindow: usize) {
    if platform::call1(FN_800C9640, (pausewindow + 0x70) as u32) == 0 {
        change_state(pausewindow + 0x208, PAUSE_STATE_BUTTON_CHANGE);
    }
}

/// Waits for the button-change animation, then enters `PauseDisp`.
///
/// # Original
/// - Ghidra: `FUN_8015aee0` at `0x8015AEE0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015aee0_FUN_8015aee0.c`
///
/// # Arguments
/// * `pausewindow` - the live pause-window object.
///
/// # Side effects
/// Queries the current animation's finished byte with index `-1`; zero means
/// finished and requests the `PauseDisp` descriptor.
///
/// # Safety
/// `pausewindow + 0x70` and `+0x208` must be live.
///
/// # Confidence
/// high - the `-1` index and the transition are both explicit.
pub unsafe fn pausewindow_wait_button_animation(pausewindow: usize) {
    if platform::call2(FN_800C9610, (pausewindow + 0x70) as u32, 0xFFFF_FFFF) == 0 {
        change_state(pausewindow + 0x208, PAUSE_STATE_PAUSE_DISP);
    }
}

/// Pause-display update: chooses the next state from the button/hit/close flags.
///
/// # Original
/// - Ghidra: `FUN_8015af60` at `0x8015AF60` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015af60_FUN_8015af60.c`
///
/// # Arguments
/// * `pausewindow` - the live pause-window object.
///
/// # Side effects
/// * If `+0x292` is set, requests `CloseAni`.
/// * Else if `+0x293` is set, requests `HitAnimeEnd`.
/// * Else if the selected index `+0x288` differs from `+0x284`, plays sound
///   `0x78` and requests `ButtonChange`.
///
/// # Safety
/// `pausewindow + 0x208` must be a live state interface; the sound-system
/// global `0x8042A488` must be live when the index changes.
///
/// # Confidence
/// high - all three branches and the sound call are visible; the two flag bytes
/// are named by their observed transitions rather than by recovered symbols.
pub unsafe fn pausewindow_update_display(pausewindow: usize) {
    if read_byte(pausewindow, 0x292) == 0 {
        if read_byte(pausewindow, 0x293) == 0 {
            if read_word(pausewindow, 0x288) != read_word(pausewindow, 0x284) {
                // observed: FUN_80195380(*(unaff_r13 - 0x51f8), 0x78, 1);
                // 0x78 is the "selection changed" UI sound id.
                let _ = platform::call3(
                    FN_80195380,
                    read_global_word(G_SOUND_SYSTEM),
                    0x78,
                    1,
                );
                change_state(pausewindow + 0x208, PAUSE_STATE_BUTTON_CHANGE);
            }
        } else {
            change_state(pausewindow + 0x208, PAUSE_STATE_HIT_ANIME_END);
        }
    } else {
        change_state(pausewindow + 0x208, PAUSE_STATE_CLOSE_ANI);
    }
}

/// Waits for the hit animation, then enters `CloseAni`.
///
/// # Original
/// - Ghidra: `FUN_8015b050` at `0x8015B050` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015b050_FUN_8015b050.c`
///
/// # Arguments
/// * `pausewindow` - the live pause-window object.
///
/// # Side effects
/// Queries the current animation's finished byte and requests `CloseAni` when
/// it reaches zero.
///
/// # Safety
/// `pausewindow + 0x70` and `+0x208` must be live.
///
/// # Confidence
/// high - identical shape to [`pausewindow_wait_button_animation`].
pub unsafe fn pausewindow_wait_hit_animation(pausewindow: usize) {
    if platform::call2(FN_800C9610, (pausewindow + 0x70) as u32, 0xFFFF_FFFF) == 0 {
        change_state(pausewindow + 0x208, PAUSE_STATE_CLOSE_ANI);
    }
}

/// Waits for the close animation, clears the opening flag and returns to
/// `InitWait`.
///
/// # Original
/// - Ghidra: `FUN_8015b0e0` at `0x8015B0E0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015b0e0_FUN_8015b0e0.c`
///
/// # Arguments
/// * `pausewindow` - the live pause-window object.
///
/// # Side effects
/// Waits for the layout animation to finish, clears byte `+0x291`, then
/// requests the `InitWait` descriptor.
///
/// # Safety
/// `pausewindow + 0x70` and `+0x208` must be live.
///
/// # Confidence
/// high - the extra byte clear and the transition are explicit.
pub unsafe fn pausewindow_wait_close_animation(pausewindow: usize) {
    if platform::call2(FN_800C9610, (pausewindow + 0x70) as u32, 0xFFFF_FFFF) == 0 {
        write_byte(pausewindow, 0x291, 0);
        change_state(pausewindow + 0x208, PAUSE_STATE_INIT_WAIT);
    }
}

// ===========================================================================
// Wm_2d_* display effects
// ===========================================================================

/// Fills the 15-word effect transform used by the `Wm_2d_*` spawn calls.
///
/// The source object holds a 3x4 matrix at `+0x84..+0xB0`. The original
/// copies twelve of those words, one SDA2 constant, and two duplicates of the
/// first two words into a 15-word stack block. The exact offsets are repeated
/// in both effect functions, so they are factored here without changing any
/// store.
///
/// # Original
/// Host-side factoring of the stack stores in `0x801590B0` and `0x80159960`;
/// no binary address of its own.
///
/// # Safety
/// `dst` must point to 15 writable words and `src` to the matrix at `+0x84`.
///
/// # Confidence
/// high - every word index is copied from the observed stack layout.
#[inline]
unsafe fn fill_effect_transform(dst: *mut u32, src: usize) {
    let m = |off: usize| read_word(src, off);

    // observed word order from 0x801590B0 / 0x80159960.
    *dst.add(0) = m(0x90);
    *dst.add(1) = m(0xA0);
    *dst.add(2) = platform::read_u32(F32_SDA2_54C0);
    *dst.add(3) = m(0x84);
    *dst.add(4) = m(0x88);
    *dst.add(5) = m(0x8C);
    *dst.add(6) = m(0x90);
    *dst.add(7) = m(0x94);
    *dst.add(8) = m(0x98);
    *dst.add(9) = m(0x9C);
    *dst.add(10) = m(0xA0);
    *dst.add(11) = m(0xA4);
    *dst.add(12) = m(0xA8);
    *dst.add(13) = m(0xAC);
    *dst.add(14) = m(0xB0);
}

/// Spawns the two `Wm_2d_1up01` / `Wm_2d_1up02` effects for one player.
///
/// # Original
/// - Ghidra: `FUN_801590b0` at `0x801590B0` (404 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801590b0_FUN_801590b0.c`
///
/// # Arguments
/// * `wm` - the world-map/display object that owns the per-player slots.
/// * `player` - player index `0..3`; used as the stride-4 slot index.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// When the player manager reports at least one player, the suppression flags
/// are clear, the mode is not `3`, and the per-player `+0x424` slot is zero:
/// writes `0xF` to `wm + player*4 + 0x424`, builds the transform from the
/// object at `wm + (player+9)*4 + 0x4E8`, and spawns both named effects.
///
/// # Safety
/// `wm` must be a live display object; the player-manager global
/// `0x8042A440` must point to a live object when `wm` is live.
///
/// # Confidence
/// medium - the flow and every call are observed, but the transform's third
/// component is the unresolved SDA2 `f32` at `0x8042DBC0`.
pub unsafe fn wm_spawn_1up_effects(wm: usize, player: u32) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);

    if positive_test(players)
        && read_global_byte(G_SUPPRESS_1UP) == 0
        && read_global_word(G_1UP_GATE) == 0
        && read_global_byte(G_SCENE_MODE_80315B98) != 3
    {
        let slot = wm + (player as usize) * 4;
        // +0x424: per-player re-trigger flag; 0xF is the observed cooldown.
        if read_word(slot, 0x424) == 0 {
            write_word(slot, 0x424, 0xF);

            let source = read_word(wm, (player as usize + 9) * 4 + 0x4E8) as usize;
            let mut transform = [0u32; 15];
            fill_effect_transform(transform.as_mut_ptr(), source);

            // observed: FUN_800b37e0(&local_48, 0)
            let _ = platform::call2(
                FN_800B37E0,
                transform.as_mut_ptr() as u32,
                0,
            );
            let _ = platform::call5(
                FN_8016C890,
                S_WM_2D_1UP01 as u32,
                0,
                transform.as_ptr() as u32,
                0,
                0,
            );

            // observed: local_48 = local_2c; local_44 = local_1c;
            transform[0] = transform[6];
            transform[1] = transform[10];
            transform[2] = platform::read_u32(F32_SDA2_54C0);

            // observed: FUN_800b37e0(&local_48, 1)
            let _ = platform::call2(
                FN_800B37E0,
                transform.as_mut_ptr() as u32,
                1,
            );
            let _ = platform::call5(
                FN_8016C890,
                S_WM_2D_1UP02 as u32,
                0,
                transform.as_ptr() as u32,
                0,
                0,
            );
        }
    }
}

/// Updates the `Wm_2d_coin100` effect when the player's coin count changes.
///
/// # Original
/// - Ghidra: `FUN_80159960` at `0x80159960` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159960_FUN_80159960.c`
///
/// # Arguments
/// * `wm` - the world-map/display object.
/// * `player` - player index stored at `wm + 0x3DC`.
///
/// # Side effects
/// If byte `+0x453` is clear and the cached player index differs, stores the
/// new index, calls `0x800B3B60(wm+0x3DC, r2-0x54CC, wm+0x4DC, 0)`, and for
/// player `0` with `+0x444 == 0` spawns `Wm_2d_coin100` after the same
/// transform setup as [`wm_spawn_1up_effects`].
///
/// # Safety
/// `wm` must be a live display object; the player-manager global and the
/// object at `wm + 0x51C` must be live when the spawn branch is taken.
///
/// # Confidence
/// medium - the flow is complete; one transform word is the unresolved SDA2
/// constant, and the `0x800B3B60` operand is passed as an address exactly as
/// Ghidra shows it.
pub unsafe fn wm_set_coin100_display(wm: usize, player: u32) {
    // +0x453 suppresses the effect; +0x3DC is the cached player index.
    if read_byte(wm, 0x453) == 0 && read_word(wm, 0x3DC) != player {
        write_word(wm, 0x3DC, player);

        // observed: FUN_800b3b60(param_1 + 0x3dc, unaff_r2 - 0x54cc,
        //                         *(param_1 + 0x4dc), 0)
        let _ = platform::call4(
            FN_800B3B60,
            (wm + 0x3DC) as u32,
            W_SDA2_54CC as u32,
            read_word(wm, 0x4DC),
            0,
        );

        if player == 0 && read_word(wm, 0x444) == 0 {
            let manager = read_global_word(G_PLAYER_MANAGER) as usize;
            let players = vcall0(manager, 0x10);

            // observed: ((1 < DAT_80315b98 - 2U) && (clz(players - 1) >> 5 != 0))
            if positive_test(players)
                && read_global_byte(G_SCENE_MODE_80315B98).wrapping_sub(2) > 1
            {
                let source = read_word(wm, 0x51C) as usize;
                let mut transform = [0u32; 15];
                fill_effect_transform(transform.as_mut_ptr(), source);

                let _ = platform::call2(
                    FN_800B37E0,
                    transform.as_mut_ptr() as u32,
                    0,
                );
                let _ = platform::call5(
                    FN_8016C890,
                    S_WM_2D_COIN100 as u32,
                    0,
                    transform.as_ptr() as u32,
                    0,
                    0,
                );
            }
        }
    }
}

// ===========================================================================
// Wave 4: pause animation, hint selection and display values
// (`0x8015A470..0x8015B840`)
// ===========================================================================

// ---------------------------------------------------------------------------
// Wave 4 small-data anchors and callees
// ---------------------------------------------------------------------------

/// `r13 - 0x5704` -> `0x80429F7C`: pointer to the object that owns the 2-D
/// player/button state words at `+0x3B4`, `+0x3B8`, `+0x3E4` and `+0x3E8`.
///
/// `// inferred:` the export only shows the indirect access; the field roles
/// (player slot, hint state, input gate) come from the two update functions
/// that share this global.
const G_DISPLAY_OBJECT: usize = SDA_BASE - 0x5704;

/// `r13 - 0x5700` -> `0x80429F80`: input/controller flag word. Bit `0x40`
/// selects the "no button row" branch of [`pausewindow_update_button_slots`].
const G_DISPLAY_INPUT_FLAGS: usize = SDA_BASE - 0x5700;

/// `0x80315B9C`: byte copied into the display player-count word and clamped to
/// `9` before the `+1` bias is applied.
const DAT_80315B9C: usize = 0x8031_5B9C;

/// `0x80315B9D`: pause hint/state selector; the switch in
/// [`pausewindow_update_hint_animation`] tests `0x14..0x28`.
const DAT_80315B9D: usize = 0x8031_5B9D;

/// `r2 - 0x54A8` -> `0x8042DBD8`: per-button index table indexed by the
/// selected button `+0x284`.
///
/// `// unknown:` the table contents are data; only the address is recovered.
const TABLE_SDA2_54A8: usize = SDA2_BASE - 0x54A8;

/// `r2 - 0x54C8` -> `0x8042DBB8`: source word passed to `0x800B3B60` by
/// [`display_set_transform_value`].
///
/// `// unknown:` the word is an SDA2 constant; the call only needs its address.
const W_SDA2_54C8: usize = SDA2_BASE - 0x54C8;

/// `r2 - 0x54C4` -> `0x8042DBBC`: source word passed to `0x800B3B60` by
/// [`display_set_large_value`].
///
/// `// unknown:` the word is an SDA2 constant; the call only needs its address.
const W_SDA2_54C4: usize = SDA2_BASE - 0x54C4;

/// `0x8015F7C0`: state-descriptor constructor `(descriptor, name)`.
const FN_8015F7C0: usize = 0x8015_F7C0;

/// `0x802DC780`: descriptor registration `(descriptor, destructor, link)`.
const FN_802DC780: usize = 0x802D_C780;

/// `0x8015A250`: destructor callback shared by the four `dGameDisplay_c`
/// descriptors.
///
/// The body is translated as [`dgame_display_state_release`]; the registration
/// call still passes the address, exactly as the original does.
const FN_8015A250: usize = 0x8015_A250;

/// `0x80327980`: common state vtable written at descriptor `+0`.
const VTABLE_DISPLAY_STATE: u32 = 0x8032_7980;

/// `0x80102750`: three-argument controller/state query used by the hint
/// switch with masks `0x80` and `0x200`.
const FN_80102750: usize = 0x8010_2750;

/// `0x800FBFF0`: boolean query used by hint case `0x25`.
const FN_800FBFF0: usize = 0x800F_BFF0;

/// `0x800FB3C0`: first boolean query used by hint case `0x26`.
const FN_800FB3C0: usize = 0x800F_B3C0;

/// `0x800FB460`: second boolean query used by hint case `0x26`.
const FN_800FB460: usize = 0x800F_B460;

/// `0x800B44D0`: applies one animation/transform pair `(pane, 0, 0)`.
const FN_800B44D0: usize = 0x800B_44D0;

/// `0x8015F800`: generic state/name destructor called with `(object, 0)`.
const FN_8015F800: usize = 0x8015_F800;

/// `0x8015F9F0`: destroys the embedded sub-object at `object + 0x18`.
const FN_8015F9F0: usize = 0x8015_F9F0;

/// `0x802B9280`: frees an object when the caller's release flag is positive.
///
/// Ghidra prints `FUN_802b9280()` without operands in several bodies; sibling
/// lanes reconstruct the same one-argument release call, and the call sites
/// here pass the object pointer explicitly.
const FN_802B9280: usize = 0x802B_9280;

/// Sets or clears bit 0 of the pane flags byte at `pane + 0xBB`.
///
/// `// inferred:` bit 0 is the pane enable/visibility flag; the export only
/// exposes the read-modify-write sequence.
#[inline]
unsafe fn pane_set_flag0(pane: u32, set: bool) {
    let addr = pane as usize + 0xBB;
    let value = platform::read_u8(addr);
    let value = if set { value | 1 } else { value & 0xFE };
    platform::write_u8(addr, value);
}

/// Updates the controller-hint animation for the current pause state.
///
/// # Original
/// - Ghidra: `FUN_8015a910` at `0x8015A910` (524 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a910_FUN_8015a910.c`
///
/// # Arguments
/// * `pausewindow` - live `Pausewindow_c`. Ghidra prints no parameter because
///   the body starts with the CodeWarrior prologue helper `0x802DCD74`
///   (`__savegprlr_27`); the caller `0x8015AB20` passes its own `this` in `r3`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Writes `display + 0x3B4 = min(DAT_80315B9C, 9) + 1`.
/// * Starts animation `0x51` on pane `+0x268`.
/// * Selects a hint frame from `DAT_80315B9D` and starts either the left pane
///   `+0x26C` (default case) or the right pane `+0x270`.
/// * The default case also writes `display + 0x3B8 = state + 1`.
///
/// # Safety
/// `pausewindow + 0x268..+0x280` must hold live pane handles and the global
/// `0x80429F7C` must point to a live display object.
///
/// # Confidence
/// medium - every branch, store and call is observed; the frame meanings come
/// from the case values and are marked `inferred` rather than named.
pub unsafe fn pausewindow_update_hint_animation(pausewindow: usize) {
    let handle = call0_r(FN_800CDC60);
    let display = read_global_word(G_DISPLAY_OBJECT) as usize;

    let mut player_slot = read_global_byte(DAT_80315B9C) as u32;
    let state = read_global_byte(DAT_80315B9D) as u32;
    if player_slot > 9 {
        player_slot = 9;
    }
    write_word(display, 0x3B4, player_slot + 1);

    let _ = platform::call5(
        FN_800C9A60,
        read_word(pausewindow, 0x268),
        handle,
        2,
        0x51,
        0,
    );

    let frame: u32 = match state {
        0x14 => 0x55,
        0x15 | 0x16 => 0x54,
        0x17 | 0x18 => {
            if player_slot == 7 {
                0x5D
            } else {
                0x53
            }
        }
        0x19..=0x1F => {
            // observed: FUN_80102750(player_slot, state, 0x80/0x200)
            if platform::call3(FN_80102750, player_slot, state, 0x80) != 0 {
                0x5B
            } else if platform::call3(FN_80102750, player_slot, state, 0x200) != 0 {
                0x5C
            } else {
                0x58
            }
        }
        0x20..=0x22 => 0x5E,
        0x23 => 0x56,
        0x24 => 0x5F,
        0x25 => {
            if call0_r(FN_800FBFF0) == 0 {
                0x57
            } else {
                0x5F
            }
        }
        0x26 => {
            if call0_r(FN_800FB3C0) != 0 {
                0x5C
            } else if call0_r(FN_800FB460) != 0 {
                0x58
            } else {
                0x5B
            }
        }
        0x28 => 0x5A,
        _ => {
            // observed: default arm; the left pane shows frame 0x52 and the
            // display hint word advances before the function returns.
            write_word(display, 0x3B8, state + 1);
            pane_set_flag0(read_word(pausewindow, 0x26C), true);
            let _ = platform::call5(
                FN_800C9A60,
                read_word(pausewindow, 0x26C),
                handle,
                2,
                0x52,
                0,
            );
            pane_set_flag0(read_word(pausewindow, 0x270), false);
            return;
        }
    };

    pane_set_flag0(read_word(pausewindow, 0x26C), false);
    pane_set_flag0(read_word(pausewindow, 0x270), true);
    let _ = platform::call5(
        FN_800C9A60,
        read_word(pausewindow, 0x270),
        handle,
        2,
        frame,
        0,
    );
}

/// Shows one button row and starts its selection animation.
///
/// # Original
/// - Ghidra: `FUN_8015ab20` at `0x8015AB20` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ab20_FUN_8015ab20.c`
///
/// # Arguments
/// * `pausewindow` - live `Pausewindow_c`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Enables pane `+0x278` and disables panes `+0x27C`/`+0x280`.
/// * When the input gate is open (`0x80429F80 & 0x40 != 0`), the display
///   `+0x3E4` word is zero and the display slot is `<= 4`, enables panes
///   `+0x26C`/`+0x250`, writes `display + 0x3B8 = slot + 1` and starts frame
///   `0x52` on `+0x26C`.
/// * Otherwise disables `+0x250` and delegates to
///   [`pausewindow_update_hint_animation`].
///
/// # Safety
/// `pausewindow + 0x250..+0x280` must hold live pane handles and the global
/// `0x80429F7C` must point to a live display object.
///
/// # Confidence
/// high - every pane write and both branches are explicit; the gate words are
/// named by their role in this function.
pub unsafe fn pausewindow_update_button_slots(pausewindow: usize) {
    pane_set_flag0(read_word(pausewindow, 0x278), true);
    pane_set_flag0(read_word(pausewindow, 0x27C), false);
    pane_set_flag0(read_word(pausewindow, 0x280), false);

    let handle = call0_r(FN_800CDC60);
    let display = read_global_word(G_DISPLAY_OBJECT) as usize;
    let mut slot = read_word(display, 0x3E8) as i32;
    if slot > 9 {
        slot -= 10;
    }

    if (read_global_word(G_DISPLAY_INPUT_FLAGS) & 0x40) == 0
        || read_word(display, 0x3E4) != 0
        || slot > 4
    {
        pane_set_flag0(read_word(pausewindow, 0x250), false);
        pausewindow_update_hint_animation(pausewindow);
    } else {
        pane_set_flag0(read_word(pausewindow, 0x26C), true);
        pane_set_flag0(read_word(pausewindow, 0x270), false);
        pane_set_flag0(read_word(pausewindow, 0x250), true);
        write_word(display, 0x3B8, (slot + 1) as u32);
        let _ = platform::call5(
            FN_800C9A60,
            read_word(pausewindow, 0x26C),
            handle,
            2,
            0x52,
            0,
        );
    }
}

/// Marks the layout animation ready and seeks the open animations.
///
/// # Original
/// - Ghidra: `FUN_8015ad80` at `0x8015AD80` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ad80_FUN_8015ad80.c`
///
/// # Arguments
/// * `pausewindow` - live `Pausewindow_c`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets byte `+0x294`, advances every bound animation to its end state through
/// `0x800C9500`, seeks slots `3` and `4`, then starts animation `0`.
///
/// # Safety
/// `pausewindow + 0x70` must be a live layout object.
///
/// # Confidence
/// high - one byte store and four layout calls, all explicit.
pub unsafe fn pausewindow_start_open_animation(pausewindow: usize) {
    write_byte(pausewindow, 0x294, 1);
    let _ = platform::call1(FN_800C9500, (pausewindow + 0x70) as u32);
    let _ = platform::call3(FN_800C93D0, (pausewindow + 0x70) as u32, 3, 0);
    let _ = platform::call3(FN_800C93D0, (pausewindow + 0x70) as u32, 4, 0);
    let _ = platform::call3(FN_800C92F0, (pausewindow + 0x70) as u32, 0, 0);
}

/// Seeks the previous and current button animations.
///
/// # Original
/// - Ghidra: `FUN_8015ae70` at `0x8015AE70` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ae70_FUN_8015ae70.c`
///
/// # Arguments
/// * `pausewindow` - live `Pausewindow_c`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Advances every bound animation, sets byte `+0x294`, seeks the previous
/// button animation `+0x288 + 7` when `+0x288 >= 0`, copies `+0x284` into
/// `+0x288`, then starts the current button animation `+0x284 + 3`.
///
/// # Safety
/// `pausewindow + 0x70` must be a live layout object.
///
/// # Confidence
/// high - the signed guard and both index biases are explicit.
pub unsafe fn pausewindow_start_button_animation(pausewindow: usize) {
    let _ = platform::call1(FN_800C9500, (pausewindow + 0x70) as u32);
    write_byte(pausewindow, 0x294, 1);

    let previous = read_word(pausewindow, 0x288) as i32;
    if previous >= 0 {
        let _ = platform::call3(
            FN_800C92F0,
            (pausewindow + 0x70) as u32,
            (previous + 7) as u32,
            0,
        );
    }

    write_word(pausewindow, 0x288, read_word(pausewindow, 0x284));
    let selected = read_word(pausewindow, 0x284);
    let _ = platform::call3(
        FN_800C92F0,
        (pausewindow + 0x70) as u32,
        selected + 3,
        0,
    );
}

/// Plays the animation associated with the selected pause button.
///
/// # Original
/// - Ghidra: `FUN_8015ac30` at `0x8015AC30` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ac30_FUN_8015ac30.c`
///
/// # Arguments
/// * `pausewindow` - live `Pausewindow_c`; `+0x284` selects the button.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Reads `TABLE_54A8[selected]`, resolves the pane word at
/// `pausewindow + index*4 + 0x244`, and calls `0x800B44D0(pane, 0, 0)`.
///
/// # Safety
/// `+0x284` must be a valid index into the SDA2 table, and the resolved pane
/// word must be a live layout pane.
///
/// # Confidence
/// medium - the table index and the final call are exact; the table's contents
/// are not recovered, so it is read from `0x8042DBD8`.
pub unsafe fn pausewindow_play_selected_button(pausewindow: usize) {
    let selected = read_word(pausewindow, 0x284) as usize;
    let slot = platform::read_i32(TABLE_SDA2_54A8 + selected * 4) as usize;
    let pane = read_word(pausewindow, slot * 4 + 0x244);
    let _ = platform::call3(FN_800B44D0, pane, 0, 0);
}

/// Conditionally releases a display object.
///
/// # Original
/// - Ghidra: `FUN_8015a470` at `0x8015A470` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a470_FUN_8015a470.c`
///
/// # Arguments
/// * `object` - display object or `0`.
/// * `free` - positive when the caller owns the storage and wants it freed.
///
/// # Returns
/// `object` unchanged, matching the C's `return param_1`.
///
/// # Side effects
/// Calls `0x802B9280(object)` when both guards pass; otherwise no target write.
///
/// # Safety
/// `object` must be null or a live object accepted by `0x802B9280`.
///
/// # Confidence
/// high - the null/free guards and the return are explicit; the class name is
/// unknown, so the function keeps its address name.
pub unsafe fn fn_8015a470(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Releases the embedded sub-object, then optionally the storage.
///
/// # Original
/// - Ghidra: `FUN_8015a4b0` at `0x8015A4B0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a4b0_FUN_8015a4b0.c`
///
/// # Arguments
/// * `object` - display object or `0`.
/// * `free` - positive when the caller owns the storage.
///
/// # Returns
/// `object` unchanged.
///
/// # Side effects
/// Calls `0x8015F9F0(object + 0x18, 0)` and, when `free > 0`, releases
/// `object` through `0x802B9280`.
///
/// # Safety
/// `object` must be null or a live object whose `+0x18` sub-object accepts the
/// destructor call.
///
/// # Confidence
/// high - the nested guard and both calls are explicit; the `+0x18` sub-object
/// type is not named in the export.
pub unsafe fn fn_8015a4b0(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Releases the embedded sub-object, then optionally the storage.
///
/// This is a second class with the same observed destructor shape as
/// [`fn_8015a4b0`]; the two addresses are kept separate because the export
/// does not prove they share a class.
///
/// # Original
/// - Ghidra: `FUN_8015a510` at `0x8015A510` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a510_FUN_8015a510.c`
///
/// # Arguments
/// * `object` - display object or `0`.
/// * `free` - positive when the caller owns the storage.
///
/// # Returns
/// `object` unchanged.
///
/// # Side effects
/// Calls `0x8015F9F0(object + 0x18, 0)` and, when `free > 0`, releases
/// `object` through `0x802B9280`.
///
/// # Safety
/// `object` must be null or a live object whose `+0x18` sub-object accepts the
/// destructor call.
///
/// # Confidence
/// high - identical to [`fn_8015a4b0`] and equally explicit.
pub unsafe fn fn_8015a510(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Sets one display value word and rebuilds its transform.
///
/// # Original
/// - Ghidra: `FUN_80159ac0` at `0x80159AC0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159ac0_FUN_80159ac0.c`
///
/// # Arguments
/// * `this` - display widget; `+0x3E0` is the cached value and `+0x4E0` the
///   animation/transform handle.
/// * `value` - new value word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// If `+0x3E0` already holds `value`, returns without writing. Otherwise stores
/// the value and calls `0x800B3B60(this + 0x3E0, 0x8042DBB8, +0x4E0, 1)`.
///
/// # Safety
/// `this + 0x3E0..+0x4E4` must be writable/readable as described.
///
/// # Confidence
/// medium - the guard, store and call are exact; the SDA2 source word's value
/// is not recovered and is passed as an address.
pub unsafe fn display_set_transform_value(this: usize, value: u32) {
    if read_word(this, 0x3E0) == value {
        return;
    }
    write_word(this, 0x3E0, value);
    let _ = platform::call4(
        FN_800B3B60,
        (this + 0x3E0) as u32,
        W_SDA2_54C8 as u32,
        read_word(this, 0x4E0),
        1,
    );
}

/// Debounces, clamps and stores a large display value.
///
/// # Original
/// - Ghidra: `FUN_80159cb0` at `0x80159CB0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159cb0_FUN_80159cb0.c`
///
/// # Arguments
/// * `this` - display widget; `+0x3E8` is the value, `+0x434` a one-shot
///   debounce counter and `+0x4E4` the animation/transform handle.
/// * `value` - requested value; values above `0x5F5E0CD` are clamped to
///   `0x5F5E0CE` after the debounce.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Returns immediately when the value is unchanged or the stored value already
/// exceeds `0x5F5E0CD`. Otherwise increments `+0x434`; the second consecutive
/// call stores the value, resets the counter and calls
/// `0x800B3B60(this + 0x3E8, 0x8042DBBC, +0x4E4, 1)`.
///
/// # Safety
/// `this + 0x3E8..+0x4E8` and `+0x434` must be writable/readable.
///
/// # Confidence
/// medium - the signed comparisons, counter and clamp are exact; the SDA2
/// source word's value is not recovered.
pub unsafe fn display_set_large_value(this: usize, value: i32) {
    if read_word(this, 0x3E8) as i32 == value {
        return;
    }
    if read_word(this, 0x3E8) as i32 > 0x5F5E0CD {
        return;
    }

    let counter = read_word(this, 0x434) as i32 + 1;
    write_word(this, 0x434, counter as u32);
    if counter < 2 {
        return;
    }

    write_word(this, 0x434, 0);
    write_word(this, 0x3E8, value as u32);
    if value > 0x5F5E0CD {
        write_word(this, 0x3E8, 0x5F5E0CE);
    }

    let _ = platform::call4(
        FN_800B3B60,
        (this + 0x3E8) as u32,
        W_SDA2_54C4 as u32,
        read_word(this, 0x4E4),
        1,
    );
}

/// Installs one `dGameDisplay_c` state descriptor.
///
/// The original repeats the same block four times: read nine descriptor words
/// from ROM, call the descriptor constructor with the state name, write the
/// common vtable at `+0` and the nine words at `+0xC..+0x2C`, then register the
/// descriptor with its destructor and link address.
///
/// # Original
/// Host-side factoring of the four identical blocks in `0x80159F10`
/// (`work/ghidra/main-gb/export/80159f10_FUN_80159f10.c`); no binary address of
/// its own.
///
/// # Arguments
/// * `base` - descriptor address (`0x80377078`, `0x803770B8`, `0x803770F8`,
///   `0x80377138`).
/// * `name` - `s_dGameDisplay_c__StateID_*` string address.
/// * `link` - registration link word passed to `0x802DC780`.
/// * `words` - nine ROM addresses copied to `base + 0xC..+0x2C`.
///
/// # Safety
/// `base` must be writable for `0x30` bytes and each `words` entry readable.
///
/// # Confidence
/// high - the constructor, nine stores and registration call are copied
/// one-for-one from the four blocks.
#[inline]
unsafe fn install_display_state(base: usize, name: u32, link: usize, words: [usize; 9]) {
    // observed: all nine values are loaded before the constructor call.
    let mut values = [0u32; 9];
    for (i, addr) in words.iter().enumerate() {
        values[i] = read_global_word(*addr);
    }

    let _ = platform::call2(FN_8015F7C0, base as u32, name);
    write_global_word(base, VTABLE_DISPLAY_STATE);
    for (i, value) in values.iter().enumerate() {
        write_global_word(base + 0xC + i * 4, *value);
    }
    let _ = platform::call3(
        FN_802DC780,
        base as u32,
        FN_8015A250 as u32,
        link as u32,
    );
}

/// Registers the four `dGameDisplay_c` state descriptors.
///
/// # Original
/// - Ghidra: `FUN_80159f10` at `0x80159F10` (824 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159f10_FUN_80159f10.c`
///
/// # Arguments
/// None. The body starts with `__savegprlr_27` (`0x802DCD74`), which only
/// preserves registers.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Installs four 0x30-byte descriptors:
///
/// | descriptor | state name | registration link |
/// |---|---|---|
/// | `0x80377078` | `0x803278E0` `ProcMain` | `0x80377068` |
/// | `0x803770B8` | `0x80327908` `ProcMain` | `0x803770A8` |
/// | `0x803770F8` | `0x80327930` `ProcGoal` | `0x803770E8` |
/// | `0x80377138` | `0x8032795C` `ProcGoal` | `0x80377128` |
///
/// Each descriptor receives the common vtable `0x80327980` and nine words at
/// `+0xC..+0x2C`. The registration call `0x802DC780` receives the shared
/// destructor `0x8015A250`.
///
/// # Safety
/// The four descriptor ranges must be writable and the nine ROM word addresses
/// per descriptor readable.
///
/// # Confidence
/// high - the four blocks differ only in their literal addresses; every store
/// and both calls are preserved.
pub unsafe fn dgame_display_register_states() {
    // Descriptor 0: dGameDisplay_c::StateID_ProcMain (first variant).
    install_display_state(
        0x8037_7078,
        0x8032_78E0,
        0x8037_7068,
        [
            0x8032_7868,
            0x8032_786C,
            0x8032_7870,
            0x8032_785C,
            0x8032_7860,
            0x8032_7864,
            0x8032_7850,
            0x8032_7854,
            0x8032_7858,
        ],
    );

    // Descriptor 1: dGameDisplay_c::StateID_ProcMain (second variant).
    install_display_state(
        0x8037_70B8,
        0x8032_7908,
        0x8037_70A8,
        [
            0x8032_788C,
            0x8032_7890,
            0x8032_7894,
            0x8032_7880,
            0x8032_7884,
            0x8032_7888,
            0x8032_7874,
            0x8032_7878,
            0x8032_787C,
        ],
    );

    // Descriptor 2: dGameDisplay_c::StateID_ProcGoal (first variant).
    install_display_state(
        0x8037_70F8,
        0x8032_7930,
        0x8037_70E8,
        [
            0x8032_78B0,
            0x8032_78B4,
            0x8032_78B8,
            0x8032_78A4,
            0x8032_78A8,
            0x8032_78AC,
            0x8032_7898,
            0x8032_789C,
            0x8032_78A0,
        ],
    );

    // Descriptor 3: dGameDisplay_c::StateID_ProcGoal (second variant).
    install_display_state(
        0x8037_7138,
        0x8032_795C,
        0x8037_7128,
        [
            0x8032_78D4,
            0x8032_78D8,
            0x8032_78DC,
            0x8032_78C8,
            0x8032_78CC,
            0x8032_78D0,
            0x8032_78BC,
            0x8032_78C0,
            0x8032_78C4,
        ],
    );
}

/// Destructor callback registered by the `Pausewindow_c` state descriptors.
///
/// # Original
/// - Ghidra: `FUN_8015b840` at `0x8015B840` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015b840_FUN_8015b840.c`
///
/// # Arguments
/// * `state` - state object passed by the descriptor manager, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `state` unchanged, matching the C's `return param_1`.
///
/// # Side effects
/// Calls `0x8015F800(state, 0)` and, when `free > 0`, `0x802B9280(state)`.
///
/// # Safety
/// `state` must be null or a live state object accepted by both callees.
///
/// # Confidence
/// high - the two guards and both calls are explicit; the descriptor link is
/// observed in `0x8015B320`.
pub unsafe fn pausewindow_state_release(state: usize, free: i32) -> usize {
    if state != 0 {
        let _ = platform::call2(FN_8015F800, state as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, state as u32);
        }
    }
    state
}

// ===========================================================================
// Wave 16: pause-menu controller (`0x800D0940..0x800D11F0`)
// ===========================================================================
//
// These eight bodies are the per-frame driver of the pause window. They live in
// the engine band `0x800C0000..0x80100000`, but they own the `Pausewindow_c`
// selection words and flags, so they are documented next to the state machine
// they drive instead of in `engine::frame`.
//
// The controller object is the global at `0x80429FD8`. `0x8005F5C0` (outside
// this lane) calls `0x800D0B20(controller, lock)` every frame, so the object
// outlives a single pause: `+0x04` is the menu phase, `+0x08` the selected
// button, `+0x0C` the player index, `+0x18` the flag byte and `+0x1B` the
// "opened at least once" byte. All offsets are observed; the phase numbering
// (`1` opening, `3` closing) is inferred from the call sites.

/// `r13 - 0x56A8` -> `0x80429FD8`: pointer to the pause-menu controller.
const G_PAUSE_CONTROLLER: usize = SDA_BASE - 0x56A8;

/// `r13 - 0x56A0` -> `0x80429FE0`: byte set when a player requests the menu.
const G_PAUSE_REQUEST: usize = SDA_BASE - 0x56A0;

/// `r13 - 0x5658` -> `0x8042A028`: pointer to the five-slot player array.
///
/// Slots `1..4` hold the per-player input objects; slot `0` is a header and is
/// never touched here.
const G_PLAYER_SLOTS: usize = SDA_BASE - 0x5658;

/// `r13 - 0x5730` -> `0x80429F50`: pointer to the array whose slot `player + 1`
/// is the per-player input object read by [`pause_menu_update`].
const G_INPUT_OBJECTS: usize = SDA_BASE - 0x5730;

/// `0x80354E60`: four per-player pause animation ids copied into
/// `Pausewindow_c + 0x28C` by [`pause_menu_begin`].
const TABLE_PAUSE_ANIM: usize = 0x8035_4E60;

/// `0x800B3B50`: returns `*(u32*)0x80429F48 & mask`.
///
/// [`pause_menu_update`] passes `-6` (`0xFFFFFFFA`) to test the flag word for
/// any bit other than bits 0 and 2.
const FN_800B3B50: usize = 0x800B_3B50;

/// `0x800B4DE0`: confirm gate. Returns non-zero when the current course/state
/// allows the `PauseDisp` confirm to act.
const FN_800B4DE0: usize = 0x800B_4DE0;

/// `0x8005F900`: maps a player index to its player object, `0` when empty.
const FN_8005F900: usize = 0x8005_F900;

/// `0x800B53A0`: per-player view query. The decompiled body tail-calls the
/// view object's vtable slot `+0x20` with `0x200`/`0x600`, so its return word is
/// that callee's result. `// inferred:` the word is non-zero while the view is
/// locked by a cutscene.
const FN_800B53A0: usize = 0x800B_53A0;

/// `0x800DC990`: sets byte `+0x55` of one player slot object.
const FN_800DC990: usize = 0x800D_C990;

/// `0x801BE2F0`: engine mode/flag setter; the frame calls it on a flag edge.
const FN_801BE2F0: usize = 0x801B_E2F0;

/// Releases a pause-menu support object when the caller owns the storage.
///
/// # Original
/// - Ghidra: `FUN_800d0940` at `0x800D0940` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0940_FUN_800d0940.c`
///
/// # Arguments
/// * `object` - object to release, or `0`.
/// * `free` - positive when the caller wants the storage freed.
///
/// # Returns
/// `object`, unchanged (`return param_1` in the C).
///
/// # Side effects
/// Calls `0x802B9280(object)` when both guards pass.
///
/// # Safety
/// `object` must be null or a live object accepted by `0x802B9280`.
///
/// # Confidence
/// medium - the guards and the return are explicit; the export prints the
/// release call without operands, so its argument is reconstructed from the
/// guard (`// partial:` operand).
pub unsafe fn pause_menu_release(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// `r13 - 0x569F` = `0x80429FE1`: byte cleared together with the controller
/// pointer by [`fn_800d09d0`]. It sits between the request byte
/// ([`G_PAUSE_REQUEST`], `-0x56A0`) and the 1-up suppressor (`-0x569E`).
const E274_G_PAUSE_ACTIVE: usize = SDA_BASE - 0x569F;

/// Destroys the pause-menu controller singleton.
///
/// The element constructor `0x800D09A0` publishes itself at
/// [`G_PAUSE_CONTROLLER`] and installs the `0x80317A60` vtable, whose `+8` slot
/// is this body. A non-null `this` clears the published pointer and the
/// `E274_G_PAUSE_ACTIVE` byte, then - only when `release` is positive - frees
/// the storage through `0x802B9280`. `this` is returned unchanged.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800D09D0`.
/// - Extent: `0x800D09D0..0x800D0A1C` (76 bytes, 18 instructions), ending at the
///   `blr` at `0x800D0A18`; three zero words precede it at
///   `0x800D09C4..0x800D09D0` and two follow before the next body.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: the only data word naming it is `0x80317A68`, slot `+0x08` of the
///   `0x80317A60` vtable installed by the `0x800D09A0` element constructor; it
///   is not inside a pointer run.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0xC(r1)`; `mr r31,r3`;
///   `bc 12,2` (`beq`) to the epilogue; `cmpi cr0,0,r4,0`; `li r0,0`;
///   `stw r0,-0x56A8(r13)`; `stb r0,-0x569F(r13)`;
///   `bc 4,1` (`ble`, skip the free); `bl 0x802B9280`; `mr r3,r31`;
///   epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: controller singleton, or `0`.
/// - `release`: signed count; only positive values free `this`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Clears the singleton pointer and the active byte; frees `this` on the
/// guarded path.
///
/// # Safety
/// A non-null `this` must be the live controller published by `0x800D09A0`,
/// and a positive `release` hands it to `0x802B9280`.
///
/// # Confidence
/// high - the guards, both SDA stores, the free call and the returned receiver
/// are literal.
pub unsafe fn fn_800d09d0(this: usize, release: i32) -> usize {
    if this != 0 {
        platform::write_u32(G_PAUSE_CONTROLLER, 0);
        platform::write_u8(E274_G_PAUSE_ACTIVE, 0);
        if release > 0 {
            let _ = platform::call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Sets or clears the pause-menu lock bit.
///
/// # Original
/// - Ghidra: `FUN_800d0b20` at `0x800D0B20` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0b20_FUN_800d0b20.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller.
/// * `locked` - `0` sets bit 1 of `+0x18`, any non-zero value clears it. The
///   inverted sense is the original's: the caller passes the "input allowed"
///   word, so a zero there means "locked".
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One read-modify-write of the byte at `controller + 0x18`.
///
/// # Safety
/// `controller + 0x18` must be writable.
///
/// # Confidence
/// high - a single read-modify-write with explicit masks.
pub unsafe fn pause_menu_set_lock(controller: usize, locked: i32) {
    let mut flags = read_byte(controller, 0x18);
    if locked == 0 {
        flags |= 2;
    } else {
        flags &= 0xFD;
    }
    write_byte(controller, 0x18, flags);
}

/// Returns the pause-menu lock bit.
///
/// # Original
/// - Ghidra: `FUN_800d0b50` at `0x800D0B50` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0b50_FUN_800d0b50.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller.
///
/// # Returns
/// Bit 1 of `controller + 0x18` (`0` or `1`).
///
/// # Side effects
/// One byte read.
///
/// # Safety
/// `controller + 0x18` must be readable.
///
/// # Confidence
/// high - the whole body is `byte >> 1 & 1`.
pub unsafe fn pause_menu_lock(controller: usize) -> u8 {
    (read_byte(controller, 0x18) >> 1) & 1
}

/// Starts the pause window from the controller's current player index.
///
/// # Original
/// - Ghidra: `FUN_800d0b60` at `0x800D0B60` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0b60_FUN_800d0b60.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller whose `+0x1B` byte records that
///   the menu was opened before.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Clears `+0x1A` unconditionally.
/// * When `+0x1B` is set: phase `+0x04 = 1`, sets flag bit 0, clears `+0x08`,
///   `+0x10` and `+0x19`, marks the pause window as opening (`+0x291 = 1`),
///   copies `TABLE_PAUSE_ANIM[+0x0C]` into `Pausewindow_c + 0x28C` and sets the
///   display object's `+0x438` word.
///
/// # Safety
/// The `Pausewindow_c` global and the optional display object must be live when
/// `+0x1B` is set. The original performs **no** null check on the display
/// object; that omission is preserved.
///
/// # Confidence
/// high - every store and the single table index are explicit.
pub unsafe fn pause_menu_begin(controller: usize) {
    write_byte(controller, 0x1A, 0);
    if read_byte(controller, 0x1B) != 0 {
        let display = globals::fn_80101960() as usize;
        write_word(controller, 0x04, 1);
        write_byte(controller, 0x18, read_byte(controller, 0x18) | 1);
        write_word(controller, 0x08, 0);
        write_word(controller, 0x10, 0);
        write_byte(controller, 0x19, 0);

        let pausewindow = read_global_word(G_PAUSEWINDOW) as usize;
        write_byte(pausewindow, 0x291, 1);
        let player = read_word(controller, 0x0C) as usize;
        write_word(
            pausewindow,
            0x28C,
            read_global_word(TABLE_PAUSE_ANIM + player * 4),
        );
        write_word(display, 0x438, 1);
    }
}

/// Clamps the requested pause button and stores it in the controller and the
/// pause window.
///
/// # Original
/// - Ghidra: `FUN_800d0c00` at `0x800D0C00` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0c00_FUN_800d0c00.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller.
/// * `index` - requested button index; clamped to `0..1`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// When the clamped index differs from `+0x08`, the move is allowed by the
/// confirm gate (`0x800B4DE0`), and `DAT_80315B9D` is outside `0x20..0x22`,
/// writes the index to `Pausewindow_c + 0x284` and `controller + 0x08`.
///
/// # Safety
/// `controller + 0x08` and the `Pausewindow_c` global must be live.
///
/// # Confidence
/// high - the clamps, both guards and both stores are explicit.
pub unsafe fn pause_menu_select_button(controller: usize, index: i32) {
    let mut index = index;
    if index < 0 {
        index = 0;
    }
    if index > 1 {
        index = 1;
    }

    let state = read_global_byte(DAT_80315B9D);
    if read_word(controller, 0x08) as i32 != index
        && (index != 1 || call0_r(FN_800B4DE0) != 0 || state == 0x23)
        && state.wrapping_sub(0x20) > 2
    {
        let pausewindow = read_global_word(G_PAUSEWINDOW) as usize;
        write_word(pausewindow, 0x284, index as u32);
        write_word(controller, 0x08, index as u32);
    }
}

/// Per-frame pause input: move the selection, confirm, or cancel.
///
/// # Original
/// - Ghidra: `FUN_800d0cb0` at `0x800D0CB0` (404 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0cb0_FUN_800d0cb0.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller; `+0x0C` selects the player and
///   `+0x08` the current button.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Returns early while `0x800B3B50(-6)` is non-zero, when the player object
///   is missing, or when the pause window's `+0x294` is set.
/// * Left/right on the input object at `input_objects[player + 1] + 0x1C`
///   moves the selection by one through [`pause_menu_select_button`].
/// * The confirm bit (`0x10`) or a locked view ends the menu via
///   [`pause_menu_end`]; the cancel bit (`0x02`) raises `Pausewindow_c + 0x292`
///   and plays sound `0x7A`; the confirm path raises `+0x293`, sets `+0x19`
///   and plays sound `0x79`.
///
/// # Safety
/// The controller, the player/input arrays and the view objects at `0x80377C88`
/// must be live. The original does not bounds-check the player index.
///
/// # Confidence
/// medium - the branch tree and every store are explicit; the return word of
/// `0x800B53A0` is only visible as a tail call, so it is read as the callee's
/// `r3` (`// inferred:`).
pub unsafe fn pause_menu_update(controller: usize) {
    if platform::call1(FN_800B3B50, 0xFFFF_FFFA) != 0 {
        return;
    }

    let player = read_word(controller, 0x0C);
    let selected = read_word(controller, 0x08) as i32;
    let player_object = platform::call1(FN_8005F900, player);
    let pausewindow = read_global_word(G_PAUSEWINDOW) as usize;
    if player_object == 0 || pausewindow == 0 || read_byte(pausewindow, 0x294) != 0 {
        return;
    }

    // observed: the four view objects are the words at 0x80377C88..0x80377C94.
    let view = read_global_word(0x8037_7C88 + player as usize * 4) as usize;
    let view_answer = vcall1(view, 0x20, 0x900);
    let view_locked = platform::call1(FN_800B53A0, player);

    let input_objects = read_global_word(G_INPUT_OBJECTS) as usize;
    let input = read_word(input_objects, (player as usize + 1) * 4) as usize;
    let pad = read_word(input, 0x1C);

    if (pad & 0x10) == 0 && view_locked == 0 && view_answer == 0 {
        if (pad & 1) == 0 {
            if (pad & 2) != 0 {
                pause_menu_select_button(controller, selected - 1);
            }
        } else {
            pause_menu_select_button(controller, selected + 1);
        }
    } else {
        if (pad & 0x10) != 0 || view_locked != 0 {
            pause_menu_end(controller);
        }
        if view_answer == 0 {
            // observed: sound 0x7A is the "cancel" UI sound.
            let _ = platform::call3(FN_80195380, read_global_word(G_SOUND_SYSTEM), 0x7A, 1);
            write_byte(pausewindow, 0x292, 1);
        } else if read_word(controller, 0x08) != 1
            || call0_r(FN_800B4DE0) != 0
            || read_global_byte(DAT_80315B9D) == 0x23
        {
            pause_menu_end(controller);
            write_byte(pausewindow, 0x293, 1);
            write_byte(controller, 0x19, 1);
            let _ = platform::call3(FN_80195380, read_global_word(G_SOUND_SYSTEM), 0x79, 1);
        }
    }
}

/// Leaves the pause menu and marks every player slot as leaving.
///
/// # Original
/// - Ghidra: `FUN_800d0e90` at `0x800D0E90` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0e90_FUN_800d0e90.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets display `+0x440` when the display object exists, writes phase `3` to
/// the controller, clears `+0x1B` and flag bit 2, then calls `0x800DC990` on
/// the `+0x55` byte of all four player slots.
///
/// # Safety
/// The player-slot array at `0x8042A028` must be live for all four entries.
///
/// # Confidence
/// high - one guarded write, three controller writes and a fixed four-iteration
/// loop are all explicit.
pub unsafe fn pause_menu_end(controller: usize) {
    let display = globals::fn_80101960() as usize;
    if display != 0 {
        write_word(display, 0x440, 1);
    }
    write_word(controller, 0x04, 3);
    write_byte(controller, 0x1B, 0);
    write_byte(controller, 0x18, read_byte(controller, 0x18) & 0xFB);

    let slots = read_global_word(G_PLAYER_SLOTS) as usize;
    let mut index = 0usize;
    while index < 4 {
        let slot = read_word(slots, (index + 1) * 4) as usize;
        let _ = platform::call1(FN_800DC990, slot as u32);
        index += 1;
    }
}

/// Opens the pause menu for one player and raises the global request byte.
///
/// # Original
/// - Ghidra: `FUN_800d11f0` at `0x800D11F0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d11f0_FUN_800d11f0.c`
///
/// # Arguments
/// * `controller` - live pause-menu controller.
/// * `player` - player index stored at `+0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8005F900(player)` and discards the result, sets `+0x1B` and the
/// global request byte, stores the player index, plays sound `0x7C` and sets
/// the engine flag through `0x801BE2F0(1)`.
///
/// # Safety
/// The sound system global and the controller must be live.
///
/// # Confidence
/// high - the dropped return value and all five calls are visible.
pub unsafe fn pause_menu_request_player(controller: usize, player: u32) {
    // observed: the player-object lookup result is discarded in the original.
    let _ = platform::call1(FN_8005F900, player);
    write_byte(controller, 0x1B, 1);
    write_global_byte(G_PAUSE_REQUEST, 1);
    write_word(controller, 0x0C, player);
    let _ = platform::call3(FN_80195380, read_global_word(G_SOUND_SYSTEM), 0x7C, 1);
    let _ = platform::call1(FN_801BE2F0, 1);
}

// ===========================================================================
// Wave 16: gameScene world-map HUD (`0x801568D0..0x8015A250`)
// ===========================================================================
//
// The `gameScene/gameScene.arc` + `gameScene_37.brlyt` layout draws the
// world-map collection/assist panes: `N_otasukeInfo_00` (0x14 panes),
// `P_collectOff_00` (10 panes), `T_left_00` (0xB panes) and `T_otaChuS_00`
// (4 panes). The object keeps a 4-entry record for each pane group, three
// counter values and three hit rectangles.
//
// The `0x802DCDxx`/`0x802DCD5C`/`0x802DCDA8` calls printed by Ghidra around
// these bodies are CodeWarrior `__savegprlr_*` / `__restgprlr_*` prologue
// helpers, not game logic (see `docs/COVERAGE_DISPLAY.md`). They are omitted;
// the `r3` value they preserve is written as the explicit `hud` argument.

/// `r2 - 0x54BC` -> `0x8042DBC4`: first horizontal offset constant.
const F32_SDA2_54BC: usize = SDA2_BASE - 0x54BC;

/// `r2 - 0x54B8` -> `0x8042DBC8`: vertical offset constant.
const F32_SDA2_54B8: usize = SDA2_BASE - 0x54B8;

/// `r2 - 0x54B4` -> `0x8042DBCC`: diagonal scale applied to a tilted pane edge.
const F32_SDA2_54B4: usize = SDA2_BASE - 0x54B4;

/// `r2 - 0x54B0` -> `0x8042DBD0`: extra offset added to the selected pane.
const F32_SDA2_54B0: usize = SDA2_BASE - 0x54B0;

/// `0x80327670`: `"gameScene/gameScene.arc"` archive name.
const S_GAMESCENE_ARC: usize = 0x8032_7670;

/// `0x80327688`: `"gameScene_37.brlyt"` layout name.
const S_GAMESCENE_LAYOUT: usize = 0x8032_7688;

/// `0x80327498`: `N_otasukeInfo_00` pane-name table (0x14 entries).
const PANE_NAMES_OTASUKE_INFO: usize = 0x8032_7498;

/// `0x80327580`: `P_collectOff_00` pane-name table (10 entries).
const PANE_NAMES_COLLECT_OFF: usize = 0x8032_7580;

/// `0x803275FC`: `T_left_00` pane-name table (0xB entries).
const PANE_NAMES_T_LEFT: usize = 0x8032_75FC;

/// `0x80327660`: `T_otaChuS_00` pane-name table (4 entries).
const PANE_NAMES_T_OTA_CHU_S: usize = 0x8032_7660;

/// `0x803276CC`: `"Wm_2d_starcoinget"` effect name.
const S_WM_2D_STARCOIN_GET: usize = 0x8032_76CC;

/// `0x802F5948`: animation table passed to `0x800C8F20` while mounting.
const TABLE_802F5948: usize = 0x802F_5948;

/// `0x802F5958`: eight source-pane indices, two per group and per player.
const TABLE_802F5958: usize = 0x802F_5958;

/// `0x802F5978`: three `P_collectOff_00` pane indices used when hiding.
const TABLE_802F5978: usize = 0x802F_5978;

/// `0x802F59A8`: three `P_collectOff_00` pane indices used when showing.
const TABLE_802F59A8: usize = 0x802F_59A8;

/// `0x802F5988`: eight `T_left_00`/`T_otaChuS_00` pane indices.
const TABLE_802F5988: usize = 0x802F_5988;

/// `0x802F5928`: four per-player controller ids.
const TABLE_802F5928: usize = 0x802F_5928;

/// `0x802F5938`: four `P_collectOff_00` pane indices for active players.
const TABLE_802F5938: usize = 0x802F_5938;

/// `0x80354E50`: four player-present bytes indexed by controller id.
const TABLE_80354E50: usize = 0x8035_4E50;

/// `0x803741B0`: three star-coin pane state words (0, 2 or 4).
const TABLE_803741B0: usize = 0x8037_41B0;

/// `0x80377138`: `dGameDisplay_c::StateID_ProcGoal` (second variant), one of the
/// four descriptors installed by [`dgame_display_register_states`].
const DISPLAY_STATE_PROC_GOAL_B: u32 = 0x8037_7138;

/// `r13 - 0x547C` -> `0x8042A204`: HUD layout mode; `4` runs the pane-position
/// pass in [`gamescene_hud_position_panes`].
const G_HUD_MODE: usize = SDA_BASE - 0x547C;

/// `r13 - 0x5610` -> `0x8042A070`: object whose `+4` word is the packed counter
/// and whose `+0xC` byte mirrors the active mode.
const G_COUNTER_OBJECT: usize = SDA_BASE - 0x5610;

/// `r13 - 0x59C0` -> `0x80429CC0`: the large counter value faded by
/// [`gamescene_hud_fade_decrement`].
const G_LARGE_COUNTER: usize = SDA_BASE - 0x59C0;

/// `r13 - 0x59D8` -> `0x80429CA8`: per-player present bitmask.
const G_PLAYER_MASK: usize = SDA_BASE - 0x59D8;

/// `r13 - 0x5354` -> `0x8042A32C`: byte saying the shared pane colour is
/// initialised.
const G_PANE_COLOUR_INIT: usize = SDA_BASE - 0x5354;

/// `0x8042A32C..0x8042A32F`: shared pane colour bytes (r, g, b, a).
const G_PANE_COLOUR: usize = SDA_BASE - 0x5354;

/// `r13 - 0x5938` -> `0x80429D48`: object whose `+0xBCB` byte gates the close
/// request in [`gamescene_hud_request_close`].
const G_HUD_GATE_5938: usize = SDA_BASE - 0x5938;

/// `r13 - 0x5640` -> `0x8042A040`: object whose record accessor
/// [`save_image_record_a`] reads the current course/map id from.
const G_COURSE_5640: usize = SDA_BASE - 0x5640;

/// `r13 - 0x54B8` -> `0x8042A1C8`: object whose `+0x120C/+0x120D` bytes are the
/// current grid position.
const G_COURSE_54B8: usize = SDA_BASE - 0x54B8;

/// `0x804293B0`: first layout data word passed to `0x800C8FB0`.
///
/// `// unknown:` the export only shows the address; the callee's role is
/// inferred as "install a layout animation table".
const W_804293B0: usize = 0x8042_93B0;

/// `0x804293B4`: second layout data word passed to `0x800C90F0`.
const W_804293B4: usize = 0x8042_93B4;

/// `r2 - 0x3478` -> `0x8042FC08`: table passed to `0x800C90F0`.
const W_SDA2_3478: usize = SDA2_BASE - 0x3478;

/// `0x8005FC70`: returns the four-bit connected-player mask.
const FN_8005FC70: usize = 0x8005_FC70;

/// `0x80060110`: maps a controller id to the player-slot index.
const FN_80060110: usize = 0x8006_0110;

/// `0x800C8CB0`: creates/mounts a layout archive `(layout, name, 0)`.
const FN_800C8CB0: usize = 0x800C_8CB0;

/// `0x800C8D60`: binds a pane-name table `(layout, names, out, count)`.
const FN_800C8D60: usize = 0x800C_8D60;

/// `0x800C8E40`: binds a pane-name table `(layout, names, out, count)`.
const FN_800C8E40: usize = 0x800C_8E40;

/// `0x800C8EB0`: binds a pane-name table `(layout, names, out, count)`.
const FN_800C8EB0: usize = 0x800C_8EB0;

/// `0x800C8F20`: binds the four-pane animation group.
const FN_800C8F20: usize = 0x800C_8F20;

/// `0x800C8FB0`: installs one layout data word `(layout, data, 1)`.
const FN_800C8FB0: usize = 0x800C_8FB0;

/// `0x800C90F0`: installs a data/table pair `(layout, data, table, 1)`.
const FN_800C90F0: usize = 0x800C_90F0;

/// `0x80007220`: returns the layout's root pane.
const FN_80007220: usize = 0x8000_7220;

/// `0x800E3910`: writes the packed counter word on the counter object.
const FN_800E3910: usize = 0x800E_3910;

/// `0x801953A0`: sound-system variant that does not take the extra flag.
const FN_801953A0: usize = 0x8019_53A0;

/// `0x80060690`: player-object counter display `(value, -1)`.
const FN_80060690: usize = 0x8006_0690;

/// `0x8006CED0`: reads a player object's position `(out, player)`.
const FN_8006CED0: usize = 0x8006_CED0;

/// `0x800B33D0`: transforms a position in place.
const FN_800B33D0: usize = 0x800B_33D0;

/// `0x800CE210`: queries one grid cell `(course, x, y, index)`.
const FN_800CE210: usize = 0x800C_E210;

/// `0x800E0380`: returns the current course/map record.
///
/// The body is translated in this file as [`save_image_record_a`]; the HUD code
/// above keeps the call-by-address form so its call sequence stays literal.
const FN_800E0380: usize = 0x800E_0380;

/// Sets or clears bit 0 of the pane flags byte at `pane + 0xBB`.
///
/// Wave 16 uses the same pane visibility bit as [`pane_set_flag0`]; this alias
/// documents the call sites that pass a raw word from the HUD object.
#[inline]
unsafe fn hud_set_pane_visible(pane: u32, visible: bool) {
    pane_set_flag0(pane, visible);
}

/// Saves one pane's position into the HUD record and applies the shared colour.
///
/// # Original
/// Host-side factoring of the three identical blocks in `0x80159250`
/// (`work/ghidra/main-gb/export/80159250_FUN_80159250.c`); no binary address.
///
/// # Arguments
/// * `hud` - the `gameScene` HUD object.
/// * `player` - player index; the record is at `hud + player * 8 + save_off`.
/// * `save_off` - `0x330`, `0x350` or `0x370`, one per pane group.
/// * `pane` - the pane whose record is copied.
///
/// # Safety
/// `hud + player * 8 + save_off` must be an 8-byte writable slot and the layout
/// vtable slot `+0x68` must return a writable 0x20-byte record.
///
/// # Confidence
/// medium - the stores are literal; the meaning of the `+0x68` result is
/// inferred from the save/restore pair.
#[inline]
unsafe fn hud_save_pane_and_colour(hud: usize, player: u32, save_off: usize, pane: usize) {
    let draw = vcall0(pane, 0x68) as usize;
    let x = read_word(draw, 0x18);
    let y = read_word(draw, 0x1C);
    let base = hud + player as usize * 8;
    write_half(base, save_off, (x >> 16) as u16);
    write_half(base, save_off + 2, x as u16);
    write_half(base, save_off + 4, (y >> 16) as u16);
    write_half(base, save_off + 6, y as u16);

    let r = read_global_byte(G_PANE_COLOUR) as u16;
    let g = read_global_byte(G_PANE_COLOUR + 1) as u16;
    let b = read_global_byte(G_PANE_COLOUR + 2) as u16;
    let a = read_global_byte(G_PANE_COLOUR + 3) as u16;
    write_half(draw, 0x18, r);
    write_half(draw, 0x1A, g);
    write_half(draw, 0x1C, b);
    write_half(draw, 0x1E, a);
}

/// Restores one pane's position from the HUD record.
///
/// # Original
/// Host-side factoring of the three identical blocks in `0x801594E0`
/// (`work/ghidra/main-gb/export/801594e0_FUN_801594e0.c`); no binary address.
///
/// # Safety
/// `hud + player * 8 + save_off` must be readable and the `+0x68` result
/// writable for four halfwords.
///
/// # Confidence
/// medium - the stores are literal; the `+0x68` result layout is inferred.
#[inline]
unsafe fn hud_restore_pane(hud: usize, player: u32, save_off: usize, pane: usize) {
    let draw = vcall0(pane, 0x68) as usize;
    let base = hud + player as usize * 8;
    write_half(draw, 0x18, read_half(base, save_off));
    write_half(draw, 0x1A, read_half(base, save_off + 2));
    write_half(draw, 0x1C, read_half(base, save_off + 4));
    write_half(draw, 0x1E, read_half(base, save_off + 6));
}

/// Counts the connected player slots.
///
/// # Original
/// - Ghidra: `FUN_801568d0` at `0x801568D0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801568d0_FUN_801568d0.c`
///
/// # Arguments
/// None; the mask comes from `0x8005FC70`.
///
/// # Returns
/// The number of set bits among bits `0..3` of the mask, as a signed byte.
///
/// # Side effects
/// None; one call and four bit tests.
///
/// # Safety
/// `0x8005FC70` must be callable on the target.
///
/// # Confidence
/// high - the four independent bit tests are literal.
pub unsafe fn player_slot_count() -> i8 {
    let mask = call0_r(FN_8005FC70);
    let mut count: i8 = if (mask & 1) != 0 { 1 } else { 0 };
    if (mask & 2) != 0 {
        count = count.wrapping_add(1);
    }
    if (mask & 4) != 0 {
        count = count.wrapping_add(1);
    }
    if (mask & 8) != 0 {
        count = count.wrapping_add(1);
    }
    count
}

/// Writes the eight HUD pane positions from the layout's offset words.
///
/// # Original
/// - Ghidra: `FUN_80158100` at `0x80158100` (420 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158100_FUN_80158100.c`
///
/// # Arguments
/// * `hud` - live `gameScene` HUD object.
///
/// # Returns
/// Nothing; the whole body is gated on `0x8042A204 == 4`.
///
/// # Side effects
/// For the eight panes at `+0x524`, `+0x528`, `+0x4EC`, `+0x500`, `+0x4E8`,
/// `+0x504`, `+0x508` and `+0x520`, writes `x/y/z` at the pane's `+0x2C`,
/// `+0x30`, `+0x34` from the offset words `+0x544..+0x5A0`, then calls the
/// layout vtable slot `+0x10`.
///
/// # Safety
/// The HUD mode global must be `4` and all eight pane handles must be live.
///
/// # Confidence
/// medium - every load and store is explicit; the four SDA2 float constants are
/// read from their addresses and their values are not recovered.
pub unsafe fn gamescene_hud_position_panes(hud: usize) {
    if read_global_word(G_HUD_MODE) != 4 {
        return;
    }

    let base = read_const_f32(F32_SDA2_54C0);
    let spread = read_const_f32(F32_SDA2_54BC);
    let span = read_const_f32(F32_SDA2_54B8);

    let mut f3 = read_float(hud, 0x548);
    let mut f4 = read_float(hud, 0x54C);
    let pane = read_word(hud, 0x524) as usize;
    write_float(pane, 0x2C, base + spread + read_float(hud, 0x544));
    let inner = base - span;
    write_float(pane, 0x30, base + f3);
    write_float(pane, 0x34, base + f4);

    f3 = read_float(hud, 0x554);
    f4 = read_float(hud, 0x558);
    let pane = read_word(hud, 0x528) as usize;
    write_float(pane, 0x2C, (base - spread) + read_float(hud, 0x550));
    write_float(pane, 0x30, base + f3);
    write_float(pane, 0x34, base + f4);

    let mut f2 = read_float(hud, 0x560);
    f3 = read_float(hud, 0x564);
    let pane = read_word(hud, 0x4EC) as usize;
    write_float(pane, 0x2C, base + read_float(hud, 0x55C));
    write_float(pane, 0x30, inner + f2);
    write_float(pane, 0x34, base + f3);

    f2 = read_float(hud, 0x56C);
    f3 = read_float(hud, 0x570);
    let pane = read_word(hud, 0x500) as usize;
    write_float(pane, 0x2C, base + read_float(hud, 0x568));
    write_float(pane, 0x30, inner + f2);
    write_float(pane, 0x34, base + f3);

    f2 = read_float(hud, 0x578);
    f3 = read_float(hud, 0x57C);
    let pane = read_word(hud, 0x4E8) as usize;
    write_float(pane, 0x2C, base + read_float(hud, 0x574));
    write_float(pane, 0x30, base + span + f2);
    write_float(pane, 0x34, base + f3);

    f2 = read_float(hud, 0x584);
    f3 = read_float(hud, 0x588);
    let pane = read_word(hud, 0x504) as usize;
    write_float(pane, 0x2C, base + read_float(hud, 0x580));
    write_float(pane, 0x30, inner + f2);
    write_float(pane, 0x34, base + f3);

    f2 = read_float(hud, 0x590);
    f3 = read_float(hud, 0x594);
    let pane = read_word(hud, 0x508) as usize;
    write_float(pane, 0x2C, base + read_float(hud, 0x58C));
    write_float(pane, 0x30, inner + f2);
    write_float(pane, 0x34, base + f3);

    f2 = read_float(hud, 0x59C);
    f3 = read_float(hud, 0x5A0);
    let pane = read_word(hud, 0x520) as usize;
    write_float(pane, 0x2C, base + read_float(hud, 0x598));
    write_float(pane, 0x30, inner + f2);
    write_float(pane, 0x34, base + f3);

    let _ = vcall_layout0(hud, 0x10);
}

/// Tilts one pane's hit rectangle from its corner flag.
///
/// # Original
/// - Ghidra: `FUN_801582b0` at `0x801582B0` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801582b0_FUN_801582b0.c`
///
/// # Arguments
/// * `hud` - live HUD object.
/// * `dst` - destination rectangle `0..2` (record at `hud + dst * 0x10`).
/// * `src` - source pane `6..8` (record at `hud + src * 4 + 0x4E8`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies the pane's `+0x90`/`+0xA0` words into the rectangle edges, then
/// expands one horizontal and one vertical edge according to
/// `byte[pane + 0xBA] % 3` and `/ 3`, using the pane's `+0x4C`/`+0x50` size.
///
/// # Safety
/// Both records and the pane must be live.
///
/// # Confidence
/// medium - every arithmetic operation is literal; the corner-byte encoding is
/// inferred from the `% 3` / `/ 3` split and the SDA2 scale is read, not guessed.
pub unsafe fn gamescene_hud_offset_pane(hud: usize, dst: u32, src: u32) {
    let dst_slot = hud + dst as usize * 0x10;
    let src_slot = hud + src as usize * 4;

    let pane = read_word(src_slot, 0x4E8) as usize;
    let x = read_word(pane, 0x90);
    let y = read_word(pane, 0xA0);
    write_word(dst_slot, 0x460, x);
    write_word(dst_slot, 0x464, x);
    write_word(dst_slot, 0x458, y);
    write_word(dst_slot, 0x45C, y);

    let pane = read_word(src_slot, 0x4E8) as usize;
    let width = read_float(pane, 0x4C);
    let height = read_float(pane, 0x50);
    let corner = read_byte(pane, 0xBA);

    match corner % 3 {
        0 => write_float(dst_slot, 0x464, read_float(dst_slot, 0x464) + width),
        1 => {
            let scaled = width * read_const_f32(F32_SDA2_54B4);
            write_float(dst_slot, 0x460, read_float(dst_slot, 0x460) - scaled);
            write_float(dst_slot, 0x464, read_float(dst_slot, 0x464) + scaled);
        }
        2 => write_float(dst_slot, 0x460, read_float(dst_slot, 0x460) - width),
        _ => {}
    }

    match corner / 3 {
        0 => write_float(dst_slot, 0x45C, read_float(dst_slot, 0x45C) - height),
        1 => {
            let scaled = height * read_const_f32(F32_SDA2_54B4);
            write_float(dst_slot, 0x458, read_float(dst_slot, 0x458) + scaled);
            write_float(dst_slot, 0x45C, read_float(dst_slot, 0x45C) - scaled);
        }
        2 => write_float(dst_slot, 0x458, read_float(dst_slot, 0x458) + height),
        _ => {}
    }
}

/// Updates the three tilts and the selected pane's edge.
///
/// # Original
/// - Ghidra: `FUN_80158480` at `0x80158480` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158480_FUN_80158480.c`
///
/// # Arguments
/// * `hud` - live HUD object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`gamescene_hud_offset_pane`] for `(0,6)`, `(1,7)`, `(2,8)`. When the
/// selected index `+0x414` is not `6`, writes the selected pane's right edge to
/// `hud + 0x464`: `pane[0x90] + scale * pane[0x4C] + extra`.
///
/// # Safety
/// The selected pane handle and its `+0x4C`/`+0x90` fields must be live.
///
/// # Confidence
/// medium - the loop and the selected-pane store are explicit; the two SDA2
/// constants are read from their addresses.
pub unsafe fn gamescene_hud_offset_panes(hud: usize) {
    let mut index: u32 = 0;
    while index < 3 {
        gamescene_hud_offset_pane(hud, index, index + 6);
        index += 1;
    }

    let selected = read_word(hud, 0x414) as usize;
    if selected != 6 {
        let pane = read_word(hud, selected * 4 + 0x494) as usize;
        write_float(
            hud,
            0x464,
            read_float(pane, 0x90)
                + read_const_f32(F32_SDA2_54B4) * read_float(pane, 0x4C)
                + read_const_f32(F32_SDA2_54B0),
        );
    }
}

/// Restores the pane positions of all active players.
///
/// # Original
/// - Ghidra: `FUN_80158580` at `0x80158580` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158580_FUN_80158580.c`
///
/// # Arguments
/// * `hud` - live HUD object. Ghidra prints `void` because the body starts with
///   the `0x802DCD70` register-save helper; `r3` is the incoming `hud`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Saves the `+0x2C..+0x34` position of the panes at `+0x4AC`, `+0x4B0`,
/// `+0x4B4`, `+0x4B8` and hides them, then for each of the four controller ids
/// in `0x802F5928` whose player byte at `0x80354E50` is non-zero, shows the
/// pane at `0x802F5938[i]` and writes the next saved position into it, also
/// writing `+0x414 = i + 6`.
///
/// # Safety
/// The four panes, the controller table and the pane-index table must be live.
///
/// # Confidence
/// high - the loop and both tables are explicit; only the register-save helper
/// is omitted.
pub unsafe fn gamescene_hud_restore_panes(hud: usize) {
    let mut saved = [0u32; 12];
    let slots = [0x4ACusize, 0x4B0, 0x4B4, 0x4B8];
    let mut record = 0usize;
    for slot in slots {
        let pane = read_word(hud, slot) as usize;
        saved[record * 3] = read_word(pane, 0x2C);
        saved[record * 3 + 1] = read_word(pane, 0x30);
        saved[record * 3 + 2] = read_word(pane, 0x34);
        write_byte(pane, 0xBB, read_byte(pane, 0xBB) & 0xFE);
        record += 1;
    }

    // observed: the saved pointer only advances for active players, so an
    // inactive slot reuses the previous record. The original does the same.
    let mut saved_index = 0usize;
    let mut index: u32 = 0;
    while index < 4 {
        let controller = read_global_word(TABLE_802F5928 + index as usize * 4);
        let player = platform::call1(FN_80060110, controller) as usize;
        if read_global_byte(TABLE_80354E50 + player) != 0 {
            let pane_index = read_global_word(TABLE_802F5938 + index as usize * 4) as usize;
            let pane = read_word(hud, pane_index * 4 + 0x494) as usize;
            write_byte(pane, 0xBB, (read_byte(pane, 0xBB) & 0xFE) | 1);
            let pane = read_word(hud, pane_index * 4 + 0x494) as usize;
            write_word(pane, 0x2C, saved[saved_index * 3]);
            write_word(pane, 0x30, saved[saved_index * 3 + 1]);
            write_word(pane, 0x34, saved[saved_index * 3 + 2]);
            saved_index += 1;
            write_word(hud, 0x414, index + 6);
        }
        index += 1;
    }
}

/// Mounts the `gameScene` archive and binds all four pane groups.
///
/// # Original
/// - Ghidra: `FUN_801586f0` at `0x801586F0` (412 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801586f0_FUN_801586f0.c`
///
/// # Arguments
/// * `hud` - uninitialised or reusable HUD object.
///
/// # Returns
/// `true` when `0x800C8CB0` reported that the archive was mounted; `false`
/// otherwise. The bindings run only in the `true` case.
///
/// # Side effects
/// Mounts `"gameScene/gameScene.arc"`, opens `"gameScene_37.brlyt"` through the
/// layout vtable `+0x14`, installs the two data words, stores the root pane at
/// `+0x490`, binds the four pane-name tables into `+0x4E8`, `+0x494`, `+0x4BC`
/// and `0x802F5948`, sets the initial visibility of five panes, and installs the
/// final animation group.
///
/// # Safety
/// The layout object at `hud + 0x70` must accept the NW4R calls and the pane
/// arrays must be writable.
///
/// # Confidence
/// high - all mounts, binds, visibility writes and the return test are literal.
pub unsafe fn gamescene_hud_mount(hud: usize) -> bool {
    let mounted = platform::call3(
        FN_800C8CB0,
        (hud + 0x70) as u32,
        S_GAMESCENE_ARC as u32,
        0,
    );
    if mounted != 0 {
        let _ = vcall_layout2(hud, 0x14, S_GAMESCENE_LAYOUT as u32, 0);
        let _ = platform::call3(FN_800C8FB0, (hud + 0x70) as u32, W_804293B0 as u32, 1);
        let _ = platform::call4(
            FN_800C90F0,
            (hud + 0x70) as u32,
            W_804293B4 as u32,
            W_SDA2_3478 as u32,
            1,
        );

        let root = platform::call1(FN_80007220, (hud + 0x70) as u32);
        write_word(hud, 0x490, root);

        let _ = platform::call4(
            FN_800C8D60,
            (hud + 0x70) as u32,
            PANE_NAMES_OTASUKE_INFO as u32,
            (hud + 0x4E8) as u32,
            0x14,
        );
        let _ = platform::call4(
            FN_800C8E40,
            (hud + 0x70) as u32,
            PANE_NAMES_COLLECT_OFF as u32,
            (hud + 0x494) as u32,
            10,
        );

        hud_set_pane_visible(read_word(hud, 0x494), true);
        hud_set_pane_visible(read_word(hud, 0x498), false);
        hud_set_pane_visible(read_word(hud, 0x49C), false);
        hud_set_pane_visible(read_word(hud, 0x49C), true);
        hud_set_pane_visible(read_word(hud, 0x4A0), false);
        hud_set_pane_visible(read_word(hud, 0x4A4), false);
        hud_set_pane_visible(read_word(hud, 0x4A4), true);
        hud_set_pane_visible(read_word(hud, 0x4A8), false);
        hud_set_pane_visible(read_word(hud, 0x4AC), false);

        let _ = platform::call4(
            FN_800C8EB0,
            (hud + 0x70) as u32,
            PANE_NAMES_T_LEFT as u32,
            (hud + 0x4BC) as u32,
            0xB,
        );
        let _ = platform::call5(
            FN_800C8F20,
            (hud + 0x70) as u32,
            PANE_NAMES_T_OTA_CHU_S as u32,
            TABLE_802F5948 as u32,
            1,
            4,
        );
    }
    mounted != 0
}

/// Latches a HUD close request and plays the close animation.
///
/// # Original
/// - Ghidra: `FUN_80158890` at `0x80158890` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158890_FUN_80158890.c`
///
/// # Arguments
/// * `hud` - live HUD object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Clears the pending byte `+0x449` when the "closing" byte `+0x44A` is set.
/// * When `+0x449` is set: clears it, sets `+0x44A`, plays animation slot `0`
///   through `0x800C92F0`, and when the gate object's `+0xBCB` byte is non-zero
///   sets the pause controller's `+0x1D`.
///
/// # Safety
/// The gate object `0x80429D48` and the pause controller must be live when the
/// close path runs.
///
/// # Confidence
/// high - two byte latches, one animation call and one guarded write.
pub unsafe fn gamescene_hud_request_close(hud: usize) {
    if read_byte(hud, 0x44A) != 0 {
        write_byte(hud, 0x449, 0);
    }
    if read_byte(hud, 0x449) != 0 {
        write_byte(hud, 0x449, 0);
        write_byte(hud, 0x44A, 1);
        let _ = platform::call3(FN_800C92F0, (hud + 0x70) as u32, 0, 0);
        let gate = read_global_word(G_HUD_GATE_5938) as usize;
        if read_byte(gate, 0xBCB) != 0 {
            let controller = read_global_word(G_PAUSE_CONTROLLER) as usize;
            write_byte(controller, 0x1D, 1);
        }
    }
}

/// Tests all four player cursors against the three HUD rectangles.
///
/// # Original
/// - Ghidra: `FUN_80158910` at `0x80158910` (376 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158910_FUN_80158910.c`
///
/// # Arguments
/// * `hud` - live HUD object whose root pane at `+0x490` must be visible.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// For each player bit set in `0x80429CA8`, reads the player position through
/// `0x8006CED0`/`0x800B33D0`. A position inside one of the three rectangles
/// `(+0x460,+0x464)x(+0x45C,+0x458)`, `(+0x470,+0x474)x(+0x46C,+0x468)` or
/// `(+0x480,+0x484)x(+0x47C,+0x478)` sets the matching latch `+0x44D/+0x44E/
/// +0x44F` and the shared `+0x450` byte.
///
/// # Safety
/// The player mask, every set player object and the HUD panes must be live.
///
/// # Confidence
/// high - the mask, the loop, both helper calls and all six comparisons are
/// literal.
pub unsafe fn gamescene_hud_hit_test(hud: usize) {
    if (read_byte(read_word(hud, 0x490) as usize, 0xBB) & 1) == 0 {
        return;
    }

    let mask = read_global_byte(G_PLAYER_MASK);
    let mut player: u32 = 0;
    while player < 4 {
        if (mask & (1u8 << (player & 0xFF))) != 0 {
            let object = platform::call1(FN_8005F900, player);
            if object != 0 {
                let mut position = [0f32; 2];
                let _ = platform::call2(FN_8006CED0, position.as_mut_ptr() as u32, object);
                let _ = platform::call1(FN_800B33D0, position.as_mut_ptr() as u32);
                let x = position[0];
                let y = position[1];

                if read_byte(hud, 0x44D) == 0
                    && read_float(hud, 0x460) < x
                    && x < read_float(hud, 0x464)
                    && y < read_float(hud, 0x458)
                    && read_float(hud, 0x45C) < y
                {
                    write_byte(hud, 0x44D, 1);
                    write_byte(hud, 0x450, 1);
                }
                if read_byte(hud, 0x44E) == 0
                    && read_float(hud, 0x470) < x
                    && x < read_float(hud, 0x474)
                    && y < read_float(hud, 0x468)
                    && read_float(hud, 0x46C) < y
                {
                    write_byte(hud, 0x44E, 1);
                    write_byte(hud, 0x450, 1);
                }
                if read_byte(hud, 0x44F) == 0
                    && read_float(hud, 0x480) < x
                    && x < read_float(hud, 0x484)
                    && y < read_float(hud, 0x478)
                    && read_float(hud, 0x47C) < y
                {
                    write_byte(hud, 0x44F, 1);
                    write_byte(hud, 0x450, 1);
                }
            }
        }
        player += 1;
    }
}

/// Steps the three counter values and their pane animation frames.
///
/// # Original
/// - Ghidra: `FUN_80158a90` at `0x80158A90` (436 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158a90_FUN_80158a90.c`
///
/// # Arguments
/// * `hud` - live HUD object.
///
/// # Returns
/// Nothing; returns early while the root pane is hidden.
///
/// # Side effects
/// * Counts how many of the three panes have an unfinished animation: a latch
///   `+0x44D..+0x44F` with frame `< 0x47`, or no latch with frame `0xFF`.
/// * Writes `+0x450 = (count < 3)`.
/// * For each of the three values `+0x418/+0x41C/+0x420`, moves it by `±0x0C`
///   depending on the latch, clamps it to `0x46..0xFF`, writes its low byte to
///   the pane's `+0xB8` and clears the latch.
///
/// # Safety
/// The three panes at `+0x500/+0x504/+0x508` must be live.
///
/// # Confidence
/// high - three identical blocks with explicit clamps and byte stores.
pub unsafe fn gamescene_hud_update_counters(hud: usize) {
    if (read_byte(read_word(hud, 0x490) as usize, 0xBB) & 1) == 0 {
        return;
    }

    let mut active: u32 = 0;
    let frame = read_byte(read_word(hud, 0x500) as usize, 0xB8);
    if read_byte(hud, 0x44D) == 0 {
        if frame == 0xFF {
            active = 1;
        }
    } else if frame < 0x47 {
        active = 1;
    }

    let frame = read_byte(read_word(hud, 0x504) as usize, 0xB8);
    if read_byte(hud, 0x44E) == 0 {
        if frame == 0xFF {
            active = active.wrapping_add(1);
        }
    } else if frame < 0x47 {
        active = active.wrapping_add(1);
    }

    let frame = read_byte(read_word(hud, 0x508) as usize, 0xB8);
    if read_byte(hud, 0x44F) == 0 {
        if frame == 0xFF {
            active = active.wrapping_add(1);
        }
    } else if frame < 0x47 {
        active = active.wrapping_add(1);
    }

    write_byte(hud, 0x450, if active < 3 { 1 } else { 0 });

    let blocks = [
        (0x44Dusize, 0x418usize, 0x500usize),
        (0x44E, 0x41C, 0x504),
        (0x44F, 0x420, 0x508),
    ];
    for (latch, value_off, pane_off) in blocks {
        let step: i32 = if read_byte(hud, latch) != 0 { -0x0C } else { 0x0C };
        let value = (read_word(hud, value_off) as i32).wrapping_add(step);
        write_word(hud, value_off, value as u32);
        if value < 0x47 {
            write_word(hud, value_off, 0x46);
        }
        if (read_word(hud, value_off) as i32) > 0xFE {
            write_word(hud, value_off, 0xFF);
        }
        let stored = read_word(hud, value_off);
        let pane = read_word(hud, pane_off) as usize;
        write_byte(pane, 0xB8, stored as u8);
        write_byte(hud, latch, 0);
    }
}

/// Commits a whole counter decrement in one step.
///
/// # Original
/// - Ghidra: `FUN_80158c50` at `0x80158C50` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158c50_FUN_80158c50.c`
///
/// # Arguments
/// * `hud` - live HUD object; `+0x40C` is the step count and `+0x410` the step
///   scale.
///
/// # Returns
/// `1` when the packed counter object has no whole units left; otherwise `0`
/// after the write-back.
///
/// # Side effects
/// Converts the counter object's `+4` word to whole units (`(value + 0xFFF) >>
/// 12`), loops at most `+0x40C` times subtracting one unit and adding `+0x410`
/// to the player-object value, writes the remaining units through `0x800E3910`,
/// plays sound `0x91` unless the pointer mode bit is set, and applies the
/// player-object value through `0x80060690`.
///
/// # Safety
/// The counter object, sound system and player counter must be live.
///
/// # Confidence
/// high - the packing, the bounded loop and all three calls are explicit.
pub unsafe fn gamescene_hud_commit_decrement(hud: usize) -> u32 {
    let counter = read_global_word(G_COUNTER_OBJECT) as usize;
    let mut units = ((read_word(counter, 4) as i32).wrapping_add(0xFFF) >> 12) as i16;
    if (units as i32) < 1 {
        return 1;
    }

    let mut total: i32 = 0;
    let mut steps = read_word(hud, 0x40C) as i32;
    if steps > 0 {
        loop {
            units = units.wrapping_sub(1);
            total = total.wrapping_add(read_word(hud, 0x410) as i32);
            if units < 1 {
                break;
            }
            steps -= 1;
            if steps == 0 {
                break;
            }
        }
    }

    let _ = platform::call2(FN_800E3910, counter as u32, units as i32 as u32);
    if (read_global_word(G_DISPLAY_INPUT_FLAGS) & 0x40) == 0 {
        let _ = platform::call3(FN_801953A0, read_global_word(G_SOUND_SYSTEM), 0x91, 1);
    }
    let _ = platform::call2(FN_80060690, total as u32, 0xFFFF_FFFF);
    0
}

/// Fades a partial counter decrement over several frames.
///
/// # Original
/// - Ghidra: `FUN_80158d00` at `0x80158D00` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158d00_FUN_80158d00.c`
///
/// # Arguments
/// * `hud` - live HUD object.
///
/// # Returns
/// `1` when the counter object, the stored large value and the display value
/// `+0x3DC` are all zero; otherwise `0`.
///
/// # Side effects
/// * Steps `+0x3DC` down by `3` (clamped at `0`) and rebuilds its transform.
/// * Subtracts `+0x40C` from the whole-unit counter (clamped at `0`) and writes
///   it through `0x800E3910`.
/// * Subtracts `+0x410 * +0x40C` from the stored large value and writes it back
///   to `0x80429CC0`.
/// * Plays sound `0x91` unconditionally.
///
/// # Safety
/// The counter object, HUD fields and sound system must be live.
///
/// # Confidence
/// medium - the flow is complete; one SDA2 source word for the transform is
/// passed as an address because its value is not recovered.
pub unsafe fn gamescene_hud_fade_decrement(hud: usize) -> u32 {
    let counter = read_global_word(G_COUNTER_OBJECT) as usize;
    let mut stored = read_global_word(G_LARGE_COUNTER) as i32;
    let mut units = ((read_word(counter, 4) as i32).wrapping_add(0xFFF) >> 12) as i16;

    if units == 0 && stored == 0 && read_word(hud, 0x3DC) == 0 {
        return 1;
    }

    if read_word(hud, 0x3DC) != 0 {
        let value = (read_word(hud, 0x3DC) as i32).wrapping_sub(3);
        write_word(hud, 0x3DC, value as u32);
        if value < 0 {
            write_word(hud, 0x3DC, 0);
        }
    }

    let _ = platform::call4(
        FN_800B3B60,
        (hud + 0x3DC) as u32,
        W_SDA2_54CC as u32,
        read_word(hud, 0x4DC),
        0,
    );

    if units != 0 {
        units = units.wrapping_sub(read_word(hud, 0x40C) as i16);
        if units < 0 {
            units = 0;
        }
    }
    let _ = platform::call2(FN_800E3910, counter as u32, units as i32 as u32);

    if stored != 0 {
        stored = stored.wrapping_sub(
            (read_word(hud, 0x410) as i32).wrapping_mul(read_word(hud, 0x40C) as i32),
        );
        if stored < 0 {
            stored = 0;
        }
    }
    write_global_word(G_LARGE_COUNTER, stored as u32);
    let _ = platform::call3(FN_801953A0, read_global_word(G_SOUND_SYSTEM), 0x91, 1);
    0
}

/// Ticks the HUD counter and leaves the state when it finishes.
///
/// # Original
/// - Ghidra: `FUN_80158f70` at `0x80158F70` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80158f70_FUN_80158f70.c`
///
/// # Arguments
/// * `hud` - live HUD object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Increments `+0x400` while it is below `+0x404`. Otherwise, when the
/// `0x80429FE2` suppression byte is clear, commits through
/// [`gamescene_hud_commit_decrement`]; when it is set, fades through
/// [`gamescene_hud_fade_decrement`]. A non-zero return resets `+0x400`, plays
/// sound `0x92` (only in the commit path when the pointer mode bit is clear)
/// and requests `dGameDisplay_c::StateID_ProcGoal` (`0x80377138`) on the
/// interface at `+0x390`.
///
/// # Safety
/// The state interface at `hud + 0x390` and the sound system must be live.
///
/// # Confidence
/// high - both paths, the sound guards and the descriptor are explicit.
pub unsafe fn gamescene_hud_tick(hud: usize) {
    if (read_word(hud, 0x400) as i32) < (read_word(hud, 0x404) as i32) {
        write_word(hud, 0x400, read_word(hud, 0x400).wrapping_add(1));
    } else if read_global_byte(G_SUPPRESS_1UP) == 0 {
        if gamescene_hud_commit_decrement(hud) != 0 {
            write_word(hud, 0x400, 0);
            if (read_global_word(G_DISPLAY_INPUT_FLAGS) & 0x40) == 0 {
                let _ = platform::call3(FN_80195380, read_global_word(G_SOUND_SYSTEM), 0x92, 1);
            }
            change_state(hud + 0x390, DISPLAY_STATE_PROC_GOAL_B);
        }
    } else if gamescene_hud_fade_decrement(hud) != 0 {
        write_word(hud, 0x400, 0);
        let _ = platform::call3(FN_80195380, read_global_word(G_SOUND_SYSTEM), 0x92, 1);
        change_state(hud + 0x390, DISPLAY_STATE_PROC_GOAL_B);
    }
}

/// Saves the pane positions and applies the shared colour before hiding.
///
/// # Original
/// - Ghidra: `FUN_80159250` at `0x80159250` (656 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159250_FUN_80159250.c`
///
/// # Arguments
/// * `hud` - live HUD object. Ghidra prints `void` because the body starts with
///   the `0x802DCD74` register-save helper; `r3` is `hud` and `r4` the player.
/// * `player` - player index; the per-player hidden byte is at
///   `hud + player + 0x454`.
///
/// # Returns
/// Nothing; returns immediately when the player byte is already set.
///
/// # Side effects
/// Initialises the shared colour at `0x8042A32C..0x8042A32F` to
/// `A0 A0 A0 FF` on first use, sets the per-player hidden byte, then for the
/// three panes selected by `0x802F5978` and `0x802F5958` saves the `+0x18/+0x1C`
/// words into `hud + player*8 + {0x330,0x350,0x370}` and writes the shared
/// colour into the pane record's `+0x18..+0x1E`.
///
/// # Safety
/// The three pane handles and the `+0x68` vtable slot result must be writable.
///
/// # Confidence
/// medium - every store is literal; the `+0x68` result is treated as a 0x20-byte
/// draw record because the same fields are read back by `0x801594E0`.
pub unsafe fn gamescene_hud_hide_panes(hud: usize, player: u32) {
    if read_byte(hud + player as usize, 0x454) != 0 {
        return;
    }

    if read_global_byte(G_PANE_COLOUR_INIT) == 0 {
        write_global_byte(G_PANE_COLOUR, 0xA0);
        write_global_byte(G_PANE_COLOUR + 1, 0xA0);
        write_global_byte(G_PANE_COLOUR + 2, 0xA0);
        write_global_byte(G_PANE_COLOUR + 3, 0xFF);
        write_global_byte(G_PANE_COLOUR_INIT, 1);
    }
    write_byte(hud + player as usize, 0x454, 1);

    let index = read_global_word(TABLE_802F5978 + player as usize * 4) as usize;
    let pane = read_word(hud, index * 4 + 0x494) as usize;
    hud_save_pane_and_colour(hud, player, 0x330, pane);

    let index = read_global_word(TABLE_802F5958 + player as usize * 8) as usize;
    let pane = read_word(hud, index * 4 + 0x4BC) as usize;
    hud_save_pane_and_colour(hud, player, 0x350, pane);

    let index = read_global_word(TABLE_802F5958 + (player as usize * 2 + 1) * 4) as usize;
    let pane = read_word(hud, index * 4 + 0x4BC) as usize;
    hud_save_pane_and_colour(hud, player, 0x370, pane);
}

/// Restores the pane positions and clears the hidden byte.
///
/// # Original
/// - Ghidra: `FUN_801594e0` at `0x801594E0` (332 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801594e0_FUN_801594e0.c`
///
/// # Arguments
/// * `hud` - live HUD object.
/// * `player` - player index; the hidden byte is at `hud + player + 0x454`.
///
/// # Returns
/// Nothing; returns immediately when the player byte is clear.
///
/// # Side effects
/// Clears the hidden byte, then restores the three pane records selected by
/// `0x802F59A8` and `0x802F5988` from
/// `hud + player*8 + {0x330,0x350,0x370}`.
///
/// # Safety
/// The three panes and the `+0x68` vtable slot result must be writable.
///
/// # Confidence
/// medium - every store is literal; the `+0x68` result layout is inferred from
/// the matching save function.
pub unsafe fn gamescene_hud_show_panes(hud: usize, player: u32) {
    if read_byte(hud + player as usize, 0x454) == 0 {
        return;
    }
    write_byte(hud + player as usize, 0x454, 0);

    let index = read_global_word(TABLE_802F59A8 + player as usize * 4) as usize;
    let pane = read_word(hud, index * 4 + 0x494) as usize;
    hud_restore_pane(hud, player, 0x330, pane);

    let index = read_global_word(TABLE_802F5988 + player as usize * 8) as usize;
    let pane = read_word(hud, index * 4 + 0x4BC) as usize;
    hud_restore_pane(hud, player, 0x350, pane);

    let index = read_global_word(TABLE_802F5988 + (player as usize * 2 + 1) * 4) as usize;
    let pane = read_word(hud, index * 4 + 0x4BC) as usize;
    hud_restore_pane(hud, player, 0x370, pane);
}

/// Spawns the `Wm_2d_starcoinget` effect at one player's star-coin pane.
///
/// # Original
/// - Ghidra: `FUN_801597a0` at `0x801597A0` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801597a0_FUN_801597a0.c`
///
/// # Arguments
/// * `hud` - live HUD object.
/// * `player` - player index; the pane is at `+0x4E8` index `player + 0x11`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds the same 15-word transform as [`wm_spawn_1up_effects`] from the pane
/// matrix, overwrites entries `6`/`10` with `0`/`1`, scales it with `0x800B37E0`
/// and spawns `"Wm_2d_starcoinget"` through `0x8016C890`.
///
/// # Safety
/// The pane handle and its matrix must be readable.
///
/// # Confidence
/// medium - the transform order is the same observed one; the third word is the
/// unresolved SDA2 constant, read from its address.
pub unsafe fn gamescene_hud_spawn_star_coin(hud: usize, player: u32) {
    let pane = read_word(hud, (player as usize + 0x11) * 4 + 0x4E8) as usize;
    let mut transform = [0u32; 15];
    fill_effect_transform(transform.as_mut_ptr(), pane);
    transform[6] = transform[0];
    transform[10] = transform[1];

    let _ = platform::call2(FN_800B37E0, transform.as_mut_ptr() as u32, 0);
    let _ = platform::call5(
        FN_8016C890,
        S_WM_2D_STARCOIN_GET as u32,
        0,
        transform.as_ptr() as u32,
        0,
        0,
    );
}

/// Chooses the star-coin pane of each of the three HUD rows.
///
/// # Original
/// - Ghidra: `FUN_80159af0` at `0x80159AF0` (444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159af0_FUN_80159af0.c`
///
/// # Arguments
/// * `hud` - live HUD object. Ghidra prints `void` because the body starts with
///   the `0x802DCD5C` register-save helper; `r3` is `hud`.
///
/// # Returns
/// Nothing; returns early when `+0x453` is set, the grid position is outside
/// `10 x 0x2A`, or the scene mode is `2..3`.
///
/// # Side effects
/// For rows `0..2`, queries `0x800CE210(course, x, y, row)`. When the row is
/// already collected (or the world mode is non-zero), shows `+0x494` when the
/// row's state word is `4` and clears `+0x498`; otherwise shows `+0x498`,
/// stores state `2` and spawns the star-coin effect when the state changed.
/// Ends with the layout vtable slot `+0x10`.
///
/// # Safety
/// The course object, the three state words at `0x803741B0` and the pane arrays
/// must be live.
///
/// # Confidence
/// high - the grid guard, both branches and every pane write are explicit.
pub unsafe fn gamescene_hud_select_player_pane(hud: usize) {
    if read_byte(hud, 0x453) != 0 {
        return;
    }

    let course = platform::call2(FN_800E0380, read_global_word(G_COURSE_5640), 0xFFFF_FFFF);
    let owner = read_global_word(G_COURSE_54B8) as usize;
    let grid_x = read_byte(owner, 0x120C);
    let grid_y = read_byte(owner, 0x120D);
    if grid_x >= 10 || grid_y >= 0x2A || read_global_byte(G_SCENE_MODE_80315B98).wrapping_sub(2) <= 1
    {
        return;
    }

    let mut player: u32 = 0;
    while player < 3 {
        let pane_slot = hud + player as usize * 8;
        let state_slot = hud + player as usize * 4;
        let pane_a = read_word(pane_slot, 0x494) as usize;
        let pane_b = read_word(pane_slot, 0x498) as usize;
        let collected = platform::call4(
            FN_800CE210,
            course,
            grid_x as u32,
            grid_y as u32,
            player,
        ) as u8;

        if collected == 0 || read_global_byte(G_SCENE_MODE_80315B98) != 0 {
            if read_global_word(TABLE_803741B0 + player as usize * 4) == 4 {
                hud_set_pane_visible(pane_a as u32, true);
                hud_set_pane_visible(pane_b as u32, false);
                if read_word(state_slot, 0x3EC) != 0 {
                    write_word(state_slot, 0x3EC, 0);
                }
            } else {
                hud_set_pane_visible(pane_a as u32, false);
                hud_set_pane_visible(pane_b as u32, true);
                if read_word(state_slot, 0x3EC) != 2 {
                    write_word(state_slot, 0x3EC, 2);
                    gamescene_hud_spawn_star_coin(hud, player);
                }
            }
        } else {
            hud_set_pane_visible(pane_a as u32, false);
            hud_set_pane_visible(pane_b as u32, true);
            if read_word(state_slot, 0x3EC) != 2 {
                write_word(state_slot, 0x3EC, 2);
            }
        }
        player += 1;
    }

    let _ = vcall_layout0(hud, 0x10);
}

/// Destructor callback registered by the four `dGameDisplay_c` descriptors.
///
/// This is the callback address passed to `0x802DC780` by
/// [`dgame_display_register_states`] (`FN_8015A250`). It has the same body as
/// [`pausewindow_state_release`] but a different address and owner.
///
/// # Original
/// - Ghidra: `FUN_8015a250` at `0x8015A250` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015a250_FUN_8015a250.c`
///
/// # Arguments
/// * `state` - state object passed by the descriptor manager, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `state` unchanged (`return param_1`).
///
/// # Side effects
/// Calls `0x8015F800(state, 0)` and, when `free > 0`, `0x802B9280(state)`.
///
/// # Safety
/// `state` must be null or a live state object accepted by both callees.
///
/// # Confidence
/// high - the two guards and both calls are explicit.
pub unsafe fn dgame_display_state_release(state: usize, free: i32) -> usize {
    if state != 0 {
        let _ = platform::call2(FN_8015F800, state as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, state as u32);
        }
    }
    state
}

// ===========================================================================
// Wave 20 - pause cursor, water-effect pool and save image
// (0x800C9B00, 0x800D14E0..0x800D3070, 0x800E0180..0x800E04B0)
// ===========================================================================
//
// The wave-16 pause-menu controller (`0x800D0940..0x800D11F0`) is followed by
// the cursor/target class it drives: `0x800D14E0..0x800D2200` selects the
// object under a player cursor and keeps its debounce counters. The same
// 0x800D2xxx block holds the ten-slot water-splash effect pool, whose element
// size (0x13C) matches the fields written by the `Wm_mr_watersplash` spawn at
// `0x800D2AC0`. Finally `0x800E0180..0x800E04B0` builds the save-image object
// that [`crate::game::scene`] reads as `0x8035DCC0`; these bodies are engine
// start-up code, not the boot scene's DVD state machine.
//
// All four clusters live in the engine band, so every callee outside this file
// is still called by absolute address.

// ---------------------------------------------------------------------------
// Wave-20 globals
// ---------------------------------------------------------------------------

/// `r13 - 0x56A4` -> `0x80429FDC`: pointer to the cursor's current selection
/// object, or `0`.
///
/// Read by [`cursor_selection_active`] only. `// unknown:` the class name; the
/// byte it reads at `+0x26D` is the "selection valid" flag.
const G_CURSOR_SELECTION: usize = SDA_BASE - 0x56A4;

/// `r13 - 0x5698` -> `0x80429FE8`: head of the cursor target list.
///
/// [`cursor_acquire_target`] walks it through `+0x08`. `// unknown:` the list
/// owner.
const G_CURSOR_LIST: usize = SDA_BASE - 0x5698;

/// `r13 - 0x5690` -> `0x80429FF0`: pointer to the water-effect pool published
/// by [`water_effect_pool_construct`].
const G_WATER_POOL: usize = SDA_BASE - 0x5690;

/// `r13 - 0x5640` -> `0x8042A040`: pointer to the save-image object.
///
/// Written by [`save_image_construct`] and read by the boot/save code in
/// [`crate::game::scene`]. The object is 0x3FC0 bytes: a 0x6A0-byte header at
/// `+0x20` (whose data starts at `+0x24` and whose checksum sits at `+0x6BC`),
/// then two arrays of three 0x980-byte records at `+0x6C0` and `+0x2340`.
const G_SAVE_IMAGE: usize = SDA_BASE - 0x5640;

/// `r13 - 0x56C8` -> `0x80429FB8`: pointer to the object whose `+0x74` word
/// gates [`save_image_flag_get`].
///
/// `// unknown:` the owner; the pointer is the same one the HUD mount uses for
/// its gate lookups.
const G_SAVE_MANAGER: usize = SDA_BASE - 0x56C8;

/// `r13 - 0x7A78` -> `0x80427C08`: descriptor passed to `0x802AC480` by
/// [`display_object_mark_dirty`].
///
/// `// unknown:` contents; forwarded by address.
const W_80427C08: usize = SDA_BASE - 0x7A78;

/// `r2 - 0x6694` -> `0x8042C9EC`: scale applied to the target's `+0xC8` value
/// in [`cursor_acquire_target`].
///
/// `// unknown:` value; read from memory instead of being invented.
const F32_R2_6694: usize = SDA2_BASE - 0x6694;

/// `r2 - 0x6698` -> `0x8042C9E8`: base offset added to the target's vertical
/// position and also used as its lower clamp.
const F32_R2_6698: usize = SDA2_BASE - 0x6698;

/// `r2 - 0x6690` -> `0x8042C9F0`: lower clamp of the horizontal position.
const F32_R2_6690: usize = SDA2_BASE - 0x6690;

/// `r2 - 0x668C` -> `0x8042C9F4`: upper clamp of the horizontal position.
const F32_R2_668C: usize = SDA2_BASE - 0x668C;

/// `r2 - 0x6688` -> `0x8042C9F8`: upper clamp of the vertical position.
const F32_R2_6688: usize = SDA2_BASE - 0x6688;

// ---------------------------------------------------------------------------
// Wave-20 vtable/descriptor words (all observed as literal stores)
// ---------------------------------------------------------------------------

/// `0x80317AFC`: vtable installed by [`water_effect_construct`].
const VTABLE_WATER_EFFECT: u32 = 0x8031_7AFC;

/// `0x80317AF0`: vtable installed by [`water_effect_pool_construct`].
const VTABLE_WATER_POOL: u32 = 0x8031_7AF0;

/// `0x80308A80`: second vtable written at `+0x04` by
/// [`water_effect_construct`]. The same table appears in the world-map classes.
const VTABLE_80308A80: u32 = 0x8030_8A80;

/// `0x802F1220`: descriptor passed to `0x802DCBFC` by
/// [`cursor_begin_track`]. `// unknown:` contents.
const DESC_802F1220: usize = 0x802F_1220;

/// `0x8016E4F0`: allocator-lock thunk to `0x802B9170`; returns the previous
/// heap selector.
const FN_8016E4F0: usize = 0x8016_E4F0;

/// `0x802B9230(size, align)`: aligned allocation on the current heap.
const FN_802B9230: usize = 0x802B_9230;

/// `0x800D29F0`: element constructor address passed to the array constructor.
const FN_800D29F0: usize = 0x800D_29F0;

/// `0x800D2A50`: element destructor address passed to both array helpers.
const FN_800D2A50: usize = 0x800D_2A50;

// ---------------------------------------------------------------------------
// Wave-20 callees (called by address; none is translated in this module)
// ---------------------------------------------------------------------------

/// `0x800D16B0`: three-argument predicate used by [`cursor_acquire_target`].
///
/// Ghidra prints a parameterless body that tail-calls `0x802DCBFC`; the call
/// site passes `(cursor, target, arg)` and consumes `r3`. The Rust keeps the
/// call-site arguments and marks the operand contract `// unknown:`.
const FN_800D16B0: usize = 0x800D_16B0;

/// `0x802DCBFC`: runtime support routine reached behind `0x800D16B0` and
/// [`cursor_begin_track`]. `// unknown:` exact semantics.
const FN_802DCBFC: usize = 0x802D_CBFC;

/// `0x802D7AA0`: C++ base-object constructor used by the water-effect objects.
const FN_802D7AA0: usize = 0x802D_7AA0;

/// `0x802D7B20`: C++ base-object destructor used by [`water_effect_release`].
const FN_802D7B20: usize = 0x802D_7B20;

/// `0x802DC9A0(base, construct, destruct, size, count)`: array constructor.
const FN_802DC9A0: usize = 0x802D_C9A0;

/// `0x802DCA98(base, destruct, size, count)`: array destructor.
const FN_802DCA98: usize = 0x802D_CA98;

/// `0x800E0290`: destructor of one 0x980-byte save record.
///
/// The body is translated as [`save_record_release`]; the array constructor in
/// [`save_image_construct`] still receives the raw address so the call site
/// stays literal.
const FN_800E0290: usize = 0x800E_0290;

/// `0x800CDEC0`: constructor of one 0x980-byte save record.
const FN_800CDEC0: usize = 0x800C_DEC0;

/// `0x800CE570`: empty initialiser of the save header (`no-op` in evidence).
const FN_800CE570: usize = 0x800C_E570;

/// `0x800CE580`: initialises the save header: clears 0x6A0 bytes and copies
/// four version bytes plus a `0x0E` tag.
const FN_800CE580: usize = 0x800C_E580;

/// `0x800CDED0`: per-record finaliser called twice per record during
/// [`save_image_construct`].
const FN_800CDED0: usize = 0x800C_DED0;

/// `0x800CE020`: marks one record as fully initialised (`+0` = `0x0E`).
const FN_800CE020: usize = 0x800C_E020;

/// `0x800CE6C0`: same tag update for the 0x6A0-byte header (`+4` = `0x0E`).
const FN_800CE6C0: usize = 0x800C_E6C0;

/// `0x80004364(dst, src, len)`: byte copy (already used elsewhere as `memcpy`).
const FN_80004364: usize = 0x8000_4364;

/// `0x800CFBB0`: returns the in-memory save image base `0x8035DCC0`.
const FN_800CFBB0: usize = 0x800C_FBB0;

/// `0x8015F130`: two-argument checksum thunk (`0x801B8620` forwarding), used by
/// [`save_image_update_checksums`].
const FN_8015F130: usize = 0x8015_F130;

/// `0x80169CD0`: constructs one 0x40-byte sub-object of a water node.
const FN_80169CD0: usize = 0x8016_9CD0;

/// `0x80169D20(object, -1)`: releases one sub-object of a water node.
const FN_80169D20: usize = 0x8016_9D20;

/// `0x800D2B90`: per-element water-effect update, translated as
/// [`water_effect_update`]. [`water_effect_pool_update`] still calls the raw
/// address so its ten-iteration loop stays literal.
const FN_800D2B90: usize = 0x800D_2B90;

/// `0x802AC480(name, descriptor)`: resource lookup used by
/// [`display_object_mark_dirty`].
const FN_802AC480: usize = 0x802A_C480;

// ---------------------------------------------------------------------------
// 0x800D14E0..0x800D2200 - pause cursor / target selection
// ---------------------------------------------------------------------------

/// Returns the cursor's "selection valid" flag.
///
/// In plain English: if no selection object is stored, the answer is `0`;
/// otherwise it is the byte at `selection + 0x26D`.
///
/// # Original
/// - Ghidra: `FUN_800d14e0` at `0x800D14E0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d14e0_FUN_800d14e0.c`
///
/// # Arguments
/// None; the selection object comes from [`G_CURSOR_SELECTION`].
///
/// # Returns
/// `0` when the slot is empty, else the flag byte.
///
/// # Side effects
/// Two reads; no writes.
///
/// # Safety
/// The global slot and `selection + 0x26D` must be readable when non-null.
///
/// # Confidence
/// high - the null guard and the load are explicit.
pub unsafe fn cursor_selection_active() -> u8 {
    let selection = read_global_word(G_CURSOR_SELECTION);
    if selection == 0 {
        return 0;
    }
    platform::read_u8(selection as usize + 0x26D)
}

/// Stores a new cursor target and its "valid" byte.
///
/// # Original
/// - Ghidra: `FUN_800d1500` at `0x800D1500` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d1500_FUN_800d1500.c`
///
/// # Arguments
/// * `cursor` - live cursor object.
/// * `target` - word stored at `cursor + 4`.
/// * `flag` - byte stored at `cursor + 0x2B`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word and one byte store.
///
/// # Safety
/// `cursor + 4` and `cursor + 0x2B` must be writable.
///
/// # Confidence
/// high - two literal stores.
pub unsafe fn cursor_set_target(cursor: usize, target: u32, flag: u8) {
    write_word(cursor, 0x04, target);
    write_byte(cursor, 0x2B, flag);
}

/// Picks the first unvisited list target accepted by `0x800D16B0` and latches
/// its position.
///
/// In plain English: when the cursor has no target yet, walk the list at
/// [`G_CURSOR_LIST`]; the first node that is not the previously used node and
/// that `0x800D16B0(cursor, node, arg)` accepts becomes the target. Its owner
/// object supplies the world position, which is written to `cursor + 0x14` /
/// `+0x18` and clamped to the four SDA2 bounds. The node's `+0x18` word
/// receives the owner's `+0x10C` value.
///
/// # Original
/// - Ghidra: `FUN_800d1510` at `0x800D1510` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d1510_FUN_800d1510.c`
///
/// # Arguments
/// * `cursor` - cursor object; `+0x0C` is the current target (early-out),
///   `+0x10` the previous target and `+0x04` the owner/player object.
/// * `arg` - third argument forwarded to `0x800D16B0`.
///
/// # Returns
/// `1` when a target was latched, `0` when the list yields none.
///
/// # Side effects
/// Writes `+0x0C`, `+0x10`, `+0x14`, `+0x18`, `+0x24`, `+0x28`, `+0x2A`,
/// `+0x2D` of the cursor and `+0x18` of the selected node.
///
/// # Safety
/// The cursor, its owner and every list node must be live; the original does
/// not null-check the owner.
///
/// # Confidence
/// medium - the control flow and every store are explicit, but the two arguments
/// of `0x800D16B0` and the five SDA2 clamp values are only known by address.
pub unsafe fn cursor_acquire_target(cursor: usize, arg: u32) -> u32 {
    if read_word(cursor, 0x0C) != 0 {
        return 0;
    }

    let mut node = read_global_word(G_CURSOR_LIST) as usize;
    while node != 0 {
        if read_word(cursor, 0x10) != node as u32 {
            // observed: the call site passes (cursor, node, arg) and tests r3.
            // unknown: 0x800D16B0's real parameter list.
            let accepted = platform::call3(FN_800D16B0, cursor as u32, node as u32, arg);
            if accepted != 0 {
                write_word(cursor, 0x0C, node as u32);

                let owner = read_word(cursor, 0x04) as usize;
                write_word(cursor, 0x10, node as u32);
                write_word(cursor, 0x24, read_word(owner, 0x10C));

                let x = platform::read_f32(owner + 0xC4);
                let y = platform::read_f32(F32_R2_6698)
                    + platform::read_f32(F32_R2_6694) * platform::read_f32(owner + 0xC8);
                platform::write_f32(cursor + 0x14, x);
                platform::write_f32(cursor + 0x18, y);

                // Horizontal clamp: low bound first, then the high bound.
                let low = platform::read_f32(F32_R2_6690);
                if low <= x {
                    let high = platform::read_f32(F32_R2_668C);
                    if high < x {
                        platform::write_f32(cursor + 0x14, high);
                    }
                } else {
                    platform::write_f32(cursor + 0x14, low);
                }

                // Vertical clamp: the C re-reads the stored word for the test.
                let low = platform::read_f32(F32_R2_6698);
                if low <= platform::read_f32(cursor + 0x18) {
                    let high = platform::read_f32(F32_R2_6688);
                    if high < platform::read_f32(cursor + 0x18) {
                        platform::write_f32(cursor + 0x18, high);
                    }
                } else {
                    platform::write_f32(cursor + 0x18, low);
                }

                platform::write_u16(cursor + 0x28, 0);
                write_byte(cursor, 0x2D, 0x1E);
                write_byte(cursor, 0x2A, 0);

                write_word(node, 0x18, read_word(owner, 0x10C));
                return 1;
            }
        }
        node = read_word(node, 0x08) as usize;
    }
    0
}

/// Ages the cursor's two debounce counters.
///
/// In plain English: decrement the lock timer at `+0x2D`; when it reaches zero
/// drop the previous target at `+0x10`. If a target is present, tick the
/// countdown at `+0x28`; when it is already zero and the two bytes `+0x2E` /
/// `+0x2F` differ, reload the countdown with `2`.
///
/// # Original
/// - Ghidra: `FUN_800d1640` at `0x800D1640` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d1640_FUN_800d1640.c`
///
/// # Arguments
/// * `cursor` - live cursor object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x2D`, `+0x10` and `+0x28`.
///
/// # Safety
/// `cursor + 0x10..+0x30` must be writable.
///
/// # Confidence
/// high - every branch and store is explicit; the byte timer uses the signed
/// `char` decrement of the original (values here are small and positive).
pub unsafe fn cursor_tick_lock(cursor: usize) {
    if read_byte(cursor, 0x2D) != 0 {
        write_byte(cursor, 0x2D, read_byte(cursor, 0x2D).wrapping_sub(1));
    }
    if read_byte(cursor, 0x2D) == 0 {
        write_word(cursor, 0x10, 0);
    }
    if read_word(cursor, 0x0C) == 0 {
        return;
    }
    let countdown = platform::read_u16(cursor + 0x28) as i16;
    if countdown > 0 {
        platform::write_u16(cursor + 0x28, (countdown - 1) as u16);
        return;
    }
    if read_byte(cursor, 0x2E) == read_byte(cursor, 0x2F) {
        return;
    }
    platform::write_u16(cursor + 0x28, 2);
}

/// Starts the target-tracking animation and publishes its phase byte.
///
/// In plain English: with no target the call is a success (`1`) and nothing is
/// written. With a target, reset the lock timer to `0x1E`, copy `+0x2E` into
/// `+0x2F`, run the `0x802DCBFC` helper with the cursor and descriptor
/// `0x802F1220`, and store the low byte of its result at `+0x2A`.
///
/// # Original
/// - Ghidra: `FUN_800d1f10` at `0x800D1F10` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d1f10_FUN_800d1f10.c`
///
/// # Arguments
/// * `cursor` - live cursor object.
///
/// # Returns
/// The raw result of `0x802DCBFC` when a target exists, otherwise `1`.
///
/// # Side effects
/// Writes `+0x2D`, `+0x2F` and `+0x2A`.
///
/// # Safety
/// `cursor + 0x2A..+0x30` must be writable.
///
/// # Confidence
/// high - the flow, the `0x802F1220` record table and the `12 * target[+0x20]`
/// index are literal (`0x800D1F44..0x800D1F5C`); [`interface_call2`] resolves
/// that descriptor and forwards the table base in `r4`.
pub unsafe fn cursor_begin_track(cursor: usize) -> u32 {
    let target = read_word(cursor, 0x0C) as usize;
    if target == 0 {
        return 1;
    }
    write_byte(cursor, 0x2D, 0x1E);
    write_byte(cursor, 0x2F, read_byte(cursor, 0x2E));
    // observed: `lis/addi r4 = 0x802F1220`, `lbz r0,0x20(r5)`,
    // `mulli r0,r0,12`, `add r12,r4,r0`, `bl 0x802DCBFC`.
    let result = interface_call2::<u32>(
        DESC_802F1220,
        cursor as u32,
        (read_byte(target, 0x20) as u32).wrapping_mul(12),
        DESC_802F1220 as u32,
    );
    write_byte(cursor, 0x2A, (result & 0xFF) as u8);
    result
}

/// Unlinks a cursor from the list that owns it.
///
/// In plain English: the cursor stores its list in `+0x0C`. Two spellings of
/// the unlink exist in the original: the flat list reaches the cursor through
/// the head's `+0x10` word, the nested list through a node's `+0x08` word.
/// Both reset the cursor's `+0x0C`.
///
/// # Original
/// - Ghidra: `FUN_800d2200` at `0x800D2200` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2200_FUN_800d2200.c`
///
/// # Arguments
/// * `cursor` - cursor object to unlink.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Rewrites one link word and clears `cursor + 0x0C`.
///
/// # Safety
/// Every node visited must be readable and the rewritten link writable.
///
/// # Confidence
/// high - the two walk shapes and both updates are literal.
pub unsafe fn cursor_unlink(cursor: usize) {
    let mut node = read_word(cursor, 0x0C) as usize;
    if node == 0 {
        return;
    }

    let mut next = read_word(node, 0x10) as usize;
    if read_word(node, 0x10) == cursor as u32 {
        write_word(node, 0x10, read_word(cursor, 0x08));
        write_word(cursor, 0x0C, 0);
        return;
    }

    loop {
        node = next;
        if node == 0 {
            return;
        }
        next = read_word(node, 0x08) as usize;
        if read_word(node, 0x08) == cursor as u32 {
            break;
        }
    }
    write_word(node, 0x08, read_word(cursor, 0x08));
    write_word(cursor, 0x0C, 0);
}

// ---------------------------------------------------------------------------
// 0x800D29F0..0x800D2D80 - ten-slot water-effect pool
// ---------------------------------------------------------------------------
//
// Element stride 0x13C. `0x800D2AC0`, translated as [`water_effect_spawn`],
// writes exactly the offsets this constructor clears, and tests `+0x138` as
// "in use", which is the word `water_effect_construct` leaves at zero.

/// Constructs one water-effect element.
///
/// In plain English: install the element vtable at `+0`, construct the base
/// object at `+4`, overwrite its vtable with `0x80308A80`, and clear the
/// "in use" word at `+0x138`.
///
/// # Original
/// - Ghidra: `FUN_800d29f0` at `0x800D29F0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d29f0_FUN_800d29f0.c`
///
/// # Arguments
/// * `element` - storage for one element (0x13C bytes).
///
/// # Returns
/// `element`, unchanged (`return param_1`).
///
/// # Side effects
/// Writes the two vtable words and clears `+0x138`.
///
/// # Safety
/// `element..element+0x13C` must be writable.
///
/// # Confidence
/// high - three literal stores plus the base constructor; the element size is
/// proven by the pool stride.
pub unsafe fn water_effect_construct(element: usize) -> usize {
    write_word(element, 0x00, VTABLE_WATER_EFFECT);
    let _ = platform::call1(FN_802D7AA0, (element + 4) as u32);
    write_word(element, 0x04, VTABLE_80308A80);
    write_word(element, 0x138, 0);
    element
}

/// Releases one water-effect element.
///
/// # Original
/// - Ghidra: `FUN_800d2a50` at `0x800D2A50` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2a50_FUN_800d2a50.c`
///
/// # Arguments
/// * `element` - element to release, or `0`.
/// * `free` - positive when the storage must be freed.
///
/// # Returns
/// `element`, unchanged.
///
/// # Side effects
/// Calls the base destructor on `element + 4` and, when `free > 0`,
/// `0x802B9280(element)`.
///
/// # Safety
/// `element` must be null or a live element.
///
/// # Confidence
/// medium - the guards and the return are explicit; the export prints the
/// release call without an operand, so the pointer is reconstructed from the
/// guard (`// partial:` operand).
pub unsafe fn water_effect_release(element: usize, free: i32) -> usize {
    if element != 0 {
        if element + 4 != 0 {
            let _ = platform::call2(FN_802D7B20, (element + 4) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, element as u32);
        }
    }
    element
}

/// Constructs the ten-element water-effect pool.
///
/// In plain English: install the pool vtable, build ten 0x13C-byte elements
/// with [`water_effect_construct`] / [`water_effect_release`] as their
/// constructor/destructor pair, and publish the pool in [`G_WATER_POOL`].
///
/// # Original
/// - Ghidra: `FUN_800d2c20` at `0x800D2C20` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2c20_FUN_800d2c20.c`
///
/// # Arguments
/// * `pool` - storage for the pool (4 + 10 * 0x13C bytes).
///
/// # Returns
/// `pool`, unchanged.
///
/// # Side effects
/// Runs the array constructor and writes the global slot.
///
/// # Safety
/// The pool storage must be writable and large enough.
///
/// # Confidence
/// high - vtable, element size, count and global store are all literal.
pub unsafe fn water_effect_pool_construct(pool: usize) -> usize {
    write_word(pool, 0x00, VTABLE_WATER_POOL);
    let _ = platform::call5(
        FN_802DC9A0,
        (pool + 4) as u32,
        FN_800D29F0 as u32,
        FN_800D2A50 as u32,
        0x13C,
        10,
    );
    write_global_word(G_WATER_POOL, pool as u32);
    pool
}

/// Destroys the ten-element water-effect pool.
///
/// # Original
/// - Ghidra: `FUN_800d2c80` at `0x800D2C80` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2c80_FUN_800d2c80.c`
///
/// # Arguments
/// * `pool` - pool to destroy, or `0`.
/// * `free` - positive when the pool storage must be freed.
///
/// # Returns
/// `pool`, unchanged.
///
/// # Side effects
/// Runs the ten-element array destructor and, when `free > 0`, frees the pool.
///
/// # Safety
/// `pool` must be null or a live pool.
///
/// # Confidence
/// high - the guards, the array destructor arguments and the return are
/// explicit.
pub unsafe fn water_effect_pool_release(pool: usize, free: i32) -> usize {
    if pool != 0 {
        let _ = platform::call4(
            FN_802DCA98,
            (pool + 4) as u32,
            FN_800D2A50 as u32,
            0x13C,
            10,
        );
        if free > 0 {
            let _ = platform::call1(FN_802B9280, pool as u32);
        }
    }
    pool
}

/// Ticks all ten water-effect elements.
///
/// # Original
/// - Ghidra: `FUN_800d2d80` at `0x800D2D80` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2d80_FUN_800d2d80.c`
///
/// # Arguments
/// * `pool` - live pool object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800D2B90` ten times.
///
/// # Safety
/// Each element must be live.
///
/// # Confidence
/// high - a literal ten-iteration loop.
pub unsafe fn water_effect_pool_update(pool: usize) {
    let mut element = pool + 4;
    let mut index = 0;
    while index < 10 {
        platform::call1(FN_800D2B90, element as u32);
        index += 1;
        element += 0x13C;
    }
}

/// Constructs one 0x80-byte water-grid node.
///
/// In plain English: initialise the two 0x40-byte sub-objects embedded in the
/// node through `0x80169CD0`.
///
/// # Original
/// - Ghidra: `FUN_800d3030` at `0x800D3030` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3030_FUN_800d3030.c`
///
/// # Arguments
/// * `node` - storage for one 0x80-byte node.
///
/// # Returns
/// `node`, unchanged.
///
/// # Side effects
/// Runs two sub-object constructors.
///
/// # Safety
/// `node..node+0x80` must be writable.
///
/// # Confidence
/// medium - both calls and the return are literal, but the export drops the
/// first call's operand (the object pointer is reconstructed from `r3`) and the
/// class names are inferred from the pool that owns the nodes.
pub unsafe fn water_node_construct(node: usize) -> usize {
    let _ = platform::call1(FN_80169CD0, node as u32);
    let _ = platform::call1(FN_80169CD0, (node + 0x40) as u32);
    node
}

/// Releases one 0x80-byte water-grid node.
///
/// # Original
/// - Ghidra: `FUN_800d3070` at `0x800D3070` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3070_FUN_800d3070.c`
///
/// # Arguments
/// * `node` - node to release, or `0`.
/// * `free` - positive when the storage must be freed.
///
/// # Returns
/// `node`, unchanged.
///
/// # Side effects
/// Releases `node + 0x40` then `node`, and frees the storage when `free > 0`.
///
/// # Safety
/// `node` must be null or a live node.
///
/// # Confidence
/// high - the guards and both sub-object releases are explicit.
pub unsafe fn water_node_release(node: usize, free: i32) -> usize {
    if node != 0 {
        let _ = platform::call2(FN_80169D20, (node + 0x40) as u32, 0xFFFF_FFFF);
        let _ = platform::call2(FN_80169D20, node as u32, 0xFFFF_FFFF);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, node as u32);
        }
    }
    node
}

// ---------------------------------------------------------------------------
// 0x800E0180..0x800E04B0 - save-image object (0x3FC0 bytes)
// ---------------------------------------------------------------------------

/// Allocates and constructs the save-image object.
///
/// In plain English: take the allocator lock, allocate `0x3FC0` bytes aligned
/// to `0x20` from the current heap, run [`save_image_construct`] when the
/// allocation succeeded, then restore the previous allocator.
///
/// # Original
/// - Ghidra: `FUN_800e0180` at `0x800E0180` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0180_FUN_800e0180.c`
/// - Caller: `0x800E48D0` (`dSys` start-up, see
///   [`crate::engine::dsys`]).
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing. The object is published by [`save_image_construct`].
///
/// # Side effects
/// Allocates 0x3FC0 bytes and constructs the save image.
///
/// # Safety
/// Must run once during engine start-up with a live default heap.
///
/// # Confidence
/// high - the allocation size, alignment and lock/unlock pair are literal.
pub unsafe fn save_image_create() {
    let previous = call0_r(FN_8016E4F0);
    let memory = platform::call2(FN_802B9230, 0x3FC0, 0x20);
    if memory != 0 {
        let _ = save_image_construct(memory as usize);
    }
    let _ = platform::call1(FN_8016E4F0, previous);
}

/// Constructs the save-image object in caller-provided storage.
///
/// In plain English: initialise the 0x6A0-byte header at `+0x20`, build the
/// two three-element arrays of 0x980-byte records at `+0x6C0` and `+0x2340`,
/// publish the object in [`G_SAVE_IMAGE`], then finalise both arrays and the
/// header.
///
/// # Original
/// - Ghidra: `FUN_800e01d0` at `0x800E01D0` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e01d0_FUN_800e01d0.c`
///
/// # Arguments
/// * `image` - storage for the 0x3FC0-byte save-image object.
///
/// # Returns
/// `image`, unchanged.
///
/// # Side effects
/// Clears and tags the header, runs two array constructors and two per-record
/// finalisers, and writes [`G_SAVE_IMAGE`].
///
/// # Safety
/// `image..image+0x3FC0` must be writable.
///
/// # Confidence
/// high - every offset, element size and count is explicit in the export.
pub unsafe fn save_image_construct(image: usize) -> usize {
    write_byte(image, 0x00, 0);
    let _ = platform::call1(FN_800CE570, (image + 0x20) as u32);

    let _ = platform::call5(
        FN_802DC9A0,
        (image + 0x6C0) as u32,
        FN_800CDEC0 as u32,
        FN_800E0290 as u32,
        0x980,
        3,
    );
    let _ = platform::call5(
        FN_802DC9A0,
        (image + 0x2340) as u32,
        FN_800CDEC0 as u32,
        FN_800E0290 as u32,
        0x980,
        3,
    );

    write_global_word(G_SAVE_IMAGE, image as u32);
    let _ = platform::call1(FN_800CE580, (image + 0x20) as u32);

    let mut index = 0i32;
    while index < 3 {
        let _ = save_image_record_a(image, index);
        let _ = platform::call0(FN_800CDED0);
        let _ = save_image_record_b(image, index);
        let _ = platform::call0(FN_800CDED0);
        index += 1;
    }
    image
}

/// Copies the on-disc save image into the live object and retags it.
///
/// In plain English: copy the 0x6A0-byte header and the two three-record
/// arrays from `0x8035DCC0` into `image`, then mark the header and every copied
/// record as initialised (`0x0E`).
///
/// # Original
/// - Ghidra: `FUN_800e02d0` at `0x800E02D0` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e02d0_FUN_800e02d0.c`
///
/// # Arguments
/// * `image` - live save-image object built by [`save_image_construct`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies 0x6A0 + 2 * 3 * 0x980 bytes and writes the `0x0E` tags.
///
/// # Safety
/// Both the source `0x8035DCC0` image and `image..image+0x3FC0` must be live.
///
/// # Confidence
/// high - the copy sizes, the three-iteration loop and the tag calls are
/// literal. The `0x802DCD74`/`0x802DCDC0` pair in the export is the CodeWarrior
/// register save/restore helper, not game logic, and is omitted.
pub unsafe fn save_image_refresh(image: usize) {
    let source = call0_r(FN_800CFBB0) as usize;

    let _ = platform::call3(FN_80004364, (image + 0x20) as u32, source as u32, 0x6A0);
    let _ = platform::call1(FN_800CE6C0, (image + 0x20) as u32);

    let mut source_a = source + 0x6A0;
    let mut source_b = source + 0x2320;
    let mut record_a = image + 0x6C0;
    let mut record_b = image + 0x2340;

    let mut index = 0;
    while index < 3 {
        let _ = platform::call3(FN_80004364, record_a as u32, source_a as u32, 0x980);
        let _ = platform::call1(FN_800CE020, record_a as u32);

        let _ = platform::call3(FN_80004364, record_b as u32, source_b as u32, 0x980);
        let _ = platform::call1(FN_800CE020, record_b as u32);

        index += 1;
        record_a += 0x980;
        source_b += 0x980;
        record_b += 0x980;
        source_a += 0x980;
    }
}

/// Returns the address of save record A for a slot.
///
/// In plain English: `slot` selects one of three 0x980-byte records in the
/// array at `+0x6C0`; `-1` selects the slot stored in the object's `+0x26`
/// byte.
///
/// # Original
/// - Ghidra: `FUN_800e0380` at `0x800E0380` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0380_FUN_800e0380.c`
///
/// # Arguments
/// * `image` - live save-image object.
/// * `slot` - `0..2`, or `-1` for the current slot.
///
/// # Returns
/// `image + slot * 0x980 + 0x6C0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `image + 0x26` must be readable when `slot == -1`; the returned address
/// must stay inside the object.
///
/// # Confidence
/// high - the whole body is one select and one multiply-add.
pub unsafe fn save_image_record_a(image: usize, slot: i32) -> usize {
    let index = if slot == -1 {
        read_byte(image, 0x26) as i8 as i32
    } else {
        slot
    };
    image + index as usize * 0x980 + 0x6C0
}

/// Returns the address of save record B for a slot.
///
/// Same selector as [`save_image_record_a`], but the array starts at `+0x2340`.
///
/// # Original
/// - Ghidra: `FUN_800e03b0` at `0x800E03B0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e03b0_FUN_800e03b0.c`
///
/// # Arguments
/// * `image` - live save-image object.
/// * `slot` - `0..2`, or `-1` for the current slot.
///
/// # Returns
/// `image + slot * 0x980 + 0x2340`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Same as [`save_image_record_a`].
///
/// # Confidence
/// high - the whole body is one select and one multiply-add.
pub unsafe fn save_image_record_b(image: usize, slot: i32) -> usize {
    let index = if slot == -1 {
        read_byte(image, 0x26) as i8 as i32
    } else {
        slot
    };
    image + index as usize * 0x980 + 0x2340
}

/// Reads the save-image "dirty" byte.
///
/// In plain English: when the object at [`G_SAVE_MANAGER`] has a zero word at
/// `+0x74`, clear the caller's byte; then return whatever the caller's byte now
/// holds.
///
/// # Original
/// - Ghidra: `FUN_800e0430` at `0x800E0430` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0430_FUN_800e0430.c`
///
/// # Arguments
/// * `flag` - address of a one-byte flag supplied by the caller.
///
/// # Returns
/// The byte at `flag`.
///
/// # Side effects
/// May write `0` to `*flag`.
///
/// # Safety
/// `flag` must be readable; it must be writable to be cleared.
///
/// # Confidence
/// high - the guard and the read-back are literal.
pub unsafe fn save_image_flag_get(flag: usize) -> u8 {
    let manager = read_global_word(G_SAVE_MANAGER) as usize;
    if read_word(manager, 0x74) == 0 {
        platform::write_u8(flag, 0);
    }
    platform::read_u8(flag)
}

/// Recomputes the save header's checksum word.
///
/// In plain English: hash the 0x698 bytes at `image + 0x24` and store the
/// result at `image + 0x6BC`.
///
/// # Original
/// - Ghidra: `FUN_800e0470` at `0x800E0470` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0470_FUN_800e0470.c`
///
/// # Arguments
/// * `image` - live save-image object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word at `+0x6BC`.
///
/// # Safety
/// `image + 0x24..+0x6C0` must be readable and `+0x6BC` writable.
///
/// # Confidence
/// high - the range, length and destination are literal.
pub unsafe fn save_image_update_header_checksum(image: usize) {
    let checksum = platform::call2(FN_8015F130, (image + 0x24) as u32, 0x698);
    write_word(image, 0x6BC, checksum);
}

/// Recomputes the header and all six record checksums.
///
/// In plain English: update the header checksum, then for each of the three
/// slots hash the 0x97C bytes of record A and record B (starting at the record
/// base) and store the result in the word that follows the payload.
///
/// # Original
/// - Ghidra: `FUN_800e04b0` at `0x800E04B0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e04b0_FUN_800e04b0.c`
///
/// # Arguments
/// * `image` - live save-image object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes seven checksum words (`+0x6BC` and six record tails).
///
/// # Safety
/// All seven record ranges must be readable and their tail words writable.
///
/// # Confidence
/// high - the loop count, record selectors and checksum lengths are explicit.
pub unsafe fn save_image_update_checksums(image: usize) {
    save_image_update_header_checksum(image);

    let mut slot = 0i32;
    while slot < 3 {
        let record_a = save_image_record_a(image, slot);
        let checksum_a = platform::call2(FN_8015F130, record_a as u32, 0x97C);
        write_word(record_a, 0x97C, checksum_a);

        let record_b = save_image_record_b(image, slot);
        let checksum_b = platform::call2(FN_8015F130, record_b as u32, 0x97C);
        write_word(record_b, 0x97C, checksum_b);

        slot += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x800C9B00 - display-object dirty flag
// ---------------------------------------------------------------------------

/// Marks the display object dirty when a named resource resolves.
///
/// In plain English: look the resource up through `0x802AC480`; when it exists
/// and its first nested word is non-zero, set the display object's `+0x3DA`
/// byte to `1`.
///
/// # Original
/// - Ghidra: `FUN_800c9b00` at `0x800C9B00` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800c9b00_FUN_800c9b00.c`
///
/// # Arguments
/// * `name` - resource name/descriptor passed straight to `0x802AC480`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `1` to `display + 0x3DA` when the resource is present.
///
/// # Safety
/// [`G_DISPLAY_OBJECT`] must point at a live display object and the resource
/// lookup must accept `name`.
///
/// # Confidence
/// medium - the branch and the store are explicit; what the resource is and
/// why it gates the dirty flag is not established.
pub unsafe fn display_object_mark_dirty(name: u32) {
    let display = read_global_word(G_DISPLAY_OBJECT) as usize;
    let resource = platform::call2(FN_802AC480, name, W_80427C08 as u32);
    if resource != 0 {
        let nested = platform::read_u32(resource as usize + 4);
        if platform::read_u32(resource as usize + nested as usize) != 0 {
            write_byte(display, 0x3DA, 1);
        }
    }
}

// ===========================================================================
// Wave 26 - water actor, cursor value and save-banner helpers
// (0x800D2270..0x800D4660, 0x800E0070..0x800E0450)
// ===========================================================================
//
// This wave completes the remaining readable bodies around the wave-20 water
// pool and save image:
//
// * `0x800D2270` / `0x800D2510` are the cursor value/target helpers that sit
//   between the pause cursor ([`cursor_acquire_target`]) and the water pool.
// * `0x800D2AC0..0x800D4660` is the `Wm_mr_water*` actor: its spawn call, the
//   per-element update, the actor constructor/destructor, its model load and
//   the model-node/animation helpers. The element stride (0x13C) and the
//   water-effect pool already translated in wave 20 pin this cluster down.
// * `0x800E0070..0x800E0450` are the save-banner slot visibility helper, the
//   already-registered save-record destructor and the save-manager busy test.
//
// The `0x802DCDxx` calls that Ghidra prints around some of these bodies are
// CodeWarrior `__savegprlr_*` / `__restgprlr_*` prologue helpers, not game
// logic (see `docs/COVERAGE_WAVE16_ENGINE.md`). They are omitted; where the
// export loses the `this`/returned pointer because of them, the Rust takes the
// object as an explicit argument and says so in the function doc.

// ---------------------------------------------------------------------------
// Wave-26 constants (all observed unless marked inferred/unknown)
// ---------------------------------------------------------------------------

/// `r2 - 0x6680` -> `0x8042CA00`: zero, used as the cursor table terminator
/// and the accumulator seed in [`cursor_position_sum`].
///
/// `// unknown:` the numeric value is read from memory, not decoded here.
const F32_SDA2_6680: usize = SDA2_BASE - 0x6680;

/// `r2 - 0x6658` -> `0x8042CA28`: vertical offset subtracted from the water
/// effect's `+0x11C` before the spawn lookup in [`water_effect_update`].
const F32_SDA2_6658: usize = SDA2_BASE - 0x6658;

/// `r2 - 0x6648` -> `0x8042CA38`: position argument passed twice by
/// [`water_actor_update_model_position`] and one animation parameter.
const F32_SDA2_6648: usize = SDA2_BASE - 0x6648;

/// `r2 - 0x664C` -> `0x8042CA34`: word copied four times into the actor's
/// `+0x784..+0x790` range by [`water_actor_construct`].
///
/// The same SDA2 word is used as a `float` by the neighbouring water code.
const F32_SDA2_664C: usize = SDA2_BASE - 0x664C;

/// `r13 - 0x5648` -> `0x8042A038`: archive/resource manager. Its `+4` word is
/// the archive passed to `0x800DF180` by [`water_actor_load_models`].
const G_ARCHIVE_5648: usize = SDA_BASE - 0x5648;

/// `r13 - 0x79F0` -> `0x80427C90`: resource name passed to `0x8023A200` by
/// [`water_actor_update_model_position`].
const W_80427C90: usize = SDA_BASE - 0x79F0;

/// `r13 - 0x7A00` -> `0x80427C80`: resource name passed to `0x800DF180` as the
/// second model of [`water_actor_load_models`].
const W_80427C80: usize = SDA_BASE - 0x7A00;

/// `0x80317AEC`: pointer to the `"Wm_mr_watersplash"` effect name.
const PTR_WATERSPLASH_NAME: usize = 0x8031_7AEC;

/// `0x80317ADC`: four pointers to the `"Wm_mr_waterwave_out"` effect names.
///
/// `// inferred:` the four entries are the wave variants selected by the
/// spawn's `kind` argument; only the base address is proven by the export.
const TABLE_WATERWAVE_NAMES: usize = 0x8031_7ADC;

/// `0x80317B08`: `"g3d_model.brres"`, printed by Ghidra as
/// `s__Pg3d_model_brres_80317b05 + 3`.
const S_G3D_MODEL_BRRES: usize = 0x8031_7B08;

/// `0x802F1250`: water-actor state table; each entry is 0x10 bytes: a name
/// pointer at `+0`, a word at `+4` and two floats at `+8`/`+0xC`.
///
/// The export does not print the table length.
const TABLE_802F1250: usize = 0x802F_1250;

/// `0x802F8FF0`: descriptor passed to vtable slot `+8` by
/// [`save_banner_set_slot_visible`]. `// unknown:` contents.
const DESC_802F8FF0: usize = 0x802F_8FF0;

/// `0x80317D30`: vtable installed by [`water_actor_construct`].
const VTABLE_80317D30: u32 = 0x8031_7D30;

/// `0x80329958`: vtable of the two embedded `+0x218` / `+0x340` sub-objects,
/// and of the two blocks released by [`water_actor_release`].
const VTABLE_80329958: u32 = 0x8032_9958;

/// `0x803295D8`: vtable written before each `0x8016A5E0` construction.
const VTABLE_803295D8: u32 = 0x8032_95D8;

/// `0x80329548`: vtable written after each `0x8016A5E0` construction.
const VTABLE_80329548: u32 = 0x8032_9548;

/// `0x80329518`: alternate vtable used for the `+0x744` sub-object.
const VTABLE_80329518: u32 = 0x8032_9518;

/// `0x80317E08`: table stored at `+0x794` for the four 0x80-byte nodes.
const VTABLE_80317E08: u32 = 0x8031_7E08;

/// `0x80317DF0`: table stored at `+0x79C`.
const VTABLE_80317DF0: u32 = 0x8031_7DF0;

/// `0x800D3030`: 0x80-byte node constructor, translated as
/// [`water_node_construct`].
const FN_800D3030: usize = 0x800D_3030;

/// `0x800D3070`: 0x80-byte node destructor, translated as
/// [`water_node_release`].
const FN_800D3070: usize = 0x800D_3070;

/// `0x800D5330`: water-actor base constructor `(actor, mode_byte)`.
const FN_800D5330: usize = 0x800D_5330;

/// `0x800D54E0`: water-actor embedded-model release `(actor, free)`.
const FN_800D54E0: usize = 0x800D_54E0;

/// `0x8016A5E0`: constructs one embedded animation sub-object.
const FN_8016A5E0: usize = 0x8016_A5E0;

/// `0x8016A000(parent, vtable_source)`: binds/releases one sub-object.
const FN_8016A000: usize = 0x8016_A000;

/// `0x80165E70(object, -1)`: releases one animation handle.
const FN_80165E70: usize = 0x8016_5E70;

/// `0x80166A70(object, -1)`: releases one animation handle.
const FN_80166A70: usize = 0x8016_6A70;

/// `0x8016D4B0(object)`: releases one base sub-object.
const FN_8016D4B0: usize = 0x8016_D4B0;

/// `0x8008FAE0`: six-argument effect spawn used for the wave variants.
const FN_8008FAE0: usize = 0x8008_FAE0;

/// `0x80075270(x, y, kind, out)`: position lookup used by
/// [`water_effect_update`].
const FN_80075270: usize = 0x8007_5270;

/// `0x800DF180(archive, name, extension)`: loads one model and returns it.
const FN_800DF180: usize = 0x800D_F180;

/// `0x8023A200(model, name)`: resolves one animation handle.
const FN_8023A200: usize = 0x8023_A200;

/// `0x8023B2A0(container, index)`: indexes the actor's record table.
const FN_8023B2A0: usize = 0x8023_B2A0;

/// `0x8025C960(out, manager, slot)`: builds the 12-byte slot key.
const FN_8025C960: usize = 0x8025_C960;

/// `0x80166BD0(node, owner, name, 1, arg)`: applies one animation.
const FN_80166BD0: usize = 0x8016_6BD0;

/// `0x80166F10(value, node, 1)`: sets one animation value.
const FN_80166F10: usize = 0x8016_6F10;

/// `0x80166EF0(value, node, 1)`: sets one animation value.
const FN_80166EF0: usize = 0x8016_6EF0;

// ---------------------------------------------------------------------------
// Wave-26 call helpers
// ---------------------------------------------------------------------------

/// Calls a vtable slot with `this` plus three integer arguments.
#[inline]
unsafe fn vcall3(this_arg: usize, slot: usize, a: u32, b: u32, c: u32) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call4(entry, this_arg as u32, a, b, c)
}

/// Calls `func(f64, f64, u32, usize) -> u32`.
///
/// Used for `0x80075270`, whose two leading parameters are printed as
/// `(double)` casts in the export. Host `extern "C"` keeps the same argument
/// order; the reconstruction is semantic, not a bit-exact ABI replay.
#[inline]
unsafe fn call_f64_f64_u32_usize(addr: usize, a: f64, b: f64, c: u32, d: usize) -> u32 {
    let f: extern "C" fn(f64, f64, u32, usize) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d)
}

/// Calls `func(f64, u32, u32) -> u32`.
#[inline]
unsafe fn call_f64_u32_u32(addr: usize, a: f64, b: u32, c: u32) -> u32 {
    let f: extern "C" fn(f64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c)
}

// ---------------------------------------------------------------------------
// 0x800D2270..0x800D2510 - cursor value helpers
// ---------------------------------------------------------------------------

/// Returns the value of the object under the pause cursor.
///
/// In plain English: a target in mode 1..3 is a table of floats and a parallel
/// table of signed halfwords. Walk backwards from the current index while the
/// float entry is zero; at index 0 read the first halfword, otherwise
/// interpolate between `halfword[index - 1]` and `halfword[index]` with the
/// float entry as the scale and `cursor + 0x20` as the weight. A target in
/// mode 0 asks its virtual slot `+0xC` instead.
///
/// # Original
/// - Ghidra: `FUN_800d2270` at `0x800D2270` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2270_FUN_800d2270.c`
///
/// # Arguments
/// * `cursor` - pause cursor. `+0x0C` is the target object, `+0x2E` the table
///   index and `+0x20` the interpolation weight.
///
/// # Returns
/// The signed value, or `0` when there is no target or the mode is unknown.
///
/// # Side effects
/// None of its own; the mode-0 virtual call may have effects.
///
/// # Safety
/// The target, its `+0x2C` float table, its `+0x30` halfword table and the
/// cursor's `+0x20` float must all be live. The original bounds the index only
/// by the backwards walk, not by a table length.
///
/// # Confidence
/// medium - every branch and arithmetic step is explicit; the table meaning
/// and the target's class are `// inferred:` from the cursor/water call sites.
pub unsafe fn cursor_target_value(cursor: usize) -> i32 {
    let target = read_word(cursor, 0x0C) as usize;
    if target == 0 {
        return 0;
    }

    let mode = read_byte(target, 0x20);
    if mode.wrapping_sub(1) < 3 {
        let table = read_word(target, 0x2C) as usize;
        let mut index = read_byte(cursor, 0x2E) as u32;
        let zero = read_const_f32(F32_SDA2_6680);
        let mut value = platform::read_f32(table + index as usize * 4);
        while value == zero && index != 0 {
            index -= 1;
            value = platform::read_f32(table + index as usize * 4);
        }

        if index == 0 {
            return read_half(target, 0x30) as i16 as i32;
        }

        let halfwords = read_word(target, 0x30) as usize;
        let previous = read_half(halfwords, (index - 1) as usize * 2) as i16 as i32;
        let current = read_half(halfwords, index as usize * 2) as i16 as i32;
        let scale = read_float(cursor, 0x20)
            / platform::read_f32(table + index as usize * 4);
        let delta = (current - previous) as f64 * scale as f64;
        return (delta as f32 + previous as f32) as i16 as i32;
    }

    if mode == 0 {
        return vcall0(target, 0x0C) as i32;
    }
    0
}

/// Sums the float table under the pause cursor into `cursor + 0x14`.
///
/// In plain English: add `count` floats starting at the cursor's `+0x2C`
/// table pointer, where `count` is the byte at `+0x3C`, and store the total.
///
/// # Original
/// - Ghidra: `FUN_800d2510` at `0x800D2510` (252 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2510_FUN_800d2510.c`
///
/// # Arguments
/// * `cursor` - object whose `+0x2C` word is the float array and `+0x3C` byte
///   is its length.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one float at `cursor + 0x14`.
///
/// # Safety
/// `cursor + 0x2C..+0x3C` and `count * 4` bytes at that pointer must be
/// readable; `cursor + 0x14` must be writable.
///
/// # Confidence
/// high - the accumulator seed, loop bound and final store are literal. The
/// export's eight-way unroll is the compiler's, and it keeps one running sum.
pub unsafe fn cursor_position_sum(cursor: usize) {
    let base = read_word(cursor, 0x2C) as usize;
    let count = read_byte(cursor, 0x3C) as usize;
    let mut sum = read_const_f32(F32_SDA2_6680);
    let mut index = 0usize;
    while index < count {
        sum += platform::read_f32(base + index * 4);
        index += 1;
    }
    write_float(cursor, 0x14, sum);
}

// ---------------------------------------------------------------------------
// 0x800D2AC0..0x800D4660 - `Wm_mr_water*` actor
// ---------------------------------------------------------------------------

/// Starts one water effect in a free pool element.
///
/// In plain English: when `element + 0x138` is zero, remember the effect kind
/// at `+0x134`, mark the element used, copy the three position words from
/// `position`, store the scale in the three float slots `+0x124..+0x12C`,
/// store the extra byte at `+0x130`, then spawn the named effect through the
/// element's base-object vtable (kind 4) or `0x8008FAE0` (other kinds).
///
/// # Original
/// - Ghidra: `FUN_800d2ac0` at `0x800D2AC0` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2ac0_FUN_800d2ac0.c`
///
/// # Arguments
/// * `scale` - effect scale, printed as a `double` and stored as `f32`.
/// * `element` - one 0x13C-byte water-effect element.
/// * `kind` - effect variant; `4` is `"Wm_mr_watersplash"`, the other values
///   index [`TABLE_WATERWAVE_NAMES`].
/// * `position` - three words copied to `+0x118..+0x120`.
/// * `param` - byte stored at `+0x130` and forwarded to the lookup.
///
/// # Returns
/// `1` when the element was free and the effect started, `0` when it was busy.
///
/// # Side effects
/// Writes `+0x118..+0x138` and runs the effect spawn.
///
/// # Safety
/// `element..element+0x13C` must be writable, `position` must have three
/// readable words, and `kind` must index the name table when it is not 4.
///
/// # Confidence
/// high - every guard, field, table index and spawn argument is explicit. The
/// `double` parameter is cast to `f32` exactly as the export shows.
pub unsafe fn water_effect_spawn(
    scale: f64,
    element: usize,
    kind: i32,
    position: usize,
    param: u8,
) -> u32 {
    if read_word(element, 0x138) != 0 {
        return 0;
    }

    write_word(element, 0x134, kind as u32);
    write_word(element, 0x138, 1);
    write_word(element, 0x118, read_word(position, 0));
    write_word(element, 0x11C, read_word(position, 4));
    write_word(element, 0x120, read_word(position, 8));
    let scale_f32 = scale as f32;
    write_float(element, 0x124, scale_f32);
    write_float(element, 0x128, scale_f32);
    write_float(element, 0x12C, scale_f32);
    write_byte(element, 0x130, param);

    let base = element + 4;
    if kind == 4 {
        let name = read_global_word(PTR_WATERSPLASH_NAME);
        let vtable = read_word(base, 0) as usize;
        let entry = read_word(vtable, 0xA0) as usize;
        let _ = platform::call6(
            entry,
            base as u32,
            name,
            0,
            (element + 0x118) as u32,
            0,
            (element + 0x124) as u32,
        );
    } else {
        let name = read_global_word(TABLE_WATERWAVE_NAMES + kind as usize * 4);
        let _ = platform::call6(
            FN_8008FAE0,
            base as u32,
            name,
            0,
            (element + 0x118) as u32,
            0,
            (element + 0x124) as u32,
        );
    }
    1
}

/// Advances one active water effect.
///
/// In plain English: when `+0x138` is `1`, look up the effect's ground height
/// through `0x80075270`; on a hit store the returned height at `+0x11C`.
/// Then call the base object's vtable slot `+0xB0`; when it reports finished,
/// clear `+0x138` so the pool can reuse the element.
///
/// # Original
/// - Ghidra: `FUN_800d2b90` at `0x800D2B90` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2b90_FUN_800d2b90.c`
///
/// # Arguments
/// * `element` - one live water-effect element (`+4` is the base object).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May write `+0x11C`, always runs vtable `+0xB0`, and may clear `+0x138`.
///
/// # Safety
/// The element and its base object must be live; the lookup's `kind` byte at
/// `+0x130` selects the collision layer.
///
/// # Confidence
/// medium - the flow and both stores are explicit; the lookup and the vtable
/// slot are called by address and their exact semantics are `// unknown:`.
pub unsafe fn water_effect_update(element: usize) {
    if read_word(element, 0x138) != 1 {
        return;
    }

    let mut out = [0u32; 5];
    let found = call_f64_f64_u32_usize(
        FN_80075270,
        read_float(element, 0x118) as f64,
        read_float(element, 0x11C) as f64 - read_const_f32(F32_SDA2_6658) as f64,
        read_byte(element, 0x130) as u32,
        out.as_mut_ptr() as usize,
    );
    if found == 1 {
        write_word(element, 0x11C, out[0]);
    }

    let finished = vcall3(
        element + 4,
        0xB0,
        (element + 0x118) as u32,
        0,
        (element + 0x124) as u32,
    );
    if finished == 0 {
        write_word(element, 0x138, 0);
    }
}

/// Constructs the `Wm_mr_water*` actor base.
///
/// In plain English: run the base constructor with `mode`, install the actor
/// vtable, then build the two embedded animation sub-objects at `+0x218` and
/// `+0x340`, the ten 0x80-byte node array at `+0x468`, the six handled
/// animation sub-objects at `+0x668..+0x750`, and finally the two adapter
/// tables at `+0x794` / `+0x79C`. The fields at `+0x784..+0x790` receive the
/// same SDA2 word and `+0x782` receives `0x400`.
///
/// # Original
/// - Ghidra: `FUN_800d2dd0` at `0x800D2DD0` (472 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2dd0_FUN_800d2dd0.c`
///
/// # Arguments
/// * `actor` - storage for the actor base.
/// * `base_mode` - byte forwarded to `0x800D5330` and stored at `+0x151` by
///   that base constructor. `// inferred:` the byte selects the character
///   model table; the export's save-register prologue hides the argument.
///
/// # Returns
/// `actor`, unchanged.
///
/// # Side effects
/// Initialises the whole object, runs one array constructor and seven
/// embedded constructors.
///
/// # Safety
/// `actor` must be writable and large enough for the full subclass (the one
/// adapted class in the export reaches beyond `+0x7A0`).
///
/// # Confidence
/// medium - all stores and calls are literal, but the `this` pointer and the
/// `base_mode` argument are recovered from the call sites because
/// `0x802DCD74`/`0x802DCDC0` hide them in the export.
pub unsafe fn water_actor_construct(actor: usize, base_mode: u8) -> usize {
    let _ = platform::call2(FN_800D5330, actor as u32, base_mode as u32);
    write_word(actor, 0x000, VTABLE_80317D30);

    write_word(actor, 0x20C, 0);
    write_word(actor, 0x210, 0);
    write_word(actor, 0x214, 0);
    let _ = platform::call1(FN_802D7AA0, (actor + 0x218) as u32);
    write_word(actor, 0x218, VTABLE_80329958);
    write_word(actor, 0x32C, 0);
    write_word(actor, 0x330, 0);
    write_byte(actor, 0x334, 0);
    write_byte(actor, 0x335, 0);
    write_word(actor, 0x338, 0);
    write_word(actor, 0x33C, 0);
    let _ = platform::call1(FN_802D7AA0, (actor + 0x340) as u32);
    write_word(actor, 0x340, VTABLE_80329958);

    write_word(actor, 0x454, 0);
    write_word(actor, 0x458, 0);
    write_byte(actor, 0x45C, 0);
    write_byte(actor, 0x45D, 0);
    write_word(actor, 0x460, 0);
    write_word(actor, 0x464, 0);

    let _ = platform::call5(
        FN_802DC9A0,
        (actor + 0x468) as u32,
        FN_800D3030 as u32,
        FN_800D3070 as u32,
        0x80,
        4,
    );

    write_word(actor, 0x66C, 0);
    write_word(actor, 0x668, VTABLE_803295D8);
    write_word(actor, 0x670, 0);
    let _ = platform::call1(FN_8016A5E0, (actor + 0x674) as u32);
    write_word(actor, 0x690, 0);
    write_word(actor, 0x668, VTABLE_80329548);

    write_word(actor, 0x694, VTABLE_803295D8);
    write_word(actor, 0x698, 0);
    write_word(actor, 0x69C, 0);
    let _ = platform::call1(FN_8016A5E0, (actor + 0x6A0) as u32);
    write_word(actor, 0x6BC, 0);
    write_word(actor, 0x694, VTABLE_80329518);

    write_word(actor, 0x6C0, VTABLE_803295D8);
    write_word(actor, 0x6C4, 0);
    write_word(actor, 0x6C8, 0);
    let _ = platform::call1(FN_8016A5E0, (actor + 0x6CC) as u32);
    write_word(actor, 0x6C0, VTABLE_80329548);

    write_word(actor, 0x6E8, 0);
    write_word(actor, 0x6EC, VTABLE_803295D8);
    write_word(actor, 0x6F0, 0);
    write_word(actor, 0x6F4, 0);
    let _ = platform::call1(FN_8016A5E0, (actor + 0x6F8) as u32);
    write_word(actor, 0x6EC, VTABLE_80329548);

    write_word(actor, 0x714, 0);
    write_word(actor, 0x718, VTABLE_803295D8);
    write_word(actor, 0x71C, 0);
    write_word(actor, 0x720, 0);
    let _ = platform::call1(FN_8016A5E0, (actor + 0x724) as u32);
    write_word(actor, 0x718, VTABLE_80329548);

    write_word(actor, 0x740, 0);
    write_word(actor, 0x744, VTABLE_803295D8);
    write_word(actor, 0x748, 0);
    write_word(actor, 0x74C, 0);
    let _ = platform::call1(FN_8016A5E0, (actor + 0x750) as u32);
    write_word(actor, 0x744, VTABLE_80329518);

    let constant = read_global_word(F32_SDA2_664C);
    write_word(actor, 0x76C, 0);
    write_word(actor, 0x770, 0);
    write_word(actor, 0x774, 0);
    write_word(actor, 0x77C, 0);
    write_half(actor, 0x780, 0);
    write_half(actor, 0x782, 0x400);
    write_word(actor, 0x784, constant);
    write_word(actor, 0x788, constant);
    write_word(actor, 0x78C, constant);
    write_word(actor, 0x790, constant);
    write_word(actor, 0x794, VTABLE_80317E08);
    write_word(actor, 0x798, actor as u32);
    write_word(actor, 0x79C, VTABLE_80317DF0);
    write_word(actor, 0x7A0, actor as u32);
    actor
}

/// Releases an optional object when it is present and `free` is positive.
///
/// In plain English: if `object` is non-null and `free > 0`, free its storage;
/// either way return the pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_800d2ff0` at `0x800D2FF0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2ff0_FUN_800d2ff0.c`
///
/// # Arguments
/// * `object` - object to free, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May call `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or a live heap allocation.
///
/// # Confidence
/// high - the two guards and the return are literal. The class is unknown, so
/// the function keeps its address name.
pub unsafe fn fn_800d2ff0(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Releases the `Wm_mr_water*` actor base.
///
/// In plain English: release the six handled animation sub-objects, destroy
/// the ten 0x80-byte node array, destroy the two embedded sub-objects at
/// `+0x340` and `+0x218`, run the embedded-model release, and free the actor
/// when `free > 0`.
///
/// # Original
/// - Ghidra: `FUN_800d30e0` at `0x800D30E0` (292 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d30e0_FUN_800d30e0.c`
///
/// # Arguments
/// * `actor` - live actor base, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `actor`, unchanged.
///
/// # Side effects
/// Releases every embedded object and possibly the actor storage.
///
/// # Safety
/// `actor` must be null or the object built by [`water_actor_construct`].
///
/// # Confidence
/// high - every offset, destructor argument and guard is explicit.
pub unsafe fn water_actor_release(actor: usize, free: i32) -> usize {
    if actor != 0 {
        let _ = platform::call2(FN_80165E70, (actor + 0x744) as u32, 0xFFFF_FFFF);
        let _ = platform::call2(FN_80166A70, (actor + 0x718) as u32, 0xFFFF_FFFF);
        let _ = platform::call2(FN_80166A70, (actor + 0x6EC) as u32, 0xFFFF_FFFF);
        let _ = platform::call2(FN_80166A70, (actor + 0x6C0) as u32, 0xFFFF_FFFF);
        let _ = platform::call2(FN_80165E70, (actor + 0x694) as u32, 0xFFFF_FFFF);
        let _ = platform::call2(FN_80166A70, (actor + 0x668) as u32, 0xFFFF_FFFF);
        let _ = platform::call4(
            FN_802DCA98,
            (actor + 0x468) as u32,
            FN_800D3070 as u32,
            0x80,
            4,
        );

        let sub_a = actor + 0x340;
        write_word(sub_a, 0, VTABLE_80329958);
        let _ = platform::call1(FN_8016D4B0, sub_a as u32);
        let _ = platform::call2(FN_802D7B20, sub_a as u32, 0);

        let sub_b = actor + 0x218;
        write_word(sub_b, 0, VTABLE_80329958);
        let _ = platform::call1(FN_8016D4B0, sub_b as u32);
        let _ = platform::call2(FN_802D7B20, sub_b as u32, 0);

        let _ = platform::call2(FN_800D54E0, actor as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, actor as u32);
        }
    }
    actor
}

/// Updates the actor's four 0x80-byte nodes and the shared sub-object.
///
/// In plain English: bind each of the four nodes at `+0x468` to the table at
/// `+0x794`, bind the object at `+0x5A8` to `+0x79C`, then refresh the model
/// position.
///
/// # Original
/// - Ghidra: `FUN_800d3280` at `0x800D3280` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3280_FUN_800d3280.c`
///
/// # Arguments
/// * `actor` - live water actor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs five bind calls and [`water_actor_update_model_position`].
///
/// # Safety
/// All five sub-objects must be constructed.
///
/// # Confidence
/// high - the loop bound, stride and the two adapters are literal.
pub unsafe fn water_actor_update(actor: usize) {
    let mut node = actor + 0x468;
    let mut index = 0;
    while index < 4 {
        let _ = platform::call2(FN_8016A000, node as u32, (actor + 0x794) as u32);
        index += 1;
        node += 0x80;
    }
    let _ = platform::call2(FN_8016A000, (actor + 0x5A8) as u32, (actor + 0x79C) as u32);
    water_actor_update_model_position(actor);
}

/// Loads the actor's three `g3d_model.brres` models.
///
/// In plain English: load the model named by the actor's record, the shared
/// name at `0x80427C80`, and the second record from the same archive; store
/// the three handles at `+0x20C`, `+0x210` and `+0x214`.
///
/// # Original
/// - Ghidra: `FUN_800d32f0` at `0x800D32F0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d32f0_FUN_800d32f0.c`
///
/// # Arguments
/// * `actor` - live water actor; `+0x778` holds the record table.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Loads three models and writes three handle words.
///
/// # Safety
/// The archive global, the actor's record table and all three names must be
/// live. The original does not null-check them.
///
/// # Confidence
/// high - all three calls, the shared extension and the store offsets are
/// explicit. `// unknown:` the model names are only known by address.
pub unsafe fn water_actor_load_models(actor: usize) {
    let archive = read_global_word(G_ARCHIVE_5648) + 4;
    let records = read_word(actor, 0x778) as usize;

    let first = platform::call3(
        FN_800DF180,
        archive,
        read_word(records, 0),
        S_G3D_MODEL_BRRES as u32,
    );
    write_word(actor, 0x20C, first);

    let second = platform::call3(
        FN_800DF180,
        archive,
        W_80427C80 as u32,
        S_G3D_MODEL_BRRES as u32,
    );
    write_word(actor, 0x210, second);

    let third = platform::call3(
        FN_800DF180,
        archive,
        read_word(records, 4),
        S_G3D_MODEL_BRRES as u32,
    );
    write_word(actor, 0x214, third);
}

/// Refreshes the actor's model position from its shared name handle.
///
/// In plain English: resolve the name at `0x80427C90` against the model at
/// `+0x20C`, then apply that handle with the SDA2 position value used twice
/// and the extra argument `0`.
///
/// # Original
/// - Ghidra: `FUN_800d3ad0` at `0x800D3AD0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3ad0_FUN_800d3ad0.c`
///
/// # Arguments
/// * `actor` - live water actor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Resolves one animation handle and applies it to the current model node.
///
/// # Safety
/// The model at `+0x20C` must be loaded.
///
/// # Confidence
/// high - the call order, the two identical float arguments and the trailing
/// `0` are literal. `// unknown:` the name and the float constant.
pub unsafe fn water_actor_update_model_position(actor: usize) {
    let handle = platform::call2(FN_8023A200, (actor + 0x20C) as u32, W_80427C90 as u32);
    let mut local = [handle];
    let value = read_const_f32(F32_SDA2_6648) as f64;
    water_actor_apply_animation(value, value, actor, local.as_mut_ptr(), 0);
}

/// Applies one animation handle to the actor's current model node.
///
/// In plain English: copy the handle, bind it to the node selected by the
/// actor's `+0x770` mode, set the two animation values, then call the node's
/// vtable slot `+0x18` with the handle again.
///
/// # Original
/// - Ghidra: `FUN_800d3b20` at `0x800D3B20` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3b20_FUN_800d3b20.c`
///
/// # Arguments
/// * `value_a`, `value_b` - animation values, printed as `double`.
/// * `actor` - live water actor.
/// * `handle` - address of the one-word handle to bind.
/// * `arg` - fifth argument forwarded to `0x80166BD0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Applies three animation calls and one virtual call.
///
/// # Safety
/// `handle` must point at a readable word; the node selected by `+0x770` must
/// be constructed.
///
/// # Confidence
/// medium - the call order and the node expression are literal; the export
/// drops the receiver of the first `0x800D3CB0` call, reconstructed as
/// `actor`.
pub unsafe fn water_actor_apply_animation(
    value_a: f64,
    value_b: f64,
    actor: usize,
    handle: *mut u32,
    arg: u32,
) {
    let mut local = [platform::read_u32(handle as usize)];
    let node = water_actor_model_node(actor) as u32;
    let owner = (actor + read_word(actor, 0x770) as usize * 0x80 + 0x4A8) as u32;
    let _ = platform::call5(
        FN_80166BD0,
        node,
        owner,
        local.as_mut_ptr() as u32,
        1,
        arg,
    );
    let node_a = water_actor_model_node(actor) as u32;
    let _ = call_f64_u32_u32(FN_80166F10, value_a, node_a, 1);
    let node_b = water_actor_model_node(actor) as u32;
    let _ = call_f64_u32_u32(FN_80166EF0, value_b, node_b, 1);
    let receiver = actor + read_word(actor, 0x770) as usize * 0x80 + 0x4A8;
    let handle_for_vcall = water_actor_model_node(actor);
    let _ = vcall1(receiver, 0x18, handle_for_vcall as u32);
}

/// Changes the actor's animation state.
///
/// In plain English: when the requested state differs from `+0x170`, store
/// it, resolve the state's name from the 0x10-byte table at `0x802F1250`,
/// apply the state's two floats and extra word, and clear `+0x174` when the
/// new state is 0.
///
/// # Original
/// - Ghidra: `FUN_800d3bf0` at `0x800D3BF0` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3bf0_FUN_800d3bf0.c`
///
/// # Arguments
/// * `actor` - live water actor.
/// * `state` - new state index; the export does not bound it.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x170`, possibly `+0x174`, and applies one animation.
///
/// # Safety
/// `actor + 0x20C` must be a loaded model and `state` must index the table.
///
/// # Confidence
/// high - the equality guard, table stride, four table fields and both stores
/// are explicit. `// inferred:` the table entries are animation states.
pub unsafe fn water_actor_set_state(actor: usize, state: i32) {
    if read_word(actor, 0x170) as i32 == state {
        return;
    }

    write_word(actor, 0x170, state as u32);
    // observed: the C multiplies the signed state by 0x10 without a bounds
    // check; wrapping keeps that behaviour for out-of-range callers.
    let offset = state.wrapping_mul(0x10) as usize;
    let name = read_global_word(TABLE_802F1250 + offset);
    let handle = platform::call2(FN_8023A200, (actor + 0x20C) as u32, name);
    let mut local = [handle];
    let _ = water_actor_apply_animation(
        read_const_f32(TABLE_802F1250 + offset + 8) as f64,
        read_const_f32(TABLE_802F1250 + offset + 0xC) as f64,
        actor,
        local.as_mut_ptr(),
        read_global_word(TABLE_802F1250 + offset + 4),
    );
    if read_word(actor, 0x170) == 0 {
        write_word(actor, 0x174, 0);
    }
}

/// Returns the animation node selected by the actor's mode.
///
/// In plain English: mode 2 selects `+0x6EC`, mode 3 selects `+0x718`, and
/// every other value (including 0 and 1) selects `+0x6C0`.
///
/// # Original
/// - Ghidra: `FUN_800d3cb0` at `0x800D3CB0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3cb0_FUN_800d3cb0.c`
///
/// # Arguments
/// * `actor` - live water actor.
///
/// # Returns
/// Address of the selected embedded node.
///
/// # Side effects
/// None.
///
/// # Safety
/// `actor` must be the object built by [`water_actor_construct`].
///
/// # Confidence
/// high - two comparisons and three fixed offsets.
pub unsafe fn water_actor_model_node(actor: usize) -> usize {
    if read_word(actor, 0x770) == 2 {
        return actor + 0x6EC;
    }
    if read_word(actor, 0x770) == 3 {
        return actor + 0x718;
    }
    actor + 0x6C0
}

/// Saves the current actor mode and installs a new one.
///
/// In plain English: copy `+0x770` to `+0x774`, then write the new mode to
/// `+0x770`.
///
/// # Original
/// - Ghidra: `FUN_800d4660` at `0x800D4660` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4660_FUN_800d4660.c`
///
/// # Arguments
/// * `actor` - live water actor.
/// * `mode` - new mode word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two word stores.
///
/// # Safety
/// `actor + 0x770` and `+0x774` must be writable.
///
/// # Confidence
/// high - two literal stores.
pub unsafe fn water_actor_set_mode(actor: usize, mode: u32) {
    write_word(actor, 0x774, read_word(actor, 0x770));
    write_word(actor, 0x770, mode);
}

// ---------------------------------------------------------------------------
// 0x800E0070..0x800E0450 - save-banner / save-manager helpers
// ---------------------------------------------------------------------------

/// Shows or hides one save-banner slot.
///
/// In plain English: resolve the banner's slot manager through vtable slot
/// `+8` and the descriptor at `0x802F8FF0`; when `slot >= 0`, build the
/// 12-byte slot key, look the slot up twice, and set or clear bit `0x100` in
/// the record's `+0x14` flags.
///
/// # Original
/// - Ghidra: `FUN_800e0070` at `0x800E0070` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0070_FUN_800e0070.c`
///
/// # Arguments
/// * `banner` - save-banner object; `+4` points at the slot manager.
/// * `slot` - slot index, or a negative value to do nothing.
/// * `visible` - `0` clears the `0x100` bit, any other value sets it.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs up to three lookups and writes one flags word.
///
/// # Safety
/// The manager and its `+0xE8` container must be live; the record returned by
/// `0x8023B2A0` must have a writable word at `+0x14`.
///
/// # Confidence
/// medium - the branches, key build, duplicate lookup and bit mask are
/// literal; the export prints the 12-byte key buffer as `auStack_1c`, which
/// the Rust models as an opaque byte array.
pub unsafe fn save_banner_set_slot_visible(banner: usize, slot: i32, visible: u32) {
    let mut descriptor = DESC_802F8FF0;
    let mut manager = read_word(banner, 4) as usize;
    let mut found = false;
    if manager != 0 {
        if vcall1(manager, 8, (&mut descriptor) as *mut usize as u32) != 0 {
            found = true;
        }
    }
    if !found {
        manager = 0;
    }

    if slot < 0 {
        return;
    }

    let mut key = [0u8; 12];
    let _ = platform::call3(
        FN_8025C960,
        key.as_mut_ptr() as u32,
        manager as u32,
        slot as u32,
    );

    let mut container_a = read_word(manager, 0xE8);
    let first = platform::call2(
        FN_8023B2A0,
        (&mut container_a) as *mut u32 as u32,
        slot as u32,
    );
    if first != 0 {
        let mut container_b = read_word(manager, 0xE8);
        let record = platform::call2(
            FN_8023B2A0,
            (&mut container_b) as *mut u32 as u32,
            slot as u32,
        );
        if record != 0 {
            let flags = read_word(record as usize, 0x14);
            if visible == 0 {
                write_word(record as usize, 0x14, flags & !0x100);
            } else {
                write_word(record as usize, 0x14, flags | 0x100);
            }
        }
    }
}

/// Releases one 0x980-byte save record.
///
/// In plain English: when the record is non-null and `free > 0`, release its
/// storage; either way return the record pointer.
///
/// # Original
/// - Ghidra: `FUN_800e0290` at `0x800E0290` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0290_FUN_800e0290.c`
///
/// # Arguments
/// * `record` - record to release, or `0`.
/// * `free` - positive when the storage must be freed.
///
/// # Returns
/// `record`, unchanged.
///
/// # Side effects
/// May call `0x802B9280(record)`.
///
/// # Safety
/// `record` must be null or one of the six live save records.
///
/// # Confidence
/// high - the two guards and the return are literal. The export omits the
/// release operand; the record pointer is the only possible argument.
pub unsafe fn save_record_release(record: usize, free: i32) -> usize {
    if record != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, record as u32);
    }
    record
}

/// Tests whether the save manager is busy.
///
/// In plain English: read the manager's `+0x74` word and return `1` when it is
/// non-zero, `0` when it is zero. The expression is the original's
/// `(-value | value) >> 31`, which is exactly "non-zero".
///
/// # Original
/// - Ghidra: `FUN_800e0450` at `0x800E0450` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e0450_FUN_800e0450.c`
///
/// # Arguments
/// None. The manager comes from [`G_SAVE_MANAGER`]. Callers in the banner
/// code pass the save-image pointer, but the decompiled body ignores it.
///
/// # Returns
/// `1` when `manager + 0x74` is non-zero, else `0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// [`G_SAVE_MANAGER`] must point at the live save manager.
///
/// # Confidence
/// high - one global load, one field load and the exact sign expression.
pub unsafe fn save_manager_busy() -> u32 {
    let manager = read_global_word(G_SAVE_MANAGER) as usize;
    let value = read_word(manager, 0x74);
    (value.wrapping_neg() | value) >> 31
}

// ---------------------------------------------------------------------------
// Wave 38: 0x800DB450..0x800DCDD0 - button triples and the four-slot menu
// state container (global at `r13 - 0x5658`, already [`G_PLAYER_SLOTS`])
// ---------------------------------------------------------------------------

/// `r13 - 0x5220` -> `0x80429E60`: view/route manager read by the state
/// helpers `0x800DC6F0` / `0x800DCA20..0x800DCDD0`.
///
/// `// unknown:` the manager's class is not recovered; only the fields
/// `+0xE24` (error interface), `+0xE28` (route count) and `+0xE2C`
/// (route table) are observed.
const W38_G_VIEW_MANAGER: usize = SDA_BASE - 0x5220;

/// `0x80377C88`: four view objects indexed by a slot's `+4` word.
///
/// observed as the four words at `0x80377C88..0x80377C94`.
const W38_TABLE_PLAYER_VIEWS: usize = 0x8037_7C88;

/// `0x80371830`: state descriptor installed by [`fn_800dc570`] and used by
/// [`fn_800dc8e0`] when the route byte at view `+0xB1C` has bit 0 clear.
const W38_DESC_80371830: u32 = 0x8037_1830;

/// `0x80371870`: state descriptor requested by [`fn_800dc6f0`] and probed
/// by [`fn_800dc480`].
const W38_DESC_80371870: u32 = 0x8037_1870;

/// `0x803718B0`: state descriptor installed by [`fn_800dc570`].
const W38_DESC_803718B0: u32 = 0x8037_18B0;

/// `0x803718F0`: descriptor for route value `0` in
/// [`fn_800dca20`] / [`fn_800dcca0`] / [`fn_800dcdd0`].
const W38_DESC_803718F0: u32 = 0x8037_18F0;

/// `0x80371930`: descriptor for route value `1` in
/// [`fn_800dca20`] / [`fn_800dcb70`] / [`fn_800dcdd0`].
const W38_DESC_80371930: u32 = 0x8037_1930;

/// `0x80371970`: descriptor for route value `2` in
/// [`fn_800dca20`] / [`fn_800dcb70`] / [`fn_800dcca0`], and the
/// fall-through descriptor for route values near `0xFB`.
const W38_DESC_80371970: u32 = 0x8037_1970;

/// `0x80318E28`: vtable installed by [`fn_800db450`].
const W38_VTABLE_80318E28: u32 = 0x8031_8E28;

/// `0x80318E80`: vtable of the four-slot container built by
/// [`fn_800dbf50`] and released by [`fn_800dbfe0`].
const W38_VTABLE_80318E80: u32 = 0x8031_8E80;

/// `0x80318E8C`: vtable of one slot object built by [`fn_800dc570`].
const W38_VTABLE_80318E8C: u32 = 0x8031_8E8C;

/// Interface word at slot `+0x08` in [`fn_800dc570`].
const W38_IFACE_80318F28: u32 = 0x8031_8F28;

/// First interface word at slot `+0x14`, replaced by
/// [`W38_IFACE_80318F38`] before the second manager is built.
const W38_IFACE_80318F68: u32 = 0x8031_8F68;

/// Replacement for the word at slot `+0x14` in [`fn_800dc570`].
const W38_IFACE_80318F38: u32 = 0x8031_8F38;

/// Interface word at slot `+0x1C` in [`fn_800dc570`].
const W38_IFACE_80318F98: u32 = 0x8031_8F98;

/// Interface word at slot `+0x20` in [`fn_800dc570`].
const W38_IFACE_80318FB0: u32 = 0x8031_8FB0;

/// Interface word at slot `+0x5C` in [`fn_800dc570`], replaced by
/// [`W38_IFACE_80318E98`] before the second manager is built.
const W38_IFACE_80318EC8: u32 = 0x8031_8EC8;

/// Replacement for the word at slot `+0x5C` in [`fn_800dc570`].
const W38_IFACE_80318E98: u32 = 0x8031_8E98;

/// Interface word at slot `+0x64` in [`fn_800dc570`].
const W38_IFACE_80318EF8: u32 = 0x8031_8EF8;

/// Interface word at slot `+0x68` in [`fn_800dc570`].
const W38_IFACE_80318F10: u32 = 0x8031_8F10;

/// `0x800DBF10`: data address passed to `0x801F3690` by [`fn_800dbf50`].
///
/// `// unknown:` contents are not recovered; only the address is observed.
const W38_DAT_800DBF10: usize = 0x800D_BF10;

/// `r2 - 0x6410` -> `0x8042CC70`: `float` constant read by
/// [`fn_800dc820`] and forwarded as a `double` to `0x802BC9A0`.
///
/// `// unknown:` the value is not recovered; the read preserves the operand.
const W38_F32_SDA2_6410: usize = SDA2_BASE - 0x6410;

/// `0x800DADB0`: base constructor called first by [`fn_800db450`].
const FN_800DADB0: usize = 0x800D_ADB0;

/// `0x80101AA0(f64) -> f64`: offset helper used by [`fn_800db620`].
const FN_80101AA0: usize = 0x8010_1AA0;

/// `0x801611C0`: angle helper whose result is stored at `+0x40` by
/// [`fn_800db620`]. The export drops its float operands there.
const FN_801611C0: usize = 0x8016_11C0;

/// `0x800B3600(pad, 0xB)`: blocks one pad button.
const FN_800B3600: usize = 0x800B_3600;

/// `0x800B3750(pad, 0xB, 0)`: releases one pad button.
const FN_800B3750: usize = 0x800B_3750;

/// `0x800B3720(pad, 0xB, 0)`: reported by the held-frame path.
const FN_800B3720: usize = 0x800B_3720;

/// `0x801E0F00(10)`: container setup called by [`fn_800dbf50`].
const FN_801E0F00: usize = 0x801E_0F00;

/// `0x801F3690(&DAT_800DBF10)`: container setup called by [`fn_800dbf50`].
const FN_801F3690: usize = 0x801F_3690;

/// `0x802B9210(size)`: heap allocation returning `0` on failure.
const FN_802B9210: usize = 0x802B_9210;

/// `0x802B9170`: heap-lock thunk bracketing [`fn_800dc430`]. Its return
/// word is passed back to the second call, as in
/// [`save_image_create`].
const FN_802B9170: usize = 0x802B_9170;

/// `0x800DC970(slot)`: per-slot tick called by [`fn_800dc480`].
///
/// `// unknown:` the export cannot recover its jumptable (16 bytes);
/// it is called by address exactly as printed.
const FN_800DC970: usize = 0x800D_C970;

/// `0x801DFBD0(0/1)`: final all-ready flag written by [`fn_800dc480`].
const FN_801DFBD0: usize = 0x801D_FBD0;

/// `0x802BD370(manager, slot)`: per-slot resource handle used by
/// [`fn_800dc570`].
const FN_802BD370: usize = 0x802B_D370;

/// `0x802BC920(handle, 8)`: binds the handle from [`FN_802BD370`].
const FN_802BC920: usize = 0x802B_C920;

/// `0x801E0E40(slot)`: per-slot setup called by [`fn_800dc6f0`] and
/// [`fn_800dc8e0`].
const FN_801E0E40: usize = 0x801E_0E40;

/// `0x800DCF00(slot + 8)`: state-interface helper called by
/// [`fn_800dc8e0`].
///
/// `// unknown:` the export cannot recover its jumptable.
const FN_800DCF00: usize = 0x800D_CF00;

/// `0x800DCEE0(slot + 8)`: state-interface helper called by
/// [`fn_800dc8a0`].
///
/// `// unknown:` the export cannot recover its jumptable.
const FN_800DCEE0: usize = 0x800D_CEE0;

/// `0x800DCED0(slot + 8)`: 4-byte empty body called by [`fn_800dc820`].
const FN_800DCED0: usize = 0x800D_CED0;

/// `0x8016F460(slot)`: lookup stored at slot `+0x50` by
/// [`fn_800dc820`] and [`fn_800dc8e0`].
const FN_8016F460: usize = 0x8016_F460;

/// `0x802D6A60(slot, 0)`: clear call made by [`fn_800dc820`].
const FN_802D6A60: usize = 0x802D_6A60;

/// `0x802D6AC0(slot, 0)`: clear call made by [`fn_800dc8a0`].
const FN_802D6AC0: usize = 0x802D_6AC0;

/// `0x802BC9A0(f64, view, 10, 0)`: route setup made by [`fn_800dc820`].
const FN_802BC9A0: usize = 0x802B_C9A0;

/// `0x802BC9C0(view)`: teardown made by [`fn_800dc9a0`].
const FN_802BC9C0: usize = 0x802B_C9C0;

/// Calls `func(f64) -> f64`.
///
/// Used for `0x80101AA0`; the export prints its operand as a `(double)`
/// cast of the float at `+0xAC`.
#[inline]
unsafe fn call_f64_ret_f64(addr: usize, a: f64) -> f64 {
    let f: extern "C" fn(f64) -> f64 = core::mem::transmute(addr);
    f(a)
}

/// Calls `func(f64, u32, u32, u32) -> u32`.
///
/// Used for `0x802BC9A0`, whose first parameter is printed as a
/// `(double)` cast of an SDA2 float in [`fn_800dc820`].
#[inline]
unsafe fn call_f64_u32_u32_u32(addr: usize, a: f64, b: u32, c: u32, d: u32) -> u32 {
    let f: extern "C" fn(f64, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d)
}

/// Builds one button-table object over a base object.
///
/// In plain English: run the base constructor at `0x800DADB0` over
/// `object`, install the vtable `0x80318E28`, store byte `2` at
/// `+0x42`, and return the object.
///
/// # Original
/// - Ghidra: `FUN_800db450` at `0x800DB450` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800db450_FUN_800db450.c`
///
/// # Arguments
/// * `object` - storage for the button-table object.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Runs the base constructor, writes the vtable word and byte `+0x42`.
///
/// # Safety
/// `object` must have at least `0x43` writable bytes and the base
/// constructor at `0x800DADB0` must accept it.
///
/// # Confidence
/// medium - the vtable, byte and return are literal; the export prints
/// the base call without operands, so the object pointer is
/// `// inferred:` from the same-object constructor pattern used by
/// [`water_actor_construct`].
pub unsafe fn fn_800db450(object: usize) -> usize {
    // observed: FUN_800dadb0() with no printed operand; // inferred: object.
    let _ = platform::call1(FN_800DADB0, object as u32);
    write_word(object, 0, W38_VTABLE_80318E28);
    write_byte(object, 0x42, 2);
    object
}

// ---------------------------------------------------------------------------
// 0x800DB530 - store a point pair, run the virtual, recompute the radius
// ---------------------------------------------------------------------------

/// Orders two points into `+0x4C..+0x5C`, runs vtable slot `+0xC`, then stores
/// a radius derived from `+0x24`/`+0x28`.
///
/// In plain English: record `param_2` at `+0x04`, let [`fn_800db5f0`] order the
/// two `double` pairs into `+0x4C..+0x5C`, invoke the object's virtual slot
/// `+0xC`, and store `sqrt(a*a + b*b)` (with `a = +0x24`, `b = +0x28`) as
/// `f32` at `+0x34`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x800DB530`.
/// - Extent: `0x800DB530..0x800DB590` (96 bytes, 24 words), ending at the
///   `blr` at `0x800DB58C`; the point-order helper [`fn_800db590`] follows.
/// - Reach: no `b`/`bl` and no data word in the main image; `d_basesNP` calls
///   it from `0x806D5F78`, `0x806F0344`, `0x80702F98` and `0x80742E78`,
///   `d_enemiesNP` from thirteen sites.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r31,0xC(r1)`;
///   `mr r31,r3`; `90830004 stw r4,4(r3)`; `bl 0x800DB5F0`; `lwz r12,0(r31)` /
///   `mr r3,r31` / `lwz r12,0xC(r12)` / `mtspr CTR,r12` /
///   `4E800421 bcctrl` (a virtual call that falls through, treated as a call);
///   `lfs f1,0x24(r31)` / `lfs f0,0x28(r31)` /
///   `EC210072 fmuls f1,f1,f1` / `EC000032 fmuls f0,f0,f0` /
///   `EC21002A fadds f1,f1,f0` / `bl 0x802BE470` / `stfs f1,0x34(r31)`;
///   epilogue.
/// - Decoder note: `EC210072` and `EC000032` have `XO = 25`, i.e. **`fmuls`**,
///   and the A-form operand order is `fmuls fD,fA,fC`: `f1 = f1 * f1`
///   (`a*a`, `FRC = 1`) and `f0 = f0 * f0` (`b*b`, `FRC = 0`). `0x802BE470` narrows its result with
///   `frsp`, so it is the `f32` square-root helper already documented as
///   [`W99_FN_802BE470`].
/// - Evidence: `work/wave284/e_dis.txt`, `work/wave284/e_refs.txt`.
///
/// # Arguments
/// - `this`: object; `+0x04` receives `param_2`, `+0x24`/`+0x28` are the two
///   floats, `+0x34` receives the radius and `+0x4C..+0x5C` the ordered points.
/// - `param_2`: word stored at `+0x04` before the calls; never read here.
/// - `first_x` / `first_y` / `second_x` / `second_y`: the two `double` points
///   forwarded to [`fn_800db5f0`] in `f1..f4`.
///
/// # Returns
/// Nothing; the virtual call's integer result is dropped.
///
/// # Side effects
/// One word store, four float stores from [`fn_800db5f0`], one virtual call and
/// the `+0x34` radius store.
///
/// # Safety
/// `this` must be a live object with a readable vtable whose slot `+0xC` is a
/// valid code address; `+0x04`, `+0x34` and `+0x4C..+0x5C` must be writable.
///
/// # Confidence
/// high - every operand, the virtual dispatch, the two `fmuls` and the
/// `fadds`/`bl`/`stfs` chain are literal; only the physical meaning of the
/// `+0x24`/`+0x28` pair (the two half-extents of the bounding box) is inferred
/// from the wave-38 point-order family.
pub unsafe fn fn_800db530(
    this: usize,
    param_2: u32,
    first_x: f64,
    first_y: f64,
    second_x: f64,
    second_y: f64,
) {
    write_word(this, 0x04, param_2);
    fn_800db5f0(first_x, first_y, second_x, second_y, this);
    let _ = vcall0(this, 0x0C);
    let first = read_float(this, 0x24);
    let second = read_float(this, 0x28);
    // observed: `fmuls f1,f1,f1` then `fmuls f0,f0,f0` square each value
    // before the add, so the sum is a*a + b*b (not a*b + b*b).
    let sum = first * first + second * second;
    let root = call_f64_ret_f64(W99_FN_802BE470, sum as f64) as f32;
    write_float(this, 0x34, root);
}

/// Orders two 2-D points so the smaller `x` comes first.
///
/// In plain English: if `*first < *second`, store `first` at
/// `+0x4C/+0x50` and `second` at `+0x54/+0x58`; otherwise store them
/// the other way round. Each point is two floats.
///
/// # Original
/// - Ghidra: `FUN_800db590` at `0x800DB590` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800db590_FUN_800db590.c`
///
/// # Arguments
/// * `object` - object whose `+0x4C..+0x5C` words become the ordered pair.
/// * `first` - two-float point.
/// * `second` - two-float point.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two ordered points at `+0x4C..+0x5C`.
///
/// # Safety
/// `object + 0x4C..+0x5C` must be writable; `first` and `second` must
/// each have two readable floats.
///
/// # Confidence
/// high - the comparison and both store orders are literal.
pub unsafe fn fn_800db590(object: usize, first: usize, second: usize) {
    // observed: if (*param_2 < *param_3) store (2,3) else store (3,2).
    if read_float(first, 0) < read_float(second, 0) {
        write_float(object, 0x4C, read_float(first, 0));
        write_float(object, 0x50, read_float(first, 4));
        write_float(object, 0x54, read_float(second, 0));
        write_float(object, 0x58, read_float(second, 4));
    } else {
        write_float(object, 0x4C, read_float(second, 0));
        write_float(object, 0x50, read_float(second, 4));
        write_float(object, 0x54, read_float(first, 0));
        write_float(object, 0x58, read_float(first, 4));
    }
}

/// Orders two 2-D points passed as `double` pairs, smaller `x` first.
///
/// In plain English: the `double` version of [`fn_800db590`]. The
/// chosen points are narrowed to `float` on store.
///
/// # Original
/// - Ghidra: `FUN_800db5f0` at `0x800DB5F0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800db5f0_FUN_800db5f0.c`
///
/// # Arguments
/// * `first_x` / `first_y` - first point as two `double`s.
/// * `second_x` / `second_y` - second point as two `double`s.
/// * `object` - object whose `+0x4C..+0x5C` words become the ordered pair.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two ordered points at `+0x4C..+0x5C`.
///
/// # Safety
/// `object + 0x4C..+0x5C` must be writable.
///
/// # Confidence
/// high - the comparison and both store orders are literal; the
/// narrowing casts are exactly the export's.
pub unsafe fn fn_800db5f0(
    first_x: f64,
    first_y: f64,
    second_x: f64,
    second_y: f64,
    object: usize,
) {
    if first_x < second_x {
        write_float(object, 0x4C, first_x as f32);
        write_float(object, 0x50, first_y as f32);
        write_float(object, 0x54, second_x as f32);
        write_float(object, 0x58, second_y as f32);
    } else {
        write_float(object, 0x4C, second_x as f32);
        write_float(object, 0x50, second_y as f32);
        write_float(object, 0x54, first_x as f32);
        write_float(object, 0x58, first_y as f32);
    }
}

/// Derives a bounding box and heading from the ordered points.
///
/// In plain English: copy the current `+0x1C/+0x20` pair to
/// `+0x2C/+0x30`, add the offset returned by `0x80101AA0` (called on
/// the float at `+4 + 0xAC`) to the ordered-point floats at
/// `+0x4C` and `+0x54`, add the float at `+4 + 0xB0` to the floats at
/// `+0x50` and `+0x58`, store those truncations as the four `int`
/// words at `+0x1C`, `+0x20`, `+0x14` and `+0x18`, and the two
/// differences at `+0x24` and `+0x28`. Then store the angle from
/// `0x801611C0` at `+0x40`, run vtable slot `+0x10`, clear `+0x10`
/// and byte `+0x46`, and store the `+0x20 - +0x30` difference at
/// `+0x38`.
///
/// # Original
/// - Ghidra: `FUN_800db620` at `0x800DB620` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800db620_FUN_800db620.c`
///
/// # Arguments
/// * `object` - object with the ordered points at `+0x4C..+0x5C`;
///   `+4` points at the float pair at `+0xAC/+0xB0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x14..+0x40` and byte `+0x46`, and runs the vtable call.
///
/// # Safety
/// `object` and the object at `+4` must be live; the angle helper
/// and vtable slot must be callable.
///
/// # Confidence
/// medium - every store, offset and truncation is explicit; the
/// export drops the two float operands of `0x801611C0`, so its call
/// is kept argument-free exactly as printed, and the SDA2-free float
/// fields are read as `float` to match [`fn_800db590`]'s layout.
pub unsafe fn fn_800db620(object: usize) {
    write_word(object, 0x2C, read_word(object, 0x1C));
    let anchor = read_word(object, 4) as usize;
    write_word(object, 0x30, read_word(object, 0x20));

    let offset = call_f64_ret_f64(FN_80101AA0, read_float(anchor, 0xAC) as f64);
    let min_x = read_float(object, 0x4C) as f64 + offset;
    write_word(object, 0x1C, (min_x as f32) as i32 as u32);
    let min_y = read_float(object, 0x50) + read_float(anchor, 0xB0);
    write_word(object, 0x14, ((read_float(object, 0x54) as f64 + offset) as f32) as i32 as u32);
    write_word(object, 0x20, min_y as i32 as u32);
    let max_y = read_float(object, 0x58) + read_float(anchor, 0xB0);
    let width = ((read_float(object, 0x54) as f64 + offset) as f32) - (min_x as f32);
    write_word(object, 0x24, width as i32 as u32);
    write_word(object, 0x18, max_y as i32 as u32);
    write_word(object, 0x28, (max_y - min_y) as i32 as u32);

    // observed: uVar3 = FUN_801611c0(); *(short*)(param_1 + 0x40) = uVar3;
    // unknown: the export prints no operands for this call.
    let angle = call0_r(FN_801611C0);
    write_half(object, 0x40, angle as u16);
    let _ = vcall0(object, 0x10);
    write_word(object, 0x10, 0);
    write_byte(object, 0x46, 0);
    let dx = read_word(object, 0x20) as i32 as f32 - read_word(object, 0x30) as i32 as f32;
    write_word(object, 0x38, (dx as i32) as u32);
}

/// Blocks one pad button when its latch byte is clear.
///
/// In plain English: if the byte at `state + 0` is zero, call
/// `0x800B3600(pad, 0xB)` and set it to `1`.
///
/// # Original
/// - Ghidra: `FUN_800dbcc0` at `0x800DBCC0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbcc0_FUN_800dbcc0.c`
///
/// # Arguments
/// * `state` - three-byte latch; byte `0` is the blocked flag.
/// * `pad` - pad handle passed to `0x800B3600`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call `0x800B3600` and writes byte `0`.
///
/// # Safety
/// `state` must be writable and `pad` a live pad handle.
///
/// # Confidence
/// high - the guard, call and store are literal.
pub unsafe fn fn_800dbcc0(state: usize, pad: u32) {
    if read_byte(state, 0) == 0 {
        let _ = platform::call2(FN_800B3600, pad, 0xB);
        write_byte(state, 0, 1);
    }
}

/// Releases one pad button when its latch byte is set.
///
/// In plain English: if the byte at `state + 0` is non-zero, call
/// `0x800B3750(pad, 0xB, 0)` and clear it.
///
/// # Original
/// - Ghidra: `FUN_800dbd10` at `0x800DBD10` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbd10_FUN_800dbd10.c`
///
/// # Arguments
/// * `state` - three-byte latch; byte `0` is the blocked flag.
/// * `pad` - pad handle passed to `0x800B3750`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call `0x800B3750` and writes byte `0`.
///
/// # Safety
/// `state` must be writable and `pad` a live pad handle.
///
/// # Confidence
/// high - the guard, call and store are literal.
pub unsafe fn fn_800dbd10(state: usize, pad: u32) {
    if read_byte(state, 0) != 0 {
        let _ = platform::call3(FN_800B3750, pad, 0xB, 0);
        write_byte(state, 0, 0);
    }
}

/// Clears one three-byte pad latch.
///
/// In plain English: write zero to bytes `0`, `1` and `2`.
///
/// # Original
/// - Ghidra: `FUN_800dbd60` at `0x800DBD60` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbd60_FUN_800dbd60.c`
///
/// # Arguments
/// * `triple` - three-byte latch to clear.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three zero bytes.
///
/// # Safety
/// `triple` must have three writable bytes.
///
/// # Confidence
/// high - three literal stores.
pub unsafe fn fn_800dbd60(triple: usize) {
    write_byte(triple, 0, 0);
    write_byte(triple, 1, 0);
    write_byte(triple, 2, 0);
}

/// Updates one three-byte pad latch from the selected pad.
///
/// In plain English: while the hold counter at byte `2` is below `2`:
/// when `pad == selected` and byte `1` is set, increment byte `2` and,
/// once it passes `1`, report the hold through `0x800B3720(pad, 0xB, 0)`
/// and clear bytes `0`/`1`; when `pad == selected` and byte `1` is
/// clear, block the button through [`fn_800dbcc0`]; otherwise release
/// it through [`fn_800dbd10`]. Byte `1` is cleared on every path
/// except the counter increment that reaches `2` reporting the hold
/// (where the clear is part of that branch). Once byte `2` reaches
/// `2`, only byte `1` is cleared.
///
/// # Original
/// - Ghidra: `FUN_800dbd80` at `0x800DBD80` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbd80_FUN_800dbd80.c`
///
/// # Arguments
/// * `triple` - three-byte latch: blocked `+0`, held `+1`, counter `+2`.
/// * `pad` - pad handle; also the loop slot index used by
///   [`fn_800dbe80`].
/// * `selected` - currently selected pad; equality with `pad` picks
///   the block vs release edge.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes latch bytes and may call `0x800B3720` or the block/release
/// helpers.
///
/// # Safety
/// `triple` must have three writable bytes and `pad` a live handle
/// when a block/release edge runs.
///
/// # Confidence
/// medium - the counter, both edges and every store are explicit;
/// the export prints the block/release calls without their pad
/// operand, which is `// inferred:` as `pad` from the `0x800B3720`
/// call in the same body, where `pad` is passed explicitly.
pub unsafe fn fn_800dbd80(triple: usize, pad: u32, selected: u32) {
    if read_byte(triple, 2) < 2 {
        if pad == selected && read_byte(triple, 1) != 0 {
            let counter = read_byte(triple, 2).wrapping_add(1);
            write_byte(triple, 2, counter);
            if counter > 1 {
                let _ = platform::call3(FN_800B3720, pad, 0xB, 0);
                write_byte(triple, 0, 0);
                write_byte(triple, 1, 0);
            }
        } else if pad == selected {
            fn_800dbcc0(triple, pad);
            write_byte(triple, 1, 0);
        } else {
            fn_800dbd10(triple, pad);
            write_byte(triple, 1, 0);
        }
    } else {
        write_byte(triple, 1, 0);
    }
}

/// Clears four consecutive three-byte latches.
///
/// In plain English: run [`fn_800dbd60`] on the latch at `base` and
/// the next three latches at 3-byte strides.
///
/// # Original
/// - Ghidra: `FUN_800dbe30` at `0x800DBE30` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbe30_FUN_800dbe30.c`
///
/// # Arguments
/// * `base` - first of four three-byte latches.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes twelve zero bytes.
///
/// # Safety
/// `base` must have twelve writable bytes.
///
/// # Confidence
/// high - the `base + 0xC` bound and 3-byte stride are literal.
pub unsafe fn fn_800dbe30(base: usize) {
    let end = base + 0xC;
    let mut latch = base;
    loop {
        fn_800dbd60(latch);
        latch += 3;
        if latch >= end {
            break;
        }
    }
}

/// Updates four consecutive three-byte latches.
///
/// In plain English: run [`fn_800dbd80`] on the latch at `base` with
/// slot index `0..3` and the selected pad, advancing 3 bytes per slot.
///
/// # Original
/// - Ghidra: `FUN_800dbe80` at `0x800DBE80` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbe80_FUN_800dbe80.c`
///
/// # Arguments
/// * `base` - first of four three-byte latches.
/// * `selected` - currently selected pad.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs the per-latch update four times.
///
/// # Safety
/// `base` must have twelve writable bytes and the pad handles for
/// slots `0..3` must be live on block/release edges.
///
/// # Confidence
/// high - the four-iteration loop and 3-byte stride are literal.
pub unsafe fn fn_800dbe80(base: usize, selected: u32) {
    let mut index: u32 = 0;
    let mut latch = base;
    loop {
        fn_800dbd80(latch, index, selected);
        index += 1;
        latch += 3;
        if index >= 4 {
            break;
        }
    }
}

/// Constructs the four-slot state container.
///
/// In plain English: install the vtable `0x80318E80`, run the
/// `10`-argument setup at `0x801E0F00` and the `0x801F3690` setup
/// over `0x800DBF10`, then allocate four `0x98`-byte slot objects with
/// [`fn_800dc570`] into `+4..+0x10`, publish the container in
/// [`G_PLAYER_SLOTS`], and return it.
///
/// # Original
/// - Ghidra: `FUN_800dbf50` at `0x800DBF50` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbf50_FUN_800dbf50.c`
///
/// # Arguments
/// * `container` - storage for the vtable and four slot pointers.
///
/// # Returns
/// `container`, unchanged.
///
/// # Side effects
/// Runs the two setups, allocates up to four slots, and writes the
/// global slot pointer.
///
/// # Safety
/// `container` must have at least `0x14` writable bytes; the heap
/// must accept `0x98`-byte allocations.
///
/// # Confidence
/// high - the vtable, both setups, the `0x98` size, the four
/// iterations and the global store are literal.
pub unsafe fn fn_800dbf50(container: usize) -> usize {
    write_word(container, 0, W38_VTABLE_80318E80);
    let _ = platform::call1(FN_801E0F00, 10);
    let _ = platform::call1(FN_801F3690, W38_DAT_800DBF10 as u32);

    let mut slot_index: u32 = 0;
    loop {
        let allocation = platform::call1(FN_802B9210, 0x98);
        let slot = if allocation != 0 {
            fn_800dc570(allocation as usize, slot_index)
        } else {
            0
        };
        write_word(container, 4 + slot_index as usize * 4, slot as u32);
        slot_index += 1;
        if slot_index >= 4 {
            break;
        }
    }
    write_global_word(G_PLAYER_SLOTS, container as u32);
    container
}

/// Releases the four-slot state container.
///
/// In plain English: when `container` is non-null, run vtable slot
/// `+8` with argument `1` on each non-null slot, clear
/// [`G_PLAYER_SLOTS`], and free the container storage when
/// `free > 0`. Always return the container pointer.
///
/// # Original
/// - Ghidra: `FUN_800dbfe0` at `0x800DBFE0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dbfe0_FUN_800dbfe0.c`
///
/// # Arguments
/// * `container` - container built by [`fn_800dbf50`], or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `container`, unchanged.
///
/// # Side effects
/// Releases the slots, clears the global, and may free the container.
///
/// # Safety
/// `container` must be null or a live container whose four slot
/// words hold null or live slot objects.
///
/// # Confidence
/// high - the four-iteration loop, global clear and guards are
/// literal.
pub unsafe fn fn_800dbfe0(container: usize, free: i32) -> usize {
    if container != 0 {
        write_word(container, 0, W38_VTABLE_80318E80);
        let mut slot_index: u32 = 0;
        loop {
            let slot = read_word(container, 4 + slot_index as usize * 4) as usize;
            if slot != 0 {
                let _ = vcall1(slot, 8, 1);
            }
            slot_index += 1;
            if slot_index >= 4 {
                break;
            }
        }
        write_global_word(G_PLAYER_SLOTS, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, container as u32);
        }
    }
    container
}

/// Releases one slot's `+0x18` sub-object and the slot when `free` is positive.
///
/// In plain English: when `object` is non-null and `object + 0x18` is
/// non-null, run `0x8015F9F0(object + 0x18, 0)`; when `free > 0`,
/// free the object. Always return the object.
///
/// # Original
/// - Ghidra: `FUN_800dc120` at `0x800DC120` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc120_FUN_800dc120.c`
///
/// # Arguments
/// * `object` - slot object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May release the `+0x18` sub-object and the object storage.
///
/// # Safety
/// `object` must be null or a live slot object.
///
/// # Confidence
/// high - both guards, the call and the return are literal, including
/// the `object + 0x18 != 0` test.
pub unsafe fn fn_800dc120(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Frees one slot object when `free` is positive.
///
/// In plain English: when `object` is non-null and `free > 0`, call
/// `0x802B9280(object)`. Always return the object.
///
/// # Original
/// - Ghidra: `FUN_800dc1f0` at `0x800DC1F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc1f0_FUN_800dc1f0.c`
///
/// # Arguments
/// * `object` - slot object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May free the object storage.
///
/// # Safety
/// `object` must be null or a live heap allocation.
///
/// # Confidence
/// high - the two guards and the return are literal.
pub unsafe fn fn_800dc1f0(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Frees one slot object when `free` is positive.
///
/// In plain English: identical shape to [`fn_800dc1f0`]; a second
/// vtable destructor for a sibling slot class.
///
/// # Original
/// - Ghidra: `FUN_800dc230` at `0x800DC230` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc230_FUN_800dc230.c`
///
/// # Arguments
/// * `object` - slot object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May free the object storage.
///
/// # Safety
/// `object` must be null or a live heap allocation.
///
/// # Confidence
/// high - the two guards and the return are literal.
pub unsafe fn fn_800dc230(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Releases one slot's `+0x24` sub-object and the slot when `free` is positive.
///
/// In plain English: when `object` is non-null, is not `-0x0C`, and
/// `object + 0x24` is non-null, run `0x8015F9F0(object + 0x24, 0)`;
/// when `free > 0`, free the object. Always return the object.
///
/// # Original
/// - Ghidra: `FUN_800dc270` at `0x800DC270` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc270_FUN_800dc270.c`
///
/// # Arguments
/// * `object` - slot object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May release the `+0x24` sub-object and the object storage.
///
/// # Safety
/// `object` must be null or a live slot object.
///
/// # Confidence
/// high - every guard, including the `-0x0C` sentinel and
/// `object + 0x24 != 0`, is literal.
pub unsafe fn fn_800dc270(object: usize, free: i32) -> usize {
    if object != 0 {
        if (object as i32) != -0x0C && object + 0x24 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x24) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Frees one slot object when `free` is positive.
///
/// In plain English: identical shape to [`fn_800dc1f0`]; another
/// sibling-class destructor.
///
/// # Original
/// - Ghidra: `FUN_800dc3b0` at `0x800DC3B0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc3b0_FUN_800dc3b0.c`
///
/// # Arguments
/// * `object` - slot object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May free the object storage.
///
/// # Safety
/// `object` must be null or a live heap allocation.
///
/// # Confidence
/// high - the two guards and the return are literal.
pub unsafe fn fn_800dc3b0(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Frees one slot object when `free` is positive.
///
/// In plain English: identical shape to [`fn_800dc1f0`]; another
/// sibling-class destructor.
///
/// # Original
/// - Ghidra: `FUN_800dc3f0` at `0x800DC3F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc3f0_FUN_800dc3f0.c`
///
/// # Arguments
/// * `object` - slot object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May free the object storage.
///
/// # Safety
/// `object` must be null or a live heap allocation.
///
/// # Confidence
/// high - the two guards and the return are literal.
pub unsafe fn fn_800dc3f0(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Creates the four-slot container under the heap lock.
///
/// In plain English: save the heap lock word from `0x802B9170`,
/// allocate `0x14` bytes, build the container with
/// [`fn_800dbf50`] when the allocation succeeds, then restore the
/// lock word.
///
/// # Original
/// - Ghidra: `FUN_800dc430` at `0x800DC430` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc430_FUN_800dc430.c`
///
/// # Arguments
/// None. The container is published in [`G_PLAYER_SLOTS`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates the container and brackets the build with the heap
/// lock/unlock calls.
///
/// # Safety
/// Must run once during start-up with a live default heap.
///
/// # Confidence
/// medium - the lock pair, `0x14` size and null check are literal;
/// the export prints the container construction without operands,
/// so the allocation pointer is `// inferred:` from
/// [`save_image_create`]'s identical shape.
pub unsafe fn fn_800dc430() {
    let previous = call0_r(FN_802B9170);
    let allocation = platform::call1(FN_802B9210, 0x14);
    if allocation != 0 {
        let _ = fn_800dbf50(allocation as usize);
    }
    let _ = platform::call1(FN_802B9170, previous);
}

/// Ticks all four slots and reports whether every slot is ready.
///
/// In plain English: run `0x800DC970` on each of the four slot
/// pointers at container `+4..+0x10`. Then, for each slot whose byte
/// at `+0x54` is set, call the function pointer at its `+0x5C`
/// interface's `+0x28` word, ask the result's vtable slot `+0x10`
/// about descriptor `0x80371870`, and latch `all_ready = false` when
/// it answers zero. Finally call `0x801DFBD0(0)` when all slots are
/// ready, else `0x801DFBD0(1)`.
///
/// # Original
/// - Ghidra: `FUN_800dc480` at `0x800DC480` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc480_FUN_800dc480.c`
///
/// # Arguments
/// None. The container comes from [`G_PLAYER_SLOTS`]. Ghidra prints
/// `void` because the body starts with the `0x802DCD74`
/// register-save helper.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Ticks the slots, probes the four route objects, and writes the
/// all-ready flag.
///
/// # Safety
/// [`G_PLAYER_SLOTS`] must point at a live container built by
/// [`fn_800dbf50`]; each slot's `+0x5C` interface and its `+0x28`
/// function word must be live when byte `+0x54` is set.
///
/// # Confidence
/// medium - the loops, stride, descriptor and final flag are
/// literal; the `+0x28` function word is called with no printed
/// operand (`// unknown:` receiver dropped by the export), and the
/// per-slot tick `0x800DC970` is called by address because its
/// jumptable is unrecovered.
pub unsafe fn fn_800dc480() {
    let container = read_global_word(G_PLAYER_SLOTS) as usize;
    let mut index: u32 = 0;
    loop {
        let slot = read_word(container, 4 + index as usize * 4) as usize;
        let _ = platform::call1(FN_800DC970, slot as u32);
        index += 1;
        if index >= 4 {
            break;
        }
    }

    let mut all_ready = true;
    let mut slot_index: u32 = 0;
    loop {
        let slot = read_word(container, slot_index as usize * 4 + 4) as usize;
        if read_byte(slot, 0x54) != 0 {
            let interface = read_word(slot, 0x5C) as usize;
            let entry = read_word(interface, 0x28) as usize;
            let route = call0_r(entry) as usize;
            if vcall1(route, 0x10, W38_DESC_80371870) == 0 {
                all_ready = false;
                break;
            }
        }
        slot_index += 1;
        if slot_index >= 4 {
            break;
        }
    }
    let _ = platform::call1(FN_801DFBD0, if all_ready { 0 } else { 1 });
}

/// Constructs one slot object with its two state managers.
///
/// In plain English: install the vtable `0x80318E8C`, store the slot
/// index at `+4` and `+0x0C`, write the interface tables at
/// `+0x08..+0x28`, build the first state manager at `+0x2C` with
/// descriptor `0x803718B0`, set `+0x50` to `-1` and the first table
/// to `0x80318F38`, set byte `+0x54` to `1` and `+0x55` to `0`, write
/// the second interface set at `+0x5C..+0x6C`, build the second state
/// manager at `+0x74` with descriptor `0x80371830`, replace the
/// `+0x5C` table with `0x80318E98`, then bind the resource handle
/// from `0x802BD370` with `0x802BC920(handle, 8)` and return the slot.
///
/// # Original
/// - Ghidra: `FUN_800dc570` at `0x800DC570` (336 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc570_FUN_800dc570.c`
///
/// # Arguments
/// * `slot` - storage for the slot object.
/// * `index` - slot index stored at `+4` and `+0x0C` and used for
///   the resource lookup.
///
/// # Returns
/// `slot`, unchanged.
///
/// # Side effects
/// Writes the whole slot and builds two state managers.
///
/// # Safety
/// `slot` must have at least `0x78` writable bytes and the manager
/// at [`W38_G_VIEW_MANAGER`] must be live for the resource lookup.
///
/// # Confidence
/// high - every offset, table, descriptor and the final bind are
/// literal.
pub unsafe fn fn_800dc570(slot: usize, index: u32) -> usize {
    write_word(slot, 0, W38_VTABLE_80318E8C);
    write_word(slot, 4, index);
    write_word(slot, 0x0C, index);
    write_word(slot, 8, W38_IFACE_80318F28);
    write_word(slot, 0x10, 3);
    write_word(slot, 0x14, W38_IFACE_80318F68);
    write_word(slot, 0x18, VTABLE_SHARED_IFACE);
    write_word(slot, 0x1C, W38_IFACE_80318F98);
    write_word(slot, 0x20, W38_IFACE_80318FB0);
    write_word(slot, 0x24, (slot + 8) as u32);
    write_word(slot, 0x28, 0);
    let _ = platform::call4(
        FN_8015FC90,
        (slot + 0x2C) as u32,
        (slot + 0x18) as u32,
        (slot + 0x1C) as u32,
        W38_DESC_803718B0,
    );

    write_word(slot, 0x50, 0xFFFF_FFFF);
    write_word(slot, 0x14, W38_IFACE_80318F38);
    write_byte(slot, 0x54, 1);
    write_byte(slot, 0x55, 0);
    write_word(slot, 0x5C, W38_IFACE_80318EC8);
    write_word(slot, 0x60, VTABLE_SHARED_IFACE);
    write_word(slot, 0x64, W38_IFACE_80318EF8);
    write_word(slot, 0x68, W38_IFACE_80318F10);
    write_word(slot, 0x6C, slot as u32);
    write_word(slot, 0x70, 0);
    let _ = platform::call4(
        FN_8015FC90,
        (slot + 0x74) as u32,
        (slot + 0x60) as u32,
        (slot + 0x64) as u32,
        W38_DESC_80371830,
    );
    write_word(slot, 0x5C, W38_IFACE_80318E98);

    let manager = read_global_word(W38_G_VIEW_MANAGER) as usize;
    let handle = platform::call2(FN_802BD370, manager as u32, index);
    let _ = platform::call2(FN_802BC920, handle, 8);
    slot
}

/// Updates one slot from its route byte.
///
/// In plain English: read the slot index at `+4` and the route byte
/// at view `+0x74`. When the index is out of range
/// (`-1 < index < manager + 0xE28`), run the error interface at
/// `manager + 0xE24`. When the route byte matches the entry at
/// `manager + 0xE2C` and the view's byte at `+0xB1C` has bit 0 set:
/// run `0x801E0E40` when byte `+0x54` is clear, latch `+0x58 = 1` when
/// it is `0`, otherwise request descriptor `0x80371870` on the
/// `+0x5C` interface.
///
/// # Original
/// - Ghidra: `FUN_800dc6f0` at `0x800DC6F0` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc6f0_FUN_800dc6f0.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run the error interface, `0x801E0E40`, or the state change,
/// and may latch `+0x58`.
///
/// # Safety
/// The slot, the view table, the manager at [`W38_G_VIEW_MANAGER`]
/// and the slot's `+0x5C` interface must be live.
///
/// # Confidence
/// medium - the bounds check, route comparison and three-way branch
/// are literal; the export prints `0x801E0E40` without operands, so
/// the slot index is `// inferred:` from the sibling
/// [`fn_800dc8e0`], where the same call is printed with that index.
pub unsafe fn fn_800dc6f0(slot: usize) {
    let index = read_word(slot, 4) as i32;
    let manager = read_global_word(W38_G_VIEW_MANAGER) as usize;
    let view = read_global_word(
        W38_TABLE_PLAYER_VIEWS.wrapping_add((index as usize).wrapping_mul(4)),
    ) as usize;
    let route = platform::read_u8(view + 0x74);

    let in_range = -1 < index && index < read_word(manager, 0xE28) as i32;
    if !in_range {
        let _ = vcall0(manager + 0xE24, 0x18);
    }
    let table = read_word(manager, 0xE2C) as usize;
    let expected = read_global_word(table.wrapping_add((index as usize).wrapping_mul(4)));
    if route as u32 == expected && (platform::read_u8(view + 0xB1C) & 1) != 0 {
        if read_byte(slot, 0x54) == 0 {
            let _ = platform::call1(FN_801E0E40, index as u32);
        } else if read_word(slot, 0x58) == 0 {
            write_word(slot, 0x58, 1);
        } else {
            change_state(slot + 0x5C, W38_DESC_80371870);
        }
    }
}

/// Refreshes one slot's lookup and starts its route when allowed.
///
/// In plain English: run `0x800DCED0(slot + 8)`, store the result of
/// `0x8016F460(slot + 4)` at `+0x50`, clear the route with
/// `0x802D6A60(slot + 4, 0)`, and when the byte at
/// `r13 - 0x7710` is clear, start the route with
/// `0x802BC9A0(float(r2 - 0x6410), view, 10, 0)`.
///
/// # Original
/// - Ghidra: `FUN_800dc820` at `0x800DC820` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc820_FUN_800dc820.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs two clear/lookup calls, writes `+0x50`, and may start the
/// route.
///
/// # Safety
/// The slot and its view must be live; the gate byte at
/// `r13 - 0x7710` must be readable.
///
/// # Confidence
/// medium - the three calls, `+0x50` store and gate are literal;
/// the SDA2 float value is read by address, and `0x800DCED0` is a
/// 4-byte empty body called by address.
pub unsafe fn fn_800dc820(slot: usize) {
    let _ = platform::call1(FN_800DCED0, (slot + 8) as u32);
    let lookup = platform::call1(FN_8016F460, read_word(slot, 4));
    write_word(slot, 0x50, lookup);
    let _ = platform::call2(FN_802D6A60, read_word(slot, 4), 0);
    if read_global_byte(SDA_BASE - 0x7710) == 0 {
        let index = read_word(slot, 4) as usize;
        let view = read_global_word(W38_TABLE_PLAYER_VIEWS + index * 4);
        let _ = call_f64_u32_u32_u32(
            FN_802BC9A0,
            read_const_f32(W38_F32_SDA2_6410) as f64,
            view,
            10,
            0,
        );
    }
}

/// Stops one slot's route.
///
/// In plain English: run `0x800DCEE0(slot + 8)`, then clear the route
/// with `0x802D6AC0(slot + 4, 0)`.
///
/// # Original
/// - Ghidra: `FUN_800dc8a0` at `0x800DC8A0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc8a0_FUN_800dc8a0.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs the two clear calls.
///
/// # Safety
/// The slot must be live.
///
/// # Confidence
/// medium - both calls and arguments are literal; `0x800DCEE0` is
/// called by address because its jumptable is unrecovered.
pub unsafe fn fn_800dc8a0(slot: usize) {
    let _ = platform::call1(FN_800DCEE0, (slot + 8) as u32);
    let _ = platform::call2(FN_802D6AC0, read_word(slot, 4), 0);
}

/// Starts or re-arms one slot's route state.
///
/// In plain English: when the view's byte at `+0xB1C` has bit 0
/// clear, request descriptor `0x80371830` on the `+0x5C` interface.
/// Otherwise, when byte `+0x54` is clear, run `0x801E0E40` with the
/// slot index; then run `0x800DCF00(slot + 8)` and store the result
/// of `0x8016F460(slot + 4)` at `+0x50`.
///
/// # Original
/// - Ghidra: `FUN_800dc8e0` at `0x800DC8E0` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc8e0_FUN_800dc8e0.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs the state change or the re-arm calls, and may write `+0x50`.
///
/// # Safety
/// The slot and its view must be live.
///
/// # Confidence
/// high - the bit test, state change and both re-arm calls are
/// literal; `0x800DCF00` is called by address because its jumptable
/// is unrecovered.
pub unsafe fn fn_800dc8e0(slot: usize) {
    let index = read_word(slot, 4) as usize;
    let view = read_global_word(W38_TABLE_PLAYER_VIEWS + index * 4) as usize;
    if (platform::read_u8(view + 0xB1C) & 1) == 0 {
        change_state(slot + 0x5C, W38_DESC_80371830);
    } else {
        if read_byte(slot, 0x54) == 0 {
            let _ = platform::call1(FN_801E0E40, index as u32);
        }
        let _ = platform::call1(FN_800DCF00, (slot + 8) as u32);
        let lookup = platform::call1(FN_8016F460, index as u32);
        write_word(slot, 0x50, lookup);
    }
}

/// Stops one slot's route when its route flag is set.
///
/// In plain English: when byte `+0x55` is set, run
/// `0x802BC9C0(view)`; then clear byte `+0x55` either way.
///
/// # Original
/// - Ghidra: `FUN_800dc9a0` at `0x800DC9A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc9a0_FUN_800dc9a0.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run `0x802BC9C0` and clears byte `+0x55`.
///
/// # Safety
/// The slot and its view must be live.
///
/// # Confidence
/// high - the guard, call and unconditional clear are literal.
pub unsafe fn fn_800dc9a0(slot: usize) {
    if read_byte(slot, 0x55) != 0 {
        let index = read_word(slot, 4) as usize;
        let view = read_global_word(W38_TABLE_PLAYER_VIEWS + index * 4);
        let _ = platform::call1(FN_802BC9C0, view);
    }
    write_byte(slot, 0x55, 0);
}

/// Requests the state for one slot from the global dispatch.
///
/// In plain English: recover the slot object hidden by the
/// register-save prologue, read its route byte at view `+0x74`,
/// run the manager's error interface when the slot index is out of
/// range, and when the route byte matches the expected entry at
/// `manager + 0xE2C`, dispatch on the route value: `1` requests
/// descriptor `0x80371930` on `+0x0C`, `0` requests `0x803718F0`,
/// values outside `0..2` (modulo the original's unsigned
/// `value - 0xFB` test) request `0x80371970` for `2` and do nothing
/// otherwise. A route byte equal to `0xFB` or `0xFC` falls through
/// to the `0x80371970` request.
///
/// # Original
/// - Ghidra: `FUN_800dca20` at `0x800DCA20` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dca20_FUN_800dca20.c`
///
/// # Arguments
/// * `slot` - slot object. Ghidra prints `void` because the body
///   starts with the `0x802DCD74` register-save helper; `r3` is the
///   incoming slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run the error interface and may request one state on `+0x0C`.
///
/// # Safety
/// The slot, its view, the manager at [`W38_G_VIEW_MANAGER`] and
/// the slot's `+0x0C` interface must be live.
///
/// # Confidence
/// medium - the bounds check, route comparison and dispatch are
/// literal; the slot argument is recovered from `r3` because the
/// export hides it behind the register-save helper.
pub unsafe fn fn_800dca20(slot: usize) {
    let index = read_word(slot, 4) as i32;
    let manager = read_global_word(W38_G_VIEW_MANAGER) as usize;
    let view = read_global_word(
        W38_TABLE_PLAYER_VIEWS.wrapping_add((index as usize).wrapping_mul(4)),
    ) as usize;
    let route = platform::read_u8(view + 0x74) as u32;

    let in_range = -1 < index && index < read_word(manager, 0xE28) as i32;
    if !in_range {
        let _ = vcall0(manager + 0xE24, 0x18);
    }
    let table = read_word(manager, 0xE2C) as usize;
    let expected = read_global_word(table.wrapping_add((index as usize).wrapping_mul(4)));
    if route != expected {
        return;
    }
    if route.wrapping_sub(0xFB) > 1 {
        if route == 1 {
            change_state(slot + 0x0C, W38_DESC_80371930);
            return;
        }
        if route == 0 {
            change_state(slot + 0x0C, W38_DESC_803718F0);
            return;
        }
        if route != 2 {
            return;
        }
    }
    change_state(slot + 0x0C, W38_DESC_80371970);
}

/// Requests the state for one slot for routes `1` and `2`.
///
/// In plain English: the per-slot version of [`fn_800dca20`]. When
/// the route byte matches the expected entry, route `1` requests
/// descriptor `0x80371930` on `+0x0C`, route `2` requests
/// `0x80371970`, and any other route (including `0`) does nothing.
/// When the route byte is `0xFB` or `0xFC` the fall-through requests
/// `0x80371970`.
///
/// # Original
/// - Ghidra: `FUN_800dcb70` at `0x800DCB70` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dcb70_FUN_800dcb70.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run the error interface and may request one state on `+0x0C`.
///
/// # Safety
/// Same as [`fn_800dca20`].
///
/// # Confidence
/// high - the bounds check, comparison and the two reachable
/// descriptors are literal; the missing route-`0` case is the
/// original's, not a gap.
pub unsafe fn fn_800dcb70(slot: usize) {
    let index = read_word(slot, 4) as i32;
    let manager = read_global_word(W38_G_VIEW_MANAGER) as usize;
    let view = read_global_word(
        W38_TABLE_PLAYER_VIEWS.wrapping_add((index as usize).wrapping_mul(4)),
    ) as usize;
    let route = platform::read_u8(view + 0x74) as u32;

    let in_range = -1 < index && index < read_word(manager, 0xE28) as i32;
    if !in_range {
        let _ = vcall0(manager + 0xE24, 0x18);
    }
    let table = read_word(manager, 0xE2C) as usize;
    let expected = read_global_word(table.wrapping_add((index as usize).wrapping_mul(4)));
    if route != expected {
        return;
    }
    if route.wrapping_sub(0xFB) > 1 {
        if route == 1 {
            change_state(slot + 0x0C, W38_DESC_80371930);
            return;
        }
        if route != 2 {
            return;
        }
    }
    change_state(slot + 0x0C, W38_DESC_80371970);
}

/// Requests the state for one slot for routes `0` and `2`.
///
/// In plain English: the mirror of [`fn_800dcb70`]. When the route
/// byte matches the expected entry, route `0` requests descriptor
/// `0x803718F0` on `+0x0C`, route `2` requests `0x80371970`, and any
/// other route (including `1`) does nothing. The `0xFB`/`0xFC`
/// fall-through requests `0x80371970`.
///
/// # Original
/// - Ghidra: `FUN_800dcca0` at `0x800DCCA0` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dcca0_FUN_800dcca0.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run the error interface and may request one state on `+0x0C`.
///
/// # Safety
/// Same as [`fn_800dca20`].
///
/// # Confidence
/// high - the bounds check, comparison and the two reachable
/// descriptors are literal.
pub unsafe fn fn_800dcca0(slot: usize) {
    let index = read_word(slot, 4) as i32;
    let manager = read_global_word(W38_G_VIEW_MANAGER) as usize;
    let view = read_global_word(
        W38_TABLE_PLAYER_VIEWS.wrapping_add((index as usize).wrapping_mul(4)),
    ) as usize;
    let route = platform::read_u8(view + 0x74) as u32;

    let in_range = -1 < index && index < read_word(manager, 0xE28) as i32;
    if !in_range {
        let _ = vcall0(manager + 0xE24, 0x18);
    }
    let table = read_word(manager, 0xE2C) as usize;
    let expected = read_global_word(table.wrapping_add((index as usize).wrapping_mul(4)));
    if route != expected {
        return;
    }
    if route.wrapping_sub(0xFB) > 1 {
        if route == 0 {
            change_state(slot + 0x0C, W38_DESC_803718F0);
            return;
        }
        if route != 2 {
            return;
        }
    }
    change_state(slot + 0x0C, W38_DESC_80371970);
}

/// Requests the state for one slot for routes `0` and `1` only.
///
/// In plain English: when the route byte matches the expected
/// entry, route `1` requests descriptor `0x80371930` on `+0x0C` and
/// route `0` requests `0x803718F0`. Route `2` and every other value
/// do nothing; there is no fall-through descriptor in this body.
///
/// # Original
/// - Ghidra: `FUN_800dcdd0` at `0x800DCDD0` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dcdd0_FUN_800dcdd0.c`
///
/// # Arguments
/// * `slot` - slot object built by [`fn_800dc570`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run the error interface and may request one state on `+0x0C`.
///
/// # Safety
/// Same as [`fn_800dca20`].
///
/// # Confidence
/// high - the bounds check, comparison and the two descriptors are
/// literal.
pub unsafe fn fn_800dcdd0(slot: usize) {
    let index = read_word(slot, 4) as i32;
    let manager = read_global_word(W38_G_VIEW_MANAGER) as usize;
    let view = read_global_word(
        W38_TABLE_PLAYER_VIEWS.wrapping_add((index as usize).wrapping_mul(4)),
    ) as usize;
    let route = platform::read_u8(view + 0x74) as u32;

    let in_range = -1 < index && index < read_word(manager, 0xE28) as i32;
    if !in_range {
        let _ = vcall0(manager + 0xE24, 0x18);
    }
    let table = read_word(manager, 0xE2C) as usize;
    let expected = read_global_word(table.wrapping_add((index as usize).wrapping_mul(4)));
    if route == expected {
        if route == 1 {
            change_state(slot + 0x0C, W38_DESC_80371930);
        } else if route == 0 {
            change_state(slot + 0x0C, W38_DESC_803718F0);
        }
    }
}

// ---------------------------------------------------------------------------
// Wave 63: 0x800D54A0..0x800D9AD0 - the `0x80303408` actor wrapper, the
// spin-jump counter object at `r13 - 0x5880`, and the actor search context
// (both the `0x80318A80`/`0x80318A90` node wrappers and the `0x80318E70`
// sensor object).
// ---------------------------------------------------------------------------

/// `0x80303408`: vtable installed by [`fn_800d54a0`] on the actor wrapper and
/// released by [`fn_800d54e0`].
const W63_VTABLE_80303408: u32 = 0x8030_3408;

/// `0x80318A80`: vtable installed by [`fn_800d6e00`] before it releases the
/// object's owner.
const W63_VTABLE_80318A80: u32 = 0x8031_8A80;

/// `0x80318A90`: vtable installed by [`fn_800d70a0`]; the word at `+0x04` is
/// initialised to `3`.
const W63_VTABLE_80318A90: u32 = 0x8031_8A90;

/// `0x80318C30`: vtable installed by [`fn_800d85d0`] on the single global
/// spin-jump counter object.
const W63_VTABLE_80318C30: u32 = 0x8031_8C30;

/// `0x80318E70`: vtable installed by [`fn_800d9860`] on the embedded search
/// context.
const W63_VTABLE_80318E70: u32 = 0x8031_8E70;

/// `0x80377C48`: data word passed as the third argument of `0x800690C0` by
/// [`fn_800d5580`]. `// unknown:` the export only prints the symbol, so the
/// word is read instead of copied as an address.
const W63_DATA_80377C48: usize = 0x8037_7C48;

/// `0x80370DA8`: per-player scale table indexed by the byte at
/// `owner + 0x152` in [`fn_800d6eb0`] and [`fn_800d6f40`].
const W63_TABLE_80370DA8: usize = 0x8037_0DA8;

/// `r13 - 0x5880` -> `0x80429E00`: pointer to the live spin-jump counter
/// object built by [`fn_800d85d0`].
const W63_G_SPINJUMP: usize = SDA_BASE - 0x5880;

/// `r13 - 0x5670` -> `0x8042A010`: word saved by [`fn_800d9940`] before it
/// overwrites the search result slot. `// unknown:` role.
const W63_G_SEARCH_SAVED: usize = SDA_BASE - 0x5670;

/// `r13 - 0x566C` -> `0x8042A014`: last accepted search result published by
/// [`fn_800d9940`] and [`fn_800d9a30`].
const W63_G_SEARCH_RESULT: usize = SDA_BASE - 0x566C;

/// `r13 - 0x5664` -> `0x8042A01C`: tail of the intrusive actor list walked by
/// [`fn_800d9940`]; `0x800DAED0` writes the same global when it unlinks.
const W63_G_ACTOR_LIST_TAIL: usize = SDA_BASE - 0x5664;

/// `r13 - 0x5986` -> `0x80429CFA`: filter byte; bit 3 (=`8`) restricts a
/// search to actors whose `+0x38E` byte also has that bit clear.
const W63_G_SEARCH_FILTER: usize = SDA_BASE - 0x5986;

/// `r13 - 0x51D8` -> `0x8042A4A8`: handle passed to `0x8019BD20` when a
/// spin-jump counter reaches zero. `// unknown:` subsystem.
const W63_W_SDA_51D8: usize = SDA_BASE - 0x51D8;

/// `r2 - 0x6438` -> `0x8042CC48`: word copied into the saved-search global by
/// [`fn_800d9940`]. `// unknown:` role; the export only shows the load.
const W63_W_SDA2_6438: usize = SDA2_BASE - 0x6438;

/// `r2 - 0x6448` -> `0x8042CC38`: word stored at `+0x28` by
/// [`fn_800d98c0`]. `// unknown:` role.
const W63_W_SDA2_6448: usize = SDA2_BASE - 0x6448;

/// `0x8000FBF0`: destructor of one `0xC`-byte element, passed to the array
/// destructor by [`fn_800d54e0`].
const W63_FN_8000FBF0: usize = 0x8000_FBF0;

/// `0x800260C0`: destructor of one `0x38`-byte element, passed to the array
/// destructor by [`fn_800d54e0`].
const W63_FN_800260C0: usize = 0x8002_60C0;

/// `0x80069060(list, mask)`: unlinks a list object; called by
/// [`fn_800d54e0`] on `object + 4`.
const W63_FN_80069060: usize = 0x8006_9060;

/// `0x800690C0(list, mask, data, 0, 0x20)`: five-argument list setup reached
/// from [`fn_800d5580`]. The body is a thunk to `0x8016A7F0`.
const W63_FN_800690C0: usize = 0x8006_90C0;

/// `0x80167EC0(embedded, 0)`: release of the object at `+0x98`.
const W63_FN_80167EC0: usize = 0x8016_7EC0;

/// `0x80168D10`: base-object constructor run by [`fn_800d54a0`].
const W63_FN_80168D10: usize = 0x8016_8D10;

/// `0x8016A8E0`: animation/matrix setup run on `actor + 4` by
/// [`fn_800d5580`]. `0x800D08A0` is a 4-byte thunk to it, already covered by
/// `engine::frame`.
const W63_FN_8016A8E0: usize = 0x8016_A8E0;

/// `0x8019BD20(handle, 8)`: signal sent when a spin-jump counter reaches
/// zero in [`fn_800d8680`]. `// unknown:` exact subsystem.
const W63_FN_8019BD20: usize = 0x8019_BD20;

/// `0x801C0540(matrix_out, matrix, matrix_in)`: applies the 4x3 matrix built
/// by `0x801C0C60` in [`fn_800d6eb0`].
const W63_FN_801C0540: usize = 0x801C_0540;

/// `0x801C0C60(f64, f64, f64, out)`: builds a 48-byte scale matrix from three
/// doubles in [`fn_800d6eb0`].
const W63_FN_801C0C60: usize = 0x801C_0C60;

/// `0x800D57F0(actor, position, rotation, scale)`: rebuilds the actor's
/// transforms; called by [`fn_800d56f0`] with the arguments its callers pass.
const W63_FN_800D57F0: usize = 0x800D_57F0;

/// Calls `func(f64, f64, f64, usize) -> u32`.
///
/// [`fn_800d6eb0`] passes the same single-precision scale three times through
/// `(double)` casts, which the host models as `f32 as f64`.
#[inline]
unsafe fn call_f64_f64_f64_usize(addr: usize, a: f64, b: f64, c: f64, d: usize) -> u32 {
    let f: extern "C" fn(f64, f64, f64, usize) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d)
}

// ---------------------------------------------------------------------------
// Wave 63, cluster A: the `0x80303408` actor wrapper
// (`0x800D54A0..0x800D5790`)
// ---------------------------------------------------------------------------

/// Constructs one `0x80303408` actor wrapper in place.
///
/// In plain English: run the base-object constructor, install the
/// `0x80303408` vtable and hand the object back.
///
/// # Original
/// - Ghidra: `FUN_800d54a0` at `0x800D54A0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d54a0_FUN_800d54a0.c`
///
/// # Arguments
/// * `object` - storage for one array element. `0x800D5330` registers this
///   function as the constructor of two `0x38`-byte elements.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Runs `0x80168D10` and writes the vtable word at `+0x00`.
///
/// # Safety
/// `object` must have at least four writable bytes and the constructor at
/// `0x80168D10` must accept it in `r3`.
///
/// # Confidence
/// medium - the vtable store and the return are literal; the export prints
/// `0x80168D10` without operands, so the `r3 = object` receiver is
/// `// inferred:` from the element-constructor registration in `0x800D5330`.
pub unsafe fn fn_800d54a0(object: usize) -> usize {
    // observed: FUN_80168d10(); *param_1 = &DAT_80303408;
    let _ = platform::call1(W63_FN_80168D10, object as u32);
    write_word(object, 0, W63_VTABLE_80303408);
    object
}

/// Destructs one `0x80303408` actor wrapper.
///
/// In plain English: destroy the six `0xC`-byte elements at `+0x190`, release
/// the embedded object at `+0x98`, destroy the two `0x38`-byte elements at
/// `+0x28`, unlink the list node at `+0x04` and free the storage when the
/// caller's release flag is positive.
///
/// # Original
/// - Ghidra: `FUN_800d54e0` at `0x800D54E0` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d54e0_FUN_800d54e0.c`
///
/// # Arguments
/// * `object` - wrapper to destroy; `0` is accepted.
/// * `free_flag` - when positive, the storage is released through
///   `0x802B9280`.
///
/// # Returns
/// `object`, unchanged, matching the original `return param_1`.
///
/// # Side effects
/// Runs the two array destructors, the `+0x98` release, the list unlink and
/// possibly the storage free.
///
/// # Safety
/// `object` must be a live wrapper and every callee must be callable.
///
/// # Confidence
/// high - both array descriptors, the unlink mask and the release flag are
/// literal. The `object + 0x98 != 0` guard is vacuous and kept as observed.
pub unsafe fn fn_800d54e0(object: usize, free_flag: i32) -> usize {
    if object != 0 {
        // observed: array destructor (base + 0x190, 6 elements of 0xC bytes).
        let _ = platform::call4(
            FN_802DCA98,
            (object + 0x190) as u32,
            W63_FN_8000FBF0 as u32,
            0xC,
            6,
        );
        // observed: FUN_80167ec0(param_1 + 0x98, 0); the guard is vacuous.
        if object + 0x98 != 0 {
            let _ = platform::call2(W63_FN_80167EC0, (object + 0x98) as u32, 0);
        }
        // observed: array destructor (base + 0x28, 2 elements of 0x38 bytes).
        let _ = platform::call4(
            FN_802DCA98,
            (object + 0x28) as u32,
            W63_FN_800260C0 as u32,
            0x38,
            2,
        );
        let _ = platform::call2(W63_FN_80069060, (object + 4) as u32, 0xFFFF_FFFF);
        if free_flag > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Opens one `0x80303408` actor after its transforms were set up.
///
/// In plain English: store the two configuration words, initialise the list
/// node at `+0x04`, run the vtable slots `+0x10`, `+0x30` and `+0x14`, and
/// report success.
///
/// # Original
/// - Ghidra: `FUN_800d5580` at `0x800D5580` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5580_FUN_800d5580.c`
///
/// # Arguments
/// * `actor` - live wrapper.
/// * `flag` - byte stored at `+0x150`.
/// * `arg` - forwarded to vtable slot `+0x30`.
/// * `word` - word stored at `+0x180`.
///
/// # Returns
/// Always `0`, mirroring the literal `return 0`.
///
/// # Side effects
/// Writes `+0x180` and `+0x150`, initialises the list at `+0x04` through
/// `0x800690C0`, then runs `0x8016A8E0(+0x04)` and three virtual methods.
///
/// # Safety
/// `actor` must be a constructed wrapper whose vtable has live slots
/// `+0x10`, `+0x14` and `+0x30`.
///
/// # Confidence
/// high - every store, the five-argument list setup and the slot numbers are
/// literal; only the word read at `0x80377C48` is `// unknown:`.
pub unsafe fn fn_800d5580(actor: usize, flag: u8, arg: u32, word: u32) -> u32 {
    write_word(actor, 0x180, word);
    write_byte(actor, 0x150, flag);
    let _ = platform::call5(
        W63_FN_800690C0,
        (actor + 4) as u32,
        0xC000,
        read_global_word(W63_DATA_80377C48),
        0,
        0x20,
    );
    let _ = vcall0(actor, 0x10);
    let _ = platform::call1(W63_FN_8016A8E0, (actor + 4) as u32);
    let _ = vcall1(actor, 0x30, arg);
    let _ = vcall0(actor, 0x14);
    0
}

/// Copies a twelve-word transform into one actor and notifies it.
///
/// In plain English: copy the twelve words at `src` into `actor + 0xD8` and
/// run vtable slot `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_800d5680` at `0x800D5680` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5680_FUN_800d5680.c`
///
/// # Arguments
/// * `actor` - live wrapper.
/// * `src` - 48-byte source matrix, twelve big-endian words.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `actor + 0xD8..+0x104` and calls the vtable slot `+0x1C`.
///
/// # Safety
/// `src` must have 48 readable bytes and `actor` 0x108 writable bytes.
///
/// # Confidence
/// medium - the twelve stores and their offsets are literal; the export
/// prints the `+0x1C` slot without operands, so passing `actor` as its
/// receiver is `// inferred:` from the surrounding virtual-call shape.
pub unsafe fn fn_800d5680(actor: usize, src: usize) {
    let mut index = 0;
    while index < 12 {
        write_word(actor, 0xD8 + index * 4, read_word(src, index * 4));
        index += 1;
    }
    let _ = vcall0(actor, 0x1C);
}

/// Rebuilds one actor's transforms.
///
/// In plain English: forward the four arguments to `0x800D57F0` and run the
/// actor's vtable slot `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_800d56f0` at `0x800D56F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d56f0_FUN_800d56f0.c`
///
/// # Arguments
/// * `actor` - live wrapper.
/// * `position` - three-word position.
/// * `rotation` - three-halfword rotation.
/// * `scale` - three-float scale.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs the transform rebuild and the vtable notify at `+0x1C`.
///
/// # Safety
/// The three buffers must match the layouts used by `0x800D57F0`.
///
/// # Confidence
/// medium - the export prints both calls without operands; the four-argument
/// forwarding is `// inferred:` from the only call site, `0x800D6F40`, which
/// passes exactly this object/position/rotation/scale tuple, and from the
/// parameter list of `0x800D57F0`.
pub unsafe fn fn_800d56f0(actor: usize, position: usize, rotation: usize, scale: usize) {
    let _ = platform::call4(
        W63_FN_800D57F0,
        actor as u32,
        position as u32,
        rotation as u32,
        scale as u32,
    );
    let _ = vcall0(actor, 0x1C);
}

/// Reads one transform triple through the actor's vtable slot `+0x50`.
///
/// In plain English: let slot `+0x50` fill a 48-byte scratch buffer and copy
/// the words at buffer offsets `0xC`, `0x1C` and `0x2C` to `out`.
///
/// # Original
/// - Ghidra: `FUN_800d5790` at `0x800D5790` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5790_FUN_800d5790.c`
///
/// # Arguments
/// * `object` - actor whose vtable provides slot `+0x50`.
/// * `out` - three-word destination.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `out + 0x08`, `out + 0x00` and `out + 0x04`, in that order.
///
/// # Safety
/// `out` must have twelve writable bytes and slot `+0x50` must accept a
/// 48-byte buffer.
///
/// # Confidence
/// medium - the three copies and their buffer offsets are literal; the
/// three-word stride of the buffer suggests a concatenated vector layout, and
/// callers pass a third selector that this body does not read
/// (`// unknown:` whether the callee consumes it).
pub unsafe fn fn_800d5790(object: usize, out: usize) {
    let mut buffer = [0u8; 48];
    let base = buffer.as_mut_ptr() as usize;
    let _ = vcall1(object, 0x50, base as u32);
    write_word(out, 8, read_word(base, 0xC));
    write_word(out, 0, read_word(base, 0x2C));
    write_word(out, 4, read_word(base, 0x1C));
}

// ---------------------------------------------------------------------------
// Wave 63, cluster B: the `0x80318A80`/`0x80318A90` node wrappers
// (`0x800D6E00..0x800D70C0`). Each wrapper owns the actor pointer at `+0x04`
// and a ready word at `+0x08`, and forwards to the cluster A bodies above.
// ---------------------------------------------------------------------------

/// Destructs one `0x80318A80` node wrapper.
///
/// In plain English: install the `0x80318A80` vtable, release the owner
/// through its vtable slot `+0x08` with flag `1`, and free the wrapper when
/// the caller's release flag is positive.
///
/// # Original
/// - Ghidra: `FUN_800d6e00` at `0x800D6E00` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6e00_FUN_800d6e00.c`
///
/// # Arguments
/// * `node` - wrapper to destroy; `0` is accepted.
/// * `free_flag` - when positive, the storage is released through
///   `0x802B9280`.
///
/// # Returns
/// `node`, unchanged.
///
/// # Side effects
/// Writes the vtable, may run the owner's slot `+0x08` and may free `node`.
///
/// # Safety
/// `node` must be a live wrapper and `node + 4` a valid owner pointer.
///
/// # Confidence
/// high - the vtable store, the slot number, the `1` argument and the release
/// flag are literal.
pub unsafe fn fn_800d6e00(node: usize, free_flag: i32) -> usize {
    if node != 0 {
        let owner = read_word(node, 4) as usize;
        write_word(node, 0, W63_VTABLE_80318A80);
        if owner != 0 {
            let _ = vcall1(owner, 8, 1);
        }
        if free_flag > 0 {
            let _ = platform::call1(FN_802B9280, node as u32);
        }
    }
    node
}

/// Applies a per-player scale matrix to one node wrapper's actor.
///
/// In plain English: mark the wrapper ready, read the scale of the actor's
/// player from the `0x80370DA8` table, build a uniform scale matrix from it,
/// apply that matrix to `target`, and notify the actor through
/// [`fn_800d5680`].
///
/// # Original
/// - Ghidra: `FUN_800d6eb0` at `0x800D6EB0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6eb0_FUN_800d6eb0.c`
///
/// # Arguments
/// * `node` - wrapper whose actor pointer sits at `+0x04`.
/// * `target` - matrix destination passed to `0x801C0540` twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the ready word at `+0x08`, the scale matrix, and `target`'s twelve
/// words through [`fn_800d5680`].
///
/// # Safety
/// The actor at `+0x04` must be live, its `+0x152` byte must index the scale
/// table, and `target` must have 48 writable bytes.
///
/// # Confidence
/// medium - the ready store, the table read and both calls are literal; the
/// double conversion and the identity of `0x801C0540` (matrix apply) are
/// `// inferred:` from the 48-byte buffer it consumes.
pub unsafe fn fn_800d6eb0(node: usize, target: usize) {
    write_word(node, 8, 1);
    let owner = read_word(node, 4) as usize;
    let player = read_byte(owner, 0x152) as usize;
    let scale = read_const_f32(W63_TABLE_80370DA8 + player * 4) as f64;
    let mut matrix = [0u8; 48];
    let base = matrix.as_mut_ptr() as usize;
    let _ = call_f64_f64_f64_usize(W63_FN_801C0C60, scale, scale, scale, base);
    let _ = platform::call3(W63_FN_801C0540, target as u32, base as u32, target as u32);
    fn_800d5680(owner, target);
}

/// Applies a position/rotation/scale tuple to one node wrapper's actor.
///
/// In plain English: mark the wrapper ready, multiply the incoming scale by
/// the actor's per-player scale, copy the position and rotation into local
/// buffers, and hand all three to [`fn_800d56f0`].
///
/// # Original
/// - Ghidra: `FUN_800d6f40` at `0x800D6F40` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6f40_FUN_800d6f40.c`
///
/// # Arguments
/// * `node` - wrapper whose actor pointer sits at `+0x04`.
/// * `position` - three input words.
/// * `rotation` - three input halfwords.
/// * `scale` - three input floats, overwritten by the scaled result.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the ready word, scales `scale[0..3]` in place, then runs the
/// transform rebuild.
///
/// # Safety
/// `position` must have three readable words, `rotation` three readable
/// halfwords and `scale` three readable/writable floats.
///
/// # Confidence
/// high - the three factor multiplications, both copies and the call are
/// literal; only `+0x152`'s meaning as a player index is `// inferred:`.
pub unsafe fn fn_800d6f40(node: usize, position: usize, rotation: usize, scale: usize) {
    write_word(node, 8, 1);
    let owner = read_word(node, 4) as usize;
    let player = read_byte(owner, 0x152) as usize;
    let factor = read_const_f32(W63_TABLE_80370DA8 + player * 4);

    let mut scaled = [
        read_float(scale, 0) * factor,
        read_float(scale, 4) * factor,
        read_float(scale, 8) * factor,
    ];
    write_float(scale, 0, scaled[0]);
    write_float(scale, 4, scaled[1]);
    write_float(scale, 8, scaled[2]);

    let mut position_copy = [
        read_word(position, 0),
        read_word(position, 4),
        read_word(position, 8),
    ];
    let mut rotation_copy = [
        read_half(rotation, 0),
        read_half(rotation, 2),
        read_half(rotation, 4),
    ];
    fn_800d56f0(
        owner,
        position_copy.as_mut_ptr() as usize,
        rotation_copy.as_mut_ptr() as usize,
        scaled.as_mut_ptr() as usize,
    );
}

/// Constructs the small `0x80318A90` holder.
///
/// In plain English: install the `0x80318A90` vtable and initialise the word
/// at `+0x04` to `3`.
///
/// # Original
/// - Ghidra: `FUN_800d70a0` at `0x800D70A0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d70a0_FUN_800d70a0.c`
///
/// # Arguments
/// * `object` - storage for the two words. `0x801442B0` constructs it at
///   `owner + 0x2CE8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x00` and `+0x04`.
///
/// # Safety
/// `object` must have eight writable bytes.
///
/// # Confidence
/// high - both stores are literal; `// unknown:` the meaning of the `3` word.
pub unsafe fn fn_800d70a0(object: usize) {
    write_word(object, 0, W63_VTABLE_80318A90);
    write_word(object, 4, 3);
}

/// Stores one drawing mode and clears the four halfwords of the holder.
///
/// In plain English: write the mode word at `+0x08`, then zero the two
/// halfwords at `+0x10`/`+0x12` and the two at `+0xC`/`+0x0E`.
///
/// # Original
/// - Ghidra: `FUN_800d70c0` at `0x800D70C0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d70c0_FUN_800d70c0.c`
///
/// # Arguments
/// * `object` - holder built by [`fn_800d70a0`].
/// * `mode` - mode word, stored at `+0x08`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes five fields in the order the export prints them.
///
/// # Safety
/// `object` must have at least `0x14` writable bytes.
///
/// # Confidence
/// high - the five stores and their offsets are literal.
pub unsafe fn fn_800d70c0(object: usize, mode: u32) {
    write_word(object, 8, mode);
    write_half(object, 0x10, 0);
    write_half(object, 0x12, 0);
    write_half(object, 0xC, 0);
    write_half(object, 0xE, 0);
}

// ---------------------------------------------------------------------------
// Wave 63, cluster C: the spin-jump counter object (`0x800D85D0..0x800D8840`).
// One global instance at `r13 - 0x5880` holds a three-slot bitmask at `+0x04`
// and three counters at `+0x08/+0x0C/+0x10`; `0x800D8640`/`0x800D8780`
// mirror those fields into the saved copy at object `+0x84..+0x90`.
// ---------------------------------------------------------------------------

/// Publishes the single spin-jump counter object.
///
/// In plain English: install the `0x80318C30` vtable and store the object in
/// the global at `r13 - 0x5880`.
///
/// # Original
/// - Ghidra: `FUN_800d85d0` at `0x800D85D0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d85d0_FUN_800d85d0.c`
///
/// # Arguments
/// * `object` - storage for the counter object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the vtable word and [`W63_G_SPINJUMP`].
///
/// # Safety
/// `object` must outlive the global publication and have four writable bytes.
///
/// # Confidence
/// high - both stores are literal.
pub unsafe fn fn_800d85d0(object: usize) {
    write_word(object, 0, W63_VTABLE_80318C30);
    write_global_word(W63_G_SPINJUMP, object as u32);
}

/// Clears the spin-jump counter global and optionally frees the object.
///
/// In plain English: clear the global pointer, then free the object when the
/// caller's release flag is positive.
///
/// # Original
/// - Ghidra: `FUN_800d85f0` at `0x800D85F0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d85f0_FUN_800d85f0.c`
///
/// # Arguments
/// * `object` - counter object; `0` skips the global clear.
/// * `free_flag` - when positive, the storage is released through
///   `0x802B9280`.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Clears [`W63_G_SPINJUMP`] and may free the storage.
///
/// # Safety
/// `object` must be the published instance when it is non-zero.
///
/// # Confidence
/// high - the guard, the global clear and the release test are literal.
pub unsafe fn fn_800d85f0(object: usize, free_flag: i32) -> usize {
    if object != 0 {
        write_global_word(W63_G_SPINJUMP, 0);
        if free_flag > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Copies the saved spin-jump state from the global object into `object`.
///
/// In plain English: read the live object at `r13 - 0x5880` and copy its
/// `+0x84`, `+0x88`, `+0x8C` and `+0x90` words into `+0x04`, `+0x08`, `+0x0C`
/// and `+0x10`.
///
/// # Original
/// - Ghidra: `FUN_800d8640` at `0x800D8640` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8640_FUN_800d8640.c`
///
/// # Arguments
/// * `object` - destination record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four words.
///
/// # Safety
/// The live object must be published and `object` must have `0x14` writable
/// bytes.
///
/// # Confidence
/// high - the four loads and stores are literal, including the `+0x8C` to
/// `+0x0C` swap that the export prints out of order.
pub unsafe fn fn_800d8640(object: usize) {
    let live = read_global_word(W63_G_SPINJUMP) as usize;
    write_word(object, 4, read_word(live, 0x84));
    let middle = read_word(live, 0x88);
    write_word(object, 0xC, read_word(live, 0x8C));
    write_word(object, 8, middle);
    write_word(object, 0x10, read_word(live, 0x90));
}

/// Copies a record back into the live spin-jump object's saved fields.
///
/// In plain English: the exact inverse of [`fn_800d8640`] for the four saved
/// words at `+0x84..+0x90`.
///
/// # Original
/// - Ghidra: `FUN_800d8780` at `0x800D8780` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8780_FUN_800d8780.c`
///
/// # Arguments
/// * `object` - source record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four words of the live global object.
///
/// # Safety
/// The live object must be published and `object` must have `0x14` readable
/// bytes.
///
/// # Confidence
/// high - the four loads and stores are literal, including the `+0x0C` to
/// `+0x8C` swap.
pub unsafe fn fn_800d8780(object: usize) {
    let live = read_global_word(W63_G_SPINJUMP) as usize;
    write_word(live, 0x84, read_word(object, 4));
    let middle = read_word(object, 0xC);
    write_word(live, 0x88, read_word(object, 8));
    write_word(live, 0x8C, middle);
    write_word(live, 0x90, read_word(object, 0x10));
}

/// Tests one slot bit of the spin-jump activity mask.
///
/// In plain English: return the mask word at `+0x04` with only bit `slot`
/// left.
///
/// # Original
/// - Ghidra: `FUN_800d87b0` at `0x800D87B0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d87b0_FUN_800d87b0.c`
///
/// # Arguments
/// * `object` - counter object.
/// * `slot` - bit index `0..2`.
///
/// # Returns
/// The masked word, non-zero when the slot is active.
///
/// # Side effects
/// None.
///
/// # Safety
/// `object` must have a readable word at `+0x04`.
///
/// # Confidence
/// high - the load and the variable shift are literal; `wrapping_shl` keeps
/// the PowerPC shift-count masking.
pub unsafe fn fn_800d87b0(object: usize, slot: u32) -> u32 {
    read_word(object, 4) & 1u32.wrapping_shl(slot)
}

/// Reports whether the spin-jump activity mask is non-zero.
///
/// In plain English: return `(mask != 0)` as `1`/`0` using the export's
/// `(-mask | mask) >> 31` sign trick.
///
/// # Original
/// - Ghidra: `FUN_800d87d0` at `0x800D87D0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d87d0_FUN_800d87d0.c`
///
/// # Arguments
/// * `object` - counter object.
///
/// # Returns
/// `1` when any slot bit is set, `0` otherwise.
///
/// # Side effects
/// None.
///
/// # Safety
/// `object` must have a readable word at `+0x04`.
///
/// # Confidence
/// high - the idiom is the literal `(-x | x) >> 31` sequence.
pub unsafe fn fn_800d87d0(object: usize) -> u32 {
    let mask = read_word(object, 4);
    (mask.wrapping_neg() | mask) >> 31
}

/// Reads one slot counter.
///
/// In plain English: return the word at `+0x08 + slot * 4`.
///
/// # Original
/// - Ghidra: `FUN_800d87f0` at `0x800D87F0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d87f0_FUN_800d87f0.c`
///
/// # Arguments
/// * `object` - counter object.
/// * `slot` - counter index `0..2`.
///
/// # Returns
/// The counter word, in frames.
///
/// # Side effects
/// None.
///
/// # Safety
/// `object + 8 + slot * 4` must be readable.
///
/// # Confidence
/// high - the single indexed load is literal.
pub unsafe fn fn_800d87f0(object: usize, slot: u32) -> u32 {
    read_word(object, 8 + (slot as usize) * 4)
}

/// Clears one slot bit of the spin-jump activity mask.
///
/// In plain English: clear bit `slot` at `+0x04`.
///
/// # Original
/// - Ghidra: `FUN_800d8820` at `0x800D8820` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8820_FUN_800d8820.c`
///
/// # Arguments
/// * `object` - counter object.
/// * `slot` - bit index `0..2`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Read-modify-writes the word at `+0x04`.
///
/// # Safety
/// `object` must have a writable word at `+0x04`.
///
/// # Confidence
/// high - the mask arithmetic is literal.
pub unsafe fn fn_800d8820(object: usize, slot: u32) {
    let mask = read_word(object, 4);
    write_word(object, 4, mask & !1u32.wrapping_shl(slot));
}

/// Writes one slot counter.
///
/// In plain English: store `value` at `+0x08 + slot * 4`.
///
/// # Original
/// - Ghidra: `FUN_800d8840` at `0x800D8840` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8840_FUN_800d8840.c`
///
/// # Arguments
/// * `object` - counter object.
/// * `slot` - counter index `0..2`.
/// * `value` - new counter value, in frames.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word.
///
/// # Safety
/// `object + 8 + slot * 4` must be writable.
///
/// # Confidence
/// high - the single indexed store is literal.
pub unsafe fn fn_800d8840(object: usize, slot: u32, value: u32) {
    write_word(object, 8 + (slot as usize) * 4, value);
}

/// Ticks the three spin-jump counters and plays their countdown signals.
///
/// In plain English: for each active slot, decrement its counter; when the
/// new value is a whole multiple of sixty frames, play sound `0xAB` below
/// `0xB5` frames and sound `0xAA` at or above it. When a counter reaches
/// zero, send signal `8` to the handle at `r13 - 0x51D8` and clear the slot
/// bit. Every active counter is then stored back.
///
/// # Original
/// - Ghidra: `FUN_800d8680` at `0x800D8680` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8680_FUN_800d8680.c`
///
/// # Arguments
/// * `object` - counter object built by [`fn_800d85d0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the three counters, clears slot bits, and emits sound-system
/// calls.
///
/// # Safety
/// `object` must be the live counter object and the sound system globals must
/// be readable.
///
/// # Confidence
/// high - the decrement, the `value % 60 == 0` test, both sound ids and the
/// zero path are literal; `// unknown:` what `0x8019BD20(handle, 8)` does.
pub unsafe fn fn_800d8680(object: usize) {
    let mut slot: u32 = 0;
    while slot < 3 {
        if fn_800d87b0(object, slot) != 0 {
            let mut value = fn_800d87f0(object, slot);
            if value != 0 {
                value -= 1;
            }
            if value == (value / 0x3C) * 0x3C {
                if value < 0xB5 {
                    if value != 0 {
                        let _ = platform::call3(
                            FN_80195380,
                            read_global_word(G_SOUND_SYSTEM),
                            0xAB,
                            1,
                        );
                    }
                } else {
                    let _ =
                        platform::call3(FN_80195380, read_global_word(G_SOUND_SYSTEM), 0xAA, 1);
                }
            }
            if value == 0 {
                let _ = platform::call2(
                    W63_FN_8019BD20,
                    read_global_word(W63_W_SDA_51D8),
                    8,
                );
                fn_800d8820(object, slot);
            }
            fn_800d8840(object, slot, value);
        }
        slot += 1;
    }
}

// ---------------------------------------------------------------------------
// Wave 63, cluster D: the embedded actor search context (`0x800D9860..0x800D9AD0`).
// `0x80064110` constructs it at `owner + 0x2DC`; `0x800D98C0` binds it to the
// owner, `0x800D9940` fills the result from the global actor list and
// `0x800D9A30` confirms it.
// ---------------------------------------------------------------------------

/// Resets one embedded search context.
///
/// In plain English: install the `0x80318E70` vtable and clear the words at
/// `+0x10` and `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_800d9860` at `0x800D9860` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9860_FUN_800d9860.c`
///
/// # Arguments
/// * `context` - search context embedded in an actor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three words.
///
/// # Safety
/// `context` must have at least `0x1C` writable bytes.
///
/// # Confidence
/// high - the vtable store and both clears are literal; `// unknown:` the
/// roles of `+0x10` and `+0x18` before the later setters run.
pub unsafe fn fn_800d9860(context: usize) {
    write_word(context, 0, W63_VTABLE_80318E70);
    write_word(context, 0x10, 0);
    write_word(context, 0x18, 0);
}

/// Destructs one embedded search context.
///
/// In plain English: when the context is live and the caller's release flag
/// is positive, free the storage, then return the pointer.
///
/// # Original
/// - Ghidra: `FUN_800d9880` at `0x800D9880` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9880_FUN_800d9880.c`
///
/// # Arguments
/// * `context` - search context; `0` is accepted.
/// * `free_flag` - when positive, the storage is released through
///   `0x802B9280`.
///
/// # Returns
/// `context`, unchanged.
///
/// # Side effects
/// May free the storage.
///
/// # Safety
/// `context` must be a live context when non-zero.
///
/// # Confidence
/// high - the two guards and the release call are literal.
pub unsafe fn fn_800d9880(context: usize, free_flag: i32) -> usize {
    if context != 0 && free_flag > 0 {
        let _ = platform::call1(FN_802B9280, context as u32);
    }
    context
}

/// Binds one search context to its owner.
///
/// In plain English: store the owner and the owner's transform pointer
/// (`owner + 0xAC`), install the group byte and the shadow global, and clear
/// the two bytes at `+0x30`/`+0x31`.
///
/// # Original
/// - Ghidra: `FUN_800d98c0` at `0x800D98C0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d98c0_FUN_800d98c0.c`
///
/// # Arguments
/// * `context` - search context.
/// * `owner` - actor that owns the context.
/// * `group` - byte stored at `+0x2E`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x04`, `+0x08`, `+0x28`, `+0x2E`, `+0x30` and `+0x31`.
///
/// # Safety
/// `context` must have `0x32` writable bytes and `owner + 0xAC` must be a
/// valid transform address.
///
/// # Confidence
/// high - every offset and store is literal; `// unknown:` the word copied
/// from `0x8042CC38` into `+0x28`.
pub unsafe fn fn_800d98c0(context: usize, owner: usize, group: u8) {
    // observed: uVar1 = *(r2 - 0x6448) is loaded before the stores.
    let shadow = read_global_word(W63_W_SDA2_6448);
    write_word(context, 4, owner as u32);
    write_word(context, 8, (owner + 0xAC) as u32);
    write_byte(context, 0x2E, group);
    write_byte(context, 0x30, 0);
    write_word(context, 0x28, shadow);
    write_byte(context, 0x31, 0);
}

/// Stores the same word in the two link fields of a search context.
///
/// In plain English: used with `0` by the actor constructors to reset the
/// `+0x10`/`+0x14` pair.
///
/// # Original
/// - Ghidra: `FUN_800d98f0` at `0x800D98F0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d98f0_FUN_800d98f0.c`
///
/// # Arguments
/// * `context` - search context.
/// * `value` - word stored at both `+0x10` and `+0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes two words.
///
/// # Safety
/// `context` must have `0x18` writable bytes.
///
/// # Confidence
/// high - both stores are literal.
pub unsafe fn fn_800d98f0(context: usize, value: u32) {
    write_word(context, 0x10, value);
    write_word(context, 0x14, value);
}

/// Returns the current search link or its fallback.
///
/// In plain English: return the word at `+0x10` when set, otherwise the word
/// at `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_800d9900` at `0x800D9900` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9900_FUN_800d9900.c`
///
/// # Arguments
/// * `context` - search context.
///
/// # Returns
/// The first non-zero of the two link words, or `0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `context` must have `0x18` readable bytes.
///
/// # Confidence
/// high - the two loads and the branch are literal.
pub unsafe fn fn_800d9900(context: usize) -> u32 {
    let current = read_word(context, 0x10);
    if current != 0 {
        return current;
    }
    read_word(context, 0x14)
}

/// Reads masked flags from the actor targeted by a search context.
///
/// In plain English: when `+0x10` holds a target, return that target's
/// `+0x3C` flag word masked with `mask`; otherwise return `0`.
///
/// # Original
/// - Ghidra: `FUN_800d9920` at `0x800D9920` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9920_FUN_800d9920.c`
///
/// # Arguments
/// * `context` - search context.
/// * `mask` - flag mask, e.g. `0x200`.
///
/// # Returns
/// The masked flags, or `0` when no target is stored.
///
/// # Side effects
/// None.
///
/// # Safety
/// The target at `+0x10` must have a readable word at `+0x3C`.
///
/// # Confidence
/// high - the guard, the dereference and the mask are literal; `+0x3C` as an
/// actor flag word is `// inferred:` from the many `& 0x200`/`& 0x1000`
/// tests at the call sites.
pub unsafe fn fn_800d9920(context: usize, mask: u32) -> u32 {
    let target = read_word(context, 0x10) as usize;
    if target != 0 {
        return read_word(target, 0x3C) & mask;
    }
    0
}

/// Searches the global actor list for one candidate.
///
/// In plain English: reset the collaborator's result slot, walk the global
/// actor list from its tail, ask `0x802DCBFC` whether each actor matches,
/// and keep the last match. Bit 3 of the global filter byte at
/// `r13 - 0x5986` restricts matches to actors whose `+0x38E` byte also has
/// that bit clear. The saved result pointer at `r13 - 0x5670` is refreshed
/// from `0x8042CC38`, and `0x802DCDB8` is told whether anything was found.
///
/// # Original
/// - Ghidra: `FUN_800d9940` at `0x800D9940` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9940_FUN_800d9940.c`
/// - Removed: the modelled `0x802DCD6C`/`0x802DCDB8` calls. They are the
///   `_savegpr_25`/`_restgpr_25` compiler helpers (register spill/restore
///   only), so the collaborator and the search word are the incoming
///   `r3`/`r4`.
///
/// # Arguments
/// * `collaborator` - search collaborator (incoming `r3`): `+0x10`/`+0x14`
///   result words.
/// * `arg_3` - third argument of `0x802DCBFC` (incoming `r4`).
/// * `arg_a` - fourth argument of `0x802DCBFC`.
/// * `arg_b` - fifth argument of `0x802DCBFC`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two result globals, clears the collaborator's `+0x10`/`+0x14`
/// and stores the accepted actor in `+0x14`.
///
/// # Safety
/// The global list must be walkable and every callee must accept five
/// integer arguments.
///
/// # Confidence
/// high - the loop, the filter and every store are literal; the five register
/// arguments and the `0x80371810` descriptor are now modelled by
/// [`interface_call_desc5`], and the list link at `+0x08` plus the `+0x38E`
/// flag are `// inferred:` from `0x800DAED0` and `0x80064110`.
pub unsafe fn fn_800d9940(collaborator: usize, arg_3: u32, arg_a: u32, arg_b: u32) {
    write_global_word(
        W63_G_SEARCH_SAVED,
        read_global_word(W63_W_SDA2_6438),
    );
    let filter = read_global_byte(W63_G_SEARCH_FILTER);
    write_global_word(W63_G_SEARCH_RESULT, read_word(collaborator, 0x10));
    write_word(collaborator, 0x10, 0);

    let mut node = read_global_word(W63_G_ACTOR_LIST_TAIL) as usize;
    write_word(collaborator, 0x14, 0);

    while node != 0 {
        // observed 0x800D9994..0x800D99AC: r3 = collaborator, r4 = node,
        // r5 = arg_3, r6/r7 = arg_a/arg_b, r12 = 0x80371810.
        let matches = interface_call_desc5::<u32>(
            Descriptor::new(0x8037_1810),
            collaborator as u32,
            node as u32,
            arg_3,
            arg_a,
            arg_b,
        );
        if matches != 0 {
            let mut accepted = false;
            if (filter & 8) == 0 {
                accepted = true;
            } else {
                let owner = read_word(node, 4) as usize;
                if owner != 0 && ((filter & 8) & read_byte(owner, 0x38E)) == 0 {
                    accepted = true;
                }
            }
            if accepted {
                write_word(collaborator, 0x14, node as u32);
            }
        }
        node = read_word(node, 8) as usize;
    }
}

/// Confirms the candidate stored by [`fn_800d9940`].
///
/// In plain English: when the context holds a candidate, publish it in the
/// result global, and when the candidate's `+0x44` byte is set ask
/// `0x802DCBFC` whether it still qualifies. A positive answer returns `1`
/// without clearing the candidate; otherwise the candidate is dropped and
/// the body returns `0`.
///
/// # Original
/// - Ghidra: `FUN_800d9a30` at `0x800D9A30` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9a30_FUN_800d9a30.c`
///
/// # Arguments
/// * `context` - search context filled by [`fn_800d9940`].
/// * `arg_a` - second argument of `0x802DCBFC`.
/// * `arg_b` - third argument of `0x802DCBFC`.
/// * `arg_c` - fourth argument of `0x802DCBFC`.
///
/// # Returns
/// `1` when the candidate is confirmed, `0` otherwise.
///
/// # Side effects
/// Writes the result global and may clear `+0x14`.
///
/// # Safety
/// The candidate at `+0x14` must be a readable actor when non-zero.
///
/// # Confidence
/// high - every load, store and branch is literal; the five-register dispatch
/// now resolves the `0x80371810` descriptor (see [`fn_800d9940`]) and the
/// `+0x44` byte's meaning is `// inferred:` from the sibling search body.
pub unsafe fn fn_800d9a30(context: usize, arg_a: u32, arg_b: u32, arg_c: u32) -> u32 {
    let candidate = read_word(context, 0x14) as usize;
    if candidate != 0 {
        write_global_word(W63_G_SEARCH_RESULT, candidate as u32);
        if read_byte(candidate, 0x44) != 0 {
            // observed 0x800D9A6C..0x800D9A80: r3 = context, r4 = candidate,
            // r5/r6/r7 = arg_a/arg_b/arg_c, r12 = 0x80371810.
            let still_valid = interface_call_desc5::<u32>(
                Descriptor::new(0x8037_1810),
                context as u32,
                candidate as u32,
                arg_a,
                arg_b,
                arg_c,
            );
            if still_valid != 0 {
                return 1;
            }
        }
        write_word(context, 0x14, 0);
    }
    0
}

/// Inserts one search context into its owner's intrusive list.
///
/// In plain English: when the context has an owner at `+0x14`, walk the
/// owner's list at `+0x10` (linked through `+0x0C`) and insert the context at
/// its head unless it is already present. The context's `+0x10` becomes the
/// owner and its `+0x0C` the previous head.
///
/// # Original
/// - Ghidra: `FUN_800d9ad0` at `0x800D9AD0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9ad0_FUN_800d9ad0.c`
///
/// # Arguments
/// * `context` - search context to link.
///
/// # Returns
/// `1` when the context was inserted, `0` when it was already listed or has
/// no owner.
///
/// # Side effects
/// Writes the context's `+0x0C`/`+0x10` and the owner's `+0x10`.
///
/// # Safety
/// Every node reachable through the owner's list must be readable.
///
/// # Confidence
/// medium - the walk, the duplicate test and the four stores are literal;
/// the owner field at `+0x14` and the list head at owner `+0x10` are
/// `// inferred:` from `0x800DAED0`, which maintains the same head/tail pair
/// as globals.
pub unsafe fn fn_800d9ad0(context: usize) -> u32 {
    let owner = read_word(context, 0x14) as usize;
    if owner != 0 {
        let mut link = read_word(owner, 0x10) as usize;
        while link != 0 {
            if link == context {
                return 0;
            }
            link = read_word(link, 0xC) as usize;
        }
        write_word(context, 0x10, owner as u32);
        write_word(context, 0xC, read_word(owner, 0x10));
        write_word(owner, 0x10, context as u32);
        return 1;
    }
    0
}

// ---------------------------------------------------------------------------
// Wave 76: four more slices of the engine band.
//
// * `0x800D5B90..0x800D6D10` continues the `0x80303408` actor wrapper begun in
//   wave 63 and adds the two `0x80318A80` node constructors;
// * `0x800D7480..0x800D8510` is the `Wm_mr_spinjump` display object that
//   `0x800D8420` spawns by name;
// * `0x800D9AC0..0x800DAED0` is the intrusive actor list node the `0x800D9940`
//   search walks, plus three candidate filters;
// * `0x800DA650`, `0x800DC990` and `0x800DF860..0x800DF890` are four standalone
//   helpers.
// ---------------------------------------------------------------------------

/// `0x80318C20`: vtable installed by [`fn_800d81e0`] on the
/// `Wm_mr_spinjump` display object.
const W76_VTABLE_80318C20: u32 = 0x8031_8C20;

/// `0x80329958`: vtable stored at `+0x14` by [`fn_800d81e0`] on the embedded
/// animation interface of the spin-jump display object.
const W76_VTABLE_80329958: u32 = 0x8032_9958;

/// `0x80318E58`: vtable installed by [`fn_800dadb0`] and [`fn_800dae00`] on one
/// actor list node.
const W76_VTABLE_80318E58: u32 = 0x8031_8E58;

/// `0x80318C10`: the `"Wm_mr_spinjump"` model name passed to the spawn calls in
/// [`fn_800d8420`].
const W76_S_WM_MR_SPINJUMP: usize = 0x8031_8C10;

/// `0x80317E38`: first of the three 2-jump string tables indexed by the mode
/// word at `object + 0x168` in [`fn_800d6840`].
const W76_TABLE_80317E38: usize = 0x8031_7E38;

/// `0x80317E5C`: second 2-jump string table used by [`fn_800d6840`] for kind 8.
const W76_TABLE_80317E5C: usize = 0x8031_7E5C;

/// `0x80317E80`: third 2-jump string table used by [`fn_800d6840`] for kind 9.
const W76_TABLE_80317E80: usize = 0x8031_7E80;

/// `r2 - 0x6568` -> `0x8042CAF8`: first `f32` target of the mode-0/2 setup in
/// [`fn_800d82a0`]. `// unknown:` the value is not recovered, so it is read.
const W76_SDA2_6568: usize = SDA2_BASE - 0x6568;

/// `r2 - 0x6564` -> `0x8042CAFC`: step paired with [`W76_SDA2_6568`].
const W76_SDA2_6564: usize = SDA2_BASE - 0x6564;

/// `r2 - 0x6570` -> `0x8042CAF0`: first target of the mode-1/3/4/5 setup in
/// [`fn_800d82a0`] and the initial word written to `+0x10` by [`fn_800d81e0`].
const W76_SDA2_6570: usize = SDA2_BASE - 0x6570;

/// `r2 - 0x6560` -> `0x8042CB00`: step used by every [`fn_800d82a0`] mode.
const W76_SDA2_6560: usize = SDA2_BASE - 0x6560;

/// `r2 - 0x655C` -> `0x8042CB04`: mode-1 target for the `+0x13C` field.
const W76_SDA2_655C: usize = SDA2_BASE - 0x655C;

/// `r2 - 0x656C` -> `0x8042CAF4`: initial word written to `+0x13C` by
/// [`fn_800d81e0`] and the mode-2/3/4/5 target for the same field.
const W76_SDA2_656C: usize = SDA2_BASE - 0x656C;

/// `r2 - 0x6578` -> `0x8042CAE8`: the deadzone constant of [`fn_800d80d0`].
/// `// unknown:` value; the sign-symmetric branches suggest zero.
const W76_SDA2_6578: usize = SDA2_BASE - 0x6578;

/// `r2 - 0x6650` -> `0x8042CA30`: halfword initialised into `+0x0C` by
/// [`fn_800d81e0`] and used as the mode-4 step in [`fn_800d82a0`].
const W76_SDA2_6650: usize = SDA2_BASE - 0x6650;

/// `r2 - 0x65E8` -> `0x8042CA98`: upper bucket edge of [`fn_800d5d10`].
const W76_SDA2_65E8: usize = SDA2_BASE - 0x65E8;

/// `r2 - 0x65EC` -> `0x8042CA94`: lower bucket edge of [`fn_800d5d10`].
const W76_SDA2_65EC: usize = SDA2_BASE - 0x65EC;

/// `r2 - 0x65F4` -> `0x8042CA8C`: animation-window constant for actions
/// `1..4`, `0x1A` and `0x6E` in [`fn_800d6ab0`].
const W76_SDA2_65F4: usize = SDA2_BASE - 0x65F4;

/// `r2 - 0x65C4` -> `0x8042CABC`: second animation-window constant of the same
/// action group.
const W76_SDA2_65C4: usize = SDA2_BASE - 0x65C4;

/// `r2 - 0x65C0` -> `0x8042CAC0`: first constant of action `0x85`.
const W76_SDA2_65C0: usize = SDA2_BASE - 0x65C0;

/// `r2 - 0x65BC` -> `0x8042CAC4`: second constant of action `0x85`.
const W76_SDA2_65BC: usize = SDA2_BASE - 0x65BC;

/// `r2 - 0x6608` -> `0x8042CA78`: shared first constant of actions `0x83` and
/// `0x8E`.
const W76_SDA2_6608: usize = SDA2_BASE - 0x6608;

/// `r2 - 0x65B8` -> `0x8042CAC8`: second constant of actions `0x83` and `0x8E`.
const W76_SDA2_65B8: usize = SDA2_BASE - 0x65B8;

/// `r2 - 0x65B4` -> `0x8042CACC`: second constant of action `0x8E`.
const W76_SDA2_65B4: usize = SDA2_BASE - 0x65B4;

/// `r2 - 0x65B0` -> `0x8042CAD0`: fourth constant of action `0x8E`.
const W76_SDA2_65B0: usize = SDA2_BASE - 0x65B0;

/// `r2 - 0x6448` -> `0x8042CC38`: the threshold of [`fn_800da650`]; the same
/// SDA2 word is used as the "nearly zero" epsilon by the wave-63 search code.
const W76_SDA2_6448: usize = SDA2_BASE - 0x6448;

/// `0x800B2EE0(id)`: sound/effect helper called by [`fn_800d5d10`]. `// unknown:`
/// the returned word is treated as an unsigned frame count.
const W76_FN_800B2EE0: usize = 0x800B_2EE0;

/// `0x800B3E50(handle, flag)`: display switch driven by the mode word at
/// `object + 0x180` in [`fn_800d5b90`].
const W76_FN_800B3E50: usize = 0x800B_3E50;

/// `0x802E19F8(destination, name, 0x20)`: copies a 0x20-byte 2-jump record from
/// the string tables in [`fn_800d6840`].
const W76_FN_802E19F8: usize = 0x802E_19F8;

/// `0x8015F480(target, step, float*)`: moves one `f32` toward its target by at
/// most `step`; returns `1` when the target was reached.
const W76_FN_8015F480: usize = 0x8015_F480;

/// `0x8015F340(target, step, limit)`: halfword form of `0x8015F480`, used for
/// the `+0x0C` timer of [`fn_800d82a0`].
const W76_FN_8015F340: usize = 0x8015_F340;

/// `0x802D7AA0(interface)`: constructor of the animation interface embedded at
/// `object + 0x14` in [`fn_800d81e0`].
const W76_FN_802D7AA0: usize = 0x802D_7AA0;

/// `0x8016F130(source, out)`: reads the source position triple
/// `source + 0x0C/+0x1C/+0x2C` into a 12-byte buffer in [`fn_800d8420`].
const W76_FN_8016F130: usize = 0x8016_F130;

/// `0x8008FBA0(player, manager, name, 0, matrix, 0, colour)`: player-aware
/// variant of the spawn call in [`fn_800d8420`].
const W76_FN_8008FBA0: usize = 0x8008_FBA0;

/// `0x802B9240(size, data, align)`: allocation helper used by [`fn_800d6d10`].
const W76_FN_802B9240: usize = 0x802B_9240;

/// `0x800BCB80(handle, mode)`: constructor for modes `2..4` in
/// [`fn_800d6d10`].
const W76_FN_800BCB80: usize = 0x800B_CB80;

/// `0x800CA060(handle, mode)`: constructor for modes `0..1` in
/// [`fn_800d6d10`].
const W76_FN_800CA060: usize = 0x800C_A060;

/// `0x800FE1D0(handle, 5)`: constructor for mode `5` in [`fn_800d6d10`].
const W76_FN_800FE1D0: usize = 0x800F_E1D0;

/// `0x80237BA0(value)`: inverse-square-root approximation. [`fn_800da650`]
/// multiplies its result by `value` to obtain a square root.
const W76_FN_80237BA0: usize = 0x8023_7BA0;

/// `0x801690E0(frame, animation)`: reports whether `frame` lies inside the
/// animation window of the interface at `animation`.
const W76_FN_801690E0: usize = 0x8016_90E0;

/// `0x801680E0(animation)`: advances the animation interface at `object + 0x60`
/// in [`fn_800d5d00`].
const W76_FN_801680E0: usize = 0x8016_80E0;

/// `0x800DAE60(node)`: clears the link and flag fields of one actor list node.
const W76_FN_800DAE60: usize = 0x800D_AE60;

/// `0x800DAED0(node)`: unlinks one actor list node from the global list.
const W76_FN_800DAED0: usize = 0x800D_AED0;

/// `0x800D6D10(node, mode)`: selects the actor constructor for a node
/// mode.
const W76_FN_800D6D10: usize = 0x800D_6D10;

/// `0x800D8420(display, source)`: spawns the `Wm_mr_spinjump` model.
const W76_FN_800D8420: usize = 0x800D_8420;

/// `r13 - 0x5660` -> `0x8042A020`: head of the intrusive actor list; the
/// mirror of [`W63_G_ACTOR_LIST_TAIL`].
const W76_G_ACTOR_LIST_HEAD: usize = SDA_BASE - 0x5660;

/// Calls `func(f64, u32) -> u32`: the PowerPC ABI passes the double in `f1` and
/// the pointer in `r3`.
#[inline]
unsafe fn call_f64_u32(addr: usize, a: f64, b: u32) -> u32 {
    let f: extern "C" fn(f64, u32) -> u32 = core::mem::transmute(addr);
    f(a, b)
}

/// Calls `0x8015F480(target, step, current)`: moves `current` toward `target`
/// by at most `step` and returns `1` when it arrived.
#[inline]
unsafe fn approach_f32(target: f64, step: f64, current: usize) -> u32 {
    let f: extern "C" fn(f64, f64, u32) -> u32 = core::mem::transmute(W76_FN_8015F480);
    f(target, step, current as u32)
}

/// Calls virtual slot `slot` of `this_arg` with five integer arguments.
///
/// observed in `0x800D8420`: the manager interface keeps its vtable pointer at
/// `+0x00`, so the receiver is `this_arg` itself and the entry is at
/// `this_arg + 0xA0`.
#[inline]
unsafe fn vcall5(
    this_arg: usize,
    slot: usize,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    let f: extern "C" fn(u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(entry);
    f(this_arg as u32, a, b, c, d, e)
}

/// Reports whether `frame` lies in the animation window identified by the SDA2
/// `constant` for the interface at `object + 0x28`.
#[inline]
unsafe fn animation_in_window(object: usize, constant: usize) -> bool {
    call_f64_u32(
        W76_FN_801690E0,
        read_const_f32(constant) as f64,
        (object + 0x28) as u32,
    ) != 0
}

// ---------------------------------------------------------------------------
// Wave 76, cluster A: the rest of the `0x80303408` actor wrapper
// (`0x800D5B90..0x800D6D10`) and the `0x80318A80` node constructors.
// ---------------------------------------------------------------------------

/// Chooses the display variant for the mode word at `+0x180`.
///
/// In plain English: below mode 2 the handle is switched off, at mode 2 it is
/// switched on, and every other mode leaves it untouched.
///
/// # Original
/// - Ghidra: `FUN_800d5b90` at `0x800D5B90` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5b90_FUN_800d5b90.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
/// * `handle` - display handle forwarded to `0x800B3E50`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call `0x800B3E50` once with flag `0` or `1`.
///
/// # Safety
/// `object` must have a readable word at `+0x180` and `handle` must be a live
/// display handle.
///
/// # Confidence
/// high - the two comparisons and both calls are literal; only the meaning of
/// the mode values is inferred from the flags passed to `0x800B3E50`.
pub unsafe fn fn_800d5b90(object: usize, handle: u32) {
    let mode = read_word(object, 0x180);
    if mode < 2 {
        let _ = platform::call2(W76_FN_800B3E50, handle, 0);
        return;
    }
    if mode == 2 {
        let _ = platform::call2(W76_FN_800B3E50, handle, 1);
    }
}

/// Reports whether the current action still differs from the previous one.
///
/// In plain English: `+0x158` is a sentinel `-1` while no change is pending;
/// any other value is compared with the live action at `+0x154`.
///
/// # Original
/// - Ghidra: `FUN_800d5be0` at `0x800D5BE0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5be0_FUN_800d5be0.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
///
/// # Returns
/// `1` when `+0x158` is neither `-1` nor equal to `+0x154`, `0` otherwise.
///
/// # Side effects
/// None.
///
/// # Safety
/// `object` must have readable words at `+0x154` and `+0x158`.
///
/// # Confidence
/// high - both loads, the `-1` sentinel and the inequality are literal.
pub unsafe fn fn_800d5be0(object: usize) -> u32 {
    let pending = read_word(object, 0x158);
    if pending != 0xFFFF_FFFF && pending != read_word(object, 0x154) {
        return 1;
    }
    0
}

/// Advances the animation interface embedded at `+0x60`.
///
/// In plain English: a one-line forwarder to `0x801680E0`, which is the
/// no-argument virtual update of the interface at `object + 0x60`.
///
/// # Original
/// - Ghidra: `FUN_800d5d00` at `0x800D5D00` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5d00_FUN_800d5d00.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Runs the interface update, which is expected to advance one animation.
///
/// # Safety
/// `object + 0x60` must hold a live interface pointer.
///
/// # Confidence
/// high - the offset and the single call are literal.
pub unsafe fn fn_800d5d00(object: usize) {
    let _ = platform::call1(W76_FN_801680E0, (object + 0x60) as u32);
}

/// Picks the animation bucket of the current mode into `+0x168`.
///
/// In plain English: mode 0 stores the result of `0x800B2EE0(3)`; mode 1 takes
/// the result of `0x800B2EE0(10)` as a frame count and buckets it against the
/// two SDA2 thresholds; a value above 2 is clamped back to 0.
///
/// # Original
/// - Ghidra: `FUN_800d5d10` at `0x800D5D10` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5d10_FUN_800d5d10.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
/// * `mode` - `0` or `1`; any other value only triggers the final clamp.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the bucket word at `+0x168` up to twice.
///
/// # Safety
/// `object` must have a writable word at `+0x168`.
///
/// # Confidence
/// medium - the three stores and the clamp are literal; the export prints the
/// frame count through the `CONCAT44(0x43300000, value)` integer-to-double
/// idiom, so the comparison is reconstructed as `value as f32`, and the two
/// thresholds are read from their SDA2 addresses instead of being guessed.
pub unsafe fn fn_800d5d10(object: usize, mode: u32) {
    if mode == 0 {
        // observed: *(object + 0x168) = FUN_800b2ee0(3);
        let value = platform::call1(W76_FN_800B2EE0, 3);
        write_word(object, 0x168, value);
    } else if mode == 1 {
        let value = platform::call1(W76_FN_800B2EE0, 0xA);
        // observed: (double)CONCAT44(0x43300000, value) - 2^52 converts the
        // unsigned frame count to a double; the comparisons are on f32.
        let frames = value as f32;
        let low = read_const_f32(W76_SDA2_65EC);
        let high = read_const_f32(W76_SDA2_65E8);
        let bucket = if low <= frames {
            if high <= frames {
                2
            } else {
                1
            }
        } else {
            0
        };
        write_word(object, 0x168, bucket);
    }
    if read_word(object, 0x168) > 2 {
        write_word(object, 0x168, 0);
    }
}

/// Chooses and copies the 2-jump record for one action kind.
///
/// In plain English: when the mask word selected by `variant` has bit `0x200`
/// set, copy the 0x20-byte record for `kind` (5, 8 or 9) out of its string
/// table; otherwise remember the current mode at `+0x16C` and clear `+0x168`.
///
/// # Original
/// - Ghidra: `FUN_800d6840` at `0x800D6840` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6840_FUN_800d6840.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
/// * `kind` - selects the table: 5, 8 or 9.
/// * `out` - destination of the 0x20-byte record.
/// * `variant` - `1` reads the mask at `+0x164`, anything else `+0x160`.
///
/// # Returns
/// `1` when a record was copied, `0` when the mask bit was clear.
///
/// # Side effects
/// Copies 0x20 bytes, or writes `+0x16C` and `+0x168`.
///
/// # Safety
/// `out` must accept 0x20 bytes and `object` must have readable mask words.
///
/// # Confidence
/// high - both masks, the three table bases, the index word and the size are
/// literal in the export.
pub unsafe fn fn_800d6840(object: usize, kind: u32, out: u32, variant: u32) -> u32 {
    let mask_word = if variant == 1 {
        read_word(object, 0x164)
    } else {
        read_word(object, 0x160)
    };
    if mask_word & 0x200 != 0 {
        let index = read_word(object, 0x168) as usize;
        let table = match kind {
            5 => Some(W76_TABLE_80317E38),
            8 => Some(W76_TABLE_80317E5C),
            9 => Some(W76_TABLE_80317E80),
            _ => None,
        };
        if let Some(base) = table {
            let name = read_word(base, index * 4);
            let _ = platform::call3(W76_FN_802E19F8, out, name, 0x20);
        }
        return 1;
    }
    write_word(object, 0x16C, read_word(object, 0x168));
    write_word(object, 0x168, 0);
    0
}

/// Steps an interpolated float toward its target over the remaining frames.
///
/// In plain English: while the frame counter at `+0x18C` is non-zero, move the
/// value at `+0x184` toward the target at `+0x188` by `(current - target) /
/// remaining` and decrement the counter; at zero, snap to the target.
///
/// # Original
/// - Ghidra: `FUN_800d6a30` at `0x800D6A30` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6a30_FUN_800d6a30.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x184` (directly or through `0x8015F480`) and `+0x18C`.
///
/// # Safety
/// `object` must have readable/writable `f32` values at `+0x184`/`+0x188` and a
/// writable word at `+0x18C`.
///
/// # Confidence
/// medium - the branch, the two `f32` loads and the counter decrement are
/// literal; the export drops the destination pointer of `0x8015F480`, which is
/// `// inferred:` to be `+0x184` because that is the only field the body
/// interpolates.
pub unsafe fn fn_800d6a30(object: usize) {
    let remaining = read_word(object, 0x18C);
    if remaining == 0 {
        write_float(object, 0x184, read_float(object, 0x188));
        return;
    }
    let current = read_float(object, 0x184);
    let target = read_float(object, 0x188);
    // observed: (float)((float)current - (float)target) / (float)remaining.
    let step = ((current as f64 - target as f64) as f32) / (remaining as i32 as f32);
    let _ = approach_f32(target as f64, step as f64, object + 0x184);
    write_word(object, 0x18C, remaining.wrapping_sub(1));
}

/// Tests the action's animation window against the current frame.
///
/// In plain English: for the action id at `+0x154`, return whether the live
/// frame of the interface at `+0x28` is inside any of the SDA2 windows that
/// belong to that action.
///
/// # Original
/// - Ghidra: `FUN_800d6ab0` at `0x800D6AB0` (336 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6ab0_FUN_800d6ab0.c`
///
/// # Arguments
/// * `object` - `0x80303408` actor wrapper.
///
/// # Returns
/// `1` when the frame is inside one of the action's windows, `0` otherwise.
///
/// # Side effects
/// None beyond the reads performed by `0x801690E0`.
///
/// # Safety
/// The interface at `object + 0x28` must be a live animation object.
///
/// # Confidence
/// medium - the action groups and the number of window tests per group are
/// literal; the eight SDA2 values are read from their addresses, and the
/// window semantics come from the callee `0x801690E0`, not from this body.
pub unsafe fn fn_800d6ab0(object: usize) -> u32 {
    let action = read_word(object, 0x154);
    if action.wrapping_sub(1) < 4 || action == 0x1A || action == 0x6E {
        if animation_in_window(object, W76_SDA2_65F4)
            || animation_in_window(object, W76_SDA2_65C4)
        {
            return 1;
        }
    } else if action == 0x85 {
        if animation_in_window(object, W76_SDA2_65C0)
            || animation_in_window(object, W76_SDA2_65BC)
        {
            return 1;
        }
    } else if action == 0x83 {
        if animation_in_window(object, W76_SDA2_6608)
            || animation_in_window(object, W76_SDA2_65B8)
        {
            return 1;
        }
    } else if action == 0x8E
        && (animation_in_window(object, W76_SDA2_6608)
            || animation_in_window(object, W76_SDA2_65B4)
            || animation_in_window(object, W76_SDA2_65B8)
            || animation_in_window(object, W76_SDA2_65B0))
    {
        return 1;
    }
    0
}

/// Constructs one `0x80318A80` node in place.
///
/// In plain English: install the vtable, clear the actor pointer and the ready
/// word, then let `0x800D6D10` build the actor for the requested mode.
///
/// # Original
/// - Ghidra: `FUN_800d6cc0` at `0x800D6CC0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6cc0_FUN_800d6cc0.c`
///
/// # Arguments
/// * `node` - storage for one node.
/// * `mode` - actor kind forwarded to [`fn_800d6d10`].
///
/// # Returns
/// `node`, unchanged.
///
/// # Side effects
/// Writes `+0x00/+0x04/+0x08` and constructs the actor at `+0x04`.
///
/// # Safety
/// `node` must have at least twelve writable bytes.
///
/// # Confidence
/// high - the vtable, the two clears and both arguments of the constructor call
/// are literal.
pub unsafe fn fn_800d6cc0(node: usize, mode: u8) -> usize {
    write_word(node, 0, W63_VTABLE_80318A80);
    write_word(node, 4, 0);
    write_word(node, 8, 0);
    let _ = platform::call2(W76_FN_800D6D10, node as u32, mode as u32);
    node
}

/// Builds the actor of a `0x80318A80` node for one mode.
///
/// In plain English: allocate one of three fixed-size blocks (0x87C, 0x874 or
/// 0x404) depending on the mode and run the matching constructor; store the
/// result at `+0x04`, or zero when the allocation failed.
///
/// # Original
/// - Ghidra: `FUN_800d6d10` at `0x800D6D10` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6d10_FUN_800d6d10.c`
///
/// # Arguments
/// * `node` - node whose `+0x04` receives the actor.
/// * `mode` - `0..1`, `2..4` or `5`; other values build nothing.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates one block and writes `+0x04`.
///
/// # Safety
/// `node` must have a writable word at `+0x04` and the allocator must be live.
///
/// # Confidence
/// high - the three size constants, the tag word at `0x80377C48`, the four-byte
/// alignment and every mode branch are literal.
pub unsafe fn fn_800d6d10(node: usize, mode: u32) {
    if mode.wrapping_sub(2) < 3 {
        let tag = read_global_word(W63_DATA_80377C48);
        let handle = platform::call3(W76_FN_802B9240, 0x87C, tag, 4) as usize;
        let mut actor = 0;
        if handle != 0 {
            actor = platform::call2(W76_FN_800BCB80, handle as u32, mode);
        }
        write_word(node, 4, actor);
    } else if mode < 2 {
        let tag = read_global_word(W63_DATA_80377C48);
        let handle = platform::call3(W76_FN_802B9240, 0x874, tag, 4) as usize;
        let mut actor = 0;
        if handle != 0 {
            actor = platform::call2(W76_FN_800CA060, handle as u32, mode);
        }
        write_word(node, 4, actor);
    } else if mode == 5 {
        let tag = read_global_word(W63_DATA_80377C48);
        let handle = platform::call3(W76_FN_802B9240, 0x404, tag, 4) as usize;
        let mut actor = 0;
        if handle != 0 {
            actor = platform::call2(W76_FN_800FE1D0, handle as u32, 5);
        }
        write_word(node, 4, actor);
    }
}

// ---------------------------------------------------------------------------
// Wave 76, cluster B: the `Wm_mr_spinjump` display object
// (`0x800D7480..0x800D8510`). `0x800D81E0` constructs it, `0x800D8280` binds
// its owner, `0x800D82A0` selects one of six motion presets and
// `0x800D8420` spawns the model by name.
// ---------------------------------------------------------------------------

/// Clamps the value at `+0x5C` into the interval spanned by `+0x3C`/`+0x4C`.
///
/// In plain English: take the two bound floats, order them, and pull `+0x5C`
/// back inside when it has left the interval.
///
/// # Original
/// - Ghidra: `FUN_800d7480` at `0x800D7480` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7480_FUN_800d7480.c`
///
/// # Arguments
/// * `state` - object holding the two bounds and the clamped value.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes at most one `f32` to `+0x5C`.
///
/// # Safety
/// `state` must have readable floats at `+0x3C`, `+0x4C`, `+0x5C` and a
/// writable float at `+0x5C`.
///
/// # Confidence
/// high - the min/max ordering and the two clamp stores are literal.
pub unsafe fn fn_800d7480(state: usize) {
    let first = read_float(state, 0x4C);
    let second = read_float(state, 0x3C);
    let (low, high) = if first < second {
        (first, second)
    } else {
        (second, first)
    };
    let current = read_float(state, 0x5C);
    if current < low {
        write_float(state, 0x5C, low);
    } else if high < current {
        write_float(state, 0x5C, high);
    }
}

/// Stores six doubles as a 3x2 float transform in a caller-chosen order.
///
/// In plain English: the caller passes six scalars and the export places them
/// at indices 2, 3, 5, 0, 1 and 4, which is the element order the spin-jump
/// motion code expects.
///
/// # Original
/// - Ghidra: `FUN_800d80b0` at `0x800D80B0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d80b0_FUN_800d80b0.c`
///
/// # Arguments
/// * `a`..`f` - six scalars, truncated to `f32`.
/// * `out` - six-float destination.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes six floats.
///
/// # Safety
/// `out` must have at least 0x18 writable bytes.
///
/// # Confidence
/// high - every store and index is literal.
pub unsafe fn fn_800d80b0(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, out: usize) {
    write_float(out, 8, a as f32);
    write_float(out, 0xC, b as f32);
    write_float(out, 0x14, c as f32);
    write_float(out, 0, d as f32);
    write_float(out, 4, e as f32);
    write_float(out, 0x10, f as f32);
}

/// Advances the spin-jump motion state by one step.
///
/// In plain English: the first word is driven by the difference between the
/// second and third words scaled by the fourth; that result is deadzoned by the
/// sixth word, clamped to the fifth, and either added to the first word or
/// pulled back toward zero when it is inside the deadzone.
///
/// # Original
/// - Ghidra: `FUN_800d80d0` at `0x800D80D0` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d80d0_FUN_800d80d0.c`
///
/// # Arguments
/// * `state` - motion state; `+0x00` position, `+0x04` velocity, `+0x08`
///   damping, `+0x0C` step, `+0x10` clamp, `+0x14` deadzone.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x00` and `+0x04`.
///
/// # Safety
/// `state` must have six readable floats and two writable ones.
///
/// # Confidence
/// medium - every branch and store is literal; the constant at `r2 - 0x6578`
/// is read from its address, and the field roles are `// inferred:` from the
/// direction each value moves.
pub unsafe fn fn_800d80d0(state: usize) {
    let position = read_float(state, 0);
    let zero = read_const_f32(W76_SDA2_6578);
    let delta = read_float(state, 4) - position * read_float(state, 8);
    let step = read_float(state, 0xC);

    // observed: move `delta` toward zero by at most `step`.
    let deadzoned = if zero <= delta {
        let moved = delta - step;
        if moved <= zero {
            zero
        } else {
            moved
        }
    } else {
        let moved = delta + step;
        if zero <= moved {
            zero
        } else {
            moved
        }
    };

    // observed: clamp `deadzoned` to [-deadzone, +deadzone].
    let deadzone = read_float(state, 0x14);
    let mut velocity = deadzone;
    if deadzoned < deadzone {
        velocity = deadzoned;
        if deadzoned <= -deadzone {
            velocity = -deadzone;
        }
    }

    let clamp = read_float(state, 0x10);
    let mut next_position;
    if velocity < -clamp || clamp < velocity {
        next_position = position + velocity;
    } else if zero <= position {
        next_position = position - clamp;
        if next_position <= zero {
            velocity = zero;
            next_position = zero;
        }
    } else {
        next_position = position + clamp;
        if zero <= next_position {
            velocity = zero;
            next_position = zero;
        }
    }

    write_float(state, 0, next_position);
    write_float(state, 4, velocity);
}

/// Constructs the `Wm_mr_spinjump` display object in place.
///
/// In plain English: install the `0x80318C20` vtable, set the mode to 4, copy
/// two SDA2 initialisation words into `+0x0C`/`+0x10`, construct the animation
/// interface at `+0x14`, install its `0x80329958` vtable and clear the state
/// block up to `+0x13C`.
///
/// # Original
/// - Ghidra: `FUN_800d81e0` at `0x800D81E0` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d81e0_FUN_800d81e0.c`
///
/// # Arguments
/// * `object` - storage for one display object.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Writes the vtable at `+0x00`, `+0x04..+0x13C` and constructs `+0x14`.
///
/// # Safety
/// `object` must have at least 0x140 writable bytes.
///
/// # Confidence
/// medium - every store is literal; the three SDA2 initialisation words are
/// read from their addresses and their roles are `// unknown:`, as is the
/// meaning of the mode value 4.
pub unsafe fn fn_800d81e0(object: usize) -> usize {
    let initial = platform::read_u32(W76_SDA2_6570);
    let pair = platform::read_u32(W76_SDA2_6650);
    write_word(object, 0, W76_VTABLE_80318C20);
    write_word(object, 4, 0);
    write_word(object, 8, 4);
    write_half(object, 0xC, pair as u16);
    write_word(object, 0x10, initial);
    let _ = platform::call1(W76_FN_802D7AA0, (object + 0x14) as u32);
    let last = platform::read_u32(W76_SDA2_656C);
    write_word(object, 0x14, W76_VTABLE_80329958);
    write_word(object, 0x128, 0);
    write_word(object, 0x12C, 0);
    write_byte(object, 0x130, 0);
    write_byte(object, 0x131, 0);
    write_word(object, 0x134, 0);
    write_word(object, 0x138, 0);
    write_word(object, 0x13C, last);
    object
}

/// Binds the display object to its owner actor.
///
/// In plain English: when an owner is supplied, store it at `+0x04` and publish
/// the display object at `owner + 0x388`; always set the mode at `+0x08` to 4.
///
/// # Original
/// - Ghidra: `FUN_800d8280` at `0x800D8280` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8280_FUN_800d8280.c`
///
/// # Arguments
/// * `display` - spin-jump display object.
/// * `owner` - actor that owns it, or `0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `display + 0x04`, `owner + 0x388` and `display + 0x08`.
///
/// # Safety
/// `owner`, when non-zero, must have a writable word at `+0x388`.
///
/// # Confidence
/// high - both guarded stores and the unconditional mode store are literal.
pub unsafe fn fn_800d8280(display: usize, owner: usize) {
    if owner != 0 {
        write_word(display, 4, owner as u32);
        write_word(owner, 0x388, display as u32);
    }
    write_word(display, 8, 4);
}

/// Applies one of six motion presets to the display object.
///
/// In plain English: mode selects a step/limit pair for the `+0x0C` timer and
/// two target values for `+0x10`/`+0x13C`; modes 0, 1 and 2 animate toward the
/// targets, while modes 3, 4 and 5 write them directly. The preset is always
/// followed by the model refresh at `0x800D8420`.
///
/// # Original
/// - Ghidra: `FUN_800d82a0` at `0x800D82A0` (376 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d82a0_FUN_800d82a0.c`
///
/// # Arguments
/// * `display` - spin-jump display object.
/// * `source` - actor passed through to `0x800D8420`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x0C`, `+0x10` and `+0x13C`, then runs `0x800D8420`.
///
/// # Safety
/// `display` must be constructed and `source` must be acceptable to the spawn
/// helper.
///
/// # Confidence
/// medium - all six modes, the step constants and the target stores are
/// literal; the eight SDA2 scalars are read from their addresses and the preset
/// meaning is `// inferred:` from which field each one feeds.
pub unsafe fn fn_800d82a0(display: usize, source: usize) {
    let timer = display + 0xC;
    let first = display + 0x10;
    let second = display + 0x13C;
    match read_word(display, 8) {
        0 => {
            let _ = platform::call3(W76_FN_8015F340, timer as u32, 0x4000, 0x800);
            let _ = approach_f32(
                read_const_f32(W76_SDA2_6568) as f64,
                read_const_f32(W76_SDA2_6564) as f64,
                first,
            );
            let _ = approach_f32(
                read_const_f32(W76_SDA2_6570) as f64,
                read_const_f32(W76_SDA2_6560) as f64,
                second,
            );
        }
        1 => {
            let _ = platform::call3(W76_FN_8015F340, timer as u32, 0x2000, 0x100);
            let _ = approach_f32(
                read_const_f32(W76_SDA2_6570) as f64,
                read_const_f32(W76_SDA2_6560) as f64,
                first,
            );
            let _ = approach_f32(
                read_const_f32(W76_SDA2_655C) as f64,
                read_const_f32(W76_SDA2_6560) as f64,
                second,
            );
        }
        2 => {
            let _ = platform::call3(W76_FN_8015F340, timer as u32, 0x4000, 0x800);
            let _ = approach_f32(
                read_const_f32(W76_SDA2_6568) as f64,
                read_const_f32(W76_SDA2_6564) as f64,
                first,
            );
            let _ = approach_f32(
                read_const_f32(W76_SDA2_656C) as f64,
                read_const_f32(W76_SDA2_6560) as f64,
                second,
            );
        }
        3 => {
            write_half(display, 0xC, 0);
            write_word(display, 0x10, platform::read_u32(W76_SDA2_6570));
            write_word(display, 0x13C, platform::read_u32(W76_SDA2_656C));
        }
        4 => {
            let step = platform::read_u32(W76_SDA2_6650) as u16 as i16 as i32;
            let _ = platform::call3(W76_FN_8015F340, timer as u32, step as u32, 0x80);
            write_word(display, 0x10, platform::read_u32(W76_SDA2_6570));
            write_word(display, 0x13C, platform::read_u32(W76_SDA2_656C));
        }
        5 => {
            let _ = platform::call3(W76_FN_8015F340, timer as u32, 0x4000, 0x800);
            write_word(display, 0x10, platform::read_u32(W76_SDA2_6570));
            write_word(display, 0x13C, platform::read_u32(W76_SDA2_656C));
        }
        _ => {}
    }
    let _ = platform::call2(W76_FN_800D8420, display as u32, source as u32);
}

/// Spawns the `Wm_mr_spinjump` model when the mode allows it.
///
/// In plain English: below mode 3, read the source position triple, build a
/// three-word colour from `+0x13C`, and spawn the named model either through
/// the manager interface directly or through the player-aware wrapper
/// `0x8008FBA0` when the owner's byte at `+0x38C` is `1`.
///
/// # Original
/// - Ghidra: `FUN_800d8420` at `0x800D8420` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8420_FUN_800d8420.c`
///
/// # Arguments
/// * `display` - spin-jump display object.
/// * `source` - actor whose position is copied into the spawn matrix.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Reads `source + 0x0C/+0x1C/+0x2C`, calls the manager interface slot `+0xA0`,
/// and may call the player-aware spawn `0x8008FBA0`.
///
/// # Safety
/// `display` must be constructed, `source` must expose the position triple and
/// the manager interface at `display + 0x14` must be live.
///
/// # Confidence
/// medium - the mode guard, the position read, the colour copies and the name
/// string are literal; the export drops the receiver of the `+0x6C` virtual
/// call that supplies the player index, so `owner + 0x60` is `// inferred:`
/// from the vtable load.
pub unsafe fn fn_800d8420(display: usize, source: usize) {
    if read_word(display, 8) >= 3 {
        return;
    }
    // 24-byte scratch buffer; 0x8016F130 writes the first three words.
    let mut position = [0u32; 6];
    let _ = platform::call2(
        W76_FN_8016F130,
        source as u32,
        position.as_mut_ptr() as u32,
    );
    let colour_word = read_word(display, 0x13C);
    let colour = [colour_word; 3];
    let owner = read_word(display, 4) as usize;
    if owner == 0 || read_byte(owner, 0x38C) != 1 {
        let _ = vcall5(
            display + 0x14,
            0xA0,
            W76_S_WM_MR_SPINJUMP as u32,
            0,
            position.as_mut_ptr() as u32,
            0,
            colour.as_ptr() as u32,
        );
    } else {
        // observed: the player index comes from a virtual call on the object
        // whose vtable pointer is stored at owner + 0x60.
        let vtable = read_word(owner, 0x60) as usize;
        let entry = read_word(vtable, 0x6C) as usize;
        let provider = platform::call1(entry, (owner + 0x60) as u32) as usize;
        let player = read_byte(provider, 0) as u32;
        let _ = platform::call7(
            W76_FN_8008FBA0,
            player,
            (display + 0x14) as u32,
            W76_S_WM_MR_SPINJUMP as u32,
            0,
            position.as_mut_ptr() as u32,
            0,
            colour.as_ptr() as u32,
        );
    }
}

/// Reports whether a value equals the mode word at `+0x08`.
///
/// In plain English: the export's `countLeadingZeros(value - mode) >> 5` is the
/// equality test, so the function returns `1` exactly when the two words match.
///
/// # Original
/// - Ghidra: `FUN_800d84f0` at `0x800D84F0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d84f0_FUN_800d84f0.c`
///
/// # Arguments
/// * `display` - spin-jump display object.
/// * `value` - candidate mode.
///
/// # Returns
/// `1` when `value == object[+0x08]`, `0` otherwise.
///
/// # Side effects
/// None.
///
/// # Safety
/// `display` must have a readable word at `+0x08`.
///
/// # Confidence
/// high - the subtraction and the `countLeadingZeros >> 5` idiom are literal.
pub unsafe fn fn_800d84f0(display: usize, value: u32) -> u32 {
    value.wrapping_sub(read_word(display, 8)).leading_zeros() >> 5
}

/// Subtracts from the halfword timer and switches to mode 4.
///
/// In plain English: `+0x0C` is a signed frame counter; subtract `amount`,
/// store the result, clamp a negative result to zero, then force mode 4.
///
/// # Original
/// - Ghidra: `FUN_800d8510` at `0x800D8510` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8510_FUN_800d8510.c`
///
/// # Arguments
/// * `display` - spin-jump display object.
/// * `amount` - frames to subtract.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x0C` (possibly twice) and `+0x08`.
///
/// # Safety
/// `display` must have a writable halfword at `+0x0C` and word at `+0x08`.
///
/// # Confidence
/// high - the signed subtraction, the clamp store and the mode store are
/// literal.
pub unsafe fn fn_800d8510(display: usize, amount: i16) {
    let remaining = (read_half(display, 0xC) as i16).wrapping_sub(amount);
    write_half(display, 0xC, remaining as u16);
    if remaining < 0 {
        write_half(display, 0xC, 0);
    }
    write_word(display, 8, 4);
}

// ---------------------------------------------------------------------------
// Wave 76, cluster C: the intrusive actor list node that the `0x800D9940`
// search walks (`0x800D9AC0..0x800DAED0`) and the three candidate filters next
// to it.
// ---------------------------------------------------------------------------

/// Clears the cached target of an actor.
///
/// In plain English: one store, `+0x14 = 0`.
///
/// # Original
/// - Ghidra: `FUN_800d9ac0` at `0x800D9AC0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9ac0_FUN_800d9ac0.c`
///
/// # Arguments
/// * `actor` - object whose cached target is cleared.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word.
///
/// # Safety
/// `actor` must have a writable word at `+0x14`.
///
/// # Confidence
/// high - the single store is literal.
pub unsafe fn fn_800d9ac0(actor: usize) {
    write_word(actor, 0x14, 0);
}

/// Stores or masks the halfword at `+0x2C`.
///
/// In plain English: when bit `0x10` of `flags` is set the halfword is cleared,
/// otherwise it receives `value`.
///
/// # Original
/// - Ghidra: `FUN_800d9b60` at `0x800D9B60` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9b60_FUN_800d9b60.c`
///
/// # Arguments
/// * `actor` - object owning the halfword.
/// * `value` - value stored when the flag is clear.
/// * `flags` - input flags; bit 4 forces zero.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one halfword.
///
/// # Safety
/// `actor` must have a writable halfword at `+0x2C`.
///
/// # Confidence
/// high - the bit test and both stores are literal.
pub unsafe fn fn_800d9b60(actor: usize, value: u16, flags: u32) {
    if flags & 0x10 != 0 {
        write_half(actor, 0x2C, 0);
    } else {
        write_half(actor, 0x2C, value);
    }
}

/// Tests whether two actor records belong to the same group and team.
///
/// In plain English: accept the candidate when the two group bytes overlap, the
/// two team bytes are equal, and the two owners differ.
///
/// # Original
/// - Ghidra: `FUN_800d9d00` at `0x800D9D00` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9d00_FUN_800d9d00.c`
///
/// # Arguments
/// * `actor` - the querying actor (`+0x2E` group, `+0x32` team, `+0x04`
///   owner).
/// * `candidate` - the other record (`+0x45` group, `+0x4A` team, `+0x04`
///   owner).
///
/// # Returns
/// `1` when all three tests pass, `0` otherwise.
///
/// # Side effects
/// None.
///
/// # Safety
/// Both objects must expose the cited bytes/words.
///
/// # Confidence
/// high - the mask, the equality and the owner inequality are literal; the
/// field names are `// inferred:` from the symmetry between the two records.
pub unsafe fn fn_800d9d00(actor: usize, candidate: usize) -> u32 {
    if read_byte(actor, 0x2E) & read_byte(candidate, 0x45) != 0
        && read_byte(actor, 0x32) == read_byte(candidate, 0x4A)
        && read_word(actor, 4) != read_word(candidate, 4)
    {
        return 1;
    }
    0
}

/// Rejects an owner that is in one of two protected modes.
///
/// In plain English: when the `0x200` flag is present and the owner's byte at
/// `+0x38C` is neither 1 nor 2, return `1`; otherwise `0`.
///
/// # Original
/// - Ghidra: `FUN_800d9d40` at `0x800D9D40` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9d40_FUN_800d9d40.c`
///
/// # Arguments
/// * `actor` - object whose `+0x04` word is the owner pointer.
/// * `flags` - input flags; bit 9 enables the test.
///
/// # Returns
/// `1` when the test fires, `0` otherwise.
///
/// # Side effects
/// None.
///
/// # Safety
/// `actor + 0x04` must be a readable owner pointer when bit 9 is set.
///
/// # Confidence
/// high - the flag test, the two mode comparisons and the owner load are
/// literal; the mode's meaning is `// unknown:`.
pub unsafe fn fn_800d9d40(actor: usize, flags: u32) -> u32 {
    if flags & 0x200 != 0 {
        let owner = read_word(actor, 4) as usize;
        let mode = read_byte(owner, 0x38C);
        if mode != 1 && mode != 2 {
            return 1;
        }
    }
    0
}

/// Constructs one actor list node in place.
///
/// In plain English: install the `0x80318E58` vtable, run the field reset and
/// clear the word at `+0x3C`.
///
/// # Original
/// - Ghidra: `FUN_800dadb0` at `0x800DADB0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dadb0_FUN_800dadb0.c`
///
/// # Arguments
/// * `node` - storage for one list node.
///
/// # Returns
/// `node`, unchanged.
///
/// # Side effects
/// Writes the vtable at `+0x00`, the fields cleared by `0x800DAE60` and `+0x3C`.
///
/// # Safety
/// `node` must have at least 0x40 writable bytes.
///
/// # Confidence
/// medium - the vtable store, the `+0x3C` clear and the return are literal; the
/// export drops the operand of `0x800DAE60`, which is `// inferred:` to be
/// `node` because it is the only object in scope.
pub unsafe fn fn_800dadb0(node: usize) -> usize {
    write_word(node, 0, W76_VTABLE_80318E58);
    let _ = platform::call1(W76_FN_800DAE60, node as u32);
    write_word(node, 0x3C, 0);
    node
}

/// Destructs one actor list node.
///
/// In plain English: reinstall the vtable, unlink the node from the global list
/// and free the storage when the release flag is positive.
///
/// # Original
/// - Ghidra: `FUN_800dae00` at `0x800DAE00` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dae00_FUN_800dae00.c`
///
/// # Arguments
/// * `node` - node to destroy; `0` is accepted.
/// * `free_flag` - when positive, the storage is released.
///
/// # Returns
/// `node`, unchanged.
///
/// # Side effects
/// Unlinks from the global list and may free the storage.
///
/// # Safety
/// `node` must be a live list node when non-zero.
///
/// # Confidence
/// medium - the vtable store, the unlink call and the release guard are
/// literal; the operand of `0x800DAED0` is dropped by the export and is
/// `// inferred:` to be `node`.
pub unsafe fn fn_800dae00(node: usize, free_flag: i32) -> usize {
    if node != 0 {
        write_word(node, 0, W76_VTABLE_80318E58);
        let _ = platform::call1(W76_FN_800DAED0, node as u32);
        if free_flag > 0 {
            let _ = platform::call1(FN_802B9280, node as u32);
        }
    }
    node
}

/// Clears the link and flag fields of one list node.
///
/// In plain English: zero the two link words at `+0x08`/`+0x0C`, the word at
/// `+0x10` and the three bytes at `+0x43`, `+0x44` and `+0x4A`.
///
/// # Original
/// - Ghidra: `FUN_800dae60` at `0x800DAE60` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dae60_FUN_800dae60.c`
///
/// # Arguments
/// * `node` - node to reset.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three words and three bytes.
///
/// # Safety
/// `node` must have the cited writable fields.
///
/// # Confidence
/// high - every store and offset is literal.
pub unsafe fn fn_800dae60(node: usize) {
    write_word(node, 8, 0);
    write_word(node, 0xC, 0);
    write_word(node, 0x10, 0);
    write_byte(node, 0x43, 0);
    write_byte(node, 0x4A, 0);
    write_byte(node, 0x44, 0);
}

/// Unlinks one node from the global intrusive actor list.
///
/// In plain English: when the node's linked flag at `+0x44` is set, patch the
/// neighbours at `+0x08`/`+0x0C`, update the global head/tail when the node was
/// an end, then clear the node.
///
/// # Original
/// - Ghidra: `FUN_800daed0` at `0x800DAED0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800daed0_FUN_800daed0.c`
///
/// # Arguments
/// * `node` - list node to remove.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the neighbours' link words, the head/tail globals and the node
/// itself through `0x800DAE60`.
///
/// # Safety
/// Every neighbour must be a live list node.
///
/// # Confidence
/// high - both neighbour patches, both global updates and the `+0x44` guard are
/// literal; `+0x08`/`+0x0C` are `// inferred:` to be previous/next because of
/// the mirrored stores.
pub unsafe fn fn_800daed0(node: usize) {
    if read_byte(node, 0x44) == 0 {
        return;
    }
    let previous = read_word(node, 8) as usize;
    let next = read_word(node, 0xC) as usize;
    if previous == 0 {
        write_global_word(W76_G_ACTOR_LIST_HEAD, next as u32);
    } else {
        write_word(previous, 0xC, next as u32);
    }
    if next == 0 {
        write_global_word(W63_G_ACTOR_LIST_TAIL, previous as u32);
    } else {
        write_word(next, 8, previous as u32);
    }
    let _ = platform::call1(W76_FN_800DAE60, node as u32);
}

// ---------------------------------------------------------------------------
// Wave 76, cluster D: standalone helpers at `0x800DA650`, `0x800DC990` and
// `0x800DF860..0x800DF890`.
// ---------------------------------------------------------------------------

/// Returns `sqrt(a * a - b * b)` through the inverse-square-root approximation.
///
/// In plain English: form `a² - b²` in single precision; when it is above the
/// SDA2 threshold, take the reciprocal square root and multiply it back in,
/// which yields the square root; otherwise return the threshold.
///
/// # Original
/// - Ghidra: `FUN_800da650` at `0x800DA650` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800da650_FUN_800da650.c`
///
/// # Arguments
/// * `a` - first leg.
/// * `b` - second leg.
///
/// # Returns
/// The positive leg of the right triangle, or the threshold when the difference
/// is not positive enough.
///
/// # Side effects
/// None.
///
/// # Safety
/// No pointers are dereferenced; the SDA2 threshold must be readable.
///
/// # Confidence
/// medium - the arithmetic, the threshold read and the two-step square root are
/// literal; the identity of `0x80237BA0` as an inverse square root is
/// `// inferred:` from its own body (`1.0 / sqrt(x)` with a Newton step).
pub unsafe fn fn_800da650(a: f64, b: f64) -> f64 {
    let square = (a * a) as f32 - (b * b) as f32;
    let mut result = read_const_f32(W76_SDA2_6448) as f64;
    if result < square as f64 {
        let inverse = call_f64_ret_f64(W76_FN_80237BA0, square as f64);
        result = ((square as f64 * inverse) as f32) as f64;
    }
    result
}

/// Marks the wave-38 menu slot as routed.
///
/// In plain English: set the byte at `+0x55`, the route flag of one slot of the
/// `r13 - 0x5658` container.
///
/// # Original
/// - Ghidra: `FUN_800dc990` at `0x800DC990` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc990_FUN_800dc990.c`
///
/// # Arguments
/// * `slot` - one slot object of the four-slot container.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one byte.
///
/// # Safety
/// `slot` must have a writable byte at `+0x55`.
///
/// # Confidence
/// high - the single store is literal, and wave 38 already documents `+0x55` as
/// the route flag of the same slot layout.
pub unsafe fn fn_800dc990(slot: usize) {
    write_byte(slot, 0x55, 1);
}

/// Stores eight halfwords in the table order used by the caller.
///
/// In plain English: the export permutes the eight arguments into the output:
/// `[a7, a6, a3, a4, a5, a8, a9, a2]`.
///
/// # Original
/// - Ghidra: `FUN_800df860` at `0x800DF860` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df860_FUN_800df860.c`
///
/// # Arguments
/// * `out` - eight-halfword destination.
/// * `a2`..`a9` - values stored in the permuted order above.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight halfwords.
///
/// # Safety
/// `out` must have at least 0x10 writable bytes.
///
/// # Confidence
/// high - every store and index is literal.
pub unsafe fn fn_800df860(
    out: usize,
    a2: u16,
    a3: u16,
    a4: u16,
    a5: u16,
    a6: u16,
    a7: u16,
    a8: u16,
    a9: u16,
) {
    write_half(out, 0, a7);
    write_half(out, 2, a6);
    write_half(out, 4, a3);
    write_half(out, 6, a4);
    write_half(out, 8, a5);
    write_half(out, 0xA, a8);
    write_half(out, 0xC, a9);
    write_half(out, 0xE, a2);
}

/// Advances a signed offset/velocity record by one integer step.
///
/// In plain English: subtract one `velocity / divisor` step from the offset,
/// clamp it against the band at `+0x06`, clamp the result to the limit at
/// `+0x08`, wrap the velocity when the offset crosses the range at `+0x0A` and
/// report the crossing through a flag word.
///
/// # Original
/// - Ghidra: `FUN_800df890` at `0x800DF890` (252 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df890_FUN_800df890.c`
///
/// # Arguments
/// * `record` - eight-halfword record: `+0x00` offset, `+0x02` velocity,
///   `+0x04` divisor, `+0x06` band, `+0x08` limit, `+0x0A` range, `+0x0C`
///   wrap threshold.
///
/// # Returns
/// The new velocity, before the halfword truncation used for the store.
///
/// # Side effects
/// Writes `+0x00`, `+0x02` and the flag word at `+0x10`.
///
/// # Safety
/// `record + 0x00..+0x10` must be readable and writable, and `+0x04` must not
/// be zero.
///
/// # Confidence
/// medium - every branch, cast and store is literal; the record's role (an
/// integer oscillator) and the field names are `// inferred:` from the way the
/// values feed each other.
pub unsafe fn fn_800df890(record: usize) -> i32 {
    let velocity = read_half(record, 2) as i16 as i32;
    let divisor = read_half(record, 4) as i16 as i32;
    let mut offset = (read_half(record, 0) as i16 as i32) + ((-velocity / divisor) as i16 as i32);

    let band = read_half(record, 6) as i16 as i32;
    if (offset as i16) < 0 {
        offset += band;
        if (offset as i16) > 0 {
            offset = 0;
        }
    } else {
        offset -= band;
        if (offset as i16) < 0 {
            offset = 0;
        }
    }

    let limit = read_half(record, 8) as i16 as i32;
    let mut clipped = limit;
    if (offset as i16) as i32 <= limit {
        clipped = offset;
        if ((offset as i16) as i32) < -limit {
            clipped = -limit;
        }
    }

    let range = read_half(record, 0xA) as i16 as i32;
    let wrap = read_half(record, 0xC) as i16 as i32;
    let mut next_velocity;
    let mut hit = 0u32;
    if (-range) < ((clipped as i16) as i32) && ((clipped as i16) as i32) < range {
        if velocity < 0 {
            next_velocity = velocity + range;
            if (-wrap) < ((next_velocity as i16) as i32) {
                next_velocity = 0;
                clipped = 0;
                hit = 1;
            }
        } else {
            next_velocity = velocity - range;
            if ((next_velocity as i16) as i32) < wrap {
                next_velocity = 0;
                clipped = 0;
                hit = 1;
            }
        }
    } else {
        next_velocity = velocity + clipped;
    }

    write_half(record, 0, clipped as i16 as u16);
    write_half(record, 2, next_velocity as i16 as u16);
    write_word(record, 0x10, hit);
    next_velocity
}

// ---------------------------------------------------------------------------
// Wave 78: boot-screen queries, destructors and `dScBoot_c` state handlers
// (`0x8015BB90..0x8015DAA0`)
// ---------------------------------------------------------------------------

/// `0x802B9280(object)`: `operator delete`-style release. The export prints the
/// call without a receiver, exactly like the wave-30 destructors in
/// [`crate::ui::menu`]; the object under destruction is passed here because it is
/// the only live operand.
const W78_FN_802B9280: usize = 0x802B_9280;

/// `0x8015F9F0(state_object, free)`: releases the state object embedded at
/// `object + 0x18`. Documented in [`crate::game::task`].
const W78_FN_8015F9F0: usize = 0x8015_F9F0;

/// `0x8008F620(id)`: input/readiness query.
///
/// `// unknown:` the shipped callee ignores `id`; the ids below are preserved as
/// observed so the call sites stay literal.
const W78_FN_8008F620: usize = 0x8008_F620;

/// `0x8001C240(receiver)`: brings up the `wipeKuppa` layout and returns `1` when
/// it is ready. The export drops the receiver operand at this call site.
const W78_FN_8001C240: usize = 0x8001_C240;

/// `0x8001BC60(receiver)`: `wipeDoor` layout bring-up, the second transition
/// scene. The export drops the receiver operand at this call site.
const W78_FN_8001BC60: usize = 0x8001_BC60;

/// `0x8001C760(receiver)`: `wipeFader` layout bring-up, the third transition
/// scene. The export drops the receiver operand at this call site.
const W78_FN_8001C760: usize = 0x8001_C760;

/// `0x8001B470(receiver)`: `wipeCircle` layout bring-up, the fourth transition
/// scene. The export drops the receiver operand at this call site.
const W78_FN_8001B470: usize = 0x8001_B470;

/// `0x8006A280()`: transition/reset helper; its body calls `0x8006A6A0(0)` and
/// sets the SDA byte at `r13 - 0x5904`.
const W78_FN_8006A280: usize = 0x8006_A280;

/// `0x80107F10()`: returns the live layout object (the "frame" object of
/// [`crate::engine::frame`]).
const W78_FN_80107F10: usize = 0x8010_7F10;

/// `0x80108160(layout, value)`: stores one byte in the layout's `+0x3D` field.
const W78_FN_80108160: usize = 0x8010_8160;

/// `0x80109340()`: returns the live display manager; its flag word is `+0x1D8`.
const W78_FN_80109340: usize = 0x8010_9340;

/// `0x800B0DB0(mode)`: resource-readiness test, called with `0` by this band.
const W78_FN_800B0DB0: usize = 0x800B_0DB0;

/// `0x800B0A20(frames)`: starts the 30-frame fade/transition timer.
const W78_FN_800B0A20: usize = 0x800B_0A20;

/// `0x800B0A90(frames)`: starts the alternative 30-frame transition timer.
const W78_FN_800B0A90: usize = 0x800B_0A90;

/// `0x801637D0(value, object)`: writes one double-converted `f32` at `object +
/// 0x08` and `+0x0C` (documented in [`crate::resource::registry`]).
const W78_FN_801637D0: usize = 0x8016_37D0;

/// `0x801644D0(descriptor)`: unpacks a three-word sound/layout descriptor and
/// forwards its float (documented in [`crate::resource::registry`]).
const W78_FN_801644D0: usize = 0x8016_44D0;

/// `r2 - 0x5490` -> `0x8042DBF0`: the `f32` written into the save/controller
/// scale by [`fn_8015ca50`] and [`fn_8015cd40`].
///
/// `// unknown:` the value is not recovered from the export.
const W78_SDA2_5490: usize = SDA2_BASE - 0x5490;

/// `r13 - 0x53A0` -> `0x8042A2E0`: scene reference count, incremented by
/// [`fn_8015bce0`] and decremented by [`fn_8015d080`]. The same word is
/// `G_SCENE_REFCOUNT` in [`crate::game::scene`].
const W78_G_SCENE_REFCOUNT: usize = SDA_BASE - 0x53A0;

/// `r13 - 0x6F44` -> `0x8042873C`: "scene finished" byte, cleared by
/// [`fn_8015ce80`] and set by [`fn_8015cb20`]/[`fn_8015d030`].
const W78_G_SCENE_FINISHED: usize = SDA_BASE - 0x6F44;

/// `r13 - 0x5218` -> `0x8042A468`: view object whose virtual slot `+0x20` tests
/// an event id; used by [`fn_8015cdf0`].
const W78_G_VIEW_OBJECT: usize = SDA_BASE - 0x5218;

/// `r13 - 0x5A40` -> `0x80429C40`: transition-scene slot tested by
/// [`fn_8015bf10`].
///
/// `// unknown:` the slot owner is not named in the export.
const W78_G_TRANSITION_SLOT_0: usize = SDA_BASE - 0x5A40;

/// `r13 - 0x5A48` -> `0x80429C38`: second transition-scene slot, tested by
/// [`fn_8015bf50`].
const W78_G_TRANSITION_SLOT_1: usize = SDA_BASE - 0x5A48;

/// `r13 - 0x5A38` -> `0x80429C48`: third transition-scene slot, tested by
/// [`fn_8015bf90`].
const W78_G_TRANSITION_SLOT_2: usize = SDA_BASE - 0x5A38;

/// `r13 - 0x5A50` -> `0x80429C30`: fourth transition-scene slot, tested by
/// [`fn_8015bfd0`].
const W78_G_TRANSITION_SLOT_3: usize = SDA_BASE - 0x5A50;

/// `0x80377408`: `dScBoot_c::StateID_ResetWait`.
const W78_D_BOOT_RESET_WAIT: u32 = 0x8037_7408;

/// `0x80377508`: `dScBoot_c::StateID_WiiStrapKeyWa...`.
const W78_D_BOOT_WII_STRAP_KEY_WAIT: u32 = 0x8037_7508;

/// `0x80377588`: `dScBoot_c::StateID_WiiStrapFadeO...`.
const W78_D_BOOT_WII_STRAP_FADE_OUT: u32 = 0x8037_7588;

/// `0x803775C8`: first `dScBoot_c::StateID_ControllerInf...`.
const W78_D_BOOT_CONTROLLER_INFO_0: u32 = 0x8037_75C8;

/// `0x80377608`: second `dScBoot_c::StateID_ControllerInf...`.
const W78_D_BOOT_CONTROLLER_INFO_1: u32 = 0x8037_7608;

/// `0x80377648`: third `dScBoot_c::StateID_ControllerInf...`.
const W78_D_BOOT_CONTROLLER_INFO_2: u32 = 0x8037_7648;

/// `0x80377688`: fourth `dScBoot_c::StateID_ControllerInf...`.
const W78_D_BOOT_CONTROLLER_INFO_3: u32 = 0x8037_7688;

/// `0x80377808`: `dScBoot_c::StateID_GoToErrorFade...`.
const W78_D_BOOT_GO_TO_ERROR_FADE: u32 = 0x8037_7808;

/// `0x80377988`: `dScBoot_c::StateID_ErrorWindowOn...`.
const W78_D_BOOT_ERROR_WINDOW_ON: u32 = 0x8037_7988;

/// `0x803779C8`: `dScBoot_c::StateID_ErrorInfinity...`.
const W78_D_BOOT_ERROR_INFINITY: u32 = 0x8037_79C8;

/// Normalises the readiness query answer with the observed
/// `countLeadingZeros(value - 1) >> 5` test, which is `1` only for `value == 1`.
#[inline]
fn exactly_one(value: u32) -> u32 {
    (value.wrapping_sub(1).leading_zeros() >> 5 != 0) as u32
}

/// Reports the readiness query `0x8008F620(10)`.
///
/// In plain English: asks the input/readiness subsystem about id `10` and
/// normalises the answer, so the result is `1` only when the query returned `1`.
///
/// # Original
/// - Ghidra: `FUN_8015bb90` at `0x8015BB90` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bb90_FUN_8015bb90.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the query answer passes the `clz(value - 1) >> 5` test, `0`
/// otherwise.
///
/// # Side effects
/// None beyond whatever `0x8008F620` observes.
///
/// # Safety
/// `0x8008F620` must be a callable function.
///
/// # Confidence
/// high - the id, the call and the `clz(value - 1) >> 5` test are literal.
pub unsafe fn fn_8015bb90() -> u32 {
    exactly_one(platform::call1(W78_FN_8008F620, 10))
}

/// Reports the readiness query `0x8008F620(0x0B)`.
///
/// In plain English: the twin of [`fn_8015bb90`] with id `0x0B`.
///
/// # Original
/// - Ghidra: `FUN_8015bbc0` at `0x8015BBC0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bbc0_FUN_8015bbc0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the query answer passes the `clz(value - 1) >> 5` test, `0`
/// otherwise.
///
/// # Side effects
/// None beyond whatever `0x8008F620` observes.
///
/// # Safety
/// `0x8008F620` must be a callable function.
///
/// # Confidence
/// high - the only difference from `0x8015BB90` is the literal id.
pub unsafe fn fn_8015bbc0() -> u32 {
    exactly_one(platform::call1(W78_FN_8008F620, 0xB))
}

/// Reports a two-stage readiness query: id `0x2C0`, then id `0x2E5`.
///
/// In plain English: the first query must return exactly `1`; only then is the
/// second id queried and normalised. Any other first answer yields `0`.
///
/// # Original
/// - Ghidra: `FUN_8015bbf0` at `0x8015BBF0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bbf0_FUN_8015bbf0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x2C0` answered `1` and `0x2E5` passed the `clz(value - 1) >> 5`
/// test, `0` otherwise.
///
/// # Side effects
/// None beyond the two queries.
///
/// # Safety
/// `0x8008F620` must be a callable function.
///
/// # Confidence
/// high - the equality against `1`, the second query and the final test are
/// literal.
pub unsafe fn fn_8015bbf0() -> u32 {
    if platform::call1(W78_FN_8008F620, 0x2C0) != 1 {
        return 0;
    }
    exactly_one(platform::call1(W78_FN_8008F620, 0x2E5))
}

/// Increments the scene reference count.
///
/// In plain English: one read-modify-write of the `s32` at `0x8042A2E0`.
///
/// # Original
/// - Ghidra: `FUN_8015bce0` at `0x8015BCE0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bce0_FUN_8015bce0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+1` over the scene reference count.
///
/// # Safety
/// The SDA word must be writable and must not be `INT32_MAX`.
///
/// # Confidence
/// high - the load, the `+1` and the store are literal.
pub unsafe fn fn_8015bce0() {
    let count = platform::read_i32(W78_G_SCENE_REFCOUNT);
    platform::write_i32(W78_G_SCENE_REFCOUNT, count.wrapping_add(1));
}

/// Runs the transition helper and reports success.
///
/// In plain English: calls `0x8006A280`, which resets the transition state and
/// sets its SDA byte, then always returns `1`.
///
/// # Original
/// - Ghidra: `FUN_8015bd10` at `0x8015BD10` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bd10_FUN_8015bd10.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Whatever `0x8006A280` does: `0x8006A6A0(0)` plus the byte store at
/// `r13 - 0x5904`.
///
/// # Safety
/// `0x8006A280` must be callable.
///
/// # Confidence
/// high - the call and the constant return are literal.
pub unsafe fn fn_8015bd10() -> u32 {
    platform::call0(W78_FN_8006A280);
    1
}

/// Reports whether the `wipeKuppa` transition is ready.
///
/// In plain English: the slot at `0x80429C40` must be non-null, then
/// `0x8001C240` is asked whether its leader/layout is loaded.
///
/// # Original
/// - Ghidra: `FUN_8015bf10` at `0x8015BF10` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bf10_FUN_8015bf10.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the slot is non-null and the scene-setup call returns non-zero,
/// `0` otherwise.
///
/// # Side effects
/// The callee may create the `wipeKuppa` layout resources on its first success.
///
/// # Safety
/// `0x8001C240` must be callable; the export drops its receiver operand, so the
/// call is emitted with no argument (`// unknown:`).
///
/// # Confidence
/// medium - the guard, the call and the comparison are literal, but the
/// callee's receiver is not printed.
pub unsafe fn fn_8015bf10() -> u32 {
    if read_global_word(W78_G_TRANSITION_SLOT_0) == 0 {
        return 0;
    }
    if call0_r(W78_FN_8001C240) == 0 {
        0
    } else {
        1
    }
}

/// Reports whether the `wipeDoor` transition is ready.
///
/// In plain English: the twin of [`fn_8015bf10`] for the slot at `0x80429C38`
/// and the `wipeDoor` layout call `0x8001BC60`.
///
/// # Original
/// - Ghidra: `FUN_8015bf50` at `0x8015BF50` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bf50_FUN_8015bf50.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the slot is non-null and `0x8001BC60` returns non-zero, `0`
/// otherwise.
///
/// # Side effects
/// The callee may create the `wipeDoor` layout resources.
///
/// # Safety
/// `0x8001BC60` must be callable; its receiver operand is dropped by the export.
///
/// # Confidence
/// medium - literal guard/call/compare, inferred callee receiver.
pub unsafe fn fn_8015bf50() -> u32 {
    if read_global_word(W78_G_TRANSITION_SLOT_1) == 0 {
        return 0;
    }
    if call0_r(W78_FN_8001BC60) == 0 {
        0
    } else {
        1
    }
}

/// Reports whether the `wipeFader` transition is ready.
///
/// In plain English: the twin of [`fn_8015bf10`] for the slot at `0x80429C48`
/// and the `wipeFader` layout call `0x8001C760`.
///
/// # Original
/// - Ghidra: `FUN_8015bf90` at `0x8015BF90` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bf90_FUN_8015bf90.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the slot is non-null and `0x8001C760` returns non-zero, `0`
/// otherwise.
///
/// # Side effects
/// The callee may create the `wipeFader` layout resources.
///
/// # Safety
/// `0x8001C760` must be callable; its receiver operand is dropped by the export.
///
/// # Confidence
/// medium - literal guard/call/compare, inferred callee receiver.
pub unsafe fn fn_8015bf90() -> u32 {
    if read_global_word(W78_G_TRANSITION_SLOT_2) == 0 {
        return 0;
    }
    if call0_r(W78_FN_8001C760) == 0 {
        0
    } else {
        1
    }
}

/// Reports whether the `wipeCircle` transition is ready.
///
/// In plain English: the twin of [`fn_8015bf10`] for the slot at `0x80429C30`
/// and the `wipeCircle` layout call `0x8001B470`.
///
/// # Original
/// - Ghidra: `FUN_8015bfd0` at `0x8015BFD0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015bfd0_FUN_8015bfd0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when the slot is non-null and `0x8001B470` returns non-zero, `0`
/// otherwise.
///
/// # Side effects
/// The callee may create the `wipeCircle` layout resources.
///
/// # Safety
/// `0x8001B470` must be callable; its receiver operand is dropped by the export.
///
/// # Confidence
/// medium - literal guard/call/compare, inferred callee receiver.
pub unsafe fn fn_8015bfd0() -> u32 {
    if read_global_word(W78_G_TRANSITION_SLOT_3) == 0 {
        return 0;
    }
    if call0_r(W78_FN_8001B470) == 0 {
        0
    } else {
        1
    }
}

/// Destroys a small state-owning object.
///
/// In plain English: the plain destructor shape of this family - non-null and a
/// positive free flag trigger one release call, then the pointer is returned
/// unchanged.
///
/// # Original
/// - Ghidra: `FUN_8015c260` at `0x8015C260` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c260_FUN_8015c260.c`
///
/// # Arguments
/// * `object` - object to destroy.
/// * `free` - free flag; only strictly positive values release the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May release `object` through `0x802B9280`.
///
/// # Safety
/// `object` must be null or a live allocation.
///
/// # Confidence
/// medium - the guard, the flag test and the call are literal, but the export
/// prints `0x802B9280()` without its receiver operand; the object is inferred to
/// be that receiver because it is the only live value.
pub unsafe fn fn_8015c260(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        platform::call1(W78_FN_802B9280, object as u32);
    }
    object
}

/// Destroys a state-owning object with an embedded state object at `+0x18`.
///
/// In plain English: release the state object at `object + 0x18` with a zero
/// free flag, then release the storage when the caller asked for it.
///
/// # Original
/// - Ghidra: `FUN_8015c2a0` at `0x8015C2A0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c2a0_FUN_8015c2a0.c`
///
/// # Arguments
/// * `object` - object to destroy.
/// * `free` - free flag; only strictly positive values release the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Runs the state-object release at `object + 0x18` and may release the storage.
///
/// # Safety
/// `object` must be null or a live allocation whose `+0x18` state object is
/// already initialised or zeroed.
///
/// # Confidence
/// high - both guards, the `+0x18` operand and the free call are literal.
pub unsafe fn fn_8015c2a0(object: usize, free: i32) -> usize {
    if object != 0 {
        platform::call2(W78_FN_8015F9F0, (object + 0x18) as u32, 0);
        if free > 0 {
            platform::call1(W78_FN_802B9280, object as u32);
        }
    }
    object
}

/// Destroys a second state-owning object with a `+0x18` state object.
///
/// In plain English: byte-for-byte the behaviour of [`fn_8015c2a0`]; only the
/// compiled size differs (`100` vs `96` bytes).
///
/// # Original
/// - Ghidra: `FUN_8015c300` at `0x8015C300` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c300_FUN_8015c300.c`
///
/// # Arguments
/// * `object` - object to destroy.
/// * `free` - free flag; only strictly positive values release the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Runs the state-object release at `object + 0x18` and may release the storage.
///
/// # Safety
/// Same as [`fn_8015c2a0`].
///
/// # Confidence
/// high - the buffered release and the free call are literal.
pub unsafe fn fn_8015c300(object: usize, free: i32) -> usize {
    if object != 0 {
        platform::call2(W78_FN_8015F9F0, (object + 0x18) as u32, 0);
        if free > 0 {
            platform::call1(W78_FN_802B9280, object as u32);
        }
    }
    object
}

/// Starts the strap-key wait branch by feeding the save-object scale.
///
/// In plain English: sets the layout's `+0x3D` byte to `1`, tests resource
/// readiness, starts the 30-frame transition, then copies the SDA2 float into
/// the object reached through `+0xF0 -> +0x1F4` and forwards its first word to
/// `0x801644D0`.
///
/// # Original
/// - Ghidra: `FUN_8015ca50` at `0x8015CA50` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ca50_FUN_8015ca50.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0xF0` holds the resource handle.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One layout byte store, one resource-readiness call, one timer start, two
/// float stores through `0x801637D0` and the forwarding call `0x801644D0`.
///
/// # Safety
/// `boot + 0xF0` must point at a resource whose `+0x1F4` word points at a
/// writable 16-byte object.
///
/// # Confidence
/// medium - every call and store is literal; the SDA2 float value is not
/// recovered and the descriptor's field roles are inferred from the call shape.
pub unsafe fn fn_8015ca50(boot: usize) {
    let layout = call0_r(W78_FN_80107F10);
    platform::call2(W78_FN_80108160, layout, 1);
    platform::call1(W78_FN_800B0DB0, 0);
    platform::call1(W78_FN_800B0A20, 0x1E);

    let resource = read_word(boot, 0xF0) as usize;
    let descriptor = read_word(resource, 0x1F4) as usize;
    call_f64_u32(
        W78_FN_801637D0,
        read_const_f32(W78_SDA2_5490) as f64,
        read_word(descriptor, 0),
    );
    platform::call1(W78_FN_801644D0, descriptor as u32);
}

/// Returns to the reset wait when no player is present any more.
///
/// In plain English: ask the player manager for its player count and, when the
/// `clz(count - 1) >> 5` test passes, change the scene state interface at
/// `+0xB0` to `ResetWait`.
///
/// # Original
/// - Ghidra: `FUN_8015cab0` at `0x8015CAB0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015cab0_FUN_8015cab0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May change the boot state through the interface at `boot + 0xB0`.
///
/// # Safety
/// `boot + 0xB0` must hold a live state interface and the player-manager global
/// `0x8042A440` must be live.
///
/// # Confidence
/// high - the virtual query, the `clz` test and the descriptor address are
/// literal.
pub unsafe fn fn_8015cab0(boot: usize) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);
    if positive_test(players) {
        change_state(boot + 0xB0, W78_D_BOOT_RESET_WAIT);
    }
}

/// Routes the strap-key state to the error fade or the controller-info branch.
///
/// In plain English: latch the scene-finished flag, hide the layout's `+0x3D`
/// flag, clear bit `0x40` of the display-manager flags, then query the error
/// windows; a hit moves the scene to `GoToErrorFade`. Otherwise the two
/// controller-info queries decide between the `0x4B0/0x3C` timers and zero.
/// Finally the `+0x101` and `+0x103` bytes are cleared.
///
/// # Original
/// - Ghidra: `FUN_8015cb20` at `0x8015CB20` (388 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015cb20_FUN_8015cb20.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two SDA byte stores, one layout byte store, one display-manager flag update,
/// up to three state changes through `boot + 0x74`, two timer stores and two
/// byte clears.
///
/// # Safety
/// `boot + 0x74` must hold a live state interface whose slot `+0x28` returns a
/// live descriptor object with slot `+0x14`.
///
/// # Confidence
/// high - every guard, store, virtual slot and descriptor address is literal;
/// only the meaning of the event ids is inferred from the state names.
pub unsafe fn fn_8015cb20(boot: usize) {
    write_global_byte(W78_G_SCENE_FINISHED, 1);

    let layout = call0_r(W78_FN_80107F10);
    platform::call2(W78_FN_80108160, layout, 0);

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);

    let iface = boot + 0x74;
    let manager = vcall0(iface, 0x28) as usize;
    if vcall1(manager, 0x14, W78_D_BOOT_ERROR_WINDOW_ON) != 0 {
        change_state(iface, W78_D_BOOT_GO_TO_ERROR_FADE);
    } else {
        let manager = vcall0(iface, 0x28) as usize;
        if vcall1(manager, 0x14, W78_D_BOOT_ERROR_INFINITY) != 0 {
            change_state(iface, W78_D_BOOT_GO_TO_ERROR_FADE);
        }
    }

    let manager = vcall0(iface, 0x28) as usize;
    let info_two = vcall1(manager, 0x14, W78_D_BOOT_CONTROLLER_INFO_2);
    let info_three = if info_two == 0 {
        let manager = vcall0(iface, 0x28) as usize;
        vcall1(manager, 0x14, W78_D_BOOT_CONTROLLER_INFO_3)
    } else {
        1
    };
    if info_two == 0 && info_three == 0 {
        write_word(boot, 0x108, 0x4B0);
        write_word(boot, 0x10C, 0x3C);
    } else {
        write_word(boot, 0x108, 0);
        write_word(boot, 0x10C, 0);
    }

    write_byte(boot, 0x101, 0);
    write_byte(boot, 0x103, 0);
}

/// Advances from the strap-key state when the player count drops.
///
/// In plain English: the same player-count test as [`fn_8015cab0`], but it
/// changes the interface at `boot + 0x74` to `WiiStrapKeyWa...`.
///
/// # Original
/// - Ghidra: `FUN_8015ccc0` at `0x8015CCC0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ccc0_FUN_8015ccc0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May change the boot state through the interface at `boot + 0x74`.
///
/// # Safety
/// `boot + 0x74` must hold a live state interface and the player-manager global
/// must be live.
///
/// # Confidence
/// high - the query, the `clz` test and the descriptor address are literal.
pub unsafe fn fn_8015ccc0(boot: usize) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);
    if positive_test(players) {
        change_state(boot + 0x74, W78_D_BOOT_WII_STRAP_KEY_WAIT);
    }
}

/// Starts the strap fade-out timers and feeds the save-object scale.
///
/// In plain English: store the `0x4B0/0x3C` timer pair and then perform the same
/// SDA2-float transfer as [`fn_8015ca50`] through `+0xF0 -> +0x1F4`.
///
/// # Original
/// - Ghidra: `FUN_8015cd40` at `0x8015CD40` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015cd40_FUN_8015cd40.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two timer stores, two float stores through `0x801637D0` and the forwarding
/// call `0x801644D0`.
///
/// # Safety
/// `boot + 0xF0` must point at a resource whose `+0x1F4` word points at a
/// writable 16-byte object.
///
/// # Confidence
/// medium - every store and call is literal; the SDA2 float value and the
/// descriptor's field roles are not recovered.
pub unsafe fn fn_8015cd40(boot: usize) {
    write_word(boot, 0x108, 0x4B0);
    write_word(boot, 0x10C, 0x3C);

    let resource = read_word(boot, 0xF0) as usize;
    let descriptor = read_word(resource, 0x1F4) as usize;
    call_f64_u32(
        W78_FN_801637D0,
        read_const_f32(W78_SDA2_5490) as f64,
        read_word(descriptor, 0),
    );
    platform::call1(W78_FN_801644D0, descriptor as u32);
}

/// Waits out the strap fade-out timer or a skip input.
///
/// In plain English: decrement the frame counter at `+0x108`; the state stays
/// only while the counter is still positive, the view event query `0x7F1F`
/// returns zero and bit `0x80000` is clear in the input flags. Any of those three
/// failing changes the state.
///
/// # Original
/// - Ghidra: `FUN_8015cdf0` at `0x8015CDF0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015cdf0_FUN_8015cdf0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x108` is the fade counter.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the decremented counter and changes the state through `boot + 0x74`.
///
/// # Safety
/// `boot + 0x74` must hold a live state interface and `0x8042A468` must hold the
/// live view object.
///
/// # Confidence
/// medium - the decrement, both guards and the transition are literal; the event
/// id and the role of the view slot `+0x20` are inferred from the call shape.
pub unsafe fn fn_8015cdf0(boot: usize) {
    let remaining = read_word(boot, 0x108).wrapping_sub(1) as i32;
    write_word(boot, 0x108, remaining as u32);

    if remaining > 0 {
        let view = read_global_word(W78_G_VIEW_OBJECT) as usize;
        let event_done = vcall1(view, 0x20, 0x7F1F) == 0;
        let skip = (read_global_word(G_DISPLAY_INPUT_FLAGS) & 0x80000) != 0;
        if event_done && !skip {
            return;
        }
    }

    change_state(boot + 0x74, W78_D_BOOT_WII_STRAP_FADE_OUT);
}

/// Enters the strap fade-out: clears the finish flag and raises the display bit.
///
/// In plain English: clear the scene-finished byte, start the alternate 30-frame
/// transition, set the layout's `+0x3D` byte to `1`, set bit `0x40` of the
/// display-manager flags and start the base transition too.
///
/// # Original
/// - Ghidra: `FUN_8015ce80` at `0x8015CE80` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ce80_FUN_8015ce80.c`
///
/// # Arguments
/// None; the object is reached through the globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One SDA byte store, two transition calls, one layout byte store and one
/// display-manager flag update.
///
/// # Safety
/// The layout and display-manager getters must return live objects.
///
/// # Confidence
/// high - every call, store and mask is literal.
pub unsafe fn fn_8015ce80() {
    write_global_byte(W78_G_SCENE_FINISHED, 0);
    platform::call1(W78_FN_800B0DB0, 0);
    platform::call1(W78_FN_800B0A90, 0x1E);

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) | 0x40);

    let layout = call0_r(W78_FN_80107F10);
    platform::call2(W78_FN_80108160, layout, 1);
}

/// Enters the first controller-info state once the save object reports ready.
///
/// In plain English: when the player-manager query returns zero, the `+0xFC`
/// save/controller object is live and its `+0x208` byte is set, clear `+0x209`
/// on the `+0xF0` object and move to `ControllerInf...`.
///
/// # Original
/// - Ghidra: `FUN_8015ced0` at `0x8015CED0` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ced0_FUN_8015ced0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One byte clear and one state change through `boot + 0x74`.
///
/// # Safety
/// `boot + 0xFC` and `boot + 0xF0` must point at live objects when their guards
/// pass.
///
/// # Confidence
/// medium - the three guards and both stores are literal; the meaning of the
/// `+0x208/+0x209` byte pair is inferred from the sibling state handlers.
pub unsafe fn fn_8015ced0(boot: usize) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);
    let save = read_word(boot, 0xFC) as usize;

    // observed: the export prints `countLeadingZeros(players) >> 5`, which is
    // true only when the query returned zero.
    if (players.leading_zeros() >> 5) != 0 && save != 0 && read_byte(save, 0x208) != 0 {
        let owner = read_word(boot, 0xF0) as usize;
        write_byte(owner, 0x209, 0);
        change_state(boot + 0x74, W78_D_BOOT_CONTROLLER_INFO_0);
    }
}

/// Starts the strap fade-out and marks the save object busy.
///
/// In plain English: run the readiness test, start the 30-frame transition and
/// set the `+0x209` byte of the `+0xFC` object.
///
/// # Original
/// - Ghidra: `FUN_8015cf70` at `0x8015CF70` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015cf70_FUN_8015cf70.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One readiness call, one timer start and one byte store.
///
/// # Safety
/// `boot + 0xFC` must point at a writable object.
///
/// # Confidence
/// high - the two calls, the `0x1E` timer value and the byte store are literal.
pub unsafe fn fn_8015cf70(boot: usize) {
    platform::call1(W78_FN_800B0DB0, 0);
    platform::call1(W78_FN_800B0A20, 0x1E);
    let save = read_word(boot, 0xFC) as usize;
    write_byte(save, 0x209, 1);
}

/// Enters the second controller-info state while a player is present.
///
/// In plain English: the twin of [`fn_8015ccc0`] with the `ControllerInf...`
/// descriptor `0x80377608`.
///
/// # Original
/// - Ghidra: `FUN_8015cfc0` at `0x8015CFC0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015cfc0_FUN_8015cfc0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May change the boot state through the interface at `boot + 0x74`.
///
/// # Safety
/// `boot + 0x74` must hold a live state interface and the player-manager global
/// must be live.
///
/// # Confidence
/// high - the query, the `clz` test and the descriptor address are literal.
pub unsafe fn fn_8015cfc0(boot: usize) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);
    if positive_test(players) {
        change_state(boot + 0x74, W78_D_BOOT_CONTROLLER_INFO_1);
    }
}

/// Leaves the strap sequence: sets the finish flag and releases the scene count.
///
/// In plain English: latch the scene-finished byte, clear bit `0x40` of the
/// display-manager flags, hide the layout's `+0x3D` flag and drop one scene
/// reference.
///
/// # Original
/// - Ghidra: `FUN_8015d030` at `0x8015D030` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d030_FUN_8015d030.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One SDA byte store, two object updates and the reference-count decrement of
/// [`fn_8015d080`].
///
/// # Safety
/// The layout and display-manager getters must return live objects.
///
/// # Confidence
/// high - every store, mask and call is literal.
pub unsafe fn fn_8015d030() {
    write_global_byte(W78_G_SCENE_FINISHED, 1);

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);

    let layout = call0_r(W78_FN_80107F10);
    platform::call2(W78_FN_80108160, layout, 0);
    fn_8015d080();
}

/// Decrements the scene reference count when it is positive.
///
/// In plain English: the counterpart of [`fn_8015bce0`]; a non-positive count is
/// left alone.
///
/// # Original
/// - Ghidra: `FUN_8015d080` at `0x8015D080` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d080_FUN_8015d080.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May write `count - 1` over the scene reference count.
///
/// # Safety
/// The SDA word must be readable and writable.
///
/// # Confidence
/// high - the `< 1` guard, the `-1` and the store are literal.
pub unsafe fn fn_8015d080() {
    let count = platform::read_i32(W78_G_SCENE_REFCOUNT);
    if count < 1 {
        return;
    }
    platform::write_i32(W78_G_SCENE_REFCOUNT, count - 1);
}

/// Starts the controller-info timers and clears the display's bit `0x08`.
///
/// In plain English: clear bit 3 of the display-manager flags, then store the
/// `300`/`30` frame pair into the boot object.
///
/// # Original
/// - Ghidra: `FUN_8015d0a0` at `0x8015D0A0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d0a0_FUN_8015d0a0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x108/+0x10C` are the timer pair.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One display-manager flag update and two timer stores.
///
/// # Safety
/// The display-manager getter must return a live object and `boot` must be
/// writable at `+0x108/+0x10C`.
///
/// # Confidence
/// high - the mask, the two constants and both stores are literal.
pub unsafe fn fn_8015d0a0(boot: usize) {
    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x08);

    write_word(boot, 0x108, 300);
    write_word(boot, 0x10C, 0x1E);
}

/// Hides the layout's `+0x3D` flag.
///
/// In plain English: read the layout object and store `0` in its `+0x3D` byte.
///
/// # Original
/// - Ghidra: `FUN_8015d780` at `0x8015D780` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d780_FUN_8015d780.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One layout byte store.
///
/// # Safety
/// The layout getter must return a writable object.
///
/// # Confidence
/// high - the getter, the operand `0` and the store are literal.
pub unsafe fn fn_8015d780() {
    let layout = call0_r(W78_FN_80107F10);
    platform::call2(W78_FN_80108160, layout, 0);
}

/// Runs the readiness test and starts the 30-frame transition.
///
/// In plain English: `0x800B0DB0(0)` followed by `0x800B0A20(0x1E)`.
///
/// # Original
/// - Ghidra: `FUN_8015d9c0` at `0x8015D9C0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d9c0_FUN_8015d9c0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The two calls' observable effects: a resource-readiness test and a timer
/// start.
///
/// # Safety
/// Both callees must be callable.
///
/// # Confidence
/// high - the two calls and the constant operand are literal.
pub unsafe fn fn_8015d9c0() {
    platform::call1(W78_FN_800B0DB0, 0);
    platform::call1(W78_FN_800B0A20, 0x1E);
}

/// Clears bit `0x40` of the display-manager flags.
///
/// In plain English: read the display manager and clear the bit that
/// [`fn_8015ce80`] sets.
///
/// # Original
/// - Ghidra: `FUN_8015daa0` at `0x8015DAA0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015daa0_FUN_8015daa0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One read-modify-write of the display manager's `+0x1D8` word.
///
/// # Safety
/// The display-manager getter must return a writable object.
///
/// # Confidence
/// high - the getter, the mask and the store are literal.
pub unsafe fn fn_8015daa0() {
    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);
}

// ---------------------------------------------------------------------------
// Wave 89, cluster A: the actor motion controller, the course cursor and the
// embedded-interface shims of `0x800D3A30..0x800DCF00`.
//
// The band is the same `0x800Dxxxx` actor code that waves 63 and 76 cover: the
// objects here all start with a vtable word at `+0x00` and keep their animation
// state in the words around `+0x10..+0x70`. The bodies below are reached from
// `0x800D5330`'s constructor family and from the `Wm_mr_spinjump` motion
// controller, so the field names are kept neutral (`state`, `object`, `slot`)
// where the export does not prove a role.
// ---------------------------------------------------------------------------

/// `0x800D4AD0(a, object, mode)`: seeds the actor's animation/rotation state.
const W89_FN_800D4AD0: usize = 0x800D_4AD0;

/// `0x800D4CD0(object)`: refreshes the interpolated `+0x1E2..+0x1E4` triple.
const W89_FN_800D4CD0: usize = 0x800D_4CD0;

/// `0x800D3AD0()`: the reset path taken when `+0x170` is still zero.
const W89_FN_800D3AD0: usize = 0x800D_3AD0;

/// `0x800D5B90(object, record)`: registers one animation record.
const W89_FN_800D5B90: usize = 0x800D_5B90;

/// `0x800D5580(object)`: releases the actor owned by a wrapper's `+0x04`.
const W89_FN_800D5580: usize = 0x800D_5580;

/// `0x800D5BE0(object)`: virtual-dispatch readiness query (`0` = ready).
const W89_FN_800D5BE0: usize = 0x800D_5BE0;

/// `0x80168FE0(node)`: releases the animation node at `node`.
const W89_FN_80168FE0: usize = 0x8016_8FE0;

/// `0x80168520(object, mode)`: switches the motion table of `object`.
const W89_FN_80168520: usize = 0x8016_8520;

/// `0x8000D260(object, index, value)`: writes one motion slot.
const W89_FN_8000D260: usize = 0x8000_D260;

/// `0x8000D3B0(object, index, value)`: writes a second motion slot.
const W89_FN_8000D3B0: usize = 0x8000_D3B0;

/// `0x80168180(node, second, first)`: starts one animation on `node`.
const W89_FN_80168180: usize = 0x8016_8180;

/// `0x802F1330`: per-variant word table indexed by the actor's variant byte.
const W89_TABLE_802F1330: usize = 0x802F_1330;

/// `0x80371720`: state descriptor used by the reset transition.
const W89_DESC_80371720: u32 = 0x8037_1720;

/// `0x803718B0`: descriptor handed to the embedded interface's slot `+0x18`.
const W89_DESC_803718B0: u32 = 0x8037_18B0;

/// `r13 - 0x5480`: shared engine tick counter read by the motion helpers.
///
/// `// unknown:` the counter's writer is outside this band; both users only
/// compare its value with a cached copy.
const W89_G_TICK_COUNTER: usize = SDA_BASE - 0x5480;

/// Calls `func(f64, f64) -> u32`.
///
/// Used by [`fn_800d9370`] for `0x801611C0`, whose two operands the export
/// prints as `(double)` casts of the delta pair.
#[inline]
unsafe fn call_f64_f64_u32(addr: usize, a: f64, b: f64) -> u32 {
    let f: extern "C" fn(f64, f64) -> u32 = core::mem::transmute(addr);
    f(a, b)
}

/// Unlinks both animation records of the actor's current variant.
///
/// In plain English: ask the actor for its current record object, release that
/// record through its own slot `+0x14`, release the variant record selected by
/// the index at `+0x770`, then register both records again through
/// `0x800D5B90` (the stale one first, the current one second).
///
/// # Original
/// - Ghidra: `FUN_800d3a30` at `0x800D3A30` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3a30_FUN_800d3a30.c`
///
/// # Arguments
/// * `object` - actor whose vtable slot `+0x28` returns the current record and
///   whose `+0x770` word selects one of the `0x20`-word variant records that
///   start at `+0x4A8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual releases and two registration calls; both records' vtable slots
/// may have their own effects.
///
/// # Safety
/// `object` must be live, slot `+0x28` must return a record with a live
/// vtable, and the variant index must address a readable record word.
///
/// # Confidence
/// medium - every call, index and store is literal; the record layout
/// (`+0x4A8`, stride `0x20` words) and both field roles are `// inferred:` from
/// the sibling bodies `0x800D4670` and `0x800D4C10`, and both virtual calls
/// print without their receiver.
pub unsafe fn fn_800d3a30(object: usize) {
    let current = vcall0(object, 0x28) as usize;
    let _ = vcall0(current, 0x14);

    // observed: param_1[param_1[0x1dc] * 0x20 + 0x12a] - a record word selected
    // by the signed index at +0x770.
    let index = read_word(object, 0x770) as i32 as isize;
    let record_offset = (index * 0x20 + 0x12A) * 4;
    let record = (object as isize + record_offset) as usize;
    let variant = read_word(record, 0) as usize;
    let _ = vcall0(variant, 0x14);

    let refreshed = vcall0(object, 0x28);
    let _ = platform::call2(W89_FN_800D5B90, object as u32, refreshed);
    let _ = platform::call2(W89_FN_800D5B90, object as u32, record as u32);
}

/// Clears the actor's cached record pointer and runs the matching reset.
///
/// In plain English: when `+0x170` is already zero, fall into the reset body at
/// `0x800D3AD0`; otherwise clear the pointer and let vtable slot `+0x80` undo
/// whatever it installed.
///
/// # Original
/// - Ghidra: `FUN_800d3c80` at `0x800D3C80` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3c80_FUN_800d3c80.c`
///
/// # Arguments
/// * `object` - actor owning the record pointer at `+0x170`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call `0x800D3AD0`; otherwise writes `+0x170` and runs slot `+0x80`.
///
/// # Safety
/// `object` must be writable at `+0x170` and its vtable must be live when the
/// pointer was non-zero.
///
/// # Confidence
/// high - the one guard, the zero store and both calls are literal.
pub unsafe fn fn_800d3c80(object: usize) {
    if read_word(object, 0x170) == 0 {
        platform::call0(W89_FN_800D3AD0);
        return;
    }
    write_word(object, 0x170, 0);
    let _ = vcall0(object, 0x80);
}

/// Switches the actor's variant and resets its animation when it changed.
///
/// In plain English: store the variant byte at `+0x152`, hand the matching
/// `0x802F1330` word to vtable slot `+0xB0`, and when the pending variant at
/// `+0x774` differs from the active one at `+0x770` rebuild the interpolation,
/// drop the cached record, mark `+0x153` as `0xFF` and pick the variant's
/// standing animation (`1` for variant `2`, `2` for `6`, otherwise `0`); finally
/// run slot `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_800d4670` at `0x800D4670` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4670_FUN_800d4670.c`
///
/// # Arguments
/// * `object` - actor; `+0x152` is the variant byte, `+0x153` the previous
///   variant, `+0x770`/`+0x774` the active/pending variant words.
/// * `variant` - new variant index, stored as a byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two word/slot updates plus up to four virtual calls and the reset body of
/// [`fn_800d3c80`].
///
/// # Safety
/// `object` and its vtable must be live; `variant` must index the `0x802F1330`
/// table.
///
/// # Confidence
/// medium - every store, branch and call is literal; the table's entries and
/// the meaning of the `0x770/0x774` pair are `// inferred:` from the sibling
/// `0x800D4D80`/`0x800D5330` bodies.
pub unsafe fn fn_800d4670(object: usize, variant: i32) {
    write_byte(object, 0x152, variant as u8);
    let entry = read_global_word((W89_TABLE_802F1330 as isize + variant as isize * 4) as usize);
    let _ = vcall1(object, 0xB0, entry);

    if read_word(object, 0x774) != read_word(object, 0x770) {
        let _ = vcall0(object, 0x6C);
        let _ = platform::call1(W89_FN_800D4CD0, object as u32);
        fn_800d3c80(object);
        write_byte(object, 0x153, 0xFF);
    }

    match read_byte(object, 0x152) {
        2 => {
            let _ = vcall1(object, 0x34, 1);
        }
        6 => {
            let _ = vcall1(object, 0x34, 2);
        }
        _ => {
            let _ = vcall1(object, 0x34, 0);
        }
    }
    let _ = vcall0(object, 0x1C);
}

/// Seeds a mode-3 animation and forwards one value to slot `+0x18`.
///
/// In plain English: run `0x800D4AD0(first, object, 3)` and then call the
/// object's slot `+0x18` with `value`.
///
/// # Original
/// - Ghidra: `FUN_800d4c10` at `0x800D4C10` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4c10_FUN_800d4c10.c`
///
/// # Arguments
/// * `first` - opaque first operand of `0x800D4AD0`.
/// * `object` - actor passed as the second operand and as the slot receiver.
/// * `value` - argument of slot `+0x18`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Delegates to `0x800D4AD0` and to one virtual call.
///
/// # Safety
/// `object` and its vtable must be live.
///
/// # Confidence
/// medium - both calls and the mode constant are literal; the receiver of slot
/// `+0x18` is dropped by the export and `first`'s role is `// unknown:`.
pub unsafe fn fn_800d4c10(first: u32, object: usize, value: u32) {
    let _ = platform::call3(W89_FN_800D4AD0, first, object as u32, 3);
    let _ = vcall1(object, 0x18, value);
}

/// Seeds a mode-2 animation and resets three motion slots.
///
/// In plain English: run `0x800D4AD0(first, object, 2)`, switch the motion
/// table through `0x80168520`, then clear motion slots `1` and `3` with the two
/// `0x8000D2xx`/`0x8000D3xx` writers.
///
/// # Original
/// - Ghidra: `FUN_800d4c70` at `0x800D4C70` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4c70_FUN_800d4c70.c`
///
/// # Arguments
/// * `first` - opaque first operand of `0x800D4AD0`.
/// * `object` - actor passed to all four callees.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The four callees' writes: one mode seed, one table switch and two zeroed
/// slots.
///
/// # Safety
/// `object` must be a live actor and each callee must accept it.
///
/// # Confidence
/// high - all four calls, the mode and both slot indices are literal.
pub unsafe fn fn_800d4c70(first: u32, object: usize) {
    let _ = platform::call3(W89_FN_800D4AD0, first, object as u32, 2);
    let _ = platform::call2(W89_FN_80168520, object as u32, 2);
    let _ = platform::call3(W89_FN_8000D260, object as u32, 1, 0);
    let _ = platform::call3(W89_FN_8000D3B0, object as u32, 3, 0);
}

/// Starts one animation on the object returned by slot `+0x28`.
///
/// In plain English: take the object's current animation node and call
/// `0x80168180(node, second, first)`.
///
/// # Original
/// - Ghidra: `FUN_800d5730` at `0x800D5730` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5730_FUN_800d5730.c`
///
/// # Arguments
/// * `object` - receiver of slot `+0x28`.
/// * `first` - second operand of `0x80168180` (the export prints it last).
/// * `second` - first operand of `0x80168180`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual read and one animation start.
///
/// # Safety
/// `object` must be live and slot `+0x28` must return a valid node.
///
/// # Confidence
/// medium - the two calls and the operand order are literal; the receiver of
/// slot `+0x28` is dropped by the export and both arguments' roles are
/// `// unknown:`.
pub unsafe fn fn_800d5730(object: usize, first: u32, second: u32) {
    let node = vcall0(object, 0x28);
    let _ = platform::call3(W89_FN_80168180, node, second, first);
}

/// Releases the animation node at `+0x28` and dispatches slot `+0x88`.
///
/// In plain English: release the node embedded at `object + 0x28` through
/// `0x80168FE0`, and when the readiness query `0x800D5BE0` says the object is
/// idle, call slot `+0x88` with the caller's 64-bit operand.
///
/// # Original
/// - Ghidra: `FUN_800d5c10` at `0x800D5C10` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5c10_FUN_800d5c10.c`
///
/// # Arguments
/// * `first` - 64-bit operand printed before the object; the export types it
///   `undefined8`, so it is carried as two words.
/// * `object` - owner of the node and receiver of slot `+0x88`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One node release and, when ready, one virtual call.
///
/// # Safety
/// `object + 0x28` must be a live node, and the vtable must be readable when
/// the readiness query returns zero.
///
/// # Confidence
/// medium - the release, the readiness test and the slot number are literal;
/// the export prints the call as `(param_1, param_2)` with the receiver
/// implicit, so the receiver is restored as `object` and `first` is split into
/// its two words.
pub unsafe fn fn_800d5c10(first: u64, object: usize) {
    let _ = platform::call1(W89_FN_80168FE0, (object + 0x28) as u32);
    if platform::call1(W89_FN_800D5BE0, object as u32) == 0 {
        let vtable = read_word(object, 0) as usize;
        let entry = read_word(vtable, 0x88) as usize;
        let _ = platform::call3(entry, object as u32, first as u32, (first >> 32) as u32);
    }
}

/// Releases the animation node at `+0x60`.
///
/// In plain English: a one-call shim over `0x80168FE0` for the node embedded at
/// `object + 0x60`.
///
/// # Original
/// - Ghidra: `FUN_800d5c80` at `0x800D5C80` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5c80_FUN_800d5c80.c`
///
/// # Arguments
/// * `object` - owner of the node at `+0x60`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The release performed by `0x80168FE0`.
///
/// # Safety
/// `object + 0x60` must be a live node.
///
/// # Confidence
/// high - the single call and its `+0x60` operand are literal.
pub unsafe fn fn_800d5c80(object: usize) {
    let _ = platform::call1(W89_FN_80168FE0, (object + 0x60) as u32);
}

/// Releases the node at `+0x28` and dispatches slot `+0x90`.
///
/// In plain English: the sibling of [`fn_800d5c10`] with the `0x801680E0`
/// release and vtable slot `+0x90`.
///
/// # Original
/// - Ghidra: `FUN_800d5c90` at `0x800D5C90` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5c90_FUN_800d5c90.c`
///
/// # Arguments
/// * `first` - 64-bit operand forwarded to slot `+0x90`.
/// * `object` - owner of the node and receiver of slot `+0x90`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One node release and, when ready, one virtual call.
///
/// # Safety
/// `object + 0x28` must be a live node and the vtable readable.
///
/// # Confidence
/// medium - every operand is literal; the export drops the receiver of the
/// final call, so it is restored as `object` and `first` is split into two
/// words.
pub unsafe fn fn_800d5c90(first: u64, object: usize) {
    let _ = platform::call1(W76_FN_801680E0, (object + 0x28) as u32);
    if platform::call1(W89_FN_800D5BE0, object as u32) == 0 {
        let vtable = read_word(object, 0) as usize;
        let entry = read_word(vtable, 0x90) as usize;
        let _ = platform::call3(entry, object as u32, first as u32, (first >> 32) as u32);
    }
}

/// Releases the actor owned by a wrapper.
///
/// In plain English: pass the wrapper's `+0x04` actor pointer to
/// `0x800D5580`.
///
/// # Original
/// - Ghidra: `FUN_800d6df0` at `0x800D6DF0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6df0_FUN_800d6df0.c`
///
/// # Arguments
/// * `wrapper` - object whose `+0x04` word is the actor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The release performed by `0x800D5580`.
///
/// # Safety
/// `wrapper + 4` must hold a live actor.
///
/// # Confidence
/// high - the load, the call and the offset are literal.
pub unsafe fn fn_800d6df0(wrapper: usize) {
    let _ = platform::call1(W89_FN_800D5580, read_word(wrapper, 4));
}

/// Runs the owned actor's vtable slot `+0x18`.
///
/// In plain English: read the actor at `+0x04` and call its third virtual slot.
///
/// # Original
/// - Ghidra: `FUN_800d6e90` at `0x800D6E90` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6e90_FUN_800d6e90.c`
///
/// # Arguments
/// * `wrapper` - object whose `+0x04` word is the actor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever slot `+0x18` does.
///
/// # Safety
/// The actor must be live and its vtable readable.
///
/// # Confidence
/// medium - the load and slot number are literal; the export drops every
/// operand of the call, so the receiver is restored from the double
/// indirection.
pub unsafe fn fn_800d6e90(wrapper: usize) {
    let actor = read_word(wrapper, 4) as usize;
    let _ = vcall0(actor, 0x18);
}

/// Runs the owned actor's vtable slot `+0x20`.
///
/// In plain English: the sibling of [`fn_800d6e90`] with slot `+0x20`.
///
/// # Original
/// - Ghidra: `FUN_800d7000` at `0x800D7000` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7000_FUN_800d7000.c`
///
/// # Arguments
/// * `wrapper` - object whose `+0x04` word is the actor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever slot `+0x20` does.
///
/// # Safety
/// The actor must be live and its vtable readable.
///
/// # Confidence
/// medium - the load and slot number are literal; the export prints the call
/// without operands, so the receiver is restored from the double indirection.
pub unsafe fn fn_800d7000(wrapper: usize) {
    let actor = read_word(wrapper, 4) as usize;
    let _ = vcall0(actor, 0x20);
}

/// Runs the owned actor's vtable slot `+0x24` when a flag is set.
///
/// In plain English: return when `+0x08` is zero, otherwise call the actor's
/// slot `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_800d7020` at `0x800D7020` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7020_FUN_800d7020.c`
///
/// # Arguments
/// * `wrapper` - object with a flag at `+0x08` and the actor at `+0x04`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run one virtual slot.
///
/// # Safety
/// The actor must be live when `+0x08` is non-zero.
///
/// # Confidence
/// medium - the guard, the store and the slot number are literal; the call's
/// operands are dropped by the export.
pub unsafe fn fn_800d7020(wrapper: usize) {
    if read_word(wrapper, 8) == 0 {
        return;
    }
    let actor = read_word(wrapper, 4) as usize;
    let _ = vcall0(actor, 0x24);
}

/// Reduces the shared engine tick into the motion's step range.
///
/// In plain English: sum the four motion lengths (`+0x68` twice plus `+0x6C`
/// four times plus the two boundary offsets `+0x48` and `+0x58`) and return the
/// tick counter modulo that total; a zero total returns zero without dividing.
///
/// # Original
/// - Ghidra: `FUN_800d7780` at `0x800D7780` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7780_FUN_800d7780.c`
///
/// # Arguments
/// * `state` - motion state laid out by `0x800D7350`.
///
/// # Returns
/// The tick counter reduced into `0 .. total - 1`, or `0` when `total` is zero.
///
/// # Side effects
/// None; only reads.
///
/// # Safety
/// The four words at `+0x48`, `+0x58`, `+0x68` and `+0x6C` and the SDA counter
/// must be readable.
///
/// # Confidence
/// medium - the four terms, the zero guard and the modulo are literal; the
/// counter's owner is `// unknown:`, so it is named after its SDA slot.
pub unsafe fn fn_800d7780(state: usize) -> u32 {
    let total = read_word(state, 0x68)
        .wrapping_mul(2)
        .wrapping_add(read_word(state, 0x6C).wrapping_mul(4))
        .wrapping_add(read_word(state, 0x48))
        .wrapping_add(read_word(state, 0x58));
    if total == 0 {
        return 0;
    }
    platform::read_u32(W89_G_TICK_COUNTER) % total
}

/// Re-seeds the motion when the shared tick changed, then clamps it.
///
/// In plain English: if the tick counter differs from the copy at `+0x70`, run
/// slot `+0x10` and store the new tick, then clamp `+0x5C` with
/// [`fn_800d7480`].
///
/// # Original
/// - Ghidra: `FUN_800d7820` at `0x800D7820` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7820_FUN_800d7820.c`
///
/// # Arguments
/// * `state` - motion state whose `+0x1C` word caches the last tick and whose
///   `+0x3C/+0x4C/+0x5C` floats are the clamp inputs.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May run slot `+0x10`, writes `+0x70`, and may write one clamped float.
///
/// # Safety
/// The state and its vtable must be live.
///
/// # Confidence
/// medium - the compare, cache store, virtual call and clamp are literal; the
/// SDA counter's owner is `// unknown:`.
pub unsafe fn fn_800d7820(state: usize) {
    let tick = platform::read_u32(W89_G_TICK_COUNTER);
    if tick != read_word(state, 0x70) {
        let _ = vcall0(state, 0x10);
        write_word(state, 0x70, tick);
    }
    fn_800d7480(state);
}

/// Rewinds the motion to its first step and jumps to the waiting state.
///
/// In plain English: clear the playback flag at `+0x74`, copy the step limit at
/// `+0x6C` into the current step at `+0x64`, and enter the `0x80371720` state
/// through slot `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_800d79b0` at `0x800D79B0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d79b0_FUN_800d79b0.c`
///
/// # Arguments
/// * `object` - motion object; `+0x64` is the current step, `+0x6C` the limit
///   and `+0x74` the playback byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two stores and one state change.
///
/// # Safety
/// `object` must be writable at those offsets and its slot `+0x18` callable.
///
/// # Confidence
/// medium - the byte clear, the word copy and the descriptor address are
/// literal; the descriptor's name is not exported, so it is cited by address.
pub unsafe fn fn_800d79b0(object: usize) {
    write_byte(object, 0x74, 0);
    write_word(object, 0x64, read_word(object, 0x6C));
    let _ = vcall1(object, 0x18, W89_DESC_80371720);
}

/// Dispatches the interface embedded at `first + 0x18`.
///
/// In plain English: `0x802DCBFC` reads the three-word descriptor at
/// `first + 0x18`, adjusts `second` by its first word and calls the selected
/// member. The Ghidra export prints `0x802DCBFC(second, second, first)` because
/// it drops the `r12` operand; `interface_call` models it.
///
/// # Original
/// - Ghidra: `FUN_800d7c20` at `0x800D7C20` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7c20_FUN_800d7c20.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - receiver forwarded to the dispatched method.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever `0x802DCBFC` does.
///
/// # Safety
/// `first`'s embedded descriptor and `second` must be live.
///
/// # Confidence
/// high - `mr r5,r3` / `mr r3,r4` / `addi r12,r5,K` / `bl 0x802DCBFC`;
/// `interface_call` models the descriptor offset.
pub unsafe fn fn_800d7c20(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Twin of [`fn_800d7c20`] with the interface at `first + 0x0C`.
///
/// In plain English: the exact twin of [`fn_800d7c20`], a second thunk in the
/// same dispatch table.
///
/// # Original
/// - Ghidra: `FUN_800d7c50` at `0x800D7C50` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7c50_FUN_800d7c50.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - receiver forwarded to the dispatched method.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever `0x802DCBFC` does.
///
/// # Safety
/// `first`'s embedded descriptor and `second` must be live.
///
/// # Confidence
/// high - identical thunk shape; `interface_call` models the `r12` descriptor.
pub unsafe fn fn_800d7c50(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Releases a motion object through its own vtable and optionally frees it.
///
/// In plain English: when the object is non-zero, run the release hook
/// `0x8015F800(object, 0)` and, if the caller's flag is positive, free the
/// storage through `0x802B9280`.
///
/// # Original
/// - Ghidra: `FUN_800d7fc0` at `0x800D7FC0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7fc0_FUN_800d7fc0.c`
///
/// # Arguments
/// * `object` - object to release; `0` is accepted.
/// * `free_flag` - when positive, the storage is freed.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// One release hook and possibly one free.
///
/// # Safety
/// `object` must be a live object when non-zero.
///
/// # Confidence
/// high - the guard, the release call, the `> 0` test and the free are
/// literal.
pub unsafe fn fn_800d7fc0(object: usize, free_flag: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free_flag > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Advances or rewinds the course-cursor frame counter by one.
///
/// In plain English: with the direction byte at `+0x2A` clear, increment the
/// frame at `+0x20`; when it reaches the frame count at `+0x1C`, either stop on
/// the last frame (returning `1`) or, when the wrap byte at `+0x30` is set,
/// flip direction and step back two frames. With the direction byte set the
/// counter decrements and the same three-way decision happens at `-1`. The
/// `+0x26` flag bit `2` makes either end wrap instead of stopping.
///
/// # Original
/// - Ghidra: `FUN_800d9290` at `0x800D9290` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9290_FUN_800d9290.c`
///
/// # Arguments
/// * `state` - cursor; `+0x1C` frame count, `+0x20` current frame, `+0x26`
///   flags, `+0x2A` direction, `+0x30` wrap-enable.
///
/// # Returns
/// `1` when the range was exhausted without wrapping, `0` otherwise.
///
/// # Side effects
/// Writes `+0x20` and possibly `+0x2A`.
///
/// # Safety
/// `state` must be readable/writable at the offsets above.
///
/// # Confidence
/// high - both directions, the four bounds tests and the two wrap flags are
/// literal; only the field names are `// inferred:` from the caller
/// [`fn_800d9560`].
pub unsafe fn fn_800d9290(state: usize) -> u32 {
    let mut result = 0;

    if read_byte(state, 0x2A) == 0 {
        let length = read_half(state, 0x1C);
        let counter = read_half(state, 0x20) as i16 as i32 + 1;
        write_half(state, 0x20, counter as u16);
        if length as i32 <= counter {
            if (read_half(state, 0x26) & 2) == 0 {
                if read_byte(state, 0x30) == 0 {
                    write_half(state, 0x20, length.wrapping_sub(1));
                    result = 1;
                } else {
                    write_byte(state, 0x2A, 1);
                    write_half(state, 0x20, length.wrapping_sub(2));
                }
            } else {
                write_half(state, 0x20, 0);
            }
        }
    } else {
        let counter = read_half(state, 0x20) as i16 as i32 - 1;
        write_half(state, 0x20, counter as u16);
        if counter < 0 {
            if (read_half(state, 0x26) & 2) == 0 {
                if read_byte(state, 0x30) == 0 {
                    write_half(state, 0x20, 0);
                    result = 1;
                } else {
                    write_byte(state, 0x2A, read_byte(state, 0x2A) ^ 1);
                    write_half(state, 0x20, 1);
                }
            } else {
                write_half(state, 0x20, read_half(state, 0x1C).wrapping_sub(1));
            }
        }
    }

    result
}

/// Converts the selected frame record into a position and heading.
///
/// In plain English: read the record selected by `+0x20` from the table at
/// `+0x04`, store its x and the negated y as floats at `+0x08/+0x0C`, then take
/// the heading from `0x801611C0(dy, dx)` relative to the target point at
/// `+0x10`, and mark the object busy with `8`.
///
/// # Original
/// - Ghidra: `FUN_800d9370` at `0x800D9370` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9370_FUN_800d9370.c`
///
/// # Arguments
/// * `state` - cursor state; `+0x04` record table, `+0x10` target position
///   pointer, `+0x20` selected record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes two floats at `+0x08/+0x0C`, the heading halfword at `+0x24` and the
/// busy word at `+0x2C`.
///
/// # Safety
/// The table and the target must point at readable memory.
///
/// # Confidence
/// medium - every load, store and the negation are literal; the
/// `CONCAT44(0x43300000, x ^ 0x80000000)` sequence is the compiler's
/// signed-halfword-to-float conversion, so it is reduced to `as i16 as f32`,
/// and `0x801611C0`'s identity as the heading helper is `// inferred:`.
pub unsafe fn fn_800d9370(state: usize) {
    let table = read_word(state, 4) as usize;
    if table == 0 {
        return;
    }

    let index = (read_half(state, 0x20) as i16 as i32) * 0x10;
    let base = (table as isize + index as isize) as usize;
    // observed: the y component is negated before the conversion.
    let x = read_half(base, 0) as i16 as f32;
    let y = -(read_half(base, 2) as i16) as f32;
    write_float(state, 8, x);
    write_float(state, 0x0C, y);

    let target = read_word(state, 0x10) as usize;
    let heading = call_f64_f64_u32(
        FN_801611C0,
        (y - read_float(target, 4)) as f64,
        (x - read_float(target, 0)) as f64,
    );
    write_half(state, 0x24, heading as u16);
    write_word(state, 0x2C, 8);
}

/// Steps the cursor along its frame table.
///
/// In plain English: clear the output halfword at `+0x28`, and when a table is
/// present and no pending frame is active, translate the position at `+0x10`
/// by the velocity at `+0x14`, test it against the target radius, and either
/// start the next frame (`0x800D9370` plus the new hold time) or set the
/// stop/wrap flags. A pending hold at `+0x22` just counts down.
///
/// # Original
/// - Ghidra: `FUN_800d9560` at `0x800D9560` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9560_FUN_800d9560.c`
///
/// # Arguments
/// * `state` - cursor state; `+0x04` frame table, `+0x08` row stride selector,
///   `+0x10/+0x14` position and velocity pointers, `+0x1E/+0x1C` previous and
///   current row, `+0x22` hold counter, `+0x26` flags, `+0x28` output word.
///
/// # Returns
/// `1` when a frame was started, `0` otherwise.
///
/// # Side effects
/// Moves the position vector, writes `+0x28/+0x26/+0x1E/+0x22`, may call
/// [`fn_800d9290`] and [`fn_800d9370`], and ends with slot `+0x0C`.
///
/// # Safety
/// The table, the two float pointers and the vtable must be live.
///
/// # Confidence
/// medium - every branch, store and call is literal; the meaning of the three
/// flag bits and of the row fields is `// inferred:` from
/// [`fn_800d9290`]/[`fn_800d9370`], and slot `+0x0C` prints without operands.
pub unsafe fn fn_800d9560(state: usize) -> u32 {
    write_half(state, 0x28, 0);

    let table = read_word(state, 4) as usize;
    if table == 0 {
        return 0;
    }
    if (read_half(state, 0x26) & 8) != 0 {
        return 0;
    }

    if read_half(state, 0x22) != 0 {
        write_half(state, 0x22, read_half(state, 0x22).wrapping_sub(1));
        return 0;
    }

    let position = read_word(state, 0x10) as usize;
    let velocity = read_word(state, 0x14) as usize;
    write_half(state, 0x26, read_half(state, 0x26) & 0xFFFB);
    for i in 0..3 {
        let moved = read_float(position, i * 4) + read_float(velocity, i * 4);
        write_float(position, i * 4, moved);
    }

    let mut result = 0;
    if fn_800d96d0(state) {
        let row = read_half(state, 0x20) as i16 as isize;
        let record = (table as isize + row * 0x10) as usize;
        let row_flags = read_half(record, 0x0E);
        write_half(state, 0x28, row_flags);

        if (row_flags & 1) == 0 {
            write_half(state, 0x1E, read_half(state, 0x20));
            if fn_800d9290(state) == 0 {
                fn_800d9370(state);
                let previous = read_half(state, 0x1E) as i16 as isize;
                let hold = read_half((table as isize + previous * 0x10) as usize, 0x0C);
                write_half(state, 0x22, hold);
                write_half(state, 0x26, read_half(state, 0x26) | 4);
            } else {
                write_half(state, 0x26, read_half(state, 0x26) | 0x0C);
            }
        } else {
            write_half(state, 0x26, read_half(state, 0x26) | 0x0C);
        }
        result = 1;
    }

    let _ = vcall0(state, 0x0C);
    result
}

/// Tests whether the cursor position is inside the target radius.
///
/// In plain English: squared distance between the position at `+0x08/+0x0C`
/// and the target at `+0x10` against the squared radius read through `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_800d96d0` at `0x800D96D0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d96d0_FUN_800d96d0.c`
///
/// # Arguments
/// * `state` - cursor; `+0x08/+0x0C` current x/y, `+0x10` target x/y pointer,
///   `+0x18` radius pointer.
///
/// # Returns
/// `true` when the cursor is strictly inside the radius.
///
/// # Side effects
/// None; only reads.
///
/// # Safety
/// All three pointers must be readable.
///
/// # Confidence
/// high - the two deltas, the squared sum, the squared radius and the strict
/// `<` are literal.
pub unsafe fn fn_800d96d0(state: usize) -> bool {
    let target = read_word(state, 0x10) as usize;
    let dx = read_float(state, 8) - read_float(target, 0);
    let dy = read_float(state, 0x0C) - read_float(target, 4);
    let radius = read_word(state, 0x18) as usize;
    let limit = read_float(radius, 0);
    dx * dx + dy * dy < limit * limit
}

/// Empty virtual hook.
///
/// In plain English: the export body is a bare `return;`, so this entry of the
/// object's vtable intentionally does nothing.
///
/// # Original
/// - Ghidra: `FUN_800dc800` at `0x800DC800` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc800_FUN_800dc800.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Safety
/// Callable with any register state.
///
/// # Confidence
/// high - the whole body is a single return instruction.
pub unsafe fn fn_800dc800() {}

/// Runs the embedded interface's vtable slot `+0x10`.
///
/// In plain English: read the interface pointer from `+0x5C` and call its
/// second slot with no arguments.
///
/// # Original
/// - Ghidra: `FUN_800dc970` at `0x800DC970` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dc970_FUN_800dc970.c`
///
/// # Arguments
/// * `object` - owner of the interface pointer at `+0x5C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever the slot does.
///
/// # Safety
/// `+0x5C` must hold a live interface and its vtable must be readable.
///
/// # Confidence
/// medium - the load and slot number are literal; the call prints no operands,
/// so the receiver is the interface pointer itself.
pub unsafe fn fn_800dc970(object: usize) {
    let interface = read_word(object, 0x5C) as usize;
    let _ = vcall0(interface, 0x10);
}

/// Empty virtual hook.
///
/// In plain English: the export body is a bare `return;`; the slot is a
/// placeholder in this object's vtable.
///
/// # Original
/// - Ghidra: `FUN_800dced0` at `0x800DCED0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dced0_FUN_800dced0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Safety
/// Callable with any register state.
///
/// # Confidence
/// high - the whole body is a single return instruction.
pub unsafe fn fn_800dced0() {}

/// Sends the `0x803718B0` event to the embedded interface.
///
/// In plain English: the interface lives at `object + 0x0C`; call its slot
/// `+0x18` with the descriptor `0x803718B0`.
///
/// # Original
/// - Ghidra: `FUN_800dcee0` at `0x800DCEE0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dcee0_FUN_800dcee0.c`
///
/// # Arguments
/// * `object` - owner of the embedded interface at `+0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call; the interface's own state may change.
///
/// # Safety
/// `object + 0x0C` must start with a readable vtable.
///
/// # Confidence
/// medium - the embedded receiver, the slot number and the descriptor are
/// literal; the event's name is not exported, so it is cited by address.
pub unsafe fn fn_800dcee0(object: usize) {
    let _ = vcall1(object + 0x0C, 0x18, W89_DESC_803718B0);
}

/// Runs the embedded interface's vtable slot `+0x10`.
///
/// In plain English: the sibling of [`fn_800dcee0`] with no argument.
///
/// # Original
/// - Ghidra: `FUN_800dcf00` at `0x800DCF00` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dcf00_FUN_800dcf00.c`
///
/// # Arguments
/// * `object` - owner of the embedded interface at `+0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever the slot does.
///
/// # Safety
/// `object + 0x0C` must start with a readable vtable.
///
/// # Confidence
/// medium - the embedded receiver and slot number are literal; the call prints
/// no operands.
pub unsafe fn fn_800dcf00(object: usize) {
    let _ = vcall0(object + 0x0C, 0x10);
}

/// Copies one `0x40`-byte record from the stream cursor and advances it.
///
/// In plain English: the pointer at `+0x5C` walks a packed record stream. This
/// body copies the record's four leading bytes to `+0xAC`, its eight words to
/// `+0xB0..+0xC8`, its four bytes at source `+0x20` to `+0xCC`, and the six
/// remaining words to `+0xD0..+0xE8`, then moves the cursor by `0x40`. The
/// export's store order is preserved inside each group.
///
/// # Original
/// - Ghidra: `FUN_800de290` at `0x800DE290` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de290_FUN_800de290.c`
///
/// # Arguments
/// * `object` - packet object; `+0x5C` is the read cursor, `+0xAC..+0xEC` the
///   copied record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 64 destination bytes and advances the cursor by 64.
///
/// # Safety
/// The cursor must point at 64 readable bytes and `object` must be writable
/// over `+0xAC..+0xEC`.
///
/// # Confidence
/// high - every offset, both halves of each 32-bit copy and the cursor
/// advance are literal.
pub unsafe fn fn_800de290(object: usize) {
    let source = read_word(object, 0x5C) as usize;

    write_byte(object, 0xAC, read_byte(source, 0));
    write_byte(object, 0xAD, read_byte(source, 1));
    write_byte(object, 0xAE, read_byte(source, 2));
    write_byte(object, 0xAF, read_byte(source, 3));
    write_word(object, 0xB0, read_word(source, 4));
    write_word(object, 0xB4, read_word(source, 8));
    write_word(object, 0xB8, read_word(source, 0x0C));
    write_word(object, 0xBC, read_word(source, 0x10));
    write_word(object, 0xC0, read_word(source, 0x14));
    write_word(object, 0xC4, read_word(source, 0x18));
    write_word(object, 0xC8, read_word(source, 0x1C));
    write_byte(object, 0xCC, read_byte(source, 0x20));
    write_byte(object, 0xCD, read_byte(source, 0x21));
    write_byte(object, 0xCE, read_byte(source, 0x22));
    write_byte(object, 0xCF, read_byte(source, 0x23));
    // observed: the export loads +0x24 before storing +0x28, so the higher
    // word is written first.
    write_word(object, 0xD4, read_word(source, 0x28));
    write_word(object, 0xD0, read_word(source, 0x24));
    write_word(object, 0xDC, read_word(source, 0x30));
    write_word(object, 0xD8, read_word(source, 0x2C));
    write_word(object, 0xE4, read_word(source, 0x38));
    write_word(object, 0xE0, read_word(source, 0x34));
    write_word(object, 0xE8, read_word(source, 0x3C));
    write_word(object, 0x5C, (source + 0x40) as u32);
}

// ---------------------------------------------------------------------------
// Wave 90: the 4 x 0x23C record container, the tail of the `dScBoot_c` state
// machine and the state objects of the `0x8015F930..0x8015FF40` band
// ---------------------------------------------------------------------------

/// `0x803272D0`: vtable written at `+0x60` by the container constructor
/// [`fn_80156ed0`].
///
/// `// inferred:` the class name is not recovered; the vtable sits at the same
/// offset as the pause-window and boot-scene base vtables.
const W90_VTABLE_803272D0: u32 = 0x8032_72D0;

/// `0x803276F0`: vtable written at `+0x60` by [`fn_80157710`].
const W90_VTABLE_803276F0: u32 = 0x8032_76F0;

/// `0x80328A80`: state-object vtable written at `+0x00` by [`fn_8015fc90`].
const W90_VTABLE_80328A80: u32 = 0x8032_8A80;

/// `0x80328B18`: vtable written at `+0x0C` of every list node by
/// [`fn_8015fef0`].
const W90_VTABLE_80328B18: u32 = 0x8032_8B18;

/// `0x80327770` / `0x80327740`: interface tables stored at `+0x390`; the second
/// one replaces the first once the state manager exists.
const W90_IFACE_80327770: u32 = 0x8032_7770;
const W90_IFACE_80327740: u32 = 0x8032_7740;

/// `0x803277A0` / `0x803277B8`: interface tables stored at `+0x398` and
/// `+0x39C` and forwarded to the state manager.
const W90_IFACE_803277A0: u32 = 0x8032_77A0;
const W90_IFACE_803277B8: u32 = 0x8032_77B8;

/// `0x80377078`: initial state descriptor handed to the state manager by
/// [`fn_80157710`].
///
/// `// unknown:` the state's name is not printed anywhere in the export.
const W90_DESC_80377078: u32 = 0x8037_7078;

/// Boot-scene state descriptors of the `0x8037_75xx..0x8037_79xx` table.
///
/// observed: every handler hands one of these to `bState` through
/// `(**(code **)(iface + 0x18))(iface, descriptor)`, exactly like the wave-78
/// descriptors. The three whose meaning the wave-78 pass already established
/// are reused from `W78_D_BOOT_*`.
const W90_DESC_803776C8: u32 = 0x8037_76C8;
const W90_DESC_80377708: u32 = 0x8037_7708;
const W90_DESC_80377748: u32 = 0x8037_7748;
const W90_DESC_803777C8: u32 = 0x8037_77C8;
const W90_DESC_80377848: u32 = 0x8037_7848;
const W90_DESC_80377888: u32 = 0x8037_7888;
const W90_DESC_80377948: u32 = 0x8037_7948;

/// `r13 - 0x5310` -> `0x8042A370`: head pointer of the doubly linked list that
/// [`fn_8015fef0`] appends to and [`fn_8015fe50`] unlinks from.
const W90_G_LIST_HEAD_5310: usize = SDA_BASE - 0x5310;

/// `r13 - 0x530C` -> `0x8042A374`: tail pointer of the same list.
const W90_G_LIST_TAIL_530C: usize = SDA_BASE - 0x530C;

/// `r13 - 0x5360` -> `0x8042A320`: pointer to the live 4 x 0x23C container
/// written by [`fn_80156ed0`].
const W90_G_CONTAINER_5360: usize = SDA_BASE - 0x5360;

/// `r13 - 0x5358` -> `0x8042A328`: pointer to the live scene object written by
/// [`fn_80157710`].
const W90_G_SCENE_5358: usize = SDA_BASE - 0x5358;

/// `r13 - 0x7710` -> `0x80427F70`: one byte cleared by [`fn_8015d330`] when the
/// controller-info step starts.
///
/// `// unknown:` the flag's owner is not named in the export.
const W90_G_BOOT_BYTE_7710: usize = SDA_BASE - 0x7710;

/// `r13 - 0x53A8` -> `0x8042A2D8`: object whose `+0xB50` word and `+0xB09` byte
/// decide the boot error path in [`fn_8015daf0`].
///
/// `// inferred:` the layout of the object comes from the two reads only; its
/// class is not recovered.
const W90_G_BOOT_OBJECT_53A8: usize = SDA_BASE - 0x53A8;

/// `r13 - 0x53A3` / `r13 - 0x53A4`: two flag bytes tested together by
/// [`fn_8015daf0`].
///
/// `// unknown:` both owners are unrecovered; the body only tests them.
const W90_G_BOOT_BYTE_53A3: usize = SDA_BASE - 0x53A3;
const W90_G_BOOT_BYTE_53A4: usize = SDA_BASE - 0x53A4;

/// `0x80006AA0`: constructor of the helper embedded at `+0x74` of the record
/// container.
const W90_FN_80006AA0: usize = 0x8000_6AA0;

/// `0x80006BF0`: readiness query of that same `+0x74` helper.
const W90_FN_80006BF0: usize = 0x8000_6BF0;

/// `0x800B14E0` / `0x800B16E0`: element constructor and destructor handed to
/// the `0x802DC9A0` array builder.
const W90_FN_800B14E0: usize = 0x800B_14E0;
const W90_FN_800B16E0: usize = 0x800B_16E0;

/// `0x800B1910`: per-element step called four times by [`fn_80157060`].
const W90_FN_800B1910: usize = 0x800B_1910;

/// `0x800B19B0`: per-element readiness query called by [`fn_80157120`].
const W90_FN_800B19B0: usize = 0x800B_19B0;

/// `0x800B2550`: per-element operation used by [`fn_801571a0`].
const W90_FN_800B2550: usize = 0x800B_2550;

/// `0x800CEE20`: save-manager shutdown used by [`fn_8015d330`].
const W90_FN_800CEE20: usize = 0x800C_EE20;

/// `0x800E03B0`: selects one record of the save image's array B; the twin of
/// [`FN_800E0380`] and of [`save_image_record_b`].
const W90_FN_800E03B0: usize = 0x800E_03B0;

/// `0x800E0450`: save-image readiness query; returns `0` when the image is
/// still valid, which the boot handlers treat as "nothing to recover".
const W90_FN_800E0450: usize = 0x800E_0450;

/// `0x800E04B0` / `0x800E0540`: the two save-image finalisers called after the
/// three-record rebuild in [`fn_8015d530`].
const W90_FN_800E04B0: usize = 0x800E_04B0;
const W90_FN_800E0540: usize = 0x800E_0540;

/// `0x8015F9A0`: state-object base constructor, documented in
/// [`crate::game::task::state_object_init`] as
/// `(object, name, value, sub)`.
const W90_FN_8015F9A0: usize = 0x8015_F9A0;

/// Constructs the 4 x 0x23C record container.
///
/// In plain English: run the C++ base constructor, install the `0x803272D0`
/// vtable at `+0x60`, construct the helper at `+0x74`, build the four
/// 0x23C-byte records at `+0x148` with the `0x802DC9A0` array builder, and
/// publish the object through `r13 - 0x5360`.
///
/// # Original
/// - Ghidra: `FUN_80156ed0` at `0x80156ED0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156ed0_FUN_80156ed0.c`
///
/// # Arguments
/// * `container` - storage for the object; at least `0x148 + 4 * 0x23C` bytes.
///
/// # Returns
/// `container`, the value the C++ constructor chain returns.
///
/// # Side effects
/// Five callee writes, one global word store and the whole object body.
///
/// # Safety
/// `container` must be a live, writable allocation of the size above and the
/// array builder must accept the `0x23C`/`4` pair.
///
/// # Confidence
/// medium - every offset, the vtable and the element size/count are literal;
/// the C++ base constructor is printed with no argument, so its receiver is
/// inferred to be `container`.
pub unsafe fn fn_80156ed0(container: usize) -> usize {
    let _ = platform::call0(FN_8006C420);
    write_word(container, 0x60, W90_VTABLE_803272D0);
    let _ = platform::call1(W90_FN_80006AA0, (container + 0x74) as u32);
    let _ = platform::call5(
        FN_802DC9A0,
        (container + 0x148) as u32,
        W90_FN_800B14E0 as u32,
        W90_FN_800B16E0 as u32,
        0x23C,
        4,
    );
    write_global_word(W90_G_CONTAINER_5360, container as u32);
    container
}

/// Steps all four records of the container.
///
/// In plain English: unless `0x800B3B50(-1)` reports the current mode, call
/// `0x800B1910` on each of the four 0x23C-byte records. Always returns `1`.
///
/// # Original
/// - Ghidra: `FUN_80157060` at `0x80157060` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157060_FUN_80157060.c`
///
/// # Arguments
/// * `container` - object built by [`fn_80156ed0`].
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Up to four `0x800B1910` calls.
///
/// # Safety
/// `container + 0x148 .. +0x148 + 4 * 0x23C` must hold live records.
///
/// # Confidence
/// high - the mode query, the `-1` argument, the loop count and the stride are
/// literal.
pub unsafe fn fn_80157060(container: usize) -> u32 {
    if platform::call1(FN_800B3B50, 0xFFFF_FFFF) == 0 {
        let mut element = container + 0x148;
        let mut pending = 4;
        while pending != 0 {
            let _ = platform::call1(W90_FN_800B1910, element as u32);
            element += 0x23C;
            pending -= 1;
        }
    }
    1
}

/// Reports whether the container and all four records are ready.
///
/// In plain English: the `+0x74` helper must report ready, then every one of the
/// four records must answer `0x800B19B0` non-zero.
///
/// # Original
/// - Ghidra: `FUN_80157120` at `0x80157120` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157120_FUN_80157120.c`
///
/// # Arguments
/// * `container` - object built by [`fn_80156ed0`].
///
/// # Returns
/// `1` when the helper and all four records are ready, `0` otherwise.
///
/// # Side effects
/// Up to five read-only queries.
///
/// # Safety
/// `container + 0x74` and the four records must be live.
///
/// # Confidence
/// high - the guard order, the count and the stride are literal.
pub unsafe fn fn_80157120(container: usize) -> u32 {
    if platform::call1(W90_FN_80006BF0, (container + 0x74) as u32) == 0 {
        return 0;
    }
    let mut element = container + 0x148;
    let mut pending = 4;
    while pending != 0 {
        if platform::call1(W90_FN_800B19B0, element as u32) == 0 {
            return 0;
        }
        element += 0x23C;
        pending -= 1;
    }
    1
}

/// Applies `0x800B2550` to one record of the container.
///
/// In plain English: the record base is `container + slot * 0x23C + 0x148`;
/// there is no bounds check in the original.
///
/// # Original
/// - Ghidra: `FUN_801571a0` at `0x801571A0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801571a0_FUN_801571a0.c`
///
/// # Arguments
/// * `container` - object built by [`fn_80156ed0`].
/// * `slot` - record index; the caller must keep it below `4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x800B2550` call on the selected record.
///
/// # Safety
/// `slot` must be below `4` so the computed base stays inside the array.
///
/// # Confidence
/// high - the stride, the `+0x148` bias and the call are literal.
pub unsafe fn fn_801571a0(container: usize, slot: u32) {
    let record = container + slot as usize * 0x23C + 0x148;
    let _ = platform::call1(W90_FN_800B2550, record as u32);
}

/// Stores a value in the first free record matching a player and mode.
///
/// In plain English: mode `8` means "player 1"; scan up to four records for the
/// first one whose `+0x381` flag is set and whose `+0x368`/`+0x364` words match
/// the mode and player, then write `value` at `+0x378` and set `+0x382`.
///
/// # Original
/// - Ghidra: `FUN_801571b0` at `0x801571B0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801571b0_FUN_801571b0.c`
///
/// # Arguments
/// * `records` - base of the record array; the caller passes `container + 0x148`
///   (`// inferred:` from [`fn_801571a0`], whose stride and bias match).
/// * `player` - player index to match against `+0x364`.
/// * `mode` - mode word to match against `+0x368`; `8` forces `player` to `1`.
/// * `value` - word stored at `+0x378` of the match.
///
/// # Returns
/// Nothing. The function simply returns when no record matches.
///
/// # Side effects
/// Writes `+0x378` and `+0x382` of the matched record.
///
/// # Safety
/// `records .. records + 4 * 0x23C` must be readable and the match writable.
///
/// # Confidence
/// high - the scan bound, the three compared fields and both stores are literal.
pub unsafe fn fn_801571b0(records: usize, player: i32, mode: i32, value: u32) {
    let player = if mode == 8 { 1 } else { player };

    let mut record = records;
    let mut pending = 4i32;
    while read_byte(record, 0x381) == 0
        || mode != read_word(record, 0x368) as i32
        || player != read_word(record, 0x364) as i32
    {
        record += 0x23C;
        pending -= 1;
        if pending == 0 {
            return;
        }
    }

    write_word(record, 0x378, value);
    write_byte(record, 0x382, 1);
}

/// Stores a value in the record matching a player and mode, unrolled.
///
/// In plain English: the same select-then-store as [`fn_801571b0`], but the
/// four records are tested with a straight chain of comparisons at
/// `+0x381/+0x368/+0x364`, `+0x5BD/+0x5A4/+0x5A0`, `+0x7F9/+0x7E0/+0x7DC` and
/// `+0xA35/+0xA1C/+0xA18`. The first match stores `value` at
/// `+0x378/+0x5B4/+0x7F0/+0xA2C` respectively and returns.
///
/// # Original
/// - Ghidra: `FUN_80157220` at `0x80157220` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157220_FUN_80157220.c`
///
/// # Arguments
/// * `container` - base of the record array; the four records are laid out at a
///   `0x23C` stride.
/// * `player` - player index to match against `+0x364` of each record.
/// * `mode` - mode word to match against `+0x368`; `8` forces `player` to `1`.
/// * `value` - word stored at `+0x378` of the match.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// At most one word store.
///
/// # Safety
/// `container .. container + 4 * 0x23C` must be readable and the match writable.
///
/// # Confidence
/// high - all four comparisons and the four offsets are literal and the stride
/// `0x23C` reproduces every printed offset.
pub unsafe fn fn_80157220(container: usize, player: i32, mode: i32, value: u32) {
    let player = if mode == 8 { 1 } else { player };

    if read_byte(container, 0x381) != 0
        && mode == read_word(container, 0x368) as i32
        && player == read_word(container, 0x364) as i32
    {
        write_word(container, 0x378, value);
        return;
    }
    if read_byte(container, 0x5BD) != 0
        && mode == read_word(container, 0x5A4) as i32
        && player == read_word(container, 0x5A0) as i32
    {
        write_word(container, 0x5B4, value);
        return;
    }
    if read_byte(container, 0x7F9) != 0
        && mode == read_word(container, 0x7E0) as i32
        && player == read_word(container, 0x7DC) as i32
    {
        write_word(container, 0x7F0, value);
        return;
    }
    if read_byte(container, 0xA35) == 0 {
        return;
    }
    if mode != read_word(container, 0xA1C) as i32 {
        return;
    }
    if player != read_word(container, 0xA18) as i32 {
        return;
    }
    write_word(container, 0xA2C, value);
}

/// Sets one byte of the display object's per-player table.
///
/// In plain English: normalise mode `8` to player `1`, reject a column outside
/// `0..=0x15` and a player outside `0..=3`, then set
/// `0xAFE + player * 0x16 + column` in the object published at `r13 - 0x5704`.
///
/// # Original
/// - Ghidra: `FUN_80157310` at `0x80157310` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157310_FUN_80157310.c`
///
/// # Arguments
/// * `owner` - unused by the export; every write goes through the global.
/// * `player` - player row; `8` in `column` forces it to `1`.
/// * `column` - table column, `0..=0x15`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One byte store when both indices are in range.
///
/// # Safety
/// [`G_DISPLAY_OBJECT`] must point at a table of at least `0xAFE + 4 * 0x16`
/// bytes.
///
/// # Confidence
/// high - both bounds, the `8` special case and the indexed store are literal.
pub unsafe fn fn_80157310(owner: u32, player: i32, column: i32) {
    let _ = owner;
    let player = if column == 8 { 1 } else { player };
    if column < 0 || 0x15 < column || player < 0 || 3 < player {
        return;
    }
    let table = read_global_word(G_DISPLAY_OBJECT) as usize;
    write_byte(table, player as usize * 0x16 + column as usize + 0xAFE, 1);
}

/// Clears one byte of the display object's per-player table.
///
/// In plain English: the exact twin of [`fn_80157310`] with a `0` store.
///
/// # Original
/// - Ghidra: `FUN_80157360` at `0x80157360` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157360_FUN_80157360.c`
///
/// # Arguments
/// * `owner` - unused by the export.
/// * `player` - player row; `8` in `column` forces it to `1`.
/// * `column` - table column, `0..=0x15`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One byte store when both indices are in range.
///
/// # Safety
/// [`G_DISPLAY_OBJECT`] must point at the same table as [`fn_80157310`].
///
/// # Confidence
/// high - only the stored constant differs from [`fn_80157310`].
pub unsafe fn fn_80157360(owner: u32, player: i32, column: i32) {
    let _ = owner;
    let player = if column == 8 { 1 } else { player };
    if column < 0 || 0x15 < column || player < 0 || 3 < player {
        return;
    }
    let table = read_global_word(G_DISPLAY_OBJECT) as usize;
    write_byte(table, player as usize * 0x16 + column as usize + 0xAFE, 0);
}

/// Constructs the scene object that owns a layout and a state manager.
///
/// In plain English: run the C++ base constructor, install the `0x803276F0`
/// vtable at `+0x60`, construct the layout at `+0x70` and the sub-object at
/// `+0x208`, retag `+0x208` with `0x80329958`, clear the six fields at
/// `+0x31C..+0x32C`, publish the three interface tables at `+0x390..+0x39C`,
/// build the state manager at `+0x3A8` with the `0x80377078` initial state,
/// swap `+0x390` to `0x80327740`, set the `+0x44B`/`+0x452` bytes and publish
/// the object through `r13 - 0x5358`.
///
/// # Original
/// - Ghidra: `FUN_80157710` at `0x80157710` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157710_FUN_80157710.c`
///
/// # Arguments
/// * `scene` - storage for the object; at least `0x453` bytes.
///
/// # Returns
/// `scene`, the value the C++ constructor chain returns.
///
/// # Side effects
/// The layout, sub-object and state-manager constructors plus roughly twenty
/// field writes and one global pointer store.
///
/// # Safety
/// `scene` must be a live, writable allocation and the three constructors must
/// accept the addresses handed to them.
///
/// # Confidence
/// medium - every offset, vtable and table address is literal; the base
/// constructor and `0x802D7AA0` are printed without a receiver, and the roles of
/// `+0x44B`/`+0x452` are `// unknown:`.
pub unsafe fn fn_80157710(scene: usize) -> usize {
    let _ = platform::call0(FN_8006C420);
    write_word(scene, 0x60, W90_VTABLE_803276F0);
    let _ = platform::call1(FN_800C8950, (scene + 0x70) as u32);

    let _ = platform::call1(FN_802D7AA0, (scene + 0x208) as u32);
    write_word(scene, 0x208, VTABLE_80329958);

    write_word(scene, 0x31C, 0);
    write_word(scene, 0x320, 0);
    write_byte(scene, 0x324, 0);
    write_byte(scene, 0x325, 0);
    write_word(scene, 0x328, 0);
    write_word(scene, 0x32C, 0);

    write_word(scene, 0x390, W90_IFACE_80327770);
    write_word(scene, 0x394, VTABLE_SHARED_IFACE);
    write_word(scene, 0x398, W90_IFACE_803277A0);
    write_word(scene, 0x39C, W90_IFACE_803277B8);
    write_word(scene, 0x3A0, scene as u32);
    write_word(scene, 0x3A4, 0);

    let _ = platform::call4(
        FN_8015FC90,
        (scene + 0x3A8) as u32,
        (scene + 0x394) as u32,
        (scene + 0x398) as u32,
        W90_DESC_80377078,
    );

    write_word(scene, 0x390, W90_IFACE_80327740);
    write_byte(scene, 0x44B, 0);
    write_byte(scene, 0x452, 1);
    write_global_word(W90_G_SCENE_5358, scene as u32);
    scene
}

/// Releases a scene object when a free count is positive.
///
/// In plain English: the `delete`-style helper of [`fn_80157710`]; the export
/// prints `0x802B9280()` with no operand.
///
/// # Original
/// - Ghidra: `FUN_80157850` at `0x80157850` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157850_FUN_80157850.c`
///
/// # Arguments
/// * `object` - object to release; a null pointer is returned untouched.
/// * `free` - when positive, the release call runs.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// One `0x802B9280` call when both guards pass.
///
/// # Safety
/// `object` must be a live allocation when `free` is positive.
///
/// # Confidence
/// medium - both guards and the return are literal; the freed receiver is
/// dropped by the export and is inferred to be `object`, as in the wave-78
/// destructors.
pub unsafe fn fn_80157850(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Releases the state object embedded at `+0x18`, then the scene object.
///
/// In plain English: release `0x8015F9F0(object + 0x18, 0)`, then the
/// `delete`-style `0x802B9280(object)` when `free` is positive.
///
/// # Original
/// - Ghidra: `FUN_80157890` at `0x80157890` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80157890_FUN_80157890.c`
///
/// # Arguments
/// * `object` - scene object; `+0x18` holds the embedded state object.
/// * `free` - when positive, the outer release call runs.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// One nested release and, when `free` is positive, one object release.
///
/// # Safety
/// `object` must be a live allocation and `object + 0x18` a live state object.
///
/// # Confidence
/// high - both guards, the `+0x18` bias, the `0` free flag and the printed
/// receiver of the outer call are literal.
pub unsafe fn fn_80157890(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Ages the controller-info timer and leaves the state when it expires.
///
/// In plain English: decrement `+0x108` when it is non-zero, ask the view for
/// event `0x7F1F`, and on that event (or when display-input bit `0x80000` is
/// set) write `4` to `(+0xFC)->+0x20C` and zero the timer. Then enter the
/// `0x803776C8` state whenever the timer is below one.
///
/// # Original
/// - Ghidra: `FUN_8015d1d0` at `0x8015D1D0` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d1d0_FUN_8015d1d0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x108` is the frame timer, `+0xFC` the
///   controller-info object and `+0x74` the state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One timer decrement, one virtual query, up to three stores and one state
/// change.
///
/// # Safety
/// The view global and `boot + 0xFC` must be live.
///
/// # Confidence
/// medium - the decrement, both guards, the `4` and the state change are
/// literal; the view query's receiver is dropped by the export and the role of
/// `+0x20C` is `// inferred:` from the write value.
pub unsafe fn fn_8015d1d0(boot: usize) {
    let remaining = platform::read_i32(boot + 0x108);
    if remaining != 0 {
        platform::write_i32(boot + 0x108, remaining - 1);
    }

    let view = read_global_word(W78_G_VIEW_OBJECT) as usize;
    let event = vcall1(view, 0x20, 0x7F1F);
    if event != 0 || (read_global_word(G_DISPLAY_INPUT_FLAGS) & 0x80000) != 0 {
        let info = read_word(boot, 0xFC) as usize;
        write_word(info, 0x20C, 4);
        write_word(boot, 0x108, 0);
    }

    if platform::read_i32(boot + 0x108) < 1 {
        change_state(boot + 0x74, W90_DESC_803776C8);
    }
}

/// Enters the scene-selection state while the save system is idle.
///
/// In plain English: when `(+0x56C8)->+0x74` is zero and the layout is not in
/// mode `5`, enter the `0x80377708` state.
///
/// # Original
/// - Ghidra: `FUN_8015d2a0` at `0x8015D2A0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d2a0_FUN_8015d2a0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x74` is the state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two reads and, when both guards pass, one state change.
///
/// # Safety
/// The save-manager global and the layout getter must return live objects.
///
/// # Confidence
/// medium - the two guards and the descriptor are literal; the meanings of the
/// layout words `+0x08`/`+0x0C` are `// inferred:` from the compared value `5`.
pub unsafe fn fn_8015d2a0(boot: usize) {
    let manager = read_global_word(G_SAVE_MANAGER) as usize;
    if read_word(manager, 0x74) != 0 {
        return;
    }

    let layout = call0_r(W78_FN_80107F10) as usize;
    let scene_mode = read_word(layout, 0x0C) == 5 || read_word(layout, 0x08) == 5;
    if !scene_mode {
        change_state(boot + 0x74, W90_DESC_80377708);
    }
}

/// Raises the busy bit, hides the layout and shuts the save manager down.
///
/// In plain English: set bit `0x40` of the display manager's `+0x1D8` flags, ask
/// the layout to hide, call `0x800CEE20` on the save manager and clear the
/// `+0x100` byte.
///
/// # Original
/// - Ghidra: `FUN_8015d330` at `0x8015D330` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d330_FUN_8015d330.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x100` is the "controller info shown" byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One flag update, one layout update, one manager call, one byte store and the
/// `r13 - 0x7710` byte clear.
///
/// # Safety
/// The display-manager and layout getters must return live objects.
///
/// # Confidence
/// high - every mask, call and store is literal.
pub unsafe fn fn_8015d330(boot: usize) {
    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) | 0x40);

    let layout = call0_r(W78_FN_80107F10);
    let _ = platform::call2(W78_FN_80108160, layout, 1);

    let _ = platform::call1(W90_FN_800CEE20, read_global_word(G_SAVE_MANAGER));
    write_global_byte(W90_G_BOOT_BYTE_7710, 0);
    write_byte(boot, 0x100, 0);
}

/// Chooses the next boot state from the save-recovery result.
///
/// In plain English: when `0x800E0450(image)` returns zero, look at the save
/// manager: a non-zero `+0x78` means "error fade", otherwise `+0x7C` picks
/// between `0x80377748` and `0x803777C8`.
///
/// # Original
/// - Ghidra: `FUN_8015d3a0` at `0x8015D3A0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d3a0_FUN_8015d3a0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x74` is the state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One readiness query and one state change.
///
/// # Safety
/// The save-image and save-manager globals must be live.
///
/// # Confidence
/// medium - the three-way branch, the descriptors and the guard are literal; the
/// roles of `+0x78` and `+0x7C` are `// inferred:` from the sibling handlers.
pub unsafe fn fn_8015d3a0(boot: usize) {
    if platform::call1(W90_FN_800E0450, read_global_word(G_SAVE_IMAGE)) != 0 {
        return;
    }

    let manager = read_global_word(G_SAVE_MANAGER) as usize;
    if read_word(manager, 0x78) != 0 {
        change_state(boot + 0x74, W78_D_BOOT_GO_TO_ERROR_FADE);
    } else if read_byte(manager, 0x7C) == 0 {
        change_state(boot + 0x74, W90_DESC_80377748);
    } else {
        change_state(boot + 0x74, W90_DESC_803777C8);
    }
}

/// Rebuilds all three record pairs of the live save image.
///
/// In plain English: the rebuild half of [`save_image_construct`]; open the
/// `+0x20` sub-object, then for each of the three slots select record A, flush
/// with `0x800CDED0`, select record B, flush again, and finish with the two
/// save-image finalisers.
///
/// # Original
/// - Ghidra: `FUN_8015d530` at `0x8015D530` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d530_FUN_8015d530.c`
///
/// # Arguments
/// None; the image comes from [`G_SAVE_IMAGE`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two sub-object calls, six record selections, six flushes and two finalisers.
///
/// # Safety
/// [`G_SAVE_IMAGE`] must point at the object built by [`save_image_construct`].
///
/// # Confidence
/// high - the three-iteration loop, the two record selectors, both flush calls
/// and the two finalisers are literal.
pub unsafe fn fn_8015d530() {
    let image = read_global_word(G_SAVE_IMAGE) as usize;
    let _ = platform::call1(FN_800CE580, (image + 0x20) as u32);

    let mut slot = 0i32;
    while slot < 3 {
        let _ = platform::call2(FN_800E0380, image as u32, slot as u32);
        let _ = platform::call0(FN_800CDED0);
        let _ = platform::call2(W90_FN_800E03B0, image as u32, slot as u32);
        let _ = platform::call0(FN_800CDED0);
        slot += 1;
    }

    let _ = platform::call1(W90_FN_800E04B0, image as u32);
    let _ = platform::call1(W90_FN_800E0540, image as u32);
}

/// Leaves the controller screen once the save image is idle.
///
/// In plain English: when `0x800E0450(image)` returns zero, set `+0x100` if the
/// save manager has no `+0x78` work, clear the scene-finished byte and enter the
/// `0x80377848` state.
///
/// # Original
/// - Ghidra: `FUN_8015d5b0` at `0x8015D5B0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d5b0_FUN_8015d5b0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x100` is the "controller info shown" byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One conditional byte store, one global byte store and one state change.
///
/// # Safety
/// The save-image and save-manager globals must be live.
///
/// # Confidence
/// high - the guard, the conditional store, the global clear and the descriptor
/// are literal.
pub unsafe fn fn_8015d5b0(boot: usize) {
    if platform::call1(W90_FN_800E0450, read_global_word(G_SAVE_IMAGE)) != 0 {
        return;
    }

    let manager = read_global_word(G_SAVE_MANAGER) as usize;
    if read_word(manager, 0x78) == 0 {
        write_byte(boot, 0x100, 1);
    }

    write_global_byte(W78_G_SCENE_FINISHED, 0);
    change_state(boot + 0x74, W90_DESC_80377848);
}

/// Restores the display before the final boot transition.
///
/// In plain English: when `0x800E0450(image)` returns zero, an error fade is
/// requested if the save manager has `+0x78` work; otherwise clear bit `0x40` of
/// the display flags, hide the layout and enter `0x80377948`.
///
/// # Original
/// - Ghidra: `FUN_8015d650` at `0x8015D650` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d650_FUN_8015d650.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x74` is the state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One readiness query, one flag update, one layout update and one state change.
///
/// # Safety
/// All four globals/getters must return live objects.
///
/// # Confidence
/// high - both branches, the mask and the descriptors are literal.
pub unsafe fn fn_8015d650(boot: usize) {
    if platform::call1(W90_FN_800E0450, read_global_word(G_SAVE_IMAGE)) != 0 {
        return;
    }

    let manager = read_global_word(G_SAVE_MANAGER) as usize;
    if read_word(manager, 0x78) != 0 {
        change_state(boot + 0x74, W78_D_BOOT_GO_TO_ERROR_FADE);
        return;
    }

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);

    let layout = call0_r(W78_FN_80107F10);
    let _ = platform::call2(W78_FN_80108160, layout, 0);

    change_state(boot + 0x74, W90_DESC_80377948);
}

/// Leaves the save screen once the owner's `+0x297` byte is clear.
///
/// In plain English: when `(+0xF4)->+0x297` is zero, clear bit `0x40` of the
/// display flags, hide the layout, clear `+0x100` and enter `0x80377948`.
///
/// # Original
/// - Ghidra: `FUN_8015d860` at `0x8015D860` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d860_FUN_8015d860.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0xF4` is the owning save object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One flag update, one layout update, one byte store and one state change.
///
/// # Safety
/// `boot + 0xF4` must point at a live object and the two getters must succeed.
///
/// # Confidence
/// high - the guard, the mask, the stores and the descriptor are literal; the
/// meaning of `+0x297` is `// unknown:`.
pub unsafe fn fn_8015d860(boot: usize) {
    let owner = read_word(boot, 0xF4) as usize;
    if read_byte(owner, 0x297) != 0 {
        return;
    }

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);

    let layout = call0_r(W78_FN_80107F10);
    let _ = platform::call2(W78_FN_80108160, layout, 0);

    write_byte(boot, 0x100, 0);
    change_state(boot + 0x74, W90_DESC_80377948);
}

/// Starts the 30-frame transition and shows the layout.
///
/// In plain English: clear the scene-finished byte, run `0x800B0DB0(0)` and
/// `0x800B0A90(0x1E)`, then show the layout with `0x80108160(layout, 1)`.
///
/// # Original
/// - Ghidra: `FUN_8015d8f0` at `0x8015D8F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d8f0_FUN_8015d8f0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One global byte store and three calls.
///
/// # Safety
/// The layout getter must return a live object.
///
/// # Confidence
/// high - the two timer calls, the `0x1E` constant and the layout operand are
/// literal.
pub unsafe fn fn_8015d8f0() {
    write_global_byte(W78_G_SCENE_FINISHED, 0);
    let _ = platform::call1(W78_FN_800B0DB0, 0);
    let _ = platform::call1(W78_FN_800B0A90, 0x1E);

    let layout = call0_r(W78_FN_80107F10);
    let _ = platform::call2(W78_FN_80108160, layout, 1);
}

/// Clears both `+0x209` bytes when no player is connected.
///
/// In plain English: when the player-manager query reports zero players, clear
/// `(+0xF0)->+0x209` and `(+0xFC)->+0x209`, then enter `0x80377848`.
///
/// # Original
/// - Ghidra: `FUN_8015d930` at `0x8015D930` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d930_FUN_8015d930.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0xF0` and `+0xFC` are the two save objects.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two byte clears and one state change.
///
/// # Safety
/// Both embedded objects must be live when the guard passes.
///
/// # Confidence
/// medium - the printed guard is `countLeadingZeros(players) >> 5 != 0`, which is
/// true only when the count is zero; both clears and the descriptor are literal;
/// the manager query's receiver is dropped by the export.
pub unsafe fn fn_8015d930(boot: usize) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);

    // observed: `countLeadingZeros(players) >> 5 != 0` is true only for zero.
    if players != 0 {
        return;
    }

    let first = read_word(boot, 0xF0) as usize;
    let second = read_word(boot, 0xFC) as usize;
    write_byte(first, 0x209, 0);
    write_byte(second, 0x209, 0);
    change_state(boot + 0x74, W90_DESC_80377848);
}

/// Leaves the boot sequence when exactly one player is present.
///
/// In plain English: when the player-manager query reports exactly one player,
/// latch the scene-finished byte and branch on the `+0x100` byte: clear it via
/// the layout and enter the error window, or enter `0x80377888` directly.
///
/// # Original
/// - Ghidra: `FUN_8015d9f0` at `0x8015D9F0` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d9f0_FUN_8015d9f0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x100` is the "controller info shown" byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One global byte store per branch, one layout update in the first branch and
/// one state change.
///
/// # Safety
/// The layout getter must return a live object.
///
/// # Confidence
/// medium - the printed guard is `countLeadingZeros(players - 1) >> 5 != 0`, which
/// is true only when the count is one; both branches and the descriptors are
/// literal and the manager query's receiver is dropped by the export.
pub unsafe fn fn_8015d9f0(boot: usize) {
    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let players = vcall0(manager, 0x10);

    // observed: `countLeadingZeros(players - 1) >> 5 != 0` is true only for one.
    if players != 1 {
        return;
    }

    if read_byte(boot, 0x100) == 0 {
        write_global_byte(W78_G_SCENE_FINISHED, 1);
        let layout = call0_r(W78_FN_80107F10);
        let _ = platform::call2(W78_FN_80108160, layout, 0);
        change_state(boot + 0x74, W78_D_BOOT_ERROR_WINDOW_ON);
    } else {
        write_global_byte(W78_G_SCENE_FINISHED, 1);
        change_state(boot + 0x74, W90_DESC_80377888);
    }
}

/// Chooses between the reset and the error state from the boot object flags.
///
/// In plain English: clear bit `0x40` of the display flags, then look at
/// `(+0x53A8)->+0xB50`. Mode `6` enters `0x803776C8` when `+0xB09` is clear;
/// any other mode enters `0x803779C8` unless both `+0x53A3` and `+0x53A4` are
/// set.
///
/// # Original
/// - Ghidra: `FUN_8015daf0` at `0x8015DAF0` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015daf0_FUN_8015daf0.c`
///
/// # Arguments
/// * `boot` - `dScBoot_c` object; `+0x74` is the state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to two flag updates and one state change.
///
/// # Safety
/// The boot object global and the display-manager getter must be live.
///
/// # Confidence
/// medium - the mode `6`, both flag bytes and all three descriptors are literal;
/// the class behind `r13 - 0x53A8` and the roles of `+0xB50`/`+0xB09` are
/// `// unknown:` and marked inferred in the constants above.
pub unsafe fn fn_8015daf0(boot: usize) {
    let flag_object = read_global_word(W90_G_BOOT_OBJECT_53A8) as usize;

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);

    if read_word(flag_object, 0xB50) as i32 == 6 {
        if read_byte(flag_object, 0xB09) == 0 {
            let display = call0_r(W78_FN_80109340) as usize;
            write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);
            change_state(boot + 0x74, W90_DESC_803776C8);
        }
        return;
    }

    let both =
        read_global_byte(W90_G_BOOT_BYTE_53A3) != 0 && read_global_byte(W90_G_BOOT_BYTE_53A4) != 0;
    if both {
        return;
    }

    let display = call0_r(W78_FN_80109340) as usize;
    write_word(display, 0x1D8, read_word(display, 0x1D8) & !0x40);
    change_state(boot + 0x74, W78_D_BOOT_ERROR_INFINITY);
}

/// Constructor thunk over `0x802DCBFC`.
///
/// In plain English: `0x802DCBFC` reads the descriptor at `first + 0x18` and
/// dispatches to the selected constructor member with `second` as receiver.
///
/// # Original
/// - Ghidra: `FUN_8015dd70` at `0x8015DD70` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015dd70_FUN_8015dd70.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - receiver forwarded to the dispatched method.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever `0x802DCBFC` does; it is not analysed here.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - `mr r5,r3` / `mr r3,r4` / `addi r12,r5,K` are literal; `interface_call`
/// models the descriptor and the receiver.
pub unsafe fn fn_8015dd70(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Enters the pending state through the state object's `+0x30` slot.
///
/// In plain English: when the `+0x14` holder is idle, the `+0x0C` "busy" byte is
/// clear and the `+0x0E` "entered" byte is clear, set busy, remember the holder
/// at `+0x1C`, run slot `+0x30`, store the result as the `+0x0E` byte and clear
/// busy.
///
/// # Original
/// - Ghidra: `FUN_8015fa30` at `0x8015FA30` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015fa30_FUN_8015fa30.c`
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x0C`/`+0x0E` bytes, `+0x14`
///   holder pointer, `+0x1C` saved holder.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to two virtual calls and four field writes.
///
/// # Safety
/// `state` and `state + 0x14`'s target must be live objects with the cited
/// vtables.
///
/// # Confidence
/// medium - every offset, slot and store is literal; the receiver of the holder's
/// `+0x0C` query is dropped by the export and is `// inferred:` to be the holder.
pub unsafe fn fn_8015fa30(state: usize) {
    let holder = read_word(state, 0x14) as usize;
    let holder_vtable = read_word(holder, 0x00) as usize;
    let query = read_word(holder_vtable, 0x0C) as usize;
    let busy = platform::call1(query, holder as u32);

    if busy != 0 || read_byte(state, 0x0C) != 0 || read_byte(state, 0x0E) != 0 {
        return;
    }

    write_byte(state, 0x0C, 1);
    write_word(state, 0x1C, holder as u32);

    let vtable = read_word(state, 0x00) as usize;
    let enter = read_word(vtable, 0x30) as usize;
    let entered = platform::call1(enter, state as u32);

    write_byte(state, 0x0E, if entered != 0 { 1 } else { 0 });
    write_byte(state, 0x0C, 0);
}

/// Leaves the pending state through the state object's `+0x38` slot.
///
/// In plain English: the counterpart of [`fn_8015fa30`]; while the holder is
/// idle, the `+0x0E` byte is set and the `+0x0C` byte is clear, set busy, copy
/// `+0x1C` to `+0x18`, run slot `+0x38`, clear `+0x0E` and clear busy.
///
/// # Original
/// - Ghidra: `FUN_8015fb80` at `0x8015FB80` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015fb80_FUN_8015fb80.c`
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x0C`/`+0x0E` bytes, `+0x14`
///   holder pointer, `+0x18`/`+0x1C` saved holder pair.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls and five field writes.
///
/// # Safety
/// `state` and its `+0x14` target must be live objects with the cited vtables.
///
/// # Confidence
/// medium - every offset, slot and store is literal; the receiver of the holder's
/// `+0x0C` query is dropped by the export and is `// inferred:` to be the holder.
pub unsafe fn fn_8015fb80(state: usize) {
    let holder = read_word(state, 0x14) as usize;
    let holder_vtable = read_word(holder, 0x00) as usize;
    let query = read_word(holder_vtable, 0x0C) as usize;
    let busy = platform::call1(query, holder as u32);

    if busy != 0 || read_byte(state, 0x0E) == 0 || read_byte(state, 0x0C) != 0 {
        return;
    }

    write_byte(state, 0x0C, 1);
    write_word(state, 0x18, read_word(state, 0x1C));

    let vtable = read_word(state, 0x00) as usize;
    let leave = read_word(vtable, 0x38) as usize;
    let _ = platform::call1(leave, state as u32);

    write_byte(state, 0x0E, 0);
    write_byte(state, 0x0C, 0);
}

/// Constructs a state manager and retags its vtable.
///
/// In plain English: run the state-object base constructor `0x8015F9A0` with the
/// two interface tables and the initial state, then overwrite the vtable at
/// `+0x00` with `0x80328A80`.
///
/// # Original
/// - Ghidra: `FUN_8015fc90` at `0x8015FC90` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015fc90_FUN_8015fc90.c`
///
/// # Arguments
/// * `manager` - storage for the state manager; `+0x00` becomes the vtable.
/// * `interface_a` - first interface table, stored at `+0x04` by the base
///   constructor.
/// * `interface_b` - second interface table, stored at `+0x08`.
/// * `initial_state` - descriptor stored at `+0x14`.
///
/// # Returns
/// `manager`.
///
/// # Side effects
/// The base constructor's eleven stores plus the vtable overwrite.
///
/// # Safety
/// `manager` must be writable for at least `0x24` bytes.
///
/// # Confidence
/// medium - the vtable and the return are literal; the export prints the base
/// constructor with no operands at all, so the four arguments are `// inferred:`
/// from the three call sites in this file (the pause window and
/// [`fn_80157710`]).
pub unsafe fn fn_8015fc90(
    manager: usize,
    interface_a: u32,
    interface_b: u32,
    initial_state: u32,
) -> usize {
    let _ = platform::call4(
        W90_FN_8015F9A0,
        manager as u32,
        interface_a,
        interface_b,
        initial_state,
    );
    write_word(manager, 0x00, W90_VTABLE_80328A80);
    manager
}

/// Unlinks one node from the intrusive list and optionally frees it.
///
/// In plain English: patch the neighbours' `+0x08`/`+0x04` links, repair the
/// head/tail globals when this node is an end, clear both links and release the
/// node through `0x802B9280` when `free` is positive.
///
/// # Original
/// - Ghidra: `FUN_8015fe50` at `0x8015FE50` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015fe50_FUN_8015fe50.c`
///
/// # Arguments
/// * `node` - node to unlink; `+0x04` is the previous link, `+0x08` the next.
/// * `free` - when positive, the node is released.
///
/// # Returns
/// `node`.
///
/// # Side effects
/// Up to four link writes, both global repairs and one release.
///
/// # Safety
/// `node` must be a node of this list or the links and the head/tail repair are
/// meaningless.
///
/// # Confidence
/// high - every link direction, both global comparisons and the release are
/// literal.
pub unsafe fn fn_8015fe50(node: usize, free: i32) -> usize {
    if node != 0 {
        let previous = read_word(node, 0x04) as usize;
        let next = read_word(node, 0x08) as usize;

        if previous != 0 {
            write_word(previous, 0x08, next as u32);
        }
        if next != 0 {
            write_word(next, 0x04, previous as u32);
        }
        if read_global_word(W90_G_LIST_HEAD_5310) as usize == node {
            write_global_word(W90_G_LIST_HEAD_5310, next as u32);
        }
        if read_global_word(W90_G_LIST_TAIL_530C) as usize == node {
            write_global_word(W90_G_LIST_TAIL_530C, previous as u32);
        }

        write_word(node, 0x08, 0);
        write_word(node, 0x04, 0);

        if free > 0 {
            let _ = platform::call1(FN_802B9280, node as u32);
        }
    }
    node
}

/// Appends one node to the intrusive list.
///
/// In plain English: install the `0x80328B18` vtable at `+0x0C`, clear the
/// halfword pair and the `+0x08` link, make the node the head when the list is
/// empty, link it after the current tail and publish it as the new tail.
///
/// # Original
/// - Ghidra: `FUN_8015fef0` at `0x8015FEF0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015fef0_FUN_8015fef0.c`
///
/// # Arguments
/// * `node` - node to append; `+0x00`/`+0x02` halfwords, `+0x04` previous link,
///   `+0x08` next link, `+0x0C` vtable.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Five node writes, one tail-link write and up to two global writes.
///
/// # Safety
/// `node` must be a writable block of at least `0x10` bytes.
///
/// # Confidence
/// high - the head/tail order, the two links and the vtable are literal and match
/// the unlink in [`fn_8015fe50`].
pub unsafe fn fn_8015fef0(node: usize) {
    write_word(node, 0x0C, W90_VTABLE_80328B18);
    write_half(node, 0x00, 0);
    write_half(node, 0x02, 0);
    write_word(node, 0x08, 0);

    if read_global_word(W90_G_LIST_HEAD_5310) == 0 {
        write_global_word(W90_G_LIST_HEAD_5310, node as u32);
    }

    let tail = read_global_word(W90_G_LIST_TAIL_530C) as usize;
    write_word(node, 0x04, tail as u32);
    if tail != 0 {
        write_word(tail, 0x08, node as u32);
    }
    write_global_word(W90_G_LIST_TAIL_530C, node as u32);
}

/// Advances one state object's halfword counters.
///
/// In plain English: while the `+0x00` halfword is zero, run the `+0x0C` holder's
/// `+0x1C` slot and its `+0x28` slot; when `+0x28` returns zero the function
/// returns `0`, otherwise the `+0x02` counter is incremented unless it is `-1`.
/// Finally the `+0x00` counter is incremented unless it is `-1`, and the function
/// returns `1`.
///
/// # Original
/// - Ghidra: `FUN_8015ff40` at `0x8015FF40` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ff40_FUN_8015ff40.c`
///
/// # Arguments
/// * `state` - state object; `+0x00`/`+0x02` are `i16` counters and `+0x0C` is the
///   holder pointer whose word `+0x00` is its vtable.
///
/// # Returns
/// `1` when the object stays live, `0` when the holder's `+0x28` slot says the
/// state is finished.
///
/// # Side effects
/// Two virtual calls and up to two halfword increments.
///
/// # Safety
/// `state` and `state + 0x0C`'s target must be live objects with the cited
/// vtables.
///
/// # Confidence
/// medium - the counters, the `-1` sentinels, both slots and the return values are
/// literal; the receiver of the `+0x1C` call is dropped by the export and is
/// `// inferred:` to be the holder.
pub unsafe fn fn_8015ff40(state: usize) -> u32 {
    if read_half(state, 0x00) == 0 {
        let holder = read_word(state, 0x0C) as usize;
        let vtable = read_word(holder, 0x00) as usize;

        let prepare = read_word(vtable, 0x1C) as usize;
        let _ = platform::call1(prepare, holder as u32);

        let advance = read_word(vtable, 0x28) as usize;
        if platform::call1(advance, state as u32) == 0 {
            return 0;
        }

        let phase = read_half(state, 0x02);
        if phase != 0xFFFF {
            write_half(state, 0x02, phase.wrapping_add(1));
        }
    }

    let counter = read_half(state, 0x00);
    if counter != 0xFFFF {
        write_half(state, 0x00, counter.wrapping_add(1));
    }
    1
}

// ---------------------------------------------------------------------------
// Wave 93 - 0x800D0340..0x800D8EA0: player-pad effects and course markers
// ---------------------------------------------------------------------------

/// `r13 - 0x56B0`: head of the payload list drained by [`fn_800d0340`].
const W93_G_LIST_56B0: usize = SDA_BASE - 0x56B0;

/// `0x80160F60(list, payload)`: unlinks one payload from the list at the first
/// argument. Its second operand is the payload word read from node `+0x08`.
const W93_FN_80160F60: usize = 0x8016_0F60;

/// `0x8016A170(object, source)`: copies a transform into a layout node.
const W93_FN_8016A170: usize = 0x8016_A170;

/// `0x8016A1A0(object, flag)`: marks a layout node's transform dirty.
const W93_FN_8016A1A0: usize = 0x8016_A1A0;

/// `0x801C0BD0(x, y, z, out)`: builds a translation matrix from three `f64`.
const W93_FN_801C0BD0: usize = 0x801C_0BD0;

/// `0x801C0540(dst, lhs, out)`: concatenates two 0x30-byte matrices.
const W93_FN_801C0540: usize = 0x801C_0540;

/// `0x8016F130(matrix, out)`: decomposes a matrix into a 3-float vector.
const W93_FN_8016F130: usize = 0x8016_F130;

/// `0x8023A0B0(owner, name)`: resolves an animation name to a handle.
const W93_FN_8023A0B0: usize = 0x8023_A0B0;

/// `0x80168400(object)`: builds the lookup key used by the archive queries in
/// [`fn_800d4ad0`] and [`fn_800df990`].
const W93_FN_80168400: usize = 0x8016_8400;

/// `0x80164F00(key, name)`: archive lookup returning `-1` when absent.
const W93_FN_80164F00: usize = 0x8016_4F00;

/// `0x800DFA40(owner, interface, index, extra)`: one replay spawn.
const W93_FN_800DFA40: usize = 0x800D_FA40;

/// `0x800DFF30(context, index, value, mode)`: replay store called by
/// [`fn_800dfe60`].
const W93_FN_800DFF30: usize = 0x800D_FF30;

/// `0x802418B0(target, out3)`: reads a 3-float position.
const W93_FN_802418B0: usize = 0x8024_18B0;

/// `0x80241BE0(target, in3)`: writes a 3-float position.
const W93_FN_80241BE0: usize = 0x8024_1BE0;

/// `0x80241AA0(target, transform)`: writes a rotation pair.
const W93_FN_80241AA0: usize = 0x8024_1AA0;

/// `0x80241A00(target, transform)`: writes a translation pair.
const W93_FN_80241A00: usize = 0x8024_1A00;

/// `0x8025C660(out, owner, index)`: builds one archive entry view.
const W93_FN_8025C660: usize = 0x8025_C660;

/// `0x8025C420(entry, 0)`: first lookup key of an entry view.
const W93_FN_8025C420: usize = 0x8025_C420;

/// `0x8023CCC0(key, name)`: name comparison returning non-zero on a match.
const W93_FN_8023CCC0: usize = 0x8023_CCC0;

/// `0x8025C5B0(entry, 0)`: second lookup key of an entry view.
const W93_FN_8025C5B0: usize = 0x8025_C5B0;

/// `0x8023C760(key, 1, out_a, out_b, out_c)`: splits a lookup key.
const W93_FN_8023C760: usize = 0x8023_C760;

/// `0x8023C830(key, 1, a, name, c)`: stores a matched part into a name entry.
const W93_FN_8023C830: usize = 0x8023_C830;

/// `0x8023CA20(key, 0)`: releases one lookup key.
const W93_FN_8023CA20: usize = 0x8023_CA20;

/// `0x8023B8C0(key)`: entry count behind the archive lookup key.
const W93_FN_8023B8C0: usize = 0x8023_B8C0;

/// `0x80069020(object)`: base constructor of the course-marker object.
const W93_FN_80069020: usize = 0x8006_9020;

/// `0x8016A5E0(object)`: animated-model constructor at `+0xA4`.
const W93_FN_8016A5E0: usize = 0x8016_A5E0;

/// `0x80069530(slot)`: resolves one player's controller object.
const W93_FN_80069530: usize = 0x8006_9530;

/// `0x80168080(object)`: first half of the two-word transform read.
const W93_FN_80168080: usize = 0x8016_8080;

/// `0x801680C0(object)`: second half of the two-word transform read.
const W93_FN_801680C0: usize = 0x8016_80C0;

/// `0x800D69B0(a, b, c, owner, mode)`: the callee of [`fn_800d6940`].
const W93_FN_800D69B0: usize = 0x800D_69B0;

/// `0x8015F7C0(state, name)`: fills a descriptor's name and vtable slot.
const W93_FN_8015F7C0: usize = 0x8015_F7C0;

/// `0x800D7FC0(owner, free)`: handler registered by [`fn_800d7c80`].
const W93_FN_800D7FC0: usize = 0x800D_7FC0;

/// `r13 - 0x53E8`: replay/save slot queried by [`fn_800ddc50`].
const W93_G_SAVE_SLOT_53E8: usize = SDA_BASE - 0x53E8;

/// `0x80109340()`: current scene object queried by [`fn_800ddc50`].
const W93_FN_80109340: usize = 0x8010_9340;

/// `0x80107850(slot)`: replay availability query used by [`fn_800ddc50`].
const W93_FN_80107850: usize = 0x8010_7850;

/// `0x800B3B50(button)`: pad-button state query used by [`fn_800ddc50`].
const W93_FN_800B3B50: usize = 0x800B_3B50;

/// `0x8008F140(object)`: constructor of the embedded object at `+0x40`.
const W93_FN_8008F140: usize = 0x8008_F140;

/// `0x8023B200(handle, partner)`: lookup used by [`fn_800dfe60`].
const W93_FN_8023B200: usize = 0x8023_B200;

/// `0x800D69B0`'s type tag passed as `&DAT_802F86C8` / `&DAT_802F8FF0`.
const W93_TAG_802F86C8: usize = 0x802F_86C8;
/// Second interface tag printed by the replay helpers.
const W93_TAG_802F8FF0: usize = 0x802F_8FF0;

/// `0x80318D38`: 0x14-byte effect records used by [`fn_800d8850`].
const W93_TABLE_80318D38: usize = 0x8031_8D38;
/// `0x80318D58`: 0x14-byte effect records used by the same function.
const W93_TABLE_80318D58: usize = 0x8031_8D58;
/// `0x80318C78`: 0x14-byte effect records used by [`fn_800d8bb0`].
const W93_TABLE_80318C78: usize = 0x8031_8C78;
/// `0x80318CD8`: 0x14-byte effect records used by [`fn_800d8e40`].
const W93_TABLE_80318CD8: usize = 0x8031_8CD8;
/// `0x80377C88`: per-slot view handles indexed by the slot's `+0x04` id.
const W93_TABLE_SLOT_VIEWS: usize = 0x8037_7C88;

/// `r2 - 0x6470`: `f32` scale applied to the second word of an effect record.
const W93_F32_SDA2_6470: usize = SDA2_BASE - 0x6470;
/// `r2 - 0x646C`: `f32` zero/identity constant compared in [`fn_800d8ea0`].
const W93_F32_SDA2_646C: usize = SDA2_BASE - 0x646C;
/// `r2 - 0x6468`: `f32` per-frame damping factor.
const W93_F32_SDA2_6468: usize = SDA2_BASE - 0x6468;
/// `r2 - 0x65A4`: `f32` direction scale of the odd marker rows.
const W93_F32_SDA2_65A4: usize = SDA2_BASE - 0x65A4;
/// `r2 - 0x65A0`: `f32` direction scale, and the marker speed divisor.
const W93_F32_SDA2_65A0: usize = SDA2_BASE - 0x65A0;
/// `r2 - 0x659C`: `f32` stored at marker `+0x4C`.
const W93_F32_SDA2_659C: usize = SDA2_BASE - 0x659C;
/// `r2 - 0x6598`: `f32` curvature factor of the marker.
const W93_F32_SDA2_6598: usize = SDA2_BASE - 0x6598;
/// `r2 - 0x6588`: `f32` acceleration factor of [`fn_800d74c0`].
const W93_F32_SDA2_6588: usize = SDA2_BASE - 0x6588;
/// `r2 - 0x6648`: `f32` y axis of the two 0x800D36E0 translation matrices.
const W93_F32_SDA2_6648: usize = SDA2_BASE - 0x6648;
/// `r2 - 0x6408` / `-0x6404` / `-0x6400`: the three-word default rotation
/// copied by [`fn_800dd950`] and [`fn_800de020`].
const W93_F32_SDA2_6408: usize = SDA2_BASE - 0x6408;
const W93_F32_SDA2_6404: usize = SDA2_BASE - 0x6404;
const W93_F32_SDA2_6400: usize = SDA2_BASE - 0x6400;

/// `r13 - 0x5986`: global byte whose bit 3 freezes the effect timers.
const W93_G_TIMER_FREEZE: usize = SDA_BASE - 0x5986;

/// `0x803716F0`: first lazily filled direction-handle table.
const W93_TABLE_803716F0: usize = 0x8037_16F0;
/// `0x80371700`: second lazily filled direction-handle table.
const W93_TABLE_80371700: usize = 0x8037_1700;
/// `r13 - 0x5688` / `-0x5687`: guards for those two tables.
const W93_G_DIR_TABLE_A: usize = SDA_BASE - 0x5688;
const W93_G_DIR_TABLE_B: usize = SDA_BASE - 0x5687;
/// `0x80355EF8`: per-player marker interfaces.
const W93_TABLE_80355EF8: usize = 0x8035_5EF8;

/// An 8-byte-aligned 0x30-byte matrix scratch block.
#[repr(C, align(8))]
struct W93Matrix48([u8; 0x30]);

/// An 8-byte-aligned 0x34-byte scratch block (`auStack_38` in the export).
#[repr(C, align(8))]
struct W93Matrix52([u8; 0x34]);

/// An 8-byte-aligned 0x28-byte scratch block (`auStack_38` of `0x800D3EF0`).
#[repr(C, align(8))]
struct W93Buffer40([u8; 0x28]);

#[inline]
unsafe fn call_f64_f64_f64_ptr(addr: usize, a: f64, b: f64, c: f64, out: u32) {
    let f: extern "C" fn(f64, f64, f64, u32) = core::mem::transmute(addr);
    f(a, b, c, out);
}

#[inline]
unsafe fn call_f64_u32_f32_u32(addr: usize, a: f64, b: u32, c: f32, d: u32) {
    let f: extern "C" fn(f64, u32, f32, u32) = core::mem::transmute(addr);
    f(a, b, c, d);
}

/// Calls the virtual method `slot` with two integer arguments.
#[inline]
unsafe fn w93_vcall2(this_arg: usize, slot: usize, a: u32, b: u32) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call3(entry, this_arg as u32, a, b)
}

// ---------------------------------------------------------------------------
// 0x800D02E0 - deleting destructor of the payload-list owner
// ---------------------------------------------------------------------------

/// `0x80429FC8` (`r13 - 0x56B8`) - word cleared by [`fn_800d02e0`]; the
/// payload-list head [`W93_G_LIST_56B0`] follows it at `r13 - 0x56B0`.
const W284_LIST_OWNER_56B8: usize = SDA_BASE - 0x56B8;

/// Drains the payload list and clears the owner word, then frees `this`.
///
/// In plain English: when `this` is non-null, run the list drain
/// [`fn_800d0340`], clear `0x80429FC8`, and free `this` when `delete_flag` is
/// positive. The address is the deleting destructor that pairs with the
/// wave-93 list band; it calls the drain *before* clearing the owner word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x800D02E0`.
/// - Extent: `0x800D02E0..0x800D033C` (92 bytes, 23 words), ending at the
///   `blr` at `0x800D0338`; one zero word at `0x800D033C` precedes the drain
///   at `0x800D0340`.
/// - Reach: no `b`/`bl` and no data word in the main image; `d_basesNP` calls
///   it from `0x807B7AA8`.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,r3,0`;
///   saves `r31`/`r30`; `mr r31,r4` / `mr r30,r3`; `41820020 bc 12,2`
///   (`beq`, null `this` skips everything); `bl 0x800D0340`;
///   `2C1F0000 cmpi cr0,r31,0`; `li r0,0`; `900DA948 stw r0,-0x56B8(r13)`;
///   `4081000C bc 4,1` (`bf cr0.gt`, free only when the flag is `> 0`);
///   `mr r3,r30` / `bl 0x802B9280`; `mr r3,r30`; epilogue.
/// - Evidence: `work/wave284/e_dis.txt`, `work/wave284/e_refs.txt`.
///
/// # Arguments
/// - `this`: list-owner object (`r3`).
/// - `delete_flag`: positive performs the final free (`r4`).
///
/// # Returns
/// `this`, unchanged, even when it is null.
///
/// # Side effects
/// One list drain, one owner-word clear and, when the flag is positive, one
/// free.
///
/// # Safety
/// `this` must be a live allocation when `delete_flag > 0`; the payload list
/// must satisfy [`fn_800d0340`]'s contract.
///
/// # Confidence
/// high - the null guard, the call order, the SDA store and the `> 0` test
/// (`bc 4,1` on `cr0.gt`) are literal.
pub unsafe fn fn_800d02e0(this: usize, delete_flag: i32) -> usize {
    if this != 0 {
        fn_800d0340();
        platform::write_u32(W284_LIST_OWNER_56B8, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Drains the payload list headed at [`W93_G_LIST_56B0`].
///
/// In plain English: walk the list through the `+0x04` next link; for every
/// node that carries a non-null `+0x08` payload, unlink the payload from the
/// same list and free it.
///
/// # Original
/// - Ghidra: `FUN_800d0340` at `0x800D0340` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0340_FUN_800d0340.c`
///
/// # Arguments
/// None. The list head and all nodes come from the global.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Unlinks and frees every payload; the loop then ends on the null next link.
///
/// # Safety
/// Every node `+0x04` and `+0x08` word must be a readable pointer, and every
/// payload must be a live allocation.
///
/// # Confidence
/// medium - the walk, the unlink call and the free are literal; the `+0x04`
/// next / `+0x08` payload roles are `// inferred:` from the order the two words
/// are read.
pub unsafe fn fn_800d0340() {
    let mut node = read_global_word(W93_G_LIST_56B0) as usize;
    while node != 0 {
        // observed: iVar2 = node[2]; iVar1 = node[1]; then unlink + free iVar2.
        let payload = read_word(node, 0x08) as usize;
        node = read_word(node, 0x04) as usize;
        if payload != 0 {
            let _ = platform::call2(W93_FN_80160F60, W93_G_LIST_56B0 as u32, payload as u32);
            let _ = platform::call1(FN_802B9280, payload as u32);
        }
    }
}

/// Rebuilds the two layout nodes of one course marker.
///
/// In plain English: after the shared marker update, reset `+0x208`, copy the
/// `+0xD8` transform into the layout node, build a translation from the three
/// int words at `+0x138..+0x140`, concatenate it and store the result at
/// `+0x4A8 + view * 0x80`; then repeat the same sequence for the `+0xCC`
/// transform with a y offset taken from the view record at `+0x778`.
///
/// # Original
/// - Ghidra: `FUN_800d36e0` at `0x800D36E0` (328 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d36e0_FUN_800d36e0.c`
///
/// # Arguments
/// * `object` - the 0x800D5330 marker; `+0x208` reset, `+0xD8` and `+0xCC`
///   source transforms, `+0x138..+0x140` translation words, `+0x770` view
///   index and `+0x778` view record array.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls through vtable slot `+0x50`, six layout calls and several
/// matrix concatenations.
///
/// # Safety
/// `object` must be a live marker of at least `0x780` bytes and its vtable slot
/// `+0x50` must accept `(receiver, matrix, word)`.
///
/// # Confidence
/// medium - every call, offset and buffer size is literal; the meaning of the
/// `+0x4E..+0x50` words (read as `int` then cast to `float`) is `// inferred:`
/// from the translation-matrix helper, and slot `+0x50`'s receiver is printed
/// as the object itself.
pub unsafe fn fn_800d36e0(object: usize) {
    // observed: FUN_800d4cd0() with no printed operand; // inferred: object.
    let _ = platform::call1(W89_FN_800D4CD0, object as u32);
    write_word(object, 0x208, 0);

    let mut src = W93Matrix52([0u8; 0x34]);
    let mut translation = W93Matrix48([0u8; 0x30]);
    let mut second = W93Matrix48([0u8; 0x30]);
    let mut third = W93Matrix48([0u8; 0x30]);

    let receiver = vcall0(object, 0x28);
    let _ = platform::call2(
        W93_FN_8016A170,
        receiver,
        (object + 0xD8) as u32,
    );
    let receiver = vcall0(object, 0x28);
    let _ = platform::call2(W93_FN_8016A1A0, receiver, 0);

    let view = read_word(object, 0x770);
    let view_record = read_word(object, 0x778) as usize;
    let _ = w93_vcall2(
        object,
        0x50,
        src.0.as_mut_ptr() as u32,
        read_word(object, 0x77C),
    );

    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        read_word(object, 0x138) as i32 as f32 as f64,
        read_word(object, 0x13C) as i32 as f32 as f64,
        read_word(object, 0x140) as i32 as f32 as f64,
        translation.0.as_mut_ptr() as u32,
    );
    let _ = platform::call3(
        W93_FN_801C0540,
        src.0.as_mut_ptr() as u32,
        translation.0.as_ptr() as u32,
        src.0.as_mut_ptr() as u32,
    );

    let view_node = object + (view as usize * 0x20 + 0x12A) * 4;
    let _ = platform::call2(W93_FN_8016A170, view_node as u32, src.0.as_ptr() as u32);
    let _ = platform::call2(W93_FN_8016A1A0, view_node as u32, 0);

    let _ = w93_vcall2(
        object,
        0x50,
        second.0.as_mut_ptr() as u32,
        read_word(object, 0x77C),
    );
    let _ = platform::call2(
        W93_FN_8016F130,
        second.0.as_ptr() as u32,
        (object + 0xCC) as u32,
    );

    // observed: y = -(*(f32 *)(view_record + view * 4 + 0x28)); x = z = SDA2 6648.
    let offset = read_const_f32(view_record + view as usize * 4 + 0x28);
    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        read_const_f32(W93_F32_SDA2_6648) as f64,
        -(offset as f64),
        read_const_f32(W93_F32_SDA2_6648) as f64,
        third.0.as_mut_ptr() as u32,
    );
    let _ = platform::call3(
        W93_FN_801C0540,
        second.0.as_mut_ptr() as u32,
        third.0.as_ptr() as u32,
        second.0.as_mut_ptr() as u32,
    );
    let _ = platform::call2(
        W93_FN_8016F130,
        second.0.as_ptr() as u32,
        (object + 0xC0) as u32,
    );
}

/// Maps an instance id to the animation name of the tanooki/statue marker.
///
/// In plain English: only when `+0x770` is `3`, translate the instance id into
/// one of the `P_R*` animation names, resolve it through `0x8023A0B0` and
/// latch the two `0x800000` "animation changed" bits.
///
/// # Original
/// - Ghidra: `FUN_800d3ce0` at `0x800D3CE0` (520 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3ce0_FUN_800d3ce0.c`
///
/// # Arguments
/// * `object` - marker; `+0x210` is passed to the name resolver and
///   `+0x160/+0x164` receive the latch bits.
/// * `instance` - instance id decoded by the nested comparisons.
/// * `out` - receives the resolved animation handle.
///
/// # Returns
/// `1` when the id mapped to a name, `0` otherwise.
///
/// # Side effects
/// One name resolution, one store through `out`, and two bit latches.
///
/// # Safety
/// `object + 0x210` must be a live name table and `out` must be writable.
///
/// # Confidence
/// high - every comparison, the name address and both bit latches are literal.
/// The id `0x66` resolves through `r13 - 0x79D0`, whose string is not printed,
/// and is kept as an unnamed SDA slot.
pub unsafe fn fn_800d3ce0(object: usize, instance: i32, out: usize) -> u32 {
    if read_word(object, 0x770) != 3 {
        return 0;
    }

    let mut name: Option<u32> = None;
    if instance == 0x67 {
        name = Some(0x8031_7C60); // s_P_RSeat_out
    } else if instance < 0x67 {
        if instance == 0x15 {
            name = Some(0x8031_7C3C); // s_P_Rstoop_start
        } else if instance < 0x15 {
            if instance == 0x13 {
                name = Some(0x8031_7C20); // s_P_Rhip_to_stoop
            } else if instance > 0x12 {
                name = Some(0x8031_7C30); // s_P_Rstoop
            }
        } else if instance == 0x18 {
            name = Some(0x8031_7C4C); // s_P_Rslip_to_stoop
        } else if instance > 0x17 && instance > 0x65 {
            // observed: the export prints `unaff_r13 + -0x79d0`; the string at
            // that SDA slot is not recovered. // unknown: name text.
            name = Some((SDA_BASE - 0x79D0) as u32);
        }
    } else if instance == 0x76 {
        name = Some(0x8031_7CA4); // s_P_Rsjump
    } else if instance < 0x76 {
        if instance == 0x6A {
            name = Some(0x8031_7C90); // s_P_RSeat_successB
        } else if instance < 0x6A {
            if instance < 0x69 {
                name = Some(0x8031_7C70); // s_P_RSeat_success
            } else {
                name = Some(0x8031_7C80); // s_P_RSeat_fail
            }
        }
    } else if instance == 0x78 {
        name = Some(0x8031_7CBC); // s_P_RSjumped
    } else if instance < 0x78 {
        name = Some(0x8031_7CB0); // s_P_RSjump2
    }

    match name {
        Some(name_addr) => {
            let handle = platform::call2(W93_FN_8023A0B0, (object + 0x210) as u32, name_addr);
            write_word(out, 0, handle);
            let flags = read_word(object, 0x160);
            write_word(object, 0x160, flags | 0x0080_0000);
            let flags = read_word(object, 0x164);
            write_word(object, 0x164, flags | 0x0080_0000);
            1
        }
        None => 0,
    }
}

/// Maps a low-state id to the `P_L_low_walk*` or event-table animation name.
///
/// In plain English: first ask the object's vtable slot `+0x58` for a name;
/// when it answers, resolve that name through `0x8023A0B0` and return `1`.
/// When it does not, only state `+0x1DC == 2` (`0x82`/`0x83`) or state
/// `+0x1DC == 3` with a non-null `P_wait` table entry may map the id, and those
/// paths latch the `0x1000000` / `0x800000` bits.
///
/// # Original
/// - Ghidra: `FUN_800d3ef0` at `0x800D3EF0` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3ef0_FUN_800d3ef0.c`
///
/// # Arguments
/// * `object` - marker; `+0x210` name table, `+0x770` state word (printed as
///   `param_1[0x1DC]`), `+0x160/+0x164` latch words.
/// * `instance` - instance id.
/// * `out` - receives the resolved handle.
/// * `skip_primary` - when `0`, the primary latch word is also set.
///
/// # Returns
/// `1` when a name was resolved, `0` otherwise.
///
/// # Side effects
/// One or two name resolutions and up to two bit latches.
///
/// # Safety
/// `object` must be a live marker and `out` writable.
///
/// # Confidence
/// medium - the branches, the names and the latches are literal, but the
/// `P_wait` table lookup's second argument to `0x8023A0B0` is dropped by the
/// export and is `// inferred:` to be the table entry itself.
pub unsafe fn fn_800d3ef0(
    object: usize,
    instance: i32,
    out: usize,
    skip_primary: i32,
) -> u32 {
    let mut buffer = W93Buffer40([0u8; 0x28]);
    // observed: iVar2 = (**(code **)(*param_1 + 0x58))(param_1, param_2, auStack_38).
    let matched = w93_vcall2(
        object,
        0x58,
        instance as u32,
        buffer.0.as_mut_ptr() as u32,
    );

    if matched != 0 {
        let handle = platform::call2(
            W93_FN_8023A0B0,
            (object + 0x210) as u32,
            buffer.0.as_ptr() as u32,
        );
        write_word(out, 0, handle);
        return 1;
    }

    let state = read_word(object, 0x770) as i32;
    if state == 2 {
        let name = if instance == 0x82 {
            Some(0x8031_7CC8) // s_PL_low_walk_start
        } else if instance == 0x83 {
            Some(0x8031_7CDC) // s_PL_low_walk
        } else {
            None
        };
        if let Some(name_addr) = name {
            let handle = platform::call2(W93_FN_8023A0B0, (object + 0x210) as u32, name_addr);
            write_word(out, 0, handle);
            if skip_primary == 0 {
                let flags = read_word(object, 0x160);
                write_word(object, 0x160, flags | 0x0100_0000);
            }
            let flags = read_word(object, 0x164);
            write_word(object, 0x164, flags | 0x0100_0000);
            return 1;
        }
    } else if state == 3 {
        // observed: (&PTR_s_P_wait_802f135c)[instance * 9] must be non-null.
        let entry = read_global_word(0x802F_135C + instance as usize * 0x24);
        if entry != 0 {
            let handle = platform::call2(W93_FN_8023A0B0, (object + 0x210) as u32, entry);
            write_word(out, 0, handle);
            if skip_primary == 0 {
                let flags = read_word(object, 0x160);
                write_word(object, 0x160, flags | 0x0080_0000);
            }
            let flags = read_word(object, 0x164);
            write_word(object, 0x164, flags | 0x0080_0000);
            return 1;
        }
    }
    0
}

/// Applies a named animation to every entry of one archive.
///
/// In plain English: build a lookup key for `owner`, take its `+0x04`
/// interface, query slot `+0x08` with the local tag, then walk every entry of
/// the key; for each entry whose name matches the requested one, split, chain
/// and commit the name through `0x8023C760/0x8023C830/0x8023CA20`.
///
/// # Original
/// - Ghidra: `FUN_800d4ad0` at `0x800D4AD0` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4ad0_FUN_800d4ad0.c`
///
/// # Arguments
/// * `owner` - object whose `+0x04` interface is queried.
/// * `name` - the animation name compared against every entry.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual query and one name commit per matching entry.
///
/// # Safety
/// `owner + 0x04` must be a live interface and `name` a readable string.
///
/// # Confidence
/// medium - the loop, the four callees and the buffer sizes are literal; the
/// interface tag `&DAT_802F8FF0` and the five staged keys are opaque library
/// values, and the export prints `FUN_80168400` with only one operand.
pub unsafe fn fn_800d4ad0(owner: usize, name: usize) {
    let mut key = [0u32; 1];
    key[0] = platform::call1(W93_FN_80168400, owner as u32);

    let mut tag = W93_TAG_802F8FF0 as u32;
    let mut interface = read_word(owner, 0x04) as usize;
    let mut valid = false;
    if interface != 0 {
        let answer = vcall1(interface, 0x08, (&mut tag as *mut u32) as u32);
        valid = answer != 0;
    }
    if !valid {
        interface = 0;
    }
    if interface == 0 {
        return;
    }

    let mut entry = [0u8; 0x40];
    let mut part_a = 0u32;
    let mut part_b = 0u32;
    let mut part_c = 0u32;

    let mut index = 0u32;
    // observed: the count is recomputed by the loop condition on every pass.
    while index < platform::call1(W93_FN_8023B8C0, key.as_ptr() as u32) {
        let _ = platform::call3(
            W93_FN_8025C660,
            entry.as_mut_ptr() as u32,
            interface as u32,
            index,
        );
        let lookup = platform::call2(W93_FN_8025C420, entry.as_mut_ptr() as u32, 0);
        let mut lookup_word = lookup;
        if platform::call2(W93_FN_8023CCC0, (&mut lookup_word as *mut u32) as u32, name as u32) != 0
        {
            let second = platform::call2(W93_FN_8025C5B0, entry.as_mut_ptr() as u32, 0);
            let mut second_word = second;
            let _ = platform::call5(
                W93_FN_8023C760,
                (&mut second_word as *mut u32) as u32,
                1,
                (&mut part_a as *mut u32) as u32,
                (&mut part_b as *mut u32) as u32,
                (&mut part_c as *mut u32) as u32,
            );
            let _ = platform::call5(
                W93_FN_8023C830,
                (&mut second_word as *mut u32) as u32,
                1,
                part_a,
                name as u32,
                part_c,
            );
            let _ = platform::call2(
                W93_FN_8023CA20,
                (&mut second_word as *mut u32) as u32,
                0,
            );
        }
        index = index.wrapping_add(1);
    }
}

/// Applies one model entry's transform to a target object.
///
/// In plain English: read the entry mode at `+0x0C`; mode `1` reads the
/// target's position, scales it by `owner + 0x788` and writes it back; every
/// other non-zero mode copies the entry's `+0x38` transform and, for modes
/// `0x0B`/`0x0E` in marker mode `3`, also writes the `+0x2C`/`+0x20` parts.
///
/// # Original
/// - Ghidra: `FUN_800d4d80` at `0x800D4D80` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4d80_FUN_800d4d80.c`
///
/// # Arguments
/// * `owner` - object holding the `+0x788` scale, `+0x770` mode and `+0x164`
///   flag word.
/// * `target` - object passed to the position helpers.
/// * `slot` - pointer to the entry pointer; entry `+0x0C` is the mode.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to four position writes through `0x802418B0/0x80241BE0/0x80241AA0/
/// 0x80241A00`.
///
/// # Safety
/// `slot` must be readable, and when the entry is non-null its `+0x38` block
/// must be a valid transform.
///
/// # Confidence
/// medium - every guard, mode and call is literal; the entry's `+0x0C` role as
/// a mode and the `+0x788` role as a scale are `// inferred:` from the two
/// call shapes.
pub unsafe fn fn_800d4d80(owner: usize, target: usize, slot: usize) {
    let entry = read_word(slot, 0) as usize;
    let mode = if entry == 0 {
        0
    } else {
        read_word(entry, 0x0C) as i32
    };
    if mode == 0 {
        return;
    }

    if mode == 1 {
        let mut position = [0f32; 3];
        let _ = platform::call2(
            W93_FN_802418B0,
            target as u32,
            position.as_mut_ptr() as u32,
        );
        let scale = read_float(owner, 0x788);
        position[0] *= scale;
        position[1] *= scale;
        position[2] *= scale;
        let _ = platform::call2(
            W93_FN_80241BE0,
            target as u32,
            position.as_ptr() as u32,
        );
    } else {
        let _ = platform::call2(W93_FN_80241BE0, target as u32, (entry + 0x38) as u32);
        if mode == 0x0B || mode == 0x0E {
            if read_word(owner, 0x770) == 3 && (read_word(owner, 0x164) & 0x0080_0000) == 0 {
                let entry = read_word(slot, 0) as usize;
                let _ = platform::call2(W93_FN_80241AA0, target as u32, (entry + 0x2C) as u32);
                let entry = read_word(slot, 0) as usize;
                let _ = platform::call2(W93_FN_80241A00, target as u32, (entry + 0x20) as u32);
            }
        }
    }
}

/// `r2 - 0x6608` / `-0x6604`: the two `f32` defaults of [`fn_800d5330`].
const W93_F32_SDA2_6608: usize = SDA2_BASE - 0x6608;
const W93_F32_SDA2_6604: usize = SDA2_BASE - 0x6604;

/// `0x800260C0`: destructor callback of the `0x38`-byte marker pool.
const W93_FN_800260C0: usize = 0x8002_60C0;
/// `0x8000FBF0`: destructor callback of the `0xC`-byte marker pool.
const W93_FN_8000FBF0: usize = 0x8000_FBF0;

/// `0x800B2EE0(5)`: selects one of the five easing rows.
const W93_FN_800B2EE0: usize = 0x800B_2EE0;

/// `0x8015F140(a, b, c, d, out)`: four-`f64` interpolation used by
/// [`fn_800d8ea0`].
const W93_FN_8015F140: usize = 0x8015_F140;

/// `0x802F2E58` / `0x802F2E5C`: parallel easing tables indexed by
/// `0x800B2EE0`'s result.
const W93_TABLE_EASE_X: usize = 0x802F_2E58;
const W93_TABLE_EASE_Y: usize = 0x802F_2E5C;

/// `0x802F2C48`: per-row marker speed table indexed by `param_6`.
const W93_TABLE_MARKER_SPEED: usize = 0x802F_2C48;

/// `0x80318BD8`: vtable installed in every `dPosGoAndComeExeFrm_c` state.
const W93_VTABLE_80318BD8: usize = 0x8031_8BD8;
/// `0x803194F4`: vtable installed in every `dReplayPlay_c` state.
const W93_VTABLE_803194F4: usize = 0x8031_94F4;
/// `0x800DE850`: handler registered by [`fn_800de510`].
const W93_FN_800DE850: usize = 0x800D_E850;

#[inline]
unsafe fn call_u64_ret(addr: usize, a: u32) -> u64 {
    let f: extern "C" fn(u32) -> u64 = core::mem::transmute(addr);
    f(a)
}

#[inline]
unsafe fn call_u64_u64_u64_u32_u32(
    addr: usize,
    a: u64,
    b: u64,
    c: u64,
    d: u32,
    e: u32,
) {
    let f: extern "C" fn(u64, u64, u64, u32, u32) = core::mem::transmute(addr);
    f(a, b, c, d, e);
}

#[inline]
unsafe fn call_f64x4_ptr(addr: usize, a: f64, b: f64, c: f64, d: f64, out: u32) -> f64 {
    let f: extern "C" fn(f64, f64, f64, f64, u32) -> f64 = core::mem::transmute(addr);
    f(a, b, c, d, out)
}

/// Constructs the `dPosGoAndCome` marker object.
///
/// In plain English: install the two base vtables, build the embedded
/// `0x38`-byte and `0xC`-byte pools, the `0x803033F0` animated model, then
/// initialise the two rotation triples from SDA2 defaults and clear the id,
/// frame, latch and counter words.
///
/// # Original
/// - Ghidra: `FUN_800d5330` at `0x800D5330` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5330_FUN_800d5330.c`
///
/// # Arguments
/// * `object` - storage for the marker.
/// * `variant` - byte stored at `+0x151`; the only constructor argument the
///   export keeps.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Runs five constructors and writes roughly forty fields.
///
/// # Safety
/// `object` must be at least `0x20C` writable bytes and both pools must accept
/// their constructor/destructor addresses.
///
/// # Confidence
/// medium - every store and call is literal; the pool element roles and the
/// `+0x154..+0x15C` `-1` sentinels are `// inferred:` from the sibling
/// `0x800D36E0`/`0x800D3CE0` bodies.
pub unsafe fn fn_800d5330(object: usize, variant: u8) -> usize {
    write_word(object, 0x00, 0x8031_89A0);
    let _ = platform::call1(W93_FN_80069020, (object + 0x04) as u32);
    write_word(object, 0x20, 0);
    write_word(object, 0x24, 0);
    let _ = platform::call5(
        FN_802DC9A0,
        (object + 0x28) as u32,
        0x800D_54A0,
        W93_FN_800260C0 as u32,
        0x38,
        2,
    );
    write_word(object, 0x9C, 0);
    write_word(object, 0x98, 0x8032_95D8);
    write_word(object, 0xA0, 0);
    let _ = platform::call1(W93_FN_8016A5E0, (object + 0xA4) as u32);

    let default_a = read_const_f32(W93_F32_SDA2_6608);
    let default_b = read_const_f32(W93_F32_SDA2_6604);
    write_word(object, 0x98, 0x8030_33F0);
    write_word(object, 0xC0, default_a.to_bits());
    write_word(object, 0xC4, default_a.to_bits());
    write_word(object, 0xC8, default_a.to_bits());
    write_word(object, 0xCC, default_a.to_bits());
    write_word(object, 0xD0, default_a.to_bits());
    write_word(object, 0xD4, default_a.to_bits());
    write_word(object, 0x138, default_a.to_bits());
    write_word(object, 0x13C, default_a.to_bits());
    write_word(object, 0x140, default_a.to_bits());
    write_word(object, 0x144, default_b.to_bits());
    write_word(object, 0x148, default_b.to_bits());
    write_word(object, 0x14C, default_b.to_bits());
    write_byte(object, 0x150, 0);
    write_byte(object, 0x151, variant);
    write_byte(object, 0x152, 0);
    write_byte(object, 0x153, 0xFF);
    write_word(object, 0x154, 0xFFFF_FFFF);
    write_word(object, 0x158, 0xFFFF_FFFF);
    write_word(object, 0x15C, 0xFFFF_FFFF);
    write_word(object, 0x168, 0);
    write_word(object, 0x16C, 0);
    write_word(object, 0x170, 0);
    write_word(object, 0x174, 0);
    write_byte(object, 0x178, 0);
    write_word(object, 0x17C, 0);
    write_word(object, 0x180, 0);
    write_word(object, 0x184, default_a.to_bits());
    write_word(object, 0x188, default_a.to_bits());
    write_word(object, 0x18C, 0);
    let _ = platform::call5(
        FN_802DC9A0,
        (object + 0x190) as u32,
        0x8001_5E30,
        W93_FN_8000FBF0 as u32,
        0x0C,
        6,
    );
    write_half(object, 0x1FC, 0);
    write_half(object, 0x1FE, 0);
    write_half(object, 0x200, 0);
    write_word(object, 0x204, 0);
    write_word(object, 0x208, 0);
    write_word(object, 0x160, 0);
    write_word(object, 0x164, 0);
    object
}

/// Forwards a two-word transform read into [`W93_FN_800D69B0`].
///
/// In plain English: read the object's `+0x28` transform twice through the two
/// halves of the accessor pair and forward the pair, the caller's first
/// argument, the object and the object's `+0x154` mode word.
///
/// # Original
/// - Ghidra: `FUN_800d6940` at `0x800D6940` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6940_FUN_800d6940.c`
///
/// # Arguments
/// * `first` - 64-bit value forwarded unchanged.
/// * `object` - object whose `+0x28` transform and `+0x154` mode are read.
///
/// # Returns
/// Nothing; the callee returns nothing.
///
/// # Side effects
/// Only the callee's writes.
///
/// # Safety
/// `object + 0x28` must be a valid transform and `0x800D69B0` must accept the
/// five-argument ABI printed by the export.
///
/// # Confidence
/// medium - the two accessor calls and the five forwarded operands are literal;
/// the export prints both accessors with no argument and both results as
/// `undefined8`, so the 64-bit register-pair ABI is `// inferred:`.
pub unsafe fn fn_800d6940(first: u64, object: usize) {
    // observed: FUN_80168080(param_2 + 0x28) and FUN_801680c0(param_2 + 0x28);
    // the first argument is dropped by the export and inferred to be the same
    // address.
    let a = call_u64_ret(W93_FN_80168080, (object + 0x28) as u32);
    let b = call_u64_ret(W93_FN_801680C0, (object + 0x28) as u32);
    call_u64_u64_u64_u32_u32(
        W93_FN_800D69B0,
        b,
        first,
        a,
        object as u32,
        read_word(object, 0x154),
    );
}

/// Turns the current pad state into one of four marker modes.
///
/// In plain English: copy the previous and current button halfwords, read the
/// player's input object, classify the direction bits into mode `0..3`, lazily
/// install the two direction-handle tables, then fire the two "direction
/// pressed" and the "mode changed" vtable calls.
///
/// # Original
/// - Ghidra: `FUN_800d70e0` at `0x800D70E0` (548 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d70e0_FUN_800d70e0.c`
///
/// # Arguments
/// * `object` - marker; `+0x08` slot index, `+0x0C/+0x0E` current buttons,
///   `+0x10/+0x12` previous buttons, `+0x04` mode.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three table initialisations and up to three virtual calls through slot
/// `+0x20` of the `0x80355EF8` interface.
///
/// # Safety
/// `object + 0x08` must be a valid slot index into [`G_INPUT_OBJECTS`] and the
/// `0x80355EF8` entry must be a live interface.
///
/// # Confidence
/// medium - the bit tests, the four modes, the two lazily built tables and the
/// three calls are literal; the roles of `+0x04` (mode) and `+0x08` (slot) are
/// `// inferred:` from how they index the tables.
pub unsafe fn fn_800d70e0(object: usize) {
    let slot = read_word(object, 0x08) as usize;
    let handle = platform::call1(W93_FN_80069530, read_word(object, 0x08));

    write_half(object, 0x10, read_half(object, 0x0C));
    write_half(object, 0x12, read_half(object, 0x0E));

    let table = read_global_word(G_INPUT_OBJECTS) as usize;
    let input = read_word(table, slot * 4 + 4) as usize;
    let pressed = read_word(input, 0x14);
    let held = read_word(input, 0x1C);
    write_half(object, 0x0C, pressed as u16);
    write_half(object, 0x0E, held as u16);

    // observed: only the four matched branches write +0x04.
    if (pressed & 2) != 0 {
        write_word(object, 0x04, 0);
    } else if (pressed & 1) != 0 {
        write_word(object, 0x04, 1);
    } else if (pressed & 8) != 0 {
        write_word(object, 0x04, 2);
    } else if (pressed & 4) != 0 {
        write_word(object, 0x04, 3);
    }
    let mode = read_word(object, 0x04) as usize;

    if read_global_byte(W93_G_DIR_TABLE_A) == 0 {
        write_global_word(W93_TABLE_803716F0, 0x7B0);
        write_global_word(W93_TABLE_803716F0 + 4, 0x7B6);
        write_global_word(W93_TABLE_803716F0 + 8, 0x7B8);
        write_global_word(W93_TABLE_803716F0 + 12, 0x7B2);
        write_global_byte(W93_G_DIR_TABLE_A, 1);
    }

    let current = read_half(object, 0x0E);
    let interface = read_global_word(W93_TABLE_80355EF8 + slot * 4) as usize;
    if (current & 0x200) != 0 {
        let index = mode as usize;
        let _ = w93_vcall2(
            interface,
            0x20,
            read_global_word(W93_TABLE_803716F0 + index * 4),
            handle,
        );
    }

    if read_global_byte(W93_G_DIR_TABLE_B) == 0 {
        write_global_word(W93_TABLE_80371700, 0x7B1);
        write_global_word(W93_TABLE_80371700 + 4, 0x7B7);
        write_global_word(W93_TABLE_80371700 + 8, 0x7B9);
        write_global_word(W93_TABLE_80371700 + 12, 0x7B4);
        write_global_byte(W93_G_DIR_TABLE_B, 1);
    }

    if (current & 0x100) != 0 {
        let index = mode as usize;
        let _ = w93_vcall2(
            interface,
            0x20,
            read_global_word(W93_TABLE_80371700 + index * 4),
            handle,
        );
    }

    if read_word(object, 0x04) == 3 {
        let previous = read_half(object, 0x10);
        let current = read_half(object, 0x0C);
        if (previous & 0x100) != 0 && (current & 0x100) == 0 {
            let _ = w93_vcall2(interface, 0x20, 0x7B5, handle);
        }
    }
}

/// Fills the sweep geometry of the course marker.
///
/// In plain English: convert the row count to a `float` with the
/// `CONCAT44(0x43300000, ...)` unsigned-int idiom, pick the sweep scale from
/// the direction, store the two pane offsets, then derive the four edge values
/// and the speed/period pair from the per-row speed table.
///
/// # Original
/// - Ghidra: `FUN_800d7350` at `0x800D7350` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7350_FUN_800d7350.c`
///
/// # Arguments
/// * `object` - marker; `+0x3C..+0x6C` receive the geometry.
/// * `count` - the row count converted to a float.
/// * `direction` - `1` or `3` selects the mirrored layout.
/// * `offset_a` - column offset multiplied by `0x10`.
/// * `offset_b` - the other column offset multiplied by `0x10`.
/// * `row` - index into the speed table at `0x802F2C48`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eleven words and floats.
///
/// # Safety
/// `object + 0x3C..+0x70` must be writable and `row` must index the speed
/// table.
///
/// # Confidence
/// medium - every store and both mirror orders are literal; the meaning of the
/// five SDA2 floats is `// unknown:` and they are read from their absolute
/// addresses rather than guessed.
pub unsafe fn fn_800d7350(
    object: usize,
    count: i32,
    direction: i32,
    offset_a: i32,
    offset_b: i32,
    row: i32,
) {
    let mirrored = direction == 1 || direction == 3;
    // observed: CONCAT44(0x43300000, count) - 2^52 is the unsigned int -> double
    // conversion idiom.
    let count_f = count as u32 as f32;
    let scale = if mirrored {
        read_const_f32(W93_F32_SDA2_65A4)
    } else {
        read_const_f32(W93_F32_SDA2_65A0)
    };
    let speed = scale * count_f;

    let pane_a = (offset_a + 1) * 0x10;
    let pane_b = (offset_b + 1) * 0x10;
    write_float(object, 0x3C, speed);
    write_float(object, 0x4C, read_const_f32(W93_F32_SDA2_659C));
    if mirrored {
        write_word(object, 0x58, pane_a as u32);
        write_word(object, 0x48, pane_b as u32);
    } else {
        write_word(object, 0x48, pane_a as u32);
        write_word(object, 0x58, pane_b as u32);
    }

    let edge = read_const_f32(W93_TABLE_MARKER_SPEED + row as usize * 4);
    let period = (read_const_f32(W93_F32_SDA2_65A0) / edge) as i32;
    let curvature = edge * edge * read_const_f32(W93_F32_SDA2_6598);
    if mirrored {
        write_float(object, 0x54, edge);
        write_float(object, 0x44, -edge);
        write_float(object, 0x40, -curvature);
        write_float(object, 0x50, curvature);
    } else {
        write_float(object, 0x44, edge);
        write_float(object, 0x40, curvature);
        write_float(object, 0x54, -edge);
        write_float(object, 0x50, -curvature);
    }
    write_word(object, 0x6C, period as u32);
    write_word(object, 0x68, ((count - 1) * period) as u32);
}

/// Advances the marker sweep by one frame and reports whether it wrapped.
///
/// In plain English: when the requested position is still ahead of the current
/// cursor, interpolate the two path values with the quadratic term; otherwise
/// commit the step to the cursor and report `false`. The return value is the
/// same comparison the caller uses to decide whether the marker crossed the
/// limit.
///
/// # Original
/// - Ghidra: `FUN_800d74c0` at `0x800D74C0` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d74c0_FUN_800d74c0.c`
///
/// # Arguments
/// * `state` - marker; `+0x6C` period, `+0x40 + row * 0x10` edge, `+0x60/+0x5C`
///   interpolated outputs and `+0x64` remaining count.
/// * `value` - `f32` path value advanced by the whole-step branch.
/// * `cursor` - `u32` cursor advanced by the same branch.
/// * `limit` - requested cursor limit.
/// * `row` - selects one of the two edge slots.
///
/// # Returns
/// `true` while `limit < cursor + period`.
///
/// # Side effects
/// Writes `+0x60`, `+0x5C`, `+0x64` or the two caller words.
///
/// # Safety
/// `state`, `value` and `cursor` must be readable/writable as described.
///
/// # Confidence
/// high - the arithmetic, both branches and the return comparison are literal;
/// the two SDA2 factors are read from their absolute addresses and the `+0x6C`
/// role as a period is `// inferred:` from [`fn_800d7350`], which writes it.
pub unsafe fn fn_800d74c0(
    state: usize,
    value: usize,
    cursor: usize,
    limit: u32,
    row: i32,
) -> bool {
    let period = read_word(state, 0x6C);
    let next = read_word(cursor, 0).wrapping_add(period);
    let edge = read_float(state, 0x40 + row as usize * 0x10);
    let step = limit.wrapping_sub(read_word(cursor, 0));

    if limit < next {
        let step_f = step as f32;
        write_float(state, 0x60, edge * step_f);
        let base = read_float(value, 0);
        write_word(state, 0x64, period.wrapping_sub(step));
        write_float(
            state,
            0x5C,
            base + read_const_f32(W93_F32_SDA2_6588) * step_f * edge * step_f,
        );
    } else {
        let base = read_float(value, 0);
        let period_f = period as f32;
        write_float(
            value,
            0,
            base + read_const_f32(W93_F32_SDA2_6588) * period_f * edge * period_f,
        );
        write_word(cursor, 0, next);
    }
    limit < next
}

/// Registers four `dPosGoAndComeExeFrm_c::StateID` descriptors.
///
/// In plain English: for each of the A(ir)/M(iddle)/D(own)/W(all) states, name
/// the descriptor, install the `0x80318BD8` vtable, copy the nine handler words
/// and register `0x800D7FC0` with the per-state support object.
///
/// # Original
/// - Ghidra: `FUN_800d7c80` at `0x800D7C80` (824 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d7c80_FUN_800d7c80.c`
/// - Removed: the modelled `0x802DCD74`/`0x802DCDC0` calls. They are the
///   `_savegpr_27`/`_restgpr_27` compiler helpers (register spill/restore
///   only); both results were already discarded.
///
/// # Arguments
/// None. The four descriptors and their support objects are absolute globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills `0x80371720`, `0x80371760`, `0x803717A0` and `0x803717E0` and
/// registers each one through `0x802DC780`.
///
/// # Safety
/// The four descriptor blocks and their support objects must be writable.
///
/// # Confidence
/// medium - the four blocks, their names, the nine copied words and the
/// registration call are literal; the roles of the nine handler words are
/// `// inferred:` from the shared `+0x0C..+0x2C` layout.
pub unsafe fn fn_800d7c80() {
    w93_register_state(
        0x8037_1720,
        0x8031_8B40, // s_dPosGoAndComeExeFrm_c__StateID_A
        W93_FN_800D7FC0 as u32,
        0x8037_1710,
        W93_VTABLE_80318BD8 as u32,
        [
            read_global_word(0x8031_8AC8),
            read_global_word(0x8031_8ACC),
            read_global_word(0x8031_8AD0),
            read_global_word(0x8031_8ABC),
            read_global_word(0x8031_8AC0),
            read_global_word(0x8031_8AC4),
            read_global_word(0x8031_8AB0),
            read_global_word(0x8031_8AB4),
            read_global_word(0x8031_8AB8),
        ],
    );
    w93_register_state(
        0x8037_1760,
        0x8031_8B68, // s_dPosGoAndComeExeFrm_c__StateID_M
        W93_FN_800D7FC0 as u32,
        0x8037_1750,
        W93_VTABLE_80318BD8 as u32,
        [
            read_global_word(0x8031_8AEC),
            read_global_word(0x8031_8AF0),
            read_global_word(0x8031_8AF4),
            read_global_word(0x8031_8AE0),
            read_global_word(0x8031_8AE4),
            read_global_word(0x8031_8AE8),
            read_global_word(0x8031_8AD4),
            read_global_word(0x8031_8AD8),
            read_global_word(0x8031_8ADC),
        ],
    );
    w93_register_state(
        0x8037_17A0,
        0x8031_8B8C, // s_dPosGoAndComeExeFrm_c__StateID_D
        W93_FN_800D7FC0 as u32,
        0x8037_1790,
        W93_VTABLE_80318BD8 as u32,
        [
            read_global_word(0x8031_8B10),
            read_global_word(0x8031_8B14),
            read_global_word(0x8031_8B18),
            read_global_word(0x8031_8B04),
            read_global_word(0x8031_8B08),
            read_global_word(0x8031_8B0C),
            read_global_word(0x8031_8AF8),
            read_global_word(0x8031_8AFC),
            read_global_word(0x8031_8B00),
        ],
    );
    w93_register_state(
        0x8037_17E0,
        0x8031_8BB4, // s_dPosGoAndComeExeFrm_c__StateID_W
        W93_FN_800D7FC0 as u32,
        0x8037_17D0,
        W93_VTABLE_80318BD8 as u32,
        [
            read_global_word(0x8031_8B34),
            read_global_word(0x8031_8B38),
            read_global_word(0x8031_8B3C),
            read_global_word(0x8031_8B28),
            read_global_word(0x8031_8B2C),
            read_global_word(0x8031_8B30),
            read_global_word(0x8031_8B1C),
            read_global_word(0x8031_8B20),
            read_global_word(0x8031_8B24),
        ],
    );
}

/// Names one state descriptor, installs its vtable and its nine words, then
/// registers the handler.
#[inline]
unsafe fn w93_register_state(
    state: usize,
    name: u32,
    handler: u32,
    support: usize,
    vtable: u32,
    fields: [u32; 9],
) {
    let _ = platform::call2(W93_FN_8015F7C0, state as u32, name);
    write_word(state, 0x00, vtable);
    let mut index = 0;
    while index < 9 {
        write_word(state, 0x0C + index * 4, fields[index]);
        index += 1;
    }
    let _ = platform::call3(FN_802DC780, state as u32, handler, support as u32);
}

/// Applies the first per-player pad effect record to one display object.
///
/// In plain English: when the "route" bit is set and the player slot is valid,
/// fire `0x802BC9A0` with the effect pair and the slot's view; when the
/// "transform" bit is set, copy five words of the row record into
/// `+0x14..+0x24`, scale the second one and mark `+0x38` ready. Always store
/// the player byte at `+0x39`.
///
/// # Original
/// - Ghidra: `FUN_800d8850` at `0x800D8850` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8850_FUN_800d8850.c`
///
/// # Arguments
/// * `object` - effect display object; `+0x14..+0x24` transform words,
///   `+0x38` ready byte, `+0x39` player byte.
/// * `player` - player slot `0..3`.
/// * `row` - index into the `0x80318D38`/`0x80318D58` record tables.
/// * `flags` - bit 1 runs the route call, bit 0 copies the transform.
/// * `extra` - forwarded to `0x802BC9A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One route call and up to seven stores.
///
/// # Safety
/// [`G_PLAYER_SLOTS`] must point at a live container, `slot + 0x55` must be
/// readable, and `row` must index both record tables.
///
/// # Confidence
/// medium - the guards, the fields and the calls are literal; the `0x14`-byte
/// record stride is `// inferred:` from [`fn_800d8e40`], which copies the same
/// five words from the sibling table, and the SDA2 scale is `// unknown:`.
pub unsafe fn fn_800d8850(
    object: usize,
    player: u8,
    row: i32,
    flags: u32,
    extra: u32,
) {
    if (flags & 2) != 0 && (player as i32) < 4 {
        let slots = read_global_word(G_PLAYER_SLOTS) as usize;
        let slot = read_word(slots, player as usize * 4 + 4) as usize;
        if slot != 0 && read_byte(slot, 0x55) != 0 {
            let record = W93_TABLE_80318D58 + row as usize * 0x14;
            let view_index = read_word(slot, 0x04) as usize;
            let view = read_global_word(W93_TABLE_SLOT_VIEWS + view_index * 4);
            call_f64_u32_f32_u32(
                FN_802BC9A0,
                read_const_f32(record) as f64,
                view,
                read_const_f32(record + 4),
                extra,
            );
        }
    }

    if (flags & 1) != 0 {
        let record = W93_TABLE_80318D38 + row as usize * 0x14;
        write_word(object, 0x14, read_const_f32(record).to_bits());
        let second = read_const_f32(record + 4);
        write_word(object, 0x18, second.to_bits());
        write_word(object, 0x1C, read_const_f32(record + 8).to_bits());
        write_word(object, 0x20, read_const_f32(record + 12).to_bits());
        write_word(object, 0x24, read_const_f32(record + 16).to_bits());
        write_float(object, 0x18, second * read_const_f32(W93_F32_SDA2_6470));
        write_byte(object, 0x38, 1);
    }

    write_byte(object, 0x39, player);
}

/// Applies the second per-player pad effect record to one display object.
///
/// In plain English: when the player slot is valid and its `+0x55` route byte
/// is set, fire `0x802BC9A0` with the `0x80318C78` record pair and the slot's
/// view; then store the player byte and OR the flag word into `+0x30`.
///
/// # Original
/// - Ghidra: `FUN_800d8bb0` at `0x800D8BB0` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8bb0_FUN_800d8bb0.c`
///
/// # Arguments
/// * `object` - effect display object; `+0x30` flag word, `+0x39` player byte.
/// * `player` - player slot `0..3`.
/// * `row` - index into the `0x80318C78` record table.
/// * `flags` - OR-ed into `+0x30`.
/// * `extra` - forwarded to `0x802BC9A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One route call and two stores.
///
/// # Safety
/// [`G_PLAYER_SLOTS`] must point at a live container, the slot's `+0x04` id must
/// index [`W93_TABLE_SLOT_VIEWS`], and `row` must index the record table.
///
/// # Confidence
/// medium - the guard, the call and both stores are literal; the record stride
/// is `// inferred:` from the sibling [`fn_800d8e40`].
pub unsafe fn fn_800d8bb0(
    object: usize,
    player: u8,
    row: i32,
    flags: u32,
    extra: u32,
) {
    if (player as i32) < 4 {
        let slots = read_global_word(G_PLAYER_SLOTS) as usize;
        let slot = read_word(slots, player as usize * 4 + 4) as usize;
        if slot != 0 && read_byte(slot, 0x55) != 0 {
            let record = W93_TABLE_80318C78 + row as usize * 0x14;
            let view_index = read_word(slot, 0x04) as usize;
            let view = read_global_word(W93_TABLE_SLOT_VIEWS + view_index * 4);
            call_f64_u32_f32_u32(
                FN_802BC9A0,
                read_const_f32(record) as f64,
                view,
                read_const_f32(record + 4),
                extra,
            );
        }
    }
    write_byte(object, 0x39, player);
    let current = read_word(object, 0x30);
    write_word(object, 0x30, current | flags);
}

/// Initialises one effect display object from a row record.
///
/// In plain English: copy the five words of `0x80318CD8[row]` into
/// `+0x00..+0x10`, scale the second word by the SDA2 factor, store the player
/// byte at `+0x39`, OR the flags into `+0x30` and set the `+0x38` ready byte.
///
/// # Original
/// - Ghidra: `FUN_800d8e40` at `0x800D8E40` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8e40_FUN_800d8e40.c`
///
/// # Arguments
/// * `object` - effect display object.
/// * `player` - stored at `+0x39`.
/// * `row` - index into `0x80318CD8`.
/// * `flags` - OR-ed into `+0x30`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Six word writes and two byte writes.
///
/// # Safety
/// `object + 0x00..+0x39` must be writable and `row` must index the table.
///
/// # Confidence
/// medium - every store is literal; the `0x14`-byte stride is `// inferred:`
/// because Ghidra prints the record base as an indexed `undefined4*`, and the
/// SDA2 factor's value is `// unknown:`.
pub unsafe fn fn_800d8e40(object: usize, player: u8, row: i32, flags: u32) {
    let record = W93_TABLE_80318CD8 + row as usize * 0x14;
    write_word(object, 0x00, read_const_f32(record).to_bits());
    let second = read_const_f32(record + 4);
    write_word(object, 0x04, second.to_bits());
    write_word(object, 0x08, read_const_f32(record + 8).to_bits());
    write_word(object, 0x0C, read_const_f32(record + 12).to_bits());
    write_word(object, 0x10, read_const_f32(record + 16).to_bits());
    write_float(object, 0x04, second * read_const_f32(W93_F32_SDA2_6470));
    write_byte(object, 0x39, player);
    let current = read_word(object, 0x30);
    write_word(object, 0x30, current | flags);
    write_byte(object, 0x38, 1);
}

/// Advances the two damped float channels of one effect object.
///
/// In plain English: for the `+0x14` channel and then the `+0x00` channel, if
/// its speed is non-zero, interpolate it with `0x8015F140` and the easing row
/// returned by `0x800B2EE0(5)`; a zero result with a zero amplitude stops both
/// channels. Then age the two `+0x3C`/`+0x40` countdown words, setting or
/// clearing bits 1 and 2 of `+0x30`, and publish the two accumulated values at
/// `+0x28`/`+0x2C`.
///
/// # Original
/// - Ghidra: `FUN_800d8ea0` at `0x800D8EA0` (544 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8ea0_FUN_800d8ea0.c`
///
/// # Arguments
/// * `state` - effect object; two 5-float channels at `+0x00` and `+0x14`, the
///   output pair `+0x28/+0x2C`, the flag word `+0x30` and the two countdowns at
///   `+0x3C/+0x40`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to two interpolation calls and about ten stores.
///
/// # Safety
/// `state + 0x00..+0x44` must be readable/writable and its `+0x18`/`+0x04`
/// words must be valid inputs for `0x8015F140`.
///
/// # Confidence
/// medium - the control flow, the two easing tables and every store are
/// literal; the five SDA2 floats are `// unknown:` and the `+0x3C/+0x40` roles
/// as countdowns are `// inferred:` from the decrement pattern.
pub unsafe fn fn_800d8ea0(state: usize) {
    let zero = read_const_f32(W93_F32_SDA2_646C);
    let mut accumulated_y = zero;
    let mut accumulated_x = zero;

    if read_float(state, 0x18) != zero {
        let easing = platform::call1(W93_FN_800B2EE0, 5);
        let result = call_f64x4_ptr(
            W93_FN_8015F140,
            read_float(state, 0x14) as f64,
            read_float(state, 0x1C) as f64,
            read_float(state, 0x20) as f64,
            read_float(state, 0x24) as f64,
            (state + 0x18) as u32,
        );
        let zero_again = read_const_f32(W93_F32_SDA2_646C);
        let mut stop = false;
        if result == zero_again as f64 {
            if read_float(state, 0x14) as f64 == zero_again as f64 {
                write_float(state, 0x30, 0.0);
                write_byte(state, 0x38, 0);
                accumulated_y = zero_again;
                stop = true;
            }
        }
        if !stop {
            accumulated_y = read_float(state, 0x18)
                * read_const_f32(W93_TABLE_EASE_Y + easing as usize * 8);
            accumulated_x = read_float(state, 0x18)
                * read_const_f32(W93_TABLE_EASE_X + easing as usize * 8)
                * read_const_f32(W93_F32_SDA2_6468);
            if (read_word(state, 0x34) & 4) != 0 {
                accumulated_x *= read_const_f32(W93_F32_SDA2_6468);
            }
        }
    }

    if read_float(state, 0x04) != read_const_f32(W93_F32_SDA2_646C) {
        let easing = platform::call1(W93_FN_800B2EE0, 5);
        let result = call_f64x4_ptr(
            W93_FN_8015F140,
            read_float(state, 0x00) as f64,
            read_float(state, 0x08) as f64,
            read_float(state, 0x0C) as f64,
            read_float(state, 0x10) as f64,
            (state + 0x04) as u32,
        );
        let zero_again = read_const_f32(W93_F32_SDA2_646C);
        let mut stop = false;
        if result == zero_again as f64 {
            if read_float(state, 0x00) as f64 == zero_again as f64 {
                write_float(state, 0x30, 0.0);
                write_byte(state, 0x38, 0);
                accumulated_y = zero_again;
                accumulated_x = zero_again;
                stop = true;
            }
        }
        if !stop {
            accumulated_y += read_float(state, 0x04)
                * read_const_f32(W93_TABLE_EASE_Y + easing as usize * 8);
            accumulated_x = (accumulated_x
                + read_float(state, 0x04)
                    * read_const_f32(W93_TABLE_EASE_X + easing as usize * 8))
                * read_const_f32(W93_F32_SDA2_6468);
            if (read_word(state, 0x34) & 4) != 0 {
                accumulated_x *= read_const_f32(W93_F32_SDA2_6468);
            }
        }
    }

    // observed: two countdown words that latch bit 1/2 while running.
    if (read_word(state, 0x3C) as i32) > 0 {
        let flags = read_word(state, 0x30);
        write_word(state, 0x30, flags | 2);
        if (read_global_byte(W93_G_TIMER_FREEZE) & 8) == 0 {
            let remaining = (read_word(state, 0x3C) as i32) - 1;
            write_word(state, 0x3C, remaining as u32);
            if remaining == 0 {
                write_word(state, 0x30, flags & 0xFFFF_FFFD);
            }
        }
    }
    if (read_word(state, 0x40) as i32) > 0 {
        let flags = read_word(state, 0x30);
        write_word(state, 0x30, flags | 4);
        if (read_global_byte(W93_G_TIMER_FREEZE) & 8) == 0 {
            let remaining = (read_word(state, 0x40) as i32) - 1;
            write_word(state, 0x40, remaining as u32);
            if remaining == 0 {
                write_word(state, 0x30, flags & 0xFFFF_FFFB);
            }
        }
    }

    write_float(state, 0x28, accumulated_x);
    write_float(state, 0x2C, accumulated_y);
}

// ---------------------------------------------------------------------------
// Wave 93 - 0x800DD770..0x800DFE60: dConnect_c, replay HUD and insert helpers
// ---------------------------------------------------------------------------

/// Releases one `dRemoconMng_c::dConnect_c` state object.
///
/// In plain English: when the object is non-null, run the generic state
/// destructor `0x8015F800(object, 0)`; when `free` is positive, also free the
/// storage. Always return the object.
///
/// # Original
/// - Ghidra: `FUN_800dd770` at `0x800DD770` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd770_FUN_800dd770.c`
///
/// # Arguments
/// * `object` - state object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// The destructor call and, conditionally, the free.
///
/// # Safety
/// `object` must be null or a live state object.
///
/// # Confidence
/// high - both guards, the destructor and the free are literal; this is the
/// same shape as [`fn_800de850`].
pub unsafe fn fn_800dd770(object: usize, free: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Constructs the `dRemoconMng_c::dConnect_c` state object.
///
/// In plain English: install the base and interface vtables, publish the owner
/// at `+0x14`, build the embedded state manager with `0x8015FC90`, run the
/// `+0x40` object constructor, then fill the two rotation triples from SDA2
/// defaults and clear the id/timer words.
///
/// # Original
/// - Ghidra: `FUN_800dd950` at `0x800DD950` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd950_FUN_800dd950.c`
///
/// # Arguments
/// * `object` - storage for the state object.
/// * `owner` - stored at `+0x54`.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Three constructor calls and about thirty field writes.
///
/// # Safety
/// `object` must be at least `0xF0` writable bytes and the `0x8015FC90`
/// manager must accept the four printed operands.
///
/// # Confidence
/// medium - every store, vtable and call is literal; the roles of
/// `+0x78..+0x98` (two rotation triples) are `// inferred:` from their SDA2
/// defaults, and the `+0x9C/+0xA8/+0xAA` halfword pairs are cleared without a
/// recovered name.
pub unsafe fn fn_800dd950(object: usize, owner: u32) -> usize {
    write_word(object, 0x00, 0x8031_92C8);
    write_word(object, 0x04, 0x8031_9308);
    write_word(object, 0x14, object as u32);
    write_word(object, 0x08, 0x802F_EAF8);
    write_word(object, 0x0C, 0x8031_9338);
    write_word(object, 0x10, 0x8031_9350);
    write_word(object, 0x18, 0);

    let _ = fn_8015fc90(
        object + 0x1C,
        (object + 0x08) as u32,
        (object + 0x0C) as u32,
        0x8037_19B0,
    );

    write_word(object, 0x04, 0x8031_92D8);
    let _ = platform::call1(W93_FN_8008F140, (object + 0x40) as u32);

    let default_a = read_const_f32(W93_F32_SDA2_6408);
    let default_b = read_const_f32(W93_F32_SDA2_6404);
    let default_c = read_const_f32(W93_F32_SDA2_6400);

    write_word(object, 0x54, owner);
    write_word(object, 0x58, 0);
    write_word(object, 0x5C, 0);
    write_word(object, 0x64, 0);
    write_word(object, 0x68, 0);
    write_word(object, 0x6C, 0);
    write_word(object, 0x70, 0);
    write_word(object, 0x74, 0);
    write_word(object, 0x78, default_a.to_bits());
    write_word(object, 0x7C, default_b.to_bits());
    write_word(object, 0x80, default_a.to_bits());
    write_word(object, 0x84, default_c.to_bits());
    write_word(object, 0x88, default_a.to_bits());
    write_word(object, 0x8C, default_a.to_bits());
    write_word(object, 0x90, default_a.to_bits());
    write_word(object, 0x94, default_a.to_bits());
    write_word(object, 0x98, default_a.to_bits());
    write_half(object, 0x9C, 0);
    write_half(object, 0xA8, 0);
    write_half(object, 0xAA, 0);
    write_word(object, 0xEC, 0);
    object
}

/// Frees one state object when `free` is positive.
///
/// In plain English: the short `dConnect_c` destructor variant. It keeps only
/// the non-null and `free > 0` guards and the free call.
///
/// # Original
/// - Ghidra: `FUN_800ddab0` at `0x800DDAB0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800ddab0_FUN_800ddab0.c`
///
/// # Arguments
/// * `object` - state object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May free the object storage.
///
/// # Safety
/// `object` must be null or a live allocation.
///
/// # Confidence
/// medium - the two guards and the return are literal; the free call is printed
/// with no operand, and `object` is `// inferred:` from the two sibling
/// destructors [`fn_800dd770`] and [`fn_800ddaf0`].
pub unsafe fn fn_800ddab0(object: usize, free: i32) -> usize {
    if object != 0 && free > 0 {
        let _ = platform::call1(FN_802B9280, object as u32);
    }
    object
}

/// Releases the embedded `+0x18` object, then the state object itself.
///
/// In plain English: when `object` is non-null, run `0x8015F9F0(object + 0x18,
/// 0)` (the export always takes this branch because `object + 0x18` cannot be
/// null) and, when `free` is positive, free the storage.
///
/// # Original
/// - Ghidra: `FUN_800ddaf0` at `0x800DDAF0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800ddaf0_FUN_800ddaf0.c`
///
/// # Arguments
/// * `object` - state object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Releases the embedded object and, conditionally, the storage.
///
/// # Safety
/// `object` must be null or a live object with a destructible `+0x18` member.
///
/// # Confidence
/// high - the nested guards, the `+0x18` offset and the free are literal.
pub unsafe fn fn_800ddaf0(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Recomputes the replay HUD's `+0xA8` visibility flags.
///
/// In plain English: set bit `0x20` while the scene's `+0x1D0` is `2`, set bit
/// `0x40` while the save slot is absent, set bit `2` while the player manager's
/// count is not `1`, and set bits `4`, `0x100` and `8` from the three
/// `0x800B3B50` pad queries; mirror the halfword at `+0xAA`, then switch between
/// the empty and the playing replay state depending on whether any flag is set.
///
/// # Original
/// - Ghidra: `FUN_800ddc50` at `0x800DDC50` (532 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800ddc50_FUN_800ddc50.c`
///
/// # Arguments
/// * `object` - replay HUD; `+0x04` state interface, `+0xA0` timer, `+0xA8`
///   flag halfword, `+0xAA` mirror.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Six bit updates, one interface state switch and one interface tick.
///
/// # Safety
/// `object + 0x04` must be a live state interface and the two SDA slots must be
/// initialised.
///
/// # Confidence
/// high - every bit, mask, table literal and call is literal; the
/// `countLeadingZeros(count - 1) >> 5` test is reproduced verbatim and therefore
/// encodes exactly the `count != 1` predicate. The two SDA slots are named as
/// in the wave-20 save-image bodies, not guessed.
pub unsafe fn fn_800ddc50(object: usize) {
    let scene = call0_r(W93_FN_80109340) as usize;
    if read_word(scene, 0x1D0) == 2 {
        let flags = read_half(object, 0xA8);
        write_half(object, 0xA8, flags | 0x20);
    } else {
        let flags = read_half(object, 0xA8);
        write_half(object, 0xA8, flags & 0xFFDF);
    }

    let slot = platform::call1(W93_FN_80107850, read_global_word(W93_G_SAVE_SLOT_53E8));
    if slot == 0 {
        let flags = read_half(object, 0xA8);
        write_half(object, 0xA8, flags & 0xFFBF);
    } else {
        let flags = read_half(object, 0xA8);
        write_half(object, 0xA8, flags | 0x40);
    }

    let manager = read_global_word(G_PLAYER_MANAGER) as usize;
    let count = vcall0(manager, 0x10);
    // observed: clz(count - 1) >> 5 == 0, i.e. count != 1.
    if (count.wrapping_sub(1).leading_zeros() >> 5) == 0 {
        let flags = read_half(object, 0xA8);
        write_half(object, 0xA8, flags | 2);
    } else {
        let flags = read_half(object, 0xA8);
        write_half(object, 0xA8, flags & 0xFFFD);
    }

    let queries = [(1u32, 0x0004u16, 0xFFFBu16), (4, 0x0100, 0xFEFF), (2, 0x0008, 0xFFF7)];
    for (button, set_bit, clear_mask) in queries {
        let answer = platform::call1(W93_FN_800B3B50, button);
        let flags = read_half(object, 0xA8);
        if answer == 0 {
            write_half(object, 0xA8, flags & clear_mask);
        } else {
            write_half(object, 0xA8, flags | set_bit);
        }
    }

    write_half(object, 0xAA, read_half(object, 0xA8));

    let interface = object + 0x04;
    if read_half(object, 0xA8) == 0 {
        let state = vcall0(interface, 0x28) as usize;
        if vcall1(state, 0x10, 0x8037_19F0) != 0 {
            let _ = vcall1(interface, 0x18, 0x8037_1A30);
        }
    } else {
        let state = vcall0(interface, 0x28) as usize;
        if vcall1(state, 0x10, 0x8037_1A30) != 0 {
            let _ = vcall1(interface, 0x18, 0x8037_19F0);
        }
    }

    write_word(object, 0xA0, 0x78);
    let _ = vcall0(interface, 0x10);
}

/// Decodes one event record from the replay stream.
///
/// In plain English: bump the step counter, read the flag word at the stream
/// cursor and advance it; `0xFFFFFFFF` ends the replay, otherwise copy the two
/// fixed words and, for each set bit of the flag word, consume the optional
/// records (transform, rotation pairs, halfword and trailing word) before
/// publishing the new flag word at `+0x68`.
///
/// # Original
/// - Ghidra: `FUN_800de020` at `0x800DE020` (488 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de020_FUN_800de020.c`
///
/// # Arguments
/// * `object` - replay state; `+0x5C` stream cursor, `+0x74` step counter,
///   `+0x64/+0x68` current/previous flags and `+0x6C..+0xA4` the decoded fields.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Advances the cursor by up to sixteen words and writes about twenty fields.
///
/// # Safety
/// The cursor at `+0x5C` must point into a readable replay stream and the
/// object must have `0xA8` writable bytes.
///
/// # Confidence
/// medium - every bit, offset and cursor increment is literal; the flag-bit
/// semantics are `// inferred:` from the order the optional records are
/// consumed, and the three SDA2 defaults are `// unknown:`.
pub unsafe fn fn_800de020(object: usize) {
    let mut cursor = read_word(object, 0x5C) as usize;
    write_word(object, 0x74, read_word(object, 0x74).wrapping_add(1));

    let flags = read_word(cursor, 0);
    write_word(object, 0x64, flags);
    cursor += 4;
    write_word(object, 0x5C, cursor as u32);

    if flags == 0xFFFF_FFFF {
        let _ = vcall1(object + 0x04, 0x18, 0x8037_1A70);
        return;
    }

    write_word(object, 0xA0, read_word(cursor, 0));
    cursor += 4;
    write_word(object, 0x6C, read_word(cursor, 0));
    cursor += 4;
    if (flags & 0x1000_0000) != 0 {
        write_word(object, 0x70, read_word(cursor, 0));
        cursor += 4;
    }

    let default_a = read_const_f32(W93_F32_SDA2_6408).to_bits();
    let default_b = read_const_f32(W93_F32_SDA2_6404).to_bits();
    let default_c = read_const_f32(W93_F32_SDA2_6400).to_bits();
    write_word(object, 0x78, default_a);
    write_word(object, 0x7C, default_b);
    write_word(object, 0x80, default_a);
    write_word(object, 0x84, default_c);
    write_word(object, 0x88, default_a);
    write_word(object, 0x8C, default_a);
    write_word(object, 0x90, default_a);
    write_word(object, 0x94, default_a);
    write_word(object, 0x98, default_a);
    write_half(object, 0x9C, 0);

    if (flags & 0x0800_0000) != 0 {
        write_word(object, 0x78, read_word(cursor, 0));
        write_word(object, 0x7C, read_word(cursor, 4));
        write_word(object, 0x80, read_word(cursor, 8));
        cursor += 12;
    }
    if (flags & 0x0400_0000) != 0 {
        write_word(object, 0x84, read_word(cursor, 0));
        write_word(object, 0x88, read_word(cursor, 4));
        cursor += 8;
    }
    if (flags & 0x0200_0000) != 0 {
        write_word(object, 0x8C, read_word(cursor, 0));
        write_word(object, 0x90, read_word(cursor, 4));
        cursor += 8;
    }
    if (flags & 0x0100_0000) != 0 {
        write_word(object, 0x94, read_word(cursor, 0));
        write_word(object, 0x98, read_word(cursor, 4));
        cursor += 8;
    }
    if (flags & 0x0080_0000) != 0 {
        write_half(object, 0x9C, read_half(cursor, 0));
        cursor += 2;
    }
    if (flags & 0x0040_0000) != 0 {
        write_word(object, 0xA4, read_word(cursor, 0));
        cursor += 4;
    }

    write_word(object, 0x5C, cursor as u32);
    write_word(object, 0x68, read_word(object, 0x64));
}

/// Registers the four `dReplayPlay_c::StateID` descriptors.
///
/// In plain English: for the StartWait, Wait, Play and PlayEnd states, name the
/// descriptor, install the `0x803194F4` vtable, copy the nine handler words and
/// register `0x800DE850` with the per-state support object.
///
/// # Original
/// - Ghidra: `FUN_800de510` at `0x800DE510` (824 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de510_FUN_800de510.c`
/// - Removed: the modelled `0x802DCD74`/`0x802DCDC0` calls. They are the
///   `_savegpr_27`/`_restgpr_27` compiler helpers (register spill/restore
///   only); both results were already discarded.
///
/// # Arguments
/// None. The four descriptors and their support objects are absolute globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills `0x803719B0`, `0x803719F0`, `0x80371A30` and `0x80371A70` and
/// registers each one through `0x802DC780`.
///
/// # Safety
/// The four descriptor blocks and their support objects must be writable.
///
/// # Confidence
/// medium - the four blocks, their names, the nine copied words and the
/// registration call are literal; the roles of the nine handler words are
/// `// inferred:` from the shared `+0x0C..+0x2C` layout.
pub unsafe fn fn_800de510() {
    w93_register_state(
        0x8037_19B0,
        0x8031_9478, // s_dReplayPlay_c__StateID_StartWait
        W93_FN_800DE850 as u32,
        0x8037_19A0,
        W93_VTABLE_803194F4 as u32,
        [
            read_global_word(0x8031_9400),
            read_global_word(0x8031_9404),
            read_global_word(0x8031_9408),
            read_global_word(0x8031_93F4),
            read_global_word(0x8031_93F8),
            read_global_word(0x8031_93FC),
            read_global_word(0x8031_93E8),
            read_global_word(0x8031_93EC),
            read_global_word(0x8031_93F0),
        ],
    );
    w93_register_state(
        0x8037_19F0,
        0x8031_949C, // s_dReplayPlay_c__StateID_Wait
        W93_FN_800DE850 as u32,
        0x8037_19E0,
        W93_VTABLE_803194F4 as u32,
        [
            read_global_word(0x8031_9424),
            read_global_word(0x8031_9428),
            read_global_word(0x8031_942C),
            read_global_word(0x8031_9418),
            read_global_word(0x8031_941C),
            read_global_word(0x8031_9420),
            read_global_word(0x8031_940C),
            read_global_word(0x8031_9410),
            read_global_word(0x8031_9414),
        ],
    );
    w93_register_state(
        0x8037_1A30,
        0x8031_94B8, // s_dReplayPlay_c__StateID_Play
        W93_FN_800DE850 as u32,
        0x8037_1A20,
        W93_VTABLE_803194F4 as u32,
        [
            read_global_word(0x8031_9448),
            read_global_word(0x8031_944C),
            read_global_word(0x8031_9450),
            read_global_word(0x8031_943C),
            read_global_word(0x8031_9440),
            read_global_word(0x8031_9444),
            read_global_word(0x8031_9430),
            read_global_word(0x8031_9434),
            read_global_word(0x8031_9438),
        ],
    );
    w93_register_state(
        0x8037_1A70,
        0x8031_94D4, // s_dReplayPlay_c__StateID_PlayEnd
        W93_FN_800DE850 as u32,
        0x8037_1A60,
        W93_VTABLE_803194F4 as u32,
        [
            read_global_word(0x8031_946C),
            read_global_word(0x8031_9470),
            read_global_word(0x8031_9474),
            read_global_word(0x8031_9460),
            read_global_word(0x8031_9464),
            read_global_word(0x8031_9468),
            read_global_word(0x8031_9454),
            read_global_word(0x8031_9458),
            read_global_word(0x8031_945C),
        ],
    );
}

/// Spawns a batch of named replay records through `0x800DF990`.
///
/// In plain English: take the replay context pair, then for each of `count`
/// names in the table call `0x800DF990(context_hi, context_lo, *names, extra)`
/// and advance the name pointer. This is the name-resolution twin of
/// [`fn_800dfb70`], which spawns record words instead.
///
/// # Original (untabled)
/// - Ghidra: none - no functions.tsv row for 0x800DFB00.
/// - Extent: `0x800DFB00..0x800DFB6C` (108 bytes, 25 instructions), ending at
///   the `blr` at `0x800DFB68`; one zero word follows at `0x800DFB6C` and the
///   next prologue is at `0x800DFB70`.
/// - Reach: five `bl` sites in `d_basesNP` (`0x80770208`, `0x80770220`,
///   `0x80777508`, `0x80777588`, `0x80777618`); no main-image branch and no
///   data word names it.
/// - Table: none.
/// - Observed: `stwu r1,-0x20(r1)`; `mflr r0`; `addi r11,r1,0x20`;
///   `bl 0x802DCD70` (`_savegpr_26`); `mr r26,r3`; `mr r27,r4`; `mr r28,r5`;
///   `mr r31,r6`; `mr r29,r7`; `li r30,0`; `b` to the test; loop
///   `lwz r5,0(r31)`; `mr r3,r26`; `mr r4,r27`; `mr r6,r29`;
///   `bl 0x800DF990`; `addi r31,r31,4`; `addi r30,r30,1`; test
///   `cmp cr0,r30,r28`; `bc 12,0` (`bt cr0.lt`) back to the loop; then
///   `bl 0x802DCDBC` (`_restgpr_26`); epilogue; `4E800020 bclr 20,0` (blr).
/// - Polarity: `4180FFE0` is `bt cr0.lt`, so the loop repeats while
///   `index < count` - a do-while with the test at the bottom.
/// - Removed: the `0x802DCD70`/`0x802DCDBC` calls are the `_savegpr_26` /
///   `_restgpr_26` register spill and restore helpers, so the live parameters
///   are the incoming `r3:r7`.
///
/// # Arguments
/// - `context_hi`, `context_lo`: the replay context pair (incoming `r3:r4`).
/// - `count`: number of names.
/// - `names`: first name word; the pointer advances by four bytes.
/// - `extra`: forwarded to every spawn.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `count` calls to [`fn_800df990`].
///
/// # Safety
/// `count` must not exceed the number of readable words at `names`, each word
/// must be a readable name, and the context pair must be live.
///
/// # Confidence
/// high - the loop, the pointer step, both context words and the callee are
/// literal; only the `_savegpr`/`_restgpr` helpers are elided.
pub unsafe fn fn_800dfb00(
    context_hi: u32,
    context_lo: u32,
    count: i32,
    names: usize,
    extra: u32,
) {
    let mut cursor = names;
    let mut index = 0i32;
    while index < count {
        fn_800df990(
            context_hi as usize,
            context_lo as usize,
            read_word(cursor, 0) as usize,
            extra,
        );
        cursor += 4;
        index += 1;
    }
}

/// Spawns every record of one table through `0x800DFA40`.
///
/// In plain English: take the replay context pair, then for each of `count`
/// entries call `0x800DFA40(context_hi, context_lo, *values, extra)` and advance
/// the value pointer.
///
/// # Original
/// - Ghidra: `FUN_800dfb70` at `0x800DFB70` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfb70_FUN_800dfb70.c`
/// - Removed: the modelled `0x802DCD70`/`0x802DCDBC` calls. They are the
///   `_savegpr_26`/`_restgpr_26` compiler helpers (register spill/restore
///   only), so the context pair is the incoming `r3:r4`.
///
/// # Arguments
/// * `context_hi`, `context_lo` - the replay context pair (incoming `r3:r4`),
///   forwarded as the first two spawn arguments.
/// * `count` - number of records.
/// * `values` - first record word; the pointer advances by four bytes.
/// * `extra` - forwarded to every spawn.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `count` spawn calls.
///
/// # Safety
/// `count` must not exceed the number of readable words at `values`, and the
/// incoming context pair must be live.
///
/// # Confidence
/// medium - the loop, the pointer step and both context words are literal; the
/// callee's own parameter names are not claimed.
pub unsafe fn fn_800dfb70(
    context_hi: u32,
    context_lo: u32,
    count: i32,
    values: usize,
    extra: u32,
) {
    let mut cursor = values;
    let mut index = 0i32;
    while index < count {
        let _ = platform::call4(
            W93_FN_800DFA40,
            context_hi,
            context_lo,
            read_word(cursor, 0),
            extra,
        );
        cursor += 4;
        index += 1;
    }
}

/// Resolves a name in an archive and forwards the index to `0x800DFA40`.
///
/// In plain English: if the owner has a `+0x04` interface, query slot `+0x08`
/// with the `0x802F86C8` tag; build the lookup key from `object`, resolve
/// `name`, and when the index is not `-1` spawn it.
///
/// # Original
/// - Ghidra: `FUN_800df990` at `0x800DF990` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df990_FUN_800df990.c`
///
/// # Arguments
/// * `object` - key owner passed to `0x80168400` and forwarded.
/// * `owner` - object whose `+0x04` interface is queried.
/// * `name` - the name resolved by `0x80164F00`.
/// * `extra` - forwarded to the spawn.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One interface query, one archive lookup and, on a hit, one spawn.
///
/// # Safety
/// `owner + 0x04` must be a live interface and `name` a readable string.
///
/// # Confidence
/// medium - the guard, the two lookups and the spawn are literal; the type tag
/// `0x802F86C8` and the interface's slot `+0x08` contract are library details.
pub unsafe fn fn_800df990(object: usize, owner: usize, name: usize, extra: u32) {
    let interface = read_word(owner, 0x04) as usize;
    if interface != 0 {
        let mut tag = W93_TAG_802F86C8 as u32;
        let _ = vcall1(interface, 0x08, (&mut tag as *mut u32) as u32);
    }

    let key = platform::call1(W93_FN_80168400, object as u32);
    let mut key_word = key;
    let index = platform::call2(
        W93_FN_80164F00,
        (&mut key_word as *mut u32) as u32,
        name as u32,
    );
    if index != 0xFFFF_FFFF {
        let _ = platform::call4(W93_FN_800DFA40, object as u32, owner as u32, index, extra);
    }
}

/// Reads the id of the current node behind a handle.
///
/// In plain English: when `handle[0]` is non-null, take the current node from
/// `0x8023B2A0`, follow its `+0x60` self-relative offset when non-zero, and
/// return the node's `+0x0C` id; otherwise return `-1`.
///
/// # Original
/// - Ghidra: `FUN_800dfc70` at `0x800DFC70` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfc70_FUN_800dfc70.c`
///
/// # Arguments
/// * `handle` - pointer to the object whose presence gates the query.
///
/// # Returns
/// The node id, or `0xFFFFFFFF`.
///
/// # Side effects
/// None beyond the query call.
///
/// # Safety
/// `handle` must be readable and the node returned by `0x8023B2A0` must be a
/// live object with a `+0x60` offset.
///
/// # Confidence
/// medium - the guard, the self-relative offset and the id load are literal;
/// the export drops `0x8023B2A0`'s operand and Ghidra removed an unreachable
/// block at `0x800DFCB8`, so the query's receiver is `// unknown:`.
pub unsafe fn fn_800dfc70(handle: usize) -> u32 {
    if read_word(handle, 0) != 0 {
        let node = call0_r(FN_8023B2A0) as usize;
        let offset = read_word(node, 0x60);
        let node = if offset == 0 {
            0
        } else {
            node.wrapping_add(offset as usize)
        };
        if node != 0 {
            return read_word(node, 0x0C);
        }
    }
    0xFFFF_FFFF
}

/// Counts the nodes of the current object's `+0x64` list.
///
/// In plain English: start from the object's current node, then walk the
/// `+0x64` self-relative next links until the next node's `+0x0C` id equals the
/// head's, returning the number of visited nodes.
///
/// # Original
/// - Ghidra: `FUN_800dfd20` at `0x800DFD20` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfd20_FUN_800dfd20.c`
///
/// # Arguments
/// * `handle` - pointer to the object whose `+0x60` word yields the head node.
///
/// # Returns
/// `0` when there is no head, otherwise the count of nodes up to the repeated
/// id.
///
/// # Side effects
/// None.
///
/// # Safety
/// `handle` and every walked node must be readable.
///
/// # Confidence
/// high - the two null guards, the self-relative `+0x64` step and the id
/// comparison are literal; the `+0x0C` word's role as an id is `// inferred:`
/// from the comparison.
pub unsafe fn fn_800dfd20(handle: usize) -> i32 {
    let mut node = read_word(handle, 0) as usize;
    if node == 0 {
        return 0;
    }
    let offset = read_word(node, 0x60);
    node = if offset == 0 {
        0
    } else {
        node.wrapping_add(offset as usize)
    };
    if node == 0 {
        return 0;
    }

    let mut count = 1i32;
    let mut next = {
        let step = read_word(node, 0x64);
        if step == 0 {
            0
        } else {
            node.wrapping_add(step as usize)
        }
    };
    loop {
        if next == 0 {
            return count;
        }
        let head_id = if node == 0 { 0 } else { read_word(node, 0x0C) };
        let next_id = if next == 0 { 0 } else { read_word(next, 0x0C) };
        if next_id == head_id {
            break;
        }
        count += 1;
        let step = read_word(next, 0x64);
        next = if step == 0 {
            0
        } else {
            next.wrapping_add(step as usize)
        };
    }
    count
}

/// Resolves one partner index and forwards it to `0x800DFF30`.
///
/// In plain English: take the replay context, query its `+0x04` interface with
/// the `0x802F8FF0` tag, resolve the `+0xE8` handle against the context's low
/// word, and when the resulting id is non-negative call `0x800DFF30`.
///
/// # Original
/// - Ghidra: `FUN_800dfe60` at `0x800DFE60` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfe60_FUN_800dfe60.c`
/// - Removed: the modelled `0x802DCD70`/`0x802DCDBC` calls. They are the
///   `_savegpr_26`/`_restgpr_26` compiler helpers (register spill/restore
///   only), so the context pair is the incoming `r3:r4`.
///
/// # Arguments
/// * `context_hi`, `context_lo` - the replay context pair (incoming `r3:r4`).
/// * `value` - forwarded as the third argument of `0x800DFF30`.
/// * `extra` - forwarded as its fourth argument.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One interface query, one partner lookup and one conditional call.
///
/// # Safety
/// The context's `+0x04` interface must be live when non-null.
///
/// # Confidence
/// medium - the query, the `+0xE8` handle, the id sign test and the forwarded
/// call are literal; the context pair is the incoming `r3:r4`.
pub unsafe fn fn_800dfe60(context_hi: u32, context_lo: u32, value: u32, extra: u32) {
    let object = context_hi as usize;

    let mut interface = read_word(object, 0x04) as usize;
    let mut tag = W93_TAG_802F8FF0 as u32;
    let mut valid = false;
    if interface != 0 {
        valid = vcall1(interface, 0x08, (&mut tag as *mut u32) as u32) != 0;
    }
    if !valid {
        interface = 0;
    }

    let mut handle = read_word(interface, 0xE8);
    let found = platform::call2(
        W93_FN_8023B200,
        (&mut handle as *mut u32) as u32,
        context_lo,
    );
    let index = if found == 0 {
        0
    } else {
        read_word(found as usize, 0x0C) as i32
    };
    if index >= 0 {
        let _ = platform::call4(W93_FN_800DFF30, object as u32, index as u32, value, extra);
    }
}

// ===========================================================================
// Wave 95: the `0x80150000` scene wrapper, its state handlers and the
// `0x8015C..`/`0x8015F..` boot helpers
// ===========================================================================
//
// The bodies below all live in the same engine bucket as waves 78/90/93. Three
// shapes repeat and are documented once here:
//
// * the scene object keeps a `+0x14D8` manager pointer, a `+0x14xx` interface
//   table pointer and the `+0x1520/+0x1524` current-owner pair;
// * every "gate" body writes `1` to `+0x1544` before a virtual call and `0`
//   after it, so the callback can detect re-entry;
// * the `r13`/`r2` anchors are the small-data bases of the module glossary,
//   printed by Ghidra as `unaff_r13`/`unaff_r2`.

/// `0x80056CB0(object, id)`: registers one pad/event channel id on `object`.
// inferred: the three `0x80056Cxx` helpers below are the pad-channel trio; the
// export only shows the call shape, never their bodies.
const W95_FN_80056CB0: usize = 0x8005_6CB0;
/// `0x80056C70(object, id)`: removes one pad/event channel id from `object`.
const W95_FN_80056C70: usize = 0x8005_6C70;
/// `0x80056CF0(object, id)`: true while `object` still holds channel `id`.
const W95_FN_80056CF0: usize = 0x8005_6CF0;
/// `0x8004DB40(object, 4)`: the second channel-state query of `0x80152020`.
const W95_FN_8004DB40: usize = 0x8004_DB40;
/// `0x80053C80(object, first, second)`: pair-contact test of `0x80152020`.
const W95_FN_80053C80: usize = 0x8005_3C80;
/// `0x80053E60(x, object, 1)`: applies the contact impulse of `0x80152020`.
const W95_FN_80053E60: usize = 0x8005_3E60;
/// `0x80053DF0(object)`: clears the contact state of `0x80152020`.
const W95_FN_80053DF0: usize = 0x8005_3DF0;
/// `0x80049C70(object, other)`: fallback contact test of `0x80152020`.
const W95_FN_80049C70: usize = 0x8004_9C70;
/// `0x80050D80()`: the mode gate of `0x80152B30`.
const W95_FN_80050D80: usize = 0x8005_0D80;
/// `0x8014ED00(scene, name)`: installs the scene's current layout name.
const W95_FN_8014ED00: usize = 0x8014_ED00;
/// `0x80150650(scene)`: "layout change finished" predicate of `0x801504D0`.
const W95_FN_80150650: usize = 0x8015_0650;
/// `0x800A7540` / `0x800A7550`: queue one screen entry for the display owner.
const W95_FN_800A7540: usize = 0x800A_7540;
const W95_FN_800A7550: usize = 0x800A_7550;
/// `0x8014E9B0(scene)`: returns the scene's current player/actor view.
const W95_FN_8014E9B0: usize = 0x8014_E9B0;
/// `0x80162D00(id)`: resolves a scene id to its scene object.
const W95_FN_80162D00: usize = 0x8016_2D00;
/// `0x801391F0(view, arg)`: forwards one event into a view.
const W95_FN_801391F0: usize = 0x8013_91F0;
/// `0x800D5790(manager, out, id)`: reads two floats of one layout pane.
const W95_FN_800D5790: usize = 0x800D_5790;
/// `0x800D8BB0(manager, player, row, a, b)`: per-player pad effect writer.
const W95_FN_800D8BB0: usize = 0x800D_8BB0;
/// `0x8015F620(table, arg)`: boot-screen state-table query.
const W95_FN_8015F620: usize = 0x8015_F620;
/// `0x80108150()`: releases the live layout object.
// inferred: the body is a bare call whose result the caller discards; the
// "release" role comes from its pairing with `0x80108160` in this cluster.
const W95_FN_80108150: usize = 0x8010_8150;
/// `0x800E1F50` / `0x800E1F60`: enable/disable one transition channel.
// inferred: both take the same `0x1E` argument and are only called as a pair.
const W95_FN_800E1F50: usize = 0x800E_1F50;
const W95_FN_800E1F60: usize = 0x800E_1F60;
/// `0x8008F610(id)`: true when the resource `id` is resident.
// inferred: the two callers test the result before committing a mode.
const W95_FN_8008F610: usize = 0x8008_F610;
/// `0x800E5060(value)`: commits the boot screen's requested mode.
// inferred: it is the last call of the successful handshake.
const W95_FN_800E5060: usize = 0x800E_5060;
/// `0x80156820(owner, index)`: maps the `index`-th connected player to its id.
const W95_FN_80156820: usize = 0x8015_6820;
/// `0x801568D0()`: counts the connected player slots.
const W95_FN_801568D0: usize = 0x8015_68D0;

/// `0x80376E88`: layout name installed when the scene's layout change ends.
const W95_G_80376E88: usize = 0x8037_6E88;
/// `0x80376AC8`: actor/model name compared by `0x80151D00`.
const W95_G_80376AC8: usize = 0x8037_6AC8;
/// `0x80376B48`: transition descriptor handed to the scene's slot `+0x294`.
const W95_G_80376B48: usize = 0x8037_6B48;
/// `0x80377448`: the `dScBoot_c::StateID_ResetFadeOut` descriptor.
const W95_G_80377448: usize = 0x8037_7448;
/// `0x80377908`: the `dScBoot_c` descriptor selected by `0x8015D7C0`.
const W95_G_80377908: usize = 0x8037_7908;
/// `0x802F5868` / `0x802F5898`: the two table bases returned by `0x801515C0`.
const W95_DAT_802F5868: u32 = 0x802F_5868;
const W95_DAT_802F5898: u32 = 0x802F_5898;
/// `0x802F58C8` / `58D8` / `58E8`: the three four-word placement rows of
/// `0x80151D00`.
const W95_DAT_802F58C8: usize = 0x802F_58C8;
const W95_DAT_802F58D8: usize = 0x802F_58D8;
const W95_DAT_802F58E8: usize = 0x802F_58E8;
/// `0x802F5918`: the sixteen-byte player-id table copied by `0x80156820`.
const W95_TABLE_802F5918: usize = 0x802F_5918;

/// `0x8042A358` (`r13 - 0x5328`): the boot screen's state-table anchor.
const W95_G_8042A358: usize = SDA_BASE - 0x5328;
/// `0x80429F10` (`r13 - 0x5770`): the display owner passed to `0x800A7540`.
// inferred: the export only forwards this global as the first queue argument.
const W95_G_80429F10: usize = SDA_BASE - 0x5770;
/// `0x8042A008` (`r13 - 0x5678`): the global owner table that
/// `0x800D8BB0` indexes by player.
const W95_G_8042A008: usize = SDA_BASE - 0x5678;
/// `0x8042A368` / `0x8042A36C` (`r13 - 0x5318` / `-0x5314`): the two words
/// cleared by `0x8015FE40`.
const W95_G_8042A368: usize = SDA_BASE - 0x5318;
const W95_G_8042A36C: usize = SDA_BASE - 0x5314;

/// `0x8042DA28` (`r2 - 0x5658`): the scene's horizontal scale constant.
const W95_SDA2_5658: usize = SDA2_BASE - 0x5658;
/// `0x8042DA18` (`r2 - 0x5668`): the scene's zero / origin constant.
const W95_SDA2_5668: usize = SDA2_BASE - 0x5668;
/// `0x8042DAC8` (`r2 - 0x55B8`): the matrix offset fed to `0x801C0BD0`.
const W95_SDA2_55B8: usize = SDA2_BASE - 0x55B8;
/// `0x8042DAA0` (`r2 - 0x55E0`): the second matrix offset of `0x80151C30`.
const W95_SDA2_55E0: usize = SDA2_BASE - 0x55E0;
/// `0x8042DA64` (`r2 - 0x561C`): the first matrix offset of `0x80151C30`.
const W95_SDA2_561C: usize = SDA2_BASE - 0x561C;
/// `0x8042DAC4` (`r2 - 0x55BC`): the default word copied by `0x801501B0`.
const W95_SDA2_55BC: usize = SDA2_BASE - 0x55BC;
/// `0x8042DA74` (`r2 - 0x560C`): the lower clamp of the scene rectangle.
const W95_SDA2_560C: usize = SDA2_BASE - 0x560C;
/// `0x8042DA70` (`r2 - 0x5610`): the upper clamp of the scene rectangle.
const W95_SDA2_5610: usize = SDA2_BASE - 0x5610;
/// `0x8042DAE4` (`r2 - 0x559C`): the placement row used when no model exists.
const W95_SDA2_559C: usize = SDA2_BASE - 0x559C;
/// `0x8042DBF8` (`r2 - 0x5488`): the comparison constant of `0x8015F140`.
const W95_SDA2_5488: usize = SDA2_BASE - 0x5488;

/// An 8-byte-aligned 16-byte scratch block (`auStack_78` of `0x80150540`).
#[repr(C, align(8))]
struct W95Buffer16([u8; 0x10]);

/// An 8-byte-aligned pair of floats (`local_18`/`local_14` of `0x80151AD0`).
#[repr(C, align(8))]
struct W95Pair8([u8; 8]);

/// The sixteen-byte copy of the `0x802F5918` table (`0x80156820`).
#[repr(C, align(8))]
struct W95Table16([u8; 0x10]);

/// Clears the sign bit of `value`, the `ABS()` of the PowerPC float helpers.
#[inline]
fn w95_abs_f32(value: f32) -> f32 {
    f32::from_bits(value.to_bits() & 0x7FFF_FFFF)
}

/// Calls `func(f64, u32, u32)` with no result.
#[inline]
unsafe fn w95_call_void_f64_u32_u32(addr: usize, a: f64, b: u32, c: u32) {
    let f: extern "C" fn(f64, u32, u32) = core::mem::transmute(addr);
    f(a, b, c);
}

// ---------------------------------------------------------------------------
// Wave 95, cluster A: the scene wrapper of `0x801501B0..0x80150830`
// ---------------------------------------------------------------------------

/// Resets the scene's pending display item.
///
/// In plain English: clear the `+0x1558` pending item, copy the small-data
/// default word at `r2 - 0x55BC` into `+0x155C`, and mark the `+0x1560` phase
/// word as `1`.
///
/// # Original
/// - Ghidra: `FUN_801501b0` at `0x801501B0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801501b0_FUN_801501b0.c`
///
/// # Arguments
/// * `scene` - the owning scene object; `+0x1558` pending item, `+0x155C`
///   default word, `+0x1560` phase.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three word writes into `scene`.
///
/// # Safety
/// `scene + 0x1558 .. +0x1564` must be writable.
///
/// # Confidence
/// medium - the three stores are literal; the value at `r2 - 0x55BC` is not
/// recovered, so it is read from its absolute address.
pub unsafe fn fn_801501b0(scene: usize) {
    write_word(scene, 0x1558, 0);
    write_word(scene, 0x155C, read_global_word(W95_SDA2_55BC));
    write_word(scene, 0x1560, 1);
}

/// Ends a layout change by switching the layout name and two channel ids.
///
/// In plain English: when the `0x80150650` "layout change finished" predicate
/// answers, install the `0x80376E88` layout name, register channels `0xB3` and
/// `3`, then remove channel `199`.
///
/// # Original
/// - Ghidra: `FUN_801504d0` at `0x801504D0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801504d0_FUN_801504d0.c`
///
/// # Arguments
/// * `scene` - the scene whose layout is being switched.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One layout-name write and three pad-channel updates.
///
/// # Safety
/// `scene` must be a live scene object.
///
/// # Confidence
/// medium - the predicate, the name and the three ids are literal; the export
/// prints `FUN_80150650()` without an operand, so its receiver is inferred to
/// be `scene`.
pub unsafe fn fn_801504d0(scene: usize) {
    // observed: FUN_80150650() with no printed operand; receiver inferred.
    if platform::call1(W95_FN_80150650, scene as u32) != 0 {
        let _ = platform::call2(W95_FN_8014ED00, scene as u32, W95_G_80376E88 as u32);
        let _ = platform::call2(W95_FN_80056CB0, scene as u32, 0xB3);
        let _ = platform::call2(W95_FN_80056CB0, scene as u32, 3);
        let _ = platform::call2(W95_FN_80056C70, scene as u32, 199);
    }
}

/// Reads the layout pane, rebuilds its placement matrix and queues a screen.
///
/// In plain English: when the layout change has finished, read pane `8` of the
/// `+0x14D8` manager into a 0x34-byte matrix, build a 0x30-byte translation
/// from the three SDA2 constants with `0x801C0BD0`, concatenate the two, and
/// extract the position triple. When `+0x1564 > 4` queue the `0x800A7540`
/// screen entry, otherwise when `+0x1568 > 0` queue `0x800A7550`; either path
/// clears its counter and returns `1`.
///
/// # Original
/// - Ghidra: `FUN_80150540` at `0x80150540` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150540_FUN_80150540.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x14D8` manager, `+0x04` player nibble,
///   `+0x348` mirror byte, `+0x38D` route byte, `+0x1564`/`+0x1568` counters.
///
/// # Returns
/// `1` when one of the two entries was queued, `0` otherwise.
///
/// # Side effects
/// One layout read, two matrix builds and at most one queue call.
///
/// # Safety
/// `scene` and the `+0x14D8` manager must be live; the manager's `+0x50` slot
/// must write at least 8 bytes into the matrix.
///
/// # Confidence
/// medium - the pane id `8`, the three SDA2 operands, the two thresholds and
/// the two queue calls are literal; the layout manager's vtable slot is
/// printed without a receiver and is passed as the manager.
pub unsafe fn fn_80150540(scene: usize) -> u32 {
    if platform::call1(W95_FN_80150650, scene as u32) == 0 {
        return 0;
    }

    let manager = read_word(scene, 0x14D8) as usize;
    let mut matrix = W93Matrix52([0u8; 0x34]);
    let mut offset = W93Matrix48([0u8; 0x30]);
    let mut out = W95Buffer16([0u8; 0x10]);

    // observed: (**(code **)(**(int **)(param_1 + 0x14d8) + 0x50))(manager, buf, 8)
    let vtable = read_word(manager, 0) as usize;
    let read = read_word(vtable, 0x50) as usize;
    let _ = platform::call3(read, manager as u32, matrix.0.as_mut_ptr() as u32, 8);

    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        read_const_f32(W95_SDA2_55B8) as f64,
        read_const_f32(W95_SDA2_5668) as f64,
        read_const_f32(W95_SDA2_5668) as f64,
        offset.0.as_mut_ptr() as u32,
    );
    let _ = platform::call3(
        W63_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        offset.0.as_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );
    let _ = platform::call2(
        W76_FN_8016F130,
        matrix.0.as_ptr() as u32,
        out.0.as_mut_ptr() as u32,
    );

    let player = read_word(scene, 4) & 0xF;
    let mirror = (read_byte(scene, 0x348) ^ 1) as u32;
    let route = (read_byte(scene, 0x38D) as i8) as i32 as u32;

    if read_word(scene, 0x1564) as i32 > 4 {
        let _ = platform::call5(
            W95_FN_800A7540,
            read_global_word(W95_G_80429F10),
            out.0.as_ptr() as u32,
            player,
            mirror,
            route,
        );
        write_word(scene, 0x1564, 0);
        return 1;
    }
    if read_word(scene, 0x1568) as i32 > 0 {
        let _ = platform::call5(
            W95_FN_800A7550,
            read_global_word(W95_G_80429F10),
            out.0.as_ptr() as u32,
            player,
            mirror,
            route,
        );
        write_word(scene, 0x1568, 0);
        return 1;
    }
    0
}

/// Forwards one callback into the owner's `+0x84` slot with re-entry set.
///
/// In plain English: when `target` is non-null, set `+0x1544` to `1`, call
/// `target`'s `+0x60` interface slot `+0x84` with `scene` as the argument, and
/// clear `+0x1544` again.
///
/// # Original
/// - Ghidra: `FUN_80150700` at `0x80150700` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150700_FUN_80150700.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x1544` is the re-entry word.
/// * `target` - the owner object whose `+0x60` interface is called.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two `+0x1544` writes and one virtual call.
///
/// # Safety
/// `target`, `target + 0x60` and its vtable must be readable.
///
/// # Confidence
/// high - the guard, both stores and the slot offset are literal.
pub unsafe fn fn_80150700(scene: usize, target: usize) {
    if target != 0 {
        write_word(scene, 0x1544, 1);
        let vtable = read_word(target, 0x60) as usize;
        let entry = read_word(vtable, 0x84) as usize;
        let _ = platform::call2(entry, target as u32, scene as u32);
        write_word(scene, 0x1544, 0);
    }
}

/// Forwards one callback into the owner's `+0x88` slot with re-entry set.
///
/// In plain English: the same guard as [`fn_80150700`] but for slot `+0x88`.
///
/// # Original
/// - Ghidra: `FUN_80150760` at `0x80150760` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150760_FUN_80150760.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x1544` is the re-entry word.
/// * `target` - the owner object whose `+0x60` interface is called.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two `+0x1544` writes and one virtual call.
///
/// # Safety
/// `target`, `target + 0x60` and its vtable must be readable.
///
/// # Confidence
/// high - the guard, both stores and the slot offset are literal.
pub unsafe fn fn_80150760(scene: usize, target: usize) {
    if target != 0 {
        write_word(scene, 0x1544, 1);
        let vtable = read_word(target, 0x60) as usize;
        let entry = read_word(vtable, 0x88) as usize;
        let _ = platform::call2(entry, target as u32, scene as u32);
        write_word(scene, 0x1544, 0);
    }
}

/// Runs the owner's `+0x90` slot and reports whether it answered non-zero.
///
/// In plain English: with the same `+0x1544` re-entry guard, call the `+0x90`
/// slot of `target`'s `+0x60` interface and return `1` when its result is
/// non-zero, `0` when it is zero or `target` is null.
///
/// # Original
/// - Ghidra: `FUN_80150830` at `0x80150830` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80150830_FUN_80150830.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x1544` is the re-entry word.
/// * `target` - the owner object whose `+0x60` interface is called.
///
/// # Returns
/// `1` when the slot answered non-zero, `0` otherwise.
///
/// # Side effects
/// Two `+0x1544` writes and one virtual call.
///
/// # Safety
/// `target`, `target + 0x60` and its vtable must be readable.
///
/// # Confidence
/// high - the guard, the slot and the `(-r | r) >> 31` normalisation are
/// literal.
pub unsafe fn fn_80150830(scene: usize, target: usize) -> u32 {
    let mut result = 0u32;
    if target != 0 {
        write_word(scene, 0x1544, 1);
        let vtable = read_word(target, 0x60) as usize;
        let entry = read_word(vtable, 0x90) as usize;
        result = platform::call2(entry, target as u32, scene as u32);
        write_word(scene, 0x1544, 0);
    }
    // observed: return (-uVar1 | uVar1) >> 31;  ->  1 when the result is non-zero.
    ((result.wrapping_neg() | result) >> 31) & 1
}

// ---------------------------------------------------------------------------
// Wave 95, cluster B: the scene state handlers of `0x80151320..0x80152B30`
// ---------------------------------------------------------------------------

/// Plays the pad effect of the current route when no owner is present.
///
/// In plain English: when the scene's `+0x60` slot `+0x3F0` reports no owner,
/// ask slot `+0x450` whether the incoming pair is accepted; if it is, play the
/// `0x800D8BB0` pad effect for the route byte at `+0x38D`. Always returns `0`,
/// so the caller keeps polling.
///
/// # Original
/// - Ghidra: `FUN_80151320` at `0x80151320` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151320_FUN_80151320.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x60` interface, `+0x38D` route byte.
/// * `first`, `second` - the pair forwarded to slot `+0x450`.
///
/// # Returns
/// Always `0`.
///
/// # Side effects
/// Two virtual calls and at most one pad-effect registration.
///
/// # Safety
/// `scene`, its `+0x60` interface and the `r13 - 0x5678` manager must be live.
///
/// # Confidence
/// medium - the guard, both slot offsets and the five effect arguments are
/// literal; the first vcall's receiver is dropped by the export and is inferred
/// to be `scene`.
pub unsafe fn fn_80151320(scene: usize, first: u32, second: u32) -> u32 {
    let vtable = read_word(scene, 0x60) as usize;
    let owner_query = read_word(vtable, 0x3F0) as usize;
    // observed: (**(code **)(...+0x3f0))() with no printed operand.
    if platform::call1(owner_query, scene as u32) == 0 {
        let route = (read_byte(scene, 0x38D) as i8) as i32 as u32;
        // observed: the export re-reads `*(int *)(param_1 + 0x60)` here.
        let vtable = read_word(scene, 0x60) as usize;
        let accept = read_word(vtable, 0x450) as usize;
        if platform::call3(accept, scene as u32, first, second) != 0 {
            let _ = platform::call5(
                W95_FN_800D8BB0,
                read_global_word(W95_G_8042A008),
                route,
                4,
                0,
                0,
            );
        }
    }
    0
}

/// Chooses one of the two placement tables from the manager's flag bits.
///
/// In plain English: read the `+0x14D8` manager's `+0x160` flag word and return
/// `0x802F5868` when bit 0 is clear, `0x802F5898` when it is set.
///
/// # Original
/// - Ghidra: `FUN_801515c0` at `0x801515C0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801515c0_FUN_801515c0.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x14D8` manager pointer.
///
/// # Returns
/// The address of one of the two four-word placement rows.
///
/// # Side effects
/// None.
///
/// # Safety
/// `scene` and its `+0x14D8` manager must be readable.
///
/// # Confidence
/// high - the bit test and the two table addresses are literal.
pub unsafe fn fn_801515c0(scene: usize) -> u32 {
    let manager = read_word(scene, 0x14D8) as usize;
    if read_word(manager, 0x160) & 1 == 0 {
        W95_DAT_802F5868
    } else {
        W95_DAT_802F5898
    }
}

/// Registers the two "scene leave" channels and latches the leave byte.
///
/// In plain English: register channels `0xE` and `0xA7`, then set the byte at
/// `+0x152D` to `1`.
///
/// # Original
/// - Ghidra: `FUN_80151a70` at `0x80151A70` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151a70_FUN_80151a70.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x152D` is the latched byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two pad-channel registrations and one byte store.
///
/// # Safety
/// `scene + 0x152D` must be writable.
///
/// # Confidence
/// high - both ids and the byte store are literal.
pub unsafe fn fn_80151a70(scene: usize) {
    let _ = platform::call2(W95_FN_80056CB0, scene as u32, 0xE);
    let _ = platform::call2(W95_FN_80056CB0, scene as u32, 0xA7);
    write_byte(scene, 0x152D, 1);
}

/// Removes the `0x7B` channel from the scene.
///
/// In plain English: one `0x80056C70(scene, 0x7B)` call.
///
/// # Original
/// - Ghidra: `FUN_80151ac0` at `0x80151AC0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151ac0_FUN_80151ac0.c`
///
/// # Arguments
/// * `scene` - the scene object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One pad-channel update.
///
/// # Safety
/// `scene` must be a live scene object.
///
/// # Confidence
/// high - the single call and its id are literal.
pub unsafe fn fn_80151ac0(scene: u32) {
    let _ = platform::call2(W95_FN_80056C70, scene, 0x7B);
}

/// Removes the `0x7C` channel from the scene.
///
/// In plain English: one `0x80056C70(scene, 0x7C)` call.
///
/// # Original
/// - Ghidra: `FUN_80151c20` at `0x80151C20` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151c20_FUN_80151c20.c`
///
/// # Arguments
/// * `scene` - the scene object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One pad-channel update.
///
/// # Safety
/// `scene` must be a live scene object.
///
/// # Confidence
/// high - the single call and its id are literal.
pub unsafe fn fn_80151c20(scene: u32) {
    let _ = platform::call2(W95_FN_80056C70, scene, 0x7C);
}

/// Rebuilds the two corner markers of the scene from panes `0x17` and `0x11`.
///
/// In plain English: read the two pane rectangles through
/// `0x800D5790(manager, out, 0x17)` and `0x800D5790(manager, out, 0x11)`, then
/// store the screen-space corner as
/// `((low + high) pair * scale - scene offset)`. Only when both spans pass the
/// SDA2 zero test is the clamped span written to `+0x122C/+0x1230`, the
/// `0x42` flag latched and `+0x1235` set to `0xF`.
///
/// # Original
/// - Ghidra: `FUN_80151ad0` at `0x80151AD0` (324 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151ad0_FUN_80151ad0.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x14D8` manager, `+0xAC`/`+0xB0` offsets,
///   `+0x1224..+0x1238` marker record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two pane reads and up to four float stores plus one flag word and one byte.
///
/// # Safety
/// `scene` and its `+0x14D8` manager must be live; both pane rectangles must
/// exist.
///
/// # Confidence
/// medium - the float algebra and the store order are literal; the scale, zero
/// and clamp constants live in SDA2 and are read from their absolute addresses,
/// so their values are not recovered.
pub unsafe fn fn_80151ad0(scene: usize) {
    let manager = read_word(scene, 0x14D8) as usize;
    let mut low = W95Pair8([0u8; 8]); // local_18 / local_14
    let mut high = W95Pair8([0u8; 8]); // local_24 / local_20

    let _ = platform::call3(
        W95_FN_800D5790,
        manager as u32,
        low.0.as_mut_ptr() as u32,
        0x17,
    );
    let _ = platform::call3(
        W95_FN_800D5790,
        manager as u32,
        high.0.as_mut_ptr() as u32,
        0x11,
    );

    let low_x = read_float(low.0.as_ptr() as usize, 0);
    let low_y = read_float(low.0.as_ptr() as usize, 4);
    let high_x = read_float(high.0.as_ptr() as usize, 0);
    let high_y = read_float(high.0.as_ptr() as usize, 4);
    let scale = read_const_f32(W95_SDA2_5658);
    let zero = read_const_f32(W95_SDA2_5668);

    write_float(
        scene,
        0x1228,
        (low_y + high_y) * scale - read_float(scene, 0xB0),
    );
    write_float(
        scene,
        0x1224,
        (low_x + high_x) * scale - read_float(scene, 0xAC),
    );

    let span_x = w95_abs_f32(low_x - high_x) * scale;
    if zero != span_x {
        let mut span_y = w95_abs_f32(low_y - high_y) * scale;
        if zero != span_y {
            let floor = read_const_f32(W95_SDA2_560C);
            if span_y < floor {
                span_y = floor;
            }
            let base = read_const_f32(W95_SDA2_5610);
            write_float(scene, 0x1230, span_y);
            write_float(scene, 0x122C, base + span_x);
            write_word(scene, 0x1238, read_word(scene, 0x1238) | 0x42);
            write_byte(scene, 0x1235, 0x0F);
        }
    }
}

/// Places the single "route marker" from pane `0x11` and a matrix build.
///
/// In plain English: read pane `0x11` into a matrix, build the translation from
/// the two SDA2 offsets and the zero constant, concatenate and extract the
/// position, then store `x - +0xAC`, `y - +0xB0` and the SDA2 word at
/// `r2 - 0x5610` into both `+0x122C` and `+0x1230`. Latches the `0x42` flag and
/// sets `+0x1235` to `0x10`.
///
/// # Original
/// - Ghidra: `FUN_80151c30` at `0x80151C30` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151c30_FUN_80151c30.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x14D8` manager, `+0xAC`/`+0xB0` offsets,
///   `+0x1224..+0x1238` marker record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One pane read, two matrix calls, two float stores, one flag word and one
/// byte.
///
/// # Safety
/// `scene` and its `+0x14D8` manager must be live; pane `0x11` must exist.
///
/// # Confidence
/// medium - every offset and call is literal; the three SDA2 float operands are
/// read from their absolute addresses instead of being guessed.
pub unsafe fn fn_80151c30(scene: usize) {
    let manager = read_word(scene, 0x14D8) as usize;
    let mut matrix = W93Matrix52([0u8; 0x34]);
    let mut offset = W93Matrix48([0u8; 0x30]);
    let mut out = W95Pair8([0u8; 8]);

    let vtable = read_word(manager, 0) as usize;
    let read = read_word(vtable, 0x50) as usize;
    let _ = platform::call3(read, manager as u32, matrix.0.as_mut_ptr() as u32, 0x11);

    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        read_const_f32(W95_SDA2_561C) as f64,
        read_const_f32(W95_SDA2_55E0) as f64,
        read_const_f32(W95_SDA2_5668) as f64,
        offset.0.as_mut_ptr() as u32,
    );
    let _ = platform::call3(
        W63_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        offset.0.as_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );
    let _ = platform::call2(
        W76_FN_8016F130,
        matrix.0.as_ptr() as u32,
        out.0.as_mut_ptr() as u32,
    );

    let shared = read_global_word(W95_SDA2_5610);
    let x = read_float(out.0.as_ptr() as usize, 0);
    let y = read_float(out.0.as_ptr() as usize, 4);
    write_float(scene, 0x1224, x - read_float(scene, 0xAC));
    write_float(scene, 0x1228, y - read_float(scene, 0xB0));
    write_word(scene, 0x122C, shared);
    write_word(scene, 0x1230, shared);
    write_word(scene, 0x1238, read_word(scene, 0x1238) | 0x42);
    write_byte(scene, 0x1235, 0x10);
}

/// Chooses the scene's placement row, model id and spawn record.
///
/// In plain English: pick one of the three four-word rows `0x802F58C8`,
/// `0x802F58D8` or `0x802F58E8` from the manager's flag bits, copy the row into
/// `+0x164..+0x170`, set the three `+0xD0..+0xD8` words and scale `+0xD4` by
/// `+0x1030`. When `+0x1524` is still empty, resolve the model through
/// `+0x1464` and compare it against `0x80376AC8` to store model id `0x109` or
/// `0x10D`, then replace the row with the fallback placement.
///
/// # Original
/// - Ghidra: `FUN_80151d00` at `0x80151D00` (324 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151d00_FUN_80151d00.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x14D8` manager, `+0x1464` model source,
///   `+0x1524` owner, `+0x10D4` flag word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// At most one model comparison and up to twelve word/float stores.
///
/// # Safety
/// `scene`, its `+0x14D8` manager and its `+0x1464` model source must be live.
///
/// # Confidence
/// medium - the three rows, both flag tests, the model ids and the store order
/// are literal; the SDA2 words are read from their absolute addresses and the
/// `+0x1464` slot `+0x28` receiver is dropped by the export, so it is inferred
/// to be the `+0x1464` object.
pub unsafe fn fn_80151d00(scene: usize) {
    let manager = read_word(scene, 0x14D8) as usize;
    let flags = read_word(manager, 0x160);
    let row = if flags & 1 != 0 {
        W95_DAT_802F58D8
    } else if flags & 0x10 != 0 {
        W95_DAT_802F58E8
    } else {
        W95_DAT_802F58C8
    };

    write_word(scene, 0x164, read_global_word(row));
    write_word(scene, 0x168, read_global_word(row + 4));
    let scale = read_const_f32(W95_SDA2_5658);
    let zero = read_global_word(W95_SDA2_5668);
    write_word(scene, 0x16C, read_global_word(row + 8));
    write_word(scene, 0x170, read_global_word(row + 0xC));
    write_word(scene, 0xD0, zero);
    write_float(scene, 0xD4, read_float(scene, 0x1030) * scale);
    write_word(scene, 0xD8, zero);

    if read_word(scene, 0x1524) != 0 {
        return;
    }

    // observed: (**(code **)(*(int *)(param_1 + 0x1464) + 0x28))() -- receiver
    // dropped by the export, inferred to be the +0x1464 object.
    let source = read_word(scene, 0x1464) as usize;
    let source_vtable = read_word(source, 0) as usize;
    let resolve = read_word(source_vtable, 0x28) as usize;
    let model = platform::call1(resolve, source as u32) as usize;

    let model_vtable = read_word(model, 0) as usize;
    let compare = read_word(model_vtable, 0x10) as usize;
    if platform::call2(compare, model as u32, W95_G_80376AC8 as u32) == 0 {
        write_word(scene, 0x178, 0x109);
    } else {
        write_word(scene, 0x178, 0x10D);
    }

    let zero = read_global_word(W95_SDA2_5668);
    let second = read_global_word(W95_SDA2_559C);
    write_byte(scene, 0x175, 0);
    write_word(scene, 0x17C, 0x0040_0000);
    write_word(scene, 0x164, zero);
    write_word(scene, 0x168, second);

    let upper = read_global_word(W95_SDA2_5610);
    if read_word(scene, 0x10D4) & 0x1800_0000 == 0 {
        write_word(scene, 0x16C, read_global_word(W95_SDA2_560C));
        write_word(scene, 0x170, upper);
    } else {
        write_word(scene, 0x16C, upper);
        write_word(scene, 0x170, upper);
    }

    let zero = read_global_word(W95_SDA2_5668);
    write_word(scene, 0xD0, zero);
    write_word(scene, 0xD4, read_global_word(W95_SDA2_560C));
    write_word(scene, 0xD8, zero);
}

/// Applies the pair contact of two actors and reports whether it was handled.
///
/// In plain English: with `first`'s `+0x04` actor and `second`'s `+0x04` actor,
/// apply the contact only while the first actor's `+0xEC` height is at or below
/// the SDA2 origin, the `0x4D` channel is free, the `4` state is free and
/// `0x80053C80` accepts the pair. Then, when the second actor's `10` channel is
/// set and its height is above the origin, apply the impulse and clear the
/// contact state, otherwise fall back to `0x80049C70`.
///
/// # Original
/// - Ghidra: `FUN_80152020` at `0x80152020` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152020_FUN_80152020.c`
///
/// # Arguments
/// * `first` - first actor holder; `+0x04` actor, `+0x70` impulse scalar.
/// * `second` - second actor holder; `+0x04` actor.
///
/// # Returns
/// `1` when one of the two contact paths fired, `0` otherwise.
///
/// # Side effects
/// Up to five queries, one impulse call and one contact reset.
///
/// # Safety
/// Both holders and their `+0x04` actors must be live.
///
/// # Confidence
/// medium - the four guards, the impulse and the fallback are literal; the SDA2
/// height threshold is read from its absolute address, and the roles of the
/// `0x4D`/`10` channel ids are inferred from their use.
pub unsafe fn fn_80152020(first: usize, second: usize) -> u32 {
    let actor_a = read_word(first, 4) as usize;
    let actor_b = read_word(second, 4) as usize;
    let origin = read_const_f32(W95_SDA2_5668);

    if read_float(actor_a, 0xEC) <= origin
        && platform::call2(W95_FN_80056CF0, actor_a as u32, 0x4D) == 0
        && platform::call2(W95_FN_8004DB40, actor_a as u32, 4) == 0
        && platform::call3(W95_FN_80053C80, actor_a as u32, first as u32, second as u32) != 0
    {
        if platform::call2(W95_FN_80056CF0, actor_b as u32, 10) != 0
            && origin < read_float(actor_b, 0xEC)
        {
            w95_call_void_f64_u32_u32(
                W95_FN_80053E60,
                read_float(first, 0x70) as f64,
                actor_a as u32,
                1,
            );
            let _ = platform::call1(W95_FN_80053DF0, actor_b as u32);
            return 1;
        }
        if platform::call2(W95_FN_80049C70, actor_a as u32, actor_b as u32) != 0 {
            return 1;
        }
    }
    0
}

/// Forwards the current view's `+0x120` slot when the scene has an owner.
///
/// In plain English: return `0` when `+0x1524` is empty or `0x8014E9B0`
/// resolves no view, otherwise call that view's `+0x60` slot `+0x120` and return
/// its result.
///
/// # Original
/// - Ghidra: `FUN_801527a0` at `0x801527A0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801527a0_FUN_801527a0.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x1524` owner flag.
///
/// # Returns
/// The virtual slot's result, or `0` when the path is not taken.
///
/// # Side effects
/// One view lookup and one virtual call.
///
/// # Safety
/// `scene` must be live and the resolved view's `+0x60` interface readable.
///
/// # Confidence
/// medium - the guards and the slot offset are literal; the export prints
/// `FUN_8014e9b0()` and the final vcall without operands, so both receivers are
/// inferred from the sibling `0x80152B30`, which passes `scene` explicitly.
pub unsafe fn fn_801527a0(scene: usize) -> u32 {
    if read_word(scene, 0x1524) == 0 {
        return 0;
    }
    let view = platform::call1(W95_FN_8014E9B0, scene as u32);
    if view == 0 {
        return 0;
    }
    let vtable = read_word(view as usize, 0x60) as usize;
    let entry = read_word(vtable, 0x120) as usize;
    platform::call1(entry, view)
}

/// Requests the scene transition from the current scene and player views.
///
/// In plain English: when `0x80050D80` reports a mode, the scene does not hold
/// channel `0x8D` and `+0x1524` is set, forward `arg` to the scene resolved from
/// `+0x1520` and to the player view from `0x8014E9B0`. Only the second forward
/// (the player view, whose `+0x38C` byte is `1`) triggers the scene's `+0x294`
/// transition and makes the body return `1`.
///
/// # Original
/// - Ghidra: `FUN_80152b30` at `0x80152B30` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152b30_FUN_80152b30.c`
///
/// # Arguments
/// * `scene` - the scene object; `+0x1520` scene id, `+0x1524` owner, `+0x60`
///   interface.
/// * `arg` - the forwarded event argument.
///
/// # Returns
/// `1` when the transition was requested, `0` otherwise.
///
/// # Side effects
/// Up to two event forwards and one transition call.
///
/// # Safety
/// `scene` must be live; the resolved scene and view objects must be readable.
///
/// # Confidence
/// medium - the guards, the byte tests and the slot offsets are literal; the
/// `+0x38C` byte roles are inferred from the sibling handlers.
pub unsafe fn fn_80152b30(scene: usize, arg: u32) -> u32 {
    let mode = call0_r(W95_FN_80050D80);
    if mode != 0
        && platform::call2(W95_FN_80056CF0, scene as u32, 0x8D) == 0
        && read_word(scene, 0x1524) != 0
    {
        let owner = platform::call1(W95_FN_80162D00, read_word(scene, 0x1520));
        if owner != 0 && read_byte(owner as usize, 0x38C) == 1 {
            let _ = platform::call2(W95_FN_801391F0, owner, arg);
        }

        // observed: FUN_8014e9b0(param_1) -- the receiver is explicit here.
        let view = platform::call1(W95_FN_8014E9B0, scene as u32);
        if view != 0
            && read_byte(view as usize, 0x38C) == 1
            && platform::call2(W95_FN_801391F0, view, arg) != 0
        {
            let vtable = read_word(scene, 0x60) as usize;
            let entry = read_word(vtable, 0x294) as usize;
            let _ = platform::call3(entry, scene as u32, W95_G_80376B48 as u32, 0);
            return 1;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// Wave 95, cluster C: the allocation thunk, the release helper and the
// player-bit resolver of `0x801533A0..0x80156D70`
// ---------------------------------------------------------------------------

/// Registers a second-level allocator over two objects.
///
/// In plain English: forward `(second, second, first)` to `0x802DCBFC`, the
/// three-operand registration that every state-object constructor thunk in this
/// band uses.
///
/// # Original
/// - Ghidra: `FUN_801533a0` at `0x801533A0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801533a0_FUN_801533a0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - the second object, forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the same thunk shape as [`fn_8015dd70`]; `interface_call` models the
/// `r12` descriptor.
pub unsafe fn fn_801533a0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Releases a state object and optionally frees it.
///
/// In plain English: when `object` is non-null, run the `0x8015F800` destructor
/// on it and, when `free > 0`, free the storage with `0x802B9280`; return the
/// object pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_80154300` at `0x80154300` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154300_FUN_80154300.c`
///
/// # Arguments
/// * `object` - the state object to release.
/// * `free` - positive to free the storage after destruction.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// One destructor call and, when `free > 0`, one free.
///
/// # Safety
/// `object` must be a state object built by the matching constructor.
///
/// # Confidence
/// high - the null guard, the destructor, the sign test and the returned
/// pointer are literal. `0x80154360` and `0x8015F040` are the same body.
pub unsafe fn fn_80154300(object: usize, free: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Resolves the `index`-th connected player's id.
///
/// In plain English: take the four-bit connected-player mask from
/// `0x8005FC70`, copy the sixteen-byte id table at `0x802F5918` onto the stack,
/// then walk the mask bits in order; the `index`-th set bit maps to the byte at
/// offset `1`, `2`, `4` or `8` of that table, which is resolved through
/// `0x8005F900` into a player object.
///
/// # Original
/// - Ghidra: `FUN_80156820` at `0x80156820` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156820_FUN_80156820.c`
///
/// # Arguments
/// * `_owner` - printed as `param_1` and never read by the body.
/// * `index` - zero-based index among the connected players.
///
/// # Returns
/// The resolved player object, or `0` when `index` is out of range.
///
/// # Side effects
/// One mask query and one id lookup.
///
/// # Safety
/// The table at `0x802F5918` must be readable for sixteen bytes.
///
/// # Confidence
/// medium - the mask walk, the byte offsets and the fallback are literal; the
/// table is copied word-wise and then read at byte offsets `1/2/4/8`, so the
/// entry layout is `// unknown:` and is reproduced byte for byte.
pub unsafe fn fn_80156820(_owner: u32, index: i32) -> u32 {
    let mask = call0_r(FN_8005FC70);
    let mut table = W95Table16([0u8; 0x10]);
    for offset in [0usize, 4, 8, 0xC] {
        write_word(
            table.0.as_mut_ptr() as usize,
            offset,
            read_global_word(W95_TABLE_802F5918 + offset),
        );
    }

    let mut bit = 1u32;
    let mut seen = 0i32;
    let mut remaining = 4;
    loop {
        if mask & 0xFF & bit != 0 {
            if index == seen {
                let id = read_byte(table.0.as_ptr() as usize, bit as usize) as u32;
                return platform::call1(FN_8005F900, id);
            }
            seen += 1;
        }
        bit = (bit & 0x7F) << 1;
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    0
}

/// Reports whether every connected player finished the scene's phase `4`.
///
/// In plain English: count the connected players whose `+0x153C` byte is `4`
/// and return `count == connected` in `r3`.
///
/// # Original
/// - Ghidra: `FUN_80156d70` at `0x80156D70` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156d70_FUN_80156d70.c`
/// - Removed: the `bl 0x802DCD74` (`__savegprlr_27`) prologue and the
///   `bl 0x802DCDC0` (`__restgprlr_27`) epilogue; the prologue only saved the
///   incoming `r3`, now the `owner` parameter, and the epilogue only carried
///   the result in `r3`.
///
/// # Arguments
/// * `owner` - incoming `r3`, forwarded to `0x80156820`, whose first parameter
///   the export never reads.
///
/// # Returns
/// `1` when every connected player's phase byte is `4`, `0` otherwise.
///
/// # Side effects
/// Four player lookups and one result hand-off.
///
/// # Safety
/// Every resolved player must be readable at `+0x153C`.
///
/// # Confidence
/// medium - the four-slot loop, the phase byte and the comparison are literal;
/// the incoming `r3` is only forwarded to `0x80156820`, whose first parameter
/// the export never reads, so its role stays `// unknown:`.
pub unsafe fn fn_80156d70(owner: u32) -> u32 {
    let connected = call0_r(W95_FN_801568D0) as i32;

    let mut matching = 0i32;
    let mut index = 0i32;
    loop {
        let player = platform::call2(W95_FN_80156820, owner, index as u32);
        if player != 0 && read_byte(player as usize, 0x153C) == 4 {
            matching += 1;
        }
        index += 1;
        if index >= 4 {
            break;
        }
    }
    (matching == connected) as u32
}

// ---------------------------------------------------------------------------
// Wave 95, cluster D: the boot/cursor helpers of `0x8015C4C0..0x8015FE40`
// ---------------------------------------------------------------------------

/// Turns the boot screen's state `2` into the game-start handshake.
///
/// In plain English: when the `+0x5328` table reports state `2`, release the
/// live layout, clear the `+0x7710` ready byte, enable both transition
/// channels, require resources `0x2C0` and `0x2E5` to be resident and commit
/// mode `0`. Any missing piece returns `0`.
///
/// # Original
/// - Ghidra: `FUN_8015c4c0` at `0x8015C4C0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c4c0_FUN_8015c4c0.c`
///
/// # Arguments
/// * `arg` - forwarded to the state-table query.
///
/// # Returns
/// `1` when the start handshake completed, `0` otherwise.
///
/// # Side effects
/// Two layout calls, one byte clear, two channel updates, two resource queries
/// and one mode commit.
///
/// # Safety
/// The `r13 - 0x5328` table and the layout objects must be live.
///
/// # Confidence
/// high - the state test, the two resource ids and the mode value are literal.
pub unsafe fn fn_8015c4c0(arg: u32) -> u32 {
    if platform::call2(W95_FN_8015F620, W95_G_8042A358 as u32, arg) != 2 {
        return 0;
    }
    let _ = call0_r(W78_FN_80107F10);
    let _ = call0_r(W95_FN_80108150);
    write_global_byte(W90_G_BOOT_BYTE_7710, 0);
    let _ = platform::call1(W95_FN_800E1F50, 0x1E);
    let _ = platform::call1(W95_FN_800E1F60, 0x1E);
    if platform::call1(W95_FN_8008F610, 0x2C0) == 0 {
        return 0;
    }
    if platform::call1(W95_FN_8008F610, 0x2E5) == 0 {
        return 0;
    }
    let _ = platform::call1(W95_FN_800E5060, 0);
    1
}

/// Fades the boot screen out once the layout reports mode `1`.
///
/// In plain English: when the live layout's `+0x28` word is `1`, set bit `0x40`
/// in the display manager's `+0x1D8` flag word and change the `+0xB0` interface
/// to the `0x80377448` fade-out descriptor.
///
/// # Original
/// - Ghidra: `FUN_8015c680` at `0x8015C680` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015c680_FUN_8015c680.c`
///
/// # Arguments
/// * `boot` - the boot screen object; `+0xB0` is its state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One display flag update and one state change.
///
/// # Safety
/// `boot`, its `+0xB0` interface and the live layout/display objects must be
/// valid.
///
/// # Confidence
/// high - the mode test, the bit mask, the descriptor and the slot are literal.
pub unsafe fn fn_8015c680(boot: usize) {
    let layout = call0_r(W78_FN_80107F10) as usize;
    if read_word(layout, 0x28) == 1 {
        let display = call0_r(W78_FN_80109340) as usize;
        write_word(display, 0x1D8, read_word(display, 0x1D8) | 0x40);
        let iface = boot + 0xB0;
        let vtable = read_word(iface, 0) as usize;
        let entry = read_word(vtable, 0x18) as usize;
        let _ = platform::call2(entry, iface as u32, W95_G_80377448 as u32);
    }
}

/// Leaves the boot screen when the view object reports event `0x900`.
///
/// In plain English: ask the `r13 - 0x5218` view object (the wave-78
/// [`W78_G_VIEW_OBJECT`]) whether event `0x900` is pending; when it is, or when
/// bit `0x00080000` of the global display flags is set, mark the live layout as
/// `1` through `0x80108160` and change the `+0x74` interface to the `0x80377908`
/// descriptor.
///
/// # Original
/// - Ghidra: `FUN_8015d7c0` at `0x8015D7C0` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015d7c0_FUN_8015d7c0.c`
///
/// # Arguments
/// * `boot` - the boot screen object; `+0x74` is its state interface.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One event query, one layout flag write and one state change.
///
/// # Safety
/// The view object, the layout and `boot + 0x74` must be live.
///
/// # Confidence
/// medium - the event id, the flag mask and the descriptor are literal; the
/// view object's `+0x20` slot is printed without a receiver and is passed the
/// object itself, and the "pending" sense of its result is inferred from the
/// wave-78 caller.
pub unsafe fn fn_8015d7c0(boot: usize) {
    let manager = read_global_word(W78_G_VIEW_OBJECT) as usize;
    let vtable = read_word(manager, 0) as usize;
    let query = read_word(vtable, 0x20) as usize;
    let pending = platform::call2(query, manager as u32, 0x900);

    if pending != 0 || read_global_word(G_DISPLAY_INPUT_FLAGS) & 0x0008_0000 != 0 {
        let layout = call0_r(W78_FN_80107F10);
        let _ = platform::call2(W78_FN_80108160, layout, 1);
        let iface = boot + 0x74;
        let iface_vtable = read_word(iface, 0) as usize;
        let entry = read_word(iface_vtable, 0x18) as usize;
        let _ = platform::call2(entry, iface as u32, W95_G_80377908 as u32);
    }
}

/// Clears the two boot-screen list anchors.
///
/// In plain English: store `0` at `r13 - 0x5318` and `r13 - 0x5314`.
///
/// # Original
/// - Ghidra: `FUN_8015fe40` at `0x8015FE40` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015fe40_FUN_8015fe40.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two global word writes.
///
/// # Safety
/// Both SDA words must be writable.
///
/// # Confidence
/// high - the two stores and their anchors are literal.
pub unsafe fn fn_8015fe40() {
    write_global_word(W95_G_8042A368, 0);
    write_global_word(W95_G_8042A36C, 0);
}

/// Steps one `f32` toward a target with rate, limit and dead-band.
///
/// In plain English: when `*value` differs from `target`, compute
/// `step = rate * (target - *value)` truncated to `f32`. Outside the dead-band
/// the step is clamped to `±limit` and added; inside the dead-band the value
/// snaps to `*value ∓ deadband` and is clamped to `target` when it overshoots.
/// The body returns the signed remaining distance.
///
/// # Original
/// - Ghidra: `FUN_8015f140` at `0x8015F140` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f140_FUN_8015f140.c`
///
/// # Arguments
/// * `target` - destination value.
/// * `rate` - per-call gain applied to the remaining distance.
/// * `limit` - maximum step outside the dead-band.
/// * `deadband` - distance below which the value snaps to the band edge.
/// * `value` - the live `f32` being stepped; read and written.
///
/// # Returns
/// The signed distance from `*value` to `target` after the step.
///
/// # Side effects
/// Up to three writes through `value`.
///
/// # Safety
/// `value` must point at a writable `f32`.
///
/// # Confidence
/// medium - every branch, the truncations to `f32` and the final signed
/// distance are literal; the comparison constant at `r2 - 0x5488` is read from
/// its absolute address, so its value is not recovered.
pub unsafe fn fn_8015f140(
    target: f64,
    rate: f64,
    limit: f64,
    deadband: f64,
    value: *mut f32,
) -> f64 {
    let zero = read_const_f32(W95_SDA2_5488) as f64;
    let current = *value as f64;
    if current != target {
        // observed: dVar3 = (double)(float)(param_2 * (double)(float)(param_1 - dVar2))
        let mut step = ((rate * ((target - current) as f32 as f64)) as f32) as f64;
        if deadband <= step || step <= -deadband {
            if limit < step {
                step = limit;
            }
            if step < -limit {
                step = -limit;
            }
            *value = (*value as f64 + step) as f32;
        } else if step <= zero {
            let edge = -deadband;
            if edge < step {
                *value = (current + edge) as f32;
                if (*value as f64) < target {
                    *value = target as f32;
                }
            }
        } else if step < deadband {
            *value = (current + deadband) as f32;
            if target < (*value as f64) {
                *value = target as f32;
            }
        }
    }

    let delta = ((target - *value as f64) as f32) as f64;
    if zero < delta {
        delta
    } else {
        -delta
    }
}

// ---------------------------------------------------------------------------
// Wave 99: the actor teardown, the display-value/star-sprite helpers, the
// `dStateMng` selector family, the command-packet builder, the ground-collision
// filters and the `dRemoconMng_c` thunk/descriptor cluster of
// `0x800D16B0..0x800DFCE0`.
//
// This is the same `0x800Dxxxx` engine band that waves 16, 20, 63, 76, 89, 90
// and 93 cover. The objects keep a vtable word at `+0x00`; the state handlers
// index a 0x24-byte record table at `0x802F1350` and mirror the selected row
// into `+0x154/+0x158/+0x15C/+0x160/+0x164`, exactly like the `0x800D6940`
// family wave 93 documented. `0x800DD2B0` is the sixth registration bank of the
// band and uses the same `0x8015F7C0` + `0x802DC780` shape as
// [`fn_800d7c80`] and [`fn_800de510`].
// ---------------------------------------------------------------------------

/// `0x800D3CB0(actor)`: returns the animation node the selectors drive.
const W99_FN_800D3CB0: usize = 0x800D_3CB0;

/// `0x800D5170(actor)`: spawns/updates the star-sprite pair.
const W99_FN_800D5170: usize = 0x800D_5170;

/// `0x800D5790(actor, out, mode)`: builds the tail transform vector.
const W99_FN_800D5790: usize = 0x800D_5790;

/// `0x800D6460(payload, actor)`: picks the effect record from the state flags.
const W99_FN_800D6460: usize = 0x800D_6460;

/// `0x800D6550(payload, actor)`: single-row effect dispatch.
const W99_FN_800D6550: usize = 0x800D_6550;

/// `0x800D6A30(actor)`: second teardown step of [`fn_800d38a0`].
const W99_FN_800D6A30: usize = 0x800D_6A30;

/// `0x800DFC70(record, value)`: submits one five-word interface record.
const W99_FN_800DFC70: usize = 0x800D_FC70;

/// `0x800046B4(dst, value, len)`: byte fill used by [`fn_800d9760`].
const W99_FN_800046B4: usize = 0x8000_46B4;

/// `0x80064AF0(position, mode, flag)`: player-aware position adjuster.
const W99_FN_80064AF0: usize = 0x8006_4AF0;

/// `0x8008FC80(player, layout, name, 0, matrix, 0, colour)`.
const W99_FN_8008FC80: usize = 0x8008_FC80;

/// `0x80161400(value) -> f64`: duration helper of the `+0x174` timer.
const W99_FN_80161400: usize = 0x8016_1400;

/// `0x80166E80(node, 1)`: clears the node's animation window.
const W99_FN_80166E80: usize = 0x8016_6E80;

/// `0x80166F30(value, node, 1) -> u32`: second animation-value query.
const W99_FN_80166F30: usize = 0x8016_6F30;

/// `0x8016ECB0(matrix, angles)`: post-multiplies an X rotation.
const W99_FN_8016ECB0: usize = 0x8016_ECB0;

/// `0x8016ED10(scale, matrix, angles)`: builds a rotation from scale + angles.
const W99_FN_8016ED10: usize = 0x8016_ED10;

/// `0x8016EEF0(matrix, angles)`: post-multiplies a Z rotation.
const W99_FN_8016EEF0: usize = 0x8016_EEF0;

/// `0x801B5F80() -> u64`: header word pair of the command packet.
const W99_FN_801B5F80: usize = 0x801B_5F80;

/// `0x801B8620(buffer, len)`: checksums/flushes the packet.
const W99_FN_801B8620: usize = 0x801B_8620;

/// `0x801DD6E0(record)`: constructor of the 24-byte packet header.
const W99_FN_801DD6E0: usize = 0x801D_D6E0;

/// `0x802BC8C0(out)`: reads the view's two position floats.
const W99_FN_802BC8C0: usize = 0x802B_C8C0;

/// `0x802BC8E0(view) -> f32`: reads the view's third position float.
const W99_FN_802BC8E0: usize = 0x802B_C8E0;

/// `0x802BE470(value) -> f64`: square-root helper.
const W99_FN_802BE470: usize = 0x802B_E470;

/// `0x802F1350`: 0x24-byte state-selector records (six entries).
const W99_TABLE_802F1350: usize = 0x802F_1350;

/// `0x802F1360`: word field `+0x10` of each selector record.
const W99_TABLE_802F1360: usize = 0x802F_1360;

/// `0x802F1354`: `Rwait` name pointer field of each selector record.
const W99_TABLE_802F1354: usize = 0x802F_1354;

/// `0x802F136C`: word field `+0x1C` of each selector record.
const W99_TABLE_802F136C: usize = 0x802F_136C;

/// `0x802F1370`: word field `+0x20` of each selector record.
const W99_TABLE_802F1370: usize = 0x802F_1370;

/// `0x802F2C38`: two effect-word rows selected by the `+0x160` flag bit.
const W99_TABLE_802F2C38: usize = 0x802F_2C38;

/// `0x80370DA4`: `f32` added to the marker height when `+0x17C` bit 10 is set.
const W99_F32_80370DA4: usize = 0x8037_0DA4;

/// `0x80370E68`: `f32` argument of the `+0x5C` dispatch in [`fn_800d5dd0`].
const W99_F32_80370E68: usize = 0x8037_0E68;

/// `0x80317D0C`: `s_Wm_2d_mrstarkira` name passed to the layout spawn.
const W99_STR_80317D0C: u32 = 0x8031_7D0C;

/// `0x80317D20`: `s_Wm_mr_startail` name passed to the tail spawn.
const W99_STR_80317D20: u32 = 0x8031_7D20;

/// `0x80319290`: vtable of the two `dConnect_c::State` descriptors.
const W99_VTABLE_80319290: u32 = 0x8031_9290;

/// `0x8031925C`: vtable of the four `dConnect_c::dExte*` descriptors.
const W99_VTABLE_8031925C: u32 = 0x8031_925C;

/// `0x800DD770(object, free)`: destructor registered by the first two states.
const W99_HANDLER_800DD770: u32 = 0x800D_D770;

/// `0x800DD7D0(object, free)`: destructor registered by the four extension
/// states; translated below as [`fn_800dd7d0`].
const W99_HANDLER_800DD7D0: u32 = 0x800D_D7D0;

/// `r2 - 0x664C`: `f32` default selector scale.
const W99_F32_664C: usize = SDA2_BASE - 0x664C;
/// `r2 - 0x6648`: `f32` fade-out scale of the `+0x174` timer.
const W99_F32_6648: usize = SDA2_BASE - 0x6648;
/// `r2 - 0x6644`: `f32` duration base of the `+0x174` timer.
const W99_F32_6644: usize = SDA2_BASE - 0x6644;
/// `r2 - 0x6640`: `f32` duration input of the `0x80161400` helper.
const W99_F32_6640: usize = SDA2_BASE - 0x6640;
/// `r2 - 0x661C` / `-0x6618`: `f32` scale of route byte `0` / `3`.
const W99_F32_661C: usize = SDA2_BASE - 0x661C;
const W99_F32_6618: usize = SDA2_BASE - 0x6618;
/// `r2 - 0x6608`: `f32` zero used by the whole selector family.
const W99_F32_6608: usize = SDA2_BASE - 0x6608;
/// `r2 - 0x6604`: `f32` one-frame step subtracted from the animation span.
const W99_F32_6604: usize = SDA2_BASE - 0x6604;
/// `r2 - 0x6600`: `f32` per-word axis scale of the marker transform.
const W99_F32_6600: usize = SDA2_BASE - 0x6600;
/// `r2 - 0x65FC`: `f32` per-row y scale of state `0x7F`.
const W99_F32_65FC: usize = SDA2_BASE - 0x65FC;
/// `r2 - 0x65F8`: `f32` height of the `0x34..0x37` states.
const W99_F32_65F8: usize = SDA2_BASE - 0x65F8;
/// `r2 - 0x65F4`: `f32` y offset of the `0x82/0x83` states.
const W99_F32_65F4: usize = SDA2_BASE - 0x65F4;
/// `r2 - 0x65F0`: `f32` y offset of every other state.
const W99_F32_65F0: usize = SDA2_BASE - 0x65F0;
/// `r2 - 0x65C8`: `f32` first operand of the `0x800D6460` dispatch.
const W99_F32_65C8: usize = SDA2_BASE - 0x65C8;
/// `r2 - 0x65D0`: two selector words used by the `0x800D6460` dispatch.
const W99_F32_65D0: usize = SDA2_BASE - 0x65D0;
/// `r2 - 0x6448`: `f32` epsilon of the collision-depth clamp.
const W99_F32_6448: usize = SDA2_BASE - 0x6448;
/// `r2 - 0x6430`: `f32` collision-height tolerance.
const W99_F32_6430: usize = SDA2_BASE - 0x6430;
/// `r2 - 0x65D8`: `f64` magic constant of the `u16 -> double` conversion.
const W99_F64_65D8: usize = SDA2_BASE - 0x65D8;
/// `r13 - 0x79B8`: two layout-name handles indexed by the route byte.
const W99_G_79B8: usize = SDA_BASE - 0x79B8;

/// An 8-byte-aligned twelve-word (0x30-byte) matrix scratch block.
#[repr(C, align(8))]
struct W99Matrix48([u32; 12]);

/// An 8-byte-aligned 0x80-byte command-packet buffer.
#[repr(C, align(8))]
struct W99Packet128([u8; 0x80]);

/// Calls `func() -> f32`.
#[inline]
unsafe fn w99_call0_f32(addr: usize) -> f32 {
    let f: extern "C" fn() -> f32 = core::mem::transmute(addr);
    f()
}

/// Calls `func(u32) -> f32`.
#[inline]
unsafe fn w99_call1_f32_u32(addr: usize, a: u32) -> f32 {
    let f: extern "C" fn(u32) -> f32 = core::mem::transmute(addr);
    f(a)
}

/// Calls `func() -> u64`.
#[inline]
unsafe fn w99_call0_u64(addr: usize) -> u64 {
    let f: extern "C" fn() -> u64 = core::mem::transmute(addr);
    f()
}

/// Reads the big-endian `f64` at an absolute address.
#[inline]
unsafe fn w99_read_f64(addr: usize) -> f64 {
    let high = platform::read_u32(addr) as u64;
    let low = platform::read_u32(addr + 4) as u64;
    f64::from_bits((high << 32) | low)
}

/// Calls `func(f64, u32, u32) -> u32`.
#[inline]
unsafe fn w99_call_f64_u32_u32(addr: usize, a: f64, b: u32, c: u32) -> u32 {
    let f: extern "C" fn(f64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c)
}

/// Calls `func(f64, u32) -> u32`.
#[inline]
unsafe fn w99_call_f64_u32(addr: usize, a: f64, b: u32) -> u32 {
    let f: extern "C" fn(f64, u32) -> u32 = core::mem::transmute(addr);
    f(a, b)
}

/// Calls `func(u64, u32, u32) -> u32`.
#[inline]
unsafe fn w99_call_u64_u32_u32(addr: usize, a: u64, b: u32, c: u32) -> u32 {
    let f: extern "C" fn(u64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c)
}

/// Calls `func(u64, u32) -> u32`.
#[inline]
unsafe fn w99_call_u64_u32(addr: usize, a: u64, b: u32) -> u32 {
    let f: extern "C" fn(u64, u32) -> u32 = core::mem::transmute(addr);
    f(a, b)
}

/// Calls `func(u64, u64, u64, u32) -> u32`.
#[inline]
unsafe fn w99_call3_u64_u32(addr: usize, a: u64, b: u64, c: u64, d: u32) -> u32 {
    let f: extern "C" fn(u64, u64, u64, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d)
}

/// Calls `func(f64, u32, u32) -> u32` where the first integer is the receiver.
#[inline]
unsafe fn w99_call_f64_ptr_ptr(addr: usize, a: f64, b: u32, c: u32) -> u32 {
    let f: extern "C" fn(f64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c)
}

/// Calls `func(u32, u32, u32)` with no result (three pointers).
#[inline]
unsafe fn w99_call3_ptr(addr: usize, a: u32, b: u32, c: u32) {
    let f: extern "C" fn(u32, u32, u32) = core::mem::transmute(addr);
    f(a, b, c);
}

/// Calls `func(f64, f64, f64, u32, u32) -> u32`.
#[inline]
unsafe fn w99_call_f64x3_u32x2(addr: usize, a: f64, b: f64, c: f64, d: u32, e: u32) -> u32 {
    let f: extern "C" fn(f64, f64, f64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d, e)
}

/// Calls `func(f64, f64, f64, u32, u32, u32) -> u32`.
#[inline]
unsafe fn w99_call_f64x3_u32x3(
    addr: usize,
    a: f64,
    b: f64,
    c: f64,
    d: u32,
    e: u32,
    f: u32,
) -> u32 {
    let g: extern "C" fn(f64, f64, f64, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    g(a, b, c, d, e, f)
}

/// Calls `func(f64, u64, f64, u32, u32) -> u32`.
#[inline]
unsafe fn w99_call_f64_u64_f64_u32_u32(
    addr: usize,
    a: f64,
    b: u64,
    c: f64,
    d: u32,
    e: u32,
) -> u32 {
    let f: extern "C" fn(f64, u64, f64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, d, e)
}

/// Reads the vtable entry `slot` of the object at `this` and returns its
/// address. The vtable pointer lives at `this + 0`.
#[inline]
unsafe fn w99_slot(this: usize, slot: usize) -> usize {
    let vtable = read_word(this, 0) as usize;
    read_word(vtable, slot) as usize
}

/// Reads the vtable entry `slot` of the interface embedded at `this + field`
/// and calls it with `this + field` as the receiver.
#[inline]
unsafe fn w99_field_call(this: usize, field: usize, slot: usize) -> u32 {
    let iface = this + field;
    let vtable = read_word(iface, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    platform::call1(entry, iface as u32)
}

/// Converts a `u16` to `f64` through the observed `0x43300000` magic pair.
///
/// observed: every selector body builds
/// `CONCAT44(0x43300000, value ^ 0x80000000)` and subtracts the `f64` at
/// `r2 - 0x65D8`; the XOR folds the sign bit into the mantissa so the result is
/// the unsigned halfword.
#[inline]
unsafe fn w99_u16_to_f64(value: u16) -> f64 {
    let bits = (0x4330_0000u64 << 32) | ((value as u32 ^ 0x8000_0000) as u64);
    f64::from_bits(bits) - w99_read_f64(W99_F64_65D8)
}

/// Reads the nine handler words of one 0x24-byte descriptor source record.
///
/// observed: the export copies the record in the order `+0x18, +0x1C, +0x20,
/// +0x0C, +0x10, +0x14, +0x00, +0x04, +0x08` into `+0x0C..+0x2C` of the state
/// block, so the destination is a permuted copy of the source record.
#[inline]
unsafe fn w99_descriptor_fields(base: usize) -> [u32; 9] {
    [
        read_global_word(base + 0x18),
        read_global_word(base + 0x1C),
        read_global_word(base + 0x20),
        read_global_word(base + 0x0C),
        read_global_word(base + 0x10),
        read_global_word(base + 0x14),
        read_global_word(base + 0x00),
        read_global_word(base + 0x04),
        read_global_word(base + 0x08),
    ]
}

/// Forwards a parameterless tail call to `0x802DCBFC`.
///
/// In plain English: the export is a single `0x802DCBFC()` call. The wave-20
/// call sites ([`cursor_acquire_target`]) pass `(cursor, target, arg)` and
/// consume `r3`, so this address is the same three-operand registration the
/// neighbouring thunks use, but the callee body does not print its operands.
///
/// # Original
/// - Ghidra: `FUN_800d16b0` at `0x800D16B0` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d16b0_FUN_800d16b0.c`
///
/// # Arguments
/// None in the export. `// unknown:` the three operands the call sites pass.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `0x802DCBFC` must accept the call sites' operands; the export does not show
/// them.
///
/// # Confidence
/// medium - the call target and the empty body are literal; the operands are
/// dropped by the decompiler and are documented in [`FN_800D16B0`].
pub unsafe fn fn_800d16b0() {
    let _ = platform::call0(FN_802DCBFC);
}

/// Tears down one actor and updates its two timers.
///
/// In plain English: run `0x800D5170` and `0x800D6A30` on the actor, release
/// the four embedded interfaces at `+0x28`, `+0x60`, `+0x744` and `+0x694`
/// through their own slot `+0x14`, release the current animation record and the
/// variant record selected by `+0x770`, then run the `+0x170/+0x174` frame
/// timer and subtract the `+0x782` halfword from the `+0x780` counter.
///
/// # Original
/// - Ghidra: `FUN_800d38a0` at `0x800D38A0` (388 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d38a0_FUN_800d38a0.c`
///
/// # Arguments
/// * `this` - the actor; `+0x170` timer state, `+0x174` frame counter,
///   `+0x770` variant index, `+0x780/+0x782` halfword pair.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Six interface releases, two animation releases, two timer updates and one
/// halfword subtraction.
///
/// # Safety
/// `this` must be a live actor of at least `0x784` bytes; every embedded
/// interface vtable and `+0x770` variant record must be valid.
///
/// # Confidence
/// medium - every call, offset and slot is literal; the two `+0x1C` record
/// calls print no operands, so their receiver is `// inferred:` from the object
/// whose vtable was read.
pub unsafe fn fn_800d38a0(this: usize) {
    // observed: FUN_800d5170() prints no operand; // inferred: this actor.
    let _ = platform::call1(W99_FN_800D5170, this as u32);
    let _ = platform::call1(W99_FN_800D6A30, this as u32);

    let _ = w99_field_call(this, 0x28, 0x14);
    let _ = w99_field_call(this, 0x60, 0x14);
    let _ = w99_field_call(this, 0x744, 0x14);
    let _ = w99_field_call(this, 0x694, 0x14);

    let record = vcall0(this, 0x28) as usize;
    let vtable = read_word(record, 0) as usize;
    let entry = read_word(vtable, 0x1C) as usize;
    let _ = platform::call1(entry, record as u32);

    let variant = read_word(this, 0x770) as usize;
    let variant_record = this + variant * 0x80 + 0x4A8;
    let vtable = read_word(variant_record, 0) as usize;
    let entry = read_word(vtable, 0x1C) as usize;
    let _ = platform::call1(entry, variant_record as u32);

    if read_word(this, 0x170) == 1 {
        if read_word(this, 0x174) == 0 {
            let node = platform::call1(W99_FN_800D3CB0, this as u32);
            w95_call_void_f64_u32_u32(
                FN_80166F10,
                read_const_f32(W99_F32_664C) as f64,
                node,
                1,
            );
            let node = platform::call1(W99_FN_800D3CB0, this as u32);
            if call_f64_u32_u32(
                W99_FN_80166F30,
                read_const_f32(W99_F32_6648) as f64,
                node,
                1,
            ) != 0
            {
                let duration =
                    call_f64_ret_f64(W99_FN_80161400, read_const_f32(W99_F32_6640) as f64);
                let frames = (read_const_f32(W99_F32_6644) as f64 + duration) as i32;
                write_word(this, 0x174, frames as u32);
            }
        } else {
            let node = platform::call1(W99_FN_800D3CB0, this as u32);
            w95_call_void_f64_u32_u32(
                FN_80166F10,
                read_const_f32(W99_F32_6648) as f64,
                node,
                1,
            );
            write_word(this, 0x174, read_word(this, 0x174).wrapping_sub(1));
        }
    }

    let node = platform::call1(W99_FN_800D3CB0, this as u32);
    let _ = platform::call2(W99_FN_80166E80, node, 1);
    let _ = w99_field_call(this, 0x668, 0x14);

    let counter = read_half(this, 0x780);
    let step = read_half(this, 0x782);
    write_half(this, 0x780, counter.wrapping_sub(step));
}

/// Refreshes the interpolated `+0x1E2..+0x1E4` display values.
///
/// In plain English: read the current record's `+0x28` scale and the value the
/// actor's `+0xB8` slot reports. When the scale equals the SDA2 default, all
/// three words take the new value; otherwise the previous value is shifted into
/// `+0x1E4` and `+0x1E2` becomes the linear blend between the two samples.
///
/// # Original
/// - Ghidra: `FUN_800d4cd0` at `0x800D4CD0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d4cd0_FUN_800d4cd0.c`
///
/// # Arguments
/// * `this` - the display object; `+0x1E2/+0x1E3/+0x1E4` are the value triple.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to three word stores on `this`.
///
/// # Safety
/// `this` and its `+0x28` record must be live.
///
/// # Confidence
/// medium - every guard and arithmetic step is literal; the export prints the
/// `+0x28` slot without a receiver and wraps the body in paired-single
/// `ldexpf` stubs that are no-ops here.
pub unsafe fn fn_800d4cd0(this: usize) {
    let record = vcall0(this, 0x28) as usize;
    let target = read_float(record, 0x28) as f64;
    let value = platform::call1(w99_slot(this, 0xB8), this as u32) as f32 as f64;
    let zero = read_const_f32(W99_F32_664C) as f64;

    if zero == target {
        let rounded = value as i32;
        write_word(this, 0x790, rounded as u32);
        write_word(this, 0x78C, rounded as u32);
        write_word(this, 0x788, rounded as u32);
    } else {
        if (read_word(this, 0x78C) as i32 as f32) as f64 != value {
            write_word(this, 0x790, read_word(this, 0x78C));
            write_word(this, 0x78C, (value as f32) as i32 as u32);
        }
        let previous = read_word(this, 0x790) as i32 as f32;
        let blended = ((value * target) as f32) + previous * ((zero - target) as f32);
        write_word(this, 0x788, (blended as i32) as u32);
    }
}

/// Spawns the star-sprite and tail effects of one marker.
///
/// In plain English: when `+0x17C` bit `0x200` is set, pick the scale of the
/// current route byte (`0`, `3` or the default), scale the `+0x144..+0x14C`
/// triple, read the actor's `+0xD8` transform into a 12-byte vector, and then
/// dispatch on the `+0x180` mode: `0` spawns through `0x8008FC80` with one of
/// the two `r13 - 0x79B8` names, `1` spawns through the `+0x218` interface slot
/// `+0xA0`, `2` through `0x8008FBA0`. Mode `0` additionally spawns the
/// `Wm_mr_startail` effect from the `+0x340` interface.
///
/// # Original
/// - Ghidra: `FUN_800d5170` at `0x800D5170` (444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5170_FUN_800d5170.c`
///
/// # Arguments
/// * `this` - the marker; `+0x152` route byte, `+0x180` spawn mode,
///   `+0x218`/`+0x340` layout interfaces, `+0x144..+0x14C` colour triple.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to two model spawns and one transform decomposition.
///
/// # Safety
/// `this`, its two interfaces and the name tables must be live; the layout
/// spawns require the global player/layout state to be initialised.
///
/// # Confidence
/// medium - every branch, offset and name is literal; the `+0xA0` virtual call
/// prints its receiver and six operands but the meaning of the two `auStack`
/// blocks is `// inferred:` from `0x8016F130`'s wave-76 use.
pub unsafe fn fn_800d5170(this: usize) {
    if read_word(this, 0x17C) & 0x200 == 0 {
        return;
    }

    let route = read_byte(this, 0x152);
    let mut scale = read_const_f32(W99_F32_664C) as f64;
    if route == 0 {
        scale = read_const_f32(W99_F32_661C) as f64;
    } else if route == 3 {
        scale = read_const_f32(W99_F32_6618) as f64;
    }

    let mut vector = [0u32; 3];
    let mut colour = [0f32; 3];
    colour[2] = (read_float(this, 0x14C) as f64 * scale) as f32;
    colour[1] = (read_float(this, 0x148) as f64 * scale) as f32;
    colour[0] = (read_float(this, 0x144) as f64 * scale) as f32;
    let _ = platform::call2(
        W93_FN_8016F130,
        (this + 0xD8) as u32,
        vector.as_mut_ptr() as u32,
    );

    let mode = read_word(this, 0x180) as i32;
    if mode == 0 {
        let index = if route == 0 || route == 3 { 1 } else { 0 };
        let _ = platform::call7(
            W99_FN_8008FC80,
            read_byte(this, 0x150) as u32,
            (this + 0x218) as u32,
            read_global_word(W99_G_79B8 + index * 4),
            0,
            vector.as_mut_ptr() as u32,
            0,
            (this + 0x144) as u32,
        );
    } else if mode == 1 {
        let iface = this + 0x218;
        let vtable = read_word(iface, 0) as usize;
        let entry = read_word(vtable, 0xA0) as usize;
        let _ = platform::call6(
            entry,
            iface as u32,
            W99_STR_80317D0C,
            0,
            vector.as_mut_ptr() as u32,
            0,
            colour.as_mut_ptr() as u32,
        );
    } else if mode == 2 {
        let _ = platform::call7(
            W76_FN_8008FBA0,
            read_byte(this, 0x150) as u32,
            (this + 0x218) as u32,
            W99_STR_80317D0C,
            0,
            vector.as_mut_ptr() as u32,
            0,
            colour.as_mut_ptr() as u32,
        );
    }

    if read_word(this, 0x180) == 0 {
        let mut tail = [0u32; 3];
        let mut tail_colour = [0f32; 3];
        tail_colour[2] = (read_float(this, 0x14C) as f64 * scale) as f32;
        tail_colour[1] = (read_float(this, 0x148) as f64 * scale) as f32;
        tail_colour[0] = (read_float(this, 0x144) as f64 * scale) as f32;
        let _ = platform::call3(
            W99_FN_800D5790,
            this as u32,
            tail.as_mut_ptr() as u32,
            8,
        );
        let _ = platform::call7(
            W76_FN_8008FBA0,
            read_byte(this, 0x150) as u32,
            (this + 0x340) as u32,
            W99_STR_80317D20,
            0,
            tail.as_mut_ptr() as u32,
            0,
            tail_colour.as_mut_ptr() as u32,
        );
    }
}

/// Rebuilds the marker's orientation and position matrices.
///
/// In plain English: when the `+0x180` mode is non-zero, copy the input scale
/// into `+0x144..+0x14C`; then choose the state-dependent height and y offset,
/// compose the X/Z rotations from the input angle pair around the marker's
/// translation, apply the input scale, and store the result at `+0x108`.
/// Finally offset the input position by `+0x184` (plus the `0x80370DA4`
/// constant when bit `0x400` is set), let `0x80064AF0` adjust it in mode `0`,
/// and store that matrix at `+0xD8`.
///
/// # Original
/// - Ghidra: `FUN_800d57f0` at `0x800D57F0` (908 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d57f0_FUN_800d57f0.c`
///
/// # Arguments
/// * `this` - the marker; `+0x154` state id, `+0x17C` flags, `+0x180` mode,
///   `+0x184` height offset.
/// * `position` - three input position floats.
/// * `angles` - two input angles; `angles[0]` is rotated about X and
///   `angles[1]` about Z.
/// * `scale` - three input scale factors.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve-word writes at `+0x108` and `+0xD8`, seven matrix calls and up to
/// three copies into `+0x144..+0x14C`.
///
/// # Safety
/// `this` must be a live marker of at least `0x188` bytes and the three input
/// pointers must point at two/four readable floats.
///
/// # Confidence
/// medium - the composition order, the state table and every store are
/// literal; the meaning of the SDA2 floats (height, per-word scale, one-frame
/// step) is `// inferred:` from the state ids, and the paired-single `ldexpf`
/// stubs at the head and tail are no-ops.
pub unsafe fn fn_800d57f0(
    this: usize,
    position: *mut f32,
    angles: *mut i16,
    scale: *mut f32,
) {
    if read_word(this, 0x180) != 0 {
        write_float(this, 0x144, *scale);
        write_float(this, 0x148, *scale.add(1));
        write_float(this, 0x14C, *scale.add(2));
    }

    let state = read_word(this, 0x154) as i32;
    let zero = read_const_f32(W99_F32_6608) as f64;
    let axis_scale = read_const_f32(W99_F32_6600) as f64;
    let mut y_offset = axis_scale * (*scale.add(1) as f64);
    let mut height = zero;
    let mut use_zero_offset = true;

    if (state.wrapping_sub(0x2B) as u32) > 3 {
        if (state.wrapping_sub(0x34) as u32) < 4 {
            height = read_const_f32(W99_F32_65F8) as f64;
            use_zero_offset = false;
        } else if state != 0x7E {
            if state == 0x7F {
                y_offset = read_const_f32(W99_F32_65FC) as f64 * (*scale.add(1) as f64);
            }
            use_zero_offset = false;
        }
    }
    if use_zero_offset {
        y_offset = zero;
    }

    let mut matrix = W99Matrix48([0u32; 12]);
    let mut scratch = W99Matrix48([0u32; 12]);
    let mut x_angle: i16 = *angles.add(1);
    let _ = w99_call_f64_ptr_ptr(
        W99_FN_8016ED10,
        axis_scale,
        matrix.0.as_mut_ptr() as u32,
        &mut x_angle as *mut i16 as u32,
    );

    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        zero,
        y_offset,
        height,
        scratch.0.as_mut_ptr() as u32,
    );
    w99_call3_ptr(
        W93_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        scratch.0.as_mut_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );

    let mut negative_angle: i16 = (-(*angles as i32)) as i16;
    let _ = platform::call2(
        W99_FN_8016ECB0,
        matrix.0.as_mut_ptr() as u32,
        &mut negative_angle as *mut i16 as u32,
    );
    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        zero,
        -y_offset,
        zero,
        scratch.0.as_mut_ptr() as u32,
    );
    w99_call3_ptr(
        W93_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        scratch.0.as_mut_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );

    let z_offset = ((axis_scale * (*scale.add(1) as f64)) as f32) as f64;
    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        zero,
        z_offset,
        zero,
        scratch.0.as_mut_ptr() as u32,
    );
    w99_call3_ptr(
        W93_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        scratch.0.as_mut_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );

    let mut z_angle: i16 = *angles.add(2);
    let _ = platform::call2(
        W99_FN_8016EEF0,
        matrix.0.as_mut_ptr() as u32,
        &mut z_angle as *mut i16 as u32,
    );
    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        zero,
        -z_offset,
        zero,
        scratch.0.as_mut_ptr() as u32,
    );
    w99_call3_ptr(
        W93_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        scratch.0.as_mut_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );

    call_f64_f64_f64_ptr(
        W63_FN_801C0C60,
        *scale as f64,
        *scale.add(1) as f64,
        *scale.add(2) as f64,
        scratch.0.as_mut_ptr() as u32,
    );
    w99_call3_ptr(
        W93_FN_801C0540,
        matrix.0.as_mut_ptr() as u32,
        scratch.0.as_mut_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
    );

    let mut index = 0;
    while index < 12 {
        write_word(this, 0x108 + index * 4, matrix.0[index]);
        index += 1;
    }

    let mut point = [*position, *position.add(1), *position.add(2)];
    if (state.wrapping_sub(0x82) as u32) < 2 {
        point[1] += read_const_f32(W99_F32_65F4);
    } else if state == 0x85 {
        point[1] += read_const_f32(W99_F32_6604);
    } else {
        point[1] -= read_const_f32(W99_F32_65F0);
    }
    point[1] += read_float(this, 0x184);
    if read_word(this, 0x17C) & 0x400 != 0 {
        point[1] += read_const_f32(W99_F32_80370DA4);
    }
    if read_word(this, 0x180) == 0 {
        let _ = platform::call3(W99_FN_80064AF0, point.as_mut_ptr() as u32, 0, 1);
    }

    let mut out = W99Matrix48([0u32; 12]);
    call_f64_f64_f64_ptr(
        W93_FN_801C0BD0,
        point[0] as f64,
        point[1] as f64,
        point[2] as f64,
        out.0.as_mut_ptr() as u32,
    );
    w99_call3_ptr(
        W93_FN_801C0540,
        out.0.as_mut_ptr() as u32,
        matrix.0.as_mut_ptr() as u32,
        out.0.as_mut_ptr() as u32,
    );

    index = 0;
    while index < 12 {
        write_word(this, 0xD8 + index * 4, out.0[index]);
        index += 1;
    }
}

/// Changes the marker's state and runs the per-state effect dispatch.
///
/// In plain English: when the route byte is `3` and the requested mode is `6`,
/// dispatch `0x5C` with the `0x80370E68` float instead of changing state.
/// Otherwise remember the previous mode, clear the pending mode `+0x158`, store
/// the new mode at `+0x154`, and either replay the already-active row (writing
/// its id byte into `+0x5C`) or install the new row: its `+0x20` word into
/// `+0x160/+0x164`, resolve the row name through `0x8023A0B0`, clamp the speed
/// to the animation span, and run the `+0x70/+0x74` effect calls plus one of
/// the three tail paths.
///
/// # Original
/// - Ghidra: `FUN_800d5dd0` at `0x800D5DD0` (1040 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d5dd0_FUN_800d5dd0.c`
///
/// # Arguments
/// * `first` - first `f64` operand forwarded to the effect calls.
/// * `payload` - the 64-bit operand forwarded to the effect calls; the export
///   types it `undefined8` and never interprets it.
/// * `speed` - requested animation speed.
/// * `this` - the marker; `+0x152` route byte, `+0x154` mode, `+0x158` pending
///   mode, `+0x160/+0x164` row timers, `+0x17C` flags.
/// * `mode` - the selector index, stride `0x24` into `0x802F1350`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to ten virtual calls, one name resolution and five word stores.
///
/// # Safety
/// `this` must be a live marker whose vtable implements slots `+0x5C` through
/// `+0x100`, and the `0x802F1350` record table must be readable.
///
/// # Confidence
/// medium - the state table, the timer clamp, the byte/word stores and every
/// branch are literal; the three `+0x70/+0x74/+0x100` calls print a raw 64-bit
/// middle operand, so whether it is a double in an FPR pair or two GPR words is
/// `// inferred:` from the `0x800D6940` family, and the paired-single stubs are
/// no-ops.
pub unsafe fn fn_800d5dd0(
    first: f64,
    payload: u64,
    speed: f64,
    this: usize,
    mode: i32,
) {
    let zero = read_const_f32(W99_F32_6608) as f64;

    if read_byte(this, 0x152) == 3 && mode == 6 {
        let entry = w99_slot(this, 0x5C);
        let _ = w99_call_f64_u32_u32(
            entry,
            read_const_f32(W99_F32_80370E68) as f64,
            this as u32,
            0xD,
        );
        return;
    }

    let previous = read_word(this, 0x154) as i32;
    let index = mode as usize * 0x24;
    write_word(this, 0x15C, 0xFFFF_FFFF);
    write_word(this, 0x154, mode as u32);

    if previous == mode {
        let id = read_global_word(W99_TABLE_802F1360 + index) as u8;
        write_byte(this, 0x5C, id);
        let entry = w99_slot(this, 0x8C);
        let _ = w99_call_f64_u32(entry, first, this as u32);
        if read_const_f32(W99_F32_6608) as f64 != speed {
            let entry = w99_slot(this, 0x84);
            let _ = w99_call_f64_u32(entry, speed, this as u32);
        }
        return;
    }

    let timer = read_global_word(W99_TABLE_802F1370 + index);
    write_word(this, 0x164, timer);
    write_word(this, 0x160, timer);

    let manager = vcall0(this, 0x2C);
    let mut handle = platform::call2(
        W93_FN_8023A0B0,
        manager,
        read_global_word(W99_TABLE_802F1350 + index),
    );
    let handle_ptr = &mut handle as *mut u32 as u32;
    let entry = w99_slot(this, 0x60);
    let _ = platform::call4(entry, this as u32, read_word(this, 0x154), handle_ptr, 0);

    let mut current = speed;
    if current == zero {
        if first < zero {
            let raw = read_half(handle as usize, 0x20);
            current = (w99_u16_to_f64(raw) as f32 - read_const_f32(W99_F32_6604)) as f64;
        }
        if read_word(this, 0x160) & 0x40 != 0
            && read_global_word(W99_TABLE_802F1370 + previous as usize * 0x24) & 0x40 != 0
        {
            current = w99_call1_f32_u32(W93_FN_80168080, (this + 0x28) as u32) as f64;
        }
    }

    let span = w99_u16_to_f64(read_half(handle as usize, 0x20)) as f32;
    if span as f64 <= current {
        current = (span - read_const_f32(W99_F32_6604)) as f64;
    }

    if read_word(this, 0x17C) & 6 != 0 {
        if read_word(this, 0x17C) & 4 != 0 {
            fn_800d69b0(
                first.to_bits(),
                payload,
                current.to_bits(),
                this,
            );
        }
        if read_word(this, 0x164) & 2 == 0 {
            let entry = w99_slot(this, 0x70);
            let _ = w99_call_f64x3_u32x3(
                entry,
                first,
                current,
                f64::from_bits(payload),
                this as u32,
                handle,
                read_global_word(W99_TABLE_802F1360 + index),
            );
            let _ = fn_800d6460(f64::from_bits(payload), this);
            return;
        }
    }

    let flags = read_word(this, 0x17C);
    if flags & 8 == 0 && flags & 0x10 == 0
        || read_word(this, 0x158) == 0xFFFF_FFFF
        || read_word(this, 0x164) & 0x20 != 0
    {
        if read_word(this, 0x164) & 0x1000 == 0 {
            let entry = w99_slot(this, 0x70);
            let _ = w99_call_f64x3_u32x3(
                entry,
                first,
                current,
                f64::from_bits(payload),
                this as u32,
                handle,
                read_global_word(W99_TABLE_802F1360 + index),
            );
            write_word(this, 0x158, 0xFFFF_FFFF);
            let entry = w99_slot(this, 0x74);
            let _ = w99_call_f64x3_u32x3(
                entry,
                first,
                current,
                f64::from_bits(payload),
                this as u32,
                handle,
                read_global_word(W99_TABLE_802F1360 + index),
            );
            let entry = w99_slot(this, 0x78);
            let _ = platform::call1(entry, this as u32);
            let entry = w99_slot(this, 0x80);
            let _ = platform::call2(
                entry,
                this as u32,
                read_global_word(W99_TABLE_802F136C + index),
            );
        } else {
            let entry = w99_slot(this, 0x70);
            let _ = w99_call_f64x3_u32x3(
                entry,
                first,
                current,
                f64::from_bits(payload),
                this as u32,
                handle,
                read_global_word(W99_TABLE_802F1360 + index),
            );
            let _ = w99_call_u64_u32(W99_FN_800D6550, payload, this as u32);
        }
    } else {
        let entry = w99_slot(this, 0x70);
        let _ = w99_call_f64x3_u32x3(
            entry,
            first,
            current,
            zero,
            this as u32,
            handle,
            read_global_word(W99_TABLE_802F1360 + index),
        );
        let word_a = call_u64_ret(W93_FN_80168080, (this + 0x18) as u32);
        let word_b = call_u64_ret(W93_FN_801680C0, (this + 0x18) as u32);
        let entry = w99_slot(this, 0x64);
        let _ = w99_call_f64x3_u32x2(
            entry,
            f64::from_bits(word_b),
            f64::from_bits(word_a),
            zero,
            this as u32,
            read_word(this, 0x158),
        );
    }
}

/// Dispatches one effect row from the `+0x160` flags.
///
/// In plain English: select bit 2 of `+0x17C`, read the `+0x160` flag word and
/// call the `0x64` slot with the fixed `0x80370E68`/zero float pair and either
/// the plain selector word from `r2 - 0x65D0` or, when the `0xC000` bits are
/// set, the actor's `+0x28` transform word and one of the two
/// `0x802F2C38` effect rows.
///
/// # Original
/// - Ghidra: `FUN_800d6460` at `0x800D6460` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6460_FUN_800d6460.c`
///
/// # Arguments
/// * `payload` - the forwarded 64-bit operand of [`fn_800d5dd0`].
/// * `this` - the marker; `+0x17C` flags and `+0x28` transform.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One vtable call through slot `+0x64`.
///
/// # Safety
/// `this` and its vtable must be live.
///
/// # Confidence
/// medium - the masks, the two selector tables and the call are literal; the
/// export prints the middle operand as `undefined8`, so its `f64` typing is
/// `// inferred:` from the `+0x70` family, and the paired-single stubs are
/// no-ops.
pub unsafe fn fn_800d6460(payload: f64, this: usize) {
    let selector = if read_word(this, 0x17C) & 4 != 0 { 1 } else { 0 };
    let flags = read_word(this, 0x160);
    let entry = w99_slot(this, 0x64);

    if flags & 0xC000 == 0 {
        let _ = w99_call_f64x3_u32x2(
            entry,
            read_const_f32(W99_F32_65C8) as f64,
            read_const_f32(W99_F32_6608) as f64,
            payload,
            this as u32,
            read_global_word(W99_F32_65D0 + selector * 4),
        );
    } else {
        let transform = call_u64_ret(W93_FN_80168080, (this + 0x28) as u32);
        let row = read_global_word(
            W99_TABLE_802F2C38 + selector * 4 + ((flags & 0x4000 != 0) as usize) * 8,
        );
        let _ = w99_call_f64_u64_f64_u32_u32(
            entry,
            read_const_f32(W99_F32_65C8) as f64,
            transform,
            payload,
            this as u32,
            row,
        );
    }
}

/// Switches the marker to `mode` and starts its row.
///
/// In plain English: store `-1` into `+0x154`, store `mode` into `+0x158`, and
/// when the previous mode already equals `mode` replay the row's id byte and
/// its `+0x8C`/`+0x84` calls; otherwise clear both timer words, resolve the
/// row's `Rwait` name through `0x8023A0B0`, clamp the speed to the row's
/// halfword span, and run the three `+0x70/+0x74/+0x80` calls.
///
/// # Original
/// - Ghidra: `FUN_800d65e0` at `0x800D65E0` (580 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d65e0_FUN_800d65e0.c`
///
/// # Arguments
/// * `first` - left operand of the speed comparison.
/// * `payload` - forwarded 64-bit operand.
/// * `requested` - requested speed; zero selects the row's span.
/// * `this` - the marker; `+0x154` active mode, `+0x158` pending mode,
///   `+0x160/+0x164` timers.
/// * `mode` - the row index, stride `0x24` into `0x802F1354`/`0x802F1360`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Five word stores and up to six virtual calls.
///
/// # Safety
/// `this` and the `0x802F1350` record table must be live.
///
/// # Confidence
/// medium - the two row branches, the span clamp and the `u16 -> double`
/// conversion are literal; the bare `+0x2C`/`+0x8C` calls print no receiver and
/// are `// inferred:` to take `this`, and the paired-single stubs are no-ops.
pub unsafe fn fn_800d65e0(
    first: f64,
    payload: u64,
    requested: f64,
    this: usize,
    mode: i32,
) {
    let index = mode as usize * 0x24;
    let previous = read_word(this, 0x154) as i32;
    write_word(this, 0x154, 0xFFFF_FFFF);
    write_word(this, 0x158, mode as u32);

    if previous == mode {
        let id = read_global_word(W99_TABLE_802F1360 + index) as u8;
        write_byte(this, 0x5C, id);
        let entry = w99_slot(this, 0x8C);
        let _ = platform::call1(entry, this as u32);
        if requested != read_const_f32(W99_F32_6608) as f64 {
            let entry = w99_slot(this, 0x84);
            let _ = w99_call_f64_u32(entry, requested, this as u32);
        }
        return;
    }

    write_word(this, 0x164, 0);
    write_word(this, 0x160, 0);
    let manager = platform::call1(w99_slot(this, 0x2C), this as u32);
    let mut handle = platform::call2(
        W93_FN_8023A0B0,
        manager,
        read_global_word(W99_TABLE_802F1354),
    );
    let name = read_global_word(W99_TABLE_802F1354 + index);
    if name != 0 {
        let manager = platform::call1(w99_slot(this, 0x2C), this as u32);
        handle = platform::call2(W93_FN_8023A0B0, manager, name);
        let entry = w99_slot(this, 0x7C);
        let _ = platform::call3(
            entry,
            this as u32,
            mode as u32,
            &mut handle as *mut u32 as u32,
        );
    }

    let zero = read_const_f32(W99_F32_6608) as f64;
    let span = w99_u16_to_f64(read_half(handle as usize, 0x20)) as f32;
    let mut current = zero;
    if requested == zero {
        if first < zero {
            current = (span - read_const_f32(W99_F32_6604)) as f64;
        }
    } else if requested < span as f64 {
        current = requested;
    }

    let entry = w99_slot(this, 0x70);
    let _ = w99_call_f64x3_u32x3(
        entry,
        first,
        current,
        f64::from_bits(payload),
        this as u32,
        &mut handle as *mut u32 as u32,
        read_global_word(W99_TABLE_802F1360 + index),
    );
    let entry = w99_slot(this, 0x74);
    let _ = w99_call_f64x3_u32x3(
        entry,
        first,
        current,
        f64::from_bits(payload),
        this as u32,
        &mut handle as *mut u32 as u32,
        read_global_word(W99_TABLE_802F1360 + index),
    );
    let entry = w99_slot(this, 0x80);
    let _ = platform::call2(
        entry,
        this as u32,
        read_global_word(W99_TABLE_802F136C + index),
    );
}

/// Runs one marker state transition.
///
/// In plain English: when the owner object at `+0x24` exists and its `+0x17C`
/// bit `0x80` is clear, read the `+0x160` flag word. When bits `1`, `4` and
/// `0xC000` are all clear, copy the `+0x168` word into the owner's `+0x168` and
/// dispatch the vtable slot `+0x64` with the three forwarded operands; otherwise
/// dispatch slot `+0x68` with the second operand.
///
/// # Original
/// - Ghidra: `FUN_800d69b0` at `0x800D69B0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d69b0_FUN_800d69b0.c`
///
/// # Arguments
/// * `first`, `second`, `third` - the three 64-bit operands forwarded by
///   [`fn_800d5dd0`] and [`fn_800d6940`].
/// * `this` - the owner; `+0x24` is the animated object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call and, on the clean path, one word copy.
///
/// # Safety
/// `this` and `this + 0x24` must be live and their vtables must implement the
/// called slots.
///
/// # Confidence
/// medium - the guards, the `+0x168` copy and the two slots are literal; the
/// operand typing follows the `0x800D6940` family convention, which the export
/// leaves as `undefined8`.
pub unsafe fn fn_800d69b0(first: u64, second: u64, third: u64, this: usize) {
    let object = read_word(this, 0x24) as usize;
    if object == 0 {
        return;
    }
    if read_word(object, 0x17C) & 0x80 != 0 {
        return;
    }

    let flags = read_word(this, 0x160);
    if flags & 2 == 0 && flags & 0x10 == 0 && flags & 0xC000 == 0 {
        write_word(object, 0x168, read_word(this, 0x168));
        let entry = w99_slot(object, 0x64);
        let _ = w99_call3_u64_u32(entry, first, third, second, object as u32);
        return;
    }

    let entry = w99_slot(object, 0x68);
    let _ = w99_call_u64_u32_u32(entry, second, object as u32, 0);
}

/// Builds and submits the 0x80-byte view command packet.
///
/// In plain English: zero the 0x80-byte buffer, store the `0x801B5F80` header
/// pair at `+0x00`, construct the 24-byte record at `+0x08`, then fill four
/// 0x18-byte view records from `0x80377C88`: the three words at `+0x24..+0x2C`,
/// the two floats from `0x802BC8C0` and the float from `0x802BC8E0`. Finally
/// hand the whole buffer to `0x801B8620`.
///
/// # Original
/// - Ghidra: `FUN_800d9760` at `0x800D9760` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9760_FUN_800d9760.c`
///
/// # Arguments
/// None. The four view objects come from the `0x80377C88` table.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the local packet and submits it through `0x801B8620`.
///
/// # Safety
/// The four `0x80377C88` entries must be live view objects and the packet
/// helpers must accept the 0x80-byte buffer.
///
/// # Confidence
/// medium - the packet size, the record stride and every field are literal; the
/// three `0x802BC8C0/E0` reads print no operand, so the view receiver is
/// `// inferred:` from the enclosing loop, and `0x801B5F80`'s pair is stored as
/// two words because the export types it `undefined8`.
pub unsafe fn fn_800d9760() {
    let mut packet = W99Packet128([0u8; 0x80]);
    let base = packet.0.as_mut_ptr() as u32;
    let _ = platform::call3(W99_FN_800046B4, base, 0, 0x80);

    let header = w99_call0_u64(W99_FN_801B5F80);
    write_word(base as usize, 0x00, header as u32);
    write_word(base as usize, 0x04, (header >> 32) as u32);
    let _ = platform::call1(W99_FN_801DD6E0, base + 0x08);

    let mut index = 0;
    while index < 4 {
        let view = read_global_word(W93_TABLE_SLOT_VIEWS + index * 4) as usize;
        let offset = 0x20 + index * 0x18;
        write_word(base as usize, offset, read_word(view, 0x24));
        write_word(base as usize, offset + 0x04, read_word(view, 0x28));
        write_word(base as usize, offset + 0x08, read_word(view, 0x2C));

        let mut pair = [0f32; 2];
        let _ = platform::call1(W99_FN_802BC8C0, pair.as_mut_ptr() as u32);
        write_word(base as usize, offset + 0x0C, pair[0].to_bits());
        write_word(base as usize, offset + 0x10, pair[1].to_bits());
        write_word(
            base as usize,
            offset + 0x14,
            w99_call1_f32_u32(W99_FN_802BC8E0, view as u32).to_bits(),
        );
        index += 1;
    }

    let _ = platform::call2(W99_FN_801B8620, base, 0x80);
}

/// Tests one point against a rectangular marker band.
///
/// In plain English: clear the output angle, read the point's y and the current
/// time, and when the object is not disabled (`+0x3C` bit `0x200`) and the
/// point's x lies in `[+0x1C, +0x14)`, interpolate the band's height from
/// `+0x20..+0x28` and accept when the point is at or below it. On success the
/// height and the `+0x40` angle are written out.
///
/// # Original
/// - Ghidra: `FUN_800dab40` at `0x800DAB40` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dab40_FUN_800dab40.c`
///
/// # Arguments
/// * `this` - the band; `+0x14/+0x1C` x bounds, `+0x20..+0x28` height ramp.
/// * `point` - the tested x/y pair.
/// * `angle_out` - receives the band's `+0x40` halfword on a hit.
/// * `depth_out` - receives the interpolated height on a hit.
///
/// # Returns
/// `1` on a hit, `0` otherwise.
///
/// # Side effects
/// Up to two output stores.
///
/// # Safety
/// `this`, `point`, `angle_out` and `depth_out` must be readable/writable.
///
/// # Confidence
/// high - the guard, the ramp and both stores are literal; `0x80101AA0`'s
/// missing receiver is the only dropped operand.
pub unsafe fn fn_800dab40(
    this: usize,
    point: *mut f32,
    angle_out: *mut u16,
    depth_out: *mut f32,
) -> u32 {
    *angle_out = 0;
    let y = *point.add(1);
    let now = w99_call0_f32(FN_80101AA0);
    let low = read_float(this, 0x1C);
    let high = read_float(this, 0x14);

    if read_word(this, 0x3C) & 0x200 == 0 && low <= *point && *point < high {
        let depth = ((now - low) * read_float(this, 0x28)) / read_float(this, 0x24)
            + read_float(this, 0x20);
        if read_const_f32(W99_F32_6430) + depth <= y {
            *depth_out = depth;
            *angle_out = read_half(this, 0x40);
            return 1;
        }
    }

    0
}

/// Tests one point against a circular marker and returns its surface height.
///
/// In plain English: clear the output angle, compute the delta from the
/// circle's `+0x54/+0x58` centre, and when the point's x lies inside
/// `[centre.x - r, centre.x + r]` and the heading from `0x801611C0` falls in the
/// `+0x40/+0x68/+0x6C` arc, solve the circle for the point's height, write the
/// y and the mirrored angle out, and report a hit.
///
/// # Original
/// - Ghidra: `FUN_800dac40` at `0x800DAC40` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dac40_FUN_800dac40.c`
///
/// # Arguments
/// * `this` - the circle; `+0x54/+0x58` centre, `+0x64` radius, `+0x40`
///   reference angle, `+0x68/+0x6C` arc window.
/// * `point` - the tested x/y pair.
/// * `angle_out` - receives `-(0x4000 - heading)` on a hit.
/// * `depth_out` - receives the solved y on a hit.
///
/// # Returns
/// `1` on a hit, `0` otherwise.
///
/// # Side effects
/// Up to two output stores.
///
/// # Safety
/// `this` and the three pointers must be readable/writable.
///
/// # Confidence
/// medium - the bounds, the arc test, the circle solve and both stores are
/// literal; the first `0x802BE470` result is discarded by the export and the
/// paired-single `ldexpf` stubs at head and tail are no-ops.
pub unsafe fn fn_800dac40(
    this: usize,
    point: *mut f32,
    angle_out: *mut i16,
    depth_out: *mut f32,
) -> u32 {
    *angle_out = 0;
    let dx = *point - read_float(this, 0x54);
    let dy = *point.add(1) - read_float(this, 0x58);
    let _ = call_f64_ret_f64(W99_FN_802BE470, (dx * dx + dy * dy) as f64);

    let radius = read_float(this, 0x64);
    let centre_x = read_float(this, 0x54);
    if centre_x - radius <= *point && *point <= radius + centre_x {
        let heading = call_f64_f64_u32(FN_801611C0, dy as f64, dx as f64) as i32;
        let arc = (heading
            - ((read_half(this, 0x40) as i16 as i32) - read_word(this, 0x68) as i32))
            as u32
            & 0xFFFF;
        if heading as i16 >= 0 && arc < read_word(this, 0x6C) {
            let mut height = read_const_f32(W99_F32_6448) as f64;
            let remainder = (((radius as f64 * radius as f64) as f32) - dx * dx) as f64;
            if height < remainder {
                let inverse = call_f64_ret_f64(W76_FN_80237BA0, remainder);
                height = ((remainder * inverse) as f32) as f64;
            }
            *depth_out = (height + read_float(this, 0x58) as f64) as f32;
            let reference = read_half(this, 0x40) as i16 as i32;
            *angle_out = (-(0x4000 - reference)) as i16;
            return 1;
        }
    }

    0
}

/// Registers the six `dRemoconMng_c::dConnect_c` state descriptors.
///
/// In plain English: for each of the six states, name the descriptor, install
/// the `0x80319290` (first two) or `0x8031925C` (last four) vtable, copy the
/// nine handler words from the matching 0x24-byte source record and register
/// the destructor `0x800DD770` or `0x800DD7D0`.
///
/// # Original
/// - Ghidra: `FUN_800dd2b0` at `0x800DD2B0` (1212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd2b0_FUN_800dd2b0.c`
///
/// # Arguments
/// None. The descriptor blocks and their source records are absolute globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills `0x80371830`, `0x80371870`, `0x803718B0`, `0x803718F0`, `0x80371930`
/// and `0x80371970`, and registers each through `0x802DC780`.
///
/// # Safety
/// The six destination blocks, the six source records and the support objects
/// must be writable/readable.
///
/// # Confidence
/// high - all six blocks, their source bases, their names, the two vtables and
/// the two destructors are literal; [`w99_descriptor_fields`] documents the one
/// non-obvious detail, the permuted copy order.
pub unsafe fn fn_800dd2b0() {
    w93_register_state(
        0x8037_1830,
        0x8031_9120, // s_dRemoconMng_c__dConnect_c__State
        W99_HANDLER_800DD770,
        0x8037_1820,
        W99_VTABLE_80319290,
        w99_descriptor_fields(0x8031_9048),
    );
    w93_register_state(
        0x8037_1870,
        0x8031_914C, // s_dRemoconMng_c__dConnect_c__State
        W99_HANDLER_800DD770,
        0x8037_1860,
        W99_VTABLE_80319290,
        w99_descriptor_fields(0x8031_906C),
    );
    w93_register_state(
        0x8037_18B0,
        0x8031_9178, // s_dRemoconMng_c__dConnect_c__dExte
        W99_HANDLER_800DD7D0,
        0x8037_18A0,
        W99_VTABLE_8031925C,
        w99_descriptor_fields(0x8031_9090),
    );
    w93_register_state(
        0x8037_18F0,
        0x8031_91B0, // s_dRemoconMng_c__dConnect_c__dExte
        W99_HANDLER_800DD7D0,
        0x8037_18E0,
        W99_VTABLE_8031925C,
        w99_descriptor_fields(0x8031_90B4),
    );
    w93_register_state(
        0x8037_1930,
        0x8031_91E8, // s_dRemoconMng_c__dConnect_c__dExte
        W99_HANDLER_800DD7D0,
        0x8037_1920,
        W99_VTABLE_8031925C,
        w99_descriptor_fields(0x8031_90D8),
    );
    w93_register_state(
        0x8037_1970,
        0x8031_9224, // s_dRemoconMng_c__dConnect_c__dExte
        W99_HANDLER_800DD7D0,
        0x8037_1960,
        W99_VTABLE_8031925C,
        w99_descriptor_fields(0x8031_90FC),
    );
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: the first `dConnect_c` state thunk; it registers the two
/// objects in the order every thunk in this band uses.
///
/// # Original
/// - Ghidra: `FUN_800dd1c0` at `0x800DD1C0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd1c0_FUN_800dd1c0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the operand order is printed literally; the same shape is documented
/// by [`fn_801533a0`].
pub unsafe fn fn_800dd1c0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Second `dConnect_c` state thunk; see [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_800dd1f0` at `0x800DD1F0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd1f0_FUN_800dd1f0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_800dd1f0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Third `dConnect_c` state thunk; see [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_800dd220` at `0x800DD220` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd220_FUN_800dd220.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_800dd220(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x24);
}

/// Fourth `dConnect_c` state thunk; see [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_800dd250` at `0x800DD250` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd250_FUN_800dd250.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_800dd250(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Fifth `dConnect_c` state thunk; see [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_800dd280` at `0x800DD280` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd280_FUN_800dd280.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_800dd280(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Releases a `dConnect_c` extension object and optionally frees it.
///
/// In plain English: when `object` is non-null, run the `0x8015F800`
/// destructor on it and, when `free > 0`, free the storage; return the object.
///
/// # Original
/// - Ghidra: `FUN_800dd7d0` at `0x800DD7D0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dd7d0_FUN_800dd7d0.c`
///
/// # Arguments
/// * `object` - the state object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// One destructor call and, conditionally, one free.
///
/// # Safety
/// `object` must be null or a live state object.
///
/// # Confidence
/// high - both guards, the destructor and the free are literal.
pub unsafe fn fn_800dd7d0(object: usize, free: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Releases an object with an embedded sub-object and optionally frees it.
///
/// In plain English: when `object` is non-null, release the sub-object at
/// `+0x18` through `0x8015F9F0`, then free the storage when `free > 0`; return
/// the object.
///
/// # Original
/// - Ghidra: `FUN_800ddb50` at `0x800DDB50` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800ddb50_FUN_800ddb50.c`
///
/// # Arguments
/// * `object` - the wrapper, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// One sub-object release and, conditionally, one free.
///
/// # Safety
/// `object` must be null or a live wrapper whose `+0x18` sub-object is valid.
///
/// # Confidence
/// high - the guards, the `+0x18` release, the free and the returned pointer are
/// literal; the export's `param_1 + 0x18 != 0` test is reproduced as written
/// because it is always true for a non-null object.
pub unsafe fn fn_800ddb50(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Releases a `dReplayPlay_c` state object and optionally frees it.
///
/// In plain English: the handler registered by [`fn_800de510`] for all four
/// replay states; it runs `0x8015F800` and optionally `0x802B9280`.
///
/// # Original
/// - Ghidra: `FUN_800de850` at `0x800DE850` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de850_FUN_800de850.c`
///
/// # Arguments
/// * `object` - the state object, or `0`.
/// * `free` - positive when the storage must be released.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// One destructor call and, conditionally, one free.
///
/// # Safety
/// `object` must be null or a live replay state object.
///
/// # Confidence
/// high - identical body to [`fn_800dd7d0`]; this is the address
/// [`fn_800de510`] registers.
pub unsafe fn fn_800de850(object: usize, free: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Sixth and seventh `dConnect_c` thunks; see [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_800de4b0` at `0x800DE4B0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de4b0_FUN_800de4b0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_800de4b0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Seventh `dConnect_c` thunk; see [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_800de4e0` at `0x800DE4E0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de4e0_FUN_800de4e0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_800de4e0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Tests one embedded object against a type tag and dispatches slot `+0x34`.
///
/// In plain English: take the object at `+0x04`, ask its vtable slot `+0x08`
/// whether it matches `&DAT_802F86C8`, and dispatch slot `+0x34` on the result.
///
/// # Original
/// - Ghidra: `FUN_800dfbe0` at `0x800DFBE0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfbe0_FUN_800dfbe0.c`
///
/// # Arguments
/// * `this` - wrapper whose `+0x04` word is the tested object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One type query and one virtual call.
///
/// # Safety
/// `this + 0x04` must be null or a live object; in practice the query always
/// succeeds before the dispatch.
///
/// # Confidence
/// medium - the tag, the two slots and the call are literal; the export clears
/// the receiver to `0` when the query fails and still dispatches, which is
/// reproduced as observed but would fault if reached.
pub unsafe fn fn_800dfbe0(this: usize) {
    let mut object = read_word(this, 0x04) as usize;
    let mut found = false;
    if object != 0 {
        let vtable = read_word(object, 0) as usize;
        let entry = read_word(vtable, 0x08) as usize;
        if platform::call2(entry, object as u32, W93_TAG_802F86C8 as u32) != 0 {
            found = true;
        }
    }
    if !found {
        object = 0;
    }

    let vtable = read_word(object, 0) as usize;
    let entry = read_word(vtable, 0x34) as usize;
    let _ = platform::call0(entry);
}

/// Submits one interface record built from `0x80168400`.
///
/// In plain English: build a five-word scratch record whose first word is the
/// `0x80168400` handle, then submit it through `0x800DFC70`.
///
/// # Original
/// - Ghidra: `FUN_800dfce0` at `0x800DFCE0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfce0_FUN_800dfce0.c`
///
/// # Arguments
/// * `_first` - unused by the body.
/// * `second` - forwarded to `0x800DFC70`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One record submission.
///
/// # Safety
/// `0x800DFC70` must accept the five-word scratch record and `second`.
///
/// # Confidence
/// medium - the scratch size, the handle store and the submission are literal;
/// the callee's argument roles are `// inferred:` from its wave-93 use.
pub unsafe fn fn_800dfce0(_first: u32, second: u32) {
    let mut record = [0u32; 5];
    record[0] = call0_r(W93_FN_80168400);
    let _ = platform::call2(
        W99_FN_800DFC70,
        record.as_mut_ptr() as u32,
        second,
    );
}

// ---------------------------------------------------------------------------
// Wave 100 constants: state handlers, vtables and the SDA slots they use
// ---------------------------------------------------------------------------

/// `0x80154300`: release callback of the four outer `daYoshi_c` states; it is
/// [`fn_80154300`] in this file.
const W100_HANDLER_80154300: u32 = 0x8015_4300;

/// `0x80154360`: release callback of the eleven riding/physics `daYoshi_c`
/// states; it is [`fn_80154360`] in this file.
const W100_HANDLER_80154360: u32 = 0x8015_4360;

/// `0x8015F040`: release callback of every `dScBoot_c` state; it is
/// [`fn_8015f040`] in this file.
const W100_HANDLER_8015F040: u32 = 0x8015_F040;

/// `0x8015B840`: release callback of every `Pausewindow_c` state; it is
/// [`pausewindow_state_release`] in this file.
const W100_HANDLER_8015B840: u32 = 0x8015_B840;

/// `0x803271F0`: state vtable of the four "outer" `daYoshi_c` states
/// (`AloneWait`, `BlockOut`, `DamageRun`, `StartRideOn`) and the four
/// `Eat*` states.
const W100_VTABLE_803271F0: u32 = 0x8032_71F0;

/// `0x803271BC`: state vtable of the eleven riding/physics `daYoshi_c`
/// states, from `Walk` through `Cloud`.
const W100_VTABLE_803271BC: u32 = 0x8032_71BC;

/// `0x803289E4`: state vtable shared by every `dScBoot_c` state.
const W100_VTABLE_803289E4: u32 = 0x8032_89E4;

/// `0x80328058`: state vtable shared by every `Pausewindow_c` state.
const W100_VTABLE_80328058: u32 = 0x8032_8058;

/// `0x8008F870`: registers a named actor list `(list, name, 0)`.
const W100_FN_8008F870: usize = 0x8008_F870;

/// `0x8008F810`: release callback for the three actor lists registered by
/// [`fn_8015ddd0`].
const W100_FN_8008F810: usize = 0x8008_F810;

/// `0x8015F600`: copies a word table into an SDA block `(dst, table, count)`.
const W100_FN_8015F600: usize = 0x8015_F600;

/// `0x8015F9F0`: state-manager destructor `(manager, free)`.
const W100_FN_8015F9F0: usize = 0x8015_F9F0;

/// `0x801515C0`: returns the twelve-word selection record of a player object.
const W100_FN_801515C0: usize = 0x8015_15C0;

/// `0x8006D090`: binds the three attribute words
/// `(context, owner, attr_a, attr_b, attr_c)`.
const W100_FN_8006D090: usize = 0x8006_D090;

/// `0x80053490`: returns the current model/effect id of a player object.
const W100_FN_80053490: usize = 0x8005_3490;

/// `0x80064610`: sets one pad-attention mask
/// `(channel, mask, record, 0, 0)`.
const W100_FN_80064610: usize = 0x8006_4610;

/// `0x80076FD0`: resets the per-frame pad/route state of the scene.
const W100_FN_80076FD0: usize = 0x8007_6FD0;

/// `0x80081780`: copies one model variant byte `(model, variant)`.
const W100_FN_80081780: usize = 0x8008_1780;

/// `0x80077A00`: rebuilds the collision frame after a pane update.
const W100_FN_80077A00: usize = 0x8007_7A00;

/// `0x80077B40`: rebuilds the render frame after a pane update.
const W100_FN_80077B40: usize = 0x8007_7B40;

/// `0x800CD470`: first half of the NW4R layout draw-order pass.
const W100_FN_800CD470: usize = 0x800C_D470;

/// `0x800CD520`: second half of the NW4R layout draw-order pass.
const W100_FN_800CD520: usize = 0x800C_D520;

/// `0x800D8640`: resets the embedded two-line text block.
const W100_FN_800D8640: usize = 0x800D_8640;

/// `0x800892F0`: resets the embedded gauge block.
const W100_FN_800892F0: usize = 0x8008_92F0;

/// `0x8007E2C0`: registers the first half of the embedded effect pair.
const W100_FN_8007E2C0: usize = 0x8007_E2C0;

/// `0x8007E3D0`: registers the second half of the embedded effect pair.
const W100_FN_8007E3D0: usize = 0x8007_E3D0;

/// `0x801543C0`: `Cloud` state table getter.
const W100_FN_801543C0: usize = 0x8015_43C0;

/// `0x801543D0`: `Funsui` state table getter.
const W100_FN_801543D0: usize = 0x8015_43D0;

/// `0x801543E0`: `HipAttack` state table getter.
const W100_FN_801543E0: usize = 0x8015_43E0;

/// `0x801543F0`: `Turn` state table getter.
const W100_FN_801543F0: usize = 0x8015_43F0;

/// `0x80154400`: `Slip` state table getter.
const W100_FN_80154400: usize = 0x8015_4400;

/// `0x80154410`: `SitJump` state table getter.
const W100_FN_80154410: usize = 0x8015_4410;

/// `0x80154420`: `Crouch` state table getter.
const W100_FN_80154420: usize = 0x8015_4420;

/// `0x80154430`: `Land` state table getter.
const W100_FN_80154430: usize = 0x8015_4430;

/// `0x80154440`: `Fall` state table getter.
const W100_FN_80154440: usize = 0x8015_4440;

/// `0x80154450`: `Jump` state table getter.
const W100_FN_80154450: usize = 0x8015_4450;

/// `0x80154460`: `Walk` state table getter.
const W100_FN_80154460: usize = 0x8015_4460;

/// `0x801548A0`: builds the four route panes `(scene)`; not translated in this
/// pass because its export loses the `r13`-relative base to `unaff`.
const W100_FN_801548A0: usize = 0x8015_48A0;

/// `0x80154A70`: builds one route pane `(scene, index, flag)`; not translated
/// in this pass for the same `unaff` reason.
const W100_FN_80154A70: usize = 0x8015_4A70;

/// `0x80376DC4`: name tag compared by the slot `+0x10` query in
/// [`fn_80151630`]; the bytes spell a class name in the binary.
const W100_TAG_80376DC4: u32 = 0x8037_6DC4;

/// `r13 - 0x5464` -> `0x8042A21C`: "route suspended" byte tested by
/// [`fn_80154740`].
const W100_SDA_5464: usize = SDA_BASE - 0x5464;

/// `r13 - 0x56C0` -> `0x80429FC0`: pointer to the scene-mode object whose
/// `+0x14` word holds the mode id.
const W100_SDA_56C0: usize = SDA_BASE - 0x56C0;

/// `r13 - 0x5890` -> `0x80429DF0`: model handle passed to `0x80081780`.
const W100_SDA_5890: usize = SDA_BASE - 0x5890;

/// `r13 - 0x54B8` -> `0x8042A1C8`: pointer to the player-info block whose
/// `+0x120E` byte selects the model variant and `+0x120F` the rider.
const W100_SDA_54B8: usize = SDA_BASE - 0x54B8;

/// `r13 - 0x5348` -> `0x8042A338`: pause-window state-callback slot cleared
/// while [`fn_8015b320`] runs.
const W100_SDA_5348: usize = SDA_BASE - 0x5348;

/// `r2 - 0x54A0`: first word copied to `0x803279D0` by [`fn_8015b320`].
const W100_SDA2_54A0: usize = SDA2_BASE - 0x54A0;

/// `r2 - 0x549C`: second word copied to `0x803279D4`.
const W100_SDA2_549C: usize = SDA2_BASE - 0x549C;

/// `r2 - 0x5498`: third word copied to `0x803279D8`.
const W100_SDA2_5498: usize = SDA2_BASE - 0x5498;

/// `r2 - 0x6114`: word copied to `0x803279C4`.
const W100_SDA2_6114: usize = SDA2_BASE - 0x6114;

/// `r2 - 0x60FC`: word copied into the `0x8042A338` callback slot.
const W100_SDA2_60FC: usize = SDA2_BASE - 0x60FC;

/// `r2 - 0x5590`: first pad-record word and the `+0x90BDC` value.
const W100_SDA2_5590: usize = SDA2_BASE - 0x5590;

/// `r2 - 0x558C`: second pad-record word.
const W100_SDA2_558C: usize = SDA2_BASE - 0x558C;

/// `r2 - 0x5588`: third pad-record word used by the second mask call.
const W100_SDA2_5588: usize = SDA2_BASE - 0x5588;

/// `0x803279D0`: first global filled by [`fn_8015b320`]; role unknown.
const W100_G_803279D0: usize = 0x8032_79D0;

/// `0x803279D4`: second global filled by [`fn_8015b320`]; role unknown.
const W100_G_803279D4: usize = 0x8032_79D4;

/// `0x803279D8`: third global filled by [`fn_8015b320`]; role unknown.
const W100_G_803279D8: usize = 0x8032_79D8;

/// `0x803279C4`: global filled from `r2 - 0x6114`; role unknown.
const W100_G_803279C4: usize = 0x8032_79C4;

/// Installs one `daYoshi_c` state descriptor.
///
/// The original repeats the same block nineteen times: read nine handler words
/// from the source record, call `0x8015F7C0` with the state name, write the
/// vtable at `+0`, the nine words at `+0x0C..+0x2C`, optionally the riding
/// vtable getter result at `+0x30`, then register the descriptor.
///
/// # Original
/// Host-side factoring of the nineteen identical blocks in `0x80153400`
/// (`work/ghidra/main-gb/export/80153400_FUN_80153400.c`); no binary address
/// of its own.
///
/// # Arguments
/// * `state` - descriptor address (`0x80376AC8` onward).
/// * `name` - `s_daYoshi_c__StateID_*` string address.
/// * `support` - registration link word passed to `0x802DC780`.
/// * `vtable` - `0x803271F0` or `0x803271BC`.
/// * `source` - 0x24-byte handler source record.
/// * `handler` - release callback address.
/// * `extra` - tenth word at `+0x30`; only the eleven riding states have it.
///
/// # Safety
/// `state` must be writable for at least `0x38` bytes and `source` readable.
///
/// # Confidence
/// high - the constructor, the nine stores, the optional tenth store and the
/// registration call are copied one-for-one from the nineteen blocks.
#[inline]
unsafe fn w100_install_yoshi_state(
    state: usize,
    name: u32,
    support: usize,
    vtable: u32,
    source: usize,
    handler: u32,
    extra: Option<u32>,
) {
    // observed: all nine source words are loaded before the name is stored.
    let fields = w99_descriptor_fields(source);

    let _ = platform::call2(FN_8015F7C0, state as u32, name);
    write_global_word(state, vtable);
    let mut index = 0;
    while index < 9 {
        write_global_word(state + 0x0C + index * 4, fields[index]);
        index += 1;
    }
    if let Some(value) = extra {
        write_global_word(state + 0x30, value);
    }
    let _ = platform::call3(FN_802DC780, state as u32, handler, support as u32);
}

/// Installs one `Pausewindow_c` state descriptor.
///
/// Same nine-word block as [`w100_install_yoshi_state`] but with the
/// pause-window vtable and the shared release callback.
///
/// # Original
/// Host-side factoring of the six identical blocks in `0x8015B320`
/// (`work/ghidra/main-gb/export/8015b320_FUN_8015b320.c`); no binary address
/// of its own.
///
/// # Arguments
/// * `state` - descriptor address (`0x80377180..0x803772C0`).
/// * `name` - `s_Pausewindow_c__StateID_*` string address.
/// * `link` - registration link word passed to `0x802DC780`.
/// * `source` - 0x24-byte handler source record.
///
/// # Safety
/// `state` must be writable for `0x30` bytes and `source` readable.
///
/// # Confidence
/// high - every store and both calls are literal in the six blocks.
#[inline]
unsafe fn w100_install_pause_state(state: u32, name: u32, link: usize, source: usize) {
    let fields = w99_descriptor_fields(source);

    let _ = platform::call2(FN_8015F7C0, state, name);
    write_global_word(state as usize, W100_VTABLE_80328058);
    let mut index = 0;
    while index < 9 {
        write_global_word(state as usize + 0x0C + index * 4, fields[index]);
        index += 1;
    }
    let _ = platform::call3(
        FN_802DC780,
        state,
        W100_HANDLER_8015B840,
        link as u32,
    );
}

/// Copies the twelve-word selection record into a player object.
///
/// In plain English: clear the `0xB8` bit, copy the twelve words returned by
/// `0x801515C0` into `+0x1094..+0x10C0`, then OR a pile of capability bits into
/// the three attribute words `+0x10A4`, `+0x1094` and `+0x10B4` depending on
/// the object's mode, the pad flags and two virtual queries. When no player
/// object exists the three words reset to `0x801` and the `0xB8` bit is set.
/// Finally bind the three words to the context at `+0x1EC` and refresh the two
/// model ids.
///
/// # Original
/// - Ghidra: `FUN_80151630` at `0x80151630` (668 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151630_FUN_80151630.c`
///
/// # Arguments
/// * `this` - player/actor object; `+0x1464` is its attached interface and
///   `+0x14D8` its character record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 12 words at `+0x1094..+0x10C0`, ORs capability bits into three of
/// them, calls `0x8006D090` with the three word addresses, and writes the two
/// bytes `+0x2D1`, `+0x30A`, `+0x2D8`, `+0x30E`.
///
/// # Safety
/// `this` must be a live player object with the interface at `+0x1464` and the
/// record at `+0x14D8`; `0x801515C0` must return a readable twelve-word record.
///
/// # Confidence
/// medium - every read, store, bit mask call and slot index is literal; the
/// `+0x1094/+0x10A4/+0x10B4` roles are `// inferred:` from the three-way bind
/// call, and the compared SDA2 float at `r2 - 0x5668` is only known by address.
pub unsafe fn fn_80151630(this: usize) {
    let _ = platform::call2(W95_FN_80056CB0, this as u32, 0xB8);
    let source = platform::call1(W100_FN_801515C0, this as u32) as usize;
    write_word(this, 0x10A4, read_word(source, 0x00));
    write_word(this, 0x10A8, read_word(source, 0x04));
    write_word(this, 0x10AC, read_word(source, 0x08));
    write_word(this, 0x10B0, read_word(source, 0x0C));
    write_word(this, 0x1094, read_word(source, 0x10));
    write_word(this, 0x1098, read_word(source, 0x14));
    write_word(this, 0x109C, read_word(source, 0x18));
    write_word(this, 0x10A0, read_word(source, 0x1C));
    write_word(this, 0x10B4, read_word(source, 0x20));
    write_word(this, 0x10B8, read_word(source, 0x24));
    write_word(this, 0x10BC, read_word(source, 0x28));
    write_word(this, 0x10C0, read_word(source, 0x2C));

    let character = platform::call1(W95_FN_8014E9B0, this as u32) as usize;
    if character == 0 {
        write_word(this, 0x10A4, 0x801);
        write_word(this, 0x1094, 0x801);
        write_word(this, 0x10B4, 0x801);
        let _ = platform::call2(W95_FN_80056C70, this as u32, 0xB8);
    } else {
        let iface = vcall0(this + 0x1464, 0x28) as usize;
        if vcall1(iface, 0x10, W100_TAG_80376DC4) != 0 {
            let record = read_word(this, 0x14D8) as usize;
            let mode = read_word(record, 0x154);
            if mode == 0x11 {
                // unknown: the threshold at `r2 - 0x5668`; only its address is
                // recovered, the value is whatever the SDA2 block holds.
                if read_float(this, 0xEC) < read_const_f32(W95_SDA2_5668) {
                    let flags = read_word(this, 0x10A4) | 0x0050_0000;
                    write_word(this, 0x10A4, flags);
                    if platform::call2(W95_FN_80056CF0, this as u32, 0xC4) != 0 {
                        let flags = read_word(this, 0x10A4) | 0x0080_0000;
                        write_word(this, 0x10A4, flags);
                    }
                }
            } else if mode == 0x12
                && platform::call2(W95_FN_80056CF0, this as u32, 0xC4) != 0
            {
                let flags = read_word(this, 0x10A4) | 0x0090_0000;
                write_word(this, 0x10A4, flags);
            }
        }

        if platform::call2(W95_FN_80056CF0, this as u32, 0x30) != 0 {
            let flags = read_word(this, 0x10A4) | 0x0001_0000;
            write_word(this, 0x10A4, flags);
        }
        if platform::call2(W95_FN_80056CF0, this as u32, 0xA4) != 0 {
            let flags = read_word(this, 0x10A4) | 0x08;
            write_word(this, 0x10A4, flags);
        }
        if platform::call2(W95_FN_80056CF0, this as u32, 0x5D) != 0 {
            let flags = read_word(this, 0x10A4) | 0x20;
            write_word(this, 0x10A4, flags);
        }
        if vcall0(this, 0x3D4) != 0 {
            let flags = read_word(this, 0x10B4) | 0x0200_0000;
            write_word(this, 0x10B4, flags);
            let flags = read_word(this, 0x1094) | 0x0200_0000;
            write_word(this, 0x1094, flags);
        }
        if platform::call2(W95_FN_80056CF0, this as u32, 0x82) != 0 {
            let flags = read_word(this, 0x1094) | 0x0400_0000;
            write_word(this, 0x1094, flags);
        }
        if vcall0(character, 0xE0) != 0 {
            let _ = platform::call2(W95_FN_80056C70, this as u32, 0xB8);
        }
    }

    let _ = platform::call5(
        W100_FN_8006D090,
        (this + 0x1EC) as u32,
        this as u32,
        (this + 0x10A4) as u32,
        (this + 0x1094) as u32,
        (this + 0x10B4) as u32,
    );
    let model = platform::call1(W100_FN_80053490, this as u32) as u8;
    write_byte(this, 0x2D1, model);
    let model = platform::call1(W100_FN_80053490, this as u32) as u8;
    write_byte(this, 0x30A, model);
    let variant = read_byte(this, 0x38F);
    write_byte(this, 0x2D8, variant);
    let variant = read_byte(this, 0x38F);
    write_byte(this, 0x30E, variant);
}

/// Resets the route panes, the pad masks and the embedded HUD blocks.
///
/// In plain English: reset the per-frame route state, copy the selected model
/// variant into the model, rebuild the four panes, clear the ready byte, rebuild
/// the collision and render frames, run the two layout draw-order passes, set
/// both pad masks and reset the text, gauge and effect blocks, then seed the
/// state words. Always returns `1`.
///
/// # Original
/// - Ghidra: `FUN_80154740` at `0x80154740` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154740_FUN_80154740.c`
///
/// # Arguments
/// * `this` - the scene/route object; the `+0x90AAC..+0x90BE0` block is its
///   HUD state.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// Two pad-mask calls, four frame rebuilds, two layout passes, four sub-object
/// resets and eleven stores in the `+0x909C8..+0x90BDC` block.
///
/// # Safety
/// `this` must be a live scene object; the SDA pointers at `r13 - 0x56C0` and
/// `r13 - 0x54B8` must be readable.
///
/// # Confidence
/// medium - every call and store is literal; the SDA slots keep their observed
/// roles only where the neighbouring wave-16/78 code proves them, the rest are
/// `// unknown:`.
pub unsafe fn fn_80154740(this: usize) -> u32 {
    let _ = platform::call0(W100_FN_80076FD0);

    let mode_object = read_global_word(W100_SDA_56C0) as usize;
    if read_global_byte(W100_SDA_5464) == 0 && read_word(mode_object, 0x14) != 2 {
        let info = read_global_word(W100_SDA_54B8) as usize;
        let _ = platform::call2(
            W100_FN_80081780,
            read_global_word(W100_SDA_5890),
            read_byte(info, 0x120E) as u32,
        );
    }

    let _ = platform::call1(W100_FN_801548A0, this as u32);
    let mut index = 0u32;
    while (index as i32) < 4 {
        let _ = platform::call3(
            W100_FN_80154A70,
            this as u32,
            index & 0xFFFF,
            1,
        );
        index += 1;
    }

    write_byte(this, 0x90124, 0);
    let _ = platform::call1(W100_FN_80077A00, this as u32);
    let _ = platform::call1(W100_FN_80077B40, this as u32);
    let _ = platform::call1(W100_FN_800CD470, (this + 0x90BA4) as u32);
    let _ = platform::call1(W100_FN_800CD520, (this + 0x90BA4) as u32);

    // observed: the record is (first, first, second), then the third word is
    // overwritten before the second mask call.
    let mut record = [0u32; 3];
    record[0] = read_global_word(W100_SDA2_5590);
    record[1] = record[0];
    record[2] = read_global_word(W100_SDA2_558C);
    let _ = platform::call5(
        W100_FN_80064610,
        0x7D,
        0x1FF,
        record.as_mut_ptr() as u32,
        0,
        0,
    );
    record[2] = read_global_word(W100_SDA2_5588);
    let _ = platform::call5(
        W100_FN_80064610,
        0x7D,
        0x2FF,
        record.as_mut_ptr() as u32,
        0,
        0,
    );

    let _ = platform::call1(W100_FN_800D8640, (this + 0x90AAC) as u32);
    let _ = platform::call1(W100_FN_800892F0, (this + 0x90AC0) as u32);
    let _ = platform::call1(W100_FN_8007E2C0, (this + 0x90B60) as u32);
    let _ = platform::call1(W100_FN_8007E3D0, (this + 0x90B60) as u32);

    let seed = read_global_word(W100_SDA2_5590);
    write_word(this, 0x90BD4, 1);
    write_word(this, 0x90BDC, seed);
    write_half(this, 0x909C8, 0);
    write_half(this, 0x90BE0, 0);
    write_byte(this, 0x909CC, 0);

    1
}

/// Releases the embedded state manager and optionally frees the storage.
///
/// In plain English: when the object is non-null, destroy the manager at
/// `+0x18`, then free the object when `free > 0`; return the object pointer
/// unchanged.
///
/// # Original
/// - Ghidra: `FUN_801578f0` at `0x801578F0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801578f0_FUN_801578f0.c`
///
/// # Arguments
/// * `object` - the container to release.
/// * `free` - positive to free the storage after destruction.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// One manager destructor and, when `free > 0`, one free.
///
/// # Safety
/// `object` must be a container built by the matching constructor.
///
/// # Confidence
/// high - the two guards, the manager offset, the destructor call and the
/// returned pointer are literal. The `object + 0x18 != 0` test is kept even
/// though it can only fail on address wrap, exactly as observed.
pub unsafe fn fn_801578f0(object: usize, free: i32) -> usize {
    if object != 0 {
        if object + 0x18 != 0 {
            let _ = platform::call2(W100_FN_8015F9F0, (object + 0x18) as u32, 0);
        }
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Releases a state object and optionally frees it.
///
/// In plain English: the `daYoshi_c` riding-state release callback. It is the
/// same body as [`fn_80154300`] and is stored in eleven state descriptors by
/// [`fn_80153400`].
///
/// # Original
/// - Ghidra: `FUN_80154360` at `0x80154360` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154360_FUN_80154360.c`
///
/// # Arguments
/// * `object` - the state object to release.
/// * `free` - positive to free the storage after destruction.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// One `0x8015F800` destructor and, when `free > 0`, one `0x802B9280` free.
///
/// # Safety
/// `object` must be a state object built by the matching constructor.
///
/// # Confidence
/// high - the null guard, the destructor, the sign test and the returned
/// pointer are literal and identical to `0x80154300`.
pub unsafe fn fn_80154360(object: usize, free: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Releases a boot-scene state object and optionally frees it.
///
/// In plain English: the release callback stored in every `dScBoot_c` state
/// descriptor by [`fn_8015ddd0`]. Same body as [`fn_80154300`].
///
/// # Original
/// - Ghidra: `FUN_8015f040` at `0x8015F040` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f040_FUN_8015f040.c`
///
/// # Arguments
/// * `object` - the state object to release.
/// * `free` - positive to free the storage after destruction.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// One `0x8015F800` destructor and, when `free > 0`, one `0x802B9280` free.
///
/// # Safety
/// `object` must be a state object built by the matching constructor.
///
/// # Confidence
/// high - the null guard, the destructor, the sign test and the returned
/// pointer are literal.
pub unsafe fn fn_8015f040(object: usize, free: i32) -> usize {
    if object != 0 {
        let _ = platform::call2(FN_8015F800, object as u32, 0);
        if free > 0 {
            let _ = platform::call1(FN_802B9280, object as u32);
        }
    }
    object
}

/// Returns the `Cloud` state table pointer `0x803545DC`.
///
/// In plain English: a constant getter. [`fn_80153400`] stores the result in
/// the tenth word of the `daYoshi_c::StateID_Cloud` descriptor.
///
/// # Original
/// - Ghidra: `FUN_801543c0` at `0x801543C0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801543c0_FUN_801543c0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x803545DC`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_803545dc`.
pub unsafe fn fn_801543c0() -> usize {
    0x8035_45DC
}

/// Returns the `Funsui` state table pointer `0x8035455C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Funsui`.
///
/// # Original
/// - Ghidra: `FUN_801543d0` at `0x801543D0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801543d0_FUN_801543d0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035455C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035455c`.
pub unsafe fn fn_801543d0() -> usize {
    0x8035_455C
}

/// Returns the `HipAttack` state table pointer `0x8035445C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_HipAttack`.
///
/// # Original
/// - Ghidra: `FUN_801543e0` at `0x801543E0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801543e0_FUN_801543e0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035445C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035445c`.
pub unsafe fn fn_801543e0() -> usize {
    0x8035_445C
}

/// Returns the `Turn` state table pointer `0x8035441C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Turn`.
///
/// # Original
/// - Ghidra: `FUN_801543f0` at `0x801543F0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801543f0_FUN_801543f0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035441C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035441c`.
pub unsafe fn fn_801543f0() -> usize {
    0x8035_441C
}

/// Returns the `Slip` state table pointer `0x803543DC`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Slip`.
///
/// # Original
/// - Ghidra: `FUN_80154400` at `0x80154400` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154400_FUN_80154400.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x803543DC`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_803543dc`.
pub unsafe fn fn_80154400() -> usize {
    0x8035_43DC
}

/// Returns the `SitJump` state table pointer `0x803542DC`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_SitJump`.
///
/// # Original
/// - Ghidra: `FUN_80154410` at `0x80154410` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154410_FUN_80154410.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x803542DC`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_803542dc`.
pub unsafe fn fn_80154410() -> usize {
    0x8035_42DC
}

/// Returns the `Crouch` state table pointer `0x8035439C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Crouch`.
///
/// # Original
/// - Ghidra: `FUN_80154420` at `0x80154420` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154420_FUN_80154420.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035439C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035439c`.
pub unsafe fn fn_80154420() -> usize {
    0x8035_439C
}

/// Returns the `Land` state table pointer `0x8035435C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Land`.
///
/// # Original
/// - Ghidra: `FUN_80154430` at `0x80154430` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154430_FUN_80154430.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035435C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035435c`.
pub unsafe fn fn_80154430() -> usize {
    0x8035_435C
}

/// Returns the `Fall` state table pointer `0x8035431C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Fall`.
///
/// # Original
/// - Ghidra: `FUN_80154440` at `0x80154440` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154440_FUN_80154440.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035431C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035431c`.
pub unsafe fn fn_80154440() -> usize {
    0x8035_431C
}

/// Returns the `Jump` state table pointer `0x8035429C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Jump`.
///
/// # Original
/// - Ghidra: `FUN_80154450` at `0x80154450` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154450_FUN_80154450.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035429C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035429c`.
pub unsafe fn fn_80154450() -> usize {
    0x8035_429C
}

/// Returns the `Walk` state table pointer `0x8035425C`.
///
/// In plain English: a constant getter used for
/// `daYoshi_c::StateID_Walk`.
///
/// # Original
/// - Ghidra: `FUN_80154460` at `0x80154460` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154460_FUN_80154460.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x8035425C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None; the address is a constant in `.data`.
///
/// # Confidence
/// high - the export is a single `return &DAT_8035425c`.
pub unsafe fn fn_80154460() -> usize {
    0x8035_425C
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: one of the six registration thunks that the wave-100
/// state descriptors store as their `dConnect_c` callbacks. The operand order
/// is the same as [`fn_800dd1c0`].
///
/// # Original
/// - Ghidra: `FUN_801533d0` at `0x801533D0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801533d0_FUN_801533d0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_801533d0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: the same registration thunk as [`fn_801533d0`], used by a
/// different descriptor.
///
/// # Original
/// - Ghidra: `FUN_80159eb0` at `0x80159EB0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159eb0_FUN_80159eb0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_80159eb0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: the same registration thunk as [`fn_801533d0`], used by a
/// different descriptor.
///
/// # Original
/// - Ghidra: `FUN_80159ee0` at `0x80159EE0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159ee0_FUN_80159ee0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_80159ee0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: the same registration thunk as [`fn_801533d0`], used by a
/// different descriptor.
///
/// # Original
/// - Ghidra: `FUN_8015b2c0` at `0x8015B2C0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015b2c0_FUN_8015b2c0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_8015b2c0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0x18);
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: the same registration thunk as [`fn_801533d0`], used by a
/// different descriptor.
///
/// # Original
/// - Ghidra: `FUN_8015b2f0` at `0x8015B2F0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015b2f0_FUN_8015b2f0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_8015b2f0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Forwards `(second, second, first)` to `0x802DCBFC`.
///
/// In plain English: the same registration thunk as [`fn_801533d0`], used by a
/// different descriptor.
///
/// # Original
/// - Ghidra: `FUN_8015dda0` at `0x8015DDA0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015dda0_FUN_8015dda0.c`
///
/// # Arguments
/// * `first` - object whose embedded descriptor supplies the `r12` interface.
/// * `second` - forwarded twice.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802DCBFC` call.
///
/// # Safety
/// `first` must be a live object with a valid embedded descriptor.
///
/// # Confidence
/// high - the body is byte-identical to [`fn_800dd1c0`].
pub unsafe fn fn_8015dda0(first: u32, second: u32) {
    let _ = interface_call(first as usize, second, 0xc);
}

/// Registers the nineteen `daYoshi_c` state descriptors.
///
/// In plain English: for each of the nineteen Yoshi states, read the nine
/// handler words from the matching 0x24-byte source record, name the
/// descriptor, install the state vtable, copy the handler words and register
/// the descriptor with its release callback and link word. The eleven
/// riding/physics states in the middle also store a state table pointer at
/// `+0x30`, produced by the `0x801543C0..0x80154460` getters.
///
/// # Original
/// - Ghidra: `FUN_80153400` at `0x80153400` (3832 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80153400_FUN_80153400.c`
///
/// # Arguments
/// None. Every descriptor, source record and name string is an absolute global.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills the nineteen 0x38-byte descriptors at `0x80376AC8..0x80376F48` and
/// registers each through `0x802DC780`.
///
/// # Safety
/// The descriptor ranges, the source records and the name strings must be
/// readable/writable.
///
/// # Confidence
/// high - all nineteen blocks share one shape, every address is literal, and
/// the handler, vtable, support and source of each block are read directly from
/// the export. The `+0x30` word exists only for the eleven middle blocks, as
/// observed.
pub unsafe fn fn_80153400() {
    // AloneWait: outer state, nine handler words, no table pointer.
    w100_install_yoshi_state(
        0x8037_6AC8,
        0x8032_6FB4, // s_daYoshi_c__StateID_AloneWait
        0x8037_6AB8,
        W100_VTABLE_803271F0,
        0x8032_6D08,
        W100_HANDLER_80154300,
        None,
    );
    // BlockOut.
    w100_install_yoshi_state(
        0x8037_6B08,
        0x8032_6FD4, // s_daYoshi_c__StateID_BlockOut
        0x8037_6AF8,
        W100_VTABLE_803271F0,
        0x8032_6D2C,
        W100_HANDLER_80154300,
        None,
    );
    // DamageRun.
    w100_install_yoshi_state(
        0x8037_6B48,
        0x8032_6FF0, // s_daYoshi_c__StateID_DamageRun
        0x8037_6B38,
        W100_VTABLE_803271F0,
        0x8032_6D50,
        W100_HANDLER_80154300,
        None,
    );
    // StartRideOn.
    w100_install_yoshi_state(
        0x8037_6B88,
        0x8032_7010, // s_daYoshi_c__StateID_StartRideOn
        0x8037_6B78,
        W100_VTABLE_803271F0,
        0x8032_6D74,
        W100_HANDLER_80154300,
        None,
    );
    // Walk: the eleven riding/physics states start here.
    w100_install_yoshi_state(
        0x8037_6BC4,
        0x8032_7030, // s_daYoshi_c__StateID_Walk
        0x8037_6BB8,
        W100_VTABLE_803271BC,
        0x8032_6D98,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154460)),
    );
    // Jump.
    w100_install_yoshi_state(
        0x8037_6C04,
        0x8032_7048, // s_daYoshi_c__StateID_Jump
        0x8037_6BF8,
        W100_VTABLE_803271BC,
        0x8032_6DBC,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154450)),
    );
    // Fall.
    w100_install_yoshi_state(
        0x8037_6C44,
        0x8032_7060, // s_daYoshi_c__StateID_Fall
        0x8037_6C38,
        W100_VTABLE_803271BC,
        0x8032_6DE0,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154440)),
    );
    // Land.
    w100_install_yoshi_state(
        0x8037_6C84,
        0x8032_7078, // s_daYoshi_c__StateID_Land
        0x8037_6C78,
        W100_VTABLE_803271BC,
        0x8032_6E04,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154430)),
    );
    // Crouch.
    w100_install_yoshi_state(
        0x8037_6CC4,
        0x8032_7090, // s_daYoshi_c__StateID_Crouch
        0x8037_6CB8,
        W100_VTABLE_803271BC,
        0x8032_6E28,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154420)),
    );
    // SitJump.
    w100_install_yoshi_state(
        0x8037_6D04,
        0x8032_70AC, // s_daYoshi_c__StateID_SitJump
        0x8037_6CF8,
        W100_VTABLE_803271BC,
        0x8032_6E4C,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154410)),
    );
    // Slip.
    w100_install_yoshi_state(
        0x8037_6D44,
        0x8032_70C8, // s_daYoshi_c__StateID_Slip
        0x8037_6D38,
        W100_VTABLE_803271BC,
        0x8032_6E70,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_80154400)),
    );
    // Turn.
    w100_install_yoshi_state(
        0x8037_6D84,
        0x8032_70E0, // s_daYoshi_c__StateID_Turn
        0x8037_6D78,
        W100_VTABLE_803271BC,
        0x8032_6E94,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_801543F0)),
    );
    // HipAttack.
    w100_install_yoshi_state(
        0x8037_6DC4,
        0x8032_70F8, // s_daYoshi_c__StateID_HipAttack
        0x8037_6DB8,
        W100_VTABLE_803271BC,
        0x8032_6EB8,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_801543E0)),
    );
    // Funsui.
    w100_install_yoshi_state(
        0x8037_6E04,
        0x8032_7118, // s_daYoshi_c__StateID_Funsui
        0x8037_6DF8,
        W100_VTABLE_803271BC,
        0x8032_6EDC,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_801543D0)),
    );
    // Cloud.
    w100_install_yoshi_state(
        0x8037_6E44,
        0x8032_7134, // s_daYoshi_c__StateID_Cloud
        0x8037_6E38,
        W100_VTABLE_803271BC,
        0x8032_6F00,
        W100_HANDLER_80154360,
        Some(call0_r(W100_FN_801543C0)),
    );
    // EatNone: the four outer "eat" states return to the 0x803271F0 vtable.
    w100_install_yoshi_state(
        0x8037_6E88,
        0x8032_7150, // s_daYoshi_c__StateID_EatNone
        0x8037_6E78,
        W100_VTABLE_803271F0,
        0x8032_6F24,
        W100_HANDLER_80154300,
        None,
    );
    // Eat.
    w100_install_yoshi_state(
        0x8037_6EC8,
        0x8032_716C, // s_daYoshi_c__StateID_Eat
        0x8037_6EB8,
        W100_VTABLE_803271F0,
        0x8032_6F48,
        W100_HANDLER_80154300,
        None,
    );
    // EatOut.
    w100_install_yoshi_state(
        0x8037_6F08,
        0x8032_7184, // s_daYoshi_c__StateID_EatOut
        0x8037_6EF8,
        W100_VTABLE_803271F0,
        0x8032_6F6C,
        W100_HANDLER_80154300,
        None,
    );
    // EatMouth.
    w100_install_yoshi_state(
        0x8037_6F48,
        0x8032_71A0, // s_daYoshi_c__StateID_EatMouth
        0x8037_6F38,
        W100_VTABLE_803271F0,
        0x8032_6F90,
        W100_HANDLER_80154300,
        None,
    );
}

/// Registers the six `Pausewindow_c` state descriptors.
///
/// In plain English: copy four SDA2 words into the globals `0x803279D0..D8` and
/// `0x803279C4`, register the "no state" sentinel, clear the state-callback
/// slot, then install the six pause descriptors `InitWait` through `CloseAni`
/// with the shared vtable and release callback.
///
/// # Original
/// - Ghidra: `FUN_8015b320` at `0x8015B320` (1276 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015b320_FUN_8015b320.c`
///
/// # Arguments
/// None. Every descriptor, name and source record is an absolute global.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the four globals above, the `0x8042A338` callback slot and the six
/// 0x30-byte descriptors at `0x80377180..0x803772C0`.
///
/// # Safety
/// The globals, the descriptors and the six source records must be
/// readable/writable.
///
/// # Confidence
/// high - all six blocks, their sources, names, links and the shared vtable and
/// callback are literal; the four SDA2 words are copied by address, so no value
/// is guessed.
pub unsafe fn fn_8015b320() {
    write_global_word(W100_G_803279D0, read_global_word(W100_SDA2_54A0));
    write_global_word(W100_G_803279D4, read_global_word(W100_SDA2_549C));
    write_global_word(W100_G_803279D8, read_global_word(W100_SDA2_5498));
    write_global_word(W100_G_803279C4, read_global_word(W100_SDA2_6114));

    // observed: the sentinel registration passes a null descriptor.
    let _ = platform::call3(FN_802DC780, 0, 0x8015_B820, 0x8037_7168);
    write_global_word(W100_SDA_5348, read_global_word(W100_SDA2_60FC));

    w100_install_pause_state(
        PAUSE_STATE_INIT_WAIT,
        0x8032_7F68, // s_Pausewindow_c__StateID_InitWait
        0x8037_7174,
        0x8032_7E90,
    );
    w100_install_pause_state(
        PAUSE_STATE_OPEN_ANIME,
        0x8032_7F88, // s_Pausewindow_c__StateID_OpenAnime
        0x8037_71B0,
        0x8032_7EB4,
    );
    w100_install_pause_state(
        PAUSE_STATE_BUTTON_CHANGE,
        0x8032_7FB0, // s_Pausewindow_c__StateID_ButtonChange
        0x8037_71F0,
        0x8032_7ED8,
    );
    w100_install_pause_state(
        PAUSE_STATE_PAUSE_DISP,
        0x8032_7FE0, // s_Pausewindow_c__StateID_PauseDisp
        0x8037_7230,
        0x8032_7EFC,
    );
    w100_install_pause_state(
        PAUSE_STATE_HIT_ANIME_END,
        0x8032_8004, // s_Pausewindow_c__StateID_HitAnimeEnd
        0x8037_7270,
        0x8032_7F20,
    );
    w100_install_pause_state(
        PAUSE_STATE_CLOSE_ANI,
        0x8032_802C, // s_Pausewindow_c__StateID_CloseAni
        0x8037_72B0,
        0x8032_7F44,
    );
}

/// Registers the twenty-four `dScBoot_c` state descriptors.
///
/// In plain English: register the two actor lists `d_enemies` and
/// `d_en_boss`, install the two SDA word tables, then install the twenty-four
/// boot-scene states `ResetWait` through `ErrorInfinity` with the shared vtable
/// and the boot release callback [`fn_8015f040`].
///
/// # Original
/// - Ghidra: `FUN_8015ddd0` at `0x8015DDD0` (4716 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015ddd0_FUN_8015ddd0.c`
///
/// # Arguments
/// None. Every descriptor, name and source record is an absolute global.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three `0x8008F870` list registrations, two `0x8015F600` table copies,
/// twenty-five `0x802DC780` registrations and the twenty-four 0x30-byte
/// descriptors at `0x80377408..0x803779C8`.
///
/// # Safety
/// The descriptor ranges, source records, name strings and the two SDA tables
/// must be readable/writable.
///
/// # Confidence
/// high - all twenty-four blocks share one shape and every address is literal;
/// the SDA blocks are passed by address, so no value is guessed.
pub unsafe fn fn_8015ddd0() {
    let _ = platform::call3(
        W100_FN_8008F870,
        0x8037_7300,
        (SDA_BASE - 0x6280) as u32,
        0,
    );
    let _ = platform::call3(
        FN_802DC780,
        0x8037_7300,
        W100_FN_8008F810 as u32,
        0x8037_72F0,
    );
    let _ = platform::call3(
        W100_FN_8008F870,
        0x8037_7358,
        0x8032_8640, // s_d_enemies
        0,
    );
    let _ = platform::call3(
        FN_802DC780,
        0x8037_7358,
        W100_FN_8008F810 as u32,
        0x8037_7348,
    );
    let _ = platform::call3(
        W100_FN_8008F870,
        0x8037_73B0,
        0x8032_864C, // s_d_en_boss
        0,
    );
    let _ = platform::call3(
        FN_802DC780,
        0x8037_73B0,
        W100_FN_8008F810 as u32,
        0x8037_73A0,
    );
    // observed: two word tables are copied into the SDA area at r13-0x5338
    // (count 3) and r13-0x5328 (count 0x20).
    let _ = platform::call3(
        W100_FN_8015F600,
        (SDA_BASE - 0x5338) as u32,
        0x8032_8090,
        3,
    );
    let _ = platform::call3(
        W100_FN_8015F600,
        (SDA_BASE - 0x5328) as u32,
        0x8032_80E0,
        0x20,
    );

    // ResetWait.
    w93_register_state(
        0x8037_7408,
        0x8032_8658, // s_dScBoot_c__StateID_ResetWait
        W100_HANDLER_8015F040,
        0x8037_73F8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_82E0),
    );
    // ResetFadeOut.
    w93_register_state(
        0x8037_7448,
        0x8032_8678, // s_dScBoot_c__StateID_ResetFadeOut
        W100_HANDLER_8015F040,
        0x8037_7438,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8304),
    );
    // ResetFadeIn.
    w93_register_state(
        0x8037_7488,
        0x8032_8698, // s_dScBoot_c__StateID_ResetFadeIn
        W100_HANDLER_8015F040,
        0x8037_7478,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8328),
    );
    // FadeOutWait.
    w93_register_state(
        0x8037_74C8,
        0x8032_86B8, // s_dScBoot_c__StateID_FadeOutWait
        W100_HANDLER_8015F040,
        0x8037_74B8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_834C),
    );
    // WiiStrapKeyWait.
    w93_register_state(
        0x8037_7508,
        0x8032_86D8, // s_dScBoot_c__StateID_WiiStrapKeyWa
        W100_HANDLER_8015F040,
        0x8037_74F8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8370),
    );
    // WiiStrapDispEnd.
    w93_register_state(
        0x8037_7548,
        0x8032_86FC, // s_dScBoot_c__StateID_WiiStrapDispE
        W100_HANDLER_8015F040,
        0x8037_7538,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8394),
    );
    // WiiStrapFadeOut.
    w93_register_state(
        0x8037_7588,
        0x8032_8724, // s_dScBoot_c__StateID_WiiStrapFadeO
        W100_HANDLER_8015F040,
        0x8037_7578,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_83B8),
    );
    // ControllerInfo, first variant.
    w93_register_state(
        0x8037_75C8,
        0x8032_8748, // s_dScBoot_c__StateID_ControllerInf
        W100_HANDLER_8015F040,
        0x8037_75B8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_83DC),
    );
    // ControllerInfo, second variant.
    w93_register_state(
        0x8037_7608,
        0x8032_8778, // s_dScBoot_c__StateID_ControllerInf
        W100_HANDLER_8015F040,
        0x8037_75F8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8400),
    );
    // ControllerInfo, third variant.
    w93_register_state(
        0x8037_7648,
        0x8032_87B0, // s_dScBoot_c__StateID_ControllerInf
        W100_HANDLER_8015F040,
        0x8037_7638,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8424),
    );
    // ControllerInfo, fourth variant.
    w93_register_state(
        0x8037_7688,
        0x8032_87E0, // s_dScBoot_c__StateID_ControllerInf
        W100_HANDLER_8015F040,
        0x8037_7678,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8448),
    );
    // NandCommandEnd.
    w93_register_state(
        0x8037_76C8,
        0x8032_8814, // s_dScBoot_c__StateID_NandCommandEn
        W100_HANDLER_8015F040,
        0x8037_76B8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_846C),
    );
    // ExistFileCheck.
    w93_register_state(
        0x8037_7708,
        0x8032_883C, // s_dScBoot_c__StateID_ExistFileChec
        W100_HANDLER_8015F040,
        0x8037_76F8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8490),
    );
    // NandSpaceCheck.
    w93_register_state(
        0x8037_7748,
        0x8032_8860, // s_dScBoot_c__StateID_NandSpaceChec
        W100_HANDLER_8015F040,
        0x8037_7738,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_84B4),
    );
    // CreateFile.
    w93_register_state(
        0x8037_7788,
        0x8032_8884, // s_dScBoot_c__StateID_CreateFile
        W100_HANDLER_8015F040,
        0x8037_7778,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_84D8),
    );
    // Load.
    w93_register_state(
        0x8037_77C8,
        0x8032_88A8, // s_dScBoot_c__StateID_Load
        W100_HANDLER_8015F040,
        0x8037_77B8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_84FC),
    );
    // GoToErrorFade, first variant.
    w93_register_state(
        0x8037_7808,
        0x8032_88C0, // s_dScBoot_c__StateID_GoToErrorFade
        W100_HANDLER_8015F040,
        0x8037_77F8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8520),
    );
    // GoToErrorFade, second variant.
    w93_register_state(
        0x8037_7848,
        0x8032_88E4, // s_dScBoot_c__StateID_GoToErrorFade
        W100_HANDLER_8015F040,
        0x8037_7838,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8544),
    );
    // NewSaveFileDialog.
    w93_register_state(
        0x8037_7888,
        0x8032_8908, // s_dScBoot_c__StateID_NewSaveFileDi
        W100_HANDLER_8015F040,
        0x8037_7878,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_8568),
    );
    // ButtonInputWait.
    w93_register_state(
        0x8037_78C8,
        0x8032_892C, // s_dScBoot_c__StateID_ButtonInputWa
        W100_HANDLER_8015F040,
        0x8037_78B8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_858C),
    );
    // WindowExitWait.
    w93_register_state(
        0x8037_7908,
        0x8032_8950, // s_dScBoot_c__StateID_WindowExitWai
        W100_HANDLER_8015F040,
        0x8037_78F8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_85B0),
    );
    // ProcEnd.
    w93_register_state(
        0x8037_7948,
        0x8032_8974, // s_dScBoot_c__StateID_ProcEnd
        W100_HANDLER_8015F040,
        0x8037_7938,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_85D4),
    );
    // ErrorWindowOn.
    w93_register_state(
        0x8037_7988,
        0x8032_8990, // s_dScBoot_c__StateID_ErrorWindowOn
        W100_HANDLER_8015F040,
        0x8037_7978,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_85F8),
    );
    // ErrorInfinity.
    w93_register_state(
        0x8037_79C8,
        0x8032_89BC, // s_dScBoot_c__StateID_ErrorInfinity
        W100_HANDLER_8015F040,
        0x8037_79B8,
        W100_VTABLE_803289E4,
        w99_descriptor_fields(0x8032_861C),
    );
}

// ---------------------------------------------------------------------------
// Wave 105 - the last 0x800D0000 bodies: effect-record list, water pool,
// four-slot record bands and the replay-interface pair
// ---------------------------------------------------------------------------
//
// The bucket's remaining exports are dominated by paired-single quantisation
// (`unaff_GQR0`/`unaff_GQR3` and `CONCAT13` stubs), which no body below touches.
// The six real functions here repeat three shapes:
//
// * an intrusive record list at `r13 - 0x56B0` whose `0x24`-byte records carry
//   `+0x00` previous, `+0x04` next, `+0x08` self, `+0x0C..+0x14` position,
//   `+0x18` kind, `+0x1C` extra and a `u16` code at `+0x20`;
// * the ten-element water pool at `r13 - 0x5690`, 0x13C bytes per element;
// * the `0x802DCD70`/`0x802DCD74` context pairs used by the replay helpers,
//   already documented in the wave-93 section.

/// Creates or retargets one effect record of the `r13 - 0x56B0` list.
///
/// In plain English: walk the record list looking for a record whose `u16` code
/// at `+0x20` and three-word position at `+0x0C..+0x14` match the arguments;
/// when one matches, only its kind word at `+0x18` is rewritten. When the walk
/// runs out, a 0x24-byte record is allocated, appended to the list tail, and
/// filled with its own self pointer, the position, the kind, the extra word and
/// the code.
///
/// # Original
/// - Ghidra: `FUN_800d0470` at `0x800D0470` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0470_FUN_800d0470.c`
/// - Removed: the modelled `0x802DCD74`/`0x802DCDC0` calls. The disassembly is
///   a leaf that ends in `blr` with no register-save helper, so both calls were
///   spurious.
///
/// # Arguments
/// * `_owner` - the first register of the export; no use is printed.
/// * `position` - three floats, compared first and then copied to `+0x0C..+0x14`.
/// * `code` - `u16` stored at `+0x20`; together with the position it is the
///   search key.
/// * `kind_selector` - `1`, `2`, `3` map to kind `3`, `4`, `5`; anything else
///   maps to `1`.
/// * `extra` - word stored at `+0x1C` of a new record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// On a hit: one `+0x18` store. On a miss: the `0x802B9210(0x24)` allocation,
/// one list append and nine record stores.
///
/// # Safety
/// Every record in the list must keep readable `+0x04`, `+0x08` and `+0x20`
/// words, `position` must have three readable floats, and the list head at
/// [`W93_G_LIST_56B0`] must be writable by `0x80160FE0`.
///
/// # Confidence
/// medium - the search keys, the kind mapping, both write paths and the
/// allocation are literal. The position copy reads Ghidra's `extraout_r4`,
/// which is the incoming `r4`: this function's only caller (`0x800369D0`)
/// passes the same `+0x6E8` position pointer that the search above compares, so
/// `r4` is the `position` argument and not a second object.
pub unsafe fn fn_800d0470(
    _owner: u32,
    position: usize,
    code: u32,
    kind_selector: i32,
    extra: u32,
) {
    let kind = match kind_selector {
        1 => 3,
        2 => 4,
        3 => 5,
        _ => 1,
    };

    let mut node = read_global_word(W93_G_LIST_56B0) as usize;
    loop {
        if node == 0 {
            let allocation = platform::call1(FN_802B9210, 0x24);
            if allocation != 0 {
                let record = allocation as usize;
                write_word(record, 0x00, 0);
                write_word(record, 0x04, 0);
                // observed: `puVar2[2] = puVar2`, so `+0x08` holds the record's
                // own address; [`fn_800d0340`] later unlinks through that word.
                write_word(record, 0x08, allocation);
                let _ = platform::call2(W105_FN_80160FE0, W93_G_LIST_56B0 as u32, allocation);
                write_word(record, 0x0C, read_word(position, 0));
                write_word(record, 0x10, read_word(position, 4));
                write_word(record, 0x14, read_word(position, 8));
                write_half(record, 0x20, code as u16);
                write_word(record, 0x18, kind);
                write_word(record, 0x1C, extra);
            }
            return;
        }

        // observed: `iVar3 = node[2]` is the record, `iVar1 = node[1]` the next
        // link; the export keeps the self pointer instead of the node itself.
        let record = read_word(node, 0x08) as usize;
        node = read_word(node, 0x04) as usize;
        let same_position = read_const_f32(position) == read_const_f32(record + 0x0C)
            && read_const_f32(position + 4) == read_const_f32(record + 0x10)
            && read_const_f32(position + 8) == read_const_f32(record + 0x14);
        if code == read_half(record, 0x20) as u32 && same_position {
            write_word(record, 0x18, kind);
            return;
        }
    }
}

/// Starts one water effect in the first free element of the ten-slot pool.
///
/// In plain English: try the ten 0x13C-byte elements of `pool` in order,
/// starting at `pool + 4`, and hand each one to [`water_effect_spawn`]; the
/// first free element wins and the search stops. When all ten are busy nothing
/// is spawned.
///
/// # Original
/// - Ghidra: `FUN_800d2cf0` at `0x800D2CF0` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2cf0_FUN_800d2cf0.c`
/// - Removed: the modelled `0x802DCDC0` calls on both return paths. It is
///   `_restgpr_27`, so the helper forwards its `r3` argument unchanged; the
///   body now returns the constants directly.
///
/// # Arguments
/// * `scale` - effect scale, forwarded as the `double` first argument of
///   [`water_effect_spawn`].
/// * `pool` - the ten-element pool at `r13 - 0x5690` ([`G_WATER_POOL`]).
/// * `kind` - effect variant forwarded to [`water_effect_spawn`].
/// * `position` - three-word position forwarded to [`water_effect_spawn`].
/// * `param` - byte forwarded to [`water_effect_spawn`].
///
/// # Returns
/// `1` when an element was free, `0` when all ten were busy.
///
/// # Side effects
/// One [`water_effect_spawn`] per tried element.
///
/// # Safety
/// `pool` must be the live pool from [`water_effect_pool_construct`] and every
/// element must be readable for 0x13C bytes.
///
/// # Confidence
/// medium - the ten-slot loop, the `0x13C` stride, the `+4` base and the two
/// result constants are literal. `scale` is the export's `extraout_f1`: both
/// callers (`0x8012FB10`, `0x8012FCF0`) pass a float there and
/// [`water_effect_spawn`] takes it as a `double`.
pub unsafe fn fn_800d2cf0(
    scale: f64,
    pool: usize,
    kind: i32,
    position: usize,
    param: u8,
) -> u32 {
    let mut element = pool + 4;
    let mut index = 0i32;
    loop {
        if water_effect_spawn(scale, element, kind, position, param) != 0 {
            return 1;
        }
        index += 1;
        element += 0x13C;
        if index >= 10 {
            return 0;
        }
    }
}

/// Applies both effect-record bands to one display object and player slot.
///
/// In plain English: for band bit 1 write the `0x80318C78` record through
/// [`fn_800d8bb0`], for band bit 0 write the `0x80318CD8` record through
/// [`fn_800d8e40`], then arm the `+0x3C` and `+0x40` countdown words to `4`
/// for tail bits 1 and 2 after clearing `+0x154` of the display owner.
///
/// # Original
/// - Ghidra: `FUN_800d8b00` at `0x800D8B00` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8b00_FUN_800d8b00.c`
///
/// # Arguments
/// * `object` - effect display object; the export keeps it in the incoming `r3`.
/// * `player` - player slot `0..3`; the export keeps it in the incoming `r4`.
/// * `row` - record index forwarded to both band writers.
/// * `bands` - bit 0 selects [`fn_800d8e40`], bit 1 selects [`fn_800d8bb0`];
///   the same word becomes their flag word.
/// * `tails` - bit 1 arms `+0x3C`, bit 2 arms `+0x40`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to two record writes, two countdown writes and two `0x800A7BF0` calls.
///
/// # Safety
/// `object` must be a live effect display object and `row` must index the
/// `0x80318C78`/`0x80318CD8` pointer tables.
///
/// # Confidence
/// medium - the bit tests, the stores and the two forwarded calls are literal.
/// The object and player arrive through the `0x802DCD74` register-save helper,
/// which leaves `r3`/`r4` untouched; every caller (`0x80060C10`, `0x80060DB0`,
/// `0x8011DCF0`) passes the effect object and the player slot explicitly, and
/// the four registers a five-argument [`fn_800d8bb0`] call needs are unchanged
/// at that point, so the callee receives `(object, player, row, bands, tails)`.
pub unsafe fn fn_800d8b00(
    object: usize,
    player: u32,
    row: i32,
    bands: u32,
    tails: u32,
) {
    if bands & 2 != 0 {
        fn_800d8bb0(object, player as u8, row, bands, tails);
    }
    if bands & 1 != 0 {
        fn_800d8e40(object, player as u8, row, tails);
    }
    if tails & 2 != 0 {
        let _ = platform::call1(W105_FN_800A7BF0, read_global_word(W95_G_80429F10));
        write_word(object, 0x3C, 4);
    }
    if tails & 4 != 0 {
        let _ = platform::call1(W105_FN_800A7BF0, read_global_word(W95_G_80429F10));
        write_word(object, 0x40, 4);
    }
}

/// Applies both effect-record bands to every connected player slot.
///
/// In plain English: for band bit 1 walk the four player slots and write the
/// `0x80318C78` record through [`fn_800d8bb0`] for every connected one; for
/// band bit 0 write the `0x80318CD8` record once with the `0xFF` "all players"
/// marker through [`fn_800d8e40`]. Then arm the `+0x3C` and `+0x40` countdown
/// words for flag bits 1 and 2.
///
/// # Original
/// - Ghidra: `FUN_800d8c60` at `0x800D8C60` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8c60_FUN_800d8c60.c`
///
/// # Arguments
/// * `object` - effect display object; the first context word of the export.
/// * `row` - record index used by both band writers; the second context word,
///   which the only printed caller (`0x800204D0`) passes as `0`.
/// * `bands` - bit 0 selects the single [`fn_800d8e40`] write, bit 1 the
///   per-player [`fn_800d8bb0`] writes.
/// * `flags` - flag word OR-ed into `+0x30` by both writers, and the bit 1/2
///   selector for the `+0x3C`/`+0x40` countdowns.
/// * `extra` - fifth argument forwarded to [`fn_800d8bb0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to four [`fn_800d8bb0`] calls, one [`fn_800d8e40`] call, two countdown
/// writes and two `0x800A7BF0` calls.
///
/// # Safety
/// `object` must be a live effect display object and `row` must index the two
/// record tables; the connected-slot query must be readable.
///
/// # Confidence
/// medium - the loop bound, the per-slot guard, the `0xFFFFFFFF` player marker
/// and every store are literal. The object and row arrive through the
/// `0x802DCD70` save/restore pair; the caller passes both explicitly, so the
/// context split is the same one [`fn_800dfb70`] already documents.
pub unsafe fn fn_800d8c60(
    object: usize,
    row: i32,
    bands: u32,
    flags: u32,
    extra: u32,
) {
    if bands & 2 != 0 {
        let mut player = 0i32;
        while player < 4 {
            if platform::call1(W105_FN_800610E0, player as u32) != 0 {
                fn_800d8bb0(object, player as u8, row, flags, extra);
            }
            player += 1;
        }
    }
    if bands & 1 != 0 {
        fn_800d8e40(object, 0xFF, row, flags);
    }
    if flags & 2 != 0 {
        let _ = platform::call1(W105_FN_800A7BF0, read_global_word(W95_G_80429F10));
        write_word(object, 0x3C, 4);
    }
    if flags & 4 != 0 {
        let _ = platform::call1(W105_FN_800A7BF0, read_global_word(W95_G_80429F10));
        write_word(object, 0x40, 4);
    }
}

/// Forwards to the effect-record slot updater at `0x800D8680`.
///
/// In plain English: the whole body of `0x800D8670` is a four-byte branch to
/// `0x800D8680`, so Ghidra exports the target's body under the thunk's name.
/// The single incoming register is forwarded unchanged.
///
/// # Original
/// - Ghidra: `thunk_FUN_800d8680` at `0x800D8670` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d8670_thunk_FUN_800d8680.c`
///
/// # Arguments
/// * `object` - forwarded to [`fn_800d8680`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Exactly those of [`fn_800d8680`]: up to three slot updates.
///
/// # Safety
/// Same contract as [`fn_800d8680`].
///
/// # Confidence
/// high - `functions.tsv` gives the body four bytes, which can only be the
/// unconditional branch Ghidra folded into the target name, and the target
/// takes exactly one argument.
pub unsafe fn fn_800d8670(object: usize) {
    fn_800d8680(object);
}

/// Sends one record through the scene's replay interface.
///
/// In plain English: take the interface at `owner + 4`, accept it only when its
/// slot `+0x08` recognises the `0x802F86C8` type tag, build a key from `object`
/// through `0x80168400`, and pass that key plus `value` and `mode` to the
/// interface's slot `+0x50`.
///
/// # Original
/// - Ghidra: `FUN_800dfa40` at `0x800DFA40` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfa40_FUN_800dfa40.c`
///
/// # Arguments
/// * `object` - key owner passed to `0x80168400`; the high word of the
///   `0x802DCD74` context pair.
/// * `owner` - object whose `+0x04` interface is queried; the low word of the
///   same pair.
/// * `value` - third argument of the interface call.
/// * `mode` - fourth argument of the interface call.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One tagged interface query, one key build and one nine-word scratch record
/// handed to slot `+0x50`.
///
/// # Safety
/// `owner + 0x04` must be a live interface when non-null, and the interface
/// must be valid at slot `+0x50`. The export calls that slot even after
/// clearing the interface to `0`; the body keeps that path literally, so the
/// caller must supply a live interface.
///
/// # Confidence
/// medium - the tag query, the key build and the forwarded call are literal and
/// match the already-translated [`fn_800df990`]; the nine-word scratch size
/// comes from the export's `local_24[9]` and its unused tail is `// unknown:`.
pub unsafe fn fn_800dfa40(object: u32, owner: u32, value: u32, mode: u32) {
    let mut interface = read_word(owner as usize, 0x04) as usize;
    let mut valid = false;
    if interface != 0 {
        let mut tag = W93_TAG_802F86C8 as u32;
        if vcall1(interface, 0x08, (&mut tag as *mut u32) as u32) != 0 {
            valid = true;
        }
    }
    if !valid {
        interface = 0;
    }

    let mut record = [0u32; 9];
    record[0] = platform::call1(W93_FN_80168400, object);
    let _ = vcall3(
        interface,
        0x50,
        record.as_mut_ptr() as u32,
        value,
        mode,
    );
}

/// Shows or hides the save slots of one replay record.
///
/// In plain English: take the interface at `owner + 4`, accept it only when its
/// slot `+0x08` recognises the `0x802F8FF0` type tag, then either publish the
/// record id directly (`mode == 0`) or publish the id of every child of the
/// interface's `+0xE8` container and then the container's own id
/// (`mode == 1`).
///
/// # Original
/// - Ghidra: `FUN_800dff30` at `0x800DFF30` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dff30_FUN_800dff30.c`
/// - Removed: the modelled `0x802DCD70`/`0x802DCDBC` calls. They are the
///   `_savegpr_26`/`_restgpr_26` compiler helpers (register spill/restore
///   only), so the owner and id are the incoming `r3`/`r4`.
///
/// # Arguments
/// * `owner` - object whose `+0x04` interface is queried and whose banner the
///   slot visibility is set on (incoming `r3`).
/// * `id` - replay record id (incoming `r4`).
/// * `visible` - forwarded unchanged as the visibility word.
/// * `mode` - `0` publishes only `id`, `1` walks the container; other values
///   publish nothing.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One tagged interface query, one `0x8023B2A0` container lookup, one
/// [`fn_800dfd20`] count and one [`save_banner_set_slot_visible`] call per
/// child, plus the trailing id.
///
/// # Safety
/// `owner + 0x04` must be a live interface and the `+0xE8` container must hold
/// readable `+0x0C` ids. `0x800DFDD0` is called by address because its own
/// export prints neither a store nor a return, while this caller uses its
/// result.
///
/// # Confidence
/// medium - the tag query, the container walk, the count loop, both id reads and
/// the mode split are literal; the `+0xE8` container layout is `// inferred:`
/// from [`fn_800dfe60`], which reads the same word before its `0x8023B2A0`
/// lookup.
pub unsafe fn fn_800dff30(owner: u32, id: u32, visible: u32, mode: i32) {
    let mut interface = read_word(owner as usize, 0x04) as usize;
    let mut valid = false;
    if interface != 0 {
        let mut tag = W93_TAG_802F8FF0 as u32;
        if vcall1(interface, 0x08, (&mut tag as *mut u32) as u32) != 0 {
            valid = true;
        }
    }
    if !valid {
        interface = 0;
    }

    if mode == 0 {
        save_banner_set_slot_visible(owner as usize, id as i32, visible);
    } else if mode == 1 {
        let mut container = read_word(interface, 0xE8);
        let mut handle = [0u32; 8];
        handle[0] = platform::call2(
            FN_8023B2A0,
            (&mut container as *mut u32) as u32,
            id,
        );
        let count = fn_800dfd20(handle.as_mut_ptr() as usize);
        let mut index = 0i32;
        while index < count {
            let node = platform::call2(
                W105_FN_800DFDD0,
                handle.as_mut_ptr() as u32,
                index as u32,
            );
            let child = if node == 0 {
                0
            } else {
                read_word(node as usize, 0x0C)
            };
            save_banner_set_slot_visible(owner as usize, child as i32, visible);
            index += 1;
        }
        let last = if handle[0] == 0 {
            0
        } else {
            read_word(handle[0] as usize, 0x0C)
        };
        save_banner_set_slot_visible(owner as usize, last as i32, visible);
    }
}

// ---------------------------------------------------------------------------
// Wave 105 constants: the effect-record list, the water pool and the four
// forwarding callees the bodies above call by address
// ---------------------------------------------------------------------------

/// `0x80160FE0(list, record)`: appends `record` at the tail of the two-word
/// list head `(head, tail)` and returns `1`; see `fn_80160fe0` in
/// `src/resource/registry.rs` for the full body.
const W105_FN_80160FE0: usize = 0x8016_0FE0;

/// `0x800A7BF0(display)`: writes zero to `display + 0x154`; called by both
/// countdown branches of [`fn_800d8b00`] and [`fn_800d8c60`].
const W105_FN_800A7BF0: usize = 0x800A_7BF0;

/// `0x800610E0(slot)`: non-zero when bit `slot` of the `r13 - 0x59D8`
/// controller mask is set, i.e. when that player slot is connected.
const W105_FN_800610E0: usize = 0x8006_10E0;

/// `0x800DFDD0(handle, index)`: returns the `index`-th node of a replay
/// container. Its own export prints neither a store nor a return, so
/// [`fn_800dff30`] calls it by address and uses its result.
const W105_FN_800DFDD0: usize = 0x800D_FDD0;

// ---------------------------------------------------------------------------
// Wave 106 - the remaining 0x80150000 bodies: the course height getter, the
// region counter, the per-slot effect switch and four step/callback thunks
// ---------------------------------------------------------------------------

/// Returns the reference height used by the course scene.
///
/// In plain English: start from the SDA2 constant at `r2 - 0x5658`; when the
/// scene owns a course object at `+0x1524` and `0x8014E9B0(scene)` resolves it,
/// replace that value with the object's virtual `+0x104` getter.
///
/// # Original
/// - Ghidra: `FUN_80151a10` at `0x80151A10` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151a10_FUN_80151a10.c`
///
/// # Arguments
/// * `scene` - live course scene; `+0x1524` is the course object.
///
/// # Returns
/// The reference height as an `f64` (the export returns it through `f1`).
///
/// # Side effects
/// One virtual call when the course object exists; no writes.
///
/// # Safety
/// `scene` must be a live scene, and the object returned by `0x8014E9B0` must
/// have a callable vtable slot `+0x104`.
///
/// # Confidence
/// medium - the guard, the SDA2 load and the virtual slot are literal; the
/// export prints `0x8014E9B0` with no argument, but the wave-95 bodies call the
/// same address with `scene` as its only argument, so that shape is reused.
pub unsafe fn fn_80151a10(scene: usize) -> f64 {
    let mut height = read_const_f32(SDA2_BASE - 0x5658) as f64;

    if read_word(scene, 0x1524) != 0 {
        let object = platform::call1(W95_FN_8014E9B0, scene as u32) as usize;
        if object != 0 {
            // observed: (**(code **)(*(int *)(object + 0x60) + 0x104))()
            let vtable = read_word(object, 0x60) as usize;
            let getter = read_word(vtable, 0x104) as usize;
            height = w99_call1_f32_u32(getter, object as u32) as f64;
        }
    }

    height
}

/// Counts the connected players inside a screen-space region.
///
/// In plain English: read the connected-slot count, then for each of the four
/// player slots take that player's object and, when its `+0x152C`/`+0x1530`
/// position lies inside the region, count it and track the highest and lowest
/// `+0x1530` values. The byte at `region + 0x10` then selects who must be
/// counted: nobody for mode `0`, everybody for mode `1`, otherwise a positive
/// query from `0x8005FD20`.
///
/// # Original
/// - Ghidra: `FUN_80156c30` at `0x80156C30` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156c30_FUN_80156c30.c`
///
/// # Arguments
/// * `scene` - scene whose player objects are tested (the caller's `r3`).
/// * `region` - four floats `(min_x, max_x, max_y, min_y)` at `+0x00..+0x0C`
///   and the mode byte at `+0x10` (the caller's `r4`).
///
/// # Returns
/// `1` when the mode's condition is met, `0` otherwise, left in `r3`.
///
/// # Side effects
/// Up to four player lookups and one `0x8005FD20` query; no writes.
///
/// # Safety
/// `scene` must be a live scene and `region` a readable `0x11`-byte block.
///
/// # Confidence
/// medium - the loop, the four bounds comparisons and the mode switch are
/// literal; the mode's `0x8005FD20` result is only tested with the exact
/// arithmetic the export prints, and the region layout is `// inferred:` from
/// the two position fields it is compared against.
///
/// # Original
/// - Ghidra: `FUN_80156c30` at `0x80156C30` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156c30_FUN_80156c30.c`
/// - Removed: the `bl 0x802DCD70` (`__savegprlr_26`) prologue and the
///   `bl 0x802DCDBC` (`__restgprlr_26`) epilogue; the former only saved the
///   incoming `r3`/`r4`, which are now the `scene`/`region` parameters.
pub unsafe fn fn_80156c30(scene: usize, region: usize) -> u32 {
    let this = scene;
    let area = region;

    let connected = call0_r(W95_FN_801568D0) as i32;
    let mut inside = 0i32;
    let mut slot = 0i32;
    while slot < 4 {
        let player = platform::call2(W95_FN_80156820, this as u32, slot as u32) as usize;
        if player != 0 {
            let x = read_float(player, 0x152C);
            let y = read_float(player, 0x1530);
            if read_float(area, 0x00) <= x
                && x <= read_float(area, 0x04)
                && y <= read_float(area, 0x08)
                && read_float(area, 0x0C) <= y
            {
                inside += 1;
            }
        }
        slot += 1;
    }

    let mode = read_byte(area, 0x10);
    let mut result = 0u32;
    if mode == 0 {
        if inside != 0 {
            result = 1;
        }
    } else if mode == 1 {
        if inside == connected {
            // observed: countLeadingZeros(0x8005FD20() - 1) >> 5 != 0
            let value = call0_r(W106_FN_8005FD20);
            if value.wrapping_sub(1).leading_zeros() >> 5 != 0 {
                result = 1;
            }
        }
    } else if inside != 0 {
        // observed: (int)(((int)(value ^ 1) >> 1) - ((value ^ 1) & value)) < 0
        let value = call0_r(W106_FN_8005FD20);
        let flipped = value ^ 1;
        if ((flipped as i32) >> 1) - ((flipped & value) as i32) < 0 {
            result = 1;
        }
    }

    result
}

/// Re-syncs one effect slot per player from an incoming value array.
///
/// In plain English: for each of the four slots compare the stored value at
/// `wm + 0x3CC + slot*4` with the caller's value; on a change spawn the 1-up
/// effect when the value rose, hide the panes when it became zero, show them
/// when it left zero, then store the new value and rebuild the slot transform
/// through `0x800B3B60` using the per-slot id from `0x802F59D0`.
///
/// # Original
/// - Ghidra: `FUN_80159880` at `0x80159880` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80159880_FUN_80159880.c`
/// - Removed: the `bl 0x802DCD70` (`__savegprlr_26`) prologue and the
///   `bl 0x802DCDBC` (`__restgprlr_26`) epilogue; the prologue only saved the
///   incoming `r3`/`r4`, now the `wm`/`values` parameters.
///
/// # Arguments
/// * `wm` - world-map/display object (the caller's `r3`), owner of the `+0x3CC`
///   counter band.
/// * `values` - pointer to four new counter words (the caller's `r4`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to four effect spawns, up to four pane hide/show calls, four counter
/// stores and four `0x800B3B60` transforms.
///
/// # Safety
/// `wm` must be a live display object and `values` must point at four readable
/// words.
///
/// # Confidence
/// medium - the loop, the comparisons and the stores are literal;
/// `0x802F59D0` is `// inferred:` to be the four per-slot effect ids because
/// its words index the `+0x4BC` word bank, and the 64-bit context split is the
/// same one [`fn_800dfb70`] documents.
pub unsafe fn fn_80159880(wm: usize, values: usize) {
    let this = wm;
    let mut cursor = values;

    let mut slot = 0i32;
    while slot < 4 {
        let offset = 0x3CC + slot as usize * 4;
        let current = read_word(this, offset);
        let next = read_word(cursor, 0);
        if current != next {
            if current < next {
                wm_spawn_1up_effects(this, slot as u32);
            }
            if next == 0 {
                gamescene_hud_hide_panes(this, slot as u32);
            } else if current == 0 {
                gamescene_hud_show_panes(this, slot as u32);
            }

            let id = read_global_word(W106_TABLE_802F59D0 + slot as usize * 4) as usize;
            write_word(this, offset, next);
            // observed: FUN_800b3b60(&counter[slot], r2 - 0x54D0, wm + id*4 + 0x4BC, 0)
            let _ = platform::call4(
                FN_800B3B60,
                (this + offset) as u32,
                (SDA2_BASE - 0x54D0) as u32,
                read_word(this, 0x4BC + id * 4),
                0,
            );
        }
        cursor += 4;
        slot += 1;
    }
}

/// Forwards `(value, target, step)` to the `i32` step helper at `0x8015F3C0`.
///
/// In plain English: `0x8015F3B0` is a four-byte branch, so Ghidra exports the
/// target's body under the thunk's name. The three registers are forwarded.
///
/// # Original
/// - Ghidra: `thunk_FUN_8015f3c0` at `0x8015F3B0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f3b0_thunk_FUN_8015f3c0.c`
///
/// # Arguments
/// * `value`, `target`, `step` - forwarded unchanged.
///
/// # Returns
/// The helper's `1`/`0` arrival flag.
///
/// # Side effects
/// Exactly those of [`crate::game::task::step_int_clamped`].
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - `functions.tsv` gives the body four bytes, which can only be the
/// unconditional branch Ghidra folded into the target name.
pub unsafe fn fn_8015f3b0(value: *mut i32, target: i32, step: i32) -> u32 {
    crate::game::task::step_int_clamped(value, target, step)
}

/// Forwards `(value, target, step)` to the `i32` step helper at `0x8015F420`.
///
/// In plain English: `0x8015F410` is a four-byte branch to the second `i32`
/// step copy, whose body Ghidra prints under the thunk's name.
///
/// # Original
/// - Ghidra: `thunk_FUN_8015f420` at `0x8015F410` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f410_thunk_FUN_8015f420.c`
///
/// # Arguments
/// * `value`, `target`, `step` - forwarded unchanged.
///
/// # Returns
/// The helper's `1`/`0` arrival flag.
///
/// # Side effects
/// Exactly those of [`crate::game::task::step_int_clamped_b`].
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - four-byte body, folded target body in the export.
pub unsafe fn fn_8015f410(value: *mut i32, target: i32, step: i32) -> u32 {
    crate::game::task::step_int_clamped_b(value, target, step)
}

/// Forwards `(target, step, value)` to the `f32` step helper at `0x8015F480`.
///
/// In plain English: `0x8015F470` is a four-byte branch to the float step
/// helper; the two doubles and the value pointer travel unchanged.
///
/// # Original
/// - Ghidra: `thunk_FUN_8015f480` at `0x8015F470` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f470_thunk_FUN_8015f480.c`
///
/// # Arguments
/// * `target`, `step` - forwarded doubles (`f1`, `f2`).
/// * `value` - forwarded in/out `f32` pointer (`r3`).
///
/// # Returns
/// The helper's `1`/`0` arrival flag.
///
/// # Side effects
/// Exactly those of [`crate::game::task::step_float_clamped`].
///
/// # Safety
/// `value` must be a valid writable pointer.
///
/// # Confidence
/// high - four-byte body, folded target body in the export, and the argument
/// order matches [`crate::game::task::step_float_clamped`].
pub unsafe fn fn_8015f470(target: f64, step: f64, value: *mut f32) -> u32 {
    crate::game::task::step_float_clamped(target, step, value)
}

/// Forwards `(a, b)` to the global task callback trampoline at `0x8015F6E0`.
///
/// In plain English: `0x8015F7B0` is a four-byte branch to `0x8015F6E0`, whose
/// export loads the installed callback through `0x8015F6D0` and calls it with
/// both words.
///
/// # Original
/// - Ghidra: `thunk_FUN_8015f6e0` at `0x8015F7B0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f7b0_thunk_FUN_8015f6e0.c`
///
/// # Arguments
/// * `a`, `b` - forwarded unchanged to the installed callback.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Exactly those of [`crate::game::task::invoke_task_callback`].
///
/// # Safety
/// A callback must be installed; the original does not null-check it.
///
/// # Confidence
/// high - four-byte body; the folded target body and its constant
/// `0x80429408` are already documented in `src/game/task.rs`.
pub unsafe fn fn_8015f7b0(a: u32, b: u32) {
    crate::game::task::invoke_task_callback(a, b);
}

// ---------------------------------------------------------------------------
// Wave 106 constants: the per-slot effect id table and the counter query
// ---------------------------------------------------------------------------

/// `0x802F59D0(slot)`: four per-slot effect ids read by [`fn_80159880`].
///
/// `// inferred:` the words index the `wm + id*4 + 0x4BC` word bank handed to
/// `0x800B3B60`; the export only shows the load and the index arithmetic.
const W106_TABLE_802F59D0: usize = 0x802F_59D0;

/// `0x8005FD20()`: unknown condition query used by [`fn_80156c30`] mode `1`
/// and the fall-through mode.
///
/// `// unknown:` the export only shows its result being tested with the
/// `countLeadingZeros` / signed-difference idioms, so no role is claimed.
const W106_FN_8005FD20: usize = 0x8005_FD20;

// ---------------------------------------------------------------------------
// Wave 108 - the last 0x800D0000 bodies: the four-view 2-D transform, the
// cursor/effect grid, the star-colour layout, the effect-table dispatch and
// the ground sampler
// ---------------------------------------------------------------------------
//
// The `0x800D0000` bucket entered this pass with eleven open addresses. The
// bodies below repeat four shapes already documented in earlier waves:
//
// * the four view objects at `0x80377C88` and the four 0x20-byte effect
//   entries at `0x80370D10` (see [`fn_800d9760`]);
// * the SDA2 float bank used by the pause cursor (`r2 - 0x6680..-0x6698`);
// * the `0x802DCD60..0x802DCDB8` CodeWarrior register-save/restore stubs,
//   which are prologue/epilogue markers rather than real calls;
// * the `0x800DAB40`/`0x800DAC40` ground filters of the wave-99 section.
//
// Every `ldexpf`/`CONCAT` block in these exports is the decompiler's failed
// model of a paired-single `psq_l`/`psq_st` save and is omitted, as in the
// wave-105 and wave-106 bodies.

/// Rebuilds the four 2-D effect transforms from the four view objects.
///
/// In plain English: for each of the four view objects at `0x80377C88`, ask
/// `0x802BC8F0` whether it is live. A live view contributes its two position
/// floats and a projection into the matching 0x20-byte entry at
/// `0x80370D10`; every view then contributes its two size floats, the length
/// of that pair, and the heading angle (stored negated). When the layout's
/// mode words are `1` or `5`, the three transform words of every view are
/// masked to their `0x8000` bit. Finally the entry pointer of the current
/// slot (`r13 - 0x521C`) is published at `r13 - 0x7A08`.
///
/// # Original
/// - Ghidra: `FUN_800d0610` at `0x800D0610` (656 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d0610_FUN_800d0610.c`
///
/// # Arguments
/// None. The views, the effect table, the layout and the slot index are all
/// absolute/SDA globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four 0x20-byte effect entries (position, projection, size, length, angle),
/// the three mask words of each view, and the effect-pointer global.
///
/// # Safety
/// The four `0x80377C88` entries must be live views, the layout returned by
/// `0x80107F10` must expose `+0x08`/`+0x0C`, and the SDA slot must be readable.
///
/// # Confidence
/// medium - every load, call, arithmetic step and store is literal. The five
/// SDA2 floats at `r2 - 0x66A0..-0x66B0` are only known by address, so their
/// roles (scale and offset pair) are `// inferred:` from the two expressions
/// they form. The `ldexpf(GQR0)` blocks at head and tail are paired-single
/// save artifacts and are not calls.
pub unsafe fn fn_800d0610() {
    platform::call0(W108_FN_8016F220);

    let y_offset = read_const_f32(W108_F32_66A8);
    let x_scale = read_const_f32(W108_F32_66AC);
    let x_offset = read_const_f32(W108_F32_66B0);
    let y_scale = read_const_f32(W108_F32_66A0);
    let projection_bias = read_const_f32(W108_F32_66A4);

    let mut index = 0usize;
    while index < 4 {
        let view = read_global_word(W93_TABLE_SLOT_VIEWS + index * 4) as usize;
        let entry = W108_EFFECT_TABLE_80370D10 + index * 0x20;

        if (platform::call1(W108_FN_802BC8F0, view as u32) as i32) > 0 {
            let mut position = [0f32; 2];
            let _ = platform::call2(
                W108_FN_802BC8C0,
                position.as_mut_ptr() as u32,
                view as u32,
            );
            let x = position[0];
            let y = position[1];
            write_float(entry, 0x00, x);
            write_float(entry, 0x04, y);
            // observed: the projection word +0x0C is written before +0x08.
            write_float(entry, 0x0C, projection_bias + y_scale * (y_offset + y));
            write_float(entry, 0x08, x_offset + x_scale * (y_offset + x));
        }

        let width = w99_call1_f32_u32(W108_FN_802BD250, view as u32);
        let height = w99_call1_f32_u32(W108_FN_802BD270, view as u32);
        write_float(entry, 0x10, width);
        write_float(entry, 0x14, height);
        let length = call_f64_ret_f64(W99_FN_802BE470, (width * width + height * height) as f64);
        write_float(entry, 0x18, length as f32);

        let heading = call_f64_f64_u32(FN_801611C0, width as f64, height as f64) as u16 as i16;
        write_half(entry, 0x1C, heading.wrapping_neg() as u16);
        index += 1;
    }

    let layout = call0_r(W78_FN_80107F10) as usize;
    let mode_a = read_word(layout, 0x08);
    let mode_b = read_word(layout, 0x0C);
    // observed: the layout is "in game" for mode 1 and "in transition" for 5.
    let suppress = mode_a == 1 || mode_b == 1 || mode_a == 5 || mode_b == 5;
    if suppress {
        let mut view_index = 0usize;
        while view_index < 4 {
            let view = read_global_word(W93_TABLE_SLOT_VIEWS + view_index * 4) as usize;
            write_word(view, 0x18, read_word(view, 0x18) & 0x8000);
            write_word(view, 0x1C, read_word(view, 0x1C) & 0x8000);
            write_word(view, 0x20, read_word(view, 0x20) & 0x8000);
            view_index += 1;
        }
    }

    let slot = read_global_word(W108_W_521C) as i32;
    let entry_pointer =
        W108_EFFECT_TABLE_80370D10.wrapping_add((slot as usize).wrapping_mul(0x20));
    write_global_word(W108_G_EFFECT_PTR_7A08, entry_pointer as u32);
}

/// Places one cursor/manager record on the projected effect grid.
///
/// In plain English: when the manager's group byte matches the state's group
/// byte, sweep a height from the SDA2 start value up to `state + 0x14` minus
/// that start, in steps of `r2 - 0x667C`, using the state's width and angle
/// arrays. At each step compute the projected x/y of the current cell and
/// accept it when the manager's owner lies within the selector's margin of
/// that projection and its vertical position matches the owner's height.
/// On a hit the manager is linked into the state's `+0x10` list (once), its
/// `+0x1C` height is stored, and the width table is walked to derive the
/// `+0x2C` column, `+0x2E` row and `+0x20` remainder fields. Returns `1` on
/// a placement and `0` otherwise.
///
/// # Original
/// - Ghidra: `FUN_800d19e0` at `0x800D19E0` (976 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d19e0_FUN_800d19e0.c`
/// - Removed: the modelled `0x802DCDAC` tail call. It is `_restgpr_22`, so the
///   helper only restores the saved registers and `r3` is the result word.
///
/// # Arguments
/// * `state` - grid state (incoming `r3`): `+0x10` list head, `+0x14` height,
///   `+0x22` group byte, `+0x23` height-mode bit, `+0x28` position array,
///   `+0x2C` width array, `+0x30` angle array, `+0x3C` row count.
/// * `manager` - the record being placed (incoming `r4`): `+0x04` owner,
///   `+0x08` next, `+0x1C` height output, `+0x20` remainder output, `+0x2B`
///   group byte, `+0x2C` column output, `+0x2E` row output.
/// * `selector` - `0` uses `r2 - 0x668C` as the acceptance margin, anything
///   else uses `r2 - 0x6678`.
///
/// # Returns
/// `1` when a grid cell was found, `0` otherwise.
///
/// # Side effects
/// Up to three manager stores, two list stores, and the `0x800B3000` rotation.
///
/// # Safety
/// The state arrays, the owner and every list node must be readable; the
/// manager must have writable `+0x08`, `+0x1C`, `+0x20`, `+0x2C`, `+0x2E`.
///
/// # Confidence
/// medium - the sweep, the three array strides, the rotation call, the clamp
/// test and the list insertion are literal. The state and manager field roles
/// are `// inferred:` from the two sibling bodies [`fn_800d27b0`] and
/// [`fn_800d2690`], which read the same arrays. The fifteen float registers
/// spilled at the head are dead saves and are omitted.
pub unsafe fn fn_800d19e0(state: usize, manager: usize, selector: i32) -> u32 {
    let start = read_const_f32(F32_R2_668C);
    let base_scale = read_const_f32(F32_R2_6698);
    let zero = read_const_f32(F32_SDA2_6680);
    let angle_scale = read_const_f32(W108_F32_6684);
    let step = read_const_f32(W108_F32_667C);
    let margin = if selector == 0 {
        start
    } else {
        read_const_f32(W108_F32_6678)
    };

    let mut found = false;
    let mut chain_value = 0f32;
    let mut column = 0u8;

    if read_byte(manager, 0x2B) & read_byte(state, 0x22) != 0 {
        let owner = read_word(manager, 0x04) as usize;
        let position_base = read_word(state, 0x28) as usize;
        let width_base = read_word(state, 0x2C) as usize;
        let angle_base = read_word(state, 0x30) as usize;

        let owner_x = read_float(owner, 0xAC);
        let owner_y = read_float(owner, 0xB0);
        let owner_offset_x = read_float(owner, 0xD0);
        let owner_offset_y = read_float(owner, 0xD4);
        let upper_layer = owner_y + base_scale * owner_offset_y;

        // observed: the sweep starts at the same SDA2 word used as a clamp by
        // the pause-cursor bodies.
        let mut sweep = start;
        let mut remaining = start;
        let mut accumulated = zero;
        let mut column_offset = 0usize;
        let mut width_index = 0usize;
        let mut angle_index = 0usize;
        let end = read_float(state, 0x14) - start;

        while sweep <= end {
            let width = read_float(width_base, width_index);
            if width <= remaining {
                remaining -= width;
                angle_index += 2;
                accumulated += width;
                column_offset += 8;
                column = column.wrapping_add(1);
                width_index += 4;
            }

            let angle = read_half(angle_base, angle_index) as i16 as f32 * angle_scale;
            let sine = w108_call_f64(W108_FN_80237BD0, angle as f64) as f32;
            let cosine = w108_call_f64(W108_FN_80237C40, angle as f64) as f32;

            let px = read_float(position_base, column_offset);
            let py = read_float(position_base, column_offset + 4);
            let projected_y = py - remaining * cosine;
            let projected_x = remaining * sine + px;

            if (owner_x - projected_x).abs() <= margin
                && ((upper_layer - projected_y).abs() <= start
                    || (owner_y - projected_y).abs() <= start)
            {
                let local_x = owner_offset_x + (px - owner_x);
                let local_y = owner_offset_y + (py - owner_y);
                let mut rotated = [0f32; 2];
                call_f64_f64_f64_ptr(
                    W108_FN_800B3000,
                    local_x as f64,
                    local_y as f64,
                    -(angle as f64),
                    rotated.as_mut_ptr() as u32,
                );

                let chain = accumulated + (rotated[1] - owner_offset_y);
                write_float(manager, 0x1C, chain);

                let mut upper = read_float(state, 0x14);
                if read_byte(state, 0x23) & 1 == 0 {
                    upper += owner_offset_y;
                }
                if base_scale * owner_offset_y <= chain && chain <= upper {
                    found = true;
                    chain_value = chain;
                    break;
                }
            }

            sweep += step;
            remaining += step;
        }

        if found {
            let mut node = read_word(state, 0x10) as usize;
            let mut insert = true;
            while node != 0 {
                if node == manager {
                    insert = false;
                    break;
                }
                node = read_word(node, 0x08) as usize;
            }
            if insert {
                write_word(manager, 0x08, read_word(state, 0x10));
                write_word(state, 0x10, manager as u32);
            }

            write_byte(manager, 0x2C, column);

            let mut chain = chain_value - owner_offset_y;
            let mut rows_left = read_byte(state, 0x3C);
            let mut row = 0u8;
            loop {
                rows_left = rows_left.wrapping_sub(1);
                if rows_left == 0 {
                    break;
                }
                let width = read_float(width_base, row as usize * 4);
                if !(width < chain) {
                    break;
                }
                chain -= width;
                row = row.wrapping_add(1);
            }
            write_byte(manager, 0x2E, row);
            let remainder = chain;
            write_float(manager, 0x20, remainder);
        }
    }

    found as u32
}

/// Projects one angle and radius onto a target object's screen position.
///
/// In plain English: read the signed 16-bit angle at `this + 0x4C`, scale it by
/// the SDA2 degree factor, take its sine and cosine through the two angle
/// helpers, and write
/// `owner + 0xAC = this + 0x30 - radius * sin` and
/// `owner + 0xB0 = this + 0x34 + radius * cos`, where the radius is
/// `target + 0x1C` and the owner is `target + 0x04`.
///
/// # Original
/// - Ghidra: `FUN_800d2460` at `0x800D2460` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2460_FUN_800d2460.c`
///
/// # Arguments
/// * `this` - placement record: angle at `+0x4C`, origin at `+0x30`/`+0x34`.
/// * `target` - object with the owner at `+0x04` and the radius at `+0x1C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two float stores on the owner.
///
/// # Safety
/// `this`, `target` and the owner returned by `target + 0x04` must be readable
/// and the owner's `+0xAC`/`+0xB0` writable.
///
/// # Confidence
/// medium - the arithmetic and both stores are literal. The angle is a
/// paired-single quantised load; it is modelled as a signed 16-bit angle,
/// matching the `psq_l`/GQR3 path in the export (the `CONCAT` branches are
/// decompiler artifacts). The identity of `0x80237BD0`/`0x80237C40` as the
/// sine/cosine pair is shared with [`fn_800dac40`].
pub unsafe fn fn_800d2460(this: usize, target: usize) {
    let angle = read_half(this, 0x4C) as i16 as f32 * read_const_f32(W108_F32_6684);
    let sine = w108_call_f64(W108_FN_80237BD0, angle as f64) as f32;
    let cosine = w108_call_f64(W108_FN_80237C40, angle as f64) as f32;

    let owner = read_word(target, 0x04) as usize;
    let radius = read_float(target, 0x1C);
    write_float(owner, 0xAC, read_float(this, 0x30) - radius * sine);
    write_float(owner, 0xB0, read_float(this, 0x34) + radius * cosine);
}

/// Re-seats the grid origin and projects every active grid column.
///
/// In plain English: copy the owner's `+0xAC`/`+0xB0` position into the first
/// slot of the context's position array, then for each of the context's `+0x3C`
/// columns take the `+0x30` angle and `+0x2C` radius, and write the projected
/// point into that column: `+8 = x + radius*sin` and
/// `+0xC = y - radius*cos`. Finally run [`fn_800d27b0`].
///
/// # Original
/// - Ghidra: `FUN_800d2690` at `0x800D2690` (276 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d2690_FUN_800d2690.c`
/// - Removed: the modelled `0x802DCDB0` call. It is `_restgpr_23`, so the
///   helper only restores the saved registers; the context is the incoming
///   `r3`.
///
/// # Arguments
/// * `context` - grid context (incoming `r3`): `+0x04` owner, `+0x28`
///   position array, `+0x2C` radius array, `+0x30` angle array, `+0x3C` count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two position words at slot 0, two stores per column and one
/// [`fn_800d27b0`] pass.
///
/// # Safety
/// All four arrays must cover `+0x3C` entries and the owner must expose
/// `+0xAC`/`+0xB0`.
///
/// # Confidence
/// medium - the copy, the loop bound, both strides and both stores are literal.
/// The angle is modelled as a signed 16-bit `psq_l` result (the `CONCAT` paths
/// are decompiler artifacts).
pub unsafe fn fn_800d2690(context: usize) {
    let owner = read_word(context, 0x04) as usize;
    let position_base = read_word(context, 0x28) as usize;
    write_word(position_base, 0x00, read_word(owner, 0xAC));
    write_word(position_base, 0x04, read_word(owner, 0xB0));

    let count = read_byte(context, 0x3C);
    let radius_base = read_word(context, 0x2C) as usize;
    let angle_base = read_word(context, 0x30) as usize;
    let angle_scale = read_const_f32(W108_F32_6684);

    let mut index = 0u32;
    while index < count as u32 {
        let angle = read_half(angle_base, index as usize * 2) as i16 as f32 * angle_scale;
        let sine = w108_call_f64(W108_FN_80237BD0, angle as f64) as f32;
        let cosine = w108_call_f64(W108_FN_80237C40, angle as f64) as f32;
        let radius = read_float(radius_base, index as usize * 4);
        let offset = index as usize * 8;
        let x = read_float(position_base, offset);
        let y = read_float(position_base, offset + 4);
        write_float(position_base, offset + 8, x + radius * sine);
        write_float(position_base, offset + 0x0C, y - radius * cosine);
        index += 1;
    }

    // observed: the export calls `FUN_800d27b0(iVar8)`; the callee re-fetches
    // the same context, so the argument is forwarded explicitly here.
    fn_800d27b0(context);
}

/// Clamps every linked grid record onto the visible screen box.
///
/// In plain English: walk the context's `+0x10` list. For each record, find the
/// last non-zero column width (walking back while the width is zero), read that
/// column's angle and position, and project the record's height into screen
/// space using the owner's `+0xB8`/`+0xBC` offsets and the `r2 - 0x6694`
/// scale. The result is clamped to the four SDA2 screen bounds and written to
/// the record's `+0x14`/`+0x18`, while the owner's `+0xAC`/`+0xB0` receive the
/// unclamped projection.
///
/// # Original
/// - Ghidra: `FUN_800d27b0` at `0x800D27B0` (560 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d27b0_FUN_800d27b0.c`
/// - Removed: the modelled `0x802DCDB8` calls in both return paths. It is
///   `_restgpr_25` (register restore only).
///
/// # Arguments
/// * `context` - grid context (incoming `r3`): `+0x10` list head, `+0x28`
///   position array, `+0x2C` width array, `+0x30` angle array.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two record stores and three owner stores per node.
///
/// # Safety
/// Every list node, owner and array entry must be readable; the record and
/// owner stores must be writable.
///
/// # Confidence
/// medium - the list walk, the zero-width backtrack, the projection and both
/// clamp pairs are literal; the four SDA2 bounds are reused from the pause
/// cursor (see [`cursor_acquire_target`]), which documents the same words.
pub unsafe fn fn_800d27b0(context: usize) {
    let mut node = read_word(context, 0x10) as usize;
    if node == 0 {
        return;
    }

    let zero = read_const_f32(F32_SDA2_6680);
    let angle_scale = read_const_f32(W108_F32_6684);
    let depth_scale = read_const_f32(F32_R2_6694);
    let vertical_low = read_const_f32(F32_R2_6698);
    let horizontal_low = read_const_f32(F32_R2_6690);
    let horizontal_high = read_const_f32(F32_R2_668C);
    let vertical_high = read_const_f32(F32_R2_6688);
    let position_base = read_word(context, 0x28) as usize;
    let width_base = read_word(context, 0x2C) as usize;
    let angle_base = read_word(context, 0x30) as usize;

    while node != 0 {
        let mut column = read_byte(node, 0x2E) as usize;
        let mut height = read_float(node, 0x20);
        while read_float(width_base, column * 4) == zero && column != 0 {
            height = read_float(node, 0x1C);
            column -= 1;
        }

        let owner = read_word(node, 0x04) as usize;
        let angle = read_half(angle_base, column * 2) as i16 as f32 * angle_scale;
        let sine = w108_call_f64(W108_FN_80237BD0, angle as f64) as f32;
        let cosine = w108_call_f64(W108_FN_80237C40, angle as f64) as f32;

        let total = height + read_float(owner, 0xD4);
        let saved = read_word(owner, 0xB4);
        let base_x = read_float(position_base, column * 8);
        let base_y = read_float(position_base, column * 8 + 4);
        let y = -(total * cosine) + base_y;
        let x = total * sine + base_x;

        let mut screen_x = x - read_float(owner, 0xB8);
        let mut screen_y = vertical_low + depth_scale * (y - read_float(owner, 0xBC));

        if horizontal_low <= screen_x {
            if horizontal_high < screen_x {
                screen_x = horizontal_high;
            }
        } else {
            screen_x = horizontal_low;
        }
        if vertical_low <= screen_y {
            if vertical_high < screen_y {
                screen_y = vertical_high;
            }
        } else {
            screen_y = vertical_low;
        }

        write_float(node, 0x14, screen_x);
        write_float(node, 0x18, screen_y);
        write_float(owner, 0xAC, x);
        write_float(owner, 0xB0, y);
        // observed: the export reloads +0xB4 into `uVar2` and stores it back.
        write_word(owner, 0xB4, saved);

        node = read_word(node, 0x08) as usize;
    }
}

/// Builds the four-player star-colour layout and its animation handles.
///
/// In plain English: for each of the four player rows, resolve the two star
/// handles (`+0x8` and `+0x18` of the row table) through `0x80239E30`, build
/// the left/right widgets through `0x80169D90`, and bind them with
/// [`fn_800d5b90`]. Then read four animation records from `0x8023B2A0`
/// (indices `3`, `4`, `6`, `7`), and store each record's heading and the
/// squared length of its `+0x38..+0x40` vector at the four `+0x7A4..+0x7F0`
/// fields of the row. Finally resolve the shared handle at `r13 - 0x79F8`,
/// build the two shared panes, and bind the `PB_star_color` and
/// `PH_star_color` name tables selected by `+0x180`.
///
/// # Original
/// - Ghidra: `FUN_800d3380` at `0x800D3380` (860 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d3380_FUN_800d3380.c`
/// - Removed: the modelled `0x802DCDB8` call. It is `_restgpr_25` (register
///   restore only); the context is the incoming `r3`.
///
/// # Arguments
/// * `context` - layout context (incoming `r3`): `+0x04` anchor, `+0x20C`
///   animation bank, `+0x210` name bank, `+0x778` row table, `+0x180`
///   star-colour index, and the widget blocks at `+0x28`, `+0x60`, `+0x98`,
///   `+0x468`, `+0x4A8`, `+0x694`, `+0x744`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Per row: two handle lookups, two widget builds, two bindings and four
/// angle/length stores. Then five shared widget builds.
///
/// # Safety
/// The context, its row table and the referenced banks must be live.
///
/// # Confidence
/// medium - every call, offset, index and store is literal. The two widget
/// builders (`0x80169D90`, `0x801650D0`/`0x80165520`/`0x80165C30`) are called
/// by address with the export's exact argument lists; their own parameter
/// names are not claimed. The paired-single blocks around the three float
/// reads are GQR artifacts and are omitted.
pub unsafe fn fn_800d3380(context: usize) {
    let mut selector: u32;
    let table = read_word(context, 0x778) as usize;
    let mut left = context + 0x4A8;
    let mut right = context + 0x468;
    let mut row = 0usize;
    let mut dst = context;

    loop {
        let mut node = [0u32; 1];
        node[0] = platform::call2(
            W108_FN_80239E30,
            (context + 0x20C) as u32,
            read_word(table, row * 4 + 0x18),
        );
        let _ = platform::call6(
            W108_FN_80169D90,
            left as u32,
            node.as_mut_ptr() as u32,
            (context + 4) as u32,
            0x97B,
            1,
            0,
        );
        fn_800d5b90(context, left as u32);

        node[0] = platform::call2(
            W108_FN_80239E30,
            (context + 0x20C) as u32,
            read_word(table, row * 4 + 0x08),
        );
        selector = node[0];
        let _ = platform::call6(
            W108_FN_80169D90,
            right as u32,
            node.as_mut_ptr() as u32,
            (context + 4) as u32,
            0x93B,
            1,
            0,
        );
        fn_800d5b90(context, right as u32);

        let mut record = w108_record_angles(platform::call2(
            FN_8023B2A0,
            (&mut selector as *mut u32) as u32,
            3,
        ) as usize);
        write_float(dst, 0x7E4, record[0]);
        write_float(dst, 0x7EC, record[1]);
        record = w108_record_angles(platform::call2(
            FN_8023B2A0,
            (&mut selector as *mut u32) as u32,
            4,
        ) as usize);
        write_float(dst, 0x7E8, record[0]);
        write_float(dst, 0x7F0, record[1]);
        record = w108_record_angles(platform::call2(
            FN_8023B2A0,
            (&mut selector as *mut u32) as u32,
            6,
        ) as usize);
        write_float(dst, 0x7A4, record[0]);
        write_float(dst, 0x7AC, record[1]);
        record = w108_record_angles(platform::call2(
            FN_8023B2A0,
            (&mut selector as *mut u32) as u32,
            7,
        ) as usize);
        write_float(dst, 0x7A8, record[0]);
        write_float(dst, 0x7B0, record[1]);

        row += 1;
        left += 0x80;
        right += 0x80;
        dst += 0x10;
        if row >= 4 {
            break;
        }
    }

    let handle = platform::call2(
        W93_FN_8023A0B0,
        (context + 0x210) as u32,
        W108_G_79F8 as u32,
    );

    let mut pair_a = [selector, handle];
    let _ = platform::call5(
        W108_FN_801650D0,
        (context + 0x28) as u32,
        pair_a.as_mut_ptr() as u32,
        (pair_a.as_mut_ptr() as u32).wrapping_add(4),
        (context + 4) as u32,
        0,
    );
    let mut pair_b = [selector, handle];
    let _ = platform::call5(
        W108_FN_801650D0,
        (context + 0x60) as u32,
        pair_b.as_mut_ptr() as u32,
        (pair_b.as_mut_ptr() as u32).wrapping_add(4),
        (context + 4) as u32,
        0,
    );
    let mut single = [selector];
    let _ = platform::call5(
        W108_FN_80165520,
        (context + 0x98) as u32,
        single.as_mut_ptr() as u32,
        2,
        (context + 4) as u32,
        0,
    );
    let blend = read_const_f32(W99_F32_664C) as f64;
    let _ = call_f64_u32_u32_u32(
        W108_FN_801656C0,
        blend,
        (context + 0x98) as u32,
        0,
        (context + 0x28) as u32,
    );
    let _ = call_f64_u32_u32_u32(
        W108_FN_801656C0,
        blend,
        (context + 0x98) as u32,
        1,
        (context + 0x60) as u32,
    );

    selector = platform::call2(
        W108_FN_80239E30,
        (context + 0x20C) as u32,
        read_word(table, 0x08),
    );
    let star_index = read_word(context, 0x180) as usize;
    let pb_name = read_global_word(W108_TABLE_PB_STAR_COLOR + star_index * 4);
    let pb_handle = platform::call2(W108_FN_8023A190, (context + 0x210) as u32, pb_name);
    let mut pb_pair = [selector, pb_handle];
    let _ = platform::call6(
        W108_FN_80165C30,
        (context + 0x694) as u32,
        pb_pair.as_mut_ptr() as u32,
        (pb_pair.as_mut_ptr() as u32).wrapping_add(4),
        (context + 4) as u32,
        0,
        1,
    );

    selector = platform::call2(
        W108_FN_80239E30,
        (context + 0x20C) as u32,
        read_word(table, 0x18),
    );
    let ph_name = read_global_word(W108_TABLE_PH_STAR_COLOR + star_index * 4);
    let ph_handle = platform::call2(W108_FN_8023A190, (context + 0x210) as u32, ph_name);
    let mut ph_pair = [selector, ph_handle];
    let _ = platform::call6(
        W108_FN_80165C30,
        (context + 0x744) as u32,
        ph_pair.as_mut_ptr() as u32,
        (ph_pair.as_mut_ptr() as u32).wrapping_add(4),
        (context + 4) as u32,
        0,
        1,
    );
}

/// Dispatches one table-selected 2-D effect through the object's vtable.
///
/// In plain English: take `this + 0x154`; the `countLeadingZeros` idiom maps
/// the value `0x97` to table row `0x99` and every other value to row `0x98`.
/// The row's first float and the SDA2 `r2 - 0x6608` float are passed with the
/// caller's 64-bit word to vtable slot `+0x64`.
///
/// # Original
/// - Ghidra: `FUN_800d6550` at `0x800D6550` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d6550_FUN_800d6550.c`
///
/// # Arguments
/// * `value` - 64-bit word forwarded unchanged as the third vtable argument
///   (incoming `r3:r4`).
/// * `this` - object (incoming `r5`): the selector word at `+0x154` and the
///   vtable at `+0x00`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call at slot `+0x64`.
///
/// # Safety
/// `this` must expose a live vtable with a callable `+0x64` entry.
///
/// # Confidence
/// medium - the selector arithmetic, the `0x80370DCC` row read and the vtable
/// slot are literal. The export's "could not recover jumptable" warning refers
/// to the `countLeadingZeros` branch, which resolves to exactly two rows; the
/// slot's own parameter names are not claimed.
pub unsafe fn fn_800d6550(value: u64, this: usize) {
    let selector = read_word(this, 0x154);
    let row = ((selector.wrapping_sub(0x97).leading_zeros() >> 5) + 0x98) & 0xFF;
    let x = read_const_f32(W108_TABLE_80370DCC + row as usize * 12);
    let y = read_const_f32(W93_F32_SDA2_6608);
    w108_vcall_f64x2_u64(this, 0x64, x as f64, y as f64, value);
}

/// Samples the highest ground band under a point.
///
/// In plain English: clear the output angle, then walk the actor list at
/// `r13 - 0x5664`. A node joins the test when its `+0x4A` team equals `team`,
/// its `+0x45` mask overlaps `mask`, and its `+0x47` mode equals `mode` (or
/// `mode` is `-1`). Nodes with a shape byte below `4` go through
/// [`fn_800dab40`]; shape `4` goes through [`fn_800dac40`]. The deepest hit
/// replaces the current height and angle. When anything hit, the height is
/// written through `out` and the angle through `angle_out`.
///
/// # Original
/// - Ghidra: `FUN_800daa10` at `0x800DAA10` (292 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800daa10_FUN_800daa10.c`
/// - Removed: the modelled `0x802DCDB0` call. It is `_restgpr_23`, so the
///   helper forwards its `r3` argument; the body returns the result word
///   directly.
///
/// # Arguments
/// * `out` - receives the best height (incoming `r3`).
/// * `point` - the sampled x/y pair, passed to both filters (incoming `r4`).
/// * `angle_out` - receives the hit angle; cleared first.
/// * `team` - required `+0x4A` byte.
/// * `mask` - bit mask tested against `+0x45`.
/// * `mode` - required `+0x47` byte; `-1` accepts every mode.
///
/// # Returns
/// `1` when a band hit, `0` otherwise.
///
/// # Side effects
/// One `angle_out` clear, up to two output stores and two list writes by the
/// filters.
///
/// # Safety
/// The actor list and every node must be readable; `point`, `out` and
/// `angle_out` must be valid.
///
/// # Confidence
/// medium - the filters, the four node bytes and both writes are literal.
/// `0x802DCD64` is the `__savegprlr_23` prologue, so its "return" pair is the
/// incoming `r3:r4`; the body takes those two pointers explicitly. The
/// `r2 - 0x6438` default height is read as a float by address.
pub unsafe fn fn_800daa10(
    out: *mut f32,
    point: usize,
    angle_out: *mut u16,
    team: u32,
    mask: u32,
    mode: i32,
) -> u32 {
    let mut best = read_const_f32(W63_W_SDA2_6438);
    let mut best_angle = 0u16;
    *angle_out = 0;

    let mut found = false;
    let mut result = point as u32;
    let mut node = read_global_word(W108_G_5664) as usize;
    while node != 0 {
        let next = read_word(node, 0x08) as usize;
        if team == read_byte(node, 0x4A) as u32
            && mask & read_byte(node, 0x45) as u32 != 0
            && (mode == -1 || mode as u32 == read_byte(node, 0x47) as u32)
        {
            let mut depth = [0f32; 1];
            let shape = read_byte(node, 0x42);
            if shape < 4 {
                result = fn_800dab40(node, point as *mut f32, angle_out, depth.as_mut_ptr());
            } else if shape == 4 {
                result = fn_800dac40(node, point as *mut f32, angle_out as *mut i16, depth.as_mut_ptr());
            }
            if result != 0 {
                found = true;
                if best as f64 <= depth[0] as f64 {
                    best = depth[0];
                    best_angle = *angle_out;
                }
            }
        }
        node = next;
    }

    if found {
        *out = best;
        *angle_out = best_angle;
    }
    found as u32
}

/// Returns the actor record at a given index of the current object's list.
///
/// In plain English: take the head from `(*handle) + 0x60`, then follow the
/// self-relative `+0x64` links `index` times. A zero link ends the walk early.
///
/// # Original
/// - Ghidra: `FUN_800dfdd0` at `0x800DFDD0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dfdd0_FUN_800dfdd0.c`
///
/// # Arguments
/// * `handle` - pointer to the word whose `+0x60` offset yields the head node.
/// * `index` - number of `+0x64` links to follow.
///
/// # Returns
/// The node reached after `index` steps, or `0` when the chain ends.
///
/// # Side effects
/// None.
///
/// # Safety
/// `handle` and every walked node must be readable.
///
/// # Confidence
/// medium - the offset walk is literal, but Ghidra prints the function as
/// `void` and drops the return. The caller [`fn_800dff30`] (and every other
/// caller, for example `0x800DFF30`) consumes `r3` as a record pointer, so the
/// returned node is recovered from those call sites.
pub unsafe fn fn_800dfdd0(handle: usize, index: i32) -> usize {
    let base = read_word(handle, 0) as usize;
    let offset = read_word(base, 0x60);
    let mut node = if offset == 0 {
        0
    } else {
        base.wrapping_add(offset as usize)
    };

    let mut remaining = index;
    while node != 0 && remaining > 0 {
        let step = read_word(node, 0x64);
        node = if step == 0 {
            0
        } else {
            node.wrapping_add(step as usize)
        };
        remaining -= 1;
    }
    node
}

/// Tests one point against the ground bands and places a shadow record.
///
/// In plain English: when the object is not disabled (`+0x3C` bit `0x40000`
/// with an `0x19B` actor, or [`fn_800d9d40`]), snap the point's x through
/// `0x80101AA0`, add the `0x80072DB0` probe height when present, and compute
/// the allowed x interval from the incoming ground height plus the record's
/// `+0x14`/`+0x1C` values. When the record belongs to the same group and team
/// as the manager and the point's x lies in the interval, interpolate the band
/// height from `+0x24`/`+0x28`, apply the flag-selected lateral offset, and
/// accept when the point's y lies inside the `+0x30`/`+0x38` band. On success
/// the manager's `+0x2C` angle, `+0x24` depth, `+0x28` offset and the point's
/// y are written.
///
/// # Original
/// - Ghidra: `FUN_800d9d70` at `0x800D9D70` (952 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d9d70_FUN_800d9d70.c`
/// - Removed: the modelled `0x802DCD68`/`0x802DCDB4` calls. They are the
///   `_savegpr_24`/`_restgpr_24` compiler helpers (register spill/restore
///   only); the helper forwards its `r3` argument, so the body returns the
///   result word directly.
///
/// # Arguments
/// * `this` - band record (incoming `r3`): flags at `+0x3C`, angle at `+0x40`,
///   vertical ramp at `+0x14`/`+0x1C`/`+0x20`/`+0x24`/`+0x28`, lateral limits
///   at `+0x30`/`+0x38`.
/// * `manager` - query record (incoming `r4`): owner at `+0x04`, point at
///   `+0x08`, actor at `+0x18`, outputs at `+0x24`/`+0x28`/`+0x2C`.
/// * `height_gate` - when non-zero the point must be strictly below the band.
/// * `filter` - flag word; bit 0 rejects the record, bit 3 forces a rejection,
///   bits 10/11/19 modify the tolerance.
/// * `side` - when non-zero apply the angle-selected lateral offset.
/// * `ground` - incoming ground height (`f1`).
///
/// # Returns
/// `1` on a hit, `0` otherwise.
///
/// # Side effects
/// Up to four manager stores and one point-y store.
///
/// # Safety
/// The record, manager, owner, actor and point must all be readable and the
/// outputs writable.
///
/// # Confidence
/// medium - the guards, the interval, the interpolation, the flag branches and
/// every store are literal. `0x802DCD68` is the `__savegprlr_24` prologue, so
/// its pair is the incoming `r3:r4`; the body takes the two pointers directly.
/// `0x80101AA0` is the x-snap helper already used by [`fn_800db620`], and its
/// return is kept as the sampled x.
pub unsafe fn fn_800d9d70(
    this: usize,
    manager: usize,
    height_gate: i32,
    filter: u32,
    side: i32,
    ground: f32,
) -> u32 {
    let mut flags = read_word(this, 0x3C);
    let mut filter = filter;
    let position = read_word(manager, 0x08) as usize;
    let actor = read_word(manager, 0x18) as usize;

    if flags & 0x40000 != 0 && read_half(read_word(manager, 0x04) as usize, 0x08) == 0x19B {
        return 0;
    }
    if fn_800d9d40(manager, flags & 0xFFFF) != 0 {
        return 0;
    }

    let mut sample_x = call_f64_ret_f64(FN_80101AA0, read_float(position, 0x00) as f64) as f32;
    let mut sample_y = read_float(position, 0x04);
    // observed: the point's z word is loaded by the export but never read again.
    let _sample_z = read_float(position, 0x08);

    let default = read_const_f32(W99_F32_6448);
    let mut probe = [default; 3];
    if platform::call2(W108_FN_80072DB0, actor as u32, probe.as_mut_ptr() as u32) != 0 {
        sample_y += probe[2];
    }

    let mut low_x = ground + read_float(this, 0x1C);
    let mut high_x = ground + read_float(this, 0x14);
    let mut top = read_float(this, 0x20);
    let mut lateral_low = read_const_f32(W108_F32_6434);
    let mut lateral_high = read_const_f32(W99_F32_6430);
    let mut special = false;
    if flags & 0x80000 == 0 && read_byte(actor, 0xBC) & 8 != 0 {
        lateral_low = read_const_f32(W108_F32_642C);
        special = true;
        lateral_high = read_const_f32(W108_F32_6428);
    }
    if !special && flags & 0x2000 != 0 {
        lateral_low = read_const_f32(W108_F32_6424);
    }

    let mut side = side;
    if side != 0 {
        let angle = read_half(this, 0x40) as i16 as f32 * read_const_f32(W108_F32_6420);
        let sine = w108_call_f64(W108_FN_80237BD0, angle as f64) as f32;
        let cosine = w108_call_f64(W108_FN_80237C40, angle as f64) as f32;
        let offset = read_const_f32(W108_F32_6434);
        let offset_x = offset * cosine;
        let offset_y = offset * sine;
        if flags & 0x800 == 0 {
            low_x -= offset_x;
            top -= offset_y;
        } else {
            side = 0;
        }
        if flags & 0x400 == 0 {
            high_x += offset_x;
        }
    }

    if fn_800d9d00(manager, this) != 0 && low_x <= sample_x && sample_x < high_x {
        let local = sample_x - low_x;
        let ratio = (local * read_float(this, 0x28)) / read_float(this, 0x24);
        let surface = ratio + top;

        let mut accept = false;
        if read_global_word(W108_G_566C) == this as u32 {
            flags &= !2;
        }
        if flags & 1 == 0 {
            if flags & 2 == 0 || read_byte(manager, 0x31) != 0 {
                if flags & 8 == 0 {
                    if flags & 4 != 0
                        && read_float(read_word(manager, 0x04) as usize, 0xEC)
                            < read_float(this, 0x38)
                    {
                        filter = 0;
                    }
                } else {
                    filter = 0;
                }
                if filter & 1 == 0
                    && sample_y <= surface + lateral_high
                    && surface - lateral_low <= sample_y
                    && (height_gate == 0 || sample_y < surface)
                {
                    accept = true;
                }
            } else if sample_y <= surface
                && read_float(this, 0x30)
                    <= read_float(read_word(manager, 0x04) as usize, 0xBC)
            {
                accept = true;
            }
        } else if sample_y <= read_const_f32(W99_F32_6430) + surface
            && (height_gate == 0 || sample_y < surface)
        {
            accept = true;
        }

        if accept {
            write_half(manager, 0x2C, read_half(this, 0x40));
            let distance = call_f64_ret_f64(
                W99_FN_802BE470,
                (local * local + ratio * ratio) as f64,
            ) as f32;
            let mut depth = distance;
            if side != 0 {
                depth = distance - read_const_f32(W108_F32_6434);
            }
            write_float(manager, 0x24, depth);
            write_float(manager, 0x28, probe[2]);
            write_float(position, 0x04, surface - probe[2]);
            return 1;
        }
    }

    0
}

// ---------------------------------------------------------------------------
// Wave 108 helpers and constants
// ---------------------------------------------------------------------------

/// Calls `func(f64) -> f64`; used for the two angular helpers.
#[inline]
unsafe fn w108_call_f64(addr: usize, a: f64) -> f64 {
    let f: extern "C" fn(f64) -> f64 = core::mem::transmute(addr);
    f(a)
}

/// Calls a virtual method with `this`, two doubles and one 64-bit word.
#[inline]
unsafe fn w108_vcall_f64x2_u64(this_arg: usize, slot: usize, a: f64, b: f64, c: u64) {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    let f: extern "C" fn(u32, f64, f64, u64) = core::mem::transmute(entry);
    f(this_arg as u32, a, b, c);
}

/// Reads the heading and squared length of one `0x8023B2A0` animation record.
///
/// The record holds three floats at `+0x38..+0x40`; the heading comes from
/// `0x801C14C0` and the length is the squared norm. The paired-single blocks
/// in the export are the GQR save of the two floats.
#[inline]
unsafe fn w108_record_angles(record: usize) -> [f32; 2] {
    let x = read_float(record, 0x38);
    let y = read_float(record, 0x3C);
    let z = read_float(record, 0x40);
    let mut state = [x, y, z];
    let heading = w99_call1_f32_u32(W108_FN_801C14C0, state.as_mut_ptr() as u32);
    [heading, z * z + x * x + y * y]
}

/// `0x80370D10`: four 0x20-byte 2-D effect entries rebuilt by
/// [`fn_800d0610`].
const W108_EFFECT_TABLE_80370D10: usize = 0x8037_0D10;

/// `0x80317B38`: four pointers to the `PB_star_color*` layout names.
const W108_TABLE_PB_STAR_COLOR: usize = 0x8031_7B38;

/// `0x80317B64`: four pointers to the `PH_star_color*` layout names.
const W108_TABLE_PH_STAR_COLOR: usize = 0x8031_7B64;

/// `0x80370DCC`: the runtime three-float vector table used by
/// [`fn_800d6550`]; entry `n` starts at `+n*0x0C`.
const W108_TABLE_80370DCC: usize = 0x8037_0DCC;

/// `r13 - 0x7A08` -> `0x80427C78`: published pointer to the active effect
/// entry.
const W108_G_EFFECT_PTR_7A08: usize = SDA_BASE - 0x7A08;

/// `r13 - 0x79F8` -> `0x80427C88`: shared animation name passed to
/// `0x8023A0B0` by [`fn_800d3380`]. `// unknown:` exact name.
const W108_G_79F8: usize = SDA_BASE - 0x79F8;

/// `r13 - 0x5664` -> `0x8042A01C`: head of the actor list sampled by
/// [`fn_800daa10`].
const W108_G_5664: usize = SDA_BASE - 0x5664;

/// `r13 - 0x566C` -> `0x8042A014`: record the manager is currently tracking;
/// [`fn_800d9d70`] clears the `0x02` flag when it matches.
const W108_G_566C: usize = SDA_BASE - 0x566C;

/// `r13 - 0x521C` -> `0x8042A464`: active effect slot index.
const W108_W_521C: usize = SDA_BASE - 0x521C;

/// `r2 - 0x66A8`: vertical base of the effect projection.
const W108_F32_66A8: usize = SDA2_BASE - 0x66A8;
/// `r2 - 0x66AC`: horizontal scale of the effect projection.
const W108_F32_66AC: usize = SDA2_BASE - 0x66AC;
/// `r2 - 0x66B0`: horizontal base of the effect projection.
const W108_F32_66B0: usize = SDA2_BASE - 0x66B0;
/// `r2 - 0x66A0`: vertical scale of the effect projection.
const W108_F32_66A0: usize = SDA2_BASE - 0x66A0;
/// `r2 - 0x66A4`: bias added to the vertical projection word.
const W108_F32_66A4: usize = SDA2_BASE - 0x66A4;

/// `r2 - 0x6684`: degree-to-radian factor of the two angular helpers.
const W108_F32_6684: usize = SDA2_BASE - 0x6684;
/// `r2 - 0x667C`: height sweep step of [`fn_800d19e0`].
const W108_F32_667C: usize = SDA2_BASE - 0x667C;
/// `r2 - 0x6678`: alternate acceptance margin of [`fn_800d19e0`].
const W108_F32_6678: usize = SDA2_BASE - 0x6678;

/// `r2 - 0x6420`: angle scale of the lateral offset in [`fn_800d9d70`].
const W108_F32_6420: usize = SDA2_BASE - 0x6420;
/// `r2 - 0x6424`: lateral tolerance used with the `0x2000` flag.
const W108_F32_6424: usize = SDA2_BASE - 0x6424;
/// `r2 - 0x6428`: vertical tolerance of the `+0xBC` bit-3 branch.
const W108_F32_6428: usize = SDA2_BASE - 0x6428;
/// `r2 - 0x642C`: horizontal tolerance of the `+0xBC` bit-3 branch.
const W108_F32_642C: usize = SDA2_BASE - 0x642C;
/// `r2 - 0x6434`: sideways offset and tolerance of [`fn_800d9d70`].
const W108_F32_6434: usize = SDA2_BASE - 0x6434;

/// `0x8016F220()`: head-of-frame effect reset called by [`fn_800d0610`].
/// `// unknown:` the export prints no operand.
const W108_FN_8016F220: usize = 0x8016_F220;

/// `0x802BC8F0(view) -> i32`: live-view test.
const W108_FN_802BC8F0: usize = 0x802B_C8F0;

/// `0x802BC8C0(out, view)`: reads the view's two position floats.
const W108_FN_802BC8C0: usize = 0x802B_C8C0;

/// `0x802BD250(view) -> f32`: first size component of the view.
const W108_FN_802BD250: usize = 0x802B_D250;

/// `0x802BD270(view) -> f32`: second size component of the view.
const W108_FN_802BD270: usize = 0x802B_D270;

/// `0x80237BD0(angle) -> f64`: sine of a degree angle.
const W108_FN_80237BD0: usize = 0x8023_7BD0;

/// `0x80237C40(angle) -> f64`: cosine of a degree angle.
const W108_FN_80237C40: usize = 0x8023_7C40;

/// `0x800B3000(x, y, out, angle)`: rotates the pair `(x, y)` into `out`.
const W108_FN_800B3000: usize = 0x800B_3000;

/// `0x80239E30(bank, handle)`: animation handle resolution.
const W108_FN_80239E30: usize = 0x8023_9E30;

/// `0x80169D90(layout, handle, anchor, id, a, b)`: widget builder.
const W108_FN_80169D90: usize = 0x8016_9D90;

/// `0x801650D0(layout, handle, name, anchor, flag)`: two-word widget builder.
const W108_FN_801650D0: usize = 0x8016_50D0;

/// `0x80165520(layout, handle, count, anchor, flag)`: shared widget builder.
const W108_FN_80165520: usize = 0x8016_5520;

/// `0x801656C0(value, layout, index, target)`: time the shared widget pair.
const W108_FN_801656C0: usize = 0x8016_56C0;

/// `0x80165C30(layout, handle, name, anchor, a, b)`: star-colour builder.
const W108_FN_80165C30: usize = 0x8016_5C30;

/// `0x8023A190(bank, name) -> u32`: name-to-handle lookup.
const W108_FN_8023A190: usize = 0x8023_A190;

/// `0x801C14C0(&x) -> f32`: heading of the three-float vector at `x`.
const W108_FN_801C14C0: usize = 0x801C_14C0;

/// `0x80072DB0(actor, out) -> u32`: reads the actor's three-float probe.
const W108_FN_80072DB0: usize = 0x8007_2DB0;

// ---------------------------------------------------------------------------
// Wave 109 - the last 0x80150000 bodies: the pending-item display pass, the
// interaction request handler, the committed-target selector, the two
// route-HUD layout passes, the band-selected player mask, the two player-band
// scans and the CRC-32 thunk
// ---------------------------------------------------------------------------

/// Re-projects the scene's pending display item every frame.
///
/// In plain English: when the `+0x1558` item exists, read its centre height and
/// the `+0x14D8` manager. In phase `1` attach the item's `+0x04` target and
/// publish the centre height through `0x800FFDB0`. Otherwise refresh the
/// attached interface, replace the height by the item's bottom or top edge
/// depending on the `+0xAC` limit, build the three-float position from that
/// height, the item's centre x at `this + 0x148` and the `+0xB4` value, hand it
/// to `0x80152880`, and publish the height again. Either way the item is
/// cleared through [`fn_801501b0`].
///
/// # Original
/// - Ghidra: `FUN_801502e0` at `0x801502E0` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801502e0_FUN_801502e0.c`
///
/// # Arguments
/// * `this` - the scene; `+0x1558` pending item, `+0x14D8` layout manager,
///   `+0x1560` phase word, `+0x148` collision area, `+0xAC` height limit and
///   `+0xB4` third position component.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One target attachment or one interface refresh, one `0x800FFDB0` store, one
/// `0x80152880` position hand-off and the three stores of [`fn_801501b0`].
///
/// # Safety
/// `this` must be a live scene, `+0x1558` a collision area when non-zero, and
/// `+0x14D8` a valid manager word.
///
/// # Confidence
/// high - the guard, the phase test, the five collision helpers, the three-float
/// stack block and both `0x800FFDB0` calls are literal. `0x800FFDB0` is the
/// `(double, manager)` setter whose own export is `FUN_800ffdb0`, so the
/// wide-double form of the height is preserved.
pub unsafe fn fn_801502e0(this: usize) {
    let item = read_word(this, 0x1558) as usize;
    if item == 0 {
        return;
    }

    let manager = read_word(this, 0x14D8);
    let mut height = crate::game::stage::collision_area_center_y(item) as f64;

    if read_word(this, 0x1560) == 1 {
        crate::game::interaction::wm_attach_target(this, read_word(item, 4) as usize);
        w109_call_f64_u32(W109_FN_800FFDB0, height, manager);
    } else {
        crate::game::worldmap::fn_8014fc50(this);
        if read_float(this, 0xAC) as f64 <= height {
            height = crate::game::stage::collision_area_min_y(item) as f64;
        } else {
            height = crate::game::stage::collision_area_max_y(item) as f64;
        }
        let side = read_float(this, 0xB4) as f64;
        let centre_x = crate::game::stage::collision_area_center_x(this + 0x148) as f64;
        // observed: local_38/local_34/local_30 are three consecutive floats.
        let mut position = [height as f32, centre_x as f32, side as f32];
        crate::game::yoshi::fn_80152880(this, position.as_mut_ptr() as u32);
        w109_call_f64_u32(W109_FN_800FFDB0, height, manager);
    }

    fn_801501b0(this);
}

/// Consumes one interaction request against the player under the cursor.
///
/// In plain English: take `controller + 4` and `link + 4`; when the player's
/// `+0x1524` course word already equals the target's identity word the request
/// is rejected. Otherwise resolve the player object and read the request byte
/// at `link + 0x2D`. Requests `0x13..0x15` only re-post the player's special
/// move when the target belongs to a different player. Requests `7` and `0xD`
/// ask the player's `+0x3F4` slot whether it accepts the target; on acceptance
/// the target's `+0x348` mirror byte is copied from the player, and when the
/// `link + 0x2C` byte is clear the shared `r13 - 0x5678` effect object is
/// updated, the target is attached through `0x8014EA30`, and its `+0x3FC`
/// callback is fired with the two SDA2/`+0x10C` floats. A non-zero `+0x2C`
/// instead posts the plain-player path when the request is `7`.
///
/// # Original
/// - Ghidra: `FUN_80151e50` at `0x80151E50` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80151e50_FUN_80151e50.c`
///
/// # Arguments
/// * `controller` - first context word (`r3`); its `+0x04` word is the source
///   player object.
/// * `link` - second context word (`r4`); its `+0x04` word is the target actor
///   and its `+0x2C`/`+0x2D` bytes are the request flags.
///
/// # Returns
/// `1` when the request was consumed, `0` otherwise.
///
/// # Side effects
/// Up to one special-move post, one effect-object update, one actor attachment,
/// one mirror-byte store and the printed virtual calls.
///
/// # Safety
/// Both context objects, the resolved player and the target must be readable,
/// and every `+0x60` interface must expose the printed slots.
///
/// # Confidence
/// medium - the guards, the request bytes, the five helpers and both virtual
/// slots are literal; the `0x802DCD74` save helper's printed pair is the
/// incoming `r3:r4`, so the two pointers are taken as parameters, and the
/// `+0x3FC` slot's two float parameters are typed as doubles because the export
/// prints them that way.
pub unsafe fn fn_80151e50(controller: usize, link: usize) -> u32 {
    // observed: the export starts with `0x802DCD74`; its printed pair is r3:r4.
    let source = read_word(controller, 4) as usize;
    let target = read_word(link, 4) as usize;
    if read_word(source, 0x1524) == read_word(target, 0) {
        return 0;
    }

    let player = crate::game::player::fn_8005f900(w109_player_index(source) as usize) as usize;
    if player == 0 {
        return 0;
    }

    let action = read_byte(link, 0x2D);
    if action.wrapping_sub(0x13) < 3 {
        if w109_player_index(player) != w109_player_index(target) {
            crate::game::worldmap::fn_80141640(player, action as i32);
        }
        return 1;
    }

    if action == 7 || action == 0xD {
        let accepted = w109_iface_call3(player, 0x3F4, target as u32, 4);
        if accepted != 0 {
            if read_byte(link, 0x2C) == 0 {
                let effects = read_global_word(W109_G_5678);
                fn_800d8bb0(effects as usize, w109_player_index(target), 4, 0, 0);
                if crate::game::worldmap::wm_attach_complete(source, target, 0) != 0 {
                    // observed: `param_2[0xd2]` is a byte store at +0x348.
                    write_byte(target, 0x348, read_byte(source, 0x348));
                    w109_iface_call_f64x2_u32x4(
                        source,
                        0x3FC,
                        read_const_f32(W109_F32_7838) as f64,
                        read_float(source, 0x10C) as f64,
                        source as u32,
                        1,
                        2,
                        0,
                    );
                }
            } else if read_byte(link, 0x2D) == 7 {
                crate::game::player::fn_80048c40(target);
            }
        }
        return 1;
    }

    0
}

/// Selects one target object for the scene and switches the scene's layout.
///
/// In plain English: run the no-argument setup call, resolve the scene's
/// `+0x1520` registry id, wait for a forced commit and require the target's
/// `+0x38C` byte to be `1`. On success hand the incoming `f1` value, the target,
/// the third integer word and the last integer argument to the target's `+0x130`
/// slot and use the target's reported player number as the result. The scene's
/// layout name is then switched to `0x80376E88` and animation `0x8035421C` is
/// started through the `+0x294` slot. The result starts at `-1` and stays there
/// when the commit is rejected.
///
/// # Original
/// - Ghidra: `FUN_80152cf0` at `0x80152CF0` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80152cf0_FUN_80152cf0.c`
///
/// # Arguments
/// * `scene` - the scene (`r3`); its `+0x1520` registry id and `+0x60`
///   interface are used.
/// * `id` - third integer word (`r4`), forwarded to the `+0x130` slot.
/// * `selector` - fifth integer word (`r5`), forwarded to the `+0x130` slot.
/// * `value` - incoming 64-bit `f1`, forwarded to the `+0x130` slot.
///
/// # Returns
/// The target's player number, or `-1` when the commit was rejected.
///
/// # Side effects
/// One setup call, one registry lookup, one commit attempt, up to two virtual
/// calls and one layout switch.
///
/// # Safety
/// `scene` must be a live scene, the resolved registry object must be readable
/// at `+0x38C`, and both printed virtual slots must be callable.
///
/// # Confidence
/// medium - the commit guard, the `+0x38C` test, the two vtable slots and the
/// `-1` default are literal. The export prints `FUN_80050e60()` with no
/// operands, so that call keeps its address; the `+0x130` slot's first argument
/// is typed `f64` because it arrives in the 64-bit `f1` register.
pub unsafe fn fn_80152cf0(scene: usize, id: u32, selector: u32, value: f64) -> i32 {
    // observed: `0x802DCD74` is the register-save helper; the body reads r3 as
    // the scene and forwards r4/r5.
    platform::call0(W109_FN_80050E60); // unknown: the export prints no operands.
    let mut result: i32 = -1;

    let object = crate::resource::registry::fn_80162d00(read_word(scene, 0x1520)) as usize;
    let committed = crate::game::interaction::wm_commit_target(scene, 1);
    if committed != 0 && read_byte(object, 0x38C) == 1 {
        w109_iface_call_f64_u32x3(object, 0x130, value, object as u32, id, selector);
        result = w109_player_index(object) as i8 as i32;
    }

    crate::game::interaction::wm_reset_attachment_phase(scene, W109_G_80376E88 as u32);
    w109_iface_call3(scene, 0x294, W109_G_8035421C as u32, 0);
    result
}

/// Rebuilds the route HUD's screen transform from the current stage entry.
///
/// In plain English: read the `r13 - 0x57E8` table entry for the stage selected
/// by the `r13 - 0x54B8` info block (entry `+0x120E`, zeroed when its `+0x2C`
/// word is zero), then write the four section-4B bytes into `+0x900FE..+0x90103`
/// and the two SDA floats at `r13 - 0x5594`/`-0x5590` into the screen object's
/// `+0x40`/`+0x44` scales and the `r2 - 0x5580` offset into its `+0x48`/`+0x4C`
/// position. The `+0x90098` and `+0x9000C` words receive the `0x80078EB0` zoom
/// and the `r2 - 0x5584` base over that zoom.
///
/// # Original
/// - Ghidra: `FUN_801548a0` at `0x801548A0` (464 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801548a0_FUN_801548a0.c`
///
/// # Arguments
/// * `this` - the route HUD object; the `+0x90000` block is its display state.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Eight HUD byte/float stores, eight screen-object stores and one entry
/// lookup.
///
/// # Safety
/// `this` must be a live HUD object, the two SDA pointers must be readable and
/// the `r13 - 0x5880` screen object writable at `+0x38..+0x7C`.
///
/// # Confidence
/// medium - every store, call and the double/float rounding sequence are
/// literal. `0x80078EB0`'s own export prints `void` while this call site
/// consumes its `f1` result, so it is called by address as a float getter, the
/// ambiguous slot pointed out by the wave-109 brief; the four SDA2 floats keep
/// their observed offsets and the stage entry layout comes from the translated
/// section-4B helpers.
pub unsafe fn fn_801548a0(this: usize) {
    // observed: the wave-63 docs name `r13 - 0x5880` the spin-jump counter; the
    // stores here (`+0x38/+0x3C` position, `+0x40/+0x44` scale) show it is the
    // route screen transform object this pass reuses.
    let screen = read_global_word(W63_G_SPINJUMP) as usize;
    let info = read_global_word(W100_SDA_54B8) as usize;
    let record = read_global_word(W109_G_57E8) as usize
        + read_byte(info, 0x120E) as usize * 0x3B0;
    let stage = if read_word(record, 0x2C) == 0 { 0 } else { record };
    let id = read_byte(info, 0x120F) as u32;

    let scale_x = read_const_f32(W109_F32_5594) as f64;
    let scale_y = read_const_f32(W109_F32_5590) as f64;

    write_byte(this, 0x900FE, crate::game::stage::section4b_byte_e(stage, id));
    write_byte(this, 0x900FF, crate::game::stage::section4b_byte_f(stage, id));
    write_byte(this, 0x90100, crate::game::stage::section4b_byte_10(stage, id));

    let zoom = w99_call1_f32_u32(W109_FN_80078EB0, this as u32) as f64;
    write_float(this, 0x90098, zoom as f32);

    let zoom = w99_call1_f32_u32(W109_FN_80078EB0, this as u32) as f64;
    let base = read_const_f32(W109_F32_5584) as f64;
    let offset = read_const_f32(W109_F32_5580);
    let ratio = (base / zoom) as f32;
    write_float(this, 0x9000C, ratio);

    // observed: `(float)(d * (double)(float)(base / (double)(float)(base / zoom)))`
    let inner = (base / ratio as f64) as f32;
    let scale_x = (scale_x * inner as f64) as f32;
    write_float(screen, 0x40, scale_x);
    let scale_y = (scale_y * inner as f64) as f32;
    write_float(screen, 0x44, scale_y);
    write_float(screen, 0x48, read_float(screen, 0x38) + offset * scale_x);
    write_float(screen, 0x4C, read_float(screen, 0x3C) - offset * scale_y);

    let next = crate::resource::course::section4b_byte_c(stage, id);
    write_byte(this, 0x90101, crate::resource::course::section4b_byte_d(stage, next as u32));
    let next = crate::resource::course::section4b_byte_c(stage, id);
    write_byte(this, 0x90102, crate::game::stage::section4b_byte_14(stage, next as u32));
    let next = crate::resource::course::section4b_byte_c(stage, id);
    write_byte(this, 0x90103, crate::game::stage::section4b_byte_14(stage, next as u32));

    let colour = read_global_word(W109_W_557C);
    let fade = read_global_word(W109_F32_5590);
    write_word(screen, 0x58, colour);
    write_word(screen, 0x5C, colour);
    write_word(screen, 0x78, fade);
    write_word(screen, 0x7C, fade);
}

/// Rebuilds the route HUD's per-slot view rectangle and scale.
///
/// In plain English: resolve the same `r13 - 0x57E8` stage entry, then fill
/// `+0x8FE64..+0x8FE70` from the section-4B record's four floats (`+0x00`,
/// `+0x00 + 0x08`, `+0x04`, `+0x04 - 0x0C`). The two `0x800789A0`/`0x80078B40`
/// edges give the `+0x8FEA0` span and the `+0x8FEA4` rectangle height, which are
/// then copied three times into `+0x90030..+0x9005C`. The smaller of the two
/// aspect ratios is clamped by `r2 - 0x5578` and divides the `r2 - 0x5570`
/// double into `+0x90028`, while `+0x90018` receives `r2 - 0x5584`. The
/// `+0x90102` selector picks the two orientation bytes at `+0x90079`/`+0x9007A`,
/// and the tail runs the five view rebuilders and seeds `+0x8FEC4..+0x8FEC8`.
///
/// # Original
/// - Ghidra: `FUN_80154a70` at `0x80154A70` (648 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154a70_FUN_80154a70.c`
///
/// # Arguments
/// * `this` - the route HUD object; the `+0x8FE64..+0x90BDC` block is its view
///   state.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four section-4B reads, twenty-five view stores, six calls and one entry
/// lookup.
///
/// # Safety
/// `this` must be a live HUD object and the resolved stage entry readable.
///
/// # Confidence
/// medium - the entry resolution, the rectangle, the two aspect ratios, the
/// clamp, the eight-case orientation switch and the tail calls are literal. The
/// caller (`fn_80154740`) passes a slot index and a `1`, but neither word is
/// read anywhere in the export, so they are kept as unused parameters rather
/// than invented; the `+0x90028` value is `// inferred:` to be a scale because
/// `0x80078860` divides by it.
pub unsafe fn fn_80154a70(this: usize, _slot: u32, _flag: u32) {
    let info = read_global_word(W100_SDA_54B8) as usize;
    let record = read_global_word(W109_G_57E8) as usize
        + read_byte(info, 0x120E) as usize * 0x3B0;
    let stage = if read_word(record, 0x2C) == 0 { 0 } else { record };
    let id = read_byte(info, 0x120F) as u32;

    let mut bounds = [0f32; 4];
    if crate::resource::course::section4b_entry_by_id(stage, id, bounds.as_mut_ptr() as usize) != 0 {
        write_float(this, 0x8FE64, bounds[0]);
        write_float(this, 0x8FE68, bounds[0] + bounds[2]);
        write_float(this, 0x8FE6C, bounds[1]);
        write_float(this, 0x8FE70, bounds[1] - bounds[3]);
    }

    let first = crate::game::stage::fn_800789a0(this) as f64;
    let second = crate::game::stage::fn_80078b40(this) as f64;
    let low = read_float(this, 0x8FE70);
    let high = read_float(this, 0x8FE6C);
    let low_bits = read_word(this, 0x8FE64);
    let high_bits = read_word(this, 0x8FE68);

    write_float(this, 0x8FEA0, (second - first) as f32);
    write_float(this, 0x8FEA4, high - low);
    for offset in [0x90030usize, 0x90040, 0x90050] {
        write_word(this, offset, low_bits);
        write_word(this, offset + 4, high_bits);
        write_float(this, offset + 8, high);
        write_float(this, offset + 0xC, low);
    }

    let scale_x = read_const_f32(W109_F32_5594) as f64;
    let scale_y = read_const_f32(W109_F32_5590);
    let ratio_a = (((second - first) as f32) as f64 / scale_x) as f32;
    let mut ratio = ((high - low) / scale_y) as f64;
    if (ratio_a as f64) < ratio {
        ratio = ratio_a as f64;
    }
    let limit = read_const_f32(W109_F32_5578) as f64;
    if limit < ratio {
        ratio = limit;
    }
    write_word(this, 0x90018, read_global_word(W109_F32_5584));
    write_float(this, 0x90028, (w99_read_f64(W109_F64_5570) / ratio) as f32);

    let selector = read_byte(this, 0x90102);
    match selector {
        0 => {
            write_byte(this, 0x90079, 0);
            write_byte(this, 0x9007A, 3);
        }
        1 => {
            write_byte(this, 0x90079, 0);
            write_byte(this, 0x9007A, 1);
        }
        2 => {
            write_byte(this, 0x90079, 2);
            write_byte(this, 0x9007A, 3);
        }
        3 => {
            write_byte(this, 0x90079, 2);
            write_byte(this, 0x9007A, 1);
        }
        4 => {
            write_byte(this, 0x90079, 3);
            write_byte(this, 0x9007A, 0);
        }
        5 => {
            write_byte(this, 0x90079, 3);
            write_byte(this, 0x9007A, 2);
        }
        6 => {
            write_byte(this, 0x90079, 1);
            write_byte(this, 0x9007A, 0);
        }
        7 => {
            write_byte(this, 0x90079, 1);
            write_byte(this, 0x9007A, 2);
        }
        // observed: the export's switch has no default, so nothing is stored.
        _ => {}
    }

    write_byte(this, 0x9002C, 0);
    crate::game::stage::fn_80077b00(this);
    let word = read_global_word(W109_W_5568);
    write_word(this, 0x8FEC4, word);
    write_word(this, 0x8FEC8, word);
    crate::game::stage::fn_8007bef0(this);
    crate::game::stage::fn_80079b60(this);
}

/// Builds the player-slot mask of everyone standing inside a y band.
///
/// In plain English: for each of the four slots resolve the player, reject
/// slots that are missing, that have the `0x53` pad flag set, or whose `+0x153C`
/// phase is `1`/`3`, then test the player's `+0x1530` height against the two
/// incoming doubles and set bit `player + 1` of the returned mask.
///
/// # Original
/// - Ghidra: `FUN_80154e50` at `0x80154E50` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80154e50_FUN_80154e50.c`
///
/// # Arguments
/// * `upper` - exclusive upper height bound (first double, `f1`).
/// * `lower` - exclusive lower height bound (second double, `f2`).
///
/// # Returns
/// A four-bit player mask shifted left by one.
///
/// # Side effects
/// Four slot lookups and four `0x80056CF0` queries; no writes.
///
/// # Safety
/// Every resolved player must be readable at `+0x1530`/`+0x153C` and expose a
/// `+0x60` interface with a `+0x6C` slot.
///
/// # Confidence
/// high - the loop bound, the four guards, both comparisons and the bit set are
/// literal, and `0x8005F900`/`0x80056CF0` are translated in `game::player`.
pub unsafe fn fn_80154e50(upper: f64, lower: f64) -> u32 {
    let mut mask = 0u32;
    let mut slot = 0usize;
    while slot < 4 {
        let player = crate::game::player::fn_8005f900(slot) as usize;
        if player != 0
            && crate::game::player::fn_80056cf0(player, 0x53) == 0
            && read_byte(player, 0x153C) != 1
            && read_byte(player, 0x153C) != 3
            && (read_float(player, 0x1530) as f64) < upper
            && lower < read_float(player, 0x1530) as f64
        {
            mask |= 1u32 << (w109_player_index(player) as u32 + 1);
        }
        slot += 1;
    }
    mask
}

/// Scans the four players inside an x band and reports the y extent.
///
/// In plain English: start from the `r2 - 0x557C` high and `r2 - 0x54D8` low
/// sentinels, count the players whose `+0x152C` x lies inside the region's
/// `+0x00`/`+0x04` pair, and track their highest and lowest `+0x1530` heights.
/// The region's `+0x40` flag word then selects who must be counted: bit
/// `0x10000000` accepts any hit, bit `0x20000000` requires every connected slot
/// and a positive `0x8005FFB0` query, and bit `0x40000000` requires every
/// connected slot and a negative `0x8005FD20` signed-difference test. The two
/// tracked heights are written out and the boolean decision is returned in
/// `r3`.
///
/// # Original
/// - Ghidra: `FUN_80156930` at `0x80156930` (372 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156930_FUN_80156930.c`
/// - Removed: the `bl 0x802DCDB4` (`__restgprlr_24`) epilogue; it only
///   restores the saved registers, so the body now returns `result` directly.
///
/// # Arguments
/// * `owner` - first context word (`r3`), forwarded to `0x80156820`, whose first
///   parameter is never read.
/// * `region` - second context word (`r4`): `+0x00`/`+0x04` x bounds and the
///   `+0x40` flag word.
/// * `high_out` - receives the highest `+0x1530` among the hits.
/// * `low_out` - receives the lowest `+0x1530` among the hits.
///
/// # Returns
/// `1` when the selected flag condition holds, `0` otherwise.
///
/// # Side effects
/// Four player lookups, at most one condition query and two output stores.
///
/// # Safety
/// `region` must be readable through `+0x43` and both outputs writable.
///
/// # Confidence
/// medium - the loop, both x comparisons, the two sentinels, the three flag
/// branches and the two stores are literal. The export reads the flag word as a
/// float (`pfVar6[0x10]`) but only tests its bits. The `0x802DCD68` save helper
/// prints the incoming `r3:r4`, so `owner`/`region` are taken as parameters.
pub unsafe fn fn_80156930(
    owner: u32,
    region: usize,
    high_out: *mut f32,
    low_out: *mut f32,
) -> u32 {
    // observed: `0x802DCD68` is the __savegprlr_24 prologue; its printed pair is
    // the incoming r3:r4.
    let connected = player_slot_count() as i32;
    let mut high = read_const_f32(W109_W_557C) as f64;
    let mut low = read_const_f32(W109_F32_54D8) as f64;
    let mut inside = 0i32;

    let mut slot = 0i32;
    while slot < 4 {
        let player = fn_80156820(owner, slot) as usize;
        if player != 0 {
            let x = read_float(player, 0x152C);
            if read_float(region, 0x00) <= x && x <= read_float(region, 0x04) {
                inside += 1;
                let y = read_float(player, 0x1530) as f64;
                if high < y {
                    high = y;
                }
                let y = read_float(player, 0x1530) as f64;
                if y < low {
                    low = y;
                }
            }
        }
        slot += 1;
    }

    let flags = read_word(region, 0x40);
    let mut result = 0u32;
    if flags & 0x1000_0000 == 0 {
        if flags & 0x2000_0000 == 0 {
            if flags & 0x4000_0000 != 0 && inside == connected {
                // observed: (int)(((int)(value ^ 1) >> 1) - ((value ^ 1) & value)) < 0
                let value = crate::game::player::fn_8005fd20() as u32;
                let flipped = value ^ 1;
                if ((flipped as i32) >> 1) - ((flipped & value) as i32) < 0 {
                    result = 1;
                }
            }
        } else if inside == connected {
            let value = crate::game::player::fn_8005ffb0() as u32;
            if positive_test(value) {
                result = 1;
            }
        }
    } else if inside != 0 {
        result = 1;
    }

    *high_out = high as f32;
    *low_out = low as f32;
    result
}

/// Scans the four players inside a y band and reports the x extent.
///
/// In plain English: the y-band sibling of [`fn_80156930`]. Count the players
/// whose `+0x1530` height lies between the region's `+0x1C` and `+0x18` words
/// and track their lowest and highest `+0x152C` x values from the same two
/// sentinels. The region's `+0x40` flags use the same three bits, but both the
/// `0x20000000` and `0x40000000` branches query `0x8005FD20`.
///
/// # Original
/// - Ghidra: `FUN_80156ab0` at `0x80156AB0` (372 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80156ab0_FUN_80156ab0.c`
/// - Removed: the `bl 0x802DCDB4` (`__restgprlr_24`) epilogue; it only
///   restores the saved registers, so the body now returns `result` directly.
///
/// # Arguments
/// * `owner` - first context word (`r3`), forwarded to `0x80156820`.
/// * `region` - second context word (`r4`): `+0x1C` low and `+0x18` high y
///   bounds, plus the `+0x40` flag word.
/// * `low_out` - receives the lowest `+0x152C` among the hits.
/// * `high_out` - receives the highest `+0x152C` among the hits.
///
/// # Returns
/// `1` when the selected flag condition holds, `0` otherwise.
///
/// # Side effects
/// Four player lookups, at most one condition query and two output stores.
///
/// # Safety
/// `region` must be readable through `+0x43` and both outputs writable.
///
/// # Confidence
/// medium - the loop, both y comparisons, the sentinels and the three flag
/// branches are literal; the export reads the flag word as a halfword at
/// `+0x40`, whose `0x1000`/`0x2000`/`0x4000` bits are the top three bits of the
/// word used here.
pub unsafe fn fn_80156ab0(
    owner: u32,
    region: usize,
    low_out: *mut f32,
    high_out: *mut f32,
) -> u32 {
    let connected = player_slot_count() as i32;
    let mut low = read_const_f32(W109_F32_54D8) as f64;
    let mut high = read_const_f32(W109_W_557C) as f64;
    let mut inside = 0i32;

    let mut slot = 0i32;
    while slot < 4 {
        let player = fn_80156820(owner, slot) as usize;
        if player != 0 {
            let y = read_float(player, 0x1530);
            if y <= read_float(region, 0x18) && read_float(region, 0x1C) <= y {
                inside += 1;
                let x = read_float(player, 0x152C) as f64;
                if x < low {
                    low = x;
                }
                let x = read_float(player, 0x152C) as f64;
                if high < x {
                    high = x;
                }
            }
        }
        slot += 1;
    }

    let flags = read_word(region, 0x40);
    let mut result = 0u32;
    if flags & 0x1000_0000 == 0 {
        if flags & 0x2000_0000 == 0 {
            if flags & 0x4000_0000 != 0 && inside == connected {
                let value = crate::game::player::fn_8005fd20() as u32;
                let flipped = value ^ 1;
                if ((flipped as i32) >> 1) - ((flipped & value) as i32) < 0 {
                    result = 1;
                }
            }
        } else if inside == connected {
            let value = crate::game::player::fn_8005fd20() as u32;
            if positive_test(value) {
                result = 1;
            }
        }
    } else if inside != 0 {
        result = 1;
    }

    *low_out = low as f32;
    *high_out = high as f32;
    result
}

/// Forwards a buffer to the CRC-32 routine at `0x801B8620`.
///
/// In plain English: `0x8015F130` is a four-byte branch, so Ghidra exports the
/// target's body under the thunk's name. The two registers are forwarded.
///
/// # Original
/// - Ghidra: `thunk_FUN_801b8620` at `0x8015F130` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f130_thunk_FUN_801b8620.c`
///
/// # Arguments
/// * `data` - first byte of the buffer.
/// * `len` - byte count.
///
/// # Returns
/// The complemented CRC-32 of the buffer.
///
/// # Side effects
/// Exactly those of [`crate::runtime::os::fn_801b8620`]; none of its own.
///
/// # Safety
/// `data` must be readable for `len` bytes.
///
/// # Confidence
/// high - `functions.tsv` gives the body four bytes and the folded target is
/// already translated in `runtime::os`.
pub unsafe fn fn_8015f130(data: usize, len: u32) -> u32 {
    crate::runtime::os::fn_801b8620(data, len)
}

// ---------------------------------------------------------------------------
// Wave 109 helpers and constants
// ---------------------------------------------------------------------------

/// Calls `func(f64, u32)` with no result.
#[inline]
unsafe fn w109_call_f64_u32(addr: usize, a: f64, b: u32) {
    let f: extern "C" fn(f64, u32) = core::mem::transmute(addr);
    f(a, b);
}

/// Reads the `slot` entry of the interface table at `obj + 0x60`.
#[inline]
unsafe fn w109_iface_slot(obj: usize, slot: usize) -> usize {
    let iface = read_word(obj, 0x60) as usize;
    read_word(iface, slot) as usize
}

/// Reads the one-character player number the `+0x6C` slot reports.
#[inline]
unsafe fn w109_player_index(obj: usize) -> u8 {
    let entry = w109_iface_slot(obj, 0x6C);
    let text = platform::call1(entry, obj as u32) as usize;
    read_byte(text, 0)
}

/// Calls interface slot `slot` of `obj` with `obj` plus two integer arguments.
#[inline]
unsafe fn w109_iface_call3(obj: usize, slot: usize, a: u32, b: u32) -> u32 {
    let entry = w109_iface_slot(obj, slot);
    platform::call3(entry, obj as u32, a, b)
}

/// Calls interface slot `slot` of `obj` with a double and three integers.
#[inline]
unsafe fn w109_iface_call_f64_u32x3(
    obj: usize,
    slot: usize,
    a: f64,
    b: u32,
    c: u32,
    d: u32,
) -> u32 {
    let entry = w109_iface_slot(obj, slot);
    let f: extern "C" fn(f64, u32, u32, u32) -> u32 = core::mem::transmute(entry);
    f(a, b, c, d)
}

/// Calls interface slot `slot` of `obj` with two doubles and four integers.
#[inline]
unsafe fn w109_iface_call_f64x2_u32x4(
    obj: usize,
    slot: usize,
    a: f64,
    b: f64,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
) -> u32 {
    let entry = w109_iface_slot(obj, slot);
    let func: extern "C" fn(f64, f64, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(entry);
    func(a, b, c, d, e, f)
}

/// `r13 - 0x57E8` -> `0x80429E98`: base of the `0x3B0`-byte stage-entry table.
const W109_G_57E8: usize = SDA_BASE - 0x57E8;

/// `r13 - 0x5678` -> `0x80429F08`: shared per-player effect object used by
/// [`fn_80151e50`]. The same slot is documented by `game::worldmap`.
const W109_G_5678: usize = SDA_BASE - 0x5678;

/// `r13 - 0x5594`: first screen scale of [`fn_801548a0`] and the divisor of
/// [`fn_80154a70`].
const W109_F32_5594: usize = SDA_BASE - 0x5594;

/// `r13 - 0x5590`: second screen scale of [`fn_801548a0`] and the divisor of the
/// rectangle height in [`fn_80154a70`].
const W109_F32_5590: usize = SDA_BASE - 0x5590;

/// `r2 - 0x5584`: base value divided by the current zoom.
const W109_F32_5584: usize = SDA2_BASE - 0x5584;

/// `r2 - 0x5580`: screen offset applied along both axes.
const W109_F32_5580: usize = SDA2_BASE - 0x5580;

/// `r2 - 0x557C`: colour word written to the screen `+0x58`/`+0x5C` pair, and
/// the "highest" sentinel of the two band scans.
const W109_W_557C: usize = SDA2_BASE - 0x557C;

/// `r2 - 0x5578`: lower clamp of the ratio in [`fn_80154a70`].
const W109_F32_5578: usize = SDA2_BASE - 0x5578;

/// `r2 - 0x5570`: 64-bit value divided by the ratio in [`fn_80154a70`].
const W109_F64_5570: usize = SDA2_BASE - 0x5570;

/// `r2 - 0x5568`: word copied into `+0x8FEC4`/`+0x8FEC8`.
const W109_W_5568: usize = SDA2_BASE - 0x5568;

/// `r2 - 0x54D8`: "lowest" sentinel of the two band scans.
const W109_F32_54D8: usize = SDA2_BASE - 0x54D8;

/// `r2 - 0x7838`: first float handed to the `+0x3FC` callback.
const W109_F32_7838: usize = SDA2_BASE - 0x7838;

/// `0x80376E88`: layout name installed by [`fn_80152cf0`].
const W109_G_80376E88: usize = 0x8037_6E88;

/// `0x8035421C`: animation name started through the `+0x294` slot.
const W109_G_8035421C: usize = 0x8035_421C;

/// `0x800FFDB0(value, manager)`: stores the absolute distance between `value`
/// and the manager's reconstructed position.
const W109_FN_800FFDB0: usize = 0x800F_FDB0;

/// `0x80050E60()`: four-argument player helper whose operands the export drops
/// in [`fn_80152cf0`]; only the call is reproduced.
const W109_FN_80050E60: usize = 0x8005_0E60;

/// `0x80078EB0(scene) -> f32`: zoom getter. Its own export prints `void`; the
/// call sites in [`fn_801548a0`] consume its `f1` result.
const W109_FN_80078EB0: usize = 0x8007_8EB0;

// ---------------------------------------------------------------------------
// Wave 114 - the last 0x800D0000 body: the element bind/attach pass
// ---------------------------------------------------------------------------
//
// The `0x800D0000` bucket entered this pass with one open address, the last of
// the bucket's 251 functions. `FUN_800d43e0` is the twin of
// `FUN_800fec10` (`0x800FEC10`, outside this lane): both take three float
// operands plus a marker object, a source record and a mode word, and both run
// the same `0x801651F0` + `0x801680E0` + `0x80168FE0` + `0x80169F80` sequence on
// the 0x38-byte element embedded at `object + 0x18`.
//
// The `ldexpf(GQR0)` block at the head of the export, like every other
// `ldexpf`/`CONCAT` block in this file, is the decompiler's failed model of a
// paired-single save and is not a call.

/// Resolves the current view's model and binds/attaches the marker element.
///
/// In plain English: the marker object keeps a view index at `+0x770` and a
/// name-pointer table at `+0x778`. The routine reads the current view's model
/// name from that table and asks the BRRES archive stored at `+0x20C` for the
/// model; the returned resource pointer is not stored. It then takes the single
/// descriptor word of the source record, files the 0x38-byte element at
/// `object + 0x18` under it through `0x801651F0` with the receiver returned by
/// vtable slot `+0x28` and the caller's mode word, hands the first float to the
/// element's animation object through `0x801680E0`, stores the second float at
/// element `+0x30` through `0x80168FE0`, and finally attaches the element with
/// `0x80169F80`, which receives the third float, a fresh `+0x28` receiver and
/// the descriptor at `object + 0x26`. The float passed to that second `+0x28`
/// invocation is the signed halfword at `+0x20` of the source record converted
/// to double.
///
/// # Original
/// - Ghidra: `FUN_800d43e0` at `0x800D43E0` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d43e0_FUN_800d43e0.c`
///
/// # Arguments
/// - `value_a`: float handed to the element's `+0x04` object slot `+0x28`
///   through `0x801680E0`. `// inferred:` an animation frame/rate; the sibling
///   call sites of the same callee pass SDA2 floats for it.
/// - `value_b`: float stored at element `+0x30` by `0x80168FE0`.
///   `// inferred:` the element's own frame value.
/// - `value_c`: float forwarded to `0x80169F80` with the receiver.
/// - `object`: the marker; `+0x18` is the 0x38-byte element, `+0x26` a
///   descriptor inside it, `+0x20C` the BRRES archive field, `+0x770` the view
///   index and `+0x778` the name-pointer table.
/// - `source`: the source record; its first word is the descriptor word and the
///   same word is later used as the base of the `+0x20` halfword.
/// - `mode`: fourth argument of `0x801651F0`.
///
/// # Returns
/// Nothing. The model pointer returned by the BRRES lookup is discarded, exactly
/// as in the export.
///
/// # Side effects
/// One BRRES dictionary lookup, one keyed element filing that runs the element's
/// `+0x0C` slot and `0x80165280`, one animation dispatch through `0x801680E0`,
/// one `+0x30` float store through `0x80168FE0`, and one attach through
/// `0x80169F80`.
///
/// # Safety
/// `object` must be a live marker: `+0x20C` must address the word holding a
/// readable BRRES image, the name table entry must point at a NUL-terminated
/// name, and the element's vtable must implement `+0x0C` and `+0x28`.
/// `source` must be readable for one word and, through that word, for the
/// halfword at `+0x20`.
///
/// # Confidence
/// medium - the five named calls, the two vtable invocations, their order and
/// every offset are literal. Two
/// operand shapes come from the surrounding exports rather than from this one:
/// the three `undefined8` parameters are the `f1..f3` float arguments (the
/// callees `0x801680E0`/`0x80168FE0` are called as `(float, element)` from
/// `0x800F0630` and `0x800EF230`), and the `f1` value Ghidra attaches to the
/// second `+0x28` invocation is printed as the SDA `0x43300000` conversion
/// constant, so it is reconstructed as the `(double)` of the signed halfword
/// built immediately before it. The field roles are `// inferred:` from
/// [`fn_800d36e0`], which reads the same `+0x770`/`+0x778` pair.
pub unsafe fn fn_800d43e0(
    value_a: f64,
    value_b: f64,
    value_c: f64,
    object: usize,
    source: usize,
    mode: u32,
) {
    // observed: the name word sits at `param_4[0x1de] + param_4[0x1dc] * 4 + 8`,
    // i.e. two words past the start of the current view entry.
    let view = read_word(object, 0x770) as usize;
    let name_table = read_word(object, 0x778) as usize;
    let model_name = read_global_word(name_table + view * 4 + 0x08);
    // observed: the lookup result is never stored by the export.
    let _ = crate::resource::bres::fn_80239e30(object + 0x20C, model_name);

    // observed: `local_58[0] = *param_5`; the word is passed by address.
    let record_word = read_word(source, 0);
    let element = object + 0x18;

    // observed: `piVar3 = param_4 + 0x18; uVar2 = (**(code **)(*param_4 + 0x28))(param_4);`
    let receiver = vcall0(object, 0x28);
    let _ = platform::call4(
        W114_FN_801651F0,
        element as u32,
        receiver,
        &record_word as *const u32 as usize as u32,
        mode,
    );

    // observed: `FUN_801680e0(param_1,piVar3); FUN_80168fe0(param_2,piVar3);`
    // `w109_call_f64_u32` is the same `(f64, u32)` call shape.
    w109_call_f64_u32(W114_FN_801680E0, value_a, element as u32);
    w109_call_f64_u32(W114_FN_80168FE0, value_b, element as u32);

    // observed: `local_50 = 0x43300000; uStack_4c = *(ushort *)(*param_5 + 0x20)
    // ^ 0x80000000;` - the PowerPC signed-halfword to double conversion.
    let halfword_value = read_half(record_word as usize, 0x20) as i16 as f64;
    let receiver = w114_vcall_f64(object, 0x28, halfword_value);
    w114_call_f64_u32_u32(
        W114_FN_80169F80,
        value_c,
        receiver,
        (object + 0x26) as u32,
    );
}

// ---------------------------------------------------------------------------
// Wave 114 helpers and constants
// ---------------------------------------------------------------------------

/// Calls vtable slot `slot` of `this_arg` with one double (`f1`) and the
/// receiver in `r3`, returning the `r3` result.
///
/// [`vcall0`] cannot express the float operand that the second `+0x28`
/// invocation of [`fn_800d43e0`] carries.
#[inline]
unsafe fn w114_vcall_f64(this_arg: usize, slot: usize, value: f64) -> u32 {
    let vtable = read_word(this_arg, 0) as usize;
    let entry = read_word(vtable, slot) as usize;
    let f: extern "C" fn(f64, u32) -> u32 = core::mem::transmute(entry);
    f(value, this_arg as u32)
}

/// Calls `func(f64, u32, u32)` with no result.
#[inline]
unsafe fn w114_call_f64_u32_u32(addr: usize, a: f64, b: u32, c: u32) {
    let f: extern "C" fn(f64, u32, u32) = core::mem::transmute(addr);
    f(a, b, c);
}

/// `0x801651F0(element, receiver, descriptor, mode)`: files the element under
/// the descriptor word and rebuilds its keyed entry. Translated in
/// `resource::registry` as [`crate::resource::registry::fn_801651f0`].
const W114_FN_801651F0: usize = 0x8016_51F0;

/// `0x801680E0(float, element)`: runs the element's `+0x04` object slot `+0x28`
/// with the float. Its own export recovers one parameter; the float is visible
/// at the `0x800F0630` call sites.
const W114_FN_801680E0: usize = 0x8016_80E0;

/// `0x80168FE0(double, element)`: runs `0x801680A0` on the element and stores
/// the double as an `f32` at element `+0x30`.
const W114_FN_80168FE0: usize = 0x8016_8FE0;

/// `0x80169F80(float, object, descriptor)`: attaches the descriptor when its
/// `+0x0C` slot reports "not attached".
const W114_FN_80169F80: usize = 0x8016_9F80;

// ---------------------------------------------------------------------------
// Wave 115 - the last 0x80150000 body: the variadic report entry
// ---------------------------------------------------------------------------
//
// The `0x80150000` bucket entered this pass with one open address, the last of
// the bucket's 209 functions: `FUN_8015f730`, the `printf`-shaped front end every
// `OSReport` call site reaches. It is the caller of the already translated
// [`fn_8015f7b0`] (`0x8015F7B0`, a four-byte branch to the callback trampoline
// `0x8015F6E0` in `game::task`), so the report finally has both halves of its
// chain in this crate.
//
// The export prints the eight `f1..f8` saves as `undefined8` parameters guarded
// by an `in_cr1` test and prints the eight `r3..r10` saves as `param_9..16`,
// because Ghidra spends its whole parameter budget on the float pairs. The
// instruction words (read from `work/dol/main.elf`) order them explicitly:
// `stw r3,0x8(r1)` .. `stw r10,0x24(r1)` for the integer save area and
// `stfd f1,0x28(r1)` .. `stfd f8,0x60(r1)` under `bne cr1,+0x24` for the float
// save area, then the twelve-byte `va_list` at `r1 + 0x68`.

/// Saves the argument registers into a `va_list` and forwards the report.
///
/// In plain English: this is the game's variadic print entry. It copies the
/// eight integer argument registers (`r3..r10`) into an eight-word save area,
/// copies the eight float argument registers (`f1..f8`) into the following
/// sixty-four bytes when the caller's `CR1[EQ]` flag says floating-point
/// arguments were passed, and then builds the three-word `va_list`
/// `{tag, overflow area, register save area}`. The report callback installed at
/// `0x80429408` receives the untouched format pointer from `r3` and the address
/// of that list.
///
/// # Original
/// - Ghidra: `FUN_8015f730` at `0x8015F730` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8015f730_FUN_8015f730.c`
///
/// # Arguments
/// - `format`: `r3`; the NUL-terminated format string pointer. It is the one
///   named parameter, so the `va_list` tag consumes `r3` (GPR index `1`).
/// - `gpr`: `r4..r10`; the seven remaining integer/pointer arguments, modelled
///   as an array because the original reads them straight out of registers.
/// - `fpr`: `f1..f8`; the floating-point arguments.
/// - `stack_args`: address of the caller's first stack argument word (the
///   original stores `r1 + 0x88`, i.e. the caller's `sp + 8`). This is the
///   `va_list` overflow area used once the seven saved integer words and the
///   eight saved floats are exhausted.
///
/// # Returns
/// Nothing. The callback's result, if any, is discarded, exactly as in the
/// export's `void` return.
///
/// # Side effects
/// Fills the `+0x08..+0x74` slice of its own 0x80-byte frame and calls the global
/// report callback through [`fn_8015f7b0`]; the observable effect is whatever
/// that callback does with the format and the list (normally a debug-console
/// print).
///
/// # Safety
/// `format` must be a valid NUL-terminated string when the installed callback
/// formats it, `stack_args` must address the caller's stack argument words for
/// as many conversions as the format consumes, and a callback must have been
/// installed at `0x80429408` (the original does not null-check it).
///
/// # Confidence
/// medium - the frame offsets, the `$cr1` guard, the `0x01000000` tag, the two
/// area pointers and the branch to `0x8015F7B0` are literal in the instruction
/// words behind the export. Two readings are recovered rather than printed:
/// `// inferred:` byte 0 of the tag is the GPR index (the format argument
/// consumed `r3`) and byte 1 the FPR index, matching the `0x03000000` tag
/// documented for [`crate::engine::main_loop::fn_802e16e8`]; and the eight
/// `undefined8` parameters are the `f1..f8` saves, which the `stfd` sequence and
/// the `CR1[EQ]` guard make certain but the C export does not spell out. Storing
/// `fpr` unconditionally is equivalent to the guard, because the guard only
/// skips stores of floats that were never passed.
pub unsafe fn fn_8015f730(format: u32, gpr: [u32; 7], fpr: [f64; 8], stack_args: u32) {
    // observed: `stwu r1,-0x80(r1)`; the frame is 0x80 bytes and 8-byte aligned
    // so the saved doubles can be reloaded with `lfd`.
    let mut frame = [0_u64; 16];
    let base = frame.as_mut_ptr() as usize;

    // observed: `stw r3,0x8(r1)` .. `stw r10,0x24(r1)` - the integer save area.
    platform::write_u32(base + 0x08, format);
    let mut index = 0;
    while index < 7 {
        platform::write_u32(base + 0x0C + index * 4, gpr[index]);
        index += 1;
    }

    // observed: `bne cr1,+0x24` then `stfd f1,0x28(r1)` .. `stfd f8,0x60(r1)`.
    let mut index = 0;
    while index < 8 {
        w115_write_f64(base + 0x28 + index * 8, fpr[index]);
        index += 1;
    }

    // observed: `stw r12,0x68(r1)` (tag), `stw r11,0x6c(r1)` (overflow area,
    // `addi r11,r1,0x88`) and `stw r0,0x70(r1)` (register save area,
    // `addi r0,r1,0x8`).
    platform::write_u32(base + 0x68, W115_VA_TAG_GPR1);
    platform::write_u32(base + 0x6C, stack_args);
    platform::write_u32(base + 0x70, (base + 0x08) as u32);

    // observed: `bl 0x8015F7B0` from `0x8015F79C` with `r3` untouched and
    // `r4 = r1 + 0x68`; `0x8015F7B0` is the thunk to the callback trampoline.
    fn_8015f7b0(format, (base + 0x68) as u32);
}

// ---------------------------------------------------------------------------
// Wave 115 helpers and constants
// ---------------------------------------------------------------------------

/// `0x01000000`: the `va_list` register word stored at frame `+0x68`.
///
/// observed: `lis r12,0x0100` / `stw r12,0x68(r1)`.
///
/// `// inferred:` big-endian byte 0 is the GPR index (the format argument
/// consumed `r3`, so the first variadic integer starts at `r4`) and byte 1 the
/// FPR index (no named float argument, so the first variadic float starts at
/// `f1`). This is the same byte-pair convention as the `0x03000000` tag in
/// [`crate::engine::main_loop::fn_802e16e8`].
const W115_VA_TAG_GPR1: u32 = 0x0100_0000;

/// Writes one `f64` as two big-endian words.
///
/// The saved `f1..f8` occupy eight-byte slots in the save area the formatter
/// reloads with `lfd`; [`platform`] only offers the 32-bit store.
#[inline]
unsafe fn w115_write_f64(addr: usize, value: f64) {
    let bits = value.to_bits();
    platform::write_u32(addr, (bits >> 32) as u32);
    platform::write_u32(addr + 4, bits as u32);
}

// ===========================================================================

// ===========================================================================
// Wave 267, lane D (`_D267`): the `0x800Dxxxx` slots of the 46-slot vtable
// `0x803175E0` of the Mario/Luigi character-model record
// ===========================================================================
//
// Seven prologue-shaped uncovered targets of the run at `0x803175E8` live in
// this band: slots 2, 6, 12, 14, 15, 19 and 20. The install site and the class
// identity are documented in `game::stage` (`_D267`): the constructor
// `0x800CA060` stores `0x803175E0` at `this + 0` and points `+0x778` at the
// 0x38-byte `"Mario"`-family row selected by the character byte at `+0x151`.
//
// The `+0x770` word selects one of four 0x80-byte animation groups based at
// `+0x4A8`; `+0x870` indexes the 0xC-byte record array at `+0x828`; the three
// floats at `+0x788`/`+0x78C`/`+0x790` are the pitch/scale triple written by
// slot 2. The names at `0x80317558..0x80317570` (`PB_switch`, `PC_switch`,
// `PH_switch`) are used by sibling slots in `game::stage`.

// ---------------------------------------------------------------------------
// Constants introduced by this lane (`_D267` suffix)
// ---------------------------------------------------------------------------

/// `0x800D32F0(object)`: reloads the three variant model handles at
/// `+0x20C`/`+0x210`/`+0x214`.
const D267_FN_800D32F0: usize = 0x800D_32F0;

/// `0x8023B860(header, index)`: indexes one child record of a model header.
const D267_FN_8023B860: usize = 0x8023_B860;

/// `0x8023B8C0(header)`: child count of the same model header.
const D267_FN_8023B8C0: usize = 0x8023_B8C0;

/// `0x8023D300(target, index, out4)`: extracts one four-byte colour from a
/// child record.
const D267_FN_8023D300: usize = 0x8023_D300;

/// `r2 - 0x6630` -> `0x8042CA50`: `255.0f`, the brightness denominator.
const D267_F32_255: usize = SDA2_BASE - 0x6630;

/// `0x80427CB8`: the `"P_jump"` name copied by [`fn_800d4060`].
const D267_STR_P_JUMP: u32 = 0x8042_7CB8;

// ---------------------------------------------------------------------------
// Bodies
// ---------------------------------------------------------------------------

/// Reloads the model handles and copies the pitch word to the three view
/// scales (slot 2).
///
/// In plain English: `0x800D32F0(this)` re-resolves the variant models, virtual
/// slot `+0xBC` runs with its result dropped, and the `f1` returned by virtual
/// slot `+0xB8` is stored at `+0x790`, `+0x78C` and `+0x788`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x800D3210`.
/// - Extent: `0x800D3210..0x800D3270` (96 bytes); an 8-byte `lfs f1`/`blr` leaf
///   sits at `0x800D3270` and the next prologue starts at `0x800D3280`.
/// - Table: slot 2 of the `0x803175E0` vtable (word `0x803175F0`, offset
///   `+0x10`); installed by [`crate::game::stage::fn_800ca060`].
/// - Evidence: the `0x800D32F0(this)` call, two `bcctr 20,0l` calls through
///   `+0xBC` and `+0xB8`, and the three `stfs` stores.
///
/// # Arguments
/// - `this`: the character-model record.
///
/// # Side effects
/// One model reload, two virtual calls and three float stores.
///
/// # Safety
/// `this` must be live and its vtable must implement `+0xB8`/`+0xBC`; the
/// record must be writable at `+0x788..+0x790`.
///
/// # Confidence
/// high - the callee, both slot offsets and the three stores are literal.
pub unsafe fn fn_800d3210(this: usize) {
    let _ = platform::call1(D267_FN_800D32F0, this as u32);

    // observed: the `+0xBC` call's `f1` is discarded.
    let first = w99_slot(this, 0xbc);
    let _ = w99_call1_f32_u32(first, this as u32);

    let second = w99_slot(this, 0xb8);
    let value = w99_call1_f32_u32(second, this as u32);
    write_float(this, 0x790, value);
    write_float(this, 0x78c, value);
    write_float(this, 0x788, value);
}

/// Stores `2` at `+0x208` and republishes the `+0xD8` transform (slot 6).
///
/// In plain English: write `2` to `+0x208`, ask virtual slot `+0x28` for the
/// animation group, copy the transform at `this + 0xD8` into it through
/// `0x8016A170`, then ask the same slot again and mark the group dirty with
/// `0x8016A1A0(group, 0)`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D3830`.
/// - Extent: `0x800D3830..0x800D3894` (100 bytes); three zero words follow and
///   the next body starts at `0x800D38A0`.
/// - Table: slot 6 of the `0x803175E0` vtable (word `0x80317600`, offset
///   `+0x20`).
/// - Evidence: `li r0,2` / `stw r0,+0x208`, the two `+0x28` virtual calls,
///   `0x8016A170(group, this+0xD8)` and `0x8016A1A0(group, 0)`.
///
/// # Arguments
/// - `this`: the record; transform at `+0xD8`, state word at `+0x208`.
///
/// # Side effects
/// One word store, two virtual calls, one transform copy and one dirty mark.
///
/// # Safety
/// `this` must be live and virtual slot `+0x28` must return a live group.
///
/// # Confidence
/// high - the constant, both calls and both offsets are literal.
pub unsafe fn fn_800d3830(this: usize) {
    write_word(this, 0x208, 2);

    let group = vcall0(this, 0x28) as usize;
    crate::resource::registry::fn_8016a170(group, (this + 0xD8) as u32);

    let group = vcall0(this, 0x28) as usize;
    crate::resource::registry::fn_8016a1a0(group, 0);
}

/// Tints the mode's animation group and every child by a brightness level
/// (slot 12).
///
/// In plain English: turn `level` into `normalized = 1 - level/255`, build the
/// four-byte grey word `(int)(255 * normalized)` repeated on all four channels,
/// submit it to the `+0x4A8 + mode*0x80` record and to the `+0x28` group
/// through `0x80168710(record, 1, &tint, 0)`, then walk that record's children:
/// extract each child's four colour bytes with `0x8023D300`, scale the fourth
/// byte by `normalized` (`fctiwz`), and submit the tint through `0x801685E0`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D4790`.
/// - Extent: `0x800D4790..0x800D4974` (484 bytes); the closing `bclr 20,0` is
///   at `0x800D4970` and four zero words separate it from `0x800D4980`. The
///   `b 0x800D4934` at `0x800D489C` is the loop's join block, not a tail call,
///   so a walker that stops at the first forward `b` reports only 272 bytes.
/// - Table: slot 12 of the `0x803175E0` vtable (word `0x80317618`, offset
///   `+0x38`).
/// - Evidence: the `0x4330_0000_8000_0000` unsigned-int bias (`xoris r0,r4,0x8000`
///   + `lfd` at `r2-0x6638`), the `255.0` (`r2-0x6630`) and `1.0`
///   (`r2-0x664C`) constants, the two `fctiwz` (`0xFC00001E`) conversions, the
///   two `0x80168710` calls and the `0x8023B8C0`/`0x8023B860`/`0x8023D300`/
///   `0x801685E0` loop.
///
/// # Arguments
/// - `this`: the record; animation group at `+0x4A8 + mode*0x80`, mode at
///   `+0x770`.
/// - `level`: brightness `0..255` (the body only divides it by `255.0`).
///
/// # Side effects
/// Two `0x80168710` submissions plus one `0x801685E0` per child.
///
/// # Safety
/// `this` must be live, the model header returned by `0x80168400` readable and
/// every child record's `+0x3C` bias valid for `0x8023D300`.
///
/// # Confidence
/// medium - the constants, both conversions, the loop and every call are
/// literal; the meaning of `level` as a fade/brightness byte is `// inferred:`
/// from the two `255.0` uses.
pub unsafe fn fn_800d4790(this: usize, level: u32) {
    let normalized = 1.0f32 - level as f32 / read_const_f32(D267_F32_255);

    // observed: `fmuls f0,f2,f30` then `fctiwz f0,f0`; the low byte of the
    // integer is copied to all four bytes of the tint word.
    let shade = (read_const_f32(D267_F32_255) * normalized) as i32 as u8;
    let mut tint = [shade; 4];
    let mut tint_second = [shade; 4];

    let node = this + read_word(this, 0x770) as usize * 0x80 + 0x4A8;
    crate::resource::registry::fn_80168710(node, 1, tint.as_mut_ptr() as usize, 0);

    let group = vcall0(this, 0x28) as usize;
    crate::resource::registry::fn_80168710(
        group,
        1,
        tint_second.as_mut_ptr() as usize,
        0,
    );

    let mut header = crate::resource::registry::fn_80168400(node);
    let mut index = 0u32;
    while index
        < platform::call1(
            D267_FN_8023B8C0,
            &mut header as *mut u32 as usize as u32,
        )
    {
        let element = platform::call2(
            D267_FN_8023B860,
            &mut header as *mut u32 as usize as u32,
            index,
        ) as usize;
        let bias = read_word(element, 0x3C) as usize;
        let base = if bias == 0 { 0usize } else { element + bias };
        let mut target = (base + 0x20) as u32;
        let mut color = [0u8; 4];
        let _ = platform::call3(
            D267_FN_8023D300,
            &mut target as *mut u32 as usize as u32,
            2,
            color.as_mut_ptr() as usize as u32,
        );

        // observed: `fctiwz` of `(f32)color[3] * normalized` written back to
        // byte 3 before the submission.
        color[3] = (color[3] as f32 * normalized) as i32 as u8;
        crate::resource::registry::fn_801685e0(node, index, 2, color.as_ptr() as usize, 0);
        index += 1;
    }
}

/// Raises the `+0x17C` "attached" bit and seeds both animation records
/// (slot 14).
///
/// In plain English: when bit `0x100` of `+0x17C` is clear, set it and call
/// `0x800D4C10(this, record, this + 0x744)` on the mode's `+0x4A8 + mode*0x80`
/// record, then repeat on the `+0x28` group with `this + 0x694`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D49D0`.
/// - Extent: `0x800D49D0..0x800D4A4C` (124 bytes); two zero words follow and
///   the next body starts at `0x800D4A50`.
/// - Table: slot 14 of the `0x803175E0` vtable (word `0x80317620`, offset
///   `+0x40`).
/// - Evidence: `rlwinm r0,r0,0,23,23` (bit `0x100`), the `ori r4,r4,0x100`
///   store, the two `0x800D4C10` calls and the `+0x744`/`+0x694` sources.
///
/// # Arguments
/// - `this`: the record; animation group at `+0x4A8 + mode*0x80`, bit word at
///   `+0x17C`.
///
/// # Side effects
/// One bit set and two `0x800D4C10` pairs.
///
/// # Safety
/// `this` must be live and both animation records writable.
///
/// # Confidence
/// high - the bit test, the store and both calls with their sources are
/// literal.
pub unsafe fn fn_800d49d0(this: usize) {
    let flags = read_word(this, 0x17C);
    if flags & 0x100 != 0 {
        write_word(this, 0x17C, flags | 0x100);
        let node = this + read_word(this, 0x770) as usize * 0x80 + 0x4A8;
        fn_800d4c10(this as u32, node, (this + 0x744) as u32);

        let group = vcall0(this, 0x28);
        fn_800d4c10(this as u32, group as usize, (this + 0x694) as u32);
    }
}

/// Clears the `+0x17C` "attached" bit and releases both animation records
/// (slot 15).
///
/// In plain English: when bit `0x100` of `+0x17C` is set, clear it, run
/// `0x800D4C70(this, record)` on the mode's `+0x4A8 + mode*0x80` record and
/// then `0x800D4C70(this, group)` with the `+0x28` group.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D4A50`.
/// - Extent: `0x800D4A50..0x800D4AC4` (116 bytes); three zero words follow and
///   the next body starts at `0x800D4AD0`.
/// - Table: slot 15 of the `0x803175E0` vtable (word `0x80317624`, offset
///   `+0x44`).
/// - Evidence: the `rlwinm r0,r0,0,23,23` guard, `rlwinm r4,r4,0,24,22`
///   (`& ~0x100`) and the two `0x800D4C70` calls.
///
/// # Arguments
/// - `this`: the record; animation group at `+0x4A8 + mode*0x80`, bit word at
///   `+0x17C`.
///
/// # Side effects
/// One bit clear and two `0x800D4C70` releases.
///
/// # Safety
/// `this` must be live and both animation records release-safe.
///
/// # Confidence
/// high - the guard, the mask and both calls are literal.
pub unsafe fn fn_800d4a50(this: usize) {
    let flags = read_word(this, 0x17C);
    if flags & 0x100 != 0 {
        write_word(this, 0x17C, flags & !0x100);
        let node = this + read_word(this, 0x770) as usize * 0x80 + 0x4A8;
        fn_800d4c70(this as u32, node);

        let group = vcall0(this, 0x28);
        fn_800d4c70(this as u32, group as usize);
    }
}

/// Queries the mode-2 animation group for one value (slot 19).
///
/// In plain English: only when `+0x770 == 2`, forward to
/// `0x80168180(record, 1, arg)` on the `+0x4A8 + mode*0x80` record and return
/// its answer; every other mode returns `0`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D4980`.
/// - Extent: `0x800D4980..0x800D49CC` (76 bytes); one zero word follows and the
///   next body starts at `0x800D49D0`.
/// - Table: slot 19 of the `0x803175E0` vtable (word `0x80317634`, offset
///   `+0x54`).
/// - Evidence: the `cmpi r0,2` gate, the mode-scaled `+0x4A8` record, the
///   `li r4,1` selector and the `li r3,1`/`li r3,0` return pair.
///
/// # Arguments
/// - `this`: the record; mode at `+0x770`.
/// - `arg`: third operand of `0x80168180`.
///
/// # Returns
/// `0x80168180`'s answer in mode `2`, else `0`.
///
/// # Safety
/// The mode-2 record at `+0x4A8 + mode*0x80` must be a live animation record.
///
/// # Confidence
/// high - the gate, the selector and the return pair are literal.
pub unsafe fn fn_800d4980(this: usize, arg: u32) -> u32 {
    if read_word(this, 0x770) != 2 {
        return 0;
    }
    let node = this + read_word(this, 0x770) as usize * 0x80 + 0x4A8;
    crate::resource::registry::fn_80168180(node, 1, arg)
}

/// Resolves a name-linked model id and copies `"P_jump"` (slot 20).
///
/// In plain English: run `0x800D6840(this, kind, out, variant)`; when it
/// answered non-zero, and only when the mode is `3`, `kind` is `5` and `+0x168`
/// is still zero, copy the 32-byte `"P_jump"` name at `0x80427CB8` into `out`.
/// The slot always reports `1` when `0x800D6840` answered and `0` otherwise.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D4060`.
/// - Extent: `0x800D4060..0x800D40E8` (136 bytes); two zero words follow and
///   the next body starts at `0x800D40F0`.
/// - Table: slot 20 of the `0x803175E0` vtable (word `0x80317638`, offset
///   `+0x58`).
/// - Evidence: the `0x800D6840` call, the `+0x770 == 3` / `kind == 5` /
///   `+0x168 == 0` triple gate, the `0x802E19F8(out, "P_jump", 0x20)` copy and
///   the `li r3,1`/`li r3,0` pair.
///
/// # Arguments
/// - `this`: the record; mode `+0x770`, mask `+0x168`.
/// - `kind`: second operand of `0x800D6840`; `5` enables the copy.
/// - `out`: destination for the 32-byte name (and `0x800D6840`'s output).
/// - `variant`: fourth operand of `0x800D6840`.
///
/// # Returns
/// `1` when `0x800D6840` answered non-zero, else `0`.
///
/// # Safety
/// `out` must be writable for at least 32 bytes.
///
/// # Confidence
/// high - every comparison, the name address and the copy length are literal.
pub unsafe fn fn_800d4060(this: usize, kind: u32, out: u32, variant: u32) -> u32 {
    if fn_800d6840(this, kind, out, variant) == 0 {
        return 0;
    }
    if read_word(this, 0x770) == 3 && kind == 5 && read_word(this, 0x168) == 0 {
        crate::engine::main_loop::fn_802e19f8(out as usize, D267_STR_P_JUMP as usize, 0x20);
    }
    1
}


// ===========================================================================
// Wave 267, lane C (`_C267`): the `0x800Dxxxx` slots of the 46-slot vtable
// `0x80315EE8` of the `Kinopio` panel object
// ===========================================================================
//
// The vtable's install site and class identity are documented in
// `game::stage` (`_C267`): the constructor `0x800BCB80` stores `0x80315EE8` at
// `this + 0` and tags the 0x880-byte object with the `Kinopio` name table
// `0x802F0D80`.
//
// Seven of this table's display-band prologue targets (`0x800D3210`,
// `0x800D3830`, `0x800D4790`, `0x800D49D0`, `0x800D4A50`, `0x800D4980`,
// `0x800D4060`) are shared with the `0x803175E0` table opened by wave 267 lane D
// and already have `_D267` bodies in this file, so they are not duplicated
// here. This section carries the two display targets unique to `0x80315EE8`
// (slots 24 and 25) and slot 23 (`0x800D6200`), which both tables name.

// ---------------------------------------------------------------------------
// Constants introduced by this lane (`_C267` suffix)
// ---------------------------------------------------------------------------

/// `0x802F1350`: 16 rows of `0x24` bytes used by vtable slots 23/24; word `0`
/// is the row's animation name, word `0x20` the row's timer word.
const C267_TABLE_802F1350: usize = 0x802F_1350;

/// `0x8023A0B0(owner, name)`: resolves one animation name to a handle.
const C267_FN_8023A0B0: usize = 0x8023_A0B0;

/// `0x80168520(block, mode)`: switches one animation block's mode.
const C267_FN_80168520: usize = 0x8016_8520;

/// `0x80168080(layout)`: first half of the two-word transform read (returns
/// `f1`).
const C267_FN_80168080: usize = 0x8016_8080;

/// `0x801680C0(layout)`: second half of the two-word transform read (returns
/// `f1`).
const C267_FN_801680C0: usize = 0x8016_80C0;

/// `0x800D5DD0(this, mode, f1, f2, f3)`: the three-float effect dispatcher.
///
/// [`fn_800d5dd0`]'s Rust signature follows Ghidra's `undefined8` reading; the
/// observed register layout (`r3 = this`, `r4 = mode`, `f1..f3`) is modelled
/// by [`c267_call_this_mode_f32x3`] so this lane's call cannot transpose the
/// receiver and the mode.
const C267_FN_800D5DD0: usize = 0x800D_5DD0;

/// `r2 - 0x6604` -> `0x8042CA7C`: `1.0f`, the clamp bias of [`fn_800d62f0`].
const C267_F32_1: usize = SDA2_BASE - 0x6604;

/// `r2 - 0x6648` -> `0x8042CA38`: `0.0f`, the second operand of the
/// [`fn_800d40f0`] tail dispatch.
const C267_F32_0: usize = SDA2_BASE - 0x6648;

// ---------------------------------------------------------------------------
// Wave-267 helpers (`_C267`)
// ---------------------------------------------------------------------------

/// Calls `func(u32, u32, f32, f32, f32) -> u32`.
///
/// The integer pair lands in `r3`/`r4` and the three floats in `f1..f3`,
/// matching the observed `0x800D5DD0(this, mode, f1, f2, f3)` call.
#[inline]
unsafe fn c267_call_this_mode_f32x3(
    addr: usize,
    this: u32,
    mode: u32,
    a: f32,
    b: f32,
    c: f32,
) -> u32 {
    let f: extern "C" fn(u32, u32, f32, f32, f32) -> u32 = core::mem::transmute(addr);
    f(this, mode, a, b, c)
}

// ---------------------------------------------------------------------------
// Bodies
// ---------------------------------------------------------------------------

/// Re-seeds the current row and dispatches one effect (slot 24).
///
/// In plain English: read the row selected by `+0x154`, store `-1` in `+0x158`
/// and the row's `+0x20` word in `+0x164`, resolve the row name through slot
/// `+0x2C` and `0x8023A0B0`, then run virtual `+0x60`. If the `+0x17C` mask
/// selects the effect path and `+0x164` bit `0x2` is clear, call
/// [`fn_800d6460`]; if `+0x164` bit `0x1000` is set, call [`fn_800d6550`];
/// otherwise sample the transform with `0x80168080`/`0x801680C0`, clamp the
/// sample below the row timer, and run virtual `+0x74` with the three floats.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D62F0`.
/// - Extent: `0x800D62F0..0x800D6458` (360 bytes); two zero words follow and
///   the next body starts at `0x800D6460`.
/// - Table: slot 24 of the `0x80315EE8` vtable (word `0x80315F50`, offset
///   `+0x060`).
/// - Evidence: the `0x24` row stride on `+0x154`, the `-1` store, the three
///   `rlwinm` mask tests (`0x6`, `0x2`, `0x1000`), the `fcmpo` + `cror 2,1,2`
///   clamp against the row float, `0x80168080`/`0x801680C0` and the `+0x74`
///   call with `this+0x5C` as `r5`.
///
/// # Arguments
/// - `this`: the panel object; row selector `+0x154`, masks `+0x160`/`+0x164`,
///   flags `+0x17C`, byte `+0x5C`.
/// - `payload`: the incoming `f1` forwarded to the three dispatch helpers.
///
/// # Safety
/// `this` must be live and slots `+0x2C`/`+0x60`/`+0x74` must accept the
/// operands listed above.
///
/// # Confidence
/// medium - the row arithmetic, the masks and the clamp are literal; the
/// payload's role in [`fn_800d6460`]/[`fn_800d6550`] keeps their documented
/// `// inferred:` typing.
pub unsafe fn fn_800d62f0(this: usize, payload: f32) {
    let row = C267_TABLE_802F1350 + read_word(this, 0x154) as usize * 0x24;
    write_word(this, 0x158, 0xFFFF_FFFF);
    write_word(this, 0x164, read_global_word(row + 0x20));

    let resolved = vcall0(this, 0x2c);
    let mut handle = platform::call2(
        C267_FN_8023A0B0,
        resolved,
        read_global_word(row),
    );

    let _ = vcall3(
        this,
        0x60,
        read_word(this, 0x154),
        &mut handle as *mut u32 as usize as u32,
        1,
    );

    if read_word(this, 0x17c) & 0x6 != 0 && read_word(this, 0x164) & 0x2 == 0 {
        fn_800d6460(f64::from(payload), this);
    } else if read_word(this, 0x164) & 0x1000 != 0 {
        fn_800d6550(f64::from(payload).to_bits(), this);
    } else {
        let sample = w99_call1_f32_u32(C267_FN_80168080, (this + 0x28) as u32);
        let limit = f32::from_bits(read_global_word(row + 0x20));
        let clamped = if sample >= limit {
            limit - read_const_f32(C267_F32_1)
        } else {
            sample
        };
        let second = w99_call1_f32_u32(C267_FN_801680C0, (this + 0x28) as u32);
        let entry = w99_slot(this, 0x74);
        let _ = w99_call_f64x3_u32x3(
            entry,
            second as f64,
            clamped as f64,
            payload as f64,
            this as u32,
            &mut handle as *mut u32 as usize as u32,
            platform::read_u8(this + 0x5C) as u32,
        );
    }
}

/// Switches the variant and re-seeds the row timer (slot 25).
///
/// In plain English: compare the signed byte at `+0x774` with the signed byte
/// at `+0x770`. On a change, reset the old `+0x468` block to mode `0`, re-tag
/// the new block through slot `+0x18` with `this + 0x98`, switch both `+0x468`
/// blocks and the old `+0x4A8` record to mode `3`, and, when `+0x17C` bit
/// `0x100` is set, run the two `0x800D4C70` tails plus the two `0x800D4C10`
/// pairs. Then, if `+0x15C == -1`, clamp `+0x154` to zero, clear it to `-1`,
/// sample the two transform floats and dispatch `0x800D5DD0(this, mode,
/// f1, f2, f3)` with `f2 = 0.0f`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D40F0`.
/// - Extent: `0x800D40F0..0x800D4238` (328 bytes); two zero words follow and
///   the next body starts at `0x800D4240`.
/// - Table: slot 25 of the `0x80315EE8` vtable (word `0x80315F54`, offset
///   `+0x064`).
/// - Evidence: `lwz r4,0x774(r3)` / `lwz r0,0x770(r3)` with `extsb` on both,
///   the `<<7` block strides, `0x80168520(block, 0/3)`, virtual `+0x18` with
///   `this+0x98`, the `0x100` flag gate, `0x800D4C70`/`0x800D4C10`, the
///   `+0x15C == -1` gate, `0x80168080`/`0x801680C0` and `0x800D5DD0`.
///
/// # Arguments
/// - `this`: the panel object; previous byte `+0x774`, mode `+0x770`, timer
///   `+0x15C`, pending row `+0x154`.
///
/// # Safety
/// `this` must be live, both `+0x468` blocks must be valid, and slot `+0x18`
/// must accept `this + 0x98`.
///
/// # Confidence
/// medium - every branch, store and call is literal; `0x800D5DD0`'s Rust
/// signature is Ghidra-derived, so the observed `r3`/`r4`/`f1..f3` layout is
/// modelled by [`c267_call_this_mode_f32x3`].
pub unsafe fn fn_800d40f0(this: usize) {
    let previous = platform::read_u8(this + 0x774) as i8;
    let current = read_word(this, 0x770) as i8;

    if previous != current {
        let previous_block =
            this + (((previous as i32 as u32) << 7) & 0x01FF_FFFF) as usize + 0x468;
        let _ = platform::call2(C267_FN_80168520, previous_block as u32, 0);

        let current_block =
            this + (((current as i32 as u32) << 7) & 0x01FF_FFFF) as usize + 0x468;
        let _ = vcall1(current_block, 0x18, (this + 0x98) as u32);

        let previous_record =
            this + (((previous as i32 as u32) << 7) & 0x01FF_FFFF) as usize + 0x4A8;
        let _ = platform::call2(C267_FN_80168520, previous_record as u32, 3);
        let _ = platform::call2(C267_FN_80168520, previous_block as u32, 3);

        if read_word(this, 0x17c) & 0x100 != 0 {
            fn_800d4c70(this as u32, previous_record);
            fn_800d4c70(this as u32, previous_block);
            let current_record =
                this + (((current as i32 as u32) << 7) & 0x01FF_FFFF) as usize + 0x4A8;
            fn_800d4c10(this as u32, current_record, (this + 0x744) as u32);
            fn_800d4c10(this as u32, current_block, (this + 0x694) as u32);
        }
    }

    if read_word(this, 0x15C) == 0xFFFF_FFFF {
        let mut mode = read_word(this, 0x154) as i32;
        if mode < 0 {
            mode = 0;
        }
        write_word(this, 0x154, 0xFFFF_FFFF);

        let third = w99_call1_f32_u32(C267_FN_80168080, (this + 0x28) as u32);
        let first = w99_call1_f32_u32(C267_FN_801680C0, (this + 0x28) as u32);
        let _ = c267_call_this_mode_f32x3(
            C267_FN_800D5DD0,
            this as u32,
            mode as u32,
            first,
            read_const_f32(C267_F32_0),
            third,
        );
    }
}


/// Switches to row `mode` and starts its animation (slot 23).
///
/// In plain English: store `mode` in `+0x158`, copy the row's `+0x20` word into
/// `+0x164`, resolve the row name through virtual `+0x2C` and `0x8023A0B0`,
/// then run virtual `+0x60(mode, &handle, 1)`,
/// `+0x74(&handle, f1, f2, f3, row+0x10)` and `+0x78()`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D6200`.
/// - Extent: `0x800D6200..0x800D62E8` (232 bytes); two zero words follow and
///   the next body starts at `0x800D62F0` ([`fn_800d62f0`]).
/// - Tables: slot 23 of the `0x80315EE8` vtable (word `0x80315F4C`, offset
///   `+0x05C`) and slot 23 of `0x8031C288` (`0x8031C2E4`); both name the same
///   body.
/// - Evidence: the `0x24` row stride into `0x802F1350`, the `+0x158`/`+0x164`
///   stores, slot `+0x2C`, `0x8023A0B0`, slot `+0x60` with `r6 = 1`, slot
///   `+0x74` with the row word at `+0x10` and slot `+0x78`.
///
/// # Arguments
/// - `this_arg`: the panel object; index `+0x158`, timer `+0x164`.
/// - `mode`: row index, `0x24` stride into `0x802F1350`.
/// - `f1`, `f2`, `f3`: the three float operands forwarded to slot `+0x74`.
///
/// # Safety
/// `this_arg` must be live and the row must exist.
///
/// # Confidence
/// high - the row arithmetic, both stores and every virtual slot are literal.
pub unsafe fn fn_800d6200(this_arg: usize, mode: u32, f1: f64, f2: f64, f3: f64) {
    let record = C267_TABLE_802F1350 + mode as usize * 0x24;

    write_word(this_arg, 0x158, mode);
    write_word(this_arg, 0x164, read_global_word(record + 0x20));

    let manager = vcall0(this_arg, 0x2c);
    let mut handle = platform::call2(
        C267_FN_8023A0B0,
        manager,
        read_global_word(record),
    );

    let _ = vcall3(
        this_arg,
        0x60,
        read_word(this_arg, 0x158),
        &mut handle as *mut u32 as usize as u32,
        1,
    );

    let entry = w99_slot(this_arg, 0x74);
    let _ = w99_call_f64x3_u32x3(
        entry,
        f1,
        f2,
        f3,
        this_arg as u32,
        &mut handle as *mut u32 as usize as u32,
        read_global_word(record + 0x10),
    );

    let _ = vcall0(this_arg, 0x78);
}

// ===========================================================================
// Wave 268, lane B (`_B268`): the two `0x800Dxxxx` prologue slots (26 and 28)
// of the 46-slot `0x80315EE8` vtable of the `Kinopio` panel object
// ===========================================================================
//
// The table, its install site (`fn_800BCB80` stores `0x80315EE8` at `this + 0`)
// and its class identity are documented in `game::stage` (`_C267`). The other
// ten bodies this lane translated are `0x800Bxxxx` and live there too.
//
// Both functions below are prologue-shaped display-band methods of the same
// object; neither has a `functions.tsv` row, a Ghidra export or a ledger row.

// ---------------------------------------------------------------------------
// Constants introduced by this lane (`_B268` suffix)
// ---------------------------------------------------------------------------

/// `0x80370DA0`: base of the per-state pitch table read by [`fn_800d4240`].
/// The object's rows start at `+0x878`: three `0x44`-byte rows selected by the
/// clamped `+0x151` state byte, each read at a `4`-byte variant field.
///
/// `lis r30,0x8037` / `addi r30,r30,0x0DA0` at `0x800D4360`/`0x800D4368`; the
/// table itself is REL data and is not mapped in `work/dol/main.elf`, so its
/// rows are documented by address only.
const B268_TABLE_80370DA0: usize = 0x8037_0DA0;

// ---------------------------------------------------------------------------
// Bodies
// ---------------------------------------------------------------------------

/// Applies one model/animation sample to the `+0x28` element (slot 26).
///
/// In plain English: resolve the current view's model name (`+0x778` table by
/// `+0x770`), file the source word under the element's animation key, set the
/// element's two float values, attach the third float to `+0x98`, store the
/// integer part of that float at `+0x18C`, then select `+0x188`: for the
/// "table" flag bit 0 of `+0x160` it indexes the `0x80370DA0` pitch table with
/// the clamped `+0x151` state byte and the variant; otherwise it stores `0.0f`.
/// Finally it refreshes the display context through `0x800D6A30`.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D4240`.
/// - Extent: `0x800D4240..0x800D43E0` (416 bytes); the next body
///   [`fn_800d43e0`] starts at the very next word.
/// - Table: slot 26 of the `0x80315EE8` vtable (word `0x80315F58`, offset
///   `+0x068`).
/// - Evidence: the `+0x770`/`+0x778` name lookup and `0x80239E30`, the
///   `0x801651F0` filing call, the `0x801680E0`/`0x80168FE0` float pairs, the
///   second `+0x28` virtual call and `0x80169F80`, `fctiwz`/`stfd` into
///   `+0x18C`, the `rlwinm.` bit-0 test on `+0x160` that guards the pitch
///   table, `0x8005DC20` clamping `+0x151`, the `0x44` stride, `+0x878`, the
///   `4 * variant` field, `0x800D6A30`, and the `0.0f` (`r2 - 0x6648`)
///   fallback.
/// - Dead code: the `fcmpo f30,f0` / `cror 2,1,2` pair at `0x800D4318` tries
///   to compare the second float with the source record's `+0x20` halfword,
///   but the following `rlwinm.` overwrites CR0 before the only branch at
///   `0x800D4358`, so that compare never selects a path. The translation keeps
///   the machine's actual condition (flag bit 0 only).
///
/// # Arguments
/// - `this`: the panel object; name table `+0x778`, variant `+0x770`, state
///   byte `+0x151`, masks `+0x160`, element `+0x28`, descriptor `+0x98`.
/// - `source`: points at the descriptor word; `*source` is also the record
///   whose unsigned halfword at `+0x20` the original samples in the dead
///   compare described below (it is not forwarded to any callee).
/// - `mode`: fourth argument of `0x801651F0`.
/// - `a`, `b`, `c`: the three incoming floats (`f1..f3`).
///
/// # Side effects
/// One BRRES lookup, one keyed element filing, two animation setters, one
/// attach, two field stores and one display refresh.
///
/// # Safety
/// `this` and `source` must be live, `*source` must point at a readable
/// record, and the element's vtable slots `+0x0C`/`+0x28` must accept the
/// listed operands.
///
/// # Confidence
/// medium - every call, offset, constant and the branch condition are literal;
/// the float roles follow the sibling [`fn_800d43e0`], which shares the same
/// callee sequence.
pub unsafe fn fn_800d4240(
    this: usize,
    source: usize,
    mode: u32,
    a: f32,
    b: f32,
    c: f32,
) {
    let view = read_word(this, 0x770) as usize;
    let records = read_word(this, 0x778) as usize;
    let model_name = read_global_word(records + view * 4 + 0x08);
    let _ = crate::resource::bres::fn_80239e30(this + 0x20C, model_name);

    let record_word = read_word(source, 0);
    let element = this + 0x28;

    let receiver = vcall0(this, 0x28);
    let _ = platform::call4(
        W114_FN_801651F0,
        element as u32,
        receiver,
        &record_word as *const u32 as usize as u32,
        mode,
    );

    w109_call_f64_u32(W114_FN_801680E0, f64::from(a), element as u32);
    w109_call_f64_u32(W114_FN_80168FE0, f64::from(b), element as u32);

    // observed: the export builds the record's unsigned `+0x20` halfword into
    // `f0` and compares it with `f30` (`fcmpo` + `cror 2,1,2`), but the only
    // branch at `0x800D4358` is re-tested from the `rlwinm.` on `+0x160`
    // below, so the compare has no effect on control flow. The second `+0x28`
    // virtual call does not set `f1`; whatever the previous callee left there
    // is therefore not reconstructed.
    let _halfword = read_half(record_word as usize, 0x20);
    let receiver = vcall0(this, 0x28);
    w114_call_f64_u32_u32(
        W114_FN_80169F80,
        f64::from(c),
        receiver,
        (this + 0x98) as u32,
    );

    write_word(this, 0x18C, c as i32 as u32);

    if read_word(this, 0x160) & 1 == 0 {
        write_float(this, 0x188, 0.0);
    } else {
        let lane = crate::game::player::fn_8005dc20(
            B268_TABLE_80370DA0 as u32,
            read_byte(this, 0x151) as u32,
        ) & 0xFF;
        let value = read_const_f32(
            B268_TABLE_80370DA0 + 0x878 + lane as usize * 0x44 + view * 4,
        );
        write_float(this, 0x188, value);
    }

    fn_800d6a30(this);
}

/// Re-binds the five interface entries of the two child objects (slot 28).
///
/// In plain English: resolve the current view's model name, run vtable slot
/// `+0x34` on the objects at `+0x2C` and `+0x64`, then submit the resolved
/// handle to their slot `+0x50` five times with the key/flag pairs
/// `(5,1)`, `(2,1)`, `(1,0)`, `(0,0)` on the first object and `(8,1)` on the
/// second.
///
/// # Original (untabled)
/// - Ghidra: none - no row/export for `0x800D4520`.
/// - Extent: `0x800D4520..0x800D465C` (316 bytes); one zero word follows and
///   the next body starts at `0x800D4660`.
/// - Table: slot 28 of the `0x80315EE8` vtable (word `0x80315F60`, offset
///   `+0x070`).
/// - Evidence: the `+0x770`/`+0x778` lookup and `0x80239E30`, the two
///   `lwz`/`bcctr` pairs on `+0x2C`/`+0x64` through vtable slot `+0x34`, the
///   stack copies of the resolved handle, and the five `bcctr` calls through
///   slot `+0x50` with `r5`/`r6` equal to `5/1`, `2/1`, `1/0`, `0/0` and
///   `8/1`.
///
/// # Arguments
/// - `this`: the panel object; name table `+0x778`, variant `+0x770`, child
///   objects at `+0x2C` and `+0x64`.
///
/// # Side effects
/// Two virtual resets and five virtual interface submissions.
///
/// # Safety
/// `this` must be live, both children constructed and their slot `+0x50`
/// must accept `(receiver, &word, key, flag)`.
///
/// # Confidence
/// high - both child offsets, the slot indices, the five key/flag pairs and
/// their order are literal.
pub unsafe fn fn_800d4520(this: usize) {
    let view = read_word(this, 0x770) as usize;
    let records = read_word(this, 0x778) as usize;
    let model_name = read_global_word(records + view * 4 + 0x08);
    let handle = crate::resource::bres::fn_80239e30(this + 0x20C, model_name);

    let first = read_word(this, 0x2C) as usize;
    let second = read_word(this, 0x64) as usize;
    let _ = vcall0(first, 0x34);
    let _ = vcall0(second, 0x34);

    let mut word = handle;
    let _ = vcall3(first, 0x50, &mut word as *mut u32 as usize as u32, 5, 1);
    let _ = vcall3(first, 0x50, &mut word as *mut u32 as usize as u32, 2, 1);
    let _ = vcall3(first, 0x50, &mut word as *mut u32 as usize as u32, 1, 0);
    let _ = vcall3(first, 0x50, &mut word as *mut u32 as usize as u32, 0, 0);
    let _ = vcall3(second, 0x50, &mut word as *mut u32 as usize as u32, 8, 1);
}

// ===========================================================================
// Wave 273 lane E - untabled gap bodies in the 0x800D main-DOL band (`_E273`)
// ===========================================================================
//
// Four prologue-shaped bodies that no documented extent covered and that the
// wave-266 sweep (`work/wave266/e_candidates.tsv`) listed as still untranslated.
// Each was re-walked against `work/dol/main.elf` for this wave: the first word
// is `stwu r1,-0x10(r1)` + `mflr r0`, the recursive descent ends on a single
// `blr`, and no extent in `src/` starts or overlaps the address.
//
// | address | extent | bytes | role |
// |---|---|---|---|
// | `0x800D7310` | `0x800D7310..0x800D7350` | 64 | deleting destructor, slot 0 of vtable `0x80318A90` |
// | `0x800D9720` | `0x800D9720..0x800D9760` | 64 | deleting destructor named by the `0x80318D80` record |
// | `0x800DB130` | `0x800DB130..0x800DB174` | 68 | derived node constructor, vtable `0x80318E40` |
// | `0x800DB6F0` | `0x800DB6F0..0x800DB734` | 68 | derived node constructor, vtable `0x80318E10` |

/// `0x80318E40`: vtable installed by [`fn_800db130`]. The two RTTI header words
/// are zero and the methods are `0x80044400`, `0x800DB2F0`, `0x800DAF40` and
/// `0x800DB120`.
const W273_VTABLE_80318E40: u32 = 0x8031_8E40;

/// `0x80318E10`: vtable installed by [`fn_800db6f0`]. The two RTTI header words
/// are zero and the methods are `0x800DBC60`, `0x800DB800`, `0x800DB9D0` and
/// `0x800DBBF0`.
const W273_VTABLE_80318E10: u32 = 0x8031_8E10;

/// Deletes the `0x80318A90` holder when the release count is positive.
///
/// The body is the scalar-release shape already documented for the actor list
/// node at [`fn_800dae00`]: `this == 0` or `release <= 0` returns `this`
/// unchanged, otherwise the storage is freed through `0x802B9280(this)`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800D7310`.
/// - Extent: `0x800D7310..0x800D7350` (64 bytes), ending at the `blr` at
///   `0x800D734C`; three zero words at `0x800D7304..0x800D7310` precede it and
///   the next body starts immediately at `0x800D7350`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: slot 0 of the vtable `0x80318A90` (data word `0x80318A98`), whose
///   constructor is [`fn_800d70a0`]. The slot is not inside a pointer run, so
///   this is not a table-target body.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0xC(r1)`; `mr r31,r3`; `bc 12,2`
///   (`beq` when null); `cmpi cr0,0,r4,0`; `bc 4,1` (`blt` when the count is
///   not positive); `bl 0x802B9280`; `mr r3,r31`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: holder returned unchanged and released only on the guarded path.
/// - `release`: signed count; only positive values free `this`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Frees `this` through `0x802B9280` when both guards pass.
///
/// # Safety
/// A non-null `this` with a positive `release` must be a live holder accepted by
/// `0x802B9280`.
///
/// # Confidence
/// high - the two guards, the free call and the returned receiver are literal.
pub unsafe fn fn_800d7310(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        let _ = platform::call1(FN_802B9280, this as u32);
    }
    this
}

/// Deletes the object named by the `0x80318D80` record when `release` is positive.
///
/// Same scalar-release shape as [`fn_800d7310`]. The data word `0x80318D80` is
/// the first word of the four-word record `{0x800D9720, 0x800D9420, 0, -1}`
/// that follows the `0x80318D78` object, so this body is the record's release
/// hook (`0x800D9420` is its sibling body).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800D9720`.
/// - Extent: `0x800D9720..0x800D9760` (64 bytes), ending at the `blr` at
///   `0x800D975C`; three zero words at `0x800D9714..0x800D9720` precede it and
///   [`fn_800d9760`] starts immediately at `0x800D9760`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: data word `0x80318D80`, first word of the four-word record that
///   follows the `0x80318D78` object. Not inside a pointer run.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0xC(r1)`; `mr r31,r3`; `bc 12,2`;
///   `cmpi cr0,0,r4,0`; `bc 4,1`; `bl 0x802B9280`; `mr r3,r31`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: object returned unchanged and released only on the guarded path.
/// - `release`: signed count; only positive values free `this`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Frees `this` through `0x802B9280` when both guards pass.
///
/// # Safety
/// A non-null `this` with a positive `release` must be freeable.
///
/// # Confidence
/// high - the two guards, the free call and the returned receiver are literal.
pub unsafe fn fn_800d9720(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        let _ = platform::call1(FN_802B9280, this as u32);
    }
    this
}

/// Constructs the derived actor-list node whose vtable is `0x80318E40`.
///
/// In plain English: run the base node constructor [`fn_800dadb0`] (which
/// installs `0x80318E58`), overwrite `+0x00` with the derived table, clear the
/// kind byte at `+0x42` and return the node.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800DB130`.
/// - Extent: `0x800DB130..0x800DB174` (68 bytes), ending at the `blr` at
///   `0x800DB170`; three zero words at `0x800DB124..0x800DB130` precede it and
///   three more at `0x800DB174..0x800DB180` follow it.
/// - Reach: no `b` or `bl` in the executable segments targets it and no data
///   word names it. The vtable it installs is not materialised by translated
///   code in `main.dol`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `bl 0x800DADB0`; `lis r3,0x8032`;
///   `li r0,0`; `addi r3,r3,-0x71C0` (`0x80318E40`); `stw r3,0(r31)`;
///   `mr r3,r31`; `stb r0,0x42(r31)`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: node to construct; at least `0x43` writable bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the base constructor, overwrites the vtable and clears the `+0x42`
/// kind byte.
///
/// # Safety
/// `this` must be a writable actor-list node; `0x800DADB0` must be the base
/// constructor.
///
/// # Confidence
/// high - the call, the address pair and both stores are literal; low for the
/// derived-class identity (no RTTI name in the image).
pub unsafe fn fn_800db130(this: usize) -> usize {
    let _ = platform::call1(FN_800DADB0, this as u32);
    write_word(this, 0, W273_VTABLE_80318E40);
    write_byte(this, 0x42, 0);
    this
}

/// Constructs the derived actor-list node whose vtable is `0x80318E10`.
///
/// Same derived-constructor shape as [`fn_800db130`]: base constructor
/// [`fn_800dadb0`], then the `0x80318E10` table and kind byte `4` at `+0x42`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800DB6F0`.
/// - Extent: `0x800DB6F0..0x800DB734` (68 bytes), ending at the `blr` at
///   `0x800DB730`; the zero word at `0x800DB6EC` precedes it and three zero
///   words at `0x800DB734..0x800DB740` follow it.
/// - Reach: no `b` or `bl` in the executable segments targets it and no data
///   word names it.
/// - Table: none; the vtable it installs is not materialised by translated
///   code in `main.dol`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `bl 0x800DADB0`; `lis r3,0x8032`;
///   `li r0,4`; `addi r3,r3,-0x71F0` (`0x80318E10`); `stw r3,0(r31)`;
///   `mr r3,r31`; `stb r0,0x42(r31)`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: node to construct; at least `0x43` writable bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the base constructor, overwrites the vtable and stores kind `4`.
///
/// # Safety
/// `this` must be a writable actor-list node; `0x800DADB0` must be the base
/// constructor.
///
/// # Confidence
/// high - the call, the address pair and both stores are literal; low for the
/// derived-class identity.
pub unsafe fn fn_800db6f0(this: usize) -> usize {
    let _ = platform::call1(FN_800DADB0, this as u32);
    write_word(this, 0, W273_VTABLE_80318E10);
    write_byte(this, 0x42, 4);
    this
}


// ---------------------------------------------------------------------------
// Wave 275 lane E - untabled gap factory for the pause window (`_E275`)
// ---------------------------------------------------------------------------

/// Allocates and constructs the pause window through [`pausewindow_construct`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015A340`.
/// - Extent: `0x8015A340..0x8015A370` (48 bytes, 12 instructions), ending at
///   the `blr` at `0x8015A36C`; two zero words precede it at
///   `0x8015A338..0x8015A340` and the next body is `0x8015A370`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: the only data word naming it is `0x803279DC`; it is not inside a
///   pointer run.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `li r3,0x298`;
///   `stw r0,0x14(r1)`; `bl 0x801628C0`; `cmpi cr0,0,r3,0`;
///   `bc 12,2` (`beq`) to the epilogue; `bl 0x8015A370`; epilogue; `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing; the constructed window is not published by this wrapper.
///
/// # Side effects
/// Allocates 0x298 bytes and runs the pause-window constructor, which installs
/// the `0x80327D30` vtable at `+0x60`.
///
/// # Safety
/// The size-class allocator at `0x801628C0` must be initialised.
///
/// # Confidence
/// high - the allocation size, the null guard and the constructor call are
/// literal.
pub unsafe fn fn_8015a340() {
    let object = platform::call1(0x8016_28C0, 0x298) as usize;
    if object != 0 {
        let _ = pausewindow_construct(object);
    }
}

// ===========================================================================
// Wave 281 lane A (`_A281`) - the eleven uncovered slots of the `dScBoot_c`
// state-object vtable `0x80328A80`
// ===========================================================================
//
// The table is the method array of the state manager installed by
// [`fn_8015fc90`]: header `0x80328A80`/`0x80328A84` (both zero), 14 method
// words at `0x80328A88..0x80328AC0` (56 bytes, end-exclusive), so dispatch is
// `8 + 4*slot`, the convention [`fn_8015fa30`] and [`fn_8015fb80`] already use.
// The word after the array, `0x80328AC0 = 0x25732F25`, is the ASCII text
// `"%s/%s"` - string data, not a zero terminator. Six zero words at
// `0x80328A70..0x80328A88` separate the base method array
// (`0x80328A48..0x80328A70`, ten slots) from this derived method array; the
// last two of those six are this table's zero header.
//
// The base vtable is `0x80328A40` (header) / `0x80328A48` (array), the one
// [`crate::game::task::state_object_init`] writes. Its ten slots are
// `0x8015F9F0`, `0x8015FA30`, `0x8015FAE0`, `0x8015FB80`, `0x8015FC10` and
// `0x80012880..0x80012900`. The derived table replaces slot 0 (deleting
// destructor `0x8000FF10` instead of `0x8015F9F0`) and adds slots 10..13.
//
// Install site: only [`fn_8015fc90`] in this module materialises the header -
// `lis r4,0x8033` at `0x8015FCA8`, `addi r4,r4,-30080` (`-0x7580`) at
// `0x8015FCB0`, `stw r4,0(r31)` at `0x8015FCB4`. No aligned data word holds
// `0x80328A80` or `0x80328A88`, and no REL ELF contains either value.
// `dScBoot_c`'s constructor
// [`crate::game::boot_scene::dsc_boot_construct`] calls [`fn_8015fc90`] twice,
// for the two state managers at `boot + 0x8C` and `boot + 0xC8`, which is the
// class identity recorded for this band.
//
// RTTI: the header type-info word `0x80328A84` is zero (stripped), there is no
// type-info accessor among the 14 slots, and no runtime-type record in the
// image names this table. Slot 0 is the deleting destructor, not the RTTI hook.
//
// Census before this wave: 14 slots, 3 covered / 11 uncovered, 14 distinct
// targets, 0 null, 0 non-text, 0 interior labels and 0 targets contained in
// another slot's measured extent. Of the eleven uncovered targets six are
// `stwu`/`mflr` prologues and five are leaf thunks. The three already-covered
// slots are forwarded, not redefined:
//
// | slot | target | existing body |
// |---:|---|---|
// | 0 | `0x8000FF10` | [`crate::game::path::fn_8000ff10`] |
// | 1 | `0x8015FA30` | [`fn_8015fa30`] |
// | 3 | `0x8015FB80` | [`fn_8015fb80`] |
//
// The eleven bodies below are the remaining slots, in table order. `bcctrl`
// (`0x4E800421`) is a call and conditional `bc`/`bclr` falls through, so each
// extent was measured with both arms. The state object's field roles come from
// [`crate::game::task::state_object_init`] and the two shipped managers:
// `+0x14` attached holder, `+0x18` active holder (initialised to the
// `0x80377A04` interface), `+0x1C` pending holder, `+0x20` result object,
// `+0x0D` busy byte, `+0x0F` repeat byte and `+0x10` again byte.
//
// | # | slot | word | target | extent | bytes | role |
// |---:|---:|---|---|---|---:|---|
// | 1 | 2 | `0x80328A90` | `0x8015FAE0` | `0x8015FAE0..0x8015FB80` | 160 | holder-driven pump |
// | 2 | 4 | `0x80328A98` | `0x8015FC10` | `0x8015FC10..0x8015FC84` | 116 | attach holder / restart |
// | 3 | 5 | `0x80328A9C` | `0x80012880` | `0x80012880..0x8001288C` | 12 | request another pump pass |
// | 4 | 6 | `0x80328AA0` | `0x800128A0` | `0x800128A0..0x800128A8` | 8 | result object getter |
// | 5 | 7 | `0x80328AA4` | `0x800128C0` | `0x800128C0..0x800128C8` | 8 | attached holder getter |
// | 6 | 8 | `0x80328AA8` | `0x800128E0` | `0x800128E0..0x800128E8` | 8 | pending holder getter |
// | 7 | 9 | `0x80328AAC` | `0x80012900` | `0x80012900..0x80012908` | 8 | active holder getter |
// | 8 | 10 | `0x80328AB0` | `0x8015FCD0` | `0x8015FCD0..0x8015FD38` | 104 | enter step (slot 1 -> 10) |
// | 9 | 11 | `0x80328AB4` | `0x8015FD40` | `0x8015FD40..0x8015FD8C` | 76 | pump step (slot 2 -> 11) |
// | 10 | 12 | `0x80328AB8` | `0x8015FD90` | `0x8015FD90..0x8015FDE4` | 84 | leave step (slot 3 -> 12) |
// | 11 | 13 | `0x80328ABC` | `0x8015FDF0` | `0x8015FDF0..0x8015FE3C` | 76 | leave-then-enter restart |

// ---------------------------------------------------------------------------
// 0x8015FAE0 - holder-driven state pump, slot 2
// ---------------------------------------------------------------------------

/// Pumps the state manager while the holder is idle and work is pending.
///
/// In plain English: when the `+0x0D` busy byte is clear, clear the `+0x10`
/// again byte, ask the `+0x14` holder's `+0x0C` query whether it is busy and,
/// when that replies zero, set `+0x0D`, run the object's `+0x34` slot (slot 11,
/// [`fn_8015fd40`]) and clear `+0x0D` again. Then stop when `+0x10` stayed
/// clear; otherwise repeat only while the `+0x0F` repeat byte is set.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015FAE0`.
/// - Extent: `0x8015FAE0..0x8015FB80` (160 bytes, 40 instructions), ending at
///   the `blr` at `0x8015FB7C`; two zero words at `0x8015FAD8..0x8015FAE0`
///   precede it and slot 3 `0x8015FB80` follows it exactly.
/// - Reach: no `b`, `bc` or `bl` in the executable segments targets it and no
///   `lis`/`addi` pair materialises it; the only data words naming it are the
///   slot-2 words `0x80328A50` (base table) and `0x80328A90` (this table).
/// - Observed: `stwu r1,-0x20(r1)`; `mflr r0`; `stw r0,0x24(r1)`;
///   `stw r31,0x1C(r1)`; `stw r30,0x18(r1)`; `stw r29,0x14(r1)`; `mr r29,r3`;
///   `lbz r0,0xD(r3)`; `cmpi cr0,0,r0,0`; `bc 4,2` (bne) to the epilogue;
///   `li r30,1`; `li r31,0`; loop at `0x8015FB10`: `stb r31,0x10(r29)`;
///   `lwz r3,0x14(r29)`; `lwz r12,0(r3)`; `lwz r12,0xC(r12)`; `mtctr r12`;
///   `bcctrl`; `cmpi cr0,0,r3,0`; `bc 4,2` (bne) to the tail test;
///   `stb r30,0xD(r29)`; `mr r3,r29`; `lwz r12,0(r29)`; `lwz r12,0x34(r12)`;
///   `mtctr r12`; `bcctrl`; `stb r31,0xD(r29)`; tail test
///   `lbz r0,0x10(r29)`; `cmpi cr0,0,r0,0`; `bc 12,2` (beq) to the epilogue;
///   `lbz r0,0xF(r29)`; `cmpi cr0,0,r0,0`; `bc 4,2` (bne) back to
///   `0x8015FB10`; epilogue restores `r29`/`r30`/`r31` and returns.
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x0D` busy byte, `+0x0F`
///   repeat byte, `+0x10` again byte, `+0x14` holder pointer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x10` once per iteration and `+0x0D` around each slot-11 call; one
/// holder `+0x0C` query and at most one slot-11 call per iteration.
///
/// # Safety
/// `state` and the `+0x14` holder must be live objects with the cited vtables;
/// with `+0x0F` set the loop runs until the holder reports busy or `+0x10`
/// clears.
///
/// # Confidence
/// high - every offset, the busy guard, the two-part loop condition and both
/// call sites are literal.
pub unsafe fn fn_8015fae0(state: usize) {
    if read_byte(state, 0x0D) != 0 {
        return;
    }
    loop {
        write_byte(state, 0x10, 0);
        let holder = read_word(state, 0x14) as usize;
        let busy = vcall0(holder, 0x0C);
        if busy == 0 {
            write_byte(state, 0x0D, 1);
            let _ = vcall0(state, 0x34);
            write_byte(state, 0x0D, 0);
        }
        if read_byte(state, 0x10) == 0 || read_byte(state, 0x0F) == 0 {
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x8015FC10 - attach a holder/descriptor, slot 4
// ---------------------------------------------------------------------------

/// Attaches a new holder/descriptor to the state manager.
///
/// In plain English: ask `descriptor`'s `+0x0C` query whether it is busy; when
/// it replies zero, store it at `+0x14`, run the object's `+0x3C` slot (slot
/// 13, [`fn_8015fdf0`]) and set the `+0x0F` repeat byte.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015FC10`.
/// - Extent: `0x8015FC10..0x8015FC84` (116 bytes, 29 instructions), ending at
///   the `blr` at `0x8015FC80`; three zero words at
///   `0x8015FC84..0x8015FC90` precede the constructor [`fn_8015fc90`].
/// - Reach: no `b`, `bc` or `bl` in the executable segments targets it and no
///   `lis`/`addi` pair materialises it; the only data words naming it are the
///   slot-4 words `0x80328A58` (base table) and `0x80328A98` (this table).
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r4`; `stw r30,0x8(r1)`; `mr r30,r3`;
///   `mr r3,r31`; `lwz r12,0(r31)`; `lwz r12,0xC(r12)`; `mtctr r12`; `bcctrl`;
///   `cmpi cr0,0,r3,0`; `bc 4,2` (bne) to the epilogue;
///   `stw r31,0x14(r30)`; `mr r3,r30`; `mr r4,r31`; `lwz r12,0(r30)`;
///   `lwz r12,0x3C(r12)`; `mtctr r12`; `bcctrl`; `li r0,1`;
///   `stb r0,0xF(r30)`; epilogue restores `r30`/`r31` and returns.
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x0F` repeat byte, `+0x14`
///   attached holder word.
/// * `descriptor` - candidate holder/descriptor; `+0x00` vtable.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `descriptor + 0x0C` query, then the `+0x14` store, the slot-13 call and
/// the `+0x0F` store when the query returned zero.
///
/// # Safety
/// `state` and `descriptor` must be live objects with the cited vtables.
///
/// # Confidence
/// high - the guard, the store offsets, both vtable offsets and the return
/// shape are literal.
pub unsafe fn fn_8015fc10(state: usize, descriptor: usize) {
    if vcall0(descriptor, 0x0C) != 0 {
        return;
    }
    write_word(state, 0x14, descriptor as u32);
    let _ = vcall0(state, 0x3C);
    write_byte(state, 0x0F, 1);
}

// ---------------------------------------------------------------------------
// 0x80012880 - request another pump pass, slot 5
// ---------------------------------------------------------------------------

/// Marks the state manager as asking for another pump pass.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80012880`.
/// - Extent: `0x80012880..0x8001288C` (12 bytes, 3 instructions), ending at the
///   `bclr 20,0` at `0x80012888`; the query thunk `0x80012870..0x80012880`
///   precedes it and a zero word at `0x8001288C` separates it from the next
///   thunk.
/// - Reach: no `b`, `bc` or `bl` targets it and no `lis`/`addi` pair
///   materialises it; the only data words naming it are the slot-5 words
///   `0x80328A5C` (base table) and `0x80328A9C` (this table).
/// - Observed: `li r0,1`; `stb r0,0x10(r3)`; `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// * `state` - state object with a writable `+0x10` again byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets `+0x10` to 1; [`fn_8015fae0`] consumes and clears it.
///
/// # Safety
/// `state` must be a live state object.
///
/// # Confidence
/// high - the store and the constant are literal.
pub unsafe fn fn_80012880(state: usize) {
    write_byte(state, 0x10, 1);
}

// ---------------------------------------------------------------------------
// 0x800128A0 - result-object getter, slot 6
// ---------------------------------------------------------------------------

/// Returns the `+0x20` result object published by the enter step.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800128A0`.
/// - Extent: `0x800128A0..0x800128A8` (8 bytes, 2 instructions), ending at the
///   `bclr 20,0` at `0x800128A4`; two zero words at
///   `0x800128A8..0x800128B0` precede the next thunk.
/// - Reach: no `b`, `bc` or `bl` targets it and no `lis`/`addi` pair
///   materialises it; the only data words naming it are the slot-6 words
///   `0x80328AA0` (this table) and `0x80328A60` (base table).
/// - Observed: `lwz r3,0x20(r3)`; `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// * `state` - state object whose `+0x20` word holds the enter-step result.
///
/// # Returns
/// The `+0x20` word.
///
/// # Side effects
/// None.
///
/// # Safety
/// `state` must be a live state object.
///
/// # Confidence
/// high - the load offset is literal.
pub unsafe fn fn_800128a0(state: usize) -> u32 {
    read_word(state, 0x20)
}

// ---------------------------------------------------------------------------
// 0x800128C0 - attached-holder getter, slot 7
// ---------------------------------------------------------------------------

/// Returns the `+0x14` attached holder/descriptor.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800128C0`.
/// - Extent: `0x800128C0..0x800128C8` (8 bytes, 2 instructions), ending at the
///   `bclr 20,0` at `0x800128C4`; two zero words at
///   `0x800128C8..0x800128D0` precede the next thunk.
/// - Reach: no `b`, `bc` or `bl` targets it and no `lis`/`addi` pair
///   materialises it; the only data words naming it are the slot-7 words
///   `0x80328AA4` (this table) and `0x80328A64` (base table).
/// - Observed: `lwz r3,0x14(r3)`; `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// * `state` - state object whose `+0x14` word holds the attached holder.
///
/// # Returns
/// The `+0x14` word.
///
/// # Side effects
/// None.
///
/// # Safety
/// `state` must be a live state object.
///
/// # Confidence
/// high - the load offset is literal; the holder role comes from
/// [`fn_8015fc10`] and [`crate::game::task::state_object_init`].
pub unsafe fn fn_800128c0(state: usize) -> u32 {
    read_word(state, 0x14)
}

// ---------------------------------------------------------------------------
// 0x800128E0 - pending-holder getter, slot 8
// ---------------------------------------------------------------------------

/// Returns the `+0x1C` pending holder word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800128E0`.
/// - Extent: `0x800128E0..0x800128E8` (8 bytes, 2 instructions), ending at the
///   `bclr 20,0` at `0x800128E4`; two zero words at
///   `0x800128E8..0x800128F0` precede the next thunk.
/// - Reach: no `b`, `bc` or `bl` targets it and no `lis`/`addi` pair
///   materialises it; the only data words naming it are the slot-8 words
///   `0x80328AA8` (this table) and `0x80328A68` (base table).
/// - Observed: `lwz r3,0x1C(r3)`; `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// * `state` - state object whose `+0x1C` word holds the pending holder.
///
/// # Returns
/// The `+0x1C` word.
///
/// # Side effects
/// None.
///
/// # Safety
/// `state` must be a live state object.
///
/// # Confidence
/// high - the load offset is literal; [`fn_8015fa30`] publishes `+0x14` here
/// and [`fn_8015fb80`] copies it to `+0x18`.
pub unsafe fn fn_800128e0(state: usize) -> u32 {
    read_word(state, 0x1C)
}

// ---------------------------------------------------------------------------
// 0x80012900 - active-holder getter, slot 9
// ---------------------------------------------------------------------------

/// Returns the `+0x18` active holder word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80012900`.
/// - Extent: `0x80012900..0x80012908` (8 bytes, 2 instructions), ending at the
///   `bclr 20,0` at `0x80012904`; two zero words at
///   `0x80012908..0x80012910` precede the next body `0x80012910`.
/// - Reach: no `b`, `bc` or `bl` targets it and no `lis`/`addi` pair
///   materialises it; the only data words naming it are the slot-9 words
///   `0x80328AAC` (this table) and `0x80328A6C` (base table).
/// - Observed: `lwz r3,0x18(r3)`; `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// * `state` - state object whose `+0x18` word holds the active holder.
///
/// # Returns
/// The `+0x18` word.
///
/// # Side effects
/// None.
///
/// # Safety
/// `state` must be a live state object.
///
/// # Confidence
/// high - the load offset is literal; the base constructor seeds `+0x18` with
/// `0x80377A04` and [`fn_8015fb80`] replaces it with `+0x1C`.
pub unsafe fn fn_80012900(state: usize) -> u32 {
    read_word(state, 0x18)
}

// ---------------------------------------------------------------------------
// 0x8015FCD0 - enter step, slot 10
// ---------------------------------------------------------------------------

/// Builds the enter-step result object and publishes it at `+0x20`.
///
/// In plain English: fetch the attached holder through the object's `+0x24`
/// slot (slot 7, [`fn_800128c0`]), pass it to the `+0x08` interface's `+0x0C`
/// slot, store the returned object at `+0x20`, run that object's `+0x0C` slot
/// and report success (`1`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015FCD0`.
/// - Extent: `0x8015FCD0..0x8015FD38` (104 bytes, 26 instructions), ending at
///   the `blr` at `0x8015FD34`; two zero words at
///   `0x8015FD38..0x8015FD40` precede slot 11.
/// - Reach: no `b`, `bc` or `bl` in the executable segments targets it and no
///   `lis`/`addi` pair materialises it; the only data word naming it is the
///   slot-10 word `0x80328AB0` in this table. [`fn_8015fa30`] calls it through
///   vtable `+0x30`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `lwz r12,0(r3)`; `lwz r12,0x24(r12)`;
///   `mtctr r12`; `bcctrl`; `mr r4,r3`; `lwz r3,0x8(r31)`; `lwz r12,0(r3)`;
///   `lwz r12,0xC(r12)`; `mtctr r12`; `bcctrl`; `stw r3,0x20(r31)`;
///   `lwz r12,0(r3)`; `lwz r12,0xC(r12)`; `mtctr r12`; `bcctrl`;
///   `lwz r31,0xC(r1)`; `li r3,1`; epilogue.
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x08` second interface,
///   `+0x14` attached holder, `+0x20` result word.
///
/// # Returns
/// `1`, the enter step's success value.
///
/// # Side effects
/// Three virtual calls and the `+0x20` store.
///
/// # Safety
/// `state` and `state + 0x08`'s target must be live objects with the cited
/// vtables.
///
/// # Confidence
/// medium - the offsets, the `r4` holder operand and the `+0x20` store are
/// literal; the last dispatch reloads neither `r3` nor `r4`, so only its
/// receiver is modelled and the leftover `r4` is not claimed.
pub unsafe fn fn_8015fcd0(state: usize) -> u32 {
    let holder = vcall0(state, 0x24);
    let interface_b = read_word(state, 0x08) as usize;
    let result = vcall1(interface_b, 0x0C, holder);
    write_word(state, 0x20, result);
    let _ = vcall0(result as usize, 0x0C);
    1
}

// ---------------------------------------------------------------------------
// 0x8015FD40 - pump step, slot 11
// ---------------------------------------------------------------------------

/// Runs the enter manager and then the result object's update slot.
///
/// In plain English: call the object's `+0x0C` slot (slot 1,
/// [`fn_8015fa30`]) and then dispatch the `+0x20` result object's `+0x10` slot
/// (slot 2).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015FD40`.
/// - Extent: `0x8015FD40..0x8015FD8C` (76 bytes, 19 instructions), ending at
///   the `blr` at `0x8015FD88`; one zero word at `0x8015FD8C` precedes slot 12.
/// - Reach: no `b`, `bc` or `bl` in the executable segments targets it and no
///   `lis`/`addi` pair materialises it; the only data word naming it is the
///   slot-11 word `0x80328AB4` in this table. [`fn_8015fae0`] calls it through
///   vtable `+0x34`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `lwz r12,0(r3)`; `lwz r12,0xC(r12)`;
///   `mtctr r12`; `bcctrl`; `lwz r3,0x20(r31)`; `lwz r12,0(r3)`;
///   `lwz r12,0x10(r12)`; `mtctr r12`; `bcctrl`; epilogue.
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x20` result object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls.
///
/// # Safety
/// `state` and its `+0x20` result must be live objects with the cited vtables.
///
/// # Confidence
/// high - both receivers, both vtable offsets and the order are literal.
pub unsafe fn fn_8015fd40(state: usize) {
    let _ = vcall0(state, 0x0C);
    let result = read_word(state, 0x20) as usize;
    let _ = vcall0(result, 0x10);
}

// ---------------------------------------------------------------------------
// 0x8015FD90 - leave step, slot 12
// ---------------------------------------------------------------------------

/// Tears the result object down and tells the second interface to clear it.
///
/// In plain English: dispatch the `+0x20` result object's `+0x14` slot (slot
/// 3), then call the `+0x08` interface's `+0x10` slot (slot 2) with the address
/// of the `+0x20` result word as its second argument.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015FD90`.
/// - Extent: `0x8015FD90..0x8015FDE4` (84 bytes, 21 instructions), ending at
///   the `blr` at `0x8015FDE0`; three zero words at
///   `0x8015FDE4..0x8015FDF0` precede slot 13.
/// - Reach: no `b`, `bc` or `bl` in the executable segments targets it and no
///   `lis`/`addi` pair materialises it; the only data word naming it is the
///   slot-12 word `0x80328AB8` in this table. [`fn_8015fb80`] calls it through
///   vtable `+0x38`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `lwz r3,0x20(r3)`; `lwz r12,0(r3)`;
///   `lwz r12,0x14(r12)`; `mtctr r12`; `bcctrl`; `lwz r3,0x8(r31)`;
///   `addi r4,r31,32`; `lwz r12,0(r3)`; `lwz r12,0x10(r12)`; `mtctr r12`;
///   `bcctrl`; epilogue.
///
/// # Arguments
/// * `state` - state object; `+0x00` vtable, `+0x08` second interface,
///   `+0x20` result object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls; the second receives `state + 0x20` in `r4`.
///
/// # Safety
/// `state`, its `+0x20` result and `state + 0x08`'s target must be live
/// objects with the cited vtables.
///
/// # Confidence
/// high - the `addi r4,r31,32` operand and both vtable offsets are literal.
pub unsafe fn fn_8015fd90(state: usize) {
    let result = read_word(state, 0x20) as usize;
    let _ = vcall0(result, 0x14);
    let interface_b = read_word(state, 0x08) as usize;
    let _ = vcall1(interface_b, 0x10, (state + 0x20) as u32);
}

// ---------------------------------------------------------------------------
// 0x8015FDF0 - leave-then-enter restart, slot 13
// ---------------------------------------------------------------------------

/// Restarts the transition by running the leave slot and then the enter slot.
///
/// In plain English: dispatch the object's `+0x14` slot (slot 3,
/// [`fn_8015fb80`]), then dispatch its `+0x0C` slot (slot 1,
/// [`fn_8015fa30`]) with the object as receiver.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8015FDF0`.
/// - Extent: `0x8015FDF0..0x8015FE3C` (76 bytes, 19 instructions), ending at
///   the `blr` at `0x8015FE38`; one zero word at `0x8015FE3C` precedes the next
///   body `0x8015FE40`.
/// - Reach: no `b`, `bc` or `bl` in the executable segments targets it and no
///   `lis`/`addi` pair materialises it; the only data word naming it is the
///   slot-13 word `0x80328ABC` in this table. [`fn_8015fc10`] calls it through
///   vtable `+0x3C`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `lwz r12,0(r3)`; `lwz r12,0x14(r12)`;
///   `mtctr r12`; `bcctrl`; `lwz r12,0(r31)`; `mr r3,r31`;
///   `lwz r12,0xC(r12)`; `mtctr r12`; `bcctrl`; epilogue.
///
/// # Arguments
/// * `state` - state object with the vtable at `+0x00`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls, leave before enter.
///
/// # Safety
/// `state` must be a live state object with the cited vtable.
///
/// # Confidence
/// high - the two offsets, the order and the receiver reload are literal.
pub unsafe fn fn_8015fdf0(state: usize) {
    let _ = vcall0(state, 0x14);
    let _ = vcall0(state, 0x0C);
}

// ===========================================================================
// Wave 281 lane E (`_E281`) - gap batch 11, course-marker and replay bodies
// ===========================================================================
//
// Two untabled gap bodies from `work/wave266/e_candidates.tsv`, re-verified
// against `work/dol/main.elf`. Selection: `work/wave281/e_pick.mjs`; ELF
// re-walk: `work/wave281/e_verify.mjs` / `e_verify_stdout.txt`. Both start with
// `9421FFF0`, each descent ends on exactly one `blr`, neither overlaps a
// documented extent and neither address is named by a pointer-run word.

/// `r2 - 0x65A8` -> `0x8042CAD8`: the four per-row bytes of the course marker.
///
/// observed: `addi r9,r2,-26024` / `lbzx r0,r9,r5` in [`fn_800d77d0`]. The
/// image holds `00 00 01 01`, i.e. rows `0`/`1` flag `0` and rows `2`/`3` flag
/// `1`, immediately followed by the `-16.0f`/`16.0f` edge pair `0x8042CADC`.
const W281E_TABLE_8042CAD8: usize = SDA2_BASE - 0x65A8;

/// Initialises the sweep geometry of one course-marker row.
///
/// The body seeds the two fields the geometry filler leaves alone - the
/// interpolation output at `+0x5C` with the `0.0f` constant that
/// [`fn_800d7350`] also stores at `+0x4C`, and the flag at `+0x75` with the
/// per-row byte from [`W281E_TABLE_8042CAD8`] - then forwards `r4..r8`
/// unchanged to [`fn_800d7350`], caches the shared tick at `+0x70` and rewinds
/// the marker through [`fn_800d79b0`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800D77D0`.
/// - Extent: `0x800D77D0..0x800D7820` (80 bytes, 19 instructions), ending at
///   the `blr` at `0x800D781C`; three zero words precede it at
///   `0x800D77C4..0x800D77CC` (after `0x800D77C0`'s `blr`) and
///   [`fn_800d7820`] starts at `0x800D7820`.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Table: none.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `addi r9,r2,-26024`;
///   `lfs f0,-0x659C(r2)` (the `0.0f` at `0x8042CAE4`); `stw r0,0x14(r1)`;
///   `lbzx r0,r9,r5`; `stw r31,0xC(r1)`; `mr r31,r3`;
///   `stfs f0,0x5C(r3)`; `stb r0,0x75(r3)`; `bl 0x800D7350`;
///   `lwz r0,-0x5480(r13)`; `mr r3,r31`; `stw r0,0x70(r31)`;
///   `bl 0x800D79B0`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `object`: course-marker object; `+0x5C`, `+0x70`, `+0x75` are written.
/// - `count`, `direction`, `offset_a`, `offset_b`, `row`: the five-parameter
///   geometry list of [`fn_800d7350`], forwarded untouched (`row` in `r5` is
///   also the byte-table index).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two seeded fields, the eleven geometry stores of [`fn_800d7350`], the tick
/// cache at `+0x70`, and the state change of [`fn_800d79b0`].
///
/// # Safety
/// `object` must be a live marker with `+0x5C..+0x75` writable and `row` must
/// index the four-entry table.
///
/// # Confidence
/// high - all six incoming registers are accounted for, the two SDA2 operands
/// resolve to `0.0f` and the `00 00 01 01` table is read from the image; the
/// "per-row flag" role of `+0x75` is `// inferred:` from its table source.
pub unsafe fn fn_800d77d0(
    object: usize,
    count: i32,
    direction: i32,
    offset_a: i32,
    offset_b: i32,
    row: i32,
) {
    write_float(object, 0x5C, read_const_f32(W93_F32_SDA2_659C));
    write_byte(
        object,
        0x75,
        platform::read_u8(W281E_TABLE_8042CAD8 + row as usize),
    );
    fn_800d7350(object, count, direction, offset_a, offset_b, row);
    write_word(object, 0x70, platform::read_u32(W89_G_TICK_COUNTER));
    fn_800d79b0(object);
}

/// Acquires the replay object's dDyl slot at `+0x40` and caches its handle.
///
/// The four arguments are reshuffled for
/// [`crate::game::stage::ddyl_slot_acquire`]: `r6` receives the incoming
/// loader heap (`r4`), `r4` the incoming name (`r5`), `r5` becomes the fixed
/// `0` third loader argument, and `r3` becomes `this + 0x40`. A non-zero
/// handle is stored at `+0xEC` and the body reports `1`; a pending or failed
/// load reports `0` without touching the field.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800DDEA0`.
/// - Extent: `0x800DDEA0..0x800DDEF4` (84 bytes, 19 instructions), ending at
///   the `blr` at `0x800DDEF0`; three zero words precede it at
///   `0x800DDE64..0x800DDE6C` (after `0x800DDE60`'s `blr`), then the three
///   16-byte vtable thunks at `0x800DDE70` (`+0x28`), `0x800DDE80` (`+0x18`)
///   and `0x800DDE90` (`+0x10`), each `lwzu r12,0x18(r3)` / `bcctr`. Three zero
///   words follow the body and the untabled gap body `0x800DDF00` starts at
///   `0x800DDF00`; the next `functions.tsv` row `0x800DE020` comes later.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Table: none.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `mr r6,r4`; `mr r4,r5`;
///   `stw r0,0x14(r1)`; `li r5,0`; `stw r31,0xC(r1)`; `mr r31,r3`;
///   `addi r3,r3,0x40`; `bl 0x8008F1B0`; `cmpi cr0,0,r3,0`;
///   `4082000C bc 4,2` (`bne`, to the store); `li r3,0`; `b +0xC`;
///   `stw r3,0xEC(r31)`; `li r3,1`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: replay object owning the dDyl slot at `+0x40`.
/// - `heap`: loader heap, forwarded as the fourth argument.
/// - `name`: resource name/id, forwarded as the second argument.
///
/// # Returns
/// `1` when the slot produced a handle, `0` while the load is pending.
///
/// # Side effects
/// May create/release a loader and writes `+0xEC` on success.
///
/// # Safety
/// `this + 0x40` must be a dDyl slot record; see
/// [`crate::game::stage::ddyl_slot_acquire`].
///
/// # Confidence
/// high - the three register moves, the branch polarity and the `+0xEC` store
/// are literal.
pub unsafe fn fn_800ddea0(this: usize, heap: u32, name: u32) -> u32 {
    let handle = crate::game::stage::ddyl_slot_acquire(this + 0x40, name, 0, heap);
    if handle == 0 {
        return 0;
    }
    write_word(this, 0xEC, handle);
    1
}

// ===========================================================================
// Wave 292 lane E (`_E292`) - the four-slot view reset and the replay-handle
// commit
// ===========================================================================
//
// Re-picked by `work/wave292/e_pick.mjs` from `work/wave266/e_candidates.tsv`
// after the 259 candidates covered by waves 266..291 and the three deferred
// addresses were removed. Both bodies are 120 bytes, start with `9421xxxx`
// (`stwu r1,-N(r1)`), reproduce their extent with `bad = 0`, end on exactly
// one `blr`, do not overlap an extent and are not pointer-run interiors.
//
// `0x800D8A80` is the four-slot partner of [`fn_800dc9a0`] and uses the same
// `0x80377C88` view table; `0x800DDF00` is the commit step directly after
// [`fn_800ddea0`]'s `+0xEC` handle.

// ---------------------------------------------------------------------------
// 0x800D8A80 - reset the secondary marker and release the four slot views
// ---------------------------------------------------------------------------

/// Clears the marker word at `+0x14`, then releases the view of each of the
/// four player slots through `0x802BC9C0`.
///
/// The slot pointers are read at `0x8042A028 + 4 + i*4`, i.e. slots `1..4` of
/// the five-word player array; each slot's `+4` word indexes the
/// `0x80377C88` view table.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x800D8A80`.
/// - Extent: `0x800D8A80..0x800D8AF8` (120 bytes, 30 words), ending at the
///   `blr` at `0x800D8AF4`; the next body `0x800D8B00` starts immediately.
/// - Reach: no `b`/`bl` and no data word in the main image.
/// - Table: [`W38_TABLE_PLAYER_VIEWS`] (`0x80377C88`), indexed by slot `+4`.
/// - Observed: `9421FFE0 stwu r1,-0x20(r1)`; `mflr r0`;
///   `lfs f0,r2,-25708` (zero); `stw r0,0x24(r1)`; `stw r31,0x1C(r1)`;
///   `lis r31,0x8037`; `addi r31,r31,31880` (`0x80377C88`);
///   `stw r30,0x18(r1)`; `li r30,0`; `stw r29,0x14(r1)`; `li r29,0`;
///   `stfs f0,0x14(r3)`; loop `lwz r0,r13,-22104` (`0x8042A028`);
///   `add r3,r0,r30`; `lwz r3,4(r3)`; `lwz r0,4(r3)`;
///   `rlwinm r0,r0,2,0,29`; `lwzx r3,r31,r0`; `bl 0x802BC9C0`;
///   `addi r29,r29,1`; `addi r30,r30,4`; `cmpi cr0,r29,4`;
///   `4180FFD8 bc 12,0` (`bt cr0.lt`, repeat); epilogue; `4E800020 blr`.
/// - Polarity: `4180FFD8` is `bc 12,0` (`bt cr0.lt`), so the loop runs while
///   the index is **less than 4**.
/// - Decoder note: `addi r31,r31,31880` is `0x7C88`, so the table base is
///   `0x80377C88`. The slot read is deliberately `+4` from the array base,
///   so slot `0` is skipped; this is preserved literally.
///
/// # Arguments
/// * `this` - marker/effect object whose `+0x14` float is cleared.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One float store and up to four `0x802BC9C0` view releases.
///
/// # Safety
/// The four entries at slots `1..4` must hold objects whose `+4` word indexes
/// the view table, and each view must be accepted by `0x802BC9C0`.
///
/// # Confidence
/// high - the loop bound, both table folds and the `+4` slot bias are literal.
pub unsafe fn fn_800d8a80(this: usize) {
    write_float(this, 0x14, read_const_f32(W93_F32_SDA2_646C));
    let mut index = 0usize;
    while index < 4 {
        let slot = read_global_word(G_PLAYER_SLOTS + 4 + index * 4) as usize;
        let view_id = read_word(slot, 4) as usize;
        let view = read_global_word(W38_TABLE_PLAYER_VIEWS + view_id * 4);
        let _ = platform::call1(FN_802BC9C0, view);
        index += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x800DDF00 - commit the replay handle into the active slot
// ---------------------------------------------------------------------------

/// Copies `+0xEC` into `+0x5C` and, when the handle is non-zero, runs the
/// packet-record reset `0x800DE290`, clears `+0x6C`, raises bit 0 of `+0xA8`
/// and requests descriptor `0x803719F0` on the `+4` object's `+0x18` virtual.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x800DDF00`.
/// - Extent: `0x800DDF00..0x800DDF78` (120 bytes, 30 words), ending at the
///   `blr` at `0x800DDF74`; the next row `0x800DE020` starts later.
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x807B2164`.
/// - Table: none.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `lwz r0,0xEC(r3)`;
///   `stw r0,0x5C(r3)`; `cmpi cr0,r0,0`; `4082000C bc 4,2`
///   (`bf cr0.eq`, non-zero continues); `li r3,0`; `b 0x800DDF64`;
///   `bl 0x800DE290`; `lhz r0,0xA8(r31)`; `li r3,0`;
///   `stw r3,0x6C(r31)`; `lis r4,0x8037`; `ori r0,r0,1`;
///   `addi r3,r31,4`; `sth r0,0xA8(r31)`; `addi r4,r4,6640`
///   (`0x803719F0`); `lwz r12,4(r31)`; `lwz r12,0x18(r12)`;
///   `mtspr ctr,r12`; `4E800421 bcctrl` (call); `li r3,1`;
///   epilogue; `4E800020 blr`.
/// - Polarity: `4082000C` is `bc 4,2` (`bf cr0.eq`), so a **non-zero**
///   `+0xEC` continues into the reset/commit path; zero returns `0`.
/// - Decoder note: `addi r4,r4,6640` is `0x19F0`, so the descriptor is
///   `0x803719F0`; the virtual call receives `(this + 4, descriptor)` and
///   falls through.
///
/// # Arguments
/// * `this` - replay object whose `+0xEC` handle is committed.
///
/// # Returns
/// `1` after the commit, `0` when the handle is zero.
///
/// # Side effects
/// Writes `+0x5C`, may run `0x800DE290`, clears `+0x6C`, sets bit 0 of the
/// `+0xA8` halfword and issues one `+0x18` virtual request.
///
/// # Safety
/// `this` must be writable through `+0xA8`, and the object at `+4` must carry
/// a callable `+0x18` slot.
///
/// # Confidence
/// high - the guard polarity, both stores, the bit set and the virtual slot
/// are literal.
pub unsafe fn fn_800ddf00(this: usize) -> u32 {
    let handle = read_word(this, 0xEC);
    write_word(this, 0x5C, handle);
    if handle == 0 {
        return 0;
    }
    fn_800de290(this);
    write_word(this, 0x6C, 0);
    let flags = platform::read_u16(this + 0xA8);
    platform::write_u16(this + 0xA8, flags | 1);
    let vtable = read_word(this, 4) as usize;
    let method = read_word(vtable, 0x18) as usize;
    let _ = platform::call2(method, (this + 4) as u32, 0x8037_19F0);
    1
}


// ===========================================================================
// Wave 293 lane C (`_C293`) - the `dGameDisplay_c` vtable `0x803276F0`
// (method array `0x803276F8`, eighteen slots).
//
// The header is the eight-byte `{0, 0}` RTTI/head pair at `0x803276F0`; the
// first method slot is `0x803276F8` (vtable offset `+0x08`) and the next
// table's header is `0x80327740`. The class name is the string
// "`dGameDisplay_c::" at `0x803278DF`, and the header is written at `+0x60`
// by [`fn_80157710`] (`0x80157730 lis r4,0x8032` / `0x80157738 addi
// r4,r4,0x76F0` / `0x8015773C stw r4,0x60(r29)`). The table is:
//
// | slot | address | target | role |
// |---:|---|---|---|
// | 0 | `0x803276F8` | `0x80157A30` | create/start: bind four player slots (this pass) |
// | 1 | `0x803276FC` | `0x8006C540` | covered ([`crate::game::player::fn_8006c540`]) |
// | 2 | `0x80327700` | `0x8006C570` | covered thunk to `0x80161DA0` |
// | 3 | `0x80327704` | `0x801580F0` | tail call `fn_800C9930(this + 0x70)` (this pass) |
// | 4 | `0x80327708` | `0x8006C580` | covered ([`crate::game::player::fn_8006c580`]) |
// | 5 | `0x8032770C` | `0x8006C5B0` | covered thunk to `0x80161F60` |
// | 6 | `0x80327710` | `0x80157F90` | per-frame update/teardown (this pass) |
// | 7 | `0x80327714` | `0x8006C5C0` | covered ([`crate::game::player::fn_8006c5c0`]) |
// | 8 | `0x80327718` | `0x8006C600` | covered thunk to `0x80162110` |
// | 9 | `0x8032771C` | `0x801580A0` | visible-state query (this pass) |
// | 10 | `0x80327720` | `0x8006C610` | covered ([`crate::game::player::fn_8006c610`]) |
// | 11 | `0x80327724` | `0x8006C650` | covered thunk to `0x80162210` |
// | 12 | `0x80327728` | `0x801622D0` | covered (`crate::game::enemies`) |
// | 13 | `0x8032772C` | `0x801625F0` | covered (`crate::game::enemies`) |
// | 14 | `0x80327730` | `0x801627F0` | covered (`crate::game::enemies`) |
// | 15 | `0x80327734` | `0x801628B0` | covered constant `1` |
// | 16 | `0x80327738` | `0x80157960` | deleting destructor (this pass) |
// | 17 | `0x8032773C` | `0x8006C660` | covered constant `0x8031F1B8` |
//
// Thirteen slots were already covered. This block translates the five that
// were not: the four prologue-shaped bodies (`0x80157A30`, `0x80157F90`,
// `0x801580A0`, `0x80157960`) and the eight-byte tail thunk `0x801580F0`.
// The array has eighteen distinct targets, no null slots and no aliases.
// ===========================================================================

/// `0x801586F0` - the create-time "ready" predicate of [`fn_80157a30`].
///
/// `// unknown:` the body is not translated in this lane; the call site only
/// requires a one-argument `u32` result.
const C293_FN_801586F0: usize = 0x8015_86F0;

/// `0x80159960(this, 0)` - first per-player initialiser of [`fn_80157a30`].
const C293_FN_80159960: usize = 0x8015_9960;
/// `0x80159AC0(this, 0)` - second per-player initialiser of [`fn_80157a30`].
const C293_FN_80159AC0: usize = 0x8015_9AC0;
/// `0x80159CB0(this, 0)` - third per-player initialiser of [`fn_80157a30`].
const C293_FN_80159CB0: usize = 0x8015_9CB0;

/// `0x80158580(this)` - rebuilds the four bound player slots.
const C293_FN_80158580: usize = 0x8015_8580;
/// `0x80158480(this)` - rebuilds the HUD bindings.
const C293_FN_80158480: usize = 0x8015_8480;

/// `0x80158100(this)` - commits the copied transform triples.
///
/// observed: the body ends with the same `*(this + 0x78)` slot `+0x10` call
/// that [`fn_80157a30`] issues after it returns, and leaves `r3 = this + 0x70`.
const C293_FN_80158100: usize = 0x8015_8100;

/// `0x80158890(this)` - pre-update step of [`fn_80157f90`].
const C293_FN_80158890: usize = 0x8015_8890;
/// `0x80158910(this)` - post-update step of [`fn_80157f90`].
const C293_FN_80158910: usize = 0x8015_8910;
/// `0x80158A90(this)` - final post-update step of [`fn_80157f90`].
const C293_FN_80158A90: usize = 0x8015_8A90;

/// `0x8015F9F0(this + 0x3A8, 0)` - releases the embedded state manager.
const C293_FN_8015F9F0: usize = 0x8015_F9F0;

/// `0x8016D4B0(this + 0x208)` - releases the sub-object before `operator
/// delete`.
const C293_FN_8016D4B0: usize = 0x8016_D4B0;

/// `0x800E0380(object, -1)` - the save-image record accessor.
const C293_FN_800E0380: usize = 0x800E_0380;

/// `r13 - 0x5461` -> `0x8042A21F`: byte selecting the `+0x490` flag polarity.
///
/// `// unknown:` the owner of the byte is not named in the export; it is read
/// only in [`fn_80157a30`].
const C293_G_BYTE_5461: usize = SDA_BASE - 0x5461;

/// `0x803741B0`: three per-slot mode words tested by the bind loop of
/// [`fn_80157a30`]; a word of `4` enables the `0x800CE210` query.
const C293_TABLE_803741B0: usize = 0x8037_41B0;

/// `0x800CE210(base, area, stage, slot)` - the per-slot bind query.
const C293_FN_800CE210: usize = 0x800C_E210;

/// Clears bit 0 of the flag byte at `object + 0xBB`.
///
/// observed: the original spells this as `lbz r0,0xBB(r3)` /
/// `rlwinm r0,r0,0,24,30` / `stb r0,0xBB(r3)`.
///
/// # Safety
/// `object` must be a live object with a writable `+0xBB` byte.
unsafe fn c293_clear_enabled(object: u32) {
    let base = object as usize;
    write_byte(base, 0xBB, read_byte(base, 0xBB) & !1);
}

/// Sets bit 0 of the flag byte at `object + 0xBB`.
///
/// observed: the original appends `ori r0,r0,1` to the clear sequence.
///
/// # Safety
/// `object` must be a live object with a writable `+0xBB` byte.
unsafe fn c293_set_enabled(object: u32) {
    let base = object as usize;
    write_byte(base, 0xBB, read_byte(base, 0xBB) | 1);
}

/// Creates the `dGameDisplay_c` scene state: binds four player slots, seeds the
/// timers/tables, applies the mode flags and publishes the "created" byte.
///
/// This is slot 0 of `0x803276F0` (vtable offset `+0x08`). The body is guarded
/// by the `+0x44B` created byte; a second call returns `1` immediately.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80157A30`.
/// - Extent: `0x80157A30..0x80157F8C` (1372 bytes, 343 instructions), ending at
///   the `blr` at `0x80157F88`; `0x80157F90` starts at the next word.
/// - Table: slot 0 of `0x803276F0` (word `0x803276F8`).
/// - Evidence: `work/wave293/c_dis_run1.txt`, `c_scan.mjs`.
/// - observed: prologue `9421FFC0 stwu r1,-0x40(r1)` / `7C0802A6 mflr r0` /
///   `48185321 bl 0x802DCD60` (`_savegpr_22`), epilogue
///   `48184E35 bl 0x802DCDAC` (`_restgpr_22`); early exits `beq` to `0x80157F74`
///   return `1` / `0`.
/// - observed stores, in order: `stb 17,+0x7C`; the four-iteration slot seed
///   `stw -1,+0x3CC/+0x3D0/+0x3D4/+0x3D8`, `stw 0,+0x424/+0x428/+0x42C/+0x430`,
///   `stb 0,+0x454..+0x457`; `stw 1,+0x3E8/+0x3E4/+0x3DC`, `stw -1,+0x3E0`,
///   `stw 2,+0x434`; `stw 0,+0x438/+0x440/+0x400/+0x40C/+0x43C`, `stw
///   -1,+0x3F8/+0x3FC`, `stb 0,+0x453`, `stw 30,+0x404/+0x408`; `stfs
///   F32_SDA2_54C0,+0x538/+0x53C/+0x540`; `stb 1/0,+0x448`; the `+0x3EC`
///   loop store `stw 3,+0x3EC+4i` (and `stw 2` on the gate path); the
///   transform commit `stfs out[0],(+0x524)->+0x44`,
///   `stfs out[1],(+0x524)->+0x48`, the same two at `+0x528`, then eight
///   triples of `stfs +0x2C/+0x30/+0x34` from the objects at
///   `+0x524/+0x528/+0x4EC/+0x500/+0x4E8/+0x504/+0x508/+0x520` into
///   `+0x544..+0x5A0`; and finally `stb 0,+0x449/+0x44A/+0x44C/+0x44D/`
///   `+0x44E/+0x44F/+0x450/+0x451`, `stb 1,+0x44B`, `stw 255,+0x418/+0x41C/`
///   `+0x420`.
/// - observed calls, in order: `0x801586F0(this)`; `0x80159880(this,
///   this+0x3CC)`; `0x80159960(this,0)`; `0x80159AC0(this,0)`;
///   `0x80159CB0(this,0)`; `0x80158580(this)`; `0x80158480(this)`;
///   `0x800C9500(this+0x70)`; `0x800C93D0(this+0x70,0,0)`;
///   `0x800E0380(*(r13-0x5640),-1)`; `0x80102750(slot,state,2)`;
///   `0x800CE210(save,slot,state,i)`; `0x800B50F0(r1+8)`;
///   `0x80158100(this)`; then `(*(this+0x78))->+0x10`.
/// - observed flag stores: the byte `+0xBB` bit 0 of the objects at `+0x490`,
///   `+0x4E8`, `+0x4EC`, `+0x4F0`, `+0x4F8`, `+0x4FC`, `+0x520` and of the
///   loop objects `this + i*8 + 0x494` / `+0x498` (`i < 3`).
/// - observed gates: `+0x44B`; `G_HUD_MODE` (`2`/`3`/`1`);
///   `G_1UP_GATE != 0`; `DAT_80315B9D == 0x28`/`== 0x23`;
///   `C293_G_BYTE_5461 != 0`; `G_SUPPRESS_1UP != 0`;
///   `G_DISPLAY_INPUT_FLAGS & 0x10`, `& 0x80000000` and `& 0x40`;
///   `DAT_80315B9C > 9` and `DAT_80315B9D > 0x29` clamp to `0`;
///   `C293_TABLE_803741B0[i] == 4`.
///
/// # Arguments
/// - `this`: `dGameDisplay_c` object; `+0x44B` is the created byte,
///   `+0x70` the layout wrapper, `+0x3CC..+0x5A0` the player/transform state.
///
/// # Returns
/// `0` when `0x801586F0` refuses, otherwise `1`.
///
/// # Safety
/// `this` must be a writable `0x5A4`-byte object whose `+0x490..+0x528` words
/// point at live sub-objects, and the callees' assets must be loaded.
///
/// # Confidence
/// high - every store offset, call target and branch condition is literal; the
/// meaning of the state block is `// inferred:` from the field pattern.
pub unsafe fn fn_80157a30(this: usize) -> u32 {
    if read_byte(this, 0x44B) != 0 {
        return 1;
    }
    if platform::call1(C293_FN_801586F0, this as u32) == 0 {
        return 0;
    }

    write_byte(this, 0x7C, 17);
    let mut index = 0usize;
    while index < 4 {
        write_word(this, 0x3CC + index * 4, 0xFFFF_FFFF);
        write_word(this, 0x424 + index * 4, 0);
        write_byte(this, 0x454 + index, 0);
        index += 1;
    }
    write_word(this, 0x3E8, 1);
    write_word(this, 0x3E4, 1);
    write_word(this, 0x3DC, 1);
    write_word(this, 0x3E0, 0xFFFF_FFFF);
    write_word(this, 0x434, 2);

    fn_80159880(this, this + 0x3CC);
    let _ = platform::call2(C293_FN_80159960, this as u32, 0);
    let _ = platform::call2(C293_FN_80159AC0, this as u32, 0);
    let _ = platform::call2(C293_FN_80159CB0, this as u32, 0);

    let seed = read_const_f32(F32_SDA2_54C0);
    write_word(this, 0x438, 0);
    write_word(this, 0x440, 0);
    write_word(this, 0x3FC, 0xFFFF_FFFF);
    write_word(this, 0x3F8, 0xFFFF_FFFF);
    write_word(this, 0x444, 0);
    write_byte(this, 0x453, 0);
    write_word(this, 0x400, 0);
    write_word(this, 0x404, 30);
    write_word(this, 0x408, 30);
    write_word(this, 0x40C, 0);
    write_word(this, 0x43C, 0);
    write_float(this, 0x540, seed);
    write_float(this, 0x53C, seed);
    write_float(this, 0x538, seed);

    let _ = platform::call1(C293_FN_80158580, this as u32);
    let _ = platform::call1(C293_FN_80158480, this as u32);
    crate::game::stage::fn_800c9500(this + 0x70);

    {
        let mode = read_global_word(G_HUD_MODE);
        let suppress = mode == 2
            || mode == 3
            || read_global_word(G_1UP_GATE) != 0
            || read_global_byte(DAT_80315B9D) == 0x28
            || read_global_byte(C293_G_BYTE_5461) != 0;
        let enabled = read_word(this, 0x490);
        if suppress {
            c293_clear_enabled(enabled);
        } else {
            c293_set_enabled(enabled);
        }
    }

    if read_global_word(G_HUD_MODE) == 1 {
        c293_set_enabled(read_word(this, 0x4E8));
    } else {
        c293_clear_enabled(read_word(this, 0x4E8));
    }

    if read_global_byte(G_SUPPRESS_1UP) != 0 {
        write_byte(this, 0x448, 1);
        c293_set_enabled(read_word(this, 0x4EC));
        c293_clear_enabled(read_word(this, 0x4F0));
        crate::game::stage::fn_800c93d0(this + 0x70, 0, 0);
    } else {
        write_byte(this, 0x448, 0);
        c293_clear_enabled(read_word(this, 0x4EC));
        c293_set_enabled(read_word(this, 0x4F0));
    }

    if read_global_byte(DAT_80315B9D) == 0x23 {
        c293_clear_enabled(read_word(this, 0x4F8));
        c293_clear_enabled(read_word(this, 0x4FC));
        c293_clear_enabled(read_word(this, 0x520));
    } else {
        c293_set_enabled(read_word(this, 0x4F8));
        c293_set_enabled(read_word(this, 0x4FC));
        c293_set_enabled(read_word(this, 0x520));
    }

    let input_flags = read_global_word(G_DISPLAY_INPUT_FLAGS);
    if input_flags & 0x40 != 0 {
        c293_clear_enabled(read_word(this, 0x4FC));
    }

    let save = platform::call2(
        C293_FN_800E0380,
        read_global_word(G_COURSE_5640),
        0xFFFF_FFFF,
    ) as usize;

    let mut slot = read_global_byte(DAT_80315B9C) as u32;
    let mut state = read_global_byte(DAT_80315B9D) as u32;
    if slot > 9 {
        slot = 0;
    }
    if state > 0x29 {
        state = 0;
    }

    if input_flags & 0x10 == 0 && input_flags & 0x8000_0000 == 0 {
        if crate::engine::globals::fn_80102750(slot as i32, state, 2) == 0 {
            c293_clear_enabled(read_word(this, 0x4F8));
        }
    }

    let mut index = 0usize;
    while index < 3 {
        let slot_state = this + index * 4;
        write_word(slot_state, 0x3EC, 3);
        let mut bind = true;
        if read_global_word(C293_TABLE_803741B0 + index * 4) == 4 {
            bind = (platform::call4(
                C293_FN_800CE210,
                save as u32,
                slot,
                state,
                index as u32,
            ) & 0xFF)
                == 0;
        }
        if bind {
            let pair = this + index * 8;
            c293_clear_enabled(read_word(pair, 0x494));
            c293_set_enabled(read_word(pair, 0x498));
            write_word(slot_state, 0x3EC, 2);
        }
        index += 1;
    }

    let mut out = [0u32; 2];
    crate::game::stage::fn_800b50f0(out.as_mut_ptr() as usize);

    let left = read_word(this, 0x524) as usize;
    write_float(left, 0x44, f32::from_bits(out[0]));
    write_float(left, 0x48, f32::from_bits(out[1]));
    let right = read_word(this, 0x528) as usize;
    write_float(right, 0x44, f32::from_bits(out[0]));
    write_float(right, 0x48, f32::from_bits(out[1]));

    let copies: [(usize, usize); 8] = [
        (0x524, 0x544),
        (0x528, 0x550),
        (0x4EC, 0x55C),
        (0x500, 0x568),
        (0x4E8, 0x574),
        (0x504, 0x580),
        (0x508, 0x58C),
        (0x520, 0x598),
    ];
    for (source, destination) in copies {
        let owner = read_word(this, source) as usize;
        write_float(this, destination, read_float(owner, 0x2C));
        write_float(this, destination + 4, read_float(owner, 0x30));
        write_float(this, destination + 8, read_float(owner, 0x34));
    }

    let receiver = platform::call1(C293_FN_80158100, this as u32);

    write_byte(this, 0x449, 0);
    write_byte(this, 0x44A, 0);
    write_byte(this, 0x44B, 1);
    write_byte(this, 0x44C, 0);
    write_word(this, 0x418, 0xFF);
    write_byte(this, 0x44D, 0);
    write_word(this, 0x41C, 0xFF);
    write_byte(this, 0x44E, 0);
    write_word(this, 0x420, 0xFF);
    write_byte(this, 0x44F, 0);
    write_byte(this, 0x450, 0);
    write_byte(this, 0x451, 0);

    // observed: the callee leaves `r3 = this + 0x70`, and this tail call reuses
    // it as the receiver while the entry still comes from `*(this + 0x78)`.
    let vtable = read_word(this, 0x78) as usize;
    let entry = read_word(vtable, 0x10) as usize;
    let _ = platform::call1(entry, receiver);
    1
}

/// Per-frame update of the created `dGameDisplay_c` scene.
///
/// This is slot 6 of `0x803276F0` (vtable offset `+0x20`). When the `+0x44B`
/// created byte is clear the body returns `1` without touching anything.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80157F90`.
/// - Extent: `0x80157F90..0x80158088` (248 bytes, 62 instructions), ending at
///   the `blr` at `0x80158084`; two zero words follow.
/// - Table: slot 6 of `0x803276F0` (word `0x80327710`).
/// - Evidence: `work/wave293/c_dis_run1.txt`.
/// - observed calls: `0x80158890(this)`; `(*(this+0x390))->+0x10` with
///   receiver `this+0x390`; `0x800C9640(this+0x70)`; when its result, `+0x450`
///   or `+0x451` is non-zero: `0x800C9560(this+0x70)` and
///   `(*(this+0x78))->+0x10` with receiver `this+0x70`; then
///   `0x80158910(this)` and `0x80158A90(this)`.
/// - observed stores: `stw value-1,+0x424/+0x428/+0x42C/+0x430`, each only
///   when its word is non-zero (`beq` skips).
/// - observed: the `+0x42C`/`+0x430` pair is reached through `addi r4,this,8`
///   and the offsets `0x424`/`0x428`, i.e. the same absolute addresses.
///
/// # Arguments
/// - `this`: `dGameDisplay_c` scene object.
///
/// # Returns
/// Always `1`, including the not-created early exit.
///
/// # Side effects
/// Up to two interface calls, four callees and four decrements.
///
/// # Safety
/// `this` must be live; `+0x390`/`+0x78` must hold live interfaces and
/// `+0x70` a live layout object.
///
/// # Confidence
/// high - every call, guard and decrement is literal.
pub unsafe fn fn_80157f90(this: usize) -> u32 {
    if read_byte(this, 0x44B) != 0 {
        let _ = platform::call1(C293_FN_80158890, this as u32);
        let _ = vcall0(this + 0x390, 0x10);

        if crate::game::stage::fn_800c9640(this + 0x70) != 0
            || read_byte(this, 0x450) != 0
            || read_byte(this, 0x451) != 0
        {
            crate::game::stage::fn_800c9560(this + 0x70);
            let vtable = read_word(this, 0x78) as usize;
            let entry = read_word(vtable, 0x10) as usize;
            let _ = platform::call1(entry, (this + 0x70) as u32);
        }

        let _ = platform::call1(C293_FN_80158910, this as u32);
        let _ = platform::call1(C293_FN_80158A90, this as u32);

        for offset in [0x424usize, 0x428, 0x42C, 0x430] {
            let value = read_word(this, offset);
            if value != 0 {
                write_word(this, offset, value.wrapping_sub(1));
            }
        }
    }
    1
}

/// Reports whether the created scene is currently visible.
///
/// This is slot 9 of `0x803276F0` (vtable offset `+0x2C`): when `+0x452` is
/// clear the answer is `1`; otherwise the layout wrapper is released first when
/// `+0x44B` is set, and `1` is still returned.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801580A0`.
/// - Extent: `0x801580A0..0x801580E8` (72 bytes, 18 instructions), ending at
///   the `blr` at `0x801580E4`.
/// - Table: slot 9 of `0x803276F0` (word `0x8032771C`).
/// - Evidence: `work/wave293/c_dis_run1.txt`.
/// - observed calls: `0x80006EA0(this+0x70)` on the `+0x452 == 0` fall-through
///   path when `+0x44B != 0`.
/// - observed stores: none outside the 0x10-byte frame.
///
/// # Arguments
/// - `this`: `dGameDisplay_c` scene object.
///
/// # Returns
/// Always `1`.
///
/// # Safety
/// `this` must be live; the `+0x70` layout node must accept the release call
/// when both bytes are set.
///
/// # Confidence
/// high - the two byte tests, the call target and the constant return are
/// literal.
pub unsafe fn fn_801580a0(this: usize) -> u32 {
    if read_byte(this, 0x452) != 0 {
        if read_byte(this, 0x44B) != 0 {
            crate::render::geometry::fn_80006ea0((this + 0x70) as u32);
        }
    }
    1
}

/// Tail-forwards the layout wrapper at `this + 0x70` to `fn_800C9930`.
///
/// This is slot 3 of `0x803276F0` (vtable offset `+0x14`). The eight-byte body
/// is `addi r3,r3,0x70` plus an unconditional `b 0x800C9930`, i.e. the same
/// body as [`crate::game::stage::fn_800c9930`] with the layout bias added.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801580F0`.
/// - Extent: `0x801580F0..0x801580F8` (8 bytes, 2 instructions), ending at the
///   tail branch; the next body `0x80158100` follows the two-byte pad at
///   `0x801580F8`.
/// - Table: slot 3 of `0x803276F0` (word `0x80327704`).
/// - observed: `38630070 addi r3,r3,112`; `4BF7183C b 0x800C9930`.
///
/// # Arguments
/// - `this`: `dGameDisplay_c` scene object; `+0x70` is the layout wrapper.
///
/// # Returns
/// The `u32` returned by `crate::game::stage::fn_800c9930`.
///
/// # Safety
/// `this + 0x70` must be a live layout object.
///
/// # Confidence
/// high - two instructions, both literal.
pub unsafe fn fn_801580f0(this: usize) -> u32 {
    crate::game::stage::fn_800c9930(this + 0x70)
}

/// Deleting destructor of the `dGameDisplay_c` scene object.
///
/// This is slot 16 of `0x803276F0` (vtable offset `+0x48`). With a non-null
/// `this` it clears the published scene pointer, releases the state manager at
/// `+0x3A8`, retags/`delete`s the sub-object at `+0x208`, releases the layout
/// wrapper and the player-side base object, and frees the block when
/// `destroy > 0`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80157960`.
/// - Extent: `0x80157960..0x80157A24` (196 bytes, 49 instructions), ending at
///   the `blr` at `0x80157A20`; `0x80157A30` starts at the next word.
/// - Table: slot 16 of `0x803276F0` (word `0x80327738`).
/// - Evidence: `work/wave293/c_dis_run1.txt`.
/// - observed stores: `stw 0,W90_G_SCENE_5358` (`0x8042A328`, the global
///   published by [`fn_80157710`]); `stw VTABLE_80329958,0x208(this)` before
///   the sub-object release.
/// - observed calls: `0x8015F9F0(this+0x3A8,0)` guarded by the two `addic.`
///   tautologies on `this+0x390`/`this+0x3A8`; `0x8016D4B0(this+0x208)`;
///   `0x802D7B20(this+0x208,0)`; `0x800C89A0(this+0x70,-1)`;
///   `0x8006C490(this,0)`; and `0x80162920(this)` when `destroy > 0`
///   (`bc 4,1` = `ble` over the call).
/// - observed: the `+0x208` block is entered through `addic. r31,r29,0x208`,
///   so the `cmpwi r31,0` before `0x802D7B20` is a tautology kept from the C.
///
/// # Arguments
/// - `this`: scene object, or `0` for a no-op.
/// - `destroy`: positive to free the block after teardown.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or a live `dGameDisplay_c` whose `+0x208`/`+0x3A8`/
/// `+0x70` members are live and whose interfaces accept the release calls.
///
/// # Confidence
/// high - every guard, store and call target is literal; the `addic.` tests
/// are the same tautology shape already documented for
/// [`crate::game::player::fn_800671b0`].
pub unsafe fn fn_80157960(this: usize, destroy: i32) -> usize {
    if this != 0 {
        write_global_word(W90_G_SCENE_5358, 0);

        if this + 0x390 != 0 && this + 0x3A8 != 0 {
            let _ = platform::call2(C293_FN_8015F9F0, (this + 0x3A8) as u32, 0);
        }

        if this + 0x208 != 0 {
            write_word(this, 0x208, VTABLE_80329958);
            let _ = platform::call1(C293_FN_8016D4B0, (this + 0x208) as u32);
            let _ = crate::runtime::cpp::fn_802d7b20(this + 0x208, 0);
        }

        let _ = crate::game::stage::fn_800c89a0(this + 0x70, -1);
        let _ = crate::game::player::fn_8006c490(this, 0);

        if destroy > 0 {
            crate::resource::registry::fn_80162920(this as u32);
        }
    }
    this
}
