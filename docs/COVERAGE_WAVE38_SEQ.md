# Coverage - Wave 38 (audio::seq, pass 2)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 30 | 4888 | 15 | 15 |

Bytes, High and Medium counts cover only the 30 bodies added in this pass.

## What this pass covers

- Control-object lifecycle, `0x80271000..0x80271BB0`: the destructor obtained
  from `0x802DCD74` (`fn_80271000`), the bank-driven rebuild
  (`fn_802712b0`), the three one-line field setters
  (`fn_802718e0`, `fn_802718f0`, `fn_80271900`), and the per-frame advance
  (`fn_80271bb0`).
- Player wrappers around the embedded control object, `0x802724E0..0x80272A40`:
  restart (`fn_802724e0`), start/playing (`fn_80272560`), callback
  registration (`fn_802725d0`, `fn_802726d0`), the three thin forwarders
  (`fn_802728a0`, `fn_802728b0`, `fn_802728c0`), the track-value accessors
  (`fn_802729c0`, `fn_80272a10`) and the enabled query (`fn_80272a40`).
- Track-object construction and node-list operations, `0x80272C40..0x80273DB0`:
  holder linking (`fn_80272c40`, `fn_80272d40`), slot store (`fn_80272d80`),
  construction and defaults (`fn_80272d90`, `fn_80272f40`), teardown
  (`fn_80272e30`), and the per-track list operations (`fn_802731d0`,
  `fn_802732a0`, `fn_80273430`, `fn_802734c0`, `fn_80273ae0`, `fn_80273b50`,
  `fn_80273c40`) plus the value-address helper (`fn_80273db0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80271000` | `fn_80271000` | 392 | medium |
| `0x802712B0` | `fn_802712b0` | 660 | medium |
| `0x802718E0` | `fn_802718e0` | 8 | high |
| `0x802718F0` | `fn_802718f0` | 8 | high |
| `0x80271900` | `fn_80271900` | 12 | high |
| `0x80271BB0` | `fn_80271bb0` | 300 | medium |
| `0x802724E0` | `fn_802724e0` | 124 | medium |
| `0x80272560` | `fn_80272560` | 104 | medium |
| `0x802725D0` | `fn_802725d0` | 244 | medium |
| `0x802726D0` | `fn_802726d0` | 116 | medium |
| `0x802728A0` | `fn_802728a0` | 8 | medium |
| `0x802728B0` | `fn_802728b0` | 8 | medium |
| `0x802728C0` | `fn_802728c0` | 8 | medium |
| `0x802729C0` | `fn_802729c0` | 80 | medium |
| `0x80272A10` | `fn_80272a10` | 44 | high |
| `0x80272A40` | `fn_80272a40` | 28 | high |
| `0x80272C40` | `fn_80272c40` | 244 | medium |
| `0x80272D40` | `fn_80272d40` | 56 | high |
| `0x80272D80` | `fn_80272d80` | 8 | high |
| `0x80272D90` | `fn_80272d90` | 160 | high |
| `0x80272E30` | `fn_80272e30` | 260 | high |
| `0x80272F40` | `fn_80272f40` | 396 | high |
| `0x802731D0` | `fn_802731d0` | 208 | medium |
| `0x802732A0` | `fn_802732a0` | 396 | medium |
| `0x80273430` | `fn_80273430` | 144 | high |
| `0x802734C0` | `fn_802734c0` | 148 | medium |
| `0x80273AE0` | `fn_80273ae0` | 108 | high |
| `0x80273B50` | `fn_80273b50` | 228 | high |
| `0x80273C40` | `fn_80273c40` | 356 | high |
| `0x80273DB0` | `fn_80273db0` | 32 | high |

## Deliberate skips

- `0x80271E30` and `0x802720A0` are sequenced-audio neighbours but their
  Ghidra output is dominated by paired-single (`ldexpf` / `unaff_GQR0`)
  arithmetic that cannot be reconstructed honestly; both are skipped per the
  lane instructions.
- `0x802700F0`, `0x80270100`, `0x80270110`, `0x80270310` touch the
  `+0x0C/+0x14` range object owned by the fellow `game::player` lane and
  are not sequenced-audio code; out of scope.
- `0x80270AD0` and `0x80270C20` initialise and dispatch the four alarm slots
  at `0x803FC990`; they belong to the `game::player` side of the bucket
  and are not translated here.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/audio/seq.rs`.
