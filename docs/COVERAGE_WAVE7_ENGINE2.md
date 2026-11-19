# Coverage — wave 7 engine 2 (`engine::globals`)

## How to read this file

This is the completion record for the second engine-core translation pass of
wave 7. It answers, for a reader with no context:

1. **What was added?** The table lists every function whose body was written in
   this pass, with its virtual address, Rust name, role, status, confidence and
   Ghidra evidence file.
2. **Where is it?** Every new body is in `engine::globals`, i.e.
   `src/engine/globals.rs`.
3. **How sure is it?** `confidence` measures the evidence, not code style:
   `high` = every branch and call argument is explicit in the Ghidra C and was
   spot-checked against the instruction words; `medium` = the control flow is
   complete but a decompiler artefact, a dropped argument or an inherited
   register had to be recovered from the instructions.
4. **What is still unknown?** See **Unknowns and limits** at the end.

Addresses in the tables are **plain lowercase hex** (no `0x`) and are virtual
image addresses, not file offsets. The evidence column is the file name inside
`work/ghidra/main-gb/export/`. Every Rust function keeps an `# Original` block
with the same address, size and evidence path.

Terms: **DOL** is the Wii executable container; **r13** is the PowerPC
small-data base (`0x8042F680`, set by `0x800041C0`); **r2** is the second
small-data base (`0x80433080`) used for float constants; **SDA** is the
small-data area. See `docs/GLOSSARY.md`.

## Scope and result

| Item | Count |
|---|---:|
| New function bodies in this wave | **25** |
| Bytes represented | **2,100** |
| `engine::globals` functions / bytes | 25 / 2,100 |
| `engine::dsys` functions / bytes | 0 / 0 |
| `engine::frame` functions / bytes | 0 / 0 |
| Status `translated` | 25 |
| Status `partial` | 0 |
| Confidence `high` / `medium` | 19 / 6 |
| Invented symbol names | 0 |

Running totals for the three destination files after this wave:

| File | Functions | Bytes | Range |
|---|---:|---:|---|
| `src/engine/globals.rs` | 47 | 5,024 | `0x80100270..0x801092F0` |
| `src/engine/dsys.rs` | 18 | 2,116 | `0x800E45F0..0x800E52D0` (unchanged) |
| `src/engine/frame.rs` | 4 | 384 | `0x800E4E40` plus three getters (unchanged) |

`src/engine/frame.rs` and `src/engine/dsys.rs` already contained complete bodies
for their clusters before this pass, so this wave added nothing there. The
selection was made against `ledger/functions.csv` and the three Rust files:
none of the addresses below had a body or a ledger row. Three of them
(`0x801073B0`, `0x80107EC0`, `0x801092F0`) were already *referenced* as raw
addresses from `dsys.rs`, and `0x80100530`/`0x801006E0` from `game::bases`;
those references were call sites, not translations.

## New bodies

| Address | Name | Module | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|---|
| `80100270` | `fn_80100270` | `engine::globals` | write the three-word scene record in `[1],[0],[2]` order | translated | high | `80100270_FUN_80100270.c` |
| `801002c0` | `fn_801002c0` | `engine::globals` | tail-call the course string table at `0x800F53A0` | translated | high | `801002c0_FUN_801002c0.c` |
| `801002f0` | `fn_801002f0` | `engine::globals` | first course string with `'W'` at 0 and `'S'` at 2 | translated | high | `801002f0_FUN_801002f0.c` |
| `80100530` | `fn_80100530` | `engine::globals` | first nonzero indexed lookup over the scene count | translated | high | `80100530_FUN_80100530.c` |
| `801005c0` | `fn_801005c0` | `engine::globals` | indexed list search with a start offset and name compare | translated | medium | `801005c0_FUN_801005c0.c` |
| `801006e0` | `fn_801006e0` | `engine::globals` | tail-call the `0xBF8` scene-record getter, forwarding `r4`/`r5` | translated | medium | `801006e0_FUN_801006e0.c` |
| `80100720` | `fn_80100720` | `engine::globals` | format and append course entries until the append fails | translated | high | `80100720_FUN_80100720.c` |
| `80100830` | `fn_80100830` | `engine::globals` | publish the `0x8031C8B0` render state and one-shot reset | translated | high | `80100830_FUN_80100830.c` |
| `80100900` | `fn_80100900` | `engine::globals` | any list node with both `+0x124` and `+0x18C` set | translated | high | `80100900_FUN_80100900.c` |
| `80100940` | `fn_80100940` | `engine::globals` | count nodes after the head with `+0x124` set | translated | high | `80100940_FUN_80100940.c` |
| `80100f70` | `fn_80100f70` | `engine::globals` | predicate for modes `7` and `8` | translated | high | `80100f70_FUN_80100f70.c` |
| `80100f90` | `fn_80100f90` | `engine::globals` | two-pass per-slot effect/update walk | translated | medium | `80100f90_FUN_80100f90.c` |
| `80101070` | `fn_80101070` | `engine::globals` | wrapper for parameter row `3` | translated | high | `80101070_FUN_80101070.c` |
| `80101080` | `fn_80101080` | `engine::globals` | wrapper for parameter row `2` | translated | high | `80101080_FUN_80101080.c` |
| `80101090` | `fn_80101090` | `engine::globals` | low bit of the `0x80354E60`-selected mask | translated | high | `80101090_FUN_80101090.c` |
| `801017b0` | `fn_801017b0` | `engine::globals` | 16-bit id to word lookup in `0x80320840` | translated | high | `801017b0_FUN_801017b0.c` |
| `801072f0` | `fn_801072f0` | `engine::globals` | clear the boot byte and start `0x801CF580` | translated | high | `801072f0_FUN_801072f0.c` |
| `80107340` | `fn_80107340` | `engine::globals` | wait for the boot byte and return `r13 - 0x53E0` | translated | high | `80107340_FUN_80107340.c` |
| `80107360` | `fn_80107360` | `engine::globals` | reset the main-object header and trailing words | translated | high | `80107360_FUN_80107360.c` |
| `801073b0` | `fn_801073b0` | `engine::globals` | allocate and initialise the 0x1C4-byte main object | translated | high | `801073b0_FUN_801073b0.c` |
| `80107410` | `fn_80107410` | `engine::globals` | initialise the main-object header fields | translated | medium | `80107410_FUN_80107410.c` |
| `80107ec0` | `fn_80107ec0` | `engine::globals` | allocate the 0x40-byte layout object | translated | high | `80107ec0_FUN_80107ec0.c` |
| `80108f10` | `fn_80108f10` | `engine::globals` | create a display record from two header halfwords | translated | medium | `80108f10_FUN_80108f10.c` |
| `80109260` | `fn_80109260` | `engine::globals` | lazily create that record and call `0x80108F40` | translated | medium | `80109260_FUN_80109260.c` |
| `801092f0` | `fn_801092f0` | `engine::globals` | allocate the 0x1F8-byte display manager | translated | high | `801092f0_FUN_801092f0.c` |

