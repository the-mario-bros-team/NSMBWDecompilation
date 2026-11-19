# Audit - Wave 232 (pointer-table batch 5, evidence rewrite)

## Bodies translated (55 across five tables)

| Lane | Table | Slots | Bodies | Bytes |
|---|---|---|---:|---:|
| A | `WM_PLAYER_VTABLE` `0x80326794` | 0/9/16/25/27/36-38/43/45-47 | 12 | 868 |
| B | player-class vtable `0x803084C8` | 9, 23-34 | 13 | 336 |
| C | WorldMap vtable `0x80325420` | 23-32 | 10 | 672 |
| D | `dEn_c` vtable `0x80314060` | 24-34 | 10 | 572 |
| E | `.ctors` `0x802ED9E0` | 85-98 | 10 | ~1,000 |

Slots already translated were skipped and reported throughout.

## Findings

- **Anchor corrections.** Lane A found the `WM_PLAYER_VTABLE` header at
  `0x80326794` (two zero words) with the first method at `0x8032679C`, matching
  `wm_player_init`'s store at `+0x60` - the brief's `0x803267A4` was two words
  late. Lane B found **slot 9 (`0x80047260`) was a false "already covered"**:
  the wave-231 scan counted the address because a neighbouring doc mentions it
  in prose. Prose mentions must not count as coverage.
- **Player-slot byte family.** Slots 25 in both `dEn_c` and the player vtable
  return `this + 0x38D`, the owning-player byte (`params & 3`, `0xFF` = none)
  already documented in `display.rs`/`bases.rs`; slots 33/34 read it with
  `lbz`/`extsb`.
- **Score award.** Slot 34 in both families selects a text row (`{1,4,200,1000}`
  at `0x8042BB58`) and points (`{200,1000,40.0f,10.0f}` at `0x8042BB60`), draws
  at the item position plus `40.0f`, awards through `fn_80060690`/`fn_800cea20`,
  then calls `0x80162510`.
- **Eight identical installers.** `.ctors` slots 88-97 share one 112-byte shape:
  descriptor `+0x0C = 23`, floats `2160.0f / -30.0f / -478.0f`, `0x802DC780`
  push, then `38` to SDA1 - the same record shape as `Wm_en_quicksand`.
- **`+0x318`/`+0x31C` seed to `32.0`, not `0.0`** in the world-map player init.
- **Decoder bug:** `extsb` keeps its source in bits 6-10 and destination in
  11-15, so wave-231's `laneE_dis.mjs` prints the operands swapped.
- **Reported, not changed:** `src/game/enemies.rs:45756` (`fn_80a9be80`) calls
  `0x80097D30` with one argument while the body reads two, and `0x80A9BE80`
  never sets `r4`.

## Evidence hygiene (lane F)

**3,293** bare `Ghidra <addr>_FUN_<addr>.c` evidence values rewritten across
**127** agent CSVs: 1,616 `main-gb`, 789 `rel-d_basesNP`, 763
`rel-d_enemiesNP`, 125 `rel-d_en_bossNP`. All 3,293 resolved to files that
exist; trailing notes preserved byte-for-byte; only column 6 changed.

Export selection is unambiguous because the ranges are disjoint:
`0x80004000-0x802ED9CC` DOL; `0x805…` profile; `0x806…-0x807C…` bases;
`0x80A…-0x80B…` enemies; `0x80C…` boss. `smoke` was excluded as the synthetic
fixture that shadows `80004050`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `merge-ledger` OK; 20,250 rows, 0 duplicates; 0 bare evidence paths remain.
- `scripts/check.ps1` exit 0.
