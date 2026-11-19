# Coverage - Wave 151 (`game::bases`)

## How to read this file

This page documents the Wave 151 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a return-value
convention had to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 15 | 5,908 | 12 | 3 |

All 15 rows are `translated`; none is `partial`. The module header now reports
3,739 bodies (3,724 + 15). The `0x8067` planner bucket drops from 72 to 65
untranslated addresses; the eight large spec rows lower the global backlog by
their own count.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE150_BASES.md`** (5,508 bytes), each
  re-read word by word against `work/ghidra/rel-d_basesNP/export/*` and the raw
  instruction stream:

  | cluster | body |
  |---|---|
  | world-map castle pop-in | `0x80775D00`: five-arm state (`+0x6C508`), `0.01` seed, timing table `0x8031C9B0` (`380`/`20`/`20`/`1.0`/`1.0`), actor `670`, effect `98`, settle `0.1`, blend `0.9`/`0.1` |
  | model-play-date ticker | `0x80626DB0`: one-shot `+0x27A`, digit bind `+0x288 + 1`, decimal text `+0x28C + 1`, style ladder `22..27`, `"Wm_2d_movieopen"` block, phase ladder `+0x294`, coin ids `{4,1,3,2}` |
  | `boss_kameck_block` placement | `0x8070F370`: 29-record table `0x80805EA0` (stride 20), pointer `+0x7B0`, extents from record `+0x0C`/`+0x10`, animator `0x8007FA40` with callbacks `0x80710120/…1F0/…290`, BSS workspaces `0x8082F788`/`0x8082F848` |
  | `daLiftRemoconSeesaw_c` stepper | `0x806D1EF0`: `+0x552` debounce, rider byte `+0x430`, `±910` band, `0x806D1E70` clamp `±0x1A2C`, `1536` quantum, `560` slew, `8`-tick cue `683`, four-way rumble |
  | `dWorldSelect_c` select state | `0x807BC310`: expansion probe `0x900`, cues `122`/`1631`, pad bits `0x2000`/`1`/`2`/`8`/`4`, state names `0x808340E8 + 0x110/0x1D0/0x210` |
  | `CS_W5` pad-mode mapper | `0x8078FC80`: pad manager `0x80429F7C`, class gate `4`/`5`/`7`, `"W502"`/`"W5G0"`/`"W503"`/`"W505"`, sub-type ladder `1..=5`, mode 1/2/3 rotations |
  | `dCsSeqMng_c` layer writer | `0x807A6BE0`: nine-float pose copy, `0.5` camera quad, dSys push/transform/pop, `+0xA4 |= 1`, fade rect `+0x244`, BSS cache `0x808333A0`, two `0x802BF420` screen pushes |
  | `daFloorDokanEight_c::StateID_Move` | `0x80680FD0`: yaw and 24 tilts by `+0x3732`, eight `0x74`-stride pipes at `400.0`, `8192` angular stride, 48 `0xE4`-byte block flush, per-player weights |

- **Seven `0x8067`-bucket bodies** (400 bytes): the two identical constant
  getters `0x8067C3A0`/`0x8067D200` returning `0x80353F0C`, the sub-object
  initialiser `0x8067F5D0` (`0x80169CD0` on the record and on `+0x40`), its
  release wrapper `0x8067F610` (`0x80169D20(+0x40,-1)`,
  `0x80169D20(record,-1)`, free when `free > 0`), the attached-mover thunk
  `0x8067B140` (tail `bctr` through member `+0x3EC` slot `+0x08`), the
  class-message body `0x8067B200` (`+0x60` slot `0x98`, then `+0x394` slot
  `0x18` with the BSS label `0x8082A3D8`) and the point-in-box test
  `0x8067DCF0`.

- **Three new host-side helpers.** `call_f32x5_u32x2` was added for the
  `0x800DB7D0(node, index, f1..f5)` pipe call at `0x8068119C`; the existing
  `call_f32x5_u32x5` has three extra word arguments. `vmethod2` was added for
  the `0x8067B140` tail call (no two-argument vtable wrapper existed), and
  `write_hold_bit` for the two identical five-word hold-flag sequences at
  `0x807BC4F4` and `0x807BC568`.

## Where the binary differed from the wave-150 prose

1. **`0x80680FD0`'s pipe call does not take `48.0`.** The call at
   `0x8068117C..0x8068119C` loads `f31 = [r31+0x40] = 0x807C9660 = 400.0`,
   `f2 = [r31+0x44] = 0x807C9664 = 0.0`, `f3 = -f31`, `f4 = f2`, `f5 = f3`. The
   `48.0` the spec cites is at `0x807C9668` and is not read by this body.
2. **The per-player neutral weight is `0.0`, not `1.0`.** `0x80681218` reloads
   `f31` from the same `0x807C9664`, so the `fcmpu`/`beq` at
   `0x80681224`/`0x80681228` skips only a weight of exactly zero. The shipped
   values are pushed for every non-zero player.
3. **`0x8070F370`'s half-extent constants are the other way round.**
   `0x807CCA84` is `16.0` and `0x807CCA88` is `0.5`
   (`0x8070F45C..0x8070F470` loads `16.0` into `f4` and `0.5` into `f3`). The
   arithmetic `(a * 0.5) * 16.0` is unchanged; only the label of each address
   moves. `0x807CCA7C` is `0.0` and `0x807CCA80` is `-256.0`, as the spec says.
4. **`0x806D1EF0` stores the *selected* quantum index at `+0x550`, not the
   target index.** The `sth r31,1360(r28)` at `0x806D20AC` uses `r31`, which is
   `q` on the no-move path, `next` on the moving path and still `0` when
   `+0x530` was already set (`0x806D20A4` stores nothing before `0x806D20A8`).
5. **`0x8067DCF0`'s y inequalities are reversed in the export.** Ghidra prints
   `top <= y` and `y <= top - height`; the raw words are `fcmpo cr0,f2,f0` +
   `cror 2,1,2` at `0x8067DD30` and `fcmpo cr0,f0,f1` + `cror 2,1,2` at
   `0x8067DD3C`, i.e. `top >= y` and `y >= top - height`. `cror 2,1,2` is
   `GT|EQ` (`>=`); `cror 2,0,2` (`LT|EQ`, `<=`) is used for the two x compares.

Two confirmations rather than corrections:

- `0x80626DB0`'s `0x800B3BE0` call really binds `+0x288 + 1` while the itoa
  argument is `+0x28C + 1`; the `crxor` at `0x80626E94` is the EABI varargs
  FP-flag clear, not a data dependency.
- `0x807A6BE0` contains no data-side `psq`: the only paired-single words are
  the `psq_st f31/f30/f29/f28` + `psq_l` spill pair with `W = 0`, `qr0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80626DB0` | `fn_80626db0` | 704 | medium |
| `0x8067B140` | `fn_8067b140` | 28 | medium |
| `0x8067B200` | `fn_8067b200` | 84 | high |
| `0x8067C3A0` | `fn_8067c3a0` | 12 | high |
| `0x8067D200` | `fn_8067d200` | 12 | high |
| `0x8067DCF0` | `fn_8067dcf0` | 104 | high |
| `0x8067F5D0` | `fn_8067f5d0` | 56 | high |
| `0x8067F610` | `fn_8067f610` | 104 | high |
| `0x80680FD0` | `fn_80680fd0` | 676 | high |
| `0x806D1EF0` | `fn_806d1ef0` | 684 | high |
| `0x8070F370` | `fn_8070f370` | 692 | high |
| `0x80775D00` | `fn_80775d00` | 708 | high |
| `0x8078FC80` | `fn_8078fc80` | 680 | high |
| `0x807A6BE0` | `fn_807a6be0` | 680 | medium |
| `0x807BC310` | `fn_807bc310` | 684 | high |

**Totals:** 15 bodies, 5,908 bytes. The address window runs from `0x80626DB0` to
`0x807BC310`; the bucket subset is `0x8067B140..0x8067F610`.

## Notes on inference

- `0x80626DB0` is `medium`: for counters `25..=36` the binary forwards the
  incoming `r6` as the style, and neither call site (`0x80626CFC`,
  `0x80627384`) sets it. The translation uses `style = 0` there and marks the
  spot `// unknown:`; the counter is bounded by the `20..24` and `37` arms, so
  the shipped script cannot reach it.
- `0x807A6BE0` is `medium`: `0x800E2580` returns two floats in `r3:r4`
  (`undefined8` in the export) which the body keeps at `r1+0x10`/`r1+0x14` and
  later stores to `+0x78`/`+0x7C`. They are modelled with
  `platform::call_u64` and `f32::from_bits`; what the two words mean to the dSys
  matrix stack stays `// unknown:`. The three floats `0x800E26F0` writes at
  `r1+0x18..0x20` are modelled as a three-float out array.
- `0x8067B140` is `medium`: the callee is only reachable through the `+0x3EC`
  member's vtable, so only the call shape (three arguments, tail branch) is
  provable. Ghidra could not recover the indirect branch
  (`WARNING: Could not recover jumptable at 0x8067B158`).
- `0x8067B200` is `high` but worth noting: Ghidra prints the first call as
  `(**(code **)(*(int *)(param_1 + 0x60) + 0x98))()` with no arguments. `r3` is
  never reloaded after `mr r31,r3`, so the receiver is `this`; the second call
  is fully explicit.
- `0x8067DCF0`'s first argument (`r3`) is never read; it is kept as `_owner` so
  the call ABI stays three words wide.
- `0x8070F370`'s `"cobKoopaCastle"` name for actor `670` and
  `0x80775D00`'s `"cobKoopaCastle"` spawn come from the object table at
  `0x80817010` (`0x808171D4`/`0x808171D8`), not from the bodies; both are
  documented as such in the doc blocks.
- `0x8067C3A0`/`0x8067D200` return a bare address; what the block at
  `0x80353F0C` represents to these callers is `// unknown:`. The same pointer is
  returned by `game::worldmap::fn_8011b210`.

## Deliberate skips

- `0x8067A790` (44 bytes) was left open: its export is a
  `WARNING: Bad instruction - Truncating control flow here` / `halt_baddata`
  cut. The raw words at `0x8067A7B0..0x8067A7B8` end in a `bl` that is followed
  by zero padding instead of a return, so the callee and the argument values
  cannot be confirmed from an instruction span that never reaches `blr`.
- The other 65 untranslated `0x8067` rows were not started (the pass budget
  went to the eight large spec bodies). The bucket now contains, from the
  planner listing: `0x8067E160` (412), `0x806787C0` (392), `0x80675090` (348),
  `0x80671A80` (256), `0x80675960` (252), `0x80678300` (248), `0x80671510`
  (248), `0x8067A520` (232), `0x8067D2F0` (224), `0x8067C490` (224) as the
  largest remaining rows, plus 27 identical 48-byte `func_0x802DCBFC`-style
  wrappers of the family already rejected in wave 150.
- Nothing was written from `docs/SPEC_WAVE150_BASES.md` section 2's
  `25..=36` style residual beyond the documented `0` deviation.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (15 rows, 5,908 bytes),
  re-read from the TSV for this pass rather than from prose. Note that
  `0x80626DB0`'s TSV extent is `704`; the terminating `blr` is one word past it
  at `0x80627070`, so the doc block cites the TSV value and the prose records
  the 708-byte real span.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 15 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave151` reports `OK` (15 rows for
  `wave151-bases.csv`, 32 unique addresses across the wave).
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
- All 15 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000` and every literal was read
  out of `work/rels/elf/d_basesNP.elf` or `work/dol/main.elf`.
