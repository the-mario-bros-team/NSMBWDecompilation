# Coverage - Wave 52 (`game::bases`, firepillar and `I_yoshi_egg` actors)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL (synthetic base `0x80600000`; every address is a static analysis address,
not a runtime address). Byte counts come from
`work/ghidra/rel-d_basesNP/export/functions.tsv`; the quoted evidence files are
the Ghidra C exports under `work/ghidra/rel-d_basesNP/export/`. Confidence is
`high` when the guards, stores, strings and call targets are literal, `medium`
when a callee's numeric role is inferred from surrounding arithmetic.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4560 | 29 | 1 |

## What this pass covers

- The `cobfirepillar` course actor, `0x807A1FC0..0x807A2EE0`: its
  `g3d/firepillar.brres` model/clip loader, the `+0x3F0` transform writer, the
  height/scale writer, the `ef_firepillar` node sampler, the height clamp, the
  two timer arm/tick pairs (`0x3C` and `0x96` frames, sound `0x309`), the
  `firepillar_loop` re-arm, and the three burst drivers that invoke the
  `Wm_ob_firespillar01/02` panes and switch to states `0x80832F60`,
  `0x80832FE0` and `0x80833060`.
- An unnamed egg actor, `0x807A3700..0x807A47A0`: its registered-state
  destructor, the `0x610`-byte heap constructor (`0x80094E80` base, vtable
  `0x8081C740`, accessor/model/clip/effect holders), the transform writer, the
  indexed `I_yoshi_egg` clip selector, the colour height selector, the four
  `Wm_ob_eggbreak_*` burst spawns, the damage-state selector that maps the
  attacker's `+0x1090` character byte through `0x807D0468`/`0x807D047C`, the
  bounce and heavy-hit reactions, and the three break-state handlers.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x807A1FC0` | `fn_807a1fc0` | 228 | high |
| `0x807A2320` | `fn_807a2320` | 228 | high |
| `0x807A2410` | `fn_807a2410` | 56 | high |
| `0x807A2450` | `fn_807a2450` | 140 | high |
| `0x807A24E0` | `fn_807a24e0` | 76 | high |
| `0x807A2530` | `fn_807a2530` | 84 | high |
| `0x807A25A0` | `fn_807a25a0` | 132 | high |
| `0x807A2630` | `fn_807a2630` | 92 | high |
| `0x807A26A0` | `fn_807a26a0` | 132 | high |
| `0x807A2730` | `fn_807a2730` | 140 | high |
| `0x807A27D0` | `fn_807a27d0` | 116 | high |
| `0x807A2AC0` | `fn_807a2ac0` | 396 | high |
| `0x807A2EE0` | `fn_807a2ee0` | 280 | high |
| `0x807A3700` | `fn_807a3700` | 88 | high |
| `0x807A3880` | `fn_807a3880` | 188 | high |
| `0x807A3C00` | `fn_807a3c00` | 244 | high |
| `0x807A3EC0` | `fn_807a3ec0` | 152 | high |
| `0x807A3F60` | `fn_807a3f60` | 36 | high |
| `0x807A3F90` | `fn_807a3f90` | 104 | high |
| `0x807A4000` | `fn_807a4000` | 108 | high |
| `0x807A4070` | `fn_807a4070` | 284 | high |
| `0x807A4190` | `fn_807a4190` | 84 | high |
| `0x807A41F0` | `fn_807a41f0` | 60 | high |
| `0x807A4230` | `fn_807a4230` | 76 | high |
| `0x807A42B0` | `fn_807a42b0` | 128 | high |
| `0x807A4360` | `fn_807a4360` | 240 | high |
| `0x807A4550` | `fn_807a4550` | 188 | medium |
| `0x807A4680` | `fn_807a4680` | 184 | high |
| `0x807A4740` | `fn_807a4740` | 80 | high |
| `0x807A47A0` | `fn_807a47a0` | 216 | high |

## Deliberate skips

- `0x807A2200` (48 bytes) is a one-line virtual call on the `+0x3F0` model
  holder that returns 1. It is real but adds no recoverable role beyond
  [`fn_807a2ac0`]'s first call, so it was left with the firepillar cluster's
  state trio rather than padded into a row.
- `0x807A28B0` (440 bytes) and `0x807A2CA0` (500 bytes) decompile with an
  unrecovered `GQR0` register and `ldexpf` bookkeeping around their paired-single
  moves. Their float semantics cannot be settled honestly, so no body was
  invented; [`fn_807a2ac0`] and [`fn_807a2ee0`] carry the surrounding literal
  work.
- `0x807A3820` and `0x807A3850` are 48-byte adjustor thunks whose whole body is
  `0x802DCBFC(a2, a2, a1)`, with no recoverable object role.
- `0x807A3D00` (444 bytes) is the egg actor's model loader and has the same
  `GQR0`/`ldexpf` decompilation failure; its two resource strings
  (`g3d/I_yoshi_egg.brres`, `I_yoshi_egg`) and its pane table are already named
  in this coverage doc so the neighbouring bodies stay readable.
- The egg actor's three state tables at `0x807A4930..0x807A5BC0` belong to the
  already-translated `daYoshiEgg_c` cluster from wave 5 and were not touched.
- The rest of the `0x807A` bucket (183 untranslated before this pass, 153
  remain) is outside this lane's window.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000` line.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave52` reports `OK` for `wave52-bases.csv`.
