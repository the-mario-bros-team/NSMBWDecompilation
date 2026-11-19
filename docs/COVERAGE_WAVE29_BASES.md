# Coverage — wave 29 (`d_basesNP`: props, mini-game parts and boss-block loaders)

## How to read this file

This page records the twenty-ninth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** — *Translated functions* lists the 30 new bodies in
   `src/game/bases.rs`: two small props (`manjirou`, `bros_mega`), the
   mini-game balloon parts, five single-pane loaders, the `boat_light_wood`
   constructor/loader pair, the two Iggy loaders, six boss-block bodies, and
   five environment/effect loaders.
2. **Why these functions?** — *Selection* explains how the pass continued the
   address-ordered sweep above the wave-28 window and which decompiler outputs
   were skipped.
3. **What is still missing?** — *Deferred candidates* records the constructors
   that pair with wave-28 loaders, the paired-single/`CONCAT44` bodies and the
   functions whose ABI is not settled.

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

| Metric | Wave 29 |
|---|---:|
| New translated functions | 30 |
| `manjirou`/`bros_mega` constructors and loaders (`0x806EF6A0..0x806F0C10`) | 7 |
| Mini-game balloon parts (`0x806F6A50..0x806F80D0`) | 3 |
| Standalone single-pane/effect loaders (`0x806FB3E0..0x80705FB0`) | 5 |
| `boat_light_wood` constructor/loader pair (`0x80701820..0x80702750`) | 2 |
| Iggy sled and chain-chomp loaders (`0x80709FC0..0x8070B7F0`) | 2 |
| Boss-block bodies (`0x8070F270..0x80718E80`) | 6 |
| Snow, magma, bolt, water-screen and otehon loaders (`0x8071A400..0x8071FC40`) | 5 |
| Selected bytes | 9,732 |
| Translated functions in `src/game/bases.rs` (cumulative) | 729 (727 translated, 2 partial) |
| Cumulative selected bytes | 231,028 (20.82 % of indexed bytes) |

The 30 new bodies select 9,732 of the 1,109,860 indexed bytes (0.88 %) and 30 of
the 5,409 eligible entries (0.55 %). 28 rows are `high` confidence; 2 are
`medium` (`0x806F0140`, `0x806F0C10`) because a dropped ABI argument, an
unresolved tail call or unnamed constants remain.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2–28 were removed before selection. Wave 28 stopped at
`0x806E9E90`; this pass began at `0x806EF6A0` and continued upward, taking
complete constructor/loader clusters rather than isolated tiny bodies.

1. **Take complete constructor/loader pairs.** `0x806EF6A0` allocates the
   `0x458`-byte `bros_mega` actor and `0x806F46A0` loads its
   `g3d/bros_mega.brres`; the same pair shape links `0x8070F270` with
   `0x8070F630` (`boss_kameck_block`) and `0x80701820` with `0x80702750`
   (`boat_light_wood`).
2. **Take the `manjirou` actor as a lifecycle.** `0x806EFEF0` constructs the
   `0x968`-byte object (four vtables, four callback tables); `0x806F0140` is the
   placement/init callback; `0x806F03E0` loads `g3d/manjirou.brres`; and
   `0x806F0C10` repacks the pose before the model update.
3. **Take the remaining mini-game balloon parts.** `0x806F6A50`,
   `0x806F7690` and `0x806F80D0` complete the `MG_balloon` pole/centre family
   next to the untranslated loader at `0x806F64B0`.
4. **Take explicit BRRES loaders.** Every remaining loader in the window with
   a complete body and named panes was translated: `mouse_hole`,
   `boss_koopaJr_toge`, `Mask_flashlight`, `obj_nut`, `MG_RotatePanel`,
   `sled_boss_iggy`, `wanwan_boss_iggy`, `boss_larry_block`,
   `lift_boss_ludwig`, `boss_morton_ashiba`, `boss_roy_dokan`, `env_snow`,
   `obj_magmadeco`, `bolt`, `obj_waterscreen` and `block_otehon`.
