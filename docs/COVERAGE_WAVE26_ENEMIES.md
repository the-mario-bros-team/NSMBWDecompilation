# Coverage - wave 26 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-sixth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the runs were found:
   the next contiguous cluster after the wave-25 block, and the clean helpers
   of the `kurage` actor that follows it.
3. **What is still missing?** - *Deferred neighbours* names the exports that
   were skipped inside those runs and the next cluster after them.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or an ABI detail is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 26 | Wave 26 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 596 | 30 | 626 |
| Of those, `translated` | 577 | 30 | 607 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Selected bytes | - | 4,488 | - |

The 30 new bodies cover 4,488 bytes (0.84 % of the 536,284 bytes that were
still untranslated before this pass; 4,488 / 536,284 = 0.84 %) and 30 of the
2,959 remaining function entries (1.01 %). Confidence is **27 high** and
**3 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 626 bodies, 607 `translated`, 15 `partial`, 2,929
function entries and 531,880 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Continued after the wave-25 edge.** Wave 25 ends at `0x80A89460`; the
   next export in `functions.tsv` is `0x80A8A5C0`, after a 0x1134-byte region
   that contains no function entries at all. The run
   `0x80A8A5C0..0x80A8BA50` is contiguous except for the exports deferred
   below.
3. **Identified the class from its resources.** `0x80A8AE30` loads
   `g3d/block_tsuta.brres` (`0x80B4EED4`), the model `block_tsuta`
   (`0x80B4EEEC`) and the clips `up_anim` (`0x80B4EEF8`) and `wait`
   (`0x80B4EF00`). The class is therefore called the `block_tsuta` actor;
   `daEnBlockTsuta_c` is `// inferred:` from those names, not a recovered
   symbol. The next block loads `g3d/kurage.brres`, so its helpers are grouped
   as the `kurage` (jellyfish) actor.
4. **Cross-checked duplicated bodies against already-translated code.** The
   REL stores several of these helpers twice. `0x80A8A770` is byte-for-byte
   [`fn_80a41f80`] (five `+0x30` owner links plus one recursion),
   `0x80A8A850` is [`fn_80a42060`] without the null guard, and
   `0x80A8A5C0`/`0x80A8A620`/`0x80A8B8D0` have the same three-way body as
   [`fn_80a41dc0`] and [`fn_80ab4fc0`]. The comparison pinned down the
   receiver of the virtual `+0xC` query and the `0x8015F800`/`0x802B9280`
   callback pair.
5. **Took one lifecycle, then its clean neighbours.** For `block_tsuta` the
   run covers the constructor (`0x80A8A970`), the variant initialiser
   (`0x80A8AA90`), the model binder (`0x80A8AE30`), the matrix rebuild
   (`0x80A8ACF0`), the floor probe and its two callers
   (`0x80A8B130`/`0x80A8B230`), the rate step (`0x80A8B300`), the effect and
   timer helper (`0x80A8B370`), three frame handlers
   (`0x80A8B3F0`/`0x80A8B4D0`/`0x80A8B560`) and the destructor
   (`0x80A8B5B0`). For `kurage` it takes the placement, blend, fade and rate
   helpers (`0x80A8BFA0` then `0x80A8C030..0x80A8C3B0`).
6. **Read every numeric constant from the REL image.** The `block_tsuta`
   constants are data-section floats: `0x80B2E744 = 16.0`,
   `0x80B2E748 = 1.0`, `0x80B2E74C = 0.0`, `0x80B2E750 = -8.0`,
   `0x80B2E754 = 8.0`, `0x80B2E758 = -12.0`, `0x80B2E75C = 12.0`,
   `0x80B2E760 = 0.1`, `0x80B2E768 = 0.0`, `0x80B2E780 = 1.0`,
   `0x80B2E790 = 0.01` and `0x80B2E794 = -0.01`. `0x80B2E788` is an 8-byte
   `double` holding `0.01`, which is why `0x80A8C310` reads it with
   [`read_f64`] while the clamps use the single-precision copies.
