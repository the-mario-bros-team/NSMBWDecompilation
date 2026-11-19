# Coverage - Wave 190 lane D (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 1704 | 9 | 1 | 0 |

Ten bodies, all from the 128-192 byte band. Source of the bodies:
`work/wave190/laneD_block.rs`; per-function ledger:
`ledger/agents/wave190-laned.csv`.

This lane is the course-tree node section around `0x8078DA..` and
`0x8079AF..0x8079BA`: the two phase handlers, the node constructor and
transform refresh, and the six "find my record in the scene" helpers. All of
them share the same object layout: actor transform at `+0xAC..+0xB4` (with the
matrix at `+0x7C`), child chain at `+0x184`, scene handle at `+0x22C`, and the
scene singleton at `0x8042A18C`.

## What this pass covers

- **Handle release/copy (1 body).** `0x8078DAC0` (164) releases the handle at
  `+0x1C0` through vtable slot `+0x14` when `+0x1A8` is set. The raw words keep
  the `+0x1BC` handle on the "player byte `+0x394` clear" side of the `beq` at
  `0x8078DAFC`; on the other side the second word of `+0x1C0` is copied into
  `+0x1C4` and that object is released. Returns `1`.
- **Second phase machine (1 body).** `0x8078F0E0` (184) mirrors the translated
  `fn_8078e6d0`: `+0x1D0` disable byte, `+0x1B4` countdown (`cmpwi` + `bc 4,1`
  is `count <= 0`, i.e. Ghidra's `< 1`), then the not-started branch writes the
  player's `+0x547 = 1` / `+0x55C = 0x11`, calls `fn_8078e680(this)` and sets
  `+0x1B0`; the started branch calls `fn_8078e690(this)` and returns `1` once
  the player byte clears. `0x80834470` is a pointer global (`lwz` then `+0x547`),
  reloaded in the second branch.
- **Node constructor (1 body).** `0x8078F500` (172) chains `0x800F5FD0`, writes
  the `-1` id at `+0x184` and the `0x8081ABF8` vtable at `+0x60`, initialises
  `+0x188` (`0x80069020`), `+0x1A4` (`0x8016A2F0`), `+0x1B0` (`0x80168D10`) and
  `+0x1F4` (`0x8016A5E0`), and installs `0x807E3E98` at `+0x1B0`,
  `0x803295D8` then `0x80329578` at `+0x1E8`. Returns `this`.
- **Transform refresh (1 body).** `0x8078FAD0` (176) is character-for-character
  `fn_8075aa50` with the animation object moved from `+0x1A8` to `+0x1A4`:
  `call_f32x3_u32(0x801C0BD0, x, y, z, this+0x7C)`, three two-halfword slots
  from the `lha` reads at `+0x100`/`+0x102`/`+0x104` into `0x8016EF50`, then
  `0x8016A170`/`0x8016A150`/`0x8016A1A0`.
- **Value apply (1 body).** `0x8079AFB0` (172) resolves `+0x22C` with
  `0x80100200`, copies the three floats into a second stack block, and picks the
  fourth argument for `fn_8079b060`: for kind `6` the node's `+0x23C` word, else
  `0x800FC1F0(record, source)`. This is the only medium body: both callee
  contracts are address-only.
- **Record lookup (1 body).** `0x8079B260` (152) is declared `void` but sets
  `r3` on both exits and recovers its argument from the `_savegpr` pair (r27),
  so it takes `this`. Gate `0x800FC7F0()`, then scan `index < 0x807CEBC4 (= 2)`
  with `0x800BBC10(manager, world, index, &record)`; on `record[1] == +0x22C`
  it stores the index at `manager + 0x9C` and returns `1`.
- **Handle install (1 body).** `0x8079B5B0` (172) looks the tag `0x8081B874`
  ("W1X0") up with `0x801002C0`, stores the handle at `+0x22C`, resolves its
  point into `+0xAC..+0xB4`, pushes the handle through `fn_80781cb0` for every
  node from `+0x184`, sets `+0x288`/`+0x28C` to `1`, copies the handle to
  `+0x238` and calls `0x80797190(this)`.
- **Scene probes (1 body).** `0x8079B660` (160) tests the handle with
  `0x800F57C0(base, handle, 0x400)` then `0x800F57C0(base, handle, 0x800)`,
  where `base = scene + scene[+0x338C] * 0x16518 + 0x3394` (the same expression
  as `fn_80799860`; the stride is the `lis 0x1` / `addi 0x6518` pair).
- **Tag test (1 body).** `0x8079B720` (160) returns `0` unless world byte
  `0x8042A24D == 2` and map byte `0x8042A24E == 1`; then it compares the name
  from `0x800F56B0(base, +0x22C)` against the 4-byte tag `0x8081B87C` ("F001")
  with `0x802E1BD0(name, tag, 4)` and inverts the result.
- **Radius-filtered target (1 body).** `0x8079BA00` (192) calls
  `fn_8079bac0(this, +0x230)`, then measures two `{x, 0, z}` planes (the
  `0x807D0160` 0.0 word fills y) with `0x801C1650` + the `0x802BE470` square
  root and keeps the target only when the distance is below
  `actor_method0_f32(target, 0x78)`. The `ldexpf`/`unaff_GQR0` blocks in the
  export are the `psq_st`/`psq_l` f31 save/restore pair.

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_*` /
  `write_*`, `abi_call1..4`, `call_f32x3_u32`, `call0_ret`, `call0_f32`,
  `vmethod0`, `actor_method0_f32`, and the already-translated
  `fn_8078e680`, `fn_8078e690`, `fn_80781cb0`, `fn_8079b060` and
  `fn_8079bac0`, all present in `src/game/bases.rs`.
- `fn_8079afb0` calls `fn_8079b060(this, point_ptr, source, argument)` with the
  established argument order of that body (its `r4` is unused, `r5` is the
  record and `r6` the applied argument).
- `fn_8079b5b0` pushes `handle` (a `u32`) into `fn_80781cb0`, whose parameter is
  already `handle: u32`.
- `0x802DCD74` / `0x802DCDC0` in `0x8079B260` are `_savegpr` / `_restgpr` and
  are never emitted.
- `0x8081B874` = `W1X0` and `0x8081B87C` = `F001` were read from the ELF; both
  are 4-byte tags, not strings with a terminator, which is why the caller passes
  the explicit length `4` to `0x802E1BD0`.

## Verification

- The block was appended to a scratch copy of the crate and `cargo check`
  passes with no errors and no new warnings.
- Every raw word was re-read from `work/rels/elf/d_basesNP.elf` via
  `ppc159.js`; each body's byte size matches `functions.tsv`
  (164, 184, 172, 176, 172, 152, 172, 160, 160, 192).
- All ten evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave190` reports the ten bodies as "not yet in
  `src/game/bases.rs`", which is the expected pre-merge state - the coordinator
  appends the block.

## Deferred

None. All ten bodies were proven from the raw instruction stream.
