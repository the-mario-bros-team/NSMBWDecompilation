# Coverage - Wave 102 (`runtime::bt`, SDP server responses, render/text helpers and six tail branches)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave102-bt.csv` names the exact Ghidra C export.

All addresses are in the `0x80220000` bucket, which holds 216 functions and had
32 untranslated entries when this pass started. This pass claims 25 of them.
After the merge, seven remain; they are listed under *Deliberate skips*.

The bucket is not one subsystem. Waves 42/47/60/69/84/92/97 used it for the
EXI/IOS service, the SDP database, the render/palette objects and the EXI
console. This pass closes the two functional clusters that were still open —
the SDP server response builders and the text formatters — and takes the six
4-byte tail branches that wave 97 declined.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles that the evidence does not prove are marked
rather than named.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 25 | 10584 | 2 | 23 |

## What this pass covers

### SDP attribute store and server responses (`0x80220524..0x80222AA8`, 4 bodies)

| Address | Role |
|---|---|
| `0x80220524` | attribute deleter: finds a record by handle, drops one `0x0C`-byte descriptor, shifts the rest, moves every value pointer above the hole down by the removed length and compacts the `0x138`-byte value area |
| `0x80221440` | recursive request decoder: turns one SDP data element into a node `{ next, tag, type/len, value }` taken from the database's node area at `database + 0x68`, expands sequences of type `6`/`7` and links the result into the record or the parent container |
| `0x80222698` | ServiceSearchResponse (PDU id `3`): parses the UUID pattern with `0x802237F0`, collects up to `0x14` handles with `0x8021FA94`, clamps the PDU to `(MTU - 12) / 4` records and writes one big-endian `u32` per handle plus the continuation state |
| `0x80222AA8` | ServiceAttributeResponse (PDU id `5`): parses the record handle, the maximum byte count and the id sequence with `0x80223B40`, rebuilds the attribute sequence at `ccb + 0x26` with `0x8021FD04`/`0x80223560`, chooses the `0x35`/`0x36` header by size and sends the continuation-tail packet |

Both response builders share the continuation protocol: the request carries a
`u8` length plus that many bytes, the CCB remembers the next record index or
byte offset in the `u16` at `+0x476`, and the response writes `0` (complete) or
`2` plus the new state (more). The four bodies use the already-named record
constants (`SDP_RECORD_TABLE`, `SDP_RECORD_ATTR_COUNT`,
`SDP_RECORD_ATTR_STRIDE`, `SDP_ATTR_ENTRY_ID`) and add
`SDP_CCB_CONTINUATION` for the `+0x476` word.

### EXI slots and tail branches (`0x802260B0..0x8022B3E0`, 8 bodies)

`0x8022863C` and `0x80228640` are single-`blr` callback slots next to the
`0x80228644` EXI worker; the export is a bare `return`. The six 4-byte
functions are unconditional branches that Ghidra exposes as
`thunk_FUN_<target>`:

| Address | Target | Forwarded signature |
|---|---|---|
| `0x802260B0` | `fn_80225eb0` | `(pool, size, align) -> pointer` |
| `0x80227800` | `fn_80225410` | `(id, value, len) -> status` |
| `0x80227A40` | `fn_80224e60` | `(id) -> status` |
| `0x8022B3C0` | `0x801AC710` | `() -> ()` |
| `0x8022B3D0` | `0x801AC740` | `() -> ()` |
| `0x8022B3E0` | `0x801AC770` | `(address, second, length) -> pages` |

### Palette callbacks (`0x802293B0..0x80229990`, 3 bodies)

Three `param_2 == 9`/`10` callbacks over the wave-97 palette object:

- `0x802293B0`/`0x80229860` advance the object's `+0x2C`/`+0x30` cursor. Kind
  `10` sets `+0x2C` to the caller offset and adds the object's row step; kind
  `9` snaps the column to the next multiple of `+0x58` cells times the row
  height. They differ only in the step source (`fn_8022DF30` versus the
  `0x802308D0` stream step).
- `0x80229990` reports the cursor rectangle `{min_x, min_y, max_x, max_y}` and
  returns `3`/`1`/`0` through the `0x802DCDC0` unlock.

### GX state, aspect setters and scroll (`0x8022C9C0..0x8022D5F0`, 4 bodies)

