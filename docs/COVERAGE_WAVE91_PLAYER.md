# Coverage - Wave 91 (`game::player`, the `0x80050000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 29 | 3364 | 18 | 11 |

## How to read this file

This document covers the Wave-91 slice of the `0x80050000` bucket of
`work/ghidra/main-gb/export/functions.tsv`. Every body lives in
`src/game/player.rs`; the ledger rows are in
`ledger/agents/wave91-player.csv`. The module now holds four hundred twelve
translated functions.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - all rows are `translated`: the whole control flow and every call
  are present.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a callee
  argument had to be inferred.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. Every `r2 - k` / `r13 - k` below is resolved against
that base. The shared float block keeps the values earlier waves documented
(`0x8042B874` = `1.0f`, `0x8042B878` = `0.0f`, `0x8042B89C` = `1.5f`,
`0x8042B8A0` = `3.0f`); constants the export does not resolve stay named
`W91_*_8042B*` with an `// unknown:` note rather than being guessed.

## What this pass covers

- **Ride-momentum accumulator (`0x80053940..0x80053E60`, 6 bodies)** - the only
  new record layout in this pass, `+0x13F4..+0x1408`. [`fn_80053aa0`] clears
  it, [`fn_80053940`] fills it from a motion entry (`+0x50` horizontal, `+0x70`
  vertical) and the ridden actor's `+0x10C`, [`fn_80053ad0`] spends it through
  the `[0x8042B8DC, 3.0]` clamp and writes `+0x10C`, and
  [`fn_80053c80`]/[`fn_80053df0`]/[`fn_80053e60`] are the falling/landing/hit
  consumers. Field roles are `// inferred:`; the guards, stores and calls are
  literal.
- **Spawn-slot counter family (`0x8005FD20..0x8005FFB0`, 5 bodies)** - the
  follow-up wave 88 explicitly left open. [`fn_8005ffb0`] counts non-zero
  handles, [`fn_8005fef0`] counts the same handles whose `{0,1,3,2}` character
  still has lives, [`fn_8005fd20`] counts occupied slots whose actor is missing
  or not at state `1`, and [`fn_8005feb0`]/[`fn_8005fed0`] are the capped
  increment/decrement of the `0x80429CA0` counter.
- **Event-flag and timer leaves (`0x800564F0..0x80057860`, 5 bodies)** -
  [`fn_80056c30`] is the plain-store sibling of the OR-based
  [`fn_80056c70`]; [`fn_80056d50`] ticks eleven counters and raises flag `0xBC`;
  [`fn_80056900`], [`fn_80056980`] and [`fn_80057860`] are the
  falling/crouch/cursor leaves.
- **Record initialisers and release wrappers (`0x8005DC20..0x8005E030`,
  7 bodies)** - two clamps, two `0x8042BA54..0x8042BA64` copy helpers, the
  mixed word/halfword clear, the conditional `0x802B9280` free, and the
  `0xFFFFFFFF` sentinel store.
- **Small manager wrappers (`0x8005C9D0`, `0x8005D7E0`, `0x8005D830`,
  3 bodies)** - the active-slot flag scan and the acquire/release pair around
  the `0x80429C98` counter.
- **Class-slot helpers (`0x800564F0`, `0x80056900`, `0x80056BD0`,
  3 bodies)** - the `+0x450` animation start, the linked-actor fall test and
  the `+0x1090 == 3` / `+0x3D0` predicate.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80053940` | `fn_80053940` | 348 | medium |
| `0x80053AA0` | `fn_80053aa0` | 36 | high |
| `0x80053AD0` | `fn_80053ad0` | 432 | medium |
| `0x80053C80` | `fn_80053c80` | 352 | medium |
| `0x80053DF0` | `fn_80053df0` | 108 | medium |
| `0x80053E60` | `fn_80053e60` | 292 | medium |
| `0x800564F0` | `fn_800564f0` | 176 | high |
| `0x80056900` | `fn_80056900` | 124 | high |
| `0x80056980` | `fn_80056980` | 64 | high |
| `0x80056BD0` | `fn_80056bd0` | 76 | high |
| `0x80056C30` | `fn_80056c30` | 52 | medium |
| `0x80056D50` | `fn_80056d50` | 216 | high |
| `0x80056E30` | `fn_80056e30` | 12 | high |
| `0x80057860` | `fn_80057860` | 104 | medium |
| `0x8005C9D0` | `fn_8005c9d0` | 128 | high |
| `0x8005D7E0` | `fn_8005d7e0` | 72 | medium |
| `0x8005D830` | `fn_8005d830` | 76 | medium |
| `0x8005DC20` | `fn_8005dc20` | 20 | high |
| `0x8005DC80` | `fn_8005dc80` | 24 | high |
| `0x8005DEA0` | `fn_8005dea0` | 48 | medium |
| `0x8005DED0` | `fn_8005ded0` | 36 | high |
| `0x8005DFD0` | `fn_8005dfd0` | 32 | high |
| `0x8005DFF0` | `fn_8005dff0` | 64 | medium |
| `0x8005E030` | `fn_8005e030` | 12 | high |
| `0x8005FD20` | `fn_8005fd20` | 140 | high |
| `0x8005FEB0` | `fn_8005feb0` | 24 | high |
| `0x8005FED0` | `fn_8005fed0` | 24 | high |
| `0x8005FEF0` | `fn_8005fef0` | 184 | high |
| `0x8005FFB0` | `fn_8005ffb0` | 88 | high |

Byte total: 3364. Address range covered: `0x80053940..0x8005FFB0`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket still holds 49 untranslated functions after this pass.

* **`0x800544F0` (2540 B), `0x80054F80` (820 B), `0x800592C0` (7620 B),
  `0x8005B8A0` (1100 B), `0x8005D280` (1064 B), `0x8005E040` (624 B)** - the
  six remaining large dispatchers. Each needs its own pass over a record layout
  this lane did not open; translating them here would have tripled the review
  surface for six bodies.
* **`0x80055A00` / `0x80055A70` (112 B each)** - the `+0x10D4 |= 0x80000` /
  `|= 0x100000` speed-clamp twins call `FUN_80055900`, whose Ghidra prototype
  is `(double, int)` while the call sites pass no printed argument. With the
  float argument unresolvable there is no honest call shape, so the pair is
  left for a pass that first reconstructs `0x80055900`.
* **`0x8005ED90` (220 B) / `0x8005EF50` (1408 B)** - the two float/double
  helpers wave 88 already deferred; they still convert through the
  `0x43300000` double trick against tables the C export does not resolve.
* **`0x8005F4D0` (156 B), `0x8005F570` (80 B), `0x8005F5C0` (696 B)** - the
  spawn wrappers wave 88 read only to fix [`fn_8005eee0`]'s argument order.
  `0x8005F4D0` indexes `0x8042BA90`, whose contents are not exported, and
  `0x8005F5C0` opens with the outlined-prologue artefact.
* **`0x8005E620` / `0x8005E680` (4 B each)** - `b` thunks onto
  [`fn_8005e590`]/[`fn_8005e5d0`]; they carry no body of their own.
* **`0x8005FDB0` / `0x8005FE30` (116/124 B)** - the two presence-bit setters
  wave 88 deferred for being the same `+0xE0`/`+0x60` inference as
  [`fn_8005fb90`] at a larger review cost.

## Verification

- Every address and byte size matches `functions.tsv` (checked row by row).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave91` reports `OK` (29 rows, 29 unique
  addresses, 7 fields each).
