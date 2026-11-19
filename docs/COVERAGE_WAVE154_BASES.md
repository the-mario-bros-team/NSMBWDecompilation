# Coverage - Wave 154 (`game::bases`)

## How to read this file

This page documents the Wave 154 pass over `src/game/bases.rs`, the semantic
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
| `src/game/bases.rs` | `game::bases` | 13 | 5,180 | 8 | 5 |

All 13 rows are `translated`; none is `partial`. The module header now reports
3,772 bodies (3,759 + 13). The 13 rows are the eight "ready" rows of
`docs/SPEC_WAVE153_BASES.md` (4,936 bytes), the 12-byte leaf `0x80794BF0` they
call (12 bytes), and four `0x806f`-bucket bodies (232 bytes), which takes that
planner bucket from 69 to 65 untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE153_BASES.md`**, the 624-to-604-byte
  band, each re-read against `work/ghidra/rel-d_basesNP/export/*` and the raw
  instruction stream with `file_offset = (address - 0x80600000) + 0x1000`:

  | cluster | body |
  |---|---|
  | `daRotDokan_c` two-anchor pose | `0x807310C0`: yaw `0x4000 - yaw`, `1/256` trig scale, radii `24.0`/`16.0`, matrix `+0x7C`, holders `+0x458`/`+0x498` |
  | `cobTsuta` growth machine | `0x80794980`: five states, record table `0x8081B4EC` stride `0x1C`, fade cross-fade, effect `62`, `+0x340` latch |
  | World-7 Koopa create tail | `0x80787BD0`: model/binding pass, four per-kind pose arms, `0x1C`-byte point record |
  | player-link table rebuild | `0x807B2050`: `0x80354E50..0x80354EA0`, four-slot id swap, `0x80833EA8` descriptor |
  | four-record permutation | `0x80637A40`: pending guard, `0x807C7D18 = {0,1,3,2}`, `0xF0`-byte staging copy |
  | `lift_zen_shiro_yogan` spawn | `0x806E7080`: five flag slices, four quadrant offsets, element `+0x474` |
  | four-player result wheel | `0x80741010`: sixteen popup slots, blocked test, `+16`/`-16`/proportional rate arms |
  | `+0x2CC` record builder | `0x8063A960`: order from `0x80354E60`, sort to `0x807C7F70`, `0x70`-byte staging copy |

- **The 12-byte leaf** `0x80794BF0` (`+0x2C4` value, `+0x350` handle,
  tail call `0x80102DB0`), reconstructed so `0x80794980` calls a real body.

- **Four `0x806f`-bucket bodies** (232 bytes):

  | cluster | body |
  |---|---|
  | resource release | `0x806F1B30`: `0x8015F800(owner, 0)` then `0x802B9280` only when `amount > 0` |
  | present countdown trio | `0x806FDC50` (`+0x218` timer, pad bits `0x100`/`0x800`), `0x806FDCA0` (timer setter), `0x806FDCB0` (`+0x1D4` clear, `0x8005C930`, `0x80429FD8+0x1D = 1`) |

## Where the binary differed from the spec prose

1. **`0x80794980`'s common tail seeds `+0x264` with `0.0`, not `1.0`.** The
   spec sketch writes `0x801680E0(1.0f, this + 0x264); // 0x807D005C`. The raw
   words at `0x80794B88..0x80794B90` are `addis r4,r0,0x807D` then
   `lfs f1,88(r4)`, i.e. `0x807D0058`, which is `0.0f`; the `0x801680E0` at
   `0x80794B94` receives that value. `0x807D005C` (`1.0f`) is used only by the
   state-3 cross-fade at `0x80794AC8`.
2. **`0x806E7080` slices the flag word differently from the sketch.** The
   `rlwinm` words fix the arguments: `r4 = (w >> 12) & 0xF` (`0x806E70A4`),
   `r5 = (w >> 24) & 3` (`0x806E70A8`), `r6 = (w >> 16) & 0xF`
   (`0x806E70C0`), `r7 = (w >> 20) & 0xF` (`0x806E70C4`), `r8 = (w >> 8) & 3`
   (`0x806E70C8`). The quadrant selector is `r5` and the offset multiplier is
   `r4`; the spec's `>>8`/`>>22`/`>>6` map is not what the code does. The
   per-quadrant formulas and all six constants are otherwise as documented.
3. **`0x807B2050`'s body is 624 bytes, not 620.** The TSV extent stops one word
   short of the `blr` at `0x807B22BC`; the row therefore cites `620 bytes` (the
   TSV value, which the lane validator checks) and explains the extra word in
   the same doc block.

Confirmations rather than corrections: `0x807310C0`'s radii are `24.0` and
`24.0 - 8.0 = 16.0` with the same `1/256` scale for both trig calls;
`0x80741010`'s `0x807CE0A0` really is loaded with `lfd` and is the double
`64.0` (its first word is the float `3.25`), and the `cror 2,1,2` + `bne` pair
is the `|angle| <= 64` window; `0x80787BD0` really does write through the
record pointer even on allocator failure, which this file guards with an
`// observed:` note.

Two signature adaptations follow the file's own convention rather than the
spec sketch: `0x807B2050` and `0x8063A960` keep `this: usize` (entry `r3`, which
the `__savegpr` prologues preserve) instead of the spec's `fn()`, matching
[`fn_80628a00`] and the other helper-preserved bodies.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80637A40` | `fn_80637a40` | 616 | high |
| `0x8063A960` | `fn_8063a960` | 604 | medium |
| `0x806E7080` | `fn_806e7080` | 616 | medium |
| `0x806F1B30` | `fn_806f1b30` | 88 | high |
| `0x806FDC50` | `fn_806fdc50` | 72 | high |
| `0x806FDCA0` | `fn_806fdca0` | 8 | high |
| `0x806FDCB0` | `fn_806fdcb0` | 64 | high |
| `0x807310C0` | `fn_807310c0` | 624 | high |
| `0x80741010` | `fn_80741010` | 612 | high |
| `0x80787BD0` | `fn_80787bd0` | 620 | medium |
| `0x80794980` | `fn_80794980` | 624 | medium |
| `0x80794BF0` | `fn_80794bf0` | 12 | high |
| `0x807B2050` | `fn_807b2050` | 620 | medium |

**Totals:** 13 bodies, 5,180 bytes. The address window runs from `0x80637A40` to
`0x807B2050`; the bucket subset is `0x806F1B30..0x806FDCB0`.

## Notes on inference

- `0x80794980` is `medium`: the five arms, the record table and the four scale
  constants are literal, but the record `+0x1C` fade halfword aliases the next
  row's first word (all three read `30`), and the effect/record tag role of
  `62` is named rather than resolved.
- `0x80787BD0` is `medium`: all seven virtual slots are fixed by their use and
  every constant is read, but the slots stay named by offset and the `+0x420`
  parameter block layout comes from the two `s16` reads.
- `0x807B2050` is `medium`: every global and loop bound is literal; the four
  collision arms are the compiler-unrolled form of the id swap (implemented as
  the first-match swap they encode).
- `0x806E7080` is `medium`: three element-builder callbacks (`0x806E6CF0`,
  `0x806E6D50`, `0x806E6DB0`) stay address-only, which the ready rule allows.
- `0x8063A960` is `medium`: the `0x1C`-byte record is copied as `0x1A` bytes;
  the original leaves `+0x1A`/`+0x1B` from the stack but never copies them back,
  so the zero-filled staging buffer is exact. `0x8063A8C0`/`0x8063A680`/
  `0x8063A850` stay address-only.
- `0x806F1B30`, `0x806FDC50`, `0x806FDCA0` and `0x806FDCB0` are `high`: every
  store, bit mask and call argument is literal; the two global cells
  (`0x80429F50`, `0x80429C94`, `0x80429FD8`) are named by their loads, and
  `0x802B9280` keeps an `// unknown:` comment.

## Deliberate skips

- The ninth and tenth rows named by `docs/SPEC_WAVE153_BASES.md`
  (`0x8068D060` and `0x80799340`, 604 bytes each) were not started: the pass
  budget went to the eight ready rows, the leaf they need and four bucket
  bodies. Both are fully decoded in the spec and remain untranslated.
- The rest of the `0x806f` planner bucket was not started (65 addresses still
  open). By this lane's TSV + ledger + `fn_`-token filter the largest remaining
  rows are `0x806F2100` and `0x806F2710` (520 bytes each), `0x806F0F20` (496) and
  `0x806F64B0` (492); the smallest open rows are the 48-byte getters
  `0x806F1C50`/`0x806F1C80`/`0x806F2DF0`/`0x806F2E20`.
- `0x806F1C50`/`0x806F1C80` (48 bytes each) were left open: both are bare
  `0x802DCBFC` indirect-jump stubs (`func_0x802dcbfc(param_2,param_2,param_1)`),
  which the module header already documents as an unresolved indirect-jump
  stub family, so a body would only restate the stub.
- Nothing was written from `docs/SPEC_WAVE153_BASES.md` that the raw words did
  not support; the three deviations above are implemented as the binary
  behaves.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (13 rows, 5,180 bytes),
  re-read from the TSV for this pass rather than from prose. `0x807B2050`'s
  doc block cites the TSV's 620 while its body note explains the 624-byte span.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 13 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave154` reports `OK` (13 rows for
  `wave154-bases.csv`, 13 unique addresses in the lane).
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`. The last full run that completed before the parallel
  `game::enemies` lane introduced its own two `E0308`/`E0277` errors in
  `src/game/enemies.rs` was clean; the only diagnostics in that run were the
  pre-existing warnings in other modules.
- All 13 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`; every literal was read out of
  `work/rels/elf/d_basesNP.elf`.
