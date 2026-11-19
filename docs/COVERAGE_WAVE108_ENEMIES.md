# Coverage - Wave 108 (`game::enemies`, the `0x80A80000` bucket)

## How to read this file

This page records the Wave 108 `game::enemies` lane: 28 functions of the
`0x80A80000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 89 of that file. Every row of the
machine-readable mirror (`ledger/agents/wave108-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 4,552 | 16 | 10 |

Two rows are `low`: the 48-byte `0x802DCBFC` forwarders (`0x80A81D30`,
`0x80A81D60`), whose hidden `r12` this-adjust pointer cannot be modelled from a
C export.

## What this pass covers

- **`0x80A819E0..0x80A81D90`, the `fugu` descriptor skeleton and constructor.**
  The two dispose callbacks (`0x80A819E0`, `0x80A81A40`), the constant producer
  that returns the shared `DieFumi` descriptor `0x80357BB4` (`0x80A81AA0`), the
  five-link `+0x30` owner walk with its recursive tail (`0x80A81B90`), the
  `:`-suffix name predicate built on `strrchr`/`strcmp` (`0x80A81C70`), the two
  `0x802DCBFC` forwarders (`0x80A81D30`, `0x80A81D60`) and the `0x848`-byte
  allocating constructor (`0x80A81D90`) whose class vtable is `0x80B4DD10`.
- **`0x80A82010..0x80A828C0`, model binding and placement.** The
  `g3d/fugu.brres` binder with node `fugu` and clip `swim` (`0x80A82010`), the
  spawn gate over `0x800964A0`/`0x80095C80` (`0x80A82300`), the node-6 matrix
  copy into `+0x5C8` (`0x80A82470`), the `Wm_en_cmnwaterdash` /
  `Wm_en_cmnwater02` effect spawner with sound `0x1DF` and stage effect `0x1B`
  (`0x80A824C0`), the height-band state predicate (`0x80A825F0`), the splash
  selector over sounds `0x1DE`/`0x1DD`/`0x282` (`0x80A826F0`), the
  reference-window x test (`0x80A82840`), the elapsed-time helper
  (`0x80A82880`) and the two-slot vertical step (`0x80A828C0`).
- **`0x80A82920..0x80A83200`, the swim/jump state machine.** The `0x11103`
  effect record builder (`0x80A82920`), the velocity aim helper (`0x80A82A30`),
  the window boundary test with its `96.0` margin (`0x80A82A90`), the 100-step
  `16.0` ground probe (`0x80A82B00`), the `swim`/`jump` clip binder
  (`0x80A82BB0`), the per-side tilt clamp over `0x80B2E370`/`0x80B2E3C8`
  (`0x80A82C20`), the two swim frame steps (`0x80A82D30`, `0x80A82F20`), the
  jump entry (`0x80A82FF0`), the jump step (`0x80A83080`) and the dive/land
  step (`0x80A83200`).

The class name is not printed in this run. The model archive `g3d/fugu.brres`,
the clips `swim`/`jump` and the two `Wm_en_*` effect names identify it as the
`fugu` (pufferfish) actor of the world-map block; the docs describe it as such
and do not invent a `daEn*` class name.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A819E0` | `fn_80a819e0` | 88 | high |
| `0x80A81A40` | `fn_80a81a40` | 92 | high |
| `0x80A81AA0` | `fn_80a81aa0` | 12 | high |
| `0x80A81B90` | `fn_80a81b90` | 224 | high |
| `0x80A81C70` | `fn_80a81c70` | 136 | high |
| `0x80A81D30` | `fn_80a81d30` | 48 | low |
| `0x80A81D60` | `fn_80a81d60` | 48 | low |
| `0x80A81D90` | `fn_80a81d90` | 236 | high |
| `0x80A82010` | `fn_80a82010` | 228 | medium |
| `0x80A82300` | `fn_80a82300` | 96 | high |
| `0x80A82470` | `fn_80a82470` | 68 | high |
| `0x80A824C0` | `fn_80a824c0` | 296 | medium |
| `0x80A825F0` | `fn_80a825f0` | 252 | medium |
| `0x80A826F0` | `fn_80a826f0` | 336 | medium |
| `0x80A82840` | `fn_80a82840` | 64 | high |
| `0x80A82880` | `fn_80a82880` | 52 | high |
| `0x80A828C0` | `fn_80a828c0` | 96 | medium |
| `0x80A82920` | `fn_80a82920` | 272 | medium |
| `0x80A82A30` | `fn_80a82a30` | 96 | high |
| `0x80A82A90` | `fn_80a82a90` | 104 | high |
| `0x80A82B00` | `fn_80a82b00` | 176 | medium |
| `0x80A82BB0` | `fn_80a82bb0` | 112 | medium |
| `0x80A82C20` | `fn_80a82c20` | 144 | high |
| `0x80A82D30` | `fn_80a82d30` | 420 | medium |
| `0x80A82F20` | `fn_80a82f20` | 200 | high |
| `0x80A82FF0` | `fn_80a82ff0` | 116 | high |
| `0x80A83080` | `fn_80a83080` | 212 | high |
| `0x80A83200` | `fn_80a83200` | 328 | medium |

### Deliberate skips

- **Everything else in the bucket.** 52 of the bucket's 201 functions remain
  untranslated after this pass. The nearest untranslated neighbours are the
  `0x80A803F0..0x80A81150` run (the previous world-map actor's state handlers
  and model helpers) and `0x80A83A30..0x80A83DE0`, which opens with the same
  five-entry descriptor skeleton as the `fugu` block and is the natural next
  target.
- **`0x80A80E50`, `0x80A80F40`, `0x80A81150`.** Three functions of the
  `0x80A803F0..0x80A81150` run keep `unaff_GQR3` conversion artefacts and
  removed unreachable blocks around the yaw/`+0x100` conversion, so their
  arithmetic cannot be reproduced from the export. They are outside this
  lane's chosen run and were not attempted.
- **`0x80A82B00` was kept, not skipped.** Its export carries three removed
  unreachable blocks and `GQR` noise, but those blocks are discarded
  `ldexpf` fragments outside the probe loop; the loop itself (100 steps of
  `16.0`, first-hit return) is literal, so the body is written at `medium`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  4,552 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was
  already in `ledger/functions.csv` (121 of the bucket's 201 rows were already
  translated, 80 were not).
- `node tools/check-lane-csv.mjs wave108` prints
  `wave108-enemies.csv: 28 rows` and `OK`, with no problem naming this CSV.
- `cargo check --message-format=short` emits no error naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 89, the cluster count
  (eighty-eight to eighty-nine), the function totals (2,359 to 2,387 public
  translations, 2,288 to 2,316 `fn_<addr>` bodies) and the record list.
