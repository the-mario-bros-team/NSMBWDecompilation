# Coverage - Wave 112 (`runtime::bt`, the L2CAP data and event dispatchers)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave112-bt.csv` names the exact Ghidra C export.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles the evidence does not prove are marked rather
than named.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 2 | 7588 | 0 | 2 | 0 |

## What this pass covers

The `0x80210000` bucket holds 198 functions. When this pass started exactly two
were untranslated: the two entries that wave 107 recorded as deliberate skips.
Both reconstructed cleanly, so the bucket is now closed and the lane has no
remaining body.

### L2CAP ACL data dispatcher (`0x80215CEC`, 2672 bytes)

The ACL data entry used by [`fn_802159bc`] for each reassembled payload. It
walks `{code, ident, length_lo, length_hi, body}` records and switches on the
Bluetooth signalling command code:

- `1` command reject: reason `1` logs the local handle and the rejected MTU;
  reason `2` resolves the channel by the remote CID and feeds event `3`,
- `2` connect request: resolves the PSM block with [`fn_80218040`], allocates a
  CCB with [`fn_80217cd8`], stores the identifier, service block and source CID,
  then feeds event `10`,
- `3` connect response: records the destination CID, result and flags in the
  shared scratch, then feeds event `0x0B`, `0x0C` or `0x0D` according to the
  result,
- `4` configuration request: decodes the MTU, flush-timeout and QoS options,
  counts unknown options and either feeds event `0x0E` or builds the
  configuration reject through [`fn_802175e0`],
- `5` configuration response: decodes the same option list and feeds event
  `0x0F` or `0x10`,
- `6` disconnect request: feeds event `0x11` or answers through
  [`fn_80217868`],
- `7` disconnect response: feeds event `0x12`,
- `8` echo request: echoes through [`fn_80217a98`] after the signed
  `length < limit - 12` test,
- `9` callback event: calls and clears the `+0x54` callback,
- `0x0A` reply path: forwards to [`fn_80217c34`],
- `0x0B` unused; anything else sends a command reject with reason `0`.

The stack scratch is now explicit. The export names it `auStack_98` with
`local_88` at `+0x10`; the option walker writes the MTU flag/value at `+2`/`+4`,
the QoS flag at `+6` and its 24-byte value at `+8`, the flush flag/value at
`+0x20`/`+0x22`, and the 11-byte QoS variant at `+0x24` onwards. Those offsets
are the ones [`fn_802180f8`] consumes, which is what proves the layout. The
`+0x10` struct is passed by pointer for events `0x0E`/`0x0F`/`0x10`; the other
events receive the scratch base, matching the printed `auStack_98` /
`&local_88` distinction.

Two limits are preserved rather than "fixed": the unknown-option test compares
the option size against the whole command length, not the remaining bytes, and
the continuation bit (`0x80`) ends the walk without setting the reject flag.
`local_88` is not written in the request case, so the original passes stale
stack bytes there; the Rust scratch is zeroed instead, which is noted in the
body.

### L2CAP channel event dispatcher (`0x80213240`, 4916 bytes)

The nine-state machine behind every translated channel helper. `ccb + 4` is
the state and the second argument is the event code; the service callbacks are
the nine pointers at `ccb + 0x30`:

| `+0x30` offset | Callback |
|---|---|
| `+0x04` | connect indication |
| `+0x08` | connect confirm |
| `+0x0C` | connect pending |
| `+0x10` | configuration indication |
| `+0x14` | configuration confirm |
| `+0x18` | disconnect indication |
| `+0x1C` | disconnect confirm |
| `+0x20` | connect timeout |
| `+0x24` | data indication |

State numbers and names come from the trace strings in the export: `0` CLOSED,
`1` ORIG_W4_SEC_COMP, `2` TERM_W4_SEC_COMP, `3` W4_L2CAP_CON_RSP,
`4` W4_L2CA_CON_RSP, `5` CONFIG, `6` OPEN, `7` W4_L2CAP_DISC_RSP and
`8` W4_L2CA_DISC_RSP. The body keeps every transition, timer arm/cancel
(`0x801FA320`/`0x801FA390`), payload free (`0x801F7240`), queue push
(`0x801F7670`) and callback invocation literal, including the three slots the
export tests for null (`0x0C`, `0x1C`, `0x20`).

The connect-start paths call [`fn_8020815c`] with the completion callback at
`0x80214AA8` ([`fn_80214aa8`]). State `2` is the only state that falls into the
common event switch printed after the outer `switch`; the body keeps that
structure as a separate match arm rather than duplicating it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80213240` | `fn_80213240` | 4916 | medium |
| `0x80215CEC` | `fn_80215cec` | 2672 | medium |

## Deliberate skips

None. The two entries wave 107 listed as deliberate skips (`0x80215CEC` and
`0x80213240`) are the two bodies added here, and both are complete. The
`0x80210000` bucket no longer has a reconstructible body left for this lane.

The other open entries recorded in earlier waves belong to different buckets
and were not in scope: `0x80228644` (empty EXI loop) and `0x8022FA70` (console
sink) from `docs/COVERAGE_WAVE108_BT.md`.

## Verification

- Every address and byte size matches `functions.tsv`; the two bodies total
  7588 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; both came from the
  bucket's two-entry untranslated list.
- `cargo check --message-format=short` reports no error or new warning in
  `src/runtime/bt.rs`; the only `bt.rs` diagnostics are the ten pre-existing
  `unnecessary parentheses` warnings in the untouched wave-6 L2CAP code.
  Errors elsewhere in the crate belong to other lanes.
- `node tools/check-lane-csv.mjs wave112` reports this lane clean.
