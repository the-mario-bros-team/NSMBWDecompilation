# Coverage - Wave 60 (`game::bases`, `0x8063` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when guards, stores, call arguments and constants are literal, and
`medium` when a callee identity, a dropped receiver argument or a field role
had to be inferred (`// inferred:` / `// unknown:` in the body).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 27 | 2984 | 16 | 11 |

## What this pass covers

- The member-body tail of the `dMultiCourseSelectContents_c` multi-course
  select screen at `0x80630030..0x80631520`, whose state table is
  [`fn_80631950`] and whose in-place constructor is [`fn_80631fe0`]. The
  object keeps the NW4R layout at `+0x70`, the interface wrapper at `+0x208`,
  the pane-pointer block at `+0x248..+0x280`, the selected course index at
  `+0x298`, the mode at `+0x29C`, the pending sound/effect kind at `+0x2A4`
  and the clear-state timer at `+0x2A8`; each pane exposes its visibility bit
  at `+0xBB`.
- The two release twins [`fn_80630030`] / [`fn_80630090`], the
  `MultiCourseSelectContents_03.brlyt` layout loader [`fn_80630360`] with its
  seven `P_enpty`, eight `T_corseNum_00` and two `N_courseInfo_00` panes, and
  the `im_W1_1.tpl` name table lookup [`fn_80630800`].
- The label machinery: the decimal formatter [`fn_80630560`], the numbered
  preview pane [`fn_80630620`], the name-pane table lookup [`fn_80630660`]
  and the threshold-based preview refresh [`fn_806306c0`].
- The effect and animation gates at `0x80630DE0..0x80631520`: the pending
  sound latch [`fn_80630de0`], the `Wm_2d_multiclear` spawn
  [`fn_80630e60`], the detach/animation helpers [`fn_80630f40`],
  [`fn_806310e0`], [`fn_806311e0`] and [`fn_80631360`], the sound helpers
  [`fn_806310a0`] / [`fn_80631190`] / [`fn_80631300`], the four copies of the
  transition gate [`fn_80631130`], [`fn_80631220`], [`fn_806312a0`] and
  [`fn_806313a0`], the clear-state transition [`fn_80631410`] and its
  countdown [`fn_80631520`].
- Four neighbouring release wrappers at `0x80632100..0x806321E0`
  ([`fn_80632100`], [`fn_80632140`], [`fn_80632180`], [`fn_806321e0`]) that
  follow the same screen's destructor and constructor.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80630030` | `fn_80630030` | 96 | high |
| `0x80630090` | `fn_80630090` | 100 | high |
| `0x80630360` | `fn_80630360` | 220 | high |
| `0x80630560` | `fn_80630560` | 180 | medium |
| `0x80630620` | `fn_80630620` | 52 | medium |
| `0x80630660` | `fn_80630660` | 92 | medium |
| `0x806306C0` | `fn_806306c0` | 316 | medium |
| `0x80630800` | `fn_80630800` | 28 | medium |
| `0x80630DE0` | `fn_80630de0` | 124 | high |
| `0x80630E60` | `fn_80630e60` | 212 | medium |
| `0x80630F40` | `fn_80630f40` | 72 | high |
| `0x806310A0` | `fn_806310a0` | 64 | medium |
| `0x806310E0` | `fn_806310e0` | 72 | high |
| `0x80631130` | `fn_80631130` | 88 | high |
| `0x80631190` | `fn_80631190` | 72 | medium |
| `0x806311E0` | `fn_806311e0` | 64 | high |
| `0x80631220` | `fn_80631220` | 88 | high |
| `0x806312A0` | `fn_806312a0` | 88 | high |
| `0x80631300` | `fn_80631300` | 88 | medium |
| `0x80631360` | `fn_80631360` | 64 | high |
| `0x806313A0` | `fn_806313a0` | 88 | high |
| `0x80631410` | `fn_80631410` | 260 | medium |
| `0x80631520` | `fn_80631520` | 132 | medium |
| `0x80632100` | `fn_80632100` | 64 | high |
| `0x80632140` | `fn_80632140` | 64 | high |
| `0x80632180` | `fn_80632180` | 96 | high |
| `0x806321E0` | `fn_806321e0` | 100 | high |

## Address range

`0x80630030..0x80631520`: the `dMultiCourseSelectContents_c` body helpers,
with the large untranslated bodies `0x80630820` (972 bytes), `0x80630BF0`
(484 bytes) and `0x80631950`-adjacent table code outside this window.

`0x80632100..0x806321E0`: four release wrappers immediately after the class
destructor `0x80631EC0` and constructor `0x80631FE0`.

The pass therefore covers twenty-seven of the 171 addresses that were
untranslated in the `0x8063` bucket at lane start (201 total; 30 were already
in `ledger/functions.csv`; 144 remain for later waves).

## Deliberate skips

- `0x80630820` (972 bytes) and `0x80630BF0` (484 bytes) are the two largest
  remaining bodies of this screen. They read the main.dol player tables at
  `0x80429F7C`/`0x80429F80` and were left for a lane that can verify the full
  table layout; the smaller callers were translated instead.
- `0x806318F0` (48) and `0x80631920` (48) are `0x802DCBFC` forwarders, the
  same unresolved indirect-jump stub already documented by earlier waves; the
  stub has no recoverable instruction stream, so no bodies were written.
- These addresses were already in `ledger/functions.csv` and were not
  duplicated: `0x80631950`, `0x80631EC0` and `0x80631FE0` in this window (the
  full pre-existing set spans the bucket).
- The rest of the `0x8063` bucket (144 untranslated at lane end, including the
  `0x80632650..0x8063F820` result/select tails) is outside this window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave60` reports `OK` for
  `ledger/agents/wave60-bases.csv` (27 rows, 16 high / 11 medium).
- `cargo check --message-format=short` reports no error or warning for
  `src/game/bases.rs`.
