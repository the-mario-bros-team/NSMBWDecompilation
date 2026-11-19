# Coverage - wave 7 (`d_basesNP` water, rotating-pipe and prop actors)

## How to read this file

This page records the seventh translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/bases.rs`, with synthetic addresses, Rust names, plain-English
   roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - the *Selection* section explains how ten actor
   classes were chosen and why each is covered as a complete lifecycle triple
   (state table, state destructor, constructor).
3. **What is still missing?** - *Deferred candidates* lists neighbouring
   functions intentionally left for later, and *Unknowns preserved* records
   every unresolved address or role instead of guessing it.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or class
  attribution is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8000xxxx`, `0x8030xxxx`, `0x8032xxxx`, `0x8037xxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 5 | Wave 7 | Cumulative |
|---|---:|---:|---:|
| Translated functions in `game/bases.rs` | 113 | 30 | 143 |
| Water actors | 0 | 15 | 15 |
| Rotating-pipe actors | 0 | 6 | 6 |
| Spring mushroom / treasure box / Peach block | 0 | 9 | 9 |
| State registrations covered | 67 | 40 | 107 |
| Selected bytes | 28,472 | 11,060 | 39,532 |
| Coverage of indexed bytes (1,109,860) | 2.57 % | 1.00 % | 3.56 % |
| Coverage of eligible entries (5,409) | 2.09 % | 0.55 % | 2.64 % |

The module still contains 5,615 function entries and 79 `StateID`-bearing
files. Confidence is **20 high** and **10 medium**; there are no `partial` or
`stub` rows.

## Selection

The pass was evidence-first and lifecycle-first:

1. **Started from `functions.tsv` and the existing `bases.rs`.** All 113
   wave-2/3/4/5 addresses were removed from the candidate set before
   selection.
2. **Swept the remaining `s_da<Class>_c__StateID` strings.** The ten classes in
   this pass were chosen because each has a complete, self-contained story: a
   state registration table with three or more states, the state destructor
   passed to `fn_80600160`, and a constructor that installs a class vtable.
3. **Required complete control flow.** Functions whose only body is a
   `0x802DCBFC` forwarder were rejected again (`0x80751400`, `0x80751430`,
   `0x80753170`, `0x807531A0`, `0x80778BF0`, `0x80778BC0`, `0x806EE5D0`,
   `0x806EE600`, `0x8073ECB0`, `0x8073ECE0`). The paired-single
   reconstruction at `0x80651640` and the truncated `0x80651590` remain
   rejected.
4. **Reused the verified registration template.** Every selected table builds
   0x30-byte descriptors at stride `0x40`, uses the same nine-word parameter
   order `6,7,8,3,4,5,0,1,2`, and calls [`fn_80600160`](../src/game/bases.rs).
   The existing `register_state_desc` helper covers 39 of the 40 descriptors;
   `daPeachCastleBlock_c` adds a tenth word at descriptor `+0x30`.
5. **Verified class names against the state strings.** The StateID strings
   were read from the REL data section, not guessed. Constructors carry no
   class string of their own, so their attribution is marked `// inferred:`
   from the vtable they install.

## Clusters

### Water actors (`0x80751060..0x80756700`)

Five three-state water actors. Each row is a table, a destructor and a
constructor; the destructor is exactly the callback stored in the table.

- `daWaterBubbleCannon_c` (`0x80751060`): `StateID_S...`, `StateID_P...`,
  `StateID_M...`; vtable `0x808129D8`; constructor `0x80751460` (`0x3E0` bytes,
  vtable `0x80812A20`).
- `daWaterMove_c` (`0x80752DD0`): `StateID_Wait`, `StateID_Udmove`,
  `StateID_Lrmove`; constructor `0x807531D0` (`0x1A4C8` bytes, vtable
  `0x80812FB0`) with two motion holders, two effect controllers and four
  `0x802DC9A0` arrays.
- `daWireMove_c` (`0x80756240`): three states plus sixteen word copies from
  `0x807CE960..0x807CE984`; constructor `0x80756700` (`0x640` bytes, vtable
  `0x80813A18`, interface vtable `0x80813B00`).
- `daWaterAllRail_c` (`0x80754A30`): `StateID_Search`, `StateID_RailMo...`,
  `StateID_RailEn...`; constructor `0x80754E30` (`0x518` bytes, vtable
  `0x808134F0`).
- `daWaterXinfinitMove_c` (`0x80753CE0`): three states; constructor
  `0x807540E0` (`0x1A4C0` bytes, vtable `0x80813240`) with the same layout as
  `daWaterMove_c`.

