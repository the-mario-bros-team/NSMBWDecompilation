# Coverage wave 5 — Bluetooth host stack (`runtime::bt`)

## How to read this file

This page records the second translation pass over the Bluetooth stack lane
`0x80200000..0x8025FFFF` (`main.dol`). Read it in five steps:

1. **Scope** — which address band, which selection rule, and how many functions
   were added.
2. **Summary** — status/confidence counts and the per-layer byte totals.
3. **Function table** — one row per translated function, in address order, with
   its role, status, confidence and Ghidra evidence file.
4. **What the pass adds** — prose for the three paths this wave closes: the HCI
   queue, the HID slot allocator, and L2CAP channel state.
5. **Unknowns, next targets and gates** — what is *not* proven and what the next
   pass should take.

Vocabulary from `docs/RUST_STYLE.md`: `translated` = complete recovered control
flow and every call accounted for; `partial` = entry/exit and major effects
only; `stub` = address/role only. **Confidence** measures evidence strength for
the *semantics*, not code quality.

All addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. The machine-readable companion is
`ledger/agents/wave5-bt.csv` (one row per function, same order as the table).

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`, entries whose **entry**
address is in `0x80200000..0x8025FFFF`. The 20 functions promoted by the first
pass (`docs/COVERAGE_r80200_80260.md`) were excluded, as were all entries
already present in `ledger/functions.csv`.

| Item | Count |
|---|---:|
| Eligible entries in the range | 1188 |
| Translated before this wave | 20 |
| **Selected and translated by this wave** | **30** |
| Status `translated` | 30 |
| Status `partial` | 0 |
| Status `stub` | 0 |
| Bytes added | 4,940 |
| Total translated in `runtime::bt` | 50 |
| Remaining untranslated in the range | 1138 |
| Coverage of eligible entries | 4.2 % |

The 30 functions were chosen to close *flows*, not to sample random addresses:
the HCI queue itself, the full HCI builder family around it, the HID slot
allocator and its disconnect counterpart, and the L2CAP PSM/channel teardown
path. Together they turn the previously isolated samples into a connected call
graph.

| Layer | Functions | Bytes | Address span |
|---|---:|---:|---|
| HID host (`bta_hh`) / slot management | 2 | 984 | `0x80200310..0x80200624` |
| BTM connection-slot release | 1 | 180 | `0x80207240` |
| HCI command queue and builders | 18 | 2,148 | `0x8020AC78..0x8020F1F4` |
| L2CAP PSM and channel state | 7 | 1,352 | `0x80212810..0x80218040` |
| RFCOMM/PORT frame send | 2 | 276 | `0x8021CE04..0x8021EBC8` |
| **total** | **30** | **4,940** | `0x80200310..0x8021EBC8` |

## Function table

| Address | Size | Rust function | Role (plain English) | Status | Confidence | Evidence |
|---|---:|---|---|---|---|---|
| `0x80200310` | 788 | `fn_80200310` | claim a HID-host slot, copy the connect payload, negotiate link policy, run the connection callback | translated | medium | `80200310_FUN_80200310.c` |
| `0x80200624` | 196 | `fn_80200624` | drop a HID-host slot by `BD_ADDR` and notify the connection callback | translated | high | `80200624_FUN_80200624.c` |
| `0x80207240` | 180 | `fn_80207240` | release connection-slot records, all or filtered by `BD_ADDR` | translated | medium | `80207240_FUN_80207240.c` |
| `0x8020AC78` | 396 | `fn_8020ac78` | drain the HCI input queue into the controller queue, copying packets and arming the retry timer | translated | high | `8020ac78_FUN_8020ac78.c` |
| `0x8020E55C` | 176 | `fn_8020e55c` | build HCI `0x0C12`: reversed `BD_ADDR` plus one reason byte | translated | high | `8020e55c_FUN_8020e55c.c` |
| `0x8020E784` | 48 | `fn_8020e784` | build HCI `0x0C1A`, one byte | translated | medium | `8020e784_FUN_8020e784.c` |
| `0x8020E7B4` | 68 | `fn_8020e7b4` | build HCI `0x0C1C`, two little-endian `u16` | translated | medium | `8020e7b4_FUN_8020e7b4.c` |
| `0x8020E7F8` | 68 | `fn_8020e7f8` | build HCI `0x0C1E`, two little-endian `u16` | translated | medium | `8020e7f8_FUN_8020e7f8.c` |
| `0x8020E83C` | 116 | `fn_8020e83c` | allocate and build HCI `0x0C20`, one byte | translated | medium | `8020e83c_FUN_8020e83c.c` |
| `0x8020E8B0` | 116 | `fn_8020e8b0` | allocate and build HCI `0x0C22`, one byte | translated | medium | `8020e8b0_FUN_8020e8b0.c` |
| `0x8020E924` | 68 | `fn_8020e924` | build HCI `0x0C24`, 24-bit value in little-endian order | translated | medium | `8020e924_FUN_8020e924.c` |
| `0x8020E968` | 68 | `fn_8020e968` | build HCI `0x0C28`, two little-endian `u16` | translated | medium | `8020e968_FUN_8020e968.c` |
| `0x8020E9AC` | 188 | `fn_8020e9ac` | allocate and build HCI `0x0C33`, handle + byte + two `u16` | translated | medium | `8020e9ac_FUN_8020e9ac.c` |
| `0x8020EA68` | 148 | `fn_8020ea68` | allocate and build HCI `0x0C37`, two little-endian `u16` | translated | high | `8020ea68_FUN_8020ea68.c` |
| `0x8020EAFC` | 112 | `fn_8020eafc` | build HCI `0x0C3A`, `count` reversed 3-byte entries | translated | high | `8020eafc_FUN_8020eafc.c` |
| `0x8020EB6C` | 96 | `fn_8020eb6c` | allocate and build HCI `0x1001`, no parameters | translated | medium | `8020eb6c_FUN_8020eb6c.c` |
| `0x8020EBCC` | 92 | `fn_8020ebcc` | allocate and build HCI `0x1003`, no parameters | translated | medium | `8020ebcc_FUN_8020ebcc.c` |
| `0x8020EC50` | 96 | `fn_8020ec50` | allocate and build HCI `0x1009`, no parameters | translated | medium | `8020ec50_FUN_8020ec50.c` |
| `0x8020ECB0` | 124 | `fn_8020ecb0` | allocate and build HCI `0x1403`, one `u16` | translated | medium | `8020ecb0_FUN_8020ecb0.c` |
| `0x8020ED2C` | 120 | `fn_8020ed2c` | allocate and build HCI `0x1405`, one `u16` | translated | medium | `8020ed2c_FUN_8020ed2c.c` |
| `0x8020F1F4` | 48 | `fn_8020f1f4` | build HCI `0x0C45`, one byte | translated | medium | `8020f1f4_FUN_8020f1f4.c` |
| `0x80212810` | 144 | `fn_80212810` | deregister one PSM with trace logging; the worker behind `0x80212648` | translated | high | `80212810_FUN_80212810.c` |
| `0x80214B5C` | 188 | `fn_80214b5c` | disconnect an ACL link: walk child channels, release slots, tear down | translated | high | `80214b5c_FUN_80214b5c.c` |
| `0x802169AC` | 256 | `fn_802169ac` | full connection teardown: timer, buffer, children, queue, load, callback | translated | high | `802169ac_FUN_802169ac.c` |
| `0x80216B34` | 144 | `fn_80216b34` | find an L2CAP connection block by 16-bit handle | translated | high | `80216b34_FUN_80216b34.c` |
| `0x80217DE4` | 420 | `fn_80217de4` | unlink a child channel block from its owner and return it to the free list | translated | high | `80217de4_FUN_80217de4.c` |
| `0x80218030` | 16 | `fn_80218030` | clear one PSM registration record | translated | high | `80218030_FUN_80218030.c` |
| `0x80218040` | 184 | `fn_80218040` | find a PSM registration record in the two-block table | translated | high | `80218040_FUN_80218040.c` |
| `0x8021CE04` | 176 | `fn_8021ce04` | build a short RFCOMM frame with the three-byte checksum and queue it | translated | medium | `8021ce04_FUN_8021ce04.c` |
| `0x8021EBC8` | 100 | `fn_8021ebc8` | append an RFCOMM frame and drain the send queue into L2CAP | translated | high | `8021ebc8_FUN_8021ebc8.c` |

Confidence split: 13 `high`, 17 `medium`. No `low`, no `partial`.

## What the pass adds

### 1. The HCI queue is no longer a black box

`0x8020AC78` was the most important open target: all three builders promoted in
wave 1 called it, but its argument was dropped by Ghidra and its header fields
were unnamed. The translation confirms the layout used by every builder in this
wave:

| Offset | Type | Meaning |
|---|---|---|
| `+0x02` | `u16` | total packet length / tag consumed by the copy step |
| `+0x04` | `u16` | header offset; the opcode lives at `packet + this + 8` |
| `+0x08` | `u16` | HCI opcode, little-endian |
| `+0x0A` | `u8` | parameter length |
| `+0x0B` | … | parameters |

The queue does three things that were invisible before:

1. It only appends a new packet when the input queue already has one, then
   computes a drain budget from the input count + 1.
2. It copies every non-`0x0C03`/`0x0C35` packet into a controller buffer, copies
   the header and `u16`-length payload separately, and arms the `0x3C`-tick
   retry timer. `0x0C03` is the HCI Reset opcode; `0x0C35` is unnamed.
3. It returns an unprocessed supplied packet to the input queue, which explains
   why builders never retry themselves.

`fn_8020e55c` and the 17 other builders then fill that layout directly. Their
opcodes are observed; only `0x0C18` has a caller string proving its mnemonic
(`BTM: BTM_WritePageTimeout`, wave 1). The rest stay `// unknown:` rather than
being given guessed names.

