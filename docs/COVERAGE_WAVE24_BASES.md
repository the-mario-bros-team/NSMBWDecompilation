# Coverage - wave 24 (`d_basesNP` Koopa attack, jump and ship actors)

## How to read this file

This page records the twenty-fourth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 29 bodies appended to
   `src/game/bases.rs`: two Koopa effect actors, the `0xC50`-byte jump actor
   with its animation state machine, and the `KoopaShip` constructor/BRRES
   loader.
2. **Why these functions?** - *Selection* explains how the pass continued the
   address-ordered sweep above the wave-23 window and which incomplete
   decompiler outputs were skipped.
3. **What is still missing?** - *Deferred candidates* records the destructor
   thunks, the `0x806B9CB0`/`0x806BA660` bodies that call truncated code, the
   jump-actor allocation pair and the remaining `KoopaShip` helpers.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or table meaning is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006xxxx`, `0x8016xxxx`, `0x802Dxxxx`, `0x8037xxxx`, `0x807Cxxxx`) are
imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 24 |
|---|---:|
| New translated functions | 29 |
| Koopa effect actors (`0x806B9C60..0x806BB230`) | 11 |
| Jump actor (`0x806BC410..0x806BE160`) | 16 |
| `KoopaShip` constructor/loader (`0x806BF680`, `0x806BFB80`) | 2 |
| Selected bytes | 7,292 |
| Translated functions in `src/game/bases.rs` (cumulative ledger) | 581 |
| Cumulative selected bytes | 200,176 (18.04 % of indexed bytes) |

The 29 new bodies select 7,292 of the 1,109,860 indexed bytes (0.66 %) and 29
of the 5,409 eligible entries (0.54 %). All 29 rows are `high` confidence: every
call, store, constant and branch is explicit. No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2-23 were removed before selection. Wave 23 stopped at
`0x806B9930` and left the next actor family untouched; this pass starts at
`0x806B9C60` and moves upward to `0x806BFB80`.

1. **Continue the address sweep.** Every selected file has a complete
   control-flow graph; no selected file contains `halt_baddata`, `unaff_`, an
   unrecovered jump table, or an `ldexpf`/paired-single reconstruction.
2. **Follow the first constructor.** `0x806BA300` allocates `0x3D8` bytes and
   installs vtable `0x807F6440`; `0x806BABC0` loads the `boss_ef_attack` BRRES
   and binds `koopa_attack`. The adjacent bodies `0x806BA530..0x806BB230` are
   that actor's collision, ground test, point setup and fire-attack spawn.
3. **Follow the second constructor.** `0x806BA9A0` builds a second `0x718`-byte
   object with vtable `0x807F66C8`; the shared helper `0x806BAED0` builds its
   collision triangle.
4. **Follow the jump actor.** `0x806BC410..0x806BE160` walk the `+0x30` chain,
   copy the player's `Position_Jr` bone, and implement every `wait`/`prov`/
   `go`/`jump_st`/`jumpA`/`jump_ed`/`shock` state.
5. **Take the ship pair.** `0x806BF680` builds the `KoopaShip` object and its
   six callback tables; `0x806BFB80` loads the `KoopaShip` BRRES.
6. **Rejected decompiler failures.** `0x806BA630` is `halt_baddata`;
   `0x806BAEB0` has an unrecovered jump table. `0x806B9CB0` and `0x806BA660`
   call truncated bodies and were skipped with them.

## Clusters

### Koopa effect actors (`0x806B9C60..0x806BB230`)

- `0x806B9C60` queues sound `0x57F` at the actor position.
- `0x806BA300` allocates the `0x3D8`-byte object with vtable `0x807F6440`;
  `0x806BA9A0` builds the second `0x718`-byte object with vtable `0x807F66C8`.
- `0x806BA530` is the collision callback; `0x806BA5A0` releases the local
  player flag; `0x806BA5E0` is the ground query.
- `0x806BABC0` opens `s_g3d_boss_ef_attack_brres_807F65F4` and binds
  `s_koopa_attack_807F6620`.
- `0x806BAED0` converts the nine triangle coordinates at
  `0x807F65AC..0x807F65CC` to fixed-point and submits three points to
  `0x8006D090`.
- `0x806BACF0` is the periodic tick; `0x806BB230` spawns
  `s_Wm_ko_fireattack_807F6630` while `0x807F66C1` is set.

The class names do not survive; the attributions are `// unknown:`.

### Jump actor (`0x806BC410..0x806BE160`)

- `0x806BC410` is the five-link `+0x30` walk with recursion; `0x806BC4F0` is
  the name-suffix compare used by the same family.
- `0x806BC7E0` starts the jump when `+0xC3C` is set; `0x806BCA10` copies the
  player's `Position_Jr` bone (`+0x910..+0x93C`) into the actor matrix.
- `0x806BCC20`/`0x806BCCF0` bind `prov` and run its state; `0x806BCDB0` binds
  `wait` and flips the variant byte.
