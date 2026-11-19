# Coverage - Wave 71 (`game::bases`, `0x8073` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the `d_basesNP`
REL. Every address is a **static analysis address** from the canonical
synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a runtime
address: the Wii REL loader chooses the real load address, so the numbers below
only identify positions inside the REL image.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; the
cited evidence files are the Ghidra C exports under
`work/ghidra/rel-d_basesNP/export/`. Confidence is `high` when the guards,
stores, constants and call arguments are literal, and `medium` when the role of
a field or of a callee had to be inferred (those bodies carry `// observed:`,
`// inferred:`, `// unknown:` or `// partial:` notes). Constants and table
entries that the C prints as `DAT_xxxx` were resolved by reading the synthetic
ELF at `work/rels/elf/d_basesNP.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 4364 | 21 | 9 |

## What this pass covers

- **Rotating-cannon tail, `0x80730440..0x80730610`** - five bodies of the actor
  whose three state descriptors (`0x80830A98`, `0x80830AD8`, `0x80830B18`) are
  registered by the already-translated [`fn_807306f0`]. `0x80730440` is the
  hit-test step (`0x8072FB30(actor, 0)` then the store `0x8072FBA0`, the
  position writer `0x80730360` and the two follower updates); `0x807304D0` and
  `0x80730610` run the same step and then switch to descriptor `0x80830AD8`,
  and `0x80730570` switches to `0x80830B18` when the translated yaw test
  [`fn_8072fc60`] reports a change. `0x807305D0` is the follower-only body.

- **Unnamed actor, `0x80732B90..0x80733240`** - seven bodies that
  share the state object at `+0x394`: the `0x171` child spawn
  (`0x80064680` with the `+0x38F` type byte, storing the returned record word
  into the `+0x3D0` slot array), the halfword angle stepper `+0x448` driven by
  the signed table `0x807CD7B0[+0x434]` with the `+0x430` direction flag, the
  four state bodies that move the float accumulator `+0x454` between the limits
  `+0x44C`/`+0x450` with the float table `0x807CD7B8[+0x438]` and switch to
  descriptors `0x80830D20`/`0x80830D60`/`0x80830DA0`/`0x80830DE0`, and the
  release wrapper `0x80733240` (`0x8015F800` plus optional `0x802B9280`).
  Those four descriptors are written at run time, so the class name stays
  `// unknown:`.

- **Item model builder, `0x8073D4D0`** - one body that walks `0x802DCD70`'s
  owner/slot pair over the seven item slots and resolves three parallel tables:
  item names at `0x8080EE84` (`I_kinoko`, `I_fireflower`, `I_propeller`,
  `I_penguin`, `I_kinoko`, `I_star`, `I_iceflower`), BRRES paths at `0x8080EF24`
  (`g3d/I_*.brres`) and animation names at `0x8080EF54` (with
  `I_propeller_model` for slot `2`). Per-slot clip start frames come from the
  word table `0x807CDE24` (`35, 32, 32, 32, 35, 32, 32`). Slots `0` and `4`
  additionally build the `I_kinoko_switch` sub-object and select the `2.0` /
  `0.0` rate pair.

- **`daStrongBox_c` state handlers, `0x8073D6B0..0x8073E360`** - seventeen
  bodies that dispatch into the eight descriptors
  `0x808312C8..0x80831488` registered by [`fn_8073e560`]. Their handler
  pointers were resolved from the parameter blocks `0x8080F0D0..` in the
  synthetic ELF, which gives the mapping below. The bodies cover the `open` clip
  rebind (`0x8073D6B0`), the three effects `Wm_ob_treasurebox` (`0x8073D730`),
  `Wm_ob_itemteil` (`0x8073D760`) and `Wm_ob_treasureboxwait` (`0x8073D7F0`),
  the speed decay `0x8073D880`, the coin cue/effect pair
  `0x8073D8D0`/`0x8073D9A0`, the flicker stepper `0x8073DA50`, the scene gate
  `0x8073DB80`, the coin-wait hand-off `0x8073DC50`, and the open/rise/collect
  handlers `0x8073DE60`, `0x8073DED0`, `0x8073DFE0`, `0x8073E080`,
  `0x8073E130`, `0x8073E280`, `0x8073E360`.

- **State-handler mapping (resolved, not guessed).** Each descriptor's parameter
  block stores the handler at `+0x14`; reading those blocks gives
  `StateID_Wait` -> `0x8073DAF0`, `StateID_CoinWait` -> `0x8073DC50`,
  `StateID_AppearWait` -> `0x8073DB80`, `StateID_Open` -> `0x8073DED0`,
  `StateID_KinopioAppear` -> `0x8073E080`, `StateID_ItemAppear` -> `0x8073E130`,
  `StateID_ItemUp` -> `0x8073E280` and `StateID_StopWait` -> `0x8073E360`.
  `0x8073DAF0` is the one handler Ghidra did not export as its own function
  (see the skips), so the five translated handlers plus `0x8073DC50` and
  `0x8073DB80` cover seven of the eight states.

