# Coverage wave 7 — Bluetooth/OS band, sub-band 4 (`runtime::bt`, `0x80230000..0x8025FFFF`)

## How to read this file

This page is the status record for **one wave** of the `main.dol` reconstruction:
wave 7, sub-band 4 of the Bluetooth/OS lane. It answers four questions:

1. **What was in scope?** — the *Scope* section: the address window, the
   eligibility rule and what was already translated.
2. **What did this wave add?** — the *Function table*: one row per translated
   function with address, size, Rust name, plain-English role, status,
   confidence and the Ghidra evidence file.
3. **Why these functions, and what do they belong to?** — the *What this window
   is* and *Clusters* sections. This sub-band contains **no** L2CAP, RFCOMM or
   HCI code; it is the OS/debug console layer plus resource-table accessors that
   shipped in the same address lane.
4. **What is still open and how was it verified?** — the *Unknowns*, *Next
   targets* and *Gates* sections.

Vocabulary follows `docs/RUST_STYLE.md`:

- **Status** — `translated` = full recovered control flow with every call
  accounted for; `partial` = entry/exit and major effects only; `stub` =
  address only. Every row here is `translated`.
- **Confidence** — strength of the *evidence*, not code quality: `high` = the
  stores/reads and the flow are explicit and self-contained; `medium` = flow is
  explicit but a role, a callee or a data structure is not proven.
- **Evidence** — path under `work/ghidra/main-gb/export/`; that C file is the
  source of truth.

All addresses are virtual addresses of the unrelocated `main.dol` (image base
`0x80004000`), not file offsets. The Rust lives in `src/runtime/bt.rs`. The
machine-readable companion is `ledger/agents/wave7-bt4.csv` (same rows, same
order); the 25 rows were also appended to `ledger/functions.csv`.

## Scope

Source index: `work/ghidra/main-gb/export/functions.tsv`, entries whose **entry**
address is in `0x80230000..0x8025FFFF`.

| Item | Count |
|---|---:|
| Function entries in the window | 551 |
| Entries with size `< 16` bytes (address-only veneers, excluded) | 12 |
| Eligible entries (size `>= 16`, all decompiled) | 539 |
| Translated before this wave (`0x80239A10`, `0x8024D390`) | 2 |
| **Selected and translated by this wave** | **25** |
| Status `translated` / `partial` / `stub` | 25 / 0 / 0 |
| Bytes added | 1,612 |
| Remaining untranslated in the window | 512 |
| Coverage of eligible entries | 5.0 % |

Selection rule: complete, self-contained bodies (no dependence on
`unaff_r13`/`unaff_r2` small-data offsets), spread across the window rather than
clustered at one address, with at least one representative of each recurring
shape. No function was selected unless its whole control flow fitted in the
Ghidra export without unresolved jump tables.

| Layer (inferred) | Functions | Bytes | Address span |
|---|---:|---:|---|
| Console/debug state | 7 | 488 | `0x80233420..0x802356F0` |
| Tagged-record writers | 3 | 192 | `0x80239980..0x80239A40` |
| Offset-table / stride accessors | 5 | 284 | `0x8023B360..0x8023CD90` |
| Table flag helpers | 2 | 156 | `0x80243300..0x80247A50` |
| Object flag/field accessors | 7 | 408 | `0x80249C60..0x8025BFE0` |
| Record copy | 1 | 84 | `0x8025F330` |
| **total** | **25** | **1,612** | `0x80233420..0x8025F330` |

Confidence split: 18 `high`, 7 `medium`. No `low`, no `partial`, no `stub`.

## What this window is

`// inferred:` this sub-band is **not** part of the Bluetooth stack. The
nearest assertion strings identify an OS/debug console and an exception
printer:

| String (as exported) | Evidence file | Suggested owner |
|---|---|---|
| `s_db_console_cpp_8034cf40`, `s_illegal_console_handle_8034cf50` | `80233c60_FUN_80233c60.c` | text console |
| `s__________EXCEPTION_OCCURRED______8034d068` | `80234b60_FUN_80234b60.c` | exception handler |
| `s____EXCEPTION_INFO_GPR____8034d0e0` | `80234b60_FUN_80234b60.c` | exception handler |

So the wave translates the layer that the Bluetooth task *logs through*, plus
generic table helpers. That is still useful: it removes the last unknown
callees from the console/error path, and it documents the record/table layouts
that several later lanes will index. It lives in `runtime::bt` because this
module owns the `0x80230000..0x80260000` continuation of the same lane; none of
the 25 functions calls into HCI, L2CAP or RFCOMM.

## Function table

