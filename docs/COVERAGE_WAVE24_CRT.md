# Coverage — Wave 24 CRT continuation (`0x802D0000..0x802E0000`)

## How to read this file

This is the completion record for wave 24 of the Metrowerks runtime / CRT lane
in [`src/runtime/cpp.rs`](../src/runtime/cpp.rs). It answers five questions for
a reader with no context:

1. **What was added?** Twenty-eight function bodies: a first-fit block
   allocator, four managed-list pop helpers, one texture/pool descriptor
   builder, three constructor/destructor pairs for small vtable objects, the
   remaining C++ object field helpers at the end of `0x802DC`, the unsigned
   64-bit decimal-record converter, the locale `mbtowc` tail-call shim and the
   `printf` conversion-specification parser.
2. **Where is it?** Exclusively in [`src/runtime/cpp.rs`](../src/runtime/cpp.rs).
   No other Rust file, `mod.rs`, `lib.rs`, `Cargo.toml` or `platform.rs` was
   edited.
3. **How sure is it?** `confidence` measures the evidence, not the code style:
   `high` = every branch, store and call operand is explicit in the Ghidra C
   or the raw instructions; `medium` = the flow is complete but one operand or
   argument had to be recovered from a caller or the register trace.
4. **What is still unknown?** See **Uncertainties** and **Deliberately
   skipped**.
5. **What is the entry point?** Read `fn_802d9380` (the allocator) first, then
   `fn_802da310` (a representative constructor) and `fn_802df4b8` (the largest
   body in this pass).

Addresses are plain lowercase hex virtual image addresses (image base
`0x80004000`), not file offsets. Evidence is the per-function Ghidra C file
under `work/ghidra/main-gb/export/`. Every Rust function keeps an `# Original`
block with the address, size and evidence path. The raw `work/dol/main.elf`
was used to recover arguments that Ghidra dropped (`stwu`, virtual calls,
`lfs`/`stfs` field copies) and to confirm the counter formula in
`fn_802dbb70`.

Terms: **DOL** is the Wii executable container; **CRT** is the C/C++ runtime;
**r2** is the second small-data base (`0x80433080`); **vtable** is the array of
function pointers stored at offset 0 of a polymorphic object; a **decimal
record** is the CodeWarrior float-printing structure `{sign: u8, exponent:
i16, length: u8, digits: [u8; 36]}`. See `docs/GLOSSARY.md`.

Status follows `docs/RUST_STYLE.md`: `translated` = the full control flow and
every call are represented; `partial` = control flow is represented but one
operand or callee body is missing; `stub` = address/role only.

## Summary

| Metric | Count |
|---|---:|
| New function bodies in this pass | **28** |
| Bytes represented | **5,020** |
| Status `translated` | 28 |
| Status `partial` | 0 |
| Status `stub` | 0 |
| Confidence `high` | 26 |
| Confidence `medium` | 2 |
| Module | `runtime::cpp` |
| Evidence | `work/ghidra/main-gb/export`, `work/dol/main.elf` |

The 5,020 bytes are the sum of the 28 `functions.tsv` sizes: 336 + 112 + 36 +
252 + 148 + 148 + 148 + 148 + 156 + 140 + 144 + 88 + 104 + 80 + 88 + 280 + 80
+ 88 + 80 + 88 + 72 + 172 + 104 + 92 + 124 + 220 + 24 + 1,468.

## Selection

The lane brief asks for 25–30 untranslated, meaningful functions in the band,
covering allocator, string/memory, integer/float math, C++ runtime, `stdio` and
save/restore stubs, while skipping 4-byte thunks and pure register-save
fragments. Selection was made in this order:

1. List every `functions.tsv` entry in `0x802D0000..0x802E0000`.
2. Remove entries that already have a body in `src/runtime/cpp.rs` (the ledger
   alone is not sufficient: several bodies, including `fn_802df42c`, were
   written before their ledger row).
3. Keep the CRT cluster `0x802DDAxx..0x802DF4B8`. Only four meaningful bodies
   remained there after wave 21, so the selection also takes the unowned
   C++-object cluster `0x802D9380..0x802DBC84` that already hosts the
   `runtime::cpp` bodies `0x802D94D0`, `0x802D94E0`, `0x802DA590` and
   `0x802DA960`.
4. Read each candidate's Ghidra C and raw entry, and keep only functions whose
   body can be represented without inventing a symbol, an argument or a return
   value.

The result is 28 bodies. Twenty-six are `high` confidence and two are
`medium`; the medium cases are listed under **Uncertainties**.

## Translated functions

