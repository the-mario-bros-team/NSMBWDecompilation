# Coverage - Wave 67 (WorldMap message loader and controller remainder)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 26 | 3572 | 14 | 12 |

## What this pass covers

- `0x80107860..0x80107C90` - the per-language message loader that precedes
  wave 55's controller. [`fn_80107860`] starts and polls the global load,
  [`fn_80107890`] resets the record and installs the 45-entry pointer table,
  [`fn_80107920`] copies fourteen dispenser words to `+0xA8` and rebuilds the
  three string columns, [`fn_801079e0`] stores a shared string or allocates and
  concatenates generated text, [`fn_801079b0`] returns one slot word,
  [`fn_80107ab0`] selects the generated columns, [`fn_80107ad0`] installs the
  `r13 - 0x65D0..-0x64B0` table, [`fn_80107c70`] clears the mode record at
  `+0xE0`, and [`fn_80107c90`] implements the full language/mode to index map.
- `0x80107F20..0x80107FC0` - the controller constructor and its display child.
  [`fn_80107f20`] clears `+0x04..+0x3E`, writes mode `5` at `+0x14` and calls
  [`fn_80107fc0`], which allocates `0x24`-byte elements, creates the display
  child from the canvas halfwords at `(*r13 - 0x5200) + 0x04/+0x06` and
  registers both callbacks.
- `0x801084E0..0x801092A0` - the poll/notification chain.
  [`fn_801084e0`] chooses pending state `2`/`3`/`4` from the two global tests,
  shifts the `+0x08`/`+0x0C`/`+0x10` history and runs the post-update records;
  [`fn_80108be0`]/[`fn_80108bf0`] register the `0x80108C00`/`0x80108C30`
  callbacks; [`fn_80108c00`]/[`fn_80108c30`] set the controller's `+0x1A`/
  `+0x1B` flags; [`fn_80108d80`] posts completion code `0x1E`; [`fn_80108e80`]
  cancels the request; [`fn_80108ef0`] reads the global load byte through
  `0x80107850`; and [`fn_801092a0`] frees the `r13 - 0x53D0` display object.
- `0x801076B0` - the load-state handler. It latches states `4`/`6`/`0xB` at
  `+0x1B4`, raises the load-book event at `(*r13 - 0x55F4) + 0x4C`, and drops
  back to idle after `0x13` frames without a load.
- `0x80106B40` - a piecewise curve sampler in the neighbouring effect cluster.
  It walks `0x14`-byte segments (`+0x00` point count, `+0x08` point array,
  `+0x0C` duration) and hands the normalized sample to `0x80106620`.
- `0x801051C0`, `0x80108CF0`, `0x80108F30` - three four-byte jump thunks. The
  Ghidra export prints the destination bodies; the Rust bodies keep only the
  branch, because the entry point itself is four bytes.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801051C0` | `fn_801051c0` | 4 | high |
| `0x80106B40` | `fn_80106b40` | 292 | medium |
| `0x801076B0` | `fn_801076b0` | 412 | medium |
| `0x80107860` | `fn_80107860` | 44 | high |
| `0x80107890` | `fn_80107890` | 132 | high |
| `0x80107920` | `fn_80107920` | 136 | medium |
| `0x801079B0` | `fn_801079b0` | 40 | medium |
| `0x801079E0` | `fn_801079e0` | 196 | medium |
| `0x80107AB0` | `fn_80107ab0` | 28 | high |
| `0x80107AD0` | `fn_80107ad0` | 404 | medium |
| `0x80107C70` | `fn_80107c70` | 28 | high |
| `0x80107C90` | `fn_80107c90` | 560 | high |
| `0x80107F20` | `fn_80107f20` | 152 | high |
| `0x80107FC0` | `fn_80107fc0` | 168 | medium |
| `0x801084E0` | `fn_801084e0` | 376 | medium |
| `0x80108BE0` | `fn_80108be0` | 12 | high |
| `0x80108BF0` | `fn_80108bf0` | 12 | high |
| `0x80108C00` | `fn_80108c00` | 48 | high |
| `0x80108C30` | `fn_80108c30` | 48 | high |
| `0x80108C60` | `fn_80108c60` | 136 | medium |
| `0x80108CF0` | `fn_80108cf0` | 4 | high |
| `0x80108D80` | `fn_80108d80` | 244 | medium |
| `0x80108E80` | `fn_80108e80` | 32 | medium |
| `0x80108EF0` | `fn_80108ef0` | 8 | high |
| `0x80108F30` | `fn_80108f30` | 4 | high |
| `0x801092A0` | `fn_801092a0` | 52 | medium |

## Deliberate skips

- `0x80106620`, `0x801068B0`, `0x80107170`, `0x80107460`, `0x80101AA0` and
  `0x80102DB0` were skipped: each export carries a `unaff_GQR0` /
  `ldexpf` reconstruction block, so the paired-single scale factors are not
  recoverable from the C. Wave 60 and Wave 64 skipped the same record class.
- `0x80107050` was skipped: its export is a variadic stack-string builder that
  reads `CR1` and eight stack arguments, so the real parameter list is not
  recoverable from the C.
- `0x80103410` (16 bytes) was skipped: the export shows only
  `FUN_80103440()` with every argument dropped, so a forwarding signature would
  have been invented.
- `0x80107860..0x80107C90` is reached from the same overlay as the controller,
  but the export does not name the class; the docs therefore keep literal
  offsets and do not claim a subsystem name.
- The remaining untranslated entries of the `0x80100000` bucket belong to the
  `0x801072F0..0x80107410` engine globals, the `0x80109350..0x8010A350`
  HomeButton2 resource set (its exports name `HomeButton2_*` strings) and the
  `0x8010A3E0..` groups; they are outside this lane's message/controller runs.

## Verification

- Every address and byte size matches `functions.tsv`; the 26 sizes sum to
  3,572 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 26 were untranslated
  entries of the `0x80100000` bucket).
- `cargo check --message-format=short` is free of errors and warnings for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave67` reports `OK` for this lane's CSV.