### Rotating pipe actors (`0x807306F0..0x807327D0`)

- `daRotDokan_c` (`0x807306F0`): `StateID_Ready`, `StateID_Wait`,
  `StateID_Turn`; constructor `0x80730AF0` (`0xA08` bytes, vtable
  `0x8080D0F8`, interface vtable `0x8080D1D8` at `+0x3D0`, self at `+0x3D4`).
- `daRotDokanCannon_c` (`0x807323D0`): three states (`Rota...`, `Rota...`,
  `Retu...`); constructor `0x807327D0` (`0x460` bytes, vtable `0x8080D340`).

### Spring mushroom, treasure box and Peach-castle block

- `daElasticKinoko_c` (`0x80679200`): nine states in four movement pairs plus
  five parent variants; constructor `0x80679A70` (`0x3E0` bytes, vtable
  `0x807EBBD8`).
- `daStrongBox_c` (`0x8073E560`): eight states from `StateID_Wait` through
  `StateID_StopWait`; constructor `0x8073ED10` (`0x548` bytes, base
  constructor variant `0x80094E80`, vtable `0x8080F348`).
- `daPeachCastleBlock_c` (`0x80720050`): two states, two vtables
  (`0x80809EB4`/`0x80809EE8`); the first descriptor receives an extra word at
  `+0x30` from the constant getter `0x8071F940`. Constructor `0x807205D0`
  allocates `0x398` bytes and delegates to `0x80720630`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave7-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80679200` | `fn_80679200` | register the nine `daElasticKinoko_c` states | translated | high | `80679200_FUN_80679200.c` |
| `0x806798F0` | `fn_806798f0` | destroy a registered spring-mushroom state | translated | high | `806798f0_FUN_806798f0.c` |
| `0x80679A70` | `fn_80679a70` | construct the `0x3E0`-byte spring mushroom | translated | medium | `80679a70_FUN_80679a70.c` |
| `0x80720050` | `fn_80720050` | register the two `daPeachCastleBlock_c` states | translated | high | `80720050_FUN_80720050.c` |
| `0x80720290` | `fn_80720290` | destroy a registered Peach-castle block state | translated | high | `80720290_FUN_80720290.c` |
| `0x807205D0` | `fn_807205d0` | allocate and construct the `0x398`-byte Peach block | translated | medium | `807205d0_FUN_807205d0.c` |
| `0x807306F0` | `fn_807306f0` | register the three `daRotDokan_c` states | translated | high | `807306f0_FUN_807306f0.c` |
| `0x80730970` | `fn_80730970` | destroy a registered `daRotDokan_c` state | translated | high | `80730970_FUN_80730970.c` |
| `0x80730AF0` | `fn_80730af0` | construct the `0xA08`-byte rotating pipe | translated | medium | `80730af0_FUN_80730af0.c` |
| `0x807323D0` | `fn_807323d0` | register the three `daRotDokanCannon_c` states | translated | high | `807323d0_FUN_807323d0.c` |
| `0x80732650` | `fn_80732650` | destroy a registered rotating-cannon state | translated | high | `80732650_FUN_80732650.c` |
| `0x807327D0` | `fn_807327d0` | construct the `0x460`-byte rotating cannon | translated | medium | `807327d0_FUN_807327d0.c` |
| `0x8073E560` | `fn_8073e560` | register the eight `daStrongBox_c` states | translated | high | `8073e560_FUN_8073e560.c` |
| `0x8073EB90` | `fn_8073eb90` | destroy a registered treasure-box state | translated | high | `8073eb90_FUN_8073eb90.c` |
| `0x8073ED10` | `fn_8073ed10` | construct the `0x548`-byte treasure box | translated | medium | `8073ed10_FUN_8073ed10.c` |
| `0x80751060` | `fn_80751060` | register the three `daWaterBubbleCannon_c` states | translated | high | `80751060_FUN_80751060.c` |
| `0x807512E0` | `fn_807512e0` | destroy a registered bubble-cannon state | translated | high | `807512e0_FUN_807512e0.c` |
| `0x80751460` | `fn_80751460` | construct the `0x3E0`-byte bubble cannon | translated | medium | `80751460_FUN_80751460.c` |
| `0x80752DD0` | `fn_80752dd0` | register the three `daWaterMove_c` states | translated | high | `80752dd0_FUN_80752dd0.c` |
| `0x80753050` | `fn_80753050` | destroy a registered `daWaterMove_c` state | translated | high | `80753050_FUN_80753050.c` |
| `0x807531D0` | `fn_807531d0` | construct the `0x1A4C8`-byte water mover | translated | medium | `807531d0_FUN_807531d0.c` |
| `0x80753CE0` | `fn_80753ce0` | register the three `daWaterXinfinitMove_c` states | translated | high | `80753ce0_FUN_80753ce0.c` |
| `0x80753F60` | `fn_80753f60` | destroy a registered infinite-water state | translated | high | `80753f60_FUN_80753f60.c` |
| `0x807540E0` | `fn_807540e0` | construct the `0x1A4C0`-byte infinite water mover | translated | medium | `807540e0_FUN_807540e0.c` |
| `0x80754A30` | `fn_80754a30` | register the three `daWaterAllRail_c` states | translated | high | `80754a30_FUN_80754a30.c` |
| `0x80754CB0` | `fn_80754cb0` | destroy a registered all-rail state | translated | high | `80754cb0_FUN_80754cb0.c` |
| `0x80754E30` | `fn_80754e30` | construct the `0x518`-byte all-rail actor | translated | medium | `80754e30_FUN_80754e30.c` |
| `0x80756240` | `fn_80756240` | register the three `daWireMove_c` states and copy config words | translated | high | `80756240_FUN_80756240.c` |
| `0x80756580` | `fn_80756580` | destroy a registered wire-move state | translated | high | `80756580_FUN_80756580.c` |
| `0x80756700` | `fn_80756700` | construct the `0x640`-byte wire mover | translated | medium | `80756700_FUN_80756700.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 20 high confidence, 10 medium
(the ten constructors).

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x80751400`, `0x80751430`, `0x80753170`, `0x807531A0` | byte-identical forwarders to the unresolved indirect stub `0x802DCBFC`. |
| `0x80751340`, `0x807513D0`, `0x807530B0`, `0x80753140` | vtable methods with no exported C file. |
| `0x807514B0`, `0x80753380`, `0x80756830`, `0x80754EA0`, `0x80754290`, `0x80730BE0`, `0x80732820` | actor-class vtables' own slot functions (draw/execute); the vtable entries are resolved but their C bodies were outside this pass. |
| `0x807204B0` | `daPeachCastleBlock_c` name/path comparison helper (`0x802E1C40`/`0x802E1AB4`); safe to batch next. |
| `0x80720230` | second `daPeachCastleBlock_c` state destructor; byte-identical to `0x80720290`. |
| `0x80720630` | `daPeachCastleBlock_c` construction body called by `0x807205D0`; its own body is 64 bytes and can be added with the next Peach-castle pass. |
| `0x80679AC0`, `0x80679B50` | `daElasticKinoko_c` actor-vtable slots, not yet translated. |
| `0x806EE630` | `daLineTrain_c` constructor (1,459 bytes); the class table `0x806EE170` is still deferred. |
| `0x80778C20` | `daWmSandPillar_c` nine-state table; its constructor slot is data, so lifecycle evidence is incomplete. |
| `0x80651640` | still **rejected**: paired-single reconstruction. |
| `0x806043C0`, `0x806083B0`, `0x8060CBE0`, `0x8060F010`, `0x80618D00` | remaining menu/state tables; unchanged from wave 4/5. |

## Unknowns preserved

1. **Constructor class attribution.** None of the ten constructors contains a
   class string. Each is tied to its class only by the vtable it installs and
   by adjacency to the `StateID` table; all ten rows are therefore
   `medium` confidence and the Rust carries `// inferred:` notes.
2. **`0x80066FC0` and `0x80094E80`.** Both are base-actor constructors called
   by address. Ghidra drops their `this` argument; the Rust passes the newly
   allocated object and documents the omission.
3. **`0x807205D0` lost argument.** The C prints `FUN_80720630()` with no
   arguments, but the callee body uses its first argument as the object. The
   Rust passes `actor` and marks the call `// observed:`.
4. **`0x8071F940`.** The two-instruction getter returns the constant
   `0x80377A04`; it is called through `call0_ret` and the returned word is
   stored at descriptor `+0x30`.
5. **`0x802DC9A0`.** The array builder takes
   `(destination, label/vtable, element-constructor, element-size, count)`.
   The element constructor `0x80601CD0` is not translated; the call is kept by
   address with an `// unknown:` note.
6. **`daWireMove_c` copy block.** Sixteen words are copied from
   `0x807CE960..0x807CE984`; the gaps in the destination are intentionally not
   written, matching the C.
7. **`daPeachCastleBlock_c` second destructor.** `0x80720230` is retained in
   the table but not translated because it is byte-identical to `0x80720290`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 143 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files.
