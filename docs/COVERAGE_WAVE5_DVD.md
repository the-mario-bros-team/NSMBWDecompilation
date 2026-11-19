# Coverage wave 5 - DVD low-level IOS submission and recovery

## How to read this file

This file records exactly what wave 5 added to `src/runtime/dvd.rs`, not what
the address band contains. Read it in five steps:

1. **Scope** gives the band, the selection rule and the byte count.
2. **Summary** gives the function/byte counts and status split.
3. **The function table** lists every translated body in address order with
   its role, status, confidence and exact Ghidra evidence file.
4. **What this pass adds** explains the four control-flow clusters in prose.
5. **Unknowns / next targets / gates** record what is still open.

Companion ledger: `ledger/agents/wave5-dvd.csv` (one row per function, same
ordering as the table below).

Status vocabulary (`docs/RUST_STYLE.md`): `translated` means complete recovered
control flow and every call accounted for; `partial` means entry/exit and major
effects only; `stub` means address/role only. **Confidence** measures evidence
strength for the *semantics*, not code quality.

Every Rust function keeps `# Original` with the Ghidra name, address, size and
evidence path. Descriptive names are used only where an embedded string, a
caller or the body shape proves the role; otherwise the function keeps its
`fn_80xxxxxx` name. `// observed:`, `// inferred:` and `// unknown:` markers in
the Rust carry the same information at the call site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`.

| Item | Count |
|---|---:|
| Functions selected | 25 |
| Status `translated` | 25 |
| Status `partial` | 0 |
| Bytes represented | 10,544 |
| Address span | `0x801CBA00..0x801D2D90` |

The selection rule was: an entry in `[0x801C0000, 0x801D4000)` that is **not**
already present in `ledger/functions.csv` or `src/runtime/dvd.rs`, with a full
Ghidra body, and that belongs to the DVD / FST / IOS cluster rather than the
graphics cluster below `0x801CA640`. Selection prioritised completion callbacks,
FST/partition open, error recovery and IOS command builders. Functions below
16 bytes were not selected; the queue helpers at 88-160 bytes were included
because they are the pop/has-work primitives of the same state machine.

The graphics block `0x801C0000..0x801CA640` was deliberately excluded:
`0x801CA640` itself writes the GX FIFO through the pointer at `r13-0x4DEC`
(initialised to `0xCC000000` by `0x801C1DB0`), so it is not an FST accessor.

## `runtime::dvd` - `src/runtime/dvd.rs`

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | role (plain English) | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x801CBA00` | 480 | `fn_801cba00` | completion: record status, then retry, stop the motor or restart the transfer | translated | high | `801cba00_FUN_801cba00.c` |
| `0x801CBBE0` | 212 | `fn_801cbbe0` | completion: record status and select the raw-error or default callback | translated | high | `801cbbe0_FUN_801cbbe0.c` |
| `0x801CBCC0` | 576 | `fn_801cbcc0` | completion: finish a waiting descriptor or arm the delayed retry | translated | high | `801cbcc0_FUN_801cbcc0.c` |
| `0x801CBF00` | 824 | `dvd_check_partition_header_801cbf00` | compare the staged partition header, then read `0x8D` or stop the motor | translated | high | `801cbf00_FUN_801cbf00.c` |
| `0x801CC3E0` | 1,144 | `dvd_partition_open_801cc3e0` | find a partition entry and open it with `0x8B` or `0x94` | translated | medium | `801cc3e0_FUN_801cc3e0.c` |
| `0x801CCB60` | 456 | `fn_801ccb60` | completion: finish a waiting descriptor or advance to state 6 | translated | high | `801ccb60_FUN_801ccb60.c` |
| `0x801CCD30` | 404 | `dvd_fst_read_completion_801ccd30` | FST-read completion: start the disc-change FST re-read (`0x71`) | translated | high | `801ccd30_FUN_801ccd30.c` |
| `0x801CCFA0` | 592 | `dvd_read_id_step_801ccfa0` | boot-state decision: `0xDD` max rotation or `0x70` read disk ID | translated | medium | `801ccfa0_FUN_801ccfa0.c` |
| `0x801CD1F0` | 228 | `dvd_retry_timer_801cd1f0` | timer: save time, clear latches and re-arm the next alarm | translated | high | `801cd1f0_FUN_801cd1f0.c` |
| `0x801CD2E0` | 200 | `dvd_max_rotation_801cd2e0` | clear cover, record `0x25` and submit `0xDD` | translated | high | `801cd2e0_FUN_801cd2e0.c` |
| `0x801CD3B0` | 576 | `fn_801cd3b0` | max-rotation completion: start read-id or advance the partition state | translated | high | `801cd3b0_FUN_801cd3b0.c` |
| `0x801CD5F0` | 376 | `fn_801cd5f0` | continue read-id: complete state `0x28` or submit `0x70` | translated | high | `801cd5f0_FUN_801cd5f0.c` |
| `0x801CD770` | 216 | `fn_801cd770` | read-id completion: retry or check the partition header | translated | high | `801cd770_FUN_801cd770.c` |
| `0x801CD850` | 392 | `dvd_cover_alarm_801cd850` | cover/alarm handler: abort, stop or retry the current command | translated | medium | `801cd850_FUN_801cd850.c` |
| `0x801CFB20` | 160 | `dvd_queue_pop_801cfb20` | pop the first node from one of four transfer-queue rings | translated | high | `801cfb20_FUN_801cfb20.c` |
| `0x801CFBC0` | 88 | `dvd_queue_has_work_801cfbc0` | test whether any transfer-queue ring is non-empty | translated | high | `801cfbc0_FUN_801cfbc0.c` |
| `0x801CFC90` | 96 | `dvd_queue_unlink_801cfc90` | unlink one node from its transfer queue | translated | high | `801cfc90_FUN_801cfc90.c` |
| `0x801D11E0` | 80 | `dvd_low_finish_801d11e0` | close the IOS DVD device and clear the ready latch | translated | high | `801d11e0_FUN_801d11e0.c` |
| `0x801D1230` | 612 | `dvd_low_init_801d1230` | init IPC, allocate buffers, seed four contexts and open `/dev/di` | translated | medium | `801d1230_FUN_801d1230.c` |
| `0x801D14A0` | 396 | `dvd_low_read_disk_id_801d14a0` | submit the `0x70` read-disk-id command | translated | high | `801d14a0_FUN_801d14a0.c` |
| `0x801D1630` | 620 | `dvd_low_open_partition_801d1630` | submit the `0x8B` open-partition command | translated | high | `801d1630_FUN_801d1630.c` |
| `0x801D18A0` | 612 | `dvd_low_open_partition_tmd_801d18a0` | submit the `0x94` open-partition-with-TMD command | translated | high | `801d18a0_FUN_801d18a0.c` |
| `0x801D28E0` | 408 | `dvd_low_audio_config_801d28e0` | submit the `0xE4` audio-buffer-configuration command | translated | high | `801d28e0_FUN_801d28e0.c` |
| `0x801D2A80` | 408 | `dvd_low_report_key_801d2a80` | submit the `0xA4` report-key command | translated | high | `801d2a80_FUN_801d2a80.c` |
| `0x801D2D90` | 388 | `dvd_low_set_max_rotation_801d2d90` | submit the `0xDD` set-max-rotation command | translated | high | `801d2d90_FUN_801d2d90.c` |

