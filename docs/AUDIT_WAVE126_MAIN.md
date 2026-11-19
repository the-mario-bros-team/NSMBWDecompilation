# Wave 126 — main.dol Module Ledger Audit

Read-only audit lane. Tool: `node tools/audit-unledgered.mjs --file <path>`, run once per file.
No Rust sources or ledger files were modified; no merge, no `scripts/check.ps1`.

## Summary

| File | Bodies | Ledgered | Unledgered | Problems |
| --- | ---: | ---: | ---: | ---: |
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
| `src/render/gx_band.rs` | 337 | 337 | 0 | 0 |
| `src/render/geometry.rs` | 50 | 50 | 0 | 0 |
| `src/engine/display.rs` | 402 | 402 | 0 | 0 |
| `src/engine/dsys.rs` | 149 | 149 | 0 | 0 |
| `src/engine/main_loop.rs` | 202 | 202 | 0 | 0 |
| `src/audio/ax.rs` | 399 | 399 | 0 | 0 |
| `src/audio/seq.rs` | 263 | 263 | 0 | 0 |
| **Total** | **6074** | **6074** | **0** | **0** |

All runs resolved to export `main-gb` (+ union fallback), with module names derived from the
source path (`runtime::*`, `ui::*`, `resource::*`, `render::*`, `engine::*`, `audio::*`).

## Salvageable addresses

None. Every run reported `unledgered: 0` and `verified: 0`, so the tool emitted no
`salvageable (...)` line (that line appears only when the salvage set is non-empty; the
`verified` counter equals the salvage set size).

## Conclusion

Every translated body in the 19 audited main.dol modules is represented in the ledger:
6074 of 6074 bodies ledgered, zero unledgered, zero problems, zero salvageable candidates.
No ledger gap exists for this lane, and no merge or ledger write is warranted from this audit.
