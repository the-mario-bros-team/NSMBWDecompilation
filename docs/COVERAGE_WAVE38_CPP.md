# Coverage - Wave 38 (runtime::cpp)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 28 | 3224 | 10 | 18 |

## What this pass covers

- the voice sub-object machinery `0x802D6110..0x802D6560`: ARC lookup/publish (`0x802D6110`), ARC reset (`0x802D6180`), descriptor initialisers (`0x802D6190/61C0`), the 0x5EC-byte voice constructor (`0x802D6210`), two deleting-destructor twins (`0x802D62F0/6350`), the one-shot output setup (`0x802D6400`) and the frame forwarder (`0x802D6560`), plus the two 8-byte voice forwards `0x802D65D0/E0` (`0x802D3CE0/3E00` at `+0xF8`).
- the 20-slot deferred-task runtime `0x802D66B0..0x802D6C70`: task table at `0x80424BE8` (12 bytes per slot, `{flag@+0, done@+1, arg@+4, callback@+8}`), enqueue (`0x802D67D0`, guarded by `0x801B1140/0x801B1180`), registration (`0x802D6860/68E0`), dispatch (`0x802D6930` via the `0x802D6740/66B0` pair), pad requeue (`0x802D6A00/6A60/6AC0`), fan-out (`0x802D6B40/6BB0/6C30`) and the latched pad byte (`0x802D6C70`).
- the gate mute/restore pair `0x802D6CD0/6E00` plus the state-record helpers `0x802D7010/7030`.
- new constants `AX_TASK_TABLE`, `AX_TASK_HEAD`, `AX_TASK_TAIL`, `AX_TASK_BUSY`, `AX_PAD_LOCK`, `AX_PAD_PENDING`, `AX_GATE_HOOK`, `AX_GATE_HOOK2`, `AX_GATE_MAIN`, `AX_GATE_SECOND`, `AX_GATE_ARG` (r13 `0x8042F680` resolved); no existing constant name reused.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D6110` | `fn_802d6110` | 112 | medium |
| `0x802D6180` | `fn_802d6180` | 8 | high |
| `0x802D6190` | `fn_802d6190` | 48 | high |
| `0x802D61C0` | `fn_802d61c0` | 68 | high |
| `0x802D6210` | `fn_802d6210` | 224 | medium |
| `0x802D62F0` | `fn_802d62f0` | 88 | high |
| `0x802D6350` | `fn_802d6350` | 168 | medium |
| `0x802D6400` | `fn_802d6400` | 352 | medium |
| `0x802D6560` | `fn_802d6560` | 76 | high |
| `0x802D65D0` | `fn_802d65d0` | 8 | high |
| `0x802D65E0` | `fn_802d65e0` | 8 | high |
| `0x802D66B0` | `fn_802d66b0` | 144 | medium |
| `0x802D6740` | `fn_802d6740` | 140 | medium |
| `0x802D67D0` | `fn_802d67d0` | 132 | medium |
| `0x802D6860` | `fn_802d6860` | 124 | medium |
| `0x802D68E0` | `fn_802d68e0` | 72 | medium |
| `0x802D6930` | `fn_802d6930` | 196 | medium |
| `0x802D6A00` | `fn_802d6a00` | 60 | medium |
| `0x802D6A60` | `fn_802d6a60` | 96 | medium |
| `0x802D6AC0` | `fn_802d6ac0` | 120 | medium |
| `0x802D6B40` | `fn_802d6b40` | 104 | medium |
| `0x802D6BB0` | `fn_802d6bb0` | 120 | medium |
| `0x802D6C30` | `fn_802d6c30` | 64 | high |
| `0x802D6C70` | `fn_802d6c70` | 8 | high |
| `0x802D6CD0` | `fn_802d6cd0` | 304 | medium |
| `0x802D6E00` | `fn_802d6e00` | 288 | medium |
| `0x802D7010` | `fn_802d7010` | 28 | medium |
| `0x802D7030` | `fn_802d7030` | 64 | high |

## Deliberate skips

- `0x802D6F20` - `CONCAT44`/`thunk_FUN_802e55a4` double scaling with an `r2`-relative float table; the arithmetic is not recoverable from the export without guessing.
- `0x802D7070` - paired-single artefacts (`ldexpf`/`unaff_GQR0`, `docs/QUALITY.md` §5.3); callers (`0x802D6560`) keep the raw address instead.
- `0x802D7170`, `0x802D71F0`, `0x802D7410`, `0x802D7500+` - different runtime context (busy-wait/input/console); left for the next pass.
- register-save stubs `0x802DCCCC..0x802DCDBC` and 4-byte thunks `0x802D2F10/3260`, `0x802DEBB4` - alternate entry points, not callable routines.
- `0x802D0030..0x802D2B60` / `0x802D3CE0..0x802D5DD0` EGG/voice twins outside this cluster, many with `ldexpf`/`FUN_802DCD74` lossage; their callees are kept as raw addresses by this pass.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` reports no errors in `src/runtime/cpp.rs` (remaining errors are in unrelated files owned by other lanes).
