# Coverage — engine frame and `dSys` start-up (`0x800E45F0..0x800E4FC8`)

## How to read this file

This is the coverage record for two engine destinations:

| Rust file | Original range | What it owns |
|---|---|---|
| [`src/engine/frame.rs`](../src/engine/frame.rs) | `0x800E4E40..0x800E4FA8` | the body of one game frame |
| [`src/engine/dsys.rs`](../src/engine/dsys.rs) | `0x800E45F0..0x800E4B50` | `dSys_c` start-up and its immediate helpers |

- **Address** is the entry virtual address from
  `work/ghidra/main-gb/export/functions.tsv`; it is not a file offset.
- **Status** follows `docs/RUST_STYLE.md`: `translated` = the full control flow
  and every evidence call are represented; `partial` = control flow is
  represented but a callee's behaviour is opaque; `stub` = address/role only.
- **Confidence** measures evidence strength: `high` = the body is explicit and
  unambiguous; `medium` = a role or argument is inferred; `low` = mostly
  unknown.
- **Evidence** is the per-function Ghidra C file under
  `work/ghidra/main-gb/export/`. Every Rust function has an `# Original` block
  with the same address and file.
- **`// observed:`** marks a fact read from the Ghidra output or the boot-chain
  instruction checks; **`// inferred:`** marks a role guessed from call sites;
  **`// unknown:`** marks an unresolved item.

Terms: `DOL` is the Wii executable container; `r13` is the PowerPC small-data
base register (`0x8042F680`); `SDA` is the small-data area; `MEM1`/`MEM2` are
the Wii's 24 MiB main RAM and the extra 64 MiB. See `docs/GLOSSARY.md`.

## Scope and method

| Item | Count |
|---|---:|
| Functions translated in this pass | 12 |
| Bytes represented | 1,624 |
| `engine::frame` functions | 1 |
| `engine::frame` bytes | 360 |
| `engine::dsys` functions | 11 |
| `engine::dsys` bytes | 1,264 |
| Status `translated` | 11 |
| Status `partial` | 1 (`0x800E48D0`) |
| Callee addresses named but not translated in this lane | 61 |
| Invented symbol names | 0 |

The two primary functions were assigned by the wave brief. The ten `dSys`
helpers are included because they share the `dSys_c` object layout and the frame
calls four of them directly (`0x800E46E0`, `0x800E4660`, `0x800E4680`,
`0x800E4740`, plus the `0x800E4820` thunk). Translating them here avoids
duplicating the same object walk in `frame.rs`.

Every callee outside this cluster is called by absolute address. No callee
behaviour was guessed: where a role is not proven, the Rust doc says
`// inferred:` or `// unknown:` and the coverage row below repeats the limit.

## Translated functions

| Address | Name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x800E4E40` | `fn_800e4e40` | One game frame: `dSys` steps, audio, 3-D setup/draw, game update, input, task list, frame counter. | translated | high | `800e4e40_FUN_800e4e40.c` |
| `0x800E48D0` | `fn_800e48d0` | `dSys_c` start-up: thread, root heaps MEM1/MEM2, sub-heaps, subsystem bring-up. | partial | high | `800e48d0_FUN_800e48d0.c` |
| `0x800E45F0` | `fn_800e45f0` | Allocates the 1 MiB MEM1 and 2.5 MiB MEM2 blocks. | translated | high | `800e45f0_FUN_800e45f0.c` |
| `0x800E4660` | `fn_800e4660` | `dSys` mid-frame step: child callback `+0xC`. | translated | high | `800e4660_FUN_800e4660.c` |
| `0x800E4680` | `fn_800e4680` | `dSys` flag step: set three one-shot bytes, callback `+0x10`. | translated | high | `800e4680_FUN_800e4680.c` |
| `0x800E46E0` | `fn_800e46e0` | `dSys` begin-of-frame step: callback `+8`, vtable `+0x24`. | translated | high | `800e46e0_FUN_800e46e0.c` |
| `0x800E4740` | `fn_800e4740` | `dSys` end-of-frame step: callback `+0x14`, vtable `+0x28`. | translated | high | `800e4740_FUN_800e4740.c` |
| `0x800E47A0` | `fn_800e47a0` | Predicate/flag helper on the `dSys` child (`+9` bit 0). | translated | high | `800e47a0_FUN_800e47a0.c` |
| `0x800E4820` | `fn_800e4820` | 4-byte thunk to the controller tick `0x8006A320`. | translated | high | `800e4820_thunk_FUN_8006a320.c` |
| `0x800E4830` | `fn_800e4830` | Byte setter on the `dSys` child (`+8`); called with `1`. | translated | high | `800e4830_FUN_800e4830.c` |
| `0x800E4850` | `fn_800e4850` | Word setter on the `dSys` child (`+0x14`); called with `0xFF`. | translated | high | `800e4850_FUN_800e4850.c` |
| `0x800E48A0` | `fn_800e48a0` | Word getter from the `dSys` child (`+0x14`), `0xFF` when null. | translated | high | `800e48a0_FUN_800e48a0.c` |

Sizes: 360 (frame) + 640 (`dSys`) + 108 + 24 + 84 + 72 + 72 + 128 + 4 + 24 +
68 + 40 = **1,624 bytes**.

## Frame order (`0x800E4E40`)

The full table with 29 steps is in the [`frame.rs` module
header](../src/engine/frame.rs). The grouping is:

```text
read flag + display mode
  dSys begin
  audio tick
  3-D view setup
  audio tick
  dSys mid
  [mode != 2 && flag == 0] 0x800B64E0(slot) + 0x800B0B20
  display mode-2 reset
  layout draw
  scene draw
  display post-draw
  dSys flags
  [mode != 2] 0x80163170 + 0x80164E70
  main-object update
  re-read flag; rising edge -> 0x801BE2F0(1)
  display state machine
  layout update
  re-read display mode
  four-slot update
  engine::globals per-slot callback
  four-entry object loop
  controller snapshot
  [mode != 2 && flag == 0] task list; on 2 -> transition + scene counter
  dSys thunk 0x800E4820
  renderer end step
  dSys end
  frame counter += 1
