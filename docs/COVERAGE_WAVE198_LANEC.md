# Wave 198 lane C coverage (`game::bases`)

Slice: the `Wm_jr` pair-separation body at `0x806A24F0` and the two number-panel
lift bodies at `0x806D5FB0`/`0x806D62C0` in `d_basesNP` (REL module 2). All three
assigned bodies are translated; nothing was deferred.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x806A24F0` | 1624 | `fn_806a24f0` | Separates the `Wm_jr` pair along the axis between them: unit vector from the `dx`/`dy` radial through `0x802BE470`, velocity dot products banded by `fn_806a22c0` (fallback band `2` when the `+0x2D4` probe is live, `*1.8` when `+0x31C` answers), two `0x802BE640` normalisations of the `+0xE8`/`+0xEC` facing plus/minus the axis with a `±1` X snap, the six-entry scale table at `0x807CA398..0x807CA2E0`, the `+0x314` radius sum minus the distance plus `2.0` positional clip, then `fn_806a2300(sample, max band)` and `fn_8069a910(other)` when `other` reports `+0x320 == 1` and the band is `2`. |
| `0x806D5FB0` | 516 | `fn_806d5fb0` | Rebuilds the nine-segment ridden trail at `+0xBB4..+0xEF0`: `+0xBB4` gets `(x-28, y) -> (x, y)`, the seven records `+0xC10..+0xE38` follow the node list `0x807CB6A1..0x807CB6A7` (`02 04 05 07 08 0A 0B`) with positions read through `0x80168180`/`0x8016F130` and biased by `(-x, 12.0 - y, -w)`, and `+0xE94` gets `(x, y) -> (x+28, y)`. |
| `0x806D62C0` | 528 | `fn_806d62c0` | Ages the four `+0xF06..+0xF0A` per-node counters, then walks every motion record's `+0x10` list and buckets each class-`1`/`2` actor with a `+0x6C` id below `4` into eight `48.0`-wide bands starting at `-192.0`; class-`1` actors with an empty `this + id + 0xF06` slot are armed through `fn_806d5c60(this, band)` via `0x80056CF0(actor, 0x1C)` and the slot is set to `4`. The eight counters land at `+0x3A` of the `0x58`-stride panel records from `+0x4C4`. |

## Notes on the raw words

- `0x806A24F0`: the export's `in_f30`/`in_f31` and `unaff_GQR0` are the
  `stfd`/`psq_st` prologue saves of `f30`/`f31` plus `psq_l` noise, not
  arguments - `f31` is the `0x802BE470` distance and `f30` the running dot.
  `func_0x802dcd74()` is `_savegpr_27_31`; `r3`/`r4` survive, so the signature is
  `(this, other)` with `r27 = this`, `r28 = other`. Ghidra's `iVar6` is `r4`.
- `0x806A24F0` comparison folds: every `fcmpo` + `bc` pair was re-checked. `bc
  4,<bit>` branches when the CR bit is **clear** (`beq` is `bc 12,2`), so
  `bc 4,1` after `fcmpo` means `<=` and `bc 4,0` means `>=`; Ghidra's `<`/`<=`
  renderings are therefore correct. The `mfcr`/`rlwinm r0,r0,1,31,31` ladder at
  `0x806A29D0` is the `2^52` (`0x807CA3B0`) unsigned-int to `double` conversion
  and the following `bc 4,1` picks the **maximum** of the two bands, truncated
  by `fctiwz`.
- `0x806A24F0` asymmetry: the second "not solid" block is gated on `this`'s
  `+0x2D4` and multiplies `this`'s `+0xE8`/`+0xEC`, but both zero stores land on
  `other + 0x11C`/`+0x114` (`stfs r0,0x11c(r28)`). That is literal, not a
  transcription slip.
- `0x806D5FB0`: the node table at `0x807CB6A0` reads `01 02 04 05 07 08 0A 0B`;
  the preamble passes the literal `1` and the loop reads `+1..+7`, i.e. the seven
  nodes `2,4,5,7,8,10,11`. `r30` is bumped once before the loop head, so the
  first byte consumed is `0x807CB6A1`. The `+0x54`/`+0x58`/`+0x5C` stores that
  record the biased `z` are dead (`0x8016F130` overwrites them next iteration)
  and are omitted.
- `0x806D62C0`: `func_0x802dcd60()` is `_savegpr_22_31`; `r3` survives, so the
  signature is `(this)`. Ghidra's `ldexpf` ladder is the `GQR0` guard noise. The
  band loop adds `48.0` before the compare, so the seven thresholds are
  `-144, -96, -48, 0, 48, 96, 144`, and a miss increments band `7`.
- `0x806D62C0` id checks: the `lbz`/`extsb`/`cmpli` triple at `0x806D6364` and
  `0x806D6430` is an unsigned compare of a sign-extended byte, which is
  equivalent to `read_u8(id) < 4`, so only ids `0..3` reach the body and
  `this + id + 0xF06` cannot go backwards.

## Reused neighbours

- Direct calls: `fn_806a22c0` (band classifier) and `fn_806a2300` (miss-hit
  ladder) from the same `0x8069/0x806A` `Wm_jr` family, `fn_806d5c60` (digit
  arming) from the number-panel family, and `fn_8069a910` (countdown).
- Helpers: `actor_method0`/`actor_method0_f32`/`actor_method1` for the `+0x60`
  vtable slots (`+0x2D4`, `+0x2F0`, `+0x314`, `+0x31C`, `+0x320`, `+0x3B4`,
  `+0x6C`), `vmethod0` for the `+0x5F8` sub-object's slot `+0x14`,
  `f32_of_f32` for `0x802BE470`, `abi_callN!` for `0x802BE640`, `0x80168180`,
  `0x8016F130`, `0x800DB590` and `0x80056CF0`, and `call0_ret` for the outlined
  register-save helpers.

## Deferred

None.
