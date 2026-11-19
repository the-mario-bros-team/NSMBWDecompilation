# Coverage - Wave 103 (`game::enemies`, the `daEnSanbo_c` block of the `0x80AF0000` bucket)

## How to read this file

This page records the Wave 103 `game::enemies` lane: 27 functions of the
`0x80AF0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 84 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave103-enemies.csv`) names the Ghidra
C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The bucket holds 208 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 121 were already in
`ledger/functions.csv` before this pass and 27 are claimed here, leaving 60.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 4,960 | 4 | 23 |

No row is `low` confidence. Twenty-three rows are `medium` because the class
kind `0x93`, the five `daEnSanbo_c::StateID_*` names and the untranslated callees
(`0x80AFD500`, `0x80AFD550`, `0x80AFD590`, and the `main.dol` helpers) are only
tied to their roles by call shape.

## What this pass covers

One near-contiguous run of 27 functions, from the slot-ladder spawner of the
move state to the retire check. Every export in the run is complete: there is
no `unaff_GQR*` quantiser pseudo-code whose folded value is consumed and no
`halt_baddata`.

- **Identity.** The class attribution is literal, not assumed. The static
  callback table at `0x80B64F3C..0x80B64FEB` holds fifteen
  `{0xFFFFFFFF, callback, 0}` records; six of them name functions of this
  block (`0x80AFBBE0`, `0x80AFBD00`, `0x80AFBEB0`, `0x80AFBFF0`, `0x80AFC110`,
  `0x80AFC190`), and the five `daEnSanbo_c::StateID_Move` / `_Damage` /
  `_Vacume` / `_CriticalDie` / `_FreezeDie` strings begin immediately after the
  table at `0x80B64FEC`. The five names were read byte by byte from
  `work/rels/elf/d_enemiesNP.elf`; the block therefore belongs to
  `daEnSanbo_c`, whose name survives only inside those StateID strings.
- **`0x80AFA2E0..0x80AFA720`, the slot ladder and the two contact callbacks.**
  `0x80AFA2E0` spawns the whole `daEnSanboEl_c` ladder: the first child sits at
  `(x, y + 16 * (count - 1), z + 16 * count)`, every later one steps `16.0`
  down on both `y` and `z`, and each spawn is tagged `(side << 4) | index` with
  the `+0x348` side byte; it returns `0` on the first failed spawn. The three
  small helpers (`0x80AFA3F0`, `0x80AFA480`, `0x80AFA4C0`) maintain the
  `+0x528` live-slot bitmask, the popcount used by the effect and speed
  builders, and each child's `+0x608` "live slots above me" value.
  `0x80AFA570` writes the three `i16` slot words `(-0x800, 0, -0x400)`.
  `0x80AFA680` and `0x80AFA720` are the two contact callbacks: both copy the
  pair position through `0x800954D0` and end in the shared record builder, and
  the second one branches on the other actor's `+0x18`, runs the child `+0x6C`
  name byte through `0x800E20A0`, redistributes the slots and switches state.
- **`0x80AFAD80..0x80AFAE30`, the three slot counters.** Counts of the live
  bits below, above and between two slot indices; `0x80AFAE30` uses
  `0x802DE514` (integer absolute value) for its span and returns `-1` for an
  out-of-range index. Its two guards are printed with `&&` by the decompiler,
  which is flagged in the body because the two tests can never both hold.
- **`0x80AFB3D0` and `0x80AFB560`, the `+0x414` contact record.** The record's
  seven words and three bytes are literal: the two position words, the two
  direction words
  (`-0.24375` and `-4.0` in the default builder, `0.0` in the contact
  builder), the `0x80B77E78` pointer word, the `+0x510` handle, the
  `0xFFFFFFFF` trailer, the side byte from `0x80064A30` and the kind byte
  (`0x32` in the default builder, the child `+0x6C` byte in the contact
  builder). The `0x80AFBFF0` damage step reacts to exactly those kind bytes
  (`< 4` uses `0x800E20A0`, `0x32` uses `0x800E23C0`).
- **`0x80AFB670` and the actor-list queries.** `0x80AFB670` redistributes the
  slot order after a landing: slots below the target get the blend rate
  `1.25 + 0.25 * (count - slot - 1)` and their live-before index, slots above
  get rate `0.0` and index `-1`, and the exact match keeps the plain side
  byte. `0x80AFB7F0` walks the children through the `+0x394` sub-object and
  the `0x80B78048` BSS name object; `0x80AFB8A0`/`0x80AFB8F0`/`0x80AFB940`/
  `0x80AFB9A0`/`0x80AFBA00`/`0x80AFBA70` are the six list-wide operations
  (existence test, `+0x5C4` flag, the two copies of the `0x18` timer triple,
  and the child class slots `+0x98`/`+0x9C`).
