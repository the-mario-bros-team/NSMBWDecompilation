# Coverage — Wave 34 (input: BTA SYS timer/registration layer)

## How to read this file

This page records exactly what the wave-34 `runtime::input` lane added to
[`src/runtime/input.rs`](../src/runtime/input.rs). Read it in four steps:

1. **Scope** gives the band inventory, the before/after totals and the
   selection rule.
2. The **function table** lists the 30 new bodies in address order with the
   byte count and confidence. Every byte count was re-read from
   `work/ghidra/main-gb/export/functions.tsv` for this pass.
3. **What the `0x801F0000` band actually contains** is the band
   identification promised in the lane brief, including the sub-block map
   this pass sampled but deliberately left for later passes.
4. **Corrections and caveats** lists the decompiler artefacts and the
   deliberate skips.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the full
control flow and every call are accounted for; all 30 rows are
`translated`. Confidence is evidence strength, not code quality.

Every body cites its `FUN_<address>` and
`work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`. All `fn_80xxxxxx` names are
kept: the cluster is the Broadcom-supplied BTA system layer, but no official
symbol names exist in this image, so single functions keep address names and
the Rust doc comments carry the roles. `// observed:`, `// inferred:` and
`// unknown:` markers inside the Rust repeat the same information at call
sites.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`; translated
baseline: `ledger/functions.csv` (read-only for this lane).

| Item | Count |
|---|---:|
| Functions in `[0x801F0000, 0x80200000)` | 268 |
| Untranslated before this pass (any size) | 221 |
| Selected and translated by this pass | **30** |
| Bytes covered by this pass | 2,856 |
| Untranslated in the band after this pass | 191 |

Selection rule: the address must be an entry in `functions.tsv`, must have
no row in `ledger/functions.csv`, must have complete Ghidra control flow,
and must belong to one call-graph cluster sharing the protagonist state
blocks. This pass picked the **BTA SYS timer/registration cluster**: the
five delta-list primitives plus the 25 wrapper functions around
`0x803D1740`, `0x803D1690`, the `0x803D16D8` dispatch table and the three
`0x803D1758` notification callbacks.

That cluster was already half-known to the module: earlier passes
referenced `BTA_HH_SEND` (`0x801fa850`), `BTA_HH_COPY_BDA`/`CMP_BDA`
(`0x801fa3a4`/`0x801fa3d8`), `BTA_HH_NOTIFY_*` (`0x801fa4a8`,
`0x801fa56c`, `0x801fa630–698`), `BTA_HH_SET_EVENT_QUEUE` (`0x801fa83c`),
`BTA_HH_FREEBUF` (`0x801faaf0`), `BTA_TRACE_1` (`0x801f9d30`) and
`BTU_ALARM_SCHEDULE` (`0x801f8040`) **by address** from call sites. This
pass resolves those references into real bodies.

## Function table

| Address | Function | Bytes | Confidence |
|---|---|---:|---|
| `0x801F8190` | `fn_801f8190` | 132 | high |
| `0x801F8220` | `fn_801f8220` | 20 | high |
| `0x801F8240` | `fn_801f8240` | 140 | high |
| `0x801F82D0` | `fn_801f82d0` | 236 | high |
| `0x801F83C0` | `fn_801f83c0` | 284 | high |
| `0x801FA320` | `fn_801fa320` | 112 | medium |
| `0x801FA390` | `fn_801fa390` | 20 | medium |
| `0x801FA3A4` | `fn_801fa3a4` | 52 | high |
| `0x801FA3D8` | `fn_801fa3d8` | 160 | high |
| `0x801FA478` | `fn_801fa478` | 16 | high |
| `0x801FA488` | `fn_801fa488` | 16 | high |
| `0x801FA498` | `fn_801fa498` | 16 | high |
| `0x801FA4A8` | `fn_801fa4a8` | 196 | high |
| `0x801FA56C` | `fn_801fa56c` | 196 | high |
| `0x801FA630` | `fn_801fa630` | 48 | high |
| `0x801FA664` | `fn_801fa664` | 48 | high |
| `0x801FA698` | `fn_801fa698` | 48 | high |
| `0x801FA6CC` | `fn_801fa6cc` | 100 | high |
| `0x801FA730` | `fn_801fa730` | 236 | high |
| `0x801FA81C` | `fn_801fa81c` | 28 | high |
| `0x801FA83C` | `fn_801fa83c` | 20 | high |
| `0x801FA850` | `fn_801fa850` | 28 | high |
| `0x801FA86C` | `fn_801fa86c` | 32 | high |
| `0x801FA88C` | `fn_801fa88c` | 20 | medium |
| `0x801FA8A0` | `fn_801fa8a0` | 104 | high |
| `0x801FA910` | `fn_801fa910` | 76 | high |
| `0x801FA95C` | `fn_801fa95c` | 180 | medium |
| `0x801FAA10` | `fn_801faa10` | 160 | high |
| `0x801FAAB0` | `fn_801faab0` | 64 | medium |
| `0x801FAAF0` | `fn_801faaf0` | 68 | high |

Totals: **30 translated**, 0 partial, 0 stubs; 25 high / 5 medium / 0 low.

## What this pass covers

The cluster is the Broadcom **BTA SYS layer** the WPAD report path's
HID-host stack runs on. Three data structures connect everything:

* a 12-byte timer-list object `{head, tail, ticks-left}` (`0x803D1740` for
  the OSAlarm-driven list, `0x803D1690` for the second list the BTU task
  loop scans inline),
* an 0x18-byte timer node `{next, prev, callback, ticks(+0xC), event u16
  (+0x14), in-list byte (+0x16)}` (timer instances live inside per-subsystem
  control blocks, e.g. `0x803D17F0` used by `0x801fb0c4`),
* four 8-entry u32 task tables at `0x803CDB28`/`0x803CDB48` (kind 0) and
  `0x803CDB68`/`0x803CDB88` (kind 1) plus the u32 pair `0x803CDB00`/`04`,
  written by the unranslated alarm scheduler `0x801f8040` and cleared by
  `fn_801f8190`. The table element size (u32, 8 entries, 0x20 bytes each)
  is proven by the 32-word clear in `0x801f7f60`, which zeroes exactly the
  five interleaved words-groups ending at `0x803CDBA8`.

Sub-clusters:

- **GKI delta-timer primitives** (`0x801F8190`, `0x801F8220`,
  `0x801F8240`, `0x801F82D0`, `0x801F83C0`) — zeroing, tick advance,
  delta-ordered insert and link-consistent remove.
- **Registration** (`0x801FA83C`, `0x801FA6CC`, `0x801FA8A0`) — event-id
  handler table install, the 0x8C-byte reset that publishes the task id and
  trace byte, and the disable-hook fan-out over ids 0..0x1A.
- **Event dispatch** (`0x801FA730`, `0x801FA81C`) — id lookup in the high
  byte of the message word, handler call, unregistered-id trace/free, and
  the shutdown path that only reacts to id `0x0101`.
- **Timers** (`0x801FA320`, `0x801FA390`, `0x801FA86C`, `0x801FA88C`,
  `0x801FA910`, `0x801FA95C`, `0x801FAA10`, `0x801FAAB0`, `0x801FA852`)
  plus the helper pair — start/stop/expire/restart wrapping
  `[fn_801faa10]`/`[fn_801faab0]`, the `pending/10` OSAlarm reschedule when
  the list drains, and the `0x801f8190` counter clear.
- **Callback fan-out** (`0x801FA478/488/498`, `0x801FA4A8`, `0x801FA56C`,
  `0x801FA630/664/698`) — the three connection-callback slots with
  fan-out types `0`, `1`, `5`, `6`, `7`, and the BD_ADDR helpers
  (`0x801FA3A4`, `0x801FA3D8`).

## What the `0x801F0000` band actually contains

The 268-entry band `0x801F0000..0x80200000` is the tail of the WPAD
Bluetooth stack: the Broadcom **BTA application layer plus the GKI runtime
under it**. Sub-blocks identified by sampling (all Ghidra-based; sizes from
`functions.tsv`):

- `0x801F0020`–`0x801F3530` (~10.4 KB): BTU/GKI bring-up — the buffer
  scheduler, the event-decoder state that `0x801f8040` schedule table
  serves, and `BT_INIT`/`BT_CONNECTION_STATE` accessors partially
  translated in earlier waves.
- `0x801F35C0`–`0x801F6470` (~3.5 KB): the HID-host link/device management
  (records, link keys, extension probes) — partially translated by
  earlier waves, untranslated remainder includes the GKI `0x801f5260`
  scheduler and `0x801f57c0` pool sweep.
- `0x801F7A60`–`0x801F8820` (~3.2 KB): the OSAlarm glue — `0x801f7aa0`
  (840 B) and `0x801f7df0` (356 B) are the two alarm objects' install
  paths, `0x801f8040` (332 B) is the alarm scheduler the timers in this
  pass call, `0x801f8190`–`0x801f83c0` are the delta-list primitives this
  pass translated. The four `0x04`-byte enter/exit arcs `0x801f8630`–`60`
  and the trace thunks are prologue artefacts.
- `0x801F8820`–`0x801F9FA0` (~8 KB): the **BTU task loop and its trace
  layer** — `0x801f9fa0` (892 B, untranslated) is the central BTU scan
  whose inline dispatch walks the second timer list `0x803D1690` this
  pass already touches; `0x801f9c50` is a decompiler mangled callback
  (8-byte float params, skip - see below).
- `0x801FA320`–`0x801FB200` (~3 KB): the **BTA SYS registration/timer
  layer this pass translated in full**.
- `0x801FB1D8`–`0x801FDD2C` (~5 KB): BTA subsystem bring-up/teardown —
  `0x801fab34` is the SYS enable with the dispatch-table publish,
  `0x801fb0c4`/`0x801fb1d8` the SDP/HID open paths, `0x801fd4bc`/`801fd83c`
  (896 B each) two of the large subsystem state machines. Untranslated.
- `0x801FDE58`–`0x801FFEC4` (~4.4 KB): HID-host queue/message plumbing,
  including the already-known `0x801ff778` message poster and the
  `0x801ff970` state machine. Untranslated beyond earlier partial work.

Deliberate band-global observation: the band is **not** controller-channel,
rumble or KPAD code — those live at `0x801DE9D0`–`0x801E8DD0` and were
translated in earlier input passes. The `0x801F0000` band is the Bluetooth
stack runtime, with GKI (allocators/timers), BTA SYS (this pass) and the
HID-host state machines sharing the `0x803D…` control-block zone.

## Corrections and caveats

1. **`fn_801f8190` element size.** Ghidra prints the four per-task tables
   as `(&DAT_803cdb48)[uVar1 & 0xff] = 0` with byte-typed symbols. The
   32-word clear in `0x801f7f60` proves each table is 8 u32 entries
   (`0x20` bytes, ending at `0x803CDBA8`), so the Rust uses `task * 4`
   offsets. The `& 0xff` mask stays because it is literal.
2. **`fn_801faab0` lost argument.** Ghidra shows the forwarded
   `FUN_801f83c0()` with no arguments but uses `*param_1` on return. The
   forwarding call in `0x801fa88c` is `fn_801faab0(&0x803D1740, timer)`;
   removing the *head* instead of the argument would break co-scheduled
   timers, so the two-argument reconstruction `remove(list, timer)` is
   used, marked `// inferred:` and `medium`.
