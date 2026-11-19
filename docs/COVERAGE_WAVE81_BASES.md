# Coverage - Wave 81 (`game::bases`, `0x8063` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** under the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers only
identify positions inside the REL image. Cross-module pointers
(`0x8037xxxx`, `0x8042xxxx`) are `main.dol` addresses imported by the REL and
keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. "Role" below is the
plain-English behaviour. Confidence is `high` when guards, stores, constants,
strings and call arguments are literal, `medium` when the role of a field,
global or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain. No body in this pass is `low`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 5,248 | 22 | 8 |

## What this pass covers

The `0x8063` bucket held 201 functions at lane start; 57 were already in
`ledger/functions.csv` when this pass began, leaving 144. This pass takes the
member tail of one results screen, `0x8063B680..0x8063D3F0`, whose state table
[`fn_8063d7a0`] was already translated.

The screen is the one whose layout loader is bound to
`pointResultMulti_18.brlyt` and whose handlers raise the descriptors
`0x80827020..0x80827460`. Those pointers sit inside the descriptor range that
[`fn_8063d7a0`] registers (`0x80826FA0..0x80827460`), which is why the handlers
are grouped with that class. The object layout used throughout is: `+0x70` the
NW4R layout, `+0x78` its vtable, `+0x208` the interface object (event slot
`0x18`), `+0x244`/`+0x248` the two sub-layout pane roots, `+0x24C` the root
pane, `+0x28C` the 10 x 42 score table (row stride `0xA8`), `+0x920`/`+0x924`
the frame counter and its limit, `+0x928` the result count, `+0x92C`/`+0x940`
the four rise countdowns and heights, `+0x93C` the layout style id and `+0x91C`
the "layout built" byte.

- **Layout and score table, `0x8063B680..0x8063BF30` (8 bodies).** The
  `pointResultMulti_18.brlyt` loader [`fn_8063b680`] (twelve `N_posFile1_00`,
  two `T_subjectS_00` and one `P_subjectBase_00` pane), the pane-table index
  [`fn_8063b8d0`] over `0x807C8018`, the frame counter [`fn_8063bba0`], the
  label style writer [`fn_8063bbd0`], the profile rank lookup
  [`fn_8063bc90`], the best-score scan [`fn_8063bd90`], the first-entry probe
  [`fn_8063bec0`] and the discount pass [`fn_8063bf30`] that keeps the probed
  entry and scales every other score at or below the threshold by `0x5F/100`.
- **Save data and profile probes, `0x8063C160..0x8063C440` (3 bodies).** The
  import [`fn_8063c160`] which advances the active profile's own cell by 100
  (clamped to 10000) and multiplies each imported score by 100, or by `0x5F`
  when the stored rank is worse; the five-entry presence test
  [`fn_8063c360`]; and the pane/animation gate [`fn_8063c440`].
- **Reveal state handlers, `0x8063C520..0x8063CA00` (10 bodies).** Small
  handlers that gate on one animation finishing or on the counter wrapping and
  then raise one descriptor: [`fn_8063c520`], [`fn_8063c620`],
  [`fn_8063c6c0`], [`fn_8063c780`], [`fn_8063c7f0`], [`fn_8063c840`],
  [`fn_8063c8a0`], [`fn_8063c8f0`], [`fn_8063c990`] and [`fn_8063ca00`].
