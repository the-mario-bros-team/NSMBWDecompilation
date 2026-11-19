# Coverage - Wave 174 lane B (`game::bases`, 368-388 byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 6 | 2,248 | 5 | 1 |

Six bodies from the 368-388 byte band. Source of the bodies:
`work/wave174/laneB_block.rs`; per-function ledger:
`ledger/agents/wave174-laneb.csv`.

## What this pass covers

- **`cobKoopaShip` prop constructor (1 body).** `0x8076ECF0` initialises the
  `+0x184` layout object, opens `g3d/model.brres` through `0x8042A038`, binds the
  `cobKoopaShip` model at `+0x1A0`, then loops twice over the pointer table at
  `0x80816D30` (`cobKoopaShip`, `cobKoopaShipFloat`) to fill the `0x38`-byte
  records at `+0x3D8`/`+0x410` and set each transform to `1.0`
  (`0x807CF2A8`). It finishes with the `0x448` eight-word animation block
  (`0x80165520`), the two already-translated siblings `0x8076FC60`/`0x8076FF80`,
  and the resource registration `0x800F6710(actor, "cobKoopaShip",
  "g3d/model.brres", "cobKoopaShip_SV", 1)`.
- **`Wm_2d_continue` spawn (1 body).** `0x8060A950` resolves the player node
  through the permutation table `0x807C6908 = [0, 1, 3, 2]` at
  `owner + table[index] * 4 + 0x284`, then builds a 15-word record: the offset
  vector `(node[+0x90] - 0.5*0.16*node[+0x4C]*count, node[+0xA0], 0.0)` followed
  by a straight copy of the twelve-word transform at `+0x84..+0xB0`. The mode
  word `0x80428732 == 3` swaps the `0x800B32E0` projection + `0x8008FB60`
  submission for `0x800B37E0` + `0x8016C890`.
- **1-Up item mode check (1 body).** `0x8078EEE0` gates on `0x800FD2E0()` and
  then tests `0x800F9840(world[+0x54])` against `4` and `9`. Mode `4` builds a
  20-byte name with `0x800F9370`, opens the scene item list
  (`0x801005C0`/`0x800DFD20`/`0x800DFDD0`) and reports a hit when an entry's
  displaced name matches `cobKinoko1up` (`0x8081A824`) over 12 bytes. Mode `9`
  reports a hit when `world[+0x5C]` is `3` or `6`.
- **Ship ride animation (1 body).** `0x807BFE20` accepts states `0`/`1` of
  `+0x608` (state `0` first calls `0x807BF560`) and drives the `+0x60C`
  parameter block while `+0x5F4` is set: either `0x8015F140(1.0, 0.05, 0.09,
  0.03, this+0x60C)` or the `1 / (int)0x8082156C` ramp through the `0x807BE3D0`
  ease curve. It then writes the `+0x5EC` lerp towards `2400.0` and clears
  `+0x604`/`+0x5F4`/`+0x5F0`/`+0x624`/`+0x608` once `|+0x60C - 1.0|` is within
  `1.1920929e-7`.
- **Run-through collection wrapper (1 body, medium).** `0x8065C9A0` calls the
  untranslated scene sweep `0x8065CB20` with its own argument registers, then
  [`fn_8065da00`] for the remaining capacity, adds the two counts, picks
  interface sound `6` or `7` and submits the player position through
  `0x8006A3F0`/`0x80198C30` (event `0x5CE`).
