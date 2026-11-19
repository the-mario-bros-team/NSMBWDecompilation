# Coverage — Wave 50 (`runtime::input`, BTA device-manager glue)

## How to read this file

This page records what the wave-50 `runtime::input` lane added to
[`src/runtime/input.rs`](../src/runtime/input.rs). Read it in four steps:

1. **Summary** gives the before/after totals for the `0x801F0000` band.
2. **What this pass covers** explains the two shapes of code added: the
   message builders and the registration/state helpers.
3. The **function table** lists all 28 bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv`.
4. **Corrections and deliberate skips** records what was left out and why.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for; all 28 rows are
`translated`. All function names stay `fn_<addr>` because the image has no
symbol table for this layer; the doc comments carry the roles, and
`// inferred:` / `// unknown:` markers repeat them at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 28 | 2,544 | 16 | 12 |

Band inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the band | 268 |
| Untranslated before this pass | 191 |
| Selected and translated by this pass | **28** |
| Untranslated in the band after this pass | 163 |

## What this pass covers

The pass takes one contiguous slice, `0x801FB924`–`0x801FD4A4`, of the
Broadcom **BTA device manager (DM)** that sits directly above the wave-34
BTA SYS timer/registration layer. It is the code the WPAD state machine in
`0x801DE9D0`–`0x801E8DD0` drives by address (`0x801F0B10`, `0x801F2B80`,
`0x801F2D10`, `0x801F4620`, `0x801F9E10`, `0x801DEB70`).

Two shapes of code repeat:

- **Message builders.** Each allocates a buffer with the GKI `getbuf`
  `0x801F6FB0`, writes the 16-bit BTA event id into the first halfword,
  fills the payload, and posts it with the wave-34 send helper
  `0x801FA850`. The event ids and payload offsets are literal:
  `0x100` (enable), `0x101` (disable), `0x102` (name, `0x20` bytes),
  `0x103` (two mode bytes), `0x10C`, `0x200` (10-byte address + two words),
  `0x201` (cancel), `0x203`, `0x204` (inquiry record + name), `0x206`.
  Covered: `fn_801fbdc0`, `fn_801fbe98`, `fn_801fbf24`, `fn_801fbfb0`,
  `fn_801fcef8`, `fn_801fcf68`, `fn_801fcfcc`, `fn_801fd024`,
  `fn_801fd0a8`, `fn_801fd2a4`.
- **Registration and state helpers.** They manage the DM globals:
  the inquiry-result buffer `0x803D17AC`, the remote-name record buffer
  `0x803D180C`, the local-name buffer `0x803D17B8`, the local `BD_ADDR`
  `0x803D17B2`, the DM timer node `0x803D17F0`, the HID application callback
  `0x803D1798`, the BTM search callback `0x803D1864`, the 0x104-byte DM state
  area `0x803D1814` and the 0x2E-byte BTA HID area `0x803D1918`.
  Covered: `fn_801fb924`, `fn_801fb970`, `fn_801fb9b4`, `fn_801fb9fc`,
  `fn_801fba44`, `fn_801fba88`, `fn_801fbacc`, `fn_801fbee4`,
  `fn_801fc03c`, `fn_801fce00`, `fn_801fce20`, `fn_801fce70`,
  `fn_801fcf30`, `fn_801fd25c`, `fn_801fd2dc`, `fn_801fd374`,
  `fn_801fd43c`, `fn_801fd4a4`.

Cross-layer facts reused from wave 34: `0x801F6FB0` is confirmed to be the
GKI `getbuf` by size (its body walks the pool-size table), `0x801F7150` is
the same allocator by pool id (`2` used here), `0x801F7240` frees a payload
pointer, `0x801FA850` posts to the HID-host queue and `0x801FA83C` installs
an event queue. The two queue objects registered by `fn_801fce70` are
`r2`-relative (`SDA2_BASE - 0x4CA0` / `- 0x4C98`), the first `r2`-relative
globals in this module; `SDA2_BASE` is imported from `crate::boot::entry`.

## Functions

| Address | Function | Bytes | Confidence |
|---|---|---:|---|
| `0x801FB924` | `fn_801fb924` | 76 | high |
| `0x801FB970` | `fn_801fb970` | 68 | high |
| `0x801FB9B4` | `fn_801fb9b4` | 72 | high |
| `0x801FB9FC` | `fn_801fb9fc` | 72 | high |
| `0x801FBA44` | `fn_801fba44` | 68 | high |
| `0x801FBA88` | `fn_801fba88` | 68 | medium |
| `0x801FBACC` | `fn_801fbacc` | 92 | medium |
| `0x801FBDC0` | `fn_801fbdc0` | 72 | high |
| `0x801FBE98` | `fn_801fbe98` | 76 | high |
| `0x801FBEE4` | `fn_801fbee4` | 64 | medium |
| `0x801FBF24` | `fn_801fbf24` | 140 | medium |
| `0x801FBFB0` | `fn_801fbfb0` | 136 | medium |
| `0x801FC03C` | `fn_801fc03c` | 200 | medium |
| `0x801FCE00` | `fn_801fce00` | 28 | high |
| `0x801FCE20` | `fn_801fce20` | 80 | medium |
| `0x801FCE70` | `fn_801fce70` | 136 | medium |
| `0x801FCEF8` | `fn_801fcef8` | 56 | high |
| `0x801FCF30` | `fn_801fcf30` | 56 | medium |
| `0x801FCF68` | `fn_801fcf68` | 100 | high |
| `0x801FCFCC` | `fn_801fcfcc` | 88 | high |
| `0x801FD024` | `fn_801fd024` | 132 | high |
| `0x801FD0A8` | `fn_801fd0a8` | 56 | high |
| `0x801FD25C` | `fn_801fd25c` | 72 | high |
| `0x801FD2A4` | `fn_801fd2a4` | 56 | high |
| `0x801FD2DC` | `fn_801fd2dc` | 152 | medium |
| `0x801FD374` | `fn_801fd374` | 200 | medium |
| `0x801FD43C` | `fn_801fd43c` | 104 | medium |
| `0x801FD4A4` | `fn_801fd4a4` | 24 | high |