- `0x806BCE60` is the main state machine (`jump_st`, `jumpA`, `jump_ed`) and
  integrates the vertical arc to estimate the descent time.
- `0x806BD2A0`/`0x806BD2F0` initialise and run the `go`/`go_wait` approach.
- `0x806BD510`/`0x806BD610`/`0x806BD7E0`/`0x806BD880`/`0x806BDB70` implement
  the `prov`/`prov2`/`wait`/`shock`/`jumpA` states.
- `0x806BE160` scales the current angle by the `main.dol` facing table
  `0x8042C1A0`.

The animation names (`wait`, `prov`, `go`, `jump_st`, `jumpA`, `jump_ed`,
`prov2`, `shock`) come from `0x807F69B4..0x807F69F8`; the class name is
`// inferred:`.

### `KoopaShip` actor (`0x806BF680..0x806BFB80`)

- `0x806BF680` runs the base constructor `0x80094E80`, installs vtable
  `0x807F74A8`, and builds six `0x802DC9A0` callback tables at
  `+0x5D8`, `+0x63C`, `+0x6A0`, `+0x734`, `+0x7C8` and `+0x85C`.
- `0x806BFB80` opens `s_g3d_KoopaShip_brres_807F746C` and binds
  `s_KoopaShip_807F7480` at `+0x540`/`+0x544`/`+0x584`.

The class attribution is `// inferred:` from the BRRES and pane strings.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave24-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806B9C60` | `fn_806b9c60` | queue sound `0x57F` at the actor position | translated | high | `806b9c60_FUN_806b9c60.c` |
| `0x806BA180` | `fn_806ba180` | release an object, optionally free it | translated | high | `806ba180_FUN_806ba180.c` |
| `0x806BA300` | `fn_806ba300` | allocate `0x3D8` bytes and install vtable `0x807F6440` | translated | high | `806ba300_FUN_806ba300.c` |
| `0x806BA530` | `fn_806ba530` | collision callback: state gate, virtual hit, player flag | translated | high | `806ba530_FUN_806ba530.c` |
| `0x806BA5A0` | `fn_806ba5a0` | set the local player's `+0x780` availability flag | translated | high | `806ba5a0_FUN_806ba5a0.c` |
| `0x806BA5E0` | `fn_806ba5e0` | ground query with the `0x8000` result mask | translated | high | `806ba5e0_FUN_806ba5e0.c` |
| `0x806BA9A0` | `fn_806ba9a0` | allocate `0x718` bytes and build the second effect actor | translated | high | `806ba9a0_FUN_806ba9a0.c` |
| `0x806BABC0` | `fn_806babc0` | load `boss_ef_attack` and bind `koopa_attack` | translated | high | `806babc0_FUN_806babc0.c` |
| `0x806BACF0` | `fn_806bacf0` | refresh the interface and count two timers | translated | high | `806bacf0_FUN_806bacf0.c` |
| `0x806BAED0` | `fn_806baed0` | build the three fixed-point collision points | translated | high | `806baed0_FUN_806baed0.c` |
| `0x806BB230` | `fn_806bb230` | spawn `Wm_ko_fireattack` when the flag is set | translated | high | `806bb230_FUN_806bb230.c` |
| `0x806BC410` | `fn_806bc410` | walk up to five `+0x30` links, then recurse | translated | high | `806bc410_FUN_806bc410.c` |
| `0x806BC4F0` | `fn_806bc4f0` | compare two names after the first `:` | translated | high | `806bc4f0_FUN_806bc4f0.c` |
| `0x806BC7E0` | `fn_806bc7e0` | refresh the interface and start the jump | translated | high | `806bc7e0_FUN_806bc7e0.c` |
| `0x806BCA10` | `fn_806bca10` | copy the player `Position_Jr` bone into the matrix | translated | high | `806bca10_FUN_806bca10.c` |
| `0x806BCC20` | `fn_806bcc20` | bind `prov` and copy the player variant | translated | high | `806bcc20_FUN_806bcc20.c` |
| `0x806BCCF0` | `fn_806bccf0` | advance the `prov` state machine | translated | high | `806bccf0_FUN_806bccf0.c` |
| `0x806BCDB0` | `fn_806bcdb0` | bind `wait`, flip the variant and set gravity | translated | high | `806bcdb0_FUN_806bcdb0.c` |
| `0x806BCE60` | `fn_806bce60` | main `jump_st`/`jumpA`/`jump_ed` state machine | translated | high | `806bce60_FUN_806bce60.c` |
| `0x806BD2A0` | `fn_806bd2a0` | reset the variant and jump latch | translated | high | `806bd2a0_FUN_806bd2a0.c` |
| `0x806BD2F0` | `fn_806bd2f0` | run the `go`/`go_wait` approach states | translated | high | `806bd2f0_FUN_806bd2f0.c` |
| `0x806BD510` | `fn_806bd510` | enter `prov` at the surface angle | translated | high | `806bd510_FUN_806bd510.c` |
| `0x806BD610` | `fn_806bd610` | run `prov2` with six animation gates | translated | high | `806bd610_FUN_806bd610.c` |
| `0x806BD7E0` | `fn_806bd7e0` | rebind `wait` and flip the variant | translated | high | `806bd7e0_FUN_806bd7e0.c` |
| `0x806BD880` | `fn_806bd880` | run `shock` with the nested landing checks | translated | high | `806bd880_FUN_806bd880.c` |
| `0x806BDB70` | `fn_806bdb70` | run the `jumpA` launch state | translated | high | `806bdb70_FUN_806bdb70.c` |
| `0x806BE160` | `fn_806be160` | scale the current angle by the facing table | translated | high | `806be160_FUN_806be160.c` |
| `0x806BF680` | `fn_806bf680` | build `KoopaShip` and six callback tables | translated | high | `806bf680_FUN_806bf680.c` |
| `0x806BFB80` | `fn_806bfb80` | load the `KoopaShip` BRRES and bind its model | translated | high | `806bfb80_FUN_806bfb80.c` |

**Totals:** 29 translated, 0 partial, 0 stub (29 high).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806B9CB0` | 72 | Calls the `halt_baddata` body `0x806B9720`; deferred with it. |
| `0x806BA2A0`, `0x806BA2D0` | 48 each | Vector-deleting-destructor thunks around `0x802DCBFC`; no independent behaviour. |
| `0x806BA630` | 24 | **Rejected**: `halt_baddata`. |
| `0x806BA660` | 76 | Calls `0x806BA630`; deferred with the truncated body. |
| `0x806BA940`, `0x806BA970` | 48 each | Vector-deleting-destructor thunks around `0x802DCBFC`. |
| `0x806BAEB0` | 28 | **Rejected**: unrecovered jump table (`Too many branches`). |
| `0x806BC2B0` | 92 | Duplicate of the release helper [`fn_806ba180`]; no independent behaviour. |
| `0x806BC5B0`, `0x806BC5E0` | 48 each | Vector-deleting-destructor thunks around `0x802DCBFC`. |
| `0x806BC610` | 60 | Jump-actor allocator; its constructor `0x806BC640` is deferred. |
| `0x806BC640` | 60 | Calls `0x8009AD30` with a dropped receiver; needs the main.dol prototype first. |
| `0x806BE780`, `0x806BE7E0`, `0x806BF4B0` | 88, 92, 88 | Duplicates of the release helper [`fn_806ba180`]. |
| `0x806BEA10` | 136 | Duplicate of [`fn_806bc4f0`] for another class. |
| `0x806BE930` | 224 | Duplicate of [`fn_806bc410`] for another class. |
| `0x806BF080` | 220 | Effect-actor setup; depends on `0x806BF1C0`. |
| `0x806BF1C0` | 252 | Receiver arrives through the `0x802DCD74` register-save thunk; needs the r3 convention checked before translating. |
| `0x806BF2C0`, `0x806BF340` | 104, 92 | Small `KoopaShip` transform helpers; batch with the remaining ship bodies. |
| `0x806BFD50` | 156 | Shared `KoopaShip` matrix update; deferred with `0x806BFDF0`/`0x806BFE60`. |
| `0x806BFDF0`, `0x806BFE60` | 104 each | `Position_Peach`/`Position_Jr` bone lookups; call sites drop their receiver. |
| `0x806BFF00` | 260 | Second jump-actor initialiser; deferred with `0x806BF1C0`. |
| `0x806C0000+` | - | Next actor family, untouched by this pass. |

