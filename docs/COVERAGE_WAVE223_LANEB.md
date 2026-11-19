# Coverage: wave 223 lane B (`game::stage`, `0x800A0EA0..0x800A62F0`)

## How to read this file

This document covers lane B of wave 223. All ten bodies are staged in
`work/wave223/laneB_block.rs` and are appended to `src/game/stage.rs` at merge
time; no source file was edited by this pass. The machine-readable rows are in
`ledger/agents/wave223-laneB.csv` (`module = game::stage`).

Every row describes one Ghidra entry point:

- **address** - virtual address of the entry in `main.dol` (image base
  `0x80004000`);
- **name** - the Rust function appended to `src/game/stage.rs`;
- **role** - one plain sentence;
- **status** - `translated` means the whole recorded control flow is present,
  with calls outside the batch left as absolute-address calls;
- **confidence** - `high` when every load/store/branch/mask was read directly
  from the disassembly; `medium` when a load type, a hidden argument or an
  output's role is inferred;
- **bytes** - the `functions.tsv` size (see the size note below);
- **evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

## Summary

| Metric | Count |
|---|---:|
| Functions translated in this batch | 10 |
| Bytes of `functions.tsv` size | 1820 |
| Real instruction extent | 1820 |
| `translated` | 10 |
| `deferred` | 0 |
| Confidence `high` | 9 |
| Confidence `medium` | 1 |

## Function table

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---|---|---|
| `0x800A0EA0` | [`fn_800a0ea0`] | initialise the `+0x414..+0x432` variant record from a source actor | translated | high | 260 | `800a0ea0_FUN_800a0ea0.c` |
| `0x800A2670` | [`fn_800a2670`] | walk five `+0x30` parent links, stopping at the first child whose `+0xC` slot is non-zero | translated | high | 224 | `800a2670_FUN_800a2670.c` |
| `0x800A2840` | [`fn_800a2840`] | forward to the `0x802DCBFC` descriptor dispatcher with the second argument duplicated | translated | medium | 48 | `800a2840_FUN_800a2840.c` |
| `0x800A2BD0` | [`fn_800a2bd0`] | return the shared `Wm` type-info record `0x80377A04` | translated | high | 12 | `800a2bd0_FUN_800a2bd0.c` |
| `0x800A2F40` | [`fn_800a2f40`] | tear the actor down: embedded vtable, two `0x114`-byte arrays, six channels, base destructor | translated | high | 288 | `800a2f40_FUN_800a2f40.c` |
| `0x800A4740` | [`fn_800a4740`] | bind the `"hit"` animation to the `+0x540` channel and play it | translated | high | 104 | `800a4740_FUN_800a4740.c` |
| `0x800A4AC0` | [`fn_800a4ac0`] | advance the two-way `+0x78C` toggle and run the selected record's `+0x14` slot | translated | high | 96 | `800a4ac0_FUN_800a4ac0.c` |
| `0x800A4C10` | [`fn_800a4c10`] | idle test: `+0xD2C` and `+0x74C` both zero | translated | high | 40 | `800a4c10_FUN_800a4c10.c` |
| `0x800A5210` | [`fn_800a5210`] | three-state angle ramp over `+0x102`/`+0xD14` driven by the `0x8042C1A0` step table | translated | high | 532 | `800a5210_FUN_800a5210.c` |
| `0x800A62F0` | [`fn_800a62f0`] | start the riding state: clear `+0x11C`, mirror the child state, seed `+0x76C`, re-bind `0x80357A34` | translated | high | 216 | `800a62f0_FUN_800a62f0.c` |

## Size note (prologue + `blr`)

Each extent was checked against the prologue and the closing `blr`, not just
against the `functions.tsv` column:

| address | `functions.tsv` | ending | `end + 4` | note |
|---|---:|---|---|---|
| `0x800A0EA0` | 260 | `bclr 20,0` at `0x800A0FA0` | `0x800A0FA4` | exact |
| `0x800A2670` | 224 | `bclr 20,0` at `0x800A274C` | `0x800A2750` | exact |
| `0x800A2840` | 48 | `bclr 20,0` at `0x800A286C` | `0x800A2870` | exact |
| `0x800A2BD0` | 12 | `bclr 20,0` at `0x800A2BD8` | `0x800A2BDC` | exact; the next body starts at `0x800A2BE0` |
| `0x800A2F40` | 288 | `bclr 20,0` at `0x800A305C` | `0x800A3060` | exact |
| `0x800A4740` | 104 | `bclr 20,0` at `0x800A47A4` | `0x800A47A8` | exact |
| `0x800A4AC0` | 96 | `bclr 20,0` at `0x800A4B1C` | `0x800A4B20` | exact |
| `0x800A4C10` | 40 | `bclr 20,0` at `0x800A4C34` | `0x800A4C38` | exact |
| `0x800A5210` | 532 | `bclr 20,0` at `0x800A5420` | `0x800A5424` | exact |
| `0x800A62F0` | 216 | `bclr 20,0` at `0x800A63C4` | `0x800A63C8` | exact |

