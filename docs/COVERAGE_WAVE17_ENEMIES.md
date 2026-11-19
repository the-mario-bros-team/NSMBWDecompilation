# Coverage - wave 17 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the seventeenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   `switch`-model entries that wave 16 deferred, then the clean helpers around
   the `0xEB0`-byte constructor, then the next constructor block.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   entries around the selected ranges, and why each was left out.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  receiver is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.
`main.dol` addresses (`0x8000xxxx`, `0x803xxxxx`, `0x8042xxxx`) are imported
absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 17 | Wave 17 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 362 | 30 | 392 |
| Of those, `translated` | 349 | 28 | 377 |
| Of those, `partial` | 13 | 2 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Constructors | - | 1 | - |
| Model/animation binders | - | 2 | - |
| Contact callbacks | - | 2 | - |
| Per-frame/state handlers | - | 4 | - |
| Registration/destructor plumbing | - | 3 | - |
| `0xEB0` state helpers | - | 11 | - |
| Adjacent helpers | - | 3 | - |
| `0x850` actor helpers | - | 2 | - |
| Callback/effect wrappers | - | 2 | - |
| Selected bytes | - | 4,728 | - |

The 30 new bodies cover 4,728 bytes (0.81 % of the 582,728 bytes that the gap
report listed as untranslated before this pass) and 30 of the 3,223 remaining
function entries (0.93 %). Confidence is **22 high** and **8 medium**. The two
48-byte forwarders are `partial`: their visible body is complete, but the
hidden `r12` operand of `0x802DCBFC` cannot be modelled by `platform::call3`.
There are no `stub` rows.

Cumulative after this pass: 392 bodies, 377 `translated`, 15 `partial`, 3,193
function entries and 578,000 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Finished the `switch`-model block.** Wave 16 named
   `0x80A771D0`/`0x80A77420` as the best remaining targets and left the
   contact callbacks and the mixed handlers untouched. The binder at
   `0x80A771D0` is strong evidence for the class identity: it loads
   `switch`/`switch_koopa` from `0x80B4BEE8`, the `g3d/switch*.brres` paths
   from `0x80B4BF28`, and the `switch_hatena` clips from `0x80B4BF68`. The two
   contact callbacks (`0x80A77680`, `0x80A77760`) are the shorter siblings of
   `fn_80a77540`; the field block `+0x8A0..+0x8A9`, the `+0x38C` state test
   and the `-2.0`/`-3.0`/`2.0` speed writes match it.
3. **Closed the class plumbing.** `0x80A792E0` is the third registration
   callback (byte-identical to `0x80A76090`), and `0x80A79400`/`0x80A79430`
   are the third and fourth `0x802DCBFC` forwarders. Both keep the hidden-`r12`
   limitation documented by wave 16.
4. **Started the `0xEB0` block.** `0x80A79610` and `0x80A79720` initialise the
   eight-byte motion records used by `+0xE9A`/`+0xEAA`; `0x80A79D90`,
   `0x80A79DF0`, `0x80A79FC0` and `0x80A7A030..0x80A7A640` are the animation,
   side and segment helpers. All of them use offsets inside the `0xEB0`-byte
   actor that `0x80A79460` constructs, so the block link is structural rather
   than name-based.
5. **Included three adjacent clean helpers.** `0x80A7AB30` is a two-step
   segment placement over `0x80A7A860`/`0x80A7A8C0`/`0x80A7AA00`; `0x80A7AE80`
   is the `0x1E1` footstep sound; `0x80A7B680` is the vtable-`+0x98` wrapper.
6. **Started the next constructor block.** `0x80A7D330` is a clean 284-byte
   `0x850` actor constructor; its animation binder `0x80A7D6B0`, matrix
   effect `0x80A7D5A0`, callback `0x80A7DB70` and effect `0x80A7E720` follow
   it. The class is only adjacent-block evidence, so no class name is claimed.
