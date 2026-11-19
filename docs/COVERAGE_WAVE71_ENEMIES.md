# Coverage - Wave 71 (enemies: `0x80A001E0..0x80A02760`)

## How to read this file

This page records the Wave 71 `game::enemies` lane: the `0x80A00000` bucket of
REL module 3, `d_enemiesNP`, promoted to `src/game/enemies.rs`. Every row below
corresponds to one real `pub unsafe fn fn_<addr>` body in that file, and the
`evidence` column names the Ghidra C export the body was reconstructed from.
The machine-readable mirror of this table is
`ledger/agents/wave71-enemies.csv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

Roles in the actor are taken from the offsets the block itself uses; no class
name is printed anywhere in this range, so the actor is described by size
(`0x940` bytes reach into `+0x93B`), offsets and behaviour only.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 5,492 | 9 | 17 |

## What this pass covers

The bucket `0x80A00000` holds 212 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 66 of them were already in
`ledger/functions.csv`, leaving 146 untranslated. This pass claims 26 of those
and leaves 120. The claimed run is the single contiguous shell-riding enemy
block `0x80A01040..0x80A02760` plus the two empty virtual stubs immediately
before the base-enemy constructor, which keeps one actor's whole state machine
in one place.

- **The shell spawner (`0x80A01120`).** The owner comes from the section
  prologue `0x802DCD74`; its `+0x933` mode byte selects one of two entry words
  from the halfword table at `0x80B2A880` and patches the spawn flags
  (`0x10000000`, `0xF000007` or `0xD`). Mode 2 spawns a second actor shifted by
  `0x80B2A88C`, which is why the owner keeps two handles (`+0x8D8`, `+0x8DC`).
- **The release/re-enable pair (`0x80A012F0`, `0x80A01450`).** The same
  four-arm mode switch sets the child flags `+0x6C5`, `+0xA9A`, `+0xE00`,
  `+0x8AD` to `1` in the first and to `0` in the second, and re-runs the
  matching animation start/stop on the child's `+0x148` or `+0x754` holder.
- **The shell state machine (`0x80A019C0..0x80A02760`).** The open/closing/ride
  phases are frame-gated on the `+0x5C4` clip time (`0x80168080`) against six
  float thresholds in `0x80B2A8C4..0x80B2A8F4`; two of them convert a
  `0x80B32DDC`/`0x80B32DE0` constant. The block arms two durations into the
  `+0x900` timer (`0x80B32DD0`, `0x80B32DD4`) and switches state through
  `0x80B6E598`, `0x80B6E5D8`, `0x80B6E658`, `0x80B6E6D8`, `0x80B6E698`,
  `0x80B6E718` and `0x80B6E758`.
- **The two rocking helpers (`0x80A01760`, `0x80A01830`).** Byte-identical
  shapes on different fields: yaw `+0x100`/step `+0x928`/phase `+0x93A` with
  `±0x4FF` limits, pitch `+0x102`/step `+0x92A`/phase `+0x93B` with `±0x1FF`.
  The rock steps are driven from the launch handler with `(0x80, 8)` and
  `(0x10, 4)`.
- **The launch frame (`0x80A01A90`).** Five modes: input bit 2 switches to the
  damaged state, input bit 0 fires `Wm_mr_sealandsmk_s` and bounces up to three
  times, the `+0x908` counter runs the roll stepper, otherwise the shell spins
  `+0x926` by `0x200` per frame and derives the rocking speed from a sine
  (`0x80237BD0`) of that angle.
- **The small helpers.** `0x80A01040` (two-point ground probe returning a
  two-bit mask), `0x80A015B0` (effect-list drain), `0x80A01600` (clip binder
  over `0x80B32E30`/`0x80B2A890`), `0x80A01690` (four-player overlap test),
  `0x80A01900` (roll-angle stepper), and the two empty four-byte stubs at
  `0x80A001E0`/`0x80A001F0`.

### Identity evidence

- `0x80A01600` is literal: `0x8023A0B0(actor + 0x540, table[index])`, then
  `0x801651F0(+0x5C4, +0x544, record, rate[index])`.
- `0x80A01A90` is literal: `_DAT_8042a200`-style input masking, the
  `0x80A01690`/`+0x90C` stop test, and the `0x80237BD0` sine call.
- `0x80A02210` is literal: `0x8016C890(s_Wm_shellopen_80b32e50, 0, +0x8EC, 0, 0)`.
- `0x80A02600`/`0x80A02760` are literal: `0x80072CB0(+0x728, +0xE8)` followed
  by the `-speed` inversion guarded by `+0x914`.
- No class name is printed anywhere in this range; the constructor of this
  actor lies outside the claimed run, so the class is described by size,
  offsets and its `+0x728` controller only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A001E0` | `fn_80a001e0` | 4 | high |