- `0x8022C9C0` resets the cached GX state. Objects that open with
  `{0, 0xFFFFFFFF}` get the 20-call default block (a private helper mirrors it)
  with four extra texture/shadow calls when the inner object's vtable slot
  `+0x30` returns a kind below `2`; every other object is handed to
  `fn_8022DAF0`.
- `0x8022D2F0`/`0x8022D3C0` divide the caller's values by the inner object's
  width/height counts (vtable slots `+0x0C`/`+0x10`, the counts
  `fn_8022D440`/`fn_8022D4A0` already established) and store both scale fields.
- `0x8022D5F0` asks the inner slot `+0x50` for an eight-byte page descriptor,
  scales its `+4`/`+6` bytes by `+0x24`, blends the `+0x44` limit in when the
  `+0x43` flag is set, advances `+0x2C` and calls `0x8022D720` with the new
  cursor triple.

### Text formatter wrappers (`0x8022DFB0..0x8022F010`, 6 bodies)

The five wrappers all format into the shared `0x8042AC50` scratch buffer and
append through `0x8022FA70`; three of them copy the `0x64`-byte text object to
the stack first and destroy the copy with `fn_8022C980`:

| Address | Copy | Append flag | Returns |
|---|---|---|---|
| `0x8022DFB0` | yes | none (`0x8022F960` render) | measured width |
| `0x8022E1C0` | yes | none (`0x8022F960` render) | measured height |
| `0x8022E5B0` | yes | none (`0x8022F960` render, caller rect) | nothing |
| `0x8022EAF0` | yes | `0` (no newline) | target `u64`, cursor written back |
| `0x8022EED0` | yes | `0` (no newline) | target `u64`, cursor written back |
| `0x8022F010` | no | `1` (newline) | nothing |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80220524` | `fn_80220524` | 572 | medium |
| `0x80221440` | `fn_80221440` | 1424 | medium |
| `0x80222698` | `fn_80222698` | 1040 | medium |
| `0x80222AA8` | `fn_80222aa8` | 912 | medium |
| `0x802260B0` | `fn_802260b0` | 4 | medium |
| `0x80227800` | `fn_80227800` | 4 | medium |
| `0x80227A40` | `fn_80227a40` | 4 | medium |
| `0x8022863C` | `fn_8022863c` | 4 | high |
| `0x80228640` | `fn_80228640` | 4 | high |
| `0x802293B0` | `fn_802293b0` | 300 | medium |
| `0x80229860` | `fn_80229860` | 300 | medium |
| `0x80229990` | `fn_80229990` | 484 | medium |
| `0x8022C9C0` | `fn_8022c9c0` | 2352 | medium |
| `0x8022D2F0` | `fn_8022d2f0` | 196 | medium |
| `0x8022D3C0` | `fn_8022d3c0` | 120 | medium |
| `0x8022D5F0` | `fn_8022d5f0` | 304 | medium |
| `0x8022DFB0` | `fn_8022dfb0` | 516 | medium |
| `0x8022E1C0` | `fn_8022e1c0` | 516 | medium |
| `0x8022E5B0` | `fn_8022e5b0` | 384 | medium |
| `0x8022EAF0` | `fn_8022eaf0` | 540 | medium |
| `0x8022EED0` | `fn_8022eed0` | 320 | medium |
| `0x8022F010` | `fn_8022f010` | 272 | medium |
| `0x8022B3C0` | `fn_8022b3c0` | 4 | medium |
| `0x8022B3D0` | `fn_8022b3d0` | 4 | medium |
| `0x8022B3E0` | `fn_8022b3e0` | 4 | medium |

## Why the twenty-three `medium` rows are `medium`

- **Register-save folds.** The four SDP bodies and the palette callbacks open
  with one of `0x802DCD40`/`0x802DCD44`/`0x802DCD58`/`0x802DCD68`/
  `0x802DCD74`. Those are the compiler's non-volatile-register save stubs, not
  calls: the high half of the fake result is `r3` and the low half `r4`, the
  convention waves 92/97/99 established. Where the export then prints a
  parameterless callee, the bodies pass the recovered `r3`/`r4` explicitly
  (`0x80222AA8` and `0x80222698` are the clearest cases, since
  `fn_802236B4(ccb, transaction, ...)` is already translated and its own export
  proves the split).
- **`va_list` modelling.** `0x8022DFB0`/`0x8022E1C0`/`0x8022EAF0`/`0x8022F010`
  are variadic. The only call sites found (`0x80107170` and `0x80233EF0`) pass a
  preformatted string and no arguments, so the bodies format with a zeroed
  register-save area and do not invent an argument list. `0x8022E5B0` has no
  call site at all; its format register (`r5`) is recovered from the fact that
  the bounded formatter takes the object and rectangle in `r3`/`r4`.
- **GQR0.** The palette callbacks and `0x8022D5F0` contain `unaff_GQR0`. Those
  `ldexpf` fragments are the compiler's `psq_l`/`psq_st` scale, not data; the
  bodies use ordinary `f32` arithmetic, as waves 93/96/97 did for the same
  object family.
- **`0x8022D5F0`** reads bytes `+4`/`+6` of the eight-byte page descriptor and
  multiplies them by `+0x24`. The descriptor's field roles are marked
  `// unknown:`; only the arithmetic is claimed.