7. **Excluded every export with damaged control flow.** The damaged and
   non-exported addresses inside the run and the `kurage` constructor were
   deferred instead of guessed; see *Deferred neighbours*.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A86600` | `fn_80a86600` | Item clip helper: `out2` (`index 1`) through `0x80A86540` | translated | high | `80a86600_FUN_80a86600.c` |
| `0x80A86B00` | `fn_80a86b00` | Item clip helper: `out3` (`index 2`), raises `+0xE02`, clears `+0xD78` | translated | high | `80a86b00_FUN_80a86b00.c` |
| `0x80A8A5C0` | `fn_80a8a5c0` | `block_tsuta` state callback (`0x8015F800` + optional `0x802B9280`) | translated | high | `80a8a5c0_FUN_80a8a5c0.c` |
| `0x80A8A620` | `fn_80a8a620` | Second copy of the same state callback | translated | high | `80a8a620_FUN_80a8a620.c` |
| `0x80A8A680` | `fn_80a8a680` | Constant getter, returns `0x80358104` | translated | high | `80a8a680_FUN_80a8a680.c` |
| `0x80A8A770` | `fn_80a8a770` | Five `+0x30` owner links plus one recursion | translated | high | `80a8a770_FUN_80a8a770.c` |
| `0x80A8A850` | `fn_80a8a850` | Name-suffix compare after the last `:` | translated | medium | `80a8a850_FUN_80a8a850.c` |
| `0x80A8A970` | `fn_80a8a970` | `0x700`-byte `block_tsuta` constructor | translated | high | `80a8a970_FUN_80a8a970.c` |
| `0x80A8AA90` | `fn_80a8aa90` | Variant initialiser: nibbles, binder, state, collision box | translated | high | `80a8aa90_FUN_80a8aa90.c` |
| `0x80A8ACC0` | `fn_80a8acc0` | Collision update, returns `1` | translated | high | `80a8acc0_FUN_80a8acc0.c` |
| `0x80A8ACF0` | `fn_80a8acf0` | Matrix rebuild and slot-`0x1D` translation store | translated | high | `80a8acf0_FUN_80a8acf0.c` |
| `0x80A8AE30` | `fn_80a8ae30` | `block_tsuta` model and clip binder | translated | high | `80a8ae30_FUN_80a8ae30.c` |
| `0x80A8B130` | `fn_80a8b130` | Floor/side probe with a vertical offset | translated | medium | `80a8b130_FUN_80a8b130.c` |
| `0x80A8B230` | `fn_80a8b230` | Target test and `-12.0`/`12.0` probe | translated | high | `80a8b230_FUN_80a8b230.c` |
| `0x80A8B300` | `fn_80a8b300` | Rate step toward `0.0` or `1.0` | translated | high | `80a8b300_FUN_80a8b300.c` |
| `0x80A8B370` | `fn_80a8b370` | Effect `0x27B` and attach timer | translated | high | `80a8b370_FUN_80a8b370.c` |
| `0x80A8B3F0` | `fn_80a8b3f0` | Frame handler: holders, rate, probe, state fallback | translated | high | `80a8b3f0_FUN_80a8b3f0.c` |
| `0x80A8B4D0` | `fn_80a8b4d0` | Frame variant gated by the tick low nibble | translated | high | `80a8b4d0_FUN_80a8b4d0.c` |
| `0x80A8B560` | `fn_80a8b560` | Advances the two holders only | translated | high | `80a8b560_FUN_80a8b560.c` |
| `0x80A8B5B0` | `fn_80a8b5b0` | `0x700`-byte actor destructor | translated | high | `80a8b5b0_FUN_80a8b5b0.c` |
| `0x80A8B8D0` | `fn_80a8b8d0` | State callback of the small `0x528` class | translated | high | `80a8b8d0_FUN_80a8b8d0.c` |
| `0x80A8BA50` | `fn_80a8ba50` | `0x528`-byte class constructor, vtable `0x80B4F2F0` | translated | high | `80a8ba50_FUN_80a8ba50.c` |
| `0x80A8BFA0` | `fn_80a8bfa0` | `kurage` release: reset two blends, free three holders | translated | high | `80a8bfa0_FUN_80a8bfa0.c` |
| `0x80A8C030` | `fn_80a8c030` | `kurage` placement: two matrices, three holders, unit scale | translated | high | `80a8c030_FUN_80a8c030.c` |
| `0x80A8C150` | `fn_80a8c150` | `kurage` release guarded by `+0x194` | translated | high | `80a8c150_FUN_80a8c150.c` |
| `0x80A8C1C0` | `fn_80a8c1c0` | Clears the blend and applies it to two holders | translated | medium | `80a8c1c0_FUN_80a8c1c0.c` |
| `0x80A8C210` | `fn_80a8c210` | Fade out by 40, hides at zero | translated | high | `80a8c210_FUN_80a8c210.c` |
| `0x80A8C290` | `fn_80a8c290` | Fade by 20, clamps at alpha `100` | translated | high | `80a8c290_FUN_80a8c290.c` |
| `0x80A8C310` | `fn_80a8c310` | Blend step with the `+/-0.01` rate clamps | translated | high | `80a8c310_FUN_80a8c310.c` |
| `0x80A8C3B0` | `fn_80a8c3b0` | Stores `+0x184`, `+0x188`, `+0x18C` from three `f64` | translated | high | `80a8c3b0_FUN_80a8c3b0.c` |

## What the handlers do

### `block_tsuta` (20 bodies)

The class is an ivy-covered block (`tsuta` is "ivy"). Its setup word at
`+0x04` carries two nibbles: the low nibble becomes the variant `+0x6E8`
(`0` = wait/up, `1` = drop) and the next nibble becomes the side `+0x6EC`
(`3` is the mirrored case). `0x80A8AA90` binds the model first, so the
`+0x6E8` value it stores is also what `0x80A8AE30` reads to choose the
`up_anim` clip (variant `0`) or the `wait` clip (anything else).

`0x80A8ACF0` is the per-state transform: it rebuilds the `+0x7C` matrix from
`+0xAC..+0xB4`, applies `+0x102`, `+0x100` and `+0x104`, commits the unit
scale to the animation holder at `+0x54C`, then snapshots animation matrix
slot `0x1D` and keeps its translation in `+0x6D8..+0x6E0`. The two height
words are the reference for the floor logic: `0x80A8B130` probes
`(+0xAC, +0x6DC + dy, +0xB4)` against the local player, and `0x80A8B230`
declares the target reached below `-8.0` (side `3`) or above `8.0`.

The frame handlers then close the loop: `0x80A8B3F0` counts the `+0x6F8`
timer down, advances the two holders through their vtable slots, steps the
rate with `0x80A8B300` and, when the timer is zero, either clears `+0x6F4`
or enters the BSS state `0x80B73C38`. `0x80A8B4D0` is the same without the
state fallback and only advances the `+0x54C` holder when the low nibble of
`0x802DCC4C` is non-zero.

`0x80A8A970` owns the `0x700`-byte allocation, the class vtable
`0x80B4EF18`, the `+0x524` sub-object (`0x80B4F198`) and the collision
object at `+0x5F4`; `0x80A8B5B0` releases them in the reverse order.

### `kurage` (8 bodies)

`0x80A8BFA0` and `0x80A8C150` are the two release paths: the first resets
the `+0xE4`/`+0x110` blends and frees the three holders unconditionally, the
second is guarded by `+0x194`. `0x80A8C030` places the three holders on two
positions, `0x80A8C1C0` clears the blend, and `0x80A8C210`/`0x80A8C290`
fade the first holder's colour (40-unit fade to zero with a visibility
clear, and a 20-unit fade clamped at alpha `100`). `0x80A8C310` steps the
blend value and keeps its rate at `+/-0.01`; `0x80A8C3B0` is the setter for
the target/rate/damping triple.

### `daEnItem_c` (2 bodies)

`0x80A86600` and `0x80A86B00` are the two remaining clip selections of the
switch-item actor; both tail-call the wave-25 [`fn_80a86540`].

## Deferred neighbours

### Holes inside the `block_tsuta` run

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A8A910` | 48 B | Body is only the prologue/epilogue pseudo-call `0x802DCBFC`; the real body is not recoverable. |
| `0x80A8A940` | 48 B | Same as `0x80A8A910`. |
| `0x80A8B0C0` | 100 B | Export ends in `halt_baddata()` after the two-sided height test. |
| `0x80A8B650` | - | Not present in `functions.tsv`; a 0x284-byte hole before `0x80A8B8D0`. |
| `0x80A8B9F0` | 48 B | Prologue/epilogue pseudo-call only. |
| `0x80A8BA20` | 48 B | Same as `0x80A8B9F0`. |

