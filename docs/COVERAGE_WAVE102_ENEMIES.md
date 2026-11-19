# Coverage - Wave 102 (`game::enemies`, the `huhu` family of the `0x80AC0000` bucket)

## How to read this file

This page records the Wave 102 `game::enemies` lane: one contiguous run,
`0x80AC7790..0x80AC9A40`, of the `0x80AC0000` bucket of REL module 3,
`d_enemiesNP`, promoted to `src/game/enemies.rs` (section 83 of that file).
Every row below is one real `pub unsafe fn fn_<addr>` body, and the `evidence`
column of the machine-readable mirror (`ledger/agents/wave102-enemies.csv`)
names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The bucket holds 177 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 88 were already in
`ledger/functions.csv` before this pass and 26 are claimed here, leaving 63.
Wave 69 (`docs/COVERAGE_WAVE69_ENEMIES.md`) deliberately skipped this exact run
as "the actor bodies behind each constructor", so the whole block was still
untouched.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 4,952 | 10 | 14 |

Two rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80AC7F70` and `0x80AC7FA0`, whose hidden-`r12` this-adjust pointer is not
modelled (the same limitation recorded for every `0x802DCBFC` copy in waves
56, 72 and many later passes).

## What this pass covers

One contiguous run of 26 functions, from the split-container initialiser to the
`attack_ed` entry. Every export in it is complete: no `unaff_GQR*` quantiser
pseudo-code whose folded value is actually consumed, and no `halt_baddata`.

- **`0x80AC7790..0x80AC7C70`, the split-container actor and the `0x770`-byte
  sibling's tail.** `0x80AC7790` seeds a two-record BSS offset table on first
  use, starts the `split` clip, stores the variant's `(0.75, 3.5)` /
  `(-0.75, 3.5)` speed pair into `+0xE8`/`+0xEC`, lifts `+0xB4` by `64.0` for
  non-zero variants, writes the `+-45` degree facing from `0x8042C1A4`,
  spawns `Wm_en_kuribosplit` and enters state `1`. `0x80AC7900` is its
  three-state frame step (`0.8` then `0.6` blends, `+0x178 |= 7`, a `10`-frame
  `+0x500` timer and the `0x8035362C` hand-off). `0x80AC7B70`/`0x80AC7BC0`
  post effects `0x1FE`/`0x1FF`, and `0x80AC7C70` is the destructor that
  releases the `+0x5F0` helper, the `+0x5C0`/`+0x584` animation pair, the
  `+0x544` holder and the `+0x524` model of the preceding `0x770`-byte
  constructor `0x80AC6F80` (translated in wave 69).
- **`0x80AC7E50..0x80AC7FA0`, the descriptor tail.** The usual normaliser pair
  member (`0x8015F800` then `0x802B9280` for `mode > 0`) and the two 48-byte
  deleting-destructor forwarders of the skeleton that fronts this block.
- **`0x80AC7FD0`, the `0xBE0`-byte huhu constructor.** Class vtable
  `0x80B59D88`; the `+0x524` model holder, the `+0x544` animation holder, the
  `+0x584` animator bank with the `0x803295D8`/`0x80329518` interface pair at
  `+0x5BC`, the embedded owner pair at `+0x5C8`, and **five** `0x128`-byte
  breath sub-objects at `+0x628`, `+0x750`, `+0x878`, `+0x9A0` and `+0xAC8`.
  The first four also clear their six-word tail; the fifth ends too close to
  the block end and gets only the `0x80B4C310` vtable, exactly as the export
  emits it.
- **`0x80AC8140..0x80AC93E0`, the huhu actor proper.** `0x80AC82B0` is the
  literal `g3d/huhu.brres` binder: it opens the archive with the `huhu` node,
  binds the model at `+0x540`/`+0x544`, attaches the `wait` clip to `+0x584`
  at rate `1.0`, and registers and plays the `dead_02` clip at `+0x5BC`.
  `0x80AC8140` is the `Entry` (scale `1.3`, bounds `16.0`/`80.0`/`128.0`, the
  `+-20` degree facing table `0x80B30540`, effect `0x148` with scale `0x30` or
  `0x10`, and the `0x80B75EB0` state). `0x80AC84D0` rebuilds `+0x7C` and the
  scale, `0x80AC8640` spawns the fifteen-word effect block at `+0x484`
  (`1.15`/`1.0`/`1.0` plus six zeros, callback `0x80A0B5A0`, lifetime `0x3C`),
  `0x80AC8980`/`0x80AC8A90` are the two pair-contact record builders for
  `+0x414..+0x432`, `0x80AC8F70` snapshots nodes `10`/`0`/`4` into the
  `+0x5F8`/`+0x604`/`+0x610` vectors, and `0x80AC93E0` is the frame step that
  posts `Wm_en_huhuhaze` and `Wm_en_huhufloat` from those vectors and counts
  `+0x5EC` down to the `0x80B75E70` state.
- **`0x80AC9020..0x80AC9A40`, the shared clip machinery.** `0x80AC9020`
  (node lookup + `0x801651F0` + `0x80169F80` + a `1.0` blend) and
  `0x80AC90B0` (node lookup + `0x80165FD0` on the `+0x5BC` owner +
  `0x80166310`) are the two binders; `0x80AC9230`/`0x80AC92A0`/`0x80AC9310`
  are the six `attack_{L,R}_{st,md,ed}` wrappers, which pick the side from the
  sign bit of `+0x102` and play at rate `6.0`; `0x80AC9380` starts the `wait`
  clip at rate `5.0` and arms `+0x5EC = 0xB4`; and `0x80AC9540`/`0x80AC97F0`/
  `0x80AC9A40` are the three attack entries (sound `0x226`, `+0x5F0 = 0xB4`
  with `+0x360 = 1`, and the plain speed clear).

### Identity evidence

- **The names are literal and were dumped from the ELF, not assumed.**
  `work/rels/elf/d_enemiesNP.elf` holds `Wm_en_kuribosplit` (`0x80B598A8`),
  `split` (`0x80B598BC`), `g3d/huhu.brres` (`0x80B59C7C`), `huhu`
  (`0x80B59C8C`), `wait` (`0x80B59C94`), `dead_02` (`0x80B59CA0`),
  `attack_L_st`/`attack_R_st` (`0x80B59CA8`/`0x80B59CB4`), the `_md` pair
  (`0x80B59CC8`/`0x80B59CD4`), the `_ed` pair (`0x80B59CE8`/`0x80B59CF4`),
  `Wm_en_huhuhaze` (`0x80B59D08`), `Wm_en_huhufloat` (`0x80B59D18`),
  `Wm_en_huhubreath` (`0x80B59D28`) and `Wm_en_huhubreathstart`
  (`0x80B59D3C`). The three two-entry pointer tables at `0x80B59CC0`,
  `0x80B59CE0` and `0x80B59D00` were read word by word.
- **The constants were dumped, not assumed.** `0x80B30470` is `0.0`,
  `0x80B304A8` `0.6`, `0x80B304D0` `0.75`, `0x80B304D4` `3.5`,
  `0x80B304D8` `-0.75`, `0x80B304DC` `-0.1875`, `0x80B304E0` `64.0`,
  `0x80B304E4` `0.8`, `0x80B304E8` `-0.5`, `0x80B30504` `1.3`,
  `0x80B30548` `16.0`, `0x80B3054C` `8.0`, `0x80B30550` `1.0`,
  `0x80B30554` `1.15`, `0x80B305CC` `7.0`, `0x80B305F0` `6.0`,
  `0x80B305F4` `5.0`, `0x80B305F8` `5500.0` and `0x80B305FC` `74.0`.
  The two facing tables are `i16` pairs: `0x80B30540` = `{+0x0E40, -0x0E40}`
  (about `+-20` degrees) and `0x8042C1A4` = `{+0x2000, -0x2000}` (`+-45`
  degrees).
- **No class name survives.** `0x80B59D88` is referenced only by the
  `0x80AC7FD0` constructor, so the actor is described by size, vtable and
  field offsets only; `huhu` is an archive/model name, not a class name.

### Deliberate skips

- **`0x80AC8850` (304 bytes).** The third pair-contact variant. Its contact
  word is `(f32 table value + 0.4 * +0xE8) * extraout_f1`, where the scale is
  folded into a `GQR` register that the C export does not preserve, so the
  real record value cannot be reconstructed from the evidence.
- **`0x80AC95D0` (540 bytes) and `0x80AC9AA0` (372 bytes).** The two frame
  steps of the `0xBE0` actor. Both call the attack wrappers as
  `FUN_80ac9230(param_1)` / `FUN_80ac9310(param_1)`, i.e. Ghidra prints the
  `r3` actor and drops the `f64` rate that travels in `f1`; the wrapper body
  would have to invent that rate. Everything else in both exports is clean and
  they are the obvious next candidates once the disassembly is available.
- **The remaining 63 functions of the bucket.** They are the actor bodies
  between `0x80AC0750` and `0x80ACFE60` that neither this pass nor waves 55,
  69 and 84 touched. The largest candidates left are
  `0x80ACA9A0..0x80ACCF90` (the `net_walk` rail family of
  `daEnRailPolyChild_c`), `0x80ACDB00..0x80ACEE40` (that actor's remaining
  handlers) and `0x80ACF950` (720 bytes).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AC7790` | `fn_80ac7790` | 352 | medium |
