# Coverage — Wave 74 (`runtime::input`, WUD device manager and BTM event handlers)

## How to read this file

1. **Summary** gives the per-file totals and the band inventory for
   `[0x801F0000, 0x80200000)`.
2. **What this pass covers** describes the four clusters of code added.
3. The **function table** lists all 28 bodies in address order with the role,
   the byte count re-read from `work/ghidra/main-gb/export/functions.tsv` and
   the confidence carried by the doc block.
4. **Corrections and caveats** records the places where the export needed an
   inference, and **Deliberate skips** says what was left out and why.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for; all 28 rows are
`translated`. Names stay `fn_<addr>` because the image has no symbol table for
this layer; the doc comments carry the roles and the `// inferred:` /
`// unknown:` markers repeat them at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 28 | 3,572 | 17 | 11 |

Band inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the band | 268 |
| Untranslated before this pass | 133 |
| Selected and translated by this pass | **28** |
| Untranslated in the band after this pass | 105 |

## What this pass covers

The pass fills the WUD device-manager band that sits below the HID-host layer
translated in wave 61, and the HID-host transmit/BTM glue that sits beside it.
Four shapes of code repeat:

- **WUD state bytes and flush callbacks (`0x801F1620`–`0x801F2CC0`).** Three
  callbacks write the init/shutdown state bytes: `0x803A312D` becomes `8`
  (`fn_801f1620`), `0x803A3130` becomes `5` (`fn_801f1f20`) and `0x803A3131`
  becomes `3` (`fn_801f2cc0`). `fn_801f2d10` is the only transition point for
  the shutdown byte: state `1` (active) tests the Bluetooth readiness probe
  `0x801DB8E0` and the AND of three 32-bit masks (`r13-0x4B4C`,
  `0x801DD500(0x803A39D8)`, `0x801DD520(0x803A5080)`); a clear result closes
  immediately (`3`), otherwise the state drains (`2`) behind the flush
  callback. State `3` stops the `0x803A3830` alarm and calls `0x801FCEF8`.
- **`WUDSet*Callback` setters (`0x801F3630`, `0x801F3690`, `0x801F3B90`,
  `0x801F3BF0`).** One shape, four callbacks at `0x803A3124`, `0x803A3128`,
  `0x803A380C` and `0x803A3810`: trace, `0x801B1140`/`0x801B1180` interrupt
  lock, read the old word, store the new one, return the old one.
  `fn_801f3c50` is the byte-wide cousin: it stores `0x803A380B`/`0x803A380A`
  under the same lock and forwards both to `0x801FCFCC`.
- **Link state and channel filter (`0x801F3840`–`0x801F3CB0`).** The link
  state byte `0x803A3828` is read by both poll helpers; when it is `3` they
  close a pending removal (`0x803A3808` non-zero and `0x803A312C` equal to `3`
  runs `0x801FD0A8`) and report success. `fn_801f3860` additionally sets
  `r13-0x4B48`, `fn_801f38f0` does not. `fn_801f3aa0` maps channel `n` to the
  AFH window `(n+1)*5 ± 14`, clamped to `0..=0x4E`, and calls the routine the
  trace string names `BTM_SetAfhChannels`.
- **HID-host pumps and BTM bring-up (`0x801F7F60`–`0x801FB040`).**
  `fn_801f7f60` zeroes the 0xA8-byte GKI statistics block at `0x803CDB00` (42
  word stores). `fn_801f92c0` and `fn_801f93b0` are the two send pumps: free
  the finished buffer, decrement the in-flight counter (`0x803CDE28` /
  `0x803CDE18`), and while that counter is below `5` and the matching list
  (`0x803CDE1C` / `0x803CDE0C`) is non-empty, pop one entry and post it with
  `0x801EFE60` (opcode `0x20`, nine arguments) or `0x801EF810` (six
  arguments, channel byte `0x803CDDF0`), re-queueing on a negative result.
  `fn_801fab34` is the BTM bring-up; `fn_801fadb8`, `fn_801fae00`,
  `fn_801fae70` and `fn_801faf0c` are the per-event handlers that resolve the
  peer key through `0x8020A68C` and hand an eight-byte key to a BTM request
  builder; `fn_801fafe0` and `fn_801fb040` install and drive the DM callback
  pair `0x803D1798`/`0x803D17A0`.

Cross-layer facts reused: `0x801F6A70` is the no-op trace printer, `0x801F8760`
and `0x801F8730` the GKI lock pair, `0x801F78F0`/`0x801F77B0` the list pop/push
helpers, `0x801F7240` the payload free, `0x801F6FB0` the GKI `getbuf`,
`0x801FA850` the HID-host post, `0x801FA478`/`0x801FA488` the two
`bta` callback installers, and `0x80201CD8` the six-byte link-key store.

## Functions

