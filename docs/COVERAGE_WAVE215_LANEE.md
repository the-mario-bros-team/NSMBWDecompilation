# Coverage - wave 215 lane E (`engine::dsys`, `0x800F1BD0..0x800F21A0`)

Lane E of wave 215 translated the ten-body animation-sprite cluster that sits
directly above the `0x800F1390`/`0x800F1550` resource bindings already in
`src/engine/dsys.rs`. All ten are `translated`; eight are `high` confidence and
two (`0x800F1BE0`, `0x800F1D10`) are `medium`. Nothing was deferred.

Block: `work/wave215/laneE_block.rs`. Ledger rows:
`ledger/agents/wave215-lanee.csv`. Target file: `src/engine/dsys.rs` (not edited
by this lane).

Total: 1,472 bytes selected across 10 functions.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | ---: | --- | --- | --- |
| `0x800F1BD0` | 12 | `fn_800f1bd0` | `*(this + 0xC) + 0x58`: the track block of the animation record behind a handle. | high |
| `0x800F1BE0` | 8 | `fn_800f1be0` | Two-word tail call (`b 0x800F22F0`) into the track teardown; the callee's `r3` is the result. | medium |
| `0x800F1BF0` | 112 | `fn_800f1bf0` | Construct the 0x94-byte record: `0x80168D10` builds the 0x38-byte track at `+0x58`, the vtable is then replaced with `0x80303408`, `+0x90` is cleared and `0x802E19F8(this, name, 0x10)` copies the 16-byte class name in. Returns `this`. | high |
| `0x800F1C60` | 176 | `fn_800f1c60` | `0x802B9210(0x94)`, resolve the `+8`-offset child name of `*(record)` (0 when the offset is 0, and the record is then built with a null name), run `0x800F1F80`, `0x800F20B0`, `0x800F1E30`, return the record (`0` if the allocation failed). | high |
| `0x800F1D10` | 256 | `fn_800f1d10` | Per-frame update: under `+0x90 & 1` push `a` into the track and `b` (or `(float)+0x80 - 1.0f` when `b < 0.0`) into the second setter, then step the `+0x50` and `+0x54` effect slots under bits 2 and 4 when each index is non-negative. | medium |
| `0x800F1E10` | 28 | `fn_800f1e10` | `+0x90 & 1` gate, then a tail call to vtable slot `+0x14` of the track at `+0x58` with that track as the receiver. | high |
| `0x800F1E30` | 96 | `fn_800f1e30` | `+0x90 & 1` gate: `0x800DFBE0(track)`, then `0x800DFB70(manager + 8, track, this->+0x4C, this + 0x10, 0)`. | high |
| `0x800F1E90` | 240 | `fn_800f1e90` | "Still running" predicate: clears the result when the track reports finished and its value is not `0.0f`, sets it when the value is exactly `length - 1.0f`, then clears it for either effect slot that reports stopped. | high |
| `0x800F1F80` | 304 | `fn_800f1f80` | Bind from one resource name: `0x80168400(manager + 8)`, the 0x20-byte key block (`+4` word only), `0x8023A0B0(key, name)`, `0x801650D0(track, &handle, &id, param_4, 0)`, flag byte `+0x8C`, both track times set to zero, flag bit 1, then `0x800F1550`/`0x800F1390` reserve `+0x50`/`+0x54` and set bits 2/4. | high |
| `0x800F20B0` | 240 | `fn_800f20b0` | Clear the `+0x4C` kind byte, walk the manager's children (`0x800DFD20`/`0x800DFDD0`) until one compares equal to the `"anim"` name word at `0x804288C0` over four bytes, then re-test the last child and hand it to `0x800F21A0`. | high |

## Decoding notes

- **Sizes are all real.** `functions.tsv` gives 12 / 8 / 112 / 176 / 256 / 28 /
  96 / 240 / 304 / 240. Every body's last instruction is a `bclr 20,0` exactly
  four bytes before the next table entry (`0x800F1BDC`, `0x800F1BE8`,
  `0x800F1C60`, `0x800F1D10`, `0x800F1E10`, `0x800F1E30`, `0x800F1E90`,
  `0x800F1F80`, `0x800F20B0`, `0x800F21A0`); `0x800F1BE0` is the two-word
  `b 0x800F22F0` tail call. None of the ten sizes is a prologue artefact.
- **`0x802DCD74`/`0x802DCDC0` in `0x800F1F80`.** They are the outlined
  `__savegprlr_27..31`/`__restgprlr_27..31` helpers: the export's
  `uVar6 = FUN_802dcd74()` is just the untouched pair `(r3, r4)`, i.e. the
  ordinary `this` and `manager` arguments, so the bodies use the parameters
  directly and neither stub is emitted.
- **`ldexpf`/`unaff_GQR0` heads.** The blocks at the top of `0x800F1D10` and
  `0x800F1E90` are the paired-single saves of `f29..f31`; the values are never
  read again and are omitted (same treatment as the rest of the module).
- **Constants.** `0x8042CF08` is `0.0f` and `0x8042CF0C` is `1.0f` (read out of
  the image, not assumed: the negative-delta branch is
  `fsubs (float)+0x80, 1.0f`). `0x804288C0` holds the address of the `"anim"`
  string at `0x804288B8`, which is why the two `0x802E1BD0` name tests compare
  four bytes.
- **`0x800EF890`/`0x800EF940` arity.** The export prints
  `(undefined8, double, int, undefined4)`; the callee's prologue shows
  `r30 = r3`, `r31 = r4`, `f29 = f1`, `f30 = f2`, and the caller
  `0x800F1D10` loads `r4 = this->+0x50` (then `+0x54`) at `0x800F1D90` /
  `0x800F1DB8` with `r3 = manager`, so the real shape is
  `(manager, slot, a, b)` and `r5`/`r6` are leftovers. Modelled with the new
  `call_u32x2_f64x2_e` wrapper.
- **Double vs single in `0x800F1D10`/`0x800F1E90`.** `a`/`b` arrive as `f64`
  (`fcmpu` is used on `f2` directly), while the length arithmetic is
  single-precision (`lfs` + `fsubs`), so the bodies keep the `f32` compare for
  `length - 1.0f` and widen only where a `f64` setter needs it.
- **`0x800F1F80` bind order.** The store `stw r3,0x50(r27)` at `0x800F2050`
  happens before the `bge` at `0x800F2054`, so the slot is always written and
  only the flag bit is conditional; the same holds for `+0x54`.
- **`0x800F20B0` re-test.** The one-word local is overwritten by every
  iteration, so the post-loop `0x802E1BD0` re-checks the *last* child examined
  (or `0`) against `"anim"` before `0x800F21A0` is called.

## Deferred

None.