5. **Rejected or deferred decompiler failures.** `0x806EA110`, `0x806EB6B0`,
   `0x806EC2B0`, `0x806EC380`, `0x806EC5F0`, `0x806EC6D0`, `0x806EC780`,
   `0x806ED340`, `0x806EDE70`, `0x806EECA0`, `0x807003C0`, `0x807005C0`,
   `0x80702AA0`, `0x80702BC0`, `0x80702CE0`, `0x80702FD0`, `0x807032E0`,
   `0x80703750`, `0x80703B40`, `0x80703DC0`, `0x80704050`, `0x807040C0` and
   `0x807108B0` decode to `halt_baddata`. `0x806F64B0`, `0x8070F370`,
   `0x8070F7C0`, `0x8070F9C0` and `0x80710690` use `CONCAT44` unsigned-integer
   to-float conversions or dropped arguments and need an ABI pass first.

## Clusters

### `manjirou` and `bros_mega` props (`0x806EF6A0..0x806F0C10`)

- `0x806EFEF0` allocates `0x968` bytes and installs vtables
  `0x807FED90`/`0x807FEF40` plus the callback tables at `+0x430`, `+0x610`
  and `+0x700`.
- `0x806F0140` reads the actor flag word at `+4`, stores the spawn variant in
  `+0x740`, builds two `0x5C`-byte motion records at `+0x610` and applies the
  ground or air transform set.
- `0x806F03E0` opens `g3d/manjirou.brres`, binds `manjirou`, `ridden` and
  `glass`, and drives the rider sub-object's virtual slots `+0x34`/`+0x50`.
- `0x806EF6A0`/`0x806F46A0`/`0x806F48A0` build and load the `bros_mega`
  actor; `0x806F48A0` copies the 12-word transform and applies it to the
  `mega_hammer` model.

### Mini-game balloon (`0x806F6A50..0x806F80D0`)

- `0x806F6A50` resolves the `pole_position` marker, tolerates a missing
  marker, and copies the sampled transform from `+0x6E4` to `+0x714`.
- `0x806F7690` opens `g3d/MG_balloon.brres` and chooses
  `MG_balloon_pole` or `MG_balloon_center` from `+0x574`.
- `0x806F80D0` repeats the selector for `pole_end` /
  `MG_balloon_center_model` and samples the model transform.

### Standalone loaders (`0x806FB3E0..0x80705FB0`)

All five share the `0x800690C0` accessor / `0x800DF180` BRRES loader /
`0x80239E30` pane / `0x80169D90` or `0x8016A3A0` binding / `0x80064BF0`
finaliser shape. `0x80701F80` is the outlier: it uses the six-argument
accessor `0x8016A7F0` and the `0x7F`/`0x86` layout mask, and it is 308 bytes
because it also attaches the `flashlight` clip.

### `boat_light_wood` (`0x80701820..0x80702750`)

`0x80701820` builds the `0xE18`-byte object; `0x80702750` opens
`g3d/boat_light_wood.brres`, binds the wood pane, the `light_fog_st` pane and
the fog animation, and starts both clips at `0x807CC530`.

### Iggy and boss blocks (`0x80709FC0..0x80718E80`)

- `0x80709FC0` loads `g3d/sled_boss_iggy.brres` and attaches `sled_to_L`;
  `0x8070B7F0` loads the chain-chomp `wanwan_boss_iggy` model and its
  `magic_body` pane.
- `0x8070F270` constructs the `0x1620`-byte `boss_kameck_block` actor;
  `0x8070F630` selects its `S`/`M`/`L` pane from the pair at `+0x7B0`.
- `0x80712B90` repeats the same `M`/`L` selection for
  `boss_larry_block`, including the separate `magic` and `mt` variants.
- `0x807146F0`, `0x80715930` and `0x80718E80` load the Ludwig lift, Morton
  axle and Roy pipe models with their magic panes.

### Environment and effect loaders (`0x8071A400..0x8071FC40`)

- `0x8071A400` loads `g3d/env_snow.brres` in one of two six-pane layouts,
  selected by the low byte of the actor flag word.
- `0x8071AFF0` loads `obj_magmadeco` and `wmy_sparkfire`, then selects one of
  three timer/word triples from `+0x12F8` and calls virtual slot `+0x20`.
