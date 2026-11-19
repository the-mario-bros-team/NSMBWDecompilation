# Coverage — `r801c0_80200` (`0x801C0000–0x80200000`)

## How to read this file

This file records what the range lane actually translated, not what the range
contains. Read it in four steps:

1. **Scope** gives the raw inventory count and the exclusion rule.
2. The **table** lists the 20 functions translated by this lane, in address
   order, with the plain-English role, the exact evidence file and the
   semantic module each row was promoted to.
3. **Clusters** explains the two SDK subsystems the lane picked and why.
4. **Next targets** lists concrete addresses for a following pass; each one is
   an entry point that is already indexed in `functions.tsv`.

Status vocabulary (`docs/READING_GUIDE.md`):
`translated` = full control flow and calls accounted for; `partial` = entry/exit
and major effects only; `stub` = address/role only.
Confidence is evidence strength (high / medium / low), not code quality.

Every translated function cites `FUN_<address>` and
`work/ghidra/main-gb/export/<addr>_<name>.c`. No symbol names were invented:
descriptive names are used only where a string, a caller, or the body shape
proves the role; everything else keeps `fn_80xxxxxx`. `// inferred:` and
`// unknown:` markers inside the Rust carry the same information at the call
site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`.

| Item | Count |
|---|---:|
| Entry addresses in `[0x801C0000, 0x80200000)` | 926 |
| Excluded, size `< 16` bytes | 68 |
| Eligible, size `>= 16` bytes | 858 |
| Translated by this lane | **20** |
| Promoted to semantic modules | **20** |
| Remaining in staging | **0** |

Bytes represented by the translated set: **4,788** of 245,252 eligible bytes
(about 2.0 %). The low byte percentage is expected: the lane was instructed to
prioritise the hinted DVD/FST and WPAD/KPAD clusters, not to sweep the range.

The 68 excluded entries are tiny thunks/accessors (4–15 bytes), e.g.
`0x801C33E0` (8), `0x801D2750` (12), `0x801ED3B0` (8). They are recorded as
index-only; a later pass can fold them into their callers instead of producing
standalone Rust functions. Note the boundary: `0x801CA640` is 16 bytes, so it is
eligible, but it was not one of the 20 selected.

## Translated functions

| Address | Rust name | Role (plain English) | Status | Confidence | Evidence | Promoted to |
|---|---|---|---|---|---|---|
| `0x801CA650` | `dvd_fs_init_801ca650` | Caches FST base, entry count and string-table base from `0x80000038`. | translated | high | `801ca650_FUN_801ca650.c` | promoted → `runtime::dvd` |
| `0x801CA680` | `dvd_convert_path_to_entrynum_801ca680` | Walks the FST to turn a disc path into an entry number (`-1` on miss). | translated | high | `801ca680_FUN_801ca680.c` | promoted → `runtime::dvd` |
| `0x801CA990` | `dvd_fast_open_801ca990` | Opens an FST entry by number and fills the file descriptor. | translated | high | `801ca990_FUN_801ca990.c` | promoted → `runtime::dvd` |
| `0x801CAA00` | `dvd_close_801caa00` | Cancels the descriptor's command and returns success. | translated | medium | `801caa00_FUN_801caa00.c` | promoted → `runtime::dvd` |
| `0x801CAA30` | `dvd_read_async_prio_801caa30` | Validates a read range and queues an asynchronous transfer. | translated | high | `801caa30_FUN_801caa30.c` | promoted → `runtime::dvd` |
| `0x801CAB20` | `dvd_read_prio_801cab20` | Queues a read and sleeps until the command block completes. | translated | high | `801cab20_FUN_801cab20.c` | promoted → `runtime::dvd` |
| `0x801CAC60` | `fn_801cac60` | Appends a five-word record to the DVD event ring. | translated | medium | `801cac60_FUN_801cac60.c` | promoted → `runtime::dvd` |
| `0x801CAE90` | `dvd_change_disk_801cae90` | Continues a disc change and re-reads the new disc's FST. | translated | medium | `801cae90_FUN_801cae90.c` | promoted → `runtime::dvd` |
| `0x801CB010` | `dvd_read_completion_801cb010` | Read/disc-change completion: record status, retry or reset. | translated | medium | `801cb010_FUN_801cb010.c` | promoted → `runtime::dvd` |
| `0x801CB2E0` | `fn_801cb2e0` | Registers `0x801CB1B0` with `0x801D2760`; clears two slots. | translated | low | `801cb2e0_FUN_801cb2e0.c` | promoted → `runtime::dvd` |
| `0x801DF570` | `wpad_control_init_801df570` | Resets one channel's 0x9C0-byte WPAD control block. | translated | medium | `801df570_FUN_801df570.c` | promoted → `runtime::input` |
| `0x801DF7F0` | `wpad_init_801df7f0` | WPAD driver bring-up for all four channels plus sampling alarm. | translated | high | `801df7f0_FUN_801df7f0.c` | promoted → `runtime::input` |
| `0x801DFB40` | `fn_801dfb40` | Locked read of channel field `+0x983`. | translated | low | `801dfb40_FUN_801dfb40.c` | promoted → `runtime::input` |
| `0x801DFB90` | `fn_801dfb90` | Locked read of global byte `r13-0x4BC4`. | translated | low | `801dfb90_FUN_801dfb90.c` | promoted → `runtime::input` |
| `0x801DFC40` | `fn_801dfc40` | Connection callback: mark disconnect or forward the link key. | translated | low | `801dfc40_FUN_801dfc40.c` | promoted → `runtime::input` |
| `0x801DFD10` | `fn_801dfd10` | Reset callback: clear report queue and forward link key. | translated | low | `801dfd10_FUN_801dfd10.c` | promoted → `runtime::input` |
| `0x801E0D30` | `fn_801e0d30` | Computes three signed accelerometer deltas for a channel. | translated | medium | `801e0d30_FUN_801e0d30.c` | promoted → `runtime::input` |
| `0x801E0F00` | `fn_801e0f00` | Writes the global mode byte `r13-0x4BC2`. | translated | low | `801e0f00_FUN_801e0f00.c` | promoted → `runtime::input` |
| `0x801E0FD0` | `kpad_set_sampling_callback_801e0fd0` | Installs/returns the per-channel sampling callback. | translated | high | `801e0fd0_FUN_801e0fd0.c` | promoted → `runtime::input` |
| `0x801E1060` | `kpad_set_connect_callback_801e1060` | Installs/returns the per-channel connect callback. | translated | high | `801e1060_FUN_801e1060.c` | promoted → `runtime::input` |

All 20 rows are **promoted**; the Rust bodies now live in
`src/runtime/dvd.rs` and `src/runtime/input.rs`. The staging file
`src/subsystems/r801c0_80200.rs` contains only its module header, so **no
function remains staged** from this lane. Low confidence on a row still means
weak evidence for the *field or callback semantics*, not an unknown subsystem;
each row has FST-global, control-block or string/caller evidence tying it to
its destination.

Evidence paths are relative to `work/ghidra/main-gb/export/`; the full file name
is the address (lowercase, no `0x`) plus `_FUN_<address>.c`, as in the table.

## Promotion status

Promotion pass: **2026-11-01**.

| Destination | Functions moved | Ledger rows |
|---|---:|---|
| `src/runtime/dvd.rs` (`runtime::dvd`) | 10 | `0x801CA650`–`0x801CB2E0` |
| `src/runtime/input.rs` (`runtime::input`) | 10 | `0x801DF570`–`0x801E1060` |

Remaining staging: **none**. `src/subsystems/r801c0_80200.rs` now contains only
its module header and a pointer to these destinations; the range module
declaration in `src/subsystems/mod.rs` is unchanged because it is outside this
lane. All `# Original` evidence blocks moved with their functions.

