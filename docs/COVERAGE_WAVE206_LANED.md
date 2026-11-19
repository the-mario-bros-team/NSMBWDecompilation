# Coverage - Wave 206 lane D (`ui::hud`, main.dol)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 10 | 1772 | 6 | 4 |

Addresses `0x8003_31E0`-`0x8003_3BF8`, the ten bodies that continue the
`0x80031xxx`/`0x80033xxx` block opened by wave 206 lane C
(`0x8003_1DF0`-`0x8003_31B0`). Wave 206 lane E starts at `0x8003_3D40`, so
there is no overlap with lanes A-C/E/F. All ten bodies are translated; nothing
in this lane is deferred.

Two adjacent bodies are outside this lane's ten and outside `functions.tsv`:
the scene constructor at `0x80033230` (which calls `0x80033310` first) and the
`0x80033C00` body. Both were read only to confirm the lane boundaries and the
object offsets, and neither is claimed here.

## Scope

| address | size | name | confidence | role |
|---|---|---:|---|---|
| `0x800331E0` | 12 | `fn_800331e0` | high | `0x80377A04` accessor |
| `0x800331F0` | 12 | `fn_800331f0` | high | second accessor |
| `0x80033200` | 12 | `fn_80033200` | high | third accessor |
| `0x80033210` | 12 | `fn_80033210` | high | fourth accessor |
| `0x80033220` | 12 | `fn_80033220` | high | fifth accessor |
| `0x80033310` | 300 | `fn_80033310` | medium | build the `kuriboBig` model and both animation records |
| `0x800334F0` | 344 | `fn_800334f0` | medium | full sprite transform, then bind the model translation |
| `0x80033650` | 228 | `fn_80033650` | medium | bind the `leg_left` / `leg_right` / `brow` animations |
| `0x800337F0` | 284 | `fn_800337f0` | high | four-argument mode dispatcher |
| `0x80033A30` | 456 | `fn_80033a30` | medium | countdown, effect request and launch-record writer |

Total: 1,772 bytes, 10/10 translated, 0 deferred.

## Size checks

`functions.tsv` reports `12/12/12/12/12/300/344/228/284/456` bytes. Every value
was re-checked against the prologue and the closing `blr` and matches the real
extent exactly, so no row is a prologue artefact:

| address | prologue | terminal `blr` | real extent |
|---|---|---|---|
| `0x800331E0`..`0x80033220` | none (`lis`/`addi`) | `0x800331E8`..`0x80033228` | 12 each |
| `0x80033310` | `stwu r1,-0x30` | `0x80033438` | 300 (`0x12C`) |
| `0x800334F0` | `stwu r1,-0xA0` | `0x80033644` | 344 (`0x158`) |
| `0x80033650` | `stwu r1,-0x50` | `0x80033730` | 228 (`0xE4`) |
| `0x800337F0` | `stwu r1,-0x20` | `0x80033908` | 284 (`0x11C`) |
| `0x80033A30` | `stwu r1,-0x50` | `0x80033BF4` | 456 (`0x1C8`) |

The five 12-byte bodies are plain `lis r3,0x8037` / `addi r3,r3,0x7A04` /
`blr` accessors with four bytes of padding after each, and they are
byte-identical to the nine `0x80377A04` accessors already in `src/ui/hud.rs`
so they return the existing `DAT_80377A04` constant rather than a new one.

## What the cluster is

The band sits between the `kuriboBig` scene constructor that follows it
(`0x80033230` calls `0x80033310` first) and the wave-206 lane C block that
precedes it. It reuses the model-scene conventions already documented for
`fn_80023f80`/`fn_80024190` in `src/ui/hud.rs`:

- **Model builder.** `0x80033310` is the `kuriboBig` twin of `fn_80023f80`:
  `0x800690C0` clears the `+0x524` buffer, `0x800DF180(*(r13 - 0x5648) + 4,
  "kuriboBig", "g3d/kuriboBig.brres")` loads the archive into `+0x540`,
  `0x80239E30` resolves the model into `+0x544` with flag `0x23`,
  `0x8023A0B0(+0x540, "walk")` resolves the clip into `+0x584`,
  `0x8023A200` resolves the second animation into `+0x5BC`/`+0x5C0`, and
  `0x80166BD0` starts it against a re-read `+0x5BC`. Two differences from
  `fn_80023f80` are preserved exactly: the closing order is `0x80166BD0`
  *then* `0x800690E0`, and the last two `0x80166BD0` flags are `(0, 0)`
  instead of `(0, 1)`.
- **Transform.** `0x800334F0` is the `kuriboBig` twin of `0x80024190`: same
  `0x80064AF0` angle normalisation, same `+0xD4`/`-0xD4` matrices around the
  second and first angles, same `+0x7C` sprite record. It differs only in the
  final binding: `0x8016A150` receives
  `{+0xDC + +0x75C, +0xE0 + +0x760, +0xE4 + +0x764}` instead of a raw `+0xDC`.
