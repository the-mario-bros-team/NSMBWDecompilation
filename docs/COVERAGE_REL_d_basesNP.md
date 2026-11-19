# Coverage — `d_basesNP` REL module 2 (`game::bases`)

## How to read this file

This page records the **first semantic slice** of the `d_basesNP` REL. It
answers:

1. **What is in the module?** — *Inventory*: how many functions Ghidra found,
   how they are distributed, and how many carry class/state names.
2. **What did this pass translate?** — *Selection method* and *Translated
   functions*: one row per function with its synthetic address, Rust name,
   plain-English role, status, confidence, and evidence file.
3. **What should the next pass do?** — *Clusters*, *Deferred candidates*, and
   *Next targets*, so the next agent does not redo the search or over-trust the
   naming.

> **Wave 3 update.** The second pass adds 25 functions: the remaining
> select-screen tails (the first four are in `game::player`), the
> `0x806511D0..0x80651910` state/collision helpers, four class-state
> registration tables, and the registered actor destructor; see
> [COVERAGE_WAVE3_BASES.md](COVERAGE_WAVE3_BASES.md). It also corrects the
> `0x80651390` attribution in this page: that constructor builds a `0x3E0`-byte
> object, not the `daBigShell` actor, because `0x80650E60` reaches `+0x4F4`.

**Address warning.** Every address below is a **synthetic analysis address**.
The REL was relocated by `tools/rel2elf.mjs` to the canonical base
**`0x80600000`**; the entry point is `0x806000F0`. These are **not runtime
addresses**. The real Wii loader chooses a different load address, and every
intra-REL pointer changes with it. `main.dol` addresses (`0x8000xxxx`,
`0x8020xxxx`, `0x8035xxxx`, `0x8042xxxx`) are imported absolute values and are
shown as they appear in the image.

Column meanings:

- **Status** — `translated` = full control flow and every visible call/store is
  represented; `partial` = major effects only, marked `// partial:` in Rust;
  `stub` = address only.
- **Confidence** — strength of the evidence: `high` = explicit bytes/flow;
  `medium` = flow explicit but a role/table/API identity is inferred; `low` =
  mostly guesswork.
- **Evidence** — `work/ghidra/rel-d_basesNP/export/<file>`. That C file is the
  source of truth.

Inline Rust markers follow `docs/RUST_STYLE.md`: `// observed:` (visible in the
Ghidra output), `// inferred:` (role guessed from callers/shape), `// unknown:`
(unresolved address or effect), `// partial:` (deliberately incomplete).

## Inventory

Source: `work/ghidra/rel-d_basesNP/export/functions.tsv` (5,615 data rows plus
the header).

| Metric | Value |
|---|---:|
| Function entries | 5,615 |
| Sum of recorded function sizes | 1,109,860 bytes |
| Average / minimum / maximum size | 197.7 / 4 / 8,344 bytes |
| Entries with size `>= 16` (eligible) | 5,409 |
| Entries with size `< 16` (address-only thunks) | 206 |
| Files whose C references `StateID` | 79 |
| Files whose C references an `s_<Class>_c__` string | 89 |
| Files whose C references any `s_` string | 599 |
| Translated by this pass | 30 |
| Added by wave 3 | 25 |
| Cumulative translated in `game/bases.rs` | 55 |
| Selected bytes | 5,844 (0.53 % of indexed bytes) |
| Coverage of eligible entries | 0.55 % |
| Wave-3 selected bytes | 4,440 |
| Cumulative selected bytes | 10,284 (0.93 % of indexed bytes) |

The sizes are Ghidra's function extents, not the REL text size. The text section
is `0x1C6004` bytes (`0x806000F0..0x807C60F4`), so the indexed functions cover
about 60 % of text; alignment padding, jump tables, and data live in the rest.

### Distribution by 64 KiB block

