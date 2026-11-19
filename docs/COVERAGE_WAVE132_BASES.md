# Coverage - Wave 132 (`game::bases`)

## How to read this file

This page documents the Wave 132 pass over `src/game/bases.rs`, the semantic slice
of the `d_basesNP` REL. Every address is a **static analysis address** under the
canonical synthetic base **`0x80600000`** produced by `tools/rel2elf.mjs`; the
Wii REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the same warning line. Cross-module pointers (`0x8037xxxx`,
`0x8042xxxx`) keep their absolute `main.dol` values; this REL's own `.data` /
`.bss` cells keep their `0x807Cxxxx` / `0x8081xxxx` / `0x8083xxxx` synthetic
values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass. All 28 bodies end
in a `blr` (or, for `0x80759770`, a `bctr` tail call) inside their TSV extent; the
only out-of-extent branches are the direct `bl` calls listed in each body.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a `.bss` table's
element meaning had to be inferred. Comments in the bodies keep
`// observed:` / `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 28 | 9,516 | 11 | 17 |

All 28 rows are `translated`; none is `partial`. The module header now reports
3361 bodies (3333 + 28).

## What this pass covers

- **The fifteen class-B bodies of `docs/SPEC_WAVE131_BASES.md` (6,160 bytes)** -
  the twelve headline addresses plus the three overflow bodies the same lane
  specified to the same standard:

  | cluster | bodies |
  |---|---|
  | pane / transform sync | `0x806333D0` four-slot 3x4 matrix scatters through `0x800B32E0` |
  | sample cues | `0x80656310` sample-record selector and two `0x80064610` cues |
  | actor events | `0x80656D20` event-`0x12A` push-apart with the `[2.0, 6.0]` clamp |
  | per-player tables | `0x8066FA10` player-order resampler and `0x80749750` height ranking |
  | launch / stomp ticks | `0x8069AB00` launch-rate splitter, `0x806A4CA0` stomp tick, `0x806A75B0` two-state lift driver |
  | probes | `0x806B7B70` ground-probe impact and `0x807819B0` proximity/pending test |
  | emitters | `0x806B7340` per-entry display emitter |
  | placement | `0x807616E0` door-driver dispatch, `0x80762CE0` spawn placement, `0x80780010` respawn initialiser |
  | spread maths | `0x807816B0` rotated-spread sample evaluator |

- **The `Wm_mr_` runner family from the `0x8075` planner bucket (3,356 bytes,
  13 bodies).** `0x80758A70` snap-and-stop arm, `0x80758C60`/`0x80759430` the two
  spin-sequence starts, `0x80758D80` angle-stepping clamp, `0x80758E00`
  landing-smoke start, `0x80758E70`/`0x80759180` the two per-frame state machines,
  `0x80759550`/`0x80759600` the two height clamps, `0x807596B0` flag reset,
  `0x80759770` state setter with the `bctr` tail call, `0x80759790` the
  `Wm_mr_2dlandsmoke` emitter and `0x80759860` the `Wm_mr_normalspin` pane driver.
  The class name is `// inferred:` from the two effect strings; every offset and
  call is literal.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806333D0` | `fn_806333d0` | 416 | medium |
| `0x80656310` | `fn_80656310` | 396 | medium |
| `0x80656D20` | `fn_80656d20` | 392 | medium |
| `0x8066FA10` | `fn_8066fa10` | 508 | medium |
| `0x8069AB00` | `fn_8069ab00` | 408 | high |
| `0x806A4CA0` | `fn_806a4ca0` | 504 | high |
| `0x806A75B0` | `fn_806a75b0` | 392 | high |
| `0x806B7340` | `fn_806b7340` | 368 | medium |
| `0x806B7B70` | `fn_806b7b70` | 344 | medium |
| `0x80749750` | `fn_80749750` | 360 | high |
| `0x80758A70` | `fn_80758a70` | 192 | medium |
| `0x80758C60` | `fn_80758c60` | 280 | medium |
| `0x80758D80` | `fn_80758d80` | 116 | high |
| `0x80758E00` | `fn_80758e00` | 96 | high |
| `0x80758E70` | `fn_80758e70` | 676 | medium |
| `0x80759180` | `fn_80759180` | 668 | medium |
| `0x80759430` | `fn_80759430` | 284 | medium |
| `0x80759550` | `fn_80759550` | 104 | high |
| `0x80759600` | `fn_80759600` | 104 | high |
| `0x807596B0` | `fn_807596b0` | 168 | medium |
| `0x80759770` | `fn_80759770` | 28 | high |
| `0x80759790` | `fn_80759790` | 196 | high |
| `0x80759860` | `fn_80759860` | 444 | medium |
| `0x807616E0` | `fn_807616e0` | 408 | medium |
| `0x80762CE0` | `fn_80762ce0` | 440 | medium |
| `0x80780010` | `fn_80780010` | 376 | medium |
| `0x807816B0` | `fn_807816b0` | 460 | medium |
| `0x807819B0` | `fn_807819b0` | 388 | high |

**Totals:** 28 bodies, 9,516 bytes (11 high, 17 medium). The address window runs
from `0x806333D0` to `0x807819B0`.

## Where the instruction stream corrects the specification

Three places where the wave-131 prose or the Ghidra rendering disagreed with the
shipped bytes; the bodies follow the bytes and say so at the site:

1. **`0x806A75B0` - the `0x8042C1A0` sampler is unsigned.** The specification
   calls it `(s8)` / "signed byte table", but the shipped pair is
   `lbz r0,0(r29)` followed by `extsh r0,r0` (`0x806A761C`/`0x806A7628`). `extsh`
   on a zero-extended byte leaves `0..255`, so the body reads `u8` and notes the
   correction. A signed read would have flipped every value at or above `0x80`.
2. **`0x807819B0` - the distance test is `length > 2.0 * radius`.** `mfcr` plus
   `rlwinm r29,r29,1,31,31` extracts `CR0.GT` from `fcmpo f31, 2.0*f1`; Ghidra
   prints the negation (`dVar6 < 2.0 * dVar7`). The body follows the mask.
3. **`0x806333D0` - the matrix block is twelve words, not fifteen.** The export
   also copies `e+0xA8`/`e+0xAC`/`e+0xB0` to `r1+0x44..0x4C`, but those words lie
   past the twelve-word block that `0x800B32E0` reads (established in
   `docs/SPEC_WAVE128_BASES.md` §4) and are dead; the body records them as dead
   stores instead of writing them.

## Deliberate skips

Read and rejected, with the reason:

- **The rest of the `0x8075` planner bucket.** 79 of the bucket's 92 untranslated
  addresses remain after this pass. The largest (`0x8075D5C0`, 1080 bytes;
  `0x80757320`, 1032; `0x8075F270`, 768) were not attempted: they are outside the
  `Wm_mr_` family and each needs its own decoded table, which this lane did not
  have budget to verify.
- **`0x807583D0` (420 bytes) and `0x80759A20` (36 bytes).** Both are called by the
  new family, but neither is inside the claimed cluster and both are entered with
  record pointers whose element roles are not visible in this REL. They stay
  address-only `abi_call` sites with `// unknown:` notes, which is exactly how the
  family itself was reconstructed.