- `0x8071C0D0`, `0x8071CC00` and `0x8071FC40` load the `bolt`, water-screen
  (two BRRES files) and `block_otehon` models with their single panes.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave29-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806EF6A0` | `fn_806ef6a0` | construct the `0x458`-byte `bros_mega` prop | translated | high | `806ef6a0_FUN_806ef6a0.c` |
| `0x806EFEF0` | `fn_806efef0` | construct the `0x968`-byte `manjirou` lift prop | translated | high | `806efef0_FUN_806efef0.c` |
| `0x806F0140` | `fn_806f0140` | place and initialise the `manjirou` actor | translated | medium | `806f0140_FUN_806f0140.c` |
| `0x806F03E0` | `fn_806f03e0` | load `manjirou` and its ridden/glass parts | translated | high | `806f03e0_FUN_806f03e0.c` |
| `0x806F0C10` | `fn_806f0c10` | repack the `manjirou` pose and update the model | translated | medium | `806f0c10_FUN_806f0c10.c` |
| `0x806F46A0` | `fn_806f46a0` | load `bros_mega` and the `mega_hammer` pane | translated | high | `806f46a0_FUN_806f46a0.c` |
| `0x806F48A0` | `fn_806f48a0` | copy the 12-word transform onto the mega hammer | translated | high | `806f48a0_FUN_806f48a0.c` |
| `0x806F6A50` | `fn_806f6a50` | resolve the balloon `pole_position` marker | translated | high | `806f6a50_FUN_806f6a50.c` |
| `0x806F7690` | `fn_806f7690` | load the balloon pole or centre model | translated | high | `806f7690_FUN_806f7690.c` |
| `0x806F80D0` | `fn_806f80d0` | resolve the balloon `pole_end` marker | translated | high | `806f80d0_FUN_806f80d0.c` |
| `0x806FB3E0` | `fn_806fb3e0` | load `MG_RotatePanel` and start its motion | translated | high | `806fb3e0_FUN_806fb3e0.c` |
| `0x806FF4C0` | `fn_806ff4c0` | load `mouse_hole` | translated | high | `806ff4c0_FUN_806ff4c0.c` |
| `0x806FFDB0` | `fn_806ffdb0` | load `boss_koopaJr_toge` into four parts | translated | high | `806ffdb0_FUN_806ffdb0.c` |
| `0x80701820` | `fn_80701820` | construct the `0xE18`-byte boat-light actor | translated | high | `80701820_FUN_80701820.c` |
| `0x80701F80` | `fn_80701f80` | load `Mask_flashlight` and its clip | translated | high | `80701f80_FUN_80701f80.c` |
| `0x80702750` | `fn_80702750` | load `boat_light_wood` and the fog parts | translated | high | `80702750_FUN_80702750.c` |
| `0x80705FB0` | `fn_80705fb0` | load `obj_nut` | translated | high | `80705fb0_FUN_80705fb0.c` |
| `0x80709FC0` | `fn_80709fc0` | load `sled_boss_iggy` and `sled_to_L` | translated | high | `80709fc0_FUN_80709fc0.c` |
| `0x8070B7F0` | `fn_8070b7f0` | load `wanwan_boss_iggy` and `magic_body` | translated | high | `8070b7f0_FUN_8070b7f0.c` |
| `0x8070F270` | `fn_8070f270` | construct the `0x1620`-byte `boss_kameck_block` actor | translated | high | `8070f270_FUN_8070f270.c` |
| `0x8070F630` | `fn_8070f630` | load `boss_kameck_block` S/M/L and its parts | translated | high | `8070f630_FUN_8070f630.c` |
| `0x80712B90` | `fn_80712b90` | load `boss_larry_block` M/L and its parts | translated | high | `80712b90_FUN_80712b90.c` |
| `0x807146F0` | `fn_807146f0` | load `lift_boss_ludwig` and its magic pane | translated | high | `807146f0_FUN_807146f0.c` |
| `0x80715930` | `fn_80715930` | load `boss_morton_ashiba` and its magic pane | translated | high | `80715930_FUN_80715930.c` |
| `0x80718E80` | `fn_80718e80` | load `boss_roy_dokan` and its magic pane | translated | high | `80718e80_FUN_80718e80.c` |
| `0x8071A400` | `fn_8071a400` | load `env_snow` in one of two layouts | translated | high | `8071a400_FUN_8071a400.c` |
| `0x8071AFF0` | `fn_8071aff0` | load `obj_magmadeco` and `wmy_sparkfire` | translated | high | `8071aff0_FUN_8071aff0.c` |
| `0x8071C0D0` | `fn_8071c0d0` | load `bolt` | translated | high | `8071c0d0_FUN_8071c0d0.c` |
| `0x8071CC00` | `fn_8071cc00` | load `obj_waterscreen` and `waterPlate_W4boss` | translated | high | `8071cc00_FUN_8071cc00.c` |
| `0x8071FC40` | `fn_8071fc40` | load `block_otehon` and its magic pane | translated | high | `8071fc40_FUN_8071fc40.c` |

**Totals:** 30 translated, 0 partial, 0 stub (28 high, 2 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806E9E10` | 116 | Constructor for the wave-28 `cube_kinoko` loader; complete but outside this pass's address window. |
| `0x806EAB20` | 208 | Unnamed `0x700`-byte constructor; pairs with a loader outside the selected range. |
| `0x806EBDC0` | 328 | `0x1398`-byte constructor; starts with the `0x802DCD74` save prolog, so the incoming argument needs the same ABI review as `0x806F0140`. |
| `0x806F0BC0` | 88 | Two-record reset loop; trivial but not selected. |
| `0x806F0D00` | 176 | Two animation updates; left for the `manjirou` lifecycle batch. |
| `0x806F3350` | 2,092 | Large `koopa` demo-field loader; needs the `0x806F...` callback/field map first. |
| `0x806F5640` | 292 | `W7_shiroboss_bg_M` loader; the C nulls a pointer and then dereferences it, so the intended non-null branch needs review. |
| `0x806F64B0` | 492 | `MG_balloon` loader with a `CONCAT44` unsigned-integer-to-float conversion. |
| `0x8070F370` | 692 | `boss_kameck_block` placement callback; `CONCAT44` conversions and a dropped argument at the `0x8070F630` call. |
| `0x8070F7C0`, `0x8070F9C0` | 500, 232 | `boss_kameck_block` timing helpers with `CONCAT44` conversions. |
| `0x80710520`, `0x807105C0`, `0x80710690` | 148, 192, 532 | Kameck block tail bodies; `CONCAT44` and a dropped argument. |
| `0x807108B0` | 2,468 | **Rejected**: `halt_baddata` in the export. |

