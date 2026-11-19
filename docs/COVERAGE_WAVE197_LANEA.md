# Wave 197 lane A coverage (`game::bases`)

Slice: nine mid-size remainder bodies in the 304-312 byte band
(`0x80638820` .. `0x80754430`). Nine translated, none deferred.

| address | size | body |
| --- | --- | --- |
| `0x8063_8820` | 312 | `fn_80638820` - rebuilds the four `0x3C`-strided tilt words from the geometry table |
| `0x8065_C740` | 308 | `fn_8065c740` - writes the cached `+0x9C54`/`+0x9C74` values back into the resource tables |
| `0x8068_C940` | 312 | `fn_8068c940` - held-item hand-over driver (`+0x468` owner, `+0x430` timer) |
| `0x806C_AD10` | 304 | `fn_806cad10` - `(angle)` rotation of a position triple plus matrix rebuild |
| `0x806D_3370` | 312 | `fn_806d3370` - ground probe under `(+0xAC,+0xB0)` and hit-panel stamp |
| `0x8070_6850` | 304 | `fn_80706850` - advances one carried item along its node ring |
| `0x8071_C180` | 304 | `fn_8071c180` - lift presence/mode scan over the `+0x3D0` child list |
| `0x8073_3900` | 312 | `fn_80733900` - `houdai_slope_gun` / `_mouth` joint writers |
| `0x8075_4430` | 308 | `fn_80754430` - lift re-seat and collision-box re-arm |

## Signature recoveries

Three exports lose arguments because Ghidra modelled the `_savegpr_N` helpers as
value-returning calls:

* `0x8065C740` - `bl 0x802DCD68` is `_savegpr_N`; `r3` survives unchanged, so
  the real signature is `fn_8065c740(this: usize)` on the `0x9CE8` manager.
* `0x80706850` - `bl 0x802DCD74` (`_savegpr_27`): `fn_80706850(this: usize)`,
  the 12-byte record whose `+0`/`+4`/`+8` words are read and written.
* `0x8071C180` - `bl 0x802DCD60`: four live arguments
  (`this, out, mask_a, mask_b`), and the return is the mode in `r3`
  (`_restgpr` preserves it) rather than `void`.

`0x806D3370` has no such helper; its argument is simply the recovered `r3`.

## Decode notes

* `0x8065C740` stores through `stwbrx`. This host memory model is already
  big-endian (`platform::write_u32` emits `to_be_bytes`), so the plain word
  store is the exact byte image Ghidra expanded into four `stb`s.
* `0x8068C940` tests bit 30 of `-status | status`
  (`neg`/`or`/`rlwinm. r0,r0,1,31,31`), which is *not* identical to
  `status == 0` (they differ at `0x80000000`), so the raw test is emitted.
  The `lwzu r12,0x394(r3)` form makes the virtual `+0x28` call receive
  `owner+0x394` as `this`; `vmethod0`/`vmethod1` express that directly.
* `0x806CAD10` - Ghidra printed one float argument for `0x801C0BD0`, but live
  `f1`/`f2`/`f3` carry all three new coordinates, so
  `call_f32x3_u32` is used.
* `0x806D3370` - the `f1` result of `0x800EB930` is dead: both readers are
  behind the scene-state bit that reloads `f1` from `scene+0x8FEC4` first, so
  only the `r1+0xC` output slot is modelled.
* `0x80733900` - `neg`/`mulli 90`/`srawi 14`/`addze` fold to a truncating
  `(angle * -90) / 0x4000`; the `stfs` at `r1+0x1C` proves the
  `0x80241710`/`0x80241AA0` buffer is three floats even though Ghidra sized it
  as eight bytes.
* `0x80754430` - `0x800789A0`/`0x80078B40` are used with `fsubs`, so their
  results are single precision, matching the crate's existing `f32` bodies.

## Deferred

None.
