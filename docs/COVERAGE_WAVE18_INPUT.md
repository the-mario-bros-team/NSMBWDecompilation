# Coverage — wave 18 input (`0x801D4000`–`0x80200000`, `runtime::input`)

## How to read this file

This page records exactly what the wave-18 input lane added to
[`src/runtime/input.rs`](../src/runtime/input.rs), not what the whole address
band contains. Read it in five steps:

1. **Scope** gives the raw inventory count, the eligibility rule and the
   before/after totals for the band.
2. The **function table** lists the 25 new bodies in address order, with the
   plain-English role, the exact Ghidra export and the confidence.
3. **Clusters** explains the five groups — the report-queue consumer, the
   diagnostic stubs and link getters, the GKI buffer pools, the queue/list
   operations and the critical-section stack — and how they connect to the
   WPAD/KPAD code already in the module.
4. **Corrections and caveats** records the decompiler artifacts that needed
   interpretation and the two intentional no-ops.
5. **Ledger** points at the machine-readable rows.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the full control
flow and every call are accounted for; `partial` means entry/exit and major
effects only; `stub` means address/role only. Confidence is evidence strength
(high / medium / low), not code quality.

Every body cites `FUN_<address>` and
`work/ghidra/main-gb/export/<addr>_<name>.c`. No official symbol names were
invented: descriptive names are used only where a string or the body shape
proves the role; everything else keeps `fn_80xxxxxx`. `// observed:`,
`// inferred:` and `// unknown:` markers inside the Rust carry the same
information at the call site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`; translated baseline:
`ledger/functions.csv`.

| Item | Count |
|---|---:|
| Entry addresses in `[0x801D4000, 0x80200000)` | 580 |
| Excluded, size `< 16` bytes | 45 |
| Eligible, size `>= 16` bytes | 535 |
| Already translated in this band before wave 18 | 97 |
| **Selected and translated by this wave** | **25** |
| Eligible untranslated after this wave | 413 |
| Remaining small (`< 16`) entries | 45 |

The 25 new bodies represent **5,444 bytes**, all of them eligible entries. The
band is still mostly untranslated (413 eligible entries): this wave was scoped
to the transport between the existing WPAD report pipeline and the Bluetooth
HID-host task, not to a full sweep.

Selection rule: the address must be an entry in `functions.tsv`, must not have
a row in `ledger/functions.csv`, and the Ghidra body must have complete control
flow. Large paired-single DSP-style bodies, the giant WPAD/KPAD state machines
and the WUD/USB cluster were deliberately deferred (see the end of this file).

## Function table

| Address | Rust name | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x801DDD20` | `fn_801ddd20` | Consume one completed WPAD report block: update the channel control block, copy the report into a GKI buffer, post a `0x1708` HID-host message. | translated | medium | `801ddd20_FUN_801ddd20.c` |
| `0x801EA8F0` | `fn_801ea8f0` | Formatted-report sink; an empty no-op in this build. | translated | high | `801ea8f0_FUN_801ea8f0.c` |
| `0x801F3530` | `fn_801f3530` | Locked read of the Bluetooth link-state byte. | translated | high | `801f3530_FUN_801f3530.c` |
| `0x801F3570` | `fn_801f3570` | Locked difference of the two link counters, masked to a byte. | translated | high | `801f3570_FUN_801f3570.c` |
| `0x801F51A0` | `fn_801f51a0` | Link-idle predicate over four open-attempt state bytes. | translated | high | `801f51a0_FUN_801f51a0.c` |
| `0x801F6A70` | `fn_801f6a70` | Unconditional Bluetooth diagnostic print; an empty no-op in this build. | translated | high | `801f6a70_FUN_801f6a70.c` |
| `0x801F6AC0` | `fn_801f6ac0` | Build one pool's free list and publish its record, stride and arena bounds. | translated | high | `801f6ac0_FUN_801f6ac0.c` |
| `0x801F6CE0` | `fn_801f6ce0` | Reset the queue arrays and pool records, create the five pools, publish the size-ordered id table. | translated | high | `801f6ce0_FUN_801f6ce0.c` |
| `0x801F6F90` | `fn_801f6f90` | Clear a list object (`head`/`tail`/`count`). | translated | high | `801f6f90_FUN_801f6f90.c` |
| `0x801F6FB0` | `fn_801f6fb0` | `getbuf` by size: first fitting, non-excluded pool. | translated | high | `801f6fb0_FUN_801f6fb0.c` |
| `0x801F7150` | `fn_801f7150` | `getbuf` by pool id, falling back to the size search when exhausted. | translated | high | `801f7150_FUN_801f7150.c` |
| `0x801F7240` | `fn_801f7240` | `freebuf`: validate the header/guard and append the block to its pool's free list. | translated | high | `801f7240_FUN_801f7240.c` |
| `0x801F73B0` | `fn_801f73b0` | Payload size of a buffer's pool (`0` for a bad header). | translated | high | `801f73b0_FUN_801f73b0.c` |
| `0x801F7400` | `fn_801f7400` | Send: append to the `(owner, destination)` FIFO and post the destination bit. | translated | high | `801f7400_FUN_801f7400.c` |
| `0x801F75C0` | `fn_801f75c0` | Receive: pop the current owner's `destination` FIFO. | translated | high | `801f75c0_FUN_801f75c0.c` |
| `0x801F7670` | `fn_801f7670` | Tail-insert a buffer into a caller-owned list object. | translated | high | `801f7670_FUN_801f7670.c` |
| `0x801F77B0` | `fn_801f77b0` | Head-insert a buffer into a caller-owned list object. | translated | high | `801f77b0_FUN_801f77b0.c` |
| `0x801F78F0` | `fn_801f78f0` | Pop the head buffer of a caller-owned list object. | translated | high | `801f78f0_FUN_801f78f0.c` |
| `0x801F7990` | `fn_801f7990` | Unlink one specific buffer from a caller-owned list object. | translated | high | `801f7990_FUN_801f7990.c` |
| `0x801F84F0` | `fn_801f84f0` | GKI init: clear the data area, create the pools, set the flag, push the initial critical level. | translated | high | `801f84f0_FUN_801f84f0.c` |
| `0x801F8580` | `fn_801f8580` | GKI shutdown: clear the flag and drain the critical-section stack. | translated | high | `801f8580_FUN_801f8580.c` |
| `0x801F8670` | `fn_801f8670` | Set pending-event bits for one queue owner with the mask push/pop inlined. | translated | high | `801f8670_FUN_801f8670.c` |
| `0x801F8730` | `fn_801f8730` | Pop the critical-section stack and restore the saved interrupt mask. | translated | high | `801f8730_FUN_801f8730.c` |
| `0x801F8760` | `fn_801f8760` | Push the current interrupt mask onto the critical-section stack. | translated | high | `801f8760_FUN_801f8760.c` |
| `0x801F87C0` | `fn_801f87c0` | Allocate through the application `MEMalloc` callback (`0x801EFF00`). | translated | medium | `801f87c0_FUN_801f87c0.c` |

