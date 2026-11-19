# Coverage - Wave 36 (runtime::dvd pass 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 30 | 6472 | 24 | 5 |

## What this pass covers

- Heap/tag allocation core (`0x801D3880`-`0x801D5310`): the three-level
  range search over the heap root list, the free-list split/merge helpers,
  first-fit / best-fit / last-fit allocators, resize/free wrappers, and
  the heap create / destroy / pop / trim entry points.
- All 30 share one heap descriptor: `+0x18`/`+0x1C` bounds, `+0x20`
  mutex, `+0x38` flags, `+0x3C`/`+0x40` free lists, `+0x44` save stack,
  `+0x4C`/`+0x4E` state, with `FR` (`0x4652`) / `DU` (`0x5544`) tags and
  the `EXPH` / `FRMH` / `UNTH` create tags.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D3880` | `fn_801d3880` | 320 | high |
| `0x801D39C0` | `fn_801d39c0` | 448 | medium |
| `0x801D3B80` | `fn_801d3b80` | 364 | medium |
| `0x801D3CF0` | `fn_801d3cf0` | 320 | high |
| `0x801D3E30` | `fn_801d3e30` | 556 | medium |
| `0x801D4060` | `fn_801d4060` | 220 | high |
| `0x801D4140` | `fn_801d4140` | 200 | high |
| `0x801D4210` | `fn_801d4210` | 360 | high |
| `0x801D4380` | `fn_801d4380` | 176 | high |
| `0x801D4430` | `fn_801d4430` | 48 | high |
| `0x801D4460` | `fn_801d4460` | 176 | high |
| `0x801D4510` | `fn_801d4510` | 508 | medium |
| `0x801D4710` | `fn_801d4710` | 196 | high |
| `0x801D4860` | `fn_801d4860` | 216 | high |
| `0x801D4940` | `fn_801d4940` | 88 | high |
| `0x801D49A0` | `fn_801d49a0` | 80 | high |
| `0x801D49F0` | `fn_801d49f0` | 156 | high |
| `0x801D4A90` | `fn_801d4a90` | 8 | high |
| `0x801D4AA0` | `fn_801d4aa0` | 216 | high |
| `0x801D4B80` | `fn_801d4b80` | 132 | high |
| `0x801D4C10` | `fn_801d4c10` | 48 | high |
| `0x801D4C40` | `fn_801d4c40` | 288 | high |
| `0x801D4D60` | `fn_801d4d60` | 160 | high |
| `0x801D4E00` | `fn_801d4e00` | 120 | high |
| `0x801D4E80` | `fn_801d4e80` | 212 | low |
| `0x801D4F60` | `fn_801d4f60` | 180 | high |
| `0x801D5020` | `fn_801d5020` | 132 | high |
| `0x801D5180` | `fn_801d5180` | 348 | medium |
| `0x801D52E0` | `fn_801d52e0` | 48 | high |
| `0x801D5310` | `fn_801d5310` | 148 | high |

## Deliberate skips

- `0x801D3870` (8 bytes, returns `0`): the `fn_801cfa30` predicate. It is a
  one-line constant stub already referenced as `DVD_QUEUE_NODE_PREDICATE`
  in `dvd.rs`; it belongs to the queue band, not the heap cluster, and was
  left untranslated to keep this pass at exactly 30 heap bodies sharing one
  tag object.
- Early band `0x801C0020`-`0x801C9E20` (SI init and related hardware
  code): not DVD/NAND and not the coherent cluster briefed for this lane;
  left untouched.
- Heap neighbours `0x801D5420`-`0x801D5990` and drive-probe steps
  `0x801D5DC0`-`0x801D6F60` (NAND thread helpers and IOS identity reads):
  outside the single-cluster scope; the `0x801D4E80` heap argument has no
  visible caller in the export and is marked `low` for that reason.
- Input lane territory (`0x801DDAC0` onward): nothing clearly input was
  touched.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/runtime/dvd.rs`.
