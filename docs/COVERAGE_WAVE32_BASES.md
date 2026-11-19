# Coverage — wave 32 (`d_basesNP`: cannon tail, water float and wire fences)

## How to read this file

This page records the thirty-second translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** — *Translated functions* lists the 30 new bodies in
   `src/game/bases.rs`: a `0x528`-byte cannon-like actor and its spawn/find
   tail, the `obj_waterfloat` constructor/loader pair, and the
   `kanaami_enkei` / `kanaami_urakabe` / `kanaami2` / `remo_kanaami`
   wire-fence actors with their loaders, transforms and lifecycle helpers.
2. **Why these functions?** — *Selection* explains the address window and the
   untranslated survivors that were taken as complete clusters.
3. **What is still missing?** — *Deferred candidates* records the adjacent
   bodies that need a paired-single or save-prologue ABI pass, plus the
   argument-dropping decompiler cases.

Column meanings:

- **Status** — `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** — strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, constant or called
  body is inferred; `low` = mostly guesswork.
- **Evidence** — file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8004xxxx`..`0x8043xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 32 |
|---|---:|
| New translated functions | 30 |
| Actor constructors (`0x80750BF0`, `0x80752100`, `0x80755950`) | 3 |
| BRRES model loaders (`0x807525E0`, `0x807550E0`, `0x80755820`, `0x80755D30`, `0x80756C70`) | 5 |
| Lifecycle, transform, sweep, sound and effect bodies | 22 |
| Selected bytes | 4,416 |
| Translated functions in `src/game/bases.rs` (cumulative) | 819 (817 translated, 2 partial) |
| Cumulative selected bytes | 245,180 (22.09 % of indexed bytes) |

The 30 new bodies select 4,416 of the 1,109,860 indexed bytes (0.40 %) and 30
of the 5,409 eligible entries (0.55 %). 29 rows are `high` confidence; one is
`medium` (`0x80755740`) because Ghidra dropped the four arguments of the
`0x801C0BD0` matrix call and the reconstruction restores them from the stack
locals and the identical call in `8074e7d0_FUN_8074e7d0.c`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs`, and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2–31 were removed before selection. Wave 31 stopped at
`0x8074FF90`; this pass continued with the next untranslated clusters:

1. **Cannon-like tail (`0x80750BF0..0x80750F20`).** `0x80750BF0` allocates
   `0x528` bytes and installs vtable `0x80812810`; `0x80750D20` advances and
   releases it, `0x80750DB0` spawns type `0x17B`, `0x80750E20` finds a
   type-`0x22A` actor by the `+0x518` key and `0x80750F20` spawns when the
   watched actor at `+0x520` is in state `1`. The nearby `0x80750000`,
   `0x80750090`, `0x80750180` and `0x80750220` belong to the wave-31
   `warp_cannon`/`0x6A5` tail and were not part of this cluster.
2. **`obj_waterfloat` (`0x80752100..0x807525E0`).** `0x80752100` builds the
   `0x4C0`-byte object with vtable `0x80812DC0`; `0x807525E0` loads
   `g3d/obj_waterfloat.brres`, selects pane/animation by `+0x4B4` and builds a
   `0x224` clip; `0x807525A0` is the small `0x800EBB30` notification used by
   the same object.
3. **`kanaami_enkei` / `kanaami_urakabe` (`0x807550E0..0x80755740`).**
   `0x807550E0` and `0x80755820` load the two BRRES files for the already
   translated constructors `0x80754E30` / `0x80755520`; `0x80755690`,
   `0x80755710` and `0x80755740` are the release/reset/transform pair for the
   `0x4D8`-byte body.
4. **`kanaami2` (`0x80755950..0x80756160`).** The `0x568`-byte constructor,
   its variant-pane loader, transform, countdown, five-step float sequence and
   the two axis updates.
5. **`remo_kanaami` (`0x80756830..0x80757220`).** The start-up body, matching
   destructor and BRRES/clip/animation loader, plus the burst, rider-count
   animation and the two sound/interaction timers. The constructor
   `0x80756700` was already translated in wave 9.
6. **Rejected or deferred nearby bodies.** `0x80752730`, `0x80752B90` and
   `0x80751C90` contain paired-single (`unaff_GQR*`, `ldexpf`) artifacts;
   `0x807529B0` and `0x80756FD0` use the `0x802DCD74` save prologue;
   `0x807571C0`, `0x807571E0` and `0x80757280` contain bad-instruction control
   flow; `0x80750EA0` calls `0x80750E20` with a dropped argument. All were left
   for a dedicated ABI pass.

## Clusters

### Cannon-like actor (`0x80750BF0..0x80750F20`)

- `0x80750BF0` builds a `0x528`-byte object with vtable `0x80812810`,
  interface `+0x3D0`, model `+0x3F0` and clip `+0x430`.
- `0x80750D20` fires virtual `+0x10` on the state object at `+0x394`, calls the
  4-byte helper `0x80750DA0`, releases with `0x80064F50` and returns `1`.
