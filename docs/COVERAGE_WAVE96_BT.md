# Coverage - Wave 96 (`runtime::bt`, RFCOMM PORT handlers and SDP database writers)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count is re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave96-bt.csv` names the Ghidra C export. All addresses are in
the `0x80210000` bucket, which holds 198 functions and had 58 untranslated
entries when this pass started.

The pass closes the two holes that waves 58/91 (RFCOMM `PORT`) and 60/84 (SDP)
had left open:

- the **RFCOMM PORT event handlers** at `0x802187EC..0x8021982C`, between the
  wave-91 L2CAP/RFCOMM helpers and the wave-58 DLC callbacks,
- the **multiplexer entries** at `0x8021C0EC..0x8021C64C` that translate one
  received PN/RPN/MSC/RLS frame into those handlers,
- the **carrier-frame builder and port-close worker** at
  `0x8021D7F4`/`0x8021E9A8`,
- the **SDP database writers** at `0x8021EC2C..0x8021FF38`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 24 | 9124 | 10 | 14 |

## What this pass covers

### RFCOMM PORT handlers (`0x802187EC..0x8021982C`, 13 bodies)

Every body resolves its `0xA4`-byte port block through [`fn_80219e40`] and then
moves it between the DLC states documented in earlier waves:

| State | Meaning | Written by |
|---:|---|---|
| `0` | idle | `802194f4`, `8021e9a8` |
| `1` | opening | `80219300`, `80218b98` (read) |
| `2` | open | `80218c9c`, `80218ddc` |
| `5` | closing | `80218ddc` (read via the channel state byte) |
| `6` | disconnecting | `8021c0ec` |

The handlers add the field names below. They are all `// observed:` at a
literal offset in the cited export:

| Offset | Field | Evidence |
|---:|---|---|
| `+0x5A` | nine-byte control/parameter descriptor | passed to `0x8021CBA0`/`0x8021D630` |
| `+0x63` | last line-status byte | written by `80219188`, `8021c4e0` |
| `+0x64` | control flag byte (bit `0` pending, bits `2`/`3` forwarded) | `80219070`, `80219188`, `80219300` |
| `+0x65` | deferred-status marker | `8021962c` |
| `+0x69` | DLC flag byte of the *port* copy (bits `0..4`) | `8021c0ec`, `8021c64c`, `8021c1d4`, `8021c4e0` |
| `+0x74` | resume flag inside a channel block | `8021e9a8`, `8021982c` |

### Multiplexer entries (`0x8021C0EC..0x8021C64C`, 4 bodies)

| Address | Frame | Confirm path | Indication path |
|---|---|---|---|
| `0x8021C0EC` | PN | clear flag `1`, stop timer, `80218B98` | state `6` rejects, else `802189E8` |
| `0x8021C1D4` | RPN | descriptor copy-back, resend `0xC00`/`0x3000` | `0x80218F20` by address |
| `0x8021C4E0` | RLS | clear flag `3`, `80219300` | `8021D460`, flow, `80219188` |
| `0x8021C64C` | MSC | clear flag `4` | `802193C4`, `8021D580` |

### SDP database writers (`0x8021EC2C..0x8021FF38`, 5 bodies)

- `0x8021EC2C` initialises a discovery database: header at `+0`/`+4`/`+8`/
  `+0x68`, up to three twenty-byte UUID records at `+0x10`, up to `0x0C`
  `u16` attribute ids at `+0x4E`, counts at `+0x0C`/`+0x4C`.
- `0x8021F158` walks the service-record container lists and returns the
  container that holds a matching value.
- `0x8021F260` builds a service record from the fixed template (strings at
  `+9`, `+0x59`, `+0xA9`; flag byte at `+8`) and adds attributes
  `0x200`..`0x205`, `0x0B`, `0x101` and `10`.
- `0x8021FA94` searches the record table with a list of attribute values; a
  record matches only when every search entry matches.
- `0x8021FF38` inserts one attribute in id order, shifts the following entries
  and copies the value bytes into the record's `0x15E`-byte value area.

