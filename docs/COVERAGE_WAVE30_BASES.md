# Coverage — wave 30 (`d_basesNP`: rotating pedestals, pipes and prop loaders)

## How to read this file

This page records the thirtieth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** — *Translated functions* lists the 30 new bodies in
   `src/game/bases.rs`: nine actor constructors and twenty-one BRRES
   loaders/helpers for blocks, doors, cannons, lifts, pedestals, pipes,
   penguins, chests and the deep-sea light.
2. **Why these functions?** — *Selection* explains how the pass continued the
   address-ordered sweep above the wave-29 window and which decompiler outputs
   were skipped.
3. **What is still missing?** — *Deferred candidates* records the nearby
   bodies that need an ABI fix, a `CONCAT44` pass, or a data-table model.

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

| Metric | Wave 30 |
|---|---:|
| New translated functions | 30 |
| Actor constructors (`0x80723C30..0x8073C2A0`) | 9 |
| BRRES/effect loaders and helpers (`0x80723E00..0x8073D340`) | 21 |
| Selected bytes | 6,032 |
| Translated functions in `src/game/bases.rs` (cumulative) | 759 (757 translated, 2 partial) |
| Cumulative selected bytes | 237,060 (21.36 % of indexed bytes) |

The 30 new bodies select 6,032 of the 1,109,860 indexed bytes (0.54 %) and 30
of the 5,409 eligible entries (0.55 %). 29 rows are `high` confidence; 1 is
`medium` (`0x8072C200`) because the six-word effect record and the callee's
interpretation of its two overlapping pointers are inferred.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2–29 were removed before selection. Wave 29 stopped at
`0x8071FC40`; this pass began at `0x80723C30` and continued upward, taking
complete constructor/loader clusters instead of isolated tiny bodies.

1. **Take complete constructor/loader pairs.** `0x80723C30` allocates the
   `0x5B8`-byte block actor whose loader `0x80723E00` opens
   `g3d/block_pow.brres`; the same shape links `0x80729FB0` with `0x8072A020`
   (`obj_door`), `0x8072CDB0` with `0x8072CAB0` (`lift_zen_chika_pori`),
   `0x8072D1C0` with `0x8072D270` (`cube_kinoko`), `0x8072F250` with
   `0x8072F430` (`obj_dokan`), `0x807333C0` with `0x8072E460`/`0x80733A40`
   (`houdai_rot`/`houdai_slope`), `0x80734950` with `0x80734D70`
   (`obj_Hgear`), `0x8073A2F0` with `0x8073A360` (`penguin2`) and
   `0x8073C2A0` with `0x8073C310` (`lift_turi_ashiba_wire`).
2. **Take the standalone loaders in the same window.** `0x80726AF0`
   (`block_fly`), `0x8072D7A0` (`hikari`), `0x80731550`
   (`houdai_rot_large`), `0x80731610` (`obj_dokan`, two holders), `0x80737F30`
   (`ice`), `0x8073B020` (`shinkai_light`), `0x8073C810` (`poltergeist`) and
   `0x8073D340` (`treasure_box`) have complete bodies and named panes.
3. **Take the two effect helpers next to the fire cannon.** `0x8072C180`
   copies the `fire_rot_cannon_gun` transform; `0x8072C200` positions the two
   `Wm_en_fireburner*` effect panes through virtual slot `+0xA0`.
4. **Rejected or deferred decompiler failures.** `0x807242C0` and
   `0x80724320` decode to `halt_baddata`. `0x80722180`, `0x807268C0`,
   `0x80729490`, `0x8072B670`, `0x8072C000`, `0x8072FA90`, `0x80731380`,
   `0x80733900`, `0x80736890`, `0x80737270`, `0x8073FF60` and the later
   `0x8074xxxx` bodies use `CONCAT44` unsigned-integer-to-float conversions or
   `unaff_` registers and need an ABI pass first.

## Clusters

### Block actors (`0x80723C30..0x80726AF0`)

- `0x80723C30` builds the `0x5B8`-byte object, vtable `0x8080AC00`,
  accessor `+0x3D0`, model `+0x3F0`, callback table `+0x430` and motion
  `+0x4CC`. `0x80723E00` then loads `g3d/block_pow.brres`, binds the
  `block_pow` pane at priority `0x20` and attaches the clip at `0x8080ABD4`.
- `0x80726AF0` is the `block_fly` loader; it binds `block_fly` with priority
  `0x128` and parents the model through `0x8016A000`.

### Door and fire cannon (`0x80729FB0..0x8072C200`)

- `0x80729FB0`/`0x8072A020` are the `obj_door` pair; the loader binds
  `obj_doorA` at priority `0x20`.
- `0x8072B8D0` loads `g3d/fire_rot_cannon.brres` twice: the gun pane at
  `+0x3F0` (priority `0x20`) and `fire_effect1x4_right` at `+0x430`
  (priority `0x224`), then creates the flame clip at `+0x474`.
