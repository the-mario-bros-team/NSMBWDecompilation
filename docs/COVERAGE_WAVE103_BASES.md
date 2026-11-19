# Coverage - Wave 103 (`game::bases`, `0x807b` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address on this page is a **static analysis address**
under the canonical synthetic base **`0x80600000`** (produced by
`tools/rel2elf.mjs`), never a runtime address: the Wii REL loader chooses the
real load address, so the numbers only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c` (checked with a glob,
never assumed). "Confidence" is `high` when guards, stores, constants and call
arguments are literal, and `medium` when a field role, a global, a callee or the
owning class had to be inferred (those bodies carry `// observed:` /
`// inferred:` / `// unknown:` notes). No body in this pass is `low`.

The bucket holds 161 functions; 39 were already in `ledger/functions.csv` when
this pass began, leaving 122. This pass takes 30 of them, all inside the
`0x807B` world-select-map band.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3,108 | 19 | 11 |

## What this pass covers

- **The world-select map screen (`0x807B5BF0..0x807BF560`, 30 bodies).** The
  screen owns nine pane pointers in an array at `+0x248`, a pane count at
  `+0x294`, a per-world object at `+0x268`, a sub-layout at `+0x26C` and an
  interface object at `+0x20C` whose virtual slot `+0x18` takes a state
  descriptor. The state descriptors dispatched in this pass are `0x808340F8`,
  `0x80834178`, `0x80834238`, `0x80834338`, `0x80834378` and `0x808343F8`.
- **The layout binding.** [`fn_807bb510`] loads
  `worldSelectMap/worldSelectMap.arc` (`0x80820D0C`), opens
  `worldSelectMap_12.brlyt` (`0x80820D30`), binds the
  `worldSelectMap_12_inWindow` animation table (`0x80820CD0`) and the
  `A00_Window` pane animation (`0x80820CE8` + shared table `0x807D09F0`), and
  publishes the animation id at `+0x270` plus the `N_wCC_00`/`P_bg_00` panes at
  `+0x274`/`+0x278`. It is the direct sibling of `fn_80602160` (character-change
  select), which uses the same NW4R helper chain.
- **The pane-array plumbing.** [`fn_807bb9f0`] writes one truncated angle to
  `pane + 0x2C6` for every pane; [`fn_807bbc30`] raises or clears
  `pane + 0x275` across `count + 1` panes; [`fn_807bc5d0`] clears
  `pane + 0x277` on all nine panes after a sound stop; [`fn_807bb840`] wraps an
  out-of-range world index (`>= count` to `0`, `< 0` to `7`).
- **The state bodies.** [`fn_807bb630`] closes the interface and layout once
  `+0x2B3` is set; [`fn_807bbca0`] runs the back-navigation delay; [`fn_807bc090`]
  leaves the screen and dims the selected pane; [`fn_807bc160`] waits twenty
  frames before the exit state; [`fn_807bcaf0`] finalises the leave;
  [`fn_807bcce0`] confirms the world after its entry delay; and
  [`fn_807bce00`]/[`fn_807bce70`] start and poll the fade on the world object.
  [`fn_807bbd20`] spawns `Wm_cs_w9appear` from the world object's `+0xAC`
  position.
- **The manager and its two tables.** [`fn_807b92c0`] and [`fn_807b9390`] map
  the world byte `0x8042A24D` to the boss and course-object tables (the tables
  carry `larry`/`lemmy`/`wendy`/`morton`/`ludwig`/`kameck` and
  `cobPeachCastle`/`cobSandpillar`/`cobHatenaSwitch`/`cobKiller`/`cobTsuta`/
  `cobKoopaCastle`/`cobBoard` names), [`fn_807b9e40`] builds the map manager
  and publishes it through `0x8042A234`, and [`fn_807ba5d0`] runs the
  two-pass material submit.
- **The release wrappers and snapshots.** [`fn_807b7500`],
  [`fn_807bb270`], [`fn_807bb2b0`], [`fn_807bb2f0`], [`fn_807bb350`] and
  [`fn_807bdc70`] are the family's array-element release shapes (the
  `0x802B9280` free under an `object != 0` / `count > 0` guard, plus a `+0x18`
  sub-object teardown for the `*f0` variants); [`fn_807bf560`] snapshots the
  `0x807BF5A0` family's start position into `+0x610..+0x61C`; and
  [`fn_807b5bf0`]/[`fn_807b8c50`] submit the two world-select material keys.
  [`fn_807bb0c0`] and [`fn_807bb0d0`] are the 8-byte and 4-byte leaves of the
  same window.