- **Ship input dispatch (1 body).** `0x8076F970` splits one event into a press
  half and an unconditional half over ids `0x1A`, `0x3F`, `0x94` and `0x95`.
  `0x94`/`0x95` arm a `20`/`30` frame hold (`0x80816C04`/`0x80816C0C`) and set
  `+0x4BC` to `1.8` (`0x80816C00`); the second half either decrements the hold or
  calls the `+0x60` vtable slot `+0x68`, and every other id sets `+0x139`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8060A950` | `fn_8060a950` | 372 | high |
| `0x8065C9A0` | `fn_8065c9a0` | 376 | medium |
| `0x8076ECF0` | `fn_8076ecf0` | 368 | high |
| `0x8076F970` | `fn_8076f970` | 388 | high |
| `0x8078EEE0` | `fn_8078eee0` | 372 | high |
| `0x807BFE20` | `fn_807bfe20` | 372 | high |

## Deferred

None. Every body in this lane is translated; the single medium row is a body
whose *callee* (`0x8065CB20`, 3,788 bytes) is still untranslated, not a body that
was guessed.

## Corrections and notes

- **`0x8060A950` re-opened from the wave-86 deferral.** That pass recorded the
  body as untranslatable because the Ghidra stack slots are named out of address
  order. The raw words give the mapping exactly: `r1+0x14` receives the computed
  X, `r1+0x18` the word at `+0xA0`, `r1+0x1C` the constant `0.0`
  (`0x807C68D0`), and `r1+0x20..r1+0x4C` the twelve consecutive words at
  `+0x84..+0xB0`. That is a 0x3C-byte record, not the 0x34 the wave-86 note
  quoted, and the earlier "slot-to-matrix-element" objection no longer holds.
- **`fcmpo` sense at `0x8065CA5C`.** The word is `40810024` (`bc 4,1`), i.e.
  branch on `cr0.GT`, and the comparison at `0x8065CA58` is
  `fcmpo cr0, f0, f2` with `f0 = total` and `f2 = 5.0` (`0x807E7038`). The body
  therefore branches away when `total > 5`, and the sound-`7` arm is only reached
  from the `total <= 0` side. Ghidra's rendering has the two arms swapped; the raw
  words win.
- **A-form `fmuls` operand.** For opcode-59 `fmuls` the destination is
  `(w >> 21) & 0x1F`, `FRA` is `(w >> 16) & 0x1F` and the third operand is
  `(w >> 6) & 0x1F`. `0x8060A9B4`/`0x8060A9C4`/`0x8060A9D4` are therefore
  `0.16 * node[+0x4C]`, `* count` and `0.5 * ...`, matching Ghidra's four-factor
  product. Decoders that read the third operand from bits 11-15 or 21-25 report
  `0.16 * count * count` and lose the `+0x4C` factor.
- **`0x807BE3D0` arity.** The callee's own export takes a single `f64` and never
  reads `f2`, yet the caller loads `0.2` (`0x807D0B20`) into `f2` at
  `0x807BFEAC`. Ghidra's caller dump prints a two-argument call. The `f2` value is
  dead: the body calls the address through `call_f64_u32` with a zero `r3`,
  because no one-`f64` wrapper exists in `src/game/bases.rs`.
- **`0x8065CB20` call ABI.** The call sits at the very top of `fn_8065c9a0`,
  before `r3`/`r4`/`r5` and `f1`/`f2`/`f3` are touched, so the callee receives
  exactly the six incoming arguments. It is emitted with
  `call_f32x3_u32x4(..., 0)`: the trailing `r9` is unused by the callee and no
  three-`f32`/three-`u32` wrapper exists. `0x8065CB20` returns its count in `r3`
  through the `0x802DCD8C` restore helper, which is why the caller can use its
  result even though Ghidra types the callee `void`.
- **`0x8076ECF0`/`0x8076F970` recover a dropped `this`.** Both start with a
  save/restore helper (`0x802DCD68`) or lose the argument through an unprototyped
  callee; the `mr r24,r3` / `mr r31,r3` stores in the raw words restore it.
  `0x800F6710` is called with five arguments (actor, class name, BRRES, `_SV`
  name, `1`).
- **`s16` loads.** `0x8076F970` uses `lha` for the `20`/`30` hold values; the body
  reproduces them as `read_u16(..) as i16 as i32` because `platform` exposes no
  `read_i16`. Both constants are positive, so the values are unchanged.
- **`0x807BFE20` PS tail.** The `+0x5EC` store comes from a paired-single block
  (`ps_muls0`/`ps_add` at `0x807BFF30..0x807BFF40`); its operands are
  `this[+0x61C] + this[+0x60C] * (2400.0 - this[+0x61C])`, which is what Ghidra
  reconstructs and what the body writes.

## Verification

- `work/wave174/laneB_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib --emit=metadata`); the
  temporary harness was deleted afterwards.
- All six evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset was
  read from the raw words of `work/rels/elf/d_basesNP.elf`.
- `node tools/check-lane-csv.mjs wave174` will report the usual pre-merge
  failure for this lane: the bodies live in `work/wave174/laneB_block.rs` until
  the coordinator appends them to `src/game/bases.rs`. Lane rows, addresses and
  evidence paths are final.
