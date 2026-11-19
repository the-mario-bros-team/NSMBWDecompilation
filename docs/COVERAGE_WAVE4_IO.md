# Coverage wave 4 — DVD/FST callbacks and KPAD report path

## How to read this file

This file records exactly what wave 4 added to the two reader-facing modules,
not what the address bands contain. Read it in five steps:

1. **Scope** states the bands and the selection rule.
2. **Summary** gives the function/byte counts and the status split.
3. **The two function tables** list every translated body in address order with
   its role, status, confidence and exact Ghidra evidence file.
4. **What the pass adds** explains the two control-flow clusters in prose.
5. **Unknowns / next targets / gates** record what is still open and the result
   of the repository checks.

Companion ledger: `ledger/agents/wave4-io.csv` (one row per function, same
ordering as the tables below).

Vocabulary (`docs/RUST_STYLE.md`): `translated` = complete recovered control
flow and every call accounted for; `partial` = entry/exit and major effects
only; `stub` = address/role only. **Confidence** measures evidence strength for
the *semantics*, not code quality.

Every Rust function keeps `# Original` with the Ghidra name, address, size and
evidence path. No symbol names were invented: descriptive names are used only
where a string or the body shape proves the role; otherwise the function keeps
its `fn_80xxxxxx` name. `// observed:`, `// inferred:` and `// unknown:` markers
carry the same information at the call site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`.

| Item | Count |
|---|---:|
| DVD/FST functions selected | 15 |
| Input (WPAD/KPAD) functions selected | 10 |
| **Total selected** | **25** |
| Already translated before this pass | 0 |
| Status `translated` | 24 |
| Status `partial` | 1 |

The selection rule was: an entry in the two bands that was **not** in
`ledger/functions.csv` and **not** already present in
`src/runtime/dvd.rs` / `src/runtime/input.rs`, with a complete Ghidra body and a
recoverable role. Functions below 16 bytes were not selected.

| Module | Functions | Bytes | Address span |
|---|---:|---:|---|
| `runtime::dvd` | 15 | 5,880 | `0x801CC240..0x801D30D0` |
| `runtime::input` | 10 | 2,996 | `0x801E0700..0x801E1DD0` |
| **total** | **25** | **8,876** | `0x801CC240..0x801E1DD0` |

## `runtime::dvd` — `src/runtime/dvd.rs`

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| address | size | Rust function | role (plain English) | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x801CC240` | 416 | `fn_801cc240` | read-id completion: record status, then retry or start the `0x8D` read | translated | medium | `801cc240_FUN_801cc240.c` |
| `0x801CC860` | 392 | `fn_801cc860` | FST-read completion: record status, then retry or start the `0x71` read | translated | medium | `801cc860_FUN_801cc860.c` |
| `0x801CC9F0` | 356 | `fn_801cc9f0` | generic completion: retry or finish the waiting descriptor | translated | medium | `801cc9f0_FUN_801cc9f0.c` |
| `0x801CCED0` | 196 | `fn_801cced0` | dispatch the current command kind to retry, finish or abort | translated | medium | `801cced0_FUN_801cced0.c` |
| `0x801CDA10` | 796 | `dvd_state_dispatch_801cda10` | pop the queue and drive the next state step | translated | medium | `801cda10_FUN_801cda10.c` |
| `0x801CF060` | 868 | `dvd_wait_command_801cf060` | wait for/advance one descriptor command | translated | medium | `801cf060_FUN_801cf060.c` |
| `0x801D0C30` | 180 | `dvd_classify_low_error_801d0c30` | classify a raw drive error and track repeats | translated | medium | `801d0c30_FUN_801d0c30.c` |
| `0x801D1060` | 184 | `dvd_low_transaction_callback_801d1060` | shared IOS transaction completion callback | translated | high | `801d1060_FUN_801d1060.c` |
| `0x801D1120` | 188 | `dvd_low_cover_callback_801d1120` | cover-register command completion callback | translated | medium | `801d1120_FUN_801d1120.c` |
| `0x801D2130` | 408 | `dvd_low_unencrypted_read_801d2130` | submit the `0x8D` unencrypted-read command | translated | high | `801d2130_FUN_801d2130.c` |
| `0x801D22D0` | 408 | `dvd_low_stop_motor_801d22d0` | submit the `0xE3` stop-motor command | translated | high | `801d22d0_FUN_801d22d0.c` |
| `0x801D2470` | 364 | `dvd_low_inquiry_801d2470` | submit the `0x12` drive-inquiry command | translated | high | `801d2470_FUN_801d2470.c` |
| `0x801D25E0` | 360 | `dvd_low_request_error_801d25e0` | submit the `0xE0` request-error command | translated | high | `801d25e0_FUN_801d25e0.c` |
| `0x801D2760` | 380 | `dvd_low_reset_801d2760` | submit the `0x8A` drive-reset command | translated | high | `801d2760_FUN_801d2760.c` |
| `0x801D30D0` | 384 | `dvd_low_seek_801d30d0` | submit the `0xAB` seek command | translated | high | `801d30d0_FUN_801d30d0.c` |

