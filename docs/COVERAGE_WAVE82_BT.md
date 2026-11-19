# Coverage - Wave 82 (`runtime::bt`, L2CAP API and connection blocks)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the Ghidra
C export. All addresses are inside this lane's `0x80210000` bucket, which held
198 functions with 115 untranslated when this pass started.

This pass does not add gameplay code. It fills two contiguous holes in the
Broadcom L2CAP layer that waves 5, 6, 68 and 79 had left between them: the four
"send this primitive" entry points and the connection-block bookkeeping that
sits underneath them.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 28 | 4056 | 18 | 10 |

## What this pass covers

- **L2CAP primitive senders** (`0x80212C64..0x8021323C`, 6 bodies). The
  configuration-request, configuration-response, disconnect-request and
  disconnect-response calls share one shape: a trace line, the CCB lookup
  [`fn_80217f88`], the "no CCB" trace line on a miss, then one event
  (`0x17`/`0x18`/`0x19`/`0x1A`/`0x1B`) through the channel dispatcher. The
  idle-timeout setter `0x80213058` either writes the global timeout or the
  owning connection's `+0x58`; `0x8021323C` is a genuinely empty callback.
- **Connection timeout and window accounting** (`0x80214C18..0x80214DFC`,
  3 bodies). `0x80214C18` notifies only the children that installed a
  connect-timeout callback. `0x80214C88` is the state-driven timeout handler:
  teardown for states `1/2/3/5`, and for state `4` the ping-timeout callback,
  the security probe `0x802097BC` and the re-arm decision (`0x1E`, `1`, or no
  timer). `0x80214DFC` converts a queued packet's length into controller
  payload chunks, clamps them against the global load and the connection's
  `+0x36` window, and moves the granted amount into `+0x38`.
- **Connection table** (`0x8021544C..0x80216BC4`, 10 bodies). The `0x803D4C42`
  / `0x803D4C44` load pair, the retained-packet room test, the state-2 channel
  close pair, the segment-complete path, the empty-child timer, the `0x7E8`-byte
  table initialiser (which links ten CCBs), the event-code dispatcher, the
  four-block allocator, the `BD_ADDR` lookup and the `0x803D4C41` getter.
- **CCB / PSM / RFCOMM helpers** (`0x80217F88..0x80219594`, 5 bodies). The
  ten-CCB lookup by local channel id, the eight-slot PSM allocator, the
  disconnect-all sweep, the RFCOMM context initialiser and the port-timeout
  close scan.
