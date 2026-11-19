# Coverage - Wave 79 (enemies: `0x80B20000` bucket, part 1)

## How to read this file

This page records the Wave 79 `game::enemies` lane: a first pass over the
`0x80B20000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave79-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 3,876 | 13 | 13 |

The bucket holds 139 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; three were already in
`ledger/functions.csv` (`0x80B213D0`, `0x80B287E0`, `0x80B28B20` - the
`daEnBlockHelp_c` and `daEnWrturn_c` registrations and the `daEnWrturn_c`
allocator), leaving 136. This pass claims 30 and leaves 106. Four of the new
rows are `low` confidence - all four are the 48-byte `0x802DCBFC` forwarders,
whose hidden-`r12` this-adjust is not modelled; the rest are `high` or
`medium`.

## What this pass covers

Four contiguous runs. The first three are all one actor class; the fourth is a
second state skeleton.

- **Run 1 (`0x80B201E0..0x80B20860`, seven bodies).** The `0x678`-byte
  actor's effect and destruction tail. Its constructor is
  `0x80B24B60` in Run 2, and every field it touches there
  (`+0x524`/`+0x544`/`+0x550`/`+0x55C`/`+0x650`/`+0x654`) reappears in this
  run: the two animation-holder matrix writers (`0x80064AF0` pre-transform,
  `0x801C0BD0` matrix build into `+0x7C`, `0x8016A170`/`0x8016A150`/
  `0x8016A1A0` per holder), the retraction driven by a neighbour's `+0x38C`
  mode (`0x80056CF0`, `0x80056BD0`, the `+0x394 -> +0x28 -> +0x14` descriptor
  test and a `+0x568` decrement), the one-shot `+0x650` spawn latch gated on
  the `+0x654` object, the `+0x55C` particle spawn through the `0x80429D60`
  manager, the `+0x560 - 16.0 * (3 - +0x568)` height ramp, the full destructor
  and the deleting destructor.
- **Run 2 (`0x80B247B0..0x80B24B60`, eight bodies).** A five-entry
  state-descriptor skeleton - two deleting destructors, the constant producer
  returning `0x80357BF4`, the five-step `+0x30` owner-chain walker, the
  `:`-suffix name predicate (`0x802E1C40`/`0x802E1AB4`) and two 48-byte
  forwarders - followed by the `0x678`-byte constructor it fronts
  (class vtable `0x80B6CF40`, a three-word block copied from `0x802FD328` into
  `+0x528..+0x530`, cleared words at `+0x534`/`+0x538`, the `Wm_ob_stream`
  helper at `+0x53C` with vtable `0x80329958`, and the zeroed
  `+0x650..+0x660` block).
- **Run 3 (`0x80B24D90..0x80B25130`, five bodies, plus the isolated producer
  `0x80B25650`).** The same actor's variant and effect helpers: the three
  lazily-initialised BSS direction tables at `0x80B797B0`, `0x80B797D8` and
  `0x80B79800` (each a one-byte guard plus eight floats copied from the
  `0x80B32A1C` block, then scaled by the observed `48.0` and `23.0`), the
  input-mask emitter toggle over the `0x8042A078` mask pair, the four-table
  orientation writer, the `Wm_ob_stream` spawn with its fixed `5500.0` height
  and `s16` variant angle, and the `0x304` effect path with its two-step
  retraction.
- **Run 4 (`0x80B26ED0..0x80B27290`, nine bodies).** A second skeleton with
  *two* constant producers (`0x80352A74`, `0x80352A34`), two deleting
  destructors, the same chain walker and name predicate, two forwarders, and
  the `0x8D0`-byte constructor it fronts (class vtable `0x80B6D6E8`, two
  model/animation holder pairs at `+0x524`/`+0x544` and `+0x58C`/`+0x5AC`, the
  `0x80315BF8` node at `+0x584`, and two `0x802DC9A0` pools of `8` and `6`
  elements).

### Identity evidence

- **The three runs really are one class.** The constructor at `0x80B24B60`
  writes `+0x528..+0x538`, `+0x53C` and `+0x650..+0x660`; Run 1 reads and
  writes `+0x524`, `+0x544`, `+0x550`, `+0x55C..+0x568`, `+0x650` and
  `+0x654`; Run 3 reads `+0x524`, `+0x148` and `+0x53C`. The 168-byte body at
  `0x80B24EE0` uses exactly the `+0x358`/`+0x35C` mask pair and the
  `+0x148` emitter pair (`0x8008C330`/`0x8008C380`) already documented for this
  module's input-driven actors.
- **The two constant producers in Run 4 name nothing.** `0x80352A74` and
  `0x80352A34` are four words apart inside one data record, i.e. two entries
  of a shared base-class descriptor group. No class name is invented for
  either.
- **The `0x678` constructor's three copied words are left unknown.** The
  `0x802FD328` block is `main.dol` data, not REL data; only the copy is
  reconstructed, and the doc marks the roles `// unknown:`.