| `0x80AC7900` | `fn_80ac7900` | 444 | medium |
| `0x80AC7B70` | `fn_80ac7b70` | 80 | high |
| `0x80AC7BC0` | `fn_80ac7bc0` | 80 | high |
| `0x80AC7C70` | `fn_80ac7c70` | 204 | medium |
| `0x80AC7E50` | `fn_80ac7e50` | 88 | high |
| `0x80AC7F70` | `fn_80ac7f70` | 48 | low |
| `0x80AC7FA0` | `fn_80ac7fa0` | 48 | low |
| `0x80AC7FD0` | `fn_80ac7fd0` | 368 | high |
| `0x80AC8140` | `fn_80ac8140` | 356 | medium |
| `0x80AC82B0` | `fn_80ac82b0` | 372 | high |
| `0x80AC84D0` | `fn_80ac84d0` | 252 | high |
| `0x80AC8640` | `fn_80ac8640` | 264 | medium |
| `0x80AC8980` | `fn_80ac8980` | 260 | medium |
| `0x80AC8A90` | `fn_80ac8a90` | 252 | medium |
| `0x80AC8F70` | `fn_80ac8f70` | 176 | medium |
| `0x80AC9020` | `fn_80ac9020` | 136 | medium |
| `0x80AC90B0` | `fn_80ac90b0` | 132 | medium |
| `0x80AC9230` | `fn_80ac9230` | 108 | medium |
| `0x80AC92A0` | `fn_80ac92a0` | 108 | medium |
| `0x80AC9310` | `fn_80ac9310` | 108 | medium |
| `0x80AC9380` | `fn_80ac9380` | 72 | high |
| `0x80AC93E0` | `fn_80ac93e0` | 352 | medium |
| `0x80AC9540` | `fn_80ac9540` | 116 | high |
| `0x80AC97F0` | `fn_80ac97f0` | 100 | high |
| `0x80AC9A40` | `fn_80ac9a40` | 76 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  4,952 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 26 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave102` reports
  `wave102-enemies.csv: 26 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace build is currently stopped by errors in
  other Wave 102 lanes (`src/game/worldmap.rs`, `src/resource/archive.rs`,
  `src/game/stage.rs`, `src/runtime/input.rs`), which do not name this file.
- The module header was updated: a new cluster item 83, the cluster count
  (eighty-two to eighty-three) and the function totals (2,191 to 2,217 public
  translations, 2,120 to 2,146 `fn_<addr>` bodies).
