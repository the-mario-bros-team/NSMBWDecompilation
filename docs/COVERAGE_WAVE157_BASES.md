# Coverage - Wave 157 (`game::bases`)

## How to read this file

This page documents the Wave 157 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the line
`Synthetic base: 0x80600000; this is not a runtime address.`

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a table meaning
had to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 16 | 6,664 | 9 | 7 |

All 16 rows are `translated`; none is `partial`. The module header now reports
3,815 bodies (3,799 before this pass + 16). Eight rows are the eight "ready"
rows of `docs/SPEC_WAVE156_BASES.md` (4,540 bytes); the other eight are one
`0x8065`-bucket family (2,124 bytes), which takes that planner bucket from 68 to
60 untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE156_BASES.md`**, the
  572-to-560-byte band:

  | cluster | body |
  |---|---|
  | `daLiftScalesStep_c` ctor | `0x806DB340`: window nibble, two width bands, `+0x4F0` record, Disp/Hold pushes, two display blocks, interface position |
  | `boat_light_wood` create | `0x807041B0`: reset, model load, optional `0x800D9130` re-place, palette clear, two scale draws, five setup bodies, keep/discard gates |
  | `untei_rope` create | `0x80745DC0`: BRRES bind, rope curve, two `0xE4` scroll layers, three `d3d` elements |
  | world-map walker frame | `0x8077D340`: `+0x2C0` countdown, mode-6/7 move effects, node transition, fall animation `5` |
  | World-7 Koopa binder | `0x80788090`: `g3d/<name>.brres`, six `0x38`-byte parts, rate `1.0`, wrapper `+0x39C` |
  | node cosine | `0x8079B7C0`: XZ segments through `0x80100200`, zero projections, `dot / (|c| * |f|)` |
  | `lift_spin_kinoko` handler | `0x806DF820`: four-player presence producer consumed by `fn_806df770` |
  | nine-row board screen | `0x80628F70`: nibble cursor, row re-evaluation through `fn_80628940`, four descriptors, sound ids `0x78`/`0x79`/`0x7B` |

- **Eight `0x8065`-bucket bodies of one `Wm_jr_bomb` family** (2,124 bytes):

  | cluster | body |
  |---|---|
  | fuse/emit pair | `0x80657FE0` (radius `24.0 * +0xDC`), `0x80657BB0` (`Wm_en_landsmoke` puff), `0x80657B30` (`Wm_jr_bombfuse` re-arm) |
  | wall bounce | `0x80657EA0`: `+0x680`/`+0x67C` bounds, velocity sign flip, sound `0x5BE` |
  | per-frame halves | `0x80658280` (fuse step, sound `0x5BD`, `+0xD4` pushes) and `0x806584B0` (sound `0x5BF` through `0x80199390`) |
  | contact machine | `0x806566F0`: `+0x3EC` sample, four-phase word `+0x3F4`, `0.1`/`0.2` thresholds, `0x806564A0` |
  | burst | `0x806587B0`: `Wm_jr_bomb`, `Wm_ob_cmnboxsmoke`, `Wm_ob_cmnboxgrain`, sound `0x5C2`, callback `0x80656EB0` |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80628F70` | `fn_80628f70` | 560 | medium |
| `0x806566F0` | `fn_806566f0` | 404 | medium |
| `0x80657B30` | `fn_80657b30` | 128 | medium |
| `0x80657BB0` | `fn_80657bb0` | 96 | high |
| `0x80657EA0` | `fn_80657ea0` | 308 | high |
| `0x80657FE0` | `fn_80657fe0` | 20 | high |
| `0x80658280` | `fn_80658280` | 352 | high |
| `0x806584B0` | `fn_806584b0` | 348 | medium |
| `0x806587B0` | `fn_806587b0` | 468 | high |
| `0x806DB340` | `fn_806db340` | 572 | high |
| `0x806DF820` | `fn_806df820` | 564 | high |
| `0x807041B0` | `fn_807041b0` | 572 | medium |
| `0x80745DC0` | `fn_80745dc0` | 568 | high |
| `0x8077D340` | `fn_8077d340` | 568 | medium |
| `0x80788090` | `fn_80788090` | 568 | medium |
| `0x8079B7C0` | `fn_8079b7c0` | 568 | high |

**Totals:** 16 bodies, 6,664 bytes. The address window runs from `0x80628F70` to
`0x8079B7C0`; the bucket subset is `0x806566F0..0x806587B0`.

## Where the binary differed from the prose

1. **`0x80628F70`'s `0x80628940` call takes a third argument.** The export prints
   `FUN_80628940(param_1, *(int *)(param_1 + 0xa90))`, but the raw words keep the
   input nibble in `r5`: `0x80628FA0` is `rlwinm r5,r0,0,28,31` (MB 28, ME 31)
   and nothing between it and the call at `0x80629030` writes `r5`. The
   translation passes `bits` and the already-translated
   `fn_80628940(this, index, kind)` consumes it.