| Address | Function | Role | Bytes | Status | Confidence | Evidence |
|---|---|---|---:|---|---|---|
| `0x801F1620` | `fn_801f1620` | delete-flush callback: `0x803A312D = 8` | 88 | translated | high | `work/ghidra/main-gb/export/801f1620_FUN_801f1620.c` |
| `0x801F1680` | `fn_801f1680` | flush link keys of the 16 device records; returns `3` | 152 | translated | high | `work/ghidra/main-gb/export/801f1680_FUN_801f1680.c` |
| `0x801F1F20` | `fn_801f1f20` | init-flush callback: `0x803A3130 = 5` | 76 | translated | high | `work/ghidra/main-gb/export/801f1f20_FUN_801f1f20.c` |
| `0x801F2CC0` | `fn_801f2cc0` | shutdown-flush callback: `0x803A3131 = 3` | 76 | translated | high | `work/ghidra/main-gb/export/801f2cc0_FUN_801f2cc0.c` |
| `0x801F2D10` | `fn_801f2d10` | shutdown state machine (`1` active / `2` drain / `3` closed) | 180 | translated | medium | `work/ghidra/main-gb/export/801f2d10_FUN_801f2d10.c` |
| `0x801F32B0` | `fn_801f32b0` | installs app alloc/free callbacks under the lock | 76 | translated | high | `work/ghidra/main-gb/export/801f32b0_FUN_801f32b0.c` |
| `0x801F35C0` | `fn_801f35c0` | builds the 9-byte channel request and sends it | 108 | translated | medium | `work/ghidra/main-gb/export/801f35c0_FUN_801f35c0.c` |
| `0x801F3630` | `fn_801f3630` | swap sync-device callback `0x803A3124` | 92 | translated | high | `work/ghidra/main-gb/export/801f3630_FUN_801f3630.c` |
| `0x801F3690` | `fn_801f3690` | swap clear-device callback `0x803A3128` | 92 | translated | high | `work/ghidra/main-gb/export/801f3690_FUN_801f3690.c` |
| `0x801F3840` | `fn_801f3840` | starts a connection attempt `(1, -1, 0, 1)` | 20 | translated | high | `work/ghidra/main-gb/export/801f3840_FUN_801f3840.c` |
| `0x801F3860` | `fn_801f3860` | link-up poll + one-shot removal; sets `r13-0x4B48` | 144 | translated | high | `work/ghidra/main-gb/export/801f3860_FUN_801f3860.c` |
| `0x801F38F0` | `fn_801f38f0` | link-up poll + one-shot removal | 136 | translated | high | `work/ghidra/main-gb/export/801f38f0_FUN_801f38f0.c` |
| `0x801F3AA0` | `fn_801f3aa0` | AFH channel filter window | 232 | translated | medium | `work/ghidra/main-gb/export/801f3aa0_FUN_801f3aa0.c` |
| `0x801F3B90` | `fn_801f3b90` | swap HID receive callback `0x803A380C` | 92 | translated | high | `work/ghidra/main-gb/export/801f3b90_FUN_801f3b90.c` |
| `0x801F3BF0` | `fn_801f3bf0` | swap HID connection callback `0x803A3810` | 92 | translated | high | `work/ghidra/main-gb/export/801f3bf0_FUN_801f3bf0.c` |
| `0x801F3C50` | `fn_801f3c50` | stores two mode bytes and forwards them | 88 | translated | high | `work/ghidra/main-gb/export/801f3c50_FUN_801f3c50.c` |
| `0x801F3CB0` | `fn_801f3cb0` | firmware banner + `0x801F4350` | 48 | translated | high | `work/ghidra/main-gb/export/801f3cb0_FUN_801f3cb0.c` |
| `0x801F5C80` | `fn_801f5c80` | device status callback: USB fatal warning | 80 | translated | high | `work/ghidra/main-gb/export/801f5c80_FUN_801f5c80.c` |
| `0x801F7F60` | `fn_801f7f60` | clears the 0xA8-byte GKI statistics block | 188 | translated | high | `work/ghidra/main-gb/export/801f7f60_FUN_801f7f60.c` |
| `0x801F92C0` | `fn_801f92c0` | send pump for the `0x803CDE1C` list | 236 | translated | medium | `work/ghidra/main-gb/export/801f92c0_FUN_801f92c0.c` |
| `0x801F93B0` | `fn_801f93b0` | send pump for the `0x803CDE0C` list | 224 | translated | medium | `work/ghidra/main-gb/export/801f93b0_FUN_801f93b0.c` |
| `0x801FAB34` | `fn_801fab34` | BTM bring-up and callback registration | 272 | translated | medium | `work/ghidra/main-gb/export/801fab34_FUN_801fab34.c` |
| `0x801FADB8` | `fn_801fadb8` | relays two connection bytes to BTM | 72 | translated | medium | `work/ghidra/main-gb/export/801fadb8_FUN_801fadb8.c` |
| `0x801FAE00` | `fn_801fae00` | peer key + four-argument BTM request | 112 | translated | medium | `work/ghidra/main-gb/export/801fae00_FUN_801fae00.c` |
| `0x801FAE70` | `fn_801fae70` | peer key + two-form BTM request | 156 | translated | medium | `work/ghidra/main-gb/export/801fae70_FUN_801fae70.c` |
| `0x801FAF0C` | `fn_801faf0c` | peer key + one feature bit + BTM request | 212 | translated | medium | `work/ghidra/main-gb/export/801faf0c_FUN_801faf0c.c` |
| `0x801FAFE0` | `fn_801fafe0` | installs the DM callback pair, starts discovery setup | 96 | translated | high | `work/ghidra/main-gb/export/801fafe0_FUN_801fafe0.c` |
| `0x801FB040` | `fn_801fb040` | stops/defers the remote-name discovery | 132 | translated | medium | `work/ghidra/main-gb/export/801fb040_FUN_801fb040.c` |

