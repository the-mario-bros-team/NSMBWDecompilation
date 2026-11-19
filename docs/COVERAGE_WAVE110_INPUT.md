# Coverage - Wave 110 (`runtime::input`, KPAD sampling callback and the DPD/accelerometer calibration)

## How to read this file

1. **Summary** gives the per-file totals and the closing inventory of the
   `[0x801E0000, 0x801F0000)` bucket.
2. **What this pass covers** describes the two bodies and how they connect to
   the wave 97/103/106 bodies around them.
3. The **function table** lists both bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** explains why the two remaining bucket entries stay
   untranslated.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. Both rows are
`translated`; both are `medium` because each carries one documented inference.
Names stay `fn_<addr>` because the image has no symbol table for this layer; the
doc comments carry the roles and repeat the `// observed:` / `// inferred:` /
`// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 2 | 4,688 | 0 | 2 |

Bucket inventory (`[0x801E0000, 0x801F0000)`):

| Item | Count |
|---|---:|
| Functions in the bucket | 103 |
| Untranslated before this pass | 4 |
| Selected and translated by this pass | **2** |
| Deliberately skipped | 2 |
| Untranslated in the bucket after this pass | 2 |

## What this pass covers

Waves 103 and 106 left exactly four entries of the band, and all four were
listed as skips in `docs/COVERAGE_WAVE106_INPUT.md`. Re-reading the exports
showed that two of them are only blocked by the *dead* paired-single prologues
the earlier passes assumed were load-bearing, so this pass lands those two:

- **DPD and accelerometer calibration finaliser (`0x801E55C0`, 2,792 B).** The
  read completion [`fn_801e7050`] calls it with `(channel, result)` for the
  `0x0` and `0x176C` factory-data reads. It seeds the four DPD corner records
  at `+0x854`..`+0x872` from the four packed words at `r2-0x4D98`..`-0x4D8C`,
  seeds the accelerometer references at `+0x874`..`+0x87E` with
  `0x212`/`0x27C`, validates the two eleven-byte DPD records with the `0x55`
  checksum (two attempts, `"Dpd_Setting_is_broken_"` between them), sorts the
  four unpacked 10-bit points into the corner record that matches their half of
  the sensor area, then averages them (`sum / 4`), derives the two rotation
  deltas from two `atan` reference angles, and writes the
  `0x803A1A60`–`0x803A1AA0` per-channel tables. The second half validates the
  two ten-byte accelerometer records (at `0x16` for current firmware) and
  stores the two 10-bit triples plus the volume/motor bytes.
- **Per-channel sampling callback (`0x801EE230`, 1,896 B).** Registered at
  `+0x54C` by [`kpad_set_sampling_callback_801e0fd0`] from
  [`fn_801edf40`] and [`fn_801edaf0`]. For one report it files the report in the
  sixteen-slot status ring (or the variable bank past slot `0x10`), stores the
  report id at `+0x36`, captures the two `1/delta` accelerometer scales when
  the accelerometer id `1` arrives, rewrites the three `0x803A1BAC` floats when
  `+0x50` is armed, runs the `0x80345770` format handshake (re-queuing the
  expected id through [`fn_801e11a0`], or starting the `0x801E39C0` command
  after the `+0x00` callback), and for the classic-controller id `3` walks the
  `r13-0x4B6E` neutral-point state machine that folds the four stick samples
  into the running means at `0x803A30D8`..`0x803A3110`. The per-channel `+0x40`
  callback always runs last.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801E55C0` | `fn_801e55c0` | DPD + accelerometer calibration from the factory buffer | 2,792 | translated | medium | `work/ghidra/main-gb/export/801e55c0_FUN_801e55c0.c` |
| `0x801EE230` | `fn_801ee230` | per-channel sampling callback | 1,896 | translated | medium | `work/ghidra/main-gb/export/801ee230_FUN_801ee230.c` |

Totals: **2 translated**, 0 partial, 0 stubs; 0 high / 2 medium / 0 low;
4,688 bytes. Address range `0x801E55C0`–`0x801EE230`.

## Corrections and caveats

1. **The two prologues are dead, not load-bearing.** Both exports open with the
   `unaff_GQR0` / `in_f28`..`in_ps31_1` spills that wave 106 treated as an
   unrecoverable entry state. In `0x801EE230` the target slots (`local_8`,
   `local_18`) are never read; in `0x801E55C0` all four (`local_8`, `local_18`,
   `local_28`, `local_38`) are never read. Both epilogues repeat the matching
   `ldexpf` calls with the results discarded. This is the same construct that
   [`fn_801e74c0`] already documents, so the spills and the `ldexpf` pairs are
   dropped and the loss is recorded in each `# Confidence` line.