7. **Excluded every export with damaged control flow.** The `2^52` GQR
   artefacts in `0x80A79630`, `0x80A79750`, `0x80A79810`, `0x80A7A170`,
   `0x80A7A2C0`, `0x80A7A360`, `0x80A7A3C0`, `0x80A7A460`, `0x80A7C600`,
   `0x80A7D760`, and the `halt_baddata()` export at `0x80A7E630` with its two
   callers `0x80A7ECA0`/`0x80A7EE20` are still called by address where a
   translated body needs them. No GQR body was guessed.
8. **Read the data constants from the REL image.** Address minus base equals
   the file offset for this module, so the `f32`/`u16`/string values were
   checked directly. The Rust keeps the original data addresses instead of
   baking the values in.

### The selected blocks (`0x80A771D0..0x80A7E720`, 30 functions)

The first nine entries are the rest of the `switch`-model class:
`0x80A771D0` rebuilds the model and four animation holders from the variant at
`+0x88C`; `0x80A77420` rebinds the open/closed pair; `0x80A77680` and
`0x80A77760` are the contact callbacks; `0x80A787D0` is the two-mode frame
handler; `0x80A78940` and `0x80A78A70` are the exit/damage steps; `0x80A78A20`
is the release path.

The plumbing group is the third registration callback and the two 48-byte
destructor forwarders.

The `0xEB0` group covers the motion-record initialisers, the two animation
completion steps, the twelve-record list initialiser, the side-byte selector,
the matrix/scale builder, the delta writer, the two segment writers, the
composite update and the tail release loop.