- **RFCOMM frame and timer tails** (`0x8021D74C`, `0x8021E628`) plus the **SDP
  attribute search** (`0x8021F08C`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80212C64` | `fn_80212c64` | 184 | high |
| `0x80212D1C` | `fn_80212d1c` | 208 | high |
| `0x80212DEC` | `fn_80212dec` | 164 | high |
| `0x80212E90` | `fn_80212e90` | 164 | high |
| `0x80213058` | `fn_80213058` | 200 | high |
| `0x8021323C` | `fn_8021323c` | 4 | high |
| `0x80214C18` | `fn_80214c18` | 112 | medium |
| `0x80214C88` | `fn_80214c88` | 372 | medium |
| `0x80214DFC` | `fn_80214dfc` | 264 | medium |
| `0x8021544C` | `fn_8021544c` | 20 | high |
| `0x80215630` | `fn_80215630` | 108 | medium |
| `0x8021569C` | `fn_8021569c` | 128 | medium |
| `0x8021571C` | `fn_8021571c` | 108 | high |
| `0x80215788` | `fn_80215788` | 220 | high |
| `0x80215864` | `fn_80215864` | 68 | medium |
| `0x802158A8` | `fn_802158a8` | 276 | high |
| `0x8021675C` | `fn_8021675c` | 76 | high |
| `0x802168D8` | `fn_802168d8` | 212 | high |
| `0x80216AAC` | `fn_80216aac` | 136 | high |
| `0x80216BC4` | `fn_80216bc4` | 16 | medium |
| `0x80217F88` | `fn_80217f88` | 92 | high |
| `0x80217FE4` | `fn_80217fe4` | 76 | high |
| `0x802183D0` | `fn_802183d0` | 108 | high |
| `0x8021879C` | `fn_8021879c` | 80 | high |
| `0x80219594` | `fn_80219594` | 152 | high |
| `0x8021D74C` | `fn_8021d74c` | 168 | medium |
| `0x8021E628` | `fn_8021e628` | 136 | medium |
| `0x8021F08C` | `fn_8021f08c` | 204 | medium |

## Why the ten `medium` rows are `medium`

- `0x80214C18`, `0x8021569C` and `0x80215864` call a callee whose first operand
  Ghidra dropped (`FUN_80216B34()` / `FUN_80216AAC()` with no argument list).
  The sole caller of each (`0x8020A6C0`, and the three `0x80216AAC` callers)
  passes a handle or a `BD_ADDR`, which is what the body needs; the bodies say
  so and the recovery is limited to restoring that argument.
- `0x80214C88` prints `FUN_801FA320(param_1 + 0x10, 2)` with two arguments while
  the timer arm takes three. The missing delay is the value the export computes
  into `iVar1` (`0x1E`, `0xFFFF` or `1`) and tests immediately before the call,
  so no other value is plausible.
- `0x80214DFC` and `0x80215630` are literal, but the `+0x36`/`+0x38`/`+0x50`
  field roles are inferred from the teardown path ([`fn_80217de4`]) and from the
  reassembly users, not from a symbol.
- `0x80216BC4` returns a byte that is initialised to `0` and never written with
  a non-zero value; its role cannot be named.
- `0x8021D74C` calls the shared finisher with two printed arguments; the third
  is the packet block the routine has just modified and the finisher's own
  signature requires it.
- `0x8021E628`'s free call is printed without an argument. As in
  [`fn_80217de4`], the dropped value is the node just popped from the queue.
- `0x8021F08C`'s control flow is exact, but the SDP node tags (`1`/`3`) and
  types (`3`/`6`) are not named by any export in this bucket.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave82-bt.csv`.

- `0x80213240` (4916 bytes) - the channel event dispatcher itself. It is a huge
  switch over every event code in this layer and needs its own pass.
- `0x8021471C` (340), `0x80214870` (568) - call `FUN_80216AAC()` / `FUN_80216B34()`
  with the argument list dropped and then use the result as a pointer and as a
  state; the register that held the argument is ambiguous in both exports.
- `0x80215368` (228), `0x80215460` (464) - open with `FUN_802DCD70()` /
  `FUN_802DCD68()` and then use the fake return as a pointer. Which half of that
  value is `r3` cannot be settled from the C, the same reason wave 79 skipped
  `0x80201D94`.
- `0x802167AC` (300) - takes its boolean from `FUN_802DCD74()` assigned to an
  `int`; unlike `0x8021CE04`, the export does not show whether that is the low
  or the high half, so the parameter cannot be placed honestly.
- `0x80216C98` (252) - consumes *two* registers from the `0x802DCD74` result and
  three further parameters after Ghidra has already printed five; the argument
  order is not recoverable.
- `0x80213120` (256) - compares against `unaff_r2 - 0x4C70`, i.e. an `r2`-relative
  base the decompiler never materialised. The table base would be a guess.
- `0x8021948C` (104), `0x8021CDA0` (44), `0x8021CDCC` (56) - each calls
  `FUN_80219E40()` with *both* arguments dropped (the translated
  [`fn_80219e40`] needs a channel block and a channel index). The values are
  unrecoverable from the C.
- `0x80214F04` (716), `0x802151D0` (408), `0x802159BC` (816), `0x80215CEC`
  (2672) - the signal sender, the window recompute and the two large
  reassembly/hold routines were not opened this pass; the new bodies call them
  by documented address instead.
- `0x802128A0` (564), `0x80212AD4` (400) - the outgoing-connection starters are
  large and were left for a later pass.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 28 bodies total
  4056 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv` (all 28 were among the
  bucket's 115 untranslated entries).
- `cargo check --message-format=short` reports no errors in
  `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave82` reports `wave82-bt.csv: 28 rows` and
  `OK` for the lane.