| Address | Rust name | Role (plain English) | Status | Conf. | Evidence |
|---|---|---|---|---|---|
| `802d9380` | `fn_802d9380` | Takes a 32-byte-aligned block from a free list and splits it when possible. | translated | high | `802d9380_FUN_802d9380.c` |
| `802d9500` | `fn_802d9500` | Releases every index of a manager-owned table. | translated | high | `802d9500_FUN_802d9500.c` |
| `802d9780` | `fn_802d9780` | Counts right shifts of `value - 1` (floor log2). | translated | high | `802d9780_FUN_802d9780.c` |
| `802d97b0` | `fn_802d97b0` | Builds a descriptor with a pitch, dummy texture name and child object. | translated | high | `802d97b0_FUN_802d97b0.c` |
| `802d9b40` | `fn_802d9b40` | Pops and destroys a node from the list at owner `+0x20`. | translated | high | `802d9b40_FUN_802d9b40.c` |
| `802d9cb0` | `fn_802d9cb0` | Same pop helper for owner `+0x18`. | translated | high | `802d9cb0_FUN_802d9cb0.c` |
| `802d9e20` | `fn_802d9e20` | Same pop helper for owner `+0x10`. | translated | high | `802d9e20_FUN_802d9e20.c` |
| `802d9f90` | `fn_802d9f90` | Same pop helper for owner `+0x8`. | translated | high | `802d9f90_FUN_802d9f90.c` |
| `802da310` | `fn_802da310` | Constructs an object, installs vtable `0x80350928` and registers it. | translated | high | `802da310_FUN_802da310.c` |
| `802da470` | `fn_802da470` | Acquires two manager handles into `+0x10`/`+0x14`. | translated | high | `802da470_FUN_802da470.c` |
| `802da500` | `fn_802da500` | Releases the two stored manager handles. | translated | high | `802da500_FUN_802da500.c` |
| `802da5a0` | `fn_802da5a0` | Constructs an object with a helper at `+0xA0`. | translated | high | `802da5a0_FUN_802da5a0.c` |
| `802da600` | `fn_802da600` | Destroys that object and optionally frees it. | translated | high | `802da600_FUN_802da600.c` |
| `802da970` | `fn_802da970` | Constructs a manager node (vtable `0x80350968`). | translated | high | `802da970_FUN_802da970.c` |
| `802da9c0` | `fn_802da9c0` | Destroys a manager node and optionally frees it. | translated | high | `802da9c0_FUN_802da9c0.c` |
| `802daf10` | `fn_802daf10` | Copies mode-selected single-precision fields. | translated | high | `802daf10_FUN_802daf10.c` |
| `802db030` | `fn_802db030` | Constructs a vtable `0x80350998` object. | translated | high | `802db030_FUN_802db030.c` |
| `802db080` | `fn_802db080` | Destroys a vtable `0x80350998` object. | translated | high | `802db080_FUN_802db080.c` |
| `802db0f0` | `fn_802db0f0` | Constructs a vtable `0x803509B8` object. | translated | high | `802db0f0_FUN_802db0f0.c` |
| `802db140` | `fn_802db140` | Destroys a vtable `0x803509B8` object. | translated | high | `802db140_FUN_802db140.c` |
| `802dbb20` | `fn_802dbb20` | Initialises the fields at `+0x8..+0x29` of a small record. | translated | high | `802dbb20_FUN_802dbb20.c` |
| `802dbb70` | `fn_802dbb70` | Constructs a record with a four-element inline array. | translated | high | `802dbb70_FUN_802dbb70.c` |
| `802dbc20` | `fn_802dbc20` | Destroys that record's inline array and optionally frees it. | translated | high | `802dbc20_FUN_802dbc20.c` |
| `802dc090` | `fn_802dc090` | Sets mask `0x1000` and stores four bytes at `+0x44`. | translated | medium | `802dc090_FUN_802dc090.c` |
| `802dc620` | `fn_802dc620` | Sets or clears two flag words from a value and two mode bits. | translated | high | `802dc620_FUN_802dc620.c` |
| `802ddaa0` | `fn_802ddaa0` | Converts an unsigned 64-bit value to a decimal digit record. | translated | high | `802ddaa0_FUN_802ddaa0.c` |
| `802ded7c` | `fn_802ded7c` | Tail-calls the locale multibyte-to-wide converter. | translated | medium | `802ded7c_thunk_FUN_802ded94.c` |
| `802df4b8` | `fn_802df4b8` | Parses one `printf` conversion specification. | translated | high | `802df4b8_FUN_802df4b8.c` |

## What each cluster does

### Block allocator and descriptor builder (`802d9380`, `802d97b0`)

`fn_802d9380` is a first-fit allocator over a doubly linked free list. Blocks
are 0x20-byte headers with `previous`, `next`, `child`, `sibling`, `size` and
an in-use byte. The request is rounded up to 32 bytes. If the chosen block is
smaller than `aligned + 0x40` it is handed out whole; otherwise a remainder
header is created at `block + aligned + 0x20`, given
`size - aligned - 0x20` bytes, and linked into both the free and in-use lists.
The payload returned is `block + 0x20`.

`fn_802d97b0` stores the two dimensions, computes a pitch shift with
`floor(log2(height))`, copies the literal at `0x80350880` (`"dummy_tex"`)
through `0x802E1938` (`strcpy`), allocates `width * 4` bytes of pointer array
with `0x802B9270`, marks `manager + 0xC064`, allocates a 0x158-byte child with
`0x802B9240` and builds it with `fn_802da970`.