- **`0x80759860`'s 7-argument callee.** `0x8008FBF0` is called through
  `platform::call7` (the `abi_call6!` ceiling stops at `r8`); the extra register is
  real (`r9 = &scale`), so the body uses `platform::call7` rather than dropping an
  argument.

## Notes on inference

- [`fn_806333d0`] is `medium`: the twelve words handed to `0x800B32E0` are exact,
  but which row/column they represent is not recoverable from this REL.
- [`fn_80656310`] is `medium`: only `+4` and `+0xC` of the `0x8008E890` sample
  buffer are read, so the buffer stays a raw word array.
- [`fn_80656d20`] is `medium`: the collider receives two floats and the actor four,
  visible from `r4`; the callee prototypes are not in this REL.
- [`fn_8066fa10`] is `medium`: the `r25`/`r26` compaction asymmetry and the
  restart-from-`this` sort are reproduced exactly; `0x80052080`'s contract is not
  visible.
- [`fn_806b7340`] is `medium`: the third argument of `0x80064AF0` has no visible
  meaning.
- [`fn_80762ce0`] is `medium`: the six facing floats at `0x80377DD4..0x80377DF4`
  live in `main.dol`'s zero-fill segment, and the record's `+0x1A` halfword
  overlaps the next 24-byte entry.
- [`fn_80780010`] is `medium`: the two snapshot vectors are consumed by
  `0x80780BD0`/`0x807813F0`, whose vector roles are not visible here.
- [`fn_807816b0`] is `medium`: the 48-byte basis is produced only by `0x8016ED10`
  and is emitted as an opaque word block.
- [`fn_80758a70`], [`fn_80758c60`], [`fn_80759430`], [`fn_807596b0`] and
  [`fn_80759860`] are `medium`: the shared `0x807CEA58..0x807CEB14` float pool and
  the model's `+0x5C` contract are inferred from the sibling bodies; every store
  and call is literal.
- [`fn_80758e70`] and [`fn_80759180`] are `medium`: the player record's
  `+0x14`/`+0x1C` bit roles are inferred from the arms that consume them; the
  angle ramps are transcribed with 16-bit wrapping and the `0x8001`/`0x7FFF`
  clamps.
- New host-side helper added by this pass: `vmethod_f32x3_u32` (three `f32` plus
  one `u32` through slot `+0x5C`, used by six of the `Wm_mr_` bodies).
- The `unaff_GQR0`/`ldexpf` blocks printed at the head and tail of
  `0x8066FA10`, `0x806A4CA0`, `0x806A75B0`, `0x806B7340`, `0x80749750`,
  `0x807816B0` and `0x807819B0` are the prologue/epilogue spill noise described in
  `docs/REL_ELF_MAPPING.md`: every `psq_l`/`psq_st` is a `qr0` `f31` (or
  `f31`/`f30`/`f29`/`f28`) save/restore paired with `stfd`/`lfd`. No body uses
  `qr3` and no body contains an `mtspr` to a `GQRn`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (28 rows, 9,516 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 28 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave132` reports `OK`;
  `wave132-bases.csv` has 28 rows and 28 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
