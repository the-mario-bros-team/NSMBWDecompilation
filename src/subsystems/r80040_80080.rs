//! Tombstone for the promoted actor/player range `0x80040000..0x80080000`.
//!
//! This file no longer contains code. It remains as the ledger anchor for the
//! `r80040_80080` lane and as the record of the promotion passes; every body,
//! doc comment and `# Original` evidence block moved unchanged to the semantic
//! modules below.
//!
//! # Promoted, by destination
//!
//! | Address | Function | New home |
//! |---|---|---|
//! | `0x80040540` | `fn_80040540` | [`crate::runtime::cpp`] |
//! | `0x800405A0` | `fn_800405a0` | [`crate::runtime::cpp`] |
//! | `0x80040720` | `fn_80040720` | [`crate::runtime::cpp`] |
//! | `0x80040800` | `fn_80040800` | [`crate::runtime::cpp`] |
//! | `0x800408C0` | `fn_800408c0` | [`crate::runtime::cpp`] |
//! | `0x800483F0` | `fn_800483f0` | [`crate::runtime::cpp`] (appended) |
//! | `0x80048C40` | `fn_80048c40` | [`crate::game::player`] |
//! | `0x8004B160` | `fn_8004b160` | [`crate::game::player`] |
//! | `0x8004B1A0` | `fn_8004b1a0` | [`crate::game::player`] |
//! | `0x8004B1C0` | `fn_8004b1c0` | [`crate::game::player`] |
//! | `0x8004B520` | `fn_8004b520` | [`crate::game::player`] |
//! | `0x8004BF00` | `fn_8004bf00` | [`crate::game::player`] |
//! | `0x8004BF80` | `fn_8004bf80` | [`crate::game::player`] |
//! | `0x8004CD20` | `fn_8004cd20` | [`crate::game::player`] |
//! | `0x8004DE30` | `fn_8004de30` | [`crate::runtime::cpp`] (appended) |
//! | `0x80041C00` | `fn_80041c00` | [`crate::game::actors::dokan`] |
//! | `0x80041D40` | `fn_80041d40` | [`crate::game::actors::dokan`] |
//! | `0x80042350` | `fn_80042350` | [`crate::game::actors::dokan`] |
//! | `0x80042970` | `fn_80042970` | [`crate::game::actors::dokan`] |
//! | `0x80042A70` | `fn_80042a70` | [`crate::game::actors::dokan`] |
//!
//! # Why the file is empty
//!
//! A range module is extraction staging, not a subsystem (see
//! `docs/ORGANIZATION.md`). Once every sampled function is understood, nothing
//! is left to stage. The ledger rows point at the destinations, and
//! `docs/COVERAGE_r80040_80080.md` keeps the inventory, the clustering and the
//! reading order for the rest of the range.
//!
//! # Range inventory
//!
//! `functions.tsv` lists 1,015 entries in this range; 874 are at least 16 bytes
//! and 141 are shorter. This lane sampled 20 entries for clear evidence rather
//! than address contiguity: all 20 are promoted (19 `translated`, 1 `partial`),
//! none remain staged. The other 854 eligible entries are unclassified; the
//! coverage doc lists the best next reads.
//!
//! # Where to read next
//!
//! * [`crate::game::player`] - the gameplay player's pose/state predicates and
//!   the two effect spawns, beside the sound library's sequence player.
//! * [`crate::runtime::cpp`] - destructor/vtable plumbing and the two generic
//!   byte-field helpers.
//! * [`crate::game::actors::dokan`] - the `obj_dokan` pipe actor.
