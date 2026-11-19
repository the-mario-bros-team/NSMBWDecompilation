//! Staging lane `0x80080000..0x800C0000` - **fully promoted, no code remains**.
//!
//! This file used to hold the course-resource translations for `main.dol`
//! entries in `[0x80080000, 0x800C0000)`. Every one of them had a proven role,
//! so the whole lane moved into semantic modules and nothing is left staged
//! here. The module is kept (it is declared by `src/subsystems/mod.rs`) so the
//! lane id stays resolvable and the history is traceable.
//!
//! # Where the functions went
//!
//! Stage runtime - the BG manager, BG scene, tile buffers and the course-file
//! parser - is in [`crate::game::stage`]:
//!
//! | address | name |
//! |---|---|
//! | `0x800813F0` | [`crate::game::stage::unit_name`] |
//! | `0x80081410` | [`crate::game::stage::copy_unit_names`] |
//! | `0x800814D0` | [`crate::game::stage::load_bg_units`] |
//! | `0x800816A0` | [`crate::game::stage::create_bg_unit_models`] |
//! | `0x80081780` | [`crate::game::stage::refresh_bg_unit_models`] |
//! | `0x80081800` | [`crate::game::stage::init_stage_resources`] |
//! | `0x80081850` | [`crate::game::stage::bg_unit_model`] |
//! | `0x80081870` | [`crate::game::stage::bg_unit_u16`] |
//! | `0x80081890` | [`crate::game::stage::bg_unit_mesh_offset`] |
//! | `0x80081BE0` | [`crate::game::stage::is_current_stage_unit`] |
//! | `0x80081D20` | [`crate::game::stage::create_bg_scene`] |
//! | `0x80081F40` | [`crate::game::stage::alloc_tile_buffers`] |
//! | `0x80082050` | [`crate::game::stage::free_tile_buffers`] |
//! | `0x800820D0` | [`crate::game::stage::free_tile_buffer`] |
//! | `0x8008DFF0` | [`crate::game::stage::parse_course_file`] |
//!
//! Course-data decoding - the bgdat layer readers and the object-id tables -
//! is in [`crate::resource::course`]:
//!
//! | address | name |
//! |---|---|
//! | `0x800818B0` | [`crate::resource::course::bg_layer_data`] |
//! | `0x800818E0` | [`crate::resource::course::object_id_table_entry`] |
//! | `0x80081900` | [`crate::resource::course::object_id_in_list`] |
//! | `0x80081BA0` | [`crate::resource::course::is_special_object_id`] |
//! | `0x80082120` | [`crate::resource::course::bg_layer_u16`] |
//!
//! # Still unknown
//!
//! No function was left behind for lack of a role. The open questions are all
//! field/table semantics, not missing translations, and they are recorded in
//! the destination docs (`# Confidence` blocks) and in
//! `docs/COVERAGE_r80080_800c0.md`:
//!
//! * the unit of `SECTION_DIVISORS` (`0x802F00F8`);
//! * the meaning of the object-id table at `0x802EF934` and the list at
//!   `0x802EF900`;
//! * the not-yet-reconstructed constructors `0x80081070` / `0x80082160` that
//!   make `create_bg_scene` `partial`.
//!
//! Ledger rows for these addresses now point at their semantic files; see
//! `ledger/agents/r80080_800c0.csv`.