## Unknowns preserved

1. **Class attribution.** None of the three families carries a
   `s_<Class>_c__` string in this window. `KoopaShip` is `// inferred:` from its
   BRRES and pane strings; the other two stay `// unknown:`.
2. **Dropped receivers.** `0x806B9CB0`, `0x806BF1C0`, `0x806BFDF0` and
   `0x806BFE60` lose their `this` through register-save thunks. Only the two
   selected call sites (`0x806BCA10`, `0x806BCE60`) pass the player object
   explicitly, with `// observed:` notes.
3. **`fn_806ba5a0` argument.** The caller `0x806BA530` passes the owner in
   `r3`, but the callee body refetches the local player and ignores it. The
   Rust keeps the no-argument body and documents the call site.
4. **`fn_806be160` virtual.** The C prints no argument for the `+0x404`
   virtual. The Rust calls it through `call0_ret` exactly as printed.
5. **Fixed-point tables.** `0x807F65AC..0x807F65CC` and
   `0x8042C1A0`/`0x8042C1A4` are read as raw words/bytes; their physical
   meaning (triangle offsets, facing scale, surface angle) is `// unknown:`.
6. **Callback-table stride.** `0x802DC9A0` is called with stride `0x18` and
   counts `2` or `4`; the table entry layout is not decoded further.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` contains 581 translated ledger rows after this pass; this
pass adds 29, all with `# Original` blocks and the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0; the remaining warnings are in other lanes' files.