| Block | Entries | Block | Entries |
|---|---:|---|---:|
| `0x80600000` | 240 | `0x806F0000` | 195 |
| `0x80610000` | 241 | `0x80700000` | 204 |
| `0x80620000` | 204 | `0x80710000` | 159 |
| `0x80630000` | 201 | `0x80720000` | 221 |
| `0x80640000` | 196 | `0x80730000` | 183 |
| `0x80650000` | 185 | `0x80740000` | 233 |
| `0x80660000` | 156 | `0x80750000` | 207 |
| `0x80670000` | 199 | `0x80760000` | 203 |
| `0x80680000` | 176 | `0x80770000` | 182 |
| `0x80690000` | 185 | `0x80780000` | 203 |
| `0x806A0000` | 158 | `0x80790000` | 184 |
| `0x806B0000` | 205 | `0x807A0000` | 212 |
| `0x806C0000` | 205 | `0x807B0000` | 161 |
| `0x806D0000` | 219 | `0x807C0000` | 69 |
| `0x806E0000` | 229 | **Total** | **5,615** |

## Selection method

The pass was evidence-first, not range-first:

1. Loaded `functions.tsv` and computed the size distribution. Entries smaller
   than 16 bytes were excluded: they are `blr`-only veneers, register-save
   helpers, or branch thunks with no recoverable body.
2. Searched every C file for the decompiler's string symbols
   `s_<Class>_c__StateID_<Name>` and `s_<Class>_c__`. This produced 89
   class-named files and 79 state-table files, concentrated in
   `0x806017E0..0x806465F0` (menus/UI actors) and `0x806510C0..` (stage actors).
3. Preferred functions that either:
   - are named by a class/state string, or
   - form a tight call cluster with the registry helpers at `0x80600160` /
     `0x80600180`, or
   - are self-contained constructor/init/model-load routines with explicit
     field offsets.
4. Rejected decompiler failures (`halt_baddata`, paired-single reconstruction
   in `FUN_80651640`) and functions whose only body is `return param;`.