## Clusters

### DVD / FST — `0x801CA640–0x801CB500` (16 eligible, 10 translated, 10 promoted to `runtime/dvd.rs`)

The hint addresses `0x801CA680` and `0x801CAE90` sit in a contiguous DVD layer:

* `0x801CA650` caches the FST triple; every later lookup uses it.
* `0x801CA680` is a **path → entry number** resolver. The diagnostic string
  says `DVDConvertEntrynumToPath`, but the body starts from a path and returns
  an index; four callers pass a buffer and use the integer result. The Rust
  name follows the behaviour, the discrepancy is recorded below.
* `0x801CA990` / `0x801CAA00` / `0x801CAA30` / `0x801CAB20` are the open,
  close, async-read and sync-read operations on the descriptor.
* `0x801CAC60` / `0x801CAE90` / `0x801CB010` form the disc-change and
  completion path; `0x801CAE90` carries the "FST in the new disc is too big"
  string.
* `0x801CB2E0` is the small retry/install callback adjacent to the completion
  state machine.

Shared layout recovered here: FST entries are 12 bytes (`name/type`, `offset`,
`length`); the string table starts at `FST + count * 12`; a directory's
`length` word is the first index *after* its subtree.

### WPAD / KPAD — `0x801DF570–0x801E1270` (22 eligible, 10 translated, 10 promoted to `runtime/input.rs`)

* `0x801DF7F0` carries the `WPADInit()` string. It sets a `0xCD0000C0` MMIO
  bit, publishes four 0x9C0-byte control blocks through the table at
  `0x8039E330`, and arms a periodic alarm.
* `0x801DF570` is the per-channel reset called by init; it also installs the
  two internal ring buffers.
* `0x801DFB40` / `0x801DFB90` are small locked field readers.
* `0x801DFC40` / `0x801DFD10` are the connection/reset callbacks; both forward
  a six-byte link key to `0x80201CD8`.
