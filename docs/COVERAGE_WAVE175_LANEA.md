# Coverage - Wave 175 lane A (`game::bases`, four 392-byte bodies + one 396)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane A block) | `game::bases` | 5 | 1,964 | 4 | 1 |

Five bodies. Source of the bodies: `work/wave175/laneA_block.rs`; per-function
ledger: `ledger/agents/wave175-lanea.csv`.

## What this pass covers

- **Multi-pane row writer (1 body, `0x806787C0`).** The actor's row end is
  `max(+0x4B0, +0x4AC)`; with `span = end - 0.5` and `travel = +0xDC - 0.5` the
  step is `16.0 * travel / span` when `span >= 0.25` and `0.0` otherwise. The
  `+0x530` panes at `+0x3F0` (12-byte stride) alternate between
  `+0xAC + 16.0*travel` and `+0xAC - 16.0*travel` and converge by
  `(index >> 1) * step`; each pane is built from `0x801C0BD0`, copied through
  `0x8016A170`, scaled by `1.0` with `0x8016A130`, ticked through slot `+0x14`,
  and the row advances `6.0` in z. Returns `1`.
- **Lift-probe scanner (1 body, `0x806C7890`).** Walks the `+0x440` list;
  kind `1`/`2` objects accumulate `node[+0x24] - 60.0`, raise the `+0x49C`
  timer to `30`, and set the caller's flag bits `1`/`2` from `+0x10D4`; a
  non-negative `+0x49F` byte gates the first bit; `0x80056CF0(object, 0x1C)`
  sets the "adopted" out-word. Kind `3` objects that pass
  `0x8006EF40(object+0x1EC, 0)` receive `+0x459 = (node[+0x24] - 60.0 < 0.0)`
  and a `fn_806C7A70` attach. Returns the kind-`1`/`2` count.
- **`daTorchiLight_c` centre-fire loader (1 body, `0x80743790`).** Opens the
  five-argument accessor `0x800690C0(+0x3D0, -1, *0x80377C48, 0, 0x20)`, loads
  `g3d/center_fire.brres`/`center_fire` into `+0x3EC`, binds the model at
  `+0x3F0` through `0x80169D90(..., 0x32C, 1, 0)`/`0x80064BF0`, attaches the
  `obj_torch` pane at `+0x430`/`+0x434` (`0x8023A270` + `0x80167420` +
  `0x80167800` + `0x80169F80`), attaches the second `obj_torch` clip at
  `+0x460` (`0x8023A190` + `0x80165C30` + `0x80165FD0`), dispatches model slot
  `+0x18` with the `+0x460` part, and closes with `0x800690E0`.
- **World-select scene reset (1 body, `0x807B9F40`).** Unpacks the request word
  at `+4` into the world byte `0x8042A24D`, the course byte `0x8042A24E` and
  the `0x800B0DB0` selector, clears flag bit `0` of `0x8042_9F80`, and on a
  world change tears down the scene manager `*0x80429F7C`
  (`+0x40/+0x44/+0x48/+0x50/+0x54/+0x60/+0x6C/+0x6D`, `0x8083_2EB0`,
  `fn_807BAF10`). Then clears `+0x5C`/`+0x3B3`/`+0xAFC`, re-clears
  `0x8083_2EB0` on a course change, resets the four player managers from
  `*0x8042A028`, stores `0x800FBDE0()` into `0x8042A259`, runs `fn_807BA5D0`
  and the `0x800E4830`/`0x800E4850`/`0x80102F10`/`0x80103E40`/`0x8019C9F0`/
  `0x80060EF0`/`0x801BE2F0` boot chain. Returns `1`.
