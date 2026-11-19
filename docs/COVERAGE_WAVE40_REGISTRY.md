# Coverage - Wave 40 (resource::registry, 0x80160000 bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/registry.rs` | `resource::registry` | 27 | 2156 | 17 | 10 |

## What this pass covers

- Sound-bank allocation and pending-queue helpers (`0x801628C0`-`0x801629B0`,
  4 bodies): the zeroed default-heap allocator, its matching free, the update
  pass that appends a finished bank to the pending list, and the first-bank
  scan for a payload whose `+0x0A` half-word is zero.
- Bank record and keyed-list layer (`0x80162A50`-`0x80162ED0`, 11 bodies):
  the four-argument stash into SDA, ordered insert by `u16` key, bucket-list
  free, descriptor-word bucket find, unlink and teardown-mask clear, the
  `id & 7` hash-bucket lookup, the `u16`/`u8` payload finds, the five-list
  reset with hash init, and the list free walk.
- Lookup and LZ-view layer (`0x80163000`-`0x801634C0`, 7 bodies): the
  `u16`/`u8` payload-key walks, the `ut::List` reset at `0x80377AB0`, the LZ
  view builder that reuses wave-39 `0x8016A5E0`/`0x8016A690`, the virtual
  pass over the same list, and the LZ-view read of `r13-0x52C0`.
- Element accessor set (`0x80168060`-`0x801680E0`, 5 bodies): the `0x38`-stride
  element class tag plus the four vtable-slot accessors at `+0x1C`, `+0x20`,
  `+0x28` and `+0x2C`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801628C0` | `fn_801628c0` | 92 | high |
| `0x80162920` | `fn_80162920` | 12 | high |
| `0x80162930` | `fn_80162930` | 120 | medium |
| `0x801629B0` | `fn_801629b0` | 112 | medium |
| `0x80162A50` | `fn_80162a50` | 20 | high |
| `0x80162B40` | `fn_80162b40` | 92 | medium |
| `0x80162BB0` | `fn_80162bb0` | 116 | high |
| `0x80162C30` | `fn_80162c30` | 44 | high |
| `0x80162CA0` | `fn_80162ca0` | 72 | high |
| `0x80162CF0` | `fn_80162cf0` | 16 | high |
| `0x80162D00` | `fn_80162d00` | 72 | medium |
| `0x80162D50` | `fn_80162d50` | 84 | high |
| `0x80162DB0` | `fn_80162db0` | 84 | high |
| `0x80162E10` | `fn_80162e10` | 192 | high |
| `0x80162ED0` | `fn_80162ed0` | 288 | high |
| `0x80163000` | `fn_80163000` | 132 | high |
| `0x80163090` | `fn_80163090` | 104 | high |
| `0x80163100` | `fn_80163100` | 104 | high |
| `0x80163170` | `fn_80163170` | 16 | high |
| `0x80163180` | `fn_80163180` | 148 | medium |
| `0x80163280` | `fn_80163280` | 120 | medium |
| `0x801634C0` | `fn_801634c0` | 8 | high |
| `0x80168060` | `fn_80168060` | 28 | high |
| `0x80168080` | `fn_80168080` | 20 | medium |
| `0x801680A0` | `fn_801680a0` | 20 | medium |
| `0x801680C0` | `fn_801680c0` | 20 | medium |
| `0x801680E0` | `fn_801680e0` | 20 | medium |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/resource/registry.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/resource/registry.rs`.

## Note

The lane process died on the free-model quota after landing the bodies, CSV and
module header, but before writing this coverage doc. The doc was reconstructed
from the validated CSV and `functions.tsv` by the coordinator; no function was
added or reclassified.