### 2. HID slot allocation is complete

Wave 1 translated the slot *teardown* (`fn_80200000`) and the slot *lookup*
(`fn_8020028c`), but not the allocator. `0x80200310` closes that gap:

- it reads the current ACL `BD_ADDR` and tag from `0x802DCD6C`;
- scans the four `0x11C`-byte slots for a duplicate, otherwise claims the first
  free one;
- copies the payload and issues `0x041F`/`0x041D`, then `0x040F` after the BTM
  record's eight feature bytes are found;
- copies the page timeout into the peer slot's `+0x10E` and re-queues
  `0x8020EA68` when that peer has no callback byte;
- calls the global five-argument connection callback.

`0x80200624` is the matching "drop by address" path, and `0x80207240` is the
connection-slot release table walk used by `0x80214B5C`. The slot layout now has
documented names for `+2` (link policy), `+8` (`BD_ADDR`), `+0x10E` (remote
timeout), `+0x110` (feature bytes), `+0x11A` (callback byte) and `+0x11B` (role
attempt flag).

### 3. L2CAP channel state is connected end to end

`0x80212810` (deregister worker) called `0x80218040` (PSM lookup), which had no
translation. This wave adds both, plus the channel-side chain:

```
0x80214B5C  disconnect link
  -> 0x80216B34  find connection by handle
  -> 0x80213240  event dispatcher (still untranslated)
  -> 0x80207240  release connection slots
  -> 0x802169AC  channel teardown
       -> 0x80217DE4  unlink + free-list push
       -> 0x80200624  drop HID slot
       -> 0x802151D0  recompute windows
```

