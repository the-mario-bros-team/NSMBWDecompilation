# Coverage - wave 15 (`d_basesNP` remaining menu/scene state tables)

## How to read this file

This page records the fifteenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 29 bodies appended to
   `src/game/bases.rs`: eleven `StateID` registration tables, their registered
   destructors and the seven in-place constructors that exist in this range.
2. **Why these functions?** - *Selection* explains why this pass closed the
   remaining menu/scene tables instead of the three tables it deferred.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the two remaining tables, the rejected paired-single table and every
   role that is only inferred.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a class attribution is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006C420`, `0x800C8950`, `0x8015F7C0`, `0x8015F800`, `0x802B9280`,
`0x8042CF6C`, `0x8042CF84`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 15 |
|---|---:|
| New translated functions | 29 |
| State registration tables | 11 |
| Registered states described | 174 |
| Registered-state destructors | 11 |
| Actor constructors (in-place) | 7 |
| Selected bytes | 36,192 |
| Translated functions in `src/game/bases.rs` (cumulative) | 345 |
| Cumulative selected bytes | 142,932 (12.88 % of indexed bytes) |

The 29 new bodies select 36,192 of the 1,109,860 indexed bytes (3.26 %) and 29
of the 5,409 eligible entries (0.54 %). Confidence is **22 high** and **7
medium**; there are no `partial` or `stub` rows. The total is 345 function
bodies in `src/game/bases.rs`.

## Selection

The pass stayed evidence-first and lifecycle-first:

1. **Started from `functions.tsv`, the existing `bases.rs` and
   `ledger/functions.csv`.** All 316 addresses already translated by waves
   2-14 were removed before selection; none of the 29 new addresses appears in
   an earlier ledger row.
2. **Scanned every remaining untranslated C file for the state-name
   constructor `func_0x8015f7c0` and the registry call `0x80600160`.** Only
   eleven untranslated tables remained after wave 14. All eleven were taken,
   together with their 88-byte registered destructors.
3. **Closed the families where the constructor exists.** Seven tables have an
   adjacent actor constructor that installs a class vtable and builds the NW4R
   layout/interface group; those were translated too. The other four tables
   (`dCharacterChangeSelectArrow_c`, `dOtasukeInfo_c`, `dScWorld9DeMo_c`,
   `dScCrsin_c`) have no adjacent `0x8006C420`-style in-place constructor in the
   untranslated slice, so only the table and its destructor were claimed.
4. **Rejected one broken table.** `0x807BD110` (`dWorldSelect_c`, 13 states)
   decompiles into `unaff_GQR0`/`ldexpf` code with removed unreachable blocks:
   the same paired-single reconstruction failure documented for `0x80651640`.
   It was left for a dedicated paired-single pass rather than guessed.
5. **Re-verified the descriptor template.** All 174 descriptors build a
   0x30-byte object at stride 0x40, read nine contiguous parameter words,
   install one shared vtable, and call [`fn_80600160`](../src/game/bases.rs)
   with the link node. The extractor checked every descriptor's nine source
   words against the write offsets; there were no mismatches and no
   non-contiguous parameter blocks.
6. **Read the real state strings.** Names were extracted from the REL data
   image at the referenced addresses, not guessed from the truncated Ghidra
   symbols (`..._807d33dc`). Original spellings are preserved (`initial`,
   `keyWait`, `Satge`, `Clouse`, `WaKu`, `Chenge`).

## Clusters

### Character-change select (`0x806017E0..0x806068E0`)

- `dCharacterChangeSelectArrow_c` (`0x806017E0`): five states
  (`OnStageWait`, `OnStageAnimeEndWait`, `ExitWait`, `HitAnimeEndWait`,
  `ExitAnimeEndWait`), vtable `0x807D1230`, destructor `0x80601BE0`.
- `dCharacterChangeSelectBase_c` (`0x806043C0`): twenty states from `initial`
  through `ExitAnimeEndWait`, vtable `0x807D1CF8`, destructor `0x806052E0`.
  A guarded one-time initialiser stores the 16-bit value `0x1E` at
  `0x80822A48`; the first registry node is `descriptor - 0x0C`.
- `dCharacterChangeSelectContents_c` (`0x80606540`): three states
  (`OnStageWait`, `AnimeWait`, `HitAnimeEndWait`), vtable `0x807D2124`,
  destructor `0x806067C0`.
- Constructors: `0x80605400` (Base) and `0x806068E0` (Contents). The Base
  constructor installs class vtable `0x807D1EB0` and derives `+0x288` from the
  low four bits of `+0x04`; the Contents constructor installs `0x807D23F0`,
  builds two helper objects and clears `+0x74..+0xD8`.

### File select (`0x80618D00..0x8061AEC0`)

- `dFileSelect_c` (`0x80618D00`): forty-four states covering the open/select
  flow, the copy and delete sub-flows, the confirmation windows, the progress
  display and the exit. Vtable `0x807D7780`, destructor `0x8061ADA0`.
- Constructor `0x8061AEC0` installs class vtable `0x807D7AD8`, builds the
  interface sub-object at `+0x208` (the imported pointer `0x80329958` is
  stored over it) and the interface group at `+0x330..+0x344`.

### Mini-game wire (`0x80622010..0x80622E00`)

- `dMiniGameWire_c` (`0x80622010`): seventeen states for the title, result,
  bad-result and turn-over face sequences. Vtable `0x807D9F78`, destructor
  `0x80622CE0`.
- Constructor `0x80622E00` installs class vtable `0x807DA178`, builds the
  layout at `+0x74`, and reuses interface context `0x80825580` (`// inferred:`
  the same address is the first `dModeSelect_c` descriptor recorded in wave 14).