## Clusters and why they belong together

### Cluster K — course/scene record lookups (`0x80100270..0x80100720`)

The seven functions share one address calculation:

* the course stride is `0x16518` bytes; the live index is the signed word at
  `scene + 0x338C`;
* the per-course record stride is `0xBF8` bytes; the live record is at
  `scene + index * 0xBF8`;
* string tables sit at `+0x3394` (base) and `+0x33A0` (count), and each string
  entry is `0x1C0` bytes apart with a `0x18`-byte header (this last addition is
  inside `0x800F56B0`, not in this module).

`0x80100270` and `0x801006E0` are the getter pair: the former allocates a
three-word stack record and copies it out in the compiler's `[1],[0],[2]` store
order; the latter is the tail-call wrapper that still forwards `r4`/`r5` to
`0x800F0EC0`. `0x801002C0` is the sibling tail call at the `0x3394` offset.
`0x801002F0` scans the string table for the `"W?S"` marker, `0x80100530` scans
the `0x3390` entry count, and `0x80100720` formats and appends entries until
`0x800F53A0` returns a negative index.

### Cluster L — state queries and small resets (`0x80100830..0x801017B0`)

`0x80100830` is the only float-heavy function in this pass. It publishes three
`/f` constants from the `r2` pool (`0x8042D0A0`, `0x8042D0A4`, `0x8042D0A8`),
a word from `0x8042CF6C`, and then runs a guarded one-shot store of the float at
`0x8042D0AC` into the `0x8031C9B0` block. The Ghidra body also emits three dead
stack spills; they are omitted, because the live stores and the callback push
through `0x802DC780` account for every observed side effect.

`0x80100900` and `0x80100940` walk the same `r13 - 0x54E0` linked list through
its `+0x184` next pointer. The first returns a truth value; the second skips the
head and counts `+0x124` nodes.

`0x80100F70` is a two-value predicate. `0x80100F90` is the largest function of
the pass: it walks the four course slots at `+0x384`, calls `0x800DC990` for
each slot whose value is `3`, then repeats the walk and calls `0x802BC9A0` with
one float, one table entry, one signed halfword and one byte. The float and the
two small integers come from a single row of the `0x8031D39C` table selected by
`n * 8`, not from the slot index. `0x80101070` and `0x80101080` are its two
fixed-row wrappers.

`0x80101090` is a one-bit query over the existing `0x80354E60`/`0x80354E80`
tables, and `0x801017B0` is a 16-bit-indexed word getter from `0x80320840`.

### Cluster M — main/layout/display lifetimes (`0x801072F0..0x801092F0`)

This group is the other half of the boot path that `dsys.rs` documents:

* `0x801072F0` clears the boot byte and starts `0x801CF580`; `0x80107340`
  busy-waits for that byte and returns the word at `r13 - 0x53E0`.
