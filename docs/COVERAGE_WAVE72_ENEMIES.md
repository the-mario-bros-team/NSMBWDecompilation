# Coverage - Wave 72 (enemies: `0x80AB0000` bucket)

## How to read this file

This page records the Wave 72 `game::enemies` lane: the `0x80AB0000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below
corresponds to one real `pub unsafe fn fn_<addr>` body in that file, and the
`evidence` column names the Ghidra C export the body was reconstructed from.
The machine-readable mirror of this table is
`ledger/agents/wave72-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 3,240 | 17 | 10 |

## What this pass covers

The bucket `0x80AB0000` holds 188 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 43 of them were already in
`ledger/functions.csv`, leaving 145 untranslated. This pass claims 27 of those
and leaves 118. The claimed run is split in two contiguous blocks, both chosen
because their exports are clean (no QGPR save/restore pseudo-code and no
`halt_baddata`).

- **The `0x80AB7810..0x80AB9030` block (15 functions).** The run opens with the
  tail of the five-entry state-descriptor skeleton the earlier waves
  established - the `:`-suffix name predicate `0x80AB7810` (`strrchr` of `':'`
  at `0x802E1C40` over the vtable `+0x20` name, then `strcmp` at `0x802E1AB4`),
  the two 48-byte argument-swapping forwarders `0x80AB78D0`/`0x80AB7900` and the
  `0x770`-byte constructor at `0x80AB7930`. The constructor is literal: size
  `0x770`, class vtable written as `0x80304E10` and replaced with `0x80B55E98`,
  then the holders at `+0x524`, `+0x540`, `+0x544`, `+0x584`, `+0x5CC` and
  `+0x5F0`, the last three with their own vtables (`0x80B330E0`, `0x803295D8`
  replaced by `0x80329548`, `0x80329958`).
- **The presentation helpers of that run.** `0x80AB7E50` spawns effect family
  `0x33` at the four corners of a rectangle whose half extents are the data
  constants `16.0`/`8.0` (`0x80B2FE48`/`0x80B2FE4C`), with the group byte
  `+0x391` shifted into bits 16..23; `0x80AB7F40` spawns the named effect
  `Wm_en_kuribobigsplit` (`0x80B55E80`); `0x80AB8240` plays sound `0x200` from
  the manager at `0x80429D5C`; `0x80AB8290` starts the `walk` clip
  (`0x80B56190`/`0x80B56188`) on both clip holders; `0x80AB85D0` binds
  `lift_buranko` from `g3d/lift_buranko.brres` (`0x80B561C4`/`0x80B561AC`);
  `0x80AB9030` binds `jump_st` (`0x80B56468`) and writes the base speeds
  `+0xE8..+0xF0`. The three remaining helpers read the field cluster
  `+0x850`/`+0x854` (the three-phase timer `0x80AB8A60`), `+0x2DC` (the handle
  getter `0x80AB8D50`), `+0x5EC`/`+0x61C` (the angle helper `0x80AB8E30` and
  the effect emitter `0x80AB8FE0`) and `+0x718` (the owner predicate
  `0x80AB8F40`).
- **The `0x80ABB3D0..0x80ABBCF8` block (12 functions).** This is the
  `lift_zen_toge` lift actor, named by the `g3d/lift_zen_toge.brres` and
  `lift_zen_toge` literals in `0x80ABB7D0`. The block starts with the deleting
  destructor `0x80ABB3D0` that `register_lift_rot_half_states` installs as the
  callback of the two `daEnLiftRotHalf_c` `Create`/`Move` records, followed by
  three more dispose forwarders (`0x80ABB5E0`, `0x80ABB620`, `0x80ABB680`), the
  model/clip binder, and the spawn-word initialiser `0x80ABB950`.
