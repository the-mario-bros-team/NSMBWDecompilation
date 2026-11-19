# Coverage - Wave 42 (runtime::bt, 0x80220000 bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 18 | 1452 | 12 | 6 |

## What this pass covers

- Doubly-linked slot ring and header accessors (`0x80228FF0`-`0x802291C0`,
  8 bodies): the direct/table-indirect slot readers, the ring init/insert/
  unlink helpers, the running-count accessor and the `u16` index link used to
  resolve a slot through the second table.
- Byte-array "range" object and its two embedding classes
  (`0x80229D60`-`0x8022A450`, 10 bodies): the `{ max, cursor }` range
  constructor and advance/query helpers, the 0x70-byte owner's array setup and
  destructor, the 0x16C-byte owner's array accessors, and the two small
  release/destructor tails.
- These clusters are **not** Bluetooth protocol code; they share the address
  lane after the RFCOMM block. The module header records the classification
  and marks unproven field roles as `// unknown:`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80228FF0` | `fn_80228ff0` | 32 | high |
| `0x80229010` | `fn_80229010` | 28 | high |
| `0x80229030` | `fn_80229030` | 132 | high |
| `0x802290C0` | `fn_802290c0` | 72 | high |
| `0x80229110` | `fn_80229110` | 68 | high |
| `0x80229160` | `fn_80229160` | 44 | high |
| `0x80229190` | `fn_80229190` | 48 | medium |
| `0x802291C0` | `fn_802291c0` | 116 | medium |
| `0x80229D60` | `fn_80229d60` | 100 | high |
| `0x80229DD0` | `fn_80229dd0` | 80 | high |
| `0x80229E20` | `fn_80229e20` | 136 | medium |
| `0x80229F30` | `fn_80229f30` | 184 | medium |
| `0x8022A120` | `fn_8022a120` | 120 | medium |
| `0x8022A1A0` | `fn_8022a1a0` | 80 | high |
| `0x8022A1F0` | `fn_8022a1f0` | 136 | medium |
| `0x8022A380` | `fn_8022a380` | 60 | high |
| `0x8022A440` | `fn_8022a440` | 8 | high |
| `0x8022A450` | `fn_8022a450` | 8 | high |

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/runtime/bt.rs`.

## Note

The Wave 42 bt lane landed these 18 bodies and its module-header section, then
died on `Free usage limit reached` before writing the CSV and this doc. The
coordinator recovered the bodies with `tools/audit-unledgered.mjs --csv` and
reconstructed this doc from the validated CSV and `functions.tsv`; no function
was added or reclassified. The remaining Wave 42 bt clusters
(`0x80229360..0x80229820` and `0x8022A9F0..0x8022B380`) were not started.