The attribute entry layout is now consistent across both writers:
`+0` value length, `+4` value pointer, `+8` attribute id (`u16`), `+0x0A` type
byte. This is the layout `0x8021FF38` writes and `0x8021FA94` reads.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802187EC` | `fn_802187ec` | 352 | high |
| `0x802189E8` | `fn_802189e8` | 432 | medium |
| `0x80218B98` | `fn_80218b98` | 260 | medium |
| `0x80218C9C` | `fn_80218c9c` | 320 | high |
| `0x80218DDC` | `fn_80218ddc` | 324 | medium |
| `0x80219070` | `fn_80219070` | 280 | high |
| `0x80219188` | `fn_80219188` | 376 | high |
| `0x80219300` | `fn_80219300` | 196 | medium |
| `0x802193C4` | `fn_802193c4` | 200 | high |
| `0x8021948C` | `fn_8021948c` | 104 | high |
| `0x802194F4` | `fn_802194f4` | 160 | high |
| `0x8021962C` | `fn_8021962c` | 512 | medium |
| `0x8021982C` | `fn_8021982c` | 304 | medium |
| `0x8021C0EC` | `fn_8021c0ec` | 232 | high |
| `0x8021C1D4` | `fn_8021c1d4` | 780 | medium |
| `0x8021C4E0` | `fn_8021c4e0` | 364 | medium |
| `0x8021C64C` | `fn_8021c64c` | 140 | high |
| `0x8021D7F4` | `fn_8021d7f4` | 192 | medium |
| `0x8021E9A8` | `fn_8021e9a8` | 344 | high |
| `0x8021EC2C` | `fn_8021ec2c` | 888 | medium |
| `0x8021F158` | `fn_8021f158` | 264 | medium |
| `0x8021F260` | `fn_8021f260` | 864 | medium |
| `0x8021FA94` | `fn_8021fa94` | 316 | medium |
| `0x8021FF38` | `fn_8021ff38` | 920 | medium |

## Why the fourteen `medium` rows are `medium`

- `0x802189E8`, `0x80218B98`, `0x80218DDC`, `0x8021982C` and `0x8021C4E0` open
  with a `0x802DCD58..0x802DCD74` register-save stub. Ghidra prints the stub as
  a 64-bit value whose halves are the incoming `r3`/`r4`, but which half is
  which varies per function. Each doc block names the recovery it used; every
  one is cross-checked against a body use (`chan + 0x72` for the channel,
  `port + 0x6C` for the port) or against a caller
  (`0x8021AAF4` for `0x802187EC`, `0x8021C4E0` for `0x80219300`).
- `0x80219300` additionally prints as `void FUN_80219300(void)`; its two
  arguments are taken from its only caller, which passes `port->channel` and
  `port->index`.
- `0x8021962C` is fully literal except for the receive-buffer layout, which is
  inferred from the `u16` length at `+2`, the `u16` payload offset at `+4` and
  the same `+2`/`+4` reads in `0x8021B738` (event `4`).
- `0x8021C1D4`, `0x8021C4E0` and `0x8021D7F4` reproduce the RFCOMM RPN/RLS
  descriptor bits literally; the *meaning* of each bit is inferred from the
  V.24 status encoding and from the DTR/DSR trace string.
- `0x8021EC2C` has the unrolled eight-entry copy loops translated as counted
  loops (the bounds are `0..1` and `0..0x0C`, so the unrolling is only a
  compiler artefact). `0x8021F158`, `0x8021F260`, `0x8021FA94` and `0x8021FF38`
  are literal, but they call `0x802203A8` and `0x8021FF38` whose exports are
  printed as `void` while both call sites test the return value; the callers
  keep the status by calling the address (`SDP_ADD_UUID_LIST_802203A8`) or by
  using the new `fn_8021ff38` signature, and the doc blocks say so.

## Repairs made in this pass

`src/runtime/bt.rs` had a structural defect inherited from wave 93: that
section was pasted in the middle of `fn_80227730`, so from `0x8020885C` to the
end of the file every item was nested inside one function body (which Rust
accepts, so `cargo check` stayed green) and the tail of `fn_80227730` sat at
end-of-file. This pass closed `fn_80227730` where it was cut, deleted the
dangling tail, and renamed the wave-93 record-offset constant `BTM_DEV_STATE`
(`0x7F`) to `BTM_SEC_STATE` so it no longer collides with the pre-existing
`BTM_DEV_STATE` global at `0x803D21C6`. No behaviour changed: the moved
statements are byte-identical to the ones the export shows.

## Deliberate skips

None of these has a row in `ledger/agents/wave96-bt.csv`.

- `0x80218F20` (336) - the port-negotiation indication builder. Its export
  writes the nine descriptor bytes through the absolute addresses
  `0x51..0x59` on the "port not found" path (the decompiler folded a null base
  pointer into a global), so that path cannot be reconstructed honestly. The
  two bodies that need it (`0x8021C0EC` indirectly, `0x8021C1D4`) call it by
  address as `RFCOMM_PORT_NEG_IND_80218F20` with
  `(chan, index, descriptor, mask)`.
- `0x8021E4D0` (344) - the RFC timer starter. Its export contains
  `extraout_r4`, a register the decompiler never assigns, so the value stored
  into `0x803D5831` is unknown. (Wave 93 reached the same conclusion.)
- `0x8021C904`, `0x8021C96C`, `0x8021C9DC`, `0x8021CAE4`, `0x8021CBC0`,
  `0x8021CD08`, `0x8021CDA0`, `0x8021D270` - the DLC callback helpers this pass
  calls. Their exports call `FUN_80219E40()` with no argument list at all (the
  register pair is gone), so their arguments cannot be reconstructed; they are
  called by address with the arguments the call sites show.
- `0x8021A704` (672), `0x8021AAF4` (2652), `0x8021B738` (2476),
  `0x8021D8B4` (1396), `0x8021DE28` (1572), `0x8021F5C0` (1200) - the larger
  RFCOMM dispatch, command and frame-assembly routines that the new bodies call
  or sit next to. They are the next targets of this cluster and were not
  started.
- `0x8021022C`, `0x802122B0`, `0x802128A0`, `0x80212AD4`, `0x80213120`,
  `0x80213240`, `0x80214F04..0x802175E0`, `0x8021843C` - the HID-host/L2CAP
  holes of the same bucket, left for a pass that owns that cluster.
  `0x8021843C` in particular selects a log string from `unaff_r13` and cannot
  be named honestly.

## Verification

- Every address and byte size in this doc and in the CSV was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass; the 24 bodies total
  9124 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`; all 24 came from the
  bucket's untranslated list.
- `cargo check --message-format=short` reports no errors for
  `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave96` reports `wave96-bt.csv: 24 rows` and
  `OK`.
