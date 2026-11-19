# Coverage - Wave 38 (`game::stage`, dLineMng line-object cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 30 | 8360 | 15 | 15 |

## What this pass covers

The `0x800Cxxxx` band opens with the **dLineMng line-object** system (line
segments with per-line state records at `0x80358E10..0x80359410`): a line
object keeps its stored point at `+0x50`, its speed at `+0x60`, its facing
byte at `+0x68`, its angle at `+100` and its state interface at `+0x6C`.
The kind of a point comes from the zone lookup [`fn_800c17a0`]; the two
membership tables at `0x802F0FE8`/`0x802F0FF8` distinguish older from newer
line types. The stage-object helpers already in the file (`+0x40/+0x44`
clamps, `-0x6810`/`-0x6818` margins) are reused here, so the field layout
is corroborated by the wave-31/wave-33 bodies.

Translated clusters:

- **membership, flip and lookup** - the two table scans (`0x800C1700`/
  `0x800C1740`), the flip (`0x800C1780`) and the zone-gated lookup
  (`0x800C17A0`).
- **line geometry** - the slope/intercept helper (`0x800C19E0`), the
  tolerance span test (`0x800C1A40`) and the squared-radius straddle test
  (`0x800C1E10`) that the nine circle handlers below share.
- **circle handlers** - the nine `0x800C2700..0x800C3050` handlers. Each
  offsets the centre by a `-0x68xx`/`-0x67xx` half-extent, rewrites two
  point pairs as relative vectors, runs `0x800C1E10`, stores the
  `0x801611C0` bearing at `+100` (clamped to `[0x100, 0x40FF]` with a
  per-family bias) and transitions to one `0x803592xx` record; `0x800C2700`
  is the only one without a clamp or a flip.
- **angle blends and clamps** - the two normalisers (`0x800C3B50`/
  `0x800C3BAC`), their two blend wrappers (`0x800C3BC0`/`0x800C3C30`) and
  the two edge clamps (`0x800C3AD0`/`0x800C3B10`).
- **state machines and dispatchers** - the four state machines
  (`0x800C3CA0`/`0x800C4060`/`0x800C43A0`/`0x800C4780`) dispatch on the
  line kind, store the point at `+0x50`, blend the `+100` angle through
  the membership gates and transition to the matching `0x80358xxx`
  record; the four dispatchers (`0x800C4B50..0x800C4E50`) retry three of
  those machines with expanding/shrinking `+-0x6818` point offsets and
  fall back to `0x80358E50`. The state machines return the value they
  pass to the `0x802DCDC0` epilogue (Ghidra prints `void`, but the
  dispatchers test it), so the Rust bodies return `u32`.

Deliberately skipped, because the export does not prove the call or the
parameter mapping:

- `0x800C0570` / `0x800C10F0` / `0x800C18B0` / `0x800C1A90` / `0x800C1C80`
  / `0x800C3A10` / `0x800C4F50` / `0x800C51A0` / `0x800C9680` / `0x800C9B50`
  / `0x800CAB30` / `0x800CAD20` / `0x800CB1C0` / `0x800CB430` and
  the other `ldexpf`/`unaff_GQR0` carriers in the bucket: paired-single
  code the brief requires skipping.
- `0x800C1E90` / `0x800C1F20`: callers of the `ldexpf`-tainted `0x800C1A90`/
  `0x800C1C80`; the calls they print omit the two double arguments, so the
  signatures would be invented.
- `0x800C2030..0x800C2690` (the fifteen transition wrappers): each calls
  `0x800C1E90`/`0x800C1F20`/`0x800C2030` with an argument list Ghidra
  dropped, so the forwarding cannot be stated honestly.
- `0x800C75B0` (4772 bytes): the full `dLineMng` state-table initialiser;
  out of scope for this pass.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800C1700` | `fn_800c1700` | 56 | high |
| `0x800C1740` | `fn_800c1740` | 56 | high |
| `0x800C1780` | `fn_800c1780` | 28 | high |
| `0x800C17A0` | `fn_800c17a0` | 104 | medium |
| `0x800C19E0` | `fn_800c19e0` | 88 | high |
| `0x800C1A40` | `fn_800c1a40` | 76 | high |
| `0x800C1E10` | `fn_800c1e10` | 128 | high |
| `0x800C2700` | `fn_800c2700` | 240 | medium |
| `0x800C27F0` | `fn_800c27f0` | 312 | medium |
| `0x800C2930` | `fn_800c2930` | 304 | medium |
| `0x800C2A60` | `fn_800c2a60` | 300 | medium |
| `0x800C2B90` | `fn_800c2b90` | 284 | medium |
| `0x800C2CB0` | `fn_800c2cb0` | 304 | medium |
| `0x800C2DE0` | `fn_800c2de0` | 312 | medium |
| `0x800C2F20` | `fn_800c2f20` | 300 | medium |
| `0x800C3050` | `fn_800c3050` | 284 | medium |
| `0x800C3AD0` | `fn_800c3ad0` | 60 | high |
| `0x800C3B10` | `fn_800c3b10` | 60 | high |
| `0x800C3B50` | `fn_800c3b50` | 72 | medium |
| `0x800C3BAC` | `fn_800c3bac` | 24 | high |
| `0x800C3BC0` | `fn_800c3bc0` | 108 | high |
| `0x800C3C30` | `fn_800c3c30` | 108 | high |
| `0x800C3CA0` | `fn_800c3ca0` | 956 | medium |
| `0x800C4060` | `fn_800c4060` | 828 | medium |
| `0x800C43A0` | `fn_800c43a0` | 988 | medium |
| `0x800C4780` | `fn_800c4780` | 972 | medium |
| `0x800C4B50` | `fn_800c4b50` | 252 | high |
| `0x800C4C50` | `fn_800c4c50` | 252 | high |
| `0x800C4D50` | `fn_800c4d50` | 252 | high |
| `0x800C4E50` | `fn_800c4e50` | 252 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/stage.rs`.
