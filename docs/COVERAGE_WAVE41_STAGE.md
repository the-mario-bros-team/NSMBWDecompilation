# Coverage - Wave 41 (`game::stage`, dLineMng line-type handler pairs)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. All fourteen
pairs share the line-object layout documented in
[`docs/COVERAGE_WAVE38_STAGE.md`](COVERAGE_WAVE38_STAGE.md): `+0x38`/`+0x3C`
per-frame delta, `+0x40`/`+0x44` running point, `+0x50`/`+0x54` stored anchor,
`+0x60` speed, `+100` the `i16` angle and `+0x6C` the state interface. Field
roles are *inferred* from the wave-38 bodies and from the stores below.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 28 | 5812 | 20 | 8 |

## What this pass covers

Wave 38 closed the dLineMng membership tables, the circle handlers, the angle
blends and the four state machines/dispatchers (`0x800C1700..0x800C4E50`).
This pass continues into the **line-type handler pairs** at
`0x800C55E0..0x800C6C80`: fourteen `enter`/`update` pairs, one pair per sloped
or straight line family.

- **enter halves** (`0x800C55E0`, `0x800C5740`, `0x800C5890`, `0x800C59C0`,
  `0x800C5AF0`, `0x800C5C30`, `0x800C5D70`, `0x800C5F80`, `0x800C6190`,
  `0x800C63A0`, `0x800C65B0`, `0x800C67D0`, `0x800C69F0`, `0x800C6C10`) -
  clamp one axis with [`fn_800c3ad0`] / [`fn_800c3b10`], write the family
  angle (`0xE000`, `0xA000`, `0xC000`, `0xD333`, `0xACCC`, `0xECCC`,
  `0x9333`) and seed the other axis from the anchor plus a `-0x6800` /
  `-0x6818` half-extent, optionally folded through the `f64` slope at
  `-0x67C0` and the extra `f64` offset at `-0x67B0`.
- **update halves** (`0x800C5650`, `0x800C57A0`, `0x800C58F0`,
  `0x800C5A20`, `0x800C5B50`, `0x800C5C90`, `0x800C5DF0`, `0x800C6000`,
  `0x800C6200`, `0x800C6410`, `0x800C6630`, `0x800C6850`, `0x800C6A70`,
  `0x800C6C80`) - step the running point by `+0x60` scaled with
  `-0x67A8`/`-0x67B8`/`-0x67C8`, shear the delta by `-0x6804` (a few also
  negate `+0x38`/`+0x3C`), then branch three ways: restore the saved
  `+0x40`/`+0x44` words when the contact query `0x800C18B0` fires, retry the
  wave-38 dispatchers [`fn_800c4b50`]/[`fn_800c4c50`]/[`fn_800c4d50`]/
  [`fn_800c4e50`] at the anchor, or look the border point up with
  [`fn_800c17a0`] and transition to the matching `0x803590xx`/`0x803591xx`
  state record.
- **contact query kept by address** - `0x800C18B0` is called through
  `platform::call1` in [`w41_line_contact`]. Its export uses
  `unaff_GQR0`/`ldexpf` paired-single loads, so wave 38 already skipped it;
  translating it here would have meant inventing the paired-single path.

Deliberately skipped:

- `0x800C4F50` and `0x800C51A0` - the two `ldexpf`/`unaff_GQR0`/`unaff_GQR3`
  carriers that precede this run. Their paired-single blocks are the reason
  the run starts at `0x800C55E0`. `0x800C4F50` is the slope integrator the
  sheared pairs conceptually replace.
- `0x800C18B0` - kept as an address call, see above.
- The rest of the `0x800C0000` bucket is outside this cluster: the layout
  library (`0x800C8CB0..0x800C9DF0`), the dMaskMng manager
  (`0x800CBF60..0x800CD770`) and the stage-info bit/flag accessors
  (`0x800CE190..0x800CEB80`) are separate systems and were left for a later
  pass. The small `0x800C6E20..0x800C71A0` wrappers were also left; they are
  `f32`-pair stubs into `0x800C3A10` and add no new field knowledge.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C55E0` | `fn_800c55e0` | 88 | high |
| `0x800C5650` | `fn_800c5650` | 240 | high |
| `0x800C5740` | `fn_800c5740` | 80 | high |
| `0x800C57A0` | `fn_800c57a0` | 240 | high |
| `0x800C5890` | `fn_800c5890` | 72 | high |
| `0x800C58F0` | `fn_800c58f0` | 204 | high |
| `0x800C59C0` | `fn_800c59c0` | 68 | high |
| `0x800C5A20` | `fn_800c5a20` | 204 | high |
| `0x800C5AF0` | `fn_800c5af0` | 68 | high |
| `0x800C5B50` | `fn_800c5b50` | 216 | high |
| `0x800C5C30` | `fn_800c5c30` | 72 | high |
| `0x800C5C90` | `fn_800c5c90` | 216 | high |
| `0x800C5D70` | `fn_800c5d70` | 100 | high |
| `0x800C5DF0` | `fn_800c5df0` | 396 | medium |
| `0x800C5F80` | `fn_800c5f80` | 100 | high |
| `0x800C6000` | `fn_800c6000` | 400 | medium |
| `0x800C6190` | `fn_800c6190` | 92 | high |
| `0x800C6200` | `fn_800c6200` | 412 | medium |
| `0x800C63A0` | `fn_800c63a0` | 96 | high |
| `0x800C6410` | `fn_800c6410` | 416 | medium |
| `0x800C65B0` | `fn_800c65b0` | 100 | high |
| `0x800C6630` | `fn_800c6630` | 404 | medium |
| `0x800C67D0` | `fn_800c67d0` | 104 | high |
| `0x800C6850` | `fn_800c6850` | 408 | medium |
| `0x800C69F0` | `fn_800c69f0` | 104 | high |
| `0x800C6A70` | `fn_800c6a70` | 416 | medium |
| `0x800C6C10` | `fn_800c6c10` | 92 | high |
| `0x800C6C80` | `fn_800c6c80` | 404 | medium |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave41` reports `OK`.
- `cargo check` is free of errors for `src/game/stage.rs`.

## Known limits

- The eight `medium` bodies all call `FUN_800c17a0`; the C export prints the
  call with **no operands** (the double arguments were dropped). The bodies
  pass the border point that the surrounding stores use, marked with an
  `// inferred:` comment at every call site.
- The `-0x67A8`/`-0x67B8`/`-0x67C8` scale values and the `-0x67B0`/`-0x67C0`
  doubles are small-data constants whose numeric values are not present in the
  C export; the bodies read them through `SDA2_BASE` like the wave-38 code
  does.
