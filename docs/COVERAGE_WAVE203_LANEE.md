# Coverage - wave 203, lane E (`ui::hud`, `0x80022370..0x80023A60`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 203,
lane E. All ten now have a faithful Rust body in
`work/wave203/laneE_block.rs` (appended to `src/ui/hud.rs` on merge); nothing in
this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x80022370` | 24 | `fn_80022370` | high | forward a kind-`4` record to the owner filter `0x80022180` |
| `0x80022390` | 48 | `fn_80022390` | high | `max(active_players, 1) - 1` through `0x8005FEF0` |
| `0x800223C0` | 208 | `fn_800223c0` | medium | start/stop the owner's sound loop and return `0`/`1` |
| `0x80022490` | 356 | `fn_80022490` | high | post one sound event (`0x237`/`0x257`/`0x258`/`0x25A`) |
| `0x80022780` | 140 | `fn_80022780` | high | kind-`4` dispatch, else fall back to `0x80022B10` |
| `0x80022810` | 764 | `fn_80022810` | medium | single-owner state table event and actor spawn |
| `0x80022B10` | 684 | `fn_80022b10` | medium | multi-hit state table event, `0x800A77A0` + per-record sounds |
| `0x800238C0` | 88 | `fn_800238c0` | high | deleting-destructor wrapper (destroy, then free if `> 0`) |
| `0x80023920` | 92 | `fn_80023920` | high | same wrapper; contains one dead duplicate null test |
| `0x80023A60` | 224 | `fn_80023a60` | medium | five-link `+0x30` chain walk with virtual `+0xC` test |

Total: 2,628 bytes, 10/10 translated, 0 deferred.

Sizes are the recorded function extents. Two of them include alignment bytes:
`0x80023920` is 88 bytes of code plus a four-byte pad, and the next body after
`0x80022370` starts at `0x80022390` after eight pad bytes.

## What the cluster is

`0x80022370..0x80023A60` is the shared "player sound event" block, not a layout
scene. It sits immediately behind lane D's `0x800217F0..0x800221E0` attack/owner
filter group and in front of the state table at `0x802EE320`, and its real
callers are attack/hit code (`0x80A1C8F0`, already translated in
`src/game/enemies.rs`) and the actor bases, not the HUD layouts. Wave 203 assigns
the range to this lane, so the bodies live in `src/ui/hud.rs`; the doc blocks say
so rather than implying the block renders anything.

The event record (`info`) passed to `0x80022490`, `0x80022810` and `0x80022B10`
is two `f32` (x, y) followed by:

| offset | type | meaning |
|---|---|---|
| `+0x00` | `f32` | x |
| `+0x04` | `f32` | y |
| `+0x0C` | `s8` | owner/player index, `-1` = none |
| `+0x10` | `u32` | state-table index |
| `+0x14` | `u8` | selects the alternate state table (and the `0x1000` mask bit) |
| `+0x15` | `u8` | role byte; `2`/`4` states zero it |
| `+0x16` | `u8` | non-zero also stamps `created + 0xBD0` |

The state id comes from `lwzx` on one of two 31-entry tables. First sixteen
words read from the image:

- `0x802EE320`: `15, 2, 0, 0, 21, 17, 25, 1, 27, 12, 4, 7, 10, 13, 8, 14`
- `0x802EE368`: `15, 2, 8, 9, 21, 17, 25, 1, 27, 12, 4, 7, 10, 13, 8, 14`

`15` (`0xF`) and `6` are dropped. `0xC` calls `0x800223C0` then latches
`+0x648`; `0xD` calls `0x800217F0`; `5` posts the positioned feedback pair
`0x80066740`/`0x800667B0` plus sound `0x238`; `0xA` calls `0x80021A20`. Every
other state builds an actor through `0x80064610(0x3C, mask, pos, 0, +0x38F)`
and then calls `0x80022490`.

## Resolved constants

`.sdata2` floats were read from `work/dol/main.elf`
(`SDA2_BASE = 0x80433080`):

| address | offset | value | use |
|---|---|---|---|
| `0x8042B2E4` | `SDA2 - 0x7D9C` | `16.0` | front-facing vertical raise |
| `0x8042B2F0` | `SDA2 - 0x7D90` | `8.0` | back-facing vertical lowering |
| `0x8042B2F4` | `SDA2 - 0x7D8C` | `-3580.0` | stage depth plane |
| `0x8042B294` | `SDA2 - 0x7DEC` | `0.0` | default depth word |

`r13` anchors: `-0x5920` = `0x80429D60` (sound manager passed to
`0x80198C30`), `-0x5940` = `0x80429D40` (input/feedback word), `-0x5868` =
`0x80429E18` (global read by `0x80088270`), `-0x5770` = `0x80429F10` (manager
passed to `0x800A77A0`).

## Notes on the bodies

1. **`0x80022490` goto merge.** The original reaches sound `0x25A` from two `b`
   targets: the `forced != 0` arm and the passing `0x80022180` arm. Both fall to
   the same call, so the Rust body uses one condition
   (`kind == 0x15 && (forced != 0 || filter != 0)`); ids and comparisons are
   unchanged.
2. **`0x80022810` mask.** Both role arms shift `info + 0x0C` left by 16, not the
   local role copy. The role byte only chooses the arm: zero adds
   `this[0x68D] << 18`, `0x80000` and (when `info + 0x16 != 0`) `info[0x16] << 20`;
   non-zero replaces them with `0xA000000`. `0x800` is OR-ed in at the call site.
3. **`0x80022B10` argument count.** `0x800A77A0` takes eight integer arguments
   (`r3..r10`), one more than `crate::platform` exposes. It is called through a
   direct eight-argument function pointer; this is the only place in the block
   that cannot use a `platform` helper.
4. **`0x800238C0`/`0x80023920`.** `0x8015F800(object, 0)` performs no call in its
   own body (it only frees when the flag is negative), so the pair is
   destroy-no-op then free when `free > 0`. `0x80023920` contains an extra
   `cmpi cr0,3,r3,0`/`beq` at `0x80023944` that re-tests the same `cr0` bits;
   nothing writes `cr0` in between, so it is dead and both wrappers behave
   identically.
5. **`0x80023A60` walk.** Five in-frame steps, then a tail recursion on the sixth
   child, so the walk is unbounded overall but consumes at most five links per
   frame. The virtual slot is `vtable + 0xC` on the child, called with the child
   in `r3`; a non-zero answer returns the parent node.
6. **`0x800223C0`/`0x80088270`.** The stop condition is
   `0x80088270(*(r13 - 0x5868)) >= active_players`. The roles of
   `0x80060AB0`/`0x80088270` are inferred from their bodies (per-player
   predicates over the table at `0x80354E20`), which is why this body is marked
   `medium`.

## Evidence

- Disassembly: `work/dol/main.elf` via `ppc159.js`, `LEN=80` (entry sweeps) and
  `LEN=200/90/70/45/20` for the interiors of `0x80022810` and `0x80022B10`,
  `STOP=0`, for all ten entries.
- Decompilation: `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` (lowercase)
  for all ten addresses; `work/ghidra/main/` was not needed as a fallback.
- Callee bodies consulted: `0x80022180`, `0x800221E0`, `0x8005FEF0`,
  `0x80088270`, `0x80060AB0`, `0x8015F800` (all `main-gb` exports) and
  `0x80198C30` (already translated in `src/audio/ax.rs`).
- Data: both state tables and the four `.sdata2` floats were read directly from
  the ELF image.

## Verification

`work/wave203/laneE_block.rs` was appended to a temporary copy of `src/`,
`Cargo.toml` and `Cargo.lock` (repo copy with `target/` and `.git/` excluded),
and `cargo check --lib` exited `0` with 0 errors. The copy reports the same 23
pre-existing warnings as the untouched tree, and none of them point at the
appended lines.
