# Audit - Wave 253 (size split resolved, duplicate owners found)

## 1. `0x801406F0`: 828 B, and the whole `main`/`main-gb` split characterised

The body ends at `bclr` `0x80140A28`, so `0x80140A2C - 0x801406F0 = 0x33C = 828`.
The `main` project's 612 B stops mid-ladder at `0x80140954` because its stock
PowerPC language cannot decode the `psq_l`/GQR angle reads.

| Comparison | Rows |
|---|---:|
| `main` / `main-gb` | 9,518 / 10,276 |
| only in `main-gb` | 762 (none only in `main`) |
| shared, different size | **549** |
| descent == `main-gb` | **533** |
| descent == `main` | **0** |

Of the 16 exceptions, 13 are `main-gb` rows that omit a trailing `_restgpr`
epilogue (4-32 B short); `0x800757B0` is over-read by `main`;
`0x80241690` is a discontiguous body; `0x802C2700` is cut short by both tables
(true extent 2,048 B).

## 2. Three more `addi` page errors fixed

Lane F's page sweep found **0 new errors from waves 250-252** but three real ones
from earlier waves in `archive.rs`:

| Constant (renamed) | Was | Correct |
|---|---|---|
| `TABLE_802FD198_F240` | `0x8030D198` | `0x802FD198` |
| `TABLE_802FD1B0_F240` | `0x8030D1B0` | `0x802FD1B0` |
| `TABLE_802FD240_F246` | `0x8030D240` | `0x802FD240` |

The third also had a **wrong `// observed:` six-word list** copied from the wrong
page; the image holds `0.0f, 0.5f, 0.0f, 0, 0, 0`, now documented with the
`lis`/`lwzu` arithmetic.

## 3. Duplicate owners: 58 addresses

A repo-wide scan for `pub unsafe fn fn_<addr>` in more than one file found
**58 duplicates** - several re-introduced since the wave-234 dedupe, because
later lanes kept reaching the same shared bodies through different vtables:

- the ten `0x802D8050..0x802D81D0` bodies exist in both `archive.rs` and
  `enemies.rs` (waves 253 D and E resolved the same `0x80329958` slots);
- the `0x8001D1xx`/`0x800645F0`/`0x80162xxx` families exist in
  `enemies.rs`/`worldmap.rs`/`player.rs` combinations.

One canonical owner per address is the project rule, and the duplicate cannot be
caught by `cargo check` across modules. Round 254 deduplicates all 58.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | die-state vtable 74-160 | 10 | 1,252 |
| C | `dEn_c` 249-258 | 10 | 1,720 |
| D | `0x803502F8` band 0x803507F4+ | 10 | 268 |
| E | vtable `0x80329958` words 15-25 | 12 | ~2,600 |

Coverage: die-state 167/235, `0x80329958` 29/45, band 237/249.

## 5. Findings

- `slw` registers print in the wrong order in wave 252's `d_dis.mjs`; the mask in
  `0x802DC1D0` is `slw r4,r0,r6` (destination `r4`). No other lane-E body uses
  `slw`.
- `0x80329958`'s word 39 (`0x8016D180`) was reported as the only *tabled* slot
  without a body (256 B in both tsvs). **Wave 254 lane F corrected this**: the
  slot is covered by the semantic body `ui::lyt::layout_register`
  (`src/ui/lyt.rs:1675`, ledger row `ui::lyt,8016d180,layout_register`), which
  the wave-253 name-only scan (`fn_<target>`) could not see. At the wave-253
  snapshot the table therefore had **15** uncovered slots, not 16; wave 254 lanes
  have been closing them since (`0x803502F8` lane D took words 26-35 through the
  shared sibling lattice). A repo-wide re-run of the name-only scan at the end
  of wave 254 finds 1,189 tabled rows without a `fn_<addr>` name; 1,187 of them
  are semantic-named bodies with a ledger row and the remaining 2 are the
  documented `game::bases` deferrals, so no ledger row is missing anywhere.
  (Before lane E added the name-shaped `fn_8016d180` delegation the class was
  1,190 / 1,188.)

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- Ledger 20,250 rows: 20,226 translated / 22 partial / 2 stub.