### Managed lists (`802d9b40`, `802d9cb0`, `802d9e20`, `802d9f90`)

The four functions are the same algorithm with the owner word read from
`+0x20`, `+0x18`, `+0x10` and `+0x8`. Each takes the first node from the list
at `owner + 4`, calls the owner's virtual slot `+0x8`, then unlinks the node
from `owner + 4` (`0x80228F80`) and `owner + 0x10` (`0x80228DF0`).

### Object lifetime (`802da310..802dbc20`)

These are CodeWarrior constructor/destructor pairs. Each constructor calls a
base constructor with `this` in `r3`, writes a vtable pointer at a fixed
offset, and then calls a virtual slot:

| Object | Base constructor | Vtable | Slot |
|---|---|---|---|
| `802da310` | `0x802BA2D0` | `0x80350928` | `+0x0C` (optional, two arguments) |
| `802da5a0` | `0x80285810` | `0x80350940` | `+0x24` |
| `802da970` | `0x80287130` | `0x80350968` | `+0x2C` |
| `802db030` | `0x802919A0` | `0x80350998` | `+0x18` |
| `802db0f0` | `0x80292810` | `0x803509B8` | `+0x24` |
| `802dbb70` | none | `0x80350A08` | `+0x0C` |

Their destructors call the matching base destructor with a zero flag and free
the storage only when the signed flag is positive. `fn_802da470` and
`fn_802da500` are the acquire/release pair for two resource-manager handles.

### Decimal records and `printf` (`802ddaa0..802df4b8`)

`fn_802ddaa0` turns a `u64` into the decimal record used by CodeWarrior's
`%f`/`%e` conversion. It stores digits least-significant first (values 0–9,
not ASCII), reverses them, and stores `length - 1` in the exponent field. The
`0x802DE370` rounding pass later adds `'0'` to each digit.

`fn_802df4b8` parses flags, width, precision, length modifiers and the
conversion character, then writes the four-word record consumed by the integer
and floating formatters. Its digit test reads the locale character-class table
at `*(0x80350E68) + 8` with a 32-bit stride; the Ghidra C shows a 16-bit read,
which the raw `lwzx` at `0x802DF654` contradicts. The raw form is used.

`fn_802ded7c` is the locale `mbtowc` tail call: it loads
`*(0x80350E68) + 0x20` and forwards the three register arguments.

## Uncertainties

* `fn_802dc090` has a third argument (`mode`) that Ghidra omits. The caller at
  `0x802D8140` passes it in `r5`, and `0x802DC0A4..0x802DC0B0` forwards that
  register to `fn_802dc5d0` unchanged. Without the caller the argument is
  invisible, hence `medium`.
* `fn_802ded7c` is Ghidra's 24-byte `thunk_FUN_802ded94`; the tail call is
  exact, but the argument names are inherited from `fn_802ded94` because the
  shim itself never touches `r3`/`r4`/`r5`.
* In `fn_802df4b8`, the `'G'`/`'g'` case falls through into `'E'`/`'e'` in
  the original. With no explicit precision the default therefore becomes `6`
  after the initial `1`; this is kept as written rather than "cleaned up".
* The block fields `+0x8` and `+0xC` in `fn_802d9380` are only linked, never
  interpreted here. They are documented as `child`/`sibling` with
  `// inferred:` and remain unnamed.

## Deliberately skipped

These addresses are **not** translated in this pass and have no row in
`ledger/agents/wave24-crt.csv`:

* `802d2f10`, `802d3260`, `802debb4` — 4-byte `b` thunks.
* `802dcd40..802dcdbc` — the CodeWarrior `__savegprlr_N` / `__restgprlr_N`
  family; each entry is a 4-byte alternate entry into one `stw`/`lwz` run.
* `802dcd18`, `802dcd2c`, `802dcdc0` and the other empty `return;` register
  save fragments.
* `802dcbfc` — the adjustor dispatcher whose jump table Ghidra could not
  recover; it is already called by address from several game lanes.
* `802dff64` — the `%a`/`%f` formatter; its C contains the `ldexpf` /
  `unaff_GQR0` paired-single artefacts described in `docs/QUALITY.md` §5.3.
* `802d0030..802d8e70` — EGG render/effect code with GQR `ldexpf` artefacts;
  it belongs to the engine lanes, not to the CRT.
* `802db6b0..802dbd50` — the lyt scratch-transform helper family already
  referenced from `src/ui/lyt.rs`; left to that lane.

`fn_802df42c` (double classification) already had a body in
`src/runtime/cpp.rs` when this pass started, so it was not duplicated and is
not counted as a new body.

## Verification

* `cargo check` finishes successfully; the remaining crate warnings are in
  other lanes' files and none points at `src/runtime/cpp.rs`.
* `node tools/doc-lint.mjs src` → `{"ok":true,"files":68,"violations":0}`.
* `node tools/merge-ledger.mjs` merges the 28 new rows exactly once.
* Footer counts: 28 new bodies, 5,020 bytes, 26 `high`, 2 `medium`.
