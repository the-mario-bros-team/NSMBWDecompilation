# Coverage - Wave 73 (enemies: `0x80AF0000` bucket)

## How to read this file

This page records the Wave 73 `game::enemies` lane: the `0x80AF0000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below
corresponds to one real `pub unsafe fn fn_<addr>` body in that file, and the
`evidence` column names the Ghidra C export the body was reconstructed from.
The machine-readable mirror of the table is
`ledger/agents/wave73-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Float and
integer constants quoted in the docs were read from the REL data segments at
the cited synthetic address, not guessed from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 5,520 | 21 | 7 |

The bucket holds 208 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 64 were already in
`ledger/functions.csv`, leaving 145. This pass claims 28 and leaves 116.

## What this pass covers

The claimed bodies form four contiguous runs, all in families whose exports are
clean (no QGPR save/restore pseudo-code, no `halt_baddata`).

- **`0x80AF0010..0x80AF0660` - the `Wm_ob_redringkira` collectible (9 bodies).**
  The run is anchored by the literal data names `red_ring`, `Wm_ob_redringkira`
  and `Wm_ob_redringget`. It contains the effect-holder placement pair
  (`0x80AF0010` builds the `+0x7C` matrix twice, committing it to `+0x544` and,
  lowered by `20.0`, to `+0x5EC`), the matrix copier into the `+0x6C8` effect
  record, the two distance helpers over the four eight-byte player slots, the
  `red_ring` clip binder, the `0x800B1100` emission, the four-player collection
  scan (sound `0xB6`, `Wm_ob_redringget`, state record `0x80B77890`) and the
  `0.03515625` angle/scale step.
- **`0x80AF0B70..0x80AF0EF0` - the red-coin descriptor and constructor
  (5 bodies).** Two deleting-destructor copies, two 48-byte argument-swapping
  forwarders to `0x802DCBFC`, and the literal `0x6A8`-byte constructor with the
  class vtable `0x80B627A8`.
- **`0x80AF1200..0x80AF19B0` - the red-coin frame logic (5 bodies).** The
  `g3d/obj_coin.brres` binder with the `obj_coin_red` node, the main frame step
  (spawns `Wm_ob_redcoinkira` eight units up, drops item ids `0x23C..0x242`
  from the lazily initialised table at `0x80B779C0`, or runs the `0x243`
  collection branch and the per-player pass), the four-player item selection
  over `0x80B31328`, the post-step mask re-test, and the three-phase landing
  timer that clears the collecting player's mask word through
  `0x800E4470`/`0x802DD1EC`.
- **`0x80AF7CA0..0x80AF8200` - the `block_roulette` object (6 bodies).** The
  model loader for `block_roulette`, `block_roulette_glass` and
  `block_roulette_empty`, the two-mode frame step and six-word drop request,
  the frame-index to item-id mapping over `0x80B31508` (`2, 7, 11, 3, 4, 15,
  5`), the gated drop submission, the nine-frame face cycle and the full reset
  that installs the `0x80021140/0x80021150/0x80021160` callback triple twice.
- **Two duplicate destructor/forwarder triples.** `0x80AF1F70`, `0x80AF2090`
  and `0x80AF20C0` are byte-for-byte copies of `0x80AF0B70`, `0x80AF0C90` and
  `0x80AF0CC0`; they belong to the next descriptor skeleton and are translated
  because their bodies are identical and the evidence is unambiguous.

### Identity evidence

- `0x80AF0010` through `0x80AF0660` share one object: the same `+0x544`,
  `+0x5C0`, `+0x5EC`, `+0x6C8`, `+0x6D8` and `+0x688..+0x6AC` fields recur
  across the nine bodies, and the only string literals in the run are the
  three red-ring names.
- `0x80AF0CF0` is literal: `0x801628C0(0x6A8)`, the `0x80B627A8` vtable store
  and the four construction calls are unmodified. The `0x6A8` size explains
  every later field (`+0x6A0` is the last word).
- `0x80AF1200`/`0x80AF1500` are literal and name their own effect
  (`Wm_ob_redcoinkira`), model (`g3d/obj_coin.brres`) and node
  (`obj_coin_red`). The item ids and the variant table were read from the REL
  data segments.