- `0x8072C180` copies the gun transform into `+0x4A8`; `0x8072C200` builds a
  six-word effect record and calls virtual slot `+0xA0` on `+0x1374` and
  `+0x149C`.

### Lifts and pedestals (`0x8072CAB0..0x80733A40`)

- `0x8072CDB0` allocates 19,000 bytes and installs the `0x80309938` /
  `0x8080C794` vtables plus a `+0x4FC` motion block of two `0x2290`-byte
  records. `0x8072CAB0` loads the indexed `lift_zen_chika_poriS` pane from
  the pointer table at `0x807CD500` and starts both animations.
- `0x8072D1C0`/`0x8072D270` are the `cube_kinoko` pair; the pane table is at
  `0x807CD550` and the binder is `0x8016A3A0`.
- `0x8072F250`/`0x8072F430` build the `obj_dokan` actor; the loader fills the
  twelve-byte model array at `+0x3FC` with `+0x400` entries.
- `0x807333C0` allocates `0x660` bytes, installs vtable `0x8080D628` and the
  sub-vtable `0x8080D708` at `+0x3D0`, and is shared by the
  `houdai_rot`/`houdai_slope` loaders `0x8072E460` and `0x80733A40`.
  `0x80731550` is the `houdai_rot_large` variant.

### Props (`0x80734950..0x8073D340`)

- `0x80734950`/`0x80734D70` build the `obj_Hgear` actor; the pane is chosen
  from the pointer table at `0x8080D8C8` by the word at `+0x514`.
- `0x80737F30` loads `g3d/ice.brres` and binds `ice_A1` at priority `0x24`.
- `0x8073A2F0`/`0x8073A360` are the `penguin2` pair; this loader finalises
  with `0x80064BD0` instead of `0x80064BF0`.
- `0x8073B020` loads `g3d/shinkai_light.brres`, attaches its `shinkai_light`
  clip through virtual slot `+0x18`, then adds the effect pane at `+0x430`.
- `0x8073C2A0`/`0x8073C310` are the hanging-wire lift pair; `0x8073C810`
  loads the poltergeist `pol_shokudai` pane.
