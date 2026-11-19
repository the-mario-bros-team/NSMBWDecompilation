# Coverage - Wave 104 (`game::bases`, `0x8062` bucket)

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

The bucket holds 204 functions; 84 were already in `ledger/functions.csv` when
this pass began, leaving 120. This pass takes 27 of them, all inside the
`0x8062C5F0..0x8062ED70` state-handler body of `dMultiCourseSelect_c`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 5,580 | 16 | 11 |

## What this pass covers

- **The `dMultiCourseSelect_c` state body (`0x8062C5F0..0x8062ED70`, 27
  bodies).** The class is identified by the eighteen state descriptors the
  constructor [`fn_8062f050`] registers (`StateID_DispWait`,
  `StateID_OpenAnimeEndWait`, `StateID_ClearWaKuAnimeEndWait`,
  `StateID_ArrowOnStageExitAnimeEndWait`, `StateID_ChangeAnimeEndWait`,
  `StateID_keyWait`, `StateID_SuccessionKeyWait`,
  `StateID_WorldSelectBeforeButtonReturn`, `StateID_WorldSelect`,
  `StateID_SuccessionWorldSelect`, `StateID_CrossMarkToReturnArrow`,
  `StateID_WorldSelectArrowAnimeEndWait`,
  `StateID_NextPageBeforeButtonReturn`, `StateID_NextPageScroll`,
  `StateID_HitAnimeEndWait`, `StateID_ExitAnimeEndWait`,
  `StateID_NextSceneWait`, `StateID_PauseWait`).
- **The screen layout used by every body.** `+0x70` is the NW4R layout
  (`0x800C9610` idle query, `0x800C92F0`/`0x800C9380` clip start,
  `0x800C9490` clip stop, `0x800C93D0` clip reset), `+0x1F4` is the arrow group
  (four panes at `+0x258`, `+0x280`, `+0x2A8`, `+0x2D0`), `+0x2DC..+0x328` is
  the twenty-entry course-pane array, `+0x32C` is the interface object whose
  virtual `+0x18` takes the next state descriptor, `+0x43C` is the page
  direction, `+0x440..+0x4DF` is the ten-record ranking table (`0x10` bytes
  each), and `+0x4F0..+0x520` holds the saved float, cursor, previous cursor,
  repeat delay, page, available count, last pad sample and the three mode
  flags.
- **The ranking-table helpers.** [`fn_8062c5f0`] copies the eleven save-slot
  availability bytes from the save object at `0x80429F7C` (`+0xAE0`) into
  `+0x515` and counts them; [`fn_8062c650`] scans forward from an index; and
  [`fn_8062c690`] performs the ten-record insertion shift (the export's
  four-record block unroll is preserved in address order).
- **The page and cursor animation drivers.** [`fn_8062d1c0`] applies pane
  variant `1`; [`fn_8062d200`] picks the clear or arrow-exit state from the
  saved slot word; [`fn_8062d370`] shows/hides the left and right page arrows
  from the page and the available count; [`fn_8062d7e0`] moves the pane
  highlight; [`fn_8062dcc0`] saves the arrow offsets and switches to the
  mirrored pane set; [`fn_8062e2b0`]/[`fn_8062e440`] push the saved arrow
  offset into the four animation channels; and [`fn_8062e580`] dims the pane
  being left.
- **The input machine.** [`fn_8062d8f0`] is the world-select handler (repeat,
  page flip, hit, pause), [`fn_8062db40`] repeats the last cursor move while
  the d-pad sample is unchanged, [`fn_8062df50`] maps the first arrow press,
  and [`fn_8062e0e0`] runs the held-arrow repeat with its five-frame delay.
  [`fn_8062e6a0`] plays the page-turn clips and [`fn_8062e830`] restores the
  pane set and walks the cursor back to a visible course.
- **The idle gates.** [`fn_8062d540`], [`fn_8062ddd0`], [`fn_8062e3a0`],
  [`fn_8062e500`], [`fn_8062e5f0`] and [`fn_8062e740`] each wait for one clip
  group and then select the next state descriptor.
