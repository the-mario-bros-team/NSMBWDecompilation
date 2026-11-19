# Audit - Wave 231 (vtable-aware rescan, batch 4, evidence hygiene)

## 1. Backlog re-ranked: the tier list was mostly wrong

Wave 230 showed that several "unreferenced state tables" are vtable method
arrays whose base is two words earlier (`header {0,0}`). Re-running the scan
with a `base - 8` probe changes the ranking completely:

| Metric | Wave 227 | Wave 231 |
|---|---:|---:|
| Table-owned entries | 1,950 | 1,950 |
| ... dispatched through vtables | not measured | 1,690 in 560 runs |
| Call-site-only | 92 | 92 |
| Unreferenced | 415 | 415 |
| **Tier 1 / 2 / 3 runs** | **12 / 18 / 651** | **458 / 185 / 38** |
| Tier entries | - | 1,602 / 315 / 29 |

432 runs (1,283 entries, ~294,600 B) moved from tier 3 to tier 1 and 179 runs
(299 entries, ~66,544 B) from tier 3 to tier 2. Nothing entered tier 3.
Applying the *old* rule to today's tree gives 14/18/649, so the delta is not an
artefact of newly added code.

Corpus progress: **485 of the 2,457 entries now appear in `src`**, leaving
**1,972** (1,727 table-owned). New top-5: `0x80325428` (79/234),
`0x8032679C` `WM_PLAYER_VTABLE` (64/241), `0x803084D0` (43/223),
`0x80314068` `dEn_c` (41/281), `.ctors` (41/142).

Caveats: the scan is a snapshot (parallel lanes raise the `namedInSrc` count),
49 runs are `base-8`-named without a zero header and flagged `vtable?`, and 36
confirmed method arrays still have no reference at all.

## 2. Bodies translated

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `.ctors` slots 57-70 | 10 | ~11,000 |
| C | `.ctors` slots 71-84 | 10 | ~1,000 |
| D | WorldMap vtable `0x80325420`, slots 10-21 | 10 | 848 |
| E | `dEn_c` vtable `0x80314060`, slots 12-21 | 10 | 1,304 |

**40 bodies.** Slots already translated were skipped and reported throughout
(`.ctors` 71/79/80/81/62/67/68/70; world-map 11/16; `dEn_c` none).

Class/role identities recovered: `dEn_c` death states (`DieFumi`...`DieOther`),
`dEnJrClownBase_c` (8 states), `dEnTorideKokoopa_c` (28 states, 5,784 B),
`dEn_c`'s deleting destructor, the Shift-JIS `"アクター"` name getter, a
font-record owner, `Wm_en_quicksand`'s descriptor, and the WorldMap pickup gate.

## 3. Extent-measurement lessons (third independent confirmation)

- `bctrl` must be treated as a **falling-through call**; a first-`blr` reading
  under-counts the world-map heap bodies by 39 and 10 instructions.
- `beqlr`-guarded leaf stubs must not be treated as returns.
- A naive scan can cross padding into the next body: world-map slot 5 is a
  4-byte branch, not 176 bytes.
- Recursive descent reproduces every adjacent `functions.tsv` size it touches
  (`.ctors` 47/54/55 matched 308/1,500/2,316 exactly).

## 4. Evidence hygiene

- **69 / 69** unprefixed `.../export/` citations fixed in
  `wave3-player.csv`, `wave4-player.csv`, `wave5-player.csv` (63 `main-gb`,
  6 `rel-d_basesNP`), preserving trailing notes byte-for-byte.
- One genuine broken path fixed in `src/game/bases.rs:213648`
  (`work/rel-d_basesNP/...` -> `work/ghidra/rel-d_basesNP/...`).
- Remaining class, not yet fixed: **3,125 rows** use a bare
  `"Ghidra <addr>_FUN_<addr>.c"` style with no directory.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `merge-ledger` OK; 20,250 rows, 0 duplicates.
- `scripts/check.ps1` exit 0.