| `0x80A001F0` | `fn_80a001f0` | 4 | high |
| `0x80A01040` | `fn_80a01040` | 216 | medium |
| `0x80A01120` | `fn_80a01120` | 464 | medium |
| `0x80A012F0` | `fn_80a012f0` | 348 | medium |
| `0x80A01450` | `fn_80a01450` | 348 | medium |
| `0x80A015B0` | `fn_80a015b0` | 76 | medium |
| `0x80A01600` | `fn_80a01600` | 136 | medium |
| `0x80A01690` | `fn_80a01690` | 108 | medium |
| `0x80A01760` | `fn_80a01760` | 200 | high |
| `0x80A01830` | `fn_80a01830` | 200 | high |
| `0x80A01900` | `fn_80a01900` | 180 | medium |
| `0x80A019C0` | `fn_80a019c0` | 124 | high |
| `0x80A01A90` | `fn_80a01a90` | 660 | medium |
| `0x80A01D50` | `fn_80a01d50` | 80 | medium |
| `0x80A01DA0` | `fn_80a01da0` | 36 | medium |
| `0x80A01E40` | `fn_80a01e40` | 104 | high |
| `0x80A01EB0` | `fn_80a01eb0` | 116 | high |
| `0x80A01F40` | `fn_80a01f40` | 404 | medium |
| `0x80A020E0` | `fn_80a020e0` | 92 | medium |
| `0x80A02150` | `fn_80a02150` | 184 | medium |
| `0x80A02210` | `fn_80a02210` | 168 | high |
| `0x80A022D0` | `fn_80a022d0` | 608 | medium |
| `0x80A02530` | `fn_80a02530` | 180 | high |
| `0x80A02600` | `fn_80a02600` | 316 | medium |
| `0x80A02760` | `fn_80a02760` | 136 | medium |

## Deliberate skips

- **`0x80A01700` (80 bytes).** The export ends in `halt_baddata()` and
  Ghidra reports "Control flow encountered bad instruction data". The visible
  part is a `+0x918` countdown; the rest of the body cannot be reconstructed
  honestly, so no row was claimed.
- **`0x80A00D00` (424 bytes).** This is the four-player carry step called by
  [`fn_80a02150`] and [`fn_80a022d0`]. Its export is dominated by QGPR
  save/restore pseudo-code (`in_f28..in_f31`, `in_ps28..in_ps31`), so the lane
  keeps the call as an address (`W71_FN_80A00D00`) and documents the role as
  inferred rather than claiming a body.
- **`0x80A00F20..0x80A01040`.** There is no exported function in this gap: the
  preceding wave-11 binder `0x80A00EB0` (400 bytes) ends exactly at
  `0x80A01040`, so the claimed run starts at the next symbol.
- **`0x80A03270`/`0x80A032A0` (48 bytes each).** Two identical three-argument
  forwarders to `0x802DCBFC`. Translating two bare thunks would pad the lane
  without adding coverage of a state machine, so they were left for a pass that
  can claim their enclosing block.
- **`0x80A03C30` and `0x80A03C90` (92 and 212 bytes).** Clean exports that sit
  just past the wave-12 `bakubaku` block's claimed range. They are the natural
  next candidates for this bucket, but they belong to that earlier actor, not
  to the shell actor this pass reconstructs, and the lane budget was spent on
  the contiguous run.
- **The `0x80A02800..0x80A03150` gap.** No exports: the next symbol after
  `0x80A02760` is `0x80A03150` (`daEnBuku_c`'s block, already claimed), so
  nothing was skipped between them.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  5,492 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave71` reports
  `wave71-enemies.csv: 26 rows` and `OK`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
