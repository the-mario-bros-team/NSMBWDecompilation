//! NW4R `lyt` core: layout objects, panes, name tables and animation binding.
//!
//! # What this module covers
//!
//! This is the semantic home of the layout layer every NSMBW UI scene sits on:
//! the NW4R (`nw4hbm::lyt`) layout object, the `.brlyt` layout files loaded from
//! a `.arc` container, the `.brlan` animation tables bound to that layout, and
//! the lookup of panes (`N_`, `T_`, `P_`, `A_`, `B_`, `C_` names) inside it.
//!
//! The code here was promoted from three staging lanes of `sys/main.dol`:
//! `r80014_80040` (`0x80014000..0x80040000`, the HUD scenes), `r80140_80180`
//! (`0x80140000..0x80180000`, layout transform and global registration) and
//! `r80180_801c0` (`0x80180000..0x801C0000`, `BRLAN` animation-transform
//! sections and intrusive lists). The HUD scenes themselves live in
//! [`crate::ui::hud`].
//!
//! # Cluster map
//!
//! | Cluster | Original addresses | Home |
//! |---|---|---|
//! | small-data anchors (`r13`/`r2`) | `0x80004244`, `0x8000423C` (`__init_registers`) | this file |
//! | layout object lifecycle | `0x800C8950` construct, `0x800C89A0` destroy | this file (callee table) |
//! | archive / `.brlyt` load | `0x800C8CB0` load, `0x80007220` animation handle | this file (callee table) |
//! | pane / name resolution | `0x800C8D60`, `0x800C8E40`, `0x800C8EB0` | this file (callee table) |
//! | animation bind / play | `0x800C8F20`, `0x800C8FB0`, `0x800C90F0`, `0x800C92F0`, `0x800C9380`, `0x800C93D0`, `0x800C9500`, `0x800C9560`, `0x800C9610`, `0x800C9A60`, `0x800CDC60` | this file (callee table) |
//! | HUD scenes using the core | `0x80014F10..0x800175C0` (`pointGet`, `batteryInfo`, `errorInfo`) | [`crate::ui::hud`] |
//! | `BRLAN` animation-transform sections | `0x80182060`, `0x80182080`, `0x801820A0`, `0x80182100`, `0x80182190`, `0x80183500` | this file (`BRLAN` section) |
//! | layout transform / global registration | `0x8016C9E0`, `0x8016D180` | this file (`layout transform` section) |
//! | NW4R intrusive lists | `0x8018A590..0x8018A6E0` | this file (`list` section) |
//! | pane construction / iteration | `0x8016CF50..0x8016D8E0` | this file (wave-4 section) |
//! | animation state machine | `0x8016DC00..0x8016DD80` | this file (wave-4 section) |
//! | animation resource alloc / length | `0x8016E610..0x8016E8F0` | this file (wave-4 section) |
//! | `RLAN` apply / material colour | `0x801822F0..0x801823E0` | this file (`BRLAN` section) |
//! | pane subtree notify / list teardown | `0x80183910..0x8018AA80` | this file (`list` section) |
//! | `DrawInfo` / texture bind / layout teardown | `0x8017F250..0x8017F980` | this file (wave-5 section) |
//! | pane constructor / child-list iteration | `0x801836B0..0x80186950` | this file (wave-5 section) |
//! | picture texture/flip and font glyph buffers | `0x801845A0..0x80185E00` | this file (wave-5 section) |
//! | `BRFNT` font sections / glyph lookup | `0x80188860..0x8018ABE0` | this file (wave-5 section) |
//! | layout lifecycle / registration | `0x8016C730..0x8016D590` | this file (wave-3 section) |
//! | layout rotation helpers | `0x8016ECB0..0x8016EF50` | this file (wave-3 section) |
//! | pane resources / subclasses / glyph records / material | `0x80180020..0x8018E500` | this file (wave-7 section) |
//! | pane matrix / material colours / font metrics / key records | `0x80180440..0x8018E660` | this file (wave-10 section) |
//! | text-writer print/measure + text-record accessors | `0x80189E20..0x8018F510` | this file (wave-17 section) |
//! | text-writer implementation / glyph / `BRLAN` buffer | `0x801819C0..0x801909F0` | this file (wave-20 section) |
//! | text-window character-cell layout | `0x801886A0`, `0x80190FB0..0x80193EF0` | this file (wave-23 section) |
//! | window records / command streams / window manager | `0x80191050..0x801949B0` | this file (wave-25 section) |
//! | text-window record builders / timing / `MTrk` parser | `0x80191250..0x801942C0` | this file (wave-30 section) |
//! | animation-object constructor / line measurement | `0x80180780`, `0x80185870`, `0x80185F10` | this file (wave-111 section) |
//! | layout node / animation functions of later lanes | remaining `0x80140xxx`, `0x80180xxx` | not promoted yet |
//!
//! # Start here
//!
//! Read [`vtable_method`] and [`copy_vec3`] first: they are the two shapes that
//! repeat in every scene. Then open [`crate::ui::hud::fn_80016020`], the
//! clearest full build sequence (archive load -> `.brlyt` load -> animation and
//! pane binding -> state init), which shows every primitive in this file in the
//! order NW4R expects.
//!
//! The promoted clusters have their own entry points:
//!
//! * **`BRLAN` animation-transform sections** - [`fn_80182060`] (first
//!   section) -> [`fn_80182080`] (next section) -> [`fn_801820a0`] (indirect
//!   material); [`fn_80182100`] and [`fn_80182190`] grow the record arrays in
//!   the same work buffer, and [`fn_80183500`] pushes a descriptor onto the
//!   transform object's embedded list.
//! * **Intrusive lists** (the `list` section) - [`fn_8018a5c0`] (`list_init`)
//!   -> [`fn_8018a5e0`] (append) -> [`fn_8018a6c0`] (iterate) ->
//!   [`fn_8018a650`] (remove); [`fn_8018a590`] is the sentinel insert used by
//!   [`fn_80183500`], and [`fn_8018a6e0`] indexes by position.
//! * **Text-window records** (the wave-25 section) - [`fn_80191050`] builds
//!   the sixteen records, [`fn_80192e60`] creates one cell,
//!   [`fn_80193f00`] retires it, and [`fn_80194470`] -> [`fn_80194630`] ->
//!   [`fn_801949b0`] own the window-manager list and singleton.
//! * **Layout transform and registration** - [`layout_set_transform`] applies
//!   position/rotation/scale, and [`layout_register`] inserts a layout into the
//!   engine-wide list at `0x80377B1C`.
//!
//! # Dependencies
//!
//! Only [`crate::platform`]: all target memory is touched as big-endian bytes
//! through `read_u8`/`read_u32`/`write_u8`/`write_u32`. The crate is `no_std`
//! and this module does not allocate. [`crate::ui::hud`] is the only in-crate
//! consumer today; the `BRLAN`, list and registration helpers have no caller
//! inside the crate yet.
//!
//! # Shared object model (observed)
//!
//! These offsets recur across all three HUD scenes and are the object layout
//! this module and its consumer rely on:
//!
//! - `+0x00`: scene vtable pointer (one per class).
//! - `+0x04` / `+0x08`: embedded `nw4r::lyt::Layout`. Its second vtable pointer
//!   lives at `layout + 0x08`, i.e. scene `+0x0C` / `+0x10`; that is the
//!   pointer used by the virtual "load", "close" and "notify" calls.
//! - pane / animation object `+0xBB`, bit 0: visibility flag. `1` = shown;
//!   every show path uses `| 1`, every hide path uses `& 0xFE`.
//! - pane object `+0x2C/+0x30/+0x34`: translation vector (X, Y, Z `f32`),
//!   copied into per-widget position records.
//! - animation objects are created by the layout and addressed later by index
//!   (`0x800C92F0`, `0x800C9380`, `0x800C9610`), never by name.
//!
//! # `BRLAN` animation-transform object (observed through its callers)
//!
//! The section and growth helpers in the `BRLAN` section read this object. Its
//! layout comes from callers `0x8017DCC0` (resource walk) and `0x8017DA20`
//! (descriptor push), not from a recovered header:
//!
//! | offset | type | meaning |
//! |---|---|---|
//! | `+0x4C` | `u32` | packed counts; bits 24..27 are one 4-bit count, bits 28..31 another |
//! | `+0x50` | `u32` | packed counts; bits 20..23 = 4-byte-record count, bits 28..31 = 0x20-byte-record count |
//! | `+0x58` | pointer | base of the animation work buffer |
//! | `+0x18` / `+0x1C` | `u32` / embedded node | list count and list head used by [`fn_80183500`] |
//!
//! # NW4R intrusive list header (observed in [`fn_8018a5c0`])
//!
//! | offset | type | meaning |
//! |---|---|---|
//! | `+0x00` | pointer | first (oldest) node |
//! | `+0x04` | pointer | last (newest) node |
//! | `+0x08` | `u16` | node count |
//! | `+0x0A` | `u16` | byte offset from a node to its two-word link |
//!
//! Each link is two words: `+0` points toward the older end, `+4` toward the
//! newer end. Iteration starts at the head and follows `+4`. [`layout_register`]
//! uses the same shape for the engine-wide layout list, with a fixed sentinel
//! at `0x80377B1C`.
//!
//! # Small-data bases (evidence)
//!
//! The Ghidra C shows `unaff_r13` / `unaff_r2` for the PowerPC small-data
//! bases. Both are recoverable from the hand-written `__init_registers` code:
//!
//! - `0x80004244`: `lis r13,0x8042`, `0x80004248`: `ori r13,r13,0xF680`
//!   -> `r13 = 0x8042F680`.
//! - `0x8000423C`: `lis r2,0x8043`, `0x80004240`: `ori r2,r2,0x3080`
//!   -> `r2 = 0x80433080`.
//!
//! Raw bytes from `work/dol/main.elf` confirm both. Every `r13`/`r2` reference
//! below is written as `SDA_BASE - off` / `SDA2_BASE - off`, so no unresolved
//! "unaff" register remains.
//!
//! # Wave 3: layout lifecycle and rotation helpers (`0x8016C730..0x8016EF50`)
//!
//! The second pass added eight bodies around the already-promoted
//! [`layout_set_transform`] / [`layout_register`] pair:
//!
//! * [`layout_init`] (`0x8016C960`) - clear, name, tag and notify a layout.
//! * [`fn_8016d590`] (`0x8016D590`) - initialise through `0x8016CEA0` and set
//!   the registered/enabled bytes.
//! * [`fn_8016cb60`] and [`fn_8016d280`] (`0x8016CB60`, `0x8016D280`) - the two
//!   "create a child from a resource" wrappers; the second one applies a
//!   position/rotation/scale through `0x8016C9E0` first.
//! * [`layout_unregister`] (`0x8016D4B0`) - unlink from the global list at
//!   `0x80377B1C` and clear the registered byte.
//! * [`fn_8016c730`] (`0x8016C730`) - tick every registered layout, dropping
//!   the ones whose virtual `+0xB8` returns zero.
//! * [`fn_8016ecb0`] and [`fn_8016ef50`] (`0x8016ECB0`, `0x8016EF50`) - the
//!   per-axis rotation helpers that [`layout_set_transform`] calls; they build
//!   a 4x4 matrix and concatenate it into the transform.
//!
//! These are the writers behind the registry documented above. They complete
//! the register/unregister pair, so a reconstruction can create, tick and
//! destroy a layout without calling raw addresses.
//!
//! # Wave 4: pane construction, pane-tree walks and animation state
//!
//! The fourth pass continues the same `lyt` band with 25 more bodies:
//!
//! * **Pane construction** - [`fn_8016cf50`] and [`fn_8016d330`] are the two
//!   "attach a prepared child" wrappers around the virtual `+0x8C`/`+0x94`
//!   slots; [`fn_8016d8e0`] builds a transform first and also runs a 48-byte
//!   scoped helper object around the attach.
//! * **Pane-tree walks** - [`fn_8016d090`] visits every node of the tree rooted
//!   at `object + 0x38` through the visitor callback at `visitor + 0x0C`;
//!   [`fn_8016d3c0`] is the same walk through the callback at
//!   `visitor + 0x120`. [`fn_8016d120`] and [`fn_8016d440`] register those two
//!   visitors with `0x802862C0`.
//! * **Animation state** - [`fn_8016dc00`] constructs the five-word state
//!   object, [`fn_8016dcf0`] copies its three colour bytes,
//!   [`fn_8016dd20`]/[`fn_8016dd50`] perform the two idle transitions and
//!   [`fn_8016dd80`] advances the counter once per tick.
//! * **Animation resources** - [`fn_8016e8f0`] allocates and constructs a
//!   resource, [`fn_8016e760`] reports the remaining length after a fixed
//!   `0x80`-unit lead-in, and [`fn_8016e610`]/[`fn_8016e7d0`] are the two
//!   alignment helpers it uses.
//! * **`RLAN` apply** - [`fn_801822f0`] applies one `0x20`-byte transform
//!   record, [`fn_80182380`] copies a record then refreshes it and
//!   [`fn_801823e0`] is the `RLMC` material-colour byte writer.
//! * **Lists** - [`fn_80183910`] inserts a descriptor into an object's
//!   embedded sentinel list, [`fn_80183f40`] notifies a pane subtree, and
//!   [`fn_8018a4b0`]/[`fn_8018a540`]/[`fn_8018aa80`] clear or release list
//!   storage.
//!
//! The two list shapes introduced in wave 2 (offset-link list and sentinel
//! list) are both used here; the section notes above still apply.
//!
//! # Wave 5: `DrawInfo`, picture children and `ut` fonts
//!
//! The fifth pass adds 25 bodies adjacent to wave 4 and follows the same
//! NW4R classes through their RTTI names in `main.elf`:
//!
//! * **`nw4r::lyt::DrawInfo`** (`0x8032B1E0`) - [`fn_8017f390`] constructs the
//!   object, [`fn_8017f250`] binds one animated texture record into a GX
//!   texture object, and [`fn_8017f450`] loads the texture list of a resource.
//! * **`nw4r::lyt::Group`** (`0x8032B210`) - [`fn_8017f610`] releases a node
//!   list and [`fn_8017f980`] is the `nw4r::lyt::Layout` teardown that walks
//!   its groups and panes.
//! * **`nw4hbm::lyt::Pane`** (`0x8032B310`) - [`fn_801836b0`] constructs the
//!   pane, [`fn_801837d0`] tears it down, and [`fn_801840e0`],
//!   [`fn_80184270`], [`fn_80184310`], [`fn_80184350`] and [`fn_801868c0`]
//!   walk, filter and extend its child lists.
//! * **Picture texture math** - [`fn_801845a0`] returns the flip/wrap offset
//!   pair, [`fn_801848a0`] applies one texture record, and [`fn_80184c00`]
//!   releases an object.
//! * **Font glyph buffers** - [`fn_80185cc0`], [`fn_80185d40`] and
//!   [`fn_80185e00`] grow, free and fill the `u16` glyph-index buffer; the
//!   `BRFNT` section walkers [`fn_80188860`], [`fn_8018a400`],
//!   [`fn_8018a730`], [`fn_8018a7d0`] and [`fn_8018a920`] parse the font file,
//!   and [`fn_8018abe0`] maps a character code to a glyph index.
//!
//! Evidence for the class names is the RTTI string each vtable points at:
//! `0x8032B1EC` is `nw4hbm::lyt::DrawInfo`, `0x8032B21C`
//! `nw4hbm::lyt::Group`, `0x8032B26C` `nw4hbm::lyt::Layout`, `0x8032B190`
//! `nw4hbm::lyt::Pane`, `0x8032C768` `nw4hbm::ut::ResFont` and `0x8032C7D0`
//! `nw4hbm::ut::Font`.
//!
//! # Wave 7: pane factory, pane subclasses and glyph records
//!
//! The wave-7 pass adds 25 bodies in `0x80180020..0x8018E500`:
//!
//! * **Pane factory** - [`fn_80180240`] turns a `pan1`/`pic1`/`txt1`/`wnd1`/
//!   `bnd1` tag into the corresponding object and constructor call.
//! * **Picture** - [`fn_801846a0`] constructs it, [`fn_80184c40`] installs the
//!   small family vtable, and [`fn_80183960`] computes its flip-aware texture
//!   coordinate rectangle from [`fn_801845a0`].
//! * **TextBox/Window** - [`fn_80184c60`] and [`fn_801860b0`] construct the
//!   two text panes, [`fn_801862c0`] destroys the window, and
//!   [`fn_80186950`] emits one glyph box.
//! * **Pane notification and geometry** - [`fn_801843d0`] propagates a key and
//!   value to the child list and material, [`fn_80180020`] builds a transform
//!   record, [`fn_80183460`] forwards to virtual `+0x14`, and [`fn_80180560`]
//!   normalises a six-float matrix.
//! * **Font metrics and glyph records** - [`fn_8018aea0`] completes the glyph
//!   lookup used by [`fn_8018abe0`], [`fn_8018af70`] builds the per-glyph
//!   `TGLP`/`CWDH` record, and [`fn_80189b50`]/[`fn_80189bc0`]/
//!   [`fn_80189c30`] read the three scaled advances.
//! * **Colours and material** - [`fn_80189740`] copies two colours,
//!   [`fn_80189790`] rebuilds four colour words, [`fn_8018e500`] resets a
//!   material, and [`fn_8018b080`]/[`fn_8018b090`] install and release the
//!   font-texture table.
//!
//! # Wave 17: text-writer print/measure helpers and text-record accessors
//!
//! The seventeenth pass adds 22 bodies in `0x80189E20..0x8018F510`, the part
//! of the font/text band that the earlier `lyt` waves left behind. The band
//! has the shape of the `ut::TextWriterBase` print/measure family
//! (`// inferred:` from the shared 0x60-byte writer record and from the
//! Print/Printf call shapes; the tag processor vtable at `0x8032C8B0` names
//! the neighbouring record handlers as `ut::TagProcessorBase`):
//!
//! * **Print/measure wrappers** - [`fn_8018c3b0`] and [`fn_8018ed70`] format a
//!   `printf`-style string into a scratch buffer, clone the caller's writer
//!   state and pass the result to the char/wide implementation. The
//!   `fn_8018c520`/`fn_8018c660`/`fn_8018c7b0`/`fn_8018c8f0` and
//!   `fn_8018eee0`/`fn_8018f020`/`fn_8018f170`/`fn_8018f2b0` groups return the
//!   measured width or height, and `fn_8018cb50`/`fn_8018f510` write a whole
//!   rectangle.
//! * **Line measurement** - [`fn_8018d920`] accumulates the union of the
//!   per-character rectangles into a four-float rectangle.
//! * **Font-state initialiser** - [`fn_8018a100`] programs the default GX TEV
//!   state used by the font texture before the first glyph is drawn.
//! * **Record accessors** - [`fn_80189e20`], [`fn_8018bd80`], and the
//!   `0x8018E6F0..0x8018E790` group expose the text-layout fields `+0x4C`,
//!   `+0x50`, `+0x54`, `+0x5C` and the line-height write at `+0x30`.
//!
//! The two print implementations (`0x8018D920` char, `0x80190300` wide) and
//! the per-character line step `0x8018D540` are called by address in this
//! pass; wave 20 adds their bodies below.
//!
//! # Wave 20: text-writer implementation, glyph emission and BRLAN buffer
//!
//! The twentieth pass adds the 20 bodies that remained in the layout band
//! after wave 17: the two per-character line steps, the two print loops, the
//! two alignment wrappers, the remaining `Print`/`Printf`/`CalcStringRect`
//! entry points, the glyph quad emitter, the sine/cosine table lookups used by
//! the pane matrix, and the `BRLAN` work-buffer allocator:
//!
//! * **Line steps** - [`fn_8018d540`] (char) and [`fn_8018ff00`] (wide) parse
//!   one character, dispatch tags through `writer + 0x5C`, measure the glyph
//!   and extend the line rectangle.
//! * **Print loops** - [`fn_8018da40`] (char) and [`fn_80190420`] (wide) call
//!   the alignment wrapper and then walk the string again, placing glyphs and
//!   handling the tag results.
//! * **Alignment** - [`fn_8018dff0`] (char) and [`fn_801909f0`] (wide) shift
//!   the start position by the measured extent and finalise the cursor.
//! * **Wrappers** - [`fn_8018ca40`]/[`fn_8018f400`] measure into a caller
//!   rectangle, [`fn_8018cc80`]/[`fn_8018f640`] format into the scratch
//!   buffer and print, and [`fn_8018fa40`]/[`fn_8018fba0`] print and restore
//!   the caller's cursor.
//! * **Glyph and GX state** - [`fn_80189cc0`] advances one glyph,
//!   [`fn_80189e70`] emits its GX quad and texture cache, and
//!   [`fn_80188df0`] programs the per-format font texture state.
//! * **Layout math and animation** - [`fn_801885b0`]/[`fn_80188630`] are the
//!   sine/cosine lookups used by [`fn_80180440`]; [`fn_801819c0`] sizes and
//!   seeds the `BRLAN` work buffer that the wave-2 `BRLAN` helpers index;
//!   [`fn_8018aac0`] stores one font-information pair.
//!
//! The wave-17 wrappers still call `0x8018D540`/`0x80190300` by address; the
//! translations above are the same entry points and are called directly by
//! the new bodies.
//!
//! # Wave 23: text-window character-cell layout (`0x80190FB0..0x80193EF0`)
//!
//! The twenty-third pass adds 25 bodies above the wave-20 text-writer band.
//! They are the small helpers around the per-message "window" records that the
//! larger, still-untranslated layout builders (`0x80191260`, `0x80191D20`) fill
//! in; `// inferred:` the record role comes from the per-character position
//! fields those builders write and from the shared 16-slot id table in
//! `crate::runtime::ids`.
//!
//! * **Window record access** - [`fn_80190fb0`] selects the two colour/offset
//!   tables for a record; [`fn_80191bb0`] clears the live object pointer and
//!   [`fn_80191be0`], [`fn_80191c20`], [`fn_80191c60`] and [`fn_80191ce0`] set
//!   one record field and mark the record dirty.
//! * **Window table lifecycle** - [`fn_80192990`] resets the 16 records,
//!   [`fn_80192a70`] retires every slot and drained list node,
//!   [`fn_80192ae0`] builds and links a message object,
//!   [`fn_80192c70`] unlinks one message from all records and
//!   [`fn_80192900`] removes it from the global list.
//! * **Slot initialisation and commands** - [`fn_80192d80`] seeds the 16
//!   per-column tables, [`fn_80193090`] executes one three-byte command, and
//!   [`fn_801931b0`] drains the command buffer.
//! * **Per-column layout and timing** - [`fn_801934f0`] advances one column's
//!   intro/outro state, [`fn_80193600`]..[`fn_801936a0`] compute and publish
//!   its position, [`fn_80193740`] converts a fixed-point frame count through
//!   the percent/minute/second tables, [`fn_80193850`] sets the spacing and
//!   glyph count, and [`fn_80193e00`] dispatches the three record modes.
//! * **Layout angle table** - [`fn_801886a0`] is the table `atan2` used by the
//!   pane matrix: it returns the angle in 1/256-turn units with zero along
//!   `+Y`, which is the same unit as [`fn_801885b0`]/[`fn_80188630`].
//!
//! The 100-byte records are reached through `0x803780EC` and the signed slot
//! byte at `0x80378088`; the 16-slot table itself is maintained by
//! `crate::runtime::ids` and is only read here. No new staging module and no
//! shared file was edited by this pass.
//!
//! # Wave 111: animation-object constructor and text-line measurement
//!
//! The last pass over the `0x80180000` bucket adds the three bodies the earlier
//! `lyt` waves had left there, so that bucket is now complete:
//!
//! * **Animation object** - [`fn_80180780`] constructs the 0x5C-byte object the
//!   pane subclasses allocate at `+0x28`. It decodes the resource pack word at
//!   `+0x3C` into six counts, sizes the work buffer through the wave-20
//!   [`fn_801819c0`], and copies the resource arrays into the buffer slots the
//!   packed words `+0x4C`/`+0x50` reserve. The `'timg'` patterns are bound
//!   through [`fn_8017f250`]; the remaining arrays are plain 0x20-, 0x14-,
//!   0x10- and 4-byte records.
//! * **Line measurement pair** - [`fn_80185870`] walks one line, unions the tag
//!   processor's character rectangle and stops when the line exceeds the `f1`
//!   width, reporting the character count that fit; [`fn_80185f10`] walks a
//!   whole string with that helper and stores the union into a four-float
//!   rectangle. Both consume a `u16` string, which is why every distance is
//!   divided or multiplied by two.
//!
//! The three bodies keep the packed-count arithmetic literal; the array roles
//! are inferred from their strides only, and the constants they read from
//! `0x802F5E50`/`0x8032B430` stay addresses because their contents are not in
//! the export.
//!
//! # Glossary
//!
//! - **NW4R** - Nintendo Ware 4th generation, Nintendo's internal engine whose
//!   `lyt` library draws 2D layouts.
//! - **BRLYT** - binary NW4R layout file (`.brlyt`); **RLAN/BRAN** - binary
//!   NW4R animation file (`.brlan`), bound per pane and played by index.
//! - **Pane** - a node of the layout tree: text (`T_`), picture (`P_`),
//!   null/position (`N_`), animation window (`A_`/`B_`/`C_`).
//! - **Layout** - the object embedded in every scene that owns the pane tree
//!   and its animations.
//! - **vtable** - PowerPC function-pointer table; a virtual call here is
//!   `(*(*(obj + vtable_off) + method_off))(...)`.
//! - **SDA / `r13` / `r2`** - the PowerPC small-data areas the game pins in
//!   `__init_registers`; see the derivation above.
//! - **DOL** - the Wii executable container (`sys/main.dol`); **ARC/U8** - the
//!   archive format `.brlyt`/`.brlan` files are packed in.
//!
//! Details of the original lane and its open questions:
//! `docs/COVERAGE_r80014_80040.md`.

use crate::platform::{
    self, call3, read_i32, read_u8, read_u16, read_u32, write_u16, write_u32, write_u8,
};

// ===========================================================================
// Small-data anchors
// ===========================================================================

/// PowerPC `r13` (small-data base), set by `__init_registers`.
///
/// observed: `lis r13,0x8042` at `0x80004244`, `ori r13,r13,0xF680` at
/// `0x80004248`.
///
/// # Original
/// Not a function: the `__init_registers` prologue of `0x80004050`
/// (`work/ghidra/main-gb/export/80004050_entry.c`), reproduced here because
/// every `r13`-relative table below is expressed from this base.
///
/// # Confidence
/// high - instruction sequence and ELF bytes agree.
pub(crate) const SDA_BASE: usize = 0x8042_F680;

/// PowerPC `r2` (small-data-2 base), set by `__init_registers`.
///
/// observed: `lis r2,0x8043` at `0x8000423C`, `ori r2,r2,0x3080` at
/// `0x80004240`.
///
/// # Original
/// Same prologue as [`SDA_BASE`]. `r2` holds read-only floats and integer
/// tables; `r13` holds pointers to name tables.
///
/// # Confidence
/// high - instruction sequence and ELF bytes agree.
pub(crate) const SDA2_BASE: usize = 0x8043_3080;

/// `r2 - 0x7ED8`: `f32 0.0`.
///
/// observed: ELF bytes at `0x8042B1A8` are `0x00000000`.
///
/// # Original
/// Read by the `pointGet` constructor `0x80014F10` (now
/// [`crate::ui::hud::fn_80014f10`]).
///
/// # Confidence
/// high - raw bytes.
pub(crate) const F32_SDA2_ZERO: usize = SDA2_BASE - 0x7ED8; // 0x8042B1A8

/// `r2 - 0x7ED4`: `f32 1.0`.
///
/// observed: ELF bytes at `0x8042B1AC` are `0x3F800000`.
///
/// # Original
/// Read by `0x80014F10` (now [`crate::ui::hud::fn_80014f10`]).
///
/// # Confidence
/// high - raw bytes.
pub(crate) const F32_SDA2_ONE: usize = SDA2_BASE - 0x7ED4; // 0x8042B1AC

/// `r13 - 0x7F28`: one-element `const char*` table -> `"N_coin_00"`.
///
/// observed: slot at `0x80427758` holds `0x802FF718`; the bytes there are
/// `"N_coin_00"`.
///
/// # Original
/// Consumed by `0x80014FF0` (`crate::ui::hud::fn_80014ff0`) as the fourth
/// argument of the pane resolver `0x800C8D60`.
///
/// # Confidence
/// high - table contents verified in the ELF.
pub(crate) const TABLE_SDA_N_COIN_00: usize = SDA_BASE - 0x7F28; // 0x80427758

/// `r13 - 0x7F08`: one-element `const char*` table -> `"T_player_01"`.
///
/// observed: slot at `0x80427778` holds `0x80300060`.
///
/// # Original
/// Consumed by `0x800171F0` (`crate::ui::hud::fn_800171f0`) to bind the
/// player-name text pane at frame `0xC9`.
///
/// # Confidence
/// high - table contents verified in the ELF.
pub(crate) const TABLE_SDA_T_PLAYER_01: usize = SDA_BASE - 0x7F08; // 0x80427778

/// `r13 - 0x7F00`: two-element `const char*` table ->
/// `"N_playerButton"`, `"N_yesnoButton_00"`.
///
/// observed: slots at `0x80427780`/`0x80427784`.
///
/// # Original
/// Consumed by `0x800171F0` to resolve the two button panes into `+0x204`.
///
/// # Confidence
/// high - table contents verified in the ELF.
pub(crate) const TABLE_SDA_PLAYER_BUTTONS: usize = SDA_BASE - 0x7F00; // 0x80427780

/// `r2 - 0x7EB0`: one-element integer table holding the value `5`.
///
/// observed: `u32` at `0x8042B1D0` is `5`.
///
/// # Original
/// Frame argument of the text-name binding in `0x800171F0`
/// (`crate::ui::hud::fn_800171f0`).
///
/// # Confidence
/// high - table contents verified in the ELF.
pub(crate) const TABLE_SDA2_FRAME_5: usize = SDA2_BASE - 0x7EB0; // 0x8042B1D0

// ===========================================================================
// NW4R lyt callees (called by address; definitions live outside this lane)
// ===========================================================================

/// `0x80007220`: returns `*(u32 *)(layout + 0x20)` (animation handle).
///
/// # Original
/// Callee of `0x80014FF0`, `0x80016020`, `0x800171F0`.
///
/// # Confidence
/// high - the returned word is stored directly into the scene's animation
/// handle slot; the load offset is visible in the Ghidra output.
pub(crate) const FN_80007220: usize = 0x8000_7220;

/// `0x800C8950`: constructs the embedded `lyt::Layout` object.
///
/// # Original
/// Callee of the three scene constructors (`0x80014F10`, `0x80015D60`,
/// `0x80016F70`).
///
/// # Confidence
/// high - one argument, always `scene + layout_offset`; paired with the
/// destructor `0x800C89A0`.
pub(crate) const FN_800C8950: usize = 0x800C_8950;

/// `0x800C89A0`: destroys the embedded `lyt::Layout` (`flag = -1`).
///
/// # Original
/// Called as `(layout, -1)` from `0x80015F90`.
///
/// # Confidence
/// high - called with the layout pointer and `-1` from `0x80015F90`.
pub(crate) const FN_800C89A0: usize = 0x800C_89A0;

/// `0x800C8CB0`: loads a `.brlyt` from an archive; returns 0 on failure.
///
/// # Original
/// Called as `(layout, "path/name.brlyt", 0)` from `0x80016020` and
/// `0x800171F0`; the archive path strings sit at `0x802FFAA8` and `0x803000D0`.
///
/// # Confidence
/// high - the string argument at the call site is the archive path and the
/// result is tested for zero.
pub(crate) const FN_800C8CB0: usize = 0x800C_8CB0;

/// `0x800C8D60`: resolves a `const char*` table into object pointers.
///
/// # Original
/// Used for `"N_coin_00"`, the two button panes and the nine battery nodes.
///
/// # Confidence
/// medium - call shape and output slots are exact; which NW4R finder this is
/// (pane vs. material vs. texture) is not proven.
pub(crate) const FN_800C8D60: usize = 0x800C_8D60;

/// `0x800C8E40`: sibling resolver (different NW4R finder).
///
/// # Original
/// Used for the 20 battery pieces and the three error button bases.
///
/// # Confidence
/// medium - same shape as [`FN_800C8D60`], different finder address.
pub(crate) const FN_800C8E40: usize = 0x800C_8E40;

/// `0x800C8EB0`: sibling resolver (different NW4R finder).
///
/// # Original
/// Used for the six `pointGet` counter panes and the nine error text panes.
///
/// # Confidence
/// medium - same shape as [`FN_800C8D60`], different finder address.
pub(crate) const FN_800C8EB0: usize = 0x800C_8EB0;

/// `0x800C8F20`: binds animation names to frames for `count` entries.
///
/// # Original
/// Called with `(layout, names, indices, frame, count)` from
/// `0x80016020`/`0x800171F0`.
///
/// # Confidence
/// medium - argument order read from the call sites; the "indices" table is a
/// parallel `u32` array verified in the ELF.
pub(crate) const FN_800C8F20: usize = 0x800C_8F20;

/// `0x800C8FB0`: binds a `.brlan` name table to the layout.
///
/// # Original
/// Called as `(layout, names, count)` with the battery (4) and error (10)
/// animation tables.
///
/// # Confidence
/// medium - name strings end in `.brlan`; the exact NW4R entry point is not
/// resolved.
pub(crate) const FN_800C8FB0: usize = 0x800C_8FB0;

/// `0x800C90F0`: binds a pane/name table plus integer table to the layout.
///
/// # Original
/// Called as `(layout, names, indices, count)`; the integer table maps each
/// pane to an animation index.
///
/// # Confidence
/// medium - call shape exact, role inferred from the paired tables.
pub(crate) const FN_800C90F0: usize = 0x800C_90F0;

/// `0x800C92F0`: starts/stops animation `index` (third arg = "seek" flag).
///
/// # Original
/// `(layout, index, seek)`.
///
/// # Confidence
/// high - call sites use literal indices and a 0/1 flag.
pub(crate) const FN_800C92F0: usize = 0x800C_92F0;

/// `0x800C9380`: clears a per-animation byte (`[entry]->+0x14 = 0`).
///
/// # Original
/// Called as `(layout, slot)` from `0x800166B0` for slots `5` and `6 + player`.
///
/// # Confidence
/// medium - effect inferred from the Ghidra body; the animation entry layout
/// is not otherwise reconstructed.
pub(crate) const FN_800C9380: usize = 0x800C_9380;

/// `0x800C93D0`: layout playback helper (`(layout, 0, 0)`).
///
/// # Original
/// Called from `0x80016020` as `(layout, 0, 0)` before the animation tick.
///
/// # Confidence
/// low - the two zero arguments and the effect are not resolved.
pub(crate) const FN_800C93D0: usize = 0x800C_93D0;

/// `0x800C9500`: advances every animation to its end state.
///
/// # Original
/// Called as `(layout)` from `0x80016020`, `0x80016210`, `0x800167B0` and
/// `0x80017340`.
///
/// # Confidence
/// high - used before "close window" transitions in all three scenes.
pub(crate) const FN_800C9500: usize = 0x800C_9500;

/// `0x800C9560`: per-frame animation tick for the bound animations.
///
/// # Original
/// Called as `(layout)` once per scene update by `0x80016020`, `0x80016210`
/// and `0x80017340`.
///
/// # Confidence
/// high - called once per scene update, immediately before the layout's
/// virtual close method.
pub(crate) const FN_800C9560: usize = 0x800C_9560;

/// `0x800C9610`: returns byte `finished[current]` (`-1` = current index).
///
/// # Original
/// Called as `(layout, -1)`; a zero result means "not finished yet".
///
/// # Confidence
/// medium - the returned byte is only ever compared with zero at the call
/// sites; the table layout comes from the Ghidra body.
pub(crate) const FN_800C9610: usize = 0x800C_9610;

/// `0x800C9A60`: plays animation `index` from `start` to `end` (varargs).
///
/// # Original
/// Called from `0x80014FF0` as `(pane, handle, 0x67, 0x32, 0)`.
///
/// # Confidence
/// medium - arguments are exact; the meaning of the two frame values is
/// inferred from the "play from start to end" shape.
pub(crate) const FN_800C9A60: usize = 0x800C_9A60;

/// `0x800CDC60`: returns the global animation handle at `0x80359CB4`.
///
/// # Original
/// Called with no arguments from `0x80014FF0`, then passed to `0x800C9A60`.
///
/// # Confidence
/// medium - address observed in the Ghidra body; the handle's owner is not
/// resolved.
pub(crate) const FN_800CDC60: usize = 0x800C_DC60;

// ===========================================================================
// Small helpers shared with the UI scenes
// ===========================================================================

/// Narrows a target address to the 32-bit width used by the PowerPC EABI.
///
/// Every call below passes addresses in registers, so the upper 32 bits of a
/// host `usize` are dropped exactly as the original `li`/`lis` sequence would.
///
/// # Original
/// Host-side helper; no binary address. Models the 32-bit address arithmetic of
/// the PowerPC EABI (`addi`, and `lis`+`ori` pairs) used at every call site.
///
/// # Safety
/// Pure arithmetic; safe to call.
///
/// # Confidence
/// high - mechanical conversion.
#[inline]
pub(crate) fn p32(addr: usize) -> u32 {
    addr as u32
}

/// Reads a big-endian `f32` from target memory.
///
/// # Original
/// Host-side helper; models the `lfs` loads of the scene constructors.
///
/// # Safety
/// `addr..addr+4` must be readable in the target address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub(crate) unsafe fn read_f32(addr: usize) -> f32 {
    f32::from_bits(platform::read_u32(addr))
}

/// Writes a big-endian `f32` to target memory.
///
/// # Original
/// Host-side helper; models the `stfs` stores of the scene constructors.
///
/// # Safety
/// `addr..addr+4` must be writable in the target address space.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub(crate) unsafe fn write_f32(addr: usize, value: f32) {
    platform::write_u32(addr, value.to_bits());
}

/// Calls a no-argument function that returns `u32`.
///
/// `crate::platform` only exposes `call0` (returning `()`), but several NW4R
/// helpers return a handle; this wrapper preserves the return register (`r3`).
///
/// # Original
/// Host-side helper; no binary address. Models the `bl` + `mr rX,r3` pair when
/// the call result must be kept.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub(crate) unsafe fn call0_r(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Calls a function with four `u32` arguments.
///
/// `crate::platform` stops at `call3`; the layout helpers take up to five
/// arguments, so the wider forms live here. Arguments go in `r3..r6`.
///
/// # Original
/// Host-side helper; no binary address. Models a `bl` with four integer
/// arguments.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub(crate) unsafe fn call4(addr: usize, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32) -> u32);
    f(a, b, c, d)
}

/// Calls a function with five `u32` arguments.
///
/// Arguments go in `r3..r7`; see [`call4`] for why this is not in
/// `crate::platform`.
///
/// # Original
/// Host-side helper; no binary address. Models a `bl` with five integer
/// arguments.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
pub(crate) unsafe fn call5(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32) -> u32);
    f(a, b, c, d, e)
}

/// Resolves a virtual method pointer.
///
/// Loads the vtable pointer at `obj + vtable_off`, then the function pointer at
/// `vtable + method_off`. The Ghidra output spells this as
/// `(**(code **)(*(int *)(obj + vtable_off) + method_off))`.
///
/// For a layout call the vtable pointer lives at `scene + layout_offset + 8`
/// (scene `+0x0C` for a layout at `+0x04`, `+0x10` for one at `+0x08`); for a
/// member sub-object it lives directly at that member (`+0x1A0`).
///
/// # Original
/// Host-side helper; no binary address. Models the two loads every virtual call
/// in these scenes emits: `lwz rX,(vtable_off)(obj)`, then
/// `lwz rX,(method_off)(rX)`.
///
/// # Safety
/// `obj + vtable_off` must hold a readable vtable pointer and
/// `vtable + method_off` a readable code pointer.
///
/// # Confidence
/// high - shape is identical at every call site.
#[inline]
pub(crate) unsafe fn vtable_method(obj: usize, vtable_off: usize, method_off: usize) -> usize {
    let vtable = platform::read_u32(obj + vtable_off) as usize;
    platform::read_u32(vtable + method_off) as usize
}

/// Copies three consecutive big-endian words (a pane translation vector).
///
/// Source is a pane's `+0x2C` vector; destination is a 12-byte per-widget
/// record. The three words are copied unchanged, so they stay `f32` bit
/// patterns.
///
/// # Original
/// Host-side helper; no binary address. Models the three `lwz`/`stw` pairs in
/// `0x80016020` (`copy_vec3(pane + 0x2C, this + 0x254)` and the next three
/// records).
///
/// # Safety
/// `src..src+12` must be readable and `dst..dst+12` writable.
///
/// # Confidence
/// high - unrolled three-word copy visible in the Ghidra output.
#[inline]
pub(crate) unsafe fn copy_vec3(src: usize, dst: usize) {
    platform::write_u32(dst, platform::read_u32(src));
    platform::write_u32(dst + 4, platform::read_u32(src + 4));
    platform::write_u32(dst + 8, platform::read_u32(src + 8));
}

// ===========================================================================
// BRLAN animation-transform sections
// (promoted from r80180_801c0: 0x80182060..0x80183500)
// ===========================================================================

/// `0x800046B4` - CRT `memset(dst, value, len)` helper.
///
/// `// inferred:` the same helper is called as `FUN_800046b4(buf, 0, 0x20)` in
/// `0x801B8020` and here, i.e. always as a zero-filler. The boot/CRT lane owns
/// the translation ([`crate::boot::mem::memset_wrapper`]); this module only
/// calls it by address.
const MEMSET: usize = 0x8000_46B4;

/// Work-buffer pointer field of the animation-transform object (`+0x58`).
const ANM_WORK_BUFFER: usize = 0x58;
/// Packed count word at `+0x4C` of the animation-transform object.
const ANM_COUNTS_4C: usize = 0x4C;
/// Packed count word at `+0x50` of the animation-transform object.
const ANM_COUNTS_50: usize = 0x50;
/// List element count at `+0x18` of the object used by [`fn_80183500`].
const ANM_LIST_COUNT: usize = 0x18;
/// Embedded list-head word at `+0x1C` of the object used by [`fn_80183500`].
const ANM_LIST_HEAD: usize = 0x1C;

/// Returns the start of the texture-SRT slot array inside an animation
/// transform's work buffer.
///
/// The object stores the work-buffer base at `+0x58`. Bits 28..31 of `+0x4C`
/// hold a four-bit count; `// inferred:` the material-colour section occupies
/// `0x20` bytes per counted entry, so the next section begins at
/// `base + count * 0x20`.
///
/// Caller `0x8017DCC0` uses the result as the base for `RLTS` (texture-SRT)
/// track values, indexing it by `material * 0x14 + texture * 4`.
///
/// # Original
/// - Ghidra: `FUN_80182060` at `0x80182060` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80182060_FUN_80182060.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `object` - animation-transform object; readable through `+0x5C`.
///
/// # Returns
/// Byte address of the first texture-SRT slot. No alignment is applied.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `object` must point to at least `0x5C` readable bytes.
///
/// # Confidence
/// high - the shift/mask arithmetic is observed exactly and the caller proves
/// the section role.
pub unsafe fn fn_80182060(object: *mut u8) -> *mut u8 {
    let object = object as usize;
    let base = read_u32(object + ANM_WORK_BUFFER) as usize;
    let packed = read_u32(object + ANM_COUNTS_4C);

    // bits 28..31 of +0x4C, scaled by 0x20 (one record per material).
    let material_section_size = ((packed >> 0x17) & 0x1E0) as usize;
    (base + material_section_size) as *mut u8
}

/// Returns the start of the texture-pattern section, one step past
/// [`fn_80182060`].
///
/// It adds another `count * 0x14` term, where this `count` is bits 24..27 of
/// `+0x4C` (the previous helper used bits 28..31). The `0x14` stride is the
/// material stride used by the caller when it indexes texture tracks.
/// `// inferred:` "texture pattern" is the next `BRLAN` section after
/// texture-SRT.
///
/// # Original
/// - Ghidra: `FUN_80182080` at `0x80182080` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80182080_FUN_80182080.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `object` - animation-transform object.
///
/// # Returns
/// Byte address of the section following the texture-SRT array.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `object` must point to at least `0x5C` readable bytes.
///
/// # Confidence
/// high for the arithmetic; medium for the "texture pattern" section name
/// (`// inferred:` from its position after the texture-SRT array).
pub unsafe fn fn_80182080(object: *mut u8) -> *mut u8 {
    let object = object as usize;
    let base = read_u32(object + ANM_WORK_BUFFER) as usize;
    let packed = read_u32(object + ANM_COUNTS_4C);

    let material_color_bytes = ((packed >> 0x17) & 0x1E0) as usize; // count * 0x20
    let material_stride_bytes = (((packed >> 0x18) & 0xF) * 0x14) as usize; // count * 0x14
    (base + material_color_bytes + material_stride_bytes) as *mut u8
}

/// Returns the start of the indirect-material section of the animation work
/// buffer.
///
/// This is the most offset-heavy of the three address helpers. `// inferred:`
/// it sums every per-material and per-texture section that precedes the `RLIM`
/// track data: seven `*4` terms (five driven by one bit, one by bits 15..17,
/// one by bits 20..23), one `*0x14` term (bits 24..27), one `*0x20` term
/// (bits 28..31), plus the base pointer at `+0x58`.
///
/// The masks, in source order, are:
/// `>>12 & 4`, `>>5 & 4`, `>>4 & 4`, `>>6 & 4`, `>>18 & 0x3C`,
/// `>>23 & 0x1E0`, `>>13 & 0x1C`, `>>3 & 4`, plus `(>>24 & 0xF) * 0x14`
/// and the base pointer at `+0x58`.
///
/// # Original
/// - Ghidra: `FUN_801820a0` at `0x801820a0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801820a0_FUN_801820a0.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `object` - animation-transform object.
///
/// # Returns
/// Byte address of the first indirect-material slot.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `object` must point to at least `0x5C` readable bytes.
///
/// # Confidence
/// medium - the arithmetic is exact, but the individual packed counts are not
/// named by the binary; their `*4` grouping is inferred from the record sizes.
pub unsafe fn fn_801820a0(object: *mut u8) -> *mut u8 {
    let object = object as usize;
    let packed = read_u32(object + ANM_COUNTS_4C);
    let base = read_u32(object + ANM_WORK_BUFFER) as usize;

    let packed_bytes = ((packed >> 0x0C) & 4)
        + ((packed >> 5) & 4)
        + ((packed >> 4) & 4)
        + ((packed >> 6) & 4)
        + ((packed >> 0x18) & 0xF) * 0x14
        + ((packed >> 0x12) & 0x3C)
        + ((packed >> 0x17) & 0x1E0)
        + ((packed >> 0x0D) & 0x1C)
        + ((packed >> 3) & 4);

    (base + packed_bytes as usize) as *mut u8
}

/// Grows the 0x20-byte record array whose length is stored in bits 28..31 of
/// `+0x50`, zero-filling only the newly added records.
///
/// The existing count comes from the same field. New records are written at
/// `buffer + old_count * 0x20` and each is cleared with 32 zero bytes. The
/// count is then stored back truncated to four bits, exactly as the original
/// `param_2 << 0x1C` does - so a caller passing more than 15 both clears more
/// than 15 records and records only `count & 0xF`.
///
/// # Original
/// - Ghidra: `FUN_80182100` at `0x80182100` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80182100_FUN_80182100.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `object` - animation-transform object.
/// * `count` - new record count; `0` is a no-op.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Clears `0x20 * (count - old_count)` bytes through `0x800046B4`.
/// * Writes `+0x50` (bits 28..31 replaced, other bits preserved).
///
/// # Safety
/// `object` must be readable/writable through `+0x58`; the work buffer must
/// have room for `count` records of `0x20` bytes.
///
/// # Confidence
/// high - control flow and mask are observed exactly.
pub unsafe fn fn_80182100(object: *mut u8, count: u32) {
    if count == 0 {
        return;
    }

    let object = object as usize;
    let packed = read_u32(object + ANM_COUNTS_50);

    // Start of the not-yet-initialised tail: base + old_count * 0x20.
    let mut record = read_u32(object + ANM_WORK_BUFFER) as usize
        + ((packed >> 0x17) & 0x1E0) as usize;

    let mut index = packed >> 0x1C;
    while index < count {
        call3(MEMSET, record as u32, 0, 0x20);
        record += 0x20;
        index += 1;
    }

    // observed: `param_2 << 0x1c | old & 0x0fffffff` (only four count bits kept).
    write_u32(
        object + ANM_COUNTS_50,
        (count << 0x1C) | (packed & 0x0FFF_FFFF),
    );
}

/// Grows the 4-byte record array whose length is stored in bits 20..23 of
/// `+0x50`, filling new records with the four-byte pattern `01 04 3C 00`.
///
/// The destination base is computed from both count words: the work buffer,
/// plus the two sections derived from `+0x4C` (0x20-byte and 0x14-byte
/// strides). The original writes up to eight records per unrolled iteration
/// and then the remainder one at a time; both paths write the same pattern.
///
/// `0x3C` is 60 decimal. The pattern is opaque here - `// unknown:` its
/// meaning; no consumer is in this lane.
///
/// # Original
/// - Ghidra: `FUN_80182190` at `0x80182190` (340 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80182190_FUN_80182190.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `object` - animation-transform object.
/// * `count` - new record count, truncated to four bits when stored.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Writes `4 * (count - old_count)` bytes of pattern data into the work
///   buffer when the array grows (never shrinks on a smaller count).
/// * Writes `+0x50` (bits 20..23 replaced, other bits preserved).
///
/// # Safety
/// `object` must be readable/writable through `+0x58`; the buffer must have
/// room for the new records. `count` is used as a loop bound before the
/// four-bit truncation, exactly as in the original.
///
/// # Confidence
/// medium - the byte pattern and loops are exact; the record role is unknown.
pub unsafe fn fn_80182190(object: *mut u8, count: u32) {
    if count == 0 {
        return;
    }

    let object = object as usize;
    let packed_50 = read_u32(object + ANM_COUNTS_50);
    let packed_4c = read_u32(object + ANM_COUNTS_4C);

    let mut old_count = (packed_50 >> 0x14) & 0xF;
    let base = read_u32(object + ANM_WORK_BUFFER) as usize
        + ((packed_4c >> 0x17) & 0x1E0) as usize
        + ((packed_4c >> 0x18) & 0xF) as usize * 0x14;

    if old_count < count {
        // observed: bulk path only when more than eight records are added.
        if count - old_count > 8 && old_count <= count {
            let mut blocks = ((count - 1) - old_count) >> 3;
            let mut dst = base + old_count as usize * 4;

            if old_count < count - 8 {
                loop {
                    // observed as eight copies unrolled by Ghidra.
                    for _ in 0..8 {
                        write_u8(dst, 0x01);
                        write_u8(dst + 1, 0x04);
                        write_u8(dst + 2, 0x3C);
                        write_u8(dst + 3, 0x00);
                        dst += 4;
                    }
                    old_count += 8;
                    blocks -= 1;
                    if blocks == 0 {
                        break;
                    }
                }
            }
        }

        // Tail, or the whole range when the bulk path was skipped. `dst` is
        // re-derived from the (possibly advanced) count, like the C does.
        let mut remaining = count - old_count;
        let mut dst = base + old_count as usize * 4;
        while remaining != 0 {
            write_u8(dst, 0x01);
            write_u8(dst + 1, 0x04);
            write_u8(dst + 2, 0x3C);
            write_u8(dst + 3, 0x00);
            dst += 4;
            remaining -= 1;
        }
    }

    // observed: `(param_2 & 0xf) << 0x14 | old & 0xff0fffff`.
    write_u32(
        object + ANM_COUNTS_50,
        ((count & 0xF) << 0x14) | (packed_50 & 0xFF0F_FFFF),
    );
}

/// Splices a 0x10-byte descriptor into the linked list embedded at `+0x18` of
/// an animation object, immediately after the list head at `+0x1C`.
///
/// This is a 12-byte wrapper: it builds a one-word stack cell containing the
/// head-node address (`object + 0x1C`) and calls [`fn_8018a590`] with the
/// count at `object + 0x18`. Caller `0x8017DA20` passes descriptors from a
/// `0x10`-byte array; the wrapper itself only touches bytes `+0`/`+4` of the
/// node (the link) and the two words at `+0x18`/`+0x1C` of the object.
///
/// # Original
/// - Ghidra: `FUN_80183500` at `0x80183500` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80183500_FUN_80183500.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `object` - owner of the embedded list.
/// * `node` - descriptor to insert; its first two words become list links.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * Writes the node's two link words.
/// * Writes the list head's newer link and the former first node's older link.
/// * Increments the count at `object + 0x18`.
///
/// # Safety
/// `object` must be writable through `+0x20`; `node` must be a writable
/// two-word link followed by caller-owned data.
///
/// # Confidence
/// medium - the wrapper is exact, but the 0x10-byte descriptor layout is
/// inferred from caller `0x8017DA20`.
pub unsafe fn fn_80183500(object: *mut u8, node: *mut u32) {
    let object = object as usize;

    // observed: local_8[0] = param_1 + 0x1c; FUN_8018a590(param_1 + 0x18, local_8, param_2)
    let head_word = crate::platform::GuestLocal::<u32>::new( (object + ANM_LIST_HEAD) as u32);
    fn_8018a590(
        (object + ANM_LIST_COUNT) as *mut u32,
        head_word.as_ptr() as *const u32,
        node,
    );
}

// ===========================================================================
// `list` - NW4R intrusive-list primitives
// (promoted from r80180_801c0: 0x8018A590..0x8018A6E0)
// ===========================================================================
//
// Not `lyt`-specific. These six functions are the complete API of an NW4R
// intrusive doubly-linked list whose two link words live at a configurable byte
// offset inside each node. They stay in this file because every `lyt` node,
// pane and animation object embeds one, and because the layout-animation
// wrapper [`fn_80183500`] is the only in-range caller.
//
// Header (`fn_8018a5c0`): `{ head, tail, u16 count, u16 link_offset }`.
// Each node link is two words: `+0` = older neighbour, `+4` = newer neighbour.
// `fn_8018a590` is the sentinel-list variant (a head node whose `+4` word is
// the first real node) and is not interchangeable with `fn_8018a5e0`.

/// Inserts `node` immediately after the node stored at `*head_word`.
///
/// The link sits at word 0/1 of each node: word 0 points toward the older end,
/// word 1 toward the newer end. Given a head (or sentinel) node, this reads the
/// old first node from `head + 4`, then:
///
/// ```text
/// node[1] = old_first
/// node[0] = head
/// head[4] = node
/// old_first[0] = node
/// (*count) += 1
/// ```
///
/// # Original
/// - Ghidra: `FUN_8018a590` at `0x8018a590` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a590_FUN_8018a590.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `count` - pointer to the list's element counter.
/// * `head_word` - pointer to a word holding the head/sentinel node address.
/// * `node` - two-word link to splice in.
///
/// # Returns
/// `node`, mirroring the original return of `param_3`.
///
/// # Side effects
/// Writes four words (two in `node`, one in the head, one in the old first
/// node) and increments `*count`.
///
/// # Safety
/// `head_word` must be readable; the head node must be readable/writable at
/// `+4`; `node` must be writable at `+0..+8`; `count` must be writable.
///
/// # Confidence
/// high - link update order is observed exactly.
pub unsafe fn fn_8018a590(count: *mut u32, head_word: *const u32, node: *mut u32) -> *mut u32 {
    let count = count as usize;
    let head = read_u32(head_word as usize) as usize;
    let old_first = read_u32(head + 4) as usize;
    let node = node as usize;

    write_u32(node + 4, old_first as u32);
    write_u32(node, head as u32);
    write_u32(head + 4, node as u32);
    write_u32(old_first, node as u32);
    write_u32(count, read_u32(count).wrapping_add(1));

    node as *mut u32
}

/// Initialises an intrusive list header and records the per-node link offset.
///
/// Clears head, tail and count, then stores `link_offset` at `+0x0A`. Every
/// other function in this cluster reads that offset back, so one list
/// implementation can serve nodes with links at any byte offset.
///
/// # Original
/// - Ghidra: `FUN_8018a5c0` at `0x8018a5c0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a5c0_FUN_8018a5c0.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `list` - list header to initialise.
/// * `link_offset` - byte offset from a node to its two-word link.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x00` (head = 0), `+0x04` (tail = 0), `+0x08` (count = 0) and
/// `+0x0A` (link offset).
///
/// # Safety
/// `list` must be writable for at least 12 bytes.
///
/// # Confidence
/// high - exact stores are observed.
pub unsafe fn fn_8018a5c0(list: *mut u8, link_offset: u16) {
    let list = list as usize;
    write_u32(list, 0);
    write_u32(list + 4, 0);
    write_u16(list + 8, 0);
    write_u16(list + 0x0A, link_offset);
}

/// Appends `node` at the tail (newest end) of an intrusive list.
///
/// If the list is empty, the node becomes both head and tail and both of its
/// links are cleared. Otherwise the new node's older link points at the old
/// tail, its newer link is cleared, the old tail's newer link points at the new
/// node, and the tail becomes the new node. The count is incremented in both
/// cases.
///
/// # Original
/// - Ghidra: `FUN_8018a5e0` at `0x8018a5e0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a5e0_FUN_8018a5e0.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `list` - initialised list header (see [`fn_8018a5c0`]).
/// * `node` - node whose two-word link at `list.link_offset` is free.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the node link, possibly the old tail's newer link, the header's tail
/// pointer and the `u16` count.
///
/// # Safety
/// `list` must be readable/writable for 12 bytes and initialised; `node` must
/// be writable at `link_offset..link_offset + 8`.
///
/// # Confidence
/// high - exact stores are observed.
pub unsafe fn fn_8018a5e0(list: *mut u8, node: *mut u8) {
    let list = list as usize;
    let link_offset = read_u16(list + 0x0A) as usize;
    let node_addr = node as usize;

    let head = read_u32(list);
    let node_link = node_addr + link_offset;

    if head == 0 {
        // observed: first element clears both links and becomes head and tail.
        write_u32(node_link + 4, 0);
        write_u32(node_link, 0);
        write_u32(list, node_addr as u32);
        write_u32(list + 4, node_addr as u32);
        write_u16(list + 8, read_u16(list + 8).wrapping_add(1));
        return;
    }

    let tail = read_u32(list + 4) as usize;
    write_u32(node_link, tail as u32); // older link = old tail
    write_u32(node_link + 4, 0); // newest end
    write_u32(tail + link_offset + 4, node_addr as u32); // old tail's newer link
    write_u32(list + 4, node_addr as u32);
    write_u16(list + 8, read_u16(list + 8).wrapping_add(1));
}

/// Unlinks `node` from an intrusive list and clears its two link words.
///
/// Both neighbours are optional: a zero older link means the node is the head
/// (so the header's head becomes the newer neighbour); a zero newer link means
/// it is the tail (so the header's tail becomes the older neighbour). The
/// count is decremented unconditionally, even if the links did not belong to
/// this list - the original has no validation.
///
/// # Original
/// - Ghidra: `FUN_8018a650` at `0x8018a650` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a650_FUN_8018a650.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `list` - list header the node currently belongs to.
/// * `node` - node to remove.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Repairs up to two neighbour links, writes the two header end pointers, clears
/// the node link, and decrements the `u16` count.
///
/// # Safety
/// `list` and `node` must be readable/writable at the offsets used; `node`
/// must carry a link at `list.link_offset`.
///
/// # Confidence
/// high - exact stores are observed.
pub unsafe fn fn_8018a650(list: *mut u8, node: *mut u8) {
    let list = list as usize;
    let link_offset = read_u16(list + 0x0A) as usize;
    let link = node as usize + link_offset;

    let older = read_u32(link) as usize;
    let newer = read_u32(link + 4) as usize;

    if older == 0 {
        write_u32(list, newer as u32);
    } else {
        write_u32(older + link_offset + 4, newer as u32);
    }

    if newer == 0 {
        write_u32(list + 4, older as u32);
    } else {
        write_u32(newer + link_offset, older as u32);
    }

    write_u32(link, 0);
    write_u32(link + 4, 0);
    write_u16(list + 8, read_u16(list + 8).wrapping_sub(1));
}

/// Returns the node after `node` in iteration order, or the head when `node`
/// is null.
///
/// Iteration follows the newer link (`link + 4`), so calling this repeatedly
/// walks from oldest to newest and finally returns 0.
///
/// # Original
/// - Ghidra: `FUN_8018a6c0` at `0x8018a6c0` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a6c0_FUN_8018a6c0.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `list` - list header.
/// * `node` - current node, or null to start at the head.
///
/// # Returns
/// Next node, or 0 at the end of the list.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `list` must be readable for 12 bytes; `node`, if non-null, must be readable
/// at `link_offset + 4`.
///
/// # Confidence
/// high - exact load is observed.
pub unsafe fn fn_8018a6c0(list: *const u8, node: *const u8) -> *mut u8 {
    let list = list as usize;
    let offset = read_u16(list + 0x0A) as usize;

    if node.is_null() {
        read_u32(list) as *mut u8
    } else {
        read_u32(node as usize + offset + 4) as *mut u8
    }
}

/// Returns the `index`-th node in iteration order (0 = head), or null.
///
/// The walk is the same as repeated [`fn_8018a6c0`] calls but starts from the
/// head directly. `index` is a signed 32-bit value: negative indices can never
/// match the non-negative counter, so they exhaust the list and return null.
///
/// # Original
/// - Ghidra: `FUN_8018a6e0` at `0x8018a6e0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a6e0_FUN_8018a6e0.c`
/// - Staged in `src/subsystems/r80180_801c0.rs` before this move.
///
/// # Arguments
/// * `list` - list header.
/// * `index` - zero-based position from the head.
///
/// # Returns
/// Node pointer, or 0 when the index is out of range.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `list` must be readable for 12 bytes; every node reached must be readable
/// at `link_offset + 4`.
///
/// # Confidence
/// high - exact loop is observed.
pub unsafe fn fn_8018a6e0(list: *const u8, index: i32) -> *mut u8 {
    let list = list as usize;
    let offset = read_u16(list + 0x0A) as usize;

    let mut current = 0usize;
    let mut position: i32 = 0;

    loop {
        if current == 0 {
            current = read_u32(list) as usize;
        } else {
            current = read_u32(current + offset + 4) as usize;
        }

        if current == 0 {
            return core::ptr::null_mut();
        }
        if index == position {
            return current as *mut u8;
        }
        position += 1;
    }
}

// ===========================================================================
// Layout transform and global registration
// (promoted from r80140_80180: 0x8016C9E0, 0x8016D180)
// ===========================================================================

/// `0x801C04D0`: resets a layout's translation. `// observed:` signature
/// `void f(float *)`; it writes identity-ish values using `r2` constants.
const LAYOUT_RESET_POSITION: usize = 0x801C_04D0;

/// `0x801C0BD0`: sets a layout's translation from three `f64` values.
/// `// observed:` `void f(double, double, double, void *)`.
const LAYOUT_SET_POSITION: usize = 0x801C_0BD0;

/// `0x8016EF50`: applies three 16-bit rotation components. Its body calls
/// `0x8016EDD0` with the second pointer, `0x8016ECB0` with the first and
/// `0x8016EEF0` with the third (`// observed:` `8016ef50_FUN_8016ef50.c`).
const LAYOUT_SET_ROTATION: usize = 0x8016_EF50;

/// `0x801C0C60`: builds a scale matrix from three `f64` values.
const LAYOUT_SCALE_MATRIX: usize = 0x801C_0C60;

/// `0x801C0540`: concatenates two matrices (`dst, src, dst` at the call site).
const LAYOUT_CONCAT_MATRIX: usize = 0x801C_0540;

/// `0x8016C960`: layout initialiser called before the list insertion. Returns
/// non-zero on success.
const LAYOUT_INIT: usize = 0x8016_C960;

/// `0x8016D4B0`: removes a layout from the global list.
const LAYOUT_UNREGISTER: usize = 0x8016_D4B0;

/// `0x802E1AB4`: `strcmp`-shaped name comparison (`// observed:` it returns 0
/// for equal strings in `8014a5d0_FUN_8014a5d0.c`).
const NAME_COMPARE: usize = 0x802E_1AB4;

/// Global layout list sentinel object, used for the list head/tail links.
/// `// observed:` `&DAT_80377b1c`.
const LAYOUT_LIST_SENTINEL: u32 = 0x8037_7B1C;

/// Global layout list: last node (`DAT_80377c30`).
const LAYOUT_LIST_TAIL: usize = 0x8037_7C30;

/// Global layout list: first node (`DAT_80377c34`); equals
/// [`LAYOUT_LIST_SENTINEL`] while the list is empty.
const LAYOUT_LIST_HEAD: usize = 0x8037_7C34;

/// Global layout list: node count (`DAT_80377c38`).
const LAYOUT_LIST_COUNT: usize = 0x8037_7C38;

/// Calls `addr(a, b, c, d)` where `a..c` are `f64` and `d` is an integer.
///
/// PowerPC keeps the doubles in `f1..f3` and the integer in `r3`; the original
/// layout helpers have the signature `void f(double, double, double, void *)`.
///
/// # Original
/// Host-side helper; no binary address. Models the `bl` emitted by
/// `0x8016C9E0` for `0x801C0BD0` and `0x801C0C60`
/// (`work/ghidra/main-gb/export/8016c9e0_FUN_8016c9e0.c`).
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - the call sites cast each `f32` to `f64` and pass a pointer last.
#[inline]
unsafe fn call_f64x3_u32(addr: usize, a: f64, b: f64, c: f64, d: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f64, f64, f64, u32) -> u32);
    f(a, b, c, d)
}

/// Applies translation, rotation and scale to a layout in one call.
///
/// # Original
/// - Ghidra: `FUN_8016c9e0` at `0x8016c9e0` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c9e0_FUN_8016c9e0.c`
/// - Staged in `src/subsystems/r80140_80180.rs` before this move.
///
/// # Arguments
/// * `layout` - layout/3-D object to modify.
/// * `position` - pointer to three `f32` (X, Y, Z), or `0` to reset.
/// * `rotation` - pointer to three `u16` angles at `+0`, `+2`, `+4`, or `0`.
/// * `scale` - pointer to three `f32` (X, Y, Z), or `0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the layout's translation and rotation, and concatenates a scale
/// matrix. All work happens through the `0x801C....` layout helpers.
///
/// # Safety
/// `layout` must be a live layout object; each non-null pointer must cover the
/// number of components read (3 x `f32` or 3 x `u16`).
///
/// # Confidence
/// medium-high - control flow is exact; the axis order inside `0x8016EF50` is
/// not proven, so it is described by call order rather than by axis name.
pub unsafe fn layout_set_transform(layout: usize, position: usize, rotation: usize, scale: usize) {
    if position == 0 {
        platform::call1(LAYOUT_RESET_POSITION, layout as u32);
    } else {
        call_f64x3_u32(
            LAYOUT_SET_POSITION,
            read_f32(position) as f64,
            read_f32(position + 4) as f64,
            read_f32(position + 8) as f64,
            layout as u32,
        );
    }
    if rotation != 0 {
        // observed: the original copies each u16 into its own stack slot and
        // passes the three slot addresses in component order (first, second,
        // third); the callee applies them in its own order.
        let rx = crate::platform::GuestLocal::new( platform::read_u16(rotation));
        let ry = crate::platform::GuestLocal::new( platform::read_u16(rotation + 2));
        let rz = crate::platform::GuestLocal::new( platform::read_u16(rotation + 4));
        call4(
            LAYOUT_SET_ROTATION,
            layout as u32,
            ((rx.as_ptr() as *const u16)) as u32,
            ((ry.as_ptr() as *const u16)) as u32,
            ((rz.as_ptr() as *const u16)) as u32,
        );
    }
    if scale != 0 {
        // observed: `undefined1 auStack_48[60]` is the scratch matrix.
        let mut matrix = crate::platform::GuestLocal::new( [0.0f32; 16]);
        call_f64x3_u32(
            LAYOUT_SCALE_MATRIX,
            read_f32(scale) as f64,
            read_f32(scale + 4) as f64,
            read_f32(scale + 8) as f64,
            matrix.as_mut_ptr() as u32,
        );
        platform::call3(
            LAYOUT_CONCAT_MATRIX,
            layout as u32,
            matrix.as_ptr() as u32,
            layout as u32,
        );
    }
}

/// Registers a layout in the global layout list.
///
/// # Original
/// - Ghidra: `FUN_8016d180` at `0x8016d180` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d180_FUN_8016d180.c`
/// - Staged in `src/subsystems/r80140_80180.rs` before this move.
///
/// # Arguments
/// * `layout` - layout object to register.
/// * `a2`, `a3` - forwarded to the layout initialiser `0x8016C960`.
///
/// # Returns
/// `1` when the layout is registered (or was already registered with an empty
/// name), `0` when the initialiser fails.
///
/// # Side effects
/// * May unregister the layout first (`0x8016D4B0`).
/// * Inserts it at the tail of the global list and increments
///   `DAT_80377C38`.
/// * Sets `+0x11C` (registered) and `+0x11D` (enabled/visible) to `1`.
///
/// # Safety
/// `layout` must be a live object of at least `0x120` bytes; the global list
/// must be in a consistent state.
///
/// # Confidence
/// high - the list operations are exact, and the two operands Ghidra dropped
/// are visible in the instruction stream: `addi r3,r3,4` at `0x8016D1B0`
/// leaves `r4 = a2` and `r5 = a3` (`mr r30,r4` / `mr r31,r5` at
/// `0x8016D190..D198`) for the `strcmp`-shaped helper `0x802E1AB4`.
pub unsafe fn layout_register(layout: usize, a2: u32, a3: u32) -> u32 {
    if platform::read_u8(layout + 0x11C) != 0 {
        // observed: `addi r3,r3,4; bl 0x802E1AB4` with `r4`/`r5` still holding
        // `a2`/`a3`. A zero result means "nothing to do".
        if platform::call3(NAME_COMPARE, (layout + 4) as u32, a2, a3) == 0 {
            platform::write_u8(layout + 0x11D, 1);
            return 1;
        }
        platform::call1(LAYOUT_UNREGISTER, layout as u32);
    }
    if platform::call3(LAYOUT_INIT, layout as u32, a2, a3) == 0 {
        return 0;
    }
    if platform::read_u32(LAYOUT_LIST_HEAD) == LAYOUT_LIST_SENTINEL {
        // observed: first node points both ways at the sentinel.
        platform::write_u32(layout + 0x118, LAYOUT_LIST_SENTINEL);
        platform::write_u32(layout + 0x114, LAYOUT_LIST_SENTINEL);
        platform::write_u32(LAYOUT_LIST_HEAD, layout as u32);
    } else {
        let tail = platform::read_u32(LAYOUT_LIST_TAIL) as usize;
        // observed: `+0x114` = previous, `+0x118` = next.
        platform::write_u32(layout + 0x114, tail as u32);
        platform::write_u32(layout + 0x118, LAYOUT_LIST_SENTINEL);
        platform::write_u32(tail + 0x118, layout as u32);
    }
    platform::write_u32(
        LAYOUT_LIST_COUNT,
        platform::read_u32(LAYOUT_LIST_COUNT).wrapping_add(1),
    );
    platform::write_u32(LAYOUT_LIST_TAIL, layout as u32);
    platform::write_u8(layout + 0x11C, 1);
    platform::write_u8(layout + 0x11D, 1);
    1
}

// ===========================================================================
// Wave 3: layout lifecycle (`0x8016C730..0x8016D590`)
// ===========================================================================
//
// The layout object used here is the one described in the module header: a
// vtable at `+0x00`, an inline name buffer at `+0x04`, a four-byte "id" slot at
// `+0x24`, list links at `+0x114`/`+0x118` and state bytes at `+0x11C`/`+0x11D`.
// `+0x11C` means "linked into the global registry"; `+0x11D` means "enabled".
//
// Every virtual call below is emitted as `bl` to the slot address, so the
// callee receives the object in `r3`. Ghidra sometimes prints an empty
// argument list for those slots (`(**...)( )`); the object is passed
// explicitly here and marked `// inferred:` where the decompiler lost it.

/// `0x802E1938`: unbounded C string copy (`strcpy`).
///
/// observed: the body copies byte-by-byte with the usual word-at-a-time
/// fast path and stops after the NUL, and the call site writes the terminator
/// again for safety.
const W3_STRCPY: usize = 0x802E_1938;

/// `0x8016C9E0`: the transform builder translated as [`layout_set_transform`].
///
/// It is called by address in [`fn_8016d280`] because the original passes a
/// 56-byte scratch matrix as its fourth argument, which does not match the
/// position/rotation/scale signature of the Rust translation.
const W3_LAYOUT_SET_TRANSFORM: usize = 0x8016_C9E0;

/// `0x8016CEA0`: "prepare a layout from three components" helper.
///
/// observed: checks `0x802967B0(layout + 0x74)`, then calls
/// `0x8016C9E0` and the object's virtual `+0x8C`/`+0x94` slots. `// unknown:`
/// the resource object behind `+0x74` is not reconstructed.
const W3_LAYOUT_PREPARE: usize = 0x8016_CEA0;

/// `0x8016EBF0`: builds the first-axis rotation matrix from one `s16` angle.
const W3_ROT_MATRIX_A: usize = 0x8016_EBF0;

/// `0x8016ECB0`: applies the first-axis rotation matrix.
const W3_ROT_APPLY_A: usize = 0x8016_ECB0;

/// `0x8016EDD0`: applies the second-axis rotation matrix.
const W3_ROT_APPLY_B: usize = 0x8016_EDD0;

/// `0x8016EEF0`: applies the third-axis rotation matrix.
const W3_ROT_APPLY_C: usize = 0x8016_EEF0;

/// Clears, names, tags and notifies a layout object.
///
/// Sequence (observed):
/// 1. virtual `+0x10` (no recovered arguments - this is passed here);
/// 2. `strcpy(layout + 4, name)`;
/// 3. `layout[+0x24] = id`;
/// 4. virtual `+0x0C(this)` - the "changed" notification.
///
/// `// inferred:` the two virtual slots are the NW4R layout begin/end-notify
/// pair used by [`layout_register`], which calls this function right before
/// linking the layout; the exact slot names are not recovered.
///
/// # Original
/// - Ghidra: `FUN_8016c960` at `0x8016C960` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c960_FUN_8016c960.c`
///
/// # Arguments
/// * `layout` - layout object with a vtable at `+0x00`.
/// * `name` - NUL-terminated name copied to `layout + 4`.
/// * `id` - word stored at `layout + 0x24` (resource/animation id).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls two virtual methods, copies the name, writes `+0x24`.
///
/// # Safety
/// `layout` must be writable through `+0x24`; `name` must be a readable
/// NUL-terminated string. The vtable slots must be valid code pointers.
///
/// # Confidence
/// medium - the byte copies and stores are exact; the two virtual slots are
/// passed `this` by inference because Ghidra dropped their argument lists.
pub unsafe fn layout_init(layout: usize, name: u32, id: u32) {
    // observed: `(**(code **)(*param_1 + 0x10))();` with `r3` still holding
    // `layout`. Ghidra prints no arguments.
    let begin = vtable_method(layout, 0, 0x10);
    platform::call1(begin, p32(layout));

    platform::call2(W3_STRCPY, p32(layout + 4), name);

    platform::write_u32(layout + 0x24, id);

    let changed = vtable_method(layout, 0, 0x0C);
    platform::call1(changed, p32(layout));
}

/// Initialises a layout through `0x8016CEA0` and marks it registered/enabled.
///
/// The helper at `0x8016CEA0` is the "build from three components" entry point
/// (it calls `0x8016C9E0` and the virtual `+0x8C`/`+0x94` slots). This wrapper
/// only translates the guard and the two state bytes.
///
/// # Original
/// - Ghidra: `FUN_8016d590` at `0x8016D590` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d590_FUN_8016d590.c`
///
/// # Arguments
/// * `layout` - layout object accepted by `0x8016CEA0`.
///
/// # Returns
/// `true` when the helper reported success.
///
/// # Side effects
/// On success writes `+0x11C = 1` (registered) and `+0x11D = 1` (enabled).
///
/// # Safety
/// `layout` must be a live layout and `0x8016CEA0` must accept it.
///
/// # Confidence
/// medium - the guard and both stores are exact; the helper's internal
/// arguments are not recovered (see [`W3_LAYOUT_PREPARE`]).
pub unsafe fn fn_8016d590(layout: usize) -> bool {
    let ok = platform::call1(W3_LAYOUT_PREPARE, p32(layout));
    if ok != 0 {
        platform::write_u8(layout + 0x11C, 1);
        platform::write_u8(layout + 0x11D, 1);
        return true;
    }
    false
}

/// Creates a child object from a resource and commits it to the layout.
///
/// The three virtual slots are the same prepare/set/commit sequence used by
/// [`fn_8016d280`]: `+0x9C` asks whether the resource can be built, `+0x8C`
/// supplies the transform/data pointer, `+0x94` finishes the build.
///
/// # Original
/// - Ghidra: `FUN_8016cb60` at `0x8016CB60` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016cb60_FUN_8016cb60.c`
///
/// # Arguments
/// * `layout` - receiver of the virtual calls.
/// * `a2`, `a3` - present in the ABI but unused by the body.
/// * `data` - pointer forwarded to the virtual `+0x8C` slot.
///
/// # Returns
/// `true` when the virtual `+0x9C` check succeeded.
///
/// # Side effects
/// Calls virtual `+0x8C` and `+0x94` only on success.
///
/// # Safety
/// `layout` must have a valid vtable; `data` must be valid for whatever the
/// `+0x8C` slot expects.
///
/// # Confidence
/// medium - the call order and the boolean result are exact; the slot roles
/// are inferred from the neighbouring layout builders.
pub unsafe fn fn_8016cb60(layout: usize, _a2: u32, _a3: u32, data: u32) -> bool {
    // inferred: the `+0x9C` slot is called with the object, although Ghidra
    // prints no arguments.
    let can_build = platform::call1(vtable_method(layout, 0, 0x9C), p32(layout));
    if can_build != 0 {
        platform::call2(vtable_method(layout, 0, 0x8C), p32(layout), data);
        platform::call1(vtable_method(layout, 0, 0x94), p32(layout));
        return true;
    }
    false
}

/// Creates a child object after building a 56-byte transform from three
/// components.
///
/// This is [`fn_8016cb60`] with a transform step: `0x8016C9E0` is called as
/// `(component1, component2, component3, scratch_matrix)` and the scratch
/// matrix is then handed to the virtual `+0x8C` slot.
///
/// # Original
/// - Ghidra: `FUN_8016d280` at `0x8016D280` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d280_FUN_8016d280.c`
///
/// # Arguments
/// * `layout` - receiver of the virtual calls.
/// * `a2`, `a3` - unused by the body.
/// * `c1`, `c2`, `c3` - three component pointers forwarded to `0x8016C9E0`.
///
/// # Returns
/// `true` when the virtual `+0x9C` check succeeded.
///
/// # Side effects
/// Fills the stack matrix through `0x8016C9E0`, then calls virtual `+0x8C`
/// and `+0x94` on success.
///
/// # Safety
/// The three component pointers must satisfy `0x8016C9E0`; `layout` must have
/// a valid vtable.
///
/// # Confidence
/// medium - the call shape is exact; `// unknown:` which component is
/// position, rotation or scale (the scratch matrix is the fourth argument,
/// unlike the Rust [`layout_set_transform`] signature).
pub unsafe fn fn_8016d280(
    layout: usize,
    _a2: u32,
    _a3: u32,
    c1: u32,
    c2: u32,
    c3: u32,
) -> bool {
    // observed: `undefined1 auStack_48[56]` scratch matrix.
    let mut scratch = crate::platform::GuestLocal::new( [0u32; 14]);

    let can_build = platform::call1(vtable_method(layout, 0, 0x9C), p32(layout));
    if can_build == 0 {
        return false;
    }

    call4(
        W3_LAYOUT_SET_TRANSFORM,
        c1,
        c2,
        c3,
        scratch.as_mut_ptr() as u32,
    );
    platform::call2(
        vtable_method(layout, 0, 0x8C),
        p32(layout),
        scratch.as_ptr() as u32,
    );
    platform::call1(vtable_method(layout, 0, 0x94), p32(layout));
    true
}

/// Unlinks a layout from the global registry at `0x80377B1C`.
///
/// The guard is `+0x11C != 0`. The two link words are at `+0x114` (previous)
/// and `+0x118` (next); the registry sentinel is `0x80377B1C`, the tail is
/// `0x80377C30`, the head is `0x80377C34` and the count is `0x80377C38`.
///
/// The original updates the head through a temporary even when the previous
/// link is the sentinel; that store is reproduced because it is observable
/// (it is a no-op value-wise).
///
/// # Original
/// - Ghidra: `FUN_8016d4b0` at `0x8016D4B0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d4b0_FUN_8016d4b0.c`
///
/// # Arguments
/// * `layout` - layout object currently in the registry.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls virtual `+0x10`, repairs up to two links, clears both link words,
/// decrements the count and clears `+0x11C`.
///
/// # Safety
/// `layout` must be registered and its neighbours must be live links.
///
/// # Confidence
/// high - every load and store is explicit and the globals match
/// [`layout_register`].
pub unsafe fn layout_unregister(layout: usize) {
    if platform::read_u8(layout + 0x11C) == 0 {
        return;
    }

    // observed: Ghidra prints the slot with no arguments.
    let notify = vtable_method(layout, 0, 0x10);
    platform::call1(notify, p32(layout));

    let prev = platform::read_u32(layout + 0x114) as usize;
    let next = platform::read_u32(layout + 0x118) as usize;

    if prev != LAYOUT_LIST_SENTINEL as usize {
        platform::write_u32(prev + 0x118, next as u32);
        // observed: the C reloads the head here (`iVar1 = DAT_80377c34`).
        let head = platform::read_u32(LAYOUT_LIST_HEAD);
        platform::write_u32(LAYOUT_LIST_HEAD, head);
    } else {
        platform::write_u32(LAYOUT_LIST_HEAD, next as u32);
    }

    if next == LAYOUT_LIST_SENTINEL as usize {
        platform::write_u32(LAYOUT_LIST_TAIL, prev as u32);
    } else {
        platform::write_u32(next + 0x114, prev as u32);
    }

    platform::write_u32(layout + 0x114, 0);
    platform::write_u32(layout + 0x118, 0);
    platform::write_u32(
        LAYOUT_LIST_COUNT,
        platform::read_u32(LAYOUT_LIST_COUNT).wrapping_sub(1),
    );
    platform::write_u8(layout + 0x11C, 0);
}

/// Ticks every registered layout and drops the ones that report "finished".
///
/// Iteration starts at the head (`0x80377C34`) and follows `+0x118`. For each
/// node the virtual `+0xB8` is called; a zero result unregisters the layout,
/// then `+0x11D` (enabled) is cleared. The next pointer is read before the
/// virtual call, so unlinking the current node is safe.
///
/// # Original
/// - Ghidra: `FUN_8016c730` at `0x8016C730` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c730_FUN_8016c730.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`layout_unregister`] for expired layouts and clears `+0x11D` on
/// every visited layout.
///
/// # Safety
/// The global layout list must be consistent; every node must be a live
/// layout with a valid vtable.
///
/// # Confidence
/// high - the traversal order and both effects are exact.
pub unsafe fn fn_8016c730() {
    let sentinel = LAYOUT_LIST_SENTINEL as usize;

    // observed: `piVar1 = &DAT_80377b1c; if (head != sentinel) piVar1 = head;`
    let mut node = platform::read_u32(LAYOUT_LIST_HEAD) as usize;

    while node != sentinel {
        let next = platform::read_u32(node + 0x118) as usize;
        let keep = platform::call1(vtable_method(node, 0, 0xB8), p32(node));
        if keep == 0 {
            layout_unregister(node);
        }
        platform::write_u8(node + 0x11D, 0);
        node = next;
    }
}

// ===========================================================================
// Wave 3: rotation helpers (`0x8016ECB0..0x8016EF50`)
// ===========================================================================
//
// Each helper takes a 4x4 transform and one signed 16-bit angle. A zero angle
// is a no-op (the original skips the matrix build). The matrix is built in a
// 60-byte stack buffer and concatenated into the transform with `0x801C0540`,
// i.e. `transform = transform * rotation`.

/// Applies one rotation component (first axis of the three-slot call).
///
/// The matrix is built by `0x8016EBF0` from the copied `s16` and concatenated
/// with `0x801C0540(transform, matrix, transform)`.
///
/// # Original
/// - Ghidra: `FUN_8016ecb0` at `0x8016ECB0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ecb0_FUN_8016ecb0.c`
///
/// # Arguments
/// * `transform` - 4x4 matrix/transform to modify in place.
/// * `angle` - pointer to one `s16`; `0` means "no rotation".
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 60 bytes of scratch and updates `transform` in place.
///
/// # Safety
/// `transform` must be a writable 4x4 matrix; `angle` must be readable.
///
/// # Confidence
/// high for the control flow; medium for the axis name (`// unknown:` which
/// axis `0x8016EBF0` builds - the slot order is observed, the axes are not).
pub unsafe fn fn_8016ecb0(transform: usize, angle: usize) {
    if platform::read_u16(angle) == 0 {
        return;
    }

    let mut value = crate::platform::GuestLocal::new( platform::read_u16(angle));
    let mut matrix = crate::platform::GuestLocal::new( [0u32; 15]); // observed: `auStack_40[60]`.
    platform::call2(
        W3_ROT_MATRIX_A,
        matrix.as_mut_ptr() as u32,
        (value.as_ptr() as *const u16) as u32,
    );
    platform::call3(
        LAYOUT_CONCAT_MATRIX,
        transform as u32,
        matrix.as_ptr() as u32,
        transform as u32,
    );
}

/// Applies the three rotation components in the order the original uses.
///
/// Slot order is observed and deliberately unusual: the third argument is
/// applied first (`0x8016EDD0`), then the first (`0x8016ECB0`), then the
/// second (`0x8016EEF0`). This is the callee behind
/// [`layout_set_transform`]'s rotation block.
///
/// # Original
/// - Ghidra: `FUN_8016ef50` at `0x8016EF50` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ef50_FUN_8016ef50.c`
///
/// # Arguments
/// * `transform` - 4x4 transform to modify.
/// * `first`, `second`, `third` - pointers to three `s16` angles.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Concatenates up to three rotation matrices into `transform`.
///
/// # Safety
/// All four pointers must be valid; zero angles are skipped.
///
/// # Confidence
/// high - the three call targets and the copying of one `s16` each are exact;
/// `// unknown:` the axis semantics of `first`/`second`/`third`.
pub unsafe fn fn_8016ef50(
    transform: usize,
    first: usize,
    second: usize,
    third: usize,
) {
    let mut c = crate::platform::GuestLocal::new( platform::read_u16(third));
    platform::call2(W3_ROT_APPLY_B, p32(transform), (c.as_ptr() as *const u16) as u32);

    let mut a = crate::platform::GuestLocal::new( platform::read_u16(first));
    platform::call2(W3_ROT_APPLY_A, p32(transform), (a.as_ptr() as *const u16) as u32);

    let mut b = crate::platform::GuestLocal::new( platform::read_u16(second));
    platform::call2(W3_ROT_APPLY_C, p32(transform), (b.as_ptr() as *const u16) as u32);
}

// ===========================================================================
// Wave 4: pane construction, pane-tree iteration and animation state
// (0x8016CF50..0x8016E8F0)
// ===========================================================================
//
// The bodies below are the next layer of `lyt`: they build child objects, walk
// the pane tree, drive the two-state open/close animation timer, and allocate
// the small animation resources. Every external call is by absolute address;
// the addresses are constants rather than invented symbol names because the
// binary export has no names for them.

/// `0x802967B0`: "resource handle is non-null" check.
///
/// unknown: the object behind the handle is not resolved. The same helper
/// guards [`fn_8016d590`]'s callee `0x8016CEA0`, so it is treated as a
/// validity test.
const FN_802967B0: usize = 0x8029_67B0;

/// `0x802967E0`: "get the resource behind a handle" accessor.
///
/// unknown: the returned object is only forwarded to `0x802862C0`.
const FN_802967E0: usize = 0x8029_67E0;

/// `0x80228FF0`: walks a tree/list and returns the next node.
///
/// observed: called as `(tree + 0x38, previous)` with `0` to start and the
/// previous node afterwards; a zero result ends the walk.
const FN_80228FF0: usize = 0x8022_8FF0;

/// `0x802862C0`: registers a visitor callback `(handle, fn, context, 1)`.
///
/// unknown: the registration object is not resolved.
const FN_802862C0: usize = 0x8028_62C0;

/// `0x802D85C0`: destructor/thunk used by the pane-tree teardown wrapper.
///
/// unknown: the MSL routine's exact role (free vs. destructor) is not proven.
const FN_802D85C0: usize = 0x802D_85C0;

/// `0x802B9280`: releases an object (`operator delete` shape).
///
/// observed: every caller guards it with `object != 0 && flag > 0`.
const FN_802B9280: usize = 0x802B_9280;

/// `0x802DB6C0`: constructs the 48-byte scoped helper at the given address.
///
/// unknown: the helper's class and fields are not recovered.
const FN_802DB6C0: usize = 0x802D_B6C0;

/// `0x802DB750`: scoped-helper step (`(helper, 0)`).
const FN_802DB750: usize = 0x802D_B750;

/// `0x802DB8C0`: scoped-helper step (`(helper, component)`).
const FN_802DB8C0: usize = 0x802D_B8C0;

/// `0x802DB970`: scoped-helper teardown.
const FN_802DB970: usize = 0x802D_B970;

/// `0x802DB710`: scoped-helper release (`(helper, -1)`).
const FN_802DB710: usize = 0x802D_B710;

/// `0x8016D090`: the `+0x0C` pane visitor, registered by [`fn_8016d120`].
const FN_8016D090: usize = 0x8016_D090;

/// `0x8016D3C0`: the `+0x120` pane visitor, registered by [`fn_8016d440`].
const FN_8016D3C0: usize = 0x8016_D3C0;

/// `0x80329AD8`: vtable of the animation-state object built by
/// [`fn_8016dc00`].
///
/// observed: stored as the first word of the object; the table itself is
/// outside this lane.
const ANM_STATE_VTABLE: usize = 0x8032_9AD8;

/// Attaches a prepared child object through the layout's virtual slots.
///
/// The guard is the resource handle at `object + 0x74` (the same handle that
/// `0x8016CEA0` checks before creating a child). When it is non-null the
/// object's virtual `+0x8C` ("set data") receives `data`, then virtual `+0x94`
/// ("update") runs with no extra argument.
///
/// # Original
/// - Ghidra: `FUN_8016cf50` at `0x8016CF50` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016cf50_FUN_8016cf50.c`
///
/// # Arguments
/// * `object` - receiver; its vtable is at `+0x00`.
/// * `data` - forwarded unchanged to virtual `+0x8C`.
///
/// # Returns
/// `true` when the `+0x74` handle was non-null (the two virtual calls then
/// ran); `false` otherwise.
///
/// # Side effects
/// Calls `0x802967B0`; on success calls the two virtual methods.
///
/// # Safety
/// `object` must be a live object whose `+0x74` is a valid handle and whose
/// vtable holds valid `+0x8C`/`+0x94` code pointers.
///
/// # Confidence
/// high - the guard and both call slots are exact; the slot roles are inferred
/// from the neighbouring builders [`fn_8016cb60`]/[`fn_8016d280`].
pub unsafe fn fn_8016cf50(object: usize, data: u32) -> bool {
    let ready = platform::call1(FN_802967B0, p32(object + 0x74));
    if ready != 0 {
        // inferred: Ghidra prints the virtual calls without arguments; r3 is
        // still `object` at each `bl`.
        platform::call2(vtable_method(object, 0, 0x8C), p32(object), data);
        platform::call1(vtable_method(object, 0, 0x94), p32(object));
    }
    ready != 0
}

/// Visits every node of a pane tree, calling the visitor's `+0x0C` slot.
///
/// `0x80228FF0` is called first with `(object + 0x38, 0)` and then with the
/// previous node until it returns zero. For each node the function pointer at
/// `visitor + 0x0C` is loaded fresh and called with `(node, visitor[+0x10])`.
/// The next node is obtained after the callback, exactly where the original
/// `for`-loop update expression runs.
///
/// # Original
/// - Ghidra: `FUN_8016d090` at `0x8016D090` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d090_FUN_8016d090.c`
///
/// # Arguments
/// * `object` - owner of the tree header at `+0x38`.
/// * `visitor` - two-word visitor record: code at `+0x0C`, context at `+0x10`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80228FF0` once per step and the visitor once per node.
///
/// # Safety
/// The tree must be consistent; the callback and context must be valid.
///
/// # Confidence
/// high - the iteration shape and both offsets are observed.
pub unsafe fn fn_8016d090(object: usize, visitor: usize) {
    // observed: `iVar1 = FUN_80228ff0(param_1 + 0x38,0)` then a for loop whose
    // update expression runs after the callback.
    let mut node = platform::call2(FN_80228FF0, p32(object + 0x38), 0);
    while node != 0 {
        let callback = read_u32(visitor + 0x0C) as usize;
        let context = read_u32(visitor + 0x10);
        platform::call2(callback, node, context);
        // observed: the for-loop update runs after the body, so the next-node
        // query sees the tree as the callback left it.
        node = platform::call2(FN_80228FF0, p32(object + 0x38), node);
    }
}

/// Registers [`fn_8016d090`] as the `+0x0C` pane visitor of `object`.
///
/// Guarded twice: the resource handle at `+0x14` must be valid and the word at
/// `+0x0C` must be non-zero. The handle's payload is obtained with
/// `0x802967E0` and passed to `0x802862C0(handle, fn_8016d090, object, 1)`.
///
/// # Original
/// - Ghidra: `FUN_8016d120` at `0x8016D120` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d120_FUN_8016d120.c`
///
/// # Arguments
/// * `object` - pane object owning the handle at `+0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Registers the visitor callback in the handle's owner.
///
/// # Safety
/// `+0x14` must be a valid handle; `object` must stay alive while the callback
/// is registered.
///
/// # Confidence
/// medium - call order and guards are exact; the two `0x8029....` helpers are
/// only identified by their call shape.
pub unsafe fn fn_8016d120(object: usize) {
    let present = platform::call1(FN_802967B0, p32(object + 0x14));
    if present != 0 && read_u32(object + 0x0C) != 0 {
        let handle = platform::call1(FN_802967E0, p32(object + 0x14));
        call4(FN_802862C0, handle, FN_8016D090 as u32, p32(object), 1);
    }
}

/// Attaches a prepared child object (virtual `+0x9C` variant).
///
/// This is [`fn_8016cf50`]'s sibling: instead of a resource-handle guard it
/// asks virtual `+0x9C`, then forwards `data` to virtual `+0x8C` and calls
/// virtual `+0x94`. `a2`/`a3` are unused by the body.
///
/// # Original
/// - Ghidra: `FUN_8016d330` at `0x8016D330` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d330_FUN_8016d330.c`
///
/// # Arguments
/// * `object` - receiver with a vtable at `+0x00`.
/// * `_a2`, `_a3` - present in the original register set, unused.
/// * `data` - forwarded unchanged to virtual `+0x8C`.
///
/// # Returns
/// `true` when virtual `+0x9C` returned non-zero.
///
/// # Side effects
/// Calls virtual `+0x9C`, and on success `+0x8C` then `+0x94`.
///
/// # Safety
/// `object` must have a valid vtable.
///
/// # Confidence
/// high - identical in shape to the already-promoted [`fn_8016cb60`].
pub unsafe fn fn_8016d330(object: usize, _a2: u32, _a3: u32, data: u32) -> bool {
    let can_build = platform::call1(vtable_method(object, 0, 0x9C), p32(object));
    if can_build != 0 {
        platform::call2(vtable_method(object, 0, 0x8C), p32(object), data);
        platform::call1(vtable_method(object, 0, 0x94), p32(object));
    }
    can_build != 0
}

/// Visits every node of a pane tree, calling the visitor's `+0x120` slot.
///
/// Same traversal as [`fn_8016d090`], but the callback pair is read from
/// `visitor + 0x120` (code) and `visitor + 0x124` (context).
///
/// # Original
/// - Ghidra: `FUN_8016d3c0` at `0x8016D3C0` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d3c0_FUN_8016d3c0.c`
///
/// # Arguments
/// * `object` - owner of the tree header at `+0x38`.
/// * `visitor` - callback record at `+0x120`/`+0x124`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80228FF0` once per step and the visitor once per node.
///
/// # Safety
/// The tree must be consistent; callback and context must be valid.
///
/// # Confidence
/// high - the traversal and both offsets are observed.
pub unsafe fn fn_8016d3c0(object: usize, visitor: usize) {
    let mut node = platform::call2(FN_80228FF0, p32(object + 0x38), 0);
    while node != 0 {
        let callback = read_u32(visitor + 0x120) as usize;
        let context = read_u32(visitor + 0x124);
        platform::call2(callback, node, context);
        // observed: for-loop update after the callback (same shape as
        // [`fn_8016d090`]).
        node = platform::call2(FN_80228FF0, p32(object + 0x38), node);
    }
}

/// Registers [`fn_8016d3c0`] as the `+0x120` pane visitor, then tears down.
///
/// The guard and registration mirror [`fn_8016d120`] with the handle at
/// `+0x74` and the callback word at `+0x120`; afterwards `0x802D85C0` is
/// called on `object` unconditionally.
///
/// # Original
/// - Ghidra: `FUN_8016d440` at `0x8016D440` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d440_FUN_8016d440.c`
///
/// # Arguments
/// * `object` - pane object with the resource handle at `+0x74`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May register the visitor, then calls `0x802D85C0(object)`.
///
/// # Safety
/// `+0x74` must be a valid handle when the visitor word is set; `object` is
/// passed to the teardown routine afterwards.
///
/// # Confidence
/// medium - registration and teardown are exact; `0x802D85C0` is only known
/// by its call shape.
pub unsafe fn fn_8016d440(object: usize) {
    let present = platform::call1(FN_802967B0, p32(object + 0x74));
    if present != 0 && read_u32(object + 0x120) != 0 {
        let handle = platform::call1(FN_802967E0, p32(object + 0x74));
        call4(FN_802862C0, handle, FN_8016D3C0 as u32, p32(object), 1);
    }
    platform::call1(FN_802D85C0, p32(object));
}

/// Builds a transform into a scratch matrix and attaches a child object.
///
/// The owner word is stored at `object + 0x24`. `0x8016C9E0` fills a 48-byte
/// scratch transform from the three component pointers; a 48-byte scoped
/// helper object is then constructed, configured with the first component,
/// and used around the virtual `+0x8C`/`+0x94` attach pair before being
/// released with `-1`.
///
/// # Original
/// - Ghidra: `FUN_8016d8e0` at `0x8016D8E0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d8e0_FUN_8016d8e0.c`
///
/// # Arguments
/// * `object` - receiver of the virtual calls.
/// * `owner` - word stored at `object + 0x24`.
/// * `c1`, `c2`, `c3` - three component pointers for `0x8016C9E0`.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Writes `object + 0x24`, fills the scratch transform, runs the scoped helper
/// and calls virtual `+0x8C`/`+0x94`.
///
/// # Safety
/// The component pointers must satisfy `0x8016C9E0`; `object` must have a
/// valid vtable; the scoped-helper calls require a writable 48-byte block.
///
/// # Confidence
/// medium - every call and store is observed; which of `c1..c3` is position,
/// rotation or scale is `// unknown:`, and the scoped helper's class is not
/// recovered.
pub unsafe fn fn_8016d8e0(
    object: usize,
    owner: u32,
    c1: u32,
    c2: u32,
    c3: u32,
) -> u32 {
    // observed: `param_1[9] = param_2` (int* indexing -> +0x24).
    write_u32(object + 0x24, owner);

    // observed: `undefined1 auStack_38[48]`; the callee writes a 4x4 matrix.
    let mut matrix = crate::platform::GuestLocal::new( [0u32; 16]);
    // observed: `FUN_8016c9e0(param_3,param_4,param_5,auStack_38)` - the
    // scratch buffer is the transform destination, not the scale pointer.
    layout_set_transform(
        matrix.as_mut_ptr() as usize,
        c1 as usize,
        c2 as usize,
        c3 as usize,
    );

    // observed: `undefined1 auStack_68[48]` scoped helper.
    let mut guard = crate::platform::GuestLocal::new( [0u32; 12]);
    platform::call2(FN_802DB6C0, guard.as_mut_ptr() as u32, p32(object));
    platform::call2(FN_802DB750, guard.as_mut_ptr() as u32, 0);
    platform::call2(FN_802DB8C0, guard.as_mut_ptr() as u32, c1);
    platform::call2(
        vtable_method(object, 0, 0x8C),
        p32(object),
        matrix.as_mut_ptr() as u32,
    );
    platform::call1(vtable_method(object, 0, 0x94), p32(object));
    platform::call1(FN_802DB970, guard.as_mut_ptr() as u32);
    platform::call2(FN_802DB710, guard.as_mut_ptr() as u32, 0xFFFF_FFFF);
    1
}

/// Constructs the small animation-state object used by the pane timer.
///
/// The body stores the class vtable, clears the flag byte at `+0x08`, sets the
/// limit `0x14` at `+0x0A`, clears the counter at `+0x0C` and the word at
/// `+0x10`, copies three colour bytes through [`fn_8016dcf0`], sets flag bit 1,
/// and finally stores `mode` at `+0x04` when it is `0` or `1`.
///
/// # Original
/// - Ghidra: `FUN_8016dc00` at `0x8016DC00` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dc00_FUN_8016dc00.c`
///
/// # Arguments
/// * `state` - object to construct (at least 20 bytes).
/// * `colour` - pointer to three bytes copied to `+0x10..+0x12`.
/// * `mode` - `0` or `1` stored at `+0x04`; other values leave it at `-1`
///   (the `0xFFFFFFFF` written earlier in the body).
///
/// # Returns
/// `state`, mirroring the original `return param_1`.
///
/// # Side effects
/// Writes the vtable word, four scalar fields and three colour bytes.
///
/// # Safety
/// `state` must be writable for 20 bytes; `colour` must be readable for three
/// bytes.
///
/// # Confidence
/// medium - the stores are exact, but Ghidra drops both arguments of the
/// `0x8016DCF0` call, so `r3 = state` / `r4 = colour` is `// inferred:`.
pub unsafe fn fn_8016dc00(state: usize, colour: u32, mode: i32) -> usize {
    write_u32(state, ANM_STATE_VTABLE as u32);
    write_u8(state + 0x08, 0);
    write_u16(state + 0x0A, 0x14); // observed: counter limit 0x14
    write_u16(state + 0x0C, 0); // observed: counter
    write_u32(state + 0x10, 0xFFFF_FFFF);

    // inferred: `FUN_8016dcf0()` is printed without arguments; the two live
    // registers still hold this object and the colour pointer.
    fn_8016dcf0(state, colour as usize);

    write_u8(state + 0x08, platform::read_u8(state + 0x08) | 2);
    if mode == 0 {
        write_u32(state + 0x04, 0);
    } else if mode == 1 {
        write_u32(state + 0x04, 1);
    }
    state
}

/// Copies three colour bytes into an animation-state object.
///
/// Source `colour[0..3]` goes to `state + 0x10`, `+0x11`, `+0x12`; the fourth
/// byte at `+0x13` is left untouched.
///
/// # Original
/// - Ghidra: `FUN_8016dcf0` at `0x8016DCF0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dcf0_FUN_8016dcf0.c`
///
/// # Arguments
/// * `state` - constructed animation-state object.
/// * `colour` - three readable bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three bytes.
///
/// # Safety
/// `state + 0x10..+0x12` and `colour + 0..3` must be valid.
///
/// # Confidence
/// high - the three byte moves are explicit in the Ghidra body.
pub unsafe fn fn_8016dcf0(state: usize, colour: usize) {
    write_u8(state + 0x10, platform::read_u8(colour));
    write_u8(state + 0x11, platform::read_u8(colour + 1));
    write_u8(state + 0x12, platform::read_u8(colour + 2));
}

/// Starts the `0 -> 2` transition of the animation-state timer.
///
/// `countLeadingZeros(word) >> 5` is one exactly when `word == 0`. The original
/// uses that idiom as a zero test on the word at `+0x04`; when it is zero the
/// state becomes `2` and the counter at `+0x0C` is cleared.
///
/// # Original
/// - Ghidra: `FUN_8016dd20` at `0x8016DD20` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dd20_FUN_8016dd20.c`
///
/// # Arguments
/// * `state` - animation-state object.
///
/// # Returns
/// `true` when the transition ran (the word at `+0x04` was zero).
///
/// # Side effects
/// Writes `+0x04 = 2` and `+0x0C = 0` when it fires.
///
/// # Safety
/// `state` must be readable/writable through `+0x0D`.
///
/// # Confidence
/// high - `countLeadingZeros(x) >> 5` is a zero test on a 32-bit word.
pub unsafe fn fn_8016dd20(state: usize) -> bool {
    if read_u32(state + 0x04) == 0 {
        write_u32(state + 0x04, 2);
        write_u16(state + 0x0C, 0);
        true
    } else {
        false
    }
}

/// Starts the `1 -> 3` transition of the animation-state timer.
///
/// The original tests `countLeadingZeros(word - 1)`, i.e. "word == 1" for a
/// 32-bit word. On a match the state becomes `3` and the counter at `+0x0C`
/// is cleared.
///
/// # Original
/// - Ghidra: `FUN_8016dd50` at `0x8016DD50` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dd50_FUN_8016dd50.c`
///
/// # Arguments
/// * `state` - animation-state object.
///
/// # Returns
/// `true` when the transition ran (the word at `+0x04` was `1`).
///
/// # Side effects
/// Writes `+0x04 = 3` and `+0x0C = 0` when it fires.
///
/// # Safety
/// `state` must be readable/writable through `+0x0D`.
///
/// # Confidence
/// high - `countLeadingZeros(x - 1) >> 5` is a "x == 1" test.
pub unsafe fn fn_8016dd50(state: usize) -> bool {
    if read_u32(state + 0x04) == 1 {
        write_u32(state + 0x04, 3);
        write_u16(state + 0x0C, 0);
        true
    } else {
        false
    }
}

/// Advances one tick of the animation-state timer.
///
/// State `2` counts up and, when the counter has passed the limit at `+0x0A`,
/// moves to state `1` and returns flag bit 0. State `3` counts up and, when
/// `limit + 1 < counter`, moves to state `0` and returns flag bit 1. Any other
/// state returns `0` without writing.
///
/// The two comparisons differ by one, exactly as the Ghidra body shows: state
/// `2` compares the limit with the counter value *before* the increment, while
/// state `3` compares `limit + 1` with the value *after* the increment.
///
/// # Original
/// - Ghidra: `FUN_8016dd80` at `0x8016DD80` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dd80_FUN_8016dd80.c`
///
/// # Arguments
/// * `state` - animation-state object.
///
/// # Returns
/// A flag byte: `+0x08 & 1` on a `2 -> 1` transition, `(+0x08 >> 1) & 1` on a
/// `3 -> 0` transition, `0` otherwise.
///
/// # Side effects
/// Increments the `u16` counter at `+0x0C` in states `2` and `3`; may write
/// `+0x04`.
///
/// # Safety
/// `state` must be readable/writable through `+0x0D`.
///
/// # Confidence
/// high - both branches and the off-by-one asymmetry are visible verbatim.
pub unsafe fn fn_8016dd80(state: usize) -> u8 {
    let mut result = 0u8;
    let mode = read_u32(state + 0x04);
    if mode == 2 {
        let old = read_u16(state + 0x0C);
        write_u16(state + 0x0C, old.wrapping_add(1));
        // observed: comparison uses the old counter value (strict `<`).
        if (read_u16(state + 0x0A) as u32) < old as u32 {
            write_u32(state + 0x04, 1);
            result = platform::read_u8(state + 0x08) & 1;
        }
    } else if mode == 3 {
        // observed: signed short increment, but the comparison is unsigned.
        let old = read_u16(state + 0x0C) as i16;
        let next = old.wrapping_add(1) as u32;
        write_u16(state + 0x0C, next as u16);
        if (read_u16(state + 0x0A) as u32).wrapping_add(1) < next {
            write_u32(state + 0x04, 0);
            result = (platform::read_u8(state + 0x08) >> 1) & 1;
        }
    }
    result
}

/// Packs the low three flag bits into one byte.
///
/// Bit 0 becomes 1, bit 1 becomes 2 and bit 2 becomes 4. Higher bits are
/// ignored. The result is assembled with two conditional ORs, not by masking,
/// which matters only for the type of the intermediate.
///
/// # Original
/// - Ghidra: `FUN_8016e4b0` at `0x8016E4B0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e4b0_FUN_8016e4b0.c`
///
/// # Arguments
/// * `flags` - bit mask.
///
/// # Returns
/// `0..7`, one bit per input bit 0..2.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure arithmetic; safe to call.
///
/// # Confidence
/// high - three explicit bit tests are observed.
pub unsafe fn fn_8016e4b0(flags: u32) -> u8 {
    let mut out = u8::from(flags & 1 != 0);
    if flags & 2 != 0 {
        out |= 2;
    }
    if flags & 4 != 0 {
        out |= 4;
    }
    out
}

/// Aligns a value upward with a `0x88`-byte header, using the original formula.
///
/// The body is `base + (~(align - 1) & (align + 0x87))`, i.e. the classic
/// `RoundUp(size + header, align)` expression. It is written out rather than
/// replaced by a division so the wrapping behaviour stays identical.
///
/// # Original
/// - Ghidra: `FUN_8016e610` at `0x8016E610` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e610_FUN_8016e610.c`
///
/// # Arguments
/// * `base` - value to advance.
/// * `align` - power-of-two alignment.
///
/// # Returns
/// `base + ((align + 0x87) & ~(align - 1))`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure arithmetic; safe to call.
///
/// # Confidence
/// high - single expression in the Ghidra body.
pub unsafe fn fn_8016e610(base: i32, align: i32) -> i32 {
    base.wrapping_add(!align.wrapping_sub(1) & align.wrapping_add(0x87))
}

/// `0x802B9430`: allocates/constructs a resource (`(a, b, 4)`).
///
/// unknown: the allocator's exact class and ownership rules are not resolved.
const FN_802B9430: usize = 0x802B_9430;

/// `0x802B9070`: releases the raw buffer argument of [`fn_8016e8f0`].
///
/// unknown: differs from `0x802B9280` only by the extra argument in the
/// original call; the body is outside this lane.
const FN_802B9070: usize = 0x802B_9070;

/// `0x802B9650`: constructs a freshly allocated resource (`(object, 0)`).
const FN_802B9650: usize = 0x802B_9650;

/// Reports how much of an animation resource remains after a `0x80` lead-in.
///
/// The virtual `+0x2C` supplies the total length. `0x8016E7D0(0, 4)` computes
/// the fixed `0x80`-unit offset (`(4 + 0x7F) & ~3`); if the offset is within
/// the total, the difference is returned, otherwise `0`.
///
/// # Original
/// - Ghidra: `FUN_8016e760` at `0x8016E760` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e760_FUN_8016e760.c`
///
/// # Arguments
/// * `object` - animation resource object, may be null.
///
/// # Returns
/// `total - 0x80` when `total >= 0x80`; `0` for a null object or a shorter
/// resource.
///
/// # Side effects
/// Calls virtual `+0x2C` on the object.
///
/// # Safety
/// A non-null `object` must have a valid vtable.
///
/// # Confidence
/// medium - call shape and arithmetic are exact; the meaning of the `0x80`
/// offset is `// inferred:` from its use as a length lead-in.
pub unsafe fn fn_8016e760(object: usize) -> u32 {
    let mut remaining = 0u32;
    if object != 0 {
        // inferred: Ghidra prints the virtual call with no arguments; the
        // receiver is still in r3.
        let total = platform::call1(vtable_method(object, 0, 0x2C), p32(object));
        let lead_in = fn_8016e7d0(0, 4) as u32;
        if lead_in <= total {
            remaining = total - lead_in;
        }
    }
    remaining
}

/// Aligns a value upward with a `0x80`-byte header, using the original formula.
///
/// Sibling of [`fn_8016e610`] with `0x7F` instead of `0x87`; the call
/// `fn_8016e7d0(0, 4)` therefore returns `0x80`.
///
/// # Original
/// - Ghidra: `FUN_8016e7d0` at `0x8016E7D0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e7d0_FUN_8016e7d0.c`
///
/// # Arguments
/// * `base` - value to advance.
/// * `align` - power-of-two alignment.
///
/// # Returns
/// `base + ((align + 0x7F) & ~(align - 1))`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure arithmetic; safe to call.
///
/// # Confidence
/// high - single expression in the Ghidra body.
pub unsafe fn fn_8016e7d0(base: i32, align: i32) -> i32 {
    base.wrapping_add(!align.wrapping_sub(1) & align.wrapping_add(0x7F))
}

/// Allocates and constructs an animation resource with optional user data.
///
/// `0x802B9430(a, b, 4)` returns the storage. A zero result makes the function
/// release `b` through `0x802B9070`; otherwise `0x802B9650(storage, 0)` runs
/// and a non-zero `user` word is stored at `storage + 0x34`.
///
/// # Original
/// - Ghidra: `FUN_8016e8f0` at `0x8016E8F0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e8f0_FUN_8016e8f0.c`
///
/// # Arguments
/// * `param_1`, `param_2` - forwarded to the allocator.
/// * `user` - optional word stored at `+0x34` after construction.
///
/// # Returns
/// The new resource, or `0` when allocation failed.
///
/// # Side effects
/// May release `param_2`; writes `+0x34` when `user != 0`.
///
/// # Safety
/// The allocator must accept the two arguments; the returned block must be at
/// least `0x38` bytes when `user != 0`.
///
/// # Confidence
/// medium - all three calls and the store are exact; the helper roles are
/// inferred from their guards and the `+0x34` user word.
pub unsafe fn fn_8016e8f0(param_1: u32, param_2: u32, user: i32) -> u32 {
    let object = platform::call3(FN_802B9430, param_1, param_2, 4);
    if object == 0 {
        platform::call1(FN_802B9070, param_2);
    } else {
        platform::call2(FN_802B9650, object, 0);
        if user != 0 {
            write_u32(object as usize + 0x34, user as u32);
        }
    }
    object
}

// ===========================================================================
// Wave 4: `RLAN` transform apply and `RLMC` material colour
// (0x801822F0..0x801823E0) - completion of the cluster started in wave 2
// ===========================================================================
//
// The three bodies below operate on the same animation work buffer as
// [`fn_80182060`]: `object + 0x58` is the 0x20-byte-record array and
// `object + 0x4C`/`+0x50` are the packed counts. `0x801C7230`/`0x801C7240`
// read and `0x801C7100`/`0x801C7160` write the record's transform words.

/// `0x801C7230`: reads the first transform word of a 0x20-byte record.
///
/// unknown: the matrix component is not named; it is only saved and restored
/// around [`fn_8017f250`].
const FN_801C7230: usize = 0x801C_7230;

/// `0x801C7240`: reads the second transform word of a 0x20-byte record.
const FN_801C7240: usize = 0x801C_7240;

/// `0x801C7100`: writes both transform words back to a record.
const FN_801C7100: usize = 0x801C_7100;

/// `0x801C7160`: rebuilds a record's cached transform (`(record, 0)`).
const FN_801C7160: usize = 0x801C_7160;

/// `0x8017F250`: applies a record to the live transform (`(record, data, 0)`).
const FN_8017F250: usize = 0x8017_F250;

/// Applies one `0x20`-byte animation record to the live transform.
///
/// The record is `work_buffer + (index & 0xFF) * 0x20`. Two words of the
/// record are saved, the record is applied through `0x8017F250`, and the two
/// saved words are put back through `0x801C7100`. This ordering means the
/// apply step cannot permanently change those two words.
///
/// # Original
/// - Ghidra: `FUN_801822f0` at `0x801822F0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801822f0_FUN_801822f0.c`
///
/// # Arguments
/// * `object` - animation-transform object (`+0x58` work buffer).
/// * `index` - record index; only the low byte is used.
/// * `data` - forwarded to `0x8017F250`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls the four `0x801C....` helpers; `0x8017F250` writes the live
/// transform.
///
/// # Safety
/// `object + 0x58` must point to at least `(index & 0xFF + 1) * 0x20` bytes.
///
/// # Confidence
/// high - the save/apply/restore sequence is exact.
pub unsafe fn fn_801822f0(object: usize, index: u32, data: u32) {
    let record = read_u32(object + ANM_WORK_BUFFER) as usize
        + ((index & 0xFF) * 0x20) as usize;

    let saved_a = platform::call1(FN_801C7230, p32(record));
    let saved_b = platform::call1(FN_801C7240, p32(record));
    platform::call3(FN_8017F250, p32(record), data, 0);
    platform::call3(FN_801C7100, p32(record), saved_a, saved_b);
}

/// Copies eight words into a `0x20`-byte record and refreshes its transform.
///
/// Source is an eight-word descriptor (32 bytes); destination is
/// `work_buffer + (index & 0xFF) * 0x20`. After the copy, `0x801C7160` is
/// called with `(record, 0)`.
///
/// # Original
/// - Ghidra: `FUN_80182380` at `0x80182380` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80182380_FUN_80182380.c`
///
/// # Arguments
/// * `object` - animation-transform object (`+0x58` work buffer).
/// * `index` - record index; only the low byte is used.
/// * `source` - eight readable words.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 32 bytes and calls `0x801C7160`.
///
/// # Safety
/// The destination record and the 32-byte source must be valid.
///
/// # Confidence
/// high - the unrolled copy and the single call are exact.
pub unsafe fn fn_80182380(object: usize, index: u32, source: usize) {
    let record = read_u32(object + ANM_WORK_BUFFER) as usize
        + ((index & 0xFF) * 0x20) as usize;

    // observed: eight `lwz`/`stw` pairs; read all first so an overlapping
    // source/destination pair behaves like the original's temporaries.
    let mut words = [0u32; 8];
    for (i, word) in words.iter_mut().enumerate() {
        *word = read_u32(source + i * 4);
    }
    for (i, word) in words.iter().enumerate() {
        write_u32(record + i * 4, *word);
    }

    platform::call2(FN_801C7160, p32(record), 0);
}

/// Writes one material-colour byte (the `RLMC` animation path).
///
/// The selector chooses both a section and a byte inside it:
///
/// | selector | destination |
/// |---|---|
/// | `0..3` | packed indirect-material base (`+0x58` + packed `+0x4C` terms), byte `selector` |
/// | `4..15` | `+0x24 + ((selector - 4) >> 2) * 8 + ((selector - 4) & 3) * 2`, a `u16` |
/// | `16..31` | `+0x3C + ((selector - 16) >> 2) * 4 + ((selector - 16) & 3)`, a `u8` |
/// | `>= 32` | no write |
///
/// For the byte paths the value is clamped exactly like the original: a
/// negative value becomes `0`, and the low 16 bits are compared as a *signed*
/// short against `0x100`; only `0.255` survives, everything else becomes
/// `0xFF`. The `u16` path truncates without clamping.
///
/// The `0..3` path is guarded by bit 7 of `+0x50`; when the bit is clear the
/// function returns without writing. Caller `0x8017DCC0` reaches this function
/// for `RLMC` (`0x524C4D43`); the individual material/texture index meanings
/// are `// inferred:` from that dispatch.
///
/// # Original
/// - Ghidra: `FUN_801823e0` at `0x801823E0` (436 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801823e0_FUN_801823e0.c`
///
/// # Arguments
/// * `object` - animation-transform object.
/// * `selector` - section/byte selector described above.
/// * `value` - colour value, clamped or truncated per path.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one byte or one halfword, depending on the selector.
///
/// # Safety
/// `object` must be readable through `+0x58` and the computed destination must
/// be writable.
///
/// # Confidence
/// high for the branches and offsets; medium for the material-colour meaning
/// (`// inferred:` from the `RLMC` dispatch at `0x8017DCC0`).
pub unsafe fn fn_801823e0(object: usize, selector: u32, value: u32) {
    let from_sixteen = selector.wrapping_sub(0x10);
    if from_sixteen < 0x10 {
        // observed: per-channel byte at +0x3C + channel * 4 + sub.
        let index = (from_sixteen >> 2) as usize;
        let sub = (from_sixteen & 3) as usize;
        let clamped = clamp_colour_byte(value);
        write_u8(object + index * 4 + 0x3C + sub, clamped);
        return;
    }

    let from_four = selector.wrapping_sub(4);
    if 0xB < from_four {
        // observed: only selectors 0..3 survive `3 < param_2` here (larger
        // selectors returned above or fall through to the no-write return).
        if 3 < selector {
            return;
        }
        if (read_u32(object + 0x50) >> 7) & 1 == 0 {
            return;
        }
        let packed = read_u32(object + 0x4C);
        // observed: (packed>>6 & 4) + (packed>>0x18 & 0xF)*0x14
        //         + work_buffer + (packed>>0x12 & 0x3C) + (packed>>0x17 & 0x1E0)
        let record = read_u32(object + ANM_WORK_BUFFER) as usize
            + ((packed >> 6) & 4) as usize
            + (((packed >> 0x18) & 0xF) * 0x14) as usize
            + ((packed >> 0x12) & 0x3C) as usize
            + ((packed >> 0x17) & 0x1E0) as usize;
        let clamped = clamp_colour_byte(value);
        write_u8(record + (selector & 3) as usize, clamped);
        return;
    }

    // observed: u16 at +0x24 + index * 8 + sub * 2, no clamping.
    let index = (from_four >> 2) as usize;
    let sub = (from_four & 3) as usize;
    write_u16(object + index * 8 + 0x24 + sub * 2, value as u16);
}

/// Applies the original clamp used by the byte paths of [`fn_801823e0`].
///
/// `value` is first clamped to non-negative with `value & ~(value >> 31)`,
/// then compared as a signed `s16`: below `0x100` the low byte is used,
/// otherwise `0xFF`. This reproduces both the negative clamp and the signed
/// comparison without changing the odd corner where a `0x8000..0xFFFF` value
/// still takes the low byte.
///
/// # Original
/// Host-side helper; no binary address. Inlined at two sites of
/// `FUN_801823e0` (`work/ghidra/main-gb/export/801823e0_FUN_801823e0.c`).
///
/// # Safety
/// Pure arithmetic; safe to call.
///
/// # Confidence
/// high - mirrors the two identical Ghidra blocks.
#[inline]
fn clamp_colour_byte(value: u32) -> u8 {
    let non_negative = value & !((value as i32 >> 31) as u32);
    if (non_negative as u16 as i16) < 0x100 {
        non_negative as u8
    } else {
        0xFF
    }
}

// ===========================================================================
// Wave 4: pane subtree notify and list teardown
// (0x80183910..0x8018AA80)
// ===========================================================================

/// Inserts a descriptor into an object's embedded sentinel list.
///
/// The list header is the word at `owner + 0x10` (count) plus the sentinel
/// node embedded at `owner + 0x14`. [`fn_8018a590`] is called with a
/// temporary head word pointing at `owner + 0x14`, and the descriptor's link
/// is its first two words after the four-byte header (`descriptor + 4`).
/// Finally the descriptor's `+0x0C` word is set to `owner`.
///
/// # Original
/// - Ghidra: `FUN_80183910` at `0x80183910` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80183910_FUN_80183910.c`
///
/// # Arguments
/// * `owner` - object with the embedded count/sentinel list.
/// * `descriptor` - node to insert; link at `+4`, back-pointer at `+0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the new node's two links, the sentinel's newer link, the previous
/// first node's older link, the count, and `descriptor + 0x0C`.
///
/// # Safety
/// `owner + 0x10..+0x1C` and `descriptor + 0..+0x10` must be writable.
///
/// # Confidence
/// high - the wrapper and its single call are exact.
pub unsafe fn fn_80183910(owner: usize, descriptor: usize) {
    // observed: `local_18[0] = param_1 + 0x14`; the word is the sentinel
    // address, not the sentinel's own link.
    let head_word = crate::platform::GuestLocal::<u32>::new( (owner + 0x14) as u32);
    fn_8018a590(
        (owner + 0x10) as *mut u32,
        head_word.as_ptr() as *const u32,
        (descriptor + 4) as *mut u32,
    );
    write_u32(descriptor + 0x0C, p32(owner));
}

/// Notifies a pane subtree with one argument.
///
/// The function is a no-op unless bit 0 of `+0xCF` is set. It then calls the
/// virtual `+0x18` slot on the pane itself and walks the embedded sentinel
/// list whose link starts at `+0x14`: each node's vtable is the word four
/// bytes before its link, and its virtual `+0x14` slot receives `(node, arg)`.
///
/// # Original
/// - Ghidra: `FUN_80183f40` at `0x80183F40` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80183f40_FUN_80183f40.c`
///
/// # Arguments
/// * `pane` - root pane; `+0xCF` bit 0 is the "active" flag.
/// * `arg` - forwarded to every node's virtual `+0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls up to `1 + node_count` virtual methods.
///
/// # Safety
/// `pane` must be live; every node in the list must start with a valid vtable
/// and the sentinel at `pane + 0x14` must terminate the list.
///
/// # Confidence
/// medium - the flag, list walk and offsets are exact; the virtual `+0x18`
/// call is printed without arguments, so passing `pane` is `// inferred:`.
pub unsafe fn fn_80183f40(pane: usize, arg: u32) {
    if platform::read_u8(pane + 0xCF) & 1 == 0 {
        return;
    }

    platform::call1(vtable_method(pane, 0, 0x18), p32(pane));

    // observed: `piVar1 = param_1[5]`, sentinel `param_1 + 5` (byte +0x14).
    let sentinel = pane + 0x14;
    let mut link = read_u32(pane + 0x14) as usize;
    while link != sentinel {
        let node = link - 4; // observed: `piVar1 + -1` is the object base.
        platform::call2(vtable_method(node, 0, 0x14), p32(node), arg);
        // observed: for-loop update after the callback.
        link = read_u32(link) as usize;
    }
}

/// Unlinks every node of an embedded sentinel list and optionally frees it.
///
/// The header is `{ u32 count, sentinel node at +4 }`; the sentinel's two link
/// words are at `+4`/`+8` and the walk starts from the word at `+4`. Each node
/// is spliced out in both directions, the count decremented, and both of the
/// node's link words cleared. After the walk, a positive `delete_flag` releases
/// the whole header through `0x802B9280`.
///
/// # Original
/// - Ghidra: `FUN_8018a4b0` at `0x8018A4B0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a4b0_FUN_8018a4b0.c`
///
/// # Arguments
/// * `list` - header, may be null.
/// * `delete_flag` - `> 0` releases the header after unlinking.
///
/// # Returns
/// `list`, unchanged.
///
/// # Side effects
/// Repairs up to two neighbour links per node, clears each node's links,
/// decrements the count, and may free the header.
///
/// # Safety
/// `list` must be a valid header whose nodes still carry their links; the
/// header must have been allocated if `delete_flag > 0`.
///
/// # Confidence
/// high - the splice order and the conditional free are exact.
pub unsafe fn fn_8018a4b0(list: *mut u8, delete_flag: i32) -> *mut u8 {
    let base = list as usize;
    if base != 0 {
        // observed: sentinel node at +4; loop until the sentinel is reached.
        let sentinel = base + 4;
        let mut node = read_u32(base + 4) as usize;
        while node != sentinel {
            let next = read_u32(node) as usize;
            let older = read_u32(node + 4) as usize;
            write_u32(next + 4, older as u32);
            write_u32(older, next as u32);
            write_u32(base, read_u32(base).wrapping_sub(1));
            write_u32(node, 0);
            write_u32(node + 4, 0);
            node = next;
        }
        if delete_flag > 0 {
            // observed: Ghidra drops the argument; the header is in r3.
            platform::call1(FN_802B9280, p32(base));
        }
    }
    list
}

/// Detaches every node in front of a circular list's sentinel.
///
/// `head_word` holds the first node. The sentinel is the node whose first
/// link points at itself (`*node == node`). Every node before it is spliced
/// out in both directions, both links are cleared, and the count at `count` is
/// decremented. The sentinel is returned.
///
/// # Original
/// - Ghidra: `FUN_8018a540` at `0x8018A540` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018a540_FUN_8018a540.c`
///
/// # Arguments
/// * `count` - list element counter.
/// * `head_word` - pointer to the first node pointer.
///
/// # Returns
/// The sentinel node.
///
/// # Side effects
/// Rewrites two neighbour links per node, clears each node's links and
/// decrements `*count`.
///
/// # Safety
/// `*head_word` must be a valid node; the list must be circular and terminated
/// by a self-linked sentinel.
///
/// # Confidence
/// high - loop condition and link updates are exact.
pub unsafe fn fn_8018a540(count: *mut u32, head_word: *const u32) -> *mut u8 {
    let first = read_u32(head_word as usize) as usize;
    // observed: `puVar2 = *(undefined4 **)*param_2` - the sentinel is reached
    // when the node's first link points back at the node.
    let sentinel = read_u32(first) as usize;
    let mut node = first;
    while node != sentinel {
        let next = read_u32(node) as usize;
        let older = read_u32(node + 4) as usize;
        write_u32(next + 4, older as u32);
        write_u32(older, next as u32);
        let count_addr = count as usize;
        write_u32(count_addr, read_u32(count_addr).wrapping_sub(1));
        write_u32(node, 0);
        write_u32(node + 4, 0);
        node = next;
    }
    sentinel as *mut u8
}

/// Releases an object when both a non-null pointer and a positive flag hold.
///
/// Identical wrapper to `0x8016C6F0` / `0x8016DCA0`: the pointer is returned
/// unchanged whether or not the release ran.
///
/// # Original
/// - Ghidra: `FUN_8018aa80` at `0x8018AA80` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8018aa80_FUN_8018aa80.c`
///
/// # Arguments
/// * `object` - object to release, may be null.
/// * `delete_flag` - `> 0` releases the object.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Calls `0x802B9280(object)` when both conditions hold.
///
/// # Safety
/// A non-null `object` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - the guard is exact; the dropped argument is restored from r3, the
/// same convention as the sibling wrappers.
pub unsafe fn fn_8018aa80(object: *mut u8, delete_flag: i32) -> *mut u8 {
    if !object.is_null() && delete_flag > 0 {
        platform::call1(FN_802B9280, p32(object as usize));
    }
    object
}

// ===========================================================================
// Wave 5 - `DrawInfo`, `Pane`, picture texture math and `ut` fonts
// (0x8017F250..0x8018ABE0)
// ===========================================================================
//
// The vtable words below are the class tables named by their RTTI strings in
// `main.elf`:
//
// | vtable | RTTI string | class |
// |---|---|---|
// | `0x8032B1E0` | `nw4hbm::lyt::DrawInfo` | draw info |
// | `0x8032B210` | `nw4hbm::lyt::Group` | draw-info group |
// | `0x8032B238` | `nw4hbm::lyt::Layout` | layout |
// | `0x8032B310` | `nw4hbm::lyt::Pane` | pane base |
// | `0x8032B3E4` | `nw4hbm::lyt::Picture` | picture pane (via `0x8032B378`) |
// | `0x8032C710` | `nw4hbm::ut::ResFont` | resource font |
// | `0x8032C800` | `nw4hbm::ut::Font` | font base |
//
// Every vtable starts with an eight-byte header (`RTTI pointer`, `0`), so the
// first method lives at `+0x08`; that is why every virtual call below uses a
// slot offset of at least `0x08`.

// ---------------------------------------------------------------------------
// Wave 5 callees (called by address; definitions live outside this lane)
// ---------------------------------------------------------------------------

/// `0x80004364`: `memcpy(dst, src, len)` (`crate::boot::mem::memcpy`).
///
/// # Original
/// Callee of [`fn_8017f450`] and [`fn_80185e00`]; translated in
/// `src/boot/mem.rs`.
///
/// # Confidence
/// high - the same address is already documented as `memcpy` in the ledger.
const FN_80004364_MEMCPY: usize = 0x8000_4364;

/// `0x802E19F8`: `strncpy`-shaped copy: bytes up to the first `NUL`, then pad.
///
/// # Original
/// Callee of [`fn_801836b0`]; Ghidra shows a byte loop that pads the rest of
/// `param_3` with `NUL` after the terminator (`802e19f8_FUN_802e19f8.c`).
///
/// # Confidence
/// high - loop shape is explicit.
const FN_802E19F8_STRNCPY: usize = 0x802E_19F8;

/// `0x801D5540`: heap allocate `(heap, size)`.
///
/// # Original
/// Callee of [`fn_8017f450`], [`fn_80185cc0`] and [`fn_80185e00`]; `heap` is
/// the word stored at [`HEAP_80378008`]. The Ghidra body is a virtual tail
/// call through the heap's first word (`801d5540_FUN_801d5540.c`).
///
/// # Confidence
/// high - a word-sized result is stored as the new buffer pointer.
const FN_801D5540_ALLOC: usize = 0x801D_5540;

/// `0x801D5550`: heap free `(heap, pointer)`.
///
/// # Original
/// Callee of [`fn_8017f610`], [`fn_8017f980`], [`fn_801837d0`],
/// [`fn_80185cc0`] and [`fn_80185e00`].
///
/// # Confidence
/// high - paired with [`FN_801D5540_ALLOC`] at every site.
const FN_801D5550_FREE: usize = 0x801D_5550;

/// `0x80378008`: word holding the process-wide graphics heap object.
///
/// # Original
/// Ghidra prints `DAT_80378008` as the first argument of every
/// `0x801D5540`/`0x801D5550` call. Raw instructions at `0x8017F4E8` are
/// `lis r31,0x8038; lwz r3,-0x7FF8(r31)`, i.e. the *value* is passed, not its
/// address.
///
/// # Confidence
/// high - instruction sequence.
const HEAP_80378008: usize = 0x8037_8008;

/// `0x8017DFA0`: find the first circular-list node whose `+0x08` word equals
/// the key; returns `0` at the sentinel.
///
/// # Original
/// Callee of [`fn_80184350`]. Translated as `menu_find_list_node` in
/// `src/ui/menu.rs`; the call is kept by address so this module has no
/// dependency on that lane.
///
/// # Confidence
/// high - `8017dfa0_FUN_8017dfa0.c` and the ledger row agree.
const FN_8017DFA0_FIND: usize = 0x8017_DFA0;

/// `0x801C6D90`: initialise a GX texture object (no palette).
///
/// # Original
/// Callee of [`fn_8017f250`] in the no-palette branch. Eight arguments:
/// `(texObj, image, width, height, format, wrapS, wrapT, mipmap)`, read from
/// the argument order in `801c6d90_FUN_801c6d90.c`.
///
/// # Confidence
/// medium - the body writes the GX tex-object words `0x10/0x90..0xD0`; the
/// individual argument names come from the register order, not a symbol.
const FN_801C6D90_TEX_INIT: usize = 0x801C_6D90;

/// `0x801C6FA0`: initialise a GX texture object with a palette.
///
/// # Original
/// Callee of [`fn_8017f250`] in the palette branch; forwards the same eight
/// arguments and adds the palette operand as the ninth
/// (`801c6fa0_FUN_801c6fa0.c`).
///
/// # Confidence
/// medium - same reasoning as [`FN_801C6D90_TEX_INIT`].
const FN_801C6FA0_TEX_INIT_CI: usize = 0x801C_6FA0;

/// `0x801C6FF0`: set texture LOD/filter fields.
///
/// # Original
/// Callee of [`fn_8017f250`]; three `f64` filter/level arguments followed by
/// `(texObj, minFilter, magFilter, biasClamp, edgeLod, maxAniso)`.
///
/// # Confidence
/// medium - argument order from `801c6ff0_FUN_801c6ff0.c`.
const FN_801C6FF0_TEX_LOD: usize = 0x801C_6FF0;

/// `0x801C7160`: store the palette pointer at `texObj + 0x10`.
///
/// # Original
/// Callee of [`fn_8017f250`] (`801c7160_FUN_801c7160.c`).
///
/// # Confidence
/// high - single store, explicit.
const FN_801C7160_TEX_PALETTE: usize = 0x801C_7160;

/// `0x802281D0`: convert a texture-pattern resource's relative offsets to
/// absolute pointers (lazy, guarded by the high bit of `+0x08`).
///
/// # Original
/// Callee of [`fn_8017f250`] (`802281d0_FUN_802281d0.c`).
///
/// # Confidence
/// high - the body adds its own base to every non-zero stored offset.
const FN_802281D0_PATTERN_FIXUP: usize = 0x8022_81D0;

/// `0x802282F0`: index one 8-byte descriptor out of the texture-pattern array.
///
/// # Original
/// Callee of [`fn_8017f250`]; returns
/// `entries + (index % count) * 8` (`802282f0_FUN_802282f0.c`).
///
/// # Confidence
/// high - explicit modulo expression.
const FN_802282F0_PATTERN_ENTRY: usize = 0x8022_82F0;

/// `0x8018AEA0`: map a character code inside one glyph-range node.
///
/// # Original
/// Callee of [`fn_8018abe0`]; not translated in this pass.
///
/// # Confidence
/// medium - the range/list shape is visible, but its three modes are not
/// reconstructed yet; it stays an unknown call by address.
const FN_8018AEA0_GLYPH_RANGE: usize = 0x8018_AEA0;

/// `0x8018AAC0`: store the font file base and the `FINF` payload pointer.
///
/// # Original
/// Callee of [`fn_8018a7d0`]; the body is two stores to `+0x10`/`+0x14`
/// (`8018aac0_FUN_8018aac0.c`).
///
/// # Confidence
/// high - `r4`/`r5` are still the file and section pointers at the `bl`
/// (`0x8018A8D0`..`0x8018A8D8`).
const FN_8018AAC0_FONT_INFO: usize = 0x8018_AAC0;

// --- vtable words -----------------------------------------------------------

/// `nw4hbm::lyt::DrawInfo` vtable (`RTTI 0x8032B1EC`).
const VT_DRAW_INFO: u32 = 0x8032_B1E0;
/// `nw4hbm::lyt::Group` vtable (`RTTI 0x8032B21C`).
const VT_GROUP: u32 = 0x8032_B210;
/// `nw4hbm::lyt::Layout` vtable (`RTTI 0x8032B26C`).
const VT_LAYOUT: u32 = 0x8032_B238;
/// `nw4hbm::lyt::Pane` vtable (`RTTI 0x8032B190`).
const VT_PANE: u32 = 0x8032_B310;
/// `nw4hbm::ut::ResFont` vtable (`RTTI 0x8032C768`).
const VT_RES_FONT: u32 = 0x8032_C710;
/// `nw4hbm::ut::Font` vtable (`RTTI 0x8032C7D0`).
const VT_FONT: u32 = 0x8032_C800;
/// Intermediate base vtable written by the `ut::Font` constructor before the
/// derived `nw4hbm::ut::Font` table; its destructor slot is `0x80185ED0`.
const VT_FONT_BASE: u32 = 0x8032_C858;

// --- small operand types ----------------------------------------------------

/// `0x52464E55`: `"RFNU"`, the NW4R file header tag inside a `BRFNT`.
const TAG_RFNU: u32 = 0x5246_4E55;
/// `0x46494E46`: `"FINF"`, the font-info section tag inside a `BRFNT`.
const TAG_FINF: u32 = 0x4649_4E46;
/// `0x52464E54`: `"RFNT"`, the `BRFNT` section tag accepted by
/// [`fn_80188860`] and [`fn_8018a7d0`].
const TAG_RFNT: u32 = 0x5246_4E54;
/// `0x54474C50`: `"TGLP"`, the texture-glyph section tag.
const TAG_TGLP: u32 = 0x5447_4C50;
/// `0x43574448`: `"CWDH"`, the character-width section tag.
const TAG_CWDH: u32 = 0x4357_4448;
/// `0x434D4150`: `"CMAP"`, the character-map section tag.
const TAG_CMAP: u32 = 0x434D_4150;
/// `0x474C4752`: `"GLGR"`, the glyph-group section tag.
const TAG_GLGR: u32 = 0x474C_4752;

// --- wider call shapes ------------------------------------------------------

/// Calls an eight-argument integer function.
///
/// # Original
/// Host-side helper; no binary address. Models the PowerPC `bl` with
/// arguments in `r3..r10`, used by the GX texture-object initialisers.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call8_u32(
    addr: usize,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32);
    f(a0, a1, a2, a3, a4, a5, a6, a7)
}

/// Calls a nine-argument integer function.
///
/// # Original
/// Host-side helper; no binary address. The ninth argument is passed on the
/// stack, matching `0x801C6FA0`'s `in_stack_00000008` operand.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call9_u32(
    addr: usize,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32);
    f(a0, a1, a2, a3, a4, a5, a6, a7, a8)
}

/// Calls `addr(f64, f64, f64, u32, u32, u32, u32, u32, u32)`.
///
/// # Original
/// Host-side helper; no binary address. Models the `0x801C6FF0` LOD call:
/// three doubles in `f1..f3`, then six integers in `r3..r8`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call_f64x3_u32x6(
    addr: usize,
    f0: f64,
    f1: f64,
    f2: f64,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f64, f64, f64, u32, u32, u32, u32, u32, u32) -> u32);
    f(f0, f1, f2, a0, a1, a2, a3, a4, a5)
}

// ---------------------------------------------------------------------------
// `nw4hbm::lyt::DrawInfo` - construction, texture bind and layout teardown
// ---------------------------------------------------------------------------

/// Binds one texture-pattern record into a GX texture object.
///
/// The texture-pattern resource is lazily fixed up (`0x802281D0`) when the
/// high bit of its `+0x08` word is clear, then the record selected by
/// `frame % count` is read. The record supplies an image pointer, width,
/// height, format, two wrap modes, two filter bytes and a palette pointer.
/// With a palette the colour-indexed initialiser `0x801C6FA0` runs and the
/// palette pointer is stored at `+0x10`; without one the plain initialiser
/// `0x801C6D90` runs. Finally `0x801C6FF0` programs the LOD fields from the
/// record's `f32` at `+0x1C` and bytes at `+0x20..+0x22`.
///
/// # Original
/// - Ghidra: `FUN_8017f250` at `0x8017F250` (320 bytes)
/// - C file: `work/ghidra/main-gb/export/8017f250_FUN_8017f250.c`
///
/// # Arguments
/// * `texture_obj` - 36-byte GX texture object to (re)initialise.
/// * `pattern` - texture-pattern resource; its `+0x08` word is the entry
///   array offset and `+0x04` the entry count.
/// * `frame` - animation frame; reduced modulo the entry count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May convert the resource's relative offsets in place, then writes the
/// texture object (`+0x00..+0x24`) through the three GX helpers.
///
/// # Safety
/// `texture_obj` must be writable for 36 bytes; `pattern` must be a valid
/// texture-pattern resource whose entry array is readable.
///
/// # Confidence
/// medium - control flow and every operand are exact; the GX helper names and
/// the record field names are inferred from the instruction order, not from
/// symbols.
pub unsafe fn fn_8017f250(texture_obj: usize, pattern: usize, frame: u32) {
    if read_u32(pattern + 0x08) < 0x8000_0000 {
        // observed: first use converts the resource's relative offsets.
        platform::call1(FN_802281D0_PATTERN_FIXUP, p32(pattern));
    }
    let entry = platform::call2(FN_802282F0_PATTERN_ENTRY, p32(pattern), frame) as usize;
    let record = read_u32(entry) as usize;

    // observed: ((a - b) | (b - a)) >> 31 is the original `a != b` test.
    let mipmap = (read_u8(record + 0x22) != read_u8(record + 0x21)) as u32;

    let palette = read_u32(entry + 4);
    if palette == 0 {
        let _ = call8_u32(
            FN_801C6D90_TEX_INIT,
            p32(texture_obj),
            read_u32(record + 0x08),
            read_u16(record + 0x02) as u32,
            read_u16(record) as u32,
            read_u32(record + 0x04),
            read_u32(record + 0x0C),
            read_u32(record + 0x10),
            mipmap,
        );
    } else {
        let _ = call9_u32(
            FN_801C6FA0_TEX_INIT_CI,
            p32(texture_obj),
            read_u32(record + 0x08),
            read_u16(record + 0x02) as u32,
            read_u16(record) as u32,
            read_u32(record + 0x04),
            read_u32(record + 0x0C),
            read_u32(record + 0x10),
            mipmap,
            0,
        );
        platform::call2(FN_801C7160_TEX_PALETTE, p32(texture_obj), palette);
    }

    // observed: `iVar4 = *piVar2` is a fresh load after the initialisers.
    let detail = read_u32(entry) as usize;
    let _ = call_f64x3_u32x6(
        FN_801C6FF0_TEX_LOD,
        read_u8(detail + 0x21) as f64,
        read_u8(detail + 0x22) as f64,
        read_f32(detail + 0x1C) as f64,
        p32(texture_obj),
        read_u32(detail + 0x14),
        read_u32(detail + 0x18),
        0,
        read_u8(detail + 0x20) as u32,
        0,
    );
}

/// Constructs a `nw4hbm::lyt::DrawInfo`.
///
/// Stores the vtable, fills four flag words at `+0x34..+0x40` with `0.0`,
/// three at `+0x44..+0x4C` with `1.0`, clears one byte at `+0x50`, and resets
/// the embedded transform at `+0x04` through `0x801C04D0`.
///
/// # Original
/// - Ghidra: `FUN_8017f390` at `0x8017F390` (124 bytes)
/// - C file: `work/ghidra/main-gb/export/8017f390_FUN_8017f390.c`
///
/// # Arguments
/// * `draw_info` - object to construct (at least `0x51` bytes).
///
/// # Returns
/// `draw_info`, mirroring the original `return param_1`.
///
/// # Side effects
/// Writes the vtable and seven floats, one byte and the transform.
///
/// # Safety
/// `draw_info` must be writable for `0x51` bytes.
///
/// # Confidence
/// high - every store is explicit; the constants were verified in
/// `main.elf` (`0.0` at `0x802F5E40`, `1.0` at `0x802F5E44`).
pub unsafe fn fn_8017f390(draw_info: usize) -> usize {
    write_u32(draw_info, VT_DRAW_INFO);
    // observed: `param_1[0xd..0x10] = DAT_802f5e40` (0.0f).
    write_f32(draw_info + 0x34, 0.0);
    write_f32(draw_info + 0x38, 0.0);
    write_f32(draw_info + 0x3C, 0.0);
    write_f32(draw_info + 0x40, 0.0);
    // observed: `param_1[0x11..0x13] = DAT_802f5e44` (1.0f).
    write_f32(draw_info + 0x44, 1.0);
    write_f32(draw_info + 0x48, 1.0);
    write_f32(draw_info + 0x4C, 1.0);
    // observed: `FUN_800046b4(param_1 + 0x14, 0, 1)` -> one zero byte at +0x50.
    platform::call3(MEMSET, p32(draw_info + 0x50), 0, 1);
    platform::call1(LAYOUT_RESET_POSITION, p32(draw_info + 0x04));
    draw_info
}

/// Constructs a `nw4hbm::lyt::Group` and loads its texture list.
///
/// Initialises the vtable and the circular child list at `+0x10`, clears the
/// enable byte at `+0x28`, copies 16 bytes from the resource at `+0x08` into
/// `+0x18`, then walks `count` 16-byte records at `resource + 0x1C`. Each
/// record that the accessor's virtual `+0x3C` accepts becomes a 12-byte list
/// node (`{0, 0, record}`) spliced into the list by [`fn_8018a590`].
///
/// # Original
/// - Ghidra: `FUN_8017f450` at `0x8017F450` (260 bytes)
/// - C file: `work/ghidra/main-gb/export/8017f450_FUN_8017f450.c`
///
/// # Arguments
/// * `group` - object to construct.
/// * `resource` - group resource: `+0x08` copied block, `u16 +0x18` count,
///   records at `+0x1C` with a 16-byte stride.
/// * `accessor` - resource accessor whose virtual `+0x3C` resolves one record.
///
/// # Returns
/// `group` (the original leaves the object pointer in `r3`).
///
/// # Side effects
/// Writes the vtable and fields, copies 16 bytes, allocates list nodes through
/// `0x801D5540`, and links them.
///
/// # Safety
/// `group` must be writable; `resource` must hold `count * 0x10` record bytes;
/// `accessor` must have a valid vtable; the heap must accept a 12-byte
/// allocation.
///
/// # Confidence
/// medium - control flow and offsets are exact; the original enters through
/// `0x802DCD64` (`__savegprlr_23`), which Ghidra prints as a call whose
/// `r3:r4` return duplicates the two leading arguments, so the signature has
/// no hidden operands.
pub unsafe fn fn_8017f450(group: usize, resource: usize, accessor: usize) -> usize {
    write_u32(group + 0x04, 0);
    write_u32(group, VT_GROUP);
    write_u32(group + 0x08, 0);
    write_u32(group + 0x0C, 0);
    // observed: `puVar1[4] = puVar1[5] = puVar1 + 4`, i.e. the sentinel at
    // +0x10 points at itself and both link words hold that address.
    write_u32(group + 0x10, p32(group + 0x10));
    write_u32(group + 0x14, p32(group + 0x10));
    write_u8(group + 0x28, 0);
    platform::call3(
        FN_802E19F8_STRNCPY,
        p32(group + 0x18),
        p32(resource + 0x08),
        0x10,
    );

    let count = read_u16(resource + 0x18) as usize;
    let mut record = resource + 0x1C;
    let mut index = 0usize;
    while index < count {
        let found = platform::call3(
            vtable_method(accessor, 0, 0x3C),
            p32(accessor),
            p32(record),
            1,
        );
        if found != 0 {
            let node = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), 0x0C);
            if node != 0 {
                let node = node as usize;
                write_u32(node, 0);
                write_u32(node + 0x04, 0);
                write_u32(node + 0x08, found);
                let head = crate::platform::GuestLocal::<u32>::new( p32(group + 0x10));
                fn_8018a590((group + 0x0C) as *mut u32, head.as_ptr() as *const u32, node as *mut u32);
            }
        }
        record += 0x10;
        index += 1;
    }
    group
}

/// Releases every node of a group's child list and optionally the header.
///
/// Walks the circular list whose sentinel is `owner + 0x04`, unlinks each
/// node with [`fn_8018a540`], and when the node's `+0x24` byte is zero
/// destroys the owning object at `node - 4` through virtual `+0x08` and frees
/// it. The header is then cleared with [`fn_8018a4b0`] and the object itself
/// may be freed.
///
/// # Original
/// - Ghidra: `FUN_8017f610` at `0x8017F610` (200 bytes)
/// - C file: `work/ghidra/main-gb/export/8017f610_FUN_8017f610.c`
///
/// # Arguments
/// * `owner` - object whose `+0x04` is the list sentinel (the list header is
///   at `+0x00`).
/// * `delete_flag` - `> 0` frees `owner` after the list is empty.
///
/// # Returns
/// `owner`, unchanged.
///
/// # Side effects
/// Unlinks every node, destroys and frees the surviving children, frees the
/// list header and possibly `owner`.
///
/// # Safety
/// `owner` must hold a consistent list; every child must be a live allocation
/// when its byte `+0x24` is zero.
///
/// # Confidence
/// medium - the list walk is exact; `0x802DCD70` is `__savegprlr_26`, so
/// Ghidra's void signature hides `(this, delete_flag)`.
pub unsafe fn fn_8017f610(owner: usize, delete_flag: i32) -> usize {
    if owner != 0 {
        let sentinel = owner + 0x04;
        let mut node = read_u32(owner + 0x04) as usize;
        while node != sentinel {
            let next = read_u32(node) as usize;
            let head = crate::platform::GuestLocal::<u32>::new( p32(node));
            fn_8018a540(owner as *mut u32, head.as_ptr() as *const u32);
            if read_u8(node + 0x24) == 0 {
                let child = node - 4;
                platform::call2(
                    vtable_method(child, 0, 0x08),
                    p32(child),
                    0xFFFF_FFFF,
                );
                platform::call2(FN_801D5550_FREE, read_u32(HEAP_80378008), p32(child));
            }
            node = next;
        }
        fn_8018a4b0(owner as *mut u8, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(owner));
        }
    }
    owner
}

/// Tears down a `nw4hbm::lyt::Layout`.
///
/// Stores the layout vtable, releases the group at `+0x14` through
/// [`fn_8017f610`], releases the pane at `+0x10` when its `+0xD0` byte is
/// zero, then unlinks and destroys every remaining child node in the list at
/// `+0x08`, clears that header and optionally frees the layout.
///
/// # Original
/// - Ghidra: `FUN_8017f980` at `0x8017F980` (296 bytes)
/// - C file: `work/ghidra/main-gb/export/8017f980_FUN_8017f980.c`
///
/// # Arguments
/// * `layout` - object to destroy.
/// * `delete_flag` - `> 0` frees `layout` itself.
///
/// # Returns
/// `layout`, unchanged.
///
/// # Side effects
/// Virtual destructor calls, heap frees, list unlinking and header clearing.
///
/// # Safety
/// `layout` must be a live layout object with valid `+0x10`/`+0x14` members
/// and a consistent list at `+0x08`.
///
/// # Confidence
/// medium - all offsets and calls are observed; `0x802DCD70` is
/// `__savegprlr_26`, so the void Ghidra signature hides `(this, delete_flag)`.
pub unsafe fn fn_8017f980(layout: usize, delete_flag: i32) -> usize {
    if layout != 0 {
        write_u32(layout, VT_LAYOUT);
        let group = read_u32(layout + 0x14);
        if group != 0 {
            fn_8017f610(group as usize, -1);
            platform::call2(FN_801D5550_FREE, read_u32(HEAP_80378008), group);
        }
        let pane = read_u32(layout + 0x10) as usize;
        if pane != 0 && read_u8(pane + 0xD0) == 0 {
            platform::call2(
                vtable_method(pane, 0, 0x08),
                p32(pane),
                0xFFFF_FFFF,
            );
            platform::call2(
                FN_801D5550_FREE,
                read_u32(HEAP_80378008),
                read_u32(layout + 0x10),
            );
        }
        let sentinel = layout + 0x08;
        let mut node = read_u32(layout + 0x08) as usize;
        while node != sentinel {
            let next = read_u32(node) as usize;
            let head = crate::platform::GuestLocal::<u32>::new( p32(node));
            fn_8018a540((layout + 0x04) as *mut u32, head.as_ptr() as *const u32);
            let child = node - 4;
            platform::call2(vtable_method(child, 0, 0x08), p32(child), 0xFFFF_FFFF);
            platform::call2(FN_801D5550_FREE, read_u32(HEAP_80378008), p32(child));
            node = next;
        }
        fn_8018a4b0((layout + 0x04) as *mut u8, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(layout));
        }
    }
    layout
}

// ---------------------------------------------------------------------------
// `nw4hbm::lyt::Pane` - constructor, teardown and child-list operations
// ---------------------------------------------------------------------------

/// Constructs a `nw4hbm::lyt::Pane`.
///
/// Stores the vtable, initialises two circular child lists (count at `+0x10`
/// with sentinel at `+0x14`, and count at `+0x1C` with sentinel at `+0x20`),
/// copies the 16-byte block at `resource + 0x0C` to `+0xB4` and the 8-byte
/// block at `resource + 0x1C` to `+0xC4`, copies eight transform words, then
/// copies the three visibility bytes at the end.
///
/// # Original
/// - Ghidra: `FUN_801836b0` at `0x801836B0` (280 bytes)
/// - C file: `work/ghidra/main-gb/export/801836b0_FUN_801836b0.c`
///
/// # Arguments
/// * `pane` - object to construct (at least `0xD1` bytes).
/// * `resource` - pane resource: name bytes at `+0x08`/`+0x09`/`+0x0A`,
///   transform words at `+0x24..+0x48`, and two zero-terminated blocks at
///   `+0x0C` and `+0x1C`.
///
/// # Returns
/// `pane`, mirroring the original `return param_1`.
///
/// # Side effects
/// Writes the vtable, 20 fields, two copied byte blocks and three flag bytes.
///
/// # Safety
/// `pane` must be writable for `0xD1` bytes; `resource` must contain the
/// blocks read at `+0x0C..+0x24` and the words up to `+0x48`.
///
/// # Confidence
/// high - every store and both copies are explicit; the two lists match the
/// headers consumed by [`fn_801837d0`], [`fn_80184270`] and [`fn_80184310`].
pub unsafe fn fn_801836b0(pane: usize, resource: usize) -> usize {
    write_u32(pane + 0x04, 0);
    write_u32(pane + 0x08, 0);
    write_u32(pane, VT_PANE);
    write_u32(pane + 0x10, 0);
    // observed: `param_1[5] = param_1[6] = param_1 + 5` -> the first list's
    // sentinel is at +0x14 and points at itself.
    write_u32(pane + 0x14, p32(pane + 0x14));
    write_u32(pane + 0x18, p32(pane + 0x14));
    write_u32(pane + 0x1C, 0);
    // observed: `param_1[8] = param_1[9] = param_1 + 8` -> second list's
    // sentinel at +0x20.
    write_u32(pane + 0x20, p32(pane + 0x20));
    write_u32(pane + 0x24, p32(pane + 0x20));
    write_u32(pane + 0x4C, 0);
    write_u32(pane + 0x50, 0);
    write_u32(pane + 0x0C, 0);
    write_u32(pane + 0x28, 0);
    write_u8(pane + 0xD0, 0);
    write_u8(pane + 0xCC, read_u8(resource + 0x09));
    platform::call3(
        FN_802E19F8_STRNCPY,
        p32(pane + 0xB4),
        p32(resource + 0x0C),
        0x10,
    );
    platform::call3(
        FN_802E19F8_STRNCPY,
        p32(pane + 0xC4),
        p32(resource + 0x1C),
        0x08,
    );
    // observed: `param_1[0xb..0x12] = *(param_2 + 0x24..0x40)`.
    let mut i = 0usize;
    while i < 8 {
        let value = read_u32(resource + 0x24 + i * 4);
        write_u32(pane + 0x2C + i * 4, value);
        i += 1;
    }
    write_u32(pane + 0x4C, read_u32(resource + 0x44));
    write_u32(pane + 0x50, read_u32(resource + 0x48));
    let flag = read_u8(resource + 0x0A);
    write_u8(pane + 0xCD, flag);
    write_u8(pane + 0xCE, flag);
    write_u8(pane + 0xCF, read_u8(resource + 0x08));
    pane
}

/// Destroys a `nw4hbm::lyt::Pane`.
///
/// Resets the vtable, unlinks and destroys every node of the child list at
/// `+0x14`, calls virtual `+0x50` on the pane, releases the material at
/// `+0x28` when its `+0x54` byte is zero, clears both list headers, and
/// optionally frees the pane.
///
/// # Original
/// - Ghidra: `FUN_801837d0` at `0x801837D0` (308 bytes)
/// - C file: `work/ghidra/main-gb/export/801837d0_FUN_801837d0.c`
///
/// # Arguments
/// * `pane` - pane to destroy, or `0`.
/// * `delete_flag` - `> 0` frees `pane` after teardown.
///
/// # Returns
/// `pane`, unchanged.
///
/// # Side effects
/// Virtual destructor calls, heap frees, list unlinking.
///
/// # Safety
/// `pane` must be a live `Pane`; child nodes must be live allocations when
/// their `+0xCC` byte is zero.
///
/// # Confidence
/// medium - the walk and all offsets are exact; `0x802DCD70` is
/// `__savegprlr_26`, so the void Ghidra signature hides `(this, delete_flag)`.
pub unsafe fn fn_801837d0(pane: usize, delete_flag: i32) -> usize {
    if pane != 0 {
        write_u32(pane, VT_PANE);
        let sentinel = pane + 0x14;
        let mut node = read_u32(pane + 0x14) as usize;
        while node != sentinel {
            let next = read_u32(node) as usize;
            let head = crate::platform::GuestLocal::<u32>::new( p32(node));
            // observed: count at +0x10, node passed as the one-element list.
            fn_8018a540((pane + 0x10) as *mut u32, head.as_ptr() as *const u32);
            if read_u8(node + 0xCC) == 0 {
                let child = node - 4;
                platform::call2(
                    vtable_method(child, 0, 0x08),
                    p32(child),
                    0xFFFF_FFFF,
                );
                platform::call2(FN_801D5550_FREE, read_u32(HEAP_80378008), p32(child));
            }
            node = next;
        }
        platform::call2(vtable_method(pane, 0, 0x50), p32(pane), 0);
        let material = read_u32(pane + 0x28) as usize;
        if material != 0 && read_u8(material + 0x54) == 0 {
            platform::call2(
                vtable_method(material, 0, 0x08),
                p32(material),
                0xFFFF_FFFF,
            );
            platform::call2(
                FN_801D5550_FREE,
                read_u32(HEAP_80378008),
                read_u32(pane + 0x28),
            );
        }
        // observed: `if (piVar1 + 7 != 0)` and `if (piVar1 + 4 != 0)` are
        // always true; they clear the headers at +0x1C and +0x10.
        fn_8018a4b0((pane + 0x1C) as *mut u8, 0);
        fn_8018a4b0((pane + 0x10) as *mut u8, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(pane));
        }
    }
    pane
}

/// Notifies every child pane and then the material of a pane.
///
/// Walks the circular list at `+0x20`; for each node whose `+0x0E` byte is
/// zero it invokes virtual `+0x18` on the object at `node + 8` with the
/// `u16` at `node + 0x0C` and the pane itself. After the walk, if the pane's
/// `+0xCF` bit 0 is set or `flags` bit 0 is clear, virtual `+0x1C` runs on the
/// material at `+0x28` when present.
///
/// # Original
/// - Ghidra: `FUN_801840e0` at `0x801840E0` (180 bytes)
/// - C file: `work/ghidra/main-gb/export/801840e0_FUN_801840e0.c`
///
/// # Arguments
/// * `pane` - pane whose children are visited.
/// * `flags` - caller flags; only bit 0 is tested.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call per eligible child and one on the material.
///
/// # Safety
/// The list and every referenced object must be live and have valid vtables.
///
/// # Confidence
/// high - the walk, both guards and the call arguments are observed.
pub unsafe fn fn_801840e0(pane: usize, flags: u32) {
    let sentinel = pane + 0x20;
    let mut node = read_u32(pane + 0x20) as usize;
    while node != sentinel {
        if read_u8(node + 0x0E) == 0 {
            let child = read_u32(node + 0x08) as usize;
            let index = read_u16(node + 0x0C) as u32;
            platform::call3(
                vtable_method(child, 0, 0x18),
                p32(child),
                index,
                p32(pane),
            );
        }
        node = read_u32(node) as usize;
    }
    if (read_u8(pane + 0xCF) & 1) != 0 || (flags & 1) == 0 {
        let material = read_u32(pane + 0x28) as usize;
        if material != 0 {
            platform::call1(vtable_method(material, 0, 0x1C), p32(material));
        }
    }
}

/// Removes children from a pane, optionally only those matching a key.
///
/// Calls virtual `+0x14` on the material at `+0x28` when present, then walks
/// the list at `+0x20`. A node is unlinked with [`fn_8018a540`] and its key
/// (`+0x08`) and `u16 +0x0C` fields are cleared when `key` is zero or equals
/// the node's key.
///
/// # Original
/// - Ghidra: `FUN_80184270` at `0x80184270` (160 bytes)
/// - C file: `work/ghidra/main-gb/export/80184270_FUN_80184270.c`
///
/// # Arguments
/// * `pane` - pane whose children are removed.
/// * `key` - `0` removes every child; otherwise only matching keys.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Unlinks nodes, clears two node fields and decrements the count at `+0x1C`.
///
/// # Safety
/// `pane` must hold a consistent list at `+0x20`.
///
/// # Confidence
/// medium - the filter and writes are exact; `0x802DCD70` is
/// `__savegprlr_26`, so the void signature hides `(this, key)`.
pub unsafe fn fn_80184270(pane: usize, key: u32) {
    let material = read_u32(pane + 0x28) as usize;
    if material != 0 {
        platform::call1(vtable_method(material, 0, 0x14), p32(material));
    }
    let sentinel = pane + 0x20;
    let mut node = read_u32(pane + 0x20) as usize;
    while node != sentinel {
        let next = read_u32(node) as usize;
        if key == 0 || read_u32(node + 0x08) == key {
            let head = crate::platform::GuestLocal::<u32>::new( p32(node));
            fn_8018a540((pane + 0x1C) as *mut u32, head.as_ptr() as *const u32);
            write_u32(node + 0x08, 0);
            write_u16(node + 0x0C, 0);
        }
        node = next;
    }
}

/// Inserts a child node into the pane's second list, just after the sentinel.
///
/// Builds a one-word stack cell containing the sentinel address `pane + 0x20`
/// and calls [`fn_8018a590`] with the count at `pane + 0x1C`.
///
/// # Original
/// - Ghidra: `FUN_80184310` at `0x80184310` (52 bytes)
/// - C file: `work/ghidra/main-gb/export/80184310_FUN_80184310.c`
///
/// # Arguments
/// * `pane` - list owner.
/// * `node` - two-word link to splice in.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the node links, the sentinel's newer link, the old first node's
/// older link and increments the count at `+0x1C`.
///
/// # Safety
/// `node` must be a writable two-word link; `pane` must be a live `Pane`.
///
/// # Confidence
/// high - the stack cell and call are observed exactly.
pub unsafe fn fn_80184310(pane: usize, node: usize) {
    let head = crate::platform::GuestLocal::<u32>::new( p32(pane + 0x20));
    fn_8018a590(
        (pane + 0x1C) as *mut u32,
        head.as_ptr() as *const u32,
        node as *mut u32,
    );
}

/// Finds a child of a pane by key, falling back to the material.
///
/// First searches the circular list at `+0x1C` for a node whose `+0x08` word
/// equals `key` through `0x8017DFA0`. When that returns zero, virtual `+0x20`
/// on the material at `+0x28` is asked instead; a missing material also
/// yields zero.
///
/// # Original
/// - Ghidra: `FUN_80184350` at `0x80184350` (120 bytes)
/// - C file: `work/ghidra/main-gb/export/80184350_FUN_80184350.c`
///
/// # Arguments
/// * `pane` - pane to search.
/// * `key` - child identifier.
///
/// # Returns
/// The child node, or `0` when neither the list nor the material knows it.
///
/// # Side effects
/// Material virtual call only.
///
/// # Safety
/// The list and material must be live.
///
/// # Confidence
/// high - both lookups and the short-circuit order are observed; the second
/// argument of `0x8017DFA0` is dropped by Ghidra but still in `r4`.
pub unsafe fn fn_80184350(pane: usize, key: u32) -> usize {
    let mut found = platform::call2(FN_8017DFA0_FIND, p32(pane + 0x1C), key);
    if found == 0 {
        let material = read_u32(pane + 0x28) as usize;
        if material != 0 {
            found = platform::call2(vtable_method(material, 0, 0x20), p32(material), key);
        }
    }
    found as usize
}

/// Forwards an argument to every texture of a picture, then to its children.
///
/// Iterates the `count` entries at `array + 4` with an 8-byte stride, calling
/// virtual `+0x14` on each texture object with `arg`, then calls
/// [`fn_80184270`] with the same argument so matching children are removed.
///
/// # Original
/// - Ghidra: `FUN_801868c0` at `0x801868C0` (144 bytes)
/// - C file: `work/ghidra/main-gb/export/801868c0_FUN_801868c0.c`
///
/// # Arguments
/// * `picture` - picture pane: `u8 +0x100` count, pointer `+0xFC`.
/// * `arg` - forwarded to each texture and to [`fn_80184270`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call per texture plus the child-list removals.
///
/// # Safety
/// The array must hold `count` readable 8-byte entries whose second word is a
/// live texture object.
///
/// # Confidence
/// high - the loop and the forwarded argument are observed; Ghidra drops the
/// second operand of `0x80184270`, but `r4` still holds `param_2`.
pub unsafe fn fn_801868c0(picture: usize, arg: u32) {
    let array = read_u32(picture + 0xFC) as usize;
    let count = read_u8(picture + 0x100) as usize;
    let mut index = 0usize;
    while index < count {
        let texture = read_u32(array + index * 8 + 4) as usize;
        platform::call2(vtable_method(texture, 0, 0x14), p32(texture), arg);
        index += 1;
    }
    fn_80184270(picture, arg);
}

// ---------------------------------------------------------------------------
// Picture texture math and font glyph buffers (`nw4hbm::lyt::Picture`)
// ---------------------------------------------------------------------------

/// Computes the horizontal and vertical flip offsets of a picture.
///
/// The flip byte at `+0xCC` is split into a horizontal mode (`% 3`) and a
/// vertical mode (`/ 3`):
///
/// | mode | returned offset |
/// |---|---|
/// | `0` | `0.0` |
/// | `1` | `-size * 0.5` |
/// | `2` | `-size` |
///
/// The horizontal size is the `f32` at `+0x4C`, the vertical size the `f32`
/// at `+0x50`.
///
/// # Original
/// - Ghidra: `FUN_801845a0` at `0x801845A0` (224 bytes)
/// - C file: `work/ghidra/main-gb/export/801845a0_FUN_801845a0.c`
///
/// # Arguments
/// * `picture` - picture pane; `u8 +0xCC` flip mode, `f32 +0x4C` width,
///   `f32 +0x50` height.
///
/// # Returns
/// `(horizontal_offset, vertical_offset)`; the original returns the pair in
/// `r3:r4`, which the caller stores as two words.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `picture` must be readable at the offsets above.
///
/// # Confidence
/// high - the selector table and the `0.5` constant (`0x802F5E98`) are
/// verified in `main.elf`.
pub unsafe fn fn_801845a0(picture: usize) -> (f32, f32) {
    let flip = read_u8(picture + 0xCC) as u32;
    let horizontal = match flip % 3 {
        1 => -read_f32(picture + 0x4C) * 0.5,
        2 => -read_f32(picture + 0x4C),
        _ => 0.0,
    };
    let vertical = match flip / 3 {
        1 => -read_f32(picture + 0x50) * 0.5,
        2 => -read_f32(picture + 0x50),
        _ => 0.0,
    };
    (horizontal, vertical)
}

/// Applies one texture record to a picture through virtual `+0x68`.
///
/// Builds a 36-byte texture object with [`fn_8017f250`] at frame `0`, then
/// passes it to the picture's virtual `+0x68` method.
///
/// # Original
/// - Ghidra: `FUN_801848a0` at `0x801848A0` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/801848a0_FUN_801848a0.c`
///
/// # Arguments
/// * `picture` - receiver with a vtable at `+0x00`.
/// * `texture` - texture-pattern resource consumed by [`fn_8017f250`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills the stack texture object and calls virtual `+0x68`.
///
/// # Safety
/// `picture` must have a valid vtable; `texture` must be a valid pattern.
///
/// # Confidence
/// high - the call order and the 36-byte scratch object are explicit.
pub unsafe fn fn_801848a0(picture: usize, texture: usize) {
    // observed: `undefined1 auStack_28[36]` is the GX texture object.
    let mut texture_obj = crate::platform::GuestLocal::new( [0u32; 9]);
    fn_8017f250(texture_obj.as_mut_ptr() as usize, texture, 0);
    platform::call2(
        vtable_method(picture, 0, 0x68),
        p32(picture),
        texture_obj.as_mut_ptr() as u32,
    );
}

/// Releases an object when both a non-null pointer and a positive flag hold.
///
/// Same shape as [`fn_8018aa80`]: the pointer is returned unchanged whether
/// or not the release ran.
///
/// # Original
/// - Ghidra: `FUN_80184c00` at `0x80184C00` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/80184c00_FUN_80184c00.c`
///
/// # Arguments
/// * `object` - object to release, may be null.
/// * `delete_flag` - `> 0` releases the object.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Calls `0x802B9280(object)` when both conditions hold.
///
/// # Safety
/// A non-null `object` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - identical guard to the sibling wrappers; the dropped argument is
/// restored from `r3`.
pub unsafe fn fn_80184c00(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(FN_802B9280, p32(object));
    }
    object
}

/// Grows the `u16` glyph-index buffer of a picture's font.
///
/// The requested byte size is `(count + 1) * 2` truncated to `u16`. When the
/// stored capacity at `+0xF8` is smaller, virtual `+0x68` releases the old
/// buffer, `0x801D5540` allocates the new size, the pointer is stored at
/// `+0xD4`, and the truncated size is stored at `+0xF8` on success.
///
/// # Original
/// - Ghidra: `FUN_80185cc0` at `0x80185CC0` (124 bytes)
/// - C file: `work/ghidra/main-gb/export/80185cc0_FUN_80185cc0.c`
///
/// # Arguments
/// * `font` - glyph-buffer owner; pointer `+0xD4`, `u16` capacity `+0xF8`.
/// * `count` - requested entry count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May release the old buffer and replace it with a new allocation.
///
/// # Safety
/// `font` must be a live font object; the heap must be initialised.
///
/// # Confidence
/// high - the size formula, comparison and stores are explicit.
pub unsafe fn fn_80185cc0(font: usize, count: i32) {
    if count != 0 {
        // observed: `(param_2 + 1) * 2 & 0xfffe` - counted in bytes.
        let size = (count as u32).wrapping_add(1).wrapping_mul(2) & 0xFFFE;
        if (read_u16(font + 0xF8) as u32) < size {
            platform::call1(vtable_method(font, 0, 0x68), p32(font));
            let buffer = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), size);
            write_u32(font + 0xD4, buffer);
            if buffer != 0 {
                write_u16(font + 0xF8, size as u16);
            }
        }
    }
}

/// Frees the glyph-index buffer of a picture's font.
///
/// When the pointer at `+0xD4` is non-null it is released through
/// `0x801D5550`, then the pointer is cleared and the `u16` capacity at `+0xF8`
/// is zeroed.
///
/// # Original
/// - Ghidra: `FUN_80185d40` at `0x80185D40` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/80185d40_FUN_80185d40.c`
///
/// # Arguments
/// * `font` - glyph-buffer owner.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One heap free and two clears.
///
/// # Safety
/// A non-null `+0xD4` must be an allocation from `0x801D5540`.
///
/// # Confidence
/// high - the guard and clears are exact; the free operand is the value at
/// `+0xD4`.
pub unsafe fn fn_80185d40(font: usize) {
    let buffer = read_u32(font + 0xD4);
    if buffer != 0 {
        platform::call2(FN_801D5550_FREE, read_u32(HEAP_80378008), buffer);
        write_u32(font + 0xD4, 0);
        write_u16(font + 0xF8, 0);
    }
}

/// Writes glyph indices into the font buffer and terminates the written run.
///
/// The usable element count is `capacity / 2 - 1` (zero when the capacity is
/// zero). When `start` is inside that range, the count is clamped to the
/// remaining space, `count` `u16` values are copied from `src`, the end index
/// is stored at `+0xFA`, and a `u16 0` terminator is written at that index.
///
/// # Original
/// - Ghidra: `FUN_80185e00` at `0x80185E00` (200 bytes)
/// - C file: `work/ghidra/main-gb/export/80185e00_FUN_80185e00.c`
///
/// # Arguments
/// * `font` - glyph-buffer owner; pointer `+0xD4`, capacity `+0xF8`.
/// * `src` - source `u16` array.
/// * `start` - first destination glyph index.
/// * `count` - requested number of entries.
///
/// # Returns
/// The number of `u16` entries actually copied, or `0` when no buffer exists
/// or `start` is outside the usable range.
///
/// # Side effects
/// Copies into the glyph buffer, writes the end index at `+0xFA` and writes
/// one terminator.
///
/// # Safety
/// `src` must cover the clamped count; `font + 0xD4` must be a live buffer of
/// the recorded capacity.
///
/// # Confidence
/// high - the clamp, copy size and terminator are exact.
pub unsafe fn fn_80185e00(font: usize, src: u32, start: u32, count: u32) -> u32 {
    let buffer = read_u32(font + 0xD4);
    if buffer == 0 {
        return 0;
    }
    let capacity = read_u16(font + 0xF8);
    // observed: `(capacity >> 1) - 1` masked off when capacity == 0.
    let usable = if capacity == 0 {
        0
    } else {
        ((capacity >> 1) as u32).wrapping_sub(1) & 0xFFFF
    };
    if start < usable {
        // observed: `uVar1 = (usable - param_3) & 0xffff; if (uVar1 < param_4)
        // param_4 = uVar1;`
        let available = (usable - start) & 0xFFFF;
        let written = if available < count { available } else { count };
        platform::call3(
            FN_80004364_MEMCPY,
            (buffer as usize).wrapping_add((start & 0xFFFF) as usize * 2) as u32,
            src,
            (written & 0xFFFF) << 1,
        );
        write_u16(font + 0xFA, (start.wrapping_add(written)) as u16);
        let terminator = (buffer as usize).wrapping_add(
            (start.wrapping_add(written)).wrapping_mul(2) as usize & 0x1_FFFE,
        );
        write_u16(terminator, 0);
        written & 0xFFFF
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// `BRFNT` parsing and `nw4hbm::ut::Font` construction
// ---------------------------------------------------------------------------

/// Validates a `BRFNT` section header.
///
/// Checks, in order: the tag at `+0x00`, the byte-order mark `0xFEFF` at
/// `+0x04`, the version at `+0x06`, and that the file size at `+0x08` can hold
/// `count * 8 + 0x10` bytes. The return value is a flag derived from the
/// section count and requested count; every failing check returns `0`.
///
/// # Original
/// - Ghidra: `FUN_80188860` at `0x80188860` (116 bytes)
/// - C file: `work/ghidra/main-gb/export/80188860_FUN_80188860.c`
///
/// # Arguments
/// * `header` - candidate `BRFNT` header.
/// * `tag` - expected `u32` tag (`0x52464E54`, `"RFNT"` at both call sites).
/// * `version` - expected `u16` version (`0x102` or `0x104`).
/// * `count` - requested section count (2 at both call sites).
///
/// # Returns
/// `0` on any mismatch, otherwise the sign flag of the count comparison.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `header` must be readable for at least `0x10` bytes.
///
/// # Confidence
/// high - every comparison is explicit; the final expression is reproduced
/// with the same precedence.
pub unsafe fn fn_80188860(header: usize, tag: u32, version: u32, count: u32) -> u32 {
    if read_u32(header) != tag {
        return 0;
    }
    // observed: `*(short *)(param_1 + 1) != -0x101` -> BOM 0xFEFF at +0x04.
    if read_u16(header + 0x04) != 0xFEFF {
        return 0;
    }
    if read_u16(header + 0x06) as u32 != version {
        return 0;
    }
    if read_u32(header + 0x08) < (count & 0xFFFF).wrapping_mul(8).wrapping_add(0x10) {
        return 0;
    }
    // observed: `((e | ~count) - ((e - count) >> 1)) >> 31`.
    let sections = read_u16(header + 0x0E) as u32;
    let value = (sections | !count).wrapping_sub(sections.wrapping_sub(count) >> 1);
    value >> 31
}

/// Selects one of four font text-encoding handlers.
///
/// Kinds `0..=3` write the same pair of fields (`0` at `+0x04`, `-1` at
/// `+0x08`) and a different code pointer at `+0x0C`:
///
/// | kind | handler |
/// |---|---|
/// | `0` | `0x801888E0` |
/// | `1` | `0x80188960` |
/// | `2` | `0x801889A0` |
/// | `>= 3` | `0x80188980` |
///
/// # Original
/// - Ghidra: `FUN_8018a400` at `0x8018A400` (164 bytes)
/// - C file: `work/ghidra/main-gb/export/8018a400_FUN_8018a400.c`
///
/// # Arguments
/// * `font` - font object; fields at `+0x04`, `+0x08`, `+0x0C`.
/// * `kind` - handler selector returned by the font's virtual `+0x54`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three words.
///
/// # Safety
/// `font` must be writable through `+0x0F`.
///
/// # Confidence
/// high - the four tables were read from `main.elf` (`0x8032C6E0..0x8032C70C`).
pub unsafe fn fn_8018a400(font: usize, kind: u32) {
    match kind {
        0 => {
            write_u32(font + 0x08, 0xFFFF_FFFF);
            write_u32(font + 0x04, 0);
            write_u32(font + 0x0C, 0x8018_88E0);
        }
        1 => {
            write_u32(font + 0x08, 0xFFFF_FFFF);
            write_u32(font + 0x04, 0);
            write_u32(font + 0x0C, 0x8018_8960);
        }
        2 => {
            write_u32(font + 0x08, 0xFFFF_FFFF);
            write_u32(font + 0x04, 0);
            write_u32(font + 0x0C, 0x8018_89A0);
        }
        _ => {
            write_u32(font + 0x08, 0xFFFF_FFFF);
            write_u32(font + 0x04, 0);
            write_u32(font + 0x0C, 0x8018_8980);
        }
    }
}

/// Constructs the `nw4hbm::ut::Font` base object.
///
/// Writes the intermediate base vtable, the three-word encoding field group
/// (`0`, `-1`, `0x80188980`), then overrides the vtable with the final
/// `nw4hbm::ut::Font` table and clears the two words at `+0x10`/`+0x14`.
///
/// # Original
/// - Ghidra: `FUN_8018aa30` at `0x8018AA30` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/8018aa30_FUN_8018aa30.c`
///
/// # Arguments
/// * `font` - object to construct (at least `0x18` bytes).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes two vtable words and five fields.
///
/// # Safety
/// `font` must be writable for `0x18` bytes.
///
/// # Confidence
/// high - all constants were read from `main.elf`
/// (`0x8032C7F0..0x8032C7F8`).
pub unsafe fn fn_8018aa30(font: usize) {
    write_u32(font, VT_FONT_BASE);
    write_u32(font + 0x08, 0xFFFF_FFFF);
    write_u32(font + 0x04, 0);
    write_u32(font + 0x0C, 0x8018_8980);
    write_u32(font, VT_FONT);
    write_u32(font + 0x10, 0);
    write_u32(font + 0x14, 0);
}

/// Constructs a `nw4hbm::ut::ResFont` on top of the `ut::Font` base.
///
/// Runs [`fn_8018aa30`] and then stores the `ResFont` vtable; the derived
/// destructor slot is `0x8018A770`.
///
/// # Original
/// - Ghidra: `FUN_8018a730` at `0x8018A730` (60 bytes)
/// - C file: `work/ghidra/main-gb/export/8018a730_FUN_8018a730.c`
///
/// # Arguments
/// * `font` - object to construct.
///
/// # Returns
/// `font`, mirroring the original `return param_1`.
///
/// # Side effects
/// Delegates to [`fn_8018aa30`] and overwrites the vtable word.
///
/// # Safety
/// `font` must be writable for at least `0x18` bytes.
///
/// # Confidence
/// high - Ghidra drops the base-constructor argument, but `r3` still holds
/// `param_1` at the `bl`.
pub unsafe fn fn_8018a730(font: usize) -> usize {
    fn_8018aa30(font);
    write_u32(font, VT_RES_FONT);
    font
}

/// Converts a `BRFNT` file's relative section offsets to absolute pointers.
///
/// Starting at `header + u16[+0x0C]`, the function walks `u16[+0x0E]`
/// sections. For each known tag it rewrites the stored relative offsets:
///
/// | tag | adjusted words |
/// |---|---|
/// | `FINF` | `+0x10`, `+0x14` (if non-zero), `+0x18` (if non-zero) |
/// | `TGLP` | `+0x1C` |
/// | `CWDH` | `+0x0C` (if non-zero) |
/// | `CMAP` | `+0x10` (if non-zero) |
/// | `GLGR` | none |
///
/// The `FINF` payload pointer (`section + 8`) is returned. When every section
/// has been visited, the header tag is overwritten with `"RFNU"`; an unknown
/// tag aborts with `0` before that write.
///
/// # Original
/// - Ghidra: `FUN_8018a920` at `0x8018A920` (264 bytes)
/// - C file: `work/ghidra/main-gb/export/8018a920_FUN_8018a920.c`
///
/// # Arguments
/// * `header` - `BRFNT` file base.
///
/// # Returns
/// Pointer to the `FINF` payload, or `0` for an unknown section tag.
///
/// # Side effects
/// Rewrites section offsets in place; may write `"RFNU"` at `header + 0x00`.
///
/// # Safety
/// The whole section chain must be readable/writable and in bounds.
///
/// # Confidence
/// high - tag dispatch and offset corrections are exact; tags decode to
/// `FINF`, `TGLP`, `CWDH`, `CMAP`, `GLGR`.
pub unsafe fn fn_8018a920(header: usize) -> usize {
    let mut info = 0usize;
    let mut section = header + read_u16(header + 0x0C) as usize;
    let count = read_u16(header + 0x0E) as i32;
    let mut index = 0i32;
    loop {
        if count <= index {
            write_u32(header, TAG_RFNU);
            return info;
        }
        let tag = read_u32(section);
        if tag == TAG_FINF {
            info = section + 8;
            write_u32(section + 0x10, (header + read_u32(section + 0x10) as usize) as u32);
            let word = read_u32(section + 0x14);
            if word != 0 {
                write_u32(section + 0x14, (header + word as usize) as u32);
            }
            let word = read_u32(section + 0x18);
            if word != 0 {
                write_u32(section + 0x18, (header + word as usize) as u32);
            }
        } else if tag == TAG_TGLP {
            write_u32(section + 0x1C, (header + read_u32(section + 0x1C) as usize) as u32);
        } else if tag == TAG_CWDH {
            let word = read_u32(section + 0x0C);
            if word != 0 {
                write_u32(section + 0x0C, (header + word as usize) as u32);
            }
        } else if tag == TAG_CMAP {
            let word = read_u32(section + 0x10);
            if word != 0 {
                write_u32(section + 0x10, (header + word as usize) as u32);
            }
        } else if tag != TAG_GLGR {
            return 0;
        }
        index += 1;
        section += read_u32(section + 0x04) as usize;
    }
}

/// Loads a `BRFNT` file into a `nw4hbm::ut::ResFont`.
///
/// Rejects a font that already holds a file (`+0x10` non-zero). A file whose
/// first word is `"RFNU"` is scanned for its `FINF` section at
/// `+u16[+0x0C]`; otherwise the file must start with `"RFNT"` and pass
/// [`fn_80188860`] with version `0x104` (or `0x102` as the fallback), after
/// which [`fn_8018a920`] fixes the section offsets. With a `FINF` payload the
/// base/file pair is stored through `0x8018AAC0`, the font kind is queried
/// through virtual `+0x54`, and [`fn_8018a400`] installs the encoder.
///
/// # Original
/// - Ghidra: `FUN_8018a7d0` at `0x8018A7D0` (328 bytes)
/// - C file: `work/ghidra/main-gb/export/8018a7d0_FUN_8018a7d0.c`
///
/// # Arguments
/// * `font` - `ResFont` to load into.
/// * `data` - `BRFNT` file image.
///
/// # Returns
/// `true` when a `FINF` section was found and the font was bound; `false`
/// when the font was already bound or the file fails validation.
///
/// # Side effects
/// Writes `+0x10`/`+0x14` of the font, the encoder fields at `+0x04..+0x0C`,
/// and may rewrite the file image's section offsets.
///
/// # Safety
/// `font` must be a live `ResFont`; `data` must be a readable (and, for the
/// fixup path, writable) `BRFNT` image.
///
/// # Confidence
/// high - the branch tree and the `r5` argument of `0x8018AAC0` were checked
/// in the instruction stream (`0x8018A8B4..0x8018A8D8`).
pub unsafe fn fn_8018a7d0(font: usize, data: usize) -> bool {
    if read_u32(font + 0x10) != 0 {
        return false;
    }

    let mut info = 0usize;
    if read_u32(data) == TAG_RFNU {
        let mut section = data + read_u16(data + 0x0C) as usize;
        let mut remaining = read_u16(data + 0x0E) as u32;
        while remaining != 0 {
            if read_u32(section) == TAG_FINF {
                info = section + 8;
                break;
            }
            section += read_u32(section + 0x04) as usize;
            remaining -= 1;
        }
    } else {
        let version = read_u16(data + 0x06) as u32;
        let valid = if version == 0x104 {
            fn_80188860(data, TAG_RFNT, 0x104, 2)
        } else {
            fn_80188860(data, TAG_RFNT, 0x102, 2)
        };
        if valid == 0 {
            return false;
        }
        info = fn_8018a920(data);
    }

    if info != 0 {
        // observed: `r5` is the FINF payload returned by the branch above.
        platform::call3(FN_8018AAC0_FONT_INFO, p32(font), p32(data), p32(info));
        let kind = platform::call1(vtable_method(font, 0, 0x54), p32(font));
        fn_8018a400(font, kind);
    }
    info != 0
}

/// Maps a character code to a glyph index through the font's glyph groups.
///
/// Walks the linked list at `info + 0x10` (where `info` is the font's `+0x14`
/// payload). Each node holds a `u16` range `[+0x00, +0x02]` and its next link
/// at `+0x08`. On the first containing range, `0x8018AEA0` maps the code to a
/// signed `u16` glyph index; a value other than `-1` is stored at `info + 2`
/// and reported as success.
///
/// # Original
/// - Ghidra: `FUN_8018abe0` at `0x8018ABE0` (152 bytes)
/// - C file: `work/ghidra/main-gb/export/8018abe0_FUN_8018abe0.c`
///
/// # Arguments
/// * `font` - `ResFont` whose `+0x14` is the `FINF` payload.
/// * `code` - character code to map.
///
/// # Returns
/// `true` when a glyph index was found and stored at `info + 2`.
///
/// # Side effects
/// Writes the `u16` glyph index at `info + 2` on success.
///
/// # Safety
/// The glyph-group chain must be readable; `0x8018AEA0`'s payload pointers
/// must be valid.
///
/// # Confidence
/// medium - the list walk and the `-1` sentinel are exact; the range mapper
/// `0x8018AEA0` is not translated in this pass.
pub unsafe fn fn_8018abe0(font: usize, code: u32) -> bool {
    let info = read_u32(font + 0x14) as usize;
    let mut node = read_u32(info + 0x10) as usize;
    loop {
        if node == 0 {
            return false;
        }
        let first = read_u16(node) as u32;
        let last = read_u16(node + 0x02) as u32;
        if first <= code && code <= last {
            let glyph = platform::call3(FN_8018AEA0_GLYPH_RANGE, p32(font), p32(node), code);
            let glyph = (glyph as u16) as i16;
            if glyph != -1 {
                write_u16(info + 0x02, glyph as u16);
            }
            return glyph != -1;
        }
        node = read_u32(node + 0x08) as usize;
    }
}

// ===========================================================================
// Wave 7: pane subclasses, font layout and material helpers
// (`0x80180020..0x8018E560`)
// ===========================================================================
//
// This pass adds the pane-resource factory, the Picture/TextBox/Window
// constructor and destructor bodies, the font glyph-metric helpers and the
// per-glyph texture-record builder. It also translates the small material and
// font-texture field helpers that sit at the end of the range.
//
// The original bodies enter through `0x802DCD44..0x802DCD74`
// (`__savegprlr_*`) helpers. Those helpers only save and restore callee-saved
// registers, so the Rust bodies start at the first observable operation and
// let the compiler own the register frame. Ghidra prints the saved `r3:r4`
// pair as the helper's return value; where a body uses that pair, the doc
// states the `(this, resource)` split.

/// `0x8017DFA0` is reused from the wave-5 callee table: circular-list lookup
/// by key. Kept as an address so this section does not depend on another lane.

/// `0x802DC9A0`: initialises a small embedded callback list.
///
/// observed: five arguments `(base, entry, callback, stride, count)`; the two
/// call sites pass `0x80180770`, `0x8017E490`, `4` and `4`/`2`.
///
/// # Confidence
/// medium - argument order is visible, the helper's own body is outside this
/// lane.
const FN_802DC9A0: usize = 0x802D_C9A0;

/// `0x802DCA98`: releases the callback list built by [`FN_802DC9A0`].
///
/// # Confidence
/// medium - paired with `0x802DC9A0` at every call site.
const FN_802DCA98: usize = 0x802D_CA98;

/// `0x80180770`: code label passed as the list entry to [`FN_802DC9A0`].
///
/// # Confidence
/// medium - an address literal in the original; its role is inferred from the
/// five-argument list shape.
const LAB_80180770: u32 = 0x8018_0770;

/// `0x8017E490`: callback passed to [`FN_802DC9A0`] and [`FN_802DCA98`].
///
/// # Confidence
/// medium - address literal; body outside this lane.
const FN_8017E490: u32 = 0x8017_E490;

/// `0x8017E5D0`: initialises the embedded animation object at `+0xE4`.
///
/// # Confidence
/// medium - called once per constructor, immediately before the count setter.
const FN_8017E5D0: usize = 0x8017_E5D0;

/// `0x8017E5F0`: releases the embedded animation object at `+0xF4`.
///
/// # Confidence
/// medium - paired with [`FN_8017E5D0`] in the destructor.
const FN_8017E5F0: usize = 0x8017_E5F0;

/// `0x8017E640`: sets the animation record count.
///
/// # Confidence
/// medium - one call site per constructor, `(object, count)`.
const FN_8017E640: usize = 0x8017_E640;

/// `0x8017E7D0`: loads animation records into the object.
///
/// # Confidence
/// medium - follows [`FN_8017E640`], `(object, source, count)`.
const FN_8017E7D0: usize = 0x8017_E7D0;

/// `0x8017E870`: looks up one animation entry by id.
///
/// # Confidence
/// medium - result is passed to virtual `+0x0C`; body outside this lane.
const FN_8017E870: usize = 0x8017_E870;

/// `0x8017E920`: binds a record and a byte flag to an animation entry.
///
/// # Confidence
/// medium - two integer arguments, visible at the call site.
const FN_8017E920: usize = 0x8017_E920;

/// `0x8017EF70`: draws one glyph box.
///
/// observed: `(top_left_xy, bottom_right_xy, byte, word, handle, id)`.
///
/// # Confidence
/// medium - six integer arguments; the body is outside this lane.
const FN_8017EF70: usize = 0x8017_EF70;

/// `0x8017E380`: `bnd1` pane constructor `(object, resource, block)`.
///
/// # Confidence
/// medium - called by the pane factory with the same three operands as the
/// other pane constructors.
const FN_8017E380: usize = 0x8017_E380;

/// `0x80180780`: constructs the pane's animation object
/// `(object, resource, block)`.
///
/// # Confidence
/// medium - three arguments at every constructor call site.
const FN_80180780: usize = 0x8018_0780;

/// `0x801C7E00`: stores a six-float matrix plus a binary exponent.
///
/// # Confidence
/// medium - `(target, matrix, exponent)` read from the call site.
const FN_801C7E00: usize = 0x801C_7E00;

/// `0x801835A0`: converts two `u16` fields of a 0x20-byte record to floats.
///
/// # Confidence
/// medium - the body is not translated here; the two-float output buffer and
/// three arguments are read from `801835a0_FUN_801835a0.c`.
const FN_801835A0: usize = 0x8018_35A0;

/// `0x80188A00`: resets a material record (colours, channel counts and flags).
///
/// # Confidence
/// high - the callee writes the material fields at `+0x00..+0x48`.
const FN_80188A00: usize = 0x8018_8A00;

/// `0x8032B378`: `nw4hbm::lyt::Picture` vtable.
const VT_PICTURE: u32 = 0x8032_B378;

/// `0x8032B418`: vtable installed by the small Picture-family helper at
/// `0x80184C40`.
const VT_PICTURE_BASE: u32 = 0x8032_B418;

/// `0x8032B434`: `nw4hbm::lyt::TextBox` vtable.
const VT_TEXT_BOX: u32 = 0x8032_B434;

/// `0x8032B51C`: `nw4hbm::lyt::Window` vtable.
const VT_WINDOW: u32 = 0x8032_B51C;

/// `0x8032C8F8`: vtable installed by the small font-texture helper at
/// `0x8018B080`.
const VT_8032C8F8: u32 = 0x8032_C8F8;

/// `0x8037806C`: global word whose address is stored at material `+0x5C`.
const DAT_8037806C: u32 = 0x8037_806C;

/// `pan1`: pane resource tag.
const TAG_PAN1: u32 = 0x7061_6E31;
/// `pic1`: picture-pane resource tag.
const TAG_PIC1: u32 = 0x7069_6331;
/// `txt1`: text-box resource tag.
const TAG_TXT1: u32 = 0x7478_7431;
/// `wnd1`: window resource tag.
const TAG_WND1: u32 = 0x776E_6431;
/// `bnd1`: bounding-pane resource tag.
const TAG_BND1: u32 = 0x626E_6431;

/// `0x802F5E48`: `0.0f` (verified in `main.elf`).
const F32_802F5E48: f32 = 0.0;
/// `0x802F5E4C`: `0.5f` (verified in `main.elf`).
const F32_802F5E4C: f32 = 0.5;
/// `0x802F5E60`: `0.5f` (verified in `main.elf`).
const F32_802F5E60: f32 = 0.5;
/// `0x802F5E70`: `2.0f` (verified in `main.elf`).
const F32_802F5E70: f32 = 2.0;
/// `0x802F5E80`: `0.0f` (verified in `main.elf`).
const F32_802F5E80: f32 = 0.0;
/// `0x802F5EA8`: `0.0f` (verified in `main.elf`).
const F32_802F5EA8: f32 = 0.0;

/// Calls a six-argument integer function.
///
/// # Original
/// Host-side helper; no binary address. Models the six `r3..r8` arguments of
/// `0x8017EF70`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call6(addr: usize, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32) -> u32);
    f(a0, a1, a2, a3, a4, a5)
}

/// Applies a pane's rotation/translation record to four output floats.
///
/// When the record's byte at `+0x20` is `1`, the function builds a half-scale
/// rotated pair from the two floats at `+0x18`/`+0x1C` and writes it in
/// `(-x, +y, +x, -y)` order. Otherwise the first two outputs are zero and the
/// record's two floats are copied unchanged.
///
/// # Original
/// - Ghidra: `FUN_80180020` at `0x80180020` (108 bytes)
/// - C file: `work/ghidra/main-gb/export/80180020_FUN_80180020.c`
///
/// # Arguments
/// * `out` - four writable floats.
/// * `record` - transform record; `+0x18`/`+0x1C` are its two floats and
///   `+0x20` selects the rotated form.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 16 bytes to `out`.
///
/// # Safety
/// Both pointers must be valid; `out + 0x10` writable.
///
/// # Confidence
/// high - every load and store is visible, and the two constants were read
/// from `main.elf`.
pub unsafe fn fn_80180020(out: usize, record: usize) {
    if read_u8(record + 0x20) == 1 {
        let x = read_f32(record + 0x18);
        let y = read_f32(record + 0x1C);
        write_f32(out + 0x08, x * F32_802F5E4C);
        write_f32(out + 0x04, y * F32_802F5E4C);
        write_f32(out, -x * F32_802F5E4C);
        write_f32(out + 0x0C, -y * F32_802F5E4C);
        return;
    }
    write_f32(out, F32_802F5E48);
    write_f32(out + 0x04, F32_802F5E48);
    write_f32(out + 0x08, read_f32(record + 0x18));
    write_f32(out + 0x0C, read_f32(record + 0x1C));
}

/// Builds one pane from a resource tag and a four-word descriptor block.
///
/// The five accepted tags map to the five pane classes:
///
/// | tag | bytes | class | object size | constructor |
/// |---|---|---|---:|---|
/// | `pan1` | `0x70616E31` | pane | `0xD4` | [`fn_801836b0`] |
/// | `pic1` | `0x70696331` | picture | `0xEC` | [`fn_801846a0`] |
/// | `txt1` | `0x74787431` | text box | `0x100` | [`fn_80184c60`] |
/// | `wnd1` | `0x776E6431` | window | `0x104` | [`fn_801860b0`] |
/// | `bnd1` | `0x626E6431` | bounding | `0xD4` | `0x8017E380` |
///
/// Allocation goes through the graphics heap word at `0x80378008`; an
/// allocation failure or an unknown tag returns `0`. For every tag except
/// `pan1`, the four descriptor words are copied to the stack before the
/// constructor runs.
///
/// # Original
/// - Ghidra: `FUN_80180240` at `0x80180240` (508 bytes)
/// - C file: `work/ghidra/main-gb/export/80180240_FUN_80180240.c`
///
/// # Arguments
/// * `tag` - big-endian four-character pane tag.
/// * `resource` - pane resource passed to the constructor.
/// * `block` - four-word descriptor block (read as a by-value copy).
///
/// # Returns
/// The new pane pointer, or `0` when the tag is unknown or allocation fails.
///
/// # Side effects
/// Allocates through `0x801D5540` and runs the class constructor, which links
/// child lists and may allocate animation objects.
///
/// # Safety
/// `resource` and `block` must be valid when the tag selects a constructor.
///
/// # Confidence
/// high - the tag dispatch, sizes and constructor addresses are explicit.
pub unsafe fn fn_80180240(tag: u32, resource: usize, block: usize) -> u32 {
    let size = match tag {
        TAG_PAN1 | TAG_BND1 => 0xD4,
        TAG_PIC1 => 0xEC,
        TAG_TXT1 => 0x100,
        TAG_WND1 => 0x104,
        _ => return 0,
    };
    let object = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), size) as usize;
    if object == 0 {
        return 0;
    }

    let mut local = crate::platform::GuestLocal::new( [0u32; 4]);
    if tag != TAG_PAN1 {
        local[0] = read_u32(block);
        local[1] = read_u32(block + 0x04);
        local[2] = read_u32(block + 0x08);
        local[3] = read_u32(block + 0x0C);
    }

    match tag {
        TAG_PAN1 => {
            fn_801836b0(object, resource);
        }
        TAG_PIC1 => {
            fn_801846a0(object, resource, local.as_ptr() as usize);
        }
        TAG_TXT1 => {
            fn_80184c60(object, resource, local.as_ptr() as usize);
        }
        TAG_WND1 => {
            fn_801860b0(object, resource, local.as_ptr() as usize);
        }
        _ => {
            platform::call3(
                FN_8017E380,
                p32(object),
                p32(resource),
                local.as_ptr() as u32,
            );
        }
    }
    object as u32
}

/// Normalises a six-float matrix into the half-open range `[0.5, 1.0)`.
///
/// The function takes the absolute value of all six entries. If any entry is
/// at least `1.0`, it halves every entry (and its cached absolute value) until
/// all are below `1.0` or a sign-magnitude exponent of `0x2D` is reached. If
/// all entries are below `0.5`, it doubles them until one reaches `0.5` or the
/// exponent reaches `-0x10`. The six adjusted entries and the exponent are
/// then passed to `0x801C7E00`.
///
/// # Original
/// - Ghidra: `FUN_80180560` at `0x80180560` (520 bytes)
/// - C file: `work/ghidra/main-gb/export/80180560_FUN_80180560.c`
///
/// # Arguments
/// * `target` - matrix consumer passed to `0x801C7E00`.
/// * `matrix` - six readable `f32` entries.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801C7E00(target, adjusted_matrix, exponent)`.
///
/// # Safety
/// `matrix` must hold six floats; `target` must be acceptable to `0x801C7E00`.
///
/// # Confidence
/// high - both loops and the two constants (`0.5`, `2.0`) are explicit; the
/// exponent clamp is preserved at `0x2D`/`-0x10`.
pub unsafe fn fn_80180560(target: usize, matrix: usize) {
    let mut values = crate::platform::GuestLocal::new( [
        read_f32(matrix),
        read_f32(matrix + 0x04),
        read_f32(matrix + 0x08),
        read_f32(matrix + 0x0C),
        read_f32(matrix + 0x10),
        read_f32(matrix + 0x14),
    ]);
    let mut abs = [values[0].abs(), values[1].abs(), values[2].abs(),
        values[3].abs(), values[4].abs(), values[5].abs()];
    let mut exponent: i8 = 0;

    if abs.iter().any(|v| *v >= 1.0) {
        loop {
            if exponent > 0x2D {
                break;
            }
            for index in 0..6 {
                values[index] *= F32_802F5E60;
                abs[index] *= F32_802F5E60;
            }
            exponent += 1;
            if !abs.iter().any(|v| *v >= 1.0) {
                break;
            }
        }
    } else if abs.iter().all(|v| *v < F32_802F5E60) {
        loop {
            for index in 0..6 {
                values[index] *= F32_802F5E70;
                abs[index] *= F32_802F5E70;
            }
            exponent -= 1;
            if abs[0..5].iter().any(|v| *v >= F32_802F5E60) {
                break;
            }
            if abs[5] >= F32_802F5E60 || exponent < -0x10 {
                break;
            }
        }
    }

    platform::call3(
        FN_801C7E00,
        p32(target),
        values.as_ptr() as u32,
        exponent as i32 as u32,
    );
}

/// Forwards a zero to virtual slot `+0x14` of a pane-like object.
///
/// # Original
/// - Ghidra: `FUN_80183460` at `0x80183460` (20 bytes)
/// - C file: `work/ghidra/main-gb/export/80183460_FUN_80183460.c`
///
/// # Arguments
/// * `object` - receiver; its vtable lives at `+0x00`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call with arguments `(object, 0)`.
///
/// # Safety
/// `object` must have a readable vtable and a valid slot `+0x14`.
///
/// # Confidence
/// high - the indirect slot and the zero argument are explicit.
pub unsafe fn fn_80183460(object: usize) {
    platform::call2(vtable_method(object, 0, 0x14), p32(object), 0);
}

/// Computes a picture's flip-aware texture coordinate rectangle.
///
/// The first two outputs are the picture's flip offsets from
/// [`fn_801845a0`]. The last two add the picture's width (`+0x4C`) and height
/// (`+0x50`). When `f32[source + 0x40] - f32[source + 0x38]` is negative, the
/// two vertical components are negated.
///
/// # Original
/// - Ghidra: `FUN_80183960` at `0x80183960` (208 bytes)
/// - C file: `work/ghidra/main-gb/export/80183960_FUN_80183960.c`
///
/// # Arguments
/// * `out` - four writable floats: `(u0, v0, u1, v1)`.
/// * `picture` - picture pane read by [`fn_801845a0`].
/// * `source` - record with `f32 +0x38` and `f32 +0x40`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 16 bytes to `out`.
///
/// # Safety
/// `out` must be writable and both source objects readable.
///
/// # Confidence
/// high - the only callee is already translated and its pair order is known
/// from `801845a0_FUN_801845a0.c`.
pub unsafe fn fn_80183960(out: usize, picture: usize, source: usize) {
    write_f32(out, F32_802F5E80);
    write_f32(out + 0x04, F32_802F5E80);
    write_f32(out + 0x08, F32_802F5E80);
    write_f32(out + 0x0C, F32_802F5E80);

    let (horizontal, vertical) = fn_801845a0(picture);
    let below = read_f32(source + 0x40) - read_f32(source + 0x38) < F32_802F5E80;
    write_f32(out, horizontal);
    write_f32(out + 0x04, vertical);
    write_f32(out + 0x08, horizontal + read_f32(picture + 0x4C));
    let vertical_end = vertical + read_f32(picture + 0x50);
    write_f32(out + 0x0C, vertical_end);
    if below {
        write_f32(out + 0x04, -vertical);
        write_f32(out + 0x0C, -vertical_end);
    }
}

/// Propagates a key/value pair to a pane's child and its material.
///
/// The child list node whose `+0x08` word equals `key` is looked up through
/// `0x8017DFA0`; when found, its `+0x0E` byte is set to `1` exactly when
/// `value == 0`. The pane's material (`+0x28`) then receives virtual `+0x24`
/// with `(key, value)`. When `recurse` is non-zero, every child in the
/// circular list at `+0x14` receives virtual `+0x58` with the same three
/// operands.
///
/// # Original
/// - Ghidra: `FUN_801843d0` at `0x801843D0` (184 bytes)
/// - C file: `work/ghidra/main-gb/export/801843d0_FUN_801843d0.c`
///
/// # Arguments
/// * `pane` - pane whose child list starts at `+0x14` and material at `+0x28`.
/// * `key` - lookup key, also forwarded to the material and children.
/// * `value` - value stored as a zero flag and forwarded.
/// * `recurse` - non-zero to notify every child.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one byte and performs one virtual call per notified object.
///
/// # Safety
/// The child list and material must be valid when present.
///
/// # Confidence
/// high - `countLeadingZeros(value) >> 5` is exactly `value == 0`; the list
/// walk and the dropped `r4` operands were checked against the call sites.
pub unsafe fn fn_801843d0(pane: usize, key: u32, value: u32, recurse: i32) {
    let node = platform::call2(FN_8017DFA0_FIND, p32(pane + 0x1C), key) as usize;
    if node != 0 {
        write_u8(node + 0x0E, u8::from(value == 0));
    }

    let material = read_u32(pane + 0x28) as usize;
    if material != 0 {
        platform::call3(
            vtable_method(material, 0, 0x24),
            p32(material),
            key,
            value,
        );
    }

    if recurse != 0 {
        let sentinel = pane + 0x14;
        let mut link = read_u32(pane + 0x14) as usize;
        while link != sentinel {
            let child = link - 4;
            platform::call4(
                vtable_method(child, 0, 0x58),
                p32(child),
                key,
                value,
                recurse as u32,
            );
            link = read_u32(link) as usize;
        }
    }
}

/// Constructs a `nw4hbm::lyt::Picture` pane.
///
/// Runs the pane base constructor, installs the Picture vtable, initialises
/// the embedded animation object at `+0xD4`/`+0xE4`, copies four words from
/// the resource `+0x4C..+0x58`, and creates the pane's animation object when
/// the resource supplies one.
///
/// # Original
/// - Ghidra: `FUN_801846a0` at `0x801846A0` (296 bytes)
/// - C file: `work/ghidra/main-gb/export/801846a0_FUN_801846a0.c`
///
/// # Arguments
/// * `picture` - object to construct.
/// * `resource` - picture resource; `+0x5E` is the record count and `+0x5C`
///   the material index.
/// * `block` - four-word descriptor block whose second word is the resource
///   table and third word the accessor.
///
/// # Returns
/// `picture`, matching the constructor convention used by
/// [`fn_80180240`].
///
/// # Side effects
/// Writes the vtable and fields at `+0xD4..+0xE4`, allocates through
/// `0x801D5540`, and calls the animation initialiser chain.
///
/// # Safety
/// `picture`, `resource` and `block` must be valid; the heap must accept the
/// two allocation sizes (`0x5C` and the animation object).
///
/// # Confidence
/// medium - field stores and the animation chain are exact; the role of the
/// `0x8017E5D0`/`0x8017E640`/`0x8017E7D0` trio is inferred from its order.
pub unsafe fn fn_801846a0(picture: usize, resource: usize, block: usize) -> usize {
    fn_801836b0(picture, resource);
    write_u32(picture, VT_PICTURE);
    call5(
        FN_802DC9A0,
        p32(picture + 0xD4),
        LAB_80180770,
        FN_8017E490,
        4,
        4,
    );
    platform::call1(FN_8017E5D0, p32(picture + 0xE4));

    let mut count = 8u8;
    let resource_count = read_u8(resource + 0x5E);
    if resource_count < 9 {
        count = resource_count;
    }
    if count != 0 {
        platform::call2(FN_8017E640, p32(picture + 0xE4), count as u32);
    }

    write_u32(picture + 0xD4, read_u32(resource + 0x4C));
    write_u32(picture + 0xD8, read_u32(resource + 0x50));
    write_u32(picture + 0xDC, read_u32(resource + 0x54));
    write_u32(picture + 0xE0, read_u32(resource + 0x58));

    if count != 0 && read_u8(picture + 0xE4) != 0 {
        platform::call3(
            FN_8017E7D0,
            p32(picture + 0xE4),
            p32(resource + 0x60),
            count as u32,
        );
    }

    let object = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), 0x5C) as usize;
    if object != 0 {
        let table = read_u32(block + 0x08) as usize;
        let index = read_u16(resource + 0x5C) as usize;
        let offset = read_u32(table + index * 4 + 0x0C) as usize;
        let created = platform::call3(
            FN_80180780,
            p32(object),
            p32(table + offset),
            p32(block),
        );
        write_u32(picture + 0x28, created);
    }
    picture
}

/// Installs the small Picture-family vtable at `0x8032B418`.
///
/// # Original
/// - Ghidra: `FUN_80184c40` at `0x80184C40` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/80184c40_FUN_80184c40.c`
///
/// # Arguments
/// * `object` - receiver whose first word is the vtable slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `object + 0x00`.
///
/// # Safety
/// `object` must be writable for four bytes.
///
/// # Confidence
/// high - one store; the address is the literal in the C file.
pub unsafe fn fn_80184c40(object: usize) {
    write_u32(object, VT_PICTURE_BASE);
}

/// Constructs a `nw4hbm::lyt::TextBox` pane.
///
/// After the pane base constructor, the function installs the TextBox vtable,
/// initialises two embedded helpers at `+0xD8`/`+0xE4`, sets the glyph and
/// scale fields, optionally loads the resource font through the accessor
/// virtual `+0x0C`, and allocates the text animation object.
///
/// # Original
/// - Ghidra: `FUN_80184c60` at `0x80184C60` (692 bytes)
/// - C file: `work/ghidra/main-gb/export/80184c60_FUN_80184c60.c`
///
/// # Arguments
/// * `text_box` - object to construct.
/// * `resource` - text-box resource; `u16 +0x4C`/`+0x4E` are string lengths,
///   `u16 +0x50`/`+0x52` are table indices and `u32 +0x58..+0x70` the field
///   block.
/// * `block` - four-word descriptor block: word 1 is a table base, word 2 an
///   accessor, word 3 a resource table.
///
/// # Returns
/// `text_box`, because the factory at [`fn_80180240`] consumes the value left
/// in `r3` even though Ghidra prints the body as `void`.
///
/// # Side effects
/// Writes the vtable and many fields, performs virtual calls `+0x64` and
/// `+0x70`, allocates a 24-byte font and a 92-byte animation object, and may
/// load a `BRFNT` through [`fn_8018a730`]/[`fn_8018a7d0`].
///
/// # Safety
/// All pointers must be valid; the `0x666F6E74` (`"font"`) table lookup must
/// return either a usable font or zero.
///
/// # Confidence
/// medium - control flow and offsets are exact; the `+0xD8`/`+0xE4` embedded
/// helpers are only known by their call order.
pub unsafe fn fn_80184c60(text_box: usize, resource: usize, block: usize) -> usize {
    fn_801836b0(text_box, resource);
    write_u32(text_box, VT_TEXT_BOX);
    call5(
        FN_802DC9A0,
        p32(text_box + 0xD8),
        LAB_80180770,
        FN_8017E490,
        4,
        2,
    );

    let zero = F32_802F5EA8;
    write_u32(text_box + 0xE4, zero.to_bits());
    write_u32(text_box + 0xE8, zero.to_bits());

    let mut cells = (read_u16(resource + 0x4C) >> 1) as u32;
    if cells != 0 {
        cells -= 1;
    }

    write_u32(text_box + 0xD4, 0);
    write_u16(text_box + 0xF8, 0);
    write_u16(text_box + 0xFA, 0);
    write_u32(text_box + 0xE0, 0);

    let previous = read_u8(text_box + 0xFC) as u32;
    let step = (previous / 3) * 3 + 1;
    // observed: the original truncates `step` to a signed char before the
    // remainder correction, so `previous == 0xFF` yields `3`, not `4`.
    let low = (step & 0xFF) as u8;
    let correction = (low / 3).wrapping_mul(0xFD); // -3 * (low / 3)
    write_u8(text_box + 0xFC, low.wrapping_add(correction).wrapping_add(3));
    platform::call3(MEMSET, p32(text_box + 0xFD), 0, 1);

    write_u32(text_box + 0xE4, zero.to_bits());
    write_u32(text_box + 0xE8, zero.to_bits());
    write_u32(text_box + 0xEC, zero.to_bits());
    write_u32(text_box + 0xF0, zero.to_bits());
    write_u32(text_box + 0xF4, 0);

    if cells != 0 {
        platform::call2(vtable_method(text_box, 0, 0x64), p32(text_box), cells);
    }

    let string_count = read_u16(resource + 0x4E);
    if string_count > 1 && read_u32(text_box + 0xD4) != 0 {
        platform::call4(
            vtable_method(text_box, 0, 0x70),
            p32(text_box),
            p32(resource + read_u32(resource + 0x58) as usize),
            0,
            (string_count >> 1) as u32 - 1,
        );
    }

    write_u32(text_box + 0xD8, read_u32(resource + 0x5C));
    write_u32(text_box + 0xDC, read_u32(resource + 0x60));
    write_u32(text_box + 0xE4, read_u32(resource + 0x64));
    write_u32(text_box + 0xE8, read_u32(resource + 0x68));
    write_u8(text_box + 0xFC, read_u8(resource + 0x54));
    write_u32(text_box + 0xF0, read_u32(resource + 0x6C));
    write_u32(text_box + 0xEC, read_u32(resource + 0x70));

    let accessor = read_u32(block + 0x0C) as usize;
    let base = read_u32(block + 0x04) as usize + 0x0C;
    let entry = base + read_u32(base + read_u16(resource + 0x52) as usize * 8) as usize;
    let found = platform::call2(vtable_method(accessor, 0, 0x10), p32(accessor), p32(entry)) as usize;
    if found == 0 {
        let font_data = platform::call4(
            vtable_method(accessor, 0, 0x0C),
            p32(accessor),
            0x666F_6E74,
            p32(entry),
            0,
        ) as usize;
        if font_data != 0 {
            let memory = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), 0x18) as usize;
            if memory != 0 {
                let font = fn_8018a730(memory);
                fn_8018a7d0(font, font_data);
                write_u32(text_box + 0xE0, font as u32);
                write_u8(text_box + 0xFD, read_u8(text_box + 0xFD) | 0x80);
            }
        }
    } else {
        write_u32(text_box + 0xE0, found as u32);
    }

    let object = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), 0x5C) as usize;
    if object != 0 {
        let table = read_u32(block + 0x08) as usize;
        let index = read_u16(resource + 0x50) as usize;
        let offset = read_u32(table + index * 4 + 0x0C) as usize;
        let created = platform::call3(
            FN_80180780,
            p32(object),
            p32(table + offset),
            p32(block),
        );
        write_u32(text_box + 0x28, created);
    }
    text_box
}

/// Constructs a `nw4hbm::lyt::Window` pane.
///
/// The window constructor installs the Window vtable, copies the resource
/// field block, builds the embedded animation object at `+0xE4`/`+0xF4`,
/// creates the text animation object, and fills an 8-byte glyph table from
/// the resource's index chain.
///
/// # Original
/// - Ghidra: `FUN_801860b0` at `0x801860B0` (528 bytes)
/// - C file: `work/ghidra/main-gb/export/801860b0_FUN_801860b0.c`
///
/// # Arguments
/// * `window` - object to construct (the original names this `param_1`).
/// * `resource` - window resource; `+0x60` is a record offset and `+0x5C` a
///   glyph count.
/// * `block` - four-word descriptor block: word 1 a table base, word 2 an
///   accessor, word 3 a resource table.
///
/// # Returns
/// `window`, following the same `r3` convention as [`fn_80184c60`].
///
/// # Side effects
/// Writes the vtable and fields, allocates the animation objects and the
/// glyph table, and calls the animation initialiser chain.
///
/// # Safety
/// All pointers must be valid; the two allocations must succeed for the
/// optional paths to run.
///
/// # Confidence
/// medium - field stores and loops are exact; the `+0xE4`/`+0xF4` helper
/// roles are inferred from their order.
pub unsafe fn fn_801860b0(window: usize, resource: usize, block: usize) -> usize {
    fn_801836b0(window, resource);
    write_u32(window, VT_WINDOW);
    call5(
        FN_802DC9A0,
        p32(window + 0xE4),
        LAB_80180770,
        FN_8017E490,
        4,
        4,
    );
    platform::call1(FN_8017E5D0, p32(window + 0xF4));

    write_u32(window + 0xD4, read_u32(resource + 0x4C));
    write_u32(window + 0xD8, read_u32(resource + 0x50));
    write_u32(window + 0xDC, read_u32(resource + 0x54));
    write_u32(window + 0xE0, read_u32(resource + 0x58));

    let header = resource + read_u32(resource + 0x60) as usize;
    write_u32(window + 0xE4, read_u32(header));
    write_u32(window + 0xE8, read_u32(header + 0x04));
    write_u32(window + 0xEC, read_u32(header + 0x08));
    write_u32(window + 0xF0, read_u32(header + 0x0C));

    let record_count = read_u8(header + 0x12);
    if record_count != 0 {
        let count = record_count.min(8);
        platform::call2(FN_8017E640, p32(window + 0xF4), count as u32);
        if read_u8(window + 0xF4) != 0 {
            platform::call3(
                FN_8017E7D0,
                p32(window + 0xF4),
                p32(header + 0x14),
                count as u32,
            );
        }
    }

    let table = read_u32(block + 0x08) as usize;
    let base = table + 0x0C;
    let object = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), 0x5C) as usize;
    if object != 0 {
        let index = read_u16(header + 0x10) as usize;
        let offset = read_u32(base + index * 4) as usize;
        let created = platform::call3(
            FN_80180780,
            p32(object),
            p32(table + offset),
            p32(block),
        );
        write_u32(window + 0x28, created);
    }

    write_u8(window + 0x100, 0);
    write_u32(window + 0xFC, 0);

    let glyph_count = read_u8(resource + 0x5C);
    if glyph_count != 0 {
        let glyphs = platform::call2(
            FN_801D5540_ALLOC,
            read_u32(HEAP_80378008),
            (glyph_count as u32) << 3,
        ) as usize;
        write_u32(window + 0xFC, glyphs as u32);
        if glyphs != 0 {
            write_u8(window + 0x100, glyph_count);
            let mut index = 0u32;
            let mut offset = 0usize;
            let mut link = resource + read_u32(resource + 0x64) as usize;
            while index < read_u8(window + 0x100) as u32 {
                let code = read_u32(link) as usize;
                write_u8(glyphs + offset, read_u8(resource + code + 0x02));
                write_u32(glyphs + offset + 0x04, 0);
                let glyph_object = platform::call2(
                    FN_801D5540_ALLOC,
                    read_u32(HEAP_80378008),
                    0x5C,
                ) as usize;
                if glyph_object != 0 {
                    let table_index = read_u16(resource + code) as usize;
                    let table_offset = read_u32(base + table_index * 4) as usize;
                    let created = platform::call3(
                        FN_80180780,
                        p32(glyph_object),
                        p32(table + table_offset),
                        p32(block),
                    );
                    write_u32(glyphs + offset + 0x04, created);
                }
                link += 4;
                offset += 8;
                index += 1;
            }
        }
    }
    window
}

/// Destroys a `nw4hbm::lyt::Window` pane.
///
/// Releases every glyph object and the glyph table at `+0xFC`, then the text
/// animation object at `+0x28`, the embedded animation object at `+0xF4`, the
/// callback list at `+0xE4`, and finally runs the pane base destructor. When
/// `delete_flag` is positive the object itself is released through
/// `0x802B9280`.
///
/// # Original
/// - Ghidra: `FUN_801862c0` at `0x801862C0` (324 bytes)
/// - C file: `work/ghidra/main-gb/export/801862c0_FUN_801862c0.c`
///
/// # Arguments
/// * `window` - object to destroy; the original exits through the
///   `0x802DCD74` prologue helper with `this` in the high return word.
/// * `delete_flag` - positive also frees `window`.
///
/// # Returns
/// `window`, matching the destructor convention of [`fn_801837d0`].
///
/// # Side effects
/// Frees the glyph table and its entries, frees two animation objects, tears
/// down the callback list, and may free `window`.
///
/// # Safety
/// The object and every stored child pointer must be live allocations.
///
/// # Confidence
/// high - every free and virtual call is explicit; the saved-register pair is
/// the only reason the Ghidra signature is `void`.
pub unsafe fn fn_801862c0(window: usize, delete_flag: i32) -> usize {
    if window != 0 {
        write_u32(window, VT_WINDOW);

        let glyphs = read_u32(window + 0xFC) as usize;
        if glyphs != 0 {
            let mut offset = 0usize;
            let count = read_u8(window + 0x100) as usize;
            let mut index = 0usize;
            while index < count {
                let glyph = read_u32(glyphs + offset + 0x04) as usize;
                platform::call2(vtable_method(glyph, 0, 0x08), p32(glyph), 0xFFFF_FFFF);
                platform::call2(
                    FN_801D5550_FREE,
                    read_u32(HEAP_80378008),
                    read_u32(glyphs + offset + 0x04),
                );
                offset += 8;
                index += 1;
            }
            platform::call2(
                FN_801D5550_FREE,
                read_u32(HEAP_80378008),
                read_u32(window + 0xFC),
            );
        }

        let text_object = read_u32(window + 0x28) as usize;
        if text_object != 0 && read_u8(text_object + 0x54) == 0 {
            platform::call2(
                vtable_method(text_object, 0, 0x08),
                p32(text_object),
                0xFFFF_FFFF,
            );
            platform::call2(
                FN_801D5550_FREE,
                read_u32(HEAP_80378008),
                read_u32(window + 0x28),
            );
            write_u32(window + 0x28, 0);
        }

        platform::call1(FN_8017E5F0, p32(window + 0xF4));
        platform::call4(
            FN_802DCA98,
            p32(window + 0xE4),
            FN_8017E490,
            4,
            4,
        );
        fn_801837d0(window, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(window));
        }
    }
    window
}

/// Draws one glyph box for a text pane.
///
/// The function resolves the animation entry through `0x8017E870`, asks the
/// pane's material (`+0x28`) for the matching object through virtual `+0x0C`,
/// binds the byte at `+0xF5` with `0x8017E920`, and calls `0x8017EF70` with two
/// computed corner pairs: the box from the pane's `+0x4C`/`+0x50` sizes and
/// the box from the two offsets passed by the caller.
///
/// # Original
/// - Ghidra: `FUN_80186950` at `0x80186950` (268 bytes)
/// - C file: `work/ghidra/main-gb/export/80186950_FUN_80186950.c`
///
/// # Arguments
/// * `text_box` - text pane whose material is at `+0x28`.
/// * `offsets` - two readable floats (the glyph origin).
/// * `rect` - four readable floats `(x0, y0, x1, y1)` of the glyph box.
/// * `id` - animation/character id forwarded to the material.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls and one six-argument call that emits the glyph quad.
///
/// # Safety
/// `offsets` and `rect` must be readable; the material vtable must be valid.
///
/// # Confidence
/// medium - the arithmetic is exact; the `r3:r4` prologue return supplies the
/// second argument as a pointer to two floats.
pub unsafe fn fn_80186950(text_box: usize, offsets: usize, rect: usize, id: u32) {
    let entry = platform::call2(FN_8017E870, p32(text_box + 0xE4), id);
    let material = read_u32(text_box + 0x28) as usize;
    let found = platform::call3(
        vtable_method(material, 0, 0x0C),
        p32(material),
        entry,
        id,
    ) as usize;
    platform::call2(FN_8017E920, found as u32, read_u8(text_box + 0xF5) as u32);

    let handle = if found == 0 { 0 } else { text_box + 0xE4 };

    let a_y = read_f32(text_box + 0xE0)
        + ((read_f32(text_box + 0xDC) + (read_f32(text_box + 0x50) - read_f32(rect + 0x08)))
            - read_f32(rect + 0x0C));
    let b_y = (read_f32(offsets + 0x04) + read_f32(rect + 0x08)) - read_f32(text_box + 0xDC);
    let a_x = read_f32(text_box + 0xD8)
        + ((read_f32(text_box + 0xD4) + (read_f32(text_box + 0x4C) - read_f32(rect)))
            - read_f32(rect + 0x04));
    let b_x = (read_f32(offsets) + read_f32(rect)) - read_f32(text_box + 0xD4);

    let pair_a = crate::platform::GuestLocal::new( [a_x, a_y]);
    let pair_b = crate::platform::GuestLocal::new( [b_x, b_y]);
    call6(
        FN_8017EF70,
        p32(pair_b.as_ptr() as usize),
        p32(pair_a.as_ptr() as usize),
        read_u8(text_box + 0xF5) as u32,
        read_u32(text_box + 0xF8),
        handle as u32,
        id,
    );
}

/// Selects two texture-coordinate source rectangles for a picture pane.
///
/// For `kind == 1`, both corners come from the record at `block + 0x04`; the
/// output is `(x, x, y, y)`. For `kind == 4` or `8`, the first corner comes
/// from `block + 0x04` and the second from `block + 0x1C`. Any other kind
/// leaves four zero floats. Each corner is produced by `0x801835A0`, which
/// converts two `u16` fields of a 0x20-byte record.
///
/// # Original
/// - Ghidra: `FUN_80188440` at `0x80188440` (248 bytes)
/// - C file: `work/ghidra/main-gb/export/80188440_FUN_80188440.c`
///
/// # Arguments
/// * `out` - four writable floats.
/// * `_unused` - unused second argument (present in the original signature).
/// * `kind` - selector `1`, `4` or `8`.
/// * `block` - record block; word 1 at `+0x04`, second record at `+0x1C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 16 bytes to `out` and calls `0x801835A0` up to twice.
///
/// # Safety
/// `out` must be writable; `block` must hold readable pointers for the
/// selected kind.
///
/// # Confidence
/// medium - the control flow is exact; `0x801835A0`'s GQR-scaled internals
/// are not translated and are called by address.
pub unsafe fn fn_80188440(out: usize, _unused: u32, kind: u32, block: usize) {
    let mut first = crate::platform::GuestLocal::new( [0f32; 2]);
    let mut second = crate::platform::GuestLocal::new( [0f32; 2]);
    let mut local = [F32_802F5E80; 4];

    if kind == 1 {
        platform::call3(
            FN_801835A0,
            first.as_mut_ptr() as u32,
            read_u32(block + 0x04),
            0,
        );
        local[0] = first[0];
        local[2] = first[1];
        local[1] = first[0];
        local[3] = first[1];
    } else if kind == 4 || kind == 8 {
        platform::call3(
            FN_801835A0,
            first.as_mut_ptr() as u32,
            read_u32(block + 0x04),
            0,
        );
        local[0] = first[0];
        local[2] = first[1];
        platform::call3(
            FN_801835A0,
            second.as_mut_ptr() as u32,
            read_u32(block + 0x1C),
            0,
        );
        local[1] = second[0];
        local[3] = second[1];
    }

    write_f32(out, local[0]);
    write_f32(out + 0x04, local[1]);
    write_f32(out + 0x08, local[2]);
    write_f32(out + 0x0C, local[3]);
}

/// Copies two four-byte colour values into an eight-byte record.
///
/// The original loads all eight source bytes before any store, so aliasing
/// between `out` and either source is preserved here.
///
/// # Original
/// - Ghidra: `FUN_80189740` at `0x80189740` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/80189740_FUN_80189740.c`
///
/// # Arguments
/// * `out` - eight writable bytes.
/// * `first` - four readable bytes.
/// * `second` - four readable bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight bytes.
///
/// # Safety
/// `out + 8` must be writable and both sources readable.
///
/// # Confidence
/// high - the eight loads and stores are explicit.
pub unsafe fn fn_80189740(out: usize, first: usize, second: usize) {
    let mut bytes = [0u8; 8];
    for index in 0..4 {
        bytes[index] = read_u8(first + index);
        bytes[4 + index] = read_u8(second + index);
    }
    for index in 0..8 {
        write_u8(out + index, bytes[index]);
    }
}

/// Rebuilds four colour words from two source colour blocks and a selector.
///
/// `selector` is stored at `+0x20`; the source block for the first word is
/// `+0x1C` when the selector is `1`, the second is `+0x1C` when the selector
/// is `2`, and the third is `+0x18` when the selector is `0`. The remaining
/// branch uses `+0x18` (first two) or `+0x1C` (third). Finally the four alpha
/// bytes at `+0x0B`, `+0x0F`, `+0x13` and `+0x17` are scaled by the byte at
/// `+0x42` over `0xFF`.
///
/// # Original
/// - Ghidra: `FUN_80189790` at `0x80189790` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/80189790_FUN_80189790.c`
///
/// # Arguments
/// * `config` - colour selection object.
/// * `selector` - `0`, `1`, `2` or any other value choosing the fallback.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x08..+0x20` of `config`.
///
/// # Safety
/// `config + 0x42` must be readable and `config + 0x08..+0x20` writable.
///
/// # Confidence
/// high - every branch and byte store is explicit.
pub unsafe fn fn_80189790(config: usize, selector: i32) {
    write_u32(config + 0x20, selector as u32);
    for index in 0..4 {
        write_u8(config + 0x08 + index, read_u8(config + 0x18 + index));
    }

    let first = if selector == 1 { config + 0x1C } else { config + 0x18 };
    for index in 0..4 {
        write_u8(config + 0x0C + index, read_u8(first + index));
    }

    let second = if selector == 2 { config + 0x1C } else { config + 0x18 };
    for index in 0..4 {
        write_u8(config + 0x10 + index, read_u8(second + index));
    }

    let third = if selector == 0 { config + 0x18 } else { config + 0x1C };
    for index in 0..4 {
        write_u8(config + 0x14 + index, read_u8(third + index));
    }

    let alpha = read_u8(config + 0x42) as u32;
    for offset in [0x0B, 0x0F, 0x13, 0x17] {
        write_u8(
            config + offset,
            ((read_u8(config + offset) as u32 * alpha) / 0xFF) as u8,
        );
    }
}

/// Returns the horizontal advance stored at `record + 0x24`, scaled by the
/// font object's virtual `+0x0C` value.
///
/// # Original
/// - Ghidra: `FUN_80189b50` at `0x80189B50` (100 bytes)
/// - C file: `work/ghidra/main-gb/export/80189b50_FUN_80189b50.c`
///
/// # Arguments
/// * `record` - text-layout record; `+0x48` holds the font object and `+0x24`
///   the scale.
///
/// # Returns
/// `f32[+0x24] * (font->+0x0C())` widened to `f64`.
///
/// # Side effects
/// One virtual call on the font object.
///
/// # Safety
/// `record + 0x48` must hold a font object with a valid vtable.
///
/// # Confidence
/// medium - Ghidra drops the virtual-call operand; `r3` still holds the font
/// object, which is the standard PPC member-call convention.
pub unsafe fn fn_80189b50(record: usize) -> f64 {
    let font = read_u32(record + 0x48) as usize;
    let value = platform::call1(vtable_method(font, 0, 0x0C), p32(font));
    (read_f32(record + 0x24) * (value as f32)) as f64
}

/// Returns the vertical advance stored at `record + 0x28`, scaled by the font
/// object's virtual `+0x10` value.
///
/// # Original
/// - Ghidra: `FUN_80189bc0` at `0x80189BC0` (100 bytes)
/// - C file: `work/ghidra/main-gb/export/80189bc0_FUN_80189bc0.c`
///
/// # Arguments
/// * `record` - text-layout record; `+0x48` holds the font object and `+0x28`
///   the scale.
///
/// # Returns
/// `f32[+0x28] * (font->+0x10())` widened to `f64`.
///
/// # Side effects
/// One virtual call on the font object.
///
/// # Safety
/// `record + 0x48` must hold a font object with a valid vtable.
///
/// # Confidence
/// medium - same dropped-operand situation as [`fn_80189b50`].
pub unsafe fn fn_80189bc0(record: usize) -> f64 {
    let font = read_u32(record + 0x48) as usize;
    let value = platform::call1(vtable_method(font, 0, 0x10), p32(font));
    (read_f32(record + 0x28) * (value as f32)) as f64
}

/// Returns the third metric stored at `record + 0x28`, scaled by the font
/// object's virtual `+0x14` value.
///
/// # Original
/// - Ghidra: `FUN_80189c30` at `0x80189C30` (100 bytes)
/// - C file: `work/ghidra/main-gb/export/80189c30_FUN_80189c30.c`
///
/// # Arguments
/// * `record` - text-layout record; `+0x48` holds the font object and `+0x28`
///   the scale.
///
/// # Returns
/// `f32[+0x28] * (font->+0x14())` widened to `f64`.
///
/// # Side effects
/// One virtual call on the font object.
///
/// # Safety
/// `record + 0x48` must hold a font object with a valid vtable.
///
/// # Confidence
/// medium - same dropped-operand situation as [`fn_80189b50`].
pub unsafe fn fn_80189c30(record: usize) -> f64 {
    let font = read_u32(record + 0x48) as usize;
    let value = platform::call1(vtable_method(font, 0, 0x14), p32(font));
    (read_f32(record + 0x28) * (value as f32)) as f64
}

/// Adds a `f64` delta to the `f32` at `record + 0x2C`.
///
/// # Original
/// - Ghidra: `FUN_80189e30` at `0x80189E30` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e30_FUN_80189e30.c`
///
/// # Arguments
/// * `delta` - added in double precision, then narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `record + 0x2C`.
///
/// # Safety
/// `record + 0x2C` must be writable.
///
/// # Confidence
/// high - the add-and-narrow sequence is explicit.
pub unsafe fn fn_80189e30(delta: f64, record: usize) {
    write_f32(record + 0x2C, (read_f32(record + 0x2C) as f64 + delta) as f32);
}

/// Adds a `f64` delta to the `f32` at `record + 0x30`.
///
/// # Original
/// - Ghidra: `FUN_80189e40` at `0x80189E40` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e40_FUN_80189e40.c`
///
/// # Arguments
/// * `delta` - added in double precision, then narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `record + 0x30`.
///
/// # Safety
/// `record + 0x30` must be writable.
///
/// # Confidence
/// high - the add-and-narrow sequence is explicit.
pub unsafe fn fn_80189e40(delta: f64, record: usize) {
    write_f32(record + 0x30, (read_f32(record + 0x30) as f64 + delta) as f32);
}

/// Maps a character code to a glyph index through one `FINF` range record.
///
/// Three modes are stored in the `u16` at `range + 0x04`:
///
/// | mode | lookup |
/// |---|---|
/// | `0` | `u16[range + 0x0C] + (code - u16[range])`, truncated to 16 bits |
/// | `1` | direct `u16` table indexed from `range + 0x0C` |
/// | `2` | binary search over 4-byte `{first, last, glyph}` records |
///
/// Any other mode returns `0xFFFF`. The binary search walks the record array
/// from `range + 0x0E` to `range + 0x0E + 4 * (count - 1)` and returns the
/// glyph word of the matching record.
///
/// # Original
/// - Ghidra: `FUN_8018aea0` at `0x8018AEA0` (204 bytes)
/// - C file: `work/ghidra/main-gb/export/8018aea0_FUN_8018aea0.c`
///
/// # Arguments
/// * `_owner` - unused first argument.
/// * `range` - `FINF` glyph-range record.
/// * `code` - character code to map.
///
/// # Returns
/// Glyph index, or `0xFFFF` when the range does not contain the code.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The range record and its direct/binary tables must be readable for the
/// mode selected by `+0x04`.
///
/// # Confidence
/// high - all three modes and the binary-search bounds are explicit; the
/// signed-division correction in the midpoint computation is preserved.
pub unsafe fn fn_8018aea0(_owner: u32, range: usize, code: u32) -> u32 {
    let mode = read_u16(range + 0x04);
    if mode == 0 {
        return (read_u16(range + 0x0C) as u32)
            .wrapping_add(code.wrapping_sub(read_u16(range) as u32))
            & 0xFFFF;
    }
    if mode == 1 {
        let index = code.wrapping_sub(read_u16(range) as u32);
        return read_u16(range.wrapping_add((index as usize).wrapping_add(6).wrapping_mul(2))) as u32;
    }
    if mode != 2 {
        return 0xFFFF;
    }

    let count = read_u16(range + 0x0C) as i32;
    let mut low = range + 0x0E;
    let mut high = (range as isize + 0x0E + (count as isize - 1) * 4) as usize;
    loop {
        if high < low {
            return 0xFFFF;
        }
        let diff = (high as i32).wrapping_sub(low as i32);
        let mut quotient = diff >> 2;
        if diff < 0 && (diff & 3) != 0 {
            quotient += 1;
        }
        let step = (quotient.wrapping_sub(quotient >> 31).wrapping_mul(2) as u32) & 0xFFFF_FFFC;
        let middle = low.wrapping_add(step as usize);

        if code <= read_u16(middle) as u32 {
            if read_u16(middle) as u32 <= code {
                return read_u16(middle + 0x02) as u32;
            }
            high = middle.wrapping_sub(4);
        } else {
            low = middle.wrapping_add(4);
        }
    }
}

/// Builds one 0x14-byte texture record for a glyph.
///
/// The glyph index selects a sheet and cell from the `TGLP` section
/// (`FINF + 0x08`): `cell_width * cell_height` cells per sheet, `sheet` and a
/// row/column pair are derived from `code`. The character-width record comes
/// from the `CWDH` chain (`FINF + 0x0C`) or the default three bytes at
/// `FINF + 0x04`. The function fills the image sheet offset, the three
/// character bytes, the filter byte, the sheet width/height, and the cell
/// origin and extent words.
///
/// # Original
/// - Ghidra: `FUN_8018af70` at `0x8018AF70` (260 bytes)
/// - C file: `work/ghidra/main-gb/export/8018af70_FUN_8018af70.c`
///
/// # Arguments
/// * `font` - `ResFont`-like object whose `+0x14` is the `FINF` payload.
/// * `out` - 0x14-byte destination record.
/// * `code` - glyph character code.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the 0x14-byte record at `out`.
///
/// # Safety
/// `font`'s `FINF` payload must be loaded; `TGLP` cell dimensions must be
/// non-zero (the original divides by them).
///
/// # Confidence
/// high - the section offsets, stride and the `u16` truncations are explicit;
/// the meaning of the record fields is inferred from the `TGLP`/`CWDH` layout.
pub unsafe fn fn_8018af70(font: usize, out: usize, code: u32) {
    let info = read_u32(font + 0x14) as usize;
    let tglp = read_u32(info + 0x08) as usize;
    let cell_width = read_u16(tglp + 0x0C) as u32;
    let columns = read_u8(tglp) as u32;
    let total = cell_width.wrapping_mul(read_u16(tglp + 0x0E) as u32);
    let sheet = code / total;
    let remainder = code.wrapping_sub(sheet.wrapping_mul(total));
    let row = (remainder / cell_width) as u16 as i16;

    write_u32(
        out,
        read_u32(tglp + 0x14).wrapping_add(sheet.wrapping_mul(read_u32(tglp + 0x04))),
    );

    let mut cwdh = read_u32(info + 0x0C) as usize;
    let record;
    loop {
        if cwdh == 0 {
            record = info + 0x04;
            break;
        }
        let first = read_u16(cwdh) as u32;
        let last = read_u16(cwdh + 0x02) as u32;
        if first <= code && code <= last {
            record = cwdh + (code - first) as usize * 3 + 0x08;
            break;
        }
        // observed: the chain link is at +0x04 (lwz r6,0x4(r6)); the +0x08 is the record offset.
        cwdh = read_u32(cwdh + 0x04) as usize;
    }

    write_u8(out + 0x04, read_u8(record));
    write_u8(out + 0x05, read_u8(record + 0x01));
    write_u8(out + 0x06, read_u8(record + 0x02));
    write_u8(out + 0x07, read_u8(tglp + 0x01));
    write_u32(out + 0x08, read_u16(tglp + 0x0A) as u32);
    write_u16(out + 0x0C, read_u16(tglp + 0x10));
    write_u16(out + 0x0E, read_u16(tglp + 0x12));

    let remainder_short = remainder as u16 as i16 as i32;
    let horizontal = remainder_short - row as i32 * cell_width as i32;
    write_u16(
        out + 0x10,
        (horizontal * (columns as i32 + 1) + 1) as u16,
    );
    write_u16(
        out + 0x12,
        (row as i32 * (read_u8(tglp + 0x01) as i32 + 1) + 1) as u16,
    );
}

/// Installs the vtable at `0x8032C8F8` on a font-texture object.
///
/// # Original
/// - Ghidra: `FUN_8018b080` at `0x8018B080` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b080_FUN_8018b080.c`
///
/// # Arguments
/// * `object` - receiver whose first word is the vtable slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `object + 0x00`.
///
/// # Safety
/// `object` must be writable for four bytes.
///
/// # Confidence
/// high - one store; the table address is the literal in the C file.
pub unsafe fn fn_8018b080(object: usize) {
    write_u32(object, VT_8032C8F8);
}

/// Releases a font-texture object when the delete flag is positive.
///
/// # Original
/// - Ghidra: `FUN_8018b090` at `0x8018B090` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b090_FUN_8018b090.c`
///
/// # Arguments
/// * `object` - object to release.
/// * `delete_flag` - positive releases `object`.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May call `0x802B9280` (release).
///
/// # Safety
/// `object` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - the guard and the single call are explicit.
pub unsafe fn fn_8018b090(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(FN_802B9280, p32(object));
    }
    object
}

/// Resets a material record and initialises its texture-matrix fields.
///
/// Runs the material reset at `0x80188A00`, then stores two zero floats at
/// `+0x4C`/`+0x50`, the channel count `4` at `+0x54`, zero at `+0x58`, and the
/// address of the global word at `0x8037806C` at `+0x5C`.
///
/// # Original
/// - Ghidra: `FUN_8018e500` at `0x8018E500` (92 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e500_FUN_8018e500.c`
///
/// # Arguments
/// * `material` - material record to construct.
///
/// # Returns
/// `material`, mirroring the original `return param_1`.
///
/// # Side effects
/// Clears colours and flags through `0x80188A00`, then writes six fields.
///
/// # Safety
/// `material` must be writable for the full material layout.
///
/// # Confidence
/// medium - the reset callee's internals are outside this lane; the six field
/// stores are literal.
pub unsafe fn fn_8018e500(material: usize) -> usize {
    platform::call1(FN_80188A00, p32(material));
    write_u32(material + 0x4C, 0);
    write_u32(material + 0x50, 0);
    write_u32(material + 0x54, 4);
    write_u32(material + 0x58, 0);
    write_u32(material + 0x5C, DAT_8037806C);
    material
}

// ===========================================================================
// Wave 10: material colours, font metrics and animation key records
// (`0x80180440..0x8018E660`)
// ===========================================================================
//
// This pass continues the NW4R `lyt` band with the bodies that sit between the
// wave-7 pane/font work and the animation key-record tables:
//
// * **pane transforms** - [`fn_80180440`] turns a rotation/scale record into a
//   3x4 affine matrix and [`fn_801835a0`] reads one texture-SRT entry.
// * **material colours** - [`fn_80188a00`] resets the full material colour
//   block and [`fn_801898e0`] selects the four colour channels used by text.
// * **font metrics** - [`fn_80189a80`] installs the horizontal/vertical scales,
//   [`fn_8018bca0`] and [`fn_8018e660`] report a line height, and the tiny
//   accessors at `0x80188DD0..0x80189E60` get or set one field each.
// * **animation key records** - [`fn_8018b0d0`], [`fn_8018b630`],
//   [`fn_8018b4e0`] and [`fn_8018ba40`] apply one animation key record for
//   modes `9`/`10`, and [`fn_8018b5e0`] installs the key-record vtable.
// * **release wrappers** - [`fn_80188d90`], [`fn_8018b5f0`] and
//   [`fn_8018e560`] are the conditional-release shapes used by the animation
//   and material classes.
//
// The Ghidra exports of the animation bodies open with `unaff_GQR0` and
// `ldexpf(...)` lines. Those are paired-single register spills and
// integer-to-float quantisation, not program logic; the translations keep the
// scalar arithmetic and drop only that decompiler noise. No memory effect is
// lost: the GQR blocks never touch target memory.

/// `0x801885B0`: sine-like table lookup used by [`fn_80180440`].
///
/// observed: one `f64` angle argument and one `f64` result, with the table at
/// `0x8032B5C8`/`0x8032B5D0`. The export is GQR-affected, so the body is kept
/// opaque here.
///
/// # Confidence
/// medium - call shape and table base are visible, the body is not translated.
const FN_801885B0: usize = 0x8018_85B0;

/// `0x80188630`: cosine-like table lookup used by [`fn_80180440`].
///
/// observed: same shape as [`FN_801885B0`] but reads the adjacent table at
/// `0x8032B5CC`/`0x8032B5D4` and returns without the final sign flip.
///
/// # Confidence
/// medium - see [`FN_801885B0`].
const FN_80188630: usize = 0x8018_8630;

/// `0x8018BD80`: returns the animation record's duration word at `+0x54`.
///
/// # Confidence
/// high - one load; the body is a two-instruction accessor.
const FN_8018BD80: usize = 0x8018_BD80;

/// `0x8018E740`: duplicate of [`FN_8018BD80`] used by the second key handler.
///
/// # Confidence
/// high - byte-for-byte behaviour; only the address differs.
const FN_8018E740: usize = 0x8018_E740;

/// `0x801C7200`: returns the height word of a palette-converted texture record.
///
/// # Confidence
/// medium - called immediately after `0x801C7160`; role inferred from the
/// paired width accessor.
const FN_801C7200: usize = 0x801C_7200;

/// `0x801C71E0`: returns the width word of a palette-converted texture record.
///
/// # Confidence
/// medium - see [`FN_801C7200`].
const FN_801C71E0: usize = 0x801C_71E0;

/// `0x8032C8B0`: vtable installed by the key-record constructor at
/// `0x8018B5E0`.
const VT_8032C8B0: u32 = 0x8032_C8B0;

/// `0x80378038`: material-manager busy byte cleared by [`fn_80188a00`].
const DAT_80378038: usize = 0x8037_8038;

/// `0x8037803C`: material-manager current-material word cleared by
/// [`fn_80188a00`].
const DAT_8037803C: usize = 0x8037_803C;

/// `0x802F5E64`: `0.7111111f`, the angle scale used by [`fn_80180440`].
///
/// Exact bits `0x3F360B61`, read from `main.elf`.
const F32_802F5E64: f32 = f32::from_bits(0x3F36_0B61);

/// `0x802F5E68`: `0.0f`, read from `main.elf`.
const F32_802F5E68: f32 = 0.0;

/// `0x802F5E6C`: `1.0f`, read from `main.elf`.
const F32_802F5E6C: f32 = 1.0;

/// `0x802F5F00`: `0.0f`, read from `main.elf`.
const F32_802F5F00: f32 = 0.0;

/// `0x802F5F04`: `1.0f`, read from `main.elf`.
const F32_802F5F04: f32 = 1.0;

/// Calls a one-argument `f64` function on the target.
///
/// # Original
/// Host-side helper; no binary address. Models the `f1 -> f1` ABI used by the
/// rotation-table functions at `0x801885B0`/`0x80188630`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call_f64_f64(addr: usize, value: f64) -> f64 {
    let f = crate::platform::guest_fn!(addr, fn(f64) -> f64);
    f(value)
}

/// Builds the 3x4 affine matrix for one pane rotation/scale record.
///
/// The first two record floats are the translation, the third is an angle
/// (scaled by `0.7111111` before the sine/cosine lookup) and the last two are
/// the X/Y scales. The 3x3 part is `[cos*sx, -sin*sy, 0; sin*sx, cos*sy, 0;
/// 0, 0, 1]` with the half-texel `-0.5` offsets folded into the translation
/// column, exactly as the original stores them.
///
/// # Original
/// - Ghidra: `FUN_80180440` at `0x80180440` (280 bytes)
/// - C file: `work/ghidra/main-gb/export/80180440_FUN_80180440.c`
///
/// # Arguments
/// * `out` - twelve writable `f32` slots (a 3x4 matrix).
/// * `record` - pane transform record:
///   `+0x00`/`+0x04` translation, `+0x08` angle, `+0x0C`/`+0x10` scales.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 48 bytes at `out`.
///
/// # Safety
/// `out + 0x00..0x30` must be writable and `record + 0x00..0x14` readable.
///
/// # Confidence
/// medium - the scalar matrix arithmetic is exact; the two GQR-affected
/// rotation-table callees stay opaque and are called by address.
pub unsafe fn fn_80180440(out: usize, record: usize) {
    let angle = F32_802F5E64 * read_f32(record + 0x08);
    let cosine = call_f64_f64(FN_80188630, angle as f64);
    let sine = call_f64_f64(FN_801885B0, angle as f64);

    let scale_x = read_f32(record + 0x0C) as f64;
    let scale_y = read_f32(record + 0x10) as f64;
    let first = (sine * scale_x) as f32;
    let second = (cosine * scale_x) as f32;
    let third = (-sine * scale_y) as f32;
    let fourth = (cosine * scale_y) as f32;

    let translation_x = read_f32(record);
    let translation_y = read_f32(record + 0x04);
    let half = F32_802F5E60;

    write_f32(out, second);
    write_f32(out + 0x04, third);
    write_f32(out + 0x08, F32_802F5E68);
    write_f32(out + 0x10, first);
    write_f32(out + 0x14, fourth);
    write_f32(out + 0x18, F32_802F5E68);
    write_f32(
        out + 0x0C,
        third * -half + translation_x + half + second * -half,
    );
    write_f32(
        out + 0x1C,
        fourth * -half + translation_y + half + first * -half,
    );
    write_f32(out + 0x20, F32_802F5E68);
    write_f32(out + 0x24, F32_802F5E68);
    write_f32(out + 0x28, F32_802F5E6C);
    write_f32(out + 0x2C, F32_802F5E68);
}

/// Reads one texture-SRT entry and returns its width and height as floats.
///
/// The entry index is checked against the four-bit count in the high nibble of
/// the animation object's `+0x50` word. A valid index selects a 0x20-byte
/// record from the work buffer at `+0x58`; that record is copied to a local
/// buffer, converted by `0x801C7160`, and its width (`0x801C71E0`) and height
/// (`0x801C7200`) words are returned through `out`. The local copy matters:
/// the conversion writes into it, not into the animation work buffer.
///
/// # Original
/// - Ghidra: `FUN_801835A0` at `0x801835A0` (272 bytes)
/// - C file: `work/ghidra/main-gb/export/801835a0_FUN_801835a0.c`
///
/// # Arguments
/// * `out` - two writable floats; `out[0]` is the width, `out[1]` the height.
/// * `record` - animation object with packed counts at `+0x50` and the work
///   buffer pointer at `+0x58`.
/// * `index` - texture index, used as `index & 0xFF`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes two floats; the three `0x801Cxxxx` callees may write the stack copy.
///
/// # Safety
/// `record + 0x50`/`+0x58` must be readable and the selected 0x20-byte record
/// must lie in readable memory.
///
/// # Confidence
/// medium - the bounds test and copy are exact; the GQR/`ldexpf` noise is the
/// integer-to-float conversion, and the three texture callees are opaque.
pub unsafe fn fn_801835a0(out: usize, record: usize, index: u32) {
    let count = read_u32(record + 0x50) >> 0x1C;
    if index < count {
        let base = read_u32(record + 0x58) as usize;
        let source = base + (index & 0xFF) as usize * 0x20;
        let mut local = crate::platform::GuestLocal::new( [0u32; 8]);
        for word in 0..8 {
            local[word] = read_u32(source + word * 4);
        }
        let local_ptr = local.as_mut_ptr() as usize;
        platform::call2(FN_801C7160, p32(local_ptr), 0);
        let height = platform::call1(FN_801C7200, p32(local_ptr)) & 0xFFFF;
        let width = platform::call1(FN_801C71E0, p32(local_ptr)) & 0xFFFF;
        write_f32(out + 0x04, height as f32);
        write_f32(out, width as f32);
        return;
    }
    write_f32(out, F32_802F5E68);
    write_f32(out + 0x04, F32_802F5E68);
}

/// Releases an object when the delete flag is positive.
///
/// The guard is `object != 0 && delete_flag > 0`; the release helper is
/// `0x802B9280`. The object pointer is returned unchanged so callers can chain
/// the result of a destructor.
///
/// # Original
/// - Ghidra: `FUN_80188D90` at `0x80188D90` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/80188d90_FUN_80188d90.c`
///
/// # Arguments
/// * `object` - object to release.
/// * `delete_flag` - positive releases `object`.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May call `0x802B9280` (release).
///
/// # Safety
/// `object` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - the two guards and the single call are explicit.
pub unsafe fn fn_80188d90(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(FN_802B9280, p32(object));
    }
    object
}

/// Resets a material colour block to its default state.
///
/// The body clears the eight colour words at `+0x00..+0x1F`, the two special
/// bytes at `+0x42`/`+0x43` and the material-manager globals at
/// `0x80378038`/`0x8037803C`, then rebuilds the four channel triples at
/// `+0x08..+0x1F` from the two source colours at `+0x18`/`+0x1C` according to
/// the selector at `+0x20`. The alpha byte at `+0x42` scales four channel
/// bytes twice: once in the first pass and once after the selector-dependent
/// copies.
///
/// # Original
/// - Ghidra: `FUN_80188A00` at `0x80188A00` (908 bytes)
/// - C file: `work/ghidra/main-gb/export/80188a00_FUN_80188a00.c`
///
/// # Arguments
/// * `material` - material colour record; `+0x20` selects the channel order.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x00..+0x4C` of `material` and the two globals at
/// `0x80378038`/`0x8037803C`.
///
/// # Safety
/// `material + 0x00..0x4C` must be writable.
///
/// # Confidence
/// high - every store and both selector branches are explicit; the selector
/// values `0`, `1` and `2` are preserved.
pub unsafe fn fn_80188a00(material: usize) {
    for word in 0..8 {
        write_u32(material + word * 4, 0xFFFF_FFFF);
    }
    write_u8(material + 0x42, 0xFF);
    write_u8(material + 0x43, 0);
    write_f32(material + 0x44, F32_802F5F00);
    write_f32(material + 0x48, F32_802F5F00);
    write_u8(DAT_80378038, 0xFF);
    write_u32(DAT_8037803C, 0);

    for index in 0..4 {
        write_u8(material + index, 0);
    }
    for index in 4..8 {
        write_u8(material + index, 0xFF);
    }
    write_u32(material + 0x20, 0);

    for index in 0..4 {
        write_u8(material + 0x08 + index, read_u8(material + 0x18 + index));
        write_u8(material + 0x0C + index, read_u8(material + 0x18 + index));
    }

    let first = if read_u32(material + 0x20) == 2 {
        material + 0x1C
    } else {
        material + 0x18
    };
    for index in 0..4 {
        write_u8(material + 0x10 + index, read_u8(first + index));
    }

    let second = if read_u32(material + 0x20) == 0 {
        material + 0x18
    } else {
        material + 0x1C
    };
    for index in 0..4 {
        write_u8(material + 0x14 + index, read_u8(second + index));
    }

    let alpha = read_u8(material + 0x42) as u32;
    let last = read_u8(second + 3);
    write_u8(material + 0x19, 0xFF);
    write_u8(material + 0x09, 0xFF);
    write_u8(material + 0x1B, 0xFF);
    write_u8(material + 0x0B, 0xFF);
    write_u8(material + 0x1A, 0xFF);
    write_u8(material + 0x0A, 0xFF);
    write_u8(material + 0x18, 0xFF);
    write_u8(material + 0x08, 0xFF);
    for offset in [0x0F, 0x13] {
        write_u8(
            material + offset,
            ((read_u8(material + offset) as u32 * alpha) / 0xFF) as u8,
        );
    }
    write_u8(material + 0x17, (last as u32 * alpha / 0xFF) as u8);

    let third = if read_u32(material + 0x20) == 1 {
        material + 0x1C
    } else {
        material + 0x18
    };
    for index in 0..4 {
        write_u8(material + 0x0C + index, read_u8(third + index));
    }

    let fourth = if read_u32(material + 0x20) == 2 {
        material + 0x1C
    } else {
        material + 0x18
    };
    for index in 0..4 {
        write_u8(material + 0x10 + index, read_u8(fourth + index));
    }

    let fifth = if read_u32(material + 0x20) == 0 {
        material + 0x18
    } else {
        material + 0x1C
    };
    for index in 0..3 {
        write_u8(material + 0x14 + index, read_u8(fifth + index));
    }

    let alpha = read_u8(material + 0x42) as u32;
    write_u8(material + 0x15, read_u8(fifth + 1));
    write_u8(material + 0x16, read_u8(fifth + 2));
    let last = read_u8(fifth + 3);
    write_u32(material + 0x38, 1);
    write_u32(material + 0x3C, 1);
    write_f32(material + 0x24, F32_802F5F04);
    write_f32(material + 0x28, F32_802F5F04);
    write_f32(material + 0x2C, F32_802F5F00);
    write_f32(material + 0x30, F32_802F5F00);
    write_f32(material + 0x34, F32_802F5F00);
    for offset in [0x0B, 0x0F, 0x13] {
        write_u8(
            material + offset,
            ((read_u8(material + offset) as u32 * alpha) / 0xFF) as u8,
        );
    }
    write_u8(material + 0x17, (last as u32 * alpha / 0xFF) as u8);
}

/// Copies the two source colours into a text colour record.
///
/// The four bytes of `first` are stored at `+0x18..+0x1B` and `+0x08..+0x0B`;
/// the four bytes of `second` go to `+0x1C..+0x1F`. The selector at `+0x20`
/// then chooses which of those two colours each of the three channel groups
/// `+0x0C`, `+0x10` and `+0x14` receives. Finally the alpha byte at `+0x42`
/// scales the last byte of each group over `0xFF`.
///
/// # Original
/// - Ghidra: `FUN_801898E0` at `0x801898E0` (384 bytes)
/// - C file: `work/ghidra/main-gb/export/801898e0_FUN_801898e0.c`
///
/// # Arguments
/// * `config` - text colour record; `+0x20` is the selector (0, 1 or 2).
/// * `first` - four source bytes for the first colour.
/// * `second` - four source bytes for the second colour.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x02..+0x1F` of `config`.
///
/// # Safety
/// `config + 0x02..0x42` must be readable/writable and both source pointers
/// must be readable for four bytes.
///
/// # Confidence
/// high - all three selector branches and the alpha scaling are explicit.
pub unsafe fn fn_801898e0(config: usize, first: usize, second: usize) {
    let mut first_bytes = [0u8; 4];
    let mut second_bytes = [0u8; 4];
    for index in 0..4 {
        first_bytes[index] = read_u8(first + index);
        second_bytes[index] = read_u8(second + index);
    }

    for index in 0..4 {
        write_u8(config + 0x18 + index, first_bytes[index]);
        write_u8(config + 0x1C + index, second_bytes[index]);
        write_u8(config + 0x08 + index, first_bytes[index]);
    }

    let source = if read_u32(config + 0x20) == 1 {
        config + 0x1C
    } else {
        config + 0x18
    };
    for index in 0..4 {
        write_u8(config + 0x0C + index, read_u8(source + index));
    }

    let source = if read_u32(config + 0x20) == 2 {
        config + 0x1C
    } else {
        config + 0x18
    };
    for index in 0..4 {
        write_u8(config + 0x10 + index, read_u8(source + index));
    }

    let source = if read_u32(config + 0x20) == 0 {
        config + 0x18
    } else {
        config + 0x1C
    };
    for index in 0..4 {
        write_u8(config + 0x14 + index, read_u8(source + index));
    }

    let alpha = read_u8(config + 0x42) as u32;
    let last = read_u8(source + 3);
    for offset in [0x0B, 0x0F, 0x13] {
        write_u8(
            config + offset,
            ((read_u8(config + offset) as u32 * alpha) / 0xFF) as u8,
        );
    }
    write_u8(config + 0x17, (last as u32 * alpha / 0xFF) as u8);
}

/// Installs the horizontal and vertical font scales.
///
/// Two virtual calls on the font object at `+0x48` return the font's
/// horizontal (`+0x0C`) and vertical (`+0x10`) reference sizes. Each argument
/// is divided by the matching size and the results are stored at `+0x24` and
/// `+0x28`.
///
/// # Original
/// - Ghidra: `FUN_80189A80` at `0x80189A80` (208 bytes)
/// - C file: `work/ghidra/main-gb/export/80189a80_FUN_80189a80.c`
///
/// # Arguments
/// * `horizontal` - requested horizontal size.
/// * `vertical` - requested vertical size.
/// * `record` - text-layout record whose `+0x48` holds the font object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x24` and `+0x28`; performs two virtual calls.
///
/// # Safety
/// `record + 0x48` must hold a font object with a valid vtable.
///
/// # Confidence
/// medium - the arithmetic is exact; the virtual-call operands are dropped by
/// Ghidra and reconstructed from the PowerPC member-call convention. The GQR
/// blocks are integer-to-float conversion noise.
pub unsafe fn fn_80189a80(horizontal: f64, vertical: f64, record: usize) {
    let font = read_u32(record + 0x48) as usize;
    let vertical_size = platform::call1(vtable_method(font, 0, 0x10), p32(font));
    let vertical_scale = (vertical / (vertical_size as f32) as f64) as f32;
    let horizontal_size = platform::call1(vtable_method(font, 0, 0x0C), p32(font));
    write_f32(record + 0x28, vertical_scale);
    write_f32(
        record + 0x24,
        (horizontal / (horizontal_size as f32) as f64) as f32,
    );
}

/// Applies one animation key record for modes `9` and `10`.
///
/// Mode `10` stores the record's current X at `+0x2C` and the sum of the
/// line-height helpers `0x8018BCA0` and `0x80189E60` at `+0x30`, then returns
/// `3`. Mode `9` reads a step count from `+0x54`; when it is positive, the
/// function advances the record's `+0x2C` cursor by whole steps and rewrites
/// it through `0x80189E10`. The return value is `1` for mode `9` and `0` for
/// any other mode.
///
/// # Original
/// - Ghidra: `FUN_8018B0D0` at `0x8018B0D0` (348 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b0d0_FUN_8018b0d0.c`
///
/// # Arguments
/// * `_unused` - unused first operand from the key-frame dispatcher.
/// * `mode` - key mode; only `9` and `10` are handled.
/// * `record` - 3-word key record: object pointer, value, current X.
///
/// # Returns
/// `3` for mode `10`, `1` for mode `9`, `0` otherwise.
///
/// # Side effects
/// Writes the embedded text-layout record selected by `record[0]`.
///
/// # Safety
/// `record[0]` must point to a live text-layout record when `mode` is `9` or
/// `10`.
///
/// # Confidence
/// medium - both modes and the step arithmetic are exact; the GQR noise is
/// conversion-only and `0x8018BD80` remains opaque.
pub unsafe fn fn_8018b0d0(_unused: u32, mode: i32, record: usize) -> u32 {
    if mode == 10 {
        let object = read_u32(record) as usize;
        let current = read_f32(record + 0x08);
        let line_height = fn_8018bca0(object);
        let extra = fn_80189e60(object);
        fn_80189e00(
            current as f64,
            ((extra + line_height) as f32) as f64,
            object,
        );
        return 3;
    }
    if mode == 9 {
        let object = read_u32(record) as usize;
        let steps = platform::call1(FN_8018BD80, p32(object));
        if (steps as i32) > 0 {
            let advance = if fn_80189ca0(object) == 0 {
                fn_80189b50(object)
            } else {
                fn_80189cb0(object)
            };
            let position = fn_80189e50(object);
            let current = read_f32(record + 0x08);
            let step = ((steps as f32) as f64 * advance) as f32;
            let quotient = ((position - current as f64) as f32 / step) as i32 as u32;
            let whole_steps = quotient.wrapping_add(1);
            let value = (current as f64 + ((step * (whole_steps as f32)) as f64)) as f32;
            fn_80189e10(value as f64, object);
        }
        return 1;
    }
    0
}

/// Applies one animation key record for modes `9` and `10` (second table).
///
/// This is the twin of [`fn_8018b0d0`]: mode `10` uses `0x8018E660` instead of
/// `0x8018BCA0` for the line-height sum, and mode `9` reads its step count
/// through `0x8018E740` instead of `0x8018BD80`. The arithmetic and return
/// values are otherwise identical.
///
/// # Original
/// - Ghidra: `FUN_8018B630` at `0x8018B630` (348 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b630_FUN_8018b630.c`
///
/// # Arguments
/// * `_unused` - unused first operand from the key-frame dispatcher.
/// * `mode` - key mode; only `9` and `10` are handled.
/// * `record` - 3-word key record: object pointer, value, current X.
///
/// # Returns
/// `3` for mode `10`, `1` for mode `9`, `0` otherwise.
///
/// # Side effects
/// Writes the embedded text-layout record selected by `record[0]`.
///
/// # Safety
/// `record[0]` must point to a live text-layout record when `mode` is `9` or
/// `10`.
///
/// # Confidence
/// medium - same shape as [`fn_8018b0d0`]; only the two callee addresses
/// differ, and `0x8018E740` remains opaque.
pub unsafe fn fn_8018b630(_unused: u32, mode: i32, record: usize) -> u32 {
    if mode == 10 {
        let object = read_u32(record) as usize;
        let current = read_f32(record + 0x08);
        let line_height = fn_8018e660(object);
        let extra = fn_80189e60(object);
        fn_80189e00(
            current as f64,
            ((extra + line_height) as f32) as f64,
            object,
        );
        return 3;
    }
    if mode == 9 {
        let object = read_u32(record) as usize;
        let steps = platform::call1(FN_8018E740, p32(object));
        if (steps as i32) > 0 {
            let advance = if fn_80189ca0(object) == 0 {
                fn_80189b50(object)
            } else {
                fn_80189cb0(object)
            };
            let position = fn_80189e50(object);
            let current = read_f32(record + 0x08);
            let step = ((steps as f32) as f64 * advance) as f32;
            let quotient = ((position - current as f64) as f32 / step) as i32 as u32;
            let whole_steps = quotient.wrapping_add(1);
            let value = (current as f64 + ((step * (whole_steps as f32)) as f64)) as f32;
            fn_80189e10(value as f64, object);
        }
        return 1;
    }
    0
}

/// Sets the text-layout cursor from the record value and the first metric.
///
/// The record's first word selects the text-layout object and `+0x08` is the
/// current X. [`fn_8018bca0`] returns the horizontal metric; adding
/// [`fn_80189e60`]'s `+0x30` height gives the Y. [`fn_80189e00`] then stores
/// `(record[+0x08], metric + height)` in the object.
///
/// This is the [`fn_8018bca0`] twin of [`fn_8018b9d0`], which uses
/// [`fn_8018e660`] instead. Both are the fix-up halves of the key-frame
/// handlers [`fn_8018b4e0`] / [`fn_8018ba40`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8018B470`.
/// - Extent: `0x8018B470..0x8018B4E0` (112 bytes, 28 words), ending at the
///   `blr` at `0x8018B4DC` flush against [`fn_8018b4e0`].
/// - Reach: none - no `b`/`bl` site in the main image or any REL and no data
///   word names the start; the translated siblings [`fn_8018b4e0`] and
///   [`fn_8018ba40`] have the same shape, so the key-frame dispatch table is
///   evidently materialised at runtime. Not a pointer-run interior.
/// - Observed: `9421FFD0 stwu r1,-0x30(r1)`; `mflr r0`; `stw r0,0x34(r1)`;
///   `stfd f31,0x20(r1)`; `psq_st f31,0x28(r1),0,0`; `stfd f30,0x10(r1)`;
///   `psq_st f30,0x18(r1),0,0`; `stw r31,0x0C(r1)`; `lwz r31,0(r4)`;
///   `lfs f30,8(r4)`; `mr r3,r31`; `48000805 bl 0x8018BCA0`;
///   `FFE00890 fmr f31,f1`; `mr r3,r31`; `4BFFE9B9 bl 0x80189E60`;
///   `EC41F82A fadds f2,f1,f31`; `mr r3,r31`; `FC20F090 fmr f1,f30`;
///   `4BFFE949 bl 0x80189E00`; epilogue with `psq_l`/`lfd` restores;
///   `4E800020 blr`.
/// - Polarity: none - the body has no conditional branch.
/// - Decoder note: `F3E10028`/`F3C10018` are opcode-60 `psq_st` and
///   `E3E10028`/`E3C10018` are opcode-56 `psq_l` (W=0, I=0) paired-single
///   save/restore words at `0x28`/`0x18`; they are register-save noise, not
///   arithmetic. `FFE00890` and `FC20F090` are opcode-63 A-form words with
///   `XO = 72`, i.e. `fmr`, not `frsp`; `EC41F82A` is opcode-59 `fadds`
///   (`XO = 21`).
///
/// # Arguments
/// * `_unused` - unused first operand from the key-frame dispatcher.
/// * `record` - 3-word key record: object pointer, value, current X.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the object's `+0x2C`/`+0x30` pair through [`fn_80189e00`].
///
/// # Safety
/// `record[0]` must point at a live text-layout record and `record + 0x08`
/// must be readable.
///
/// # Confidence
/// medium - the loads, the `fadds` and the store call are literal; the meaning
/// of the two metrics is `// inferred:` from their use in [`fn_8018b630`].
pub unsafe fn fn_8018b470(_unused: u32, record: usize) {
    let object = read_u32(record) as usize;
    let current = read_f32(record + 0x08);
    let height = fn_8018bca0(object);
    let extra = fn_80189e60(object);
    fn_80189e00(current as f64, ((extra + height) as f32) as f64, object);
}

/// Sets the text-layout cursor from the record value and the second metric.
///
/// Identical to [`fn_8018b470`] except that the horizontal metric comes from
/// [`fn_8018e660`], the duplicate of [`fn_8018bca0`]. This mirrors the
/// [`fn_8018b4e0`] / [`fn_8018ba40`] pair, which differs only in its duration
/// accessor (`0x8018BD80` / `0x8018E740`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8018B9D0`.
/// - Extent: `0x8018B9D0..0x8018BA40` (112 bytes, 28 words), ending at the
///   `blr` at `0x8018BA3C` flush against [`fn_8018ba40`].
/// - Reach: none - no `b`/`bl` site in the main image or any REL and no data
///   word names the start (same as [`fn_8018b470`] and
///   [`fn_8018ba40`]).
/// - Observed: `9421FFD0 stwu r1,-0x30(r1)`; `mflr r0`; `stw r0,0x34(r1)`;
///   `stfd f31,0x20(r1)`; `psq_st f31,0x28(r1),0,0`; `stfd f30,0x10(r1)`;
///   `psq_st f30,0x18(r1),0,0`; `stw r31,0x0C(r1)`; `lwz r31,0(r4)`;
///   `lfs f30,8(r4)`; `mr r3,r31`; `48002C65 bl 0x8018E660`;
///   `FFE00890 fmr f31,f1`; `mr r3,r31`; `4BFFE459 bl 0x80189E60`;
///   `EC41F82A fadds f2,f1,f31`; `mr r3,r31`; `FC20F090 fmr f1,f30`;
///   `4BFFE3E9 bl 0x80189E00`; epilogue; `4E800020 blr`.
/// - Polarity: none - no conditional branch.
/// - Decoder note: the paired-single `psq_st`/`psq_l` words are the same
///   register-save pair as in [`fn_8018b470`]; the only arithmetic word is the
///   opcode-59 `fadds` `EC41F82A`.
///
/// # Arguments
/// * `_unused` - unused first operand from the key-frame dispatcher.
/// * `record` - 3-word key record: object pointer, value, current X.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the object's `+0x2C`/`+0x30` pair through [`fn_80189e00`].
///
/// # Safety
/// `record[0]` must point at a live text-layout record.
///
/// # Confidence
/// medium - see [`fn_8018b470`]; only the metric accessor differs.
pub unsafe fn fn_8018b9d0(_unused: u32, record: usize) {
    let object = read_u32(record) as usize;
    let current = read_f32(record + 0x08);
    let height = fn_8018e660(object);
    let extra = fn_80189e60(object);
    fn_80189e00(current as f64, ((extra + height) as f32) as f64, object);
}

/// Advances a text-layout record by whole animation steps.
///
/// The step count comes from the record's `+0x54` word. When it is positive,
/// the function computes the per-step advance from the horizontal metric and
/// rewrites the cursor at `+0x2C` with `current + steps * step_width`, where
/// the step width is rounded up to a whole number of steps.
///
/// # Original
/// - Ghidra: `FUN_8018B4E0` at `0x8018B4E0` (248 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b4e0_FUN_8018b4e0.c`
///
/// # Arguments
/// * `_unused` - unused first operand.
/// * `record` - 3-word key record: object pointer, value, current X.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the embedded text-layout record's `+0x2C` cursor.
///
/// # Safety
/// `record[0]` must point to a live text-layout record.
///
/// # Confidence
/// medium - the arithmetic is exact; `0x8018BD80` stays opaque and the GQR
/// blocks are conversion noise.
pub unsafe fn fn_8018b4e0(_unused: u32, record: usize) {
    let object = read_u32(record) as usize;
    let steps = platform::call1(FN_8018BD80, p32(object));
    if (steps as i32) > 0 {
        let advance = if fn_80189ca0(object) == 0 {
            fn_80189b50(object)
        } else {
            fn_80189cb0(object)
        };
        let position = fn_80189e50(object);
        let current = read_f32(record + 0x08);
        let step = ((steps as f32) as f64 * advance) as f32;
        let quotient = ((position - current as f64) as f32 / step) as i32 as u32;
        let whole_steps = quotient.wrapping_add(1);
        let value = (current as f64 + ((step * (whole_steps as f32)) as f64)) as f32;
        fn_80189e10(value as f64, object);
    }
}

/// Advances a text-layout record using the second duration accessor.
///
/// Identical to [`fn_8018b4e0`] except that the step count is read through
/// `0x8018E740` (the duplicate of `0x8018BD80`).
///
/// # Original
/// - Ghidra: `FUN_8018BA40` at `0x8018BA40` (248 bytes)
/// - C file: `work/ghidra/main-gb/export/8018ba40_FUN_8018ba40.c`
///
/// # Arguments
/// * `_unused` - unused first operand.
/// * `record` - 3-word key record: object pointer, value, current X.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the embedded text-layout record's `+0x2C` cursor.
///
/// # Safety
/// `record[0]` must point to a live text-layout record.
///
/// # Confidence
/// medium - see [`fn_8018b4e0`]; only the accessor address differs.
pub unsafe fn fn_8018ba40(_unused: u32, record: usize) {
    let object = read_u32(record) as usize;
    let steps = platform::call1(FN_8018E740, p32(object));
    if (steps as i32) > 0 {
        let advance = if fn_80189ca0(object) == 0 {
            fn_80189b50(object)
        } else {
            fn_80189cb0(object)
        };
        let position = fn_80189e50(object);
        let current = read_f32(record + 0x08);
        let step = ((steps as f32) as f64 * advance) as f32;
        let quotient = ((position - current as f64) as f32 / step) as i32 as u32;
        let whole_steps = quotient.wrapping_add(1);
        let value = (current as f64 + ((step * (whole_steps as f32)) as f64)) as f32;
        fn_80189e10(value as f64, object);
    }
}

/// Installs the key-record vtable at `0x8032C8B0`.
///
/// # Original
/// - Ghidra: `FUN_8018B5E0` at `0x8018B5E0` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b5e0_FUN_8018b5e0.c`
///
/// # Arguments
/// * `object` - receiver whose first word is the vtable slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `object`.
///
/// # Safety
/// `object` must be writable for four bytes.
///
/// # Confidence
/// high - one store; the table address is the literal in the C file.
pub unsafe fn fn_8018b5e0(object: usize) {
    write_u32(object, VT_8032C8B0);
}

/// Releases an object when the delete flag is positive (second wrapper).
///
/// # Original
/// - Ghidra: `FUN_8018B5F0` at `0x8018B5F0` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/8018b5f0_FUN_8018b5f0.c`
///
/// # Arguments
/// * `object` - object to release.
/// * `delete_flag` - positive releases `object`.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// May call `0x802B9280` (release).
///
/// # Safety
/// `object` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - the guard and the call are explicit.
pub unsafe fn fn_8018b5f0(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(FN_802B9280, p32(object));
    }
    object
}

// ---------------------------------------------------------------------------
// 0x8018BB40 - reset one material record and seed its +0x4C block
// ---------------------------------------------------------------------------

/// `0x80378064`: first layout descriptor, stored at `+0x5C` by
/// [`fn_8018bb40`] (the same descriptor `0x8018B080`/`0x802DC780` register in
/// [`crate::boot::init_tables`]).
const W284_DESC_80378064: usize = 0x8037_8064;

/// Resets a material record, then seeds `+0x4C..+0x5C`.
///
/// In plain English: run the material reset [`fn_80188a00`], store `0.0f` at
/// `+0x4C` and `+0x50`, `4` at `+0x54`, `0` at `+0x58` and the descriptor
/// `0x80378064` at `+0x5C`, and return the record.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018BB40`.
/// - Extent: `0x8018BB40..0x8018BB9C` (92 bytes, 23 words), ending at the
///   `blr` at `0x8018BB98`; one zero word at `0x8018BB9C` precedes the
///   deleting release wrapper [`fn_8018bba0`].
/// - Reach: no `b`/`bl` and no data word in the main image, and none of the
///   four REL ELFs references it, so no shipped vtable slot names it.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `bl 0x80188A00`;
///   `3C60802F lis r3,0x802f`; `3C808038 lis r4,0x8038`;
///   `C0035F28 lfs f0,0x5F28(r3)` (`0x802F5F28` = `0.0f`);
///   `38848064 addi r4,r4,-32668` (`0x80380000 - 0x7F9C` = `0x80378064`);
///   `li r5,4`; `li r0,0`; `stfs f0,0x4C(r31)`; `mr r3,r31`;
///   `stfs f0,0x50(r31)`; `stw r5,0x54(r31)`; `stw r0,0x58(r31)`;
///   `stw r4,0x5C(r31)`; epilogue.
/// - Decoder note: the `addi` immediate `0x8064` is the signed `-32668`; the
///   `lis` page `0x8038` plus that negative offset is `0x80378064`, not
///   `0x80388064`. The `lis` value itself is the high half `0x80380000`.
/// - Evidence: `work/wave284/e_dis.txt`, `work/wave284/e_refs.txt`.
///
/// # Arguments
/// - `material`: material record to reset and seed (`r3`); it is also the
///   return value.
///
/// # Returns
/// `material`, unchanged.
///
/// # Side effects
/// [`fn_80188a00`]'s colour/flag reset plus five field stores.
///
/// # Safety
/// `material + 0x4C..+0x60` must be writable and the record must satisfy
/// [`fn_80188a00`]'s contract.
///
/// # Confidence
/// high - the reset call, the signed `addi`, the `0.0f` load and all five
/// stores are literal. The descriptor role at `+0x5C` is inferred from the
/// same `0x80378064` descriptor being registered by `0x8018B080`.
pub unsafe fn fn_8018bb40(material: usize) -> usize {
    fn_80188a00(material);
    write_f32(material + 0x4C, F32_802F5F28);
    write_f32(material + 0x50, F32_802F5F28);
    platform::write_u32(material + 0x54, 4);
    platform::write_u32(material + 0x58, 0);
    platform::write_u32(material + 0x5C, W284_DESC_80378064 as u32);
    material
}

// ---------------------------------------------------------------------------
// 0x8018BBA0 - release wrapper (guarded release + free)
// ---------------------------------------------------------------------------

/// Releases a text-layout record through the guarded teardown shape.
///
/// The body runs `0x80188D90(record, 0)` and then frees `record` with
/// `0x802B9280` when its own delete flag is positive. The return value is the
/// record pointer. It is the [`fn_8018e560`] shape without the second release,
/// and the release-wrapper sibling of [`fn_8018b5f0`] one step up in the same
/// band.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018BBA0`.
/// - Extent: `0x8018BBA0..0x8018BBF8` (88 bytes, 22 words), ending at the
///   `blr` at `0x8018BBF4`; two zero words fill `0x8018BBF8..0x8018BC00` and
///   the line-height helper [`fn_8018bca0`] follows.
/// - Reach: no `b`/`bl` in the main image and no data word names it; none of
///   the four REL ELFs references it either, so no vtable slot in the shipped
///   images points at this deleting destructor.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `2C030000 cmpi cr0,r3,0`;
///   `stw r0,0x14(r1)`; `mr r31,r4`; `stw r31,0xC(r1)`; `mr r30,r3`;
///   `stw r30,0x8(r1)`; `4182001C bc 12,2` (`beq`, skip everything);
///   `38800000 li r4,0` / `4BFFD1C9 bl 0x80188D90`; `2C1F0000 cmpi`;
///   `4081000C bc 4,1` (`bf cr0.gt`, free only when the flag is `> 0`);
///   `7FC3F378 mr r3,r30` / `4812D6A9 bl 0x802B9280`; `mr r3,r30`; epilogue.
/// - Evidence: `work/wave283/e_dis.txt`.
///
/// # Arguments
/// - `record`: text-layout record to release (`r3`).
/// - `delete_flag`: positive performs the final release (`r4`).
///
/// # Returns
/// `record`, unchanged.
///
/// # Side effects
/// One guarded release and, when the flag is positive, one free.
///
/// # Safety
/// `record` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - the null guard, the `> 0` test (via `bc 4,1` on `cr0.gt`) and both
/// callees are explicit.
pub unsafe fn fn_8018bba0(record: usize, delete_flag: i32) -> usize {
    if record != 0 {
        fn_80188d90(record, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(record));
        }
    }
    record
}

/// Returns a text-layout record's line height.
///
/// The font object at `+0x48` is asked for a metric through virtual `+0x34`;
/// that metric is widened to `f32`, multiplied by the scale at `+0x28`, and
/// added to the base value at `+0x50`. A null font object contributes zero.
///
/// # Original
/// - Ghidra: `FUN_8018BCA0` at `0x8018BCA0` (140 bytes)
/// - C file: `work/ghidra/main-gb/export/8018bca0_FUN_8018bca0.c`
///
/// # Arguments
/// * `record` - text-layout record; `+0x48` is the font object, `+0x28` the
///   scale and `+0x50` the base line height.
///
/// # Returns
/// `f32[+0x50] + (font->+0x34() as f32) * f32[+0x28]`, widened to `f64`.
///
/// # Side effects
/// One virtual call on the font object.
///
/// # Safety
/// `record + 0x48` must hold a valid font object or be zero.
///
/// # Confidence
/// medium - the virtual call is reconstructed from the member-call convention;
/// the GQR magic in the export is an unsigned-to-float conversion.
pub unsafe fn fn_8018bca0(record: usize) -> f64 {
    let font = read_u32(record + 0x48) as usize;
    let metric = if font == 0 {
        0
    } else {
        platform::call1(vtable_method(font, 0, 0x34), p32(font))
    };
    let scale = fn_80189a70(record);
    let extra = ((metric as f32) as f64 * scale) as f32;
    (read_f32(record + 0x50) + extra) as f64
}

/// Returns a text-layout record's line height (second copy).
///
/// Identical to [`fn_8018bca0`]; the binary carries a second copy at this
/// address for the second key table.
///
/// # Original
/// - Ghidra: `FUN_8018E660` at `0x8018E660` (140 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e660_FUN_8018e660.c`
///
/// # Arguments
/// * `record` - text-layout record; `+0x48` is the font object, `+0x28` the
///   scale and `+0x50` the base line height.
///
/// # Returns
/// `f32[+0x50] + (font->+0x34() as f32) * f32[+0x28]`, widened to `f64`.
///
/// # Side effects
/// One virtual call on the font object.
///
/// # Safety
/// `record + 0x48` must hold a valid font object or be zero.
///
/// # Confidence
/// medium - same shape as [`fn_8018bca0`].
pub unsafe fn fn_8018e660(record: usize) -> f64 {
    let font = read_u32(record + 0x48) as usize;
    let metric = if font == 0 {
        0
    } else {
        platform::call1(vtable_method(font, 0, 0x34), p32(font))
    };
    let scale = fn_80189a70(record);
    let extra = ((metric as f32) as f64 * scale) as f32;
    (read_f32(record + 0x50) + extra) as f64
}

/// Releases a text-layout record through the two-step teardown shape.
///
/// The body first runs `0x80188D90(record, 0)`, which is the guarded release
/// with a zero flag, then calls `0x802B9280(record)` when its own delete flag
/// is positive. The return value is the record pointer.
///
/// # Original
/// - Ghidra: `FUN_8018E560` at `0x8018E560` (88 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e560_FUN_8018e560.c`
///
/// # Arguments
/// * `record` - text-layout record to release.
/// * `delete_flag` - positive performs the final release.
///
/// # Returns
/// `record`, unchanged.
///
/// # Side effects
/// May call `0x802B9280` twice when both guards pass.
///
/// # Safety
/// `record` must be a live allocation when `delete_flag > 0`.
///
/// # Confidence
/// high - the nested guards and both callees are explicit.
pub unsafe fn fn_8018e560(record: usize, delete_flag: i32) -> usize {
    if record != 0 {
        fn_80188d90(record, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(record));
        }
    }
    record
}

/// Stores a font object pointer at `record + 0x48`.
///
/// # Original
/// - Ghidra: `FUN_80188DD0` at `0x80188DD0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80188dd0_FUN_80188dd0.c`
///
/// # Arguments
/// * `record` - text-layout record.
/// * `font` - font object pointer to store.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `record + 0x48`.
///
/// # Safety
/// `record + 0x48` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_80188dd0(record: usize, font: usize) {
    write_u32(record + 0x48, p32(font));
}

/// Loads the font object pointer from `record + 0x48`.
///
/// # Original
/// - Ghidra: `FUN_80188DE0` at `0x80188DE0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80188de0_FUN_80188de0.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// The word at `record + 0x48`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x48` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80188de0(record: usize) -> u32 {
    read_u32(record + 0x48)
}

/// Reads the horizontal scale at `record + 0x24`.
///
/// # Original
/// - Ghidra: `FUN_80189A60` at `0x80189A60` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189a60_FUN_80189a60.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// `f32[+0x24]` widened to `f64`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x24` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80189a60(record: usize) -> f64 {
    read_f32(record + 0x24) as f64
}

/// Reads the vertical scale at `record + 0x28`.
///
/// # Original
/// - Ghidra: `FUN_80189A70` at `0x80189A70` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189a70_FUN_80189a70.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// `f32[+0x28]` widened to `f64`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x28` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80189a70(record: usize) -> f64 {
    read_f32(record + 0x28) as f64
}

/// Reads the vertical-layout flag at `record + 0x43`.
///
/// # Original
/// - Ghidra: `FUN_80189CA0` at `0x80189CA0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189ca0_FUN_80189ca0.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// The byte at `record + 0x43`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x43` must be readable.
///
/// # Confidence
/// high - one byte load; the call sites use it to choose horizontal or
/// vertical advance.
pub unsafe fn fn_80189ca0(record: usize) -> u8 {
    read_u8(record + 0x43)
}

/// Reads the vertical advance at `record + 0x44`.
///
/// # Original
/// - Ghidra: `FUN_80189CB0` at `0x80189CB0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189cb0_FUN_80189cb0.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// `f32[+0x44]` widened to `f64`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x44` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80189cb0(record: usize) -> f64 {
    read_f32(record + 0x44) as f64
}

/// Stores two floats at `record + 0x2C` and `record + 0x30`.
///
/// # Original
/// - Ghidra: `FUN_80189E00` at `0x80189E00` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e00_FUN_80189e00.c`
///
/// # Arguments
/// * `x` - value for `+0x2C`, narrowed to `f32`.
/// * `y` - value for `+0x30`, narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight bytes at `record + 0x2C`.
///
/// # Safety
/// `record + 0x2C..0x34` must be writable.
///
/// # Confidence
/// high - two stores.
pub unsafe fn fn_80189e00(x: f64, y: f64, record: usize) {
    write_f32(record + 0x2C, x as f32);
    write_f32(record + 0x30, y as f32);
}

/// Stores one float at `record + 0x2C`.
///
/// # Original
/// - Ghidra: `FUN_80189E10` at `0x80189E10` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e10_FUN_80189e10.c`
///
/// # Arguments
/// * `value` - value narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four bytes at `record + 0x2C`.
///
/// # Safety
/// `record + 0x2C` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_80189e10(value: f64, record: usize) {
    write_f32(record + 0x2C, value as f32);
}

/// Reads the current cursor at `record + 0x2C`.
///
/// # Original
/// - Ghidra: `FUN_80189E50` at `0x80189E50` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e50_FUN_80189e50.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// `f32[+0x2C]` widened to `f64`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x2C` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80189e50(record: usize) -> f64 {
    read_f32(record + 0x2C) as f64
}

/// Reads the line advance at `record + 0x30`.
///
/// # Original
/// - Ghidra: `FUN_80189E60` at `0x80189E60` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e60_FUN_80189e60.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// `f32[+0x30]` widened to `f64`.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `record + 0x30` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80189e60(record: usize) -> f64 {
    read_f32(record + 0x30) as f64
}

// ===========================================================================
// Wave 17: text-writer print/measure helpers and text-record accessors
// ===========================================================================

// Callees used by this section. Their bodies are outside this pass; the
// addresses are the callable entry points from `functions.tsv`.
//
// `0x8018D540` steps over one character of a line and writes its rectangle;
// `0x80190300` is the wide-character twin of the char line accumulator
// [`fn_8018d920`]. `0x802DC69C`/`0x802E441C` are the char/wide string-length
// loops and `0x802E15DC`/`0x802E4390` are the char/wide `vsnprintf` family.
// inferred: the MSL names come from the loop shapes in
// `802dc69c_FUN_802dc69c.c`, `802e441c_FUN_802e441c.c`, `802e15dc_FUN_802e15dc.c`
// and `802e4390_FUN_802e4390.c`; the binary carries no symbol string for them.

/// `0x802DC69C`: char string length (NUL terminator excluded).
const FN_802DC69C_STRLEN: usize = 0x802D_C69C;

/// `0x802E15DC`: char `vsnprintf(buffer, count, format, args)`.
const FN_802E15DC_VSNPRINTF: usize = 0x802E_15DC;

/// `0x802E4390`: wide `vsnwprintf(buffer, count, format, args)`.
const FN_802E4390_VSNWPRINTF: usize = 0x802E_4390;

/// `0x802E441C`: wide (`u16`) string length in characters.
const FN_802E441C_STRLEN16: usize = 0x802E_441C;

/// `0x8018D540`: measure one char and advance the writer cursor.
///
/// # Original
/// Called by [`fn_8018d920`]; Ghidra `FUN_8018D540` at `0x8018D540`
/// (`988` bytes, `8018d540_FUN_8018d540.c`). Arguments are
/// `(writer, rect_out, text, remaining)`; the return is the consumed count.
const FN_8018D540_LINE_STEP: usize = 0x8018_D540;

/// `0x80190300`: wide-character twin of [`fn_8018d920`].
///
/// # Original
/// Ghidra `FUN_80190300` at `0x80190300` (`280` bytes,
/// `80190300_FUN_80190300.c`). `// unknown:` the body is not translated in this
/// pass; the call shape is `(writer, rect_out, text, remaining)`.
const FN_80190300_WIDE_IMPL: usize = 0x8019_0300;

/// GX texture-coordinate/range setter: four `f32`, a selector word and a
/// pointer to one 4-byte value.
///
/// # Original
/// Ghidra `FUN_801C8920` at `0x801C8920` (`801c8920_FUN_801c8920.c`).
/// `// unknown:` exact GX field; used here only by [`fn_8018a100`].
const FN_801C8920: usize = 0x801C_8920;

// GX state setters called by `fn_8018a100`. The binary does not name the
// fields they program, so each constant is an address only; the arguments are
// the literal values printed by the C export.
const FN_801C37C0: usize = 0x801C_37C0;
const FN_801C4070: usize = 0x801C_4070;
const FN_801C40B0: usize = 0x801C_40B0;
const FN_801C47D0: usize = 0x801C_47D0;
const FN_801C4A20: usize = 0x801C_4A20;
const FN_801C6AC0: usize = 0x801C_6AC0;
const FN_801C6AF0: usize = 0x801C_6AF0;
const FN_801C8110: usize = 0x801C_8110;
const FN_801C8130: usize = 0x801C_8130;
const FN_801C82F0: usize = 0x801C_82F0;
const FN_801C8330: usize = 0x801C_8330;
const FN_801C8370: usize = 0x801C_8370;
const FN_801C83D0: usize = 0x801C_83D0;
const FN_801C8430: usize = 0x801C_8430;
const FN_801C8600: usize = 0x801C_8600;
const FN_801C8640: usize = 0x801C_8640;
const FN_801C8700: usize = 0x801C_8700;
const FN_801C8790: usize = 0x801C_8790;
const FN_801C88F0: usize = 0x801C_88F0;
const FN_801C8DC0: usize = 0x801C_8DC0;

/// Scratch size used by the `printf` wrappers, from `0x8032C938`/`0x8032C93C`.
///
/// # Original
/// The two words are `0x00000100` in `main.elf`; the char wrapper passes it as
/// a byte count, the wide wrapper as a character count.
const TEXT_FORMAT_BUFFER: usize = 0x100;

/// Copies the 0x60-byte text-writer state record.
///
/// The original copies three separate spans: the words at `+0x00..+0x3F`, the
/// halfword at `+0x40` and the two bytes at `+0x42`/`+0x43`, then the words at
/// `+0x44..+0x5F`. The byte at `+0x41` is padding and is deliberately not
/// copied.
///
/// # Original
/// Not a separate function: the inline copy at the head of every text-writer
/// wrapper in this section (`0x8018C3B0`, `0x8018C520`, `0x8018C660`, ...).
/// Ghidra prints it as `local_a8 = *puVar1` through `local_4c = puVar1[0x17]`.
///
/// # Arguments
/// * `dst` - 0x60-byte destination (a stack clone in every caller).
/// * `src` - writer state record.
///
/// # Safety
/// `src + 0x60` must be readable and `dst + 0x60` writable.
///
/// # Confidence
/// high - every load/store offset is explicit in the callers.
unsafe fn copy_text_state(dst: usize, src: usize) {
    let mut offset = 0;
    while offset < 0x40 {
        write_u32(dst + offset, read_u32(src + offset));
        offset += 4;
    }
    write_u16(dst + 0x40, read_u16(src + 0x40));
    write_u8(dst + 0x42, read_u8(src + 0x42));
    write_u8(dst + 0x43, read_u8(src + 0x43));
    offset = 0x44;
    while offset < 0x60 {
        write_u32(dst + offset, read_u32(src + offset));
        offset += 4;
    }
}

/// Calls `addr(f32, f32, f32, f32, u32, u32) -> u32`.
///
/// # Original
/// Host-side helper; no binary address. Models the `0x801C8920` call shape:
/// four floats in `f1..f4`, then a selector and a pointer in `r3`/`r4`.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper, same shape used by the GX lane.
unsafe fn call_f32x4_u32_ptr(
    addr: usize,
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    a: u32,
    b: u32,
) -> u32 {
    let function = crate::platform::guest_fn!(addr, fn(f32, f32, f32, f32, u32, u32) -> u32);
    function(f0, f1, f2, f3, a, b)
}

/// Writes one line height into the text-layout record's `+0x30` slot.
///
/// # Original
/// - Ghidra: `FUN_80189E20` at `0x80189E20` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e20_FUN_80189e20.c`
///
/// # Arguments
/// * `value` - new line advance; narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the `f32` at `record + 0x30`.
///
/// # Safety
/// `record + 0x30` must be writable.
///
/// # Confidence
/// high - one store, offset explicit.
pub unsafe fn fn_80189e20(value: f64, record: usize) {
    write_f32(record + 0x30, value as f32);
}

/// Programs the default GX TEV state used by the font texture.
///
/// The first call seeds the GX texture-coordinate/range selector with the
/// global at `0x80378034`; on the first use the flag at `0x80378030` is set to
/// one and the cached selector is cleared to zero. The remaining 33 calls are
/// literal state writes, and the two arguments supply the two source colour
/// words handed to `0x801C8430`.
/// // inferred: the 0x80378034 access is one byte wide in the reconstruction;
/// the export does not reveal whether the original word was a byte or a word,
/// but every path writes zero before the read.
///
/// # Original
/// - Ghidra: `FUN_8018A100` at `0x8018A100` (764 bytes)
/// - C file: `work/ghidra/main-gb/export/8018a100_FUN_8018a100.c`
///
/// # Arguments
/// * `first` - pointer to the first four-byte colour word.
/// * `second` - pointer to the second four-byte colour word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two GX globals at `0x80378030`/`0x80378034`; performs 34 GX
/// state calls whose argument lists are literal in the C export.
///
/// # Safety
/// Both arguments must be readable for four bytes and the GX callees must be
/// present in the target address space.
///
/// # Confidence
/// medium - every call and literal is exact; the individual GX fields are
/// unnamed (`// unknown:`), and only the first callee takes float registers.
pub unsafe fn fn_8018a100(first: usize, second: usize) {
    if read_u8(0x8037_8030) == 0 {
        write_u8(0x8037_8034, 0);
        write_u8(0x8037_8030, 1);
    }

    let mut selector = crate::platform::GuestLocal::new( [0u32; 1]);
    selector[0] = read_u8(0x8037_8034) as u32;
    let _ = call_f32x4_u32_ptr(
        FN_801C8920,
        0.0,
        0.0,
        0.0,
        0.0,
        0,
        selector.as_mut_ptr() as u32,
    );

    let _ = platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    let _ = platform::call3(FN_801C8700, 0, 0x11, 0);
    let _ = platform::call1(FN_801C6AC0, 1);
    let _ = platform::call7(FN_801C6AF0, 4, 0, 0, 1, 0, 0, 2);
    let _ = platform::call7(FN_801C6AF0, 5, 0, 0, 0, 0, 0, 2);
    let _ = platform::call1(FN_801C4A20, 1);
    let _ = platform::call6(FN_801C47D0, 0, 1, 4, 0x3C, 0, 0x7D);
    let _ = platform::call1(FN_801C8110, 0);
    let _ = platform::call4(FN_801C8DC0, 1, 4, 5, 0xF);
    let _ = platform::call1(FN_801C88F0, 2);
    let _ = platform::call1(FN_801C8130, 0);
    let _ = platform::call1(FN_801C8130, 1);
    let _ = platform::call3(FN_801C8600, 0, 0, 0);
    let _ = platform::call3(FN_801C8600, 1, 0, 0);
    let _ = platform::call4(FN_801C8790, 0, 0, 0, 0xFF);

    let mut first_colour = crate::platform::GuestLocal::new( [0u32; 1]);
    first_colour[0] = read_u32(first);
    let _ = platform::call2(FN_801C8430, 1, first_colour.as_mut_ptr() as u32);

    let mut second_colour = crate::platform::GuestLocal::new( [0u32; 1]);
    second_colour[0] = read_u32(second);
    let _ = platform::call2(FN_801C8430, 2, second_colour.as_mut_ptr() as u32);

    let _ = platform::call5(FN_801C82F0, 0, 2, 4, 8, 0xF);
    let _ = platform::call5(FN_801C8330, 0, 1, 2, 4, 7);
    let _ = platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
    let _ = platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
    let _ = platform::call4(FN_801C8790, 1, 0xFF, 0xFF, 4);
    let _ = platform::call5(FN_801C82F0, 1, 0xF, 0, 10, 0xF);
    let _ = platform::call5(FN_801C8330, 1, 7, 0, 5, 7);
    let _ = platform::call6(FN_801C8370, 1, 0, 0, 0, 1, 0);
    let _ = platform::call6(FN_801C83D0, 1, 0, 0, 0, 1, 0);

    let _ = platform::call5(FN_801C40B0, 0, 9, 1, 4, 0);
    let _ = platform::call5(FN_801C40B0, 0, 0xB, 1, 5, 0);
    let _ = platform::call5(FN_801C40B0, 0, 0xD, 1, 2, 0xF);
    platform::call0(FN_801C4070);
    let _ = platform::call2(FN_801C37C0, 9, 1);
    let _ = platform::call2(FN_801C37C0, 0xB, 1);
    let _ = platform::call2(FN_801C37C0, 0xD, 1);
}

/// Returns the text record's `+0x54` word.
///
/// # Original
/// - Ghidra: `FUN_8018BD80` at `0x8018BD80` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018bd80_FUN_8018bd80.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// The `u32` at `record + 0x54` (the animation step count).
///
/// # Side effects
/// None.
///
/// # Safety
/// `record + 0x54` must be readable.
///
/// # Confidence
/// high - one load, offset explicit. This is the body behind the opaque
/// `FN_8018BD80` call used by [`fn_8018b0d0`] and [`fn_8018b4e0`].
pub unsafe fn fn_8018bd80(record: usize) -> u32 {
    read_u32(record + 0x54)
}

/// Formats a string and measures it with the char text writer.
///
/// The `printf` arguments are consumed by `vsnprintf` into a 0x100-byte
/// scratch buffer (the global scratch pointer at `0x80378060` when it is
/// non-null, otherwise a stack buffer). The writer state is then cloned and
/// [`fn_8018d920`] accumulates the formatted text into the caller's rectangle.
/// The clone is released with a zero delete flag.
///
/// # Original
/// - Ghidra: `FUN_8018C3B0` at `0x8018C3B0` (368 bytes)
/// - C file: `work/ghidra/main-gb/export/8018c3b0_FUN_8018c3b0.c`
///
/// # Arguments
/// * `writer` - source text-writer state; only read.
/// * `rect` - four-float output rectangle.
/// * `format` - `printf` format string.
/// * `args` - pointer to the caller's `va_list`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the scratch buffer, the rectangle, and the transient writer clone.
///
/// # Safety
/// `format` and `args` must satisfy the `vsnprintf` contract, `rect + 0x10`
/// must be writable and `writer + 0x60` readable.
///
/// # Confidence
/// medium - the buffer, range and clone flow are exact; Ghidra drops the
/// `format`/`args` operands of the `vsnprintf` call, so the `va_list` is
/// modelled as an explicit pointer.
pub unsafe fn fn_8018c3b0(writer: usize, rect: usize, format: usize, args: usize) {
    let mut stack_buffer = crate::platform::GuestLocal::new( [0u8; TEXT_FORMAT_BUFFER]);
    let global_buffer = read_u32(0x8037_8060) as usize;
    let buffer = if global_buffer != 0 {
        global_buffer
    } else {
        stack_buffer.as_mut_ptr() as usize
    };
    let length = platform::call4(
        FN_802E15DC_VSNPRINTF,
        p32(buffer),
        TEXT_FORMAT_BUFFER as u32,
        p32(format),
        p32(args),
    );

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_8018d920(copy_ptr, rect, buffer, length as i32);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Measures a char string of known length and returns its width.
///
/// The writer state is cloned, [`fn_8018d920`] fills a local rectangle, the
/// clone is released and the horizontal extent (`right - left`) is returned.
///
/// # Original
/// - Ghidra: `FUN_8018C520` at `0x8018C520` (320 bytes)
/// - C file: `work/ghidra/main-gb/export/8018c520_FUN_8018c520.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - char string.
/// * `len` - number of characters to measure.
///
/// # Returns
/// The measured width as `f32`.
///
/// # Side effects
/// Calls the line accumulator and the releasing destructor on the clone.
///
/// # Safety
/// `text + len` must be readable and `writer + 0x60` readable.
///
/// # Confidence
/// medium - the clone/measure/return flow is exact; Ghidra prints the function
/// without a prototype, so the operands are recovered from the `r3`/`r4`/`r5`
/// uses in the C.
pub unsafe fn fn_8018c520(writer: usize, text: usize, len: i32) -> f32 {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    fn_8018d920(copy_ptr, rect.as_mut_ptr() as usize, text, len);
    let _ = fn_80188d90(copy_ptr, 0);
    rect[2] - rect[0]
}

/// Measures a NUL-terminated char string and returns its width.
///
/// The length comes from `0x802DC69C`; the rest is the same clone/measure/
/// release flow as [`fn_8018c520`].
///
/// # Original
/// - Ghidra: `FUN_8018C660` at `0x8018C660` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/8018c660_FUN_8018c660.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - NUL-terminated char string.
///
/// # Returns
/// The measured width as `f32`.
///
/// # Side effects
/// Calls the string-length loop, the line accumulator and the destructor.
///
/// # Safety
/// `text` must be NUL-terminated and `writer + 0x60` readable.
///
/// # Confidence
/// medium - see [`fn_8018c520`]; the length call is explicit.
pub unsafe fn fn_8018c660(writer: usize, text: usize) -> f32 {
    let len = platform::call1(FN_802DC69C_STRLEN, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    fn_8018d920(copy_ptr, rect.as_mut_ptr() as usize, text, len);
    let _ = fn_80188d90(copy_ptr, 0);
    rect[2] - rect[0]
}

/// Measures a char string of known length and returns its height.
///
/// Identical to [`fn_8018c520`] except that the vertical extent
/// (`bottom - top`) is returned instead of the width.
///
/// # Original
/// - Ghidra: `FUN_8018C7B0` at `0x8018C7B0` (320 bytes)
/// - C file: `work/ghidra/main-gb/export/8018c7b0_FUN_8018c7b0.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - char string.
/// * `len` - number of characters to measure.
///
/// # Returns
/// The measured height as `f32`.
///
/// # Side effects
/// Same as [`fn_8018c520`].
///
/// # Safety
/// Same as [`fn_8018c520`].
///
/// # Confidence
/// medium - same body as [`fn_8018c520`] with the other two rectangle corners
/// subtracted.
pub unsafe fn fn_8018c7b0(writer: usize, text: usize, len: i32) -> f32 {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    fn_8018d920(copy_ptr, rect.as_mut_ptr() as usize, text, len);
    let _ = fn_80188d90(copy_ptr, 0);
    rect[3] - rect[1]
}

/// Measures a NUL-terminated char string and returns its height.
///
/// Identical to [`fn_8018c660`] except for the returned extent.
///
/// # Original
/// - Ghidra: `FUN_8018C8F0` at `0x8018C8F0` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/8018c8f0_FUN_8018c8f0.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - NUL-terminated char string.
///
/// # Returns
/// The measured height as `f32`.
///
/// # Side effects
/// Calls the string-length loop, the line accumulator and the destructor.
///
/// # Safety
/// `text` must be NUL-terminated and `writer + 0x60` readable.
///
/// # Confidence
/// medium - see [`fn_8018c660`].
pub unsafe fn fn_8018c8f0(writer: usize, text: usize) -> f32 {
    let len = platform::call1(FN_802DC69C_STRLEN, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    fn_8018d920(copy_ptr, rect.as_mut_ptr() as usize, text, len);
    let _ = fn_80188d90(copy_ptr, 0);
    rect[3] - rect[1]
}

/// Measures a NUL-terminated char string into a caller rectangle.
///
/// The string length comes from `0x802DC69C`; [`fn_8018d920`] writes the four
/// rectangle floats directly into `rect`.
///
/// # Original
/// - Ghidra: `FUN_8018CB50` at `0x8018CB50` (304 bytes)
/// - C file: `work/ghidra/main-gb/export/8018cb50_FUN_8018cb50.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `rect` - four-float output rectangle.
/// * `text` - NUL-terminated char string.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `rect + 0x00..0x10`; calls the string-length loop, the line
/// accumulator and the destructor.
///
/// # Safety
/// `text` must be NUL-terminated, `rect + 0x10` writable and `writer + 0x60`
/// readable.
///
/// # Confidence
/// medium - operands recovered from `r3`/`r4`/`r5`; the flow is exact.
pub unsafe fn fn_8018cb50(writer: usize, rect: usize, text: usize) {
    let len = platform::call1(FN_802DC69C_STRLEN, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_8018d920(copy_ptr, rect, text, len);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Accumulates the union of the per-character rectangles of one line.
///
/// The output rectangle is cleared, the writer cursor is reset to `(0, 0)`,
/// and `0x8018D540` is called once per character. Each step returns the number
/// of characters consumed and fills a local rectangle; the output rectangle is
/// extended by taking the minimum left/top and the maximum right/bottom. The
/// loop always runs at least once and stops when the remaining length is not
/// positive.
///
/// # Original
/// - Ghidra: `FUN_8018D920` at `0x8018D920` (276 bytes)
/// - C file: `work/ghidra/main-gb/export/8018d920_FUN_8018d920.c`
///
/// # Arguments
/// * `writer` - text-writer state (the clone from the wrappers).
/// * `rect` - four-float output rectangle.
/// * `text` - char string.
/// * `length` - number of characters to consume.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `rect + 0x00..0x10`, advances the writer cursor through
/// [`fn_80189e00`], and calls `0x8018D540` at least once.
///
/// # Safety
/// `text + length` must be readable and `writer + 0x60` must be a valid text
/// record.
///
/// # Confidence
/// medium - the rectangle accumulation is exact; the first cursor reset has
/// its record operand dropped by Ghidra and is reconstructed as `writer`. The
/// GQR/paired-single blocks in the export are register-save noise and are
/// omitted.
pub unsafe fn fn_8018d920(writer: usize, rect: usize, text: usize, length: i32) {
    write_f32(rect, 0.0);
    write_f32(rect + 0x08, 0.0);
    write_f32(rect + 0x04, 0.0);
    write_f32(rect + 0x0C, 0.0);
    fn_80189e00(0.0, 0.0, writer);

    let mut remaining = length;
    let mut cursor = text;
    loop {
        let mut local = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let consumed = platform::call4(
            FN_8018D540_LINE_STEP,
            p32(writer),
            local.as_mut_ptr() as u32,
            p32(cursor),
            remaining as u32,
        ) as i32;
        // The original advances a 32-bit pointer by a signed count; keep the
        // arithmetic 32-bit so a target address above 0x7FFFFFFF does not get
        // sign-extended on a 64-bit host.
        cursor = (cursor as u32).wrapping_add(consumed as u32) as usize;
        remaining = remaining.wrapping_sub(consumed);

        let mut value = read_f32(rect);
        if local[0] < value {
            value = local[0];
        }
        write_f32(rect, value);

        value = read_f32(rect + 0x04);
        if local[1] < value {
            value = local[1];
        }
        write_f32(rect + 0x04, value);

        value = read_f32(rect + 0x08);
        if value < local[2] {
            value = local[2];
        }
        write_f32(rect + 0x08, value);

        value = read_f32(rect + 0x0C);
        if value < local[3] {
            value = local[3];
        }
        write_f32(rect + 0x0C, value);

        if remaining <= 0 {
            break;
        }
    }
}

/// Stores the horizontal scale at the text record's `+0x50` slot.
///
/// # Original
/// - Ghidra: `FUN_8018E6F0` at `0x8018E6F0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e6f0_FUN_8018e6f0.c`
///
/// # Arguments
/// * `value` - new value; narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the `f32` at `record + 0x50`.
///
/// # Safety
/// `record + 0x50` must be writable.
///
/// # Confidence
/// high - one store, offset explicit.
pub unsafe fn fn_8018e6f0(value: f64, record: usize) {
    write_f32(record + 0x50, value as f32);
}

/// Stores the letter-spacing value at the text record's `+0x4C` slot.
///
/// # Original
/// - Ghidra: `FUN_8018E700` at `0x8018E700` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e700_FUN_8018e700.c`
///
/// # Arguments
/// * `value` - new value; narrowed to `f32`.
/// * `record` - text-layout record.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the `f32` at `record + 0x4C`.
///
/// # Safety
/// `record + 0x4C` must be writable.
///
/// # Confidence
/// high - one store, offset explicit.
pub unsafe fn fn_8018e700(value: f64, record: usize) {
    write_f32(record + 0x4C, value as f32);
}

/// Returns the letter-spacing value at the text record's `+0x4C` slot.
///
/// # Original
/// - Ghidra: `FUN_8018E720` at `0x8018E720` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e720_FUN_8018e720.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// The `f32` at `record + 0x4C`, widened to `f64`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `record + 0x4C` must be readable.
///
/// # Confidence
/// high - one load, offset explicit.
pub unsafe fn fn_8018e720(record: usize) -> f64 {
    read_f32(record + 0x4C) as f64
}

/// Returns the text record's `+0x54` word (duplicate entry point).
///
/// # Original
/// - Ghidra: `FUN_8018E740` at `0x8018E740` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e740_FUN_8018e740.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// The `u32` at `record + 0x54`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `record + 0x54` must be readable.
///
/// # Confidence
/// high - one load; this is the body behind the opaque `FN_8018E740` call in
/// [`fn_8018b630`] and [`fn_8018ba40`].
pub unsafe fn fn_8018e740(record: usize) -> u32 {
    read_u32(record + 0x54)
}

/// Stores the glyph source pointer at the text record's `+0x5C` slot.
///
/// # Original
/// - Ghidra: `FUN_8018E770` at `0x8018E770` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e770_FUN_8018e770.c`
///
/// # Arguments
/// * `record` - text-layout record.
/// * `value` - pointer to the glyph/tag source object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the word at `record + 0x5C`.
///
/// # Safety
/// `record + 0x5C` must be writable.
///
/// # Confidence
/// high - one store, offset explicit.
pub unsafe fn fn_8018e770(record: usize, value: u32) {
    write_u32(record + 0x5C, value);
}

/// Returns the glyph source pointer at the text record's `+0x5C` slot.
///
/// # Original
/// - Ghidra: `FUN_8018E790` at `0x8018E790` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/8018e790_FUN_8018e790.c`
///
/// # Arguments
/// * `record` - text-layout record.
///
/// # Returns
/// The word at `record + 0x5C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `record + 0x5C` must be readable.
///
/// # Confidence
/// high - one load, offset explicit.
pub unsafe fn fn_8018e790(record: usize) -> u32 {
    read_u32(record + 0x5C)
}

/// Formats a wide string and measures it with the wide text writer.
///
/// The wide twin of [`fn_8018c3b0`]: `vsnwprintf` fills the 0x100-character
/// scratch buffer (global pointer `0x80378068` when non-null), the writer state
/// is cloned, `0x80190300` accumulates the text into `rect`, and the clone is
/// released.
///
/// # Original
/// - Ghidra: `FUN_8018ED70` at `0x8018ED70` (368 bytes)
/// - C file: `work/ghidra/main-gb/export/8018ed70_FUN_8018ed70.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `rect` - four-float output rectangle.
/// * `format` - wide `printf` format string.
/// * `args` - pointer to the caller's `va_list`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the scratch buffer, the rectangle, and the transient clone.
///
/// # Safety
/// Same contract as `vsnwprintf`, plus `rect + 0x10` writable and
/// `writer + 0x60` readable.
///
/// # Confidence
/// medium - same reconstruction as [`fn_8018c3b0`]; the wide buffer holds
/// `0x100` `u16` characters.
pub unsafe fn fn_8018ed70(writer: usize, rect: usize, format: usize, args: usize) {
    let mut stack_buffer = crate::platform::GuestLocal::new( [0u16; TEXT_FORMAT_BUFFER]);
    let global_buffer = read_u32(0x8037_8068) as usize;
    let buffer = if global_buffer != 0 {
        global_buffer
    } else {
        stack_buffer.as_mut_ptr() as usize
    };
    let length = platform::call4(
        FN_802E4390_VSNWPRINTF,
        p32(buffer),
        TEXT_FORMAT_BUFFER as u32,
        p32(format),
        p32(args),
    );

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    let _ = platform::call4(
        FN_80190300_WIDE_IMPL,
        p32(copy_ptr),
        p32(rect),
        p32(buffer),
        length,
    );
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Measures a wide string of known length and returns its width.
///
/// The wide twin of [`fn_8018c520`]: the clone is measured by `0x80190300`
/// and `right - left` is returned.
///
/// # Original
/// - Ghidra: `FUN_8018EEE0` at `0x8018EEE0` (320 bytes)
/// - C file: `work/ghidra/main-gb/export/8018eee0_FUN_8018eee0.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - `u16` string.
/// * `len` - number of characters to measure.
///
/// # Returns
/// The measured width as `f32`.
///
/// # Side effects
/// Calls the wide line accumulator and the destructor.
///
/// # Safety
/// `text + len * 2` must be readable and `writer + 0x60` readable.
///
/// # Confidence
/// medium - the body matches [`fn_8018c520`] except for the impl address and
/// the two-byte character stride taken inside `0x80190300`.
pub unsafe fn fn_8018eee0(writer: usize, text: usize, len: i32) -> f32 {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    let _ = platform::call4(
        FN_80190300_WIDE_IMPL,
        p32(copy_ptr),
        p32(rect.as_mut_ptr() as usize),
        p32(text),
        len as u32,
    );
    let _ = fn_80188d90(copy_ptr, 0);
    rect[2] - rect[0]
}

/// Measures a NUL-terminated wide string and returns its width.
///
/// The length comes from `0x802E441C`; the rest matches [`fn_8018eee0`].
///
/// # Original
/// - Ghidra: `FUN_8018F020` at `0x8018F020` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/8018f020_FUN_8018f020.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - NUL-terminated `u16` string.
///
/// # Returns
/// The measured width as `f32`.
///
/// # Side effects
/// Calls the wide length loop, the wide line accumulator and the destructor.
///
/// # Safety
/// `text` must be NUL-terminated and `writer + 0x60` readable.
///
/// # Confidence
/// medium - see [`fn_8018eee0`]; the length call is explicit.
pub unsafe fn fn_8018f020(writer: usize, text: usize) -> f32 {
    let len = platform::call1(FN_802E441C_STRLEN16, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    let _ = platform::call4(
        FN_80190300_WIDE_IMPL,
        p32(copy_ptr),
        p32(rect.as_mut_ptr() as usize),
        p32(text),
        len as u32,
    );
    let _ = fn_80188d90(copy_ptr, 0);
    rect[2] - rect[0]
}

/// Measures a wide string of known length and returns its height.
///
/// The wide twin of [`fn_8018c7b0`].
///
/// # Original
/// - Ghidra: `FUN_8018F170` at `0x8018F170` (320 bytes)
/// - C file: `work/ghidra/main-gb/export/8018f170_FUN_8018f170.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - `u16` string.
/// * `len` - number of characters to measure.
///
/// # Returns
/// The measured height as `f32`.
///
/// # Side effects
/// Calls the wide line accumulator and the destructor.
///
/// # Safety
/// `text + len * 2` must be readable and `writer + 0x60` readable.
///
/// # Confidence
/// medium - see [`fn_8018eee0`]; the returned extent is `bottom - top`.
pub unsafe fn fn_8018f170(writer: usize, text: usize, len: i32) -> f32 {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    let _ = platform::call4(
        FN_80190300_WIDE_IMPL,
        p32(copy_ptr),
        p32(rect.as_mut_ptr() as usize),
        p32(text),
        len as u32,
    );
    let _ = fn_80188d90(copy_ptr, 0);
    rect[3] - rect[1]
}

/// Measures a NUL-terminated wide string and returns its height.
///
/// The wide twin of [`fn_8018c8f0`].
///
/// # Original
/// - Ghidra: `FUN_8018F2B0` at `0x8018F2B0` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/8018f2b0_FUN_8018f2b0.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `text` - NUL-terminated `u16` string.
///
/// # Returns
/// The measured height as `f32`.
///
/// # Side effects
/// Calls the wide length loop, the wide line accumulator and the destructor.
///
/// # Safety
/// `text` must be NUL-terminated and `writer + 0x60` readable.
///
/// # Confidence
/// medium - see [`fn_8018f020`]; the returned extent is `bottom - top`.
pub unsafe fn fn_8018f2b0(writer: usize, text: usize) -> f32 {
    let len = platform::call1(FN_802E441C_STRLEN16, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);

    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
    let _ = platform::call4(
        FN_80190300_WIDE_IMPL,
        p32(copy_ptr),
        p32(rect.as_mut_ptr() as usize),
        p32(text),
        len as u32,
    );
    let _ = fn_80188d90(copy_ptr, 0);
    rect[3] - rect[1]
}

/// Measures a NUL-terminated wide string into a caller rectangle.
///
/// The wide twin of [`fn_8018cb50`]: the length comes from `0x802E441C` and
/// `0x80190300` writes the four rectangle floats directly.
///
/// # Original
/// - Ghidra: `FUN_8018F510` at `0x8018F510` (304 bytes)
/// - C file: `work/ghidra/main-gb/export/8018f510_FUN_8018f510.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `rect` - four-float output rectangle.
/// * `text` - NUL-terminated `u16` string.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `rect + 0x00..0x10`; calls the wide length loop, the wide line
/// accumulator and the destructor.
///
/// # Safety
/// `text` must be NUL-terminated, `rect + 0x10` writable and `writer + 0x60`
/// readable.
///
/// # Confidence
/// medium - operands recovered from `r3`/`r4`/`r5`; the flow is exact.
pub unsafe fn fn_8018f510(writer: usize, rect: usize, text: usize) {
    let len = platform::call1(FN_802E441C_STRLEN16, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    let _ = platform::call4(
        FN_80190300_WIDE_IMPL,
        p32(copy_ptr),
        p32(rect),
        p32(text),
        len as u32,
    );
    let _ = fn_80188d90(copy_ptr, 0);
}

// ===========================================================================
// Wave 20: remaining `lyt` text-writer, glyph and animation-transform bodies
// (`0x801819C0..0x801909F0`)
// ===========================================================================
//
// This pass finishes the NW4R layout band of `0x80180000..0x801C0000`. It
// adds the two ends of the text pipeline that wave 17 could only call by
// address:
//
// * the per-character draw/measure step (`0x8018D540` char, `0x8018FF00`
//   wide) that turns one parsed character into a glyph rectangle and advances
//   the cursor;
// * the two print loops (`0x8018DA40` char, `0x80190420` wide) that walk a
//   string, dispatch the tag processor and place each glyph;
// * the alignment wrappers (`0x8018DFF0` char, `0x801909F0` wide) that shift
//   the start position for left/centre/right and top/middle/bottom layout;
// * the surrounding `Print`/`Printf`/`CalcStringRect` wrappers and the
//   sine/cosine table lookups used by the pane matrix at `0x80180440`.
//
// The one non-text body is `0x801819C0`, the `BRLAN` animation work-buffer
// allocator, which sizes and seeds the `RLMC`/`RLTP`/`RLTS` record arrays the
// wave-2 `BRLAN` helpers index. `0x80188DF0` programs the GX texture state
// that the glyph draw path expects.
//
// All target memory is accessed through [`crate::platform`]. The
// `__savegprlr_*` prologue helpers (`0x802DCD40..0x802DCD74`) and their
// `__restgprlr_*` twins only save/restore callee-saved registers; as in the
// wave-7 section, the Rust bodies begin at the first observable operation.

/// `0x802DCBFC`: parse the next character of a text string.
///
/// The argument is a pointer to the string cursor; the function returns the
/// next character (a `u16` widened to `u32`) and advances the cursor.
///
/// # Original
/// Called by [`fn_8018d540`], [`fn_8018da40`], [`fn_8018ff00`] and
/// [`fn_80190420`]; the body is outside this pass
/// (`802dcbfc_FUN_802dcbfc.c`). `// inferred:` the name comes from the
/// pointer-to-pointer call shape and the character/stride loop that follows
/// it.
///
/// # Confidence
/// medium - call shape is explicit at every site; the body is not translated.
const FN_802DCBFC_NEXT_CHAR: usize = 0x802D_CBFC;

/// `0x801C8250`: GX texture-coordinate generation mode setter.
///
/// # Original
/// Called by the default font-state branch of [`fn_80188df0`]; the body is
/// outside this pass (`801c8250_FUN_801c8250.c`). `// unknown:` the exact GX
/// field is not named by the binary.
///
/// # Confidence
/// medium - address and two integer arguments are observed at the call site.
const FN_801C8250: usize = 0x801C_8250;

/// `0x801C74C0`: finishes the GX texture-object setup started by `0x801C6D90`.
///
/// # Original
/// Called by [`fn_80189e70`]; the body is outside this pass
/// (`801c74c0_FUN_801c74c0.c`). `// unknown:` the second argument is the
/// literal `0` in the one call site.
///
/// # Confidence
/// medium - `(texture_object, 0)` is visible in the export.
const FN_801C74C0: usize = 0x801C_74C0;

/// `0x801C5570`: GX begin/primitive setup used before the glyph quad word.
///
/// # Original
/// Called as `(0x80, 0, 4)` by [`fn_80189e70`]; the body is outside this pass
/// (`801c5570_FUN_801c5570.c`). `// unknown:` the three fields are the literal
/// arguments from the export.
///
/// # Confidence
/// medium - literal call only.
const FN_801C5570: usize = 0x801C_5570;

/// `f32 0.0` at `0x802F5F28`, used by the text-writer loops as the neutral
/// rectangle value.
///
/// # Original
/// observed in `main.elf`; read by [`fn_8018d540`], [`fn_8018da40`],
/// [`fn_8018dff0`], [`fn_8018ff00`], [`fn_80190420`] and [`fn_801909f0`].
///
/// # Confidence
/// high - raw bytes.
const F32_802F5F28: f32 = 0.0;

/// `f32 0.5` at `0x802F5F38`, the half-extent factor used for centred layout.
///
/// # Original
/// observed in `main.elf`; read by the alignment branches of
/// [`fn_8018dff0`] and [`fn_801909f0`].
///
/// # Confidence
/// high - raw bytes.
const F32_802F5F38: f32 = 0.5;

/// `f32 0.5` at `0x802F5F10`, the vertical centring factor of
/// [`fn_80189cc0`].
///
/// # Original
/// observed in `main.elf`; read by the vertical-layout branch.
///
/// # Confidence
/// high - raw bytes.
const F32_802F5F10: f32 = 0.5;

/// `f64 0x4330000000000000` at `0x802F5F30`, the unsigned-int to double magic.
///
/// # Original
/// observed in `main.elf`; the text-writer loops build this value and subtract
/// it after loading an integer into the low word, which is the standard
/// PowerPC unsigned conversion sequence.
///
/// # Confidence
/// high - raw bytes and sequence shape.
const F64_802F5F30: f64 = 4503599627370496.0;

/// `0x8032B5C8`: 256-entry sine/cosine table used by [`fn_801885b0`] and
/// [`fn_80188630`].
///
/// Each entry is 16 bytes: `+0x00` sine, `+0x04` cosine, `+0x08` sine slope,
/// `+0x0C` cosine slope, for one index step of a full turn. The table wraps
/// every 256 entries; `main.elf` holds the 4 KiB block at this address.
///
/// # Original
/// observed in `main.elf`; entry 0 is `(0, 1, 0.024541, -0.000301)` and entry
/// 64 is `(1, 0, -0.000301, -0.024541)`.
///
/// # Confidence
/// high - table contents dumped from the ELF.
const TRIG_TABLE_8032B5C8: usize = 0x8032_B5C8;

/// Stores the font-information pair at `+0x10`/`+0x14`.
///
/// # Original
/// - Ghidra: `FUN_8018AAC0` at `0x8018AAC0` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/8018aac0_FUN_8018aac0.c`
///
/// # Arguments
/// * `record` - font-information record.
/// * `first` - word stored at `record + 0x10`.
/// * `second` - word stored at `record + 0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight bytes at `record + 0x10`.
///
/// # Safety
/// `record + 0x10..0x18` must be writable.
///
/// # Confidence
/// high - two stores, both offsets explicit. This is the body behind the
/// `FN_8018AAC0_FONT_INFO` call used by the wave-7 glyph code.
pub unsafe fn fn_8018aac0(record: usize, first: u32, second: u32) {
    write_u32(record + 0x10, first);
    write_u32(record + 0x14, second);
}

/// Returns the sine of an angle in 1/256-turn index units.
///
/// The input is reduced modulo 65536 (256 turns) and split into an integer
/// index and an 8-bit fraction. The table entry supplies the sine and its
/// slope, so the result is `sin[i] + frac * dsin[i]`. A negative input negates
/// the result. This is the table lookup the pane-matrix builder
/// [`fn_80180440`] uses for its rotation.
///
/// # Original
/// - Ghidra: `FUN_801885B0` at `0x801885B0` (124 bytes)
/// - C file: `work/ghidra/main-gb/export/801885b0_FUN_801885b0.c`
///
/// # Arguments
/// * `index` - angle in 1/256-turn units (`360` degrees is `256.0`).
///
/// # Returns
/// The interpolated sine as `f64`.
///
/// # Side effects
/// None; the table at [`TRIG_TABLE_8032B5C8`] is read-only.
///
/// # Safety
/// The target image must be mapped at `0x8032B5C8`.
///
/// # Confidence
/// medium - the reduction, table lookup and interpolation are exact; the
/// original's paired-single store quantises index and fraction, which Ghidra
/// expands into `ldexpf` noise. The table values and the entry-0/entry-64
/// samples confirm the interpretation.
pub unsafe fn fn_801885b0(index: f64) -> f64 {
    let mut value = if index < 0.0 { -index } else { index };
    while value >= 65536.0 {
        // observed: the loop stores each subtraction back as `f32`.
        value = ((value - 65536.0) as f32) as f64;
    }

    let integer = value as u32;
    let fraction = (value - integer as f64) as f32;
    let entry = TRIG_TABLE_8032B5C8 + ((integer & 0xFF) as usize) * 0x10;
    let result = read_f32(entry) + fraction * read_f32(entry + 0x08);

    if index < 0.0 {
        -(result as f64)
    } else {
        result as f64
    }
}

/// Returns the cosine of an angle in 1/256-turn index units.
///
/// The exact twin of [`fn_801885b0`] using the cosine column (`+0x04`) and its
/// slope (`+0x0C`); the original performs the same reduction and has no sign
/// fix-up because cosine is even.
///
/// # Original
/// - Ghidra: `FUN_80188630` at `0x80188630` (104 bytes)
/// - C file: `work/ghidra/main-gb/export/80188630_FUN_80188630.c`
///
/// # Arguments
/// * `index` - angle in 1/256-turn units.
///
/// # Returns
/// The interpolated cosine as `f64`.
///
/// # Side effects
/// None; read-only table access.
///
/// # Safety
/// The target image must be mapped at `0x8032B5C8`.
///
/// # Confidence
/// medium - same reasoning as [`fn_801885b0`]; the constant `0.7111111` at
/// `0x802F5E64` converts degrees to index units in the caller.
pub unsafe fn fn_80188630(index: f64) -> f64 {
    let mut value = if index < 0.0 { -index } else { index };
    while value >= 65536.0 {
        value = ((value - 65536.0) as f32) as f64;
    }

    let integer = value as u32;
    let fraction = (value - integer as f64) as f32;
    let entry = TRIG_TABLE_8032B5C8 + ((integer & 0xFF) as usize) * 0x10;
    (read_f32(entry + 0x04) + fraction * read_f32(entry + 0x0C)) as f64
}

/// Measures a char string into a caller rectangle using a clone of the writer.
///
/// The writer state is copied to a stack record, [`fn_8018d920`] accumulates
/// the character rectangles into `rect`, and the clone is released with a zero
/// delete flag. Unlike the wave-17 wrappers, the text pointer and length are
/// forwarded untouched.
///
/// # Original
/// - Ghidra: `FUN_8018CA40` at `0x8018CA40` (272 bytes)
/// - C file: `work/ghidra/main-gb/export/8018ca40_FUN_8018ca40.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `rect` - four-float output rectangle.
/// * `text` - char string.
/// * `length` - number of characters to consume.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `rect + 0x00..0x10` and the transient clone.
///
/// # Safety
/// `writer + 0x60` must be readable, `rect + 0x10` writable and
/// `text + length` readable.
///
/// # Confidence
/// medium - the clone/measure/release order is exact; the two trailing
/// operands stay in `r4`/`r5`/`r6` because Ghidra prints the entry as `void`.
pub unsafe fn fn_8018ca40(writer: usize, rect: usize, text: usize, length: i32) {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_8018d920(copy_ptr, rect, text, length);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Measures a wide string into a caller rectangle using a clone of the writer.
///
/// The wide twin of [`fn_8018ca40`]: [`fn_80190300`] accumulates the two-byte
/// characters and the clone is released afterwards.
///
/// # Original
/// - Ghidra: `FUN_8018F400` at `0x8018F400` (272 bytes)
/// - C file: `work/ghidra/main-gb/export/8018f400_FUN_8018f400.c`
///
/// # Arguments
/// * `writer` - source text-writer state.
/// * `rect` - four-float output rectangle.
/// * `text` - `u16` string.
/// * `length` - number of characters to consume.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `rect + 0x00..0x10` and the transient clone.
///
/// # Safety
/// `writer + 0x60` readable, `rect + 0x10` writable and `text + length * 2`
/// readable.
///
/// # Confidence
/// medium - same flow as [`fn_8018ca40`] with the wide accumulator.
pub unsafe fn fn_8018f400(writer: usize, rect: usize, text: usize, length: i32) {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_80190300(copy_ptr, rect, text, length);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Prints a wide string of known length and restores the caller's cursor.
///
/// The writer state is cloned, [`fn_80190420`] runs the print loop on the
/// clone, then the clone's `+0x2C`/`+0x30` cursor words are copied back to the
/// caller's record before the clone is released.
///
/// # Original
/// - Ghidra: `FUN_8018FA40` at `0x8018FA40` (340 bytes)
/// - C file: `work/ghidra/main-gb/export/8018fa40_FUN_8018fa40.c`
///
/// # Arguments
/// * `writer` - text-writer state to print with (only its cursor is updated).
/// * `text` - `u16` string.
/// * `length` - number of characters to print.
///
/// # Returns
/// Nothing; the original leaves an unspecified value in `r3`.
///
/// # Side effects
/// Writes the clone, then `writer + 0x2C`/`+0x30`; frees the clone.
///
/// # Safety
/// `writer + 0x60` readable/writable and `text + length * 2` readable.
///
/// # Confidence
/// medium - the clone/print/restore order is exact; the export shows the
/// print implementation without its first three parameters.
pub unsafe fn fn_8018fa40(writer: usize, text: usize, length: i32) {
    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_80190420(copy_ptr, text, length);
    let x = fn_80189e50(copy_ptr);
    let y = fn_80189e60(copy_ptr);
    fn_80189e00(x, y, writer);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Prints a NUL-terminated wide string and restores the caller's cursor.
///
/// The length comes from `0x802E441C`; the rest is the same
/// clone/print/restore sequence as [`fn_8018fa40`].
///
/// # Original
/// - Ghidra: `FUN_8018FBA0` at `0x8018FBA0` (368 bytes)
/// - C file: `work/ghidra/main-gb/export/8018fba0_FUN_8018fba0.c`
///
/// # Arguments
/// * `writer` - text-writer state to print with.
/// * `text` - NUL-terminated `u16` string.
///
/// # Returns
/// Nothing; the original leaves an unspecified value in `r3`.
///
/// # Side effects
/// Writes the clone, then `writer + 0x2C`/`+0x30`; frees the clone.
///
/// # Safety
/// `text` must be NUL-terminated and `writer + 0x60` readable/writable.
///
/// # Confidence
/// medium - the length call and the clone/print/restore flow are exact.
pub unsafe fn fn_8018fba0(writer: usize, text: usize) {
    let length = platform::call1(FN_802E441C_STRLEN16, p32(text)) as i32;

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_80190420(copy_ptr, text, length);
    let x = fn_80189e50(copy_ptr);
    let y = fn_80189e60(copy_ptr);
    fn_80189e00(x, y, writer);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Formats a `printf`-style string and prints it with the char writer.
///
/// The formatted text goes into the global scratch buffer at `0x80378060`
/// when it is non-null, otherwise into a 0x100-byte stack buffer. A clone of
/// the writer prints it through [`fn_8018da40`], the clone's cursor is copied
/// back to the caller and the clone is released.
///
/// # Original
/// - Ghidra: `FUN_8018CC80` at `0x8018CC80` (552 bytes)
/// - C file: `work/ghidra/main-gb/export/8018cc80_FUN_8018cc80.c`
///
/// # Arguments
/// * `writer` - text-writer state.
/// * `format` - `printf` format string.
/// * `args` - pointer to the caller's `va_list`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the scratch buffer, the clone and `writer + 0x2C`/`+0x30`.
///
/// # Safety
/// `format` and `args` must satisfy the `vsnprintf` contract; `writer + 0x60`
/// must be readable and writable.
///
/// # Confidence
/// medium - buffer fallback, length and clone flow are exact; the varargs
/// prologue is flattened to an explicit `va_list` pointer, as in wave 17.
pub unsafe fn fn_8018cc80(writer: usize, format: usize, args: usize) {
    let mut stack_buffer = crate::platform::GuestLocal::new( [0u8; TEXT_FORMAT_BUFFER]);
    let global_buffer = read_u32(0x8037_8060) as usize;
    let buffer = if global_buffer != 0 {
        global_buffer
    } else {
        stack_buffer.as_mut_ptr() as usize
    };
    let length = platform::call4(
        FN_802E15DC_VSNPRINTF,
        p32(buffer),
        TEXT_FORMAT_BUFFER as u32,
        p32(format),
        p32(args),
    );

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_8018da40(copy_ptr, buffer, length as i32);
    let x = fn_80189e50(copy_ptr);
    let y = fn_80189e60(copy_ptr);
    fn_80189e00(x, y, writer);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Formats a `printf`-style wide string and prints it with the wide writer.
///
/// The wide twin of [`fn_8018cc80`]: the scratch pointer is `0x80378068`, the
/// stack fallback holds 0x100 `u16` characters, and the print loop is
/// [`fn_80190420`].
///
/// # Original
/// - Ghidra: `FUN_8018F640` at `0x8018F640` (552 bytes)
/// - C file: `work/ghidra/main-gb/export/8018f640_FUN_8018f640.c`
///
/// # Arguments
/// * `writer` - text-writer state.
/// * `format` - wide format string.
/// * `args` - pointer to the caller's `va_list`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the scratch buffer, the clone and `writer + 0x2C`/`+0x30`.
///
/// # Safety
/// `format` and `args` must satisfy the `vsnwprintf` contract; `writer + 0x60`
/// must be readable and writable.
///
/// # Confidence
/// medium - same flattening as [`fn_8018cc80`]; the 0x100 count is a character
/// count, matching the `u16` stack buffer.
pub unsafe fn fn_8018f640(writer: usize, format: usize, args: usize) {
    let mut stack_buffer = crate::platform::GuestLocal::new( [0u16; TEXT_FORMAT_BUFFER]);
    let global_buffer = read_u32(0x8037_8068) as usize;
    let buffer = if global_buffer != 0 {
        global_buffer
    } else {
        stack_buffer.as_mut_ptr() as usize
    };
    let length = platform::call4(
        FN_802E4390_VSNWPRINTF,
        p32(buffer),
        TEXT_FORMAT_BUFFER as u32,
        p32(format),
        p32(args),
    );

    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let copy_ptr = copy.as_mut_ptr() as usize;
    copy_text_state(copy_ptr, writer);
    fn_80190420(copy_ptr, buffer, length as i32);
    let x = fn_80189e50(copy_ptr);
    let y = fn_80189e60(copy_ptr);
    fn_80189e00(x, y, writer);
    let _ = fn_80188d90(copy_ptr, 0);
}

/// Accumulates the union of the per-character rectangles of one wide line.
///
/// The wide twin of [`fn_8018d920`]: the output rectangle is cleared, the
/// writer cursor is reset to `(0, 0)`, and [`fn_8018ff00`] is called once per
/// line step. Each step returns the number of *characters* consumed (the
/// original doubles it to advance the `u16` pointer) and fills a local
/// rectangle; the output is extended by taking the minimum left/top and the
/// maximum right/bottom.
///
/// # Original
/// - Ghidra: `FUN_80190300` at `0x80190300` (280 bytes)
/// - C file: `work/ghidra/main-gb/export/80190300_FUN_80190300.c`
///
/// # Arguments
/// * `writer` - text-writer state (the clone from the wrappers).
/// * `rect` - four-float output rectangle.
/// * `text` - `u16` string.
/// * `length` - number of characters to consume.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `rect + 0x00..0x10`, resets the writer cursor through
/// [`fn_80189e00`], and calls [`fn_8018ff00`] at least once.
///
/// # Safety
/// `text + length * 2` readable and `writer + 0x60` a valid text record.
///
/// # Confidence
/// medium - the rectangle accumulation and the `consumed * 2` pointer step are
/// exact; the first cursor reset's record operand is dropped by Ghidra and is
/// reconstructed as `writer`.
pub unsafe fn fn_80190300(writer: usize, rect: usize, text: usize, length: i32) {
    write_f32(rect, 0.0);
    write_f32(rect + 0x08, 0.0);
    write_f32(rect + 0x04, 0.0);
    write_f32(rect + 0x0C, 0.0);
    fn_80189e00(0.0, 0.0, writer);

    let mut remaining = length;
    let mut cursor = text;
    loop {
        let mut local = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let consumed = fn_8018ff00(writer, local.as_mut_ptr() as usize, cursor, remaining);
        cursor = (cursor as u32).wrapping_add((consumed as u32).wrapping_mul(2)) as usize;
        remaining = remaining.wrapping_sub(consumed);

        let mut value = read_f32(rect);
        if local[0] < value {
            value = local[0];
        }
        write_f32(rect, value);

        value = read_f32(rect + 0x04);
        if local[1] < value {
            value = local[1];
        }
        write_f32(rect + 0x04, value);

        value = read_f32(rect + 0x08);
        if value < local[2] {
            value = local[2];
        }
        write_f32(rect + 0x08, value);

        value = read_f32(rect + 0x0C);
        if value < local[3] {
            value = local[3];
        }
        write_f32(rect + 0x0C, value);

        if remaining <= 0 {
            break;
        }
    }
}

/// Advances the cursor by one glyph and emits its quad state.
///
/// The font object at `writer + 0x48` is asked for the glyph record through
/// virtual `+0x50`; the record's signed bytes at `+0x04` and `+0x06` are the
/// glyph offset and advance. The advance is scaled by the horizontal scale at
/// `writer + 0x24`. In vertical layout (`writer + 0x43 != 0`) the step is the
/// vertical advance at `+0x44` and the offset is centred from it. Finally
/// [`fn_80189e70`] emits the glyph and the x cursor at `+0x2C` is advanced.
///
/// # Original
/// - Ghidra: `FUN_80189CC0` at `0x80189CC0` (316 bytes)
/// - C file: `work/ghidra/main-gb/export/80189cc0_FUN_80189cc0.c`
///
/// # Arguments
/// * `writer` - text-writer state.
/// * `character` - character code passed to the font's glyph lookup.
///
/// # Returns
/// The horizontal advance in scaled units (`f64`).
///
/// # Side effects
/// Writes the writer cursor and the GX quad state through [`fn_80189e70`].
///
/// # Safety
/// `writer + 0x60` readable/writable and the font object at `writer + 0x48`
/// must have a valid vtable.
///
/// # Confidence
/// medium - the call order, offsets and both layout branches are exact; the
/// 0x14-byte glyph record layout is reconstructed from the bytes read by
/// [`fn_80189e70`] and from the two signed fields used here.
pub unsafe fn fn_80189cc0(writer: usize, character: u32) -> f64 {
    let mut glyph = crate::platform::GuestLocal::new( [0u8; 0x14]);
    let font = read_u32(writer + 0x48) as usize;
    let _ = platform::call3(
        vtable_method(font, 0, 0x50),
        p32(font),
        glyph.as_mut_ptr() as usize as u32,
        character,
    );

    let glyph_offset = glyph[0x04] as i8 as f64;
    let glyph_advance = glyph[0x06] as i8 as f64;
    let scale = read_f32(writer + 0x24);
    let advance = (glyph_advance as f32) * scale;
    let offset = (glyph_offset as f32) * scale;

    let (step, shift) = if read_u8(writer + 0x43) == 0 {
        (advance as f64, offset as f64)
    } else {
        let line = read_f32(writer + 0x44) as f64;
        (
            line,
            ((line - advance as f64) as f32 * F32_802F5F10 + offset) as f64,
        )
    };

    fn_80189e70(
        (read_f32(writer + 0x2C) as f64 + shift) as f32,
        read_f32(writer + 0x30),
        read_f32(writer + 0x34),
        writer,
        glyph.as_mut_ptr() as usize,
    );

    write_f32(writer + 0x2C, (read_f32(writer + 0x2C) as f64 + step) as f32);
    step
}

/// Emits the GX state for one glyph quad.
///
/// The glyph record supplies the texture handle (`+0x00`), the palette
/// (`+0x08`), the texture format (`+0x0C`), the texture width (`+0x0E`) and the
/// u-coordinate base (`+0x12`). When the cached texture differs from the
/// writer's two texture words at `+0x38`/`+0x3C`, the texture object is
/// rebuilt with `0x801C6D90`, `0x801C6FF0` and `0x801C74C0` and the four cache
/// words at `0x80378038`/`0x8037803C`/`0x80378040`/`0x80378044` are updated.
/// The function then issues `0x801C5570(0x80, 0, 4)` and writes one 32-bit
/// word to the GX FIFO at `0xCC008000`: the high half is the glyph's texture
/// u-coordinate (scaled to 0x8000 and divided by the texture width), the low
/// half is the writer's `+0x10` word.
///
/// # Original
/// - Ghidra: `FUN_80189E70` at `0x80189E70` (644 bytes)
/// - C file: `work/ghidra/main-gb/export/80189e70_FUN_80189e70.c`
///
/// # Arguments
/// * `x`, `y`, `z` - glyph position; the original receives them in `f1..f3`
///   and the body ignores them (the writer cursor is authoritative).
/// * `writer` - text-writer state.
/// * `glyph` - 0x14-byte glyph record filled by [`fn_80189cc0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Rebuilds the GX texture object when the cache misses, then writes the cache
/// words and one GX FIFO word.
///
/// # Safety
/// `glyph + 0x14` and `writer + 0x60` readable; the target image must map the
/// GX FIFO at `0xCC008000`.
///
/// # Confidence
/// medium - the cache test, texture rebuild, FIFO word and all offsets are
/// exact; the position arguments are unused by the original, and the integer
/// division is the original's unscaled `index * 0x8000 / width`.
pub unsafe fn fn_80189e70(x: f32, y: f32, z: f32, writer: usize, glyph: usize) {
    let _ = (x, y, z);

    let handle = read_u32(glyph);
    let index = read_u16(glyph + 0x12) as u32 + read_u8(glyph + 0x07) as u32;
    let width = read_u16(glyph + 0x0E) as u32;
    let texture_a = read_u32(writer + 0x38);
    let texture_b = read_u32(writer + 0x3C);

    if read_u32(0x8037_8038) != 0
        || handle != read_u32(0x8037_803C)
        || texture_a != read_u32(0x8037_8040)
        || texture_b != read_u32(0x8037_8044)
    {
        let mut texture = crate::platform::GuestLocal::new( [0u8; 0x24]);
        let _ = call8_u32(
            FN_801C6D90_TEX_INIT,
            p32(texture.as_mut_ptr() as usize),
            handle,
            read_u16(glyph + 0x0C) as u32,
            width,
            read_u32(glyph + 0x08),
            0,
            0,
            0,
        );
        let _ = call_f64x3_u32x6(
            FN_801C6FF0_TEX_LOD,
            0.0,
            0.0,
            0.0,
            p32(texture.as_mut_ptr() as usize),
            texture_a,
            texture_b,
            0,
            0,
            0,
        );
        platform::call2(FN_801C74C0, p32(texture.as_mut_ptr() as usize), 0);
        write_u32(0x8037_8038, 0);
        write_u32(0x8037_803C, handle);
        write_u32(0x8037_8040, texture_a);
        write_u32(0x8037_8044, texture_b);
    }

    platform::call3(FN_801C5570, 0x80, 0, 4);

    let u_coordinate = if width != 0 {
        index.wrapping_mul(0x8000) / width
    } else {
        0
    };
    let command = ((u_coordinate as u16 as u32) << 16) | (read_u32(writer + 0x10) & 0xFFFF);
    write_u32(0xCC00_8000, command);
}

/// Measures or draws one char of a line and fills its rectangle.
///
/// The function measures the writer's line extents (`+0x50` plus the glyph
/// height scaled by `+0x28`), then walks the string one parsed character at a
/// time:
///
/// * characters below `0x20` are tag bytes: the tag processor at
///   `writer + 0x5C` is called through virtual `+0x10` and the four-float
///   rectangle is unioned into the output;
/// * printable characters advance the cursor by the glyph width (horizontal
///   layout) or by `+0x44` (vertical layout) and extend only the left/right
///   extents.
///
/// The return value is the number of bytes consumed from `text`.
///
/// # Original
/// - Ghidra: `FUN_8018D540` at `0x8018D540` (988 bytes)
/// - C file: `work/ghidra/main-gb/export/8018d540_FUN_8018d540.c`
///
/// # Arguments
/// * `writer` - text-writer state (the clone from the wrappers).
/// * `rect` - four-float output rectangle.
/// * `text` - char string.
/// * `length` - remaining character budget, used as a byte count.
///
/// # Returns
/// The number of bytes consumed from `text`.
///
/// # Side effects
/// Writes `rect + 0x00..0x10`, the writer cursor `+0x2C`, and calls the tag
/// processor and the glyph-width virtuals.
///
/// # Safety
/// `text + length` readable and `writer + 0x60` a valid text record.
///
/// # Confidence
/// medium - the loop, the tag dispatch and every store are exact; the
/// paired-single quantisation of the two extents is reproduced as the plain
/// integer-to-`f32` conversion it encodes.
pub unsafe fn fn_8018d540(writer: usize, rect: usize, text: usize, length: i32) -> i32 {
    let font = fn_80188de0(writer) as usize;

    let mut tag_state = crate::platform::GuestLocal::new( [0u32; 5]);
    tag_state[0] = p32(writer);
    tag_state[1] = p32(text);

    write_f32(rect, 0.0);
    write_f32(rect + 0x08, 0.0);

    let mut height = if font != 0 {
        platform::call1(vtable_method(font, 0, 0x34), p32(font))
    } else {
        0
    };
    let scale = fn_80189a70(writer);
    let delta = (height as f64 * scale) as f32;
    let base = read_f32(writer + 0x50) + delta;
    let mut value = 0.0f32;
    if base < 0.0 {
        value = base;
    }
    write_f32(rect + 0x04, value);

    height = if font != 0 {
        platform::call1(vtable_method(font, 0, 0x34), p32(font))
    } else {
        0
    };
    let delta = (height as f64 * scale) as f32;
    let base = read_f32(writer + 0x50) + delta;
    value = 0.0f32;
    if 0.0 < base {
        value = base;
    }
    write_f32(rect + 0x0C, value);

    let mut cursor = text;
    let mut character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    let mut x = 0.0f64;
    let mut wrap = 0i32;

    loop {
        if cursor.wrapping_sub(text) as i32 > length {
            break;
        }
        if character < 0x20 {
            let mut local = crate::platform::GuestLocal::new( [0.0f32; 4]);
            local[0] = x as f32;
            local[1] = 0.0;
            local[2] = 0.0;
            local[3] = 0.0;
            tag_state[1] = p32(cursor);
            tag_state[4] = if wrap == 0 { 1 } else { 0 };
            fn_80189e10(x, writer);

            let source = read_u32(writer + 0x5C) as usize;
            let tag = platform::call4(
                vtable_method(source, 0, 0x10),
                p32(source),
                local.as_mut_ptr() as usize as u32,
                character as u32,
                tag_state.as_mut_ptr() as usize as u32,
            ) as i32;
            cursor = tag_state[1] as usize;

            let mut extent = read_f32(rect);
            if local[0] < extent {
                extent = local[0];
            }
            write_f32(rect, extent);

            extent = read_f32(rect + 0x04);
            if local[1] < extent {
                extent = local[1];
            }
            write_f32(rect + 0x04, extent);

            extent = read_f32(rect + 0x08);
            if extent < local[2] {
                extent = local[2];
            }
            write_f32(rect + 0x08, extent);

            extent = read_f32(rect + 0x0C);
            if extent < local[3] {
                extent = local[3];
            }
            write_f32(rect + 0x0C, extent);

            x = fn_80189e50(writer);
            if tag == 4 {
                break;
            }
            if tag == 1 {
                wrap = 0;
            } else if tag == 2 {
                wrap = 1;
            } else if tag == 3 {
                break;
            }
        } else {
            if wrap != 0 {
                x = (x as f32 + read_f32(writer + 0x4C)) as f64;
            }
            wrap = 1;
            if fn_80189ca0(writer) == 0 {
                let letter_scale = fn_80189a60(writer);
                let glyph = platform::call2(
                    vtable_method(font, 0, 0x48),
                    p32(font),
                    character as u32,
                );
                x += glyph as f64 * letter_scale;
            } else {
                x += fn_80189cb0(writer);
            }
            x = (x as f32) as f64;

            let mut left = read_f32(rect);
            if (x as f32) < left {
                left = x as f32;
            }
            write_f32(rect, left);

            let mut right = read_f32(rect + 0x08);
            if right < (x as f32) {
                right = x as f32;
            }
            write_f32(rect + 0x08, right);
        }

        character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    }

    cursor.wrapping_sub(text) as i32
}

/// Measures or draws one wide character of a line and fills its rectangle.
///
/// The wide twin of [`fn_8018d540`]: the same line-extent seed, the same tag
/// dispatch through `writer + 0x5C` virtual `+0x10`, and the same glyph-width
/// advance. The two differences are that the character is a `u16` and that the
/// budget and returned count are `u16` character counts, so the byte distance
/// is halved.
///
/// # Original
/// - Ghidra: `FUN_8018FF00` at `0x8018FF00` (1012 bytes)
/// - C file: `work/ghidra/main-gb/export/8018ff00_FUN_8018ff00.c`
///
/// # Arguments
/// * `writer` - text-writer state.
/// * `rect` - four-float output rectangle.
/// * `text` - `u16` string.
/// * `length` - remaining character budget.
///
/// # Returns
/// The number of characters consumed from `text`.
///
/// # Side effects
/// Writes `rect + 0x00..0x10` and the writer cursor `+0x2C`.
///
/// # Safety
/// `text + length * 2` readable and `writer + 0x60` a valid text record.
///
/// # Confidence
/// medium - the body mirrors the char twin instruction for instruction; the
/// `/ 2` conversions are visible in the export.
pub unsafe fn fn_8018ff00(writer: usize, rect: usize, text: usize, length: i32) -> i32 {
    let font = fn_80188de0(writer) as usize;

    let mut tag_state = crate::platform::GuestLocal::new( [0u32; 5]);
    tag_state[0] = p32(writer);
    tag_state[1] = p32(text);

    write_f32(rect, 0.0);
    write_f32(rect + 0x08, 0.0);

    let mut height = if font != 0 {
        platform::call1(vtable_method(font, 0, 0x34), p32(font))
    } else {
        0
    };
    let scale = fn_80189a70(writer);
    let delta = (height as f64 * scale) as f32;
    let base = read_f32(writer + 0x50) + delta;
    let mut value = 0.0f32;
    if base < 0.0 {
        value = base;
    }
    write_f32(rect + 0x04, value);

    height = if font != 0 {
        platform::call1(vtable_method(font, 0, 0x34), p32(font))
    } else {
        0
    };
    let delta = (height as f64 * scale) as f32;
    let base = read_f32(writer + 0x50) + delta;
    value = 0.0f32;
    if 0.0 < base {
        value = base;
    }
    write_f32(rect + 0x0C, value);

    let mut cursor = text;
    let mut character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    let mut x = 0.0f64;
    let mut wrap = 0i32;

    loop {
        if cursor.wrapping_sub(text) as i32 / 2 > length {
            break;
        }
        if character < 0x20 {
            let mut local = crate::platform::GuestLocal::new( [0.0f32; 4]);
            local[0] = x as f32;
            local[1] = 0.0;
            local[2] = 0.0;
            local[3] = 0.0;
            tag_state[1] = p32(cursor);
            tag_state[4] = if wrap == 0 { 1 } else { 0 };
            fn_80189e10(x, writer);

            let source = read_u32(writer + 0x5C) as usize;
            let tag = platform::call4(
                vtable_method(source, 0, 0x10),
                p32(source),
                local.as_mut_ptr() as usize as u32,
                character as u32,
                tag_state.as_mut_ptr() as usize as u32,
            ) as i32;
            cursor = tag_state[1] as usize;

            let mut extent = read_f32(rect);
            if local[0] < extent {
                extent = local[0];
            }
            write_f32(rect, extent);

            extent = read_f32(rect + 0x04);
            if local[1] < extent {
                extent = local[1];
            }
            write_f32(rect + 0x04, extent);

            extent = read_f32(rect + 0x08);
            if extent < local[2] {
                extent = local[2];
            }
            write_f32(rect + 0x08, extent);

            extent = read_f32(rect + 0x0C);
            if extent < local[3] {
                extent = local[3];
            }
            write_f32(rect + 0x0C, extent);

            x = fn_80189e50(writer);
            if tag == 4 {
                break;
            }
            if tag == 1 {
                wrap = 0;
            } else if tag == 2 {
                wrap = 1;
            } else if tag == 3 {
                break;
            }
        } else {
            if wrap != 0 {
                x = (x as f32 + read_f32(writer + 0x4C)) as f64;
            }
            wrap = 1;
            if fn_80189ca0(writer) == 0 {
                let letter_scale = fn_80189a60(writer);
                let glyph = platform::call2(
                    vtable_method(font, 0, 0x48),
                    p32(font),
                    character as u32,
                );
                x += glyph as f64 * letter_scale;
            } else {
                x += fn_80189cb0(writer);
            }
            x = (x as f32) as f64;

            let mut left = read_f32(rect);
            if (x as f32) < left {
                left = x as f32;
            }
            write_f32(rect, left);

            let mut right = read_f32(rect + 0x08);
            if right < (x as f32) {
                right = x as f32;
            }
            write_f32(rect + 0x08, right);
        }

        character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    }

    cursor.wrapping_sub(text) as i32 / 2
}

/// Prints a char string, handling the writer's alignment flags.
///
/// The body has three stages:
///
/// 1. If the horizontal/vertical alignment nibbles in `writer + 0x58` are not
///    `0x300` and not zero, the string is measured with [`fn_8018d920`] into a
///    temporary rectangle and the caller's start position is shifted by the
///    full width/height (none, half or all, according to bits `0x30` and
///    `0x300`).
/// 2. The per-line alignment (`+0x58 & 3`) measures the first line with
///    [`fn_8018d540`] and shifts the x cursor by `(width - line_width) * 0.5`
///    for centre, `width - line_width` for right, or nothing for left.
/// 3. The y cursor is finalised: for bottom alignment (`0x300`) it keeps the
///    caller's value, otherwise it adds the line height from
///    [`fn_80189c30`].
///
/// # Original
/// - Ghidra: `FUN_8018DFF0` at `0x8018DFF0` (1260 bytes)
/// - C file: `work/ghidra/main-gb/export/8018dff0_FUN_8018dff0.c`
///
/// # Arguments
/// * `writer` - text-writer state; its cursor is updated.
/// * `x_ptr` - pointer to the caller's x position.
/// * `y_ptr` - pointer to the caller's y position.
/// * `text` - char string.
/// * `length` - character budget.
///
/// # Returns
/// The full measured width (`right + left`) as `f32`; zero when the alignment
/// nibble is `0x300` or zero.
///
/// # Side effects
/// Writes both position pointers, the writer cursor, and temporary clones.
///
/// # Safety
/// Both pointers must be writable for four bytes; `writer + 0x60` readable;
/// `text + length` readable.
///
/// # Confidence
/// medium - every branch, offset and call is exact; the 0x60-byte clone copy
/// is the same inline sequence as [`copy_text_state`], and the alignment
/// constants are the literals `0x30`/`0x300`.
pub unsafe fn fn_8018dff0(
    writer: usize,
    x_ptr: usize,
    y_ptr: usize,
    text: usize,
    length: i32,
) -> f32 {
    let flags = read_u32(writer + 0x58);
    let mut width = 0.0f32;
    let mut height = 0.0f32;

    let alignment = flags & 0x333;
    if alignment != 0x300 && alignment != 0 {
        let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
        let copy_ptr = copy.as_mut_ptr() as usize;
        copy_text_state(copy_ptr, writer);
        fn_8018d920(copy_ptr, rect.as_mut_ptr() as usize, text, length);
        let _ = fn_80188d90(copy_ptr, 0);
        width = rect[0] + rect[2];
        height = rect[1] + rect[3];
    }

    if (flags & 0x30) == 0x10 {
        write_f32(x_ptr, read_f32(x_ptr) - width * F32_802F5F38);
    } else if (flags & 0x30) == 0x20 {
        write_f32(x_ptr, read_f32(x_ptr) - width);
    }

    if (flags & 0x300) == 0x100 {
        write_f32(y_ptr, read_f32(y_ptr) - height * F32_802F5F38);
    } else if (flags & 0x300) == 0x200 {
        write_f32(y_ptr, read_f32(y_ptr) - height);
    }

    match flags & 3 {
        1 => {
            let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
            let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
            let copy_ptr = copy.as_mut_ptr() as usize;
            copy_text_state(copy_ptr, writer);
            fn_80189e00(0.0, 0.0, copy_ptr);
            fn_8018d540(copy_ptr, rect.as_mut_ptr() as usize, text, length);
            let line_width = rect[2] - rect[0];
            let _ = fn_80188d90(copy_ptr, 0);
            fn_80189e10(
                (read_f32(x_ptr) + (width - line_width) * F32_802F5F38) as f64,
                writer,
            );
        }
        2 => {
            let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
            let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
            let copy_ptr = copy.as_mut_ptr() as usize;
            copy_text_state(copy_ptr, writer);
            fn_80189e00(0.0, 0.0, copy_ptr);
            fn_8018d540(copy_ptr, rect.as_mut_ptr() as usize, text, length);
            let line_width = rect[2] - rect[0];
            let _ = fn_80188d90(copy_ptr, 0);
            fn_80189e10((read_f32(x_ptr) + (width - line_width)) as f64, writer);
        }
        _ => {
            fn_80189e10(read_f32(x_ptr) as f64, writer);
        }
    }

    if (flags & 0x300) == 0x300 {
        fn_80189e20(read_f32(y_ptr) as f64, writer);
    } else {
        fn_80189e20((read_f32(y_ptr) as f64 + fn_80189c30(writer)) as f64, writer);
    }

    width
}

/// Prints a char string with the writer's cursor.
///
/// The string is printed with [`fn_8018dff0`], which applies the alignment and
/// leaves the writer cursor at the end position. The difference between the
/// y position before and after the call is kept, and the function then walks
/// the string again to handle tag characters:
///
/// * tag result `3` measures the rest of the line with [`fn_8018d540`] and
///   shifts the x cursor by the line remainder (centre/right/left);
/// * tag results `1`/`2` set the wrap state, `4` ends the loop, `3` also ends
///   the loop in the printable branch.
///
/// Printable characters advance y by the negative line advance and draw the
/// glyph through [`fn_80189cc0`]. At the end the original y is restored.
///
/// # Original
/// - Ghidra: `FUN_8018DA40` at `0x8018DA40` (1452 bytes)
/// - C file: `work/ghidra/main-gb/export/8018da40_FUN_8018da40.c`
///
/// # Arguments
/// * `writer` - text-writer state.
/// * `text` - char string.
/// * `length` - character budget.
///
/// # Returns
/// The measured width as `f32`.
///
/// # Side effects
/// Writes the writer cursor, the glyph quad state and temporary clones.
///
/// # Safety
/// `text + length` readable and `writer + 0x60` a valid text record.
///
/// # Confidence
/// medium - the control flow, virtual calls and stores are exact; the tag
/// state block is an 8-byte `{writer, cursor}` pair, matching the two-word
/// state the export passes to the tag virtual.
pub unsafe fn fn_8018da40(writer: usize, text: usize, length: i32) -> f32 {
    let original_x = fn_80189e50(writer) as f32;
    let original_y = fn_80189e60(writer) as f32;
    let mut start_x = crate::platform::GuestLocal::new( original_x);
    let mut start_y = crate::platform::GuestLocal::new( original_y);
    let mut width = fn_8018dff0(
        writer,
        (start_x.as_mut_ptr() as *mut f32) as usize,
        (start_y.as_mut_ptr() as *mut f32) as usize,
        text,
        length,
    );
    let end_y = fn_80189e60(writer);
    let y_delta = (original_y as f64 - end_y) as f32;

    let base_x = start_x;

    let mut tag_state = crate::platform::GuestLocal::new( [p32(writer), p32(text)]);
    let mut cursor = text;
    let mut character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    let mut wrap = 0i32;

    loop {
        if cursor.wrapping_sub(text) as i32 > length {
            break;
        }
        if character < 0x20 {
            tag_state[1] = p32(cursor);
            let source = read_u32(writer + 0x5C) as usize;
            let tag = platform::call3(
                vtable_method(source, 0, 0x0C),
                p32(source),
                character as u32,
                tag_state.as_mut_ptr() as usize as u32,
            ) as i32;
            cursor = tag_state[1] as usize;

            if tag == 3 {
                let flags = read_u32(writer + 0x58);
                if (flags & 3) == 1 {
                    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
                    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
                    let copy_ptr = copy.as_mut_ptr() as usize;
                    copy_text_state(copy_ptr, writer);
                    fn_80189e00(0.0, 0.0, copy_ptr);
                    let consumed = fn_8018d540(
                        copy_ptr,
                        rect.as_mut_ptr() as usize,
                        cursor,
                        length.wrapping_sub(cursor.wrapping_sub(text) as i32),
                    );
                    let _ = consumed;
                    let line_width = rect[2] - rect[0];
                    let _ = fn_80188d90(copy_ptr, 0);
                    fn_80189e10(
                        (*base_x + (width - line_width) * F32_802F5F38) as f64,
                        writer,
                    );
                } else if (flags & 3) == 2 {
                    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
                    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
                    let copy_ptr = copy.as_mut_ptr() as usize;
                    copy_text_state(copy_ptr, writer);
                    fn_80189e00(0.0, 0.0, copy_ptr);
                    let consumed = fn_8018d540(
                        copy_ptr,
                        rect.as_mut_ptr() as usize,
                        cursor,
                        length.wrapping_sub(cursor.wrapping_sub(text) as i32),
                    );
                    let _ = consumed;
                    let line_width = rect[2] - rect[0];
                    let _ = fn_80188d90(copy_ptr, 0);
                    fn_80189e10((*base_x + (width - line_width)) as f64, writer);
                } else {
                    let x = fn_80189e50(writer);
                    if (width as f64) < (x - *base_x as f64) {
                        width = (x - *base_x as f64) as f32;
                    }
                    fn_80189e10(*base_x as f64, writer);
                }
                wrap = 0;
            } else if tag == 1 {
                wrap = 0;
            } else if tag == 2 {
                wrap = 1;
            } else if tag == 4 {
                break;
            }
        } else {
            let line_y = fn_80189e60(writer);
            if wrap != 0 {
                fn_80189e30(read_f32(writer + 0x4C) as f64, writer);
            }
            wrap = 1;
            let font_ptr = fn_80188de0(writer) as usize;
            let scale = fn_80189a70(writer);
            let advance =
                platform::call1(vtable_method(font_ptr, 0, 0x1C), p32(font_ptr)) as i32;
            fn_80189e40((-advance) as f64 * scale, writer);
            fn_80189cc0(writer, character as u32);
            fn_80189e20(line_y, writer);
        }

        character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    }

    if (read_u32(writer + 0x58) & 0x300) == 0x100
        || (read_u32(writer + 0x58) & 0x300) == 0x200
    {
        fn_80189e20(original_y as f64, writer);
    } else {
        fn_80189e40(y_delta as f64, writer);
    }

    width
}

/// Prints a wide string, handling the writer's alignment flags.
///
/// The wide twin of [`fn_8018da40`]: [`fn_801909f0`] applies the alignment,
/// the tag processor is called through virtual `+0x0C`, the line remainder is
/// measured with [`fn_8018ff00`], and every distance is divided by two because
/// the characters are `u16`.
///
/// # Original
/// - Ghidra: `FUN_80190420` at `0x80190420` (1488 bytes)
/// - C file: `work/ghidra/main-gb/export/80190420_FUN_80190420.c`
///
/// # Arguments
/// * `writer` - text-writer state.
/// * `text` - `u16` string.
/// * `length` - character budget.
///
/// # Returns
/// The measured width as `f32`.
///
/// # Side effects
/// Writes the writer cursor, the glyph quad state and temporary clones.
///
/// # Safety
/// `text + length * 2` readable and `writer + 0x60` a valid text record.
///
/// # Confidence
/// medium - the wide body mirrors the char twin; all `/ 2` conversions are
/// visible in the export.
pub unsafe fn fn_80190420(writer: usize, text: usize, length: i32) -> f32 {
    let original_x = fn_80189e50(writer) as f32;
    let original_y = fn_80189e60(writer) as f32;
    let mut start_x = crate::platform::GuestLocal::new( original_x);
    let mut start_y = crate::platform::GuestLocal::new( original_y);
    let mut width = fn_801909f0(
        writer,
        (start_x.as_mut_ptr() as *mut f32) as usize,
        (start_y.as_mut_ptr() as *mut f32) as usize,
        text,
        length,
    );
    let end_y = fn_80189e60(writer);
    let y_delta = (original_y as f64 - end_y) as f32;

    let base_x = start_x;

    let mut tag_state = crate::platform::GuestLocal::new( [p32(writer), p32(text)]);
    let mut cursor = text;
    let mut character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    let mut wrap = 0i32;

    loop {
        if cursor.wrapping_sub(text) as i32 / 2 > length {
            break;
        }
        if character < 0x20 {
            tag_state[1] = p32(cursor);
            let source = read_u32(writer + 0x5C) as usize;
            let tag = platform::call3(
                vtable_method(source, 0, 0x0C),
                p32(source),
                character as u32,
                tag_state.as_mut_ptr() as usize as u32,
            ) as i32;
            cursor = tag_state[1] as usize;

            if tag == 3 {
                let flags = read_u32(writer + 0x58);
                if (flags & 3) == 1 {
                    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
                    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
                    let copy_ptr = copy.as_mut_ptr() as usize;
                    copy_text_state(copy_ptr, writer);
                    fn_80189e00(0.0, 0.0, copy_ptr);
                    let consumed = fn_8018ff00(
                        copy_ptr,
                        rect.as_mut_ptr() as usize,
                        cursor,
                        length.wrapping_sub(cursor.wrapping_sub(text) as i32 / 2),
                    );
                    let _ = consumed;
                    let line_width = rect[2] - rect[0];
                    let _ = fn_80188d90(copy_ptr, 0);
                    fn_80189e10(
                        (*base_x + (width - line_width) * F32_802F5F38) as f64,
                        writer,
                    );
                } else if (flags & 3) == 2 {
                    let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
                    let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
                    let copy_ptr = copy.as_mut_ptr() as usize;
                    copy_text_state(copy_ptr, writer);
                    fn_80189e00(0.0, 0.0, copy_ptr);
                    let consumed = fn_8018ff00(
                        copy_ptr,
                        rect.as_mut_ptr() as usize,
                        cursor,
                        length.wrapping_sub(cursor.wrapping_sub(text) as i32 / 2),
                    );
                    let _ = consumed;
                    let line_width = rect[2] - rect[0];
                    let _ = fn_80188d90(copy_ptr, 0);
                    fn_80189e10((*base_x + (width - line_width)) as f64, writer);
                } else {
                    let x = fn_80189e50(writer);
                    if (width as f64) < (x - *base_x as f64) {
                        width = (x - *base_x as f64) as f32;
                    }
                    fn_80189e10(*base_x as f64, writer);
                }
                wrap = 0;
            } else if tag == 1 {
                wrap = 0;
            } else if tag == 2 {
                wrap = 1;
            } else if tag == 4 {
                break;
            }
        } else {
            let line_y = fn_80189e60(writer);
            if wrap != 0 {
                fn_80189e30(read_f32(writer + 0x4C) as f64, writer);
            }
            wrap = 1;
            let font_ptr = fn_80188de0(writer) as usize;
            let scale = fn_80189a70(writer);
            let advance =
                platform::call1(vtable_method(font_ptr, 0, 0x1C), p32(font_ptr)) as i32;
            fn_80189e40((-advance) as f64 * scale, writer);
            fn_80189cc0(writer, character as u32);
            fn_80189e20(line_y, writer);
        }

        character = platform::call1(FN_802DCBFC_NEXT_CHAR, p32(cursor)) as u16;
    }

    if (read_u32(writer + 0x58) & 0x300) == 0x100
        || (read_u32(writer + 0x58) & 0x300) == 0x200
    {
        fn_80189e20(original_y as f64, writer);
    } else {
        fn_80189e40(y_delta as f64, writer);
    }

    width
}

/// Aligns a wide string before printing and finalises the writer cursor.
///
/// The wide twin of [`fn_8018dff0`]: [`fn_80190300`] measures the full
/// rectangle, [`fn_8018ff00`] measures the first line for the per-line
/// alignment, and [`fn_80189c30`] supplies the line height for the y cursor.
///
/// # Original
/// - Ghidra: `FUN_801909F0` at `0x801909F0` (1260 bytes)
/// - C file: `work/ghidra/main-gb/export/801909f0_FUN_801909f0.c`
///
/// # Arguments
/// * `writer` - text-writer state; its cursor is updated.
/// * `x_ptr` - pointer to the caller's x position.
/// * `y_ptr` - pointer to the caller's y position.
/// * `text` - `u16` string.
/// * `length` - character budget.
///
/// # Returns
/// The full measured width (`right + left`) as `f32`.
///
/// # Side effects
/// Writes both position pointers, the writer cursor, and temporary clones.
///
/// # Safety
/// Both pointers writable for four bytes, `writer + 0x60` readable and
/// `text + length * 2` readable.
///
/// # Confidence
/// medium - the branches and every call match the char twin.
pub unsafe fn fn_801909f0(
    writer: usize,
    x_ptr: usize,
    y_ptr: usize,
    text: usize,
    length: i32,
) -> f32 {
    let flags = read_u32(writer + 0x58);
    let mut width = 0.0f32;
    let mut height = 0.0f32;

    let alignment = flags & 0x333;
    if alignment != 0x300 && alignment != 0 {
        let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
        let copy_ptr = copy.as_mut_ptr() as usize;
        copy_text_state(copy_ptr, writer);
        fn_80190300(copy_ptr, rect.as_mut_ptr() as usize, text, length);
        let _ = fn_80188d90(copy_ptr, 0);
        width = rect[0] + rect[2];
        height = rect[1] + rect[3];
    }

    if (flags & 0x30) == 0x10 {
        write_f32(x_ptr, read_f32(x_ptr) - width * F32_802F5F38);
    } else if (flags & 0x30) == 0x20 {
        write_f32(x_ptr, read_f32(x_ptr) - width);
    }

    if (flags & 0x300) == 0x100 {
        write_f32(y_ptr, read_f32(y_ptr) - height * F32_802F5F38);
    } else if (flags & 0x300) == 0x200 {
        write_f32(y_ptr, read_f32(y_ptr) - height);
    }

    match flags & 3 {
        1 => {
            let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
            let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
            let copy_ptr = copy.as_mut_ptr() as usize;
            copy_text_state(copy_ptr, writer);
            fn_80189e00(0.0, 0.0, copy_ptr);
            fn_8018ff00(copy_ptr, rect.as_mut_ptr() as usize, text, length);
            let line_width = rect[2] - rect[0];
            let _ = fn_80188d90(copy_ptr, 0);
            fn_80189e10(
                (read_f32(x_ptr) + (width - line_width) * F32_802F5F38) as f64,
                writer,
            );
        }
        2 => {
            let mut rect = crate::platform::GuestLocal::new( [0.0f32; 4]);
            let mut copy = crate::platform::GuestLocal::new( [0u8; 0x60]);
            let copy_ptr = copy.as_mut_ptr() as usize;
            copy_text_state(copy_ptr, writer);
            fn_80189e00(0.0, 0.0, copy_ptr);
            fn_8018ff00(copy_ptr, rect.as_mut_ptr() as usize, text, length);
            let line_width = rect[2] - rect[0];
            let _ = fn_80188d90(copy_ptr, 0);
            fn_80189e10((read_f32(x_ptr) + (width - line_width)) as f64, writer);
        }
        _ => {
            fn_80189e10(read_f32(x_ptr) as f64, writer);
        }
    }

    if (flags & 0x300) == 0x300 {
        fn_80189e20(read_f32(y_ptr) as f64, writer);
    } else {
        fn_80189e20((read_f32(y_ptr) as f64 + fn_80189c30(writer)) as f64, writer);
    }

    width
}

/// Fills `count` consecutive 0x14-byte transform records with `[0,0,0,1,1]`.
///
/// This is the inline fill sequence shared by the two loops of
/// [`fn_801819c0`]. Ghidra unrolls eight records per iteration; the Rust body
/// keeps that batch size so the pointer arithmetic and the trailing partial
/// batch match the original exactly.
///
/// # Original
/// Not a separate function: the two `do { ... } while` loops of
/// `FUN_801819C0` (`801819c0_FUN_801819c0.c`).
///
/// # Safety
/// `base + count * 0x14` must be writable.
///
/// # Confidence
/// high - the pattern words come from `0x802F5E68` (0.0) and `0x802F5E6C`
/// (1.0), and both loops have the same eight-record batch shape.
unsafe fn fill_transform_records(mut base: usize, count: u32) {
    if count == 0 {
        return;
    }

    let mut written = 0u32;
    if count > 8 {
        let batches = (count - 1) >> 3;
        if count != 8 {
            let mut remaining = batches;
            while remaining != 0 {
                let mut record = base;
                let mut index = 0;
                while index < 8 {
                    write_u32(record, 0);
                    write_u32(record + 0x04, 0);
                    write_u32(record + 0x08, 0);
                    write_u32(record + 0x0C, 1.0f32.to_bits());
                    write_u32(record + 0x10, 1.0f32.to_bits());
                    record += 0x14;
                    index += 1;
                }
                written += 8;
                base += 0xA0;
                remaining -= 1;
            }
        }
    }

    let mut rest = count - written;
    while rest != 0 {
        write_u32(base, 0);
        write_u32(base + 0x04, 0);
        write_u32(base + 0x08, 0);
        write_u32(base + 0x0C, 1.0f32.to_bits());
        write_u32(base + 0x10, 1.0f32.to_bits());
        base += 0x14;
        rest -= 1;
    }
}

/// Sizes and seeds the `BRLAN` animation work buffer.
///
/// The animation-transform object keeps its packed counts at `+0x4C`/`+0x50`
/// and the work buffer at `+0x58`. The function compares every requested
/// count against the packed fields; when any request is larger it frees the
/// old buffer, allocates `(texture patterns * 0x10) + (materials * 0x20) +
/// (texture SRTs * 0x14) + (colour/indirect counts * 4)` bytes, rewrites both
/// packed words and then seeds:
///
/// 1. one `[0,0,0,1,1]` record per material at `buffer + materials * 0x20`;
/// 2. one record per packed `0xC0000` count at the derived offset;
/// 3. five optional fixed records selected by bits `0x100`, `0x80`, `0x4000`,
///    `0x40` and `0x20` (glyph/edge colours and material colours).
///
/// # Original
/// - Ghidra: `FUN_801819C0` at `0x801819C0` (1688 bytes)
/// - C file: `work/ghidra/main-gb/export/801819c0_FUN_801819c0.c`
///
/// # Arguments
/// * `object` - animation-transform object.
/// * `c4`..`c10` - requested counts; the first seven are packed into `+0x4C`
///   (4/4/4/5/1/3/2 bits), the rest select the optional records.
/// * `c11`..`c14` - the four stack bytes packed into bits 8/7/6/5.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Frees and allocates through `0x801D5550`/`0x801D5540`, rewrites `+0x4C`,
/// `+0x50` and `+0x58`, and writes the record arrays.
///
/// # Safety
/// `object + 0x60` readable/writable; the heap handle at `0x80378008` must be
/// valid; the requested counts must keep the size below the heap limit.
///
/// # Confidence
/// medium - every test, packed bit and store is copied from the export; the
/// count names are unknown (the object layout only names them by bit position).
pub unsafe fn fn_801819c0(
    object: usize,
    c4: u32,
    c5: u32,
    c6: u32,
    c7: u32,
    c8: u32,
    c9: u32,
    c10: u32,
    c11: u32,
    c12: u32,
    c13: u32,
    c14: u32,
) {
    let counts = read_u32(object + 0x4C);
    let needs_buffer = (counts >> 28 < c4)
        || ((counts >> 24 & 0xF) < c5)
        || ((counts >> 20 & 0xF) < c6)
        || ((counts >> 9 & 0x1F) < c7)
        || ((counts >> 14 & 1) < c8)
        || ((counts >> 15 & 7) < c9)
        || ((counts >> 18 & 3) < c10)
        || ((counts >> 8 & 1) < (c11 & 0xFF))
        || ((counts >> 7 & 1) < (c12 & 0xFF))
        || ((counts >> 6 & 1) < (c13 & 0xFF))
        || ((counts >> 5 & 1) < (c14 & 0xFF));

    if !needs_buffer {
        return;
    }

    let old_buffer = read_u32(object + 0x58) as usize;
    if old_buffer != 0 {
        platform::call2(
            FN_801D5550_FREE,
            read_u32(HEAP_80378008),
            p32(old_buffer),
        );
        write_u32(object + 0x58, 0);
        write_u32(object + 0x4C, read_u32(object + 0x4C) & 0x1F);
        write_u32(object + 0x50, read_u32(object + 0x50) & 0x1F);
    }

    let size = (c7 & 0xFF)
        .wrapping_mul(0x10)
        .wrapping_add((c4 & 0xFF).wrapping_mul(0x20))
        .wrapping_add(c10.wrapping_add(c5).wrapping_mul(0x14))
        .wrapping_add(
            c14.wrapping_add(c12)
                .wrapping_add(c9)
                .wrapping_add(c6)
                .wrapping_add(c13)
                .wrapping_add(c8)
                .wrapping_add(c11)
                .wrapping_mul(4),
        );
    let buffer = platform::call2(FN_801D5540_ALLOC, read_u32(HEAP_80378008), size);
    write_u32(object + 0x58, buffer);
    let buffer = buffer as usize;
    if buffer == 0 {
        return;
    }

    let high = (c5 & 0xF) << 0x18;
    let packed = ((c14 & 1) << 5)
        | ((c13 & 1) << 6)
        | ((c12 & 1) << 7)
        | ((c11 & 1) << 8)
        | ((c7 & 0x1F) << 9)
        | ((c8 & 1) << 0x0E)
        | ((c9 & 7) << 0x0F)
        | ((c10 & 3) << 0x12)
        | ((c6 & 0xF) << 0x14)
        | high
        | (c4 << 0x1C)
        | (read_u32(object + 0x4C) & 0x1F);
    write_u32(object + 0x4C, packed);
    write_u32(
        object + 0x50,
        high | (read_u32(object + 0x50) & 0xF0FF_FFFF),
    );

    fill_transform_records(
        buffer + (c4 & 0xF) as usize * 0x20,
        c5 & 0xF,
    );

    let flags = read_u32(object + 0x4C);
    let mask = flags & 0x000C_0000;
    write_u32(object + 0x50, mask | (read_u32(object + 0x50) & 0xFFF3_FFFF));
    let base = buffer
        + ((flags >> 0x0C) & 4) as usize
        + ((flags >> 0x05) & 4) as usize
        + ((flags >> 0x04) & 4) as usize
        + ((flags >> 0x06) & 4) as usize
        + ((flags >> 0x18) & 0x0F) as usize * 0x14
        + ((flags >> 0x12) & 0x3C) as usize
        + ((flags >> 0x17) & 0x1E0) as usize
        + ((flags >> 0x0D) & 0x1C) as usize
        + ((flags >> 0x03) & 4) as usize;
    fill_transform_records(base, mask >> 0x12);

    let flags = read_u32(object + 0x4C);
    write_u32(
        object + 0x50,
        (flags & 0x100) | (read_u32(object + 0x50) & !0x100u32),
    );
    if (flags & 0x100) != 0 {
        let p = buffer
            + ((flags >> 0x18) & 0x0F) as usize * 0x14
            + ((flags >> 0x12) & 0x3C) as usize
            + ((flags >> 0x17) & 0x1E0) as usize;
        write_u8(p, 1);
        write_u8(p + 1, 1);
        write_u8(p + 2, 0);
        write_u8(p + 3, 0);
    }

    let flags = read_u32(object + 0x4C);
    write_u32(
        object + 0x50,
        (flags & 0x80) | (read_u32(object + 0x50) & !0x80u32),
    );
    if (flags & 0x80) != 0 {
        let p = buffer
            + ((flags >> 0x06) & 4) as usize
            + ((flags >> 0x18) & 0x0F) as usize * 0x14
            + ((flags >> 0x12) & 0x3C) as usize
            + ((flags >> 0x17) & 0x1E0) as usize;
        write_u32(p, 0xFFFF_FFFF);
    }

    let flags = read_u32(object + 0x4C);
    write_u32(
        object + 0x50,
        (flags & 0x4000) | (read_u32(object + 0x50) & !0x4000u32),
    );
    if (flags & 0x4000) != 0 {
        let p = buffer
            + ((flags >> 0x05) & 4) as usize
            + ((flags >> 0x06) & 4) as usize
            + ((flags >> 0x18) & 0x0F) as usize * 0x14
            + ((flags >> 0x12) & 0x3C) as usize
            + ((flags >> 0x17) & 0x1E0) as usize;
        write_u8(p, 0xE4);
        write_u8(p + 1, 0xC0);
        write_u8(p + 2, 0xD5);
        write_u8(p + 3, 0xEA);
    }

    let flags = read_u32(object + 0x4C);
    write_u32(
        object + 0x50,
        (flags & 0x40) | (read_u32(object + 0x50) & !0x40u32),
    );
    if (flags & 0x40) != 0 {
        let p = buffer
            + ((flags >> 0x06) & 4) as usize
            + ((flags >> 0x0C) & 4) as usize
            + ((flags >> 0x05) & 4) as usize
            + ((flags >> 0x18) & 0x0F) as usize * 0x14
            + ((flags >> 0x12) & 0x3C) as usize
            + ((flags >> 0x17) & 0x1E0) as usize;
        write_u8(p, 0x77);
        write_u8(p + 1, 0);
        write_u8(p + 2, 0);
        write_u8(p + 3, 0);
    }

    let flags = read_u32(object + 0x4C);
    write_u32(
        object + 0x50,
        (flags & 0x20) | (read_u32(object + 0x50) & !0x20u32),
    );
    if (flags & 0x20) != 0 {
        let p = buffer
            + ((flags >> 0x0C) & 4) as usize
            + ((flags >> 0x05) & 4) as usize
            + ((flags >> 0x04) & 4) as usize
            + ((flags >> 0x06) & 4) as usize
            + ((flags >> 0x18) & 0x0F) as usize * 0x14
            + ((flags >> 0x12) & 0x3C) as usize
            + ((flags >> 0x17) & 0x1E0) as usize;
        write_u8(p, 1);
        write_u8(p + 1, 4);
        write_u8(p + 2, 5);
        write_u8(p + 3, 0x0F);
    }
}

/// Emits the common first half of the default font texture GX state.
///
/// Used by every branch of [`fn_80188df0`] except the `format < 2` branch,
/// which inserts four TEV calls before the tail. The selector word comes from
/// `0x80378034`, with the one-time initialisation flag at `0x80378030`.
///
/// # Original
/// Not a separate function: the repeated block in `FUN_80188DF0`
/// (`80188df0_FUN_80188df0.c`).
///
/// # Side effects
/// May set `0x80378030`/`0x80378034` and issues the GX calls through
/// `0x801C8920..0x801C8790`.
///
/// # Safety
/// The target image must map the GX command entry points.
///
/// # Confidence
/// medium - the calls and literals are copied exactly; the GX field names are
/// unknown.
unsafe fn gx_font_state_prefix() {
    if read_u8(0x8037_8030) == 0 {
        write_u8(0x8037_8034, 0);
        write_u8(0x8037_8030, 1);
    }

    let mut selector = crate::platform::GuestLocal::new( [read_u8(0x8037_8034) as u32]);
    let _ = call_f32x4_u32_ptr(
        FN_801C8920,
        0.0,
        0.0,
        0.0,
        0.0,
        0,
        selector.as_mut_ptr() as u32,
    );
    let _ = platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    let _ = platform::call3(FN_801C8700, 0, 0x11, 0);
    let _ = platform::call1(FN_801C6AC0, 1);
    let _ = platform::call7(FN_801C6AF0, 4, 0, 0, 1, 0, 0, 2);
    let _ = platform::call7(FN_801C6AF0, 5, 0, 0, 0, 0, 0, 2);
    let _ = platform::call1(FN_801C4A20, 1);
    let _ = platform::call6(FN_801C47D0, 0, 1, 4, 0x3C, 0, 0x7D);
    let _ = platform::call1(FN_801C8110, 0);
    let _ = platform::call4(FN_801C8DC0, 1, 4, 5, 0xF);
    let _ = platform::call1(FN_801C88F0, 1);
    let _ = platform::call1(FN_801C8130, 0);
    let _ = platform::call3(FN_801C8600, 0, 0, 0);
    let _ = platform::call4(FN_801C8790, 0, 0, 0, 4);
}

/// Emits the common last half of the default font texture GX state.
///
/// # Original
/// Not a separate function: the tail shared by all branches of
/// `FUN_80188DF0` (`80188df0_FUN_80188df0.c`).
///
/// # Side effects
/// Issues the three `0x801C40B0` pair writers, `0x801C4070` and the three
/// `0x801C37C0` enables.
///
/// # Safety
/// The target image must map the GX command entry points.
///
/// # Confidence
/// medium - literals copied from the export.
unsafe fn gx_font_state_tail() {
    let _ = platform::call5(FN_801C40B0, 0, 9, 1, 4, 0);
    let _ = platform::call5(FN_801C40B0, 0, 0xB, 1, 5, 0);
    let _ = platform::call5(FN_801C40B0, 0, 0xD, 1, 2, 0xF);
    platform::call0(FN_801C4070);
    let _ = platform::call2(FN_801C37C0, 9, 1);
    let _ = platform::call2(FN_801C37C0, 0xB, 1);
    let _ = platform::call2(FN_801C37C0, 0xD, 1);
}

/// Programs the default GX texture state for a font glyph texture.
///
/// The descriptor's first two words are checked against `(0, -1)`. When they
/// match, the texture object at `+0x48` selects one of four format branches
/// (`< 2`, `2..3`, `4..6`, everything else); every branch programs the same
/// base state, the indexed formats only replace the last TEV calls with the
/// four `0x801C82F0`/`0x801C8330`/`0x801C8370`/`0x801C83D0` setups. When the
/// descriptor does not match, the two colour words are handed to
/// [`fn_8018a100`], which programs the full font TEV state.
///
/// # Original
/// - Ghidra: `FUN_80188DF0` at `0x80188DF0` (2384 bytes)
/// - C file: `work/ghidra/main-gb/export/80188df0_FUN_80188df0.c`
///
/// # Arguments
/// * `descriptor` - texture descriptor: `+0x00`/`+0x04` colour pair and
///   `+0x48` texture object pointer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two cache words at `0x80378038`/`0x8037803C` and issues the GX
/// state calls.
///
/// # Safety
/// `descriptor + 0x4C` readable and the texture object must have a valid
/// vtable when it is non-null.
///
/// # Confidence
/// medium - branch conditions and every literal call are exact; the GX field
/// names are unknown and are documented by address only.
pub unsafe fn fn_80188df0(descriptor: usize) {
    write_u32(0x8037_8038, 0xFF);
    write_u32(0x8037_803C, 0);

    let first = read_u32(descriptor);
    let second = read_u32(descriptor + 0x04);
    if first == 0 && second == 0xFFFF_FFFF {
        let texture = read_u32(descriptor + 0x48) as usize;
        if texture == 0 {
            gx_font_state_prefix();
            let _ = platform::call2(FN_801C8250, 0, 0);
            gx_font_state_tail();
        } else {
            let format =
                platform::call1(vtable_method(texture, 0, 0x30), p32(texture)) as u32;
            if format.wrapping_sub(4) < 3 {
                gx_font_state_prefix();
                let _ = platform::call2(FN_801C8250, 0, 0);
                gx_font_state_tail();
            } else if format < 2 {
                gx_font_state_prefix();
                let _ = platform::call5(FN_801C82F0, 0, 0xF, 0xF, 0xF, 0xA);
                let _ = platform::call5(FN_801C8330, 0, 7, 4, 5, 7);
                let _ = platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
                let _ = platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
                gx_font_state_tail();
            } else if format.wrapping_sub(2) < 2 {
                gx_font_state_prefix();
                let _ = platform::call2(FN_801C8250, 0, 0);
                gx_font_state_tail();
            } else {
                gx_font_state_prefix();
                let _ = platform::call2(FN_801C8250, 0, 0);
                gx_font_state_tail();
            }
        }
    } else {
        let mut colours = crate::platform::GuestLocal::new( [first, second]);
        fn_8018a100(
            colours.as_mut_ptr() as usize,
            colours.as_mut_ptr().wrapping_add(1) as usize,
        );
    }
}

// ===========================================================================
// Wave 23: text-window character-cell layout
// (0x80190FB0..0x80193EF0, plus the layout angle table at 0x801886A0)
// ===========================================================================
//
// A 100-byte window record is addressed by `0x803780EC + slot * 100`, where
// `slot` is the signed byte at `0x80378088 + message_id` (the inverse table of
// `crate::runtime::ids`). The record fields named below are the ones the
// translated helpers read or write; the layout builders `0x80191260` and
// `0x80191D20` (translated in the wave-30 section below) fill in the remaining
// character cells.

/// `0x803780E8`: record mode; `2` selects the alternate colour/offset tables.
const WND_MODE: usize = 0x8037_80E8;
/// `0x803780EC`: pointer to the first of sixteen 100-byte window records.
const WND_RECORDS: usize = 0x8037_80EC;
/// `0x80378088`: inverse id -> slot table (signed byte per id).
const WND_SLOT_TABLE: usize = 0x8037_8088;
/// `0x80378738`: head of the global message list (`+0x00` is the next link).
const WND_LIST_HEAD: usize = 0x8037_8738;
/// `0x8037873C`: pointer to the first 0x4C-byte message record.
const WND_MESSAGE_TABLE: usize = 0x8037_873C;
/// `0x80378740`: static array of sixteen 0x4C-byte message records.
const WND_STATIC_TABLE: usize = 0x8037_8740;
/// `0x80378C00`: one-time guard for the window table.
const WND_INIT_GUARD: usize = 0x8037_8C00;

/// `0x8032D0D0`: default first table, one word per record index (0..0x7F).
const WND_TABLE_D0D0: usize = 0x8032_D0D0;
/// `0x8032D2CC`: default word stored at record `+0x24` (`0xFFFFFC78`).
const WND_WORD_D2CC: usize = 0x8032_D2CC;
/// `0x8032D2D0`: alternate first table, one signed halfword per record index.
const WND_TABLE_D2D0: usize = 0x8032_D2D0;
/// `0x8032D3CE`: alternate halfword stored at record `+0x24`.
const WND_HALF_D3CE: usize = 0x8032_D3CE;
/// `0x8032D3D0`: alternate signed-halfword table.
const WND_TABLE_D3D0: usize = 0x8032_D3D0;
/// `0x8032D6D0`: signed position table used by the command handlers.
const WND_POS_TABLE_D6D0: usize = 0x8032_D6D0;
/// `0x8032D860`: second position table used by [`fn_80192d80`].
const WND_POS_TABLE_D860: usize = 0x8032_D860;
/// `0x8032D8D0`: per-frame state table used by [`fn_801934f0`].
const WND_STATE_TABLE_D8D0: usize = 0x8032_D8D0;
/// `0x8032DA60`: percent scale table (100 entries).
const WND_PERCENT_TABLE: usize = 0x8032_DA60;
/// `0x8032DBF0`: minute scale table.
const WND_MINUTE_TABLE: usize = 0x8032_DBF0;
/// `0x8032DC20`: positive second scale table.
const WND_SECOND_TABLE: usize = 0x8032_DC20;
/// `0x8032DC50`: negative second scale table (index counted downwards).
const WND_SECOND_NEG_TABLE: usize = 0x8032_DC50;
/// `0x8032C5D8`: `{angle, slope}` pairs, 33 entries of 8 bytes.
const WND_ATAN_TABLE: usize = 0x8032_C5D8;

/// `0x801B1140`: acquires the message-list lock.
const FN_801B1140: usize = 0x801B_1140;
/// `0x801B1180`: releases the message-list lock.
const FN_801B1180: usize = 0x801B_1180;
/// `0x801A00B0`: returns the message-system singleton (SDA `r13 - 0x5188`).
const FN_801A00B0: usize = 0x801A_00B0;
/// `0x801A0300`: releases one message object.
const FN_801A0300: usize = 0x801A_0300;
/// `0x801928D0`: frees one id of the shared 16-slot id table.
const FN_801928D0: usize = 0x8019_28D0;
/// `0x80193F00`: retires one message record by index.
const FN_80193F00: usize = 0x8019_3F00;
/// `0x80192E60`: creates one window record (body in the wave-25 section).
const FN_80192E60: usize = 0x8019_2E60;
/// `0x801939C0`: mode-0 window update (body in the wave-25 section).
const FN_801939C0: usize = 0x8019_39C0;
/// `0x80193BE0`: mode-1 window update (body in the wave-25 section).
const FN_80193BE0: usize = 0x8019_3BE0;
/// `0x80193CF0`: mode-2 window update (body in the wave-25 section).
const FN_80193CF0: usize = 0x8019_3CF0;

/// Reads a big-endian signed halfword.
///
/// # Original
/// Host-side helper; models the `lha` loads used by [`fn_80190fb0`],
/// [`fn_80193850`] and [`fn_80193e00`].
///
/// # Safety
/// `addr..addr + 2` must be readable.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn read_i16(addr: usize) -> i16 {
    read_u16(addr) as i16
}

/// Adds a signed element index to a base address.
///
/// # Original
/// Host-side helper; models the `base + index * stride` address arithmetic the
/// table lookups below emit. Signed indices are preserved, so a negative index
/// addresses before `base` exactly as the original does.
///
/// # Safety
/// The resulting address is only computed, not dereferenced.
///
/// # Confidence
/// high - mechanical arithmetic.
#[inline]
fn table_entry(base: usize, index: i32, stride: isize) -> usize {
    base.wrapping_add_signed(index as isize * stride)
}

/// Resolves a message id to its 100-byte window record address.
///
/// The id is read from `message + 0x18`, looked up in the signed inverse table
/// at `0x80378088`, and scaled by the observed 100-byte record stride. No
/// range check is performed, matching the original.
///
/// # Original
/// Host-side helper used by [`fn_80191bb0`], [`fn_80191be0`],
/// [`fn_80191c20`], [`fn_80191c60`] and [`fn_80191ce0`]; the address arithmetic
/// is the repeated `0x803780EC + (char)table[id] * 100` load.
///
/// # Safety
/// `message + 0x18` must be readable and the resolved address writable.
///
/// # Confidence
/// high - identical expression at all five call sites.
#[inline]
unsafe fn window_record(message: usize) -> usize {
    let id = read_u32(message + 0x18) as i32;
    let slot = read_u8(WND_SLOT_TABLE.wrapping_add_signed(id as isize)) as i8 as isize;
    (read_u32(WND_RECORDS) as isize + slot * 100) as usize
}

/// Returns `atan2(x, y)` in 1/256-turn units, table interpolated.
///
/// Zero is the `+Y` axis, so the result matches the sine/cosine table used by
/// [`fn_801885b0`]/[`fn_80188630`]: `(0, 1)` is `0`, `(1, 0)` is `64`
/// (90 degrees) and `(-1, 0)` is `-64`. The body reduces the argument to
/// `min / max`, scales it by 32, splits the scaled value into a table index and
/// an 8-bit fraction, and adds or subtracts the interpolated table angle from
/// a quadrant base of `0`, `+/-64` or `+128`.
///
/// # Original
/// - Ghidra: `FUN_801886A0` at `0x801886A0` (448 bytes)
/// - C file: `work/ghidra/main-gb/export/801886a0_FUN_801886a0.c`
///
/// # Arguments
/// * `x` - horizontal component (`param_1` in the Ghidra output).
/// * `y` - vertical component (`param_2`); this is the zero axis.
///
/// # Returns
/// Angle in 1/256-turn units as `f64` (one full turn is `256.0`).
///
/// # Side effects
/// None; the table at `0x8032C5D8` is read-only.
///
/// # Safety
/// The target image must be mapped at `0x8032C5D8`.
///
/// # Confidence
/// medium - the quadrant branches and table stride are exact, but the original
/// computes the fraction through a paired-single 8-bit quantised store; this
/// body uses the same truncated integer index and the exact single-precision
/// fraction, which is the same model as the wave-20 sine/cosine lookups.
pub unsafe fn fn_801886a0(x: f64, y: f64) -> f64 {
    let zero = 0.0f64;
    if x == zero && y == zero {
        return zero;
    }

    // (base, minor, major, subtract); the C code calls the ratio minor / major.
    let (base, minor, major, subtract) = if y < zero {
        if x < zero {
            let abs_x = -x;
            let abs_y = -y;
            if abs_y < abs_x {
                (-64.0f64, abs_y, abs_x, true)
            } else {
                (-128.0f64, abs_x, abs_y, false)
            }
        } else if -y < x {
            (64.0f64, -y, x, false)
        } else {
            (128.0f64, x, -y, true)
        }
    } else if x < zero {
        let abs_x = -x;
        if y < abs_x {
            (-64.0f64, y, abs_x, false)
        } else {
            (0.0f64, abs_x, y, true)
        }
    } else if y < x {
        (64.0f64, y, x, true)
    } else {
        (0.0f64, x, y, false)
    };

    // observed: `(float)(minor / major) * 32.0`; the low byte of the quantised
    // result is the table index.
    let scaled = ((minor / major) as f32) * 32.0;
    let index = (scaled as u32 & 0xFF) as i32;
    let fraction = scaled - index as f32;

    let entry = table_entry(WND_ATAN_TABLE, index, 8);
    let delta = read_f32(entry) + fraction * read_f32(entry + 0x04);
    let angle = if subtract {
        base - delta as f64
    } else {
        base + delta as f64
    };
    angle as f32 as f64
}

/// Selects and stores the two colour/offset tables of one window record.
///
/// The record's `+0x10` word is an index into a 128-entry table. The body
/// stores the entry at that index and its mirror (`0x7F - index`) at
/// `+0x18`/`+0x1C`, the first table halfword at `+0x20`, a sign-extended bias
/// at `+0x24`, and two signed halfwords at `+0x28`/`+0x2C`. Mode `2` uses the
/// signed-halfword table pair `0x8032D2D0`/`0x8032D3D0`; the default branch
/// uses the word table `0x8032D0D0` and stores zero in the two halfword slots.
///
/// # Original
/// - Ghidra: `FUN_80190FB0` at `0x80190FB0` (160 bytes)
/// - C file: `work/ghidra/main-gb/export/80190fb0_FUN_80190fb0.c`
///
/// # Arguments
/// * `record` - 100-byte window record; `+0x10` is the table index.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `record + 0x18..0x30` and reads `0x803780E8` plus the selected table.
///
/// # Safety
/// `record` must point to at least `0x30` writable bytes and the index must
/// stay inside the 128-entry tables.
///
/// # Confidence
/// high - both branches and every table base are explicit; only the semantic
/// name of the tables is inferred.
pub unsafe fn fn_80190fb0(record: usize) {
    let index = read_i32(record + 0x10);
    let mirror = 0x7F - index;

    if read_u32(WND_MODE) == 2 {
        // observed: the mode-2 tables are signed halfwords (lhax), not words.
        write_u32(
            record + 0x18,
            read_i16(table_entry(WND_TABLE_D2D0, index, 2)) as i32 as u32,
        );
        write_u32(
            record + 0x1C,
            read_i16(table_entry(WND_TABLE_D2D0, mirror, 2)) as i32 as u32,
        );
        // observed: +0x20 and +0x24 take sign-extended halfwords (`lha`).
        write_u32(record + 0x20, read_i16(WND_TABLE_D2D0) as i32 as u32);
        write_u32(record + 0x24, read_i16(WND_HALF_D3CE) as i32 as u32);
        write_u32(
            record + 0x28,
            read_i16(table_entry(WND_TABLE_D3D0, mirror, 2)) as i32 as u32,
        );
        write_u32(
            record + 0x2C,
            read_i16(table_entry(WND_TABLE_D3D0, index, 2)) as i32 as u32,
        );
    } else {
        write_u32(record + 0x18, read_u32(table_entry(WND_TABLE_D0D0, index, 4)));
        write_u32(
            record + 0x1C,
            read_u32(table_entry(WND_TABLE_D0D0, mirror, 4)),
        );
        // observed: +0x20 is the first word of the table (0) and +0x24 is the
        // word immediately before it (0xFFFFFC78).
        write_u32(record + 0x20, read_u32(WND_TABLE_D0D0));
        write_u32(record + 0x24, read_u32(WND_WORD_D2CC));
        write_u32(record + 0x28, 0);
        write_u32(record + 0x2C, 0);
    }
}

/// Clears the live-object pointer of one window record.
///
/// # Original
/// - Ghidra: `FUN_80191BB0` at `0x80191BB0` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/80191bb0_FUN_80191bb0.c`
///
/// # Arguments
/// * `message` - object whose `+0x18` word is the message id.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes zero at the start of the record selected by [`window_record`].
///
/// # Safety
/// The id must resolve to a writable record.
///
/// # Confidence
/// high - the expression is the shared record-address form and one store.
pub unsafe fn fn_80191bb0(message: usize) {
    write_u32(window_record(message), 0);
}

/// Stores a word at record `+0x08` and marks the record dirty.
///
/// # Original
/// - Ghidra: `FUN_80191BE0` at `0x80191BE0` (56 bytes)
/// - C file: `work/ghidra/main-gb/export/80191be0_FUN_80191be0.c`
///
/// # Arguments
/// * `message` - object whose `+0x18` word is the message id.
/// * `value` - word stored at `record + 0x08`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets bit `0x10000000` in the record's `+0x04` flag word.
///
/// # Safety
/// The id must resolve to a writable record.
///
/// # Confidence
/// high - one store and one OR, both offsets explicit.
pub unsafe fn fn_80191be0(message: usize, value: u32) {
    let record = window_record(message);
    write_u32(record + 0x08, value);
    write_u32(record + 0x04, read_u32(record + 0x04) | 0x1000_0000);
}

/// Stores a word at record `+0x0C` and marks the record dirty.
///
/// # Original
/// - Ghidra: `FUN_80191C20` at `0x80191C20` (56 bytes)
/// - C file: `work/ghidra/main-gb/export/80191c20_FUN_80191c20.c`
///
/// # Arguments
/// * `message` - object whose `+0x18` word is the message id.
/// * `value` - word stored at `record + 0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets bit `0x40000000` in the record's `+0x04` flag word.
///
/// # Safety
/// The id must resolve to a writable record.
///
/// # Confidence
/// high - one store and one OR, both offsets explicit.
pub unsafe fn fn_80191c20(message: usize, value: u32) {
    let record = window_record(message);
    write_u32(record + 0x0C, value);
    write_u32(record + 0x04, read_u32(record + 0x04) | 0x4000_0000);
}

/// Stores a clamped byte at record `+0x10`, then rebuilds the record tables.
///
/// The input is clamped to `0..0x7F` (negative becomes `0`, values above
/// `0x7F` become `0x7F`), stored, and the body runs [`fn_80190fb0`] and marks
/// the record dirty.
///
/// # Original
/// - Ghidra: `FUN_80191C60` at `0x80191C60` (128 bytes)
/// - C file: `work/ghidra/main-gb/export/80191c60_FUN_80191c60.c`
///
/// # Arguments
/// * `message` - object whose `+0x18` word is the message id.
/// * `value` - requested table index.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `record + 0x10`, rewrites `record + 0x18..0x30` through
/// [`fn_80190fb0`], and sets bit `0x40000000` in `record + 0x04`.
///
/// # Safety
/// The id must resolve to a writable record.
///
/// # Confidence
/// high - clamp, store, helper call and flag are all explicit.
pub unsafe fn fn_80191c60(message: usize, value: i32) {
    let record = window_record(message);
    let clamped = if value < 0 {
        0
    } else if value < 0x80 {
        value
    } else {
        0x7F
    };
    write_u32(record + 0x10, clamped as u32);
    fn_80190fb0(record);
    write_u32(record + 0x04, read_u32(record + 0x04) | 0x4000_0000);
}

/// Stores a word at record `+0x14` and marks the record dirty.
///
/// # Original
/// - Ghidra: `FUN_80191CE0` at `0x80191CE0` (56 bytes)
/// - C file: `work/ghidra/main-gb/export/80191ce0_FUN_80191ce0.c`
///
/// # Arguments
/// * `message` - object whose `+0x18` word is the message id.
/// * `value` - word stored at `record + 0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets bit `0x40000000` in the record's `+0x04` flag word.
///
/// # Safety
/// The id must resolve to a writable record.
///
/// # Confidence
/// high - one store and one OR, both offsets explicit.
pub unsafe fn fn_80191ce0(message: usize, value: u32) {
    let record = window_record(message);
    write_u32(record + 0x14, value);
    write_u32(record + 0x04, read_u32(record + 0x04) | 0x4000_0000);
}

/// Removes one message from the global singly-linked list at `0x80378738`.
///
/// The walk keeps the previous node and relinks it past `message`; when the
/// removed node is the head the first iteration only advances, which is why the
/// original stores the last visited node back into the head afterwards. The
/// list is updated under the `0x801B1140`/`0x801B1180` lock pair.
///
/// # Original
/// - Ghidra: `FUN_80192900` at `0x80192900` (140 bytes)
/// - C file: `work/ghidra/main-gb/export/80192900_FUN_80192900.c`
///
/// # Arguments
/// * `message` - node to unlink; it is matched by address.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears the predecessor's next word, terminates the list and rewrites the
/// head when the removed node was not the only one.
///
/// # Safety
/// Every node in the list must be readable through `+0x00`.
///
/// # Confidence
/// high - the loop and the final head store are exact; the lock callees are
/// documented at their addresses.
pub unsafe fn fn_80192900(message: usize) {
    platform::call0(FN_801B1140);
    let mut previous = 0usize;
    let mut first = 0usize;
    let mut node = read_u32(WND_LIST_HEAD) as usize;
    while node != 0 {
        let next = read_u32(node) as usize;
        if node != message {
            if first == 0 {
                // observed: the first surviving node becomes the new head and
                // is never replaced, even when later nodes are relinked.
                first = node;
            } else {
                write_u32(previous, node as u32);
            }
            previous = node;
        }
        node = next;
    }
    if previous != 0 {
        write_u32(previous, 0);
    }
    write_u32(WND_LIST_HEAD, first as u32);
    platform::call0(FN_801B1180);
}

/// Resets the sixteen message records and the global message list.
///
/// Runs only when the message singleton is live and the guard at `0x80378C00`
/// is clear. It points `0x8037873C` at the static record array, clears the
/// `+0x08` owner word of all sixteen 0x4C-byte records, empties the list head
/// and sets the guard.
///
/// # Original
/// - Ghidra: `FUN_80192990` at `0x80192990` (216 bytes)
/// - C file: `work/ghidra/main-gb/export/80192990_FUN_80192990.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the message table pointer, sixteen owner words, the list head and the
/// one-time guard.
///
/// # Safety
/// Call once per process after the message singleton exists.
///
/// # Confidence
/// high - the unrolled stores are a regular 0x4C stride; the loop reproduces
/// the same sixteen addresses.
pub unsafe fn fn_80192990() {
    if call0_r(FN_801A00B0) != 0 && read_u32(WND_INIT_GUARD) == 0 {
        write_u32(WND_MESSAGE_TABLE, WND_STATIC_TABLE as u32);
        for index in 0..16 {
            write_u32(WND_STATIC_TABLE + 0x08 + index * 0x4C, 0);
        }
        write_u32(WND_LIST_HEAD, 0);
        write_u32(WND_INIT_GUARD, 1);
    }
}

/// Retires every message record and every node of the global list.
///
/// The first loop calls `0x80193F00` for indices `0..15`; the second walks the
/// list captured after those calls and passes each node to [`fn_801931b0`].
///
/// # Original
/// - Ghidra: `FUN_80192A70` at `0x80192A70` (108 bytes)
/// - C file: `work/ghidra/main-gb/export/80192a70_FUN_80192a70.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever the two callees do; the list is not relinked here.
///
/// # Safety
/// The guard must be set by [`fn_80192990`] and the list nodes must be live.
///
/// # Confidence
/// high - both loops and their update expressions are explicit.
pub unsafe fn fn_80192a70() {
    if read_u32(WND_INIT_GUARD) != 0 {
        for index in 0..16 {
            let _ = platform::call1(FN_80193F00, index);
        }
        let mut node = read_u32(WND_LIST_HEAD) as usize;
        while node != 0 {
            fn_801931b0(node);
            node = read_u32(node) as usize;
        }
    }
}

/// Builds one message object, initialises its sixteen columns and links it.
///
/// The six pointer words at `object + 0x04..0x18` are rebased onto `table`.
/// `value` is biased by `0x80000000`, halved and doubled into `+0x1C..0x24`;
/// the body then runs [`fn_80192d80`], installs the command cursor at `+0x3FC`,
/// zeroes `0x2000` bytes of per-column pointers at `+0x408`, and pushes the
/// object onto the global list under the lock pair.
///
/// # Original
/// - Ghidra: `FUN_80192AE0` at `0x80192AE0` (392 bytes)
/// - C file: `work/ghidra/main-gb/export/80192ae0_FUN_80192ae0.c`
///
/// # Arguments
/// * `object` - message object to initialise.
/// * `table` - resource base added to the six stored offsets.
/// * `value` - fixed-point size value biased into the three `+0x1C` fields.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the object header, the sixteen column tables and the global head.
///
/// # Safety
/// `object` must provide `0x2408` writable bytes and `table` must be a valid
/// mapped base for the six offsets.
///
/// # Confidence
/// medium - the header and zero-fill extents are exact; the meaning of `value`
/// and of the six offsets is inferred.
pub unsafe fn fn_80192ae0(object: usize, table: usize, value: u32) {
    for index in 0..6 {
        let offset = read_u32(table + index * 4);
        write_u32(object + (index + 1) * 4, (table as u32).wrapping_add(offset));
    }
    write_u32(object + 0x1C, (value.wrapping_add(0x8000_0000)) >> 1);
    write_u32(object + 0x20, value.wrapping_add(0x8000_0000));
    write_u32(object + 0x24, value << 1);
    write_u32(object + 0x68, 0);

    fn_80192d80(object);

    // observed: `param_1[0xFF] = param_1 + 0x3F` stores the command cursor.
    write_u32(object + 0x3FC, p32(object + 0xFC));
    write_u32(object + 0x400, 0);
    write_u32(object + 0x404, 0);

    // observed: 16 outer blocks x 4 inner blocks x 32 words = 0x2000 bytes.
    for outer in 0..16 {
        for inner in 0..4 {
            for word in 0..32 {
                let address = object + 0x408 + outer * 0x200 + inner * 0x80 + word * 4;
                write_u32(address, 0);
            }
        }
    }

    platform::call0(FN_801B1140);
    let head = read_u32(WND_LIST_HEAD);
    write_u32(object, head);
    write_u32(WND_LIST_HEAD, object as u32);
    platform::call0(FN_801B1180);
}

/// Unlinks one message from every window record and from the global list.
///
/// For all sixteen 0x4C-byte records whose `+0x08` owner equals `message`, the
/// body clears the window object through [`fn_80191bb0`], frees the record's
/// `+0x18` id through `0x801928D0`, releases the object through `0x801A0300`
/// and clears the owner word. It then removes `message` itself through
/// [`fn_80192900`].
///
/// # Original
/// - Ghidra: `FUN_80192C70` at `0x80192C70` (168 bytes)
/// - C file: `work/ghidra/main-gb/export/80192c70_FUN_80192c70.c`
///
/// # Arguments
/// * `message` - message object being destroyed. The Ghidra entry is `void`;
///   the value arrives in `r3` and is preserved by the `0x802DCD6C`
///   register-save helper that Ghidra models as a call.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Frees ids and objects, clears owner words and unlinks the message.
///
/// # Safety
/// `message` must be a live object with a valid `+0x404` count and every
/// matching record must still hold a live `+0x04` window object.
///
/// # Confidence
/// medium - every offset and call is explicit; the hidden `this` operand is
/// recovered from the register-save helper, as in the wave-5 teardown bodies.
pub unsafe fn fn_80192c70(message: usize) {
    platform::call0(FN_801B1140);
    if read_u32(message + 0x404) != 0 {
        for index in 0..16 {
            let record = read_u32(WND_MESSAGE_TABLE) as usize + index * 0x4C;
            if read_u32(record + 0x08) == message as u32 {
                let window = read_u32(record + 0x04) as usize;
                fn_80191bb0(window);
                let _ = platform::call1(FN_801928D0, read_u32(window + 0x18));
                let _ = platform::call1(FN_801A0300, window as u32);
                write_u32(record + 0x08, 0);
            }
        }
    }
    fn_80192900(message);
    platform::call0(FN_801B1180);
}

/// Seeds the sixteen per-column position/colour tables of a message object.
///
/// Two passes of eight entries fill `+0x28`, `+0x6C`, `+0xAC` (one word each)
/// and the byte at `+0xEC` in each pass. Every word slot receives the same
/// value: the object's `+0x08` word for `+0x28`, the first word at
/// `0x8032D860` for `+0x6C` and the first word at `0x8032D6D0` for `+0xAC`;
/// the byte slots receive `0x40`.
///
/// # Original
/// - Ghidra: `FUN_80192D80` at `0x80192D80` (220 bytes)
/// - C file: `work/ghidra/main-gb/export/80192d80_FUN_80192d80.c`
///
/// # Arguments
/// * `object` - message object being initialised.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes sixteen words at each of the three arrays and sixteen bytes.
///
/// # Safety
/// `object + 0x28..0xFC` must be writable.
///
/// # Confidence
/// medium - the original is fully unrolled; the two-pass loop reproduces the
/// observed `8 + 8` entries and the `+0x20` word / `+8` byte pass stride, while
/// the printed offsets in the second half are the decompiler's folded constants.
pub unsafe fn fn_80192d80(object: usize) {
    let position = read_u32(object + 0x08);
    let first = read_u32(WND_POS_TABLE_D860);
    let second = read_u32(WND_POS_TABLE_D6D0);

    for pass in 0..2 {
        let word_base = object + pass * 0x20;
        let byte_base = object + pass * 8;
        for index in 0..8 {
            let word = word_base + index * 4;
            write_u32(word + 0x28, position);
            write_u32(word + 0x6C, first);
            write_u32(word + 0xAC, second);
            write_u8(byte_base + index + 0xEC, 0x40);
        }
    }
}

/// Executes one three-byte window command.
///
/// The first byte is split into a 4-bit opcode (high nibble) and a 4-bit
/// column (low nibble); the second byte is a row or value, the third a payload:
///
/// | opcode | action |
/// |---|---|
/// | `8` | clear the `[column][row]` window pointer, marking its old object |
/// | `9` | create a window through `0x80192E60(object, column, row, payload)` |
/// | `0xB` | set column field `+0x6C` (row 7), byte `+0xEC` (row 10) or `+0xAC` (row 0x5B) |
/// | `0xC` | set column `+0x28` to `object[+0x08] + row * 0x100` |
///
/// # Original
/// - Ghidra: `FUN_80193090` at `0x80193090` (284 bytes)
/// - C file: `work/ghidra/main-gb/export/80193090_FUN_80193090.c`
///
/// # Arguments
/// * `object` - message object holding the column tables.
/// * `command` - three readable command bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one table entry, byte or pointer depending on the opcode; opcode `9`
/// calls the untranslated window constructor.
///
/// # Safety
/// `command + 0..3` and the selected column slots must be readable/writable.
///
/// # Confidence
/// high - every opcode branch, table base and offset is explicit; the meaning
/// of rows 7/10/0x5B is inferred from their positions.
pub unsafe fn fn_80193090(object: usize, command: usize) {
    let first = read_u8(command);
    let opcode = (first >> 4) as i32;
    let column = (first & 0x0F) as usize;
    let row = read_u8(command + 1);
    let value = read_u8(command + 2);

    if opcode == 8 {
        let slot = object + column * 0x200 + row as usize * 4;
        let window = read_u32(slot + 0x408);
        if window != 0 {
            fn_80193ef0(window as usize);
            write_u32(slot + 0x408, 0);
        }
    } else if opcode == 9 {
        let _ = platform::call4(
            FN_80192E60,
            object as u32,
            column as u32,
            row as u32,
            value as u32,
        );
    } else if opcode == 0x0B {
        if row == 7 {
            write_u32(
                object + column * 4 + 0x6C,
                read_u32(table_entry(WND_POS_TABLE_D6D0, value as i32, 4)),
            );
        } else if row == 10 {
            write_u8(object + column + 0xEC, value);
        } else if row == 0x5B {
            write_u32(
                object + column * 4 + 0xAC,
                read_u32(table_entry(WND_POS_TABLE_D6D0, value as i32, 4)),
            );
        }
    } else if opcode == 0x0C {
        let base = read_u32(object + 0x08);
        write_u32(
            object + column * 4 + 0x28,
            base.wrapping_add((row as u32) * 0x100),
        );
    }
}

/// Drains the three-byte command buffer of a message object.
///
/// The cursor starts at `object + 0xFC`, the remaining byte count is at
/// `object + 0x400`, and each [`fn_80193090`] call consumes three bytes.
///
/// # Original
/// - Ghidra: `FUN_801931B0` at `0x801931B0` (104 bytes)
/// - C file: `work/ghidra/main-gb/export/801931b0_FUN_801931b0.c`
///
/// # Arguments
/// * `object` - message object holding the command buffer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Executes the commands and decrements `object + 0x400` once per command.
///
/// # Safety
/// The buffer must contain `3 * count` readable bytes.
///
/// # Confidence
/// high - cursor, stride and count update are exact.
pub unsafe fn fn_801931b0(object: usize) {
    let mut cursor = object + 0xFC;
    while read_u32(object + 0x400) != 0 {
        fn_80193090(object, cursor);
        cursor += 3;
        let remaining = read_u32(object + 0x400).wrapping_sub(1);
        write_u32(object + 0x400, remaining);
    }
}

/// Advances one column through the window intro/outro state machine.
///
/// State `0` adds the two fixed-point rates at `+0x38`/`+0x3C` and looks up the
/// state table; when the lookup returns zero the body moves to state `1`.
/// State `1` adds `+0x40`, clamps to `+0x44`, moves to state `2` and either
/// returns or moves to state `4` at the lower bound. State `3` walks towards
/// the bound at `+0x48`, giving up at `-0x2CFFFFF`; state `4` clears the
/// `[column][row]` pointer.
///
/// # Original
/// - Ghidra: `FUN_801934F0` at `0x801934F0` (268 bytes)
/// - C file: `work/ghidra/main-gb/export/801934f0_FUN_801934f0.c`
///
/// # Arguments
/// * `object` - message object; `+0x30` is the state word.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x30`, `+0x34`, `+0x38` and the `+0x408` column slot.
///
/// # Safety
/// The state table index must stay inside `0x8032D8D0` (`0..0x62`) and the
/// column bytes `+0xC`/`+0xD` must select a live slot.
///
/// # Confidence
/// high - every branch, constant and table read is explicit.
pub unsafe fn fn_801934f0(object: usize) {
    let state = read_i32(object + 0x30);
    if state == 0 {
        let mut value = read_i32(object + 0x38).wrapping_add(read_i32(object + 0x3C));
        write_u32(object + 0x38, value as u32);
        let table_value = if value < 0x63_0000 {
            read_u32(table_entry(WND_STATE_TABLE_D8D0, value >> 0x10, 4))
        } else {
            0
        };
        write_u32(object + 0x34, table_value);
        if table_value == 0 {
            write_u32(object + 0x30, 1);
        }
    } else if state == 1 {
        let value = read_i32(object + 0x34).wrapping_add(read_i32(object + 0x40));
        write_u32(object + 0x34, value as u32);
        if value <= read_i32(object + 0x44) {
            write_u32(object + 0x34, read_u32(object + 0x44));
            write_u32(object + 0x30, 2);
        }
        if read_i32(object + 0x34) > -0x2D0_0000 {
            return;
        }
        write_u32(object + 0x30, 4);
        let column = read_u8(object + 0x0C) as usize;
        let row = read_u8(object + 0x0D) as usize;
        let base = read_u32(object + 0x08) as usize;
        write_u32(base + column * 0x200 + row * 4 + 0x408, 0);
    } else if state == 3 {
        if read_i32(object + 0x34) < -0x2CF_FFFF {
            write_u32(object + 0x30, 4);
        } else {
            let value = read_i32(object + 0x34).wrapping_add(read_i32(object + 0x48));
            write_u32(object + 0x34, value as u32);
        }
    }
}

/// Computes one column's text position from the object's `+0x04` pane.
///
/// # Original
/// - Ghidra: `FUN_80193600` at `0x80193600` (40 bytes)
/// - C file: `work/ghidra/main-gb/export/80193600_FUN_80193600.c`
///
/// # Arguments
/// * `object` - message object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the computed word at `object + 0x2C`.
///
/// # Safety
/// `object + 0x10` must point to a block whose `+0x04` word is readable.
///
/// # Confidence
/// high - one table read and one add.
pub unsafe fn fn_80193600(object: usize) {
    let block = read_u32(object + 0x10) as usize;
    let offset = read_u8(object + 0x0E) as i32;
    let value = read_i32(block + 0x04)
        .wrapping_add(read_u32(table_entry(WND_POS_TABLE_D6D0, offset, 4)) as i32);
    write_u32(object + 0x2C, value as u32);
}

/// Copies one column's byte from the text block to `object + 0x0F`.
///
/// # Original
/// - Ghidra: `FUN_80193630` at `0x80193630` (24 bytes)
/// - C file: `work/ghidra/main-gb/export/80193630_FUN_80193630.c`
///
/// # Arguments
/// * `object` - message object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one byte at `object + 0x0F`.
///
/// # Safety
/// `object + 0x08` must point to a block with a byte at `+0xEC + column`.
///
/// # Confidence
/// high - two loads and one store.
pub unsafe fn fn_80193630(object: usize) {
    let block = read_u32(object + 0x08) as usize;
    let column = read_u8(object + 0x0C) as usize;
    write_u8(object + 0x0F, read_u8(block + column + 0xEC));
}

/// Returns the fixed-point sum at `+0x2C`/`+0x34` shifted down by 16.
///
/// # Original
/// - Ghidra: `FUN_80193650` at `0x80193650` (20 bytes)
/// - C file: `work/ghidra/main-gb/export/80193650_FUN_80193650.c`
///
/// # Arguments
/// * `object` - message object.
///
/// # Returns
/// `(i32[+0x2C] + i32[+0x34]) >> 16`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `object + 0x34` must be readable.
///
/// # Confidence
/// high - exact arithmetic.
pub unsafe fn fn_80193650(object: usize) -> i32 {
    read_i32(object + 0x2C).wrapping_add(read_i32(object + 0x34)) >> 0x10
}

/// Returns the fixed-point position of one column shifted down by 16.
///
/// # Original
/// - Ghidra: `FUN_80193670` at `0x80193670` (36 bytes)
/// - C file: `work/ghidra/main-gb/export/80193670_FUN_80193670.c`
///
/// # Arguments
/// * `object` - message object.
///
/// # Returns
/// `(block[+0x68] + block[+0x6C + column * 4]) >> 16`.
///
/// # Side effects
/// None.
///
/// # Safety
/// The block at `object + 0x08` must be readable through `+0x6C`.
///
/// # Confidence
/// high - exact arithmetic.
pub unsafe fn fn_80193670(object: usize) -> i32 {
    let block = read_u32(object + 0x08) as usize;
    let column = read_u8(object + 0x0C) as usize;
    read_i32(block + 0x68).wrapping_add(read_i32(block + column * 4 + 0x6C)) >> 0x10
}

/// Publishes one column's position, texture offset, depth and scale.
///
/// Reads the four fields from the column tables and forwards them to
/// [`fn_80191be0`], [`fn_80191c20`], [`fn_80191ce0`] and [`fn_80191c60`],
/// shifting the three fixed-point values down by 16 bits first.
///
/// # Original
/// - Ghidra: `FUN_801936A0` at `0x801936A0` (160 bytes)
/// - C file: `work/ghidra/main-gb/export/801936a0_FUN_801936a0.c`
///
/// # Arguments
/// * `object` - message object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four record updates through the record accessors.
///
/// # Safety
/// `object + 0x04` must name a live window record and the text block must be
/// readable.
///
/// # Confidence
/// high - the four calls and their source fields are explicit.
pub unsafe fn fn_801936a0(object: usize) {
    let record = read_u32(object + 0x04) as usize;
    let block = read_u32(object + 0x08) as usize;
    let column = read_u8(object + 0x0C) as usize;

    let position = read_i32(object + 0x2C).wrapping_add(read_i32(object + 0x34)) >> 0x10;
    fn_80191be0(record, position as u32);

    let texture = read_i32(block + column * 4 + 0xAC) >> 0x10;
    fn_80191c20(record, texture as u32);

    let depth =
        read_i32(block + 0x68).wrapping_add(read_i32(block + column * 4 + 0x6C)) >> 0x10;
    fn_80191ce0(record, depth as u32);

    fn_80191c60(record, read_u8(block + column + 0xEC) as i32);
}

/// Converts a fixed-point frame count through the percent/minute/second tables.
///
/// The `+0x28` word is a signed 16.16 value. Its integer part is split into
/// hundredths, minutes (`/ 1200`) and seconds (`/ 100`); each remainder indexes
/// one of the three single-precision tables and the product is returned.
/// Negative and zero inputs take the two separate branches of the original.
///
/// # Original
/// - Ghidra: `FUN_80193740` at `0x80193740` (272 bytes)
/// - C file: `work/ghidra/main-gb/export/80193740_FUN_80193740.c`
///
/// # Arguments
/// * `object` - message object; `+0x28` is the fixed-point value.
///
/// # Returns
/// Product of the three scale factors as `f64`; `1.0` when the integer part is
/// zero.
///
/// # Side effects
/// None; only the read-only tables are accessed.
///
/// # Safety
/// The remainder indices must stay inside the three tables.
///
/// # Confidence
/// medium - the integer arithmetic is exact, but the three tables' physical
/// meaning (percent, minute, second) is inferred from their values.
pub unsafe fn fn_80193740(object: usize) -> f64 {
    let raw = read_u32(object + 0x28);
    let signed = raw as i32;
    let rounded = (signed >> 0x10)
        + if signed < 0 && (raw & 0xFFFF) != 0 {
            1
        } else {
            0
        };

    if rounded > 0 {
        let minutes = rounded / 0x4B0;
        let mut seconds = rounded + (minutes - (minutes >> 0x1F)) * -0x4B0;
        seconds = seconds / 100 + (seconds >> 0x1F);
        let hundredths = rounded / 100;
        let percent_index = rounded + (hundredths - (hundredths >> 0x1F)) * -100;
        let minute_index = minutes - (minutes >> 0x1F);
        let second_index = seconds - (seconds >> 0x1F);
        let result = read_f32(table_entry(WND_PERCENT_TABLE, percent_index, 4))
            * read_f32(table_entry(WND_MINUTE_TABLE, minute_index, 4))
            * read_f32(table_entry(WND_SECOND_TABLE, second_index, 4));
        result as f64
    } else if rounded < 0 {
        let hundredths = rounded / 100 + (rounded >> 0x1F);
        let mut seconds = hundredths - (hundredths >> 0x1F);
        let mut percent_index = rounded + (hundredths - (hundredths >> 0x1F)) * -100;
        if percent_index != 0 {
            percent_index += 100;
            seconds -= 1;
        }
        let result = read_f32(table_entry(WND_SECOND_NEG_TABLE, seconds, -4))
            * read_f32(table_entry(WND_PERCENT_TABLE, percent_index, 4));
        result as f64
    } else {
        1.0
    }
}

/// Computes the column spacing and glyph count for one text block.
///
/// Stores the block's `+0x02` halfword divided by `32000.0` at `object + 0x24`,
/// then writes `(column - block[+0]) * 100` at `+0x28`, followed by
/// `(that + block[+0x02]) << 16`.
///
/// # Original
/// - Ghidra: `FUN_80193850` at `0x80193850` (104 bytes)
/// - C file: `work/ghidra/main-gb/export/80193850_FUN_80193850.c`
///
/// # Arguments
/// * `object` - message object; `+0x10` points at the text block.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `object + 0x24` and `object + 0x28` twice, as the original does.
///
/// # Safety
/// `object + 0x10` must point to a block with at least four readable bytes.
///
/// # Confidence
/// medium - the conversion and stores are exact; `32000.0` is read from
/// `0x802F5F64` and the two-step `+0x28` write is preserved.
pub unsafe fn fn_80193850(object: usize) {
    let block = read_u32(object + 0x10) as usize;
    let units = read_u16(block + 0x02) as f32;
    write_f32(object + 0x24, units / 32000.0);

    let difference = (read_u8(object + 0x0D) as i32 - read_u8(block) as i32) * 100;
    write_u32(object + 0x28, difference as u32);

    let with_base = difference.wrapping_add(read_i16(block + 0x02) as i32);
    write_u32(object + 0x28, with_base.wrapping_mul(0x1_0000) as u32);
}

/// Dispatches one window record on the mode halfword at `*object[+0x18]`.
///
/// Mode `0` calls `0x801939C0`, mode `1` calls `0x80193BE0`, mode `2` calls
/// `0x80193CF0`; any other value returns. The three bodies now live in the
/// wave-25 section; this dispatcher keeps its original by-address calls.
///
/// # Original
/// - Ghidra: `FUN_80193E00` at `0x80193E00` (48 bytes)
/// - C file: `work/ghidra/main-gb/export/80193e00_FUN_80193e00.c`
///
/// # Arguments
/// * `object` - message object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever the selected callee does.
///
/// # Safety
/// `object + 0x18` must point to a readable mode halfword.
///
/// # Confidence
/// high - three-way branch, exact addresses; the callee signatures are
/// reconstructed from the single `r3` operand.
pub unsafe fn fn_80193e00(object: usize) {
    let mode_block = read_u32(object + 0x18) as usize;
    match read_i16(mode_block) {
        0 => {
            let _ = platform::call1(FN_801939C0, object as u32);
        }
        1 => {
            let _ = platform::call1(FN_80193BE0, object as u32);
        }
        2 => {
            let _ = platform::call1(FN_80193CF0, object as u32);
        }
        _ => {}
    }
}

/// Marks one window record as state `3` (retiring).
///
/// # Original
/// - Ghidra: `FUN_80193EF0` at `0x80193EF0` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/80193ef0_FUN_80193ef0.c`
///
/// # Arguments
/// * `record` - window record to mark.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `3` at `record + 0x30`.
///
/// # Safety
/// `record + 0x30` must be writable.
///
/// # Confidence
/// high - one literal store.
pub unsafe fn fn_80193ef0(record: usize) {
    write_u32(record + 0x30, 3);
}

// ===========================================================================
// Wave 25: text-window records, command streams and the window manager
// (0x80191050..0x801949B0)
// ===========================================================================
//
// This section completes the record band started in wave 23. The pieces are:
//
// * the one-time initialiser of the sixteen 100-byte records (`0x80191050`);
// * the command writer and the byte-stream interpreter that feed those
//   records (`0x80192D20`, `0x80193FF0`);
// * the per-record update bodies for the three window modes
//   (`0x801939C0`, `0x80193BE0`, `0x80193CF0`) that `0x80193E00` dispatches;
// * the create/retire paths (`0x80192E60`, `0x80193E40`, `0x80193F00`);
// * the window-manager list and singleton around `0x80378C08`
//   (`0x80194470`..`0x801949B0`).
//
// The Ghidra entry points `0x80192D20`, `0x801946C0` and `0x80194760` are
// printed as `void`; their real operands arrive in `r3`/`r4` and survive the
// `__savegprlr_*` prologue helpers (`0x802DCD64`/`0x802DCD70`), which Ghidra
// models as calls. The signatures below restore those operands.

/// `0x80378078`: sixteen forward handle slots (`0xFF` = free).
const WND_FORWARD_TABLE: usize = 0x8037_8078;
/// `0x803780F0`: first 100-byte window record (the pointer at `0x803780EC`
/// points here).
const WND_RECORD_STORAGE: usize = 0x8037_80F0;
/// `0x80378730`: one-time guard for the window-record table.
const WND_TABLE_GUARD: usize = 0x8037_8730;
/// `0x80378C08`: head of the window-manager list.
const WND_WINDOW_LIST: usize = 0x8037_8C08;
/// `0x80378C0C`: one-time guard for [`WND_WINDOW_LIST`].
const WND_WINDOW_LIST_READY: usize = 0x8037_8C0C;
/// `0x8032DDD0`: 256-byte per-command payload-size table (0, 1 or 2 bytes).
const WND_COMMAND_TABLE: usize = 0x8032_DDD0;

/// `0x802F5F70`: `f32 65536.0` (16.16 scale) applied to the frame product.
const WND_F32_5F70: usize = 0x802F_5F70;
/// `0x802F5F78`: `f32 1000000.0`, numerator of the `Q` command duration.
const WND_F32_5F78: usize = 0x802F_5F78;
/// `0x802F5F7C`: `f32 65536.0`, first factor of the `Q` command result.
const WND_F32_5F7C: usize = 0x802F_5F7C;
/// `0x802F5F80`: `f32 96.0`, second factor of the `Q` command result.
const WND_F32_5F80: usize = 0x802F_5F80;
/// `0x802F5F84`: `f32 32000.0`, third factor of the `Q` command result.
const WND_F32_5F84: usize = 0x802F_5F84;

/// `0x8032DEF0`: window-manager vtable at `+0x00`.
const VT_WINDOW_MANAGER: usize = 0x8032_DEF0;
/// `0x8032DF00`: window-manager vtable at `+0x08`.
const VT_WINDOW_MANAGER_08: usize = 0x8032_DF00;
/// `0x8032DF14`: window-manager vtable at `+0xF8`.
const VT_WINDOW_MANAGER_F8: usize = 0x8032_DF14;
/// `0x8032DFC4`: vtable of the child object at manager `+0x8DC`.
const VT_WINDOW_CHILD: usize = 0x8032_DFC4;

/// `0x80192820`: allocates a slot for a message id.
const FN_80192820: usize = 0x8019_2820;
/// `0x80192880`: resolves a message id to its slot.
const FN_80192880: usize = 0x8019_2880;
/// `0x801928A0`: frees one slot in the shared handle table.
const FN_801928A0: usize = 0x8019_28A0;
/// `0x80193220`: builds the per-record timing fields (body [`fn_80193220`] in
/// the wave-30 section).
const FN_80193220: usize = 0x8019_3220;
/// `0x80191260`: per-character cell builder (body [`fn_80191260`] in the
/// wave-30 section).
const FN_80191260: usize = 0x8019_1260;
/// `0x801942C0`: `MTrk`-tag track parser (body [`fn_801942c0`] in the wave-30
/// section).
const FN_801942C0: usize = 0x8019_42C0;
/// `0x801A0380`: allocates and links one text object for a message.
const FN_801A0380: usize = 0x801A_0380;
/// `0x802D46E0`: base constructor of the window-manager object.
const FN_802D46E0: usize = 0x802D_46E0;
/// `0x802BA2D0`: constructor of the child object embedded at manager `+0x8DC`.
const FN_802BA2D0: usize = 0x802B_A2D0;
/// `0x802BA330`: destructor of that child object.
const FN_802BA330: usize = 0x802B_A330;
/// `0x802B9210`: allocator used for the 0x970-byte window manager.
const FN_802B9210: usize = 0x802B_9210;

/// Initialises the sixteen 100-byte window records.
///
/// Runs once, when the message singleton exists and the guard at `0x80378730`
/// is clear. It fills the sixteen forward handle slots and the first `0x60`
/// bytes of the inverse table with `0xFF` (the remaining inverse entries are
/// deliberately left untouched by the original), points `0x803780EC` at the
/// record storage, and stores the same six fields plus thirteen zero halfwords
/// in every record. [`fn_80190fb0`] then fills the colour/offset fields at
/// `+0x18..0x2C`. Mode is set to `1` and the guard is raised.
///
/// # Original
/// - Ghidra: `FUN_80191050` at `0x80191050` (512 bytes)
/// - C file: `work/ghidra/main-gb/export/80191050_FUN_80191050.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the two handle tables, the record-table pointer, all sixteen records,
/// `0x803780E8` and `0x80378730`.
///
/// # Safety
/// The BSS range `0x80378078..0x80378730` must be writable. The prologue
/// helper `0x802DCD6C` only saves registers and is not modelled.
///
/// # Confidence
/// high - both initialisation regions, every record field and the `0x60`-byte
/// extent of the inverse clear are literal in the export.
pub unsafe fn fn_80191050() {
    if call0_r(FN_801A00B0) != 0 && read_u32(WND_TABLE_GUARD) == 0 {
        for index in 0..16 {
            write_u8(WND_FORWARD_TABLE + index, 0xFF);
        }
        for index in 0..0x60 {
            write_u8(WND_SLOT_TABLE + index, 0xFF);
        }
        write_u32(WND_RECORDS, WND_RECORD_STORAGE as u32);

        for index in 0..16 {
            let record = WND_RECORD_STORAGE + index * 100;
            write_u32(record, 0);
            write_u32(record + 0x04, 0x5000_0000);
            write_u32(record + 0x08, 0);
            write_u32(record + 0x0C, 0xFFFF_FC40);
            write_u32(record + 0x10, 0x40);
            write_u32(record + 0x14, 0);
            // observed: the original writes +0x30..+0x60 as zero halfwords in
            // descending order; the loop stores the same addresses.
            let mut offset = 0x30;
            while offset <= 0x60 {
                write_u16(record + offset, 0);
                offset += 4;
            }
            fn_80190fb0(record);
        }

        write_u32(WND_MODE, 1);
        write_u32(WND_TABLE_GUARD, 1);
    }
}

/// Appends one three-byte command to a message object's command buffer.
///
/// The cursor lives at `object + 0x3FC`; the body writes the three source
/// bytes, advances the cursor after each one and increments the command count
/// at `object + 0x400`.
///
/// # Original
/// - Ghidra: `FUN_80192D20` at `0x80192D20` (80 bytes)
/// - C file: `work/ghidra/main-gb/export/80192d20_FUN_80192d20.c`
///
/// # Arguments
/// * `object` - message object holding the command buffer.
/// * `source` - three readable command bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three target bytes, the cursor and the count.
///
/// # Safety
/// The cursor must point inside a buffer with three writable bytes.
///
/// # Confidence
/// high - the unaligned cursor update is explicit at each step.
pub unsafe fn fn_80192d20(object: usize, source: usize) {
    let mut cursor = read_u32(object + 0x3FC) as usize;
    write_u8(cursor, read_u8(source));
    cursor += 1;
    write_u32(object + 0x3FC, cursor as u32);

    write_u8(cursor, read_u8(source + 1));
    cursor += 1;
    write_u32(object + 0x3FC, cursor as u32);

    write_u8(cursor, read_u8(source + 2));
    write_u32(object + 0x3FC, cursor.wrapping_add(1) as u32);
    write_u32(object + 0x400, read_u32(object + 0x400).wrapping_add(1));
}

/// Stores a 16.16 fixed-point value at `object + 0x68`.
///
/// # Original
/// - Ghidra: `FUN_80192D70` at `0x80192D70` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/80192d70_FUN_80192d70.c`
///
/// # Arguments
/// * `object` - object receiving the value.
/// * `value` - integer part; shifted left by 16 before the store.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word.
///
/// # Safety
/// `object + 0x68` must be writable.
///
/// # Confidence
/// high - one shift and one store.
pub unsafe fn fn_80192d70(object: usize, value: i32) {
    write_u32(object + 0x68, (value as u32) << 16);
}

/// Creates or clears one window cell for a message object.
///
/// `value == 0` clears the `[column][row]` pointer at `+0x408` after marking
/// its current object as state `3`. Otherwise the body allocates a text object
/// through `0x801A0380`, reserves an id slot, and fills one 0x4C-byte record:
/// the text object at `+0x04`, the message object at `+0x08`, the three
/// column/row/value bytes, and the glyph-derived pointers at `+0x10..+0x1C`.
/// A glyph index of `0xFFFF` means "no glyph": the record is rolled back and
/// the id and object are released. On success it runs the layout helpers
/// [`fn_80193850`], [`fn_80193600`], [`fn_80193630`], `0x80193220`,
/// [`fn_80193670`], [`fn_80193650`], `0x80191260`, [`fn_80193e00`] and
/// [`fn_801938c0`], then marks the text object visible.
///
/// # Original
/// - Ghidra: `FUN_80192E60` at `0x80192E60` (556 bytes)
/// - C file: `work/ghidra/main-gb/export/80192e60_FUN_80192e60.c`
///
/// # Arguments
/// * `object` - message object holding the sixteen column tables.
/// * `column` - low byte selects the 0x200-byte column block.
/// * `row` - low byte selects the 4-byte row slot.
/// * `value` - nonzero creates, zero clears; also stored at record `+0x0E`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates and frees text objects and id slots, writes one 0x4C-byte record
/// and one `+0x408` pointer, and invokes nine layout helpers.
///
/// # Safety
/// The sixteen column tables at `+0x28`, `+0x0C`..`+0x18`, `+0xAC` and
/// `+0xEC` must be live; the heap and the shared handle table must be usable.
///
/// # Confidence
/// medium - every offset and call is literal; the leading operands are
/// recovered from the `0x802DCD64` helper, and the glyph stride/derived
/// pointers are inferred from the `+0x18`/`+0x50`/`+0x10`/`0x2E` arithmetic.
pub unsafe fn fn_80192e60(object: usize, column: u32, row: u32, value: u32) {
    let column_index = (column & 0xFF) as usize;
    let row_index = (row & 0xFF) as usize;

    if value == 0 {
        let slot = object + column_index * 0x200 + row_index * 4;
        let current = read_u32(slot + 0x408) as usize;
        if current != 0 {
            fn_80193ef0(current);
            write_u32(slot + 0x408, 0);
        }
        return;
    }

    let element = object + column_index * 0x200 + 0x408 + row_index * 4;
    let current = read_u32(element) as usize;
    if current != 0 {
        fn_80193ef0(current);
        write_u32(element, 0);
    }

    let text = platform::call3(FN_801A0380, 0x1F, 0x8019_3E40, object as u32);
    if text == 0 {
        return;
    }
    let text = text as usize;

    let slot_index = platform::call1(FN_80192820, read_u32(text + 0x18)) as i32;
    if slot_index < 0 {
        platform::call1(FN_801A0300, text as u32);
        return;
    }

    let record = read_u32(WND_MESSAGE_TABLE) as usize + slot_index as usize * 0x4C;
    write_u32(record + 0x04, text as u32);
    write_u32(record + 0x08, object as u32);
    write_u8(record + 0x0C, column as u8);
    write_u8(record + 0x0D, row as u8);
    write_u8(record + 0x0E, value as u8);

    let glyph_table = read_u32(object + column_index * 4 + 0x28) as usize;
    let glyph_index = read_u16(glyph_table + row_index * 2);
    if glyph_index == 0xFFFF {
        write_u32(record + 0x08, 0);
        fn_80191bb0(text);
        platform::call1(FN_801928A0, slot_index as u32);
        platform::call1(FN_801A0300, text as u32);
        return;
    }

    let glyph = read_u32(object + 0x0C) as usize + glyph_index as usize * 0x18;
    write_u32(record + 0x10, glyph as u32);
    write_u32(
        record + 0x14,
        read_u32(object + 0x10).wrapping_add(read_u32(glyph + 0x10).wrapping_mul(0x50)),
    );
    let block = read_u32(object + 0x14).wrapping_add(read_u32(glyph + 0x14).wrapping_mul(0x10));
    write_u32(record + 0x18, block);
    write_u32(
        record + 0x1C,
        read_u32(object + 0x18)
            .wrapping_add((read_u16(block as usize + 0x0C) as u32).wrapping_mul(0x2E)),
    );

    write_u32(element, record as u32);
    write_u32(object + 0x404, read_u32(object + 0x404).wrapping_add(1));

    fn_80193850(record);
    fn_80193600(record);
    fn_80193630(record);
    platform::call1(FN_80193220, record as u32);
    let first_position = fn_80193670(record) as u32;
    let second_position = fn_80193650(record) as u32;
    let depth = (read_i32(object + column_index * 4 + 0xAC) >> 0x10) as u32;
    let block_byte = read_u8(object + column_index + 0xEC) as u32;
    platform::call5(
        FN_80191260,
        text as u32,
        second_position,
        depth,
        block_byte,
        first_position,
    );

    fn_80193e00(record);
    fn_801938c0(record);
    write_u16(text + 0x38, 1);
    write_u32(text + 0x1C, read_u32(text + 0x1C) | 4);
}

/// Reads the window-manager pointer stored at `manager + 0x04`.
///
/// # Original
/// - Ghidra: `FUN_801948B0` at `0x801948B0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/801948b0_FUN_801948b0.c`
///
/// # Arguments
/// * `manager` - window-manager object.
///
/// # Returns
/// The word at `manager + 0x04`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `manager + 0x04` must be readable.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_801948b0(manager: usize) -> u32 {
    read_u32(manager + 0x04)
}

/// Forwards a fixed-point value to the sub-object at `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_801948C0` at `0x801948C0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/801948c0_FUN_801948c0.c`
///
/// # Arguments
/// * `object` - window-manager object.
/// * `value` - forwarded to [`fn_80192d70`] in `r4`; the Ghidra signature
///   prints one operand but both call sites pass two.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `object + 0x7C` through [`fn_80192d70`].
///
/// # Safety
/// `object + 0x14` must be a live sub-object.
///
/// # Confidence
/// high - the call and both observed arguments are explicit.
pub unsafe fn fn_801948c0(object: usize, value: i32) {
    fn_80192d70(object + 0x14, value);
}

/// Destroys the child object embedded at window-manager `+0x8DC`.
///
/// Stores the child vtable, clears the manager's singleton pointers when this
/// is the registered child, destroys the embedded base object through
/// `0x802BA330` and optionally frees the storage.
///
/// # Original
/// - Ghidra: `FUN_801948D0` at `0x801948D0` (120 bytes)
/// - C file: `work/ghidra/main-gb/export/801948d0_FUN_801948d0.c`
///
/// # Arguments
/// * `child` - child object, or `0`.
/// * `delete_flag` - `> 0` frees `child`.
///
/// # Returns
/// `child`, unchanged.
///
/// # Side effects
/// Writes the vtable, may clear `SDA_BASE - 0x51F8`/`-0x51F4`, and may free.
///
/// # Safety
/// `child` must be a live child object; the singleton slots must be coherent.
///
/// # Confidence
/// high - the singleton comparison and the two destructor calls are explicit.
pub unsafe fn fn_801948d0(child: usize, delete_flag: i32) -> usize {
    if child != 0 {
        write_u32(child, VT_WINDOW_CHILD as u32);
        if child == read_u32(SDA_BASE - 0x51F4) as usize {
            fn_801949a0();
        }
        platform::call2(FN_802BA330, child as u32, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, child as u32);
        }
    }
    child
}

/// Returns the window-manager singleton, allocating it on first use.
///
/// When `SDA_BASE - 0x51F8` is zero the body allocates 0x970 bytes through
/// `0x802B9210`, runs [`fn_801949b0`], and stores the object plus its embedded
/// child at `+0x8DC` in the two singleton slots.
///
/// # Original
/// - Ghidra: `FUN_80194950` at `0x80194950` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/80194950_FUN_80194950.c`
///
/// # Returns
/// The singleton pointer (`0` when allocation failed).
///
/// # Side effects
/// May allocate and construct the manager; writes both singleton slots.
///
/// # Safety
/// The heap must accept a 0x970-byte allocation and the target image must map
/// the constructor.
///
/// # Confidence
/// high - the guard, size and both slot stores are explicit.
pub unsafe fn fn_80194950() -> usize {
    if read_u32(SDA_BASE - 0x51F8) == 0 {
        let allocated = platform::call1(FN_802B9210, 0x970);
        let mut manager = 0u32;
        if allocated != 0 {
            manager = fn_801949b0(allocated as usize) as u32;
        }
        write_u32(SDA_BASE - 0x51F8, manager);
        write_u32(SDA_BASE - 0x51F4, manager.wrapping_add(0x8DC));
    }
    read_u32(SDA_BASE - 0x51F8) as usize
}

/// Clears both window-manager singleton slots.
///
/// # Original
/// - Ghidra: `FUN_801949A0` at `0x801949A0` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/801949a0_FUN_801949a0.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes zero at `SDA_BASE - 0x51F8` and `SDA_BASE - 0x51F4`.
///
/// # Safety
/// The two small-data words must be writable.
///
/// # Confidence
/// high - two stores, both addresses explicit.
pub unsafe fn fn_801949a0() {
    write_u32(SDA_BASE - 0x51F8, 0);
    write_u32(SDA_BASE - 0x51F4, 0);
}

/// Constructs the 0x970-byte window manager.
///
/// Runs the base constructor `0x802D46E0`, installs the three manager vtables,
/// constructs the child object at `+0x8DC` through `0x802BA2D0`, copies the
/// default `f32 1.0` bits from `SDA2_BASE - 0x52E8` into `+0x924`, and clears
/// the remaining counters, byte flags and the `0xFFFF` sentinel at `+0x93E`.
///
/// # Original
/// - Ghidra: `FUN_801949B0` at `0x801949B0` (248 bytes)
/// - C file: `work/ghidra/main-gb/export/801949b0_FUN_801949b0.c`
///
/// # Arguments
/// * `manager` - 0x970-byte object to construct.
///
/// # Returns
/// `manager`, unchanged.
///
/// # Side effects
/// Writes four vtables and about forty scalar fields; calls two constructors.
///
/// # Safety
/// `manager` must provide 0x970 writable bytes and the mapped data at
/// `SDA2_BASE - 0x52E8` must be readable.
///
/// # Confidence
/// medium - every store is exact; the field meanings are inferred from the
/// window animation code that reads them.
pub unsafe fn fn_801949b0(manager: usize) -> usize {
    platform::call1(FN_802D46E0, manager as u32);
    write_u32(manager, VT_WINDOW_MANAGER as u32);
    write_u32(manager + 0x08, VT_WINDOW_MANAGER_08 as u32);
    write_u32(manager + 0xF8, VT_WINDOW_MANAGER_F8 as u32);

    platform::call1(FN_802BA2D0, (manager + 0x8DC) as u32);
    let default_float = read_u32(SDA2_BASE - 0x52E8);

    write_u32(manager + 0x8DC, VT_WINDOW_CHILD as u32);
    write_u32(manager + 0x8F8, 0);
    write_u32(manager + 0x8FC, 0);
    write_u32(manager + 0x900, 0);
    write_u32(manager + 0x904, 0);
    write_u32(manager + 0x908, 0);
    write_u32(manager + 0x90C, 0);
    write_u32(manager + 0x910, 0);
    write_u8(manager + 0x8F1, 0);
    write_u8(manager + 0x8F2, 0);
    write_u8(manager + 0x8F4, 0);
    write_u32(manager + 0x8EC, 0);
    write_u8(manager + 0x8F0, 0);
    write_u32(manager + 0x91C, 0xFFFF_FFFF);
    write_u32(manager + 0x920, 0);
    write_u32(manager + 0x924, default_float);
    write_u32(manager + 0x928, 0);
    write_u8(manager + 0x92C, 0);
    write_u8(manager + 0x942, 0);
    write_u32(manager + 0x944, 0);
    write_u32(manager + 0x948, 0);
    write_u8(manager + 0x940, 0);
    write_u8(manager + 0x941, 0);
    write_u8(manager + 0x92D, 0);
    write_u8(manager + 0x93C, 0);
    write_u16(manager + 0x93E, 0xFFFF);
    write_u32(manager + 0x938, 0);
    write_u32(manager + 0x950, 0);
    write_u32(manager + 0x954, 0);
    write_u32(manager + 0x958, 0);
    write_u8(manager + 0x95C, 0);
    write_u8(manager + 0x964, 0);
    write_u8(manager + 0x96C, 0);
    manager
}

/// Interprets one record's byte stream and emits three-byte window commands.
///
/// The stream is a sequence of command bytes followed by their payload. A byte
/// above `0x7F` becomes the new pending command (stored at `+0x10`). Pending
/// commands `0xF0`/`0xF7` skip a varint-length blob, `0xFF` reads an extended
/// tag (`/` ends the stream, `Q` stores a 24-bit duration and a derived frame
/// value, anything else skips a blob), and every other command takes 0, 1 or 2
/// payload bytes according to the table at `0x8032DDD0` before
/// [`fn_80192d20`] appends the three bytes. When the cursor reaches the end
/// pointer at `+0x08`, the owning message's count at `+0x0C` is decremented and
/// state `+0x24` is cleared.
///
/// # Original
/// - Ghidra: `FUN_80193FF0` at `0x80193FF0` (704 bytes)
/// - C file: `work/ghidra/main-gb/export/80193ff0_FUN_80193ff0.c`
///
/// # Arguments
/// * `message` - message object that receives the decoded commands.
/// * `command` - 0x28-byte stream record: `+0x00` owner, `+0x08` end,
///   `+0x0C` cursor, `+0x10` pending command, `+0x14`/`+0x1C` decoded values,
///   `+0x24` state.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Advances the cursor, writes the pending/decoded fields, decrements the
/// owner's count and may set its `+0x10` finished flag.
///
/// # Safety
/// The cursor/end addresses must describe a readable stream; the owner object
/// and the id-derived fields must be live.
///
/// # Confidence
/// medium - every branch and store is literal; `0x802DCC4C` (the MWCC
/// double-to-int helper) is modelled with Rust's saturating `as i32`, and the
/// float constants are read from the named .sdata2 addresses.
pub unsafe fn fn_80193ff0(message: usize, command: usize) {
    let mut cursor = read_u32(command + 0x0C);
    let first = read_u8(cursor as usize);
    if first > 0x7F {
        write_u8(command + 0x10, first);
        cursor = cursor.wrapping_add(1);
        write_u32(command + 0x0C, cursor);
    }

    let pending = read_u8(command + 0x10);
    if pending == 0xF0 || pending == 0xF7 {
        let mut byte = read_u8(cursor as usize);
        let mut length = (byte & 0x7F) as u32;
        while (byte & 0x80) != 0 {
            cursor = cursor.wrapping_add(1);
            write_u32(command + 0x0C, cursor);
            byte = read_u8(cursor as usize);
            length = length.wrapping_mul(0x80).wrapping_add((byte & 0x7F) as u32);
        }
        write_u32(
            command + 0x0C,
            cursor.wrapping_add(1).wrapping_add(length),
        );
    } else if pending == 0xFF {
        let tag = read_u8(cursor as usize) as i8;
        cursor = cursor.wrapping_add(1);
        write_u32(command + 0x0C, cursor);

        if tag == b'/' as i8 {
            let owner = read_u32(command) as usize;
            write_u32(owner + 0x0C, read_u32(owner + 0x0C).wrapping_sub(1));
            write_u32(command + 0x24, 0);
            if read_u32(owner + 0x0C) == 0 {
                write_u32(owner + 0x10, 1);
            }
        } else if tag == b'Q' as i8 {
            let mut byte = read_u8(cursor as usize);
            while (byte & 0x80) != 0 {
                cursor = cursor.wrapping_add(1);
                write_u32(command + 0x0C, cursor);
                byte = read_u8(cursor as usize);
            }

            let high = read_u8(cursor as usize + 1) as u32;
            let middle = read_u8(cursor as usize + 2) as u32;
            let low = read_u8(cursor as usize + 3) as u32;
            cursor = cursor.wrapping_add(4);
            write_u32(command + 0x0C, cursor);

            let ticks = (high * 0x100 + middle) * 0x100 + low;
            let value = read_f32(WND_F32_5F78) / ticks as f32;
            write_u32(command + 0x14, value as i32 as u32);

            let signed = read_i16(read_u32(command) as usize + 0x0A) as f64 as f32;
            let result = read_f32(WND_F32_5F7C)
                * (read_f32(WND_F32_5F80)
                    / ((read_f32(WND_F32_5F84) / value) / signed));
            write_u32(command + 0x1C, result as i32 as u32);
        } else {
            let mut byte = read_u8(cursor as usize);
            let mut length = (byte & 0x7F) as u32;
            while (byte & 0x80) != 0 {
                cursor = cursor.wrapping_add(1);
                write_u32(command + 0x0C, cursor);
                byte = read_u8(cursor as usize);
                length = length.wrapping_mul(0x80).wrapping_add((byte & 0x7F) as u32);
            }
            write_u32(
                command + 0x0C,
                cursor.wrapping_add(1).wrapping_add(length),
            );
        }
    } else {
        let mut bytes = crate::platform::GuestLocal::new( [pending, 0, 0]);
        match read_u8(WND_COMMAND_TABLE + pending as usize) {
            1 => {
                bytes[1] = read_u8(cursor as usize);
                cursor = cursor.wrapping_add(1);
            }
            2 => {
                bytes[1] = read_u8(cursor as usize);
                bytes[2] = read_u8(cursor as usize + 1);
                cursor = cursor.wrapping_add(2);
            }
            _ => {}
        }
        write_u32(command + 0x0C, cursor);
        fn_80192d20(message, bytes.as_ptr() as usize);
    }

    if read_u32(command + 0x08) <= read_u32(command + 0x0C) {
        let owner = read_u32(command) as usize;
        write_u32(owner + 0x0C, read_u32(owner + 0x0C).wrapping_sub(1));
        write_u32(command + 0x24, 0);
        if read_u32(owner + 0x0C) == 0 {
            write_u32(owner + 0x10, 1);
        }
    }
}

/// Sets one record's timing fields from its fixed-point frame count.
///
/// Computes the frame scale through [`fn_80193740`], multiplies it by the
/// per-record factor at `+0x24`, scales by the constant at `0x802F5F70`, and
/// stores the 16-bit high and low halves at window `+0xCE`/`+0xD0`. The five
/// following halfwords are cleared and the window flags at `+0x1C` are updated
/// (`& 0xFFFDFFFF | 0x00010001`, clearing bit 17).
///
/// # Original
/// - Ghidra: `FUN_801938C0` at `0x801938C0` (156 bytes)
/// - C file: `work/ghidra/main-gb/export/801938c0_FUN_801938c0.c`
///
/// # Arguments
/// * `record` - window record; `+0x04` is the window object, `+0x24` the
///   frame factor, `+0x28` the fixed-point count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the window object's halfword `+0x30`, word range `+0xCE..+0xDA` and
/// the `+0x1C` flag word.
///
/// # Safety
/// `record + 0x04` must point to a live window object and the tables behind
/// [`fn_80193740`] must be readable.
///
/// # Confidence
/// medium - the arithmetic and every store are exact; `0x802DCC4C` is modelled
/// with Rust's saturating `as i32`, and the conversion sequence is documented
/// in [`fn_80193740`].
pub unsafe fn fn_801938c0(record: usize) {
    let scale = fn_80193740(record);
    let product = (read_f32(record + 0x24) as f64 * scale) as f32;
    let value = (read_f32(WND_F32_5F70) * product) as i32;

    let window = read_u32(record + 0x04) as usize;
    write_u16(window + 0x30, 1);
    write_u16(window + 0xCE, ((value as u32) >> 16) as u16);
    write_u16(window + 0xD0, value as u16);
    write_u16(window + 0xD2, 0);
    write_u16(window + 0xD4, 0);
    write_u16(window + 0xD6, 0);
    write_u16(window + 0xD8, 0);
    write_u16(window + 0xDA, 0);
    write_u32(
        window + 0x1C,
        (read_u32(window + 0x1C) & 0xFFFD_FFFF) | 0x0001_0001,
    );
}

/// Stores the low 32 bits of the frame-count product at window `+0xCE`.
///
/// # Original
/// - Ghidra: `FUN_80193960` at `0x80193960` (92 bytes)
/// - C file: `work/ghidra/main-gb/export/80193960_FUN_80193960.c`
///
/// # Arguments
/// * `record` - window record with the same fields as [`fn_801938c0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one unaligned word at window `+0xCE` and sets bit 17 of `+0x1C`.
///
/// # Safety
/// `record + 0x04` must point to a live window object.
///
/// # Confidence
/// medium - same conversion caveat as [`fn_801938c0`]; the single store and
/// the flag OR are exact.
pub unsafe fn fn_80193960(record: usize) {
    let scale = fn_80193740(record);
    let product = (read_f32(record + 0x24) as f64 * scale) as f32;
    let value = (read_f32(WND_F32_5F70) * product) as i32;

    let window = read_u32(record + 0x04) as usize;
    write_u32(window + 0xCE, value as u32);
    write_u32(window + 0x1C, read_u32(window + 0x1C) | 0x0002_0000);
}

/// Updates a record whose glyph block has both coordinates non-zero.
///
/// The `+0x10` glyph block holds a start at `+0x08` and a length at `+0x0C`.
/// When either is non-zero the body sets the window spacing to `0x10000`, maps
/// the start and end glyph indices onto a 14-row, 16-pixel grid, copies ten
/// words and three halfwords of colour/UV data from record `+0x1C`, and sets
/// the window flags to mode `0x48400`. The zero case lays the single glyph at
/// the block's `+0x08 - 1` end index and uses mode `0x8400`.
///
/// # Original
/// - Ghidra: `FUN_801939C0` at `0x801939C0` (532 bytes)
/// - C file: `work/ghidra/main-gb/export/801939c0_FUN_801939c0.c`
///
/// # Arguments
/// * `record` - window record being laid out.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes record `+0x20`, window `+0x96..+0xE0` and window `+0x1C`.
///
/// # Safety
/// `record + 0x04`, `+0x08`, `+0x10`, `+0x18` and `+0x1C` must all point to
/// live objects/blocks.
///
/// # Confidence
/// high - both branches, the divide-by-14 magic, the ten-word copy and the
/// three halfwords are literal; the grid semantics are inferred from the
/// `% 14` / 16-pixel arithmetic.
pub unsafe fn fn_801939c0(record: usize) {
    let glyph = read_u32(record + 0x10) as usize;
    let window = read_u32(record + 0x04) as usize;
    let start = read_u32(glyph + 0x08);
    let count = read_u32(glyph + 0x0C);

    if start.wrapping_add(count) != 0 {
        write_u32(record + 0x20, 1);
        let end = start.wrapping_add(count).wrapping_sub(1);
        let colours = read_u32(record + 0x1C) as usize;
        let base = read_u32(read_u32(record + 0x08) as usize + 0x24)
            .wrapping_add(read_u32(read_u32(record + 0x18) as usize + 0x04));

        write_u32(window + 0x96, 0x1_0000);
        write_u32(
            window + 0x9A,
            (start % 0x0E)
                .wrapping_add(base)
                .wrapping_add((((start as u64 * 0x1_2492_4925) >> 32) as u32) & 0xFFFF_FFF0)
                .wrapping_add(2),
        );
        write_u32(
            window + 0x9E,
            (end % 0x0E)
                .wrapping_add(base)
                .wrapping_add((((end as u64 * 0x1_2492_4925) >> 32) as u32) & 0xFFFF_FFF0)
                .wrapping_add(2),
        );
        write_u32(window + 0xA2, base.wrapping_add(2));
        copy_colour_words(colours, window + 0xA6);
        // observed: only the multi-glyph branch copies the three halfwords.
        write_u16(window + 0xDC, read_u16(colours + 0x28));
        write_u16(window + 0xDE, read_u16(colours + 0x2A));
        write_u16(window + 0xE0, read_u16(colours + 0x2C));
        write_u32(
            window + 0x1C,
            (read_u32(window + 0x1C) & 0xFFFF_87FF) | 0x0004_8400,
        );
    } else {
        write_u32(record + 0x20, 0);
        let colours = read_u32(record + 0x1C) as usize;
        let end = read_u32(read_u32(record + 0x18) as usize + 0x08).wrapping_sub(1);
        let base = read_u32(read_u32(record + 0x08) as usize + 0x24)
            .wrapping_add(read_u32(read_u32(record + 0x18) as usize + 0x04));

        write_u32(window + 0x96, 0);
        write_u32(window + 0x9A, base);
        write_u32(
            window + 0x9E,
            (end % 0x0E)
                .wrapping_add(base)
                .wrapping_add((((end as u64 * 0x1_2492_4925) >> 32) as u32) & 0xFFFF_FFF0)
                .wrapping_add(2),
        );
        write_u32(window + 0xA2, base.wrapping_add(2));
        copy_colour_words(colours, window + 0xA6);
        write_u32(
            window + 0x1C,
            (read_u32(window + 0x1C) & 0xFFFF_87FF) | 0x0000_8400,
        );
    }
}

/// Copies the ten words of a record's colour/UV block.
///
/// # Original
/// Not a separate function: the identical ten-word copy tail of both branches
/// of `FUN_801939C0` (`801939c0_FUN_801939c0.c`). The multi-glyph branch then
/// copies three additional halfwords at `+0x28..0x2C`; the zero branch does
/// not, so that part stays in the caller.
///
/// # Arguments
/// * `source` - 0x28-byte block.
/// * `destination` - window address receiving ten words.
///
/// # Side effects
/// Writes ten words.
///
/// # Safety
/// `source` must be readable through `+0x28`; `destination` writable through
/// `+0x28`.
///
/// # Confidence
/// high - the ten-word copy is unrolled identically in both branches.
unsafe fn copy_colour_words(source: usize, destination: usize) {
    for index in 0..10 {
        write_u32(destination + index * 4, read_u32(source + index * 4));
    }
}

/// Updates a record whose glyph block has both coordinates zero (mode 1).
///
/// # Original
/// - Ghidra: `FUN_80193BE0` at `0x80193BE0` (272 bytes)
/// - C file: `work/ghidra/main-gb/export/80193be0_FUN_80193be0.c`
///
/// # Arguments
/// * `record` - window record being laid out.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes record `+0x20`, window `+0x96..+0xCA` and window `+0x1C`.
///
/// # Safety
/// The referenced glyph/message blocks must be live.
///
/// # Confidence
/// high - both branches and all ten cleared words are literal.
pub unsafe fn fn_80193be0(record: usize) {
    let glyph = read_u32(record + 0x10) as usize;
    let window = read_u32(record + 0x04) as usize;

    if read_u32(glyph + 0x08).wrapping_add(read_u32(glyph + 0x0C)) == 0 {
        write_u32(record + 0x20, 0);
        let length = read_u32(glyph + 0x08);
        let base = read_u32(read_u32(record + 0x08) as usize + 0x1C)
            .wrapping_add(read_u32(read_u32(record + 0x18) as usize + 0x04));

        write_u32(window + 0x96, 10);
        write_u32(window + 0x9A, base);
        write_u32(window + 0x9E, base.wrapping_add(length).wrapping_sub(1));
        write_u32(window + 0xA2, base);
        write_u32(window + 0xA6, 0);
        write_u32(window + 0xAA, 0);
        write_u32(window + 0xAE, 0);
        write_u32(window + 0xB2, 0);
        write_u32(window + 0xB6, 0);
        write_u32(window + 0xBA, 0);
        write_u32(window + 0xBE, 0);
        write_u32(window + 0xC2, 0);
        write_u32(window + 0xC6, 0x0800_0000);
        write_u32(window + 0xCA, 0);
    } else {
        write_u32(record + 0x20, 1);
        let count = read_u32(glyph + 0x0C);
        let base = read_u32(read_u32(record + 0x08) as usize + 0x1C)
            .wrapping_add(read_u32(read_u32(record + 0x18) as usize + 0x04));
        let start = base.wrapping_add(read_u32(glyph + 0x08));

        write_u32(window + 0x96, 0x1_000A);
        write_u32(window + 0x9A, start);
        write_u32(window + 0x9E, start.wrapping_add(count).wrapping_sub(1));
        write_u32(window + 0xA2, base);
        write_u32(window + 0xA6, 0);
        write_u32(window + 0xAA, 0);
        write_u32(window + 0xAE, 0);
        write_u32(window + 0xB2, 0);
        write_u32(window + 0xB6, 0);
        write_u32(window + 0xBA, 0);
        write_u32(window + 0xBE, 0);
        write_u32(window + 0xC2, 0);
        write_u32(window + 0xC6, 0x0800_0000);
        write_u32(window + 0xCA, 0);
    }

    write_u32(
        window + 0x1C,
        (read_u32(window + 0x1C) & 0xFFFF_87FF) | 0x0000_8400,
    );
}

/// Updates a record whose glyph block has both coordinates zero (mode 2).
///
/// Same shape as [`fn_80193be0`] but reads the base from message `+0x20` and
/// uses mode values `0x19`/`0x10019` with a `0x01000000` colour word.
///
/// # Original
/// - Ghidra: `FUN_80193CF0` at `0x80193CF0` (272 bytes)
/// - C file: `work/ghidra/main-gb/export/80193cf0_FUN_80193cf0.c`
///
/// # Arguments
/// * `record` - window record being laid out.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes record `+0x20`, window `+0x96..+0xCA` and window `+0x1C`.
///
/// # Safety
/// The referenced glyph/message blocks must be live.
///
/// # Confidence
/// high - the two branches differ only in the observed constants and base.
pub unsafe fn fn_80193cf0(record: usize) {
    let glyph = read_u32(record + 0x10) as usize;
    let window = read_u32(record + 0x04) as usize;

    if read_u32(glyph + 0x08).wrapping_add(read_u32(glyph + 0x0C)) == 0 {
        write_u32(record + 0x20, 0);
        let length = read_u32(glyph + 0x08);
        let base = read_u32(read_u32(record + 0x08) as usize + 0x20)
            .wrapping_add(read_u32(read_u32(record + 0x18) as usize + 0x04));

        write_u32(window + 0x96, 0x19);
        write_u32(window + 0x9A, base);
        write_u32(window + 0x9E, base.wrapping_add(length).wrapping_sub(1));
        write_u32(window + 0xA2, base);
        write_u32(window + 0xA6, 0);
        write_u32(window + 0xAA, 0);
        write_u32(window + 0xAE, 0);
        write_u32(window + 0xB2, 0);
        write_u32(window + 0xB6, 0);
        write_u32(window + 0xBA, 0);
        write_u32(window + 0xBE, 0);
        write_u32(window + 0xC2, 0);
        write_u32(window + 0xC6, 0x0100_0000);
        write_u32(window + 0xCA, 0);
    } else {
        write_u32(record + 0x20, 1);
        let count = read_u32(glyph + 0x0C);
        let base = read_u32(read_u32(record + 0x08) as usize + 0x20)
            .wrapping_add(read_u32(read_u32(record + 0x18) as usize + 0x04));
        let start = base.wrapping_add(read_u32(glyph + 0x08));

        write_u32(window + 0x96, 0x1_0019);
        write_u32(window + 0x9A, start);
        write_u32(window + 0x9E, start.wrapping_add(count).wrapping_sub(1));
        write_u32(window + 0xA2, base);
        write_u32(window + 0xA6, 0);
        write_u32(window + 0xAA, 0);
        write_u32(window + 0xAE, 0);
        write_u32(window + 0xB2, 0);
        write_u32(window + 0xB6, 0);
        write_u32(window + 0xBA, 0);
        write_u32(window + 0xBE, 0);
        write_u32(window + 0xC2, 0);
        write_u32(window + 0xC6, 0x0100_0000);
        write_u32(window + 0xCA, 0);
    }

    write_u32(
        window + 0x1C,
        (read_u32(window + 0x1C) & 0xFFFF_87FF) | 0x0000_8400,
    );
}

/// Releases one live text object and its message record.
///
/// Resolves the id at `+0x18` to a slot, frees the slot, finds the 0x4C-byte
/// message record, clears the window record through [`fn_80191bb0`], removes
/// the record from the `[column][row]` slot when it still points there, clears
/// the owner word and decrements the message object's live count at `+0x404`.
///
/// # Original
/// - Ghidra: `FUN_80193E40` at `0x80193E40` (164 bytes)
/// - C file: `work/ghidra/main-gb/export/80193e40_FUN_80193e40.c`
///
/// # Arguments
/// * `text` - allocated text object; `+0x14` message object, `+0x18` id.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Frees an id slot, clears one `+0x408` slot and one record owner word, and
/// decrements the message live count.
///
/// # Safety
/// `text + 0x14`/`+0x18` must be live and the message table must be initialised.
///
/// # Confidence
/// high - the slot resolution, table stride and every store are explicit.
pub unsafe fn fn_80193e40(text: usize) {
    let message = read_u32(text + 0x14) as usize;
    let slot = platform::call1(FN_80192880, read_u32(text + 0x18));
    platform::call1(FN_801928A0, slot);

    let record = read_u32(WND_MESSAGE_TABLE) as usize + slot as usize * 0x4C;
    fn_80191bb0(text);

    let cell = message
        + read_u8(record + 0x0C) as usize * 0x200
        + read_u8(record + 0x0D) as usize * 4;
    if read_u32(cell + 0x408) == record as u32 {
        write_u32(cell + 0x408, 0);
    }
    write_u32(record + 0x08, 0);
    write_u32(message + 0x404, read_u32(message + 0x404).wrapping_sub(1));
}

/// Retires one message record by index.
///
/// When the record's `+0x08` owner is live and its `+0x20` flag is clear and
/// window `+0x38` is zero, the body unlinks the `[column][row]` slot and moves
/// the record to state `4`. [`fn_801934f0`] then advances the record; state `4`
/// clears the owner, releases the window object, frees the id, releases the
/// text object and decrements the message live count. Other states refresh the
/// position and timing through [`fn_801936a0`] and [`fn_80193960`].
///
/// # Original
/// - Ghidra: `FUN_80193F00` at `0x80193F00` (240 bytes)
/// - C file: `work/ghidra/main-gb/export/80193f00_FUN_80193f00.c`
///
/// # Arguments
/// * `index` - record index `0..15`; the record is `table + index * 0x4C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Unlinks cell slots, frees ids/objects, writes the record state and updates
/// one live count.
///
/// # Safety
/// The message table and every referenced object must be live.
///
/// # Confidence
/// high - every branch, offset and callee is literal.
pub unsafe fn fn_80193f00(index: i32) {
    let record = read_u32(WND_MESSAGE_TABLE) as usize + index as usize * 0x4C;
    let message = read_u32(record + 0x08) as usize;
    if message == 0 {
        return;
    }

    if read_u32(record + 0x20) == 0
        && read_u16(read_u32(record + 0x04) as usize + 0x38) == 0
    {
        let cell = message
            + read_u8(record + 0x0C) as usize * 0x200
            + read_u8(record + 0x0D) as usize * 4;
        if read_u32(cell + 0x408) == record as u32 {
            write_u32(cell + 0x408, 0);
        }
        write_u32(record + 0x30, 4);
    }

    fn_801934f0(record);
    if read_u32(record + 0x30) == 4 {
        let text = read_u32(record + 0x04) as usize;
        write_u32(record + 0x08, 0);
        fn_80191bb0(text);
        platform::call1(FN_801928D0, read_u32(text + 0x18));
        platform::call1(FN_801A0300, text as u32);
        write_u32(message + 0x404, read_u32(message + 0x404).wrapping_sub(1));
    } else {
        fn_801936a0(record);
        fn_80193960(record);
    }
}

/// Parses one track descriptor into a window's command records.
///
/// The descriptor at `+0x04` is an offset from `descriptor + 8`; `+0x08` is
/// the selector (0 = one track, 1 = count from the descriptor's `+0x0A`
/// halfword, anything else does nothing), `+0x0A`/`+0x0C` are copied into the
/// object's two halfwords, and `+0x0C` then receives the selector halfword.
///
/// # Original
/// - Ghidra: `FUN_801943E0` at `0x801943E0` (136 bytes)
/// - C file: `work/ghidra/main-gb/export/801943e0_FUN_801943e0.c`
///
/// # Arguments
/// * `object` - window object receiving the track counts.
/// * `descriptor` - track descriptor.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes object `+0x08`, `+0x0A` and `+0x0C`; calls `0x801942C0` with the
/// resolved track data.
///
/// # Safety
/// The descriptor's offset must resolve to `count` readable chunks.
///
/// # Confidence
/// medium - the fields and first call are literal; the second call's count
/// register is not printed by Ghidra, so it is taken from the halfword that
/// was just stored (the same value the first call passes as `1`).
pub unsafe fn fn_801943e0(object: usize, descriptor: usize) {
    let selector = read_i16(descriptor + 0x08);
    let data = descriptor + 8 + read_u32(descriptor + 0x04) as usize;

    write_u16(object + 0x08, read_u16(descriptor + 0x0A));
    write_u16(object + 0x0A, read_u16(descriptor + 0x0C));

    if selector == 0 {
        write_u16(object + 0x08, 1);
    }

    if selector == 0 || selector == 1 {
        platform::call3(
            FN_801942C0,
            object as u32,
            data as u32,
            read_u16(object + 0x08) as u32,
        );
    }

    write_u32(object + 0x0C, read_u16(object + 0x08) as u32);
}

/// Raises the one-time guard of the window-manager list.
///
/// # Original
/// - Ghidra: `FUN_80194470` at `0x80194470` (40 bytes)
/// - C file: `work/ghidra/main-gb/export/80194470_FUN_80194470.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears the list head and sets `0x80378C0C`; a second call does nothing.
///
/// # Safety
/// The two BSS words must be writable.
///
/// # Confidence
/// high - one guard and two stores.
pub unsafe fn fn_80194470() {
    if read_u32(WND_WINDOW_LIST_READY) != 0 {
        return;
    }
    write_u32(WND_WINDOW_LIST, 0);
    write_u32(WND_WINDOW_LIST_READY, 1);
}

/// Advances every live window's command stream by the elapsed frame count.
///
/// Walks the window list at `0x80378C08`. For mode `1`/`2` windows it visits
/// the command records at `+0x241C` while their state is `1`/`2`, consuming the
/// per-command duration at `+0x20` and calling [`fn_80193ff0`] whenever the
/// elapsed amount covers a record. When a record ends, the next varint in the
/// stream becomes the new duration. After the stream, windows whose `+0x10`
/// flag is set are reset through [`fn_80194760`] (mode `2` windows twice).
///
/// # Original
/// - Ghidra: `FUN_801944A0` at `0x801944A0` (380 bytes)
/// - C file: `work/ghidra/main-gb/export/801944a0_FUN_801944a0.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Advances cursors, rewrites durations/states and may reset whole windows.
///
/// # Safety
/// Every list node and command record must be live; the guard at `0x80378C0C`
/// must have been raised by [`fn_80194470`].
///
/// # Confidence
/// medium - the list walk, duration arithmetic and both reset calls are
/// literal; the stream/state field roles are inferred from the adjacent
/// bodies.
pub unsafe fn fn_801944a0() {
    if read_u32(WND_WINDOW_LIST_READY) == 0 {
        return;
    }

    let mut node = read_u32(WND_WINDOW_LIST) as usize;
    while node != 0 {
        let next = read_u32(node) as usize;

        if read_u32(node + 0x04).wrapping_sub(1) < 2 {
            let mut command = node + 0x241C;
            let count = read_u16(node + 0x08) as usize;
            for _ in 0..count {
                if read_u32(command + 0x24).wrapping_sub(1) < 2 {
                    let mut remaining = read_u32(command + 0x1C);
                    let step = read_u32(command + 0x20);
                    if remaining < step {
                        write_u32(command + 0x20, step.wrapping_sub(remaining));
                    } else {
                        while read_u32(command + 0x20) <= remaining {
                            remaining = remaining.wrapping_sub(read_u32(command + 0x20));
                            fn_80193ff0(node + 0x14, command);
                            if read_u32(command + 0x24) == 0 {
                                break;
                            }

                            let mut cursor = read_u32(command + 0x0C) as usize;
                            let mut byte = read_u8(cursor);
                            let mut length = (byte & 0x7F) as u32;
                            while (byte & 0x80) != 0 {
                                cursor = cursor.wrapping_add(1);
                                write_u32(command + 0x0C, cursor as u32);
                                byte = read_u8(cursor);
                                length = length
                                    .wrapping_mul(0x80)
                                    .wrapping_add((byte & 0x7F) as u32);
                            }
                            write_u32(command + 0x0C, cursor.wrapping_add(1) as u32);
                            write_u32(command + 0x20, length << 16);
                        }
                        let current = read_u32(command + 0x20);
                        write_u32(command + 0x20, current.wrapping_sub(remaining));
                    }
                }
                command += 0x28;
            }
        }

        if read_u32(node + 0x10) != 0 {
            if read_u32(node + 0x04) == 2 {
                fn_80194760(node, 0);
                fn_80194760(node, 2);
            } else {
                fn_80194760(node, 0);
            }
        }

        node = next;
    }
}

/// Links one window object into the window-manager list.
///
/// Initialises the embedded message at `+0x14` through [`fn_80192ae0`], clears
/// the mode at `+0x04`, parses the descriptor through [`fn_801943e0`], and
/// pushes the object onto `0x80378C08` under the interrupt lock pair.
///
/// # Original
/// - Ghidra: `FUN_80194630` at `0x80194630` (144 bytes)
/// - C file: `work/ghidra/main-gb/export/80194630_FUN_80194630.c`
///
/// # Arguments
/// * `object` - window object being linked.
/// * `descriptor` - track descriptor handed to [`fn_801943e0`].
/// * `table` - resource base handed to [`fn_80192ae0`].
/// * `value` - fixed-point size handed to [`fn_80192ae0`].
/// * `unused` - fifth argument; the body never reads it.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the message header, the window fields and the list head.
///
/// # Safety
/// `object` must provide the message object's writable range and `table` must
/// be a valid resource base.
///
/// # Confidence
/// high - every store, both locks and the three calls are explicit.
pub unsafe fn fn_80194630(
    object: usize,
    descriptor: usize,
    table: u32,
    value: u32,
    _unused: u32,
) {
    fn_80192ae0(object + 0x14, table as usize, value);
    write_u32(object + 0x04, 0);
    fn_801943e0(object, descriptor);

    let state = platform::call1(FN_801B1140, 0);
    let head = read_u32(WND_WINDOW_LIST);
    if head == 0 {
        write_u32(object, 0);
    } else {
        write_u32(object, head);
    }
    write_u32(WND_WINDOW_LIST, object as u32);
    platform::call1(FN_801B1180, state);
}

/// Unlinks one window object and destroys its embedded message.
///
/// Rebuilds the window list without the object under the lock pair, then calls
/// [`fn_80192c70`] with the embedded message at `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_801946C0` at `0x801946C0` (152 bytes)
/// - C file: `work/ghidra/main-gb/export/801946c0_FUN_801946c0.c`
///
/// # Arguments
/// * `object` - window object to unlink. The Ghidra entry is `void`; the
///   pointer arrives in `r3` and is preserved by `0x802DCD70`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Rewrites the list links/head and destroys the embedded message.
///
/// # Safety
/// Every list node must be readable through `+0x00`; `object + 0x14` must be a
/// live message object.
///
/// # Confidence
/// medium - the relink loop and both lock regions are exact; the hidden `this`
/// operand is recovered from the `__savegprlr_26` helper.
pub unsafe fn fn_801946c0(object: usize) {
    let state = platform::call1(FN_801B1140, 0);
    let mut node = read_u32(WND_WINDOW_LIST) as usize;
    write_u32(WND_WINDOW_LIST, 0);

    while node != 0 {
        let next = read_u32(node) as usize;
        if node != object {
            let inner_state = platform::call1(FN_801B1140, 0);
            let head = read_u32(WND_WINDOW_LIST);
            write_u32(node, head);
            write_u32(WND_WINDOW_LIST, node as u32);
            platform::call1(FN_801B1180, inner_state);
        }
        node = next;
    }

    platform::call1(FN_801B1180, state);
    fn_80192c70(object + 0x14);
}

/// Resets or re-seeds the command records of one window object.
///
/// For mode `1`/`2` and a zero mode field, the body walks the command records
/// at `+0x241C` (count at `+0x08`) and restarts each one from its `+0x04`
/// stream: cursor `+0x0C` is reset, the `+0x18` word is copied to `+0x1C`, the
/// first varint becomes the duration at `+0x20`, and state `+0x24` becomes
/// `1`. Modes `0` and `3` instead append sixteen `0xB0..0xBF 0x7B 0x00`
/// commands to the embedded message at `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_80194760` at `0x80194760` (316 bytes)
/// - C file: `work/ghidra/main-gb/export/80194760_FUN_80194760.c`
///
/// # Arguments
/// * `object` - window object. The Ghidra entry is `void`; the pointer and
///   mode arrive in `r3`/`r4` and are preserved by `0x802DCD70`.
/// * `mode` - requested mode; stored at `object + 0x04` at the end.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Rewrites command cursors/durations/states, the count at `+0x0C`, the flag at
/// `+0x10`, or appends sixteen commands, then stores the mode.
///
/// # Safety
/// The command records must contain readable streams and the embedded message
/// must be live for the append path.
///
/// # Confidence
/// medium - every store and both lock regions are exact; the hidden `this` and
/// mode operands are recovered from the register-save helper.
pub unsafe fn fn_80194760(object: usize, mode: i32) {
    if (mode as u32).wrapping_sub(1) < 2 {
        if read_u32(object + 0x04) == 0 {
            let state = platform::call1(FN_801B1140, 0);
            let mut command = object + 0x241C;
            let count = read_u16(object + 0x08) as usize;

            for _ in 0..count {
                let start = read_u32(command + 0x04);
                write_u32(command + 0x0C, start);
                write_u32(command + 0x1C, read_u32(command + 0x18));

                let mut cursor = start as usize;
                let mut byte = read_u8(cursor);
                let mut length = (byte & 0x7F) as u32;
                while (byte & 0x80) != 0 {
                    cursor = cursor.wrapping_add(1);
                    write_u32(command + 0x0C, cursor as u32);
                    byte = read_u8(cursor);
                    length = length.wrapping_mul(0x80).wrapping_add((byte & 0x7F) as u32);
                }
                write_u32(command + 0x0C, cursor.wrapping_add(1) as u32);
                write_u32(command + 0x20, length << 16);
                write_u32(command + 0x24, 1);
                command += 0x28;
            }

            write_u32(object + 0x0C, read_u16(object + 0x08) as u32);
            platform::call1(FN_801B1180, state);
        }
        write_u32(object + 0x10, 0);
    } else if mode == 0 || mode == 3 {
        let mut index = 0u32;
        loop {
            let state = platform::call1(FN_801B1140, 0);
            let mut command = crate::platform::GuestLocal::new( [0u8; 3]);
            command[0] = (index as u8) | 0xB0;
            command[1] = 0x7B;
            command[2] = 0;
            fn_80192d20(object + 0x14, command.as_ptr() as usize);
            platform::call1(FN_801B1180, state);

            index += 1;
            if index >= 0x10 {
                break;
            }
        }
    }

    write_u32(object + 0x04, mode as u32);
}

// ===========================================================================
// Wave 30: text-window record builders and per-frame channel update
// (0x80191250..0x801942C0)
// ===========================================================================
//
// Wave 23 translated the small record setters around `0x80191BB0..0x80191CE0`
// and stopped before the bodies that actually build and flush the 100-byte
// records. This section adds them:
//
// * `0x80191250` stores the global record mode (`0`, `1` or `2`); both
//   builders read it through `0x803780E8`.
// * `0x80191260` claims the record slot for a layout object, initialises the
//   record, runs the colour/offset selector [`fn_80190fb0`], computes the
//   initial channel angles and copies twelve angle values into the layout
//   object.
// * `0x80191D20` is the per-frame flush over all sixteen records: it consumes
//   the dirty flags, rebuilds the six (mode `0`/`1`) or eight (mode `2`)
//   channel angles and pushes the twelve angle/delta pairs to the layout
//   object.
// * `0x80193220` turns one record's rate words into the six fixed-point
//   timing fields at `+0x30..+0x48` through the float helper `0x802E55A4`.
// * `0x801942C0` scans `MTrk` chunks and fills the 0x28-byte command records
//   at `+0x241C` for `0x801943E0`.
//
// Both angle builders read the signed halfword table at `0x8032C940`. The
// per-frame body addresses the same table through `0x8032D050`, which is the
// table plus `0x388` entries (`0x710` bytes); the two views are identical.
// Entry `-0x388` is never read because the clamp rejects everything below
// `-0x387`.

/// `0x8032C940`: signed halfword angle table in 1/256-turn units.
const WND_ANGLE_TABLE: usize = 0x8032_C940;
/// Entry offset of the `0x8032D050` view inside [`WND_ANGLE_TABLE`].
const WND_ANGLE_BIAS: i32 = 0x388;
/// Value stored for indices at or above `0x3C` (`0xFF64` as a signed halfword).
const WND_ANGLE_CLAMP: i16 = -156;

/// Looks up one angle in [`WND_ANGLE_TABLE`].
///
/// observed: the original tests the index twice and produces three cases:
/// below `-0x387` gives `0`, below `0x3C` reads the signed halfword at
/// `table + (index + 0x388) * 2`, and everything else gives `0xFF64` (-156).
///
/// # Original
/// Host-side helper for the repeated clamp in `FUN_80191260` and
/// `FUN_80191D20`; the index arithmetic is wrapping to match the PowerPC
/// `add`/`addi` pairs.
///
/// # Safety
/// The target image must be mapped at `0x8032C942` (the first reachable
/// entry, `index = -0x387`).
///
/// # Confidence
/// high - both callers print the same three-arm clamp literally.
#[inline]
unsafe fn wnd_angle(index: i32) -> i16 {
    if index < -0x387 {
        0
    } else if index < 0x3C {
        let biased = index.wrapping_add(WND_ANGLE_BIAS);
        read_i16(WND_ANGLE_TABLE.wrapping_add_signed(biased as isize * 2))
    } else {
        WND_ANGLE_CLAMP
    }
}

/// Stores one angle channel and clears its delta slot in a layout object.
///
/// observed: `FUN_80191260` writes twelve angle halfwords at `+0x3C`, `+0x40`,
/// `+0x44`, ... and always clears the following halfword. A non-zero angle
/// raises the channel's presence bit in the object flag word at `+0x34`.
///
/// # Original
/// Host-side helper for the twelve repeated stores at the end of
/// `FUN_80191260`.
///
/// # Safety
/// `object + dst .. object + dst + 4` must be writable.
///
/// # Confidence
/// high - one store pair plus one flag test, repeated twelve times.
#[inline]
unsafe fn wnd_store_angle(object: usize, dst: usize, value: i16, flag: u32, flags: &mut u32) {
    write_u16(object + dst, value as u16);
    if value != 0 {
        *flags |= flag;
    }
    write_u16(object + dst + 2, 0);
}

/// Copies one angle/delta channel pair into a layout object.
///
/// observed: `FUN_80191D20` writes the record angle to `object + dst`, then
/// `(record[dst + 2] - record[dst]) / 0x60` to `object + dst + 2`, comparing
/// the low halfword of each with zero to raise the two presence bits.
///
/// # Original
/// Host-side helper for the twelve repeated blocks at the end of
/// `FUN_80191D20`. The divide-by-96 is the original's
/// `x / 0x60 + (x >> 31)` pair followed by the opposite correction, which
/// reduces exactly to C's truncating signed division; Rust's `/` truncates
/// toward zero in the same way. Both operands of the subtraction are
/// zero-extended halfwords, as printed in the C.
///
/// # Safety
/// The record and object fields must be readable/writable.
///
/// # Confidence
/// high - the store order, subtraction width and both flag tests are explicit.
#[inline]
unsafe fn wnd_copy_channel(
    object: usize,
    dst: usize,
    record: usize,
    src: usize,
    angle_flag: u32,
    delta_flag: u32,
    flags: &mut u32,
) {
    let angle = read_u16(record + src);
    write_u16(object + dst, angle);
    if angle != 0 {
        *flags |= angle_flag;
    }

    let next = read_u16(record + src + 2) as i32;
    let delta = (next - angle as i32) / 0x60;
    let delta_bits = delta as u16;
    write_u16(object + dst + 2, delta_bits);
    if delta_bits != 0 {
        *flags |= delta_flag;
    }
}

/// Copies the record's twelve angle halfwords one slot forward.
///
/// observed: `FUN_80191D20` copies `+0x36` to `+0x34`, `+0x3A` to `+0x38`,
/// and so on in ascending order, `+0x62` to `+0x60`.
///
/// # Original
/// Host-side helper for the twelve-copy block that runs while record flag
/// `0x80000000` is set.
///
/// # Safety
/// `record + 0x34 .. record + 0x64` must be readable and writable.
///
/// # Confidence
/// high - ascending four-byte stride, twelve iterations.
#[inline]
unsafe fn wnd_copy_pairs(record: usize) {
    let mut dst = 0x34;
    while dst <= 0x60 {
        write_u16(record + dst, read_u16(record + dst + 2));
        dst += 4;
    }
}

/// Sets the text-window mode.
///
/// The mode selects which record fields the two window builders use: `0` and
/// `1` rebuild six channel angles, while `2` rebuilds eight and starts the
/// object flag word with `0x80000000`.
///
/// # Original
/// - Ghidra: `FUN_80191250` at `0x80191250` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/80191250_FUN_80191250.c`
///
/// # Arguments
/// * `mode` - value written to `0x803780E8`; the original performs no range
///   check, so unknown values simply leave both builders' mode branches
///   unentered.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the global mode word.
///
/// # Safety
/// `0x803780E8` must be writable.
///
/// # Confidence
/// high - one literal store.
pub unsafe fn fn_80191250(mode: u32) {
    write_u32(WND_MODE, mode);
}

/// Builds one text-window record and applies its initial channel angles.
///
/// The layout object at `window + 0x18` names a message id. The id selects a
/// slot through the signed inverse table at `0x80378088`, the slot selects one
/// of the sixteen 100-byte records at `0x803780EC`, and the record is stamped
/// with `window`, a zero flag word, the angle index and the three remaining
/// arguments. [`fn_80190fb0`] then fills `+0x18..+0x30` from the record's
/// colour/offset index.
///
/// The body stores `window`'s table index (`index`) as an angle at `+0x30`,
/// computes six angles in mode `0`/`1` or eight in mode `2` (`record + 0x34`
/// through `+0x5C`, with the unused slots cleared), and finally copies twelve
/// angle values into the layout object: `+0x3C`, `+0x40`, `+0x44`, `+0x48`,
/// `+0x4C`, `+0x50`, `+0x54`, `+0x58`, `+0x5C`, `+0x60`, `+0x64`, `+0x68`.
/// The paired delta halfwords are cleared, and each non-zero angle raises one
/// presence bit in the object flag word at `+0x34` (the same bit assignment
/// the per-frame builder uses).
///
/// The copy runs under the message-list lock `0x801B1140`/`0x801B1180`, sets
/// state bits `0x112` at `+0x1C`, clears the 32-byte block at `+0x102`, then
/// sets bits `0x3000000` at `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_80191260` at `0x80191260` (2,376 bytes)
/// - C file: `work/ghidra/main-gb/export/80191260_FUN_80191260.c`
///
/// # Arguments
/// * `window` - layout window object; `+0x18` is the message id and
///   `+0x3C..+0x6A` receive the channel values.
/// * `index` - signed value stored at record `+0x08`; it is also the angle
///   table index used for record `+0x30`.
/// * `arg3`, `arg4`, `arg5` - stored verbatim at record `+0x0C`, `+0x10` and
///   `+0x14`. `arg4` is the colour/offset index read by [`fn_80190fb0`]; the
///   roles of `arg3` and `arg5` are not recovered (they feed the mode
///   formulas at `+0x0C` and `+0x14`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one 100-byte record, the layout object's channel/flag/state fields,
/// takes and releases the message-list lock, and calls the CRT `memset`.
///
/// # Safety
/// The window, the selected record slot and the target's small-data area must
/// be mapped and writable; `window + 0x18` must hold a valid message id.
///
/// # Confidence
/// high - every branch, table index and store is present in the C export; the
/// only inferred parts are the plain-English field roles.
pub unsafe fn fn_80191260(window: usize, index: i32, arg3: u32, arg4: u32, arg5: u32) {
    let record = window_record(window);

    write_u32(record, window as u32);
    write_u32(record + 0x04, 0);
    write_u32(record + 0x08, index as u32);
    write_u32(record + 0x0C, arg3);
    write_u32(record + 0x10, arg4);
    write_u32(record + 0x14, arg5);
    fn_80190fb0(record);

    write_u16(record + 0x30, wnd_angle(index) as u16);

    let mut flags = 0u32;
    let mode = read_u32(WND_MODE);
    let f0c = read_i32(record + 0x0C);
    let f14 = read_i32(record + 0x14);

    if mode == 0 {
        let f20 = read_i32(record + 0x20);
        let f24 = read_i32(record + 0x24);

        write_u16(record + 0x34, wnd_angle(f14.wrapping_add(f20)) as u16);
        write_u16(record + 0x38, wnd_angle(f14.wrapping_add(f20)) as u16);
        write_u16(
            record + 0x3C,
            wnd_angle(f14.wrapping_add(f24).wrapping_sub(0x1E)) as u16,
        );
        write_u16(
            record + 0x40,
            wnd_angle(f0c.wrapping_add(f20).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x44,
            wnd_angle(f0c.wrapping_add(f20).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x48,
            wnd_angle(f14.wrapping_add(f0c).wrapping_add(f24).wrapping_sub(0x1E)) as u16,
        );
        for dst in [0x4C, 0x50, 0x54, 0x58, 0x5C, 0x60] {
            write_u16(record + dst, 0);
        }
    } else if mode == 1 {
        let f18 = read_i32(record + 0x18);
        let f1c = read_i32(record + 0x1C);
        let f20 = read_i32(record + 0x20);
        let f24 = read_i32(record + 0x24);

        write_u16(
            record + 0x34,
            wnd_angle(f18.wrapping_add(f20).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x38,
            wnd_angle(f1c.wrapping_add(f20).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x3C,
            wnd_angle(f14.wrapping_add(f24).wrapping_sub(0x1E)) as u16,
        );
        write_u16(
            record + 0x40,
            wnd_angle(f20.wrapping_add(f18).wrapping_add(f14).wrapping_add(f0c)) as u16,
        );
        write_u16(
            record + 0x44,
            wnd_angle(f20.wrapping_add(f1c).wrapping_add(f14).wrapping_add(f0c)) as u16,
        );
        write_u16(
            record + 0x48,
            wnd_angle(f14.wrapping_add(f0c).wrapping_add(f24).wrapping_sub(0x1E)) as u16,
        );
        for dst in [0x4C, 0x50, 0x54, 0x58, 0x5C, 0x60] {
            write_u16(record + dst, 0);
        }
    } else if mode == 2 {
        let f18 = read_i32(record + 0x18);
        let f1c = read_i32(record + 0x1C);
        let f20 = read_i32(record + 0x20);
        let f24 = read_i32(record + 0x24);
        let f28 = read_i32(record + 0x28);
        let f2c = read_i32(record + 0x2C);

        write_u16(
            record + 0x34,
            wnd_angle(f18.wrapping_add(f20).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x38,
            wnd_angle(f1c.wrapping_add(f20).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x3C,
            wnd_angle(f28.wrapping_add(f24).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x58,
            wnd_angle(f2c.wrapping_add(f24).wrapping_add(f14)) as u16,
        );
        write_u16(
            record + 0x40,
            wnd_angle(f20.wrapping_add(f18).wrapping_add(f14).wrapping_add(f0c)) as u16,
        );
        write_u16(
            record + 0x44,
            wnd_angle(f20.wrapping_add(f1c).wrapping_add(f14).wrapping_add(f0c)) as u16,
        );
        write_u16(
            record + 0x48,
            wnd_angle(f24.wrapping_add(f28).wrapping_add(f14).wrapping_add(f0c)) as u16,
        );
        write_u16(
            record + 0x5C,
            wnd_angle(f24.wrapping_add(f2c).wrapping_add(f14).wrapping_add(f0c)) as u16,
        );
        flags = 0x8000_0000;
        for dst in [0x4C, 0x50, 0x54, 0x60] {
            write_u16(record + dst, 0);
        }
    }

    let lock = call0_r(FN_801B1140);
    write_u16(window + 0x92, read_u16(record + 0x30));
    write_u16(window + 0x94, 0);

    wnd_store_angle(window, 0x3C, read_i16(record + 0x34), 0x0000_0001, &mut flags);
    wnd_store_angle(window, 0x40, read_i16(record + 0x38), 0x0000_0002, &mut flags);
    wnd_store_angle(window, 0x44, read_i16(record + 0x40), 0x0001_0000, &mut flags);
    wnd_store_angle(window, 0x48, read_i16(record + 0x44), 0x0002_0000, &mut flags);
    wnd_store_angle(window, 0x4C, read_i16(record + 0x4C), 0x0020_0000, &mut flags);
    wnd_store_angle(window, 0x50, read_i16(record + 0x50), 0x0040_0000, &mut flags);
    wnd_store_angle(window, 0x54, read_i16(record + 0x58), 0x0400_0000, &mut flags);
    wnd_store_angle(window, 0x58, read_i16(record + 0x5C), 0x0800_0000, &mut flags);
    wnd_store_angle(window, 0x5C, read_i16(record + 0x3C), 0x0000_0008, &mut flags);
    wnd_store_angle(window, 0x60, read_i16(record + 0x48), 0x0008_0000, &mut flags);
    wnd_store_angle(window, 0x64, read_i16(record + 0x54), 0x0100_0000, &mut flags);
    wnd_store_angle(window, 0x68, read_i16(record + 0x60), 0x2000_0000, &mut flags);

    write_u32(window + 0x34, flags);
    write_u32(window + 0x1C, read_u32(window + 0x1C) | 0x112);
    call3(MEMSET, p32(window + 0x102), 0, 0x20);
    write_u16(window + 0x100, 0);
    write_u32(window + 0x1C, read_u32(window + 0x1C) | 0x0300_0000);
    platform::call1(FN_801B1180, lock);
}

/// Flushes the sixteen text-window records to their layout objects.
///
/// The body only runs after [`fn_80191050`] has set the guard at
/// `0x80378730`. It walks the sixteen 100-byte records at `0x803780EC`, skips
/// records whose `+0x00` layout pointer is null, and consumes four dirty bits
/// of the record flag word at `+0x04`:
///
/// * `0x20000000` - copy the pending angle at `+0x32` to `+0x30` and clear it;
/// * `0x10000000` - rebuild `+0x32` from the record index at `+0x08` and turn
///   the bit into `0x20000000`;
/// * `0x80000000` - copy the twelve angle halfwords one slot forward
///   (`+0x36` to `+0x34`, ...) and clear the bit;
/// * `0x40000000` - rebuild the six (mode `0`/`1`) or eight (mode `2`) channel
///   angles, then replace the bit with `0x80000000`.
///
/// After the flag work it writes the pending angle pair to the layout object
/// (`+0x92`/`+0x94`, the latter as `(record+0x32 - record+0x30) / 0x60`) and,
/// when the angle copy ran, writes the twelve angle/delta pairs to
/// `+0x3C..+0x6A`, the presence bits to `+0x34`, and state `0x12` to `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_80191D20` at `0x80191D20` (2,808 bytes)
/// - C file: `work/ghidra/main-gb/export/80191d20_FUN_80191d20.c`
///
/// # Arguments
/// None; the record table and guard are globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes record flags and angles, and layout-object fields `+0x1C`, `+0x34`,
/// `+0x3C..+0x6A`, `+0x92` and `+0x94`, for every live record.
///
/// # Safety
/// The guard, the record table and every non-null layout object must be mapped
/// and writable. The loop trusts the table's 1,600-byte extent exactly as the
/// original does.
///
/// # Confidence
/// high - all four flag branches, the three mode branches and the twelve
/// angle/delta pairs are literal in the C export.
pub unsafe fn fn_80191d20() {
    if read_u32(WND_TABLE_GUARD) == 0 {
        return;
    }

    let base = read_u32(WND_RECORDS) as usize;
    let mut offset = 0usize;
    let mut remaining = 0x10;
    while remaining != 0 {
        let record = base + offset;
        offset += 100;
        remaining -= 1;

        let object = read_u32(record) as usize;
        if object == 0 {
            continue;
        }

        let mut object_flags = 0u32;
        let mut record_flags = read_u32(record + 0x04);

        let mut flush_color = record_flags & 0x2000_0000 != 0;
        if flush_color {
            write_u16(record + 0x30, read_u16(record + 0x32));
            record_flags &= 0xDFFF_FFFF;
            write_u32(record + 0x04, record_flags);
        }

        if record_flags & 0x1000_0000 != 0 {
            let table_index = read_i32(record + 0x08);
            write_u16(record + 0x32, wnd_angle(table_index) as u16);
            flush_color = true;
            record_flags = (record_flags & 0xEFFF_FFFF) | 0x2000_0000;
            write_u32(record + 0x04, record_flags);
        }

        let mut rebuild = record_flags & 0x8000_0000 != 0;
        if rebuild {
            wnd_copy_pairs(record);
            record_flags &= 0x7FFF_FFFF;
            write_u32(record + 0x04, record_flags);
        }

        if record_flags & 0x4000_0000 != 0 {
            let mode = read_u32(WND_MODE);
            let f0c = read_i32(record + 0x0C);
            let f14 = read_i32(record + 0x14);

            if mode == 0 {
                let f20 = read_i32(record + 0x20);
                let f24 = read_i32(record + 0x24);

                write_u16(record + 0x36, wnd_angle(f14.wrapping_add(f20)) as u16);
                write_u16(record + 0x3A, wnd_angle(f14.wrapping_add(f20)) as u16);
                write_u16(
                    record + 0x3E,
                    wnd_angle(f14.wrapping_add(f24).wrapping_sub(0x1E)) as u16,
                );
                write_u16(
                    record + 0x42,
                    wnd_angle(f0c.wrapping_add(f20).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x46,
                    wnd_angle(f0c.wrapping_add(f20).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x4A,
                    wnd_angle(f14.wrapping_add(f0c).wrapping_add(f24).wrapping_sub(0x1E)) as u16,
                );
                for dst in [0x4E, 0x52, 0x56, 0x5A, 0x5E, 0x62] {
                    write_u16(record + dst, 0);
                }
            } else if mode == 1 {
                let f18 = read_i32(record + 0x18);
                let f1c = read_i32(record + 0x1C);
                let f20 = read_i32(record + 0x20);
                let f24 = read_i32(record + 0x24);

                write_u16(
                    record + 0x36,
                    wnd_angle(f18.wrapping_add(f20).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x3A,
                    wnd_angle(f1c.wrapping_add(f20).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x3E,
                    wnd_angle(f14.wrapping_add(f24).wrapping_sub(0x1E)) as u16,
                );
                write_u16(
                    record + 0x42,
                    wnd_angle(f20.wrapping_add(f18).wrapping_add(f14).wrapping_add(f0c)) as u16,
                );
                write_u16(
                    record + 0x46,
                    wnd_angle(f20.wrapping_add(f1c).wrapping_add(f14).wrapping_add(f0c)) as u16,
                );
                write_u16(
                    record + 0x4A,
                    wnd_angle(f14.wrapping_add(f0c).wrapping_add(f24).wrapping_sub(0x1E)) as u16,
                );
                for dst in [0x4E, 0x52, 0x56, 0x5A, 0x5E, 0x62] {
                    write_u16(record + dst, 0);
                }
            } else if mode == 2 {
                let f18 = read_i32(record + 0x18);
                let f1c = read_i32(record + 0x1C);
                let f20 = read_i32(record + 0x20);
                let f24 = read_i32(record + 0x24);
                let f28 = read_i32(record + 0x28);
                let f2c = read_i32(record + 0x2C);

                write_u16(
                    record + 0x36,
                    wnd_angle(f18.wrapping_add(f20).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x3A,
                    wnd_angle(f1c.wrapping_add(f20).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x3E,
                    wnd_angle(f28.wrapping_add(f24).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x5A,
                    wnd_angle(f2c.wrapping_add(f24).wrapping_add(f14)) as u16,
                );
                write_u16(
                    record + 0x42,
                    wnd_angle(f20.wrapping_add(f18).wrapping_add(f14).wrapping_add(f0c)) as u16,
                );
                write_u16(
                    record + 0x46,
                    wnd_angle(f20.wrapping_add(f1c).wrapping_add(f14).wrapping_add(f0c)) as u16,
                );
                write_u16(
                    record + 0x4A,
                    wnd_angle(f24.wrapping_add(f28).wrapping_add(f14).wrapping_add(f0c)) as u16,
                );
                write_u16(
                    record + 0x5E,
                    wnd_angle(f24.wrapping_add(f2c).wrapping_add(f14).wrapping_add(f0c)) as u16,
                );
                object_flags = 0x8000_0000;
                for dst in [0x4E, 0x52, 0x56, 0x62] {
                    write_u16(record + dst, 0);
                }
            }

            rebuild = true;
            record_flags = (record_flags & 0xBFFF_FFFF) | 0x8000_0000;
            write_u32(record + 0x04, record_flags);
        }

        if flush_color {
            write_u16(object + 0x92, read_u16(record + 0x30));
            let difference =
                read_u16(record + 0x32) as i32 - read_u16(record + 0x30) as i32;
            write_u16(object + 0x94, (difference / 0x60) as u16);
            write_u32(object + 0x1C, read_u32(object + 0x1C) | 0x100);
        }

        if rebuild {
            wnd_copy_channel(
                object, 0x3C, record, 0x34, 0x0000_0001, 0x0000_0004, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x40, record, 0x38, 0x0000_0002, 0x0000_0004, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x44, record, 0x40, 0x0001_0000, 0x0004_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x48, record, 0x44, 0x0002_0000, 0x0004_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x4C, record, 0x4C, 0x0020_0000, 0x0080_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x50, record, 0x50, 0x0040_0000, 0x0080_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x54, record, 0x58, 0x0400_0000, 0x1000_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x58, record, 0x5C, 0x0800_0000, 0x1000_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x5C, record, 0x3C, 0x0000_0008, 0x0000_0010, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x60, record, 0x48, 0x0008_0000, 0x0010_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x64, record, 0x54, 0x0100_0000, 0x0200_0000, &mut object_flags,
            );
            wnd_copy_channel(
                object, 0x68, record, 0x60, 0x2000_0000, 0x4000_0000, &mut object_flags,
            );

            write_u32(object + 0x34, object_flags);
            write_u32(object + 0x1C, read_u32(object + 0x1C) | 0x12);
        }
    }
}

/// Reads a big-endian `f64` from target memory.
///
/// # Original
/// Host-side helper for the two floating-point layout builders
/// (`FUN_80193220`, `FUN_801942C0`); the target stores the high word first.
///
/// # Safety
/// `addr..addr + 8` must be readable.
///
/// # Confidence
/// high - mechanical wrapper around two big-endian word reads.
#[inline]
unsafe fn read_f64(addr: usize) -> f64 {
    let high = platform::read_u32(addr) as u64;
    let low = platform::read_u32(addr + 4) as u64;
    f64::from_bits((high << 32) | low)
}

/// Calls a function taking and returning `f64`.
///
/// # Original
/// Host-side helper for the math helper `0x802E55A4` called by
/// [`fn_80193220`]; `crate::platform` only exposes integer calls.
///
/// # Safety
/// `addr` must point to a function with the `(f64, f64) -> f64` EABI shape.
///
/// # Confidence
/// high - the Ghidra signature is `double FUN_802e55a4(double, double)`.
#[inline]
unsafe fn call_f64x2_f64(addr: usize, a: f64, b: f64) -> f64 {
    let f = crate::platform::guest_fn!(addr, fn(f64, f64) -> f64);
    f(a, b)
}

/// Calls a function taking one `f64` and returning an `i32`.
///
/// # Original
/// Host-side helper for `0x802DCC4C`, the MWCC double-to-int helper used by
/// [`fn_801942c0`].
///
/// # Safety
/// `addr` must point to a function with the `(f64) -> i32` EABI shape.
///
/// # Confidence
/// high - the helper is called with a `double` and its result is stored as an
/// integer in the original.
#[inline]
unsafe fn call_f64_i32(addr: usize, value: f64) -> i32 {
    let f = crate::platform::guest_fn!(addr, fn(f64) -> i32);
    f(value)
}

/// `0x802E55A4`: frame conversion helper called with base `2.0`.
const FN_802E55A4: usize = 0x802E_55A4;
/// `0x802DCC4C`: MWCC double-to-int helper (saturating on the host).
const FN_802DCC4C: usize = 0x802D_CC4C;
/// `0x802F5F40`: `f64 2.0`, the base argument of `0x802E55A4`.
const WND_RATE_BASE: usize = 0x802F_5F40;
/// `0x802F5F48`: `f32 78643200.0` (frames per high-rate unit).
const WND_RATE_HIGH: usize = 0x802F_5F48;
/// `0x802F5F4C`: `f32 1000.0`, the scale applied to the helper result.
const WND_RATE_SCALE: usize = 0x802F_5F4C;
/// `0x802F5F50`: `f32 78642000.0` (frames per combined unit).
const WND_RATE_LOW: usize = 0x802F_5F50;
/// `0x8032D4D0`: per-row `f32` weight table (indexed by the record byte).
const WND_RATE_TABLE: usize = 0x8032_D4D0;

/// Computes one record's frame timing fields.
///
/// The body reads the text/layout object at `record + 0x14`. When its `+0x18`
/// word is the `0x80000000` sentinel, the record becomes a two-state marker:
/// `+0x30 = 1`, `+0x34 = 0`, and `+0x30 = 2`/`+0x34 = object[+0x20]` only when
/// `object + 0x1C` is also the sentinel.
///
/// Otherwise the rate value at `object + 0x18` is converted to frames through
/// `0x802E55A4(2.0, value / 78643200)`; when `object + 0x28` is not the
/// sentinel, the row-weighted value from `0x8032D4D0` is mixed in and divided
/// by `78642000`. The result is scaled by `1000` and divided by three to
/// produce a 16.16 fixed-point duration at `+0x3C` (`0x640000` when the
/// divisor is zero, `-0x3C00000 / divisor` otherwise). `+0x34` is the
/// `-0x3C00000` (-960.0) sentinel in the rate branch and `+0x38` is cleared.
///
/// When `+0x30` is below two, the same conversion is repeated from
/// `object + 0x1C` and the row-weight byte at `record + 0x0D`, and the
/// quotient is stored at `+0x40`. Finally the object's `+0x20` and `+0x24`
/// words are copied to `record + 0x44` and `record + 0x48`.
///
/// # Original
/// - Ghidra: `FUN_80193220` at `0x80193220` (716 bytes)
/// - C file: `work/ghidra/main-gb/export/80193220_FUN_80193220.c`
///
/// # Arguments
/// * `record` - message record; `+0x14` is the text/layout object and
///   `+0x0D`/`+0x0E` are the two row bytes used to index the weight table.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `record + 0x30..+0x48`; reads the object fields and the weight
/// table. Calls the float helper `0x802E55A4` by address.
///
/// # Safety
/// The record, the object at `record + 0x14`, the weight table and the float
/// helper must be mapped. The record fields are mixed 16.16 fixed-point and
/// flag words; the original stores them without conversion.
///
/// # Confidence
/// medium - the control flow and every store are literal, but the four float
/// constants are typed `f32` here because the DOL words at `0x802F5F48`,
/// `0x802F5F4C` and `0x802F5F50` are 78643200/1000/78642000 as singles, and
/// the `f64 -> i32` casts use Rust's saturating `as` (the original `fctiwz`
/// result is undefined outside the `i32` range).
pub unsafe fn fn_80193220(record: usize) {
    let object = read_u32(record + 0x14) as usize;
    let value = read_u32(object + 0x18);

    if value == 0x8000_0000 {
        write_u32(record + 0x30, 1);
        write_u32(record + 0x34, 0);
        if read_u32(object + 0x1C) == 0x8000_0000 {
            write_u32(record + 0x30, 2);
            write_u32(record + 0x34, read_u32(object + 0x20));
        }
    } else {
        // observed: the `CONCAT44(0x43300000, value ^ 0x80000000)` sequence
        // minus the `2^52 + 2^31` constant is the signed conversion of the
        // 32-bit fixed-point word.
        let signed_value = (value as i32) as f32;
        let rate = if read_u32(object + 0x28) == 0x8000_0000 {
            signed_value / read_f32(WND_RATE_HIGH)
        } else {
            let extra = (read_u32(object + 0x28) as i32) as f32;
            let row = read_u8(record + 0x0E) as usize;
            let weight = read_f32(WND_RATE_TABLE + row * 4);
            (signed_value + extra * weight) / read_f32(WND_RATE_LOW)
        };

        let converted = call_f64x2_f64(FN_802E55A4, read_f64(WND_RATE_BASE), rate as f64);
        let frames = read_f32(WND_RATE_SCALE) * (converted as f32);
        let divisor = (frames as i32) / 3;
        if divisor == 0 {
            write_u32(record + 0x38, 0);
            write_u32(record + 0x3C, 0x0064_0000);
            write_u32(record + 0x34, 0xFC40_0000);
            write_u32(record + 0x30, 0);
        } else {
            write_u32(record + 0x38, 0);
            write_u32(record + 0x34, 0xFC40_0000);
            write_u32(record + 0x30, 0);
            write_u32(record + 0x3C, (0x0064_0000i32 / divisor) as u32);
        }
    }

    if read_u32(record + 0x30) < 2 {
        let first = read_u32(object + 0x1C);
        let second = read_u32(object + 0x2C);
        let count = if first == 0x8000_0000 {
            0
        } else if second == 0x8000_0000 {
            let rate = ((first as i32) as f32) / read_f32(WND_RATE_HIGH);
            let converted = call_f64x2_f64(FN_802E55A4, read_f64(WND_RATE_BASE), rate as f64);
            (read_f32(WND_RATE_SCALE) * (converted as f32)) as i32
        } else {
            let row = read_u8(record + 0x0D) as usize;
            let weight = read_f32(WND_RATE_TABLE + row * 4);
            let sum = (first as i32) as f32 + ((second as i32) as f32) * weight;
            let rate = sum / read_f32(WND_RATE_LOW);
            let converted = call_f64x2_f64(FN_802E55A4, read_f64(WND_RATE_BASE), rate as f64);
            (read_f32(WND_RATE_SCALE) * (converted as f32)) as i32
        };

        let divisor = count / 3;
        if divisor == 0 {
            write_u32(record + 0x40, 0xFC40_0000);
        } else {
            write_u32(record + 0x40, (-0x03C0_0000i32 / divisor) as u32);
        }
    }

    write_u32(record + 0x44, read_u32(object + 0x20));
    write_u32(record + 0x48, read_u32(object + 0x24));
}

/// Parses one or more `MTrk` chunks into the object's command records.
///
/// `stream` points at the first chunk header. A chunk is an `u32` tag, an
/// `u32` payload size and the payload; the scan advances by
/// `payload + 8` until it finds the tag `0x4D54726B` (`MTrk`). For every
/// requested chunk it fills one 0x28-byte command record at `object + 0x241C`:
/// `+0x00` owner, `+0x04` payload start, `+0x08` payload end, `+0x0C` cursor,
/// `+0x18` the frame count derived from the signed halfword at `object + 0x0A`
/// (`65536 * (96 / (16000 / value))` through `0x802DCC4C`), and `+0x24` zero.
///
/// `// inferred:` the four paired-single loads at the top of the Ghidra body
/// are argument loads the decompiler could not type; they are not referenced
/// again and are not modelled. The `object`/`stream` operands are recovered
/// from the caller (`0x801943E0`) and the `0x802DCD6C` register-save helper.
///
/// # Original
/// - Ghidra: `FUN_801942C0` at `0x801942C0` (284 bytes)
/// - C file: `work/ghidra/main-gb/export/801942c0_FUN_801942c0.c`
///
/// # Arguments
/// * `object` - text-window object; its `+0x0A` halfword is the tempo value
///   and its command records start at `+0x241C`.
/// * `stream` - first `MTrk` chunk header.
/// * `count` - number of chunks to place; the original does not bound the
///   scan, so the stream must contain enough `MTrk` chunks.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `count` 0x28-byte command records and reads the stream.
///
/// # Safety
/// The stream must be readable up to the last selected chunk and the command
/// record array must be writable.
///
/// # Confidence
/// medium - the loop, record stride and all stores are literal; the hidden
/// operands and the single-precision intermediate rounding are inferred from
/// the caller and the float constants (16000/96/65536 at `0x802F5F98`,
/// `0x802F5F80`, `0x802F5F7C`).
pub unsafe fn fn_801942c0(object: usize, stream: usize, count: i32) {
    let mut chunk = stream;
    let mut record = object + 0x241C;
    let mut remaining = count;

    while remaining != 0 {
        let mut size = read_u32(chunk + 0x04);
        let mut payload = chunk + 0x08;
        while read_u32(chunk) != 0x4D54_726B {
            chunk = payload + size as usize;
            size = read_u32(chunk + 0x04);
            payload = chunk + 0x08;
        }

        write_u32(record, object as u32);
        write_u32(record + 0x04, payload as u32);
        write_u32(record + 0x08, (payload + size as usize) as u32);
        write_u32(record + 0x0C, payload as u32);

        // observed: 16000 / value, then 96 / that, then 65536 * that.
        let value = read_i16(object + 0x0A) as f32;
        let scaled = 16000.0f32 / value;
        let step = 96.0f32 / scaled;
        let frames = 65536.0f32 * step;
        write_u32(record + 0x18, call_f64_i32(FN_802DCC4C, frames as f64) as u32);

        chunk = payload + size as usize;
        write_u32(record + 0x24, 0);
        record += 0x28;
        remaining -= 1;
    }
}

// ===========================================================================
// Wave 111: animation-object constructor and text-line measurement
// (`0x80180780`, `0x80185870`, `0x80185F10`)
// ===========================================================================
//
// This pass closes the last three bodies of the `0x80180000` bucket. Two are
// the line-measurement pair a layout text box calls to turn a string into a
// rectangle plus a consumed-character count; the third is the resource
// constructor of the 0x5C-byte animation object that the pane subclasses
// allocate and store at `+0x28`.
//
// The bodies keep the original arithmetic literal: the packed count words
// `+0x4C`/`+0x50` of the animation object, the resource offset-table lookups
// and the `/ 2` character accounting are copied from the export rather than
// renamed.

/// `0x8032B2C8`: vtable installed in every layout animation object.
///
/// # Original
/// First store of [`fn_80180780`]
/// (`work/ghidra/main-gb/export/80180780_FUN_80180780.c`).
///
/// # Confidence
/// high - pointer literal in the constructor.
const VT_8032B2C8: u32 = 0x8032_B2C8;

/// `timg`: resource tag of one texture pattern.
const TAG_TIMG: u32 = 0x7469_6D67;

/// `0x802F5E50`: first of four `u16` defaults written to `object + 0x24`.
///
/// # Original
/// Read by [`fn_80180780`] before the resource copies its six colours over the
/// same twelve halfwords (`80180780_FUN_80180780.c`).
///
/// # Confidence
/// low - only the address is evidence; the four halfword values are not in the
/// export and the copy that follows always replaces them, so the reads are
/// preserved as reads instead of inventing constants.
const U16_802F5E50: usize = 0x802F_5E50;

/// `0x8032B430`: the string measured when a line overflows.
///
/// # Original
/// Passed to [`fn_80185870`] by [`fn_80185f10`] with a length of one
/// (`80185f10_FUN_80185f10.c`).
///
/// # Confidence
/// low - the address is literal; the bytes it points at are not in the export,
/// so the constant stays an address.
const DAT_8032B430: usize = 0x8032_B430;

/// Copies `len` bytes between two target addresses.
///
/// Used by [`fn_80180780`] for the resource arrays. The original copies each
/// array with a four- or eight-record unrolled `lwz`/`stw` batch; the byte loop
/// produces the same memory image.
///
/// # Original
/// Not a separate function: the unrolled copy loops of `FUN_80180780`
/// (`80180780_FUN_80180780.c`).
///
/// # Safety
/// `src + len` must be readable and `dst + len` writable.
///
/// # Confidence
/// high - the loops are plain forward copies with no gaps.
unsafe fn copy_bytes(dst: usize, src: usize, len: usize) {
    let mut offset = 0usize;
    while offset < len {
        write_u8(dst + offset, read_u8(src + offset));
        offset += 1;
    }
}

/// Writes `len` copies of one byte.
///
/// Used by [`fn_80180780`] for the zero-filled gaps of the animation work
/// buffer.
///
/// # Original
/// Not a separate function: the `0x800046B4` (`memset`) calls and the inline
/// zero loops of `FUN_80180780` (`80180780_FUN_80180780.c`).
///
/// # Safety
/// `dst + len` must be writable.
///
/// # Confidence
/// high - one value, one forward loop.
unsafe fn fill_bytes(dst: usize, value: u8, len: usize) {
    let mut offset = 0usize;
    while offset < len {
        write_u8(dst + offset, value);
        offset += 1;
    }
}

/// Constructs one layout animation object from its resource.
///
/// The object is 0x5C bytes. The body installs the vtable, starts two empty
/// lists at `+0x18..+0x24`, seeds the twelve colour halfwords at `+0x24`, masks
/// the two packed count words to their low five bits, clears the `+0x54` byte
/// and the work-buffer pointer, copies the 0x14-byte resource name and then
/// copies the resource's own twelve halfwords and four words over the defaults.
///
/// The pack word at resource `+0x3C` is then decoded into six counts. Three of
/// them are clamped (8, 10, 8) and passed to [`fn_801819c0`], which sizes the
/// work buffer and writes its packed words; the remaining three and five flag
/// bits reach the same call. When a buffer exists, the resource arrays that
/// follow the pack word are copied into the buffer slots the packed words
/// reserve:
///
/// 1. `A` - the `'timg'` texture patterns (0x20 bytes each). Every pattern is
///    cleared, then bound through [`fn_8017f250`] after a virtual `+0x0C`
///    lookup whose name comes from the resource's 8-byte offset table, and
///    finished with the entry's two trailing bytes through `0x801C7100`;
/// 2. `B` - the 0x14-byte transform records;
/// 3. `C` - the 4-byte records filled with `01 04 3C 00`;
/// 4. the four optional 4-byte words selected by descriptor bits 25, 27, 12,
///    23 and 24, each written into its own packed slot;
/// 5. `D` - the second 0x14-byte transform array, `E` - the second 4-byte
///    array (cleared first), and `F` - the 0x10-byte records (filled with
///    `FF 04 FF 00 61 00 AF FF 81 00 57 77 00 00 00 00`).
///
/// # Original
/// - Ghidra: `FUN_80180780` at `0x80180780` (4484 bytes)
/// - C file: `work/ghidra/main-gb/export/80180780_FUN_80180780.c`
///
/// # Arguments
/// * `object` - 0x5C-byte animation object to construct; also the return value.
/// * `resource` - animation resource: `+0x00` the 0x14-byte name,
///   `+0x14..+0x2A` the colour halfwords, `+0x2C..+0x38` the four words copied
///   to `+0x3C`, `+0x3C` the pack word and `+0x40` the first offset table.
/// * `block` - resource-accessor block: `+0x00` is the name-table base and
///   `+0x0C` the object whose virtual `+0x0C` resolves a `'timg'` name.
///
/// # Returns
/// `object`, matching the constructor convention of [`fn_801846a0`].
///
/// # Side effects
/// Writes the whole 0x5C-byte object, allocates the work buffer through
/// [`fn_801819c0`], and issues one virtual resource lookup per `'timg'` pattern
/// plus the [`fn_8017f250`] texture bind.
///
/// # Safety
/// `object` must be a writable 0x5C-byte allocation; `resource` must contain
/// every array the pack word announces; `block + 0x0C` must be a live accessor
/// object; the heap handle at `0x80378008` must be valid.
///
/// # Confidence
/// medium - the vtable, every offset, every count clamp and every destination
/// expression are literal; the array roles (`A`..`F`) are inferred from their
/// strides because the export names them only by packed bit position. The
/// twelve default halfwords are read from `0x802F5E50` instead of being
/// invented, and the resource copy replaces them unconditionally.
pub unsafe fn fn_80180780(object: usize, resource: usize, block: usize) -> usize {
    write_u32(object, VT_8032B2C8);
    write_u32(object + 0x18, 0);
    write_u32(object + 0x1C, p32(object + 0x1C));
    write_u32(object + 0x20, p32(object + 0x1C));
    let _ = call5(FN_802DC9A0, p32(object + 0x3C), LAB_80180770, FN_8017E490, 4, 4);

    // observed: four halfwords from the `0x802F5E50` block, then eight 0x00FF
    // halfwords. The resource copy below overwrites all twelve.
    let mut half = 0usize;
    while half < 4 {
        write_u16(object + 0x24 + half * 2, read_u16(U16_802F5E50 + half * 2));
        half += 1;
    }
    while half < 12 {
        write_u16(object + 0x24 + half * 2, 0x00FF);
        half += 1;
    }

    // observed: both packed count words keep only their low five bits, which
    // is how a fresh allocation is made safe for `fn_801819c0`.
    write_u32(object + 0x4C, read_u32(object + 0x4C) & 0x1F);
    write_u32(object + 0x50, read_u32(object + 0x50) & 0x1F);
    write_u8(object + 0x54, 0);
    write_u32(object + 0x58, 0);

    platform::call3(FN_802E19F8_STRNCPY, p32(object + 0x04), p32(resource), 0x14);

    let mut index = 0usize;
    while index < 12 {
        write_u16(
            object + 0x24 + index * 2,
            read_u16(resource + 0x14 + index * 2),
        );
        index += 1;
    }
    index = 0;
    while index < 4 {
        write_u32(
            object + 0x3C + index * 4,
            read_u32(resource + 0x2C + index * 4),
        );
        index += 1;
    }

    let descriptor = read_u32(resource + 0x3C);
    let raw_a = descriptor & 0x0F;
    let raw_b = (descriptor >> 4) & 0x0F;
    let raw_c = (descriptor >> 8) & 0x0F;
    let raw_d = (descriptor >> 13) & 0x03;
    let raw_e = (descriptor >> 15) & 0x07;
    let raw_f = (descriptor >> 18) & 0x1F;
    let flag_12 = (descriptor >> 12) & 1;
    let flag_23 = (descriptor >> 23) & 1;
    let flag_24 = (descriptor >> 24) & 1;
    let flag_25 = (descriptor >> 25) & 1;
    let flag_27 = (descriptor >> 27) & 1;

    let count_a = if raw_a < 9 { raw_a } else { 8 };
    let count_b = if raw_b < 0x0B { raw_b } else { 10 };
    let count_c = if raw_c < 9 { raw_c } else { 8 };
    let count_d = if raw_d < 4 { raw_d } else { 3 };
    let count_e = if raw_e < 5 { raw_e } else { 4 };
    let count_f = if raw_f < 0x11 { raw_f } else { 0x10 };

    fn_801819c0(
        object, count_a, count_b, count_c, count_f, flag_12, count_e, count_d, flag_25, flag_27,
        flag_23, flag_24,
    );

    let buffer = read_u32(object + 0x58) as usize;
    if buffer == 0 {
        return object;
    }

    // Packed sizes of the buffer sections. `+0x4C` never changes after
    // `fn_801819c0`; `+0x50` is re-read because this body rewrites it.
    let flags = read_u32(object + 0x4C);
    let a_size = ((flags >> 0x17) & 0x1E0) as usize;
    let b_size = (((flags >> 0x18) & 0x0F) * 0x14) as usize;
    let c_size = ((flags >> 0x12) & 0x3C) as usize;
    let d_size = (((flags >> 0x12) & 0x03) * 0x14) as usize;
    let e_size = ((flags >> 0x0D) & 0x1C) as usize;
    // The five optional 4-byte words, in the slot order the original writes
    // them: bit 25, bit 27, bit 12, bit 23, bit 24.
    let slot_25 = ((flags >> 0x06) & 4) as usize;
    let slot_27 = ((flags >> 0x05) & 4) as usize;
    let slot_23 = ((flags >> 0x04) & 4) as usize;
    let slot_24 = ((flags >> 0x03) & 4) as usize;
    let slot_12 = ((flags >> 0x0C) & 4) as usize;
    let slot_total = slot_25 + slot_27 + slot_23 + slot_24 + slot_12;

    // ---- A: 0x20-byte texture patterns ----
    if count_a != 0 {
        let present = read_u32(object + 0x50) >> 0x1C;
        let mut dst = buffer + ((read_u32(object + 0x50) >> 0x17) & 0x1E0) as usize;
        if count_a > present {
            let mut remaining = count_a - present;
            while remaining != 0 {
                fill_bytes(dst, 0, 0x20);
                dst += 0x20;
                remaining -= 1;
            }
        }
        write_u32(
            object + 0x50,
            (count_a << 0x1C) | (read_u32(object + 0x50) & 0x0FFF_FFFF),
        );
    }

    if count_a != 0 {
        let names = read_u32(block) as usize;
        let resolver = read_u32(block + 0x0C) as usize;
        let mut pattern = 0u32;
        while (pattern & 0xFF) < read_u32(object + 0x50) >> 0x1C {
            let entry = resource + 0x40 + (pattern & 0xFF) as usize * 4;
            let name_index = read_u16(entry) as usize;
            // observed: an 8-byte entry whose signed first word is the offset
            // from the name-table base + 0x0C.
            let offset = read_i32(names + 0x0C + name_index * 8);
            let name = p32(names)
                .wrapping_add(0x0C)
                .wrapping_add(offset as u32) as usize;
            let texture = platform::call4(
                vtable_method(resolver, 0, 0x0C),
                p32(resolver),
                TAG_TIMG,
                p32(name),
                0,
            );
            let record = buffer + (pattern & 0xFF) as usize * 0x20;
            fn_8017f250(record, texture as usize, 0);
            platform::call3(
                FN_801C7100,
                p32(record),
                read_u8(entry + 2) as u32,
                read_u8(entry + 3) as u32,
            );
            pattern += 1;
        }
    }

    // ---- B: 0x14-byte transform records ----
    if count_b != 0 {
        copy_bytes(
            buffer + a_size,
            resource + raw_a as usize * 4 + 0x40,
            count_b as usize * 0x14,
        );
    }

    // ---- C: 4-byte records, seeded with 01 04 3C 00 ----
    let mut cursor = raw_a as usize * 4 + 0x40 + raw_b as usize * 0x14;
    let dst_c = buffer + a_size + b_size;
    if count_c != 0 {
        let present = (read_u32(object + 0x50) >> 0x14) & 0x0F;
        if present < count_c {
            let mut p = dst_c + present as usize * 4;
            let mut remaining = count_c - present;
            while remaining != 0 {
                write_u32(p, 0x0104_3C00);
                p += 4;
                remaining -= 1;
            }
        }
        write_u32(
            object + 0x50,
            (count_c << 0x14) | (read_u32(object + 0x50) & 0xFF0F_FFFF),
        );
    }
    let mut record = 0u32;
    while record < ((read_u32(object + 0x50) >> 0x14) & 0x0F) {
        write_u32(dst_c + record as usize * 4, read_u32(resource + cursor));
        cursor += 4;
        record += 1;
    }
    cursor += raw_c as usize * 4;

    // ---- optional 4-byte words 25, 27 and 12 ----
    if flag_25 != 0 {
        copy_bytes(buffer + a_size + b_size + c_size, resource + cursor, 4);
        cursor += 4;
    }
    if flag_27 != 0 {
        copy_bytes(
            buffer + a_size + b_size + c_size + slot_25,
            resource + cursor,
            4,
        );
        cursor += 4;
    }
    if flag_12 != 0 {
        copy_bytes(
            buffer + a_size + b_size + c_size + slot_25 + slot_27,
            resource + cursor,
            4,
        );
        cursor += 4;
    }

    // ---- D: second 0x14-byte transform array ----
    if count_d != 0 {
        copy_bytes(
            buffer + a_size + b_size + c_size + slot_total + e_size,
            resource + cursor,
            count_d as usize * 0x14,
        );
    }
    cursor += raw_d as usize * 0x14;

    // ---- E: second 4-byte array, cleared before the copy ----
    if count_e != 0 {
        let dst = buffer + a_size + b_size + c_size + slot_total;
        let present = (read_u32(object + 0x50) >> 15) & 0x07;
        if present < count_e {
            let mut p = dst + present as usize * 4;
            let mut remaining = count_e - present;
            while remaining != 0 {
                write_u32(p, 0);
                p += 4;
                remaining -= 1;
            }
        }
        write_u32(
            object + 0x50,
            (count_e << 15) | (read_u32(object + 0x50) & 0xFFFC_7FFF),
        );
        copy_bytes(dst, resource + cursor, count_e as usize * 4);
    }
    cursor += raw_e as usize * 4;

    // ---- F: 0x10-byte records ----
    if count_f != 0 {
        let dst = buffer + a_size + b_size + c_size + d_size + e_size + slot_total;
        let present = (read_u32(object + 0x50) >> 9) & 0x1F;
        if present < count_f {
            let mut p = dst + present as usize * 0x10;
            let mut remaining = count_f - present;
            while remaining != 0 {
                write_u32(p, 0xFF04_FF00);
                write_u32(p + 0x04, 0x6100_AFFF);
                write_u32(p + 0x08, 0x8100_5777);
                write_u32(p + 0x0C, 0);
                p += 0x10;
                remaining -= 1;
            }
        }
        write_u32(
            object + 0x50,
            (count_f << 9) | (read_u32(object + 0x50) & 0xFFFF_C1FF),
        );
        copy_bytes(dst, resource + cursor, count_f as usize * 0x10);
    }
    cursor += raw_f as usize * 0x10;

    // ---- optional 4-byte words 23 and 24 ----
    if flag_23 != 0 {
        copy_bytes(
            buffer + a_size + b_size + c_size + slot_25 + slot_12 + slot_27,
            resource + cursor,
            4,
        );
        cursor += 4;
    }
    if flag_24 != 0 {
        copy_bytes(
            buffer + a_size + b_size + c_size + slot_12 + slot_27 + slot_23 + slot_25,
            resource + cursor,
            4,
        );
        cursor += 4;
    }
    let _ = cursor;

    object
}

/// Reads one character through the `0x802DCBFC` encoding thunk.
///
/// The thunk receives the address of the 32-bit cursor word and advances it
/// while returning the decoded character, exactly as the `&local_a8` operand of
/// [`fn_80185870`] and of the already translated [`fn_8018d540`].
///
/// # Original
/// Called by the measurement loops of `FUN_80185870`
/// (`work/ghidra/main-gb/export/80185870_FUN_80185870.c`). The thunk body is
/// outside this lane.
///
/// # Arguments
/// * `cursor` - string cursor word, advanced in place by the thunk.
///
/// # Returns
/// The decoded character, used as a `u16`.
///
/// # Side effects
/// Advances `*cursor`.
///
/// # Safety
/// `cursor` must point to a live 32-bit word and the encoding table the thunk
/// dispatches through must be installed for the string.
///
/// # Confidence
/// medium - the pointer-to-slot call shape is explicit in the export;
/// `// inferred:` the encoding table is chosen by the caller in `r12` and is
/// not modelled here.
unsafe fn next_text_char(cursor: &mut u32) -> u16 {
    platform::call1(
        FN_802DCBFC_NEXT_CHAR,
        cursor as *mut u32 as usize as u32,
    ) as u16
}

/// Measures one line of a `u16` string and reports whether it overflows.
///
/// The body seeds a four-float rectangle with the writer's line extent, resets
/// the tag state, and walks the string one decoded character at a time. The
/// budget is a character count: the loop stops once
/// `(cursor - text) / 2` exceeds `length`.
///
/// * Control characters below `0x20` are tags. The tag processor at
///   `writer + 0x5C` is called through virtual `+0x10` with the current cursor,
///   the character and a five-word state block; its four-float rectangle is
///   unioned into the output; a return of `1`/`2` clears/sets the wrap flag, `3`
///   ends the measurement and `4` ends it without re-counting.
/// * Printable characters advance the cursor by the glyph width scaled by
///   `+0x24` (horizontal layout) or by `+0x44` (vertical layout), applying the
///   `+0x4C` letter spacing first when the previous character wrapped. Only the
///   left and right extents grow.
///
/// After every step the rectangle width is compared with `width`; an
/// overflowing line sets `*overflow` and stops, and the function reports the
/// number of characters of the last line that fit.
///
/// # Original
/// - Ghidra: `FUN_80185870` at `0x80185870` (1092 bytes)
/// - C file: `work/ghidra/main-gb/export/80185870_FUN_80185870.c`
///
/// # Arguments
/// * `rect` - four-float output rectangle.
/// * `writer` - text-layout record; `+0x48` is the font, `+0x5C` the tag
///   processor, `+0x4C` the letter spacing.
/// * `text` - first character of the line (a `u16` string).
/// * `length` - character budget for the line.
/// * `overflow` - one-byte output flag; cleared, then set to `1` on overflow.
/// * `width` - maximum line width, received in `f1` by the original and read
///   back from the save-helper's extra output.
///
/// # Returns
/// The number of characters consumed (the last complete line on overflow).
///
/// # Safety
/// `text + length * 2` must be readable, `writer + 0x60` must be a valid text
/// record, `rect + 0x10` and `overflow` writable.
///
/// # Confidence
/// medium - the loop, the tag dispatch, every clamp and every store are
/// literal; the GQR blocks are integer-to-float conversion noise (reproduced as
/// the plain `u32 as f64` conversion they encode), and the `f1` width operand
/// is reconstructed from the helper's extra output.
pub unsafe fn fn_80185870(
    rect: usize,
    writer: usize,
    text: usize,
    length: i32,
    overflow: usize,
    width: f32,
) -> i32 {
    // observed: the export loads the font pointer and copies its header words
    // (+0x04, +0x08, +0x0C) into three stack slots that are never read again.
    let _font = fn_80188de0(writer);

    let mut tag_state = crate::platform::GuestLocal::new( [0u32; 5]);
    tag_state[0] = p32(writer);

    write_f32(rect, 0.0);
    write_f32(rect + 0x08, 0.0);

    let mut top = 0.0f64;
    let height = fn_8018e660(writer);
    if height < 0.0 {
        top = height;
    }
    write_f32(rect + 0x04, top as f32);

    let mut bottom = 0.0f64;
    let height = fn_8018e660(writer);
    if 0.0 < height {
        bottom = height;
    }
    write_f32(rect + 0x0C, bottom as f32);

    write_u8(overflow, 0);
    let mut saved = [
        read_f32(rect),
        read_f32(rect + 0x04),
        read_f32(rect + 0x08),
        read_f32(rect + 0x0C),
    ];

    let base = p32(text);
    let mut cursor = base;
    let mut character = next_text_char(&mut cursor);
    let mut x = 0.0f64;
    let mut last_good = 0i32;
    let mut wrap = 0i32;
    let mut finished = false;

    loop {
        if length < (cursor.wrapping_sub(base) as i32) / 2 {
            break;
        }

        if character < 0x20 {
            let mut local = crate::platform::GuestLocal::new( [0.0f32; 4]);
            local[0] = x as f32;
            local[1] = 0.0;
            local[2] = 0.0;
            local[3] = 0.0;
            tag_state[1] = cursor;
            tag_state[4] = if wrap == 0 { 1 } else { 0 };
            fn_80189e10(x, writer);

            let source = fn_8018e790(writer) as usize;
            let tag = platform::call4(
                vtable_method(source, 0, 0x10),
                p32(source),
                local.as_mut_ptr() as usize as u32,
                character as u32,
                tag_state.as_mut_ptr() as usize as u32,
            ) as i32;
            cursor = tag_state[1];

            let mut value = read_f32(rect);
            if local[0] < value {
                value = local[0];
            }
            write_f32(rect, value);

            value = read_f32(rect + 0x04);
            if local[1] < value {
                value = local[1];
            }
            write_f32(rect + 0x04, value);

            value = read_f32(rect + 0x08);
            if value < local[2] {
                value = local[2];
            }
            write_f32(rect + 0x08, value);

            value = read_f32(rect + 0x0C);
            if value < local[3] {
                value = local[3];
            }
            write_f32(rect + 0x0C, value);

            x = fn_80189e50(writer);
            if (width as f64) < (read_f32(rect + 0x08) - read_f32(rect)) as f64 {
                write_u8(overflow, 1);
                break;
            }
            if tag == 4 {
                finished = true;
                break;
            }
            if tag == 1 {
                wrap = 0;
            } else if tag == 2 {
                wrap = 1;
            } else if tag == 3 {
                break;
            }
        } else {
            if wrap != 0 {
                x = ((x + fn_8018e720(writer)) as f32) as f64;
            }
            wrap = 1;
            if fn_80189ca0(writer) == 0 {
                let letter_scale = fn_80189a60(writer);
                let font = fn_80188de0(writer) as usize;
                let glyph = platform::call2(
                    vtable_method(font, 0, 0x48),
                    p32(font),
                    character as u32,
                );
                x += glyph as f64 * letter_scale;
            } else {
                x += fn_80189cb0(writer);
            }
            x = (x as f32) as f64;

            let mut value = read_f32(rect);
            if (x as f32) < value {
                value = x as f32;
            }
            write_f32(rect, value);

            value = read_f32(rect + 0x08);
            if value < (x as f32) {
                value = x as f32;
            }
            write_f32(rect + 0x08, value);

            if (width as f64) < (read_f32(rect + 0x08) - read_f32(rect)) as f64 {
                write_u8(overflow, 1);
                break;
            }
        }

        last_good = cursor as i32;
        character = next_text_char(&mut cursor);
        saved = [
            read_f32(rect),
            read_f32(rect + 0x04),
            read_f32(rect + 0x08),
            read_f32(rect + 0x0C),
        ];
    }

    if finished {
        return length;
    }
    if read_u8(overflow) == 0 || last_good == 0 {
        (cursor.wrapping_sub(base) as i32) / 2
    } else {
        write_f32(rect, saved[0]);
        write_f32(rect + 0x04, saved[1]);
        write_f32(rect + 0x08, saved[2]);
        write_f32(rect + 0x0C, saved[3]);
        (last_good.wrapping_sub(base as i32)) / 2
    }
}

/// Measures a whole `u16` string into a four-float rectangle.
///
/// The rectangle is cleared and the writer cursor is reset to `(0, 0)`.
/// [`fn_80185870`] then measures one line at a time: the first call fills a
/// temporary rectangle and returns the number of characters that fit, and when
/// that call reports an overflow the last character is re-measured on its own
/// through the fixed string at `0x8032B430`. Either way the temporary rectangle
/// is unioned into the output (minimum left/top, maximum right/bottom), the
/// cursor advances by `2 * consumed` bytes and the budget shrinks by the
/// consumed character count.
///
/// # Original
/// - Ghidra: `FUN_80185F10` at `0x80185F10` (360 bytes)
/// - C file: `work/ghidra/main-gb/export/80185f10_FUN_80185f10.c`
///
/// # Arguments
/// * `this` - four-float output rectangle.
/// * `writer` - text-layout record passed through to [`fn_80185870`].
/// * `text` - first character of the string (a `u16` string).
/// * `length` - character budget for the whole string.
/// * `width` - maximum line width, received in `f1` by the original.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the rectangle, the writer cursor and the temporary line rectangle.
///
/// # Safety
/// `text + length * 2` must be readable, `writer + 0x60` must be a valid text
/// record, and `this + 0x10` writable.
///
/// # Confidence
/// medium - the walk, the overflow re-measure and the `*2`/`/2` accounting are
/// literal; the one-character string at `0x8032B430` and the `f1` width operand
/// are reconstructed from the call shape. The loop is unbounded when a call
/// consumes nothing, exactly as in the original.
pub unsafe fn fn_80185f10(this: usize, writer: usize, text: usize, length: i32, width: f32) {
    write_f32(this, 0.0);
    write_f32(this + 0x04, 0.0);
    write_f32(this + 0x08, 0.0);
    write_f32(this + 0x0C, 0.0);
    fn_80189e00(0.0, 0.0, writer);

    let mut cursor = text;
    let mut remaining = length;
    let mut overflow = crate::platform::GuestLocal::<u8>::new( 0u8);

    loop {
        let mut local = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let consumed = fn_80185870(
            local.as_mut_ptr() as usize,
            writer,
            cursor,
            remaining,
            (overflow.as_mut_ptr() as *mut u8) as usize,
            width,
        );
        if *overflow != 0 {
            let _ = fn_80185870(
                local.as_mut_ptr() as usize,
                writer,
                DAT_8032B430,
                1,
                (overflow.as_mut_ptr() as *mut u8) as usize,
                width,
            );
        }

        let mut value = read_f32(this);
        cursor = (cursor as u32).wrapping_add((consumed as u32).wrapping_mul(2)) as usize;
        remaining = remaining.wrapping_sub(consumed);

        if local[0] < value {
            value = local[0];
        }
        write_f32(this, value);

        value = read_f32(this + 0x04);
        if local[1] < value {
            value = local[1];
        }
        write_f32(this + 0x04, value);

        value = read_f32(this + 0x08);
        if value < local[2] {
            value = local[2];
        }
        write_f32(this + 0x08, value);

        value = read_f32(this + 0x0C);
        if value < local[3] {
            value = local[3];
        }
        write_f32(this + 0x0C, value);

        if remaining <= 0 {
            break;
        }
    }
}


// ===========================================================================
// Wave 270: `nw4hbm::lyt::Window` vtable bodies
// (vtable header `0x8032B51C`, method array `0x8032B524..0x8032B598`)
// ===========================================================================

/// `0x8017E500`: compares the pane name embedded at `+0xB4`.
const FN_8017E500: usize = 0x8017_E500;

/// `0x8017E530`: compares a name against a pane-like object's name field.
const FN_8017E530: usize = 0x8017_E530;

/// `0x8032B4E0`: the six ten-byte window glyph-cell metric records. The
/// `Window` vtable header follows the last record at `0x8032B51C`.
const WINDOW_GLYPH_RECORDS: usize = 0x8032_B4E0;

/// `0x802F5E84`: `1.0f`.
const F32_W270_ONE: f32 = 1.0;

/// `0x802F5E88`: degrees-to-radians scale (`0x3C8EFA35`).
const F32_W270_DEG2RAD: f32 = f32::from_bits(0x3C8E_FA35);

/// `0x802F5E8C`: `1.0 / 255.0` (`0x3B808081`).
const F32_W270_INV255: f32 = f32::from_bits(0x3B80_8081);

/// Reads one of the four texture coordinates of a glyph cell.
///
/// # Original (untabled)
/// Host-side decomposition of the `lfsx` picks in `0x80186A60` and
/// `0x80187040`; the original indexes a four-float stack vector by the
/// record's `+0x08`/`+0x09` bytes.
///
/// # Safety
/// `vector` is a local four-float array; the index is masked.
///
/// # Confidence
/// high - mechanical helper.
#[inline]
fn window_uv(vector: &[f32; 4], index: usize) -> f32 {
    vector[index & 3]
}

/// Builds the four texture-coordinate corners for the first metric layout.
///
/// Template A used by block 1 of `0x80186A60` and `0x80187040`: the record's
/// `+0x08`/`+0x09` bytes select two sub-offsets and the cell spans `b[0]`
/// and `b[1]`.
///
/// # Original (untabled)
/// - untabled body at `0x80186A60` block 1 and `0x80187040` block 1.
///
/// # Safety
/// `record` must be readable for at least ten bytes from both sub-offsets.
///
/// # Confidence
/// high - transcribed from the decompiled blocks.
unsafe fn window_rows_a(record: usize, vector: [f32; 4], b: [f32; 2]) -> [f32; 8] {
    let b8 = read_u8(record + 0x08) as usize;
    let b9 = read_u8(record + 0x09) as usize;
    let o8 = read_u8(record + b8) as u32;
    let o9 = read_u8(record + b9) as u32;
    let da = read_u8(record + b8 + 2) as i32 - o8 as i32;
    let db = read_u8(record + b9 + 4) as i32 - o9 as i32;
    let f8 = o8 as f32;
    let f9 = o9 as f32;

    let mut rows = [0f32; 8];
    rows[b8] = f8;
    rows[b8 + 4] = f8;
    rows[b9] = f9;
    rows[b9 + 2] = f9;
    rows[b8 + 2] = f8 + b[0] / (da as f32 * window_uv(&vector, b8));
    rows[b8 + 6] = rows[b8 + 2];
    rows[b9 + 4] = f9 + b[1] / (db as f32 * window_uv(&vector, b9));
    rows[b9 + 6] = rows[b9 + 4];
    rows
}

/// Builds the four texture-coordinate corners for the second metric layout.
///
/// Template B used by block 2 of `0x80186A60` and `0x80187040`.
///
/// # Original (untabled)
/// - untabled body at `0x80186A60` block 2 and `0x80187040` block 2.
///
/// # Safety
/// Same requirements as [`window_rows_a`].
///
/// # Confidence
/// high - transcribed from the decompiled blocks.
unsafe fn window_rows_b(record: usize, vector: [f32; 4], b: [f32; 2]) -> [f32; 8] {
    let b8 = read_u8(record + 0x08) as usize;
    let b9 = read_u8(record + 0x09) as usize;
    let p8 = read_u8(record + b8 + 2) as u32;
    let p9 = read_u8(record + b9 + 2) as u32;
    let da = read_u8(record + b8) as i32 - p8 as i32;
    let db = read_u8(record + b9 + 6) as i32 - p9 as i32;
    let f8 = p8 as f32;
    let f9 = p9 as f32;

    let mut rows = [0f32; 8];
    rows[b8 + 2] = f8;
    rows[b8 + 6] = f8;
    rows[b9] = f9;
    rows[b9 + 2] = f9;
    rows[b8] = f8 + b[0] / (da as f32 * window_uv(&vector, b8));
    rows[b8 + 4] = rows[b8];
    rows[b9 + 4] = f9 + b[1] / (db as f32 * window_uv(&vector, b9));
    rows[b9 + 6] = rows[b9 + 4];
    rows
}

/// Builds the four texture-coordinate corners for the third metric layout.
///
/// Template C used by block 3 of `0x80186A60` and `0x80187040`.
///
/// # Original (untabled)
/// - untabled body at `0x80186A60` block 3 and `0x80187040` block 3.
///
/// # Safety
/// Same requirements as [`window_rows_a`].
///
/// # Confidence
/// high - transcribed from the decompiled blocks.
unsafe fn window_rows_c(record: usize, vector: [f32; 4], b: [f32; 2]) -> [f32; 8] {
    let b8 = read_u8(record + 0x08) as usize;
    let b9 = read_u8(record + 0x09) as usize;
    let o8 = read_u8(record + b8 + 6) as u32;
    let o9 = read_u8(record + b9 + 6) as u32;
    let da = read_u8(record + b8 + 4) as i32 - o8 as i32;
    let db = read_u8(record + b9 + 2) as i32 - o9 as i32;
    let f8 = o8 as f32;
    let f9 = o9 as f32;

    let mut rows = [0f32; 8];
    rows[b8 + 2] = f8;
    rows[b8 + 6] = f8;
    rows[b9 + 4] = f9;
    rows[b9 + 6] = f9;
    rows[b8] = f8 + b[0] / (da as f32 * window_uv(&vector, b8));
    rows[b8 + 4] = rows[b8];
    rows[b9] = f9 + b[1] / (db as f32 * window_uv(&vector, b9));
    rows[b9 + 2] = rows[b9];
    rows
}

/// Builds the four texture-coordinate corners for the fourth metric layout.
///
/// Template D used by block 4 of `0x80187040`.
///
/// # Original (untabled)
/// - untabled body at `0x80187040` block 4.
///
/// # Safety
/// Same requirements as [`window_rows_a`].
///
/// # Confidence
/// high - transcribed from the decompiled block.
unsafe fn window_rows_d(record: usize, vector: [f32; 4], b: [f32; 2]) -> [f32; 8] {
    let b8 = read_u8(record + 0x08) as usize;
    let b9 = read_u8(record + 0x09) as usize;
    let o8 = read_u8(record + b8 + 4) as u32;
    let o9 = read_u8(record + b9 + 4) as u32;
    let da = read_u8(record + b8 + 6) as i32 - o8 as i32;
    let db = read_u8(record + b9) as i32 - o9 as i32;
    let f8 = o8 as f32;
    let f9 = o9 as f32;

    let mut rows = [0f32; 8];
    rows[b8] = f8;
    rows[b8 + 4] = f8;
    rows[b9 + 4] = f9;
    rows[b9 + 6] = f9;
    rows[b8 + 6] = f8 + b[0] / (da as f32 * window_uv(&vector, b8));
    rows[b8 + 2] = rows[b8 + 6];
    rows[b9] = f9 + b[1] / (db as f32 * window_uv(&vector, b9));
    rows[b9 + 2] = rows[b9];
    rows
}

/// Forwards one completed glyph cell to `0x8017EF70`.
///
/// The colour block is four `0xFFFFFFFF` words when the material lookup
/// succeeded and the null pointer otherwise, matching the original's
/// `&local_colours & (-found | found) >> 31` idiom.
///
/// # Original (untabled)
/// - untabled bodies `0x80186A60` and `0x80187040` (emitter call).
///
/// # Safety
/// `a`, `b` and `rows` are readable for their full extents; `0x8017EF70`
/// writes into the GX FIFO.
///
/// # Confidence
/// high - the emitter is the translated [`crate::ui::menu::fn_8017ef70`].
unsafe fn window_emit_cell(
    a: [f32; 2],
    b: [f32; 2],
    rows: [f32; 8],
    found: usize,
    id: u32,
    notify: bool,
) {
    // The game is given its address: keep it in guest memory.
    let mut a = crate::platform::GuestLocal::new(a);
    // The game is given its address: keep it in guest memory.
    let mut b = crate::platform::GuestLocal::new(b);
    // The game is given its address: keep it in guest memory.
    let mut rows = crate::platform::GuestLocal::new(rows);
    if notify {
        crate::ui::menu::fn_8017e920(found, 1);
    }
    let colours = crate::platform::GuestLocal::new( [0xFFFF_FFFFu32; 4]);
    let colours_pointer = if found != 0 {
        colours.as_ptr() as usize
    } else {
        0
    };
    crate::ui::menu::fn_8017ef70(
        a.as_ptr() as usize,
        b.as_ptr() as usize,
        1,
        rows.as_ptr() as u32,
        colours_pointer,
        id,
    );
}

/// Recalculates a pane's local and global matrix and propagates the draw
/// alpha to its children.
///
/// This is `nw4hbm::lyt::Pane::CalculateMtx` as invoked through the `Window`
/// vtable (`0x8032B524 + 0x08`). It early-outs unless the pane's `+0xCF` bit
/// 0 is set or the draw-info `+0x50` bit 3 (`0x08`) is set. The local matrix
/// is a scale followed by X/Y/Z rotation and translation; the result is
/// stored at `+0x54`.
///
/// # Original (untabled)
/// - Address: `0x80183C40` (768 bytes), method 2 (run offset `+0x008`, vtable offset `+0x010`)
/// - Disassembly: `work/wave270/b_dis.txt`
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `pane` - `Window` instance; rotation at `+0x38..+0x41`, translation at
///   `+0x2c..+0x35`, scale at `+0x44`/`+0x48`, alpha bytes `+0xCD`/`+0xCE`.
/// * `draw_info` - draw context; `+0x4C` is the inherited alpha, `+0x50`
///   the flags byte and `+0x44`/`+0x48` the parent scale.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the matrices at `+0x54`/`+0x84`, the alpha byte at `+0xCE`, the
/// draw-info alpha and flag bit 6, and calls virtual `+0x10` on every child.
///
/// # Safety
/// `pane` must be a live `Window`; every child must carry a valid vtable.
///
/// # Confidence
/// high - the decompiler output and the instruction listing agree on every
/// branch, matrix call and alpha restore.
pub unsafe fn fn_80183c40(pane: usize, draw_info: usize) {
    let control = read_u8(pane + 0xCF);
    let draw_flags = read_u8(draw_info + 0x50);
    if (control & 1) == 0 && (draw_flags & 0x08) == 0 {
        return;
    }

    let mut scale_xy = [read_f32(pane + 0x44), read_f32(pane + 0x48)];
    if (draw_flags & 0x10) != 0 && (control & 4) != 0 {
        scale_xy[0] *= read_f32(draw_info + 0x44);
        scale_xy[1] *= read_f32(draw_info + 0x48);
    }

    let mut scale = crate::platform::GuestLocal::new( [0f32; 12]);
    let mut rotation = crate::platform::GuestLocal::new( [0f32; 12]);
    let mut composed = crate::platform::GuestLocal::new( [0f32; 12]);
    crate::runtime::dvd::fn_801c0c60(
        scale_xy[0],
        scale_xy[1],
        F32_W270_ONE,
        scale.as_mut_ptr() as usize,
    );
    crate::runtime::dvd::fn_801c0970(
        (F32_W270_DEG2RAD * read_f32(pane + 0x38)) as f64,
        rotation.as_mut_ptr() as usize,
        0x78,
    );
    crate::runtime::dvd::fn_801c0540(
        rotation.as_ptr() as usize,
        scale.as_ptr() as usize,
        composed.as_mut_ptr() as usize,
    );
    crate::runtime::dvd::fn_801c0970(
        (F32_W270_DEG2RAD * read_f32(pane + 0x3C)) as f64,
        rotation.as_mut_ptr() as usize,
        0x79,
    );
    crate::runtime::dvd::fn_801c0540(
        rotation.as_ptr() as usize,
        composed.as_ptr() as usize,
        scale.as_mut_ptr() as usize,
    );
    crate::runtime::dvd::fn_801c0970(
        (F32_W270_DEG2RAD * read_f32(pane + 0x40)) as f64,
        rotation.as_mut_ptr() as usize,
        0x7A,
    );
    crate::runtime::dvd::fn_801c0540(
        rotation.as_ptr() as usize,
        scale.as_ptr() as usize,
        composed.as_mut_ptr() as usize,
    );
    crate::runtime::dvd::fn_801c0c10(
        read_f32(pane + 0x2C),
        read_f32(pane + 0x30),
        read_f32(pane + 0x34),
        composed.as_ptr() as usize,
        pane + 0x54,
    );

    if read_u32(pane + 0x0C) != 0 {
        crate::runtime::dvd::fn_801c0540(
            read_u32(pane + 0x0C) as usize + 0x84,
            pane + 0x54,
            pane + 0x84,
        );
    } else if (draw_flags & 0x40) != 0 {
        let mut offset = 0usize;
        while offset < 0x30 {
            write_u32(pane + 0x84 + offset, read_u32(pane + 0x54 + offset));
            offset += 4;
        }
    } else {
        crate::runtime::dvd::fn_801c0540(draw_info + 4, pane + 0x54, pane + 0x84);
    }

    if (draw_flags & 0x40) != 0 && read_u32(pane + 0x0C) != 0 {
        let scaled = read_u8(pane + 0xCD) as f32 * read_f32(draw_info + 0x4C);
        write_u8(pane + 0xCE, scaled as i32 as u8);
    } else {
        write_u8(pane + 0xCE, read_u8(pane + 0xCD));
    }

    let previous_alpha = read_f32(draw_info + 0x4C);
    let previous_flag = (draw_flags & 0x40) != 0;
    let scale_alpha = (control & 2) != 0 && read_u8(pane + 0xCD) != 0xFF;
    if scale_alpha {
        let level = read_u8(pane + 0xCD) as f32;
        write_u8(draw_info + 0x50, draw_flags | 0x40);
        write_f32(draw_info + 0x4C, F32_W270_INV255 * (previous_alpha * level));
    }

    let sentinel = pane + 0x14;
    let mut link = read_u32(pane + 0x14) as usize;
    while link != sentinel {
        let child = link - 4;
        platform::call2(vtable_method(child, 0, 0x10), p32(child), p32(draw_info));
        link = read_u32(link) as usize;
    }

    if scale_alpha {
        write_f32(draw_info + 0x4C, previous_alpha);
        let base = read_u8(draw_info + 0x50) & !0x40;
        write_u8(
            draw_info + 0x50,
            if previous_flag { base | 0x40 } else { base },
        );
    }
}

/// Notifies a pane subtree with one argument through virtual `+0x20` and
/// `+0x1C`.
///
/// The first call is made unconditionally on the pane itself. The child walk
/// over the sentinel list at `+0x14` runs when the pane's `+0xCF` bit 0 is
/// set or the argument's bit 0 is clear.
///
/// # Original (untabled)
/// - Address: `0x80184050` (140 bytes), method 5 (run offset `+0x014`, vtable offset `+0x01C`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `pane` - receiver; child list sentinel at `+0x14`.
/// * `flags` - forwarded to virtual `+0x20` and `+0x1C`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The pane and every list node must carry valid vtables.
///
/// # Confidence
/// high - the decompiled body and listing agree.
pub unsafe fn fn_80184050(pane: usize, flags: u32) {
    platform::call2(vtable_method(pane, 0, 0x20), p32(pane), flags);
    if (read_u8(pane + 0xCF) & 1) != 0 || (flags & 1) == 0 {
        let sentinel = pane + 0x14;
        let mut link = read_u32(pane + 0x14) as usize;
        while link != sentinel {
            let child = link - 4;
            platform::call2(vtable_method(child, 0, 0x1C), p32(child), flags);
            link = read_u32(link) as usize;
        }
    }
}

/// Notifies the pane children and then every window glyph object.
///
/// Runs [`fn_801840e0`] first, then under the same
/// `control bit 0 || !(flags bit 0)` condition walks the glyph array at
/// `+0xFC` and invokes virtual `+0x1C` on each entry's object.
///
/// # Original (untabled)
/// - Address: `0x80186830` (144 bytes), method 6 (run offset `+0x018`, vtable offset `+0x020`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver; glyph count at `+0x100`, array at `+0xFC`.
/// * `flags` - forwarded to [`fn_801840e0`].
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The glyph array must hold `+0x100` eight-byte entries whose second word is
/// a live object with a valid vtable.
///
/// # Confidence
/// high - the loop and condition are literal.
pub unsafe fn fn_80186830(window: usize, flags: u32) {
    fn_801840e0(window, flags);
    if (read_u8(window + 0xCF) & 1) != 0 || (flags & 1) == 0 {
        let array = read_u32(window + 0xFC) as usize;
        let count = read_u8(window + 0x100) as usize;
        let mut index = 0usize;
        while index < count {
            let glyph = read_u32(array + index * 8 + 4) as usize;
            platform::call1(vtable_method(glyph, 0, 0x1C), p32(glyph));
            index += 1;
        }
    }
}

/// Finds a pane by name, starting with the receiver's embedded name field.
///
/// Compares the name at `pane + 0xB4` through `0x8017E500`. When it does not
/// match and `recurse` is non-zero, every child is asked through virtual
/// `+0x3C(child, name, 1)`; the first non-zero result is returned.
///
/// # Original (untabled)
/// - Address: `0x80183AD0` (172 bytes), method 13 (run offset `+0x034`, vtable offset `+0x03C`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `pane` - receiver; embedded name at `+0xB4`.
/// * `name` - compared name.
/// * `recurse` - non-zero to search the children at `+0x14`.
///
/// # Returns
/// The matching pane (normally `pane` itself) or `0`.
///
/// # Safety
/// The name pointer and every child must be readable/live.
///
/// # Confidence
/// high - comparison, recursion guard and return value are literal.
pub unsafe fn fn_80183ad0(pane: usize, name: u32, recurse: i32) -> usize {
    let own = platform::call1(FN_8017E500, p32(pane + 0xB4));
    if own != 0 {
        return pane;
    }
    if recurse != 0 {
        let sentinel = pane + 0x14;
        let mut link = read_u32(pane + 0x14) as usize;
        while link != sentinel {
            let child = link - 4;
            let found = platform::call3(
                vtable_method(child, 0, 0x3C),
                p32(child),
                name,
                1,
            ) as usize;
            if found != 0 {
                return found;
            }
            link = read_u32(link) as usize;
        }
    }
    0
}

/// Finds a pane by name through the material and the window glyph objects.
///
/// The material at `+0x28`, when present, is asked first through
/// `0x8017E530(material + 4, name)`. Every glyph object at `+0xFC` is then
/// asked the same way. When `recurse` is non-zero the children at `+0x14`
/// are asked through virtual `+0x40(child, name, 1)`.
///
/// # Original (untabled)
/// - Address: `0x80186410` (252 bytes), method 14 (run offset `+0x038`, vtable offset `+0x040`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver.
/// * `name` - compared name.
/// * `recurse` - non-zero to search the children.
///
/// # Returns
/// The material, a glyph object, a child, or `0`.
///
/// # Safety
/// The material, glyph array and children must be live.
///
/// # Confidence
/// high - the three search stages and their short circuits are literal.
pub unsafe fn fn_80186410(window: usize, name: u32, recurse: i32) -> usize {
    let material = read_u32(window + 0x28) as usize;
    if material != 0 && platform::call2(FN_8017E530, p32(material + 4), name) != 0 {
        return material;
    }

    let array = read_u32(window + 0xFC) as usize;
    let count = read_u8(window + 0x100) as usize;
    let mut index = 0usize;
    while index < count {
        let glyph = read_u32(array + index * 8 + 4) as usize;
        if platform::call2(FN_8017E530, p32(glyph + 4), name) != 0 {
            return glyph;
        }
        index += 1;
    }

    if recurse != 0 {
        let sentinel = window + 0x14;
        let mut link = read_u32(window + 0x14) as usize;
        while link != sentinel {
            let child = link - 4;
            let found = platform::call3(
                vtable_method(child, 0, 0x40),
                p32(child),
                name,
                1,
            ) as usize;
            if found != 0 {
                return found;
            }
            link = read_u32(link) as usize;
        }
    }
    0
}

/// Forwards two arguments to virtual `+0x50` and then to every child's
/// virtual `+0x48`.
///
/// # Original (untabled)
/// - Address: `0x801841C0` (144 bytes), method 16 (run offset `+0x040`, vtable offset `+0x048`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `pane` - receiver.
/// * `first` - forwarded first argument.
/// * `second` - forwarded second argument; `0` skips the child walk.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The pane and every child must carry valid vtables.
///
/// # Confidence
/// high - both call sites and the guard are literal.
pub unsafe fn fn_801841c0(pane: usize, first: u32, second: u32) {
    platform::call3(
        vtable_method(pane, 0, 0x50),
        p32(pane),
        first,
        second,
    );
    if second != 0 {
        let sentinel = pane + 0x14;
        let mut link = read_u32(pane + 0x14) as usize;
        while link != sentinel {
            let child = link - 4;
            platform::call3(
                vtable_method(child, 0, 0x48),
                p32(child),
                first,
                second,
            );
            link = read_u32(link) as usize;
        }
    }
}

/// Finds a child by key, then searches the window glyph objects.
///
/// Calls [`fn_80184350`] first. When it returns zero, every glyph object in
/// the `+0xFC` array is asked through virtual `+0x20(glyph, key)`.
///
/// # Original (untabled)
/// - Address: `0x80186510` (164 bytes), method 19 (run offset `+0x04C`, vtable offset `+0x054`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver; glyph count at `+0x100`.
/// * `key` - lookup key.
///
/// # Returns
/// The first matching child or glyph, or `0`.
///
/// # Safety
/// The pane list and glyph array must be live.
///
/// # Confidence
/// high - the fallback order and return are literal.
pub unsafe fn fn_80186510(window: usize, key: u32) -> usize {
    let found = fn_80184350(window, key);
    if found != 0 {
        return found;
    }
    let array = read_u32(window + 0xFC) as usize;
    let count = read_u8(window + 0x100) as usize;
    let mut index = 0usize;
    while index < count {
        let glyph = read_u32(array + index * 8 + 4) as usize;
        let result = platform::call2(vtable_method(glyph, 0, 0x20), p32(glyph), key) as usize;
        if result != 0 {
            return result;
        }
        index += 1;
    }
    0
}

/// Forwards a key/value pair to every window glyph object and to
/// [`fn_801843d0`].
///
/// # Original (untabled)
/// - Address: `0x801865C0` (148 bytes), method 20 (run offset `+0x050`, vtable offset `+0x058`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver; glyph array at `+0xFC`.
/// * `key` - forwarded key.
/// * `value` - forwarded value.
/// * `recurse` - forwarded to [`fn_801843d0`].
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The glyph array and child list must be live.
///
/// # Confidence
/// high - the loop and the final call are literal.
pub unsafe fn fn_801865c0(window: usize, key: u32, value: u32, recurse: u32) {
    let array = read_u32(window + 0xFC) as usize;
    let count = read_u8(window + 0x100) as usize;
    let mut index = 0usize;
    while index < count {
        let glyph = read_u32(array + index * 8 + 4) as usize;
        platform::call3(
            vtable_method(glyph, 0, 0x24),
            p32(glyph),
            key,
            value,
        );
        index += 1;
    }
    fn_801843d0(window, key, value, recurse as i32);
}

/// Draws a window's glyph frame.
///
/// Invokes virtual `+0x60` (the texture-coordinate calculation), resolves the
/// flip offsets through [`fn_801845a0`], calls virtual `+0x6C` with both
/// rectangles, and dispatches on the glyph count at `+0x100`: `1`, `4` and
/// `8` select virtual `+0x70`, `+0x74` and `+0x78`.
///
/// # Original (untabled)
/// - Address: `0x801866F0` (312 bytes), method 4 (run offset `+0x010`, vtable offset `+0x018`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver.
/// * `context` - forwarded unchanged as `r4` of the first virtual call.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// Every virtual slot must exist; the glyph array must be readable.
///
/// # Confidence
/// high - the four virtual offsets, the argument order and the count dispatch
/// are explicit in both the listing and the decompiled body.
pub unsafe fn fn_801866f0(window: usize, context: u32) {
    platform::call2(vtable_method(window, 0, 0x60), p32(window), context);

    let kind = read_u8(window + 0x100) as u32;
    let mut rect = crate::platform::GuestLocal::new( [0f32; 4]);
    fn_80188440(
        rect.as_mut_ptr() as usize,
        p32(window),
        kind,
        read_u32(window + 0xFC) as usize,
    );
    let (horizontal, vertical) = fn_801845a0(window);
    let offsets = crate::platform::GuestLocal::new( [horizontal, vertical]);
    let alpha = read_u8(window + 0xCE) as u32;
    platform::call4(
        vtable_method(window, 0, 0x6C),
        p32(window),
        offsets.as_ptr() as u32,
        rect.as_ptr() as u32,
        alpha,
    );

    let slot = match kind {
        1 => 0x70,
        4 => 0x74,
        8 => 0x78,
        _ => return,
    };
    platform::call5(
        vtable_method(window, 0, slot),
        p32(window),
        offsets.as_ptr() as u32,
        read_u32(window + 0xFC),
        rect.as_ptr() as u32,
        alpha,
    );
}

/// Selects the window glyph texture-coordinate matrix.
///
/// With draw-info `+0x50` bit 7 clear the function concatenates the record at
/// `+0x04` into the pane matrix and stores the result on the stack. With the
/// bit set it copies the pane matrix, or concatenates it, depending on the
/// sign of `record[+0x40] - record[+0x38]`. The selected matrix's second
/// column (`+0x04`, `+0x14`, `+0x24`) is negated when the span is negative,
/// then published through `0x801C9940` and the range is set with `0x801C9A60(0)`.
///
/// # Original (untabled)
/// - Address: `0x80184490` (264 bytes), method 22 (run offset `+0x060`, vtable offset `+0x068`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - pane whose matrix is at `+0x84`.
/// * `record` - texture record; `+0x04` matrix, `+0x38`/`+0x40` flip span.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The matrix and record pointers must be readable; `0x801C9940` consumes the
/// selected matrix.
///
/// # Confidence
/// high - both branches, the sign test and the two GX calls are explicit.
pub unsafe fn fn_80184490(window: usize, record: usize) {
    let mut matrix = crate::platform::GuestLocal::new( [0f32; 12]);
    let negative = read_f32(record + 0x40) - read_f32(record + 0x38) < 0.0;
    let pointer = if (read_u8(record + 0x50) & 0x80) == 0 {
        crate::runtime::dvd::fn_801c0540(
            record + 4,
            window + 0x84,
            matrix.as_mut_ptr() as usize,
        );
        if negative {
            matrix[1] = -matrix[1];
            matrix[5] = -matrix[5];
            matrix[9] = -matrix[9];
        }
        matrix.as_ptr() as usize
    } else if negative {
        crate::runtime::dvd::fn_801c0500(window + 0x84, matrix.as_mut_ptr() as usize);
        matrix[1] = -matrix[1];
        matrix[5] = -matrix[5];
        matrix[9] = -matrix[9];
        matrix.as_ptr() as usize
    } else {
        window + 0x84
    };
    crate::runtime::dvd::fn_801c9940(pointer, 0);
    crate::runtime::dvd::fn_801c9a60(0);
}

/// Draws the three-cell window frame used for the single-glyph layout.
///
/// Every cell resolves the animation entry once, asks its material for the
/// matching texture object, samples the material's texture rectangle, builds
/// an eight-float four-corner row array from the selected metric record and
/// hands it to [`crate::ui::menu::fn_8017ef70`].
///
/// # Original (untabled)
/// - Address: `0x80186A60` (1500 bytes), method 26 (run offset `+0x068`, vtable offset `+0x070`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver; width at `+0x4C`, height at `+0x50`.
/// * `point` - two readable floats (`x`, `y`).
/// * `glyphs` - glyph table; entry 0 carries the material at `+0x04`.
/// * `rect` - four readable floats `(x, y, z, w)`.
/// * `id` - animation/character id.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// All pointers must be readable; the material vtable must be valid.
///
/// # Confidence
/// high - the three blocks are transcribed from the decompiled body; the
/// record offsets and both emitter calls match the instruction listing.
pub unsafe fn fn_80186a60(window: usize, point: usize, glyphs: usize, rect: usize, id: u32) {
    let entry = platform::call2(FN_8017E870, 0, id);
    let material = read_u32(glyphs + 0x04) as usize;
    let found = platform::call3(
        vtable_method(material, 0, 0x0C),
        p32(material),
        entry,
        id,
    ) as usize;
    crate::ui::menu::fn_8017e920(found, 1);

    let mut uv = crate::platform::GuestLocal::new( [0f32; 2]);
    fn_801835a0(uv.as_mut_ptr() as usize, material, 0);

    let px = read_f32(point);
    let py = read_f32(point + 0x04);
    let width = read_f32(window + 0x4C);
    let height = read_f32(window + 0x50);
    let rx = read_f32(rect);
    let ry = read_f32(rect + 0x04);
    let rz = read_f32(rect + 0x08);
    let rw = read_f32(rect + 0x0C);

    let vector_a = [uv[0], uv[1], width - ry, rz];
    window_emit_cell(
        [uv[0], uv[1]],
        [vector_a[2], vector_a[3]],
        window_rows_a(WINDOW_GLYPH_RECORDS, vector_a, [vector_a[2], vector_a[3]]),
        found,
        id,
        false,
    );

    let vector_b = [uv[0], uv[1], (px + width) - ry, py];
    window_emit_cell(
        [vector_b[2], vector_b[3]],
        [ry, height - rw],
        window_rows_b(
            WINDOW_GLYPH_RECORDS + 0x0A,
            vector_b,
            [ry, height - rw],
        ),
        found,
        id,
        false,
    );

    let vector_c = [uv[0], uv[1], px + rx, (py + height) - rw];
    window_emit_cell(
        [vector_c[2], vector_c[3]],
        [width - rx, height - rw],
        window_rows_c(
            WINDOW_GLYPH_RECORDS + 0x28,
            vector_c,
            [width - rx, height - rw],
        ),
        found,
        id,
        false,
    );
}

/// Draws the four-cell window frame used for the four-glyph layout.
///
/// The first glyph-table entry (`+0x00`) selects metric record A, `+0x08`
/// selects B, `+0x10` selects C and `+0x18` selects D. Each entry's second
/// word is the material used for that cell; every cell is committed through
/// `0x8017E920` immediately before the emitter call.
///
/// # Original (untabled)
/// - Address: `0x80187040` (1736 bytes), method 27 (run offset `+0x06C`, vtable offset `+0x074`)
/// - Ghidra: `work/wave270/b_decomp.txt`
///
/// # Arguments
/// * `window` - receiver; width at `+0x4C`, height at `+0x50`.
/// * `point` - two readable floats.
/// * `glyphs` - four eight-byte records `{ u8 metric_index; pad; material }`.
/// * `rect` - four readable floats.
/// * `id` - animation/character id.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// All pointers must be readable; every material vtable must be valid.
///
/// # Confidence
/// high - all four blocks follow the same four templates and were transcribed
/// from the decompiled body.
pub unsafe fn fn_80187040(window: usize, point: usize, glyphs: usize, rect: usize, id: u32) {
    let entry = platform::call2(FN_8017E870, 0, id);
    let px = read_f32(point);
    let py = read_f32(point + 0x04);
    let width = read_f32(window + 0x4C);
    let height = read_f32(window + 0x50);
    let rx = read_f32(rect);
    let ry = read_f32(rect + 0x04);
    let rz = read_f32(rect + 0x08);
    let rw = read_f32(rect + 0x0C);

    let mut uv = crate::platform::GuestLocal::new( [0f32; 2]);

    let material_a = read_u32(glyphs + 0x04) as usize;
    let found_a = platform::call3(
        vtable_method(material_a, 0, 0x0C),
        p32(material_a),
        entry,
        id,
    ) as usize;
    fn_801835a0(uv.as_mut_ptr() as usize, material_a, 0);
    let record_a = WINDOW_GLYPH_RECORDS + read_u8(glyphs) as usize * 10;
    let vector_a = [uv[0], uv[1], width - ry, rz];
    window_emit_cell(
        [px, py],
        [vector_a[2], vector_a[3]],
        window_rows_a(record_a, vector_a, [vector_a[2], vector_a[3]]),
        found_a,
        id,
        true,
    );

    let material_b = read_u32(glyphs + 0x0C) as usize;
    let found_b = platform::call3(
        vtable_method(material_b, 0, 0x0C),
        p32(material_b),
        entry,
        id,
    ) as usize;
    fn_801835a0(uv.as_mut_ptr() as usize, material_b, 0);
    let record_b = WINDOW_GLYPH_RECORDS + read_u8(glyphs + 0x08) as usize * 10;
    let vector_b = [uv[0], uv[1], (px + width) - ry, py];
    window_emit_cell(
        [vector_b[2], vector_b[3]],
        [ry, height - rw],
        window_rows_b(record_b, vector_b, [ry, height - rw]),
        found_b,
        id,
        true,
    );

    let material_c = read_u32(glyphs + 0x14) as usize;
    let found_c = platform::call3(
        vtable_method(material_c, 0, 0x0C),
        p32(material_c),
        entry,
        id,
    ) as usize;
    fn_801835a0(uv.as_mut_ptr() as usize, material_c, 0);
    let record_c = WINDOW_GLYPH_RECORDS + read_u8(glyphs + 0x10) as usize * 10;
    let vector_c = [uv[0], uv[1], px + rx, (py + height) - rw];
    window_emit_cell(
        [vector_c[2], vector_c[3]],
        [width - rx, height - rw],
        window_rows_c(record_c, vector_c, [width - rx, height - rw]),
        found_c,
        id,
        true,
    );

    let material_d = read_u32(glyphs + 0x1C) as usize;
    let found_d = platform::call3(
        vtable_method(material_d, 0, 0x0C),
        p32(material_d),
        entry,
        id,
    ) as usize;
    fn_801835a0(uv.as_mut_ptr() as usize, material_d, 0);
    let record_d = WINDOW_GLYPH_RECORDS + read_u8(glyphs + 0x18) as usize * 10;
    let vector_d = [uv[0], uv[1], px, py + rz];
    window_emit_cell(
        [vector_d[2], vector_d[3]],
        [rx, height - rz],
        window_rows_d(record_d, vector_d, [rx, height - rz]),
        found_d,
        id,
        true,
    );
}


// ===========================================================================
// Wave 271 (_A271): remaining `nw4hbm::lyt::Window` vtable bodies
// (header `0x8032B51C`, method array `0x8032B524..0x8032B598`, 29 slots)
// ===========================================================================
//
// The wave-270 section above translated the thirteen prologue-shaped entries
// in table order. This section closes the remaining thirteen slots:
//
//   slot  1 `+0x004` -> `0x80188580`   slot 21 `+0x054` -> `0x80184680`
//   slot  7 `+0x01C` -> `0x80186660`   slot 23 `+0x05C` -> `0x80188570`
//   slot  8 `+0x020` -> `0x80186680`   slot 24 `+0x060` -> `0x80188540`
//   slot  9 `+0x024` -> `0x80183A50`   slot 28 `+0x070` -> `0x80187710`
//   slot 10 `+0x028` -> `0x80183A80`
//   slot 11 `+0x02C` -> `0x801866B0`
//   slot 12 `+0x030` -> `0x801866D0`
//   slot 15 `+0x03C` -> `0x801841A0`
//   slot 17 `+0x044` -> `0x80184250`
//
// Slots 9, 10, 15, 17 and 21 hold the exact target words of the sibling
// `TextBox` vtable (`0x8032B43C`, same slot numbers), so the five bodies below
// serve both tables; they are keyed by `fn_<address>` and are not repeated in
// the TextBox section. The remaining eight targets are Window-only.

/// `0x80378028`: the `Window` runtime-type-info head returned by slot 1.
const A271_WINDOW_RTTI: usize = 0x8037_8028;

/// Draws one cell of the eight-cell window frame.
///
/// # Original (untabled)
/// Host-side decomposition of the eight repeated blocks in `0x80187710`, which
/// are the same four templates already factored out by wave 270. `glyph_offset`
/// selects the eight-byte glyph record (`{ u8 metric_index; pad; material }`);
/// `template` selects A/B/C/D.
///
/// # Safety
/// `glyphs`, `point`-derived arrays and the selected material must be readable.
///
/// # Confidence
/// high - each block of `0x80187710` matches the corresponding `window_rows_*`
/// template store-for-store.
unsafe fn a271_window_cell(
    glyphs: usize,
    glyph_offset: usize,
    entry: u32,
    id: u32,
    template: u8,
    a: [f32; 2],
    b: [f32; 2],
) {
    let material = read_u32(glyphs + glyph_offset + 0x04) as usize;
    let found = platform::call3(
        vtable_method(material, 0, 0x0C),
        p32(material),
        entry,
        id,
    ) as usize;
    let mut uv = crate::platform::GuestLocal::new( [0f32; 2]);
    fn_801835a0(uv.as_mut_ptr() as usize, material, 0);
    let record = WINDOW_GLYPH_RECORDS + read_u8(glyphs + glyph_offset) as usize * 10;
    let vector = [uv[0], uv[1], 0.0, 0.0];
    let rows = match template {
        0 => window_rows_a(record, vector, b),
        1 => window_rows_b(record, vector, b),
        2 => window_rows_c(record, vector, b),
        _ => window_rows_d(record, vector, b),
    };
    window_emit_cell(a, b, rows, found, id, true);
}

/// Returns the static `Window` runtime-type-info record.
///
/// # Original (untabled)
/// - Address: `0x80188580` (12 bytes), slot 1 (run offset `+0x004`, vtable offset `+0x00C`)
/// - Observed: `lis r3,0x8038`; `addi r3,r3,-32728`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Returns
/// `0x80378028`, the RTTI word immediately after TextBox's `0x80378020`.
///
/// # Safety
/// No memory is touched.
///
/// # Confidence
/// high - leaf body of three instructions.
pub unsafe fn fn_80188580() -> usize {
    A271_WINDOW_RTTI
}

/// Copies one 32-bit per-vertex colour out of the four-word block at `+0xE4`.
///
/// # Original (untabled)
/// - Address: `0x80186660` (20 bytes), slot 7 (run offset `+0x01C`, vtable offset `+0x024`)
/// - Observed: `rlwinm r0,r5,2,0,29`; `add r4,r4,r0`; `lwz r0,r4+0xE4`;
///   `stw r0,r3+0x0`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `out` - one writable word.
/// * `base` - Window receiver.
/// * `index` - colour index (`0..3`).
///
/// # Safety
/// `out` must be writable and `base + 0xE4 + index*4` readable.
///
/// # Confidence
/// high - leaf accessor.
pub unsafe fn fn_80186660(out: usize, base: usize, index: u32) {
    write_u32(out, read_u32(base + index as usize * 4 + 0xE4));
}

/// Stores four colour bytes at `+0xE4 + index*4`.
///
/// # Original (untabled)
/// - Address: `0x80186680` (44 bytes), slot 8 (run offset `+0x020`, vtable offset `+0x028`)
/// - Observed: `rlwinm r0,r4,2,0,29`; `add r4,r3,r0`; four `lbz` from `r5+0..3`
///   and four `stb` to `r4+0xE4..0xE7`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `base` - Window receiver.
/// * `index` - colour index (`0..3`).
/// * `colour` - four readable colour bytes.
///
/// # Safety
/// `base + 0xE4 + index*4` must be writable for four bytes and `colour`
/// readable for four bytes.
///
/// # Confidence
/// high - leaf accessor with no arithmetic beyond the index scale.
pub unsafe fn fn_80186680(base: usize, index: u32, colour: usize) {
    let target = base + index as usize * 4 + 0xE4;
    write_u8(target, read_u8(colour));
    write_u8(target + 1, read_u8(colour + 1));
    write_u8(target + 2, read_u8(colour + 2));
    write_u8(target + 3, read_u8(colour + 3));
}

/// Reads the pane alpha byte for vtable key `0x10`, otherwise forwards.
///
/// `bc 4,2` is the `index != 0x10` branch; when it is not taken the leaf
/// returns `+0xCD`. The taken path is a tail dispatch through virtual `+0x34`
/// (`bcctr 20,0`, no link register write), so it does not return to this body.
///
/// # Original (untabled)
/// - Address: `0x80183A50` (32 bytes), slot 9 (run offset `+0x024`, vtable offset `+0x02C`)
/// - Shared with `TextBox` slot 9 (`0x8032B460`).
/// - Observed: `cmpli cr0,0,r4,16`; `bc 4,2,+0x0C`; `lbz r3,r3+0xCD`;
///   `bclr 20,0`; fallthrough `lwz r12,[r3]`; `lwz r12,[r12+0x34]`; `bcctr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `pane` - receiver.
/// * `key` - vtable key; `0x10` selects the local alpha byte.
///
/// # Returns
/// The `+0xCD` alpha byte for key `0x10`, otherwise the virtual result.
///
/// # Safety
/// `pane` must carry a valid vtable for any other key.
///
/// # Confidence
/// high - branch and tail dispatch are both explicit.
pub unsafe fn fn_80183a50(pane: usize, key: u32) -> u32 {
    if key == 0x10 {
        read_u8(pane + 0xCD) as u32
    } else {
        platform::call2(vtable_method(pane, 0, 0x34), p32(pane), key)
    }
}

/// Writes the pane alpha byte for vtable key `0x10`, otherwise forwards.
///
/// # Original (untabled)
/// - Address: `0x80183A80` (32 bytes), slot 10 (run offset `+0x028`, vtable offset `+0x030`)
/// - Shared with `TextBox` slot 10 (`0x8032B464`).
/// - Observed: `cmpli cr0,0,r4,16`; `bc 4,2,+0x0C`; `stb r5,r3+0xCD`;
///   `bclr 20,0`; fallthrough `lwz r12,[r3]`; `lwz r12,[r12+0x38]`; `bcctr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `pane` - receiver.
/// * `key` - vtable key; `0x10` selects the local alpha byte.
/// * `value` - byte stored for key `0x10`, forwarded otherwise.
///
/// # Safety
/// `pane` must carry a valid vtable for any other key.
///
/// # Confidence
/// high - branch and tail dispatch are both explicit.
pub unsafe fn fn_80183a80(pane: usize, key: u32, value: u8) {
    if key == 0x10 {
        write_u8(pane + 0xCD, value);
    } else {
        platform::call3(
            vtable_method(pane, 0, 0x38),
            p32(pane),
            key,
            value as u32,
        );
    }
}

/// Reads one byte of the four-byte colour word at `+0xE4 + index`.
///
/// # Original (untabled)
/// - Address: `0x801866B0` (24 bytes), slot 11 (run offset `+0x02C`, vtable offset `+0x034`)
/// - Observed: `rlwinm r5,r4,0,0,29`; `rlwinm r0,r4,0,30,31`; `add r3,r3,r5`;
///   `add r3,r3,r0`; `lbz r3,r3+0xE4`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `base` - Window receiver.
/// * `index` - byte index inside the four-word colour block.
///
/// # Returns
/// The byte at `base + 0xE4 + index`.
///
/// # Safety
/// `base + 0xE4 + index` must be readable.
///
/// # Confidence
/// high - the two masks recombine to the original index.
pub unsafe fn fn_801866b0(base: usize, index: u32) -> u8 {
    read_u8(base + index as usize + 0xE4)
}

/// Writes one byte of the four-byte colour word at `+0xE4 + index`.
///
/// # Original (untabled)
/// - Address: `0x801866D0` (24 bytes), slot 12 (run offset `+0x030`, vtable offset `+0x038`)
/// - Observed: `rlwinm r6,r4,0,0,29`; `rlwinm r0,r4,0,30,31`; `add r3,r3,r6`;
///   `add r3,r3,r0`; `stb r5,r3+0xE4`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `base` - Window receiver.
/// * `index` - byte index inside the four-word colour block.
/// * `value` - byte to store.
///
/// # Safety
/// `base + 0xE4 + index` must be writable.
///
/// # Confidence
/// high - the two masks recombine to the original index.
pub unsafe fn fn_801866d0(base: usize, index: u32, value: u8) {
    write_u8(base + index as usize + 0xE4, value);
}

/// Forwards `argument` to another pane's virtual `+0x10`.
///
/// The two arguments are swapped before the tail dispatch: the incoming
/// receiver is the second argument of the virtual call.
///
/// # Original (untabled)
/// - Address: `0x801841A0` (28 bytes), slot 15 (run offset `+0x03C`, vtable offset `+0x044`)
/// - Shared with `TextBox` slot 15 (`0x8032B478`).
/// - Observed: `lwz r12,[r4]`; `mr r0,r3`; `mr r3,r4`; `lwz r12,[r12+0x10]`;
///   `mr r4,r0`; `mtspr ctr,r12`; `bcctr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `argument` - forwarded as the virtual call's second argument.
/// * `pane` - receiver of the virtual `+0x10`.
///
/// # Safety
/// `pane` must carry a valid vtable and `bcctr` never returns here.
///
/// # Confidence
/// high - all five register moves are explicit.
pub unsafe fn fn_801841a0(argument: u32, pane: usize) {
    platform::call2(vtable_method(pane, 0, 0x10), p32(pane), argument);
}

/// Forwards `value` to virtual `+0x48` with a zero second argument.
///
/// # Original (untabled)
/// - Address: `0x80184250` (24 bytes), slot 17 (run offset `+0x044`, vtable offset `+0x04C`)
/// - Shared with `TextBox` slot 17 (`0x8032B480`).
/// - Observed: `lwz r12,[r3]`; `mr r5,r4`; `li r4,0`; `lwz r12,[r12+0x48]`;
///   `mtspr ctr,r12`; `bcctr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `pane` - receiver.
/// * `value` - forwarded as the third argument.
///
/// # Safety
/// `pane` must carry a valid vtable and `bcctr` never returns here.
///
/// # Confidence
/// high - register shuffle is explicit.
pub unsafe fn fn_80184250(pane: usize, value: u32) {
    platform::call3(vtable_method(pane, 0, 0x48), p32(pane), 0, value);
}

/// Returns the pointer stored at `+0x28`.
///
/// # Original (untabled)
/// - Address: `0x80184680` (8 bytes), slot 21 (run offset `+0x054`, vtable offset `+0x05C`)
/// - Shared with `TextBox` slot 21 (`0x8032B490`).
/// - Observed: `lwz r3,r3+0x28`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `pane` - receiver.
///
/// # Returns
/// The word at `+0x28` (the owned material pointer in the destructor).
///
/// # Safety
/// `pane + 0x28` must be readable.
///
/// # Confidence
/// high - two-instruction leaf.
pub unsafe fn fn_80184680(pane: usize) -> u32 {
    read_u32(pane + 0x28)
}

/// Tail-dispatches through virtual `+0x5C`.
///
/// # Original (untabled)
/// - Address: `0x80188570` (16 bytes), slot 23 (run offset `+0x05C`, vtable offset `+0x064`)
/// - Observed: `lwz r12,[r3]`; `lwz r12,[r12+0x5C]`; `mtspr ctr,r12`;
///   `bcctr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `pane` - receiver forwarded unchanged.
///
/// # Safety
/// `pane` must carry a valid vtable and `bcctr` never returns here.
///
/// # Confidence
/// high - leaf tail dispatch.
pub unsafe fn fn_80188570(pane: usize) {
    platform::call1(vtable_method(pane, 0, 0x5C), p32(pane));
}

/// Returns the second word of indexed eight-byte records at `+0xFC`.
///
/// The count byte at `+0x100` bounds the index; an out-of-range index returns
/// zero. The `bc 12,0` (`index >= count`) branch is the early return, otherwise
/// the record's `+0x04` word is loaded.
///
/// # Original (untabled)
/// - Address: `0x80188540` (40 bytes), slot 24 (run offset `+0x060`, vtable offset `+0x068`)
/// - Observed: `lbz r0,r3+0x100`; `cmpl cr0,r4,r0`; `bc 12,0,+0x0C`;
///   `li r3,0`; `bclr 20,0`; `lwz r3,r3+0xFC`; `rlwinm r0,r4,3,0,28`;
///   `add r3,r3,r0`; `lwz r3,r3+0x4`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `base` - Window receiver.
/// * `index` - record index.
///
/// # Returns
/// The record's second word, or zero when `index >= base[0x100]`.
///
/// # Safety
/// A non-empty record array must lie in readable memory.
///
/// # Confidence
/// high - both exits and the eight-byte stride are explicit.
pub unsafe fn fn_80188540(base: usize, index: u32) -> u32 {
    if read_u8(base + 0x100) as u32 <= index {
        0
    } else {
        let array = read_u32(base + 0xFC) as usize;
        read_u32(array + index as usize * 8 + 0x04)
    }
}

/// Draws the eight-cell window frame from eight glyph-table records.
///
/// The records are processed in the original order `+0x00`, `+0x30`, `+0x08`,
/// `+0x28`, `+0x18`, `+0x38`, `+0x10`, `+0x20`: eight bytes each
/// (`{ u8 metric_index; pad; material }`). The rect is the border quad
/// `(left, right, top, bottom)`; the eight cells are the four corners and four
/// edges of a 3x3 frame. Templates pair up: A for the two top-start cells, B
/// for the two right-top cells, C for the two bottom cells, D for the two
/// left-bottom cells. Every cell uses the same animation entry, resolves the
/// material's texture object, calls `0x8017E920(found, 1)` and commits through
/// `0x8017EF70`.
///
/// `bcctr 20,0l` at `0x80187790`, `0x8018791C`, `0x80187AAC`, `0x80187C3C`,
/// `0x80187DE0`, `0x80187F7C`, `0x80188120` and `0x801882AC` are linked
/// indirect calls; execution falls through to the material/UV work, so they do
/// not terminate the body. The final `bclr 20,0` at `0x8018843C` does.
///
/// # Original (untabled)
/// - Address: `0x80187710` (3376 bytes), slot 28 (run offset `+0x070`, vtable offset `+0x078`)
/// - Observed: `stwu`/`mflr` prologue; `bl 0x802DCD48`; `bl 0x8017E870`;
///   eight `bcctr 20,0l` material lookups; eight `bl 0x801835A0`; eight
///   `bl 0x8017E920`; eight `bl 0x8017EF70`; `bl 0x802DCD94`; `bclr 20,0`
/// - Evidence: `work/wave271/a_dis.txt`
/// - Ghidra: `work/wave271/a_decomp.txt`
///
/// # Arguments
/// * `window` - receiver (`r4` in the original ABI); width at `+0x4C`, height at
///   `+0x50`.
/// * `point` - `r3`: two readable floats `(x, y)`.
/// * `glyphs` - `r5`: eight eight-byte records.
/// * `rect` - `r6`: four floats `(left, right, top, bottom)`.
/// * `id` - `r7`: animation/character id.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Resolves eight materials and emits eight textured quads through `0x8017EF70`.
///
/// # Safety
/// All pointers must be readable and every material vtable valid.
///
/// # Confidence
/// high - the eight blocks match the four wave-270 templates; the corner,
/// edge and border arithmetic was checked against the Ghidra pseudocode.
pub unsafe fn fn_80187710(window: usize, point: usize, glyphs: usize, rect: usize, id: u32) {
    let entry = platform::call2(FN_8017E870, 0, id);
    let px = read_f32(point);
    let py = read_f32(point + 0x04);
    let width = read_f32(window + 0x4C);
    let height = read_f32(window + 0x50);
    let left = read_f32(rect);
    let right = read_f32(rect + 0x04);
    let top = read_f32(rect + 0x08);
    let bottom = read_f32(rect + 0x0C);

    a271_window_cell(glyphs, 0x00, entry, id, 0, [px, py], [left, top]);
    a271_window_cell(
        glyphs,
        0x30,
        entry,
        id,
        0,
        [px + left, py],
        [(width - left) - right, top],
    );
    a271_window_cell(
        glyphs,
        0x08,
        entry,
        id,
        1,
        [(px + width) - right, py],
        [right, top],
    );
    a271_window_cell(
        glyphs,
        0x28,
        entry,
        id,
        1,
        [(px + width) - right, py + top],
        [right, (height - top) - bottom],
    );
    a271_window_cell(
        glyphs,
        0x18,
        entry,
        id,
        2,
        [(px + width) - right, (py + height) - bottom],
        [right, bottom],
    );
    a271_window_cell(
        glyphs,
        0x38,
        entry,
        id,
        2,
        [px + left, (py + height) - bottom],
        [(width - left) - right, bottom],
    );
    a271_window_cell(
        glyphs,
        0x10,
        entry,
        id,
        3,
        [px, (py + height) - bottom],
        [left, bottom],
    );
    a271_window_cell(
        glyphs,
        0x20,
        entry,
        id,
        3,
        [px, py + top],
        [left, (height - top) - bottom],
    );
}

// ===========================================================================
// Wave 271: `nw4hbm::lyt::TextBox` vtable bodies
// (header `0x8032B434`, method array `0x8032B43C..0x8032B4A4`, 27 slots)
// ===========================================================================
//
// The run is the sibling of the `Window` vtable at `0x8032B524`. Eleven
// method words are shared by the two tables - slots 2, 3, 5, 9, 10, 13, 15,
// 16, 17, 21 and 22 - and are served by one `fn_<address>` body each (six
// from the wave-270 `Window` section, five from the concurrent `0x8032B524`
// lane); this section adds only the TextBox-specific bodies. Five of the nine
// are `stwu`-shaped; the remaining four are leaf accessors that the
// recursive-descent extent still proves are complete bodies (the next known
// start follows their `bclr`).
//
// The other five uncovered TextBox slots (`0x80183A50`, `0x80183A80`,
// `0x801841A0`, `0x80184250`, `0x80184680`) are exactly the shared methods
// assigned to the concurrent `0x8032B524` lane. They are not duplicated
// here; keying by `fn_<address>` makes one body serve both vtables.

/// `0x80378020`: static head returned by `0x80186080`.
const W271_STATIC_80378020: usize = 0x8037_8020;

/// `0x8032B430`: the wide newline `"\n"` (`0x000A` followed by a NUL).
const W271_NEWLINE16: usize = 0x8032_B430;

/// `0x8018E490`: element destructor passed to `0x802DCA98` for `+0xD8`.
const FN_8018E490: usize = 0x8018_E490;

/// `0.5f` from `0x802F5EB8` (text alignment centre factor).
const F32_W271_HALF: f32 = 0.5;

/// `1.0f` from `0x802F5EBC` (text alignment right/bottom factor).
const F32_W271_ONE: f32 = 1.0;

/// Saturates one signed 16-bit colour channel to an unsigned byte.
///
/// # Original (untabled)
/// Host-side helper for the four copies of the clamp sequence in
/// `0x80185150` (`0x8018528C..0x801853DC`): `cmpi`/`li 0`/`cmpi 0xFF`/`li 255`.
///
/// # Confidence
/// high - the clamp bounds are the immediates `0` and `255`.
#[inline]
fn w271_clamp_channel(value: i32) -> u8 {
    if value < 0 {
        0
    } else if value > 0xFF {
        0xFF
    } else {
        value as u8
    }
}

/// Destroys a `nw4hbm::lyt::TextBox` pane.
///
/// Reinstalls the TextBox vtable, releases the owned font (when `+0xFD` bit 7
/// marks it as owned), resets the text metrics at `+0xE4`/`+0xE8`, releases
/// the owned material when `+0x54` says it is not shared, runs the TextBox
/// teardown virtual `+0x68`, destroys the embedded array at `+0xD8`, chains to
/// the `Pane` destructor, and finally frees the object when `delete_flag > 0`.
///
/// # Original (untabled)
/// - Address: `0x80184F20` (400 bytes), slot 0 (vtable offset `+0x008`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - receiver; `0` returns `0` without touching memory.
/// * `delete_flag` - forwarded to the final heap free (`> 0` frees).
///
/// # Returns
/// `text_box`, matching the constructor convention.
///
/// # Side effects
/// Frees the font/material, writes the vtable and `+0xE0..+0xE8`, and tears
/// down the `+0xD8` array.
///
/// # Safety
/// A non-zero `text_box` must be a live TextBox with valid owned pointers.
///
/// # Confidence
/// high - every branch, free and field store is literal in the listing.
pub unsafe fn fn_80184f20(text_box: usize, delete_flag: i32) -> usize {
    if text_box == 0 {
        return text_box;
    }

    write_u32(text_box, VT_TEXT_BOX);
    let control = read_u8(text_box + 0xFD);
    if (control & 0x80) != 0 {
        let font = read_u32(text_box + 0xE0) as usize;
        platform::call2(
            vtable_method(font, 0, 0x08),
            p32(font),
            0xFFFF_FFFFu32,
        );
        let heap = read_u32(HEAP_80378008) as usize;
        platform::call2(FN_801D5550_FREE, p32(heap), read_u32(text_box + 0xE0));
        write_u8(text_box + 0xFD, read_u8(text_box + 0xFD) & !0x80);
    }

    write_u32(text_box + 0xE0, 0);
    let zero = F32_802F5EA8.to_bits();
    write_u32(text_box + 0xE4, zero);
    write_u32(text_box + 0xE8, zero);

    let material = read_u32(text_box + 0x28) as usize;
    if material != 0 && read_u8(material + 0x54) == 0 {
        platform::call2(
            vtable_method(material, 0, 0x08),
            p32(material),
            0xFFFF_FFFFu32,
        );
        let heap = read_u32(HEAP_80378008) as usize;
        platform::call2(FN_801D5550_FREE, p32(heap), read_u32(text_box + 0x28));
        write_u32(text_box + 0x28, 0);
    }

    platform::call1(vtable_method(text_box, 0, 0x68), p32(text_box));
    platform::call4(
        FN_802DCA98,
        p32(text_box + 0xD8),
        FN_8018E490 as u32,
        4,
        2,
    );
    fn_801837d0(text_box, 0);

    if delete_flag > 0 {
        platform::call1(FN_802B9280, p32(text_box));
    }
    text_box
}

/// Returns the static object head at `0x80378020`.
///
/// # Original (untabled)
/// - Address: `0x80186080` (12 bytes), slot 1 (vtable offset `+0x00C`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Returns
/// `0x80378020`. The original materialises it with `lis r3,0x8038` /
/// `addi r3,r3,-0x7FE0`.
///
/// # Safety
/// Read-only; no target memory is touched.
///
/// # Confidence
/// high - three instructions and a `bclr`.
pub unsafe fn fn_80186080() -> usize {
    W271_STATIC_80378020
}

/// Draws a `TextBox` pane: configures a text writer, measures the string and
/// emits one aligned line at a time.
///
/// Early-outs unless the text pointer `+0xD4`, the font `+0xE0` and the
/// material `+0x28` are all non-null. It then calls virtual `+0x60` (texture
/// coordinates), builds a 0x60-byte text-writer state at the stack with
/// `0x8018E500`, installs the font and the scales at `+0xE4`/`+0xE8`, applies
/// the letter spacing `+0xEC` and line spacing `+0xF0`, derives the four
/// gradient colours from `+0xD8`/`+0xDC` scaled by the alpha `+0xCE`, clamps
/// the material colour block at `+0x24..+0x30`, and measures the whole string
/// into a rectangle with `0x80185F10`.
///
/// The rectangle is then expanded by the pane metrics from `0x801845A0` and
/// by the alignment packed in `+0xFC` (`% 3` selects left/centre/right,
/// `/ 3` selects top/middle/bottom). Each loop iteration clones the writer,
/// measures the next line with `0x80185870`, destroys the clone, shifts the
/// cursor by the horizontal alignment remainder and prints the line with
/// `0x8018FA40`; an overflow prints the wide newline at `0x8032B430`.
///
/// # Original (untabled)
/// - Address: `0x80185150` (1812 bytes), slot 4 (vtable offset `+0x018`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - receiver; text `+0xD4`, length `+0xFA`, font `+0xE0`,
///   material `+0x28`, width `+0x4C`, height `+0x50`, alignment `+0xFC`.
/// * `draw_info` - forwarded unchanged to virtual `+0x60`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the stack writer state and the pane's text cursor, and emits GX
/// glyphs through the writer/line helpers.
///
/// # Safety
/// The text, font, material and every indirect call target must be live. The
/// `0x802DCD40`/`0x802DCD8C` prologue/epilogue register-save helpers are
/// compiler bookkeeping and are omitted.
///
/// # Confidence
/// high - the guards, writer configuration, colour clamps, alignment
/// selection and per-line loop are all literal in the listing.
pub unsafe fn fn_80185150(text_box: usize, draw_info: u32) {
    if read_u32(text_box + 0xD4) == 0
        || read_u32(text_box + 0xE0) == 0
        || read_u32(text_box + 0x28) == 0
    {
        return;
    }

    platform::call2(vtable_method(text_box, 0, 0x60), p32(text_box), draw_info);

    let mut writer = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let writer_ptr = writer.as_mut_ptr() as usize;
    fn_8018e500(writer_ptr);
    fn_80188dd0(writer_ptr, read_u32(text_box + 0xE0) as usize);
    fn_80189a80(
        read_f32(text_box + 0xE4) as f64,
        read_f32(text_box + 0xE8) as f64,
        writer_ptr,
    );
    fn_8018e6f0(read_f32(text_box + 0xEC) as f64, writer_ptr);
    fn_8018e700(read_f32(text_box + 0xF0) as f64, writer_ptr);

    let alpha = read_u8(text_box + 0xCE) as u32;
    let mut gradient_a = crate::platform::GuestLocal::new( [0u8; 4]);
    let mut gradient_b = crate::platform::GuestLocal::new( [0u8; 4]);
    let gradient_a_ptr = gradient_a.as_mut_ptr() as usize;
    let gradient_b_ptr = gradient_b.as_mut_ptr() as usize;
    write_u32(gradient_a_ptr, read_u32(text_box + 0xD8));
    crate::ui::menu::menu_scale_color_alpha(gradient_a_ptr, gradient_a_ptr, alpha);
    write_u32(gradient_b_ptr, read_u32(text_box + 0xDC));
    crate::ui::menu::menu_scale_color_alpha(gradient_b_ptr, gradient_b_ptr, alpha);

    let selector = if read_u32(gradient_a_ptr) == read_u32(gradient_b_ptr) {
        0
    } else {
        2
    };
    fn_80189790(writer_ptr, selector);
    fn_801898e0(writer_ptr, gradient_a_ptr, gradient_b_ptr);

    let material = read_u32(text_box + 0x28) as usize;
    let mut primary = crate::platform::GuestLocal::new( [0u8; 4]);
    let mut secondary = crate::platform::GuestLocal::new( [0u8; 4]);
    for index in 0..4 {
        let channel = read_u16(material + 0x24 + index * 2) as i16 as i32;
        primary[index] = w271_clamp_channel(channel);
        let channel = read_u16(material + 0x2C + index * 2) as i16 as i32;
        secondary[index] = w271_clamp_channel(channel);
    }
    fn_80189740(
        writer_ptr,
        primary.as_ptr() as usize,
        secondary.as_ptr() as usize,
    );

    if read_u32(text_box + 0xF4) != 0 {
        fn_8018e770(writer_ptr, read_u32(text_box + 0xF4));
    }
    fn_80188df0(writer_ptr);

    let mut rect = crate::platform::GuestLocal::new( [0f32; 4]);
    let rect_ptr = rect.as_mut_ptr() as usize;
    fn_80189e00(0.0, 0.0, rect_ptr);
    // The original measures with a clone of the writer (copied to `r1+0xD8`)
    // and destroys the clone, so the configured writer is left untouched.
    let mut measure = crate::platform::GuestLocal::new( [0u8; 0x60]);
    let measure_ptr = measure.as_mut_ptr() as usize;
    copy_text_state(measure_ptr, writer_ptr);
    fn_80185f10(
        rect_ptr,
        measure_ptr,
        read_u32(text_box + 0xD4) as usize,
        read_u16(text_box + 0xFA) as i32,
        read_f32(text_box + 0x4C),
    );
    let _ = fn_8018e560(measure_ptr, -1);

    let (horizontal_metric, vertical_metric) = fn_801845a0(text_box);
    let alignment = read_u8(text_box + 0xFC);
    let horizontal = match alignment % 3 {
        1 => F32_W271_HALF,
        2 => F32_W271_ONE,
        _ => 0.0,
    };
    let vertical = match alignment / 3 {
        1 => F32_W271_HALF,
        2 => F32_W271_ONE,
        _ => 0.0,
    };

    let span_x = rect[2] - rect[0];
    let span_y = rect[3] - rect[1];
    rect[0] += horizontal_metric + (read_f32(text_box + 0x4C) - span_x) * horizontal;
    rect[1] += vertical_metric + (read_f32(text_box + 0x50) - span_y) * vertical;
    rect[2] = rect[0] + span_x;
    rect[3] = rect[1] + span_y;
    fn_80189e00(rect[0] as f64, rect[1] as f64, writer_ptr);

    let mut text = read_u32(text_box + 0xD4) as usize;
    let mut remaining = read_u16(text_box + 0xFA) as i32;
    while remaining > 0 {
        let mut line = crate::platform::GuestLocal::new( [0f32; 4]);
        let line_ptr = line.as_mut_ptr() as usize;
        let mut clone = crate::platform::GuestLocal::new( [0u8; 0x60]);
        let clone_ptr = clone.as_mut_ptr() as usize;
        copy_text_state(clone_ptr, writer_ptr);
        fn_80189e00(0.0, 0.0, clone_ptr);

        let mut overflow = crate::platform::GuestLocal::<u8>::new( 0u8);
        let consumed = fn_80185870(
            line_ptr,
            clone_ptr,
            text,
            remaining,
            (overflow.as_mut_ptr() as *mut u8) as usize,
            read_f32(text_box + 0x4C),
        );
        let line_width = line[2] - line[0];
        let _ = fn_8018e560(clone_ptr, -1);

        let cursor = rect[0] + horizontal * (span_x - line_width);
        fn_80189e10(cursor as f64, writer_ptr);
        fn_8018fa40(writer_ptr, text, consumed);
        if *overflow != 0 {
            fn_8018fba0(writer_ptr, W271_NEWLINE16);
        }

        text = text.wrapping_add((consumed as usize).wrapping_mul(2));
        remaining = remaining.wrapping_sub(consumed);
    }
    let _ = fn_8018e560(writer_ptr, -1);
}

/// Copies one glyph colour word out of a pane's `+0xD8` table.
///
/// # Original (untabled)
/// - Address: `0x801850B0` (20 bytes), slot 7 (vtable offset `+0x024`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `out` - destination word.
/// * `base` - pane whose `+0xD8` table is read.
/// * `index` - entry index; the original scales it with
///   `rlwinm r0,r5,1,0,29`, i.e. `(index << 1) & !3`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `out` must be writable and `base + 0xD8 + offset` readable.
///
/// # Confidence
/// high - one indexed load and one store.
pub unsafe fn fn_801850b0(out: usize, base: usize, index: u32) {
    let offset = ((index << 1) & 0xFFFF_FFFC) as usize;
    write_u32(out, read_u32(base + 0xD8 + offset));
}

/// Copies one four-byte glyph colour into a pane's `+0xD8` table.
///
/// # Original (untabled)
/// - Address: `0x801850D0` (60 bytes), slot 8 (vtable offset `+0x028`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - pane receiving the colour.
/// * `index` - entry index; scaled as `(index << 1) & !3`.
/// * `source` - readable four-byte colour word.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `source..source+4` readable and `text_box + 0xD8 + offset` writable.
///
/// # Confidence
/// high - the stack word is only a four-byte copy.
pub unsafe fn fn_801850d0(text_box: usize, index: u32, source: usize) {
    let offset = ((index << 1) & 0xFFFF_FFFC) as usize;
    write_u32(text_box + 0xD8 + offset, read_u32(source));
}

/// Reads one byte of the pane's `+0xD8` colour/glyph table.
///
/// # Original (untabled)
/// - Address: `0x80185110` (24 bytes), slot 11 (vtable offset `+0x034`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - pane owning the table.
/// * `index` - packed entry index; the original adds
///   `(index & 0x7FFF_FFFC) >> 1` and `index & 3`.
///
/// # Returns
/// The byte at `+0xD8 + offset`.
///
/// # Safety
/// `text_box + 0xD8 + offset` must be readable.
///
/// # Confidence
/// high - two `rlwinm` masks, one add and one byte load.
pub unsafe fn fn_80185110(text_box: usize, index: u32) -> u8 {
    let offset = (((index & 0x7FFF_FFFC) >> 1) + (index & 3)) as usize;
    read_u8(text_box + 0xD8 + offset)
}

/// Writes one byte of the pane's `+0xD8` colour/glyph table.
///
/// # Original (untabled)
/// - Address: `0x80185130` (24 bytes), slot 12 (vtable offset `+0x038`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - pane owning the table.
/// * `index` - packed entry index, decoded like [`fn_80185110`].
/// * `value` - byte stored at `+0xD8 + offset`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `text_box + 0xD8 + offset` must be writable.
///
/// # Confidence
/// high - same index decode as the reader, one byte store.
pub unsafe fn fn_80185130(text_box: usize, index: u32, value: u32) {
    let offset = (((index & 0x7FFF_FFFC) >> 1) + (index & 3)) as usize;
    write_u8(text_box + 0xD8 + offset, value as u8);
}

/// Finds a TextBox pane by name, checking its material first.
///
/// The material at `+0x28`, when present, is compared through `0x8017E530`
/// on the embedded name at `material + 4`. When that fails and `recurse` is
/// non-zero, every child at `+0x14` is asked through virtual
/// `+0x40(child, name, 1)`; the first non-zero answer wins.
///
/// # Original (untabled)
/// - Address: `0x80183B80` (184 bytes), slot 14 (vtable offset `+0x040`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - receiver; material `+0x28`, child sentinel `+0x14`.
/// * `name` - compared name pointer.
/// * `recurse` - non-zero to walk the children.
///
/// # Returns
/// The material, a matching child, or `0`.
///
/// # Safety
/// The material, name and every child must be live.
///
/// # Confidence
/// high - the two search stages and their short circuits are literal.
pub unsafe fn fn_80183b80(text_box: usize, name: u32, recurse: i32) -> usize {
    let material = read_u32(text_box + 0x28) as usize;
    if material != 0
        && platform::call2(FN_8017E530, p32(material + 4), name) != 0
    {
        return material;
    }

    if recurse != 0 {
        let sentinel = text_box + 0x14;
        let mut link = read_u32(text_box + 0x14) as usize;
        while link != sentinel {
            let child = link - 4;
            let found = platform::call3(
                vtable_method(child, 0, 0x40),
                p32(child),
                name,
                1,
            ) as usize;
            if found != 0 {
                return found;
            }
            link = read_u32(link) as usize;
        }
    }
    0
}

/// Sets a TextBox string through virtual `+0x70` with its measured length.
///
/// The length comes from `0x802E441C` (wide string length), truncated to
/// `u16` before the call, matching the original's `rlwinm r6,r3,0,16,31`.
///
/// # Original (untabled)
/// - Address: `0x80185D90` (104 bytes), slot 25 (vtable offset `+0x06C`)
/// - Evidence: `work/wave271/f_dis.txt`
/// - Ghidra: none - address outside `functions.tsv`
///
/// # Arguments
/// * `text_box` - receiver; virtual `+0x70` performs the text update.
/// * `text` - NUL-terminated wide string.
/// * `extra` - third argument forwarded unchanged (the call-site selector).
///
/// # Returns
/// The value returned by virtual `+0x70`.
///
/// # Safety
/// `text` must be a NUL-terminated `u16` string and the vtable valid.
///
/// # Confidence
/// high - the length call, the truncation and the tail call are explicit.
pub unsafe fn fn_80185d90(text_box: usize, text: usize, extra: u32) -> u32 {
    let length = platform::call1(FN_802E441C_STRLEN16, p32(text)) as u32 & 0xFFFF;
    platform::call4(
        vtable_method(text_box, 0, 0x70),
        p32(text_box),
        p32(text),
        extra,
        length,
    )
}

// ===========================================================================
// Wave 273, lane D - the `+0xF8` window-manager vtable bodies
// (header `0x8032DF14`, method array `0x8032DF1C..0x8032DFC0`, 42 slots)
// ===========================================================================
//
// `0x8032DF1C` is the method array installed at `+0xF8` of the 0x970-byte
// window manager by [`fn_801949b0`]:
//
//   `0x8032DEF0`  `+0x00`  `0, 0, 0x80194B90, 0x80194C50`          (2 methods)
//   `0x8032DF00`  `+0x08`  `0, 0, 0x801971A0, 0x80197140, 0x80197060`
//   `0x8032DF14`  `+0xF8`  `0, 0, 0x801972F0, ... 0x80195290`     (42 methods)
//
// Each table is a two-zero-word header followed by its method array, exactly
// like the `Window` (`0x8034F9FC`) and `TextBox` (`0x8032B43C`) tables. The
// header of this run is therefore `0x8032DF14` (two zero words at
// `0x8032DF14`/`0x8032DF18`), the method words are `0x8032DF1C + 4*slot`, the
// last slot is 41 at `0x8032DFC0`, and the two zero words at
// `0x8032DFC4`/`0x8032DFC8` are the next table's header (`0x8032DFC4`, the
// child vtable installed at manager `+0x8DC`), not a terminator.
//
// Of the 42 words, 21 are eight-byte adjustor thunks that start
// `addi r3,r3,-248` and branch unconditionally. Ten were already translated
// with the identical thunk block used by the 0x8AC-byte archive object
// (`work/wave247`/`wave248`, `src/resource/archive.rs`); this section adds the
// nine method bodies they forward to (`0x80194xxx`), and the archive section
// notes the eleven remaining thunks.
//
// Census at lane start: 42 slots, 22 covered, 20 uncovered, 42 distinct
// targets, 0 null, 0 non-text, 0 aliases, 0 target inside another listed
// function extent, 9 `stwu`-shaped prologue targets and 11 uncovered -0xF8
// thunks. All nine prologue bodies are stale `functions.tsv`/ledger gaps; none
// had a `fn_<address>` body.

/// `0x8027B160` - `(table, first, second, 0)`: the first BRSAR group lookup.
const FN_8027B160_D273: usize = 0x8027_B160;
/// `0x8027B1C0` - `(table, first, second, 0)`: the alternate group lookup.
const FN_8027B1C0_D273: usize = 0x8027_B1C0;
/// `0x8027B270` - `(table, first, second, 0)`: the notification-free lookup.
const FN_8027B270_D273: usize = 0x8027_B270;
/// `0x80275730` - `(record, second)` -> resolved group word.
const FN_80275730_D273: usize = 0x8027_5730;

// ---------------------------------------------------------------------------
// `0x80194AB0` - deleting destructor of the window manager
// ---------------------------------------------------------------------------

/// Destroys the 0x970-byte window manager and, when `delete_flag > 0`, frees
/// the storage.
///
/// This is method 21 of [`VT_WINDOW_MANAGER_F8`] (word `0x8032DF70`) and the
/// target of the `-0xF8` slot-0 thunk `0x801972F0` (`0x8032DF1C`). The body
/// reinstalls the three manager tables, releases the `+0x94C` cue object
/// through [`crate::audio::ax::fn_80194df0`], unlinks the seven list nodes at
/// `+0x8F8..+0x910`, destroys the `+0x8DC` child through [`fn_801948d0`], runs
/// the inherited 0x8AC-byte base destructor
/// [`crate::runtime::cpp::fn_802d4780`] with flag `0` and finally frees the
/// complete object when the delete flag is positive.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80194AB0`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80194AB0..0x80194B8B`.
///   `stwu r1,-0x10`; `mflr r0`; `cmpi cr0,0,r3,0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r4`; `stw r30,0x8(r1)`; `mr r30,r3`;
///   `bc 12,2,0x80194B70`; `lis r5,0x8033` / `addi r5,r5,-0x2110` (that is
///   `0x8032DEF0`) / `stw r5,0(r3)`; `addi r4,r5,0x10` / `addi r0,r5,0x24`
///   (`0x8032DF00`/`0x8032DF14`) / `stw r4,0x8(r3)` / `stw r0,0xF8(r3)`;
///   `bl 0x80194DF0`; then seven `addic. r3,r30,OFF` / `bc 12,2,+8` /
///   `bl 0x8027A200` pairs at `0x910,0x90C,0x908,0x904,0x900,0x8FC,0x8F8`;
///   `addi r3,r30,0x8DC` / `li r4,-1` / `bl 0x801948D0`; `mr r3,r30` /
///   `li r4,0` / `bl 0x802D4780`; `cmpi cr0,0,r31,0` / `bc 4,1,0x80194B70`
///   (branch if the flag is not greater than zero); `mr r3,r30` /
///   `bl 0x802B9280`; then `mr r3,r30`, restore `r31/r30/r0`, `addi r1,r1,0x10`
///   and `bclr 20,0`.
/// - extent: 220 bytes, `0x80194AB0..0x80194B8C`; one zero word at
///   `0x80194B8C` pads up to the unrelated `0x80194B90` body (table A method 0).
/// - table evidence: word `0x8032DF70`, method 21 of the 42-slot run; slot 0
///   (`0x8032DF1C`) is `0x801972F0`, `addi r3,r3,-248; b 0x80194AB0`, which
///   receives the `+0xF8` sub-object and restores the complete object.
/// - verified: `bc 4,1` is `ble`, so the free happens only for `flag > 0`; the
///   `addic.` guards test the computed member address and can never be zero.
/// - observed: [`crate::runtime::cpp::fn_802d4780`] is the inherited base
///   destructor of the 0x8AC-byte object, called with `delete_flag = 0` so the
///   derived destructor owns the single free.
///
/// # Arguments
/// - `manager`: the 0x970-byte window manager (`this`, `r3`).
/// - `delete_flag`: `r4`; `> 0` also releases the storage.
///
/// # Returns
/// `manager`, unchanged.
///
/// # Side effects
/// Installs three tables, releases the cue, unlinks seven list nodes, destroys
/// the `+0x8DC` child, runs the base teardown and may free the object.
///
/// # Safety
/// `manager` must be null or a fully constructed 0x970-byte manager whose
/// `+0x8F8..+0x910` nodes and `+0x8DC` child are live.
///
/// # Confidence
/// high - the null guard, the three table stores, all nine teardown calls and
/// the `> 0` free test are literal.
pub unsafe fn fn_80194ab0(manager: usize, delete_flag: i32) -> usize {
    if manager == 0 {
        return manager;
    }
    write_u32(manager, VT_WINDOW_MANAGER as u32);
    write_u32(manager + 0x08, VT_WINDOW_MANAGER_08 as u32);
    write_u32(manager + 0xF8, VT_WINDOW_MANAGER_F8 as u32);
    crate::audio::ax::fn_80194df0(manager as u32);
    // observed: `addic. r3,r30,OFF` is the vacuous member-address guard of the
    // emitted member destructors; the calls are unconditional.
    for offset in [0x910usize, 0x90C, 0x908, 0x904, 0x900, 0x8FC, 0x8F8] {
        crate::audio::seq::fn_8027a200(manager + offset);
    }
    fn_801948d0(manager + 0x8DC, -1);
    crate::runtime::cpp::fn_802d4780(manager, 0);
    if delete_flag > 0 {
        platform::call1(FN_802B9280, manager as u32);
    }
    manager
}

// ---------------------------------------------------------------------------
// `0x80194E70` - first group lookup through `0x8027B160`
// ---------------------------------------------------------------------------

/// Reports whether the `0x8027B160` group lookup returned no entry.
///
/// One of the four "new-style" method families that replace wave-25's
/// three-argument `0x80194E30`/`0x80194F20` pair with the notification-free
/// `0x8027B270`: the table at `*(manager + 0x5BC) + 0x0C` is queried with the
/// caller's two selectors and the result is normalised with
/// `countLeadingZeros(result) >> 5`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80194E70`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80194E70..0x80194EA3`:
///   `stwu r1,-0x10`; `mflr r0`; `li r6,0`; `stw r0,0x14(r1)`;
///   `lwz r3,0x5BC(r3)`; `addi r3,r3,0x0C`; `bl 0x8027B160`;
///   `cntlzw r3,r3`; `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 52 bytes, `0x80194E70..0x80194EA4`; three zero words pad up to
///   the `0x80194EB0` sibling.
/// - table evidence: word `0x8032DFA4`, slot 34. Slot 13 (`0x8032DF50`) is
///   `0x80197220`, `addi r3,r3,-248; b 0x80194E70`.
/// - observed: instruction-identical to [`crate::audio::ax::fn_80194e30`]
///   except that its Ghidra row exists; the body differs only in address.
///
/// # Arguments
/// - `manager`: window manager; `+0x5BC` is the group-table handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`).
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// `manager + 0x5BC` must hold a readable group-table pointer.
///
/// # Confidence
/// high - the four call arguments and the `clz >> 5` normalisation are exact.
pub unsafe fn fn_80194e70(manager: u32, first: u32, second: u32) -> u32 {
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B160_D273, table, first, second, 0);
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// `0x80194EB0` - resolved first group lookup through `0x8027B160`
// ---------------------------------------------------------------------------

/// Resolves the manager's `+0x100` record through `0x80275730` and reports
/// whether the `0x8027B160` group lookup on that mode returned no entry.
///
/// The `-1` default survives when `+0x100` is zero; otherwise the record and
/// the caller's second selector produce the lookup mode.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80194EB0`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80194EB0..0x80194F1F`:
///   `stwu r1,-0x10`; `mflr r0`; `stw r0,0x14(r1)`; `stw r31,0xC(r1)` /
///   `mr r31,r4`; `stw r30,0x8(r1)` / `mr r30,r3`;
///   `lwz r0,0x100(r3)`; `li r3,-1`; `cmpi cr0,0,r0,0`;
///   `bc 12,2,0x80194EE8`; `mr r3,r0` / `mr r4,r5` / `bl 0x80275730`; then
///   `lwz r7,0x5BC(r30)` / `mr r5,r3` / `mr r4,r31` / `li r6,0` /
///   `addi r3,r7,0x0C` / `bl 0x8027B160`; `cntlzw r3,r3` /
///   `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 112 bytes, `0x80194EB0..0x80194F20`; the already covered
///   `0x80194F20` body starts immediately.
/// - table evidence: word `0x8032DFA8`, slot 35; slot 14 (`0x8032DF54`) is
///   `0x80197230`, `addi r3,r3,-248; b 0x80194EB0`.
/// - observed: same body as [`fn_802d4530`] in
///   `src/resource/archive.rs` except that the group table is reached through
///   the manager's `+0x5BC` word instead of a view `+0x4C4`.
///
/// # Arguments
/// - `manager`: window manager; `+0x100` is the record and `+0x5BC` the handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`), also the record resolver's value.
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// One optional `0x80275730` call and one `0x8027B160` call.
///
/// # Safety
/// `+0x100` must be null or a live record; `+0x5BC` must be a readable table.
///
/// # Confidence
/// high - the `-1` default, the resolved mode and the `clz >> 5` are literal.
pub unsafe fn fn_80194eb0(manager: u32, first: u32, second: u32) -> u32 {
    let record = read_u32(manager as usize + 0x100);
    let mode = if record != 0 {
        platform::call2(FN_80275730_D273, record, second)
    } else {
        0xFFFF_FFFF
    };
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B160_D273, table, first, mode, 0);
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// `0x80194F60` - first group lookup through `0x8027B1C0`
// ---------------------------------------------------------------------------

/// Reports whether the `0x8027B1C0` group lookup returned no entry.
///
/// Same body as [`fn_80194e70`] with the alternate lookup callee, mirroring
/// the wave-25 [`crate::audio::ax::fn_80194f20`] pair.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80194F60`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80194F60..0x80194F93`:
///   `stwu r1,-0x10`; `mflr r0`; `li r6,0`; `stw r0,0x14(r1)`;
///   `lwz r3,0x5BC(r3)`; `addi r3,r3,0x0C`; `bl 0x8027B1C0`;
///   `cntlzw r3,r3`; `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 52 bytes, `0x80194F60..0x80194F94`; three zero words pad up to
///   the `0x80194FA0` sibling.
/// - table evidence: word `0x8032DFB0`, slot 37; slot 19 (`0x8032DF68`) is
///   `0x801971C0`, `addi r3,r3,-248; b 0x80194F60`.
/// - observed: instruction-identical to [`crate::audio::ax::fn_80194f20`].
///
/// # Arguments
/// - `manager`: window manager; `+0x5BC` is the group-table handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`).
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// `manager + 0x5BC` must hold a readable group-table pointer.
///
/// # Confidence
/// high - the four call arguments and the `clz >> 5` normalisation are exact.
pub unsafe fn fn_80194f60(manager: u32, first: u32, second: u32) -> u32 {
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B1C0_D273, table, first, second, 0);
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// `0x80194FA0` - resolved first group lookup through `0x8027B1C0`
// ---------------------------------------------------------------------------

/// Resolves `+0x100` through `0x80275730` and reports whether the
/// `0x8027B1C0` group lookup on that mode returned no entry.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80194FA0`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80194FA0..0x8019500F`:
///   `stwu r1,-0x10`; `mflr r0`; `stw r0,0x14(r1)`; `stw r31,0xC(r1)` /
///   `mr r31,r4`; `stw r30,0x8(r1)` / `mr r30,r3`;
///   `lwz r0,0x100(r3)`; `li r3,-1`; `cmpi cr0,0,r0,0`;
///   `bc 12,2,0x80194FD8`; `mr r3,r0` / `mr r4,r5` / `bl 0x80275730`; then
///   `lwz r7,0x5BC(r30)` / `mr r5,r3` / `mr r4,r31` / `li r6,0` /
///   `addi r3,r7,0x0C` / `bl 0x8027B1C0`; `cntlzw r3,r3` /
///   `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 112 bytes, `0x80194FA0..0x80195010`; the untabled `0x80195010`
///   body starts immediately.
/// - table evidence: word `0x8032DFB4`, slot 38; slot 20 (`0x8032DF6C`) is
///   `0x801971D0`, `addi r3,r3,-248; b 0x80194FA0`.
/// - observed: same body as [`fn_80194eb0`] with the alternate lookup.
///
/// # Arguments
/// - `manager`: window manager; `+0x100` is the record and `+0x5BC` the handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`), also the record resolver's value.
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// One optional `0x80275730` call and one `0x8027B1C0` call.
///
/// # Safety
/// `+0x100` must be null or a live record; `+0x5BC` must be a readable table.
///
/// # Confidence
/// high - the `-1` default, the resolved mode and the `clz >> 5` are literal.
pub unsafe fn fn_80194fa0(manager: u32, first: u32, second: u32) -> u32 {
    let record = read_u32(manager as usize + 0x100);
    let mode = if record != 0 {
        platform::call2(FN_80275730_D273, record, second)
    } else {
        0xFFFF_FFFF
    };
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B1C0_D273, table, first, mode, 0);
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// `0x80195210` - first group lookup through `0x8027B270`
// ---------------------------------------------------------------------------

/// Reports whether the notification-free `0x8027B270` group lookup returned no
/// entry.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80195210`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80195210..0x80195243`:
///   `stwu r1,-0x10`; `mflr r0`; `li r6,0`; `stw r0,0x14(r1)`;
///   `lwz r3,0x5BC(r3)`; `addi r3,r3,0x0C`; `bl 0x8027B270`;
///   `cntlzw r3,r3`; `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 52 bytes, `0x80195210..0x80195244`; three zero words pad up to
///   the `0x80195250` sibling.
/// - table evidence: word `0x8032DFB8`, slot 39; slot 15 (`0x8032DF58`) is
///   `0x801971E0`, `addi r3,r3,-248; b 0x80195210`.
/// - observed: the same three-instruction lookup shape as the wave-248
///   `0x802d44f0` body, using the manager's `+0x5BC` handle.
///
/// # Arguments
/// - `manager`: window manager; `+0x5BC` is the group-table handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`).
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// `manager + 0x5BC` must hold a readable group-table pointer.
///
/// # Confidence
/// high - the four call arguments and the `clz >> 5` normalisation are exact.
pub unsafe fn fn_80195210(manager: u32, first: u32, second: u32) -> u32 {
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B270_D273, table, first, second, 0);
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// `0x80195250` - second `0x8027B270` group lookup
// ---------------------------------------------------------------------------

/// Identical body to [`fn_80195210`], at a second address.
///
/// The compiler emitted the same 52-byte lookup twice because the two slots
/// belong to different translation units of the manager; both take the same
/// arguments and reach the same `0x8027B270` helper.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80195250`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80195250..0x80195283`:
///   `stwu r1,-0x10`; `mflr r0`; `li r6,0`; `stw r0,0x14(r1)`;
///   `lwz r3,0x5BC(r3)`; `addi r3,r3,0x0C`; `bl 0x8027B270`;
///   `cntlzw r3,r3`; `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 52 bytes, `0x80195250..0x80195284`; three zero words pad up to
///   the `0x80195290` sibling.
/// - table evidence: word `0x8032DFBC`, slot 40; slot 16 (`0x8032DF5C`) is
///   `0x801971F0`, `addi r3,r3,-248; b 0x80195250`.
/// - observed: byte-identical to [`fn_80195210`]; kept as a separate
///   `fn_<address>` body because both words are distinct vtable slots.
///
/// # Arguments
/// - `manager`: window manager; `+0x5BC` is the group-table handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`).
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// `manager + 0x5BC` must hold a readable group-table pointer.
///
/// # Confidence
/// high - byte-identical to [`fn_80195210`].
pub unsafe fn fn_80195250(manager: u32, first: u32, second: u32) -> u32 {
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B270_D273, table, first, second, 0);
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// `0x80195290` - resolved `0x8027B270` group lookup
// ---------------------------------------------------------------------------

/// Resolves `+0x100` through `0x80275730` and reports whether the
/// notification-free `0x8027B270` lookup on that mode returned no entry.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80195290`.
/// - Evidence: `work/dol/main.elf`, disassembly `0x80195290..0x801952FF`:
///   `stwu r1,-0x10`; `mflr r0`; `stw r0,0x14(r1)`; `stw r31,0xC(r1)` /
///   `mr r31,r4`; `stw r30,0x8(r1)` / `mr r30,r3`;
///   `lwz r0,0x100(r3)`; `li r3,-1`; `cmpi cr0,0,r0,0`;
///   `bc 12,2,0x801952C8`; `mr r3,r0` / `mr r4,r5` / `bl 0x80275730`; then
///   `lwz r7,0x5BC(r30)` / `mr r5,r3` / `mr r4,r31` / `li r6,0` /
///   `addi r3,r7,0x0C` / `bl 0x8027B270`; `cntlzw r3,r3` /
///   `rlwinm r3,r3,27,5,31`; restore; `bclr 20,0`.
/// - extent: 112 bytes, `0x80195290..0x80195300`; the untabled `0x80195300`
///   body starts immediately.
/// - table evidence: word `0x8032DFC0`, the last word of the run (slot 41);
///   slot 17 (`0x8032DF60`) is `0x80197200`, `addi r3,r3,-248`;
///   `b 0x80195290`.
/// - observed: same shape as [`fn_80194fa0`], with the `0x8027B270` lookup.
///
/// # Arguments
/// - `manager`: window manager; `+0x100` is the record and `+0x5BC` the handle.
/// - `first`: first selector (`r4`).
/// - `second`: second selector (`r5`), also the record resolver's value.
///
/// # Returns
/// `1` when the lookup returned zero, otherwise `0`.
///
/// # Side effects
/// One optional `0x80275730` call and one `0x8027B270` call.
///
/// # Safety
/// `+0x100` must be null or a live record; `+0x5BC` must be a readable table.
///
/// # Confidence
/// high - the `-1` default, the resolved mode and the `clz >> 5` are literal.
pub unsafe fn fn_80195290(manager: u32, first: u32, second: u32) -> u32 {
    let record = read_u32(manager as usize + 0x100);
    let mode = if record != 0 {
        platform::call2(FN_80275730_D273, record, second)
    } else {
        0xFFFF_FFFF
    };
    let table = read_u32(manager as usize + 0x5BC).wrapping_add(0x0C);
    let result = platform::call4(FN_8027B270_D273, table, first, mode, 0);
    (result == 0) as u32
}

// ===========================================================================
// Wave 278 lane E (`_E278`) - gap batch 8, lyt-band bodies
// ===========================================================================
//
// Two untabled gap bodies from the wave-266 candidate set, re-verified against
// `work/dol/main.elf` with the current documented extents. Selection:
// `work/wave278/e_pick.mjs`; ELF re-walk and disassembly: `e_verify.mjs` and
// `e_verify_stdout.txt`. Both start with `9421FFF0`, their recursive descents
// end on exactly one `blr`, neither overlaps a documented extent and no
// pointer-run word names either. Both are the deleting destructor that the
// `+0x08` slot of their class table names.

/// Deleting destructor of the `nw4hbm::lyt::DrawInfo` object.
///
/// Slot 0 of [`VT_DRAW_INFO`] (`0x8032B1E0`): the word at `0x8032B1E8` names
/// this body, and [`fn_8017f390`] installs that table.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8017F410`.
/// - Extent: `0x8017F410..0x8017F450` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x8017F44C`; one zero word precedes it at `0x8017F40C` and
///   the next body starts at `0x8017F450` ([`fn_8017f450`]).
/// - Reach: no `b` or `bl` in the executable segments targets it. The one data
///   word naming it, `0x8032B1E8`, is slot 0 (`+0x08`) of [`VT_DRAW_INFO`].
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `41820010 bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`;
///   `40810008 bc 4,1,+0x08` (ble); `48139E4D bl 0x802B9280`; `mr r3,r31`;
///   `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`; `mtlr r0`; `addi r1,r1,0x10`;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: `DrawInfo` object, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`, so the caller can chain the release.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live `DrawInfo` allocation.
///
/// # Confidence
/// high - the double guard, the one call and the `this` return are literal.
pub unsafe fn fn_8017f410(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(FN_802B9280, this as u32);
    }
    this
}

/// Deleting destructor of the intermediate `nw4hbm::ut::Font` base.
///
/// Slot 0 of [`VT_FONT_BASE`] (`0x8032C858`): the word at `0x8032C860` names
/// this body. That table is installed by the `ut::Font` constructor before the
/// derived table [`VT_FONT`], so this is the base-class release entry.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80185ED0`.
/// - Extent: `0x80185ED0..0x80185F10` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x80185F0C`; two zero words precede it at
///   `0x80185EC8..0x80185ED0` and the next body starts at `0x80185F10`
///   ([`fn_80185f10`]).
/// - Reach: no `b` or `bl` in the executable segments targets it. The one data
///   word naming it, `0x8032C860`, is slot 0 (`+0x08`) of [`VT_FONT_BASE`].
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `41820010 bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`;
///   `40810008 bc 4,1,+0x08` (ble); `4813338D bl 0x802B9280`; `mr r3,r31`;
///   `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`; `mtlr r0`; `addi r1,r1,0x10`;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: base sub-object, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`, so the caller can chain the release.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live `ut::Font` allocation.
///
/// # Confidence
/// high - the double guard, the one call and the `this` return are literal.
pub unsafe fn fn_80185ed0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(FN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// Wave 285 lane B (`_B285`): the `nw4hbm::lyt::Material` animation table
// (`0x8032B2C8`, 8 slots) -- seven untabled bodies
// ===========================================================================
//
// **Span.** Header `0x8032B2C8 = 0x8032B308` (non-zero: a type/name
// descriptor, not the usual zero pair), `0x8032B2CC = 0`. The descriptor at
// `0x8032B308` is `{0x8032B2F0, 0}` and `0x8032B2F0` is the NUL-terminated
// string `"nw4hbm::lyt::Material"`. The method array is
// `0x8032B2D0..0x8032B2F0` (8 slots, end-exclusive), so the name string
// immediately follows slot 7 and is also the array's terminator. The next unit
// is the table whose header is `0x8032B310 = 0x8032B1B0` (the
// `nw4hbm::lyt::detail::PaneBase` descriptor) with its method array at
// `0x8032B318` and terminator `0x8032B374`, then header
// `0x8032B378 = 0x8032B410`.
//
// **Identity.** The type descriptor names `nw4hbm::lyt::Material`. The object
// is the 0x5C-byte per-material animation transform built by
// [`fn_80180780`] and stored at pane `+0x28`; its `+0x4C`/`+0x50` packed
// counts and `+0x58` work buffer are the same fields the wave-20/wave-111
// `BRLAN` helpers in this file index. Slot 1 ([`fn_801825a0`]) programs the GX
// material state from those tracks.
//
// **Install sites.** `0x8032B2C8` is materialised at exactly two code sites
// and by one literal data word (`work/wave285/b_refs.txt`):
// `lis r12,0x8033` at `0x80180794` + `addi r12,r12,-0x4D38` at `0x8018079C`,
// stored at `0x801807A0` in the constructor [`fn_80180780`]; and the same pair
// at `0x80181934`/`0x80181938`, stored at `0x8018193C` in the destructor
// [`fn_80181910`]. The data word is the type pointer inside the vtable header
// itself.
//
// Covered slot 4 (`0x80183460`) forwards to the existing [`fn_80183460`].
// Census before this pass (`work/wave285/b_scan.txt`):
// `slots=8 covered=1 uncovered=7 prologueAmongUncovered=5 prologueAll=5
// distinct=8`; this pass closes the seven below.

/// `0x801C69E0`: selects one GX TEV stage word from a pointer.
///
/// # Original
/// Called once by [`fn_801825a0`] with the stage number and the word produced
/// by `0x8017E8E0`. `// unknown:` the concrete GX field.
const W285_FN_801C69E0: usize = 0x801C_69E0;
/// `0x8017E8E0`: maps a material track word to a 4-byte texture colour.
///
/// # Original
/// Called by [`fn_801825a0`] with `(out4, track_word, context)`.
/// `// unknown:` the exact interpolation; the three-word shape is literal.
const W285_FN_8017E8E0: usize = 0x8017_E8E0;
/// `0x801C7120`: applies one texture-object index.
const W285_FN_801C7120: usize = 0x801C_7120;
/// `0x801C7170`: returns the texture-object record behind one 0x20-byte entry.
const W285_FN_801C7170: usize = 0x801C_7170;
/// `0x801C7220`: classifies one 0x20-byte texture record.
const W285_FN_801C7220: usize = 0x801C_7220;
/// `0x801C7520`: loads one texture record into a GX texture object.
const W285_FN_801C7520: usize = 0x801C_7520;
/// `0x801C7550`: finishes the texture-object load started by `0x801C7520`.
const W285_FN_801C7550: usize = 0x801C_7550;
/// `0x801C74C0`: releases one temporary texture object.
const W285_FN_801C74C0: usize = 0x801C_74C0;
/// `0x801C7D90`: ten-argument TEV colour/alpha writer.
///
/// # Original
/// Called once by [`fn_801825a0`] with the eight register words and two stack
/// words of the 16-byte texture record. `// unknown:` the concrete GX field.
const W285_FN_801C7D90: usize = 0x801C_7D90;
/// `0x801C7F40`: writes one texture filter pair.
const W285_FN_801C7F40: usize = 0x801C_7F40;
/// `0x801C8040`: writes one texture wrap pair.
const W285_FN_801C8040: usize = 0x801C_8040;
/// `0x801C8490`: writes one two-word material colour block.
const W285_FN_801C8490: usize = 0x801C_8490;
/// `0x801C8500`: writes one material colour word.
const W285_FN_801C8500: usize = 0x801C_8500;
/// `0x801C8560`: writes one TEV colour-channel word.
const W285_FN_801C8560: usize = 0x801C_8560;
/// `0x801C85B0`: writes one TEV alpha-channel word.
const W285_FN_801C85B0: usize = 0x801C_85B0;
/// `0x801C86C0`: writes one indirect-stage word.
const W285_FN_801C86C0: usize = 0x801C_86C0;
/// `0x801C9A80`: uploads one 0x60-byte matrix into the GX matrix slot `+30`.
const FN_801C9A80: usize = 0x801C_9A80;
/// `0x8033B288`: array of TEV colour-channel words used by the long-stage arm.
const W285_TEV_COLOUR: usize = 0x8033_B288;
/// `0x8033B2A8`: array of TEV alpha-channel words used by the long-stage arm.
const W285_TEV_ALPHA: usize = 0x8033_B2A8;

/// Calls a ten-argument integer function.
///
/// # Original
/// Host-side helper; no binary address. Models `0x801C7D90`, whose last two
/// arguments are passed on the stack (`sp+8`/`sp+0xC` at the call site).
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn w285_call10(
    addr: usize,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32);
    f(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9)
}

/// Destroys one `nw4hbm::lyt::Material` animation object.
///
/// Reinstalls the header, runs virtual `+0x18`, frees the `+0x58` work buffer,
/// releases the `+0x3C` array through `0x802DCA98`, clears the `+0x18` list and
/// frees the object when the release flag is positive.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80181910`.
/// - Extent: `0x80181910..0x801819C0` (176 bytes, 44 insns).
/// - Table: slot 0 (`0x8032B2D0`).
/// - Evidence: `work/wave285/b_dis.txt`.
///
/// # Arguments
/// - `object`: 0x5C-byte animation object, or `0`.
/// - `release`: positive releases `object`.
///
/// # Returns
/// `object`.
///
/// # Safety
/// A non-zero `object` must be a live allocation with valid owned pointers.
///
/// # Confidence
/// high - every guard, free, virtual call and field store is literal.
pub unsafe fn fn_80181910(object: usize, release: i32) -> usize {
    if object == 0 {
        return object;
    }
    write_u32(object, VT_8032B2C8);
    platform::call1(vtable_method(object, 0, 0x18), p32(object));
    let buffer = read_u32(object + 0x58) as usize;
    if buffer != 0 {
        platform::call2(FN_801D5550_FREE, read_u32(HEAP_80378008), buffer as u32);
        write_u32(object + 0x58, 0);
    }
    platform::call4(
        FN_802DCA98,
        p32(object + 0x3C),
        FN_8018E490 as u32,
        4,
        4,
    );
    fn_8018a4b0((object + 0x18) as *mut u8, 0);
    if release > 0 {
        platform::call1(FN_802B9280, p32(object));
    }
    object
}

/// Binds the material's GX state from its animation tracks.
///
/// The packed words `+0x4C`/`+0x50` select eight parallel work-buffer sections
/// (material colours, texture SRT, texture patterns, indirect stages and the
/// two colour-channel tables). The body mirrors the GX state sequence of
/// [`fn_80188df0`] and [`fn_8018a100`]: per-record `0x801C8790` colour writes,
/// `0x801C82F0`/`0x801C8330` channel selectors, `0x801C8370`/`0x801C83D0`
/// swap tables, and the final `0x801C8110`/`0x801C86C0`/`0x801C8DC0`
/// stage writes.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x801825A0`.
/// - Extent: `0x801825A0..0x801833BC` (3612 bytes, 903 insns).
/// - Table: slot 1 (`0x8032B2D4`).
/// - Evidence: `work/wave285/b_big.txt`.
///
/// # Arguments
/// - `object`: animation object; `+0x4C`/`+0x50` pack counts, `+0x58` buffer.
/// - `flag`: argument that enables the `r29` predicate returned at the end.
/// - `context`: forwarded to `0x8017E8E0`.
///
/// # Returns
/// `1` when both the track predicate and the `flag` predicate hold, else `0`.
///
/// # Safety
/// The packed counts must describe a buffer that is live for every indexed
/// section; the GX entry points must be mapped.
///
/// # Confidence
/// medium - every branch, offset, mask and call is literal; the GX field
/// names and the `0x801C88F0` stage count reused from `r3` at `0x80182B4C`
/// are `// unknown:`.
pub unsafe fn fn_801825a0(object: usize, flag: usize, context: usize) -> u32 {
    let packed4c = read_u32(object + 0x4C);
    let packed50 = read_u32(object + 0x50);
    let base = read_u32(object + 0x58) as usize;

    let mut predicate = 1u32;
    let mut pattern = 0u32;
    platform::call1(FN_801C6AC0, 1);

    if packed4c & 0x100 != 0 {
        let record = base
            + (((packed4c >> 24) & 0xF) as usize) * 0x14
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize);
        let first = read_u8(record);
        let second = read_u8(record + 1);
        let _ = platform::call7(FN_801C6AF0, 0, 0, 0, first as u32, 0, 0, 2);
        let _ = platform::call7(FN_801C6AF0, 2, 0, 0, second as u32, 0, 0, 2);
        if first != 1 && second != 1 {
            predicate = 0;
        }
        pattern = u32::from(first == 0 || second == 0);
    } else {
        let _ = platform::call7(FN_801C6AF0, 4, 0, 0, 1, 0, 0, 2);
    }

    let mut bound = u32::from(predicate != 0 && flag != 0);

    if pattern != 0 {
        let mut track = 0xFFFF_FFFFu32;
        if packed4c & 0x80 != 0 {
            let record = base
                + (((packed4c >> 24) & 0xF) as usize) * 0x14
                + ((((packed4c >> 20) & 0xF) << 2) as usize)
                + ((((packed4c >> 28) & 0xF) << 5) as usize)
                + ((((packed4c >> 8) & 1) << 2) as usize);
            track = read_u32(record);
        }
        let mut colour = crate::platform::GuestLocal::new( [0u8; 4]);
        let mut source = crate::platform::GuestLocal::new( track);
        platform::call3(
            W285_FN_8017E8E0,
            colour.as_mut_ptr() as u32,
            (source.as_mut_ptr() as *mut u32) as u32,
            p32(context),
        );
        let packed = crate::platform::GuestLocal::new( u32::from_be_bytes(*colour));
        let _ = platform::call2(W285_FN_801C69E0, 4, (packed.as_ptr() as *const u32) as u32);
        bound = if bound != 0 || packed.wrapping_add(0x10000) != 0xFFFF {
            1
        } else {
            0
        };
    }

    let mut flags = [0u8; 10];
    let mut processed = 0u32;
    let stage_count = (packed50 >> 20) & 0xF;
    let _ = platform::call1(FN_801C4A20, stage_count);
    if stage_count != 0 {
        let textures = base
            + (((packed4c >> 24) & 0xF) as usize) * 0x14
            + ((((packed4c >> 28) & 0xF) << 5) as usize);
        let mut index = 0usize;
        while index < stage_count as usize {
            let entry = textures + index * 4;
            let first = read_u8(entry);
            let second = read_u8(entry + 1);
            let third = read_u8(entry + 2);
            if first == 1 && third != 60 {
                let scaled = (0xAAABu64
                    .wrapping_mul(third.wrapping_sub(30) as u32 as u64)
                    >> 32) as u32
                    >> 1;
                if (scaled as usize) < flags.len() {
                    flags[scaled as usize] = 1;
                }
                processed = 1;
            }
            let _ = call6(
                FN_801C47D0,
                index as u32,
                first as u32,
                second as u32,
                third as u32,
                0,
                125,
            );
            index += 1;
        }
    }

    if processed != 0 {
        let srt = base + ((((packed4c >> 28) & 0xF) << 5) as usize);
        let mut matrix = crate::platform::GuestLocal::new( [0u32; 0x60 / 4]);
        let mut index = 0usize;
        while index < ((packed50 >> 20) & 0xF) as usize {
            if flags[index] != 0 {
                let record = srt + index * 0x14;
                fn_80180440(matrix.as_mut_ptr() as usize, record);
                let _ = platform::call3(
                    FN_801C9A80,
                    matrix.as_mut_ptr() as u32,
                    index.wrapping_mul(3).wrapping_add(30) as u32,
                    1,
                );
            }
            index += 1;
        }
    }

    if (packed50 >> 28) & 0xF != 0 {
        let mut index = 0usize;
        let mut loaded = 0u32;
        let count = ((packed50 >> 28) & 0xF) as usize;
        while index < count {
            let mut record = crate::platform::GuestLocal::new( [0u32; 8]);
            let source = base + index * 0x20;
            let mut word = 0usize;
            while word < 8 {
                record[word] = read_u32(source + word * 4);
                word += 1;
            }
            let record_ptr = record.as_mut_ptr() as u32;
            let kind = platform::call1(W285_FN_801C7220, record_ptr);
            if kind.wrapping_sub(8) <= 1 {
                let texture = platform::call1(W285_FN_801C7170, record_ptr) as usize;
                if texture != 0 {
                    let mut object_word = crate::platform::GuestLocal::new( [0u32; 4]);
                    platform::call4(
                        W285_FN_801C7520,
                        object_word.as_mut_ptr() as u32,
                        read_u32(texture + 8),
                        read_u32(texture + 4),
                        read_u16(texture) as u32,
                    );
                    let _ = platform::call2(
                        W285_FN_801C7550,
                        object_word.as_mut_ptr() as u32,
                        loaded,
                    );
                    let _ = platform::call2(W285_FN_801C7120, record_ptr, loaded);
                    loaded += 1;
                }
            }
            let _ = platform::call2(W285_FN_801C74C0, record_ptr, index as u32);
            index += 1;
        }
    }

    let colours: [usize; 3] = [0x24, 0x2C, 0x34];
    for (position, offset) in colours.iter().enumerate() {
        let _ = platform::call2(W285_FN_801C8490, position as u32 + 1, p32(object + offset));
    }
    let channel_words: [usize; 4] = [0x3C, 0x40, 0x44, 0x48];
    for (position, offset) in channel_words.iter().enumerate() {
        let _ = platform::call2(W285_FN_801C8500, position as u32, p32(object + offset));
    }

    if packed4c & 0x2000 != 0 {
        let record = base
            + (((packed4c >> 24) & 0xF) as usize) * 0x14
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize)
            + (((((packed4c >> 5) & 1) + ((packed4c >> 6) & 1)) << 2) as usize);
        let mut index = 0usize;
        while index < 4 {
            let value = read_u8(record + index);
            let _ = call5(
                FN_801C8640,
                index as u32,
                (value & 3) as u32,
                ((value >> 1) & 3) as u32,
                ((value >> 2) & 3) as u32,
                ((value >> 3) & 3) as u32,
            );
            index += 1;
        }
    } else {
        let _ = call5(FN_801C8640, 0, 0, 1, 2, 3);
        let _ = call5(FN_801C8640, 1, 0, 0, 0, 3);
        let _ = call5(FN_801C8640, 2, 1, 1, 1, 3);
        let _ = call5(FN_801C8640, 3, 2, 2, 2, 3);
    }

    let mut stages = [0u8; 3];
    let mut indirect = 0u32;
    let long_count = (packed50 >> 9) & 0x1F;
    if long_count != 0 {
        let _ = platform::call1(FN_801C88F0, long_count);
        let record = base
            + (((packed4c >> 18) & 3) + ((packed4c >> 24) & 0xF)) as usize * 0x14
            + ((((packed4c >> 8) & 1) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize)
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + (((((packed4c >> 7) & 1)
                + ((packed4c >> 14) & 1)
                + ((packed4c >> 4) & 1)
                + ((packed4c >> 5) & 1)
                + ((packed4c >> 15) & 7)) << 2) as usize);
        let mut index = 0usize;
        while index < long_count as usize {
            let entry = record + index * 16;
            let b0 = read_u8(entry);
            let b1 = read_u8(entry + 1);
            let b2 = read_u8(entry + 2);
            let b3 = read_u8(entry + 3);
            let _ = call4(
                FN_801C8790,
                index as u32,
                b0 as u32,
                (b2 as u32) | ((b3 as u32 & 1) << 8),
                b1 as u32,
            );
            let _ = platform::call3(
                FN_801C8600,
                index as u32,
                ((b3 >> 1) & 3) as u32,
                ((b3 >> 3) & 3) as u32,
            );
            let b4 = read_u8(entry + 4);
            let b5 = read_u8(entry + 5);
            let _ = call5(
                FN_801C82F0,
                index as u32,
                (b4 & 0xF) as u32,
                ((b4 >> 4) & 0xF) as u32,
                (b5 & 0xF) as u32,
                ((b5 >> 4) & 0xF) as u32,
            );
            let b6 = read_u8(entry + 6);
            let b7 = read_u8(entry + 7);
            let _ = platform::call6(
                FN_801C8370,
                index as u32,
                (b6 & 0xF) as u32,
                ((b6 >> 4) & 3) as u32,
                ((b6 >> 6) & 3) as u32,
                (b7 & 1) as u32,
                ((b7 >> 1) & 3) as u32,
            );
            let _ = platform::call2(W285_FN_801C8560, index as u32, ((b7 >> 3) & 0x1F) as u32);
            let b8 = read_u8(entry + 8);
            let b9 = read_u8(entry + 9);
            let _ = call5(
                FN_801C8330,
                index as u32,
                (b8 & 0xF) as u32,
                ((b8 >> 4) & 0xF) as u32,
                (b9 & 0xF) as u32,
                ((b9 >> 4) & 0xF) as u32,
            );
            let b10 = read_u8(entry + 10);
            let b11 = read_u8(entry + 11);
            let _ = platform::call6(
                FN_801C83D0,
                index as u32,
                (b10 & 0xF) as u32,
                ((b10 >> 4) & 3) as u32,
                ((b10 >> 6) & 3) as u32,
                (b11 & 1) as u32,
                ((b11 >> 1) & 3) as u32,
            );
            let _ = platform::call2(W285_FN_801C85B0, index as u32, ((b11 >> 3) & 0x1F) as u32);
            let b12 = read_u8(entry + 12);
            let b13 = read_u8(entry + 13);
            let b14 = read_u8(entry + 14);
            let b15 = read_u8(entry + 15);
            let scale = ((b13 >> 3) & 0xF) as u32;
            let _ = w285_call10(
                W285_FN_801C7D90,
                index as u32,
                b12 as u32,
                (b13 & 7) as u32,
                scale,
                (b14 & 7) as u32,
                (b15 & 3) as u32,
                ((b14 >> 3) & 7) as u32,
                ((b15 >> 2) & 1) as u32,
                ((b15 >> 3) & 0x1F) as u32,
                ((b15 >> 4) & 3) as u32,
            );
            if (1..=3).contains(&scale) {
                stages[scale as usize - 1] = 1;
                indirect = 1;
            }
            index += 1;
        }
    } else {
        let material_count = (packed50 >> 28) & 0xF;
        let mut stage = 0u32;
        let mut finish = false;
        if material_count == 0 {
            let _ = call4(FN_801C8790, 0, 0, 0xFF, 0xFF);
            let _ = call5(FN_801C82F0, 0, 0xF, 4, 10, 0xF);
            let _ = call5(FN_801C8330, 0, 7, 2, 5, 7);
            bound = 1;
            stage = 1;
            finish = true;
        } else if material_count == 1 {
            let _ = call4(FN_801C8790, 0, 0, 0, 0xFF);
            let _ = call5(FN_801C82F0, 0, 2, 4, 8, 0xF);
            let _ = call5(FN_801C8330, 0, 1, 2, 4, 7);
            stage = 1;
        } else if material_count == 2 {
            let _ = call4(FN_801C8790, 0, 0, 0, 0xFF);
            let _ = call5(FN_801C82F0, 0, 0xF, 0xF, 0xF, 8);
            let _ = call5(FN_801C8330, 0, 7, 7, 7, 4);
            let _ = call4(FN_801C8790, 1, 1, 1, 0xFF);
            let _ = call5(FN_801C82F0, 1, 8, 0, 14, 0xF);
            let _ = call5(FN_801C8330, 1, 4, 0, 6, 7);
            let _ = platform::call2(W285_FN_801C8560, 1, read_u32(W285_TEV_COLOUR));
            let _ = platform::call2(W285_FN_801C85B0, 1, read_u32(W285_TEV_ALPHA));
            stage = 2;
        } else {
            let mut index = 0u32;
            while index < material_count {
                let stage_index = stage & 0xFF;
                let _ = call4(FN_801C8790, stage_index, index, index, 0xFF);
                let first = if index == 0 { 0xF } else { 0 };
                let second = if index == 0 { 7 } else { 0 };
                let _ = call5(FN_801C82F0, stage_index, 0xF, 8, 14, 0xF);
                let _ = platform::call6(
                    FN_801C8330,
                    stage_index,
                    7,
                    4,
                    6,
                    7,
                    second,
                );
                let _ = platform::call2(
                    W285_FN_801C8560,
                    stage_index,
                    read_u32(W285_TEV_COLOUR + index as usize * 4),
                );
                let _ = platform::call2(
                    W285_FN_801C85B0,
                    stage_index,
                    read_u32(W285_TEV_ALPHA + index as usize * 4),
                );
                let _ = first;
                stage += 1;
                index += 1;
            }
            finish = false;
        }
        if !finish {
            let default_colours = (0..4).all(|i| {
                read_u16(object + 0x24 + i * 2) == read_u16(U16_802F5E50 + i * 2)
            });
            let alpha_full = (0..4).all(|i| read_u16(object + 0x2C + i * 2) == 255);
            let skip = default_colours && alpha_full;
            if !skip {
                let stage_index = stage & 0xFF;
                let _ = call4(FN_801C8790, stage_index, 0xFF, 0xFF, 0xFF);
                let _ = call5(FN_801C82F0, stage_index, 2, 4, 0, 0xF);
                let _ = call5(FN_801C8330, stage_index, 1, 2, 0, 7);
                stage += 1;
            }
            if bound != 0 {
                let stage_index = stage & 0xFF;
                let _ = call4(FN_801C8790, stage_index, 0xFF, 0xFF, 4);
                let _ = call5(FN_801C82F0, stage_index, 0xF, 0, 10, 0xF);
                let _ = call5(FN_801C8330, stage_index, 7, 0, 5, 7);
                stage += 1;
            }
        }
        let final_stage = stage & 0xFF;
        let mut index = 0u32;
        while index < final_stage {
            let _ = platform::call6(FN_801C8370, index, 0, 0, 0, 1, 0);
            let _ = platform::call6(FN_801C83D0, index, 0, 0, 0, 1, 0);
            let _ = platform::call1(FN_801C8130, index);
            let _ = platform::call3(FN_801C8600, index, 0, 0);
            index += 1;
        }
        let _ = platform::call1(FN_801C88F0, final_stage);
        return u32::from(bound != 0 && predicate != 0);
    }

    if indirect != 0 {
        let sin_scale = F32_802F5E64;
        let record = base
            + (((packed4c >> 24) & 0xF) as usize) * 0x14
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize)
            + ((((packed4c >> 1) & 1) << 2) as usize)
            + ((((packed4c >> 3) & 1) << 2) as usize)
            + (((((packed4c >> 2) & 1)
                + ((packed4c >> 9) & 1)
                + ((packed4c >> 8) & 7)
                + ((packed4c >> 0) & 1)) << 2) as usize);
        let mut index = 0usize;
        while index < ((packed50 >> 16) & 3) as usize {
            if stages[index] != 0 {
                let angle = read_f32(record + index * 0x14 + 8);
                let cosine = call_f64_f64(FN_80188630, (sin_scale * angle) as f64) as f32;
                let sine = call_f64_f64(FN_801885B0, (sin_scale * angle) as f64) as f32;
                let mut matrix = crate::platform::GuestLocal::new( [0f32; 6]);
                matrix[0] = cosine * read_f32(record + index * 0x14 + 0x0C);
                matrix[1] = -sine * read_f32(record + index * 0x14 + 0x10);
                matrix[2] = read_f32(record + index * 0x14);
                matrix[3] = sine * read_f32(record + index * 0x14 + 0x0C);
                matrix[4] = cosine * read_f32(record + index * 0x14 + 0x10);
                matrix[5] = read_f32(record + index * 0x14 + 4);
                fn_80180560(index + 1, matrix.as_mut_ptr() as usize);
            }
            index += 1;
        }
    }

    let small_count = (packed50 >> 12) & 7;
    let _ = platform::call1(FN_801C8110, small_count);
    if small_count != 0 {
        let record = base
            + (((packed4c >> 18) & 3) + ((packed4c >> 24) & 0xF)) as usize * 0x14
            + ((((packed4c >> 8) & 1) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize)
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + (((((packed4c >> 7) & 1)
                + ((packed4c >> 14) & 1)
                + ((packed4c >> 4) & 1)
                + ((packed4c >> 5) & 1)
                + ((packed4c >> 15) & 7)) << 2) as usize);
        let mut index = 0usize;
        while index < small_count as usize {
            let entry = record + index * 4;
            let _ = platform::call2(
                W285_FN_801C8040,
                index as u32,
                ((read_u8(entry) as u32) << 8) | read_u8(entry + 1) as u32,
            );
            let _ = platform::call2(
                W285_FN_801C7F40,
                index as u32,
                ((read_u8(entry + 2) as u32) << 8) | read_u8(entry + 3) as u32,
            );
            index += 1;
        }
    }

    if packed4c & 0x20 != 0 {
        let record = base
            + (((packed4c >> 24) & 0xF) as usize) * 0x14
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize)
            + (((((packed4c >> 5) & 1)
                + ((packed4c >> 6) & 1)
                + ((packed4c >> 7) & 1)
                + ((packed4c >> 14) & 1)
                + ((packed4c >> 8) & 7)
                + ((packed4c >> 0) & 1)) << 2) as usize);
        let value = read_u8(record);
        let _ = call5(
            W285_FN_801C86C0,
            (value & 0xF) as u32,
            read_u8(record + 2) as u32,
            read_u8(record + 1) as u32,
            ((value >> 4) & 0xF) as u32,
            read_u8(record + 3) as u32,
        );
    } else {
        let _ = call5(W285_FN_801C86C0, 7, 0, 0, 7, 0);
    }

    if packed4c & 0x10 != 0 {
        let record = base
            + (((packed4c >> 24) & 0xF) as usize) * 0x14
            + ((((packed4c >> 20) & 0xF) << 2) as usize)
            + ((((packed4c >> 28) & 0xF) << 5) as usize)
            + (((packed4c >> 8) & 0xF) << 2) as usize
            + ((((packed4c >> 4) & 1) << 2) as usize)
            + (((((packed4c >> 9) & 1)
                + ((packed4c >> 2) & 1)
                + ((packed4c >> 3) & 1)
                + ((packed4c >> 1) & 1)) << 2) as usize);
        let _ = call4(
            FN_801C8DC0,
            read_u8(record) as u32,
            read_u8(record + 1) as u32,
            read_u8(record + 2) as u32,
            read_u8(record + 3) as u32,
        );
    } else {
        let _ = call4(FN_801C8DC0, 1, 4, 5, 0xF);
    }

    u32::from(bound != 0 && predicate != 0)
}

/// Removes list nodes that match a key (or every node when the key is zero).
///
/// The list is the embedded sentinel at `+0x1C` (head) with the count/header at
/// `+0x18`. Each removed node's `+0x08` key word and `+0x0C` halfword are
/// cleared.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x801833E0`.
/// - Extent: `0x801833E0..0x80183460` (128 bytes, 32 insns).
/// - Table: slot 3 (`0x8032B2DC`).
/// - Evidence: `work/wave285/b_dis.txt`.
///
/// # Arguments
/// - `object`: animation object owning the list.
/// - `key`: node key, or `0` to remove every node.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// The list must be a live circular list of writable nodes.
///
/// # Confidence
/// high - the sentinel walk, the key test and the two clears are literal.
pub unsafe fn fn_801833e0(object: usize, key: u32) {
    let sentinel = object + 0x1C;
    let mut node = read_u32(sentinel) as usize;
    while node != sentinel {
        let current = node;
        node = read_u32(current) as usize;
        if key == 0 || read_u32(current + 8) == key {
            let mut word = crate::platform::GuestLocal::new( current as u32);
            fn_8018a540((object + 0x18) as *mut u32, word.as_ptr() as *const u32);
            write_u32(current + 8, 0);
            write_u16(current + 0x0C, 0);
        }
    }
}

/// Notifies every live list node through its `+0x08` object.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80183480`.
/// - Extent: `0x80183480..0x801834F8` (120 bytes, 30 insns).
/// - Table: slot 5 (`0x8032B2E4`).
/// - Evidence: `work/wave285/b_dis.txt`.
///
/// # Arguments
/// - `object`: animation object owning the list at `+0x1C`.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// Every node's `+0x08` object must expose a callable `+0x1C` slot.
///
/// # Confidence
/// high - the node fields, the `+0x0E` guard and the virtual call are literal.
pub unsafe fn fn_80183480(object: usize) {
    let sentinel = object + 0x1C;
    let mut node = read_u32(sentinel) as usize;
    while node != sentinel {
        if read_u8(node + 0x0E) == 0 {
            let target = read_u32(node + 8) as usize;
            let key = read_u16(node + 0x0C) as u32;
            platform::call3(
                vtable_method(target, 0, 0x1C),
                p32(target),
                key,
                p32(object),
            );
        }
        node = read_u32(node) as usize;
    }
}

/// Finds the list node with a given key.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80183540`.
/// - Extent: `0x80183540..0x80183548` (8 bytes, 2 insns).
/// - Table: slot 6 (`0x8032B2E8`).
/// - Evidence: `work/wave285/b_dis.txt`.
///
/// # Arguments
/// - `object`: animation object owning the `+0x18` list header.
/// - `key`: lookup key.
///
/// # Returns
/// The node pointer, or `0`.
///
/// # Safety
/// The list must be a live circular list.
///
/// # Confidence
/// high - `addi r3,r3,0x18` then `b 0x8017DFA0`.
pub unsafe fn fn_80183540(object: usize, key: u32) -> u32 {
    platform::call2(FN_8017DFA0_FIND, p32(object + 0x18), key)
}

/// Marks the result of virtual `+0x20` as dirty.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80183550`.
/// - Extent: `0x80183550..0x8018359C` (76 bytes, 19 insns).
/// - Table: slot 7 (`0x8032B2EC`).
/// - Evidence: `work/wave285/b_dis.txt`.
///
/// # Arguments
/// - `object`: animation object.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `object` must expose a callable `+0x20` slot.
///
/// # Confidence
/// medium - the non-null test and the `+0x0E` store are literal, but the
/// original computes the stored byte from `r0`, which the virtual call does
/// not define; on the non-null path that value is modelled as `0`.
pub unsafe fn fn_80183550(object: usize) {
    let result = platform::call1(vtable_method(object, 0, 0x20), p32(object)) as usize;
    if result != 0 {
        write_u8(result + 0x0E, 0);
    }
}

/// Forwards `(this, one)` to virtual `+0x14` of another object.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x801833C0`.
/// - Extent: `0x801833C0..0x801833DC` (28 bytes, 7 insns), ending in `bcctr`.
/// - Table: slot 2 (`0x8032B2D8`).
/// - Evidence: `work/wave285/b_dis.txt`.
///
/// # Arguments
/// - `object`: receiver whose `+0x14` slot is called.
/// - `other`: argument forwarded as the first call parameter.
///
/// # Returns
/// The forwarded result.
///
/// # Safety
/// `object` must expose a callable `+0x14` slot.
///
/// # Confidence
/// high - one vtable load, one `bcctr` and the argument copy are literal.
pub unsafe fn fn_801833c0(object: usize, other: usize) -> u32 {
    platform::call2(vtable_method(object, 0, 0x14), p32(object), p32(other))
}



// ===========================================================================
// Wave 289 lane A (`_A289`): the `nw4hbm::ut::ResFont` vtable
// (`0x8032C718`, 20 slots) -- nineteen untabled bodies
// ===========================================================================
//
// **Span.** The vtable header is `0x8032C710 = {0x8032C790, 0}` - the usual
// eight-byte NW4R header, except that word 0 is the class RTTI record rather
// than a zero pair. The method array is `0x8032C718..0x8032C768` (20 words,
// 80 bytes): `+0x08..+0x54` from the header, `+0x00..+0x4C` from the array
// base the brief quotes. Slot `n` lives at header `+0x08+4n`; slot 14
// (`+0x40` from the header, `+0x38` from the array base, `0x8032C750`) is the
// already-translated [`fn_8018abe0`]. The array ends flush against the
// NUL-terminated string `"nw4hbm::ut::ResFont"` at `0x8032C768`.
//
// **Install site.** One code site and no literal data word
// (`work/wave289/a_refs.mjs`): `lis r4,0x8033` at `0x8018A748` +
// `addi r4,r4,-14576` at `0x8018A750` (`0xC710`, so `0x8032C710`) stored by
// `stw r4,0(r31)` at `0x8018A754` inside [`fn_8018a730`]. The neighbouring
// table `0x8032C800` is installed by the same constructor's base call
// [`fn_8018aa30`] (`lis r4,0x8033` at `0x8018AA40` + `addi r4,r4,-14336` at
// `0x8018AA4C`), and the base-only table `0x8032C858` at `0x8018AA38`
// (`addi r4,r4,-14248`).
//
// **Identity (RTTI).** `0x8032C710[0] = 0x8032C790 = {0x8032C768, 0x8032C77C}`:
// the record names `"nw4hbm::ut::ResFont"` and links to the one-entry parent
// node `0x8032C77C = {0x8032C7E8, 0}`, which resolves to the `Font` record
// `0x8032C7E8 = {0x8032C7D0, 0}` (`"nw4hbm::ut::Font"`). The neighbouring
// header `0x8032C800[0] = 0x8032C7C8 = {0x8032C798, 0x8032C7B8}` names
// `"nw4hbm::ut::detail::ResFontBase"` and links to
// `0x8032C7B8 = {0x8032C7E8, 0}`, i.e. also to `Font`; `0x8032C858[0]` is
// `0x8032C7E8` itself. Correction for the wave-5 comments: the table at
// `0x8032C800` is the `ResFontBase` table (its header record names
// `ResFontBase`), and `Font`'s own table is `0x8032C858` (one slot,
// `0x80185ED0`), not `0x8032C800`.
//
// The 20 slots split into 19 new bodies plus the covered one:
//
// | slot | header | array | target | bytes | observed role |
// |---:|---|---|---|---:|---|
// | 0 | `+0x08` | `+0x00` | `0x8018A770` | 88 | deleting destructor |
// | 1 | `+0x0C` | `+0x04` | `0x8018AAD0` | 12 | `FINF+0x15` byte |
// | 2 | `+0x10` | `+0x08` | `0x8018AAE0` | 12 | `FINF+0x14` byte (vertical size) |
// | 3 | `+0x14` | `+0x0C` | `0x8018AAF0` | 12 | `FINF+0x16` byte (second metric) |
// | 4 | `+0x18` | `+0x10` | `0x8018AB00` | 20 | byte0 - byte2 of that triple |
// | 5 | `+0x1C` | `+0x14` | `0x8018AB20` | 20 | `(s8)TGLP+2` |
// | 6 | `+0x20` | `+0x18` | `0x8018AB40` | 16 | `TGLP+1` byte |
// | 7 | `+0x24` | `+0x1C` | `0x8018AB50` | 16 | `TGLP+0` byte |
// | 8 | `+0x28` | `+0x20` | `0x8018AB60` | 16 | `TGLP+3` byte |
// | 9 | `+0x2C` | `+0x24` | `0x8018AB70` | 8 | constant `2` |
// | 10 | `+0x30` | `+0x28` | `0x8018AB80` | 16 | `TGLP+0x0A` halfword |
// | 11 | `+0x34` | `+0x2C` | `0x8018AB90` | 16 | `(s8)FINF+1` line feed |
// | 12 | `+0x38` | `+0x30` | `0x8018ABA0` | 24 | packed default `CharWidth` |
// | 13 | `+0x3C` | `+0x34` | `0x8018ABC0` | 32 | store three bytes at `FINF+4` |
// | 14 | `+0x40` | `+0x38` | `0x8018ABE0` | 156 | covered: [`fn_8018abe0`] |
// | 15 | `+0x44` | `+0x3C` | `0x8018AC80` | 12 | store one byte at `FINF+1` |
// | 16 | `+0x48` | `+0x40` | `0x8018AC90` | 68 | `+0x4C` relay, returns one byte |
// | 17 | `+0x4C` | `+0x44` | `0x8018ACE0` | 244 | `CWDH` lookup, packed `CharWidth` |
// | 18 | `+0x50` | `+0x48` | `0x8018ADE0` | 164 | glyph resolve + `0x8018AF70` |
// | 19 | `+0x54` | `+0x4C` | `0x8018AE90` | 12 | `FINF+7` byte |
//
// **Shared slots.** `0x8018AAD0..0x8018AE90` are the same 19 words as the
// `ResFontBase` table's slots 1..19 (`0x8032C80C..0x8032C854`); only slot 0 is
// overridden (`0x8018A770` here, `0x8018AA80` in the base table). So every
// body below except the destructor is also reachable through `0x8032C800`.
//
// **Containment.** Nothing crosses a slot boundary:
//
// ```text
// 0x8032C6E0..0x8032C70C  four 12-byte encoding tables {0, -1, handler}
// 0x8032C710  0x8032C790 0     this header
// 0x8032C718  20 method words  ends 0x8032C768
// 0x8032C768  "nw4hbm::ut::ResFont"    ends 0x8032C77B, word-aligned 0x8032C77C
// 0x8032C77C  {0x8032C7E8, 0}  parent node of the ResFont record
// 0x8032C784  {0x8032C7C8, 0}  unreferenced second parent node
// 0x8032C790  {0x8032C768, 0x8032C77C}  ResFont RTTI record
// 0x8032C798  "nw4hbm::ut::detail::ResFontBase"
// 0x8032C7B8  {0x8032C7E8, 0}  parent node of the ResFontBase record
// 0x8032C7C8  {0x8032C798, 0x8032C7B8}  ResFontBase RTTI record
// 0x8032C7D0  "nw4hbm::ut::Font"
// 0x8032C7E8  {0x8032C7D0, 0}  Font RTTI record
// 0x8032C7F0  {0, -1, 0x80188980}  encoding table copied by fn_8018aa30
// 0x8032C800  {0x8032C7C8, 0}  ResFontBase header, slots 0x8032C808..0x8032C854
// 0x8032C858  {0x8032C7E8, 0}  Font header, slot 0x8032C860 = 0x80185ED0
// ```
//
// **Census before this pass** (`work/wave289/a_census.txt`): 20 slots, 20
// distinct, 1 covered (slot 14, `functions.tsv` row + [`fn_8018abe0`]),
// 19 uncovered, 4 of the uncovered carry a real prologue (slots 0, 16, 17,
// 18; the other 15 are leaves), 0 interior labels, 0 targets outside the main
// text. This pass closes all nineteen (808 bytes).
//
// **Padding.** Slot 0 ends `0x8018A7C8` with two zero words before
// `0x8018A7D0`; slots 1/2/3 end one zero word short of the next leaf; slots
// 4/5, 9, 12 and 16/17/18 end on two, two, two and three zero words; slots
// 6/7/8/10/11/13 end flush; slot 19 ends `0x8018AE9C` one zero word short of
// [`fn_8018aea0`].

/// Deleting destructor of `nw4hbm::ut::ResFont` (vtable slot 0).
///
/// Runs the base release [`fn_8018aa80`] with a zero flag, then frees the
/// object through `0x802B9280` when the caller's flag is positive, and returns
/// the object pointer unchanged.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018A770`.
/// - Extent: `0x8018A770..0x8018A7C8` (88 bytes, 22 insns).
/// - Table: slot 0 (`0x8032C718`, `+0x08` from the header).
/// - Evidence: `work/wave289/a_dis.txt` (`0x8018A770..`), `a_slots.mjs`.
///
/// # Arguments
/// - `font`: object to release.
/// - `delete_flag`: `> 0` frees the object after the base release.
///
/// # Returns
/// `font`, as the original leaves `r3 = r30`.
///
/// # Safety
/// A non-null `font` must be a live `ResFont` when `delete_flag > 0`.
///
/// # Confidence
/// high - the null guard, the `bl 0x8018AA80` (`r4 = 0`) and the
/// `bc 4,1` flag test are literal; `0x802B9280` is the shared deallocator
/// (`li r4,0` + `b 0x802B8F70`).
pub unsafe fn fn_8018a770(font: usize, delete_flag: i32) -> usize {
    if font != 0 {
        fn_8018aa80(font as *mut u8, 0);
        if delete_flag > 0 {
            platform::call1(FN_802B9280, p32(font));
        }
    }
    font
}

/// Returns `FINF + 0x15`, the middle byte of the height triple (slot 1).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AAD0`.
/// - Extent: `0x8018AAD0..0x8018AADC` (12 bytes, 3 insns).
/// - Table: slot 1 (`0x8032C71C`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - `lwz r3,0x14(r3)` + `lbz r3,0x15(r3)` + `blr`.
pub unsafe fn fn_8018aad0(font: usize) -> u8 {
    read_u8(read_u32(font + 0x14) as usize + 0x15)
}

/// Returns `FINF + 0x14`, the first byte of the height triple (slot 2).
///
/// `0x80189BC0` multiplies this value by the text scale to obtain a vertical
/// size, so the byte behaves as a cell height (`// inferred:`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AAE0`.
/// - Extent: `0x8018AAE0..0x8018AAEC` (12 bytes, 3 insns).
/// - Table: slot 2 (`0x8032C720`).
/// - Evidence: `work/wave289/a_dis3.txt`; call site `src/ui/lyt.rs`
///   [`fn_80189bc0`].
///
/// # Confidence
/// high - `lwz r3,0x14(r3)` + `lbz r3,0x14(r3)` + `blr`.
pub unsafe fn fn_8018aae0(font: usize) -> u8 {
    read_u8(read_u32(font + 0x14) as usize + 0x14)
}

/// Returns `FINF + 0x16`, the third byte of the height triple (slot 3).
///
/// `0x80189C30` scales this byte the same way slot 2 is scaled, so it is the
/// second vertical metric of the record (`// inferred:`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AAF0`.
/// - Extent: `0x8018AAF0..0x8018AAFC` (12 bytes, 3 insns).
/// - Table: slot 3 (`0x8032C724`).
/// - Evidence: `work/wave289/a_dis3.txt`; call site `src/ui/lyt.rs`
///   [`fn_80189c30`].
///
/// # Confidence
/// high - `lwz r3,0x14(r3)` + `lbz r3,0x16(r3)` + `blr`.
pub unsafe fn fn_8018aaf0(font: usize) -> u8 {
    read_u8(read_u32(font + 0x14) as usize + 0x16)
}

/// Returns the difference of the two outer height bytes (slot 4).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB00`.
/// - Extent: `0x8018AB00..0x8018AB14` (20 bytes, 5 insns).
/// - Table: slot 4 (`0x8032C728`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Returns
/// `FINF[+0x14] - FINF[+0x16]`, zero-extended bytes, signed difference.
///
/// # Confidence
/// high - the two `lbz` loads and `subf r3,r3,r0` are literal.
pub unsafe fn fn_8018ab00(font: usize) -> i32 {
    let info = read_u32(font + 0x14) as usize;
    read_u8(info + 0x14) as i32 - read_u8(info + 0x16) as i32
}

/// Returns the signed byte at `TGLP + 2` (slot 5).
///
/// The `TGLP` pointer is `FINF + 0x08`; `0x8018AF70` reads the same section at
/// `+0x0C`/`+0x0E` for the cell grid and at `+0x14` for the sheet data.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB20`.
/// - Extent: `0x8018AB20..0x8018AB34` (20 bytes, 5 insns).
/// - Table: slot 5 (`0x8032C72C`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - two `lwz` dereferences plus `lbz`/`extsb` are literal.
pub unsafe fn fn_8018ab20(font: usize) -> i32 {
    let info = read_u32(font + 0x14) as usize;
    let tglp = read_u32(info + 0x08) as usize;
    read_u8(tglp + 0x02) as u8 as i8 as i32
}

/// Returns the byte at `TGLP + 1` (slot 6).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB40`.
/// - Extent: `0x8018AB40..0x8018AB50` (16 bytes, 4 insns).
/// - Table: slot 6 (`0x8032C730`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - `lbz r3,1(r3)` after the two pointer loads.
pub unsafe fn fn_8018ab40(font: usize) -> u8 {
    let info = read_u32(font + 0x14) as usize;
    read_u8(read_u32(info + 0x08) as usize + 0x01)
}

/// Returns the byte at `TGLP + 0` (slot 7).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB50`.
/// - Extent: `0x8018AB50..0x8018AB60` (16 bytes, 4 insns).
/// - Table: slot 7 (`0x8032C734`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - `lbz r3,0(r3)` after the two pointer loads.
pub unsafe fn fn_8018ab50(font: usize) -> u8 {
    let info = read_u32(font + 0x14) as usize;
    read_u8(read_u32(info + 0x08) as usize)
}

/// Returns the byte at `TGLP + 3` (slot 8).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB60`.
/// - Extent: `0x8018AB60..0x8018AB70` (16 bytes, 4 insns).
/// - Table: slot 8 (`0x8032C738`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - `lbz r3,3(r3)` after the two pointer loads.
pub unsafe fn fn_8018ab60(font: usize) -> u8 {
    let info = read_u32(font + 0x14) as usize;
    read_u8(read_u32(info + 0x08) as usize + 0x03)
}

/// Returns the constant `2` (slot 9).
///
/// No argument is read, so the Rust body takes none. The value is the encoding
/// kind the font reports to [`fn_8018a400`]'s selector (`// inferred:`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB70`.
/// - Extent: `0x8018AB70..0x8018AB78` (8 bytes, 2 insns).
/// - Table: slot 9 (`0x8032C73C`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - `li r3,2` + `blr`.
pub unsafe fn fn_8018ab70() -> i32 {
    2
}

/// Returns the halfword at `TGLP + 0x0A` (slot 10).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB80`.
/// - Extent: `0x8018AB80..0x8018AB90` (16 bytes, 4 insns).
/// - Table: slot 10 (`0x8032C740`).
/// - Evidence: `work/wave289/a_dis3.txt`.
///
/// # Confidence
/// high - `lhz r3,0x0A(r3)` after the two pointer loads.
pub unsafe fn fn_8018ab80(font: usize) -> u16 {
    let info = read_u32(font + 0x14) as usize;
    read_u16(read_u32(info + 0x08) as usize + 0x0A)
}

/// Returns the signed line feed, `FINF + 1` (slot 11).
///
/// `0x8018BCA0`/`0x8018E660` add this value (scaled) to the base line height,
/// which identifies the byte as the `FINF` line feed (`// inferred:`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AB90`.
/// - Extent: `0x8018AB90..0x8018ABA0` (16 bytes, 4 insns).
/// - Table: slot 11 (`0x8032C744`).
/// - Evidence: `work/wave289/a_dis3.txt`; call sites [`fn_8018bca0`],
///   [`fn_8018e660`].
///
/// # Confidence
/// high - `lbz r3,1(r3)` + `extsb`.
pub unsafe fn fn_8018ab90(font: usize) -> i32 {
    let info = read_u32(font + 0x14) as usize;
    read_u8(info + 0x01) as u8 as i8 as i32
}

/// Returns the packed default `CharWidth` of the `FINF` record (slot 12).
///
/// The three bytes at `FINF + 0x04..0x06` are returned left-justified in a
/// 32-bit word (`left << 24 | glyphWidth << 16 | charWidth << 8`), which is the
/// NW4R three-byte-struct return convention.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018ABA0`.
/// - Extent: `0x8018ABA0..0x8018ABB8` (24 bytes, 6 insns).
/// - Table: slot 12 (`0x8032C748`).
/// - Evidence: `work/wave289/a_dis4.txt`; the `rlwinm`/`rlwimi` pair was
///   evaluated with `work/wave289/a_rot.mjs` (`sh=16, mask 0xFFFF0000`, then
///   `sh=8, mask 0x0000FF00`).
///
/// # Returns
/// `FINF[+4..+6]` packed into the top three bytes.
///
/// # Confidence
/// high - the two shifts and the mask are literal; the byte roles come from
/// `0x8018AF70`'s default-record path (`FINF + 0x04`).
pub unsafe fn fn_8018aba0(font: usize) -> u32 {
    let info = read_u32(font + 0x14) as usize;
    ((read_u16(info + 0x04) as u32) << 16) | ((read_u8(info + 0x06) as u32) << 8)
}

/// Copies a three-byte `CharWidth` record into `FINF + 0x04` (slot 13).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018ABC0`.
/// - Extent: `0x8018ABC0..0x8018ABE0` (32 bytes, 8 insns), flush against the
///   covered [`fn_8018abe0`].
/// - Table: slot 13 (`0x8032C74C`).
/// - Evidence: `work/wave289/a_dis4.txt`.
///
/// # Arguments
/// - `font`: `ResFont` whose `+0x14` is the `FINF` payload.
/// - `source`: three readable bytes (left, glyph width, char width).
///
/// # Returns
/// The `FINF` pointer, the value the body leaves in `r3`.
///
/// # Safety
/// `source` must be readable for three bytes; `font` must be a live font.
///
/// # Confidence
/// high - three `lbz`/`stb` pairs at offsets `0..2` -> `4..6`.
pub unsafe fn fn_8018abc0(font: usize, source: usize) -> usize {
    let info = read_u32(font + 0x14) as usize;
    write_u8(info + 0x04, read_u8(source));
    write_u8(info + 0x05, read_u8(source + 0x01));
    write_u8(info + 0x06, read_u8(source + 0x02));
    info
}

/// Overwrites the `FINF` line-feed byte (slot 15).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AC80`.
/// - Extent: `0x8018AC80..0x8018AC8C` (12 bytes, 3 insns).
/// - Table: slot 15 (`0x8032C754`).
/// - Evidence: `work/wave289/a_dis4.txt`.
///
/// # Arguments
/// - `font`: `ResFont` whose `+0x14` is the `FINF` payload.
/// - `value`: byte stored at `FINF + 1`.
///
/// # Returns
/// The `FINF` pointer, the value the body leaves in `r3`.
///
/// # Safety
/// `font` must be a live bindable font.
///
/// # Confidence
/// high - `lwz r3,0x14(r3)` + `stb r4,1(r3)`.
pub unsafe fn fn_8018ac80(font: usize, value: u8) -> usize {
    let info = read_u32(font + 0x14) as usize;
    write_u8(info + 0x01, value);
    info
}

/// Relays to virtual `+0x4C` and returns one byte of the packed result (slot
/// 16).
///
/// The original passes its own `r4` through unchanged (the callee only reads
/// `r3`/`r4`), materialises the three `CharWidth` bytes in its own frame
/// (`sth r0,8(r1)` / `stb r0,0xA(r1)`) and returns the last one sign-extended.
/// The `+0x48` call site treats the value as a horizontal advance
/// (`0x80189CC0`), so the byte is the character width (`// inferred:`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AC90`.
/// - Extent: `0x8018AC90..0x8018ACD4` (68 bytes, 17 insns).
/// - Table: slot 16 (`0x8032C758`).
/// - Evidence: `work/wave289/a_dis4.txt`; call site [`fn_80189cc0`].
///
/// # Arguments
/// - `font`: `ResFont` receiver.
/// - `character`: character code forwarded to slot 17.
///
/// # Returns
/// The `charWidth` byte of the `CharWidth` record, sign-extended.
///
/// # Safety
/// The font's vtable must expose slot `+0x4C` and its `FINF` payload must be
/// loaded.
///
/// # Confidence
/// high - the vtable load, the `+0x4C` dispatch and both `rlwinm` masks are
/// literal; the local record is dead after the return.
pub unsafe fn fn_8018ac90(font: usize, character: u32) -> i32 {
    let packed = platform::call2(
        vtable_method(font, 0, 0x4C),
        p32(font),
        p32(character as usize),
    );
    let mut width = crate::platform::GuestLocal::new( [0u8; 3]);
    write_u16(width.as_mut_ptr() as usize, (packed >> 16) as u16);
    width[2] = (packed >> 8) as u8;
    width[2] as i8 as i32
}

/// Looks a character up in the `CWDH` chain and returns its packed
/// `CharWidth` (slot 17).
///
/// Walks the glyph-range list at `FINF + 0x10` with [`fn_8018aea0`]; when no
/// range matches (or the mapper answers `0xFFFF`) the alternate character
/// index `FINF + 0x02` is used. The index then selects the `CWDH` record
/// (`first at +0x00`, `last at +0x02`, links at `+0x04`, three bytes per
/// entry starting at `+0x08`); an empty chain falls back to `FINF + 0x04`.
/// The record's `u16[+0]` and `u8[+2]` are returned as
/// `u16 << 16 | u8 << 8`, the same packing as slot 12.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018ACE0`.
/// - Extent: `0x8018ACE0..0x8018ADD4` (244 bytes, 61 insns).
/// - Table: slot 17 (`0x8032C75C`).
/// - Evidence: `work/wave289/a_dis4.txt`.
///
/// # Arguments
/// - `font`: `ResFont` receiver.
/// - `code`: character code to look up.
///
/// # Returns
/// The packed `CharWidth` of the selected record.
///
/// # Safety
/// The glyph-range list, the `CWDH` chain and the record array must be
/// readable.
///
/// # Confidence
/// high - both walks, the `0xFFFF` sentinel, the `*3` stride and the
/// `rlwinm`/`rlwimi` packing are literal. Note the `CWDH` link is at `+0x04`,
/// while the existing [`fn_8018af70`] body reads `+0x08` (see the lane
/// report); this body follows the instruction stream.
pub unsafe fn fn_8018ace0(font: usize, code: u32) -> u32 {
    let info = read_u32(font + 0x14) as usize;
    let mut node = read_u32(info + 0x10) as usize;
    let mut glyph = 0xFFFFu32;
    loop {
        if node == 0 {
            break;
        }
        let first = read_u16(node) as u32;
        let last = read_u16(node + 0x02) as u32;
        if first <= code && code <= last {
            glyph = platform::call3(
                FN_8018AEA0_GLYPH_RANGE,
                p32(font),
                p32(node),
                p32(code as usize),
            );
            break;
        }
        node = read_u32(node + 0x08) as usize;
    }

    let index = if (glyph & 0xFFFF) != 0xFFFF {
        glyph & 0xFFFF
    } else {
        read_u16(info + 0x02) as u32
    };

    let mut cwdh = read_u32(info + 0x0C) as usize;
    let record;
    loop {
        if cwdh == 0 {
            record = info + 0x04;
            break;
        }
        let first = read_u16(cwdh) as u32;
        let last = read_u16(cwdh + 0x02) as u32;
        if first <= index && index <= last {
            record = cwdh + (index - first) as usize * 3 + 0x08;
            break;
        }
        cwdh = read_u32(cwdh + 0x04) as usize;
    }

    ((read_u16(record) as u32) << 16) | ((read_u8(record + 0x02) as u32) << 8)
}

/// Resolves a character code to a glyph and fills a 0x14-byte texture record
/// (slot 18).
///
/// Runs the same glyph-range walk as slot 17, substitutes `FINF + 0x02` when
/// the mapper answers `0xFFFF`, and tail-calls [`fn_8018af70`] with
/// `(font, out, glyph)`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018ADE0`.
/// - Extent: `0x8018ADE0..0x8018AE84` (164 bytes, 41 insns).
/// - Table: slot 18 (`0x8032C760`).
/// - Evidence: `work/wave289/a_dis4.txt`; callee body `0x8018AF70` in
///   `work/wave289/a_dis.mjs` output.
///
/// # Arguments
/// - `font`: `ResFont` receiver.
/// - `out`: 0x14-byte glyph/texture record written by [`fn_8018af70`].
/// - `code`: character code to resolve.
///
/// # Returns
/// Nothing; the callee's `r3` is not preserved as a value.
///
/// # Safety
/// `font`'s `FINF` payload and `TGLP` cell dimensions must be valid, and
/// `out` must be writable for 0x14 bytes.
///
/// # Confidence
/// high - the register shuffle (`r5 = r4` at entry, `r4 = r31` at the tail),
/// the `0xFFFF` substitution and the tail call are literal.
pub unsafe fn fn_8018ade0(font: usize, out: usize, code: u32) {
    let info = read_u32(font + 0x14) as usize;
    let mut node = read_u32(info + 0x10) as usize;
    let mut glyph = 0xFFFFu32;
    loop {
        if node == 0 {
            break;
        }
        let first = read_u16(node) as u32;
        let last = read_u16(node + 0x02) as u32;
        if first <= code && code <= last {
            glyph = platform::call3(
                FN_8018AEA0_GLYPH_RANGE,
                p32(font),
                p32(node),
                p32(code as usize),
            );
            break;
        }
        node = read_u32(node + 0x08) as usize;
    }
    let glyph = if (glyph & 0xFFFF) != 0xFFFF {
        glyph & 0xFFFF
    } else {
        read_u16(info + 0x02) as u32
    };
    fn_8018af70(font, out, glyph);
}

/// Returns the byte at `FINF + 7` (slot 19).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x8018AE90`.
/// - Extent: `0x8018AE90..0x8018AE9C` (12 bytes, 3 insns).
/// - Table: slot 19 (`0x8032C764`).
/// - Evidence: `work/wave289/a_dis4.txt`.
///
/// # Confidence
/// high - `lwz r3,0x14(r3)` + `lbz r3,7(r3)` + `blr`.
pub unsafe fn fn_8018ae90(font: usize) -> u8 {
    read_u8(read_u32(font + 0x14) as usize + 0x07)
}

// ===========================================================================
// Wave 292 lane E (`_E292`) - the second three-axis rotation apply helper
// ===========================================================================
//
// Re-picked by `work/wave292/e_pick.mjs` from `work/wave266/e_candidates.tsv`
// after the 259 candidates covered by waves 266..291 and the three deferred
// addresses were removed. The body is 120 bytes, starts with `9421xxxx`
// (`stwu r1,-N(r1)`), reproduces its extent with `bad = 0`, ends on exactly one
// `blr`, does not overlap an extent and is not a pointer-run interior.

/// Applies the three `s16` rotation components in natural argument order.
///
/// Copies one halfword from each incoming pointer into the stack frame, then
/// concatenates the matrices in the order third, first, second - the same
/// per-axis order as [`fn_8016ef50`], but with the second and third operands
/// mapped to `0x8016EDD0` and `0x8016EEF0` instead of the reverse.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x8016EFD0`.
/// - Extent: `0x8016EFD0..0x8016F048` (120 bytes, 30 words), ending at the
///   `blr` at `0x8016F044`; `0x8016EFC8..0x8016EFD0` is the zero padding
///   after [`fn_8016ef50`]'s `blr`.
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x8066C5CC`.
/// - Table: none.
/// - Observed: `9421FFD0 stwu r1,-0x30(r1)`; `mflr r0`; `stw r0,0x34(r1)`;
///   `stw r31,0x2C(r1)`; `mr r31,r5`; `stw r30,0x28(r1)`; `mr r30,r4`;
///   `addi r4,r1,0x10`; `stw r29,0x24(r1)`; `mr r29,r3`;
///   `lhz r0,0(r6)`; `sth r0,0x10(r1)`; `bl 0x8016EEF0`;
///   `lhz r0,0(r31)`; `mr r3,r29`; `sth r0,0xC(r1)`; `addi r4,r1,0xC`;
///   `bl 0x8016EDD0`; `lhz r0,0(r30)`; `mr r3,r29`; `sth r0,8(r1)`;
///   `addi r4,r1,8`; `bl 0x8016ECB0`; epilogue; `4E800020 blr`.
/// - Polarity: no branch.
/// - Decoder note: the three literals fold to the module constants
///   [`W3_ROT_APPLY_C`] (`0x8016EEF0`, third argument),
///   [`W3_ROT_APPLY_B`] (`0x8016EDD0`, second argument) and
///   [`W3_ROT_APPLY_A`] (`0x8016ECB0`, first argument). The registers are
///   saved only to keep `r4/r5/r6` live across the three calls.
///
/// # Arguments
/// * `transform` - 4x4 transform to modify.
/// * `first`, `second`, `third` - pointers to three `s16` angles.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Concatenates up to three rotation matrices into `transform`.
///
/// # Safety
/// All four pointers must be valid; zero angles are skipped by the callees.
///
/// # Confidence
/// high - the three call targets, the stack copies and the register order are
/// exact; the axis semantics remain the ones [`fn_8016ef50`] documents.
pub unsafe fn fn_8016efd0(
    transform: usize,
    first: usize,
    second: usize,
    third: usize,
) {
    let mut c = crate::platform::GuestLocal::new( platform::read_u16(third));
    platform::call2(W3_ROT_APPLY_C, p32(transform), (c.as_ptr() as *const u16) as u32);

    let mut b = crate::platform::GuestLocal::new( platform::read_u16(second));
    platform::call2(W3_ROT_APPLY_B, p32(transform), (b.as_ptr() as *const u16) as u32);

    let mut a = crate::platform::GuestLocal::new( platform::read_u16(first));
    platform::call2(W3_ROT_APPLY_A, p32(transform), (a.as_ptr() as *const u16) as u32);
}