Totals for this wave: **25 translated, 0 partial, 0 stubs**; confidence split
23 high / 2 medium / 0 low. Evidence paths are relative to
`work/ghidra/main-gb/export/`; the full file name is the address (lowercase, no
`0x`) plus `_FUN_<address>.c`, as in the table.

## Clusters

### Report-queue consumer — `0x801DDD20`

This is the sink side of the WPAD command/report ring that wave 5's
`fn_801e4c50` and wave 14's builders produce. `0x801DEB70` calls it as
`fn_801ddd20(channel, &block)` once a 0x30-byte block has completed. The block
layout is the same one the builders use: the kind is the low byte of the first
big-endian word (block byte `+3`), the payload starts at `+4`, and the outgoing
length is the `u16` at `+0x1A`.

Under the interrupt lock the function:

* treats a negative handle byte at `+0x8C3` as an inactive channel and returns
  without changing anything or forwarding a report;
* writes `+0x8BC = 0xFFFFFFFE` as a "busy" marker, then restores the saved
  state for kinds `0x10`, `0x15` and `0x18`;
* kind `0x18` additionally decrements the retry byte `+0x8CF`;
* kind `0x17` publishes an extension-register read: `+0x974`/`+0x978` from
  block `+0x1C`/`+0x24`, the `u16` at `+0x980` from block `+0x20`, and clears
  `+0x97C`;
