# Audit - Wave 236 (`.ctors` finished, 25 more page-offset bugs fixed)

## 1. `.ctors` complete: 181 / 181

| Class | Slots | Bytes |
|---|---:|---:|
| Tabled (`functions.tsv` row) | 38 | 54,148 |
| Untabled | 143 | 60,732 |
| Skipped | 0 | 0 |
| **Total** | **181** | **114,880** |

All 181 slots have a body: 36 as `fn_<addr>`, 2 under semantic names
(`wm_projection_globals_init`, `dgame_display_register_states`), and 143 newly
translated across waves 229-236.

Off-by-one found: the three zero terminators start at `0x802EDCB4`, not
`0x802EDCB0` as a wave-229 comment says.

## 2. Literal verification: 2,320 of 2,338 checked, 25 wrong, all fixed

An intraprocedural provenance tracker was built over **all five images**
(main.dol + 4 RELs), following `lis`/`addi`/`addis`/`ori`/`rlwinm`/`add`/`subf`
folding with sign-extended immediates, image loads (so table and `.sdata` base
pointers resolve), float loads/stores, and `lwzu`/`stwu` base updates. The SDA
bases were read from `__start` (`r13 = 0x8042F680`, `r2 = 0x80433080`) - wave
235's guesses were wrong.

Resolved: exact 1,572; materialised 124; pointer-table slot 198; indexed table
152; `.ctors` member 146; object-relative 86; string 21; fixed page shift 21.
The 18 residual are 12 BSS fields and 6 data cells with no code reference in any
image.

**25 literals in 10 clusters were wrong**, all the same `addi` sign-extension
error inflating a `lis` high-half result by one page, and each was verified
against both the producing instruction pair and the data at both pages:

- `boot/init_tables.rs`: `0x80338A18`, `0x8031F520`, `0x8030D328/32C/330`,
  `0x8032D378/39C/890/89C/8A8/8AC/8B0`
- `engine/globals.rs`, `game/bases.rs`: `0x8032D39C` (bases.rs had even
  rationalised the wrong page as a "REL recipe adds 0x10000")
- `engine/dsys.rs`: `0x8032BCC0`
- `game/player.rs`: `0x803396A8`
- `resource/archive.rs`: `0x8030D1EC/1E0/208`
- `game/enemies.rs`, `game/actor.rs`, `game/worldmap.rs`: `0x803395D8/548/989C`,
  `0x80339958`, `0x802FE018`, `0x802FEEF0/EF68/EFE0`

Constants were renamed to match the corrected pages.

## 3. Bodies translated (54)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `.ctors` 155-168 | 12 | 792 |
| C | `.ctors` 169-180 | 12 | 840 |
| D | `WM_PLAYER_VTABLE` 91-102 | 12 | 72 |
| E | WorldMap vtable 66-80 | 10 | 868 |
| F | `dEn_c` 65-75 | 10 | 2,740 |

Notable: slot 168 computes the timer scale as `0x800000F8 >> 2` through the
`0x431BDE83` reciprocal multiply, giving `18225` at 243 MHz (300 us of
time-base ticks). `dEn_c` slots 73/74 compare "no progress" against the count
captured **before** the SDA clear. `WM_PLAYER_VTABLE` 91-102 are four mode
families of one/two-instruction forwarders.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0.
- Ledger 20,250 rows, 0 duplicates.