```

Two facts that are easy to miss:

1. **The mode is read twice.** The first comparison (`0x800E4E54`) is used for
   the two early conditional blocks; the second (`0x800E4F44`) is used for the
   task-list block. `0x80109660` runs in between and can move the display state
   from `1` to `3`, so the two values are not the same read.
2. **The flag is read twice.** The first byte at `main+4` and the second byte
   after `0x801076B0` form an edge detector; `0x801BE2F0(1)` runs only on
   `0 -> non-0`.

Global addresses used by the frame (all resolved from `r13 = 0x8042F680`):

| Global | Address | Role |
|---|---|---|
| `G_MAIN_OBJECT` | `0x8042A298` | main game object pointer (`+4` flag) |
| `G_DISPLAY_MANAGER` | `0x8042A2B4` | display manager (`+0x1D0` mode) |
| `G_LAYOUT_OBJECT` | `0x8042A2AC` | 2-D layout object |
| `G_FRAME_COUNTER` | `0x8042A368` | incremented last |
| `G_SCENE_COUNTER` | `0x8042A36C` | incremented when the task list returns `2` |
| `G_TASK_LIST` | `0x8042A0A0` | `{table, count, index}` header |
| `G_5728_SLOT` | `0x80429F58` | pointer slot for `0x800B64E0` |
| `G_5730_ARRAY` | `0x80429F50` | four pointers for `0x800B5AB0` |
| `G_5240_OBJECT` | `0x8042A440` | object for `0x800B0B00` |

## `dSys` heap model (`0x800E48D0`)

The class name is proven by the two literal strings passed to the root-heap
constructor:

| String | Address passed | Heap |
|---|---|---|
| `dSys_c::RootHeapMEM1` | `0x8031B050` | MEM1 root heap |
| `dSys_c::RootHeapMEM2` | `0x8031B068` | MEM2 root heap |

`// observed:` both calls are `0x8016E500(0xFFFFFFFF, dSys+0x18/+0x1C, name,
0x20, 4)` and their `r3` results are stored at `0x8042A090` / `0x8042A094`.
`// inferred:` the two object fields are the heap sizes; the decompiled callee
drops its first two arguments, so the exact parameter split is an open item.

The three-word heap-handle array is at `0x80377C48`:

| Slot | Address | Created by | Size |
|---|---|---|---|
| 0 (default) | `0x80377C48` | mirror of the selected slot, `0x8016E9F0` | — |
| 1 | `0x80377C4C` | `0x8016EAA0` (`0x8016E9F0(1, ...)`) | `0x980000` from MEM1 |
| 2 | `0x80377C50` | `0x8016EAB0` (`0x8016E9F0(2, ...)`) | `0x700000` from MEM2 |

Additional fixed blocks, all created in `0x800E48D0` unless noted:

