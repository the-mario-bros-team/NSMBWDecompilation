# Audit - Wave 259 (three more tables complete)

## 1. Tables finished this wave

| Table | Slots | Notes |
|---|---:|---|
| die-state `0x80313760` | **235/235** | 235 unique targets, no repeated slots; 205 untabled bodies across `stage` (115), `enemies` (89), `worldmap` (24), `player` (7) |
| `0x80312200` | **70/70** | slot 69 shared with `dEnBoss_c` slot 225 |
| `0x80311F88` array | **226/226** | only slot 9 (`0x801621D0`, constant-true) was open outside `0x80312200` |

**Six vtables are complete**: player-class (275), WorldMap actor (278),
`WM_PLAYER_VTABLE` (311), `0x80329958` (45), `dEn_c` (306), die-state (235).

## 2. Coverage after the wave

| Table | Covered |
|---|---|
| `dEnBoss_c` `0x80312750` | 240/265 (25 uncovered) |
| archive band `0x80350854..0x80350A18` | 76/79 (3 uncovered) |

## 3. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | die-state 221-234 (closes table) | 14 | 692 |
| B | `dEnBoss_c` 225-236 | 11 + 1 callee | 1,516 |
| C | `0x80312200` last 6 (closes table) | 6 | stubs |
| D | archive band 0x8035098C+ | 10 + 2 tails | ~2,100 |
| E | `0x80311F88` slot 9 (closes array) | 1 | 8 |

## 4. Findings

- Lane B avoided a duplicate: slot 225's target was already defined by lane E
  in the same wave, so it documented the slot as closed rather than redefining.
- Lane D corrected a signature outside its lane: `render::gx_band::fn_80292b60`
  consumes its node in `r3` (`0x80292C60: mr r3,r30`) but was typed `void`; it
  now returns `usize` (node or 0). Its only Rust caller is the new `0x802DB210`.
- The `0x802DBxxx` words are create/attach/update/draw overrides of the
  `0x802929D0/2B60/2C90/941F0` base class.
- Lane F audited 48 wave-258 bodies: 48/48 words and extents correct; 7 doc
  fixes including a byte-count correction (592 -> 600) and three wrong
  "next body" citations.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0.
- `scripts/check.ps1` exit 0; ledger 20,250 rows, 0 staging.
