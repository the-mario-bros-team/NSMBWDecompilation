# Wave 214, lane E - `0x8006DFC0..0x8006F600` (game::player)

Nine bodies of the gameplay actor's ground-contact resolver. All nine are
`translated`; one is `medium` because its virtual dispatch cannot carry the
hidden `r12` descriptor through `crate::platform`.

Block file: `work/wave214/laneE_block.rs`
Ledger: `ledger/agents/wave214-laneE.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x8006DFC0` | 1232 | `0x8006DFC0..0x8006E490` (1232) | [`fn_8006dfc0`] | high | full contact resolver: probe, four contact tests, snap, vertical-speed clamp |
| `0x8006E490` | 400 | `0x8006E490..0x8006E620` (400) | [`fn_8006e490`] | high | shorter resolver + split-position effect spawn |
| `0x8006E620` | 244 | `0x8006E620..0x8006E714` (244) | [`fn_8006e620`] | high | classify one tile into mode `0..=3` |
| `0x8006E720` | 1668 | `0x8006E720..0x8006EDA4` (1668) | [`fn_8006e720`] | high | landing/step driver: second probe, slope plane, landing commit |
| `0x8006EDB0` | 212 | `0x8006EDB0..0x8006EE84` (212) | [`fn_8006edb0`] | high | landing predicate (class tables `0x802EF520`/`0x802EF534`) |
| `0x8006EF40` | 404 | `0x8006EF40..0x8006F0D4` (404) | [`fn_8006ef40`] | medium | sloped-ground entry, selector resolution, dispatch |
| `0x8006F0E0` | 232 | `0x8006F0E0..0x8006F1C8` (232) | [`fn_8006f0e0`] | high | one grid-record move at the record's own offset |
| `0x8006F1D0` | 520 | `0x8006F1D0..0x8006F3D8` (520) | [`fn_8006f1d0`] | high | scanned column (two `8.0`-step sweeps) |
| `0x8006F3E0` | 544 | `0x8006F3E0..0x8006F600` (544) | [`fn_8006f3e0`] | high | single-point contact test used by both sweeps |

Every size was re-checked against the prologue (`stwu r1,-N`) and the closing
`bclr`; all nine agree with `work/ghidra/main-gb/export/functions.tsv` byte for
byte, so no size in this lane is a prologue artefact. The few pad words between
bodies (`0x8006E714..0x8006E720`, `0x8006EDA4..0x8006EDB0`, `0x8006EE84..
0x8006EE90`) belong to no function.

## Shared frame

All nine read the same actor layout: `+0x14` position (`f32 x`, `f32 y`),
`+0x18` probe origin, `+0x1C` surface normal/velocity, `+0x08`/`+0x0C` grid
records, `+0x88` state flags, `+0xEC` terrain layer. The tile queries
`0x80070BA0`/`0x80070BF0` (point) and `0x80070C40`/`0x80070C60` (whole tile)
take `(x, y, layer)`; the first pair takes floats, the second already-truncated
integers.

## Evidence notes (Ghidra export artefacts)

Three exports mis-state their entry state, because `0x802DCD64`, `0x802DCD70`
and `0x802DCD74` are the **outlined register-save stubs**
(`stw r23..r31` / `stw r26..r31` / `stw r27..r31` to `-0x4..(r11)` followed by
`bclr`; and `0x802DCDB0`/`0x802DCDBC`/`0x802DCDC0` their restore twins), not
getters:

* `FUN_8006dfc0` prints `void FUN_8006dfc0(undefined8, double)` - the real
  arguments are `f1`/`f2` (`dx`/`dy`) and `r3`/`r4` (`this`/`flags`).
* `FUN_8006e720` prints `void FUN_8006e720(void)` - the real arguments are
  `r3`/`r4`.
* `FUN_8006f1d0` / `FUN_8006f3e0` print `(undefined4, ...)` - the arguments are
  recovered from the incoming `r3`/`r4`/`r5` and `f1`/`f2`.

Their `GQR0`/`ldexpf`/`CONCAT` blocks are `psq_st`/`psq_l` register saves of
`f28..f31`, i.e. noise; the corresponding Rust values are ordinary locals.

Constant pool `r2 = 0x80433080`; every float used here was read back from
`work/dol/main.elf` and is kept as an address const (`F32_*_8042BCxx_E`) so the
value stays in the image:

| address | value | use |
|---|---|---|
| `0x8042BC40` | `0.0` | zero |
| `0x8042BC44` | `8.0` | mode-3 margin |
| `0x8042BC48` | `2^52` (`f64`) | int-to-double bias |
| `0x8042BC50` | `2.75` (`f64`) | `fmod` period |
| `0x8042BC58` | `16.0` | one tile / snap margin |
| `0x8042BC60` | `2.0` | second-probe offset |
| `0x8042BC68` | `1.0` | reach floor / window widen |
| `0x8042BC74` | `4096.0` | grid-record fixed-point scale |
| `0x8042BC78` | `0.1` | tile-grid epsilon |
| `0x8042BC88` | `-2.0` | correction threshold |
| `0x8042BC8C` | `0.5` | reach base / vertical clamp |
| `0x8042BC90` | `4.0` | step-up margin |
| `0x8042BC94` | `-4.0` | vertical clamp |
| `0x8042BC98` | `0.6` | flat-ground margin |
| `0x8042BC9C` | `1.5` | landing predicate threshold |

## Partial

`fn_8006ef40` is `// partial:` at one call only: the original loads
`r12 = 0x80355F14`, the three-word pointer-to-member descriptor consumed by
`0x802DCBFC`, and `crate::platform` cannot set `r12`. The visible `f1 = 0.0`,
`r3 = this`, `r4 = selector` arguments are preserved.

## Verification

The block was appended to a copy of `src/game/player.rs` in a scratch crate and
`cargo check --lib` exits 0 with no diagnostic pointing into the appended
range. No new warnings are introduced by this lane.

## No deferrals

All nine bodies are translated; nothing in the range is deferred.