### Model-play manager and multi-course select (`0x8062A2C0..0x8062FEF0`)

- `dModelPlayManager_c` (`0x8062A2C0`): nineteen states, vtable `0x807DBD9C`,
  destructor `0x8062B1B0`. Its prelude copies the main.dol words
  `0x8042CF6C`/`0x8042CF84` and the REL block `0x807C79DC..0x807C79E4`,
  pushes a descriptor-less registry node (callback `0x8062B190`), and runs a
  guarded one-time initialiser that stores four 16-bit defaults
  (`0x1E`, `5`, `0x1E`, `0x0F`). The first registry node is
  `descriptor - 0x0C`.
- Constructor `0x8062B2D0` installs class vtable `0x807DC550`, builds helper
  object `+0x208`, the interface group at `+0x32C..+0x344` and clears
  `+0x2DC..+0x328`.
- `dMultiCourseSelect_c` (`0x8062F050`): eighteen states, vtable `0x807DCCC0`,
  destructor `0x8062FDD0`; constructor `0x8062FEF0` installs class vtable
  `0x807DD650` and derives `+0x298` from the low five bits of `+0x04`.

### Point-result multi (`0x8063D7A0..0x8063E7C0`)

- `dPointResultMulti_c` (`0x8063D7A0`): twenty states from `OnSatgeSetup`
  through `NothingWait` (the `Satge` spelling is in the original strings),
  vtable `0x807E0D64`, destructor `0x8063E6A0`.
- Constructor `0x8063E7C0` installs class vtable `0x807E1198` and builds the
  interface group directly at `+0x70..+0x84`, with the layout at `+0xAC`.

### Otasuke info and scene tables (`0x807A9BC0..0x807B38F0`)

- `dOtasukeInfo_c` (`0x807A9BC0`): eight states (`Initial` through
  `OtehonCourseOutWait`), vtable `0x8081DA38`, destructor `0x807AA1F0`. There
  is no in-place constructor in the untranslated slice.
- `dScWorld9DeMo_c` (`0x807B0C50`): ten save/fade/next-scene states, vtable
  `0x8081EB8C`, destructor `0x807B13F0`; the first node is
  `descriptor - 0x0C`.