- **The leaves.** [`fn_8062e990`] clears the selection byte on all twenty
  panes, and [`fn_8062ed60`]/[`fn_8062ed70`] read the per-page course tables at
  `0x807C7AF0` and `0x807C7AFC`.

Start with [`fn_8062d8f0`] for the input machine, [`fn_8062d370`] for the arrow
visibility rules and [`fn_8062c690`] for the ranking table.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8062C5F0` | `fn_8062c5f0` | 92 | high |
| `0x8062C650` | `fn_8062c650` | 56 | high |
| `0x8062C690` | `fn_8062c690` | 220 | medium |
| `0x8062D1C0` | `fn_8062d1c0` | 64 | high |
| `0x8062D200` | `fn_8062d200` | 220 | medium |
| `0x8062D370` | `fn_8062d370` | 452 | medium |
| `0x8062D540` | `fn_8062d540` | 276 | high |
| `0x8062D7E0` | `fn_8062d7e0` | 116 | high |
| `0x8062D8F0` | `fn_8062d8f0` | 544 | medium |
| `0x8062DB40` | `fn_8062db40` | 352 | medium |
| `0x8062DCC0` | `fn_8062dcc0` | 272 | high |
| `0x8062DDD0` | `fn_8062ddd0` | 180 | high |
| `0x8062DE90` | `fn_8062de90` | 172 | medium |
| `0x8062DF50` | `fn_8062df50` | 348 | medium |
| `0x8062E0E0` | `fn_8062e0e0` | 440 | medium |
| `0x8062E2B0` | `fn_8062e2b0` | 240 | high |
| `0x8062E3A0` | `fn_8062e3a0` | 152 | high |
| `0x8062E440` | `fn_8062e440` | 144 | medium |
| `0x8062E500` | `fn_8062e500` | 112 | high |
| `0x8062E580` | `fn_8062e580` | 112 | high |
| `0x8062E5F0` | `fn_8062e5f0` | 160 | high |
| `0x8062E6A0` | `fn_8062e6a0` | 156 | medium |
| `0x8062E740` | `fn_8062e740` | 236 | high |
| `0x8062E830` | `fn_8062e830` | 196 | medium |
| `0x8062E990` | `fn_8062e990` | 228 | high |
| `0x8062ED60` | `fn_8062ed60` | 16 | high |
| `0x8062ED70` | `fn_8062ed70` | 24 | high |

**Totals:** 27 translated, 0 partial, 0 stub (16 high, 11 medium, 5,580 bytes).

## Deliberate skips

Read and rejected, with the reason:

- `0x8062EA90` (196 bytes) and `0x8062ED90` (240 bytes) hand their object
  pointer through the `0x802DCD74` compiler save/restore thunk, which Ghidra
  models as a `u64` return. The decompiler then uses the recovered "high word"
  as the object in `0x8062ED90` while its caller passes the object in the first
  argument, so the real argument order cannot be proven from the export. They
  stay callable by address.
- `0x8062EFF0` (48 bytes) and `0x8062F020` (48 bytes) are two-word stubs that
  tail-call the unresolved indirect-jump helper `0x802DCBFC`; the same trap is
  documented in the module header and in `docs/COVERAGE_r80260_802ed.md`.
- `0x8062D120` (156 bytes) reads its object from the same `0x802DCD74` thunk
  and then uses it as `this` for `0x8062C800`, so its signature is equally
  unrecoverable.
- `0x8062D660` (384 bytes) opens with an `unaff_GQR0`/`ldexpf` paired-single
  bracket; the incoming GQR cannot be reconstructed.

The remaining 87 untranslated functions of the bucket were not opened in this
pass (budget, not doubt); the earlier waves already recorded the front of the
bucket, and this pass deliberately stops before `0x8062F050`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (27 rows, 5,580 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c` (globbed for
  each address prefix).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the bucket's 84 translated
  addresses were re-read first and none is duplicated here. The file now holds
  2,567 bodies.
- `node tools/check-lane-csv.mjs wave104` reports
  `wave104-bases.csv: 27 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`. The crate currently fails on nine errors in the parallel
  wave-104 lanes (`src/audio/ax.rs` and `src/game/worldmap.rs`), which are
  outside this lane and left to their owners.
