# Coverage - Wave 140 (`game::enemies`)

## How to read this file

This page records the Wave 140 `game::enemies` lane: 30 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 122 and mirrored
one row per body in `ledger/agents/wave140-enemies.csv`. Each `evidence` cell
names the Ghidra C export the body was reconstructed from; the byte counts are
the `size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` with the segment shift of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`). The
Ghida exports for these rows truncate at every out-of-image `b`/`bl`
(`halt_baddata`), so all callee addresses were resolved from the REL's module-0
`R_PPC_REL24` import stream instead of from the raw branch displacement; the
addends match `docs/SPEC_WAVE139_ENEMIES.md` for every spec row.

The lane has three parts:

- **12 bodies** are the ready rows of `docs/SPEC_WAVE139_ENEMIES.md` (ranks
  1-12; rank 5 is the specification's single conditional row).
- **9 bodies** are the remaining class-B rows listed at the end of that
  specification.
- **9 bodies** are still-open rows of the `0x80A50000` planner bucket: one
  `+0x102` facing step and eight `0x802DCBFC` deleting-destructor forwarders.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 872 | 17 | 13 |

Status split: 22 `translated` (17 high, 5 medium) and 8 `partial` (medium).

## What this pass covers

- **The two side steppers.** `0x80AD0A90` reads the `+0x348` side byte, loads the
  `s16` at `0x80B30850 + 2*side` (`{-5461, +5461}` = ±30° in 16-bit turn units)
  with `lhax` and steps `+0x8A8` by `0x400`. `0x80B1B0E0` is the same shape over
  the eight-entry `0x80B32488` pivot table, stepping `+0x102` by `0x800`. Both
  tail-branch to the ledgered `0x8015F4F0`.
- **The row-2/band-1 rumble thunk `0x80A18770`.** No incoming register is read:
  the body overwrites `r3` with `*(u32*)0x8042A008` and tail-branches to
  `0x800D8C60(object, 2, 1, 0, 0)`.
- **The two object-message emitters.** `0x80A40FA0` builds
  `((slot & 0xFF) << 12) | 9` and passes it with kind `0x191` and the position
  record as the third factory word; `0x80A443A0` is the `0x192` position lookup
  with three zero arguments.
- **The `+0x1080` sound-stop guard `0x80A9F750`.** `lbz`/`extsb`/`cmpi 7` with
  `bltlr`, then the tail call to `0x800CE8C0(*(u32*)0x80429FB0)`. `r3` is never
  read, so the port keeps `_self` for the ABI and threads `other` as the second
  argument (the specification's conditional datum).
- **The bit-0 ground-probe dispatcher `0x80AA0080`.** Bit 0 of `+4` selects
  `0x80097170` (clear) or `0x80097250` (set); `f1`, `r3` and `r4` are forwarded
  unchanged, and the dead `cmpli 1`/`bnelr` pair is kept as the `bit != 1`
  return.
- **The two `+0x10C` float steppers.** `0x80A18730` passes `f1 = 2.0`,
  `f2 = 0.1`; `0x80A18750` passes `1.25`/`1.25`. The constants are the shipped
  words `0x40000000`, `0x3DCCCCCD` and `0x3FA00000` at
  `0x80B2B438..0x80B2B444`.
- **The `+0x616` retire guard `0x80A1B2E0`.** The
  `xori 0xFFFF`/`cntlzw`/`rlwinm` triple is the `marker == 0xFFFF` boolean, and
  the mode word is passed to `0x80064D10`.
- **The per-frame base select `0x80A2D460`.** `lbz` of `+0x666` against `2`
  chooses `0x80021600` or `0x800214F0`; the load is unsigned as in the words.
- **The `0x81`/`0x82` spawn pair.** `0x80AAA3E0` and `0x80AAA480` are
  word-identical except for the kind byte: `mr r5,r3` makes the first caller
  word the factory's third argument, `r4` stays the second.
- **Three descriptor-word producers.** `0x80A2F730` returns `0x80357C34`,
  `0x80A2F740` returns `0x80357BF4` and `0x80A4EC90` returns `0x80358084`.
- **Five tiny tails.** `0x80A3F6D0` returns while `+0xA90` is zero and otherwise
  tail-calls `0x800279F0`; `0x80A12110`/`0x80A7E630` pass `this + 0x98C` and
  `this + 0x5FC` to `0x800944C0`; `0x80AB1A00` unregisters `this + 0x7C0`
  through the ledgered `0x8008C380`; `0x80ADA6E0` updates `this + 0x548`
  through the ledgered `0x8007FDA0`.
- **The bucket facing step `0x80A5D9D0`.** `+0x634` selects a four-byte row of
  `0x80B2D190` (`{{+1820, -1820}, {+8192, -8192}}`), `+0x348` the per-side
  halfword at `+2*side`; a zero row index is the "no target" case that steps to
  `0`. Both paths step `+0x102` by `0x400`.
- **Eight `0x802DCBFC` forwarders.** `0x80A5B180`, `0x80A5BB30`,
  `0x80A5D2D0` and `0x80A5F7B0` use the `+0x18` adjustor;
  `0x80A5B1B0`, `0x80A5BB60`, `0x80A5D300` and `0x80A5F7E0` use `+0x24`. Each
  is twelve words: `stwu`, `mflr`, `mr r5,r3`, `mr r3,r4`, `addi r12,r5,N`,
  `bl 0x802DCBFC`, `nop`, epilogue.

### Confirmations and corrections worth recording

- **Import-stream callee confirmation.** Every out-of-image tail target in this
  pass was read from the module-0 relocation list: `0x8015F4F0`,
  `0x8015F470`, `0x800D8C60`, `0x80064610`, `0x80064D10`, `0x800CE8C0`,
  `0x80097170`, `0x80097250`, `0x80021600`, `0x800214F0`, `0x800279F0`,
  `0x800944C0`, `0x8008C380`, `0x8007FDA0` and `0x802DCBFC`. The raw branch
  fields in the REL point at unrelated in-image words, which is exactly why the
  Ghidra exports show `halt_baddata`.
- **`0x80B2D190` is a row table, not a plain pivot list.** The first two rows are
  `{+1820, -1820}` and `{+8192, -8192}`; the following words are floats
  (`1.0`, `0.0`, `12.0`, ...). `0x80A5D9D0` therefore special-cases a zero
  `+0x634` and otherwise reads `4*index + 2*side`, which the port keeps
  literally.
- **`0x80A1B2E0`'s Ghidra output is lossy.** The export prints a bare
  `countLeadingZeros(...)` and drops the `rlwinm 27,24,31` that turns it into
  `0`/`1`; the words `0x6800FFFF`, `0x7C000034`, `0x5404DE3E` are the exact
  boolean materialisation.
- **`0x80AA0080`'s `cmpli`/`bnelr` is dead.** Bit 0 can only be `0` or `1`, so
  the re-test never returns early; the port keeps it as documentation of the
  stream.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A12110` | `fn_80a12110` | 8 | translated | medium |