- **Recurring layout.** The treasure-box object keeps its model holder at
  `+0x408`, three animation blocks at `+0x410`, `+0x450` and `+0x490`, three
  effect owners at `+0x530`, `+0x658` and `+0x780`, the item position
  `+0x8A8..+0x8B4`, the rise speed `+0x8B8`, the effect anchor `+0x8C0`, the
  spawned-item handle `+0x8D4`, the ground offset `+0x8D8`, the item index
  `+0x8DC`, the `open`/`wait` variant `+0x8E0`, the required reader count
  `+0x8E8` and the counters/flags `+0x8EC..+0x905`. The two main.dol helpers
  used throughout are the sound manager `0x80429D60` and the position packer
  `0x8006A4B0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80730440` | `fn_80730440` | 104 | medium |
| `0x807304D0` | `fn_807304d0` | 120 | medium |
| `0x80730570` | `fn_80730570` | 80 | high |
| `0x807305D0` | `fn_807305d0` | 52 | medium |
| `0x80730610` | `fn_80730610` | 88 | medium |
| `0x80732B90` | `fn_80732b90` | 100 | high |
| `0x80732C00` | `fn_80732c00` | 84 | high |
| `0x80732C80` | `fn_80732c80` | 84 | high |
| `0x80732D00` | `fn_80732d00` | 124 | high |
| `0x80732DA0` | `fn_80732da0` | 84 | high |
| `0x80732E20` | `fn_80732e20` | 124 | high |
| `0x80733240` | `fn_80733240` | 88 | high |
| `0x8073D4D0` | `fn_8073d4d0` | 476 | medium |
| `0x8073D6B0` | `fn_8073d6b0` | 120 | high |
| `0x8073D730` | `fn_8073d730` | 44 | high |
| `0x8073D760` | `fn_8073d760` | 140 | high |
| `0x8073D7F0` | `fn_8073d7f0` | 140 | high |
| `0x8073D880` | `fn_8073d880` | 44 | high |
| `0x8073D8D0` | `fn_8073d8d0` | 196 | high |
| `0x8073D9A0` | `fn_8073d9a0` | 104 | high |
| `0x8073DA50` | `fn_8073da50` | 120 | high |
| `0x8073DB80` | `fn_8073db80` | 156 | high |
| `0x8073DC50` | `fn_8073dc50` | 528 | medium |
| `0x8073DE60` | `fn_8073de60` | 84 | high |
| `0x8073DED0` | `fn_8073ded0` | 264 | medium |
| `0x8073DFE0` | `fn_8073dfe0` | 136 | high |
| `0x8073E080` | `fn_8073e080` | 132 | medium |
| `0x8073E130` | `fn_8073e130` | 196 | medium |
| `0x8073E280` | `fn_8073e280` | 180 | high |
| `0x8073E360` | `fn_8073e360` | 172 | high |

## Address range

`0x80730440..0x8073E360`, in three runs:

- `0x80730440..0x80730610` - five bodies, interrupted only by the already
  translated state table `0x807306F0` that closes the run;
- `0x80732B90..0x80733240` - seven bodies, interrupted by the two `0x802DCBFC`
  stubs `0x80733360`/`0x80733390` (not in this pass) that precede the already
  translated constructor `0x807333C0`;
- `0x8073D4D0..0x8073E360` - eighteen bodies, interrupted by the truncated
  `0x8073DA10` and by `0x8073DF00`-area helpers left for a later wave.

At lane start the `0x8073` bucket held 183 functions, 29 of them already
translated. This pass adds 30, leaving 124 for later waves.

## Deliberate skips

- `0x80730360` (212) - the paired-single position writer that copies the linked
  actor's halfword angle into `+0x104` and scales it into `+0xAC`/`+0xB0`. The
  export carries `unaff_GQR3`, `ldexpf` and the `CONCAT22`/`CONCAT31` lane
  split, so the scaled angle would be a guess; the three bodies that call it
  keep it as an `abi_call1!(0x8073_0360, actor)` with an `// unknown:` note.
  The same rejection is recorded for `0x80651640`, `0x80682F40` and
  `0x806843B0` in earlier waves.
- `0x8073DA10` (60) - truncated by bad instruction data (`halt_baddata`)
  directly after the `+0x8E0 == 1` guard, the `+0x8F4` decrement and the
  `+0x8F4 != 1` guard. The visible prefix carries no state transition, so only
  the two guards survive; the three callers (`0x8073DED0`, `0x8073E080`,
  `0x8073E130`) call it by address and are marked `medium`.
- `0x8073DAF0` - the `daStrongBox_c::StateID_Wait` handler. It is not a row in
  `functions.tsv`: Ghidra folded it into the neighbouring bodies, so there is
  no byte count and no evidence file to cite.
- The untried remainder of the bucket - `0x80730360..0x80730440` (behind the
  rejected position writer), `0x80730A90..0x807323D0` (the `daRotDokan_c`
  table/constructor block), `0x80733390..0x8073D4D0` (the `houdai`/`penguin2`/
  `shinkai_light` loader block) and `0x8073E360..0x8073FFFF` (the ice/treasure
  tail) - belongs to later waves, not to a rejection.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 29 pre-existing rows of
  this bucket were checked before writing, and none of them is in this pass.
- `node tools/check-lane-csv.mjs wave71` reports `OK` with
  `wave71-bases.csv: 30 rows`.
- `cargo check --message-format=short` reports no diagnostic for
  `src/game/bases.rs`.
