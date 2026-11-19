# Coverage - Wave 189 lane B (`game::bases` 128-192 band)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1,424 | 10 | 0 | 0 |

Ten functions from the `0x807189D0..0x807204B0` run of the `lift_turi_ashiba`
cable actor and its neighbours. All ten are translated; nothing is deferred.

## Highlights

- **Cable loader (`0x8071B8E0`).** Loads `g3d/lift_turi_ashiba.brres`
  (`0x80809228`) as `lift_turi_ashiba` (`0x80809244`), then binds the pane table
  at `0x807CCF50` (`lift_turi_ashiba_L/M/R`) through `0x800BC5D0` with the
  second float `1.0` from `0x807CCF60`.
- **Cable constructor (`0x8071BBB0`).** Allocates `0x4A18` bytes, runs
  `0x80066FC0`, installs `0x80307B48` then `0x80809354`, and builds the
  two-element `0x2290`-stride array at `+0x4E0` with constructors
  `0x80717B80` / destructors `0x80717CF0`.
- **Rider attach (`0x807189D0`).** Resolves the character byte `+0x5CD`
  through `0x8005F900` and posts the raised position
  (`+0xB0 + 16.0 + +0x5C8`) to `0x80050D40` at the receiver `+0x4E4`.
- **Record writers (`0x8071B990` already present, new `0x8071BC50`).** The new
  body is the variant-selecting twin that reads `0x807CCF98..0x807CCFB4`
  instead of the fixed `+0x14..+0x20` window used by `fn_8071b990`.
- **Matrix writer (`0x8071ED20`).** Same `0x801C0500` / `0x801C0BD0` /
  `0x801C0540` shape as `fn_806907d0`: copy out, translate by
  `model[+0x554]` on X, concat back, copy in.
- **Comparator (`0x807204B0`).** Identical `0x802E1C40`/`0x802E1AB4` tail
  comparison to the established `fn_80700ed0`, including the unchecked second
  separator lookup.
- **Factories and releases.** `0x80719D00` (`0x5C8`, vtable `0x808088B0`) and
  `0x8071A230` (releases `+0x584`, `+0x544`, `+0x524`, base, then
  `0x80162920`).
- **Effect block (`0x8071D340`).** Two `0x40`-stride adapters at `+0x1A4C0`,
  gated by `+0x1A59A + index`, plus the `+0x514`/`+0x3D0` tail resets.
- **Animation binder (`0x8071EFB0`).** Indexes `0x807CD064`
  (`obj_yashi_wait`/`yashi_wait`/`i_wait`) and drives `0x801651F0`,
  `0x80169F80` and `0x801680E0` at `+0x3B0`/`+0x3B4`/`+0x3F4`.

## Corrections and recoveries

- `0x807189D0`: the `mr r3,r0` at `0x80718A18` reloads the zero-extended
  `+0x5CD` byte from GPR `r0`; the intervening `lfs r0,0x807CCDE0` writes only
  `f0`, so the resolver argument is the character index, not `0x41800000`.
- `0x8071B8E0`: Ghidra's `undefined8 param_1` is `f1`, and `param_2` is really
  the object in `r3`; `r4` is never read. The `ldexpf`/`GQR0` guards are
  compiler noise.
- `0x8071ED20`: `addic. r4,r1,0x38` only tests the outgoing stack address;
  the first `0x801C0500` call takes `(*out, scratch)` and `f3` is a copy of
  `f2` (`0.0` from `0x807CD050`).
- `0x8071A230`: the `addic. r3,r3,0x584` guard is an always-true address test
  folded into the outer null test.
- `0x8071D340`: the `addis`/`addi` pair computes `this + 0x1A59A`, and the
  `0x800EAB20` argument is the `rlwinm 0,24,31` low byte of `0x80809620`.
- `0x8071BC50`: all four table variants were decoded from the raw `lfs`
  sequence rather than from Ghidra's shared-`uVar` form; the store order is
  `+0x320`, `+0x324`, `+0x318`, `+0x31C` in every arm.

## Verification

- The block type-checks standalone against stub helpers and macros (rustc
  1.99.1, `--edition 2021`).
- All ten evidence exports exist; 1,424 bytes total.
- No duplicate `fn_` names against the current `src/game/bases.rs`.