| `0x80A18730` | `fn_80a18730` | 24 | translated | high |
| `0x80A18750` | `fn_80a18750` | 24 | translated | high |
| `0x80A18770` | `fn_80a18770` | 28 | translated | high |
| `0x80A1B2E0` | `fn_80a1b2e0` | 20 | translated | high |
| `0x80A2D460` | `fn_80a2d460` | 20 | translated | high |
| `0x80A2F730` | `fn_80a2f730` | 12 | translated | high |
| `0x80A2F740` | `fn_80a2f740` | 12 | translated | high |
| `0x80A3F6D0` | `fn_80a3f6d0` | 16 | translated | medium |
| `0x80A40FA0` | `fn_80a40fa0` | 28 | translated | high |
| `0x80A443A0` | `fn_80a443a0` | 24 | translated | high |
| `0x80A4EC90` | `fn_80a4ec90` | 12 | translated | high |
| `0x80A5B180` | `fn_80a5b180` | 48 | partial | medium |
| `0x80A5B1B0` | `fn_80a5b1b0` | 48 | partial | medium |
| `0x80A5BB30` | `fn_80a5bb30` | 48 | partial | medium |
| `0x80A5BB60` | `fn_80a5bb60` | 48 | partial | medium |
| `0x80A5D2D0` | `fn_80a5d2d0` | 48 | partial | medium |
| `0x80A5D300` | `fn_80a5d300` | 48 | partial | medium |
| `0x80A5D9D0` | `fn_80a5d9d0` | 68 | translated | medium |
| `0x80A5F7B0` | `fn_80a5f7b0` | 48 | partial | medium |
| `0x80A5F7E0` | `fn_80a5f7e0` | 48 | partial | medium |
| `0x80A7E630` | `fn_80a7e630` | 8 | translated | medium |
| `0x80A9F750` | `fn_80a9f750` | 28 | translated | medium |
| `0x80AA0080` | `fn_80aa0080` | 28 | translated | high |
| `0x80AAA3E0` | `fn_80aaa3e0` | 20 | translated | high |
| `0x80AAA480` | `fn_80aaa480` | 20 | translated | high |
| `0x80AB1A00` | `fn_80ab1a00` | 8 | translated | high |
| `0x80AD0A90` | `fn_80ad0a90` | 32 | translated | high |
| `0x80ADA6E0` | `fn_80ada6e0` | 8 | translated | high |
| `0x80B1B0E0` | `fn_80b1b0e0` | 32 | translated | high |

