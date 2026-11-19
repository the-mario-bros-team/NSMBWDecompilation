# Wave 194 lane D coverage (`game::bases`)

Ten functions in the `0x8075FA40..0x80783300` band were decoded from the raw
PowerPC words of `work/rels/elf/d_basesNP.elf` and checked against the Ghidra
exports. All ten are translated; none are deferred.

## Translated

| Address | Size | Body |
| --- | --- | --- |
| `0x8075FA40` | 240 | Rebuilds `+0x1AC`/`+0x260`, dispatches vtable slot `+0x18`, publishes the `cobCastle` event. |
| `0x80762FE0` | 212 | Looks up the per-actor sound-bank entry and writes `+0x258`/`+0x25C`. |
| `0x80764E00` | 252 | Initialises six `0x38`-byte wave records seeded by `+0x460..` and `+0xAC..`, then `0x80765290`. |
| `0x8076E490` | 228 | Re-binds animation slot `index` at `+0x1E8` when `+0x354` differs. |
| `0x8076E760` | 252 | Registered destructor; releases `+0x4B4` and seven sub-objects. |
| `0x80774D00` | 236 | Returns whether the mode-selected zone flag is still clear (`1/2/4` masks). |
| `0x80775730` | 196 | Returns `1` when exactly one of six zone switches is active. |
| `0x80782970` | 204 | Handles the `0x5E` event by flushing the sound slot or arming `+0x139`. |
| `0x80782B20` | 220 | `cobHatenaSwitch` constructor: two vtables and four `0x802DC9A0` adapters. |
| `0x80783300` | 252 | Rebuilds the three `+0x1B0` matrices from position, `+0x434` bias and angles. |

## Signature notes

- `fn_8075fa40(this: usize, f1: f32, f2: f32)` — the sole caller
  `fn_8075f9e0` allocates `0x272` bytes (`0x80162D50`) and calls it with the
  returned pointer in `r3` and its own incoming `f1`/`f2` unchanged.
- `fn_8076e490(this, index, f1, f2, f3)` — `r3`/`r4` plus three `f32`; Ghidra
  prints the floats as `double` because the callee has no prototype.
- `fn_80774d00`/`fn_80775730` return through `_restgpr_27`, so `r3` is a real
  `u32` result even though Ghidra declares both `void` (`fn_80774d00` is tested
  as a boolean at `0x80772370`, `fn_80775730` at `0x807953B8`).
- `fn_80762fe0`, `fn_80774d00` and `fn_80775730` and `fn_80783300` take `this`
  from an incoming `r3` that the `_savegpr_27` prologue preserves; Ghidra
  prints `void`/no arguments for them.

## Deferred

None.

## Caveats

- `0x801C0BD0` is the NW4R transform builder. Its three `f32` position operands
  are kept in Ghidra stack locals that the call printout drops; `fn_80783300`
  reconstructs them from `+0xAC`, `+0xB0 + [ [this+0x434] + 0xC ]`, `+0xB4`
  (same pattern as the previously translated `fn_807828c0`).
- `fn_80782B20` and `fn_80783300` are `medium`: all offsets/constants/calls are
  literal, but several engine constructors and the `+0x434` source object are
  named by address only.
