# Coverage - Wave 102 (`game::worldmap`, `0x80120000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from the
object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the bodies reuse the module's already-verified `R2_*`/`SDA_*`
constants where a name existed and declare every new offset in a `W102_` block
so the pass cannot collide with earlier names.

The bucket had 40 untranslated entries when wave 96 finished. Twenty-six are
written here; the fourteen deliberate skips are listed below.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 26 | 13,416 | 1 | 24 | 1 |

Every row is `translated`; no body is `stub` or `partial`.

## What this pass covers

- **`0x801200C0..0x80121990` - the early fall/splash actor.** `0x801200C0`
  runs the two-state `+0x360` machine (timer `+0x5CC` climbing to
  `0x802F5A9C`, drop scale `+0x5C8` from `0x802F4CA0[+0x618]`, contact word
  `+0x5C0`), probes `+0x5F4` through `0x80075270` and posts
  `0x803756F8`/`0x80375738`/`0x803757B8` on the `+0x394` component.
  `0x80120520` adds the `0x8015F140` descent on `+0xEC` (rates
  `0x802F5A84`/`0x802F5A80`, snap below `0x802F5B2C`) plus the collider and
  `0x8011EE80` contact report. `0x80120B40` is the `+0x604` countdown with the
  `r2 - 0x5A68` collider-layer pair. `0x80120D30` is the full contact ladder
  (`+0xC8` slot, `0x8011F430`/`0x8011F520`, `0x80066630`/`0x80198C30`).
  `0x80121090` is its single-probe variant and `0x80121990` rebuilds the
  `+0xE8..+0xF0` landing triple and the six flag words around `+0x530`.
- **`0x80125670`/`0x80125830` - per-player ground placement.** Both are the
  `0x8015F140`-driven height eases into `+0x3AC`/`+0x3B4`/`+0x3B8`, with the
  target selected from `0x802F4D6C..0x802F4E04` by the pad-mode byte at
  `r13 - 0x56E0 + 0x20`, `0x80125F70`/`0x80125FA0` as the tie-break, and the
  two `r13 - 0x5378`/`- 0x5377` one-shot latches that seed `0x80375C08..C1C`.
  `0x80126310` is called by raw address with the three scratch floats.
- **`0x80125F70` - the `+0x392` slot test.** A branch-free bit trick whose only
  observable output is its sign bit; reproduced literally (low confidence
  because the export never names the predicate).
- **`0x80126A80..0x8012B440` - the jump/trick ladder.** `0x80126A80` gates the
  pose on action `0x1F`, calls `0x80141F40`, `0x8012FB10` and
  `wm_find_selected_actor`, then drives `+0x14A4` states `0`/`1` through the
  `+0x60` slots `0x3C4`/`0x43C`/`0x390`/`0x444`, stream event `0x4C` and the
  five action writes. `0x80127470` is the four-threshold step scale.
  `0x80127B30` is the `+0x1564` pose-`2` branch with streams `6`/`0x0B`/`0x45`.
  `0x80128980` is the wall-slip selection around the `+0x14AC` facing counter.
  `0x80128D60` is the wall-kick start. `0x80129050` toggles the ground-contact
  effect record from the two animation entries and the `0x802F4E28`/`0x802F4E34`
  colour tables. `0x8012A210` is the action abort/reset.
  `0x8012A430` is the three-state grab/impact machine around `+0x2780`/
  `+0x278C`. `0x8012AB00` is the zero-speed gate.
- **`0x8012AE60..0x8012FB10` - water entry and the splash tail.** `0x8012AE60`
  is the swim ladder with the model's `0x8C`/`0x84` slots and the
  `0.5 * |+0x10C|` ramp clamped to `[0.1, 1.0]`. `0x8012B1D0` starts the entry
  (`+0x1A18`/`+0x1A1C` seeds, stream `0x8B`), `0x8012B440` commits or cancels
  it, and `0x8012B5B0` is the per-frame step with the `+0x1A1C`/`+0x1A18`
  timers and the `+0x110`/`+0x11C` speed rewrite. `0x8012E360` stores the
  single ground probe at `+0x27D0`; `0x8012EA10` places the grabbed target
  through `0x80075FD0`/`0x800757B0`/`0x80075CA0`; `0x8012F6D0` is the action
  `0x3B` slide damper and `0x8012FB10` is the `Wm_mr_wfloatsplash` emitter.
- **Already-translated neighbours are called by name:** [`fn_801266b0`],
  [`fn_80128190`], [`fn_801284c0`], [`fn_8012bd20`], [`fn_8012d5b0`],
  [`fn_8012e510`], [`fn_8012e5a0`], [`fn_80141f40`], [`wm_find_selected_actor`],
  [`wm_spindown_effects`], [`wm_water_effect`] and [`wm_wallslip_effect`].

## Note on the paired-single scaffolding