- `0x80750DB0` spawns type `0x17B` through `0x80064610` with the flags word at
  `+0x04` and the position at `+0xAC`.
- `0x80750E20` walks the type-`0x22A` list with `0x80162D50`, feeding the
  returned actor back as the next cursor, and matches `+0x518` against
  `+0x3D8`.
- `0x80750F20` resolves `+0x520` with `0x80162D00` and spawns when the target's
  state byte `+0x3DC` is `1`.

### `obj_waterfloat` (`0x80752100..0x807525E0`)

- `0x80752100` installs vtable `0x80812DC0`, interface `+0x3D4`, resource
  `+0x3F0`, model `+0x3F4`, animation vtable `+0x438` (first `0x803295D8`, then
  `0x80329578`), clip `+0x444` and clears `+0x460`.
- `0x807525E0` opens `+0x3D4` at priority `0x20`, loads
  `g3d/obj_waterfloat.brres`, picks the pane from `0x80812D7C` and the
  animation from `0x80812D88` by `+0x4B4`, builds the `0x224` clip, starts it
  at rate `0x85` and closes the accessor.
- `0x807525A0` calls `0x800EBB30(*(0x8042A100), actor[+0x4A4])` and returns
  `1`; the callee role is `// unknown:`.

### Wire fences (`0x807550E0..0x80757220`)

- `0x807550E0` loads `g3d/kanaami_enkei.brres` and binds pane
  `kanaami_enkei` for the `0x518`-byte constructor `0x80754E30`.
- `0x80755820` loads `g3d/kanaami_urakabe.brres` and binds pane
  `kanaami_urakabe` for the `0x4D8`-byte constructor `0x80755520`.
  `0x80755740` copies that actor's transform, `0x80755690` releases it and
  `0x80755710` resets the model.
- `0x80755950` builds the `0x568`-byte `kanaami2` actor with vtable
  `0x808137D8`. `0x80755D30` loads `g3d/kanaami2.brres` and picks a pane from
  the four-entry table `0x808137A8` by `+0x560`; `0x80755DF0` copies the
  transform. `0x80755ED0`..`0x80756160` implement the ping-pong sweep: a
  `+0x548` countdown gates the step, `0x80755F70` clamps to the two `+0x538`
  limits and `0x80755F00` adds a five-step float sequence.
- `0x80756830` initialises the `0x640`-byte `remo_kanaami` actor: it derives
  the variant from the low flags nibble through `0x807CE990`/`0x807CE998`,
  loads the model with `0x80756C70`, builds the ten-word `0x8007FB10` block,
  starts the clip and fires state event `0x80832280`.
- `0x80756C70` loads `g3d/remo_kanaami.brres`, binds `remo_kanaami` and
  `change_mark`, builds the clip at priority `0x23`, creates the second
  animation and starts both at constant `0x807CE9B8`.
