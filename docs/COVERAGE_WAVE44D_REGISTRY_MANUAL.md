# Coverage - Wave 44d (resource::registry lookup/copy batch)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 15 | 560 | 12 | 3 |

## What this pass covers

- Pointer/flag queries: the `+4` fallback getter, the two `+0xB`/`+0xF` mask
  tests, the `+0x10`/`+0x14` double-indirect getters and the
  `(-x | x) >> 31` non-zero helper.
- Store/copy helpers: the guarded `+8` install, the three-word copy to
  `[this+4]+0xDC..0xE4`, the three-word copy from `+0xC/+0x1C/+0x2C`, and the
  `r13-0x4810` to `r13-0x5220` install with its three calls.
- Forwarders: the `r13-0x46B0` five-argument call, the two-call init wrapper
  and the `r13-0x5250` virtual-slot `+0xC` call.
- Table lookup: the flagged index read from `r13-0x520C` and the
  `0x80377D1C + index*0x18` byte result.
- Virtual dispatch: one `+0x14` no-argument trampoline.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801616C0` | `fn_801616c0` | 56 | high |
| `0x801620E0` | `fn_801620e0` | 40 | high |
| `0x801621E0` | `fn_801621e0` | 40 | high |
| `0x80162590` | `fn_80162590` | 28 | high |
| `0x801625B0` | `fn_801625b0` | 28 | high |
| `0x80162A20` | `fn_80162a20` | 44 | high |
| `0x80163660` | `fn_80163660` | 32 | high |
| `0x801683E0` | `fn_801683e0` | 28 | medium |
| `0x8016A150` | `fn_8016a150` | 32 | high |
| `0x8016C7F0` | `fn_8016c7f0` | 36 | high |
| `0x8016C830` | `fn_8016c830` | 32 | high |
| `0x8016F130` | `fn_8016f130` | 28 | high |
| `0x8016F1F0` | `fn_8016f1f0` | 48 | high |
| `0x8016F460` | `fn_8016f460` | 48 | medium |
| `0x8016C5A0` | `fn_8016c5a0` | 32 | medium |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs`.