- `dScCrsin_c` (`0x807B3140`): ten alternating stage/resource/effect states,
  vtable `0x8081F2F8`, destructor `0x807B38F0`. Its prelude calls
  `0x8015F600` to register a two-entry table at `0x80833C90` from the pointer
  table `0x8081EE40`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave15-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806017E0` | `fn_806017e0` | register the five `dCharacterChangeSelectArrow_c` states | translated | high | `806017e0_FUN_806017e0.c` |
| `0x806043C0` | `fn_806043c0` | register the twenty `dCharacterChangeSelectBase_c` states | translated | high | `806043c0_FUN_806043c0.c` |
| `0x80606540` | `fn_80606540` | register the three `dCharacterChangeSelectContents_c` states | translated | high | `80606540_FUN_80606540.c` |
| `0x80618D00` | `fn_80618d00` | register the forty-four `dFileSelect_c` states | translated | high | `80618d00_FUN_80618d00.c` |
| `0x80622010` | `fn_80622010` | register the seventeen `dMiniGameWire_c` states | translated | high | `80622010_FUN_80622010.c` |
| `0x8062A2C0` | `fn_8062a2c0` | register the nineteen `dModelPlayManager_c` states and prelude words | translated | high | `8062a2c0_FUN_8062a2c0.c` |
| `0x8062F050` | `fn_8062f050` | register the eighteen `dMultiCourseSelect_c` states | translated | high | `8062f050_FUN_8062f050.c` |
| `0x8063D7A0` | `fn_8063d7a0` | register the twenty `dPointResultMulti_c` states | translated | high | `8063d7a0_FUN_8063d7a0.c` |
| `0x807A9BC0` | `fn_807a9bc0` | register the eight `dOtasukeInfo_c` states | translated | high | `807a9bc0_FUN_807a9bc0.c` |
| `0x807B0C50` | `fn_807b0c50` | register the ten `dScWorld9DeMo_c` states | translated | high | `807b0c50_FUN_807b0c50.c` |
| `0x807B3140` | `fn_807b3140` | register the ten `dScCrsin_c` states and prelude table | translated | high | `807b3140_FUN_807b3140.c` |
| `0x80601BE0` | `fn_80601be0` | destroy a registered `dCharacterChangeSelectArrow_c` state | translated | high | `80601be0_FUN_80601be0.c` |
| `0x806052E0` | `fn_806052e0` | destroy a registered `dCharacterChangeSelectBase_c` state | translated | high | `806052e0_FUN_806052e0.c` |
| `0x806067C0` | `fn_806067c0` | destroy a registered `dCharacterChangeSelectContents_c` state | translated | high | `806067c0_FUN_806067c0.c` |
| `0x8061ADA0` | `fn_8061ada0` | destroy a registered `dFileSelect_c` state | translated | high | `8061ada0_FUN_8061ada0.c` |
| `0x80622CE0` | `fn_80622ce0` | destroy a registered `dMiniGameWire_c` state | translated | high | `80622ce0_FUN_80622ce0.c` |
| `0x8062B1B0` | `fn_8062b1b0` | destroy a registered `dModelPlayManager_c` state | translated | high | `8062b1b0_FUN_8062b1b0.c` |
| `0x8062FDD0` | `fn_8062fdd0` | destroy a registered `dMultiCourseSelect_c` state | translated | high | `8062fdd0_FUN_8062fdd0.c` |
| `0x8063E6A0` | `fn_8063e6a0` | destroy a registered `dPointResultMulti_c` state | translated | high | `8063e6a0_FUN_8063e6a0.c` |
| `0x807AA1F0` | `fn_807aa1f0` | destroy a registered `dOtasukeInfo_c` state | translated | high | `807aa1f0_FUN_807aa1f0.c` |
| `0x807B13F0` | `fn_807b13f0` | destroy a registered `dScWorld9DeMo_c` state | translated | high | `807b13f0_FUN_807b13f0.c` |
| `0x807B38F0` | `fn_807b38f0` | destroy a registered `dScCrsin_c` state | translated | high | `807b38f0_FUN_807b38f0.c` |
| `0x80605400` | `fn_80605400` | construct a `dCharacterChangeSelectBase_c` screen in place | translated | medium | `80605400_FUN_80605400.c` |
| `0x806068E0` | `fn_806068e0` | construct a `dCharacterChangeSelectContents_c` screen in place | translated | medium | `806068e0_FUN_806068e0.c` |
| `0x8061AEC0` | `fn_8061aec0` | construct a `dFileSelect_c` screen in place | translated | medium | `8061aec0_FUN_8061aec0.c` |
| `0x80622E00` | `fn_80622e00` | construct a `dMiniGameWire_c` screen in place | translated | medium | `80622e00_FUN_80622e00.c` |
| `0x8062B2D0` | `fn_8062b2d0` | construct a `dModelPlayManager_c` screen in place | translated | medium | `8062b2d0_FUN_8062b2d0.c` |
| `0x8062FEF0` | `fn_8062fef0` | construct a `dMultiCourseSelect_c` screen in place | translated | medium | `8062fef0_FUN_8062fef0.c` |
| `0x8063E7C0` | `fn_8063e7c0` | construct a `dPointResultMulti_c` screen in place | translated | medium | `8063e7c0_FUN_8063e7c0.c` |

**Totals:** 29 translated, 0 partial, 0 stub. 22 high confidence, 7 medium.
The medium rows are the seven constructors, whose class attribution is inferred
from the adjacent table and vtable family.

## Deferred candidates

| Address range | Why deferred |
|---|---|
| `0x807C3D70` | `dCourseSelectManager_c` state table: 35 states, 6,744 bytes, and one extra `FUN_80600160` node in the prelude; it needs its own table pass. |
| `0x807AC320` | `dScGameSetup_c` state table: 11 states, 2,140 bytes; no adjacent in-place constructor, so the lifecycle is split. |
| `0x807BD110` | `dWorldSelect_c` state table: 13 states, 2,908 bytes; **rejected** because Ghidra emits `unaff_GQR0`/`ldexpf` and removes unreachable blocks. Wait for a paired-single pass. |
| `0x806017E0..0x807B38F0` | The state handler bodies for the eleven selected classes; the tables and lifecycles are closed first. |
| `0x8006C420`, `0x800C8950`, `0x8015FC90`, `0x80006AA0`, `0x8015F600` | Main.dol/NW4R helpers emitted by address; their names and prototypes are outside this slice. |
| `0x80645F90`, `0x80646180`, `0x806463B0` | `dTheEnd_c`/`dTimeUp_c` state handlers, still deferred from wave 13. |

## Unknowns preserved

1. **Constructor class attribution.** None of the seven constructors contains a
   class string. Each is tied to its class by the vtable family and adjacency
   to the table; those rows are `medium` confidence.
2. **Shared interface contexts.** The Base constructor passes `0x80822F60`,
   which is also the first `dCharacterChangeSelectContents_c` descriptor; the
   ModelPlayManager constructor passes `0x80825F88`, which is also the first
   `dMultiCourseSelect_c` descriptor; the MiniGameWire constructor passes
   `0x80825580`, which wave 14 recorded as the first `dModeSelect_c` descriptor.
   The stores are translated by address; whether the screens deliberately share
   a dispatcher object or the BSS slots are mutually exclusive is unresolved.
3. **Descriptor-less registry node.** `0x8062A2C0` pushes
   `{context 0, callback 0x8062B190, node 0x80825A78}` before its table. The
   callback body is outside this slice.
4. **Two-entry table call.** `0x807B3140` calls `0x8015F600(0x80833C90,
   0x8081EE40, 2)`. The call shape is explicit; the semantics of the callee and
   the two entries are `// unknown:`.