None of the ten is a prologue artefact: every listed size is the real distance
from the entry to the word after the closing `blr`.

## Notes that a reader should carry into the merge

1. **The cluster is a second `Wm` subclass.** It repeats the layout wave 84
   documented for the course actor at `0x80098350..0x80098D90`: the vtable
   pointer at `+0x60`, the variant record at `+0x414..+0x432`, the channel
   objects at `+0x524`/`+0x544`/`+0x584`/`+0x5BC`/`+0x5EC`/`+0x61C`/`+0x654`,
   and the two-element arrays of `0x114`-byte records at `+0x798`/`+0x9C0`
   (whose element destructor is `0x80046410`). `0x800A2BD0` returns the same
   shared type-info record `0x80377A04` as the wave-84 stubs, so the batch
   reuses `W84_TYPE_INFO` rather than defining a second constant. The class
   adds `+0xBE8` (an embedded object with vtable `0x80329958`) and the
   `+0xD14..+0xD3C` ramp state.
2. **`0x802DCBFC` keeps its descriptor in `r12`.** `0x800A2840` sets
   `r12 = param_1 + 0x24` before the call, exactly like the wave-59 wrappers at
   `0x800C7550`/`0x800C7580`; the Rust body forwards the three argument
   registers and documents the `r12` gap. This is the only `medium` row.
3. **The signed word-to-float chain.** `0x800A5210` uses
   `0x4330_0000 | (word ^ 0x8000_0000)` against the `2^52 + 2^31` double at
   `r2 - 0x6D40` (`0x8042C340`), which is the plain signed-int idiom, not the
   `2^52` unsigned one. The neighbouring slots are `0.0f` (`r2 - 0x6D44`),
   `1.0f` (`r2 - 0x6D4C`), `-1.0f` (`r2 - 0x6D1C`), `8192.0f`
   (`r2 - 0x6D18`), `0.9f` (`r2 - 0x6D14`) and `32.0f` (`r2 - 0x6CDC`); the
   step table at `0x8042C1A0` holds `{1, -1, 0, 0, 32, 0, -32, 0, 4, 0, -4, 0}`
   sign-extended per byte.
4. **Branch decoding in `0x800A5210`.** The state-0 advance sits behind
   `41810124` = `bc 12,1`, i.e. branch when `CR0[GT]` (BI 1 is the GT bit, not
   LT), which skips the advance for `timer - 1 > 0`. That agrees with the
   export's `if (iVar4 < 1)`, and the state-1 exit at `0x800A53DC` is the same
   encoding over `0xB01`.
5. **`0x800A4740` binds the string `"hit"`.** `r13 - 0x7BE8` is
   `0x80427A98`, whose image bytes are `68 69 74 00`; `0x8023A0B0` is the
   name-to-id resolver wave 92 already calls with
   `s_clown_damage_down_803126e8` and friends. The two scales it then plays are
   `lfs` loads, so the block adds `w223b_call_f32_ptr`/`w223b_call_f32_ptr2`
   and forwards the values as `f32`; the `f64` helpers wave 92 used for these
   two callees would not put the single-precision value in `f1`.
6. **`0x800A4AC0` passes the record, not the actor.** `lwzu r12,0x798(r3)` both
   loads the vtable and advances `r3` to `this + index*0x114 + 0x798`, so the
   `+0x14` slot is invoked with the selected record as `r3`.
7. **Dead stack copies are omitted.** `0x800A0EA0` writes a second copy of the
   variant record into `r1+0x8..0x25` (`0x800A0F10..0x800A0F8C`) that nothing
   reads, and `0x800A5210` only uses the `0x4330_0000` stack words as
   conversion templates. Neither is reproduced; both are noted in the doc
   blocks.
8. **Name-based reads stay reads.** Every field role (speed word, angle ramp,
   riding state, idle test) is descriptive; all addresses, offsets, call
   targets and constants come from the code bytes in `work/dol/main.elf`.
