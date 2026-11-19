# Coverage wave 5 — WPAD/KPAD report pipeline and extension decoders

## How to read this file

This file records what the input lane added in wave 5, not what the address
band contains. Read it in five steps:

1. **Scope** gives the address band, the selection rule and the byte counts.
2. **Summary** states the function count and the status/confidence split.
3. The **function table** lists every translated body in address order with
   its plain-English role, exact Ghidra evidence and destination.
4. **What the pass adds** explains the attach, dispatch, decoding and
   calibration clusters in prose.
5. **Unknowns / next targets / gates** record what is still open.

The complete per-function ledger is `ledger/agents/wave5-input.csv`; it uses
the header `module,address,name,status,confidence,evidence,rust_file` and the
module name `runtime::input`.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` = the complete
recovered control flow and every call are accounted for; `partial` = only the
entry/exit and major effects; `stub` = address and role only. **Confidence**
measures the strength of the evidence for the *semantics*, not code quality.

Every Rust function keeps `# Original` with the Ghidra name, address, size and
evidence path. No symbol names were invented. Functions keep their
`fn_80xxxxxx` names unless the role is proven; `// observed:`, `// inferred:`
and `// unknown:` markers carry the same information at the call site.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`.

| Item | Count |
|---|---:|
| Entry addresses in `[0x801D4000, 0x801F0000)` | 312 |
| Eligible, size `>= 16` bytes | 281 |
| Eligible bytes | 104,820 |
| Already translated in `src/runtime/input.rs` before this pass | 30 |
| Selected and translated by this pass | **24** |
| Bytes represented by this pass | **14,684** |
| Status `translated` | 24 |
| Status `partial` | 0 |
| Status `stub` | 0 |

The selection rule was: an entry in the band that was **not** in
`ledger/functions.csv` and **not** already present in
`src/runtime/input.rs`, with a complete Ghidra body, at least 16 bytes, and a
role that belongs to WPAD/KPAD (control-block offsets, the `0x8039E330` channel
table, the `0x80343E90` report table, or the `0x803A1A60` load-cell tables).
Large command-builder routines were deliberately left for a later pass; see
**Next targets**.

The 24 bodies cover 14 % of the eligible bytes in the band. They are not a
sweep: they close the path from a Bluetooth attach event to a decoded
`KPADStatus`-shaped block, including the nunchuk/classic calibration records.

## Translated functions

Evidence paths are relative to `work/ghidra/main-gb/export/`.

| Address | Size | Rust function | Role (plain English) | Status | Confidence | Evidence |
|---|---:|---|---|---|---|---|
| `0x801DE9D0` | 404 | `fn_801de9d0` | Accepts a decoded report, snapshots the previous copy and retries a stale link. | translated | medium | `801de9d0_FUN_801de9d0.c` |
| `0x801DFE00` | 2,292 | `fn_801dfe00` | Queues the seven-command attach/init sequence for one channel. | translated | medium | `801dfe00_FUN_801dfe00.c` |
| `0x801E0820` | 1,204 | `fn_801e0820` | Bluetooth connect/disconnect handler; assigns a channel and drains/queues the ring. | translated | high | `801e0820_FUN_801e0820.c` |
| `0x801E3420` | 52 | `fn_801e3420` | Reads the global sampling-mode byte `r13-0x4BC9`. | translated | high | `801e3420_FUN_801e3420.c` |
| `0x801E3460` | 80 | `fn_801e3460` | Writes the global sampling-mode byte with the `0..=0x7F` clamp. | translated | high | `801e3460_FUN_801e3460.c` |
| `0x801E34B0` | 312 | `fn_801e34b0` | Multi-factor "may this channel sample now?" gate. | translated | medium | `801e34b0_FUN_801e34b0.c` |
| `0x801E35F0` | 132 | `fn_801e35f0` | Public link/connection/ready predicate. | translated | high | `801e35f0_FUN_801e35f0.c` |
| `0x801E3680` | 528 | `fn_801e3680` | Queues a short `0x18` command block and bumps the command counter. | translated | medium | `801e3680_FUN_801e3680.c` |
| `0x801E3890` | 52 | `fn_801e3890` | Reads the global extension kind `r13-0x4BC3`. | translated | high | `801e3890_FUN_801e3890.c` |
| `0x801E38D0` | 68 | `fn_801e38d0` | Reads the per-channel word at `+0x838`. | translated | low | `801e38d0_FUN_801e38d0.c` |
| `0x801E3920` | 88 | `fn_801e3920` | Reads the extension sub-mode `+0x8CB` behind the `+0x838` guard. | translated | medium | `801e3920_FUN_801e3920.c` |
| `0x801E4A40` | 524 | `fn_801e4a40` | Queues the `AA AA AA 55 xx xx xx` extension handshake. | translated | medium | `801e4a40_FUN_801e4a40.c` |
| `0x801E4C50` | 616 | `fn_801e4c50` | Generic 0x30-byte command-ring push with the `0x12` block layout. | translated | high | `801e4c50_FUN_801e4c50.c` |
| `0x801E5460` | 348 | `fn_801e5460` | Extension-detach completion and up-to-`0x20` retry loop. | translated | medium | `801e5460_FUN_801e5460.c` |
| `0x801E60B0` | 1,340 | `fn_801e60b0` | Nunchuk/classic calibration-record checksum and decode. | translated | high | `801e60b0_FUN_801e60b0.c` |
| `0x801E65F0` | 1,080 | `fn_801e65f0` | Extension identification and calibration command selection. | translated | medium | `801e65f0_FUN_801e65f0.c` |
| `0x801E6A30` | 420 | `fn_801e6a30` | Checksums and copies the 0x38-byte calibration record. | translated | high | `801e6a30_FUN_801e6a30.c` |
| `0x801E6BE0` | 296 | `fn_801e6be0` | HID report-id dispatch through the `0x80343E90` table. | translated | high | `801e6be0_FUN_801e6be0.c` |
| `0x801E7050` | 808 | `fn_801e7050` | Extension-register read reply, range check and calibration dispatch. | translated | high | `801e7050_FUN_801e7050.c` |
| `0x801E74C0` | 1,172 | `fn_801e74c0` | Balance-board load-cell decode and per-channel rotation. | translated | medium | `801e74c0_FUN_801e74c0.c` |
| `0x801E7BE0` | 1,228 | `fn_801e7be0` | Nunchuk/classic stick, trigger and checksum decode. | translated | high | `801e7be0_FUN_801e7be0.c` |
| `0x801E80B0` | 444 | `fn_801e80b0` | Classic-controller 16-byte extension decode. | translated | high | `801e80b0_FUN_801e80b0.c` |
| `0x801E8A30` | 928 | `fn_801e8a30` | Core report decode plus nunchuk/classic extension dispatch. | translated | high | `801e8a30_FUN_801e8a30.c` |
| `0x801E8DD0` | 268 | `fn_801e8dd0` | Alternate core/balance-board report decode. | translated | high | `801e8dd0_FUN_801e8dd0.c` |

All 24 rows are `translated` and live in **`src/runtime/input.rs`**
(`runtime::input`). The table is ordered by address, matching the ledger.

## What the pass adds

The pass follows one report from attach to decode:

1. **Attach and detach.** `0x801E0820` is the connection callback registered
   through `0x801F3BF0`. On connect it asks `0x801E0700` for a channel, resets
   the channel with `0x801DF570`, records the device name (`Nintendo RVL CNT`
   vs `Nintendo RVL WBC`), sets the extension kind at `+0x8C1`, and queues a
   `0x17` read whose completion is `0x801DFE00`. `0x801DFE00` then queues the
   seven-command bring-up sequence: a `0x12` reset, a `0x1A` LED/state block, a
   `0x11` sampling block, three `0x17` register reads (`0x2A`, `0x62`, and
   `0x176C` or `0`) and a `0x15` report request. On disconnect the same
   handler marks the link `-1`, drains every queued block and invokes its
   `+0x2C` completion with `-1`, clears the report ring through `0x801E19C0`,
   and resets the channel.
2. **Command rings.** `0x801E4C50` is the standalone ring push used by the
   attach path; it reproduces the `(head - tail + capacity) % capacity` count,
   the `capacity - 1` full test, the `memset`/`memcpy` slot update and the
   `capacity - 1` head wrap. `0x801E4A40` queues the `AA AA AA 55` handshake in
   one-, seven- or four-times-seven-byte chunks, and `0x801E3680` queues the
   short `0x18` command used by the player layer.
3. **Report dispatch.** `0x801E6BE0` accepts ids `0x20`–`0x3F`, indexes
   `0x80343E90[(id - 0x20)]`, clears the selected 0x60-byte block for normal
   reports, calls the decoder as `(channel, report, out)`, toggles the
   `+0x8C8` buffer selector with `CLZ >> 5`, and then calls the existing
   `0x801E1B70` / `0x801E1C90` status and sampling helpers.
4. **Core decode.** `0x801E8A30` and `0x801E8DD0` fill the button word,
   accelerometer deltas and stale marker, copy the extension bytes to
   `0x803A1AB0`, XOR-decode them in place through `0x801EA520`, and dispatch on
   the extension kind. `0x801E8A30` handles the nunchuk inline (kind `1`),
   calls `0x801E7BE0` for classic (kind `2`) and `0x801E80B0` for kind `4`.
   An identity match against `0x802F6D18` upgrades a clean marker to `0xF9`.
5. **Extension decode.** `0x801E7BE0` unpacks the nunchuk/classic stick and
   trigger bytes and clamps them against the per-channel calibration values.
   `0x801E80B0` unpacks the 16-byte classic report (two sticks, two triggers,
   shoulder bits). `0x801E74C0` unpacks the balance-board load cells (four
   3-byte or four 10-byte samples), then applies the per-channel offset and
   rotation tables at `0x803A1A60`–`0x803A1AA0` using the original sin/cos
   helpers.
6. **Calibration.** `0x801E7050` is the extension-register read reply: it
   range-checks the address, copies the payload into `control + 0x974`, and
   dispatches by register address. `0x801E6A30` checksums the 0x38-byte
   record, `0x801E60B0` unpacks the nunchuk accelerometer or classic stick
   calibration records and prints the `0G`/`1G`/`FS`/`CL`/`CR`/`LR` diagnostics,
   and `0x801E65F0` identifies the attached extension, starts the matching
   sampling path, and queues the `0x4A400xx` calibration reads. `0x801E5460`
   is the detach/retry completion for those reads. `0x801DE9D0` is the accept
   gate that stores a changed report and retries an idle link after
   `mode * 60` seconds.

## Unknowns and caveats

1. **Report table contents.** `0x80343E90` is indexed exactly, but the table
   is data and the Ghidra export does not name its 32 entries. The call is
   therefore made through the table address; the per-id handler names are not
   invented.
2. **`fn_801e74c0` floating-point artifacts.** Ghidra emits `ldexpf` calls on
   `unaff_GQR0` at entry and exit. Those are paired-single/GQR load-scaling
   artifacts, not functions with observable results; they are documented and
   omitted. The load-cell packing and rotation are transcribed.
3. **`fn_801e60b0` classic checksum index.** If neither 16-byte record matches
   its checksum, `checksum_index` stays `-1`; the original classic branch then
   reads one byte before the calibration buffer. The Rust preserves that
   address arithmetic and marks it `// unknown:`.
