//! Main-`DOL` text, entry addresses `0x80100000..0x80140000` (lane 10).
//!
//! **Status: fully promoted.** This lane translated 28 functions in two passes
//! and none of them remain here:
//!
//! * clusters A-C (four-slot global state, bit table/reset, packed identifier)
//!   moved to [`crate::engine::globals`];
//! * clusters D-E (WorldMap effect slots and shared transform math) moved to
//!   [`crate::game::worldmap`].
//!
//! **Why the file still exists.** `subsystems/mod.rs` still declares the lane,
//! so this module keeps a header for readers who arrive from an address in the
//! range. There is no code to read here; follow the links above.
//!
//! **Evidence.** `work/ghidra/main-gb/export/functions.tsv` and the
//! per-function C files cited in the `# Original` blocks of the destination
//! modules.
//!
//! **Coverage.** `docs/COVERAGE_r80100_80140.md` tracks the range inventory and
//! `ledger/agents/r80100_80140.csv` records the final module for every row.
//!
//! **Still deferred.** The three paired-single/GQR bodies `0x80101AA0`,
//! `0x80102DB0` and `0x80103B40` were never translated, so they are not
//! promoted; they remain future targets in the coverage doc.
