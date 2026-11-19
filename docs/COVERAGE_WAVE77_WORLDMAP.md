# Coverage - Wave 77 (WorldMap player action gates, `0x80120000` bucket)

## How to read this file

Every row below is one `pub unsafe fn` body in `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. The evidence column points at the
Ghidra C export the body was transcribed from; byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`. Field offsets are literal offsets
from the object pointer. No body in this pass stores an `r2` or `r13` constant,
so no data-section reads were needed; the only static tables used are
`0x802F4E14` (the five-entry air scale) and the event records
`0x80375D2C`/`0x80375E6C`/`0x80375F70`/`0x80376130`/`0x80376170`/`0x80376330`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 3864 | 16 | 14 |

## What this pass covers

- `0x80120CF0`, `0x80121440` - the two isolated status/hand-off helpers that
  precede the 0x80125xxx group. They clear `+0x574` bit `0x40000000`, resolve
  the `+0x5C4` handle through `0x8005F900`, hand the actor to `0x8012E510` and
  clear `+0x12C` bits `0`/`1` while setting `+0x178` bit `0x20`.
- `0x80125CC0`, `0x80125F10`, `0x80125FF0` - the player resolver and probe
  group. `+0x392` selects a player actor through `0x8005F900` (slots `0..=3`)
  or `0x8005FB70(index - 4)` (slots `4..=7`); `0x80125FF0` runs one of two
  `0x80075FD0` point probes, tests the `+0x50C` mode byte and the `0x19C` hit
  halfword, then falls through to `0x801260F0`.
- `0x80126630`, `0x801266B0` - the effect/action gate pair. The first selects
  `Wm_mr_sealandsmk_ss` (`0x80130A00`) plus events `0x153`/`0x107`/`0x105` when
  `+0x10D4` bit `0x4000` is set; the second walks input `0xA7`, the `+0x10D4`
  bit `0` and `+0x10D8` bit `0x40` before posting `0x80375F70` or
  `0x80375E2C` through vtable slot `0x294`.
- `0x80126E90`..`0x80127410` - the input-reset and frame-chain block. It clears
  ten inputs (`0x2B/0x1F/0x20/0x21/0xA8/0xAA/0x9F/0x22/0x8F`) and the
  `+0xEA4` state `4`, then runs the shared chain `0x80047D60`/`0x8004C4F0`/
  slot `0x44C`/`0x8004B730`/`0x8004C450` behind the `0x80134B10` gate, and
  wraps the `+0x1564` three-frame counter.
- `0x80127F10`..`0x80128FC0` - the air-scale and action-clear block. The first
  multiplies `+0xEC` by `0x802F4E14[level - 1]` (level clamped to `1..=5`) and
  clears `+0x10D4` bit `0`; `0x801287B0`/`0x80128D20` copy the first word of
  the `0x3C4` vtable slot to `+0x114`; `0x80128FC0` dispatches the `+0x2A78`
  link either to the actor itself or to the `0x8012DBE0` result through
  `0x80129050`.
- `0x80129790`, `0x801299B0` - the full reset pair. It zeroes `+0x251C`/
  `+0x252C`, clears ten inputs and the `+0xEA4` states `3/4/1/0/5`, stores the
  slot-`0x444` heading at `+0x102` and clears input `0x2B` on the linked actor;
  the second only zeroes the `+0x2534` halfword.
- `0x8012AB40`..`0x8012CAD0` - the mode/eligibility block. `0x8012AB40` runs
  `0x8004CF20` and `0x8012AB00`; `0x8012BD20` sets mode `0` and posts `0x164`;
  `0x8012C5E0` reports whether the `0x800000` action bit, the `0x8005E7A0`
  query and the `+0x1090` index allow the action; `0x8012CAD0` is the guarded
  signed halfword decrement used by the timers.
- `0x8012D490`..`0x8012E990` - the action ladder and link hand-off. Inputs
  `0x51`/`0x0F`/`0xBE`, the `0x18000000` mask and slot `0x3C8` gate the
  `0x8005E830` thunk, after which `0x80376330` or `0x80375D2C` is posted.
  `0x8012E140` keeps the nearest eligible target (`+0x27D4`/`+0x27D8`);
  `0x8012E5A0`/`0x8012E6E0` attach, copy or release the `+0x2A78` link;
  `0x8012E760` compares the `+0x1464` name record with `0x80376230` and posts
  `0x80376130` or `0x80376170`; `0x8012E990` clears input `0x18`.