4. **`fn_801de9d0` elapsed-time helper.** Ghidra calls `0x802DCEC4` with the
   64-bit time difference and `bus_clock >> 2` and compares the uninitialised
   `r4` result against `mode * 60`. The Rust performs the same 64-bit division
   directly and marks the reconstructed quotient `// inferred:`.
5. **`fn_801e4c50` default selector.** The command switch only covers
   `0..=0xF`; other selectors read stale stack data in the original. The Rust
   zeroes byte `+5` for those selectors and marks it `// unknown:`.
6. **`fn_801e34b0` Bluetooth counters.** `0x801F3570`, `0x801F61C0`,
   `0x801F6220` and `0x801F6280` are called by address. Their exact SDK names
   and units are not recovered; the gate uses the observed byte/word shapes.
7. **`fn_801e0820` inline labels.** The `r13-0x5E6C` / `-0x5E74` and
   `r13-0x5E78` arguments are SDA *addresses* of inline strings, not pointers
   stored in SDA; the code passes the addresses exactly as Ghidra shows.
8. **`fn_801e65f0` extension probes.** `0x801DDA50/60/70/80` and
   `0x801EFEF0` are probed for kind values `4`, `0x10`, `0x11`, `0x12` and
   balance board. The returned booleans are exact; the official sub-id names
   are not proven.

## Next targets

Closest remaining bodies in the same band, all still untranslated:

| Address | Size | Why next |
|---|---:|---|
| `0x801DDF00` | 272 | Link-quality filter and hysteresis around `+0x983`/`+0x984`. |
| `0x801DE010` | 2,296 | Report comparator called by `fn_801de9d0`; closes the accept path. |
| `0x801DEB70` | 2,524 | Attach/report decoder that registers `0x801E0820`. |
| `0x801E1E20` | 5,628 | Public mode/command builder used by the player layer. |
| `0x801E39C0` | 4,224 | Large remaining channel/command routine. |
| `0x801E55C0` | 2,792 | Balance-board calibration and weight filter called by `fn_801e7050`. |
| `0x801E9730` / `0x801E9E20` | 1,776 / 1,784 | Nunchuk/classic and fallback sampling starts. |
| `0x801EA520` | 720 | In-place extension XOR decoder called by both core decoders. |
| `0x801EA820` | 204 | Raw extension command queue helper used by `fn_801e4a40`. |

## Gates

- `node tools/doc-lint.mjs src` → `ok: true`, 0 violations (68 files).
- `cargo check` → exit 0, no diagnostics for `src/runtime/input.rs`.
- No files outside this lane were edited.