- `0x80AF7CA0` is literal: all three node names and the archive path are
  visible, and `0x80AF8200` installs three code addresses
  (`0x80021140/0x80021150/0x80021160`) that are `main.dol` callbacks.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AF0010` | `fn_80af0010` | 336 | high |
| `0x80AF0160` | `fn_80af0160` | 84 | high |
| `0x80AF01E0` | `fn_80af01e0` | 68 | high |
| `0x80AF0230` | `fn_80af0230` | 144 | high |
| `0x80AF02C0` | `fn_80af02c0` | 164 | medium |
| `0x80AF03A0` | `fn_80af03a0` | 156 | high |
| `0x80AF0440` | `fn_80af0440` | 96 | medium |
| `0x80AF04A0` | `fn_80af04a0` | 364 | high |
| `0x80AF0660` | `fn_80af0660` | 248 | high |
| `0x80AF0B70` | `fn_80af0b70` | 88 | high |
| `0x80AF0C90` | `fn_80af0c90` | 48 | high |
| `0x80AF0CC0` | `fn_80af0cc0` | 48 | high |
| `0x80AF0CF0` | `fn_80af0cf0` | 108 | high |
| `0x80AF0EF0` | `fn_80af0ef0` | 104 | high |
| `0x80AF1200` | `fn_80af1200` | 764 | medium |
| `0x80AF1500` | `fn_80af1500` | 172 | high |
| `0x80AF15B0` | `fn_80af15b0` | 232 | medium |
| `0x80AF18B0` | `fn_80af18b0` | 136 | high |
| `0x80AF19B0` | `fn_80af19b0` | 404 | medium |
| `0x80AF1F70` | `fn_80af1f70` | 88 | high |
| `0x80AF2090` | `fn_80af2090` | 48 | high |
| `0x80AF20C0` | `fn_80af20c0` | 48 | high |
| `0x80AF7CA0` | `fn_80af7ca0` | 472 | high |
| `0x80AF7E80` | `fn_80af7e80` | 372 | medium |
| `0x80AF8000` | `fn_80af8000` | 68 | high |
| `0x80AF8050` | `fn_80af8050` | 256 | medium |
| `0x80AF8150` | `fn_80af8150` | 176 | high |
| `0x80AF8200` | `fn_80af8200` | 228 | high |

## Deliberate skips

- **`0x80AF0370` (44 bytes).** The export is a `halt_baddata` stub
  ("Control flow encountered bad instruction data"), so the body cannot be
  reconstructed. `0x80AF0440` still calls it as an address when the actor's
  `+0x355` byte is set, with a `// unknown:` comment.
- **`0x80AF16A0` (312 bytes).** The export is dominated by
  `unaff_GQR`/`ldexpf` quantiser fiction, so the float arithmetic is not
  recoverable. `0x80AF19B0` calls it as an address on the landing path.
- **`0x80AF2400`, `0x80AF2CF0`, `0x80AF3400`, `0x80AF3570`, `0x80AF5C60`.** All
  five carry the same GQR/unaff damage; `0x80AF2CF0` (624 bytes) is the largest
  clean-looking candidate that was rejected for that reason.
- **`0x80AF3840` (16 bytes).** A second `halt_baddata` export inside the
  `0x80AF3850` angle helper's block; not translated.
- **`0x80AF6190` (88 bytes).** A third byte-for-byte copy of the
  `0x80AF0B70` deleting destructor, left for the next pass because the three
  duplicates already in this lane establish the pattern and the budget went to
  the `block_roulette` run.
- **The remaining 116 untranslated functions of the bucket.** They are the
  actors between `0x80AF0660` and `0x80AFFFB0` that this pass did not touch.
  The largest clean candidates left are `0x80AF6ED0` (572 bytes),
  `0x80AF6CD0` (504) and `0x80AFFC90` (788).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  5,520 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave73` prints
  `wave73-enemies.csv: 28 rows`, `unique addresses: 113` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The remaining warnings are in `src/runtime/*` and do
  not involve this lane's file.
