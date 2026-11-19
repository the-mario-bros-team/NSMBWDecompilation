# Coverage - Wave 144 (`game::enemies`)

## How to read this file

This page records the Wave 144 `game::enemies` lane: 17 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 126 and mirrored
one row per body in `ledger/agents/wave144-enemies.csv`. Each `evidence` cell
names the Ghidra C export the body was reconstructed from; the byte counts are
the `size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` with the segment shift of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`), and
every body was checked instruction by instruction with its control flow followed
to `blr`, a tail `b` or a tail call.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 17 | 2924 | 13 | 4 |

Status split: 17 `translated` (13 high, 4 medium), no `partial` and no `stub`.

Covered address range: `0x80A03C90..0x80B23480`.

## What this pass covers

- **The `docs/SPEC_WAVE143_ENEMIES.md` ready tranche (`0x80A40CA0..0x80B23480`).**
  The `daEnCoin_c` water bob (`0x80A40CA0`: pitch `0x400` step, the
  `water >= y` predicate, the `0x8042A100 + 0x8C0` drift, `4.0` margin and
  amplitude from one word, `0x400` bob step), the contact push `0x80A79810`
  (`2.0 * amount` on the target's y, the `+0xEC` clear, the two
  `0x8013AAA0`-linked arms and the `0x80A79750` tail), the
  `daEnIbaramushi_c` antiphase pair `0x80A7C600`, the `4`/`4`/`32` offset point
  `0x80AA3F50`, the `daEnKopone_c` state step `0x80AB21D0`, the
  radius/heading publisher `0x80ABA950`, the mode-driven heading step
  `0x80AFD170` with its `s16[4]` table, and the `block_pata` pair
  `0x80ADDA40`/`0x80ADDAC0` (spring centre plus the `16.0` commit, including the
  double advance of `+0x78C`).
- **The two conditional rows (`0x80AEC4E0`, `0x80AEEFA0`).** The 32-entry
  table-walk twins that rebuild the `+0x5C4` and `+0x544` record lists: the
  iterator from `0x80168400`, the count from `0x8023B8C0`, the null
  substitution when the object's vtable slot 2 rejects the `0x802F8FF0`
  functor, the `0x8025C660` activation record, the `0x8025C510`/`0x8023D120`/
  `0x8023D170` reduction and the `{ element + 0x3F0, 0xFFFFFFFF }` descriptor
  handed to `0x8023DC10`. The `extraout_r4` of the export is the documented `r4`
  (`or r25,r4,r4` at `0x80AEC4F4`); `r3` is dead on entry and the port drops it.
- **Five `0x80A00000` planner-bucket rows (`0x80A03C90..0x80A073C0`).** The
  heading/pitch half-extent pair `0x80A03C90` (three `qr3` widenings, a `16.0`
  amplitude on two of them and a bare sine on the third, plus the `fneg`), the
  normalised direction rescale `0x80A04BF0` (turn by `fn_80a04940`, in-place
  normalise through `0x80170A00`, scale by `f1` into `+0xE8/+0xEC`, `+0xF0` from
  the `0.0` constant, then `0x8006CD40`), the free-with-count helper
  `0x80A05F40`, and the two twin constant producers `0x80A073B0`/`0x80A073C0`
  that materialise `0x80377A04`.

### Why four rows are `medium`

- **`0x80A03C90`** - every trig call, the `16.0` amplitude, the `fneg` and both
  store offsets are literal, but nothing in the module reads `+0x5D8`/`+0x5DC`,
  so the pair is documented by its arithmetic only.
- **`0x80A04BF0`** - the turn, the normalise, the three multiplies and the two
  stores are literal; the roles of `+0x670` (source pair) and
  `+0xE8..+0xF0` (scaled direction) are inferred from their use as a vector
  triple.
- **`0x80AEC4E0` / `0x80AEEFA0`** - the loops, the null substitution, the
  `0x3F0` bias and the descriptor stores are literal, but `0x8023D170`,
  `0x8023DC10` and the functor word `0x802F8FF0` are unledgered `main.dol`
  symbols, so the meaning of the 8-byte descriptor is not established. The
  bodies stay faithful call-for-call and mark each of the three `// unknown:`.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A03C90` | `fn_80a03c90` | 212 | translated | medium |
