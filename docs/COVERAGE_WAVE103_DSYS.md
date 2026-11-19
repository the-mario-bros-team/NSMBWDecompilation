# Coverage - Wave 103 (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and notification helpers around it in the main `DOL`
(bucket `0x800E0000`, 189 functions in total).

Each row below is one `pub unsafe fn fn_<address>` in `src/engine/dsys.rs`. The
doc block above that body cites the Ghidra export it came from, the byte count
from `work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a register-pair recovery was
  inferred and is marked `// inferred:` or `// unknown:` in the body;
- `low` means the instruction sequence is certain but the callee's argument list
  is not recoverable from the export.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 15 | 6424 | 0 | 15 |

The file already held 127 bodies (18 `dSys_c` start-up, 30 Wave 75, 28 Wave 89,
26 Wave 93 and 25 Wave 97); this pass adds 15 more, for 142 bodies total. The
bucket still has 7 untranslated entries, all listed under **Deliberately
skipped** below.

## What this pass covers

- **Notification pose push, `0x800E20A0` (1 body).** The `0x802DCD6C` pair is
  read with the object in its **low** word (the opposite of
  `fn_800eee70`), the two incoming floats are biased by `+0xAC`/`+0xB0`, the
  ten-entry table at `0x802F2EE0` gates one `0x800CEA70` dispatch and one
  `0x80198C30` sound event, and the band pass `fn_800e1f80` runs on the high
  word.
- **Packed display metrics, `0x800E28A0` (1 body).** The `u16` fields `+4`/`+6`
  of the display record are widened with the `2^52` bias, `0x801DD1D0` selects
  whether the second value is re-derived from the first, and the four-entry
  table at `0x802F2FC8` applies the sign-flipped correction. The result is
  returned as one `u64` of two `f32` halves.
- **Mesh strip decoders, `0x800E2DE0..0x800E3160` (4 bodies).** Two wrapper
  passes over the `0x802DCD68` pair (three-float and two-float stride) plus the
  two vertex transformers they call. The transformers evaluate the three
  quadratic weights with `0x800E2D70`, `0x800E2D90` and `0x800E2DB0`, combine
  them with the two scale pointers of the `0x802DCD74` pair through
  `0x800E2DC0`, write two floats per vertex and return the advanced cursor.
- **Sprite slot reset, `0x800E3BA0` (1 body).** The eight slot blocks, the
  `0x21..0x3F` mask sweep with `0x802DD1EC`, and the stage effect words that are
  OR-ed or copied into the two mask halves.
- **Effect solver records, `0x800E6090..0x800E63A0` (2 bodies).** The mode-chain
  walk that accumulates frame time until the `0xF` terminator, and the step
  accumulation that widens a two-float window.
- **Resource-name scan, `0x800EEAE0` (1 body).** The two passes over the name
  table, the nine-byte `R[WF]???[WF]` test, the four sub-record checks and the
  `0x800F1A00` handle buffer.
- **Two-slot variant rebuild, `0x800EEFF0` (1 body).** The slot choice derived
  from the count-leading-zeros of the pair's low word, the six lookups, the
  record resets and the `0x2C`/`0x2B` sound selection.
- **Effect selector, `0x800EF570` (1 body).** The descriptor probe, the
  `0x8025C660`/`0x8025C560` record build, the selector write and the
  create/release toggle at `+0xB7C`.
- **Keyed track family, `0x800EF770..0x800EF940` (3 bodies).** The track rebuild
  whose second loop continues at the first loop's count, and the two frame snaps
  that wrap a frame below the threshold to `position - bias`.

### Evidence that unblocked the pass

1. The `ldexpf` prologue at the head of `0x800E6090` and `0x800E63A0` is again
   the dead `f27..f31` save; the real body starts after it.
2. `0x800E2DE0`/`0x800E2F10` print six values for the five GPRs of their callee.
   The first value is the incoming `f1` `extraout_f1`; the remaining five map to
   `r3..r7`, which makes `(junk, junk, src, dst, count)` the honest callee
   signature and `step` an explicit `f64`.