- `0x8012F060`, `0x8012F970`, `0x8012FA00`, `0x8012FEF0` - the closing update
  and action-priority block. `0x8012F060` branches between the normal chain and
  `0x80131990`; `0x8012F970` starts state `1` on a `0x80375E6C` name match;
  `0x8012FA00` posts `0x80375E6C` with argument `0`/`1`/`2` from the action
  bits; `0x8012FEF0` runs slot `0x10` on the `+0x2074`/`+0x219C` records.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80120CF0` | `fn_80120cf0` | 52 | high |
| `0x80121440` | `fn_80121440` | 88 | medium |
| `0x80125CC0` | `fn_80125cc0` | 88 | medium |
| `0x80125F10` | `fn_80125f10` | 92 | medium |
| `0x80125FF0` | `fn_80125ff0` | 228 | medium |
| `0x80126630` | `fn_80126630` | 128 | medium |
| `0x801266B0` | `fn_801266b0` | 248 | high |
| `0x80126E90` | `fn_80126e90` | 176 | high |
| `0x801270F0` | `fn_801270f0` | 124 | medium |
| `0x80127410` | `fn_80127410` | 84 | high |
| `0x80127F10` | `fn_80127f10` | 164 | high |
| `0x801287B0` | `fn_801287b0` | 120 | high |
| `0x80128D20` | `fn_80128d20` | 64 | medium |
| `0x80128FC0` | `fn_80128fc0` | 136 | high |
| `0x80129790` | `fn_80129790` | 276 | medium |
| `0x801299B0` | `fn_801299b0` | 12 | high |
| `0x8012AB40` | `fn_8012ab40` | 52 | medium |
| `0x8012BD20` | `fn_8012bd20` | 76 | high |
| `0x8012C5E0` | `fn_8012c5e0` | 96 | high |
| `0x8012CAD0` | `fn_8012cad0` | 28 | high |
| `0x8012D490` | `fn_8012d490` | 276 | high |
| `0x8012E140` | `fn_8012e140` | 176 | medium |
| `0x8012E5A0` | `fn_8012e5a0` | 172 | high |
| `0x8012E6E0` | `fn_8012e6e0` | 124 | medium |
| `0x8012E760` | `fn_8012e760` | 188 | medium |
| `0x8012E990` | `fn_8012e990` | 8 | high |
| `0x8012F060` | `fn_8012f060` | 108 | medium |
| `0x8012F970` | `fn_8012f970` | 136 | medium |
| `0x8012FA00` | `fn_8012fa00` | 264 | high |
| `0x8012FEF0` | `fn_8012fef0` | 80 | high |

## Deliberate skips

- **GQR reconstruction records.** `0x801200C0`, `0x80120310`, `0x80120520`,
  `0x80120B40`, `0x80120D30`, `0x80121090`, `0x80125670`, `0x80125830`,
  `0x80126310`, `0x80127470`, `0x80128D60`, `0x8012B1D0`, `0x8012C640`,
  `0x8012E360`, `0x8012EA10`, `0x8012F6D0` and `0x8012FB10` all carry
  `unaff_GQR*` blocks (`ldexpf` reconstruction) and usually a removed
  unreachable block, so the paired-single scale factors are not recoverable
  from the C. Waves 36, 60, 64, 73 and 76 skipped the same record class.
- **`0x80125F70`.** Its only statement is
  `(~uVar1 | 3) - (3 - uVar1 >> 1) >> 0x1f`. Evaluated literally with the C
  integer types, that expression is `1` for every one of the 256 possible
  `+0x392` bytes, so Ghidra has mis-rendered the comparison that the
  neighbouring `0x80125F10` implies (the `4..=7` player band). The body cannot
  be reconstructed honestly, so it was left out.
- **`0x80125FA0` and `0x801291F0`.** Both load a new receiver (`+0x60`,
  `+0x2538`) and call a slot with no arguments at all in the export; the
  incoming `this` pointer is not visible, so the argument list would be
  invented. `0x8012FEF0` was taken because its receiver is the embedded record
  address itself and its argument is explicit.
- **`0x80125210`, `0x80125240`, `0x80125270`, `0x801252A0`.** Four small
  record-copy helpers whose final call is an unrecovered jump table
  (`Could not recover jumptable ... Too many branches`); the call target and
  its argument count are unknown.
- **Larger state handlers.** `0x80126A80` (1,032), `0x80127600` (724),
  `0x80127B30` (992), `0x80129500` (652), `0x8012B5B0` (1,516) and the other
  remaining untranslated entries of the bucket were not attempted: they are
  multi-branch state machines that would need their own policy for the many
  dropped callee arguments. They stay as raw addresses in the bodies above.

## Verification

- Every address and byte size matches `functions.tsv`; the 30 sizes sum to
  3,864 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 30 were untranslated
  entries of the `0x80120000` bucket at the start of the pass).
- `cargo check --message-format=short` reports no error and no diagnostic that
  names `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave77` reports no problem naming
  `wave77-worldmap.csv` (30 rows, unique addresses); the one remaining problem
  names another lane's CSV.