* kind `0x15` stores block `+0x28` at `+0x850` and sets the latch `+0x8C0`;
* every other kind (including `0x16`) sets bit 1 of the first payload byte.

It then stores block `+0x2C` at `+0x89C`, the kind at `+0x987`, a 64-bit
deadline at `+0x908` (high) / `+0x90C` (low), and clears `+0x910`. The deadline
is `get_time() + (bus clock >> 1 & 0x7FFFFFFE)`, the same tick-based timeout
`0x801DF570` uses.

Outside the lock it reports the handle/report id through `0x801EA8F0`,
allocates `(length + 0x12) & 0xFF` bytes with `0x801F6FB0`, fills the four-byte
message header (`+2` = `length + 1`, `+4` = `0x0A`), copies the payload to
`+0x13`, derives bit 0 of the copied byte from `(+0x8D4 & r13-0x4BC8)`, and
posts the message for the HID host with `0x801FF778` (kind `0x1708`).

### Diagnostic stubs and link getters — `0x801EA8F0`, `0x801F3530`–`0x801F51A0`

* `0x801EA8F0` is the formatted printer that every WPAD/KPAD diagnostic and
  the three `WBC*Dummy` placeholders call. The shipped body is a bare return,
  so the format string and arguments are ignored; the Rust documents this as
  intentional, not as a missing translation.