Start with [`fn_807bb510`] for the layout, [`fn_807bc090`] for the state
sequence and [`fn_807b92c0`] for the per-world tables.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807B5BF0` | `fn_807b5bf0` | 84 | medium |
| `0x807B7500` | `fn_807b7500` | 64 | high |
| `0x807B8C50` | `fn_807b8c50` | 84 | medium |
| `0x807B92C0` | `fn_807b92c0` | 200 | medium |
| `0x807B9390` | `fn_807b9390` | 200 | medium |
| `0x807B9E40` | `fn_807b9e40` | 108 | medium |
| `0x807BA5D0` | `fn_807ba5d0` | 132 | medium |
| `0x807BB0C0` | `fn_807bb0c0` | 8 | high |
| `0x807BB0D0` | `fn_807bb0d0` | 4 | high |
| `0x807BB270` | `fn_807bb270` | 64 | high |
| `0x807BB2B0` | `fn_807bb2b0` | 64 | high |
| `0x807BB2F0` | `fn_807bb2f0` | 96 | high |
| `0x807BB350` | `fn_807bb350` | 100 | high |
| `0x807BB510` | `fn_807bb510` | 212 | high |
| `0x807BB630` | `fn_807bb630` | 116 | medium |
| `0x807BB840` | `fn_807bb840` | 40 | high |
| `0x807BB9F0` | `fn_807bb9f0` | 64 | medium |
| `0x807BBC30` | `fn_807bbc30` | 112 | medium |
| `0x807BBCA0` | `fn_807bbca0` | 112 | medium |
| `0x807BBD20` | `fn_807bbd20` | 88 | high |
| `0x807BC090` | `fn_807bc090` | 196 | high |
| `0x807BC160` | `fn_807bc160` | 144 | high |
| `0x807BC5D0` | `fn_807bc5d0` | 136 | high |
| `0x807BCA90` | `fn_807bca90` | 88 | high |
| `0x807BCAF0` | `fn_807bcaf0` | 132 | high |
| `0x807BCCE0` | `fn_807bcce0` | 120 | high |
| `0x807BCE00` | `fn_807bce00` | 104 | medium |
| `0x807BCE70` | `fn_807bce70` | 88 | high |
| `0x807BDC70` | `fn_807bdc70` | 88 | high |
| `0x807BF560` | `fn_807bf560` | 60 | high |

**Totals:** 30 translated, 0 partial, 0 stub (19 high, 11 medium, 3,108 bytes).

## Deliberate skips

Read and rejected, with the reason:

- `0x807B08A0` (8 bytes), `0x807B5C50` (8 bytes), `0x807B8EC0` (8 bytes) and
  `0x807BA740` (8 bytes) are `halt_baddata()` stubs: the export contains no
  instruction at all.
- `0x807B0BF0`, `0x807B0C20`, `0x807B30E0`, `0x807B3110`, `0x807B43E0` and
  `0x807B4410` (48 bytes each) are two-word stubs that tail-call the unresolved
  indirect-jump helper `0x802DCBFC`; the same trap is documented in the module
  header and in `docs/COVERAGE_r80260_802ed.md`.
- `0x807BB710` (188 bytes) is the pane-position publish. Half of its stores are
  paired-single writes that the decompiler prints as `x = x` self-assignments,
  so the real store width cannot be proven from the export.
- `0x807BB870` (84 bytes), `0x807BB8D0` (276 bytes), `0x807BBE20` (584 bytes),
  `0x807BC7E0` (484 bytes) and `0x807BCC00` (220 bytes) all close a
  paired-single int/fixed conversion with the magic constant
  `4503601774854169.0` (`0x4330000000000000` family). The constant carries
  decompiler-rounded digits, so the exact integer-to-float scale cannot be
  reconstructed honestly. They stay callable by address; [`fn_807bbca0`]
  calls `0x807BB8D0` that way.
- `0x807BC220` (156 bytes) and `0x807BC660` (220 bytes) open with an
  `unaff_GQR0`/`ldexpf` paired-single save bracket; the incoming GQR cannot be
  reconstructed.
- `0x807BD110` (2,908 bytes) is the last `StateID` table, still rejected for the
  same paired-single decompilation failure recorded in the wave-19 header note.
- `0x807BA510` (188 bytes) reads `extraout_r4`, a register the decompiler
  dropped, and passes it as the effect owner; the argument cannot be proven.
- `0x807B9F40` (392 bytes), `0x807BC310` (684 bytes), `0x807BF6B0` (164 bytes),
  `0x807BF760` (244 bytes), `0x807BFBE0` (64 bytes), `0x807BFC20` (220 bytes)
  and `0x807BFD00` (276 bytes) are readable and were left for budget: they are
  the map-open body and the `0x807BF5A0` state continuation, which need their
  own pass to name the `0x80429Fxx` globals they touch.

The remaining 92 untranslated functions of the bucket were not opened in this
pass (budget, not doubt).

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,108 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c` (globbed for
  each address prefix).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 39 translated
  addresses were re-read first and none is duplicated here.
- `node tools/check-lane-csv.mjs wave103` reports
  `wave103-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`; the crate's remaining errors belong to the parallel
  wave-103 lanes (`src/game/worldmap.rs`, `src/game/stage.rs`,
  `src/runtime/dvd.rs`).