2. **`0x806DF820`'s vtable is not at the receiver.** Both gates
   (`+0xE0` and `+0x6C`) read the vtable from `subject + 0x60` while passing
   `subject` as `this`, which [`vmethod0`] cannot express; the body reads the
   vtable word explicitly and calls the two slots with `r3 = subject`.
3. **`0x807041B0` calls `0x80702F40` with a receiver.** The translated
   `fn_80702f40` declares no arguments, but the call site passes `actor` in
   `r3`, so this body calls the address directly and says so in a comment.
4. **`0x80657EA0`'s first `0x80657FE0` call is printed without arguments.** `r3`
   still holds the actor at that point, so both calls pass it.
5. **`0x80657B30`'s `+0x298` slot is printed with no arguments.** The element
   calls in the same body all pass `this = actor`, so the slot is called with
   `actor`; this is the only inferred argument in that body.

Confirmations rather than corrections: `0x806DB340`'s `unaff_GQR0`/`ldexpf`
block is only the `qr0` `f31` prologue save (the wave-9/112/155 rejection was
false); `0x8079B7C0`'s `double` return is the `f31` restore guess around a
`fdivs`; `0x80788090`'s loop stride is `0x38`, the sixth part repeats name `0`
and the flag table `0x807CFA38` holds `{0,0,1,1,1}`; `0x806DF820`'s array index
is the signed byte from the `+0x6C` slot (`extsb` then `<<2`), and `0x806587B0`
really does multiply the three effect scales by `0.8` for the third emit.

## Notes on inference

- `0x80628F70` is `medium` only because the class's nominal name is not
  recoverable from the body; every offset, nibble, sound id and descriptor is
  literal.
- `0x807041B0` is `medium`: the tilt/emitter roles and the
  `boat_light_wood` identification come from the neighbouring `0x8070` bodies;
  `0x80043CA0` and `0x80702FD0` stay address-only.
- `0x8077D340` is `medium`: the `+0x2C0` delay-counter name is inferred from
  `fn_8077d7b0`'s respawn sub-machine. `0x80104FD0`, `0x8079EE70` and
  `0x8077E410` are address-only calls with the argument shape fixed at the call
  site.
- `0x80788090` is `medium`: only the roles of the six part slots and of the
  `+0x218` byte are inferred; the `0x38` stride is fixed independently by
  `fn_80789b80`/`fn_80789c60`.
- `0x806566F0` is `medium`: `0x80162D00`'s record (`+0x394`/`+0x398`) is named
  by the two compared floats and `0x806564A0` is called address-only (see the
  skip list).
- `0x80657B30` and `0x806584B0` are `medium` for the single inferred receiver
  argument and for the word-array spelling of their offsets.
- The remaining nine bodies are `high`: every guard, store, constant and call
  argument is visible in the export and the raw words.

## Deliberate skips

- `0x8065CB20` (3,788), `0x8065DD50` (1,772) and `0x8065F090` (1,772) were not
  started: they are the three largest rows left in the bucket and each needs a
  full paired-single reconstruction.
- `0x8065C9A0` (376) is skipped: its export declares an `undefined8` third
  argument (a paired-single pair) and it calls the untranslated `0x8065CB20`, so
  the argument layout cannot be stated without guessing.
- `0x806564A0` (92) is skipped: its export ends in a call to `0x80A46F60`,
  which is outside this REL's text range, so the body cannot be completed from
  the given evidence. It is therefore called address-only by
  `0x806566F0`.
- `0x80651A60` (452) is skipped: it is a 21-word ring-buffer compaction over
  `0x80828430..0x80828610` whose record fields and the `0x8082860C`/
  `0x80828610` words have no proven meaning; writing it would be guesswork.
- `0x80651640` (384) stays deferred: the module header already records it as a
  rejected paired-single reconstruction.
- Nothing was written that the raw words did not support.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (16 rows, 6,664 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 16 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave157` reports `OK` (16 rows, 16 unique
  addresses in `wave157-bases.csv`).
- `cargo check --message-format=short` exits 0 and reports no diagnostic naming
  `src/game/bases.rs`.
- All 16 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`; every literal was read out of
  `work/rels/elf/d_basesNP.elf` (for example `0x807C89B0 = 24.0f`,
  `0x807C89C8 = 0.8f`, `0x807C894C = 2.2f`) and the strings
  `Wm_jr_bomb`/`Wm_ob_cmnboxsmoke`/`Wm_ob_cmnboxgrain`/`Wm_jr_bombfuse`/
  `Wm_en_landsmoke` from the `0x807E6xxx` string block.