* `0x801F3530` returns the sign-extended link-state byte `0x803A3828` (the
  value wave 5's `fn_801ddb50` compares with `0`–`4`).
* `0x801F3570` returns `(0x803A3866 - 0x803A3864) & 0xFF`, the same
  high/low counter pair the HID-host event `0x0F` updates.
* `0x801F51A0` is the "link is idle" predicate:
  `0x803A312C == 0 && 0x803A312D == 0 && 0x803A312F == 4 &&
  0x803A3130 == 6` returns `0`, anything else `1`.
* `0x801F6A70` is the unconditional Bluetooth trace call (`bta_hh_co_open`,
  `WUDSetSyncDeviceCallback`, …). It is also a bare return in this image and is
  already referenced by address as `BTA_TRACE_LOG` from the HID-host layer.

All three getters take and release the interrupt lock through
`0x801B1140`/`0x801B1180`, exactly as Ghidra shows.

### GKI buffer pools — `0x801F6AC0`–`0x801F73B0`

The Wii Bluetooth stack (GKI) owns nine pool records of 0x10 bytes starting at
`0x803CDCA8`, each `{head u32, tail u32, size u16, count u16, used u16,
high u16}`. Every pool block is an 8-byte header (`{next, pool id, owner/tag,
state}`), the payload, and a `0xDDBADDBA` guard word directly after the
payload, so the stride is `align4(size) + 12`. The caller's pointer is the
payload start (`block + 8`); the header is at `-8` and the guard at
`size + 8`.

* `0x801F6AC0` builds one pool's free list. It publishes the arena base/end at
  `0x803CDD38`/`0x803CDD5C`, the stride at `0x803CDD80`, the six record
  fields, then chains every block, tagging the header with the pool id and the
  free tag `0xF0` and writing the guard word.
* `0x801F6CE0` clears the two 32-entry queue arrays (`0x803CDBA8`,
  `0x803CDC28`) and all nine records, sets the exclusion mask
  `0x803CDD98 = 0xFFF0`, and creates the fixed pools:

  | Pool | Payload | Count | Arena |
  |---:|---:|---:|---|
  | 0 | `0x40` | `0x30` | `0x803A531C` |
  | 1 | `0x80` | `0x1A` | `0x803A615C` |
  | 2 | `0x294` | `0x2D` | `0x803A6F94` |
  | 3 | `0x708` | `0x1E` | `0x803AE5B4` |
  | 4 | `0x2000` | `9` | `0x803BBA0C` |

  It then publishes the size-ordered id list `0..4` at `0x803CDD9A` with count
  `5` at `0x803CDDA3`.
* `0x801F6FB0` is `getbuf(size)`: `0` is rejected (`0xFFF6`), a size larger
  than every pool is rejected (`0xFFF7`), and the search walks the ordered id
  table, skipping pools whose bit is set in `0x803CDD98`. A hit pops the free
  list head, bumps `used`/`high`, marks the header `state = 1` with the task id
  from `0x801F8720` and returns the payload.
* `0x801F7150` is `getbuf(pool id)`: anything `>= 9` returns `0`; on exhaustion
  it falls back to `0x801F6FB0` with that pool's size, so a large request can
  still be served by a bigger pool.
* `0x801F7240` is `freebuf`: it rejects misaligned/corrupt buffers
  (`0xFFFF`), still-linked buffers (`0xFFFC`) and bad pool ids (`0xFFFD`), then
  appends the block to the pool free list, sets `state = 0`, restores the
  `0xF0` tag and decrements `used`.
* `0x801F73B0` returns the pool size for a buffer pointer; the same computation
  is inlined at four call sites and factored into the `gki_buf_size` helper.

### Owner/destination queues and list objects — `0x801F7400`–`0x801F7990`

There are two independent list conventions, both using the same 8-byte block
header:

* **Owner/destination FIFOs.** `0x801F7400` validates owner `< 8`,
  destination `< 4` and the per-owner registration byte `0x803CDAC8`, then
  appends the block to `0x803CDBA8 + owner * 0x10 + destination * 4` under the
  lock and posts `1 << destination` through `0x801F8670`. `0x801F75C0` pops the
  same slot for the current owner (`0x801F8720`, constant `2` in this build).
  Bad routes free the buffer (`0xFFFB`), corrupt headers are reported
  (`0xFFFF`) and already-linked buffers are rejected (`0xFFFA`).
* **Caller-owned list objects.** `0x801F6F90` clears `{head, tail, count}`;
  `0x801F7670` tail-inserts, `0x801F77B0` head-inserts, `0x801F78F0` pops the
  head and `0x801F7990` unlinks one specific buffer (repairing the tail when
  needed). All four run under the lock, keep the count in step, and use state
  `2` while linked and `1` after removal. The three diagnostics are
  `0xFFF9`/`0xFFFF` for enqueue and the shared free diagnostics.

### Critical sections and bring-up — `0x801F84F0`–`0x801F8730`

`0x801F8760`/`0x801F8730` are a nesting stack, not a simple lock: the depth
byte is `0x803A52C8` and the saved masks live at `0x803A52CC + depth * 4`.
`0x801F8760` pushes the mask returned by `0x801B1140`; `0x801F8730` pops it and
restores through `0x801B1180`. Every queue operation brackets its list update
with the pair, and `0x801F8670` inlines both halves around its event-word OR.

`0x801F84F0` is the subsystem init: `memset(0x803A52C8, 0, 0x28AE0)` clears the
GKI data area (including the five arenas and the pool records), `0x801F6CE0`
creates the pools, `0x801F7F60` clears the per-owner statistics, the init flag
`0x803CDACA` becomes `1`, `0x803CDAB0`/`0x803CDB10`/`0x803CDAD4` are cleared,
and the level from `0x801B1160` becomes the first stack entry. `0x801F8580`
reverses it: it pushes one more mask, clears the init flag, pops that mask,
then drains the remaining depth so the last pop re-enables interrupts.

## Corrections and caveats

1. **Artifact-based signatures.** `0x802DCD50`, `0x802DCD68` and the other
   `0x802DCD*` bodies are Ghidra's prologue/epilogue artifacts: they store or
   return saved register pairs. `fn_801f6ac0(pool, block_size, count, arena)`
   comes from the `0x802DCD50` return (`r3` in the high half, `r4` in the low
   half), and `fn_801ddd20(channel, block)` from `0x802DCD68`. The Rust header
   cites those artifacts instead of inventing a prototype.
2. **Block kind is byte `+3`.** The report consumer tests `word & 0xFF` on the
   first big-endian word of the block, so the kind is block byte `+3`. This is
   the same placement wave 14 applied to the queue builders; the consumer and
   the builders therefore agree on the layout.
3. **`0x803CDAD0` event stride (inferred).** Ghidra prints
   `&DAT_803cdad0 + owner * 2` on a `u16` datum, but `0x801F84F0` clears a
   separate word at `0x803CDAD4` for owner 1. The Rust uses one 32-bit word per
   owner (owner stride 4) and only touches the low 16 bits as the mask; the
   constant carries an `// inferred:` comment.
4. **`0x801F6AC0` unroll collapsed.** The original writes eight blocks per
   loop iteration and finishes with a remainder loop. The Rust uses one loop
   over `count`; the resulting free list, header tags and guard words are
   identical.
5. **`0x801F6AC0` block-size rounding.** The original computes
   `align4(block_size)` as `(size + 3) & ~3` for the stored pool size and the
   stride, and the guard word sits at `aligned + 8`. All five call sites pass
   sizes that are already 4-byte multiples, so rounding is invisible here, but
   the Rust still rounds up to match the original.
6. **`0x801F6AC0` zero-count guard.** The original closes the list
   unconditionally (`*last = 0; pool.tail = last`) even when `count == 0`,
   which would store to address 0. The Rust only closes when at least one block
   was written; every caller uses a non-zero count. The deviation is marked
   `// observed:` in the source.
7. **Deadline half order.** The 64-bit deadline is stored high word first at
   `+0x908` and low word at `+0x90C`, matching the existing
   `wpad_control_init_801df570`. An earlier draft had the halves reversed and
   was corrected before `cargo check`.
8. **Empty bodies are real.** `0x801EA8F0` and `0x801F6A70` are bare returns in
   the shipped image (80 bytes each, just the frame). They are translated as
   intentional no-ops and are still listed as `translated`, because their
   control flow (none) and their call sites are fully accounted for.
9. **Two medium-confidence rows.** `fn_801ddd20` is complete branch-for-branch,
   but the meaning of `+0x8BC`/`+0x8D4` and the id-specific fields is not named
   by any string. `fn_801f87c0` forwards to `0x801EFF00`, which Ghidra declares
   `void`; the returned pointer is recovered from the pass-through register, so
   the wrapper is marked medium.
10. **Deferred neighbours.** `0x801E1E20` (5,628 bytes) and `0x801E39C0`
    (4,224 bytes) are the large WPAD/KPAD state machines; `0x801E55C0`
    (2,792 bytes) is the paired-single `ldexpf` decoder; `0x801E9730`/
    `0x801E9E20` are the MotionPlus key derivations; `0x801DE010` and
    `0x801DEB70` are the WPAD queue workers. The `0x801EE*` WUD/USB cluster is
    out of this lane. All remain untranslated.
11. **No new staging.** All 25 bodies live in `src/runtime/input.rs`; no
    `src/subsystems/*` file, `lib.rs`, `mod.rs`, `Cargo.toml` or `platform.rs`
    was touched. `ledger/functions.csv` is deliberately unchanged by this lane.

## Ledger

Machine-readable rows: `ledger/agents/wave18-input.csv` (25 rows, module
`runtime::input`, same address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
