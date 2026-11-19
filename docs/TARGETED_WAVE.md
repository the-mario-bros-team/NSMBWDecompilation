# Targeted wave 2

Goal: extend coverage along systems, not address ranges, and start the four REL
modules. Sources:

- `work/ghidra/main-gb/export` — 10,276 functions (main.dol)
- `work/ghidra/rel-d_profileNP/export` — pending
- `work/ghidra/rel-d_basesNP/export` — pending
- `work/ghidra/rel-d_enemiesNP/export` — pending
- `work/ghidra/rel-d_en_bossNP/export` — pending

REL addresses are synthetic canonical bases (`0x80500000`, `0x80600000`,
`0x80A00000`, `0x80C00000`); every function doc must say so.

## Lanes

| # | Scope | Destination | Evidence |
|---:|---|---|---|
| 1 | `d_profileNP` — profile registry, actor descriptors | `game/actor/profile.rs` | REL export |
| 2 | `d_basesNP` — base gameplay / player | `game/player.rs` | REL export |
| 3 | `d_enemiesNP` — enemy actors and AI | `game/enemies.rs` | REL export |
| 4 | `d_en_bossNP` — boss actors | `game/bosses.rs` | REL export |
| 5 | GX/render band `0x8028–0x802A` + paired-single pass | `render/gx.rs` | main.dol export |
| 6 | Remaining NW4R lyt/list/BRLAN functions | `ui/lyt.rs` | range coverage docs |
| 7 | `OSLink` / REL loader, real REL bases | `runtime/link.rs` | main.dol export |
| 8 | BRRES payloads + `dRes_c` registry (no BCSV in this build) | `resource/bres.rs`, `resource/registry.rs` | format docs + arcs |

## Ground rules

1. Write directly into the semantic module; do not create new range staging.
2. Each lane owns its destination file(s) and one source export range.
3. Keep `# Original` evidence and synthetic-base notes for REL functions.
4. `cargo check` + `doc-lint` must pass for the lane's files.
5. Update `ledger/agents/wave2-<lane>.csv` and `docs/COVERAGE.md`.
6. Unknown names stay `fn_<addr>`; never invent Nintendo symbols.

## Exit criteria

- Wave 2 adds at least one complete cluster per lane (not just stubs).
- `docs/FUNCTION_INDEX.md` shows the new semantic modules.
- No new code lands in `src/subsystems/`; that tree only shrinks.
- REL functions carry `// synthetic base:` markers.