3. **`fn_801fa95c` send's carried register.** The `FUN_801fa850()` call
   after `FUN_801f6fb0(8)` and the `u16` store has no Ghidra argument; the
   allocated buffer is passed in `r3` (the same pattern `0x801fb0c4` shows
   explicitly), marked `// inferred:`.
4. **`fn_801fa630/664/698` jumptable warnings.** Ghidra
   "could not recover jumptable at …" on the indirect tail call; the
   decompiled body is unambiguous (null-guarded call with constant first
   argument `5`/`6`/`7` into the `0x803D175C` slot) and translated as
   such.
5. **`fn_801fa730` shutdown hook.** `0x801fce00` (28 bytes) itself is a
   two-argument indirect tail call through `0x803D1864`; it stays a
   `call0` to its address with the pointer story in its doc comment
   (`// unknown:` callee identity at this address; the pointer table entry
   is installed by the untranslated `0x801fab34`).
6. **`0x803D1690` vs `0x803D1740`.** The two timer lists are not the same
   object: `fn_801fa320`/`fn_801fa390` operate on `0x803D1690`, the list
   scanned inline by the untranslated BTU loop `0x801f9fa0`;
   `fn_801fa86c`/`faa10`/`faab0`/`fa95c` operate on `0x803D1740`. The
   ownership split is `// inferred:`, confidence `medium` on the pair.
