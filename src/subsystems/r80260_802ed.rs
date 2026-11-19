//! Promotion tombstone - `0x80260000..0x802ED9DF` (`r80260_802ed`).
//!
//! # What this file is
//!
//! This range module was the extraction home of twenty translated functions in
//! the tight cluster `0x8026F2D0..0x80270DA0`. The promotion pass **moved** every
//! one of them (definitions, doc comments and `# Original` evidence) into
//! semantic modules and left this tombstone behind so the address lane stays
//! traceable. Nothing is duplicated: the code lives only at the destinations.
//!
//! | cluster | functions | destination | ledger module |
//! |---|---|---|---|
//! | archive reader + packed values | 5 | `src/resource/archive.rs` | `resource/archive` |
//! | four-slot player table, pool, alarms, completions | 13 | `src/game/player.rs` | `game/player` |
//! | `RSEQ` header + name lookup | 2 | `src/audio/seq.rs` | `audio/seq` |
//!
//! The lane-local call helpers moved with their callers: the four-, five-,
//! seven-argument and 64-bit forms now come from `crate::platform`, and the two
//! destinations that need a result-preserving `call0` declare `call0_u32`
//! locally (`crate::platform::call0` discards its result).
//!
//! # Start here
//!
//! * `resource::archive` - read `fn_8026f770` first (the reader layout and the
//!   path normaliser), then `fn_8026fc50`, `fn_8026fdb0`, `fn_8026f930` and
//!   `fn_8026f2d0`.
//! * `game::player` - read `fn_80270000` first (the pool header), then
//!   `fn_80270120` → `fn_802701a0` → `fn_80270330` → `fn_80270410`, and
//!   `fn_80270b70` + `fn_80270720` for the completion path.
//! * `audio::seq` - read `fn_80270d20` (the `RSEQ` magic/version test) then
//!   `fn_80270da0` (the name-table walk).
//!
//! # What remains staged
//!
//! **Nothing.** All twenty translated functions had an identified subsystem and
//! an evidence chain, so none was unclassifiable and none was left behind.
//!
//! The rest of the address range was never translated by this lane: 1678
//! eligible entries existed and only these 20 were translated. The untranslated
//! neighbours that matter to the promoted cluster - `0x80270AD0` (lazy table
//! initialiser), `0x80270C20` (periodic tick), `0x80270920` (completion C),
//! `0x8026FD00` (index-block sibling), `0x8026FB90` (archive header loader) and
//! the path helpers `0x802756E0` / `0x80275CC0` / `0x80275ED0` - are listed with
//! rationale in `docs/COVERAGE_r80260_802ed.md`.
//!
//! # Glossary
//!
//! * **DOL** - the Wii executable container `sys/main.dol`; addresses here are
//!   its *virtual* addresses (image base `0x80004000`), not file offsets.
//! * **NW4R / nw4hbm** - NintendoWare 4 Revolution, the SDK layer this sound
//!   code belongs to (see `docs/GLOSSARY.md`).
//! * **RSEQ** - the sound-sequence file format; magic bytes `R S E Q`.
//! * **Stage / promotion** - see `docs/ORGANIZATION.md`: an address-range module
//!   is staging; a semantic module is the reader-facing home.
