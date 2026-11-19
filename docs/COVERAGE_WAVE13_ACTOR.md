# Coverage — Wave 13 actor band `0x80014000..0x80040000`

## How to read this file

This document describes the Wave 13 translation pass over the main.dol actor
band. A **lane** is a disjoint address range of `sys/main.dol`; this lane owns
every function whose **entry address** is in `0x80014000 <= address <
0x80040000` (image base `0x80004000`).

- **Address** — virtual entry point in the DOL image, not a file offset.
- **Name** — the Rust function name. No original symbol table survives, so the
  `fn_<address>` form is the honest identity; descriptive roles are marked as
  inferred in the Rust doc comments.
- **Role** — one plain sentence. Roles marked *(inferred)* come from call
  sites, field use or the neighbouring constructor/destructor pair, not from a
  recovered symbol.
- **Status** — `translated` = complete control flow, all calls accounted for;
  `partial` = entry/exit and major effects present with a marked gap; `stub` =
  address/role only.
- **Confidence** — strength of **evidence**, not code quality:
  `high` = instruction-level match; `medium` = some field roles or class
  identities inferred; `low` = mostly unresolved.
- **Evidence** — the Ghidra C file under
  `work/ghidra/main-gb/export/`. All bodies live in `src/game/actor.rs`.
- **Observed / inferred / unknown** markers inside the Rust file say where a
  claim comes from: `// observed:` (Ghidra output or image bytes),
  `// inferred:` (naming/role from call sites or strings), `// unknown:`
  (unresolved).

Terms: DOL, MEM1, SDA/r2/r13 and NW4R are defined in `docs/GLOSSARY.md`.

## Scope and counts

| Metric | Count |
|---|---:|
| Indexed entries in `0x80014000..0x80040000` | 553 |
| Eligible entries (size >= 16 bytes) | 470 |
| Already translated in the band before this pass | 41 |
| Translated by this pass | 25 |
| Bytes covered by this pass | 2,144 |
| Partial rows in this pass | 0 |
| Stub rows in this pass | 0 |
| Eligible entries still untranslated in the band | 404 |

The 25 entries were chosen for complete, self-contained evidence: the
release/list framework, two per-frame node ticks, three scene-node list
helpers and two actor hit predicates. All are ≥16 bytes, and every callee is
recorded by address in a documented constant (untranslated leaves are called by
address).

## Function table