3. `0x800EF770`'s second loop was called ambiguous in Wave 97, but the first
   loop's condition literally assigns `iVar3 = *(int *)(param_1 + 0x7cc)` on
   every test, so the second loop starts at that count.
4. `0x800E28A0`'s `dVar4` reuse resolves once the branch is read as "second
   value": the final `CONCAT44` consumes exactly the two post-branch locals.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E20A0` | `fn_800e20a0` | 380 | medium |
| `0x800E28A0` | `fn_800e28a0` | 420 | medium |
| `0x800E2DE0` | `fn_800e2de0` | 304 | medium |
| `0x800E2F10` | `fn_800e2f10` | 296 | medium |
| `0x800E3040` | `fn_800e3040` | 276 | medium |
| `0x800E3160` | `fn_800e3160` | 276 | medium |
| `0x800E3BA0` | `fn_800e3ba0` | 892 | medium |
| `0x800E6090` | `fn_800e6090` | 500 | medium |
| `0x800E63A0` | `fn_800e63a0` | 572 | medium |
| `0x800EEAE0` | `fn_800eeae0` | 908 | medium |
| `0x800EEFF0` | `fn_800eeff0` | 576 | medium |
| `0x800EF570` | `fn_800ef570` | 396 | medium |
| `0x800EF770` | `fn_800ef770` | 276 | medium |
| `0x800EF890` | `fn_800ef890` | 176 | medium |
| `0x800EF940` | `fn_800ef940` | 176 | medium |

## Field notes

- `CONCAT44(0x43300000, x)` is the CodeWarrior `2^52` widening. Where the export
  uses it only to recover a signed integer, the body writes the equivalent
  `x as i32 as f64` cast and says so in the confidence line.
- `0x800E3BA0` walks two arrays with different strides: the word array advances
  eight words per slot, the pair array sixteen, while the byte arrays advance by
  eight. The nested loop in the body keeps all three counters explicit instead of
  flattening them into one index.
- `0x800EF890`/`0x800EF940` are the only call sites that print `0x80166310` and
  its `0x80167B40` peer with a leading `undefined8` before the double. That mixed
  layout is modelled by `call_u64_f64_u32x2` and marked medium rather than
  reconciled with the `(f64, u32, u32)` sites elsewhere.
- `0x800E2D70`/`0x800E2D90` live in `src/engine/frame.rs`, so the decoders call
  them by absolute address through `call_f64_arg_ret` instead of naming another
  lane's functions.

## Deliberately skipped

7 bucket entries remain. Each was read in full before being left out.

- **Head `psq_l` operand pair with no visible source, 3 bodies** - `0x800E5820`,
  `0x800E5AD0`, `0x800E6680`. Each starts with one or five paired-single loads
  whose GQR dequantisation is in the export but whose base address is inside the
  removed head block. The dequantised pair is then stored into a stack record, so
  reconstructing the body would require inventing where the load reads from.
- **Varying printed arity of `0x802DCC4C`, 1 body** - `0x800E5DD0`. The same
  callee appears with one, two, three and six printed arguments in a single body,
  and two of the results overwrite the locals block in place, so neither the call
  shape nor the field assignment can be pinned down.
- **Byte-for-byte stack structure copy, 1 body** - `0x800E0540` (1756 bytes).
  Three 0x3FC0-byte overlapping copies with per-field unpacking; the source
  layout is not proven and a byte-for-byte body would be speculation.
- **Eleven-pair float body, 1 body** - `0x800E9130` (1036 bytes). Seven live
  `in_ps*` pairs feed a 0x1C4-byte stack frame; the same missing `psq_l` source
  as above applies, and the pairing of the seven inputs is not provable.
- **Eighteen-pair float body, 1 body** - `0x800EC3B0` (1836 bytes). Same
  blocker at a larger scale (eleven pairs, 1836 bytes).

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/dsys.rs`.
- `node tools/check-lane-csv.mjs wave103` reports `OK` (15 rows for this lane).
