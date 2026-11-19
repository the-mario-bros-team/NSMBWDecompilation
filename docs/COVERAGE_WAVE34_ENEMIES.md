# Coverage - Wave 34 (`d_enemiesNP`, salvage of the aborted pass)

## How to read this file

The Wave 34 `d_enemiesNP` lane on the primary free model was cut off after it
had written the bodies but before it wrote its ledger and coverage files. A
second attempt produced nothing at all. Rather than rewrite the code, this pass
audited what was on disk and recovered it.

This file documents only those recovered bodies. Earlier enemy work is in
`docs/COVERAGE_REL_d_enemiesNP.md` and `docs/COVERAGE_WAVE*`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 5,464 | 27 | 3 |

Addresses are **synthetic analysis addresses** based at `0x80A00000`; they are
not runtime addresses.

## What this pass covers

- The `daEnKanibo_c` transform/setter tail (`0x80A9C080..0x80A9D630`):
  pair selection by mode, the `+0x348` guard, model-transform writes and the
  two animation/state helpers.
- The `daEnKanon_c` head (`0x80AA00A0..0x80AA0F90`): constructor support,
  the Walk/Turn contact reaction, owner/state-object queries, the model
  transform, effect spawns and the per-frame state steps.
- The `0x80AA0340` stale byte count inherited from an older coverage doc was
  corrected from 52 to 48 bytes against `functions.tsv`.

## Functions

| address | function | bytes | confidence | first doc line |
|---|---|---:|---|---|
| `0x80A9C080` | `fn_80a9c080` | 80 | high | Chooses the `daEnKanibo_c` speed pair for one state. |
| `0x80A9C0D0` | `fn_80a9c0d0` | 588 | high | Builds and commits both `daEnKanibo_c` model transforms. |
| `0x80A9C320` | `fn_80a9c320` | 360 | high | Binds the `g3d/kanibo.brres` models and the `wait_L`/`wait_R` clip. |
| `0x80A9C490` | `fn_80a9c490` | 136 | high | Emits the `0x11F` ground-hit effect at the stored `+0x610` position. |
| `0x80A9C520` | `fn_80a9c520` | 360 | high | Tests whether another actor is inside the `daEnKanibo_c` contact band. |
| `0x80A9C770` | `fn_80a9c770` | 72 | high | Flushes the pending `daEnKanibo_c` ground effect. |
| `0x80A9C7C0` | `fn_80a9c7c0` | 108 | medium | Queries the stage surface below a candidate point. |
| `0x80A9C860` | `fn_80a9c860` | 272 | high | `daEnKanibo_c` Wait step: contact test, countdown and RockUp switch. |
| `0x80A9C9A0` | `fn_80a9c9a0` | 208 | high | `daEnKanibo_c` RockUp step: animation watch and Jump switch. |
| `0x80A9CAE0` | `fn_80a9cae0` | 232 | high | `daEnKanibo_c` Jump entry: one-shot sound, speed pair and BackMove test. |
| `0x80A9CBD0` | `fn_80a9cbd0` | 256 | high | `daEnKanibo_c` BackMove entry: probe the step and seed the motion words. |
| `0x80A9CCE0` | `fn_80a9cce0` | 296 | medium | `daEnKanibo_c` Jump step: clamp the horizontal position, react to ground |
| `0x80A9D5D0` | `fn_80a9d5d0` | 88 | high | Common `daEnKanibo_c` state callback: arm the node and optionally play it. |
| `0x80A9D630` | `fn_80a9d630` | 92 | high | Death-state copy of [`fn_80a9d5d0`]. |
| `0x80AA00A0` | `fn_80aa00a0` | 292 | high | Emits the landing smoke for a `daEnKanon_c` animation frame. |
| `0x80AA01D0` | `fn_80aa01d0` | 148 | high | Emits the landing smoke at the actor's current position. |
| `0x80AA0270` | `fn_80aa0270` | 88 | high | Steps one `daEnKanon_c` joint towards its target with a maximum delta. |
| `0x80AA02D0` | `fn_80aa02d0` | 44 | high | Integrates one `daEnKanon_c` vertical joint and clamps it to `+0x118`. |
| `0x80AA0340` | `fn_80aa0340` | 48 | high | Returns the current `0x801625B0` query result. |
| `0x80AA0370` | `fn_80aa0370` | 216 | high | Starts the `daEnKanon_c` contact reaction when the actor is in Walk/Turn. |
| `0x80AA0450` | `fn_80aa0450` | 104 | high | Advances the `daEnKanon_c` reaction timer. |
| `0x80AA04C0` | `fn_80aa04c0` | 84 | high | `daEnKanon_c` FumiStop entry: select the side speed and clear the timer. |
| `0x80AA0700` | `fn_80aa0700` | 204 | high | `daEnKanon_c` Walk step: landing-frame effect and Walk/Turn transitions. |
| `0x80AA07D0` | `fn_80aa07d0` | 92 | high | `daEnKanon_c` impact entry: zero the motion and start clip `3`. |
| `0x80AA0840` | `fn_80aa0840` | 140 | high | `daEnKanon_c` Damage step: return to the saved state when the clip ends. |
| `0x80AA08D0` | `fn_80aa08d0` | 132 | high | `daEnKanon_c` hit/knockback entry: choose the `+0x877` speed row. |
| `0x80AA0970` | `fn_80aa0970` | 264 | high | `daEnKanon_c` Wait step: brake, wait for the clip, switch to Wait. |
| `0x80AA0AC0` | `fn_80aa0ac0` | 208 | high | `daEnKanon_c` GetUp step: brake, countdown and GetUp switch. |
| `0x80AA0BB0` | `fn_80aa0bb0` | 108 | medium | `daEnKanon_c` death entry: release the effect and possibly retire. |
| `0x80AA0F90` | `fn_80aa0f90` | 144 | high | `daEnKanon_c` damage entry: seed the six motion words from `+0x87A`. |

## Verification

- Every address and byte size above matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv` after the `0x80AA0340`
  correction.
- Every evidence file exists and contains the control flow the body claims.
- Each address has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was present in `ledger/functions.csv` before this salvage.
- No placeholder or `todo!`/unimplemented body exists in the module.

## Note on the aborted run

The bodies were written in a single pass and never reviewed by their author, so
each one was re-checked here at three levels: the doc block's address and byte
size against `functions.tsv`, the cited evidence file's existence, and the
body's own structure against the export. One defect was found and fixed (the
`0x80AA0340` size); no body was discarded.
