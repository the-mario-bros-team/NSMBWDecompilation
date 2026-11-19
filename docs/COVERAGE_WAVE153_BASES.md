# Coverage - Wave 153 (`game::bases`)

## How to read this file

This page documents the Wave 153 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the line
`Synthetic base: 0x80600000; this is not a runtime address.`

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a table meaning had
to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 5,864 | 4 | 6 |

All 10 rows are `translated`; none is `partial`. The module header now reports
3,759 bodies (3,749 + 10). Eight rows are the eight "ready" rows of
`docs/SPEC_WAVE152_BASES.md` (5,088 bytes); the other two are `0x8064`-bucket
bodies (776 bytes), which takes that planner bucket from 70 to 68 untranslated
addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE152_BASES.md`**, the 648-to-628-byte
  band, each re-read against `work/ghidra/rel-d_basesNP/export/*` and the raw
  instruction stream with `file_offset = (address - 0x80600000) + 0x1000`:

  | cluster | body |
  |---|---|
  | `dGameOver_c` save-image round trip | `0x8061B890`: four palette bytes, 10 x 42 ink body, four id/pointer snapshots, seven releases, palette and body written back, `0x800FBD30(1)` |
  | `dMultiCourseSelect_c` cursor step | `0x8062BE20`: D-pad bits `1`/`2`/`4`/`8`, cell availability through `+0x248`/`+0x24C` bit 0, page crossings through `0x8062BCF0` |
  | boat-light band histogram | `0x80703DC0`: four claim timers at `+0xE0A`, six player-switch lists at `+0xB84`, five bands at `-63/-21/21/63`, colour claim `0x80702EA0`, mask `+0xE11`, popcount return |
  | four-slot join-panel tick | `0x8074CBA0`: state delay `+0x42E`, travel `+0x408`, countdown `+0x42A`, random 320/576 re-arm, selection through `0x8074C280`, redraw `0x8074C540` |
  | `obj_waterfloat` transform | `0x80752730`: position/angle copy, scale, blend `+0x4A0` via `0x8015F140`, two lean pairs around `+0x488`/`+0x48C`, holder `+0x3F4` |
  | `cobKinoko` model/clip binder | `0x8076B710`: `g3d/model.brres` variant pane and `cobKinokoRed`, body model `+0x1A8`, clips `kinoko`/`trunk`, eight-part group `+0x258` |
  | Kameck ride/flight binder | `0x8078A8F0`: `g3d/kameck.brres`, broom model `+0x610`, ride clips `+0x44C`, wand-node hides, map clips `+0x4BC`/`+0x564` |
  | map-enemy appear/fly machine | `0x80792570`: five states, appear ramp, effect `100`, particle call, velocity, flight repetitions, `150.0` retire test |

- **Two `0x8064`-bucket bodies** (776 bytes):

  | cluster | body |
  |---|---|
  | 64-bit mask shifter | `0x80648770`: 64-bit word `{+0x358, +0x35C}`, mask/input pairs `{+0x1A490, +0x1A494}` / `{+0x1A488, +0x1A48C}`, selection record `[0x8042A078]`, left latch `1` / right latch `0xFF` at `+0x1A4F6` |
  | `I_kinoko` variant loader | `0x8064CD60`: variant `+0x6E4` indexes three parallel 5-entry tables at `0x807E43AC`/`0x807E444C`/`0x807E447C`, model `+0x3F0`, `wait2` clip `+0x430`, `I_kinoko_switch` animation `+0x468` for variants 0/4 |

## Where the binary differed from the spec prose

1. **`0x8074CBA0`'s travel test is `d > 0.0`, not `d < 0.0`.** The export prints
   `if ((double)(player[+0xAC] - this[+0x3F8]) < 0.0) store; else accumulate`,
   but the raw words at `0x8074CC48..0x8074CC50` are `fcmpo cr0,f0,f31`,
   `cror 2,1,2` and `bne 0x8074CC64` — `cr0.eq` is rewritten to `lt | eq`, so the
   `bne` (to the store) is taken exactly when `d > 0.0` or unordered. The
   translation keeps `if !(d <= 0.0)` so the NaN case still stores, matching the
   branch.
2. **`0x80792570`'s state-2 particle call passes `f2 = 35.0`.** The spec reads
   `f2 = [0x807CFED8] * 3.5 = 3.5`; the raw words at `0x80792650..0x80792670`
   load `f1` from `0x807CFED0` (`10.0`), `f3` from `0x807CFEAC` (`3.5`) and
   `f0` from `0x807CFEDC` (`1.0`), then `fmuls f2,f1,f3` (`10.0 * 3.5 = 35.0`)
   and `fmuls f3,f0,f3` (`3.5`). `0x807CFED8` is never loaded. The body passes
   `10.0`, `35.0`, `3.5`.
3. **`0x8076B710`'s clip loop contains the `0x801651F0` binding call.** The
   spec's step-7 Rust sketch omits it, but its own callee table lists it
   (`0x8076B82C`: `entry, this+0x1A8, &clip, 1`) and the raw word at
   `0x8076B82C` is the `bl`. The translation includes it.

Confirmations rather than corrections: `0x80703DC0`'s band loop is `d < t` with
an accumulated `-105.0 + k*42.0` (Ghidra agrees; the `mfcr`/`rlwinm` extraction
is `cr0.lt`); `0x8061B890`'s four id words are read before `0x800B2FB0` and are
never rewritten; `0x8074CBA0`'s `+0x431` double compare at
`0x8074CC8C`/`0x8074CC94` is dead and omitted with a note; the `r26`/`r30`
candidate select at `0x8074CD64` is a no-op and omitted with a note.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8061B890` | `fn_8061b890` | 628 | medium |
| `0x8062BE20` | `fn_8062be20` | 644 | high |
| `0x80648770` | `fn_80648770` | 288 | medium |
| `0x8064CD60` | `fn_8064cd60` | 488 | medium |
| `0x80703DC0` | `fn_80703dc0` | 648 | high |
| `0x8074CBA0` | `fn_8074cba0` | 636 | medium |
| `0x80752730` | `fn_80752730` | 628 | medium |
| `0x8076B710` | `fn_8076b710` | 644 | high |
| `0x8078A8F0` | `fn_8078a8f0` | 632 | high |
| `0x80792570` | `fn_80792570` | 628 | medium |

**Totals:** 10 bodies, 5,864 bytes. The address window runs from `0x8061B890` to
`0x80792570`; the bucket subset is `0x80648770..0x8064CD60`.

## Notes on inference

- `0x8061B890` is `medium`: the call sites and loops are literal, but the record
  layout (`+0x1A` palette, `+0x7C4` 10 x 42 body) is read out of the `main.dol`
  accessors `0x800CE260..0x800CE320`, not provable from this body.
- `0x8074CBA0` is `medium`: every store and constant is literal, but the
  contracts of `0x8074C540` (draw/commit), `0x8074C990` (travel accumulator) and
  `0x8074C280` (player-in-rectangle) are named by their effect, not resolved.
- `0x80752730` is `medium`: `0x8015F140`'s `(target, rate, limit, band)` order is
  provable from its `main.dol` body, and `0x8016EEF0`'s `s16 * 1/256` rotation is
  taken from `0x8016EE30`; neither is visible in this caller.
- `0x80792570` is `medium`: the five states and all constants are literal, but
  `0x800F8540`, `0x800F6AB0` and `0x800F7AB0` are named by address and their
  `+0x6BC`/`+0x6C4`/`+0x6C8` side effects come from their own bodies.
- `0x80648770` is `medium`: the shift pair, both masks, the two latches and the
  multiword compare are literal; the roles of the 64-bit word, the two scan
  masks and the `[0x8042A078]` selection record are `// unknown:`.
- `0x8064CD60` is `medium`: the three pointer tables, the `wait2` and
  `I_kinoko_switch` names and the `2.0`/`0.0` frame pair are literal; which
  power-up each variant names is inferred from the strings in the tables, and
  `0x80166830`/`0x80166BD0`/`0x80166EF0` are named by address only.
- `0x80703DC0`'s `kind` getter (`lwz r12,96(actor); lwz r12,108(r12); bctrl;
  lbz; extsb`) is factored into the host-side `actor_kind_byte` helper; the
  unsigned `< 4` gate therefore rejects negative kinds, as the `cmpli` does.
- `0x8062BE20`'s repeated `+0x248`/`+0x24C` bit-0 test is factored into the
  host-side `course_cell_present` helper; the five `9..5` early returns are kept
  as returns so `+0x4F8` is not written on that path.

## Deliberate skips

- The rest of the `0x8064` planner bucket was not started (the pass budget went
  to the eight spec bodies plus two bucket bodies). By this lane's TSV + ledger +
  `fn_`-token filter the largest remaining rows are `0x8064DCC0` (408),
  `0x8064DEF0` (368), `0x8064F510` and `0x8064F820` (360 each), and
  `0x8064F390`/`0x8064F6A0` (348 each).
- `0x80648060` (504 bytes) was decoded and left open: its `0x800EAA00` call
  mixes three float and two pointer arguments whose prototype is not provable
  from the call site, and the `0x1A4FA` state byte feeds an argument register
  (`r8`) computed by a six-way byte ladder, so the argument list would be a
  guess.
- `0x806488E0` (404 bytes) and `0x80648AA0` (456 bytes) were left open: both
  issue virtual `+0x2C` calls on `[0x80429D64]` with a packed
  `(id, s16, 0)` argument list and read camera fields at `+0x3C`/`+0x44`/`+0x4C`
  of `[0x80429E00]`; the receiver and the packed argument's role are not
  provable from the call site.
- Nothing was written from `docs/SPEC_WAVE152_BASES.md` that the raw words did
  not support; the three deviations above are implemented as the binary behaves,
  not as the prose describes.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (10 rows, 5,864 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 10 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave153` reports `OK` (10 rows for
  `wave153-bases.csv`, 10 unique addresses in the lane).
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
- All 10 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`; every literal was read out of
  `work/rels/elf/d_basesNP.elf`.
