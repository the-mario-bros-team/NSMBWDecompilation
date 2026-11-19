# Coverage - Wave 36 (engine::main_loop)

## How to read this file

`src/engine/main_loop.rs` is the MetroTRK debugger lane. The two original
boot entries (`0x802E9300` / `0x802E9398`) are partial (their register /
SPR save-restore cannot be expressed without inline assembly). This pass
adds 28 `translated` bodies around them: the debugger main wrapper,
dispatch loop, init sequence, handler-table trampolines and the two-slot
event queue. Each body documents its Ghidra name, address, size from
`functions.tsv`, and evidence path, in that order.

| address | function | role | status | confidence | evidence |
|---|---|---|---|---|---|
| `0x802E8E54` | `fn_802e8e54` | debugger main wrapper | translated | high | `work/ghidra/main-gb/export/802e8e54_FUN_802e8e54.c` |
| `0x802E8E90` | `fn_802e8e90` | dispatch loop | translated | low | `work/ghidra/main-gb/export/802e8e90_FUN_802e8e90.c` |
| `0x802E9430` | `fn_802e9430` | address map | translated | medium | `work/ghidra/main-gb/export/802e9430_FUN_802e9430.c` |
| `0x802E95CC` | `fn_802e95cc` | init-flag seed | translated | low | `work/ghidra/main-gb/export/802e95cc_FUN_802e95cc.c` |
| `0x802E9614` | `fn_802e9614` | `(0,0,0)` trampoline | translated | high | `work/ghidra/main-gb/export/802e9614_FUN_802e9614.c` |
| `0x802E9624` | `fn_802e9624` | saved-arg forward | translated | medium | `work/ghidra/main-gb/export/802e9624_FUN_802e9624.c` |
| `0x802E96AC` | `fn_802e96ac` | connection callback | translated | medium | `work/ghidra/main-gb/export/802e96ac_FUN_802e96ac.c` |
| `0x802E9820` | `fn_802e9820` | handler install | translated | medium | `work/ghidra/main-gb/export/802e9820_FUN_802e9820.c` |
| `0x802E9874` | `fn_802e9874` | guarded indirect | translated | medium | `work/ghidra/main-gb/export/802e9874_FUN_802e9874.c` |
| `0x802E98A0` | `fn_802e98a0` | indirect `DC` | translated | medium | `work/ghidra/main-gb/export/802e98a0_FUN_802e98a0.c` |
| `0x802E98B4` | `fn_802e98b4` | poll `E0` | translated | medium | `work/ghidra/main-gb/export/802e98b4_FUN_802e98b4.c` |
| `0x802E98F0` | `fn_802e98f0` | poll `E4` | translated | medium | `work/ghidra/main-gb/export/802e98f0_FUN_802e98f0.c` |
| `0x802E992C` | `fn_802e992c` | indirect `F4` | translated | medium | `work/ghidra/main-gb/export/802e992c_FUN_802e992c.c` |
| `0x802E9940` | `fn_802e9940` | indirect `F0` | translated | medium | `work/ghidra/main-gb/export/802e9940_FUN_802e9940.c` |
| `0x802E9954` | `fn_802e9954` | print helper | translated | high | `work/ghidra/main-gb/export/802e9954_FUN_802e9954.c` |
| `0x802E996C` | `fn_802e996c` | word copy + service | translated | medium | `work/ghidra/main-gb/export/802e996c_FUN_802e996c.c` |
| `0x802E9A50` | `fn_802e9a50` | queue reset | translated | high | `work/ghidra/main-gb/export/802e9a50_FUN_802e9a50.c` |
| `0x802E9A74` | `fn_802e9a74` | event dequeue | translated | high | `work/ghidra/main-gb/export/802e9a74_FUN_802e9a74.c` |
| `0x802E9B00` | `fn_802e9b00` | event enqueue | translated | high | `work/ghidra/main-gb/export/802e9b00_FUN_802e9b00.c` |
| `0x802E9BCC` | `fn_802e9bcc` | 12-byte fill | translated | high | `work/ghidra/main-gb/export/802e9bcc_FUN_802e9bcc.c` |
| `0x802E9BEC` | `fn_802e9bec` | init sequence | translated | high | `work/ghidra/main-gb/export/802e9bec_FUN_802e9bec.c` |
| `0x802E9C98` | `fn_802e9c98` | teardown wrapper | translated | high | `work/ghidra/main-gb/export/802e9c98_FUN_802e9c98.c` |
| `0x802E9CBC` | `fn_802e9cbc` | banner print | translated | high | `work/ghidra/main-gb/export/802e9cbc_FUN_802e9cbc.c` |
| `0x802E9E08` | `fn_802e9e08` | idle poll | translated | high | `work/ghidra/main-gb/export/802e9e08_FUN_802e9e08.c` |
| `0x802E9E34` | `fn_802e9e34` | task builder | translated | high | `work/ghidra/main-gb/export/802e9e34_FUN_802e9e34.c` |
| `0x802E9E74` | `fn_802e9e74` | return 0 | translated | high | `work/ghidra/main-gb/export/802e9e74_FUN_802e9e74.c` |
| `0x802E9E7C` | `fn_802e9e7c` | return 0 | translated | high | `work/ghidra/main-gb/export/802e9e7c_FUN_802e9e7c.c` |
| `0x802E9E84` | `fn_802e9e84` | `strlen` | translated | high | `work/ghidra/main-gb/export/802e9e84_FUN_802e9e84.c` |

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/main_loop.rs` | `engine::main_loop` | 28 | 1868 | 15 | 11 (+ 2 low) |

## What this pass covers

- **Debugger entry / loop** (`0x802E8E54`, `0x802E8E90`): the `fn_802e9300`
  / `fn_802e9398` tail-call target now has a body, plus the event dispatch
  loop it runs.
- **Address / state helpers** (`0x802E9430`, `0x802E95CC`, `0x802E996C`):
  the `r13 - 0x4658` window map, the `0xE0000000` base seed, and the
  `r2 - 0x34C0` word copy.
- **Handler-table trampolines** (`0x802E9820` .. `0x802E9940`): the eight
  slots installed by `0x802E96E4`, the guard in `0x802E9874`, and the two
  `0` / `-1` polls.
- **Event queue** (`0x802E9A50`, `0x802E9A74`, `0x802E9B00`, `0x802E9BCC`,
  `0x802E9E34`): the two 12-byte slots at `0x80425700` with count / head /
  sequence globals.
- **Init / teardown / utilities** (`0x802E9614`, `0x802E96AC`, `0x802E9624`,
  `0x802E9954`, `0x802E9BEC`, `0x802E9C98`, `0x802E9CBC`, `0x802E9E08`,
  `0x802E9E74`, `0x802E9E7C`, `0x802E9E84`): the seven-step init chain,
  the banner print, the idle poll, both `return 0` stubs, and `strlen`.

Bucket note: `0x802E0000` (`203` functions in `functions.tsv`, `201`
untranslated before this lane) is the MetroTRK cluster, not the game frame
loop. The lane samples (`0x802E032C` decimal helper, `0x802E0454` double
format, `0x802E7904`/`0x802E8620` double math, `0x802E8E90` dispatch,
`0x802EA094` request-send validation, `0x802EB268` host handler) show
three sub-blocks: string/double helpers (`0x802E032C..0x802E7FFF`),
the MetroTRK core (`0x802E8620..0x802EA9E4`), and host-message handlers
(`0x802EAA88..0x802ED9CC`). This pass takes the MetroTRK core's
boot/init/queue centre (`0x802E8E54..0x802E9E84`) because it shares the TRK
context / queue objects with the module's boot entries.

Deliberate skips in the immediate span:

- `0x802E8E1C` (56 B, cache block store/flush + `sync` + invalidate):
  privileged memory/cache sequence; no platform helper models it.
- `0x802E8F7C` (316 B, memcpy) / `0x802E90B8` (296 B, memset): unaligned
  word read-modify-write loops. A byte loop would be semantically equal but
  not literal; left for a memory-lane pass.
- `0x802E91E0` (288 B, dispatcher): small switch, but its 13 arms call the
  `0x802EBxxx` host handlers outside this cluster.
- `0x802E9498` (308 B, 15-slot flush loop): depends on the skipped cache
  flush and memcpy above, plus a `DAT_00000044` low-vector branch.
- `0x802E9CC8` (116 B): duplicate SDA stores (`1` written twice); the call
  is preserved by address in `fn_802e9bec`.
- `0x802E9D3C` (204 B, reply reader with two stack buffers): callable by
  address from `fn_802e9e08`; left together with the `0x802EA820..` message
  buffers for the next MetroTRK pass.
- `0x802EA8A4` / `0x802EA820` / `0x802EA884` / `0x802EA940` / `0x802EA964` /
  `0x802EA98C` / `0x802EA9B4` and `0x802E942C` (4 B thunk): next cluster;
  `fn_802e9bec` still calls `0x802EA884` by address.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802E8E54` | `fn_802e8e54` | 60 | high |
