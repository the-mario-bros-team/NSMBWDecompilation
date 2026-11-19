# Coverage wave 3 - DVD/FST and WPAD/KPAD second pass

## How to read this file

- **Scope** - 23 previously untranslated functions from `main.dol` in the
  `0x801C0000..0x80200000` band, split across two reader-facing modules:
  `src/runtime/dvd.rs` (`runtime::dvd`) and `src/runtime/input.rs`
  (`runtime::input`). The pass finishes the asynchronous/event side of the DVD
  layer and the WPAD start plus KPAD status/callback extension helpers.
- **Addresses** are virtual `main.dol` addresses, not file offsets. The image
  base is `0x80004000`.
- **Evidence** means the Ghidra C under
  `work/ghidra/main-gb/export/`; every row names the exact file.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = complete recovered
  control flow and every call accounted for; `partial` = entry/exit plus major
  effects; `stub` = address/role only.
- **Confidence** measures the strength of the evidence for the translation, not
  code quality. `medium` marks an exact body whose public name, callback label
  or numeric meaning is inferred.
- **Start here** - for DVD, read `dvd_init_801cad30` (bring-up), then
  `fn_801cb4f0` (completion/error dispatch), then
  `dvd_queue_transfer_801cecc0` (the queue every read uses). For input, read
  `wpad_start_801dfa50`, then `wpad_init_801df7f0` and
  `wpad_control_init_801df570`; the extension accessors at the end show the
  per-channel control-block slots.
- **Companion ledger** - `ledger/agents/wave3-io.csv`.

## Summary

| module | functions | bytes (`functions.tsv`) | address span | status | unknowns |
|---|---:|---:|---|---|---:|
| `runtime::dvd` | 12 | 3,656 | `0x801CAD30..0x801D3868` | 12 translated | 4 callback labels + 1 undefined register + 2 drive magics |
| `runtime::input` | 11 | 1,180 | `0x801DFA50..0x801E1624` | 10 translated + 1 partial | 1 callback label + 1 unresolved argument pair + 1 list node |
| **total** | **23** | **4,836** | `0x801CAD30..0x801D3868` | 22 translated + 1 partial | |

The two modules are independent. `runtime::dvd` now covers the moment a
completion/error word arrives through the four-context IOS ioctl ring.
`runtime::input` now covers the outer WPAD start gate and the KPAD status,
link-key and callback-slot helpers that sit on top of the existing
`WPADInit`/`KPADSet*Callback` code.

## Function table

Evidence paths are relative to `work/ghidra/main-gb/export/`.

### `runtime::dvd` - `src/runtime/dvd.rs`

