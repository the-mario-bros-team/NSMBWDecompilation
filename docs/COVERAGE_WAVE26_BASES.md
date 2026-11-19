# Coverage - wave 26 (`d_basesNP`: zen-yogan and huriko lifts)

## How to read this file

This page records the twenty-sixth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended to
   `src/game/bases.rs`: the `lift_zen_yogan_down` elevator family, the
   `lift_huriko` axle actor, the hand destructor plus the shared lift helpers,
   and five bodies of the unnamed `0x9B38`-byte lift manager.
2. **Why these functions?** - *Selection* explains how the pass continued the
   address-ordered sweep above the wave-25 window and which decompiler outputs
   were skipped.
3. **What is still missing?** - *Deferred candidates* records the duplicate
   release helpers, the paired-single bodies and the recursive collector that
   need their own pass.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, table meaning or
  called body is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006xxxx`, `0x8016xxxx`, `0x8023xxxx`, `0x802Dxxxx`, `0x8032xxxx`,
`0x8037xxxx`, `0x8042xxxx`, `0x807Fxxxx`, `0x8082xxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output (or in the instruction stream where the C drops an argument);
`// inferred:` = role guessed from callers/shape; `// unknown:` = unresolved;
`// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 26 |
|---|---:|
| New translated functions | 30 |
| `lift_zen_yogan_down` family (`0x806C9080..0x806C9B70`) | 8 |
| `lift_huriko` axle family (`0x806CA130..0x806CB2F0`) | 11 |
| `lift_huriko` hand and shared helpers (`0x806CBEC0..0x806CC990`) | 6 |
| Unnamed `0x9B38` lift manager (`0x806CD050..0x806CD720`) | 5 |
| Selected bytes | 4,512 |
| Translated functions in `src/game/bases.rs` (cumulative ledger rows) | 641 (639 translated, 2 partial) |
| Cumulative selected bytes | 209,904 (18.91 % of indexed bytes) |

The 30 new bodies select 4,512 of the 1,109,860 indexed bytes (0.41 %) and 30 of
the 5,409 eligible entries (0.55 %). 27 rows are `high` confidence; 3 are
`medium` (`0x806C91B0`, `0x806CA300`, `0x806CAAD0`) because a probe layout or a
called rendering helper is not fully recovered. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2-25 were removed before selection. Wave 25 stopped at
`0x806C8AD0`; this pass starts at `0x806C9080` and moves upward to `0x806CD720`.

1. **Finish the `lift_zen_yogan_down` actor.** `0x806C9080` and `0x806C94A0`
   are the two heap constructors (`0x4F0` and `0x628` bytes); `0x806C9100`
   opens `g3d/lift_zen_yogan_down.brres`; `0x806C91B0` is the variant
   initialiser; `0x806C9590` builds the `0x2C`-byte array element;
   `0x806C9690` selects the animation from two named panes; `0x806C9760` is the
   small membership test used by the collectors; `0x806C9B70` is the frame
   callback.
2. **Take the `lift_huriko` axle actor as a lifecycle.** `0x806CA9A0` is the
   heap constructor (`0x550` bytes, vtable `0x807F9C10`); `0x806CAE40` loads
   `g3d/lift_huriko.brres` and three `lift_huriko_kusari` chain parts through
   `0x806CABD0`; `0x806CAAD0` rebuilds the visible rope/cart attachment;
   `0x806CA130`/`0x806CA300` drive the rider count and settle timer;
   `0x806CAF50`/`0x806CAFB0`/`0x806CB010` are the two actor lookups;
   `0x806CB2F0` builds the player-slot mask.
3. **Continue the hand and shared helper bodies.** `0x806CBEC0` is the only
   hand-specific body (it releases `+0x3B4` and `+0x394`, the fields built by
   the already translated `0x806CBC70`/`0x806CBCE0`). `0x806CC410` and
   `0x806CC480` are the model callback and clamped rider count; `0x806CC870`/
   `0x806CC8B0` reset the model and position; `0x806CC990` chooses a state from
   input and occupancy. Their `+0x508`/`+0x654` fields identify them as helpers
   for the same lift group, not as hand methods (`// inferred:`).
