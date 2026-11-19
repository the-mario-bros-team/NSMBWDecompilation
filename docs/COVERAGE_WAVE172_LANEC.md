# Coverage - Wave 172, lane C (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane C block) | `game::bases` | 5 | 1,704 | 4 | 1 |

Lane C of wave 172: five REL bodies in the 340-344 byte band, spread across the
`0x8070xxxx`, `0x8073xxxx`, `0x8074xxxx` and `0x8075xxxx` ranges. All five are
translated; nothing is deferred.

## What this pass covers

- **Lift-state advance (2 bodies).** `0x80706510` and `0x80706690` are twins of
  the same unsigned-16-bit height follower: `+0x3FC -= +0x400`, clamp against
  `+0x3FE`, feed the step to `0x80706230` (scales it into `+0xE8`), advance the
  rotation word `+0x100`, emit sound `0x2F7`/`0x2F6`, and reset when flag bits
  of `+0x274` are set. The first body adds the positive-step interface dispatch
  (`0x8082F3A0`) and the `+0x40C` hold counter that snaps the floor to `-512`.
- **Burst-table spawn (1 body).** `0x80733C30` indexes the three parallel tables
  at `0x807C_D820`/`0x807C_D830`/`0x807C_D858` by `2 * +0x634 + u8(+0x348)`,
  builds the `0x30000000 | mode<<6 | param<<4` effect argument for `0x80064610`
  (id `0x85`), then spawns sound `0x2A2` and the `Wm_en_burst_ss` layout effect
  (`0x8080_D618`) before running `0x80733EC0`.
- **`cobKoopaShip` frame body (1 body).** `0x8075C550` ticks and tears down the
  node at `+0x1EC`, dispatches through the class callback `+0x60`/`+0x60` or the
  `0x807C_EC20` descriptor helper, writes the actor position into the scene
  object named by `0x8081_45E8`, ticks `+0x1AC`/`+0x1B8` and rebuilds the
  transform.
- **Rail scan mask (1 body).** `0x80746910` samples `0x8008E890`'s four-float
  bounds record (count = `bounds[3] * 0.03125`, step `32.0`), walks downwards
  from `bounds[1]` and writes a `0`/`1` validity word per point, using the camera
  ceiling `0x8042_9E00 + 0x3C` and floor `ceiling - +0x44`, with
  [`fn_80746560`] as the tie-breaker.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80706510` | `fn_80706510` | 340 | high |
| `0x80706690` | `fn_80706690` | 340 | high |
| `0x80733C30` | `fn_80733c30` | 340 | high |
| `0x8075C550` | `fn_8075c550` | 340 | medium |
| `0x80746910` | `fn_80746910` | 344 | high |

## Corrections and notes

- `0x80706510`/`0x80706690`: the exports assign the untruncated 32-bit
  subtraction to the 16-bit field. The raw sequence is `subf`, `sth`, `extsh`,
  `cmp` (0x8070_6588..0x8070_6594 / 0x8070_66BC..0x8070_66C8), so the clamp compares
  the **truncated** value. Both bodies model the `extsh` explicitly. The raw
  `lha`/`sth` at `0x8070_65AC` also reloads `+0x3FC` after `0x80706230`; that
  reload is preserved.
- `0x80706510`: the `andi. 0x2A` block stores `+0x3FE = 0`, `+0x3FC = 0`,
  `+0xE8 = 0.0` and `+0xAC = +0xB8`. Ghidra names the two float constants
  `DAT_807cc6c4`/`DAT_807cc6e8`; they are `0.0` and `-1.0` in the ELF. The
  `-1.0` is passed by address to `0x8006FA40`, not by value.
- `0x80706690`: the final `0x8082F360` dispatch fires when `& 0x2A != 0` **or**
  `+0x3FC == 0`; Ghidra renders these as an if/else pair but the raw `beq` at
  0x8070_6768 and the `beq` at 0x8070_679C make the two paths identical, so the
  body collapses them into one condition and re-reads `+0x3FC` as the raw does.
- `0x80733C30`: `rlwinm r6,r6,1,0,30` is `field * 2` (the shifted-in bit is
  already clear) and `rlwinm r0,r0,4,0,27` is `(parameter << 4) & 0xFFFFFFF0`;
  both masks are modelled. The three tables are rows of `0x807C_D7C8`:
  X-offset `+0x58` (12.0 for row 0), Y-offset `+0x68` (20.0 for row 0), parameter
  `+0x90` (0), and the two scalars `+0x34` (`8.0`) and `+0x44` (`0.0`).
- `0x8075C550`: `0x807A_7F80` is declared `fn(void)` in `bases.rs`, but its raw
  words forward `r3` to `0x801015E0`, so the body passes the manager explicitly
  through `abi_call1!`. The `0x8081_45E8` operand is a **pointer** to the
  `cobKoopaShip` string at `0x8081_45D8`, so it is loaded, not used as an address.
  `0x802D_CBFC` is the module's unresolved indirect-jump stub (same convention as
  `fn_80762890`); its hidden `r12` descriptor is documented but not modelled,
  which is why this body is `medium`.
- `0x80746910`:
  - The export's GQR0 "dequantise" prologue/epilogue is compiler noise; GQR0 is
    `0`, so the `psq_st`/`psq_l` pairs are plain `f32` saves.
  - `0x802D_CD70`/`0x802D_CDBC` are `_savegpr_26`/`_restgpr_26` fragments whose
    `r3`/`r4` survive; the body takes `actor` and `out` directly.
  - Ghidra's `local_5c` loop test is an aliasing artefact: the `fcmpo` operands
    come from `f2`, which is carried across iterations (0x8074_6A28). The window
    tests therefore use the decremented sample, which is what the body does.
  - The loop bound is an `fctiwz` (`0xFC00001E`) conversion of
    `bounds[3] * 0.03125` stored through `stfd`/`lwz`; it is a signed count.
  - The `0x8008E890` first argument is `stage_view()`, the existing helper for
    `0x8042_9E98 + u8(*(0x8042_A1C8) + 0x120E) * 0x3B0` with the `+0x2C == 0`
    null test.

## Verification

- The lane block type-checks against stub `platform`/`abi_call*` definitions
  (`rustc --crate-type lib`, exit 0); it is not yet compiled into `bases.rs`
  because lane blocks are appended by the coordinator.
- Every float/u32 constant was re-read from `work/rels/elf/d_basesNP.elf`:
  `0x807C_C6E8 = -1.0`, `0x807C_C6C4 = 0.0`, `0x807C_D7FC = 8.0`,
  `0x807C_D80C = 0.0`, `0x807C_D820 = 12.0`,
  `0x807C_D830 = 20.0`, `0x807C_D850 = 5500.0`, `0x807C_D858 = 0`,
  `0x807C_E3C0 = 32.0`, `0x807C_E464 = 0.03125`.
- All five evidence exports exist under `work/ghidra/rel-d_basesNP/export/` and
  their byte sizes match `functions.tsv` (340/340/340/340/344).