- **`0x8022D720` is not translated** (see *Deliberate skips*), so `0x8022D5F0`
  calls it by address through a typed wrapper.
- **`0x8022C9C0`** contains 20 GX calls whose roles are `// inferred:` from the
  argument shapes; the call sequence and the five-way branch are literal. The
  helper `gx_default_state` reproduces the identical prefix once.
- **The six thunks** are one branch each (Ghidra's `thunk_` name plus the
  4-byte size). Their argument lists are the translated targets' own
  signatures; `0x8022B3C0`/`0x8022B3D0`/`0x8022B3E0` forward to
  `0x801AC710`/`0x801AC740`/`0x801AC770`, whose exports are in the same image
  but a different lane.

## The two `high` rows

`0x8022863C` and `0x80228640` are single `blr` slots: the export is a bare
`return` and the body is empty. Nothing is inferred.

## Deliberate skips

Seven addresses of the bucket's 32 are still untranslated and have no row in
the CSV.

- `0x80222E38` (1052) - the ServiceSearchAttributeResponse builder. It copies
  the parsed search-pattern block twice through overlapping `0x40`-byte stack
  moves whose base is offset by four bytes, and the two copies disagree about
  which half of the id table they restore. Translating one of the two readings
  would document a different state machine; left for a pass that can pin the
  block layout from the disassembly.
- `0x80225A80` (764) - the five-argument IPC text helper. The
  `&stack0x00000000 == 0x28` test is a decompiler artefact with no recoverable
  operand, and the function's ring-buffer globals are still unnamed; the same
  reason wave 97 skipped it.
- `0x8022D720` (636) - the five-vertex GX emitter. Its vertex pairs are read
  from `f27..f31`, which no C caller in the image loads; the export removes the
  blocks that would show where those values come from (`Removing unreachable
  block` at `0x8022D960..0x8022D980`). Declaring them as parameters would be an
  invention, so the body that calls it does so by address.
- `0x8022F350` (1468), `0x8022F960` (272) and `0x8022FA70` (2052) - the text
  layout/render/append core. All three read five paired-single registers and
  contain `unaff_GQR0`; `0x8022F350`'s callers do not agree on its fourth
  argument, and `0x8022FA70` is the largest remaining body after `0x8022C9C0`.
  The formatter wrappers call `0x8022F960`/`0x8022FA70` by address, exactly as
  wave 97 did.
- `0x80228644` (744) - the EXI block-transfer worker. Its export is a dead
  `while(1)` loop (every block below it was removed as unreachable), so it is
  called by address as before.

## Repairs made in this pass

No structural repair was needed. The append point was the true end of the file
and every pre-existing item still parses. The only edit outside the new block
is the module header: the `0x80220000` pass list gained wave 102, the SDP layer
row gained the four new SDP functions, and a new status paragraph names the
three clusters above.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 25 bodies total
  10584 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_<name>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all 25 came from the
  bucket's untranslated list.
- `cargo check --message-format=short` reports no error and no new warning for
  `src/runtime/bt.rs`. The only `bt.rs` diagnostics are the ten pre-existing
  `unnecessary parentheses` warnings at lines `5194..5836`, which belong to the
  untouched wave-6 L2CAP code (the numbers moved by the header insert).
- `node tools/check-lane-csv.mjs wave102` prints `wave102-bt.csv: 25 rows` and
  `OK`.
