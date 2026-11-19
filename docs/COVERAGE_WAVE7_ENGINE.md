# Coverage — wave 7 engine core (`globals`, `frame`, `dSys`)

## How to read this file

This is the completion record for one translation wave of the engine core. It
answers, for a reader with no context:

1. **What was added?** The table lists every function whose body was written in
   this wave, with its virtual address, Rust name, role, status, confidence and
   Ghidra evidence file.
2. **Where is it?** The `module` column is the Rust path with `::` separators:
   `engine::globals` = `src/engine/globals.rs`, `engine::dsys` =
   `src/engine/dsys.rs`, `engine::frame` = `src/engine/frame.rs`.
3. **How sure is it?** `confidence` measures the evidence, not code style:
   `high` = every branch and call argument is explicit in the Ghidra C and was
   spot-checked against the instruction words; `medium` = the control flow is
   complete but a decompiler artefact or an inherited argument had to be
   recovered from the instructions.
4. **What is still unknown?** See **Unknowns** at the end.

Addresses in the table are **plain lowercase hex** (no `0x`) and are virtual
image addresses, not file offsets. The evidence column is the file name inside
`work/ghidra/main-gb/export/`. Every Rust function keeps an `# Original` block
with the same address, size and evidence path.

Terms: **DOL** is the Wii executable container; **r13** is the PowerPC
small-data base (`0x8042F680`, set by `0x800041C0`); **SDA** is the small-data
area; **MEM1/MEM2** are the Wii's 24 MiB main RAM and the extra 64 MiB;
**`dSys_c`** is the engine root object. See `docs/GLOSSARY.md`.

## Scope and result

| Item | Count |
|---|---:|
| New function bodies in this wave | **19** |
| Bytes represented | **3,092** |
| `engine::globals` functions / bytes | 9 / 2,216 |
| `engine::dsys` functions / bytes | 7 / 852 |
| `engine::frame` functions / bytes | 3 / 24 |
| Status `translated` | 19 |
| Status `partial` | 0 |
| Confidence `high` / `medium` | 16 / 3 |
| Invented symbol names | 0 |

Running totals for the three destination files after this wave:

| File | Functions | Bytes | Range |
|---|---:|---:|---|
| `src/engine/globals.rs` | 22 | 2,924 | `0x801015A0..0x80102A80` |
| `src/engine/dsys.rs` | 18 | 2,116 | `0x800E45F0..0x800E52D0` |
| `src/engine/frame.rs` | 4 | 384 | `0x800E4E40`, plus the three getters |

The three existing files were checked before selection: `src/engine/frame.rs`
already contained `0x800E4E40`; `dsys.rs` already contained the eleven
`0x800E45F0..0x800E48A0` helpers; `globals.rs` already contained the thirteen
`0x80101950..0x80102A60` helpers. None of the addresses below appeared in
`ledger/functions.csv` or in any other lane's ledger before this pass.

## New bodies

| Address | Name | Module | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `801015a0` | `fn_801015a0` | `engine::globals` | select an 8-byte record from a two-field owner | translated | high | `801015a0_FUN_801015a0.c` |
| `801015e0` | `fn_801015e0` | `engine::globals` | first word of that record, `0xFFFFFFFF` when disabled | translated | high | `801015e0_FUN_801015e0.c` |
| `80101690` | `fn_80101690` | `engine::globals` | store four words in compiler order `[0],[3],[1],[2]` | translated | high | `80101690_FUN_80101690.c` |
| `801016b0` | `fn_801016b0` | `engine::globals` | find the first negative `+0x168` record and fill it | translated | medium | `801016b0_FUN_801016b0.c` |
| `801017d0` | `fn_801017d0` | `engine::globals` | table-driven reset of the `0x80373D60` state block | translated | high | `801017d0_FUN_801017d0.c` |
| `801018c0` | `fn_801018c0` | `engine::globals` | neutral reset of the same state block | translated | high | `801018c0_FUN_801018c0.c` |
| `80102260` | `fn_80102260` | `engine::globals` | three-path mode/course dispatcher | translated | medium | `80102260_FUN_80102260.c` |
| `801027c0` | `fn_801027c0` | `engine::globals` | challenge/flag bits and one-shot change bytes | translated | high | `801027c0_FUN_801027c0.c` |
| `80102a80` | `fn_80102a80` | `engine::globals` | load the 10x4 course record | translated | medium | `80102a80_FUN_80102a80.c` |
| `80107850` | `fn_80107850` | `engine::frame` | main-object flag byte at `+4` | translated | high | `80107850_FUN_80107850.c` |
| `80107f10` | `fn_80107f10` | `engine::frame` | layout-object pointer (`r13 - 0x53D4`) | translated | high | `80107f10_FUN_80107f10.c` |
| `80109340` | `fn_80109340` | `engine::frame` | display-manager pointer (`r13 - 0x53CC`) | translated | high | `80109340_FUN_80109340.c` |
| `800e5060` | `fn_800e5060` | `engine::dsys` | release the 1 MiB/2.5 MiB blocks and two objects | translated | high | `800e5060_FUN_800e5060.c` |
| `800e5110` | `fn_800e5110` | `engine::dsys` | lazy 0xC800 object plus its release slot | translated | high | `800e5110_FUN_800e5110.c` |
| `800e51a0` | `fn_800e51a0` | `engine::dsys` | clamp the OS arena to the MEM1/MEM2 window | translated | high | `800e51a0_FUN_800e51a0.c` |
| `800e5200` | `fn_800e5200` | `engine::dsys` | reset one allocator/heap object to a size | translated | high | `800e5200_FUN_800e5200.c` |
| `800e52d0` | `fn_800e52d0` | `engine::dsys` | reset the three engine heap objects | translated | high | `800e52d0_FUN_800e52d0.c` |
| `800e5400` | `fn_800e5400` | `engine::dsys` | construct the `0x8031B194` object | translated | high | `800e5400_FUN_800e5400.c` |
| `800e5480` | `fn_800e5480` | `engine::dsys` | destroy that object when the free count is positive | translated | high | `800e5480_FUN_800e5480.c` |

