# Coverage - wave 195, lane E

Lane E of wave 195 covers the `0x80736890..0x80765980` band of `d_basesNP`
(`game::bases`). All ten assigned functions were read from both the Ghidra
export and the raw instruction stream; all ten were translated.

## Translated (10)

| Address | Size | Rust | Confidence | Notes |
| --- | --- | --- | --- | --- |
| `0x80736890` | 284 | `fn_80736890` | high | Downward collision sweep; `16.0 * steps` stored at `+0x64C`. |
| `0x80737270` | 284 | `fn_80737270` | high | Mirrored sideways sweep of the same shape. |
| `0x80738460` | 276 | `fn_80738460` | medium | Position resolver; raw `+0xAC` vector or a matrix translation. |
| `0x807435A0` | 288 | `fn_807435a0` | high | Factory for the `0x5F8`-byte actor (vtable `0x80810310`). |
| `0x80746400` | 268 | `fn_80746400` | medium | Rectangle overlap test over the four player actors. |
| `0x807529B0` | 260 | `fn_807529b0` | medium | Rebuilds the `+0x4B8` player mask and resets `+0x4A8`. |
| `0x807539E0` | 284 | `fn_807539e0` | high | Timer/`+0xB0` resample plus tilt vtable drive at `+0x394`. |
| `0x80764550` | 292 | `fn_80764550` | high | Constructor: five `0x802DC9A0` array blocks. |
| `0x807646D0` | 284 | `fn_807646d0` | high | Destructor matching `fn_80764550`, optional free. |
| `0x80765980` | 272 | `fn_80765980` | medium | `character/SV` load and five animation bindings. |

## Deferred

None.

## Decoding notes

- `0x802DCD6C`/`0x802DCD70`/`0x802DCD74` and `0x802DCDB8`/`0x802DCDBC` are
  compiler register save/restore helpers; the incoming `r3` survives, so
  `0x80736890`, `0x80737270`, `0x807529B0` and `0x80765980` take a `this`
  argument that Ghidra typed as `void`.
- `0x80070BA0` is called with `f1` = first probe axis and `f2` = second probe
  axis, both established by register reads at the call sites.
- Constants re-read from the ELF: `0x807CDA48 = 16.0`, `0x807CDA54 = 24.0`,
  `0x807CDA58 = 8.0`, `0x807CDA60 = 4503601774854144.0` (the `2^52` integer
  magic), `0x807CDB00 = 16.0`, `0x807CDB04 = 24.0`, `0x807CDB08 = 8.0`,
  `0x807CDBE4 = 0.0`, `0x807CE2F0 = 0.0`, `0x807CE840 = {1,2,4,8}`.
- The `0x802DCD74` register-save helper is also why the void `FUN_807529B0`
  writes through `r27`; `r27` is the surviving argument register, not a
  return value.