- `0x80756E30` spawns `Wm_en_burst_s`; `0x80756E90` follows the player count
  at `0x8005F900`'s `+0x14D4` and plays the mount/dismount burst;
  `0x80757060`, `0x80757140` and `0x80757220` own the interaction timer and
  sounds `0x2A7` / `0x2A6`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave32-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80750BF0` | `fn_80750bf0` | construct the `0x528`-byte cannon-like actor | translated | high | `80750bf0_FUN_80750bf0.c` |
| `0x80750D20` | `fn_80750d20` | advance and release the actor | translated | high | `80750d20_FUN_80750d20.c` |
| `0x80750DB0` | `fn_80750db0` | spawn actor type `0x17B` | translated | high | `80750db0_FUN_80750db0.c` |
| `0x80750E20` | `fn_80750e20` | find actor type `0x22A` by key | translated | high | `80750e20_FUN_80750e20.c` |
| `0x80750F20` | `fn_80750f20` | spawn when the watched actor is ready | translated | high | `80750f20_FUN_80750f20.c` |
| `0x80752100` | `fn_80752100` | construct the `obj_waterfloat` actor | translated | high | `80752100_FUN_80752100.c` |
| `0x807525A0` | `fn_807525a0` | notify `0x800EBB30` and return `1` | translated | high | `807525a0_FUN_807525a0.c` |
| `0x807525E0` | `fn_807525e0` | load `obj_waterfloat` and its clip | translated | high | `807525e0_FUN_807525e0.c` |
| `0x807550E0` | `fn_807550e0` | load `kanaami_enkei` | translated | high | `807550e0_FUN_807550e0.c` |
| `0x80755690` | `fn_80755690` | release the `0x4D8`-byte fence actor | translated | high | `80755690_FUN_80755690.c` |
| `0x80755710` | `fn_80755710` | reset the fence model | translated | high | `80755710_FUN_80755710.c` |
| `0x80755740` | `fn_80755740` | copy the `0x4D8`-byte fence transform | translated | medium | `80755740_FUN_80755740.c` |
| `0x80755820` | `fn_80755820` | load `kanaami_urakabe` | translated | high | `80755820_FUN_80755820.c` |
| `0x80755950` | `fn_80755950` | construct the `0x568`-byte `kanaami2` actor | translated | high | `80755950_FUN_80755950.c` |
| `0x80755D00` | `fn_80755d00` | reset the `kanaami2` model | translated | high | `80755d00_FUN_80755d00.c` |
| `0x80755D30` | `fn_80755d30` | load `kanaami2` and its variant pane | translated | high | `80755d30_FUN_80755d30.c` |
| `0x80755DF0` | `fn_80755df0` | copy the `kanaami2` transform | translated | high | `80755df0_FUN_80755df0.c` |
| `0x80755ED0` | `fn_80755ed0` | decrement the sweep countdown | translated | high | `80755ed0_FUN_80755ed0.c` |
| `0x80755F00` | `fn_80755f00` | return the next five-step float | translated | high | `80755f00_FUN_80755f00.c` |
| `0x80755F70` | `fn_80755f70` | clamp one axis at the sweep limits | translated | high | `80755f70_FUN_80755f70.c` |
| `0x807560D0` | `fn_807560d0` | advance the y axis | translated | high | `807560d0_FUN_807560d0.c` |
| `0x80756160` | `fn_80756160` | advance the x axis | translated | high | `80756160_FUN_80756160.c` |
| `0x80756830` | `fn_80756830` | initialise the `remo_kanaami` actor | translated | high | `80756830_FUN_80756830.c` |
| `0x80756970` | `fn_80756970` | release the `remo_kanaami` actor | translated | high | `80756970_FUN_80756970.c` |
| `0x80756C70` | `fn_80756c70` | load `remo_kanaami`, clip and animation | translated | high | `80756c70_FUN_80756c70.c` |
| `0x80756E30` | `fn_80756e30` | spawn `Wm_en_burst_s` | translated | high | `80756e30_FUN_80756e30.c` |
| `0x80756E90` | `fn_80756e90` | update the rider-count animation | translated | high | `80756e90_FUN_80756e90.c` |
| `0x80757060` | `fn_80757060` | start the player interaction | translated | high | `80757060_FUN_80757060.c` |
| `0x80757140` | `fn_80757140` | play sound `0x2A6` | translated | high | `80757140_FUN_80757140.c` |
| `0x80757220` | `fn_80757220` | advance the interaction timer | translated | high | `80757220_FUN_80757220.c` |

**Totals:** 30 translated, 0 partial, 0 stub (29 high, 1 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x80750EA0` | 92 | calls `0x80750E20` with a dropped argument; needs the caller-register pass. |
| `0x80751C90` | 120 | paired-single reconstruction (`unaff_GQR3`, `ldexpf`). |
| `0x80752730` | 628 | paired-single reconstruction (`unaff_GQR0`, two `ldexpf` blocks). |
| `0x807529B0` | 260 | `0x802DCD74` save prologue and a returned-frame value. |
| `0x80752B90` | 156 | paired-single reconstruction (`unaff_GQR3`). |
| `0x80756FD0` | 136 | `0x802DCD74` save prologue; returned frame word split across two words. |
| `0x807571C0` | 28 | Ghidra `halt_baddata`; no recoverable control flow. |
| `0x807571E0` | 64 | wraps the bad `0x807571C0` body. |
| `0x80757280` | 120 | bad-instruction tail after the two interaction checks. |

The two 48-byte `0x802DCBFC` thunks in this window (`0x80750B90`, `0x80750BC0`)
and the equivalent `0x80753170` / `0x807531A0` pairs are compiler veneers with
no Rust-visible effect.

## Unknowns preserved

1. **`0x800DF180`.** The BRRES loader identity is not in this export; all five
   loaders keep the `// unknown:` note.
2. **`0x8042A100` / `0x800EBB30`.** The water manager pointer and its
   notification call are kept by address; the callee role is unresolved.
3. **`0x8005F900`.** The C prints the player-count getter with no arguments in
   `0x80756E90`; the reconstruction uses `call0_ret`, matching the empty
   prototype.
4. **Cannon-like identity.** The `0x528`-byte actor has no string. Its role is
   `// inferred:` from its spawn (`0x17B`), watcher (`0x22A`) and the adjacent
   `warp_cannon` tail.
5. **`0x80755740` argument restoration.** The C prints only the matrix pointer
   for `0x801C0BD0`; the three position floats are restored from the untouched
   stack locals and the explicit call in `8074e7d0_FUN_8074e7d0.c`.
6. **`0x80756830` parameter block.** The ten-word stack block is modelled as
   one array with the two original pointers at offsets `0` and `3`, preserving
   the C's `&local_38` / `&local_2c` split.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 819 functions after this pass (817 `translated`, 2
pre-existing `partial`); this pass adds 30, all with `# Original` blocks and the
synthetic-base warning. `cargo check` exits 0 and produces no warnings in
`src/game/bases.rs`. `node tools/doc-lint.mjs src` reports `ok: true`, 0
violations over 68 files.
