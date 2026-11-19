# Coverage wave 13 - DVD queue, completion and error-log tail

## How to read this file

This file records what wave 13 added to `src/runtime/dvd.rs`, not everything
the address band contains. Read it in five steps:

1. **Scope** gives the band, the selection rule and the byte count.
2. **Summary** gives the function/byte counts and status split.
3. **The function table** lists every added body in address order with its
   plain-English role, status, confidence and exact Ghidra evidence file.
4. **What this pass adds** explains the three clusters in prose.
5. **Unknowns / next targets / gates** record what is still open.

Companion ledger: `ledger/agents/wave13-dvd.csv` (one row per function, same
ordering as the table below). The global `ledger/functions.csv` received the
same rows under `runtime::dvd`.

Status vocabulary (`docs/RUST_STYLE.md`): `translated` means complete recovered
control flow and every call accounted for; `partial` means entry/exit and major
effects only; `stub` means address/role only. **Confidence** measures evidence
strength for the *semantics*, not code quality.

Every Rust function keeps `# Original` with the Ghidra name, address, size and
evidence path. Descriptive names are used only where the body, caller or the
queue layout proves the role; otherwise the function keeps its `fn_80xxxxxx`
name. `// observed:`, `// inferred:` and `// unknown:` markers in the Rust
carry the same information at the call site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`.

| Item | Count |
|---|---:|
| Functions selected | 21 |
| Status `translated` | 21 |
| Status `partial` | 0 |
| Bytes represented | 7,280 |
| Address span | `0x801CDD30..0x801D6290` |

The selection rule was: an entry in `[0x801C0000, 0x80200000)` that is **not**
already in `ledger/functions.csv` or `src/runtime/dvd.rs`, with a full Ghidra
body, and that still belongs to the DVD / FST / IOS cluster. The remaining
`0x801CDD30..0x801D0950` block is the state machine, its transfer queues and
the error-record callbacks; `0x801D6230`/`0x801D6290` are the open/close pair
that `dvd_init_801cad30` uses on its drive-probe branch. The heap routines at
`0x801D3880..0x801D3E30` (tags `EXPH`/`FRMH`/`UNTH`) were deliberately
excluded: they are a heap manager, not a DVD accessor.

## `runtime::dvd` - `src/runtime/dvd.rs`

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | role (plain English) | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x801CDD30` | 1,456 | `dvd_state_step_801cdd30` | executes one state-machine step and submits the matching drive command | translated | high | `801cdd30_FUN_801cdd30.c` |
| `0x801CE2E0` | 2,528 | `dvd_completion_801ce2e0` | shared completion: record status, retry, finish or reset the state object | translated | high | `801ce2e0_FUN_801ce2e0.c` |
| `0x801CEDB0` | 216 | `dvd_queue_inquiry_801cedb0` | fills a descriptor with the `0x0E` 0x20-byte command and queues it on ring 2 | translated | high | `801cedb0_FUN_801cedb0.c` |
| `0x801CEE90` | 200 | `dvd_descriptor_state_801cee90` | maps a descriptor step word to the caller-visible state code | translated | medium | `801cee90_FUN_801cee90.c` |
| `0x801CEF60` | 156 | `dvd_current_state_801cef60` | fatal/restart/idle guard around the descriptor state code | translated | high | `801cef60_FUN_801cef60.c` |
| `0x801CF4B0` | 204 | `fn_801cf4b0` | one cover-register poll with a bus-clock settle timeout | translated | medium | `801cf4b0_FUN_801cf4b0.c` |
| `0x801CF580` | 568 | `dvd_begin_transfer_801cf580` | arms or rejects a prepared transfer according to the current state | translated | medium | `801cf580_FUN_801cf580.c` |
| `0x801CF7C0` | 284 | `dvd_abort_transfers_801cf7c0` | resets the rings, drains queued nodes and completes the active descriptor | translated | high | `801cf7c0_FUN_801cf7c0.c` |
| `0x801CF8F0` | 312 | `dvd_abort_transfers_sync_801cf8f0` | synchronous abort: drain, re-enable interrupts and wait for the latch | translated | high | `801cf8f0_FUN_801cf8f0.c` |
| `0x801CFA30` | 28 | `fn_801cfa30` | tests for the `0x80395820` sentinel, else calls `0x801D3870` | translated | low | `801cfa30_FUN_801cfa30.c` |
| `0x801CFA70` | 56 | `dvd_queue_list_init_801cfa70` | reinitialises the four circular queue rings | translated | high | `801cfa70_FUN_801cfa70.c` |
| `0x801CFAB0` | 104 | `dvd_queue_insert_801cfab0` | appends one node at the tail of a ring | translated | high | `801cfab0_FUN_801cfab0.c` |
| `0x801CFC20` | 100 | `dvd_queue_first_801cfc20` | returns the first non-empty ring's head node | translated | high | `801cfc20_FUN_801cfc20.c` |
| `0x801CFD20` | 92 | `dvd_error_log_cb_sync_801cfd20` | syncs the error log file and reports `(2, 0)` on success | translated | high | `801cfd20_FUN_801cfd20.c` |
| `0x801CFD80` | 252 | `dvd_error_log_cb_write_801cfd80` | rewrites the 0x80-byte error block when the expected byte count arrives | translated | high | `801cfd80_FUN_801cfd80.c` |
| `0x801CFE80` | 340 | `dvd_error_log_cb_seek_801cfe80` | seeks to the next 0x80-byte slot and falls back to a rewrite at offset `-1` | translated | high | `801cfe80_FUN_801cfe80.c` |
| `0x801CFFE0` | 188 | `dvd_error_log_cb_slot_801cffe0` | advances the slot modulo 7 and rewrites the secondary block | translated | high | `801cffe0_FUN_801cffe0.c` |
| `0x801D0930` | 20 | `dvd_error_reporter_active_801d0930` | non-zero test for the installed drive-reset reporter | translated | high | `801d0930_FUN_801d0930.c` |
| `0x801D0950` | 20 | `dvd_error_reporter_call_801d0950` | invokes the drive-reset reporter when installed | translated | high | `801d0950_FUN_801d0950.c` |
| `0x801D6230` | 84 | `fn_801d6230` | opens the auxiliary IOS device used by the init probe | translated | medium | `801d6230_FUN_801d6230.c` |
| `0x801D6290` | 72 | `fn_801d6290` | closes that auxiliary device and clears its descriptor | translated | medium | `801d6290_FUN_801d6290.c` |

