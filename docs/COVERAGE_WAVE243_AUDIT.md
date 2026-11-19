# Audit - Wave 243 (constant-label audit, batch 15)

## 1. Constant label-vs-value audit: 18 real mismatches fixed

A scanner resolved **22,733 address-form constants** in `src/` (including
`SDA_BASE`/`SDA2_BASE`/`R2_BASE +/- offset` and `sda2(-0xNNNN)`) against the
shipped words in `sys/main.dol`, and compared them with numeric hints in the
names. 578 constants carry hints; **18 were wrong**, all fixed with the address
preserved:

| Area | Correction |
|---|---|
| worldmap | `R2_58B4_NEG3` is `-0.5`; `R2_5718_NEG0_1` is `-0.06`; `R2_5714_NEG2000` is `-0.047`; `R2_5710_2800` is `-0.1` |
| bres | **`SPLINE_THREE` and `TWO` swapped**; `MATH_ONE` is `0.5`; `MATH_HALF` is `2.0`; `MATH_DISC` is `4.0`; `MATH_HALF_B` is `-0.5`; `TRIG_HALF` is `0.0` |
| archive | `SDA2_802B4EE0_ONE` is `-2.0` |
| stage | `W88_F_ONE_6970` is `256.0`; `W94_F_HALF_TEXEL` is `2.0` |
| os | `WAVE96_FLOAT_ONE` is `0.0`; `HALF_TURN_A/B` are `+/-90.0`; `W101_HALF` is `256/360`; `W108_SPLINE_THREE`/`TWO` swapped |

All readers were updated, including local variable names, and two Hermite
expressions were adjusted so `u*u*(2u-3)` is preserved. The wave-242 duplicate
`D242_R2_58B4_NEG0_5` was removed. The 40 remaining hits were reviewed as false
positives (halfword `HALF`, function pointers, fractions like `17/14`, `2^52`
magic doubles, role-named `ZERO` slots).

## 2. Bodies translated (54)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 204-217 | 12 | 1,008 |
| C | `dEn_c` 143-152 | 10 | ~690 |
| D | WorldMap vtable 208-223 | 10 | 3,564 |
| E | player-class vtable 207-220 | 12 | 648 |
| F | `0x803502F8` band 0x803503E4+ | 10 | 948 |

## 3. Cross-lane and cross-table findings

- Player slot 219 is the default target of the `+0x374` virtual probed by slot
  213, so that gate is **always false** on this class.
- Slots 212-217 appear again as the six-entry interface table `0x80325778` in
  the same order.
- WorldMap slots 214-219 were already covered by lane B's concurrent
  `WM_PLAYER_VTABLE` batch (same `0x8004A...` targets); lane D skipped them.
- `dEn_c` slots 145/147/148 share the untabled stub `0x800967A0`, translated
  once as a shared helper; a separate translated body
  (`game::stage::fn_800967a0`) exists with a different mixed-ABI mapping, so the
  two are documented as distinct call shapes.
- Lane F finished the `0x803503D0` pane sub-object and the real targets of wave
  242's forwarders.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0, body enforcement armed.
- Ledger 20,250 rows; statuses now 20,028 translated / 215 partial / 7 stub.
