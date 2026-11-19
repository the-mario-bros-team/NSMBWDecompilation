# Coverage — wave 188, lane E

Module: `game::bases` (REL `d_basesNP`, synthetic base `0x80600000`).
Band: 128–192 bytes. All ten assigned functions are translated.

## Bodies

| Address | Size | Name | Summary |
| --- | --- | --- | --- |
| `0x806F4BE0` | 140 | `fn_806f4be0` | Seeds variant speed from `0x807CC1A0`, four float constants, rebuilds `+0x7C` matrix, arms `+0x41C = 4`, resets `+0x148`. |
| `0x806F4C80` | 184 | `fn_806f4c80` | Advances `+0x100` angle by `0x807CC1B4[variant]`, runs the `0x8006CDE0`/`0x8006CD40` physics pair, ticks `+0x41C` and sets `+0x200`, posts cue `0x1ED`. |
| `0x806F6120` | 136 | `fn_806f6120` | Colon-tail name compare (`0x802E1C40` + `0x802E1AB4`, own name via vtable `+0x20`). |
| `0x806F7430` | 140 | `fn_806f7430` | `0x718`-byte actor constructor (`fn_806f7400`): vtables `0x808020A0`/`0x80802320`/`0x807E3E98`, movers `+0x590`/`+0x5B0`/`+0x5F0`/`+0x628`. |
| `0x806F8950` | 136 | `fn_806f8950` | Byte-identical twin of `fn_806f6120`. |
| `0x806FB7C0` | 188 | `fn_806fb7c0` | Re-aims and re-colours all 17 child slots after a round reset, installs callback `0x80095950`, flushes via `0x80044AC0(0)`. |
| `0x806FDBB0` | 152 | `fn_806fdbb0` | Bumps the per-row counter byte of the audio row table through `0x800CE330`. |
| `0x806FDF80` | 152 | `fn_806fdf80` | Restarts the mini-game round: clears shared fade value, cue `0x5DD`, resets player state, event `3`, `fn_806fd440(this, 1)`. |
| `0x806FE170` | 156 | `fn_806fe170` | Advances `+0x208` to the next resolvable player, wrapping past `3`, and syncs `+0x20C`. |
| `0x806FE3F0` | 140 | `fn_806fe3f0` | Stops the round: clears `+0x1D4`/`+0x21C`, arms `+0x214` with `60`/`150`, resets the player state and sets `+0x94 = 1`. |

## Reused neighbours

- `fn_806fd440`, `fn_806fd510`, `fn_806fd6e0`, `fn_806fd7a0`, `fn_806fd820`,
  `fn_806fe020`, `fn_806fe480`, `fn_806fe810` pin the mini-game panel-manager
  layout (`+0x1D4`/`+0x1D5`/`+0x1D6`/`+0x208`/`+0x20C`/`+0x210`/`+0x214`/`+0x21C`
  and the `0x80429C94`/`0x8042A488`/`0x8042A4A8`/`0x80429DD0` globals).
- `fn_806fb610`, `fn_806fb740`, `fn_806fba90`, `fn_806fc020` pin the segmented
  table layout (`+0x6E8` handles, `+0x884` base angles, `+0x840` scales) and are
  the existing call sites of `fn_806fb7c0`.
- `fn_806f7400` documents the `0x718` allocation consumed by `fn_806f7430`.
- `fn_80700ed0` is the already-translated twin of `fn_806f6120`/`fn_806f8950`.

## Notes / inferred

- `0x807CC408` reads as `0.0` and feeds `[0x80429DD0] + 0x90110`, the float that
  `fn_806fe020` lerps toward `0.5`; `+0x90114` is a flag byte set to `0xB`.
- `806FB7C0`'s export loses both parameters because the body opens with the
  `0x802DCD70` register-save helper. `r3` (receiver) and `r4` (heading) are both
  live; the two existing call sites pass `this` and `read_u16(this + 0x8B4)`.

## Deferred

None.
