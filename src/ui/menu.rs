//! Home-button menu and dialog helpers (`hmMenu`, `T_Dialog`).
//!
//! # What lives here
//!
//! This module is the semantic home for the Wii home-button overlay and its
//! dialog data model. One hundred fifty-one functions are translated here.
//! The wave tables below list the individual passes; the latest is wave 114,
//! which adds the last open body of the `0x80170000` bucket, the text-run page
//! draw `0x801701A0`:
//!
//! | pass | bodies | subject |
//! |---|---:|---|
//! | original | 8 | data-model helpers |
//! | wave 3 | 11 | dialog/button bodies |
//! | wave 4 | 10 | confirm, dismiss and vertex buffers |
//! | wave 7 | 14 | home-menu grid and dialog widgets |
//! | wave 12 | 10 | animation lists and splines |
//! | wave 19 | 16 | item records and entry points |
//! | wave 21 | 18 | per-player widgets and menu boot |
//! | wave 26 | 18 | widget setters, grid/record helpers and wrappers |
//! | wave 28 | 18 | records, timers, wrappers and GX state |
//! | wave 32 | 2 | quad draw and dialog-tree tagging |
//! | wave 102 | 19 | the remaining band-P and home-menu bodies |
//! | wave 108 | 6 | page state machines and the scrolling-text updater |
//! | wave 114 | 1 | the last `0x80170000` text-run page body |
//! | wave 273 B | 10 | `homebutton::gui::PaneManager` (`0x80329D78`) child loops, teardown and list helpers |
//! | wave 274 A | 18 | `PaneManager` (`0x80329D78`) close-out and `PaneComponent` (`0x80329CD0`) accessors |
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8017CF10` | [`menu_button_index`] | resolve a pane name to a `B_btnL_00` / `B_btn_00` index |
//! | `0x8017CFC0` | [`menu_dialog_result`] | return the selected result word for dialog type `0x12` |
//! | `0x8017DFA0` | [`menu_find_list_node`] | find a list node by id |
//! | `0x8017E270` | [`menu_bind_message`] | bind a message object and copy its text |
//! | `0x8017E310` | [`menu_find_named_value`] | find a named node and return its value |
//! | `0x8017E560` | [`menu_event_is_button`] | test the fixed button event `0xFEFF/8` |
//! | `0x8017E590` | [`menu_event_matches`] | test an event id against the fixed button event |
//! | `0x8017E8E0` | [`menu_scale_color_alpha`] | copy a colour and scale its alpha byte |
//!
//! # Original address ranges
//!
//! | band | addresses | entries | what it contains |
//! |---|---|---|---|
//! | N | `0x8016FD30..0x80175510` | 100 (87 eligible) | home-button menu (`hmMenu`); strings `P1_Def_brlyt`, `hmMenu_strt`, `_cntBtn_in_brlan`, `midi_00_home_button_mid`, `HomeButtonSe` |
//! | O | `0x80175D70..0x8017CF0F` | 19 (19 eligible) | home-button dialogs/battery UI; strings `T_Dialog`, `T_msg_00/01`, `btryPwr_00_0`, `B_btn_00`, `B_optnBtn_00` |
//! | P | `0x8017CF10..0x8017F980` | 44 (42 eligible) | remaining dialog/menu input helpers; strings `B_btnL_00`, `B_btn_00` |
//!
//! The original bodies are in band P except [`menu_button_index`], which is the
//! first entry of band P and reads the `B_btnL_00` / `B_btn_00` name tables
//! from band O. Wave 4 starts in band O at `0x8017CCA0` and continues through
//! the band-P helpers to `0x8017F940`. The ranges belong to the address lane
//! `0x80140000..0x80180000`, documented in `docs/COVERAGE_r80140_80180.md`;
//! this module's wave-4 coverage is tracked in
//! `docs/COVERAGE_WAVE4_DISPLAY.md`.
//!
//! # Why it exists
//!
//! The home-button overlay is UI *behaviour* (menu state, dialogs, battery
//! warning), while [`crate::ui::lyt`] is the NW4R layout/animation core owned
//! by another promotion lane. Keeping the overlay here lets this module depend
//! on the layout primitives without taking ownership of them.
//!
//! # Object model used here (observed)
//!
//! | owner | offset | width | meaning | confidence |
//! |---|---|---|---|---|
//! | menu object | `+0x0B4` | char[] | inline pane/button name compared by [`menu_button_index`] | observed |
//! | button table | `+0x00C` | word | number of `B_btnL_00` entries | observed |
//! | dialog object | `+0x014` | word | dialog type; `0x12` is the result-bearing dialog | observed |
//! | dialog object | `+0x0B8` | word | result/selection word returned by [`menu_dialog_result`] | observed |
//! | list owner | `+0x004` | list | intrusive list; nodes' first word is `next`, `+0x08` is the id | observed |
//! | named-list owner | `+0x028` | list | intrusive list; `+0x08` is a name buffer, `+0x88` the value | observed |
//! | message node | `+0x020` | word | source message object stored by [`menu_bind_message`] | observed |
//! | message node | `+0x030` | char[0x7F] | copied message text | observed |
//! | message node | `+0x0AF` | byte | cleared after a successful bind | observed |
//! | event record | `+0x000` | word | event id | observed |
//! | event record | `+0x004` | u16 | event class `0xFEFF` (`-0x101`) | observed |
//! | event record | `+0x006` | u16 | event code `8` | observed |
//! | colour word | `+0x003` | byte | alpha component (big-endian `0xRRGGBBAA`) | observed |
//!
//! The `0xFEFF/8` pair is used only through the equality tests in
//! [`menu_event_is_button`] and [`menu_event_matches`]; the export does not name
//! the button, so it is left as the literal pair.
//!
//! # Dependencies
//!
//! [`crate::platform`] for volatile loads/stores and the `strcmp`/`strncpy`
//! calls at `0x802E1AB4` / `0x802E19F8`. The layout objects and pane names live
//! in the `hmMenu`/`T_Dialog` resources; no allocation is performed here and
//! the crate remains `no_std`.
//!
//! # Start here
//!
//! [`menu_button_index`] is the strongest entry point: it is the only function
//! here that directly reads the `B_btnL_00` and `B_btn_00` name tables. Then
//! read [`menu_bind_message`] and [`menu_find_named_value`] for the dialog data
//! model, and the two event predicates for the input shape.
//!
//! # Wave 3: dialog tables and button pages (`0x80175D70..0x8017E7D0`)
//!
//! The second pass added eleven bodies from the dialog/home-button data model:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80175D70` | [`dialog_parse_entry`] | split a `name,name,0/1,...` row and record its flags |
//! | `0x80175EE0` | [`dialog_split_quoted`] | record pointers to the quoted strings of a UTF-16 row |
//! | `0x80179E60` | [`menu_refresh_buttons`] | show/hide the ten `B_btn_00` panes for menu page 0/1/2 |
//! | `0x8017CFE0` | [`menu_set_button_layout`] | compact/full button-bar mode |
//! | `0x8017D3E0` | [`menu_reset_player_animations`] | reset the four player widgets |
//! | `0x8017D430` | [`dialog_keyframe_lookup_u16`] | binary-search a `(time, id)` key table |
//! | `0x8017E210` | [`dialog_node_construct`] | construct a dialog node with an embedded list |
//! | `0x8017E500` | [`menu_name_eq16`] | 16-byte name prefix test |
//! | `0x8017E530` | [`menu_name_eq20`] | 20-byte name prefix test |
//! | `0x8017E6D0` | [`dialog_init_vertex_rows`] | fill `count` 32-byte vertex rows with one colour |
//! | `0x8017E7D0` | [`dialog_copy_vertex_rows`] | copy `count` 32-byte vertex rows |
//!
//! `0x80175D70`/`0x80175EE0` parse the `T_Dialog`/`T_msg_*` text rows;
//! `0x8017E6D0`/`0x8017E7D0` fill the vertex buffer that draws them. The
//! `0x8017CFE0..0x8017E530` group is the home-button menu state used by
//! [`menu_button_index`].
//!
//! # Wave 4: confirm, dismiss and vertex buffers (`0x8017CCA0..0x8017F940`)
//!
//! The third pass adds the pending-selection confirm path, the dialog dismiss
//! path and the small constructors/buffer helpers around them:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8017CCA0` | [`menu_confirm_pending`] | activate the pending menu entries and selection panes |
//! | `0x8017D1D0` | [`dialog_begin_dismiss`] | dismiss/finish a dialog and choose its timer path |
//! | `0x8017D700` | [`dialog_panel_construct`] | construct a small dialog panel object |
//! | `0x8017DFE0` | [`dialog_find_named_entry`] | recursively find a named entry in the parsed document |
//! | `0x8017E5F0` | [`dialog_vertex_buffer_free`] | free the vertex-row buffer |
//! | `0x8017E640` | [`dialog_vertex_buffer_reserve`] | grow the vertex-row buffer to `count` rows |
//! | `0x8017E870` | [`dialog_record_has_value`] | test whether a four-word record is unset |
//! | `0x8017F6E0` | [`dialog_list_insert`] | insert a node after the embedded list head |
//! | `0x8017F720` | [`dialog_find_child_by_name`] | find a child node by 16-byte name |
//! | `0x8017F940` | [`dialog_group_construct`] | construct a dialog group with an embedded list |
//!
//! # Wave 7: home-menu grid and dialog widgets (`0x8016FE40..0x8017E920`)
//!
//! The fourth pass adds the home-button grid descriptor helpers and the two
//! dialog-widget setters/toggles that the earlier passes only called by
//! address:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8016FE40` | [`fn_8016fe40`] | release one grid entry |
//! | `0x8016FE90` | [`fn_8016fe90`] | create the grid resource |
//! | `0x80170110` | [`fn_80170110`] | release every grid entry and reset the list |
//! | `0x80170670` | [`fn_80170670`] | compute grid dimensions, page counts and cell area |
//! | `0x801706E0` | [`fn_801706e0`] | page number for a coordinate |
//! | `0x80170700` | [`fn_80170700`] | in-page cell offset |
//! | `0x80170730` | [`fn_80170730`] | global cell index with bounds check |
//! | `0x80170830` | [`fn_80170830`] | write one four-bit cell value |
//! | `0x80170900` | [`fn_80170900`] | flush the whole grid buffer |
//! | `0x8017D3C0` | [`fn_8017d3c0`] | store the three dialog-mode bytes |
//! | `0x8017D3D0` | [`fn_8017d3d0`] | store the dialog timer |
//! | `0x8017D6F0` | [`fn_8017d6f0`] | read the current message id |
//! | `0x8017E5D0` | [`fn_8017e5d0`] | clear the vertex-row counters |
//! | `0x8017E920` | [`fn_8017e920`] | enable and commit the dialog pane group |
//!
//! The grid descriptor is `{ width, height, cell_w, cell_h, area, columns,
//! rows, pages, buffer }` at `+0..+0x24`; the two nibble writers pack two
//! cells per byte. The dialog toggles write the bytes/word used by the
//! existing dismiss and vertex-buffer paths.
//!
//! # Wave 12: animation state and dialog helpers (`0x801713C0..0x8017E380`)
//!
//! The fifth pass adds the menu-item animation record setter, the pane-list
//! activation walk, the four small list helpers used by the home-menu state
//! machine, the dialog cubic-key evaluator and one alternate dialog-node
//! constructor:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x801713C0` | [`fn_801713c0`] | configure one animation record |
//! | `0x80171570` | [`fn_80171570`] | construct the dialog pane-list object |
//! | `0x801715D0` | [`fn_801715d0`] | activate/deactivate every pane in the list |
//! | `0x80172CE0` | [`fn_80172ce0`] | read the list-entry flag bit |
//! | `0x80172CF0` | [`fn_80172cf0`] | read the child pointer |
//! | `0x80173860` | [`fn_80173860`] | query one list entry's enabled byte |
//! | `0x801738F0` | [`fn_801738f0`] | close one active list entry |
//! | `0x80174330` | [`fn_80174330`] | convert an enabled byte to `0/1` |
//! | `0x8017D520` | [`fn_8017d520`] | evaluate one channel of a cubic key table |
//! | `0x8017E380` | [`fn_8017e380`] | construct an alternate dialog list node |
//!
//! `0x8017D520` is the evaluator called by the dialog animation task in
//! `0x8017DCC0`; it uses the same tolerance pair (`0x802F5E10/14`) and four
//! interpolation coefficients (`0x802F5E18/1C/20/24`) that the earlier
//! keyframe lookup documented.
//!
//! # Wave 19: item records and home-button entry points
//!
//! The sixth pass adds the remaining home-menu record helpers and the small
//! call-throughs that connect the overlay to the dialog bodies already in
//! this module:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80171400` | [`menu_item_recompute_current`] | copy the mode-selected item value into the live slot |
//! | `0x80173880` | [`fn_80173880`] | start one entry action |
//! | `0x80173920` | [`fn_80173920`] | submit one entry action |
//! | `0x801739E0` | [`fn_801739e0`] | read a per-player menu byte |
//! | `0x80173A10` | [`fn_80173a10`] | clear a per-player menu byte |
//! | `0x80173CA0` | [`fn_80173ca0`] | reset the four entry records |
//! | `0x80173F20` | [`fn_80173f20`] | clear the block timer and four record timers |
//! | `0x80173F80` | [`fn_80173f80`] | install the one-argument callback |
//! | `0x80174090` | [`fn_80174090`] | install the four-argument callback |
//! | `0x801742E0` | [`fn_801742e0`] | test an entry's enabled word |
//! | `0x80174300` | [`fn_80174300`] | test enabled word plus selection id |
//! | `0x801744F0` | [`fn_801744f0`] | notify/reset the active menu manager |
//! | `0x80174F60` | [`fn_80174f60`] | open a dialog through the live menu |
//! | `0x80175040` | [`fn_80175040`] | apply the three dialog-mode bytes |
//! | `0x801754B0` | [`fn_801754b0`] | allocate and construct the menu object |
//! | `0x80175500` | [`fn_80175500`] | return the live menu object |
//!
//! The four 0x68-byte records are the same entries used by
//! [`fn_801738f0`] and `0x80174330`; [`fn_801742e0`] is the strict
//! enabled-word test, while `0x80174330` additionally requires the byte at
//! `+0x62`. The wrappers [`fn_80174f60`]/[`fn_80175040`] are the public
//! entry points used by the pause/home-button overlay.
//!
//! # Wave 21: per-player widgets, animation records and menu boot
//!
//! The seventh pass adds the four-player widget table, the shared 0x2E2C-byte
//! animation records, and the pause-menu boot/shutdown path:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80172EA0` | [`fn_80172ea0`] | per-player input hook / teardown |
//! | `0x80172FB0` | [`fn_80172fb0`] | per-player event hook and retrigger |
//! | `0x801730C0` | [`fn_801730c0`] | per-player hook timer callback |
//! | `0x80173130` | [`fn_80173130`] | construct and register one player widget |
//! | `0x80173290` | [`fn_80173290`] | uninstall one player widget's hooks |
//! | `0x80173310` | [`fn_80173310`] | copy an event colour record into a widget |
//! | `0x80173510` | [`fn_80173510`] | per-frame widget update and hold timer |
//! | `0x80173FB0` | [`fn_80173fb0`] | state callback: enter the four-argument handler |
//! | `0x801740C0` | [`fn_801740c0`] | state callback: four-argument handler |
//! | `0x801741B0` | [`fn_801741b0`] | reset one 0x68-byte record and install a callback |
//! | `0x80174240` | [`fn_80174240`] | lock and write one animation-record slot |
//! | `0x80174350` | [`fn_80174350`] | take an animation record from the free pool |
//! | `0x80174560` | [`fn_80174560`] | pump and retire finished animation records |
//! | `0x801747A0` | [`fn_801747a0`] | play the home-button MIDI sound and link its record |
//! | `0x801748E0` | [`fn_801748e0`] | initialise the pause/home-button menu |
//! | `0x80174AF0` | [`fn_80174af0`] | release every animation record on shutdown |
//! | `0x80174E10` | [`fn_80174e10`] | choose the menu viewport and boot the menu |
//! | `0x80175330` | [`fn_80175330`] | construct the home-button menu object |
//!
//! The widget table is `0x80377F88[4]`; each widget owns a `+0x24` chained
//! callback, a `+0x28` saved callback and a `+0x30` 64-bit timestamp. The
//! animation records are 0x2E2C bytes each: four in the normal pool at `+0`,
//! three in the alternate pool at `+0xB8B0`, with `+0x2E1C` active, `+0x2E20`
//! previous, `+0x2E24` next and `+0x2E28` id. [`fn_801748e0`] is the start
//! point: it clears all seven records, loads the two `HomeButtonSe` sounds and
//! starts the 0x80174560 pump.
//!
//! # Wave 26: widget setters, grid/record helpers and menu wrappers
//!
//! The eighth pass adds the small accessors around the per-player widget and
//! animation-record tables, the four grid misc helpers that the earlier passes
//! only called by address, and the first three pause/home-button wrappers:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x8016FFC0` | [`fn_8016ffc0`] | store the grid's `+4` word |
//! | `0x80170630` | [`fn_80170630`] | create the grid buffer object |
//! | `0x80170CD0` | [`fn_80170cd0`] | destroy a grid object and optionally free it |
//! | `0x80171DE0` | [`fn_80171de0`] | read one list entry's value word |
//! | `0x80172000` | [`fn_80172000`] | close every pane in the list |
//! | `0x80172180` | [`fn_80172180`] | store one byte at `+0x24` |
//! | `0x80172E70` | [`fn_80172e70`] | store the widget's `+4` word |
//! | `0x80172E90` | [`fn_80172e90`] | store the widget's `+0x24` word |
//! | `0x801731F0` | [`fn_801731f0`] | install the two channel hooks and connection byte |
//! | `0x801734A0` | [`fn_801734a0`] | clear three widget words |
//! | `0x801734C0` | [`fn_801734c0`] | copy the two default words into a widget |
//! | `0x801734E0` | [`fn_801734e0`] | reset a record and install its callback |
//! | `0x80173610` | [`fn_80173610`] | clear the widget hold bytes |
//! | `0x80174CC0` | [`fn_80174cc0`] | update both record pools from a level value |
//! | `0x80174F30` | [`fn_80174f30`] | release-build print, menu reset and pane build |
//! | `0x80174FD0` | [`fn_80174fd0`] | apply the compact/full button mode |
//! | `0x801750A0` | [`fn_801750a0`] | store the menu timer word |
//! | `0x801750E0` | [`fn_801750e0`] | initialise the sound/menu manager |
//!
//! `0x801731F0` is the useful entry point: it installs the two channel hooks
//! translated in wave 21 and sets the widget connection byte `+0x40` from the
//! controller query `0x801E0F40`.
//!
//! # Wave 28: record, timer, wrapper and GX-state bodies
//!
//! The ninth pass adds the small bodies around the per-player widget, the
//! four-record scrolling-text group and the two GX frame resets:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80171C30` | [`fn_80171c30`] | clear nine record state bytes |
//! | `0x80172330` | [`fn_80172330`] | conditional delete of a UI record |
//! | `0x80172BE0` | [`fn_80172be0`] | walk a chain of up to five enabled records |
//! | `0x801735B0` | [`fn_801735b0`] | select record 5 and test its selector |
//! | `0x80173620` | [`fn_80173620`] | advance the widget press/hold timer |
//! | `0x80173A30` | [`fn_80173a30`] | update the four scrolling-text widgets |
//! | `0x80173CD0` | [`fn_80173cd0`] | construct the four-record widget group |
//! | `0x80173E40` | [`fn_80173e40`] | arm the scrolling-text timer |
//! | `0x80174DF0` | [`fn_80174df0`] | allocate through the viewport allocator |
//! | `0x80174FA0` | [`fn_80174fa0`] | refresh the battery gauge of the live menu |
//! | `0x80175010` | [`fn_80175010`] | start the live menu's dialog dismiss |
//! | `0x80175140` | [`fn_80175140`] | reset the live menu's player animations |
//! | `0x80175170` | [`fn_80175170`] | reset the GX state for the text/quad pass |
//! | `0x80178A20` | [`fn_80178a20`] | feed one widget's text or arm its timer |
//! | `0x80178B40` | [`fn_80178b40`] | poll the release query and rearm the timer |
//! | `0x8017E490` | [`fn_8017e490`] | dialog-record element destructor |
//! | `0x8017F0B0` | [`fn_8017f0b0`] | draw the dialog text quad |
//!
//! [`fn_80173cd0`]/[`fn_80173e40`] build the 0x68-byte record group that
//! [`fn_80173a30`] scrolls; [`fn_80173620`] is the matching press/hold state
//! machine for the same widget object. The three wrappers
//! `0x80174FA0..0x80175140` forward the live menu to already-translated
//! bodies, exactly like the wave-26 wrappers.
//!
//! # Wave 32: quad draw and dialog-tree tagging
//!
//! The tenth pass adds the two remaining untranslated bodies at the two ends
//! of the file's address range:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x80171680` | [`fn_80171680`] | set the GX state and push one quad word |
//! | `0x8017F7A0` | [`fn_8017f7a0`] | recursively tag the dialog list tree with one value |
//!
//! `0x80171680` is a `0x801Cxxxx` GX-state sequence like [`fn_80175170`] and
//! [`fn_8017f0b0`]: it programs TEV stages 9 and 0xB, texture 0, channels 4/5,
//! blend mode 1, the texture/colour arrays and the viewport, then writes the
//! word at `param_2` to the FIFO at `0xCC008000`. `0x8017F7A0` walks the
//! intrusive lists reached through virtual slot `+0xC` of a node's vtable and
//! stores its second argument at `+0xF4` of every object whose list contains
//! the global sentinel `0x80378020`.
//!
//! # Wave 102: the remaining band-P and home-menu bodies
//!
//! The eleventh pass fills the rest of the bucket:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x801704D0` | [`fn_801704d0`] | allocate and fill one home-menu record |
//! | `0x801707E0` | [`fn_801707e0`] | set an 8x8 cell grid and store a word |
//! | `0x80170980` | [`fn_80170980`] | normalise a vector above a threshold |
//! | `0x80170C50` | [`fn_80170c50`] | construct the dialog pane-list holder |
//! | `0x80172370` | [`fn_80172370`] | clear the event word and forward a payload |
//! | `0x80172660` | [`fn_80172660`] | store one word at `+0x2C` |
//! | `0x80172910` | [`fn_80172910`] | point-in-rectangle test for one pane |
//! | `0x801734F0` | [`fn_801734f0`] | empty widget tail |
//! | `0x80173500` | [`fn_80173500`] | store the narrowed double at `+4` |
//! | `0x80173870` | [`fn_80173870`] | identity accessor |
//! | `0x80174AE0` | [`fn_80174ae0`] | empty shutdown tail |
//! | `0x80174DE0` | [`fn_80174de0`] | four-byte thunk to `0x80191250` |
//! | `0x80175510` | [`fn_80175510`] | build the whole home-button pane tree |
//! | `0x80175F70` | [`fn_80175f70`] | first-run pane setup of the overlay |
//! | `0x80178B10` | [`fn_80178b10`] | close the active list entry |
//! | `0x8017DA20` | [`fn_8017da20`] | bind the matching pane records |
//! | `0x8017DCC0` | [`fn_8017dcc0`] | apply one brlan channel's tags |
//! | `0x8017EA10` | [`fn_8017ea10`] | write one quad to the GX FIFO |
//! | `0x8017EF70` | [`fn_8017ef70`] | scale four colours and draw the quad |
//!
//! `0x80175510` and `0x80175F70` are the two constructors the wave-26/28
//! wrappers already called; their bodies were the last large gap. The two
//! `0x8017EA10`/`0x8017EF70` bodies form the quad writer used by the dialog
//! draw pass, and `0x8017DCC0` is the `RLMC`/`RLTS`/`RLTP`/`RLIM` evaluator
//! that feeds it.
//!
//! # Wave 108: page state machines and scrolling text (`0x801769B0..0x8017BA40`)
//!
//! The twelfth pass adds the six state machines the wave-102 scan left out
//! because of their size. They share the `0x802F5B18` `(pane, action)` table
//! and the `+0x2F0` pane array, so the repeated table walks are factored into
//! [`find_menu_pair`] and [`activate_pair`].
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x801769B0` | [`fn_801769b0`] | main page state machine (states 0..0x13) |
//! | `0x80178C50` | [`fn_80178c50`] | per-player scrolling-text and battery update |
//! | `0x801795B0` | [`fn_801795b0`] | scrolling-text input and page-change handler for one player |
//! | `0x8017A120` | [`fn_8017a120`] | home-button page transition on button release |
//! | `0x8017AF70` | [`fn_8017af70`] | dialog page transition on button release |
//! | `0x8017BA40` | [`fn_8017ba40`] | option/dialog page state machine |
//!
//! `0x80178C50` walks the four per-player records passed by `0x801769B0`
//! (stride `0x10`) and calls `0x801795B0` for each player. The four page
//! machines are the handlers for the `+0x14` state word and the `+0x0C`
//! button count; they differ only in the pane slots and sound ids they use.
//!
//! # Wave 114: the last `0x80170000` body (`0x801701A0`)
//!
//! Wave 108 left `0x801701A0` untranslated because the export only showed the
//! paired-single probes as `ldexpf` calls and the text-context calls with
//! truncated argument lists. Wave 110 later translated the byte-for-byte twin
//! `0x8016FA10` (`src/resource/registry.rs`), which proves the same context
//! and record layout with the same `r2 - 0x5348` / `r2 - 0x5344` scales. Wave
//! 114 reuses that model, so the body is now reconstructible:
//!
//! | address | Rust function | role |
//! |---|---|---|
//! | `0x801701A0` | [`fn_801701a0`] | draw and age one text-run page's records |
//!
//! It differs from `0x8016FA10` in three literals only: the stream
//! constructors (`0x80230780`/`0x802307E0` instead of
//! `0x8022DDE0`/`0x8022DE40`), the payload offset (`+0x1E` instead of
//! `+0x1D`), and the run length (the `+0x06` halfword shifted right by one
//! before it is passed to `0x80231870`). With this body the `0x80170000`
//! bucket has no open address left.
//!
//! # Glossary
//!
//! * **home-button menu** - the overlay opened by the Wii Remote's Home button.
//! * **`hmMenu`** - the original class/prefix used by the menu code and its
//!   `brlyt`/`brlan` resources.
//! * **`brlyt` / `brlan`** - NW4R layout and layout-animation resource files.
//! * **`B_btn_00` / `B_btnL_00`** - button pane names used by the menu; the
//!   longer table is the left-button variant.
//! * **battery UI** - the `btryPwr_*` indicator and its low-battery dialog.
//! * **intrusive list** - a list whose nodes embed the `next` pointer at
//!   offset 0 and whose sentinel is the list head itself.
//! * **SDA** - the PowerPC small-data areas: `r13` (`0x8042F680`) is not read
//!   by this module; `r2` (`0x80433080`) holds the two defaults read by
//!   [`fn_80170980`] and [`fn_80170c50`].
//!
//! # Confidence summary
//!
//! Seven bodies are `high`. [`menu_button_index`] is `medium`: the two name
//! loops and the returned index are exact, but the function's first two
//! arguments are reconstructed from the `__savegprlr` prologue (`FUN_802dcd74`)
//! because Ghidra printed the signature as `void`. The argument roles
//! (`this + 0xB4` name buffer, second argument `+0xC` count) are observed.
//!
//! In wave 4, [`dialog_record_has_value`], [`dialog_list_insert`],
//! [`dialog_find_child_by_name`] and [`dialog_group_construct`] are `high`.
//! [`menu_confirm_pending`], [`dialog_begin_dismiss`],
//! [`dialog_panel_construct`] and [`dialog_find_named_entry`] are `medium`
//! because their state words or parser iterator layout are inferred from the
//! call sites. The two vertex-buffer helpers are `high`: the buffer pointer
//! stored at `+4` is explicit, and the heap operand of the
//! allocator/free wrappers `0x801D5550` / `0x801D5540` is the word loaded from
//! `0x80378008` (`lwz r3,0x80378008`), not the address that Ghidra printed.

use crate::boot::entry::SDA2_BASE;
use crate::platform;
use crate::ui::lyt::{
    fn_8017f390, fn_8018a5e0, fn_8018a650, fn_8018a6c0, fn_8018a6e0, p32, read_f32,
    vtable_method,
};

// ---------------------------------------------------------------------------
// Name tables and callees (called by address; definitions live outside)
// ---------------------------------------------------------------------------

/// `0x8032A3A0`: `const char*[]` of `B_btnL_00` button pane names.
///
/// observed: `ppuVar6 = &PTR_s_B_btnL_00_8032a3a0;` in `0x8017CF10`, indexed
/// once per left-button candidate.
const TABLE_B_BTNL_00: usize = 0x8032_A3A0;

/// `0x8032ACA0`: `const char*[]` of `B_btn_00` button pane names (10 entries).
const TABLE_B_BTN_00: usize = 0x8032_ACA0;

/// `0x802E1AB4`: string comparison (`strcmp`), returns `0` on equality.
///
/// observed: used by [`menu_button_index`] and [`menu_find_named_value`] with
/// a name buffer and a table entry / node name.
const FN_802E1AB4: usize = 0x802E_1AB4;

/// `0x802E19F8`: bounded string copy; the third argument is the maximum byte
/// count.
const FN_802E19F8: usize = 0x802E_19F8;

/// `0x8019F660`: attaches a message object to a dialog node.
const FN_8019F660: usize = 0x8019_F660;

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

/// Reads the big-endian signed word at `base + offset`.
#[inline]
unsafe fn read_i32(base: usize, offset: usize) -> i32 {
    platform::read_i32(base + offset)
}

/// Reads the big-endian `u16` at `base + offset`.
#[inline]
unsafe fn read_u16(base: usize, offset: usize) -> u16 {
    platform::read_u16(base + offset)
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

// ===========================================================================
// Button-name resolution
// ===========================================================================

/// Resolves the menu's inline pane name to a `B_btnL_00` / `B_btn_00` index.
///
/// The first loop walks the `B_btnL_00` table for `table[0xC]` entries and
/// remembers the matching index. The second loop always runs: it walks the ten
/// `B_btn_00` entries and, on a match, returns `index + left_count`. If neither
/// table matches, the first loop leaves `-1` (or its last index), which is
/// returned unchanged.
///
/// # Original
/// - Ghidra: `FUN_8017cf10` at `0x8017CF10` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017cf10_FUN_8017cf10.c`
/// - observed: the entry calls `FUN_802dcd74` (`__savegprlr_27`), so Ghidra
///   printed `void`; the preserved `r3`/`r4` pair is the two arguments used
///   below. The low word is `this`, the high word is the button table.
///
/// # Arguments
/// * `button_table` - first argument (`r3`); its `+0xC` word is the number of
///   `B_btnL_00` entries.
/// * `menu` - second argument (`r4`); its inline name buffer starts at
///   `menu + 0xB4`.
///
/// # Returns
/// Zero-based button index, `left_count + index` for a right-button match, or
/// the first loop's `-1` when no name matches.
///
/// # Side effects
/// Performs up to `count + 10` string comparisons; writes no target memory.
///
/// # Safety
/// `button_table + 0xC` and `menu + 0xB4` must be readable; the name tables
/// must hold `count` and ten valid string pointers.
///
/// # Confidence
/// medium - the loops and the index arithmetic are exact; the two arguments
/// are reconstructed from the register-save prologue (`r3` is the high word of
/// Ghidra's `u64`, `r4` the low word).
pub unsafe fn menu_button_index(button_table: usize, menu: usize) -> i32 {
    let name = (menu + 0xB4) as u32;
    let left_count = read_i32(button_table, 0xC);

    // observed: iVar4 = -1; while (iVar2 < iVar5 && strcmp(name, table[i])) ...
    let mut index: i32 = -1;
    let mut i = 0;
    while i < left_count {
        let entry = read_word(TABLE_B_BTNL_00, i as usize * 4);
        index = i;
        if platform::call2(FN_802E1AB4, name, entry) == 0 {
            break;
        }
        i += 1;
        index = -1;
    }

    // observed: the B_btn_00 loop always runs and can override the index.
    let mut j = 0;
    while j < 10 {
        let entry = read_word(TABLE_B_BTN_00, j as usize * 4);
        if platform::call2(FN_802E1AB4, name, entry) == 0 {
            index = j + left_count;
            break;
        }
        j += 1;
    }

    index
}

/// Returns the selected result word for dialog type `0x12`.
///
/// # Original
/// - Ghidra: `FUN_8017cfc0` at `0x8017CFC0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017cfc0_FUN_8017cfc0.c`
///
/// # Arguments
/// * `dialog` - dialog object.
///
/// # Returns
/// `dialog + 0xB8` when `dialog + 0x14 == 0x12`; otherwise `-1`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `dialog + 0x14` and `dialog + 0xB8` must be readable.
///
/// # Confidence
/// high - a type test and one load.
pub unsafe fn menu_dialog_result(dialog: usize) -> i32 {
    if read_i32(dialog, 0x14) != 0x12 {
        return -1;
    }
    read_i32(dialog, 0xB8)
}

// ===========================================================================
// Dialog data model
// ===========================================================================

/// Finds an intrusive-list node by id.
///
/// The list head is `owner + 4`; the sentinel is the head itself. Each node's
/// first word is `next` and its `+8` word is the id compared here.
///
/// # Original
/// - Ghidra: `FUN_8017dfa0` at `0x8017DFA0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017dfa0_FUN_8017dfa0.c`
///
/// # Arguments
/// * `owner` - object that owns the list; the head pointer lives at `+4`.
/// * `id` - id to find.
///
/// # Returns
/// Pointer to the matching node, or `0` when the sentinel is reached.
///
/// # Side effects
/// None.
///
/// # Safety
/// `owner + 4` must be a readable list head; every `next` pointer must be
/// either null-terminated by the sentinel or a valid node.
///
/// # Confidence
/// high - the sentinel, the `+8` id and the `next` walk are all explicit.
pub unsafe fn menu_find_list_node(owner: usize, id: u32) -> usize {
    let head = owner + 4;
    let mut node = read_word(owner, 4) as usize;

    while node != head {
        if read_word(node, 8) == id {
            return node;
        }
        node = read_word(node, 0) as usize;
    }
    0
}

/// Binds a message object to a dialog node and copies its text.
///
/// # Original
/// - Ghidra: `FUN_8017e270` at `0x8017E270` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e270_FUN_8017e270.c`
///
/// # Arguments
/// * `node` - dialog/message node.
/// * `message` - message object passed to `0x8019F660`.
/// * `text` - source text copied into `node + 0x30` (at most `0x7F` bytes).
///
/// # Returns
/// `true` when `0x8019F660(message, node + 4)` succeeds.
///
/// # Side effects
/// On success stores `message` at `node + 0x20`, copies `text` into
/// `node + 0x30` with limit `0x7F`, and clears byte `node + 0xAF`.
///
/// # Safety
/// `node + 4..node + 0xAF` must be writable; `text` must be a readable
/// NUL-terminated string for the copy.
///
/// # Confidence
/// high - the success test, the three writes and the copy limit are explicit.
pub unsafe fn menu_bind_message(node: usize, message: u32, text: u32) -> bool {
    let attached = platform::call2(FN_8019F660, message, (node + 4) as u32);
    if attached == 0 {
        return false;
    }

    write_word(node, 0x20, message);
    let _ = platform::call3(FN_802E19F8, (node + 0x30) as u32, text, 0x7F);
    write_byte(node, 0xAF, 0);
    true
}

/// Finds a named node in the dialog's named list and returns its value.
///
/// The list head is `owner + 0x28`; each node's `+8` is the name buffer passed
/// to `strcmp`, and the returned value is the node's `+0x88` word.
///
/// # Original
/// - Ghidra: `FUN_8017e310` at `0x8017E310` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e310_FUN_8017e310.c`
///
/// # Arguments
/// * `owner` - dialog object that owns the named list.
/// * `name` - NUL-terminated name to compare against each node's `+8` buffer.
///
/// # Returns
/// The matching node's `+0x88` value, or `0` when the sentinel is reached.
///
/// # Side effects
/// None beyond the string comparisons.
///
/// # Safety
/// `owner + 0x28` must be a readable list head and `name` a readable string.
///
/// # Confidence
/// high - the sentinel, the name comparison and the returned `+0x88` word are
/// all visible.
pub unsafe fn menu_find_named_value(owner: usize, name: u32) -> u32 {
    let head = owner + 0x28;
    let mut node = read_word(owner, 0x28) as usize;

    while node != head {
        if platform::call2(FN_802E1AB4, name, (node + 8) as u32) == 0 {
            return read_word(node, 0x88);
        }
        node = read_word(node, 0) as usize;
    }
    0
}

// ===========================================================================
// Event and colour helpers
// ===========================================================================

/// Tests the fixed button event record `(class = 0xFEFF, code = 8)`.
///
/// # Original
/// - Ghidra: `FUN_8017e560` at `0x8017E560` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e560_FUN_8017e560.c`
///
/// # Arguments
/// * `event` - event record; the class is at `+4`, the code at `+6`.
///
/// # Returns
/// `true` when both fields match `0xFEFF` and `8`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `event + 4..event + 8` must be readable.
///
/// # Confidence
/// high - two halfword comparisons.
pub unsafe fn menu_event_is_button(event: usize) -> bool {
    read_u16(event, 4) == 0xFEFF && read_u16(event, 6) == 8
}

/// Tests an event id and the fixed button fields in one predicate.
///
/// # Original
/// - Ghidra: `FUN_8017e590` at `0x8017E590` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e590_FUN_8017e590.c`
///
/// # Arguments
/// * `event` - event record; the id is at `+0`, class at `+4`, code at `+6`.
/// * `id` - event id to compare.
///
/// # Returns
/// `true` when `id == event[0]`, class is `0xFEFF`, and code is `8`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `event + 0..event + 8` must be readable.
///
/// # Confidence
/// high - one word and two halfword comparisons.
pub unsafe fn menu_event_matches(event: usize, id: u32) -> bool {
    id == read_word(event, 0)
        && read_u16(event, 4) == 0xFEFF
        && read_u16(event, 6) == 8
}

/// Copies a colour word and scales its alpha byte by `scale`.
///
/// # Original
/// - Ghidra: `FUN_8017e8e0` at `0x8017E8E0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e8e0_FUN_8017e8e0.c`
///
/// # Arguments
/// * `dst` - destination colour word.
/// * `src` - source colour word (big-endian `0xRRGGBBAA`).
/// * `scale` - alpha multiplier `0..=0xFF`; `0xFF` leaves the word unchanged.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies all four bytes, then (unless `scale == 0xFF`) replaces the alpha byte
/// with `(src_alpha * scale) / 0xFF`, using the original signed division and
/// sign-fixup sequence.
///
/// # Safety
/// `dst..dst+4` must be writable and `src..src+4` readable.
///
/// # Confidence
/// high - the copy and the exact signed arithmetic are visible.
pub unsafe fn menu_scale_color_alpha(dst: usize, src: usize, scale: u32) {
    write_word(dst, 0, read_word(src, 0));
    if scale == 0xFF {
        return;
    }

    // observed: param_3 = (uint)src[3] * param_3;
    //           iVar1 = param_3 / 0xff + (param_3 >> 0x1f);
    //           dst[3] = (char)iVar1 - (char)(iVar1 >> 0x1f);
    let product = (read_byte(src, 3) as u32).wrapping_mul(scale) as i32;
    let quotient = product / 0xFF + (product >> 31);
    let alpha = (quotient as u8).wrapping_sub((quotient >> 31) as u8);
    write_byte(dst, 3, alpha);
}

// ===========================================================================
// Wave 3: dialog rows and button pages (`0x80175D70..0x8017E7D0`)
// ===========================================================================

/// `0x802E1BD0`: bounded string compare (`strncmp`).
///
/// observed: returns `0` when the first `n` bytes are equal (or a NUL is
/// reached in both strings); used by [`menu_name_eq16`] and
/// [`menu_name_eq20`].
const FN_802E1BD0: usize = 0x802E_1BD0;

/// `0x80174DF0`: menu allocator (`size -> pointer`).
const FN_80174DF0: usize = 0x8017_4DF0;

/// `0x80174AE0`: per-frame reset used by [`menu_reset_player_animations`].
/// `// unknown:` the body is not translated.
const FN_80174AE0: usize = 0x8017_4AE0;

/// `0x80173620`: resets one per-player animation widget.
const FN_80173620: usize = 0x8017_3620;

/// `0x80184C40`: base constructor of the dialog-node class.
const FN_80184C40: usize = 0x8018_4C40;

/// `0x8032B080`: vtable installed by [`dialog_node_construct`].
const VT_DIALOG_NODE: u32 = 0x8032_B080;

/// `0x802F5D94`: `f32` constant used as the compact button-bar translation.
const DAT_802F5D94: usize = 0x802F_5D94;

/// `0x802F5E10`/`0x802F5E14`: lower/upper tolerance for the keyframe snap in
/// [`dialog_keyframe_lookup_u16`]. The pair is compared as
/// `low < diff < high`.
const DAT_802F5E10: usize = 0x802F_5E10;
const DAT_802F5E14: usize = 0x802F_5E14;

/// `0x80171400`: activates one pane/animation slot (`slot + 0x14 = 1`).
const FN_80171400: usize = 0x8017_1400;

/// `0x80377FE0`: one-shot flag for the eight words copied by
/// [`dialog_init_vertex_rows`].
const DAT_80377FE0: usize = 0x8037_7FE0;

/// `0x80377FE8..0x80378004`: the eight words written into every vertex row.
const DAT_80377FE8: usize = 0x8037_7FE8;

/// `0x802F5E30`/`0x802F5E34`: initial values of the vertex-row words.
const DAT_802F5E30: usize = 0x802F_5E30;
const DAT_802F5E34: usize = 0x802F_5E34;

/// Returns the number of bytes before the next `,` or NUL.
///
/// # Original
/// Host-side helper extracted from the two name scans of `0x80175D70`
/// (`work/ghidra/main-gb/export/80175d70_FUN_80175d70.c`).
///
/// # Safety
/// `text` must be a readable NUL-terminated string.
#[inline]
unsafe fn run_to_separator(text: usize) -> usize {
    let mut len = 0usize;
    loop {
        let byte = platform::read_u8(text + len);
        if byte == 0 || byte == b',' {
            return len;
        }
        len += 1;
    }
}

/// Splits a dialog text row into two names and a flag list.
///
/// The row comes from `*(resource + 0xC)`, bounded by the length at
/// `*(resource + 0x28)`. The first two comma-separated fields are copied into
/// freshly allocated strings stored at `this + 0xB0` and `this + 0xB4`. The
/// rest of the row is scanned for commas; every comma writes `1` to
/// `this + 0xA0 + n*4` when the following byte is `'1'`, otherwise `0`.
/// Finally `+0xC` receives the flag count and `+0x10` receives `count * 3`.
///
/// # Original
/// - Ghidra: `FUN_80175d70` at `0x80175D70` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175d70_FUN_80175d70.c`
///
/// # Arguments
/// * `this` - dialog row object; `+0x04` points at the resource record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates two strings through `0x80174DF0`, copies both fields, writes the
/// flag array at `+0xA0..` and the two counts at `+0xC`/`+0x10`.
///
/// # Safety
/// `this` and its resource must be live; the row must be NUL-terminated (the
/// second name scan has no explicit bound).
///
/// # Confidence
/// high - both scans, the copies and the flag arithmetic are explicit.
pub unsafe fn dialog_parse_entry(this: usize) {
    let resource = platform::read_u32(this + 4) as usize;
    let text = platform::read_u32(resource + 0x0C) as usize;
    let limit = platform::read_u32(resource + 0x28) as usize;

    let first_len = run_to_separator(text);
    let first = platform::call1(FN_80174DF0, (first_len + 1) as u32) as usize;
    platform::write_u32(this + 0xB0, first as u32);
    platform::call3(
        FN_802E19F8,
        first as u32,
        text as u32,
        first_len as u32,
    );
    platform::write_u8(first + first_len, 0);

    let second_text = text + first_len + 1;
    let second_len = run_to_separator(second_text);
    let second = platform::call1(FN_80174DF0, (second_len + 1) as u32) as usize;
    platform::write_u32(this + 0xB4, second as u32);
    platform::call3(
        FN_802E19F8,
        second as u32,
        second_text as u32,
        second_len as u32,
    );
    platform::write_u8(second + second_len, 0);

    // observed: scan the remainder for `,1` / `,0`, bounded by `text + limit`.
    let mut offset = 0usize;
    let mut flags = 0usize;
    let mut dst = this + 0xA0;
    loop {
        let cursor = second_text + offset + second_len;
        if platform::read_u8(cursor) == 0 || text + limit <= cursor {
            break;
        }
        if platform::read_u8(cursor) == b',' {
            let value = if platform::read_u8(cursor + 1) == b'1' {
                1u32
            } else {
                0u32
            };
            platform::write_u32(dst, value);
            dst += 4;
            flags += 1;
        }
        offset += 1;
    }

    platform::write_u32(this + 0x0C, flags as u32);
    platform::write_u32(this + 0x10, (flags * 3) as u32);
}

/// Records pointers to every quoted string of a UTF-16 dialog row.
///
/// The row starts at `*(resource + 8)`. Each `"` (U+0022) toggles the
/// in-quote state; on the opening quote the address just after it is stored
/// into the record array at `this + 0xBC`. Records are 0x18 bytes apart, ten
/// per row (`+0`, `+4`, ... after ten entries). The scan stops at the UTF-16
/// NUL.
///
/// # Original
/// - Ghidra: `FUN_80175ee0` at `0x80175EE0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175ee0_FUN_80175ee0.c`
///
/// # Arguments
/// * `this` - dialog row object; `+0x04` points at the resource record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes up to 40 string pointers into `this + 0xBC..` (four 10-entry rows).
///
/// # Safety
/// `this` and the UTF-16 row must be readable/writable as described.
///
/// # Confidence
/// high - the quote toggle and the record stride are explicit.
pub unsafe fn dialog_split_quoted(this: usize) {
    let resource = platform::read_u32(this + 4) as usize;
    let text = platform::read_u32(resource + 8) as usize;

    let mut index = 0usize;
    let mut slot = 0usize;
    let mut row = 0usize;
    let mut in_quote = false;

    loop {
        let unit = platform::read_u16(text + index * 2);
        if unit == 0 {
            break;
        }
        if unit == 0x22 {
            if in_quote {
                in_quote = false;
            } else {
                let offset = slot * 0x18;
                slot += 1;
                in_quote = true;
                platform::write_u32(
                    this + 0xBC + offset + row,
                    (text + (index + 1) * 2) as u32,
                );
                if slot == 10 {
                    slot = 0;
                    row += 4;
                }
            }
        }
        index += 1;
    }
}

/// Shows or hides the ten `B_btn_00` panes for menu page 0, 1 or 2.
///
/// The page is `*this` (a word). Page 0 keeps buttons `0,1,9`; page 1 keeps
/// `1..6,9`; page 2 keeps `0..6,9`. Every transition is applied through the
/// pane control object at `this + 0x250` (virtual `+0x4C` resolves the control,
/// virtual `+0x50` sets visibility). Other page values do nothing.
///
/// # Original
/// - Ghidra: `FUN_80179e60` at `0x80179E60` (700 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80179e60_FUN_80179e60.c`
///
/// # Arguments
/// * `this` - home-button menu object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Resolves ten panes by name and writes the control visibility for each.
///
/// # Safety
/// `this` must be the live menu; the layout owner at `+0x238` and the control
/// object at `+0x250` must be valid.
///
/// # Confidence
/// high - the three page masks are literal and the two virtual calls match the
/// `B_btn_00` table walk.
pub unsafe fn menu_refresh_buttons(this: usize) {
    let page = read_i32(this, 0);
    if page < 0 || page > 2 {
        return;
    }

    let layout_owner = read_word(this, 0x238) as usize;
    let layout = read_word(layout_owner, 0x10) as usize;
    let control_object = read_word(this, 0x250) as usize;

    for i in 0..10usize {
        let visible = match page {
            0 => i < 2 || i == 9,
            1 => (1..7).contains(&i) || i == 9,
            // observed: page 2 hides 0..6 and 9, so only 7 and 8 are shown.
            2 => !(i < 7 || i == 9),
            _ => false,
        };

        let name = read_word(TABLE_B_BTN_00, i * 4);
        let pane = platform::call3(
            vtable_method(layout, 0, 0x3C),
            layout as u32,
            name,
            1,
        );
        let control = platform::call2(
            vtable_method(control_object, 0, 0x4C),
            control_object as u32,
            pane,
        ) as usize;
        platform::call2(
            vtable_method(control, 0, 0x50),
            control as u32,
            u32::from(visible),
        );
    }
}

/// Switches the button bar between compact and full layout.
///
/// `enabled != 0` sets bit `0x20` of `+0x2A8`, sets `+0x8F = 1`, and uses the
/// resource translation `+0x34`/`+0x38`; otherwise both bytes are cleared and
/// the constant at `0x802F5D94` is used for both coordinates. The five
/// per-player layout owners receive `+0x44` (X) and `+0x48` (Y) when the
/// resource's `+0x20` word is zero.
///
/// # Original
/// - Ghidra: `FUN_8017cfe0` at `0x8017CFE0` (292 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017cfe0_FUN_8017cfe0.c`
///
/// # Arguments
/// * `this` - home-button menu object.
/// * `enabled` - non-zero selects the compact translation, zero the constant.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x2A8` (bit 0x20), `+0x8F`, and `+0x44`/`+0x48` of five layout
/// objects.
///
/// # Safety
/// `this`, the resource at `+0x04` and both layout owners must be live.
///
/// # Confidence
/// medium - the branch and stores are exact; "compact/full" is inferred from
/// the two coordinate sources.
pub unsafe fn menu_set_button_layout(this: usize, enabled: u32) {
    let nonzero = enabled != 0;

    let state = read_byte(this, 0x2A8);
    let state = if nonzero {
        state | 0x20
    } else {
        state & 0xDF
    };
    write_byte(this, 0x2A8, state);
    write_byte(this, 0x8F, u8::from(nonzero));

    let resource = read_word(this, 4) as usize;
    let (x, y) = if nonzero {
        (read_word(resource, 0x34), read_word(resource, 0x38))
    } else {
        let constant = read_word(DAT_802F5D94, 0);
        (constant, constant)
    };

    let primary_owner = read_word(this, 0x238) as usize;
    let primary = read_word(primary_owner, 0x10) as usize;
    write_word(primary, 0x44, x);
    write_word(primary, 0x48, y);

    if read_word(resource, 0x20) == 0 {
        for offset in [0x23Cusize, 0x240, 0x244, 0x248] {
            let owner = read_word(this, offset) as usize;
            let layout = read_word(owner, 0x10) as usize;
            write_word(layout, 0x44, x);
            write_word(layout, 0x48, y);
        }
    }
}

/// Resets the four per-player animation widgets after a menu transition.
///
/// # Original
/// - Ghidra: `FUN_8017d3e0` at `0x8017D3E0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d3e0_FUN_8017d3e0.c`
///
/// # Arguments
/// * `this` - home-button menu object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80174AE0` once and `0x80173620` for each widget at
/// `this + 0x2AC + i*4`.
///
/// # Safety
/// `this` must be the live menu; the four widgets must be valid.
///
/// # Confidence
/// high - a one-call preamble plus a fixed four-iteration loop.
pub unsafe fn menu_reset_player_animations(this: usize) {
    platform::call0(FN_80174AE0);

    for player in 0..4usize {
        platform::call1(
            FN_80173620,
            read_word(this, 0x2AC + player * 4),
        );
    }
}

/// Binary-searches a `(time, value)` key table and returns the `u16` value.
///
/// The table is a sequence of `f32 time` + `u16 value` records with an 8-byte
/// stride (`count` entries). The search mirrors the original half-open
/// `[low, high]` scheme, then snaps to the upper key when the distance to it is
/// inside `(0x802F5E10, 0x802F5E14)`.
///
/// # Original
/// - Ghidra: `FUN_8017d430` at `0x8017D430` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d430_FUN_8017d430.c`
///
/// # Arguments
/// * `value` - time to look up.
/// * `table` - first record of the key table.
/// * `count` - number of records; the original assumes `count >= 1`.
///
/// # Returns
/// The `u16` payload of the selected record.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `table` must cover `count * 8` bytes.
///
/// # Confidence
/// high - the search and snap thresholds are reproduced from the C; the
/// `u16` offsets use the same `float*` pointer arithmetic.
pub unsafe fn dialog_keyframe_lookup_u16(value: f64, table: usize, count: i32) -> u16 {
    let first = platform::read_f32(table);
    if count == 1 || value <= first as f64 {
        return platform::read_u16((table as isize + 4) as usize);
    }

    let last_time_addr = (table as isize + (count as isize * 2 - 2) * 4) as usize;
    if platform::read_f32(last_time_addr) as f64 <= value {
        let last_value_addr = (table as isize + (count as isize * 2 - 1) * 4) as usize;
        return platform::read_u16(last_value_addr);
    }

    let mut high = count - 1;
    let mut low = 0i32;

    loop {
        if low == high - 1 || low == high {
            break;
        }
        let sum = low + high;
        let mid = sum / 2;

        // observed: compare `table[mid * 2]` with the requested time.
        let mid_addr = (table as isize + (mid as isize * 2) * 4) as usize;
        if platform::read_f32(mid_addr) as f64 <= value {
            low = mid;
        } else {
            high = mid;
        }
    }

    // observed: snap to the upper key when it is very close.
    let high_addr = (table as isize + (high as isize * 2) * 4) as usize;
    let distance = (value - platform::read_f32(high_addr) as f64) as f32;
    if platform::read_f32(DAT_802F5E10) < distance && distance < platform::read_f32(DAT_802F5E14) {
        return platform::read_u16((table as isize + (high as isize * 2 + 1) * 4) as usize);
    }

    platform::read_u16((table as isize + (low as isize * 2 + 1) * 4) as usize)
}

/// Constructs a dialog node with a vtable and an embedded list head.
///
/// The base constructor `0x80184C40` runs first, then `+0x20` and `+0x24` are
/// cleared, the vtable `0x8032B080` is installed, and both `+0x28` and `+0x2C`
/// are set to the address of the embedded head at `+0x28` (an empty intrusive
/// list).
///
/// # Original
/// - Ghidra: `FUN_8017e210` at `0x8017E210` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e210_FUN_8017e210.c`
///
/// # Arguments
/// * `this` - storage for the node; at least `0x30` bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes the vtable, three zero words and two self-referential head words.
///
/// # Safety
/// `this` must be writable for `0x30` bytes and acceptable to `0x80184C40`.
///
/// # Confidence
/// medium - the stores are exact; the vtable's class name is unknown.
pub unsafe fn dialog_node_construct(this: usize) -> usize {
    // inferred: the base constructor takes `this`, although Ghidra prints no
    // arguments.
    platform::call1(FN_80184C40, this as u32);

    write_word(this, 0x20, 0);
    write_word(this, 0, VT_DIALOG_NODE);
    write_word(this, 0x24, 0);
    write_word(this, 0x28, (this + 0x28) as u32);
    write_word(this, 0x2C, (this + 0x28) as u32);

    this
}

/// Tests two names for equality over the first 16 bytes.
///
/// `0x80184C40`-style names are compared with `strncmp(..., 16) == 0`; the
/// `countLeadingZeros` trick in the original is just an "equal" test.
///
/// # Original
/// - Ghidra: `FUN_8017e500` at `0x8017E500` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e500_FUN_8017e500.c`
///
/// # Arguments
/// * `a`, `b` - NUL-terminated names.
///
/// # Returns
/// `true` when the 16-byte prefixes match.
///
/// # Side effects
/// None beyond reading both strings.
///
/// # Safety
/// Both pointers must be readable for up to 16 bytes.
///
/// # Confidence
/// high - `strncmp` plus `countLeadingZeros(x) >> 5` is an equality test.
pub unsafe fn menu_name_eq16(a: u32, b: u32) -> bool {
    platform::call3(FN_802E1BD0, a, b, 0x10) == 0
}

/// Tests two names for equality over the first 20 bytes.
///
/// # Original
/// - Ghidra: `FUN_8017e530` at `0x8017E530` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e530_FUN_8017e530.c`
///
/// # Arguments
/// * `a`, `b` - NUL-terminated names.
///
/// # Returns
/// `true` when the 20-byte prefixes match.
///
/// # Side effects
/// None beyond reading both strings.
///
/// # Safety
/// Both pointers must be readable for up to 20 bytes.
///
/// # Confidence
/// high - same shape as [`menu_name_eq16`] with length `0x14`.
pub unsafe fn menu_name_eq20(a: u32, b: u32) -> bool {
    platform::call3(FN_802E1BD0, a, b, 0x14) == 0
}

/// Fills new 32-byte vertex rows with one eight-word pattern.
///
/// The object holds a row count at `+1` and a buffer pointer at `+4`. The
/// eight pattern words are initialised once from `0x802F5E30`/`0x802F5E34`
/// (guarded by `0x80377FE0`), then every row from the old count up to `count`
/// is filled. Rows beyond the current count are not touched.
///
/// # Original
/// - Ghidra: `FUN_8017e6d0` at `0x8017E6D0` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e6d0_FUN_8017e6d0.c`
///
/// # Arguments
/// * `this` - vertex buffer owner; byte at `+0`, count at `+1`, pointer at
///   `+4`.
/// * `count` - requested row count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May initialise the eight global pattern words, writes
/// `32 * (count - old_count)` bytes and updates the count byte.
///
/// # Safety
/// The buffer must have room for `count * 32` bytes; `count` must fit in a
/// byte when stored (the original truncates).
///
/// # Confidence
/// high - the guard and unrolled eight-word store are explicit.
pub unsafe fn dialog_init_vertex_rows(this: usize, count: u32) {
    if read_word(this, 4) == 0 {
        return;
    }
    if (read_byte(this, 0) as u32) < count {
        return;
    }

    if platform::read_u8(DAT_80377FE0) == 0 {
        platform::write_u32(DAT_80377FE8, platform::read_u32(DAT_802F5E30));
        platform::write_u32(DAT_80377FE8 + 4, platform::read_u32(DAT_802F5E30));
        platform::write_u32(DAT_80377FE8 + 8, platform::read_u32(DAT_802F5E34));
        platform::write_u32(DAT_80377FE8 + 0x0C, platform::read_u32(DAT_802F5E30));
        platform::write_u32(DAT_80377FE8 + 0x10, platform::read_u32(DAT_802F5E30));
        platform::write_u32(DAT_80377FE8 + 0x14, platform::read_u32(DAT_802F5E34));
        platform::write_u32(DAT_80377FE8 + 0x18, platform::read_u32(DAT_802F5E34));
        platform::write_u32(DAT_80377FE8 + 0x1C, platform::read_u32(DAT_802F5E34));
        platform::write_u8(DAT_80377FE0, 1);
    }

    let mut words = [0u32; 8];
    for (i, word) in words.iter_mut().enumerate() {
        *word = platform::read_u32(DAT_80377FE8 + i * 4);
    }

    let old_count = read_byte(this, 1);
    let mut offset = old_count as usize * 0x20;

    if (old_count as u32) < count {
        let mut remaining = count - old_count as u32;
        loop {
            let buffer = read_word(this, 4) as usize;
            for (i, word) in words.iter().enumerate() {
                platform::write_u32(buffer + offset + i * 4, *word);
            }
            offset += 0x20;
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }

    write_byte(this, 1, count as u8);
}

/// Copies `count` 32-byte vertex rows into the buffer.
///
/// The count byte at `+1` only grows (`max(old, count)`); shorter requests do
/// not shrink it. Each row copies eight words from `src`.
///
/// # Original
/// - Ghidra: `FUN_8017e7d0` at `0x8017E7D0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e7d0_FUN_8017e7d0.c`
///
/// # Arguments
/// * `this` - vertex buffer owner (count at `+1`, pointer at `+4`).
/// * `src` - source rows, `count * 32` readable bytes.
/// * `count` - number of rows to copy.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `count * 32` bytes and raises the count byte.
///
/// # Safety
/// `src` and the destination buffer must cover `count * 32` bytes.
///
/// # Confidence
/// high - the eight-word copy loop and the grow-only count are exact.
pub unsafe fn dialog_copy_vertex_rows(this: usize, src: usize, count: u32) {
    let mut total = read_byte(this, 1);
    if (total as u32) < count {
        total = count as u8;
    }
    write_byte(this, 1, total);

    if count < 1 {
        return;
    }

    let mut offset = 0usize;
    let mut source = src;
    let mut remaining = count;

    loop {
        let buffer = read_word(this, 4) as usize;
        for i in 0..8usize {
            platform::write_u32(buffer + offset + i * 4, platform::read_u32(source + i * 4));
        }
        source += 0x20;
        offset += 0x20;
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
}

// ===========================================================================
// Wave 4: confirm, dismiss and vertex buffers (`0x8017CCA0..0x8017F940`)
// ===========================================================================

// ---------------------------------------------------------------------------
// Wave 4 addresses and callees
// ---------------------------------------------------------------------------

/// `0x80378008`: process-wide heap used by the dialog buffer allocator.
const HEAP_DAT_80378008: usize = 0x8037_8008;

/// `0x801D5540`: heap allocate `(heap, size)`.
const FN_801D5540: usize = 0x801D_5540;

/// `0x801D5550`: heap free `(heap, pointer)`.
///
/// Ghidra prints only the heap operand in the two wave-4 call sites; the
/// pointer is the value read from `this + 4` immediately before the call.
/// `// inferred:` the second operand is reconstructed from that sequence.
const FN_801D5550: usize = 0x801D_5550;

/// `0x8018A590`: insert `node` after the list head passed in `head`.
const FN_8018A590: usize = 0x8018_A590;

/// `0x8019FF00`: begin a document-node iteration `(document, name, iterator)`.
const FN_8019FF00: usize = 0x8019_FF00;

/// `0x8019FF80`: advance one document node `(iterator, record)`.
const FN_8019FF80: usize = 0x8019_FF80;

/// `0x8019FEA0`: seek the iterator to a named child/close tag.
///
/// The definition prints one parameter, but both wave-4 call sites pass
/// `(document, name)`; the second operand is preserved here.
const FN_8019FEA0: usize = 0x8019_FEA0;

/// `0x801A0040`: end the document-node iteration.
const FN_801A0040: usize = 0x801A_0040;

/// `0x802E886C`: string comparison (`strcmp`), returns `0` on equality.
const FN_802E886C: usize = 0x802E_886C;

/// `0x801738F0`: close one active home-menu list entry.
///
/// The body is translated as [`fn_801738f0`]; the address remains for the
/// earlier call site.
const FN_801738F0: usize = 0x8017_38F0;

/// `0x801AB640`: clear one timer object.
const FN_801AB640: usize = 0x801A_B640;

/// `0x801ABA70`: set the timer object's id.
const FN_801ABA70: usize = 0x801A_BA70;

/// `0x801AB540`: arm a timer `(timer, magic, 0, ticks, callback)`.
const FN_801AB540: usize = 0x801A_B540;

/// `0x801F3630`: immediate dialog/controller release query.
const FN_801F3630: usize = 0x801F_3630;

/// `0x801F38F0`: deferred dialog/controller release query.
const FN_801F38F0: usize = 0x801F_38F0;

/// `0x80178B40`: timer callback armed by [`dialog_begin_dismiss`].
const FN_80178B40: usize = 0x8017_8B40;

/// `0x800000F8`: OS bus-clock word used to convert milliseconds to ticks.
const G_OS_BUS_CLOCK: usize = 0x8000_00F8;

/// `0x802F5AB8`: `(entry index, kind)` pairs scanned by
/// [`menu_confirm_pending`] for the per-entry panes.
const TABLE_802F5AB8: usize = 0x802F_5AB8;

/// `0x802F5B18`: four-word groups scanned by [`menu_confirm_pending`] for the
/// two selection panes.
const TABLE_802F5B18: usize = 0x802F_5B18;

/// `0x802F5E28`: initial word stored at panel `+0x10` by
/// [`dialog_panel_construct`].
const W_DAT_802F5E28: usize = 0x802F_5E28;

/// `0x802F5E48`: initial word stored at group `+0x18`/`+0x1C` by
/// [`dialog_group_construct`].
const W_DAT_802F5E48: usize = 0x802F_5E48;

/// `0x8032AFF8`: vtable installed by [`dialog_panel_construct`].
const VT_DIALOG_PANEL: u32 = 0x8032_AFF8;

/// `0x8032B238`: vtable installed by [`dialog_group_construct`].
const VT_DIALOG_GROUP: u32 = 0x8032_B238;

/// `0x8032B078`: opening tag name used by [`dialog_find_named_entry`].
const S_DAT_8032B078: u32 = 0x8032_B078;

/// `0x8032B07C`: closing tag name used by [`dialog_find_named_entry`].
const S_DAT_8032B07C: u32 = 0x8032_B07C;

/// Calls a no-argument function that returns `u32`.
#[inline]
unsafe fn call0_r(addr: usize) -> u32 {
    let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
    f()
}

/// Scans the `0x802F5B18` four-word groups for `(5, mode)`.
///
/// Each iteration examines two candidate pairs: `(word0, word1)` and
/// `(word2, word3)`. The returned index is in pair units, so the second pair
/// of one group is `index + 1`.
///
/// # Original
/// Host-side factoring of the two identical scans in `0x8017CCA0`
/// (`work/ghidra/main-gb/export/8017cca0_FUN_8017cca0.c`); no binary address
/// of its own.
///
/// # Safety
/// The table must contain `0x25` readable 16-byte groups.
///
/// # Confidence
/// high - the pair order and the `0x25` bound are copied from the C.
#[inline]
unsafe fn find_dialog_group(mode: i32) -> i32 {
    let mut group = 0i32;
    let mut remaining = 0x25i32;
    let mut cursor = TABLE_802F5B18;

    loop {
        if platform::read_i32(cursor) == 5 && platform::read_i32(cursor + 4) == mode {
            return group;
        }
        if platform::read_i32(cursor + 8) == 5 && platform::read_i32(cursor + 0xC) == mode {
            return group + 1;
        }
        cursor += 0x10;
        group += 2;
        remaining -= 1;
        if remaining == 0 {
            return -1;
        }
    }
}

/// Activates the pending menu entries and selection panes.
///
/// The object holds an entry count at `+0xC`, a flag word per entry at
/// `+0x20 + i*4`, a mapping-pair count at `+0x10`, one pane array at `+0x2C0`,
/// a second pane array at `+0x2F0` and a single pane at `+0x448`.
///
/// # Original
/// - Ghidra: `FUN_8017cca0` at `0x8017CCA0` (620 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017cca0_FUN_8017cca0.c`
///
/// # Arguments
/// * `this` - home-button menu/dialog object. The body starts with
///   `__savegprlr_19` (`0x802DCD6C`); the call site `0x801769B0` passes the
///   menu pointer in `r3`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// For every pending entry, resolves `(entry, 2)` in `0x802F5AB8`, starts the
/// pane at `+0x2C0 + index*4`, writes `+0x14 = 1` on that pane and clears the
/// pending flag. Then handles the two global pending slots at `+0x20 +
/// count*4` and `+0x24 + count*4`, using `0x802F5B18` and the `+0x2F0` pane
/// array. Every pane write is one `0x80171400` call followed by `+0x14 = 1`.
///
/// # Safety
/// `this` must be the live menu; all pane words resolved from it must be live
/// layout panes. The two data tables must be readable.
///
/// # Confidence
/// medium - the loop bounds, table scans and stores are exact; the entry and
/// pane roles are inferred from the surrounding menu state.
pub unsafe fn menu_confirm_pending(this: usize) {
    let count = read_i32(this, 0x0C);

    let mut i: i32 = 0;
    while i < count {
        let flag_offset = 0x20usize + i as usize * 4;
        if read_word(this, flag_offset) != 0 {
            let pair_count = read_i32(this, 0x10);
            let mut index: i32 = -1;
            let mut k: i32 = 0;
            while k < pair_count {
                let entry = TABLE_802F5AB8 + k as usize * 8;
                if read_i32(entry, 0) == i && read_i32(entry, 4) == 2 {
                    index = k;
                    break;
                }
                k += 1;
            }

            let pane = read_word(this, (0xB0 + index) as usize * 4) as usize;
            let _ = platform::call1(FN_80171400, pane as u32);
            write_word(pane, 0x14, 1);
            write_word(this, flag_offset, 0);
        }
        i += 1;
    }

    // observed: piVar1[piVar1[3] + 8] -> +0x20 + count*4.
    let slot8_offset = 0x20usize + count as usize * 4;
    if read_word(this, slot8_offset) != 0 {
        let pane = read_word(this, 0x448) as usize;
        let _ = platform::call1(FN_80171400, pane as u32);
        write_word(pane, 0x14, 1);
        // observed: piVar1[piVar1[3] + 9] = 0.
        write_word(this, 0x24usize + count as usize * 4, 0);
    }

    // observed: the second pending slot selects one of two table modes.
    let slot9_offset = 0x24usize + count as usize * 4;
    if read_word(this, slot9_offset) != 0 {
        if read_word(this, 0) == 1 {
            if read_word(this, 0x1C) == 0 {
                let index = find_dialog_group(0x14);
                let pane = read_word(this, (0xBC + index) as usize * 4) as usize;
                let _ = platform::call1(FN_80171400, pane as u32);
                write_word(pane, 0x14, 1);
                write_word(this, slot8_offset, 0);
            }
        } else {
            let index = find_dialog_group(3);
            let pane = read_word(this, (0xBC + index) as usize * 4) as usize;
            let _ = platform::call1(FN_80171400, pane as u32);
            write_word(pane, 0x14, 1);
            write_word(this, slot8_offset, 0);
        }
    }
}

/// Runs the common `LAB_8017d274` step of the dialog dismiss path.
///
/// # Original
/// Host-side factoring of the shared block in `0x8017D1D0`
/// (`work/ghidra/main-gb/export/8017d1d0_FUN_8017d1d0.c`); no binary address
/// of its own.
///
/// # Arguments
/// * `dialog` - dialog object.
/// * `kind` - the value read from `dialog + 0x14` before the block.
///
/// # Side effects
/// Clears or re-arms the timer at `dialog + 0x5E8`, queries the two
/// dialog/controller predicates, and sets byte `+0x99`.
///
/// # Safety
/// `dialog + 0x5E8` must be a live timer object and `dialog + 0x20C` a live
/// object accepted by `0x801F3630`.
///
/// # Confidence
/// medium - the branches and timer arguments are exact; the predicate meanings
/// are not named in the export.
#[inline]
unsafe fn dialog_dismiss_step(dialog: usize, kind: i32) {
    if (kind == 3 && read_i32(dialog, 0x18) == 5)
        || (kind == 5 && read_byte(dialog, 0x91) == 0)
    {
        let _ = platform::call1(FN_801AB640, (dialog + 0x5E8) as u32);
        let _ = platform::call1(FN_801F3630, read_word(dialog, 0x20C));
    } else if read_byte(dialog, 0x92) == 0 && kind > 3 {
        write_byte(dialog, 0x98, 1);
        if call0_r(FN_801F38F0) == 0 {
            let _ = platform::call1(FN_801AB640, (dialog + 0x5E8) as u32);
            let _ = platform::call2(FN_801ABA70, (dialog + 0x5E8) as u32, 1);
            let ticks = (platform::read_u32(G_OS_BUS_CLOCK) / 4000) * 100;
            let _ = platform::call5(
                FN_801AB540,
                (dialog + 0x5E8) as u32,
                0x1062_4DD3,
                0,
                ticks,
                FN_80178B40 as u32,
            );
        }
    } else {
        let _ = platform::call1(FN_801F3630, read_word(dialog, 0x20C));
    }
    write_byte(dialog, 0x99, 1);
}

/// Dismisses/finishes a dialog and chooses its timer path.
///
/// # Original
/// - Ghidra: `FUN_8017d1d0` at `0x8017D1D0` (488 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d1d0_FUN_8017d1d0.c`
///
/// # Arguments
/// * `dialog` - home-button dialog object. `+0x14` holds the dialog kind,
///   `+0x18` a secondary kind, `+0x458` a numeric value and `+0x4` an interface
///   whose `+0x14` slot is an optional completion callback.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Returns immediately when byte `+0x9A` is already set; otherwise sets it.
/// * Resets the four widgets at `+0x2AC + i*4` through `0x801738F0` and clears
///   bytes `+0x95`, `+0x96`, `+0x98`, `+0x99`.
/// * Chooses the timer/callback branch from `+0x14`, then writes
///   `+0x14 = 0x13`, `+0x45C = 1`, `+0xB8 = 2`, stores the converted `+0x458`
///   value at `+0x79C`, and invokes the interface callback with `(3, value)`.
///
/// # Safety
/// `dialog + 0x4..+0x79C` must be readable/writable as described and the
/// timer/interface objects must be live.
///
/// # Confidence
/// medium - the full control flow and every call are observed; the kind values
/// and the completion callback are named by their role only.
pub unsafe fn dialog_begin_dismiss(dialog: usize) {
    if read_byte(dialog, 0x9A) != 0 {
        return;
    }
    write_byte(dialog, 0x9A, 1);

    for i in 0..4usize {
        let _ = platform::call1(FN_801738F0, read_word(dialog, 0x2AC + i * 4));
    }

    let kind = read_i32(dialog, 0x14);
    write_byte(dialog, 0x95, 0);
    write_byte(dialog, 0x96, 0);
    write_byte(dialog, 0x98, 0);
    write_byte(dialog, 0x99, 0);

    // observed: `(uint)(kind - 5) < 3` covers kinds 5, 6 and 7.
    if (kind as u32).wrapping_sub(5) < 3 {
        dialog_dismiss_step(dialog, kind);
    } else if kind == 0 {
        write_byte(dialog, 0x95, 1);
        write_byte(dialog, 0x96, 1);
    } else if kind == 1 {
        write_byte(dialog, 0x96, 1);
    } else if kind == 3 {
        dialog_dismiss_step(dialog, kind);
    }

    // observed: u32 -> double through the 2^52 bias at 0x802F5DC0, then a
    // float store and a double->int callback argument. `as f64` is the same
    // conversion without the host-specific bit pattern.
    let numeric = read_word(dialog, 0x458) as f64;
    write_word(dialog, 0x14, 0x13);
    write_word(dialog, 0x45C, 1);
    write_word(dialog, 0xB8, 2);
    platform::write_f32(dialog + 0x79C, numeric as f32);

    let interface = read_word(dialog, 4) as usize;
    let callback = read_word(interface, 0x14) as usize;
    if callback != 0 {
        let _ = platform::call2(callback, 3, numeric as i32 as u32);
    }
}

/// Constructs a small dialog panel object.
///
/// # Original
/// - Ghidra: `FUN_8017d700` at `0x8017D700` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d700_FUN_8017d700.c`
///
/// # Arguments
/// * `this` - storage for the panel; at least `0x1E` bytes.
///
/// # Returns
/// `this`. The original is declared `void`; the pointer is returned for
/// host-side chaining and no target value is invented.
///
/// # Side effects
/// Clears `+4..+0xC`, stores the constant `0x802F5E28` at `+0x10`, installs
/// vtable `0x8032AFF8` at `+0`, clears `+0x14`/`+0x18`, and zeroes the `u16`
/// at `+0x1C`, in the observed store order.
///
/// # Safety
/// `this` must be writable for `0x1E` bytes.
///
/// # Confidence
/// medium - every store is exact; the vtable's class name is unknown.
pub unsafe fn dialog_panel_construct(this: usize) -> usize {
    let value = read_word(W_DAT_802F5E28, 0);

    write_word(this, 4, 0);
    write_word(this, 8, 0);
    write_word(this, 0xC, 0);
    write_word(this, 0x10, value);
    write_word(this, 0, VT_DIALOG_PANEL);
    write_word(this, 0x14, 0);
    write_word(this, 0x18, 0);
    platform::write_u16(this + 0x1C, 0);

    this
}

/// Recursively finds a named entry in the parsed dialog document.
///
/// The iterator is a 16-byte record filled by `0x8019FF00` and advanced by
/// `0x8019FF80`; each advance also fills a four-word node record whose `+4`
/// word is the result, `+8` the flag that selects a recursive descent and
/// `+0xC` the node-name pointer. A matching node returns its `+4` word.
///
/// # Original
/// - Ghidra: `FUN_8017dfe0` at `0x8017DFE0` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017dfe0_FUN_8017dfe0.c`
///
/// # Arguments
/// * `document` - parsed document/context passed to the iterator functions.
/// * `name` - NUL-terminated name compared with `strcmp` (`0x802E886C`).
///
/// # Returns
/// The matching node's `+4` value, or `-1` when the iterator reaches the end.
///
/// # Side effects
/// Reads the document only; the local iterator and node records are host-side
/// stack storage. The nested calls may move the document cursor.
///
/// # Safety
/// `document` must be a live parsed document and `name` a readable string.
///
/// # Confidence
/// medium - the recursion, comparison and return value are exact; the iterator
/// layout is reconstructed from the access offsets of `0x8019FF00` and
/// `0x8019FF80` because Ghidra prints the stack frame only as `auStack_*`.
pub unsafe fn dialog_find_named_entry(document: usize, name: u32) -> i32 {
    // observed: 0x8019FF00 writes four iterator words; 0x8019FEA0 also uses
    // the word at +0x18, so the host block keeps seven words.
    let mut iterator = [0u32; 7];
    let mut node = [0u32; 4];

    let _ = platform::call3(
        FN_8019FF00,
        document as u32,
        S_DAT_8032B078,
        iterator.as_mut_ptr() as u32,
    );

    let mut pending: i32 = -1;
    let result: i32;
    loop {
        // observed: iVar3 = iVar2 at the top of every iteration; the value is
        // returned unchanged when the iterator reports the end.
        if platform::call2(
            FN_8019FF80,
            iterator.as_mut_ptr() as u32,
            node.as_mut_ptr() as u32,
        ) == 0
        {
            result = pending;
            break;
        }

        if node[2] != 0 {
            let _ = platform::call2(FN_8019FEA0, document as u32, node[3]);
            pending = dialog_find_named_entry(document, name);
            let _ = platform::call2(FN_8019FEA0, document as u32, S_DAT_8032B07C);
            if pending != -1 {
                result = pending;
                break;
            }
        } else {
            let equal = platform::call2(FN_802E886C, name, node[3]);
            pending = node[1] as i32;
            if equal == 0 {
                result = pending;
                break;
            }
        }
    }

    let _ = platform::call1(FN_801A0040, iterator.as_mut_ptr() as u32);
    result
}

/// Frees the dialog vertex-row buffer.
///
/// # Original
/// - Ghidra: `FUN_8017e5f0` at `0x8017E5F0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e5f0_FUN_8017e5f0.c`
///
/// # Arguments
/// * `this` - vertex buffer owner; byte `+0` capacity, byte `+1` row count,
///   pointer `+4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// When `+4` is non-zero, frees that pointer through `0x801D5550` with the heap
/// word at `0x80378008`, then clears the pointer and both counter bytes.
///
/// # Safety
/// `this + 4` must be null or a buffer allocated by `0x801D5540`.
///
/// # Confidence
/// high - the guard and all clears are exact; `0x8017E610..618` loads the heap
/// with `lwz r3,0x80378008`, so the first argument is the word stored there,
/// not the address.
pub unsafe fn dialog_vertex_buffer_free(this: usize) {
    let buffer = read_word(this, 4);
    if buffer != 0 {
        let heap = platform::read_u32(HEAP_DAT_80378008);
        let _ = platform::call2(FN_801D5550, heap, buffer);
        write_word(this, 4, 0);
        write_byte(this, 0, 0);
        write_byte(this, 1, 0);
    }
}

/// Grows the dialog vertex-row buffer to `count` rows of 32 bytes.
///
/// # Original
/// - Ghidra: `FUN_8017e640` at `0x8017E640` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e640_FUN_8017e640.c`
///
/// # Arguments
/// * `this` - vertex buffer owner; byte `+0` capacity, byte `+1` row count,
///   pointer `+4`.
/// * `count` - requested row count; the allocation uses `(count & 0xFF) * 32`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Only runs when the stored capacity is below `count`. Frees the old buffer,
/// clears `+4..+7` and the two counter bytes, allocates the new size through
/// `0x801D5540`, stores the pointer at `+4`, and writes the truncated capacity
/// at `+0` when allocation succeeds.
///
/// # Safety
/// `this + 4` must be null or a buffer allocated by `0x801D5540`.
///
/// # Confidence
/// high - the comparison, byte clears and allocation size are exact, and both
/// heap operands are `lwz r3,0x80378008` (`0x8017E674..678` and
/// `0x8017E690..698`), i.e. the stored word rather than the address.
pub unsafe fn dialog_vertex_buffer_reserve(this: usize, count: u32) {
    if (read_byte(this, 0) as u32) < count {
        if read_word(this, 4) != 0 {
            let heap = platform::read_u32(HEAP_DAT_80378008);
            let _ = platform::call2(FN_801D5550, heap, read_word(this, 4));
            write_byte(this, 4, 0);
            write_byte(this, 5, 0);
            write_byte(this, 6, 0);
            write_byte(this, 7, 0);
            write_byte(this, 0, 0);
            write_byte(this, 1, 0);
        }

        let allocated = platform::call2(
            FN_801D5540,
            platform::read_u32(HEAP_DAT_80378008),
            (count & 0xFF) << 5,
        );
        write_word(this, 4, allocated);
        if allocated != 0 {
            write_byte(this, 0, count as u8);
        }
    }
}

/// Tests whether a four-word record contains any value other than `-1`.
///
/// # Original
/// - Ghidra: `FUN_8017e870` at `0x8017E870` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e870_FUN_8017e870.c`
///
/// # Arguments
/// * `record` - four signed words, or `0`.
/// * `selector` - only `0xFF` triggers the record check; every other value
///   returns `1` immediately.
///
/// # Returns
/// `1` when `selector != 0xFF`, or when `record` is non-null and at least one
/// of its four words is not `-1`; `0` otherwise.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record` must be null or cover four readable words.
///
/// # Confidence
/// high - the selector test, null guard and four comparisons are literal.
pub unsafe fn dialog_record_has_value(record: usize, selector: u32) -> u32 {
    if selector != 0xFF {
        return 1;
    }
    if record != 0
        && (read_i32(record, 0) != -1
            || read_i32(record, 4) != -1
            || read_i32(record, 8) != -1
            || read_i32(record, 0xC) != -1)
    {
        return 1;
    }
    0
}

/// Inserts a node after the embedded list head of `owner`.
///
/// # Original
/// - Ghidra: `FUN_8017f6e0` at `0x8017F6E0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017f6e0_FUN_8017f6e0.c`
///
/// # Arguments
/// * `owner` - object whose list head starts at `+4`.
/// * `node` - node storage; the link fields are written at `node + 4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8018A590(owner, &head, node + 4)`, which links the node into the
/// list and increments the owner's count.
///
/// # Safety
/// `owner + 4` must be a live list head and `node + 4..+0xC` writable.
///
/// # Confidence
/// high - the local head pointer and the three arguments are explicit.
pub unsafe fn dialog_list_insert(owner: usize, node: usize) {
    let mut head = [(owner + 4) as u32];
    let _ = platform::call3(
        FN_8018A590,
        owner as u32,
        head.as_mut_ptr() as u32,
        (node + 4) as u32,
    );
}

/// Finds a child node by its 16-byte name.
///
/// # Original
/// - Ghidra: `FUN_8017f720` at `0x8017F720` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017f720_FUN_8017f720.c`
///
/// # Arguments
/// * `owner` - object whose list head starts at `+4`.
/// * `name` - NUL-terminated name compared with [`menu_name_eq16`].
///
/// # Returns
/// The node pointer (`list node - 4`), or `0` when the sentinel is reached.
///
/// # Side effects
/// None beyond the name comparisons.
///
/// # Safety
/// `owner + 4` must be a live list head and `name` readable.
///
/// # Confidence
/// high - the sentinel walk, the `+0x14` name field and the `-4` result are
/// all explicit.
pub unsafe fn dialog_find_child_by_name(owner: usize, name: u32) -> usize {
    let head = owner + 4;
    let mut node = read_word(owner, 4) as usize;

    loop {
        if node == head {
            return 0;
        }
        if menu_name_eq16((node + 0x14) as u32, name) {
            return node - 4;
        }
        node = read_word(node, 0) as usize;
    }
}

/// Constructs a dialog group with an embedded list head.
///
/// # Original
/// - Ghidra: `FUN_8017f940` at `0x8017F940` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017f940_FUN_8017f940.c`
///
/// # Arguments
/// * `this` - storage for the group; at least `0x24` bytes.
///
/// # Returns
/// `this`. The original is declared `void`; the pointer is returned for
/// host-side chaining and no target value is invented.
///
/// # Side effects
/// Clears `+4`, installs vtable `0x8032B238` at `+0`, points `+8`/`+0xC` at
/// the embedded head `this + 8`, clears `+0x10`/`+0x14`, stores
/// `0x802F5E48` at `+0x18`/`+0x1C`, and clears byte `+0x20`, in the observed
/// store order.
///
/// # Safety
/// `this` must be writable for `0x24` bytes.
///
/// # Confidence
/// high - every store and the self-referential list head are explicit.
pub unsafe fn dialog_group_construct(this: usize) -> usize {
    let value = read_word(W_DAT_802F5E48, 0);

    write_word(this, 4, 0);
    write_word(this, 0, VT_DIALOG_GROUP);
    write_word(this, 8, (this + 8) as u32);
    write_word(this, 0xC, (this + 8) as u32);
    write_word(this, 0x10, 0);
    write_word(this, 0x14, 0);
    write_word(this, 0x18, value);
    write_word(this, 0x1C, value);
    write_byte(this, 0x20, 0);

    this
}

// ===========================================================================
// Wave 7: home-menu grid/layout helpers and dialog widget toggles
// ===========================================================================

/// `0x8022_8F80`: unlink a node from an embedded NW4R list.
///
/// observed: called by [`fn_8016fe40`] and [`fn_80170110`] with the list head
/// at `this + 8`.
const FN_80228F80: usize = 0x8022_8F80;

/// `0x801D_4710`: release a resource/list entry.
///
/// observed: called by [`fn_8016fe40`] with the resource at `*this`.
const FN_801D4710: usize = 0x801D_4710;

/// `0x801D_4380`: create a resource/list entry from two arguments.
///
/// observed: return value stored at `*this` by [`fn_8016fe90`].
const FN_801D4380: usize = 0x801D_4380;

/// `0x8022_8FF0`: iterate the embedded NW4R list at `this + 8`.
///
/// observed: first call starts the walk, second call advances it.
const FN_80228FF0: usize = 0x8022_8FF0;

/// `0x8022_8DD0`: reset an embedded NW4R list with element size `0x14`.
const FN_80228DD0: usize = 0x8022_8DD0;

/// `0x8017_05E0`: free one grid/list entry; paired with [`fn_8016fe40`].
const FN_801705E0: usize = 0x8017_05E0;

/// `0x801A_C500`: flush or invalidate a range of grid bytes.
///
/// observed: called with `(buffer, count)` and with `(buffer + index, 1)`.
const FN_801AC500: usize = 0x801A_C500;

/// `0x801C_4070`: begin a dialog-pane visibility batch.
const FN_801C4070: usize = 0x801C_4070;

/// `0x801C_37C0`: enable one dialog pane by id.
const FN_801C37C0: usize = 0x801C_37C0;

/// `0x801C_40B0`: commit one dialog-pane visibility range.
const FN_801C40B0: usize = 0x801C_40B0;

/// Releases one entry of the home-menu grid.
///
/// The embedded list at `+8` is unlinked, then the resource stored at `+0` is
/// released with `value`.
///
/// # Original
/// - Ghidra: `FUN_8016fe40` at `0x8016FE40` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016fe40_FUN_8016fe40.c`
///
/// # Arguments
/// - `this`: grid/list owner.
/// - `value`: operand forwarded to the resource release call.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Unlinks a list node and releases the entry's resource.
///
/// # Safety
/// `this` must point at a valid list owner and resource pointer.
///
/// # Confidence
/// high - two calls with observed operands.
pub unsafe fn fn_8016fe40(this: usize, value: u32) {
    platform::call1(FN_80228F80, p32(this + 8));
    platform::call2(FN_801D4710, platform::read_u32(this), value);
}

/// Initialises a home-menu grid from two resource arguments.
///
/// `0x801D4380(a, b, 0)` returns the grid resource, which is stored at `+0`.
///
/// # Original
/// - Ghidra: `FUN_8016fe90` at `0x8016FE90` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016fe90_FUN_8016fe90.c`
///
/// # Arguments
/// - `this`: grid owner.
/// - `a`, `b`: resource arguments forwarded unchanged.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores the created resource at `+0`.
///
/// # Safety
/// `this` must be writable.
///
/// # Confidence
/// high - one call and one store.
pub unsafe fn fn_8016fe90(this: usize, a: u32, b: u32) {
    let resource = platform::call3(FN_801D4380, a, b, 0);
    platform::write_u32(this, resource);
}

/// Clears every entry of the home-menu grid.
///
/// When `+0` holds a resource, the embedded list at `+8` is walked and each
/// node is released through `0x801705E0`. The list is then reset with element
/// size `0x14`.
///
/// # Original
/// - Ghidra: `FUN_80170110` at `0x80170110` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170110_FUN_80170110.c`
///
/// # Arguments
/// - `this`: grid owner.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Releases every list node and resets the embedded list.
///
/// # Safety
/// `this` must be a constructed grid owner.
///
/// # Confidence
/// high - the list walk and reset are literal.
pub unsafe fn fn_80170110(this: usize) {
    if platform::read_u32(this) != 0 {
        let mut node = platform::call2(FN_80228FF0, p32(this + 8), 0);
        while node != 0 {
            let next = platform::call2(FN_80228FF0, p32(this + 8), node);
            platform::call2(FN_801705E0, p32(this), node);
            node = next;
        }
    }
    platform::call2(FN_80228DD0, p32(this + 8), 0x14);
}

/// Computes the home-menu grid dimensions and page counts.
///
/// Fields at `+0/+4/+8/+0xC` are the width, height, cell width and cell
/// height. `+0x10` is the cell area, `+0x14`/`+0x18` are the column/row
/// counts, and `+0x1C` is their product.
///
/// # Original
/// - Ghidra: `FUN_80170670` at `0x80170670` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170670_FUN_80170670.c`
///
/// # Arguments
/// - `this`: grid descriptor.
/// - `width`, `height`: grid extent in pixels.
/// - `cell_w`, `cell_h`: cell extent in pixels.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight descriptor fields. When a cell dimension is zero the matching
/// count is left untouched, exactly as in the original.
///
/// # Safety
/// `this` must be writable for `0x20` bytes.
///
/// # Confidence
/// high - the arithmetic and store order are literal.
pub unsafe fn fn_80170670(this: usize, width: u32, height: u32, cell_w: u32, cell_h: u32) {
    platform::write_u32(this, width);
    platform::write_u32(this + 4, height);
    platform::write_u32(this + 8, cell_w);
    platform::write_u32(this + 0xC, cell_h);
    platform::write_u32(this + 0x10, cell_w.wrapping_mul(cell_h));

    if cell_w != 0 {
        let columns = width.wrapping_add(cell_w).wrapping_sub(1) / cell_w;
        platform::write_u32(this + 0x14, columns);
    }

    let height_cell = platform::read_u32(this + 0xC);
    if height_cell != 0 {
        let rows = height_cell
            .wrapping_add(platform::read_u32(this + 4))
            .wrapping_sub(1)
            / height_cell;
        platform::write_u32(this + 0x18, rows);
    }

    let columns = platform::read_u32(this + 0x14);
    let rows = platform::read_u32(this + 0x18);
    platform::write_u32(this + 0x1C, columns.wrapping_mul(rows));
}

/// Returns the page number for a grid coordinate.
///
/// The result is `x / cell_w + columns * (y / cell_h)`.
///
/// # Original
/// - Ghidra: `FUN_801706e0` at `0x801706E0` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801706e0_FUN_801706e0.c`
///
/// # Arguments
/// - `this`: grid descriptor.
/// - `x`, `y`: pixel coordinate.
///
/// # Returns
/// Page index, or a division trap when a cell dimension is zero (the target
/// behaves the same way).
///
/// # Side effects
/// None.
///
/// # Safety
/// `this + 8` and `this + 0xC` must be readable and non-zero.
///
/// # Confidence
/// high - one expression.
pub unsafe fn fn_801706e0(this: usize, x: u32, y: u32) -> u32 {
    x / platform::read_u32(this + 8)
        + platform::read_u32(this + 0x14) * (y / platform::read_u32(this + 0xC))
}

/// Returns the linear cell offset within one grid page.
///
/// The result is `x % cell_w + cell_w * (y % cell_h)`.
///
/// # Original
/// - Ghidra: `FUN_80170700` at `0x80170700` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170700_FUN_80170700.c`
///
/// # Arguments
/// - `this`: grid descriptor.
/// - `x`, `y`: pixel coordinate.
///
/// # Returns
/// Cell offset inside the page.
///
/// # Side effects
/// None.
///
/// # Safety
/// `this + 8` and `this + 0xC` must be readable and non-zero.
///
/// # Confidence
/// high - the modulus is written as `n - (n / d) * d`.
pub unsafe fn fn_80170700(this: usize, x: u32, y: u32) -> u32 {
    let cell_w = platform::read_u32(this + 8);
    let cell_h = platform::read_u32(this + 0xC);
    (x - (x / cell_w) * cell_w) + cell_w * (y - (y / cell_h) * cell_h)
}

/// Resolves a grid coordinate to its global cell index.
///
/// Returns `-1` when `x >= width` or `y >= height`. Otherwise the page number
/// from [`fn_801706e0`] is multiplied by the cell area and added to the
/// in-page offset from [`fn_80170700`].
///
/// # Original
/// - Ghidra: `FUN_80170730` at `0x80170730` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170730_FUN_80170730.c`
///
/// # Arguments
/// - `this`: grid descriptor.
/// - `x`, `y`: pixel coordinate.
///
/// # Returns
/// Non-negative global cell index, or `-1` when out of bounds.
///
/// # Side effects
/// None.
///
/// # Safety
/// `this` must be a fully initialised grid descriptor.
///
/// # Confidence
/// high - bounds test and both helper calls are observed.
pub unsafe fn fn_80170730(this: usize, x: u32, y: u32) -> i32 {
    if x >= platform::read_u32(this) || y >= platform::read_u32(this + 4) {
        return -1;
    }

    let page = fn_801706e0(this, x, y);
    let offset = fn_80170700(this, x, y);
    offset
        .wrapping_add(page.wrapping_mul(platform::read_u32(this + 0x10)))
        as i32
}

/// Writes one four-bit grid cell value.
///
/// The cell index comes from [`fn_80170730`]. Two cells share one byte, so the
/// nibble is selected by `((index + 1) * 4) & 4`. The old nibble is cleared,
/// the low four bits of `value` are inserted, and an optional flush is issued
/// for the single cell.
///
/// # Original
/// - Ghidra: `FUN_80170830` at `0x80170830` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170830_FUN_80170830.c`
///
/// # Arguments
/// - `this`: grid descriptor; `+0x24` holds the cell-byte buffer.
/// - `x`, `y`: pixel coordinate.
/// - `value`: low nibble to store.
/// - `commit`: non-zero flushes the affected cell.
///
/// # Returns
/// `1` when the cell was written, `0` when the buffer is absent or the
/// coordinate is out of bounds.
///
/// # Side effects
/// May write one byte and call the grid flush helper.
///
/// # Safety
/// `this + 0x24` must be empty or point at a writable cell buffer.
///
/// # Confidence
/// high - the nibble mask and shift are literal.
pub unsafe fn fn_80170830(this: usize, x: u32, y: u32, value: u32, commit: u32) -> u32 {
    let buffer = platform::read_u32(this + 0x24) as usize;
    if buffer == 0 {
        return 0;
    }

    let index = fn_80170730(this, x, y);
    if index == -1 {
        return 0;
    }

    let byte_index = index as usize / 2;
    let shift = ((index as u32).wrapping_add(1).wrapping_mul(4)) & 4;
    let mask = !(0x0Fu8 << shift);
    let old = platform::read_u8(buffer + byte_index);
    let new = (old & mask) | (((value as u8) & 0x0F) << shift);
    platform::write_u8(buffer + byte_index, new);

    if commit != 0 {
        platform::call2(FN_801AC500, p32(buffer + index as usize), 1);
    }

    1
}

/// Flushes the whole grid cell buffer.
///
/// The byte count is `height * rows * columns * width`, shifted right once.
/// Nothing is done when `+0x24` is null.
///
/// # Original
/// - Ghidra: `FUN_80170900` at `0x80170900` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170900_FUN_80170900.c`
///
/// # Arguments
/// - `this`: grid descriptor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801AC500(buffer, count >> 1)`.
///
/// # Safety
/// `this + 0x24` must be empty or point at a readable cell buffer.
///
/// # Confidence
/// high - one guarded call and the observed product.
pub unsafe fn fn_80170900(this: usize) {
    let buffer = platform::read_i32(this + 0x24);
    if buffer == 0 {
        return;
    }

    let count = platform::read_i32(this + 0xC)
        .wrapping_mul(platform::read_i32(this + 0x18))
        .wrapping_mul(platform::read_i32(this + 0x14))
        .wrapping_mul(platform::read_i32(this + 8));
    platform::call2(FN_801AC500, buffer as u32, (count as u32) >> 1);
}

/// Stores the three dialog-mode bytes at `+0x461..+0x463`.
///
/// # Original
/// - Ghidra: `FUN_8017d3c0` at `0x8017D3C0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d3c0_FUN_8017d3c0.c`
///
/// # Arguments
/// - `this`: dialog object.
/// - `a`, `b`, `c`: bytes for `+0x461`, `+0x462`, `+0x463`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three bytes.
///
/// # Safety
/// `this + 0x461..+0x464` must be writable.
///
/// # Confidence
/// high - three direct stores.
pub unsafe fn fn_8017d3c0(this: usize, a: u8, b: u8, c: u8) {
    platform::write_u8(this + 0x461, a);
    platform::write_u8(this + 0x462, b);
    platform::write_u8(this + 0x463, c);
}

/// Stores the dialog timer at `+0x618`.
///
/// # Original
/// - Ghidra: `FUN_8017d3d0` at `0x8017D3D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d3d0_FUN_8017d3d0.c`
///
/// # Arguments
/// - `this`: dialog object.
/// - `value`: timer word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store.
///
/// # Safety
/// `this + 0x618` must be writable.
///
/// # Confidence
/// high - one direct store.
pub unsafe fn fn_8017d3d0(this: usize, value: u32) {
    platform::write_u32(this + 0x618, value);
}

/// Reads the dialog message id.
///
/// The message record is reached through `+0xC`; its `+8` halfword is the id.
///
/// # Original
/// - Ghidra: `FUN_8017d6f0` at `0x8017D6F0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d6f0_FUN_8017d6f0.c`
///
/// # Arguments
/// - `this`: dialog object.
///
/// # Returns
/// The `u16` at `*(this + 0xC) + 8`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `this + 0xC` must hold a readable record pointer.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_8017d6f0(this: usize) -> u16 {
    let record = platform::read_u32(this + 0xC) as usize;
    platform::read_u16(record + 8)
}

/// Clears the dialog vertex-row counters.
///
/// The row buffer record is `{ u8 capacity, u8 used, u16 pad, void* rows }`:
/// capacity and used are cleared, and the row pointer at `+4` is set to null.
///
/// # Original
/// - Ghidra: `FUN_8017e5d0` at `0x8017E5D0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e5d0_FUN_8017e5d0.c`
///
/// # Arguments
/// - `this`: vertex-row record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0` to `+0`, `+1` and `+4`.
///
/// # Safety
/// `this` must be writable for at least eight bytes.
///
/// # Confidence
/// high - three direct stores.
pub unsafe fn fn_8017e5d0(this: usize) {
    platform::write_u8(this, 0);
    platform::write_u8(this + 1, 0);
    platform::write_u32(this + 4, 0);
}

/// Enables the dialog pane group used by a message.
///
/// Pane `9` and pane `0xB` are enabled when a secondary object is present;
/// panes `0xD` through `0xD + count - 1` are always enabled. The same groups
/// are then committed with their visibility ranges.
///
/// # Original
/// - Ghidra: `FUN_8017e920` at `0x8017E920` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e920_FUN_8017e920.c`
///
/// # Arguments
/// - `this`: secondary dialog object; `0` skips pane `0xB`.
/// - `count`: number of consecutive pane ids starting at `0xD`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Enables and commits dialog panes through `0x801C37C0`/`0x801C40B0`.
///
/// # Safety
/// The dialog pane manager must be initialised; `count` must be non-negative.
///
/// # Confidence
/// high - both loops and the two fixed pane ids are literal.
pub unsafe fn fn_8017e920(this: usize, count: i32) {
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    if this != 0 {
        platform::call2(FN_801C37C0, 0xB, 1);
    }
    for index in 0..count {
        platform::call2(FN_801C37C0, index as u32 + 0xD, 1);
    }

    platform::call5(FN_801C40B0, 0, 9, 0, 4, 0);
    if this != 0 {
        platform::call5(FN_801C40B0, 0, 0xB, 1, 5, 0);
    }
    for index in 0..count {
        platform::call5(FN_801C40B0, 0, index as u32 + 0xD, 1, 4, 0);
    }
}

// ===========================================================================
// Wave 12: menu-item animation state and dialog spline/list helpers
// ===========================================================================

/// `0x8032_9C80`: vtable installed by [`fn_80171570`].
const VT_DIALOG_PANE_LIST: u32 = 0x8032_9C80;

/// `0x8032_B0F8`: vtable installed by [`fn_8017e380`].
const VT_DIALOG_NODE_ALT: u32 = 0x8032_B0F8;

/// `0x8017_4330`: test whether one 0x68-byte entry is enabled.
///
/// observed: `-(u8 entry) >> 31`; called by [`fn_80173860`] and returns a
/// boolean-like `0/1`.
const FN_80174330: usize = 0x8017_4330;

/// `0x801E_14E0`: close one entry of the home-menu list (`index, 0`).
const FN_801E14E0: usize = 0x801E_14E0;

/// `0x8018_36B0`: base constructor of the dialog node used by
/// [`fn_8017e380`].
const FN_801836B0: usize = 0x8018_36B0;

/// `0x802F_5E18`, `0x802F_5E1C`, `0x802F_5E20`, `0x802F_5E24`: cubic
/// interpolation coefficients read by [`fn_8017d520`].
///
/// observed: four `f32` words in the image; the export does not name the
/// curve, so they are used by address.
const DAT_802F5E18: usize = 0x802F_5E18;
const DAT_802F5E1C: usize = 0x802F_5E1C;
const DAT_802F5E20: usize = 0x802F_5E20;
const DAT_802F5E24: usize = 0x802F_5E24;

/// Configures one home-menu item animation record.
///
/// Stores the three interpolation times and the mode at `+0x18`, clears the
/// accumulator at `+0x14` and the flag at `+0x1C`, and copies the first time
/// into `+0x0C` when the mode is `1`, otherwise the second time.
///
/// # Original
/// - Ghidra: `FUN_801713c0` at `0x801713C0` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801713c0_FUN_801713c0.c`
///
/// # Arguments
/// - `a`, `b`, `c`: durations in seconds (PowerPC `f64` registers, stored as
///   `f32`).
/// - `this`: animation record; written through `+0x1C`.
/// - `mode`: `1` selects `a` for the current time, any other value selects `b`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x04`, `+0x08`, `+0x0C`, `+0x10`, `+0x14`, `+0x18` and `+0x1C`.
///
/// # Safety
/// `this` must be writable for `0x1D` bytes.
///
/// # Confidence
/// high - every store and the two-way selection are literal.
pub unsafe fn fn_801713c0(a: f64, b: f64, c: f64, this: usize, mode: i32) {
    platform::write_i32(this + 0x18, mode);
    platform::write_f32(this + 4, a as f32);
    platform::write_f32(this + 8, b as f32);
    platform::write_f32(this + 0x10, c as f32);
    platform::write_u32(this + 0x14, 0);
    platform::write_u8(this + 0x1C, 0);

    let selected = if mode == 1 { a } else { b };
    platform::write_f32(this + 0xC, selected as f32);
}

/// Constructs the dialog pane-list object.
///
/// Installs vtable `0x80329C80` and clears the two pointer/count words at
/// `+0x20` and `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_80171570` at `0x80171570` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80171570_FUN_80171570.c`
///
/// # Arguments
/// - `this`: storage for the list object; must be writable through `+0x27`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the vtable and two zero words.
///
/// # Safety
/// `this` must point at writable target memory.
///
/// # Confidence
/// high - one vtable store and two zero stores are literal.
pub unsafe fn fn_80171570(this: usize) {
    platform::write_u32(this, VT_DIALOG_PANE_LIST);
    platform::write_u32(this + 0x20, 0);
    platform::write_u32(this + 0x24, 0);
}

/// Activates or deactivates every pane in the dialog pane list.
///
/// When the mode at `+0x14` is `1`, the object's virtual slot `+0x0C` is called
/// and the current value at `+0x0C` is copied into `+0x10` of the list owner at
/// `+0x24`. Every child of the embedded list at `+0x20` is then updated through
/// virtual slot `+0x58` with `(child, owner, mode == 1, 0)`.
///
/// # Original
/// - Ghidra: `FUN_801715d0` at `0x801715D0` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801715d0_FUN_801715d0.c`
///
/// # Arguments
/// - `this`: dialog pane-list object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call the object's virtual slot `+0x0C`, writes `+0x10` of the list
/// owner, and updates every child through virtual slot `+0x58`.
///
/// # Safety
/// `this`, the vtable at `+0`, the owner at `+0x24`, and every child must be
/// live objects.
///
/// # Confidence
/// medium - the list walk and every store are literal; the virtual call in the
/// mode branch has no formal arguments in the export, so passing `this` is
/// inferred from the method shape.
pub unsafe fn fn_801715d0(this: usize) {
    let mode = read_i32(this, 0x14);
    if mode == 1 {
        // inferred: Ghidra lost the argument list for this virtual call.
        let activate = vtable_method(this, 0, 0x0C);
        platform::call1(activate, p32(this));

        let owner = read_word(this, 0x24) as usize;
        write_word(owner, 0x10, read_word(this, 0xC));
    }

    let list_owner = read_word(this, 0x20) as usize;
    let end = list_owner + 0x10;
    let mut node = platform::read_u32(end) as usize;
    while node != end {
        let child = read_word(node, 8) as usize;
        let update = vtable_method(child, 0, 0x58);
        platform::call4(
            update,
            p32(child),
            read_word(this, 0x24),
            if mode == 1 { 1 } else { 0 },
            0,
        );
        node = platform::read_u32(node) as usize;
    }
}

/// Returns bit 0 of the byte at `+0xCF`.
///
/// # Original
/// - Ghidra: `FUN_80172ce0` at `0x80172CE0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172ce0_FUN_80172ce0.c`
///
/// # Arguments
/// - `this`: dialog node.
///
/// # Returns
/// `0` or `1`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `this + 0xCF` must be readable.
///
/// # Confidence
/// high - one masked byte load.
pub unsafe fn fn_80172ce0(this: usize) -> u8 {
    platform::read_u8(this + 0xCF) & 1
}

/// Returns the child pointer at `+0x0C`.
///
/// # Original
/// - Ghidra: `FUN_80172cf0` at `0x80172CF0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172cf0_FUN_80172cf0.c`
///
/// # Arguments
/// - `this`: dialog node.
///
/// # Returns
/// The word stored at `+0x0C` (`0` when absent).
///
/// # Side effects
/// None.
///
/// # Safety
/// `this + 0x0C` must be readable.
///
/// # Confidence
/// high - one word load.
pub unsafe fn fn_80172cf0(this: usize) -> u32 {
    platform::read_u32(this + 0xC)
}

/// Tests whether one entry of the home-menu list is enabled.
///
/// Calls `0x80174330(this[8], this[0])`, which returns the boolean-like value
/// `-(u8 entry) >> 31`; the result is discarded in this wrapper.
///
/// # Original
/// - Ghidra: `FUN_80173860` at `0x80173860` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173860_FUN_80173860.c`
///
/// # Arguments
/// - `this`: list wrapper.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Reads the entry byte at `entry + index*0x68 + 0x62`; no writes.
///
/// # Safety
/// `this[8]` must point at a readable entry table.
///
/// # Confidence
/// high - one call with both observed arguments.
pub unsafe fn fn_80173860(this: usize) {
    let _ = platform::call2(
        FN_80174330,
        platform::read_u32(this + 0x20),
        platform::read_u32(this),
    );
}

/// Closes one home-menu list entry when it is active.
///
/// Ignores indices above `3` and entries whose byte at `+0x1C` is already
/// clear; otherwise clears that byte and calls `0x801E14E0(index, 0)`.
///
/// # Original
/// - Ghidra: `FUN_801738f0` at `0x801738F0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801738f0_FUN_801738f0.c`
///
/// # Arguments
/// - `this`: menu list entry.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears `+0x1C` and may close the entry through `0x801E14E0`.
///
/// # Safety
/// `this` must be readable through `+0x1C`.
///
/// # Confidence
/// high - both guards and the close call are literal.
pub unsafe fn fn_801738f0(this: usize) {
    if read_i32(this, 0) > 3 {
        return;
    }
    if read_byte(this, 0x1C) == 0 {
        return;
    }
    write_byte(this, 0x1C, 0);
    let _ = platform::call2(FN_801E14E0, read_word(this, 0), 0);
}

/// Converts one entry's enabled byte to `0` or `1`.
///
/// The original computes `-(u32)byte >> 31`, which yields `1` for every
/// non-zero byte and `0` for zero. The byte is at `base + index*0x68 + 0x62`.
///
/// # Original
/// - Ghidra: `FUN_80174330` at `0x80174330` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174330_FUN_80174330.c`
///
/// # Arguments
/// - `base`: start of the 0x68-byte entry table.
/// - `index`: entry index.
///
/// # Returns
/// `1` when the enabled byte is non-zero, otherwise `0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `base + index*0x68 + 0x62` must be readable.
///
/// # Confidence
/// high - the exact negation and shift are observed.
pub unsafe fn fn_80174330(base: usize, index: u32) -> u32 {
    let offset = index as usize * 0x68 + 0x62;
    let value = platform::read_u8(base + offset) as u32;
    value.wrapping_neg() >> 31
}

/// Evaluates one channel of a cubic keyframe curve.
///
/// This is the dialog keyframe evaluator used by the animation task:
/// `count` triples `(time, value, slope)` start at `table`. Times before the
/// first key return its value, times after the last return the last value, and
/// in-between times are found by binary search. Times within the tolerance
/// pair `0x802F5E10`/`0x802F5E14` of a key snap to that key's value (or the
/// next key's value when two adjacent keys share a time); otherwise a cubic
/// Hermite-style blend of the two neighbouring keys is returned.
///
/// # Original
/// - Ghidra: `FUN_8017d520` at `0x8017D520` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017d520_FUN_8017d520.c`
///
/// # Arguments
/// - `value`: input time in seconds.
/// - `table`: first `(time, value, slope)` triple; `count * 12` bytes.
/// - `count`: number of keyframes; must be at least `1`.
///
/// # Returns
/// The interpolated channel value as `f64` (the original returns a `double`).
///
/// # Side effects
/// None; every input is read-only.
///
/// # Safety
/// `table .. table + count*12` must be readable.
///
/// # Confidence
/// medium - the control flow, comparisons and arithmetic are literal; the
/// four image coefficients are used by address and the export contains an
/// unreachable-block warning after the closing branch.
pub unsafe fn fn_8017d520(value: f64, table: usize, count: i32) -> f64 {
    let first_time = platform::read_f32(table) as f64;
    if count == 1 || value <= first_time {
        return platform::read_f32(table + 4) as f64;
    }

    let last = count as usize * 3;
    let last_time = platform::read_f32(table + (last - 3) * 4) as f64;
    if value >= last_time {
        return platform::read_f32(table + (last - 2) * 4) as f64;
    }

    // observed: binary search with the upper bound kept as the previous high
    // on a successful lower-bound move.
    let mut low = 0u32;
    let mut high = count as u32 - 1;
    loop {
        let previous = high;
        if low == previous.wrapping_sub(1) || low == previous {
            break;
        }
        let mid = (low + high) / 2;
        let mid_time = platform::read_f32(table + mid as usize * 12) as f64;
        if mid_time < value {
            low = mid;
            high = previous;
        } else {
            high = mid;
        }
    }

    let low_key = table + low as usize * 12;
    let high_key = table + high as usize * 12;
    let delta = (value - platform::read_f32(high_key) as f64) as f32;

    if platform::read_f32(DAT_802F5E10) < delta && delta < platform::read_f32(DAT_802F5E14) {
        if high < count as u32 - 1
            && platform::read_f32(high_key) == platform::read_f32(high_key + 12)
        {
            return platform::read_f32(high_key + 16) as f64;
        }
        return platform::read_f32(high_key + 4) as f64;
    }

    let t = (value - platform::read_f32(low_key) as f64) as f32;
    let inverse_span = platform::read_f32(DAT_802F5E18)
        / ((platform::read_f32(high_key) as f64 - platform::read_f32(low_key) as f64) as f32);
    let t2 = inverse_span * t * t;
    let t3 = t * t2 * inverse_span;
    let t4 = platform::read_f32(DAT_802F5E20) * t2 * inverse_span;

    let result = platform::read_f32(high_key + 8) * (t3 - t2)
        + platform::read_f32(low_key + 8)
            * (t + (t3 - platform::read_f32(DAT_802F5E1C) * t2))
        + platform::read_f32(low_key + 4)
            * (platform::read_f32(DAT_802F5E18)
                + (platform::read_f32(DAT_802F5E1C) * t3 * inverse_span - t4))
        + platform::read_f32(high_key + 4)
            * (platform::read_f32(DAT_802F5E24) * t3 * inverse_span + t4);

    result as f64
}

/// Constructs an alternate dialog list node.
///
/// Calls the base constructor `0x801836B0` and installs vtable `0x8032B0F8`.
///
/// # Original
/// - Ghidra: `FUN_8017e380` at `0x8017E380` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e380_FUN_8017e380.c`
///
/// # Arguments
/// - `this`: storage for the node.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// The base constructor writes the embedded list fields; the vtable is then
/// replaced.
///
/// # Safety
/// `this` must be writable target memory of the size expected by
/// `0x801836B0`.
///
/// # Confidence
/// medium - the call and vtable are literal; the base constructor's size and
/// field layout are outside this module.
pub unsafe fn fn_8017e380(this: usize) -> usize {
    platform::call0(FN_801836B0);
    platform::write_u32(this, VT_DIALOG_NODE_ALT);
    this
}

// ===========================================================================
// Wave 19: menu item records and home-button menu entry points
// ===========================================================================

/// `0x801E12C0`: submit one menu-entry action `(index, argument, callback)`.
const FN_801E12C0: usize = 0x801E_12C0;

/// `0x801739B0`: continuation callback passed to `0x801E12C0`.
///
/// `unknown:` the block is not a `functions.tsv` entry; it is used verbatim as
/// the third argument of the call in [`fn_80173920`].
const CALLBACK_801739B0: u32 = 0x8017_39B0;

/// `0x80377E00`: four per-player bytes read and cleared by
/// [`fn_801739e0`]/[`fn_80173a10`].
const DAT_80377E00: usize = 0x8037_7E00;

/// `0x801ABA80`: returns the active home-button menu manager.
const FN_801ABA80: usize = 0x801A_BA80;

/// `0x801E1E20`: installs a home-button callback `(manager, slot, function)`.
const FN_801E1E20: usize = 0x801E_1E20;

/// `0x80173FB0`: callback installed in slot `1` by [`fn_80173f80`].
const FN_80173FB0: usize = 0x8017_3FB0;

/// `0x801740C0`: callback installed in slot `4` by [`fn_80174090`].
const FN_801740C0: usize = 0x8017_40C0;

/// `0x801B23F0`: resets the menu manager's widget at `+0x14680`.
const FN_801B23F0: usize = 0x801B_23F0;

/// `0x80377FA0`: global pointer to the active home-button menu manager.
const DAT_80377FA0: usize = 0x8037_7FA0;

/// `0x801769B0`: opens the home-button dialog with the menu and one argument.
const FN_801769B0: usize = 0x8017_69B0;

/// `0x80175330`: constructs the home-button menu object `(memory, argument)`.
const FN_80175330: usize = 0x8017_5330;

/// `0x80377FB8`: the single live home-button menu object.
const DAT_80377FB8: usize = 0x8037_7FB8;

/// Recomputes the current animation value of one menu item.
///
/// When the mode at `+0x18` is `1` the value at `+4` is copied into `+0x0C`;
/// every other mode copies the value at `+8`.
///
/// # Original
/// - Ghidra: `FUN_80171400` at `0x80171400` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80171400_FUN_80171400.c`
///
/// # Arguments
/// - `this`: menu-item animation record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word at `+0x0C`.
///
/// # Safety
/// `this + 0x18` must be readable and `+0x0C` writable.
///
/// # Confidence
/// high - the branch and both copies are literal.
pub unsafe fn menu_item_recompute_current(this: usize) {
    let value = if read_i32(this, 0x18) == 1 {
        read_word(this, 4)
    } else {
        read_word(this, 8)
    };
    write_word(this, 0xC, value);
}

/// Starts the action of one home-menu entry.
///
/// The entry index must be below `4`, the byte at `+0x44` must be set, and the
/// 0x68-byte record at `(*this)[8] + index*0x68` must not already be enabled.
/// The function then marks `+0x1C` and closes the entry through `0x801E14E0`.
///
/// # Original
/// - Ghidra: `FUN_80173880` at `0x80173880` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173880_FUN_80173880.c`
///
/// # Arguments
/// - `this`: home-menu entry controller; `+0` is the selected index.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the `+0x1C` flag and calls `0x801E14E0(index, 1)`.
///
/// # Safety
/// The record array word at `+0x20` must point at four valid 0x68-byte records.
///
/// # Confidence
/// medium - the guards and stores are literal; the export prints only one
/// operand on the `0x801742E0` call, so the index is supplied from `*this`.
pub unsafe fn fn_80173880(this: usize) {
    if read_word(this, 0) < 4 && read_byte(this, 0x44) != 0 {
        if fn_801742e0(read_word(this, 8) as usize, read_word(this, 0)) == 0 {
            write_byte(this, 0x1C, 1);
            platform::call2(FN_801E14E0, read_word(this, 0), 1);
        }
    }
}

/// Submits one home-menu entry action.
///
/// The index must be below `4`, the record must not be enabled, and the
/// `+0x1C` flag must be clear; in that case `0x801E12C0(index, argument,
/// callback)` is called. Every other path returns `0xFFFFFFFE`.
///
/// # Original
/// - Ghidra: `FUN_80173920` at `0x80173920` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173920_FUN_80173920.c`
///
/// # Arguments
/// - `this`: home-menu entry controller.
/// - `argument`: value forwarded to the submit call.
///
/// # Returns
/// The submit result, or `0xFFFFFFFE` when the entry cannot be submitted.
///
/// # Side effects
/// Calls `0x801E12C0`; no direct target-memory write.
///
/// # Safety
/// The record array at `+0x20` and the callback address must be valid.
///
/// # Confidence
/// high - both guards and both return constants are literal.
pub unsafe fn fn_80173920(this: usize, argument: u32) -> u32 {
    if read_word(this, 0) < 4 {
        if fn_801742e0(read_word(this, 8) as usize, read_word(this, 0)) == 0
            && read_byte(this, 0x1C) == 0
        {
            return platform::call3(
                FN_801E12C0,
                read_word(this, 0),
                argument,
                CALLBACK_801739B0,
            );
        }
    }
    0xFFFF_FFFE
}

/// Reads one of the four per-player home-menu bytes.
///
/// # Original
/// - Ghidra: `FUN_801739e0` at `0x801739E0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801739e0_FUN_801739e0.c`
///
/// # Arguments
/// - `this`: object whose first word is the player index.
///
/// # Returns
/// The byte at `0x80377E00 + index`, or `0` for an index above `3`.
///
/// # Safety
/// `this + 0` must be readable.
///
/// # Confidence
/// high - the bound and table address are literal.
pub unsafe fn fn_801739e0(this: usize) -> u8 {
    let index = read_word(this, 0);
    if index > 3 {
        return 0;
    }
    platform::read_u8(DAT_80377E00 + index as usize)
}

/// Clears one of the four per-player home-menu bytes.
///
/// # Original
/// - Ghidra: `FUN_80173a10` at `0x80173A10` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173a10_FUN_80173a10.c`
///
/// # Arguments
/// - `this`: object whose first word is the player index.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0` to `0x80377E00 + index` when the index is below `4`.
///
/// # Safety
/// `this + 0` must be readable.
///
/// # Confidence
/// high - the bound and table address are literal.
pub unsafe fn fn_80173a10(this: usize) {
    let index = read_word(this, 0);
    if index > 3 {
        return;
    }
    platform::write_u8(DAT_80377E00 + index as usize, 0);
}

/// Resets four 0x68-byte home-menu records.
///
/// Each record receives `0` at `+0x50` and `-1` at `+0x58`, then the same pair
/// at `+0xB8/+0xC0`, `+0x120/+0x128` and `+0x188/+0x190`.
///
/// # Original
/// - Ghidra: `FUN_80173ca0` at `0x80173CA0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173ca0_FUN_80173ca0.c`
///
/// # Arguments
/// - `this`: first record of the four-record group.
///
/// # Side effects
/// Writes eight words.
///
/// # Safety
/// `this + 0x190` must be writable.
///
/// # Confidence
/// high - all eight stores are literal.
pub unsafe fn fn_80173ca0(this: usize) {
    write_word(this, 0x50, 0);
    write_word(this, 0x58, 0xFFFF_FFFF);
    write_word(this, 0xB8, 0);
    write_word(this, 0xC0, 0xFFFF_FFFF);
    write_word(this, 0x120, 0);
    write_word(this, 0x128, 0xFFFF_FFFF);
    write_word(this, 0x188, 0);
    write_word(this, 0x190, 0xFFFF_FFFF);
}

/// Clears the timer object of a home-menu block and its four records.
///
/// # Original
/// - Ghidra: `FUN_80173f20` at `0x80173F20` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173f20_FUN_80173f20.c`
///
/// # Arguments
/// - `this`: first of four 0x68-byte records; the block starts at `this`.
///
/// # Side effects
/// Calls `0x801AB640` five times: once at `+0x1A0`, then at `+0`, `+0x68`,
/// `+0xD0` and `+0x138`.
///
/// # Safety
/// All five timer objects must be initialised.
///
/// # Confidence
/// high - the five calls and the loop stride are literal.
pub unsafe fn fn_80173f20(this: usize) {
    platform::call1(FN_801AB640, p32(this + 0x1A0));
    let mut record = this;
    for _ in 0..4 {
        platform::call1(FN_801AB640, p32(record));
        record += 0x68;
    }
}

/// Installs the one-argument home-button callback.
///
/// # Original
/// - Ghidra: `FUN_80173f80` at `0x80173F80` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173f80_FUN_80173f80.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Calls `0x801E1E20(manager, 1, 0x80173FB0)`.
///
/// # Safety
/// `0x801ABA80` must return a live menu manager.
///
/// # Confidence
/// high - the manager query, slot and callback are literal.
pub unsafe fn fn_80173f80() {
    let manager = call0_r(FN_801ABA80);
    platform::call3(FN_801E1E20, manager, 1, FN_80173FB0 as u32);
}

/// Installs the four-argument home-button callback.
///
/// # Original
/// - Ghidra: `FUN_80174090` at `0x80174090` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174090_FUN_80174090.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Calls `0x801E1E20(manager, 4, 0x801740C0)`.
///
/// # Safety
/// `0x801ABA80` must return a live menu manager.
///
/// # Confidence
/// high - the manager query, slot and callback are literal.
pub unsafe fn fn_80174090() {
    let manager = call0_r(FN_801ABA80);
    platform::call3(FN_801E1E20, manager, 4, FN_801740C0 as u32);
}

/// Tests whether one 0x68-byte home-menu record is enabled.
///
/// # Original
/// - Ghidra: `FUN_801742e0` at `0x801742E0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801742e0_FUN_801742e0.c`
///
/// # Arguments
/// - `base`: first record of the group.
/// - `index`: record index (`0..3` at the call sites).
///
/// # Returns
/// `1` when the word at `record + 0x50` is non-zero, otherwise `0`.
///
/// # Safety
/// `base + index*0x68 + 0x50` must be readable.
///
/// # Confidence
/// high - the arithmetic is the original `(-u | u) >> 31`.
pub unsafe fn fn_801742e0(base: usize, index: u32) -> u32 {
    let value = read_word(base + index as usize * 0x68, 0x50);
    (0u32.wrapping_sub(value) | value) >> 31
}

/// Tests one record's enabled word and matching selection id.
///
/// # Original
/// - Ghidra: `FUN_80174300` at `0x80174300` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174300_FUN_80174300.c`
///
/// # Arguments
/// - `base`: first record of the group.
/// - `index`: record index.
/// - `selector`: value compared with the record's `+0x58` word.
///
/// # Returns
/// `1` only when `+0x50` is non-zero and `+0x58 == selector`.
///
/// # Safety
/// `base + index*0x68 + 0x58` must be readable.
///
/// # Confidence
/// high - both tests and the success constant are literal.
pub unsafe fn fn_80174300(base: usize, index: u32, selector: i32) -> u32 {
    let record = base + index as usize * 0x68;
    if read_i32(record, 0x50) != 0 && read_i32(record, 0x58) == selector {
        1
    } else {
        0
    }
}

/// Notifies the active home-button menu manager.
///
/// When the global manager pointer is non-zero, the widget at `+0x14680` is
/// reset with `0x801B23F0`, and the optional callback at `+0x14348` is called.
///
/// # Original
/// - Ghidra: `FUN_801744f0` at `0x801744F0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801744f0_FUN_801744f0.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Resets one manager widget and may call a function pointer stored in the
/// manager.
///
/// # Safety
/// A non-zero `0x80377FA0` must point at a live manager with the documented
/// offsets.
///
/// # Confidence
/// high - the null guard and both calls are literal.
pub unsafe fn fn_801744f0() {
    let manager = platform::read_u32(DAT_80377FA0) as usize;
    if manager != 0 {
        platform::call3(FN_801B23F0, p32(manager + 0x14680), 1, 0);
        let callback = platform::read_u32(manager + 0x14348) as usize;
        if callback != 0 {
            platform::call0(callback);
        }
    }
}

/// Opens a dialog through the live home-button menu object.
///
/// The first call fetches the menu and passes it with `argument` to the dialog
/// constructor `0x801769B0`. The second fetch result is handed to
/// [`menu_dialog_result`]; the original left it in `r3`, so the argument is
/// reconstructed rather than printed by Ghidra.
///
/// # Original
/// - Ghidra: `FUN_80174f60` at `0x80174F60` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174f60_FUN_80174f60.c`
///
/// # Arguments
/// - `argument`: dialog payload forwarded to `0x801769B0`.
///
/// # Side effects
/// Opens a dialog and queries its result.
///
/// # Safety
/// `0x80377FB8` must hold a live home-button menu object.
///
/// # Confidence
/// medium - both menu fetches and the constructor call are literal; the final
/// result query has no printed argument, so `r3` retention is inferred.
pub unsafe fn fn_80174f60(argument: u32) {
    let menu = fn_80175500();
    platform::call2(FN_801769B0, menu, argument);
    let menu_again = fn_80175500();
    let _ = menu_dialog_result(menu_again as usize);
}

/// Applies the three dialog-mode bytes to the live menu object.
///
/// # Original
/// - Ghidra: `FUN_80175040` at `0x80175040` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175040_FUN_80175040.c`
///
/// # Arguments
/// - `a`, `b`, `c`: byte values forwarded to `0x8017D3C0`.
///
/// # Side effects
/// Writes the three mode bytes on the live menu object.
///
/// # Safety
/// `0x80377FB8` must hold a live menu object.
///
/// # Confidence
/// high - the getter and the four call operands are literal.
pub unsafe fn fn_80175040(a: u32, b: u32, c: u32) {
    let menu = fn_80175500() as usize;
    fn_8017d3c0(menu, a as u8, b as u8, c as u8);
}

/// Allocates and constructs the home-button menu object.
///
/// # Original
/// - Ghidra: `FUN_801754b0` at `0x801754B0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801754b0_FUN_801754b0.c`
///
/// # Arguments
/// - `argument`: forwarded to the menu constructor `0x80175330`.
///
/// # Side effects
/// Allocates `0x7A0` bytes and stores the constructed object in
/// `0x80377FB8` when allocation succeeds.
///
/// # Safety
/// The menu allocator must return a valid block or null.
///
/// # Confidence
/// high - the size, null guard and store are literal.
pub unsafe fn fn_801754b0(argument: u32) {
    let memory = platform::call1(FN_80174DF0, 0x7A0);
    if memory != 0 {
        let menu = platform::call2(FN_80175330, memory, argument);
        platform::write_u32(DAT_80377FB8, menu);
    }
}

/// Returns the live home-button menu object.
///
/// # Original
/// - Ghidra: `FUN_80175500` at `0x80175500` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175500_FUN_80175500.c`
///
/// # Returns
/// The word stored at `0x80377FB8` (zero before construction).
///
/// # Confidence
/// high - one global load.
pub unsafe fn fn_80175500() -> u32 {
    platform::read_u32(DAT_80377FB8)
}

// ===========================================================================
// Wave 21: player widgets, animation records and pause-menu boot
// ===========================================================================
//
// The home-button overlay owns four per-player widgets (`0x80377F88[4]`) and
// two pools of animation records (four at `+0` and three at `+0xB8B0` of the
// manager). This pass translates the widget lifecycle, the record pool
// allocator/pump, and the pause-menu boot/shutdown path.
//
// Record layout (0x2E2C bytes, observed): `+0x2E1C` active byte, `+0x2E20`
// forward link, `+0x2E24` back link, `+0x2E28` id. The C inserts at the tail
// and walks the `+0x2E20` chain from the oldest record.
//
// Widget layout (observed): `+0` player index, `+4` unscaled speed word,
// `+0x20` owning argument, `+0x24` chained input callback, `+0x28` saved
// callback, `+0x30` 64-bit press timestamp, `+0x40` connected byte,
// `+0x41` timer-armed byte, `+0x42` hold-active byte, `+0x43` hold-started
// byte and `+0x44` enabled byte.

/// `0x80377F98`: manager pointer used by the two animation-record callbacks.
const DAT_80377F98: usize = 0x8037_7F98;

/// `0x80377F88`: four-word per-player widget pointer table.
const TABLE_PLAYER_WIDGET: usize = 0x8037_7F88;

/// `0x80377E08`: four timer objects used by the chained input hook.
const TABLE_HOOK_TIMER: usize = 0x8037_7E08;

/// `0x80377EC8`: four timer objects used by the event/retrigger hook.
const TABLE_EVENT_TIMER: usize = 0x8037_7EC8;

/// `0x802F5A68`: default animation-speed word stored at widget `+4`.
const DAT_802F5A68: usize = 0x802F_5A68;

/// `0x802F5A74`: scale applied to the widget speed word before narrow.
const DAT_802F5A74: usize = 0x802F_5A74;

/// `0x802F5A78`: `"wt_HomeButtonSe_wt"` sound name.
const S_HOME_BUTTON_SE_WT: usize = 0x802F_5A78;

/// `0x802F5A8C`: `"wt_HomeButtonSe_pcm"` sound name.
const S_HOME_BUTTON_SE_PCM: usize = 0x802F_5A8C;

/// `0x8032A248`: `const char*[]` of `midi_00_home_button_mid` sound names,
/// indexed by the menu item id.
const TABLE_HOME_BUTTON_MIDI: usize = 0x8032_A248;

/// `0x80377FA8`: 0x20-byte viewport record filled by `0x801D5560`.
const DAT_80377FA8: usize = 0x8037_7FA8;

/// `0x8032A304`: global viewport pointer.
const DAT_8032A304: usize = 0x8032_A304;

/// `0x80378008`: second viewport pointer, used when no menu item needs one.
const DAT_80378008: usize = 0x8037_8008;

/// `0x80377FC0`: global passed to `0x801B2E20` at menu construction.
const DAT_80377FC0: usize = 0x8037_7FC0;

/// `0x800000F8`: OS bus clock in ticks/second.
const OS_BUS_CLOCK: usize = 0x8000_00F8;

/// `0x801E1DD0`: tests whether controller channel `index` is connected.
const FN_801E1DD0: usize = 0x801E_1DD0;

/// `0x801E1060`: binds the chained input callback and returns its handle.
const FN_801E1060: usize = 0x801E_1060;

/// `0x801E10F0`: binds a channel callback and returns its handle.
const FN_801E10F0: usize = 0x801E_10F0;

/// `0x801E5450`: selects the controller channel for the next binding call.
const FN_801E5450: usize = 0x801E_5450;

/// `0x801B1140`: enters the sound critical section, returning its state.
const FN_801B1140: usize = 0x801B_1140;

/// `0x801B1180`: leaves the sound critical section.
const FN_801B1180: usize = 0x801B_1180;

/// `0x801B5F80`: returns the current 64-bit OS time.
const FN_801B5F80: usize = 0x801B_5F80;

/// `0x801B24C0`: pumps the menu widget and writes its state to the record.
const FN_801B24C0: usize = 0x801B_24C0;

/// `0x80194760`: enables/disables one animation record.
const FN_80194760: usize = 0x8019_4760;

/// `0x801946C0`: stops one animation record.
const FN_801946C0: usize = 0x8019_46C0;

/// `0x80194630`: starts one animation record `(record, sound, a, b, 0)`.
const FN_80194630: usize = 0x8019_4630;

/// `0x801948B0`: queries whether one animation record is still running.
const FN_801948B0: usize = 0x8019_48B0;

/// `0x8019F700`: opens a sound archive entry into a stack handle.
const FN_8019F700: usize = 0x8019_F700;

/// `0x8019FE50`: reads the sound id from a stack handle.
const FN_8019FE50: usize = 0x8019_FE50;

/// `0x8019FE80`: reads the sound duration from a stack handle.
const FN_8019FE80: usize = 0x8019_FE80;

/// `0x8019FE90`: releases a stack sound handle.
const FN_8019FE90: usize = 0x8019_FE90;

/// `0x8019F9B0`: opens a sound archive entry with a caller-supplied parameter.
const FN_8019F9B0: usize = 0x8019_F9B0;

/// `0x8019F1E0`: queries whether the sound system is idle.
const FN_8019F1E0: usize = 0x8019_F1E0;

/// `0x8019F1F0`: sets the sound-system idle flag.
const FN_8019F1F0: usize = 0x8019_F1F0;

/// `0x801A0050`: resets the sound-system state.
const FN_801A0050: usize = 0x801A_0050;

/// `0x80191050`: menu sound hook A.
const FN_80191050: usize = 0x8019_1050;

/// `0x80192990`: menu sound hook B.
const FN_80192990: usize = 0x8019_2990;

/// `0x80192A70`: menu sound hook C.
const FN_80192A70: usize = 0x8019_2A70;

/// `0x80191D20`: menu sound hook D.
const FN_80191D20: usize = 0x8019_1D20;

/// `0x80194470`: menu animation stop hook A.
const FN_80194470: usize = 0x8019_4470;

/// `0x801944A0`: menu animation stop hook B.
const FN_801944A0: usize = 0x8019_44A0;

/// `0x801AB2E0`: initialises one timer object.
const FN_801AB2E0: usize = 0x801A_B2E0;

/// `0x801B2390`: binds a timer group `(object, argument, count)`.
const FN_801B2390: usize = 0x801B_2390;

/// `0x801B5130`: registers the repeating animation-record pump.
const FN_801B5130: usize = 0x801B_5130;

/// `0x801B5860`: starts the registered pump.
const FN_801B5860: usize = 0x801B_5860;

/// `0x801A1FD0`: registers a menu callback and returns its handle.
const FN_801A1FD0: usize = 0x801A_1FD0;

/// `0x801D5560`: fills the viewport record. The lookup helper `0x801D4380` is
/// the constant of the same name declared earlier in the file.
const FN_801D5560: usize = 0x801D_5560;

/// `0x801B2E20`: initialises the global menu widget.
const FN_801B2E20: usize = 0x801B_2E20;

/// `0x80175510`: constructs the large menu viewport body.
const FN_80175510: usize = 0x8017_5510;

/// `0x801744F0`: callback registered by the pause-menu boot path.
const FN_801744F0: usize = 0x8017_44F0;

/// `0x80174560`: pump created by [`fn_801748e0`].
const FN_80174560: usize = 0x8017_4560;

/// Handles one per-player input-hook event.
///
/// On event `0` the widget is marked connected and the chained hook is
/// installed if this is the first connection. On event `-1` the saved callback
/// is restored and the three state bytes are cleared. Every path then forwards
/// to the widget's `+0x24` callback unless it is null or this function itself.
///
/// # Original
/// - Ghidra: `FUN_80172ea0` at `0x80172EA0` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172ea0_FUN_80172ea0.c`
///
/// # Arguments
/// - `player`: widget index (`0..3`).
/// - `event`: `0` on connect, `-1` on disconnect.
///
/// # Side effects
/// Writes widget state bytes `+0x40`, `+0x42`, `+0x43` and calls the channel
/// binding helpers.
///
/// # Safety
/// `0x80377F88[player]` must point at a live widget.
///
/// # Confidence
/// high - the two events and the chained-callback guard are literal.
pub unsafe fn fn_80172ea0(player: u32, event: i32) {
    if event == 0 {
        let widget = platform::read_u32(TABLE_PLAYER_WIDGET + player as usize * 4) as usize;
        if platform::read_u8(widget + 0x40) == 0 {
            platform::call2(FN_801E10F0, player, FN_80172FB0 as u32);
            platform::write_u8(widget + 0x40, 1);
        }
        platform::call3(FN_801E1E20, player, 0, 0);
    } else if event == -1 {
        let widget = platform::read_u32(TABLE_PLAYER_WIDGET + player as usize * 4) as usize;
        platform::call2(FN_801E10F0, player, platform::read_u32(widget + 0x28));
        platform::write_u8(widget + 0x40, 0);
        platform::write_u8(widget + 0x42, 0);
        platform::write_u8(widget + 0x43, 0);
    }

    let widget = platform::read_u32(TABLE_PLAYER_WIDGET + player as usize * 4) as usize;
    let chained = platform::read_u32(widget + 0x24) as usize;
    if chained != 0 && chained != FN_80172EA0 {
        platform::call2(chained, player, event as u32);
    }
}

/// `0x80172EA0`: address of [`fn_80172ea0`], used for the self-callback guard.
const FN_80172EA0: usize = 0x8017_2EA0;

/// Handles the per-player event hook and retriggers the hold timer.
///
/// Event `0xFF` starts the "press" animation on the widget's channel, re-arms
/// the shared timer for 1000 ms and sets the timer-armed byte. The saved
/// callback at `+0x28` is then always forwarded.
///
/// # Original
/// - Ghidra: `FUN_80172fb0` at `0x80172FB0` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172fb0_FUN_80172fb0.c`
///
/// # Arguments
/// - `player`: widget index (`0..3`).
/// - `event`: event code; `0xFF` is the press/retrigger event.
///
/// # Side effects
/// Starts an animation, arms a timer and writes state byte `+0x41`.
///
/// # Safety
/// `0x80377F88[player]` must point at a live widget.
///
/// # Confidence
/// high - the event value, timer arguments and callback are literal.
pub unsafe fn fn_80172fb0(player: u32, event: u32) {
    if event == 0xFF {
        let widget = platform::read_u32(TABLE_PLAYER_WIDGET + player as usize * 4) as usize;
        let channel = platform::read_u32(widget);
        if platform::call1(FN_801E1DD0, channel) != 0 {
            platform::call3(FN_801E1E20, channel, 2, 0);
            let timer = TABLE_EVENT_TIMER + channel as usize * 0x30;
            platform::call2(FN_801ABA70, p32(timer), channel);
            platform::call1(FN_801AB640, p32(timer));
            platform::call5(
                FN_801AB540,
                p32(timer),
                0x1062_4DD3,
                0,
                (platform::read_u32(OS_BUS_CLOCK) / 4000) * 1000,
                FN_801730C0 as u32,
            );
            platform::write_u8(widget + 0x41, 1);
        }
    }

    let widget = platform::read_u32(TABLE_PLAYER_WIDGET + player as usize * 4) as usize;
    let saved = platform::read_u32(widget + 0x28) as usize;
    if saved != 0 {
        platform::call2(saved, player, event);
    }
}

/// `0x80172FB0`: address of [`fn_80172fb0`], installed as a channel callback.
const FN_80172FB0: usize = 0x8017_2FB0;

/// Stops the press animation when the per-player timer expires.
///
/// The active channel is read from `0x801ABA80`; its widget's `+0x41` byte is
/// cleared after a `3` notification is sent to a connected channel.
///
/// # Original
/// - Ghidra: `FUN_801730c0` at `0x801730C0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801730c0_FUN_801730c0.c`
///
/// # Side effects
/// Sends one channel notification and clears widget byte `+0x41`.
///
/// # Safety
/// The active widget pointer must be live.
///
/// # Confidence
/// high - the channel query, notification and store are literal.
pub unsafe fn fn_801730c0() {
    let player = call0_r(FN_801ABA80);
    let widget = platform::read_u32(TABLE_PLAYER_WIDGET + player as usize * 4) as usize;
    let channel = platform::read_u32(widget);
    if platform::call1(FN_801E1DD0, channel) != 0 {
        platform::call3(FN_801E1E20, channel, 3, 0);
    }
    platform::write_u8(widget + 0x41, 0);
}

/// `0x801730C0`: address of [`fn_801730c0`], installed as the timer callback.
const FN_801730C0: usize = 0x8017_30C0;

/// Constructs and registers one per-player widget.
///
/// The widget receives its player index, the default speed word `0x802F5A68`,
/// the owning argument at `+0x20`, zeroed handles and callbacks, and the
/// enabled byte `1` at `+0x44`. For players below `4` the per-player byte
/// table is cleared, the two timers are initialised and the widget is published
/// in `0x80377F88[player]`.
///
/// # Original
/// - Ghidra: `FUN_80173130` at `0x80173130` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173130_FUN_80173130.c`
///
/// # Arguments
/// - `this`: writable widget storage.
/// - `player`: player index stored at `+0`.
/// - `owner`: owning object stored at `+0x20`.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be writable and large enough for the widget fields up to
/// `+0x44`; `player` below `4` makes the global tables writable.
///
/// # Confidence
/// high - every store and both timer calls are literal.
pub unsafe fn fn_80173130(this: usize, player: u32, owner: u32) -> usize {
    platform::write_u32(this, player);
    platform::write_u8(this + 0x1C, 0);
    platform::write_u32(this + 4, platform::read_u32(DAT_802F5A68));
    platform::write_u32(this + 0x20, owner);
    platform::write_u32(this + 0x24, 0);
    platform::write_u32(this + 0x28, 0);
    platform::write_u8(this + 0x40, 0);
    platform::write_u8(this + 0x41, 0);
    platform::write_u8(this + 0x44, 1);

    if player < 4 {
        platform::write_u32(DAT_80377E00 + player as usize * 4, 0);
        platform::call1(FN_801AB2E0, p32(TABLE_HOOK_TIMER + player as usize * 0x30));
        platform::call1(FN_801AB2E0, p32(TABLE_EVENT_TIMER + player as usize * 0x30));
        platform::write_u32(TABLE_PLAYER_WIDGET + player as usize * 4, p32(this));
    }
    this
}

/// Uninstalls one per-player widget's channel hooks.
///
/// # Original
/// - Ghidra: `FUN_80173290` at `0x80173290` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173290_FUN_80173290.c`
///
/// # Arguments
/// - `this`: widget pointer; `+0` is the player index.
///
/// # Side effects
/// Clears the two channel bindings and zeroes the two handle slots.
///
/// # Safety
/// `this` must be a live widget created by [`fn_80173130`].
///
/// # Confidence
/// high - the channel selection and both saved handles are literal.
pub unsafe fn fn_80173290(this: usize) {
    let player = platform::read_u32(this);
    platform::call3(FN_801E1E20, player, 1, 0);
    platform::call1(FN_801E5450, 0);
    platform::call2(FN_801E1060, player, platform::read_u32(this + 0x24));
    platform::call1(FN_801E5450, 1);
    platform::write_u32(this + 0x24, 0);
    platform::call2(FN_801E10F0, player, platform::read_u32(this + 0x28));
    platform::write_u32(this + 0x28, 0);
}

/// Copies one event colour record into a widget.
///
/// When `copy_position` is non-zero and the event carries type `2` with a
/// matching selection record, the two position words are copied from the
/// caller's record; otherwise they come from the event's `+0x20/+0x24`. The
/// three colour words are always copied from the event, and the four flag words
/// at `+0x60..+0x68` are folded into the colour bits.
///
/// # Original
/// - Ghidra: `FUN_80173310` at `0x80173310` (388 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173310_FUN_80173310.c`
///
/// # Arguments
/// - `this`: destination widget/colour record.
/// - `source`: caller record; `+0` is the event object.
/// - `copy_position`: non-zero to take the caller's two position words.
///
/// # Side effects
/// Writes `this + 0x08..+0x18`.
///
/// # Safety
/// Both records and the event object must be readable; `this` writable.
///
/// # Confidence
/// high - the two paths, all three colour words and the twelve bit tests are
/// literal.
pub unsafe fn fn_80173310(this: usize, source: usize, copy_position: u32) {
    let event = platform::read_u32(source) as usize;
    if event == 0 {
        return;
    }

    if copy_position != 0 {
        if platform::read_u8(event + 0x5C) == 2 && platform::read_i32(source + 0x0C) == 2 {
            platform::write_u32(this + 0x08, platform::read_u32(source + 0x04));
            platform::write_u32(this + 0x0C, platform::read_u32(source + 0x08));
        } else {
            platform::write_u32(this + 0x08, platform::read_u32(event + 0x20));
            platform::write_u32(this + 0x0C, platform::read_u32(event + 0x24));
        }
    }

    platform::write_u32(this + 0x10, platform::read_u32(event + 0x04));
    let base_colour = platform::read_u32(event);
    platform::write_u32(this + 0x14, base_colour);
    platform::write_u32(this + 0x18, platform::read_u32(event + 0x08));

    if platform::read_u8(event + 0x5C) != 2 {
        return;
    }
    if platform::read_i32(source + 0x0C) != 2 {
        return;
    }

    let flags_a = platform::read_u32(event + 0x60);
    let flags_b = platform::read_u32(event + 0x64);
    let flags_c = platform::read_u32(event + 0x68);

    if flags_a & 0x10 != 0 {
        platform::write_u32(this + 0x14, base_colour | 0x800);
    }
    if flags_b & 0x10 != 0 {
        platform::write_u32(this + 0x10, platform::read_u32(this + 0x10) | 0x800);
    }
    if flags_c & 0x10 != 0 {
        platform::write_u32(this + 0x18, platform::read_u32(this + 0x18) | 0x800);
    }
    if flags_a & 0x400 != 0 {
        platform::write_u32(this + 0x14, platform::read_u32(this + 0x14) | 0x10);
    }
    if flags_b & 0x400 != 0 {
        platform::write_u32(this + 0x10, platform::read_u32(this + 0x10) | 0x10);
    }
    if flags_c & 0x400 != 0 {
        platform::write_u32(this + 0x18, platform::read_u32(this + 0x18) | 0x10);
    }
    if flags_a & 0x1000 != 0 {
        platform::write_u32(this + 0x14, platform::read_u32(this + 0x14) | 0x1000);
    }
    if flags_b & 0x1000 != 0 {
        platform::write_u32(this + 0x10, platform::read_u32(this + 0x10) | 0x1000);
    }
    if flags_c & 0x1000 != 0 {
        platform::write_u32(this + 0x18, platform::read_u32(this + 0x18) | 0x1000);
    }
    if flags_a & 0x800 != 0 {
        platform::write_u32(this + 0x14, platform::read_u32(this + 0x14) | 0x8000);
    }
    if flags_b & 0x800 != 0 {
        platform::write_u32(this + 0x10, platform::read_u32(this + 0x10) | 0x8000);
    }
    if flags_c & 0x800 != 0 {
        platform::write_u32(this + 0x18, platform::read_u32(this + 0x18) | 0x8000);
    }
}

/// Per-frame update of one player widget.
///
/// When the press timer is not armed, the scaled speed word is written through
/// [`fn_80174240`]. A connected channel starts the hold timestamp on the first
/// frame and marks the hold active.
///
/// # Original
/// - Ghidra: `FUN_80173510` at `0x80173510` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173510_FUN_80173510.c`
///
/// # Arguments
/// - `this`: widget pointer.
/// - `value`: item/sound value forwarded to [`fn_80174240`].
///
/// # Side effects
/// Writes the animation-record slot, the `+0x30` timestamp and the two hold
/// bytes.
///
/// # Safety
/// `this` must be a live widget; `+0x20` must be a valid record base.
///
/// # Confidence
/// high - the narrow cast and all stores are literal.
pub unsafe fn fn_80173510(this: usize, value: u32) {
    if platform::read_u8(this + 0x41) == 0 {
        let scaled = (platform::read_f32(DAT_802F5A74) * platform::read_i32(this + 4) as f32)
            as i32;
        fn_80174240(
            platform::read_u32(this + 0x20) as usize,
            platform::read_u32(this),
            value,
            scaled as i8 as u8,
        );

        let channel = platform::read_u32(this);
        if platform::call1(FN_801E1DD0, channel) != 0 {
            if platform::read_u8(this + 0x42) == 0 {
                let stamp = platform::call_u64(FN_801B5F80);
                platform::write_u32(this + 0x30, (stamp >> 32) as u32);
                platform::write_u32(this + 0x34, stamp as u32);
            }
            platform::write_u8(this + 0x42, 1);
            platform::write_u8(this + 0x43, 0);
        }
    }
}

/// Reports whether one widget's animation record is enabled for a selection.
///
/// # Original
/// - Ghidra: `FUN_801735b0` at `0x801735B0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801735b0_FUN_801735b0.c`
///
/// # Arguments
/// - `this`: widget pointer.
/// - `selector`: value compared with the record's `+0x58`.
///
/// # Returns
/// `0` when the record is disabled, otherwise `1` when the enabled word and
/// `selector` match (through [`fn_80174300`]).
///
/// # Safety
/// `this + 0x20` must point at the four-record group.
///
/// # Confidence
/// high - both calls are the already-translated record predicates.
pub unsafe fn fn_801735b0(this: usize, selector: u32) -> u32 {
    let records = platform::read_u32(this + 0x20) as usize;
    let index = platform::read_u32(this);
    if fn_801742e0(records, index) == 0 {
        0
    } else {
        fn_80174300(records, index, selector as i32)
    }
}

/// State callback installed in slot `1`: enters the four-argument handler.
///
/// Event `0` marks record `+0x5C` and installs [`fn_801740c0`] in slot `4`.
/// Event `-3` clears `+0x62`. Event `-2` resets the record timer and re-arms it
/// for 50 ms with [`fn_80173f80`].
///
/// # Original
/// - Ghidra: `FUN_80173fb0` at `0x80173FB0` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173fb0_FUN_80173fb0.c`
///
/// # Arguments
/// - `index`: 0x68-byte record index.
/// - `event`: `0`, `-3` or `-2`.
///
/// # Side effects
/// Writes record bytes and arms the record's embedded timer.
///
/// # Safety
/// A non-zero `0x80377F98` must be the live 4-record manager.
///
/// # Confidence
/// high - the three event arms and the timer arguments are literal.
pub unsafe fn fn_80173fb0(index: u32, event: i32) {
    let manager = platform::read_u32(DAT_80377F98) as usize;
    if manager != 0 {
        if event == 0 {
            let record = manager + index as usize * 0x68;
            platform::write_u8(record + 0x5C, 1);
            platform::call3(FN_801E1E20, index, 4, FN_801740C0 as u32);
        } else if event == -3 {
            let record = manager + index as usize * 0x68;
            platform::write_u8(record + 0x62, 0);
        } else if event == -2 {
            let record = manager + index as usize * 0x68;
            platform::call2(FN_801ABA70, p32(record), index);
            platform::call1(FN_801AB640, p32(record));
            platform::call5(
                FN_801AB540,
                p32(record),
                0x1062_4DD3,
                0,
                (platform::read_u32(OS_BUS_CLOCK) / 4000) * 0x32,
                FN_80173F80 as u32,
            );
        }
    }
}

/// `0x80173F80`: callback installed by [`fn_80173fb0`].
const FN_80173F80: usize = 0x8017_3F80;

/// `0x80174090`: callback installed by [`fn_801740c0`].
const FN_80174090: usize = 0x8017_4090;

/// State callback installed in slot `4`: marks the record's `+0x62` byte.
///
/// Event `0` sets `+0x62`; `-3` and `-1` clear it; `-2` resets the record timer
/// and re-arms it for 50 ms with [`fn_80174090`].
///
/// # Original
/// - Ghidra: `FUN_801740c0` at `0x801740C0` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801740c0_FUN_801740c0.c`
///
/// # Arguments
/// - `index`: 0x68-byte record index.
/// - `event`: `0`, `-1`, `-3` or `-2`.
///
/// # Side effects
/// Writes record byte `+0x62` and arms the record's embedded timer.
///
/// # Safety
/// A non-zero `0x80377F98` must be the live 4-record manager.
///
/// # Confidence
/// high - all four event arms and the timer arguments are literal.
pub unsafe fn fn_801740c0(index: u32, event: i32) {
    let manager = platform::read_u32(DAT_80377F98) as usize;
    if manager != 0 {
        if event == 0 {
            let record = manager + index as usize * 0x68;
            platform::write_u8(record + 0x62, 1);
        } else if event == -3 {
            let record = manager + index as usize * 0x68;
            platform::write_u8(record + 0x62, 0);
        } else if event == -1 {
            let record = manager + index as usize * 0x68;
            platform::write_u8(record + 0x62, 0);
        } else if event == -2 {
            let record = manager + index as usize * 0x68;
            platform::call2(FN_801ABA70, p32(record), index);
            platform::call1(FN_801AB640, p32(record));
            platform::call5(
                FN_801AB540,
                p32(record),
                0x1062_4DD3,
                0,
                (platform::read_u32(OS_BUS_CLOCK) / 4000) * 0x32,
                FN_80174090 as u32,
            );
        }
    }
}

/// Resets one 0x68-byte record and installs its state callback.
///
/// When the manager's `+0x1EC` byte is set, [`fn_80173fb0`] is installed in
/// slot `1`, the record's timer words `+0x30..+0x4C` are cleared, and the two
/// state bytes are set to `1`/`0`.
///
/// # Original
/// - Ghidra: `FUN_801741b0` at `0x801741B0` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801741b0_FUN_801741b0.c`
///
/// # Arguments
/// - `this`: manager pointer.
/// - `index`: record index.
///
/// # Side effects
/// Writes eight timer words and two state bytes on the record.
///
/// # Safety
/// `this + 0x1EC` must be readable and the record writable.
///
/// # Confidence
/// high - the guard, the eight stores and both bytes are literal.
pub unsafe fn fn_801741b0(this: usize, index: u32) {
    if platform::read_u8(this + 0x1EC) != 0 {
        platform::call3(FN_801E1E20, index, 1, FN_80173FB0 as u32);
        let record = this + index as usize * 0x68;
        platform::write_u32(record + 0x30, 0);
        platform::write_u32(record + 0x34, 0);
        platform::write_u32(record + 0x38, 0);
        platform::write_u32(record + 0x3C, 0);
        platform::write_u32(record + 0x40, 0);
        platform::write_u32(record + 0x44, 0);
        platform::write_u32(record + 0x48, 0);
        platform::write_u32(record + 0x4C, 0);
        platform::write_u8(record + 0x5C, 1);
        platform::write_u8(record + 0x62, 0);
    }
}

/// Writes one animation-record slot under the manager lock.
///
/// The manager is the first argument and the record index the second; when
/// `+0x1EC` is set, the sound handle is opened, its id and duration are read,
/// and the record's `+0x50..+0x5E` fields are written.
///
/// # Original
/// - Ghidra: `FUN_80174240` at `0x80174240` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174240_FUN_80174240.c`
/// - Removed: the `bl 0x802DCD70` (`__savegprlr_26`) prologue and the
///   `bl 0x802DCDBC` (`__restgprlr_26`) epilogue; the prologue only saved the
///   incoming `r3`/`r4`, now the `record_group`/`player` parameters.
///
/// # Arguments
/// - `record_group`: manager object (incoming `r3`).
/// - `player`: record index (incoming `r4`).
/// - `value`: sound/item value passed to `0x8019F9B0` and stored at `+0x58`.
/// - `mode`: byte stored at record `+0x5D`.
///
/// # Side effects
/// Writes five record fields.
///
/// # Safety
/// `record_group` must be a live manager and the sound stack handle must be
/// valid.
///
/// # Confidence
/// medium - the body is literal; the manager/index split is the incoming
/// `r3`/`r4` parameter pair.
pub unsafe fn fn_80174240(record_group: usize, player: u32, value: u32, mode: u8) {
    let manager = record_group;
    let index = player;

    if platform::read_u8(manager + 0x1EC) != 0 {
        let mut handle = [0u32; 10];
        platform::call3(
            FN_8019F9B0,
            p32(manager + 0x1D0),
            value,
            p32(&mut handle as *mut [u32; 10] as usize),
        );
        let id = platform::call1(FN_8019FE50, p32(&mut handle as *mut [u32; 10] as usize));
        let length = platform::call1(FN_8019FE80, p32(&mut handle as *mut [u32; 10] as usize));
        platform::call1(FN_8019FE90, p32(&mut handle as *mut [u32; 10] as usize));

        let record = manager + index as usize * 0x68;
        platform::write_u8(record + 0x5E, 0);
        platform::write_u32(record + 0x58, value);
        platform::write_u32(record + 0x54, length);
        platform::write_u8(record + 0x5D, mode);
        platform::write_u32(record + 0x50, id);
    }
}

/// Takes one animation record from the matching free pool.
///
/// Menus `4`, `0x17` and `0x19` use the three-record pool at `+0xB8B0`; every
/// other menu uses the four-record pool at `+0`. The first record whose
/// `+0x2E1C` byte is clear is marked active. If none is free, the oldest record
/// of the corresponding list is unlinked, re-initialised and marked active.
///
/// # Original
/// - Ghidra: `FUN_80174350` at `0x80174350` (408 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174350_FUN_80174350.c`
///
/// # Arguments
/// - `menu`: menu id selecting the pool and the list.
///
/// # Returns
/// The record pointer (null only if the list head is null, in which case the
/// original dereferences null immediately).
///
/// # Side effects
/// Marks a record active, calls the record stop helpers and rewrites the list
/// links.
///
/// # Safety
/// `0x80377FA0` must point at the live manager and the list heads must be
/// valid records or null.
///
/// # Confidence
/// high - the pool bounds, the free test and the unlink order are literal; the
/// field roles are inferred from the surrounding record code.
pub unsafe fn fn_80174350(menu: u32) -> usize {
    let manager = platform::read_u32(DAT_80377FA0) as usize;
    let alternate = menu == 4 || menu == 0x17 || menu == 0x19;
    let (count, base) = if alternate {
        (3usize, manager + 0xB8B0)
    } else {
        (4usize, manager)
    };

    let mut record = 0usize;
    let mut cursor = base;
    let mut index = 0usize;
    let mut remaining = count;
    loop {
        if remaining == 0 {
            break;
        }
        if platform::read_u8(cursor + 0x2E1C) == 0 {
            record = base + index * 0x2E2C;
            platform::write_u8(record + 0x2E1C, 1);
            break;
        }
        cursor += 0x2E2C;
        index += 1;
        remaining -= 1;
    }

    if record == 0 {
        let list = if alternate {
            manager + 0x1433C
        } else {
            manager + 0x14334
        };
        record = platform::read_u32(list) as usize;
        let record_id = platform::read_i32(record + 0x2E28);
        let record_list = if record_id == 4 || record_id == 0x17 || record_id == 0x19 {
            manager + 0x1433C
        } else {
            manager + 0x14334
        };

        platform::call2(FN_80194760, p32(record), 0);
        platform::call1(FN_801946C0, p32(record));
        platform::write_u8(record + 0x2E1C, 0);
        unlink_record(record_list, record);
        platform::write_u8(record + 0x2E1C, 1);
    }

    record
}

/// Pumps the menu widget and retires finished animation records.
///
/// The loop calls `0x801B24C0` with a one-word state record; while the state is
/// `1` it advances the three menu hooks and walks both animation lists,
/// removing every active record that has stopped and has no pending work at
/// `+0x418`. The loop ends when the pump reports `8`.
///
/// # Original
/// - Ghidra: `FUN_80174560` at `0x80174560` (576 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174560_FUN_80174560.c`
/// - Removed: the `bl 0x802DCD74` (`__savegprlr_27`) prologue and the
///   `bl 0x802DCDC0` (`__restgprlr_27`) epilogue; both only save and restore
///   non-volatile registers, and the epilogue left `0` in `r3`.
///
/// # Arguments
/// None; the manager comes from `0x80377FA0`.
///
/// # Side effects
/// Calls the three menu hooks and unlinks/rewrites record links.
///
/// # Safety
/// `0x80377FA0` must point at the live manager; the records and list heads
/// must be valid.
///
/// # Confidence
/// high - the two list walks and the termination value are literal.
pub unsafe fn fn_80174560() {
    let mut state = 0u32;
    loop {
        loop {
            platform::call3(
                FN_801B24C0,
                p32(platform::read_u32(DAT_80377FA0) as usize + 0x14680),
                p32(&mut state as *mut u32 as usize),
                1,
            );
            if state != 1 {
                break;
            }
            platform::call0(FN_801944A0);
            platform::call0(FN_80192A70);
            platform::call0(FN_80191D20);

            let manager = platform::read_u32(DAT_80377FA0) as usize;
            if manager != 0 {
                let mut node = platform::read_u32(manager + 0x14334) as usize;
                while node != 0 {
                    let next = platform::read_u32(node + 0x2E20) as usize;
                    if platform::read_u8(node + 0x2E1C) != 0
                        && platform::call1(FN_801948B0, p32(node)) == 0
                        && platform::read_i32(node + 0x418) == 0
                    {
                        let record_id = platform::read_i32(node + 0x2E28);
                        let list = if record_id == 4
                            || record_id == 0x17
                            || record_id == 0x19
                        {
                            manager + 0x1433C
                        } else {
                            manager + 0x14334
                        };
                        platform::call2(FN_80194760, p32(node), 0);
                        platform::call1(FN_801946C0, p32(node));
                        platform::write_u8(node + 0x2E1C, 0);
                        unlink_record(list, node);
                    }
                    node = next;
                }

                let mut node = platform::read_u32(manager + 0x1433C) as usize;
                while node != 0 {
                    let next = platform::read_u32(node + 0x2E20) as usize;
                    if platform::read_u8(node + 0x2E1C) != 0
                        && platform::call1(FN_801948B0, p32(node)) == 0
                        && platform::read_i32(node + 0x418) == 0
                    {
                        let record_id = platform::read_i32(node + 0x2E28);
                        let list = if record_id == 4
                            || record_id == 0x17
                            || record_id == 0x19
                        {
                            manager + 0x1433C
                        } else {
                            manager + 0x14334
                        };
                        platform::call2(FN_80194760, p32(node), 0);
                        platform::call1(FN_801946C0, p32(node));
                        platform::write_u8(node + 0x2E1C, 0);
                        unlink_record(list, node);
                    }
                    node = next;
                }
            }
        }
        if state == 8 {
            break;
        }
    }
}

/// Removes one record from a doubly-linked animation list.
///
/// Host-side factoring of the identical unlink block in [`fn_80174350`],
/// [`fn_80174560`] and [`fn_80174af0`]. `+0x2E24` is the back link (zero on the
/// oldest record) and `+0x2E20` the forward link (zero on the newest record).
///
/// # Original
/// - Ghidra: none - host-side factoring of the identical unlink block in the
///   three animation-list walkers that use it.
/// - Evidence: `work/ghidra/main-gb/export/80174560_FUN_80174560.c` (shape)
///
/// # Arguments
/// - `list`: two-word list head/tail (`+0` oldest, `+4` newest).
/// - `node`: record to remove.
///
/// # Safety
/// `list` and every linked record must be valid or null.
///
/// # Confidence
/// high - the same six stores appear in all three callers.
unsafe fn unlink_record(list: usize, node: usize) {
    let forward = platform::read_u32(node + 0x2E20);
    let back = platform::read_u32(node + 0x2E24);
    if back == 0 {
        platform::write_u32(list, forward);
    } else {
        platform::write_u32(back as usize + 0x2E20, forward);
    }
    if forward == 0 {
        platform::write_u32(list + 4, back);
    } else {
        platform::write_u32(forward as usize + 0x2E24, back);
    }
    platform::write_u32(node + 0x2E20, 0);
    platform::write_u32(node + 0x2E24, 0);
}

/// Plays the `midi_00_home_button_mid` sound for one menu item.
///
/// The sound name is selected from the table at `0x8032A248`, opened on the
/// manager's `+0x1434C` sound object, and, when a sound id is returned, bound
/// to a record from [`fn_80174350`]. The record is then appended to the list
/// matching the item id.
///
/// # Original
/// - Ghidra: `FUN_801747a0` at `0x801747A0` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801747a0_FUN_801747a0.c`
///
/// # Arguments
/// - `menu`: menu item id used for the sound name and record list.
///
/// # Side effects
/// Starts a sound, writes the record id and links the record into the list.
///
/// # Safety
/// `0x80377FA0` must point at the live manager with a valid sound object.
///
/// # Confidence
/// high - the table, the three sound calls and the two list-insertion arms are
/// literal.
pub unsafe fn fn_801747a0(menu: u32) {
    let manager = platform::read_u32(DAT_80377FA0) as usize;
    let name = platform::read_u32(TABLE_HOME_BUTTON_MIDI + menu as usize * 4);
    let mut handle = [0u32; 6];
    let opened = platform::call3(
        FN_8019F700,
        p32(manager + 0x1434C),
        name,
        p32(&mut handle as *mut [u32; 6] as usize),
    );
    let sound = if opened == 0 {
        0
    } else {
        platform::call1(FN_8019FE50, p32(&mut handle as *mut [u32; 6] as usize))
    };

    if sound != 0 {
        let record = fn_80174350(menu);
        platform::call5(
            FN_80194630,
            p32(record),
            sound,
            platform::read_u32(manager + 0x146B4),
            platform::read_u32(manager + 0x146B8),
            0,
        );
        platform::call2(FN_80194760, p32(record), 1);
        platform::write_i32(record + 0x2E28, menu as i32);

        let list = if menu == 4 || menu == 0x17 || menu == 0x19 {
            manager + 0x1433C
        } else {
            manager + 0x14334
        };
        let tail = platform::read_u32(list + 4);
        if tail == 0 {
            platform::write_u32(list, p32(record));
            platform::write_u32(list + 4, p32(record));
            platform::write_u32(record + 0x2E20, 0);
            platform::write_u32(record + 0x2E24, 0);
        } else {
            platform::write_u32(tail as usize + 0x2E20, p32(record));
            platform::write_u32(record + 0x2E24, tail);
            platform::write_u32(record + 0x2E20, 0);
            platform::write_u32(list + 4, p32(record));
        }
    }
}

/// Initialises the pause/home-button menu.
///
/// Only runs for a resource block larger than `0x186FF` bytes. It resets the
/// sound-system idle flag, clears all seven animation-record state words, loads
/// the `wt_HomeButtonSe_wt` and `wt_HomeButtonSe_pcm` sounds, binds the timer
/// group and registers [`fn_80174560`] as the pump. On success the manager is
/// published at `0x80377FA0`.
///
/// # Original
/// - Ghidra: `FUN_801748e0` at `0x801748E0` (500 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801748e0_FUN_801748e0.c`
///
/// # Arguments
/// - `sound`: sound archive/object passed to `0x8019F660`.
/// - `manager`: writable menu manager.
/// - `size`: resource size; must exceed `0x186FF`.
///
/// # Side effects
/// Clears twenty-two record words, opens two sounds, arms the pump and stores
/// the manager pointer.
///
/// # Safety
/// `manager` must be a writable block of the documented shape.
///
/// # Confidence
/// high - every store, sound name and call operand is literal; the two stack
/// sound handles are opaque records.
pub unsafe fn fn_801748e0(sound: usize, manager: usize, size: u32) {
    if size > 0x186FF {
        if call0_r(FN_8019F1E0) == 0 {
            platform::call1(FN_8019F1F0, 0);
            platform::call0(FN_801A0050);
        }
        platform::call0(FN_80191050);
        platform::call0(FN_80192990);
        platform::call0(FN_80194470);

        platform::write_u8(manager + 0x2E1C, 0);
        platform::write_u32(manager + 0x2E20, 0);
        platform::write_u32(manager + 0x2E24, 0);
        platform::write_u8(manager + 0x5C48, 0);
        platform::write_u32(manager + 0x5C4C, 0);
        platform::write_u32(manager + 0x5C50, 0);
        platform::write_u8(manager + 0x8A74, 0);
        platform::write_u32(manager + 0x8A78, 0);
        platform::write_u32(manager + 0x8A7C, 0);
        platform::write_u8(manager + 0xB8A0, 0);
        platform::write_u32(manager + 0xB8A4, 0);
        platform::write_u32(manager + 0xB8A8, 0);
        platform::write_u8(manager + 0xE6CC, 0);
        platform::write_u32(manager + 0xE6D0, 0);
        platform::write_u32(manager + 0xE6D4, 0);
        platform::write_u8(manager + 0x114F8, 0);
        platform::write_u32(manager + 0x114FC, 0);
        platform::write_u32(manager + 0x11500, 0);
        platform::write_u8(manager + 0x14324, 0);
        platform::write_u32(manager + 0x14328, 0);
        platform::write_u32(manager + 0x1432C, 0);
        platform::write_u32(manager + 0x146B4, 0);
        platform::write_u32(manager + 0x146B8, 0);
        platform::write_u32(manager + 0x146B0, p32(manager + 0x14700));
        platform::write_u32(manager + 0x14334, 0);
        platform::write_u32(manager + 0x14338, 0);
        platform::write_u32(manager + 0x1433C, 0);
        platform::write_u32(manager + 0x14340, 0);
        platform::write_u32(manager + 0x14348, 0);

        if platform::call2(FN_8019F660, sound as u32, p32(manager + 0x1434C)) != 0 {
            let mut wt_handle = [0u32; 4];
            if platform::call3(
                FN_8019F700,
                p32(manager + 0x1434C),
                p32(S_HOME_BUTTON_SE_WT),
                p32(&mut wt_handle as *mut [u32; 4] as usize),
            ) != 0
            {
                let wt_id =
                    platform::call1(FN_8019FE50, p32(&mut wt_handle as *mut [u32; 4] as usize));
                platform::write_u32(manager + 0x146B4, wt_id);

                let mut pcm_handle = [0u32; 3];
                if platform::call3(
                    FN_8019F700,
                    p32(manager + 0x1434C),
                    p32(S_HOME_BUTTON_SE_PCM),
                    p32(&mut pcm_handle as *mut [u32; 3] as usize),
                ) != 0
                {
                    let pcm_id = platform::call1(
                        FN_8019FE50,
                        p32(&mut pcm_handle as *mut [u32; 3] as usize),
                    );
                    platform::write_u32(manager + 0x146B8, pcm_id);
                    platform::call3(FN_801B2390, p32(manager + 0x14680), p32(manager + 0x146A0), 4);

                    let offset = size - 0x14700;
                    let callback = platform::call7(
                        FN_801B5130,
                        p32(manager + 0x14368),
                        FN_80174560 as u32,
                        0,
                        platform::read_u32(manager + 0x146B0).wrapping_add(offset),
                        offset,
                        4,
                        0,
                    );
                    if callback != 0 {
                        platform::write_u32(DAT_80377FA0, p32(manager));
                        platform::call1(FN_801B5860, p32(manager + 0x14368));
                        let lock = call0_r(FN_801B1140);
                        let handle = platform::call1(FN_801A1FD0, FN_801744F0 as u32);
                        platform::write_u32(manager + 0x14348, handle);
                        platform::call1(FN_801B1180, lock);
                    }
                }
            }
        }
    }
}

/// Releases every animation record of both pools.
///
/// The four-record pool at `+0` and the three-record pool at `+0xB8B0` are
/// walked; every active record is stopped, unlinked and cleared.
///
/// # Original
/// - Ghidra: `FUN_80174af0` at `0x80174AF0` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174af0_FUN_80174af0.c`
/// - Removed: the `bl 0x802DCD70` (`__savegprlr_26`) prologue and the
///   `bl 0x802DCDBC` (`__restgprlr_26`) epilogue; both only save and restore
///   non-volatile registers.
///
/// # Arguments
/// None; the manager comes from `0x80377FA0`.
///
/// # Side effects
/// Stops animations and rewrites both list pairs.
///
/// # Safety
/// `0x80377FA0` must be null or point at the live manager.
///
/// # Confidence
/// high - both recorded loop bounds and every unlink store are literal.
pub unsafe fn fn_80174af0() {
    let manager = platform::read_u32(DAT_80377FA0) as usize;

    if manager != 0 {
        let mut index = 0usize;
        let mut offset = 0usize;
        while index < 4 {
            let record = manager + offset;
            if platform::read_u8(record + 0x2E1C) != 0 {
                let record_id = platform::read_i32(record + 0x2E28);
                let list = if record_id == 4 || record_id == 0x17 || record_id == 0x19 {
                    manager + 0x1433C
                } else {
                    manager + 0x14334
                };
                platform::call2(FN_80194760, p32(record), 0);
                platform::call1(FN_801946C0, p32(record));
                platform::write_u8(record + 0x2E1C, 0);
                unlink_record(list, record);
            }
            index += 1;
            offset += 0x2E2C;
        }

        let mut index = 0usize;
        let mut offset = 0usize;
        while index < 3 {
            let record = manager + 0xB8B0 + offset;
            if platform::read_u8(record + 0x2E1C) != 0 {
                let record_id = platform::read_i32(record + 0x2E28);
                let list = if record_id == 4 || record_id == 0x17 || record_id == 0x19 {
                    manager + 0x1433C
                } else {
                    manager + 0x14334
                };
                platform::call2(FN_80194760, p32(record), 0);
                platform::call1(FN_801946C0, p32(record));
                platform::write_u8(record + 0x2E1C, 0);
                unlink_record(list, record);
            }
            index += 1;
            offset += 0x2E2C;
        }
    }
}

/// Chooses the menu viewport and boots the home-button menu object.
///
/// The mode is `0` when `+0x3C` is non-null, `1` when `+0x10` is non-null and
/// `2` otherwise. Mode `1` fills the global viewport record and publishes it.
/// The mode is then recomputed and the two global viewport pointers are
/// updated. Finally the menu object is allocated and its two constructors run.
///
/// # Original
/// - Ghidra: `FUN_80174e10` at `0x80174E10` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174e10_FUN_80174e10.c`
///
/// # Arguments
/// - `this`: boot record; `+0x10`, `+0x2C` and `+0x3C` are read.
///
/// # Side effects
/// Fills `0x80377FA8`, writes both viewport globals, allocates `0x7A0` bytes
/// and constructs the menu.
///
/// # Safety
/// `this` must be a readable boot record; `0x80377FA8` writable.
///
/// # Confidence
/// medium - every branch and store is literal; the three viewport pointers
/// are global data whose roles are inferred from the surrounding boot code.
pub unsafe fn fn_80174e10(this: usize) {
    let mode = if platform::read_i32(this + 0x3C) == 0 {
        if platform::read_i32(this + 0x10) != 0 {
            1
        } else {
            2
        }
    } else {
        0
    };

    if mode == 1 {
        let viewport = platform::call3(
            FN_801D4380,
            platform::read_u32(this + 0x10),
            platform::read_u32(this + 0x2C),
            0,
        );
        platform::call3(FN_801D5560, p32(DAT_80377FA8), viewport, 0x20);
        platform::write_u32(DAT_8032A304, p32(DAT_80377FA8));
    }

    let mode = if platform::read_i32(this + 0x3C) == 0 {
        if platform::read_i32(this + 0x10) != 0 {
            1
        } else {
            2
        }
    } else {
        0
    };

    if mode == 0 {
        platform::write_u32(DAT_8032A304, platform::read_u32(this + 0x3C));
        platform::write_u32(DAT_80378008, platform::read_u32(this + 0x3C));
    } else if mode == 1 {
        platform::write_u32(DAT_80378008, platform::read_u32(DAT_8032A304));
    } else if mode == 2 {
        platform::write_u32(DAT_8032A304, platform::read_u32(DAT_80378008));
    }

    fn_801754b0(this as u32);
    fn_80175500();
    fn_80175510(fn_80175500() as usize);
}

/// Constructs the home-button menu object.
///
/// Clears the menu's handles, state words and list pairs, constructs the
/// embedded draw-info object at `+0x258`, initialises the eight timer objects
/// (`+0x468..` and `+0x528..`, stride `0x30`) plus the timer at `+0x5E8`, and
/// initialises the global widget at `0x80377FC0`.
///
/// # Original
/// - Ghidra: `FUN_80175330` at `0x80175330` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175330_FUN_80175330.c`
///
/// # Arguments
/// - `this`: writable `0x7A0`-byte menu object.
/// - `argument`: owning object stored at `+4`.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be a writable `0x7A0`-byte block; `0x80377FC0` must be a valid
/// widget.
///
/// # Confidence
/// high - every offset and the two timer loops are literal.
pub unsafe fn fn_80175330(this: usize, argument: u32) -> u32 {
    platform::write_u32(this + 4, argument);
    platform::write_u32(this + 8, 0);
    platform::write_u32(this + 0x238, 0);
    platform::write_u32(this + 0x250, 0);
    fn_8017f390(this + 0x258);
    platform::write_u32(this + 0x454, 0);
    platform::write_u32(this + 0x458, 0x1E);
    platform::write_u32(this + 0x45C, 0);
    platform::write_u8(this + 0x461, 0);
    platform::write_u8(this + 0x462, 0);
    platform::write_u8(this + 0x463, 0);
    platform::write_u8(this + 0x460, 1);
    platform::write_u32(this + 0x618, 1);
    platform::write_u32(this + 0x14, 2);
    platform::write_u32(this + 0xB8, 0xFFFF_FFFF);
    platform::write_u32(this + 0x18, 0xFFFF_FFFF);
    platform::write_u32(this + 0x1C, 0);
    platform::write_u32(this, 0);
    platform::write_u32(this + 0x68, 0);
    platform::write_u8(this + 0x8E, 0);
    platform::write_u32(this + 0x6C, 0);
    platform::write_u32(this + 0x70, 0);
    platform::write_u32(this + 0x74, 0);
    platform::write_u32(this + 0x78, 0);
    platform::write_u8(this + 0x8F, 0);
    platform::write_u8(this + 0x90, 0);
    platform::write_u8(this + 0x97, 0);

    let mut index = 0usize;
    let mut offset = 0usize;
    while index < 4 {
        platform::call1(FN_801AB2E0, p32(this + 0x468 + offset));
        platform::call1(FN_801AB2E0, p32(this + 0x528 + offset));
        index += 1;
        offset += 0x30;
    }
    platform::call1(FN_801AB2E0, p32(this + 0x5E8));
    platform::call1(FN_801B2E20, p32(DAT_80377FC0));

    this as u32
}

// ===========================================================================
// Wave 26: widget setters, grid/record helpers and menu wrappers
// ===========================================================================
//
// The entries below are the small bodies around the wave-7 grid descriptor,
// the wave-21 widget table and the pause-menu entry points. They keep the
// original stores and call order; the inferred field roles are named in each
// doc block. The wrappers at 0x80174FD0..0x801750E0 are the public operations
// used by the pause overlay.

/// `0x8016DCA0`: grid/draw-object destructor used by [`fn_80170cd0`].
const FN_8016DCA0: usize = 0x8016_DCA0;

/// `0x802B9280`: `operator delete`-style release.
const FN_802B9280: usize = 0x802B_9280;

/// `0x8018A6E0`: indexed lookup in a list/array (`base + 8`, index).
const FN_8018A6E0: usize = 0x8018_A6E0;

/// `0x801AB1C0`: debug-print helper used by [`fn_80174f30`].
const FN_801AB1C0: usize = 0x801A_B1C0;

/// `0x80175F70`: battery/controller pane builder (still untranslated).
const FN_80175F70: usize = 0x8017_5F70;

/// `0x802F5A6C`: first default widget word copied by [`fn_801734c0`].
const DAT_802F5A6C: usize = 0x802F_5A6C;

/// `0x802F5A70`: second default widget word copied by [`fn_801734c0`].
const DAT_802F5A70: usize = 0x802F_5A70;

/// `0x802F5AA0`: lower level bound compared by [`fn_80174cc0`].
const DAT_802F5AA0: usize = 0x802F_5AA0;

/// `0x802F5AA4`: first speed scale of [`fn_80174cc0`].
const DAT_802F5AA4: usize = 0x802F_5AA4;

/// `0x802F5AA8`: second speed scale of [`fn_80174cc0`].
const DAT_802F5AA8: usize = 0x802F_5AA8;

/// `0x802E5490`: double-shaped curve routine called by [`fn_80174cc0`]
/// (the call site uses the 4-byte thunk `0x802E8618`).
const FN_802E5490: usize = 0x802E_5490;

/// `0x801948C0`: apply one speed value to an animation record.
const FN_801948C0: usize = 0x8019_48C0;

/// `0x801E0F40`: controller connection query used by [`fn_801731f0`].
const FN_801E0F40: usize = 0x801E_0F40;

/// `0x8032A300`: `"<< RVL_SDK - HBM release build: ..."` debug string.
const S_HBM_RELEASE_BUILD: usize = 0x8032_A300;

/// Calls a one-argument `f64` function on the target.
///
/// # Original
/// Host-side helper; no binary address. Models the `f1 -> f1` ABI of the
/// routine at `0x802E5490` invoked by `0x80174CC0`.
///
/// # Arguments
/// - `addr`: target function address.
/// - `value`: the `f64` argument passed in `f1`.
///
/// # Returns
/// The `f64` result left in `f1`.
///
/// # Safety
/// `addr` must point to a function with this exact ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call_f64_f64(addr: usize, value: f64) -> f64 {
    let f: extern "C" fn(f64) -> f64 = core::mem::transmute(addr);
    f(value)
}

/// Stores one word at `+4` of a grid/record object.
///
/// # Original
/// - Ghidra: `FUN_8016ffc0` at `0x8016FFC0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ffc0_FUN_8016ffc0.c`
///
/// # Arguments
/// - `this`: writable object.
/// - `value`: word stored at `+4`.
///
/// # Side effects
/// Writes `this + 4`.
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - one literal store; the object role is inferred from the callers in
/// this module.
pub unsafe fn fn_8016ffc0(this: usize, value: u32) {
    write_word(this, 4, value);
}

/// Creates the grid's buffer object from two resource arguments.
///
/// `0x801D4380(a, b, 0)` returns the new object, which is stored at `+0`.
///
/// # Original
/// - Ghidra: `FUN_80170630` at `0x80170630` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170630_FUN_80170630.c`
///
/// # Arguments
/// - `this`: writable grid object; receives the created object.
/// - `a`, `b`: resource/format arguments forwarded to `0x801D4380`.
///
/// # Side effects
/// Creates one object and stores it at `+0`.
///
/// # Safety
/// `this` must be writable; `a`/`b` must be valid for `0x801D4380`.
///
/// # Confidence
/// medium - the call and store are literal; the meaning of `a`/`b` is inferred
/// from the neighbouring grid constructor.
pub unsafe fn fn_80170630(this: usize, a: u32, b: u32) {
    let object = platform::call3(FN_801D4380, a, b, 0);
    write_word(this, 0, object);
}

/// Destroys a grid object and optionally frees it.
///
/// The original uses a comma expression: `0x8016DCA0(this, 0)` always runs for
/// a non-null object, and `0x802B9280(this)` runs only when `flag > 0`.
///
/// # Original
/// - Ghidra: `FUN_80170cd0` at `0x80170CD0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170cd0_FUN_80170cd0.c`
///
/// # Arguments
/// - `this`: grid object; may be null.
/// - `flag`: when greater than zero, the object is freed.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Destroys the object, then may free it.
///
/// # Safety
/// A non-null `this` must be a live grid object.
///
/// # Confidence
/// high - the comma expression, guards and both calls are literal.
pub unsafe fn fn_80170cd0(this: usize, flag: i32) -> usize {
    if this != 0 {
        platform::call2(FN_8016DCA0, p32(this), 0);
        if flag > 0 {
            platform::call1(FN_802B9280, p32(this));
        }
    }
    this
}

/// Reads one list entry's value word.
///
/// The entry is found by `0x8018A6E0(this + 8, index)`; its `+4` word is
/// returned unchanged.
///
/// # Original
/// - Ghidra: `FUN_80171de0` at `0x80171DE0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80171de0_FUN_80171de0.c`
///
/// # Arguments
/// - `this`: list/array owner.
/// - `index`: 16-bit entry index.
///
/// # Returns
/// The word at `entry + 4`.
///
/// # Safety
/// `this + 8` must be a valid lookup base and the returned entry must be
/// readable at `+4`.
///
/// # Confidence
/// high - the lookup and load are literal.
pub unsafe fn fn_80171de0(this: usize, index: u16) -> u32 {
    let entry = platform::call2(FN_8018A6E0, p32(this + 8), index as u32) as usize;
    read_word(entry, 4)
}

/// Closes every pane in the list.
///
/// The count is the `u16` at `+0x10`; each entry is resolved through
/// `0x8018A6E0(this + 8, index)`, and slot `+0x10` of the object at `entry + 4`
/// is called. The virtual call has no operands in the export.
///
/// # Original
/// - Ghidra: `FUN_80172000` at `0x80172000` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172000_FUN_80172000.c`
///
/// # Arguments
/// - `this`: list owner with a `u16` count at `+0x10`.
///
/// # Side effects
/// Calls one virtual close method per entry.
///
/// # Safety
/// `this + 0x10` must be readable and every resolved entry must hold a live
/// object at `+4`.
///
/// # Confidence
/// medium - loop bound, lookup and slot are literal; the pane role and the
/// zero-argument virtual call are inferred from the list shape.
pub unsafe fn fn_80172000(this: usize) {
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = platform::call2(FN_8018A6E0, p32(this + 8), index & 0xFFFF) as usize;
        let object = read_word(entry, 4) as usize;
        let close = vtable_method(object, 0, 0x10);
        platform::call0(close);
        index += 1;
    }
}

/// Stores one byte at `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_80172180` at `0x80172180` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172180_FUN_80172180.c`
///
/// # Arguments
/// - `this`: writable object.
/// - `value`: byte stored at `+0x24`.
///
/// # Side effects
/// Writes `this + 0x24`.
///
/// # Safety
/// `this + 0x24` must be writable.
///
/// # Confidence
/// high - one literal store.
pub unsafe fn fn_80172180(this: usize, value: u8) {
    write_byte(this, 0x24, value);
}

/// Stores the widget's `+4` word.
///
/// # Original
/// - Ghidra: `FUN_80172e70` at `0x80172E70` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172e70_FUN_80172e70.c`
///
/// # Arguments
/// - `this`: writable widget.
/// - `value`: word stored at `+4`.
///
/// # Side effects
/// Writes `this + 4`.
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - one literal store.
pub unsafe fn fn_80172e70(this: usize, value: u32) {
    write_word(this, 4, value);
}

/// Stores the widget's `+0x24` callback word.
///
/// # Original
/// - Ghidra: `FUN_80172e90` at `0x80172E90` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172e90_FUN_80172e90.c`
///
/// # Arguments
/// - `this`: writable widget.
/// - `value`: word stored at `+0x24`.
///
/// # Side effects
/// Writes `this + 0x24`.
///
/// # Safety
/// `this + 0x24` must be writable.
///
/// # Confidence
/// high - one literal store; the callback role follows the wave-21 widget
/// table.
pub unsafe fn fn_80172e90(this: usize, value: u32) {
    write_word(this, 0x24, value);
}

/// Installs the widget's two channel hooks and connection byte.
///
/// The body selects channel `0`, installs [`fn_80172ea0`] through
/// `0x801E1060` and stores the handle at `+0x24`; selects channel `1`, installs
/// [`fn_80172fb0`] through `0x801E10F0` and stores the handle at `+0x28`; sets
/// the enabled byte at `+0x44`; then queries `0x801E0F40` and writes the
/// connection byte at `+0x40` for results `0` and `-1` only.
///
/// # Original
/// - Ghidra: `FUN_801731f0` at `0x801731F0` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801731f0_FUN_801731f0.c`
///
/// # Arguments
/// - `this`: live per-player widget.
///
/// # Side effects
/// Installs two hooks, writes the two handles, sets `+0x44`, and may set
/// `+0x40`.
///
/// # Safety
/// `this` must be a live widget and the controller subsystem must be ready.
///
/// # Confidence
/// high - the two channel calls, both stores and the three-way query result
/// are literal; the 16-byte stack argument is callee-owned.
pub unsafe fn fn_801731f0(this: usize) {
    let player = read_word(this, 0);
    platform::call1(FN_801E5450, 0);
    let input_handle = platform::call2(FN_801E1060, player, FN_80172EA0 as u32);
    write_word(this, 0x24, input_handle);
    platform::call1(FN_801E5450, 1);
    let event_handle = platform::call2(FN_801E10F0, player, FN_80172FB0 as u32);
    write_word(this, 0x28, event_handle);
    write_byte(this, 0x44, 1);

    let mut scratch = [0u8; 16];
    let query = platform::call2(
        FN_801E0F40,
        player,
        scratch.as_mut_ptr() as u32,
    ) as i32;
    if query == 0 {
        write_byte(this, 0x40, 1);
    } else if query == -1 {
        write_byte(this, 0x40, 0);
    }
}

/// Clears the three widget words at `+0x10`, `+0x14` and `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_801734a0` at `0x801734A0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801734a0_FUN_801734a0.c`
///
/// # Arguments
/// - `this`: writable widget.
///
/// # Side effects
/// Writes three zero words.
///
/// # Safety
/// `this + 0x10..+0x18` must be writable.
///
/// # Confidence
/// high - three literal stores.
pub unsafe fn fn_801734a0(this: usize) {
    write_word(this, 0x10, 0);
    write_word(this, 0x14, 0);
    write_word(this, 0x18, 0);
}

/// Copies the two default widget words into `+8` and `+0xC`.
///
/// # Original
/// - Ghidra: `FUN_801734c0` at `0x801734C0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801734c0_FUN_801734c0.c`
///
/// # Arguments
/// - `this`: writable widget.
///
/// # Side effects
/// Copies `0x802F5A6C` to `+8` and `0x802F5A70` to `+0xC`.
///
/// # Safety
/// `this + 8..+0xC` must be writable.
///
/// # Confidence
/// high - both source and destination offsets are literal.
pub unsafe fn fn_801734c0(this: usize) {
    let first = platform::read_u32(DAT_802F5A6C);
    let second = platform::read_u32(DAT_802F5A70);
    write_word(this, 8, first);
    write_word(this, 0xC, second);
}

/// Resets one animation record and installs its callback.
///
/// The record base is the word at `+0x20`; the index is the widget's `+0`
/// word. Both are forwarded to [`fn_801741b0`] unchanged.
///
/// # Original
/// - Ghidra: `FUN_801734e0` at `0x801734E0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801734e0_FUN_801734e0.c`
///
/// # Arguments
/// - `this`: widget whose `+0x20` word is the record base and `+0` the index.
///
/// # Side effects
/// Resets one 0x68-byte record and installs `0x80173FB0` on it.
///
/// # Safety
/// The `+0x20` base must hold at least `index + 1` records.
///
/// # Confidence
/// high - both operands and the target are literal.
pub unsafe fn fn_801734e0(this: usize) {
    fn_801741b0(read_word(this, 0x20) as usize, read_word(this, 0));
}

/// Clears the two widget hold bytes at `+0x42` and `+0x43`.
///
/// # Original
/// - Ghidra: `FUN_80173610` at `0x80173610` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173610_FUN_80173610.c`
///
/// # Arguments
/// - `this`: writable widget.
///
/// # Side effects
/// Writes zero to the hold-active and hold-started bytes.
///
/// # Safety
/// `this + 0x42..+0x43` must be writable.
///
/// # Confidence
/// high - two literal byte stores.
pub unsafe fn fn_80173610(this: usize) {
    write_byte(this, 0x42, 0);
    write_byte(this, 0x43, 0);
}

/// Updates both animation-record pools from a level value.
///
/// When the manager at `0x80377FA0` is live and `level` is above the
/// `0x802F5AA0` threshold, the speed offset is
/// `scale_a * scale_b * curve(level)`; otherwise it is the fixed `-0x388`.
/// The result is clamped to `[-0x388, 0x3C]` and applied to every active record
/// of the four-record pool at `+0` and the three-record pool at `+0xB8B0`
/// through `0x801948C0`.
///
/// # Original
/// - Ghidra: `FUN_80174cc0` at `0x80174CC0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174cc0_FUN_80174cc0.c`
/// - observed: the curve is reached through the 4-byte thunk `0x802E8618`,
///   whose body is `0x802E5490` (276 bytes).
///
/// # Arguments
/// - `level`: level/volume value; only used above the threshold.
///
/// # Side effects
/// Calls `0x801948C0(record, offset)` once per active record.
///
/// # Safety
/// `0x80377FA0` must be null or point at a live manager; every active record
/// must be a valid animation record.
///
/// # Confidence
/// medium - the manager guard, both loops, both clamps and all offsets are
/// literal; the curve's identity and the `f64` argument recovery are inferred
/// from the call shape (`FUN_802e5490` and its SDA2 coefficients).
pub unsafe fn fn_80174cc0(level: f64) {
    let manager = platform::read_u32(DAT_80377FA0) as usize;
    if manager == 0 {
        return;
    }

    let mut offset = if (read_f32(DAT_802F5AA0) as f64) < level {
        let curve = call_f64_f64(FN_802E5490, level);
        (read_f32(DAT_802F5AA4) * read_f32(DAT_802F5AA8) * (curve as f32)) as i32
    } else {
        -0x388
    };
    if offset > 0x3C {
        offset = 0x3C;
    }
    if offset < -0x388 {
        offset = -0x388;
    }

    let mut index = 0usize;
    let mut record_offset = 0usize;
    while index < 4 {
        let record = manager + record_offset;
        if platform::read_u8(record + 0x2E1C) != 0 {
            platform::call2(FN_801948C0, p32(record), offset as u32);
        }
        index += 1;
        record_offset += 0x2E2C;
    }

    let mut index = 0usize;
    let mut record_offset = 0usize;
    while index < 3 {
        let record = manager + 0xB8B0 + record_offset;
        if platform::read_u8(manager + record_offset + 0xE6CC) != 0 {
            platform::call2(FN_801948C0, p32(record), offset as u32);
        }
        index += 1;
        record_offset += 0x2E2C;
    }
}

/// Prints the release-build banner, resets the menu and rebuilds its panes.
///
/// # Original
/// - Ghidra: `FUN_80174f30` at `0x80174F30` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174f30_FUN_80174f30.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Prints the `HBM release build` string, resets the menu object and calls the
/// untranslated pane builder `0x80175F70`.
///
/// # Safety
/// The menu global read by `0x80175500` must be live.
///
/// # Confidence
/// medium - all three calls are literal; the build-banner role is inferred
/// from the string at `0x8032A300`.
pub unsafe fn fn_80174f30() {
    platform::call1(FN_801AB1C0, S_HBM_RELEASE_BUILD as u32);
    let menu = fn_80175500() as usize;
    fn_80175f70(menu);
}

/// Applies the compact/full button mode to the live menu.
///
/// # Original
/// - Ghidra: `FUN_80174fd0` at `0x80174FD0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174fd0_FUN_80174fd0.c`
///
/// # Arguments
/// - `mode`: forwarded to [`menu_set_button_layout`].
///
/// # Side effects
/// Rewrites the menu's button-bar mode bytes.
///
/// # Safety
/// The menu returned by `0x80175500` must be live.
///
/// # Confidence
/// high - the menu fetch and forwarded argument are literal.
pub unsafe fn fn_80174fd0(mode: u32) {
    let menu = fn_80175500() as usize;
    menu_set_button_layout(menu, mode);
}

/// Stores the menu's `+0x618` timer word.
///
/// # Original
/// - Ghidra: `FUN_801750a0` at `0x801750A0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801750a0_FUN_801750a0.c`
///
/// # Arguments
/// - `value`: word forwarded to [`fn_8017d3d0`].
///
/// # Side effects
/// Writes the live menu's `+0x618`.
///
/// # Safety
/// The menu returned by `0x80175500` must be live.
///
/// # Confidence
/// high - the menu fetch and forwarded argument are literal.
pub unsafe fn fn_801750a0(value: u32) {
    let menu = fn_80175500() as usize;
    fn_8017d3d0(menu, value);
}

/// Initialises the pause/home-button sound and menu manager.
///
/// The live menu is fetched first but its value is not consumed; the three
/// arguments are forwarded to [`fn_801748e0`] unchanged.
///
/// # Original
/// - Ghidra: `FUN_801750e0` at `0x801750E0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801750e0_FUN_801750e0.c`
///
/// # Arguments
/// - `sound`: sound object.
/// - `manager`: resource block.
/// - `size`: resource size.
///
/// # Side effects
/// Opens both home-button sounds and starts the animation-record pump.
///
/// # Safety
/// `sound`, `manager` and `size` must be valid for [`fn_801748e0`].
///
/// # Confidence
/// high - the order and all three forwarded arguments are literal.
pub unsafe fn fn_801750e0(sound: u32, manager: u32, size: u32) {
    let _ = fn_80175500();
    fn_801748e0(sound as usize, manager as usize, size);
}

// ===========================================================================
// Wave 28: menu record, timer, wrapper and GX-state bodies
// ===========================================================================

// Callees outside this file, called by address exactly as the export prints
// them. `unknown:` the exporting tool did not name them; the comments give the
// only role the call sites prove.

/// `0x80173860`: forwards `(record[8], record[0])` to `0x80174330`; the
/// caller at `0x80178A20` consumes the result left in `r3`.
const FN_80173860: usize = 0x8017_3860;
/// `0x801E35F0`: queries whether channel `i` owns a running sequence.
const FN_801E35F0: usize = 0x801E_35F0;
/// `0x80227EF0`: prices the scaled 16-bit samples into the output buffer.
const FN_80227EF0: usize = 0x8022_7EF0;
/// `0x801E3680`: submits the 0x14-byte sample block for channel `i`.
const FN_801E3680: usize = 0x801E_3680;
/// `0x801DFB40`: reads the per-channel byte at `channel_state + 0x983`.
const FN_801DFB40: usize = 0x801D_FB40;
/// `0x801DFAC0`: 4-byte thunk to `0x801F3840` (immediate release query).
const FN_801DFAC0: usize = 0x801D_FAC0;
/// `0x801DFAD0`: 4-byte thunk to `0x801F38F0` (deferred release query).
const FN_801DFAD0: usize = 0x801D_FAD0;
/// `0x801AB5B0`: arms a timer with a 64-bit start time
/// `(timer, delay, time_hi, time_lo, 0, period, callback)`.
const FN_801AB5B0: usize = 0x801A_B5B0;
/// `0x80329F38`: vtable/pointer table stored at manager `+0x1F0`.
///
/// unknown: the table's class; the store is the only evidence.
const PTR_80329F38: usize = 0x8032_9F38;

// GX/render-state callees used by the two frame bodies at `0x80175170` and
// `0x8017F0B0`. They are the same `0x801Cxxxx` state setters the other UI
// modules call by address; none has a recovered name in the export.
const FN_801C5860: usize = 0x801C_5860;
const FN_801C04D0: usize = 0x801C_04D0;
const FN_801C9940: usize = 0x801C_9940;
const FN_801C9A60: usize = 0x801C_9A60;
const FN_801C6AC0: usize = 0x801C_6AC0;
const FN_801C6AF0: usize = 0x801C_6AF0;
const FN_801C4A20: usize = 0x801C_4A20;
const FN_801C88F0: usize = 0x801C_88F0;
const FN_801C8790: usize = 0x801C_8790;
const FN_801C82F0: usize = 0x801C_82F0;
const FN_801C8370: usize = 0x801C_8370;
const FN_801C8330: usize = 0x801C_8330;
const FN_801C83D0: usize = 0x801C_83D0;
const FN_801C86C0: usize = 0x801C_86C0;
const FN_801C8DC0: usize = 0x801C_8DC0;
const FN_801C8E40: usize = 0x801C_8E40;
const FN_801C8E70: usize = 0x801C_8E70;
const FN_801C6130: usize = 0x801C_6130;
const FN_801C8110: usize = 0x801C_8110;
const FN_801C8600: usize = 0x801C_8600;
const FN_801C8640: usize = 0x801C_8640;
const FN_801C8250: usize = 0x801C_8250;
const FN_801C8130: usize = 0x801C_8130;
const FN_801C57A0: usize = 0x801C_57A0;
const FN_801C5570: usize = 0x801C_5570;
const FN_801C69E0: usize = 0x801C_69E0;

/// Unlinks one grid entry and releases its resource.
///
/// This is the destroy half of the pair whose create half is
/// [`fn_8016fe90`]; it matches the already-translated twin [`fn_8016fe40`]
/// byte for byte. The node at `+8` is unlinked through `0x80228F80`, then the
/// resource word at `+0` is passed to `0x801D4710` with the caller's value.
///
/// # Original
/// - Ghidra: `FUN_801705e0` at `0x801705E0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801705e0_FUN_801705e0.c`
/// - observed: the export prints only one operand for `0x80228F80`; the twin
///   `0x8016FE40` is translated the same way.
///
/// # Arguments
/// - `this`: grid object; resource at `+0`, list node at `+8`.
/// - `value`: forwarded to `0x801D4710` unchanged.
///
/// # Side effects
/// Unlinks the list node and releases the resource.
///
/// # Safety
/// `this + 8` must be a linked list node and `this + 0` a live resource.
///
/// # Confidence
/// high - both calls and the store order match the twin body.
pub unsafe fn fn_801705e0(this: usize, value: u32) {
    platform::call1(FN_80228F80, p32(this + 8));
    platform::call2(FN_801D4710, read_word(this, 0), value);
}

/// Clears the nine state bytes of a small UI record.
///
/// The original stores zero at `+0x18` and then at `+4..+0xB`, one byte at a
/// time. No caller survives in the export, so the record's owner is unknown.
///
/// # Original
/// - Ghidra: `FUN_80171c30` at `0x80171C30` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80171c30_FUN_80171c30.c`
///
/// # Arguments
/// - `this`: writable record; bytes `+4..+0xB` and `+0x18` are cleared.
///
/// # Side effects
/// Nine byte stores.
///
/// # Safety
/// `this + 0x18` must be writable.
///
/// # Confidence
/// high - every offset and value is literal; only the role is inferred.
pub unsafe fn fn_80171c30(this: usize) {
    write_byte(this, 0x18, 0);
    write_byte(this, 4, 0);
    write_byte(this, 5, 0);
    write_byte(this, 6, 0);
    write_byte(this, 7, 0);
    write_byte(this, 8, 0);
    write_byte(this, 9, 0);
    write_byte(this, 10, 0);
    write_byte(this, 11, 0);
}

/// Deletes one UI record when the delete flag is positive.
///
/// Same shape as the nunchuk record destructors: a non-null test, a positive
/// flag test and one `0x802B9280(this)` call. The free operand is not printed
/// by the export, but `0x802B9280` is declared with `param_1` and every twin
/// passes the record pointer.
///
/// # Original
/// - Ghidra: `FUN_80172330` at `0x80172330` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172330_FUN_80172330.c`
///
/// # Arguments
/// - `this`: record pointer; may be null.
/// - `flag`: when greater than zero, the record is freed.
///
/// # Returns
/// `this`, so the caller can reuse the pointer.
///
/// # Side effects
/// May release the record through `0x802B9280`.
///
/// # Safety
/// A non-null `this` must be a live record.
///
/// # Confidence
/// high - both guards and the call target are literal; the operand follows
/// the twin bodies.
pub unsafe fn fn_80172330(this: usize, flag: i32) -> usize {
    if this != 0 && flag > 0 {
        platform::call1(FN_802B9280, p32(this));
    }
    this
}

/// Walks a chain of up to five menu records and reports whether all are
/// enabled.
///
/// The first four hops are unrolled: each record's `+0xCF` bit 0 is tested,
/// then `+0xC` is followed. At the fifth record the getters `0x80172CE0`
/// (enabled bit) and `0x80172CF0` (`+0xC` next pointer) are called; when a
/// next pointer exists the body recurses on it. An absent next pointer counts
/// as success, a disabled record as failure.
///
/// # Original
/// - Ghidra: `FUN_80172be0` at `0x80172BE0` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172be0_FUN_80172be0.c`
/// - observed: `0x80172CE0` is `byte[+0xCF] & 1` and `0x80172CF0` is the
///   `+0xC` word.
/// - inferred: the recursive call's operand is the second `0x80172CF0`
///   result; Ghidra printed the call with no operand.
///
/// # Arguments
/// - `this`: first record of the chain.
///
/// # Returns
/// `1` when every visited record is enabled and the chain ends, `0` as soon as
/// one record is disabled.
///
/// # Side effects
/// Reads only; the recursion depth is bounded by the chain length.
///
/// # Safety
/// Every record reached through `+0xC` must be readable at `+0xCF`.
///
/// # Confidence
/// medium - the unrolled checks are literal; the recursive operand is inferred
/// from the double getter call.
pub unsafe fn fn_80172be0(this: usize) -> u32 {
    if read_byte(this, 0xCF) & 1 == 0 {
        return 0;
    }

    let mut node = read_word(this, 0xC) as usize;
    if node == 0 {
        return 1;
    }
    if read_byte(node, 0xCF) & 1 == 0 {
        return 0;
    }

    node = read_word(node, 0xC) as usize;
    if node == 0 {
        return 1;
    }
    if read_byte(node, 0xCF) & 1 == 0 {
        return 0;
    }

    node = read_word(node, 0xC) as usize;
    if node == 0 {
        return 1;
    }
    if read_byte(node, 0xCF) & 1 == 0 {
        return 0;
    }

    node = read_word(node, 0xC) as usize;
    if node == 0 {
        return 1;
    }
    if fn_80172ce0(node) == 0 {
        return 0;
    }
    let next = fn_80172cf0(node);
    if next == 0 {
        return 1;
    }
    // observed: the getter is evaluated a second time for the recursive
    // operand; both calls read the same pure field.
    let _again = fn_80172cf0(node);
    fn_80172be0(_again as usize)
}

/// Advances one per-player widget's press/hold timer.
///
/// With no active record (`0x801742E0` returns 0) the body arms the hold timer
/// once (`+0x43` latch, 64-bit start stored at `+0x38`) and clears both bytes
/// after 1000 ms. With an active record it clears the latch, and after
/// 480,000 ms sends notification `2` to a connected channel, arms the shared
/// 1000 ms timer and sets `+0x41`. A release query below `0x56` starts the same
/// notification path immediately.
///
/// # Original
/// - Ghidra: `FUN_80173620` at `0x80173620` (568 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173620_FUN_80173620.c`
/// - observed: `0x801B5F80` returns the 64-bit OS time in `r3:r4`; Ghidra
///   printed the low half as `extraout_r4`.
///
/// # Arguments
/// - `this`: per-player widget (same layout as [`fn_80172ea0`]).
///
/// # Side effects
/// Writes `+0x38..+0x3F`, `+0x41`, `+0x42`, `+0x43` and may arm a timer.
///
/// # Safety
/// The widget and its `0x80377EC8` timer slot must be live.
///
/// # Confidence
/// medium - the state machine and timer arithmetic are literal; the elapsed
/// checks use the recovered low half of the 64-bit time.
pub unsafe fn fn_80173620(this: usize) {
    let channel = read_word(this, 0);
    let active = fn_801742e0(read_word(this, 0x20) as usize, channel);

    if active == 0 {
        if read_byte(this, 0x42) != 0 {
            if read_byte(this, 0x43) == 0 {
                let now = platform::call_u64(FN_801B5F80);
                platform::write_u32(this + 0x38, (now >> 32) as u32);
                platform::write_u32(this + 0x3C, now as u32);
                write_byte(this, 0x43, 1);
            } else {
                let now = platform::call_u64(FN_801B5F80);
                let ticks = platform::read_u32(OS_BUS_CLOCK) / 4000;
                let elapsed = (now as u32).wrapping_sub(read_word(this, 0x3C)) / ticks;
                if elapsed > 999 {
                    write_byte(this, 0x42, 0);
                    write_byte(this, 0x43, 0);
                }
            }
        }
    } else {
        if read_byte(this, 0x42) != 0 {
            write_byte(this, 0x43, 0);
            let now = platform::call_u64(FN_801B5F80);
            let ticks = platform::read_u32(OS_BUS_CLOCK) / 4000;
            let elapsed = (now as u32).wrapping_sub(read_word(this, 0x34)) / ticks;
            if elapsed > 479_999 {
                let channel = read_word(this, 0);
                write_byte(this, 0x42, 0);
                write_byte(this, 0x43, 0);
                if platform::call1(FN_801E1DD0, channel) != 0 {
                    platform::call3(FN_801E1E20, channel, 2, 0);
                    let timer = TABLE_EVENT_TIMER + channel as usize * 0x30;
                    platform::call2(FN_801ABA70, p32(timer), channel);
                    platform::call1(FN_801AB640, p32(timer));
                    platform::call5(
                        FN_801AB540,
                        p32(timer),
                        0x1062_4DD3,
                        0,
                        (platform::read_u32(OS_BUS_CLOCK) / 4000) * 1000,
                        FN_801730C0 as u32,
                    );
                    write_byte(this, 0x41, 1);
                }
                return;
            }
        }
        if read_byte(this, 0x41) == 0
            && platform::call1(FN_801DFB40, channel) < 0x56
        {
            let channel = read_word(this, 0);
            if platform::call1(FN_801E1DD0, channel) != 0 {
                platform::call3(FN_801E1E20, channel, 2, 0);
                let timer = TABLE_EVENT_TIMER + channel as usize * 0x30;
                platform::call2(FN_801ABA70, p32(timer), channel);
                platform::call1(FN_801AB640, p32(timer));
                platform::call5(
                    FN_801AB540,
                    p32(timer),
                    0x1062_4DD3,
                    0,
                    (platform::read_u32(OS_BUS_CLOCK) / 4000) * 1000,
                    FN_801730C0 as u32,
                );
                write_byte(this, 0x41, 1);
            }
        }
    }
}

/// Updates the four home-menu scrolling-text widgets.
///
/// For each of the four 0x68-byte records at `0x80377F98`, when the record has
/// a source buffer (`+0x50`) and its channel is connected, the body either
/// counts the finished frames (`+0x5E`, clearing `+0x50` after 60) or scales
/// up to 40 16-bit samples by `byte[+0x5D] / 10`, zero-fills the remainder,
/// submits the block through `0x80227EF0`/`0x801E3680`, advances the source
/// pointer by 0x50 bytes and shrinks `+0x54`. The sound lock taken at the top
/// is released with `0x801B1180`.
///
/// # Original
/// - Ghidra: `FUN_80173a30` at `0x80173A30` (620 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173a30_FUN_80173a30.c`
/// - Removed: the `bl 0x802DCD70` (`__savegprlr_26`) prologue and the
///   `bl 0x802DCDBC` (`__restgprlr_26`) epilogue; both only save and restore
///   non-volatile registers.
/// - observed: the sample scaling performs two sign corrections that cancel,
///   leaving a plain signed `value / 10`; the Rust keeps the single divide and
///   documents the original pair.
///
/// # Arguments
/// None; the record base is the global `0x80377F98`.
///
/// # Side effects
/// Consumes source samples, writes the 40-entry stack block and updates the
/// four records' `+0x50`, `+0x54`, `+0x58`, `+0x5C`, `+0x5E` fields.
///
/// # Safety
/// `0x80377F98` must point at four live 0x68-byte records, and each active
/// record's `+0x50` must address `(+0x54 / 2)` readable samples.
///
/// # Confidence
/// medium - the loops and stores are literal; the scaling reduction is
/// algebraic, and `0x80227EF0`'s internal SDA2 polynomial is out of scope.
pub unsafe fn fn_80173a30() {
    let base = platform::read_u32(DAT_80377F98);
    if base != 0 {
        let mut player: u32 = 0;
        let mut record = base as usize;
        loop {
            if platform::read_i32(record + 0x50) != 0
                && platform::call1(FN_801E1DD0, player) != 0
            {
                let lock = call0_r(FN_801B1140);
                if platform::call1(FN_801E35F0, player) == 0 {
                    let frames = platform::read_u8(record + 0x5E).wrapping_add(1);
                    platform::write_u8(record + 0x5E, frames);
                    if frames > 0x3C {
                        platform::write_u32(record + 0x50, 0);
                    }
                } else {
                    let mut scaled = [0i16; 0x28];
                    let samples = platform::read_u32(record + 0x54) >> 1;
                    let src = platform::read_u32(record + 0x50) as usize;
                    let gain = platform::read_u8(record + 0x5D) as i8 as i32;
                    let to_copy = if samples < 0x29 { samples } else { 0x28 };
                    let mut i = 0usize;
                    while i < to_copy as usize {
                        let value = platform::read_u16(src + i * 2) as i16 as i32;
                        scaled[i] = (value * gain / 10) as i16;
                        i += 1;
                    }
                    if samples < 0x29 {
                        let mut i = to_copy as usize;
                        while i < 0x28 {
                            scaled[i] = 0;
                            i += 1;
                        }
                    }
                    let mut output = [0u32; 6];
                    let clear = (platform::read_u8(record + 0x5C) == 0) as u32;
                    platform::call5(
                        FN_80227EF0,
                        p32(record + 0x30),
                        clear,
                        p32(scaled.as_mut_ptr() as usize),
                        0x28,
                        p32(output.as_mut_ptr() as usize),
                    );
                    platform::call3(
                        FN_801E3680,
                        player,
                        p32(output.as_mut_ptr() as usize),
                        0x14,
                    );
                    platform::write_u8(record + 0x5C, 0);
                    platform::write_u8(record + 0x5E, 0);
                    let pointer = platform::read_u32(record + 0x50).wrapping_add(0x50);
                    platform::write_u32(record + 0x50, pointer);
                    let remaining = platform::read_i32(record + 0x54) - 0x50;
                    platform::write_i32(record + 0x54, remaining);
                    if remaining < 1 {
                        platform::write_u32(record + 0x58, 0xFFFF_FFFF);
                        platform::write_u32(record + 0x50, 0);
                    }
                }
                platform::call1(FN_801B1180, lock);
            }
            player += 1;
            record += 0x68;
            if player >= 4 {
                break;
            }
        }
    }
}

/// Constructs the four-record home-menu widget group.
///
/// Installs the pointer table at `+0x1F0`, publishes the object at
/// `0x80377F98`, opens the archive handle `sound` into the `+0x1D0` block when
/// it is non-null (`+0x1EC` becomes the success byte), initialises the timer
/// at `+0x1A0`, then resets the four 0x68-byte records: `+0x50 = 0`,
/// `+0x58 = -1`, `+0x5C = 1`, `+0x62 = 1`.
///
/// # Original
/// - Ghidra: `FUN_80173cd0` at `0x80173CD0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173cd0_FUN_80173cd0.c`
/// - Removed: the `bl 0x802DCDBC` (`__restgprlr_26`) epilogue after the record
///   loop; it only restores the saved registers.
/// - observed: the only export caller passes `(manager, archive)` and keeps
///   the two return registers as `r3`/`r4`.
///
/// # Arguments
/// - `this`: manager object (records at `+0`, timer at `+0x1A0`).
/// - `sound`: archive pointer; `0` leaves the archive-ready byte clear.
///
/// # Side effects
/// Writes `+0x1F0`, `0x80377F98`, the `+0x1D0` archive block, `+0x1EC`, the
/// timer block and four record groups.
///
/// # Safety
/// `this` must be at least `0x1F4` bytes and writable; `sound` must be a valid
/// archive block when non-null.
///
/// # Confidence
/// medium - every offset and store is literal; the argument split comes from
/// the caller (incoming `r3`/`r4`).
pub unsafe fn fn_80173cd0(this: usize, sound: usize) {
    platform::write_u32(this + 0x1F0, PTR_80329F38 as u32);
    platform::write_u32(DAT_80377F98, p32(this));

    if sound == 0 {
        platform::write_u8(this + 0x1EC, 0);
    } else {
        let ready = platform::call2(FN_8019F660, p32(sound), p32(this + 0x1D0));
        platform::write_u8(this + 0x1EC, (ready != 0) as u8);
    }

    platform::call1(FN_801AB2E0, p32(this + 0x1A0));

    let mut record = this;
    let mut i = 0;
    while i < 4 {
        platform::call1(FN_801AB2E0, p32(record));
        platform::write_u32(record + 0x50, 0);
        platform::write_u32(record + 0x58, 0xFFFF_FFFF);
        platform::write_u8(record + 0x5C, 1);
        platform::write_u8(record + 0x62, 1);
        record += 0x68;
        i += 1;
    }
}

/// Arms the scrolling-text timer when the widget group's archive is ready.
///
/// When `+0x1EC` is set, initialises the timer at `+0x1A0`, reads the 64-bit
/// OS time, arms it for `(bus / 500000) * 0x65B9AB / 8000` ticks with callback
/// `0x80173A30`, then resets the four records exactly as
/// [`fn_80173cd0`] does.
///
/// # Original
/// - Ghidra: `FUN_80173e40` at `0x80173E40` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173e40_FUN_80173e40.c`
/// - Removed: the `bl 0x802DCDC0` (`__restgprlr_27`) epilogue; it only
///   restores the saved registers.
/// - observed: the only caller passes the manager pointer in `r3`.
///
/// # Arguments
/// - `this`: manager object built by [`fn_80173cd0`].
///
/// # Side effects
/// Re-initialises the timer and the four records; no store when `+0x1EC` is
/// clear.
///
/// # Safety
/// `this` must be a live manager and `+0x1EC` readable.
///
/// # Confidence
/// medium - the guard, 64-bit time split, period arithmetic and loop are
/// literal; the manager role is inferred from the caller.
pub unsafe fn fn_80173e40(this: usize) {
    if platform::read_u8(this + 0x1EC) != 0 {
        platform::call1(FN_801AB2E0, p32(this + 0x1A0));

        let now = platform::call_u64(FN_801B5F80);
        let delay = platform::read_u32(OS_BUS_CLOCK) / 500_000;
        platform::call7(
            FN_801AB5B0,
            p32(this + 0x1A0),
            delay,
            (now >> 32) as u32,
            now as u32,
            0,
            delay.wrapping_mul(0x65B9AB) / 8000,
            fn_80173a30 as *const () as usize as u32,
        );

        let mut record = this;
        let mut i = 0;
        while i < 4 {
            platform::call1(FN_801AB2E0, p32(record));
            platform::write_u32(record + 0x50, 0);
            platform::write_u32(record + 0x58, 0xFFFF_FFFF);
            platform::write_u8(record + 0x5C, 1);
            platform::write_u8(record + 0x62, 1);
            record += 0x68;
            i += 1;
        }
    }
}

/// Allocates one home-menu object through the global viewport allocator.
///
/// The allocator `0x801D5540` receives the viewport pointer stored at
/// `0x8032A304` and the requested size in bytes.
///
/// # Original
/// - Ghidra: `FUN_80174df0` at `0x80174DF0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174df0_FUN_80174df0.c`
///
/// # Arguments
/// - `size`: byte count passed through unchanged.
///
/// # Returns
/// The allocator's pointer (zero when allocation fails).
///
/// # Side effects
/// Allocates target memory.
///
/// # Safety
/// The viewport at `0x8032A304` must be live.
///
/// # Confidence
/// high - both operands are literal.
pub unsafe fn fn_80174df0(size: u32) -> u32 {
    platform::call2(
        FN_801D5540,
        platform::read_u32(DAT_8032A304),
        size,
    )
}

/// Refreshes the battery gauge of the live menu.
///
/// The live menu is fetched first; the value stays in `r3` and is the argument
/// of [`crate::ui::hud::fn_801788c0`], which Ghidra printed without an operand.
///
/// # Original
/// - Ghidra: `FUN_80174fa0` at `0x80174FA0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174fa0_FUN_80174fa0.c`
/// - inferred: the menu pointer is forwarded, matching the neighbouring
///   wrappers `0x80174FD0` and `0x801750A0`.
///
/// # Side effects
/// Recomputes the gauge quads of the menu.
///
/// # Safety
/// The live menu must be non-null.
///
/// # Confidence
/// medium - both calls are literal; the forwarded operand is inferred.
pub unsafe fn fn_80174fa0() {
    let menu = fn_80175500() as usize;
    crate::ui::hud::fn_801788c0(menu);
}

/// Starts the dialog dismiss transition of the live menu.
///
/// # Original
/// - Ghidra: `FUN_80175010` at `0x80175010` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175010_FUN_80175010.c`
/// - inferred: the menu pointer is forwarded, as in [`fn_80174fa0`].
///
/// # Side effects
/// Calls [`dialog_begin_dismiss`] on the live menu.
///
/// # Safety
/// The live menu must be non-null.
///
/// # Confidence
/// medium - both calls are literal; the forwarded operand is inferred.
pub unsafe fn fn_80175010() {
    let menu = fn_80175500() as usize;
    dialog_begin_dismiss(menu);
}

/// Resets the four player animations of the live menu.
///
/// # Original
/// - Ghidra: `FUN_80175140` at `0x80175140` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175140_FUN_80175140.c`
/// - inferred: the menu pointer is forwarded, as in [`fn_80174fa0`].
///
/// # Side effects
/// Calls [`menu_reset_player_animations`] on the live menu.
///
/// # Safety
/// The live menu must be non-null.
///
/// # Confidence
/// medium - both calls are literal; the forwarded operand is inferred.
pub unsafe fn fn_80175140() {
    let menu = fn_80175500() as usize;
    menu_reset_player_animations(menu);
}

/// Resets the GX state used by the home-button menu text/quad pass.
///
/// The body is a straight sequence of 25 state-setter calls (no branches).
/// Two of them receive a 56-byte stack block: `0x801C04D0` fills it and
/// `0x801C9940` applies it.
///
/// # Original
/// - Ghidra: `FUN_80175170` at `0x80175170` (440 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175170_FUN_80175170.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Writes GX render state through the `0x801Cxxxx` setters.
///
/// # Safety
/// The GX engine must be initialised; the stack block is host-local.
///
/// # Confidence
/// high - every call and every operand is literal.
pub unsafe fn fn_80175170() {
    let mut block = [0u32; 14];
    platform::call1(FN_801C5860, 0);
    platform::call1(FN_801C04D0, p32(block.as_mut_ptr() as usize));
    platform::call2(FN_801C9940, p32(block.as_mut_ptr() as usize), 0);
    platform::call1(FN_801C9A60, 0);
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call5(FN_801C40B0, 0, 9, 0, 4, 0);
    platform::call1(FN_801C6AC0, 1);
    platform::call6(FN_801C6AF0, 4, 0, 0, 0, 0, 2);
    platform::call6(FN_801C6AF0, 5, 0, 0, 0, 0, 2);
    platform::call1(FN_801C4A20, 0);
    platform::call1(FN_801C88F0, 1);
    platform::call4(FN_801C8790, 0, 0xFF, 0xFF, 0xFF);
    platform::call5(FN_801C82F0, 0, 0xF, 0xF, 0xF, 2);
    platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
    platform::call5(FN_801C8330, 0, 7, 7, 7, 1);
    platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
    platform::call5(FN_801C86C0, 7, 0, 1, 7, 0);
    platform::call4(FN_801C8DC0, 1, 4, 5, 5);
    platform::call1(FN_801C8E40, 0);
    platform::call3(FN_801C8E70, 0, 7, 0);
    platform::call1(FN_801C6130, 0);
    platform::call1(FN_801C8110, 0);
    platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    platform::call3(FN_801C8600, 0, 0, 0);
}

/// Feeds one menu widget's scrolling text or arms its retry timer.
///
/// The word returned by `0x801ABA80` carries the player index in its low half
/// and the controller channel in its high half. When that channel is
/// connected and the widget's active record is enabled (`0x80173860` returns
/// the `+0x62` test from `0x80174330`), the value's low half is forwarded to
/// `0x80173510` and the body stops. Otherwise the per-channel timer at
/// `menu + 0x528 + channel*0x30` is armed for `(bus/4000) * 50` ticks with
/// [`fn_80178a20`] itself as the callback.
///
/// # Original
/// - Ghidra: `FUN_80178a20` at `0x80178A20` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80178a20_FUN_80178a20.c`
/// - observed: `0x80173860` is declared `void` but leaves the `0x80174330`
///   result in `r3`, which is what this caller tests.
///
/// # Arguments
/// None; the value comes from `0x801ABA80` and the menu from `0x80377FB8`.
///
/// # Side effects
/// May start a widget sequence or arm a repeating timer.
///
/// # Safety
/// `0x80377FB8` must point at the live menu with its four `+0x2AC` widgets.
///
/// # Confidence
/// medium - the split of the query word and the timer arguments are literal;
/// the "player/channel" naming is inferred from the shifts.
pub unsafe fn fn_80178a20() {
    let value = call0_r(FN_801ABA80);
    let menu = platform::read_u32(DAT_80377FB8) as usize;
    let channel = value >> 16;

    if platform::call1(FN_801E1DD0, channel) != 0 {
        let widget = platform::read_u32(menu + 0x2AC + channel as usize * 4) as usize;
        if platform::call1(FN_80173860, p32(widget)) != 0 {
            fn_80173510(widget, value & 0xFFFF);
            return;
        }
    }

    let timer = menu + 0x528 + channel as usize * 0x30;
    platform::call2(
        FN_801ABA70,
        p32(timer),
        (channel << 16) | (channel + 2),
    );
    platform::call1(FN_801AB640, p32(timer));
    platform::call5(
        FN_801AB540,
        p32(timer),
        0x1062_4DD3,
        0,
        (platform::read_u32(OS_BUS_CLOCK) / 4000) * 0x32,
        fn_80178a20 as *const () as usize as u32,
    );
}

/// Polls the controller release query and rearms the menu retry timer.
///
/// When `0x801ABA80` returns `0`, the immediate query `0x801DFAC0` is used and
/// success sets the menu byte `+0x91`; otherwise the deferred query
/// `0x801DFAD0` is used. If neither reports a release, the timer at
/// `menu + 0x5E8` is restarted for `(bus/4000) * 100` ticks with this function
/// as the callback.
///
/// # Original
/// - Ghidra: `FUN_80178b40` at `0x80178B40` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80178b40_FUN_80178b40.c`
///
/// # Arguments
/// None; the selector comes from `0x801ABA80`.
///
/// # Side effects
/// May set the menu byte `+0x91`; otherwise rewrites the `+0x5E8` timer.
///
/// # Safety
/// `0x80377FB8` must point at the live menu.
///
/// # Confidence
/// medium - every call and argument is literal; the thunks' `r3` return is
/// consumed as the release flag, matching the export.
pub unsafe fn fn_80178b40() {
    let menu = platform::read_u32(DAT_80377FB8) as usize;
    let value = call0_r(FN_801ABA80);
    let mut released = false;

    if value == 0 {
        if call0_r(FN_801DFAC0) != 0 {
            platform::write_u8(menu + 0x91, 1);
            released = true;
        }
    } else if call0_r(FN_801DFAD0) != 0 {
        released = true;
    }

    if !released {
        platform::call1(FN_801AB640, p32(menu + 0x5E8));
        platform::call2(FN_801ABA70, p32(menu + 0x5E8), value);
        platform::call5(
            FN_801AB540,
            p32(menu + 0x5E8),
            0x1062_4DD3,
            0,
            (platform::read_u32(OS_BUS_CLOCK) / 4000) * 100,
            FN_80178B40 as u32,
        );
    }
}

/// Deletes one 16-byte dialog record when the delete flag is positive.
///
/// This is the element destructor installed through the placement helpers
/// `0x802DC9A0`/`0x802DCA98` at `0x80180780`, `0x801846A0`, `0x80184C60`,
/// `0x801860B0` and `0x801862C0`. The body is identical to [`fn_80172330`].
///
/// # Original
/// - Ghidra: `FUN_8017e490` at `0x8017E490` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017e490_FUN_8017e490.c`
///
/// # Arguments
/// - `this`: record pointer; may be null.
/// - `flag`: when greater than zero, the record is released.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May release the record through `0x802B9280`.
///
/// # Safety
/// A non-null `this` must be a live 16-byte record.
///
/// # Confidence
/// high - the guards are literal; the operand follows the twin bodies.
pub unsafe fn fn_8017e490(this: usize, flag: i32) -> usize {
    if this != 0 && flag > 0 {
        platform::call1(FN_802B9280, p32(this));
    }
    this
}

/// Draws the dialog text quad with the channel's scale value.
///
/// The body resets the GX text state through nineteen setters, copies the
/// first word of `param_3` into the `0x801C69E0(4, ...)` array, selects state
/// `0xB0`, and finally writes the word at `param_1 + 4` to the FIFO. Every
/// call and every immediate is printed by the export.
///
/// # Original
/// - Ghidra: `FUN_8017f0b0` at `0x8017F0B0` (408 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017f0b0_FUN_8017f0b0.c`
///
/// # Arguments
/// - `param_1`: draw object; its `+4` word is the final FIFO value.
/// - `param_2`: unused by the recovered body (kept to preserve the signature).
/// - `param_3`: pointer whose first word is loaded into the `0x801C69E0`
///   array.
///
/// # Side effects
/// Rewrites GX render state and pushes floats to `0xCC008000`.
///
/// # Safety
/// `param_1 + 4` and `param_3` must be readable; the GX engine must be ready.
///
/// # Confidence
/// high - the call sequence and all operands are literal; `param_2` is
/// unused in the export, and the FIFO write is the documented MMIO address.
pub unsafe fn fn_8017f0b0(param_1: usize, param_2: u32, param_3: usize) {
    let _ = param_2;
    let mut local_18 = [0u32; 3];
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call5(FN_801C40B0, 0, 9, 0, 4, 0);
    platform::call1(FN_801C6AC0, 1);
    platform::call6(FN_801C6AF0, 4, 0, 0, 0, 0, 2);
    local_18[0] = platform::read_u32(param_3);
    platform::call2(FN_801C69E0, 4, p32(local_18.as_mut_ptr() as usize));
    platform::call1(FN_801C4A20, 0);
    platform::call1(FN_801C88F0, 1);
    platform::call1(FN_801C8110, 0);
    platform::call4(FN_801C8790, 0, 0xFF, 0xFF, 4);
    platform::call2(FN_801C8250, 0, 4);
    platform::call1(FN_801C8130, 0);
    platform::call3(FN_801C8600, 0, 0, 0);
    platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    platform::call5(FN_801C86C0, 7, 0, 0, 7, 0);
    platform::call4(FN_801C8DC0, 1, 4, 5, 0xF);
    platform::call2(FN_801C57A0, 6, 0);
    platform::call3(FN_801C5570, 0xB0, 0, 5);
    platform::write_u32(0xCC00_8000, platform::read_u32(param_1 + 4));
}

// ===========================================================================
// Wave 32: GX quad state and dialog-tree tagging (`0x80171680`, `0x8017F7A0`)
// ===========================================================================

/// `0x801C0BD0`: builds a translation matrix from three `f64` values.
///
/// observed: the same three-double ABI used by `0x8016C9E0` in
/// `crate::ui::lyt`; the fourth operand is the 48-byte matrix scratch block.
const FN_801C0BD0: usize = 0x801C_0BD0;

/// `0x80378020`: global sentinel whose presence marks a dialog-tree object.
///
/// observed: [`fn_8017f7a0`] compares every list node against this address.
const MARKER_80378020: usize = 0x8037_8020;

/// Calls a three-`f64`, one-pointer function on the target.
///
/// # Original
/// Host-side helper; no binary address. Models the `f1`, `f2`, `f3`, `r3` ABI
/// of [`FN_801C0BD0`].
///
/// # Arguments
/// - `addr`: target function address.
/// - `a`, `b`, `c`: the three `f64` arguments.
/// - `dst`: the pointer argument passed in `r3`.
///
/// # Returns
/// The target's `r3` result.
///
/// # Safety
/// `addr` must point to a function with this exact ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call_f64x3_u32(addr: usize, a: f64, b: f64, c: f64, dst: u32) -> u32 {
    let f: extern "C" fn(f64, f64, f64, u32) -> u32 = core::mem::transmute(addr);
    f(a, b, c, dst)
}

/// Sets the GX state for one home-menu quad and pushes its colour word.
///
/// The body first runs the fixed `0x801Cxxxx` state sequence (TEV stages 9 and
/// `0xB`, texture 0, channels 4/5, blend mode 1, four texture/colour arrays and
/// the viewport), builds an identity translation with the three equal doubles
/// from `0x802F5A58`, uploads that matrix, selects texture state `0xA8`, and
/// finally writes the word at `param_2` to the FIFO.
///
/// # Original
/// - Ghidra: `FUN_80171680` at `0x80171680` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80171680_FUN_80171680.c`
///
/// # Arguments
/// - `param_1`: draw object/texture identity passed to `0x801C57A0`.
/// - `param_2`: address of the word pushed to the FIFO (a packed colour).
///
/// # Side effects
/// Rewrites GX render state, uploads a 48-byte matrix and writes
/// `0xCC008000`.
///
/// # Safety
/// `param_2` must be readable and the GX engine ready; `param_1` must be a
/// valid texture/draw identity for `0x801C57A0`.
///
/// # Confidence
/// high - every call and every immediate is literal; the `ldexpf` blocks in
/// the export are compiler paired-single restore code, not control flow.
pub unsafe fn fn_80171680(param_1: u32, param_2: usize) {
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call2(FN_801C37C0, 0xB, 1);
    platform::call5(FN_801C40B0, 0, 9, 1, 4, 0);
    platform::call5(FN_801C40B0, 0, 0xB, 1, 5, 0);
    platform::call1(FN_801C5860, 0);
    platform::call1(FN_801C6AC0, 1);
    platform::call7(FN_801C6AF0, 4, 0, 1, 1, 0, 0, 2);
    platform::call1(FN_801C4A20, 0);
    platform::call1(FN_801C88F0, 1);
    platform::call2(FN_801C8250, 0, 4);
    platform::call4(FN_801C8790, 0, 0xFF, 0xFF, 4);
    platform::call4(FN_801C8DC0, 0, 0, 0, 5);

    let mut matrix = [0u32; 12];
    let scale = read_f32(0x802F_5A58) as f64;
    let _ = call_f64x3_u32(
        FN_801C0BD0,
        scale,
        scale,
        scale,
        matrix.as_mut_ptr() as u32,
    );
    platform::call2(FN_801C9940, matrix.as_ptr() as u32, 0);
    platform::call2(FN_801C57A0, param_1, 0);
    platform::call3(FN_801C5570, 0xA8, 0, 2);

    platform::write_u32(0xCC00_8000, platform::read_u32(param_2));
}

/// Returns whether the object's vtable slot `+0xC` list contains the global
/// sentinel `0x80378020`.
///
/// # Original
/// - Ghidra: inline of the three list walks in `FUN_8017f7a0`
/// - Evidence: `work/ghidra/main-gb/export/8017f7a0_FUN_8017f7a0.c`
///
/// # Arguments
/// - `object`: object whose vtable is read at `+0` and whose getter is slot
///   `+0xC`.
///
/// # Returns
/// `true` when the returned list reaches `0x80378020`; otherwise `false`.
///
/// # Safety
/// `object` must have a readable vtable and the getter must return a
/// terminated intrusive list.
///
/// # Confidence
/// medium - the walk is literal; the sentinel's role is inferred from the
/// equality test.
unsafe fn list_contains_marker(object: usize) -> bool {
    let get_list = vtable_method(object, 0, 0xC);
    let mut cursor = platform::call1(get_list, p32(object)) as usize;
    while cursor != 0 {
        if cursor == MARKER_80378020 {
            return true;
        }
        cursor = platform::read_u32(cursor) as usize;
    }
    false
}

/// Recursively tags the dialog list tree with one value.
///
/// The node itself is tagged at `+0xF4` when its own `+0xC` vtable list
/// contains `0x80378020`. The body then walks the `+0x14` list; each element
/// object (link minus one word) is tagged when its list contains the sentinel,
/// each element's `+0x10` list is tagged the same way, and every element of
/// that `+0x10` list is passed back to this function. The walk therefore
/// covers three nesting levels per call and recurses for the deepest one.
///
/// # Original
/// - Ghidra: `FUN_8017f7a0` at `0x8017F7A0` (416 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017f7a0_FUN_8017f7a0.c`
/// - observed: the entry calls `0x802DCD68`; the preserved `r3`/`r4` pair is
///   the two arguments. The recursive call `FUN_8017f7a0(link - 1, value)`
///   confirms the order (`r3` node, `r4` value).
///
/// # Arguments
/// - `node`: root object with a vtable at `+0` and an intrusive list at
///   `+0x14`.
/// - `value`: word stored at `+0xF4` of every tagged object.
///
/// # Side effects
/// Writes `+0xF4` of the root and of matching objects, and recurses through
/// the `+0x10` list of each `+0x14` element.
///
/// # Safety
/// Every reached object must have a readable vtable and its list links must be
/// valid; `+0xF4` must be writable on every tagged object.
///
/// # Confidence
/// medium - the list arithmetic and the recursive shape are literal; the
/// meaning of the marker and of the `+0xF4` tag word is inferred.
pub unsafe fn fn_8017f7a0(node: usize, value: u32) {
    // observed 0x8017F7BC: the save thunk preserves incoming r3/r4; the
    // recursive call below proves `node` is r3 and `value` is r4.
    if list_contains_marker(node) {
        platform::write_u32(node + 0xF4, value);
    }

    let mut outer = platform::read_u32(node + 0x14) as usize;
    let outer_sentinel = node + 0x14;
    while outer != outer_sentinel {
        let object = outer - 4;
        if list_contains_marker(object) {
            platform::write_u32(object + 0xF4, value);
        }

        let mut child = platform::read_u32(outer + 0x10) as usize;
        let child_sentinel = outer + 0x10;
        while child != child_sentinel {
            let child_object = child - 4;
            if list_contains_marker(child_object) {
                platform::write_u32(child_object + 0xF4, value);
            }

            let mut grand = platform::read_u32(child + 0x10) as usize;
            let grand_sentinel = child + 0x10;
            while grand != grand_sentinel {
                fn_8017f7a0(grand - 4, value);
                grand = platform::read_u32(grand) as usize;
            }

            child = platform::read_u32(child) as usize;
        }

        outer = platform::read_u32(outer) as usize;
    }
}

// ===========================================================================
// Wave 102: the last unfilled bodies of the home-menu band
// ===========================================================================

// Callees and data outside this file, addressed exactly as the export prints
// them. `unknown:` marks the ones the export does not name; the comment gives
// only the role the call sites prove.

/// `0x801D49A0`: lock the allocator reached through `(this)`.
const FN_801D49A0: usize = 0x801D_49A0;
/// `0x801D4460`: allocate `size` bytes with the third argument as its mode.
const FN_801D4460: usize = 0x801D_4460;
/// `0x800046B4`: `memset(dst, 0, len)`.
const FN_800046B4: usize = 0x8000_46B4;
/// `0x80004364`: `memcpy(dst, src, len)`.
const FN_80004364: usize = 0x8000_4364;
/// `0x80228DF0`: append one node to the embedded list at `owner + 8`.
const FN_80228DF0: usize = 0x8022_8DF0;
/// `0x801C14C0`: length of a 3-float vector; the result arrives in `f1`.
const FN_801C14C0: usize = 0x801C_14C0;
/// `0x8016DC00`: base constructor of the object that owns `+0x14`.
const FN_8016DC00: usize = 0x8016_DC00;
/// `0x801DD1D0`: returns the current layout mode in the low byte of `r3`.
const FN_801DD1D0: usize = 0x801D_D1D0;
/// `0x801C07A0`: fill a 48-byte matrix from a 0x84-byte layout transform.
const FN_801C07A0: usize = 0x801C_07A0;
/// `0x801C1160`: project a 3-float point through that matrix.
const FN_801C1160: usize = 0x801C_1160;
/// `0x80183960`: build the six-float screen bounds of one pane.
const FN_80183960: usize = 0x8018_3960;
/// `0x80191250`: store its argument in `0x803780E8`.
const FN_80191250: usize = 0x8019_1250;
/// `0x80183500`: bind one pane record to a menu object.
const FN_80183500: usize = 0x8018_3500;
/// `0x80182060`: array written by the `RLTS` animation tags.
const FN_80182060: usize = 0x8018_2060;
/// `0x801820A0`: array written by the `RLIM` animation tags.
const FN_801820A0: usize = 0x8018_20A0;
/// `0x801822F0`: apply one `RLTP` texture-pattern entry.
const FN_801822F0: usize = 0x8018_22F0;
/// `0x801823E0`: apply one `RLMC` material entry.
const FN_801823E0: usize = 0x8018_23E0;
/// `0x80377DF8`: word cleared before the dialog event is dispatched.
const DAT_80377DF8: usize = 0x8037_7DF8;
/// `0x80329C28`: vtable installed by [`fn_80170c50`].
const VT_80329C28: u32 = 0x8032_9C28;
/// `0x802F5A58`: third component fed to the projection in [`fn_80172910`].
const G_802F5A58: usize = 0x802F_5A58;
/// `0x802F5E2C`: offset added to every `RLMC` key value.
const F_802F5E2C: usize = 0x802F_5E2C;
/// `0xCC008000`: the GX write-gather FIFO port.
const GX_FIFO: usize = 0xCC00_8000;

/// `r2 - 0x532C`: magnitude threshold used by [`fn_80170980`].
const SDA2_MAGNITUDE_LIMIT: usize = SDA2_BASE - 0x532C;
/// `r2 - 0x5328`: numerator divided by the magnitude in [`fn_80170980`].
const SDA2_SCALE_NUMERATOR: usize = SDA2_BASE - 0x5328;
/// `r2 - 0x5320`: layout mode `1`'s default stored by [`fn_80170c50`].
const SDA2_MODE_ONE_VALUE: usize = SDA2_BASE - 0x5320;
/// `r2 - 0x531C`: the other layout mode's default stored by [`fn_80170c50`].
const SDA2_MODE_OTHER_VALUE: usize = SDA2_BASE - 0x531C;

/// Calls a function `(u32) -> f32` and keeps the `f1` result.
///
/// # Original
/// Host-side helper; no binary address. Models the `bl` + `fmr` pair used when
/// the callee returns a float in `f1` (`0x801C14C0`).
///
/// # Safety
/// `addr` must point to a function with this exact ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call_f32_ptr_ret(addr: usize, argument: u32) -> f32 {
    let f: extern "C" fn(u32) -> f32 = core::mem::transmute(addr);
    f(argument)
}

/// Allocates and fills one home-menu record.
///
/// The handle at `+0` is locked, a block of `size + 0x20` bytes is requested,
/// and the record is filled from the arguments: two halfwords, the mode, the
/// payload size, two words, the float, one byte, then `size` bytes cleared and
/// copied at `+0x1E`. The finished record is appended to the list at `+8`.
///
/// # Original
/// - Ghidra: `FUN_801704d0` at `0x801704D0` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801704d0_FUN_801704d0.c`
/// - observed: the float argument arrives in `f1` and is stored at `+0x10`.
/// - inferred: the allocator's third argument is the export's
///   `0xFFFFFFFC`/`4` pair, selected by `mode == 1`. The trailing
///   `0x802DCDA8` register-restore helper is omitted, as in the rest of this
///   module.
///
/// # Arguments
/// - `this`: owner whose `+0` word is the allocator handle.
/// - `first`, `second`: stored at `+0` and `+2`.
/// - `arg4`, `arg5`: stored at `+8` and `+0xC`.
/// - `byte`: stored at `+0x1C`.
/// - `mode`: `1` selects the `4` allocator mode, anything else `-4`.
/// - `data`, `size`: payload copied to `+0x1E`.
/// - `value`: stored at `+0x10`.
///
/// # Side effects
/// Locks and unlocks the allocator, allocates, writes the record and links it.
///
/// # Safety
/// `this` must hold a live allocator handle; `data..data+size` readable.
///
/// # Confidence
/// medium - every store is literal; the allocator mode and the `+0x1E`
/// payload are inferred from the two library calls.
pub unsafe fn fn_801704d0(
    this: usize,
    first: u16,
    second: u16,
    arg4: u32,
    arg5: u32,
    byte: u8,
    mode: i32,
    data: u32,
    size: u32,
    value: f32,
) {
    let handle = platform::read_u32(this);
    platform::call2(FN_801D49A0, handle, 0);
    let delta = if mode == 1 { 4 } else { 0xFFFF_FFFC };
    let record = platform::call3(FN_801D4460, handle, size.wrapping_add(0x20), delta);
    if record != 0 {
        let record = record as usize;
        platform::write_u16(record, first);
        platform::write_u16(record + 2, second);
        platform::write_u16(record + 4, mode as u16);
        platform::write_u16(record + 6, size as u16);
        platform::write_u32(record + 8, arg4);
        platform::write_u32(record + 0xC, arg5);
        platform::write_f32(record + 0x10, value);
        platform::write_u8(record + 0x1C, byte);
        platform::call3(FN_800046B4, p32(record + 0x1E), 0, size);
        platform::call3(FN_80004364, p32(record + 0x1E), data, size);
        platform::call2(FN_80228DF0, p32(this + 8), p32(record));
    }
}

/// Sets the cell size of a home-menu grid to 8x8 and stores a value.
///
/// The export prints the dimension call with no arguments because the callee's
/// registers were already loaded; the register trace loads `r6 = r7 = 8` before
/// the branch, and the fourth argument is stored at `+0x24` afterwards.
///
/// # Original
/// - Ghidra: `FUN_801707e0` at `0x801707E0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801707e0_FUN_801707e0.c`
///
/// # Arguments
/// - `this`: grid descriptor.
/// - `width`, `height`: grid extent in pixels.
/// - `value`: stored at `+0x24`.
///
/// # Side effects
/// Writes the eight dimension words through [`fn_80170670`] plus `+0x24`.
///
/// # Safety
/// `this` must be writable for `0x28` bytes.
///
/// # Confidence
/// medium - the 8x8 literals come from the call-site registers; every store
/// after the call is literal.
pub unsafe fn fn_801707e0(this: usize, width: u32, height: u32, value: u32) {
    fn_80170670(this, width, height, 8, 8);
    platform::write_u32(this + 0x24, value);
}

/// Normalises a 3-float vector when it is longer than a threshold.
///
/// The magnitude comes from `0x801C14C0`. When `|magnitude|` exceeds the
/// `r2-0x532C` constant, the vector is multiplied by
/// `(r2-0x5328 constant) / magnitude`.
///
/// # Original
/// - Ghidra: `FUN_80170980` at `0x80170980` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170980_FUN_80170980.c`
///
/// # Arguments
/// - `vector`: three consecutive `f32` values.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May overwrite `vector[0..3]`.
///
/// # Safety
/// `vector` must be readable and writable for 12 bytes and `0x801C14C0` must
/// return a float.
///
/// # Confidence
/// medium - the comparison and scaling are literal; `r2` is mapped to the
/// documented `SDA2_BASE`, and the two words there are unnamed constants.
pub unsafe fn fn_80170980(vector: usize) {
    let magnitude = call_f32_ptr_ret(FN_801C14C0, p32(vector)) as f64;
    if read_f32(SDA2_MAGNITUDE_LIMIT) < (magnitude.abs() as f32) {
        let scale = (read_f32(SDA2_SCALE_NUMERATOR) as f64 / magnitude) as f32;
        platform::write_f32(vector, read_f32(vector) * scale);
        platform::write_f32(vector + 4, read_f32(vector + 4) * scale);
        platform::write_f32(vector + 8, read_f32(vector + 8) * scale);
    }
}

/// Constructs the object that holds the dialog pane list at `+0x14`.
///
/// Runs the base constructor, installs vtable `0x80329C28`, clears the three
/// words at `+0x4C..+0x54` around the embedded list, and stores one of two
/// mode-dependent defaults in `+0x48`.
///
/// # Original
/// - Ghidra: `FUN_80170c50` at `0x80170C50` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80170c50_FUN_80170c50.c`
///
/// # Arguments
/// - `this`: storage for the object.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes the vtable and the five words listed above.
///
/// # Safety
/// `this` must be writable for `0x58` bytes.
///
/// # Confidence
/// medium - the stores are literal; the two `r2-relative` defaults are unnamed
/// constants and the mode query's meaning is inferred.
pub unsafe fn fn_80170c50(this: usize) -> usize {
    platform::call1(FN_8016DC00, p32(this));
    platform::write_u32(this, VT_80329C28);
    platform::write_u32(this + 0x4C, 0);
    platform::call1(FN_801C04D0, p32(this + 0x14));
    platform::write_u32(this + 0x50, 0);
    platform::write_u32(this + 0x54, 0);

    let mode = call0_r(FN_801DD1D0) & 0xFF;
    let source = if mode == 1 {
        SDA2_MODE_ONE_VALUE
    } else {
        SDA2_MODE_OTHER_VALUE
    };
    platform::write_f32(this + 0x48, read_f32(source));
    this
}

/// Clears a dialog event word and forwards an event payload to the object.
///
/// `0x80377DF8` is cleared; the node at `param_2 + 0x10` is advanced by `0x10`
/// and passed with `this` to virtual slot `+0x5C`.
///
/// # Original
/// - Ghidra: `FUN_80172370` at `0x80172370` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172370_FUN_80172370.c`
/// - observed: the export warns that the indirect jump at `0x80172390` could
///   not be recovered as a jump table and prints it as a call.
///
/// # Arguments
/// - `this`: object whose vtable supplies slot `+0x5C`.
/// - `event`: node whose `+0x10` word is forwarded after `+0x10`.
///
/// # Side effects
/// Clears the global word and calls the virtual method.
///
/// # Safety
/// `this` must have a readable vtable; `event + 0x10` must be readable.
///
/// # Confidence
/// medium - the clear and the payload arithmetic are literal; the slot is a
/// tail call in the original.
pub unsafe fn fn_80172370(this: usize, event: usize) {
    platform::write_u32(DAT_80377DF8, 0);
    let argument = platform::read_u32(event + 0x10).wrapping_add(0x10);
    // inferred: Ghidra did not recover the argument list of the indirect call.
    let method = vtable_method(this, 0, 0x5C);
    platform::call2(method, p32(this), argument);
}

/// Stores one word at `+0x2C` of a menu record.
///
/// # Original
/// - Ghidra: `FUN_80172660` at `0x80172660` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172660_FUN_80172660.c`
///
/// # Arguments
/// - `this`: writable record.
/// - `value`: word stored at `+0x2C`.
///
/// # Side effects
/// One store.
///
/// # Safety
/// `this + 0x2C` must be writable.
///
/// # Confidence
/// high - one `stw`.
pub unsafe fn fn_80172660(this: usize, value: u32) {
    platform::write_u32(this + 0x2C, value);
}

/// Tests whether a point falls inside a pane's projected screen rectangle.
///
/// The object at `+0x28` is asked for its current frame through virtual slot
/// `+0x50`. Its transform at `+0x2C + 0x84` is projected, the point is
/// projected through it, and the six-float bounds from `0x80183960` are
/// compared against the projected `(x, y)`.
///
/// # Original
/// - Ghidra: `FUN_80172910` at `0x80172910` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80172910_FUN_80172910.c`
///
/// # Arguments
/// - `x`, `y`: point in the pane's local space.
/// - `this`: owner; `+0x28` is the pane, `+0x2C` the transform owner.
///
/// # Returns
/// `1` when the point is inside the bounds, `0` otherwise or when the pane is
/// missing.
///
/// # Side effects
/// None; the stack matrices are scratch space.
///
/// # Safety
/// `this`, its pane and the transform must be live.
///
/// # Confidence
/// medium - the four comparisons and the two helper calls are literal; the
/// unused `+0x50` result is treated as a frame selector, as in the export.
pub unsafe fn fn_80172910(x: f64, y: f64, this: usize) -> u32 {
    let object = platform::read_u32(this + 0x28) as usize;
    if object == 0 {
        return 0;
    }

    let frame = platform::call1(vtable_method(object, 0, 0x50), p32(object));
    if frame == 0 {
        return 0;
    }

    let mut matrix = [0u32; 12];
    platform::call2(
        FN_801C07A0,
        platform::read_u32(this + 0x2C).wrapping_add(0x84),
        p32(matrix.as_mut_ptr() as usize),
    );

    let point = [x as f32, y as f32, read_f32(G_802F5A58)];
    let mut projected = [0f32; 4];
    platform::call3(
        FN_801C1160,
        p32(matrix.as_mut_ptr() as usize),
        p32(point.as_ptr() as usize),
        p32(projected.as_mut_ptr() as usize),
    );

    let mut bounds = [0f32; 6];
    platform::call3(
        FN_80183960,
        p32(bounds.as_mut_ptr() as usize),
        platform::read_u32(this + 0x2C),
        frame,
    );

    let outside = projected[0] < bounds[0]
        || bounds[2] < projected[0]
        || projected[1] < bounds[3]
        || bounds[1] < projected[1];
    if outside {
        0
    } else {
        1
    }
}

/// Empty tail of a per-player widget hook.
///
/// # Original
/// - Ghidra: `FUN_801734f0` at `0x801734F0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801734f0_FUN_801734f0.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// None; the four-byte body is the return instruction alone.
///
/// # Safety
/// Nothing to uphold.
///
/// # Confidence
/// high - the export contains no statements and the ELF holds one word.
pub unsafe fn fn_801734f0() {}

/// Stores a double, narrowed to `f32`, at `+4` of a widget record.
///
/// # Original
/// - Ghidra: `FUN_80173500` at `0x80173500` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173500_FUN_80173500.c`
///
/// # Arguments
/// - `value`: `f64` arriving in `f1`.
/// - `this`: widget record; the `f32` is stored at `+4`.
///
/// # Side effects
/// One store.
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - one `stfs` after the `f64` to `f32` conversion.
pub unsafe fn fn_80173500(value: f64, this: usize) {
    platform::write_f32(this + 4, value as f32);
}

/// Returns its argument unchanged.
///
/// The four-byte body is the return instruction; the caller at `0x801795B0`
/// reads the result and dereferences `+0x10`, so the incoming `r3` is returned.
///
/// # Original
/// - Ghidra: `FUN_80173870` at `0x80173870` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80173870_FUN_80173870.c`
///
/// # Arguments
/// - `this`: value returned to the caller.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Nothing to uphold; the pointer is not dereferenced here.
///
/// # Confidence
/// medium - the body is a bare return; the identity shape is inferred from the
/// caller's use of `r3`.
pub unsafe fn fn_80173870(this: usize) -> usize {
    this
}

/// Empty shutdown tail of the per-player widget group.
///
/// # Original
/// - Ghidra: `FUN_80174ae0` at `0x80174AE0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174ae0_FUN_80174ae0.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// None; the four-byte body is the return instruction alone.
///
/// # Safety
/// Nothing to uphold.
///
/// # Confidence
/// high - the export contains no statements.
pub unsafe fn fn_80174ae0() {}

/// Four-byte thunk to `0x80191250`.
///
/// Ghidra follows the branch and prints the target's first store
/// (`0x803780E8 = parameter`) as the thunk body; the branch itself is the whole
/// four-byte function.
///
/// # Original
/// - Ghidra: `thunk_FUN_80191250` at `0x80174DE0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80174de0_thunk_FUN_80191250.c`
///
/// # Arguments
/// - `value`: forwarded in `r3`.
///
/// # Side effects
/// Whatever `0x80191250` does; it stores the argument in `0x803780E8`.
///
/// # Safety
/// The callee's requirements apply.
///
/// # Confidence
/// high - one unconditional branch at the address.
pub unsafe fn fn_80174de0(value: u32) {
    platform::call1(FN_80191250, value);
}

/// Closes the active list entry of the live menu.
///
/// `0x801ABA80` returns the menu manager, which is handed to the already
/// translated [`fn_801738f0`].
///
/// # Original
/// - Ghidra: `FUN_80178b10` at `0x80178B10` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80178b10_FUN_80178b10.c`
///
/// # Arguments
/// None.
///
/// # Side effects
/// Closes one list entry through [`fn_801738f0`].
///
/// # Safety
/// `0x801ABA80` must return a live manager.
///
/// # Confidence
/// high - the two calls are literal and the manager call is the same one
/// [`fn_80173a30`] already uses.
pub unsafe fn fn_80178b10() {
    let manager = call0_r(FN_801ABA80) as usize;
    fn_801738f0(manager);
}

/// Binds the matching pane records of one dialog document.
///
/// The document at `this + 0xC` holds a count at `+0xE` and a table of signed
/// offsets at `+0x10`. Each record whose byte `+0x15` is `1` and whose name
/// matches `target + 4` gets a slot at `this + 0x18`: the owner is stored at
/// `+8`, the record index at `+0xC`, and the record is bound through
/// `0x80183500`.
///
/// # Original
/// - Ghidra: `FUN_8017da20` at `0x8017DA20` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017da20_FUN_8017da20.c`
/// - observed: the second argument of `0x8017E530` is the record pointer; the
///   export omits it but the register trace loads it into `r4`.
///
/// # Arguments
/// - `this`: dialog object; `+0xC` document, `+0x18` slot array.
/// - `target`: object whose name at `+4` is matched.
///
/// # Side effects
/// Writes one `0x10`-byte slot per matching record and calls `0x80183500`.
///
/// # Safety
/// Both objects must be live; the slot array must hold `count` entries.
///
/// # Confidence
/// medium - the loop and all offsets are literal; the second comparison
/// operand comes from the call-site register.
pub unsafe fn fn_8017da20(this: usize, target: usize) {
    let document = platform::read_u32(this + 0xC) as usize;
    let table = document + platform::read_u32(document + 0x10) as usize;
    let count = platform::read_u16(document + 0xE) as usize;

    for index in 0..count {
        let record = document + platform::read_u32(table + index * 4) as usize;
        if platform::read_u8(record + 0x15) != 1 {
            continue;
        }
        if !menu_name_eq20(p32(target + 4), p32(record)) {
            continue;
        }

        let slot = platform::read_u32(this + 0x18) as usize + index * 0x10;
        platform::write_u32(slot + 8, p32(this));
        platform::write_u16(slot + 0xC, index as u16);
        platform::call2(FN_80183500, p32(target), p32(slot));
    }
}

/// Writes one quad's four corners from two 2-float inputs and a colour array.
///
/// The GX FIFO receives four vertices in the order `(a0,a1)`, `(a0+b0,a1)`,
/// `(a0+b0,a1+b1)`, `(a0,a1+b1)`. After each position the colour word selected
/// by the original order (`0`, `1`, `3`, `2`) is pushed when `colour != 0`, and
/// then `count` rows of two floats are pushed from `rows + i*0x20 + offset`
/// with offsets `0`, `8`, `0x10`, `0x18`.
///
/// # Original
/// - Ghidra: `FUN_8017ea10` at `0x8017EA10` (1376 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017ea10_FUN_8017ea10.c`
/// - Removed: the `bl 0x802DCDC0` (`__restgprlr_27`) epilogue; it only
///   restores the saved registers.
/// - observed: the export's empty `param_3` loops are the bookkeeping of the
///   unrolled FIFO writes; the ELF shows each iteration emitting two `stfs`.
///
/// # Arguments
/// - `a`: 2-float corner source.
/// - `b`: 2-float extent added to the corner.
/// - `count`: number of row pairs pushed after every vertex.
/// - `rows`: row array, stride `0x20`.
/// - `colour`: four colour words (memory order `R,G,B,A`), or `0`.
///
/// # Side effects
/// Writes to the GX FIFO at `0xCC008000`.
///
/// # Safety
/// `a`, `b` must be readable for 8 bytes, `rows` for `count * 0x20`, and
/// `colour` for 16 bytes when non-zero.
///
/// # Confidence
/// high - the cornder arithmetic, the colour offsets and the row stride are
/// literal in the unrolled body.
pub unsafe fn fn_8017ea10(a: usize, b: usize, count: i32, rows: usize, colour: usize) {
    platform::call3(FN_801C5570, 0x80, 0, 4);

    let a0 = read_f32(a);
    let a1 = read_f32(a + 4);
    let b0 = read_f32(b);
    let b1 = read_f32(b + 4);

    push_vertex(a0, a1, colour, 0, rows, count);
    push_vertex(a0 + b0, a1, colour, 4, rows, count);
    push_vertex(a0 + b0, a1 + b1, colour, 0xC, rows, count);
    push_vertex(a0, a1 + b1, colour, 8, rows, count);
}

/// Pushes one quad vertex, its colour word and its row pairs to the FIFO.
///
/// # Original
/// Host-side decomposition of the four repeated vertex blocks in
/// `work/ghidra/main-gb/export/8017ea10_FUN_8017ea10.c`; not a separate
/// function in the binary.
///
/// # Arguments
/// - `x`, `y`: vertex position.
/// - `colour`: colour array or `0`.
/// - `colour_offset`: byte offset of the word pushed when a colour exists.
/// - `rows`, `count`: row array and number of `0x20`-byte rows.
///
/// # Safety
/// See [`fn_8017ea10`].
///
/// # Confidence
/// high - each of the four blocks in the export has exactly this shape.
#[inline]
unsafe fn push_vertex(
    x: f32,
    y: f32,
    colour: usize,
    colour_offset: usize,
    rows: usize,
    count: i32,
) {
    platform::write_f32(GX_FIFO, x);
    platform::write_f32(GX_FIFO, y);
    if colour != 0 {
        platform::write_u32(GX_FIFO, platform::read_u32(colour + colour_offset));
    }
    let mut index = 0usize;
    while (index as i32) < count {
        platform::write_f32(GX_FIFO, read_f32(rows + index * 0x20 + colour_offset));
        platform::write_f32(GX_FIFO, read_f32(rows + index * 0x20 + colour_offset + 4));
        index += 1;
    }
}

/// Scales the alpha byte of four colour words and draws the quad.
///
/// Two source words are read per iteration (two iterations), the low byte of
/// each (the alpha byte of a big-endian `0xRRGGBBAA` colour) is multiplied by
/// `scale / 255`, and the resulting sixteen bytes are handed to
/// [`fn_8017ea10`] together with the caller's first four arguments.
///
/// # Original
/// - Ghidra: `FUN_8017ef70` at `0x8017EF70` (308 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017ef70_FUN_8017ef70.c`
/// - observed: `r8 != 0xFF` selects the scaling path; the tail passes
///   `&local_colours` as the fifth argument only when `r7 != 0`.
///
/// # Arguments
/// - `a`, `b`: forwarded unchanged to [`fn_8017ea10`].
/// - `count`: forwarded row count.
/// - `rows`: forwarded row array.
/// - `colours`: two source colour words per iteration, or `0`.
/// - `scale`: `0xFF` copies the colours unchanged.
///
/// # Side effects
/// Writes to the GX FIFO through [`fn_8017ea10`].
///
/// # Safety
/// `colours` must be readable for 16 bytes when non-zero; the forwarded
/// pointers must satisfy [`fn_8017ea10`].
///
/// # Confidence
/// high - the scale expression and the two-word fan-out are literal.
pub unsafe fn fn_8017ef70(
    a: usize,
    b: usize,
    count: u8,
    rows: u32,
    colours: usize,
    scale: u32,
) {
    let mut words = [0xFFFF_FFFFu32; 4];
    if colours != 0 {
        let mut index = 0usize;
        while index < 2 {
            words[index * 2] = scale_alpha(platform::read_u32(colours + index * 8), scale);
            words[index * 2 + 1] =
                scale_alpha(platform::read_u32(colours + index * 8 + 4), scale);
            index += 1;
        }
    }

    let colour_pointer = if colours != 0 {
        words.as_mut_ptr() as usize
    } else {
        0
    };
    fn_8017ea10(a, b, count as i32, rows as usize, colour_pointer);
}

/// Multiplies the alpha byte of one `0xRRGGBBAA` colour word.
///
/// # Original
/// Host-side decomposition of the scale block repeated twice in
/// `work/ghidra/main-gb/export/8017ef70_FUN_8017ef70.c`.
///
/// # Arguments
/// - `word`: colour word as loaded from big-endian memory.
/// - `scale`: `0..=0xFE`; `0xFF` leaves the word unchanged.
///
/// # Returns
/// The word with `(alpha * scale) / 0xFF` in its low byte.
///
/// # Safety
/// Pure arithmetic.
///
/// # Confidence
/// high - the multiply, the `0x80808081` reciprocal and the byte merge are
/// literal.
#[inline]
fn scale_alpha(word: u32, scale: u32) -> u32 {
    if scale == 0xFF {
        word
    } else {
        let alpha = (word & 0xFF).wrapping_mul(scale) / 0xFF;
        (word & 0xFFFF_FF00) | alpha
    }
}

/// Applies one channel's animation tags to a target object.
///
/// `owner + 0xC` selects the animation document, `index` picks one chunk table
/// inside it, and `owner + 0x10` is the current time. Every chunk is matched by
/// its four-byte tag:
///
/// | tag | meaning in the export | effect |
/// |---|---|---|
/// | `RLMC` | material colour | `0x801823E0(target, entry[1], clamp(value))` |
/// | `RLTS` | texture matrix | writes `f32` into the array from `0x80182060` |
/// | `RLTP` | texture pattern | `0x801822F0(target, entry[0], table[index])` |
/// | `RLIM` | indirect material | writes `f32` into the array from `0x801820A0` |
///
/// Each key value comes from [`fn_8017d520`] or
/// [`dialog_keyframe_lookup_u16`] evaluated at `owner + 0x10`.
///
/// # Original
/// - Ghidra: `FUN_8017dcc0` at `0x8017DCC0` (728 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017dcc0_FUN_8017dcc0.c`
/// - observed: the four tag constants are `0x524C4D43`, `0x524C5453`,
///   `0x524C5450` and `0x524C494D`; the export prints the argument order of
///   `0x8017D520` as `(time, key table, key count)`.
///
/// # Arguments
/// - `owner`: animation owner; `+0xC` document, `+0x10` time, `+0x14` texture
///   pattern table.
/// - `index`: chunk-table selector.
/// - `target`: object the values are applied to.
///
/// # Side effects
/// Calls three material/texture setters and writes two `f32` arrays.
///
/// # Safety
/// `owner` and `target` must be live animation objects.
///
/// # Confidence
/// medium - every offset, tag and comparison is literal; the `RLMC` clamp is a
/// saturating `i16` conversion standing in for the export's `psq_st` sequence,
/// and the tag names are inferred from the NW4R brlan chunk layout.
pub unsafe fn fn_8017dcc0(owner: usize, index: u32, target: usize) {
    const TAG_RLMC: u32 = 0x524C_4D43;
    const TAG_RLTS: u32 = 0x524C_5453;
    const TAG_RLTP: u32 = 0x524C_5450;
    const TAG_RLIM: u32 = 0x524C_494D;

    let document = platform::read_u32(owner + 0xC) as usize;
    let list = document + platform::read_u32(document + 0x10) as usize;
    let chunk = list + platform::read_u32(list + index as usize * 4) as usize;

    let time = read_f32(owner + 0x10) as f64;
    let addend = read_f32(F_802F5E2C) as f64;
    let count = platform::read_u8(chunk + 0x14) as usize;

    for chunk_index in 0..count {
        let node = chunk + platform::read_u32(chunk + 0x18 + chunk_index * 4) as usize;
        let tag = platform::read_u32(node);
        let entries = platform::read_u8(node + 4) as usize;

        match tag {
            TAG_RLMC => {
                for entry_index in 0..entries {
                    let entry =
                        node + platform::read_u32(node + 8 + entry_index * 4) as usize;
                    let value = fn_8017d520(
                        time,
                        entry + platform::read_u32(entry + 8) as usize,
                        platform::read_u16(entry + 4) as i32,
                    );
                    // observed: the psq_st pair saturates the value to i16
                    // before the material setter sees it.
                    let clamped = (((value + addend) as f32) as i32).max(-0x400).min(0x3FF);
                    platform::call3(
                        FN_801823E0,
                        p32(target),
                        platform::read_u8(entry + 1) as u32,
                        clamped as u32,
                    );
                }
            }
            TAG_RLTS => {
                let limit = (platform::read_u32(target + 0x4C) >> 24) & 0xF;
                for entry_index in 0..entries {
                    let entry =
                        node + platform::read_u32(node + 8 + entry_index * 4) as usize;
                    if (platform::read_u8(entry) as u32) >= limit {
                        continue;
                    }
                    let value = fn_8017d520(
                        time,
                        entry + platform::read_u32(entry + 8) as usize,
                        platform::read_u16(entry + 4) as i32,
                    ) as f32;
                    let array = platform::call1(FN_80182060, p32(target)) as usize;
                    platform::write_f32(
                        array
                            + platform::read_u8(entry) as usize * 0x14
                            + platform::read_u8(entry + 1) as usize * 4,
                        value,
                    );
                }
            }
            TAG_RLTP => {
                let table = platform::read_u32(owner + 0x14) as usize;
                if table == 0 {
                    continue;
                }
                let limit = (platform::read_u32(target + 0x50) >> 28) & 0xF;
                for entry_index in 0..entries {
                    let entry =
                        node + platform::read_u32(node + 8 + entry_index * 4) as usize;
                    if (platform::read_u8(entry) as u32) >= limit
                        || platform::read_u8(entry + 1) != 0
                    {
                        continue;
                    }
                    let key = dialog_keyframe_lookup_u16(
                        time,
                        entry + platform::read_u32(entry + 8) as usize,
                        platform::read_u16(entry + 4) as i32,
                    ) as u32;
                    platform::call3(
                        FN_801822F0,
                        p32(target),
                        platform::read_u8(entry) as u32,
                        platform::read_u32(table + (key & 0xFFFF) as usize * 4),
                    );
                }
            }
            TAG_RLIM => {
                let limit = (platform::read_u32(target + 0x4C) >> 0x12) & 3;
                for entry_index in 0..entries {
                    let entry =
                        node + platform::read_u32(node + 8 + entry_index * 4) as usize;
                    if (platform::read_u8(entry) as u32) >= limit {
                        continue;
                    }
                    let value = fn_8017d520(
                        time,
                        entry + platform::read_u32(entry + 8) as usize,
                        platform::read_u16(entry + 4) as i32,
                    ) as f32;
                    let array = platform::call1(FN_801820A0, p32(target)) as usize;
                    platform::write_f32(
                        array
                            + platform::read_u8(entry) as usize * 0x14
                            + platform::read_u8(entry + 1) as usize * 4,
                        value,
                    );
                }
            }
            _ => {}
        }
    }
}

/// `0x801A3420`: release one sound handle stored at `+0x634`.
const FN_801A3420: usize = 0x801A_3420;
/// `0x801A0CA0`: release a `(bank, id)` pair.
const FN_801A0CA0: usize = 0x801A_0CA0;
/// `0x801A9AF0`: release a `(group, id)` pair.
const FN_801A9AF0: usize = 0x801A_9AF0;
/// `0x801A1910`: release one 16-bit sound id.
const FN_801A1910: usize = 0x801A_1910;
/// `0x801A1920`: release one 16-bit sound id.
const FN_801A1920: usize = 0x801A_1920;
/// `0x801A1930`: release one 16-bit sound id.
const FN_801A1930: usize = 0x801A_1930;
/// `0x80180020`: fill a 16-byte rectangle record from a layout.
const FN_80180020: usize = 0x8018_0020;
/// `0x802F5D90`: neutral scale used for the four widget records.
const DAT_802F5D90: usize = 0x802F_5D90;
/// `0x802F5DA0`: horizontal design width divided by the layout scale.
const DAT_802F5DA0: usize = 0x802F_5DA0;
/// `0x802F5DA4`: vertical design height divided by the layout scale.
const DAT_802F5DA4: usize = 0x802F_5DA4;
/// `0x802F5DA8`: first `N_Trans` translation pair.
const DAT_802F5DA8: usize = 0x802F_5DA8;
/// `0x802F5DAC`: per-frame slide value passed to the control object.
const DAT_802F5DAC: usize = 0x802F_5DAC;
/// `0x802F5DB0`: same design width, used by the scroll timer.
const DAT_802F5DB0: usize = 0x802F_5DB0;
/// `0x8032A360`: `N_Trans` pane name.
const S_N_TRANS: usize = 0x8032_A360;
/// `0x8032AC08`: `let_icn_00` pane name.
const S_LET_ICN_00: usize = 0x8032_AC08;
/// `0x8032AE48`: `N_cntrl_01` pane name.
const S_N_CNTRL_01: usize = 0x8032_AE48;
/// `0x8032ACA8`: first of the five `B_optnBtn_00` pane names.
const S_B_OPTN_BTN_00: usize = 0x8032_ACA8;
/// `0x8032ACEC`: first of the three `T_msg_00` pane names.
const S_T_MSG_00: usize = 0x8032_ACEC;

/// Calls `(f64, f64, u32 x6) -> ()`.
///
/// # Original
/// Host-side helper; no binary address. Models the `+0x30` control-object call
/// in [`fn_80175f70`], which takes two doubles in `f1`/`f2` and six words in
/// `r3..r8`.
///
/// # Safety
/// `addr` must accept that exact argument list.
///
/// # Confidence
/// high - the register trace loads six integer registers and two float
/// registers before the indirect call.
#[inline]
unsafe fn call_f64x2_u32x6(
    addr: usize,
    a: f64,
    b: f64,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
) {
    let f: extern "C" fn(f64, f64, u32, u32, u32, u32, u32, u32) = core::mem::transmute(addr);
    f(a, b, r3, r4, r5, r6, r7, r8);
}

/// Builds the home-button pane tree the first time the overlay opens.
///
/// The body is guarded by its own `+0x94` "built" byte. It sizes the four
/// player roots, releases the old sound ids when `+0x97` says a previous run
/// used them, clears the menu state words, turns on every `B_btnL_00` button,
/// builds the four `N_Trans` players, runs the control object's `+0x30` setup
/// eight times, resets the four widget records, hides the icon, option and
/// text panes, then arms the scroll timer.
///
/// # Original
/// - Ghidra: `FUN_80175f70` at `0x80175F70` (1184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175f70_FUN_80175f70.c`
/// - observed: the export prints `FUN_802dcd70()` as the source of `this`; the
///   register trace shows the helper only saves `r24..r31` and the incoming
///   `r3` is the menu object.
///
/// # Arguments
/// - `this`: live home-button menu object.
///
/// # Side effects
/// Writes many menu fields, calls sound release, layout and control methods.
///
/// # Safety
/// `this` must be the `0x7A0`-byte menu built by [`fn_80175330`].
///
/// # Confidence
/// medium - the stores, tables and virtual slots are literal; the two object
/// pointers at `+0x238`/`+0x250` are named from their use.
pub unsafe fn fn_80175f70(this: usize) {
    if platform::read_u8(this + 0x94) != 0 {
        return;
    }
    platform::write_u8(this + 0x94, 1);

    let _ = call0_r(FN_801DFAD0);
    platform::write_u8(this + 0x95, 0);
    platform::write_u8(this + 0x96, 0);
    platform::write_u8(this + 0x9A, 0);
    platform::write_u8(this + 0x98, 0);
    platform::write_u32(this + 0x20C, 0);

    let owner = platform::read_u32(DAT_80377FB8) as usize;
    let scale = read_f32(platform::read_u32(owner + 4) as usize + 0x30);
    platform::write_u32(this + 0x61C, (read_f32(DAT_802F5DA0) / scale) as i32 as u32);
    platform::write_u32(this + 0x620, (read_f32(DAT_802F5DA4) / scale) as i32 as u32);

    if platform::read_u8(this + 0x97) != 0 {
        platform::call1(FN_801A3420, p32(this + 0x634));
        platform::call2(
            FN_801A0CA0,
            platform::read_u32(this + 0x794),
            platform::read_u32(this + 0x798),
        );
        platform::call2(
            FN_801A9AF0,
            platform::read_u32(this + 0x62C),
            platform::read_u32(this + 0x630),
        );
        platform::call1(FN_801A1910, platform::read_u16(this + 0x624) as u32);
        platform::call1(FN_801A1920, platform::read_u16(this + 0x626) as u32);
        platform::call1(FN_801A1930, platform::read_u16(this + 0x628) as u32);
        platform::write_u8(this + 0x97, 0);
    }

    for offset in (0x20..=0x54).step_by(4) {
        platform::write_u32(this + offset, 0);
    }
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this, 0);
    platform::write_u8(this + 0x90, 0);

    let layout = platform::read_u32(this + 0x250) as usize;
    platform::call1(vtable_method(layout, 0, 0xC), p32(layout));
    platform::call2(vtable_method(layout, 0, 0x38), p32(layout), 0);

    let buttons = platform::read_u32(this + 0xC) as usize;
    for index in 0..buttons {
        let name = platform::read_u32(TABLE_B_BTNL_00 + index * 4);
        let group = platform::read_u32(this + 0x238) as usize;
        let layout = platform::read_u32(group + 0x10) as usize;
        let pane = platform::call3(
            vtable_method(layout, 0, 0x3C),
            p32(layout),
            name,
            1,
        );
        let control_object = platform::read_u32(this + 0x250) as usize;
        let control = platform::call2(
            vtable_method(control_object, 0, 0x4C),
            p32(control_object),
            pane,
        );
        platform::call2(vtable_method(control as usize, 0, 0x50), control, 1);
    }

    menu_refresh_buttons(this);

    let mut rectangle = [0u32; 4];
    platform::call2(
        FN_80180020,
        p32(rectangle.as_mut_ptr() as usize),
        platform::read_u32(this + 0x238),
    );
    for index in 0..4 {
        platform::write_u32(this + 0x28C + index * 4, rectangle[index]);
    }

    let group = platform::read_u32(this + 0x238) as usize;
    platform::call2(
        vtable_method(group, 0, 0x24),
        p32(group),
        p32(this + 0x258),
    );

    for player in 0..4usize {
        let owner = platform::read_u32(this + 0x23C + player * 4) as usize;
        platform::call2(
            vtable_method(owner, 0, 0x24),
            p32(owner),
            p32(this + 0x258),
        );
        let pane = platform::call3(
            vtable_method(platform::read_u32(owner + 0x10) as usize, 0, 0x3C),
            platform::read_u32(owner + 0x10),
            platform::read_u32(S_N_TRANS),
            1,
        ) as usize;
        platform::write_f32(pane + 0x2C, read_f32(DAT_802F5DA8));
        platform::write_f32(pane + 0x30, read_f32(DAT_802F5DA8));
        platform::write_f32(pane + 0x34, read_f32(DAT_802F5D90));
    }

    let control = platform::read_u32(this + 0x250) as usize;
    let slide = read_f32(DAT_802F5DAC) as f64;
    for index in 0..8u32 {
        call_f64x2_u32x6(
            vtable_method(control, 0, 0x30),
            slide,
            slide,
            p32(control),
            index,
            0,
            0,
            0,
            0,
        );
    }

    for player in 0..4usize {
        platform::write_u32(this + 0x58 + player * 4, 0);
        let widget = platform::read_u32(this + 0x2AC + player * 4) as usize;
        fn_801734c0(widget);
        fn_801734a0(widget);
        fn_801734f0();
        fn_80173a10(widget);
        fn_801731f0(widget);
        fn_80173610(widget);
        platform::write_f32(this + 0x210 + player * 4, read_f32(DAT_802F5D90));
        platform::write_f32(this + 0x220 + player * 4, read_f32(DAT_802F5D90));
    }

    let group = platform::read_u32(this + 0x238) as usize;
    let icon = platform::call3(
        vtable_method(platform::read_u32(group + 0x10) as usize, 0, 0x3C),
        platform::read_u32(group + 0x10),
        platform::read_u32(S_LET_ICN_00),
        1,
    ) as usize;
    platform::write_u8(icon + 0xCF, platform::read_u8(icon + 0xCF) & 0xFE);

    let group = platform::read_u32(this + 0x238) as usize;
    let control_pane = platform::call3(
        vtable_method(platform::read_u32(group + 0x10) as usize, 0, 0x3C),
        platform::read_u32(group + 0x10),
        platform::read_u32(S_N_CNTRL_01),
        1,
    ) as usize;
    platform::write_u8(
        control_pane + 0xCF,
        (platform::read_u8(control_pane + 0xCF) & 0xFE) | 1,
    );

    for index in 0..5usize {
        let group = platform::read_u32(this + 0x238) as usize;
        let pane = platform::call3(
            vtable_method(platform::read_u32(group + 0x10) as usize, 0, 0x3C),
            platform::read_u32(group + 0x10),
            platform::read_u32(S_B_OPTN_BTN_00 + index * 4),
            1,
        ) as usize;
        platform::write_u8(pane + 0xCF, platform::read_u8(pane + 0xCF) & 0xFE);
    }

    for index in 0..3usize {
        let group = platform::read_u32(this + 0x238) as usize;
        let pane = platform::call3(
            vtable_method(platform::read_u32(group + 0x10) as usize, 0, 0x3C),
            platform::read_u32(group + 0x10),
            platform::read_u32(S_T_MSG_00 + index * 4),
            1,
        ) as usize;
        platform::write_u8(pane + 0xCF, platform::read_u8(pane + 0xCF) & 0xFE);
    }

    fn_80173e40(platform::read_u32(this + 0x2BC) as usize);
    platform::call2(FN_801769B0, p32(this), 0);

    let owner = platform::read_u32(DAT_80377FB8) as usize;
    let scale = read_f32(platform::read_u32(owner + 4) as usize + 0x30);
    let ticks = read_f32(DAT_802F5DB0) / scale;
    platform::write_u8(this + 0x460, 1);
    platform::write_u32(this + 0x454, 0);
    platform::write_u32(this + 0x45C, 0);
    platform::write_u32(this + 0x458, ticks as i32 as u32);
}

/// `0x802F5AB0`: `(pane index, kind)` word table for the first pane group.
const TABLE_802F5AB0: usize = 0x802F_5AB0;
/// `0x8032A458`: `btnL_00_inOut` child-pane names.
const TABLE_BTNL_00_INOUT: usize = 0x8032_A458;
/// `0x8032A348`: `P1_Def_brlyt` player layout names.
const TABLE_P1_DEF_BRLYT: usize = 0x8032_A348;
/// `0x8032A4B4`: `_cntBtn_in_brlan` animation names.
const TABLE_CNTBTN_IN_BRLAN: usize = 0x8032_A4B4;
/// `0x8032A918`: `_hmMenu_strt_brlan` animation names.
const TABLE_HMMENU_STRT_BRLAN: usize = 0x8032_A918;
/// `0x8032AB40`: `hmMenu_strt` child-pane names.
const TABLE_HMMENU_STRT: usize = 0x8032_AB40;
/// `0x8032A5F8`: `_ltrIcn_on_brlan` animation names.
const TABLE_LTRICN_ON_BRLAN: usize = 0x8032_A5F8;
/// `0x8032A708`: `ltrIcn_on` child-pane names.
const TABLE_LTRICN_ON: usize = 0x8032_A708;
/// `0x8032AE38`: name bound to the dialog message node.
const S_DAT_8032AE38: usize = 0x8032_AE38;
/// `0x8032AE40`: `back_01` node name.
const S_BACK_01: usize = 0x8032_AE40;
/// `0x8032AF30`: vtable of the small `+0x254` object.
const PTR_8032AF30: usize = 0x8032_AF30;
/// `0x80329E10`: first vtable of the `+0x250` control object.
const PTR_80329E10: usize = 0x8032_9E10;
/// `0x80329D70`: second vtable of the `+0x250` control object.
const PTR_80329D70: usize = 0x8032_9D70;
/// `0x802E1938`: copy the menu name prefix into a 64-byte buffer.
const FN_802E1938: usize = 0x802E_1938;
/// `0x802E1A3C`: append one name to that buffer.
const FN_802E1A3C: usize = 0x802E_1A3C;
/// `0x8018A5C0`: construct one embedded list at `owner + offset`.
const FN_8018A5C0: usize = 0x8018_A5C0;
/// `0x802F5D8C`: value stored in the `back_01` node at `+0x40`/`+0x44`.
const DAT_802F5D8C: usize = 0x802F_5D8C;
/// `0x802F5D98`: `2^52` bias of the unsigned-to-double conversion.
const DAT_802F5D98: usize = 0x802F_5D98;

/// Returns the number of 16-bit characters before the first `"` (`0x22`).
///
/// # Original
/// Host-side decomposition of the `psVar10` scan repeated in
/// `work/ghidra/main-gb/export/80175510_FUN_80175510.c`.
///
/// # Arguments
/// - `text`: UTF-16 string.
///
/// # Returns
/// Character count before the quote.
///
/// # Safety
/// `text` must be a NUL-terminated UTF-16 string that also contains `"`.
///
/// # Confidence
/// high - the loop only increments while the halfword is not `0x22`.
#[inline]
unsafe fn text_length_to_quote(text: usize) -> u32 {
    let mut length = 0u32;
    while platform::read_u16(text + length as usize * 2) != 0x22 {
        length += 1;
    }
    length
}

/// Builds one group of named panes and their animation lists.
///
/// Each index gets a composed name, a layout pane, a `0x28`-byte list object,
/// the root's pane handle, the child found by name and one keyframe entry.
///
/// # Original
/// Host-side decomposition of the three consecutive loops in
/// `work/ghidra/main-gb/export/80175510_FUN_80175510.c`; each loop has this
/// exact shape and differs only in its tables, count and destination field.
///
/// # Arguments
/// - `this`: menu object.
/// - `field`: byte offset of the four-byte destination slot array.
/// - `count`: number of panes.
/// - `name_table`, `name_base`, `name_stride`: animation name source.
/// - `pane_table`, `pane_base`, `pane_stride`: child-pane name source.
///
/// # Safety
/// All tables and the menu object must be live.
///
/// # Confidence
/// high - the three loops are byte-identical apart from their constants.
#[inline]
unsafe fn build_pane_group(
    this: usize,
    field: usize,
    count: usize,
    name_table: usize,
    name_base: usize,
    name_stride: usize,
    pane_table: usize,
    pane_base: usize,
    pane_stride: usize,
) {
    for index in 0..count {
        let mut name = [0u8; 64];
        platform::call2(
            FN_802E1938,
            p32(name.as_mut_ptr() as usize),
            platform::read_u32(this + 0xB4),
        );
        platform::call2(
            FN_802E1A3C,
            p32(name.as_mut_ptr() as usize),
            platform::read_u32(name_table + (name_base + index * name_stride) * 4),
        );

        let layout = platform::read_u32(this + 0x24C) as usize;
        let pane = platform::call4(
            vtable_method(layout, 0, 0xC),
            p32(layout),
            0,
            p32(name.as_ptr() as usize),
            0,
        );

        let block = platform::call1(FN_80174DF0, 0x28);
        if block != 0 {
            let list = block as usize;
            fn_80171570(list);
            platform::write_u32(this + field + index * 4, p32(list));
        }

        let root = platform::read_u32(this + 0x238) as usize;
        let handle = platform::call3(
            vtable_method(root, 0, 0x10),
            p32(root),
            pane,
            p32(layout),
        );
        let list = platform::read_u32(this + field + index * 4) as usize;
        platform::write_u32(list + 0x24, handle);

        let child_name =
            platform::read_u32(pane_table + (pane_base + index * pane_stride) * 4);
        let child =
            dialog_find_child_by_name(platform::read_u32(root + 0x14) as usize, child_name);
        platform::write_u32(list + 0x20, p32(child));

        let child_object = platform::read_u32(list + 0x20) as usize;
        let sentinel = child_object + 0x10;
        let mut node = platform::read_u32(child_object + 0x10) as usize;
        while node != sentinel {
            let object = platform::read_u32(node + 8) as usize;
            platform::call3(
                vtable_method(object, 0, 0x44),
                p32(object),
                platform::read_u32(list + 0x24),
                0,
            );
            node = platform::read_u32(node) as usize;
        }

        let ticks = fn_8017d6f0(platform::read_u32(list + 0x24) as usize);
        let scale = read_f32(platform::read_u32(this + 4) as usize + 0x30);
        fn_801713c0(
            (ticks as f64) as f32 as f64,
            read_f32(DAT_802F5D90) as f64,
            scale as f64,
            list,
            0,
        );
    }
}

/// Builds the whole home-button pane tree.
///
/// Runs the two dialog parsers, allocates the dialog node at `+0x24C`, binds
/// its message, allocates the four player groups at `+0x23C` and the root
/// group at `+0x238`, looks for the `back_01` node, then builds the three pane
/// groups (`+0x2C0`, `+0x2F0`, `+0x418`), the two control objects at `+0x250`
/// and `+0x254`, the four scroll-text players at `+0x2AC`, the draw-info block
/// at `+0x25C`, and finally binds the three `T_msg_00` strings.
///
/// # Original
/// - Ghidra: `FUN_80175510` at `0x80175510` (2144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80175510_FUN_80175510.c`
/// - observed: `FUN_802dcd5c()` is the save-register helper; the incoming `r3`
///   is the menu object and becomes every `iVar5` in the export.
///
/// # Arguments
/// - `this`: live home-button menu object.
///
/// # Side effects
/// Allocates about seventy small objects and fills most of the menu.
///
/// # Safety
/// `this` must be the menu built by [`fn_80175330`]; the resource at `+4` must
/// be loaded.
///
/// # Confidence
/// medium - every allocation, table index and virtual slot is literal; the
/// three pane loops are factored into [`build_pane_group`].
pub unsafe fn fn_80175510(this: usize) {
    platform::write_u8(this + 0x94, 0);
    platform::write_u8(this + 0x95, 0);
    platform::write_u8(this + 0x96, 0);

    dialog_parse_entry(this);
    dialog_split_quoted(this);

    let block = platform::call1(FN_80174DF0, 0xB0);
    if block != 0 {
        let node = dialog_node_construct(block as usize);
        platform::write_u32(this + 0x24C, p32(node));
    }

    let node = platform::read_u32(this + 0x24C) as usize;
    let resource = platform::read_u32(this + 4) as usize;
    menu_bind_message(
        node,
        platform::read_u32(resource),
        p32(S_DAT_8032AE38),
    );

    if platform::read_u32(resource + 0x20) == 0 {
        for index in 0..4usize {
            let block = platform::call1(FN_80174DF0, 0x24);
            if block != 0 {
                let group = dialog_group_construct(block as usize);
                platform::write_u32(this + 0x23C + index * 4, p32(group));
            }

            let layout = platform::read_u32(this + 0x24C) as usize;
            let pane = platform::call4(
                vtable_method(layout, 0, 0xC),
                p32(layout),
                0,
                platform::read_u32(TABLE_P1_DEF_BRLYT + index * 4),
                0,
            );
            let group = platform::read_u32(this + 0x23C + index * 4) as usize;
            platform::call3(
                vtable_method(group, 0, 0xC),
                p32(group),
                pane,
                p32(layout),
            );
        }
    }

    let block = platform::call1(FN_80174DF0, 0x24);
    if block != 0 {
        let group = dialog_group_construct(block as usize);
        platform::write_u32(this + 0x238, p32(group));
    }

    let layout = platform::read_u32(this + 0x24C) as usize;
    let pane = platform::call4(
        vtable_method(layout, 0, 0xC),
        p32(layout),
        0,
        platform::read_u32(this + 0xB0),
        0,
    );
    let root = platform::read_u32(this + 0x238) as usize;
    platform::call3(
        vtable_method(root, 0, 0xC),
        p32(root),
        pane,
        p32(layout),
    );

    let child = platform::read_u32(root + 0x10) as usize;
    let sentinel = child + 0x14;
    let mut walk = platform::read_u32(child + 0x14) as usize;
    while walk != sentinel {
        if platform::call2(FN_802E1AB4, p32(walk + 0xB0), S_BACK_01 as u32) == 0 {
            let value = platform::read_u32(DAT_802F5D8C);
            platform::write_u32(walk + 0x40, value);
            platform::write_u32(walk + 0x44, value);
            break;
        }
        walk = platform::read_u32(walk) as usize;
    }

    build_pane_group(
        this,
        0x2C0,
        platform::read_u32(this + 0x10) as usize,
        TABLE_802F5AB0,
        3,
        2,
        TABLE_BTNL_00_INOUT,
        2,
        2,
    );
    build_pane_group(
        this,
        0x2F0,
        0x4A,
        TABLE_802F5B18,
        1,
        2,
        TABLE_HMMENU_STRT,
        0,
        2,
    );
    build_pane_group(
        this,
        0x418,
        0xF,
        TABLE_LTRICN_ON_BRLAN,
        0,
        1,
        TABLE_LTRICN_ON,
        0,
        1,
    );

    let block = platform::call1(FN_80174DF0, 0xC) as usize;
    if block != 0 {
        platform::write_u32(block, PTR_8032AF30 as u32);
        platform::write_u32(block + 8, p32(this));
        platform::write_u32(this + 0x254, p32(block));
    }

    let block = platform::call1(FN_80174DF0, 0x2C) as usize;
    if block != 0 {
        let owner = platform::read_u32(this + 0x254) as usize;
        platform::write_u32(block, PTR_80329E10 as u32);
        platform::write_u32(block + 4, p32(owner));
        platform::write_u32(block + 0x14, platform::read_u32(DAT_8032A304));
        if owner != 0 {
            platform::call2(vtable_method(owner, 0, 0xC), p32(owner), p32(block));
        }
        platform::call2(FN_8018A5C0, p32(block + 8), 8);
        platform::write_u32(block, PTR_80329D70 as u32);
        platform::write_u32(block + 0x24, 0);
        platform::call2(FN_8018A5C0, p32(block + 0x18), 8);
        platform::write_u32(this + 0x250, p32(block));
    }

    let control = platform::read_u32(this + 0x250) as usize;
    platform::call2(
        vtable_method(control, 0, 0x40),
        p32(control),
        platform::read_u32(this + 0x238),
    );

    let block = platform::call1(FN_80174DF0, 0x1F8) as usize;
    if block != 0 {
        fn_80173cd0(
            block,
            platform::read_u32(platform::read_u32(this + 4) as usize + 4) as usize,
        );
        platform::write_u32(this + 0x2BC, p32(block));
    }

    let group = platform::read_u32(this + 0x2BC) as usize;
    for player in 0..4u32 {
        let block = platform::call1(FN_80174DF0, 0x48) as usize;
        if block != 0 {
            fn_80173130(block, player, p32(group));
            platform::write_u32(this + 0x2AC + player as usize * 4, p32(block));
        }
    }

    let control = platform::read_u32(this + 0x250) as usize;
    platform::call2(vtable_method(control, 0, 0x54), p32(control), p32(this + 0x258));

    let scale = read_f32(platform::read_u32(this + 4) as usize + 0x34);
    let unit = read_f32(DAT_802F5D94);
    platform::write_f32(this + 0x2A0, unit);
    let byte = platform::read_u8(this + 0x2A8);
    platform::write_u8(this + 0x2A8, ((platform::read_u8(this + 0x8F) & 1) << 5) | (byte & 0xDF));
    platform::write_f32(this + 0x29C, unit / scale);

    let mut draw = [0u32; 12];
    platform::call1(FN_801C04D0, p32(draw.as_mut_ptr() as usize));
    for index in 0..12 {
        platform::write_u32(this + 0x25C + index * 4, draw[index]);
    }

    for index in 0..3usize {
        let root = platform::read_u32(this + 0x238) as usize;
        let pane = platform::call3(
            vtable_method(platform::read_u32(root + 0x10) as usize, 0, 0x3C),
            platform::read_u32(root + 0x10),
            platform::read_u32(S_T_MSG_00 + index * 4),
            1,
        ) as usize;

        let mut present = false;
        let mut node = platform::call1(vtable_method(pane, 0, 0xC), p32(pane)) as usize;
        while node != 0 {
            if node == MARKER_80378020 {
                present = true;
                break;
            }
            node = platform::read_u32(node) as usize;
        }

        // inferred: the export stores 0 in the pane pointer when the marker is
        // absent and would then call through that null pointer; the
        // reconstruction skips the call instead.
        let node = if present { pane } else { 0 };
        let text_index =
            platform::read_u32(platform::read_u32(this + 4) as usize + 0x1C) as usize * 0x18
                + index * 4;
        let text = platform::read_u32(this + 0xBC + text_index) as usize;
        let length = text_length_to_quote(text);
        if node != 0 {
            platform::call4(
                vtable_method(node, 0, 0x70),
                p32(node),
                p32(text),
                0,
                length,
            );
        }
    }
}

// ===========================================================================
// Wave 108: page state machines and the scrolling-text updater
// ===========================================================================

// ---------------------------------------------------------------------------
// Wave 108 data addresses
// ---------------------------------------------------------------------------

/// `0x802F5DBC`: divisor that turns a scroll index into a progress value.
const DAT_802F5DBC: usize = 0x802F_5DBC;

/// `0x802F5DC0`: the 2^52 bias used by the integer-to-double conversions in
/// the scroll updates.
const DAT_802F5DC0: usize = 0x802F_5DC0;

/// `0x802F5DCC`: per-frame scroll step.
const DAT_802F5DCC: usize = 0x802F_5DCC;

/// `0x802F5DD0`: target value set when a scrolling widget finishes.
const DAT_802F5DD0: usize = 0x802F_5DD0;

/// `0x802F5DE4`: frame-time numerator of the scrolling-text timer.
const DAT_802F5DE4: usize = 0x802F_5DE4;

/// `0x802F5DE8`: Z rotation used when a page is already selected.
const DAT_802F5DE8: usize = 0x802F_5DE8;

/// `0x802F5DEC`: scale applied to the computed text angle.
const DAT_802F5DEC: usize = 0x802F_5DEC;

/// `0x802F5DF0`: X scale of the per-player text quad.
const DAT_802F5DF0: usize = 0x802F_5DF0;

/// `0x802F5DF4`: Y scale of the per-player text quad.
const DAT_802F5DF4: usize = 0x802F_5DF4;

/// `0x802F5DF8`: control-object Y argument used by the page reset.
const DAT_802F5DF8: usize = 0x802F_5DF8;

/// `0x802F5DFC`: scroll value written when a page becomes active.
const DAT_802F5DFC: usize = 0x802F_5DFC;

/// `0x801786B0`: refresh the battery panes of one player.
const FN_801786B0: usize = 0x8017_86B0;

/// `0x801886A0`: angle helper used for the text rotation.
const FN_801886A0: usize = 0x8018_86A0;

/// `0x801E15B0`: enable/disable the dialog cursor.
const FN_801E15B0: usize = 0x801E_15B0;

/// `0x801E15F0`: query the dialog cursor position.
const FN_801E15F0: usize = 0x801E_15F0;

/// `0x8032ACF4`: `T_Dialog` pane name.
const S_T_DIALOG: usize = 0x8032_ACF4;

/// `0x8032ACF0`: second `T_msg_*` pane name.
const S_T_MSG_01: usize = 0x8032_ACF0;

/// `0x8032AE58`: `back_00` pane name.
const S_BACK_00: usize = 0x8032_AE58;

/// `0x8032AE60`: `back_02` pane name.
const S_BACK_02: usize = 0x8032_AE60;

/// `0x8032ADF8`: first of the four per-player `btryPwr_00_0` names.
const S_BTRY_PWR: usize = 0x8032_ADF8;

/// `0x8032A36C`: `N_Rot` pane name.
const S_N_ROT: usize = 0x8032_A36C;

/// `0x8032A378`: `N_SRot` pane name.
const S_N_SROT: usize = 0x8032_A378;

// ---------------------------------------------------------------------------
// Wave 108 shared helpers
// ---------------------------------------------------------------------------

/// Reads the word at `base + index * 4`, allowing negative indices.
///
/// # Original
/// Host-side factoring of the `param_1[i]` accesses in the five wave-108
/// bodies; no binary address of its own.
///
/// # Safety
/// The resulting address must be readable.
///
/// # Confidence
/// high - a single indexed word load.
#[inline]
unsafe fn indexed_word(base: usize, index: i32) -> u32 {
    platform::read_u32((base as isize + index as isize * 4) as usize)
}

/// Writes the word at `base + index * 4`, allowing negative indices.
///
/// # Original
/// Host-side factoring of the `param_1[i]` stores in the five wave-108
/// bodies; no binary address of its own.
///
/// # Safety
/// The resulting address must be writable.
///
/// # Confidence
/// high - a single indexed word store.
#[inline]
unsafe fn set_indexed_word(base: usize, index: i32, value: u32) {
    platform::write_u32((base as isize + index as isize * 4) as usize, value);
}

/// Scans the `0x802F5B18` four-word groups for `(first, second)`.
///
/// This is the general form of [`find_dialog_group`]: every wave-108 page
/// machine walks the same table, but with different pair values.
///
/// # Original
/// Host-side factoring of the table walks in
/// `work/ghidra/main-gb/export/8017a120_FUN_8017a120.c`,
/// `8017af70_FUN_8017af70.c` and `8017ba40_FUN_8017ba40.c`.
///
/// # Arguments
/// `first`, `second`: the two words of a pair; the table stores two pairs per
/// 16-byte group.
///
/// # Returns
/// The pair index, or `-1` after `0x25` groups.
///
/// # Safety
/// The table must contain `0x25` readable 16-byte groups.
///
/// # Confidence
/// high - the pair order and bound are literal in all three bodies.
#[inline]
unsafe fn find_menu_pair(first: i32, second: i32) -> i32 {
    let mut group = 0i32;
    let mut remaining = 0x25i32;
    let mut cursor = TABLE_802F5B18;

    loop {
        if platform::read_i32(cursor) == first && platform::read_i32(cursor + 4) == second {
            return group;
        }
        if platform::read_i32(cursor + 8) == first
            && platform::read_i32(cursor + 0xC) == second
        {
            return group + 1;
        }
        cursor += 0x10;
        group += 2;
        remaining -= 1;
        if remaining == 0 {
            return -1;
        }
    }
}

/// Activates the `+0x2F0` pane selected by a pair of table words.
///
/// # Original
/// Host-side factoring of the `pane[+0x14] = 1` blocks in the wave-108 page
/// machines; no binary address of its own.
///
/// # Safety
/// The pair must resolve to a live pane.
///
/// # Confidence
/// high - the lookup and the two writes are literal.
#[inline]
unsafe fn activate_pair(menu: usize, first: i32, second: i32) {
    let pane = indexed_word(menu, 0xBC + find_menu_pair(first, second)) as usize;
    let _ = platform::call1(FN_80171400, pane as u32);
    write_word(pane, 0x14, 1);
}

/// Clears the `+0x14` word of the `+0x2F0` pane selected by a pair.
///
/// # Original
/// Host-side factoring of the `pane[+0x14] = 0` blocks in the wave-108 page
/// machines; no binary address of its own.
///
/// # Safety
/// The pair must resolve to a live pane.
///
/// # Confidence
/// high - the lookup and the store are literal.
#[inline]
unsafe fn clear_pair(menu: usize, first: i32, second: i32) {
    let pane = indexed_word(menu, 0xBC + find_menu_pair(first, second)) as usize;
    write_word(pane, 0x14, 0);
}

/// Activates one pane from a direct menu index slot.
///
/// # Original
/// Host-side factoring of the `menu[+0x109]`-style activations in the wave-108
/// page machines; no binary address of its own.
///
/// # Safety
/// The slot must hold a live pane.
///
/// # Confidence
/// high - the two writes are literal.
#[inline]
unsafe fn activate_slot(menu: usize, index: i32) {
    let pane = indexed_word(menu, index) as usize;
    let _ = platform::call1(FN_80171400, pane as u32);
    write_word(pane, 0x14, 1);
}

/// Clears one pane from a direct menu index slot.
///
/// # Original
/// Host-side factoring of the `menu[+0x109]`-style clears in the wave-108 page
/// machines; no binary address of its own.
///
/// # Safety
/// The slot must hold a live pane.
///
/// # Confidence
/// high - the store is literal.
#[inline]
unsafe fn clear_slot(menu: usize, index: i32) {
    write_word(indexed_word(menu, index) as usize, 0x14, 0);
}

/// Finds an entry pane in the `0x802F5AB8` `(entry, kind)` pair table.
///
/// # Original
/// Host-side factoring of the two `0x802F5AB8` walks in
/// `work/ghidra/main-gb/export/8017a120_FUN_8017a120.c` and
/// `8017ba40_FUN_8017ba40.c`.
///
/// # Arguments
/// `menu`: owner of the pair count at `+0x10`.
/// `entry`, `kind`: values compared against the two table words.
///
/// # Returns
/// The zero-based pair index, or `-1`.
///
/// # Safety
/// `menu + 0x10` must be readable and the table must hold that many pairs.
///
/// # Confidence
/// high - the loop bound and both comparisons are literal.
#[inline]
unsafe fn find_entry_pair(menu: usize, entry: i32, kind: i32) -> i32 {
    let count = read_i32(menu, 0x10);
    let mut index = -1;
    let mut cursor = 0;
    while cursor < count {
        let pair = TABLE_802F5AB8 + cursor as usize * 8;
        if read_i32(pair, 0) == entry && read_i32(pair, 4) == kind {
            index = cursor;
            break;
        }
        cursor += 1;
    }
    index
}

/// Resolves a pane by name through the layout object at `menu + owner_offset`.
///
/// # Original
/// Host-side factoring of the repeated `vtable+0x3C` pane lookups in the
/// wave-108 bodies; no binary address of its own.
///
/// # Safety
/// `menu + owner_offset` must hold a live layout object whose control object
/// is at `+0x10`.
///
/// # Confidence
/// high - the same three-step lookup is literal in every call site.
#[inline]
unsafe fn layout_pane(menu: usize, owner_offset: usize, name: usize) -> usize {
    let owner = read_word(menu, owner_offset) as usize;
    let control = read_word(owner, 0x10) as usize;
    platform::call3(
        vtable_method(control, 0, 0x3C),
        p32(control),
        name as u32,
        1,
    ) as usize
}

/// Sets or clears the visible bit at `pane + 0xCF`.
///
/// # Original
/// Host-side factoring of the `& 0xFE` / `| 1` byte updates in the wave-108
/// bodies; no binary address of its own.
///
/// # Safety
/// `pane + 0xCF` must be writable.
///
/// # Confidence
/// high - one read-modify-write of bit 0.
#[inline]
unsafe fn set_pane_visible(pane: usize, visible: bool) {
    let value = read_byte(pane, 0xCF);
    write_byte(
        pane,
        0xCF,
        if visible { value | 1 } else { value & 0xFE },
    );
}

/// Runs the `(5, id)` interface sound callback, or the fallback if unset.
///
/// The interface object at `menu + 4` installs the callback at `+0x14`; a
/// zero result means the callback did not handle the event and the MIDI
/// player at [`fn_801747a0`] runs instead.
///
/// # Original
/// Host-side factoring of the repeated five-line sound block in
/// `work/ghidra/main-gb/export/801795b0_FUN_801795b0.c` and the page machines.
///
/// # Safety
/// The callback, when non-zero, must accept `(5, id)`.
///
/// # Confidence
/// high - the guard and the fallback test are literal.
#[inline]
unsafe fn menu_sound_event(menu: usize, id: u32) {
    let interface = read_word(menu, 4) as usize;
    let callback = read_word(interface, 0x14) as usize;
    let mut handled = 0u32;
    if callback != 0 {
        handled = platform::call2(callback, 5, id);
    }
    if handled == 0 {
        fn_801747a0(id);
    }
}

/// Converts a scroll index to progress and updates the four player widgets.
///
/// The original uses the `0x43300000`/2^52 sequence to convert the unsigned
/// word at `menu + 0x84` to a double and then divides by
/// `0x802F5DBC`; `as f64` is the same value without the host-specific bit
/// pattern.
///
/// # Original
/// Host-side factoring of the four loops in
/// `work/ghidra/main-gb/export/801795b0_FUN_801795b0.c` and
/// `8017ba40_FUN_8017ba40.c`.
///
/// # Safety
/// The four widget pointers at `menu + 0x2AC` must be live.
///
/// # Confidence
/// medium - the four-call shape is literal; the scroll divisor's role is
/// inferred from the surrounding index range.
#[inline]
unsafe fn scroll_progress(menu: usize, value: u32) {
    let progress = (value as f64 / read_f32(DAT_802F5DBC) as f64) as f32;
    for player in 0..4usize {
        let widget = indexed_word(menu, 0xAB + player as i32) as usize;
        fn_80173500(progress as f64, widget);
        fn_80173510(widget, 1);
    }
}

/// Calls a two-`f64` function returning `f64`.
///
/// # Original
/// Host-side helper for the `0x801886A0` call in [`fn_80178c50`]; no binary
/// address of its own.
///
/// # Safety
/// `addr` must accept two doubles and return one.
///
/// # Confidence
/// high - the export passes both `f1`/`f2` operands and uses `f1` as result.
#[inline]
unsafe fn call_f64x2(addr: usize, a: f64, b: f64) -> f64 {
    let f: extern "C" fn(f64, f64) -> f64 = core::mem::transmute(addr);
    f(a, b)
}

/// Counts UTF-16 units after the first one until a quote (`0x22`).
///
/// # Original
/// Host-side factoring of the first length scan in
/// `work/ghidra/main-gb/export/8017ba40_FUN_8017ba40.c`.
///
/// # Safety
/// `text` must be readable and contain a quote.
///
/// # Confidence
/// high - the `+2` start and the `0x22` stop are literal.
#[inline]
unsafe fn text_length_after_open_quote(text: usize) -> u32 {
    let mut length = 0u32;
    while platform::read_u16(text + length as usize * 2 + 2) != 0x22 {
        length += 1;
    }
    length
}

/// Counts UTF-16 units until `0xFF1F` or `0x3F`.
///
/// # Original
/// Host-side factoring of the second length scan in
/// `work/ghidra/main-gb/export/8017ba40_FUN_8017ba40.c`.
///
/// # Safety
/// `text` must be readable and contain one of the two terminators.
///
/// # Confidence
/// high - both halfword comparisons are literal.
#[inline]
unsafe fn text_length_to_menu_terminator(text: usize) -> u32 {
    let mut length = 0u32;
    loop {
        let unit = platform::read_u16(text + length as usize * 2);
        if unit == 0xFF1F || unit == 0x3F {
            return length;
        }
        length += 1;
    }
}

/// `0x802F5DD4`: option-page dwell time in the `+0x234` countdown.
const DAT_802F5DD4: usize = 0x802F_5DD4;

/// `0x802F5DD8`: scale applied to the transition progress.
const DAT_802F5DD8: usize = 0x802F_5DD8;

/// `0x802F5DDC`: gain applied to the scroll index in the shutdown sound.
const DAT_802F5DDC: usize = 0x802F_5DDC;

/// `0x80176410`: reset the four widget records for a new page.
const FN_80176410: usize = 0x8017_6410;

/// `0x80176610`: capture the audio/effect state before the first page.
const FN_80176610: usize = 0x8017_6610;

/// `0x80176740`: refresh the four widget records from the current selection.
const FN_80176740: usize = 0x8017_6740;

/// `0x80179E60`: show/hide the ten `B_btn_00` panes for a menu page.
const FN_80179E60: usize = 0x8017_9E60;

/// `0x801E0E40`: release one controller channel.
const FN_801E0E40: usize = 0x801E_0E40;

/// `0x801E1820`: enable/disable the effect sound state.
const FN_801E1820: usize = 0x801E_1820;

/// `0x801E3460`: set the dialog sound level.
const FN_801E3460: usize = 0x801E_3460;

/// `0x801F3840`: query the deferred dialog release state.
const FN_801F3840: usize = 0x801F_3840;

/// `0x801BDE50`: set the dialog visibility flag.
const FN_801BDE50: usize = 0x801B_DE50;

/// `0x801BDCB0`: commit the dialog visibility change.
const FN_801BDCB0: usize = 0x801B_DCB0;

/// Runs the home-button page state machine.
///
/// # Original
/// - Ghidra: `FUN_801769b0` at `0x801769B0` (7420 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801769b0_FUN_801769b0.c`
/// - observed: the entry calls `FUN_802dcd6c` (`__savegprlr_23`); the `r3`/`r4`
///   pair is the menu and the optional four-record array. Call sites
///   `0x80174F60` and `0x80175F70` pass exactly those two values.
///
/// # Arguments
/// * `menu` - home-button menu.
/// * `records` - optional four-record scrolling-text array; `0` on the boot
///   call from `0x80175F70`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Walks the `+0x2C0`, `+0x418` and `+0x2F0` pane arrays, advances the `+0x14`
/// page state, plays sound ids 0..0x16, arms the `+0x5E8` and scrolling-text
/// timers, and calls [`fn_80178c50`] when the record array is present.
///
/// # Safety
/// `menu` must be live; `records` is either zero or four initialized records;
/// every pane, widget and audio block must be initialized.
///
/// # Confidence
/// medium - every switch arm and store is literal; four helper bodies
/// (`0x80176410`, `0x80176610`, `0x80176740`, `0x80179E60`) are called by
/// address, and the `pcVar19` load in state `0x10` is reconstructed from the
/// preceding `+0x2F0` indexing because the export loses its assignment.
pub unsafe fn fn_801769b0(menu: usize, records: usize) {
    // observed: the three arrays are released before the state machine runs.
    let pair_count = read_i32(menu, 0x10);
    for index in 0..pair_count as usize {
        fn_801715d0(indexed_word(menu, 0xB0 + index as i32) as usize);
    }
    for index in 0..0x0Fusize {
        fn_801715d0(indexed_word(menu, 0x106 + index as i32) as usize);
    }
    for index in 0..0x4Ausize {
        fn_801715d0(indexed_word(menu, 0xBC + index as i32) as usize);
    }

    let resource = read_word(menu, 4) as usize;
    let slide = read_f32(DAT_802F5DAC) as f64;
    let settle = read_f32(DAT_802F5DD0);

    // observed: the four per-player scroll/fade fields are advanced before the
    // state switch, exactly like fn_80178c50.
    for player in 0..4usize {
        let slice = menu + player * 4;
        let widget = indexed_word(menu, 0xAB + player as i32) as usize;
        let current = read_f32(slice + 0x210);
        if current as f64 <= read_f32(DAT_802F5D90) as f64 {
            if read_f32(slice + 0x220) as f64 > read_f32(DAT_802F5D90) as f64 {
                let mut target = read_f32(slice + 0x220) - read_f32(resource + 0x30);
                platform::write_f32(slice + 0x220, target);
                if target as f64 <= read_f32(DAT_802F5D90) as f64 {
                    target = read_f32(DAT_802F5D90);
                    platform::write_f32(slice + 0x220, target);
                }
            }
        } else if read_byte(menu, 0x80 + player) == 0 || read_byte(widget, 0x1C) == 0 {
            platform::write_f32(slice + 0x210, read_f32(DAT_802F5D90));
            platform::write_f32(slice + 0x220, read_f32(DAT_802F5D90));
            if read_byte(widget, 0x1C) != 0 {
                fn_801738f0(widget);
            }
        } else {
            let mut value = current - read_f32(resource + 0x30);
            platform::write_f32(slice + 0x210, value);
            if value as f64 <= read_f32(DAT_802F5D90) as f64 || read_i32(menu, 0x14) == 0x11 {
                fn_801738f0(widget);
                platform::write_f32(slice + 0x210, read_f32(DAT_802F5D90));
                platform::write_f32(slice + 0x220, settle);
            }
        }
    }

    match read_i32(menu, 0x14) {
        0 => {
            if read_word(resource, 0x18) == 0 {
                let index = find_menu_pair(0, 0);
                write_word(menu, 0x18, index as u32);
                set_pane_visible(layout_pane(menu, 0x238, S_BACK_00), true);
                set_pane_visible(layout_pane(menu, 0x238, S_BACK_02), false);
            } else {
                let index = find_menu_pair(2, 0);
                write_word(menu, 0x18, index as u32);
                set_pane_visible(layout_pane(menu, 0x238, S_BACK_00), false);
                set_pane_visible(layout_pane(menu, 0x238, S_BACK_02), true);
            }
            let index = read_i32(menu, 0x18);
            let pane = indexed_word(menu, 0xBC + index) as usize;
            let _ = platform::call1(FN_80171400, pane as u32);
            write_word(pane, 0x14, 1);

            if records != 0 {
                write_word(menu, 0x14, 1);
                platform::call2(FN_80176740, p32(menu), p32(records));
            }
        }
        1 => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xBC + index) as usize, 0x14) != 1 {
                platform::call1(FN_80176410, p32(menu));
                let cursor = call0_r(FN_801E15F0);
                let present = if cursor != 0 { 1u8 } else { 0 };
                write_byte(menu, 0x8C, present);
                if cursor != 0 {
                    activate_pair(menu, 0x0D, 6);
                    activate_pair(menu, 0x0E, 8);
                } else {
                    activate_pair(menu, 0x0D, 8);
                    activate_pair(menu, 0x0E, 6);
                }
                platform::call1(FN_80176610, p32(menu));
                menu_sound_event(menu, 0);
                write_word(menu, 0x14, 2);
            }
        }
        3 => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xBC + index) as usize, 0x14) != 1
                && index != 5
            {
                for player in 0..4usize {
                    for sub in 0..4usize {
                        let name = platform::read_u32(
                            S_BTRY_PWR + (player * 4 + sub) * 4,
                        ) as usize;
                        set_pane_visible(layout_pane(menu, 0x238, name), false);
                    }
                }
                write_word(menu, 0x18, 5);
                activate_slot(menu, 0x10B);
            }

            let remaining = read_i32(menu, 0x230);
            write_word(menu, 0x230, (remaining - 1) as u32);
            if remaining - 1 < 1 {
                for player in 0..4u32 {
                    platform::call1(FN_801E0E40, player);
                }
                write_word(menu, 0x14, 4);
                let owner = read_word(DAT_80377FB8, 4) as usize;
                write_word(
                    menu,
                    0x234,
                    (read_f32(DAT_802F5DD4) / read_f32(owner + 0x30)) as i32 as u32,
                );
            }
        }
        4 => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xBC + index) as usize, 0x14) != 1 {
                if read_i32(menu, 0x234) > 0 {
                    let mut channel = 0usize;
                    let mut buffer = [0u32; 2];
                    while channel < 4 {
                        let status = platform::call2(
                            FN_801E0F40,
                            channel as u32,
                            p32(buffer.as_mut_ptr() as usize),
                        ) as i32;
                        if status != -1 {
                            break;
                        }
                        channel += 1;
                    }
                    if channel < 4 {
                        let remaining = read_i32(menu, 0x234);
                        write_word(menu, 0x234, (remaining - 1) as u32);
                        // observed: the C `break` leaves this switch arm only.
                        // The rest of state 4 is skipped.
                    } else {
                        write_word(menu, 0x234, 0);
                        arm_option_page(menu);
                    }
                } else {
                    arm_option_page(menu);
                }
            }
        }
        5 | 6 => {
            let index = read_i32(menu, 0x18);
            if read_byte(menu, 0x91) != 0
                && read_word(indexed_word(menu, 0x106 + index) as usize, 0x14) != 1
            {
                if read_i32(menu, 0x1C) == 0 {
                    for sub in 0..5i32 {
                        activate_pair(menu, sub + 6, 7);
                    }
                    menu_confirm_pending(menu);
                    let pane = indexed_word(menu, 0x114) as usize;
                    write_word(pane, 0x18, 2);
                    activate_slot(menu, 0x114);
                }

                let mut connected = 0u32;
                if read_byte(menu, 0x80) != 0 {
                    connected = 1;
                    if read_byte(menu, 0x81) != 0 {
                        connected = 2;
                        if read_byte(menu, 0x82) != 0 {
                            connected = 3;
                            if read_byte(menu, 0x83) != 0 {
                                connected = 4;
                            }
                        }
                    }
                }
                if connected > 3 {
                    write_byte(menu, 0x93, 1);
                }

                if read_byte(menu, 0x93) == 0 {
                    let elapsed = read_i32(menu, 0x1C);
                    write_word(menu, 0x1C, (elapsed + 1) as u32);
                    if read_i32(menu, 0x61C) < elapsed + 1 {
                        write_word(menu, 0x14, 7);
                        if call0_r(FN_801F38F0) == 0 {
                            arm_scroll_timer(menu, 1);
                        }
                        write_byte(menu, 0x92, 1);
                    }
                } else if read_i32(menu, 0x14) == 6 {
                    let elapsed = read_i32(menu, 0x1C);
                    write_word(menu, 0x1C, (elapsed + 1) as u32);
                    if read_i32(menu, 0x61C) < elapsed + 1 {
                        write_word(menu, 0x14, 7);
                        if call0_r(FN_801F38F0) == 0 {
                            arm_scroll_timer(menu, 1);
                        }
                        write_byte(menu, 0x92, 1);
                    }
                } else {
                    let selected = read_i32(menu, 0x88);
                    let widget = indexed_word(menu, 0xAB + selected) as usize;
                    let enabled = platform::call1(FN_80173860, p32(widget));
                    if enabled == 0 || fn_801735b0(widget, 5) != 0 {
                        write_word(menu, 0x14, 6);
                        write_word(menu, 0x1C, read_word(menu, 0x620));
                    }
                    let elapsed = read_i32(menu, 0x9C);
                    write_word(menu, 0x9C, (elapsed + 1) as u32);
                    if read_i32(menu, 0x620) < elapsed + 1 {
                        write_word(menu, 0x14, 6);
                        write_word(menu, 0x1C, read_word(menu, 0x620));
                    }
                }
            }
        }
        7 => {
            if read_byte(menu, 0x92) != 0 {
                platform::call1(FN_801F3630, read_word(menu, 0x20C));
                fn_80173ca0(read_word(menu, 0x2BC) as usize);
                let control = read_word(menu, 0x250) as usize;
                for index in 0..8u32 {
                    call_f64x2_u32x6(
                        vtable_method(control, 0, 0x30),
                        slide,
                        slide,
                        p32(control),
                        index,
                        0,
                        0,
                        0,
                        0,
                    );
                }
                write_word(menu, 0x18, 6);
                activate_slot(menu, 0x10C);
                write_word(menu, 0x14, 8);
                let pane = indexed_word(menu, 0x114) as usize;
                write_word(pane, 0x18, 0);
                menu_sound_event(menu, 0x15);
            }
        }
        8 => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0x106 + index) as usize, 0x14) != 1 {
                if index == 0x0D {
                    activate_pair(menu, 0x11, 0x0C);
                    activate_pair(menu, 0x12, 0x0C);
                    set_pane_visible(layout_pane(menu, 0x238, S_T_DIALOG), false);
                } else if index == 6 {
                    set_pane_visible(layout_pane(menu, 0x238, S_T_MSG_00), false);
                    set_pane_visible(layout_pane(menu, 0x238, S_T_MSG_01), false);
                }
                write_word(menu, 0x14, 2);
            }
        }
        9 => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xBC + index) as usize, 0x14) != 1 {
                if read_byte(menu, 0x8C) != 0 {
                    for player in 0..4usize {
                        fn_801738f0(indexed_word(menu, 0xAB + player as i32) as usize);
                    }
                }
                write_word(menu, 0x14, 2);
            }
        }
        0x0A => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0x106 + index) as usize, 0x14) != 1 {
                write_word(menu, 0x1B, 0);
                write_word(menu, 0x1C, 0);
                write_word(menu, 0x1D, 0);
                write_word(menu, 0x1E, 0);
                if read_i32(menu, 0) == 1 {
                    write_word(menu, 0x18, 10);
                    activate_slot(menu, 0x110);
                    write_word(menu, 0x18, 1);
                    menu_sound_event(menu, 7);
                    write_word(menu, 0x14, 8);
                } else {
                    for index in 0..5usize {
                        let pane = layout_pane(
                            menu,
                            0x238,
                            platform::read_u32(S_B_OPTN_BTN_00 + index * 4) as usize,
                        );
                        set_pane_visible(pane, false);
                    }
                    write_word(menu, 0x14, 2);
                }
                platform::call1(FN_80179E60, p32(menu));
            }
        }
        0x0B => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xB0 + index) as usize, 0x14) != 1 {
                write_word(menu, 0x18, 7);
                activate_slot(menu, 0x10D);
                write_word(menu, 0x14, 0x0C);
            }
        }
        0x0C => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0x106 + index) as usize, 0x14) != 1 {
                platform::call1(FN_80179E60, p32(menu));
                menu_confirm_pending(menu);
                write_word(menu, 0x14, 2);
            }
        }
        0x0D => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xBC + index) as usize, 0x14) != 1 {
                if read_i32(menu, 0xB8) < 0 {
                    platform::call1(FN_80179E60, p32(menu));
                    write_word(menu, 0x18, 0x0D);
                    activate_slot(menu, 0x113);
                    write_word(menu, 0x14, 8);
                } else {
                    let numeric = read_i32(menu, 0x458) as f64;
                    write_word(menu, 0x45C, 1);
                    write_word(menu, 0x14, 0x13);
                    platform::write_f32(menu + 0x79C, numeric as f32);
                    let interface = read_word(menu, 4) as usize;
                    let callback = read_word(interface, 0x14) as usize;
                    if read_i32(menu, 0xB8) != 3 && callback != 0 {
                        let _ = platform::call2(callback, 3, numeric as i32 as u32);
                    }
                }
                let control = read_word(menu, 0x250) as usize;
                for index in 0..8u32 {
                    call_f64x2_u32x6(
                        vtable_method(control, 0, 0x30),
                        slide,
                        slide,
                        p32(control),
                        index,
                        0,
                        0,
                        0,
                        0,
                    );
                }
            }
        }
        0x0E => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0x106 + index) as usize, 0x14) != 1 {
                set_pane_visible(layout_pane(menu, 0x238, S_T_DIALOG), false);
                let index = if read_word(resource, 0x18) == 0 {
                    find_menu_pair(1, 1)
                } else {
                    find_menu_pair(3, 1)
                };
                write_word(menu, 0x18, index as u32);
                let pane = indexed_word(menu, 0xBC + index) as usize;
                let _ = platform::call1(FN_80171400, pane as u32);
                write_word(pane, 0x14, 1);
                write_word(menu, 0x14, 0x10);
                let value = read_f32(pane + 4);
                platform::write_f32(menu + 0x79C, value);
                let interface = read_word(menu, 4) as usize;
                let callback = read_word(interface, 0x14) as usize;
                if callback != 0 {
                    let _ = platform::call2(callback, 2, value as i32 as u32);
                }
            }
        }
        0x0F => {
            let index = read_i32(menu, 0x18);
            if read_word(indexed_word(menu, 0xB0 + index) as usize, 0x14) != 1 {
                let numeric = read_i32(menu, 0x458) as f64;
                write_byte(menu, 0x9A, 1);
                write_word(menu, 0x14, 0x13);
                platform::write_f32(menu + 0x79C, numeric as f32);
                let interface = read_word(menu, 4) as usize;
                let callback = read_word(interface, 0x14) as usize;
                if read_i32(menu, 0xB8) != 3 && callback != 0 {
                    let _ = platform::call2(callback, 3, numeric as i32 as u32);
                }
            }
        }
        0x10 => {
            // inferred: the export loses the `pcVar19` assignment; the
            // surrounding indexing and every other arm use the +0x2F0 pane.
            let index = read_i32(menu, 0x18);
            let pane = indexed_word(menu, 0xBC + index) as usize;
            if read_word(pane, 0x14) == 1 {
                let span = read_f32(pane + 4) - read_f32(pane + 0xC);
                let progress = (span / read_i32(menu, 0x79C) as f32) as f64;
                if read_i32(menu, 0xB8) != 3 {
                    if read_byte(menu, 0x97) != 0 {
                        let value = (read_f32(DAT_802F5DD8) as f64 * progress) as i32;
                        platform::call1(FN_801A1910, value as u32 & 0xFFFF);
                    }
                    fn_80174cc0(progress);
                }
            } else {
                write_word(menu, 0x14, 0x11);
                if read_i32(menu, 0xB8) != 3 {
                    if read_byte(menu, 0x97) != 0 {
                        platform::call1(FN_801A1910, 0);
                    }
                    fn_80174cc0(read_f32(DAT_802F5D90) as f64);
                }
            }
        }
        0x11 => {
            write_word(menu, 0x14, 0x12);
            if read_i32(menu, 0xB8) != 3 {
                fn_80174af0();
                if read_byte(menu, 0x97) != 0 {
                    platform::call1(FN_801A3420, p32(menu + 0x634));
                    platform::call2(
                        FN_801A0CA0,
                        read_word(menu, 0x794),
                        read_word(menu, 0x798),
                    );
                    platform::call2(
                        FN_801A9AF0,
                        read_word(menu, 0x62C),
                        read_word(menu, 0x630),
                    );
                    platform::call1(
                        FN_801A1910,
                        platform::read_u16(menu + 0x624) as u32,
                    );
                    platform::call1(
                        FN_801A1920,
                        platform::read_u16(menu + 0x626) as u32,
                    );
                    platform::call1(
                        FN_801A1930,
                        platform::read_u16(menu + 0x628) as u32,
                    );
                }
                write_byte(menu, 0x97, 0);
            }

            let level = read_i32(menu, 0x84) as f32 * read_f32(DAT_802F5DDC);
            platform::call1(FN_801E3460, level as u32 & 0xFF);
            platform::call1(FN_801E1820, 0);
            fn_80173ca0(read_word(menu, 0x2BC) as usize);
            fn_80173f20(read_word(menu, 0x2BC) as usize);

            for player in 0..4usize {
                let widget = indexed_word(menu, 0xAB + player as i32) as usize;
                fn_801738f0(widget);
                fn_80173290(widget);
            }

            let interface = read_word(menu, 4) as usize;
            let callback = read_word(interface, 0x14) as usize;
            if read_i32(menu, 0xB8) != 3 && callback != 0 {
                let _ = platform::call2(callback, 4, 0);
            }

            for player in 0..4i32 {
                clear_pair(menu, player + 0x1F, 0x11);
                clear_pair(menu, player + 0x1F, 0x12);
                clear_pair(menu, player + 0x1F, 0x0F);
            }
            write_byte(menu, 0x94, 0);
        }
        0x12 => {
            write_word(menu, 0x14, 2);
        }
        0x13 => {
            if read_byte(menu, 0x95) != 0 {
                platform::call2(FN_80176740, p32(menu), p32(records));
                write_byte(menu, 0x95, 0);
            }
            if read_byte(menu, 0x96) != 0 {
                platform::call1(FN_80176410, p32(menu));
                let cursor = call0_r(FN_801E15F0);
                write_byte(menu, 0x8C, if cursor != 0 { 1 } else { 0 });
                if cursor != 0 {
                    activate_pair(menu, 0x0D, 6);
                    activate_pair(menu, 0x0E, 8);
                } else {
                    activate_pair(menu, 0x0D, 8);
                    activate_pair(menu, 0x0E, 6);
                }
                write_byte(menu, 0x96, 0);
            }

            let finished = (read_i32(menu, 0x45C) == 1
                && read_i32(menu, 0x454) == read_i32(menu, 0x458))
                || (read_i32(menu, 0x45C) == 2 && read_i32(menu, 0x454) == 0);
            let mut skip = false;
            if finished {
                if read_byte(menu, 0x98) != 0 {
                    if read_byte(menu, 0x92) == 0 {
                        skip = true;
                    } else {
                        platform::call1(FN_801F3630, read_word(menu, 0x20C));
                        write_byte(menu, 0x98, 0);
                    }
                }
                if !skip {
                    if read_byte(menu, 0x99) != 0 {
                        let pane = indexed_word(menu, 0x10B) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 0);
                        set_pane_visible(layout_pane(menu, 0x238, S_T_MSG_00), false);
                        set_pane_visible(layout_pane(menu, 0x238, S_T_MSG_01), false);
                        let dialog = indexed_word(menu, 0x114) as usize;
                        if read_word(dialog, 0x14) == 1 {
                            let _ = platform::call1(FN_80171400, p32(dialog));
                            write_word(dialog, 0x14, 0);
                        }
                    }
                    write_word(menu, 0x14, 0x11);
                    let mut value = 0u32;
                    if read_i32(menu, 0x618) != 0 || read_byte(menu, 0x460) == 0 {
                        value = 1;
                    }
                    platform::call1(FN_801BDE50, value);
                    platform::call0(FN_801BDCB0);
                    if read_i32(menu, 0xB8) != 3 {
                        if read_byte(menu, 0x97) != 0 {
                            platform::call1(FN_801A1910, 0);
                        }
                        fn_80174cc0(read_f32(DAT_802F5D90) as f64);
                    }
                }
            } else {
                let difference = read_i32(menu, 0x458).wrapping_sub(read_i32(menu, 0x454)) as u32;
                let progress = (difference as f32 / read_i32(menu, 0x79C) as f32) as f64;
                if read_i32(menu, 0xB8) != 3 {
                    if read_byte(menu, 0x97) != 0 {
                        let value = (read_f32(DAT_802F5DD8) as f64 * progress) as i32;
                        platform::call1(FN_801A1910, value as u32 & 0xFFFF);
                    }
                    fn_80174cc0(progress);
                }
            }
        }
        _ => {}
    }

    let pending = read_i32(menu, 0x1B);
    if pending != 0 {
        let mut apply = true;
        if read_i32(menu, 0x14) != 2
            || read_word(indexed_word(menu, 0x112) as usize, 0x14) == 1
            || read_word(indexed_word(menu, 0x109) as usize, 0x14) == 1
        {
            apply = false;
        }
        if apply {
            if pending != read_i32(menu, 0x1D) {
                let pane = indexed_word(menu, 0x106 + pending) as usize;
                let _ = platform::call1(FN_80171400, pane as u32);
                write_word(pane, 0x14, 1);
                write_word(menu, 0x1D, read_word(menu, 0x1B));
            }
            write_word(menu, 0x1B, 0);
        }
    }

    let pending = read_i32(menu, 0x1C);
    if pending != 0 {
        let first = find_menu_pair(4, 2);
        let second = find_menu_pair(5, 3);
        let third = find_menu_pair(4, 0x13);
        let fourth = find_menu_pair(5, 0x14);
        let mut apply = true;
        if read_i32(menu, 0x14) != 2
            || read_word(indexed_word(menu, 0xBC + first) as usize, 0x14) == 1
            || read_word(indexed_word(menu, 0xBC + second) as usize, 0x14) == 1
            || read_word(indexed_word(menu, 0xBC + third) as usize, 0x14) == 1
            || read_word(indexed_word(menu, 0xBC + fourth) as usize, 0x14) == 1
        {
            apply = false;
        }
        if apply {
            if pending != read_i32(menu, 0x1E) {
                let pane = indexed_word(menu, 0xBC + pending) as usize;
                let _ = platform::call1(FN_80171400, pane as u32);
                write_word(pane, 0x14, 1);
                write_word(menu, 0x1E, read_word(menu, 0x1C));
            }
            write_word(menu, 0x1C, 0);
        }
    }

    if records != 0 {
        fn_80178c50(menu, records);
    }

    let root = read_word(menu, 0x238) as usize;
    platform::call2(vtable_method(root, 0, 0x2C), p32(root), 0);
    platform::call2(vtable_method(root, 0, 0x24), p32(root), p32(menu + 0x258));
    if read_word(resource, 0x20) == 0 {
        for player in 0..4usize {
            let group = indexed_word(menu, 0x8F + player as i32) as usize;
            platform::call2(vtable_method(group, 0, 0x24), p32(group), p32(menu + 0x258));
        }
    }

    if read_i32(menu, 0x68) < 3 {
        write_word(menu, 0x68, read_word(menu, 0x68) + 1);
    }

    // observed: 0x802DCDB8 is the matching register-restore thunk; omitted like
    // the other save/restore helpers in this module.
}

/// Arms the `+0x5E8` timer used by state 4 of [`fn_801769b0`].
///
/// # Original
/// Host-side factoring of the state-4 tail in
/// `work/ghidra/main-gb/export/801769b0_FUN_801769b0.c`; no binary address of
/// its own.
///
/// # Safety
/// `menu` must be the live menu and `+0x20C` a valid release callback.
///
/// # Confidence
/// medium - the five setup stores and the timer arguments are literal.
unsafe fn arm_option_page(menu: usize) {
    write_word(menu, 0x14, 5);
    write_word(menu, 0x1C, 0);
    write_word(menu, 0x9C, 0);
    let release = platform::call1(FN_801F3630, 0x8017_8C10);
    write_word(menu, 0x20C, release);
    write_byte(menu, 0x92, 0);
    write_byte(menu, 0x93, 0);
    for player in 0..4usize {
        write_byte(indexed_word(menu, 0xAB + player as i32) as usize, 0x44, 1);
    }
    let deferred = call0_r(FN_801F3840);
    write_byte(menu, 0x91, if deferred != 0 { 1 } else { 0 });
    if deferred == 0 {
        arm_scroll_timer(menu, 0);
    }
}

/// Arms the `+0x5E8` scrolling-text timer with one id.
///
/// # Original
/// Host-side factoring of the two timer blocks in
/// `work/ghidra/main-gb/export/801769b0_FUN_801769b0.c`; no binary address of
/// its own.
///
/// # Safety
/// The timer object at `menu + 0x5E8` must be initialized.
///
/// # Confidence
/// high - the four calls and the 100 ms tick count are literal.
unsafe fn arm_scroll_timer(menu: usize, id: u32) {
    let timer = menu + 0x5E8;
    platform::call1(FN_801AB640, p32(timer));
    platform::call2(FN_801ABA70, p32(timer), id);
    let clock = platform::read_u32(G_OS_BUS_CLOCK);
    platform::call5(
        FN_801AB540,
        p32(timer),
        0x1062_4DD3,
        0,
        (clock / 4000) * 100,
        fn_80178b40 as usize as u32,
    );
}

/// Updates the four per-player scrolling-text widgets and the battery panes.
///
/// # Original
/// - Ghidra: `FUN_80178c50` at `0x80178C50` (2388 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80178c50_FUN_80178c50.c`
/// - observed: the entry calls `FUN_802dcd40` (`__savegprlr_25`) and keeps its
///   `r3`/`r4` pair: `r3` is the menu (fields `+0x45C`, `+0x2F0`) and `r4` the
///   four-record array advanced by `0x10`.
///
/// # Arguments
/// * `menu` - home-button menu.
/// * `records` - four 0x10-byte records; word 0 is the widget state and word 3
///   the page mode.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Advances the scroll/fade fields, starts and clears panes in the `+0x2F0`
/// array, updates the four widget scales, and advances the `+0x7C` frame
/// counter. The two per-player timers run callbacks `0x80178B10` and
/// `0x80178A20`.
///
/// # Safety
/// `menu` must be the live menu, `records` must hold four initialized records,
/// and every pane name must exist in the layout.
///
/// # Confidence
/// medium - every branch and store is literal; `0x80180020` is modelled with a
/// four-float rectangle, and the GQR conversion block at the entry only feeds
/// five unused locals, so it is omitted.
pub unsafe fn fn_80178c50(menu: usize, records: usize) {
    let direction = read_i32(menu, 0x45C);
    let mut current = read_i32(menu, 0x454);
    if direction == 1 {
        current += 1;
    } else if direction == 2 {
        current -= 1;
    }
    if current < 0 {
        current = 0;
    } else if read_i32(menu, 0x458) < current {
        current = read_i32(menu, 0x458);
    }
    write_word(menu, 0x454, current as u32);

    let resource = read_word(menu, 4) as usize;
    let frame_owner = read_word(DAT_80377FB8, 4) as usize;
    let frame_delta = read_f32(frame_owner + 0x30) as f64;
    let frames = (read_f32(DAT_802F5DCC) as f64
        + (read_f32(DAT_802F5DE4) as f64 / frame_delta) as f32 as f64)
        as i32;
    let slide = read_f32(DAT_802F5DAC) as f64;

    for player in 0..4usize {
        let record = records + player * 0x10;
        let slice = menu + player * 4;
        let widget = indexed_word(menu, 0xAB + player as i32) as usize;
        let mut state = read_word(record, 0) as usize;
        let record_mode = read_i32(record, 0xC);

        if state == 0 {
            if read_byte(menu, 0x80 + player) != 0 {
                activate_pair(menu, 0x1F + player as i32, 0x11);
                activate_pair(menu, 0x1F + player as i32, 0x12);

                for index in 0..4usize {
                    let name = platform::read_u32(S_BTRY_PWR + (player * 4 + index) * 4) as usize;
                    set_pane_visible(layout_pane(menu, 0x238, name), false);
                }
                if read_word(resource, 0x20) == 0 {
                    set_pane_visible(layout_pane(menu, 0x23C, S_N_TRANS), false);
                }

                // observed: `iVar18 + 0x58` is the per-player hold timer.
                write_word(slice, 0x58, 0);
                write_byte(menu, 0x80 + player, 0);
                fn_801734c0(widget);
                fn_801734a0(widget);
                fn_801734f0();
            }

            // observed: the export's `iVar11 < 0` arm is unreachable for this
            // 0..4 loop; the other arm slides the control object by `slide`.
            let control = read_word(menu, 0x250) as usize;
            call_f64x2_u32x6(
                vtable_method(control, 0, 0x30),
                slide,
                slide,
                p32(control),
                player as u32,
                0,
                0,
                0,
                0,
            );
        } else {
            let phase = read_byte(state, 0x5D) as i8;
            if phase == -1 {
                fn_801734c0(widget);
                if read_word(resource, 0x20) == 0 {
                    set_pane_visible(layout_pane(menu, 0x23C, S_N_TRANS), false);
                }
            } else {
                let hold = read_i32(slice, 0x58);
                if frames < hold {
                    if phase == 0 || phase == -7 {
                        let flag = if record_mode == 2 && read_byte(state, 0x5C) as i8 == 2 {
                            1u32
                        } else if (read_byte(state, 0x5E) as i8) < 1 {
                            0
                        } else {
                            1
                        };
                        fn_80173310(widget, record, flag);
                        if read_word(resource, 0x20) == 0 {
                            set_pane_visible(layout_pane(menu, 0x23C, S_N_TRANS), true);
                        }
                    }
                } else {
                    write_word(slice, 0x58, hold as u32 + 1);
                }

                if (record_mode != 2 || read_byte(state, 0x5C) as i8 != 2)
                    && (read_byte(state, 0x5E) as i8) < 1
                {
                    let mut buffer = [0u32; 4];
                    let status = platform::call2(
                        FN_801E0F40,
                        player as u32,
                        p32(buffer.as_mut_ptr() as usize),
                    ) as i32;
                    if read_byte(state, 0x5D) as i8 != -2 && status != -2 {
                        fn_801734c0(widget);
                    }
                }
            }

            if read_byte(menu, 0x80 + player) == 0 {
                write_word(menu, 0x88, player as u32);
                write_byte(menu, 0x80 + player, 1);
                let _ = fn_80173920(widget, (menu + 0x1AC + player * 0x18) as u32);

                activate_pair(menu, 0x1F + player as i32, 0x11);
                clear_pair(menu, 0x1F + player as i32, 0x12);
                activate_pair(menu, 0x1F + player as i32, 0x0F);

                menu_sound_event(menu, player as u32 + 0x11);
                fn_801734e0(widget);
                fn_80173880(widget);

                let timer = menu + 0x468 + player * 0x30;
                platform::call2(FN_801ABA70, p32(timer), p32(widget));
                platform::call1(FN_801AB640, p32(timer));
                let clock = platform::read_u32(G_OS_BUS_CLOCK);
                platform::call5(
                    FN_801AB540,
                    p32(timer),
                    0x8018_0000,
                    0,
                    (clock / 4000) * 300,
                    fn_80178b10 as usize as u32,
                );

                // observed: the second timer id is `player << 16 | (player + 2)`.
                let timer2 = menu + 0x528 + player * 0x30;
                let id = ((player as u32) << 16) | (player as u32 + 2);
                platform::call2(FN_801ABA70, p32(timer2), id);
                platform::call1(FN_801AB640, p32(timer2));
                platform::call5(
                    FN_801AB540,
                    p32(timer2),
                    0x8018_0000,
                    0,
                    (clock / 4000) * 400,
                    fn_80178a20 as usize as u32,
                );
            }

            state = read_word(record, 0) as usize;
            let phase = read_byte(state, 0x5D) as i8;
            if phase == 0 || phase == -7 {
                let z = if record_mode == 2 && read_byte(state, 0x5C) as i8 == 2 {
                    read_f32(DAT_802F5DE8)
                } else {
                    let angle = call_f64x2(
                        FN_801886A0,
                        -read_f32(state + 0x38) as f64,
                        read_f32(state + 0x34) as f64,
                    );
                    (read_f32(DAT_802F5DEC) as f64 * angle) as f32
                };

                if read_word(resource, 0x20) == 0 {
                    let pane = layout_pane(menu, 0x23C, S_N_ROT);
                    write_word(pane, 0x38, 0);
                    write_word(pane, 0x3C, 0);
                    platform::write_f32(pane + 0x40, z);
                    let pane = layout_pane(menu, 0x23C, S_N_SROT);
                    write_word(pane, 0x38, 0);
                    write_word(pane, 0x3C, 0);
                    platform::write_f32(pane + 0x40, z);
                }

                if read_i32(menu, 0x7C) > 100 {
                    let _ = fn_80173920(widget, (menu + 0x1AC + player * 0x18) as u32);
                }

                fn_801795b0(menu, player as u32);

                if read_word(resource, 0x20) == 0 {
                    let widget_record = fn_80173870(widget);
                    let mut rect = [0f32; 4];
                    platform::call2(
                        FN_80180020,
                        p32(rect.as_mut_ptr() as usize),
                        read_word(menu, 0x238),
                    );
                    let pane = layout_pane(menu, 0x23C, S_N_TRANS);
                    platform::write_f32(pane + 0x2C, read_f32(widget_record + 8) * rect[2]);
                    platform::write_f32(pane + 0x30, read_f32(widget_record + 0xC) * rect[3]);
                    platform::write_f32(pane + 0x34, read_f32(DAT_802F5D90));
                }
            }

            if fn_801739e0(widget) != 0 {
                platform::call2(FN_801786B0, p32(menu), player as u32);
            }
        }
    }

    let frame = read_i32(menu, 0x7C);
    if frame < 0x65 {
        write_word(menu, 0x7C, frame as u32 + 1);
    } else {
        write_word(menu, 0x7C, 0);
    }
}

/// Handles the scrolling-text input and page changes for one player.
///
/// # Original
/// - Ghidra: `FUN_801795b0` at `0x801795B0` (2212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801795b0_FUN_801795b0.c`
///
/// # Arguments
/// * `menu` - home-button menu.
/// * `player` - player index 0..3.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Slides the control object, changes the "scroll index" at `+0x84`, starts
/// and clears `0x802F5B18` panes, plays sound ids 1..0x0F, and updates the
/// four widget scales.
///
/// # Safety
/// `menu` must be live, `player < 4`, and every referenced pane and widget
/// must be initialized.
///
/// # Confidence
/// medium - the state machine, table pairs and sound ids are literal; the
/// meaning of the state byte at `record + 0x5D` is inferred.
pub unsafe fn fn_801795b0(menu: usize, player: u32) {
    let resource = read_word(menu, 4) as usize;

    if read_i32(menu, 0x14) == 2 {
        let widget = indexed_word(menu, 0xAB + player as i32) as usize;
        let record = fn_80173870(widget);

        let mut x = (read_f32(DAT_802F5DF0) * read_f32(record + 8)
            * read_f32(DAT_802F5DCC)) as f64;
        let mut y = (read_f32(DAT_802F5DF4) * read_f32(record + 0xC)
            * read_f32(DAT_802F5DCC)) as f64;
        if read_byte(menu, 0x8F) != 0 {
            x = (x as f32 * read_f32(resource + 0x34)) as f64;
            y = (y as f32 * read_f32(resource + 0x38)) as f64;
        }

        let control = read_word(menu, 0x250) as usize;
        call_f64x2_u32x6(
            vtable_method(control, 0, 0x30),
            x,
            -y,
            p32(control),
            player,
            read_word(record, 0x10),
            read_word(record, 0x14),
            read_word(record, 0x18),
            p32(record),
        );

        let status = read_word(record, 0x10);
        if (status & 0x8000) == 0 || read_i32(menu, 0x14) != 2 {
            if read_i32(menu, 0) == 1 && read_i32(menu, 0x14) == 2 {
                if (status & 0x1000) == 0 {
                    if (status & 0x10) != 0 {
                        let current = read_i32(menu, 0x84);
                        if current < 10 {
                            clear_pair(menu, current + 0x15, 9);
                            activate_pair(menu, current + 0x15, 10);
                            write_word(menu, 0x84, (current + 1) as u32);
                            activate_pair(menu, 0x0C, 5);

                            if read_i32(menu, 0x84) == 10 {
                                menu_sound_event(menu, 0xB);
                                scroll_progress(menu, 10);
                            } else {
                                menu_sound_event(menu, 9);
                                scroll_progress(menu, read_i32(menu, 0x84) as u32);
                            }
                        } else {
                            menu_sound_event(menu, 0xD);
                        }
                    }
                } else {
                    let current = read_i32(menu, 0x84);
                    if current < 1 {
                        menu_sound_event(menu, 0xD);
                    } else {
                        write_word(menu, 0x84, (current - 1) as u32);
                        clear_pair(menu, current + 0x14, 10);
                        activate_pair(menu, read_i32(menu, 0x84) + 0x15, 9);
                        activate_pair(menu, 0x0B, 5);

                        if read_i32(menu, 0x84) == 0 {
                            menu_sound_event(menu, 0xC);
                            scroll_progress(menu, 0);
                        } else {
                            menu_sound_event(menu, 10);
                            scroll_progress(menu, read_i32(menu, 0x84) as u32);
                        }
                    }
                }
            }
        } else if read_i32(menu, 0) == 1 {
            call_f64x2_u32x6(
                vtable_method(control, 0, 0x30),
                read_f32(DAT_802F5D90) as f64,
                read_f32(DAT_802F5DF8) as f64,
                p32(control),
                player,
                0,
                0,
                0,
                0,
            );
            activate_slot(menu, 0x10A);
            write_word(menu, 0x18, 2);
            activate_slot(menu, 0x108);
            activate_slot(menu, 0x111);
            write_word(menu, 0x14, 10);
            write_word(menu, 0, 0);
            menu_sound_event(menu, 8);
        } else if read_i32(menu, 0) == 0 {
            if read_word(indexed_word(menu, 0x109) as usize, 0x14) == 1 {
                clear_slot(menu, 0x109);
            }
            if read_word(indexed_word(menu, 0x112) as usize, 0x14) == 1 {
                clear_slot(menu, 0x112);
            }
            write_word(menu, 0xB8, 0);
            write_word(menu, 0x18, 4);
            activate_slot(menu, 0x10A);
            write_word(menu, 0x14, 0xE);
            menu_sound_event(menu, 1);
        }
    } else if read_i32(menu, 0) == 1 && read_i32(menu, 0x14) == 5 {
        let slot = indexed_word(menu, 0x106 + read_i32(menu, 0x18)) as usize;
        if read_word(slot, 0x14) != 1 {
            let widget = indexed_word(menu, 0xAB + player as i32) as usize;
            if read_word(fn_80173870(widget), 0x10) != 0 {
                write_word(menu, 0x1C, read_word(menu, 0x61C));
            }
        }
    }
}

/// Handles a home-button page transition on button release.
///
/// # Original
/// - Ghidra: `FUN_8017a120` at `0x8017A120` (3660 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017a120_FUN_8017a120.c`
/// - observed: the export prints `FUN_802dcd70()` and `FUN_8017cf10()` with no
///   arguments; the body uses one pointer as the menu and a second object as
///   the button table, so both are kept as explicit parameters.
///
/// # Arguments
/// * `menu` - home-button menu.
/// * `button_table` - object whose `+0xC` word is the button count.
/// * `selection` - optional pointer to the player index used at the end.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Activates the pane selected by the `0x802F5B18` pair, advances the pending
/// counters, plays sound id 4 on every transition, and may start a widget.
///
/// # Safety
/// `menu` and `button_table` must be live; `selection`, when non-zero, must
/// point to a readable player index.
///
/// # Confidence
/// medium - the switch and table walks are literal; the argument roles are
/// inferred from the two register-save calls.
pub unsafe fn fn_8017a120(menu: usize, button_table: usize, selection: usize) {
    let selected = menu_button_index(button_table, menu);
    let mut changed = false;

    if read_i32(menu, 0x14) == 2
        && selected != -1
        && read_i32(menu, (selected + 8) as usize * 4) == 0
    {
        let mode = read_i32(menu, 0);
        let count = read_i32(menu, 0x0C);

        if mode == 2 || count <= selected {
            match selected - count {
                0 => {
                    if mode == 0 {
                        let blocked = read_word(indexed_word(menu, 0x112) as usize, 0x14) == 1
                            || read_word(indexed_word(menu, 0x109) as usize, 0x14) == 1;
                        if !blocked {
                            activate_slot(menu, 0x109);
                            write_word(menu, 0x1D, 3);
                            write_word(menu, 0x1B, 0);
                            if read_i32(menu, 0x1A) > 2 {
                                menu_sound_event(menu, 4);
                            }
                            write_word(menu, 0x1A, 0);
                            changed = true;
                        } else {
                            write_word(menu, 0x1B, 3);
                        }
                    }
                }
                1 | 9 => {
                    if mode == 0 {
                        let first = find_menu_pair(4, 2);
                        let second = find_menu_pair(4, 2);
                        let third = find_menu_pair(5, 3);
                        let fourth = find_menu_pair(4, 0x13);
                        let fifth = find_menu_pair(5, 0x14);
                        let blocked = read_word(
                            indexed_word(menu, 0xBC + second) as usize,
                            0x14,
                        ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + third) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fourth) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fifth) as usize,
                                0x14,
                            ) == 1;
                        if !blocked {
                            let pane = indexed_word(menu, 0xBC + first) as usize;
                            let _ = platform::call1(FN_80171400, pane as u32);
                            write_word(pane, 0x14, 1);
                            write_word(menu, 0x1E, first as u32);
                            write_word(menu, 0x1C, 0);
                            if read_i32(menu, 0x1A) > 2 {
                                menu_sound_event(menu, 4);
                            }
                            write_word(menu, 0x1A, 0);
                            changed = true;
                        } else {
                            write_word(menu, 0x1C, first as u32);
                        }
                    } else if mode == 1 {
                        let first = find_menu_pair(5, 0x14);
                        let second = find_menu_pair(4, 2);
                        let third = find_menu_pair(5, 3);
                        let fourth = find_menu_pair(4, 0x13);
                        let fifth = find_menu_pair(5, 0x14);
                        let blocked = read_word(
                            indexed_word(menu, 0xBC + second) as usize,
                            0x14,
                        ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + third) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fourth) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fifth) as usize,
                                0x14,
                            ) == 1;
                        if !blocked {
                            let pane = indexed_word(menu, 0xBC + first) as usize;
                            let _ = platform::call1(FN_80171400, pane as u32);
                            write_word(pane, 0x14, 1);
                            write_word(menu, 0x1E, first as u32);
                            write_word(menu, 0x1C, 0);
                            if read_i32(menu, 0x1A) > 2 {
                                menu_sound_event(menu, 4);
                            }
                            write_word(menu, 0x1A, 0);
                            changed = true;
                        } else {
                            write_word(menu, 0x1C, first as u32);
                        }
                    }
                }
                2..=6 => {
                    if mode == 1 {
                        let first = find_menu_pair(selected - count + 4, 4);
                        let pane = indexed_word(menu, 0xBC + first) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 1);
                        if read_i32(menu, 0x1A) > 2 {
                            menu_sound_event(menu, 4);
                        }
                        write_word(menu, 0x1A, 0);
                        changed = true;
                    }
                }
                7 => {
                    if mode == 2 {
                        let first = find_menu_pair(0x11, 0xB);
                        let pane = indexed_word(menu, 0xBC + first) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 1);
                        if read_i32(menu, 0x1A) > 2 {
                            menu_sound_event(menu, 4);
                        }
                        write_word(menu, 0x1A, 0);
                        changed = true;
                    }
                }
                8 => {
                    if mode == 2 {
                        let first = find_menu_pair(0x12, 0xB);
                        let pane = indexed_word(menu, 0xBC + first) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 1);
                        if read_i32(menu, 0x1A) > 2 {
                            menu_sound_event(menu, 4);
                        }
                        write_word(menu, 0x1A, 0);
                        changed = true;
                    }
                }
                _ => {}
            }
        } else {
            let index = find_entry_pair(menu, selected, 0);
            let pane = indexed_word(menu, 0xB0 + index) as usize;
            let _ = platform::call1(FN_80171400, pane as u32);
            write_word(pane, 0x14, 1);
            if read_i32(menu, 0) == 0 {
                if read_i32(menu, 0x1A) > 2 {
                    menu_sound_event(menu, 4);
                }
                write_word(menu, 0x1A, 0);
                changed = true;
            }
        }
    }

    let count = read_i32(menu, 0x0C);
    if selected == count + 1 || selected == count + 9 {
        let first = count + 9;
        set_indexed_word(menu, first, indexed_word(menu, first) + 1);
        let second = count + 0x11;
        set_indexed_word(menu, second, indexed_word(menu, second) + 1);
    } else {
        let pending = selected + 8;
        set_indexed_word(menu, pending, indexed_word(menu, pending) + 1);
    }

    if changed && selection != 0 {
        let player = read_i32(selection, 0);
        let widget = indexed_word(menu, player + 0xAB) as usize;
        if read_byte(widget, 0x1C) == 0
            && read_f32(menu + (player + 0x88) as usize * 4) <= read_f32(DAT_802F5D90)
        {
            platform::write_u32(
                menu + (player + 0x84) as usize * 4,
                platform::read_u32(DAT_802F5DFC),
            );
            fn_80173880(widget);
        }
    }

    // observed: 0x802DCDBC is the matching register-restore thunk; omitted like
    // the other save/restore helpers in this module.
}

/// Handles a dialog page transition on button release.
///
/// # Original
/// - Ghidra: `FUN_8017af70` at `0x8017AF70` (2764 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017af70_FUN_8017af70.c`
/// - observed: `FUN_8017cf10()` is printed with no arguments and its result is
///   compared against the menu's `+0x0C` button count, so the menu doubles as
///   the button table.
///
/// # Arguments
/// * `menu` - dialog/menu object; `+0x0C` is the button count and `+0x14` the
///   page state.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Activates or clears panes in the `+0x2F0` array, writes the pending words at
/// `+0x6C..+0x78`, and activates one `+0x2C0` entry pane in the entry branch.
///
/// # Safety
/// `menu` must be live and every resolved pane must exist.
///
/// # Confidence
/// medium - all switch arms and stores are literal; the argument mapping of
/// `0x8017CF10` is inferred from the `+0x0C` count comparison.
pub unsafe fn fn_8017af70(menu: usize) {
    let selected = menu_button_index(menu, menu);
    let count = read_i32(menu, 0x0C);

    if read_i32(menu, (selected + 8) as usize * 4) > 0 {
        if selected == count + 1 || selected == count + 9 {
            let first = count + 9;
            set_indexed_word(menu, first, indexed_word(menu, first) - 1);
            let second = count + 0x11;
            set_indexed_word(menu, second, indexed_word(menu, second) - 1);
        } else {
            let pending = selected + 8;
            set_indexed_word(menu, pending, indexed_word(menu, pending) - 1);
        }
    }

    if read_i32(menu, 0x14) == 2 && selected != -1 && read_i32(menu, (selected + 8) as usize * 4) == 0 {
        let mode = read_i32(menu, 0);
        if mode == 2 || count <= selected {
            match selected - count {
                0 => {
                    if mode == 0 {
                        let blocked = read_word(indexed_word(menu, 0x112) as usize, 0x14) == 1
                            || read_word(indexed_word(menu, 0x109) as usize, 0x14) == 1;
                        if !blocked {
                            activate_slot(menu, 0x112);
                            write_word(menu, 0x1D, 0xC);
                            write_word(menu, 0x1B, 0);
                        } else {
                            write_word(menu, 0x1B, 0xC);
                        }
                    }
                }
                1 | 9 => {
                    if mode == 0 {
                        let first = find_menu_pair(5, 3);
                        let second = find_menu_pair(4, 2);
                        let third = find_menu_pair(5, 3);
                        let fourth = find_menu_pair(4, 0x13);
                        let fifth = find_menu_pair(5, 0x14);
                        let blocked = read_word(
                            indexed_word(menu, 0xBC + second) as usize,
                            0x14,
                        ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + third) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fourth) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fifth) as usize,
                                0x14,
                            ) == 1;
                        if !blocked {
                            let pane = indexed_word(menu, 0xBC + first) as usize;
                            let _ = platform::call1(FN_80171400, pane as u32);
                            write_word(pane, 0x14, 1);
                            write_word(menu, 0x1E, first as u32);
                            write_word(menu, 0x1C, 0);
                        } else {
                            write_word(menu, 0x1C, first as u32);
                        }
                    } else if mode == 1 {
                        let first = find_menu_pair(5, 0x14);
                        let second = find_menu_pair(4, 2);
                        let third = find_menu_pair(5, 3);
                        let fourth = find_menu_pair(4, 0x13);
                        let fifth = find_menu_pair(5, 0x14);
                        let blocked = read_word(
                            indexed_word(menu, 0xBC + second) as usize,
                            0x14,
                        ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + third) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fourth) as usize,
                                0x14,
                            ) == 1
                            || read_word(
                                indexed_word(menu, 0xBC + fifth) as usize,
                                0x14,
                            ) == 1;
                        if !blocked {
                            let pane = indexed_word(menu, 0xBC + first) as usize;
                            let _ = platform::call1(FN_80171400, pane as u32);
                            write_word(pane, 0x14, 1);
                            write_word(menu, 0x1E, first as u32);
                            write_word(menu, 0x1C, 0);
                        } else {
                            write_word(menu, 0x1C, first as u32);
                        }
                    }
                }
                2..=6 => {
                    if mode == 1 {
                        let first = find_menu_pair(selected - count + 4, 7);
                        let pane = indexed_word(menu, 0xBC + first) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 1);
                    }
                }
                7 => {
                    if mode == 2 {
                        let first = find_menu_pair(0x11, 0xC);
                        let pane = indexed_word(menu, 0xBC + first) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 1);
                    }
                }
                8 => {
                    if mode == 2 {
                        let first = find_menu_pair(0x12, 0xC);
                        let pane = indexed_word(menu, 0xBC + first) as usize;
                        let _ = platform::call1(FN_80171400, pane as u32);
                        write_word(pane, 0x14, 1);
                    }
                }
                _ => {}
            }
        } else {
            let index = find_entry_pair(menu, selected, 2);
            let pane = indexed_word(menu, 0xB0 + index) as usize;
            let _ = platform::call1(FN_80171400, pane as u32);
            write_word(pane, 0x14, 1);
        }
    }
}

/// Runs the option and dialog page state machine.
///
/// # Original
/// - Ghidra: `FUN_8017ba40` at `0x8017BA40` (4692 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8017ba40_FUN_8017ba40.c`
/// - observed: `FUN_8017cf10()` supplies the selected button, `menu + 0x0C` is
///   the button count and `menu + 0x14` the page state.
///
/// # Arguments
/// * `menu` - home-button menu.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds the dialog text line, toggles the option list, moves the scroll
/// index, plays sound ids 1..0x10, and updates the four widgets.
///
/// # Safety
/// `menu` must be live, its layout and pane arrays initialized, and the dialog
/// text table at `+0xBC` valid for the selected entry.
///
/// # Confidence
/// medium - the branch tree and stores are literal; the text scans use the two
/// terminators found in the export, and the `+0x8C` byte role is inferred.
pub unsafe fn fn_8017ba40(menu: usize) {
    let selected = menu_button_index(menu, menu);
    let resource = read_word(menu, 4) as usize;

    if read_i32(menu, 0x14) == 2 && selected != -1 {
        let mode = read_i32(menu, 0);
        let count = read_i32(menu, 0x0C);

        if mode == 0 && selected < count {
            let pair_count = read_i32(menu, 0x10);
            write_word(menu, 0xB8, (selected + 1) as u32);
            let mut index = -1;
            let mut k = 0;
            while k < pair_count {
                let pair = TABLE_802F5AB8 + k as usize * 8;
                if read_i32(pair, 0) == selected + 4 && read_i32(pair, 4) == 1 {
                    index = k;
                    break;
                }
                k += 1;
            }
            write_word(menu, 0x18, index as u32);
            activate_slot(menu, 0xB0 + index);
            menu_sound_event(menu, 5);

            if read_i32(menu, (selected + 0x28) as usize * 4) == 0 {
                write_word(menu, 0x14, 0x0F);
            } else {
                write_word(menu, 0x14, 0x0B);
                write_word(menu, 0, 2);

                let pane = layout_pane(menu, 0x238, S_T_DIALOG);
                let pane = if list_contains_marker(pane) { pane } else { 0 };
                let text_offset = 0xBC
                    + (selected as usize + 2) * 4
                    + read_word(resource, 0x1C) as usize * 0x18;
                let text = read_word(menu, text_offset) as usize;
                let length = if read_word(resource, 0x24) & (selected as u32 + 1) == 0 {
                    text_length_after_open_quote(text)
                } else {
                    text_length_to_menu_terminator(text)
                } + 1;
                if pane != 0 {
                    let _ = platform::call4(
                        vtable_method(pane, 0, 0x70),
                        p32(pane),
                        p32(text),
                        0,
                        length,
                    );
                }
                set_pane_visible(layout_pane(menu, 0x238, S_T_DIALOG), true);
            }
        } else {
            match selected - count {
                0 => {
                    if read_word(indexed_word(menu, 0x112) as usize, 0x14) == 1 {
                        clear_slot(menu, 0x112);
                    }
                    if read_word(indexed_word(menu, 0x109) as usize, 0x14) == 1 {
                        clear_slot(menu, 0x109);
                    }
                    write_word(menu, 0xB8, 0);
                    write_word(menu, 0x18, 4);
                    activate_slot(menu, 0x10A);
                    write_word(menu, 0x14, 0x0E);
                    menu_sound_event(menu, 1);
                }
                1 | 9 => {
                    if mode == 1 {
                        activate_slot(menu, 0x10A);
                        activate_slot(menu, 0x111);
                        write_word(menu, 0x18, 2);
                        activate_slot(menu, 0x108);
                        write_word(menu, 0x14, 10);
                        write_word(menu, 0, 0);
                        menu_sound_event(menu, 8);
                    } else if mode == 0 {
                        activate_slot(menu, 0x107);
                        write_word(menu, 0x18, 9);
                        activate_slot(menu, 0x10F);
                        write_word(menu, 0x14, 10);
                        for index in 0..5usize {
                            let pane = layout_pane(
                                menu,
                                0x238,
                                platform::read_u32(S_B_OPTN_BTN_00 + index * 4) as usize,
                            );
                            set_pane_visible(pane, true);
                        }
                        write_word(menu, 0, 1);
                        menu_sound_event(menu, 5);
                    }
                }
                2 => {
                    let current = read_i32(menu, 0x84);
                    if current < 1 {
                        menu_sound_event(menu, 0xD);
                    } else {
                        write_word(menu, 0x84, (current - 1) as u32);
                        clear_pair(menu, current + 0x14, 10);
                        activate_pair(menu, read_i32(menu, 0x84) + 0x15, 9);
                        if read_i32(menu, 0x84) == 0 {
                            menu_sound_event(menu, 0xC);
                            scroll_progress(menu, 0);
                        } else {
                            menu_sound_event(menu, 10);
                            scroll_progress(menu, read_i32(menu, 0x84) as u32);
                        }
                        activate_pair(menu, 0x0B, 5);
                    }
                }
                3 => {
                    if read_i32(menu, 0x84) < 10 {
                        let current = read_i32(menu, 0x84);
                        write_word(menu, 0x84, (current + 1) as u32);
                        clear_pair(menu, current + 0x15, 9);
                        activate_pair(menu, current + 0x15, 10);
                        if read_i32(menu, 0x84) == 10 {
                            menu_sound_event(menu, 0xB);
                            scroll_progress(menu, 10);
                        } else {
                            menu_sound_event(menu, 9);
                            scroll_progress(menu, read_i32(menu, 0x84) as u32);
                        }
                        activate_pair(menu, 0x0C, 5);
                    } else {
                        menu_sound_event(menu, 0xD);
                    }
                }
                4 => {
                    if read_byte(menu, 0x8C) == 0 {
                        write_byte(menu, 0x8C, 1);
                        platform::call1(FN_801E15B0, 1);
                        activate_pair(menu, 0x0E, 8);
                        let index = find_menu_pair(0x10, 6);
                        write_word(menu, 0x18, index as u32);
                        activate_pair(menu, 0x10, 6);
                        menu_sound_event(menu, 0x0E);
                    } else {
                        let index = find_menu_pair(0x10, 0x0E);
                        write_word(menu, 0x18, index as u32);
                        activate_pair(menu, 0x10, 0x0E);
                        menu_sound_event(menu, 0xD);
                    }
                    for player in 0..4usize {
                        let widget = indexed_word(menu, 0xAB + player as i32) as usize;
                        fn_80173880(widget);
                        platform::write_u32(
                            menu + 0x210 + player * 4,
                            platform::read_u32(DAT_802F5D90),
                        );
                        platform::write_u32(
                            menu + 0x220 + player * 4,
                            platform::read_u32(DAT_802F5D90),
                        );
                    }
                    write_word(menu, 0x14, 9);
                }
                5 => {
                    if read_byte(menu, 0x8C) == 0 {
                        menu_sound_event(menu, 0xD);
                    } else {
                        write_byte(menu, 0x8C, 0);
                        platform::call1(FN_801E15B0, 0);
                        activate_pair(menu, 0x0E, 6);
                        let index = find_menu_pair(0x0D, 8);
                        write_word(menu, 0x18, index as u32);
                        activate_pair(menu, 0x0D, 8);
                        menu_sound_event(menu, 0x0F);
                    }
                    write_word(menu, 0x14, 9);
                }
                6 => {
                    let index = find_menu_pair(0x0F, 5);
                    write_word(menu, 0x18, index as u32);
                    activate_pair(menu, 0x0F, 5);
                    write_word(menu, 0x14, 3);
                    write_byte(menu, 0x90, 1);

                    for player in 0..4usize {
                        let widget = indexed_word(menu, 0xAB + player as i32) as usize;
                        platform::write_u32(
                            menu + 0x210 + player * 4,
                            platform::read_u32(DAT_802F5D90),
                        );
                        platform::write_u32(
                            menu + 0x220 + player * 4,
                            platform::read_u32(DAT_802F5D90),
                        );
                        fn_801738f0(widget);
                        write_byte(widget, 0x44, 0);
                    }

                    let frame_owner = read_word(DAT_80377FB8, 4) as usize;
                    write_word(
                        menu,
                        0x230,
                        (read_f32(DAT_802F5DB0) / read_f32(frame_owner + 0x30)) as i32 as u32,
                    );
                    set_pane_visible(layout_pane(menu, 0x238, S_T_MSG_00), true);
                    set_pane_visible(layout_pane(menu, 0x238, S_T_MSG_01), true);
                    menu_sound_event(menu, 5);
                    menu_sound_event(menu, 0x10);
                }
                7 => {
                    let index = find_menu_pair(0x13, 0x0D);
                    write_word(menu, 0x18, index as u32);
                    activate_pair(menu, 0x13, 0x0D);
                    let page = read_i32(menu, 0xB8);
                    write_word(menu, 0x14, 0x0D);
                    write_word(menu, 0, 0);
                    if page == 1 {
                        menu_sound_event(menu, 2);
                        write_byte(menu, 0x460, 0);
                    } else if page == 2 {
                        menu_sound_event(menu, 3);
                        write_byte(menu, 0x460, 1);
                    } else if page == 4 {
                        menu_sound_event(menu, 3);
                        write_byte(menu, 0x460, 0);
                    }
                }
                8 => {
                    let index = find_menu_pair(0x14, 0x0D);
                    write_word(menu, 0x18, index as u32);
                    activate_pair(menu, 0x14, 0x0D);
                    write_word(menu, 0x14, 0x0D);
                    write_word(menu, 0xB8, 0xFFFF_FFFF);
                    write_word(menu, 0, 0);
                    menu_sound_event(menu, 6);
                }
                _ => {}
            }
        }
    }
}

// ===========================================================================
// Wave 114: the last 0x80170000 body - the text-run page draw (0x801701A0)
// ===========================================================================
//
// `0x801701A0` and the already-translated `0x8016FA10`
// (`src/resource/registry.rs`) are the same function with three different
// literals. The shared constants and callees are named here with the roles the
// twin proves; the export is
// `work/ghidra/main-gb/export/801701a0_FUN_801701a0.c`.

/// `0x80230780`: begin a console-stream text pass in a caller buffer.
///
/// observed: the twin `0x8016FA10` uses `0x8022DDE0`, whose body
/// `src/resource/bres.rs` documents as the same 0x64-byte constructor with a
/// neighbouring class record.
const FN_80230780: usize = 0x8023_0780;

/// `0x802307E0`: end a console-stream text pass; `-1` disposes nothing.
const FN_802307E0: usize = 0x8023_07E0;

/// `0x8022D5C0`: open a text pass with two mode words.
const FN_8022D5C0: usize = 0x8022_D5C0;

/// `0x8022D3C0`: scale the stream's two position factors by one `f64`.
const FN_8022D3C0: usize = 0x8022_D3C0;

/// `0x8022D9A0`: commit the four colour bytes at `stream + 0x18`.
const FN_8022D9A0: usize = 0x8022_D9A0;

/// `0x8022C9C0`: flush the stream's colour change.
const FN_8022C9C0: usize = 0x8022_C9C0;

/// `0x80231870`: copy a console stream and run one byte run through the copy.
const FN_80231870: usize = 0x8023_1870;

/// `r2 - 0x5348` (`0x8042DD38`): the draw-scale multiplier stored at
/// `stream + 0x44`. The twin reads the same word and passes it to the same
/// commit sequence; the value is the first of the two motion scales
/// `src/resource/registry.rs` names for `0x8016FA10`.
const SDA2_DRAW_SCALE: usize = SDA2_BASE - 0x5348;

/// `r2 - 0x5344` (`0x8042DD3C`): the position-shift multiplier added to both
/// axes on the first pass only.
const SDA2_SHIFT_SCALE: usize = SDA2_BASE - 0x5344;

/// Calls a routine shaped `(f64, pointer) -> u32`.
///
/// # Original
/// Host-side helper for the `0x8022D3C0` call in [`fn_801701a0`]; no binary
/// address of its own. The twin `0x8016FA10` performs the same call
/// (`FUN_8022d3c0((double)*(float *)(psVar4 + 8),auStack_c0)`).
///
/// # Safety
/// `addr` must accept one double in `f1` and one word in `r3`.
///
/// # Confidence
/// high - the export passes the float as a `double` in `f1` and the stream in
/// the next argument slot; the returned word is unused.
#[inline]
unsafe fn call_f64_u32(addr: usize, value: f64, second: u32) -> u32 {
    let f: extern "C" fn(f64, u32) -> u32 = core::mem::transmute(addr);
    f(value, second)
}

/// Copies a big-endian colour word into the stream's four bytes at `+0x18`.
///
/// # Original
/// Host-side helper for the two colour blocks of [`fn_801701a0`]; no binary
/// address of its own. `0x8022D9A0` commits exactly these four bytes.
///
/// # Safety
/// `stream` must be writable for `0x1C` bytes.
///
/// # Confidence
/// high - the four shifts and stores match the export's byte copies.
#[inline]
unsafe fn write_stream_colour(stream: usize, colour: u32) {
    platform::write_u8(stream + 0x18, (colour >> 24) as u8);
    platform::write_u8(stream + 0x19, (colour >> 16) as u8);
    platform::write_u8(stream + 0x1A, (colour >> 8) as u8);
    platform::write_u8(stream + 0x1B, colour as u8);
}

/// Draws and ages one text-run page's records.
///
/// The page at `page` is active only when its `+0x15` byte and its `+0x04`
/// word are both non-zero; an inactive page only closes the text stream. Each
/// record in the list at `page + 8` is drawn twice when the page's `+0x14`
/// byte is set: first with the `+0x0C` colour word, the stream's `+0x44` scale
/// set to `draw_scale * scale`, the `+0x43` byte set to "the record's `+0x1C`
/// byte is non-zero" and the position shifted by `shift_scale * scale` on both
/// axes, then again with the `+0x08` colour and the plain position. The two
/// signed halfwords at `+0x00`/`+0x02` are `x`/`y`, `+0x10` the float scale,
/// `+0x1E` the run bytes and `+0x06` the count word that is shifted right by
/// one before the run copy.
///
/// After the draw the `+0x04` counter is decremented; when it drops below one
/// the record is retired through `0x801705E0`.
///
/// # Original
/// - Ghidra: `FUN_801701a0` at `0x801701A0` (612 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801701a0_FUN_801701a0.c`
/// - observed: the body is the twin of `0x8016FA10` (`604` bytes): the same
///   `0x43300000` halfword conversions, the same `r2 - 0x5348` /
///   `r2 - 0x5344` scales, the same `+0x43` flag, `+0x44` scale and
///   `+0x18..+0x1B` colour bytes. Only the stream constructors
///   (`0x80230780`/`0x802307E0`), the payload offset (`+0x1E`) and the
///   halved run length differ. The twin's translation in
///   `src/resource/registry.rs` proves those field roles.
/// - observed: `FUN_80231870(auStack_c0,psVar4 + 0xf,(uint)(int)psVar4[3] >> 1)`
///   is the run copy; `psVar4 + 0xf` is byte `+0x1E` and the `>> 1` is literal.
/// - observed: the three entry and three exit `ldexpf` probes discard their
///   results; `src/resource/bres.rs` documents the same probes around
///   `0x80231870` as the decompiler's rendering of the paired-single
///   conversions, so they are omitted.
///
/// # Arguments
/// - `page`: the text page (`r3`): `+0x04` stream/resource word, `+0x08`
///   embedded record list, `+0x14` draw flag, `+0x15` active flag.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Opens and closes a stream in a stack buffer, draws each record twice, writes
/// the record's `+0x04` counter and may unlink and release the record through
/// `0x801705E0`.
///
/// # Safety
/// `page` must be live and its embedded list must contain readable records;
/// every record must be readable through `+0x1E` plus its halved `+0x06` run
/// length. The stream buffer is local and 8-byte aligned for the paired-single
/// conversions.
///
/// # Confidence
/// medium - the guard, the two draw passes, the colour and scale stores, the
/// counter and the retire call are literal, and the field roles come from the
/// byte-for-byte twin `0x8016FA10`. The `+0x06` half is kept exactly as the
/// export writes it because the record flavour's unit (bytes or halfwords) is
/// not provable from this export alone.
pub unsafe fn fn_801701a0(page: usize) {
    // The stream is the 0x64-byte object `0x80230780` constructs; 0x100 bytes
    // give the paired-single helpers room and keep the base 8-byte aligned.
    let mut stream = [0u64; 0x20];
    let ctx = stream.as_mut_ptr() as usize;

    platform::call1(FN_80230780, ctx as u32);
    if read_byte(page, 0x15) == 0 || read_i32(page, 4) == 0 {
        platform::call2(FN_802307E0, ctx as u32, 0xFFFF_FFFF);
        return;
    }

    platform::call3(FN_8022D5C0, ctx as u32, 0, 0);

    let draw_scale = read_f32(SDA2_DRAW_SCALE);
    let shift_scale = read_f32(SDA2_SHIFT_SCALE);

    let mut record = platform::call2(FN_80228FF0, p32(page + 8), 0) as usize;
    while record != 0 {
        if read_byte(page, 0x14) != 0 {
            let scale = read_f32(record + 0x10);
            call_f64_u32(FN_8022D3C0, scale as f64, ctx as u32);

            // observed: the original turns the flag byte into `byte != 0`
            // with `neg`/`or`/`rlwinm` before storing it at `+0x43`.
            write_byte(ctx, 0x43, (read_byte(record, 0x1C) != 0) as u8);
            platform::write_f32(ctx + 0x44, draw_scale * scale);
            write_stream_colour(ctx, read_word(record, 0x0C));
            platform::call1(FN_8022D9A0, ctx as u32);
            platform::call1(FN_8022C9C0, ctx as u32);

            let x = read_u16(record, 0) as i16 as i32;
            let y = read_u16(record, 2) as i16 as i32;
            let shift = shift_scale * scale;
            platform::write_f32(ctx + 0x30, y as f32 + shift);
            platform::write_f32(ctx + 0x2C, x as f32 + shift);
            let run_len = ((read_u16(record, 6) as i16 as i32) as u32) >> 1;
            platform::call3(
                FN_80231870,
                ctx as u32,
                p32(record + 0x1E),
                run_len,
            );

            write_stream_colour(ctx, read_word(record, 0x08));
            platform::call1(FN_8022D9A0, ctx as u32);
            platform::call1(FN_8022C9C0, ctx as u32);
            platform::write_f32(ctx + 0x30, y as f32);
            platform::write_f32(ctx + 0x2C, x as f32);
            platform::call3(
                FN_80231870,
                ctx as u32,
                p32(record + 0x1E),
                run_len,
            );
        }

        let remaining = (read_u16(record, 4) as i16).wrapping_sub(1);
        platform::write_u16(record + 4, remaining as u16);
        let next = platform::call2(FN_80228FF0, p32(page + 8), record as u32) as usize;
        if remaining < 1 {
            platform::call2(FN_801705E0, page as u32, record as u32);
        }
        record = next;
    }

    platform::call2(FN_802307E0, ctx as u32, 0xFFFF_FFFF);
}

// ===========================================================================
// homebutton::gui::PaneManager (`0x80329D78`) - wave 273 lane B (`_B273`)
// ===========================================================================

/// `0x80329D70`: `homebutton::gui::PaneManager` vtable header (`RTTI 0x80329E08`).
const VT_80329D70_B273: u32 = 0x8032_9D70;
/// `0x80329E10`: `homebutton::gui::Manager` base-vtable header.
const VT_80329E10_B273: u32 = 0x8032_9E10;
/// `0x80329E80`: `homebutton::gui::Component` base-vtable header.
const VT_80329E80_B273: u32 = 0x8032_9E80;
/// `0x80329CC8`: `homebutton::gui::PaneComponent` vtable header.
const VT_80329CC8_B273: u32 = 0x8032_9CC8;
/// `0x80377DF8`: global PaneComponent serial counter.
const DAT_80377DF8_B273: usize = 0x8037_7DF8;
/// `0x802B9210`: size-class allocator used when the manager owns no heap.
const FN_802B9210: usize = 0x802B_9210;
/// `0x80378018`: runtime type-descriptor word for the pane interface.
const TYPE_80378018_B273: u32 = 0x8037_8018;
/// `0x80378028`: runtime type-descriptor word of the pane manager object.
const TYPE_80378028_B273: u32 = 0x8037_8028;

/// Walks the descriptor chain returned by virtual slot `+0x0C` and reports
/// whether `descriptor` is reachable.
///
/// # Original
/// Host-side factoring of the two identical walks in [`fn_80172840`] and
/// [`fn_801723c0`]: the slot result is compared with the wanted word, then the
/// word at its `+0x00` is followed until it is zero.
///
/// # Safety
/// `obj` must carry a readable vtable with at least four entries, and every
/// descriptor on the chain must be readable at `+0x00`.
///
/// # Confidence
/// high - the compare/follow run is literal in both callers.
#[inline]
unsafe fn type_chain_contains_b273(obj: usize, descriptor: u32) -> bool {
    let mut current = platform::call1(vtable_method(obj, 0, 0x0C), p32(obj));
    while current != 0 {
        if current == descriptor {
            return true;
        }
        current = platform::read_u32(current as usize);
    }
    false
}

/// Calls a virtual method that takes two `f32` and six integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Models the `+0x48` pane hit-test call in
/// [`fn_80171e10`], which loads `f30`/`f31` into `f1`/`f2` and five words into
/// `r4..r8` before the `bcctrl`.
///
/// # Safety
/// `addr` must accept that exact argument list.
///
/// # Confidence
/// high - the register trace is literal.
#[inline]
unsafe fn call_f32x2_u32x6(
    addr: usize,
    a: f32,
    b: f32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
) -> u32 {
    let f: extern "C" fn(f32, f32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(addr);
    f(a, b, r3, r4, r5, r6, r7, r8)
}

/// Initialises a freshly allocated `homebutton::gui::Component` node.
///
/// # Original
/// Host-side factoring of the two identical 0x30-byte initialisations inside
/// [`fn_801723c0`] (`0x8017243C..0x8017246C` and `0x801724A0..0x801724CC`).
///
/// # Safety
/// `node` must be 0x30 writable bytes; `0x80377DF8` must be readable.
///
/// # Confidence
/// high - the store set is literal.
#[inline]
unsafe fn init_component_b273(node: usize) {
    write_word(node, 0, VT_80329E80_B273);
    write_byte(node, 0x18, 0);
    write_word(node, 0x1C, 0);
    write_word(node, 0x20, platform::read_u32(DAT_80377DF8_B273));
    write_byte(node, 0x24, 0);
    write_word(node, 0x28, 0);
}

/// Slot 1 of `0x80329D78`: dispatches virtual `+0x0C` on every child.
///
/// The count is the `u16` at `+0x10`; each entry is resolved through
/// [`fn_8018a6e0`] from the list at `this + 8`, and the object at `entry + 4`
/// receives its virtual `+0x0C` with the object as receiver.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171BC0`.
/// - Extent: `0x80171BC0..0x80171C28` (104 bytes); `0x80171BBC` is zero padding.
/// - Table: slot 1 of `0x80329D78` (word `0x80329D7C`, offset `+0x04`).
/// - Evidence: `rlwinm r4,r31,0,16,31`, the `lhz +0x10` bound and the
///   `lwz r12,0x0C(r12); bcctrl` tail call.
///
/// # Arguments
/// - `this`: manager whose child list starts at `+8` and whose count is at `+0x10`.
///
/// # Side effects
/// One indirect call per child.
///
/// # Safety
/// `this + 0x10` must be readable and every resolved entry must hold a live
/// object at `+4`.
///
/// # Confidence
/// medium - the loop, lookup and slot are literal; the child role is inferred
/// from the list shape and the sibling slot 4.
pub unsafe fn fn_80171bc0(this: usize) {
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = fn_8018a6e0((this + 8) as *const u8, (index & 0xFFFF) as i32) as usize;
        let object = read_word(entry, 4) as usize;
        platform::call1(vtable_method(object, 0, 0x0C), p32(object));
        index += 1;
    }
}

/// Slot 4 of `0x80329D78`: dispatches virtual `+0x18` on every child.
///
/// Identical to [`fn_80171bc0`] except for the virtual slot (`+0x18`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172080`.
/// - Extent: `0x80172080..0x801720E8` (104 bytes); seven zero words precede it.
/// - Table: slot 4 of `0x80329D78` (word `0x80329D88`, offset `+0x10`).
/// - Evidence: `lhz +0x10` bound, `fn_8018A6E0(this + 8, index)` and the
///   `lwz r12,0x18(r12); bcctrl` tail call.
///
/// # Arguments
/// - `this`: manager whose child list starts at `+8`.
///
/// # Side effects
/// One indirect call per child.
///
/// # Safety
/// Same requirements as [`fn_80171bc0`].
///
/// # Confidence
/// medium - the slot is literal; the child role is inferred.
pub unsafe fn fn_80172080(this: usize) {
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = fn_8018a6e0((this + 8) as *const u8, (index & 0xFFFF) as i32) as usize;
        let object = read_word(entry, 4) as usize;
        platform::call1(vtable_method(object, 0, 0x18), p32(object));
        index += 1;
    }
}

/// Slot 5 of `0x80329D78`: deleting destructor of `homebutton::gui::PaneManager`.
///
/// Installs the `0x80329D70` header, tears down the `+0x18` list (freeing each
/// node's `+4` payload through virtual `+0x1C` when the manager owns no heap
/// object), switches the header to the `Manager` base table `0x80329E10`, tears
/// down the `+8` list, and finally releases `this` when `flags > 0`. Returns
/// `this` in every path.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172190`.
/// - Extent: `0x80172190..0x801722E4` (340 bytes); four zero words follow.
/// - Table: slot 5 of `0x80329D78` (word `0x80329D8C`, offset `+0x14`); the
///   `stw r5,0(r3)` at `0x801721C4` is the only direct install of the header.
/// - Evidence: `lis r5,0x8033 / addi r5,r5,-25232` (`0x80329D70`),
///   `addi r4,r4,-25072` (`0x80329E10`), the two `fn_8018A6C0` walks and the
///   `bc 4,1` guard around `0x802B9280(this)`.
///
/// # Arguments
/// - `this`: manager to destroy; `+8` and `+0x18` are intrusive lists, `+0x14`
///   is the optional heap object used for both allocation and release.
/// - `flags`: when greater than zero the manager itself is released.
///
/// # Returns
/// `this` (or zero when it was null).
///
/// # Side effects
/// Removes and releases every node of both lists and may release `this`.
///
/// # Safety
/// `this` must be a live PaneManager; both list headers and `+0x14` must be
/// valid for the calls the original makes.
///
/// # Confidence
/// high - the install words, list calls and release guard are literal.
pub unsafe fn fn_80172190(this: usize, flags: u32) -> usize {
    if this == 0 {
        return this;
    }

    write_word(this, 0, VT_80329D70_B273);

    let mut node = fn_8018a6c0((this + 0x18) as *const u8, core::ptr::null()) as usize;
    while node != 0 {
        fn_8018a650((this + 0x18) as *mut u8, node as *mut u8);
        let heap = read_word(this, 0x14) as usize;
        if heap != 0 {
            platform::call2(FN_801D5550, p32(heap), read_word(node, 4));
            platform::call2(FN_801D5550, p32(heap), p32(node));
        } else {
            let payload = read_word(node, 4) as usize;
            if payload != 0 {
                platform::call2(vtable_method(payload, 0, 0x1C), p32(payload), 1);
            }
            platform::call1(FN_802B9280, p32(node));
        }
        node = fn_8018a6c0((this + 0x18) as *const u8, core::ptr::null()) as usize;
    }

    if this != 0 {
        write_word(this, 0, VT_80329E10_B273);
        let mut node = fn_8018a6c0((this + 8) as *const u8, core::ptr::null()) as usize;
        while node != 0 {
            fn_8018a650((this + 8) as *mut u8, node as *mut u8);
            let heap = read_word(this, 0x14) as usize;
            if heap != 0 {
                platform::call2(FN_801D5550, p32(heap), p32(node));
            } else {
                platform::call1(FN_802B9280, p32(node));
            }
            node = fn_8018a6c0((this + 8) as *const u8, core::ptr::null()) as usize;
        }
    }

    if flags > 0 {
        platform::call1(FN_802B9280, p32(this));
    }
    this
}

/// Slot 6 of `0x80329D78`: adds one child to the `+8` list.
///
/// Reads the child's sort key through virtual `+0x20`, lets the child record
/// its owner through virtual `+0x54`, then allocates a 16-byte pair
/// `{key, child}` from `this + 0x14` (or `0x802B9210` when no heap object is
/// present) and appends it to the list at `+8`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171C60`.
/// - Extent: `0x80171C60..0x80171D24` (196 bytes); four zero words follow.
/// - Table: slot 6 of `0x80329D78` (word `0x80329D90`, offset `+0x18`).
/// - Evidence: the `+0x20`/`+0x54` virtual pair, the two `li r4,16`
///   allocator paths and the `stw r31,0(r3); stw r30,4(r3)` pair stores.
///
/// # Arguments
/// - `this`: manager whose list starts at `+8`.
/// - `child`: object to append.
///
/// # Side effects
/// One 16-byte allocation, two virtual calls and one list append.
///
/// # Safety
/// `this` must be a live PaneManager and `child` must have a live vtable with
/// the `+0x20`/`+0x54` slots.
///
/// # Confidence
/// medium - the calls and stores are literal; the key/owner roles are inferred
/// from the sibling accessors.
pub unsafe fn fn_80171c60(this: usize, child: usize) {
    let key = platform::call1(vtable_method(child, 0, 0x20), p32(child));
    platform::call2(vtable_method(child, 0, 0x54), p32(child), p32(this));

    let heap = read_word(this, 0x14) as usize;
    let node = if heap != 0 {
        platform::call2(FN_801D5540, p32(heap), 16) as usize
    } else {
        platform::call1(FN_802B9210, 16) as usize
    };
    if node != 0 {
        write_word(node, 0, key);
        write_word(node, 4, p32(child));
    }
    fn_8018a5e0((this + 8) as *mut u8, node as *mut u8);
}

/// Slot 7 of `0x80329D78`: removes the child whose value equals `target`.
///
/// Walks the `+8` list with [`fn_8018a6c0`], comparing the `+4` word of each
/// entry with `target`, then unlinks the matching entry and releases it through
/// the manager's heap object or `0x802B9280`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171D40`.
/// - Extent: `0x80171D40..0x80171DE0` (160 bytes); the next word starts
///   [`fn_80171de0`].
/// - Table: slot 7 of `0x80329D78` (word `0x80329D94`, offset `+0x1C`).
/// - Evidence: `lwz r0,4(r31); cmpl cr0,r0,r30`, the `fn_8018A650` unlink and
///   the two release branches.
///
/// # Arguments
/// - `this`: manager whose list starts at `+8`.
/// - `target`: child pointer stored in the entry's `+4` word.
///
/// # Side effects
/// Unlinks and releases one entry (or unlinks/releases a null node if no entry
/// matched, matching the original).
///
/// # Safety
/// `this` must be a live PaneManager; the list must be well formed.
///
/// # Confidence
/// high - the comparison, unlink and release paths are literal.
pub unsafe fn fn_80171d40(this: usize, target: usize) {
    let mut node = fn_8018a6c0((this + 8) as *const u8, core::ptr::null()) as usize;
    while node != 0 {
        if read_word(node, 4) as usize == target {
            break;
        }
        node = fn_8018a6c0((this + 8) as *const u8, node as *const u8) as usize;
    }
    fn_8018a650((this + 8) as *mut u8, node as *mut u8);

    let heap = read_word(this, 0x14) as usize;
    if heap != 0 {
        platform::call2(FN_801D5550, p32(heap), p32(node));
    } else {
        platform::call1(FN_802B9280, p32(node));
    }
}

/// Slot 10 of `0x80329D78`: forwards a hit test to every child.
///
/// Each child is resolved from the `+8` list and asked through virtual `+0x48`
/// with five words and two floats. The first child that accepts and then also
/// reports `+0x4C` non-zero is remembered. When that child exists, the optional
/// `arg2` and `arg4` out-parameter blocks are filled through virtual `+0x3C`
/// and the manager forwards the child's `+0x20` result twice through virtual
/// `+0x34` (selector `0` and `5`, both carrying `arg5`). Returns the
/// remembered flag.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171E10`.
/// - Extent: `0x80171E10..0x80171FB4` (420 bytes); four zero words follow.
/// - Table: slot 10 of `0x80329D78` (word `0x80329DA0`, offset `+0x28`).
/// - Evidence: the `f30`/`f31` save pair, the `+0x48` call with `r4..r8`,
///   `+0x4C`, the two `+0x3C(&stack)` calls and the `li r5,0` / `li r5,5`
///   pair around `+0x34`.
///
/// # Arguments
/// - `this`: manager whose child list starts at `+8`.
/// - `arg1..arg5`: integer operands forwarded to the child and manager calls.
/// - `f1`, `f2`: float operands forwarded to the child hit test.
///
/// # Returns
/// `1` when an accepting child was found, otherwise `0`.
///
/// # Side effects
/// One virtual call per child plus up to four calls on the selected child or
/// manager.
///
/// # Safety
/// `this` and every resolved child must be live; the virtual slots used must
/// have the modelled signatures.
///
/// # Confidence
/// medium - the register flow is literal; the hit-test and out-parameter roles
/// are inferred from the neighbouring slot shapes.
pub unsafe fn fn_80171e10(
    this: usize,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    f1: f32,
    f2: f32,
) -> u32 {
    let mut selected = 0usize;
    let mut found: u32 = 0;
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = fn_8018a6e0((this + 8) as *const u8, (index & 0xFFFF) as i32) as usize;
        let object = read_word(entry, 4) as usize;
        let accepted = call_f32x2_u32x6(
            vtable_method(object, 0, 0x48),
            f1,
            f2,
            p32(object),
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        );
        if accepted != 0
            && platform::call1(vtable_method(object, 0, 0x4C), p32(object)) != 0
        {
            selected = object;
            found = 1;
        }
        index += 1;
    }

    if selected != 0 {
        if arg2 != 0 {
            let mut scratch = [0u32; 4];
            platform::call3(
                vtable_method(selected, 0, 0x3C),
                p32(selected),
                arg2,
                p32(scratch.as_mut_ptr() as usize),
            );
            let handle = platform::call1(vtable_method(selected, 0, 0x20), p32(selected));
            platform::call4(vtable_method(this, 0, 0x34), p32(this), handle, 0, arg5);
        }
        if arg4 != 0 {
            let mut scratch = [0u32; 4];
            platform::call3(
                vtable_method(selected, 0, 0x3C),
                p32(selected),
                arg4,
                p32(scratch.as_mut_ptr() as usize),
            );
            let handle = platform::call1(vtable_method(selected, 0, 0x20), p32(selected));
            platform::call4(vtable_method(this, 0, 0x34), p32(this), handle, 5, arg5);
        }
    }
    found
}

/// Slot 12 of `0x80329D78`: dispatches virtual `+0x50` on every child.
///
/// The count is the `u16` at `+0x10`, entries come from the `+8` list and each
/// object receives the caller's word in `r4`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172100`.
/// - Extent: `0x80172100..0x80172178` (120 bytes); two zero words follow.
/// - Table: slot 12 of `0x80329D78` (word `0x80329DA8`, offset `+0x30`).
/// - Evidence: `lhz +0x10`, the `fn_8018A6E0(this + 8, index)` lookup and
///   `lwz r12,0x50(r12); bcctrl`.
///
/// # Arguments
/// - `this`: manager whose child list starts at `+8`.
/// - `value`: word forwarded in `r4`.
///
/// # Side effects
/// One indirect call per child.
///
/// # Safety
/// `this + 0x10` and the list must be valid, and each child must be live.
///
/// # Confidence
/// high - loop, lookup and slot are literal.
pub unsafe fn fn_80172100(this: usize, value: u32) {
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = fn_8018a6e0((this + 8) as *const u8, (index & 0xFFFF) as i32) as usize;
        let object = read_word(entry, 4) as usize;
        platform::call2(vtable_method(object, 0, 0x50), p32(object), value);
        index += 1;
    }
}

/// Slot 17 of `0x80329D78`: finds a child record by its first word.
///
/// Walks the `+0x18` list with [`fn_8018a6e0`]; the first entry whose `+0` word
/// equals `key` yields its `+4` word. Returns zero when no entry matches.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801727C0`.
/// - Extent: `0x801727C0..0x80172838` (120 bytes); two zero words follow.
/// - Table: slot 17 of `0x80329D78` (word `0x80329DBC`, offset `+0x44`).
/// - Evidence: `lwz r0,0(r3); cmpl cr0,r0,r30`, the `lwz r3,4(r3)` result and
///   the trailing `li r3,0`.
///
/// # Arguments
/// - `this`: manager whose record list starts at `+0x18`.
/// - `key`: value compared with the entry's first word.
///
/// # Returns
/// The entry's `+4` word, or zero.
///
/// # Safety
/// `this + 0x10` and the list at `+0x18` must be readable.
///
/// # Confidence
/// high - comparison, result load and miss value are literal.
pub unsafe fn fn_801727c0(this: usize, key: u32) -> u32 {
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = fn_8018a6e0((this + 0x18) as *const u8, (index & 0xFFFF) as i32) as usize;
        if read_word(entry, 0) == key {
            return read_word(entry, 4);
        }
        index += 1;
    }
    0
}

/// Slot 20 of `0x80329D78`: notifies the children whose type chain matches
/// `0x80378018`.
///
/// Each `+0x18` entry's first word is an object; its virtual `+0x0C` result is
/// walked through `+0x00` until zero, and a match against `0x80378018` makes the
/// entry's `+4` object receive virtual `+0x50` with `value`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172840`.
/// - Extent: `0x80172840..0x80172910` (208 bytes); the next word starts the
///   0x1F0-byte body at `0x80172910`.
/// - Table: slot 20 of `0x80329D78` (word `0x80329DC8`, offset `+0x50`).
/// - Evidence: `lis r29,0x8037 / addi r29,r29,32728` (`0x80377FD8`),
///   `lis r31,0x8038 / addi r31,r31,-32744` (`0x80378018`) and the
///   `lwz r3,0(r3)` chain walk.
///
/// # Arguments
/// - `this`: manager whose record list starts at `+0x18`.
/// - `value`: word forwarded to each matching object's `+0x50` slot.
///
/// # Side effects
/// One virtual call per matching entry.
///
/// # Safety
/// The list, every entry's object and descriptor chain must be readable.
///
/// # Confidence
/// medium - the chain walk and slot are literal; the notify role is inferred.
pub unsafe fn fn_80172840(this: usize, value: u32) {
    let count = read_u16(this, 0x10);
    let mut index: u32 = 0;
    while index < count as u32 {
        let entry = fn_8018a6e0((this + 0x18) as *const u8, (index & 0xFFFF) as i32) as usize;
        let object = read_word(entry, 0) as usize;
        if type_chain_contains_b273(object, TYPE_80378018_B273) {
            let target = read_word(entry, 4) as usize;
            platform::call2(vtable_method(target, 0, 0x50), p32(target), value);
        }
        index += 1;
    }
}

/// Slot 21 of `0x80329D78`: clones one circular node list into children.
///
/// Walks the circular list whose sentinel is `list + 4`, and for every node:
/// allocates a 0x30-byte `PaneComponent` and a 16-byte `{node - 4, component}`
/// pair, initialises the component through the `0x80329E80` base table, runs
/// its virtual `+0x0C`, installs the `0x80329CC8` table, appends the pair to the
/// `+0x18` list, bumps the global serial, stores the serial into `+0x20`,
/// forwards the node body through virtual `+0x60`, marks the component when the
/// body matches `0x80378018` or `0x80378028`, then hands the component to the
/// manager's virtual `+0x20` and the body's `+0x10` block to virtual `+0x5C`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801723C0`.
/// - Extent: `0x801723C0..0x80172658` (664 bytes); two zero words follow.
/// - Table: slot 21 of `0x80329D78` (word `0x80329DCC`, offset `+0x54`).
/// - Evidence: `lis r23,0x8033 / addi -24960` (`0x80329E80`),
///   `lis r25,0x8033 / addi -25400` (`0x80329CC8`), the two allocator paths,
///   `lwz r0,0x7DF8(r22)` / `addi r0,r4,1` / `stw` serial bump and the two
///   `0x80378018`/`0x80378028` chain walks.
///
/// # Arguments
/// - `this`: manager with the `+0x18` list and the optional `+0x14` heap.
/// - `list`: circular list root; `list + 4` is the sentinel.
///
/// # Side effects
/// Allocates two blocks per node, appends to the `+0x18` list, bumps the global
/// serial and makes several virtual calls.
///
/// # Safety
/// `list` must be a well-formed circular list of node bodies (each body is the
/// list pointer minus four) and `this` must be a live PaneManager.
///
/// # Confidence
/// medium - the allocation, store and call sequence is literal; the clone role
/// and the two type tests are inferred.
pub unsafe fn fn_801723c0(this: usize, list: usize) {
    let sentinel = list + 4;
    let mut node = read_word(list, 4) as usize;
    while node != sentinel {
        let heap = read_word(this, 0x14) as usize;
        let component;
        let pair;
        if heap != 0 {
            component = platform::call2(FN_801D5540, p32(heap), 0x30) as usize;
            pair = platform::call2(FN_801D5540, p32(heap), 0x10) as usize;
            if component != 0 {
                init_component_b273(component);
                platform::call1(vtable_method(component, 0, 0x0C), p32(component));
                write_word(component, 0, VT_80329CC8_B273);
                write_word(component, 0x2C, 0);
            }
            if pair != 0 {
                write_word(pair, 0, (node - 4) as u32);
                write_word(pair, 4, p32(component));
            }
        } else {
            component = platform::call1(FN_802B9210, 0x30) as usize;
            if component != 0 {
                init_component_b273(component);
                platform::call1(vtable_method(component, 0, 0x0C), p32(component));
                write_word(component, 0, VT_80329CC8_B273);
                write_word(component, 0x2C, 0);
            }
            pair = platform::call1(FN_802B9210, 0x10) as usize;
            if pair != 0 {
                write_word(pair, 0, (node - 4) as u32);
                write_word(pair, 4, p32(component));
            }
        }

        fn_8018a5e0((this + 0x18) as *mut u8, pair as *mut u8);

        let body = node - 4;
        let serial = platform::read_u32(DAT_80377DF8_B273);
        platform::write_u32(DAT_80377DF8_B273, serial.wrapping_add(1));

        platform::call2(vtable_method(component, 0, 0x60), p32(component), p32(body));
        if type_chain_contains_b273(body, TYPE_80378018_B273) {
            platform::call2(vtable_method(component, 0, 0x50), p32(component), 1);
        }
        if type_chain_contains_b273(body, TYPE_80378028_B273) {
            platform::call2(vtable_method(component, 0, 0x50), p32(component), 1);
        }
        platform::call2(vtable_method(this, 0, 0x20), p32(this), p32(component));
        platform::call2(vtable_method(this, 0, 0x5C), p32(this), p32(body + 0x10));

        node = read_word(node, 0) as usize;
    }
}

// ---------------------------------------------------------------------------
// PaneManager close-out - wave 274 lane A (`_A274`)
// ---------------------------------------------------------------------------

/// Slot 0 of `0x80329D78`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172DF0`.
/// - Extent: `0x80172DF0..0x80172DF4` (4 bytes); it is a lone `bclr 20,0`.
/// - Table: slot 0 of `0x80329D78` (word `0x80329D78`, offset `+0x00`); the
///   same word is slot 0 of the `Manager` (`0x80329E10`) and `Component`
///   (`0x80329E80`) base runs.
/// - Evidence: `0x80172DF0: 4e800020  bclr 20,0`, three zero words before it.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80172df0(_this: usize) {}

/// Slot 3 of `0x80329D78`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172DE0`.
/// - Extent: `0x80172DE0..0x80172DE4` (4 bytes); a lone `bclr 20,0` between
///   zero alignment words.
/// - Table: slot 3 of `0x80329D78` (word `0x80329D84`, offset `+0x0C`); slot 0
///   of `0x80329CD0` names the same body.
/// - Evidence: `0x80172DE0: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80172de0(_this: usize) {}

/// Slot 9 of `0x80329D78`: virtual predicate that always returns zero.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172E80`.
/// - Extent: `0x80172E80..0x80172E88` (8 bytes); two zero words follow.
/// - Table: slot 9 of `0x80329D78` (word `0x80329D9C`, offset `+0x24`).
/// - Evidence: `0x80172E80: 38600000  li r3,0` / `0x80172E84: 4e800020`.
///
/// # Returns
/// Zero.
///
/// # Confidence
/// high - immediate and return are literal.
pub unsafe fn fn_80172e80(_this: usize) -> u32 {
    0
}

/// Slot 11 of `0x80329D78`: forwards slot `+0x08` of the object held at `+4`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171A90`.
/// - Extent: `0x80171A90..0x80171AAC` (28 bytes); the following word starts the
///   empty body at `0x80171AAC`.
/// - Table: slot 11 of `0x80329D78` (word `0x80329DA4`, offset `+0x2C`).
/// - Evidence: `lwz r3,4(r3)`, `cmpi cr0,0,r3,0`, `bclr 12,2` (null guard),
///   then `lwz r12,0(r3)` / `lwz r12,8(r12)` / `bcctr 20,0`.
///
/// # Arguments
/// - `this`: manager whose `+4` word holds an optional child object.
///
/// # Returns
/// The forwarded `+0x08` result, or zero when `+4` is null.
///
/// # Safety
/// `this + 4` must be readable and any non-null child must carry a live vtable
/// with a `+0x08` entry.
///
/// # Confidence
/// high - the load, null guard and tail dispatch are literal.
pub unsafe fn fn_80171a90(this: usize) -> u32 {
    let object = read_word(this, 4) as usize;
    if object == 0 {
        return 0;
    }
    platform::call1(vtable_method(object, 0, 0x08), p32(object))
}

/// Slot 13 of `0x80329D78`: stores the object at `+4` and forwards to its
/// `+0x0C` slot with the previous owner as argument.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172E40`.
/// - Extent: `0x80172E40..0x80172E68` (40 bytes); two zero words follow.
/// - Table: slot 13 of `0x80329D78` (word `0x80329DAC`, offset `+0x34`).
/// - Evidence: `cmpi cr0,0,r4,0`, `stw r4,4(r3)`, `mr r0,r3`, `bclr 12,2`,
///   then `mr r3,r4` / `mr r4,r0` / `lwz r12,0x0c(r12)` / `bcctr 20,0`.
///
/// # Arguments
/// - `this`: manager receiving the object.
/// - `object`: object stored at `+4`; may be null.
///
/// # Returns
/// The forwarded `+0x0C` result, or zero when `object` is null.
///
/// # Safety
/// `this + 4` must be writable; a non-null `object` must carry a live vtable.
///
/// # Confidence
/// high - the store, guard and tail dispatch are literal.
pub unsafe fn fn_80172e40(this: usize, object: usize) -> u32 {
    write_word(this, 4, p32(object));
    if object == 0 {
        return 0;
    }
    platform::call2(vtable_method(object, 0, 0x0C), p32(object), p32(this))
}

/// Slot 15 of `0x80329D78`: forwards `arg->[0x10] + 0x10` to virtual `+0x5C`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801723A0`.
/// - Extent: `0x801723A0..0x801723B8` (24 bytes); two zero words follow and
///   `0x801723C0` starts slot 21.
/// - Table: slot 15 of `0x80329D78` (word `0x80329DB4`, offset `+0x3C`).
/// - Evidence: `lwz r12,0(r3)`, `lwz r4,0x10(r4)`, `lwz r12,0x5c(r12)`,
///   `addi r4,r4,16`, `bcctr 20,0`.
///
/// # Arguments
/// - `this`: manager receiving the call.
/// - `arg`: record whose `+0x10` word points at a sub-object.
///
/// # Returns
/// The forwarded `+0x5C` result.
///
/// # Safety
/// `arg + 0x10` must be readable, `this` must carry a live vtable, and the
/// sub-object pointer plus `0x10` must be a valid address for the callee.
///
/// # Confidence
/// high - the two loads, addend and slot are literal.
pub unsafe fn fn_801723a0(this: usize, arg: usize) -> u32 {
    let inner = read_word(arg, 0x10) as usize;
    platform::call2(
        vtable_method(this, 0, 0x5C),
        p32(this),
        p32(inner.wrapping_add(0x10)),
    )
}

/// Slot 16 of `0x80329D78`: forwards `arg->[0x10] + 0x10` to virtual `+0x60`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172680`.
/// - Extent: `0x80172680..0x80172698` (24 bytes); two zero words follow.
/// - Table: slot 16 of `0x80329D78` (word `0x80329DB8`, offset `+0x40`).
/// - Evidence: `lwz r12,0(r3)`, `lwz r4,0x10(r4)`, `lwz r12,0x60(r12)`,
///   `addi r4,r4,16`, `bcctr 20,0`.
///
/// # Arguments
/// - `this`: manager receiving the call.
/// - `arg`: record whose `+0x10` word points at a sub-object.
///
/// # Returns
/// The forwarded `+0x60` result.
///
/// # Safety
/// Same requirements as [`fn_801723a0`].
///
/// # Confidence
/// high - the two loads, addend and slot are literal.
pub unsafe fn fn_80172680(this: usize, arg: usize) -> u32 {
    let inner = read_word(arg, 0x10) as usize;
    platform::call2(
        vtable_method(this, 0, 0x60),
        p32(this),
        p32(inner.wrapping_add(0x10)),
    )
}

/// Slot 18 of `0x80329D78`: returns the `+0x24` word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172A40`.
/// - Extent: `0x80172A40..0x80172A48` (8 bytes); two zero words follow.
/// - Table: slot 18 of `0x80329D78` (word `0x80329DC0`, offset `+0x48`);
///   `Component` slot 7 (`0x80329E9C`) names the same body.
/// - Evidence: `0x80172A40: 80630024  lwz r3,r3+0x24` / `4e800020`.
///
/// # Returns
/// The word at `this + 0x24`.
///
/// # Safety
/// `this + 0x24` must be readable.
///
/// # Confidence
/// high - a single load.
pub unsafe fn fn_80172a40(this: usize) -> u32 {
    read_word(this, 0x24)
}

/// Slot 22 of `0x80329D78`: removes every entry of an external circular list
/// from the manager's `+0x18` record list.
///
/// For each node of the list rooted at `list` (`list + 4` is the sentinel, each
/// node's body is the node pointer minus four), the body finds the `+0x18`
/// entry whose first word equals that body, forwards the entry's `+4` word to
/// virtual `+0x24`, unlinks and releases the entry exactly as slot 5 does,
/// decrements the global serial counter `0x80377DF8`, and forwards the node
/// body plus `0xC` to virtual `+0x60`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801726A0`.
/// - Extent: `0x801726A0..0x801727BC` (284 bytes); the next word is zero
///   padding before `0x801727C0` (slot 17's body).
/// - Table: slot 22 of `0x80329D78` (word `0x80329DD0`, offset `+0x58`), the
///   last word of the method array.
/// - Evidence: `stwu r1,-0x20` + `bl 0x802dcd70` (`__savegprlr_26`), the
///   `lwz r31,4(r4)` / `addi r30,r4,4` sentinel pair, the two `0x8018A6C0`
///   walks over `this + 0x18`, `lwz r12,0x24(r12)` / `bcctrl 20,0l`, the
///   `lwz r3,0x7df8(r29)` / `addi r0,r3,-1` / `stw` decrement, the two
///   `0x801D5550` heap releases, the `+0x1C` destructor call and the
///   `lwz r12,0x60(r12)` / `bcctrl 20,0l` tail dispatch.
///
/// # Arguments
/// - `this`: manager owning the `+0x18` list and the optional `+0x14` heap.
/// - `list`: intrusive circular list root whose nodes are to be retired.
///
/// # Side effects
/// Removes and releases one `+0x18` entry per node, decrements the global
/// serial and makes two virtual calls per node.
///
/// # Safety
/// `list` must be a well-formed circular list; `this` must be a live
/// PaneManager. The original faults when no `+0x18` entry matches a body (it
/// dereferences the null walk result), and this body keeps that behaviour.
///
/// # Confidence
/// high - the loop, both release paths and the calls are literal.
pub unsafe fn fn_801726a0(this: usize, list: usize) {
    let sentinel = list + 4;
    let mut node = read_word(list, 4) as usize;
    while node != sentinel {
        let key = (node - 4) as u32;
        let mut entry = fn_8018a6c0((this + 0x18) as *const u8, core::ptr::null()) as usize;
        while entry != 0 {
            if read_word(entry, 0) == key {
                break;
            }
            entry = fn_8018a6c0((this + 0x18) as *const u8, entry as *const u8) as usize;
        }

        platform::call2(vtable_method(this, 0, 0x24), p32(this), read_word(entry, 4));
        fn_8018a650((this + 0x18) as *mut u8, entry as *mut u8);
        platform::write_u32(
            DAT_80377DF8_B273,
            platform::read_u32(DAT_80377DF8_B273).wrapping_sub(1),
        );

        let heap = read_word(this, 0x14) as usize;
        if heap != 0 {
            platform::call2(FN_801D5550, p32(heap), read_word(entry, 4));
            platform::call2(FN_801D5550, p32(heap), p32(entry));
        } else {
            let payload = read_word(entry, 4) as usize;
            if payload != 0 {
                platform::call2(vtable_method(payload, 0, 0x1C), p32(payload), 1);
            }
            platform::call1(FN_802B9280, p32(entry));
        }

        platform::call2(
            vtable_method(this, 0, 0x60),
            p32(this),
            p32(node.wrapping_add(0x0C)),
        );
        node = read_word(node, 0) as usize;
    }
}

// ===========================================================================
// homebutton::gui::PaneComponent (`0x80329CD0`) - wave 274 lane A (`_A274`)
// ===========================================================================
//
// Table shape: header `0x80329CC8` -> RTTI record `0x80329D68`, `0x80329CCC = 0`,
// method array `0x80329CD0..0x80329D30` (24 slots, 96 B), then
// the class-name string at `0x80329D30`. All 24 words are distinct code
// addresses in the main text segment; 0 interior labels, 0 nulls, 0 aliases.
// The run is a class vtable, not a dispatch table: the deleting destructor
// `0x801722F0` (slot 5) and the getters/setters identify it.
//
// Identity: RTTI record `0x80329D68` = {name `0x80329D30`, base-info
// `0x80329D50`}; the string is `"homebutton::gui::PaneComponent"`
// (`work/dol/strings.txt`). The base-info lists `0x80329F30`
// (`homebutton::gui::Interface`) and `0x80329F08` (`homebutton::gui::Component`),
// so PaneComponent derives from both, matching the two base tables the clone
// body initialises.
//
// Install site: `0x801723F0` inside `fn_801723c0` (slot 21 of `0x80329D78`):
// `lis r25,0x8033` / `addi r25,r25,-25400` (= `0x8032_9CC8`) / `stw r25,0(r3)`,
// i.e. the clone body stamps the header onto every new 0x30-byte component.
// No other `lis`/`addi` pair or raw data word materialises the header.

/// `0x80171680`: the nine-argument quad emitter used by slot 4.
const FN_80171680_A274: usize = 0x8017_1680;

/// Emits one quad through `0x80171680` with the original float arguments.
///
/// # Original
/// Host-side shim; no binary address of its own. Mirrors the four calls in
/// [`fn_80172a50`], each of which sets `r3 = 8`, `r4 = &colour` and `f1..f5`
/// before `bl 0x80171680`. The existing [`fn_80171680`] body only models the
/// `r3`/`r4` GX-state half of the callee, so this shim keeps the five floats
/// visible instead of silently dropping them.
///
/// # Arguments
/// - `colour`: packed colour word; the callee reads it through `r4`.
/// - `f1..f5`: quad geometry floats.
///
/// # Safety
/// The GX engine must be ready, as for [`fn_80171680`].
///
/// # Confidence
/// high - the register/float setup is literal at `0x80172AE0..0x80172B0C`.
#[inline]
unsafe fn draw_quad_a274(colour: u32, f1: f32, f2: f32, f3: f32, f4: f32, f5: f32) {
    let emit: extern "C" fn(u32, *const u32, f32, f32, f32, f32, f32) =
        core::mem::transmute(FN_80171680_A274);
    let packed = colour;
    emit(8, &packed, f1, f2, f3, f4, f5);
}

/// Slot 2 of `0x80329CD0`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172070`.
/// - Extent: `0x80172070..0x80172074` (4 bytes); a lone `bclr 20,0` between
///   zero alignment words.
/// - Table: slot 2 of `0x80329CD0` (word `0x80329CD8`, offset `+0x08`).
/// - Evidence: `0x80172070: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80172070(_this: usize) {}

/// Slot 4 of `0x80329CD0`: draws the four-quad frame of the component.
///
/// Returns early unless virtual `+0x50` of the object at `+0x28` reports
/// non-zero. Then reads the frame object at `+0x2C`: `+0x4C`/`+0x50` are its
/// width/height and `+0x90`/`+0xA0` its centre. When the byte at `+0x4` is set
/// the packed colour `0xFF0000FF` becomes `0x0000FFFF`. Four quads are emitted
/// from the half extents around that centre (`0x80171680`), in the order top,
/// right, bottom, left.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172A50`.
/// - Extent: `0x80172A50..0x80172BD4` (388 bytes); three zero words follow.
/// - Table: slot 4 of `0x80329CD0` (word `0x80329CE0`, offset `+0x10`).
/// - Evidence: the `stwu r1,-0x40` / `stfd f30/f31` frame, `lwz r3,0x28(r3)`
///   followed by `lwz r12,0x50(r12)` / `bcctrl 20,0l` and `bc 12,2` to the
///   epilogue, `lwz r3,0x2c(r30)` with `lfs` from `+0x4C`/`+0x50`/`+0x90`/
///   `+0xA0`, `lis r31,0x802f` / `addi r31,r31,23128` (`0x802F5A58`: `0.0`,
///   `0xFF0000FF`, `0.5`), the `lbz r0,4(r30)` colour switch with
///   `stb r3,8(r1)` / `stb r0,0xA(r1)`, and the four `li r3,8` /
///   `addi r4,r1,8` / `bl 0x80171680` groups.
///
/// # Geometry
/// `cx = [frame+0x90]`, `cy = [frame+0xA0]`, `w = 0.5 * [frame+0x4C]`,
/// `h = 0.5 * [frame+0x50]`. The four quads trace the frame rectangle: top
/// edge, right edge, bottom edge, left edge.
///
/// # Side effects
/// Up to four GX quad emissions.
///
/// # Safety
/// `this` must be a live component; the `+0x28` object and its vtable, and the
/// `+0x2C` frame object, must be valid.
///
/// # Confidence
/// high - every load, the colour switch and all four call groups are literal;
/// the four float sets were folded from the `fmuls`/`fadds`/`fsubs` pairs.
pub unsafe fn fn_80172a50(this: usize) {
    let owner = read_word(this, 0x28) as usize;
    if platform::call1(vtable_method(owner, 0, 0x50), p32(owner)) == 0 {
        return;
    }

    let frame = read_word(this, 0x2C) as usize;
    // observed: `lfs` from `+0x4C`/`+0x50` are the frame extents and
    // `+0x90`/`+0xA0` its centre; each edge pair is `centre -/+ 0.5*extent`.
    let half_width = read_f32(frame + 0x4C) * 0.5f32;
    let half_height = read_f32(frame + 0x50) * 0.5f32;
    let centre_x = read_f32(frame + 0x90);
    let centre_y = read_f32(frame + 0xA0);

    let mut colour = 0xFF00_00FFu32;
    if read_byte(this, 4) != 0 {
        colour = 0x0000_FFFF;
    }

    let left = centre_x - half_width;
    let right = centre_x + half_width;
    let top = centre_y - half_height;
    let bottom = centre_y + half_height;

    draw_quad_a274(colour, left, top, right, top, 0.0);
    draw_quad_a274(colour, right, top, right, bottom, 0.0);
    draw_quad_a274(colour, right, bottom, left, bottom, 0.0);
    draw_quad_a274(colour, left, bottom, left, top, 0.0);
}

/// Slot 5 of `0x80329CD0`: scalar deleting destructor of a render component.
///
/// Frees `this` through `0x802B9280` when the object is non-null and `flags` is
/// greater than zero, then returns `this` in every path.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801722F0`.
/// - Extent: `0x801722F0..0x80172330` (64 bytes); the next function starts at
///   `0x80172330`.
/// - Table: slot 5 of `0x80329CD0` (word `0x80329CE4`, offset `+0x14`).
/// - Evidence: `stwu r1,-0x10` / `mflr r0` / `stw r0,0x14(r1)`,
///   `cmpi cr0,0,r3,0` + `bc 12,2` null guard, `cmpi cr0,0,r4,0` +
///   `bc 4,1` (`bgt`) flag guard, `bl 0x802b9280`, `mr r3,r31` return.
///
/// # Arguments
/// - `this`: component to release; may be null.
/// - `flags`: released only when greater than zero.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May release `this`.
///
/// # Safety
/// A non-null `this` must be an allocator-owned block.
///
/// # Confidence
/// high - both guards and the release call are literal.
pub unsafe fn fn_801722f0(this: usize, flags: u32) -> usize {
    if this != 0 && (flags as i32) > 0 {
        platform::call1(FN_802B9280, p32(this));
    }
    this
}

/// Slot 6 of `0x80329CD0`: returns the `+0x20` word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171AC0`.
/// - Extent: `0x80171AC0..0x80171AC8` (8 bytes); two zero words follow.
/// - Table: slot 6 of `0x80329CD0` (word `0x80329CE8`, offset `+0x18`);
///   `Component` slot 3 (`0x80329E8C`) names the same body.
/// - Evidence: `0x80171AC0: 80630020  lwz r3,r3+0x20` / `4e800020`.
///
/// # Returns
/// The word at `this + 0x20`.
///
/// # Safety
/// `this + 0x20` must be readable.
///
/// # Confidence
/// high - a single load.
pub unsafe fn fn_80171ac0(this: usize) -> u32 {
    read_word(this, 0x20)
}

/// Slot 7 of `0x80329CD0`: returns the byte at `(this + index) + 4`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171A70`.
/// - Extent: `0x80171A70..0x80171A7C` (12 bytes); one zero word follows.
/// - Table: slot 7 of `0x80329CD0` (word `0x80329CEC`, offset `+0x1C`);
///   `Component` slot 4 (`0x80329E90`) names the same body.
/// - Evidence: `add r3,r3,r4` then `lbz r3,4(r3)` then `bclr 20,0`.
///
/// # Arguments
/// - `this`: component base.
/// - `index`: byte offset added to the base before the load.
///
/// # Returns
/// The byte at `this + index + 4`.
///
/// # Safety
/// The computed address must be readable.
///
/// # Confidence
/// high - the add and load are literal.
pub unsafe fn fn_80171a70(this: usize, index: u32) -> u8 {
    let address = this.wrapping_add(index as usize);
    read_byte(address, 4)
}

/// Slot 8 of `0x80329CD0`: stores a byte at `(this + index) + 4`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171AD0`.
/// - Extent: `0x80171AD0..0x80171ADC` (12 bytes); one zero word follows.
/// - Table: slot 8 of `0x80329CD0` (word `0x80329CF0`, offset `+0x20`);
///   `Component` slot 5 (`0x80329E94`) names the same body.
/// - Evidence: `add r3,r3,r4` then `stb r5,4(r3)` then `bclr 20,0`.
///
/// # Arguments
/// - `this`: component base.
/// - `index`: byte offset added to the base before the store.
/// - `value`: byte written at `this + index + 4`.
///
/// # Safety
/// The computed address must be writable.
///
/// # Confidence
/// high - the add and store are literal.
pub unsafe fn fn_80171ad0(this: usize, index: u32, value: u8) {
    let address = this.wrapping_add(index as usize);
    write_byte(address, 4, value);
}

/// Slot 13 of `0x80329CD0`: copies a 3-float vector into `+0xC`.
///
/// Returns early when the word at `+0x1C` is zero (the original tests it with
/// `and. r4,r0,r0`, so the incoming `r4` is clobbered). Otherwise stores
/// `vec[0..3]` at `+0xC`, `+0x10`, `+0x14` and sets the `+0x18` byte to one.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171FD0`.
/// - Extent: `0x80171FD0..0x80172000` (48 bytes); the next function starts at
///   `0x80172000`.
/// - Table: slot 13 of `0x80329CD0` (word `0x80329D04`, offset `+0x34`).
/// - Evidence: `lwz r0,0x1c(r3)` / `and. r4,r0,r0` / `bclr 12,2`, then three
///   `lfs f0,K(r5)` / `stfs f0,K(r3)` pairs at `0xC`/`0x10`/`0x14`, then
///   `li r0,1` / `stb r0,0x18(r3)`.
///
/// # Arguments
/// - `this`: component receiving the vector.
/// - `_flags`: incoming `r4`, consumed by the `and.` test only.
/// - `vec`: address of the three consecutive `f32` values.
///
/// # Safety
/// `this` must be writable through `+0x18`; `vec` must hold three readable
/// `f32` values.
///
/// # Confidence
/// high - the guard, three copies and flag store are literal.
pub unsafe fn fn_80171fd0(this: usize, _flags: u32, vec: usize) {
    if read_word(this, 0x1C) == 0 {
        return;
    }
    write_word(this, 0x0C, platform::read_u32(vec));
    write_word(this, 0x10, platform::read_u32(vec + 4));
    write_word(this, 0x14, platform::read_u32(vec + 8));
    write_byte(this, 0x18, 1);
}

/// Slot 14 of `0x80329CD0`: stores the incoming word at `+0x1C`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172E10`.
/// - Extent: `0x80172E10..0x80172E18` (8 bytes); two zero words follow.
/// - Table: slot 14 of `0x80329CD0` (word `0x80329D08`, offset `+0x38`).
/// - Evidence: `0x80172E10: 9083001c  stw r4,r3+0x1c` / `4e800020`.
///
/// # Arguments
/// - `this`: component receiving the value.
/// - `value`: word stored at `+0x1C`.
///
/// # Safety
/// `this + 0x1C` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_80172e10(this: usize, value: u32) {
    write_word(this, 0x1C, value);
}

/// Calls a virtual method taking `this` and two floats.
///
/// # Original
/// Host-side helper; no binary address. Models the `+0x5C` and `+0x38` calls in
/// [`fn_80171850`], which load `f1`/`f2` from `f30`/`f31` and pass `r3 = this`
/// before `bcctrl 20,0l`.
///
/// # Safety
/// `addr` must accept `(this, f32, f32)`.
///
/// # Confidence
/// high - the register trace is literal.
#[inline]
unsafe fn call_f32x2_a274(addr: usize, this: usize, f1: f32, f2: f32) -> u32 {
    let call: extern "C" fn(u32, f32, f32) -> u32 = core::mem::transmute(addr);
    call(p32(this), f1, f2)
}

/// Slot 16 of `0x80329CD0`: selects and drives the component's animation or
/// drawing sub-object.
///
/// The component is asked through virtual `+0x58`; when that reports zero the
/// body returns zero. Otherwise it tests `+0x5C(f1, f2)`:
///
/// * match: `+0x24(index)` decides between the `+0x38(f1, f2)` path with mode
///   `3` and the `+0x28(index, 1)` + `+0x2C()` path with mode `1`;
/// * no match: `+0x24(index)` gates the `+0x28(index, 0)` + `+0x30()` path with
///   mode `2`.
///
/// Each completed path asks `+0x20` for a handle and forwards it with its mode
/// and `sink` to virtual `+0x34` of the object at `+0x28`. The `+0x5C` match
/// path returns one; the `+0x5C == 0` path falls through the epilogue with
/// `r31 == 0` and therefore returns zero.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171850`.
/// - Extent: `0x80171850..0x80171A58` (520 bytes, 130 instructions); three zero
///   words follow and `0x80171A70` starts slot 7.
/// - Table: slot 16 of `0x80329CD0` (word `0x80329D10`, offset `+0x40`); also
///   slot 16 of the `Component` base run `0x80329E80`.
/// - Evidence: `stwu r1,-0x30` / `stfd f30/f31` with `fmr f31,f2` / `fmr f30,f1`,
///   `mr r30,r8` (the `sink` argument), the `lwz r12,0x58(r12)` / `bcctrl 20,0l`
///   gate, the `lwz r12,0x5C(r12)` and `0x38(r12)` float calls,
///   `lwz r12,0x24(r12)`, `li r5,3` / `li r5,1` / `li r5,0` / `li r5,2` modes,
///   and `lwz r3,0x28(r28)` before each `lwz r12,0x34(r12)` dispatch.
///
/// # Arguments
/// - `this`: component driven by the call.
/// - `index`: selector forwarded to `+0x24` and `+0x28`.
/// - `_arg3`, `_arg4`, `_arg5`: incoming `r5`/`r6`/`r7`, unused.
/// - `sink`: incoming `r8`, forwarded to the `+0x34` dispatch.
/// - `f1`, `f2`: float operands forwarded to `+0x5C` and `+0x38`.
///
/// # Returns
/// `1` when a path completed, otherwise `0`.
///
/// # Side effects
/// Up to eight virtual calls.
///
/// # Safety
/// `this` and the `+0x28` object must carry live vtables with the used slots.
///
/// # Confidence
/// high - every branch, mode value and call is literal.
pub unsafe fn fn_80171850(
    this: usize,
    index: u32,
    _arg3: u32,
    _arg4: u32,
    _arg5: u32,
    sink: u32,
    f1: f32,
    f2: f32,
) -> u32 {
    if platform::call1(vtable_method(this, 0, 0x58), p32(this)) == 0 {
        return 0;
    }

    if call_f32x2_a274(vtable_method(this, 0, 0x5C), this, f1, f2) != 0 {
        if platform::call2(vtable_method(this, 0, 0x24), p32(this), index) != 0 {
            call_f32x2_a274(vtable_method(this, 0, 0x38), this, f1, f2);
            let handle = platform::call1(vtable_method(this, 0, 0x20), p32(this));
            let emitter = read_word(this, 0x28) as usize;
            platform::call4(
                vtable_method(emitter, 0, 0x34),
                p32(emitter),
                handle,
                3,
                sink,
            );
        } else {
            platform::call3(vtable_method(this, 0, 0x28), p32(this), index, 1);
            platform::call1(vtable_method(this, 0, 0x2C), p32(this));
            let handle = platform::call1(vtable_method(this, 0, 0x20), p32(this));
            let emitter = read_word(this, 0x28) as usize;
            platform::call4(
                vtable_method(emitter, 0, 0x34),
                p32(emitter),
                handle,
                1,
                sink,
            );
        }
        return 1;
    }

    if platform::call2(vtable_method(this, 0, 0x24), p32(this), index) == 0 {
        return 0;
    }
    platform::call3(vtable_method(this, 0, 0x28), p32(this), index, 0);
    platform::call1(vtable_method(this, 0, 0x30), p32(this));
    let handle = platform::call1(vtable_method(this, 0, 0x20), p32(this));
    let emitter = read_word(this, 0x28) as usize;
    platform::call4(
        vtable_method(emitter, 0, 0x34),
        p32(emitter),
        handle,
        2,
        sink,
    );
    // observed: this arm falls into the epilogue with r31 still 0 (r31 is only
    // set to 1 on the +0x5C match path), so it answers 0.
    0
}

// ===========================================================================
// homebutton::gui::PaneComponent close-out (`0x80329CD0`) - wave 275 lane B
// (`_B275`)
// ===========================================================================
//
// The nine slots that were still uncovered at the end of wave 274. Seven of
// them (slots 9, 10, 11, 12, 15, 17 and 19) are also slots of the `Component`
// base run `0x80329E80` (method array `0x80329E88..0x80329EDC`, 21 slots plus
// a null word), so the four empty members and the accessor/setter pair are
// shared bodies rather than PaneComponent-only code. Slots 20 and 23
// (`0x80172D00` and `0x80172E30`) are PaneComponent-only.
//
// Control-flow convention: `bclr` and `bcctrl` end a path; a conditional
// `bc` falls through to the next instruction; padding zero words are not
// counted in an extent.

/// Slot 9 of `0x80329CD0`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171AE0`.
/// - Extent: `0x80171AE0..0x80171AE4` (4 bytes); a lone `bclr 20,0` between
///   zero alignment words.
/// - Table: slot 9 of `0x80329CD0` (word `0x80329CF4`, offset `+0x24`); slot 9
///   of the `Component` base run `0x80329E80` (word `0x80329EAC`) names the
///   same body.
/// - Evidence: `0x80171AE0: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80171ae0(_this: usize) {}

/// Slot 10 of `0x80329CD0`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171AF0`.
/// - Extent: `0x80171AF0..0x80171AF4` (4 bytes); the next word is the
///   `stwu r1,-0x20` prologue of `0x80171B00`.
/// - Table: slot 10 of `0x80329CD0` (word `0x80329CF8`, offset `+0x28`); slot
///   10 of `0x80329E80` (word `0x80329EB0`) names the same body.
/// - Evidence: `0x80171AF0: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80171af0(_this: usize) {}

/// Slot 11 of `0x80329CD0`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172E20`.
/// - Extent: `0x80172E20..0x80172E24` (4 bytes); two zero words follow.
/// - Table: slot 11 of `0x80329CD0` (word `0x80329CFC`, offset `+0x2C`); slot
///   11 of `0x80329E80` (word `0x80329EB4`) names the same body.
/// - Evidence: `0x80172E20: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80172e20(_this: usize) {}

/// Slot 12 of `0x80329CD0`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171A80`.
/// - Extent: `0x80171A80..0x80171A84` (4 bytes); three zero words follow and
///   `0x80171A90` starts slot 11 of `0x80329D78`.
/// - Table: slot 12 of `0x80329CD0` (word `0x80329D00`, offset `+0x30`); slot
///   12 of `0x80329E80` (word `0x80329EB8`) names the same body.
/// - Evidence: `0x80171A80: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_80171a80(_this: usize) {}

/// Slot 15 of `0x80329CD0`: virtual predicate that always returns zero.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172E00`.
/// - Extent: `0x80172E00..0x80172E08` (8 bytes); two zero words follow.
/// - Table: slot 15 of `0x80329CD0` (word `0x80329D0C`, offset `+0x3C`); slot
///   15 of `0x80329E80` (word `0x80329EC4`) names the same body.
/// - Evidence: `0x80172E00: 38600000  li r3,0` / `0x80172E04: 4e800020`.
///
/// # Returns
/// Zero.
///
/// # Confidence
/// high - immediate and return are literal.
pub unsafe fn fn_80172e00(_this: usize) -> u32 {
    0
}

/// Slot 17 of `0x80329CD0`: returns the byte at `+0x24`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171FC0`.
/// - Extent: `0x80171FC0..0x80171FC8` (8 bytes); two zero words follow.
/// - Table: slot 17 of `0x80329CD0` (word `0x80329D14`, offset `+0x44`); slot
///   17 of `0x80329E80` (word `0x80329ECC`) names the same body.
/// - Evidence: `0x80171FC0: 88630024  lbz r3,r3+0x24` / `4e800020`.
///
/// # Returns
/// The byte at `this + 0x24`.
///
/// # Safety
/// `this + 0x24` must be readable.
///
/// # Confidence
/// high - a single byte load.
pub unsafe fn fn_80171fc0(this: usize) -> u8 {
    read_byte(this, 0x24)
}

/// Slot 19 of `0x80329CD0`: stores the incoming word at `+0x28`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171D30`.
/// - Extent: `0x80171D30..0x80171D38` (8 bytes); the next word is the
///   `stwu r1,-0x20` prologue of `0x80171D40`.
/// - Table: slot 19 of `0x80329CD0` (word `0x80329D1C`, offset `+0x4C`); slot
///   19 of `0x80329E80` (word `0x80329ED4`) names the same body.
/// - Evidence: `0x80171D30: 90830028  stw r4,r3+0x28` / `4e800020`.
///
/// # Arguments
/// - `this`: component receiving the value.
/// - `value`: word stored at `+0x28`.
///
/// # Safety
/// `this + 0x28` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_80171d30(this: usize, value: u32) {
    write_word(this, 0x28, value);
}

/// Slot 20 of `0x80329CD0`: tests a five-deep chain of `+0x2C`/`+0x0C`
/// records, then hands the rest of the walk to [`fn_80172be0`].
///
/// The first record is the component's `+0x2C` word; every following record is
/// the `+0x0C` word of the previous one. Each record must have bit 0 of its
/// `+0xCF` byte set. The body returns `0` as soon as a record is disabled,
/// returns `1` when the chain ends (a zero link), and after five unrolled
/// levels tail-calls [`fn_80172be0`] on the remaining, non-null record so that
/// the shared five-level walk continues there.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172D00`.
/// - Extent: `0x80172D00..0x80172DD0` (208 bytes, 52 instructions); its last
///   instruction is the `b 0x80172BE0` tail call into the shared walker.
/// - Table: slot 20 of `0x80329CD0` (word `0x80329D20`, offset `+0x50`); it is
///   one of the two PaneComponent-only bodies of this batch (with slot 23).
/// - Evidence: the five `lbz r0,r3+0xcf` / `rlwinm. r0,r0,0,31,31` /
///   `bc 4,2` groups at `0x80172D04`, `0x80172D2C`, `0x80172D54`,
///   `0x80172D7C`, `0x80172DA4`, the five `lwz r3,r3+0xc` /
///   `cmpi cr0,0,r3,0` link steps, the `li r3,0` / `li r3,1` returns, and the
///   closing `4bfffe14  b 0x80172be0`.
///
/// # Returns
/// `0` when a visited record is disabled, `1` when the chain ends first, or
/// the result of [`fn_80172be0`] for the remainder.
///
/// # Safety
/// `this + 0x2C` and every record reached through `+0x0C` must be readable at
/// `+0xCF`; the records use the same 0xCF flag byte as [`fn_80172be0`].
///
/// # Confidence
/// high - the five unrolled levels and the tail call are literal.
pub unsafe fn fn_80172d00(this: usize) -> u32 {
    let mut node = read_word(this, 0x2C) as usize;
    for _ in 0..5 {
        if read_byte(node, 0xCF) & 1 == 0 {
            return 0;
        }
        node = read_word(node, 0xC) as usize;
        if node == 0 {
            return 1;
        }
    }
    fn_80172be0(node)
}

/// Slot 23 of `0x80329CD0`: returns the `+0x2C` word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80172E30`.
/// - Extent: `0x80172E30..0x80172E38` (8 bytes); two zero words follow.
/// - Table: slot 23 of `0x80329CD0` (word `0x80329D2C`, offset `+0x5C`), the
///   last word of the method array; it is PaneComponent-only.
/// - Evidence: `0x80172E30: 8063002c  lwz r3,r3+0x2c` / `4e800020`.
///
/// # Returns
/// The word at `this + 0x2C` (the frame object slot 20 walks).
///
/// # Safety
/// `this + 0x2C` must be readable.
///
/// # Confidence
/// high - a single load.
pub unsafe fn fn_80172e30(this: usize) -> u32 {
    read_word(this, 0x2C)
}

// ===========================================================================
// homebutton::gui::Manager (`0x80329E18`) - wave 275 lane B (`_B275`)
// ===========================================================================
//
// Span and shape: vtable header `0x80329E10` -> RTTI record `0x80329E78`,
// `0x80329E14 = 0`; method array `0x80329E18..0x80329E50` (14 slots, 56 B);
// the class-name string follows at `0x80329E50`. All 14 words are distinct
// main-text addresses: 0 words null, 0 non-text, 0 interior labels, 0 aliases.
// The run is a class vtable (base class of `PaneManager`): slots `+0x00..+0x10`
// and `+0x18..+0x34` are byte-identical to `0x80329D78` slots 0-13; the only
// difference is slot 5, where Manager carries its own deleting destructor
// `0x80171B00` instead of PaneManager's `0x80172190`.
//
// Identity: RTTI record `0x80329E78` = {name `0x80329E50`, base-info
// `0x80329E6C`}; the string is `"homebutton::gui::Manager"`. The base-info is
// the single pair `{0x80329F30, 0}` (`homebutton::gui::Interface`) terminated
// by a null pointer, so Manager derives from Interface only - PaneManager then
// lists Manager as its second base (`0x80329DF4`).
//
// Install site: `0x80329E10` appears in no raw data word anywhere in the
// image, so no static vtable pointer exists; the header is materialised by
// three `lis rX,0x8033 / addi rX,rX,-25072` pairs:
//
// * `0x80175510`+0x5D4 (`0x80175AE4`), inside the untabled menu builder
//   `0x80175510..0x80175D70`: it allocates a 0x2C-byte object at `0x80175AC0`
//   and stamps the header at `0x80175AEC` (`stw r4,0(r3)`) - the constructor.
// * `0x80171B30`, inside Manager's own deleting destructor (slot 5), which
//   re-stamps the header before tearing down the `+8` list.
// * `0x80172258`, inside PaneManager's deleting destructor `0x80172190`,
//   which switches the object to the base header at `0x8017225C` after
//   destroying the derived `+0x18` list.
//
// Census: 14 slots / 14 distinct targets. At lane start 13 covered, 1
// uncovered (`0x80171B00`, prologue-shaped); the other eight prologue-shaped
// slots were already covered - six by shared bodies from waves 273-274 and two
// (`0x80172000`, `0x80171DE0`) by earlier `functions.tsv` rows. After this
// lane: 14 covered, 0 uncovered, 0 prologue-shaped uncovered.

/// Slot 5 of `0x80329E18`: deleting destructor of `homebutton::gui::Manager`.
///
/// Installs the `0x80329E10` (Manager) header, tears down the `+8` list (each
/// node is unlinked and released through the `+0x14` heap object when present,
/// otherwise through `0x802B9280`), and finally releases `this` when `flags` is
/// greater than zero. Returns `this` in every path.
///
/// This is the base half of [`fn_80172190`]; the derived destructor calls the
/// same two stores and the same `+8` drain inline because it must also retire
/// the derived `+0x18` list first.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171B00`.
/// - Extent: `0x80171B00..0x80171BBC` (188 bytes); the next word is the
///   `stwu r1,-0x10` prologue of `0x80171BC0`.
/// - Table: slot 5 of `0x80329E18` (word `0x80329E2C`, offset `+0x14`); the
///   same slot of `0x80329D78` is the different body `0x80172190`.
/// - Evidence: `stwu r1,-0x20` / `mflr r0` / `stw r0,0x24(r1)` with the
///   `cmpi cr0,0,r3,0` / `bc 12,2` null guard, the
///   `lis r5,0x8033` / `addi r5,r5,-25072` / `stw r5,0(r3)` header install at
///   `0x80171B28..0x80171B34`, the two `fn_8018A6C0`/`fn_8018A650` walks of
///   `this + 8`, the `lwz r3,0x14(r29)` / `bc 12,2` heap choice with
///   `bl 0x801D5550` versus `bl 0x802B9280`, and the `bc 4,1` flag guard
///   around `bl 0x802B9280(this)`.
///
/// # Arguments
/// - `this`: manager to destroy; `+8` is the child list and `+0x14` the
///   optional heap object used for release.
/// - `flags`: when greater than zero the manager itself is released.
///
/// # Returns
/// `this` (zero when the incoming pointer was zero).
///
/// # Side effects
/// Removes and releases every `+8` node and may release `this`.
///
/// # Safety
/// `this` must be a live Manager; the `+8` list header and the `+0x14` heap
/// word must be valid for the calls the original makes.
///
/// # Confidence
/// high - the install words, both release paths and the flag guard are
/// literal.
pub unsafe fn fn_80171b00(this: usize, flags: u32) -> usize {
    if this == 0 {
        return this;
    }

    write_word(this, 0, VT_80329E10_B273);

    let mut node = fn_8018a6c0((this + 8) as *const u8, core::ptr::null()) as usize;
    while node != 0 {
        fn_8018a650((this + 8) as *mut u8, node as *mut u8);
        let heap = read_word(this, 0x14) as usize;
        if heap != 0 {
            platform::call2(FN_801D5550, p32(heap), p32(node));
        } else {
            platform::call1(FN_802B9280, p32(node));
        }
        node = fn_8018a6c0((this + 8) as *const u8, core::ptr::null()) as usize;
    }

    if (flags as i32) > 0 {
        platform::call1(FN_802B9280, p32(this));
    }
    this
}

// ===========================================================================
// homebutton::gui::Component (`0x80329E88`) - wave 276 lane B (`_B276`)
// ===========================================================================
//
// Span: header `0x80329E80` = {RTTI `0x80329F08`, offset-to-top `0`}, method
// array `0x80329E88..0x80329ED8` (21 slots, 84 bytes) and one null terminator
// at `0x80329EDC`. 21 slots, 21 distinct targets, no in-table aliases.
//
// RTTI: `0x80329F08` = {name `0x80329EE0` "homebutton::gui::Component",
// base-info list `0x80329EFC`}; `0x80329EFC` = {`0x80329F30`, 0, 0} and the
// single base record `0x80329F30` = {name `0x80329F10`
// "homebutton::gui::Interface", 0}. So Component derives from Interface only,
// the same two-word record shape `PaneComponent` uses for its bases.
//
// Install: `0x801723EC` inside [`fn_801723c0`] (`lis r23,0x8033` at
// `0x801723D4` / `addi r23,r23,-24960` at `0x801723EC`); both allocation paths
// store it at `+0x00` (`0x80172444`, `0x801724A4`). No other text instruction
// pair and no raw data word materialises `0x80329E80`.
//
// Sharing: 18 of the 21 slots name bodies the `PaneComponent` run
// (`0x80329CD0`) already owns, and slots 0 and 3 also belong to the
// `PaneManager`/`Manager` runs. Only slots 4, 5 and 20 are in no other table.
// Slots 5 (`fn_80172330`) and 16 (`fn_80171850`) are the only prologue-shaped
// targets and both are already translated, so this lane adds the two remaining
// uncovered leaves instead of redefining any shared body.

/// Slot 4 of `0x80329E88`: empty virtual member.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801720F0`.
/// - Extent: `0x801720F0..0x801720F4` (4 bytes); it is a lone `bclr 20,0`
///   between zero alignment words (`0x801720E8`, `0x801720EC`, `0x801720F4`).
///   `0x80172100` starts the next body with `stwu r1,-0x20`.
/// - Table: slot 4 of `0x80329E88` (word `0x80329E98`, offset `+0x10`); it is
///   not named by `0x80329CD0`, `0x80329D78` or `0x80329E18`.
/// - Evidence: `0x801720F0: 4e800020  bclr 20,0`.
///
/// # Confidence
/// high - the body is one instruction.
pub unsafe fn fn_801720f0(_this: usize) {}

/// Slot 20 of `0x80329E88`: virtual predicate that always returns one.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80171A60`.
/// - Extent: `0x80171A60..0x80171A68` (8 bytes); two zero words follow and
///   `0x80171A70` starts slot 7.
/// - Table: slot 20 of `0x80329E88` (word `0x80329ED8`, offset `+0x50`); it is
///   not named by `0x80329CD0`, `0x80329D78` or `0x80329E18`.
/// - Evidence: `0x80171A60: 38600001  li r3,1` /
///   `0x80171A64: 4e800020  bclr 20,0`.
///
/// # Returns
/// One.
///
/// # Confidence
/// high - immediate and return are literal.
pub unsafe fn fn_80171a60(_this: usize) -> u32 {
    1
}
// ===========================================================================
// Wave 277 lane E (`_E277`) - gap batch 7, menu-band bodies
// ===========================================================================
//
// Two untabled gap bodies from the wave-266 candidate set, re-verified against
// `work/dol/main.elf` with the current documented extents. Selection:
// `work/wave277/e_pick.mjs`; ELF re-walk and disassembly: `e_verify.mjs` and
// `e_verify_rest.txt`. Both start with `9421FFF0`, end on exactly one `blr`,
// overlap no documented extent and each is slot 0 of a menu class table.

/// Deleting destructor of the grid palette object.
///
/// Slot 0 of the table `0x80329C18`; the word at `0x80329C20` names this body
/// and the next entry `0x80329C24` is the cell writer [`fn_80170830`].
/// `src/engine/dsys.rs` records `0x80329C18` as the gradient-palette
/// descriptor.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80170940`.
/// - Extent: `0x80170940..0x80170980` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x8017097C`; one `blr` word precedes it at `0x80170938`
///   (the tail of the preceding body) with one zero word at `0x8017093C`, and
///   the next body starts at `0x80170980`.
/// - Reach: no `b` or `bl` in the executable segments targets it. The one data
///   word naming it, `0x80329C20`, is slot 0 (`+0x08`) of the table
///   `0x80329C18`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`; `bc 4,1,+0x08` (ble);
///   `bl 0x802B9280`; `mr r3,r31`; `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`;
///   `mtlr r0`; `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: grid palette object, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live object of the `0x80329C18` class.
///
/// # Confidence
/// high - the guard, the call and the return are literal, and the table slot
/// proves the destructor role.
pub unsafe fn fn_80170940(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Deleting destructor of the dialog pane-list object.
///
/// Slot 0 of [`VT_DIALOG_PANE_LIST`] (`0x80329C80`); the word at `0x80329C88`
/// names this body and [`fn_80171570`] is the constructor that installs the
/// table.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80171590`.
/// - Extent: `0x80171590..0x801715D0` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x801715CC`; the `blr` at `0x80171588` (end of
///   [`fn_80171570`]) is followed by one zero word at `0x8017158C`, and the next
///   body starts at `0x801715D0`.
/// - Reach: no `b` or `bl` in the executable segments targets it. The one data
///   word naming it, `0x80329C88`, is slot 0 (`+0x08`) of
///   [`VT_DIALOG_PANE_LIST`].
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`; `bc 4,1,+0x08` (ble);
///   `bl 0x802B9280`; `mr r3,r31`; `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`;
///   `mtlr r0`; `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: dialog pane-list object, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live object of the [`VT_DIALOG_PANE_LIST`] class.
///
/// # Confidence
/// high - the guard, the call and the return are literal, and the vtable slot
/// proves the destructor role.
pub unsafe fn fn_80171590(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}
