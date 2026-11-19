# Wave 197 lane C coverage (`game::bases`)

Slice: the `0x807B8A90..0x807C08B0` world-map / ship-ride remainder of
`d_basesNP` (REL module 2). All ten assigned bodies are translated; nothing was
deferred.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x807B8A90` | 244 | `fn_807b8a90` | Seeds `+0x120C`/`+0x120D` from `0x80315B9C`/`0x80315B9D`, derives stage (bits 8-11 of `+4`) and row (low byte), applies the `0x8042A21C` save-record override, and fills `+0x120E..+0x1211` from the `0x8008E3D0` node bytes `+0x0D`/`+0x0E`. |
| `0x807BA440` | 204 | `fn_807ba440` | Formats `light/W%X.blight`/`.blmap` (world+1) while `0x8042A24D < 9` and binds both `Env_world` members to slot `+0x08` of the world and of its `+0x14` child. |
| `0x807BA660` | 216 | `fn_807ba660` | Builds the `MainSelect` scene object from `scene/scene.brres` (`0x8023A2E0` + `0x802426D0`), re-enters it through `0x802C8840(0.0, world, &sel, -1, 3)`, then dispatches index `0`/`1` to `fn_807ba440`/`fn_807ba510`. |
| `0x807BAA30` | 232 | `fn_807baa30` | Guarded (`0x8042A24F`, `0x800F9930`, `0x80102750(...,2)`, `0x800FD120`) save-table commit through `0x8008EC90`; mode `0` refreshes `[0x8042A038]+4`, any other mode rebuilds the `Stage` member. |
| `0x807BCC00` | 220 | `fn_807bcc00` | Opens the selected island: state word `+0x26C` = `3` for `+0x27C`, `2` otherwise, timer `+0x29C = 60`, and speed `-150.0`/`25.0`/`9.0` with `speed / 25.0` into `+0x2B0`/`+0x2A8`. |
| `0x807BF760` | 244 | `fn_807bf760` | Tracks the `+0x5F0` actor, adds the `+0x5F4` offset, and either copies `+0xAC..+0xB4` straight into `+0x4BC..+0x4C4` (byte `+0x624` set) or routes it through `0x80160AE0(0.1, 5.0, 1.0, ...)`. |
| `0x807BF860` | 200 | `fn_807bf860` | Measures the `+0x5F4`/`+0x71C` target with `0x80160AE0` (either the `0x808215B4` triple or the `+0x71C` floats) and clears `+0x5F4`/`+0x71C` when `|len| <= FLT_EPSILON`. |
| `0x807BFC20` | 220 | `fn_807bfc20` | Eases along the target's `+0x10C` value: `(v, 0.1 v)` above `FLT_EPSILON`, otherwise `(5.0, 1.0)`, stepped by `0x80160AE0(0.05, ...)`; clears `+0x5F4` on arrival. |
| `0x807C0330` | 200 | `fn_807c0330` | Copies and transforms one probe (`0x800F9CD0`, `0x80238540` twice) and grows the aim window `+0x658..+0x66C` (min/max x and z, with the paired y at `+0x65C`/`+0x668`). |
| `0x807C08B0` | 252 | `fn_807c08b0` | Clamps the cached aim offset `+0x6A4..+0x6AC` to the window and returns the class mask with `0xF7`/`0xFB`/`0xFD`/`0xFE` bits cleared for every axis pinned to an edge. |

## Notes on the raw words

- `0x807B8A90`: Ghidra prints `func_0x8008e3d0(iVar2)` with one argument, but
  `r4` still holds the row byte written to `+0x1211` at the `bl`; the
  already-translated `fn_807b7540` calls the same helper as `(stage, row)`, so
  the two-argument form is used. The stage-table null fold
  (`[record+0x2C] == 0 -> 0`) is the same as in `fn_807b7540`.
- `0x807BA660`: the export types the body `void FUN_807ba660(void)` and shows
  `func_0x802dcd74()` returning a `u64`. That is `_savegpr_27_31`, not a call:
  `mr r27,r3` / `mr r28,r4` preserve both incoming arguments, `mr r3,r28` feeds
  `0x80164B70` and `cmpi cr0,28,r28,0` is the dispatch key, so the signature is
  `(r3, index)` and the second argument is the index.
- `0x807BA440`/`0x807BA660` string base: `lis/addi 0x8082,0x0618` gives
  `Env_world` `+0x500`, the two `light/W%X` formats `+0x4D8`/`+0x4F0`,
  `scene/scene.brres` `+0x550`, `MainSelect` `+0x564` and `Stage` `+0x570`.
- `0x807BCC00`: the `xoris 0x8000`/`0x4330`/`lfs`/`fsubs` ladder with `li r0,25`
  and the double `0x807D09F8` (`2^52 + 2^31`) is the compiler's signed-int to
  `f32` conversion of `25` (same decode as `fn_807bb870`); Ghidra's
  `4503601774854169.0 - dVar2` is not a real `2^52` subtraction. The three
  speeds were read from the image: `0x807D0A24 = -150.0`,
  `0x807D0A1C = 25.0`, `0x807D0A20 = 9.0`.
- `0x807BF760`/`0x807BF860`/`0x807BFC20`: `0x80160AE0` takes five arguments
  (`f1..f3`, then `r3` = destination `this+0x4BC`, `r4` = source). Ghidra drops
  the fifth argument in `0x807BF860`/`0x807BFC20`; the register is still live
  (`r4 = [+0x5F4]`) at the `bl`, and the order matches `fn_807bfd00`.
- `0x807C0330`: Ghidra shows `func_0x800f9cd0(auStack_18)` with one argument,
  but `r4` still holds `param_2`, exactly as in the translated caller at
  `0x807C05xx`; `lfs 0xA6C(0x807D)` then zeroes the third local float.
- `0x807C08B0`: the three offset floats are spilled to `r1+8`/`r1+0xC`/`r1+0x10`
  and each bound test re-loads the spill, so all clamps use the pre-clamp value;
  the equality/mask block re-loads the live fields.

## Reused neighbours

- Direct calls: `fn_807bf560` (state-0 snapshot), `fn_807bf5a0` (tail update),
  `fn_807ba440` and `fn_807ba510` (dispatch targets of `fn_807ba660`).
- `vmethod1(obj, 0x08, res)` is used instead of `fn_807ac270`/`fn_807ac260`
  because those Rust bodies only forward `this`; the raw sequence is
  `lwz/lwz/mtctr/bctr` with `r4` still holding the resource, which is exactly
  `vmethod1` (same choice as `fn_807ba510`).
- Host ABI wrappers: `call_f32x3_u32x2_f64` for `0x80160AE0`,
  `call_f32_u32x4` for `0x802C8840`, `abi_callN!` for the remaining callees.

## Deferred

None.
