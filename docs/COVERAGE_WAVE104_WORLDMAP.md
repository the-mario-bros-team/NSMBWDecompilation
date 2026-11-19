# Coverage - Wave 104 (`game::worldmap`, `0x80130000` bucket)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the offsets this pass introduces are declared in a `W104_` block
so they cannot collide with earlier passes. `// observed:` marks a literal from
the export, `// inferred:` a role guessed from call sites and `// unknown:` an
unresolved role.

The `0x80130000` bucket had 25 untranslated entries when wave 98 finished.
Twenty-one are written here; the four deliberate skips are listed below.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 21 | 8,020 | 0 | 20 | 1 |

Twenty rows are `translated`; `0x80133E10` is `partial` because its export
reports an unrecovered jump table. No body is a `stub`.

## What this pass covers

- **`0x80130340` - the `+0x2060` re-arm gate.** Refreshes the ride's
  sub-controller with `0x8005E6F0`/`0x8005E4F0`, stores `0x14` at `+0x2060`,
  then either posts `0x80375CAC` or builds the four-word `0x803089B0` record
  and posts `0x80375C6C` before calling `0x8005E3B0(+0xEA4, 3)` and
  `0x80056C70(object, 0xA8)`.
- **`0x80132340` - the `ivy` effect placement.** Samples the ride's `+0x50`
  slot at `0xB` and `0xE`, averages the two points through
  `0x8016F130` and emits `Wm_mr_ivy` through `0x8008FBA0` with the un-halved
  axis sums.
- **`0x80132580`/`0x80133B60` - the unsigned travel conversion pair.** Both
  reduce `+0x14F8`/`+0x10A0` with the compiler's
  `CONCAT44(0x43300000, x)` idiom, which is reproduced through
  `unsigned_to_double`; `0x80132580` probes forward/backward and
  `0x80133B60` slides toward the partner actor.
- **`0x80131990`/`0x80132A80` - the speed and turn selectors.** The first eases
  the vertical/lateral pair through `0x8015F480` and writes the `+0x11C` state
  word from nine constants; the second maps the `+0x2B8` variant to `+0xEC` and
  `+0x10C`.
- **`0x80131F20` - the two-point ride probe.** Builds the three-axis matrix
  with `0x801C0BD0`/`0x801C0540` and the `0x8016ECB0`/`0x8016EDD0`/
  `0x8016EEF0` halfword setters, then walks the two `+0xF0` offsets through
  `0x80075FD0` and publishes `+0x2A6C`/`0x2A70`.
- **`0x80133950`/`0x80137E20`/`0x8013A880` - the placement/variant setters.**
  `0x80133950` stores the offset angle, the three-word position and `+0x1544`;
  `0x80137E20` resets the `+0x14A0` variant and selects the `+0xEC`/`+0x10C`
  rate; `0x8013A880` clears attach bit `0x80` and forwards the `+0xDE8` word to
  `0x800D6940`.
- **`0x80135070..0x801363C0` - the contact ladder.** The forward snap
  `0x80135810`, the landing commit `0x801358E0`, the full ground/lean frame
  `0x801359C0` and the trick attempt `0x801363C0`, all keyed on the
  `0x800757B0`/`0x80075270`/`0x80075CA0` probe family.
- **`0x80137550`/`0x8013EFF0` - the partner hand-over.** `0x80137550` forwards
  the delay/event pair to `0x80137590` and clears `+0x14AC`; `0x8013EFF0` runs
  the three `+0x1458` phases with the `r13-0x59EC` manager.
- **`0x801396C0`/`0x8013BB90`/`0x8013E850` - the distance and list helpers.**
  The slide-contact commit, the horizontal distance publish with the
  `0x802BE470` square root and the class-`2` anchor scan.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80130340` | `fn_80130340` | 444 | medium |
| `0x80131990` | `fn_80131990` | 1,096 | medium |
| `0x80131F20` | `fn_80131f20` | 780 | medium |
| `0x80132340` | `fn_80132340` | 244 | medium |
| `0x80132580` | `fn_80132580` | 292 | medium |
| `0x80132A80` | `fn_80132a80` | 496 | medium |
| `0x80133950` | `fn_80133950` | 136 | medium |
| `0x80133B60` | `fn_80133b60` | 324 | medium |
| `0x80133E10` | `fn_80133e10` | 76 | low |
| `0x80135070` | `fn_80135070` | 260 | medium |
| `0x80135810` | `fn_80135810` | 196 | medium |
| `0x801358E0` | `fn_801358e0` | 224 | medium |
| `0x801359C0` | `fn_801359c0` | 608 | medium |
| `0x801363C0` | `fn_801363c0` | 644 | medium |
| `0x80137550` | `fn_80137550` | 52 | medium |
| `0x80137E20` | `fn_80137e20` | 676 | medium |
| `0x801396C0` | `fn_801396c0` | 268 | medium |
| `0x8013A880` | `fn_8013a880` | 160 | medium |
| `0x8013BB90` | `fn_8013bb90` | 572 | medium |
| `0x8013E850` | `fn_8013e850` | 204 | medium |
| `0x8013EFF0` | `fn_8013eff0` | 268 | medium |

## Deliberate skips

Four entries of the 25 remain untranslated. Each one loses a value through the
`GQR3` paired-single register, so the arithmetic cannot be reproduced without
inventing the scale.

- **`0x80130F40` (1,160).** The `GQR3` block computes `dVar17` from
  `ldexpf(-(bVar7 & 0x3f))` and that value is multiplied into `+0x2068`,
  `+0x206C` and `+0x110`, so the scale reaches a stored field.
- **`0x80137320` (492).** Same pattern with two `GQR3` blocks feeding
  `+0x2A6C` and `+0x2A70`; the `CONCAT22`/`CONCAT31` arms are the compiler's
  subnormal fixups, not readable data.
- **`0x80138510` (416).** Both the success and the failure branch multiply the
  angle through `ldexpf` before the `+0x3FC`/`+0x410` slot calls.
- **`0x8013B0B0` (200).** The `GQR3` scale divides the `+0x2774` offset
  (`(x - x') / dVar5`), so the divisor is not recoverable from the C export.

Notes on the accepted records:

- Eight of the accepted exports keep the `GQR0` scaffolding
  (`if ((unaff_GQR0 & 0x3f00) != 0) { ldexpf(...); }`) but discard the result;
  as in wave 103 these bodies transcribe the printed arithmetic and drop the
  dead scaffolding.
- Where a `+0x5C`/`+0x8C` controller slot is printed with only float operands,
  the receiver sits in `r3`; the bodies pass the controller there and say so in
  their `# Confidence` line. `0x80132340`'s eleven-operand effect call keeps the
  undecoded incoming `r10` word as a zero argument and marks it `// unknown:`.
- `0x80133E10` is `partial`: Ghidra reports "Could not recover jumptable at
  `0x80133E58`" and folds an indirect branch into the slot call, so an arm may
  be missing. The state store, the table index and the vector read are literal.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the 21 sizes sum to 8,020 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 21 were untranslated
  entries of the `0x80130000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave104` reports `OK` for
  `ledger/agents/wave104-worldmap.csv`.