The four **connection blocks** at `0x803D4C48` have a timer at `+0x10`, a
16-bit handle at `+0x28`, `BD_ADDR` at `+0x2A`, flags at `+0x34`, load at
`+0x38`, send queue at `+0x44`, retained buffer at `+0x50`, callback at `+0x54`
and retry timeout at `+0x58`. A **child channel block** instead has list links
at `+8`/`+0xC`, the owner pointer at `+0x10`, a timer at `+0x18` and a pending
queue at `+0x70`. The two structures overlap at `+0x10`/`+0x58` with different
meanings, so the Rust constants are named per structure. The free list at
`0x803D53F0`/`0x803D53F4` is reproduced exactly, including the two different
zeroing widths in its two branches.

### 4. RFCOMM frame send

`0x8021CE04` builds a short frame (control byte derived from the channel flag
and the lock tag, `0x3F`, `0x01`, then the checksum from wave 1's
`fn_8021e44c`) and hands it to `0x8021EBC8`. The drain loop pops until the
channel's stop byte `+0x73` is set or the queue is empty and submits each frame
to the L2CAP data-write entry with the handle at `+0x68`.

## Unknowns and next targets

1. **Dropped arguments.** Ghidra drops arguments at five call sites in this
   wave: `0x802169AC -> 0x80217DE4`/`0x801F7240`, `0x80217DE4 -> 0x801F7240`,
   `0x80217DE4 -> 0x801FA320` (third timer argument), and
   `0x802169AC -> 0x801F7240`. The reconstructed arguments are marked
   `// inferred:`/`// unknown:` at the call site; they are the list head, the
   popped buffer, and `0` for the missing timer value.