- **`0x80AFBAE0..0x80AFC190`, the effect, speed and frame steps.**
  `0x80AFBAE0` raises `Wm_en_sanbospillsand` at `(x, y + 8 * live, 5500.0)`
  with the `0x80B316D8` scale entry; `0x80AFBBE0` selects the side speed from
  the `0x80B315E0` `(left, right)` pair table, writes `+0xD4 = 8 * live` and
  `+0x590 = 4096 * (12 + 16 * (live - 1))`, and hands the `+0x58C` block to
  `0x8006D090`; the three frame steps then implement the turn (side flip,
  negated `+0xE8`, `+0xB8` copied into `+0xAC`), the `Wm_en_sanbosmoke` death
  puff, the `+0x534`/`+0x36D` damage latch with its `100`-frame `+0x174`
  counter, the state switches to `0x80B77DB8`/`0x80B77F88`, and the retire
  path.

### Deliberate skips

- **The nine table callbacks without a Ghidra function.** The identity table
  also names `0x80AFBCF0`, `0x80AFBE80`, `0x80AFBEA0`, `0x80AFBFB0`,
  `0x80AFBFD0`, `0x80AFBFE0`, `0x80AFC100`, `0x80AFC150` and `0x80AFC180`,
  but none of them is a row of `functions.tsv` and there is no export file to
  reconstruct them from. They are not claimed.
- **`0x80AF5C60` (260 bytes).** It sits just before this run and is free, but
  its body is pure paired-single pseudo-code: the `unaff_GQR3` merge mask
  selects between the high and low halves of `sVar8` and the folded value is
  consumed by both `0x80237C40` and `0x80237BD0`, so the two scale terms
  cannot be reconstructed from the C.
- **The remaining 60 functions of the bucket.** The obvious next candidates
  are the `daEnSanboParts_c` block at `0x80AFC810..0x80AFD590` (whose
  `sanbo_head`/`sanbo_body` pointer table sits at `0x80B650E0`, and which owns
  the slot writers `0x80AFD500`/`0x80AFD550`/`0x80AFD590` referenced above) and
  the nineteen `daEnSanboEl_c` callbacks at `0x80AFEC20..0x80AFFFB0` that
  `register_sanbo_el_states` already names.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AFA2E0` | `fn_80afa2e0` | 272 | medium |
| `0x80AFA3F0` | `fn_80afa3f0` | 132 | medium |
| `0x80AFA480` | `fn_80afa480` | 56 | high |
| `0x80AFA4C0` | `fn_80afa4c0` | 164 | medium |
| `0x80AFA570` | `fn_80afa570` | 28 | high |
| `0x80AFA680` | `fn_80afa680` | 160 | medium |
| `0x80AFA720` | `fn_80afa720` | 376 | medium |
| `0x80AFAD80` | `fn_80afad80` | 80 | high |
| `0x80AFADD0` | `fn_80afadd0` | 88 | high |
| `0x80AFAE30` | `fn_80afae30` | 236 | medium |
| `0x80AFB3D0` | `fn_80afb3d0` | 392 | medium |
| `0x80AFB560` | `fn_80afb560` | 260 | medium |
| `0x80AFB670` | `fn_80afb670` | 372 | medium |
| `0x80AFB7F0` | `fn_80afb7f0` | 172 | medium |
| `0x80AFB8A0` | `fn_80afb8a0` | 72 | medium |
| `0x80AFB8F0` | `fn_80afb8f0` | 76 | medium |
| `0x80AFB940` | `fn_80afb940` | 96 | medium |
| `0x80AFB9A0` | `fn_80afb9a0` | 96 | medium |
| `0x80AFBA00` | `fn_80afba00` | 112 | medium |
| `0x80AFBA70` | `fn_80afba70` | 112 | medium |
| `0x80AFBAE0` | `fn_80afbae0` | 248 | medium |
| `0x80AFBBE0` | `fn_80afbbe0` | 264 | medium |
| `0x80AFBD00` | `fn_80afbd00` | 380 | medium |
| `0x80AFBEB0` | `fn_80afbeb0` | 248 | medium |
| `0x80AFBFF0` | `fn_80afbff0` | 268 | medium |
| `0x80AFC110` | `fn_80afc110` | 64 | medium |
| `0x80AFC190` | `fn_80afc190` | 136 | medium |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  4,960 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave103` reports
  `wave103-enemies.csv: 27 rows` and `OK`.
- `cargo check --message-format=short` finishes the library build and emits no
  error or warning naming `src/game/enemies.rs`. The 14 remaining warnings all
  belong to other files (`src/runtime/bt.rs`, `src/runtime/cpp.rs`,
  `src/runtime/input.rs`).
- The module header was updated: a new cluster item 84, the cluster count
  (eighty-three to eighty-four), the function totals (2,217 to 2,244 public
  translations, 2,146 to 2,173 `fn_<addr>` bodies) and the record list.