5. **Float word copies.** `0x8062A2C0` copies `0x8042CF6C` to `0x807DB55C` and
   three words from `0x807C79DC..0x807C79E4` with `lfs`/`stfs`; the Rust copies
   the same four bytes with `read_u32`/`write_u32`.
6. **Halfword stores.** The one-time defaults in `0x806043C0` and
   `0x8062A2C0` are `sth` instructions, not byte stores; the Rust uses
   `write_u16` for them and `write_u8` only for the guard bytes.
7. **Node offsets.** Most registry nodes sit at `descriptor - 0x10`; the first
   nodes of `dCharacterChangeSelectBase_c`, `dModelPlayManager_c` and
   `dScWorld9DeMo_c` sit at `descriptor - 0x0C`. Every node is emitted with its
   explicit address.
8. **Compiler thunks.** The `0x802DCD74`/`0x802DCDC0` save/restore thunks that
   bracket the tables are omitted because they have no Rust-visible effect; see
   `docs/COVERAGE_r80260_802ed.md`.
9. **In-place constructors.** The constructors take the object as their first
   argument; the allocator and factory are outside the translated slice, so
   their callers and allocated sizes are not claimed.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 345 translated functions; this pass adds 29,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining 11 warnings are in `src/runtime/bt.rs` and `src/runtime/input.rs`,
outside this lane, and `src/game/bases.rs` produces none. The module also
compiles cleanly in isolation with a copy of `platform.rs` (one pre-existing
dead-helper warning).
