# Coverage — Wave 91 (`runtime::input`, BTA name/discovery search, service registry, HID events)

## How to read this file

1. **Summary** gives the per-file totals for this pass and the band inventory.
2. **What this pass covers** describes the two clusters that were added and how
   they connect to the wave-50/74/89 bodies around them.
3. The **function table** lists all 26 bodies in address order with their role,
   the byte count re-read from `work/ghidra/main-gb/export/functions.tsv` and the
   confidence carried by the doc block.
4. **Corrections and caveats** records every inference the export forced, and
   **Deliberate skips** says what was left out and why.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for; all 26 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and repeat the `// observed:` /
`// inferred:` / `// unknown:` markers at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 26 | 8,420 | 5 | 21 | 0 |

Band inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the band | 268 |
| Untranslated before this pass | 78 |
| Selected and translated by this pass | **26** |
| Untranslated in the band after this pass | 52 |

## What this pass covers

The wave-50 pass stopped in front of the BTA device-manager search layer and
wave 89 explicitly deferred it ("the BTA HID/SDP state handlers between
`0x801FB0C4` and `0x801FD16C` ... remain for a later pass"). This pass takes
that run plus the WUD handshake steps and the HID device-event bridge that feed
it. Two shapes of code repeat:

- **BTA DM search, service registry and HID events (`0x801FB0C4`–`0x801FD16C`,
  19 bodies).** One shared state machine sits behind the `0x803D17xx` globals:
  the address list and its pending/found bitmasks (`0x803D17A4`/`0x803D17A8`),
  the scan index `0x803D1808`, the clock table `0x802F6DE8`, the deferred-search
  timer `0x803D17D8` and the application callback `0x803D1798`.
  `fn_801fb0c4` starts a search, `fn_801fbb40` starts one connection per
  pending entry, `fn_801fb5cc` handles the completion, and
  `fn_801fb1d8`/`fn_801fb3fc` walk the inquiry database for the first record
  whose name is still missing. Above them sit the PIN-code reply pair
  (`fn_801fc104`/`fn_801fc1c4`), the link-key reply pair
  (`fn_801fc2fc`/`fn_801fc398`) and the `0x0B`-stride discovery-slot
  insert/remove pair (`fn_801fc5e4`, `fn_801fca3c`). `fn_801fc480` is the
  inquiry callback registration timer; `fn_801fbe08` and `fn_801fdbbc` are
  message builders on the same callback.
- **WUD handshake and HID event bridge (`0x801F1720`–`0x801FF808`, 7 bodies).**
  `fn_801f1db0` is the WUD state step that starts discovery and then picks the
  BCM2045 patch path (firmware `0x0A7`) or the normal bring-up;
  `fn_801f1ca0` is its "resolve or remove the device" step and `fn_801f1720`
  the removal/checksum snapshot; `fn_801f3310` is the shutdown snapshot that
  copies both link-key tables; `fn_801f57c0` is the BTM discovery callback the
  step above registers. `fn_801ff308` maps the nine `HID_HDEV_EVT_*` events onto
  the `0x170x` HID-host API ids and `fn_801ff808` builds the `0x170A`
  add-device message those ids feed.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---:|---|---|---|
| `0x801F1720` | `fn_801f1720` | removes linked records, folds the checksum, returns `3`/`5`/`100` | 508 | translated | medium | `work/ghidra/main-gb/export/801f1720_FUN_801f1720.c` |
| `0x801F1CA0` | `fn_801f1ca0` | resolves or clears the event's device record | 268 | translated | medium | `work/ghidra/main-gb/export/801f1ca0_FUN_801f1ca0.c` |
| `0x801F1DB0` | `fn_801f1db0` | WUD state step: discovery, version probe, firmware branch | 324 | translated | medium | `work/ghidra/main-gb/export/801f1db0_FUN_801f1db0.c` |
| `0x801F3310` | `fn_801f3310` | shutdown snapshot of both link-key tables | 544 | translated | medium | `work/ghidra/main-gb/export/801f3310_FUN_801f3310.c` |
| `0x801F57C0` | `fn_801f57c0` | BTM discovery callback, events `0`..`4` | 520 | translated | medium | `work/ghidra/main-gb/export/801f57c0_FUN_801f57c0.c` |
| `0x801FB0C4` | `fn_801fb0c4` | starts a name/discovery search over one address list | 276 | translated | medium | `work/ghidra/main-gb/export/801fb0c4_FUN_801fb0c4.c` |
| `0x801FB1D8` | `fn_801fb1d8` | inquiry-database walk for the first unnamed record | 548 | translated | medium | `work/ghidra/main-gb/export/801fb1d8_FUN_801fb1d8.c` |
| `0x801FB3FC` | `fn_801fb3fc` | cursor advance plus the same walk | 364 | translated | medium | `work/ghidra/main-gb/export/801fb3fc_FUN_801fb3fc.c` |
| `0x801FB568` | `fn_801fb568` | `0x207` post and event `2` for one request | 100 | translated | high | `work/ghidra/main-gb/export/801fb568_FUN_801fb568.c` |
| `0x801FB5CC` | `fn_801fb5cc` | connection completion, found mask, `0x208` payload | 616 | translated | medium | `work/ghidra/main-gb/export/801fb5cc_FUN_801fb5cc.c` |
| `0x801FB89C` | `fn_801fb89c` | completes the search or defers it for 3 s | 136 | translated | high | `work/ghidra/main-gb/export/801fb89c_FUN_801fb89c.c` |
| `0x801FBB40` | `fn_801fbb40` | one connection attempt per pending clock entry | 416 | translated | medium | `work/ghidra/main-gb/export/801fbb40_FUN_801fbb40.c` |
| `0x801FBCE0` | `fn_801fbce0` | next inquiry record or `0x207` end of search | 224 | translated | high | `work/ghidra/main-gb/export/801fbce0_FUN_801fbce0.c` |
| `0x801FBE08` | `fn_801fbe08` | event `0` inquiry-record report | 144 | translated | medium | `work/ghidra/main-gb/export/801fbe08_FUN_801fbe08.c` |
| `0x801FC104` | `fn_801fc104` | PIN reply to the BTM search callback | 192 | translated | medium | `work/ghidra/main-gb/export/801fc104_FUN_801fc104.c` |
| `0x801FC1C4` | `fn_801fc1c4` | PIN request: guards, name request, fallback report | 304 | translated | medium | `work/ghidra/main-gb/export/801fc1c4_FUN_801fc1c4.c` |
| `0x801FC2FC` | `fn_801fc2fc` | link-key event `3` report | 156 | translated | medium | `work/ghidra/main-gb/export/801fc2fc_FUN_801fc2fc.c` |
| `0x801FC398` | `fn_801fc398` | rejected link-key report, all flags clear | 132 | translated | high | `work/ghidra/main-gb/export/801fc398_FUN_801fc398.c` |
| `0x801FC480` | `fn_801fc480` | inquiry callback registration and search timer | 244 | translated | high | `work/ghidra/main-gb/export/801fc480_FUN_801fc480.c` |
| `0x801FC574` | `fn_801fc574` | `0x105` address/state message | 112 | translated | medium | `work/ghidra/main-gb/export/801fc574_FUN_801fc574.c` |
| `0x801FC5E4` | `fn_801fc5e4` | discovery-slot insert/remove, events `5` and `6` | 640 | translated | medium | `work/ghidra/main-gb/export/801fc5e4_FUN_801fc5e4.c` |
| `0x801FCA3C` | `fn_801fca3c` | compressed-link service open/close | 568 | translated | medium | `work/ghidra/main-gb/export/801fca3c_FUN_801fca3c.c` |
| `0x801FCD7C` | `fn_801fcd7c` | link-mode store and state reset | 132 | translated | medium | `work/ghidra/main-gb/export/801fcd7c_FUN_801fcd7c.c` |
| `0x801FDBBC` | `fn_801fdbbc` | `0x109` handle/value/flag message | 120 | translated | medium | `work/ghidra/main-gb/export/801fdbbc_FUN_801fdbbc.c` |
| `0x801FF308` | `fn_801ff308` | HID device events to `0x170x` API requests | 664 | translated | medium | `work/ghidra/main-gb/export/801ff308_FUN_801ff308.c` |
| `0x801FF808` | `fn_801ff808` | `0x170A` HID add-device message | 168 | translated | medium | `work/ghidra/main-gb/export/801ff808_FUN_801ff808.c` |

Totals: **26 translated**, 0 partial, 0 stubs; 5 high / 21 medium / 0 low;
8,420 bytes. Address range `0x801F1720`–`0x801FF808`.

## Corrections and caveats

1. **`0x802DCDxx` are epilogue helpers, not getters.** `0x801F1CA0` and the
   two builders end with `0x802DCDC0(...)`/`0x802DCDBC()`, and the export shows
   the state word as an *argument* of that helper. As in the wave-89 note, the
   bodies pass the word through and return it explicitly
   ([`fn_801f1ca0`] returns `2`/`3`). The getter constants
   `BT_CURRENT_RECORD_64/6C/70` are the same family.
2. **Lost register arguments.** Four bodies declare an argument Ghidra prints
   as `extraout_r4` or `in_r7`:
   `0x801FC574` (`state`, fifth argument register `r7`),
   `0x801FDBBC` (`handle`, second argument),
   `0x801FF808` (`attr`, second argument). The pointer names come from the
   register convention only and are marked `medium`.
3. **`0x801FF808` address source.** The body copies the `BD_ADDR` from the
   `0x802DCD70` getter, not from its first argument (`FUN_801fa3a4(...,uVar1)`
   with `uVar1` = getter result). The Rust keeps the first parameter for the
   call-site shape and documents that it is unused, exactly as the export
   shows.
4. **Payload sizes come from overlapping stack locals.** In
   `0x801FC104`/`0x801FC1C4` the name is copied with a `0x20` bound while the
   terminator local sits at `+0x26` (resp. `+0x16`); the reply buffers are
   modelled as `0x101` bytes so that the `+0xFE`..`+0x100` PIN bytes fit. Both
   offsets are kept literally and flagged as observed.
5. **`0x801FBE08` undefined byte.** Payload `+0x0A` is only initialised on the
   found-record path; the body writes the same `0` and the doc says so instead
   of pretending the original is defined.
6. **`0x801FC5E4` pending-state byte.** `0x802011D8` is only known to return
   the byte stored in the event-6 payload (`// unknown:` what it counts).
7. **`0x801F1720` checksum.** The 128-byte block at `0x803A3E40` is folded
   with the `r13-0x4B34` seed (high half = sum, low half = ones-complement
   sum); both halves are named from their use only.
8. **`0x801F3310` link-list heads.** The ten- and six-entry lists start at
   `0x803A3184`/`0x803A3134`; the two key tables are `0x803A39D9`
   (0x46-byte records) and `0x803A5081` (0x56-byte records). The counters
   copied into their headers are the wave-89 `0x803A3132`/`0x803A3133` words.
9. **`0x801FCA3C` table pointer.** `r13-0x5DA4` points at the three-byte
   `{id, class, kind}` table; `kind` `1` = open request, `2` = already open.
   This is inferred from the two branches that read byte `+2`.
10. **Module header.** The twelfth-pass bullet and the promotion note were
    appended; no earlier count was stale (the previous text named no total).
11. **No staging.** All 26 bodies live in `src/runtime/input.rs`; no other Rust
    file, generated doc or `ledger/functions.csv` was touched.

## Deliberate skips

- **`0x801FD4BC` (896 B) and `0x801FD83C` (896 B).** The pair wave 89 deferred
  and for good reason: both read a 64-bit value from the `0x802DCD58`/
  `0x802DCD74` family and use *both* halves (pointer in one, id/flag in the
  other). That is the live-register artifact family the wave-74/89 docs skip;
  the argument list cannot be recovered honestly.
- **`0x801FCC74` (264 B), `0x801FD0E0` (140 B), `0x801FD16C` (240 B).** Same
  split return (`0x802DCD70`/`0x802DCD74`): the low word is an id and the high
  word a `BD_ADDR` pointer, so a faithful body would need both halves.
  `0x801FD16C` is also the `BTA_DmAddDevice` entry already called by address
  from [fn_801f44c0].
- **`0x801FC958` (228 B).** Reads `in_r10` plus a stack argument and forwards
  both to `0x801F6A60`; the arity is not recoverable.
- **The HID-host receive path carried over from waves 61/74/89**
  (`0x801F8820`, `0x801F8BC0`, `0x801F9490`, `0x801F96A0`, `0x801F9840`,
  `0x801F9FA0`, `0x801F8F70`, `0x801F9120`, `0x801F9C50`, `0x801F9D10`–
  `0x801F9DD0`, `0x801F9830`, `0x801F8E80`, `0x801FF970`): split-frame and
  variadic-forwarding artifacts.
- **The 4-byte thunks** (`0x801F3300`, `0x801F8630`–`0x801F87F0`): pure tail
  jumps to already-translated bodies.
- **The `WUD` state handlers left in the band** (`0x801F0020`–`0x801F0B10`,
  `0x801F1930`, `0x801F1F70`, `0x801F2650`, `0x801F2B80`, `0x801F2DF0`,
  `0x801F3190`, `0x801F36F0`, `0x801F5260`, `0x801F59D0`, `0x801F7AA0`):
  larger `unaff_r13`/`extraout_r4` machines that need their own pass; they were
  not touched to keep this lane's evidence tight.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 8,420 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs` (checked by name for all 26 addresses).
- No address was already in `ledger/functions.csv`; the 78-entry untranslated
  list for the band was derived from it first.
- `cargo check --message-format=short` reports no errors and no new warnings in
  `src/runtime/input.rs`; the only warnings in the file are the two
  pre-existing `function_casts_as_integer` sites (now lines 5321 and 10517),
  which this pass did not touch. The remaining build errors belong to other
  Wave 91 lanes: `src/engine/main_loop.rs` and `src/game/stage.rs`.
- `node tools/check-lane-csv.mjs wave91` prints `OK` and reports
  `wave91-input.csv: 26 rows`, 200 unique addresses across the lane set.

## Ledger

Machine-readable rows: `ledger/agents/wave91-input.csv` (26 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