5. Translates faithfully: every visible call becomes a `platform::callN` by
   address, every store/load keeps its offset, and unknown calls are marked
   `// unknown:`. Compiler save/restore thunks in the `0x802DCD70` family are
   documented and omitted (they save registers but have no Rust-visible effect).

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`). Evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/rel-d_basesNP.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806000F0` | `fn_806000f0` | REL prolog: register ctor table, call entry stub | translated | high | `806000f0_entry.c` |
| `0x80600160` | `fn_80600160` | push one `{next, callback, context}` node onto the registry | translated | high | `80600160_FUN_80600160.c` |
| `0x80600180` | `fn_80600180` | destroy the registry list, calling each callback | translated | high | `80600180_FUN_80600180.c` |
| `0x80601380` | `fn_80601380` | advance left/right arrow state from the layout animation | translated | medium | `80601380_FUN_80601380.c` |
| `0x80601500` | `fn_80601500` | confirm arrow choice and play its out animation | translated | high | `80601500_FUN_80601500.c` |
| `0x80601590` | `fn_80601590` | notify the parent when the arrow layout finishes | translated | high | `80601590_FUN_80601590.c` |
| `0x80601D50` | `fn_80601d50` | allocate the `0x2F4`-byte select screen | translated | high | `80601d50_FUN_80601d50.c` |
| `0x80601D80` | `fn_80601d80` | construct the select screen (vtables, layouts, helpers) | translated | high | `80601d80_FUN_80601d80.c` |
| `0x80602160` | `fn_80602160` | load the arrow layout and bind its widgets | translated | high | `80602160_FUN_80602160.c` |
| `0x806023C0` | `fn_806023c0` | clear the selected player slot | translated | high | `806023c0_FUN_806023c0.c` |
| `0x80602460` | `fn_80602460` | choose the first free player slot / validate current | translated | medium | `80602460_FUN_80602460.c` |
| `0x80602600` | `fn_80602600` | test whether another slot already uses a character | translated | high | `80602600_FUN_80602600.c` |
| `0x806026F0` | `fn_806026f0` | map a character id to preview/“taken” state | translated | medium | `806026f0_FUN_806026f0.c` |
| `0x806027C0` | `fn_806027c0` | copy the preview id for the selected slot | translated | high | `806027c0_FUN_806027c0.c` |
| `0x806027E0` | `fn_806027e0` | derive the preview id from the selected slot | translated | medium | `806027e0_FUN_806027e0.c` |
| `0x80602810` | `fn_80602810` | hide all four preview actors | translated | high | `80602810_FUN_80602810.c` |
| `0x80602860` | `fn_80602860` | commit the preview and mark state 2 | translated | high | `80602860_FUN_80602860.c` |
| `0x80602900` | `fn_80602900` | copy the selected widget transform to the manager | translated | high | `80602900_FUN_80602900.c` |
| `0x80602A10` | `fn_80602a10` | confirm the highlighted slot (both game modes) | translated | high | `80602a10_FUN_80602a10.c` |
| `0x80602B70` | `fn_80602b70` | start the out transition and show the cursor | translated | high | `80602b70_FUN_80602b70.c` |
| `0x80602DB0` | `fn_80602db0` | handle “decide”: sound + slot choice | translated | medium | `80602db0_FUN_80602db0.c` |
| `0x80603060` | `fn_80603060` | cancel, or re-commit a prepared slot | translated | high | `80603060_FUN_80603060.c` |
| `0x80609B70` | `fn_80609b70` | register the three `dCollectionCoinDate_c` states | translated | high | `80609b70_FUN_80609b70.c` |
| `0x806501D0` | `fn_806501d0` | constructor for the class registered by `0x80650870` | translated | medium | `806501d0_FUN_806501d0.c` |
| `0x806503D0` | `fn_806503d0` | state/parameter init for the same class | translated | high | `806503d0_FUN_806503d0.c` |
| `0x80650600` | `fn_80650600` | destructor for the same class | translated | high | `80650600_FUN_80650600.c` |
| `0x80650870` | `fn_80650870` | register `daBgmInterlockingDummyBlockMgr_c` class object | translated | high | `80650870_FUN_80650870.c` |
| `0x80650D40` | `fn_80650d40` | set one of five model-part transforms | translated | high | `80650d40_FUN_80650d40.c` |
| `0x80650E60` | `fn_80650e60` | load the five `big_shell` BRRES parts | translated | high | `80650e60_FUN_80650e60.c` |
| `0x806510C0` | `fn_806510c0` | register `daBigShell_c::StateID_Wait` | translated | high | `806510c0_FUN_806510c0.c` |

**Totals:** 30 translated by this pass, 0 partial, 0 stub (24 high confidence,
6 medium: `0x80601380`, `0x80602460`, `0x806026F0`, `0x806027E0`,
`0x80602DB0`, `0x806501D0`). Wave 3 adds 25 more for a cumulative 55; see
[COVERAGE_WAVE3_BASES.md](COVERAGE_WAVE3_BASES.md).

## Clusters

### 1. Registry (`0x806000F0..0x80600180`)

The REL entry calls `0x80160A20` with the function table at `0x807C60F4`, then
the four-byte stub at `0x806001E0`. The only registry state is the head word at
`0x80822900` (BSS). Each node is 12 bytes: `next`, callback, context. Every
class/state registration function in the module ends by calling `0x80600160`,
which makes this three-function cluster the root of the module's boot-time
table construction.

### 2. Character-change select screen (`0x80601380..0x80603060`)

This is the player-select UI used before a stage starts:

- **Arrow sub-state** — `0x80601380` observes the layout animation and advances
  `+0x250`/`+0x254`; `0x80601500` starts the out animation; `0x80601590`
  notifies the parent when the arrow layout is idle.
- **Construction** — `0x80601D50` allocates `0x2F4` bytes; `0x80601D80`
  installs vtables at `+0x60`, layout at `+0x90`, helper objects at `+0x240`
  and `+0x2A4`; `0x80602160` loads the arrow layout and binds widgets.
- **Slot model** — `+0x2D8` is the selected slot (-1 = none); `+0x280` is a
  four-word slot-state array; `+0x284` stores the chosen player indices;
  `+0x28C` stores character ids; `+0x2E0` is a preview id.