## `runtime::input` — `src/runtime/input.rs`

| address | size | Rust function | role (plain English) | status | confidence | evidence |
|---|---:|---|---|---|---|---|
| `0x801E0700` | 288 | `fn_801e0700` | assign a Bluetooth device to a KPAD channel | translated | medium | `801e0700_FUN_801e0700.c` |
| `0x801E11A0` | 184 | `fn_801e11a0` | offer a report id to a channel and store it | partial | low | `801e11a0_FUN_801e11a0.c` |
| `0x801E12C0` | 544 | `fn_801e12c0` | queue the `0x15` extension-report request | translated | medium | `801e12c0_FUN_801e12c0.c` |
| `0x801E1630` | 488 | `fn_801e1630` | queue a `0x11` KPAD event block | translated | medium | `801e1630_FUN_801e1630.c` |
| `0x801E1820` | 172 | `fn_801e1820` | gate and start sampling with a callback | translated | medium | `801e1820_FUN_801e1820.c` |
| `0x801E18D0` | 228 | `fn_801e18d0` | copy the selected report sub-block to a caller buffer | translated | high | `801e18d0_FUN_801e18d0.c` |
| `0x801E19C0` | 428 | `fn_801e19c0` | fill a report ring with zeroed slots and markers | translated | high | `801e19c0_FUN_801e19c0.c` |
| `0x801E1B70` | 276 | `fn_801e1b70` | clear report status bits for the selected sub-block | translated | medium | `801e1b70_FUN_801e1b70.c` |
| `0x801E1C90` | 320 | `fn_801e1c90` | copy the report into the ring and call the sampling callback | translated | high | `801e1c90_FUN_801e1c90.c` |
| `0x801E1DD0` | 68 | `fn_801e1dd0` | read the per-channel word at `+0x83C` | translated | low | `801e1dd0_FUN_801e1dd0.c` |

## What the DVD pass adds

The pass closes the callback side of the low-level DVD layer:

1. **Three completion callbacks.** `0x801CC240`, `0x801CC860` and `0x801CC9F0`
   share the status bookkeeping (`0x80395AB8`, plus the active ring record or
   its header) and the `0x10`/`0x20`/even/odd dispatch. Their odd branches are
   the three distinct continuations: unencrypted read-id (`0x8D` into
   `0x80395800` at offset `0x10008`), low-level FST read (`0x71` into
   `0x80395700` at offset `0x108`), and descriptor completion.
2. **State dispatch.** `0x801CDA10` pops one transfer through `0x801CFB20`,
   publishes its kind in `r13-0x4CDC`, and maps the pending flag and reason
   word onto the state codes (`0x0B`/`4`/`5`/`3`) and the delayed-retry alarm.
   `0x801CCED0` is the smaller kind switch used by the same machine.
3. **Descriptor wait.** `0x801CF060` is the function `0x801CF3D0` (translated in
   wave 3) calls before sleeping: it either waits, cancels, forwards the
   callback or restarts the state machine.
4. **Error classification.** `0x801D0C30` is the low-level twin of
   `dvd_classify_error_801cb320`, using `r13-0x4CA0`/`-0x4CA4` rather than
   `-0x4CE8`/`-0x4D48`. It confirms that `0x31100` is the special retryable
   code.
5. **IOS command builders.** `0x801D2130`, `0x801D22D0`, `0x801D2470`,
   `0x801D25E0`, `0x801D2760` and `0x801D30D0` all share the four 0x20-byte
   contexts at `0x80395BE0` and the ring of 0x20-byte command slots at
   `r13-0x4C7C`. Each has a distinct command byte (`0x8D`, `0xE3`, `0x12`,
   `0xE0`, `0x8A`, `0xAB`) and its own diagnostic string, so the names are
   proven by evidence rather than inferred from the shape alone.
6. **Two completion callbacks for those builders.** `0x801D1060` is the common
   `IOS_IoctlAsync` callback; `0x801D1120` additionally snapshots the cover
   register block at `0x80395C80` into `0x80395C64`.

The four-context layout already documented in wave 3 is unchanged:
`+0` callback, `+4` queued flag, `+8` state byte, `+0xC` `0xFEEBDAED` magic.
This pass reuses `dvd_low_claim_context`, extended with a `report_index` flag so
`0x801D2F20`/`0x801D2130` (which pass the index with the diagnostic) and the
other builders (which do not) match their Ghidra calls exactly.

## What the input pass adds

The pass closes the report-producing side of KPAD:

1. **Device assignment.** `0x801E0700` reads the Bluetooth device id at
   `+0x56`, forwards the key through `0x801F6160`, and pins
   `"Nintendo RVL WBC"` (balance board) to channel 3. Other devices are matched
   against `0x801F6350` and the free bytes at `r13-0x4BD0`.
2. **Report-ring producers.** `0x801E12C0` and `0x801E1630` build 0x30-byte
   blocks (kinds `0x15` and `0x11`) and push them into the ring described by
   `+0x160`/`+0x161`/`+0x164`/`+0x168`. The shared push helper reproduces the
   pending-count arithmetic, the full test (`count == capacity - 1`), the
   slot clear/copy and the head wrap.
3. **Double-buffered report readers.** `0x801E18D0`, `0x801E19C0`,
   `0x801E1B70` and `0x801E1C90` all select `control + 0xA0` or
   `control + 0x100` from `CLZ(byte +0x8C8) >> 5`, then use the report length
   table keyed by `+0x8B8`. `0x801E1C90` is the sampling dispatch: it advances
   `+0x8B0`, copies the current report into `+0x8AC`, calls the `+0x8A8`
   callback and bumps the 16-bit counter at `+0x984`.
4. **Sampling gate.** `0x801E1820` combines the extension-type query
   `0x801DD6D0(mode)` with the global gate `r13-0x4BC8`, calls the callback
   with `2` when refused, and otherwise forwards it to `0x801DCC70`.
5. **Channel accessor.** `0x801E1DD0` exposes `+0x83C` under the lock.

## Unknowns and partials

1. **`fn_801e11a0` is `partial`.** The Ghidra body starts with a register-save
   artifact and reads its second argument from `r4`, but the only recovered
   caller (`0x801EE230`) passes one register. The Rust signature exposes
   `(channel, value)` and documents the unresolved argument. The rest of the
   body is exact.
2. **Unassigned stack bytes in `0x801E12C0` / `0x801E1630`.** Both functions
   copy a 0x30-byte stack block whose fields outside the explicit constants,
   parameters and callback are never assigned by the Ghidra body. The Rust
   builds a zeroed array and marks the gap `// unknown:`; only the assigned
   bytes are claimed.
3. **Event/command kinds.** `0x11`, `0x15`, `0x21`, `0x27` and the
   `0x8D`/`0xE3`/`0x12`/`0xE0`/`0x8A`/`0xAB` command bytes are observed
   values. Their official meanings are named only where an embedded string
   proves them.
4. **Control-block field names.** `+0x83C`, `+0x8AC`..`+0x8B8`, `+0x8C8`,
   `+0x8D8`, `+0x98C`/`+0x988` remain numeric. The wave does not invent
   `KPADStatus`-style substructure names.
5. **Diagnostic strings.** The new message constants are taken from the
   addresses embedded in the Ghidra symbol names
   (`0x803424D0`, `0x80342B78`, `0x80342BB8`, `0x80342C38`, `0x80342C70`,
   `0x80342D30`, `0x803431D0`); only the prefix of each string is quoted in the
   comment.
6. **`0x801D1120` role.** The callback is proven to be the one passed by the
   cover-register builder (`0x7A`), but the meaning of the copied
   `0x80395C64` word is not established.

## Next targets

Closest remaining bodies in the same bands, all still untranslated:

### DVD / FST

| Address | Size | Why next |
|---|---:|---|
| `0x801CDD30` | 1,456 | the state-step body `0x801CDA10` calls |
| `0x801CE2E0` | 2,528 | largest remaining queue/state routine |
| `0x801CFB20` / `0x801CFBC0` | 160 / 88 | queue pop and has-work helpers, now named by this pass |
| `0x801CFC90` | 96 | descriptor cancel used by `dvd_wait_command_801cf060` |
| `0x801D33F0` / `0x801D3560` | 364 / 364 | `0x95`/`0x96` prepare-status/control builders |
| `0x801D3880`+ | 320+ | remaining low-level builders and register helpers |

### WPAD / KPAD

| Address | Size | Why next |
|---|---:|---|
| `0x801DFE00` | 2,292 | main WPAD/KPAD command dispatcher |
| `0x801E0820` | 1,204 | data/connection handler calling the new report helpers |
| `0x801E1E20` | 5,628 | largest remaining WPAD/KPAD routine |
| `0x801E4C50` | 616 | ring push helper called by `fn_801e11a0` |

## Gates

- `node tools/doc-lint.mjs src` → `ok: true`, 0 violations (68 files).
- `cargo check` on the full crate is blocked by an unrelated other-lane error
  in `src/game/worldmap.rs` (`wm_effect_slot_index` missing). `src/runtime/dvd.rs`
  and `src/runtime/input.rs` were checked together in an isolated crate with
  `platform.rs` and `runtime/os.rs`: exit 0, no diagnostics.
- No files outside this lane were edited.