## What this pass adds

### 1. State machine and completion

`dvd_state_step_801cdd30` is the per-step driver. It installs itself in
`r13-0x4CCC`, records one event-ring entry for the kinds that carry a disc
position, and dispatches on the `+0x08` kind word to the matching low-level
builder (`0x801D2F20`, `0x801D2130`, `0x801D14A0`, `0x801D1630`,
`0x801D18A0`, `0x801D1B10`, `0x801D1D20`, `0x801D1FC0`, `0x801D22D0`,
`0x801D2470`, `0x801D28E0`, `0x801D2D90`, `0x801D3280`, `0x801D3700`), all
with `0x801CE2E0` as completion. The two streamed-read arms share the clamped
`0x80000`-byte step helper.

`dvd_completion_801ce2e0` is the other half: it records the status in the
event ring, maps status bits `0x10`/`0x20` to the two special callbacks, and
otherwise resolves the current kind. It grows the completed byte count for
kinds `1/4/5/0x21/0x0E` plus `r13-0x6054`, completes a waiting descriptor with
reason `0` or `7`, or resets the state object and calls its `+0x28` callback.
The `0x24`/`0x26` branches read the cover word and the saved 64-bit timestamp;
the `0x28` branch compares or copies the 0x20-byte partition header through
`0x801D0710`/`0x80004364`; the `0x29` branch either advances or completes.

### 2. Transfer queues and aborts

