# Coverage - Wave 96 (`game::worldmap`, `0x80120000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); every `W96_R2_*` constant below is read at run time from the
absolute address recorded in its comment, and its value was decoded from
`work/dol/main.elf`. The export never names the classes involved, so the bodies
keep literal offsets and the groups are labelled by address.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 29 | 6,584 | 14 | 15 |

Every row is `translated`; no body is `stub` or `partial`.

## What this pass covers

- **`0x80125210..0x801252A0` - the four position-publish records.** Each of the
  four 44/48-byte entries copies the actor's `+0x3A0` triple into a private
  record (`+0x464`, `+0x4B8`, `+0x4D4`, `+0x4F0`), stores the float argument
  at the record's fourth word and dispatches that record's `+0x0C` virtual
  slot. `0x801252A0` carries a second float at `+0x500`.
- **`0x80126970..0x8012A800` - the held-item and throw handlers.** The group
  owns the `+0x14A0`/`+0x14A4` state pair, the `+0x1564` pose selector, the
  `+0x2A64` model/stream controller and the `+0xEA4` sub-controller. It adds
  the held-item entry `0x80126970` (stream event `0x4B`, four action clears,
  the `0x802F5740`/`-6.0f` split), the movement-mode rebuild `0x80127FC0`
  (action `0x0C`/`0x0E`/`0x47`/`0x48` ladder, `0x80127600`), the reset
  `0x801286D0` (stream event `0x1D`), the name-gated lift `0x80128EA0`
  (`0x803762F0` compare, render event `7`), the `0x801291F0` teardown,
  the throw entry `0x80129330` (`3.628f - 0.4f` / `- 0.7f` record into
  `0x80127FC0`), the coil updater `0x80129500` (countdown `0x15`/`0x0F`,
  `0x8015F480`), and the two small resets `0x8012A800`/`0x8012B3B0`.
- **`0x8012CCD0..0x8012F8E0` - the target/lock and coil handlers.** The group
  owns the `+0x27D4`/`+0x27D8` target cache, the `+0x27E0`/`+0x27E4` counters
  and the `+0x2A78` link id. It adds the movement-anchor refresh `0x8012CCD0`,
  the render-group effect `0x8012CF30` (`r13 - 0x6310` table), the direction
  selector `0x8012D110`, the movement re-filter `0x8012D5B0`, the target cache
  `0x8012E120`, the lock acquisition `0x8012E1F0`, the grab start/release pair
  `0x8012E400`/`0x8012E510`, the facing copy `0x8012E650`, the grab reaction
  `0x8012E820`, the three-state grab machine `0x8012EE20`, and the four
  `0x8012F0F0..0x8012F8E0` motion/coil updaters.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80125210` | `fn_80125210` | 44 | medium |
| `0x80125240` | `fn_80125240` | 44 | medium |
| `0x80125270` | `fn_80125270` | 44 | medium |
| `0x801252A0` | `fn_801252a0` | 48 | medium |
| `0x80126970` | `fn_80126970` | 264 | high |
| `0x80127FC0` | `fn_80127fc0` | 460 | medium |
| `0x801286D0` | `fn_801286d0` | 216 | high |
| `0x80128EA0` | `fn_80128ea0` | 276 | medium |
| `0x801291F0` | `fn_801291f0` | 36 | medium |
| `0x80129330` | `fn_80129330` | 456 | medium |
| `0x80129500` | `fn_80129500` | 652 | medium |
| `0x8012A800` | `fn_8012a800` | 200 | high |
| `0x8012B3B0` | `fn_8012b3b0` | 144 | high |
| `0x8012CCD0` | `fn_8012ccd0` | 356 | medium |
| `0x8012CF30` | `fn_8012cf30` | 64 | high |
| `0x8012D110` | `fn_8012d110` | 240 | high |
| `0x8012D5B0` | `fn_8012d5b0` | 104 | medium |
| `0x8012E120` | `fn_8012e120` | 20 | high |
| `0x8012E1F0` | `fn_8012e1f0` | 364 | medium |
| `0x8012E400` | `fn_8012e400` | 264 | high |
| `0x8012E510` | `fn_8012e510` | 144 | high |
| `0x8012E650` | `fn_8012e650` | 136 | high |
| `0x8012E820` | `fn_8012e820` | 276 | high |
| `0x8012EE20` | `fn_8012ee20` | 532 | medium |
| `0x8012F0F0` | `fn_8012f0f0` | 164 | high |
| `0x8012F340` | `fn_8012f340` | 284 | high |
| `0x8012F460` | `fn_8012f460` | 96 | high |
| `0x8012F4C0` | `fn_8012f4c0` | 524 | medium |
| `0x8012F8E0` | `fn_8012f8e0` | 132 | medium |

## Deliberate skips

- **`0x80127990`.** Its stream-event call drops both the receiver and the event
  id (`(double)...` arguments only), so the dispatch cannot be reconstructed
  honestly.
- **`0x80125D20`, `0x80125FA0`.** Both call `0x80125F10` with no argument, but
  that function reads `+0x392` from its first parameter; the lost actor cannot
  be recovered from the export.
- **`0x80125F70`.** The 40-byte body is an opaque unsigned bit trick
  (`(~u | 3) - (3 - u >> 1) >> 0x1F`); the source predicate cannot be named
  without guessing, so no body was written.
- **GQR/`ldexpf` records.** `0x801200C0`, `0x80120310`, `0x80120520`,
  `0x80120B40`, `0x80120D30`, `0x80121090`, `0x80125670`, `0x80125830`,
  `0x80126310`, `0x80127470`, `0x80128D60`, `0x8012B1D0`, `0x8012C640`,
  `0x8012E360`, `0x8012EA10`, `0x8012F6D0` and `0x8012FB10` carry
  `unaff_GQR0`/`ldexpf` reconstruction blocks, so the float constants are not
  recoverable from the C alone. `0x8012EA10` is still called by raw address
  from [`fn_8012ee20`], and `0x8012F6D0` from [`fn_8012f8e0`].
- **Lost float arguments.** `0x8012DE80` and `0x8012DFB0` show
  `FUN_80101AA0()` and `FUN_8012DE20()` with no arguments, so their float
  inputs cannot be reproduced.
- **The remaining `0x80120000` entries.** This pass selected the three
  contiguous clusters above. The rest - `0x801200C0..0x80121990`,
  `0x801252D0..0x80127B30`, `0x80128980..0x80129050`, `0x8012A210..0x8012B5B0`
  and the target/lock neighbours listed above - stay untranslated for a later
  pass. `0x801275B0`, `0x80127600` and `0x8012AB00` are deliberately left as
  raw-address callees because they are small helpers of other clusters.

## Verification

- Every address and byte size matches `functions.tsv`; the 29 sizes sum to
  6,584 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 29 were untranslated
  entries of the `0x80120000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs`. At the end of the pass the only remaining crate error
  was a parse error in `src/runtime/os.rs` (another parallel lane).
- `node tools/check-lane-csv.mjs wave96` reports `OK`.
