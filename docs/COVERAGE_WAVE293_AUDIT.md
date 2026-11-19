# Audit — Wave 293 (six tables closed, 41 bodies, wave-292 audit + store census)

## 1. Tables closed

| Table | Slots | Identity |
|---|---:|---|
| `0x80311370` | **5/5** | stage-effect / animation-set vtable (`ice_piece` block) |
| `0x8031151C` | **5/5** | six-segment water-wave block vtable |
| `0x80322DD8` | **19/19** | WiiStrap overlay method array |
| `0x80324A44` | **51/51** | `0x510`-byte WorldMap effect object method array |
| `0x803276F8` | **18/18** | `dGameDisplay_c` vtable |
| `0x80309940` | **54/54** | preliminary actor / display table |

**One hundred three vtables/method arrays complete** (97 at wave 292 + these 6).

Notable structure:

- `0x80311370` and `0x8031151C` both install base header `0x8031133C`; neither
  run contains it as a slot, so they share no target. Each has one plain-`blr`
  empty hook (`0x80090ED0`, `0x80091990`), which is the 4-byte difference from
  the 1,644 / 480-byte extents in `docs/UNTABLED_POINTER_TABLES.md`.
- `0x8031151C` has no zero terminator: the word after the run is the first byte
  of `"Wm_en_waterwave_in"`.
- `0x803276F8` is installed by `fn_80157710` at object `+0x60` and published
  through `r13 - 0x5358`; the class-name string ``"`dGameDisplay_c::"`` sits at
  `0x803278DF`. The following header `0x80327740` is a distinct 9-slot scene
  interface table.
- `0x80309940` closed with 4 prologue bodies (476 B) on top of its 50 already
  covered slots.
- `0x80322DD8` header follows the `"NedEU_00"` language block; `0x80324A44`
  header follows the three zero words terminating sibling table `0x803249E0`.

## 2. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `src/game/stage.rs` (`_A293`) | 10 | 2,132 |
| B | `src/game/worldmap.rs` (`_B293`) | 10 | 1,088 |
| C | `src/engine/display.rs` + `src/game/player.rs` (`_C293`) | 9 | 2,372 |
| D | survey refresh only | 0 | — |
| E | gap batch 23 (audio, archive, hud, stage, dsys, …) | 12 | 1,472 |
| F | audit + store census | 0 | — |

**41 bodies, 7,064 bytes** translated this wave. All are **untabled** (no
`functions.tsv` row); each carries `# Original (untabled)` and adds no ledger
row. `ledger/functions.csv` therefore stays at **20,250 rows**.

## 3. Survey (post-work refresh)

`work/wave293/d_scan_after.mjs` / `after/d_*` reuse the wave-293 scanner
unchanged except for `OUT`.

| Metric | Wave 292 | Wave 293 (pre-work) | Wave 293 (post-work) |
|---|---:|---:|---:|
| Runs `>= 4` | 540 | 540 | 540 |
| Runs with uncovered slots | 423 | 417 | **410** |
| Runs with uncovered prologue | 284 | 277 | **270** |
| **Runs fully covered** | **117** | 123 | **130** |
| Uncovered slots | 4,420 | 4,028 | **3,996** |
| Uncovered prologue slots | 634 | 523 | **496** |
| Uncovered other (untabulated) | 1,659 | 1,378 | **1,373** |
| Interior-label slots | 2,127 | 2,127 | 2,127 |
| Distinct uncovered targets | 2,617 | 2,587 | **2,558** |
| **Distinct uncovered prologue targets (runs)** | **484** | 465 | **441** |
| Distinct prologue targets window-wide | 609 | 589 | **562** |

Seven runs completed, zero regressed: `0x80309940`, `0x80309a48` (via the gap
batch), `0x80311370`, `0x8031151c`, `0x80322dd8`, `0x80324a44`, `0x803276f8`.

No run now has more than **4** uncovered prologue targets. The five remaining
4-prologue runs are `0x803114b0`, `0x803228c0`, `0x80323b48`, `0x80327d38`,
`0x8032b000`.

## 4. Gap corpus

Batch 23 moved the wave-266 corpus from **271 → 283 covered / 396 → 384
eligible**. The next pick is the 128-byte `0x8004AB90`. The three deferrals
`0x800211B0`, `0x800D03A0`, `0x8002B260` remain deliberately unclaimed.

## 5. Audit lane F (wave-292 bodies + store class)

- **42 wave-292 bodies** (A 12, B 11, D 7, E 12; lane C added none) re-derived:
  single owner, recursive-descent extent equals documented extent, prologue and
  instruction counts, and `functions.tsv`/ledger membership. **0 failures**;
  **7,984 documented bytes**.
- Five wave-292 tables re-censused slot by slot: **57/57, 51/51, 10/10, 11/11,
  12/12** distinct and covered, headers `{0,0}`, each report's
  former-uncovered target present at its claimed slot. Constants
  `0x8034FB38`, `0x8030EFC0`, `0x8032269C` resolve as reported.
- Lane C warnings reproduced: 21 warnings, no blanket allows, 0 dead seeds at
  the 20 named sites, 0 double-paren `S_L2CAP` sites.
- Store census: the 20 wave-292 bodies with the most non-stack stores plus the
  three dropped-store anchors. **22 non-stack stores, 22 matched, 0 missing**;
  7 residuals in `fn_8010b890` are documented record-stride writes from a fixed
  `r28` base (the same word is written by `w110_fill_state_record` at
  `record+0`).
- Whole census: 42 bodies, 223 stores, 0 findings. `fn_8010b890`'s
  `stw r30,-0x53C0(r13)` matches, and the deletion self-test now **passes**
  (the checker was tightened to match the write role, not the bare offset).

## 6. Warnings

`cargo check`: **0 errors, 21 warnings** (unchanged; no new warnings from this
wave).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` 0 failing; `check-page-offsets` 63 pairs / 0 mismatches;
  `check-helper-calls` 0 refs; `check-source-integrity` 0 findings.
- `scripts/check.ps1` exit 0.
