# Coverage - wave 16 (`d_basesNP` final state tables and remaining constructors)

## How to read this file

This page records the sixteenth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: the last two `StateID` tables, their registered
   destructors, the character-change indicator helpers, and the remaining
   in-place constructors with their allocator wrappers.
2. **Why these functions?** - *Selection* explains how the pass closed the two
   deferred tables and then swept the untranslated constructors.
3. **What is still missing?** - *Deferred candidates* and *Unknowns preserved*
   record the rejected paired-single table, the state-handler bodies, and every
   class role that is only inferred.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a class attribution or a
  vtable role is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006C420`, `0x800C8950`, `0x8015F7C0`, `0x8015F800`, `0x802B9280`,
`0x80371870`, `0x8042A028`, `0x8042CF6C`, `0x8042CF84`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 16 |
|---|---:|
| New translated functions | 30 |
| State registration tables | 2 |
| Registered states described | 46 |
| Registered-state destructors | 2 |
| In-place constructors | 17 |
| Allocator wrappers | 6 |
| Character-change indicator helpers | 3 |
| Selected bytes | 13,480 |
| Translated functions in `src/game/bases.rs` (cumulative) | 375 |
| Cumulative selected bytes | 156,412 (14.09 % of indexed bytes) |

The 30 new bodies select 13,480 of the 1,109,860 indexed bytes (1.21 %) and 30
of the 5,409 eligible entries (0.55 %). Confidence is **14 high** and **16
medium**; there are no `partial` or `stub` rows. The total is 375 function
bodies in `src/game/bases.rs`.

## Selection

The pass followed the two deferred tables first and then swept the remaining
constructor family:

1. **Started from `functions.tsv`, the existing `bases.rs` and
   `ledger/functions.csv`.** All addresses already translated by waves 2-15
   were removed before selection; none of the 30 new addresses appears in an
   earlier ledger row.
2. **Scanned every remaining untranslated C file for `StateID` strings and
   `FUN_80600160`.** Exactly three files matched: `0x807AC320`
   (`dScGameSetup_c`, 11 states), `0x807C3D70` (`dCourseSelectManager_c`, 35
   states) and `0x807BD110` (`dWorldSelect_c`, 13 states). The first two are
   translated here; the third remains rejected.
3. **Rejected `0x807BD110` again.** Its decompilation contains
   `unaff_GQR0`/`ldexpf` and removed unreachable blocks, the same paired-single
   reconstruction failure documented in wave 15. It was not guessed.
4. **Searched for the actor base constructor `0x8006C420` in the untranslated
   slice.** Eighteen matches remained. Seventeen are in-place constructors and
   were selected, together with the six allocator wrappers that call them. The
   remaining match, `0x806C3010`, is a tiny inline factory with no adjacent
   table or lifecycle and is deferred.
5. **Closed the `characterChangeIndicator` cluster.** `0x80600400` names the
   `characterChangeIndicator_00` layout and its five pane strings;
   `0x80600600` is its state/player update; `0x80600510` is the two-byte mode
   gate; `0x80600200`/`0x80600230` are its small allocation pair and
   `0x80600890` is the owning select screen.
6. **Verified every table descriptor mechanically.** All eleven
   `dScGameSetup_c` descriptors use the same nine parameter words at
   `0x8081DE08`; all thirty-five `dCourseSelectManager_c` descriptors use the
   same block at `0x80821FB0`. The extractor checked every descriptor address,
   name address, callback and node against the C file; there were no
   mismatches.
7. **Read the real state strings.** Names came from the REL data image at the
   referenced addresses, not from the truncated Ghidra symbols
   (`..._8081de2c`). Original spellings are preserved.

## Clusters

### Character-change indicator (`0x80600200..0x80600600`)

- `0x80600200` allocates `0x250` bytes and calls the constructor
  `0x80600230`, which installs vtable `0x807D0D78`, builds the layout at
  `+0x70`, and clears `+0x238`.
- `0x80600400` loads `characterChangeIndicator_00.brlyt`, the loop animation
  and the `A00_button` animation, then stores the `P_lamp_08` and
  `T_buttonInfo_00` widgets at `+0x208`, `+0x20C` and `+0x224`.
- `0x80600510` is the mode gate: both `0x8042A2DD` and `0x8042A2DC` must be
  non-zero.
- `0x80600600` compares the applied state at `+0x248` with the requested state
  at `+0x24C`, queries the main.dol player object through `0x8042A028`, and
  starts the `0x0A`/`0x0D` animations.
- `0x80600890` is the select screen that owns the arrow family; its interface
  context is the first `dCharacterChangeSelectArrow_c` descriptor
  `0x80822918`.

### Remaining screen constructors (`0x80608F20..0x80644EF0`)

Six independent in-place constructors install one class vtable, one interface
group and one NW4R wrapper. Their descriptor contexts identify the adjacent
families: `0x80823200` (`dCollectionCoinDate_c`), `0x808232C0`
(`dContinue_c`), `0x80824C40` (`dLetterWindow_c`), `0x80825AA0`
(`dModelPlayManager_c`), `0x808276C0` (`dStaffCreditScore_c`) and
`0x80827B10` (`dTheEnd_c`). Only `0x80608F20` has a sufficiently direct tie to
be named `dCollectionCoinDate_c`; the other five keep `fn_` names and an
`// unknown:` class note.