- **The ten-record variant table.** `0x80ABB950` recovers the owner through the
  section prologue `0x802DCD74`, decodes the word at owner `+0x04` into a
  record index (`& 0xF`, clamped to 0 beyond record 9), a variant count
  (`>> 12 & 0xF`), a mirrored flag (`>> 24 & 1`) and three animator parameters,
  and selects one of ten `0x40`-byte records at `0x80B57060`. The record holds
  the animation block (`0x28` bytes), two corner pairs (`+0x24..+0x30`), the
  spawn position (`+0x34`/`+0x38`) and the clip-name pointer (`+0x3C`, the
  export's `PTR_s_lift_zen_toge_A_left_80b5709c`). The layout constants are
  `192.0`, `8.0`, `896.0`, `16.0`, `-376.0` and `752.0`
  (`0x80B2FFB4..0x80B2FFC8`), all read back from the REL data section.
- **The lift's motion helpers.** `0x80ABB880` latches the travelling coordinate
  from `+0xAC` or `+0xB0` (axis byte `+0x5C5`), `0x80ABB8B0` integrates it with
  the speed at `+0x5AC`, `0x80ABB8E0` selects one of two state records from the
  spawn word through the class vtable slot `+0xD4`, `0x80ABBB40` releases the
  owned sub-object once the instance is no longer current, `0x80ABBBA0`
  rebuilds and commits the render matrix, and `0x80ABBCC0` steps the animator
  and re-applies the coordinate.

### Identity evidence

- `0x80AB7930` is literal: `0x801628C0(0x770)`, the two vtable stores and every
  holder constructor call are unmodified in the export.
- `0x80AB85D0` and `0x80ABB7D0` are literal: both resource-name pairs and the
  `0x20`-byte element size match the other model binders of this file.
- `0x80ABB3D0` is literal *and* identified by the caller: the state table at
  `0x80ABB210` registers it as the callback of the two
  `daEnLiftRotHalf_c` records, and its body is the standard
  `0x8015F800(this, 0)` + `0x802B9280(this)` destructor pair.
- `0x80ABB950` is literal: the masks, the `0x40` stride, the six layout
  constants, the `0x8008C3E0`/`0x8008C330` holder pair and the ten-argument
  `0x8007FAD0` call are all visible. The two code labels `0x80ABB6F0` and
  `0x80ABB780` it passes as callbacks have no `functions.tsv` entry, so they
  stay addresses.
- No class name is printed for the first block; its objects are described by
  size, offsets and the literal resource names. The constructor allocates
  `0x770` bytes while `0x80AB8A60`/`0x80AB8F40` use `+0x850`/`+0x854` and
  `+0x718`, so more than one object shares that address range.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB7810` | `fn_80ab7810` | 136 | high |
| `0x80AB78D0` | `fn_80ab78d0` | 48 | medium |
| `0x80AB7900` | `fn_80ab7900` | 48 | medium |
| `0x80AB7930` | `fn_80ab7930` | 248 | high |
| `0x80AB7E50` | `fn_80ab7e50` | 240 | high |
| `0x80AB7F40` | `fn_80ab7f40` | 68 | high |
| `0x80AB8240` | `fn_80ab8240` | 80 | high |
| `0x80AB8290` | `fn_80ab8290` | 84 | high |
| `0x80AB85D0` | `fn_80ab85d0` | 172 | high |
| `0x80AB8A60` | `fn_80ab8a60` | 188 | high |
| `0x80AB8D50` | `fn_80ab8d50` | 56 | medium |
| `0x80AB8E30` | `fn_80ab8e30` | 100 | medium |
| `0x80AB8F40` | `fn_80ab8f40` | 72 | medium |
| `0x80AB8FE0` | `fn_80ab8fe0` | 80 | high |
| `0x80AB9030` | `fn_80ab9030` | 140 | high |
| `0x80ABB3D0` | `fn_80abb3d0` | 88 | high |
| `0x80ABB5E0` | `fn_80abb5e0` | 64 | medium |
| `0x80ABB620` | `fn_80abb620` | 96 | high |
| `0x80ABB680` | `fn_80abb680` | 100 | high |
| `0x80ABB7D0` | `fn_80abb7d0` | 176 | high |
| `0x80ABB880` | `fn_80abb880` | 36 | high |
| `0x80ABB8B0` | `fn_80abb8b0` | 40 | high |
| `0x80ABB8E0` | `fn_80abb8e0` | 112 | medium |
| `0x80ABB950` | `fn_80abb950` | 496 | medium |
| `0x80ABBB40` | `fn_80abbb40` | 96 | medium |
| `0x80ABBBA0` | `fn_80abbba0` | 120 | high |
| `0x80ABBCC0` | `fn_80abbcc0` | 56 | medium |

## Deliberate skips

- **The nine QGPR-damaged exports of the bucket** - `0x80AB1780`,
  `0x80AB1900`, `0x80AB21D0`, `0x80AB29C0`, `0x80AB5710`, `0x80ABA080`,
  `0x80ABA950`, `0x80ABAB50` and `0x80ABDF80`. Their C bodies are dominated by
  `unaff_GQR0`/`unaff_GQR3` quantiser pseudo-code and `ldexpf` fiction, so the
  real float arithmetic cannot be recovered from the export. `0x80AB1900`
  (256 bytes) is the clearest example: the same `(x, y)` conversion block is
  inlined four times with different unmodelled scale factors.
- **The two `halt_baddata` exports** - `0x80AB1830` (92 bytes) and
  `0x80AB1A00` (8 bytes). Ghidra reports "Control flow encountered bad
  instruction data", so the tail of both bodies is missing.
- **`0x80AB82E4..0x80AB85D0` and `0x80AB90BC..0x80AB98B0`.** No exported
  function lies in either gap; the neighbouring exports end exactly at the
  stated addresses, so nothing was skipped inside them.
- **`0x80AB98B0` (56 bytes).** A clean `0x8015F800`/`0x802B9280` release
  forwarder that belongs to the next actor's descriptor block
  (`0x80AB98B0`/`0x80AB99D0`/`0x80AB9A00`); the lane budget went to the two
  contiguous runs that carry real behaviour instead.
- **The remaining 118 untranslated functions of the bucket.** They are the
  actors between `0x80AB0570` and `0x80ABF650` that this pass did not touch;
  the largest clean candidates left are `0x80AB0570` (664 bytes), `0x80AB0EE0`
  (640) and `0x80ABF1A0` (444).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  3,240 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave72` prints `wave72-enemies.csv: 27 rows`,
  `unique addresses: 87` across the three lane files present at the time and
  `OK`.
- `cargo check --message-format=short` produces no diagnostic that names
  `src/game/enemies.rs`. The three type errors the run reports are in
  `src/game/bases.rs`, which belongs to the parallel `game::bases` lane.