2. **The register-save folds.** `0x801EE230` prints its channel as a 32-bit
   `0x802DCD74` return, and `0x801E55C0` prints `(channel, result)` as the two
   halves of a `0x802DCD44` return. The halves follow the convention already
   used by [`fn_801edf40`] (high = `r3` = channel, low = `r4`): the caller of
   `0x801E55C0` at `0x801E7050` passes `(channel, result)` and the body indexes
   the control table with the high half.
3. **The integer/double chains.** Every `CONCAT44(0x43300000, x) ^ 0x80000000`
   minus a `r2-0x4D50` / `r2-0x4D18` double is the Metrowerks signed-integer
   conversion idiom (`(x as i32) as f64`); the same chain without the XOR
   minus `r2-0x4CF0` is the unsigned one. The Rust writes the conversions
   directly and keeps every `as f32` the export spells out, so the rounding
   order of the running sums and centres is preserved.
4. **`centX` loses its second argument.** In `0x801E55C0` the
   `"centX=%lf, centY=%lf"` call is printed with a single double, unlike the
   three neighbouring two-double lines. The Rust prints both centres and marks
   the second one `// inferred:`; the value is the one already stored at
   `0x803A1A90 + channel*4`.
5. **`0x801E11A0` loses its value argument.** In `0x801EE230` the format
   handshake calls `0x801E11A0` with only the channel in the export, but that
   function re-queues the word it is given into `+0x8B8`. The Rust passes the
   expected report id from the same `0x80345770` table entry it just compared
   against, which is the only in-scope value consistent with the `+0x8B8`
   write; this is `// inferred:` in the body.
6. **Two callbacks have no export.** `0x801EE230` hands `0x801EE110` and
   `0x801EE130` to [`fn_801e4a40`] as the classic-controller completion
   callbacks. Ghidra created no functions at those addresses (they sit between
   `FUN_801ee0b0` and `FUN_801ee150`), so the Rust passes the two addresses with
   an `// unknown:` note instead of inventing bodies.
7. **The zero-point stub keeps its dropped arguments.** The neutral-point
   state machine ends with `0x801DDaf0(&0x803A30D8, 4)`. The translated body
   [`wbc_set_zero_point_dummy_801ddaf0`] is the balance-board dummy that takes
   no arguments and prints its own name, so the Rust calls it as-is and records
   the dropped arguments at the call site.
8. **`0x803A1BAC` triple.** The three floats written when `+0x50` is armed are
   `0.0`, `-offset` and `sqrt(1 + tilt^2) / min(1, tilt)`. The roles of the
   three slots are left `// unknown:`; only the arithmetic is claimed.

## Deliberate skips

- **`0x801E1E20` (5,628 B).** The first WPAD command-record writer. Its body is
  thirteen `0x30`-byte stack records copied into the ring at `+0x164`, and the
  record payload is assembled from locals (`local_84`, `local_80`, `local_7c`,
  `local_78`, `local_74`, `local_6c`, ..., `local_60`) that the export never
  initialises — only the low half of `local_70` is written, through a
  `CONCAT22` over undefined bits. The payload, which is the entire point of the
  function, cannot be recovered; reproducing the queue mechanics with an
  invented record would be a fabricated translation.
- **`0x801E39C0` (4,224 B).** The second WPAD command-record writer, called by
  [`fn_801ee230`] for the extension-format change and by the rest of the
  extension handshake. It has the identical defect: thirteen `0x30`-byte
  records are copied from uninitialised stack unions (`local_70`, `local_d0`,
  `local_130`, `local_a4`, ..., `local_144`) and pushed into the same ring.
  [`fn_801ee230`] therefore calls it by address and passes the completion
  callback, exactly as the original does.

Both remaining entries are the same shape and the same defect: the command
*queue* is reconstructible, the command *records* are not in the export.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (2,792 + 1,896 =
  4,688 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- Neither address was in `ledger/functions.csv`; the four-entry untranslated
  list for the bucket was derived from it first.
- `node tools/check-lane-csv.mjs wave110` prints `OK` and reports
  `wave110-input.csv: 2 rows`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/runtime/input.rs` (the remaining errors in the run belong to other
  lanes).

## Ledger

Machine-readable rows: `ledger/agents/wave110-input.csv` (2 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