## What this pass adds

### 1. Shared completion bookkeeping

`dvd_record_completion` reproduces the prologue common to the new completion
callbacks: the status goes into the event-ring header for the current index
(`0x80395A54 + index*0x14`) or into the `+0x0C` field of the previous record
(`0x80395A68 + (index-1)*0x14`), and `0x80395AB8` keeps the previous value.
`dvd_complete_waiting_descriptor` centralises the "state 10, callback `-3`"
tail used by `0x801CBCC0`, `0x801CCB60`, `0x801CC3E0`, `0x801CD3B0` and
`0x801CD5F0`.

### 2. FST / partition path

`0x801CC3E0` scans the partition table at `0x80395800`, using the game words at
`0x80003194`/`0x80003198` as the selection key, then records event `0x22` and
opens the partition. It chooses `0x801D18A0` (`0x94`, TMD + eTicketView) when
the drive probe byte `0x80003187` is `0x80`, otherwise `0x801D1630` (`0x8B`).
Both builders use the shared vector table at `0x80395CA0`. The FST side is
closed by `0x801CCD30`, which installs `0x801CAE90` as the disc-change callback
and re-submits the `0x71` FST read through `0x801D2F20`.

### 3. Error recovery and queue primitives

`0x801CD2E0`/`0x801CD3B0`/`0x801CD5F0` form the no-disc and max-rotation
recovery chain; `0x801CD770` hands a successful `0x70` read to the header check
at `0x801CBF00`; `0x801CD850` reacts to cover-status bits `0`/`2` and either
aborts to state `0x0C`, records reason `7`, or re-enters the state machine.
`0x801CFB20`/`0x801CFBC0`/`0x801CFC90` are the four-ring pop, has-work and
unlink primitives used by that machine.

