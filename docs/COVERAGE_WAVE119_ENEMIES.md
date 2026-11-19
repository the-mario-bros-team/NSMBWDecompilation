# Coverage - Wave 119 (`game::enemies`, the `0x80AF0000` bucket)

## How to read this file

This page records the Wave 119 `game::enemies` lane: 27 functions of the
`0x80AF0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 100 of that file. Every row of the machine
readable mirror (`ledger/agents/wave119-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 3,744 | 6 | 21 |

The bucket holds 208 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 148 were already in
`ledger/functions.csv`, leaving 60. This pass claims 27 and leaves 33.

## What this pass covers

- **The `0x618`-byte sanbo head actor (`0x80AFC810..0x80AFD590` and
  `0x80AFD670..0x80AFDFB0`, 26 bodies).** The constructor `0x80AFC990`
  installs the class vtable `0x80B65138`; `0x80AFCB50` opens
  `g3d/sanbo.brres` (`0x80B65120`) and selects `sanbo_head` (`0x80B650E0`) or
  `sanbo_head_orange` (`0x80B65110`) from the `+0x604` slot index. The run then
  adds the two-slot matrix table at `+0x544` (placement, offset add and the two
  base-matrix builders with the lazy `0x1333/0/-0xE38` angle offsets), the
  `+0x5E0..+0x5E8` scale ramp, the per-type speed seeds for the descriptors
  `0x80B78008`/`0x80B78088`/`0x80B780C8`/`0x80B78144`/`0x80B77FC8`, the
  interpolation counters at `+0x5F0..+0x5FC` with the `0x80152480` sampling
  blend, the yaw decay and slot-angle adds, the `+0x348`/`+0x459` side and
  landing bytes, the `0x80B317FC`/`0x80B31820` roll tables, the `0x800E20A0`
  slot-0 sparkle and the two collision/position integration tails.
- **The `0x550`-byte `daEnSanboEl_c` descriptor (`0x80AFECE0`, one body).**
  The constant producer returns the `main.dol` descriptor word `0x80358004`
  that the existing `daEnSanboEl_c` state registration stores. The child
  constructor `0x80AFEFD0` is already translated in this file as
  [`alloc_sanbo_el_actor`], so it is not duplicated here.

### Why twenty-one rows are `medium`

- **Hidden-`r12` forwarders.** `0x802DCBFC` adjusts `r12`; only the three
  visible arguments can be modelled, so `0x80AFC930`/`0x80AFC960` are
  `medium`.
- **Unnamed engine callees.** `0x80069020`, `0x802DC9A0`, `0x8016A170`,
  `0x8016A150`, `0x80152480` and `0x8016F130` have no established names; their
  roles are inferred from the pointers and offsets they are paired with.
- **Virtual slots.** The `+0x10` call on the `+0x394` sub-object
  (`0x80AFCC50`) and the `+0x1C` call (`0x80AFD670`) are read from the object
  vtable at run time; the state-switch slot `+0xD4` is the file-wide idiom.
- **Dropped arguments.** The export prints `0x80AFD860()` and
  `0x8006CDE0()` without the `param_1` their sibling bodies pass; the
  translation forwards the actor and marks the call site.
- **Unproven data roles.** The `f32` words `0x80B317A4..0x80B317EC`, the
  per-type tables `0x80B77EE8`/`0x80B77F30` and the `0x8042C1B0` table are read
  from the image, never transcribed as invented constants. The
  `(double)CONCAT44(0x43300000, x ^ 0x80000000) - 0x80B317B8` sequence is the
  Metrowerks unsigned-int-to-double idiom and is written as `x as f64`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AFC810` | `fn_80afc810` | 88 | high |
| `0x80AFC930` | `fn_80afc930` | 48 | medium |
| `0x80AFC960` | `fn_80afc960` | 48 | medium |
| `0x80AFC990` | `fn_80afc990` | 148 | medium |
| `0x80AFCB50` | `fn_80afcb50` | 256 | medium |
| `0x80AFCC50` | `fn_80afcc50` | 156 | medium |
| `0x80AFCDD0` | `fn_80afcdd0` | 244 | medium |
| `0x80AFCED0` | `fn_80afced0` | 200 | medium |
| `0x80AFCFA0` | `fn_80afcfa0` | 184 | medium |
| `0x80AFD060` | `fn_80afd060` | 272 | medium |
| `0x80AFD210` | `fn_80afd210` | 108 | medium |
| `0x80AFD280` | `fn_80afd280` | 196 | medium |
| `0x80AFD500` | `fn_80afd500` | 76 | medium |
| `0x80AFD550` | `fn_80afd550` | 64 | medium |
| `0x80AFD590` | `fn_80afd590` | 88 | medium |
| `0x80AFD670` | `fn_80afd670` | 168 | medium |
| `0x80AFD720` | `fn_80afd720` | 312 | medium |
| `0x80AFD860` | `fn_80afd860` | 32 | high |
| `0x80AFD880` | `fn_80afd880` | 96 | medium |
| `0x80AFDCA0` | `fn_80afdca0` | 56 | high |
| `0x80AFDCF0` | `fn_80afdcf0` | 52 | medium |
| `0x80AFDE80` | `fn_80afde80` | 124 | high |
| `0x80AFDF20` | `fn_80afdf20` | 124 | high |
| `0x80AFDFB0` | `fn_80afdfb0` | 248 | medium |
| `0x80AFE1D0` | `fn_80afe1d0` | 220 | medium |
| `0x80AFE2C0` | `fn_80afe2c0` | 124 | medium |
| `0x80AFECE0` | `fn_80afece0` | 12 | high |

## Deliberate skips

- **`0x80AFD170` (156 bytes).** The helper returns a `double` built from a
  `short` through a `GQR3` paired-single conversion; the export carries no
  readable operand for the `ldexpf` arms and Ghidra removed two blocks.
- **`0x80AFD910` (388 bytes), `0x80AFDAC0` (480 bytes) and `0x80AFDD50`
  (300 bytes).** These are the three per-frame steps that use the same
  `GQR0`/`GQR3` `ldexpf` idiom plus `ps_` artefacts. The surrounding stores
  are readable, but the angle math cannot be separated from the decompiler
  noise, so they are left for a pass that can decode the paired-single stream.
- **`0x80AFEC20`/`0x80AFEC80` (88/92 bytes), `0x80AFEDD0` (224 bytes),
  `0x80AFEEB0` (136 bytes) and `0x80AFEF70`/`0x80AFEFA0` (48 bytes each).**
  The remaining `0x80AFEXXX` helpers: two generic detach/free copies, the
  `+0x30` chain walker with its five-fold inline unroll, the `:`-suffix name
  predicate and two more hidden-`r12` forwarders. They are straightforward but
  belong to the `daEnSanboEl_c` skeleton rather than this actor, and the pass
  budget was spent on the head actor.
- **`0x80AFEFD0` (100 bytes).** Already translated as
  [`alloc_sanbo_el_actor`] in this file, so no second body or ledger row is
  added for it.
- **Remaining bucket tail.** The 33 still-untranslated addresses are the ones
  listed above plus the larger `0x80AFF0D0`/`0x80AFF4F0`/`0x80AFFAF0` and
  `0x80AFFC90` (788 bytes) blocks; none is claimed here.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  3,744 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave119` reports the lane CSV clean.
- `cargo check --message-format=short` emits no error and no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: cluster item 100, the cluster count
  (ninety-nine to one hundred), the function totals (2,670 to 2,697 public
  translations, 2,599 to 2,626 `fn_<addr>` bodies), the "start here" list and
  the records list.