Totals: **28 translated**, 0 partial, 0 stubs; 16 high / 12 medium / 0 low.

## Corrections and caveats

1. **`fn_801fba88` implicit send argument.** Ghidra renders the post as
   `FUN_801fa850()` with no argument, but the loaded `0x803D180C` pointer is
   in `r3` by the ABI. The same register-carried argument pattern was
   accepted for `fn_801fa95c` in wave 34; the body is `medium`.
2. **`fn_801fbf24` / `fn_801fbfb0` inquiry-record pointer.** `0x803D179C` is
   a *pointer variable*, not the record itself: it is assigned from
   `0x80204CE4` and every use is `<value> + 2` for the six-byte `BD_ADDR`.
   The Rust reads the pointer and adds `2` before the `BD_ADDR` copy.
3. **`fn_801fce70` `r2`-relative queues.** `unaff_r2 - 0x4CA0` and
   `- 0x4C98` are converted with `SDA2_BASE` (`0x80433080`) from
   `crate::boot::entry`, matching the `r2` convention already used in
   `src/runtime/ax.rs`. The role (event-queue objects for ids `1`/`2`) is
   inferred from the immediate `0x801FA83C` registrations.
4. **`fn_801fce00` Ghidra jumptable warning.** Ghidra "could not recover
   jumptable at 0x801fce18" and treats the indirect jump as a call. The
   decompiled body is a single `(*0x803D1864)(1, 0)` and is translated as
   that; confidence stays `high` because both arguments and the slot are
   literal.
5. **`fn_801fd25c` flag inversion.** `0x80202914`/`0x8020213C` return
   "found" flags, but the `countLeadingZeros(x & 0xFF) >> 5` idiom in the
   export returns `1` when the callee returned `0`. The Rust reproduces the
   inverted result and the caller `0x801F4620` logs the path as
   `"BTA_DmRemoveDevice"`, which corroborates the role.
6. **`0x8020E784` is a header filler, not a submitter.** It writes opcode
   `4`, length `0`, bytes `0x1A`/`0x0C`/`1` at `+8..+0x0B` and calls
   `0x8020AC78` itself; `fn_801fce20` therefore only calls it once and does
   not call the submitter separately.
7. **No staging.** All bodies live in `src/runtime/input.rs`; no other Rust
   file, generated doc or `ledger/functions.csv` was touched.

## Deliberate skips

Every skipped function is in the same two neighbouring blocks but could not
be reconstructed honestly in this pass:

- **`0x801FD0E0` (140 B) and `0x801FD16C` (240 B).** Both begin with
  `FUN_802DCD74`, which is not a real callee: it is a decompiler-split
  epilogue that stores `r27`–`r31` through `r11` and returns. The functions
  then consume the `undefined8` "return value" as if it were a live
  `{r3, r4}` pair (`(int)(uVar >> 32)` becomes a pointer argument). Any Rust
  body would have to invent the missing register story, so both stay
  untranslated. `0x801FDBBC` uses the same epilogue artifact and is skipped
  for the same reason.
- **`0x801FBE08` (144 B).** The stack frame is three overlapping anonymous
  arrays plus an uninitialised byte, and the callback receives the whole
  12-byte frame. The field roles are not provable from the export.
- **`0x801FC104`, `0x801FC1C4`, `0x801FC2FC`, `0x801FD4BC`, `0x801FD83C`.**
  These are larger DM/SDP state handlers whose bodies reference the same
  split frames and unrecovered jump tables; they belong to a later DM pass.
- **The four-byte thunks `0x801F8630`–`0x801F8660`, `0x801F87B0`,
  `0x801F87F0` and `0x801F3300`.** Prologue/epilogue artifacts, unchanged
  from the wave-34 skip list.
- **The rest of the band** (BTU bring-up `0x801F0020`–`0x801F35C0`, the
  HID-host link management, the big BTU loop `0x801F9FA0`, the subsystem
  bring-up `0x801FAB34`–`0x801FDD2C` and the message plumbing
  `0x801FDE58`–`0x801FFEC4`) remains for later passes; this pass deliberately
  stayed inside one contiguous DM slice.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- No address was already in `ledger/functions.csv` (the 191-entry
  untranslated list for the band was derived from it first).
- `cargo check --message-format=short` exits 0; the only new warning is the
  existing-style "direct cast of function item into an integer" for the
  self-registration in `fn_801fbee4`.
- `node tools/check-lane-csv.mjs wave50` is clean for
  `ledger/agents/wave50-input.csv` (28 rows).

## Ledger

Machine-readable rows: `ledger/agents/wave50-input.csv` (28 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