`dvd_queue_list_init_801cfa70`, `dvd_queue_insert_801cfab0` and
`dvd_queue_first_801cfc20` are the ring primitives behind the existing
pop/has-work/unlink functions. Rings are four 8-byte head/tail pairs at
`0x803958B0`; a node stores `next` at `+0` and `previous` at `+4`.

`dvd_abort_transfers_801cf7c0` and `dvd_abort_transfers_sync_801cf8f0` reset
all four rings and drain them through `0x801CF060`. The asynchronous variant
takes the caller's callback for the active descriptor; the synchronous one
uses the fixed label `0x801CF8E0`, re-enables interrupts and spins until
`r13-0x4D1C` is `1`.

### 3. Error-record callbacks and probe device

The four `0x801CFD20..0x801CFFE0` callbacks implement the
`/shared2/test2/dvderror.dat` write chain: seek to `(slot+1)*0x80`, rewrite
the 0x80-byte block, sync, then bump the seven-slot index and rewrite the
secondary block. The reporter hooks `0x801D0930`/`0x801D0950` expose the
drive-reset callback installed by `0x801D08D0`.

`fn_801d6230`/`fn_801d6290` are the `IOS_Open`/`IOS_Close` pair used by
`dvd_init_801cad30` before it reads the drive identity. The device path is the
`r13-0x6018` buffer; the descriptor is `r13-0x6020`.

## Unknowns and partials

1. **Private hook `r13-0x6060`.** The default arm of `0x801CDD30` calls this
   pointer with `(state, 0x801CE2E0)`. No writer was recovered in this lane, so
   the Rust keeps it by address and marks it `// unknown:`.
2. **Sentinel predicate.** `0x801CFA30` returns `1` for `0x80395820` and
   otherwise calls the 8-byte `0x801D3870`; neither the sentinel nor the
   predicate's return meaning is established.
3. **Cover bits.** `0x801CF4B0` only uses bit `0` of the polled word; the
   `0x24`/`0x26` branches also read bit `2`. The bit names are not proven.
4. **State-kind names.** Kinds `1,4,5,9,10,0x0B..0x0E,0x10,0x21,0x22,0x24..0x2A`
   are observed values. The Rust names only the actions, never the public SDK
   command names.
5. **`0x80341930` series.** Three extra kinds are loaded from these globals;
   their values and roles are unresolved.
6. **Probe device identity.** The `r13-0x6018` path is not present in the
   export, so `fn_801d6230`/`fn_801d6290` keep address names; the docs say
   "auxiliary IOS device" rather than guessing a device node.
7. **`IOS_Close` argument in `0x801D6290`.** Ghidra drops the visible argument;
   the Rust passes the descriptor stored at `r13-0x6020`, matching the
   open/close shape used by `dvd_low_finish_801d11e0`.
8. **Excluded heap block.** `0x801D3880..0x801D3E30` builds `EXPH`/`FRMH`/`UNTH`
   heap headers with `0x801D55C0`/`0x801D56C0`; it is not a DVD routine and was
   left for a memory-management lane.

## Next targets

Closest remaining bodies in the same `main.dol` band:

| Address | Size | Why next |
|---|---:|---|
| `0x801D3880` | 320 | heap interval lookup (`EXPH`/`FRMH` tree); needs a memory lane |
| `0x801D39C0` | 448 | heap node insert with semaphores |
| `0x801D3E30` | 556 | heap block split/merge (`RF`/`DU` tags) |
| `0x801D64A0` | 164 | remaining auxiliary-device ioctl wrapper |
| `0x801D6550` | 236 | remaining auxiliary-device ioctl wrapper |

## Gates

- `cargo check` -> `Finished dev profile`, no diagnostics in
  `src/runtime/dvd.rs`.
- `node tools/doc-lint.mjs src` -> `ok: true`, 0 violations (68 files).
- No files outside this lane were edited; only `src/runtime/dvd.rs`,
  `ledger/functions.csv`, `ledger/agents/wave13-dvd.csv` and this file changed.
