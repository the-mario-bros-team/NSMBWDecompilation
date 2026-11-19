//! Tombstone for the `0x80140000..0x80180000` range lane.
//!
//! # Status: no staged functions
//!
//! The original pass translated twenty functions in this lane. All twenty are
//! now promoted to semantic modules; **zero remain staged here**. This file
//! stays behind as the lane's header and navigation point so the
//! `subsystems::r80140_80180` path keeps resolving and so a reader lands on the
//! right module instead of an empty gap.
//!
//! | band | functions | new home |
//! |---|---|---|
//! | world-map interaction object (`0x8014E4E0..0x80150AB0`) | 10 | [`crate::game::interaction`] |
//! | world-map actor animation/state (`0x80142580..0x80145C50`) | 5 | [`crate::game::worldmap`] |
//! | task/state helpers (`0x8015F200..0x8015F850`) | 3 | [`crate::game::task`] |
//! | NW4R layout transform/registration (`0x8016C9E0`, `0x8016D180`) | 2 | [`crate::ui::lyt`] |
//!
//! Every move cut the definition and its doc block from this file; no copy was
//! left behind, and the `# Original` address/evidence chain travelled with each
//! function. The final pass moved `0x80144030` [`crate::game::worldmap::wm_update_held_item`]
//! and the three task helpers, then emptied the rest of this file.
//!
//! # Original address range
//!
//! `0x80140000..0x80180000` (lane 11 in `docs/LANES.md`). The twenty promoted
//! functions cover `0x80142580..0x8016D180`; the full inventory, including the
//! untranslated neighbours that still make up most of the lane, is in
//! `docs/COVERAGE_r80140_80180.md`.
//!
//! # Why it exists
//!
//! `src/subsystems/` is the extraction layer: one file per address range,
//! translated in parallel without symbols. A function moves out only when its
//! subsystem is identified, it has a full doc/evidence block, and its
//! destination module is ready (`docs/ORGANIZATION.md`). This lane has finished
//! that loop; the file remains only because the module is still declared in
//! `src/subsystems/mod.rs`.
//!
//! # Dependencies
//!
//! None - this module has no code and imports nothing. The promoted code
//! depends on [`crate::platform`] and, for the effect/transform helpers,
//! [`crate::engine::globals::SDA_BASE`].
//!
//! # Start here
//!
//! Read the destination modules instead: [`crate::game::worldmap`] for the
//! actor and effect helpers, [`crate::game::task`] for the task list,
//! [`crate::game::interaction`] for the pick-up/release object and
//! [`crate::ui::lyt`] for the layout pair.
//!
//! # Glossary
//!
//! * **DOL** - the Wii executable container (`sys/main.dol`); its text starts
//!   at virtual `0x80004000`.
//! * **task list** - a header `{ table, u16 count, u16 index }` whose entries
//!   are called until one returns something other than `1`; promoted to
//!   [`crate::game::task`].
//! * **NW4R `lyt`** - Nintendo's layout library used for menus/HUD; the two
//!   layout helpers moved to [`crate::ui::lyt`].
//! * **r2 / r13** - PowerPC small-data base registers. Ghidra prints them as
//!   `unaff_r2` / `unaff_r13`; the r13 base is decoded in the destination
//!   modules, the r2 base is used by the promoted world-map float helpers.
//!
//! # Confidence summary
//!
//! No confidence is recorded here because no code lives here. The individual
//! `# Confidence` notes moved with their functions; three task helpers are
//! `high`, `wm_update_held_item` is `medium` (inferred "held item" role) and
//! the layout pair is `medium`.
