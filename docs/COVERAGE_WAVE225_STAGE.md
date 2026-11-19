# Coverage - Waves 224-225 (`game::stage` complete, main.dol complete)

## Summary

| Wave | File | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| 224 | `src/game/stage.rs` | 60 | 3,988 | 58 | 2 |
| 225 | `src/game/stage.rs` | 12 | 2,112 | 10 | 2 |

`game::stage` is now **1226 / 1226**, and with it **main.dol is 10,276/10,276**
— every function in `work/ghidra/main-gb/export/functions.tsv` is translated.
Project total: **20,250 / 20,252 (99.99%)**; only the two documented
`game::bases` deferrals (`0x806E2BF0`, `0x80697090`) remain.

## What landed

Wave 224 was thunk-dense: 28 of its 60 bodies are 12-byte `W84_TYPE_INFO`
accessors (`lis 0x8037; addi 0x7A04; blr`). The substantive bodies were the
conflict test `0x800A78D0`, the per-unit effect spawner `0x800A77A0`, the
nearest-active-player scan `0x800AA2A0`, the reaction block `0x800A9550`, the
aim-angle clamp `0x800AA400` and the actor-end path `0x800A9440`.

Wave 225 closed the file with the two interface-gated animation-range tests
(`0x800AB650`, `0x800AB700`), the model-table resolvers
(`0x800ABC30`, `0x800ABFD0`), the effect submitter `0x800AADE0`, the map probe
`0x800AB590`, the record resolvers `0x800AC310`/`0x800ACF20`/`0x800AD090` and
the highest function in the image, `0x800ADB00` (468 B).

## Corrections and recoveries

- `0x800AA090`'s `CONCAT44` is the signed int->float conversion of the
  `extsb`'d **table byte** (bias `0x8042C430`), not of the `+0xAC` float.
- `0x800AA620` multiplies the **full `r3` return word** (`mullw`) and truncates
  only the product.
- `0x800AA7F0`'s `0xF1` bound is `bf 0` (`>=`), so `0x00F1` itself answers 1;
  Ghidra collapses the upper arms to 0.
- `0x800AA2A0` returns the winner index in `r3` (`mr r3,r29` before the restore
  stub), despite Ghidra's `void`.
- `0x800ABC30`/`0x800ABFD0`/`0x800AC310` all drop the live `r4` handle that
  `0x8023A0B0` consumes (`mr r31,r4` -> `0x80239C40`); restored explicitly.
- `0x800A7560`'s export loses the vector argument (`extraout_r4`); the raw
  listing shows it is the incoming `r4`, confirmed by caller `0x80150540`.
- `0x800A9FC0` is single precision throughout (`fadds`/`fsubs`/`fabs`/`frsp`/
  `fdivs`, `fmr f1,f30`); the export's `double` is an FPR spill artefact.
- `0x800AC310`/`0x800ACF20` tsv sizes overshoot by one `OP_0` alignment word
  (112/192 real); `0x800A8AB0`'s two byte clears are `stb`, not word stores.
- `0x800AB700`'s `FUN_801690e0(dVar4, ptr)` is really pointer in `r3`, scalar
  in `f1`.
- `(int)(-u | u) < 0` appears three times as the `!= 0` idiom, not a sign test.
- **Stale prose found:** `W224_F_ZERO_6C44_A224` documents `0x8042C43C` as
  `0.0f`, but the image word is `0x45ABE000` (`5500.0f`). `W224B_F_5500` names
  the same slot correctly; `fn_800aa7b0`'s code is unaffected. Queued for the
  audit round.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave224`: 60 rows, 60 unique, 58 high + 2 medium.
- `check-lane-csv.mjs wave225`: 12 rows, 12 unique, 10 high + 2 medium.
- Ledger: 20,250 rows (99.99%); `game::stage` 1226/1226, main.dol 10,276/10,276.
- `scripts/check.ps1` exit 0.

## Next

Rounds 226-229 are audit/repair: untabled bodies (`0x800FC2D0`, `0x800FC370`,
`0x802C3460`, `0x802C3490` and two thunks), the `check-lane-csv` size column,
the decoder's `fcmpu` operand order, stale `void` signatures, the
`W224_F_ZERO_6C44` prose, and a final parity + gate pass.