4. **Open the unnamed `0x9B38`-byte manager.** `0x806CD050` allocates the
   object and its two `0x802DC9A0` array blocks; `0x806CD120`/`0x806CD190` are
   the element constructor/destructor; `0x806CD6E0`/`0x806CD720` are the first
   two frame callbacks.
5. **Rejected or deferred decompiler failures.** `0x806CAC40`, `0x806CAD10`,
   `0x806CB050` and `0x806CB140` are paired-single reconstructions
   (`ldexpf`, `unaff_GQR0`/`unaff_GQR3`). `0x806C97D0` uses the
   `0x802DCD74`/`0x802DCDC0` recursive frame helpers and is left for a
   dedicated collector pass. `0x806C9600` prints three `undefined8`
   parameters and needs an ABI review.

## Clusters

### `lift_zen_yogan_down` (`0x806C9080..0x806C9B70`)

- `0x806C9080` installs the temporary vtable `0x80307910` and then
  `0x807F9700`, builds the `+0x3DC` callback block, the `+0x4C0` model accessor
  and the `+0x4E0` part.
- `0x806C9100` is the shared loader: it opens
  `s_g3d_lift_zen_yogan_down_brres_807F96CC`, binds the caller's pane into
  `+0x4E0` and releases the accessor.
- `0x806C91B0` reads the variant from `+0x04 & 3` and indexes the 8-byte table
  at `0x807CB188` (`S`/`M`/`L` pane plus a size class). It writes `1.0` at
  `+0xDC..+0xE4`, `192`/`-192`/`96`/`-96` at `+0x318..+0x324`, `8 * class` at
  `+0xD0`, `-72` at `+0xD4`, and builds the seven-word probe
  `(0, 0, 16 * class, -144)`; `0x800454C0` fills its three tail words with the
  handler pointers `0x800451F0`/`0x80045270`/`0x800452E0`, then `0x8007FB10`
  registers it.
- `0x806C94A0` is the second constructor (`0x628` bytes, vtable `0x807F9838`)
  with `0x40 x 3` and `0x2C x 3` arrays. `0x806C9590` is the `0x2C`-byte array
  element; `0x806C9690` picks `1.0`/`2.0`/`0.0` from the panes at
  `0x8035404C`/`0x8035408C`.
- `0x806C9B70` reads the actor position, builds `+0x7C` and binds the model at
  `+0x3F8`.

### `lift_huriko` axle and shared helpers (`0x806CA130..0x806CC990`)

- `0x806CA130` counts riders on the `+0x518` list and compares the count with
  `max(1, players - (3 - pair_count))`; `0x806CA240` uses that answer to pick
  `0x8082D208` or `0x8082D248`. `0x806CA300` adds a small random hover every
  third tick and settles back to `+0x584..+0x58C` at tick 32.
- `0x806CA9A0` builds the axle (`0x550` bytes); `0x806CAE40` loads the two
  caller panes and three `0x5C`-byte chain slots; `0x806CABD0` is the single
  chain-part loader; `0x806CAAD0` rebuilds the cart attachment.
- `0x806CAF50`/`0x806CAFB0` search the global list for type `0x1A3`/`0x1A4`
  and a matching byte at `+0x39C`/`+0x3E2`; `0x806CB010` chains them.
  `0x806CB2F0` turns the rider list at `+0x420` into a four-bit player mask.
- `0x806CBEC0` releases the hand object; `0x806CC410` advances the lift model
  while active; `0x806CC480` is the clamped count used by the manager;
  `0x806CC870`/`0x806CC8B0` reset the lift model and position; `0x806CC990`
  chooses a state from input masks `0x8042A078` and the occupancy answer.

### Unnamed `0x9B38` lift manager (`0x806CD050..0x806CD720`)

- `0x806CD050` allocates `0x9B38` bytes, installs vtable `0x807FA1A8`, builds
  the `0x60 x 0x100` array at `+0x3F0`, the `0x0C x 0x400` array at `+0x63F4`
  and the callback block at `+0x9A0C`.
- `0x806CD120`/`0x806CD190` are the array element constructor/destructor
  (vtable `0x807FA290`, seeds `+0x58 = -1`, `+0x5C = 0xFF`).
