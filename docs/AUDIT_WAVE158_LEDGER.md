# Wave 158 — ledger completeness audit (read-only)

Answer to one question: **now that `game::enemies` and `game::bosses` are
complete, is every translated body in the audited modules represented in
`ledger/functions.csv`?**

Read-only lane. No Rust file, ledger CSV or existing document was modified; the
only file written is this report. `tools/merge-ledger.mjs`,
`tools/check-lane-csv.mjs` and `scripts/check.ps1` were not run.

Method: `node tools/audit-unledgered.mjs --file <path>` for each module, from the
repository root. The tool counts a body only when a `pub unsafe fn` carries a
`# Original` doc section with an address, a byte count, an evidence file and a
confidence line, then checks the address against the Ghidra `functions.tsv` and
against `ledger/functions.csv`.

## Results

| file | bodies | ledgered | unledgered | problems |
| --- | ---: | ---: | ---: | ---: |
| `src/game/bases.rs` | 3816 | 3816 | 0 | 0 |
| `src/game/enemies.rs` | 3555 | 3555 | 0 | 0 |
| `src/game/bosses.rs` | 769 | 769 | 0 | 0 |
| `src/game/player.rs` | 537 | 537 | 0 | 0 |
| `src/game/worldmap.rs` | 1001 | 1001 | 0 | 0 |
| `src/game/stage.rs` | 1092 | 1092 | 0 | 0 |
| `src/render/gx_band.rs` | 337 | 337 | 0 | 0 |
| `src/render/geometry.rs` | 50 | 50 | 0 | 0 |
| `src/runtime/os.rs` | 508 | 508 | 0 | 0 |
| `src/runtime/bt.rs` | 693 | 693 | 0 | 0 |
| `src/runtime/dvd.rs` | 533 | 533 | 0 | 0 |
| `src/runtime/input.rs` | 388 | 388 | 0 | 0 |
| `src/runtime/cpp.rs` | 348 | 348 | 0 | 0 |
| `src/runtime/init.rs` | 5 | 5 | 0 | 0 |
| `src/ui/hud.rs` | 192 | 192 | 0 | 0 |
| `src/ui/menu.rs` | 151 | 151 | 0 | 0 |
| `src/ui/lyt.rs` | 221 | 221 | 0 | 0 |
| `src/resource/bres.rs` | 261 | 261 | 0 | 0 |
| `src/resource/archive.rs` | 549 | 549 | 0 | 0 |
| `src/resource/registry.rs` | 423 | 423 | 0 | 0 |
| `src/engine/display.rs` | 402 | 402 | 0 | 0 |
| `src/engine/dsys.rs` | 149 | 149 | 0 | 0 |
| `src/engine/main_loop.rs` | 202 | 202 | 0 | 0 |
| `src/audio/ax.rs` | 399 | 399 | 0 | 0 |
| `src/audio/seq.rs` | 263 | 263 | 0 | 0 |
| **total** | **16844** | **16844** | **0** | **0** |

Every file exits 0 (no verification failures). `verified` (salvageable) is 0 for
every file.

## Salvageable addresses

None. No audited module contains a translated body that is missing from the
ledger, so there is no salvage CSV to produce.

## Blind spots checked

The tool only recognizes `pub unsafe fn` with an evidence `.c` path, so three
classes of declaration were cross-checked by hand against `ledger/functions.csv`
and `work/ghidra/*/export/functions.tsv`:

- **48 `pub unsafe fn` bodies not counted by the tool.** 45 are already in the
  ledger but lack an evidence `.c` path in the doc block (`src/game/stage.rs` ×1,
  `src/render/gx_band.rs` ×8, `src/runtime/bt.rs` ×2, `src/runtime/dvd.rs` ×3,
  `src/runtime/cpp.rs` ×1, `src/engine/main_loop.rs` ×2, and 28 addressed
  hardware helpers in `src/runtime/os.rs`). The remaining 3 are the explicit
  instruction-encoding no-ops `os::sync`, `os::isync`, `os::syscall`, which have
  no function address and are documented as non-ledger helpers.
- **12 `pub fn` bodies in `src/runtime/cpp.rs`** (`fn_802dcc4c`, `fn_802dcdd8`,
  `fn_802dcec4`, `fn_802dcffc`, `fn_802dd0e0`, `fn_802dd1ec`, `fn_802dd210`,
  `fn_802df42c`, `fn_802dd238`, `fn_802df3b4`, `fn_802df414`, `fn_802d6f20`):
  all 12 already have ledger rows.
- **`pub(crate) unsafe fn` helpers** (e.g. `src/ui/lyt.rs` `read_f32`,
  `write_f32`, `call0_r`, `call4`, `call5`, `vtable_method`, `copy_vec3`): host
  shims with no translated function address, correctly outside the ledger.

Conclusion: all 16,844 audited translated bodies across the 25 modules are ledgered, with zero unledgered and zero problems.
