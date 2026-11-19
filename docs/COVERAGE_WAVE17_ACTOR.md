# Coverage — Wave 17 actor band `0x80014000..0x80040000`

## How to read this file

This document covers the Wave 17 translation pass over the main.dol **actor
band**. A lane is a disjoint address range of `sys/main.dol`; this lane owns
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
| Translated in the band before this pass | 66 |
| Translated by this pass | 25 |
| Bytes covered by this pass | 3,696 |
| Partial rows in this pass | 0 |
| Stub rows in this pass | 0 |
| Eligible entries still untranslated in the band | 379 |

The 25 entries were chosen for complete, self-contained evidence: the warning
and error scene teardown/state helpers at `0x80017060..0x80017FD0`, the scene
node and pane helpers at `0x80019080..0x80019390`, the classic-controller
warning scene constructor/builder at `0x80019990`/`0x80019C10`, and the
`Wm_en_hit` actor state steps at `0x8001DF60..0x8001E850`. Every entry is at
least 16 bytes and every callee is recorded by address.

## Function table

| Address | Name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80017060` | `fn_80017060` | Release helper: free `this` only when non-null and `release > 0`. | translated | high | `80017060_FUN_80017060.c` |
| `0x800170A0` | `fn_800170a0` | Release helper with a guarded `+0x18` sub-object call. | translated | high | `800170a0_FUN_800170a0.c` |
| `0x80017170` | `fn_80017170` | Error-window release: `+0x1B8` sub-object, pane cleanup at `+0x08`, optional free. | translated | medium | `80017170_FUN_80017170.c` |
| `0x80017BB0` | `fn_80017bb0` | Error-window state step selected by the active player's buttons. | translated | medium | `80017bb0_FUN_80017bb0.c` |
| `0x80017D00` | `fn_80017d00` | Select and restart an error-window animation. | translated | high | `80017d00_FUN_80017d00.c` |
| `0x80017EE0` | `fn_80017ee0` | Ask the pane-group owner for `0x900` bytes, then send the finished event. | translated | medium | `80017ee0_FUN_80017ee0.c` |
| `0x80017F70` | `fn_80017f70` | Start the close animation and arm a 30-frame timer. | translated | high | `80017f70_FUN_80017f70.c` |
| `0x80017FD0` | `fn_80017fd0` | Count down the close timer and send the closed event. | translated | high | `80017fd0_FUN_80017fd0.c` |
| `0x80018BB0` | `fn_80018bb0` | Release helper with an unconditional base release call `0x8015F800(this,0)`. | translated | high | `80018bb0_FUN_80018bb0.c` |
| `0x80018D50` | `fn_80018d50` | Minimal release helper (guard plus optional free). | translated | high | `80018d50_FUN_80018d50.c` |
| `0x80018DD0` | `fn_80018dd0` | Release helper with a guarded `+0x18` sub-object call. | translated | high | `80018dd0_FUN_80018dd0.c` |
| `0x80018EA0` | `fn_80018ea0` | Nunchuk release: `+0x1B4` sub-object, pane cleanup at `+0x04`, optional free. | translated | medium | `80018ea0_FUN_80018ea0.c` |
| `0x80019080` | `fn_80019080` | Detach a registered scene node from its pane group and lists. | translated | medium | `80019080_FUN_80019080.c` |
| `0x80019140` | `fn_80019140` | Register a scene node with the pane manager and send the pane event. | translated | medium | `80019140_FUN_80019140.c` |
| `0x80019200` | `fn_80019200` | Send the nunchuk close event when the pane animation has finished. | translated | high | `80019200_FUN_80019200.c` |
| `0x800192F0` | `fn_800192f0` | Send the nunchuk start event when the pane animation has finished. | translated | high | `800192f0_FUN_800192f0.c` |
| `0x80019390` | `fn_80019390` | Attach an interface object to a scene node when its virtual check accepts. | translated | medium | `80019390_FUN_80019390.c` |
| `0x80019990` | `fn_80019990` | Construct the classic-controller warning scene object (six vtables/pointers). | translated | medium | `80019990_FUN_80019990.c` |
| `0x80019C10` | `fn_80019c10` | Build the classic-controller warning scene: archive, layout, panes, player lookup. | translated | high | `80019c10_FUN_80019c10.c` |
| `0x8001DF60` | `fn_8001df60` | Per-frame `Wm_en_hit` dispatch over the scene-record pair. | translated | medium | `8001df60_FUN_8001df60.c` |
| `0x8001E1C0` | `fn_8001e1c0` | First hit-state entry: scale from the two `+0xAC` values, next state `2`. | translated | high | `8001e1c0_FUN_8001e1c0.c` |
| `0x8001E2E0` | `fn_8001e2e0` | Second hit-state entry: scale from the other height, next state `3`. | translated | high | `8001e2e0_FUN_8001e2e0.c` |
| `0x8001E400` | `fn_8001e400` | Third hit-state entry: same trigger as `0x8001E2E0`, scale mode `0xC00`, next state `4`. | translated | high | `8001e400_FUN_8001e400.c` |
| `0x8001E520` | `fn_8001e520` | Fourth hit-state entry: inverse `+0x348` selector, next state `1`. | translated | high | `8001e520_FUN_8001e520.c` |
| `0x8001E850` | `fn_8001e850` | Forward a value to the actor vtable slot `+0x3F4` with a zero third argument. | translated | medium | `8001e850_FUN_8001e850.c` |

## Clusters

### A. Warning-scene teardown helpers (`0x80017060`, `0x800170A0`,
`0x80017170`, `0x80018BB0`, `0x80018D50`, `0x80018DD0`, `0x80018EA0`)

Seven emitted destructor helpers with the same `(this, release)` shape. The
small ones free `this` only when it is non-null and `release > 0`. The larger
ones first pass a sub-object to `0x8015F9F0` (which, with a zero release
argument, only performs a null check in this image) and clear a pane group
through `0x800C89A0`. `0x80018BB0` additionally calls the base release helper
`0x8015F800(this, 0)` unconditionally for every non-null object.

### B. Error-window state steps (`0x80017BB0..0x80017FD0`)

`0x80017BB0` maps two bits of the active player's controller record to state
`0`/`1`, records the previous state, plays sound `0x78`, and sends one of two
BSS events. When the force byte `+0x22F` is set it instead asks the global
allocation interface for `0x900` bytes and enables vibration for state `3`.
`0x80017D00` selects animation `previous + 7` or `9`; `0x80017F70` starts the
close animation, stops slot `0x12` and arms a 30-frame timer; `0x80017FD0`
consumes that timer once `0x800C9610` reports the animation finished.

### C. Scene-node and pane helpers (`0x80019080..0x80019390`)

`0x80019080` walks the enable byte `+0x1E5`, calls the scene vtable slot
`+0x10`, detaches the pane group at `+0x04`, and calls the pane vtable slot
`+0x10`. `0x80019140` registers the node through `0x800C9A60` using the link
word at `+0x1E8` and sends event `0x80352338`. `0x80019200`/`0x800192F0` send
the close/start events on the same animation-finished test, and `0x80019390`
attaches an interface after its virtual slot `+0x0C` returns zero.

### D. Classic-controller warning scene (`0x80019990`, `0x80019C10`)

A constructor/builder pair for `extensionControllerClasic`. The constructor
installs the base vtable `0x80300B30`, the sub-vtables at `+0x19C..+0x1B0`,
constructs the block at `+0x1B4`, and swaps in the constructed-state vtable
`0x80300B40`. The builder loads
`extensionControllerClasic/extensionControllerClasic.arc` and
`extensionControllerClasic_05.brlyt`, binds the three-name pane tables, looks
up `T_player`, and marks the scene loaded by setting `+0x1E4`.

### E. `Wm_en_hit` actor state steps (`0x8001DF60..0x8001E850`)

A five-entry state family plus the per-frame dispatch. `0x8001DF60` obtains a
two-object pair from `0x802DCD74`, compares the actors' `+0x38C` markers and
`+0x6C` virtual name, and drives slots `+0x164`/`+0x168` when the records
disagree. `0x8001E1C0`/`0x8001E2E0`/`0x8001E400` share one body: they write a
variant flag at `+0x3F0`, build the scale
`[(1.75 + 0.35*|height|) * ±1, y_scale, 0.0]`, call vtable slot `+0x150`,
spawn the `"Wm_en_hit"` effect at `+0x1D8..+0x1E0`, play effect `0x1A4`, and
store the next state at `+0x3F4`. `0x8001E520` is the fourth entry (scale
`1.75*±1`, Y `2.75`, mode `0x600`), and `0x8001E850` forwards a value through
slot `+0x3F4`.

## Where Ghidra C and the PowerPC bytes disagree

| Address | Ghidra C | Instruction stream |
|---|---|---|
| `0x80017BB0` | indexes the controller table with one indirection and adds `+4` to the array address | `lwzx` loads the slot pointer, `lwz 4(slot)` loads the controller record pointer, and `lwz 0x1C(record)` loads the button word. The Rust follows the three loads. |
| `0x80017170`, `0x80018EA0` | prints `param_1 != -0x1A0` / `-0x19C` | `addic.` computes `this + 0x1A0` / `this + 0x19C`; the next `addic.` adds `0x18`, i.e. the guard is on `+0x1B8` / `+0x1B4`. The Rust keeps both checks. |
| `0x80019080` | first virtual call has no printed arguments | `r3` still holds `this` at the `bctrl`; the Rust passes `this`. |
| `0x8001E850` | warns that the jump table at `0x8001E86C` is unrecoverable and treats the tail branch as an indirect call | the stream loads vtable slot `+0x3F4` and calls it with `(actor, value, 0)`. |
| `0x80019990` | uses `param_1[0x67]`-style `int*` indexing | the byte stores are at `+0x1E4` and `+0x1E8`; the pointer stores are at `+0x19C..+0x1B0`. |
| `0x80019C10` | calls `thunk_FUN_80007320` | the direct call target is `0x800C8A90`, a thunk to `0x80007320`. The Rust calls the thunk address and documents it. |

## Uncertainties

- **`0x8015F9F0` with a zero release argument** is a no-op release in this
  image; the warning scenes still call it. Whether a future image uses a
  non-zero release is not known.
- **Controller bits 3 and 2** in `fn_80017bb0` are only known to select state
  `1` and state `0`; the physical buttons are not named.
- **Event objects** `0x803520F8`, `0x80352178`, `0x803521F8`, `0x80352238`,
  `0x803522F8`, `0x80352338`, `0x80352378` and `0x803523F8` are empty BSS
  objects passed to vtable slot `+0x18`; their types are not recovered.
- **The classic-controller scene constants** `0x68`, `0x97`, and the pane
  tables `0x80300A74`/`0x80300A8C`/`0x80427790`/`0x80427798` are recorded but
  the layout roles are inferred from call order.
- **`fn_8001df60`'s record pair** comes from `0x802DCD74`; the `+0x34` flag
  bit `0x00080000`, the marker values `1`/`2`, and the `+0x120` bit `0x80`
  are recorded exactly but not named.
- **`fn_8001e850`'s slot `+0x3F4`** dispatches through an unrecovered jump
  table; the callee identity is unknown.
- **The `Wm_en_hit` scale formula** is confirmed instruction-by-instruction
  (`fabs`, `frsp`, `lfsx` from the ±1 table, `fmuls`/`fadds`); the physical
  meaning of the variant flag `+0x3F0` is not proven.
- **`fn_8001e520`'s `+0x348` byte** toggles bit 0 before indexing the ±1
  table; other bits are preserved by `xori`, so a byte value above `1` would
  index outside the two-entry table exactly as the original does.

## How these numbers were produced

`work/ghidra/main-gb/export/functions.tsv` was filtered on the hexadecimal
`address` column (`0x80014000 <= address < 0x80040000`) and on `size >= 16`.
The pre-pass translated count (66) is the number of those entries already in
`ledger/functions.csv`. The selected 25 entries sum to 3,696 bytes. Call
targets and branch polarities were checked against the raw words in
`work/dol/main.elf` where the exported C was ambiguous. No Ghidra run and no
source edits outside `src/game/actor.rs` were made.

The per-function ledger for this pass is `ledger/agents/wave17-actor.csv`.