## Unknowns preserved

1. **`0x800DF180`.** The BRRES loader identity is not in this export; every
   loader keeps an `// unknown:` note.
2. **`0x802DCD6C` / `0x802DCDB8`.** [`fn_806f0140`] starts with the outlined
   register-save pair; the Rust follows the existing convention that the saved
   `r3` is the object pointer.
3. **`0x806F0F20`.** [`fn_806f0c10`] calls this body after the pose update; the
   export decodes it to `halt_baddata`, so the call is address-only.
4. **`0x807CC0xx` / `0x807CC3B8` / `0x807CC530` / `0x807CCB70` /
   `0x807CCB80` / `0x807CCC48` / `0x807CCE14` / `0x807CCF00..08` /
   `0x807CD0C0..D0`.** These are single-precision constants or timer values;
   their units are not recovered. The Rust keeps raw reads.
5. **`s_YPg3d_wanwan_boss_iggy_brres_8080567d + 3`.** The C adds three to the
   symbol base; [`fn_8070b7f0`] passes the effective address `0x80805680`.
6. **`0x80094E80`.** [`fn_8070f270`] uses this base constructor instead of the
   usual `0x80066FC0`; its role is inferred from the constructor shape.
7. **Class identity.** `manjirou`, `bros_mega`, `boat_light_wood` and the boss
   block names are BRRES/pane names; no class string appears in the window.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` has 729 functions after this pass (727 `translated`,
2 pre-existing `partial`); this pass adds 30, all with `# Original` blocks and
the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0; `src/game/bases.rs` produces no warnings. The file also
compiled in an isolated `no_std` crate with a copy of `platform.rs`, so its
status is independent of concurrent edits in other lanes.
