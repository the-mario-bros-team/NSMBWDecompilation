# Coverage - Wave 115 (`runtime::bt`, the HID device-database dump)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count was re-read from
`work/ghidra/main-gb/export/functions.tsv` for this pass; the `evidence` column
of `ledger/agents/wave115-bt.csv` names the exact Ghidra C export.

`// observed:`, `// inferred:` and `// unknown:` markers follow
`docs/RUST_STYLE.md`. Field roles the evidence does not prove are marked rather
than named. This pass touches the single entry that the `0x80200000` bucket had
left open; the other 255 addresses in that bucket already have bodies in this
file from waves 5-104 and are not re-claimed here.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 1 | 224 | 1 | 0 | 0 |

## What this pass covers

The `0x80200000` bucket holds 256 functions. When this pass started exactly one
was untranslated: `0x80200190`, recorded as a deliberate skip by wave 99
because its export appeared to index the device table with two incompatible
strides. That objection does not hold:

- `(&DAT_803d1966)[i * 0x10]` is a `u16` load, so index `i * 0x10` of a `u16`
  array is byte offset `i * 0x20`;
- `(&DAT_803d1970)[i * 0x20]` and its neighbours are byte loads at the same
  `i * 0x20` offset.

Both forms therefore describe one `0x20`-byte record, sixteen of them from
`0x803D1966` to `0x803D1B66`.

### Device-database dump (`0x80200190`, 224 bytes)

The routine is read-only and entirely diagnostic. It:

1. prints the header `bta_hh_trace_dev_db:: Device DB list********************`
   when the HID-host trace level at `0x8042AB98` (`r13 - 0x4AE8`, already
   [`BTA_HH_LOG_LEVEL`] in this file) is above `4`;
2. walks records `0` through `0xF`, and for each record prints
   - `kdev[%d] in_use[%d]  handle[%d]` from the index, the `+0x0A` flag and the
     `+0x08` handle byte,
   - `attr_mask[%04x] state [%d] sub_class[%02x] index = %d` from the `+0x00`
     `u16`, the `+0x0E`, `+0x05` and `+0x04` bytes;
3. prints the closing line of forty-one `*` characters.

The guard is re-read before every line, which is what the export does; the
body keeps that order and the do/while increment-before-test shape.

The field roles are not guesses: `work/dol/strings.txt` holds the three full
format strings (file offsets `0x3442A0` and `0x3442C7`), the initialiser
`0x801FDE58` writes the record index to `+4` and `0xFF` to the handle at `+8`,
and the readers `0x801FF024`/`0x801FFEC4` test `+0x0A` as in-use and `+0x0E` as
state. The handle is the one field the dump reads narrower than the protocol's
usual `u16`; the export loads a single byte there and the body reproduces that
byte load.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80200190` | `fn_80200190` | 224 | high |

## Deliberate skips

None in this bucket. The only remaining documented skip in `src/runtime/bt.rs`
is the `0x80220000` EXI block worker `0x80228644`, which belongs to the
`0x80220000` bucket and is unchanged by this pass
(see `docs/COVERAGE_WAVE113_BT.md`).

## Verification

- Every address and byte size matches `functions.tsv` (`80200190`, 224 bytes).
- The evidence file exists at
  `work/ghidra/main-gb/export/80200190_FUN_80200190.c`.
- The row has exactly one `pub unsafe fn fn_80200190` body in
  `src/runtime/bt.rs`.
- `0x80200190` was not in `ledger/functions.csv` before this pass and is not
  claimed by any other Wave 115 lane.
- `cargo check --message-format=short` reports no error in `src/runtime/bt.rs`.
- `node tools/check-lane-csv.mjs wave115` reports this lane clean.