### `kurage` constructor

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A8BD20` | 640 B | The export loses the frame-pointer chain: the prologue/epilogue are printed as calls to `0x802DCD68`/`0x802DCDB4` and the actor is read from the high half of their 64-bit result. The five clip lookups (`kurage_light_mask`, `_display`, `_fog`, `_display_angle`, `_display_noise`) are visible, but the actor register cannot be recovered with confidence. |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80A8C3C0` | 400 B | First `kurage` state helper; not started. |
| `0x80A8C550`..`0x80A8E8E0` | 9 KB + | The rest of the `kurage` state machine, including its own state callbacks/chain walkers at `0x80A8E810..0x80A8E8E0`. |

## Unknowns preserved in code

1. **Class names.** `block_tsuta` and `kurage` come from the BRRES/model
   strings; `daEnBlockTsuta_c` and `daEnKurage_c` are not in the export. The
   field maps are marked `// inferred:`.
2. **BSS descriptors.** `0x80B73BF8`, `0x80B73C78` and `0x80B73C38` are
   written by the runtime state registration, so their static contents are
   empty and the state names are `// unknown:`.
3. **`0x8007FB10`.** The six-argument collision/probe call is kept by address.
   The two stack vectors (near `(-8.0, h, 8.0)` plus trailing zeros, and the
   unit extent `(1, 1, 1)`) are modelled word-for-word; the parameter roles are
   `// inferred:`.
