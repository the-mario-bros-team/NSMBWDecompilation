# Coverage - Wave 44b (resource::registry manual leaf batch)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 17 | 280 | 16 | 1 |

## What this pass covers

- Zero and byte-flag helpers (`0x80161560`, `0x80168BB0`, `0x8016DCE0`): the
  four-word clear, the `+0x1C`/`+0x1D` flag pair and the `+10` half-word store.
- Element-dispatch wrappers (`0x80167B10`-`0x80167B50`, 5 bodies): the same
  `this+0x28` array load and literal `0x38` stride as the first manual batch,
  each tail-calling a different element method.
- Element accessors (`0x80166F40`, `0x80167B60`, `0x80167B80`, `0x80167BA0`):
  the `+0x34` byte setter, its duplicate, the `+0x28` `f32` getter and the
  `+0x2C` `f32` setter.
- Object call/store helpers (`0x80169C80`, `0x8016A000`, `0x8016A010`,
  `0x8016DA50`): two `this+4` calls, the `+0x38` word store and the
  `0x80329670` vtable install with a cleared `+4`.
- Triple-float store (`0x8016A130`): three double-converted values written at
  `[this+4] + 0xDC`, `+0xE0` and `+0xE4`.

This batch continues the coordinator-written Wave 44 work while the free model
remains at `Free usage limit reached`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80161560` | `fn_80161560` | 24 | high |
| `0x80166F40` | `fn_80166f40` | 20 | high |
| `0x80167B10` | `fn_80167b10` | 16 | high |
| `0x80167B20` | `fn_80167b20` | 16 | high |
| `0x80167B30` | `fn_80167b30` | 16 | high |
| `0x80167B40` | `fn_80167b40` | 16 | high |
| `0x80167B50` | `fn_80167b50` | 16 | high |
| `0x80167B60` | `fn_80167b60` | 20 | high |
| `0x80167B80` | `fn_80167b80` | 20 | high |
| `0x80167BA0` | `fn_80167ba0` | 20 | high |
| `0x80169C80` | `fn_80169c80` | 8 | high |
| `0x8016A000` | `fn_8016a000` | 8 | high |
| `0x8016A010` | `fn_8016a010` | 24 | medium |
| `0x80168BB0` | `fn_80168bb0` | 20 | high |
| `0x8016DA50` | `fn_8016da50` | 8 | high |
| `0x8016DCE0` | `fn_8016dce0` | 8 | high |
| `0x8016A130` | `fn_8016a130` | 20 | high |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs`.