The last two groups start the `0x850` actor block: its constructor, animation
binder, matrix effect, callback and effect spawner.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A771D0` | `fn_80a771d0` | Rebuild the `switch` model and its four animation holders from `+0x88C` | translated | high | `80a771d0_FUN_80a771d0.c` |
| `0x80A77420` | `fn_80a77420` | Rebind the lower/upper clip pair; variant 4 skips the `+0x5BC` pair | translated | high | `80a77420_FUN_80a77420.c` |
| `0x80A77680` | `fn_80a77680` | Contact callback: character-id bookkeeping and `-2.0` speed write | translated | high | `80a77680_FUN_80a77680.c` |
| `0x80A77760` | `fn_80a77760` | Contact callback: per-id `+0x894` slot, `-3.0`/`2.0` speed writes | translated | high | `80a77760_FUN_80a77760.c` |
| `0x80A787D0` | `fn_80a787d0` | Two-mode frame handler with mask/rumble test and `DESC_80B72F68` | translated | medium | `80a787d0_FUN_80a787d0.c` |
| `0x80A78940` | `fn_80a78940` | Exit/damage step: reset, variant-4 clip or sound `0x28C` + burst | translated | high | `80a78940_FUN_80a78940.c` |
| `0x80A78A20` | `fn_80a78a20` | Release `+0x148`, `0x80A778F0` and the `+0x754` animation node | translated | high | `80a78a20_FUN_80a78a20.c` |
| `0x80A78A70` | `fn_80a78a70` | Timer countdown, exit animation, `+0x584` completion and state switch | translated | high | `80a78a70_FUN_80a78a70.c` |
| `0x80A792E0` | `fn_80a792e0` | Third registration callback (same body as `0x80A76090`) | translated | high | `80a792e0_FUN_80a792e0.c` |
| `0x80A79400` | `fn_80a79400` | Third `0x802DCBFC` destructor forwarder | partial | medium | `80a79400_FUN_80a79400.c` |
| `0x80A79430` | `fn_80a79430` | Fourth `0x802DCBFC` destructor forwarder | partial | medium | `80a79430_FUN_80a79430.c` |
| `0x80A79610` | `fn_80a79610` | Initialise one eight-byte motion record to neutral | translated | high | `80a79610_FUN_80a79610.c` |
| `0x80A79720` | `fn_80a79720` | Bind a motion record to an eight-byte table entry | translated | medium | `80a79720_FUN_80a79720.c` |
| `0x80A79D90` | `fn_80a79d90` | Stop both animation slots while `+0xEA7 < 3` | translated | medium | `80a79d90_FUN_80a79d90.c` |
| `0x80A79DF0` | `fn_80a79df0` | Animation completion handler with three mode paths | translated | high | `80a79df0_FUN_80a79df0.c` |
| `0x80A79EC0` | `fn_80a79ec0` | Initialise the twelve embedded list records | translated | medium | `80a79ec0_FUN_80a79ec0.c` |
| `0x80A79FC0` | `fn_80a79fc0` | Choose the `+0x348` side byte from spawn flags | translated | high | `80a79fc0_FUN_80a79fc0.c` |
| `0x80A7A030` | `fn_80a7a030` | Build the `+0x7C` matrix, apply `+0x6A0` offset and scale | translated | high | `80a7a030_FUN_80a7a030.c` |
| `0x80A7A430` | `fn_80a7a430` | Write the `+0x164`/`+0x168` movement deltas | translated | high | `80a7a430_FUN_80a7a430.c` |
| `0x80A7A5A0` | `fn_80a7a5a0` | Write segment offsets from the `+0x688` table | translated | high | `80a7a5a0_FUN_80a7a5a0.c` |
| `0x80A7A600` | `fn_80a7a600` | Composite delta and segment update | translated | high | `80a7a600_FUN_80a7a600.c` |
| `0x80A7A640` | `fn_80a7a640` | Release the two tail records at `+0xD1C` | translated | high | `80a7a640_FUN_80a7a640.c` |
| `0x80A7AB30` | `fn_80a7ab30` | Two-step segment placement over `0x80A7A860`/`A8C0`/`AA00` | translated | medium | `80a7ab30_FUN_80a7ab30.c` |
| `0x80A7AE80` | `fn_80a7ae80` | Play footstep sound `0x1E1` at the actor position | translated | high | `80a7ae80_FUN_80a7ae80.c` |
| `0x80A7B680` | `fn_80a7b680` | Run vtable `+0x98` and return 1 | translated | medium | `80a7b680_FUN_80a7b680.c` |
| `0x80A7D330` | `fn_80a7d330` | Allocate and construct the `0x850`-byte actor | translated | high | `80a7d330_FUN_80a7d330.c` |
| `0x80A7D5A0` | `fn_80a7d5a0` | Spawn effect `0x76` from the `+0x5EC` matrix | translated | high | `80a7d5a0_FUN_80a7d5a0.c` |
| `0x80A7D6B0` | `fn_80a7d6b0` | Bind `throw`, set rate and speeds, enter phase 1 | translated | high | `80a7d6b0_FUN_80a7d6b0.c` |
| `0x80A7DB70` | `fn_80a7db70` | Fourth registration callback (same body as `0x80A76090`) | translated | high | `80a7db70_FUN_80a7db70.c` |
| `0x80A7E720` | `fn_80a7e720` | Spawn effect `0x153` with packed flags and store the handle | translated | high | `80a7e720_FUN_80a7e720.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A78700` | 64 B | Duplicate of `0x80A786A0`; one body is enough |
| `0x80A79630` | 236 B | GQR/`ldexpf` artefacts in the angle conversion |
| `0x80A79750` | 188 B | `2^52` double-conversion artefact in the phase timer |
| `0x80A79810` | 360 B | GQR/`ldexpf` preamble and unreachable blocks |
| `0x80A7A170` | 324 B | GQR register arguments in the segment matrix walk |
| `0x80A7A2C0` | 156 B | GQR/`ldexpf` artefacts |
| `0x80A7A360` | 96 B | GQR/`ldexpf` artefacts |
| `0x80A7A3C0` | 104 B | GQR/`ldexpf` artefacts |
| `0x80A7A460` | 312 B | `2^52` unsigned-to-double idiom; still called by address |
| `0x80A7A700` | 148 B | Not selected this pass; clean neighbour |
| `0x80A7A7A0` | 108 B | Not selected this pass; clean neighbour |
| `0x80A7A810` | 72 B | Not selected this pass; clean neighbour |
| `0x80A7A860` | 88 B | Forward-offset query; kept as `FN_80A7A860` |
| `0x80A7A8C0` | 320 B | Many unreachable conversion blocks; kept by address |
| `0x80A7AA00` | 292 B | Segment fallback; kept by address |
| `0x80A7AB90` | 272 B | Clean mixer, next pass |
| `0x80A7ACA0` | 252 B | Clean mixer, next pass |
| `0x80A7ADA0` | 220 B | Clean mixer, next pass |
| `0x80A7AEF0` | 200 B | Clean helper, next pass |
| `0x80A7B370` | 116 B | Clean helper, next pass |
| `0x80A7B7C0` | 184 B | Clean helper, next pass |
| `0x80A7BB00` | 224 B | Clean helper, next pass |
| `0x80A7BBF0` | 320 B | Clean helper, next pass |
| `0x80A7BD30` | 372 B | Clean helper, next pass |
| `0x80A7BEC0` | 1,464 B | Large mixed handler, next pass |
| `0x80A7C600` | 184 B | GQR/`ldexpf` artefacts in the timer conversion |
| `0x80A7C7D0` | 628 B | Large mixed handler, next pass |
| `0x80A7D760` | 400 B | GQR/`ldexpf` artefacts in the phase handler |
| `0x80A7E630` | 8 B | Export is `halt_baddata()` |
| `0x80A7ECA0` | 84 B | Calls `0x80A7E630`, whose export is `halt_baddata()` |
| `0x80A7EE20` | 128 B | Calls `0x80A7E630`; the tail cannot be verified |

The clean neighbours `0x80A7A700`, `0x80A7A7A0`, `0x80A7A810`, `0x80A7AB90`,
`0x80A7ACA0`, `0x80A7ADA0`, `0x80A7AEF0` and `0x80A7B370` are the best next
targets; together they continue the `0xEB0` helper block.

## Unknowns preserved in code

1. **`switch`-model class and state names.** Its state table is absent from
   the export, so the descriptors keep `DESC_` names and the class label stays
   `// inferred:` from the model/effect strings.