4. **`0x80168180` slot `0x1D`.** The 12-word output is a 3x4 matrix; words
   3, 7 and 11 are its translation, which is why they land in
   `+0x6D8..+0x6E0`. The meaning of slot `0x1D` itself is not established.
5. **`0x8008E770`, `0x80070BA0`, `0x80070BF0`.** The probe helpers are kept by
   address. Their side/blocked returns are inferred from the comparisons
   (`+0x349`, `0x23`); the export does not name them.
6. **`0x80358104`.** The object behind the constant returned by `0x80A8A680`
   is outside this REL. The coin block's `0x80A41E90` returns the same word.
7. **Vtables.** `0x80B4EF18`, `0x80B4F198` and `0x80B4F2F0` are installed but
   their methods are not part of this pass; only the slots used here
   (`+0x14`, `+0x1C`, `+0xD4`) are documented.
8. **`0x80A8C1C0`.** The export prints the first `0x80167B20` call with two
   arguments. The missing `double` is taken from `+0x180`, which the line above
   stores as `0.0` and which the second call reads explicitly. The Rust keeps
   that reading and marks the inference.
9. **`0x80A8C3B0` argument order.** The three `double`s travel in `f1..f3` and
   the actor pointer in `r3`, so the Rust signature lists them before `actor`;
   the `extern "C" fn(f64, f64, f64, u32)` shape is the one already used by
   `call4_f64x3`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`node tools/doc-lint.mjs src` reports 0 violations across 68 files.
`cargo check` succeeds and reports no error or warning in
`src/game/enemies.rs`; the remaining warnings are in other lanes' files and
were not modified here.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave26-enemies.csv`.