| Address | Size | Rust function | Role (plain English) | Status | Confidence | Evidence |
|---|---:|---|---|---|---|---|
| `0x80233420` | 52 | `fn_80233420` | Fill the 16-byte console geometry block: handle, 16-byte-aligned pitch, width, height, pitch, `pitch*count*2` | translated | high | `80233420_FUN_80233420.c` |
| `0x802340A0` | 124 | `fn_802340a0` | Repaint one console screen record: query target, place the rectangle, paint, flush | translated | medium | `802340a0_FUN_802340a0.c` |
| `0x802345C0` | 108 | `fn_802345c0` | Ring-buffer pending count with IRQ masked, added to the buffer base | translated | high | `802345c0_FUN_802345c0.c` |
| `0x80234860` | 132 | `fn_80234860` | Shut the debug console down: mask IRQ, save context, stop both outputs, free context | translated | medium | `80234860_FUN_80234860.c` |
| `0x802356B0` | 28 | `fn_802356b0` | Install console handler + argument, return the previous handler | translated | high | `802356b0_FUN_802356b0.c` |
| `0x802356D0` | 20 | `fn_802356d0` | Store the second console callback pair | translated | high | `802356d0_FUN_802356d0.c` |
| `0x802356F0` | 24 | `fn_802356f0` | Exchange the `u16` console mode word | translated | high | `802356f0_FUN_802356f0.c` |
| `0x80239980` | 40 | `fn_80239980` | Write a type-`0x61` record: one big-endian `u32` | translated | high | `80239980_FUN_80239980.c` |
| `0x802399B0` | 88 | `fn_802399b0` | Merge masked bits into a type-`0x61` record's big-endian `u32` | translated | high | `802399b0_FUN_802399b0.c` |
| `0x80239A40` | 64 | `fn_80239a40` | Write a type-`0x10` record: big-endian `u16` + big-endian `u32` | translated | high | `80239a40_FUN_80239a40.c` |
| `0x8023B360` | 52 | `fn_8023b360` | Resolve the first sub-table of an offset-table object, read `+4` | translated | medium | `8023b360_FUN_8023b360.c` |
| `0x8023B3A0` | 84 | `fn_8023b3a0` | Resolve record `index` from the second sub-table (offsets are relative) | translated | medium | `8023b3a0_FUN_8023b3a0.c` |
| `0x8023CC60` | 44 | `fn_8023cc60` | Entry `index < 8` of a `0x20`-stride table (`+4` skips the header) | translated | high | `8023cc60_FUN_8023cc60.c` |
| `0x8023CCC0` | 60 | `fn_8023ccc0` | Test bit `index < 8` in the table's flags word | translated | high | `8023ccc0_FUN_8023ccc0.c` |
| `0x8023CD90` | 44 | `fn_8023cd90` | Entry `index < 8` of a `0xC`-stride table (`+4` skips the header) | translated | high | `8023cd90_FUN_8023cd90.c` |
| `0x80243300` | 60 | `fn_80243300` | Fill `count` `u16` slots with `0x8000`, then clear flag mask `4` | translated | medium | `80243300_FUN_80243300.c` |
| `0x80247A50` | 96 | `fn_80247a50` | Run `vtable[0x40](table, index)` over every entry | translated | medium | `80247a50_FUN_80247a50.c` |
| `0x80249C60` | 40 | `fn_80249c60` | Set or clear a bit mask in the `u32` flags word at `+0xC` | translated | high | `80249c60_FUN_80249c60.c` |
| `0x80249C90` | 24 | `fn_80249c90` | Test whether any bit of `mask` is set at `+0xC` | translated | high | `80249c90_FUN_80249c90.c` |
| `0x80253C30` | 56 | `fn_80253c30` | Store four components as `f32`, zero `+0xA8`, mark dirty bit `5` | translated | medium | `80253c30_FUN_80253c30.c` |
| `0x80253E60` | 112 | `fn_80253e60` | Copy the six-word block at `+0xDC` into six optional outputs | translated | high | `80253e60_FUN_80253e60.c` |
| `0x8025BEF0` | 64 | `fn_8025bef0` | Set bits `1`, `2`, `4` of the byte at `+0x120` per mask bits | translated | high | `8025bef0_FUN_8025bef0.c` |
| `0x8025BF30` | 36 | `fn_8025bf30` | End address of a `0x30`-stride block array (pitch rounded to 32) | translated | high | `8025bf30_FUN_8025bf30.c` |
| `0x8025BFE0` | 76 | `fn_8025bfe0` | Index `0..=3` selects one of four `u32` fields at `+0x108` | translated | high | `8025bfe0_FUN_8025bfe0.c` |
| `0x8025F330` | 84 | `fn_8025f330` | Copy a 48-byte record (six 8-byte units) out of an object | translated | high | `8025f330_FUN_8025f330.c` |

## What the pass adds

### 1. The console/error path no longer ends in untranslated code

The shutdown routine `0x80234860` is the composition root of the console
teardown: it reads and rewrites the OS debug flags, saves a handler context
through `0x801B24C0`, masks the debug interrupt, stops two output channels and
only then releases the pending hook at `0x803DA8A0`. Translating it pins down
the *order* of those operations, which is the part a port must reproduce if it
ever wants the debug output teardown to be deterministic.

