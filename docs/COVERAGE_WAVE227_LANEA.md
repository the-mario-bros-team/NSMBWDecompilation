# Wave 227 lane A - size-mismatch adjudication

Sixteen historical waves had an `# Original` byte size in the doc that differed
from `functions.tsv`. Each row was measured in the shipped image: `main.dol`
rows against `work/dol/main.elf`, REL rows against the matching
`work/rels/elf/<module>.elf` (`d_enemiesNP.elf` for `0x80A...`,
`d_basesNP.elf` for `0x806...`/`0x807...`). Measurement = linear sweep from the
entry point to the first closing control transfer that is followed by alignment
padding, the next known entry, or the next prologue. Every one of the sixteen
real extents equals the `functions.tsv` value; **no tsv size is a prologue or
jump-table artefact**. In each case the doc number was wrong and was corrected
to the tsv value, with an `Extent:` line added under `# Original` recording the
prologue-to-transfer range and the closing instruction.

| # | address | module | doc (before) | tsv | real extent | closing transfer | action |
|---:|---|---|---:|---:|---:|---|---|
| 1 | `0x80A1C0E0` | `game::enemies` | 132 | 124 | 124 | `blr` `0x80A1C158` | doc fixed to 124 + extent line |
| 2 | `0x80A2D2D0` | `game::enemies` | 60 | 76 | 76 | `blr` `0x80A2D318` | doc fixed to 76 + extent line |
| 3 | `0x80A39DE0` | `game::enemies` | 108 | 308 | 308 | `blr` `0x80A39F10` | doc fixed to 308 + extent line |
| 4 | `0x8026FB20` | `resource::archive` | 48 | 88 | 88 | `blr` `0x8026FB74` | doc fixed to 88 + extent line |
| 5 | `0x800E2340` | `game::scene` | 68 | 128 | 128 | `blr` `0x800E23BC` | doc fixed to 128 + extent line |
| 6 | `0x80A9B870` | `game::enemies` | 168 | 268 | 268 | `blr` `0x80A9B978` | doc fixed to 268 + extent line |
| 7 | `0x80746510` | `game::bases` | 48 | 72 | 72 | `blr` `0x80746554` | doc fixed to 72 + extent line |
| 8 | `0x80AC1D50` | `game::enemies` | 180 | 192 | 192 | `blr` `0x80AC1E0C` | doc fixed to 192 + extent line |
| 9 | `0x80108070` | `game::worldmap` | 280 | 128 | 128 | `blr` `0x801080EC` | doc fixed to 128 + extent line |
| 10 | `0x800B3780` | `game::stage` | 40 | 32 | 32 | `b 0x80157310` `0x800B379C` | doc fixed to 32 + extent line |
| 11 | `0x80693EE0` | `game::bases` | 32 | 24 | 24 | `bcctr` `0x80693EF4` | doc fixed to 24 + extent line |
| 12 | `0x80780F50` | `game::bases` | 92 | 68 | 68 | `blr` `0x80780F90` | doc fixed to 68 + extent line |
| 13 | `0x80654580` | `game::bases` | 92 | 128 | 128 | `blr` `0x806545FC` | doc fixed to 128 + extent line |
| 14 | `0x80219BE8` | `runtime::bt` | 196 | 380 | 380 | `blr` `0x80219D60` | doc fixed to 380 + extent line |
| 15 | `0x8021FD44` | `runtime::bt` | 64 | 224 | 224 | `blr` `0x8021FE20` | doc fixed to 224 + extent line |
| 16 | `0x80AE7E90` | `game::enemies` | 64 | 96 | 96 | `blr` `0x80AE7EEC` | doc fixed to 96 + extent line |

No lane CSV was edited: every ledger row's address/name/status was already
correct; only the doc's extent figure disagreed, and it was the wrong side of
the comparison in all sixteen cases.

## Evidence notes by row

* **1-3 (`0x80A1C0E0`, `0x80A2D2D0`, `0x80A39DE0`)**: REL constructors. `1` is
  the `0x828`-byte Black Pakkun allocator (early `blr` at `0x80A1C158`, one
  zero word of padding before the next constructor at `0x80A1C160`); `2` the
  minimal `0x528` Bounce Ball constructor; `3` the `0x9A60` Cliff Chikuwa
  constructor, whose `__savegprlr_20` prologue at `0x80A39DF0` explains the
  long 308-byte body the doc had truncated to 108.
* **4 (`0x8026FB20`)**: `main.dol` block-list release; the 88-byte body ends in
  `blr` at `0x8026FB74`, with the next entry (`0x8026FB90`) after padding.
* **5 (`0x800E2340`)**: scene sound dispatch; the internal `b 0x800E23A8` is a
  real branch inside the body, not a tail call, and the body closes at
  `0x800E23BC`.
* **6 (`0x80A9B870`)**: slot toggle with two internal forward branches
  (`0x80A9B8C4`, `0x80A9B940`); the export's 268 bytes are one function.
* **7 (`0x80746510`)**: timer countdown with an early `blr` at `0x80746524`
  and a second arm that reads `+0x3D8` and closes at `0x80746554`. Both arms
  are inside the same 72-byte function.
* **8 (`0x80AC1D50`)**: `mecha_koopa` step; full `__savegprlr_20` prologue and
  epilogue, 192 bytes to `blr` at `0x80AC1E0C`.
* **9 (`0x80108070`)**: state-controller init; 128 bytes, not 280 (the 280
  figure came from a larger neighbouring Ghidra entry).
* **10 (`0x800B3780`)**: 32-byte forwarder; the conditional `bnelr` at
  `0x800B3790` is followed by the tail `b 0x80157310`, so the tsv extent is 32,
  not 40.
* **11 (`0x80693EE0`)**: 24-byte virtual thunk (`lwzu`/`mtctr`/`bcctr`).
* **12 (`0x80780F50`)**: ring-slot writer, 68 bytes.
* **13 (`0x80654580`)**: ground-surface test with an internal `b 0x806545E8`
  and a 128-byte body ending at `0x806545FC`.
* **14 (`0x80219BE8`)**: RFCOMM credit-limit setup; the three forward branches
  at `0x80219C50`, `0x80219C98` and `0x80219CCC` all land on the shared
  epilogue at `0x80219CF4`, so the body is the full 380 bytes.
* **15 (`0x8021FD44`)**: SDP record append; 224 bytes, with the early-out
  branch at `0x8021FD68` and two internal forward branches.
* **16 (`0x80AE7E90`)**: 96-byte attack-mask test (`0x802DD1EC` + two global
  words), closing at `0x80AE7EEC`.

## Verification

* `node work/wave227/laneA_verify.mjs` - doc value vs `functions.tsv` for all
  sixteen addresses: all `OK`.
* `node tools/check-lane-csv.mjs <wave>` for `wave3`, `wave7`, `wave31`,
  `wave48`, `wave55`, `wave62`, `wave65`, `wave66`, `wave68`, `wave74` - zero
  `byte size ... in doc, ... in functions.tsv` problems in every wave. (These
  historical waves still report their pre-existing already-merged /
  `0x`-prefixed / evidence-path complaints, which are out of this lane's scope
  and were not introduced by these edits.)
* `node tools/doc-lint.mjs src` - `ok: true`, 0 violations.
* `cargo check` - succeeds (exit 0). Only doc comments changed; no Rust body
  text was touched.
