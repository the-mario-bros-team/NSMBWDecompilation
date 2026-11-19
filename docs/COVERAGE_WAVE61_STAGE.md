# Coverage - Wave 61 (`game::stage`, map-cell probes and candidate ranking)

## How to read this file

Each row is one translated `main.dol` function. The address is the original
PowerPC address from `work/ghidra/main-gb/export/functions.tsv`; the function
column is the Rust name in `src/game/stage.rs`; "bytes" is the `functions.tsv`
size; confidence matches the `# Confidence` line of the body. The evidence
column points at the Ghidra C export the body was written from.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 25 | 1912 | 14 | 11 |

No body in this pass is `low` confidence and none is `partial`.

## What this pass covers

This pass fills the contiguous map-cell probe cluster immediately below the
wave-33 BG scroll-layer run, `0x800704D0..0x80070CD0`:

- **Quantized cell cache** (`0x800704D0`) - truncates `x` and `-y`, rounds both
  down to `0x10`, compares them with the two halfwords at `r13-0x58D8`/
  `r13-0x58D6`, rewrites the pair and reports whether it changed.
- **Candidate tag/weight selectors** (`0x80070600`, `0x80070630`, `0x80070650`,
  `0x800706A0`, `0x800706E0`, `0x80070740`) - all use the same
  `0x802EF598` signed weight table indexed by the candidate tag in bits 16..23.
  `0x80070650` keeps the heavier candidate in the `+0xA0`/`+0xA4` slot, while
  `0x800706A0` and `0x800706E0` keep the lighter one in the `+0xA8`/`+0xAC`
  slot and the indexed `+0xB0`/`+0xB8` pairs. `0x80070630` clears kind `7`
  before forwarding, `0x80070600` writes the tag and its shifted word, and
  `0x80070740` reads one indexed tag back.
- **Height readers** (`0x80070760`, `0x800707E0`, `0x80070820`, `0x800708C0`,
  `0x80070900`, `0x80070920`, `0x80070960`, `0x80070980`) - signed `i16`
  heights, either from `this+0xBE` or from the linked record at
  `[this+0x50]+0x2C` when bit `0x8000` of `+0x88` is set. The small wrappers
  invert the value, add the half-turn offset `-0x8000` in 16-bit arithmetic,
  or classify the result into bands through `0x802DE514`.
- **Direction classifier** (`0x80070870`) - reads the `0x802EF548` halfword
  table and maps zero to `2`, a positive entry to `1` and a negative entry to
  `0`, honouring the optional negation.
- **Two-cell probes** (`0x80070A40`, `0x80070B10`, `0x80070BA0`, `0x80070BF0`,
  `0x80070C40`, `0x80070C60`) - quantize the position to 16-bit map
  coordinates, forward it to the two cell tests behind `0x80077650`/
  `0x80077700`, then combine the `0x800` flag and the bit-3 pair. The public
  entry point is [`fn_80070a40`] and its blocking summary is [`fn_80070b10`].
- **Status-flag applier** (`0x80070CD0`) - obtains the current object from the
  packed `0x802DCD74` result, sets `0x8000`/`0xC000` in `+0x88`, mirrors four
  record bits from `+0x3C`, updates `+0x46`/`+0x47`, clears `0x4000` for
  record bit `0x200`, and publishes the accumulated word through `0x802DCDC0`.
- **Small accessors** (`0x80070C80`, `0x80070CB0`) - the optional
  `[this+4]+0x38C` byte and the `+0x88 & 0x1FE000` nonzero test.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800704D0` | `fn_800704d0` | 92 | high |
| `0x80070600` | `fn_80070600` | 36 | high |
| `0x80070630` | `fn_80070630` | 20 | high |
| `0x80070650` | `fn_80070650` | 76 | medium |
| `0x800706A0` | `fn_800706a0` | 52 | medium |
| `0x800706E0` | `fn_800706e0` | 64 | medium |
| `0x80070740` | `fn_80070740` | 20 | high |
| `0x80070760` | `fn_80070760` | 120 | medium |
| `0x800707E0` | `fn_800707e0` | 52 | high |
| `0x80070820` | `fn_80070820` | 68 | high |
| `0x80070870` | `fn_80070870` | 72 | high |
| `0x800708C0` | `fn_800708c0` | 64 | medium |
| `0x80070900` | `fn_80070900` | 32 | medium |
| `0x80070920` | `fn_80070920` | 64 | high |
| `0x80070960` | `fn_80070960` | 24 | high |
| `0x80070980` | `fn_80070980` | 64 | high |
| `0x80070A40` | `fn_80070a40` | 200 | medium |
| `0x80070B10` | `fn_80070b10` | 132 | medium |
| `0x80070BA0` | `fn_80070ba0` | 72 | medium |
| `0x80070BF0` | `fn_80070bf0` | 72 | medium |
| `0x80070C40` | `fn_80070c40` | 28 | high |
| `0x80070C60` | `fn_80070c60` | 28 | high |
| `0x80070C80` | `fn_80070c80` | 28 | high |
| `0x80070CB0` | `fn_80070cb0` | 24 | high |
| `0x80070CD0` | `fn_80070cd0` | 408 | medium |

## Deliberately skipped

- `0x80070530` (120 bytes) and `0x800705B0` (68 bytes) - both exports hide the
  position argument that travels in `f1`; `0x80070530` also carries the usual
  `ldexpf`/`unaff_GQR0` paired-single noise. A body would have to invent the
  missing argument, so both are left. No translated body in this pass calls
  either function, so neither is needed by address yet.
- `0x80070720`, `0x80070730` and `0x80070CA0` (12 bytes each) - below this
  module's 16-byte floor; each is only a load, shift and mask, and the two
  tag getters are reachable through the indexed [`fn_80070740`].
- `0x800709C0` (128 bytes) - the export splits its position arguments into
  `undefined8` pairs, so the real argument mapping cannot be recovered.
- `0x80070E70` (128 bytes) and `0x80070EF0` (360 bytes) - both depend on
  `0x8006DB30`/`0x8006EF40`, whose arguments are dropped in the decompile, so a
  faithful call sequence cannot be written yet.
- `0x80070130` (916 bytes) and the rest of the `0x80070000` bucket - outside
  this contiguous run and still needing their own field maps.
- The already translated wave-33 and wave-59 entries in this bucket were not
  duplicated.

## Verification

- Every address and byte size matches `functions.tsv` (25 rows, 1912 bytes).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/game/stage.rs` (only pre-existing warnings in other modules).
- `node tools/check-lane-csv.mjs wave61` reports `OK`.
