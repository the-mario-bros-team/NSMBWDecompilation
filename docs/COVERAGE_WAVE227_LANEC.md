# Adjudication - Wave 227 lane C (15 size mismatches)

Fifteen rows where the `# Original` byte count in the Rust doc disagreed with
the `size` column of `functions.tsv`. Every row was measured against the image
independently; in **all fifteen the tsv is right and the doc was wrong**, so
each doc block was corrected to the tsv value and given an `- Extent:` line
naming the epilogue instruction.

## Method

`work/dol/main.elf` only maps `0x80006780..0x802ED9E0`, so the two DOL rows
(`0x80133950`, `0x800214F0`) were read from it and the thirteen REL rows from
the matching `work/rels/elf/*.elf`
(`d_basesNP.elf` `0x806000F0..0x807C60F4`,
`d_enemiesNP.elf` `0x80A000F0..0x80B2A518`).

Extent = union of every instruction reachable from the entry by falling
through and by following internal branches, terminated at a `blr`, a
`bctr`/`bclr` with `LK=0` (tail dispatch), or an unconditional `b` leaving the
function (tail call). Traps avoided:

* a `bcctr` with `LK=1` is an **indirect call**, not an epilogue - stopping
  there under-counts `0x80761260` (96 vs 168), `0x8062C360` (260 vs 328) and
  `0x80A19A70` (268 vs 352). These three are exactly the rows where a naive
  linear "first `blr` or `bcctr`" scan contradicted the tsv;
* unconditional `b` inside the body (`0x8076107C`, `0x80638F48`) is a switch
  dispatch, not the end, so its target was followed.

Cross-check: in every row the measured end lands exactly on the tsv size and no
later tsv start overlaps it (0-byte gap for five rows, 4-80 bytes of alignment
padding for the rest).

## Rows

`doc` = byte count before the fix, `tsv` = `functions.tsv`, `real` = measured
extent, `end` = last instruction of the body.

| Wave | Address | Module | doc | tsv | real | end | Action |
|---|---|---:|---:|---:|---:|---|---|
| 104 | `0x80133950` | worldmap | 76 | 136 | **136** | `0x801339D4` `blr` | doc 76 -> 136 |
| 114 | `0x80760F40` | bases | 168 | 176 | **176** | `0x80760FEC` `blr` | doc 168 -> 176 |
| 114 | `0x80761040` | bases | 220 | 544 | **544** | `0x8076125C` `blr` | doc 220 -> 544 (event-dispatch arms + shared tail are inside) |
| 114 | `0x80761260` | bases | 132 | 168 | **168** | `0x80761304` `blr` | doc 132 -> 168 |
| 116 | `0x80A19A70` | enemies | 92 | 352 | **352** | `0x80A19BCC` `blr` | doc 92 -> 352 |
| 116 | `0x80A1CEA0` | enemies | 112 | 136 | **136** | `0x80A1CF24` `blr` | doc 112 -> 136 |
| 123 | `0x80A296E0` | enemies | 228 | 284 | **284** | `0x80A297F8` `blr` | doc 228 -> 284 |
| 125 | `0x8072D8F0` | bases | 180 | 128 | **128** | `0x8072D96C` `blr` | doc 180 -> 128 (doc over-counted) |
| 128 | `0x80A85610` | enemies | 100 | 64 | **64** | `0x80A8564C` `blr` | doc 100 -> 64 (doc over-counted) |
| 160 | `0x807571C0` | bases | 32 | 28 | **28** | `0x807571D8` tail `b` | doc 32 -> 28 |
| 166 | `0x807C2970` | bases | 60 | 64 | **64** | `0x807C29AC` `blr` | doc 60 -> 64 |
| 168 | `0x8062C360` | bases | 324 | 328 | **328** | `0x8062C4A4` `blr` | doc 324 -> 328 |
| 168 | `0x807C25B0` | bases | 300 | 328 | **328** | `0x807C26F4` `blr` | doc 300 -> 328 |
| 192 | `0x80638F00` | bases | 252 | 248 | **248** | `0x80638FF4` `blr` | doc 252 -> 248 |
| 203 | `0x800214F0` | hud | 128 | 136 | **136** | `0x80021574` `blr` | doc 128 -> 136 |

Notable cases:

* `0x80761040` (220 recorded) and `0x80A19A70` (92 recorded) were the largest
  under-counts - 324 and 260 bytes missing respectively. Both bodies contain
  indirect `bcctr` calls whose following instructions belong to the function,
  so the old doc stopped mid-body.
* `0x8072D8F0` and `0x80A85610` are over-counts (180 vs 128, 100 vs 64): the
  docs included trailing alignment/neighbour bytes.
* `0x807571C0` is a pure tail-call thunk: 28 bytes, ending in `b 0x800B3980`.

No lane CSV row changed: the `address`/`evidence`/`rust_file` columns were
already correct, and the tsv size was already right, so the only edit was in
`src/`.

## Verification

* `node tools/check-lane-csv.mjs wave{104,114,116,123,125,128,160,166,168,192,203}`
  - **0** `byte size ... in doc, ... in functions.tsv` problems in all eleven
  waves. The command still exits non-zero for these waves because of the
  separate, pre-existing `is already in ledger/functions.csv` rule: lane CSVs
  of merged historical waves remain on disk, and that rule (not the size rule)
  accounts for every remaining message, including rows this lane never
  touched.
* `node tools/doc-lint.mjs src` - `ok: true`, 68 files, 0 violations.
* `cargo check` - exit 0 (26 pre-existing warnings, none in the edited doc
  blocks; no Rust body text changed).