Totals: **28 translated**, 0 partial, 0 stubs; 17 high / 11 medium / 0 low;
3,572 bytes. Address range `0x801F1620`–`0x801FB040`.

## Corrections and caveats

1. **Nine-argument call.** `0x801EFE60`, used by `fn_801f92c0`, takes nine
   arguments and `crate::platform` stops at `call7`. The wave adds a private
   `call9` helper next to the bodies, spelled with the same
   `core::mem::transmute` technique the file already uses for `call_f64_1`.
2. **Function-pointer arguments are written as literals.** The callbacks this
   layer installs are passed to callees as `0x801F_2CC0`, `0x801F_92C0`,
   `0x801F_93B0`, `0x801F_C574` and so on, with a comment naming the function.
   `cargo check` reports the two pre-existing "direct cast of function item
   into an integer" warnings (lines 5286 and 10482) and no new one.
3. **`fn_801f2d10` masks.** The export ANDs `r13-0x4B4C` with the results of
   `0x801DD500` and `0x801DD520`. The identity of each mask source is not
   stated anywhere in the image, so the body keeps the literal AND and the doc
   marks the roles as inferred; confidence stays `medium`.
4. **`fn_801f35c0` handle width.** `0x803A382A` is passed straight into the
   argument register and has no other reference in the band. It is read as a
   16-bit value because the record it feeds is halfword-sized; `medium`.
5. **`fn_801fab34` configuration halfwords.** `0x802F6D4C`, `0x802F6D4E` and
   `0x802F6D50` are two bytes apart and only ever used as call arguments, so
   they are read as 16-bit values (`// inferred:` at the constants).
6. **`fn_801f93b0` channel byte.** `0x803CDDF0` is written as `2` by
   `0x801F9490` (which is still untranslated), so the byte width comes from
   that sibling write rather than from this body; `medium`.
7. **`fn_801f7f60` loop form.** The export contains 42 consecutive
   `= 0` word stores at `0x803CDB00`..`0x803CDBA4` and nothing else; the Rust
   loop covers exactly that range in the same order.
8. **No staging.** All 28 bodies live in `src/runtime/input.rs`; no other Rust
   file, generated doc or `ledger/functions.csv` was touched.

## Deliberate skips

- **`0x801F8820` (920 B), `0x801F8BC0` (428 B), `0x801F9840` (524 B),
  `0x801F9490` (520 B) and `0x801F96A0` (396 B).** The HID-host receive path.
  Every one of them reads a live register pair out of `FUN_802DCD6C` /
  `FUN_802DCD74`/`FUN_802DCD70` (`extraout_r4`, a 64-bit split return), so the
  decompiled body is a split-frame artifact and the arguments would have to be
  invented. They are the same family the wave-50/61 docs already skip.
- **`0x801F9D10`–`0x801F9DD0` (7 × 20 B), `0x801F9830` (8 B),
  `0x801F8E80` (60 B), `0x801FF808` (168 B), `0x801FFEC4` (316 B) and
  `0x801FF970` (852 B).** Carried over from the wave-61 skip list; the
  reasoning there still holds (variadic forwarding with no recoverable arity,
  a single-`return` artifact, an unprovable global width, live-register
  splits, and a state machine whose private stub already exists).
- **`0x801F6A00` (56 B).** Its body tests `in_r8`, a register the ABI does not
  pass for this signature, and then jumps through `0x803A380C` with no
  recoverable argument list. The setter for that callback (`fn_801f3b90`) is
  translated this pass; the dispatcher is left for a lane that can recover the
  arity.
- **The BTA HID/SDP state handlers `0x801FBB40`–`0x801FD83C`** and the BTM
  teardown/registration middle `0x801FB0C4`–`0x801FB89C` remain for later
  passes; they are larger, split-frame-heavy and mostly call into the same
  untranslated `0x801FD4BC`/`0x801FDC34` pair.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 3,572 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- No address was already in `ledger/functions.csv`; the 133-entry untranslated
  list for the band was derived from it first.
- `cargo check --message-format=short` reports no errors; the only warnings in
  this file are the two pre-existing "direct cast of function item into an
  integer" sites (lines 5286 and 10482), which this pass did not touch.
- `node tools/check-lane-csv.mjs wave74` prints `OK` and reports
  `wave74-input.csv: 28 rows`.

## Ledger

Machine-readable rows: `ledger/agents/wave74-input.csv` (28 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
