# Coverage - wave 199 lane E (`game::bases`, `0x8076EF80`, `0x807962B0`)

Lane E of wave 199 translated both giant bodies from `rel/d_basesNP.elf`
(bucket `0x80600000`, synthetic analysis base; not a runtime address). Both are
`translated` at `high` confidence. Nothing was deferred, and no part of
`0x8076EF80` was left out.

Block: `work/wave199/laneE_block.rs`. Ledger rows:
`ledger/agents/wave199-lanee.csv`.

Verification: the block was appended to a scratch copy of
`src/game/bases.rs` (`%TEMP%\opencode\w199lanee`) and `cargo check --lib`
finishes with exit code `0` and no warning inside the appended range. The merge
watcher picked this block up while the lane was still verifying, so the real
`src/game/bases.rs` now already carries both bodies (identical text); a scratch
check of that merged snapshot is also clean, and `src/game/bases.rs` was never
edited by hand from this lane.

## Bodies

| Address | Bytes | Name | Description | Confidence |
| --- | --- | --- | --- | --- |
| `0x8076EF80` | 2544 | `fn_8076ef80` | Two-phase airship-boarding state table for `+0x27C`-style records: `flag != 0` runs the install half (lazy `0x80832800`/`0x80832880` offset tables, ship spawn, target/angle, per-state state word), then the per-frame half drives the `0x28`/`0x29`/`0x2B`/`0x2F` states, the camera anchor at `0x8042A284+0x5F4` and the three-step `+0x398` walking sub-machine. | high |
| `0x807962B0` | 1428 | `fn_807962b0` | Course-record start-up: resolves `+0x22C` (saved record `[0x80429F7C]+0x44` or the `0x801002C0` result), places the actor through `0x80100200`, then walks the character/area ladder (`0x800F9840`/`0x800FA9C0`), the `0x8079B1D0`/`0x8079B700`/`0x8079B240` gates and the `0x800FC7F0` exit to pick `+0x27C`, writes the `+0xDC` scale triple and publishes the previous position and the `0x80775080` record. | high |

## Decoding notes

- **`0x8076EF80` frames.** `bl 0x802DCD70` / `bl 0x802DCDBC` are the
  `_savegpr`/`_restgpr` compiler helpers and are not emitted. `r27 = this`,
  `r28 = state`, `r29 = *(u32*)0x8042A284`, `r30 = 0x808327D0`,
  `r31 = 0x807CF290`. The `cmpi r4,-1` at `0x8076EF94` makes `state == -1` an
  immediate return, before either phase.
- **Table fill.** Both ten-triple tables are copied word for word from their REL
  constants (`0x807CF2AC`, `0x807CF2A0`, `0x807CF2B0`, `0x807CF2AC`,
  `0x807CF2B4`, `0x807CF2B8`, `0x807CF2A4`, `0x807CF2A0`, `0x807CF2BC`, then
  seven `-500/100/50` triples; second table `0x807CF2C0`/`F2A0`/`F2C4`,
  `0x807CF2C8`/`F2B4`/`F2CC`, `0x807CF2D0`/`F2A0`/`F2D4`, then seven
  `-180/100/-50` triples). No float literal was invented; the block copies the
  bit patterns from the listed addresses.
- **`0x80816C0E` asymmetry.** The install half reads the scene cell twice. The
  first read at `0x8076F164` selects the explicit `0x80816C1C/20/24` triple when
  it is non-zero and the **second** table (`0x80832880`) otherwise; the second
  read at `0x8076F224` selects `0x80816C10/14/18` when non-zero and the first
  table (`0x80832800`) otherwise. The `lfsx` at `0x8076F1DC` is decoded as
  `table + index*12 + 0`, i.e. the x component.
- **Sub-state `1` of `0x2B`.** The `0x80816C0E`-independent scene cell
  `*(u32*)0x8042A480` (then `+4`, a `u16`) feeds the compiler's
  `0x43300000` int-to-double conversion, so the bound is
  `200.0 + (count as f64 - 176.0) as f32` (`0x807CF2A0`, `0x807CF310`).
- **Helper arities already in `bases.rs`.** `fn_80775080`, `fn_80775100`,
  `fn_80775150`, `fn_807c3b00` and `fn_807c3ac0` take only `r3`; the original
  also passes the record id in `r4`, which those bodies do not read, so the call
  sites pass the scene/effect pointer only. `fn_8076fef0(r3)` and
  `fn_8076ff10(r3)` forward `r3` into the third argument of `0x801006E0` /
  `0x801006C0`, so the block hands them the 12-byte stack buffer address the
  original leaves in `r3`. `fn_807700f0` receives the actor in `r3` and ignores
  the buffer the export prints as a parameter.
- **Camera-threshold compare at `0x8076F860`/`0x8076F89C`.** `fc030040` and
  `fc010040` are `fcmpo` with the operands in bits 16-20 / 11-15, and `bc 4,1`
  / `bc 4,0` branch when the condition is false, so the gates are
  `probe.x > limit` and `probe.y < -200.0`.
- **`0x807962B0` id source.** `0x801002C0` is `void` in `src/engine/globals.rs`,
  but the caller stores the `r3` left by its tail call `0x800F53A0`
  (`stw r3,0x22C(r27)` at `0x80796328`). The block keeps that value by reading
  the `abi_call2!` return, which is the same register, so no value is invented.
- **`0x807962B0` effect record.** The local passed to `0x800F6410` is six
  floats: `{pos - anchor, anchor}` (`0x8..0x1C` of the frame); the duplicated
  stores at `0x2C`/`0x38` are dead and are not reproduced. `this+0xDC/+0xE0/
  +0xE4` use the `0x8031D3B4` (main.dol `0x8031D39C + 0x18`) scale, read at
  runtime rather than guessed.
- **`0x807962B0` `0x800FB040` call sites.** The first one reuses `r3 = 7` /
  `r4 = 0x25` left over from the two compares (`0x8076F6D0..0x8076F6E4`), so the
  block passes `(7, 0x25)`; the second is the explicit `(0, 2)`.

## Deferred

None.