| Size | Source heap | Stored at | Creator |
|---:|---|---|---|
| `0x500000` (5 MiB) | MEM1 | `0x8042A454` | `0x8016EB20` |
| `0x1200000` (18 MiB) | MEM2 | `0x8042A44C` | `0x8016EAC0` |
| `0x1000` (4 KiB) | MEM2 | `0x8042A450` | `0x8016EAF0` |
| `0x400000` (4 MiB, lazy) | MEM2 | `0x8042A098` | `0x8016E630`, only if null |
| `0x100000` (1 MiB) | MEM1 | `0x8042A080` | `0x800E45F0` / `0x8016E630` |
| `0x280000` (2.5 MiB) | MEM2 | `0x8042A084` | `0x800E45F0` / `0x8016E630` |
| `0x1C2000` pool (element 4) | MEM2 | `0x8042A22C` via `0x801021D0` | `0x800E48D0` |

The allocator switch is observable: `0x802B9170(value)` **returns the previous**
`0x8042AE10` and writes the new value. `0x800E48D0` installs the default heap
before the 0x38-byte object and the 0x2A0000 subsystem, then restores the saved
value. Because the first install already pointed at the default heap, the final
global remains the default heap, which is what the later engine allocations
expect.

## `dSys` bring-up order

| # | Address | Call | Inferred role |
|---:|---|---|---|
| 1 | `0x801B4B30` | `OSGetCurrentThread` | remember the game thread |
| 2 | `0x80371AA0 + 0x38` | vtable slot | `dSys_c` initialiser |
| 3 | `0x8016E500` ×2 | root heap constructor | MEM1 / MEM2 root heaps |
| 4 | `0x800E8AB0` | display metrics | screen size/aspect |
| 5 | `0x8016EB20` | create block | 5 MiB MEM1 |
| 6 | `0x8016EAA0` | `0x8016E9F0(1, ...)` | 9.5 MiB MEM1 sub-heap |
| 7 | `0x8016EAB0` | `0x8016E9F0(2, ...)` | 7 MiB MEM2 sub-heap |
| 8 | `0x8016EAC0` | create block | 18 MiB MEM2 |
| 9 | `0x8016EAF0` | create block | 4 KiB MEM2 |
| 10 | `0x802B9170` / `0x8016A700` | allocator swap + 0x38 object | effect/particle root |
| 11 | `0x802D3490(0x2A0000, mem2)` | 0x80 × 0x3C linked records | effect/particle system |
| 12 | `0x8016F1F0` | renderer pointer + view update | renderer |
| 13 | `0x800B58E0` / `0x800DC430` | 0x14-byte objects | resource services |
| 14 | `0x801EE150` ×4 | clear `0x803A201E + i*0x578` | four player/unit slots |
| 15 | `0x8016F220` | four view objects | camera/view setup |
| 16 | view vtable `+8` | two float arguments | view setter |
| 17 | `0x8016F410` | renderer virtual `+0xC` | renderer step |
| 18 | `0x800B6460(heap, mem1, mem2)` | 1 MiB + 3 MiB + 4-byte allocations | resource/scene |
| 19 | `0x800B0B50` / `0x800B0DB0(0)` | 4 resources + 1 object | resource bring-up |
| 20 | `0x8016E630` (lazy) / `0x80106DF0` | 4 MiB block → 0x24 object | layout data |
| 21 | `0x801073B0` | 0x1C4-byte main object + title string | main game object |
| 22 | `0x8015FE40` | clear counters | frame/scene clock |
| 23 | `0x8008F110(prio-1, 4K, 18M)` | three calls | worker service |
| 24 | `0x800E0180` | 0x3FC0 allocation | unknown |
| 25 | `0x800CFAB0` | 0x80 allocation + thread create/resume | engine worker thread |
| 26 | `0x80107EC0` / `0x801092F0` | 0x40 / 0x1F8 objects | layout / display manager |
| 27 | `0x80069720(mem2)` | 0xD01000 block + callbacks | effect/particle archive |
| 28 | `0x800E45F0` | 1 MiB / 2.5 MiB | see helper |
| 29 | `0x801021D0(mem2, 0x1C2000, 4)` | lazy pool | pool allocator |
| 30 | `0x800E1DB0` | reset scene transition | scene history |
| 31 | `0x800E4830(1)` / `0x800E4850(0xFF)` | child setters | engine mode / byte |
| 32 | `0x80006790` | empty body | no-op in evidence |

## `dSys_c` object layout used here

