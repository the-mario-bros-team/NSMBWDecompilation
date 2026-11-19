# Coverage - Wave 191 lane D (`game::bases`, 128-192 byte sweep)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 5 | 824 | 5 | 0 | 0 |

Five bodies, all from the 128-192 byte band. Source of the bodies:
`work/wave191/laneD_block.rs`; per-function ledger:
`ledger/agents/wave191-laned.csv`.

This lane sits on the seam between the `cobKoopaShip` ride family
(`0x807BF5A0..0x807BFE20`) and the world-map aim/ride object
(`0x807C0A80..0x807C28A0`). All five bodies are state-gated on a field of the
object and most of them end in a shared tail call, so they reuse the already
translated snapshot helpers instead of re-deriving the layouts.

## What this pass covers

- **Pose re-sample (1 body).** `0x807BF6B0` (164) gates on state `+0x608` (`0`
  runs [`fn_807bf560`] first, `1` goes straight to the body, anything else
  jumps to the tail), samples `*(0x8042A1A0) + 0x29C` twice through
  [`fn_8079a310`] and [`fn_8079a330`], and stores the results at
  `+0x4BC..+0x4C4` and `+0x634..+0x63C` before the unconditional
  `0x807BF5A0(this)`.
- **Step integration (1 body).** `0x807BFB20` (184) adds the pending step
  `+0x5F8..+0x600` to both position triples (`+0x4B0..+0x4B8` and
  `+0x4BC..+0x4C4`) and then reloads the step from the zero vector
  `0x80377DC8..0x80377DD0`. The same `0`/`1`/other state gate applies, with a
  bare `bclr` for the other states.
- **Scene tag probe loop (1 body).** `0x807C0A80` (152) resolves
  `scene + scene[+0x338C] * 0x16518` from the `0x8042A18C` singleton and calls
  `0x807C09B0` up to `record[+0x33A0]` times, returning `1` at the first
  non-zero result (Ghidra's `return 1` / `return 0` pair is the `blt` back edge
  at `0x807C0AF0`). This is the body behind the `fn_807c13a0` call site. The
  callee overwrites `r3`/`r4` (`lis r3,0x1` at `0x807C09C8`) and loads its own
  scene name and the four-tag table at `0x8082152C`, so the two forwarded
  arguments are dead; they are still passed to keep the call shape.
- **Aim step (1 body).** `0x807C10A0` (132) returns `1`
  when the 3D-mode byte `0x808215E4` is clear; otherwise it runs
  `0x8015F140(+0x6F8, 0.4, 100.0, 0.5, &+0x6FC)`, and on `+0x6FC == +0x6F8`
  resets `+0x6F8` from `0x807D0A6C` (`0.0`) and returns `1`, else `0`. This is
  the predicate that the translated aim state machine [`fn_807c0400`] calls in
  its state-`4` arm.
- **Idle close / pane event (1 body).** `0x807C28A0` (192) reads the world
  object at `+0xB0`: when `+0x680` is clear and `+0x67E` is also clear it
  calls `0x800B4710` for slots `0..4`, writes `+0x558 = 4`,
  runs [`fn_807c1fe0`] and writes `+0x4D4 = -10`; when `+0x680` is set it tests
  bit `0x10` of the current pad record's `+0x1C` word and dispatches pane
  virtual slot `+0x18` with `0x80834A00`.

## Reuse and helper notes

- No new helpers were introduced. The block uses only `platform::read_*` /
  `write_*`, `abi_call1`, `abi_call2` and `call_f32x4_u32`, plus the
  already-translated [`fn_807bf560`], [`fn_8079a310`], [`fn_8079a330`] and
  [`fn_807c1fe0`], all present in `src/game/bases.rs`.
- `0x807BF5A0` (260 bytes) and `0x807C09B0` (196 bytes) have no Rust body yet and
  go through `abi_call1!` / `abi_call2!`. A later pass can turn both call sites
  into direct calls; `fn_807c0a80` documents the dead arguments of the second.
- `0x80377DC8..0x80377DD0` is the family's three-word zero vector in `main.dol`
  data, outside the `d_basesNP` image (the ELF ends at `0x808228B8`). It is copied
  as raw words, exactly as `fn_80795140` and `fn_80760250` already do, because
  the value is not in the REL and need not be known to preserve the bits.
- `0x80429F50` is a **pointer** global: `lwz r3,-0x60B0(r3)` loads the base and
  the record is then fetched at `base + index * 4 + 4`. `fn_807c28a0` reads it
  through the pointer; the same base-pointer form appears in [`fn_807b2ed0`].
- `0x80821598` = `0.4`, `0x8082159C` = `100.0`, `0x808215A0` = `0.5` and
  `0x808215E4` = the 3D-mode flag; `0x807D0A6C` = `0.0`. All were read out of
  the ELF.

## Verification

- Every body was re-read from `work/rels/elf/d_basesNP.elf` with `ppc159.js`;
  each byte size matches `functions.tsv` (164, 184, 152, 132, 192 = 824).
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; the CSV targets `src/game/bases.rs`.
- The block was compiled with a stub harness (`rustc --edition 2021
  --crate-type lib`) that defines `platform`, `abi_call1/2` and the five
  callee stubs; it reports no errors and no warnings, so every helper name and
  call shape used here exists in the crate.

## Deferred

None. All five bodies were proven from the raw instruction stream.