| `0x80A04BF0` | `fn_80a04bf0` | 176 | translated | medium |
| `0x80A05F40` | `fn_80a05f40` | 64 | translated | high |
| `0x80A073B0` | `fn_80a073b0` | 12 | translated | high |
| `0x80A073C0` | `fn_80a073c0` | 12 | translated | high |
| `0x80A40CA0` | `fn_80a40ca0` | 208 | translated | high |
| `0x80A79810` | `fn_80a79810` | 360 | translated | high |
| `0x80A7C600` | `fn_80a7c600` | 184 | translated | high |
| `0x80AA3F50` | `fn_80aa3f50` | 200 | translated | high |
| `0x80AB21D0` | `fn_80ab21d0` | 176 | translated | high |
| `0x80ABA950` | `fn_80aba950` | 156 | translated | high |
| `0x80ADDA40` | `fn_80adda40` | 120 | translated | high |
| `0x80ADDAC0` | `fn_80addac0` | 124 | translated | high |
| `0x80AEC4E0` | `fn_80aec4e0` | 292 | translated | medium |
| `0x80AEEFA0` | `fn_80aeefa0` | 292 | translated | medium |
| `0x80AFD170` | `fn_80afd170` | 156 | translated | high |
| `0x80B23480` | `fn_80b23480` | 180 | translated | high |

## Deliberate skips

- **`0x80AE2250` (108 bytes, `qr3`).** The 13th row of the `docs/SPEC_WAVE143_ENEMIES.md`
  selection and the last `qr3` class-D row; the specification page explicitly
  leaves it for the next tranche.
- **`0x80B07D70` (252 bytes).** Already has a complete `pub unsafe fn fn_80b07d70`
  body in this file (wave 142), so it is not untranslated.
- **`0x80A337B0` (4 bytes).** A `b 0x80A337C0` alias with no body of its own.
- **`0x80A07C80` (16 bytes) of the bucket.** The Ghidra export is a single
  `halt_baddata()`. The raw stream does decode (`addi r5,r3,0x710`,
  `li r4,0`, `addi r3,r3,0x5B4`, `b 0x80168180`), but the tail branch leaves the
  REL for `main.dol` and the three staged arguments cannot be cross-checked
  against a callee contract, so no body was written.
- **Ten further `0x80A00000` bucket rows** are left open: `0x80A00D00` (424),
  `0x80A03C30` (92), `0x80A03FF0` (708), `0x80A04A90` (348), `0x80A04D10` (360),
  `0x80A05000` (616), `0x80A05DF0` (116), `0x80A06220` (212), `0x80A06560` (500)
  and `0x80A0F610` (660). `0x80A03C30` and `0x80A05DF0` recover their receiver
  from a CodeWarrior save helper, and `0x80A00D00`/`0x80A03FF0` mix `psq`
  prologue saves of `f28..f31` with the body's own paired-single traffic, so a
  port would have to guess part of the register state.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 17 sizes sum to 2924
  bytes (2448 for the twelve specification rows, 476 for the five bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 17 addresses were absent from `ledger/functions.csv`
  and from every earlier `fn_<addr>` body before this pass.
- Every `psq_l ...,qr3` in the specification rows is one `(f32)(u16)` widen; the
  `qr0` prologue pairs are `f64` spills and are not ported.
- Every constant quoted in a section comment was read from
  `work/rels/elf/d_enemiesNP.elf` at the address shown, including the `s16[4]`
  table at `0x80B31778` (raw bytes `fc 00 08 00 fa 00 04 00`).
- The module header was updated: cluster item 126, the cluster count (125 to
  126), the function totals (3335 to 3352 public translations, 3264 to 3281
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` exits `0` with no error and no warning
  naming `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave144` reports 17 rows for this lane
  (`translated/high=13`, `translated/medium=4`), 17 unique addresses and `OK`.
