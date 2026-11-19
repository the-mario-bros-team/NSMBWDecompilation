# Coverage - wave 192, lane E

Module: `game::bases` (REL `d_basesNP`, synthetic base `0x80600000`).
Band: 192-256 bytes. Nine of the ten assigned functions are translated;
`0x80697090` is deferred (see below).

## Bodies

| Address | Size | Name | Summary |
| --- | --- | --- | --- |
| `0x8067A520` | 232 | `fn_8067a520` | Polar placement: `delta = (+0x460..+0x468) - (+0xAC..+0xB4)`, `angle = 0x801611C0(dy, dx)`, `+0x104 = angle - 0x4000`, `+0xE8 = radius * f(angle)` and `+0xEC = radius * g(angle)` with `radius = 0x807C9460[flags & 1]` (`1.3333333` / `2.0`) and `turn = (angle as u16) * 1/256`, plus `+0xF0 = 0.0`. |
| `0x8067C490` | 224 | `fn_8067c490` | Walks the `+0x30` child chain while `vtable[+0xC]` of the child returns `0`; returns the last node whose child reported non-zero. |
| `0x8067C690` | 216 | `fn_8067c690` | Allocates `0x560`, runs `0x80066FC0`, starts `+0x3D0`, `+0x42C`, installs `+0x3EC = 0x80317860`, `+0x3FC = 0.0`, `+0x414 = 1`, clears the `+0x400`/`+0x428`/`+0x540` blocks, and ends with vtable `0x807EC5D0` at `+0x60`. |
| `0x8067D2F0` | 224 | `fn_8067d2f0` | Byte-identical to `0x8067C490` (same probe, same five unrolled descents, same self-call displacement); same `+0x30` chain walk. |
| `0x80680AB0` | 244 | `fn_80680ab0` | In state 1/2: when `index % 3 == 0` and slot word `+0xD0C` has bit `0x10` and `0x80056CF0(actor, 0xA4) != 0`, claims `table+0x374D`/`+0x3750` from actor slot `+0x6C`; then calls `0x80056370(actor, table, 1)` when `+0xCD8 - +0xCE0 > 0.0`. |
| `0x80691D20` | 248 | `fn_80691d20` | Advances the `i16` phases `+0x500`/`+0x502`/`+0x504` by `0x200`/`0x400`/`0x200`, stores `trunc(sin(phase * 1/256) * k)` with `k = 700`/`1024`/`256` at `+0x100`/`+0x102`/`+0x104`. |
| `0x80693A50` | 256 | `fn_80693a50` | Builds the `+0x7C` transform from `+0xAC` and the `+0x100..+0x104` angles (after `0x80064AF0(pos, angles, 1)`), applies the three angles, then copies `+0x7C` and `+0xDC` into the part at `+0x3F0 + *(this+0x4A0) * 0x40` and enables it. |
| `0x80697C30` | 204 | `fn_80697c30` | Clamps `+0x768` to the `+0x77C`/`+0x778` window (else the `+0x348`-selected `+0x778` entry, with `+0x76C = *(+0x754)`), then stores `(+0x768 - +0xAC) <= 0.0` at `+0x348` on this object and on `actor_method0(this, 0x2DC)`. |
| `0x80698090` | 244 | `fn_80698090` | Divides the two input floats by slot `+0x35C` into `+0xE8`/`+0xEC`, queries `*(actor_method0(this, 0x2DC) + 0x5F8)` slots `+0xC`/`+0x10`, calls slot `+0x37C` when either is non-zero, and sets `+0xD28 = 1`. |

## Deferred

- **`0x80697090` (248 bytes).** The body reads two stack slots that it never
  writes (`lfs f2,0x18(r1)` at `0x8069709C` and `lfs f2,0x14(r1)` at
  `0x806970BC`) and treats them as the target `(x, y)` of a
  normalise-move-towards loop: `vec = (slot14 - +0xAC, slot18 - 16.0 - +0xB0,
  0.0)`, `0x80170A00(&vec)`, `vec *= 12.0`, add to `+0xAC`/`+0xB0`, then
  `0x80064D10(this, 1)` when both components are within `13.0`. Ghidra agrees
  and prints `local_1c`/`local_18` as used-before-set locals. The slot-`0x18`
  value is additionally rewritten with `slot18 - 16.0`, so both the direction
  and the arrival test depend on stale stack contents. There is no field on
  `this` (or documented parameter) holding the target, so no Rust body can
  reproduce the values without inventing a source; omitting it is safer than
  guessing.

## Reused neighbours

- `fn_80650d40` (`0x80650D40`) already pins the whole
  `0x801C0BD0` / `0x8016ECB0` / `0x8016EDD0` / `0x8016EEF0` /
  `0x8016A170` / `0x8016A150` / `0x8016A1A0` transform sequence, including the
  `call_f32x3_u32` wrapper for the lost `f32` arguments. `fn_80693a50` is the
  same sequence with part base `+0x3F0` and the `0x80064AF0` pre-pass.
- `fn_8067a840` (`0x8067A840`) already called `0x8067A520` by address and left
  it as "paired-single receiver"; this lane resolves that body (the two `psq_l`
  lanes are `GQR3` unsigned-16-bit samples, so no quantisation is involved).
- `fn_8067a7e0` (`0x8067A7E0`) and `fn_80691f30` (`0x80691F30`) pin
  `0x807C9408` as the `f32` zero and `0x80691D20` as the `GQR` conversion body
  that `0x80691F30` calls after `vmethod0(actor + 0x394, 0x10)`.
- `f32_of_f32` is the existing wrapper for `0x80237BD0`/`0x80237C40`;
  `actor_method0_f32` is the existing `+0x60`-vtable float wrapper used for slot
  `+0x35C`.

## Notes / inferred

- `0x801611C0` is the `atan2`-style `(y, x) -> s16` helper: other call sites in
  the export pass the y difference first (`local - *(float *)(this + 0xB0)`),
  and `0x8067A520` loads `f1 = +0x464 - +0xB0`, `f2 = +0x460 - +0xAC`.
- The raw word at `0x8067A580` is `0x7C7E0734` (`extsh r3, r30`); because both
  the `extsh` result and `r30` are stored as halfwords and only their low 16
  bits are loaded by the two `psq_l` samples, the emitted `angle as u16` is
  exact either way. The `+0x468 - +0xB4` component is computed and spilled but
  never read (dead).
- `0x8067C490`/`0x8067D2F0` are a five-times-unrolled loop that re-enters
  itself on the sixth child; each recursive call receives exactly the node the
  loop would advance to, so the single `loop` is behaviourally identical. The
  `bc 4,2` at `0x8067C4BC` is `bne` (BO=4 is "branch if condition false"), which
  is why a non-zero probe returns the current node.
- `0x80697C30` selects between `0x807CA048 = 1.0` and `0x807CA04C = -1.0`
  against `0x807CA038 = 0.0` and then compares the selection with `1.0`; that
  fold is reproduced literally rather than as `delta <= 0.0` so the unordered
  case also matches.
