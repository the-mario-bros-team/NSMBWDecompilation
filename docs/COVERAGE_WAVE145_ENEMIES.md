# Coverage - Wave 145 (`game::enemies`)

## How to read this file

This page records the Wave 145 `game::enemies` lane: 15 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 127 and mirrored
one row per body in `ledger/agents/wave145-enemies.csv`. Each `evidence` cell
names the Ghidra C export the body was reconstructed from; the byte counts are
the `size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` with the segment shift of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`), and
every body was followed from its entry to `blr`, a tail `b` or a tail call.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 15 | 9652 | 11 | 4 |

Status split: 15 `translated` (11 high, 4 medium), no `partial` and no `stub`.

Covered address range: `0x80A673D0..0x80B147D0` (6516 bytes of
`docs/SPEC_WAVE144_ENEMIES.md` rows and 3136 bytes of `0x80AC0000` planner-bucket
rows).

## What this pass covers

- **Six of the nine ready `docs/SPEC_WAVE144_ENEMIES.md` rows.**
  `0x80AE2250` (the `qr3` wing step: one unsigned halfword widen, the
  `1/256` pre-scale, the `0xC00` step and the `1.5` amplitude), `0x80A85650`
  (the one-shot resolver: the `+0xBEC`/`+0xDFF` guard, the `+0xDCA == 6` and
  `== 2` consumptions, the six `(state, kind)` arms, SE `0x116` and both hit
  values), `0x80B147D0` (the four `daEnTogeTekkyu_c` variants with the shared
  `0x802EE810` descriptor and the eleven-word animation block),
  `0x80A673D0` (the `g3d/goal_set.brres` binder: three characters, the
  `0x2^52 + 2^31` frame conversion and five animation holders),
  `0x80B0C900` (the `g3d/teresa.brres` binder: A, B and fog sets) and
  `0x80AAE520` (the fifteen-segment ring constructor with its `{1,0,2,3}` side
  table and three `0x91804100` record arms).
- **One conditional specification row, `0x80AA0C20`.** The approach latch, the
  `16.0`/`55.0` arming pair, SE `0x1D6`/`0x1D3`, the `112.0` drop clear and the
  four-clip ladder over `0x80B74648`/`0x80B74688`/`0x80B746C8`/`0x80B74788`.
  It stays `medium` because those four clip objects are BSS slots filled by the
  constructor, so their names are outside the body.
- **Eight `0x80AC0000` planner-bucket rows.** Three of them are siblings of the
  wave-134 learner block (`0x80ACDEF0`, `0x80ACDCC0`, `0x80ACE900`): the
  `10.0 / |0.8 * +0xE8|` count divisor, the two walk-clip selectors and the
  `0x80B6AB9C` pointer-table bind. Five are the surrounding learner steps:
  `0x80ACCE20` and `0x80ACCC00` (the `264.0` x clamp, the ground block and the
  `0x80B76178`/`0x80B76138` token switches), `0x80ACEE40` (the doubled turn
  step, the `0x4000`/`-0x4000` boundary and the landing clip), `0x80AC9870`
  (the breathe countdown plus the haze/float/breath descriptor tables and the
  `5500.0` vector component) and `0x80ACC8F0` (the co-op follower step with SE
  `0x1E7`/`0x1E8`, the `0x80ACADD0` clip switch and the `0x15 << facing` flag
  mask).

### Export corrections made while porting

The specification page is accurate on the whole, but four places in the Ghidra
exports disagree with the raw stream and the raw stream won:

- **`0x80B147D0` tail.** `lfs f0,+0xAC` / `stfs f0,+0x238` is a **float copy**,
  not a pointer store; the spec's "position pointer" note is wrong.
- **`0x80B0C900` A-set pattern.** The pattern name is `0x80B67F80` = `"wait"`,
  not `"teresaA"`; the `teresaA` string is the character name only.
- **`0x80A673D0` opener.** `0x800690C0` receives `r3..r7` only; the export's
  `double` first argument is the live `f1` bias word, not a parameter.
- **`0x80ACEE40` tables.** The `lhax` sources are `0x8042C1A8` (per-facing step)
  and `0x8042B540` (per-slot limit angle); the export prints `+0x5D0` for both.
- **`0x80AC9870` names.** `0x80B69D08`/`0x80B69D18`/`0x80B69D28` are sixteen-word
  function-pointer tables, not name strings.

### Why four rows are `medium`

- **`0x80AA0C20`** - the flow and every token address are literal, but the four
  `0x80B746xx` objects are BSS slots whose clip names only the constructor
  knows.
- **`0x80ACDEF0`** - the divisor arithmetic is explicit, but the `+0x14` record
  token and the `0x8042C1A0` byte table are external. The original `divw` has
  no zero guard; the port keeps 0 there and marks it `// unknown:`.