- **Choice/confirm** — `0x80602460` picks a free slot by querying the main.dol
  player table at `0x8042A028`; `0x80602600` rejects duplicate characters;
  `0x80602A10` is the confirm path; `0x80603060` is cancel/re-commit.

### 3. State-table example (`0x80609B70`)

`dCollectionCoinDate_c` is the smallest complete state-table constructor
translated here. It shows the family pattern: build the class-name node, then
for each state build a 0x30-byte object from nine data words, install the shared
vtable `0x807D2CD4`, and call [`fn_80600160`] with callback `0x80609E70` and a
per-state key. The three state strings are `StateID_O...`, `StateID_N...`, and
`StateID_E...`. This is the template for the other class-named constructors.

### 4. Stage actor cluster (`0x806501D0..0x806510C0`)

Two adjacent classes share this range:

- `0x806501D0`/`0x806503D0`/`0x80650600` construct, initialise, and destroy the
  object registered by `0x80650870`. The registration string is
  `s_daBgmInterlockingDummyBlockMgr_c_...`; the constructor itself has no
  string, so that class attribution is `// inferred:`.
- `0x80650D40`/`0x80650E60`/`0x806510C0` belong to `daBigShell_c`: the model
  names `g3d/big_shell.brres` and `big_shell_A_temae/oku/futa/filter` are
  explicit, and the state name is `s_daBigShell_c__StateID_Wait`. The actor is
  **larger than `0x3E0` bytes**: `0x80650E60` writes a resource accessor at
  `+0x3D4`, a model holder at `+0x3F0`, and five 0x40-byte part objects at
  `+0x3F4`, `+0x434`, `+0x474`, `+0x4B4`, `+0x4F4`. `0x80650E60` loads the
  BRRES; `0x806510C0` adds the single `Wait` state with callback `0x806511D0`.
  The adjacent `0x80651390` allocates only `0x3E0` bytes and installs a
  different vtable (`0x807E5328`); it is a separate, unnamed class, now
  translated in wave 3.

## Deferred candidates

These are high-value but were left for a later pass. “Deferred” is not
“unknown”: each has an evidence file and a reason.

> **Wave 3 status:** `0x80602C20`, `0x80602EE0`, `0x80602FB0`, and
> `0x80602FF0` were translated in `game::player`; wave 3 adds the remaining
> select tails `0x80603110`, `0x806031D0`, `0x80603260`, `0x806034D0`,
> `0x80603570`, `0x80603610`, `0x80603680`, and `0x80603700`, plus the
> `0x806511D0` / `0x806512F0..0x806515B0` helpers and `0x806517C0`,
> `0x80651830`, `0x80651910`. The rows below are retained as the original
> deferral record; `0x806043C0`, `0x80651990`, and `0x80651A60` remain
> deferred.

| Address | Size | Why deferred |
|---|---:|---|
| `0x806017E0` | 1,012 | `dCharacterChangeSelectArrow_c` state table; mostly 9-word data copies, better generated once the table layout is named |
| `0x80602C20` | 328 | select-screen apply path; needs the `0x80428732` mode field and five transform words fully named first |
| `0x80602EE0` | 180 | select-screen input path; calls main.dol player objects through `0x80377C88` |
| `0x80602FB0`, `0x80602FF0` | 64, 88 | tiny select-screen transitions; batch them with the next select pass |
| `0x80603110`, `0x806031D0` | 144, 96 | final select-screen notifications; depend on `0x80822B18/2C18/2C58` state objects |
| `0x806043C0` | 1,340 | `dCharacterChangeSelectBase_c` state table; same mechanical family |
| `0x806083B0` | 1,012 | `dCollectionCoin_c` state table; 8 states, named strings |
| `0x8060CBE0` | 604 | `dCourseClear_c` state table; 5 states |
| `0x8060F010` | 436 | `dCourseTimeUp_c` state table; named state strings |
| `0x80618D00` | 1,988 | `dFileSelect_c` state table; 44 `StateID` references, largest single cluster |
| `0x806415D0` | 1,576 | `dStaffCreditScore_c` state table; deferred with the menu batch |
| `0x806511D0` | 88 | `daBigShell_c::StateID_Wait` destructor; trivial once handler naming starts |
| `0x806512F0`..`0x806515B0` | 48..184 | `daBigShell` state transitions; several are compiler thunks |
| `0x80651640` | 384 | **rejected**: Ghidra paired-single reconstruction (`ldexpf`, `unaff_GQR0`, unreachable blocks); wait for a paired-single pass |
| `0x80651990`, `0x80651A60` | 184, 452 | `daBigShell` collision/spawn table logic; needs the `0x80828430` table layout |