### Allocator wrappers and model constructors (`0x806F8AA0..0x807A8140`)

Five pairs of `allocate -> construct` functions build effects/models:

- `0x806F8AA0`/`0x806F8B90` (`0xF4` bytes, three 12-byte entries at `+0x7C`).
- `0x806FC9F0`/`0x806FCAE0` (`0x708` bytes, three sub-objects at `+0x224`,
  `+0x34C`, `+0x464`).
- `0x80720600`/`0x80720720` (`0xB8` bytes).
- `0x807A67D0`/`0x807A6800` (`0x384` bytes, three frame blocks).
- `0x807A8110`/`0x807A8140` (`0x29C` bytes, two frame blocks).

The frame blocks start with tag `0x80350078` and copy the shared REL defaults
`0x807D0580..0x807D058C` (first pair) or `0x807D0670..0x807D067C` (second
pair).

### `dCourseSelectManager_c` constructor (`0x807C19D0`)

The constructor passes `0x80834480`, the first `dCourseSelectManager_c` state
descriptor, to `0x8015FC90`, stores itself at the singleton `0x80834470`, and
clears `+0x518..+0x524` and `+0x53C`. That context is the evidence for the
class attribution.

### Final state tables (`0x807AC320`, `0x807C3D70`)

- `dScGameSetup_c` (`0x807AC320`): eleven states from `FadeInEndWait` through
  `NextSceneWait`, vtable `0x8081DFCC`, callback `0x807ACB80`. There is no
  class prelude; the table starts directly with the first descriptor.
