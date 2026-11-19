# Wave 227 - lane B: size-mismatch adjudication

Lane B covers 15 rows across nine historical waves. Each row was adjudicated by
disassembling the shipped body in `work/dol/main.elf` (or the matching REL for
`rel-*` rows), measuring the `stwu` prologue at the claimed address to the final
`blr`, and comparing that true extent against the `# Original` byte count in
`src/` and the `size` column of `functions.tsv`.

Method: `work/`-local `ppc159.js`-style decoder reading the ELF program
headers, printing every instruction up to and including the last `blr`.
In-body indirect calls (`bcctr`) and forward branches were followed, so the end
of the body is the final `blr` rather than the first terminator.

## Result

All 15 rows landed on the same verdict: **the `functions.tsv` size is right and
the `# Original` doc figure was wrong**. In every case the measured
prologue->`blr` extent equals the tsv size exactly, with no prologue or
jump-table artefact, so the doc was corrected to the tsv value and a
`# Extent:` bullet recording the measured `blr` address was added to each
`# Original` block. No lane CSV row changed (the CSVs never carried a size
column), and `functions.tsv` was not touched.

| Wave | Address | Module | Doc before | TSV | Real (prologue->`blr`) | Final `blr` | Action |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 74 | `80ae8ef0` | game::enemies | 48 | 64 | 64 | `0x80AE8F2C` | doc fixed 48 -> 64 |
| 74 | `80ae90c0` | game::enemies | 172 | 152 | 152 | `0x80AE9154` | doc fixed 172 -> 152 |
| 75 | `800b90c0` | game::stage | 44 | 92 | 92 | `0x800B9118` | doc fixed 44 -> 92 |
| 75 | `800b94c0` | game::stage | 92 | 100 | 100 | `0x800B9520` | doc fixed 92 -> 100 |
| 75 | `800baac0` | game::stage | 48 | 88 | 88 | `0x800BAB14` | doc fixed 48 -> 88 |
| 83 | `80077d80` | game::stage | 148 | 168 | 168 | `0x80077E24` | doc fixed 148 -> 168 |
| 88 | `80af9ef0` | game::enemies | 268 | 212 | 212 | `0x80AF9FC0` | doc fixed 268 -> 212 |
| 90 | `802bd1e0` | resource::archive | 116 | 104 | 104 | `0x802BD244` | doc fixed 116 -> 104 |
| 92 | `80261e70` | resource::archive | 296 | 320 | 320 | `0x80261FAC` | doc fixed 296 -> 320 |
| 92 | `80267e30` | resource::archive | 296 | 272 | 272 | `0x80267F3C` | doc fixed 296 -> 272 |
| 95 | `802bd370` | resource::archive | 56 | 116 | 116 | `0x802BD3E0` | doc fixed 56 -> 116 |
| 95 | `802bef00` | resource::archive | 132 | 332 | 332 | `0x802BF048` | doc fixed 132 -> 332 |
| 98 | `80727220` | game::bases | 100 | 148 | 148 | `0x807272B0` | doc fixed 100 -> 148 |
| 98 | `801da1f0` | runtime::dvd | 272 | 212 | 212 | `0x801DA2C0` | doc fixed 272 -> 212 |
| 104 | `80288390` | render::gx_band | 460 | 252 | 252 | `0x80288488` | doc fixed 460 -> 252 |

Notes on the less obvious bodies:

- `80ae90c0` has a `bcctr` at `0x80AE9138` (virtual call, returns in place) and
  continues to `0x80AE9154`; stopping at the first terminator would undercount
  and was not done.
- `802bd370` has an in-body `bcctr` at `0x802BD3BC` before the final `blr` at
  `0x802BD3E0`; likewise followed through.
- `80077d80` has two `bcctr` virtual destroys inside the loop body; the extent
  ends at `0x80077E24`.
- `801da1f0` uses `X31_954` (`extsb`/`extsh` family operand form) bytes that the
  shared decoder prints as a raw mnemonic; the flow to `0x801DA2C0` is
  unaffected.
- The `rel-*` rows (`80ae8ef0`, `80ae90c0`, `80af9ef0`, `80727220`) were read
  from `work/rels/elf/` (synthetic base `0x80A00000` / REL load addresses), not
  `main.elf`; the extents match their REL `functions.tsv` entries.

## Verification

- `node tools/check-lane-csv.mjs wave74|75|83|88|90|92|95|98|104`: none of the
  15 addresses is reported for a byte-size mismatch any more. The remaining
  failures in those waves are the pre-existing "already in ledger/functions.csv"
  notices and other lanes' rows (for example wave74 `80ae7e90`, lane A).
  Problem counts before -> after: wave74 116 -> 114, wave75 116 -> 113; the
  other touched waves were clean of lane-B size problems already and are
  unchanged.
- `node tools/doc-lint.mjs src`: `ok: true`, 68 files, 0 violations.
- `cargo check`: exits 0 (`Finished dev profile`), warnings only, all
  pre-existing.

## Files touched

- `src/game/enemies.rs` - three `# Original` blocks (wave74, wave74, wave88).
- `src/game/stage.rs` - four blocks (wave75 x3, wave83).
- `src/game/bases.rs` - one block (wave98).
- `src/resource/archive.rs` - five blocks (wave90, wave92 x2, wave95 x2).
- `src/runtime/dvd.rs` - one block (wave98).
- `src/render/gx_band.rs` - one block (wave104).
- No `ledger/agents/*.csv` row needed a change; no Rust body text changed, only
  `///` doc comments.