* `0x80107360` resets the main-object header (`+0`, `+4`, `+0x1B0`, `+0x1B4`).
* `0x801073B0` allocates 0x1C4 bytes, resets the object, stores it at
  `r13 - 0x53E8`, and forwards it to `0x80107410` with the literal title
  `"New SUPER MARIO BROS. Wii"` at `0x80321C20`.
* `0x80107410` calls `0x80107920(object + 8)` and clears `+0`, `+4`, `+0x1B8`
  and `+0x1BC`; its `title` and `heap` arguments are part of the call shape but
  are not read by the body.
* `0x80107EC0`, `0x80108F10`, `0x80109260` and `0x801092F0` cover the layout
  object (0x40 bytes at `r13 - 0x53D4`) and the display record/manager
  (`r13 - 0x53D0`, `r13 - 0x53CC`).

## Method

1. **Inventory.** Read `work/ghidra/main-gb/export/functions.tsv` and filtered
   the `0x80100270..0x801092F0` window. Every candidate was checked against
   `ledger/functions.csv` and against `src/engine/globals.rs`,
   `src/engine/frame.rs`, `src/engine/dsys.rs` and the other lanes before it was
   selected.
2. **Evidence.** Read each `*_FUN_*.c` file and kept its call order, masks and
   store order.
3. **Instruction check.** Dumped the raw big-endian words from
   `work/dol/main.elf` for all 25 functions. This confirmed:
   * `0x802DCD70` / `0x802DCD74` / `0x802DCDBC` / `0x802DCDC0` are
     save/restore-register helpers, not calls;
   * `0x801006E0` forwards `r4`/`r5` to `0x800F0EC0` even though Ghidra prints a
     one-argument prototype;
   * `0x801005C0` passes `&handle` to `0x8023B360`/`0x8023B2A0` and returns the
     matched element;
   * `0x80100F90` passes an `f32` in `f1` to `0x802BC9A0`, and the parameter row
     is fixed by `n` rather than by the slot;
   * `0x80107410` consumes only `r3` and `r5`;
   * `0x80108F10` and `0x80109260` keep `r3` across the calls that Ghidra
     prints as `void`.
4. **Constants.** The `r2`-relative addresses were expressed as
   `SDA2_BASE - offset`; the values are still loaded at run time rather than
   baked into the Rust.
5. **Verification.** `cargo check` exits 0 and
   `node tools/doc-lint.mjs src` reports 0 violations.

## Unknowns and limits

1. **`0x801005C0`'s dropped bookkeeping.** Ghidra prints `void`, omits the
   `0x802DCD70`/`0x802DCDBC` save/restore helpers and drops the `&handle`
   argument. The Rust recovers the return value and the handle pointer from the
   instruction stream, but the role of `context[0] + 0xC` is unknown.
2. **`0x801006E0`'s forwarded registers.** The body itself touches only `r3`.
   `r4`/`r5` are forwarded because `0x800F0EC0` (a three-argument function)
   reads them. A caller that does not supply them would see different behaviour.
3. **`0x80100F90`'s `f32` ABI.** Ghidra prints `(double)` for `0x802BC9A0` and
   drops the last two integer arguments; the `lfs f1` and `r4`/`r5` loads show a
   single-precision float followed by an `i16` and a `u8`. The Rust uses a
   dedicated `call_f32_3` helper so the register assignment stays explicit.
4. **`0x80100F90`'s table semantics.** The `0x8031D39C` row layout is exact
   (`+0x26C` float, `+0x270` halfword, `+0x272` byte), but what the four slots
   and the two passes mean in the effect system is unknown.
5. **`0x80107410`'s unused parameters.** The call site passes a title pointer and
   a heap handle; the body reads neither. They are retained in the signature and
   marked unused rather than silently deleted.
6. **`0x80108F10` / `0x80109260` return values.** Both functions are printed as
   `void` by Ghidra, but the callers store `r3`. The Rust returns `u32` and
   forwards the record to `0x80108F40` explicitly.
7. **The boot barrier.** `0x801072F0` and `0x80107340` share the byte at
   `r13 - 0x53E4` and the word at `r13 - 0x53E0`, but which thread sets the
   byte and what the word means are not proven. The wait loop is faithful and
   can block forever, exactly like the original.
8. **`r2` constant names.** The constants are named by address (`SDA2_BASE -
   offset`), not by meaning; some are shared with float-heavy code in
   `engine::display` / `game::worldmap`.

## Lane boundaries

* This wave owns only `src/engine/globals.rs`; `src/engine/frame.rs` and
  `src/engine/dsys.rs` were read but not modified.
* Every callee outside this file is called by absolute address. In particular
  `0x800F0EC0`, `0x800F53A0`, `0x80168400`, `0x801CF580`, `0x802B9240`,
  `0x8023B360`, `0x8023B2A0`, `0x802BC9A0` and `0x802E16E8` remain other-lane
  implementations.
* `ledger/agents/wave7-engine2.csv` has one row per body above, with the same
  status and confidence. `ledger/functions.csv` and `ledger/SUMMARY.md` are
  generated by `tools/merge-ledger.mjs`; this lane does not hand-edit them.