- `dCourseSelectManager_c` (`0x807C3D70`): thirty-five states from
  `FadeEndWait` through `LetterWindowWait`, vtable `0x80822710`, callback
  `0x807C57F0`. Its prelude copies `0x8042CF6C` and three REL words into
  `0x80821964..0x80821978`, pushes a descriptor-less node with callback
  `0x807C57D0`, then copies `0x8042CF84` to `0x8083446C`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave16-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80600200` | `fn_80600200` | allocate `0x250` bytes and run the small character-change constructor | translated | medium | `80600200_FUN_80600200.c` |
| `0x80600230` | `fn_80600230` | construct the `0x250`-byte character-change helper | translated | high | `80600230_FUN_80600230.c` |
| `0x80600400` | `fn_80600400` | load the `characterChangeIndicator_00` layout and bind its widgets | translated | high | `80600400_FUN_80600400.c` |
| `0x80600510` | `fn_80600510` | test the two main.dol character-change mode bytes | translated | high | `80600510_FUN_80600510.c` |
| `0x80600600` | `fn_80600600` | advance the indicator state and lamp animation | translated | high | `80600600_FUN_80600600.c` |
| `0x80600890` | `fn_80600890` | construct the character-change select screen that owns the arrows | translated | medium | `80600890_FUN_80600890.c` |
| `0x80608F20` | `fn_80608f20` | construct a `dCollectionCoinDate_c` screen in place | translated | medium | `80608f20_FUN_80608f20.c` |
| `0x80609F90` | `fn_80609f90` | construct the screen sharing the `dContinue_c` descriptor context | translated | medium | `80609f90_FUN_80609f90.c` |
| `0x8061C560` | `fn_8061c560` | construct the screen sharing the `dLetterWindow_c` descriptor context | translated | medium | `8061c560_FUN_8061c560.c` |
| `0x80627C40` | `fn_80627c40` | construct the large screen sharing the `dModelPlayManager_c` context | translated | medium | `80627c40_FUN_80627c40.c` |
| `0x80640480` | `fn_80640480` | construct the screen sharing the `dStaffCreditScore_c` context | translated | medium | `80640480_FUN_80640480.c` |
| `0x80644EF0` | `fn_80644ef0` | construct the screen sharing the `dTheEnd_c` context | translated | medium | `80644ef0_FUN_80644ef0.c` |
| `0x806F8AA0` | `fn_806f8aa0` | allocate `0xF4` bytes and construct a model/effect object | translated | medium | `806f8aa0_FUN_806f8aa0.c` |
| `0x806F8B90` | `fn_806f8b90` | construct the `0xF4`-byte model/effect object | translated | high | `806f8b90_FUN_806f8b90.c` |
| `0x806FC9F0` | `fn_806fc9f0` | allocate `0x708` bytes and construct a scene/screen object | translated | medium | `806fc9f0_FUN_806fc9f0.c` |
| `0x806FCAE0` | `fn_806fcae0` | construct the `0x708`-byte scene/screen object | translated | high | `806fcae0_FUN_806fcae0.c` |
| `0x80720600` | `fn_80720600` | allocate `0xB8` bytes and construct a small actor | translated | medium | `80720600_FUN_80720600.c` |
| `0x80720720` | `fn_80720720` | construct the `0xB8`-byte small actor | translated | high | `80720720_FUN_80720720.c` |
| `0x807A67D0` | `fn_807a67d0` | allocate `0x384` bytes and construct a two-body actor | translated | medium | `807a67d0_FUN_807a67d0.c` |
| `0x807A6800` | `fn_807a6800` | construct the `0x384`-byte actor with three frame blocks | translated | high | `807a6800_FUN_807a6800.c` |
| `0x807A8110` | `fn_807a8110` | allocate `0x29C` bytes and construct a smaller two-body actor | translated | medium | `807a8110_FUN_807a8110.c` |
| `0x807A8140` | `fn_807a8140` | construct the `0x29C`-byte actor with two frame blocks | translated | high | `807a8140_FUN_807a8140.c` |
| `0x807A8A90` | `fn_807a8a90` | construct the screen sharing the `0x808333E0` context | translated | medium | `807a8a90_FUN_807a8a90.c` |
| `0x807BB1C0` | `fn_807bb1c0` | construct the screen sharing the `0x808340F8` context | translated | medium | `807bb1c0_FUN_807bb1c0.c` |
| `0x807BDD90` | `fn_807bdd90` | construct the small screen with vtable `0x80821490` | translated | high | `807bdd90_FUN_807bdd90.c` |
| `0x807C19D0` | `fn_807c19d0` | construct a `dCourseSelectManager_c` screen in place | translated | medium | `807c19d0_FUN_807c19d0.c` |
| `0x807AC320` | `fn_807ac320` | register the eleven `dScGameSetup_c` states | translated | high | `807ac320_FUN_807ac320.c` |
| `0x807ACB80` | `fn_807acb80` | destroy a registered `dScGameSetup_c` state | translated | high | `807acb80_FUN_807acb80.c` |
| `0x807C3D70` | `fn_807c3d70` | register the thirty-five `dCourseSelectManager_c` states and prelude | translated | high | `807c3d70_FUN_807c3d70.c` |
| `0x807C57F0` | `fn_807c57f0` | destroy a registered `dCourseSelectManager_c` state | translated | high | `807c57f0_FUN_807c57f0.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 14 high confidence, 16 medium.
The medium rows are constructors and wrappers whose class name is not present in
the body; their flow is explicit and only the attribution is inferred.

