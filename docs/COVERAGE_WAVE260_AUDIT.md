# Audit - Wave 260 (seventh vtable, archive band closed, survey)

## 1. Two more tables complete

| Table | Slots | Notes |
|---|---:|---|
| `dEnBoss_c` `0x80312750` | **265/265** | 265 unique targets, 0 non-code |
| archive band `0x80350854..0x80350A18` | **79/79** | last three were `lyt` method slots (stop / copy-value / begin) |

**Seven vtables are complete**: player-class (275), WorldMap actor (278),
`WM_PLAYER_VTABLE` (311), `0x80329958` (45), `dEn_c` (306), die-state (235),
`dEnBoss_c` (265).

## 2. Survey: most "uncovered" pointer slots are dispatch tables

Across 540 runs in `0x80300000..0x80350000` there are 4,639 distinct uncovered
targets, but **2,127 are interior labels** (switch/jump-table entries). The runs
with the most uncovered slots are mostly dispatch tables materialised by
`lis`/`addi` inside already-translated functions - e.g. `0x803218ec` at
`0x801056c8` inside `0x801056c0`.

Real remaining body work, ranked by **prologue-shaped** targets:

| Run | Prologue | Total uncovered |
|---|---:|---:|
| `0x80306400` | 36 | 63 |
| `0x80303104` | 31 | 54 |
| `0x8030241c` | 26 | 60 |
| `0x80303aa4` | 25 | - |
| `0x80304e18` | 24 | 43 |
| `0x80304940` | 23 | 44 |
| `0x80305958` | 22 | 48 |

## 3. New band opened: `0x80350A18..0x80350C80`

Three `0x50`-stride records plus a **73-slot switch jump table** for the decimal
formatter `fn_802ddef0` (`exponent+64`, unsigned `cmpli 72`, indexed `bcctr`).
52 slots are the default arm and every target is an interior label. The three
genuine bodies (no-op record arm, table case `-64`, default arm) are translated;
the shipped default uses a truncating signed divide by `2` (`rlwinm`/`add`/
`srawi`, Ghidra's `(int)param_2 / 2`), and the wave-260 body's `>> 1` needs a
follow-up alignment.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `dEnBoss_c` 237-252 | 13 | 1,316 |
| B | `dEnBoss_c` 253-264 (closes table) | 12 | 476 |
| C | archive band last 3 (closes band) | 3 | 356 |
| D | next band 0x80350A18+ | 3 distinct | ~260 |

## 5. Findings

- Lane B: `work/wave259/b/disasm.txt` was overwritten with a byte-identical copy
  of wave 260 lane A's file (same SHA-256), so that evidence path needs
  re-deriving before an audit of wave-259 lane B. No source or ledger impact.
- Lane F audited 43 wave-259 bodies: all words, slots, extents, ownership and
  dispatch counts reproduce; 10 doc corrections, notably that slot 234's
  reachability claim holds only for `main.dol` (`d_en_bossNP.elf` calls slots
  228/235/236) and two wrong "next body" sizes.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0.
- `scripts/check.ps1` exit 0.
