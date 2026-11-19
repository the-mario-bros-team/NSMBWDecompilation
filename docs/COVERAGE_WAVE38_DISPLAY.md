# Coverage - Wave 38 (engine::display)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 30 | 4044 | 21 | 9 |

## What this pass covers

The bucket `0x800D0000` holds 251 functions, 190 of them still
untranslated. Sampling showed three kinds of content: large
actor bodies (the `+0x154`/`+0x770` object at `0x800D36E0..0x800D5D10`),
paired-single/GQR bodies that this pass must skip, and the engine
band's small-object helpers. This pass takes one coherent cluster
that shares two objects:

- **Pad-button triples** (`0x800DBCC0..0x800DBE80`, 6 bodies). A
  three-byte latch (blocked `+0`, held `+1`, hold counter `+2`) for
  each of four pads. `0x800DBCC0`/`0x800DBD10` block/release the
  button through `0x800B3600`/`0x800B3750`, `0x800DBD60` clears
  one latch, `0x800DBD80` applies the two-held-frame rule through
  `0x800B3720`, and `0x800DBE30`/`0x800DBE80` run the four latches
  at 3-byte strides.
- **The four-slot menu state container** (`0x800DBF50..0x800DCDD0`,
  20 bodies). [`fn_800dbf50`] builds four `0x98`-byte slots with
  [`fn_800dc570`] and publishes the container in the global at
  `r13 - 0x5658` (already [`G_PLAYER_SLOTS`]). Each slot owns two
  state managers (descriptors `0x803718B0` and `0x80371830`) and
  the ready byte at `+0x54`. The route helpers dispatch on the
  route byte at view `+0x74` against the manager table at `+0xE2C`,
  requesting descriptors `0x803718F0`/`0x80371930`/`0x80371970`
  on the `+0x0C` interface.
- **Earlier helpers of the same band** (`0x800DB450..0x800DB620`,
  4 bodies). A vtable/key object constructor, the two point
  orderings that feed the bounding-box fields at `+0x4C..+0x58`,
  and the box/heading derivation with its `0x801611C0` angle.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800DB450` | `fn_800db450` | 68 | medium |
| `0x800DB590` | `fn_800db590` | 88 | high |
| `0x800DB5F0` | `fn_800db5f0` | 48 | high |
| `0x800DB620` | `fn_800db620` | 204 | medium |
| `0x800DBCC0` | `fn_800dbcc0` | 72 | high |
| `0x800DBD10` | `fn_800dbd10` | 76 | high |
| `0x800DBD60` | `fn_800dbd60` | 20 | high |
| `0x800DBD80` | `fn_800dbd80` | 176 | medium |
| `0x800DBE30` | `fn_800dbe30` | 72 | high |
| `0x800DBE80` | `fn_800dbe80` | 96 | high |
| `0x800DBF50` | `fn_800dbf50` | 144 | high |
| `0x800DBFE0` | `fn_800dbfe0` | 172 | high |
| `0x800DC120` | `fn_800dc120` | 100 | high |
| `0x800DC1F0` | `fn_800dc1f0` | 64 | high |
| `0x800DC230` | `fn_800dc230` | 64 | high |
| `0x800DC270` | `fn_800dc270` | 108 | high |
| `0x800DC3B0` | `fn_800dc3b0` | 64 | high |
| `0x800DC3F0` | `fn_800dc3f0` | 64 | high |
| `0x800DC430` | `fn_800dc430` | 72 | medium |
| `0x800DC480` | `fn_800dc480` | 212 | medium |
| `0x800DC570` | `fn_800dc570` | 336 | high |
| `0x800DC6F0` | `fn_800dc6f0` | 272 | medium |
| `0x800DC820` | `fn_800dc820` | 120 | medium |
| `0x800DC8A0` | `fn_800dc8a0` | 60 | medium |
| `0x800DC8E0` | `fn_800dc8e0` | 140 | high |
| `0x800DC9A0` | `fn_800dc9a0` | 84 | high |
| `0x800DCA20` | `fn_800dca20` | 280 | medium |
| `0x800DCB70` | `fn_800dcb70` | 260 | high |
| `0x800DCCA0` | `fn_800dcca0` | 260 | high |
| `0x800DCDD0` | `fn_800dcdd0` | 248 | high |

## Deliberate skips

- **Paired-single/GQR bodies** anywhere in the bucket that
  needed `ldexpf`/`unaff_GQR0` reconstruction were skipped, e.g.
  `0x800D0610`, `0x800D2460`, `0x800D2690`, `0x800D27B0`,
  `0x800D3380`, `0x800D5170`, `0x800D57F0`, `0x800D5DD0`,
  `0x800D9D70`. Their float semantics cannot be recovered
  honestly from the export.
- **Unrecovered jumptables** were not translated as bodies. The
  small helpers behind them (`0x800DC970`, `0x800DCEE0`,
  `0x800DCF00`, 12–16 bytes) are called by address from
  [`fn_800dc480`], [`fn_800dc8a0`] and [`fn_800dc8e0`] instead,
  and the `0x800DCED0` 4-byte empty body is called by address
  from [`fn_800dc820`].
- The container state managers at `+0x2C`/`+0x74` are built by
  the already-translated `0x8015FC90`; the manager fields
  `+0xE24/+0xE28/+0xE2C` belong to the manager class at
  `r13 - 0x5220` and are used by address only.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/engine/display.rs`
  (the remaining errors are in `src/resource/registry.rs`, which
  this lane does not own).