- **Title-star spawn on the five map panes (1 body, `0x8060FB50`).** Guarded by
  `+0xE4F != 0`, `+0xE50 == 0` and `camera(+0xDDC)[+0x48] >= 1.0`; for each of
  the five slots whose `0xE18[i]` object has bit `0` of `+0xBB` set, copies the
  `0xE2C[i]` matrix into a 4x4 record (`w0 = +0x90`, `w1 = +0xA0`,
  `w4..w15 = +0x84..+0xB0`, duplicates at `w7`/`w11`, `w2 = 0.0`), prepares it
  with `0x800B37E0`, and calls pane slot `+0xA0` with `Wm_2d_titlestar01`
  (`0x20C + i*0x128`) and `Wm_2d_titlestar02` (`0x7D4 + i*0x128`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806787C0` | `fn_806787c0` | 392 | high |
| `0x806C7890` | `fn_806c7890` | 392 | high |
| `0x80743790` | `fn_80743790` | 392 | high |
| `0x807B9F40` | `fn_807b9f40` | 392 | medium |
| `0x8060FB50` | `fn_8060fb50` | 396 | high |

## Deferred

None. All five assigned bodies are translated.

## Corrections and notes

- **The three earlier deferrals were GQR false alarms.** Waves 62/103/108/109
  rejected `0x8060FB50`, `0x807B9F40`, `0x806787C0` because the export opens
  with `unaff_GQR0`/`ldexpf` text. Every `psq_st`/`psq_l` in these bodies is a
  register-save slot: `0x806787C0` saves `f27..f31` at `r1+0x38..0x78`,
  `0x8060FB50` saves `f31` at `r1+0x78`, and `0x807B9F40` has no paired-single
  access at all. On GQR0 = `0x00000000` these are plain scalar `f32` accesses
  (brief, "Decoding facts"), so no vector semantics were dropped.
- **Four dropped `this` arguments.** `0x806787C0` opens with `bl 0x802DCD70`
  (`_savegpr_26` class), `0x806C7890` with `0x802DCD64`, `0x807B9F40` with
  `0x802DCD74` and `0x8060FB50` with `0x802DCD68`; `r3` survives each thunk, so
  the exports' `void` signatures are wrong and all four bodies take
  `this: usize`. `0x80743790` is a normal `r3` argument (its only caller,
  `FUN_807436C0`, passes its own receiver).
- **`0x806787C0` `fcmpo` operand order.** The raw word at `0x80678810` is
  `fcmpu cr0,f2,f0` (fA = `+0x4AC`, fB = `+0x4B0`) followed by `bc 4,1` =
  "branch if GT clear". The compare therefore keeps the larger of the two, as
  Ghidra prints; reading the operands as `f0,f2` would invert it to `min`.
  The same `fA`/`fB` reading confirms the `0.25` window at `0x80678834`.
- **`0x806787C0` integer-to-float idiom.** `stw`/`lfd 0x43300000`/`fsubs`
  against `0x807C93A8 = 2^52` is the unsigned `index >> 1 -> f32` conversion;
  the subtraction is exact in double before rounding, so the body writes
  `(index >> 1) as f32 * step`.
- **`0x806C7890` out-parameter order is proven by two callers.** `fn_806c7ba0`
  passes `sp+0x10` (float sum), `sp+0x0C` (adopted word) and `sp+0x08` (flag
  word); `fn_806c7c20` reads `local_28 & 1`/`& 2` for flags and `local_24 != 0`
  for adoption. The return value is the probe count both callers compare
  against `+0x49A/+0x49B`.
- **`0x806C7890` `+0x49F` gate.** The export prints `-1 < *(char *)(...+0x49F)`
  from `rlwinm r0,r0,25,31,31`; on MSB-based numbering that masks bit `31`
  after a 25-bit rotate, i.e. the sign bit, so `read_u8(..) as i8 >= 0` is the
  literal test (and matches `fn_806c7c20`'s `(char)` compare).
- **`0x80743790` reuses the translated `shinkai_light` loader shape.**
  [`fn_8073b020`] is the same body with `0x8016A7F0`/`+0x45C` and different
  names; every helper here (`0x80169D90`, `0x80167420`, `0x80167800`,
  `0x80165C30`, `0x80165FD0`, slot `+0x18`, `0x800690E0`) is in the same
  position, which fixes the argument order of the two model/clip word sockets.
  `0x807CE2F0` reads as `0.0`.
- **`0x807B9F40` scene-pointer reload.** The raw words reload
  `*0x80429F7C` after `0x800FCDB0` (`0x807BA004`) before the `+0x5C` clear but
  keep the earlier `r31` for `+0x3B3` and `+0xAFC`; the body keeps both, so a
  reallocation inside the callback cannot change the later stores.
- **`0x807B9F40` player table is doubly indirect.** `0x807BA03C` loads the word
  at `0x8042A028` and then indexes `+4 + slot*4` off that *value* (the same
  shape as `_DAT_8042a028` in the `0x80602460` export), so the body reads
  `read_u32(read_u32(0x8042A028) + slot*4 + 4)`. It is not the flat table the
  local `player_actor` helper assumes.
- **`0x807B9F40` global roles are inferred.** `0x80429F7C` is the scene/course
  manager that many wave-103 bodies index; `0x80429045`/`0x80429046` are the
  remembered world/course ids (`0xFF` = "none"); `0x80832EB0` is a BSS clear
  flag. The code is literal; only the names are `// inferred:`, which is why
  this row is `medium` while the other four are `high`.
- **`0x8060FB50` record layout is the translated `fn_80630e60` layout.**
  `fn_80630e60` builds the identical 16-word block from `src+0x84..+0xB0` with
  `w0 = src+0x90`, `w1 = src+0xA0` and duplicates at `w7`/`w11`; the only
  difference is `w2`, which is the constant `0x807C7B88` there and the `0.0`
  float here. Slot `w3` is never written in either body and is zero-filled with
  the same `// unknown:` note.

## Verification

- `work/wave175/laneA_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`), with `abi_call1/2/3/5/6`,
  `call0_ret`, `call_f32x3_u32`, `call_f32_u32x2`, `vmethod0/1` and the three
  sibling bodies stubbed.
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/` and their `functions.tsv` sizes
  (392/392/392/392/396) match the brief. Every constant
  (`0x807C93A8`, `0x807C93C0/C/D/E4`, `0x807CB100/B104`, `0x807CE2F0`,
  `0x807C6A18`, `0x807C69F8`), every string
  (`0x808102DC/F4/10300`, `0x807D49AC/C0`) and every branch was re-read from the
  raw words of `work/rels/elf/d_basesNP.elf`.
- The lane CSV is written against `src/game/bases.rs` as the brief requires; the
  bodies live in `work/wave175/laneA_block.rs` until the coordinator appends
  them. `src/game/bases.rs` was not modified.
