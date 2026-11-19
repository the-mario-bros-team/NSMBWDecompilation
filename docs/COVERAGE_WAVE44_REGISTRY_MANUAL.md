# Coverage - Wave 44 (resource::registry manual leaf batch)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 20 | 264 | 20 | 0 |

## What this pass covers

- SDA and storage wrappers (`0x80161380`-`0x801637E0`, 7 bodies): the
  `r13-0x52F0` store/call pair, the constant-`1` return, the four-argument
  forwarder, the `+8` clear with a `+4` release call, and the two
  double-to-`f32` field stores.
- Object clear/call helpers (`0x80168B20`-`0x8016F830`, 5 bodies): the `+8`
  clear plus no-argument call, the `+0x1C` byte clear plus call, the
  `r13-0x5208` install/getter pair with its null-path teardown, and the `+4`
  word store.
- Element dispatch wrappers (`0x80166EF0`-`0x80166F30`, 5 bodies): each loads
  the element array at `this+0x28`, applies the literal `0x38` stride and
  tail-calls one element method.
- Handle-forwarding wrappers (`0x8016A170`-`0x8016A190`, 3 bodies): each loads
  the `this+4` handle and calls `FUN_802574E0`/`FUN_802575D0` with mode `0`
  or `2`.

This batch was written directly by the coordinator after the free model hit
`Free usage limit reached`; it follows the same lane rules and was compiled
and validated before merging.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80161380` | `fn_80161380` | 8 | high |
| `0x80161390` | `fn_80161390` | 8 | high |
| `0x80161D90` | `fn_80161d90` | 8 | high |
| `0x80162B20` | `fn_80162b20` | 20 | high |
| `0x80163690` | `fn_80163690` | 16 | high |
| `0x801637D0` | `fn_801637d0` | 12 | high |
| `0x801637E0` | `fn_801637e0` | 8 | high |
| `0x80168B20` | `fn_80168b20` | 12 | high |
| `0x80168B80` | `fn_80168b80` | 12 | high |
| `0x8016F640` | `fn_8016f640` | 16 | high |
| `0x8016F660` | `fn_8016f660` | 8 | high |
| `0x8016F830` | `fn_8016f830` | 8 | high |
| `0x80166EF0` | `fn_80166ef0` | 16 | high |
| `0x80166F00` | `fn_80166f00` | 16 | high |
| `0x80166F10` | `fn_80166f10` | 16 | high |
| `0x80166F20` | `fn_80166f20` | 16 | high |
| `0x80166F30` | `fn_80166f30` | 16 | high |
| `0x8016A170` | `fn_8016a170` | 16 | high |
| `0x8016A180` | `fn_8016a180` | 16 | high |
| `0x8016A190` | `fn_8016a190` | 16 | high |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs`.