- **The float words were read, not guessed.** `0x80B3274C` is `16.0f`,
  `0x80B32728`/`0x80B3272C`/`0x80B32730` are `0.0f`/`-1.0f`/`30.0f`,
  `0x80B32A28` is `48.0f`, `0x80B329D8` is `23.0f` and `0x80B32A78` is
  `5500.0f` in the REL image; the bodies load them through `read_f32` and the
  docs record the observed value.
- **The `0x80B32A1C` block is `0.0`/`1.0`/`-1.0`.** The three direction
  tables in `0x80B24D90` are byte copies of those three words, which is why
  the bodies read them from the source block instead of hard-coding floats.
- **The 48-byte forwarders** are the same `0x802DCBFC(param_2, param_2,
  param_1)` bodies already documented in this module; their hidden `r12`
  this-adjust is not modelled and each doc says so.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B201E0` | `fn_80b201e0` | 276 | medium |
| `0x80B20300` | `fn_80b20300` | 332 | medium |
| `0x80B20450` | `fn_80b20450` | 80 | high |
| `0x80B204A0` | `fn_80b204a0` | 76 | medium |
| `0x80B20550` | `fn_80b20550` | 192 | medium |
| `0x80B20610` | `fn_80b20610` | 140 | high |
| `0x80B20860` | `fn_80b20860` | 88 | high |
| `0x80B247B0` | `fn_80b247b0` | 88 | high |
| `0x80B24810` | `fn_80b24810` | 92 | high |
| `0x80B24870` | `fn_80b24870` | 12 | high |
| `0x80B24960` | `fn_80b24960` | 224 | medium |
| `0x80B24A40` | `fn_80b24a40` | 136 | medium |
| `0x80B24B00` | `fn_80b24b00` | 48 | low |
| `0x80B24B30` | `fn_80b24b30` | 48 | low |
| `0x80B24B60` | `fn_80b24b60` | 184 | high |
| `0x80B24D90` | `fn_80b24d90` | 328 | medium |
| `0x80B24EE0` | `fn_80b24ee0` | 168 | medium |
| `0x80B24F90` | `fn_80b24f90` | 136 | medium |
| `0x80B25020` | `fn_80b25020` | 204 | medium |
| `0x80B25130` | `fn_80b25130` | 132 | medium |
| `0x80B25650` | `fn_80b25650` | 12 | high |
| `0x80B26ED0` | `fn_80b26ed0` | 88 | high |
| `0x80B26F30` | `fn_80b26f30` | 92 | high |
| `0x80B26F90` | `fn_80b26f90` | 12 | high |
| `0x80B26FA0` | `fn_80b26fa0` | 12 | high |
| `0x80B27090` | `fn_80b27090` | 224 | medium |
| `0x80B27170` | `fn_80b27170` | 136 | medium |
| `0x80B27230` | `fn_80b27230` | 48 | low |
| `0x80B27260` | `fn_80b27260` | 48 | low |
| `0x80B27290` | `fn_80b27290` | 220 | high |

## Deliberate skips

- **`0x80B20010`** (152 bytes) is the bucket's first export and an effect
  spawn that adds the `0x80B32760` four-float offset table and a `1500.0f`
  word to the actor position before calling `0x80064610(0xBD, 0, &vector, 0,
  0)`. Its export carries paired-single `GQR0` noise on the vector argument
  (`ldexpf` calls with a fake `GQR0`), so the ABI of the third argument is not
  certain. It was left for a later pass rather than claimed at `low`.
- **`0x80B250F0`** (56 bytes) is referenced by `fn_80b25020`, but its export
  stops on bad instruction data (`halt_baddata`) and only the guard bit is
  visible. The caller keeps the call by address and its doc records why.
- **`0x80B25280` and `0x80B253D0`** (328 bytes each) are the two
  trail-history sprite steps. Both read their float pair from unmodelled
  paired-single argument registers (`in_f31`/`in_ps31_1`) that Ghidra never
  binds to the real parameters, so a faithful signature cannot be written from
  the export.
- **`0x80B25830..0x80B2A360`** (`0x80B25830`, `0x80B25930`, `0x80B25960`,
  `0x80B25A20`, `0x80B25CA0..0x80B26A10`, `0x80B27980..0x80B28480`,
  `0x80B28710`, `0x80B289A0`, `0x80B28AC0`, `0x80B28AF0`, `0x80B28F30..0x80B297A0`,
  `0x80B29910..0x80B2A070`, `0x80B2A360`) are ordinary clean exports but lie
  outside this lane's 25-30 body budget. They are left for a later pass in the
  same bucket.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  3,876 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's three existing rows are untouched).
- `node tools/check-lane-csv.mjs wave79` reports
  `wave79-enemies.csv: 30 rows`, 30 unique addresses and `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