| address | size | Rust function | role | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x801CAD30` | 340 | `dvd_init_801cad30` | one-shot drive probe, FST cache, queue init and reset handler | translated | medium | `801cad30_FUN_801cad30.c` |
| `0x801CB1B0` | 272 | `fn_801cb1b0` | low-level error callback; fatal state or retry alarm | translated | medium | `801cb1b0_FUN_801cb1b0.c` |
| `0x801CB320` | 244 | `dvd_classify_error_801cb320` | classify raw error words and track repeats | translated | medium | `801cb320_FUN_801cb320.c` |
| `0x801CB420` | 196 | `fn_801cb420` | record event `0x10` and continue the low-level retry | translated | medium | `801cb420_FUN_801cb420.c` |
| `0x801CB4F0` | 1,284 | `fn_801cb4f0` | completion dispatcher: retry, fatal, restart, delayed alarm | translated | medium | `801cb4f0_FUN_801cb4f0.c` |
| `0x801CECC0` | 228 | `dvd_queue_transfer_801cecc0` | fill descriptor and submit one transfer | translated | high | `801cecc0_FUN_801cecc0.c` |
| `0x801CF000` | 16 | `fn_801cf000` | get/set the optional read hook | translated | low | `801cf000_FUN_801cf000.c` |
| `0x801CF010` | 80 | `fn_801cf010` | clear busy flag and restart the state machine | translated | low | `801cf010_FUN_801cf010.c` |
| `0x801CF3D0` | 192 | `fn_801cf3d0` | wait for a command block and cancel it | translated | medium | `801cf3d0_FUN_801cf3d0.c` |
| `0x801D2750` | 12 | `fn_801d2750` | store the timer hook word and return success | translated | low | `801d2750_FUN_801d2750.c` |
| `0x801D2F20` | 432 | `dvd_low_read_fst_801d2f20` | submit the low-level FST read command `0x71` | translated | medium | `801d2f20_FUN_801d2f20.c` |
| `0x801D3700` | 360 | `dvd_low_clear_cover_interrupt_801d3700` | submit the cover-interrupt command `0x86` | translated | medium | `801d3700_FUN_801d3700.c` |

### `runtime::input` - `src/runtime/input.rs`

| address | size | Rust function | role | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x801DFA50` | 112 | `wpad_start_801dfa50` | register the idle hook, run BTA init, bring up WPAD | translated | medium | `801dfa50_FUN_801dfa50.c` |
| `0x801DFBD0` | 104 | `fn_801dfbd0` | enable/disable Bluetooth when the host is ready | translated | low | `801dfbd0_FUN_801dfbd0.c` |
| `0x801E0CE0` | 80 | `fn_801e0ce0` | HID parser failure report for one channel | partial | low | `801e0ce0_FUN_801e0ce0.c` |
| `0x801E0E40` | 192 | `fn_801e0e40` | drop the link key and queue the next KPAD event | translated | medium | `801e0e40_FUN_801e0e40.c` |
| `0x801E0F40` | 140 | `fn_801e0f40` | read link handle/status byte with `-1`/`-2` overrides | translated | medium | `801e0f40_FUN_801e0f40.c` |
| `0x801E10F0` | 92 | `fn_801e10f0` | swap callback word `+0x8A0` | translated | low | `801e10f0_FUN_801e10f0.c` |
| `0x801E1150` | 68 | `fn_801e1150` | read word `+0x8B8` | translated | low | `801e1150_FUN_801e1150.c` |
| `0x801E1260` | 84 | `fn_801e1260` | invoke and clear extension callback `+0x988` | translated | medium | `801e1260_FUN_801e1260.c` |
| `0x801E14E0` | 204 | `fn_801e14e0` | update per-channel `+0x8D4` mode | translated | medium | `801e14e0_FUN_801e14e0.c` |
| `0x801E15B0` | 52 | `fn_801e15b0` | set the global `r13-0x4BC8` gate | translated | medium | `801e15b0_FUN_801e15b0.c` |
| `0x801E15F0` | 52 | `fn_801e15f0` | read the global `r13-0x4BC8` gate | translated | medium | `801e15f0_FUN_801e15f0.c` |

## What the DVD pass adds

`fn_801cb4f0` is the centre of the low-level completion path:

1. The status word is copied into the event-ring header and
   `0x80395AB8` under the interrupt lock.
2. `0x10`/`0x20` register one of the two drive-specific retry callbacks.
3. Other even words are classified by `dvd_classify_error_801cb320`.
4. Classification `1` installs the default error callback; `2` installs
   `fn_801cb420`; `3` either reads the block at `state+0x10` or calls the
   registered completion callback.
5. Families `0x01000000`/`0x03000000` arm the delayed-retry alarm; `02` enters
   the retry state; `0`/`other` install the default callback.
6. When `r13-0x4D50` is set the dispatcher finishes the current state with
   `-3`, calls both completion slots and restarts `0x801CDA10`.

