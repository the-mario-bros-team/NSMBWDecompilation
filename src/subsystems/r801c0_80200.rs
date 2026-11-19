//! Staging range `0x801C0000-0x80200000` - promoted out.
//!
//! This address-range lane translated two SDK clusters and has now handed all
//! 20 functions to semantic modules:
//!
//! * **DVD / FST** (`0x801CA650`-`0x801CB2E0`) → [`crate::runtime::dvd`]:
//!   FST caching, path lookup, open/read/close, disc-change and completion
//!   callbacks.
//! * **WPAD / KPAD** (`0x801DF570`-`0x801E1060`) → [`crate::runtime::input`]:
//!   Wii Remote control-block reset/bring-up, locked field accessors,
//!   connection/reset callbacks and the public callback setters.
//!
//! # Remaining staging
//!
//! **None.** Every translated function had evidence for its subsystem, so none
//! was left behind. The low-confidence entries are field-semantics gaps, not
//! role gaps: the DVD event-ring helpers and the WPAD/KPAD accessors still
//! share the FST globals or the 0x9C0-byte control blocks named in their docs.
//!
//! This file is retained only because [`crate::subsystems`] still declares
//! the range module; that declaration and this header can be retired by the
//! range owner. Per-function promotion records live in
//! `docs/COVERAGE_r801c0_80200.md` and `ledger/agents/r801c0_80200.csv`.