| Address | Name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80014200` | `fn_80014200` | Release-if-positive helper: free `this` only when non-null and `release > 0`. | translated | high | `80014200_FUN_80014200.c` |
| `0x80014240` | `fn_80014240` | Release helper with an unconditional `0x8015F800(this,0)` release-helper call before the guarded free. | translated | high | `80014240_FUN_80014240.c` |
| `0x80014340` | `fn_80014340` | Second emitted copy of the release-if-positive helper. | translated | high | `80014340_FUN_80014340.c` |
| `0x80014380` | `fn_80014380` | Register sub-objects selected by a 9-bit mask and OR the mask into the object flags. | translated | medium | `80014380_FUN_80014380.c` |
| `0x80014480` | `fn_80014480` | Tear down the sub-objects selected by the flags, then clear the flag word. | translated | medium | `80014480_FUN_80014480.c` |
| `0x800145F0` | `fn_800145f0` | Third emitted copy of the release-if-positive helper. | translated | high | `800145f0_FUN_800145f0.c` |
| `0x80014640` | `fn_80014640` | Boolean view of the flag byte at `+0x0B`. | translated | high | `80014640_FUN_80014640.c` |
| `0x800146B0` | `fn_800146b0` | Store a three-float position at `+0x0C..+0x14`. | translated | high | `800146b0_FUN_800146b0.c` |
| `0x800146F0` | `fn_800146f0` | Initialise a node record: state `2`, float, `u16` id, cleared byte. | translated | high | `800146f0_FUN_800146f0.c` |
| `0x80014710` | `fn_80014710` | State-scaled float accessor; states 1/2 use `+0x18`, otherwise `1.0`. | translated | high | `80014710_FUN_80014710.c` |
| `0x80014750` | `fn_80014750` | Convert the signed 16-bit angle at `+0x1C` to radians for states 2/3. | translated | high | `80014750_FUN_80014750.c` |
| `0x800147A0` | `fn_800147a0` | Sign-extended angle getter for states 2/3, else `0`. | translated | high | `800147a0_FUN_800147a0.c` |
| `0x800147E0` | `fn_800147e0` | Payload-byte getter for state 3, else `0`. | translated | high | `800147e0_FUN_800147e0.c` |
| `0x80014820` | `fn_80014820` | Append one node to an intrusive doubly-linked list and bump the count. | translated | high | `80014820_FUN_80014820.c` |
| `0x80014860` | `fn_80014860` | Unlink one node, repair the head/tail and decrement the count. | translated | high | `80014860_FUN_80014860.c` |
| `0x800148B0` | `fn_800148b0` | Empty an intrusive list by removing the tail until the list is empty. | translated | high | `800148b0_FUN_800148b0.c` |
| `0x80014910` | `fn_80014910` | Destroy a list: empty it when non-empty, then optionally free it. | translated | high | `80014910_FUN_80014910.c` |
| `0x80014970` | `fn_80014970` | Vtable constructor (`0x802FF690`): base constructor, vtable store, clear `+0x08`. | translated | high | `80014970_FUN_80014970.c` |
| `0x800156B0` | `fn_800156b0` | Per-frame node interpolation: poll global state, derive a half-speed step, write four floats and an active byte. | translated | medium | `800156b0_FUN_800156b0.c` |
| `0x80015C10` | `fn_80015c10` | Coin-pane fade tick: step the scale, clamp the screen point and write the pane transform. | translated | medium | `80015c10_FUN_80015c10.c` |
| `0x800190F0` | `fn_800190f0` | Insert a scene node into the global sorted list when its enable byte is set. | translated | medium | `800190f0_FUN_800190f0.c` |
| `0x80019360` | `fn_80019360` | Mark a scene node registered and store its list-link word. | translated | high | `80019360_FUN_80019360.c` |
| `0x80019380` | `fn_80019380` | Set the two pending-update bytes at `+0x1E6`/`+0x1E7`. | translated | high | `80019380_FUN_80019380.c` |
| `0x8001E0B0` | `fn_8001e0b0` | Hit predicate: kind `3` plus actor virtual slot `+0x158`. | translated | medium | `8001e0b0_FUN_8001e0b0.c` |
| `0x8001E100` | `fn_8001e100` | Second emitted copy of the kind-`3` hit predicate. | translated | medium | `8001e100_FUN_8001e100.c` |

## Clusters

### A. Release helpers (`0x80014200`, `0x80014240`, `0x80014340`, `0x800145F0`)

Four copies of the CodeWarrior "delete this when the signed release flag is
positive" shape. `0x80014240` is the only one with an extra step: it calls the
release-if-positive helper `0x8015F800(this, 0)` for every non-null object,
then frees only when `release > 0`. The helper's own free is guarded by its
second argument, so the zero flag makes that call a no-op in this image.
The other three helpers are byte-identical and differ only in which translation
unit emitted them.

### B. Scene-node registration (`0x80014380`, `0x80014480`)

A constructor/destructor pair over nine optional sub-objects. The mask at
`+0x00` records which sub-objects are live; `0x80014380` initialises the
selected ones and ORs the mask in, while `0x80014480` walks the stored mask in
call order, finalises each sub-object, and clears the word. The sub-object
roles are inferred from the matching initialise/teardown call pairs:
`+0x04` (array 9..0x14), `+0x1B4` (216-byte, 27 x 8 table), `+0x28C`,
`+0x2A8` (6 floats), `+0x2C0` (4 words), `+0x2D0` (pointer) and the three
default bytes at `+0x2D4..+0x2D6`.

### C. Node record accessors (`0x80014640`..`0x800147E0`)

A five-function vocabulary over one record layout:

| Offset | Meaning in this cluster |
|---|---|
| `+0x0A` | state selector (read by `0x800146C0`) |
| `+0x0B` | boolean flag (read by `0x80014640`) |
| `+0x0C..+0x14` | three-float position (`0x800146B0`) |
| `+0x18` | per-state float (`0x800146F0`, `0x80014710`) |
| `+0x1C` | signed 16-bit angle / id (`0x800146F0`, `0x800147A0`) |
| `+0x1E` | payload byte (`0x800146F0`, `0x800147E0`) |

States 1 and 2 select the float; states 2 and 3 select the angle; state 3
selects the payload. The meaning of each state number is not recovered.

### D. Intrusive list (`0x80014820`..`0x80014910`) and adjacent constructor

A complete doubly-linked list implementation with one list header and one node
layout:

- list `+0x00` = count, `+0x04` = head, `+0x08` = tail;
- node `+0x00` = link toward the head, `+0x04` = link toward the tail.

`0x80014820` appends at the tail, `0x80014860` unlinks one node, `0x800148B0`
drains the list and `0x80014910` destroys the list object. The adjacent
`0x80014970` is a vtable constructor (`0x802FF690`), but its layout is not
proven to be the list header: it writes a vtable at `+0x00`, whereas the list
functions treat `+0x00` as the count. The list layout itself is unambiguous
because every link update is explicit.

### E. Per-frame node animation (`0x800156B0`, `0x80015C10`)

`0x80015C10` is the coin-pane tick that `src/ui/hud.rs` already calls by
address (`FN_80015C10`), and the HUD coverage file names it as the next target
for that lane. It requires the pane's `+0xBB` bit 0 to be set, updates the step
and offset fields, clamps the screen point and writes `+0x2C`/`+0x30`/`+0x34`.

`0x800156B0` is the same per-frame family: it polls a global state byte, reads
a global `u16` counter, derives a half-speed step, and always writes four
floats plus the active byte at `+0xA4`. The neighbouring `0x80015C10` is the
`pointGet` coin tick, but the exact owner of this record is not proven.

### F. Scene-node list flags (`0x800190F0`, `0x80019360`, `0x80019380`)

Three leaf helpers over the enable byte `+0x1E5`, the pending bytes
`+0x1E6`/`+0x1E7` and the link word `+0x1E8`. `0x800190F0` tail-calls the global
sorted-list insert `0x80163850` with `this + 4`. The bytes at `+0x1E6`/`+0x1E7`
are set by both `0x80019360` and `0x80019380`; their exact consumer is not yet
traced, and the enable/disable polarity of `+0x1E5` is inferred from this call
site.

### G. Actor hit predicates (`0x8001E0B0`, `0x8001E100`)

Two byte-identical predicates. Each first checks the hit record's kind byte at
`+0x2D == 3`; only then does it dispatch through the actor's vtable slot
`+0x158` with `(this, hit)`. A non-zero result accepts the hit. Which actor
class uses each copy is not proven; the second copy belongs to a separate
translation unit.

## Where Ghidra C and the PowerPC bytes disagree

These were checked against `work/dol/main.elf`; the Rust follows the bytes.

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x800156B0` | computes the interpolation only when the poll byte is zero, and stores `+0xA4 = (poll == 0)` | `beq` skips the computation when the poll byte is zero; the computation block sets the boolean to `1`. The Rust body computes on non-zero and stores `1`. |
| `0x800146B0`, `0x800146F0`, `0x80014710`, `0x80014750` | `double` arguments/return | `stfs`/`lfs` and `fmuls`; the values are single precision. |
| `0x80014750` | `CONCAT44(0x43300000, (int)short ^ 0x80000000) - 2^52` | the standard signed-16-bit to double conversion; the Rust uses `i16 as f32`, which is exact. |
| `0x800148B0` | calls `FUN_80014860(list)` with one argument | `r4` still holds the tail loaded by the loop condition; the Rust passes it explicitly. |
| `0x80014970` | calls `FUN_8016A010()` with no argument | `r3` still holds `this`; the Rust passes it explicitly. |
| `0x800156B0` | call named `thunk_FUN_801dd1d0()` | the call at `0x800156E0` targets the 4-byte thunk `0x800B54B0`, which branches to `0x801DD1D0`; the callee takes no arguments. |