| Offset | Width | Use | Evidence |
|---|---:|---|---|
| `+0x00` | 4 | vtable pointer | standard NW4R object layout; slot `+0x38` called at start |
| `+0x18` | 4 | second argument of `RootHeapMEM1` call | `800e48d0` line 18 |
| `+0x1C` | 4 | second argument of `RootHeapMEM2` call | `800e48d0` line 21 |
| `+0x44` | 4 | display info pointer | `800e8ab0` line 13 |
| `+0x4C` | 4 | child manager | step helpers and `800e47a0/4830/4850/48a0` |
| `+0x08` | 1 | byte setter | `800e4830` |
| `+0x09` | 1 | bit-0 flag | `800e4680`, `800e47a0` |
| `+0x14` | 4 | word getter/setter | `800e4850`, `800e48a0` |

## Unknowns and partial markers

1. **`0x800E48D0` is `partial`.** Its call sequence is complete, but every
   callee above is opaque; the Rust calls them by address and records their
   arguments. No callee body was reconstructed.
2. **Root-heap argument split.** Ghidra prints `0x8016E500(void)` but callers
   consume `r3`; the body drops its first two arguments and uses the fourth
   (`0x20`) as an internal allocation size. The Rust passes the call-site
   arguments exactly as shown and marks `+0x18/+0x1C` as inferred sizes.
3. **`0x20`, `4`, `0xFFFFFFFF`.** Alignment, flag, or sentinel — not
   established. They are named constants with `// unknown:` docs.
4. **`dSys` child type.** `+0x4C` points at an object whose `+4` word is a
   callback table (`+8`, `+0xC`, `+0x10`, `+0x14`) and whose `+8`, `+9`, `+0x14`
   are fields. The object's class is not identified.
5. **View call ABI.** Ghidra renders the two arguments as `(double)` casts of
   `float` loads. The Rust uses an `f64` signature to match the evidence; if the
   real callee takes `f32`, the register assignment is the same but the value
   interpretation differs. Marked `// inferred:` in `dsys.rs`.
6. **Float constants.** `0x8042CCD8` / `0x8042CCDC` are not present in the
   Ghidra C export. They are read from memory at run time; no value is invented.
7. **Default-heap selector.** `0x80429490` is inferred to hold the index
   (`1` = MEM1, `2` = MEM2) copied into `0x80377C48` by `0x8016E9F0`.
8. **Worker-thread role.** `0x800CFAB0` allocates 0x80 bytes, calls
   `0x800CEC10(buf, priority - 1, 0)` and `0x801B5860(thread+8)`. The thread
   *is* created and resumed (observed); what it runs is unknown.
9. **`0x8008F110` argument forwarding.** Its decompiled body shows three
   argument-less calls, but the `0x800E48D0` call site passes three arguments.
   The Rust preserves the outer arguments and reproduces the three inner calls
   exactly; marked `// partial:` in the constant's doc.
10. **State flag at `0x8042A860`.** `0x801BE2F0(1)` stores `1` (or `0` when
    `0x801DD3F0()` returns 0) and returns the old value. The flag's meaning is
    unknown.
11. **Task-list semantics.** `0x8015F620` is translated elsewhere as
    `run_task_list`; its `2` return means "exhausted". The frame treats that as
    the scene-transition trigger. The task functions themselves are unknown.
12. **Three one-shot bytes.** `0x8042A09C..0x8042A09E` are set to `1` if zero
    by `0x800E4680`; what they gate is unknown.
13. **Display mode `2`.** The frame has three `mode != 2` gates. What mode `2`
    means (and why those blocks are skipped in it) is not established.

## Lane boundaries

* This lane owns only `src/engine/frame.rs` and `src/engine/dsys.rs`.
* `src/engine/startup.rs` already translates the callers (`game_main`,
  `game_startup`, `frame_loop`) and is not modified.
* `src/engine/globals.rs` already translates `0x80102140` and `0x801021D0`;
  this lane calls their addresses and documents the link instead of importing
  them, so the frame stays a pure sequencer.
* `src/subsystems/r80140_80180.rs` already translates `0x8015F620`; same
  treatment.
* OS/runtime, NW4R, GX/AX and every `0x8010xxxx`/`0x8016xxxx` callee remain
  other lanes' scope.

## Ledger

`ledger/agents/engine-frame.csv` has one row per function above, with
`module = engine::frame` or `engine::dsys`, the seven-column header from
`docs/RUST_STYLE.md`, and the same status/confidence values as this table.
