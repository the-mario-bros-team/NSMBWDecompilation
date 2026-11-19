# Coverage - Wave 189 lane D (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 1652 | 10 | 0 | 0 |

Ten bodies, all from the 128-192 byte band. Source of the bodies:
`work/wave189/laneD_block.rs`; per-function ledger:
`ledger/agents/wave189-laned.csv`.

## What this pass covers

- **Model transform rebuild (1 body).** `0x80743AF0` (136) is the same shape as
  the already-translated `fn_806b1970`, minus the angle install: copy
  `+0xAC..+0xB4` into a stack vector, `0x80064AF0(vector, 0, 1)` (the angle
  source is the null pointer here), build the 48-byte 3x4 transform with
  `0x801C0BD0`, then `0x8016A170(+0x3F0, matrix)`,
  `0x8016A150(+0x3F0, +0xDC)` and `0x8016A1A0(+0x3F0, 0)`.
- **Ground-marker emitter (1 body).** `0x80745D10` (176) starts at
  `+0xAC + 16.0` (`0x807CE378`), steps `+16.0`, and calls
  `0x80077860(0x80429DD0, x_trunc & 0xFFFF, (-y)_trunc & 0xFFFF, 0, 0x93)`
  once per `count`. The `fctiwz` pairs are the two low-halfword arguments; the
  decompiler's `ldexpf`/`unaff_GQR0` blocks are the usual compiler noise.
- **Ground snap + effect cue (1 body).** `0x80747120` (192) copies a three-float
  point, probes the floor with
  `call_f32x2_u32x2(0x80075270, cam_x, cam_y, +0x38F, &floor)` (the same shape
  as `fn_8074b770`), clamps `y` to `floor - 40.0` (`0x807CE480`) on success and
  posts `0x80064610(0x145, 0x10001010, point, 0, 0)`.
- **Position blend (1 body).** `0x80749C50` (164) calls `fn_80749c00` twice.
  Ghidra prints one argument per call, but the raw words keep `r4 = param_2`
  and `r5 = param_3` / `param_4`, so both sample player positions; the result is
  `(1.0 - 0.5) * first + 0.5 * second` on x/y (`0x807CE584`/`0x807CE588`) and
  `actor + 0xB4` on z.
- **Per-slot travel odometer (1 body).** `0x8074C990` (164) takes the class byte
  from vtable slot `+0x6C` through `actor_method0(player, 0x6C)`, sign-extends it
  (`lbz` + `extsb`), and when `fn_8074c280(this, player) != 0` adds
  `abs(+0x3F8 - player[+0xAC])` to `+0x408`, then refreshes `+0x3F8`.
- **Wobble offsets (1 body).** `0x80752B90` (156) samples the unsigned halfword
  at `+0x4BA` (GQR3 type 5), scales by `0x807CE84C = 0.00390625`, stores
  `2.0 * cos` at `+0x498` and `2.0 * sin` at `+0x49C` (`0x807CE850 = 2.0`) and
  advances the angle by `0x400` (one turn).
- **Availability mask (1 body).** `0x80756FD0` (136) is argument-bearing despite
  Ghidra's `void`: `_savegpr` keeps `r3`/`r4`, which the body moves to
  `r27`/`r28`. It probes `0x80080490(this + 0x524, slot)` for `slot = 0..4`,
  ORs the matching bits of `0x807CE9D0` (`1, 2, 4, 8`), then stores
  `0x800440C0(this, mask)` through `r28`.
- **HUD character install (1 body).** `0x80758310` (184) selects with
  `(word[+0x4] & 0xF) * 4`, dereferences the parallel tables at `0x807CEA70`
  (`[0,1,2,3]`, resource id) and `0x807CEA80` (`[0,1,2,3]`, index into the
  global `0x80354E70`), stores the resolved value at `+0x238`, allocates `0xC`
  bytes through `0x802B9210`, wraps them with `0x800D6CC0`, configures with
  `0x800D6DF0(obj, nibble, value & 0xFF, 2)`, and calls the new object's virtual
  `+0x5C` with `(1.2, 10.0, 0.0)` (`0x807CEA90`, `0x807CEA94`, `0x807CEA5C`).
  `0x80354E70` is main-DOL data (outside the REL), so it stays a literal runtime
  address.
- **HUD/model constructor (1 body).** `0x8075ADA0` (184) chains
  `0x800F5FD0` and `0x800F6F40(+0x184)`, swaps `+0x60` from `0x8031BF68` to
  `0x80814410`, builds `+0x6E8`, `+0x704`, `+0x744`, `+0x788`, installs
  `0x807E3E98` / `0x803295D8` / `0x80329578`, clears `+0x780`, `+0x784`,
  `+0x7A4`, and returns `this`.
- **Name release sweep (1 body).** `0x8075BF70` (160) is the teardown sibling of
  the translated `fn_8075bd20`: for `pass, slot in 0..2` it reads an entry with
  `0x800BBC10(names, scene[+0x3388], slot + pass*2, &entry)` and, when
  `entry[1] >= 0`, releases it with
  `0x800BBBD0(names, scene[+0x3388], index, scene[+0x338C], -1)`.

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_*` /
  `write_*`, `abi_call1..5`, `call_f32x3_u32`, `call_f32x3_u32x2`,
  `call_f32x2_u32x2`, `call_f32_0`, `f32_of_f32`, `actor_method0` and the
  already-translated `fn_80749c00` / `fn_8074c280`, all present in
  `src/game/bases.rs`.
- `0x80749C50` reuses `fn_80749c00(out, _unused, player_index)` directly, so the
  blend does not go through raw `abi_call`.
- All other callees (`0x80064AF0`, `0x801C0BD0`, `0x8016A170`/`0x150`/`0x1A0`,
  `0x80077860`, `0x80075270`, `0x800B30C0`/`0x800B30E0`, `0x80064610`,
  `0x80237BD0`/`0x80237C40`, `0x80080490`, `0x800440C0`, `0x802B9210`,
  `0x800D6CC0`/`0x800D6DF0`, `0x800BBC10`/`0x800BBBD0`) are used with the same
  ABI as in neighbouring translated bodies, so nothing was guessed from the
  decompiler's lost prototypes.
- `0x802DCD74` / `0x802DCDC0` are `_savegpr` / `_restgpr` pairs and are never
  emitted.

## Verification

- The block was appended to a scratch copy of the crate and `cargo check`
  passes with no errors or new warnings.
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf` via
  `ppc159.js`; each body's byte size matches `functions.tsv`
  (136, 176, 192, 164, 164, 156, 136, 184, 184, 160).
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave189` reports the bodies as "not yet in
  `src/game/bases.rs`", which is the expected pre-merge state - the coordinator
  appends the block.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
