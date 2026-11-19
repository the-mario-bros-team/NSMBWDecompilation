# Coverage snapshot

Generated from `ledger/functions.csv` by `tools/coverage-doc.mjs`.
Do not edit by hand; regenerate after merging lane ledgers.

Ledger total: **20250 functions** - 20250 semantic, 0 staging.

## Semantic modules (reader-facing)

| Module | Functions | Translated | Partial | Stub |
|---|---:|---:|---:|---:|
| audio::ax | 399 | 398 | 1 | 0 |
| audio::seq | 263 | 263 | 0 | 0 |
| boot::cache | 1 | 1 | 0 | 0 |
| boot::entry | 6 | 5 | 1 | 0 |
| boot::init_tables | 1 | 1 | 0 | 0 |
| boot::mem | 3 | 3 | 0 | 0 |
| engine::display | 402 | 402 | 0 | 0 |
| engine::dsys | 397 | 397 | 0 | 0 |
| engine::frame | 15 | 15 | 0 | 0 |
| engine::globals | 47 | 47 | 0 | 0 |
| engine::main_loop | 203 | 198 | 5 | 0 |
| engine::startup | 4 | 4 | 0 | 0 |
| game::actor | 57 | 57 | 0 | 0 |
| game::actors::dokan | 5 | 5 | 0 | 0 |
| game::bases | 5607 | 5607 | 0 | 0 |
| game::boot_scene | 12 | 12 | 0 | 0 |
| game::bosses | 770 | 770 | 0 | 0 |
| game::enemies | 3555 | 3553 | 2 | 0 |
| game::interaction | 17 | 17 | 0 | 0 |
| game::path | 37 | 37 | 0 | 0 |
| game::player | 830 | 830 | 0 | 0 |
| game::profile | 36 | 36 | 0 | 0 |
| game::scene | 31 | 31 | 0 | 0 |
| game::stage | 1226 | 1226 | 0 | 0 |
| game::task | 16 | 16 | 0 | 0 |
| game::worldmap | 1001 | 998 | 3 | 0 |
| game::yoshi | 7 | 7 | 0 | 0 |
| render::geometry | 50 | 50 | 0 | 0 |
| render::gx | 3 | 3 | 0 | 0 |
| render::gx_band | 345 | 342 | 3 | 0 |
| resource::archive | 747 | 747 | 0 | 0 |
| resource::bres | 261 | 256 | 5 | 0 |
| resource::course | 19 | 19 | 0 | 0 |
| resource::registry | 423 | 423 | 0 | 0 |
| runtime::bt | 695 | 695 | 0 | 0 |
| runtime::cpp | 361 | 361 | 0 | 0 |
| runtime::dvd | 536 | 536 | 0 | 0 |
| runtime::ids | 4 | 4 | 0 | 0 |
| runtime::init | 7 | 5 | 1 | 1 |
| runtime::input | 388 | 388 | 0 | 0 |
| runtime::link | 12 | 12 | 0 | 0 |
| runtime::os | 537 | 536 | 0 | 1 |
| ui::hud | 542 | 541 | 1 | 0 |
| ui::lyt | 221 | 221 | 0 | 0 |
| ui::menu | 151 | 151 | 0 | 0 |

Semantic total: **20250** (20226 translated, 22 partial, 2 stub).

## Staging ranges (not reader-facing)

| Module | Functions | Translated | Partial | Stub |
|---|---:|---:|---:|---:|

Staging total: **0**. These shrink as functions are promoted; see docs/PROMOTION.md.

## Context

- `main.dol` has **10,276** decompiled functions; the four RELs add **9,976** (36 + 5,615 + 3,555 + 770).
- The ledger counts only functions that have been translated at least once; it is not a progress bar for all 20,252.
- "Translated" means the recorded control flow is represented; inferred field names may still be unproven.
- Gates: `cargo check` must be 0 and `node tools/doc-lint.mjs src` must be 0 before a lane is merged.

## Refresh

```powershell
node tools/merge-ledger.mjs
node tools/index-functions.mjs
node tools/coverage-doc.mjs
```