- `0x806CD6E0` runs `0x800BE550` while `+0x9B34` is zero; `0x806CD720` runs
  `0x800BE5F0` and releases `+0x9A0C`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave26-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806C9080` | `fn_806c9080` | allocate and construct the `0x4F0`-byte zen-yogan lift | translated | high | `806c9080_FUN_806c9080.c` |
| `0x806C9100` | `fn_806c9100` | load `lift_zen_yogan_down` and bind the caller pane | translated | high | `806c9100_FUN_806c9100.c` |
| `0x806C91B0` | `fn_806c91b0` | initialise the variant, bounds and first probe | translated | medium | `806c91b0_FUN_806c91b0.c` |
| `0x806C94A0` | `fn_806c94a0` | allocate and construct the `0x628`-byte lift | translated | high | `806c94a0_FUN_806c94a0.c` |
| `0x806C9590` | `fn_806c9590` | construct the `0x2C`-byte array element | translated | high | `806c9590_FUN_806c9590.c` |
| `0x806C9690` | `fn_806c9690` | choose the animation rate from two panes | translated | high | `806c9690_FUN_806c9690.c` |
| `0x806C9760` | `fn_806c9760` | test a word for membership in a short array | translated | high | `806c9760_FUN_806c9760.c` |
| `0x806C9B70` | `fn_806c9b70` | place the model from the actor position | translated | high | `806c9b70_FUN_806c9b70.c` |
| `0x806CA130` | `fn_806ca130` | count riders and compare with the player capacity | translated | high | `806ca130_FUN_806ca130.c` |
| `0x806CA240` | `fn_806ca240` | switch to the full or waiting state | translated | high | `806ca240_FUN_806ca240.c` |
| `0x806CA300` | `fn_806ca300` | hover the lift and settle it at the timer | translated | medium | `806ca300_FUN_806ca300.c` |
| `0x806CA9A0` | `fn_806ca9a0` | allocate and construct the `0x550`-byte huriko axle | translated | high | `806ca9a0_FUN_806ca9a0.c` |
| `0x806CAAD0` | `fn_806caad0` | rebuild the visible rope/cart attachment | translated | medium | `806caad0_FUN_806caad0.c` |
| `0x806CABD0` | `fn_806cabd0` | load one `lift_huriko_kusari` chain part | translated | high | `806cabd0_FUN_806cabd0.c` |
| `0x806CAE40` | `fn_806cae40` | load `lift_huriko` and three chain parts | translated | high | `806cae40_FUN_806cae40.c` |
| `0x806CAF50` | `fn_806caf50` | find the `0x1A3` actor by its `+0x39C` byte | translated | high | `806caf50_FUN_806caf50.c` |
| `0x806CAFB0` | `fn_806cafb0` | find the `0x1A4` actor by its `+0x3E2` byte | translated | high | `806cafb0_FUN_806cafb0.c` |
| `0x806CB010` | `fn_806cb010` | try the first lookup, then the second | translated | high | `806cb010_FUN_806cb010.c` |
| `0x806CB2F0` | `fn_806cb2f0` | build the four-bit player-slot mask | translated | high | `806cb2f0_FUN_806cb2f0.c` |
| `0x806CBEC0` | `fn_806cbec0` | destroy the huriko hand object | translated | high | `806cbec0_FUN_806cbec0.c` |
| `0x806CC410` | `fn_806cc410` | advance the lift model while active | translated | high | `806cc410_FUN_806cc410.c` |
| `0x806CC480` | `fn_806cc480` | count riders and report whether the limit is reached | translated | high | `806cc480_FUN_806cc480.c` |
| `0x806CC870` | `fn_806cc870` | reset the lift model and sub-object | translated | high | `806cc870_FUN_806cc870.c` |
| `0x806CC8B0` | `fn_806cc8b0` | restore the lift spawn position and reset both parts | translated | high | `806cc8b0_FUN_806cc8b0.c` |
| `0x806CC990` | `fn_806cc990` | choose the lift state from input and occupancy | translated | high | `806cc990_FUN_806cc990.c` |
| `0x806CD050` | `fn_806cd050` | allocate and construct the `0x9B38`-byte lift manager | translated | high | `806cd050_FUN_806cd050.c` |
| `0x806CD120` | `fn_806cd120` | construct one `0x60`-byte manager array element | translated | high | `806cd120_FUN_806cd120.c` |
| `0x806CD190` | `fn_806cd190` | destroy one `0x60`-byte manager array element | translated | high | `806cd190_FUN_806cd190.c` |
| `0x806CD6E0` | `fn_806cd6e0` | advance the manager model while idle | translated | high | `806cd6e0_FUN_806cd6e0.c` |
| `0x806CD720` | `fn_806cd720` | advance the manager model and callback block | translated | high | `806cd720_FUN_806cd720.c` |

**Totals:** 30 translated, 0 partial, 0 stub (27 high, 3 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806C8F00`, `0x806CA820`, `0x806CBAF0`, `0x806CCED0` | 88 each | Release helpers with the same null-gate + `0x8015F800` + `0x802B9280` shape as [`fn_806ba180`]. |
| `0x806C9020`, `0x806C9050`, `0x806CA940`, `0x806CA970`, `0x806CBC10`, `0x806CBC40`, `0x806CCFF0`, `0x806CD020` | 48 each | `0x802DCBFC` thunks with no independent behaviour. |
| `0x806C9600` | 136 | Model attach whose C signature prints three `undefined8` parameters; needs an ABI review. |
| `0x806C97D0`, `0x806C9940` | 360, 132 | Recursive scene-graph collector built on `0x802DCD74`/`0x802DCDC0`; leave for a dedicated collector pass. |
| `0x806CAC40`, `0x806CAD10`, `0x806CB050`, `0x806CB140` | 196..304 | **Rejected**: paired-single reconstruction (`ldexpf`, `unaff_GQR0`/`unaff_GQR3`). |
| `0x806CB210`, `0x806CB270` | 84, 124 | Callers of the rejected `0x806CB140`; their own flow is trivial but the callee is not translated. |
| `0x806CC530`, `0x806CC620` | 228, 196 | Rendering helpers that index the computed table at `0x807CB2F8` and call three `(double)`-printed main.dol routines; need typed wrappers. |
| `0x806CC6F0` | 384 | **Rejected**: reads `extraout_f1`, so the landing height does not survive decompilation. |
| `0x806CD210` | 196 | Identical twin of [`fn_806cd050`]; translate with the next manager pass. |
| `0x806CD760`, `0x806CD950` | 136, 228 | Manager height and panel-switch bodies above this pass's window. |