- **`0x80AC9870`** - the `+0xA0` slot contract of the effect holders is not
  established anywhere in the module.
- **`0x80ACC8F0`** - the `0x80B76078` token and the partner `+0x5D7` clip
  semantics are external.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A673D0` | `fn_80a673d0` | 1080 | translated | high |
| `0x80A85650` | `fn_80a85650` | 976 | translated | high |
| `0x80AA0C20` | `fn_80aa0c20` | 872 | translated | medium |
| `0x80AAE520` | `fn_80aae520` | 1156 | translated | high |
| `0x80AC9870` | `fn_80ac9870` | 464 | translated | medium |
| `0x80ACC8F0` | `fn_80acc8f0` | 652 | translated | medium |
| `0x80ACCC00` | `fn_80accc00` | 428 | translated | high |
| `0x80ACCE20` | `fn_80acce20` | 368 | translated | high |
| `0x80ACDCC0` | `fn_80acdcc0` | 304 | translated | high |
| `0x80ACDEF0` | `fn_80acdef0` | 252 | translated | medium |
| `0x80ACE900` | `fn_80ace900` | 296 | translated | high |
| `0x80ACEE40` | `fn_80acee40` | 372 | translated | high |
| `0x80AE2250` | `fn_80ae2250` | 108 | translated | high |
| `0x80B0C900` | `fn_80b0c900` | 1140 | translated | high |
| `0x80B147D0` | `fn_80b147d0` | 1184 | translated | high |

## Deliberate skips

- **`0x80A3DEA0` (2940 bytes, rank 1).** The 16-case jump table at `0x80B3EE94`
  is in the image, but the body is 735 instructions of constructor state writes
  plus a per-state animation selection; reproducing it byte-faithfully needs a
  second full pass over the table that this lane did not have. Left open rather
  than approximated.
- **`0x80B04E90` (1592 bytes, rank 2).** The two-pass polyline analyser has 44
  conditional branches and two unrolled byte-fill arms. The export's block
  structure is exact, but the run-length emission index arithmetic
  (`min(ax,ay)` pairs then the remainder) could not be cross-checked against the
  raw stream within this pass.
- **`0x80A7BEC0` (1464 bytes, rank 3).** The clip-name stride table
  (`0x24`/`0x12`/`6`/`2` over the `0x80B2DE38` bit bytes) needs the byte table
  read out of the image; the six vtable `bctrl` slots were not all verified, so
  the body was left for the next tranche.
- **`0x80A9B3C0` (1196 bytes, rank 4).** Conditional: the `slot` argument is
  staged only by the indirect caller, and four effect-band callees are known by
  use alone.
- **`0x80A92F40` (1172 bytes, rank 6).** Conditional: it consumes three
  unledgered `main.dol` helpers (`0x8004A7B0`, `0x80056CF0`, `0x80064D10`)
  whose contracts are not in the module.
- **`0x80AC8850` (304 bytes, bucket).** The export is a broken `unaff_GQR0`
  frame; the raw stream takes an incoming `f1` scale, so its argument list can
  only be established from the caller, which is not in the export.
- **`0x80A337B0` (4 bytes).** Still the `b 0x80A337C0` alias with no body of its
  own.
- The `0x80AC0000` bucket candidate list from the planner walk is now fully
  accounted for: eight of its nine export-backed addresses are in this pass and
  the ninth is `0x80AC8850`, skipped above. Any other untranslated address in
  the bucket has no `functions.tsv` export of its own.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 15 sizes sum to 9652
  bytes (6516 for the seven specification rows, 3136 for the eight bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 15 addresses were absent from `ledger/functions.csv`
  and from every earlier `fn_<addr>` body before this pass.
- The only `psq_l ...,qr3` in the new rows is the single halfword widen of
  `0x80AE2250`; every other `psq` is a `qr0` `f31`/`f30`/`f29` spill and is not
  ported.
- Every constant quoted in a section comment was read from
  `work/rels/elf/d_enemiesNP.elf` at the address shown (`0.8`/`10.0`,
  `{1,0,2,3}`, `112/64/16/48/32/512`, `{3648,-3648}`, `5500.0`).
- The module header was updated: cluster item 127, the cluster count
  (126 to 127), the function totals (3352 to 3367 public translations, 3281 to
  3296 `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` exits `0` with no error and no warning
  naming `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave145` reports 15 rows for this lane
  (`translated/high=11`, `translated/medium=4`), 15 unique addresses and `OK`.
