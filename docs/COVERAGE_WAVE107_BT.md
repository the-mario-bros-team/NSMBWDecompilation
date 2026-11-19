# Coverage - Wave 107 (`runtime::bt`, the RFCOMM mux dispatchers and the HID fragmenter)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave107-bt.csv` names the exact Ghidra C export.

All seven addresses are in the `0x80210000` bucket, which holds 198 functions.
Wave 101 landed 25 of the 34 untranslated entries and listed the remaining nine
under *Deliberate skips*; this pass claims seven of those nine, leaving two
large dispatchers open.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles that the evidence does not prove are marked
rather than named.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 7 | 8720 | 0 | 7 |

## What this pass covers

### HID report fragmenter (`0x802122B0`)

The single entry point behind [`fn_8021022c`]. It indexes the sixteen `0x34`-byte
records of [`BTM_POOL`], refuses to start when the record's `+0x11` byte has bit
`0x20` set, selects the control (`+0x14`) or interrupt (`+0x16`) channel id from
the report type, and then fragments the caller's buffer:

- a supplied buffer that still fits (`+2` length `<=` the `+0x18` ceiling minus
  one) is filled in place; its `+4` offset is decremented by one first,
- a larger buffer is split: each fragment copies `ceiling - 1` bytes with
  `0x80004364` and the source's `+2`/`+4` pair is advanced,
- a fresh `2`-class buffer starts at offset `9` and carries a one-byte type at
  `(buffer + offset) + 8`,
- types `9`, `4` and `10` get the extra trailer byte(s), and every fragment is
  handed to [`fn_80212f34`] (the L2CAP data write).

Return values are `0` success, `3` allocation failure, `5` unknown type and `8`
when the record went busy or submission failed.

### RFCOMM mux receive path (`0x8021A704`, `0x8021DE28`)

The mux channel block is now named [`RF_MCB`] (`0x803D57C4`, 0x78 bytes):

| Offset | Meaning |
|---|---|
| `+0x10` | timer context (block pointer written before arming) |
| `+0x18` | ten-byte queue head |
| `+0x24` | port-index table, one byte per DLCI |
| `+0x62` | six-byte peer `BD_ADDR` |
| `+0x68` | `u16` L2CAP channel id |
| `+0x6A` | `u16` MTU (`0x29A` on connect) |
| `+0x6C` | mux state `0..6` |
| `+0x6D` | role: `0` when this side initiated |
| `+0x6E`/`+0x6F` | unnamed bytes cleared with the restart flag |
| `+0x70` | restart-after-disconnect flag |
| `+0x71` | running flag |

[`fn_8021a704`] is the L2CAP data callback: it resolves the LCID through the
ten-slot table, classifies the frame with [`fn_8021d8b4`], and either parses a
multiplexer frame (DLCI `0`), forwards a mux command, or resolves the port and
delivers data event `4`. [`fn_8021de28`] is the parser itself: it fills the
receive context at [`RFCOMM_CONTEXT`] (DLCI at `+0`, EA at `+1`, C/R at `+2`,
masked control at `+3`) and decodes the eight command formats
(`0x80` PN, `0x50` RPN, `0x20` test, `0x10` flow control, `0x60`/`0xA0`/`0x90`/
`0xE0`), then builds a three-byte empty response through [`fn_8021d0d8`].

The `CONCAT*n(...) & mask` folds in that export are reproduced as byte stores,
because the masks zero exactly the bits the composed value does not carry: for
example `CONCAT31(CONCAT21(CONCAT11(old0, p1), p1), p2) & 0xff0ff03f` becomes
"byte 0 keeps its old value, byte 1 keeps the low nibble of `p1`, byte 2 the high
nibble, byte 3 keeps `0x3f`".

### Mux state machine (`0x8021AAF4`) and port state machine (`0x8021B738`)

[`fn_8021aaf4`] runs the seven mux states (`0` idle, `1` wait-conn-cnf,
`2` configure, `3` SABME-wait-UA, `4` wait-SABME, `5` connected, `6`
DISC-wait-UA). Two behaviours are worth naming:

- state `0` event `6` sets the MTU to `0x29A`, calls [`fn_802128a0`] with PSM `3`
  and stores the returned channel id at `+0x68`; a `0` result is reported
  through [`fn_802187ec`] instead of entering state `1`,
- state `1` shares the post-switch block: event `9` with a non-zero status
  reports the failure, event `9` with `0` sends the configuration request built
  in a 0x3C-byte block (`+2 = 1`, `+4 = 0x069B`, `+0x20 = 0`), and events `5`/`7`
  release the channel.

