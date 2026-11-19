# Coverage - Wave 143 (`game::enemies`)

## How to read this file

This page records the Wave 143 `game::enemies` lane: 18 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 125 and mirrored
one row per body in `ledger/agents/wave143-enemies.csv`. Each `evidence` cell
names the Ghidra C export the body was reconstructed from; the byte counts are
the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` with the segment shift of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`).

The lane has two parts:

- **12 bodies** are the whole "ready" set of `docs/SPEC_WAVE142_ENEMIES.md`
  (ranks 1-12). Every one of them was class D only because of a
  `psq_l ...,qr3`; they are unblocked by the pinned `GQR3 = 0x00050005`
  (unsigned 16-bit, scale 0), so each such site is written as
  `(f32)(u16)field`.
- **6 bodies** are still-open rows of the `0x80B10000` planner bucket:
  `0x80B12470`, `0x80B174F0`, `0x80B1DA40`, `0x80B178B0`, `0x80B173B0` and
  `0x80B171B0`. All six sit in the `0x80B16xxx..0x80B17xxx` actor block and
  share the float block at `0x80B32230`, so their field roles cross-check each
  other.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 18 | 4432 | 16 | 2 |

Status split: 18 `translated` (16 high, 2 medium), no `partial` and no `stub`.

## What this pass covers

- **The `docs/SPEC_WAVE142_ENEMIES.md` ready tranche (`0x80A1D530..0x80AFDD50`).**
  The sub-actor orbit offset `0x80AD8900` (sub-actor heading cached in
  `+0x6F6`, orbit angle from `+0x706`, per-axis pivot and output bias), the
  `daEnSanboEl_c` lean/pitch step `0x80AFDD50` (child height plus `16.0` scale,
  the two lean tables, the `0x100` stepper, the `0x800` spin and `1536.0 * sin`),
  the mirrored `-4`/`+12` direction pair `0x80AB1900`, the fixed-radius
  sub-actor follow `0x80A43810` (including the `0x80064D10` detach path), the
  32-point trail generator `0x80AA37E0`, the two-state sine envelope
  `0x80A79630`, the `daEnLiftRotHalf_c` orbit `0x80ABAB50` (with the
  `0x80ABA950` publish and the `+0x584` vtable slot `0x0C`), the `daEnCoin_c`
  bounce and probe `0x80A40290`, the radius-vs-vector predicate `0x80A20810`,
  the `1.5`/`0.5` squash pair `0x80A7ADA0`, the `72`/`32` platform offsets
  `0x80A6EA20` and the mirrored 32-unit offset `0x80A1D530`.
- **The `0x80B10000` bucket rows (`0x80B12470..0x80B178B0`).** The
  side-dependent `+0x5CC` sidestep and the `+0x102` table step (`0x80B12470`),
  the five-step `0.25` ramp between `1.5` and `2.0` (`0x80B174F0`), the
  cached-point lerp with the `(4 + 14 + query) / distance` scale
  (`0x80B1DA40`), the state initialiser with the `[63, 70]` clamp and the
  `0x8006D090` record (`0x80B178B0`), the side-clamped ramp with the mode-1
  ground probe and stage effect (`0x80B173B0`), and the `0x40`/`0x64C` timer
  pair plus the `1.0..1.5` hover ramp (`0x80B171B0`).

### Why two rows are `medium`

- **`0x80B12470`** - the side test, both `lha`/`sth` pairs, the `0x8042C2DC`
  halfword (`0x0C00`) and the `0x80B32094` table (`0x0100`/`0xFF00`) are literal,
  but the role of `+0x459` (side/state index) is inferred from its two uses.
- **`0x80B171B0`** - both timer machines, the `0x40` reload and the
  `0.05`/`1.5`/`1.0` ramp are literal, but `0x80078470` is an unnamed stage
  helper, so its effect role is inferred from the same call shape in the
  neighbouring bodies.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A1D530` | `fn_80a1d530` | 208 | translated | high |
| `0x80A20810` | `fn_80a20810` | 228 | translated | high |
| `0x80A40290` | `fn_80a40290` | 232 | translated | high |
| `0x80A43810` | `fn_80a43810` | 252 | translated | high |
| `0x80A6EA20` | `fn_80a6ea20` | 212 | translated | high |
| `0x80A79630` | `fn_80a79630` | 236 | translated | high |
| `0x80A7ADA0` | `fn_80a7ada0` | 220 | translated | high |
| `0x80AA37E0` | `fn_80aa37e0` | 240 | translated | high |
| `0x80AB1900` | `fn_80ab1900` | 256 | translated | high |
| `0x80ABAB50` | `fn_80abab50` | 236 | translated | high |
| `0x80AD8900` | `fn_80ad8900` | 308 | translated | high |
| `0x80AFDD50` | `fn_80afdd50` | 300 | translated | high |
| `0x80B12470` | `fn_80b12470` | 168 | translated | medium |
| `0x80B171B0` | `fn_80b171b0` | 388 | translated | medium |
| `0x80B173B0` | `fn_80b173b0` | 320 | translated | high |
| `0x80B174F0` | `fn_80b174f0` | 180 | translated | high |
| `0x80B178B0` | `fn_80b178b0` | 268 | translated | high |
| `0x80B1DA40` | `fn_80b1da40` | 180 | translated | high |

## Deliberate skips

- **`0x80B14D80` (28 bytes).** The Ghidra export is a single `halt_baddata()`
  with a "bad instruction" warning, so there is no honest body to reconstruct.
- **The other 10 untranslated `0x80B10000` rows.** `0x80B115A0` (480),
  `0x80B147D0` (1184), `0x80B16060` (608), `0x80B162C0` (704),
  `0x80B16A80` (276), `0x80B16BA0` (316), `0x80B16CE0` (504),
  `0x80B16F40` (560), `0x80B179D0` (736) and `0x80B1AD30` (264) are left open.
  `0x80B115A0` is a full matrix/sprite construction, `0x80B16CE0` recovers its
  `this` from a CodeWarrior save helper that Ghidra models as a 64-bit return,
  and `0x80B1AD30` has the same save-helper shape; guessing the receiver would
  be worse than leaving them open.
- **`0x80ABA950`.** The lift-direction publisher called by `fn_80abab50` is a
  156-byte untranslated `qr3` row. Its two stores are documented by
  `docs/SPEC_WAVE142_ENEMIES.md`, so the body calls it by analysis address and
  keeps `r * sin` live in `f1` for the following virtual dispatch; it is not
  re-claimed as a ledger row here.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 18 sizes sum to 4432
  bytes (2928 for the twelve specification rows, 1504 for the six bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 18 addresses were absent from
  `ledger/functions.csv` and from every earlier `fn_<addr>` body before this
  pass.
- Every `psq_l ...,qr3` in the twelve specification rows is one `(f32)(u16)`
  widen; the `qr0` prologue pairs are `f64` spills and are not ported.
- The module header was updated: cluster item 125, the cluster count (124 to
  125), the function totals (3317 to 3335 public translations, 3246 to 3264
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` exits `0` with no error and no warning
  naming `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave143` reports 18 rows for this lane
  (`translated/high=16`, `translated/medium=2`), 18 unique addresses and `OK`.
