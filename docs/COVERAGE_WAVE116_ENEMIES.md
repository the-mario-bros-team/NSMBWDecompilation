# Coverage - Wave 116 (`game::enemies`, the `0x80A10000` bucket)

## How to read this file

This page records the Wave 116 `game::enemies` lane: 28 functions of the
`0x80A10000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 97 of that file. Every row of the machine
readable mirror (`ledger/agents/wave116-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 4,308 | 18 | 10 |

## What this pass covers

- **`0x80A19A70..0x80A1A3A0`, the `Wm_en_birikyu` world-map enemy.** Ten
  functions of one actor. `0x80A19A70` is the model binder: it opens
  `g3d/bilikyu.brres`, resolves the `bilikyu` node three times (model at
  `+0x540`, matrix holder at `+0x544`, animation holder at `+0x5BC`) and starts
  the `body_wait` clip on `+0x584`. `0x80A19DF0` rebuilds the matrix from the
  `+0xAC..+0xB4` position plus the `+0x724` bob offset and the
  `+0x100..+0x104` angles, then commits it to `+0x544` together with the
  reference vector at `+0xDC`. `0x80A19EE0` is the triangle-wave bob stepper
  (`+0x720` counter, bit `0x20` selects the direction, `0x80B2B500` is the
  magnitude). `0x80A19F30` copies animation matrix `0` of `+0x544` to `+0x5E8`.
  The remaining five are the `0x1CE` retire path against the `0x80358004`
  (`InIceLump`) state, the contact callback, the `body_wait`/`body_stop` clip
  entries and the `+0x71C` countdown that leaves the state.
- **`0x80A1C790..0x80A1DAD0`, one `0x730`-byte actor of the same bucket.**
  Eighteen functions. The state word is `+0x800`, the variant word `+0x804`,
  the four player slot bytes are `+0x66C..+0x66F` and the swing angle is
  `+0x81A`. The run covers the `0x80B2B5E0` byte-table getter, the turn/limit
  step (with the `0x237` sound and the `+0xEC` bounce), the four
  `0x80064610` message senders (`0x3C`, `0x191`, `0xB0`, `0x1F1`) with their
  `0x80B2B678`/`0x80B2B688`/`0x80B2B670` offset tables, the `+0x80C` hit
  counter drain, the `0x80077860` stage-position selector over the
  `0x80B2B690` id table, the `+0x800` dispatcher `0x80A1D130` and its two
  wrapper steps, the `+0x81A` swing stepper and the `0x730`-byte allocator
  `0x80A1DAD0` (vtable `0x80B387B8`, holders at `+0x534` and `+0x698`).

The literal names are the only identity evidence: `g3d/bilikyu.brres`,
`bilikyu`, `body_wait`, `body_stop`, `Wm_en_birikyu` and `Wm_en_burst_s`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A19A70` | `fn_80a19a70` | 92 | high |
| `0x80A19BD0` | `fn_80a19bd0` | 264 | high |
| `0x80A19DF0` | `fn_80a19df0` | 236 | high |
| `0x80A19EE0` | `fn_80a19ee0` | 68 | high |
| `0x80A19F30` | `fn_80a19f30` | 76 | high |
| `0x80A1A150` | `fn_80a1a150` | 176 | high |
| `0x80A1A200` | `fn_80a1a200` | 120 | high |
| `0x80A1A290` | `fn_80a1a290` | 112 | high |
| `0x80A1A300` | `fn_80a1a300` | 140 | high |
| `0x80A1A3A0` | `fn_80a1a3a0` | 140 | medium |
| `0x80A1C790` | `fn_80a1c790` | 16 | high |
| `0x80A1C7A0` | `fn_80a1c7a0` | 336 | medium |
| `0x80A1CB70` | `fn_80a1cb70` | 92 | high |
| `0x80A1CBD0` | `fn_80a1cbd0` | 264 | medium |
| `0x80A1CCE0` | `fn_80a1cce0` | 136 | high |
| `0x80A1CD70` | `fn_80a1cd70` | 120 | high |
| `0x80A1CDF0` | `fn_80a1cdf0` | 164 | medium |
| `0x80A1CEA0` | `fn_80a1cea0` | 112 | high |
| `0x80A1CF30` | `fn_80a1cf30` | 124 | high |
| `0x80A1CFB0` | `fn_80a1cfb0` | 40 | high |
| `0x80A1CFE0` | `fn_80a1cfe0` | 336 | medium |
| `0x80A1D130` | `fn_80a1d130` | 508 | medium |
| `0x80A1D330` | `fn_80a1d330` | 52 | medium |
| `0x80A1D4F0` | `fn_80a1d4f0` | 52 | medium |
| `0x80A1D600` | `fn_80a1d600` | 116 | medium |
| `0x80A1D690` | `fn_80a1d690` | 212 | medium |
| `0x80A1D950` | `fn_80a1d950` | 88 | high |
| `0x80A1DAD0` | `fn_80a1dad0` | 116 | high |

The ten `medium` rows are medium for one of three reasons, each stated in the
body:

- **The export prints a call without its receiver.** Six bodies call a helper
  whose body declares an `int param_1` but whose call site names no argument
  (`0x80A19EE0` in `0x80A1A3A0`, `0x80A1C7A0` in `0x80A1D690`, `0x80A1CBD0` and
  `0x80A1CDF0` in `0x80A1D130`, `0x80A1CFE0` in `0x80A1D330`/`0x80A1D4F0`,
  `0x80A1D530` in `0x80A1D600`/`0x80A1D690`). `r3` still holds the actor at
  each site, so `this` is passed and the inference is marked `// inferred:`
  in the body - the same treatment as `fn_80a50780` and `fn_80a53e70` in
  earlier waves.
