# Wave 194 lane E coverage (`game::bases`)

Ten functions in the `0x80789990..0x80798E20` band were decoded from the raw
PowerPC words of `work/rels/elf/d_basesNP.elf` and checked against the Ghidra
exports. All ten are translated; none are deferred.

## Translated

| Address | Size | Body |
| --- | --- | --- |
| `0x80789990` | 208 | Publishes a ride anchor to `0x8042A284` in modes `0`/`1`/`2` (`+0x604` = `6`/`7`/`2`). |
| `0x80789D90` | 200 | Returns `1` when `0x801015E0(0x8042A1AC)` is one of the eight ids at `0x807CFAA0`. |
| `0x8078DB80` | 240 | Builds the `daWmDirector` `+0x188` range block and its `+0x1BC`/`+0x1C0`/`+0x1C4` members. |
| `0x8078ED10` | 224 | Director phase: countdown `+0x1B4`, player latch, state `0x12`/`0x1C`, finished `1`. |
| `0x8078EDF0` | 240 | Key `4`/`9` gate with the two one-shot latches `0x8042A25A`/`0x8042A25C`. |
| `0x8078F350` | 196 | Director `0xA8` phase: virtual `+0x64` gate, state `0x1B`, then virtual `+0x68`. |
| `0x8078FB80` | 256 | Mode-`4` ride animation start/finish, `0x6E` silence sound, `+0x139` finished byte. |
| `0x80796CF0` | 196 | Side select from `+0x258`/`+0x259` bits into `+0x288`, with the `+0x300` ramp. |
| `0x807985C0` | 232 | Ride anchor from `+0x2A0` with the `60.0` bias; state `5` and `0x807986B0` on arrival. |
| `0x80798E20` | 236 | Mode `0`/`1`/`2..5` speed and angle limits; `7.0`, `-7.0`, `-20.0`, `±0.5`. |

## Signature notes

- `fn_80789990(this: usize, mode: i32, point: usize, extra: u32)` — the existing
  caller [`fn_80789790`] passes `(this, 0|1, &vector, 0)` at `0x8078_9990`; mode
  `1` never reads `point`.
- `fn_80798e20(this: usize, mode: i32)` — `FUN_80799200` passes
  `(param_1, param_2)`, and `FUN_807982B0` passes `*(this + 0x23C)` as the mode.
- `fn_8078edf0(this: usize) -> u32` — `0x8078ECC0` calls it without setting
  `r3`, so the parameter is the actor already in `r3`; both internal probes are
  declared no-argument and do not consume it.
- `fn_8078db80(this)`, `fn_8078ed10(this) -> u32`, `fn_8078f350(this)`,
  `fn_8078fb80(this, mode, active)`, `fn_80796cf0(this, active)` and
  `fn_807985c0(this)` keep the exported prototypes.

## Deferred

None.

## Caveats

- `fn_8078FB80` calls `0x807900B0` with `this` in `r3`. The already-translated
  wrapper `fn_807900b0()` recovers `r3` through the `0x802DCD68` save helper, so
  the call is emitted as `abi_call1!(0x8079_00B0, this)` instead of a bare Rust
  call that would not set `r3`.
- `fn_807985C0` emits `0x8006CC00` as `platform::call0` following this file's
  existing convention; the raw `r3` still holds `this` at that point.
- `fn_80798E20` needs a single-precision return that no existing helper exposes
  with `(f32, u32, u32, u32)`; the body uses an inline `extern "C"` transmute
  for `0x800FDF90`, the same pattern already used for `0x801637D0` in this file.
- `0x8031D39C..D3AC` are main.dol data; the raw `addi` displacements resolve to
  `0x8031D3A0 = 7.0`, `0x8031D3A8 = -7.0`, `0x8031D3AC = -20.0`, matching
  `fn_8079b3f0`.