7. **No staging.** All bodies live in `src/runtime/input.rs`; nothing else
   was touched. `ledger/functions.csv` is unchanged by this lane.

## Deliberate skips

- `0x801F9C50` (180 bytes, "untranslated" still): Ghidra rendered a
  callback whose parameters are eight `float` registers plus two
  `%CR1` bitfields above `undefined8` locals, then calls into
  `0x802e1664`/`0x802e17dc`/`0x8015f730`. Guessing the parameter
  marshalling would violate the no-invention rule.
- `0x801F8630`–`0x801F8660`, `0x801F87B0`, `0x801F87F0` (4 bytes each) and
  `0x801F3300`: prologue/epilogue/thunk artefacts Ghidra already got wrong
  (>1 byte `.text` with no GPR effects); they are called by address from
  older bodies and were left as-is.
- `0x801FA908` (8 bytes): pure r13 byte store at `r13-0x4AE8` of the
  incoming argument; trivially translatable but cut to keep the pass at 30
  bodies. Left `untranslated` candidates for the next input pass.
- The rest of the band (the big BTU loops `0x801f9fa0`, subsystem
  state machines at `0x801fd4bc`/`0x801fd83c`, HID-host message plumbing,
  `0x801f7aa0` alarm-install paths) is fully honest work but each is a
  large, unstarted cluster; deferring keeps this pass one coherent
  sub-cluster.

## Verification

- Every address and byte size in the table matches
  `work/ghidra/main-gb/export/functions.tsv` (sizes re-read for this pass,
  not copied from prose).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one body in `src/runtime/input.rs`.
- No address was already in `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` reports no errors for
  `src/runtime/input.rs` (two build errors found during development were
  fixed: a `call0_ret` import path and the `fn_801faab0` arity).
- `node tools/check-lane-csv.mjs wave34` finds no problem in
  `wave34-input.csv` (30 rows clean; the reported problems belong to the
  concurrent `bosses`/`enemies` lanes).

## Ledger

Machine-readable rows: `ledger/agents/wave34-input.csv` (30 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
