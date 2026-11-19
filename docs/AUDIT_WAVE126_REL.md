# Wave 126 — REL Module Ledger Audit

Read-only audit lane. Tool: `node tools/audit-unledgered.mjs --file <path>`, run once per file.
No Rust sources or ledger files were modified; no merge, no `scripts/check.ps1`.

## Summary

| File | Bodies | Ledgered | Unledgered | Problems |
| --- | ---: | ---: | ---: | ---: |
| `src/game/bases.rs` | 3172 | 3172 | 0 | 0 |
| `src/game/enemies.rs` | 2866 | 2866 | 0 | 0 |
| `src/game/bosses.rs` | 769 | 769 | 0 | 0 |
| `src/game/player.rs` | 537 | 537 | 0 | 0 |
| `src/game/worldmap.rs` | 1001 | 1001 | 0 | 0 |
| `src/game/stage.rs` | 1092 | 1092 | 0 | 0 |
| **Total** | **9437** | **9437** | **0** | **0** |

Module/export bindings resolved by the tool:

- `bases.rs` → `game::bases`, export `rel-d_basesNP` (+ union fallback)
- `enemies.rs` → `game::enemies`, export `rel-d_enemiesNP` (+ union fallback)
- `bosses.rs` → `game::bosses`, export `rel-d_en_bossNP` (+ union fallback)
- `player.rs` → `game::player`, export `main-gb` (+ union fallback)
- `worldmap.rs` → `game::worldmap`, export `main-gb` (+ union fallback)
- `stage.rs` → `game::stage`, export `main-gb` (+ union fallback)

## Salvageable addresses

None.

All six runs reported `unledgered: 0`, so the tool emitted no `salvageable (...)` line (that line is only printed when the salvage set is non-empty; the `verified` counter equals the salvage set size and was `0` in every run).

## Conclusion

Every translated body in the six audited REL modules is already represented in the ledger: 9437 of 9437 bodies ledgered, zero unledgered, zero problems, and zero salvageable candidates. No ledger gap exists for this lane, and no merge or ledger write is warranted from this audit.