## Clusters and why they belong together

### `engine::globals` — record accessors and state resets

The four accessors `0x801015A0..0x801016B0` share one owner layout: a table
index at `+0x1A8`, a row number at `+0x158`, an enable byte at `+0x165`, and
four `0x10`-byte records at `+0x168`. They are the only untranslated entries in
the `0x801015xx` block and are used by the `0x800Fxxxx` course/record code.

The two resets `0x801017D0` and `0x801018C0` write the same `0x80373D60` block
that the display/layout code later reads. `0x801017D0` selects a
two-byte row from `0x802F49D0` using `0x803741BC`, while `0x801018C0` is the
neutral `0`/`0x28` pair. Both publish the block through `0x800B0DB0` and request
scene `6`; both are called by the mode dispatcher below.

`0x80102260` is the dispatcher that ties the group together. Its three paths
are selected by the mode word at `r13 - 0x547C` (`2`/`3`), by bit `0x10` of
`r13 - 0x5700`, and by `0x80315B98 == 4`. The first two paths call the resets
above; the third walks four entries of `0x80354E60`/`0x80354E90`, then issues
the `0x2C7` or `9` scene request. Its Ghidra output contains the Metrowerks
`0x802DCD74` / `0x802DCDC0` save/restore helpers. They are **not calls**; the
Rust body omits them and recovers `param_1`/`param_2` from the saved `r29`/`r28`
registers (the same artefact is documented in `docs/COVERAGE_r80005_80014.md`).

`0x801027C0` is the flag scan behind `0x80102750` (already in this file). It
tests four independent bits (`0x02`, `0x04`, `0x08`, `0x10`), re-scans the
`0x800FD3B0` 9x42 grid, and sets `0x20` only when every check passes. The two
one-shot bytes at `0x8042A19D` / `0x8042A19C` mark a `0 -> 1` transition of the
`0x10` / `0x20` bits.

`0x80102A80` loads the 10x4 course table into the current course object. It is
the largest accessor user: three `0x800CE4xx` getters per cell, `0x800CE540`
twice per row, and the `0x800BBB70` cell writer. The Ghidra output drops the
first argument of the first `0x800CE560` call; the instruction stream shows
`r3` still holds the record, so the Rust passes it explicitly.

### `engine::dsys` — shutdown, arena and heap resets

The seven `0x800E5060..0x800E5480` functions sit immediately after the `dSys`
start-up helpers and are the engine's tear-down/reset half:

* `0x800E5060` releases the two big blocks created by `0x800E45F0` and the two
  `+0x10`-linked objects, then clears both slots.
* `0x800E5110` lazily creates a `0xC800`-byte object from heap slot 2 and calls
  its `+0x2C` release slot when `0x800CDB30` reports it finished.
* `0x800E51A0` is the arena clamp from `docs/ENGINE_STARTUP.md`: it guarantees
  `[0x806E0000, 0x817F0000]` before the root heaps are carved. Both comparisons
  are unsigned (`cmplw`).
* `0x800E5200` is a generic allocator reset: `0x802B9660(object, 0xFF)`, a
  resize through `vtable+0x14`, repeated `+0x28`/`+0x14` element removal, then
  `vtable+0x18` and `0x802B9660(object, 1)`. A zero first result **hangs the
  thread** in the original (`b .` at `0x800E5250`); the Rust reproduces that
  deliberate infinite loop.
* `0x800E52D0` caches the three heap handles and calls `0x800E5200` with
  `0x680000`, `0x500000` and `0xA00000`.
* `0x800E5400` / `0x800E5480` are the constructor/destructor of the object whose
  vtable is `0x8031B194`. The constructor initialises three 0x28-byte entries
  (the Ghidra count `(0xEB - 0x4C) / 0x28` evaluates to `3`) with the float at
  `0x8042CCF0` (image value `0.0f`).

