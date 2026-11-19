# Coverage - Wave 210, lane E (`game::player`, the `0x800622A0..0x800625A0` run)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 800 | 10 | 0 |

Bodies: `work/wave210/laneE_block.rs`. Ledger rows:
`ledger/agents/wave210-laneE.csv`.

## How to read this file

Ten functions from one contiguous run in the `0x80060000` bucket of
`work/ghidra/main-gb/export/functions.tsv`. All ten are `translated`; none is
deferred.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra `FUN_<address>` name, kept as `fn_<address>` in Rust.
* **Confidence** - `high` everywhere: each body is a literal instruction-level
  match. The only decompiler-level ambiguity is noted per row below.

## The object this run works on

Every body but the two constant getters takes a scene actor:

| offset | role |
|---|---|
| `+0x00` | id word (remembered in `+0x3D8` by [`fn_80062420`]) |
| `+0xAC` / `+0xB0` | world X / world Y (`f32`) |
| `+0x104` | facing angle, signed 16-bit, 65536 per turn |
| `+0x394` | interface object; its first word is the vtable |
| `+0x3D0` / `+0x3D4` | local X / Y offset to the anchor (`f32`) |
| `+0x3D8` | anchor id word |
| `+0x39C` / `+0x3E2` | per-class tag bytes read by the two registry scans |

## What this pass covers

- **The release pair** - [`fn_800622a0`] is the family destructor: reset through
  `0x800671B0(this, 0)` and free through `0x80162920` only when `destroy > 0`,
  returning `this`. It is the same shape as the wave-72 `fn_80046070` and reuses
  that lane's two constants.
- **The two constant getters** - [`fn_80062300`] and [`fn_80062310`] are
  byte-identical (12 bytes each, `lis`/`addi`/`blr`) and return the shared
  handler address `0x80377A04`, i.e. Ghidra's `&DAT_80377a04`. That address is
  already the return value of a dozen getters in this module (for example
  `fn_80040920`), so the body returns the literal.
- **The two registry scans** - [`fn_80062320`] (class `0x1A3`, tag byte
  `+0x39C`) and [`fn_80062380`] (class `0x1A4`, tag byte `+0x3E2`) both walk
  `0x80162D50(class, previous)` from `previous = 0`, stop on a `0` result, and
  return the first object whose zero-extended tag byte equals the argument.
- **The combiner** - [`fn_800623e0`] prefers the `0x1A3` scan and falls back to
  the `0x1A4` scan. Ghidra prints `FUN_80062320()` with no argument and a `void`
  result; the disassembly keeps `r3` live into both calls and returns the
  callee's `r3`, and [`fn_80062520`] tests that result for zero, so the tag is
  forwarded and the pointer returned.
- **The transform pair** - [`fn_80062420`] records
  `(this->+0xAC - src->+0xAC, this->+0xB0 - src->+0xB0)` in `+0x3D0`/`+0x3D4`
  and copies `src->+0x00` into `+0x3D8`. [`fn_80062450`] then adds a quarter
  turn (`0x4000`) to the anchor's `+0x104` angle, stores it on `this`, and
  places `this` at the anchor position plus that local offset rotated by the new
  angle:
  `x = srcX + ox*cos - oy*sin`, `y = srcY + ox*sin + oy*cos`.
- **The two callers** - [`fn_80062520`] resolves a tag, records the offset,
  places `this`, then calls interface slot `+0x18` on the object at
  `this + 0x394` with `0x80355D8C`. [`fn_800625a0`] resolves the id remembered
  in `+0x3D8` through `0x80162D00` and re-applies [`fn_80062450`] only.

## The trig helpers and the angle unit

[`fn_80062450`] is the only body that measures anything, so the two constants it
needs are pinned here:

* `0x80237C40` is the cosine and `0x80237BD0` the sine. Both are 256-entry
  interpolated tables (`DAT_802f74a0` + `0x10 * index`), and both take their
  argument in 256-units-per-turn angle space, not radians.
* The `lfs` from `r2 - 0x75C8` (`0x8042BAB8`) multiplies the signed 16-bit angle
  before the call; the word in `work/dol/main.elf` is `0x3B800000`, i.e.
  `0.00390625` = `1/256`, which converts 65536-per-turn counts into those 256
  units. The body reads the word from the image rather than hard-coding it.

`crate::engine::display` reaches the same two helpers with the same `1/256`
factor (`W108_F32_6684`), so the identification is cross-checked against a
second, independently translated lane.

## Size notes

`functions.tsv` sizes were used verbatim in the `# Original` lines. One of them
conflicts with a lane target list: `0x80062450` was listed as 20 bytes, but
`functions.tsv` says 196 and the prologue (`stwu r1,-0x30`) plus the `blr` at
`0x80062510` confirm `0x80062450..0x80062514` = 196 bytes. The body follows the
196-byte extent.

The other nine extents agree with both sources, including the two 12-byte
getters (8 bytes of code plus 4 bytes of padding to the next function).

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to `src/game/player.rs`, and `cargo check --lib` exited
`0` there (23 warnings, all pre-existing and none from this lane's bodies or from
`src/game/player.rs`).