- `0x8073D340` loads `g3d/treasure_box.brres` and branches on `+0x8E0`:
  value `1` attaches the `shake` clip and sets its speed from `+0x4B8`; any
  other value uses the unnamed clip name at `0x8080EE34`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave30-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80723C30` | `fn_80723c30` | construct the `0x5B8`-byte `block_pow` actor | translated | high | `80723c30_FUN_80723c30.c` |
| `0x80723E00` | `fn_80723e00` | load `block_pow` and its clip | translated | high | `80723e00_FUN_80723e00.c` |
| `0x80726AF0` | `fn_80726af0` | load `block_fly` and parent the model | translated | high | `80726af0_FUN_80726af0.c` |
| `0x80729FB0` | `fn_80729fb0` | construct the `0x438`-byte `obj_door` actor | translated | high | `80729fb0_FUN_80729fb0.c` |
| `0x8072A020` | `fn_8072a020` | load `obj_door` and its `obj_doorA` pane | translated | high | `8072a020_FUN_8072a020.c` |
| `0x8072B8D0` | `fn_8072b8d0` | load the rotating fire cannon, flame pane and clip | translated | high | `8072b8d0_FUN_8072b8d0.c` |
| `0x8072C180` | `fn_8072c180` | copy the `fire_rot_cannon_gun` transform | translated | high | `8072c180_FUN_8072c180.c` |
| `0x8072C200` | `fn_8072c200` | position the fire-burner effect panes | translated | medium | `8072c200_FUN_8072c200.c` |
| `0x8072CAB0` | `fn_8072cab0` | load `lift_zen_chika_pori` and its indexed pane | translated | high | `8072cab0_FUN_8072cab0.c` |
| `0x8072CDB0` | `fn_8072cdb0` | construct the 19,000-byte pori lift actor | translated | high | `8072cdb0_FUN_8072cdb0.c` |
| `0x8072D1C0` | `fn_8072d1c0` | construct the `0x510`-byte `cube_kinoko` actor | translated | high | `8072d1c0_FUN_8072d1c0.c` |
| `0x8072D270` | `fn_8072d270` | load `cube_kinoko` and its indexed pane | translated | high | `8072d270_FUN_8072d270.c` |
| `0x8072D7A0` | `fn_8072d7a0` | load `hikari` and its clip | translated | high | `8072d7a0_FUN_8072d7a0.c` |
| `0x8072E460` | `fn_8072e460` | load `houdai_rot` and parent the model | translated | high | `8072e460_FUN_8072e460.c` |
| `0x8072F250` | `fn_8072f250` | construct the `0x500`-byte `obj_dokan` actor | translated | high | `8072f250_FUN_8072f250.c` |
| `0x8072F430` | `fn_8072f430` | load `obj_dokan` and fill the pipe list | translated | high | `8072f430_FUN_8072f430.c` |
| `0x80731550` | `fn_80731550` | load `houdai_rot_large` and parent the model | translated | high | `80731550_FUN_80731550.c` |
| `0x80731610` | `fn_80731610` | load `obj_dokan` into two model holders | translated | high | `80731610_FUN_80731610.c` |
| `0x807333C0` | `fn_807333c0` | construct the `0x660`-byte rotating pedestal | translated | high | `807333c0_FUN_807333c0.c` |
| `0x80733A40` | `fn_80733a40` | load `houdai_slope` and parent the model | translated | high | `80733a40_FUN_80733a40.c` |
| `0x80734950` | `fn_80734950` | construct the `0x540`-byte `obj_Hgear` actor | translated | high | `80734950_FUN_80734950.c` |
| `0x80734D70` | `fn_80734d70` | load `obj_Hgear` and the variant pane | translated | high | `80734d70_FUN_80734d70.c` |
| `0x80737F30` | `fn_80737f30` | load `ice` and its `ice_A1` clip | translated | high | `80737f30_FUN_80737f30.c` |
| `0x8073A2F0` | `fn_8073a2f0` | construct the `0x460`-byte `penguin2` actor | translated | high | `8073a2f0_FUN_8073a2f0.c` |
| `0x8073A360` | `fn_8073a360` | load `penguin2` and its glasses pane | translated | high | `8073a360_FUN_8073a360.c` |
| `0x8073B020` | `fn_8073b020` | load `shinkai_light` with its clip and effect pane | translated | high | `8073b020_FUN_8073b020.c` |
| `0x8073C2A0` | `fn_8073c2a0` | construct the `0x410`-byte hanging-wire lift | translated | high | `8073c2a0_FUN_8073c2a0.c` |
| `0x8073C310` | `fn_8073c310` | load `lift_turi_ashiba_wire` | translated | high | `8073c310_FUN_8073c310.c` |
| `0x8073C810` | `fn_8073c810` | load `poltergeist` and `pol_shokudai` | translated | high | `8073c810_FUN_8073c810.c` |
| `0x8073D340` | `fn_8073d340` | load `treasure_box` with shake/plain clips | translated | high | `8073d340_FUN_8073d340.c` |

**Totals:** 30 translated, 0 partial, 0 stub (29 high, 1 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x8072D6C0` | 220 | `hikari` placement helper; copies the actor transform into the model but has no BRRES load, so it belongs with the unused-actor lifecycle batch. |
| `0x8073D4D0` | 476 | `I_kinoko` loader with a `0x802DCD70` save prolog; the first argument is dropped and the seven-entry pane tables need their own pass. |
| `0x80744DC0` | 356 | `obj_chikuwa_block` create body; calls the local helpers `0x80744CF0` and returns `1`, so it must be translated with the surrounding callback block. |
| `0x80745550` | 192 | `rail` create body; calls `0x80745380`, `0x80745410` and `0x807452A0`, none of which are translated yet. |
| `0x8074E8D0` | 172 | `box_iron_machine` loader; complete, but outside this pass's selected window (above `0x8073D340`). |
| `0x8074F1A0` | 436 | `warp_cannon` loader; uses the `language`/`number` panes and needs the variant map first. |
| `0x807242C0`, `0x80724320` | 88, 88 | **Rejected**: `halt_baddata` in the export. |

## Unknowns preserved

1. **`0x800DF180`.** The BRRES loader identity is not in this export; every
   loader keeps an `// unknown:` note.
2. **Constructor pairing.** The REL keeps only vtables, so the nine
   constructors are named by the loader whose field offsets they satisfy.
   Every such pairing is marked `// inferred:` in the Rust doc.
3. **`0x8072C200` record.** The C builds `{u16, u16, s16, u32, u32, u32}` on
   the stack and passes `&local_20` (the vector) and `&local_28` (the record)
   to virtual slot `+0xA0`; the callee's interpretation of the overlapping
   pointers is inferred.
4. **`0x801650D0` / `0x801651F0` / `0x80168FE0`.** These apply clips and
   speeds; the units of the constants at `0x807CDE1C` are not recovered.
5. **Class identity.** `block_pow`, `block_fly`, `obj_door`,
   `fire_rot_cannon`, `lift_zen_chika_pori`, `cube_kinoko`, `obj_dokan`,
   `obj_Hgear`, `penguin2`, `shinkai_light` and `treasure_box` are
   BRRES/pane names; no class string appears in this window.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 759 functions after this pass (757 `translated`,
2 pre-existing `partial`); this pass adds 30, all with `# Original` blocks and
the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0; `src/game/bases.rs` produces no warnings.
