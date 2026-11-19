# Coverage - Wave 93 (`runtime::bt`, BTM security manager and its HCI builders)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count is re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the evidence column of
`ledger/agents/wave93-bt.csv` names the Ghidra C export. All addresses are in
the `0x80200000` bucket, which holds 256 functions and had 71 untranslated
entries when this pass started (this lane's count of the bucket; the coordinator
reported 72, one of which is accounted for below as a branch thunk).

The pass covers two clusters that the earlier waves in this bucket had left
open:

- the BTM **security manager** at `0x8020885C..0x8020A68C`, which sits between
  the wave-66 inquiry helpers and the wave-89 GAP block,
- the **HCI builders** that the security manager calls, at
  `0x8020CD34..0x8020E39C`, between the wave-66 connection-mode builders and
  the wave-84 IPC wrappers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 25 | 8892 | 5 | 20 |

## What this pass covers

- **Security manager**
  (`0x8020885C..0x8020A68C`, 17 bodies). The cluster resolves a
  [`BTM_DEV_TABLE`] record and moves it between security states, so the same
  seven field offsets recur in every body:

  | Offset | Field | Evidence |
  |---:|---|---|
  | `+0x18` | `u16` connection handle | stored from the folded handle in `8020933c`, cleared to `0xFFFF` in `80209878` |
  | `+0x1C` | six-byte `BD_ADDR` | compared with `0x803D432C` in `80209878`/`80209d04` |
  | `+0x22` | three class-of-device bytes | copied to `0x803D4332` in `80209d04` |
  | `+0x25` | sixteen-byte link key | copied from the event in `802099b4`, from the PIN buffer in `80209b60` |
  | `+0x35` | 64-byte remote name | filled by `80208a98` |
  | `+0x76` | flag byte (existing) | bits `8` name known, `2` key known, `4` key type, `0x10` key stored |
  | `+0x7F` | procedure state | `0` idle, `1` authenticating, `2` encrypting, `3` awaiting name, `4` authorizing, `5` pairing |
  | `+0x81` | request flags | `0x40` = link authenticated |
  | `+0x82` | deferred completion | set by `802099b4`, released by `80208a98` |
  | `+0x83` | key type | forced to `0xFF` when `0x803D21AE` is zero |
  | `+0x85` | authenticated-link marker | set on connect, tested before the key callbacks |

  The record's `+4` word holds the pending completion callback and `+8` its
  context; four paths clear an owner word at `+0` (see the `clear_owner`
  helper).