`0x802345C0` documents the console ring buffer: `+0x06` size, `+0x0C` write
index, `+0x10` read index, `+0x0E` wrap flag, `+0x14` base. The pending count is
computed with interrupts masked, which confirms the structure is shared with an
interrupt handler.

### 2. The record serialisers are now a family

Wave 1 translated `fn_80239a10` (type byte `0x08`). This wave adds its two
direct siblings, `0x80239980`/`0x802399B0` (type `0x61`) and `0x80239A40`
(type `0x10`), so the "type byte first, then big-endian fields" convention is
now supported by four functions instead of one. `0x802399B0` also shows the
merge operation used to update a field in place without rewriting the record.

### 3. Offset tables: absolute vs relative

`0x8023B360` and `0x8023B3A0` read a *byte offset* from the object header and
add it to the object base, then read another offset inside the sub-table and add
that too. `0x8023CC60`/`0x8023CCC0`/`0x8023CD90` instead take a table base
pointer directly and apply a fixed stride (`0x20` and `0xC`) with an eight-entry
bound. Keeping both shapes side by side in the module makes it obvious which
one a caller is using.

## Unknowns

1. **Console record semantics.** `0x802340A0`'s arithmetic (cell origin minus
   6/3, `columns * 6`, `rows * 10`) is exact, but the callees `0x80233290`
   (rectangle), `0x80233EF0` (paint) and `0x80233460` (flush) are named from
   their arguments only; none of them is translated yet.
2. **Flag meanings.** The flags words at `+0x70` (`fn_80253c30`), `+0xC`
   (`fn_80249c60`/`fn_80249c90`), `+0xD8`/`+0x120` (wave 7 neighbours) and the
   table head word (`fn_8023ccc0`) are translated bit-for-bit, but no string in
   this window names a bit.
3. **`0x8000` fill value.** `fn_80243300` writes `0x8000` into every `u16` of
   the table. Whether that is "unset", "identity" or a matrix tag is unknown.
4. **Virtual method `vtable[0x40]`.** `fn_80247a50` is complete, but the method
   it calls is not in this lane; only its `(table, index)` shape is proven.
5. **The four `double` arguments of `fn_80253c30`.** Ghidra presents them as
   `double` and narrows each to `f32` at the store. The Rust mirrors the
   decompiler exactly; whether the real ABI passes `float` in `f1..f4` or
   `double` cannot be decided from the C export alone.
6. **Sub-band ownership.** The console/debug attribution comes from assertion
   strings in two *neighbouring* functions, not from the 25 translated bodies.
   If a later lane proves these belong to a dedicated `os::console` module,
   move them and update the ledger — do not copy them.

## Next targets

Closest remaining bodies in this window, all `>= 16` bytes and free of
small-data (`unaff_r13`/`unaff_r2`) offsets unless noted:

| Address | Size | Why next |
|---|---:|---|
| `0x80233260` | 40 | The console-target query behind `fn_802340a0`; it closes that flow. |
| `0x80233290` | 80 | The rectangle primitive; names the four geometry arguments. |
| `0x80233EF0` | 84 | The record painter called with `(record, 0)`. |
| `0x80233460` | 20 | The flush the repaint path ends with. |
| `0x802348F0` | 132 | Frees/releases the saved console context. |
| `0x8023C280` | 908 | Only caller of `fn_80239a10`; would name the tagged-record owner. |
| `0x80242B50` | 60 | Small neighbour of the table helpers; first step into the `0x8024xxxx` cluster. |
| `0x80249A70` | 48 | Destructor of the `0x8024Axxx` object family; names the vtable base `0x8034E6E8`. |
| `0x8025A780` | 52 | Virtual-call wrapper with a `countLeadingZeros` flag decode (skip until its jump table is recovered). |

Deliberately **not** next: the 12 `< 16`-byte veneers in the window, functions
whose bodies read `unaff_r13`/`unaff_r2` (for example `0x80235800`,
`0x80237BA0`, `0x80249D30`), and anything whose export still contains
`Could not recover jumptable` (for example `0x8025A780`, `0x80257740`).

## Gates

- `node tools/doc-lint.mjs src` → `ok: true`, 0 violations.
- `cargo check` → `Finished dev profile`, exit 0, with no error and no warning
  from the 25 functions added by this wave. Twelve pre-existing warnings remain
  elsewhere (eleven `unnecessary parentheses` from the earlier wave-6 builders
  in `src/runtime/bt.rs` and one function-pointer cast warning in
  `src/runtime/input.rs`); they were left untouched.
- Files edited by this wave: `src/runtime/bt.rs`,
  `ledger/agents/wave7-bt4.csv`, `ledger/functions.csv` (25 appended rows) and
  this document. No staging module, `lib.rs`, `mod.rs`, `Cargo.toml`,
  `platform.rs` or other lane's file was touched.
