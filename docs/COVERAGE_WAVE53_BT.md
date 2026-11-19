# Coverage - Wave 53 (`runtime::bt`, `0x80200000` bucket)

## How to read this file

Every row below is one `pub unsafe fn fn_<address>` body in
`src/runtime/bt.rs`. The address is the unrelocated `main.dol` virtual address
(image base `0x80004000`); the byte count is taken from
`work/ghidra/main-gb/export/functions.tsv`; the evidence column names the
Ghidra C export. None of these addresses was previously present in
`ledger/functions.csv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/bt.rs` | `runtime::bt` | 25 | 3368 | 23 | 2 |

## What this pass covers

- **Link-control packet builders** (`0x8020CC2C`-`0x8020D970`, 15 bodies):
  the six `0x8020CC2C..0x8020D048` builders (commands `0x0401`, `0x0402`,
  `0x0406`, `0x0407`, `0x0409`, `0x040A`) and the nine
  `0x8020D4C8..0x8020D970` builders (commands `0x040E`, `0x040F`, `0x0411`,
  `0x0413`, `0x0419`, `0x041A`, `0x041B`, `0x041D`, `0x041F`). Thirteen
  allocate their own `2`-class buffer through `0x801F7150`; two
  (`0x8020CFE8`, `0x8020D048`) fill a caller-supplied buffer. All of them end
  in [`fn_8020ac78`], so the packet layout (length at `+2`, header offset at
  `+4`, command word at `+8`, parameter length at `+10`) is the one that
  routine consumes. Address parameters are copied in reverse byte order, the
  convention already used by the `0x8020E*` builders.
- **HID-host bring-up** (`0x8020B91C`-`0x8020BAD4`, 5 bodies): the four-call
  initialiser chain, the `0x84`-byte control-block clear with its `0x2A4`
  maximum-ACL value, the channel-open routine over
  `0x8021F5C0`/`0x8021F260`/`0x802202D0`/`0x8021FF38`, the `0x3AC`-byte block
  clear with its two data pointers and count `5`, and the ten-entry callback
  table installer at `0x803D43C4`.
- **Repage / originate state** (`0x8020F348`-`0x8020F8B4`, 5 bodies): the
  `0x144`-byte repage block initialiser (installs [`fn_8020f470`]), the
  originate attempt on PSM `0x11`, the repage timeout handler with its
  `0x16`-tick timer re-arm and indirect callback, the ten-byte address copy,
  and the single-shot SDP registration guarded by `0x803D4BB8`.

The command mnemonics are deliberately not named: no string in these exports
identifies them, so the bodies document the literal command word only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8020B91C` | `fn_8020b91c` | 44 | high |
| `0x8020B948` | `fn_8020b948` | 76 | high |
| `0x8020B994` | `fn_8020b994` | 224 | medium |
| `0x8020BA74` | `fn_8020ba74` | 96 | high |
| `0x8020BAD4` | `fn_8020bad4` | 112 | high |
| `0x8020CC2C` | `fn_8020cc2c` | 168 | high |
| `0x8020CCD4` | `fn_8020ccd4` | 96 | high |
| `0x8020CED4` | `fn_8020ced4` | 136 | high |
| `0x8020CF5C` | `fn_8020cf5c` | 140 | high |
| `0x8020CFE8` | `fn_8020cfe8` | 96 | high |
| `0x8020D048` | `fn_8020d048` | 92 | high |
| `0x8020D4C8` | `fn_8020d4c8` | 160 | high |
| `0x8020D568` | `fn_8020d568` | 144 | high |
| `0x8020D5F8` | `fn_8020d5f8` | 124 | high |
| `0x8020D674` | `fn_8020d674` | 140 | high |
| `0x8020D700` | `fn_8020d700` | 216 | high |
| `0x8020D7D8` | `fn_8020d7d8` | 160 | high |
| `0x8020D878` | `fn_8020d878` | 124 | high |
| `0x8020D8F4` | `fn_8020d8f4` | 124 | high |
| `0x8020D970` | `fn_8020d970` | 124 | high |
| `0x8020F348` | `fn_8020f348` | 104 | high |
| `0x8020F3B0` | `fn_8020f3b0` | 192 | high |
| `0x8020F470` | `fn_8020f470` | 200 | high |
| `0x8020F868` | `fn_8020f868` | 76 | high |
| `0x8020F8B4` | `fn_8020f8b4` | 200 | medium |

## Deliberate skips

These are in the same bucket and were read, but cannot be reconstructed
honestly from the export:

- `0x8020CD34`, `0x8020CDF4` - companions of the `0x8020CC2C..0x8020D048`
  builders. Both consume the result of `0x802DCD74` / `0x802DCD70`, which
  returns the address and a small tag split across `r3`/`r4`; Ghidra prints the
  second half as `extraout_r4`. That value is also written into the packet, so
  omitting or guessing it would silently change the payload.
- `0x8020D9EC`, `0x8020DAE8` - the two largest builders of the `0x8020D9xx`
  block. They call `0x802DCD6C` (a 64-bit lock/tick) and write into the
  returned pointer without a null check; Ghidra represents the result as
  `undefined6`/`undefined8` and the packet bytes are slices of that value.
- `0x8020F538`, `0x8020F710`, `0x8020F97C` - the 472-, 344- and 1104-byte
  companions of the repage block. They are real targets but were left for a
  later pass so this lane stays within the 25-30 body budget; they are still
  untranslated and are called by address from the bodies above.

The rest of the `0x80200000` bucket (large HCI/L2CAP workers in
`0x80201320..0x8020B8FC`, `0x8020E0B0..0x8020E39C` and `0x8020FDCC..0x8020FF70`)
was out of scope for a 25-30 body lane and is untouched.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn` body in `src/runtime/bt.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no errors; the only new warning
  was resolved before the final check.
- `node tools/check-lane-csv.mjs wave53` reports `OK` for `wave53-bt.csv`
  (25 rows, 25 unique addresses).
