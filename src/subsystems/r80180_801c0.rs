//! Range lane `r80180_801c0` - main.dol entry addresses `0x80180000..0x801C0000`.
//!
//! # What this module covers
//!
//! Nothing: every function this lane translated has been promoted. The file is
//! kept because `src/subsystems/mod.rs` still declares the lane, and it remains
//! the address record for range 12 in `docs/LANES.md`; it is not a home for
//! code.
//!
//! The range itself holds 753 eligible functions (872 raw entries minus 111
//! bodies shorter than 16 bytes minus 8 addresses owned by the runtime lane).
//! This lane translated 20 of them, all of which now live in semantic modules.
//!
//! | promoted functions | destination |
//! |---|---|
//! | `0x80192820`, `0x80192880`, `0x801928A0`, `0x801928D0` - 16-slot id/slot table | [`crate::runtime::ids`] |
//! | `0x8019F180`, `0x8019F1A0`, `0x8019F1B0`, `0x8019F1D0` - audio-DMA MMIO | [`crate::audio::ax`] |
//! | `0x80182060`, `0x80182080`, `0x801820A0`, `0x80182100`, `0x80182190`, `0x80183500` - `BRLAN` animation-transform sections | [`crate::ui::lyt`] (`BRLAN` section) |
//! | `0x8018A590`, `0x8018A5C0`, `0x8018A5E0`, `0x8018A650`, `0x8018A6C0`, `0x8018A6E0` - NW4R intrusive lists | [`crate::ui::lyt`] (`list` section) |
//!
//! # Why it exists
//!
//! The DOL is partitioned into disjoint address ranges so several lanes can
//! translate in parallel. A range module keeps the translations until their
//! subsystem is identified and a semantic module owns them. For this lane that
//! is now true for all 20 functions, so the module is empty by design.
//!
//! # What did not move
//!
//! Nothing is staged: none of the 20 translations was unclassifiable. The
//! remaining 733 eligible functions in the range were never translated; the
//! unit inventory and next targets stay in `docs/COVERAGE_r80180_801c0.md`.
//!
//! # Object layouts recovered from the evidence
//!
//! The `BRLAN` transform object and the intrusive-list header are documented in
//! [`crate::ui::lyt`], next to the code that reads them:
//!
//! * transform object `+0x4C`/`+0x50` packed counts, `+0x58` work buffer,
//!   `+0x18`/`+0x1C` embedded list count/head;
//! * list header `{head, tail, u16 count, u16 link_offset}` with two-word node
//!   links (`+0` older, `+4` newer).
//!
//! # Start here
//!
//! There is no code here. For the promoted bodies read [`crate::ui::lyt`]'s
//! `BRLAN` and `list` sections; for the evidence still attached to this range
//! read `docs/COVERAGE_r80180_801c0.md`.
//!
//! # Dependencies
//!
//! None. The module has no imports, no calls and no constants.
//!
//! # Glossary (first use)
//!
//! * **DOL** - the Wii executable container (`sys/main.dol`); see
//!   `docs/GLOSSARY.md`.
//! * **NW4R / nw4hbm** - NintendoWare libraries used by the game.
//! * **BRLAN / `RLMC`/`RLTS`/`RLTP`/`RLIM`** - layout animation resource and
//!   its material-colour, texture-SRT, texture-pattern and indirect-material
//!   track tags, observed as big-endian magic words in `0x8017DCC0`.
//!
//! # Confidence summary
//!
//! Not applicable: no code remains. Per-function confidence travelled with each
//! body and stays visible in its `# Confidence` block and in the ledger row.
