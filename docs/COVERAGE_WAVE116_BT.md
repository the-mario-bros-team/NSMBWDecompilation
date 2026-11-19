# Coverage - Wave 116 (`runtime::bt`, the EXI block worker)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave116-bt.csv` names the exact Ghidra C export.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. The `0x80220000` bucket is shared with the SDP, IPC/EXI,
render and text code documented by waves 60, 69, 84, 92, 97, 102, 108 and 113;
this pass only touches the single entry those passes left open.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 1 | 744 | 1 | 0 | 0 |

## What this pass covers

The `0x80220000` bucket holds 216 functions. When this pass started exactly one
was untranslated: `0x80228644`, the register-level EXI block worker, which waves
108 and 113 recorded as a deliberate skip because its Ghidra export collapses to
`do { } while (true)`.

### EXI block worker (`0x80228644`, 744 bytes)

The export is not usable: Ghidra models the `TSTART` poll at `0x802287D4` as an
infinite loop and then removes the twelve blocks below it as unreachable
(`0x802287E0..0x80228910`). The bytes are intact in `work/dol/main.elf`, so this
pass disassembled the 186-instruction stream and reconstructed the routine from
it:

1. With `write != 0` the caller buffer is packed into one data word. The
   compiler emits an eight-byte loop plus a byte tail; both OR each byte into a
   single accumulator (`r0`, cleared once at `0x80228664`), and the shift count
   is `(3 - index) * 8` masked to the six bits `slw` uses. Byte `i` therefore
   contributes at bit `24 - 8 * (i & 3)` for `i & 4 == 0` and contributes
   nothing for `i & 4 != 0`, because the wrapped count lands in `32..63`. Every
   caller passes `length = 4`, where this is the plain big-endian word of the
   four bytes.
2. The accumulator is stored to the EXI channel-2 data register
   `0xCD006838`. An empty buffer still reaches that store with a zero word.
3. Both directions then write the command word
   `TSTART | (write << 2) | ((length - 1) << 4)` to `0xCD006834` and spin while
   bit `0` (`TSTART`) remains set.
4. With `write == 0` the data register is read once and unpacked into the
   buffer with the same shift table (`byte = word >> shift`, zero for counts of
   `32..63`).

The routine always returns `1` (`li r3,1` at `0x80228914`).

The command word is the standard EXI `CR` layout (`TSTART` bit 0, `RW` bit 2,
`TLEN` from bit 4), and the `0x14`-byte channel layout puts the channel-2 block
at `0xCD006828` (`+0` CSR, `+0xC` command, `+0x10` data), which is why this pass
names `0xCD006838` as the data register (`// inferred:`). The pre-existing
constant names `EXI_CONTROL_CD006828` and `EXI_STATUS_CD006834` are unchanged;
the polled bit is documented as `TSTART`. The four helpers that reach this
routine (`0x802289E8`, `0x80228B0C`, `0x80228C10`, `0x80228CF0`) still call it
by address, which is behaviourally identical to calling [`fn_80228644`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80228644` | `fn_80228644` | 744 | high |

## Deliberate skips

None. This was the last open entry of the `0x80220000` bucket, which is now
216 of 216 translated.

## Verification

- Every address and byte size matches `functions.tsv` (`80228644`, 744 bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/80228644_FUN_80228644.c`.
- The row has exactly one `pub unsafe fn fn_80228644` body in
  `src/runtime/bt.rs`.
- `0x80228644` was not in `ledger/functions.csv` before this pass, and no other
  Wave 116 lane claims it.
- `cargo check --message-format=short` reports no error and no new warning in
  `src/runtime/bt.rs`; the only `bt.rs` diagnostics are the pre-existing
  `unnecessary parentheses` warnings in the untouched L2CAP code.
- `node tools/check-lane-csv.mjs wave116` reports this lane clean.
