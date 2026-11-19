# Coverage - Wave 33 (`game::stage`, BG scroll-layer band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/stage.rs` | `game::stage` | 25 | 2956 | 19 | 6 |

## What this pass covers

The `0x8007xxxx` band that this pass opens is **not** stage-block code. It is the
BG scene / scroll-layer object module that sits in front of the stage blocks:

- the camera owner is read from `r13-0x58B0`, the scroll-layer list head/tail
  from `r13-0x58A0`/`r13-0x589C`, the list source and its generation counter from
  `r13-0x5898`/`r13-0x5894`;
- the ground probe keeps its state, x, z and layer words at
  `r13-0x58D4..-0x58C8`;
- a layer object identifies itself with kind `+0xCC`, flag word `+0xD4`, mask
  word `+0xD8`, linked byte `+0xDC`, mask/group bytes `+0xDD`/`+0xDE` and
  generation stamp `+0xE0`, and carries its eight-scroll-element array at
  `+0x60`;
- the wave-31 body `0x80080170` (already translated as
  `bg_scene_follow_layers`) consumes exactly the `+0xAC..+0xB4` camera triple
  and the `r2-0x71F8` zero word that this band produces, which corroborates the
  field layout above.

Translated clusters:

- **attach and match** - the attach test that latches `+0xD4`/`+0xD8` from the
  layer kind (`0x80074FF0`), and the two mask/group point queries
  (`0x80076630`, `0x800767F0`) that walk the list and call
  `bg_unit_contains_point`.
- **list management** - the five-word edge clear (`0x80075070`), the two-slot
  `(id, value)` claim/lookup pair (`0x800751C0`/`0x80075230`), the insert-at-head
  (`0x8007F900`) with the shared field reset (`0x8007F8A0`), the unlink-plus-reset
  (`0x8007F950`) and the conditional delete helper (`0x8007F700`).
- **construction** - the layer constructor that builds the eight 8-byte element
  slots and links itself (`0x8007F7A0`), its destructor (`0x8007F810`), the
  owner/mask/callback binding (`0x8007F9B0`), and the three `0x80075820` spawn
  wrappers (`0x800757B0`/`0x800757E0`/`0x80075800`, all forwarding calls).
- **edge and quad update** - the ground-probe integrator (`0x80075270`) and its
  two result copies (`0x80075750`/`0x80075780`), the delta/absolute edge stores
  (`0x8007FCE0`, `0x8007FD10`, `0x8007FD40`, `0x8007FD60`), the tile-pointer set
  (`0x8007FD80`) and the big quad/centre rebuild (`0x8007FDA0`).

Deliberately skipped, because the export does not prove the argument list:

- `0x8007FA40` - reads floating-point arguments from `f1..f8` plus integers that
  Ghidra printed as undeclared locals; the roles are not recoverable, and
  `0x8007FAD0`/`0x8007FB10` are thin wrappers that would inherit the same guess.
- `0x80075820` - the eight-argument layer initialiser itself; it is dominated by
  GQR-configured `psq_l`/`psq_st` blocks over `f25..f31`, so the parameter-to-field
  mapping cannot be stated honestly. Its three wrappers above forward to it by
  absolute address instead.
- `0x8007FFA0` - takes a *pair* of objects plus GQR trig tables; the two object
  roles are not distinguishable from the export.
- `0x80076530`/`0x800768D0` - same GQR-plus-dropped-register problem.
- `0x800775A0`/`0x80077610`/`0x80077700`/`0x800777B0`/`0x80077810` - arguments are
  passed implicitly in registers that Ghidra dropped, so the reconstructed
  signatures would be invented.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80074FF0` | `fn_80074ff0` | 128 | high |
| `0x80075070` | `fn_80075070` | 28 | high |
| `0x800751C0` | `fn_800751c0` | 112 | high |
| `0x80075230` | `fn_80075230` | 64 | high |
| `0x80075270` | `fn_80075270` | 752 | medium |
| `0x80075750` | `fn_80075750` | 48 | high |
| `0x80075780` | `fn_80075780` | 36 | high |
| `0x800757B0` | `fn_800757b0` | 40 | high |
| `0x800757E0` | `fn_800757e0` | 20 | high |
| `0x80075800` | `fn_80075800` | 28 | high |
| `0x80076630` | `fn_80076630` | 184 | medium |
| `0x800767F0` | `fn_800767f0` | 224 | medium |
| `0x8007F700` | `fn_8007f700` | 64 | high |
| `0x8007F7A0` | `fn_8007f7a0` | 84 | high |
| `0x8007F810` | `fn_8007f810` | 108 | high |
| `0x8007F8A0` | `fn_8007f8a0` | 84 | high |
| `0x8007F900` | `fn_8007f900` | 72 | high |
| `0x8007F950` | `fn_8007f950` | 80 | high |
| `0x8007F9B0` | `fn_8007f9b0` | 144 | high |
| `0x8007FCE0` | `fn_8007fce0` | 44 | medium |
| `0x8007FD10` | `fn_8007fd10` | 44 | medium |
| `0x8007FD40` | `fn_8007fd40` | 20 | high |
| `0x8007FD60` | `fn_8007fd60` | 20 | high |
| `0x8007FD80` | `fn_8007fd80` | 28 | high |
| `0x8007FDA0` | `fn_8007fda0` | 500 | medium |

The `0x8007F800` element constructor is not a `functions.tsv` entry (Ghidra
prints it as `&DAT_8007f800`); it is documented as a constant
(`W33_LAYER_ELEMENT_CTOR`) and passed by address to `0x802DC9A0`.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/stage.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/stage.rs`.