- **Bone binder.** `0x80033650` asks `0x80168400(+0x544)` for the animation
  object, then for each of `"leg_left"`, `"leg_right"`, `"brow"` does
  `0x8023B200(&object, name)`, takes the node's `+0x0C` word (or `0` on a null
  answer), builds through `0x80168180(+0x544, id, &scratch)` into one shared
  60-byte stack slot and copies it with `0x8016F130(&scratch, dest)` to
  `+0x718`/`+0x724`/`+0x730`.
- **Mode dispatcher.** `0x800337F0` calls `0x800964A0(this, contact, object, 0)`;
  modes `1` and `3` have byte-identical bodies (virtual `+0x2D8`, `+0x2DC`,
  then `*contact + 0xA1 |= 2`, then virtual `+0xD4` with the shared
  `0x8035366C` record), mode `0` forwards to the base collision reply
  `0x80095C20`, and any other mode does nothing. This is the exact `+0x1000`
  neighbour of lane F's `fn_8002f910`, which guards the same predicate and
  forwards to `0x80095C80`.
- **Burst body.** `0x80033A30` decrements the `+0x748` signed halfword, derives
  a side flag from `clz(0.0 <= *(contact_object + 0xE8)) >> 5` (i.e. *negative*
  means `1`), and either writes a ten-word launch record
  (`+0x414 = ±1.5` from `0x8042C1B0`, `+0x418 = 3.9`, `+0x41C = -4.0`,
  `+0x420 = -0.24375`, `+0x424 = 0x80357BF4`, `+0x428` = `0x8009F6D0(+0x510)`,
  `+0x42C = -1`, `+0x430 = side`, `+0x431` = the contact class byte,
  `+0x432 = 1`) or installs the `{0, -1, 0x80034140}` descriptor through
  `0x80034060` and calls virtual `+0x2D4`. It returns `1` in every path.

## Ghidra artefacts handled in this lane

1. **`unaff_r13` / `unaff_r2`.** Ghidra loses the small-data base register in
   four bodies. Re-read from the disassembly and recorded on the constant:
   `r13 - 0x5648` (`0x8042A038`, `0x80033310`), `r13 - 0x7DD8`
   (`0x804278A8` `"walk"`, `0x80033310`), `r13 - 0x7DD0` (`0x804278B0`
   `"brow"`, `0x80033650`), `r2 - 0x7B74` (`0x8042B50C` `0.0`,
   `0x800334F0` and `0x80033A30`), `r13 - 0x5940` (`0x80429D40`,
   `0x80033A30`) and `r2 - 0x6DB0`/`-0x6DAC`/`-0x6DA8`
   (`0x8042C2D0`/`D4`/`D8`, `0x80033A30`).
2. **Dropped call operands.** Ghidra prints `FUN_800964a0()` with no operands
   in `0x800337F0`, but the disassembly at `0x800337F8..0x80033818` sets
   `r6 = 0` and passes the three live arguments. `0x80095c20` prints with
   three operands and is called with the same three.
3. **`thunk_FUN_8016a8e0`.** The last call of `0x80033310` is
   `bl 0x800690e0`, i.e. the accessor-finish thunk already named
   `FN_800690E0`, not the `0x8016A8E0` body itself.
4. **Dead eight-word copy.** `0x80033A30` copies eight words from
   `0x802EE678` to the stack and then overwrites the first four with the four
   floats before anything reads them. The four dead words
   (`0x802EE678..0x802EE684`, all zero) are not modelled; the four live words
   (`0x802EE688..0x802EE694` = `0x80357BF4`, `0`, `-1`, `0`) are stored
   directly. This is documented in the body's prose.
5. **`clz` boolean.** `clz(0.0 <= v) >> 5` is `v < 0.0` (with NaN counting as
   `1`); the body keeps the exact `!(0.0 <= v)` form so the NaN behaviour is
   preserved.
6. **Reused lane-suffixed constants.** Wave 205 lane D already defined
   `G_SDA_5648_D`, `G_SDA_5940_D`, `FN_80066630_D` and `FN_8009F6D0_D` in
   `src/ui/hud.rs`; this lane reuses them instead of redefining them, and
   every new constant carries the `_D` suffix.

## Confidence notes

`high` for the five accessors (single literal `lis`/`addi` pair) and for
`0x800337F0` (three literal modes, two byte operations, every vtable slot
literal). `medium` for the four remaining bodies: the `0x23` bind flag, the
archive/clip lookup roles, the `0x80357BF4`/`-1` words of the launch record,
the `0x8035366C` shared record and the `0x80168400`/`0x8023B200` node protocol
are all inferred from their call shapes and adjacent string data rather than
from recovered types. No body is `low` and none is deferred.
