# Coverage - Wave 122 (`game::enemies`, the `0x80AE0000` bucket tail)

## How to read this file

This page records the Wave 122 `game::enemies` lane: 28 functions spread over
three runs of the `0x80AE0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 103 of that file. Every row of the machine
readable mirror (`ledger/agents/wave122-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 5,604 | 10 | 17 |

The bucket holds 160 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 104 were already in
`ledger/functions.csv`, leaving 56. This pass claims 28 of them, all in the
three runs below, and leaves 28 untouched.

One row is `low` (`0x80AEB3D0`, a probe predicate whose output role the export
does not establish) and four rows are `partial` (the `0x802DCBFC`
state-destroying-destructor trampolines, whose hidden `r12` this-adjust pointer
`platform::call3` cannot model - the same limitation recorded for the copies in
waves 56, 72, 75, 80, 81, 82, 97, 98, 99 and 101).

## What this pass covers

- **The five remaining `daEnPuku_c` per-frame handlers
  (`0x80AE8140..0x80AE8E40`, 5 bodies).** Section 54 built the `0xEC8`-byte
  water enemy; this pass fills the holes its state machine left. `0x80AE8140`
  and `0x80AE8780` are the two chase/return steps sharing the `+0xEAE` phase
  byte (0 = idle, 1 = contact, 2 = return): the first steers with the
  normalised `contact - position` vector scaled by `0.75`, snaps at `2.0` and
  keeps the `13.0` band; the second biases the vertical leg by `64.0` and
  scales by `0.5`. `0x80AE8440` is the escape step behind the `+0xEC0` latch
  and enters `EscapeSwim` (`0x80B77450`) or `HellSrcRetur` (`0x80B77290`);
  `0x80AE8610` counts the three "motion finished" conditions and enters
  `SrcSwim` (`0x80B77210`) or `LrSwim` (`0x80B77390`); `0x80AE8E40` is the
  cosine turn-swim step. The state records are the ones
  [`register_puku_states`] names, so no record had to be guessed.
- **The afterimage-trail emitter (`0x80AEA590..0x80AEB3D0`, 9 bodies).** One
  actor keeps sixteen effect handles at `+0x3D0`, with the next free slot at
  `+0x828`. `0x80AEAB90` spawns one class-`0x9F` effect
  (`0x80064680(0x9F, actor, flags, point, 0, actor[+0x38F])`), switching the
  flag word between `0x10000000`/`0x10001000` on `+0x824` and adding
  `0x20000000` on `+0x820`. `0x80AEAC20`/`0x80AEACE0` expand one point into a
  `step`-spaced run (every step, or every other step); `0x80AEADB0` fans the
  samples over the seventeen signed spacings at `0x80B31158` (`0, +-32, ...,
  +-256`) with `64.0` vertical runs; `0x80AEB0D0`/`0x80AEB200` are the two
  diagonal fans (`+32.0` and `-32.0` per sample on `y`). `0x80AEB330` retires
  the slots whose object `0x80162D00` no longer resolves and destroys the
  owner when all are gone; `0x80AEB3D0` is the `0x80064790` probe predicate.
  `0x80AEA590` is the class's deleting destructor.
- **The `0x730`-byte actor shell and the follow-controller helpers
  (`0x80AEB880..0x80AECD10`, 14 bodies).** `0x80AEBA30` is the constructor
  behind the `0x730`-byte allocation: it installs the class vtable
  `0x80B61270` at `+0x60`, builds the `+0x528`/`+0x540` and `+0x564`/`+0x57C`
  animation-holder pairs through two `0x8015FC90` calls, the `+0x5A4` matrix,
  the `+0x5C4` clip holder and the `+0x604` object. Around it sit the four
  destruction paths (`0x80AEB880`, `0x80AEBB90`, `0x80AEBBD0`, `0x80AEBC30`,
  which release the `+0x18` sub-object through `0x8015F9F0`), the allocator
  `0x80AEBA00` and four `0x802DCBFC` trampolines. The block closes with the
  follow-controller helpers: `0x80AEC610` rebuilds the four-player latch block
  `+0x722..+0x729` and its timers `+0x70C..+0x718` from the `+0x610` child
  chain, `0x80AEC7B0` searches the class-`0x1E9` effect records for the one
  whose `+0x4` slot nibble matches and enters `0x80B77610`, `0x80AEC850`
  recomputes the `+0x700..+0x708` offset triple and `0x80AEC8E0` applies it.
