//! `0x80200000..0x8025FFFF` - Broadcom BTE Bluetooth host stack (staging).
//!
//! # Status: promoted, nothing left here
//!
//! All 20 functions translated by this lane now live in [`crate::runtime::bt`],
//! organised by Bluetooth layer (HID host, BTM, HCI, L2CAP, RFCOMM, utilities).
//! This file is intentionally a pointer: the crate keeps exactly one
//! implementation per original address, so the staging file must not hold
//! copies (`docs/ORGANIZATION.md`).
//!
//! The machine-readable provenance is still
//! `ledger/agents/r80200_80260.csv` (rows now point at `src/runtime/bt.rs`),
//! and the lane status is `docs/COVERAGE_r80200_80260.md`.
//!
//! # What remains in the range
//!
//! 1168 of the 1188 eligible entries (size `>= 16` bytes) are still
//! untranslated. New translations belong in [`crate::runtime::bt`] while they
//! serve the Bluetooth stack; a helper that proves to belong elsewhere should
//! go to that semantic module directly and cite this range in its `# Original`
//! block.
//!
//! Highest-value next targets, from the coverage document:
//!
//! | Address | Size | Why |
//! |---|---:|---|
//! | `0x8020AC78` | 396 | HCI queue function used by all three translated builders; translating it names the `+2/+4` header fields and closes the packet path. |
//! | `0x80200310` | 788 | HID-host slot allocator + connect-complete path; defines the full `0x11C` slot layout. |
//! | `0x80200624` | 196 | Disconnect by `BD_ADDR`; counterpart of `fn_8020213c`. |
//! | `0x8020E55C` | 176 | Worker called by the stored-link-key deletion path. |
//! | `0x80212810` | 144 | L2CAP deregistration worker behind `fn_80212648`. |
//! | `0x80214B5C` | 188 | Disconnect worker behind `fn_802006e8`. |
//! | `0x80218040` | 184 | PSM lookup used by `0x80212810`. |
//! | `0x8021CE04` | 176 | Only caller that appends the `fn_8021e44c` checksum byte. |
//! | `0x8023C280` | 908 | Only caller of `fn_80239a10`; would identify the 6-byte record format. |
//! | `0x80251930` | 1308 | Only caller of `fn_8024d390`; would identify the `0x803DFFB4` table. |
//!
//! Functions whose Ghidra body depends on `unaff_r13` stay untranslated until
//! the small-data-area base is recovered; encoding them now would guess an
//! address (see `docs/GLOSSARY.md`).
//!
//! # Evidence
//!
//! Ghidra exports for this range live in
//! `work/ghidra/main-gb/export/<addr>_<name>.c`; addresses are virtual
//! addresses of the unrelocated `main.dol` (image base `0x80004000`).