The 89 class-named files are the largest deferred family. They are mechanical
(read nine data words, build a state object, link a node), but translating them
one by one without a data-table model would mostly reproduce addresses. The
next pass should:
1. generalise the `0x80609B70` pattern into a `StateNode`/`StateTable` view over
   the `0x8082xxxx` BSS objects,
2. decode `0x806083B0` (8 states) as the next sample,
3. generate or template the remaining registrations.

## Next targets

1. **Finish the select screen** — `0x80602C20`, `0x80602EE0`, `0x80602FB0`,
   `0x80602FF0`, `0x80603110`, `0x806031D0`, then `0x806043C0`.
2. **State-table family** — use `0x80609B70` as the template; continue with
   `0x806083B0` (`dCollectionCoin_c`), `0x8060CBE0`, and the 44-reference
   `0x80618D00`.
3. **`daBigShell` behavior** — `0x806511D0..0x806515B0`, then
   `0x806517C0`/`0x80651830`/`0x80651990`/`0x80651A60`.
4. **Remaining stage actors** — the 0x8065..0x807C actor range; use the class
   string at each registration function as the anchor.
5. **Main.dol name resolution** — resolve `0x800C92F0` (NW4R layout animation),
   `0x8015F7C0` (state-name constructor), `0x8015FC90` (interface constructor),
   `0x80064680` (actor resource registration), and `0x800DF180` (BRRES load)
   against `work/ghidra/main-gb/export`; then replace the `// unknown:` comments
   with evidence-backed names.

## Uncertainties

- The `0x80822900` registry is shared by every state-table constructor in the
  module. The callback argument `0xFFFFFFFF` passed during teardown probably
  means “shutdown”, but the callback bodies are outside this slice.
- `0x80602460` and `0x80602DB0` query player objects through the main.dol table
  `0x8042A028` and a virtual factory at `+0x5C`/`+0x28`. The expression is
  explicit; the semantic name of each indirection is inferred.
- `0x80650870` registers a `daBgmInterlockingDummyBlockMgr_c` class object at
  `0x808283B8`. The constructor `0x806501D0` publishes the same area
  (`0x808283E8`) and is treated as that class's constructor, but it has no
  direct class string; the attribution is inferred from adjacency and shared
  globals. The `daBigShell_c` strings are separate (`0x807E5154..`, and
  `0x807E52C4` for `StateID_Wait`). `0x80651390` is *not* the `daBigShell`
  constructor: it allocates only `0x3E0` bytes, while `0x80650E60` writes
  through `+0x4F4`. Its class identity is unknown (wave 3 translates the body
  and records the vtable `0x807E5328` without naming the class).
- The first `FUN_80600160` call in each registration passes a code/data pointer
  (`&DAT_80608980`) as the callback and `0` as context; whether that is a
  default destructor or a name stub is not resolved.
- Ghidra loses the first argument of functions following the `0x802DCD70`
  register-save thunk (`FUN_80602460`, `FUN_80602810`, `FUN_80602860`,
  `FUN_80602DB0`). The Rust calls pass `screen` and document the omission; this
  matches the existing note in `docs/COVERAGE_r80260_802ed.md`.

## Validation

- `node tools/doc-lint.mjs` reports `ok: true`, 0 violations.
- `cargo check` passes for the crate (exit 0); `src/game/bases.rs` has no errors
  or warnings.
- The module also compiled cleanly in isolation with a copy of `platform.rs`, so
  its status is independent of concurrent edits in other lanes.