- **Constants.** Every float table used here was read out of
  `work/rels/elf/d_enemiesNP.elf` and is listed once in the section header of
  the Rust file, including the `0.5`/`-0.5` per-side speed tables
  (`0x80B30F18`, `0x80B30F48`), the `1.0`/`-1.0` direction table
  (`0x80B310F8`) and the spacing table `0x80B31158`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AE8140` | `fn_80ae8140` | 716 | medium |
| `0x80AE8440` | `fn_80ae8440` | 352 | medium |
| `0x80AE8610` | `fn_80ae8610` | 324 | high |
| `0x80AE8780` | `fn_80ae8780` | 620 | medium |
| `0x80AE8E40` | `fn_80ae8e40` | 172 | medium |
| `0x80AEA590` | `fn_80aea590` | 92 | high |
| `0x80AEAB90` | `fn_80aeab90` | 144 | high |
| `0x80AEAC20` | `fn_80aeac20` | 192 | medium |
| `0x80AEACE0` | `fn_80aeace0` | 200 | medium |
| `0x80AEADB0` | `fn_80aeadb0` | 284 | medium |
| `0x80AEB0D0` | `fn_80aeb0d0` | 300 | medium |
| `0x80AEB200` | `fn_80aeb200` | 300 | medium |
| `0x80AEB330` | `fn_80aeb330` | 148 | high |
| `0x80AEB3D0` | `fn_80aeb3d0` | 96 | low |
| `0x80AEB880` | `fn_80aeb880` | 88 | high |
| `0x80AEB9A0` | `fn_80aeb9a0` | 48 | medium (partial) |
| `0x80AEB9D0` | `fn_80aeb9d0` | 48 | medium (partial) |
| `0x80AEBA00` | `fn_80aeba00` | 48 | high |
| `0x80AEBA30` | `fn_80aeba30` | 280 | medium |
| `0x80AEBB90` | `fn_80aebb90` | 64 | high |
| `0x80AEBBD0` | `fn_80aebbd0` | 96 | high |
| `0x80AEBC30` | `fn_80aebc30` | 100 | high |
| `0x80AEC610` | `fn_80aec610` | 384 | medium |
| `0x80AEC7B0` | `fn_80aec7b0` | 152 | medium |
| `0x80AEC850` | `fn_80aec850` | 116 | high |
| `0x80AEC8E0` | `fn_80aec8e0` | 144 | medium |
| `0x80AECCE0` | `fn_80aecce0` | 48 | medium (partial) |
| `0x80AECD10` | `fn_80aecd10` | 48 | medium (partial) |

## Deliberate skips

- **The other 28 addresses of the bucket were not attempted.** In ascending
  order they are: `0x80AE0780` (248), `0x80AE2250` (108), `0x80AE4A50` (660),
  `0x80AE63D0` (388), `0x80AE69E0` (444), `0x80AEAED0` (284),
  `0x80AEAFF0` (216), `0x80AEB450` (700), `0x80AEC4E0` (292),
  `0x80AECF00` (88), `0x80AED170` (64), `0x80AED1B0` (96),
  `0x80AED210` (100), `0x80AEDA60` (48), `0x80AEDA90` (48),
  `0x80AEDB90` (76), `0x80AEDC90` (48), `0x80AEDAC0` (208),
  `0x80AEDD40` (1,208), `0x80AEE6D0` (88), `0x80AEE7F0` (48),
  `0x80AEE820` (48), `0x80AEE850` (296), `0x80AEEA30` (124),
  `0x80AEEFA0` (292), `0x80AEF130` (1,180), `0x80AEF910` (88) and
  `0x80AEFCF0` (600). Nothing was written for them, so no ledger row claims
  them.
- **`0x80AEAED0` and `0x80AEAFF0` are skipped on purpose.** Both are members of
  the trail block, but their exports take their inputs through the
  paired-single `f31`/`ps31` registers (`in_f31`, `in_ps31_1`) and then store
  the quantised pair into a stack slot that is never read, while the body
  itself only uses the actor recovered by `0x802DCD74`. Whether the original
  consumes those registers cannot be decided from the export, so no body was
  written rather than guessing.
- **`0x80AEB450` is skipped.** It is the largest fan writer of the same block;
  its export combines nine `GQR0`/`GQR3` prologue artefacts with a mode switch
  over `+0x810`, and the three branches each mix the `2^52` bias trick with
  the unread pair slots. The visible stores are recoverable but the mode
  selection depends on values the export no longer types, so it was left for a
  pass that can read the instruction stream.
- **`0x80AEC4E0` is skipped.** It recovers its receiver through `0x802DCD6C`
  and then uses `extraout_r4` - a register the decompiler no longer defines -
  as both the owner and the loop bound. Reconstructing it would require
  inventing the missing `r4` value.
- **`0x80AECF00`, `0x80AED170`, `0x80AED1B0` and `0x80AED210` are not claimed
  here.** They are the `daEnRailPolyChild_c` state callback and destructor
  copies; two of them are byte-identical to bodies in this pass, and they were
  left to the pass that owns the rest of that class so the same shape is not
  documented twice under different field names.
- **No address already in `ledger/functions.csv` was touched.** In particular
  the eleven `daEnPuku_c` helpers referenced by this pass
  ([`fn_80ae6560`], [`fn_80ae65c0`], [`fn_80ae71a0`], [`fn_80ae7240`],
  [`fn_80ae76c0`], [`fn_80ae7810`], [`fn_80ae78a0`], [`fn_80ae79e0`],
  [`fn_80ae7bf0`], [`fn_80ae7ca0`], [`fn_80ae7e00`], [`fn_80ae7f10`] and
  [`fn_80ae7b00`]) keep their existing rows.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  5,604 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave122` reports the lane CSV clean.
- `cargo check --message-format=short` emits no error naming
  `src/game/enemies.rs`.
- The module header was updated: cluster item 103, the cluster count (one
  hundred and two to one hundred and three), the function totals (2,755 to
  2,783 public translations, 2,684 to 2,712 `fn_<addr>` bodies), the "start
  here" list and the records list.
