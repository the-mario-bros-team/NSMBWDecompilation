# Coverage - Wave 141 (`game::bases`)

## How to read this file

This page documents the Wave 141 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**, produced by
`tools/rel2elf.mjs`; the Wii REL loader chooses the real load address, so these
numbers identify positions inside the REL image, not runtime addresses. Every
`# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a virtual dispatch
had to be named rather than resolved. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 11 | 12,700 | 0 | 11 |

All 11 rows are `translated`; none is `partial`. The module header now reports
3586 bodies (3575 + 11).

## What this pass covers

- **Four large "ready" bodies of `docs/SPEC_WAVE140_BASES.md`** (9,428 bytes).
  These are the state tables the earlier waves deferred for their size; each was
  re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000` and every store was cross-checked against
  the export.

  | cluster | bodies |
  |---|---|
  | Koopa Jr airship sequence | `0x8076DA30` (16 states, `koopaJr_all_root` helper, `-180/10`, `180/4`, `90/angle` step rates) |
  | airship departure | `0x8065F910` (4 states, four-player rail lining, `Wm_ob_witchcraftup` emitter, camera staging) |
  | Toad/kinopio sequence | `0x8076C900` (19 states, `kinopio_all_root`, scene flags at `0x80834470+0x544..0x54D`) |
  | world-map spawner | `0x807108B0` (slot ring, `0x80805EA0` enemy rows, `0x808065A0` bounds, `Wm_ka_coin` burst) |

- **Seven `0x8067` planner-bucket bodies (3,272 bytes)**, taking that bucket
  from 82 to 75 untranslated addresses:

  | cluster | bodies |
  |---|---|
  | pad-mask refreshers | `0x8067E320`, `0x8067EAF0` (four-phase auto-repeat), `0x8067DE40`, `0x8067DFD0` (the empty-mask twin) |
  | stage overlap test | `0x8067E500` (kind-3 actor inside an `0x8008E890` rectangle) |
  | balloon-scale stepper | `0x80671D70` (`+0x8AC`/`+0x8B0` steps, `+0xB0` ratio interpolation) |
  | three-pane projection | `0x80676A20` (`+0x3F8`/`+0x438`/`+0x478` screen rectangles) |

## Where the binary differed from the wave-140 prose

1. **`0x8065F910`'s timer constants are not `475`/`30`/`20`.** The words at
   `0x807E72F4`/`0x807E72F8` are `5` and `165` (loaded with `lwz`, compared with
   `+0x3D0`), `0x807E72FC` is `330`, and `0x807E7320` is the **float** `850.0`
   materialised with `lfs` + `fctiwz` before the compare. The body writes
   `5`/`165`, `330` and `850`; the prose values would have changed the state
   timing.
2. **The two `0x43300000` ladders in `0x8076DA30` carry literals.** `case 3` and
   `case 6` convert `10` (`0x43300000_8000000A`) and `case 9` converts `4`
   (`0x43300000_80000004`), so the steps are `-180/10*scale` and
   `180/4*scale`. `0x807CF278`/`0x807CF180` are the **double** `2^52 + 2^31`,
   not the `176.0f` Ghidra prints from their low word.
3. **`0x807108B0`'s row base is `0x808063F0`, not `0x80805EA0`.** The record
   pointer is `0x80805EA0 + 0x550 + mode*0x90 + k*0x14`; the weight is at
   `+0x10`, the id list starts at `+4` as consecutive `i16` and the coin count is
   at `+0xC`. The four corners are `u32 as f32` conversions, written plainly in
   Rust.
4. **`0x8076C900` case `0xC` overwrites two of its three camera words.** The
   scene-path point supplies `x`, but `y`/`z` are then replaced by the second and
   third floats of the `0x800F1160` camera record. Both buffers are kept.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8065F910` | `fn_8065f910` | 2,204 | medium |
| `0x80671D70` | `fn_80671d70` | 536 | medium |
| `0x80676A20` | `fn_80676a20` | 800 | medium |
| `0x8067DE40` | `fn_8067de40` | 364 | medium |
| `0x8067DFD0` | `fn_8067dfd0` | 356 | medium |
| `0x8067E320` | `fn_8067e320` | 472 | medium |
| `0x8067E500` | `fn_8067e500` | 284 | medium |
| `0x8067EAF0` | `fn_8067eaf0` | 460 | medium |
| `0x807108B0` | `fn_807108b0` | 2,468 | medium |
| `0x8076C900` | `fn_8076c900` | 2,100 | medium |
| `0x8076DA30` | `fn_8076da30` | 2,656 | medium |

**Totals:** 11 bodies, 12,700 bytes (all `medium`). The address window runs from
`0x8065F910` to `0x8076DA30`.

## Notes on inference

- All eleven rows are `medium` for the same reason: each body calls at least one
  unclaimed sibling (`0x8076C900`'s `0x8076D280`/`0x8076AF60`/`0x80792A10`/
  `0x80792A20`, `0x807108B0`'s `0x806601B0`/`0x8065E440`/`0x807103D0`, the
  `0x8067E3xx` pad family's `0x802DD1EC`/`0x800E41C0`, the NW4R interfaces in
  `0x80676A20`), or names a field role by use. Every argument register is
  visible in the instruction stream and is written at the call site, so nothing
  in a body is guessed.
- `0x807108B0` and `0x8065F910` take their receiver from the `bl 0x802DCD40` /
  `bl 0x802DCD68` save helpers only as the surviving entry `r3`; the
  register-recovery notes in `docs/SPEC_WAVE140_BASES.md` are confirmed by the
  raw `or rN,r3,r3` heads.
- `0x8067E320` models the `0x802DD1EC` result as a 64-bit pair, as
  [`fn_80648550`] already does: the low half pairs with `mask[1]`, the high half
  with `mask[0]`.

## Deliberate skips

Read and rejected, or deferred with the reason:

- **`0x8077B450` (4,076) and `0x8065CB20` (3,788)** are the two remaining
  "ready" rows of `docs/SPEC_WAVE140_BASES.md`. Both are fully specified but
  each needs its own multi-phase call graph (39 action blocks for the first, a
  four-phase terrain grid with two join tables for the second) and was left for
  a dedicated pass rather than started at the end of this one.
- **`0x8079C330` (4,232), `0x80661070` (3,432), `0x8079D3C0` (3,176),
  `0x8078DC70` (2,576), `0x8076EF80` (2,544) and `0x80773210` (2,096)** are the
  six "conditional" spec rows; they remain open.
- **`0x8067E160` (412)** reads its only non-zero argument from the incoming
  `r7` (`in_r7` in the export) while Ghidra prints a `void` signature over a
  dropped register bracket. The argument cannot be recovered honestly, so the
  body was not written.
- **`0x8067FC80` (1,676), `0x8067E640` (1,160), `0x806765B0` (1,052)** and the
  rest of the bucket above `0x80676A20` each need their table generators
  (`0x807ECB48`, the `0x807C92xx` pools) and were deferred.
- **`0x807BD110`** is still the `dFileSelect_c` registration table, not a body.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (11 rows, 12,700 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 11 were checked against
  the ledger's address column before being written, and none is duplicated.
- Each body's last word was re-decoded from `work/rels/elf/d_basesNP.elf` at
  `(address - 0x80600000) + 0x1000`; all 11 end in `0x4E800020` (`blr`) at
  `entry + size - 4`, and no `functions.tsv` row starts inside any body.
- `node tools/check-lane-csv.mjs wave141` reports `OK`: 11 rows, 11 unique
  addresses, all `translated`/`medium`.
- `cargo check --message-format=short` finishes with no error naming
  `src/game/bases.rs`.