- **A table-fed id or offset whose meaning is not established.** `0x80A1CBD0`,
  `0x80A1CDF0`, `0x80A1CFE0` and `0x80A1C7A0` read the `0x80B2B650`,
  `0x80B2B670`, `0x80B2B690` and `0x80B2B624` tables; the reads, the shifts and
  the masks are literal, the semantic name of each id is not.
- **The callee is one of the two blocked helpers of this bucket**
  (`0x80A1C8F0`, `0x80A1D530`), which are called by address.

## Deliberate skips

The bucket had 62 untranslated addresses before this pass and 34 after it.
The neighbours this pass examined and rejected are:

- **`0x80A1C8F0` (636 bytes).** The whole body sits between the mutex pair
  `0x802DCD6C` / `0x802DCDB8`, and the decompiled `func_0x802dcd6c()` returns
  the actor in the *high* half of an untyped 64-bit result while the low half is
  used as a selector flag. Neither the locked object nor the meaning of the two
  halves is recoverable, exactly like the mutex skips of waves 113, 114 and 115.
- **`0x80A1D530` (208 bytes).** A GQR conversion block (`unaff_GQR3`,
  `ldexpf`) turns the `+0x818`/`+0x81A` angle words into the `+0xAC`/`+0xB0`
  position, and the same two conversions are emitted twice; the quantisation
  mode register is not printed. This is the same reason five addresses were
  skipped in `docs/COVERAGE_WAVE113_ENEMIES.md` and three in
  `docs/COVERAGE_WAVE115_ENEMIES.md`.
- **`0x80A1DA70` (48 bytes), `0x80A1DAA0` (48 bytes).** Both are two-argument
  forwarders to the mutex stub `0x802DCBFC` whose visible call is exact but
  whose hidden third operand is not modelled; they are the same shape as the
  forwarders recorded in waves 13, 114 and 115 and would have to be shipped
  `partial`. They are left for the lane that finally models `0x802DCBFC`.
- **Not attempted in this pass.** `0x80A10220` (484), `0x80A10880` (940),
  `0x80A12110` (8), `0x80A18730` (24), `0x80A18750` (24), `0x80A18770` (28),
  `0x80A18E30` (68), `0x80A197F0` (48), `0x80A19820` (48), `0x80A1B2E0` (20),
  the twelve-step block `0x80A1E100..0x80A1EF30` and the ten-step block
  `0x80A1F390..0x80A1FFB0` are all readable, but the lane budget was spent on
  the two coherent clusters above; they are the natural start of the next pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  4,308 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave116` prints `wave116-enemies.csv: 28 rows`
  and `OK` for this lane.
- `cargo check --message-format=short` emits no error and no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 97, the cluster count
  (ninety-six to ninety-seven), the function totals (2,584 to 2,612 public
  translations, 2,513 to 2,541 `fn_<addr>` bodies), the "start here" list and
  the records list.