## Uncertainties

- **`0x80014380`/`0x80014480` sub-object roles** are inferred from their
  initialise/teardown pairs. The class name and vtable are not recovered.
- **`0x800156B0`'s global counter** at `0x8042A480` (`r13-0x5200`) is a
  pointer to an object with a `u16` at `+0x06`; when and where that object is
  created is not traced.
- **`0x8001E0B0`/`0x8001E100` kind `3`** is only known to be the accepted hit
  kind; which device or damage type it represents is not proven.
- **Scene-node bytes** `+0x1E5..+0x1E8` are recorded exactly, but their
  consumer (draw/update order or collision filtering) is not yet identified.
- **The `+0x0A` state numbers** in the node-record accessors are only known by
  their selected fields; no state table names them.
- **`0x80014970`** is a vtable constructor whose class is unnamed and has no
  caller in main.dol; it is adjacent to the list code but is not proven to be
  the list object.
- **The `0x80015C10` projection helper** `0x800B33D0` modifies a 2-float point
  in place; whether it is a camera projection or a screen-space adjustment is
  inferred from its use, not from a symbol.

## How these numbers were produced

`work/ghidra/main-gb/export/functions.tsv` was filtered on the hexadecimal
`address` column (`0x80014000 <= address < 0x80040000`) and on `size >= 16`.
The already-translated count (41) is the number of those entries already
present in `ledger/functions.csv` before this pass. The selected 25 entries
sum to 2,144 bytes. No Ghidra run, no re-analysis and no source edits outside
`src/game/actor.rs`: the C files under `work/ghidra/main-gb/export/` are the
only evidence, plus raw `work/dol/main.elf` bytes for the SDA constants and
branch encodings.

The per-function ledger for this pass is `ledger/agents/wave13-actor.csv`.