### Why eight rows are `partial`

The eight `0x802DCBFC` forwarders pass `(param_2, param_2, param_1)` exactly,
but the original also loads `r12 = param_1 + {0x18|0x24}` as a hidden
this-adjust pointer. `platform::call3` cannot model a fourth, non-ABI register
operand, so the adjustor is documented and left out. This is the same limitation
the wave-135/136/137/138/139 forwarders record.

### Why four `translated` rows are `medium`

- **`0x80A9F750`** - the load, signed compare and tail target are exact; only
  the caller-side name of `r4` is inferred (the specification's conditional
  row).
- **`0x80A3F6D0`** - the guard and the import addend `0x800279F0` are exact; the
  callee has no export, so its role is not established.
- **`0x80A12110` / `0x80A7E630`** - the offsets and the import addend
  `0x800944C0` are exact; the callee has no export.
- **`0x80A5D9D0`** - the seventeen words are exact; the role of `+0x634` as a
  row index is inferred from the `*4` scaling and the two populated rows.

## Deliberate skips

Rows of the `0x80A50000` bucket that were left for a dedicated pass rather than
padded in:

- `0x80A5AEF0` (12) - a second producer of the constant `0x80357BF4`; it adds no
  new behaviour beyond `fn_80a2f740` and was left so the lane could cover the
  forwarder cluster instead.
- `0x80A53850` (368), `0x80A56620` (736), `0x80A56920` (476), `0x80A56B10` (672),
  `0x80A584E0` (232), `0x80A589A0` (144), `0x80A58D70` (308), `0x80A5C660`
  (436), `0x80A5DBC0` (212), `0x80A5E1A0` (136), `0x80A5E590` (252) and
  `0x80A5EB00` (448) - the model/animation and state-machine bodies of the
  bucket. Their prologues save `f31`/`f30` and they contain several
  import-stream float calls whose argument orders need their own verification
  pass; rushing them would have produced low-confidence rows.
- The six GQR3 class-C rows named in `docs/COVERAGE_WAVE139_ENEMIES.md` are
  still class C and were not touched.

No address that already had a `ledger/functions.csv` row or a `fn_<addr>` body
in `src/game/enemies.rs` was re-claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to 872
  bytes (316 for the twelve specification rows, 104 for the nine class-B rows,
  452 for the nine bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every out-of-image callee was confirmed from the REL module-0 `R_PPC_REL24`
  import stream, not from the raw branch field.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice, and none of the 30 was in
  `ledger/functions.csv` before this pass.
- The module header was updated: cluster item 122, the cluster count (121 to
  122), the function totals (3,253 to 3,283 public translations, 3,182 to 3,212
  `fn_<addr>` bodies), the "start here" list, the wave count line and the
  records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave140` reports 30 rows for this lane
  (`translated/high=17`, `translated/medium=5`, `partial/medium=8`), 30 unique
  addresses and `OK`.