2. **HCI command names.** Only `0x0C18` has a proving string. `0x0C12`,
   `0x0C1A`, `0x0C1C`, `0x0C1E`, `0x0C20`, `0x0C22`, `0x0C24`, `0x0C28`,
   `0x0C33`, `0x0C37`, `0x0C3A`, `0x0C45`, `0x1001`, `0x1003`, `0x1009`,
   `0x1403` and `0x1405` are observed only.
3. **`0x0C35`.** The queue treats it like Reset and skips the copy. No string in
   this lane names it.
4. **HID slot field meanings.** `+4` is always zeroed; `+0x11B` is a role-attempt
   flag inferred from `0x8020074C`. `+0x10E` is a timeout because it receives
   `0x803D203E`; its unit is still unknown.
5. **`0x80207240` callback ABI.** The call takes one integer (the slot index)
   and is never null-checked in the original. The table's owner is inferred
   from its callers, not from a string.
6. **`0x80217DE4` owner/timer branch.** The state-4/no-children arm path and the
   `0x1E`/`-1`/`1` timeout choice are exact, but the security probe
   `0x802097BC` is outside this lane and its return values are only used as
   branch selectors.
7. **`0x8021CE04` frame type.** The byte layout and checksum are exact; whether
   the frame is UIH, SABM or a PORT control frame is not proven.

Closest remaining bodies, all still untranslated:

| Address | Size | Why next |
|---|---:|---|
| `0x80213240` | 4,916 | The L2CAP per-state event switch this wave keeps calling; it names every channel state `0`..`8`. |
| `0x8020EDA4` | 1,056 | Largest remaining HCI builder/worker between this wave's `0x8020ED2C` and `0x8020F1C4`. |
| `0x8020074C` | 540 | Role-switch request path; defines slot `+0x11B` and the `0x80205CEC`/`0x80202460` branches. |
| `0x80200AD8` | 496 | Link-policy filter called by `0x80200310`; names the four `0x803D4338` diagnostics. |
| `0x80200CD8` / `0x80200DDC` / `0x80200E68` | 260 / 140 / 556 | Remaining slot/link-policy helpers around the allocator. |
| `0x802180F8` / `0x80218240` / `0x80218280` / `0x80218390` | 328 / 64 / 272 / 64 | L2CAP configuration event helpers used by `0x80213240`. |
| `0x80218DDC` | 324 | RFCOMM `PORT_DlcEstablishCnf` entry; the string that names the layer. |
| `0x8021E44C` neighbours | — | The RFCOMM frame builders that append the checksum to longer payloads. |

Deliberately **not** next: the `< 16`-byte veneers listed in
`docs/COVERAGE_r80200_80260.md`, and any function whose body depends on
`unaff_r13`/`unaff_r2` small-data offsets. Those need the SDA base recovered
first; this wave avoided them (for example `0x80213120` was passed over because
it reads `unaff_r2 - 0x4C70`).

## Gates

- `node tools/doc-lint.mjs src` → `ok: true`, 0 violations (68 files).
- `cargo check` → `Finished dev profile`, exit 0, with **0 errors and 0 warnings
  from `src/runtime/bt.rs`** (the other lanes' earlier errors have since been
  fixed).
- No files outside this lane were edited: only `src/runtime/bt.rs`,
  `ledger/agents/wave5-bt.csv` and this document.
