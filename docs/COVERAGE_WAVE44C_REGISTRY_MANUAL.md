# Coverage - Wave 44c (resource::registry virtual-dispatch batch)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 4 | 84 | 0 | 4 |

## What this pass covers

- Virtual-dispatch wrappers: three trampolines that load the object at
  `this+4`, follow its vtable and tail-call slot `+0xC`, `+0x20` or `+0x24`
  (`0x801636A0`, `0x8016A0F0`, `0x8016A110`).
- SDA-rooted virtual dispatch (`0x8016F410`): reads the `r13-0x5220` global,
  follows its `+0x10` field to a vtable and calls slot `+0xC`.

All four are medium confidence: the indirection chains and slot offsets are
literal in the exports, but the object/vtable types are inferred.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801636A0` | `fn_801636a0` | 24 | medium |
| `0x8016A0F0` | `fn_8016a0f0` | 20 | medium |
| `0x8016A110` | `fn_8016a110` | 20 | medium |
| `0x8016F410` | `fn_8016f410` | 20 | medium |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs`.