Thirteen of the selected exports open or close with
`if ((unaff_GQR0 & 0x3f00) != 0) { ldexpf(...); }` or the mirrored
`0x3f000000` form. In every one of those records the `ldexpf` result is
discarded and each float operand in the body is a plain `r2` load or a value the
export already computes, so no expression depends on the `GQR0` scale. The
bodies therefore transcribe the arithmetic the export prints and ignore the
dead scaffolding. Records where a value *does* flow through `ldexpf`/`CONCAT`
were not selected.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801200C0` | `fn_801200c0` | 472 | medium |
| `0x80120520` | `fn_80120520` | 500 | medium |
| `0x80120B40` | `fn_80120b40` | 304 | medium |
| `0x80120D30` | `fn_80120d30` | 672 | medium |
| `0x80121090` | `fn_80121090` | 328 | medium |
| `0x80121990` | `fn_80121990` | 292 | medium |
| `0x80125670` | `fn_80125670` | 436 | medium |
| `0x80125830` | `fn_80125830` | 504 | medium |
| `0x80125F70` | `fn_80125f70` | 40 | low |
| `0x80126A80` | `fn_80126a80` | 1,032 | medium |
| `0x80127470` | `fn_80127470` | 308 | medium |
| `0x80127B30` | `fn_80127b30` | 992 | medium |
| `0x80128980` | `fn_80128980` | 376 | medium |
| `0x80128D60` | `fn_80128d60` | 244 | medium |
| `0x80129050` | `fn_80129050` | 416 | medium |
| `0x8012A210` | `fn_8012a210` | 348 | high |
| `0x8012A430` | `fn_8012a430` | 972 | medium |
| `0x8012AB00` | `fn_8012ab00` | 56 | medium |
| `0x8012AE60` | `fn_8012ae60` | 868 | medium |
| `0x8012B1D0` | `fn_8012b1d0` | 480 | medium |
| `0x8012B440` | `fn_8012b440` | 368 | medium |
| `0x8012B5B0` | `fn_8012b5b0` | 1,516 | medium |
| `0x8012E360` | `fn_8012e360` | 160 | medium |
| `0x8012EA10` | `fn_8012ea10` | 940 | medium |
| `0x8012F6D0` | `fn_8012f6d0` | 312 | medium |
| `0x8012FB10` | `fn_8012fb10` | 480 | medium |

## Deliberate skips

Fourteen entries of the 40 remain untranslated. Each one loses an operand that
the C export cannot supply, so a body would have to invent it.

- **`0x80120310` (452).** Eleven `unaff_GQR0` uses with three removed
  unreachable blocks; the paired-single scale reaches the float operands, so the
  values cannot be recovered from the C alone.
- **`0x80121280` (264).** Calls `0x8015F140` with three float operands while the
  callee takes four; the fourth (dead-band) operand is not in the export, and
  every sibling call site passes a different constant there.
- **`0x801252D0` (376).** The export stores `extraout_var` at `+0x49A`; the
  halfword has no source in the C, so the record write cannot be reconstructed.
- **`0x80125D20` (112) and `0x80125FA0` (76).** Both call `0x80125F10` with no
  argument and the export shows neither an actor nor a return value that would
  let it be inferred (wave 96 skipped the same call shape).
- **`0x801260E0` (4) and `0x801260F0` (544).** `0x801260E0` is a four-byte thunk
  onto `0x801260F0`, whose body dereferences `extraout_r4`; the register's
  producer is not in the export, so neither the thunk's signature nor the
  callee's arguments can be stated.
- **`0x80126310` (380).** Uses the 64-bit `0x802DCD74` result both as a pointer
  (`*(undefined4 *)uVar7 = ...`) and as the high word of the actor pointer, the
  same shape wave 101 skipped at `0x801468D0`.
- **`0x801275B0` (68).** Calls `0x80127470` with no argument and stores the
  returned `f64` at `+0xEC`; the actor operand that [`fn_80127470`] needs is not
  in the export, the same shape wave 93 skipped at `0x80137550`.
- **`0x80127600` (724).** Calls `0x80145E90` with both arguments dropped; the
  voice mode cannot be inferred, and every branch below it depends on that mode.
- **`0x80127990` (412).** The `+0x2A64` slot `0x5C` call prints only three of its
  five arguments (the model and the event id are missing), so the event id
  would have to be invented.
- **`0x8012C640` (824).** One `+0x2A64` slot `0x5C` call prints only the three
  floats and drops both the model and the event id; the other branch of the same
  function proves the id is required.
- **`0x8012DE80` (296) and `0x8012DFB0` (356).** Both call `0x80101AA0` twice;
  the second call prints no argument at all, so the second `f1` operand is lost.

One translated record carries a related caveat: `0x8012E360` also shows a
`0x43300000`/`+0x10C0` pair whose value is never read in the body. It is
documented in the doc block as a dropped argument of the probe instead of being
reproduced.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`;
  the 26 sizes sum to 13,416 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 26 were untranslated
  entries of the `0x80120000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`. The only errors in the tree at the end of the pass come
  from `src/runtime/os.rs` (another lane).
- `node tools/check-lane-csv.mjs wave102` reports `OK` for
  `ledger/agents/wave102-worldmap.csv`.

[`fn_801266b0`]: crate::game::worldmap::fn_801266b0
[`fn_80128190`]: crate::game::worldmap::fn_80128190
[`fn_801284c0`]: crate::game::worldmap::fn_801284c0
[`fn_8012bd20`]: crate::game::worldmap::fn_8012bd20
[`fn_8012d5b0`]: crate::game::worldmap::fn_8012d5b0
[`fn_8012e510`]: crate::game::worldmap::fn_8012e510
[`fn_8012e5a0`]: crate::game::worldmap::fn_8012e5a0
[`fn_80141f40`]: crate::game::worldmap::fn_80141f40
[`wm_find_selected_actor`]: crate::game::worldmap::wm_find_selected_actor
[`wm_spindown_effects`]: crate::game::worldmap::wm_spindown_effects
[`wm_water_effect`]: crate::game::worldmap::wm_water_effect
[`wm_wallslip_effect`]: crate::game::worldmap::wm_wallslip_effect