## Unknowns preserved

1. **`0x800B2F50`.** [`fn_806ca300`] calls it twice with no arguments; the
   Rust models it with [`call0_f32`] and marks the role `// unknown:`.
2. **Dropped receivers.** `0x8012DBE0`, `0x8005FFB0`, `0x8016A010` and the
   second `+0x6C` virtual print no argument in the C. The Rust passes the
   actor, cross-checked against `0x806CB2F0`, where the same `0x8012DBE0` call
   prints its argument.
3. **`0x800454C0` / `0x8007FB10`.** [`fn_806c91b0`] registers a three-point
   probe with them; the prototype and the probe semantics are unresolved.
4. **`0x800C0210`.** [`fn_806c9690`] applies `1.0`/`2.0`/`0.0` through it; the
   export names no operation.
5. **Rendering chain.** `0x8025C660`, `0x8025C420`, `0x8025C470`,
   `0x8023CC90` and `0x8023CFA0` in [`fn_806caad0`] are grouped by shape, not
   by a recovered API.
6. **Probe layout.** The C of [`fn_806c91b0`] names the probe words
   `local_3c`..`local_30`; the instruction stream shows four `stfs` to
   `sp+0x14..+0x20` and then passes that pointer to `0x800454C0` and
   `0x8007FB10`. `0x800454C0` (`0x800454C0..0x800454E4`, unindexed in the
   main.dol export) writes three handler pointers at `+0x10..+0x18`, so the
   Rust uses a `[u32; 7]` buffer.
7. **Flag fields.** The `+0x04` word supplies the variant (`& 3`), the
   pair-count field (bits 12..15) and the state variant (bits 4..7); the meaning
   of each field is `// inferred:` from these three uses.
8. **Class attribution.** `lift_zen_yogan_down` and `lift_huriko` are explicit
   BRRES names; the `0x9B38` manager has no string in this window and stays
   unnamed.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 641 ledger rows after this pass (639 `translated`,
2 pre-existing `partial`); this pass adds 30, all with `# Original` blocks and
the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0; the remaining warnings are in other lanes' files
(`src/runtime/bt.rs`, `src/runtime/input.rs`).