- **Grouping, commit and write-back, `0x8063CA70..0x8063D3F0` (9 bodies).**
  The reveal grouping [`fn_8063ca70`] (both layout styles, sound `0x93`), the
  follow-up selector [`fn_8063cc40`], the rise reset [`fn_8063cd50`], the
  per-frame rise step [`fn_8063ce40`], two more counter handlers
  [`fn_8063cf80`]/[`fn_8063d040`], the confirm handler [`fn_8063d160`], the
  save-data write-back [`fn_8063d2b0`] and the flush gate [`fn_8063d3f0`].

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x8063B680` | `fn_8063b680` | 280 | high | `pointResultMulti_18.brlyt` loader and pane binding |
| `0x8063B8D0` | `fn_8063b8d0` | 36 | high | pane-table index over `0x807C8018` |
| `0x8063BBA0` | `fn_8063bba0` | 48 | high | frame counter with wrap |
| `0x8063BBD0` | `fn_8063bbd0` | 180 | high | result label style writer |
| `0x8063BC90` | `fn_8063bc90` | 252 | medium | profile pair rank lookup |
| `0x8063BD90` | `fn_8063bd90` | 300 | high | best-score scan |
| `0x8063BEC0` | `fn_8063bec0` | 100 | high | first entry at or below threshold |
| `0x8063BF30` | `fn_8063bf30` | 340 | high | discount pass, `0x5F/100` |
| `0x8063C160` | `fn_8063c160` | 252 | medium | score-table import from save data |
| `0x8063C360` | `fn_8063c360` | 212 | high | active profile presence test |
| `0x8063C440` | `fn_8063c440` | 212 | high | pane and animation gate |
| `0x8063C520` | `fn_8063c520` | 176 | medium | special-mode exit / player-count gate |
| `0x8063C620` | `fn_8063c620` | 88 | high | animation 0 idle handler |
| `0x8063C6C0` | `fn_8063c6c0` | 160 | high | paired / split layout split |
| `0x8063C780` | `fn_8063c780` | 88 | high | animation 1 idle handler |
| `0x8063C7F0` | `fn_8063c7f0` | 72 | high | animation 2 start and hold arm |
| `0x8063C840` | `fn_8063c840` | 80 | high | counter wrap to result reveal |
| `0x8063C8A0` | `fn_8063c8a0` | 76 | high | animation 2/3 stop and hold clear |
| `0x8063C8F0` | `fn_8063c8f0` | 112 | high | split layout to paired entry |
| `0x8063C990` | `fn_8063c990` | 80 | high | result card hold |
| `0x8063CA00` | `fn_8063ca00` | 88 | high | animation 6 idle handler |
| `0x8063CA70` | `fn_8063ca70` | 460 | medium | four-player reveal grouping |
| `0x8063CC40` | `fn_8063cc40` | 252 | high | follow-up state selector |
| `0x8063CD50` | `fn_8063cd50` | 236 | medium | rise reset for four records |
| `0x8063CE40` | `fn_8063ce40` | 288 | medium | per-frame rise step |
| `0x8063CF80` | `fn_8063cf80` | 80 | high | counter wrap handler |
| `0x8063D040` | `fn_8063d040` | 80 | high | second counter wrap handler |
| `0x8063D160` | `fn_8063d160` | 208 | high | confirm commit / discard |
| `0x8063D2B0` | `fn_8063d2b0` | 244 | medium | score write-back to save data |
| `0x8063D3F0` | `fn_8063d3f0` | 168 | medium | save flush gate and return |

## Address range

`0x8063B680..0x8063D3F0`, three runs:

- Run 1 is `0x8063B680..0x8063BF30`, with the deliberate hole at
  `0x8063B900`/`0x8063BA50` described below.
- Run 2 is `0x8063C160..0x8063CA00`; `0x8063C090` inside the surrounding range
  is deferred (see below) and `0x8063C440` is the run's last address before the
  `0x8063C520` handler gap.
- Run 3 is `0x8063CA70..0x8063D3F0`, with the two 48-byte destructor
  instantiations `0x8063D740`/`0x8063D770` left for a later pass.
- Everything else in the bucket is untouched: `0x80630820..0x80630BF0`,
  `0x806318F0..0x80631920`, `0x80632650..0x80634800`,
  `0x806360D0..0x80636AB0` (the destructor twins `0x806369D0..0x80636AB0` and
  the `pointResultDateFile` loader `0x80636CB0` are good next candidates),
  `0x8063A1D0..0x8063A2B0`, `0x8063A470..0x8063ABE0`,
  `0x8063D740..0x8063D770` and `0x8063E870..0x8063F820`.

## Deliberate skips

- `0x8063B900` (324 bytes) and `0x8063BA50` (324 bytes) are **not**
  translated. Both are the per-player placement writers and both start with a
  `GQR0` lane split (`unaff_GQR0 & 0x3f00`, `ldexpf` artefacts). The integer
  structure is readable, but the paired-single lanes are not recoverable from
  the export, so they stay as address-only calls (`abi_call1!(0x8063_B900, …)`
  and `abi_call1!(0x8063_BA50, …)`) from [`fn_8063ca70`], [`fn_8063cc40`],
  [`fn_8063cd50`] and [`fn_8063ce40`].
- `0x8063C090` (208 bytes) is deferred, not rejected: the score-table driver
  calls `0x8063BF30` with a single argument in the export
  (`FUN_8063bf30(iVar1);`) although the callee reads the threshold from `r4`,
  so the call would need an inferred argument. A later pass can settle that
  once the surrounding save-data path is reviewed.
- `0x8063D740`/`0x8063D770` are plain 48-byte destructor instantiations; they
  were left out only to stay inside the 30-body budget.
- Nothing else in this range was judged unreconstructable.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 5,248 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 57 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave81` reports `wave81-bases.csv: 30 rows`
  and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
