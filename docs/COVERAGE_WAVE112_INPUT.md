# Coverage - Wave 112 (`runtime::input`, the two WPAD command-record writers)

## How to read this file

1. **Summary** gives the per-file totals and the closing inventory of the
   `[0x801E0000, 0x801F0000)` bucket.
2. **What this pass covers** describes the two bodies, the record sets each one
   queues, and why wave 110's skip no longer applies.
3. The **function table** lists both bodies in address order with the byte count
   re-read from `work/ghidra/main-gb/export/functions.tsv` and the confidence
   carried by each doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** states that none remain.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for. Both rows are
`translated`; both are `medium` because a handful of record bytes and two code
addresses are named by inference only. Names stay `fn_<addr>` because the image
has no symbol table for this layer; the doc comments carry the roles and repeat
the `// observed:` / `// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 2 | 9,852 | 0 | 2 |

Bucket inventory (`[0x801E0000, 0x801F0000)`):

| Item | Count |
|---|---:|
| Functions in the bucket | 103 |
| Untranslated before this pass | 2 |
| Selected and translated by this pass | **2** |
| Deliberately skipped | 0 |
| Untranslated in the bucket after this pass | **0** |

## What this pass covers

Waves 106 and 110 left exactly two entries of the band, both listed as skips in
`docs/COVERAGE_WAVE110_INPUT.md` because their command records were believed to
be assembled from stack slots the export never initialises. Re-reading the two
exports against the queue consumer [`fn_801ddd20`] shows that reading is wrong:

- the record's payload length is the halfword at `+0x1A`, and the consumer
  copies exactly that many bytes from `+4`; the slots wave 110 called "missing
  payload" lie past that length;
- a `0x16` extension write additionally carries its own size byte at `+8`, so
  only `9 + size` bytes of the 21-byte record are data;
- the seven-byte template that one record writes is not a stack slot at all: it
  is the static `r2-0x4DA0` block, read at run time and patched with the global
  sampling mode.

With that, every record the two functions queue is reconstructible:

- **Extension enable/disable writer (`0x801E1E20`, 5,628 B).** Maps a mode word
  onto a fixed record set for the `0x04A20xxx` extension registers. Mode `0`
  clears (five records: `0x19`→`04`, `0x16`→`0x04A20001`←`01`,
  `0x16`→`0x04A20009`←`00`, `0x14`→`00`, `0x15`→`00` with the completion),
  modes `1`/`5` enable (seven records, including the seven-byte `r2-0x4DA0`
  template written to `0x04A20001`), modes `2`/`3` queue one `0x19` record and
  mode `4` writes `0x04A20008`←`01`. The multi-record modes test the ring for
  room (`count + n <= capacity - 1`) before queueing; the single-record modes let
  the ring push reject the record. The `+0x83C` word gates the clear path.
- **Extension activation writer (`0x801E39C0`, 4,224 B).** Called by
  [`fn_801ee230`] for the format change and re-queued by [`fn_801e5460`]'s retry
  ladder. Mode `0` clears the `+0x8CC` latch and queues three records (`0x13`,
  `0x1A` with completion `0x801E3980`, `0x15` with the caller completion); any
  other mode that differs from the latch stores it, sets `+0x994` and queues
  eight records — `0x13`, `0x1A`, then five `0x16` writes to the `0x4B000xx`
  bank (`0x4B00030`←`01`, `0x4B00000`← nine bytes from
  `0x802F6CD8 + (kind-1)*9`, `0x4B0001A`← two bytes from
  `0x802F6D08 + (kind-1)*2`, `0x4B00033`← the mode byte, `0x4B00030`←`08` with
  completion `0x801E3980`) and a final `0x15` with the caller completion. The
  two table rows are selected by the global extension kind `r13-0x4BC3`.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `0x801E1E20` | `fn_801e1e20` | extension enable/disable register sequence | 5,628 | translated | medium | `work/ghidra/main-gb/export/801e1e20_FUN_801e1e20.c` |
| `0x801E39C0` | `fn_801e39c0` | per-kind extension activation sequence | 4,224 | translated | medium | `work/ghidra/main-gb/export/801e39c0_FUN_801e39c0.c` |

Totals: **2 translated**, 0 partial, 0 stubs; 0 high / 2 medium / 0 low;
9,852 bytes. Address range `0x801E1E20`–`0x801E39C0`. The
`[0x801E0000, 0x801F0000)` bucket is closed: 103 of 103 entries translated.

## Corrections and caveats

1. **The wave-110 skip reason was a misreading, recorded here deliberately.**
   The exports do contain `CONCAT13(4, ...)` / `local_x & 0xFFFFFF` stores whose
   remaining bytes are never written, which is what wave 110 saw. Those bytes
   are outside the record's declared payload: the one-byte records declare a
   length of `1` at `+0x1A`, and the `0x16` writes declare `1`, `7`, `9` or `2`
   data bytes at `+8`. The Rust reproduces every byte the producer writes and
   leaves the rest zero, which is the same treatment [`fn_801e12c0`] and
   [`fn_801e1630`] already document.
2. **The argument halves.** Both functions are `void FUN(a, b, callback)` in the
   export but open with the `0x802DCD6C` / `0x802DCD68` register-save fold,
   whose 64-bit return carries `(r3, r4)` = `(channel, mode)`. The Rust takes
   both explicitly, following [`fn_801e4a40`]; the doc blocks say so.
3. **The callback's first argument.** Both tails call
   `(*param_3)(iVar6, iVar5)` / `(*param_3)(iVar7, iVar5)`, where Ghidra has
   already reused those registers for ring indices. The Rust passes `channel`,
   the only value the callers' `(channel, result)` convention accepts; this is
   `// inferred:` in the bodies.