[`fn_8021b738`] runs the six port states (`0` closed, `1` orig-wait-sec,
`2` sabme-wait-ua, `3` term-wait-sec, `4` opened, `5` disc-wait-ua) and is a flat
`switch` per state, with every event id, trace string and callee literal. The
opened handler's data event (`0x0E`) grants credits when the peer runs in
credit mode, the frame fits the window and the flow-pause byte is clear, then
builds the UIH frame with [`fn_8021d0d8`].

### Mux timer (`0x8021E4D0`)

Compares the static MCB's stored address with the argument: a free or foreign
block is cleared, addressed, queue-initialised and armed for `0x3C` ticks; a
block for the same peer is only stopped. It returns the block (or `0` for a
foreign peer), which is exactly what [`fn_8021a23c`] stores in the LCID table.

### Port-negotiation indication (`0x80218F20`)

Resolves the port (allocating the first unbound one when needed, which also
records the port index in the MCB's index table), copies the nine-byte
descriptor to `port + 0x51` and submits a PN response. The no-port fallback is
the original's store through a null port base: the export prints absolute
addresses `0x51..0x59` and the body reproduces those nine stores literally.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802122B0` | `fn_802122b0` | 668 | medium |
| `0x80218F20` | `fn_80218f20` | 336 | medium |
| `0x8021A704` | `fn_8021a704` | 672 | medium |
| `0x8021AAF4` | `fn_8021aaf4` | 2652 | medium |
| `0x8021B738` | `fn_8021b738` | 2476 | medium |
| `0x8021DE28` | `fn_8021de28` | 1572 | medium |
| `0x8021E4D0` | `fn_8021e4d0` | 344 | medium |

## Why the seven rows are `medium`

- `0x802122B0` names two pool fields the module header does not: `+0x18` (the
  fragment ceiling) and `+0x11` bit `0x20` (the "busy/abort" bit). Both are
  `// unknown:`/`// inferred:` from their only reader.
- `0x80218F20` cannot mark the null-port path as unreachable and instead
  reproduces the literal stores to `0x51..0x59`; that is guest behaviour, but it
  reads like a bug, so the row is not `high`.
- `0x8021A704` uses receive-context bytes `+4` and `+5` whose producers are not
  in this lane, and forwards the credit-restore call with `r3` only (the export
  does not recover that call's second argument).
- `0x8021AAF4` and `0x8021B738` recover three arguments the export prints with a
  shorter list: `fn_8021aad8`'s table key (the cid just stored),
  `fn_8021e9a8(port)` and `fn_8021d0d8(chan, dlci, packet)`. Each is proven by
  the callee's own body.
- `0x8021DE28` transcribes `CONCAT*n` folds into byte stores and supplies the
  second vararg of the `"Illegal MX Frame ea %d len %d"` call, which Ghidra
  dropped.
- `0x8021E4D0` recovers its address argument from the six-byte `memcmp`/`memcpy`
  and from the only call site, because Ghidra folded the `0x802DCD74` save pair
  into a 32-bit variable in that one export; the role byte is the `extraout_r4`
  store at `+0x6D`.

## Deliberate skips

Two of the bucket's nine former holes stay untranslated and have no CSV row.

- `0x80215CEC` (2672) - the L2CAP ACL data dispatcher. It is a twenty-case
  switch over the remote command code that builds a reply for every case and
  keeps per-case stack buffers whose roles (`local_5e`, `local_5a`, ...) are not
  pinned down by any translated neighbour. Translating it needs the L2CAP
  option-encoding rules first; the bodies added here call it by address through
  [`L2CAP_DATA_80215CEC`].
- `0x80213240` (4916) - the L2CAP channel event dispatcher. At 4.9 KB it is the
  largest function of the bucket and mixes the state machine, the configuration
  option walker and the trace paths; waves 97/101/102 also left it open, and the
  translated bodies reach it through [`L2CAP_EVENT_80213240`]. It is the natural
  first target for a dedicated pass.

Both addresses remain reachable by address, so no translated body depends on an
untracked stub.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the seven bodies
  total 8720 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all seven came from the
  bucket's untranslated list, and all seven were re-checked against
  `ledger/agents/wave107-*.csv` for cross-lane duplicates.
- `cargo check --message-format=short` produces no error in `src/runtime/bt.rs`
  and no new warning there; the only `bt.rs` diagnostics are the ten
  pre-existing `unnecessary parentheses` warnings at lines `5215..5857`, which
  belong to the untouched wave-6 L2CAP code. The crate-wide run still stops on
  one error outside this lane
  (`src/game/player.rs`: `crate::game::worldmap::wm_attach_complete` not yet
  defined), which belongs to the `game::worldmap` lane and does not involve
  `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave107` prints `wave107-bt.csv: 7 rows` and
  `OK`.