The transfer path is `dvd_queue_transfer_801cecc0` (descriptor fields `+8`,
`+0x10`, `+0x14`, `+0x18`, `+0x20`, `+0x28`, `+0x0C`; optional
`DCInvalidateRange`; queue insert `0x801CFAB0`). `dvd_low_read_fst_801d2f20`
and `dvd_low_clear_cover_interrupt_801d3700` share the four 0x20-byte context
records at `0x80395BE0` (callback, queued flag, state byte, `0xFEEBDAED`
sentinel) and submit commands through the eight-argument
`IOS_IoctlAsync`-shaped wrapper at `0x80225500`.

## What the input pass adds

`wpad_start_801dfa50` sets the start latch, inserts `0x80343B18` into the OS
function list once, and calls the existing `wpad_init_801df7f0` only when
`0x801F3190` reports a fresh Bluetooth initialisation.

The extension helpers show the per-channel slots that the public setters do
not expose:

| slot | reader/writer | observed role |
|---|---|---|
| `+0x8A0` | `fn_801e10f0` | callback word, swapped under the lock |
| `+0x8B8` | `fn_801e1150` | word read under the lock |
| `+0x8BC` | several | Bluetooth link handle (`-1` = no link) |
| `+0x8C1` | `fn_801e0f40` | byte returned to the caller; `0xFD` maps to `-1` |
| `+0x8D4`/`+0x8D0` | `fn_801e14e0` | mode word plus dirty flag |
| `+0x988`/`+0x98C` | `fn_801e1260` | extension callback plus its active byte |
| `+0x986` | `fn_801e0e40` | event-queued byte |

## Unknowns and partials

One function is marked `partial`: `fn_801e0ce0` has an unresolved argument pair
to the HID parser. Every other function has all recovered branches and stores
present. The following values remain unknown and are called out in the Rust
docs:

1. `0x801CB2C0`, `0x801CB1A0`, `0x801CD9E0`, `0x801CF490` and `0x801E0E10`
   are code labels Ghidra did not export as functions. They are passed as raw
   callback addresses.
2. `fn_801cb1b0` passes `0` for the undefined `extraout_r4` argument of
   `0x801AB540`; the callee ignores that register.
3. `fn_801e0ce0` calls `0x801E6BE0` with no visible arguments even though the
   callee body reads `r3`/`r4` as a report pointer and channel. The Rust call
   keeps the recovered no-argument shape, marks the function `partial`, and
   leaves the registers unresolved.
4. The numeric error families and status values in `fn_801cb4f0`,
   `dvd_classify_error_801cb320` and `fn_801cb1b0` are grouped by observed
   dispatch only; no official names were invented.
5. The roles of `fn_801cf000`, `fn_801cf010` and `fn_801d2750` are low
   confidence: the bodies are exact but no caller or string identifies them.
6. `dvd_init_801cad30` and `wpad_start_801dfa50` are descriptive names inferred
   from their position in the bring-up sequence. Their addresses stay in the
   function names.

## Next targets

The closest remaining bodies in the same band are:

- `0x801D25E0` and `0x801D2760` - sibling IOS ioctl context builders
  (`0xE0` request-error and reset commands).
- `0x801CDA10`, `0x801CDD30`, `0x801CE2E0` - the low-level DVD state machine
  that every dispatcher restarts.
- `0x801CC240`, `0x801CC860`, `0x801CC9F0` - disc-change FST helpers.
- `0x801E12C0` and `0x801E1630` - KPAD event queue writers whose Ghidra bodies
  contain `FUN_802dcd68`/`0x802dcd70` register-save artifacts; they need the
  raw instruction words before a faithful call signature can be written.
- `0x801DFE00` and `0x801E0820` - WPAD command builder and data handler.

## Gates

- `node tools/doc-lint.mjs src` -> `ok: true`, 0 violations after this pass.
- `cargo check` -> no diagnostics in `src/runtime/dvd.rs` or
  `src/runtime/input.rs`. The two files were also checked in an isolated crate
  with `platform.rs` and `runtime/os.rs` (exit 0); the full workspace build is
  shared with the other lanes.
