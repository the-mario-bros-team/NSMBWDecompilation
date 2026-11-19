# Audit - Wave 258 (fifth vtable complete, boss tables decoded)

## 1. `dEn_c` vtable complete: 306/306

The `0x80314060` table had only four slots left (302-305); they are translated and
the table is done.

**Five vtables are now complete:**

| Table | Slots |
|---|---:|
| player-class `0x803084C8` | 275 |
| WorldMap actor `0x80325420` | 278 |
| `WM_PLAYER_VTABLE` `0x8032679C` | 311 |
| `0x80329958` (derived archive class) | 45 |
| `dEn_c` `0x80314060` | 306 |

## 2. Boss tables

- `0x80312200` is **not a standalone vtable**: it is slots 156-225 of a
  **226-slot array headed by `0x80311F88`** (installed at `+0x60` by
  `fn_800983c0`), terminated by the header of the next 14-slot vtable at
  `0x80312320`. Coverage there went 23 -> 6 uncovered.
- `dEnBoss_c` (`0x80312750`, 265 slots) is down to **37 uncovered**.
- The damage-spin family (bodies 187/189/190/195) differs only in entry guard,
  notice/stat calls and final mode.

## 3. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | die-state 206-220 | 14 | 592 |
| B | `dEn_c` 302-305 (closes table) | 4 | 76 |
| C | `dEnBoss_c` 198-224 | 10 | 184 |
| D | archive band 0x80350950+ | 10 | ~2,600 |
| E | `0x80312200` first 10 | 10 | stubs |

Coverage: die-state 221/235 (14 tail slots left), boss 37/265, archive band
66/79, `dEn_c` 306/306.

## 4. Findings

- Float-bearing calls must use the `f32` helpers when the callee does
  `stfs`/`fmr` on `f1` (slot 220), not the older `f64` widening used by
  zero-only call sites.
- Lane D corrected the wave-257 note: the band's first vtable base is
  `0x80350938` (destructor `0x802DA500` at `+0x00`, then three nulls), not
  `0x80350940`, matching its sibling `0x8034F340`.
- The band holds two vtables (`0x80350938` 12 words, `0x80350970` 10 words) each
  with a near-identical sibling, and the three `0x802DAAxx` bodies are overrides
  of the sibling's create/attach/final-shape methods.
- Lane F audited 40 wave-257 bodies: no wrong word, target or extent; 11 doc
  corrections (slot offsets, descriptor-run end, padding counts, a slot number,
  a false twin claim).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0.
- `scripts/check.ps1` exit 0.