### `engine::frame` — per-frame getters

`src/engine/frame.rs` already had the 360-byte frame body. Its range has no
other Ghidra function entry, so this wave added the only three callees of that
body that had no translation anywhere: `0x80107850` (main flag byte at `+4`),
`0x80109340` (display manager) and `0x80107F10` (layout object). The frame still
calls their original addresses so its sequencing remains a literal copy of
`0x800E4E40`; the three bodies are available here for later modules and for
readers who want the getter semantics.

## Method

1. **Inventory.** Read `work/ghidra/main-gb/export/functions.tsv`, filtered the
   `0x800E45F0..0x800E52D0` and `0x801015A0..0x80102A80` windows, and excluded
   every address already present in `ledger/functions.csv` (including
   `0x801025D0`, `0x80102690` and `0x80102C10`, which belong to
   `game::worldmap`).
2. **Evidence.** Read each `*_FUN_*.c` file and kept its call order, masks and
   store order.
3. **Instruction check.** Dumped the raw big-endian words from
   `work/dol/main.elf` for every function that contained a decompiler artefact.
   This confirmed:
   * `0x802DCD6C` / `0x802DCD74` / `0x802DCDB8` / `0x802DCDC0` are
     save/restore-register helpers, not calls;
   * `0x801016B0` forwards its caller's `r4..r7` to `0x80101690`;
   * `0x801015E0` forwards its own `r3` to `0x801015A0`;
   * `0x80102A80`'s first `0x800CE560` call receives the record pointer in
     `r3`;
   * `0x800E5200`'s zero-result path is a self-branch (`b .`);
   * `0x800E5400`'s count is three 0x28-byte entries and its stores are `stfs`.
4. **Constants.** The float at `0x8042CCF0` was read from the image
   (`0x00000000`). It is still loaded at run time in the Rust.
5. **Verification.** `cargo check` exits 0 and `node tools/doc-lint.mjs src`
   reports 0 violations.

## Unknowns and limits

1. **`0x801016B0`'s forwarded arguments.** The callee writes four words, but the
   only known caller (`0x800F8F70`) passes five arguments to `0x801016B0`. The
   Rust makes that inheritance explicit with a five-argument signature; a
   different caller would need the same treatment. Confidence is medium.
2. **`0x80102260`'s saved-register artefact.** The decompiler prints
   `FUN_802dcd74()` and uses its return as two arguments. The Rust recovers
   `param_1`/`param_2` from the saved `r29`/`r28` registers. The control flow is
   complete, but the semantic role of `param_3` is only "mode value".
3. **`0x80102A80`'s dropped argument.** As above, `0x800CE560` receives the
   record pointer even though Ghidra prints no argument. The signed `char`
   return of `0x800CE4D0` was recovered from `extsb`.
4. **`0x801027C0`'s bits.** The masks and one-shot bytes are exact; what each
   bit gates in gameplay is unknown. The `ok` variable (`bVar2` in the
   decompiler) accumulates the `0x08`, `0x10`, `0x800FD3B0` and `0x23`-bit
   results and only gates the final `0x20` bit; it does not gate entry to the
   `0x10` block, which depends on bit `0x10` alone.
5. **`0x800E5200`'s deliberate hang.** The original has no timeout or recovery
   path when the first resize returns zero. The Rust `loop {}` reproduces that;
   it is not a placeholder.
6. **`0x800E5110`'s release slot.** The virtual call at `+0x2C` is observed;
   the object's class and the slot's name are unknown.
7. **`0x800E5400`'s class.** The vtable `0x8031B194` is exact, but no RTTI string
   identifies the class. The two `+0x40`/`+0x44` floats and the three 0x28-byte
   entries are initialised, not semantically named.
8. **`0x800E51A0` ordering.** It runs inside `start_main_thread` before
   `0x800E48D0`; `docs/ENGINE_STARTUP.md` documents the arena window. The Rust
   preserves the observed store/branch order, including storing the guarantee
   constant before testing the saved bound.

## Lane boundaries

* This wave owns only `src/engine/globals.rs`, `src/engine/frame.rs` and
  `src/engine/dsys.rs`.
* `src/engine/startup.rs`, `src/engine/main_loop.rs` and `src/engine/display.rs`
  were not modified.
* Every callee outside the three files is called by absolute address. In
  particular `0x800E1E60` (`scene_request`, `src/game/scene.rs`), `0x80102690`
  (`wm_effect_request`, `src/game/worldmap.rs`) and `0x801021D0`
  (`engine::globals`) remain other-lane implementations.
* `ledger/agents/wave7-engine.csv` has one row per body above, with the same
  status and confidence. `ledger/functions.csv` is generated by
  `tools/merge-ledger.mjs`; this lane does not rewrite it.