## Deferred candidates

| Address range | Why deferred |
|---|---|
| `0x807BD110` | `dWorldSelect_c` state table: 13 states, 2,908 bytes; **rejected** because Ghidra emits `unaff_GQR0`/`ldexpf` and removes unreachable blocks. Wait for a dedicated paired-single pass. |
| `0x807AC320`, `0x807C3D70` | The 46 state handler bodies are still untranslated; this pass closed the tables and their destructors first. |
| `0x806C3010` | 76-byte allocator wrapper for a `0x84`-byte object with vtable `0x807F8068`; left because the class and lifecycle have no adjacent table. |
| `0x80764270`, `0x80765C10`, `0x80779820`, `0x80785BB0`, `0x8078A280`, `0x8078B460`, `0x8078D1B0`, `0x807A1170` | Descriptor-less class registration nodes with parameter defaults; each needs its own class attribution pass. |
| `0x8006C420`, `0x800C8950`, `0x8015F7C0`, `0x8015FC90`, `0x801628C0`, `0x801637D0`, `0x802B9280`, `0x8015F800` | Main.dol/NW4R helpers emitted by address; their names and prototypes are outside this slice. |

## Unknowns preserved

1. **Constructor class attribution.** Fourteen constructors have no class string
   in their body. They are tied to an adjacent family only when the body passes
   a known state-descriptor address to `0x8015FC90`; otherwise the class stays
   `// unknown:` and the row is `medium`.
2. **Shared descriptor contexts.** `0x80608F20` passes `0x80823200`, the first
   `dCollectionCoinDate_c` descriptor. `0x80609F90` passes `0x808232C0`, the
   first `dContinue_c` descriptor; `0x8061C560` passes `0x80824C40`, the first
   `dLetterWindow_c` descriptor; `0x80627C40` passes `0x80825AA0`, the first
   `dModelPlayManager_c` descriptor; `0x80640480` passes `0x808276C0`, the first
   `dStaffCreditScore_c` descriptor; `0x80644EF0` passes `0x80827B10`, the first
   `dTheEnd_c` descriptor; `0x807C19D0` passes `0x80834480`, the first
   `dCourseSelectManager_c` descriptor. Whether these are sub-screens of those
   classes or independent classes sharing the interface context is unresolved.
3. **Player vtable factory.** `0x80600600` calls the `+0x28` slot of the player
   object's `+0x5C` vtable to obtain a second object, then calls that object's
   `+0x10` slot with `0x80371870`. The call shape is explicit; the slot roles
   are `// inferred:`.
4. **Float copy call.** `0x801637D0` is declared by Ghidra with a `double` cast
   but is called with one single-precision load and one pointer. The Rust uses a
   local `extern "C" fn(f32, u32)` wrapper and marks the prototype `// unknown:`.
5. **Compiler thunks.** `0x802DCD70`, `0x802DCD74`, `0x802DCDBC` and
   `0x802DCDC0` save/restore registers or return the object pointer; they have
   no Rust-visible effect and are omitted, following
   `docs/COVERAGE_r80260_802ed.md`.
6. **Shared parameter blocks.** All 11 `dScGameSetup_c` descriptors reuse
   `0x8081DE08`; all 35 `dCourseSelectManager_c` descriptors reuse
   `0x80821FB0`. The Rust expresses this with one `register_state_desc` call per
   state, so every descriptor still receives its explicit vtable, name, node
   and callback.
7. **Global singleton.** `0x807C19D0` stores itself at `0x80834470`. The body
   is explicit; whether that word is a manager pointer or a debug handle is not
   resolved.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 375 translated functions; this pass adds 30,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files. `cargo check` passes (exit 0); the
remaining warnings are in `src/runtime/bt.rs` and `src/runtime/input.rs`,
outside this lane, and `src/game/bases.rs` produces none.