| `0x802E8E90` | `fn_802e8e90` | 236 | low |
| `0x802E9430` | `fn_802e9430` | 104 | medium |
| `0x802E95CC` | `fn_802e95cc` | 72 | low |
| `0x802E9614` | `fn_802e9614` | 16 | high |
| `0x802E9624` | `fn_802e9624` | 136 | medium |
| `0x802E96AC` | `fn_802e96ac` | 56 | medium |
| `0x802E9820` | `fn_802e9820` | 84 | medium |
| `0x802E9874` | `fn_802e9874` | 40 | medium |
| `0x802E98A0` | `fn_802e98a0` | 20 | medium |
| `0x802E98B4` | `fn_802e98b4` | 60 | medium |
| `0x802E98F0` | `fn_802e98f0` | 60 | medium |
| `0x802E992C` | `fn_802e992c` | 20 | medium |
| `0x802E9940` | `fn_802e9940` | 20 | medium |
| `0x802E9954` | `fn_802e9954` | 24 | high |
| `0x802E996C` | `fn_802e996c` | 84 | medium |
| `0x802E9A50` | `fn_802e9a50` | 36 | high |
| `0x802E9A74` | `fn_802e9a74` | 140 | high |
| `0x802E9B00` | `fn_802e9b00` | 204 | high |
| `0x802E9BCC` | `fn_802e9bcc` | 24 | high |
| `0x802E9BEC` | `fn_802e9bec` | 172 | high |
| `0x802E9C98` | `fn_802e9c98` | 36 | high |
| `0x802E9CBC` | `fn_802e9cbc` | 12 | high |
| `0x802E9E08` | `fn_802e9e08` | 44 | high |
| `0x802E9E34` | `fn_802e9e34` | 64 | high |
| `0x802E9E74` | `fn_802e9e74` | 8 | high |
| `0x802E9E7C` | `fn_802e9e7c` | 8 | high |
| `0x802E9E84` | `fn_802e9e84` | 28 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/main_loop.rs`.
- No address was already in `ledger/functions.csv` (the `802e8e54`
  mention in the `802e9300` evidence string is a callee reference, not a
  ledger row).
- `cargo check --message-format=short` reports no error in
  `src/engine/main_loop.rs` (only transient `src/runtime/dvd.rs` errors
  from a concurrent lane remain, which this finish rule does not own).
- `node tools/check-lane-csv.mjs wave36` reports `OK` for
  `wave36-main-loop.csv`.