* `0x801E0D30` produces accelerometer deltas; `0x801E0F00` writes a global
  mode byte.
* `0x801E0FD0` and `0x801E1060` are the two hinted callback setters; their
  strings name `KPADSetSamplingCallback` and `KPADSetConnectCallback`, and the
  slots (`+0x8A8`, `+0x8A4`) are the ones the callbacks above invoke.

The KPAD control-block pointer table (`0x8039E330`) is written by WPAD init and
read by every KPAD accessor in this module, which is why the two clusters were
translated together.

## Next targets

Ordered by evidence-per-byte; all addresses are entry addresses from
`functions.tsv`.

### DVD / FST

| Address | Size | Why it is next |
|---|---:|---|
| `0x801CAD30` | 340 | DVD one-time init; wires FST, state objects and `0x801CA650`. |
| `0x801CB1B0` | 272 | Retry handler installed by `0x801CB2E0`; completes the pair. |
| `0x801CB320` | 244 | Error-code classifier (`0x20400`, `0x62800`, `0x52000`, …). |
| `0x801CB4F0` | 1284 | Large retry/continuation used by `0x801CB010`. |
| `0x801CC3E0` | 1144 | Disc-change state machine that calls `0x801CAC60`. |

### WPAD / KPAD

| Address | Size | Why it is next |
|---|---:|---|
| `0x801DFA50` | 112 | WPAD task start; calls `0x801DF7F0`. |
| `0x801E0F40` | 140 | Status/connection query adjacent to the setters. |
| `0x801E0CE0` | 80 | HID parser report counter (string evidence). |
| `0x801E1260` | 84 | Invokes and clears the per-channel `+0x988` callback. |
| `0x801DFE00` | 2292 | Main WPAD/KPAD state machine; registers `0x801DFC40`/`0x801DFD10`. |
| `0x801E1E20` | 5628 | Largest remaining WPAD/KPAD routine. |

### Unexplored bulk

`0x801E1270–0x80200000` contains 339 eligible entries and is the largest
untouched region in this lane. It is not obviously one subsystem: pick a
function with readable string/caller evidence before expanding, and keep the
range-lane rule (entry address only, `< 16` bytes excluded).

One caution from the inventory: `0x801C9E20` (2076 bytes, just below the DVD
block) is **not** DVD. It writes the GX FIFO at `0xCC008000` and manipulates
graphics state, so it belongs to a rendering lane, not this one.

## Uncertainties and caveats

1. **Decompiler prologue artifacts.** `0x802DCD64/68/70/74` and
   `0x802DCDB0/B4/BC/C0` are register-save/restore fragments. Ghidra presents
   them as calls that "return" `r3`/`r4`; the Rust module takes those values as
   normal arguments and returns. Signatures for `0x801CA680`, `0x801CAA00`,
   `0x801CAA30`, `0x801CAB20`, `0x801CAC60` and `0x801DF570` depend on this
   recovery.
2. **`0x801CA680` name/behaviour mismatch.** The embedded string says
   `DVDConvertEntrynumToPath`, while the code resolves a path to an entry
   number. The Rust name follows the code and callers; the string is retained
   as a constant. Treat the official symbol name as unresolved.
3. **`unaff_r27` in `0x801CA680`.** The 8.3 validator reads a register Ghidra
   never defines. The Rust seeds it to the component start; it is only read
   after the first `.` sets it, so reachable behaviour is unchanged. Marked
   `// unknown:` in place.
4. **DVD event ring (`0x80395A5C`).** The five-record layout and timestamp are
   observed; whether it is an error history, command trace or both is not
   proven. `fn_801cac60` and `dvd_read_completion_801cb010` therefore keep
   neutral names/field descriptions.
5. **WPAD r13 tunables.** `r13-0x4BC0`, `-0x4BBC`, `-0x4BC3`, `-0x4BC4`,
   `-0x4BC8`, `-0x4BC9` are stored from six helper calls with no symbols. The
   Rust documents the clamps and shapes but does not guess "sensitivity",
   "volume" or similar names.
6. **MMIO `0xCD0000C0`.** Only the read-modify-write of bit `0x100` is
   observed; the Hollywood block is not identified.
7. **Callback parameter lists.** The connect callback at control-block
   `+0x8A4` and the state callback at `+0x28` are invoked with no/one argument
   exactly as Ghidra shows. If a real caller expects different arguments, the
   `call0`/`call1` sites are the places to correct.
8. **Timestamp source.** `0x801B5FA0` is treated as a 32-bit tick (stored into
   the ring's `+0x10` word); Ghidra types it `void`, so this is shape-based.
9. **No original symbols.** Apart from the two KPAD setter strings and the
   `WPADInit`/DVD messages, all names are address-based or marked inferred.