4. **`0x801E3980` is a label, not a function.** `FUN_801e39c0` stores
   `&LAB_801e3980` at `+0x2C` of two records. Ghidra created no function at that
   address (it sits inside `FUN_801e3680`), so the body passes the address with
   an `// observed:` note instead of inventing a body.
5. **The two read-only tables are read at run time.**
   `0x802F6CD8 + (kind-1)*9` and `0x802F6D08 + (kind-1)*2` have no export and no
   text dump in `work/ghidra`, so the body reads them from the image exactly as
   the original does rather than guessing their contents.
6. **The `+0x28` value word stays zero.** The consumer stores `+0x28` at
   `+0x850` for the `0x15` records; every producer here leaves it at zero, which
   the zeroed record already provides.
7. **The reported link word.** When no record is queued (a mode already latched,
   an unknown mode, or the `+0x83C == 0` clear), the result is the raw `+0x8BC`
   word rather than a normalised `0`; the bodies transcribe that unchanged.
8. **`0x4B000xx` roles.** The register bank the activation sequence writes has
   no name in the export; only the addresses, the payload sizes and the table
   strides are claimed, so the roles are `// unknown:` in the body.

## Deliberate skips

None. Both remaining entries of the bucket are reconstructed, and the bucket is
closed.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (5,628 + 4,224 =
  9,852 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- Neither address was in `ledger/functions.csv`; the two-entry untranslated list
  for the bucket was derived from it first (103 bucket entries, 101 rows).
- `node tools/check-lane-csv.mjs wave112` prints `OK` and reports
  `wave112-input.csv: 2 rows`.
- `cargo check --message-format=short` reports no error in
  `src/runtime/input.rs` and no warning at either wave-112 body (the two
  `direct cast of function item into an integer` warnings in the file belong to
  earlier passes).
- The module header gained a nineteenth-pass item and the promotion note now
  records that the two writers are landed.

## Ledger

Machine-readable rows: `ledger/agents/wave112-input.csv` (2 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
