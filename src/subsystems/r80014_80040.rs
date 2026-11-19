//! Staging lane `0x80014000..0x80040000` - translated code promoted, none left.
//!
//! # What this file is
//!
//! This was the extraction lane for every function whose **entry address** is
//! in `0x80014000..0x80040000` of `sys/main.dol` (image base `0x80004000`).
//! Extraction lanes exist so parallel translators can work by address without
//! symbols; they are not reader-facing. On 2026-11-01 all 20 functions that
//! this lane had translated were promoted, by moving them, into the semantic
//! `ui` modules:
//!
//! | Cluster | Original entries | Destination |
//! |---|---|---|
//! | `pointGet` coin pop-up | `0x80014F10`, `0x80014FF0`, `0x80015160`, `0x800151D0` | [`crate::ui::hud`] |
//! | `batteryInfo` warning scene | `0x80015D60..0x80016870` (12 entries) | [`crate::ui::hud`] |
//! | `errorInfo` warning scene | `0x80016F70..0x800175C0` (4 entries) | [`crate::ui::hud`] |
//! | NW4R `lyt` primitives shared by all three | `0x80004244`/`0x8000423C` SDA anchors, `0x800C8xxx`/`0x800C9xxx` callees | [`crate::ui::lyt`] |
//!
//! **No function definitions remain here.** The file is kept because
//! `src/subsystems/mod.rs` still declares the module and that file is outside
//! this lane's ownership. Do not add new translations here: pick the semantic
//! module first (see `docs/PROMOTION.md`).
//!
//! `pointGet`/`batteryInfo`/`errorInfo` were the only clusters in this lane
//! with strong string evidence (`*.brlyt`, `*.brlan`, `*.arc`). Their ledger
//! rows now point at `src/ui/hud.rs` and `src/ui/lyt.rs`.
//!
//! # Still staged in this range (not translated, not classified)
//!
//! These neighbours were identified by string evidence but have no translation
//! yet, so they remain first-pass next targets rather than promoted code. Sizes
//! and priorities come from `docs/COVERAGE_r80014_80040.md`:
//!
//! | Address | Size | Why it is interesting |
//! |---|---:|---|
//! | `0x80015C10` | 208 B | Coin motion/fade used by `fn_80015160`; removes the only `partial` |
//! | `0x80015780` | 216 B | Coin bounce tick, uses `r2` float `0.08` at `0x8042B1C0` |
//! | `0x80016A80` | 1012 B | `dWarningBattery_c` state table (5 `StateID_*` strings) |
//! | `0x80016430` | 596 B | `batteryInfo` per-frame update |
//! | `0x800174C0` | 256 B | Error window open animation (7-argument call, `0x10`-byte stack buffer) |
//! | `0x80017630` | 736 B | Remaining `errorInfo` state-machine body |
//! | `0x80018330` | 2168 B | `dWarningErrorInfo_c` state table (11 `StateID_*` strings) |
//! | `0x80018F20` | 352 B | `extensionControllerNunchuk` layout scene |
//! | `0x80019560` | 824 B | `dWarningNunchuk_c` state table |
//! | `0x8001B470` | 272 B | `wipeCircle` transition scene |
//!
//! Once translated, each belongs with its HUD/lyt peers: warning and info
//! windows in `ui/hud.rs`, transition wipes and menus in `ui/menu.rs`.
//!
//! # See also
//!
//! - `docs/COVERAGE_r80014_80040.md` - counts, verified tables and open
//!   questions for this range.
//! - [`crate::ui::hud`] - the three reconstructed scenes.
//! - [`crate::ui::lyt`] - the layout/animation primitives they use.