- **HCI builders** (`0x8020CD34..0x8020E39C`, 8 bodies). Each one writes the
  same three-byte header (`+2` total length, `+4` zero, `+8` opcode low byte,
  `+9` opcode high byte, `+0xA` parameter length) and queues the packet through
  [`fn_8020ac78`]. Six allocate a size-class `2` buffer from [`HEAP_ALLOC`];
  `0x8020DAE8` writes a caller buffer and `0x8020E39C` writes a
  caller-supplied multi-record packet.

  | Address | Opcode | Parameters | Form |
  |---|---|---:|---|
  | `0x8020CD34` | `0x0403` | 9 | allocate |
  | `0x8020CDF4` | `0x0405` | 13 | allocate |
  | `0x8020D0A4` | `0x040B` | 22 | allocate |
  | `0x8020D2E4` | `0x040D` | 23 | allocate |
  | `0x8020D9EC` | `0x0428` | 17 | allocate |
  | `0x8020DAE8` | `0x0429` | 21 | caller buffer |
  | `0x8020DCE4` | `0x0803` | 10 | caller buffer or allocate |
  | `0x8020E39C` | `0x0C11` | 1 + 22 n | caller buffer |

  The opcodes are the two literal header bytes; the mnemonic in each doc block
  is inferred from the opcode and the parameter length, which is why six of the
  eight are `medium`. Byte-for-byte the bodies follow the export.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8020885C` | `fn_8020885c` | 300 | medium |
| `0x802089E4` | `fn_802089e4` | 180 | medium |
| `0x80208A98` | `fn_80208a98` | 744 | medium |
| `0x80208D80` | `fn_80208d80` | 836 | medium |
| `0x802091C0` | `fn_802091c0` | 316 | medium |
| `0x802092FC` | `fn_802092fc` | 64 | high |
| `0x8020933C` | `fn_8020933c` | 1148 | medium |
| `0x802097BC` | `fn_802097bc` | 188 | medium |
| `0x80209878` | `fn_80209878` | 316 | medium |
| `0x802099B4` | `fn_802099b4` | 428 | medium |
| `0x80209B60` | `fn_80209b60` | 304 | medium |
| `0x80209C90` | `fn_80209c90` | 116 | high |
| `0x80209D04` | `fn_80209d04` | 860 | medium |
| `0x8020A060` | `fn_8020a060` | 80 | high |
| `0x8020A490` | `fn_8020a490` | 284 | medium |
| `0x8020A5AC` | `fn_8020a5ac` | 224 | medium |
| `0x8020A68C` | `fn_8020a68c` | 52 | medium |
| `0x8020CD34` | `fn_8020cd34` | 192 | medium |
| `0x8020CDF4` | `fn_8020cdf4` | 224 | medium |
| `0x8020D0A4` | `fn_8020d0a4` | 416 | high |
| `0x8020D2E4` | `fn_8020d2e4` | 484 | high |
| `0x8020D9EC` | `fn_8020d9ec` | 252 | medium |
| `0x8020DAE8` | `fn_8020dae8` | 232 | medium |
| `0x8020DCE4` | `fn_8020dce4` | 204 | medium |
| `0x8020E39C` | `fn_8020e39c` | 448 | medium |

## Why the twenty `medium` rows are `medium`

- `0x8020885C`, `0x802089E4`, `0x80208A98`, `0x80208D80`, `0x802091C0`,
  `0x8020933C`, `0x802099B4`, `0x80209B60`, `0x80209D04` and `0x8020A68C`
  take at least one argument from a
  `0x802DCD60..0x802DCD74` register-save stub. Ghidra prints those stubs as
  calls returning a 64-bit value whose halves are the incoming `r3`/`r4`; which
  half is which varies per stub and per export. Each doc block names the
  recovery it used, and where a caller exists (`0x802063B8` for `0x8020DCE4`,
  `0x80206908` for `0x8020DAE8`, `0x80206AE4` for `0x8020D9EC`, `0x80207D90`
  for `0x8020CDF4`, `0x80203B7C` for `0x8020E39C`, `0x80205350` for
  `0x8020CD34`) the argument order is taken from that call site.
- `0x8020933C`, `0x802099B4`, `0x80209B60` and `0x80208D80` have no translated
  caller at all; their handle/address split is inferred from use inside the
  body.
- `0x80209878`, `0x8020A490` and `0x8020A5AC` are fully literal except for the
  role of the owner object at record `+0` (only its state word `3` and its
  clear at `+4` are observed) and, in `0x8020A490`, the two forwarded context
  offsets `+0x10`/`+0x26`.
- `0x8020A0B0`'s per-service capability bitmap is not proven, so the five
  bodies that need it call it by address instead of reproducing an unproven
  interpretation.

## Deliberate skips

These neighbours were read and left untranslated. None has a row in
`ledger/agents/wave93-bt.csv`.

- `0x802090C4` (252) - the master-key completion handler. Its first argument is
  a six-byte value read from `unaff_r2 - 0x4C78`, an SDA-relative origin the
  decompiler does not name (`unaff_r2` is a register the export never
  assigns), so the address passed to the callback cannot be reconstructed.
- `0x8020A0B0` (992) - the security procedure executor. It walks a
  per-service capability bitmap at record `+0x10` whose bit roles are only
  visible inside its own export; left for a pass that can prove the mask.
  Five translated bodies call it by address as
  `BTM_SEC_EXECUTE_8020A0B0`.
- `0x8020A6C0` (1464) - the HCI event dispatcher. It is a forty-target switch
  that fans out to the whole stack (HID host, L2CAP, RFCOMM, BTM), including
  routines in this lane that are still untranslated; not started in this pass.
- `0x802078C8` (552) - the security service registration. It owns the 24-record
  `0x803D3508` table, which no other body in this pass touches; it is better
  done together with that table's readers in one later pass.
- `0x80204138`, `0x80205088`, `0x802072F4` - 4-byte branch thunks
  (`thunk_FUN_*`) with no body of their own.
- The remaining holes between `0x8020413C` and `0x802085C0` (inquiry,
  remote-name and SCO handlers) were not started; they are the same
  inquiry-database cluster that waves 66, 79 and 89 document elsewhere in this
  file.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 25 bodies total
  8892 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all 25 came from the
  bucket's untranslated list.
- `cargo check --message-format=short` reports no errors for
  `src/runtime/bt.rs` (the remaining warnings in the workspace belong to other
  files).
- `node tools/check-lane-csv.mjs wave93` reports `wave93-bt.csv: 25 rows` and
  `OK`.
