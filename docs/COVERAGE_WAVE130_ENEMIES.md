# Coverage - Wave 130 (`game::enemies`, wave-129 specification and `0x80AD0000` tail)

## How to read this file

This page records the Wave 130 `game::enemies` lane: 28 functions of REL
module 3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 112. Every
row of the machine-readable mirror (`ledger/agents/wave130-enemies.csv`) is one
real `pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra
C export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32`, never transcribed as invented values. Every constant table of
`docs/SPEC_WAVE129_ENEMIES.md` was re-read from
`work/rels/elf/d_enemiesNP.elf` at the mapping in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`); all of them matched, as did
the five `1.0` words at `0x80B2DD0C`, the `s16` pairs at `0x80B30540` and
`0x80B32588`, the four `Wm_en_*` strings and the two `fly_*` strings.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 8,536 | 24 | 4 |

The lane has two halves:

- **12 bodies** are the "ready" and "conditional" rows of
  `docs/SPEC_WAVE129_ENEMIES.md`. They were written from that page's
  instruction-level decode and re-verified word by word against both the raw
  ELF stream and the Ghidra C export.
- **16 bodies** are still-open entries of the planner bucket `0x80AD0000`
  (40 of that bucket's 189 functions were open when the pass started; this
  pass takes 16 and leaves 24).

## What this pass covers

- **The twelve specification bodies (`0x80A0BD20..0x80B1DFE0`).** Two matrix
  refreshes (`0x80A96230` model node, `0x80ADD930` bucket sibling), three
  spawn entries (`0x80ADD350` `block_pata`, `0x80A45DB0` water surface,
  `0x80AD5760` 9-cell grid), three timered state steps (`0x80B0F800`
  water-wave, `0x80B1DFE0` wall bounce, `0x80ACC520` Huhu frame), the Huhu
  breath/haze emitter `0x80AC95D0`, the two-arm pad gate `0x80A77C30`, the
  ice-slide ramp `0x80A35170`, the Choropu-like appear handler `0x80A0BD20`
  and the two-actor push exchange `0x80A2BC50`.
- **The `0x80AD0000` tail (16 bodies, `0x80ADD0E0..0x80ADE450` and
  `0x80AD3CE0..0x80AD7310`).** The `block_pata` model binder `0x80ADD570`
  (BRRES load, node bind, `fly_normal` clip pair), its clip binder
  `0x80ADE240`, its three `0x8007FA40` contact callbacks
  (`0x80ADDB40`/`0x80ADDBD0`/`0x80ADDD10`), its `+0xD4` state dispatch
  `0x80ADDA10`, its `0x898`-byte constructor `0x80ADD260` and the two
  `0x802DCBFC` forwarders `0x80ADD200`/`0x80ADD230`; the two frame tails
  `0x80ADE300`/`0x80ADE450`; the holder release `0x80ADD0E0`; the water-surface
  predicate `0x80AD4220`; and the `0x4C30` container actor's rail/range helpers
  `0x80AD3CE0`/`0x80AD7310`.
- **Two section-local adapters**: `w130_state_call` (the `+0x60` `+0xD4`
  class-slot call used nine times), `w130_rotate` (one `u16` rotation through
  one of the three matrix helpers), and the two mixed call shapes the bucket
  needed: `w130_call6_f64` (`f1` + `r3..r7`, the `0x800BFC60` grid emitter) and
  `w130_call10_f64x4` (`f1..f4` + `r3..r8`, the `0x8007FB10` water emitter).

### Corrections and confirmations against the specification

The specification is instruction-exact everywhere else it was re-checked. Four
statements needed repair, and each repair was re-read from the raw ELF words:

- **`0x80A35170`'s saved query slot is `+0x6B0`, not `+0x6B4`.** The store at
  `0x80A351C8` is `stw r0,0x6b0(r31)`; `+0x6B4` is the ramp direction written
  later at `0x80A352B0`. The export agrees with the raw word.
- **`0x80ADD350`'s `0x80166EF0` call passes `r4 = 0`.** The spec's call table
  says `r4 = cell`; `0x80ADD524` is `addi r4,r0,0`, and only `f1` carries the
  `(f32)cell` value. The export prints the same two arguments.
- **`0x80A77C30`'s `0x80167B20` calls pass `r4 = 0`, not the level index.** The
  indices only scale the `f1` table load (`rlwinm ...,2,0,29` + `lfsx`).
- **The `rlwinm` mask convention.** `MB`/`ME` are PowerPC bit numbers (bit 0 =
  most significant), so `rlwinm r11,r0,24,31,31` selects the *least* significant
  bit of the rotate and extracts `(task >> 8) & 1`. Decoding `MB`/`ME` as LSB
  indices inverts every mask and would also break arm B of `0x80A77C30`; with
  the correct convention the `cntlzw` + `rlwinm 27,5,31` + `srawi 31` triple is
  exactly `(x == 0)` with a zero sign word, as the specification states.

### Two operand corrections that belong to the exports, not the specification

- **`0x80ADE240`** keeps the incoming `r4` clip word for `0x8023A0B0`: only `r3`
  is rewritten at `0x80ADE260`. Ghidra's two-parameter declaration hides it, so
  the body takes the clip word as an explicit argument.
- **`0x80ADD260`'s `0x80094E80` call takes `r3 = this`.** Ghidra prints no
  argument, but `r3` is set once at `0x80ADD284` and is not reloaded before the
  call at `0x80ADD28C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A96230` | `fn_80a96230` | 572 | high |
| `0x80B0F800` | `fn_80b0f800` | 560 | high |
| `0x80A77C30` | `fn_80a77c30` | 548 | high |
| `0x80AC95D0` | `fn_80ac95d0` | 540 | high |
| `0x80B1DFE0` | `fn_80b1dfe0` | 540 | high |
| `0x80ADD350` | `fn_80add350` | 536 | medium |
| `0x80A45DB0` | `fn_80a45db0` | 500 | high |
| `0x80AD5760` | `fn_80ad5760` | 500 | high |
| `0x80ACC520` | `fn_80acc520` | 492 | high |
| `0x80A35170` | `fn_80a35170` | 464 | high |
| `0x80A0BD20` | `fn_80a0bd20` | 460 | medium |
| `0x80A2BC50` | `fn_80a2bc50` | 456 | high |
| `0x80ADD570` | `fn_80add570` | 280 | high |
| `0x80ADE240` | `fn_80ade240` | 124 | high |
| `0x80ADDB40` | `fn_80addb40` | 144 | high |
| `0x80ADDBD0` | `fn_80addbd0` | 308 | high |
| `0x80ADDD10` | `fn_80addd10` | 256 | high |
| `0x80ADDA10` | `fn_80adda10` | 36 | high |
| `0x80ADD930` | `fn_80add930` | 220 | high |
| `0x80ADD260` | `fn_80add260` | 240 | high |
| `0x80ADD200` | `fn_80add200` | 48 | medium |
| `0x80ADD230` | `fn_80add230` | 48 | medium |
| `0x80ADD0E0` | `fn_80add0e0` | 88 | high |
| `0x80ADE300` | `fn_80ade300` | 116 | high |
| `0x80ADE450` | `fn_80ade450` | 68 | high |
| `0x80AD4220` | `fn_80ad4220` | 136 | high |
| `0x80AD3CE0` | `fn_80ad3ce0` | 108 | high |
| `0x80AD7310` | `fn_80ad7310` | 148 | high |

### Why four rows are `medium`

- **The two `0x802DCBFC` forwarders.** The three-register shuffle is literal,
  but the hidden `r12` this-adjust pointer cannot be modelled, exactly as wave
  129 recorded for the five forwarders of the `0x80B20000` bucket.
- **`0x80ADD350`.** The control flow and every store are exact, but the two
  floats copied from `0x8042BB08`/`0x8042BB0C` live in `main.dol`; they are
  operands, not unresolved bases, and they are read verbatim.
- **`0x80A0BD20`.** The probe's direction byte is indexed from the `main.dol`
  byte table at `0x8042BB18`, so the probe argument is only as exact as that
  table.

## Deliberate skips

- **`0x80AD0A90` (32 bytes) and `0x80AD42B0` (36 bytes).** Both exports end in
  `halt_baddata` after the first guard: the control flow past the truncation is
  not recoverable, so no body was invented.
- **`0x80ADDE10` (580 bytes).** It is the fourth `block_pata` contact callback
  and is called by three bodies of this pass, but it was not in this lane's
  selection and was left untranslated; the three call sites cite it by address
  only. No row claims it.
- **The remaining `0x80AD0000` bucket rows** (24 still open, including
  `0x80ADDA40`'s neighbours `0x80ADDAC0`/`0x80ADE060`, the `0x80AD4xxx`
  cluster and `0x80AD8900`). They are readable and were left for a later pass
  once the body target was met. Nothing was written for them, so no ledger row
  claims them.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel_d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  8,536 bytes (6,168 for the twelve specification rows and 2,368 for the
  sixteen bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel_d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- All constants of `docs/SPEC_WAVE129_ENEMIES.md` and of the sixteen bucket
  bodies were re-read from `work/rels/elf/d_enemiesNP.elf` at
  `file_offset = address - 0x80A00000 + 0x1000`; the four tables cited above
  were also re-read word by word.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave130` reports
  `wave130-enemies.csv: 28 rows`, 28 unique addresses and `OK`.
- The module header was updated: cluster item 112, the function totals (2,987
  to 3,015 public translations, 2,916 to 2,944 `fn_<addr>` bodies) and the
  "start here" and records lists.