2. **`0x802DCD70`/`0x802DCD68`/`0x802DCDB4`/`0x802DCDBC`.** These are
   prologue/epilogue register-save fragments, not real calls. The Rust takes
   the actor as a parameter or skips the matching epilogue, and each doc says
   so.
3. **`0x800E41C0`.** Seven-argument input/rumble helper in `main.dol`. The
   mask pair at `0x8042A078` is read as two words; the exact meaning of the
   other arguments is `// inferred:`.
4. **`0x80A7A860`/`0x80A7A8C0`/`0x80A7AA00`.** Only the call shapes are used.
   `0x80A7A860` returns a single-precision value that Ghidra prints as
   `double`; the Rust uses `call1_ret_f32`.
5. **`0x80A79610..0x80A7A640` ownership.** The offsets fit the `0xEB0`-byte
   actor built by `0x80A79460`, but the export has no state table for it, so
   the docs say "the `0xEB0` actor" rather than a class name.
6. **`0x80A7D330` class link.** The constructor is complete and high
   confidence, but the class is only adjacent-block evidence; the Rust keeps
   `fn_80a7d330` and does not claim a name.
7. **Hidden `r12` forwarders.** `0x80A79400`/`0x80A79430` are `partial` for
   the same reason as wave 16's `0x80A763F0`/`0x80A76420`.
8. **`0x80A7D330` first vtable.** The constructor installs `0x80302414` early
   and replaces it with `0x80B4CD64` at the end; both stores are kept because
   the intermediate value may be observed by a base constructor.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. `cargo check` reports no warning or error in
`src/game/enemies.rs`. This lane modified only `src/game/enemies.rs`, this
page and `ledger/agents/wave17-enemies.csv`.
