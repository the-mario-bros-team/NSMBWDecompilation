# Coverage - wave 189, lane E

Module: `game::bases` (REL `d_basesNP`, synthetic base `0x80600000`).
Band: 128-192 bytes. All ten assigned functions are translated; nothing deferred.

## Bodies

| Address | Size | Name | Summary |
| --- | --- | --- | --- |
| `0x8075C320` | 144 | `fn_8075c320` | Course-object constructor: `0x800F5FD0` base, vtable `0x80814668`, resource block `+0x18C`, arrays `+0x1AC`/`+0x1C4`, interface word `+0x1B8` written twice. |
| `0x80763740` | 188 | `fn_80763740` | Constructor with three arrays: `0x38`x6 at `+0x1B4`, `0xC`x3 at `+0x304`, `0x38`x3 at `+0x328`; vtable `0x80815778`. |
| `0x80763B70` | 176 | `fn_80763b70` | Rebuilds the `+0x7C` matrix from the live position and the three `s16` angles, copies it into the animation object at `+0x1A8`. |
| `0x80764910` | 132 | `fn_80764910` | Releases the `+0x38C` object, the four `0x40`-stride panes at `+0x1A8`, then the five `+0x440` panes via `fn_80765ba0`. |
| `0x80764F00` | 152 | `fn_80764f00` | Clears `+0x47C`/`+0x480`, writes `2.5` at `+0x10C`, samples the `W502` point and hands the delta against `+0xAC` to `0x800F6410`. |
| `0x807658D0` | 144 | `fn_807658d0` | Fetches the scene's `W502` position through `0x80100270` and adds the three range constants at `0x80815964..0x8081596C`. |
| `0x8076B9B0` | 176 | `fn_8076b9b0` | Word-for-word twin of `fn_80763b70`; called by `fn_8076b560` with `r3 = this`. |
| `0x8076C4E0` | 160 | `fn_8076c4e0` | Allocates/constructs the `+0x188` descriptor, binds `character_SV` + `g3d/model.brres`, and when `fn_8076d280` is ready creates the `0x28F` object at `+0x1B8`. |
| `0x8076C580` | 160 | `fn_8076c580` | `0x800F6910(0.5, 0.8)`, then `0x800D6F40(handle, &position, &angles, &scale)` and `0x800D6E90(handle)`. |
| `0x8076C620` | 144 | `fn_8076c620` | Resets `+0xAC..+0xB4` to the `main.dol` origin, overrides `+0xAC = -500.0`, calls vtable `+0x5C` of `*(*(+0x188)+4)` with `(5.0, 5.0, 0.0, 4)`, clears `+0x1B4`, runs `fn_8076c6b0`. |

## Reused neighbours

- `fn_8075aa50` (`0x8075AA50`, same 176 bytes) is the already-translated twin of
  `fn_80763b70`/`fn_8076b9b0`; it also pins the `0x801C0BD0`/`0x8016EF50`/
  `0x8016A170`/`0x8016A150`/`0x8016A1A0` sequence and the `+0x7C` matrix,
  `+0x100..+0x104` angles and `+0xDC` colour block.
- `fn_8076b3f0` pins the constructor template used by `fn_8075c320` (same
  `0x800F5FD0` / `+0x184 = -1` / `+0x60` vtable / `0x80069020` /
  `0x8016A2F0` / `0x803295D8` -> `0x80329578` sequence, shifted by four bytes).
- `fn_8076d3b0` pins the `0x802DC9A0(element, ctor, dtor, size, count)` descriptor
  triple used by `fn_80763740`.
- `fn_80765860`, `fn_80765ba0`, `fn_8076c6b0`, `fn_8076d280` already exist and are
  called directly instead of by address.

## Signatures of the two already-referenced callees

- `pub unsafe fn fn_8076b9b0(this: usize)` - one receiver in `r3`, `void`.
  `fn_8076b560` at `0x8076B5A8` does `mr r3,r31` before the call, so the existing
  `abi_call1!(0x8076_B9B0, this)` site is ABI-correct.
- `pub unsafe fn fn_8076c4e0(this: usize)` - one receiver in `r3`, `void`.
  `fn_8076c360` at `0x8076C374` calls it with `r3 = this`; likewise correct.
- `fn_8076c580` / `fn_8076c620` take the same single receiver.

## Notes / inferred

- `fn_807658d0` ignores its caller-visible `r4`: `0x807658D8` reloads `r4` with
  the scene pointer `0x8042A18C`, and `r3` is the only live argument. The
  existing `abi_call2!(0x8076_58D0, out, actor)` sites in `fn_807657f0` and
  `fn_80765860` remain valid.
- `fn_807658d0`'s export is GQR-damaged. The raw words are two `qr0` doubleword
  accesses: pair `0x80815964`/`0x80815968` added into `r1+0x14`/`r1+0x18` and
  pair `0x8081596C`/`0x80815970` into `r1+0x1C`; the three results are then read
  back as `out[0..3]`. This is the same three-float add that `fn_807657f0` and
  `fn_80765860` already assume.
- All six range/bias constants at `0x80815964..0x80815984` are `0.0` in the
  shipped REL, so `fn_807658d0` currently reduces to the `W502` point itself.
  They are still read with `read_f32` rather than folded, matching the existing
  neighbours.
- `fn_8076c580` reads the `s16` angles with `lha`/`sth`; only the low halfword
  survives, so `read_u16` reproduces the stored bits exactly.
- `fn_8076c620`'s three float arguments are `lfs`/`fmr` in `f1..f3`; Ghidra's
  `(double)` print for slot `+0x5C` is a prototype artefact.

## Deferred

None.