### 4. IOS device and command submission

`0x801D1230` initialises `IPCCltInit`, allocates a 0x80-byte command area and a
0x20-byte path buffer from the IOS heap, seeds the four 0x20-byte contexts at
`0x80395BE0` with the `0xFEEBDAED` magic, and opens `/dev/di` through
`0x80224C70`. `0x801D11E0` is the matching close. The six new builders submit
`0x70`, `0x8B`, `0x94`, `0xE4`, `0xA4` and `0xDD`; every one claims a context,
advances the 4-slot command ring at `r13-0x4C7C`, and clears the context state
byte on an IOS error after reporting the embedded string.

## Unknowns and partials

1. **Partition-key words.** `0x80003194`/`0x80003198` are treated as the two
   words of the selected boot/partition entry. The names are descriptive; the
   official meaning is not established by the export.
2. **Boot-state word.** `0x800030E6` is read as a 16-bit value; `0x8003`
   selects the no-disc max-rotation path in `0x801CCFA0`. The related code in
   `0x80004050` also masks it with `0x8000`/`0x7FFF`, confirming a halfword
   state, but the individual state numbers are not named.
3. **Header comparison.** `0x801D0710` is described as an 8-byte disc-header
   comparator from its body (bytes `0..3`, `4..5` and per-disc bytes `6`/`7`);
   `0x802DF098` is `memcmp`. The meaning of the compared fields is not proven.
4. **Cover bits.** Bits `0` and `2` of `0x80395C64` and the state codes
   `5`/`0x0C`/`3` are observed values, not official names.
5. **Heap alignment mask.** Ghidra prints `(value + 0x1F) & 0x1F` in
   `0x801D1230`. The surrounding pointer arithmetic and the `+0x80` / `+0x20`
   size checks only make sense with the 32-byte alignment mask, so the Rust
   uses `& !0x1F` and records the discrepancy in the function doc.
6. **`0x801D18A0` argument order.** The order
   `(offset, eTicketView, tmdSize, tmd, certSize, cert, callback)` is recovered
   from the caller at `0x801CC3E0`, the fixed `0xD8` eTicketView length and the
   embedded `tmd`/`eTicketView` diagnostics. All recovered callers pass a null
   certificate, so that pair is not exercised.
7. **IOS command names.** The bytes `0x70`, `0x8B`, `0x94`, `0xA4`, `0xDD` and
   `0xE4` come from the bodies; their public SDK names are taken only from the
   embedded diagnostic strings.

## Next targets

Closest remaining bodies in the same band:

| Address | Size | Why next |
|---|---:|---|
| `0x801CDD30` | 1,456 | state-step dispatcher that calls every new builder |
| `0x801CE2E0` | 2,528 | largest remaining command-completion routine |
| `0x801CF4B0` | 204 | queue/state helper adjacent to the pop primitives |
| `0x801D1B10` | 528 | `0x92` no-disc buffer-size builder |
| `0x801D1D20` | 664 | `0x90` no-disc partition-parameter builder |
| `0x801D2C20` | 364 | `0xDA` read-disc-BCA builder |
| `0x801D3280` | 364 | `0x7A` prepare-cover-register builder |
| `0x801D33F0` / `0x801D3560` | 364 / 364 | `0x95`/`0x96` prepare-status/control builders |

## Gates

- `node tools/doc-lint.mjs src` -> `ok: true`, 0 violations (68 files).
- Full `cargo check` on the crate: `Finished dev profile`, no diagnostics in
  `src/runtime/dvd.rs`.
- Isolated crate containing only `src/platform.rs`, `src/runtime/os.rs` and
  `src/runtime/dvd.rs`: `cargo check` finishes successfully with no warnings.
- No files outside this lane were edited.
