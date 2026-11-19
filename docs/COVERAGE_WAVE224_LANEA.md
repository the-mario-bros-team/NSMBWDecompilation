# Wave 224, lane A - `src/game/stage.rs`, addresses `0x800A7890..0x800AA7B0`

Ten entries of `work/ghidra/main-gb/export/functions.tsv`, all in the
`0x800A0000` run of the `game::stage` ledger module. All ten are translated;
none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800A7890` | 56 | 56 (`blr` at `0x800A78C4`) | `[0x8005F900(r4)+0x27FC]` or `0` |
| `0x800A8170` | 8 | 8 (tail `b` at `0x800A8174`) | `0x800B8850(this + 0x484)` |
| `0x800A8730` | 12 | 12 (`blr` at `0x800A8738`) | return the shared type-info record `0x80377A04` |
| `0x800A8740` | 12 | 12 (`blr` at `0x800A8748`) | same |
| `0x800A87F0` | 12 | 12 (`blr` at `0x800A87F8`) | same |
| `0x800A8800` | 12 | 12 (`blr` at `0x800A8808`) | same |
| `0x800A9550` | 452 | 452 (`blr` at `0x800A9710`) | arm the `+0x414..+0x432` reaction block and the `+0x84C` timer |
| `0x800AA400` | 124 | 124 (`blr` at `0x800AA478`) | recompute the aim angle `+0x792` and clamp it to `0..0x1800` |
| `0x800AA6A0` | 144 | 144 (`blr` at `0x800AA72C`) | resolve, cache and copy the placement triple (`+0x774` -> `target+0xAC`) |
| `0x800AA7B0` | 64 | 64 (`blr` at `0x800AA7EC`) | rebind `this+0x6D8` and zero `+0xAEC` |

Bytes translated: 896.

No `functions.tsv` size in this lane is a prologue artefact. `0x800A8170` is
the one case worth recording: it has no frame and no `blr` at all - the body is
`addi r3,r3,0x484` followed by `b 0x800B8850`, so `8` is the true extent.

## What the run is

* `0x800A7890` is the actor-field lookup that the `Wm`/`Wm_jr` bands use to
  reach a sub-actor: it ignores its own `r3`, resolves `r4` through
  `0x8005F900` and returns the word at `+0x27FC` of the record, or `0`.
* `0x800A8170` is a pure tail forward of the `+0x484` sub-object into the
  destructor `0x800B8850`.
* `0x800A8730`/`0x800A8740`/`0x800A87F0`/`0x800A8800` are four of the
  twelve-byte `0x80377A04` type-info accessors and reuse `W84_TYPE_INFO`.
  The Ghidra export prints `&DAT_80377a04` for all four, which the raw
  `lis r3,0x8037` / `addi r3,r3,31236` (`0x7A04`) / `blr` confirms.
* `0x800A9550` is the reaction-block setter that pairs with wave 223's
  `fn_800a16a0` (which *reads* `+0x414`, `+0x418`, `+0x41C`, `+0x420`,
  `+0x428`, `+0x430`, `+0x431`) and with the wave-84 `fn_80097f70`
  (`+0x414..+0x432` variant record). It computes the "not less" byte with the
  `fcmpu`/`mfcr`/`cntlzw` idiom, unregisters `+0x79C`, seeds the effect
  spawner `0x800E20A0` when the other actor's kind byte is `<= 3`, resolves
  `+0x770` through `0x80162D00` into `0x80162510`, scales `+0x67C` through
  `0x801662F0`, arms the `0x84C` timer with `60`, clears bit `0x8` of
  `+0x120`, and copies the eight-word record at `0x802F0900`
  (`0, 3.0f, -4.0f, -0.1875f, 0x80357734, -1, -1, 0`) into
  `+0x414..+0x42C`, finishing with `+0x430` (not-less), `+0x431` (kind byte)
  and `+0x432 = 1`.
* `0x800AA400` is the aim-angle clamp: the tracked actor's `+0xB0`/`+0xAC`
  against the own `+0x788`/`+0x784`, `fabs` on the horizontal delta,
  `0x801611C0` (`W38_ANGLE_CONV`, the 1/256-turn `atan2`) and the
  `0 < angle <= 0x1800` clamp. Note the vertical delta is the *first*
  operand: `f1 = [+0xB0] - [+0x788]`, `f2 = |[+0xAC] - [+0x784]|`.
* `0x800AA6A0` runs the virtual `+0x484`, then virtual `+0x4E4(out, this)`
  into a twelve-byte scratch, caches the triple at `+0x774..+0x77C` and
  writes it to `target+0xAC..+0xB4`; the first component comes from the
  scratch register, the other two from the fresh cache.
* `0x800AA7B0` rebinds the motion node `this+0x6D8` through `0x80168230` with
  the actor word `+0xAD4` and the destination `+0xAE4`, then stores `0.0f`
  (`r2 - 0x6C44`) at `+0xAEC`.

## Values read from the image

Both new `sdata2` slots are literal zero, and the `0x802F0900` record was read
out of the mapped image of `work/dol/main.elf`:

* `r2 - 0x6C6C` -> `0x8042C414` = `0x00000000` (`0.0f`), the `0x801662F0`
  scale in `0x800A9550`;
* `r2 - 0x6C44` -> `0x8042C43C` = `0x00000000` (`0.0f`), the `+0xAEC` value;
* `0x802F0900`: `0x00000000`, `0x40400000`, `0xC0800000`, `0xBE400000`,
  `0x80357734`, `0xFFFFFFFF`, `0xFFFFFFFF`, `0x00000000`.

## Deferrals

None.

## Caveats recorded in the bodies

* `0x800E20A0` takes two floats plus `r4`/`r5`/`r6`; this module's existing
  `w223_call2_f64_ptr2_F` helper stops after `r4`, so a new
  `call2_f64_u32x3_A224` carries the third integer. `0x800A9550` leaves `r6`
  untouched in the original and the helper passes `0`; that is the only
  non-literal operand in the lane and it is recorded at the call site.
* The trailing `stb r7,0x25(r1)` in `0x800A9550` after the frame restore is a
  dead stack write and is not modelled.
* `0x800A9550`'s word `0x802F0900+0x1C` is stacked but never consumed; it is
  documented rather than reproduced.
* Ghidra's export of `0x800AA400` prints both float arguments as `double`; the
  disassembly (`lfs` + `frsp f2`) shows single-precision inputs, and the
  module's existing `call2_f64_i16` convention is reused for the `0x801611C0`
  operand pair, exactly as the earlier `0x801611C0` call sites in this module
  already do (`ax as f64`, `-(ay as f64)`).

## Verification

Block: `work/wave224/laneA_block.rs`. Appended to a scratch copy of `src/`,
`Cargo.toml` and `Cargo.lock` and `cargo check --lib` run there: exit 0, no
errors, no warnings from the block.
